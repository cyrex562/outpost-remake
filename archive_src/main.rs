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
use crate::string::string_1000::str_op_1000_28dc;
use crate::sys_api::{dos3_call_1000_23ea, dos3_call_op_1000_435c};
use crate::util::{make_u16_ptr, make_u8_ptr, CONCAT11, CONCAT12, CONCAT13, CONCAT22, SUB42};
use crate::win_struct::{CONTEXT, HWND16, WPARAM16, HINSTANCE16, HANDLE16};
use crate::winapi::{
    FatalAppExit16, FatalExit, GetVersion16, GetWindowLong16, InitApp16, InitTask16, LockSegment16,
    WaitEvent16,
};
use crate::defines::U32Ptr;

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
mod resources;


// TODO: refactor for loops
// TODO: refactor use of GOTO
// TODO: remove use of `*(astruct_99 **)`
// TODO: fix up function declarations

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

const INT21_PROG_TERM: u16 = 0x0;

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
    string_1: &mut String,
) -> U32Ptr {
    let ptr_1: U32Ptr;
    let var2: i16;
    let mut string_3: String = "".to_string();
    let var8: u8;
    let var5: u16;
    let var6: i16;
    let mut string_2: String = "".to_string();
    let ptr_2: U32Ptr;
    let var9: u32;
    let ptr_3: U32Ptr;
    let mut string_1 = "".to_string();
    let var12: bool;
    let version: u32;
    let var15: u32;
    let var16: i16;
    let var17: i16;
    let var18: U32Ptr;
    let int21_code: u16;
    let mut var4 = 0;

    let mut var14 = CONCAT22(param_7, ctx.PTR_LOOP_1050_5f84 as u16);
    loop {
        int21_code = INT21_PROG_TERM;
        InitTask16(in_task_context);
        ctx.PTR_LOOP_1050_5f84 = var14;
        var12 = *param_1 < 0xff00;
        *param_1 = *param_1 + 0x100;
        ctx.PTR_LOOP_1050_5f7e = param_5 as u32;
        // if ((param_8 != 0x0) &&
        //    (bVar12 = param_1 < 0xff00, param_1 += 0x100,
        //    ctx.PTR_LOOP_1050_5f7e = param_5, bVar12)) {
        if *param_8 != 0 && var12 {
            ctx.PTR_LOOP_1050_5f48 = *param_1 as u32;
            ctx.PTR_LOOP_1050_5f4a = param_3 as u32;
            ctx.PTR_LOOP_1050_5f4c = param_4 as u32;
            ctx.PTR_LOOP_1050_5f4e = param_2 as u32;
            ctx.PTR_LOOP_1050_5f50 = param_5 as u32;
            LockSegment16(ctx.s_tile2_bmp_1050_1538 as HANDLE16);
            ctx.PTR_LOOP_1050_5f52 = (var14 >> 0x10);
            ctx.PTR_LOOP_1050_5f84 = var14;
            version = GetVersion16();
            ctx.PTR_LOOP_1050_5f52 = (var14 >> 0x10);
            ctx.PTR_LOOP_1050_5f84 = var14;
           // u_var9 = (version >> 0x10);
            var8 = (version >> 0x8) as u8;
            ctx.PTR_LOOP_1050_5f80 = version;
            var5 = CONCAT11(0x30, var8);
            if true {
                var4 = swi(0x21);
                var15 = var14;
                let fn_ptr_1 = get_fn_ptr_1(var4);
                // u_var14 = (*pc_var4)(u_var19);
                var14 = fn_ptr_1(int21_code) as u32;
                ctx.PTR_LOOP_1050_5f52 = var15 >> 0x10;
                ctx.PTR_LOOP_1050_5f84 = var15;
            }
            ctx._DAT_1050_5f82 = CONCAT11(var14 as u8, ((var14 >> 0x8) as u8));
            if true {
                ctx.DAT_1050_5f87 = 0x0;
            }
            WaitEvent16(0x1000);
            ctx.PTR_LOOP_1050_5f84 = var14;
            // pu_var18 = ctx.PTR_LOOP_1050_5f4c;
            var18 = make_u8_ptr(ctx.PTR_LOOP_1050_5f4c);
            *param_8 = InitApp16(ctx.s_tile2_bmp_1050_1538 as HINSTANCE16);
            ctx.PTR_LOOP_1050_5f84 = var14;
            if *param_8 != 0x0 {
                break;
            }
        }
        // TODO: HGLOBAL16/into: HINSTANCE16 CONTEXT
        in_task_context.context_flags = ctx.s_tile2_bmp_1050_1538;
        *param_8 = CONCAT11((*param_8 >> 0x8) as u8, 0xff) as i16;
        pass1_1000_24db(ctx, *param_8, 0x0);
        ctx.PTR_LOOP_1050_5f84 = var14;
    }
    dos3_call_1000_23ea(param_2, param_5, 0x0, string_1);
    ctx.PTR_LOOP_1050_5f84 = var14;
    pass1_1000_262c(
        ctx,
        0x238d,
        ctx.s_tile2_bmp_1050_1538 as u16,
        string_1,
        ctx.s_tile2_bmp_1050_1538 as HINSTANCE16,
        0,
        0
    );
    ctx.PTR_LOOP_1050_5f84 = var14;
    pass1_1000_27d6(ctx, var14);
    var14 = ret_op_1000_55ac(var18, 0, 0, 0);
    var6 = var14 as i16;
    let mut param_6: i16 = 0;
    init_1000_23be(
        ctx,
        *param_1,
        var14 as u16,
        string_1,
        0,
        0,
        &mut param_6,
    );
    fn_ptr_op_1000_24cd(ctx, var6, 0x0);
    var17 = 0x15;
    var16 = 0x15;
    pass1_1000_25a8(ctx, param_5, ctx.s_tile2_bmp_1050_1538 as u16);
    pass1_1000_2913(ctx, var16, param_5, ctx.s_tile2_bmp_1050_1538 as u16);
    string_2 = str_op_1000_28dc(ctx, var17);
    if string_2.chars().nth(0).unwrap() != '\0' {
        var16 = 0x9;
        if string_2.chars().nth(0).unwrap() == 'M' {
            var16 = 0xf;
        }
        string_2 = string_2[var16..].to_string();
        var16 = 0x22;
        string_1 = string_2;
        loop {
            if var16 == 0x0 {
                break;
            }
            var16 += -0x1;
            string_3 = string_1;
            string_1 = string_1[0x1..].to_string();
            if string_3.chars().nth(0).unwrap() == '\r' {
                break;
            }
        }
        // pc_var11[-0x1] = '\0';
    }
    FatalAppExit16(ctx.s_tile2_bmp_1050_1538 as u16, string_2.as_str());
    FatalExit();
    ptr_3 = ctx.PTR_LOOP_1050_63fe;
    loop {
        ptr_1 = ptr_3;
        ptr_3 = ptr_3 + 0x1;
        var2 = ptr_1[0];
        ptr_2 = ptr_3;
        ptr_2[0] = var2 + 0x1;
        if (var2 == var6) || ptr_2.is_null() {
            return ptr_2;
        }
        var16 = -0x1;
        loop {
            if var16 == 0x0 {
                break;
            }
            var16 += -0x1;
            ptr_1 = ptr_3;
            ptr_3 = (ptr_3 + 0x1);
            if *ptr_1 == '\0' as i16 {
                break;
            }
        }
    }
}
