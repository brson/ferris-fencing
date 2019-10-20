#![allow(unused)]
#![no_std]

use ckb_vm_syscall::{ecall4, ecall1};

fn main() {
    ckb_vm_glue::init();

    loop {
        let mut p1_pos = 0;
        let mut p2_pos = 0;
        let mut p1_energy = 0;
        let mut p2_energy = 0;
        e_state(&mut p1_pos, &mut p2_pos,
                &mut p1_energy, &mut p2_energy);
        e_move(1);
    }
}

const ECALL_STATE: usize = 0x0100;
const ECALL_MOVE: usize =  0x0101;

fn e_state(p1_pos: &mut i32, p2_pos: &mut i32,
           p1_energy: &mut i32, p2_energy: &mut i32) -> i32 {
    let p1_pos = (p1_pos as *mut _) as usize;
    let p2_pos = (p2_pos as *mut _) as usize;
    let p1_energy = (p1_energy as *mut _) as usize;
    let p2_energy = (p2_energy as *mut _) as usize;

    let r = unsafe { ecall4(ECALL_STATE, p1_pos, p2_pos, p1_energy, p2_energy) };

    r as _
}

fn e_move(kind: i32) -> i32 {
    let kind = kind as usize;
    let r = unsafe { ecall1(ECALL_MOVE, kind) };

    r as _
}
