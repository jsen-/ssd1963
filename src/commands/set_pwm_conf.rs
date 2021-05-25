use super::{ControlledBy};
use packed_struct::{
    derive::{PackedStruct, PrimitiveEnum_u8},
};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum Output {
    Zero = 0,
    One = 1,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum BrightnessPrescaler {
    Off = 0b0000,
    B1 = 0b0001,
    B2 = 0b0010,
    B3 = 0b0011,
    B4 = 0b0100,
    B6 = 0b0101,
    B8 = 0b0110,
    B12 = 0b0111,
    B16 = 0b1000,
    B24 = 0b1001,
    B32 = 0b1010,
    B48 = 0b1011,
    B64 = 0b1100,
    B96 = 0b1101,
    B128 = 0b1110,
    B192 = 0b1111,
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0")]
pub struct SetPwmConf {
    /// PWM frequency in system clock
    /// PWM signal frequency = PLL clock / (256 * (pwm_frequency + 1)) / 256
    pub pwm_frequency: u8,
    pub pwm_duty_cycle: u8,
    #[packed_field(bits = "20", ty = "enum")]
    pub pwm_control: ControlledBy,
    #[packed_field(bits = "23")]
    pub pwm_enable: bool,
    pub dbc_manual_brightness: u8,
    pub dbc_minimum_brightness: u8,
    #[packed_field(bits = "44..=47", ty = "enum")]
    /// Set the brightness prescaler to control how gradually the manual brightness is changed between different levels.
    /// A filter will undergo a number of iterations before the manual brightness is saturated.
    /// This parameter is valid when Transition Effect enable (0xD0) A5 = 1
    /// The iteration ratio = system frequency / brightness_prescaler / 32768
    pub brightness_prescaler: BrightnessPrescaler,
}

#[cfg(test)]
mod test {
    use super::*;
    use packed_struct::PackedStruct;

    #[test]
    fn set_post_proc() {
        let mut pc = SetPwmConf {
            pwm_frequency: 0,
            pwm_duty_cycle: 0,
            pwm_control: ControlledBy::Host,
            pwm_enable: false,
            dbc_manual_brightness: 0,
            dbc_minimum_brightness: 0,
            brightness_prescaler: BrightnessPrescaler::Off,
        };
        assert_eq!(pc.pack().unwrap(), [0, 0, 0, 0, 0, 0]);
        pc.pwm_frequency = 255;
        assert_eq!(pc.pack().unwrap(), [255, 0, 0, 0, 0, 0]);
        pc.pwm_duty_cycle = 255;
        assert_eq!(pc.pack().unwrap(), [255, 255, 0, 0, 0, 0]);
        pc.pwm_control = ControlledBy::Lcdc;
        assert_eq!(pc.pack().unwrap(), [255, 255, 0b1000, 0, 0, 0]);
        pc.pwm_enable = true;
        assert_eq!(pc.pack().unwrap(), [255, 255, 0b1001, 0, 0, 0]);
        pc.dbc_manual_brightness = 255;
        assert_eq!(pc.pack().unwrap(), [255, 255, 0b1001, 255, 0, 0]);
        pc.dbc_minimum_brightness = 255;
        assert_eq!(pc.pack().unwrap(), [255, 255, 0b1001, 255, 255, 0]);
        pc.brightness_prescaler = BrightnessPrescaler::B192;
        assert_eq!(pc.pack().unwrap(), [255, 255, 0b1001, 255, 255, 0b1111]);
    }
}
