use crate::arch::x86_64::io::{Port};

use core::fmt;
use bitflags::bitflags;


macro_rules! wait_for {
    ($cond:expr) => {
        while !$cond {
            core::hint::spin_loop()
        }
    };
}

bitflags! {
    /// Interrupt enable flags
    struct InterruptEnableFlags: u8 {
        const NONE = 0;

        const RECEIVED = 1;
        const SENT = 1 << 1;
        const ERRORED = 1 << 2;
        const STATUS_CHANGE = 1 << 3;
        
        // 4 to 7 are unused
    }

    struct LineControl: u8 {
        const NONE = 0;

        const CHAR6 = 1;
        const CHAR7 = 2;
        const CHAR8 = 3;

        const ODD = 1 << 3;
        const EVEN = 2 << 3;
        const MARK = 5 << 3;
        const SPACE = 7 << 3;

        const DLAB = 1 << 7;
    }

    /// Line status flags
    struct LineStatusFlags: u8 {
        const INPUT_FULL = 1;
        // 1 to 4 unknown
        const OUTPUT_EMPTY = 1 << 5;
        // 6 and 7 unknown
    }
}


/// A port-mapped UART.
pub struct SerialPort {
    data: Port<u8>,
    int_en: Port<u8>,
    fifo_ctrl: Port<u8>,
    line_ctrl: Port<u8>,
    modem_ctrl: Port<u8>,
    line_sts: Port<u8>,
}

impl SerialPort {
    /// Creates a new serial port interface on the given I/O port.
    pub fn new(base: u16) -> Self {
        match base {
            0x2e8 | 0x2f8 | 0x3e8 | 0x3f8 | 0x4e8 | 0x4f8 | 0x5e8 | 0x5f8 => {},
            _ => panic!("Invalid COM address {:x}", base)
        }

        Self {
            data:       Port::<u8>::new(base),
            int_en:     Port::<u8>::new(base + 1),
            fifo_ctrl:  Port::<u8>::new(base + 2),
            line_ctrl:  Port::<u8>::new(base + 3),
            modem_ctrl: Port::<u8>::new(base + 4),
            line_sts:   Port::<u8>::new(base + 5),
        }
    }

    /// Initializes the serial port.
    ///
    /// The default configuration of [38400/8-N-1](https://en.wikipedia.org/wiki/8-N-1) is used.
    pub fn init(&mut self) {
        unsafe {
            // Disable interrupts
            self.int_en.write(InterruptEnableFlags::NONE.bits());

            // Enable DLAB - this changes the meaning of the data and InterruptEnable Registers
            self.line_ctrl.write(LineControl::DLAB.bits());

            // Set maximum speed to 38400 bps by configuring DLL and DLM
            self.data.write(0x03);
            self.int_en.write(0x00);

            // Disable DLAB and set data word length to 8 bits
            self.line_ctrl.write(LineControl::CHAR8.bits());

            // Enable FIFO, clear TX/RX queues and
            // set interrupt watermark at 14 bytes
            self.fifo_ctrl.write(0xC7);

            // Mark data terminal ready, signal request to send
            // and enable auxilliary output #2 (used as interrupt line for CPU)
            self.modem_ctrl.write(0x0B);

            // Enable interrupts
            self.int_en.write(InterruptEnableFlags::RECEIVED.bits());
        }
    }

    fn line_sts(&mut self) -> LineStatusFlags {
        unsafe { LineStatusFlags::from_bits_truncate(self.line_sts.read()) }
    }

    /// Sends a byte on the serial port.
    pub fn send(&mut self, data: u8) {
        match data {
            8 | 0x7F => {
                self.send_raw(b'\x08');
                self.send_raw(b' ');
                self.send_raw(b'\x08')
            }
            _ => {
                self.send_raw(data)
            }
        }
    }

    /// Sends a raw byte on the serial port, intended for binary data.
    pub fn send_raw(&mut self, data: u8) {
        unsafe {
            wait_for!(self.line_sts().contains(LineStatusFlags::OUTPUT_EMPTY));
            self.data.write(data);
        }
    }

    /// Receives a byte on the serial port.
    pub fn receive(&mut self) -> u8 {
        unsafe {
            wait_for!(self.line_sts().contains(LineStatusFlags::INPUT_FULL));
            self.data.read()
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}