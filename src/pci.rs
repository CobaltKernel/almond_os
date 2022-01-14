//! PCI Device API
//! <https://wiki.osdev.org/PCI>

use core::fmt::Display;

use alloc::string::String;
use x86_64::instructions::port::*;

const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeviceConfig {
    bus: u8,
    slot: u8,
    func: u8,

    vendor_id: u16,
    device_id: u16,

    bars: [u16; 6],

    irq_line: u8,
}

impl DeviceConfig {
    pub fn load(bus: u8, slot: u8, func: u8) -> Option<DeviceConfig> {
        if read_word(bus, slot, func, 0) == 0xFFFF {return None};

        Some(Self {

            bus, slot, func,

            vendor_id: read_word(bus, slot, func, 0),
            device_id: read_word(bus, slot, func, 2),
            bars: [
                read_word(bus, slot, func, 0x10),
                read_word(bus, slot, func, 0x14),
                read_word(bus, slot, func, 0x18),
                read_word(bus, slot, func, 0x1C),
                read_word(bus, slot, func, 0x20),
                read_word(bus, slot, func, 0x24),
            ],

            irq_line: (read_word(bus, slot, func, 0x3E) & 0xFF) as u8,
        })
    }

    pub fn vendor(&self) -> u16 {
        self.vendor_id
    }

    pub fn device(&self) -> u16 {
        self.device_id
    }

    pub fn bars(&self) -> [u16; 6] {
        self.bars
    }
}

impl Display for DeviceConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{} - {:04x} - {:04x} - {}/{}", self.bus, self.slot, self.func, self.vendor_id, self.device_id, vendor_str(self.vendor_id), device_str(self.device_id))
    }
}


pub fn read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let mut data_port: Port<u32> = Port::new(CONFIG_DATA);
    let mut address_port: Port<u32> = Port::new(CONFIG_ADDRESS);

    let address: u32 = pci_address(bus, slot, func, offset);

    let offset = offset as u32;

    unsafe {address_port.write(address);}

    unsafe {(data_port.read() >> (((offset & 2) * 8) & 0xFFFF)) as u16}
}

pub fn pci_address(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let bus = bus as u32;
    let slot = slot as u32;
    let func = func as u32;
    let offset = offset as u32;
    (bus << 16) | (slot << 11) | (func << 8) | (offset & 0xFC) | (0x8000_0000)
}

pub fn vendor_str(id: u16) -> String {
    match id {
        0x1234 => "QEMU".into(),
        0x8086 => "INTEL".into(),
        _ => "UNKNOWN VENDOR".into(),
    }
}

pub fn device_str(id: u16) -> String {
    match id {
        0x1111 => "QEMU VGA Controller".into(),
        0x100e => "82540EM Gigabit Ethernet Controller".into(),
        0x7000 => "82371SB PIIX3 ISA".into(),
        0x7010 => "82371SB PIIX3 IDE".into(),
        0x7020 => "82371SB PIIX3 USB".into(),
        0x7113 => "82371AB/EB/MB PIIX4".into(),
        _ => "UNKNOWN VENDOR".into(),
    }
}