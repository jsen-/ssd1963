#![cfg_attr(not(test), no_std)]
#![feature(min_type_alias_impl_trait)]

mod bit_twiddling;
mod commands;
mod gpio16bit_interface;

use commands::{CommandCode, CommandData};
pub use gpio16bit_interface::{GpioReadWrite16BitInterface, GpioWriteOnly16BitInterface};
use gpio16bit_interface::{ReadWriteInterface, WriteOnlyInterface, WritePort, Writer};

#[cfg(feature = "stm32f1xx")]
#[path = "features/stm32f1xx.rs"]
pub mod stm32f1xx;

fn send_command<'c, 'i: 'c, PortX, DC, WR, Error, Command, Delay, const N: usize>(
    writer: &'i mut Writer<PortX, DC, WR, Error>,
    delay: &mut Delay,
    command: &'c Command,
) -> Result<(), Error>
where
    PortX: WritePort,
    DC: embedded_hal::digital::v2::OutputPin<Error = Error>,
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
    Command: CommandCode + CommandData<N>,
    Delay: embedded_hal::blocking::delay::DelayUs<u8>,
{
    let mut w = writer.command()?;
    let mut comitter = w.set_value(Command::CODE.into())?;
    delay.delay_us(1);
    comitter.commit()?;
    let mut d = writer.data()?;
    for data in command.data() {
        let mut comitter = d.set_value(data.into())?;
        comitter.commit()?;
    }
    Ok(())
}

fn send_command_slow<'c, 'i: 'c, PortX, DC, WR, Error, Command, Delay, const N: usize>(
    writer: &'i mut Writer<PortX, DC, WR, Error>,
    delay: &mut Delay,
    command: &'c Command,
) -> Result<(), Error>
where
    PortX: WritePort,
    DC: embedded_hal::digital::v2::OutputPin<Error = Error>,
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
    Command: CommandCode + CommandData<N>,
    Delay: embedded_hal::blocking::delay::DelayUs<u8>,
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

pub struct Ssd1963<Lcd, Delay, Interface>
where
    Lcd: Screen,
    Delay: embedded_hal::blocking::delay::DelayUs<u8>,
    Interface: WriteOnlyInterface,
{
    interface: Interface,
    delay: Delay,
    #[allow(dead_code)]
    lcd: Lcd,
}

impl<Lcd, Delay, Interface> Ssd1963<Lcd, Delay, Interface>
where
    Lcd: Screen,
    Delay: embedded_hal::blocking::delay::DelayUs<u8>,
    Interface: WriteOnlyInterface,
{
    pub fn new(lcd: Lcd, mut interface: Interface, mut delay: Delay) -> Result<Self, Interface::Error> {
        let mut writer = interface.write()?;
        send_command_slow(
            &mut writer,
            &mut delay,
            &commands::SetPllMn {
                pll_multiplier: 0x1E,
                pll_divider: 0x02.into(),
                use_multiplier_and_divider: true,
            },
        )?;
        send_command_slow(
            &mut writer,
            &mut delay,
            &commands::SetPll {
                system_clock_source: commands::set_pll::SystemClockSource::ReferenceClock,
                enable_pll: true,
            },
        )?;
        // delay.delay_us(10);
        send_command_slow(
            &mut writer,
            &mut delay,
            &commands::SetPll {
                system_clock_source: commands::set_pll::SystemClockSource::PllOutput,
                enable_pll: true,
            },
        )?;
        // delay.delay_us(100);
        send_command_slow(&mut writer, &mut delay, &commands::SoftReset)?;
        delay.delay_us(100);
        send_command_slow(
            &mut writer,
            &mut delay,
            &commands::SetLShiftFreq {
                lcdc_fpr: 0b11_11111111_11111111.into(),
            },
        )?;
        send_command_slow(
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
        send_command_slow(
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
        send_command_slow(
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
        send_command(
            &mut writer,
            &mut delay,
            &commands::SetAddressMode {
                page_address_order: commands::set_address_mode::PageAddressOrder::TopToBottom,
                column_address_order: commands::set_address_mode::ColumnAddressOrder::LeftToRight,
                page_column_order: commands::set_address_mode::PageColumnOrder::Reverse,
                line_address_order: commands::set_address_mode::LineAddressOrder::LcdRefreshTopToBottom,
                color_order: commands::set_address_mode::ColorOrder::Rgb,
                data_latch_order: commands::set_address_mode::DataLatchOrder::LcdRefreshLeftToRight,
                flip_horizontal: true,
                flip_vertical: false,
            },
        )?;
        send_command(
            &mut writer,
            &mut delay,
            &commands::SetPixelDataInterface {
                pixel_data_interface_format: commands::set_pixel_data_interface::PixelDataInterfaceFormat::B16Format565,
            },
        )?;
        // interface.delay_us(1);
        // interface.write_command(0xB8u8)?;
        // interface.write_data(0x0fu8)?; //GPIO is controlled by host GPIO[3:0]=output   GPIO[0]=1  LCD ON  GPIO[0]=1  LCD OFF
        // interface.write_data(0x01u8)?; //GPIO0 normal
        // interface.write_command(0xBAu8)?;
        // interface.write_data(0x01u8)?; //GPIO[0] out 1 --- LCD display on/off control PIN
        send_command(&mut writer, &mut delay, &commands::SetDisplayOn)?;
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

    pub fn set_area(&mut self, x_start: u16, x_end: u16, y_start: u16, y_end: u16) -> Result<(), Interface::Error> {
        let mut writer = self.interface.write()?;
        send_command(&mut writer, &mut self.delay, &commands::SetColumnAddress { start: x_start, end: x_end })?;
        send_command(&mut writer, &mut self.delay, &commands::SetPageAddress { start: y_start, end: y_end })?;
        Ok(())
    }

    pub fn fill_area(&mut self, x_start: u16, x_end: u16, y_start: u16, y_end: u16, color: u16) -> Result<(), Interface::Error> {
        let mut writer = self.interface.write()?;
        send_command(&mut writer, &mut self.delay, &commands::SetColumnAddress { start: x_start, end: x_end })?;
        send_command(&mut writer, &mut self.delay, &commands::SetPageAddress { start: y_start, end: y_end })?;
        send_command(&mut writer, &mut self.delay, &commands::WriteMemoryStart)?;
        let mut data = writer.data()?;
        let mut comitter = data.set_value(color)?;
        for _ in 0..(x_end - x_start + 1) as usize * (y_end - y_start + 1) as usize {
            comitter.commit()?;
        }
        Ok(())
    }

    pub fn clear_screen(&mut self, color: u16) -> Result<(), Interface::Error> {
        self.fill_area(0, Lcd::WIDTH, 0, Lcd::HEIGHT, color)?;
        Ok(())
    }
}

impl<Lcd, Delay, Interface> Ssd1963<Lcd, Delay, Interface>
where
    Lcd: Screen,
    Delay: embedded_hal::blocking::delay::DelayUs<u8>,
    Interface: ReadWriteInterface,
{
    pub fn get_power_mode(&mut self) -> Result<u16, Interface::Error> {
        // TODO
        unimplemented!();
        // self.interface.read()?.data()?.get_value()
    }
}

pub trait Screen {
    const WIDTH: u16;
    const HEIGHT: u16;
}

pub struct Lcd800x480;
impl Screen for Lcd800x480 {
    const WIDTH: u16 = 800;
    const HEIGHT: u16 = 480;
}
