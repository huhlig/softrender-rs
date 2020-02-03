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

pub struct PPM{
    dimensions: (usize,usize),
    buffer: Vec<(u8)>,
}

impl PPM {
    pub fn from_canvas(&mut self, canvas: &Canvas) {

    }
    pub fn to_file(canvas: &Canvas) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(20 + canvas.width() * canvas.height() * 12);
        write!(output, "P3\n")?;
        write!(output, "{} {}\n", canvas.width(), canvas.height())?;
        write!(output, "255\n")?;

        let mut count = 0;
        for y in 0..canvas.height() {
            for x in 0..canvas.width() {
                let color = canvas.get(x, y);
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
}