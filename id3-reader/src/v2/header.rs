#[derive(Debug)]
pub struct Header {
    pub version: (u8, u8),
    pub header_flags: HeaderFlags,
    pub size: i32,
    pub extended_header: Option<ExtendedHeader>,
}

bitflags! {
    pub struct HeaderFlags: u8 {
        const UNSYNCHRONISATION = 0b10000000;
        const EXTENDED_HEADER = 0b01000000;
        const EXPERIMENTAL = 0b00100000;
        const FOOTER = 0b00010000;
    }
}

#[derive(Debug)]
pub struct ExtendedHeader {
    pub extended_header_size: u32,
    pub extended_flags: ExtendedHeaderFlags,
    pub padding_size: u32,
    pub total_frame_crc: u32,
}

bitflags! {
    pub struct ExtendedHeaderFlags: u16 {
        const CRC_DATA = 0b10000000;
    }
}
