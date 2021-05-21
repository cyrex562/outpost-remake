use crate::pass::pass8_funcs::pass1_1010_2ee2;
use crate::prog_structs::prog_structs_29::Struct136;
use crate::prog_structs::prog_structs_30::Struct137;
use crate::prog_structs::prog_structs_31::{Struct138, Struct139};
use crate::prog_structs::prog_structs_7::Struct135;
use crate::typedefs::HDC16;
use crate::util::ZEXT24;
use crate::winapi_funcs::{GetCurrentPosition16, LineTo16, MoveTo16};

pub unsafe fn draw_lines_1040_c38e(in_Struct135: *mut Struct135, in_hdc_2: u16) {
    let mut i_var1: i32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let local_a_x_262: *mut Struct137;
    let pi_var6: *mut u16;
    let mut extraout_d_x: u16;
    let local_Struct135: *mut Struct135;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut d_var9: u32;
    let mut curr_pos_10: u32;
    let mut local_1e: u16;
    let mut counter: u16;
    let mut local_18: u16;
    let mut x_coord: u16;
    let mut y_coord: u16;
    let mut local_10: u16;
    let local_e: *mut Struct136;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = (in_Struct135 >> 0x10);
    local_Struct135 = in_Struct135;
    u_var2 = local_Struct135.field_0x6;
    i_var1 = (u_var2 + 0x18);
    if ((i_var1 != 0) && (u_var2 = local_Struct135.field_0x6, (u_var2 + 0x16) != 0)) {
        i_var4 = i_var1;
        pass1_1010_2ee2(local_Struct135.field_0x6);
        counter = 0;
        while (counter < i_var1) {
            u_var2 = (counter * 4 + i_var4);
            i_var5 = u_var2;
            u_var2 = u_var2 & 0xffff0000;
            pi_var6 = (u_var2 | i_var5 + 0x1e);
            u_var8 = (u_var2 >> 0x10);
            i_var5 = (i_var5 + 0x24) / 2 + (i_var5 + 0x20);
            let pi_var6_val = unsafe { *pi_var6 };
            MoveTo16(i_var5, pi_var6_val, in_hdc_2);
            LineTo16(i_var5, pi_var6_val + -0xf, in_hdc_2);
            d_var9 = GetCurrentPosition16(in_hdc_2);
            y_coord = (d_var9 >> 0x10);
            x_coord = d_var9;
            if (counter == 0) {
                local_10 = x_coord;
                local_e = y_coord;
            }
            counter = counter + 1;
        }
        u_var2 = local_Struct135.field_0x6;
        if ((u_var2 + 0x24) != 0) {
            local_e = local_e + -0xd;
        }
        u_var2 = local_Struct135.field_0x6;
        if ((u_var2 + 0x26) != 0) {
            y_coord = y_coord + 0xd;
        }
        u_var2 = local_Struct135.field_0x6;
        u_var3 = local_Struct135.field_0x6;
        u_var2 = (u_var2 + (u_var3 + 0x16) * 4 + 0x26);
        local_a_x_262 = u_var2;
        local_a_x_262 = &local_a_x_262.field_0x1e;
        u_var2 = u_var2 & 0xffff0000;
        u_var7 = (u_var2 >> 0x10);
        MoveTo16(
            local_a_x_262.field_0x24 / 2 + local_a_x_262.field_0x20,
            local_a_x_262.field_0x22 + (u_var2 | ZEXT24(local_a_x_262)),
            in_hdc_2,
        );
        LineTo16(
            local_a_x_262.field_0x24 / 2 + local_a_x_262.field_0x20,
            local_10,
            in_hdc_2,
        );
        curr_pos_10 = GetCurrentPosition16(in_hdc_2);
        curr_pos_10._2_2_ = (curr_pos_10 >> 0x10);
        if (curr_pos_10._2_2_ < local_e) {
            local_e = curr_pos_10._2_2_;
        }
        if (y_coord < curr_pos_10._2_2_) {
            y_coord = curr_pos_10._2_2_;
        }
        MoveTo16(local_e, local_10, in_hdc_2);
        LineTo16(y_coord, x_coord, in_hdc_2);
    }
    return;
}

pub fn draw_lines_1040_c302(param_1: u32, in_hdc_2: HDC16) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let mut u_var5: i32;
    let local_AX_46: *mut Struct138;
    let local_AX_100: *mut Struct139;
    let mut u_var6: u16;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    u_var4 = (param_1 + 6);
    iVar1 = (u_var4 + 0x16);
    if (1 < iVar1) {
        u_var2 = (param_1 + 6);
        u_var5 = u_var2 + 0x2a;
        u_var2 = u_var2 & 0xffff0000;
        u_var3 = (u_var2 | u_var5);
        local_AX_46 = u_var3;
        local_AX_46 = &local_AX_46.field_0x1e;
        u_var3 = u_var3 & 0xffff0000;
        u_var6 = (u_var3 >> 0x10);
        MoveTo16(
            local_AX_46.field_0x20 + local_AX_46.field_0x24,
            local_AX_46.field_0x22 / 2 + (u_var3 | ZEXT24(local_AX_46)),
            in_hdc_2,
        );
        u_var2 = (u_var5 + iVar1 * 4 + -4);
        local_AX_100 = u_var2;
        local_AX_100 = &local_AX_100.field_0x1e;
        u_var2 = u_var2 & 0xffff0000;
        u_var6 = (u_var2 >> 0x10);
        LineTo16(
            local_AX_100.field_0x20,
            local_AX_100.field_0x22 / 2 + (u_var2 | ZEXT24(local_AX_100)),
            in_hdc_2,
        );
    }
    return;
}
