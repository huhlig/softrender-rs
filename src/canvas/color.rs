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

use super::Channel;
use std::{fmt, ops};

///
/// Color
///
#[derive(Copy, Clone)]
pub struct Color {
    pub r: Channel,
    pub g: Channel,
    pub b: Channel,
}

impl Color {
    /// Create a new Custom Color
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: Channel::from(r),
            g: Channel::from(g),
            b: Channel::from(b),
        }
    }
    /// Create Color `Black` (0.0, 0.0, 0.0)
    pub fn black() -> Self { Self::new(0.0, 0.0, 0.0) }
    /// Create Color `Dark Red` (0.5, 0.0, 0.0)
    pub fn dark_red() -> Self { Self::new(0.5, 0.0, 0.0) }
    /// Create Color `Dark Green` (0.0, 0.5, 0.0)
    pub fn dark_green() -> Self { Self::new(0.0, 0.5, 0.0) }
    /// Create Color `Dark Blue` (0.0, 0.0, 0.5)
    pub fn dark_blue() -> Self { Self::new(0.0, 0.0, 0.5) }
    /// Create Color `Dark Yellow` (0.5, 0.5, 0.0)
    pub fn dark_yellow() -> Self { Self::new(0.5, 0.5, 0.0) }
    /// Create Color `Dark Yellow` (0.0, 0.5, 0.5)
    pub fn dark_cyan() -> Self { Self::new(0.0, 0.5, 0.5) }
    /// Create Color `Dark Yellow` (0.5, 0.0, 0.5)
    pub fn dark_magenta() -> Self { Self::new(0.5, 0.0, 0.5) }
    /// Create Color `Grey` (0.5, 0.5, 0.5)
    pub fn grey() -> Self { Self::new(0.5, 0.5, 0.5) }
    /// Create Color `Bright Red` (0.1, 0.0, 0.0)
    pub fn bright_red() -> Self { Self::new(1.0, 0.0, 0.0) }
    /// Create Color `Bright Green` (0.0, 1.0, 0.0)
    pub fn bright_green() -> Self { Self::new(0.0, 1.0, 0.0) }
    /// Create Color `Bright Blue` (0.0, 0.0, 1.0)
    pub fn bright_blue() -> Self { Self::new(0.0, 0.0, 1.0) }
    /// Create Color `Bright Yellow` (1.0, 1.0, 0.0)
    pub fn bright_yellow() -> Self { Self::new(1.0, 1.0, 0.0) }
    /// Create Color `Bright Cyan` (0.0, 1.0, 1.0)
    pub fn bright_cyan() -> Self { Self::new(0.0, 1.0, 1.0) }
    /// Create Color `Bright Magenta` (1.0, 0.0, 1.0)
    pub fn bright_magenta() -> Self { Self::new(1.0, 0.0, 1.0) }
    /// Create Color `White` (1.0, 1.0, 1.0)
    pub fn white() -> Self { Self::new(1.0, 1.0, 1.0) }
    pub fn to_rgba(&self) -> u32 {
        let r = (u8::from(self.r) as u32) << 24;
        let g = (u8::from(self.g) as u32) << 16;
        let b = (u8::from(self.b) as u32) << 08;
        let a = 0x000000FFu32;
        r | g | b | a
    }
    pub fn to_argb(&self) -> u32 {
        let r = (u8::from(self.r) as u32) << 24;
        let g = (u8::from(self.g) as u32) << 16;
        let b = (u8::from(self.b) as u32) << 08;
        let a = 0xFF000000u32;
        a | r | g | b
    }
}


impl Default for Color {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl PartialEq<Self> for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
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

impl ops::Sub<Self> for Color {
    type Output = Self;

    fn sub(self, rhs: Color) -> Self {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl ops::SubAssign<Self> for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color {{ r: {}, g: {}, b: {} }}", self.r, self.g, self.b)
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color {{ r: {} ({}), g: {} ({}), b: {} ({}) }}",
               f32::from(self.r), u8::from(self.r),
               f32::from(self.g), u8::from(self.g),
               f32::from(self.b), u8::from(self.b),
        )
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
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
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