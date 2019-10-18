#![allow(unused)]

use bytes::Bytes;
use std::io::Read;
use std::fs::File;
use b_error::{BResult, ResultExt};
use std::path::{PathBuf, Path};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    p1exe: PathBuf,
    p2exe: PathBuf,
}

fn main() {
    b_error::main(run)
}

fn run() -> BResult<()> {
    let opts = Opts::from_args();

    let p1exe = load_file(&opts.p1exe)
        .ec("loading player 1 exe")?;
    let p2exe = load_file(&opts.p2exe)
        .ec("loading player 2 exe")?;

    Ok(())
}

fn load_file(path: &Path) -> BResult<Bytes> {
    let mut file = File::open(path).e()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).e()?;
    let buffer = Bytes::from(buffer);
    Ok(buffer)
}
