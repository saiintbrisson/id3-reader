use super::super::{Version};

#[derive(Debug)]
pub struct ID3v1 {

    pub version: Version,

    pub name: String,
    pub artist: String,
    pub album: String,
    pub year: String,
    pub comment: String,    
    pub genre: i8,    

}