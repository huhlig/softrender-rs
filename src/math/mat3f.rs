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

#[cfg(test)]
mod tests {
    use super::Mat3f;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_creation() {
        let m = Mat3f::from_array(
            [
                [1.0, 2.0, 3.0],
                [5.5, 6.5, 7.5],
                [9.0, 10.0, 11.0],
            ]
        );
        assert_approx_eq!(m.m00, 1.0);
        assert_approx_eq!(m.m10, 5.5);
        assert_approx_eq!(m.m12, 7.5);
        assert_approx_eq!(m.m22, 11.0);
    }

    #[test]
    fn test_multiply() {
        let a = Mat3f::from_array(
            [
                [1.0, 2.0, 3.0],
                [4.0, 3.0, 2.0],
                [1.0, 2.0, 3.0],
            ]
        );
        let b = Mat3f::from_array(
            [
                [4.0, 3.0, 2.0],
                [1.0, 2.0, 3.0],
                [4.0, 3.0, 2.0],
            ]
        );
        let c = Mat3f::from_array(
            [
                [22.0, 24.0, 26.0],
                [28.0, 26.0, 24.0],
                [22.0, 24.0, 26.0],
            ]
        );
        assert_eq!(a*b,c);
    }
}
