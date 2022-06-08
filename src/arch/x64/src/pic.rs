use crate::ports::{FIRST_PIC, SECOND_PIC};

pub struct PIC {}

impl PIC {
    pub unsafe fn init(offset: u8) {
        assert!(offset >= 32);

        FIRST_PIC.write(0, 1);
    }

    pub unsafe fn eoi(vector: u8) {
        assert!(vector < 16);

        if vector >= 8 {
            SECOND_PIC.write(0, 0x20);
        }

	    FIRST_PIC.write(0, 0x20);
    }

    pub fn get_mask() -> (u8, u8) {
        unsafe {
            (FIRST_PIC.read(1), SECOND_PIC.read(1))
        }
    }

    pub unsafe fn set_mask(first: u8, second: u8) {
        FIRST_PIC.write(1, first);
        SECOND_PIC.write(1, second);
    }
}