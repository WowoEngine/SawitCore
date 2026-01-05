use alloc::vec::Vec;
use alloc::vec;
use alloc::boxed::Box;
use crate::drivers::block::{BlockIO, BlockError, BLOCK_SIZE};
use core::convert::TryInto;

pub const MAGIC: &[u8; 4] = b"WOWO";

pub struct Pager {
    disk: Box<dyn BlockIO>,
}

impl Pager {
    pub fn new(disk: Box<dyn BlockIO>) -> Result<Self, BlockError> {
        let mut p = Pager { disk };
        
        // Check if file is initialized or empty
        // RamDisk is zeroed by default.
        // We check if block 0 is empty? Or we just init if magic missing.
        let mut buf = [0u8; BLOCK_SIZE];
        if p.disk.read_block(0, &mut buf).is_ok() {
            if &buf[0..4] != MAGIC {
                // Initialize new file
                p.init_new_file()?;
            }
        } else {
             // Maybe empty or error, try init
             p.init_new_file()?;
        }
        
        Ok(p)
    }

    fn init_new_file(&mut self) -> Result<(), BlockError> {
        let mut buf = [0u8; BLOCK_SIZE];
        buf[0..4].copy_from_slice(MAGIC);
        // Total pages = 1 (Page 0 itself)
        buf[4..8].copy_from_slice(&1u32.to_le_bytes()); 
        // Num Tables = 0
        buf[8..12].copy_from_slice(&0u32.to_le_bytes());

        self.disk.write_block(0, &buf)
    }

    pub fn read_page(&self, page_id: u32) -> Result<Vec<u8>, BlockError> {
        let mut buf = vec![0u8; BLOCK_SIZE];
        self.disk.read_block(page_id, &mut buf)?;
        Ok(buf)
    }

    pub fn write_page(&mut self, page_id: u32, buf: &[u8]) -> Result<(), BlockError> {
        if buf.len() != BLOCK_SIZE {
            return Err(BlockError::WriteError); // Or generic invalid arg
        }
        self.disk.write_block(page_id, buf)
    }

    pub fn alloc_page(&mut self) -> Result<u32, BlockError> {
        // Read Page 0 to get total pages
        let mut page0 = self.read_page(0)?;
        
        let total_pages_bytes: [u8; 4] = page0[4..8].try_into().unwrap_or([0; 4]);
        let total_pages = u32::from_le_bytes(total_pages_bytes);

        let new_page_id = total_pages;
        let new_total = total_pages + 1;

        // Update total pages
        page0[4..8].copy_from_slice(&new_total.to_le_bytes());
        self.write_page(0, &page0)?;

        // Initialize new page
        let mut new_page = vec![0u8; BLOCK_SIZE];
        // Go impl: 
        // 0-4: Next Page (0)
        // 4-6: Count (0)
        // 6-8: Free Offset (8) - Though this looks like specific logic for Table/Leaf pages, generic pager just gives zeroed page
        // But the Go code did this in AllocPage explicitly, so I will too.
        new_page[0..4].copy_from_slice(&0u32.to_le_bytes());
        new_page[4..6].copy_from_slice(&0u16.to_le_bytes()); 
        new_page[6..8].copy_from_slice(&8u16.to_le_bytes());

        self.write_page(new_page_id, &new_page)?;

        Ok(new_page_id)
    }
}
