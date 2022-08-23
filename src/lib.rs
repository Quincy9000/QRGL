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

    let mut sq = Square::new(vec2(-0.25, -0.25), vec2(0.5, 0.5));
    let ds = DrawStream::from(sq);

    let mut mat = Material::default();
    mat.set_color(Color::rgb(0, 200, 0));

    while !window.should_close() {
        while let Some(e) = window.events() {
            if e.is_key_pressed(Key::Escape) {
                window.set_should_close(true);
            }

            if let WindowEvent::Size(x, y) = e.events {
                window.update_viewport();
            }
        }

        window.clear(Color::rgb(100, 149, 237));
        ds.draw(&mat);
        window.swap_buffers();
    }
}
