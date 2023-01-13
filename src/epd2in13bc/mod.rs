//! A simple Driver for the Waveshare 2.13" (B/C) E-Ink Display via SPI
//! More information on this display can be found at the [Waveshare Wiki](https://www.waveshare.com/wiki/2.13inch_e-Paper_HAT_(B))
//! This driver was build and tested for 212x104, 2.13inch E-Ink display HAT for Raspberry Pi, three-color, SPI interface
//!
//! # Example for the 2.13" E-Ink Display
//!
//!```rust, no_run
//!# use embedded_hal_mock::*;
//!# fn main() -> Result<(), MockError> {
//!use embedded_graphics::{prelude::*, primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder}};
//!use epd_waveshare::{epd2in13bc::*, prelude::*};
//!#
//!# let expectations = [];
//!# let mut spi = spi::Mock::new(&expectations);
//!# let expectations = [];
//!# let cs_pin = pin::Mock::new(&expectations);
//!# let busy_in = pin::Mock::new(&expectations);
//!# let dc = pin::Mock::new(&expectations);
//!# let rst = pin::Mock::new(&expectations);
//!# let mut delay = delay::MockNoop::new();
//!
//!// Setup EPD
//!let mut epd = Epd2in13bc::new(&mut spi, cs_pin, busy_in, dc, rst, &mut delay, None)?;
//!
//!// Use display graphics from embedded-graphics
//!// This display is for the black/white/chromatic pixels
//!let mut tricolor_display = Display2in13bc::default();
//!
//!// Use embedded graphics for drawing a black line
//!let _ = Line::new(Point::new(0, 120), Point::new(0, 200))
//!    .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
//!    .draw(&mut tricolor_display);
//!
//!// We use `chromatic` but it will be shown as red/yellow
//!let _ = Line::new(Point::new(15, 120), Point::new(15, 200))
//!    .into_styled(PrimitiveStyle::with_stroke(TriColor::Chromatic, 1))
//!    .draw(&mut tricolor_display);
//!
//!// Display updated frame
//!epd.update_color_frame(
//!    &mut spi,
//!    &mut delay,
//!    &tricolor_display.bw_buffer(),
//!    &tricolor_display.chromatic_buffer()
//!)?;
//!epd.display_frame(&mut spi, &mut delay)?;
//!
//!// Set the EPD to sleep
//!epd.sleep(&mut spi, &mut delay)?;
//!# Ok(())
//!# }
//!```
use embedded_hal::{
    blocking::{delay::*, spi::Write},
    digital::v2::*,
};

use crate::interface::DisplayInterface;
use crate::traits::{
    InternalWiAdditions, RefreshLut, WaveshareDisplay, WaveshareThreeColorDisplay,
};

/// Width of epd2in13bc in pixels
pub const WIDTH: u32 = 104;
/// Height of epd2in13bc in pixels
pub const HEIGHT: u32 = 212;
/// Default background color (white) of epd2in13bc display
pub const DEFAULT_BACKGROUND_COLOR: TriColor = TriColor::White;

/// Number of bits for b/w buffer and same for chromatic buffer
const NUM_DISPLAY_BITS: u32 = WIDTH * HEIGHT / 8;

const IS_BUSY_LOW: bool = true;
const VCOM_DATA_INTERVAL: u8 = 0x07;
const WHITE_BORDER: u8 = 0x70;
const BLACK_BORDER: u8 = 0x30;
const CHROMATIC_BORDER: u8 = 0xb0;
const FLOATING_BORDER: u8 = 0xF0;

use crate::color::TriColor;

pub(crate) mod command;
use self::command::Command;
use crate::buffer_len;

/// Full size buffer for use with the 2.13" b/c EPD
#[cfg(feature = "graphics")]
pub type Display2in13bc = crate::graphics::Display<
    WIDTH,
    HEIGHT,
    true,
    { buffer_len(WIDTH as usize, HEIGHT as usize * 2) },
    TriColor,
>;

/// Epd2in13bc driver
pub struct Epd2in13bc<SPI, CS, BUSY, DC, RST, DELAY> {
    interface: DisplayInterface<SPI, CS, BUSY, DC, RST, DELAY>,
    color: TriColor,
}

impl<SPI, CS, BUSY, DC, RST, DELAY> InternalWiAdditions<SPI, CS, BUSY, DC, RST, DELAY>
    for Epd2in13bc<SPI, CS, BUSY, DC, RST, DELAY>
