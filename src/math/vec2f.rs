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

#[derive(Copy, Clone, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    ///
    /// Create a new `Vec2f`
    ///
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    ///
    /// Normalize Vector
    ///
    pub fn normalize(&self) -> Self {
        let nor2 = (self.x * self.x) + (self.y * self.y);
        if nor2 > 0.0 {
            let inv_nor = 1.0f32 / nor2.sqrt();
            Self {
                x: self.x * inv_nor,
                y: self.y * inv_nor,
            }
        } else {
            Self {
                x: self.x,
                y: self.y,
            }
        }
    }
    ///
    /// Dot Product
    ///
    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
    /// Magnitude
    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
}

impl Default for Vec2f {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl fmt::Debug for Vec2f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl ops::Add<Self> for Vec2f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Self> for Vec2f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Div<f32> for Vec2f {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec2f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Div<Self> for Vec2f {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::DivAssign<Self> for Vec2f {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl ops::Mul<f32> for Vec2f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::MulAssign<f32> for Vec2f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Mul<Self> for Vec2f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::MulAssign<Self> for Vec2f {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::Neg for Vec2f {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Sub<f32> for Vec2f {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl ops::SubAssign<f32> for Vec2f {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl ops::Sub<Self> for Vec2f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}


impl ops::SubAssign<Self> for Vec2f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use super::Vec2f;

    #[test]
    fn test_vec2f_addition() {
        let a1 = Vec2f::new(3.0, -2.0);
        let a2 = Vec2f::new(-2.0, 3.0);
        assert_eq!(a1 + a2, Vec2f::new(1.0, 1.0));
    }

    #[test]
    fn test_vec2f_subtraction() {
        let p1 = Vec2f::new(3.0, 2.0);
        let p2 = Vec2f::new(5.0, 6.0);
        assert_eq!(p1 - p2, Vec2f::new(-2.0, -4.0));

        let p = Vec2f::new(3.0, 2.0);
        let v = Vec2f::new(5.0, 6.0);
        assert_eq!(p - v, Vec2f::new(-2.0, -4.0));

        let v1 = Vec2f::new(3.0, 2.0);
        let v2 = Vec2f::new(5.0, 6.0);
        assert_eq!(v1 - v2, Vec2f::new(-2.0, -4.0));

        let zero = Vec2f::new(0.0, 0.0);
        let v = Vec2f::new(1.0, -2.0);
        assert_eq!(zero - v, Vec2f::new(-1.0, 2.0));
    }

    #[test]
    fn test_negation() {
        let a = Vec2f::new(1.0, -2.0);
        assert_eq!(-a, Vec2f::new(-1.0, 2.0));
    }

    #[test]
    fn test_scalar_multiplication() {
        let a = Vec2f::new(1.0, -2.0);
        assert_eq!(a * 3.5, Vec2f::new(3.5, -7.0));

        let b = Vec2f::new(1.0, -2.0);
        assert_eq!(b * 0.5, Vec2f::new(0.5, -1.0));
    }

    #[test]
    fn test_scalar_division() {
        let a = Vec2f::new(1.0, -2.0);
        assert_eq!(a / 2.0, Vec2f::new(0.5, -1.0));
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Vec2f::new(0.0, 0.0).magnitude(), 0.0);
        assert_eq!(Vec2f::new(1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Vec2f::new(0.0, 1.0).magnitude(), 1.0);
        assert_eq!(Vec2f::new(1.0, 2.0).magnitude(), 5.0f32.sqrt());
        assert_eq!(Vec2f::new(-1.0, -2.0).magnitude(), 5.0f32.sqrt());
    }

    #[test]
    fn test_normalization() {
        assert_eq!(Vec2f::new(4.0, 0.0).normalize(), Vec2f::new(1.0, 0.0));
        assert_eq!(Vec2f::new(1.0, 2.0).normalize(), Vec2f::new(0.4472135955, 0.894427191));
        assert_approx_eq!(Vec2f::new(1.0, 2.0).normalize().magnitude(), 1.0);
    }

    #[test]
    fn test_dot_product() {
        let a = Vec2f::new(1.0, 2.0);
        let b = Vec2f::new(2.0, 3.0);
        assert_approx_eq!(a.dot(b), 8.0)
    }
}
