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

///
/// A 2x2 Matrix of 32 bit floats.
///
#[derive(Copy, Clone, PartialEq)]
pub struct Mat2f {
    pub m00: f32,
    pub m01: f32,
    pub m10: f32,
    pub m11: f32,
}

impl Mat2f {
    ///
    /// Create 2x2 Matrix from an array of column arrays.
    ///
    /// ```
    /// use render::math::Mat2f;
    ///
    /// let m = Mat2f::from_array_cols(
    ///     [
    ///         [1.0, 2.0],
    ///         [3.0, 4.0],
    ///     ]
    /// );
    /// ```
    ///
    ///                          0  1
    ///                      0 | a, b |
    /// ( a, c ), ( b, d ) = 1 | c, d |
    ///
    pub fn from_array_cols(data: [[f32; 2]; 2]) -> Self {
        Self {
            m00: data[0][0],
            m01: data[1][0],
            m10: data[0][1],
            m11: data[1][1],
        }
    }

    ///
    /// Create 2x2 Matrix from an array of row arrays.
    ///
    /// ```
    /// use render::math::Mat2f;
    ///
    /// let m = Mat2f::from_array_rows(
    ///     [
    ///         [1.0, 2.0],
    ///         [3.0, 4.0],
    ///     ]
    /// );
    /// ```
    ///
    ///                          0  1
    ///                      0 | a, b |
    /// ( a, b ), ( c, d ) = 1 | c, d |
    ///
    pub fn from_array_rows(data: [[f32; 2]; 2]) -> Self {
        Self {
            m00: data[0][0],
            m01: data[0][1],
            m10: data[1][0],
            m11: data[1][1],
        }
    }

    ///
    /// Create 2x2 Matrix from an array of column vectors
    ///
    /// ```
    /// use render::math::{Mat2f, Vec2f};
    ///
    /// let m = Mat2f::from_vec2f_cols(
    ///     [
    ///         Vec2f::from_parts(1.0, 3.0),
    ///         Vec2f::from_parts(2.0, 4.0),
    ///     ]
    /// );
    /// ```
    ///                               0   1
    ///                          0 | x1, x2 |
    /// ( x1, y1 ), ( x2, y2 ) = 1 | y1, y2 |
    ///
    pub fn from_vec2f_cols(data: [Vec2f; 2]) -> Self {
        Self {
            m00: data[0].x,
            m01: data[1].x,
            m10: data[0].y,
            m11: data[1].y,
        }
    }

    ///
    /// Create 2x2 Matrix from an array of row vectors
    ///
    /// ```
    /// use render::math::{Mat2f, Vec2f};
    ///
    /// let m = Mat2f::from_vec2f_rows(
    ///     [
    ///         Vec2f::from_parts(1.0, 2.0),
    ///         Vec2f::from_parts(3.0, 4.0),
    ///     ]
    /// );
    /// ```
    ///                               0   1
    ///                          0 | x1, y1 |
    /// ( x1, y1 ), ( x2, y2 ) = 1 | x2, y2 |
    ///
    pub fn from_vec2f_rows(data: [Vec2f; 2]) -> Self {
        Self {
            m00: data[0].x,
            m01: data[1].x,
            m10: data[0].y,
            m11: data[1].y,
        }
    }

    ///
    /// Create an array of row arrays from Matrix
    ///
    /// ```
    /// use render::math::{Mat2f, Vec2f};
    ///
    /// let a = Mat2f::identity().to_array_rows();
    /// ```
    ///      0   1
    /// 0 | x1, x2 |   ( x1, y1 )
    /// 1 | y1, y2 | = ( x2, y2 )
    ///
    pub fn to_array_rows(&self) -> [[f32; 2]; 2] {
        [
            [self.m00, self.m01],
            [self.m10, self.m11]
        ]
    }

    ///
    /// Create an array of column arrays from the Mat2f
    ///
    /// ```
    /// use render::math::{Mat2f, Vec2f};
    ///
    /// let m = Mat2f::identity().to_array_cols();
    /// ```
    ///      0   1
    /// 0 | x1, x2 |
    /// 1 | y1, y2 | = ( x1, y1 ), ( x2, y2 )
    ///
    pub fn to_array_cols(&self) -> [[f32; 2]; 2] {
        [
            [self.m00, self.m10],
            [self.m01, self.m11]
        ]
    }

    ///
    /// Create an array of Row Vectors
    ///
    /// ```
    /// use render::math::{Mat2f, Vec2f};
    ///
    /// let m = Mat2f::identity().to_vec2f_rows();
    /// ```
    ///
    ///      0   1
    /// 0 | x1, y1 |   ( x1, y1 )
    /// 1 | x2, y2 | = ( x2, y2 )
    ///
    pub fn to_vec2f_rows(&self) -> [Vec2f; 2] {
        [
            Vec2f::from_parts(self.m00, self.m01),
            Vec2f::from_parts(self.m10, self.m11),
        ]
    }

