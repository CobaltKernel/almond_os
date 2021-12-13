//! Kernel Debugger

use alloc::string::String;
use x86_64::{registers::control::{Cr0Flags, Cr2, Cr3Flags, Cr4Flags}, VirtAddr, structures::paging::{PageTable, Size4KiB, PhysFrame}};

pub mod disassembler;

/// See [disassembler::disassemble]
pub fn disassemble(addr: usize, len: usize) -> String {
    disassembler::disassemble(addr, len)
}

/// Read The Value Of EAX. Clobbers ECX
pub fn read_eax() -> i32 {
    let mut eax: i32;
    let mut tmp: i32;
    unsafe {
        asm!("mov eax, eax", out("eax") eax);
    }
    eax
}

/// Read The Value Of EBX. Clobbers ECX
pub fn read_ebx() -> i32 {
    let mut ebx: i32;
    let mut tmp: i32;
    unsafe {
        asm!("mov ecx, ecx", out("ecx") tmp);
        asm!("mov ecx, ebx", out("ecx") ebx);
        asm!("mov ecx, ecx", in("ecx") tmp);
    }
    ebx
}

/// Read The Value Of ECX. Clobbers EAX
pub fn read_ecx() -> i32 {
    let mut ecx: i32;
    let mut tmp: i32;
    unsafe {
        asm!("mov ecx, ecx", out("ecx") ecx);

    }
    ecx
}



/// Read The Value Of RIP
#[inline(always)]
pub fn read_rip() -> u64 {
    x86_64::registers::read_rip().as_u64()
}

/// Read CR0
pub fn read_cr0_raw() -> u64 {
    x86_64::registers::control::Cr0::read_raw()
}

/// Read CR2
pub fn read_cr2_raw() -> u64 {
    x86_64::registers::control::Cr2::read().as_u64()
}

/// Read CR3
pub fn read_cr3_raw() -> u64 {
    x86_64::registers::control::Cr3::read_raw().0.start_address().as_u64()
}

/// Read CR4
pub fn read_cr4_raw() -> u64 {
    x86_64::registers::control::Cr4::read_raw()
}

/// Read CR0
pub fn read_cr0() -> Cr0Flags {
    x86_64::registers::control::Cr0::read()
}

/// Read CR2
pub fn read_cr2() -> VirtAddr {
    x86_64::registers::control::Cr2::read()
}

/// Read CR3
pub fn read_cr3() -> (PhysFrame<Size4KiB>, Cr3Flags) {
    x86_64::registers::control::Cr3::read()
}

/// Read CR4
pub fn read_cr4() -> Cr4Flags {
    x86_64::registers::control::Cr4::read()
}