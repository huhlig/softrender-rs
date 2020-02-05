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

use super::Canvas;
use std::io::{Result, Write};

pub trait BMP {
    fn to_bmp<T: Write>(&self, output: &mut T) -> Result<()>;
}

impl BMP for Canvas {
    fn to_bmp<T: Write>(&self, output: &mut T) -> Result<()> {
        use byteorder::{LittleEndian, WriteBytesExt};

        let data_size = self.width() * self.height() * 4;
        let file_size = 14 + 40 + data_size;
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

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::canvas::{Canvas, Color, BMP};

    //#[test]
    fn test_to_bmp() {
        let c = {
            let mut c = Canvas::new(5, 3);
            c.set(0, 0, Color::new(1.5, 0.0, 0.0));
            c.set(2, 1, Color::new(0.0, 0.5, 0.0));
            c.set(4, 2, Color::new(-0.5, 0.0, 1.0));
            c
        };
        let expected = vec![
            // FILE HEADER
            0x42, 0x4D, // Magic Number
            0xFF, 0xFF, 0xFF, 0xFF, // File Size - TODO - Calculate
            0x00, 0x00, 0x00, 0x00, // Reserved
            0x00, 0x00, 0x00, 0x36, // Data Offset
            // DIB HEADER
            0x28, // DIB Header Size
            0x00, 0x00, 0x00, 0x00, // Width
            0x00, 0x00, 0x00, 0x00, // Height
            0x00, 0x00, 0x00, 0x01, // Color Planes
            0x00, 0x00, 0x00, 0x20, // Bits Per Pixel
            0x00, 0x00, 0x00, 0x00, // Compression
            0x00, 0x00, 0x00, 0x00, // Data Size -- TODO - Calculate
            0x00, 0x00, 0x00, 0x00, // Horizontal Resolution
            0x00, 0x00, 0x00, 0x00, // Vertical Resolution
            0x00, 0x00, 0x00, 0x00, // Colors in Palette
            0x00, 0x00, 0x00, 0x00, // Important Colors in Palette
            // Image Data
            0x00, 0x00, 0x00, 0x00, // Pixel (0, 0)
            0x00, 0x00, 0x00, 0x00, // Pixel (1, 0)
            0x00, 0x00, 0x00, 0x00, // Pixel (2, 0)
            0x00, 0x00, 0x00, 0x00, // Pixel (3, 0)
            0x00, 0x00, 0x00, 0x00, // Pixel (4, 0)
            0x00, 0x00, 0x00, 0x00, // Pixel (0, 1)
            0x00, 0x00, 0x00, 0x00, // Pixel (1, 1)
            0x00, 0x00, 0x00, 0x00, // Pixel (2, 1)
            0x00, 0x00, 0x00, 0x00, // Pixel (3, 1)
            0x00, 0x00, 0x00, 0x00, // Pixel (4, 1)
            0x00, 0x00, 0x00, 0x00, // Pixel (0, 2)
            0x00, 0x00, 0x00, 0x00, // Pixel (1, 2)
            0x00, 0x00, 0x00, 0x00, // Pixel (2, 2)
            0x00, 0x00, 0x00, 0x00, // Pixel (3, 2)
            0x00, 0x00, 0x00, 0x00, // Pixel (4, 2)
        ];

        let mut result = Vec::with_capacity(20 + c.width() * c.height() * 4);
        c.to_bmp(&mut result).unwrap();
        assert_eq!(result, expected);
    }
}