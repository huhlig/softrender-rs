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
use super::Mesh;
use super::Triangle;
use super::Vec3f;

pub fn cube(size: f32) -> Mesh {
    let rad = size / 2.0;
    // Front
    let swb = Vec3f::new(-rad, -rad, -rad);
    let swt = Vec3f::new(-rad, rad, -rad);
    let seb = Vec3f::new(rad, -rad, -rad);
    let set = Vec3f::new(rad, rad, -rad);
    let nwb = Vec3f::new(-rad, -rad, rad);
    let nwt = Vec3f::new(-rad, rad, rad);
    let neb = Vec3f::new(rad, -rad, rad);
    let net = Vec3f::new(rad, rad, rad);
    Mesh::new(
        vec!(
            // South
            Triangle::new(swb, swt, set),
            Triangle::new(set, seb, swb),
            // East
            Triangle::new(seb, set, net),
            Triangle::new(net, neb, seb),
            // North
            Triangle::new(net, nwt, nwb),
            Triangle::new(nwb, neb, net),
            // West
            Triangle::new(nwt, swt, swb),
            Triangle::new(swb, nwb, nwt),
            // Top
            Triangle::new(swt, nwt, net),
            Triangle::new(net, set, swt),
            // Bottom
            Triangle::new(neb, nwb, swb),
            Triangle::new(swb, seb, neb),
        )
    )
}
