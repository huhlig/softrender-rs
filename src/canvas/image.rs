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

/// Image
pub struct Image {
    dimensions: (usize, usize),
    color_buffer: Vec<Color>,
    image_buffer: Vec<u32>,
}

impl Image {
    /// Create a new Image
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            dimensions: (width, height),
            color_buffer: vec![Color::black(); width * height],
            image_buffer: vec![0; width * height],
        }
    }
    /// Get Width
    pub fn width(&self) -> usize {
        self.dimensions.0
    }
    /// Get Height
    pub fn height(&self) -> usize {
        self.dimensions.0
    }
    /// Get Dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }
    /// Get Color of Pixel at (x, y)
    pub fn get(&self, x: usize, y: usize) -> Color {
        assert!(x < self.dimensions.0);
        assert!(y < self.dimensions.1);
        self.color_buffer[(y * self.width()) + x]
    }
    /// Set Color of Pixel at (x, y) to color
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < self.dimensions.0);
        assert!(y < self.dimensions.1);
        self.color_buffer[(y * self.dimensions.0) + x] = color;
        //self.image_buffer[(y * self.dimensions.0) + x] = color.into();
    }
    /// Get Image as slice
    pub fn as_u32_slice(&self) -> &[u32] {
        &self.image_buffer as &[u32]
    }
}
