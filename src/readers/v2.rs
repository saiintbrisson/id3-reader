use crate::utils::{bytes, bytes::{Bytes, ContinuousReader}};

use crate::models::Version;
use crate::models::v2::{id3v2::ID3v2, header::*, frames::*};

pub fn read_id3v2(version: Version, mut bytes: &mut Bytes) -> ID3v2 {
    let flags = read_header_flags(bytes.read_byte());
    let size = bytes.read_signed(4) as usize;

    bytes.set_offset(10);
    bytes.set_cap(size);

    let mut frames: Vec<Frame> = Vec::new();

    while bytes.get_readable_bytes() > 10 {
        let frame = read_frame(&mut bytes);
        if frame.is_none() {
            continue
        }

        frames.push(frame.unwrap());
    }

    ID3v2 {
        version,
        header: Header { flags, size: size as u32 },
        frames
    }
}

fn read_frame(mut bytes: &mut Bytes) -> Option<Frame> {
    let mut id: [u8; 4] = [0; 4];
    bytes.read_to_slice(&mut id);

    let size = bytes.read_signed(4) as usize;

    let mut flags: [u8; 2] = [0; 2];
    bytes.read_to_slice(&mut flags);

    let value = parse_frame_body(&mut bytes, &id, size);
    if value.is_none() {
        return None
    }
    
    let value = value.unwrap();

    let frame_type = FrameType::from_slice(&id, value);
    if frame_type.is_none() {
        return None
    }

    let frame_type = frame_type.unwrap();
    let frame_flags = read_frame_flags(flags);

    Some(Frame { frame_type, frame_flags, size: size as u32 })
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
