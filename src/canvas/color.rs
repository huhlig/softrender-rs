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

use std::{fmt, ops};

///
/// Color
///
#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    /// Create a new Custom Color
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
    /// Create Color `Black` (0.0, 0.0, 0.0)
    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }
    /// Create Color `Dark Red` (0.5, 0.0, 0.0)
    pub fn dark_red() -> Self {
        Self { r: 0.5, g: 0.0, b: 0.0 }
    }
    /// Create Color `Dark Green` (0.0, 0.5, 0.0)
    pub fn dark_green() -> Self {
        Self { r: 0.0, g: 0.5, b: 0.0 }
    }
    /// Create Color `Dark Blue` (0.0, 0.0, 0.5)
    pub fn dark_blue() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.5 }
    }
    /// Create Color `Dark Yellow` (0.5, 0.5, 0.0)
    pub fn dark_yellow() -> Self {
        Self { r: 0.5, g: 0.5, b: 0.0 }
    }
    /// Create Color `Dark Yellow` (0.0, 0.5, 0.5)
    pub fn dark_cyan() -> Self {
        Self { r: 0.0, g: 0.5, b: 0.5 }
    }
    /// Create Color `Dark Yellow` (0.5, 0.0, 0.5)
    pub fn dark_magenta() -> Self {
        Self { r: 0.5, g: 0.0, b: 0.5 }
    }
    /// Create Color `Grey` (0.5, 0.5, 0.5)
    pub fn grey() -> Self {
        Self { r: 0.5, g: 0.5, b: 0.5 }
    }
    /// Create Color `Bright Red` (0.1, 0.0, 0.0)
    pub fn bright_red() -> Self {
        Self { r: 1.0, g: 0.0, b: 0.0 }
    }
    /// Create Color `Bright Green` (0.0, 1.0, 0.0)
    pub fn bright_green() -> Self {
        Self { r: 0.0, g: 1.0, b: 0.0 }
    }
    /// Create Color `Bright Blue` (0.0, 0.0, 1.0)
    pub fn bright_blue() -> Self {
        Self { r: 0.0, g: 0.0, b: 1.0 }
    }
    /// Create Color `Bright Yellow` (1.0, 1.0, 0.0)
    pub fn bright_yellow() -> Self {
        Self { r: 1.0, g: 1.0, b: 0.0 }
    }
    /// Create Color `Bright Cyan` (0.0, 1.0, 1.0)
    pub fn bright_cyan() -> Self {
        Self { r: 0.0, g: 1.0, b: 1.0 }
    }
    /// Create Color `Bright Magenta` (1.0, 0.0, 1.0)
    pub fn bright_magenta() -> Self {
        Self { r: 1.0, g: 0.0, b: 1.0 }
    }
    /// Create Color `White` (1.0, 1.0, 1.0)
    pub fn white() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0 }
    }
}


impl Default for Color {
    fn default() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0 }
    }
}

impl PartialEq<Color> for Color {
    fn eq(&self, other: &Color) -> bool {
        u32::from(self) == u32::from(other)
    }
}

impl ops::Add<Self> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::AddAssign<Self> for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl ops::Mul<Self> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl ops::MulAssign<Self> for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}


impl ops::Sub<Color> for Color {
    type Output = Self;

    fn sub(self, rhs: Color) -> Self {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl ops::SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = (self.r * 255.0) as u8;
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;
        write!(f, "0x{:02X}{:02X}{:02X}", r, g, b)
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = (self.r.min(0.0).max(1.0) * 255.0) as u8;
        let g = (self.g.min(0.0).max(1.0) * 255.0) as u8;
        let b = (self.b.min(0.0).max(1.0) * 255.0) as u8;
        write!(f, "Color {{ r: {}, g: {}, b: {}, 0x{:02X}{:02X}{:02X} }}", self.r, self.g, self.b, r, g, b)
    }
}

impl From<&Color> for u32 {
    fn from(value: &Color) -> Self {
        let r = ((value.r.min(0.0).max(1.0) * 255.0) as u32) << 24;
        let g = ((value.g.min(0.0).max(1.0) * 255.0) as u32) << 16;
        let b = ((value.b.min(0.0).max(1.0) * 255.0) as u32) << 08;
        r | g | b | 0xFF
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_color_equality() {
        assert_eq!(Color::new(0.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0));
        assert_eq!(Color::new(0.25, 0.50, 0.75), Color::new(0.25, 0.50, 0.75));
        assert_eq!(Color::new(-5.0, 0.5, 5.0), Color::new(0.0, 0.5, 1.0));
    }

    #[test]
    fn test_color_addition() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_color_subtraction() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_scalar_multiplication() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_color_multiplication() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}