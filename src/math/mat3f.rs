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
use super::Vec3f;
use std::{fmt, ops};

///
/// A 3x3 Matrix of 32 bit floats.
///
#[derive(Copy, Clone, PartialEq)]
pub struct Mat3f {
    pub c0r0: f32,
    pub c0r1: f32,
    pub c0r2: f32,
    pub c1r0: f32,
    pub c1r1: f32,
    pub c1r2: f32,
    pub c2r0: f32,
    pub c2r1: f32,
    pub c2r2: f32,
}

impl Mat3f {
    ///
    /// Create 3x3 Matrix from an array of column arrays.
    ///
    /// ```
    /// use softrender::math::Mat3f;
    ///
    /// let m = Mat3f::from_cols(
    ///     [
    ///         [1.0, 2.0, 3.0],
    ///         [4.0, 5.0, 6.0],
    ///         [4.0, 5.0, 6.0],
    ///     ]
    /// );
    /// ```
    ///
    ///     x  y  z          0  1  2
    /// 0 ( a, d, g )    0 | a, b, c |
    /// 1 ( b, e, h )    1 | d, e, f |
    /// 2 ( c, f, i ) -> 2 | g, h, i |
    ///
    pub fn from_cols(cols: [[f32; 3]; 3]) -> Mat3f {
        Mat3f {
            c0r0: cols[0][0],
            c0r1: cols[1][0],
            c0r2: cols[2][0],
            c1r0: cols[0][1],
            c1r1: cols[1][1],
            c1r2: cols[2][1],
            c2r0: cols[0][2],
            c2r1: cols[1][2],
            c2r2: cols[2][2],
        }
    }

    ///
    /// Create 3x3 Matrix from an array of row arrays.
    ///
    /// ```
    /// use softrender::math::Mat3f;
    ///
    /// let m = Mat3f::from_rows(
    ///     [
    ///         [1.0, 2.0, 3.0],
    ///         [4.0, 5.0, 6.0],
    ///         [4.0, 5.0, 6.0],
    ///     ]
    /// );
    /// ```
    ///
    ///     x  y  z        0  1  2
    /// 0 ( a, b, c )    | a, b, c | 0
    /// 1 ( d, e, f )    | d, e, f | 1
    /// 2 ( g, h, i ) -> | g, h, i | 2
    ///
    pub fn from_rows(rows: [[f32; 3]; 3]) -> Mat3f {
        Mat3f {
            c0r0: rows[0][0],
            c0r1: rows[0][1],
            c0r2: rows[0][2],
            c1r0: rows[1][0],
            c1r1: rows[1][1],
            c1r2: rows[1][2],
            c2r0: rows[2][0],
            c2r1: rows[2][1],
            c2r2: rows[2][2],
        }
    }

    ///
    /// Create an array of rows from a 3x3 Matrix.
    ///
    /// ```
    /// use softrender::math::Mat3f;
    ///
    /// let m = Mat3f::identity().to_rows();
    /// ```
    ///
    ///   0  1  2            x  y  z
    /// 0 | a, b, c |    0 ( a, b, c )
    /// 1 | d, e, f |    1 ( d, e, f )
    /// 2 | g, h, i | -> 2 ( g, h, i )
    ///
    pub fn to_rows(&self) -> [[f32; 3]; 3] {
        [
            [self.c0r0, self.c0r1, self.c0r2],
            [self.c1r0, self.c1r1, self.c1r2],
            [self.c2r0, self.c2r1, self.c2r2],
        ]
    }

    ///
    /// Create an array of columns from a 3x3 Matrix.
    ///
    /// ```
    /// use softrender::math::Mat3f;
    ///
    /// let m = Mat3f::identity().to_cols();
    /// ```
    ///
    ///     0  1  2          x  y  z
    /// 0 | a, b, c |    0 ( a, d, g )
    /// 1 | d, e, f |    1 ( b, e, h )
    /// 2 | g, h, i | -> 2 ( c, f, i )
    ///
    pub fn cols(&self) -> [[f32; 3]; 3] {
        [
            [self.c0r0, self.c1r0, self.c2r0],
            [self.c0r1, self.c1r1, self.c2r1],
            [self.c0r2, self.c1r2, self.c2r2],
        ]
    }

