use super::super::{Version, v2::{header::*, frames::*}};

#[derive(Debug)]
pub struct ID3v2 {

    pub version: Version,
    pub header: Header,
    pub frames: Vec<Frame>

}
