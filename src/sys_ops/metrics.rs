use crate::{mixed_fn_1010_830a, winapi_funcs};
use crate::draw::text;
use crate::other_funcs::{modify_list_1008_3f62, zero_list_1008_3e38};
use crate::pass::pass14_funcs::pass1_1008_3e94;
use crate::prog_structs::prog_structs_24::Struct2111;
use crate::prog_structs::prog_structs_29::Struct375;
use crate::prog_structs::prog_structs_2::Struct199;
use crate::prog_structs::prog_structs_4::Struct652;
use crate::struct_ops1::{process_struct_1008_4772, process_struct_1010_1d48};
use crate::struct_ops2::process_struct_1010_20ba;
use crate::typedefs::{HDC16, HWND16};
use crate::ui_ops::window;
use crate::util::{CONCAT22, SUB42};
use crate::winapi_funcs::{GetDC16, GetSystemMetrics16, GetWindowRect16, IsIconic16, ReleaseDC16};

pub unsafe fn get_sys_metrics_1010_46f6(param_1: u32) {
    let mut u_var1: u16;
    let i_var2: u16;
    let i_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let ppVar6: Struct2111;
    let mut u_var7: u32;
    let puVar8: u16;
    let pu_var9: Vec<u8>;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var9 = CONCAT22(unaff_ss, &local_4);
    puVar8 = &local_6;
    ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(puVar8, 0x48));
    pass1_1008_3e94((ppVar6 + 0xe), CONCAT22(unaff_ss, puVar8), pu_var9);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var7 = process_struct_1008_4772((i_var4 + 0x66));
    u_var1 = (u_var7 >> 0x10);
    (i_var4 + 0x18) = local_4 + 8;
    (i_var4 + 0x1a) = local_6 + 9;
    i_var2 = winapi_funcs::GetSystemMetrics16(5);
    (i_var4 + 0x1c) = i_var2 * 2 + (u_var7 + 4);
    i_var2 = winapi_funcs::GetSystemMetrics16(4);
    i_var3 = winapi_funcs::GetSystemMetrics16(6);
    (i_var4 + 0x1e) = i_var3 + i_var2 + (u_var7 + 8);
    return;
}

pub unsafe fn get_sys_metrics_1018_09a8(param_1: u32) {
    let mut u_var1: u32;
    let i_var2: u16;
    let i_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let ppVar6: Struct2111;
    let pu_var7: u16;
    let puVar8: Vec<u8>;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = winapi_funcs::GetSystemMetrics16(4);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    local_6 = (i_var4 + 0x12) - 2;
    puVar8 = CONCAT22(unaff_ss, &local_8);
    pu_var7 = &local_a;
    ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var7, 0x48));
    pass1_1008_3e94((ppVar6 + 0xe), CONCAT22(unaff_ss, pu_var7), puVar8);
    (i_var4 + 0x18) = local_6 * local_4 + local_8 + 0x146;
    (i_var4 + 0x1a) = local_6 * local_4 + local_a + 9;
    i_var2 = winapi_funcs::GetSystemMetrics16(5);
    u_var1 = (i_var4 + 0x5a);
    (i_var4 + 0x1c) = i_var2 * 2 + (u_var1 + 4);
    i_var2 = winapi_funcs::GetSystemMetrics16(4);
    i_var3 = winapi_funcs::GetSystemMetrics16(6);
    u_var1 = (i_var4 + 0x5a);
    (i_var4 + 0x1e) = i_var3 + i_var2 + (u_var1 + 8);
    return;
}

pub fn get_sys_metrics_1018_1ea0(param_1: u32) {
    let i_var1: u16;
    let i_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    i_var1 = winapi_funcs::GetSystemMetrics16(5);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0x2e) = i_var1 * 2 + (i_var3 + 0x36);
    i_var1 = winapi_funcs::GetSystemMetrics16(4);
    i_var2 = winapi_funcs::GetSystemMetrics16(6);
    (i_var3 + 0x30) = i_var1 + (i_var3 + 0x38) + i_var2;
    return;
}

pub unsafe fn get_sys_metrics_1018_2f56(param_1: u32) {
    let mut u_var1: u16;
    let i_var2: u16;
    let i_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let ppVar6: Struct2111;
    let mut u_var7: u32;
    let puVar8: u16;
    let pu_var9: Vec<u8>;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var9 = CONCAT22(unaff_ss, &local_4);
    puVar8 = &local_6;
    ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(puVar8, 0x48));
    pass1_1008_3e94((ppVar6 + 0xe), CONCAT22(unaff_ss, puVar8), pu_var9);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var7 = process_struct_1008_4772((i_var4 + 0x24));
    u_var1 = (u_var7 >> 0x10);
    (i_var4 + 0x18) = local_4 + 0xb5;
    (i_var4 + 0x1a) = local_6 + 9;
    i_var2 = winapi_funcs::GetSystemMetrics16(5);
    (i_var4 + 0x1c) = i_var2 * 2 + (u_var7 + 4);
    i_var2 = winapi_funcs::GetSystemMetrics16(4);
    i_var3 = winapi_funcs::GetSystemMetrics16(6);
    (i_var4 + 0x1e) = i_var3 + i_var2 + (u_var7 + 8);
    return;
}

pub fn get_sys_metrics_1018_4b1e(
    param_1: &mut Struct375,
    param_2: u16,
    param_3: u16,
) -> &mut Struct375 {
    let mut i_var1: i32;
    let mut u_var2: u16;

    process_struct_1010_1d48(param_1, param_3);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x12) = param_2;
    (i_var1 + 0x14) = 0;
    param_1.ptr_1_lo = &PTR_LOOP_1050_4c9e;
    (i_var1 + 2) = 0x1018;
    if (PTR_LOOP_1050_416c == 0x0) {
        PTR_LOOP_1050_416c = winapi_funcs::GetSystemMetrics16(4);
        PTR_LOOP_1050_416e = winapi_funcs::GetSystemMetrics16(5);
        PTR_LOOP_1050_4170 = winapi_funcs::GetSystemMetrics16(6);
    }
    return param_1;
}

