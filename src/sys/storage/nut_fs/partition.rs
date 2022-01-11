//! Partition Info

/// Disk Size In Blocks
pub const DISK_SIZE: u32 = (16 << 20) / 512;
/// How Many Files / Directories Can There Be?
pub const ENTRY_COUNT: u32 = 16;
/// The size Of The Data Sectors On The Disk.
pub const DATA_SIZE: usize = (1 << 20) / 512;

/// Kernel start Block
pub const KERNEL_START: u32 = 0;
/// Kernel Size (Blocks)
pub const KERNEL_SIZE: u32 = 1024;

/// [SuperBlock] Start
pub const SUPERBLOCK_START: u32 = KERNEL_START + KERNEL_SIZE;
/// [SuperBlock] Size
pub const SUPERBLOCK_SIZE: u32 = 1;

/// [MetaBitmap] Start
pub const META_BITMAP_START: u32 = SUPERBLOCK_START + SUPERBLOCK_SIZE;
/// [MetaBitmap] Size
pub const META_BITMAP_SIZE: u32 = ENTRY_COUNT / 4096;

/// [DataBitmap] Start
pub const DATA_BITMAP_START: u32 = META_BITMAP_START + META_BITMAP_SIZE;
/// [DataBitmap] Size
pub const DATA_BITMAP_SIZE: u32 = (DATA_SIZE / 512 / 4096) as u32;

/// [MetaData] Start
pub const META_START: u32 = DATA_BITMAP_START + DATA_BITMAP_SIZE;

/// [MetaData] Size
pub const META_SIZE: u32 = ENTRY_COUNT;

/// [DataBlock] Start
pub const DATA_START: u32 = META_START + META_SIZE;
