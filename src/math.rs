use std::{
    fmt::Debug,
    ops::{Index, IndexMut, Neg},
};

pub type Vec2b = Vector<bool, 2>;
pub type Vec3b = Vector<bool, 3>;
pub type Vec4b = Vector<bool, 4>;

pub type Vec2i = Vector<i32, 2>;
pub type Vec3i = Vector<i32, 3>;
pub type Vec4i = Vector<i32, 4>;

pub type Vec2 = Vector<f32, 2>;
pub type Vec3 = Vector<f32, 3>;
pub type Vec4 = Vector<f32, 4>;

pub type Vec2d = Vector<f64, 2>;
pub type Vec3d = Vector<f64, 3>;
pub type Vec4d = Vector<f64, 4>;

pub struct X;
pub struct Y;
pub struct Z;
pub struct W;

pub mod vec2 {
    use super::*;
    pub const LEFT2: Vec2 = Vec2 { v: [-1., 0.] };
    pub const RIGHT2: Vec2 = Vec2 { v: [1., 0.] };
    pub const UP2: Vec2 = Vec2 { v: [0., 1.] };
    pub const DOWN2: Vec2 = Vec2 { v: [0., -1.] };
}

pub mod vec3 {
    use super::*;
    pub const LEFT3: Vec3 = Vec3 { v: [-1., 0., 0.] };
    pub const RIGHT3: Vec3 = Vec3 { v: [1., 0., 0.] };
    pub const UP3: Vec3 = Vec3 { v: [0., 1., 0.] };
    pub const DOWN3: Vec3 = Vec3 { v: [0., -1., 0.] };
    pub const FWD3: Vec3 = Vec3 { v: [0., 0., 1.] };
    pub const BACK3: Vec3 = Vec3 { v: [0., 0., -1.] };
}

trait True {}

pub trait Forward {
    fn fwd() -> Self;
}

pub trait Backward {
    fn back() -> Self;
}

pub trait Left {
    fn left() -> Self;
}

pub trait Right {
    fn right() -> Self;
}

pub trait Up {
    fn up() -> Self;
}

pub trait Down {
    fn down() -> Self;
}

pub trait FirstDimension: Left + Right {}

pub trait SecondDimension: Left + Right + Up + Down {}

pub trait ThirdDimension: Left + Right + Up + Down + Forward + Backward {}

pub trait One: Neg<Output = Self> {
    fn one() -> Self;
}

pub trait Zero {
    fn zero() -> Self;
}

struct If<const B: bool>;
impl True for If<true> {}

pub struct Vector<T, const N: usize> {
    v: [T; N],
}

impl<T, const N: usize> Debug for Vector<T, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector").field("v", &self.v).finish()
    }
}

