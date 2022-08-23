use crate::math::vector::{Vec2, *};
use crate::render::ogl::Vbo;

use super::ogl::{Material, Vao, VertexPosInfo2D};

pub trait Shape {
    fn get_arrays(&self) -> Vec<f32>;

    fn set_attributes(&mut self, vbo: &Vbo, vao: &mut Vao);
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

    pub const fn resolution() -> usize {
        16
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

        let (cx, cy) = (self.pos[X], self.pos[Y]);

        let res = Circle::resolution();
        let mut v = Vec::new();
        let step = (std::f32::consts::TAU) / (res as f32);

        let len = self.radius;

        let mut last_point = (1f32, 0f32);

        for i in 1..res+1 {
            let turn = i as f32 * step;

            let (y, x) = turn.sin_cos();

            v.push(cx);
            v.push(cy);

            v.push(last_point.0 * len);
            v.push(last_point.1 * len);

            v.push(x * len);
            v.push(y * len);

            last_point = (x, y);
        }

        v
    }

    fn set_attributes(&mut self, vbo: &Vbo, vao: &mut Vao) {
        vao.add_attribute(vbo, &VertexPosInfo2D);
    }
}

impl Shape for Square {
    #[rustfmt::skip]
    fn get_arrays(&self) -> Vec<f32> {
        // todo: will optimize this later for using element buffers, for now just two tris
        vec![
            // firs tri
            self.pos[X], self.pos[Y], 
            self.pos[X] + self.size[X], self.pos[Y],
            self.pos[X], self.pos[Y] + self.size[Y],
            // second try
            self.pos[X] + self.size[X], self.pos[Y],
            self.pos[X] + self.size[X], self.pos[Y] + self.size[Y],
            self.pos[X], self.pos[Y] + self.size[Y] 
        ]
    }

    fn set_attributes(&mut self, vbo: &Vbo, vao: &mut Vao) {
        vao.add_attribute(vbo, &VertexPosInfo2D);
    }
}

impl Shape for Triangle {
    #[rustfmt::skip]
    fn get_arrays(&self) -> Vec<f32> {
        vec![
            self.pos1[X], self.pos1[Y], 
            self.pos2[X], self.pos2[Y], 
            self.pos3[X], self.pos3[Y], 
        ]
    }
    
    fn set_attributes(&mut self, vbo: &Vbo, vao: &mut Vao) {
        vao.add_attribute(vbo, &VertexPosInfo2D);
    }
}
