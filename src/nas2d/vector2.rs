use std::ops::{AddAssign, Div, DivAssign, Mul, MulAssign, SubAssign};

#[derive(Clone,Debug,Default,PartialOrd,PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl Vector2<T> {
    fn new<T>() -> Vector2<T> {
        Vector2 {
            x: 0,
            y: 0,
        }
    }

    fn skew_by(self, other: &Self) -> Vector2<T> {
        Vector2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    fn skew_inverse_by(self, other: &Self) -> Vector2<T> {
        Vector2 {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }

    fn length_squared(self) -> T {
        (self.x * self.x) + (self.y * self.y)
    }

    fn dot_product(self, other: &Self) -> T {
        (self.x * other.x) + (self.y * other.y)
    }

    fn reflect_x(self) -> Vector2<T> {
        Self {
            x: self.x * -1,
            y: self.y
        }
    }

    fn reflect_y(self) -> Vector2<T> {
        Self {
            x: self.x,
            y: self.y* -1,
        }
    }
}

impl Into<T> for Vector2<U> {
    fn into(self) -> Self {
        Self {
            x: self.x as T,
            y: self.y as T,
        }
    }
}

impl AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: &Self) {
        *self = self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl MulAssign for Vector2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl DivAssign for Vector2<T> {
    fn div_assign(&mut self, other: Self) {
        *self = self {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

impl Mul for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// TODO:
// vector/scalar op
// 	template <typename BaseType>
// 	constexpr Vector<BaseType> operator*(BaseType scalar, Vector<BaseType> vector)
// 	{
// 		return vector * scalar;
// 	}


