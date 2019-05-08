use std::io::{self, Read, Seek, SeekFrom};

pub mod guid;
pub mod ntfs;
pub mod security;
pub mod timestamp;
pub mod err;

pub(crate) mod utils;

pub trait ReadSeek: Read + Seek {
    fn tell(&mut self) -> io::Result<u64> {
        self.seek(SeekFrom::Current(0))
    }
}

impl<T: Read + Seek> ReadSeek for T {}
