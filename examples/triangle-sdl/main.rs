// Copyright 2015 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate gfx;
extern crate gfx_window_sdl;
extern crate sdl2;

use gfx::traits::FactoryExt;
use gfx::Device;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] }
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    video.gl_attr().set_context_profile(sdl2::video::GLProfile::Core);
    video.gl_attr().set_context_version(3, 2);
    let builder = video.window("Triangle Example SDL", 1024, 768);
    let (window, _gl_context, mut device, mut factory, main_color, _main_depth) =
        gfx_window_sdl::init::<ColorFormat, DepthFormat>(builder).unwrap();

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory
        .create_pipeline_simple(include_bytes!("../triangle/shader/triangle_150.glslv"),
                                include_bytes!("../triangle/shader/triangle_150.glslf"),
                                pipe::new())
        .unwrap();
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color
    };

    let mut events = sdl_context.event_pump().unwrap();

    let mut running = true;
    while running {
        // handle events
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyUp { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                }
                _ => {}
            }
        }

        // draw a frame
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.gl_swap_window();
        device.cleanup();
    }
}
