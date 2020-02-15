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
use super::{Mat4f, Vec3f};

///
/// 4 Dimensional Vector
///
#[derive(Copy, Clone, Debug)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4f {
    ///
    /// Create a new `Vec4f` from parts
    ///
    pub fn from_parts(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    ///
    /// Create a new `Vec4f` from an array
    ///
    pub fn from_array(data: [f32; 4]) -> Self {
        Self { x: data[0], y: data[1], z: data[2], w: data[3] }
    }
    ///
    /// Create an array from a Vec4f
    ///
    pub fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
    ///
    /// Normalize Vector
    ///
    pub fn normalize(&self) -> Self {
        let nor2 = (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w);
        if nor2 > 0.0 {
            let inv_nor = 1.0f32 / nor2.sqrt();
            Self {
                x: self.x * inv_nor,
                y: self.y * inv_nor,
                z: self.z * inv_nor,
                w: self.w * inv_nor,
            }
        } else {
            Self {
                x: self.x,
                y: self.y,
                z: self.z,
                w: self.w,
            }
        }
    }
    /// Dot Product
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
    /// Magnitude
    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }
}

impl Default for Vec4f {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
}

impl From<Vec3f> for Vec4f {
    fn from(value: Vec3f) -> Self {
        Vec4f { x: value.x, y: value.y, z: value.z, w: 1.0 }
    }
}

impl fmt::Display for Vec4f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl PartialEq<Self> for Vec4f {
    fn eq(&self, other: &Self) -> bool {
        let x = (self.x - other.x).abs() < std::f32::EPSILON;
        let y = (self.y - other.y).abs() < std::f32::EPSILON;
        let z = (self.z - other.z).abs() < std::f32::EPSILON;
        let w = (self.w - other.w).abs() < std::f32::EPSILON;
        x & y & z & w
    }
}

impl ops::Add<Self> for Vec4f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::AddAssign<Self> for Vec4f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl ops::Div<f32> for Vec4f {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec4f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl ops::Div<Self> for Vec4f {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl ops::DivAssign<Self> for Vec4f {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl ops::Mul<f32> for Vec4f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::MulAssign<f32> for Vec4f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl ops::Mul<Self> for Vec4f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl ops::MulAssign<Self> for Vec4f {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl ops::Mul<Mat4f> for Vec4f {
    type Output = Self;

    fn mul(self, rhs: Mat4f) -> Self {
        let x = self.x * rhs.m00 + self.y * rhs.m10 + self.z * rhs.m20 + self.w * rhs.m30;
        let y = self.x * rhs.m01 + self.y * rhs.m11 + self.z * rhs.m21 + self.w * rhs.m31;
        let z = self.x * rhs.m02 + self.y * rhs.m12 + self.z * rhs.m22 + self.w * rhs.m32;
        let w = self.x * rhs.m03 + self.y * rhs.m13 + self.z * rhs.m23 + self.w * rhs.m33;
        if w != 0.0 {
            Self {
                x: x / w,
                y: y / w,
                z: z / w,
                w: w / w,
            }
        } else {
            Self { x, y, z, w }
        }
    }
}

impl ops::Neg for Vec4f {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Sub<f32> for Vec4f {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}

impl ops::SubAssign<f32> for Vec4f {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}

impl ops::Sub<Self> for Vec4f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::SubAssign<Self> for Vec4f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl From<Vec4f> for [f32; 4] {
    fn from(other: Vec4f) -> Self {
        other.to_array()
    }
}

impl From<[f32; 4]> for Vec4f {
    fn from(other: [f32; 4]) -> Self {
        Self::from_array(other)
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use super::Vec4f;

    #[test]
    fn test_vec4f_addition() {
        let a1 = Vec4f::from_parts(3.0, -2.0, 5.0, 1.0);
        let a2 = Vec4f::from_parts(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, Vec4f::from_parts(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn test_vec4f_subtraction() {
        let p1 = Vec4f::from_parts(3.0, 2.0, 1.0, 1.0);
        let p2 = Vec4f::from_parts(5.0, 6.0, 7.0, 1.0);
        assert_eq!(p1 - p2, Vec4f::from_parts(-2.0, -4.0, -6.0, 0.0));

        let p = Vec4f::from_parts(3.0, 2.0, 1.0, 1.0);
        let v = Vec4f::from_parts(5.0, 6.0, 7.0, 0.0);
        assert_eq!(p - v, Vec4f::from_parts(-2.0, -4.0, -6.0, 1.0));

        let v1 = Vec4f::from_parts(3.0, 2.0, 1.0, 0.0);
        let v2 = Vec4f::from_parts(5.0, 6.0, 7.0, 0.0);
        assert_eq!(v1 - v2, Vec4f::from_parts(-2.0, -4.0, -6.0, 0.0));

        let zero = Vec4f::from_parts(0.0, 0.0, 0.0, 0.0);
        let v = Vec4f::from_parts(1.0, -2.0, 3.0, 0.0);
        assert_eq!(zero - v, Vec4f::from_parts(-1.0, 2.0, -3.0, 0.0));
    }

    #[test]
    fn test_negation() {
        let a = Vec4f::from_parts(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Vec4f::from_parts(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_scalar_multiplication() {
        let a = Vec4f::from_parts(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Vec4f::from_parts(3.5, -7.0, 10.5, -14.0));

        let b = Vec4f::from_parts(1.0, -2.0, 3.0, -4.0);
        assert_eq!(b * 0.5, Vec4f::from_parts(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_scalar_division() {
        let a = Vec4f::from_parts(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Vec4f::from_parts(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Vec4f::from_parts(0.0, 0.0, 0.0, 0.0).magnitude(), 0.0);
        assert_eq!(Vec4f::from_parts(1.0, 0.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Vec4f::from_parts(0.0, 1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Vec4f::from_parts(0.0, 0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Vec4f::from_parts(0.0, 0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(Vec4f::from_parts(1.0, 2.0, 3.0, 0.0).magnitude(), 14.0f32.sqrt());
        assert_eq!(Vec4f::from_parts(-1.0, -2.0, -3.0, 0.0).magnitude(), 14.0f32.sqrt());
    }

    #[test]
    fn test_normalization() {
        assert_eq!(Vec4f::from_parts(4.0, 0.0, 0.0, 0.0).normalize(), Vec4f::from_parts(1.0, 0.0, 0.0, 0.0));
        assert_eq!(Vec4f::from_parts(1.0, 2.0, 3.0, 0.0).normalize(), Vec4f::from_parts(0.26726124, 0.5345225, 0.8017837, 0.0));
        assert_approx_eq!(Vec4f::from_parts(1.0, 2.0, 3.0, 0.0).normalize().magnitude(), 1.0);
    }

    #[test]
    fn test_dot_product() {
        let a = Vec4f::from_parts(1.0, 2.0, 3.0, 0.0);
        let b = Vec4f::from_parts(2.0, 3.0, 4.0, 0.0);
        assert_approx_eq!(a.dot(b), 20.0)
    }
}
