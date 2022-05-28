use crate::math::vector::*;

pub type Mat2d = Matrix<f32, 3, 3>;
pub type Mat3d = Matrix<f32, 4, 4>;

pub struct Matrix<T, const C: usize, const R: usize> {
    v: [Vector<T, R>; C],
}

impl<T, const C: usize, const R: usize> Matrix<T, C, R> {
    
}
