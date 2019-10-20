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
use crate::game::{Match, Game};
use crate::game::GAMES_PER_MATCH;

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
    let game_state = Rc::new(RefCell::new(GameState {
        p1wait: false,
        p2wait: false,
    }));

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

        if !game_state.borrow().p1wait {
            p1m.step(&decoder).e()?; // TODO
        }
        if !game_state.borrow().p2wait {
            p2m.step(&decoder).e()?; // TODO
        }
        game_state.borrow_mut().evaluate();
    }
    println!("ending");

    let r = game_state.borrow().to_game_result();
    Ok(r)
}    

struct GameState {
    pub p1wait: bool,
    pub p2wait: bool,
}

impl GameState {
    fn evaluate(&mut self) {
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

#[derive(Eq, PartialEq)]
enum Player { P1, P2 }

struct GameSyscalls {
    player: Player,
    game_state: Rc<RefCell<GameState>>,
}

impl Syscalls<GameCoreMachine> for GameSyscalls {
    fn initialize(&mut self, machine: &mut GameCoreMachine) -> Result<(), CkbError> { Ok(()) }

    fn ecall(&mut self, machine: &mut GameCoreMachine) -> Result<bool, CkbError> {
        use ckb_vm::registers::*;

        let num = machine.registers()[A7];
        match num as u32 {
            ECALL_STATE => {
                println!("ecall state");
                machine.set_running(false);
                Ok(true)
            },
            ECALL_MOVE => {
                println!("ecall move");
                machine.set_running(false);
                Ok(true)
            },
            _ => Ok(false),
        }
    }
}

const ECALL_STATE: u32 = 0x0100;
const ECALL_MOVE: u32 =  0x0101;

fn e_state(game_state: &GameState,
           p1_pos: &mut i32, p2_pos: &mut i32,
           p1_energy: &mut i32, p2_energy: &mut i32) -> i32 {
    0
}

fn e_move(game_state: &mut GameState,
          move_kind: i32) -> i32 {
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
