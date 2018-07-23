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
#![feature(nll)]
extern crate minifb;
extern crate render;

use minifb::{Key, WindowOptions, Window};
use render::color::ColorRGBAu8;
use render::image::ImageRGBAu8;
use render::math::Mat4f;
use render::Rasterizer;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut buffer = ImageRGBAu8::new(WIDTH, HEIGHT);
    let mut window = Window::new("Rasterizer Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    println!("Created {:?}", buffer);
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let projection = Mat4f::perspective(90.0, aspect_ratio, 0.0001, 1000.0);
    let mut view = Mat4f::identity();
    let mut model = Mat4f::identity();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.clear(ColorRGBAu8::black());
        buffer.draw_line(0, 0, 639, 479, ColorRGBAu8::white());

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(buffer.as_u32_slice()).unwrap();
    }
}