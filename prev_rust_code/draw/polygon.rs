use crate::sys_structs::POINT16;
use crate::typedefs::HDC16;
use crate::winapi::Polygon16;

pub fn polygon_1018_661c(param_2: &HDC16, in_count: i16, param_struct4: &mut POINT16) -> bool {
    return Polygon16(param_2, param_struct4, in_count);
}

pub fn polygon_1020_3602(param_1: i16, param_2: &HDC16, param_3: &mut POINT16, param_4: HDC16) {
    Polygon16(param_2, param_3, param_1);
    return;
}

pub fn polygon_1020_2474(
    param_2: i16,
    point_param_3: &mut POINT16,
    hdc_param_4: &HDC16,
) {
    Polygon16(hdc_param_4, point_param_3, param_2);
    return;
}
