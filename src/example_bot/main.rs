#![allow(unused)]
#![no_std]

use ckb_vm_syscall::{ecall4, ecall1};

fn main() {
    ckb_vm_glue::init();

    const MOVE_BACK: i32 = 1;
    const MOVE_STAND: i32 = 2;
    const MOVE_FORWARD: i32 = 3;
    const MOVE_LUNGE: i32 = 4;

    loop {
        let mut my_pos = 0;
        let mut other_pos = 0;
        let mut my_energy = 0;
        let mut other_energy = 0;
        let r = e_state(&mut my_pos, &mut other_pos,
                        &mut my_energy, &mut other_energy);
        assert!(r == 0);

        assert!(other_pos - my_pos > 0);
        let sep = other_pos - my_pos - 1;
        let mut next_move = MOVE_FORWARD;
        if sep == 1 && my_pos >= 3 {
            next_move = MOVE_BACK;
        } else if sep == 2 {
            next_move = MOVE_LUNGE;
        }

        let r = e_move(next_move);
        assert!(r == 0);
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
