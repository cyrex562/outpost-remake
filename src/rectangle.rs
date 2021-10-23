use crate::point::Point;
use crate::vector2::Vector2;

pub struct Rectangle<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl Rectangle<T> {
    fn create(start_point: &Point<T>, size: &Vector2<T>) -> Rectangle<T> {
        Self {
            x: start_point.x,
            y: start_point.y,
            width: size.x,
            height: size.y
        }
    }
}
