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

///
/// Color Channel
///
#[derive(Copy, Clone, Debug)]
pub struct Channel(f32);

impl ops::Add<Self> for Channel {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign<Self> for Channel {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl ops::Mul<f32> for Channel {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs)
    }
}

impl ops::MulAssign<f32> for Channel {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl ops::Mul<Self> for Channel {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl ops::MulAssign<Self> for Channel {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl ops::Sub<Self> for Channel {
    type Output = Self;

    fn sub(self, rhs: Channel) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl ops::SubAssign<Self> for Channel {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl PartialEq<Self> for Channel {
    fn eq(&self, other: &Channel) -> bool {
        (self.0.max(0.0).min(1.0) - other.0.max(0.0).min(1.0)).abs() < 0.004
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.4}", self.0)
    }
}

impl From<u8> for Channel {
    fn from(value: u8) -> Self {
        Channel(value as f32 / 255.0)
    }
}

impl From<f32> for Channel {
    fn from(value: f32) -> Self {
        Channel(value)
    }
}

impl From<Channel> for u8 {
    fn from(value: Channel) -> Self {
        (value.0.max(0.0).min(1.0) * 255.0) as u8
    }
}

impl From<Channel> for f32 {
    fn from(value: Channel) -> Self {
        value.0.max(0.0).min(1.0)
    }
}
