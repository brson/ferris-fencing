#![allow(unused)]

#[macro_use]
extern crate env_logger;

use bytes::Bytes;
use std::fs::File;
use std::io::Read;
use b_error::{BResult, ResultExt};
use std::path::{PathBuf, Path};
use structopt::StructOpt;
use ff_rt::game::{Match, ActiveState, MovePair, Player};
use std::iter;
use ff_rt::game::{GAME_FIELD_SIZE, EndState};
use itertools::Itertools;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    silent: bool,
    p1exe: PathBuf,
    p2exe: PathBuf,
}

fn main() {
    b_error::main(run)
}

fn run() -> BResult<()> {
    env_logger::init();
    
    let opts = Opts::from_args();

    let p1exe = load_file(&opts.p1exe)
        .ec("loading player 1 exe")?;
    let p2exe = load_file(&opts.p2exe)
        .ec("loading player 2 exe")?;

    let match_res = ff_rt::run_match(&p1exe, &p2exe)?;

    if opts.silent {
        for (i, game) in match_res.games.iter().enumerate() {
           println!("{} ({})", game.end.winner(), game.end.explain());
        }
    } else {
        print_match_results(&match_res);
    }

    Ok(())
}

fn load_file(path: &Path) -> BResult<Bytes> {
    let mut file = File::open(path).ec("opening exe")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ec("reading exe")?;
    let buffer = Bytes::from(buffer);
    Ok(buffer)
}

fn print_match_results(match_res: &Match) {
    println!();
    println!("match results:");
    for (i, game) in match_res.games.iter().enumerate() {
        println!(" game {}:", i);
        for (i, turn) in game.turns.iter().enumerate() {
            let grid = graphic_grid(&turn.state);
            let energy = graphic_energy(&turn.state);
            let moves = graphic_moves(&turn.moves);
            println!("  turn {:2}: {} (e {}) (m {})", i, grid, energy, moves);
            //println!("  energy   : {}", energy);
            //println!("  moves    : {}", moves);
        }
        println!("  -------");
        let grid = graphic_end_grid(&game.end);
        let energy = graphic_energy(&game.end.inner_state());
        let winner = game.end.winner();
        let reason = game.end.explain();
        println!("  end    : {}", grid);
        //println!("  energy: {}", energy);
        println!("  winner : {} ({})", winner, reason);
    }
    println!();
}

fn graphic_grid(s: &ActiveState) -> String {
    let s: String = iter::repeat('_')
        .take(GAME_FIELD_SIZE as usize)
        .enumerate()
        .map(|(i, blank)| {
            if i == s.p1.pos as usize {
                '1'
            } else if i == s.p2.pos as usize {
                '2'
            } else {
                blank
            }
        })
        .intersperse('|')
        .collect();

    format!("|{}|", s)
}

fn graphic_end_grid(s: &EndState) -> String {
    let s: String = iter::repeat('_')
        .take(GAME_FIELD_SIZE as usize)
        .enumerate()
        .map(|(i, blank)| {
            if s.victor() == None {
                if i == s.inner_state().p1.pos as usize {
                    '1'
                } else if i == s.inner_state().p2.pos as usize {
                    '2'
                } else {
                    blank
                }
            } else {
                blank
            }
        })
        .enumerate()
        .map(|(i, what)| {
            if s.victor() == Some(Player::P1) {
                if i == s.inner_state().p2.pos as usize {
                    '2'
                } else {
                    what
                }
            } else {
                what
            }
        })
        .enumerate()
        .map(|(i, what)| {
            if s.victor() == Some(Player::P2) {
                if i == s.inner_state().p1.pos as usize {
                    '1'
                } else {
                    what
                }
            } else {
                what
            }
        })
        .intersperse('|')
        .collect();

    format!("|{}|", s)
}

fn graphic_energy(s: &ActiveState) -> String {
    format!("{:05} | {:05}", s.p1.energy, s.p2.energy)
}

fn graphic_moves(s: &MovePair) -> String {
    let p1 = s.p1.kind.as_str();
    let p2 = s.p2.kind.as_str();
    format!("{:7} | {:7}", p1, p2)
}
