#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(asm)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod multiboot;
mod vga;
mod serial;
mod uart_16550;

#[no_mangle]
pub extern "C" fn main() -> ! {
    println!("some numbers: {} {}", 42, 1.337);

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    unsafe {
        asm!(
            "out dx, al",
            in("rdx") 0xf4,
            in("rax") 0x11
        )
    }
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    unsafe {
        asm!(
            "out dx, al",
            in("rdx") 0xf4,
            in("rax") 0x10
        )
    }
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}