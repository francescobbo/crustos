pub struct Descriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    limit_flags: u8,
    base_hi: u8
}