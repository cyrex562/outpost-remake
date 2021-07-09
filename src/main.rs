use fn_ptr::util::get_fn_ptr_1;
use winapi::swi;

use crate::fn_ptr::fn_ptr_1000::fn_ptr_op_1000_24cd;
use crate::global::AppContext;
use crate::init::init_1000_23be;
use crate::mem_1000::mem_op_1000_1902;
use crate::misc::ret_op_1000_55ac;
use crate::pass::pass_1000::{
    pass1_1000_24db, pass1_1000_25a8, pass1_1000_262c, pass1_1000_27d6, pass1_1000_2913,
};
use crate::string::string_1000::poss_str_op_1000_28dc;
use crate::sys_api::{dos3_call_1000_23ea, dos3_call_op_1000_435c};
use crate::util::{make_u16_ptr, make_u8_ptr, CONCAT11, CONCAT12, CONCAT13, CONCAT22, SUB42};
use crate::win_struct::{CONTEXT, HWND16, WPARAM16};
use crate::winapi::{
    FatalAppExit16, FatalExit, GetVersion16, GetWindowLong16, InitApp16, InitTask16, LockSegment16,
    WaitEvent16,
};

mod bad;
mod cleanup;
mod debug;
mod defines;
mod draw;
mod exit;
mod file;
mod fn_ptr;
mod global;
mod init;
mod mci;
mod mem_1000;
mod mem_1008;
mod misc;
mod msg_box;
mod pass;
mod string;
mod struct_ops;
mod switch_ops;
mod sys_api;
mod ui;
mod util;
mod win_struct;
mod winapi;
mod mixed;

