#![allow(unused)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

use std::rc::Rc;
use std::cell::RefCell;
use bytes::Bytes;
use b_error::{BResult, ResultExt};
use ckb_vm::{
    SparseMemory, Register, Memory, TraceMachine, DefaultCoreMachine,
    WXorXMemory, DefaultMachineBuilder, DefaultMachine, SupportMachine,
};
use ckb_vm::decoder::{build_imac_decoder, Decoder};
use ckb_vm::Error as CkbError;
use ckb_vm::Syscalls;
use ckb_vm::CoreMachine;
use crate::game::{Match, Game, PlayerState, Move, MoveKind, MovePair, NextGameState, ActiveState, EndState, Turn, Player};
use crate::game::{GAMES_PER_MATCH, P1_START_POS, P2_START_POS, START_ENERGY, GAME_FIELD_SIZE};
use ckb_vm::registers::*;
use std::convert::TryFrom;
use std::mem;

pub mod game;
mod transition;
mod cost_model;

pub fn run_match(p1exe: &Bytes, p2exe: &Bytes) -> BResult<Match> {

    let mut games = vec![];
    for _ in 0..GAMES_PER_MATCH {
        let game = run_game(p1exe, p2exe)?;
        games.push(game);
    }

    Ok(Match { games })
}

fn run_game(p1exe: &Bytes, p2exe: &Bytes) -> BResult<Game> {
    let game_state = Rc::new(RefCell::new(GameState::new()));

    let p1syscalls = GameSyscalls {
        player: Player::P1,
        game_state: game_state.clone(),
    };
    let p2syscalls = GameSyscalls {
        player: Player::P2,
        game_state: game_state.clone(),
    };
    
    let mut p1m = make_vm(p1syscalls)?;
    let mut p2m = make_vm(p2syscalls)?;

    p1m.load_program(p1exe, &[]).e()?; // TODO
    p2m.load_program(p2exe, &[]).e()?; // TODO

    p1m.set_running(true);
    p2m.set_running(true);
    p1m.set_cycles(0);
    p2m.set_cycles(0);

    let decoder = build_imac_decoder::<u32>();

    debug!("running");
    let mut turn_no = 0;
    loop {
        if game_state.borrow().p1next.is_none() {
            match p1m.step(&decoder) {
                Err(CkbError::InvalidCycles) => {
                    game_state.borrow_mut().no_energy(Player::P1);
                },
                Err(e) => Err(e).e()?,
                Ok(_) => { }
            }
        }
        if game_state.borrow().p2next.is_none() {
            match p2m.step(&decoder) {
                Err(CkbError::InvalidCycles) => {
                    game_state.borrow_mut().no_energy(Player::P2);
                },
                Err(e) => Err(e).e()?,
                Ok(_) => { }
            }
        }
        if !p1m.running() {
            panic!("p1 stopped: {}", p1m.exit_code());
        }
        if !p2m.running() {
            panic!("p2 stopped: {}", p2m.exit_code());
        }
        if game_state.borrow().ready_for_turn() {
            debug!("turn {}", turn_no);
            game_state.borrow_mut().evaluate(&mut p1m, &mut p2m, turn_no);
            turn_no += 1;
            if game_state.borrow().end.is_some() {
                break;
            }
        }
    }
    debug!("ending");

    let r = game_state.borrow().to_game_result();
    Ok(r)
}    

