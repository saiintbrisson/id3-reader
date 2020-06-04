use std::fs::File;
use std::io::{Seek, SeekFrom, BufReader};

use super::{bytes::{Bytes, ContinuousReader}};
use crate::{models::Version};

pub fn get_version(file: &mut File) -> (Option<Bytes>, Option<Version>) {
    let version = get_v2_version(file);
    if version.1.is_some() {
        return version
    }
    
    get_v1_version(file)
}

pub fn get_v1_version(file: &mut File) -> (Option<Bytes>, Option<Version>) {
    match file.seek(SeekFrom::End(-128)) {
        Ok(_) => {}
        Err(err) => {
            println!("Could not read file, {:?}", err);
            return (None, None)
        }
    }

    let mut reader = BufReader::new(file);

    let mut bytes = match Bytes::from_reader(&mut reader) {
        Ok(bytes) => bytes,
        Err(err) => {
            println!("Could not read file, {:?}", err);
            return (None, None)
        }
    };

    let tag = bytes.read_utf8(3).ok();
    if tag.is_none() {
        return (Some(bytes), None)
    }

    let tag = tag.unwrap();
    if tag != "TAG" {
        return (Some(bytes), None)
    }

    let version = Version {
        tag,
        major: 1,
        minor: 0,
        revision: 0
    };

    (Some(bytes), Some(version))
}

pub fn get_v2_version(file: &mut File) -> (Option<Bytes>, Option<Version>) {
    let mut reader = BufReader::new(file);
    
    let mut bytes = match Bytes::from_reader(&mut reader) {
        Ok(bytes) => bytes,
        Err(err) => {
            println!("Could not read file, {:?}", err);
            return (None, None)
        }
    };


    let version = bytes.read_utf8(3);
    if version.is_err() {
        return (Some(bytes), None)
    }
    
    let tag = version.unwrap();
    if tag != "ID3" {
        return (Some(bytes), None)
    }

    let version = Version {
        tag,
        major: 2,
        minor: bytes.read_byte(),
        revision: bytes.read_byte()
    };

    (Some(bytes), Some(version))
}