    ///
    /// Create a 3x3 Zero Matrix.
    ///
    /// ```
    /// use softrender::math::Mat3f;
    ///
    /// let m = Mat3f::zero();
    /// ```
    ///
    ///       0    1    2
    /// 0 | 0.0, 0.0, 0.0 |
    /// 1 | 0.0, 0.0, 0.0 |
    /// 2 | 0.0, 0.0, 0.0 |
    ///
    pub fn zero() -> Mat3f {
        Mat3f {
            c0r0: 0.0,
            c0r1: 0.0,
            c0r2: 0.0,
            c1r0: 0.0,
            c1r1: 0.0,
            c1r2: 0.0,
            c2r0: 0.0,
            c2r1: 0.0,
            c2r2: 0.0,
        }
    }

    ///
    /// Create a 3x3 Identity Matrix.
    ///
    /// ```
    /// use softrender::math::Mat3f;
    ///
    /// let m = Mat3f::identity();
    /// ```
    ///
    ///       0    1    2
    /// 0 | 1.0, 0.0, 0.0 |
    /// 1 | 0.0, 1.0, 0.0 |
    /// 2 | 0.0, 0.0, 1.0 |
    ///
    pub fn identity() -> Mat3f {
        Mat3f {
            c0r0: 1.0,
            c0r1: 0.0,
            c0r2: 0.0,
            c1r0: 0.0,
            c1r1: 1.0,
            c1r2: 0.0,
            c2r0: 0.0,
            c2r1: 0.0,
            c2r2: 1.0,
        }
    }

    ///
    /// Calculate the transpose of this matrix.
    ///
    /// ```
    /// use softrender::math::Mat3f;
    ///
    /// let m = Mat3f::identity().transpose();
    /// ```
    ///
    ///     0  1  2          0  1  2
    /// 0 | a, b, c |    0 | a, d, g |
    /// 1 | d, e, f |    1 | b, e, h |
    /// 2 | g, h, i | -> 2 | c, f, i |
    ///
    pub fn transpose(&self) -> Self {
        Self {
            c0r0: self.c0r0,
            c0r1: self.c1r0,
            c0r2: self.c2r0,
            c1r0: self.c0r1,
            c1r1: self.c1r1,
            c1r2: self.c2r1,
            c2r0: self.c0r2,
            c2r1: self.c1r2,
            c2r2: self.c2r2,
        }
    }

    ///
    /// Calculate the determinant of this Matrix
    ///
    /// ```
    /// use softrender::math::Mat3f;
    ///
    /// let m = Mat3f::identity().determinant();
    /// ```
    ///
    pub fn determinant(&self) -> f32 {
        let b01 = self.c0r0 * (self.c1r1 * self.c2r2 - self.c2r1 * self.c1r2);
        let b02 = self.c1r0 * (self.c0r1 * self.c2r2 - self.c2r1 * self.c0r2);
        let b03 = self.c2r0 * (self.c0r1 * self.c1r2 - self.c1r1 * self.c0r2);
        b01 - b02 + b03
    }
    ///
    /// Calculate the inversion of this Matrix
    ///
    pub fn invert(&self) -> Option<Mat3f> {
        let det = self.determinant();
        if det == 0.0 {
            None
        } else {
            Some(
                Self {
                    c0r0: (self.c1r1 * self.c2r2 - self.c1r2 * self.c2r1) / det,
                    c0r1: -(self.c0r1 * self.c2r2 - self.c0r2 * self.c2r1) / det,
                    c0r2: (self.c0r1 * self.c1r2 - self.c0r2 * self.c1r1) / det,
                    c1r0: -(-self.c2r0 * self.c1r2 + self.c1r0 * self.c2r2) / det,
                    c1r1: (-self.c2r0 * self.c0r2 + self.c0r0 * self.c2r2) / det,
                    c1r2: -(-self.c1r0 * self.c0r2 + self.c0r0 * self.c1r2) / det,
                    c2r0: (-self.c2r0 * self.c1r1 + self.c1r0 * self.c2r1) / det,
                    c2r1: -(-self.c2r0 * self.c0r1 + self.c0r0 * self.c2r1) / det,
                    c2r2: (-self.c1r0 * self.c0r2 + self.c0r0 * self.c1r1) / det,
                }
            )
        }
    }
}

impl Default for Mat3f {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Debug for Mat3f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n[ {}, {}, {} ]\n[ {}, {}, {} ]\n[ {}, {}, {} ]\n",
               self.c0r0, self.c0r1, self.c0r2,
               self.c1r0, self.c1r1, self.c1r2,
               self.c2r0, self.c2r1, self.c2r2,
        )
    }
}

impl ops::Add<Self> for Mat3f {
    type Output = Self;

