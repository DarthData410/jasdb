// lock.rs
use std::fs::{File, OpenOptions};
use std::io;
use fs2::FileExt;
use anyhow::{Result, Context};

/// Acquires a shared (read) lock on the file.
/// Use when performing read-only operations.
pub fn acquire_shared_lock(file: &File) -> io::Result<()> {
    file.lock_shared()
}

/// Acquires an exclusive (write) lock on the file.
/// Use when performing write or mutating operations.
pub fn acquire_exclusive_lock(file: &File) -> io::Result<()> {
    file.lock_exclusive()
}

/// Releases any lock held on the file.
pub fn release_lock(file: &File) -> io::Result<()> {
    file.unlock()
}

/// Wrapper for safe shared access with read-locking.
pub fn with_shared_access<T>(
    path: &str,
    f: impl FnOnce(&mut File) -> io::Result<T>,
) -> Result<T> {
    let mut file = File::open(path).with_context(|| format!("Failed to open file '{}'", path))?;
    acquire_shared_lock(&file).with_context(|| "Failed to acquire shared lock")?;

    let result = f(&mut file).with_context(|| "Error during shared file operation")?;

    release_lock(&file).with_context(|| "Failed to release shared lock")?;
    Ok(result)
}

/// Wrapper for safe exclusive access with write-locking.
pub fn with_exclusive_access<T>(
    path: &str,
    f: impl FnOnce(&mut File) -> io::Result<T>,
) -> Result<T> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .with_context(|| format!("Failed to open file '{}'", path))?;

    acquire_exclusive_lock(&file).with_context(|| "Failed to acquire exclusive lock")?;

    let result = f(&mut file).with_context(|| "Error during exclusive file operation")?;

    release_lock(&file).with_context(|| "Failed to release exclusive lock")?;
    Ok(result)
}
