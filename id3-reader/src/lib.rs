#![feature(bool_to_option)]
#![feature(try_blocks)]
#![feature(test)]

extern crate test;

#[macro_use]
extern crate bitflags;

pub mod buf_ext;
mod v1;
mod v2;

pub use v1::{decoder as v1_decoder, ID3v1};
pub use v2::{decoder as v2_decoder, header::Header};

pub struct ID3Version {
    pub v1: bool,
    pub v2: bool
}
