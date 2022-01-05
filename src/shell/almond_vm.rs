use core::fmt::Alignment;

use super::*;

use alloc::vec;
use vec::Vec;

const MEM_SIZE: usize = u16::MAX as usize;

pub struct AlmondVM {
    ip: usize,
    acc: usize,
    x: usize,
    y: usize,
    sp: usize,

    mem: [u8; MEM_SIZE],
}


impl AlmondVM {
}