struct GameState {
    pub past_turns: Vec<Turn>,
    pub p1state: PlayerState,
    pub p2state: PlayerState,
    pub p1next: Option<Move>,
    pub p2next: Option<Move>,
    pub p1cycles: i32,
    pub p2cycles: i32,
    pub end: Option<EndState>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            past_turns: vec![],
            p1state: PlayerState {
                pos: P1_START_POS,
                energy: START_ENERGY,
            },
            p2state: PlayerState {
                pos: P2_START_POS,
                energy: START_ENERGY,
            },
            p1next: None,
            p2next: None,
            p1cycles: 0,
            p2cycles: 0,
            end: None,
        }
    }

    fn ready_for_turn(&self) -> bool {
        self.p1next.is_some() && self.p2next.is_some()
    }
    
    fn evaluate(&mut self, p1m: &mut GameMachine, p2m: &mut GameMachine, turn_no: i32) {
        assert!(self.ready_for_turn());
        let (p1next, p2next) = (self.p1next.expect("p1next"),
                                self.p2next.expect("p2next"));

        let this_state = ActiveState {
            p1: self.p1state,
            p2: self.p2state,
        };

        let move_pair = MovePair {
            p1: p1next,
            p2: p2next,
        };

        debug!("evaluating next game state");

        let (turn, next_state) = this_state.make_move(move_pair, turn_no);

        debug!("current_state: {:?}", this_state);
        debug!("next_state: {:?}", next_state);

        self.past_turns.push(turn);

        match next_state {
            NextGameState::Active(state) => {
                self.p1state = state.p1;
                self.p2state = state.p2;
            },
            NextGameState::End(state) => {
                self.p1state = state.inner_state().p1;
                self.p2state = state.inner_state().p2;
                self.end = Some(state);
            }
        }

        self.p1next = None;
        self.p2next = None;
    }

    fn no_energy(&mut self, player: Player) {
        debug!("player {:?} ran out of energy", player);
        match player {
            Player::P1 => {
                self.p1next = Some(Move {
                    kind: MoveKind::NoEnergy,
                    energy_spent: self.p1state.energy,
                });
            },
            Player::P2 => {
                self.p2next = Some(Move {
                    kind: MoveKind::NoEnergy,
                    energy_spent: self.p2state.energy,
                });
            },
        }
    }

    fn to_game_result(&self) -> Game {
        Game {
            turns: self.past_turns.clone(),
            end: self.end.expect("end state"),
        }
    }
}

struct GameSyscalls {
    player: Player,
    game_state: Rc<RefCell<GameState>>,
}

impl Syscalls<GameCoreMachine> for GameSyscalls {
    fn initialize(&mut self, machine: &mut GameCoreMachine) -> Result<(), CkbError> { Ok(()) }

    fn ecall(&mut self, machine: &mut GameCoreMachine) -> Result<bool, CkbError> {
        let mut game_state = self.game_state.borrow_mut();
        let num = machine.registers()[A7];
        match num as u32 {
            ECALL_STATE => {
                ecall4(machine, |m, pa_pos_ptr, pb_pos_ptr, pa_energy_ptr, pb_energy_ptr| {
                    let r = e_state(m,
                                    &mut game_state,
                                    self.player,
                                    pa_pos_ptr, pb_pos_ptr,
                                    pa_energy_ptr, pb_energy_ptr);
                    r
                });
                Ok(true)
            },
            ECALL_MOVE => {
                ecall1(machine, |m, move_kind| {
                    e_move(m,
                           &mut game_state,
                           self.player,
                           move_kind)
                });
                Ok(true)
            },
            _ => Ok(false),
        }
    }
}

fn ecall4<F>(m: &mut GameCoreMachine, mut f: F)
    where F: FnMut(&mut GameCoreMachine, u32, u32, u32, u32) -> u32
{
    let arg0 = m.registers()[A0];
    let arg1 = m.registers()[A1];
    let arg2 = m.registers()[A2];
    let arg3 = m.registers()[A3];
    let r = f(m, arg0, arg1, arg2, arg3);
    m.set_register(A0, r);
}

fn ecall1<F>(m: &mut GameCoreMachine, mut f: F)
    where F: FnMut(&mut GameCoreMachine, u32) -> u32
{
    let arg0 = m.registers()[A0];
    let r = f(m, arg0);
    m.set_register(A0, r);
}

