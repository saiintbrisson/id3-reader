use std::io::{BufReader, BufRead, Read, Error};
use std::string::{FromUtf8Error, FromUtf16Error};

pub struct Bytes {

    buf: Vec<u8>,
    index: usize

}

impl Bytes {

    pub fn from_slice(buf: &[u8]) -> Bytes {
        Bytes {
            buf: Vec::from(buf),
            index: 0
        }
    }

    pub fn from_vec(buf:  Vec<u8>) -> Bytes {
        Bytes {
            buf,
            index: 0
        }
    }
    
    pub fn from_reader<T: Read>(reader: &mut BufReader<T>) -> Result<Bytes, Error> {
        let mut buffer = reader.buffer();
        if buffer.is_empty() {
            buffer = reader.fill_buf()?;
        }

        Ok(Bytes {
            buf: Vec::from(buffer),
            index: 0
        })
    }

    pub fn override_buf<T: Read>(&mut self, reader: &mut BufReader<T>) -> Result<(), Error> {
        let mut buffer = reader.buffer();
        if buffer.is_empty() {
            buffer = reader.fill_buf()?;
		}
		
		self.buf = Vec::from(buffer);
		Ok(())
    }

    pub fn get_buf(&self) -> &Vec<u8> {
        &self.buf
    }
    
    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn set_cap(&mut self, cap: usize) {
        if self.index > cap {
            self.index = cap;
        }

        self.buf = self.buf.as_slice()[..cap].into();
    }

    pub fn set_offset(&mut self, offset: usize) {
        if self.index < offset {
            self.index = 0;
        } else {
            self.index -= offset;
        }

        self.buf = self.buf.as_slice()[offset..].into();
    }

    pub fn get_readable_bytes(&self) -> usize {
        self.buf.len() - self.index
    }

    pub fn has_readable_bytes(&self) -> bool {
        self.index < self.buf.len()
    }

}

pub trait IndexedReader {

    fn read_bit(&self, index: usize, position: u8) -> bool;

    fn read_byte(&self, index: usize) -> u8;

    fn get_next_null(&self, from: usize) -> usize;

	fn read_slice(&self, from: usize, to: usize) -> &[u8];
	fn read_to_slice(&self, from: usize, slice: &mut [u8]);
    fn read_until_null(&self, from: usize, to: usize) -> &[u8];
    
    fn read_unsigned(&self, from: usize, to: usize) -> u128;
    fn read_signed(&self, from: usize, to: usize) -> u128;

    fn read_latin1(&self, from: usize, to: usize) -> String;
    fn read_utf8(&self, from: usize, to: usize) -> Result<String, FromUtf8Error>;
    fn read_utf16(&self, from: usize, to: usize) -> Result<String, FromUtf16Error>;

    fn read_text(&self, from: usize, to: usize) -> Option<String>;

}

pub trait ContinuousReader {

    fn read_bit(&mut self, position: u8) -> bool;

    fn read_byte(&mut self) -> u8;

    fn get_next_null(&self) -> usize;

	fn read_slice(&mut self, length: usize) -> &[u8];
	fn read_to_slice(&mut self, slice: &mut [u8]);
    fn read_until_null(&mut self, length: usize) -> &[u8];

    fn read_unsigned(&mut self, length: usize) -> u128;
    fn read_signed(&mut self, length: usize) -> u128;

    fn read_latin1(&mut self, length: usize) -> String;
    fn read_utf8(&mut self, length: usize) -> Result<String, FromUtf8Error>;
    fn read_utf16(&mut self, length: usize) -> Result<String, FromUtf16Error>;
    
    fn read_text(&mut self, length: usize) -> Option<String>;

}

impl IndexedReader for Bytes {

    fn read_bit(&self, index: usize, position: u8) -> bool {
        bit_at(self.read_byte(index), position)
    }

    fn read_byte(&self, index: usize) -> u8 {
        self.buf.as_slice()[index]
    }

    fn get_next_null(&self, from: usize) -> usize {
        next_null(&self.buf.as_slice()[from..])
    }

    fn read_slice(&self, from: usize, to: usize) -> &[u8] {
        &self.buf.as_slice()[from..to]
    }

    fn read_to_slice(&self, from: usize, slice: &mut [u8]) {
		for i in 0..slice.len() {
			if from + i > self.buf.len() {
				break
			}

			slice[i] = self.buf.as_slice()[from];
		}
    }

    fn read_until_null(&self, from: usize, to: usize) -> &[u8] {
        until_null(IndexedReader::read_slice(self, from, from + to))
    }

    fn read_unsigned(&self, from: usize, to: usize) -> u128 {
        convert_unsigned(self.read_slice(from, to))
    }

