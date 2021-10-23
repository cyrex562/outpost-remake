use std::ops::{AddAssign, SubAssign, Sub, Add};
use crate::vector2::Vector2;


#[derive(Clone,Debug,PartialEq,Default,PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl AddAssign for Point<T> {
    fn add_assign(&mut self, other: &Vector2<T>) {
        *self = self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl SubAssign for Point<T> {
    fn sub_assign(&mut self, other: &Vector2<T>) {
        *self = self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Add for Point<T> {
    type Output = Self;

    fn add(&mut self, other: &Vector2<T>) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point<T> {
    type Output = Self;

    fn sub(&mut self, other: &Vector2<T>) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Point<T> {
    fn skew_by(&mut self, other: &Vector2<T>) -> Point<T> {
        Point {
            x: other.x * self.x,
            y: other.y * self.y,
        }
    }

    fn skew_inverse_by(&mut self, other: &Vector2<T>) -> Point<T> {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Into<T> for Point<U> {
    fn into(self) -> Point<T> {
        Point {
            x: self.x as T,
            y: self.y as T,
        }
    }
}

// TODO:
// Commutative Vector + Point
// 	template <typename BaseType>
// 	constexpr Point<BaseType> operator+(Vector<BaseType> vector, Point<BaseType> point)
// 	{
// 		return point + vector;
// 	}
