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

pub trait PPM {
    fn to_ppm<T: Write>(&self, output: &mut T) -> Result<()>;
}

impl PPM for Canvas {
    fn to_ppm<T: Write>(&self, output: &mut T) -> Result<()> {
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::{Canvas, Color, PPM};

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

        let mut result = Vec::with_capacity(20 + c.width() * c.height() * 4);
        c.to_ppm(&mut result).unwrap();
        assert_eq!(result, expected);
    }
}