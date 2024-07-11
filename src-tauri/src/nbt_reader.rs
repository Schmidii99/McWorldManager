use std::fs::File;
use std::io::{self, Read, Seek};

pub fn detect_and_read_nbt(input: &mut File) -> io::Result<nbt::Blob> {
    input.seek(std::io::SeekFrom::Start(0))?;
    let mut header = [0; 2];
    input.read_exact(&mut header)?;
    input.seek(std::io::SeekFrom::Start(0))?;
    match header {
        // Common ZLIB headers: https://stackoverflow.com/a/17176881
        [0x78, 0x01] | [0x78, 0x9C] | [0x78, 0xDA] => Ok(nbt::from_zlib_reader(input)?),
        // GZIP header: https://en.wikipedia.org/wiki/Gzip#File_format
        [0x1f, 0x8b] => Ok(nbt::from_gzip_reader(input)?),
        // Assume we have raw (uncompressed) NBT
        _ => Ok(nbt::from_reader(input)?),
    }
}