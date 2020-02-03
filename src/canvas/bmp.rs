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

pub struct BMP;

impl BMP {
    pub fn from_canvas(canvas: &Canvas) -> Result<Vec<u8>> {
        use byteorder::{LittleEndian, WriteBytesExt};

        let data_size = canvas.width() * canvas.height() * 4;
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