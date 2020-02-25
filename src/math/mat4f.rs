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
use super::{Vec3f, Vec4f};

///
/// 4x4 Matrix
///
#[derive(Copy, Clone, PartialEq)]
pub struct Mat4f {
    pub c0r0: f32,
    pub c0r1: f32,
    pub c0r2: f32,
    pub c0r3: f32,
    pub c1r0: f32,
    pub c1r1: f32,
    pub c1r2: f32,
    pub c1r3: f32,
    pub c2r0: f32,
    pub c2r1: f32,
    pub c2r2: f32,
    pub c2r3: f32,
    pub c3r0: f32,
    pub c3r1: f32,
    pub c3r2: f32,
    pub c3r3: f32,
}

impl Mat4f {
    ///
    /// Create 4x4 Matrix from an array of column arrays.
    ///
    /// ```
    /// use softrender::math::Mat4f;
    ///
    /// let m = Mat4f::from_cols(
    ///     [
    ///         [ 1.0,  2.0,  3.0,  4.0],
    ///         [ 5.0,  6.0,  7.0,  8.0],
    ///         [ 9.0, 10.0, 11.0, 12.0],
    ///         [13.0, 14.0, 15.0, 16.0],
    ///     ]
    /// );
    /// ```
    ///
    ///                       0  1  2  3
    /// ( a, e, i, m )    0 | a, b, c, d |
    /// ( b, f, j, n )    1 | e, f, g, h |
    /// ( c, g, k, o )    2 | i, j, k, l |
    /// ( d, h, l, p )  = 3 | m, n, o, p |
    ///
    pub fn from_cols(data: [[f32; 4]; 4]) -> Mat4f {
        Mat4f {
            c0r0: data[0][0],
            c0r1: data[1][0],
            c0r2: data[2][0],
            c0r3: data[3][0],
            c1r0: data[0][1],
            c1r1: data[1][1],
            c1r2: data[2][1],
            c1r3: data[3][1],
            c2r0: data[0][2],
            c2r1: data[1][2],
            c2r2: data[2][2],
            c2r3: data[3][2],
            c3r0: data[0][3],
            c3r1: data[1][3],
            c3r2: data[2][3],
            c3r3: data[3][3],
        }
    }

    ///
    /// Create 4x4 Matrix from an array of row arrays.
    ///
    /// ```
    /// use softrender::math::Mat4f;
    ///
    /// let m = Mat4f::from_rows(
    ///     [
    ///         [ 1.0,  5.0,  9.0, 13.0],
    ///         [ 2.0,  6.0, 10.0, 14.0],
    ///         [ 3.0,  7.0, 11.0, 15.0],
    ///         [ 4.0,  8.0, 12.0, 16.0],
    ///     ]
    /// );
    /// ```
    ///
    ///   x  y  z  w          0  1  2  3
    /// ( a, b, c, d )    0 | a, b, c, d |
    /// ( e, f, g, h )    1 | e, f, g, h |
    /// ( i, j, k, l )    2 | i, j, k, l |
    /// ( m, n, o, p )  = 3 | m, n, o, p |
    ///
    pub fn from_rows(data: [[f32; 4]; 4]) -> Mat4f {
        Mat4f {
            c0r0: data[0][0],
            c0r1: data[0][1],
            c0r2: data[0][2],
            c0r3: data[0][3],
            c1r0: data[1][0],
            c1r1: data[1][1],
            c1r2: data[1][2],
            c1r3: data[1][3],
            c2r0: data[2][0],
            c2r1: data[2][1],
            c2r2: data[2][2],
            c2r3: data[2][3],
            c3r0: data[3][0],
            c3r1: data[3][1],
            c3r2: data[3][2],
            c3r3: data[3][3],
        }
    }
    /// Create 4x4 Zero Matrix.
    ///
    /// ```
    /// use softrender::math::Mat4f;
    ///
    /// let m = Mat4f::zero();
    /// ```
    ///
    ///       0    1    2    3
    /// 0 | 0.0, 0.0, 0.0, 0.0 |
    /// 1 | 0.0, 0.0, 0.0, 0.0 |
    /// 2 | 0.0, 0.0, 0.0, 0.0 |
    /// 3 | 0.0, 0.0, 0.0, 0.0 |
    ///
    pub fn zero() -> Mat4f {
        Mat4f {
            c0r0: 0.0,
            c0r1: 0.0,
            c0r2: 0.0,
            c0r3: 0.0,
            c1r0: 0.0,
            c1r1: 0.0,
            c1r2: 0.0,
            c1r3: 0.0,
            c2r0: 0.0,
            c2r1: 0.0,
            c2r2: 0.0,
            c2r3: 0.0,
            c3r0: 0.0,
            c3r1: 0.0,
            c3r2: 0.0,
            c3r3: 0.0,
        }
    }

