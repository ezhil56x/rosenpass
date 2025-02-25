use anyhow::ensure;
use std::fs::File;
use std::io::Read;
use std::result::Result;
use std::{fs::OpenOptions, path::Path};

/// Open a file writable
pub fn fopen_w<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
    Ok(OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?)
}
/// Open a file readable
pub fn fopen_r<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
    Ok(OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .truncate(false)
        .open(path)?)
}

pub trait ReadExactToEnd {
    type Error;

    fn read_exact_to_end(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
}

impl<R: Read> ReadExactToEnd for R {
    type Error = anyhow::Error;

    fn read_exact_to_end(&mut self, buf: &mut [u8]) -> anyhow::Result<()> {
        let mut dummy = [0u8; 8];
        self.read_exact(buf)?;
        ensure!(self.read(&mut dummy)? == 0, "File too long!");
        Ok(())
    }
}

pub trait LoadValue {
    type Error;

    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait LoadValueB64 {
    type Error;

    fn load_b64<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait StoreValue {
    type Error;

    fn store<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::Error>;
}
