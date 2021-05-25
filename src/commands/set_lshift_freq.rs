use packed_struct::{
    derive::{PackedStruct, PrimitiveEnum_u8},
    types::{bits::Bits20, Integer},
};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum EnergySaving {
    DbcDisable = 0b00,
    ConservativeMode = 0b01,
    NormalMode = 0b10,
    AgressiveMode = 0b11,
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "msb")]
/// Set the LSHIFT (pixel clock) frequency
pub struct SetLShiftFreq {
    /// For parallel LCD interface, configure the pixel clock to:
    ///   PLL freq x ((LCDC_FPR + 1) / 2^20)
    ///   To obtain PCLK = 5.3MHz with PLL Frequency = 100MHz:
    ///     5.3MHz = 100MHz * ( LCDC_FPR+ 1) / 2^20
    ///     LCDC_FPR = 55574
    /// For serial LCD interface, configure the pixel clock to:
    ///   PLL freq x ((LCDC_FPR + 1) / 2^20) *4
    ///   To obtain PCLK = 5.3MHz with PLL Frequency = 100MHz:
    ///     5.3MHz = 100MHz * ( ( LCDC_FPR+ 1) / 2^20   )*4
    ///     LCDC_FPR = 13892
    #[packed_field(bits = "4..=23")]
    pub lcdc_fpr: Integer<u32, Bits20>,
}

#[cfg(test)]
mod test {
    use super::*;
    use packed_struct::PackedStruct;

    #[test]
    fn set_lshift_freq() {
        let mut lf = SetLShiftFreq { lcdc_fpr: 0.into() };
        assert_eq!(lf.pack().unwrap(), [0, 0, 0]);
        lf.lcdc_fpr = 0b1111_11111111_11111111.into();
        assert_eq!(lf.pack().unwrap(), [0b1111, 255, 255]);
    }
}