    /// Create 4x4 Identity Matrix.
    ///
    /// ```
    /// use softrender::math::Mat4f;
    ///
    /// let m = Mat4f::identity();
    /// ```
    ///
    ///       0    1    2    3
    /// 0 | 1.0, 0.0, 0.0, 0.0 |
    /// 1 | 0.0, 1.0, 0.0, 0.0 |
    /// 2 | 0.0, 0.0, 1.0, 0.0 |
    /// 3 | 0.0, 0.0, 0.0, 1.0 |
    ///
    pub fn identity() -> Mat4f {
        Mat4f {
            c0r0: 1.0,
            c0r1: 0.0,
            c0r2: 0.0,
            c0r3: 0.0,
            c1r0: 0.0,
            c1r1: 1.0,
            c1r2: 0.0,
            c1r3: 0.0,
            c2r0: 0.0,
            c2r1: 0.0,
            c2r2: 1.0,
            c2r3: 0.0,
            c3r0: 0.0,
            c3r1: 0.0,
            c3r2: 0.0,
            c3r3: 1.0,
        }
    }
    ///
    /// Calculate the transpose of this matrix.
    ///
    /// ```
    /// use softrender::math::Mat4f;
    ///
    /// let m = Mat4f::identity().transpose();
    /// ```
    ///
    ///     0  1  2  3          0  1  2  3
    /// 0 | a, b, c, d |    0 | a, e, i, m |
    /// 1 | e, f, g, h |    1 | b, f, j, n |
    /// 2 | i, j, k, l |    2 | c, g, k, o |
    /// 3 | m, n, o, p | -> 3 | d, h, l, p |
    ///
    pub fn transpose(&self) -> Self {
        Self {
            c0r0: self.c0r0,
            c0r1: self.c1r0,
            c0r2: self.c2r0,
            c0r3: self.c3r0,
            c1r0: self.c0r1,
            c1r1: self.c1r1,
            c1r2: self.c2r1,
            c1r3: self.c3r1,
            c2r0: self.c0r2,
            c2r1: self.c1r2,
            c2r2: self.c2r2,
            c2r3: self.c3r2,
            c3r0: self.c0r3,
            c3r1: self.c1r3,
            c3r2: self.c2r3,
            c3r3: self.c3r3,
        }
    }
    ///
    /// Calculate the determinant of this Matrix
    ///
    /// ```
    /// use softrender::math::Mat4f;
    ///
    /// let m = Mat4f::identity().determinant();
    /// ```
    ///
    pub fn determinant(&self) -> f32 {
        let b00 = self.c0r0 * self.c1r1 - self.c0r1 * self.c1r0;
        let b01 = self.c0r0 * self.c1r2 - self.c0r2 * self.c1r0;
        let b02 = self.c0r0 * self.c1r3 - self.c0r3 * self.c1r0;
        let b03 = self.c0r1 * self.c1r2 - self.c0r2 * self.c1r1;
        let b04 = self.c0r1 * self.c1r3 - self.c0r3 * self.c1r1;
        let b05 = self.c0r2 * self.c1r3 - self.c0r3 * self.c1r2;
        let b06 = self.c2r0 * self.c3r1 - self.c2r1 * self.c3r0;
        let b07 = self.c2r0 * self.c3r2 - self.c2r2 * self.c3r0;
        let b08 = self.c2r0 * self.c3r3 - self.c2r3 * self.c3r0;
        let b09 = self.c2r1 * self.c3r2 - self.c2r2 * self.c3r1;
        let b10 = self.c2r1 * self.c3r3 - self.c2r3 * self.c3r1;
        let b11 = self.c2r2 * self.c3r3 - self.c2r3 * self.c3r2;

        b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06
    }
    ///
    /// Calculate the inversion of this Matrix
    ///
    /// ```
    /// use softrender::math::Mat4f;
    ///
    /// let m = Mat4f::identity().determinant();
    /// ```
    ///
    pub fn invert(&self) -> Option<Self> {
        let x00 = self.c0r0;
        let x01 = self.c0r1;
        let x02 = self.c0r3;
        let x03 = self.c0r3;
        let x04 = self.c1r0;
        let x05 = self.c1r1;
        let x06 = self.c1r2;
        let x07 = self.c1r3;
        let x08 = self.c2r0;
        let x09 = self.c2r1;
        let x10 = self.c2r2;
        let x11 = self.c2r3;
        let x12 = self.c3r0;
        let x13 = self.c3r1;
        let x14 = self.c3r2;
        let x15 = self.c3r3;
        let a00 = x00 * x05 - x01 * x04;
        let a01 = x00 * x06 - x02 * x04;
        let a02 = x00 * x07 - x03 * x04;
        let a03 = x01 * x06 - x02 * x05;
        let a04 = x01 * x07 - x03 * x05;
        let a05 = x02 * x07 - x03 * x06;
        let b00 = x08 * x13 - x09 * x12;
        let b01 = x08 * x14 - x10 * x12;
        let b02 = x08 * x15 - x11 * x12;
        let b03 = x09 * x14 - x10 * x13;
        let b04 = x09 * x15 - x11 * x13;
        let b05 = x10 * x15 - x11 * x14;
        let det = a00 * b05 - a01 * b04 + a02 * b03 + a03 * b02 - a04 * b01 + a05 * b00;
        if det == 0.0 {
            None
        } else {
            let inv_det = 1.0 / det;
            Some(
                Self {
                    c0r0: (0.0 + x05 * b05 - x06 * b04 + x07 * b03) * inv_det,
                    c0r1: (0.0 - x01 * b05 + x02 * b04 - x03 * b03) * inv_det,
                    c0r2: (0.0 + x13 * a05 - x14 * a04 + x15 * a03) * inv_det,
                    c0r3: (0.0 - x09 * a05 + x10 * a04 - x11 * a03) * inv_det,
                    c1r0: (0.0 - x04 * b05 + x06 * b02 - x07 * b01) * inv_det,
                    c1r1: (0.0 + x00 * b05 - x02 * b02 + x03 * b01) * inv_det,
                    c1r2: (0.0 - x12 * a05 + x14 * a02 - x15 * a01) * inv_det,
                    c1r3: (0.0 + x08 * a05 - x10 * a02 + x11 * a01) * inv_det,
                    c2r0: (0.0 + x04 * b04 - x05 * b02 + x07 * b00) * inv_det,
                    c2r1: (0.0 - x00 * b04 + x01 * b02 - x03 * b00) * inv_det,
                    c2r2: (0.0 + x12 * a04 - x13 * a02 + x15 * a00) * inv_det,
                    c2r3: (0.0 - x08 * a04 + x09 * a02 - x11 * a00) * inv_det,
                    c3r0: (0.0 - x04 * b03 + x05 * b01 - x06 * b00) * inv_det,
                    c3r1: (0.0 + x00 * b03 - x01 * b01 + x02 * b00) * inv_det,
                    c3r2: (0.0 - x12 * a03 + x13 * a01 - x14 * a00) * inv_det,
                    c3r3: (0.0 + x08 * a03 - x09 * a01 + x10 * a00) * inv_det,
                }
            )
        }
    }
    pub fn perspective(fov_deg: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let fov_rad = 1.0 / (fov_deg * 0.5 / 180.0 * std::f32::consts::PI).tan();
        Mat4f {
            c0r0: aspect_ratio * fov_rad,
            c0r1: 0.0,
            c0r2: 0.0,
            c0r3: 0.0,
            c1r0: 0.0,
            c1r1: fov_rad,
            c1r2: 0.0,
            c1r3: 0.0,
            c2r0: 0.0,
            c2r1: 0.0,
            c2r2: far / (far - near),
            c2r3: 1.0,
            c3r0: 0.0,
            c3r1: 0.0,
            c3r2: (-far * near) / (far - near),
            c3r3: 0.0,
        }
    }
    pub fn look_at(eye: Vec3f, target: Vec3f, up: Vec3f) -> Self {
        let zaxis = (eye - target).normalize(); // The "forward" vector.
        let xaxis = Vec3f::cross(up, zaxis).normalize(); // The "right" vector.
        let yaxis = Vec3f::cross(zaxis, xaxis); // The "up" vector.

        // Create a 4x4 view matrix from the right, up, forward and eye position vectors
        Mat4f::from_rows([
            [xaxis.x, yaxis.x, zaxis.x, 0.0],
            [xaxis.y, yaxis.y, zaxis.y, 0.0],
            [xaxis.z, yaxis.z, zaxis.z, 0.0],
            [-xaxis.dot(eye), -yaxis.dot(eye), -zaxis.dot(eye), 1.0]
        ])
    }
}

