use std::io::{self, Write};

use crate::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0, "canvas width must be greater than zero");
        assert!(height > 0, "canvas height must be greater than zero");

        Self {
            width,
            height,
            pixels: vec![Color::zero(); width * height],
        }
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub fn pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[self.index(x, y)]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.index(x, y);
        self.pixels[index] = color;
    }

    pub fn write_ppm(&self, writer: &mut impl Write) -> io::Result<()> {
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        for color in &self.pixels {
            writeln!(
                writer,
                "{} {} {}",
                channel_to_byte(color.x),
                channel_to_byte(color.y),
                channel_to_byte(color.z)
            )?;
        }

        Ok(())
    }

    fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width, "pixel x coordinate is outside the canvas");
        assert!(y < self.height, "pixel y coordinate is outside the canvas");
        y * self.width + x
    }
}

fn channel_to_byte(channel: f64) -> u8 {
    (channel.clamp(0.0, 1.0) * 255.0).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_pixels_by_column_and_row() {
        let mut canvas = Canvas::new(3, 2);
        let orange = Color::new(1.0, 0.5, 0.0);

        canvas.set_pixel(2, 1, orange);

        assert_eq!(canvas.pixel(2, 1), orange);
        assert_eq!(canvas.pixel(0, 0), Color::zero());
    }

    #[test]
    fn writes_a_complete_p3_ppm_and_clamps_channels() {
        let mut canvas = Canvas::new(2, 1);
        canvas.set_pixel(0, 0, Color::new(-0.1, 0.5, 1.5));
        canvas.set_pixel(1, 0, Color::new(1.0, 0.0, 0.25));
        let mut output = Vec::new();

        canvas.write_ppm(&mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "P3\n2 1\n255\n0 128 255\n255 0 64\n"
        );
    }
}
