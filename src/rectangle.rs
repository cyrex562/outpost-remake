use crate::point::Point;
use crate::vector2::Vector2;

#[derive(Clone,Debug,PartialOrd, PartialEq, Default)]
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

    fn create2(start_point: &Point<T>, end_point: &Point<T>) -> Rectangle<T> {
        Self::create(start_point, end_point - start_point)
    }

    fn size(&self) -> Vector2<T> {
        Vector2{x: self.width, y: self.height}
    }

    fn start_point(&self) -> Point<T> {
        Point{
            x: self.x,
            y: self.y
        }
    }

    fn end_point<T>(&self) -> Point<T> {
        let p: Point<T> = Point {
            x: self.x,
            y: self.y
        };
        let q: Point<T> = p + Vector2{x: self.width, y: self.height};

    }

    fn cross_x_point<T>(&self) -> Point<T> {
        Point {
            x: self.x + self.width,
            y: self.y,
        }
    }

    fn cross_y_point<T>(&self) -> Point<T> {
        Point {
            x: self.x,
            y: self.x + self.height,
        }
    }

    fn is_zero_dim(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    fn set_size<T>(&mut self, new_size: &Vector2<T>) {
        self.width = &new_size.x;
        self.height = &new_size.y;
    }

    fn set_start_point<T>(&mut self, new_start_point: &Point<T>) {
        self.x = &new_start_point.x;
        self.y = &new_start_point.y;
    }

    fn inset<T>(&self, amount: &T) -> Rectangle<T> {
        Rectangle {
            x: self.x + &amount,
            y: self.y + &amount,
            width: self.width - 2 * &amount,
            height: self.height - 2 * &amount,
        }
    }

    fn inset2<T>(&self,amount: &Vector2<T>) -> Rectangle<T> {
        Rectangle {
            x: self.x + &amount.x,
            y: self.y + &amount.y,
            width: self.width - 2 * &amount.x,
            height: self.height - 2 * &amount.y
        }
    }

    fn inset3<T>(&self, amount_start: &Vector2<T>, amount_end: &Vector2<T>) -> Rectangle<T> {
        Rectangle {
            x: self.x + &amount_start.x,
            y: self.y + &amount_start.y,
            width: self.width - &amount_start.x - &amount_end.x,
            height: self.height - &amount_start.y - &amount_end.y
        }
    }
}
