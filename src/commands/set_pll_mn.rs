use packed_struct::{
    derive::PackedStruct,
    types::{bits::Bits5, Integer},
};

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0")]
pub struct SetPllMn {
    #[packed_field(bits = "0..=7")]
    pub pll_multiplier: u8,
    #[packed_field(bits = "11..=15")]
    pub pll_divider: Integer<u8, Bits5>,
    #[packed_field(bits = "21")]
    pub use_multiplier_and_divider: bool,
}

#[cfg(test)]
mod test {
    use super::*;
    use packed_struct::PackedStruct;

    #[test]
    fn set_pll_mn() {
        let mut pc = SetPllMn {
            pll_multiplier: 0,
            pll_divider: 0.into(),
            use_multiplier_and_divider: false,
        };
        assert_eq!(pc.pack().unwrap(), [0, 0, 0]);
        pc.pll_multiplier = 255;
        assert_eq!(pc.pack().unwrap(), [255, 0, 0]);
        pc.pll_divider = 0b11111.into();
        assert_eq!(pc.pack().unwrap(), [255, 0b11111, 0]);
        pc.use_multiplier_and_divider = true;
        assert_eq!(pc.pack().unwrap(), [255, 0b11111, 0b100]);
    }
}
