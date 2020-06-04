use crate::utils::bytes::{Bytes, ContinuousReader};

use crate::models::Version;
use crate::models::v1::id3v1::ID3v1;

pub fn read_id3v1(version: Version, bytes: &mut Bytes) -> ID3v1 {
    bytes.set_offset(3);

    ID3v1 {
        version,

        name: bytes.read_latin1(30),
        artist: bytes.read_latin1(30),
        album: bytes.read_latin1(30),
        year: bytes.read_latin1(4),
        comment: bytes.read_latin1(30),
        genre: bytes.read_byte() as i8,
    }
}