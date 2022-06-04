#![allow(unused)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod math;
mod render;
mod window;
use math::vector::*;
use render::{basic_shaders, ogl::*};
use window::*;

#[test]
fn test_window() {
    let mut window = window::Window::new();

    let mut vao = Vao::new();
    vao.bind();
    let mut vbo = Vbo::new();
    #[rustfmt::skip]
    vbo.set_data(&[
        // first 3 are pos, second 3 are color data
        -0.5f32,  -0.5f32, 0f32, /**/   /* 0f32,  1f32, 0f32, */
         0f32,     0.5f32, 0f32, /**/   /* 1f32,  0f32, 0f32, */
         0.5f32,  -0.5f32, 0f32, /**/   /* 0f32,  0f32, 1f32, */
    ]);
    vao.add_attribute(&VertexPosInfo);
    // vao.add_attribute(&VertexColorInfo);

    let mut shader_builder = ShaderBuilder::new();
    shader_builder.add_shader(basic_shaders::BASIC_VERTEX, ShaderType::Vertex);
    shader_builder.add_shader(basic_shaders::BASIC_FRAG, ShaderType::Fragment);

    let mut shader = shader_builder.build();
    shader.bind();
    shader.set_uniform("color", Vec3::from([1f32, 1f32, 0f32]));

    while !window.should_close() {
        window.swap_buffers();

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

        draw();
    }
}