where
    SPI: Write<u8>,
    CS: OutputPin,
    BUSY: InputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayUs<u32>,
{
    fn init(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
        // Values taken from datasheet and sample code

        self.interface.reset(delay, 10_000, 10_000);

        // start the booster
        // self.interface
        //     .cmd_with_data(spi, Command::BoosterSoftStart, &[0x17, 0x17, 0x17])?;

        //WIP
        // power on
        // self.command(spi, Command::Init)?;
        self.interface
        .wait_until_idle_with_cmd(spi, delay, false,  Command::Init)?;
        // // Set RAM Data Entry Mode
        self.interface
        .wait_until_idle_with_cmd(spi, delay, false,  Command::Init2)?;
        // self.cmd_with_data(spi, Command::DataStartTransmission2, &[0x03])?;
        self.interface.data(spi, &[0x03]);

        let x_pixels_par:u8 = 212 - 1;
        let y_pixels_par:u8 = 104 - 1;
        let x_start:u8 = 0;
        let x_end = x_pixels_par / 8;
        let y_start:u8 = 0x00;
        let y_start1:u8 = 0x00;
        let y_end = y_pixels_par;// % 256;
        let y_end1 = y_pixels_par;// / 256;

        // // Set RAM area
    
        self.interface
        .wait_until_idle_with_cmd(spi, delay, false,  Command::SetRamArea)?;
        self.interface.data(spi,  &[0x01, 0x10])?;

        self.interface
        .wait_until_idle_with_cmd(spi, delay, false,  Command::SetRamArea2)?;
        self.interface.data(spi,  &[0x00, 0x00, 0xF9, 0x00])?;

        // self.cmd_with_data(spi, Command::SetRamArea2, &[y_start, y_start1, y_end, y_end1])?;

        // // Set RAM pointer
        self.interface
        .wait_until_idle_with_cmd(spi, delay, false,  Command::SetRamPointer)?;
        self.interface.data(spi,  &[0x01])?;

        self.interface
        .wait_until_idle_with_cmd(spi, delay, false,  Command::SetRamPointer2)?;
        self.interface.data(spi,  &[0x00, 0x00])?;
        // self.cmd_with_data(spi, Command::SetRamPointer, &[0x01])?;
        // self.cmd_with_data(spi, Command::SetRamPointer2, &[0x00, 0x00])?;


        // // Set LUT
        let lut_data:[u8;70]= [
            0x80, 0x60, 0x40, 0x00, 0x00, 0x00, 0x00, //LUT0: BB:     VS 0 ~7
            0x10, 0x60, 0x20, 0x00, 0x00, 0x00, 0x00, //LUT1: BW:     VS 0 ~7
            0x80, 0x60, 0x40, 0x00, 0x00, 0x00, 0x00, //LUT2: WB:     VS 0 ~7
            0x10, 0x60, 0x20, 0x00, 0x00, 0x00, 0x00, //LUT3: WW:     VS 0 ~7
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //LUT4: VCOM:   VS 0 ~7
            0x03, 0x03, 0x00, 0x00, 0x02, // TP0 A~D RP0
            0x09, 0x09, 0x00, 0x00, 0x02, // TP1 A~D RP1
            0x03, 0x03, 0x00, 0x00, 0x02, // TP2 A~D RP2
            0x00, 0x00, 0x00, 0x00, 0x00, // TP3 A~D RP3
            0x00, 0x00, 0x00, 0x00, 0x00, // TP4 A~D RP4
            0x00, 0x00, 0x00, 0x00, 0x00, // TP5 A~D RP5
            0x00, 0x00, 0x00, 0x00, 0x00, // TP6 A~D RP6
        ];
        // self.interface
        // .wait_until_idle_with_cmd(spi, delay, false,  Command::LutData)?;
        // self.interface.data(spi, &lut_data)?;
        // self.cmd_with_data(spi, Command::LutData, &lut_data)?;

        // // PowerOn
        // self.interface
        // .wait_until_idle_with_cmd(spi, delay, false,  Command::PowerOn)?;
        // self.interface.data(spi, &[0xc0, 0x20])?;
        // self.cmd_with_data(spi, Command::PowerOn, &[0xc0, 0x20])?;
    

        // Update
        let mut write:[u8; 4000] = [0x00;4000];
        write[0] = 0x00;
        write[10] = 0x00;
        write[20] = 0x00;
        self.interface
        .wait_until_idle_with_cmd(spi, delay, false,  Command::DisplayRefresh)?;

        self.interface.wait_until_idle(delay, false);
        self.interface.data(spi, &write)?;

        // self.interface.wait_until_idle(delay, false);
        // self.interface.data(spi, &write)?;


        // self.interface.wait_until_idle(delay, false);
        // self.interface.data(spi, &write)?;

        // let mut write:[u8; 928] = [0xFF;928];
        // self.interface.wait_until_idle(delay, false);
        // self.interface.data(spi, &write)?;

        // delay.delay_us(5000);
        // self.wait_until_idle(spi, delay)?;

        self.interface
        .wait_until_idle_with_cmd(spi, delay, false,  Command::LutForVcom)?;
        self.interface
        .wait_until_idle_with_cmd(spi, delay, false,  Command::DataStartTransmission1)?;
        self.interface.data(spi,  &[0x01])?;
        // set the panel settings
        // self.cmd_with_data(spi, Command::PanelSetting, &[0x8F])?;

        // self.cmd_with_data(
        //     spi,
        //     Command::VcomAndDataIntervalSetting,
        //     &[WHITE_BORDER | VCOM_DATA_INTERVAL],
        // )?;

        // // set resolution
        // self.send_resolution(spi)?;

        // self.cmd_with_data(spi, Command::VcmDcSetting, &[0x0A])?;

        // self.wait_until_idle(spi, delay)?;

        Ok(())
    }
}

