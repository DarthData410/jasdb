use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::Result;

use crate::header::{Header, write_header, read_header};
use crate::section::Section;
use crate::footer::Footer;

/// Manages all file operations for the database
pub struct FileSystem {
    /// The underlying file handle
    file: File,
    /// The current header state
    header: Header,
    /// Current file size
    size: u64,
}

impl FileSystem {
    /// Creates a new database file with initial header
    pub fn create(path: &Path) -> Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
            
        // Initialize with empty TOC right after header
        let toc_start = crate::header::HEADER_SIZE as u64;
        let toc_end = toc_start; // Empty TOC initially
        
        // Write initial header
        write_header(&mut file, toc_start, toc_end)?;
        
        // Read it back to initialize our state
        let header = read_header(&mut file)?;
        let size = file.metadata()?.len();
        
        Ok(Self {
            file,
            header,
            size,
        })
    }

    /// Opens an existing database file
    pub fn open(path: &Path) -> Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?;
            
        let header = read_header(&mut file)?;
        let size = file.metadata()?.len();
        
        Ok(Self {
            file,
            header,
            size,
        })
    }

    /// Returns a reference to the underlying file
    pub fn get_file(&mut self) -> &mut File {
        &mut self.file
    }

    /// Returns the current file size
    pub fn get_size(&self) -> u64 {
        self.size
    }

    /// Updates TOC region boundaries in header
    pub fn update_toc_bounds(&mut self, start: u64, end: u64) -> Result<()> {
        write_header(&mut self.file, start, end)?;
        self.header = read_header(&mut self.file)?;
        Ok(())
    }
}