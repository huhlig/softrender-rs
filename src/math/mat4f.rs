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
use super::{Mat3f, Vec3f, Vec4f};

#[derive(Copy, Clone, PartialEq)]
pub struct Mat4f {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m03: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m30: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
}

impl Mat4f {
    pub fn from_array(data: [[f32; 4]; 4]) -> Mat4f {
        Mat4f {
            m00: data[0][0],
            m01: data[0][1],
            m02: data[0][2],
            m03: data[0][3],
            m10: data[1][0],
            m11: data[1][1],
            m12: data[1][2],
            m13: data[1][3],
            m20: data[2][0],
            m21: data[2][1],
            m22: data[2][2],
            m23: data[2][3],
            m30: data[3][0],
            m31: data[3][1],
            m32: data[3][2],
            m33: data[3][3],
        }
    }
    pub fn identity() -> Mat4f {
        Mat4f {
            m00: 1.0,
            m01: 0.0,
            m02: 0.0,
            m03: 0.0,
            m10: 0.0,
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m30: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
        }
    }
    pub fn transpose(&self) -> Self {
        Self {
            m00: self.m00,
            m01: self.m10,
            m02: self.m20,
            m03: self.m30,
            m10: self.m01,
            m11: self.m11,
            m12: self.m21,
            m13: self.m31,
            m20: self.m02,
            m21: self.m12,
            m22: self.m22,
            m23: self.m32,
            m30: self.m03,
            m31: self.m13,
            m32: self.m23,
            m33: self.m33,
        }
    }
    pub fn submatrix(&self, row: u8, column: u8) -> Mat3f {
        Mat3f::identity()
    }
    pub fn perspective(fov_deg: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4f {
        let fov_rad = 1.0 / (fov_deg * 0.5 / 180.0 * std::f32::consts::PI).tan();
        Mat4f {
            m00: aspect_ratio * fov_rad,
            m01: 0.0,
            m02: 0.0,
            m03: 0.0,
            m10: 0.0,
            m11: fov_rad,
            m12: 0.0,
            m13: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: far / (far - near),
            m23: 1.0,
            m30: 0.0,
            m31: 0.0,
            m32: (-far * near) / (far - near),
            m33: 0.0,
        }
    }
    pub fn look_at(eye: Vec3f, target: Vec3f, up: Vec3f) -> Mat4f {
        let zaxis = (eye - target).normalize(); // The "forward" vector.
        let xaxis = Vec3f::cross(up, zaxis).normalize(); // The "right" vector.
        let yaxis = Vec3f::cross(zaxis, xaxis); // The "up" vector.

        // Create a 4x4 view matrix from the right, up, forward and eye position vectors
        Mat4f::from_array([
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
               self.m00, self.m01, self.m02, self.m03,
               self.m10, self.m11, self.m12, self.m13,
               self.m20, self.m21, self.m22, self.m23,
               self.m30, self.m31, self.m32, self.m33,
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
            m00: self.m00 + rhs.m00,
            m01: self.m01 + rhs.m01,
            m02: self.m02 + rhs.m02,
            m03: self.m03 + rhs.m03,
            m10: self.m10 + rhs.m10,
            m11: self.m11 + rhs.m11,
            m12: self.m12 + rhs.m12,
            m13: self.m13 + rhs.m13,
            m20: self.m20 + rhs.m20,
            m21: self.m21 + rhs.m21,
            m22: self.m22 + rhs.m22,
            m23: self.m23 + rhs.m23,
            m30: self.m30 + rhs.m30,
            m31: self.m31 + rhs.m31,
            m32: self.m32 + rhs.m32,
            m33: self.m33 + rhs.m33,
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
            m00: (self.m00 * rhs.m00) + (self.m01 * rhs.m10) + (self.m02 * rhs.m20) + (self.m03 * rhs.m30),
            m01: (self.m00 * rhs.m01) + (self.m01 * rhs.m11) + (self.m02 * rhs.m21) + (self.m03 * rhs.m31),
            m02: (self.m00 * rhs.m02) + (self.m01 * rhs.m12) + (self.m02 * rhs.m22) + (self.m03 * rhs.m32),
            m03: (self.m00 * rhs.m03) + (self.m01 * rhs.m13) + (self.m02 * rhs.m23) + (self.m03 * rhs.m33),
            m10: (self.m10 * rhs.m00) + (self.m11 * rhs.m10) + (self.m12 * rhs.m20) + (self.m13 * rhs.m30),
            m11: (self.m10 * rhs.m01) + (self.m11 * rhs.m11) + (self.m12 * rhs.m21) + (self.m13 * rhs.m31),
            m12: (self.m10 * rhs.m02) + (self.m11 * rhs.m12) + (self.m12 * rhs.m22) + (self.m13 * rhs.m32),
            m13: (self.m10 * rhs.m03) + (self.m11 * rhs.m13) + (self.m12 * rhs.m23) + (self.m13 * rhs.m33),
            m20: (self.m20 * rhs.m00) + (self.m21 * rhs.m10) + (self.m22 * rhs.m20) + (self.m23 * rhs.m30),
            m21: (self.m20 * rhs.m01) + (self.m21 * rhs.m11) + (self.m22 * rhs.m21) + (self.m23 * rhs.m31),
            m22: (self.m20 * rhs.m02) + (self.m21 * rhs.m12) + (self.m22 * rhs.m22) + (self.m23 * rhs.m32),
            m23: (self.m20 * rhs.m03) + (self.m21 * rhs.m13) + (self.m22 * rhs.m23) + (self.m23 * rhs.m33),
            m30: (self.m30 * rhs.m00) + (self.m31 * rhs.m10) + (self.m32 * rhs.m20) + (self.m33 * rhs.m30),
            m31: (self.m30 * rhs.m01) + (self.m31 * rhs.m11) + (self.m32 * rhs.m21) + (self.m33 * rhs.m31),
            m32: (self.m30 * rhs.m02) + (self.m31 * rhs.m12) + (self.m32 * rhs.m22) + (self.m33 * rhs.m32),
            m33: (self.m30 * rhs.m03) + (self.m31 * rhs.m13) + (self.m32 * rhs.m23) + (self.m33 * rhs.m33),
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
            x: (self.m00 * rhs.x) + (self.m01 * rhs.y) + (self.m02 * rhs.z) + (self.m03 * rhs.w),
            y: (self.m10 * rhs.x) + (self.m11 * rhs.y) + (self.m12 * rhs.z) + (self.m13 * rhs.w),
            z: (self.m20 * rhs.x) + (self.m21 * rhs.y) + (self.m22 * rhs.z) + (self.m23 * rhs.w),
            w: (self.m30 * rhs.x) + (self.m31 * rhs.y) + (self.m32 * rhs.z) + (self.m33 * rhs.w),
        }
    }
}


impl ops::Sub<Self> for Mat4f {
    type Output = Self;

    ///     0  1  2  3       0  1  2  3           0      1      2      3
    /// 0 | A, B, C, D |   | a, b, c, d |   | A - a, B - b, C - c, D - d |
    /// 1 | E, F, G, H | - | e, f, g, h | = | E - e, F - f, G - g, H - h |
    /// 2 | I, J, K, L |   | i, j, k, l |   | I - i, J - j, K - k, L - l |
    /// 3 | M, N, O, P |   | m, n, o, p |   | M - m, N - n, O - o, P - p |
    fn sub(self, rhs: Self) -> Self {
        Self {
            m00: self.m00 - rhs.m00,
            m01: self.m01 - rhs.m01,
            m02: self.m02 - rhs.m02,
            m03: self.m03 - rhs.m03,
            m10: self.m10 - rhs.m10,
            m11: self.m11 - rhs.m11,
            m12: self.m12 - rhs.m12,
            m13: self.m13 - rhs.m13,
            m20: self.m20 - rhs.m20,
            m21: self.m21 - rhs.m21,
            m22: self.m22 - rhs.m22,
            m23: self.m23 - rhs.m23,
            m30: self.m30 - rhs.m30,
            m31: self.m31 - rhs.m31,
            m32: self.m32 - rhs.m32,
            m33: self.m33 - rhs.m33,
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
    use super::{Mat3f, Mat4f, Vec3f, Vec4f};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_creation() {
        let m = Mat4f::from_array(
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ]
        );
        assert_approx_eq!(m.m00, 1.0);
        assert_approx_eq!(m.m03, 4.0);
        assert_approx_eq!(m.m10, 5.5);
        assert_approx_eq!(m.m12, 7.5);
        assert_approx_eq!(m.m22, 11.0);
        assert_approx_eq!(m.m30, 13.5);
        assert_approx_eq!(m.m32, 15.5);
    }

    #[test]
    fn test_transpose() {
        let a = Mat4f::from_array(
            [
                [00.0, 01.0, 02.0, 03.0],
                [07.0, 06.0, 05.0, 04.0],
                [08.0, 09.0, 10.0, 11.0],
                [15.0, 14.0, 13.0, 12.0],
            ]
        );
        let b = Mat4f::from_array(
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
    fn test_submatrix() {
        let a = Mat4f::from_array(
            [
                [-6.0, 01.0, 01.0, 06.0],
                [-8.0, 05.0, 08.0, 06.0],
                [-1.0, 00.0, 08.0, 02.0],
                [-7.0, 01.0, -1.0, 01.0],
            ]
        );
        let b = Mat3f::from_array(
            [
                [-6.0, 01.0, 06.0],
                [-8.0, 05.0, 06.0],
                [-7.0, 01.0, 01.0],
            ]
        );
        assert_eq!(a.submatrix(2, 1), b);
    }

    #[test]
    fn test_lookat() {
        let eye = Vec3f::new(0.0, 0.0, 0.0);
        let target = Vec3f(1.0, 1.0, 1.0);
        let up = Vec3f::new(0.0, 0.0, 1.0);
        let a = Mat4f::look_at(eye, target, up);
        unimplemented!("Not yet written")
    }

    #[test]
    fn test_lookat() {
        let fov = 90.0;
        let aspect_ratio = 90.0;
        let near = 0.0001;
        let far = 1.0000;
        let a = Mat4f::perspective(fov, aspect_ratio, near, far);
        let b = Mat4f::from_array([
            [-6.0, 01.0, 01.0, 06.0],
            [-8.0, 05.0, 08.0, 06.0],
            [-1.0, 00.0, 08.0, 02.0],
            [-7.0, 01.0, -1.0, 01.0],
        ]);
        unimplemented!("Not yet written")
    }

    #[test]
    fn test_partialeq() {
        let a = Mat4f::from_array(
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ]
        );
        let b = Mat4f::from_array(
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ]
        );
        let c = Mat4f::from_array(
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
        let a = Mat4f::from_array(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        let b = Mat4f::from_array(
            [
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
            ]
        );
        let c = Mat4f::from_array(
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
        let mut a = Mat4f::from_array(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        a += Mat4f::from_array(
            [
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
            ]
        );
        let c = Mat4f::from_array(
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
        let a = Mat4f::from_array(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        let b = Mat4f::from_array(
            [
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
            ]
        );
        let c = Mat4f::from_array(
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
        let mut a = Mat4f::from_array(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        a *= Mat4f::from_array(
            [
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
            ]
        );
        let c = Mat4f::from_array(
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
        let a = Mat4f::from_array(
            [
                [0.0, 1.0, 2.0, 3.0],
                [7.0, 6.0, 5.0, 4.0],
                [8.0, 9.0, 8.0, 7.0],
                [3.0, 4.0, 5.0, 6.0],
            ]
        );
        let b = Vec4f::new(2.0, 1.0, 0.0, 1.0);
        let c = Vec4f::new(4.0, 24.0, 32.0, 16.0);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_sub_mat4f() {
        let a = Mat4f::from_array(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        let b = Mat4f::from_array(
            [
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
            ]
        );
        let c = Mat4f::from_array(
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
        let mut a = Mat4f::from_array(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        a -= Mat4f::from_array(
            [
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
                [5.0, 6.0, 7.0, 8.0],
                [8.0, 7.0, 6.0, 5.0],
            ]
        );
        let c = Mat4f::from_array(
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
