pub mod utils;
pub mod models;

use std::fs::File;

use std::io::{BufReader};

use clap::{Arg, App};

use utils::{bytes, bytes::{Bytes, ContinuousReader}};

use models::Version;

use models::v2::header::{Header, Flags};
use models::v2::frames::*;

fn main() {
	let matches = App::new("ID3 Reader")
	.arg(Arg::with_name("file")
			 .short("f")
			 .long("file")
			 .takes_value(true)
			 .required(true)
			 .help("The file to be parsed"))
	.get_matches();

	let file_name = matches.value_of("file");
	if file_name.is_none() {
		println!("Please specify the file to be parsed with --file <name>");
		return
	}

	let file_name = file_name.unwrap();

	let mut file = match File::open(file_name) {
		Ok(file) => file,
		Err(err) => {
			println!("Could not open file, {:?}", err);
			return
		}
	};

    let version = read_version(&mut file);
    if version.0.is_none() || version.1.is_none() {
		println!("Invalid ID3 file");
        return
    }
	let mut bytes = version.0.unwrap();
	let version = version.1.unwrap();
	
	println!("{:#?}", version);

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

    println!("{:#?}", header);

    for x in frames {
        println!("{:#?}", x.frame_type);
    }
}

use std::io::{Seek, SeekFrom};

fn read_version(file: &mut File) -> (Option<Bytes>, Option<Version>) {
	let version = get_v2_version(file);
	if version.1.is_some() {
		return version
	}
	
	get_v1_version(file)
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

	let version = Some(Version {
		tag,
		major: 2,
		minor: bytes.read_byte(),
		revision: bytes.read_byte()
	});

	(Some(bytes), version)
}

pub fn get_v1_version(file: &mut File) -> (Option<Bytes>, Option<Version>) {
    file.seek(SeekFrom::End(-128))
        .expect("Could not perform IO operation");

	let mut reader = BufReader::new(file);
    let mut bytes = Bytes::from_reader(&mut reader).unwrap();

	let tag = bytes.read_utf8(3).ok();
	if tag.is_none() {
		return (Some(bytes), None)
	}

	let tag = tag.unwrap();
	if tag != "TAG" {
		return (Some(bytes), None)
	}

    (Some(bytes), Some(Version {
        tag,
        major: 1,
        minor: 0,
        revision: 0
    }))
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
