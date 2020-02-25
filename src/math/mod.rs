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

//! Column Major math Library

mod mat2f;
mod mat3f;
mod mat4f;
mod vec2f;
mod vec3f;
mod vec4f;

pub use self::mat2f::Mat2f;
pub use self::mat3f::Mat3f;
pub use self::mat4f::Mat4f;
pub use self::vec2f::Vec2f;
pub use self::vec3f::Vec3f;
pub use self::vec4f::Vec4f;
