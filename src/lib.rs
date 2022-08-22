#![allow(unused)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod math;
mod render;
mod window;
use math::{color::Color, vector::*};
use render::{
    ogl::*,
    shaders,
    shapes::{DrawStream, Square},
};
use window::*;

#[test]
fn test_window() {
    let mut window = window::Window::new();

    let mut sq = Square::new(vec2(0., 0.), vec2(10., 10.));

    let mut mat = Material::default();
    
    let ds = DrawStream::from(sq);

    while !window.should_close() {
        window.clear(Color::BLUE);

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

        ds.draw(&mat);
        window.swap_buffers();
    }
}
