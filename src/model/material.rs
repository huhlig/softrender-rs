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

use crate::image::Color;

pub struct Material<T: Color> {
    pub surface: T,
    pub emission: T,
    pub transparency: f32,
    pub reflectivity: f32,
}

impl<T: Color> Material<T> {
    pub fn new() -> Material<T> {
        Material {
            surface: T::white(),
            emission: T::white(),
            transparency: 0.0,
            reflectivity: 0.0,
        }
    }
}