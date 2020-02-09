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
use super::{Mat2f, Vec3f};
use std::{fmt, ops};

#[derive(Copy, Clone, PartialEq)]
pub struct Mat3f {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
}

impl Mat3f {
    pub fn from_array(data: [[f32; 3]; 3]) -> Mat3f {
        Mat3f {
            m00: data[0][0],
            m01: data[0][1],
            m02: data[0][2],
            m10: data[1][0],
            m11: data[1][1],
            m12: data[1][2],
            m20: data[2][0],
            m21: data[2][1],
            m22: data[2][2],
        }
    }
    pub fn identity() -> Mat3f {
        Mat3f {
            m00: 1.0,
            m01: 0.0,
            m02: 0.0,
            m10: 0.0,
            m11: 1.0,
            m12: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
        }
    }
    pub fn transpose(&self) -> Self {
        Mat3f {
            m00: self.m00,
            m01: self.m10,
            m02: self.m20,
            m10: self.m01,
            m11: self.m11,
            m12: self.m21,
            m20: self.m02,
            m21: self.m12,
            m22: self.m22,
        }
    }
    fn determinant(&self) -> f32 {
        self.m00 * (self.m11 * self.m22 - self.m21 * self.m12)
            - self.m10 * (self.m01 * self.m22 - self.m21 * self.m02)
            + self.m20 * (self.m01 * self.m12 - self.m11 * self.m02)
    }
    fn invert(&self) -> Option<Mat3f> {
        let det = self.determinant();
        if det == 0.0 {
            None
        } else {
            Some(
                Mat3f::from_cols(
                    self[1].cross(self[2]) / det,
                    self[2].cross(self[0]) / det,
                    self[0].cross(self[1]) / det,
                )                    .transpose(),
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
               self.m00, self.m01, self.m02,
               self.m10, self.m11, self.m12,
               self.m20, self.m21, self.m22,
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
            m00: (self.m00 + rhs.m00),
            m01: (self.m01 + rhs.m01),
            m02: (self.m02 + rhs.m02),
            m10: (self.m10 + rhs.m10),
            m11: (self.m11 + rhs.m11),
            m12: (self.m12 + rhs.m12),
            m20: (self.m20 + rhs.m20),
            m21: (self.m21 + rhs.m21),
            m22: (self.m22 + rhs.m22),
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
            m00: (self.m00 * rhs.m00) + (self.m01 * rhs.m10) + (self.m02 * rhs.m20),
            m01: (self.m00 * rhs.m01) + (self.m01 * rhs.m11) + (self.m02 * rhs.m21),
            m02: (self.m00 * rhs.m02) + (self.m01 * rhs.m12) + (self.m02 * rhs.m22),
            m10: (self.m10 * rhs.m00) + (self.m11 * rhs.m10) + (self.m12 * rhs.m20),
            m11: (self.m10 * rhs.m01) + (self.m11 * rhs.m11) + (self.m12 * rhs.m21),
            m12: (self.m10 * rhs.m02) + (self.m11 * rhs.m12) + (self.m12 * rhs.m22),
            m20: (self.m20 * rhs.m00) + (self.m21 * rhs.m10) + (self.m22 * rhs.m20),
            m21: (self.m20 * rhs.m01) + (self.m21 * rhs.m11) + (self.m22 * rhs.m21),
            m22: (self.m20 * rhs.m02) + (self.m21 * rhs.m12) + (self.m22 * rhs.m22),
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
            x: (self.m00 * rhs.x) + (self.m01 * rhs.y) + (self.m02 * rhs.z),
            y: (self.m10 * rhs.x) + (self.m11 * rhs.y) + (self.m12 * rhs.z),
            z: (self.m20 * rhs.x) + (self.m21 * rhs.y) + (self.m22 * rhs.z),
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
            m00: (self.m00 - rhs.m00),
            m01: (self.m01 - rhs.m01),
            m02: (self.m02 - rhs.m02),
            m10: (self.m10 - rhs.m10),
            m11: (self.m11 - rhs.m11),
            m12: (self.m12 - rhs.m12),
            m20: (self.m20 - rhs.m20),
            m21: (self.m21 - rhs.m21),
            m22: (self.m22 - rhs.m22),
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
    use super::{Mat2f, Mat3f, Vec3f};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_creation() {
        let m = Mat3f::from_array(
            [
                [-3.0, 5.0, 0.0],
                [1.0, -2.0, -7.0],
                [0.0, 1.0, 1.0],
            ]
        );
        assert_approx_eq!(m.m00, -3.0);
        assert_approx_eq!(m.m11, -2.0);
        assert_approx_eq!(m.m22, 1.0);
    }

    #[test]
    fn test_transpose() {
        let a = Mat3f::from_array(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        let b = Mat2f::from_array(
            [
                [-3.0, 02.0],
                [00.0, 06.0],
            ]
        );
        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn test_submatrix() {
        let a = Mat3f::from_array(
            [
                [01.0, 05.0, 00.0],
                [-3.0, 02.0, 07.0],
                [00.0, 06.0, -3.0],
            ]
        );
        let b = Mat2f::from_array(
            [
                [-6.0, 01.0, 06.0],
                [-8.0, 05.0, 06.0],
                [-7.0, 01.0, 01.0],
            ]
        );
        assert_eq!(a.submatrix(2, 1), b);
    }

    #[test]
    fn test_partialeq() {
        let a = Mat3f::from_array(
            [
                [1.0 + 1.0, 2.0 + 2.0, 3.0 + 1.5],
                [1.5 - 0.5, 3.0, 2.25 + 1.10],
                [3.0 / 2.0, 4.0 * 2.0, 4.5],
            ]
        );
        let b = Mat3f::from_array(
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
        let a = Mat3f::from_array(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        let b = Mat3f::from_array(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_array(
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
        let mut a = Mat3f::from_array(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        a += Mat3f::from_array(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_array(
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
        let a = Mat3f::from_array(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        let b = Mat3f::from_array(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_array(
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
        let mut a = Mat3f::from_array(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        a *= Mat3f::from_array(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_array(
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
        let a = Mat3f::from_array(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        let b = Vec3f::new(10.0, 11.0, 12.0);
        let c = Vec3f::new(68.0, 167.0, 266.0);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_sub_mat3f() {
        let a = Mat3f::from_array(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        let b = Mat3f::from_array(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_array(
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
        let mut a = Mat3f::from_array(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        a -= Mat3f::from_array(
            [
                [9.0, 8.0, 7.0],
                [6.0, 5.0, 4.0],
                [3.0, 2.0, 1.0],
            ]
        );
        let c = Mat3f::from_array(
            [
                [-8.0, -6.0, -4.0],
                [-2.0, 0.0, 2.0],
                [4.0, 6.0, 8.0],
            ]
        );
        assert_eq!(a, c);
    }
}
