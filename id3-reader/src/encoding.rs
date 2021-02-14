#[derive(Debug)]
pub enum EncodingTypes {
    Latin1,
    Utf16Le,
    Utf16Be,
    Utf8,
}

use encoding_rs::{Encoding, UTF_16BE, UTF_16LE, UTF_8, WINDOWS_1252};

#[allow(dead_code)]
impl EncodingTypes {
    pub fn from_byte(byte: u8) -> Option<Self> {
        Some(match byte {
            0x00 => EncodingTypes::Latin1,
            0x01 => EncodingTypes::Utf16Le,
            0x02 => EncodingTypes::Utf16Be,
            0x03 => EncodingTypes::Utf8,
            _ => None?,
        })
    }
    pub fn get_encoding(&self) -> &'static Encoding {
        match self {
            EncodingTypes::Latin1 => WINDOWS_1252,
            EncodingTypes::Utf16Le => UTF_16LE,
            EncodingTypes::Utf16Be => UTF_16BE,
            EncodingTypes::Utf8 => UTF_8,
        }
    }
    pub fn decode(&self, string: &[u8]) -> String {
        match self {
            EncodingTypes::Latin1 => WINDOWS_1252.decode(string).0,
            EncodingTypes::Utf16Le => UTF_16BE.decode(string).0,
            EncodingTypes::Utf16Be => UTF_16BE.decode_without_bom_handling(string).0,
            EncodingTypes::Utf8 => UTF_8.decode(string).0,
        }
        .into()
    }
}
