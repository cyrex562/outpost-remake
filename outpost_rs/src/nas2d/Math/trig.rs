use crate::nas2d::Math::vector2::Vector2;

pub const PI: f32 = 3.14159265;
pub const PI_2: f32 = PI * 2;
pub const DEG2RAD: f32 = PI / 180;
pub const RAD2DEG: f32 = 180 / PI;

pub fn deg_to_rad(degree: f32) -> f32 {
    degree * DEG2RAD
}

pub fn rad_to_deg(rad: f32) -> f32 {
    rad * RAD2DEG
}

pub fn get_angle(direction: Vector2<f32>) -> f32 {
    direction.y.atan2(direction.x)
}

pub fn get_direction_vector(radian: f32) -> Vector2<f32> {
    Vector2 {
        x: radian.cos(),
        y: radian.sin()
    }
}


