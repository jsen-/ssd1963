use super::{ControlledBy, DataFormat, DisplayError, WriteCommand, WriteOnlyDataCommand};
use packed_struct::{
    derive::{PackedStruct, PrimitiveEnum_u8},
    PackedStruct,
};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Input = 0,
    Output = 1,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum PowerControl {
    /// GPIO0 is used to control the panel power with  enter_sleep_mode (0x10) or exit_sleep_mode (0x11)
    Power = 0,
    /// GPIO0 is used as normal GPIO
    Normal = 1,
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0")]
pub struct SetGpioConf {
    #[packed_field(bits = "0", ty = "enum")]
    pub gpio3_conf: ControlledBy,
    #[packed_field(bits = "1", ty = "enum")]
    pub gpio2_conf: ControlledBy,
    #[packed_field(bits = "2", ty = "enum")]
    pub gpio1_conf: ControlledBy,
    #[packed_field(bits = "3", ty = "enum")]
    pub gpio0_conf: ControlledBy,
    #[packed_field(bits = "4", ty = "enum")]
    pub gpio3_dir: Direction,
    #[packed_field(bits = "5", ty = "enum")]
    pub gpio2_dir: Direction,
    #[packed_field(bits = "6", ty = "enum")]
    pub gpio1_dir: Direction,
    #[packed_field(bits = "7", ty = "enum")]
    pub gpio0_dir: Direction,
    #[packed_field(bits = "15", ty = "enum")]
    pub gpio0_power_control: PowerControl,
}

impl WriteCommand for SetGpioConf {
    fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
    where
        I: WriteOnlyDataCommand,
    {
        ifc.send_commands(DataFormat::U8(&[0xB8]))?;
        ifc.send_data(DataFormat::U8(&self.pack().unwrap()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_gpio_conf() {
        let mut gc = SetGpioConf {
            gpio3_conf: ControlledBy::Host,
            gpio2_conf: ControlledBy::Host,
            gpio1_conf: ControlledBy::Host,
            gpio0_conf: ControlledBy::Host,
            gpio3_dir: Direction::Input,
            gpio2_dir: Direction::Input,
            gpio1_dir: Direction::Input,
            gpio0_dir: Direction::Input,
            gpio0_power_control: PowerControl::Power,
        };
        assert_eq!(gc.pack().unwrap(), [0, 0]);
        gc.gpio3_conf = ControlledBy::Lcdc;
        assert_eq!(gc.pack().unwrap(), [0b10000000, 0]);
        gc.gpio2_conf = ControlledBy::Lcdc;
        assert_eq!(gc.pack().unwrap(), [0b11000000, 0]);
        gc.gpio1_conf = ControlledBy::Lcdc;
        assert_eq!(gc.pack().unwrap(), [0b11100000, 0]);
        gc.gpio0_conf = ControlledBy::Lcdc;
        assert_eq!(gc.pack().unwrap(), [0b11110000, 0]);
        gc.gpio3_dir = Direction::Output;
        assert_eq!(gc.pack().unwrap(), [0b11111000, 0]);
        gc.gpio2_dir = Direction::Output;
        assert_eq!(gc.pack().unwrap(), [0b11111100, 0]);
        gc.gpio1_dir = Direction::Output;
        assert_eq!(gc.pack().unwrap(), [0b11111110, 0]);
        gc.gpio0_dir = Direction::Output;
        assert_eq!(gc.pack().unwrap(), [0b11111111, 0]);
        gc.gpio0_power_control = PowerControl::Normal;
        assert_eq!(gc.pack().unwrap(), [0b11111111, 0b1]);
    }
}
