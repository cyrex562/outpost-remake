use std::ops::{AddAssign, SubAssign, Sub, Add};

#[derive(Clone,Debug,PartialEq,Default,PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        *self = self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl SubAssign for Point<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Add for Point<T> {
    type Output = Self;

    fn add(&mut self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point<T> {
    type Output = Self;

    fn sub(&mut self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Point<T> {
    fn skew_by(&mut self, other: Self) -> Point<T> {
        Point {
            x: other.x * self.x,
            y: other.y * self.y,
        }
    }

    fn skew_inverse_by(&mut self, other: Self) -> Point<T> {
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

// Vector + (append) Point
// TODO: no equivalent

// TODO?
// template <typename BaseType>
// 	Point(BaseType, BaseType) -> Point<BaseType>;

// pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
// where
//     Rhs: ?Sized,
// {
//     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
//
//     fn lt(&self, other: &Rhs) -> bool { ... }
//     fn le(&self, other: &Rhs) -> bool { ... }
//     fn gt(&self, other: &Rhs) -> bool { ... }
//     fn ge(&self, other: &Rhs) -> bool { ... }
// }