impl fmt::Debug for Mat4f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n[ {}, {}, {}, {} ]\n[ {}, {}, {}, {} ]\n[ {}, {}, {}, {} ]\n[ {}, {}, {}, {} ]\n",
               self.c0r0, self.c0r1, self.c0r2, self.c0r3,
               self.c1r0, self.c1r1, self.c1r2, self.c1r3,
               self.c2r0, self.c2r1, self.c2r2, self.c2r3,
               self.c3r0, self.c3r1, self.c3r2, self.c3r3,
        )
    }
}

impl ops::Add<Self> for Mat4f {
    type Output = Self;

    ///     0  1  2  3       0  1  2  3           0      1      2      3
    /// 0 | A, B, C, D |   | a, b, c, d |   | A + a, B + b, C + c, D + d |
    /// 1 | E, F, G, H | + | e, f, g, h | = | E + e, F + f, G + g, H + h |
    /// 2 | I, J, K, L |   | i, j, k, l |   | I + i, J + j, K + k, L + l |
    /// 3 | M, N, O, P |   | m, n, o, p |   | M + m, N + n, O + o, P + p |
    fn add(self, rhs: Self) -> Self {
        Self {
            c0r0: self.c0r0 + rhs.c0r0,
            c0r1: self.c0r1 + rhs.c0r1,
            c0r2: self.c0r2 + rhs.c0r2,
            c0r3: self.c0r3 + rhs.c0r3,
            c1r0: self.c1r0 + rhs.c1r0,
            c1r1: self.c1r1 + rhs.c1r1,
            c1r2: self.c1r2 + rhs.c1r2,
            c1r3: self.c1r3 + rhs.c1r3,
            c2r0: self.c2r0 + rhs.c2r0,
            c2r1: self.c2r1 + rhs.c2r1,
            c2r2: self.c2r2 + rhs.c2r2,
            c2r3: self.c2r3 + rhs.c2r3,
            c3r0: self.c3r0 + rhs.c3r0,
            c3r1: self.c3r1 + rhs.c3r1,
            c3r2: self.c3r2 + rhs.c3r2,
            c3r3: self.c3r3 + rhs.c3r3,
        }
    }
}

