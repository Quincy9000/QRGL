#![allow(unused)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod math;
mod render;
mod window;
use math::vector::*;
use render::{ogl::*, shaders};
use window::*;

#[test]
fn test_window() {
    let mut window = window::Window::new();

    let mut mat = Material::default();

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

        window.swap_buffers();
        
    }
}