    ///     0  1  2       0  1  2           0      1      2
    /// 0 | A, B, C |   | a, b, c |   | A + a, B + b, C + c |
    /// 1 | D, E, F | + | d, e, f | = | D + d, E + e, F + f |
    /// 2 | G, H, I |   | G, H, I |   | G + g, H + h, I + i |
    fn add(self, rhs: Self) -> Self {
        Self {
            c0r0: (self.c0r0 + rhs.c0r0),
            c0r1: (self.c0r1 + rhs.c0r1),
            c0r2: (self.c0r2 + rhs.c0r2),
            c1r0: (self.c1r0 + rhs.c1r0),
            c1r1: (self.c1r1 + rhs.c1r1),
            c1r2: (self.c1r2 + rhs.c1r2),
            c2r0: (self.c2r0 + rhs.c2r0),
            c2r1: (self.c2r1 + rhs.c2r1),
            c2r2: (self.c2r2 + rhs.c2r2),
        }
    }
}

impl ops::AddAssign<Self> for Mat3f {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Mul<Self> for Mat3f {
    type Output = Self;

    ///     0  1  2       0  1  2                  0             1             2             3
    /// 0 | A, B, C |   | a, b, c |   | Aa + Be + Ci, Ab + Bf + Cj, Ac + Bg + Ck, Ad + Bh + Cl |
    /// 1 | E, F, G | x | e, f, g | = | Ea + Fe + Gi, Eb + Ff + Gj, Ec + Fg + Gk, Ed + Fh + Gl |
    /// 2 | I, J, K |   | i, j, k |   | Ia + Je + Ki, Ib + Jf + Kj, Ic + Jg + Kk, Id + Jh + Kl |
    fn mul(self, rhs: Self) -> Self {
        Self {
            c0r0: (self.c0r0 * rhs.c0r0) + (self.c0r1 * rhs.c1r0) + (self.c0r2 * rhs.c2r0),
            c0r1: (self.c0r0 * rhs.c0r1) + (self.c0r1 * rhs.c1r1) + (self.c0r2 * rhs.c2r1),
            c0r2: (self.c0r0 * rhs.c0r2) + (self.c0r1 * rhs.c1r2) + (self.c0r2 * rhs.c2r2),
            c1r0: (self.c1r0 * rhs.c0r0) + (self.c1r1 * rhs.c1r0) + (self.c1r2 * rhs.c2r0),
            c1r1: (self.c1r0 * rhs.c0r1) + (self.c1r1 * rhs.c1r1) + (self.c1r2 * rhs.c2r1),
            c1r2: (self.c1r0 * rhs.c0r2) + (self.c1r1 * rhs.c1r2) + (self.c1r2 * rhs.c2r2),
            c2r0: (self.c2r0 * rhs.c0r0) + (self.c2r1 * rhs.c1r0) + (self.c2r2 * rhs.c2r0),
            c2r1: (self.c2r0 * rhs.c0r1) + (self.c2r1 * rhs.c1r1) + (self.c2r2 * rhs.c2r1),
            c2r2: (self.c2r0 * rhs.c0r2) + (self.c2r1 * rhs.c1r2) + (self.c2r2 * rhs.c2r2),
        }
    }
}

impl ops::MulAssign<Self> for Mat3f {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Vec3f> for Mat3f {
    type Output = Vec3f;

    ///     0  1  2       0                  0
    /// 0 | A, B, C |   | x |   | Ax + By + Cz |
    /// 1 | D, E, F | x | y | = | Dx + Ey + Fz |
    /// 2 | G, H, I |   | z | = | Gx + Hy + Iz |
    fn mul(self, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: (self.c0r0 * rhs.x) + (self.c0r1 * rhs.y) + (self.c0r2 * rhs.z),
            y: (self.c1r0 * rhs.x) + (self.c1r1 * rhs.y) + (self.c1r2 * rhs.z),
            z: (self.c2r0 * rhs.x) + (self.c2r1 * rhs.y) + (self.c2r2 * rhs.z),
        }
    }
}

impl ops::Sub<Self> for Mat3f {
    type Output = Self;

