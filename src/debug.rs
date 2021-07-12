use crate::{
    pass::pass_1000::{pass1_1000_2b5c, pass1_1000_2f48, pass1_1000_3ec0},
    winapi::{wvsprintf16, OutputDebugString16},
};

pub fn debug_print_1008_6048(
    param_1: u32,
    param_2: &mut String,
    param_3: *mut u16,
    in_DX: u16,
    unaff_ES: u16,
    in_AF: u8,
    stack0x0008: u16,
    stack0xfffe: u16,
) {
    // let in_DX: u16;
    // let unaff_ES: u16;
    // let in_AF: u8;
    let u_stack266: u16;
    let mut p_c_stack6: String;
    let args: *mut u16;

    if (ctx.PTR_LOOP_1050_02ec != 0x0) {
        p_c_stack6 = &stack0x0008;
        args = param_3;
        if (ctx.DAT_1050_02ee == 0xffff) {
            // param_2 =  & ctx.PTR_LOOP_1050_1000;
            u_stack266 = pass1_1000_3ec0(0x2f4, ctx.data_seg);
            ctx.DAT_1050_02ee = ((in_DX | u_stack266) != 0x0);
        }
        if (ctx.DAT_1050_02ee != 0x0) {
            wvsprintf16(param_2, p_c_stack6, args);
            OutputDebugString16(ctx.s_tile2_bmp_1050_1538);
            OutputDebugString16(ctx.s_tile2_bmp_1050_1538);
            if (ctx.PTR__LOOP_1050_02f0 != 0x0) {
                pass1_1000_2b5c(
                    ctx._PTR_LOOP_1050_02f0,
                    (ctx.PTR__LOOP_1050_02f0 >> 0x10),
                    0x2fd,
                    ctx.data_seg,
                    unaff_ES,
                    &stack0xfffe,
                    0x1000,
                    param_3,
                );
                pass1_1000_2f48(
                    ctx._PTR_LOOP_1050_02f0,
                    &stack0xfffe,
                    unaff_ES,
                    0x1000,
                    param_3,
                    in_AF,
                );
            }
        }
    }
    return;
}
