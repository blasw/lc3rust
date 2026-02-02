use std::fs::File;
use std::io::{BufReader, Read};

pub struct U16FileReader {
    reader: BufReader<File>,
}

impl U16FileReader {
    pub fn new(reader: BufReader<File>) -> U16FileReader {
        U16FileReader {
            reader
        }
    }

    pub fn read_u16(&mut self) -> Result<u16, anyhow::Error> {
        let mut u16_buf = [0; 2];
        self.reader.read_exact(&mut u16_buf)?;
        Ok(u16::from_be_bytes(u16_buf))
    }
}