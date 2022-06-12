use core::arch::asm;

#[inline]
pub fn disable() {
    unsafe {
        asm!("cli", options(nomem, nostack))
    }
}

#[inline]
pub fn enable() {
    unsafe {
        asm!("sti", options(nomem, nostack))
    }
}

#[inline]
pub fn enable_and_yield() {
    unsafe {
        asm!("sti
              hlt",
              options(nomem, nostack))
    }
}