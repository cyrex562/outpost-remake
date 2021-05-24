use crate::pass::pass8_funcs::pass1_1010_2ee2;
use crate::structs::prog_structs_29::Struct136;
use crate::structs::prog_structs_30::Struct137;
use crate::structs::prog_structs_31::{Struct138, Struct139};
use crate::structs::prog_structs_7::Struct135;
use crate::typedefs::HDC16;
use crate::util::ZEXT24;
use crate::winapi::{GetCurrentPosition16, LineTo16, MoveTo16};
use crate::structs::prog_structs_20::Struct388;

pub unsafe fn draw_lines_1040_c38e(struct_param_1:  &mut Struct135, hdc_param_1: u16) {
    let mut i_var1: i32;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut i_var5: &mut Struct137;
    let local_a_x_262: &mut Struct137;
    let pi_var6:  i16;
    let mut u_var7: u16;
    let mut d_var9: u32;
    let mut curr_pos_10: u32;
    let mut x_coord_1: i16 = 0;
    let mut y_coord_1: i16 = 0;
    let mut x_coord_2: i16 = 0;
    let mut y_coord_2: i16 = 0;

    let mut struct_var2: &mut Struct137 = &mut struct_param_1.field_0x6;
    i_var1 = (struct_var2.field_0x18);
    if (i_var1 != 0) && ((struct_var2.field_0x16) != 0) {
        i_var4 = i_var1;
        pass1_1010_2ee2(struct_param_1.field_0x6);
        let mut counter = 0;
        while counter < i_var1 {
            struct_var2.field_0x0 = (counter * 4 + i_var4);
            i_var5 = struct_var2;
            pi_var6 = (struct_var2 | i_var5.field_0x1e);
            i_var5.field_0x0 = (i_var5.field_0x24) / 2 + (i_var5.field_0x20);
            MoveTo16(hdc_param_1, pi_var6, i_var5.field_0x0 as i16);
            LineTo16(hdc_param_1, pi_var6 - 0xf, i_var5.field_0x0 as i16);
            d_var9 = GetCurrentPosition16(hdc_param_1);
          // y_coord_1 = (d_var9  >> 0x10) as i16;
            x_coord_1 = d_var9 as i16;
            if counter == 0 {
                x_coord_2 = x_coord_1;
                y_coord_2 = y_coord_1;
            }
            counter = counter + 1;
        }
        struct_var2 = struct_param_1.field_0x6;
        if (struct_var2 + 0x24) != 0 {
            y_coord_2 = y_coord_2 + -0xd;
        }
        struct_var2 = struct_param_1.field_0x6;
        if (struct_var2 + 0x26) != 0 {
            y_coord_1 = y_coord_1 + 0xd;
        }
        struct_var2 = struct_param_1.field_0x6;
        u_var3 = struct_param_1.field_0x6;
        struct_var2 = (struct_var2 + (u_var3 + 0x16) * 4 + 0x26);
        local_a_x_262 = struct_var2;
        struct_var2 = struct_var2 & 0xffff0000;
      // u_var7 = (struct_var2  >> 0x10);
        MoveTo16(
            hdc_param_1,
            local_a_x_262.field_0x22 + (struct_var2 | local_a_x_262.field_0x0),
            (local_a_x_262.field_0x24 / 2 + local_a_x_262.field_0x20) as i16
        );
        LineTo16(
            hdc_param_1,
            x_coord_2,
            (local_a_x_262.field_0x24 / 2 + local_a_x_262.field_0x20) as i16
        );
        curr_pos_10 = GetCurrentPosition16(hdc_param_1);
        curr_pos_10._2_2_ = (curr_pos_10 >> 0x10);
        if curr_pos_10._2_2_ < y_coord_2 {
            y_coord_2 = curr_pos_10._2_2_;
        }
        if y_coord_1 < curr_pos_10._2_2_ {
            y_coord_1 = curr_pos_10._2_2_;
        }
        MoveTo16(hdc_param_1, x_coord_2, y_coord_2);
        LineTo16(hdc_param_1, x_coord_1, y_coord_1);
    }
    return;
}

pub fn draw_lines_1040_c302(param_1: u32, hdc_param_2: HDC16) {
    let mut i32_var1: i32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let mut u_var5: i32;
    let local_AX_46:  Struct138;
    let local_AX_100:  Struct139;
    let mut u_var6: u16;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

    //// _var6 = (param_1  >> 0x10);
    u_var4 = (param_1 + 6);
    i32_var1 = (u_var4 + 0x16);
    if 1 < i32_var1 {
        u_var2 = (param_1 + 6);
        u_var5 = u_var2 + 0x2a;
        u_var2 = u_var2 & 0xffff0000;
        u_var3 = (u_var2 | u_var5);
        local_AX_46 = u_var3;
        local_AX_46 = &local_AX_46.field_0x1e;
        u_var3 = u_var3 & 0xffff0000;
      // u_var6 = (u_var3  >> 0x10);
        MoveTo16(hdc_param_2,
                 local_AX_46.field_0x22 / 2 + (u_var3 | ZEXT24(local_AX_46)),
                 local_AX_46.field_0x20 + local_AX_46.field_0x24);
        u_var2 = (u_var5 + i32_var1 * 4 + -4);
        local_AX_100 = u_var2;
        local_AX_100 = &local_AX_100.field_0x1e;
        u_var2 = u_var2 & 0xffff0000;
      // u_var6 = (u_var2  >> 0x10);
        LineTo16(hdc_param_2,
                 local_AX_100.field_0x22 / 2 + (u_var2 | ZEXT24(local_AX_100)),
                 local_AX_100.field_0x20);
    }
    return;
}
