#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;
mod multiboot;
mod vga;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn main() -> ! {
    println!("some numbers: {} {}", 42, 1.337);
    panic!("I was not expecting that");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}