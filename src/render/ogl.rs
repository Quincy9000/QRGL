use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::hash::Hash;

pub trait Bindable {
    fn bind(&self);
}

pub trait Uniform {
    fn apply_uniform(&mut self, loc: i32);
}

use gl::*;

use crate::math::color::Color;
use crate::math::vector::*;
use crate::render::shaders;

use crate::math::*;

use super::shapes::Shape;

// TODO: we need to put more meta data on the buffer types because they do not store enough information to make them more efficient

pub struct Vbo {
    id: u32,
    size: usize,
}

impl Vbo {
    pub fn new() -> Self {
        let vbo = unsafe {
            let mut v = 0;
            gl::GenBuffers(1, &mut v);
            v
        };
        Self { id: vbo, size: 0 }
    }

    pub fn new_bind() -> Self {
        let v = Vbo::new();
        v.bind();
        v
    }

    pub fn new_bind_buffer<T>(data: &[T]) -> Self {
        let mut v = Vbo::new();
        v.set_data(&data);
        v
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn set_data<T>(&mut self, data: &[T]) {
        unsafe {
            self.bind();
            self.size = data.len();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<T>() * self.size) as isize,
                data.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::DeleteBuffers(1, &self.id)
        }
    }
}

pub trait VertexInfo {
    fn dims(&self) -> u32;
    fn kind(&self) -> u32;
    fn size(&self) -> usize;
    fn normalized(&self) -> u8;
}

pub struct VertexPosInfo2D;
impl VertexInfo for VertexPosInfo2D {
    fn dims(&self) -> u32 {
        2
    }

    fn size(&self) -> usize {
        std::mem::size_of::<f32>() * self.dims() as usize
    }

    fn kind(&self) -> u32 {
        gl::FLOAT
    }

    fn normalized(&self) -> u8 {
        gl::FALSE
    }
}

pub struct VertexColorInfo;
impl VertexInfo for VertexColorInfo {
    fn dims(&self) -> u32 {
        3
    }

    fn size(&self) -> usize {
        std::mem::size_of::<f32>() * self.dims() as usize
    }

    fn kind(&self) -> u32 {
        gl::FLOAT
    }

    fn normalized(&self) -> u8 {
        gl::FALSE
    }
}

pub struct Vao {
    id: u32,
    info: Vec<&'static dyn VertexInfo>,
}

impl Vao {
    pub fn new() -> Self {
        Self {
            id: unsafe {
                let mut vao = 0;
                gl::GenVertexArrays(1, &mut vao);
                vao
            },
            info: Vec::new(),
        }
    }

    pub fn new_bind() -> Self {
        let v = Vao::new();
        v.bind();
        v
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }

    pub fn add_attribute<T: VertexInfo>(&mut self, vbo: &Vbo, data: &'static T) {
        self.bind();
        vbo.bind();
        self.info.push(data);

        let mut stride = 0;
        for data in &self.info {
            stride += data.size() as i32;
        }

        for (i, data) in self.info.iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(
                    i as u32,
                    data.dims() as i32,
                    data.kind(),
                    data.normalized(),
                    stride as i32,
                    (i * data.size()) as *const _,
                );
            }
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            // TODO! These can be potentially buggy.
            // just because we are dropping a vao, doesnt mean its the current bound vao.
            // we need to find a way to check if this vao is bound currently

            // this goes for every type of buffer that we make
            // vbo, vao, ebo.. etc
            gl::BindVertexArray(0);
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

pub struct Ebo {
    id: u32,
    size: usize,
}

impl Ebo {
    pub fn new() -> Self {
        Self {
            id: unsafe {
                let mut id = 0;
                gl::GenBuffers(1, &mut id);
                id
            },
            size: 0,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn set_data(&mut self, data: &Vec<u32>) {
        self.bind();
        self.size = std::mem::size_of::<u32>() * data.len();
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                self.size as isize,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            )
        }
    }

