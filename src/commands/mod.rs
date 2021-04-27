pub mod set_address_mode;
pub mod set_gpio_conf;
pub mod set_gpio_value;
pub mod set_hori_period;
pub mod set_lcd_mode;
pub mod set_lcd_genx;
pub mod set_pll;
pub mod set_pll_mn;
pub mod set_post_proc;
pub mod set_pwm_conf;
pub mod set_vert_period;

use super::WriteCommand;
use core::convert::TryInto;
use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use packed_struct::derive::PrimitiveEnum_u8;

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum ControlledBy {
    Host = 0,
    Lcdc = 1,
}

macro_rules! simple_write {
    ($name:tt, $code:literal) => {
        pub struct $name {}
        impl WriteCommand for $name {
            fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
            where
                I: WriteOnlyDataCommand,
            {
                ifc.send_commands(DataFormat::U8(&[$code]))
            }
        }
    };
}

simple_write!(Nop, 0x00);
simple_write!(SoftReset, 0x01);
simple_write!(EnterSleepMode, 0x10);
simple_write!(ExitSleepMode, 0x11);
simple_write!(EnterPartialMode, 0x12);
simple_write!(EnterNormalMode, 0x12);
simple_write!(ExitInvertMode, 0x20);
simple_write!(EnterInvertMode, 0x21);
simple_write!(SetDisplayOff, 0x28);
simple_write!(SetDisplayOn, 0x29);
simple_write!(WriteMemoryStart, 0x2C);
simple_write!(ReadMemoryStart, 0x2E);
simple_write!(SetTearOff, 0x34);
simple_write!(ExistIdleMode, 0x38);
simple_write!(EnterIdleMode, 0x39);
simple_write!(WriteMemoryContinue, 0x3C);
simple_write!(ReadMemoryContinue, 0x3E);
simple_write!(SetDeepSleep, 0xE5);

fn as_u8(primitive_enum: isize) -> u8 {
    TryInto::<usize>::try_into(primitive_enum).unwrap().try_into().unwrap()
}

macro_rules! simpl_write_primitive_enum {
    ($name:tt, $code:literal) => {
        impl WriteCommand for $name {
            fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
            where
                I: WriteOnlyDataCommand,
            {
                ifc.send_commands(DataFormat::U8(&[$code]))?;
                ifc.send_data(DataFormat::U8(&[as_u8(*self as isize)]))
            }
        }
    };
}

#[derive(Clone, Copy)]
pub enum SetGammaCurve {
    Curve0 = 0b0001,
    Curve1 = 0b0010,
    Curve2 = 0b0100,
    Curve3 = 0b1000,
}
simpl_write_primitive_enum!(SetGammaCurve, 0x26);

#[derive(Clone, Copy)]
pub enum SetTearOn {
    /// The tearing effect output line consists of V-blanking information only
    V = 0,
    /// The tearing effect output line consists of both V-blanking and H-blanking information by set_tear_scanline(0x44)
    VH = 1,
}
simpl_write_primitive_enum!(SetTearOn, 0x35);

pub struct SetScrollStart(u16);
impl WriteCommand for SetScrollStart {
    fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
    where
        I: WriteOnlyDataCommand,
    {
        ifc.send_commands(DataFormat::U8(&[0x37]))?;
        ifc.send_data(DataFormat::U8(&self.0.to_be_bytes()))
    }
}

pub struct SetTearScanline(u16);
impl WriteCommand for SetTearScanline {
    fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
    where
        I: WriteOnlyDataCommand,
    {
        ifc.send_commands(DataFormat::U8(&[0x44]))?;
        ifc.send_data(DataFormat::U8(&self.0.to_be_bytes()))
    }
}
