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
use super::math::Vec3f;
use super::image::ColorRGBf32;
use super::model::Sphere;

struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

fn intersect(ray: &Ray, sphere: &Sphere) -> Option<(f32, f32)> {
    let l = sphere.center - ray.origin;
    let tca = l.dot(ray.direction);
    if tca < 0.0 {
        return Option::None;
    }
    let d2 = l.dot(l) - (tca * tca);
    let r2 = sphere.radius * sphere.radius;
    if d2 > r2 {
        return Option::None;
    }
    let thc = (r2 - d2).sqrt();
    let t0 = tca - thc;
    let t1 = tca + thc;
    return Option::Some((t0, t1));
}

fn mix(a: f32, b: f32, mix: f32) -> f32 { b * mix + a * (1.0 - mix) }

fn nearest_sphere<'a>(ray: &Ray, spheres: &'a [Sphere]) -> Option<(&'a Sphere, f32)> {
    let mut t_near = ::std::f32::INFINITY;
    let mut t_sphere = Option::None;
    for sphere in spheres {
        if let Some((t0, t1)) = intersect(ray, sphere) {
            let mut t = if t0 < 0.0 { t1 } else { t0 };
            if t < t_near {
                t_near = t;
                t_sphere = Option::Some(sphere);
            }
        }
    }
    match t_sphere {
        None => Option::None,
        Some(sphere) => Option::Some((sphere, t_near)),
    }
}

fn trace(ray: &Ray, spheres: &[Sphere], background: ColorRGBf32, depth: usize) -> ColorRGBf32 {
    let (t_sphere, t_near) = {
        let mut t_near = ::std::f32::INFINITY;
        let mut t_sphere = Option::None;
        for sphere in spheres {
            if let Some((t0, t1)) = intersect(ray, sphere) {
                let mut t3;
                if t0 < 0.0 {
                    t3 = t1;
                }
                if t3 < t_near {
                    t_near = t3;
                    t_sphere = Option::Some(sphere);
                }
            }
        }
        match t_sphere {
            Option::Some(sphere) => (sphere, t_near),
            Option::None => { return background; }
        }
    };
    match nearest_sphere(ray, spheres) {
        None => { return background; }
        Some(sphere, t_near) => {
            let surface_color = ColorRGBf32::black();
            let phit = ray.origin + ray.direction * t_near;
            let nhit = (phit - sphere.center).normalize();
        }
    }


    ColorRGBf32::default()
}