impl ops::AddAssign<Self> for Mat4f {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Mul<Self> for Mat4f {
    type Output = Self;

    ///     0  1  2  3       0  1  2  3                       0                  1                  2                  3
    /// 0 | A, B, C, D |   | a, b, c, d |   | Aa + Be + Ci + Dm, Ab + Bf + Cj + Dn, Ac + Bg + Ck + Do, Ad + Bh + Cl + Dp |
    /// 1 | E, F, G, H | x | e, f, g, h | = | Ea + Fe + Gi + Hm, Eb + Ff + Gj + Hn, Ec + Fg + Gk + Ho, Ed + Fh + Gl + Hp |
    /// 2 | I, J, K, L |   | i, j, k, l |   | Ia + Je + Ki + Lm, Ib + Jf + Kj + Ln, Ic + Jg + Kk + Lo, Id + Jh + Kl + Lp |
    /// 3 | M, N, O, P |   | m, n, o, p |   | Ma + Ne + Oi + Pm, Mb + Nf + Oj + Pn, Mc + Ng + Ok + Po, Md + Nh + Ol + Pp |
    fn mul(self, rhs: Self) -> Self {
        Self {
            c0r0: (self.c0r0 * rhs.c0r0) + (self.c0r1 * rhs.c1r0) + (self.c0r2 * rhs.c2r0) + (self.c0r3 * rhs.c3r0),
            c0r1: (self.c0r0 * rhs.c0r1) + (self.c0r1 * rhs.c1r1) + (self.c0r2 * rhs.c2r1) + (self.c0r3 * rhs.c3r1),
            c0r2: (self.c0r0 * rhs.c0r2) + (self.c0r1 * rhs.c1r2) + (self.c0r2 * rhs.c2r2) + (self.c0r3 * rhs.c3r2),
            c0r3: (self.c0r0 * rhs.c0r3) + (self.c0r1 * rhs.c1r3) + (self.c0r2 * rhs.c2r3) + (self.c0r3 * rhs.c3r3),
            c1r0: (self.c1r0 * rhs.c0r0) + (self.c1r1 * rhs.c1r0) + (self.c1r2 * rhs.c2r0) + (self.c1r3 * rhs.c3r0),
            c1r1: (self.c1r0 * rhs.c0r1) + (self.c1r1 * rhs.c1r1) + (self.c1r2 * rhs.c2r1) + (self.c1r3 * rhs.c3r1),
            c1r2: (self.c1r0 * rhs.c0r2) + (self.c1r1 * rhs.c1r2) + (self.c1r2 * rhs.c2r2) + (self.c1r3 * rhs.c3r2),
            c1r3: (self.c1r0 * rhs.c0r3) + (self.c1r1 * rhs.c1r3) + (self.c1r2 * rhs.c2r3) + (self.c1r3 * rhs.c3r3),
            c2r0: (self.c2r0 * rhs.c0r0) + (self.c2r1 * rhs.c1r0) + (self.c2r2 * rhs.c2r0) + (self.c2r3 * rhs.c3r0),
            c2r1: (self.c2r0 * rhs.c0r1) + (self.c2r1 * rhs.c1r1) + (self.c2r2 * rhs.c2r1) + (self.c2r3 * rhs.c3r1),
            c2r2: (self.c2r0 * rhs.c0r2) + (self.c2r1 * rhs.c1r2) + (self.c2r2 * rhs.c2r2) + (self.c2r3 * rhs.c3r2),
            c2r3: (self.c2r0 * rhs.c0r3) + (self.c2r1 * rhs.c1r3) + (self.c2r2 * rhs.c2r3) + (self.c2r3 * rhs.c3r3),
            c3r0: (self.c3r0 * rhs.c0r0) + (self.c3r1 * rhs.c1r0) + (self.c3r2 * rhs.c2r0) + (self.c3r3 * rhs.c3r0),
            c3r1: (self.c3r0 * rhs.c0r1) + (self.c3r1 * rhs.c1r1) + (self.c3r2 * rhs.c2r1) + (self.c3r3 * rhs.c3r1),
            c3r2: (self.c3r0 * rhs.c0r2) + (self.c3r1 * rhs.c1r2) + (self.c3r2 * rhs.c2r2) + (self.c3r3 * rhs.c3r2),
            c3r3: (self.c3r0 * rhs.c0r3) + (self.c3r1 * rhs.c1r3) + (self.c3r2 * rhs.c2r3) + (self.c3r3 * rhs.c3r3),
        }
    }
}

impl ops::MulAssign<Self> for Mat4f {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Vec4f> for Mat4f {
    type Output = Vec4f;

