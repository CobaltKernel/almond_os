//! Virtual FileSystem
use alloc::{vec, boxed::Box};
use vec::Vec;
use crate::{sys::{storage::ata}, KResult, no_interrupt, slog};
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

pub const BLOCK_SIZE: usize = 512;
pub const CLUSTER_SIZE: usize = 4;

lazy_static! {
    static ref DEVICE: Mutex<Option<Device>> = Mutex::new(None);  
}

pub fn mount(dev: Device) {
    no_interrupt!({
        slog!("Mounting Device: {:?}\n", dev);
        *DEVICE.lock() = Some(dev);
        slog!("Mounted Device\n");
    })
}

pub fn is_mounted() -> bool {
    DEVICE.lock().is_some()
}


pub fn device<'dev>() -> MutexGuard<'dev, Option<Device>> {
        DEVICE.lock()
}

pub trait FileSystem {
    fn block_count(&self) -> usize;

    fn create_file(&mut self, filename: &str) -> Option<Box<dyn FileIO>>;
    fn open_file(&mut self, filename: &str) -> Option<Box<dyn FileIO>>;
    fn create_dir(&mut self, dirname: &str) -> Option<Box<dyn Dir>>;
    fn current_dir(&self) -> Option<Box<dyn Dir>>;
    fn change_dir(&self, dirname: &str) -> Option<Box<dyn Dir>>;

    fn file_exists(&self, filename: &str) -> bool;
    fn dir_exists(&self, dirname: &str) -> bool;
}

pub trait FileIO {
    fn size(&self) -> usize;
    fn pos(&self) -> usize;
    fn seek(&mut self, pos: isize) -> usize;
    fn read(&mut self, buffer: &mut [u8]) -> usize;
    fn write(&mut self, buffer: &[u8]) -> usize;
    fn append(&mut self, buffer: &[u8]) -> usize;
    fn close(&mut self);
}

pub trait Dir {
    fn files(&self) -> Vec<Box<dyn FileIO>>;
}

pub trait BlockDeviceIO {
    fn read(&self, index: usize) -> KResult<[u8; BLOCK_SIZE]>;
    fn write(&mut self, index: usize, data: &[u8]) -> KResult<()>;

    fn block_size(&self) -> usize;
    fn block_count(&self) -> usize;
}

#[derive(Debug, Clone)]
pub enum Device {
    Ata(AtaDevice),
    Mem(MemDevice),
}

impl Device {
    pub fn ata(drive: usize) -> Device {
        Device::Ata(AtaDevice::new(drive))
    }

    pub fn mem(size: usize) -> Device {
        Device::Mem(MemDevice::new(size))
    }
}

impl BlockDeviceIO for Device {
    fn read(&self, index: usize) -> KResult<[u8; BLOCK_SIZE]> {
        match self {
            Self::Ata(dev) => {dev.read(index)}
            Self::Mem(dev) => {dev.read(index)}
        }
    }

    fn write(&mut self, index: usize, data: &[u8]) -> KResult<()> {
        match self {
            Self::Ata(dev) => {dev.write(index, data)}
            Self::Mem(dev) => {dev.write(index, data)}
        }
    }

    fn block_count(&self) -> usize {
        match self {
            Self::Ata(dev) => {dev.block_count()}
            Self::Mem(dev) => {dev.block_count()}
        }
    }

    fn block_size(&self) -> usize {
        match self {
            Self::Ata(dev) => {dev.block_size()}
            Self::Mem(dev) => {dev.block_size()}
        }
    }
}
#[derive(Debug, Clone)]
pub struct AtaDevice(usize);
#[derive(Debug, Clone)]
pub struct MemDevice(Vec<[u8; BLOCK_SIZE]>);

impl AtaDevice {
    pub fn new(drive: usize) -> Self {
        Self(drive)
    }
}

impl MemDevice {
    pub fn new(size: usize) -> Self {
        Self(vec![[0; BLOCK_SIZE]; size])
    }
}

impl BlockDeviceIO for AtaDevice {
    fn read(&self, index: usize) -> KResult<[u8; BLOCK_SIZE]> {
        ata::read_block(self.0, index as u32)
    }

    fn write(&mut self, index: usize, data: &[u8]) -> KResult<()> {
        ata::write(self.0, index as u32, data)
    }

    fn block_count(&self) -> usize {
        262_144
    }

    fn block_size(&self) -> usize {
        512
    }
}

impl BlockDeviceIO for MemDevice {
    fn read(&self, index: usize) -> KResult<[u8; BLOCK_SIZE]> {
        if index < self.0.len() {
            Ok(self.0[index])
        } else {
            Err("Index Out Of Bounds")
        }
    }

    fn write(&mut self, index: usize, data: &[u8]) -> KResult<()> {
        if index < self.0.len() {
            let mut buf = [0; BLOCK_SIZE];
            for (idx, byte) in data.iter().enumerate() {
                buf[idx] = *byte;
            }
            self.0[index] = buf;
            Ok(())
        } else {
            Err("Index Out Of Bounds")
        }
    }

    fn block_count(&self) -> usize {
        self.0.len()
    }

    fn block_size(&self) -> usize {
        512
    }
}