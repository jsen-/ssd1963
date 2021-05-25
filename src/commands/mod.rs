pub trait CommandCode {
    const CODE: u8;
}
pub trait CommandData<const N: usize> {
    fn data(&self) -> [u8; N];
}


macro_rules! impl_command_int {
    ($path:ident :: $name:ident, $code:literal) => {
        pub use $path::$name;
        impl CommandCode for $path::$name {
            const CODE: u8 = $code;
        }
        impl CommandData<{ core::mem::size_of::<<$path::$name as packed_struct::PackedStruct>::ByteArray>() }> for $path::$name {
            fn data(&self) -> [u8; { core::mem::size_of::<<$path::$name as packed_struct::PackedStruct>::ByteArray>() }] {
                <$path::$name as packed_struct::PackedStruct>::pack(self).unwrap()
            }
        }
    }
}

macro_rules! impl_command {
    ($path:ident :: $name:ident, $code:literal) => {
        mod $path;
        impl_command_int!($path::$name, $code);
    };
    (pub $path:ident :: $name:ident, $code:literal) => {
        pub mod $path;
        impl_command_int!($path::$name, $code);
    };
}

impl_command!(pub set_pll::SetPll, 0xE0);
impl_command!(set_pll_mn::SetPllMn, 0xE2);
impl_command!(set_lshift_freq::SetLShiftFreq, 0xE6);
impl_command!(pub set_lcd_mode::SetLcdMode, 0xB0);
impl_command!(set_hori_period::SetHoriPeriod, 0xB4);
impl_command!(set_vert_period::SetVertPeriod, 0xB6);
impl_command!(set_gpio_value::SetGpioValue, 0xBA);
impl_command!(set_gpio_conf::SetGpioConf, 0xB8);
impl_command!(pub set_address_mode::SetAddressMode, 0x36);
impl_command!(pub set_pixel_data_interface::SetPixelDataInterface, 0xF0);
impl_command!(set_pwm_conf::SetPwmConf, 0xBE);
impl_command!(set_dbc_conf::SetDbcConf, 0xD0);
impl_command!(set_column_address::SetColumnAddress, 0x2A);
impl_command!(set_page_address::SetPageAddress, 0x2B);

use packed_struct::derive::PrimitiveEnum_u8;
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum ControlledBy {
    Host = 0,
    Lcdc = 1,
}

macro_rules! simple_write {
    ($name:ident, $code:literal) => {
        pub struct $name;
        impl CommandCode for $name {
            const CODE: u8 = $code;
        }
        impl CommandData<0> for $name {
            fn data(&self) -> [u8; 0] {
                [0; 0]
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

// fn as_u8(primitive_enum: isize) -> u8 {
//     use core::convert::TryInto;
//     TryInto::<usize>::try_into(primitive_enum).unwrap().try_into().unwrap()
// }

// macro_rules! simpl_write_primitive_enum {
//     ($name:tt, $code:literal) => {
//         impl crate::Command for $name {
//             fn send<I>(&self, ifc: &mut I) -> Result<(), I::Error>
//             where
//                 I: crate::WriteOnlyInterface,
//             {
//                 let code: u8 = $code;
//                 ifc.write_command(code)?;
//                 ifc.write_data(&[as_u8(*self as isize)])
//             }
//         }
//     };
// }

// #[derive(Clone, Copy)]
// pub enum SetGammaCurve {
//     Curve0 = 0b0001,
//     Curve1 = 0b0010,
//     Curve2 = 0b0100,
//     Curve3 = 0b1000,
// }
// simpl_write_primitive_enum!(SetGammaCurve, 0x26);

// #[derive(Clone, Copy)]
// pub enum SetTearOn {
//     /// The tearing effect output line consists of V-blanking information only
//     V = 0,
//     /// The tearing effect output line consists of both V-blanking and H-blanking information by set_tear_scanline(0x44)
//     VH = 1,
// }
// simpl_write_primitive_enum!(SetTearOn, 0x35);

// pub struct SetScrollStart(u16);
// impl WriteCommand for SetScrollStart {
//     fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
//     where
//         I: WriteOnlyDataCommand,
//     {
//         ifc.send_commands(DataFormat::U8(&[0x37]))?;
//         ifc.send_data(DataFormat::U8(&self.0.to_be_bytes()))
//     }
// }

// pub struct SetTearScanline(u16);
// impl WriteCommand for SetTearScanline {
//     fn send<I>(&self, mut ifc: I) -> Result<(), DisplayError>
//     where
//         I: WriteOnlyDataCommand,
//     {
//         ifc.send_commands(DataFormat::U8(&[0x44]))?;
//         ifc.send_data(DataFormat::U8(&self.0.to_be_bytes()))
//     }
// }
