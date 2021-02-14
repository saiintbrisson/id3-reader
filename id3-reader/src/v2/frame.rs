use bytes::{Buf, Bytes};

use crate::{buf_ext::BufExt, encoding::EncodingTypes};

#[derive(Debug)]
pub struct Frame {
    pub frame_type: FrameType,
    pub size: u32,
    pub flags: FrameFlags,
}

#[derive(Debug)]
pub enum FrameType {
    TIT2(String),
    TPE1(String),
    TALB(String),
    TYER(String),
    TCON(String),
    TXXX(String),
    COMM([u8; 3], String, String),
}

macro_rules! decode_text_frame {
    ($e:ident, $src:expr) => {
        FrameType::$e(crate::buf_ext::BufExt::get_string($src)?)
    };
}

#[allow(dead_code)]
impl FrameType {
    pub fn from_name(name: &[u8], src: &mut Bytes) -> Option<FrameType> {
        Some(match name {
            b"TT2" | b"TIT2" => decode_text_frame!(TIT2, src),
            b"TPE1" => decode_text_frame!(TPE1, src),
            b"TALB" => decode_text_frame!(TALB, src),
            b"TYER" => decode_text_frame!(TYER, src),
            b"TCON" => decode_text_frame!(TCON, src),
            b"TXXX" => decode_text_frame!(TXXX, src),
            b"COMM" => {
                let encoding = EncodingTypes::from_byte(src.get_u8())?;
                let mut language = [0u8; 3];
                src.copy_to_slice(&mut language);

                let short = src.get_encoded_string(&encoding);
                src.advance(2);

                let long = src.get_encoded_string(&encoding);

                FrameType::COMM(language, short, long)
            }
            _ => None?,
        })
    }
    pub fn get_description(&self) -> String {
        match self {
            FrameType::TIT2(_) => "Title/songname/content description",
            FrameType::TPE1(_) => "Lead performer(s)/Soloist(s)",
            FrameType::TALB(_) => "Album/Movie/Show title",
            FrameType::TYER(_) => "Year",
            FrameType::TCON(_) => "Content type",

            FrameType::TXXX(_) => "User defined text information frame",

            FrameType::COMM(_, _, _) => "Comments",
        }
        .into()
    }
}

bitflags! {
    pub struct FrameFlags: u16 {
        const TAG_ALTER_PRESERVATION = 0b10000000;
        const FILE_ALTER_PRESERVATION = 0b01000000;
        const READ_ONLY = 0b00100000;
        const COMPRESSION = 0b0000000010000000;
        const ENCRYPTION = 0b0000000001000000;
        const GROUPING = 0b0000000000100000;
    }
}
