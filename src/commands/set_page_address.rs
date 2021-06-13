use packed_struct::derive::PackedStruct;

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "msb")]
pub struct SetPageAddress {
    pub start: u16,
    pub end: u16,
}

#[cfg(test)]
mod test {
    use super::*;
    use packed_struct::PackedStruct;

    #[test]
    fn set_page_address() {
        let mut ca = SetPageAddress { start: 0, end: 0 };
        assert_eq!(ca.pack().unwrap(), [0, 0, 0, 0]);
        ca.start = u16::MAX;
        assert_eq!(ca.pack().unwrap(), [255, 255, 0, 0]);
        ca.end = u16::MAX;
        assert_eq!(ca.pack().unwrap(), [255, 255, 255, 255]);
    }
}
