use crate::{
    global::AppContext,
    pass::{pass_1020::pass1_1020_b97e, pass_1030::pass1_1030_de7c},
};

use super::{
    file_1008::{read_file_1008_7cfe, read_file_1008_7dee},
    file_1028::write_to_file_1028_b5ec,
    write::write_to_file_1008_7e1c,
};

pub fn read_file_1020_a65e(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u32,
    param_3: u16,
    param_4: u16,
) -> bool {
    let b_var1: bool;
    let in_DX: u16;
    let local_a: [u8; 2];
    let local_8: [u8; 2];
    let local_6: [u8; 2];
    let local_4: [u8; 2];
    let u_var3: u16;
    let u_var2: u16;

    u_var2 = param_2;
    // u_var3 = (param_2 >> 0x10);
    read_file_1008_7cfe(ctx, u_var2, u_var3, 0xb, 0x1008);
    if param_4 != 0x0 {
        if 0x1 < ctx.PTR_LOOP_1050_0312 {
            //LAB_1020_a6dc:
            pass1_1020_b97e(param_3, param_4, in_DX, param_1, (param_1 >> 0x10), 0x0);
            return 0x1;
        }
        b_var1 = read_file_1008_7dee(u_var2, u_var3, local_4, 0x0, param_3, 0x2, 0x1008);
        if b_var1 != 0x0 {
            b_var1 = read_file_1008_7dee(u_var2, u_var3, local_8, 0x0, param_3, 0x2, 0x1008);
            if b_var1 != 0x0 {
                b_var1 = read_file_1008_7dee(u_var2, u_var3, local_6, 0x0, param_3, 0x2, 0x1008);
                if b_var1 != 0x0 {
                    *param_4 =
                        read_file_1008_7dee(u_var2, u_var3, local_a, 0x0, param_3, 0x2, 0x1008);
                    if param_4 != 0x0 {
                        // goto LAB_1020_a6dc;
                    }
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return 0x0;
}

pub fn write_to_file_1020_d3d4(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u32,
    param_3: u16,
) -> bool {
    let mut local_c: Vec<u8>;
    let mut b_var1 = write_to_file_1028_b5ec(param_1, param_2, param_3);
    if b_var1 != 0x0 {
        local_c[0] = (param_1 + 0x20);
        b_var1 = write_to_file_1008_7e1c(param_2, (param_2 >> 0x10), local_c, param_3, 0x2, 0x1008);
        if b_var1 == 0x0 {
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return b_var1;
        }
        b_var1 = 0x1;
    }

    return b_var1;
}

pub fn write_to_file_1020_e6a4(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u32,
    param_3: u16,
) -> bool {
    let in_AX: i16;
    let b_var1: bool;
    let u_var2: u16;
    let u_var3: u16;
    let local_c: [u16; 0x3];
    let local_6: [u16; 0x2];

    pass1_1030_de7c(param_1, param_2, param_3);
    if (in_AX != 0x0) {
        // u_var2 = (param_1 >> 0x10);
        local_c[0] = (param_1 + 0x24);
        // u_var3 = (param_2 >> 0x10);
        b_var1 = write_to_file_1008_7e1c(param_2, u_var3, local_c, param_3, 0x2, 0x1008);
        if (b_var1 != 0x0) {
            local_6[0] = (param_1 + 0x26);
            b_var1 = write_to_file_1008_7e1c(param_2, u_var3, local_6, param_3, 0x2, 0x1008);
            if (b_var1 != 0x0) {
                return 0x1;
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return 0x0;
}
