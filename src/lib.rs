#![cfg_attr(not(test), no_std)]
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod bit_twiddling;
mod bounds;
mod commands;
pub mod display;
mod gpio16bit_interface;

pub use bounds::Bounds;

use commands::{CommandCode, CommandData};
use core::fmt::Debug;
use core::{cmp::min, convert::TryFrom, ops::RangeBounds};
use display::CopyArea;
pub use display::Display;
pub use gpio16bit_interface::{GpioReadWrite16BitInterface, GpioWriteOnly16BitInterface, WriteOnlyInterface};
use gpio16bit_interface::{ReadWriteInterface, ValueGetter, Writer};

#[cfg(feature = "stm32f1xx")]
#[path = "features/stm32f1xx.rs"]
pub mod stm32f1xx;

pub struct Ssd1963<Lcd, Delay, Interface>
where
    Lcd: Screen,
    Delay: embedded_hal::blocking::delay::DelayUs<u8>,
    Interface: WriteOnlyInterface,
{
    interface: Interface,
    pub delay: Delay,
    #[allow(dead_code)]
    lcd: Lcd,
}

impl<Lcd, Delay, Interface> Ssd1963<Lcd, Delay, Interface>
where
    Lcd: Screen,
    Delay: embedded_hal::blocking::delay::DelayUs<u8>,
    Interface: WriteOnlyInterface,
{
    fn send_command<'c, 'i: 'c, Command, const N: usize>(
        writer: &'i mut Writer<Interface::Port, Interface::DC, Interface::WR, Interface::Error>,
        delay: &mut Delay,
        command: &'c Command,
    ) -> Result<(), Interface::Error>
    where
        Command: CommandCode + CommandData<N>,
    {
        let mut w = writer.command()?;
        let mut comitter = w.set_value(Command::CODE.into())?;
        delay.delay_us(1);
        comitter.commit()?;
        let mut d = writer.data()?;
        for data in command.data() {
            let mut comitter = d.set_value(data.into())?;
            delay.delay_us(1);
            comitter.commit()?;
        }
        Ok(())
    }

    pub fn new(lcd: Lcd, mut interface: Interface, mut delay: Delay) -> Result<Self, Interface::Error> {
        let mut writer = interface.write()?;
        Self::send_command(
            &mut writer,
            &mut delay,
            &commands::SetPllMn {
                pll_multiplier: 0x1E,
                pll_divider: 0x02.into(),
                use_multiplier_and_divider: true,
            },
        )?;
        Self::send_command(
            &mut writer,
            &mut delay,
            &commands::SetPll {
                system_clock_source: commands::set_pll::SystemClockSource::ReferenceClock,
                enable_pll: true,
            },
        )?;
        Self::send_command(
            &mut writer,
            &mut delay,
            &commands::SetPll {
                system_clock_source: commands::set_pll::SystemClockSource::PllOutput,
                enable_pll: true,
            },
        )?;
        Self::send_command(&mut writer, &mut delay, &commands::SoftReset)?;
        delay.delay_us(1);
        Self::send_command(
            &mut writer,
            &mut delay,
            &commands::SetLShiftFreq {
                lcdc_fpr: 0b11_11111111_11111111.into(),
            },
        )?;
        Self::send_command(
            &mut writer,
            &mut delay,
            &commands::SetLcdMode {
                data_width: commands::set_lcd_mode::TftPanelDataWidth::B24,
                color_depth_enhancement_enable: false,
                frc_enable: false,
                lshift_polarity: commands::set_lcd_mode::Edge::Falling,
                lline_polarity: commands::set_lcd_mode::Active::ActiveLow,
                lframe_polarity: commands::set_lcd_mode::Active::ActiveLow,
                tft_type: commands::set_lcd_mode::TftType::TftMode0,
                hdp: (Lcd::WIDTH - 1).into(),
                vdp: (Lcd::HEIGHT - 1).into(),
                even_line_color_sequence: commands::set_lcd_mode::ColorSequence::Rgb,
                odd_line_color_sequence: commands::set_lcd_mode::ColorSequence::Rgb,
            },
        )?;
        Self::send_command(
            &mut writer,
            &mut delay,
            &commands::SetHoriPeriod {
                ht: 928.into(),
                hps: 46.into(),
                hpw: 48.into(),
                lps: 15.into(),
                lpspp: 0.into(),
            },
        )?;
        Self::send_command(
            &mut writer,
            &mut delay,
            &commands::SetVertPeriod {
                vt: 525.into(),
                vps: 16.into(),
                vpw: 16.into(),
                fps: 8.into(),
            },
        )?;
        // commands::set_gpio_value::SetGpioValue {
        //     gpio3_value: commands::set_gpio_value::Output::One,
        //     gpio2_value: commands::set_gpio_value::Output::One,
        //     gpio1_value: commands::set_gpio_value::Output::One,
        //     gpio0_value: commands::set_gpio_value::Output::One,
        // }.send(&mut interface)?;
        // commands::set_gpio_conf::SetGpioConf {
        //     gpio3_conf: commands::ControlledBy::Host,
        //     gpio2_conf: commands::ControlledBy::Host,
        //     gpio1_conf: commands::ControlledBy::Host,
        //     gpio0_conf: commands::ControlledBy::Host,
        //     gpio3_dir: commands::set_gpio_conf::Direction::Input,
        //     gpio2_dir: commands::set_gpio_conf::Direction::Output,
        //     gpio1_dir: commands::set_gpio_conf::Direction::Output,
        //     gpio0_dir: commands::set_gpio_conf::Direction::Output,
        //     gpio0_power_control: commands::set_gpio_conf::PowerControl::Normal,
        // }
        Self::send_command(
            &mut writer,
            &mut delay,
            &commands::SetAddressMode {
                page_address_order: commands::set_address_mode::PageAddressOrder::TopToBottom,
                column_address_order: commands::set_address_mode::ColumnAddressOrder::LeftToRight,
                page_column_order: commands::set_address_mode::PageColumnOrder::Normal,
                line_address_order: commands::set_address_mode::LineAddressOrder::LcdRefreshTopToBottom,
                color_order: commands::set_address_mode::ColorOrder::Rgb,
                data_latch_order: commands::set_address_mode::DataLatchOrder::LcdRefreshLeftToRight,
                flip_horizontal: false,
                flip_vertical: false,
            },
        )?;
        Self::send_command(
            &mut writer,
            &mut delay,
            &commands::SetPixelDataInterface {
                pixel_data_interface_format: commands::set_pixel_data_interface::PixelDataInterfaceFormat::B16Format565,
            },
        )?;
        // interface.write_command(0xB8u8)?;
        // interface.write_data(0x0fu8)?; //GPIO is controlled by host GPIO[3:0]=output   GPIO[0]=1  LCD ON  GPIO[0]=1  LCD OFF
        // interface.write_data(0x01u8)?; //GPIO0 normal
        // interface.write_command(0xBAu8)?;
        // interface.write_data(0x01u8)?; //GPIO[0] out 1 --- LCD display on/off control PIN
        Self::send_command(&mut writer, &mut delay, &commands::SetDisplayOn)?;
        // commands::set_pwm_conf::SetPwmConf {
        //     pwm_frequency: 6,
        //     pwm_duty_cycle: 0xf0,
        //     pwm_control: commands::ControlledBy::Host,
        //     pwm_enable: true,
        //     dbc_manual_brightness: 0xf0,
        //     dbc_minimum_brightness: 0,
        //     brightness_prescaler: commands::set_pwm_conf::BrightnessPrescaler::Off,
        // }
        // .send(&mut interface)?;
        // commands::set_dbc_conf::SetDbcConf {
        //     manual_brightness_enable: false,
        //     transition_effect_enable: false,
        //     energy_saving: commands::set_dbc_conf::EnergySaving::AgressiveMode,
        //     master_enable: true,
        // }
        // .send(&mut interface)?;

        Ok(Self { interface, lcd, delay })
    }

    pub fn release(self) -> (Interface, Delay) {
        (self.interface, self.delay)
    }
    pub fn width(&self) -> u16 {
        Lcd::WIDTH
    }
    pub fn height(&self) -> u16 {
        Lcd::HEIGHT
    }
    fn display_size(&self) -> Bounds {
        let width = Lcd::WIDTH;
        let height = Lcd::HEIGHT;
        Bounds {
            x_start: 0,
            x_end: width - 1,
            y_start: 0,
            y_end: height - 1,
        }
    }

    fn set_area_bounds(&mut self, bounds: &Bounds) -> Result<(), Interface::Error> {
        let &Bounds {
            x_start,
            x_end,
            y_start,
            y_end,
        } = bounds;
        let mut writer = self.interface.write()?;
        Self::send_command(&mut writer, &mut self.delay, &commands::SetColumnAddress { start: x_start, end: x_end })?;
        Self::send_command(&mut writer, &mut self.delay, &commands::SetPageAddress { start: y_start, end: y_end })?;
        Ok(())
    }

    fn set_area<'a, X, Y>(&mut self, x: X, y: Y) -> Result<Bounds, Interface::Error>
    where
        X: RangeBounds<u16>,
        Y: RangeBounds<u16>,
    {
        let bounds = Bounds::new_within(x, y, &self.display_size()).unwrap();
        self.set_area_bounds(&bounds)?;
        Ok(bounds)
    }

    fn fill_area_bounds(&mut self, bounds: &Bounds, it: &mut dyn Iterator<Item = u16>) -> Result<(), Interface::Error> {
        self.set_area_bounds(bounds)?;
        let mut writer = self.interface.write()?;
        Self::send_command(&mut writer, &mut self.delay, &commands::WriteMemoryStart)?;
        let mut data = writer.data()?;
        for color in it.limit(bounds.area()) {
            let mut c = data.set_value(color)?;
            c.commit()?;
        }
        Ok(())
    }

    pub fn fill_area_color<X, Y>(&mut self, x: X, y: Y, color: u16) -> Result<(), Interface::Error>
    where
        X: RangeBounds<u16>,
        Y: RangeBounds<u16>,
    {
        let bounds = self.set_area(x, y)?;
        let mut writer = self.interface.write()?;
        Self::send_command(&mut writer, &mut self.delay, &commands::WriteMemoryStart)?;
        let mut data = writer.data()?;
        let mut comitter = data.set_value(color)?;
        for _ in 0..bounds.area() {
            comitter.commit()?;
        }
        Ok(())
    }

    pub fn clear_screen(&mut self, color: u16) -> Result<(), Interface::Error> {
        self.fill_area_color(.., .., color)?;
        Ok(())
    }
}

