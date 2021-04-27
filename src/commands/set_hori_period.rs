use super::{DataFormat, DisplayError, WriteCommand, WriteOnlyDataCommand};
use packed_struct::{
    derive::PackedStruct,
    types::{
        bits::{Bits11, Bits2, Bits7},
        Integer,
    },
    PackedStruct,
};

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "msb")]
pub struct SetHoriPeriod {
    #[packed_field(bits = "5..=15")]
    pub ht: Integer<u16, Bits11>,
    #[packed_field(bits = "21..=31")]
    pub hps: Integer<u16, Bits11>,
    #[packed_field(bits = "33..=39")]
    pub hpw: Integer<u8, Bits7>,
    #[packed_field(bits = "45..=55")]
    pub lps: Integer<u16, Bits11>,
    #[packed_field(bits = "62..=63")]
    pub lpspp: Integer<u8, Bits2>,
}

impl WriteCommand for SetHoriPeriod {
    fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
    where
        I: WriteOnlyDataCommand,
    {
        ifc.send_commands(DataFormat::U8(&[0xB4]))?;
        ifc.send_data(DataFormat::U8(&self.pack().unwrap()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_hori_period() {
        let mut hp = SetHoriPeriod {
            ht: 0.into(),
            hps: 0.into(),
            hpw: 0.into(),
            lps: 0.into(),
            lpspp: 0.into(),
        };
        assert_eq!(hp.pack().unwrap(), [0, 0, 0, 0, 0, 0, 0, 0]);
        hp.ht = 0b111_11111111.into();
        assert_eq!(hp.pack().unwrap(), [0b111, 0b11111111, 0, 0, 0, 0, 0, 0]);
        hp.hps = 0b111_11111111.into();
        assert_eq!(hp.pack().unwrap(), [0b111, 0b11111111, 0b111, 0b11111111, 0, 0, 0, 0]);
        hp.hpw = 0b1111111.into();
        assert_eq!(hp.pack().unwrap(), [0b111, 0b11111111, 0b111, 0b11111111, 0b1111111, 0, 0, 0]);
        hp.lps = 0b111_11111111.into();
        assert_eq!(
            hp.pack().unwrap(),
            [0b111, 0b11111111, 0b111, 0b11111111, 0b1111111, 0b111, 0b11111111, 0]
        );
        hp.lpspp = 0b11.into();
        assert_eq!(
            hp.pack().unwrap(),
            [0b111, 0b11111111, 0b111, 0b11111111, 0b1111111, 0b111, 0b11111111, 0b11]
        );
    }
}
