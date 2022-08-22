pub trait Shape {}

pub struct Square {}

pub struct Circle {}

pub struct Triangle {}

impl Shape for Circle {}

impl Shape for Square {}

impl Shape for Triangle {}

pub struct Drawable<'a> {
    draw: &'a dyn Shape,
}
