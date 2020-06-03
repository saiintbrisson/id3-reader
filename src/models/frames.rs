use crate::bytes::{Bytes, ContinuousReader, decode_text};
use crate::models::frames::FrameType::*;

#[derive(Debug)]
pub struct Frame {

    pub frame_type: FrameType,
    pub frame_flags: FrameFlags,
    pub size: u32

}

#[derive(Debug)]
pub struct FrameFlags {

    pub discard: bool,
	pub read_only: bool,
	
	pub has_id: bool,

    pub compressed: bool,
    pub encrypted: bool,
	pub unsynchronized: bool,
	pub has_indicator: bool

}

#[derive(Debug)]
pub enum FrameType {

    TIT2(String),
    TPE1(String),
    TALB(String),
    TYER(String),

    TCON(String),

    COMM(String)

}

impl FrameType {

    pub fn from_slice(name: &[u8], value: String) -> Option<FrameType> {
        let name = String::from_utf8(name.into()).ok()?;
        FrameType::get_type(&name, value)
    }

    pub fn get_type(name: &String, value: String) -> Option<FrameType> {
        match name.as_ref() {
            "TIT2" => Some(TIT2(value)),
            "TPE1" => Some(TPE1(value)),
            "TALB" => Some(TALB(value)),
            "TYER" => Some(TYER(value)),

            "TCON" => Some(TCON(value)),

            "COMM" => Some(COMM(value)),
            _ => None
        }
    }

    pub fn get_description(&self) -> String {
        match self {
            TIT2(_) => "Title/songname/content description",
            TPE1(_) => "Lead performer(s)/Soloist(s)",
            TALB(_) => "Album/Movie/Show title",
            TYER(_) => "Year",

            TCON(_) => "Content type",

            COMM(_) => "Comments"
        }.to_string()
	}

    pub fn get_value(&self) -> String {
        match self {
            TIT2(value) => value,
            TPE1(value) => value,
            TALB(value) => value,
            TYER(value) => value,

            TCON(value) => value,

            COMM(value) => value
        }.to_string()
    }

}

pub fn parse_frame_body(bytes: &mut Bytes, id: &[u8], size: usize) -> Option<String> {
    if id[0] == b'T' {
        if id == [b'T', b'X', b'X', b'X'] {
            None
        } else {
            bytes.read_text(size)
        }
    } else if id == [b'C', b'O', b'M', b'M'] {
		let encoding = bytes.read_byte();
		
		bytes.read_latin1(3);
		let desc_size = bytes.get_next_null() + 2;

		let desc = bytes.read_slice(size - desc_size);
		decode_text(desc, encoding)
    } else {
        None
    }
}