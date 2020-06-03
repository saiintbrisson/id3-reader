#[derive(Debug)]
pub struct Header {

    pub flags: Flags,
    pub size: u32

}

#[derive(Debug)]
pub struct Flags {

    pub unsynchronized: bool,
    pub extended: bool,
    pub experimental: bool,
    pub has_footer: bool

}