use std::io::Result;
use crate::io::{read_at, write_at};

pub struct Section {
    /// Starting offset of this section
    pub offset: u64,
    /// Current length of section
    pub length: u64,
}

impl Section {
    pub fn new(offset: u64) -> Self {
        Self {
            offset,
            length: 0,
        }
    }

    pub fn read(&self, file: &mut File) -> Result<Vec<u8>> {
        read_at(file, self.offset, self.length as usize)
    }

    pub fn write(&mut self, file: &mut File, data: &[u8]) -> Result<()> {
        write_at(file, self.offset + self.length, data)?;
        self.length += data.len() as u64;
        Ok(())
    }
}