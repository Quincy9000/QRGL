#![allow(unused)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod math;
pub mod render;

use math::{color::Color, vector::*};
use render::{
    cam,
    ogl::*,
    shaders,
    shapes::{Circle, Square, Triangle},
    window::*,
};
use std::{any::Any, cell::RefCell, rc::Rc};

#[test]
fn test_window() {
    fn setup() -> Vec<(DrawStream, Material)> {
        let mut sq = Square::new(vec2(-0.25, -0.95), vec2(0.5, 0.6));
        let d1 = DrawStream::from(sq);

        let mut ci = Circle::new(vec2(0., 0.3), 0.3);
        let d2 = DrawStream::from(ci);

        let mut tri = Triangle::new(vec2(0., 0.9), vec2(-0.5, 0.4), vec2(0.5, 0.4));
        let d3 = DrawStream::from(tri);

        let mut draws = Vec::<(DrawStream, Material)>::new();

        {
            let mut mat1 = Material::default();
            mat1.set_color(Color::rgb(255, 255, 0));

            let mut mat2 = Material::default();
            mat2.set_color(Color::rgb(0, 255, 255));

            let mut mat3 = Material::default();
            mat3.set_color(Color::rgb(255, 0, 255));

            draws.push((d1, mat1));
            draws.push((d2, mat2));
            draws.push((d3, mat3));
        }

        draws
    }

    let mut window = Window::new("Game", 800, 800);

    let mut draws: Vec<(DrawStream, Material)> = setup();

    while !window.should_close() {
        while let Some(e) = window.events() {
            if e.is_key_pressed(Key::Escape) {
                window.set_should_close(true);
            }
        }

        window.clear(Color::rgb(100, 149, 237));
        for d in &draws {
            d.0.draw(&d.1);
        }
        window.swap_buffers();
    }
}
