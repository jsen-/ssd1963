use packed_struct::derive::{PackedStruct, PrimitiveEnum_u8};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Output {
    Zero = 0,
    One = 1,
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0")]
pub struct SetGpioValue {
    #[packed_field(bits = "4", ty = "enum")]
    pub gpio3_value: Output,
    #[packed_field(bits = "5", ty = "enum")]
    pub gpio2_value: Output,
    #[packed_field(bits = "6", ty = "enum")]
    pub gpio1_value: Output,
    #[packed_field(bits = "7", ty = "enum")]
    pub gpio0_value: Output,
}

#[cfg(test)]
mod test {
    use super::*;
    use packed_struct::PackedStruct;

    #[test]
    fn set_gpio_conf() {
        let mut gv = SetGpioValue {
            gpio3_value: Output::Zero,
            gpio2_value: Output::Zero,
            gpio1_value: Output::Zero,
            gpio0_value: Output::Zero,
        };
        assert_eq!(gv.pack().unwrap(), [0]);
        gv.gpio3_value = Output::One;
        assert_eq!(gv.pack().unwrap(), [0b1000]);
        gv.gpio2_value = Output::One;
        assert_eq!(gv.pack().unwrap(), [0b1100]);
        gv.gpio1_value = Output::One;
        assert_eq!(gv.pack().unwrap(), [0b1110]);
        gv.gpio0_value = Output::One;
        assert_eq!(gv.pack().unwrap(), [0b1111]);
    }
}