pub struct DisplayDataIter<'r, Interface>
where
    Interface: ReadWriteInterface,
{
    getter: ValueGetter<'r, <Interface as ReadWriteInterface>::Port, Interface::RD, Interface::Error>,
    count: u32,
}

impl<'r, Interface> Iterator for DisplayDataIter<'r, Interface>
where
    Interface: ReadWriteInterface,
{
    type Item = Result<u16, Interface::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        Some(self.getter.get_value())
    }
}

impl<Lcd, Delay, Interface> Ssd1963<Lcd, Delay, Interface>
where
    Lcd: Screen,
    Delay: embedded_hal::blocking::delay::DelayUs<u8>,
    Interface: ReadWriteInterface,
{
    fn read_area_bounds(&mut self, bounds: &Bounds) -> Result<DisplayDataIter<Interface>, Interface::Error> {
        self.set_area_bounds(&bounds)?;
        Self::send_command(&mut self.interface.write()?, &mut self.delay, &commands::ReadMemoryStart)?;

        let it = DisplayDataIter {
            getter: self.interface.read()?.into_data()?,
            count: bounds.area(),
        };
        Ok(it)
    }

    fn copy_area_bounds(&mut self, from: &Bounds, horiz_by: i16, vert_by: i16, buffer: &mut [u16]) -> Result<(), Interface::Error> {
        fn copy<Lcd, Delay, Interface>(
            me: &mut Ssd1963<Lcd, Delay, Interface>,
            from: &Bounds,
            to: &Bounds,
            buffer: &mut [u16],
        ) -> Result<(), Interface::Error>
        where
            Lcd: Screen,
            Delay: embedded_hal::blocking::delay::DelayUs<u8>,
            Interface: ReadWriteInterface,
        {
            let it = me.read_area_bounds(from)?;
            if let Some(err) = buffer.iter_mut().zip(it).find_map(|(dest, item)| match item {
                Ok(color) => {
                    *dest = color;
                    None
                }
                Err(err) => Some(err),
            }) {
                return Err(err);
            }
            me.fill_area_bounds(to, &mut buffer.iter().copied())?;
            Ok(())
        }

        let mut source = *from;
        let mut target = source;
        target.move_by(horiz_by, vert_by);
        if !target.is_within(self.display_size()) {
            panic!("OutOfBounds")
        }
        let buffer_lines = u16::try_from(min(usize::from(source.height()), buffer.len() / usize::from(source.width()))).unwrap();
        if buffer_lines == 0 {
            panic!("Buffer too small")
        }
        let buffer = &mut buffer[0..usize::from(buffer_lines) * usize::from(source.width())];
        let remainder = source.height() % u16::try_from(buffer_lines).unwrap();

        source.set_height(buffer_lines);
        target.set_height(buffer_lines);

        let mut buffer_lines = i16::try_from(buffer_lines).unwrap();
        // if target is below source, we need to start from bottom
        let break_offset = if vert_by > 0 {
            source.move_by(0u16, from.height() - source.height());
            target.move_by(0u16, from.height() - source.height());
            buffer_lines = -buffer_lines;
            from.y_start + remainder
        } else {
            from.y_end - remainder - source.height() + 1
        };

        loop {
            copy(self, &source, &target, buffer)?;
            if source.y_start == break_offset {
                break;
            }
            source.move_by(0u16, buffer_lines);
            target.move_by(0u16, buffer_lines);
        }
        if remainder > 0 {
            source.y_start = u16::try_from(i16::try_from(source.y_start).unwrap() + buffer_lines).unwrap();
            target.y_start = u16::try_from(i16::try_from(target.y_start).unwrap() + buffer_lines).unwrap();
            copy(self, source.set_height(remainder), target.set_height(remainder), buffer)?;
        }
        Ok(())
    }
}

