use super::{DataFormat, DisplayError, WriteCommand, WriteOnlyDataCommand};
use packed_struct::{
    derive::{PackedStruct, PrimitiveEnum_u8},
    types::{bits::Bits11, Integer},
    PackedStruct,
};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum ZeroInLines {
    NormalInBoth = 0b00,
    ZeroInOddLines = 0b001,
    ZeroInEvenLines = 0b010,
    ZeroInBoth = 0b011,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum ToggleMode {
    Disable = 0b00,
    /// LSHIFT
    ToggleByPixelClock = 0b01,
    /// LLINE
    ToggleByLine = 0b010,
    /// LFRAME
    ToggleByFrame = 0b011,
}

macro_rules! LcdGen {
    ($struct_name:ident, $camel_case:ident, $code:literal) => {
        #[derive(PackedStruct, Debug, PartialEq)]
        #[packed_struct(bit_numbering = "msb0", endian = "msb")]
        pub struct $struct_name {
            #[packed_field(bits = "0")]
            pub reset_every_frame_start: bool,
            #[packed_field(bits = "13..=23")]
            pub generator_falling_position: Integer<u16, Bits11>,
            #[packed_field(bits = "29..=39")]
            pub generator_rising_position: Integer<u16, Bits11>,
            #[packed_field(bits = "40")]
            pub zero_in_non_display_period: bool,
            #[packed_field(bits = "41..=42", ty = "enum")]
            pub zero_in_lines: ZeroInLines,
            #[packed_field(bits = "43..=44", ty = "enum")]
            pub toggle_mode: ToggleMode,
            #[packed_field(bits = "45..=55")]
            pub period: Integer<u16, Bits11>,
        }

        impl WriteCommand for $struct_name {
            fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
            where
                I: WriteOnlyDataCommand,
            {
                ifc.send_commands(DataFormat::U8(&[$code]))?;
                ifc.send_data(DataFormat::U8(&self.pack().unwrap()))
            }
        }

        #[cfg(test)]
        mod $camel_case {
            use super::*;

            #[test]
            fn $camel_case() {
                let mut g0 = $struct_name {
                    reset_every_frame_start: false,
                    generator_falling_position: 0.into(),
                    generator_rising_position: 0.into(),
                    zero_in_non_display_period: false,
                    zero_in_lines: ZeroInLines::NormalInBoth,
                    toggle_mode: ToggleMode::Disable,
                    period: 0.into(),
                };
                assert_eq!(g0.pack().unwrap(), [0, 0, 0, 0, 0, 0, 0]);
                g0.reset_every_frame_start = true;
                assert_eq!(g0.pack().unwrap(), [0b10000000, 0, 0, 0, 0, 0, 0]);
                g0.generator_falling_position = 0b111_11111111.into();
                assert_eq!(g0.pack().unwrap(), [0b10000000, 0b111, 0b11111111, 0, 0, 0, 0]);
                g0.generator_rising_position = 0b111_11111111.into();
                assert_eq!(g0.pack().unwrap(), [0b10000000, 0b111, 0b11111111, 0b111, 0b11111111, 0, 0]);
                g0.zero_in_non_display_period = true;
                assert_eq!(
                    g0.pack().unwrap(),
                    [0b10000000, 0b111, 0b11111111, 0b111, 0b11111111, 0b10000000, 0]
                );
                g0.zero_in_lines = ZeroInLines::ZeroInBoth;
                assert_eq!(
                    g0.pack().unwrap(),
                    [0b10000000, 0b111, 0b11111111, 0b111, 0b11111111, 0b11100000, 0]
                );
                g0.toggle_mode = ToggleMode::ToggleByFrame;
                assert_eq!(
                    g0.pack().unwrap(),
                    [0b10000000, 0b111, 0b11111111, 0b111, 0b11111111, 0b11111000, 0]
                );
                g0.period = 0b111_11111111.into();
                assert_eq!(
                    g0.pack().unwrap(),
                    [0b10000000, 0b111, 0b11111111, 0b111, 0b11111111, 0b11111111, 0b11111111]
                );
            }
        }
    };
}

LcdGen!(SetLcdGen0, set_lcd_gen0, 0xC0);
LcdGen!(SetLcdGen1, set_lcd_gen1, 0xC2);
LcdGen!(SetLcdGen2, set_lcd_gen2, 0xC4);
LcdGen!(SetLcdGen3, set_lcd_gen3, 0xC6);
