use packed_struct::derive::{PackedStruct, PrimitiveEnum_u8};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum PageAddressOrder {
    TopToBottom = 0,
    BottomToTop = 1,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum ColumnAddressOrder {
    LeftToRight = 0,
    RightToLeft = 1,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum PageColumnOrder {
    Normal = 0,
    Reverse = 1,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum LineAddressOrder {
    LcdRefreshTopToBottom = 0,
    LcdRefreshBottomToTop = 1,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum ColorOrder {
    Rgb = 0,
    Bgr = 1,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum DataLatchOrder {
    LcdRefreshLeftToRight = 0,
    LcdRefreshRightToLeft = 1,
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0")]
pub struct SetAddressMode {
    #[packed_field(bits = "0", ty = "enum")]
    pub page_address_order: PageAddressOrder,
    #[packed_field(bits = "1", ty = "enum")]
    pub column_address_order: ColumnAddressOrder,
    #[packed_field(bits = "2", ty = "enum")]
    pub page_column_order: PageColumnOrder,
    #[packed_field(bits = "3", ty = "enum")]
    pub line_address_order: LineAddressOrder,
    #[packed_field(bits = "4", ty = "enum")]
    pub color_order: ColorOrder,
    #[packed_field(bits = "5", ty = "enum")]
    pub data_latch_order: DataLatchOrder,
    #[packed_field(bits = "6")]
    pub flip_horizontal: bool,
    #[packed_field(bits = "7")]
    pub flip_vertical: bool,
}

#[cfg(test)]
mod test {
    use super::*;
    use packed_struct::PackedStruct;

    #[test]
    fn set_addr_mode() {
        let mut zero = SetAddressMode {
            page_address_order: PageAddressOrder::TopToBottom,
            column_address_order: ColumnAddressOrder::LeftToRight,
            page_column_order: PageColumnOrder::Normal,
            line_address_order: LineAddressOrder::LcdRefreshTopToBottom,
            color_order: ColorOrder::Rgb,
            data_latch_order: DataLatchOrder::LcdRefreshLeftToRight,
            flip_horizontal: false,
            flip_vertical: false,
        };
        assert_eq!(zero.pack().unwrap(), [0]);
        zero.page_address_order = PageAddressOrder::BottomToTop;
        assert_eq!(zero.pack().unwrap(), [1 << 7]);
        zero.page_address_order = PageAddressOrder::TopToBottom;
        assert_eq!(zero.pack().unwrap(), [0]);

        zero.page_address_order = PageAddressOrder::BottomToTop;
        zero.column_address_order = ColumnAddressOrder::RightToLeft;
        zero.page_column_order = PageColumnOrder::Reverse;
        zero.line_address_order = LineAddressOrder::LcdRefreshBottomToTop;
        zero.color_order = ColorOrder::Bgr;
        zero.data_latch_order = DataLatchOrder::LcdRefreshRightToLeft;
        zero.flip_horizontal = true;
        zero.flip_vertical = true;
        assert_eq!(zero.pack().unwrap(), [255]);
    }
}
