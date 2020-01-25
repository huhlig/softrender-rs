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
use std::{f32::consts::PI, fmt, ops};
use crate::math::Vec3f;

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
    pub fn perspective(fov_deg: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4f {
        let fov_rad = 1.0 / (fov_deg * 0.5 / 180.0 * PI).tan();
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


impl ops::Mul<Mat4f> for Mat4f {
    type Output = Self;

    ///     0  1  2  3       0  1  2  3                       0                  1                  2                  3
    /// 0 | A, B, C, D |   | a, b, c, d |   | Aa + Be + Ci + Dm, Ab + Bf + Cj + Dn, Ac + Bg + Ck + Do, Ad + Bh + Cl + Dp |
    /// 1 | E, F, G, H | x | e, f, g, h | = | Ea + Fe + Gi + Hm, Eb + Ff + Gj + Hn, Ec + Fg + Gk + Ho, Ed + Fh + Gl + Hp |
    /// 2 | I, J, K, L |   | i, j, k, l |   | Ia + Je + Ki + Lm, Ib + Jf + Kj + Ln, Ic + Jg + Kk + Lo, Id + Jh + Kl + Lp |
    /// 3 | M, N, O, P |   | m, n, o, p |   | Ma + Ne + Oi + Pm, Mb + Nf + Oj + Pn, Mc + Ng + Ok + Po, Md + Nh + Ol + Pp |
    fn mul(self, rhs: Mat4f) -> Self {
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

#[cfg(test)]
mod tests {
    use super::Mat4f;

    #[test]
    #[allow(non_snake_case)]
    fn test_multiply() {
        let A = Mat4f::from_array(
            [
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
            ]
        );
        let B = Mat4f::from_array(
            [
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
                [4.0, 3.0, 2.0, 1.0],
                [1.0, 2.0, 3.0, 4.0],
            ]
        );
        let C = Mat4f::from_array(
            [
                [22.0, 24.0, 26.0, 28.0],
                [28.0, 26.0, 24.0, 22.0],
                [22.0, 24.0, 26.0, 28.0],
                [28.0, 26.0, 24.0, 22.0],
            ]
        );
        let D = A * B;
        assert_eq!(C, D);
    }
}

// static mat4 operator*(const mat4& a, const mat4& b) {
//    mat4 out;
//    __m128 row1 = _mm_loadu_ps(&a.data()[0]), row2 = _mm_loadu_ps(&a.data()[4]), row3 = _mm_loadu_ps(&a.data()[8]), row4 = _mm_loadu_ps(&a.data()[12]);
//    __m128 col1, col2, col3, col4;
//    __m128 row;
//
//    col1 = _mm_set_ps1(b.data()[0]);
//    col2 = _mm_set_ps1(b.data()[1]);
//    col3 = _mm_set_ps1(b.data()[2]);
//    col4 = _mm_set_ps1(b.data()[3]);
//    row = _mm_add_ps(_mm_add_ps(_mm_mul_ps(row1, col1), _mm_mul_ps(row2, col2)), _mm_add_ps(_mm_mul_ps(row3, col3), _mm_mul_ps(row4, col4)));
//    _mm_storeu_ps(&out.data()[0], row);
//
//    col1 = _mm_set_ps1(b.data()[4]);
//    col2 = _mm_set_ps1(b.data()[5]);
//    col3 = _mm_set_ps1(b.data()[6]);
//    col4 = _mm_set_ps1(b.data()[7]);
//    row = _mm_add_ps(_mm_add_ps(_mm_mul_ps(row1, col1), _mm_mul_ps(row2, col2)), _mm_add_ps(_mm_mul_ps(row3, col3), _mm_mul_ps(row4, col4)));
//    _mm_storeu_ps(&out.data()[4], row);
//
//    col1 = _mm_set_ps1(b.data()[8]);
//    col2 = _mm_set_ps1(b.data()[9]);
//    col3 = _mm_set_ps1(b.data()[10]);
//    col4 = _mm_set_ps1(b.data()[11]);
//    row = _mm_add_ps(_mm_add_ps(_mm_mul_ps(row1, col1), _mm_mul_ps(row2, col2)), _mm_add_ps(_mm_mul_ps(row3, col3), _mm_mul_ps(row4, col4)));
//    _mm_storeu_ps(&out.data()[8], row);
//
//    col1 = _mm_set_ps1(b.data()[12]);
//    col2 = _mm_set_ps1(b.data()[13]);
//    col3 = _mm_set_ps1(b.data()[14]);
//    col4 = _mm_set_ps1(b.data()[15]);
//    row = _mm_add_ps(_mm_add_ps(_mm_mul_ps(row1, col1), _mm_mul_ps(row2, col2)), _mm_add_ps(_mm_mul_ps(row3, col3), _mm_mul_ps(row4, col4)));
//    _mm_storeu_ps(&out.data()[12], row);
//
//    return out;
//}