use std::iter::Sum;

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: u32,
    pub y: u32,
}

impl Vector2 {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl Sum for Vector2{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector2::new(0, 0), |a, b| {
            Vector2::new(a.x+b.x, a.y+b.y)
        })
    }
}

#[derive(Clone, Copy)]
pub struct Rectangle {
    pub pos: Vector2,
    pub size: Vector2,
}

impl Rectangle {
    pub fn new(pos: Vector2, size: Vector2) -> Self {
        Self { pos, size }
    }
}
