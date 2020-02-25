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
/// A Column Major 2x2 Matrix of 32 bit floats.
///
#[derive(Copy, Clone, PartialEq)]
pub struct Mat2f {
    pub c0r0: f32,
    pub c0r1: f32,
    pub c1r0: f32,
    pub c1r1: f32,
}

impl Mat2f {
    ///
    /// Create Matrix from Rows
    /// ```
    /// use softrender::math::Mat2f;
    ///
    /// let m = Mat2f::from_rows(
    ///     [
    ///         [1.0, 2.0],
    ///         [3.0, 4.0],
    ///     ]
    /// );
    /// ```
    ///
    ///     x  y         0  1
    /// 0 ( a, b )    | a, b | 0
    /// 1 ( c, d )  = | c, d | 1
    ///
    pub fn from_rows(rows: [[f32; 2]; 2]) -> Mat2f {
        Self {
            c0r0: rows[0][0],
            c1r0: rows[0][1],
            c0r1: rows[1][0],
            c1r1: rows[1][1],
        }
    }

    ///
    /// Create Matrix from Columns
    /// ```
    /// use softrender::math::Mat2f;
    ///
    /// let m = Mat2f::from_cols(
    ///     [
    ///         [1.0, 3.0],
    ///         [2.0, 4.0],
    ///     ]
    /// );
    /// ```
    ///
    ///     x  y         0  1
    /// 0 ( a, c )    | a, b | 0
    /// 1 ( b, d )  = | c, d | 1
    ///
    pub fn from_cols(cols: [[f32; 2]; 2]) -> Mat2f {
        Self {
            c0r0: cols[0][0],
            c0r1: cols[0][1],
            c1r0: cols[1][0],
            c1r1: cols[1][1],
        }
    }

    ///
    /// Create a 2x2 Zero Matrix
    ///
    /// ```
    /// use softrender::math::Mat2f;
    ///
    /// let m = Mat2f::identity();
    /// ```
    ///
    ///       0    1
    /// 0 | 0.0, 0.0 |
    /// 1 | 0.0, 0.0 |
    ///
    pub fn zero() -> Self {
        Self {
            c0r0: 1.0,
            c0r1: 0.0,
            c1r0: 0.0,
            c1r1: 1.0,
        }
    }

