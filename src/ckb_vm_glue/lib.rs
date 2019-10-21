#![no_std]
#![feature(lang_items)]

use ckb_vm_syscall as syscall;

pub const ECALL_EXIT: usize = 93;

const PANIC_EXIT_CODE: i32 = 102;
const ABORT_EXIT_CODE: i32 = 103;

pub fn exit(code: i32) -> ! {
    unsafe { syscall::ecall1(ECALL_EXIT, code as usize) };
    unreachable!()
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    exit(PANIC_EXIT_CODE)
}

// HACK: If a linking binary doesn't explicitly reference this crate, then rustc
// isn't smart enough to pull in the various lang items defined here and ends up
// erroring. Asking the user to call this function gives an excuse to reference
// something in this library.
pub fn init() { }

/*use core::any::Any;

#[lang = "begin_panic"]
#[inline(never)]
#[cold]
pub fn begin_panic<M: Any + Send>(msg: M, file_line_col: &(&'static str, u32, u32)) -> ! {
    abort()
}*/
 

// The abort intrinsic for RISC-V apparently just calls this function
#[no_mangle]
pub fn abort() -> ! {
    exit(ABORT_EXIT_CODE)
}

#[lang = "start"]
fn lang_start<T: Termination + 'static>
    (main: fn() -> T, argc: isize, argv: *const *const u8) -> isize
{
    lang_start_internal(&move || main().report(), argc, argv)
}

fn lang_start_internal(main: &(dyn Fn() -> i32),
                       _argc: isize, _argv: *const *const u8) -> isize {
    exit(main());
}

pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 { 0 }
}

// HACK: Unlike seemingly every other architecture, it seems we must define
// a _start function to act is the entry point on RISC-V. This one immediately
// calls the main function emitted by rustc, the function rustc things is the
// application entry point. I don't really understand what's going on here. It
// must normally be provided by some system startup object not provided by
// rustc.
//
// FIXME: Probably need to use assembly here to access the program arguments
#[no_mangle]
pub fn _start(argc: i32, argv: *const *const u8) -> i32 {
    extern "C" {
        fn main(argc: i32, argv: *const *const u8) -> i32;
    }

    unsafe { main(argc, argv) }
}
