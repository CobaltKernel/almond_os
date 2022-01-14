

#[allow(dead_code)]
const MEM_SIZE: usize = u16::MAX as usize;

#[allow(dead_code)]
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