    pub fn gen_indices<T>(&self, data: &Vec<T>) -> Vec<u32>
    where
        T: Copy,
    {
        let mut indices = Vec::new();

        if data.len() == 0 {
            return indices;
        }

        let last = &data[0];

        let mut map = HashMap::<u32, T>::new();

        for i in 0..data.len() {
            map.insert(i as u32, data[i]);
        }

        for i in 0..data.len() {
            let kv = map.get_key_value(&(i as u32));
            if let Some(index) = kv {
                indices.push(*index.0);
            }
        }

        println!("{indices:?}");

        indices
    }
}

impl Drop for Ebo {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::DeleteBuffers(1, &self.id)
        }
    }
}

pub enum ShaderType {
    Vertex(&'static str),
    Fragment(&'static str),
}

impl ShaderType {
    pub fn get_type(&self) -> gl::types::GLuint {
        match self {
            ShaderType::Vertex(_) => gl::VERTEX_SHADER,
            ShaderType::Fragment(_) => gl::FRAGMENT_SHADER,
        }
    }

    pub fn get_source(&self) -> &'static str {
        match self {
            ShaderType::Vertex(s) => s,
            ShaderType::Fragment(s) => s,
        }
    }
}

pub struct ShaderBuilder {
    program: u32,
    shaders: Vec<u32>,
}

impl ShaderBuilder {
    pub fn new() -> Self {
        Self {
            program: unsafe { gl::CreateProgram() },
            shaders: Vec::new(),
        }
    }

    pub fn add_shader(&mut self, shader_type: ShaderType) {
        let shader_id = unsafe { gl::CreateShader(shader_type.get_type()) };
        let source = CString::new(shader_type.get_source()).unwrap();
        unsafe {
            gl::ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);
        }

        self.shaders.push(shader_id);
    }

    pub fn build(mut self) -> Shader {
        unsafe {
            for s in &self.shaders {
                gl::AttachShader(self.program, *s);
            }

            gl::LinkProgram(self.program);

            for s in &self.shaders {
                gl::DetachShader(self.program, *s);
            }

            for s in &self.shaders {
                gl::DeleteShader(*s);
            }

            self.shaders.clear();
        }

        Shader {
            program: self.program,
        }
    }
}

pub struct Shader {
    program: u32,
}

impl Shader {
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn set_uniform<T: Uniform>(&mut self, name: &str, mut uniform: T) {
        self.bind();
        unsafe {
            let a = CString::new(name).expect("Failed to make CString from Shader set_uniform");
            let loc = gl::GetUniformLocation(self.program, a.as_ptr());
            uniform.apply_uniform(loc);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}

pub struct Material {
    pub shader: Shader,
}

impl Material {
    pub fn set_color(&mut self, color: Color) {
        unsafe {
            self.shader.set_uniform("color", color);
        }
    }

    pub fn bind(&self) {
        self.shader.bind();
    }
}

impl Default for Material {
    fn default() -> Self {
        let mut shader_builder = ShaderBuilder::new();
        shader_builder.add_shader(shaders::BASIC_VERTEX_2D);
        shader_builder.add_shader(shaders::BASIC_FRAGMENT_2D);

        let mut shader = shader_builder.build();
        shader.bind();
        shader.set_uniform("color", Color::WHITE);

        Self { shader }
    }
}

pub struct DrawStream {
    vao: Vao,
    vbo: Vbo,
    ebo: Ebo,
}

impl DrawStream {
    pub fn draw(&self, mat: &Material) {
        mat.bind();
        self.vao.bind();

        // 2 because its 2d, each vertex is xy, but this wont be the case if we store color date in each vertex as well
        // it should be : get_size() / vertex.size()
        // let size = self.vbo.get_size() as i32 / 2;
        unsafe {
            //gl::DrawArrays(gl::TRIANGLES, 0, size);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

impl<T: Shape> From<T> for DrawStream {
    fn from(mut s: T) -> Self {
        // step0: bind vao
        let mut vao = Vao::new_bind();

        // step1: get verts
        let mut verts = s.get_arrays();

        // step2: make ebo indices from verts
        let mut ebo = Ebo::new();

        // step3: get indice list from verts
        // let indices = vec![0, 1, 3, 1, 2, 3]; hard coded way
        let indices = ebo.gen_indices(&verts);
        ebo.set_data(&indices);

        // step4: remove duplicate verts
        let mut vbo = Vbo::new_bind_buffer(&verts);
        s.set_attributes(&mut vbo, &mut vao);

        // step5: return
        Self { vbo, vao, ebo }
    }
}
