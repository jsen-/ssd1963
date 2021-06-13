use packed_struct::derive::PackedStruct;

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "msb")]
pub struct SetScrollStart {
    pub source_line: u16,
}

#[cfg(test)]
mod test {
    use super::*;
    use packed_struct::PackedStruct;

    #[test]
    fn set_scroll_start() {
        let mut ss = SetScrollStart { source_line: 0 };
        assert_eq!(ss.pack().unwrap(), [0, 0]);
        ss.source_line = 0b11111111_00000001;
        assert_eq!(ss.pack().unwrap(), [255, 1]);
    }
}