impl<Lcd, Delay, Interface> Display for Ssd1963<Lcd, Delay, Interface>
where
    Lcd: Screen,
    Delay: embedded_hal::blocking::delay::DelayUs<u8>,
    Interface: ReadWriteInterface,
{
    const WIDTH: u16 = Lcd::WIDTH;
    const HEIGHT: u16 = Lcd::HEIGHT;

    type Color = u16;
    type Error = Interface::Error;

    fn fill_area<X, Y>(&mut self, x: X, y: Y, it: &mut dyn Iterator<Item = u16>) -> Result<(), Self::Error>
    where
        X: RangeBounds<u16>,
        Y: RangeBounds<u16>,
    {
        self.fill_area_bounds(&Bounds::new_within(x, y, &self.display_size()).unwrap(), it)
    }
}

impl<Lcd, Delay, Interface> CopyArea for Ssd1963<Lcd, Delay, Interface>
where
    Lcd: Screen,
    Delay: embedded_hal::blocking::delay::DelayUs<u8>,
    Interface: ReadWriteInterface,
{
    fn copy_area<X, Y>(&mut self, x: X, y: Y, horiz_by: i16, vert_by: i16, buffer: &mut [u16]) -> Result<(), Self::Error>
    where
        X: RangeBounds<u16>,
        Y: RangeBounds<u16>,
    {
        self.copy_area_bounds(&Bounds::new_within(x, y, &self.display_size()).unwrap(), horiz_by, vert_by, buffer)
    }
}

