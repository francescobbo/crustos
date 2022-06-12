struct Segment {
    offset_low: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
}

type HandlerFunc = extern "x86-interrupt" fn(_: InterruptStackFrame);