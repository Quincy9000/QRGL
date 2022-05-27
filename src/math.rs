use std::fmt::Debug;

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

pub trait SecondDimension: FirstDimension + Up + Down {}

pub trait ThirdDimension: SecondDimension + Forward + Backward {}

pub trait One {
    fn one() -> Self;
}

pub trait Zero {
    fn zero() -> Self;
}
struct If<const B: bool>;
impl True for If<true> {}

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

impl<T, const N: usize> FirstDimension for Vector<T, N>
where
    If<{ N > 0 }>: True,
    T: Default + Copy + Zero + One,
{
}

impl<T, const N: usize> Left for Vector<T, N>
where
    If<{ N > 0 }>: True,
    T: Default + Copy + Zero + One,
{
    fn left() -> Self {
        let mut v = [T::default(); N];
        v[0] = T::one();
        Self {
            v: [T::default(); N],
        }
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
        Self {
            v: [T::default(); N],
        }
    }
}

impl<T, const N: usize> Up for Vector<T, N>
where
    If<{ N > 1 }>: True,
    T: Default + Copy + Zero + One,
{
    fn up() -> Self {
        let mut v = [T::default(); N];
        v[0] = T::one();
        Self {
            v: [T::default(); N],
        }
    }
}

impl<T, const N: usize> Down for Vector<T, N>
where
    If<{ N > 1 }>: True,
    T: Default + Copy + Zero + One,
{
    fn down() -> Self {
        let mut v = [T::default(); N];
        v[0] = T::one();
        Self {
            v: [T::default(); N],
        }
    }
}

impl<T, const N: usize> Forward for Vector<T, N>
where
    If<{ N > 2 }>: True,
    T: Default + Copy + Zero + One,
{
    fn fwd() -> Self {
        let mut v = [T::default(); N];
        v[0] = T::one();
        Self {
            v: [T::default(); N],
        }
    }
}

impl<T, const N: usize> Backward for Vector<T, N>
where
    If<{ N > 2 }>: True,
    T: Default + Copy + Zero + One,
{
    fn back() -> Self {
        let mut v = [T::default(); N];
        v[0] = T::one();
        Self {
            v: [T::default(); N],
        }
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
    let v5 = Vec2::fwd();
    let v6 = Vec2::back();
    println!("{:?}", v);
    println!("{:?}", v2);
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
}
