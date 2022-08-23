use crate::math::vector::{Vec2, *};
use crate::render::ogl::Vbo;

use super::ogl::{Material, Vao, VertexPosInfo};

pub trait Shape {
    fn get_arrays(&self) -> Vec<f32>;

    fn set_attributes(&mut self, vao: &Vao){}
}

pub struct Square {
    pos: Vec2,
    size: Vec2,
}

impl Square {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }
}

pub struct Circle {
    pub pos: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(pos: Vec2, radius: f32) -> Self {
        Self { pos, radius }
    }
}

pub struct Triangle {
    pos1: Vec2,
    pos2: Vec2,
    pos3: Vec2,
}

impl Triangle {
    pub fn new(pos1: Vec2, pos2: Vec2, pos3: Vec2) -> Self {
        Self { pos1, pos2, pos3 }
    }
}

impl Shape for Circle {
    fn get_arrays(&self) -> Vec<f32> {
        todo!("Make the circle function")
    }
}

impl Shape for Square {
    #[rustfmt::skip]
    fn get_arrays(&self) -> Vec<f32> {
        // todo: will optimize this later for using element buffers, for now just two tris
        // vec![
        //     // firs tri
        //     self.pos[X], self.pos[Y], 
        //     self.pos[X] + self.size[X], self.pos[Y],
        //     self.pos[X], self.pos[Y] + self.size[Y],
        //     // second try
        //     self.pos[X] + self.size[X], self.pos[Y],
        //     self.pos[X] + self.size[X], self.pos[Y] + self.size[Y],
        //     self.pos[X], self.pos[Y] + self.size[Y] 
        // ]
        vec![
            0.0, 0.0, 
            0.2, 0.0, 
            0.0, 0.2,
            //
            0.2, 0.0,
            0.2, 0.2,
            0.0, 0.2
        ]
    }
}

impl Shape for Triangle {
    #[rustfmt::skip]
    fn get_arrays(&self) -> Vec<f32> {
        // todo: will optimize this later for using element buffers, for now just two tris
        vec![
            // firs tri
            self.pos1[X], self.pos1[Y], 
            self.pos1[X], self.pos1[Y],
            // second try
            self.pos2[X], self.pos2[Y], 
            self.pos2[X], self.pos2[Y],
            // third tri
            self.pos3[X], self.pos3[Y], 
            self.pos3[X], self.pos3[Y],
        ]
    }
}
