use super::{DataFormat, DisplayError, WriteCommand, WriteOnlyDataCommand};
use packed_struct::{
    derive::{PackedStruct, PrimitiveEnum_u8},
    PackedStruct,
};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Output {
    Zero = 0,
    One = 1,
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0")]
pub struct SetPostProc {
    pub contrast: u8,
    pub brightness: u8,
    pub saturation: u8,
    #[packed_field(bits = "31")]
    pub enable: bool,
}

impl WriteCommand for SetPostProc {
    fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
    where
        I: WriteOnlyDataCommand,
    {
        ifc.send_commands(DataFormat::U8(&[0xBC]))?;
        ifc.send_data(DataFormat::U8(&self.pack().unwrap()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_post_proc() {
        let mut pp = SetPostProc {
            contrast: 0,
            brightness: 0,
            saturation: 0,
            enable: false,
        };
        assert_eq!(pp.pack().unwrap(), [0, 0, 0, 0]);
        pp.contrast = 0b11000111;
        assert_eq!(pp.pack().unwrap(), [0b11000111, 0, 0, 0]);
        pp.brightness = 0b11000111;
        assert_eq!(pp.pack().unwrap(), [0b11000111, 0b11000111, 0, 0]);
        pp.saturation = 0b11000111;
        assert_eq!(pp.pack().unwrap(), [0b11000111, 0b11000111, 0b11000111, 0]);
        pp.enable = true;
        assert_eq!(pp.pack().unwrap(), [0b11000111, 0b11000111, 0b11000111, 0b1]);
    }
}
