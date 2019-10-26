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

    emit_match_results(&match_res)?;

    Ok(())
}

fn load_file(path: &Path) -> BResult<Bytes> {
    let mut file = File::open(path).ec("opening exe")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ec("reading exe")?;
    let buffer = Bytes::from(buffer);
    Ok(buffer)
}

fn emit_match_results(match_res: &Match) -> BResult<()> {
    let s = serde_json::to_string_pretty(match_res).ec("emitting json")?;
    println!("{}", s);
    Ok(())
}
