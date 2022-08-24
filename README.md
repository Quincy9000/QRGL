# QRGL - Quincy Rendering OpenGL Framework

Pretty much for right now the repo is to be an educational tool for learning opengl using the rust programming langauge for me. 

I will probably never accept any pull requests because I don't want code in here that I do not fully understand, or haven't typed out myself. 

The goal of this project is to make a tool for rendering graphics from scratch in opengl, and maybe other graphics apis if I have time. 

One inspiration to make this was how annoying it was to use other frameworks that had weird apis, and felt weird to use in rust. I want to make a consistient and feature full framework games could be made from without understanding any low level graphics programming. 

The main feature of this project will be the ease of use for the api. Performance and efficiency are an after thought right now because I want to understand the code first because I go ahead and optimize. However that doesn't mean I don't want to optimize the code. It's just not a top priority for this project. 

```rust
use qrgl::{math::color::*, math::vector::*, render::ogl::*, render::shapes::*, window::*};

fn main() {
    let mut window = Window::default();
    window.set_size(800, 600);
    window.set_title("Game");

    // make a buffer for drawing into
    let mut draw_streams = Vec::<(DrawStream, Material)>::new();

    // make some objects to draw, right now coordiantes are in NDC, this will change
    let square = Square::new(vec2(-0.25, -0.95), vec2(0.5, 0.6));
    let d1 = DrawStream::from(square);

    let circle = Circle::new(vec2(0., 0.3), 0.3);
    let d2 = DrawStream::from(circle);

    let triangle = Triangle::new(vec2(0., 0.9), vec2(-0.5, 0.4), vec2(0.5, 0.4));
    let d3 = DrawStream::from(triangle);

    // make re-useable materials
    let mut material1 = Material::default();
    material1.set_color(Color::rgb(255, 255, 0));

    let mut material2 = Material::default();
    material2.set_color(Color::rgb(0, 255, 255));

    let mut material3 = Material::default();
    material3.set_color(Color::rgb(255, 0, 255));

    draw_streams.push((d1, material1));
    draw_streams.push((d2, material2));
    draw_streams.push((d3, material3));

    while !window.should_close() {
        while let Some(e) = window.events() {
            if e.is_key_pressed(Key::Escape) {
                window.set_should_close(true);
            }
        }

        window.clear(Color::rgb(100, 149, 237));

        for d in &draw_streams {
            d.0.draw(&d.1);
        }

        window.swap_buffers();
    }
}

```