#![allow(unused)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod math;
mod render;
mod window;
use math::{color::Color, vector::*};
use render::{ogl::*, shaders, shapes::Square};
use window::*;

#[test]
fn test_window() {
    let mut window = Window::default();

    let mut sq = Square::new(vec2(0., 0.), vec2(0.1, 0.1));

    let ds = DrawStream::from(sq); // contains the vao, and vbo
    let mut mat = Material::default(); // makes the shader program

    while !window.should_close() {
        while let Some(e) = window.events() {
            if e.is_key_pressed(Key::Escape) {
                window.set_should_close(true);
            }

            if let WindowEvent::Size(x, y) = e.events {
                println!("resized");
                unsafe {
                    gl::Viewport(0, 0, x, y);
                }
            }
        }

        window.clear(Color::BLUE);
        ds.draw(&mat);
        window.swap_buffers();
    }
}
