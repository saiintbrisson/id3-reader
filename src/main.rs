pub mod utils;
pub mod models;

use std::fs::File;
use std::env::{args};

use std::io::{BufReader};

use utils::{bytes, bytes::{Bytes, ContinuousReader}};

use models::Version;

use models::v2::header::{Header, Flags};
use models::v2::frames::*;

fn main() {
    let mut reader = BufReader::new(select_file().expect(""));
    let mut bytes = Bytes::from_reader(&mut reader).expect("a");

    let version = read_version(&mut bytes);
    if version.is_none() {
        return
    }
    let version = version.unwrap();

    let flags = read_header_flags(bytes.read_byte());

    let size = bytes.read_signed(4) as usize;

    bytes.set_offset(10);
    bytes.set_cap(size);

    let mut frames: Vec<Frame> = Vec::new();

    while bytes.get_readable_bytes() > 10 {
        let mut id: [u8; 4] = [0; 4];
        bytes.read_to_slice(&mut id);

        let size = bytes.read_signed(4) as usize;

        let mut flags: [u8; 2] = [0; 2];
        bytes.read_to_slice(&mut flags);

        let value = parse_frame_body(&mut bytes, &id, size);
        if value.is_none() {
            continue;
        }
        let value = value.unwrap();

        let frame_type = FrameType::from_slice(&id, value);
        if frame_type.is_none() {
            continue;
        }
        let frame_type = frame_type.unwrap();

        let frame_flags = read_frame_flags(flags);

        frames.push(Frame { frame_type, frame_flags, size: size as u32 });
    }
    
    let header = Header {
        flags, size: size as u32
    };

    println!("{:#?}", version);
    println!("{:#?}", header);

    for x in frames {
        println!("{:#?}", x.frame_type);
    }
}

pub fn select_file() -> Result<File, String> {
    let args_vec: Vec<String> = args().collect();

    if args_vec.len() < 2 {
        return Result::Err("First command arg should be the file to be selected".to_string())
    }

    Ok(
        File::open(&args_vec[1])
            .expect("Could not open file")
    )
}

fn read_version(bytes: &mut Bytes) -> Option<Version> {
    let version = bytes.read_utf8(3);
    if version.is_err() {
        return None
    }
    
    Some(Version {
        tag: version.unwrap(),
        major: 2,
        minor: bytes.read_byte(),
        revision: bytes.read_byte()
    })
}

fn read_header_flags(byte: u8) -> Flags {
    Flags {
        unsynchronized: bytes::bit_at(byte, 7),
        extended: bytes::bit_at(byte, 6),
        experimental: bytes::bit_at(byte, 5),
        has_footer: bytes::bit_at(byte, 4)
    }
}

fn read_frame_flags(slice: [u8; 2]) -> FrameFlags {
    FrameFlags {
        discard: bytes::bit_at(slice[0], 6) || bytes::bit_at(slice[0], 5),
        read_only: bytes::bit_at(slice[0], 4),

        has_id: bytes::bit_at(slice[1], 6),
        
        compressed: bytes::bit_at(slice[1], 3),
        encrypted: bytes::bit_at(slice[1], 2),
        unsynchronized: bytes::bit_at(slice[1], 1),
        has_indicator: bytes::bit_at(slice[1], 0)
    }
}
