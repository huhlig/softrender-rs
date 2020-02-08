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

use super::Vec2f;
use std::{fmt, ops};

#[derive(Copy, Clone, PartialEq)]
pub struct Mat2f {
    pub m00: f32,
    pub m01: f32,
    pub m10: f32,
    pub m11: f32,
}

impl Mat2f {
    pub fn from_array(data: [[f32; 2]; 2]) -> Self {
        Self {
            m00: data[0][0],
            m01: data[0][1],
            m10: data[1][0],
            m11: data[1][1],
        }
    }
    pub fn identity() -> Self {
        Self {
            m00: 1.0,
            m01: 0.0,
            m10: 0.0,
            m11: 1.0,
        }
    }
    pub fn transpose(&self) -> Self {
        Self {
            m00: self.m00,
            m01: self.m10,
            m10: self.m01,
            m11: self.m11,
        }
    }
    pub fn determinant(&self) -> f32 {
        //     0  1
        // 0 | A, B |
        // 1 | C, D | = AD - BC
        (self.m00 * self.m11) - (self.m01 * self.m10)
    }
}

impl Default for Mat2f {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Debug for Mat2f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n[ {}, {} ]\n[ {}, {} ]\n",
               self.m00, self.m01,
               self.m10, self.m11,
        )
    }
}

impl ops::Add<Self> for Mat2f {
    type Output = Self;

    ///     0  1       0  1           0      1
    /// 0 | A, B |   | a, b |   | A + a, B + b |
    /// 1 | C, D | + | c, D | = | C + c, D + d |
    fn add(self, rhs: Self) -> Self {
        Self {
            m00: self.m00 + rhs.m00,
            m01: self.m01 + rhs.m01,
            m10: self.m10 + rhs.m10,
            m11: self.m11 + rhs.m11,
        }
    }
}

impl ops::AddAssign<Self> for Mat2f {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Mul<Self> for Mat2f {
    type Output = Self;

    ///     0  1       0  1             0        1
    /// 0 | A, B |   | a, b |   | Aa + Bc, Ab + Bd |
    /// 1 | C, D | x | c, d | = | Ca + Dc, Cb + Dd |
    fn mul(self, rhs: Self) -> Self {
        Self {
            m00: (self.m00 * rhs.m00) + (self.m01 * rhs.m10),
            m01: (self.m00 * rhs.m01) + (self.m01 * rhs.m11),
            m10: (self.m10 * rhs.m00) + (self.m11 * rhs.m10),
            m11: (self.m10 * rhs.m01) + (self.m11 * rhs.m11),
        }
    }
}

impl ops::MulAssign<Self> for Mat2f {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Vec2f> for Mat2f {
    type Output = Vec2f;

    ///     0  1       0  1          0
    /// 0 | A, B |   | x |   | Ax + By |
    /// 1 | C, D | x | y | = | Cx + Dy |
    fn mul(self, rhs: Vec2f) -> Vec2f {
        Vec2f {
            x: (self.m00 * rhs.x) + (self.m01 * rhs.y),
            y: (self.m10 * rhs.x) + (self.m11 * rhs.y),
        }
    }
}

impl ops::Sub<Self> for Mat2f {
    type Output = Self;

    ///     0  1       0  1           0      1
    /// 0 | A, B |   | a, b |   | A - a, B - b |
    /// 1 | C, D | - | c, d | = | C - c, D - d |
    fn sub(self, rhs: Self) -> Self {
        Self {
            m00: self.m00 - rhs.m00,
            m01: self.m01 - rhs.m01,
            m10: self.m10 - rhs.m10,
            m11: self.m11 - rhs.m11,
        }
    }
}

impl ops::SubAssign<Self> for Mat2f {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}


#[cfg(test)]
mod tests {
    use super::{Mat2f, Vec2f};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_creation() {
        let m = Mat2f::from_array(
            [
                [-3.0, 5.0],
                [1.0, -2.0],
            ]
        );
        assert_approx_eq!(m.m00, -3.0);
        assert_approx_eq!(m.m01, 5.0);
        assert_approx_eq!(m.m10, 1.0);
        assert_approx_eq!(m.m11, -2.0);
    }

    #[test]
    fn test_transpose() {
        let a = Mat2f::from_array(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let b = Mat2f::from_array(
            [
                [1.0, 3.0],
                [2.0, 4.0],
            ]
        );
        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn test_partialeq() {
        let a = Mat2f::from_array(
            [
                [1.0 + 1.0, 2.0 + 2.0],
                [1.5 - 0.5, 3.0],
            ]
        );
        let b = Mat2f::from_array(
            [
                [2.0, 4.0],
                [1.0, 1.5 + 1.5],
            ]
        );
        assert_eq!(a, b);
    }

    #[test]
    fn test_add_mat2f() {
        let a = Mat2f::from_array(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        let b = Mat2f::from_array(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array(
            [
                [5.0, 5.0],
                [5.0, 5.0],
            ]
        );
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_addassign_mat2f() {
        let mut a = Mat2f::from_array(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        a += Mat2f::from_array(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array(
            [
                [5.0, 5.0],
                [5.0, 5.0],
            ]
        );
        assert_eq!(a, c);
    }

    #[test]
    fn test_mul_mat2f() {
        let a = Mat2f::from_array(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        let b = Mat2f::from_array(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array(
            [
                [6.0, 7.0],
                [19.0, 18.0],
            ]
        );
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_mulassign_mat2f() {
        let mut a = Mat2f::from_array(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        a *= Mat2f::from_array(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array(
            [
                [6.0, 7.0],
                [19.0, 18.0],
            ]
        );
        assert_eq!(a, c);
    }

    #[test]
    fn test_mul_vec2f() {
        let a = Mat2f::from_array(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let b = Vec2f::new(5.0, 6.0);
        let c = Vec2f::new(17.0, 39.0);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_sub_mat2f() {
        let a = Mat2f::from_array(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        let b = Mat2f::from_array(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array(
            [
                [-3.0, -1.0],
                [3.0, 1.0],
            ]
        );
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_subassign_mat2f() {
        let mut a = Mat2f::from_array(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        a -= Mat2f::from_array(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array(
            [
                [-3.0, -1.0],
                [3.0, 1.0],
            ]
        );
        assert_eq!(a, c);
    }
}