    ///     0  1  2       0  1  2           0      1      2
    /// 0 | A, B, C |   | a, b, c |   | A - a, B - b, C - c |
    /// 1 | D, E, F | - | d, e, f | = | D - d, E - e, F - f |
    /// 2 | G, H, I |   | G, H, I |   | G - g, H - h, I - i |
    fn sub(self, rhs: Self) -> Self {
        Self {
            c0r0: (self.c0r0 - rhs.c0r0),
            c0r1: (self.c0r1 - rhs.c0r1),
            c0r2: (self.c0r2 - rhs.c0r2),
            c1r0: (self.c1r0 - rhs.c1r0),
            c1r1: (self.c1r1 - rhs.c1r1),
            c1r2: (self.c1r2 - rhs.c1r2),
            c2r0: (self.c2r0 - rhs.c2r0),
            c2r1: (self.c2r1 - rhs.c2r1),
            c2r2: (self.c2r2 - rhs.c2r2),
        }
    }
}

impl ops::SubAssign<Self> for Mat3f {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}


#[cfg(test)]
mod tests {
    use super::{Mat3f, Vec3f};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_rows() {
        let a = [
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0],
        ];
        let m = Mat3f::from_rows(a.clone());
        let b = m.to_rows();
        assert_eq!(a, b);
        assert_approx_eq!(m.c0r0, -3.0);
        assert_approx_eq!(m.c1r1, -2.0);
        assert_approx_eq!(m.c2r2, 1.0);
    }

    #[test]
    fn test_transpose() {
        let a = Mat3f::from_rows(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        let b = Mat3f::from_rows(
            [
                [1.0, 4.0, 7.0],
                [2.0, 5.0, 8.0],
                [3.0, 6.0, 9.0]
            ]
        );
        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn test_determinant() {
        let a = Mat3f::from_rows(
            [
                [1.0, 2.0, 3.0],
                [3.0, 1.0, 2.0],
                [3.0, 2.0, 1.0],
            ]
        );
        assert_approx_eq!(a.determinant(), 8.0)
    }

    #[test]
    fn test_invert() {
        let a = Mat3f::from_rows(
            [
                [1.0, 2.0, 3.0],
                [3.0, 1.0, 2.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let b = Mat3f::from_rows(
            [
                [-0.375, 0.5, 0.125],
                [0.5, -1.0, 0.5],
                [0.125, 0.5, -0.375],
            ]
        );
        assert_eq!(a.invert().unwrap(), b)
    }

    #[test]
    fn test_partialeq() {
        let a = Mat3f::from_rows(
            [
                [1.0 + 1.0, 2.0 + 2.0, 3.0 + 1.5],
                [1.5 - 0.5, 3.0, 2.25 + 1.10],
                [3.0 / 2.0, 4.0 * 2.0, 4.5],
            ]
        );
        let b = Mat3f::from_rows(
            [
                [2.0, 4.0, 4.5],
                [1.0, 3.0, 3.35],
                [1.5, 8.0, 4.500],
            ]
        );
        assert_eq!(a, b);
    }

    #[test]
    fn test_add_mat3f() {
        let a = Mat3f::from_rows(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        let b = Mat3f::from_rows(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_rows(
            [
                [10.0, 10.0, 10.0],
                [10.0, 10.0, 10.0],
                [10.0, 10.0, 10.0],
            ]
        );
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_addassign_mat3f() {
        let mut a = Mat3f::from_rows(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        a += Mat3f::from_rows(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_rows(
            [
                [10.0, 10.0, 10.0],
                [10.0, 10.0, 10.0],
                [10.0, 10.0, 10.0],
            ]
        );
        assert_eq!(a, c);
    }

    #[test]
    fn test_mul_mat3f() {
        let a = Mat3f::from_rows(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        let b = Mat3f::from_rows(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_rows(
            [
                [30.0, 24.0, 18.0],
                [84.0, 69.0, 54.0],
                [138.0, 114.0, 90.0],
            ]
        );
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_mulassign_mat3f() {
        let mut a = Mat3f::from_rows(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        a *= Mat3f::from_rows(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_rows(
            [
                [30.0, 24.0, 18.0],
                [84.0, 69.0, 54.0],
                [138.0, 114.0, 90.0],
            ]
        );
        assert_eq!(a, c);
    }

    #[test]
    fn test_mul_vec3f() {
        let a = Mat3f::from_rows(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        let b = Vec3f::from_parts(10.0, 11.0, 12.0);
        let c = Vec3f::from_parts(68.0, 167.0, 266.0);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_sub_mat3f() {
        let a = Mat3f::from_rows(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        let b = Mat3f::from_rows(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_rows(
            [
                [-8.0, -6.0, -4.0],
                [-2.0, 0.0, 2.0],
                [4.0, 6.0, 8.0],
            ]
        );
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_subassign_mat3f() {
        let mut a = Mat3f::from_rows(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        a -= Mat3f::from_rows(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_rows(
            [
                [-8.0, -6.0, -4.0],
                [-2.0, 0.0, 2.0],
                [4.0, 6.0, 8.0],
            ]
        );
        assert_eq!(a, c);
    }
}
