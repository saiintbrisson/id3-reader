pub mod models;

pub mod readers;

pub mod utils;

use std::fs::File;
use clap::{Arg, App};

use utils::{bytes, identifier::get_version};
use readers::{v1, v2};

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

    let version = get_version(&mut file);
    if version.0.is_none() || version.1.is_none() {
        println!("Invalid ID3 file");
        return
    }
    
    let mut bytes = version.0.unwrap();
    let version = version.1.unwrap();

    if version.is_v2() {
        println!("{:#?}", v2::read_id3v2(version, &mut bytes));
    } else {
        println!("{:#?}", v1::read_id3v1(version, &mut bytes));
        println!("ID3v1 is not currently supported");
    }
}
