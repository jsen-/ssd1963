use packed_struct::derive::PackedStruct;

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "msb")]
pub struct SetScrollArea {
    pub top_fixed_area: u16,
    pub vertical_scroll_area: u16,
    pub bottom_fixed_area: u16,
}

#[cfg(test)]
mod test {
    use super::*;
    use packed_struct::PackedStruct;

    #[test]
    fn set_scroll_area() {
        let mut sa = SetScrollArea {
            top_fixed_area: 0,
            vertical_scroll_area: 0,
            bottom_fixed_area: 0,
        };
        assert_eq!(sa.pack().unwrap(), [0, 0, 0, 0, 0, 0]);
        sa.top_fixed_area = 0b11111111_00000001;
        assert_eq!(sa.pack().unwrap(), [255, 1, 0, 0, 0, 0]);
        sa.vertical_scroll_area = 0b11111111_00000001;
        assert_eq!(sa.pack().unwrap(), [255, 1, 255, 1, 0, 0]);
        sa.bottom_fixed_area = 0b11111111_00000001;
        assert_eq!(sa.pack().unwrap(), [255, 1, 255, 1, 255, 1]);
    }
}