impl One for i32 {
    fn one() -> Self {
        1
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
}

impl One for f32 {
    fn one() -> Self {
        1.0
    }
}

// impl<T, const N: usize> FirstDimension for Vector<T, N>
// where
//     If<{ N > 0 }>: True,
//     T: Default + Copy + Zero + One,
// {
// }

// impl<T, const N: usize> SecondDimension for Vector<T, N>
// where
//     If<{ N > 1 }>: True,
//     T: Default + Copy + Zero + One,
// {
// }

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

impl<T, const N: usize> Index<X> for Vector<T, N>
where
    If<{ N > 0 }>: True,
{
    type Output = T;

    fn index(&self, _: X) -> &Self::Output {
        &self.v[0]
    }
}

impl<T, const N: usize> Index<Y> for Vector<T, N>
where
    If<{ N > 1 }>: True,
{
    type Output = T;

    fn index(&self, _: Y) -> &Self::Output {
        &self.v[1]
    }
}

impl<T, const N: usize> Index<Z> for Vector<T, N>
where
    If<{ N > 2 }>: True,
{
    type Output = T;

    fn index(&self, _: Z) -> &Self::Output {
        &self.v[2]
    }
}

impl<T, const N: usize> Index<W> for Vector<T, N>
where
    If<{ N > 3 }>: True,
{
    type Output = T;

    fn index(&self, _: W) -> &Self::Output {
        &self.v[3]
    }
}

impl<T, const N: usize> IndexMut<X> for Vector<T, N>
where
    If<{ N > 0 }>: True,
{
    fn index_mut(&mut self, _: X) -> &mut Self::Output {
        &mut self.v[0]
    }
}

impl<T, const N: usize> IndexMut<Y> for Vector<T, N>
where
    If<{ N > 1 }>: True,
{
    fn index_mut(&mut self, _: Y) -> &mut Self::Output {
        &mut self.v[1]
    }
}

impl<T, const N: usize> IndexMut<Z> for Vector<T, N>
where
    If<{ N > 2 }>: True,
{
    fn index_mut(&mut self, _: Z) -> &mut Self::Output {
        &mut self.v[2]
    }
}

impl<T, const N: usize> IndexMut<W> for Vector<T, N>
where
    If<{ N > 3 }>: True,
{
    fn index_mut(&mut self, _: W) -> &mut Self::Output {
        &mut self.v[3]
    }
}

impl<T, const N: usize> Left for Vector<T, N>
where
    If<{ N > 0 }>: True,
    T: Default + Copy + Zero + One,
{
    fn left() -> Self {
        let mut v = [T::default(); N];
        v[0] = -T::one();
        Self { v }
    }
}

impl<T, const N: usize> Right for Vector<T, N>
where
    If<{ N > 0 }>: True,
    T: Default + Copy + Zero + One,
{
    fn right() -> Self {
        let mut v = [T::default(); N];
        v[0] = T::one();
        Self { v }
    }
}

impl<T, const N: usize> Up for Vector<T, N>
where
    If<{ N > 1 }>: True,
    T: Default + Copy + Zero + One,
{
    fn up() -> Self {
        let mut v = [T::default(); N];
        v[1] = T::one();
        Self { v }
    }
}

impl<T, const N: usize> Down for Vector<T, N>
where
    If<{ N > 1 }>: True,
    T: Default + Copy + Zero + One,
{
    fn down() -> Self {
        let mut v = [T::default(); N];
        v[1] = -T::one();
        Self { v }
    }
}

impl<T, const N: usize> Forward for Vector<T, N>
where
    If<{ N > 2 }>: True,
    T: Default + Copy + Zero + One,
{
    fn fwd() -> Self {
        let mut v = [T::default(); N];
        v[2] = T::one();
        Self { v }
    }
}

impl<T, const N: usize> Backward for Vector<T, N>
where
    If<{ N > 2 }>: True,
    T: Default + Copy + Zero + One,
{
    fn back() -> Self {
        let mut v = [T::default(); N];
        v[2] = -T::one();
        Self { v }
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default,
    T: Copy,
{
    fn default() -> Self {
        Self {
            v: [T::default(); N],
        }
    }
}

impl<T, const N: usize> Clone for Vector<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self { v: self.v.clone() }
    }
}
impl<T, const N: usize> Copy for Vector<T, N> where T: Copy {}

impl<T, const N: usize> Vector<T, N> {
    pub const fn len() -> usize {
        N
    }
}

#[test]
fn test_2d() {
    let v = Vec2::left();
    let v2 = Vec2::right();
    let v3 = Vec2::up();
    let v4 = Vec2::down();
    // let v5 = Vec2::fwd();
    // let v6 = Vec2::back();
    println!("{:?}", v);
    println!("{:?}", v2);
    println!("{:?}", v3);
    println!("{:?}", v4);

    println!("X: {}, Y: {}, Z: N/A, W: N/A", v[X], v[Y]);
}

#[test]
fn test_3d() {
    let v = Vec3::left();
    let v2 = Vec3::right();
    let v3 = Vec3::up();
    let v4 = Vec3::down();
    let v5 = Vec3::fwd();
    let v6 = Vec3::back();
    println!("{:?}", v);
    println!("{:?}", v2);
    println!("{:?}", v3);
    println!("{:?}", v4);
    println!("{:?}", v5);
    println!("{:?}", v6);

    println!("X: {}, Y: {}, Z: {}, W: N/A", v[X], v[Y], v[Z]);
}

#[test]
fn test_4d() {
    let v1 = Vec4::left();
    let v2 = Vec4::right();
    let v3 = Vec4::up();
    let v4 = Vec4::down();
    let v5 = Vec4::fwd();
    let v6 = Vec4::back();
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
    println!("{:?}", v4);
    println!("{:?}", v5);
    println!("{:?}", v6);

    println!("X: {}, Y: {}, Z: {}, W: {}", v1[X], v1[Y], v1[Z], v1[W]);
}
