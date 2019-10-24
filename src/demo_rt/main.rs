use std::path::PathBuf;
use std::fs;
use bytes::Bytes;
use structopt::StructOpt;
use ckb_vm::{
    SparseMemory, DefaultCoreMachine, SupportMachine,
    WXorXMemory, DefaultMachineBuilder, DefaultMachine,
};
use ckb_vm::decoder::build_imac_decoder;
use b_error::{BResult, ResultExt};

#[derive(StructOpt)]
struct Opts {
    exe: PathBuf,
}

fn main() {
    b_error::main(run)
}

fn run() -> BResult<()> {
    env_logger::init();
    let opts = Opts::from_args();
    let exe = fs::read(&opts.exe).ec("reading exe")?;
    let exe = Bytes::from(exe);
    let decoder = build_imac_decoder::<u32>();

    let mut vm = make_vm();
    vm.load_program(&exe, &[]).ec("loading exe")?;
    vm.set_running(true);
    while vm.running() {
        vm.step(&decoder).ec("stepping")?;
    }
    Ok(())
}

type MyCoreMachine = DefaultCoreMachine<u32, WXorXMemory<u32, SparseMemory<u32>>>;
type MyMachine<'a> = DefaultMachine<'a, MyCoreMachine>;

fn make_vm<'a>() -> MyMachine<'a> {
    let core_machine = MyCoreMachine::default();
    let builder = DefaultMachineBuilder::new(core_machine);
    let machine = builder.build();
    machine
}
