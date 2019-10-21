#![allow(unused)]

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
use crate::game::{Match, Game, PlayerState};
use crate::game::{GAMES_PER_MATCH, P1_START_POS, P2_START_POS, START_ENERGY};
use ckb_vm::registers::*;

pub mod game;
mod transition;

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

    let decoder = build_imac_decoder::<u32>();

    println!("running");
    loop {
        assert!(p1m.running() == p2m.running());
        if !p1m.running() { break; }

        if !game_state.borrow().p1waiting {
            p1m.step(&decoder).e()?; // TODO
        }
        if !game_state.borrow().p2waiting {
            p2m.step(&decoder).e()?; // TODO
        }
        game_state.borrow_mut().evaluate(&mut p1m, &mut p2m);
    }
    println!("ending");

    let r = game_state.borrow().to_game_result();
    Ok(r)
}    

struct GameState {
    pub p1waiting: bool,
    pub p2waiting: bool,
    pub p1state: PlayerState,
    pub p2state: PlayerState,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            p1waiting: false,
            p2waiting: false,
            p1state: PlayerState {
                pos: P1_START_POS,
                energy: START_ENERGY,
            },
            p2state: PlayerState {
                pos: P2_START_POS,
                energy: START_ENERGY,
            },
        }
    }
    
    fn evaluate(&mut self, p1m: &mut GameMachine, p2m: &mut GameMachine) {
        if !(self.p1waiting && self.p2waiting) {
            return;
        }

        // TODO

        self.p1waiting = false;
        self.p2waiting = false;
    }

    fn to_game_result(&self) -> Game {
        use game::*;
        Game {
            turns: vec![],
            end: EndState::EnergyTie(ActiveState {
                p1: PlayerState {
                    pos: 0, energy: 0,
                },
                p2: PlayerState {
                    pos: 0, energy: 0,
                },
            }),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Player { P1, P2 }

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
    0
}

fn e_move(machine: &mut GameCoreMachine,
          game_state: &mut GameState,
          player: Player,
          move_kind: MWord) -> MWord {
    0
}           

type GameCoreMachine = DefaultCoreMachine<u32, WXorXMemory<u32, SparseMemory<u32>>>;
type GameMachine<'a> = DefaultMachine<'a, GameCoreMachine>;

fn make_vm<'a>(sys: GameSyscalls) -> BResult<GameMachine<'a>> {
    let core_machine = DefaultCoreMachine::default();
    let builder = DefaultMachineBuilder::new(core_machine);
    let builder = builder.syscall(Box::new(sys));
    let machine = builder.build();
    Ok(machine)
}
