#![allow(unused)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod math;
mod window;
use window::*;

#[test]
fn test_window() {
    let mut w = window::Window::new();

    while !w.should_close() {
        w.swap_buffers();
        while let Some(e) = w.events() {
            if e.is_key_pressed(Key::Escape) {
                w.set_should_close(true);
            }
        }
    }
}