    ///
    /// Create an array of Column Vectors
    ///
    /// ```
    /// use render::math::{Mat2f, Vec2f};
    ///
    /// let m = Mat2f::identity().to_vec2f_cols();
    /// ```
    ///
    ///      0   1
    /// 0 | x1, y1 |   ( x1, y1 )
    /// 1 | x2, y2 | = ( x2, y2 )
    ///
    pub fn to_vec2f_cols(&self) -> [Vec2f; 2] {
        [
            Vec2f::from_parts(self.m00, self.m10),
            Vec2f::from_parts(self.m01, self.m11),
        ]
    }

    ///
    /// Create the 2x2 Identity Matrix
    ///
    /// ```
    /// use render::math::{Mat2f, Vec2f};
    ///
    /// let m = Mat2f::identity();
    /// ```
    ///
    ///       0    1
    /// 0 | 1.0, 0.0 |
    /// 1 | 0.0, 1.0 |
    ///
    pub fn identity() -> Self {
        Self {
            m00: 1.0,
            m01: 0.0,
            m10: 0.0,
            m11: 1.0,
        }
    }
    ///
    /// Calculate the transpose of this matrix
    ///
    /// ```
    /// use render::math::{Mat2f, Vec2f};
    ///
    /// let m = Mat2f::identity().transpose();
    /// ```
    ///
    ///     0  1         0  1
    /// 0 | a, b |   0 | a, c |
    /// 1 | c, d | = 1 | b, d |
    ///
    pub fn transpose(&self) -> Self {
        Self {
            m00: self.m00,
            m01: self.m10,
            m10: self.m01,
            m11: self.m11,
        }
    }
    ///
    /// Calculate the determinant of this Matrix
    ///
    /// ```
    /// use render::math::{Mat2f, Vec2f};
    ///
    /// let m = Mat2f::identity().determinant();
    /// ```
    ///
    ///     0  1
    /// 0 | A, B |
    /// 1 | C, D | = AD - BC
    ///
    pub fn determinant(&self) -> f32 {
        (self.m00 * self.m11) - (self.m01 * self.m10)
    }

    /// Calculate the inverse of this Matrix
    ///
    /// ```
    /// use render::math::{Mat2f, Vec2f};
    ///
    /// let m = Mat2f::identity().invert();
    /// ```
    ///
    ///     0  1                  0             1
    /// 0 | A, B |   | -d / (ad-bc),  b / (ad-bc) |
    /// 1 | C, D | = |  c / (ad-bc), -a / (ad-bc) |
    ///
    pub fn invert(&self) -> Self {
        let det = self.determinant();
        Self {
            m00: self.m11 / det,
            m01: self.m01 / det,
            m10: self.m10 / det,
            m11: self.m00 / det,
        }
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
    fn test_from_rows() {
        let m = Mat2f::from_array_rows(
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
        let a = Mat2f::from_array_rows(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let b = Mat2f::from_array_rows(
            [
                [1.0, 3.0],
                [2.0, 4.0],
            ]
        );
        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn test_determinant() {
        let a = Mat2f::from_array_rows(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        assert_approx_eq!(a.determinant(), -2.0)
    }

    #[test]
    fn test_invert() {
        let a = Mat2f::from_array_rows(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let b = Mat2f::from_array_rows(
            [
                [-2.0, 1.0],
                [1.5, -0.5],
            ]
        );
        assert_eq!(a.invert(), b)
    }

    #[test]
    fn test_partialeq() {
        let a = Mat2f::from_array_rows(
            [
                [1.0 + 1.0, 2.0 + 2.0],
                [1.5 - 0.5, 3.0],
            ]
        );
        let b = Mat2f::from_array_rows(
            [
                [2.0, 4.0],
                [1.0, 1.5 + 1.5],
            ]
        );
        assert_eq!(a, b);
    }

    #[test]
    fn test_add_mat2f() {
        let a = Mat2f::from_array_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        let b = Mat2f::from_array_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array_rows(
            [
                [5.0, 5.0],
                [5.0, 5.0],
            ]
        );
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_addassign_mat2f() {
        let mut a = Mat2f::from_array_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        a += Mat2f::from_array_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array_rows(
            [
                [5.0, 5.0],
                [5.0, 5.0],
            ]
        );
        assert_eq!(a, c);
    }

    #[test]
    fn test_mul_mat2f() {
        let a = Mat2f::from_array_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        let b = Mat2f::from_array_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array_rows(
            [
                [6.0, 7.0],
                [19.0, 18.0],
            ]
        );
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_mulassign_mat2f() {
        let mut a = Mat2f::from_array_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        a *= Mat2f::from_array_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array_rows(
            [
                [6.0, 7.0],
                [19.0, 18.0],
            ]
        );
        assert_eq!(a, c);
    }

    #[test]
    fn test_mul_vec2f() {
        let a = Mat2f::from_array_rows(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let b = Vec2f::from_parts(5.0, 6.0);
        let c = Vec2f::from_parts(17.0, 39.0);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_sub_mat2f() {
        let a = Mat2f::from_array_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        let b = Mat2f::from_array_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array_rows(
            [
                [-3.0, -1.0],
                [3.0, 1.0],
            ]
        );
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_subassign_mat2f() {
        let mut a = Mat2f::from_array_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        a -= Mat2f::from_array_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_array_rows(
            [
                [-3.0, -1.0],
                [3.0, 1.0],
            ]
        );
        assert_eq!(a, c);
    }
}
