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
}

impl fmt::Debug for Mat2f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n[ {}, {} ]\n[ {}, {} ]\n",
               self.m00, self.m01,
               self.m10, self.m11,
        )
    }
}


impl ops::Mul<Mat2f> for Mat2f {
    type Output = Self;

    ///     0  1  2       0  1  2                  0             1             2             3
    /// 0 | A, B, C |   | a, b, c |   | Aa + Be + Ci, Ab + Bf + Cj, Ac + Bg + Ck, Ad + Bh + Cl |
    /// 1 | E, F, G | x | e, f, g | = | Ea + Fe + Gi, Eb + Ff + Gj, Ec + Fg + Gk, Ed + Fh + Gl |
    /// 2 | I, J, K |   | i, j, k |   | Ia + Je + Ki, Ib + Jf + Kj, Ic + Jg + Kk, Id + Jh + Kl |
    fn mul(self, rhs: Mat2f) -> Self {
        Self {
            m00: (self.m00 * rhs.m00) + (self.m01 * rhs.m10),
            m01: (self.m00 * rhs.m01) + (self.m01 * rhs.m11),
            m10: (self.m10 * rhs.m00) + (self.m11 * rhs.m10),
            m11: (self.m10 * rhs.m01) + (self.m11 * rhs.m11),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Mat2f;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_creation() {
        let m = Mat2f::from_array(
            [
                [1.0, 2.0],
                [5.5, 6.5],
            ]
        );
        assert_approx_eq!(m.m00, 1.0);
        assert_approx_eq!(m.m10, 5.5);
    }

    #[test]
    fn test_multiply() {
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
                [22.0, 24.0],
                [28.0, 26.0],
            ]
        );
        assert_eq!(a * b, c);
    }
}
