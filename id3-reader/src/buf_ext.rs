use bytes::Buf;
use encoding_rs::{UTF_8, WINDOWS_1252};

use crate::v2::EncodingTypes;

pub trait BufExt {
    fn get_string(&mut self) -> Option<String>;

    fn get_latin1_string(&mut self, size: usize) -> String;
    fn get_utf8_string(&mut self, size: usize) -> String;
    fn get_utf16_string(&mut self, size: usize) -> Result<String, std::string::FromUtf16Error>;

    fn find_utf16_string(&mut self) -> Result<String, std::string::FromUtf16Error>;

    fn get_synchsafe_int(&mut self) -> i32;
    fn get_synchsafe_uint(&mut self) -> u32;

    fn find_null_u8(&mut self) -> usize;
    fn find_null_u16(&mut self) -> usize;
}

impl <B> BufExt for B 
where
    B: AsRef<[u8]> + Buf
{
    fn get_string(&mut self) -> Option<String> {
        let encoding = EncodingTypes::from_byte(self.get_u8())?;
        let pos = match encoding {
            EncodingTypes::Utf16Le | EncodingTypes::Utf16Be => self.find_null_u16(),
            _ => self.find_null_u8(),
        };
        
        let result = encoding.decode(&self.as_ref()[..pos]);
        self.advance(pos);

        Some(result)
    }

    fn get_latin1_string(&mut self, size: usize) -> String {
        let pos = self.find_null_u8();
        let result = WINDOWS_1252.decode(&self.as_ref()[..pos]).0.into();
        self.advance(size);

        result
    }
    fn get_utf8_string(&mut self, size: usize) -> String {
        let pos = self.find_null_u8();
        let result = UTF_8.decode(&self.as_ref()[..pos]).0.into();
        self.advance(size);

        result

    }
    fn get_utf16_string(&mut self, size: usize) -> Result<String, std::string::FromUtf16Error> {
        let mut vec = vec![0u8; size];
        self.copy_to_slice(&mut vec);

        get_string_utf16(vec)
    }

    fn find_utf16_string(&mut self) -> Result<String, std::string::FromUtf16Error> {
        let mut vec = Vec::new();

        while self.remaining() >= 2 {
            let byte = self.get_u8();
            let second_byte = self.get_u8();

            let composition = u16::from_ne_bytes([byte, second_byte]);
            if composition == 0x0000 || composition == 0xFEFF || composition == 0xFFFE { break; }

            vec.push(byte);
            vec.push(second_byte);
        }

        get_string_utf16(vec)
    }

    fn get_synchsafe_int(&mut self) -> i32 {
        let mut result = 0;

        for i in 0..4 {
            result |= (self.get_u8() as i32) << 7 * (4  - (i + 1));
        }

        result
    }
    fn get_synchsafe_uint(&mut self) -> u32 {
        let mut result = 0;

        for i in 0..4 {
            result |= (self.get_u8() as u32) << 8 * (4  - (i + 1));
        }

        result
    }
    
    fn find_null_u8(&mut self) -> usize {
        self.as_ref().iter().position(|b| *b == 0).unwrap_or(self.as_ref().len())
    }
    fn find_null_u16(&mut self) -> usize {
        unsafe { // that's what we call "gambiarra" in Brazil :sunglasses:
            std::slice::from_raw_parts(self.as_ref().as_ptr() as *const u16, self.as_ref().len() / 2)
        }.iter().position(|b| *b == 0).map(|pos| pos * 2).unwrap_or(self.as_ref().len())
    }
}

fn get_string_utf16(vec: Vec<u8>) -> Result<String, std::string::FromUtf16Error> {
    String::from_utf16(
        &vec.chunks_exact(2)
        .into_iter()
        .map(|byte| u16::from_ne_bytes([byte[0], byte[1]]))
        .take_while(|byte| {
            *byte >= 0x20 as u16
            && *byte != 0xFEFF as u16
            && *byte != 0xFFFE as u16
            && *byte != 0x0000 as u16
        })
        .collect::<Vec<u16>>()[..]
    )
}