pub fn get_system_metrics_1038_a18c(param_1: u32) {
    let pp_var1: fn();
    let paVar2: &mut Struct199;
    let i_var3: u16;

    let mut u_var4: u16;
    let unaff_ss: HWND16;
    let pu_var5: &mut u16;
    let h_var6: HWND16;
    let pu_var7: Vec<u8>;
    let HVar8: HWND16;
    let mut local_4c: u32;
    let mut local_48: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_28: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: [u8; 2];
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 6];
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_4c, 0x27));
    zero_list_1008_3e38(CONCAT22(unaff_ss, local_c));
    modify_list_1008_3f62(
        CONCAT22(unaff_ss, local_c),
        _local_6 & 0xffff0000 | (_local_6 + 0x52),
    );
    paVar2 = local_c;
    pass1_1008_3e94(
        paVar2,
        CONCAT22(unaff_ss, &local_10),
        CONCAT22(unaff_ss, &local_e),
    );
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1c0);
    _local_14 = CONCAT22(ctx.dx_reg, paVar2);
    _local_18 = process_struct_1008_4772(CONCAT22(ctx.dx_reg, paVar2));
    pu_var7 = local_1a;
    pu_var5 = &local_1c;
    h_var6 = unaff_ss;
    HVar8 = unaff_ss;
    _local_24 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var5, 0x48));
    pass1_1008_3e94(
        (_local_24 + 0xe),
        CONCAT22(h_var6, pu_var5),
        CONCAT22(HVar8, pu_var7),
    );
    u_var4 = (_local_24 >> 0x10);
    local_1e = (_local_24 + 10);
    local_20 = (_local_24 + 0xc);
    local_10 = local_10 + (local_20 * 10) / 600 + (_local_18 + 8) + local_1c;
    GetWindowRect16(CONCAT22(&local_2c, 0x1008), unaff_ss);
    i_var3 = GetSystemMetrics16(0);
    local_e = (i_var3 - (local_28 - local_2c)) / 2;
    window::move_window_1040_826c(param_1, (param_1 >> 0x10), local_10, local_e);
    local_4c._0_2_ = SUB42(_local_14, 0);
    local_4c._2_2_ = (_local_14 >> 0x10);
    if (_local_14 != 0x0) {
        pp_var1 = *_local_14;
        (**pp_var1)(
            &ctx.PTR_LOOP_1050_1040,
            local_4c,
            local_4c._2_2_,
            1,
            local_4c,
            local_4c._2_2_,
            _local_14,
        );
    }
    return;
}

pub fn get_sys_metrics_1040_8c66(param_1: u32) {
    let pi_var1: *mut i32;
    let mut b_var2: u8;
    let mut HVar3: HDC16;
    let mut i_var4: u16;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    i_var5 = param_1;
    HVar3 = GetDC16((i_var5 + 6));
    text::draw_text_1040_8d14(param_1, HVar3);
    (i_var5 + 0xa6) = (i_var5 + 0x9e);
    (i_var5 + 0xaa) = (i_var5 + 0xa2);
    i_var4 = GetSystemMetrics16(4);
    pi_var1 = (i_var5 + 0xac);
    unsafe { *pi_var1 = *pi_var1 + i_var4 };
    b_var2 = *(i_var5 + 0x98) & 0xf0;
    if ((((b_var2 == 0x30) || (b_var2 == 0x10)) || (b_var2 == 0x40)) || (b_var2 == 0x20)) {
        i_var4 = GetSystemMetrics16(0xc);
        if ((i_var5 + 0xac) < i_var4) {
            i_var4 = GetSystemMetrics16(0xc);
            (i_var5 + 0xac) = i_var4;
        }
    }
    pi_var1 = (i_var5 + 0xaa);
    unsafe {
        *pi_var1 = *pi_var1 + 0x14;
        pi_var1 = (i_var5 + 0xac);
        *pi_var1 = *pi_var1 + 10;
        (i_var5 + 0xb0) = (i_var5 + 0xac);
        pi_var1 = (i_var5 + 0xac);
        *pi_var1 = *pi_var1 + 0x30;
    }
    HVar3 = (i_var5 + 6);
    ReleaseDC16(HVar3, HVar3);
    return;
}

pub fn get_sys_metrics_1020_7a50(param_1: *mut Struct652) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let b_var3: bool;
    let mut i_var4: u16;
    let mut i_var5: u16;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: HWND16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut offset: u16;

    local_a = 0;
    local_8 = 0;
    local_6 = 0;
    local_4 = 0;
    u_var7 = (param_1 >> 0x10);
    i32_var6 = param_1;
    b_var3 = IsIconic16((i32_var6 + 8));
    if (b_var3 == 0) {
        GetWindowRect16(CONCAT22(&local_a, 0x1538), unaff_ss);
        local_6 = local_6 - local_a;
        local_4 = local_4 - local_8;
        i_var4 = GetSystemMetrics16(5);
        i_var5 = GetSystemMetrics16(6);
        local_a = local_a + i_var4 * 2;
        local_8 = local_8 + i_var5 * 2;
    }
    u_var1 = (i32_var6 + 0xe0);
    ppc_var2 = ((i32_var6 + 0xe0) + 0x14);
    ppc_var2(offset, u_var1, (u_var1 >> 0x10), &local_a);
    return;
}
