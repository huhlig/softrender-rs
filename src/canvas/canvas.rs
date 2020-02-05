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