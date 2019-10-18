#![allow(unused)]

use bytes::Bytes;
use b_error::{BResult, ResultExt};
use ckb_vm::{
    SparseMemory, Register, Memory, TraceMachine, DefaultCoreMachine,
    WXorXMemory, DefaultMachineBuilder,
};
use crate::game::Game;

mod game;

pub fn run_game(p1exe: &Bytes, p2exe: &Bytes) -> BResult<Game> {
    panic!()
}    

fn run_vm(program: &Bytes, args: &[Bytes]) -> BResult<i8> {
    run_vm_mem::<u32, SparseMemory<u32>>(program, args)
}

fn run_vm_mem<R: Register, M: Memory<R> + Default>(
    program: &Bytes,
    args: &[Bytes],
) -> BResult<i8> {
    let core_machine = DefaultCoreMachine::<R, WXorXMemory<R, M>>::default();
    let builder = DefaultMachineBuilder::new(core_machine);
    let machine = builder.build();
    let mut machine = TraceMachine::new(machine);
    machine.load_program(program, args).e()?;
    Ok(machine.run().e()?)
}