pub trait Screen {
    const WIDTH: u16;
    const HEIGHT: u16;
}

pub struct Lcd800x480;
impl Screen for Lcd800x480 {
    const WIDTH: u16 = 800;
    const HEIGHT: u16 = 481;
}

struct Limit<I, N> {
    it: I,
    remaining: N,
}

impl<I, N> Iterator for Limit<I, N>
where
    N: core::cmp::PartialEq<N> + core::ops::SubAssign<N> + TryFrom<u8>,
    <N as TryFrom<u8>>::Error: Debug,
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == N::try_from(0u8).unwrap() {
            return None;
        }
        self.remaining -= N::try_from(1u8).unwrap();
        self.it.next()
    }
}
trait Limited
where
    Self: Sized,
{
    fn limit<N>(self, n: N) -> Limit<Self, N>
    where
        N: core::cmp::PartialEq<N> + core::ops::SubAssign<N> + TryFrom<u8>,
        <N as TryFrom<u8>>::Error: Debug;
}

impl<I> Limited for I
where
    I: Iterator,
{
    fn limit<N>(self, n: N) -> Limit<Self, N>
    where
        N: core::cmp::PartialEq<N> + core::ops::SubAssign<N> + TryFrom<u8>,
        <N as TryFrom<u8>>::Error: Debug,
    {
        Limit { it: self, remaining: n }
    }
}

// impl<Lcd, Delay, Interface> ReadArea for Ssd1963<Lcd, Delay, Interface>
// where
//     Lcd: Screen,
//     Delay: embedded_hal::blocking::delay::DelayUs<u8>,
//     Interface: ReadWriteInterface,
// {
//     type Iter<'r> = DisplayDataIter<'r, Interface>;

//     fn read_area<'r, X, Y>(&'r mut self, x: X, y: Y) -> Result<Self::Iter<'r>, Self::Error>
//     where
//         X: RangeBounds<u16>,
//         Y: RangeBounds<u16>,
//     {
//         self.read_area_bounds(&Bounds::new_within(x, y, &self.display_size()).unwrap())
//     }
// }