const ECALL_STATE: u32 = 0x0100;
const ECALL_MOVE: u32 =  0x0101;

type MWord = u32;

fn e_state(machine: &mut GameCoreMachine,
           game_state: &GameState,
           player: Player,
           pa_pos_ptr: MWord, pb_pos_ptr: MWord,
           pa_energy_ptr: MWord, pb_energy_ptr: MWord) -> MWord {

    const ERR_BAD_STORE: MWord = 1;
    
    // Note that every bot sees itself as player 1,
    // so for player 2 we have to swap the positions.

    let mut pa_pos = game_state.p1state.pos as MWord;
    let mut pa_energy = game_state.p1state.energy as MWord;
    let mut pb_pos = game_state.p2state.pos as MWord;
    let mut pb_energy = game_state.p2state.energy as MWord;

    if player == Player::P2 {
        let pa_pos_tmp = pa_pos;
        pa_pos = GAME_FIELD_SIZE as u32 - 1 - pb_pos;
        pb_pos = GAME_FIELD_SIZE as u32 - 1 - pa_pos_tmp;
        mem::swap(&mut pa_energy, &mut pb_energy);
    }

    let mem = machine.memory_mut();
    if mem.store32(&pa_pos_ptr, &pa_pos).is_err() {
        return ERR_BAD_STORE;
    }
    if mem.store32(&pb_pos_ptr, &pb_pos).is_err() {
        return ERR_BAD_STORE;
    }
    if mem.store32(&pa_energy_ptr, &pa_energy).is_err() {
        return ERR_BAD_STORE;
    }
    if mem.store32(&pb_energy_ptr, &pb_energy).is_err() {
        return ERR_BAD_STORE;
    }

    0
}

fn e_move(machine: &mut GameCoreMachine,
          game_state: &mut GameState,
          player: Player,
          move_kind: MWord) -> MWord {

    const MOVE_BACK: MWord = 1;
    const MOVE_STAND: MWord = 2;
    const MOVE_FORWARD: MWord = 3;
    const MOVE_LUNGE: MWord = 4;

    const ERR_BAD_MOVE: MWord = 1;
    
    debug!("making move for {:?}: {:?}", player, move_kind);

    let move_kind = match move_kind {
        MOVE_BACK => MoveKind::Back,
        MOVE_STAND => MoveKind::Stand,
        MOVE_FORWARD => MoveKind::Forward,
        MOVE_LUNGE => MoveKind::Lunge,
        _ => return ERR_BAD_MOVE,
    };

    debug!("making move for {:?}: {:?}", player, move_kind);

    let cycles = i32::try_from(machine.cycles()).expect("cycle overflow");

    match player {
        Player::P1 => {
            assert!(cycles >= game_state.p1cycles);
            game_state.p1next = Some(Move {
                kind: move_kind,
                energy_spent: cycles.checked_sub(game_state.p1cycles)
                    .expect("cycle counting"),
            });
            game_state.p1cycles = cycles;
        },
        Player::P2 => {
            game_state.p2next = Some(Move {
                kind: move_kind,
                energy_spent: cycles.checked_sub(game_state.p2cycles)
                    .expect("cycle counting"),
            });
            game_state.p2cycles = cycles;
        },
    }

    0
}           

type GameCoreMachine = DefaultCoreMachine<u32, WXorXMemory<u32, SparseMemory<u32>>>;
type GameMachine<'a> = DefaultMachine<'a, GameCoreMachine>;

fn make_vm<'a>(sys: GameSyscalls) -> BResult<GameMachine<'a>> {
    let core_machine = GameCoreMachine::new_with_max_cycles(START_ENERGY as u64);
    let builder = DefaultMachineBuilder::new(core_machine);
    let builder = builder.syscall(Box::new(sys));
    let builder = builder.instruction_cycle_func(Box::new(cost_model::instruction_cycles));
    let machine = builder.build();
    Ok(machine)
}
