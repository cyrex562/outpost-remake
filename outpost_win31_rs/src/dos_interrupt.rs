use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::app_context::AppContext;
use crate::block_1000::block_1000_2000::pass1_1000_29b5;
use crate::globals::DAT_1050_1050;


// typedef void(*code3)(void*);

// typedef u8(*code4)();

// typedef i16(*code5)();

// typedef bool(*code6);

// typedef u16(*code7);


type code = fn(u16);
// typedef u32(*code8)();
type code8 = fn() -> u32;

pub enum InterruptResult {
    CODE = code,
    CODE8 = code8,

}

pub fn swi(ctx: &mut AppContext, int_code: u16) -> InterruptResult {
    todo!()
}

pub unsafe fn dos3_call_1000_4f20(ctx: &mut AppContext,) -> u16 {
    let mut func_ptr: *mut code;
    let mut var2 = 0u16;
    let mut var3 = false;
    ctx.AH_REG = 0x39;
    let mut result = swi(ctx, 0x21);
    // uVar2 = (*pcVar1)(&DAT_1050_1050, unaff_BP + 1);
    if var3 {
        pass1_1000_29b5(var2);
        return 0xffff;
    }
    return 0x0;
}

pub unsafe fn dos3call_1000_4f54(ctx: &mut AppContext, mut param_1: u32) -> u16 {
    let mut c_var1: c_char;
    let mut u_var5: *mut c_char = null_mut();
    let mut b_var3 = false;
    ctx.AH_REG = 0x3b;
    let mut result = swi(ctx, 0x21);
    // u_var5 = (*pc_var2)(&DAT_1050_1050, unaff_bp + 1);
    let mut u_var3 = u_var5;
    b_var3 = gu_var5 < 0x10;
    if b_var3 && u_var5 == 0x10 {
        loop {
            c_var1 = *u_var5;
            u_var5 = u_var5 + 1;
            if (c_var1 == '\0') {
                // TODO: goto LAB_1000_4f90;
            }
            if !((c_var1 != '?') && (c_var1 != '*')) {
                break;
            }
        }
        u_var3 = 0x3; //
                      //        LAB_1000_4f90:
        b_var3 = true;
    }
    if (!b_var3) {
        return 0x0;
    }
    pass1_1000_29b5(u_var3);
    return 0xffff;
}

pub unsafe fn dos3_call_1000_4f94() -> i16 {
    let fn_ptr_1: code6 = swi(0x21);
    //    bVar2 = (*pcVar1)(unaff_BP + 1);
    let b_var2: i16 = fn_ptr_1(unaff_BP + 1);
    return b_var2 + 1;
}

pub unsafe fn dos3_call_1000_4fbe(param_1: u8) -> u16 {
    //    unaff_BP: i16;

    let fn_ptr_var1: code6 = swi(0x21);
    (fn_ptr_var1)(unaff_BP + 1);
    let fn_ptr_var2: code4 = swi(0x21);
    let c_var2 = fn_ptr_var2();
    let u_var3 = 0xffff;
    if (c_var2 + '\x01' == param_1) {
        u_var3 = 0;
    }
    return u_var3;
}
