#[derive(Debug)]
pub struct Version {

    pub tag: String,
    pub major: u8,
    pub minor: u8,
    pub revision: u8

}

impl Version {

    pub fn is_v2(&self) -> bool {
        self.tag == "ID3"
    }

}