    ///
    /// Create a 2x2 Identity Matrix
    ///
    /// ```
    /// use softrender::math::Mat2f;
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
            c0r0: 1.0,
            c0r1: 0.0,
            c1r0: 0.0,
            c1r1: 1.0,
        }
    }

    ///
    /// Get Rows
    ///
    /// ```
    /// use softrender::math::Mat2f;
    ///
    /// let m = Mat2f::identity().to_rows();
    /// ```
    ///
    ///     0  1          x  y
    /// 0 | a, b |    0 ( a, b )
    /// 1 | c, d | => 1 ( c, d )
    ///
    pub fn to_rows(&self) -> [[f32; 2]; 2] {
        [
            [self.c0r0, self.c1r0],
            [self.c0r1, self.c1r1],
        ]
    }

    ///
    /// Get Columns
    ///
    /// ```
    /// use softrender::math::Mat2f;
    ///
    /// let m = Mat2f::identity().to_cols();
    /// ```
    ///
    ///     0  1          x  y
    /// 0 | a, b |    0 ( a, c )
    /// 1 | c, d | => 1 ( b, d )
    ///
    pub fn to_cols(&self) -> [[f32; 2]; 2] {
        [
            [self.c0r0, self.c0r1],
            [self.c1r0, self.c1r1],
        ]
    }

    ///
    /// Calculate the transpose of this matrix
    ///
    /// ```
    /// use softrender::math::Mat2f;
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
            c0r0: self.c0r0,
            c0r1: self.c1r0,
            c1r0: self.c0r1,
            c1r1: self.c1r1,
        }
    }

    ///
    /// Calculate the determinant of this Matrix
    ///
    /// ```
    /// use softrender::math::Mat2f;
    ///
    /// let m = Mat2f::identity().determinant();
    /// ```
    ///
    ///     0  1
    /// 0 | A, B |
    /// 1 | C, D | = AD - BC
    ///
    pub fn determinant(&self) -> f32 {
        (self.c0r0 * self.c1r1) - (self.c0r1 * self.c1r0)
    }

    /// Calculate the inverse of this Matrix
    ///
    /// ```
    /// use softrender::math::Mat2f;
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
            c0r0: -self.c1r1 / det, c1r0:  self.c1r0 / det,
            c0r1:  self.c0r1 / det, c1r1: -self.c0r0 / det,
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
               self.c0r0, self.c1r0,
               self.c0r1, self.c1r1,
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
            c0r0: self.c0r0 + rhs.c0r0,
            c0r1: self.c0r1 + rhs.c0r1,
            c1r0: self.c1r0 + rhs.c1r0,
            c1r1: self.c1r1 + rhs.c1r1,
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
            c0r0: (self.c0r0 * rhs.c0r0) + (self.c0r1 * rhs.c1r0),
            c0r1: (self.c0r0 * rhs.c0r1) + (self.c0r1 * rhs.c1r1),
            c1r0: (self.c1r0 * rhs.c0r0) + (self.c1r1 * rhs.c1r0),
            c1r1: (self.c1r0 * rhs.c0r1) + (self.c1r1 * rhs.c1r1),
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
            x: (self.c0r0 * rhs.x) + (self.c0r1 * rhs.y),
            y: (self.c1r0 * rhs.x) + (self.c1r1 * rhs.y),
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
            c0r0: self.c0r0 - rhs.c0r0,
            c0r1: self.c0r1 - rhs.c0r1,
            c1r0: self.c1r0 - rhs.c1r0,
            c1r1: self.c1r1 - rhs.c1r1,
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
        let m = Mat2f::from_rows(
            [
                [-1.0, 2.0],
                [-3.0, 4.0],
            ]
        );
        assert_approx_eq!(m.c0r0, -1.0);
        assert_approx_eq!(m.c1r0, 2.0);
        assert_approx_eq!(m.c0r1, -3.0);
        assert_approx_eq!(m.c1r1, 4.0);
    }

    #[test]
    fn test_from_cols() {
        let m = Mat2f::from_cols(
            [
                [-1.0, 3.0],
                [-2.0, 4.0],
            ]
        );
        assert_approx_eq!(m.c0r0, -1.0);
        assert_approx_eq!(m.c0r1, 3.0);
        assert_approx_eq!(m.c1r0, -2.0);
        assert_approx_eq!(m.c1r1, 4.0);
    }

    #[test]
    fn test_zero() {
        let m = Mat2f::zero();

        assert_approx_eq!(m.c0r0, 0.0);
        assert_approx_eq!(m.c0r1, 0.0);
        assert_approx_eq!(m.c1r0, 0.0);
        assert_approx_eq!(m.c1r1, 0.0);
    }

    #[test]
    fn test_identity() {
        let m = Mat2f::identity();

        assert_approx_eq!(m.c0r0, 1.0);
        assert_approx_eq!(m.c0r1, 0.0);
        assert_approx_eq!(m.c1r0, 0.0);
        assert_approx_eq!(m.c1r1, 1.0);
    }

    #[test]
    fn test_to_rows() {
        let a = [
            [-1.0, 2.0],
            [-3.0, 4.0],
        ];
        let m = Mat2f::from_rows(a.clone());
        let b = m.to_rows();

        assert_eq!(a, b);
    }

    #[test]
    fn test_to_cols() {
        let a = [
            [-1.0, 2.0],
            [-3.0, 4.0],
        ];
        let m = Mat2f::from_cols(a.clone());
        let b = m.to_cols();

        assert_eq!(a, b);
    }

    #[test]
    fn test_transpose() {
        let a = Mat2f::from_rows(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let b = Mat2f::from_rows(
            [
                [1.0, 3.0],
                [2.0, 4.0],
            ]
        );
        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn test_determinant() {
        let a = Mat2f::from_rows(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        assert_approx_eq!(a.determinant(), -2.0)
    }

    #[test]
    fn test_invert() {
        let a = Mat2f::from_rows(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let b = Mat2f::from_rows(
            [
                [-2.0,  1.0],
                [ 1.5, -0.5],
            ]
        );
        assert_eq!(a.invert(), b)
    }

    #[test]
    fn test_default() {
        assert_eq!(Mat2f::default(), Mat2f::default());
    }

    #[test]
    fn test_partialeq() {
        let a = Mat2f::from_rows(
            [
                [1.0 + 1.0, 2.0 + 2.0],
                [1.5 - 0.5, 3.0],
            ]
        );
        let b = Mat2f::from_rows(
            [
                [2.0, 4.0],
                [1.0, 1.5 + 1.5],
            ]
        );
        assert_eq!(a, b);
    }

    #[test]
    fn test_add_mat2f() {
        let a = Mat2f::from_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        let b = Mat2f::from_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_rows(
            [
                [5.0, 5.0],
                [5.0, 5.0],
            ]
        );
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_addassign_mat2f() {
        let mut a = Mat2f::from_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        a += Mat2f::from_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_rows(
            [
                [5.0, 5.0],
                [5.0, 5.0],
            ]
        );
        assert_eq!(a, c);
    }

    #[test]
    fn test_mul_mat2f() {
        let a = Mat2f::from_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        let b = Mat2f::from_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_rows(
            [
                [6.0, 7.0],
                [19.0, 18.0],
            ]
        );
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_mulassign_mat2f() {
        let mut a = Mat2f::from_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        a *= Mat2f::from_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_rows(
            [
                [6.0, 7.0],
                [19.0, 18.0],
            ]
        );
        assert_eq!(a, c);
    }

    #[test]
    fn test_mul_vec2f() {
        let a = Mat2f::from_rows(
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
        let a = Mat2f::from_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        let b = Mat2f::from_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_rows(
            [
                [-3.0, -1.0],
                [3.0, 1.0],
            ]
        );
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_subassign_mat2f() {
        let mut a = Mat2f::from_rows(
            [
                [1.0, 2.0],
                [4.0, 3.0],
            ]
        );
        a -= Mat2f::from_rows(
            [
                [4.0, 3.0],
                [1.0, 2.0],
            ]
        );
        let c = Mat2f::from_rows(
            [
                [-3.0, -1.0],
                [3.0, 1.0],
            ]
        );
        assert_eq!(a, c);
    }
}