impl<SPI, CS, BUSY, DC, RST, DELAY> WaveshareThreeColorDisplay<SPI, CS, BUSY, DC, RST, DELAY>
    for Epd2in13bc<SPI, CS, BUSY, DC, RST, DELAY>
where
    SPI: Write<u8>,
    CS: OutputPin,
    BUSY: InputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayUs<u32>,
{
    fn update_color_frame(
        &mut self,
        spi: &mut SPI,
        delay: &mut DELAY,
        black: &[u8],
        chromatic: &[u8],
    ) -> Result<(), SPI::Error> {
        self.update_achromatic_frame(spi, delay, black)?;
        self.update_chromatic_frame(spi, delay, chromatic)
    }

    /// Update only the black/white data of the display.
    ///
    /// Finish by calling `update_chromatic_frame`.
    fn update_achromatic_frame(
        &mut self,
        spi: &mut SPI,
        _delay: &mut DELAY,
        black: &[u8],
    ) -> Result<(), SPI::Error> {
        self.interface.cmd(spi, Command::DataStartTransmission1)?;
        self.interface.data(spi, black)?;
        Ok(())
    }

    /// Update only chromatic data of the display.
    ///
    /// This data takes precedence over the black/white data.
    fn update_chromatic_frame(
        &mut self,
        spi: &mut SPI,
        delay: &mut DELAY,
        chromatic: &[u8],
    ) -> Result<(), SPI::Error> {
        self.interface.cmd(spi, Command::DataStartTransmission2)?;
        self.interface.data(spi, chromatic)?;

        self.wait_until_idle(spi, delay)?;
        Ok(())
    }
}

impl<SPI, CS, BUSY, DC, RST, DELAY> WaveshareDisplay<SPI, CS, BUSY, DC, RST, DELAY>
    for Epd2in13bc<SPI, CS, BUSY, DC, RST, DELAY>
