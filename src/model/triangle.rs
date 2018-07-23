//
// Copyright 2017 Hans W. Uhlig.
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
use super::super::math::Vec3f;

#[derive(Copy, Clone, PartialEq)]
pub struct Triangle {
    pub a: Vec3f,
    pub b: Vec3f,
    pub c: Vec3f,
}

impl Triangle {
    pub fn new(a: Vec3f, b: Vec3f, c: Vec3f) -> Triangle {
        Triangle { a, b, c }
    }
}
