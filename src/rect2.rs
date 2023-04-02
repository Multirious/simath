use super::*;
use vec2::Vec2;

pub struct Rect2<T> {
    pub position: Vec2<T>,
    pub size: Vec2<T>,
}

impl<T> Rect2<T> {
    pub const fn from_borders(x: T, y: T, width: T, height: T) -> Rect2<T> {
        Rect2 {
            position: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }

    pub const fn new(position: Vec2<T>, size: Vec2<T>) -> Rect2<T> {
        Rect2 { position, size }
    }
}
