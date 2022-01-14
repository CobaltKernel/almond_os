use super::block::Block;

const DATA_OFFSET: usize = 4;

#[derive(Debug)]
pub struct LinkedBlock {
    block: Block
}

impl LinkedBlock {
    pub fn new(addr: u32) -> Self {
        Self { block: Block::empty(addr) }
    }

    pub fn alloc() -> Option<Self> {
        Block::allocate_mfs().map(|block| Self { block })
    }

    pub fn read(addr: u32) -> Self {
        Self { block: Block::read(addr).expect("BRF") }
    }

    pub fn write(&self) {
        self.block.write().expect("LBWF");
    }

    pub fn addr(&self) -> u32 {
        self.block.addr()
    }

    pub fn data(&self) -> &[u8] {
        &self.block.data()[DATA_OFFSET..super::BLOCK_SIZE]
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.block.data_mut()[DATA_OFFSET..super::BLOCK_SIZE]
    }

    pub fn len(&self) -> usize {
        super::BLOCK_SIZE - DATA_OFFSET
    }

    pub fn next(&self) -> Option<Self> {
        let addr = u32::from_be_bytes(self.block.data()[0..4].try_into().unwrap());
        if addr == 0 {
            None
        } else {
            Some(Self::read(addr))
        }
    }

    pub fn alloc_next(&mut self) -> Option<Self> {
        let new_block = LinkedBlock::alloc()?;
        self.set_next_addr(new_block.addr());
        self.write();
        Some(new_block)
    }

    pub fn set_next_addr(&mut self, addr: u32) {
        self.block.data_mut()[0..4].clone_from_slice(&addr.to_be_bytes());
    }
}