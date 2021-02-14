use bytes::{Buf, Bytes};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};

use crate::{ID3v1, buf_ext::BufExt};

use super::ID3v1Ext;

pub fn read<R: Read + Seek>(src: &mut R) -> Result<ID3v1> {
    src.seek(SeekFrom::End(-128))?;
    let tag = {
        let mut dst = vec![0u8; 128];
        src.read_exact(&mut dst[..])?;
        let mut bytes = Bytes::from(dst);
        read_bytes(&mut bytes)?
    };

    src.seek(SeekFrom::End(-256))?;
    let tag = {
        let mut dst = vec![0u8; 128];
        src.read_exact(&mut dst[..])?;
        let mut bytes = Bytes::from(dst);
        match read_ext(&mut bytes) {
            Some(ext) => tag.merge(ext),
            None => tag,
        }
    };

    Ok(tag)
}

fn read_ext(mut bytes: &mut Bytes) -> Option<ID3v1Ext> {
    if &bytes[..3] != b"EXT" {
        bytes.advance(128);
        return None;
    }

    bytes.advance(3);
    Some(read_tag_ext(&mut bytes))
}

fn read_bytes(mut bytes: &mut Bytes) -> Result<ID3v1> {
    if &bytes[..3] == b"TAG" {
        bytes.advance(3);
        Ok(read_tag(&mut bytes))
    } else {
        Err(Error::new(ErrorKind::NotFound, "not a valid ID3v1 tag"))
    }
}

fn read_tag(bytes: &mut Bytes) -> ID3v1 {
    let song_name = bytes.get_latin1_string(30);
    let artist = bytes.get_latin1_string(30);
    let album_name = bytes.get_latin1_string(30);
    let year = bytes.get_latin1_string(4);
    
    let mut comment = vec![0u8; 30];
    bytes.copy_to_slice(&mut comment);

    let album_track = if comment[28] == 0 { comment[29] } else { 0 };
    let comment_pos = comment[..30].iter().position(|b| *b == 0).unwrap_or(30);
    let comment = encoding_rs::UTF_8.decode(&comment[..comment_pos]).0.into();
    let song_genre = bytes.get_u8();

    ID3v1 {
        song_name,
        artist,
        album_name,
        year,
        comment,
        album_track,
        song_genre,
        ..Default::default()
    }
}

fn read_tag_ext(bytes: &mut Bytes) -> ID3v1Ext {
    let song_name = bytes.get_latin1_string(30);
    let artist = bytes.get_latin1_string(30);
    let album_name = bytes.get_latin1_string(30);
    let comment = bytes.get_latin1_string(15);
    let sub_genre = bytes.get_latin1_string(20);

    ID3v1Ext {
        song_name,
        artist,
        album_name,
        comment,
        sub_genre
    }
}
