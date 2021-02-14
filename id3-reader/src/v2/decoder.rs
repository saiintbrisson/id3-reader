use bytes::{Buf, Bytes};
use std::io::{BufReader, Error, ErrorKind, Read, Result};

use crate::buf_ext::BufExt;

use super::{ID3v2, frame::{Frame, FrameFlags, FrameType}, header::{ExtendedHeader, ExtendedHeaderFlags, Header, HeaderFlags}};

const ZEROED_ID: [u8; 4] = [0u8; 4];

pub fn read<R: Read>(src: &mut R) -> Result<ID3v2> {
    let mut reader = BufReader::new(src);

    let header = read_header(&mut reader)?;

    let mut remaining = header.size as u32;
    let mut frames = vec![];
    while remaining > 10 {
        let frame = match read_frame(&mut reader) {
            Ok(frame) => frame,
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => { continue; },
                    ErrorKind::Other => { break; },
                    _ => Err(err)?
                }
            }
        };

        remaining -= frame.size + 10;
        frames.push(frame);
    }

    Ok(ID3v2 {
        header,
        frames
    })
}

fn read_header<R: Read>(mut src: &mut BufReader<R>) -> Result<Header> {
    let mut read = vec![0u8; 10];
    src.read_exact(&mut read)?;

    let mut bytes = Bytes::from(read);

    if &bytes[..3] != b"ID3" {
        return Err(Error::new(ErrorKind::NotFound, "not a valid ID3v2 tag"))
    }
    bytes.advance(3);

    let version = bytes.get_u16();
    let header_flags = HeaderFlags::from_bits(bytes.get_u8())
        .ok_or(Error::new(ErrorKind::InvalidInput, "invalid header flags"))?;
    let mut size = bytes.get_synchsafe_int();

    let extended_header = match header_flags.contains(HeaderFlags::EXTENDED_HEADER)
        .then(|| { size = size - 10; read_ext_header(&mut src) }) {
        Some(ext) => Some(ext?),
        None => None
    };

    Ok(Header {
        version: ((version >> 8) as u8, version as u8),
        header_flags,
        size,
        extended_header
    })
}

fn read_ext_header<R: Read>(src: &mut BufReader<R>) -> Result<ExtendedHeader> {
    let mut read = vec![0u8; 10];
    src.read_exact(&mut read)?;

    let mut bytes = Bytes::from(read);

    let extended_header_size = bytes.get_u32();
    let extended_flags = ExtendedHeaderFlags::from_bits(bytes.get_u16())
        .ok_or(Error::new(ErrorKind::InvalidInput, "invalid extended header flags"))?;
    let padding_size = bytes.get_u32();
    let total_frame_crc = extended_flags.contains(ExtendedHeaderFlags::CRC_DATA).then_some(bytes.get_u32())    
        .ok_or(Error::new(ErrorKind::InvalidInput, "missing crc data"))?;

    Ok(ExtendedHeader {
        extended_header_size,
        extended_flags,
        padding_size,
        total_frame_crc
    })
}

fn read_frame<R: Read>(src: &mut BufReader<R>) -> Result<Frame> {
    let mut read = vec![0u8; 10];
    src.read_exact(&mut read)?;

    let mut bytes = Bytes::from(read);

    let mut id = [0u8; 4];
    bytes.copy_to_slice(&mut id);

    if &id == &ZEROED_ID {
        return Err(Error::from(ErrorKind::Other));
    }

    let size = bytes.get_synchsafe_uint() as u32;
    let flags = FrameFlags::from_bits(bytes.get_u16())
        .ok_or(Error::new(ErrorKind::InvalidInput, format!("invalid frame flags on {:?}", id)))?;

    let mut read = vec![0u8; size as usize];
    src.read_exact(&mut read)?;

    let mut bytes = Bytes::from(read);
    let frame_type = FrameType::from_name(&id[..], &mut bytes)
        .ok_or(Error::new(ErrorKind::NotFound, format!("invalid frame {:?}", id)))?;

    Ok(Frame {
        frame_type,
        size,
        flags,
    })
}
