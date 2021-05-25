use super::{DataFormat, DisplayError, WriteCommand, WriteOnlyDataCommand};
use packed_struct::{
    derive::{PackedStruct, PrimitiveEnum_u8},
    PackedStruct,
};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Source {
    Generator0 = 0b00,
    Generator1 = 0b01,
    Generator2 = 0b10,
    Generator3 = 0b11,
}

macro_rules! gpio_rop {
    ($struct_name:ident, $camel_case:ident, $code:literal) => {
        #[derive(PackedStruct, Debug, PartialEq)]
        #[packed_struct(bit_numbering = "msb0")]
        pub struct $struct_name {
            /// Source 1 for GPIO3 when controlled by LCDC
            #[packed_field(bits = "1..=2", ty = "enum")]
            pub source1: Source,

            /// Source 2 for GPIO3 when controlled by LCDC
            #[packed_field(bits = "4..=5", ty = "enum")]
            pub source2: Source,

            /// Source 3 for GPIO3 when controlled by LCDC
            #[packed_field(bits = "6..=7", ty = "enum")]
            pub source3: Source,

            /// ROP operation to mux the source 1, 2 and 3 for this GPIO
            /// Please refer to the Application note for the ROP operation
            #[packed_field(bits = "8..=15")]
            pub rop_operation_mux: u8,
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
                let mut gc = $struct_name {
                    source1: Source::Generator0,
                    source2: Source::Generator0,
                    source3: Source::Generator0,
                    rop_operation_mux: 0,
                };
                assert_eq!(gc.pack().unwrap(), [0, 0]);
                gc.source1 = Source::Generator3;
                assert_eq!(gc.pack().unwrap(), [0b01100000, 0]);
                gc.source2 = Source::Generator1;
                assert_eq!(gc.pack().unwrap(), [0b01100100, 0]);
                gc.source3 = Source::Generator2;
                assert_eq!(gc.pack().unwrap(), [0b01100110, 0]);
                gc.rop_operation_mux = 255;
                assert_eq!(gc.pack().unwrap(), [0b01100110, 255]);

            }
        }
    }
}

gpio_rop!(SetGpio0Rop, set_gpio0_rop, 0xC8);
gpio_rop!(SetGpio1Rop, set_gpio1_rop, 0xCA);
gpio_rop!(SetGpio2Rop, set_gpio2_rop, 0xCC);
gpio_rop!(SetGpio3Rop, set_gpio3_rop, 0xCD);
