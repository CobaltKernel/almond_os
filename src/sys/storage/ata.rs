//! Interfaces With ATA PIO-LBA24 Disks.
//! https://wiki.osdev.org/ATA_PIO_Mode
use core::fmt::Debug;

use alloc::string::String;
use bit_field::BitField;
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};

use crate::{
    log,
    sys::{
        terminal::Spinner,
        timer::{self, sleep_ticks, uptime},
    },
    KResult,
};

/// Index A Sector On A Disk. Bits 0:28 Are Used,
pub type SectorIndex = u32;

/// ATA Commands
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Command {
    NOP = 0x00,
    ReadSectors = 0x20,
    WriteSectors = 0x30,
    Indentify = 0xEC,
}
#[allow(unused)]
impl Command {
    pub fn into(&self) -> u8 {
        unsafe { *(self as *const Command) as u8 }
    }

    pub fn from(value: u8) -> Command {
        match value {
            0xEC => Command::Indentify,
            0x20 => Command::ReadSectors,
            0x30 => Command::WriteSectors,
            _ => Command::NOP,
        }
    }
}

/// ATA I/O Registers
///
/// [OSDev Wiki](https://wiki.osdev.org/ATA_PIO_Mode#Registers)
#[derive(Debug)]
#[allow(unused)]
struct IoRegisters {
    data: Port<u16>,            // Offset 0,
    error: PortReadOnly<u8>,    // Offset 1, READ-ONLY
    features: PortReadOnly<u8>, // Offset 1, WRITE-ONLY
    sector_count: Port<u8>,     // Offset 2,
    lba_lo: Port<u8>,           // Offset 3,
    lba_mid: Port<u8>,          // Offset 4,
    lba_hi: Port<u8>,           // Offset 5,
    drive: Port<u8>,            // Offset 6,
    status: PortReadOnly<u8>,   // Offset 7, READ-ONLY
    command: PortWriteOnly<u8>, // Offset 7, WRITE-ONLY
}

#[derive(Debug)]
#[allow(unused)]
struct ControlRegisters {
    alt_status: PortReadOnly<u8>,   // Offset 0, READ-ONLY
    dev_control: PortWriteOnly<u8>, // Offset 0, WRITE-ONLY
    drv_address: PortWriteOnly<u8>, // Offset 1,
}
#[derive(Debug)]
struct Bus {
    io_reg: IoRegisters,
    ctrl_reg: ControlRegisters,
}

#[derive(Debug)]
/// A Single ATA Disk
pub struct Disk {
    /// Bus ID (0|1)
    pub bus: u8,
    /// Disk ID (0|1)
    pub disk: u8,
    sectors: SectorIndex,

    ata_bus: Bus,
}

struct Status {
    pub value: u8,
    pub error: bool,
    pub index: bool,
    pub corrected: bool,
    pub drive_request: bool,
    pub service_request: bool,
    pub drive_fault: bool,
    pub ready: bool,
    pub busy: bool,
}

impl Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Status {{ ")?;
        write!(f, "{}", if self.busy { 'B' } else { '-' })?;
        write!(f, "{}", if self.ready { 'R' } else { '-' })?;
        write!(f, "{}", if self.drive_fault { "DF" } else { "--" })?;
        write!(f, "{}", if self.service_request { "SR" } else { "--" })?;
        write!(f, "{}", if self.drive_request { "DR" } else { "--" })?;
        write!(f, "{}", if self.corrected { "C" } else { "-" })?;
        write!(f, "{}", if self.index { "I" } else { "-" })?;
        write!(f, "{}", if self.error { "E" } else { "-" })?;
        write!(f, " }}")?;
        Ok(())
    }
}

impl Status {
    pub fn new(state: u8) -> Self {
        Self {
            value: state,
            error: state.get_bit(0),
            index: state.get_bit(1),
            corrected: state.get_bit(2),
            drive_request: state.get_bit(3),
            service_request: state.get_bit(4),
            drive_fault: state.get_bit(5),
            ready: state.get_bit(6),
            busy: state.get_bit(7),
        }
    }
}

impl Disk {
    /// Returns A Disk From The Given Index. Returns None If Index Is Greater Than 3;
    pub fn new(drive: usize) -> Option<Disk> {
        if drive > 3 {
            return None;
        };
        let drive: u8 = drive as u8;
        let bus = drive / 2;
        let io_base: u16 = if bus == 0 { 0x1F0 } else { 0x170 };
        let ctl_base: u16 = if bus == 0 { 0x3F6 } else { 0x376 };
        let irq = if bus == 0 { 14 } else { 15 };
        Some(Self {
            bus: drive / 2,
            disk: drive % 2,
            sectors: 0,
            ata_bus: Bus::new(io_base, ctl_base, irq),
        })
    }
}

