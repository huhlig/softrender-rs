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
    image_buffer: Vec<u32>,
}

impl Canvas {
    /// Create new Canvas
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            dimensions: (width, height),
            color_buffer: vec![Color::black(); width * height],
            image_buffer: vec![Color::black().into(); width * height],
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
        assert!(0 <= x && x < self.dimensions.0);
        assert!(0 <= y && y < self.dimensions.1);
        self.color_buffer[(y * self.dimensions.0) + x]
    }
    /// Set `Color` of Pixel at (x, y) to color
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        assert!(0 <= x && x < self.dimensions.0);
        assert!(0 <= y && y < self.dimensions.1);
        self.color_buffer[(y * self.dimensions.0) + x] = color;
    }
    /// As a u32 buffer slice for MiniFB
    pub fn as_slice(&self) -> &[u32] {
        &self.image_buffer
    }
    /// Save to PPM File
    pub fn save_to_ppm(&self, file: File) -> Result<()> {
        file.write_fmt(format_args!("P3\n{} {}\n255\n", self.width(), self.height()))?;
        let mut len = 0;
        for y in 0..self.height() {
            for x in 0..self.height() {
                file.write()?;
            }
        }


        Ok(())
    }
    /// Save to PPM File
    pub fn save_to_bmp(&self, file: File) -> Result<()> {
        // https://en.wikipedia.org/wiki/BMP_file_format
        // BMP Header
        let magic = [0x42u8, 0x4D]; // Magic Number (BM) - 2 bytes
        let file_size = 0u32; // File Size - 4 Bytes
        let reserved = 0u32; // Reserved - 4 Bytes
        let offset = 0u32; // Offset to Data - 4 bytes

        // DIB Header
        let dib_size = 40u32; // DIB Header Size - 4 bytes
        let width = self.dimensions.0 as i32; // Image Width - 4 bytes signed
        let height = self.dimensions.1 as i32; // Image Height - 4 bytes signed
        let color_planes = 1u16; // Color Planes - 2 bytes
        let bits_per_pixel = 32u16; // Bits Per Pixel - 2 bytes
        let compression= 0u32; // Compression - 4 Bytes
        let data_size = 0u32; // Image Data Size - 4 Bytes
        let horiz_res = 0i32; // Horizontal Resolution - 4 bytes
        let vert_res = 0i32; // Vertical Resolution - 4 bytes
        let color_count = 0u32; // Colors in Palette - 4 bytes
        let color_important = 0u32; // Important Collors - 4 bytes



        Ok(())
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
}