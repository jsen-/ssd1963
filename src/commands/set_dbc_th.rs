use super::{DataFormat, DisplayError, WriteCommand, WriteOnlyDataCommand};
use packed_struct::{
    derive::{PackedStruct, PrimitiveEnum_u8},
    types::{
        bits::{Bits17},
        Integer,
    },
    PackedStruct,
};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum EnergySaving {
    DbcDisable = 0b00,
    ConservativeMode = 0b01,
    NormalMode = 0b10,
    AgressiveMode = 0b11,
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian="msb")]
/// Set the threshold for each level of power saving
pub struct SetDbcTh {
    /// Threshold setting for the Conservative mode of DBC
    /// th1 = display width * display height * 3 * 0.1 /16
    #[packed_field(bits = "7..=23")]
    dbc_th1: Integer<u32, Bits17>,

    /// Threshold setting for the Normal mode of DBC
    /// th2 = display width * display height * 3 * 0.25 /16
    #[packed_field(bits = "31..=47")]
    dbc_th2: Integer<u32, Bits17>,

    /// Threshold setting for the Aggressive mode of DBC
    /// th3 = display width * display height * 3 * 0.6 /16
    #[packed_field(bits = "55..=71")]
    dbc_th3: Integer<u32, Bits17>,
}

impl WriteCommand for SetDbcTh {
    fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
    where
        I: WriteOnlyDataCommand,
    {
        ifc.send_commands(DataFormat::U8(&[0xD4]))?;
        ifc.send_data(DataFormat::U8(&self.pack().unwrap()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_dbc_th() {
        let mut th = SetDbcTh {
            dbc_th1: 0.into(),
            dbc_th2: 0.into(),
            dbc_th3: 0.into(),
        };
        assert_eq!(th.pack().unwrap(), [0, 0, 0, 0, 0, 0, 0, 0, 0]);
        th.dbc_th1 = 0b1_11111111_11111111.into();
        assert_eq!(th.pack().unwrap(), [1, 255, 255, 0, 0, 0, 0, 0, 0]);
        th.dbc_th2 = 0b1_11111111_11111111.into();
        assert_eq!(th.pack().unwrap(), [1, 255, 255, 1, 255, 255, 0, 0, 0]);
        th.dbc_th3 = 0b1_11111111_11111111.into();
        assert_eq!(th.pack().unwrap(), [1, 255, 255, 1, 255, 255, 1, 255, 255]);
    }
}
