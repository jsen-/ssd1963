use super::{DataFormat, DisplayError, WriteCommand, WriteOnlyDataCommand};
use packed_struct::{
    derive::{PackedStruct, PrimitiveEnum_u8},
    types::{bits::Bits11, Integer},
    PackedStruct,
};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum TftPanelDataWidth {
    /// 18-bit
    B18 = 0,
    /// 24-bit
    B24 = 1,
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Edge {
    Falling = 0,
    Rising = 1,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Active {
    ActiveLow = 0,
    ActiveHigh = 1,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum TftType {
    TftMode0 = 0b00,
    TftMode1 = 0b01,
    SerialRgbMode = 0b10,
    SerialRgbDummyMode = 0b11,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum ColorSequence {
    Rgb = 0b000,
    Rbg = 0b001,
    Grb = 0b010,
    Gbr = 0b011,
    Brg = 0b100,
    Bgr = 0b101,
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "msb")]
pub struct SetLcdMode {
    #[packed_field(bits = "2", ty = "enum")]
    pub data_width: TftPanelDataWidth,
    #[packed_field(bits = "3")]
    pub color_depth_enhancement_enable: bool,
    #[packed_field(bits = "4")]
    pub frc_enable: bool,
    #[packed_field(bits = "5", ty = "enum")]
    pub lshift_polarity: Edge,
    #[packed_field(bits = "6", ty = "enum")]
    pub lline_polarity: Active,
    #[packed_field(bits = "7", ty = "enum")]
    pub lframe_polarity: Active,
    #[packed_field(bits = "9..=10", ty = "enum")]
    pub tft_type: TftType,
    #[packed_field(bits = "21..=31")]
    pub hdp: Integer<u16, Bits11>,
    #[packed_field(bits = "37..=47")]
    pub vdp: Integer<u16, Bits11>,
    #[packed_field(bits = "50..=52", ty = "enum")]
    pub even_line_color_sequence: ColorSequence,
    #[packed_field(bits = "53..=55", ty = "enum")]
    pub odd_line_color_sequence: ColorSequence,
}

impl WriteCommand for SetLcdMode {
    fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
    where
        I: WriteOnlyDataCommand,
    {
        ifc.send_commands(DataFormat::U8(&[0xB0]))?;
        ifc.send_data(DataFormat::U8(&self.pack().unwrap()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_lcd_mode() {
        let mut lm = SetLcdMode {
            data_width: TftPanelDataWidth::B18,
            color_depth_enhancement_enable: false,
            frc_enable: false,
            lshift_polarity: Edge::Falling,
            lline_polarity: Active::ActiveLow,
            lframe_polarity: Active::ActiveLow,
            tft_type: TftType::TftMode0,
            hdp: 0.into(),
            vdp: 0.into(),
            even_line_color_sequence: ColorSequence::Rgb,
            odd_line_color_sequence: ColorSequence::Rgb,
        };
        assert_eq!(lm.pack().unwrap(), [0, 0, 0, 0, 0, 0, 0]);
        lm.data_width = TftPanelDataWidth::B24;
        assert_eq!(lm.pack().unwrap(), [0b00100000, 0, 0, 0, 0, 0, 0]);
        lm.color_depth_enhancement_enable = true;
        assert_eq!(lm.pack().unwrap(), [0b00110000, 0, 0, 0, 0, 0, 0]);
        lm.frc_enable = true;
        assert_eq!(lm.pack().unwrap(), [0b00111000, 0, 0, 0, 0, 0, 0]);
        lm.lshift_polarity = Edge::Rising;
        assert_eq!(lm.pack().unwrap(), [0b00111100, 0, 0, 0, 0, 0, 0]);
        lm.lline_polarity = Active::ActiveHigh;
        assert_eq!(lm.pack().unwrap(), [0b00111110, 0, 0, 0, 0, 0, 0]);
        lm.lframe_polarity = Active::ActiveHigh;
        assert_eq!(lm.pack().unwrap(), [0b00111111, 0, 0, 0, 0, 0, 0]);
        lm.tft_type = TftType::SerialRgbDummyMode;
        assert_eq!(lm.pack().unwrap(), [0b00111111, 0b01100000, 0, 0, 0, 0, 0]);
        lm.hdp = 0b111_11111111.into();
        assert_eq!(lm.pack().unwrap(), [0b00111111, 0b01100000, 0b111, 0b11111111, 0, 0, 0]);
        lm.vdp = 0b111_11111111.into();
        assert_eq!(lm.pack().unwrap(), [0b00111111, 0b01100000, 0b111, 0b11111111, 0b111, 0b11111111, 0]);
        lm.even_line_color_sequence = ColorSequence::Bgr;
        assert_eq!(
            lm.pack().unwrap(),
            [0b00111111, 0b01100000, 0b111, 0b11111111, 0b111, 0b11111111, 0b00101000]
        );
        lm.odd_line_color_sequence = ColorSequence::Bgr;
        assert_eq!(
            lm.pack().unwrap(),
            [0b00111111, 0b01100000, 0b111, 0b11111111, 0b111, 0b11111111, 0b00101101]
        );
    }
}
