//
// Copyright 2017 Hans W. Uhlig.
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

use crate::color::ColorRGBAu8;
use crate::image::ImageRGBAu8;
use crate::rasterizer::Rasterizer;

impl Rasterizer<ColorRGBAu8> for ImageRGBAu8 {
    /// Clear an canvas
    fn clear(&mut self, color: ColorRGBAu8) {
        self.fill(color);
    }
    fn draw_point(&mut self, x: usize, y: usize, color: ColorRGBAu8) {
        self.set(x, y, color);
    }
    /// Draw a line from (x1, y1) to (x2, y2) using Bresenham's line algorithm.
    fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: ColorRGBAu8) {
        let m_new = 2 * (y2 - y1) as isize;
        let mut slope_error_new = m_new - (x2 - x1) as isize;
        let mut y = y1;
        for x in x1..x2 {
            // Set Pixel to line color
            self.set(x, y, color);
            // Add slope to increment angle formed
            slope_error_new += m_new;
            // Slope Error Reached Limit, time to
            // increment y and update slope error.
            if slope_error_new >= 0 {
                y += 1;
                slope_error_new -= 2 * (x2 - x1) as isize;
            }
        }
    }
}