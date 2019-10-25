#![no_std]

#[panic_handler]
pub fn panic_fmt(_: &::core::panic::PanicInfo) -> ! {
    loop { }
}

fn main() {
}