pub fn main() {
    let mut ctx = AppContext::new();
    let mut task_ctx = CONTEXT::new();
    let mut param_8: i16 = 0;
    let mut param_1: u16 = 0;
    let mut param_9 = "".to_string();

    let result = unsafe {
        entry(
            &mut ctx,
            &mut param_1,
            0,
            0,
            0,
            0,
            &mut task_ctx,
            0,
            &mut param_8,
            &mut param_9,
        )
    };
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

unsafe fn entry(
    ctx: &mut AppContext,
    param_1: &mut u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    in_task_context: &mut CONTEXT,
    param_7: u16,
    param_8: &mut i16,
    param_9: &mut String,
) -> *mut i16 {
    let pu_var1: *mut i16;
    let u_var2: i16;
    // char * pc_var3;
    let mut pc_var3: String = "".to_string();
    // code * pcVar4;
    let u_var8: u8;
    let u_var5: u16;
    let u_var6: i16;
    let mut str_a: String = "".to_string();
    let pu_var7: *mut i16;
    let u_var9: u32;
    let pu_var10: *mut i16;
    // char * pc_var11;
    let mut pc_var11 = "".to_string();
    let b_var12: bool;
    let version: u32;
    let u_var14: u32;
    let u_var15: u32;
    let i_var16: i16;
    let i_var17: i16;
    let pu_var18: *mut u8;
    let u_var19: u16;
    let mut pc_var4 = 0;

    u_var14 = CONCAT22(param_7, ctx.PTR_LOOP_1050_5f84 as u16);
    loop {
        u_var19 = 0x0;
        InitTask16(in_task_context);
        ctx.PTR_LOOP_1050_5f84 = u_var14;
        b_var12 = *param_1 < 0xff00;
        *param_1 = *param_1 + 0x100;
        ctx.PTR_LOOP_1050_5f7e = param_5 as u32;
        // if ((param_8 != 0x0) &&
        //    (bVar12 = param_1 < 0xff00, param_1 += 0x100,
        //    ctx.PTR_LOOP_1050_5f7e = param_5, bVar12)) {
        if *param_8 != 0 && b_var12 {
            ctx.PTR_LOOP_1050_5f48 = *param_1 as u32;
            ctx.PTR_LOOP_1050_5f4a = param_3 as u32;
            ctx.PTR_LOOP_1050_5f4c = param_4 as u32;
            ctx.PTR_LOOP_1050_5f4e = param_2 as u32;
            ctx.PTR_LOOP_1050_5f50 = param_5 as u32;
            LockSegment16(ctx.s_tile2_bmp_1050_1538 as u16);
            ctx.PTR_LOOP_1050_5f52 = (u_var14 >> 0x10);
            ctx.PTR_LOOP_1050_5f84 = u_var14;
            version = GetVersion16();
            ctx.PTR_LOOP_1050_5f52 = (u_var14 >> 0x10);
            ctx.PTR_LOOP_1050_5f84 = u_var14;
            u_var9 = (version >> 0x10);
            u_var8 = (version >> 0x8) as u8;
            ctx.PTR_LOOP_1050_5f80 = version;
            u_var5 = CONCAT11(0x30, u_var8);
            if true {
                pc_var4 = swi(0x21);
                u_var15 = u_var14;
                let fn_ptr_1 = get_fn_ptr_1(pc_var4);
                // u_var14 = (*pc_var4)(u_var19);
                u_var14 = fn_ptr_1(u_var19) as u32;
                ctx.PTR_LOOP_1050_5f52 = u_var15 >> 0x10;
                ctx.PTR_LOOP_1050_5f84 = u_var15;
            }
            ctx._DAT_1050_5f82 = CONCAT11(u_var14 as u8, ((u_var14 >> 0x8) as u8));
            if true {
                ctx.DAT_1050_5f87 = 0x0;
            }
            WaitEvent16(0x1000);
            ctx.PTR_LOOP_1050_5f84 = u_var14;
            // pu_var18 = ctx.PTR_LOOP_1050_5f4c;
            pu_var18 = make_u8_ptr(ctx.PTR_LOOP_1050_5f4c);
            *param_8 = InitApp16(ctx.s_tile2_bmp_1050_1538);
            ctx.PTR_LOOP_1050_5f84 = u_var14;
            if *param_8 != 0x0 {
                break;
            }
        }
        // TODO: HGLOBAL16/HINSTANCE16 into CONTEXT
        //*in_task_context = ctx.s_tile2_bmp_1050_1538;
        *param_8 = CONCAT11((*param_8 >> 0x8) as u8, 0xff) as i16;
        pass1_1000_24db(*param_8, 0x0);
        ctx.PTR_LOOP_1050_5f84 = u_var14;
    }
    dos3_call_1000_23ea(param_2, param_5, 0x0, param_9);
    ctx.PTR_LOOP_1050_5f84 = u_var14;
    pass1_1000_262c(
        make_u8_ptr(0x238d),
        make_u8_ptr(ctx.s_tile2_bmp_1050_1538),
        param_9,
        ctx.s_tile2_bmp_1050_1538 as u16,
    );
    ctx.PTR_LOOP_1050_5f84 = u_var14;
    pass1_1000_27d6(make_u16_ptr(u_var14 >> 0x10));
    u_var14 = ret_op_1000_55ac(pu_var18);
    u_var6 = u_var14 as i16;
    let mut param_6: i16 = 0;
    init_1000_23be(
        ctx,
        *param_1,
        (u_var14 >> 0x10) as u16,
        param_9,
        0,
        0,
        &mut param_6,
    );
    fn_ptr_op_1000_24cd(u_var6, 0x0);
    i_var17 = 0x15;
    i_var16 = 0x15;
    pass1_1000_25a8(param_5, ctx.s_tile2_bmp_1050_1538 as u16);
    pass1_1000_2913(i_var16, param_5, ctx.s_tile2_bmp_1050_1538 as u16);
    str_a = poss_str_op_1000_28dc(i_var17);
    if str_a.chars().nth(0).unwrap() != '\0' {
        i_var16 = 0x9;
        if str_a.chars().nth(0).unwrap() == 'M' {
            i_var16 = 0xf;
        }
        str_a = str_a + i_var16;
        i_var16 = 0x22;
        pc_var11 = str_a;
        loop {
            if i_var16 == 0x0 {
                break;
            }
            i_var16 += -0x1;
            pc_var3 = pc_var11;
            pc_var11 = pc_var11 + 0x1;
            if pc_var3.chars().nth(0).unwrap() == '\r' {
                break;
            }
        }
        // pc_var11[-0x1] = '\0';
    }
    FatalAppExit16(ctx.s_tile2_bmp_1050_1538 as u16, str_a.as_str());
    FatalExit();
    pu_var10 = &ctx.PTR_LOOP_1050_63fe as *mut i16;
    loop {
        pu_var1 = pu_var10;
        pu_var10 = pu_var10 + 0x1;
        u_var2 = *pu_var1;
        pu_var7 = pu_var10;
        pu_var7[0] = u_var2 + 0x1;
        if (u_var2 == u_var6) || pu_var7.is_null() {
            return pu_var7;
        }
        i_var16 = -0x1;
        loop {
            if i_var16 == 0x0 {
                break;
            }
            i_var16 += -0x1;
            pu_var1 = pu_var10;
            pu_var10 = (pu_var10 + 0x1);
            if *pu_var1 == '\0' as i16 {
                break;
            }
        }
    }
}
