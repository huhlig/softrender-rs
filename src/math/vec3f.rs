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
use super::{Mat4f, Vec4f};

///
/// 3 Dimensional Vector
///
#[derive(Copy, Clone, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    ///
    /// Create a new `Vec3f`
    ///
    pub fn from_parts(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    ///
    /// Create a new `Vec3f`
    ///
    pub fn from_array(data: [f32; 3]) -> Self {
        Self { x: data[0], y: data[1], z: data[2] }
    }
    ///
    /// Turn Vec3f into an Array
    ///
    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    ///
    /// Normalize Vector
    ///
    pub fn normalize(&self) -> Self {
        let nor2 = (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
        if nor2 > 0.0 {
            let inv_nor = 1.0f32 / nor2.sqrt();
            Self {
                x: self.x * inv_nor,
                y: self.y * inv_nor,
                z: self.z * inv_nor,
            }
        } else {
            Self {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }
    }
    ///
    /// Dot Product
    ///
    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    ///
    /// Cross Product
    ///
    pub fn cross(l: Vec3f, r: Vec3f) -> Self {
        Self {
            x: (l.y * r.z) - (l.z * r.y),
            y: (l.z * r.x) - (l.x * r.z),
            z: (l.x * r.y) - (l.y * r.x),
        }
    }
    /// Magnitude
    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }
}

impl Default for Vec3f {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl From<Vec4f> for Vec3f {
    fn from(value: Vec4f) -> Self {
        Self { x: value.x, y: value.y, z: value.z }
    }
}

impl fmt::Debug for Vec3f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl ops::Add<Self> for Vec3f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Self> for Vec3f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Div<f32> for Vec3f {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec3f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Div<Self> for Vec3f {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::DivAssign<Self> for Vec3f {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl ops::Mul<f32> for Vec3f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f32> for Vec3f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Mul<Self> for Vec3f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::MulAssign<Self> for Vec3f {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::Mul<Mat4f> for Vec3f {
    type Output = Self;

    fn mul(self, rhs: Mat4f) -> Self {
        let x = self.x * rhs.c0r0 + self.y * rhs.c1r0 + self.z * rhs.c2r0 + rhs.c3r0;
        let y = self.x * rhs.c0r1 + self.y * rhs.c1r1 + self.z * rhs.c2r1 + rhs.c3r1;
        let z = self.x * rhs.c0r2 + self.y * rhs.c1r2 + self.z * rhs.c2r2 + rhs.c3r2;
        let w = self.x * rhs.c0r3 + self.y * rhs.c1r3 + self.z * rhs.c2r3 + rhs.c3r3;
        if w != 0.0 {
            Self {
                x: x / w,
                y: y / w,
                z: z / w,
            }
        } else {
            Self { x, y, z }
        }
    }
}

impl ops::Neg for Vec3f {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Sub<f32> for Vec3f {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl ops::SubAssign<f32> for Vec3f {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl ops::Sub<Self> for Vec3f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign<Self> for Vec3f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl From<[f32; 3]> for Vec3f {
    fn from(other: [f32; 3]) -> Self {
        Vec3f::from_array(other)
    }
}

impl From<Vec3f> for [f32; 3] {
    fn from(other: Vec3f) -> Self {
        other.to_array()
    }
}


#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use super::Vec3f;

    #[test]
    fn test_from_parts() {
        let a = Vec3f::from_parts(3.0, -2.0, 5.0);
        assert_approx_eq!(a.x, 3.0);
        assert_approx_eq!(a.x, -2.0);
        assert_approx_eq!(a.x, 5.0);
    }

    #[test]
    fn test_from_array() {
        let a = Vec3f::from_array([3.0, -2.0, 5.0]);
        assert_approx_eq!(a.x, 3.0);
        assert_approx_eq!(a.x, -2.0);
        assert_approx_eq!(a.x, 5.0);
    }

    #[test]
    fn test_to_array() {
        assert_eq!(Vec3f::from_parts(3.0, -2.0, 5.0).to_array(), [3.0, -2.0, 5.0]);
    }

    #[test]
    fn test_vec3f_addition() {
        let a1 = Vec3f::from_parts(3.0, -2.0, 5.0);
        let a2 = Vec3f::from_parts(-2.0, 3.0, 1.0);
        assert_eq!(a1 + a2, Vec3f::from_parts(1.0, 1.0, 6.0));
    }

    #[test]
    fn test_vec3f_subtraction() {
        let p1 = Vec3f::from_parts(3.0, 2.0, 1.0);
        let p2 = Vec3f::from_parts(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Vec3f::from_parts(-2.0, -4.0, -6.0));

        let p = Vec3f::from_parts(3.0, 2.0, 1.0);
        let v = Vec3f::from_parts(5.0, 6.0, 7.0);
        assert_eq!(p - v, Vec3f::from_parts(-2.0, -4.0, -6.0));

        let v1 = Vec3f::from_parts(3.0, 2.0, 1.0);
        let v2 = Vec3f::from_parts(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Vec3f::from_parts(-2.0, -4.0, -6.0));

        let zero = Vec3f::from_parts(0.0, 0.0, 0.0);
        let v = Vec3f::from_parts(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Vec3f::from_parts(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_negation() {
        let a = Vec3f::from_parts(1.0, -2.0, 3.0);
        assert_eq!(-a, Vec3f::from_parts(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_scalar_multiplication() {
        let a = Vec3f::from_parts(1.0, -2.0, 3.0);
        assert_eq!(a * 3.5, Vec3f::from_parts(3.5, -7.0, 10.5));

        let b = Vec3f::from_parts(1.0, -2.0, 3.0);
        assert_eq!(b * 0.5, Vec3f::from_parts(0.5, -1.0, 1.5));
    }

    #[test]
    fn test_scalar_division() {
        let a = Vec3f::from_parts(1.0, -2.0, 3.0);
        assert_eq!(a / 2.0, Vec3f::from_parts(0.5, -1.0, 1.5));
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Vec3f::from_parts(0.0, 0.0, 0.0).magnitude(), 0.0);
        assert_eq!(Vec3f::from_parts(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Vec3f::from_parts(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Vec3f::from_parts(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(Vec3f::from_parts(1.0, 2.0, 3.0).magnitude(), 14.0f32.sqrt());
        assert_eq!(Vec3f::from_parts(-1.0, -2.0, -3.0).magnitude(), 14.0f32.sqrt());
    }

    #[test]
    fn test_normalization() {
        assert_eq!(Vec3f::from_parts(4.0, 0.0, 0.0).normalize(), Vec3f::from_parts(1.0, 0.0, 0.0));
        assert_eq!(Vec3f::from_parts(1.0, 2.0, 3.0).normalize(), Vec3f::from_parts(0.26726124, 0.5345225, 0.8017837));
        assert_approx_eq!(Vec3f::from_parts(1.0, 2.0, 3.0).normalize().magnitude(), 1.0);
    }

    #[test]
    fn test_dot_product() {
        let a = Vec3f::from_parts(1.0, 2.0, 3.0);
        let b = Vec3f::from_parts(2.0, 3.0, 4.0);
        assert_approx_eq!(a.dot(b), 20.0)
    }

    #[test]
    fn test_cross_product() {
        let a = Vec3f::from_parts(1.0, 2.0, 3.0);
        let b = Vec3f::from_parts(2.0, 3.0, 4.0);
        assert_eq!(Vec3f::cross(a, b), Vec3f::from_parts(-1.0, 2.0, -1.0));
        assert_eq!(Vec3f::cross(b, a), Vec3f::from_parts(1.0, -2.0, 1.0));
    }
}