    ///     0  1  2  3       0                       0
    /// 0 | A, B, C, D |   | x |   | Ax + By + Cz + Dw |
    /// 1 | E, F, G, H | x | y | = | Ex + Fy + Gz + Hw |
    /// 2 | I, J, K, L |   | z |   | Ix + Jy + Kz + Lw |
    /// 3 | M, N, O, P |   | w |   | Mx + Ny + Oz + Pw |
    fn mul(self, rhs: Vec4f) -> Vec4f {
        Vec4f {
            x: (self.c0r0 * rhs.x) + (self.c0r1 * rhs.y) + (self.c0r2 * rhs.z) + (self.c0r3 * rhs.w),
            y: (self.c1r0 * rhs.x) + (self.c1r1 * rhs.y) + (self.c1r2 * rhs.z) + (self.c1r3 * rhs.w),
            z: (self.c2r0 * rhs.x) + (self.c2r1 * rhs.y) + (self.c2r2 * rhs.z) + (self.c2r3 * rhs.w),
            w: (self.c3r0 * rhs.x) + (self.c3r1 * rhs.y) + (self.c3r2 * rhs.z) + (self.c3r3 * rhs.w),
        }
    }
}


impl ops::Sub<Self> for Mat4f {
    type Output = Self;

    /// Subtract one Mat4f from another.
    ///
    ///     0  1  2  3       0  1  2  3           0      1      2      3
    /// 0 | A, B, C, D |   | a, b, c, d |   | A - a, B - b, C - c, D - d |
    /// 1 | E, F, G, H | - | e, f, g, h | = | E - e, F - f, G - g, H - h |
    /// 2 | I, J, K, L |   | i, j, k, l |   | I - i, J - j, K - k, L - l |
    /// 3 | M, N, O, P |   | m, n, o, p |   | M - m, N - n, O - o, P - p |
    fn sub(self, rhs: Self) -> Self {
        Self {
            c0r0: self.c0r0 - rhs.c0r0,
            c0r1: self.c0r1 - rhs.c0r1,
            c0r2: self.c0r2 - rhs.c0r2,
            c0r3: self.c0r3 - rhs.c0r3,
            c1r0: self.c1r0 - rhs.c1r0,
            c1r1: self.c1r1 - rhs.c1r1,
            c1r2: self.c1r2 - rhs.c1r2,
            c1r3: self.c1r3 - rhs.c1r3,
            c2r0: self.c2r0 - rhs.c2r0,
            c2r1: self.c2r1 - rhs.c2r1,
            c2r2: self.c2r2 - rhs.c2r2,
            c2r3: self.c2r3 - rhs.c2r3,
            c3r0: self.c3r0 - rhs.c3r0,
            c3r1: self.c3r1 - rhs.c3r1,
            c3r2: self.c3r2 - rhs.c3r2,
            c3r3: self.c3r3 - rhs.c3r3,
        }
    }
}

impl ops::SubAssign<Self> for Mat4f {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::{Mat4f, Vec3f, Vec4f};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_from_rows() {
        let m = Mat4f::from_rows(
            [
                [01.0, 02.0, 03.0, 04.0],
                [05.0, 06.0, 07.0, 08.0],
                [09.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        );
        assert_approx_eq!(m.c0r0, 01.0);
        assert_approx_eq!(m.c3r0, 03.0);
        assert_approx_eq!(m.c1r1, 06.5);
        assert_approx_eq!(m.c1r3, 08.5);
        assert_approx_eq!(m.c3r0, 13.5);
        assert_approx_eq!(m.c3r2, 15.5);
    }

    #[test]
    fn test_from_cols() {
        let m = Mat4f::from_cols(
            [
                [01.0, 05.0, 09.0, 13.0],
                [02.0, 06.0, 10.0, 14.0],
                [03.0, 07.0, 11.0, 15.0],
                [04.5, 08.0, 12.0, 16.0],
            ]
        );
        assert_approx_eq!(m.c0r0, 1.0);
        assert_approx_eq!(m.c0r3, 4.0);
        assert_approx_eq!(m.c1r0, 5.5);
        assert_approx_eq!(m.c1r2, 7.5);
        assert_approx_eq!(m.c2r2, 11.0);
        assert_approx_eq!(m.c3r0, 13.5);
        assert_approx_eq!(m.c3r2, 15.5);
    }

    #[test]
    fn test_transpose() {
        let a = Mat4f::from_rows(
            [
                [00.0, 01.0, 02.0, 03.0],
                [07.0, 06.0, 05.0, 04.0],
                [08.0, 09.0, 10.0, 11.0],
                [15.0, 14.0, 13.0, 12.0],
            ]
        );
        let b = Mat4f::from_rows(
            [
                [00.0, 07.0, 08.0, 15.0],
                [01.0, 06.0, 09.0, 14.0],
                [02.0, 05.0, 10.0, 13.0],
                [03.0, 04.0, 11.0, 12.0],
            ]
        );
        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn test_determinant() {
        let a = Mat4f::from_rows(
            [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, 2.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        assert_eq!(a.determinant(), -20.0);
    }

    #[test]
    fn test_invert() {
        let a = Mat4f::from_rows(
            [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, 2.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        let b = Mat4f::from_rows(
            [
                [-0.4, 00.5, 00.0, 00.1],
                [00.5, -1.0, 00.5, 00.0],
                [00.0, 00.5, -1.0, 00.5],
                [00.1, 00.0, 00.5, -0.4],
            ]
        );
        assert_eq!(a.invert().unwrap(), b);
    }

    #[test]
    fn test_lookat() {
        let eye = Vec3f::from_parts(0.0, 0.0, 0.0);
        let target = Vec3f::from_parts(1.0, 1.0, 1.0);
        let up = Vec3f::from_parts(0.0, 0.0, 1.0);
        let a = Mat4f::look_at(eye, target, up);
        let b = Mat4f::from_rows([
            [-6.0, 01.0, 01.0, 06.0],
            [-8.0, 05.0, 08.0, 06.0],
            [-1.0, 00.0, 08.0, 02.0],
            [-7.0, 01.0, -1.0, 01.0],
        ]);
        assert_eq!(a, b)
    }

    #[test]
    fn test_perspective() {
        let fov = 90.0;
        let aspect_ratio = 90.0;
        let near = 0.0001;
        let far = 1.0000;
        let a = Mat4f::perspective(fov, aspect_ratio, near, far);
        let b = Mat4f::from_rows([
            [01.810660, 00.000000, 00.000000, 00.000000],
            [00.000000, 02.414213, 00.000000, 00.000000],
            [00.000000, 00.000000, -1.002002, -1.000000],
            [00.000000, 00.000000, -0.200200, 00.000000],
        ]);
        assert_eq!(a, b)
    }

    #[test]
    fn test_partialeq() {
        let a = Mat4f::from_rows(
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ]
        );
        let b = Mat4f::from_rows(
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ]
        );
        let c = Mat4f::from_rows(
            [
                [13.5, 14.5, 15.5, 16.5],
                [9.0, 10.0, 11.0, 12.0],
                [5.5, 6.5, 7.5, 8.5],
                [1.0, 2.0, 3.0, 4.0],
            ]
        );
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add_mat4f() {
        let a = Mat4f::from_rows(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        let b = Mat4f::from_rows(
            [
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
            ]
        );
        let c = Mat4f::from_rows(
            [
                [6.0, 8.0, 10.0, 12.0],
                [12.0, 10.0, 8.0, 6.0],
                [6.0, 8.0, 10.0, 12.0],
                [12.0, 10.0, 8.0, 6.0],
            ]
        );
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_addassign_mat4f() {
        let mut a = Mat4f::from_rows(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        a += Mat4f::from_rows(
            [
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
            ]
        );
        let c = Mat4f::from_rows(
            [
                [6.0, 8.0, 10.0, 12.0],
                [12.0, 10.0, 8.0, 6.0],
                [6.0, 8.0, 10.0, 12.0],
                [12.0, 10.0, 8.0, 6.0],
            ]
        );
        assert_eq!(a, c);
    }

    #[test]
    fn test_mul_mat4f() {
        let a = Mat4f::from_rows(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        let b = Mat4f::from_rows(
            [
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
            ]
        );
        let c = Mat4f::from_rows(
            [
                [22.0, 24.0, 26.0, 28.0],
                [28.0, 26.0, 24.0, 22.0],
                [22.0, 24.0, 26.0, 28.0],
                [28.0, 26.0, 24.0, 22.0],
            ]
        );
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_mulassign_mat4f() {
        let mut a = Mat4f::from_rows(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        a *= Mat4f::from_rows(
            [
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
            ]
        );
        let c = Mat4f::from_rows(
            [
                [22.0, 24.0, 26.0, 28.0],
                [28.0, 26.0, 24.0, 22.0],
                [22.0, 24.0, 26.0, 28.0],
                [28.0, 26.0, 24.0, 22.0],
            ]
        );
        assert_eq!(a, c);
    }

    #[test]
    fn test_mul_vec4f() {
        let a = Mat4f::from_rows(
            [
                [0.0, 1.0, 2.0, 3.0],
                [7.0, 6.0, 5.0, 4.0],
                [8.0, 9.0, 8.0, 7.0],
                [3.0, 4.0, 5.0, 6.0],
            ]
        );
        let b = Vec4f::from_parts(2.0, 1.0, 0.0, 1.0);
        let c = Vec4f::from_parts(4.0, 24.0, 32.0, 16.0);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_sub_mat4f() {
        let a = Mat4f::from_rows(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        let b = Mat4f::from_rows(
            [
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
            ]
        );
        let c = Mat4f::from_rows(
            [
                [-4.0, -4.0, -4.0, -4.0],
                [-4.0, -4.0, -4.0, -4.0],
                [-4.0, -4.0, -4.0, -4.0],
                [-4.0, -4.0, -4.0, -4.0],
            ]
        );
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_subassign_mat4f() {
        let mut a = Mat4f::from_rows(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        a -= Mat4f::from_rows(
            [
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
            ]
        );
        let c = Mat4f::from_rows(
            [
                [-4.0, -4.0, -4.0, -4.0],
                [-4.0, -4.0, -4.0, -4.0],
                [-4.0, -4.0, -4.0, -4.0],
                [-4.0, -4.0, -4.0, -4.0],
            ]
        );
        assert_eq!(a, c);
    }
}
