use core::arch::asm;

// #[inline]
// pub unsafe fn lgdt(gdt: &DescriptorTablePointer) {
//     unsafe {
//         asm!("lgdt [{}]", in(reg) gdt, options(readonly, nostack, preserves_flags));
//     }
// }

// #[inline]
// pub unsafe fn lidt(idt: &DescriptorTablePointer) {
//     unsafe {
//         asm!("lidt [{}]", in(reg) idt, options(readonly, nostack, preserves_flags));
//     }
// }

// #[inline]
// pub fn sgdt() -> DescriptorTablePointer {
//     let mut gdt: DescriptorTablePointer = DescriptorTablePointer {
//         limit: 0,
//         base: VirtAddr::new(0),
//     };
//     unsafe {
//         asm!("sgdt [{}]", in(reg) &mut gdt, options(nostack, preserves_flags));
//     }
//     gdt
// }

// #[inline]
// pub fn sidt() -> DescriptorTablePointer {
//     let mut idt: DescriptorTablePointer = DescriptorTablePointer {
//         limit: 0,
//         base: VirtAddr::new(0),
//     };
//     unsafe {
//         asm!("sidt [{}]", in(reg) &mut idt, options(nostack, preserves_flags));
//     }
//     idt
// }

// #[inline]
// pub unsafe fn load_tss(sel: SegmentSelector) {
//     unsafe {
//         asm!("ltr {0:x}", in(reg) sel.0, options(nostack, preserves_flags));
//     }
// }