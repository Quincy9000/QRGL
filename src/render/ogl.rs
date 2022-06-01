use std::{
    ffi::{CStr, CString},
    mem::size_of,
};

pub trait Bindable {
    fn bind(&self);
}

pub trait Uniform {
    fn apply_uniform(&mut self);
}

use gl::*;

pub struct Vbo {
    id: u32,
}

impl Vbo {
    pub fn new() -> Self {
        let vbo = unsafe {
            let mut v = 0;
            gl::GenBuffers(1, &mut v);
            v
        };
        Self { id: vbo }
    }

    pub fn set_data<T>(&mut self, data: &[T]) {
        unsafe {
            self.bind();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of::<T>() as isize * data.len() as isize,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn bind(&mut self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

pub trait VertexInfo {
    fn dims(&self) -> i32;
    fn data_type(&self) -> u32;
    fn size(&self) -> usize;
    fn normalized(&self) -> u8;
    fn offset(&self) -> u32;
}

pub struct VertexPosInfo;
impl VertexInfo for VertexPosInfo {
    fn dims(&self) -> i32 {
        3
    }

    fn data_type(&self) -> u32 {
        gl::FLOAT
    }

    fn size(&self) -> usize {
        std::mem::size_of::<f32>()
    }

    fn normalized(&self) -> u8 {
        gl::FALSE
    }

    fn offset(&self) -> u32 {
        0
    }
}

pub struct VertexColorInfo;
impl VertexInfo for VertexColorInfo {
    fn dims(&self) -> i32 {
        3
    }

    fn size(&self) -> usize {
        std::mem::size_of::<f32>()
    }

    fn data_type(&self) -> u32 {
        gl::FLOAT
    }

    fn normalized(&self) -> u8 {
        gl::FALSE
    }

    fn offset(&self) -> u32 {
        self.dims() as u32 * self.size() as u32
    }
}

pub struct Vao<'a> {
    id: u32,
    info: Vec<&'a dyn VertexInfo>,
}

impl<'a> Vao<'a> {
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

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn add_attribute<T: VertexInfo>(&mut self, data: &'a T) {
        self.info.push(data);

        let mut stride = 0;
        for data in &self.info {
            stride += data.size() as i32 * data.dims();
        }

        for (i, data) in self.info.iter().enumerate() {
            unsafe {
                gl::VertexAttribPointer(
                    i as u32,
                    data.dims(),
                    data.data_type(),
                    data.normalized(),
                    stride as i32,
                    data.offset() as *const std::os::raw::c_void,
                );
                gl::EnableVertexAttribArray(i as u32);
            }
        }
    }
}

pub enum ShaderType {
    Vertex,
    Fragment,
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

    pub fn add_shader(&mut self, source: &str, shader_type: ShaderType) {
        let cstr = CString::new(source).expect("Failed to make cstring from add_shader");

        let shader_id = unsafe {
            gl::CreateShader(match shader_type {
                ShaderType::Vertex => gl::VERTEX_SHADER,
                ShaderType::Fragment => gl::FRAGMENT_SHADER,
            })
        };

        unsafe {
            gl::ShaderSource(shader_id, 1, &cstr.as_ptr(), std::ptr::null());
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
        unsafe {
            let a = CString::new(name).expect("Failed to make CString from uniform trait");
            let loc = gl::GetUniformLocation(0, a.as_ptr());
            uniform.apply_uniform();
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

pub fn draw() {
    unsafe {
        gl::ClearColor(0.0, 0.1, 0.2, 1f32);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}
