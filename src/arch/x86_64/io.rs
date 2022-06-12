use core::arch::asm;
use core::fmt;
use core::marker::PhantomData;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Port<T> {
    port: u16,
    unused: PhantomData<T>
}

impl<T> Port<T> {
    pub const fn new(port: u16) -> Port<T> {
        Port { port: port, unused: PhantomData }
    }
}

impl<T> fmt::Debug for Port<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Port")
            .field("port", &self.port)
            .field("size", &core::mem::size_of::<T>())
            .finish()
    }
}

impl Port<u8> {
    #[inline]
    pub unsafe fn read(&self) -> u8 {
        let value: u8;
        asm!(
            "in al, dx",
            out("al") value,
            in("dx") self.port,
            options(nomem, nostack, preserves_flags)
        );
        value
    }

    #[inline]
    pub unsafe fn write(&self, value: u8) {
        asm!(
            "out dx, al",
            in("dx") self.port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

impl Port<u16> {
    #[inline]
    pub unsafe fn read(&self) -> u16 {
        let value: u16;
        asm!(
            "in ax, dx",
            out("ax") value,
            in("dx") self.port,
            options(nomem, nostack, preserves_flags)
        );
        value
    }

    #[inline]
    pub unsafe fn write(&self, value: u16) {
        asm!(
            "out dx, ax",
            in("dx") self.port,
            in("ax") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

impl Port<u32> {
    #[inline]
    pub unsafe fn read(&self) -> u32 {
        let value: u32;
        asm!(
            "in eax, dx",
            out("eax") value,
            in("dx") self.port,
            options(nomem, nostack, preserves_flags)
        );
        value
    }

    #[inline]
    pub unsafe fn write(&self, value: u32) {
        asm!(
            "out dx, eax",
            in("dx") self.port,
            in("eax") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}