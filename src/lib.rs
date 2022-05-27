#![feature(trivial_bounds)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(const_generics_defaults)]
#![feature(generic_arg_infer)]

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
