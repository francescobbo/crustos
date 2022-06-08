use core::marker::PhantomData;

pub struct Port<T> {
    port: u16,
    unused: PhantomData<T>
}

impl<T> Port<T> {
    #[inline]
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port: port,
            unused: PhantomData
        }
    }
}

impl Port<u8> {
    #[inline]
    pub unsafe fn read(&self) -> u8 {
        let value: u8;
        asm!("in al, dx", out("al") value, in("dx") self.port, options(nomem, nostack, preserves_flags));
        value
    }

    #[inline]
    pub unsafe fn write(&self, value: u8) {
        asm!("out dx, al", in("dx") self.port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

impl Port<u16> {
    #[inline]
    pub unsafe fn read(&self) -> u16 {
        let value: u16;
        asm!("in ax, dx", out("ax") value, in("dx") self.port, options(nomem, nostack, preserves_flags));
        value
    }

    #[inline]
    pub unsafe fn write(&self, value: u16) {
        asm!("out dx, ax", in("dx") self.port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
}

impl Port<u32> {
    #[inline]
    pub unsafe fn read(&self) -> u32 {
        let value: u32;
        asm!("in eax, dx", out("eax") value, in("dx") self.port, options(nomem, nostack, preserves_flags));
        value
    }

    #[inline]
    pub unsafe fn write(&self, value: u32) {
        asm!("out dx, eax", in("dx") self.port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}

pub struct PortRange<T> {
    start: u16,
    size: u16,
    unused: PhantomData<T>
}

impl<T> PortRange<T> {
    pub const fn new(start: u16, size: u16) -> PortRange<T> {
        PortRange { start: start, size: size, unused: PhantomData }
    }
}

impl PortRange<u8> {
    pub unsafe fn read(&self, index: u16) -> u8 {
        assert!(index < self.size);

        let port: Port<u8> = Port::new(self.start + index);
        port.read()
    }

    pub unsafe fn write(&self, index: u16, value: u8) {
        assert!(index < self.size);

        let port: Port<u8> = Port::new(self.start + index);
        port.write(value)
    }
}

impl PortRange<u16> {
    pub unsafe fn read(&self, index: u16) -> u16 {
        assert!(index < self.size);

        let port: Port<u16> = Port::new(self.start + index);
        port.read()
    }

    pub unsafe fn write(&self, index: u16, value: u16) {
        assert!(index < self.size);

        let port: Port<u16> = Port::new(self.start + index);
        port.write(value)
    }
}

impl PortRange<u32> {
    pub unsafe fn read(&self, index: u16) -> u32 {
        assert!(index < self.size);

        let port: Port<u32> = Port::new(self.start + index);
        port.read()
    }

    pub unsafe fn write(&self, index: u16, value: u32) {
        assert!(index < self.size);

        let port: Port<u32> = Port::new(self.start + index);
        port.write(value)
    }
}

pub const PS2: PortRange<u8> = PortRange::new(0x60, 4);
pub const PS2_CTRL_A: Port<u8> = Port::new(0x92);
pub const PIT: PortRange<u8> = PortRange::new(0x40, 8);
pub const CMOS: PortRange<u8> = PortRange::new(0x70, 2);
pub const FIRST_PIC: PortRange<u8> = PortRange::new(0x20, 2);
pub const SECOND_PIC: PortRange<u8> = PortRange::new(0xa0, 2);
pub const FIRST_DMA: PortRange<u8> = PortRange::new(0x00, 16);
pub const SECOND_DMA: PortRange<u8> = PortRange::new(0xc0, 16);

pub const COM1: PortRange<u8> = PortRange::new(0x3f8, 8);
pub const COM2: PortRange<u8> = PortRange::new(0x2f8, 8);
pub const COM3: PortRange<u8> = PortRange::new(0x3e8, 8);
pub const COM4: PortRange<u8> = PortRange::new(0x2e8, 8);
pub const COM5: PortRange<u8> = PortRange::new(0x5f8, 8);
pub const COM6: PortRange<u8> = PortRange::new(0x4f8, 8);
pub const COM7: PortRange<u8> = PortRange::new(0x5e8, 8);
pub const COM8: PortRange<u8> = PortRange::new(0x4e8, 8);