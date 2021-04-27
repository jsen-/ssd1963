use super::{DataFormat, DisplayError, WriteCommand, WriteOnlyDataCommand};
use packed_struct::{
    derive::{PackedStruct, PrimitiveEnum_u8},
    PackedStruct,
};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum SystemClockSource {
    /// Use reference clock as system clock
    ReferenceClock = 0,
    /// Use PLL output as system clock
    PllOutput = 1,
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0")]
pub struct SetPll {
    #[packed_field(bits = "6", ty = "enum")]
    pub system_clock_source: SystemClockSource,
    #[packed_field(bits = "7")]
    pub enable_pll: bool,
}

impl WriteCommand for SetPll {
    fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
    where
        I: WriteOnlyDataCommand,
    {
        ifc.send_commands(DataFormat::U8(&[0xE0]))?;
        ifc.send_data(DataFormat::U8(&self.pack().unwrap()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_pll() {
        let mut pc = SetPll {
            system_clock_source: SystemClockSource::ReferenceClock,
            enable_pll: false,
        };
        assert_eq!(pc.pack().unwrap(), [0]);
        pc.system_clock_source = SystemClockSource::PllOutput;
        assert_eq!(pc.pack().unwrap(), [0b10]);
        pc.enable_pll = true;
        assert_eq!(pc.pack().unwrap(), [0b11]);
    }
}