where
    SPI: Write<u8>,
    CS: OutputPin,
    BUSY: InputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayUs<u32>,
{
    type DisplayColor = TriColor;
    fn new(
        spi: &mut SPI,
        cs: CS,
        busy: BUSY,
        dc: DC,
        rst: RST,
        delay: &mut DELAY,
        delay_us: Option<u32>,
    ) -> Result<Self, SPI::Error> {
        let interface = DisplayInterface::new(cs, busy, dc, rst, delay_us);
        let color = DEFAULT_BACKGROUND_COLOR;

        let mut epd = Epd2in13bc { interface, color };

        epd.init(spi, delay)?;

        Ok(epd)
    }

    fn sleep(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
        // Section 8.2 from datasheet
        self.interface.cmd_with_data(
            spi,
            Command::VcomAndDataIntervalSetting,
            &[FLOATING_BORDER | VCOM_DATA_INTERVAL],
        )?;

        self.command(spi, Command::PowerOff)?;
        // The example STM code from Github has a wait after PowerOff
        self.wait_until_idle(spi, delay)?;

        self.cmd_with_data(spi, Command::DeepSleep, &[0xA5])?;

        Ok(())
    }

    fn wake_up(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
        self.init(spi, delay)
    }

    fn set_background_color(&mut self, color: TriColor) {
        self.color = color;
    }

    fn background_color(&self) -> &TriColor {
        &self.color
    }

    fn width(&self) -> u32 {
        WIDTH
    }

    fn height(&self) -> u32 {
        HEIGHT
    }

    fn update_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        delay: &mut DELAY,
    ) -> Result<(), SPI::Error> {
        self.interface.cmd(spi, Command::DataStartTransmission1)?;

        self.interface.data(spi, buffer)?;

        // Clear the chromatic layer
        let color = self.color.get_byte_value();

        self.interface.cmd(spi, Command::DataStartTransmission2)?;
        self.interface.data_x_times(spi, color, NUM_DISPLAY_BITS)?;

        self.wait_until_idle(spi, delay)?;
        Ok(())
    }

    #[allow(unused)]
    fn update_partial_frame(
        &mut self,
        spi: &mut SPI,
        delay: &mut DELAY,
        buffer: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), SPI::Error> {
        Ok(())
    }

    fn display_frame(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
        self.command(spi, Command::DisplayRefresh)?;

        self.wait_until_idle(spi, delay)?;
        Ok(())
    }

    fn update_and_display_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        delay: &mut DELAY,
    ) -> Result<(), SPI::Error> {
        self.update_frame(spi, buffer, delay)?;
        self.display_frame(spi, delay)?;
        Ok(())
    }

    fn clear_frame(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
        self.send_resolution(spi)?;

        let color = DEFAULT_BACKGROUND_COLOR.get_byte_value();

        // Clear the black
        self.interface.cmd(spi, Command::DataStartTransmission1)?;

        self.interface.data_x_times(spi, color, NUM_DISPLAY_BITS)?;

        // Clear the chromatic
        self.interface.cmd(spi, Command::DataStartTransmission2)?;
        self.interface.data_x_times(spi, color, NUM_DISPLAY_BITS)?;

        self.wait_until_idle(spi, delay)?;
        Ok(())
    }

    fn set_lut(
        &mut self,
        _spi: &mut SPI,
        _delay: &mut DELAY,
        _refresh_rate: Option<RefreshLut>,
    ) -> Result<(), SPI::Error> {
        Ok(())
    }

    // fn wait_until_idle_wit_cmd(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
    //     self.interface
    //         .wait_until_idle_with_cmd(spi, delay, IS_BUSY_LOW, Command::GetStatus)
    // }

    fn wait_until_idle(&mut self, _spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
        self.interface.wait_until_idle(delay, IS_BUSY_LOW);
        Ok(())
    }
}

impl<SPI, CS, BUSY, DC, RST, DELAY> Epd2in13bc<SPI, CS, BUSY, DC, RST, DELAY>
where
    SPI: Write<u8>,
    CS: OutputPin,
    BUSY: InputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayUs<u32>,
{
    fn command(&mut self, spi: &mut SPI, command: Command) -> Result<(), SPI::Error> {
        self.interface.cmd(spi, command)
    }

    fn send_data(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {
        self.interface.data(spi, data)
    }

    fn cmd_with_data(
        &mut self,
        spi: &mut SPI,
        command: Command,
        data: &[u8],
    ) -> Result<(), SPI::Error> {
        self.interface.cmd_with_data(spi, command, data)
    }

    fn send_resolution(&mut self, spi: &mut SPI) -> Result<(), SPI::Error> {
        let w = self.width();
        let h = self.height();

        self.command(spi, Command::ResolutionSetting)?;

        self.send_data(spi, &[w as u8])?;
        self.send_data(spi, &[(h >> 8) as u8])?;
        self.send_data(spi, &[h as u8])
    }

    /// Set the outer border of the display to the chosen color.
    pub fn set_border_color(&mut self, spi: &mut SPI, color: TriColor) -> Result<(), SPI::Error> {
        let border = match color {
            TriColor::Black => BLACK_BORDER,
            TriColor::White => WHITE_BORDER,
            TriColor::Chromatic => CHROMATIC_BORDER,
        };
        self.cmd_with_data(
            spi,
            Command::VcomAndDataIntervalSetting,
            &[border | VCOM_DATA_INTERVAL],
        )
    }
}
