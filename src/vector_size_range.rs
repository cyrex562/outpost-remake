use crate::vector2::Vector2;

#[derive(Clone,Debug,Default,PartialOrd,PartialEq,Iterator)]
pub struct Vector2SizeRange {
    pub m_size: Vector2<T>,
}

impl Vector2SizeRange {
    fn new(size: Vector2<T>) -> Self {
        Self {
            m_size: size,
        }
    }
}