impl IoRegisters {
    pub fn new(base: u16) -> Self {
        Self {
            data: Port::new(base + 0),
            error: PortReadOnly::new(base + 1),
            features: PortReadOnly::new(base + 1),
            sector_count: Port::new(base + 2),
            lba_lo: Port::new(base + 3),
            lba_mid: Port::new(base + 4),
            lba_hi: Port::new(base + 5),
            drive: Port::new(base + 6),
            status: PortReadOnly::new(base + 7),
            command: PortWriteOnly::new(base + 7),
        }
    }

    pub fn is_floating(&mut self) -> bool {
        unsafe { self.status.read() == 0xFF }
    }

    pub fn status(&mut self) -> Status {
        Status::new(unsafe { self.status.read() })
    }
}

impl ControlRegisters {
    pub fn new(base: u16) -> Self {
        Self {
            alt_status: PortReadOnly::new(base + 0),
            dev_control: PortWriteOnly::new(base + 0),
            drv_address: PortWriteOnly::new(base + 1),
        }
    }
}

/// Returns True If The Bus Is Present.
/// Returns False If Bus Is Not Present OR if the bus number is neither 0 or 1.
pub fn is_present(bus: u8) -> bool {
    match bus {
        0 => !IoRegisters::new(0x1F0).is_floating(),
        1 => !IoRegisters::new(0x170).is_floating(),
        _ => false,
    }
}

/// Returns The Sector Count Of The Given Disk.
pub fn sector_count() -> usize {
    unsafe { IoRegisters::new(0x1F0).sector_count.read() as usize }
}

impl Disk {
    /// Read The Given Sector Data Off Of Disk. Reads Until Buffer Is Full OR if 512 Bytes
    /// Have Been Read.
    pub fn read(&mut self, addr: SectorIndex, buf: &mut [u8]) -> KResult<()> {
        self.ata_bus.read_lba_24(self.disk, addr, buf);
        Ok(())
    }
    /// Write To The Given Sector Data Onto The Disk. Writes Until Buffer Is 'Empty' OR if 512 Bytes
    /// Have Been Written.
    pub fn write(&mut self, addr: SectorIndex, buf: &[u8]) -> KResult<()> {
        self.ata_bus.write_lba_24(self.disk, addr, buf)?;
        Ok(())
    }

    /// Times The Read & Write Speed Of The Disk.
    pub fn bandwidth_test(&mut self, index: SectorIndex) -> (f64, f64) {
        let mut buffer: [u8; 512] = [0; 512];
        let mut start = uptime();
        self.read(index, &mut buffer).expect("");
        let mut end = uptime();
        let read_bandwidth = 1.0 / ((end - start) / 512.0);

        start = uptime();
        self.write(index, &buffer).expect("");
        end = uptime();
        let write_bandwidth = 1.0 / ((end - start) / 512.0);

        (read_bandwidth, write_bandwidth)
    }

    /// Identify This Drive, Returns A Tuple Containing (Model, Serial, sector count), OR NONE.
    pub fn identify(&mut self) -> Option<(String, String, u32)> {
        if let Some(buf) = self.ata_bus.identify_drive(self.disk) {
            let mut serial = String::new();
            for i in 10..20 {
                for &b in &buf[i].to_be_bytes() {
                    serial.push(b as char);
                }
            }
            serial = serial.trim().into();
            let mut model = String::new();
            for i in 27..47 {
                for &b in &buf[i].to_be_bytes() {
                    model.push(b as char);
                }
            }
            model = model.trim().into();
            // Total number of 28-bit LBA addressable blocks
            let blocks = (buf[61] as u32) << 16 | (buf[60] as u32);
            self.set_sector_count(blocks);
            Some((model, serial, blocks))
        } else {
            None
        }
    }

    fn set_sector_count(&mut self, count: SectorIndex) {
        self.sectors = count;
    }
}

impl Bus {
    pub fn new(io_base: u16, ctrl_base: u16, _: u8) -> Bus {
        Bus {
            io_reg: IoRegisters::new(io_base),
            ctrl_reg: ControlRegisters::new(ctrl_base),
        }
    }

    pub fn read_lba_24(&mut self, drive: u8, index: SectorIndex, buf: &mut [u8]) {
        self.setup(drive, index);
        self.write_command(Command::ReadSectors);
        self.buzy_loop();
        //print!("Transfering Data");
        for i in (0..512).step_by(2) {
            let data = self.read_data();
            buf[i + 0] = data.get_bits(0..8) as u8;
            buf[i + 1] = data.get_bits(8..16) as u8;
        }
    }

    pub fn write_lba_24(&mut self, drive: u8, index: SectorIndex, buf: &[u8]) -> KResult<()> {
        self.setup(drive, index);
        self.write_command(Command::WriteSectors);
        self.buzy_loop();
        for i in (0..512).step_by(2) {
            let mut data = 0 as u16;
            data.set_bits(0..8, buf[i + 0] as u16);
            data.set_bits(8..16, buf[i + 1] as u16);
            self.write_data(data);
        }
        Ok(())
    }

