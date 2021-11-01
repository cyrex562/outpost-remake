use crate::point::Point;
use crate::vector2::Vector2;


pub fn line_intersects_circle(p: &Point<i64>, q: &Point<i64>, c: &Point<i64>, r: f64) -> bool {
    let mut center_to_start: Vector2<i64> = p - c;
    let mut line_size: Vector2<i64> = q - p;
    // pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T
    // const auto t = std::clamp<float>(-centerToStart.dotProduct(lineSize) / lineSize.lengthSquared(), 0, 1);
    let mut t = clamp(center_to_start.dot_product(line_size) * -1 / line_size.length_squared(), 0, 1);
    let mut min_distance = lin_size * t + center_to_start;
    min_distance.length_squared() < (r * r)
}

pub fn divide_up(to_divide: i64, divisor: i64) -> i64 {
    (to_divide + (divisor - 1)) / divisor
}

pub fn round_up_power_of_2(number: &mut u32) -> u32 {
    *number -= 1;
    *number |= *number >> 1;
    *number |= *number >> 2;
    *number |= *number >> 4;
    *number |= *number >> 8;
    *number |= *number >> 16;
    *number += 1;
    *number
}

pub fn scale_linear(value: &T, domain_point_1: &T, domain_point_2: &T, range_point_1: &U, range_point_2: &U) -> U {
    (value - domain_point_1) * (range_point_2 - range_point_1) / (domain_point_2 - domain_point_1) + range_point_1
}


