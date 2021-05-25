use packed_struct::derive::{PackedStruct, PrimitiveEnum_u8};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelDataInterfaceFormat {
    /// 8-bit
    B8 = 0b000,
    /// 12-bit
    B12 = 0b001,
    /// 16-bit packed
    B16Packed = 0b010,
    /// 16-bit (565 format) packed
    B16Format565 = 0b011,
    /// 18-bit
    B18 = 0b100,
    /// 24-bit
    B24 = 0b101,
    /// 9-bit
    B9 = 0b110,
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "msb")]
/// DescriptionSet the pixel data format to 8-bit / 9-bit / 12-bit / 16-bit / 16-bit(565) / 18-bit / 24-bit in the parallel host processor interface.
/// This command is used for display data only, the command format is always 8 bit.
/// Note: The unused data bus will be driven to ground by SSD1963, so don’t connect the un-used data bus to MCU.
pub struct SetPixelDataInterface {
    #[packed_field(bits = "5..=7", ty = "enum")]
    pub pixel_data_interface_format: PixelDataInterfaceFormat,
}

#[cfg(test)]
mod test {
    use super::*;
    use packed_struct::PackedStruct;

    #[test]
    fn set_pixel_data_interface() {
        let mut lf = SetPixelDataInterface {
            pixel_data_interface_format: PixelDataInterfaceFormat::B8,
        };
        assert_eq!(lf.pack().unwrap(), [0]);
        lf.pixel_data_interface_format = PixelDataInterfaceFormat::B9;
        assert_eq!(lf.pack().unwrap(), [0b110]);
    }
}
