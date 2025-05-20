use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::Result;
use crate::lock::{with_exclusive_access, with_shared_access};
use crate::io::{read_at, write_at, get_eof};

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

    /// In-memory TOC cache
    toc: TableOfContents,
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
            toc: TableOfContents::new(), // Initialize empty TOC
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
            toc: TableOfContents::new(), // Initialize empty TOC
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

    /// Allocates a new collection section
    pub fn create_collection(&mut self, name: &str, schema: Option<Vec<u8>>) -> Result<()> {
        let section_start = get_eof(self.get_file())?;
        
        // Create TOC entry for the new collection
        let entry = TocEntry {
            section_type: SectionType::Collection,
            offset: section_start,
            length: 0,
            schema,
            metadata: HashMap::new(),
        };
        
        self.toc.add_entry(name.to_string(), entry);
        self.write_toc()?;
        Ok(())
    }

    /// Writes the current TOC to disk
    fn write_toc(&mut self) -> Result<()> {
        let toc_start = get_eof(self.get_file())?;
        let toc_bytes = bincode::serialize(&self.toc)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        write_at(self.get_file(), toc_start, &toc_bytes)?;
        let toc_end = get_eof(self.get_file())?;
        
        // Update header with new TOC bounds
        self.update_toc_bounds(toc_start, toc_end)?;
        Ok(())
    }
}