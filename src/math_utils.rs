use crate::point::Point;

pub fn line_intersects_circle(p: &Point<i64>, q: &Point<i64>, c: &Point<i64>, r: f64) -> bool {
    let mut center_to_start = p - c;
    let mut line_size = q - p;
    // const auto t = std::clamp<float>(-centerToStart.dotProduct(lineSize) / lineSize.lengthSquared(), 0, 1);
    let mut t =
}
