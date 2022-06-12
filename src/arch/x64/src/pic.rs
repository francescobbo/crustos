use crate::ports::{FIRST_PIC, SECOND_PIC};

pub struct PIC {}

const COMMAND: u16 = 0;
const DATA: u16 = 1;

const COMMAND_GET_IRR: u8 = 0x0a;
const COMMAND_GET_ISR: u8 = 0x0b;
const COMMAND_INIT: u8 = 0x10;
const COMMAND_EOI: u8 = 0x20;

const ICW1_HAS_ICW4: u8 = 0x01;
const ICW4_X86: u8 = 0x01;

impl PIC {
    /// Initializes the two PICs in a primary/secondary setup.
    ///
    /// The PICs are configured in the only reasonable setup in a modern system.
    /// 
    /// After the setup the PICs will be fully masked (all IRQs ignored).
    /// 
    /// The IRQs are set to be sent to the bootstrap CPU starting at Interrupt vector `offset`.
    pub unsafe fn init(offset: u8) {
        debug_assert!(offset >= 32);

        // Initialize the two PICs:
        //   Cascade mode, Edge triggered, with an ICW4
        FIRST_PIC.write(COMMAND, COMMAND_INIT | ICW1_HAS_ICW4);
        SECOND_PIC.write(COMMAND, COMMAND_INIT | ICW1_HAS_ICW4);

        // ICW2: Interrupt vector destination
        // TODO: check if bits 0-2 can be set
        FIRST_PIC.write(DATA, offset);
        SECOND_PIC.write(DATA, offset + 8);

        // ICW3: Cascade configuration
        //   On the primary, set IRQ2 to be the cascading one
        //   On the secondary, set it as PIC #2. In the original design there might have been up to 8 PICs in a system.
        FIRST_PIC.write(DATA, 1 << 2);
        SECOND_PIC.write(DATA, 2);

        // IWC4:
        //   x86 mode, normal EOI, non-buffered, non-special-fully-nested
        FIRST_PIC.write(DATA, ICW4_X86);
        SECOND_PIC.write(DATA, ICW4_X86);
    }

    /// Masks all IRQs on both PICs found on a modern x64 system.
    /// This effectively disables the PICs. Useful if the better IO-APIC is going to be used.
    #[inline]
    pub fn disable() {
        unsafe {
            FIRST_PIC.write(DATA, 0xff);
            SECOND_PIC.write(DATA, 0xff);
        }
    }
    
    /// Sends a generic EOI to one or both PICs (depending on the IRQ number).
    /// This allows a new IRQ to be processed.
    /// 
    /// The `vector` is used to determine which PIC needs to receive an EOI.
    #[inline]
    pub unsafe fn eoi(vector: u8) {
        debug_assert!(vector < 16);

        if vector >= 8 {
            SECOND_PIC.write(COMMAND, COMMAND_EOI);
        }

	    FIRST_PIC.write(COMMAND, COMMAND_EOI);
    }

    /// Returns the content of the two PIC IMRs. These bytes have bits set if the corresponding IRQs is masked (ignored).
    /// 
    /// The first byte is the IMR for the primary PIC.
    #[inline]
    pub fn get_mask() -> (u8, u8) {
        unsafe {
            (FIRST_PIC.read(DATA), SECOND_PIC.read(DATA))
        }
    }

    /// Sets the two PIC IMRs. The two arguments are the masks for the two PICs.
    #[inline]
    pub unsafe fn set_mask(primary_imr: u8, secondary_imr: u8) {
        FIRST_PIC.write(DATA, primary_imr);
        SECOND_PIC.write(DATA, secondary_imr);
    }

    /// Returns the content of the two PIC IRRs. These bytes have bits set if the corresponding IRQs has been received
    /// by the PIC but not yet sent to the CPU. This would happen because a previous interrupt is still "In Service"
    /// (an EOI has not been sent).
    /// 
    /// The first byte is the ISR for the primary PIC.
    #[inline]
    pub fn get_irr() -> (u8, u8) {
        unsafe {
            FIRST_PIC.write(COMMAND, COMMAND_GET_IRR);
            SECOND_PIC.write(COMMAND, COMMAND_GET_IRR);
            (FIRST_PIC.read(COMMAND), SECOND_PIC.read(COMMAND))
        }
    }

    /// Returns the content of the two PIC ISRs. These bytes have bits set if the corresponding IRQs has been delivered
    /// to the CPU but it has not been yet completed processing with an EOI.
    /// 
    /// The first byte is the ISR for the primary PIC.
    #[inline]
    pub fn get_isr() -> (u8, u8) {
        unsafe {
            FIRST_PIC.write(COMMAND, COMMAND_GET_ISR);
            SECOND_PIC.write(COMMAND, COMMAND_GET_ISR);
            (FIRST_PIC.read(COMMAND), SECOND_PIC.read(COMMAND))
        }
    }

    /// Unmasks (enables) an IRQ line. If the IRQ is served by the secondary PIC, the cascading IRQ line is also 
    /// unmasked.
    pub unsafe fn enable_irq(vector: u8) {
        debug_assert!(vector < 16);

        let (mut p, mut s) = PIC::get_mask();
        if vector >= 8 {
            s &= !(1 << (vector - 8));

            // Also unmask IRQ2 (slave cascade)
            p &= !(1 << 2);
        } else {
            p &= !(1 << vector);
        }

        PIC::set_mask(p, s);
    }

    /// Masks (disables) an IRQ line.
    pub unsafe fn disable_irq(vector: u8) {
        debug_assert!(vector < 16);

        let (mut p, mut s) = PIC::get_mask();
        if vector >= 8 {
            s |= 1 << (vector - 8);
        } else {
            p |= 1 << vector;
        }

        PIC::set_mask(p, s);
    }
}