    fn is_busy(&mut self) -> bool {
        let state = self.io_reg.status();
        return state.busy;
    }

    fn is_err(&mut self) -> bool {
        let state = self.io_reg.status();
        return state.error;
    }

    fn is_ready(&mut self) -> bool {
        self.io_reg.status().ready
    }

    fn wait(&self) {
        timer::sleep_ticks(1);
    }

    fn buzy_loop(&mut self) {
        self.wait();
        let start = uptime();
        while self.is_busy() {
            if (uptime() - start) > 1.0 {
                self.reset()
            };
            crate::spin();
        }
    }

    fn reset(&mut self) {
        unsafe {
            self.ctrl_reg.dev_control.write(0b0000_0100);
            sleep_ticks(2);
            self.ctrl_reg.dev_control.write(0b0000_0000);
            sleep_ticks(2);
        }
    }

    fn select_drive(&mut self, drive: u8) {
        // Drive #0 (primary) = 0xA0
        // Drive #1 (secondary) = 0xB0
        let drive_id = 0xA0 | (drive << 4);
        unsafe {
            self.io_reg.drive.write(drive_id);
        }
    }

    fn setup(&mut self, drive: u8, block: u32) {
        let drive_id = 0xE0 | (drive << 4);
        unsafe {
            self.io_reg
                .drive
                .write(drive_id | ((block.get_bits(24..28) as u8) & 0x0F));
            self.io_reg.sector_count.write(1);
            self.io_reg.lba_lo.write(block.get_bits(0..8) as u8);
            self.io_reg.lba_mid.write(block.get_bits(8..16) as u8);
            self.io_reg.lba_hi.write(block.get_bits(16..24) as u8);
        }
    }

    fn write_command(&mut self, cmd: Command) {
        unsafe {
            self.io_reg.command.write(cmd as u8);
        }
    }

    fn read_data(&mut self) -> u16 {
        unsafe { self.io_reg.data.read() }
    }

    fn write_data(&mut self, data: u16) {
        unsafe { self.io_reg.data.write(data) }
    }

    fn identify_drive(&mut self, drive: u8) -> Option<[u16; 256]> {
        self.reset();
        self.wait();
        self.select_drive(drive);
        self.wait();

        unsafe {
            self.io_reg.sector_count.write(0);
            self.io_reg.lba_lo.write(0);
            self.io_reg.lba_mid.write(0);
            self.io_reg.lba_hi.write(0);
        }

        self.write_command(Command::Indentify);

        if self.io_reg.status().value == 0 {
            return None;
        };

        self.buzy_loop();

        if self.lba_mid() != 0 || self.lba_high() != 0 {
            return None;
        }

        for i in 0.. {
            if i == 25 {
                // Waited 10ms (400ns * 25)
                self.reset();
                return None;
            }
            if self.is_err() {
                return None;
            }
            if self.is_ready() {
                break;
            }
            self.wait();
        }

        let mut res = [0; 256];
        for i in 0..256 {
            res[i] = self.read_data();
        }
        Some(res)
    }

    fn lba_mid(&mut self) -> u8 {
        unsafe { self.io_reg.lba_mid.read() }
    }

    fn lba_high(&mut self) -> u8 {
        unsafe { self.io_reg.lba_hi.read() }
    }
}

impl Into<u8> for Command {
    fn into(self) -> u8 {
        self as u8
    }
}

#[doc(hidden)]
pub fn bus_0_irq(_: u8) {}

#[doc(hidden)]
pub fn bus_1_irq(_: u8) {}

/// Read From The Given Drive.
pub fn read(drive: usize, addr: SectorIndex, buf: &mut [u8]) -> KResult<()> {
    if let Some(mut disk) = Disk::new(drive) {
        disk.read(addr, buf)
    } else {
        return Err("Failed To Get Drive.");
    }
}

/// Write To The Given Drive.
pub fn write(drive: usize, addr: SectorIndex, buf: &[u8]) -> KResult<()> {
    if let Some(mut disk) = Disk::new(drive) {
        disk.write(addr, buf)
    } else {
        return Err("Failed To Get Drive.");
    }
}

/// Copies Sectors Between Drives
pub unsafe fn copy(dest_drive: usize, src_drive: usize, start: u32, amount: usize) -> KResult<()> {
    let mut data = [0; 512];
    let mut prog = 0;
    let mut spinner = Spinner::new();
    log!(
        "Copying Sectors...{} - ({:05}/{:05})\r",
        spinner.glyph(),
        prog,
        amount
    );
    for index in start..start + amount as u32 {
        read(src_drive, index, &mut data)?;
        write(dest_drive, index, &data)?;
        if prog > 0 && prog % 128 == 0 {
            spinner.update();
            log!(
                "Copying Sectors...{} - ({:05}/{:05})\r",
                spinner.glyph(),
                prog,
                amount
            );
        }

        prog += 1;
    }
    log!("Complete...\n");
    Ok(())
}
