use alloc::vec::Vec;
use alloc::vec;

pub const BLOCK_SIZE: usize = 4096;

#[derive(Debug)]
pub enum BlockError {
    ReadError,
    WriteError,
    OutOfBounds,
}

pub trait BlockIO {
    fn read_block(&self, block_id: u32, buf: &mut [u8]) -> Result<(), BlockError>;
    fn write_block(&mut self, block_id: u32, buf: &[u8]) -> Result<(), BlockError>;
    fn num_blocks(&self) -> u32;
}

pub struct RamDisk {
    data: Vec<u8>,
    size: usize,
}

impl RamDisk {
    pub fn new(size: usize) -> Self {
        // Ensure size is multiple of BLOCK_SIZE
        let aligned_size = (size + BLOCK_SIZE - 1) / BLOCK_SIZE * BLOCK_SIZE;
        RamDisk {
            data: vec![0; aligned_size],
            size: aligned_size,
        }
    }
}

impl BlockIO for RamDisk {
    fn read_block(&self, block_id: u32, buf: &mut [u8]) -> Result<(), BlockError> {
        let offset = block_id as usize * BLOCK_SIZE;
        if offset + BLOCK_SIZE > self.size {
            return Err(BlockError::OutOfBounds);
        }
        if buf.len() != BLOCK_SIZE {
            return Err(BlockError::ReadError);
        }

        buf.copy_from_slice(&self.data[offset..offset + BLOCK_SIZE]);
        Ok(())
    }

    fn write_block(&mut self, block_id: u32, buf: &[u8]) -> Result<(), BlockError> {
        let offset = block_id as usize * BLOCK_SIZE;
        if offset + BLOCK_SIZE > self.size {
            return Err(BlockError::OutOfBounds);
        }
        if buf.len() != BLOCK_SIZE {
            return Err(BlockError::WriteError);
        }

        self.data[offset..offset + BLOCK_SIZE].copy_from_slice(buf);
        Ok(())
    }

    fn num_blocks(&self) -> u32 {
        (self.size / BLOCK_SIZE) as u32
    }
}
