use bytes::Buf;

use crate::encoding::EncodingTypes;

pub trait BufExt {
    fn get_string(&mut self) -> Option<String>;
    fn get_encoded_string(&mut self, encoding: &EncodingTypes) -> String;
    fn get_sized_string(&mut self, encoding: &EncodingTypes, size: usize) -> String;

    fn get_synchsafe_int(&mut self) -> i32;
    fn get_synchsafe_uint(&mut self) -> u32;

    fn find_null_u8_until(&mut self, limit: usize) -> usize;
    fn find_null_u16_until(&mut self, limit: usize) -> usize;

    fn find_null_u8(&mut self) -> usize;
    fn find_null_u16(&mut self) -> usize;
}

impl<B> BufExt for B
where
    B: AsRef<[u8]> + Buf,
{
    fn get_string(&mut self) -> Option<String> {
        let encoding = self.get_u8();
        Some(self.get_encoded_string(&EncodingTypes::from_byte(encoding)?))
    }

    fn get_encoded_string(&mut self, encoding: &EncodingTypes) -> String {
        let pos = match encoding {
            EncodingTypes::Utf16Le | EncodingTypes::Utf16Be => self.find_null_u16(),
            _ => self.find_null_u8(),
        };

        let result = encoding.decode(&self.as_ref()[..pos]);
        self.advance(pos);

        result
    }

    fn get_sized_string(&mut self, encoding: &EncodingTypes, size: usize) -> String {
        let pos = match encoding {
            EncodingTypes::Utf16Le | EncodingTypes::Utf16Be => self.find_null_u16_until(size),
            _ => self.find_null_u8_until(size),
        };
        let result = encoding.decode(&self.as_ref()[..pos]);
        self.advance(size);

        result
    }

    fn get_synchsafe_int(&mut self) -> i32 {
        let mut result = 0;

        for i in 0..4 {
            result |= (self.get_u8() as i32) << 7 * (4 - (i + 1));
        }

        result
    }
    fn get_synchsafe_uint(&mut self) -> u32 {
        let mut result = 0;

        for i in 0..4 {
            result |= (self.get_u8() as u32) << 8 * (4 - (i + 1));
        }

        result
    }

    fn find_null_u8_until(&mut self, limit: usize) -> usize {
        let limit = limit.min(self.as_ref().len());
        self.as_ref()[..limit]
            .iter()
            .position(|b| *b == 0)
            .unwrap_or(limit)
    }
    fn find_null_u16_until(&mut self, limit: usize) -> usize {
        let limit = limit.min(self.as_ref().len()) / 2;
        unsafe {
            // that's what we call "gambiarra" in Brazil :sunglasses:
            std::slice::from_raw_parts(self.as_ref().as_ptr() as *const u16, limit)
        }
        .iter()
        .position(|b| *b == 0)
        .map(|pos| pos * 2)
        .unwrap_or(limit)
    }

    fn find_null_u8(&mut self) -> usize {
        self.find_null_u8_until(self.as_ref().len())
    }
    fn find_null_u16(&mut self) -> usize {
        self.find_null_u16_until(self.as_ref().len())
    }
}
