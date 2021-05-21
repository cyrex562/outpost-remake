use crate::sys_structs::POINT16;
use crate::typedefs::HDC16;
use crate::winapi_funcs::Polygon16;

pub fn polygon_1018_661c(param_1: u16, param_2: u16, in_count: u16, in_Paint16: *mut POINT16) {
    let mut hdc: HDC16;

    Polygon16(in_count, in_Paint16, hdc);
    return;
}

pub fn polygon_1020_3602(param_1: u16, param_2: u16, param_3: u32, param_4: HDC16) {
    Polygon16(param_1, (param_1 >> 0x10), param_2);
    return;
}

pub fn polygon_1020_2474(
    param_1: u16,
    param_2: u16,
    count: *mut POINT16,
    device_ctx_handle: HDC16,
) {
    Polygon16(count >> 0x10, device_ctx_handle, 0);
    return;
}
