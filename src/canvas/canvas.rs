//
// Copyright 2020 Hans W. Uhlig.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use super::Color;

/// Image Canvas
pub struct Canvas {
    dimensions: (usize, usize),
    color_buffer: Vec<Color>,
}

impl Canvas {
    /// Create new Canvas
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            dimensions: (width, height),
            color_buffer: vec![Color::black(); width * height],
        }
    }
    /// Get Width of Canvas
    pub fn width(&self) -> usize {
        self.dimensions.0
    }
    /// Get Height of Canvas
    pub fn height(&self) -> usize {
        self.dimensions.1
    }
    /// Get Color of Pixel at (x, y)
    pub fn get(&self, x: usize, y: usize) -> Color {
        assert!(x < self.dimensions.0);
        assert!(y < self.dimensions.1);
        self.color_buffer[(y * self.dimensions.0) + x]
    }
    /// Set `Color` of Pixel at (x, y) to color
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < self.dimensions.0);
        assert!(y < self.dimensions.1);
        self.color_buffer[(y * self.dimensions.0) + x] = color;
        self.image_buffer[(y * self.dimensions.0) + x] = color.to_argb();
    }
    /// As a u32 buffer slice for MiniFB
    pub fn as_slice(&self) -> &[u32] {
        &self.image_buffer
    }
    /// Save to PPM File
    pub fn to_ppm(&self) -> Result<Vec<u8>, std::io::Error> {
        use std::io::Write;
        let mut output = Vec::with_capacity(20 + self.width() * self.height() * 12);
        write!(output, "P3\n")?;
        write!(output, "{} {}\n", self.width(), self.height())?;
        write!(output, "255\n")?;

        let mut count = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let color = self.get(x, y);
                write!(output, "{} {} {}", u8::from(color.r), u8::from(color.g), u8::from(color.b))?;
                if count < 4 {
                    write!(output, " ")?;
                    count += 1;
                } else {
                    write!(output, "\n")?;
                    count = 0;
                }
            }
        }

        Ok(output)
    }
    /// Save to PPM File
    /// https://en.wikipedia.org/wiki/BMP_file_format
    pub fn to_bmp(&self) -> Result<Vec<u8>, std::io::Error> {
        use byteorder::{LittleEndian, WriteBytesExt};

        let data_size = self.width() * self.height() * 4;
        let file_size = 14 + 40 + data_size;
        let mut output = Vec::with_capacity(file_size);
        // BMP Header - 14 Bytes
        output.write_u8(0x42)?; // Magic Number (BM) - 2 bytes
        output.write_u8(0x4D)?; // Magic Number (BM) - 2 bytes
        output.write_u32::<LittleEndian>(file_size as u32)?; // File Size - 4 Bytes Unsigned
        output.write_u32::<LittleEndian>(0)?; // Reserved - 4 Bytes Unused
        output.write_u32::<LittleEndian>(54)?; // Data Offset - 4 Bytes Unsigned

        // DIB Header - BITMAPINFOHEADER - 40 Bytes
        output.write_u32::<LittleEndian>(40)?; // DIB Header Size - 4 bytes Unsigned
        output.write_i32::<LittleEndian>(self.width() as i32)?; // Image Width - 4 bytes Signed
        output.write_i32::<LittleEndian>(self.height() as i32)?; // Image Height - 4 bytes Signed
        output.write_u16::<LittleEndian>(1)?; // Color Planes - 2 Bytes Unsigned
        output.write_u16::<LittleEndian>(32)?; // Bits Per Pixel - 2 Bytes Unsigned
        output.write_u32::<LittleEndian>(0)?; // Compression - 4 bytes Unsigned
        output.write_u32::<LittleEndian>(data_size as u32)?; // Image Data Size - 4 Bytes Unsigned
        output.write_i32::<LittleEndian>(0)?; // Horizontal Resolution - 4 bytes Unsigned
        output.write_i32::<LittleEndian>(0)?; // Vertical Resolution - 4 bytes Unsigned
        output.write_u32::<LittleEndian>(0)?; // Colors in Palette - 4 bytes Unsigned
        output.write_u32::<LittleEndian>(0)?; // Important Colors in Palette - 4 bytes Unsigned

        // Write out Image Data
        for y in 0..self.height() {
            for x in 0..self.width() {
                let color = self.get(x, y);
                output.write_u8(0xFFu8)?;
                output.write_u8(color.r.into())?;
                output.write_u8(color.g.into())?;
                output.write_u8(color.b.into())?;
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::{Canvas, Color};

    #[test]
    fn test_creation() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);
        for y in 0..20 {
            for x in 0..10 {
                assert_eq!(c.get(x, y), Color::black());
            }
        }
    }

    #[test]
    fn test_to_ppm() {
        let c = {
            let mut c = Canvas::new(5, 3);
            c.set(0, 0, Color::new(1.5, 0.0, 0.0));
            c.set(2, 1, Color::new(0.0, 0.5, 0.0));
            c.set(4, 2, Color::new(-0.5, 0.0, 1.0));
            c
        };
        let expected = Vec::from(
            "P3\n\
             5 3\n\
             255\n\
             255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
             0 0 0 0 0 0 0 127 0 0 0 0 0 0 0\n\
             0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");
        let result = c.to_ppm().unwrap();
        assert_eq!(result, expected);
    }
}