    fn read_signed(&self, from: usize, to: usize) -> u128 {
        convert_signed(self.read_slice(from, to))
    }

    fn read_latin1(&self, from: usize, to: usize) -> String {
        convert_latin1(IndexedReader::read_until_null(self, from, to))
    }

    fn read_utf8(&self, from: usize, to: usize) -> Result<String, FromUtf8Error> {
        convert_utf8(IndexedReader::read_until_null(self, from, to))
    }

    fn read_utf16(&self, from: usize, to: usize) -> Result<String, FromUtf16Error> {
        convert_utf16(IndexedReader::read_until_null(self, from, to))
    }
    
    fn read_text(&self, from: usize, to: usize) -> Option<String> {
        decode_text(IndexedReader::read_until_null(self, from + 1, to), self.read_byte(from))
    } 

}


impl ContinuousReader for Bytes {

    fn read_bit(&mut self, position: u8) -> bool {
        bit_at(self.read_byte(), position)
    }

    fn read_byte(&mut self) -> u8 {
        self.index += 1;
        self.buf.as_slice()[self.index - 1]
    }

    fn get_next_null(&self) -> usize {
        next_null(&self.buf.as_slice()[self.index..])
    }

    fn read_slice(&mut self, length: usize) -> &[u8] {
        self.index += length;
        &self.buf.as_slice()[self.index - length..self.index]
    }

    fn read_to_slice(&mut self, slice: &mut [u8]) {
		for i in 0..slice.len() {
			if self.index + i > self.buf.len() {
				break
			}

			slice[i] = self.buf.as_slice()[self.index];
			self.index += 1;
		}
    }

    fn read_until_null(&mut self, length: usize) -> &[u8] {
        until_null(&self.read_slice(length))
    }
    
    fn read_unsigned(&mut self, length: usize) -> u128 {
        convert_unsigned(self.read_slice(length))
    }
    
    fn read_signed(&mut self, length: usize) -> u128 {
        convert_signed(self.read_slice(length))
    }

    fn read_latin1(&mut self, length: usize) -> String {
        convert_latin1(ContinuousReader::read_slice(self, length))
    }

    fn read_utf8(&mut self, length: usize) -> Result<String, FromUtf8Error> {
        convert_utf8(ContinuousReader::read_slice(self, length))
    }

    fn read_utf16(&mut self, length: usize) -> Result<String, FromUtf16Error> {
        convert_utf16(ContinuousReader::read_slice(self, length))
    }

    fn read_text(&mut self, length: usize) -> Option<String> {
        let encoding = self.read_byte();
        decode_text(ContinuousReader::read_slice(self, length - 1), encoding)
    } 

}

pub fn bit_at(byte: u8, position: u8) -> bool {
    (byte & (1 << position)) != 0
}

pub fn next_null(slice: &[u8]) -> usize {
    match slice.iter().position(|byte| byte == &0) {
        None => slice.len() - 1,
        Some(position) => position
    }
}

pub fn until_null(slice: &[u8]) -> &[u8] {
    &slice[..next_null(slice)]
}

pub fn convert_unsigned(slice: &[u8]) -> u128 {
    let mut result = 0;

    for i in 0..slice.len() {
        result |= (slice[i] as u128) << 8 * (slice.len()  - (i + 1)) as u128;
	}

    result
}

pub fn convert_signed(slice: &[u8]) -> u128 {
    let mut result = 0;

    for i in 0..slice.len() {
        result |= ((slice[i] & 0x7F) as u128) << 7 * (slice.len()  - (i + 1)) as u128;
    }

    result
}

pub fn convert_latin1(slice: &[u8]) -> String {
    slice.iter().map(|&c| c as char).collect()
}

pub fn convert_utf8(slice: &[u8]) -> Result<String, FromUtf8Error> {
   String::from_utf8(slice.into())
}

pub fn convert_utf16(slice: &[u8]) -> Result<String, FromUtf16Error> {
    let vec = slice.chunks_exact(2)
        .into_iter()
        .map(|byte| u16::from_ne_bytes([byte[0], byte[1]]))
		.filter(|byte| {
			byte >= &(0x20 as u16) 
				&& byte != &(0xFEFF as u16) 
				&& byte != &(0xFFFE as u16)
		})
        .collect::<Vec<u16>>();

    String::from_utf16(vec.as_slice())
}

pub fn decode_text(slice: &[u8], encoding: u8) -> Option<String> {
    match encoding {
        0x00 => Some(convert_latin1(slice)),
        0x01 => convert_utf16(slice).ok(),
        0x03 => convert_utf8(slice).ok(),
        _ => None
    }
}