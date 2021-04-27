use super::{DataFormat, DisplayError, WriteCommand, WriteOnlyDataCommand};
use packed_struct::{
    derive::PackedStruct,
    types::{
        bits::{Bits11, Bits7},
        Integer,
    },
    PackedStruct,
};

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "msb")]
pub struct SetVertPeriod {
    #[packed_field(bits = "5..=15")]
    pub vt: Integer<u16, Bits11>,
    #[packed_field(bits = "21..=31")]
    pub vps: Integer<u16, Bits11>,
    #[packed_field(bits = "33..=39")]
    pub vpw: Integer<u8, Bits7>,
    #[packed_field(bits = "45..=55")]
    pub fps: Integer<u16, Bits11>,
}

impl WriteCommand for SetVertPeriod {
    fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
    where
        I: WriteOnlyDataCommand,
    {
        ifc.send_commands(DataFormat::U8(&[0xB6]))?;
        ifc.send_data(DataFormat::U8(&self.pack().unwrap()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_vert_period() {
        let mut vp = SetVertPeriod {
            vt: 0.into(),
            vps: 0.into(),
            vpw: 0.into(),
            fps: 0.into(),
        };
        assert_eq!(vp.pack().unwrap(), [0, 0, 0, 0, 0, 0, 0]);
        vp.vt = 0b111_11111111.into();
        assert_eq!(vp.pack().unwrap(), [0b111, 0b11111111, 0, 0, 0, 0, 0]);
        vp.vps = 0b111_11111111.into();
        assert_eq!(vp.pack().unwrap(), [0b111, 0b11111111, 0b111, 0b11111111, 0, 0, 0]);
        vp.vpw = 0b1111111.into();
        assert_eq!(vp.pack().unwrap(), [0b111, 0b11111111, 0b111, 0b11111111, 0b1111111, 0, 0]);
        vp.fps = 0b111_11111111.into();
        assert_eq!(vp.pack().unwrap(), [0b111, 0b11111111, 0b111, 0b11111111, 0b1111111, 0b111, 0b11111111]);
    }
}
