use crate::epd7in5_hd::{DEFAULT_BACKGROUND_COLOR, HEIGHT, WIDTH};
use crate::graphics::{Display, DisplayRotation};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;

/// Full size buffer for use with the 7in5 EPD
///
/// Can also be manually constructed:
/// `buffer: [DEFAULT_BACKGROUND_COLOR.get_byte_value(); WIDTH / 8 * HEIGHT]`
pub struct Display7in5 {
    buffer: [u8; WIDTH as usize * HEIGHT as usize / 8],
    rotation: DisplayRotation,
}

impl Default for Display7in5 {
    fn default() -> Self {
        Display7in5 {
            buffer: [DEFAULT_BACKGROUND_COLOR.get_byte_value();
                WIDTH as usize * HEIGHT as usize / 8],
            rotation: DisplayRotation::default(),
        }
    }
}

impl DrawTarget for Display7in5 {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<BinaryColor>) -> Result<(), Self::Error> {
        self.draw_helper(WIDTH, HEIGHT, pixel)
    }

    fn size(&self) -> Size {
        Size::new(WIDTH, HEIGHT)
    }
}

impl Display for Display7in5 {
    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn get_mut_buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    fn set_rotation(&mut self, rotation: DisplayRotation) {
        self.rotation = rotation;
    }

    fn rotation(&self) -> DisplayRotation {
        self.rotation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Black, Color};
    use crate::epd7in5_hd;
    use crate::graphics::{Display, DisplayRotation};
    use embedded_graphics::{primitives::Line, style::PrimitiveStyle};

    // test buffer length
    #[test]
    fn graphics_size() {
        let display = Display7in5::default();
        assert_eq!(display.buffer().len(), 58080);
    }

    // test default background color on all bytes
    #[test]
    fn graphics_default() {
        let display = Display7in5::default();
        for &byte in display.buffer() {
            assert_eq!(byte, epd7in5_hd::DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }

    #[test]
    fn graphics_rotation_0() {
        let mut display = Display7in5::default();

        let _ = Line::new(Point::new(0, 0), Point::new(7, 0))
            .into_styled(PrimitiveStyle::with_stroke(Black, 1))
            .draw(&mut display);

        let buffer = display.buffer();

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, epd7in5_hd::DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }

    #[test]
    fn graphics_rotation_90() {
        let mut display = Display7in5::default();
        display.set_rotation(DisplayRotation::Rotate90);

        let _ = Line::new(Point::new(0, 872), Point::new(0, 879))
            .into_styled(PrimitiveStyle::with_stroke(Black, 1))
            .draw(&mut display);

        let buffer = display.buffer();

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, epd7in5_hd::DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }

    #[test]
    fn graphics_rotation_180() {
        let mut display = Display7in5::default();
        display.set_rotation(DisplayRotation::Rotate180);

        let _ = Line::new(Point::new(872, 527), Point::new(879, 527))
            .into_styled(PrimitiveStyle::with_stroke(Black, 1))
            .draw(&mut display);

        let buffer = display.buffer();

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, epd7in5_hd::DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }

    #[test]
    fn graphics_rotation_270() {
        let mut display = Display7in5::default();
        display.set_rotation(DisplayRotation::Rotate270);

        let _ = Line::new(Point::new(527, 0), Point::new(527, 7))
            .into_styled(PrimitiveStyle::with_stroke(Black, 1))
            .draw(&mut display);

        let buffer = display.buffer();

        assert_eq!(buffer[0], Color::Black.get_byte_value());

        for &byte in buffer.iter().skip(1) {
            assert_eq!(byte, epd7in5_hd::DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }
}
