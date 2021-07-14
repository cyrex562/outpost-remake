use crate::{string::string_1000::poss_str_op_1000_28dc, win_struct::{HINSTANCE16, SEGPTR}, winapi::{DOS3Call, GetDOSEnvironment16}};
use crate::defines::{Struct18, Struct20, StructA, Struct99};
use crate::exit::exit_1000_25f2;
use crate::fn_ptr::fn_ptr_1000::{call_fn_ptr_1000_0dc6, fn_ptr_op_1000_2594};
use crate::global::AppContext;
use crate::mem_1000::{free_mem_1000_407a, mem_1000_0670, mem_1000_167a, mem_op_1000_0510, mem_op_1000_0838, mem_op_1000_0a48, mem_op_1000_1532, mem_op_1000_160a, mem_op_1000_1b9a, mem_op_1000_1dfa, mem_op_1000_21b6, mixed_mem_op_1000_3c51, mem_1000_2bb6};
use crate::misc::ret_op_1000_55ac;
use crate::msg_box::{msg_box_op_1000_1f24, msg_box_op_1000_214c};
use crate::string::string_1000::{str_op_1000_3da4, unk_str_op_1000_3d3e};
use crate::string::string_1000;
use crate::sys_api::{dos3_call_1000_514e, dos3_call_1000_5174, dos3_call_op_1000_35fe, dos3_op_1000_256b, mixed_dos3_call_1000_39f2};
use crate::util::{CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, get_string_from_addr, SUB42, ZEXT24, CARRY4};
use crate::win_struct::WNDCLASS16;
use crate::winapi::{FatalAppExit16, FatalExit, GetModuleFileName16, GlobalDOSFree16, SegmentLimit, swi};

// pub fn pass1_1000_010c

pub unsafe fn pass1_1000_0368(param_1: u16, param_2: u16, param_3: u16)

{
    let pu_var1: *mut u16;

    if (param_1 + 0x4) == param_1 {
        (param_3 + param_2 * 0x2) = 0x0;
    } else {
        ((param_1 + 0x6) + 0x4) = (param_1 + 0x4);
        ((param_1 + 0x4) + 0x6) = (param_1 + 0x6);
        pu_var1 = (param_2 * 0x2 + param_3);
        if *pu_var1 == param_1 {
            *pu_var1 = (param_1 + 0x4);
        }
    }
    (param_1 + 0x4) = (param_3 + 0xa);
    (param_3 + 0xa) = param_1;
    return;
}

pub fn pass1_1000_05b4(param_1: u8, param_2: i16)

{
    (param_2 + 0xa) = 0x1;
    (param_2 + 0x8) = 0x668;
    (param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
    (param_2 + 0x10) = 0x0;
    (param_2 + 0xe) = 0x0;
    return;
}

pub unsafe fn pass1_1000_0782(param_1: u32, param_2: u16, param_3: i16, in_stack_00000004: u16) -> u16

{
    (param_3 + 0xe) = 0x0;
    (param_3 + 0x10) = param_3 + 0x14;
    (param_3 + 0x8) = 0x9a0;
    pass1_1000_07ac((param_1 + 0x18), param_2, param_3);
    return 0x1;
}


pub unsafe fn pass1_1000_07ac(param_1: u16, param_2: i16, param_3: i16)

{
    let pu_var1: *mut u16;
    let i_var2: i16;
    let u_var3: u16;

    pu_var1 = (param_3 + 0x10);
    (param_3 + 0xe) = pu_var1;
    u_var3 = param_2 + (param_3 - pu_var1);
    i_var2 = pu_var1 + (u_var3 - u_var3 % param_1);
    (param_3 + 0x10) = i_var2;
    while pu_var1 < (i_var2 - param_1) {
        *pu_var1 = (pu_var1 + param_1);
        pu_var1 = (pu_var1 + param_1);
    }
    *pu_var1 = 0x0;
    return;
}


pub unsafe fn pass1_1000_07fc(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u32,
    mut out: Option<&mut Struct99>)

{
    let pa_var1: &mut Struct99;

    if (param_2 + 0x14) != -0x4153 {
        pass1_1000_1e61(ctx, param_1, 0xa, None, 0x0);
        out = None;
    }
    pa_var1 = mem_op_1000_0838(ctx, None, param_1);
    out = Some(pa_var1);
}

pub unsafe fn pass1_1000_093a(
    ctx: &mut AppContext,
    param_1: *mut i16,
    param_2: u16,
    param_3: u16) -> u16

{
    let pi_var1: *mut i16;
    if &ctx.PTR_LOOP_1050_000c != -0x352f {
        pass1_1000_1e61(ctx, param_3, 0xe, None, 0);
        return 0x0;
    }
    *param_1 = &ctx.PTR_LOOP_1050_000e;
    if *param_1 == 0x0 {
        &ctx.DAT_1050_0004 = 0x1;
    }
    &ctx.PTR_LOOP_1050_000e = param_1;
    pi_var1 = &ctx.PTR_LOOP_1050_000a;
    *pi_var1 = *pi_var1 + -0x1;
    if *pi_var1 == 0x0 {
        mem_op_1000_0510(ctx, 0x1, 0x0, param_3);
    }
    return 0x1;
}


pub unsafe fn pass1_1000_09a0(
    ctx: &mut AppContext,
    param_1: *mut u16,
    param_2: u16) -> *mut u8

{
    let pu_var1: *mut u8;
    let u_var2: u32;

    *param_1 = ctx.PTR_LOOP_1050_000e;
    if ctx.PTR_LOOP_1050_000e == 0x0 {
        *ctx.DAT_1050_0004 = 0x1;
    }
    ctx.PTR_LOOP_1050_000a = ctx.PTR_LOOP_1050_000a + -0x1;
    pu_var1 = ctx.PTR_LOOP_1050_000e;
    ctx.PTR_LOOP_1050_000e = param_1;
    if ctx.PTR_LOOP_1050_000a == 0x0 {
        u_var2 = mem_op_1000_0510(ctx, 0x1, 0x0, param_2);
        pu_var1 = u_var2;
    }
    return pu_var1;
}


pub unsafe fn pass1_1000_09ca(param_1: i16, param_2: *mut u32) -> u16

{
    let pu_var1: *mut u16;
    let i_var2: i16;
    let u_var3: u32;
    let pu_var4: *mut u16;

    pu_var1 = param_2 + 0xa;
    pu_var4 = ((param_2 + (param_1 - pu_var1) + -0x6 & 0xfffc) + pu_var1);
    *pu_var4 = 0x1;
    param_2[0x7] = pu_var1;
    pu_var4[0x2] = pu_var4;
    pu_var4[0x1] = pu_var4;
    param_2[0x8] = pu_var4;
    if (((param_2 + 0x6) & 0x7) == 0x2) {
        param_2[0x9] = 0x8;
    } else {
        u_var3 = param_2;
        i_var2 = (u_var3 + 0x18);
        param_2[0x9] = (i_var2 - 0x5 & !-(i_var2 + 0x3 < 0x8)) + 0x8;
    }
    pu_var4[-0x1] = (pu_var4 - pu_var1);
    *pu_var1 = (pu_var4 - pu_var1) | 0x2;
    param_2[0xc] = pu_var4;
    param_2[0xb] = pu_var4[0x1];
    (pu_var4[0x1] + 0x4) = pu_var1;
    pu_var4[0x1] = pu_var1;
    param_2[0x4] = 0xe08;
    return *pu_var1 & 0xfffc;
}


pub unsafe fn pass1_1000_0c32(param_1: u16, param_2: u16, param_3: u16) -> u32

{
    let pu_var1: *mut u16;
    let pb_var2: *mut u8;
    let pi_var3: *mut i16;
    let u_var4: u32;
    let u_var5: u16;
    let pu_var6: *mut u16;
    let i_var7: i16;
    let pu_var8: *mut u16;
    let u_var9: u16;
    let u_stack14: u16;
    let pu_stack8: *mut u16;
    let u_stack6: u16;

    pu_var8 = (param_3 + 0xe);
    u_stack6 = 0x0;
    pu_var6 = pu_var8;
    loop {
        loop {
            u_var5 = *pu_var6;
            if param_1 <= u_var5 {
                u_var5 = (u_var5 & 0xfffc) - param_1;
                pu_var1 = (param_3 + 0x12);
                pu_stack8 = pu_var6;
                if *pu_var1 < u_var5 || *pu_var1 == u_var5 {
                    u_stack14 = param_1;
                    if (param_2 & 0x6) == 0x0 {
                        pu_stack8 = (u_var5 + pu_var6);
                        pu_stack8[-0x1] = u_var5;
                        *pu_var6 = u_var5 | 0x2;
                        pu_var8 = pu_var6[0x1];
                        pb_var2 = (pu_stack8 + param_1);
                        *pb_var2 = *pb_var2 | 0x2;
                        *pu_stack8 = param_1 | 0x1;
                    } else {
                        *pu_var6 = param_1 & 0xff00 | pu_var6 & 0x2 | param_1 & 0xff | 0x1;
                        (pu_var6[0x2] + 0x2) = pu_var6[0x1];
                        (pu_var6[0x1] + 0x4) = pu_var6[0x2];
                        pu_var8 = (pu_var6 + param_1);
                        (pu_var8 + (u_var5 - 0x2)) = u_var5;
                        *pu_var8 = u_var5 | 0x2;
                        u_var5 = (param_3 + 0x10);
                        pu_var8[0x2] = u_var5;
                        pu_var8[0x1] = (u_var5 + 0x2);
                        ((u_var5 + 0x2) + 0x4) = pu_var8;
                        (u_var5 + 0x2) = pu_var8;
                    }
                } else {
                    pu_var8 = pu_var6[0x1];
                    (pu_var6[0x2] + 0x2) = pu_var8;
                    (pu_var6[0x1] + 0x4) = pu_var6[0x2];
                    pu_var1 = pu_var6;
                    pu_var1 = pu_var1 | 0x1;
                    u_stack14 = *pu_var6 & 0xfffc;
                    (pu_var6 + u_stack14) = (pu_var6 + u_stack14) | 0x2;
                }
                (param_3 + 0xe) = pu_var8;
                if (param_2 & 0x1) != 0x0 {
                    pu_var6 = pu_stack8;
                    // TODO
                    // for (u_var5 = u_stack14 - 0x2 >> 0x1; pu_var6 = pu_var6 + 0x1, u_var5 != 0x0;
                    //     u_var5 -= 0x1) {
                    //   *pu_var6 = 0x0;
                    // }
                    if (u_stack14 - 0x2 & 0x1) != 0x0 {
                        *pu_var6 = 0x0;
                    }
                }
                if ((param_2 & 0x2) != 0x0) && (pu_var8[0x1] == pu_var8[0x2]) {
                    *(param_3 + 0x4) = *((param_3 + 0x10) + 0x2) & 0xfffc;
                    u_var4 = (param_3 + 0x4);
                    pb_var2 = (u_var4 + 0x3);
                    *pb_var2 = *pb_var2 | 0x80;
                }
                pi_var3 = (param_3 + 0xa);
                *pi_var3 = *pi_var3 + 0x1;
                return CONCAT22(0x1050, pu_stack8 + 0x1);
            }
            if u_stack6 < u_var5 {
                u_stack6 = u_var5;
            }
            pu_var6 = pu_var6[0x1];
            if pu_var6 == pu_var8 {
                break;
            }
        }
        if ((param_2 & 0x2) == 0x0) || ((param_2 & 0x40) != 0x0) { break; }
        u_var4 = param_3;
        // u_var9 = (u_var4 >> 0x10);
        i_var7 = u_var4;
        if (i_var7 + 0x34) == 0x0 { break; }
        u_stack6 = (i_var7 + 0x34)();
        if (u_stack6 < param_1) || (pu_var6 = (param_3 + 0xe), pu_var6 == 0x0) { break; }
    }
    *(param_3 + 0x4) = u_stack6 & 0xfffc;
    return 0x0;
}


pub unsafe fn pass1_1000_0e08(
    ctx: &mut AppContext,
    param_1: i16,
    param_2: u16) -> u16

{
    let pu_var1: *mut u16;
    let pb_var2: *mut u8;
    let u_var3: u16;
    let pu_var4: *mut u16;
    let pu_var5: *mut u16;
    let b_var6: bool;
    let u_var7: u32;

    pu_var5 = (param_1 + -0x2);
    b_var6 = (pu_var5 & 0x2) != 0x0;
    if b_var6 {
        pu_var1 = pu_var5;
        pu_var1 = pu_var1 & 0xfe;
    } else {
        pu_var4 = (pu_var5 - (param_1 + -0x4));
        pu_var1 = pu_var4;
        *pu_var1 = *pu_var1 + (*pu_var5 & 0xfffc);
        pu_var5 = pu_var4;
    }
    pu_var4 = ((*pu_var5 & 0xfffc) + pu_var5);
    if (pu_var4 & 0x1) == 0x0 {
        pu_var1 = pu_var5;
        *pu_var1 = *pu_var1 + (*pu_var4 & 0xfffc);
        if pu_var4 == ctx.PTR_LOOP_1050_000e {
            ctx.PTR_LOOP_1050_000e = pu_var5;
        }
        (pu_var4[0x2] + 0x2) = pu_var4[0x1];
        (pu_var4[0x1] + 0x4) = pu_var4[0x2];
        pu_var4 = ((*pu_var5 & 0xfffc) + pu_var5);
    }
    pu_var4[-0x1] = *pu_var5 & 0xfffc;
    u_var3 = *ctx.DAT_1050_0004;
    pu_var1 = pu_var4 + -0x1;
    if u_var3 <= *pu_var1 && *pu_var1 != u_var3 {
        u_var3 = *pu_var5 & 0xfffc;
        *ctx.DAT_1050_0004 = u_var3;
    }
    pu_var1 = pu_var4;
    pu_var1 = pu_var1 & 0xfd;
    if b_var6 {
        if (ctx.PTR_LOOP_1050_0010 + 0x2) != ctx.PTR_LOOP_1050_0010 {
            pb_var2 = (ctx.DAT_1050_0004 + 0x3);
            *pb_var2 = *pb_var2 & 0x7f;
        }
        pu_var5[0x2] = ctx.PTR_LOOP_1050_0010;
        u_var3 = (ctx.PTR_LOOP_1050_0010 + 0x2);
        pu_var5[0x1] = u_var3;
        ((ctx.PTR_LOOP_1050_0010 + 0x2) + 0x4) = pu_var5;
        (ctx.PTR_LOOP_1050_0010 + 0x2) = pu_var5;
    }
    ctx.PTR_LOOP_1050_000a = ctx.PTR_LOOP_1050_000a + -0x1;
    if ctx.PTR_LOOP_1050_000a == 0x0 {
        u_var7 = mem_op_1000_0510(ctx, 0x1, 0x0, param_2);
        u_var3 = u_var7;
    }
    return u_var3;
}


pub unsafe fn pass1_1000_0ed4(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: &mut Struct18,
    param_7: u16) -> i32 {
    let pu_var1: *mut u16;
    let pu_var2: *mut u16;
    let u_var3: u16;
    let mut ppu_var4: &mut Struct18;
    let u_var5: u16;
    let pu_var6: *mut u16;
    let pu_var7: *mut u16;
    let uvar8: u16;
    let uvar9: u16;
    let uvar10: u16;
    let l_stack12: i32;
    let u_stack8: u16;
    let ustack: u16;
    let ustack4: u16;

    if (&ctx.PTR_LOOP_1050_000c & 0xfff8) == 0xcad0 {
        UStack6 = 0x0;
        ustack4 = &ctx.PTR_LOOP_1050_0002;
        if (param_3 & 0x8) == 0x0 {
            ppu_var4 = param_6;
        } else {
            ppu_var4 = 0x0;
            param_2 = 0x0;
        }
        l_stack12 = CONCAT22(param_2, ppu_var4);
        u_stack8 = pass1_1000_0fb8(ctx, param_1, param_4, param_6, param_5, param_3, ppu_var4, param_2);
        if u_stack8 == 0x0 {
            return CONCAT22(param_7, param_6);
        }
        if (param_3 & 0x8) == 0x0 {
            l_stack12 = mem_op_1000_0a48(ctx, param_3, param_4, param_5, CONCAT22(ustack4, UStack6), param_1);
            // u_var3 = (l_stack12 >> 0x10);
            pu_var7 = l_stack12;
            if l_stack12 != 0x0 {
                pu_var6 = param_6;
                // TODO
                // for (u_var5 = u_stack8 >> 0x1; u_var5 != 0x0; u_var5 -= 0x1) {
                //   pu_var2 = pu_var7;
                //   pu_var7 = pu_var7 + 0x1;
                //   pu_var1 = pu_var6;
                //   pu_var6 = pu_var6 + 0x1;
                //   *pu_var2 = *pu_var1;
                // }

                // TODO
                // for (u_var5 = ((u_stack8 & 0x1) != 0x0); u_var5 != 0x0; u_var5 -= 0x1) {
                //   pu_var2 = pu_var7;
                //   pu_var7 = (pu_var7 + 0x1);
                //   pu_var1 = pu_var6;
                //   pu_var6 = (pu_var6 + 0x1);
                //   *pu_var2 = *pu_var1;
                // }
                call_fn_ptr_1000_0dc6(ctx, param_6, param_1);
            }
            return l_stack12;
        }
        if (param_5 | param_4) == 0x0 {
            return 0x0;
        }
        uvar8 = 0x5;
        uvar9 = UStack6;
        uvar10 = ustack4;
    } else {
        uvar8 = 0xe;
        uvar9 = 0x0;
        uvar10 = 0x0;
    }
    pass1_1000_1e61(ctx, param_1, uvar8, uvar9, uvar10);
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1000_0fb8(ctx: &mut AppContext, param_1: u16, param_2: u16, param_3: i16, param_4: u16, param_5: u16,
                              param_6: *mut u16, param_7: u16) -> u16

{
    let pu_var1: *mut u16;
    let b_var2: u8;
    let u_var3: u16;
    let bvar4: bool;
    let i_var5: i16;
    let u_var6: u16;
    let pu_var7: *mut u16;
    let pu_var8: *mut u16;
    let u_var9: u32;
    let u_stack4: u16;

    if (param_4 | param_2) == 0x0 {
        pass1_1000_1e61(ctx,
                        param_1,
                        0x4,
                        ctx.PTR_LOOP_1050_0000,
                        ctx.PTR_LOOP_1050_0002);
        if (param_7 | param_6) != 0x0 {
            param_6[0x1] = 0x0;
            *param_6 = 0x0;
            return 0x0;
        }
        return 0x1;
    }
    b_var2 = ctx.PTR_LOOP_1050_000c & 0x7;
    if (ctx.PTR_LOOP_1050_000c & 0x7) != 0x0 {
        if b_var2 == 0x1 {
            u_var3 = (ctx.PTR_LOOP_1050_0000 + 0x18);
            if false {
                return 0x0;
            }
            if param_4 == 0x0 {
                if param_2 <= u_var3 {
                    return 0x0;
                }
                return u_var3;
            }
            return u_var3;
        }
        if b_var2 != 0x2 {
            if b_var2 != 0x3 {
                if (param_7 | param_6) != 0x0 {
                    param_6[0x1] = 0x0;
                    *param_6 = 0x0;
                    return 0x0;
                }
                return 0x1;
            }
            if (((param_7 | param_6) != 0x0) && (param_4 == 0x0)) && (false || (param_2 <= (ctx.PTR_LOOP_1050_0000 + 0x1c))) {
                u_var9 = pass1_1000_1284(ctx, CONCAT22(0x1050, param_3), param_1);
                u_var3 = u_var9;
                if u_var9 <= CONCAT22(param_4, param_2) {
                    return u_var3;
                }
                if (false) && (u_var3 <= param_2) {
                    return u_var3;
                }
                return param_2;
            }
            i_var5 = mem_1000_0670(param_5,
                                   CONCAT22(param_7, param_6),
                                   param_2,
                                   0x0,
                                   param_4,
                                   param_1);
            if i_var5 != 0x0 {
                return 0x0;
            }
            if (param_7 | param_6) != 0x0 {
                return 0x0;
            }
            return 0x1;
        }
    }
    pu_var8 = (param_3 + -0x2);
    u_var3 = *pu_var8 & 0x7ffc;
    u_stack4 = u_var3 - 0x2;
    if ((param_3 + -0x1) & 0x80) != 0x0 {
        u_stack4 = u_var3 - 0x6;
    }
    if (true) && (param_4 != 0x0 || (u_stack4 < param_2)) {
        if (true) {
            if param_4 != 0x0 {
                return u_stack4;
            }
            if (ctx.PTR_LOOP_1050_0000 + 0x1c) < param_2 {
                return u_stack4;
            }
        }
    }
    BVar4 = pass1_1000_115c(ctx, param_2, pu_var8);
    if BVar4 == 0x0 {
        return u_stack4;
    }
    if (param_5 & 0x1) != 0x0 {
        u_var3 = (*pu_var8 & 0x7ffc) - 0x2;
        if u_stack4 < param_2 {
            pu_var7 = (u_stack4 + param_3);
            i_var5 = -u_stack4;
        } else {
            if u_var3 <= param_2 {
                return 0x0;
            }
            pu_var7 = (param_2 + param_3);
            i_var5 = -param_2;
        }
        u_var3 += i_var5;
        // TODO
        // for (u_var6 = u_var3 >> 0x1; u_var6 != 0x0; u_var6 -= 0x1) {
        //   pu_var1 = pu_var7;
        //   pu_var7 = pu_var7 + 0x1;
        //   *pu_var1 = 0x0;
        // }
        if (u_var3 & 0x1) != 0x0 {
            *pu_var7 = 0x0;
        }
    }
    return 0x0;
}


pub unsafe fn pass1_1000_115c(ctx: &mut AppContext, param_1: i16, param_2: *mut u16) -> bool

{
    let pb_var1: *mut u8;
    let pu_var2: *mut u16;
    let u_var3: u16;
    let u_var4: u16;
    let pu_var5: *mut u16;
    let i_var6: i16;
    let u_stack4: u16;

    u_var3 = *param_2 & 0x7ffc;
    u_var4 = param_1 + 0x5 & 0xfffc;
    u_var4 = (u_var4 - 0x8 & !-(u_var4 < 0x8)) + 0x8;
    if u_var3 < u_var4 {
        pu_var5 = (u_var3 + param_2);
        if ((pu_var5 & 0x1) != 0x0) || ((*pu_var5 & 0xfffc) + u_var3 < u_var4) {
            return false;
        }
        if pu_var5 == ctx.PTR_LOOP_1050_000e {
            ctx.PTR_LOOP_1050_000e = pu_var5[0x1];
        }
        (pu_var5[0x2] + 0x2) = pu_var5[0x1];
        (pu_var5[0x1] + 0x4) = pu_var5[0x2];
        u_stack4 = ((*pu_var5 & 0xfffc) + u_var3) - u_var4;
        if u_stack4 < ctx.s_version__d__d_1050_0012._0_2_ {
            pu_var2 = param_2;
            *pu_var2 = *pu_var2 + (*pu_var5 & 0xfffc);
            pb_var1 = (pu_var5 + (*pu_var5 & 0xfffc));
            *pb_var1 = *pb_var1 | 0x2;
            return true;
        }
    } else {
        u_stack4 = u_var3 - u_var4;
        if u_stack4 < ctx.s_version__d__d_1050_0012._0_2_ {
            return true;
        }
        pu_var5 = (u_var3 + param_2);
        if (pu_var5 & 0x1) == 0x0 {
            u_stack4 += *pu_var5 & 0xfffc;
            if pu_var5 == ctx.PTR_LOOP_1050_000e {
                ctx.PTR_LOOP_1050_000e = pu_var5[0x1];
            }
            (pu_var5[0x2] + 0x2) = pu_var5[0x1];
            (pu_var5[0x1] + 0x4) = pu_var5[0x2];
        }
        if *ctx.DAT_1050_0004 < u_stack4 {
            *ctx.DAT_1050_0004 = u_stack4;
        }
    }
    *param_2 = *param_2 & 0x8003 | u_var4;
    (u_var4 + param_2) = u_stack4 | 0x2;
    i_var6 = u_var4 + param_2;
    (i_var6 + 0x4) = ctx.PTR_LOOP_1050_0010;
    (i_var6 + 0x2) = (ctx.PTR_LOOP_1050_0010 + 0x2);
    ((ctx.PTR_LOOP_1050_0010 + 0x2) + 0x4) = i_var6;
    (ctx.PTR_LOOP_1050_0010 + 0x2) = i_var6;
    ((i_var6 + u_stack4) + -0x2) = u_stack4;
    pb_var1 = (i_var6 + u_stack4);
    *pb_var1 = *pb_var1 & 0xfd;
    return true;
}


pub fn pass1_1000_1284(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u16) -> u32 {
    let b_var1: u8;
    let u_var2: u16;
    let u_var3: u32;
    let b_var4: u8;
    let u_var5: u16;
    let b_var6: bool;
    let dvar7: u32;
    let u_stack6: u16;
    let i_stack4: i16;

    if (&ctx.PTR_LOOP_1050_000c & 0xfff8) != 0xcad0 {
        pass1_1000_1e61(ctx, param_2, 0xe, None, 0);
        return 0xffffffff;
    }
    b_var1 = &ctx.PTR_LOOP_1050_000c;
    b_var4 = b_var1 & 0x7;
    if (b_var1 & 0x7) != 0x0 {
        if b_var4 == 0x1 {
            u_var3 = 0x0;
            return u_var3 + 0x18;
        }
        if b_var4 != 0x2 {
            if b_var4 != 0x3 {
                return 0xffffffff;
            }
            dvar7 = mem_op_1000_1532(ctx, param_2);
            return CONCAT22((dvar7 >> 0x10) - (dvar7 < 0x14), dvar7 - 0x14,
            );
        }
    }
    u_var2 = (param_1 + -0x2);
    u_var5 = u_var2 & 0x7ffc;
    u_stack6 = u_var5 - 0x2;
    i_stack4 = 0x0;
    if (u_var2 & 0x8000) != 0x0 {
        b_var6 = u_stack6 < 0x4;
        u_stack6 = u_var5 - 0x6;
        i_stack4 = -b_var6;
    }
    return CONCAT22(i_stack4, u_stack6);
}


pub unsafe fn pass1_1000_15ce(
    ctx: &mut AppContext,
    param_1: *mut u16,
    param_2: u16,
    param_3: u16)

{
    let pu_var1: *mut u16;
    let u_var2: u16;

    u_var2 = param_2 | param_1;
    while u_var2 != 0x0 {
        pu_var1 = *param_1;
        param_2 = param_1[0x1];
        GlobalDOSFree16(param_3);
        param_1 = pu_var1;
        param_3 = ctx.s_tile2_bmp_1050_1538;
        u_var2 = param_2 | pu_var1;
    }
    return;
}


pub unsafe fn pass1_1000_16aa(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16) -> u16 {
    let u_var1: u16;
    let l_var1: i32;

    if (param_2 | param_1) == 0x0 {
        u_var1 = mem_1000_167a(ctx, param_3, param_5, param_4);
        return u_var1;
    }
    if param_3 == 0x0 {
        pass1_1000_16ee(ctx, param_1, param_2, param_5);
        return 0x0;
    }
    l_var1 = pass1_1000_0ed4(ctx, param_5, param_6, 0x0, param_3, 0x0, param_1, param_2);
    return l_var1 as u16;
}


pub fn pass1_1000_16ee(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u16,
    param_3: u16) {
    if (param_2 | param_1) != 0x0 {
        call_fn_ptr_1000_0dc6(ctx, param_1, param_3);
    }
    return;
}


pub fn pass1_1000_17e8(
    ctx: &mut AppContext,
    param_1: *mut u8,
    param_2: *mut u8) -> *mut u8 {
    let pu_var1: *mut u8;
    pu_var1 = ctx.PTR_LOOP_1050_5f34;
    ctx.PTR_LOOP_1050_5f34 = param_1;
    ctx.PTR_LOOP_1050_5f36 = param_2;
    return pu_var1;
}


pub unsafe fn pass1_1000_180c(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16) -> u16 {
    let pu_var1: *mut u8;
    let l_var2: i32;

    if (ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0 {
        pu_var1 = mem_op_1000_160a(ctx, param_2, param_3);
        if (param_2 | pu_var1) == 0x0 {
            return 0x0;
        }
    }
    l_var2 = mem_op_1000_0a48(ctx, 0x0, param_1, 0x0, CONCAT22(ctx.PTR_LOOP_1050_5f2e, ctx.PTR_LOOP_1050_5f2c), param_3);
    return l_var2;
}


pub unsafe fn pass1_1000_183c(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16) -> u16 {
    let pu_var1: *mut u8;
    let l_var2: i32;

    pu_var1 = 0x0;
    if (param_2 * param_1 >> 0x10) != 0x0 {
        return 0x0;
    }
    if ((ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0) && (ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx, 0x0, param_3), ctx.PTR_LOOP_1050_5f2e = pu_var1, (pu_var1 | ctx.PTR_LOOP_1050_5f2c) == 0x0) {
        return 0x0;
    }
    l_var2 = mem_op_1000_0a48(ctx, 0x1, (param_2 * param_1), 0x0,
                              CONCAT22(ctx.PTR_LOOP_1050_5f2e, ctx.PTR_LOOP_1050_5f2c), param_3);
    return l_var2;
}


pub unsafe fn pass1_1000_188e(
    ctx: &mut AppContext,
    param_1: *mut u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16) -> u16 {
    let u_var1: u16;
    let l_var2: i32;

    if (param_2 | param_1) == 0x0 {
        u_var1 = pass1_1000_180c(ctx, param_3, param_4, param_5);
        return u_var1;
    }
    if param_3 == 0x0 {
        pass1_1000_18d2(ctx, param_1, param_2, param_5);
        return 0x0;
    }
    l_var2 = pass1_1000_0ed4(ctx, param_5, param_6, 0x0, param_3, 0x0, param_1, param_2);
    return l_var2;
}

pub fn pass1_1000_18d2(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16)

{
    if (param_2 | param_1) != 0x0 {
        call_fn_ptr_1000_0dc6(ctx, param_1, param_3);
    }
    return;
}


pub fn pass1_1000_1a54(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: i16,
    param_3: u16,
    param_4: u16) -> u16

{
    let u_var1: u16;
    let u_var2: u16;

    if (param_2 + 0x14) != -0x4153 {
        pass1_1000_1e61(ctx,param_4, 0xa, 0x0, 0x0);
        return 0x0;
    }
    u_var1 = pass1_1000_1ab0(param_1);
    if u_var1 < (param_2 + 0x18) + 0x14 {
        u_var2 = 0x0;
    } else {
        u_var2 = (param_2 + 0x1a);
        (param_2 + 0x1a) = u_var1;
        (param_2 + 0x1c) = u_var1 >> 0x2;
    }
    return u_var2;
}


pub fn pass1_1000_1ab0(param_1: u16) -> u16

{
    let u_var1: u16;
    let u_var2: u16;

    if param_1 == 0x2000 {
        return 0x2000;
    }
    if param_1 < 0xfff0 {
        if param_1 < 0x1001 {
            return 0x1000;
        }
        u_var1 = 0x2000;
        if param_1 < 0x2001 {
            loop {
                u_var2 = u_var1;
                u_var1 = u_var2 >> 0x1;
                if param_1 > u_var1 {
                    break;
                }
            }
            return u_var2 & 0xfffe;
        }
        while (u_var1 *= 0x2, u_var1 != 0x0) {
            if param_1 <= u_var1 {
                return (u_var1 + 0x10 & -(u_var1 < 0xfff0)) - 0x10;
            }
        }
    }
    return 0xfff0;
}


pub fn pass1_1000_1afe(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: &mut StructA,
    param_3: u16,
    unaff_cs: u16) -> bool
{
    let u_var1: u16;

    if param_1 == 0x0 {
        u_var1 = 0x0;
    } else {
        u_var1 = param_1 + 0x1 & 0xfffe;
    }
    if (param_2 + 0x14) == -0x4153 {
        if (u_var1 < param_1) || ((param_2 + 0x1a) - 0x14 < u_var1) {
            pass1_1000_1e61(ctx, unaff_cs, 0x3, Some(param_2), param_3);
        } else {
            if (param_2 + 0x2) == 0x0 {
                (param_2 + 0x18) = u_var1;
                return true;
            }
        }
        return false;
    }
    pass1_1000_1e61(ctx, unaff_cs, 0xa, None, 0x0);
    return false;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1000_1e61(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: Option<&mut StructA>,
    param_4: u16) -> u16 {
    let i_var1: i16;
    let b_var2: bool;
    let u_var3: u16;
    let u_stack64: u16;
    let u_stack62: u16;
    let u_stack60: u16;
    let pc_stack6: u32;
    let pu_stack4: *mut u8;
    let u_var3: u16;

    u_var3 = ctx.data_seg;
    u_stack62 = param_3;
    u_stack60 = param_4;
    u_stack64 = param_2;
    pu_stack4 = ctx.data_seg;
    if true {
        pc_stack6 = &ctx.PTR_PTR_1050_5f1a;
        if (ctx.PTR_LOOP_1050_5f1c | ctx.PTR_PTR_1050_5f1a) == 0x0 {
            pc_stack6 = 0x0;
            pu_stack4 = 0x0;
        } else {
            i_var1 = mem_op_1000_21b6(ctx.PTR_PTR_1050_5f1a, ctx.PTR_LOOP_1050_5f1c);
            pc_stack6 = ctx.PTR_PTR_1050_5f1a;
            pu_stack4 = ctx.PTR_LOOP_1050_5f1c;
            if i_var1 == 0x0 {
                ctx.PTR_PTR_1050_5f1a = &ctx.PTR_PTR_1050_1f7e;
                ctx.PTR_LOOP_1050_5f1c = &ctx.PTR_LOOP_1050_1000;
                pc_stack6 = &ctx.PTR_PTR_1050_1f7e;
                pu_stack4 = &ctx.PTR_LOOP_1050_1000;
            }
        }
        if (pu_stack4 | pc_stack6) != 0x0 {
            b_var2 = msg_box_op_1000_1f24(&ctx.PTR_PTR_1050_5f1a, ctx.data_seg, 0x0, 0x1000);
            if b_var2 == 0x0 {
                u_var3 = (*pc_stack6)(0x1000, &u_stack64);
            } else {
                pu_stack4 = 0x0;
                pc_stack6 = 0x0;
                u_var3 = 0x0;
            }
            if (pu_stack4 | pc_stack6) != 0x0 {
                pass1_1000_1f68(u_var3);
            }
            return u_var3;
        }
    }
    return 0x0;
}


pub fn pass1_1000_1f68(
    ctx: &mut AppContext,
    param_1: u16) {
    if true && (ctx.PTR_LOOP_1050_5f26 = ctx.PTR_LOOP_1050_5f26 + -0x1, ctx.PTR_LOOP_1050_5f26 < 0x0) {
        ctx.PTR_LOOP_1050_5f26 = 0x0;
    }
    return;
}


pub unsafe fn pass1_1000_1f7e(param_1: *mut u16, param_2: u16) -> bool

{
    let c_var1: u8;
    let b_var2: bool;
    let u_var3: u16;
    let i_var4: i16;
    // let mut pc_var5: String;
    let mut pc_var5 = "".to_string();

    u_var3 = *param_1;
    if false {
        return 0x0;
    }
    if u_var3 == 0xf {
//LAB_1000_1fb6:
        i_var4 = 0x1;
    } else {
        if u_var3 < 0x10 {
            c_var1 = u_var3;
            if c_var1 == '\x02' {
                // TODO
                // goto LAB_1000_1fb6;
            }
            if ('\0' < (c_var1 + -0x2)) && ((c_var1 + -0x3) < 0xf) {
                i_var4 = 0x0;
                // TODO
                // goto LAB_1000_1fbe;
            }
        }
        i_var4 = 0x0;
        u_var3 = 0x1;
    }
//LAB_1000_1fbe:
    pc_var5 = string_1000::string_1000_1fd2(u_var3);
    b_var2 = msg_box_op_1000_214c(0x0, i_var4, &pc_var5, param_2);
    return b_var2;
}


pub fn pass1_1000_1fea(ctx: &mut AppContext) -> bool

{
    let pu_var1: *mut u8;
    let b_var2: bool;

    if (true && (pu_var1 = ctx.PTR_LOOP_1050_5f22 + 0x1, b_var2 = ctx.PTR_LOOP_1050_5f22 == 0x0,
                 ctx.PTR_LOOP_1050_5f22 = pu_var1, b_var2)) && ((ctx.PTR_LOOP_1050_5f20 | ctx.PTR_LOOP_1050_5f1e) != 0x0) {
        ctx.PTR_LOOP_1050_5f22 = &ctx.PTR_LOOP_1050_0002;
    }
    if true {
        return true;
    }
    return false;
}


pub unsafe fn pass1_1000_201c(param_1: i16, param_2: i16, param_3: u16) {
    let u_var1: u16;
    let u_var2: u32;
    let u_var3: u16;
    let bvar4: bool;
    let i_var5: i16;
    let u_var6: u16;

    if param_1 == 0x0 {
        (param_2 + 0x6) = 0x0;
        (param_2 + 0x4) = 0x0;
    }
    u_var3 = (param_2 + 0x6) | (param_2 + 0x4);
    while u_var3 != 0x0 {
        BVar4 = pass1_1000_206c((param_2 + 0x4), (param_2 + 0x6));
        if BVar4 == 0x0 {
            u_var2 = (param_2 + 0x4);
            // u_var6 = (u_var2 >> 0x10);
            i_var5 = u_var2;
            u_var1 = (i_var5 + 0x2c);
            (param_2 + 0x4) = (i_var5 + 0x2a);
            (param_2 + 0x6) = u_var1;
        } else {
            mem_op_1000_1b9a(0x1, (param_2 + 0x4), (param_2 + 0x6),
                             param_3);
        }
        u_var3 = (param_2 + 0x6) | (param_2 + 0x4);
    }
    return;
}


pub unsafe fn pass1_1000_206c(param_1: u16, param_2: u16) -> bool

{
    let u_var1: u16;

    u_var1 = pass1_1000_21d2(0x2, 0x42, param_1, param_2, 0x1);
    if (u_var1 != 0x0) && ((param_1 + 0x14) == -0x4153) {
        return true;
    }
    return false;
}


pub fn pass1_1000_20a2(param_1: u16, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let u_stack8: u16;
    let u_stack4: u16;

    i_var1 = (param_1 + 0x2e);
    u_var2 = (param_1 + 0x30);
    u_stack8 = 0x0;
    u_var3 = (i_var1 + 0x4);
    u_stack4 = (i_var1 + 0x6);
    u_var7 = 0x0;
    if (u_stack4 | u_var3) != 0x0 {
        while (u_var6 = u_var3, u_var4 = u_stack4, u_var6 != param_1 || (u_stack4 != param_2)) {
            u_var3 = (u_var6 + 0x2a);
            u_stack4 = (u_var6 + 0x2c);
            u_var7 = u_var6;
            u_stack8 = u_var4;
            if (u_stack4 | u_var3) == 0x0 {
                return;
            }
        }
        if (u_stack8 | u_var7) != 0x0 {
            u_var2 = (u_var6 + 0x2c);
            (u_var7 + 0x2a) = (u_var6 + 0x2a);
            (u_var7 + 0x2c) = u_var2;
            return;
        }
        u_var5 = (u_var6 + 0x2c);
        (i_var1 + 0x4) = (u_var6 + 0x2a);
        (i_var1 + 0x6) = u_var5;
    }
    return;
}


pub unsafe fn pass1_1000_21d2(
    param_1: u8,
    param_2: i32,
    param_3: u16,
    param_4: u16,
    param_5: u8) -> u16

{
    let mut u_var1 = 0u32;
    let b_var2: bool;

    if (true) {
        b_var2 = mem_op_1000_1dfa(0x0, param_1, param_3, param_4);
        if b_var2 != 0x0 {
            return 0x0;
        }
        if (param_1 & 0x4) == 0x0 {
            u_var1 = SegmentLimit(param_4);
            if !((u_var1 >> 0x10) & 0x1) {
                return 0x0;
            }
            if param_2 == 0x0 {
                return 0x1;
            }
            if CARRY4(param_3, param_2 - 0x1) {
                return 0x0;
            }
            if param_3 + (param_2 - 0x1) <= u_var1 {
                return 0x1;
            }
            return 0x0;
        }
    }
    b_var2 = pass1_1000_22c0(param_3, param_4, param_2, param_2._2_2_, _param_1);
    if b_var2 == 0x0 {
        return 0x0;
    }
    return 0x1;
}


pub unsafe fn pass1_1000_2242(param_1: u16, param_2: u16, param_3: u16, param_4: i16, param_5: u16,
                              param_6: *mut u8) -> u32

{
    let u_var1: u16;
    let u_var2: u16;
    let b_var3: bool;

    u_var1 = param_2 | param_1;
    loop {
        if u_var1 == 0x0 {
            return 0x0;
        }
        u_var1 = param_1;
        if param_2 != 0x0 {
            u_var1 = 0xffff;
        }
        if CARRY2(param_3, u_var1) != false {
            u_var1 = -param_3;
        }
        b_var3 = param_1 < u_var1;
        param_1 -= u_var1;
        param_2 -= b_var3;
        u_var2 = (*param_6)(u_var1, param_5, param_3, param_4);
        if (u_var2 != 0x0) { break; }
        b_var3 = CARRY2(param_3, u_var1);
        param_3 += u_var1;
        param_4 += b_var3 * 0x100;
        u_var1 = param_2 | param_1;
    }
    return CONCAT22(param_2 + CARRY2(u_var2, param_1), u_var2 + param_1);
}


pub unsafe fn pass1_1000_22c0(param_1: u16, param_2: u16, param_3: u16, param_4: u16,
                              param_5: u1) -> bool

{
    let u_var1: u32;

    u_var1 = pass1_1000_2242(param_3, param_4, param_1, param_2, param_5, 0x1dfa);
    if u_var1 == 0x0 {
        return 0x1;
    }
    return 0x0;
}


pub fn pass1_1000_24db(
    ctx: &mut AppContext,
    param_1: i16,
    param_2: u16) {
    let pc_var1: u32;
    let i_var2: i16;
    let u_var3: u16;
    let c_var4: u8;
    let u_var5: u16;

    i_var2 = param_2 + 0x1;
    u_var5 = SUB42(ctx.data_seg, 0x0);
    ctx.PTR_LOOP_1050_5fc9._0_1_ = 0x0;
    u_var3 = 0x1;
    if (false) {
        fn_ptr_op_1000_2594(0x68b6, 0x68b6);
        fn_ptr_op_1000_2594(&ctx.PTR_LOOP_1050_6210, 0x620c);
        ret_op_1000_55ac(param_1, u_var3, u_var5, i_var2);
    }
    c_var4 = (u_var3 >> 0x8);
    fn_ptr_op_1000_2594(&ctx.PTR_LOOP_1050_6210, &ctx.PTR_LOOP_1050_6210);
    fn_ptr_op_1000_2594(&ctx.PTR_LOOP_1050_6210, &ctx.PTR_LOOP_1050_6210);
    dos3_op_1000_256b();
    if (c_var4 == '\0') {
        if (true) {
            pc_var1 = swi(0x21);
            (*pc_var1)();
        } else {
            DOS3Call(&ctx.PTR_LOOP_1050_1000);
        }
    }
    return;
}


pub fn pass1_1000_25a8(param_1: u16, param_2: u16) {
    pass1_1000_2913(ctx, 0xfc, param_1, param_2);
    pass1_1000_2913(ctx, 0xff, param_1, param_2);
    return;
}


pub unsafe fn pass1_1000_25d2(ctx: &mut AppContext, param_1: i16, param_2: i16, param_3: u16, param_4: u16, param_5: u16, param_6: *mut u8) -> *mut i16

{
    let pu_var1: *mut u16;
    let pi_var2: *mut i16;
    let mut pc_var3: String;
    let pu_var4: *mut u8;
    let u_var5: u16;
    let pi_var6: *mut i16;
    // let mut str: String;
    let pc_var10 = "".String();
    let pi_var7: *mut i16;
    let mut pc_var8: String;
    let i_var9: i16;

    pu_var4 = (param_2 + 0x1 & 0xfffe);
    if (pu_var4 < &param_1) && (u_var5 = -(pu_var4 + -&param_1), pu_var1 = &ctx.PTR_LOOP_1050_000a,
                                *pu_var1 < u_var5 || *pu_var1 == u_var5) {
        pu_var1 = &ctx.PTR_LOOP_1050_000c;
        if u_var5 <= *pu_var1 && *pu_var1 != u_var5 {
            &ctx.PTR_LOOP_1050_000c = u_var5;
        }
        // WARNING: Could not recover jumptable at 0x100025f0. Too many
        // branches
        // WARNING: Treating indirect jump as call
        pi_var6 = (*param_6)();
        return pi_var6;
    }
    i_var9 = 0x0;
    pass1_1000_25a8(param_3, param_4);
    pass1_1000_2913(ctx, i_var9, param_3, param_4);
    pc_var10 = poss_str_op_1000_28dc(ctx, 0x0);
    if pc_var10 != 0x0 {
        i_var9 = 0x9;
        if *pc_var10 == 'M' {
            i_var9 = 0xf;
        }
        pc_var10 = pc_var10 + i_var9;
        i_var9 = 0x22;
        pc_var8 = pc_var10;
        loop {
            if i_var9 == 0x0 { break; }
            i_var9 += -0x1;
            pc_var3 = pc_var8;
            pc_var8 = pc_var8 + 0x1;
            if *pc_var3 == '\r' {
                break;
            }
        }
        pc_var8[-0x1] = '\0';
    }
    FatalAppExit16(param_4, pc_var10);
    FatalExit();
    pi_var6 = &ctx.PTR_LOOP_1050_63fe;
    loop {
        pi_var2 = pi_var6;
        pi_var6 = pi_var6 + 0x1;
        i_var9 = *pi_var2;
        pi_var7 = pi_var6;
        if (i_var9 == param_1) || (pi_var7 = (i_var9 + 0x1), pi_var7 == 0x0) {
            return pi_var7;
        }
        i_var9 = -0x1;
        loop {
            if (i_var9 == 0x0) { break; }
            i_var9 += -0x1;
            pi_var2 = pi_var6;
            pi_var6 = (pi_var6 + 0x1);
            if *pi_var2 == '\0' {
                break;
            }
        }
    }
}


// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 :
// 0x1000270c
// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram

pub fn pass1_1000_262c(
    ctx: &mut AppContext,
    param_1: *mut u8,
    param_2: *mut u8,
    param_3: &str,
    param_4: HINSTANCE16,
    in_dx: *mut u8,
    unaff_es: u16,
)

{
    let mut pc_var1: String;
    let c_var2: u8;
    let u_var3: u16;
    let pu_var4: *mut u8;
    let ivar5: i16;
    let u_var6 = 0u16;
    let u_var7: u16;
    let u_var8: u16;
    // let in_dx: *mut u8;
    let i_var9: i16;
    // char * *ppc_var10;
    let mut ppc_var10: Vec<String>;
    let mut pc_var11: String;
    let mut pc_var12: String;
    let mut pc_var13: String;
    // let unaff_es: u16;
    let u_var14: u16;
    let pu_stack4: *mut u8;
    let mut p_cstack2: String;

    ctx.PTR_LOOP_1050_5fd2 = param_1;
    ctx.PTR_LOOP_1050_5fd4 = param_2;
    param_2 = 0x263d;
    param_1 = pass1_1000_2950(0x8, in_dx, unaff_es, param_4);
    p_cstack2 = ctx.PTR_LOOP_1050_5f4c;
    pu_stack4 = in_dx;
    ctx.PTR_LOOP_1050_5fc2 = param_1;
    ctx.PTR_LOOP_1050_5fc4 = in_dx;
    ivar5 = GetModuleFileName16(param_4, (ctx.s_You_may_not_run_a_turn__The_game_1050_00df + 0x25), param_1);
    pu_stack4[ivar5] = 0x0;
    i_var9 = 0x1;
    ctx.PTR_LOOP_1050_5fb8 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
    pc_var11 = (ctx.s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
//LAB_1000_266c:
    loop {
        loop {
            pc_var1 = pc_var11;
            pc_var11 = pc_var11 + 0x1;
            c_var2 = *pc_var1;
            if c_var2 != ' ' {
                break;
            }
        }
        if c_var2 != '\t' {
            break;
        }
    }
    if (c_var2 != '\r') && (c_var2 != '\0') {
        ctx.PTR_LOOP_1050_5fb8 = ctx.PTR_LOOP_1050_5fb8 + 0x1;
        loop {
            pc_var11 = pc_var11 + -0x1;
//LAB_1000_267f:
            pc_var1 = pc_var11;
            pc_var11 = pc_var11 + 0x1;
            c_var2 = *pc_var1;
            if (c_var2 == ' ') || (c_var2 == '\t') {
                // goto LAB_1000_266c;
            }
            if (c_var2 == '\r') || (c_var2 == '\0') { break; }
            if c_var2 == '\"' {
//LAB_1000_26b8:
                loop {
                    loop {
                        loop {
                            pc_var1 = pc_var11;
                            pc_var11 = pc_var11 + 0x1;
                            c_var2 = *pc_var1;
                            if (c_var2 == '\r') || (c_var2 == '\0') {
                                // goto LAB_1000_26e8;
                            }
                            if c_var2 == '\"' {
                                // goto LAB_1000_267f;
                            }
                            if c_var2 == '\\' { break; }
                            i_var9 += 0x1;
                        }
                        u_var7 = 0x0;
                        loop {
                            pc_var13 = pc_var11;
                            u_var7 += 0x1;
                            pc_var11 = pc_var13 + 0x1;
                            c_var2 = *pc_var13;
                            if c_var2 == '\\' {
                                break;
                            }
                        }
                        if c_var2 == '\"' { break; }
                        i_var9 += u_var7;
                        pc_var11 = pc_var13;
                    }
                    i_var9 = i_var9 + (u_var7 >> 0x1) + ((u_var7 & 0x1) != 0x0);
                    if (u_var7 & 0x1) == 0 {
                        break;
                    }
                }
                // TODO: goto LAB_1000_267f;
            }
            if c_var2 != '\\' {
                i_var9 += 0x1;
                // TODO: goto LAB_1000_267f;
            }
            u_var7 = 0x0;
            loop {
                u_var7 += 0x1;
                pc_var1 = pc_var11;
                pc_var11 = pc_var11 + 0x1;
                c_var2 = *pc_var1;
                if c_var2 == '\\' {
                    break;
                }
            }
            if c_var2 == '\"' {
                i_var9 = i_var9 + (u_var7 >> 0x1) + ((u_var7 & 0x1) != 0x0);
                if (u_var7 & 0x1) == 0x0 {
                    // TODO: goto LAB_1000_26b8;
                }
                // TODO: goto LAB_1000_267f;
            }
            i_var9 += u_var7;
        }
    }
//LAB_1000_26e8:
    p_cstack2 = ctx.data_seg;
    i_var9 = -((ctx.PTR_LOOP_1050_5fb8 + (ctx.PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + i_var9 + 0x1) & 0xfffe);
    ctx.PTR_LOOP_1050_5fba = (&param_1 + i_var9);
    pc_var13 = (&param_1 + (ctx.PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + i_var9);
    ctx.PTR_LOOP_1050_5fbc = param_3;
    (&p_cstack2 + i_var9) = param_3;
    pu_var4 = ctx.PTR_LOOP_1050_5fc4;
    u_var14 = (&p_cstack2 + i_var9);
    (&param_1 + i_var9) = ctx.PTR_LOOP_1050_5fc2;
    (&param_2 + i_var9) = pu_var4;
    ppc_var10 = (&stack0x0004 + i_var9);
    (&p_cstack2 + i_var9) = (&param_1 + i_var9);
    (&pu_stack4 + i_var9) = ctx.s_tile2_bmp_1050_1538;
    (&stack0xfffa + i_var9) = 0x271f;
    u_var6 = pass1_1000_29dc(param_3);
    u_var3 = &ctx.PTR_LOOP_1050_5f7e;
    pc_var11 = (ctx.s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
//LAB_1000_272e:
    loop {
        loop {
            pc_var1 = pc_var11;
            pc_var11 = pc_var11 + 0x1;
            c_var2 = *pc_var1;
            if c_var2 != ' ' {
                break;
            }
        }
        if c_var2 == '\t' {
            break;
        }
    }
    if (c_var2 == '\r') || (c_var2 == '\0') {
//LAB_1000_27c1:
        (&p_cstack2 + i_var9) = ctx.s_tile2_bmp_1050_1538;
        (&pu_stack4 + i_var9) = 0x27c5;
        u_var6 = pass1_1000_29dc(param_3);
        *ppc_var10 = 0x0;
        ppc_var10[0x1] = 0x0;
        // WARNING: Could not recover jumptable at 0x100027d2. Too many
        // branches
        // WARNING: Treating indirect jump as call
        (*&ctx.PTR_LOOP_1050_5fd2)();
        ctx._PTR_LOOP_1050_5fc2 = CONCAT22(ctx.PTR_LOOP_1050_5fc4, ctx.PTR_LOOP_1050_5fc2);
        return;
    }
    *ppc_var10 = pc_var13;
    ppc_var10[0x1] = param_3;
    ppc_var10 = ppc_var10 + 0x2;
    loop {
        pc_var11 = pc_var11 + -0x1;
//LAB_1000_274f:
        pc_var1 = pc_var11;
        pc_var11 = pc_var11 + 0x1;
        c_var2 = *pc_var1;
        if ((c_var2 == ' ') || (c_var2 == '\t')) {
            pc_var1 = pc_var13;
            pc_var13 = pc_var13 + 0x1;
            *pc_var1 = '\0';
//       TODO: goto LAB_1000_272e;
        }
        if ((c_var2 == '\r') || (c_var2 == '\0')) {
//LAB_1000_27be:
            *pc_var13 = '\0';
//       TODO: goto LAB_1000_27c1;
        }
        pc_var12 = pc_var11;
        if (c_var2 == '\"') {
//LAB_1000_278b:
            loop {
                pc_var11 = pc_var12 + 0x1;
                c_var2 = *pc_var12;
                if ((c_var2 == '\r') || (c_var2 == '\0')) {
                    // goto LAB_1000_27be;
                }
                if (c_var2 == '\"') {
                    break;
                }
                if (c_var2 == '\\') {
                    u_var7 = 0x0;
                    loop {
                        pc_var12 = pc_var11;
                        u_var7 += 0x1;
                        pc_var11 = pc_var12 + 0x1;
                        c_var2 = *pc_var12;
                        if c_var2 != '\\' {
                            break;
                        }
                    }
                    if (c_var2 == '\"') {
                        // TODO: refactor for loop
                        // for (u_var8 = u_var7 >> 0x1; u_var8 != 0x0; u_var8 -= 0x1) {
                        //   pc_var1 = pc_var13;
                        //   pc_var13 = pc_var13 + 0x1;
                        //   *pc_var1 = '\\';
                        // }
                        if ((u_var7 & 0x1) == 0x0) { break; }
                        pc_var1 = pc_var13;
                        pc_var13 = pc_var13 + 0x1;
                        *pc_var1 = '\"';
                        pc_var12 = pc_var11;
                    } else {
                        // TODO: refactor for loop
                        // for (; u_var7 != 0x0; u_var7 -= 0x1) {
                        //   pc_var1 = pc_var13;
                        //   pc_var13 = pc_var13 + 0x1;
                        //   *pc_var1 = '\\';
                        // }
                    }
                } else {
                    pc_var1 = pc_var13;
                    pc_var13 = pc_var13 + 0x1;
                    *pc_var1 = c_var2;
                    pc_var12 = pc_var11;
                }
            }
//       TODO: goto LAB_1000_274f;
        }
        if (c_var2 != '\\') {
            pc_var1 = pc_var13;
            pc_var13 = pc_var13 + 0x1;
            *pc_var1 = c_var2;
//       TODO: goto LAB_1000_274f;
        }
        u_var7 = 0x0;
        loop {
            u_var7 += 0x1;
            pc_var1 = pc_var11;
            pc_var11 = pc_var11 + 0x1;
            c_var2 = *pc_var1;
            if c_var2 != '\\' {
                break;
            }
        }
        if (c_var2 == '\"') {
            // TODO: refactor for loop
            // for (u_var8 = u_var7 >> 0x1; u_var8 != 0x0; u_var8 -= 0x1) {
            //   pc_var1 = pc_var13;
            //   pc_var13 = pc_var13 + 0x1;
            //   *pc_var1 = '\\';
            // }
            pc_var12 = pc_var11;
            if ((u_var7 & 0x1) == 0x0) {
                // goto LAB_1000_278b;
            }
            pc_var1 = pc_var13;
            pc_var13 = pc_var13 + 0x1;
            *pc_var1 = '\"';
//       TODO: goto LAB_1000_274f;
        }
        // TODO: refactor for loop
        // for (; u_var7 != 0x0; u_var7 -= 0x1) {
        //   pc_var1 = pc_var13;
        //   pc_var13 = pc_var13 + 0x1;
        //   *pc_var1 = '\\';
        // }
    }
}


pub unsafe fn pass1_1000_27d6(ctx: &mut AppContext, param_1: *mut u16) {
    let pi_var1: *mut i16;
    let mut pc_var2: String;
    let pu_var3: *mut u16;
    let pi_var4: *mut i16;
    let c_var5: u8;
    let svar6: SEGPTR;
    let pu_var7: *mut u16;
    let ppu_var8: *mut *mut u16;
    let i_var9: i16;
    let u_var10: u16;
    let pu_var11: *mut u16;
    let i_var12: i16;
    let pi_var13: *mut i16;
    let pi_var14: *mut i16;
    let mut pc_var15: String;
    let pi_var16: *mut i16;
    let b_var17: bool;
    let pu_var18: *mut u16;

    svar6 = GetDOSEnvironment16();
    if (svar6 != 0x0) {
        param_1 = 0x0;
    }
    i_var12 = 0x0;
    pc_var15 = 0x0;
    i_var9 = -0x1;
    if (param_1 != 0x0) {
        c_var5 = *0x0;
        while (c_var5 != '\0') {
            loop {
                if (i_var9 == 0x0) { break; }
                i_var9 += -0x1;
                pc_var2 = pc_var15;
                pc_var15 = pc_var15 + 0x1;
                if *pc_var2 == '\0' {
                    break;
                }
            }
            i_var12 += 0x1;
            pc_var2 = pc_var15;
            pc_var15 = pc_var15 + 0x1;
            c_var5 = *pc_var2;
        }
    }
    u_var10 = 0x9;
    pu_var11 = param_1;
    pu_var7 = pass1_1000_2950(0x9, param_1, param_1, ctx.s_tile2_bmp_1050_1538);
    pu_var18 = pu_var11;
    ppu_var8 = pass1_1000_2950(u_var10, pu_var11, param_1, ctx.s_tile2_bmp_1050_1538);
    pi_var13 = 0x0;
    ctx.PTR_LOOP_1050_5fbe = ppu_var8;
    ctx.PTR_LOOP_1050_5fc0 = pu_var11;
    loop {
        if (i_var12 == 0x0) {
            *ppu_var8 = 0x0;
            ppu_var8[0x1] = 0x0;
            return;
        }
        b_var17 = *pi_var13 == ctx.s__C_FILE_INFO__1050_5f5c._0_2_;
        if (b_var17) {
            pi_var16 = ctx.s__C_FILE_INFO__1050_5f5c;
            i_var9 = 0x6;
            pi_var14 = pi_var13;
            loop {
                if (i_var9 == 0x0) {
                    break;
                }
                i_var9 += -0x1;
                pi_var4 = pi_var16;
                pi_var16 = pi_var16 + 0x1;
                pi_var1 = pi_var14;
                pi_var14 = pi_var14 + 0x1;
                b_var17 = *pi_var1 == *pi_var4;
                if b_var17 == false {
                    break;
                }
            }
            if (!b_var17) {
                // goto LAB_1000_2867;
            }
        } else {
//LAB_1000_2867:
            *ppu_var8 = pu_var7;
            ppu_var8[0x1] = pu_var18;
            ppu_var8 = ppu_var8 + 0x2;
        }
        loop {
            pi_var1 = pi_var13;
            pi_var13 = (pi_var13 + 0x1);
            c_var5 = *pi_var1;
            pu_var3 = pu_var7;
            pu_var7 = (pu_var7 + 0x1);
            *pu_var3 = c_var5;
            if c_var5 == '\0' {
                break;
            }
        }
        i_var12 += -0x1;
    }
}


pub fn pass1_1000_2913(
    ctx: &mut AppContext,
    param_1: i16,
    param_2: u16,
    param_3: u16) {
    let mut pc_var1: String;
    let mut pc_var2: String;
    let i_var3: i16;

    if ctx.PTR_LOOP_1050_61ec != 0x0 {
        pc_var2 = poss_str_op_1000_28dc(ctx, param_1);
        if pc_var2 != 0x0 {
            i_var3 = -0x1;
            loop {
                if i_var3 == 0x0 { break; }
                i_var3 += -0x1;
                pc_var1 = pc_var2;
                pc_var2 = pc_var2 + 0x1;
                if *pc_var1 == '\0' {
                    break;
                }
            }
            pass1_1000_55b1(0x2944, param_2, param_3);
        }
    }
    return;
}


pub fn pass1_1000_2950(
    ctx: &mut AppContext,
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u16) -> *mut u16

{
    let pu_var1: *mut u16;
    let u_var2: u16;
    let mut pc_var3: String;
    let pu_var4: *mut u8;
    let mut string_a: String;
    let i_var5: i16;
    let pu_var6: *mut u16;
    let in_ax: u16;
    let pu_var7: *mut u16;
    let mut pc_var8: String;
    let u_var9: u16;

    pu_var4 = ctx.PTR_LOOP_1050_6066;
    ctx.PTR_LOOP_1050_6066 = &ctx.PTR_LOOP_1050_1000;
    u_var9 = param_3;
    pu_var7 = (fn mem_1000_167a(in_AX, param_4, param_2) -> u16;
    ctx.PTR_LOOP_1050_6066 = pu_var4;
    if (param_2 | pu_var7) != 0x0 {
        return pu_var7;
    }
    i_var5 = param_1;
    pass1_1000_25a8(param_3, param_4);
    pass1_1000_2913(ctx, param_1, param_3, param_4);
    string_a = poss_str_op_1000_28dc(ctx, i_var5);
    if string_a != 0x0 {
        i_var5 = 0x9;
        if *string_a == 'M' {
            i_var5 = 0xf;
        }
        string_a = string_a + i_var5;
        i_var5 = 0x22;
        pc_var8 = string_a;
        loop {
            if i_var5 == 0x0 { break; }
            i_var5 += -0x1;
            pc_var3 = pc_var8;
            pc_var8 = pc_var8 + 0x1;
            if *pcVar == '\r' {
                break;
            }
        }
        pc_var8[-0x1] = '\0';
    }
    FatalAppExit16(param_4, string_a);
    FatalExit();
    pu_var7 = &ctx.PTR_LOOP_1050_63fe;
    loop {
        pu_var1 = pu_var7;
        pu_var7 = pu_var7 + 0x1;
        u_var2 = *pu_var1;
        pu_var6 = pu_var7;
        if ((u_var2 == u_var9) || (pu_var6 = (u_var2 + 0x1), pu_var6 == 0x0)) {
            return pu_var6;
        }
        i_var5 = -0x1;
        loop {
            if (i_var5 == 0x0) { break; }
            i_var5 += -0x1;
            pu_var1 = pu_var7;
            pu_var7 = (pu_var7 + 0x1);
            if *pu_var1 == '\0' {
                break;
            }
        }
    }
}


pub fn pass1_1000_29af(param_1: u16) {
    pass1_1000_29b5(param_1 & 0xff);
    return;
}


pub fn pass1_1000_29b5(param_1: u16) {
    let c_var1: u8;

    ctx.PTR_LOOP_1050_5f88._0_1_ = param_1;
    c_var1 = (param_1 >> 0x8);
    if (c_var1 != '\0') {
        // goto
        // LAB_1000_29d2;
    }
    if (ctx.PTR_LOOP_1050_5f88 < 0x22) {
        if (ctx.PTR_LOOP_1050_5f88 < 0x20) {
            if (0x13 < ctx.PTR_LOOP_1050_5f88) {
                // goto
                // LAB_1000_29cc;
            }
        } else {
            param_1 = 0x5;
        }
    } else {
//LAB_1000_29cc:
        param_1 = 0x13;
    }
    c_var1 = *((param_1 & 0xff) + 0x5fd6);
//LAB_1000_29d2:
    ctx.PTR_LOOP_1050_5f78 = c_var1;
    return;
}


pub fn pass1_1000_29dc(param_1: u16) -> u16

{
    if (___EXPORTEDSTUB != (code)
    0xb8) {
    return param_1;
}
    return uRam100029ed;
}


pub fn pass1_1000_2a00(param_1: *mut u16, param_2: i16, param_3: u16, param_4: u16, param_5: u16,
                       param_6: u8) -> u16

{
    let b_var1: bool;
    let pi_var2: *mut i16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let pu_stack20: *mut u8;
    let local_10: u8;
    let u_stack15: u8;
    let local_e: [u8; 8];
    let u_stack6: u16;
    let local_4: u16;
    let i_stack2: i16;

    i_stack2 = param_2 + 0x1;
    local_4 = SUB42(ctx.data_seg, 0x0);
    u_var5 = 0xffff;
    if (((param_1 + 0x5) & 0x40) != 0x0) {
        *(param_1 + 0x5) = 0x0;
        return 0xffff;
    }
    if (((param_1 + 0x5) & 0x83) == 0x0) {
        // goto
        // LAB_1000_2af2;
    }
    u_var5 = pass1_1000_2fa4(param_1, param_3, param_4, param_5, param_6);
    u_stack6 = param_1[0x7a];
    pass1_1000_2cb0(param_1, param_4);
    if (ctx.DAT_1050_5f8a < (param_1 + 0xb)) {
        pi_var2 = pass1_1000_55b1(0x2a63, param_3, param_4);
        if (pi_var2 < 0x0) {
            // goto
            // LAB_1000_2a6a;
        }
//LAB_1000_2a82:
        b_var1 = false;
    } else {
        i_var3 = dos3_call_op_1000_35fe((param_1 + 0xb), &i_stack2);
        if (-0x1 < i_var3) {
            // goto
            // LAB_1000_2a82;
        }
//LAB_1000_2a6a:
        b_var1 = true;
    }
    if (!b_var1) {
        if (u_stack6 == 0x0) {
            // goto
            // LAB_1000_2af2;
        }
        unk_str_op_1000_3d3e(CONCAT22(param_5, &local_10), 0x10505fea);
        pu_stack20 = local_e;
        if (local_10 == '\\') {
            pu_stack20 = &u_stack15;
        } else {
            pass1_1000_3cea(CONCAT22(param_5, &local_10), 0x10505fec);
        }
        pass1_1000_3e82(u_stack6, CONCAT22(param_5, pu_stack20), 0xa);
        u_var4 = dos3_call_1000_514e(&i_stack2);
        if (u_var4 == 0x0) {
            // goto
            // LAB_1000_2af2;
        }
    }
    u_var5 = 0xffff;
//LAB_1000_2af2:
    *(param_1 + 0x5) = 0x0;
    return u_var5;
}


pub fn pass1_1000_2b02(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u8,
                       param_6: u16, param_7: i16) -> *mut u16

{
    let pu_var1: *mut u16;
    let i_stack2: i16;

    i_stack2 = param_7 + 0x1;
    pu_var1 = pass1_1000_35aa();
    if ((param_6 | pu_var1) == 0x0) {
        pu_var1 = 0x0;
    } else {
        pu_var1 = pass1_1000_2d34(param_1, param_2, CONCAT22(param_4, param_3), param_5,
                                  pu_var1, &i_stack2);
    }
    return pu_var1;
}


pub fn pass1_1000_2b3c(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16, param_6: i16)

{
    let i_stack2: i16;

    i_stack2 = param_6 + 0x1;
    pass1_1000_2b02(param_1, param_2, param_3, param_4, 0x0, param_5, &i_stack2);
    return;
}


pub fn pass1_1000_2b5c(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16, param_6: i16, param_7: u16, param_8: u16) -> u16

{
    let u_var1: u16;
    let u_var2: u16;
    let in_af: u8;
    let i_stack2: i16;

    i_stack2 = param_6 + 0x1;
    u_var1 = pass1_1000_2e74(param_1, param_7);
    u_var2 = sys_1000_30b4(param_1, ctx.data_seg,
                           CONCAT22(param_4, param_3), &i_stack2, param_1, param_5,
                           param_7, param_8);
    pass1_1000_2f00(u_var1, param_1, param_5, param_7, param_8, in_af);
    return u_var2;
}


pub fn pass1_1000_2ba0(param_1: u16, param_2: u16, param_3: u16, param_4: u8) {
    pass1_1000_3024(param_1, param_2, param_3, param_4);
    if (ctx.PTR_LOOP_1050_5fc9 != '\0') {
        pass1_1000_3f5c(&stack0xfffe, param_1, param_2, param_3, param_4);
    }
    return;
}


pub fn pass1_1000_2cb0(param_1: *mut u16, param_2: u16) {
    let pu_var1: *mut u16;
    let b_var2: u8;

    b_var2 = (param_1 + 0x5);
    if (((b_var2 & 0x83) != 0x0) && ((b_var2 & 0x8) != 0x0)) {
        pass1_1000_16ee(ctx, param_1[0x3], param_1[0x4], param_2);
        pu_var1 = param_1 + 0x5;
        pu_var1 = pu_var1 & 0xf7;
        param_1[0x3] = 0x0;
        param_1[0x4] = 0x0;
        *param_1 = 0x0;
        param_1[0x1] = 0x0;
        param_1[0x2] = 0x0;
    }
    return;
}


pub fn pass1_1000_2d34(param_1: u16, param_2: u16, param_3: *mut u8, param_4: u8, param_5: *mut u16,
                       param_6: i16) -> *mut u16

{
    let b_var1: u8;
    let b_var2: bool;
    let b_var3: bool;
    let u_var4: u16;
    let u_stack14: u8;
    let b_stack8: u8;
    let u_stack6: u8;
    let i_stack2: i16;

    i_stack2 = param_6 + 0x1;
    b_stack8 = ctx.PTR_LOOP_1050_6062;
    b_var3 = false;
    b_var1 = *param_3;
    if (b_var1 == 0x77) {
        u_var4 = 0x301;
    } else {
        if (0x77 < b_var1) {
            return 0x0;
        }
        if (b_var1 != 0x61) {
            if (b_var1 != 0x72) {
                return 0x0;
            }
            u_var4 = 0x0;
            u_stack6 = 0x1;
//       TODO: goto LAB_1000_2d6c;
        }
        u_var4 = 0x109;
    }
    u_stack6 = 0x2;
//LAB_1000_2d6c:
    b_var2 = true;
//LAB_1000_2d71:
    param_3 = (param_3 & 0xffff0000 | (param_3 + 0x1));
    if ((*param_3 == 0x0) || (!b_var2)) {
        u_var4 = mixed_dos3_call_1000_370a(param_1, param_2, u_var4, param_4, 0x1a4, &i_stack2);
        if (u_var4 < 0x0) {
            return 0x0;
        }
        ctx.PTR_LOOP_1050_5fee = ctx.PTR_LOOP_1050_5fee + 0x1;
        *(param_5 + 0x5) = u_stack6;
        param_5[0x1] = 0x0;
        *param_5 = 0x0;
        param_5[0x4] = 0x0;
        param_5[0x3] = 0x0;
        u_stack14 = u_var4;
        *(param_5 + 0xb) = u_stack14;
        (param_5 + 0x78) = b_stack8;
        param_5[0x2] = 0x0;
        param_5[0x7a] = 0x0;
        return param_5;
    }
    b_var1 = *param_3;
    if (b_var1 == 0x74) {
        if ((u_var4 & 0xc000) == 0x0) {
            u_var4 |= 0x4000;
//       TODO: goto LAB_1000_2d71;
        }
    } else {
        if (0x74 < b_var1) {
            // goto
            // LAB_1000_2da4;
        }
        if (b_var1 == 0x2b) {
            if ((u_var4 & 0x2) != 0x0) {
                // goto
                // LAB_1000_2da4;
            }
            u_var4 = u_var4 & 0xfffe | 0x2;
            u_stack6 = 0x80;
//       TODO: goto LAB_1000_2d71;
        }
        if (b_var1 == 0x62) {
            if ((u_var4 & 0xc000) == 0x0) {
                u_var4 |= 0x8000;
//         TODO: goto LAB_1000_2d71;
            }
        } else {
            if (b_var1 != 0x63) {
                if ((b_var1 != 0x6e) || (b_var3)) {
                    // goto
                    // LAB_1000_2da4;
                }
                b_var3 = true;
                b_stack8 &= 0xbf;
//         TODO: goto LAB_1000_2d71;
            }
            if (!b_var3) {
                b_var3 = true;
                b_stack8 |= 0x40;
//         TODO: goto LAB_1000_2d71;
            }
        }
    }
//LAB_1000_2da4:
    b_var2 = false;
//   TODO: goto LAB_1000_2d71;
}


pub fn pass1_1000_2e74(param_1: *mut u16, param_2: u16) -> u16

{
    let pu_var1: *mut u16;
    let u_var2: u16;
    let u_var3: u16;
    let pu_var4: *mut u16;
    let pu_var5: *mut u16;

    if (ctx.PTR_LOOP_1050_61ec != 0x0) {
        pu_var5 = param_1 + 0x78;
        pu_var4 = 0x5ff2;
        if ((param_1 == 0x621c) || (pu_var4 = &ctx.PTR_LOOP_1050_5ff6, param_1 == 0x6228)) {
            if ((((param_1 + 0x5) & 0xc) == 0x0) && ((pu_var5 & 0x1) == 0x0)) {
                u_var2 = *pu_var4;
                u_var3 = pu_var4[0x1];
                if ((u_var2 | u_var3) == 0x0) {
                    u_var2 = mem_1000_167a(0x200, param_2, u_var3);
                    if (u_var3 == 0x0) {
                        return 0x0;
                    }
                    *pu_var4 = u_var2;
                    pu_var4[0x1] = u_var3;
                }
                param_1[0x3] = u_var2;
                param_1[0x4] = u_var3;
                *param_1 = u_var2;
                param_1[0x1] = u_var3;
                param_1[0x2] = 0x200;
                param_1[0x79] = 0x200;
                pu_var1 = param_1 + 0x5;
                pu_var1 = pu_var1 | 0x2;
                pu_var5 = 0x11;
                return 0x1;
            }
        } else {
            if (ctx.DAT_1050_5f8a <= (param_1 + 0xb)) {
                pu_var1 = pu_var5;
                pu_var1 = pu_var1 | 0x10;
            }
        }
    }
    return 0x0;
}


pub fn pass1_1000_2f00(param_1: i16, param_2: &mut i16, param_3: u16, param_4: u16, param_5: u16,
                       param_6: u8)

{
    if ((((param_2 + 0x78) & 0x10) != 0x0) && ((((param_2 + 0xb) + 0x5f90) & 0x40) != 0x0)) {
        pass1_1000_2fa4(param_2, param_3, param_4, param_5, param_6);
        if (param_1 != 0x0) {
            (param_2 + 0x78) = 0x0;
            param_2[0x79] = 0x0;
            *param_2 = 0x0;
            param_2[0x1] = 0x0;
            param_2[0x3] = 0x0;
            param_2[0x4] = 0x0;
        }
    }
    return;
}


pub fn pass1_1000_2f48(param_1: i32, param_2: i16, param_3: u16, param_4: u16, param_5: u16,
                       param_6: u8) -> u16

{
    let u_var1: u16;
    let pu_var2: *mut u8;
    let i_stack2: i16;

    i_stack2 = param_2 + 0x1;
    if (param_1 == 0x0) {
        u_var1 = pass1_1000_3038(0x0, param_3, param_4, param_5, param_6);
    } else {
        u_var1 = pass1_1000_2fa4(param_1, param_3, param_4, param_5, param_6);
        if (u_var1 == 0x0) {
            if (((param_1 + 0x78) & 0x40) != 0x0) {
                pu_var2 = pass1_1000_400a((param_1 + 0xb),
                                          &i_stack2);
                u_var1 = -(pu_var2 != 0x0);
            }
        } else {
            u_var1 = 0xffff;
        }
    }
    return u_var1;
}


pub fn pass1_1000_2fa4(param_1: &mut i16, param_2: u16, param_3: u16, param_4: u16, param_5: u8) -> u16

{
    let pi_var1: *mut i16;
    let b_var2: u8;
    let i_var3: i16;
    let pu_var4: *mut u8;
    let pu_var5: *mut u8;
    let u_var6: u16;

    u_var6 = 0x0;
    b_var2 = (param_1 + 0x5);
    if (((b_var2 & 0x3) == 0x2) && (((b_var2 & 0x8) != 0x0 || (((param_1 + 0x78) & 0x1) != 0x0)))) {
        pu_var4 = (*param_1 - param_1[0x3]);
        if (0x0 < pu_var4) {
            pu_var5 = mixed_dos3_call_1000_39f2((param_1 + 0xb),
                                                CONCAT22(param_1[0x4], param_1[0x3]), pu_var4, param_2,
                                                param_3, param_4, param_5);
            if (pu_var5 == pu_var4) {
                if (((param_1 + 0x5) & 0x80) != 0x0) {
                    pi_var1 = param_1 + 0x5;
                    pi_var1 = pi_var1 & 0xfd;
                }
            } else {
                pi_var1 = param_1 + 0x5;
                pi_var1 = pi_var1 | 0x20;
                u_var6 = 0xffff;
            }
        }
    }
    i_var3 = param_1[0x4];
    *param_1 = param_1[0x3];
    param_1[0x1] = i_var3;
    param_1[0x2] = 0x0;
    return u_var6;
}


pub fn pass1_1000_3024(param_1: u16, param_2: u16, param_3: u16, param_4: u8) {
    pass1_1000_3038(0x1, param_1, param_2, param_3, param_4);
    return;
}


pub fn pass1_1000_3038(param_1: i16, param_2: u16, param_3: u16, param_4: u16, param_5: u8,
) -> i16

{
    let u_var1: u16;
    let pu_var2: *mut u8;
    let i_var3: i16;
    let i_stack4: i16;

    i_var3 = 0x0;
    i_stack4 = 0x0;
// TODO: refactor for loop
//   for (pu_var2 = &ctx.PTR_LOOP_1050_6210; pu_var2 <= ctx.PTR_LOOP_1050_5ff0;
//       pu_var2 = pu_var2 + 0xc) {
//     if ((param_1 == 0x1) && ((pu_var2[0xa] & 0x83) != 0x0)) {
//       u_var1 = pass1_1000_2f48(CONCAT22(0x1050,pu_var2),&stack0xfffe,param_2,param_3,
//                               param_4,param_5);
//       if (u_var1 != 0xffff) {
//         i_var3 += 0x1;
//       }
//     }
//     else {
//       if ((param_1 == 0x0) &&
//          (((pu_var2[0xa] & 0x2) != 0x0 &&
//           (u_var1 = pass1_1000_2f48(CONCAT22(0x1050,pu_var2),&stack0xfffe,param_2,
//                                    param_3,param_4,param_5), u_var1 == 0xffff)))) {
//         i_stack4 = -0x1;
//       }
//     }
//   }
    if param_1 == 0x1 {
        i_stack4 = i_var3;
    }
    return i_stack4;
}


// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING: Unable to track spacebase fully for stack

pub fn pass1_1000_30a4(param_1: i16, param_2: u16, param_3: u16, param_4: u16, param_5: i16,
                       param_6: u16, param_7: u16, param_8: u16, param_9: u16,
                       param_10: u8) -> u16

{
    let pu_var1: *mut u16;
    let c_var2: u8;
    let mut pc_var3: String;
    let b_var4: u8;
    let u_var5: u16;
    let pu_var6: *mut u16;

    pu_var6 = (param_5 + (param_3 + param_6) + param_10);
    pu_var1 = pu_var6;
    *pu_var1 = *pu_var1 ^ pu_var6;
    pu_var1 = (pu_var6 + param_3 + 0x31);
    *pu_var1 = *pu_var1 ^ param_4;
    pu_var1 = (pu_var6 + -0x3acf);
    *pu_var1 = *pu_var1 ^ param_3;
    pu_var1 = pu_var6 + -0x3794;
    *pu_var1 = *pu_var1 ^ param_2;
    (param_1 + -0x2) = param_4 + 0x1;
    (param_1 + -0x4) = ctx.data_seg;
    (param_1 + -0x6) = param_8;
    (param_1 + -0x8) = 0x30c5;
    exit_1000_25f2((param_1 + -0x8), (param_1 + -0x6),
                   (param_1 + -0x4), 0x214, param_7, param_8, param_9);
    (param_1 + -0x6) = pu_var6;
    (param_1 + -0x8) = param_6 ^ pu_var6;
    (param_1 + -0xc) = 0x0;
    *(param_1 + -0x9) = 0x0;
    pc_var3 = (param_1 + 0x8);
    c_var2 = *pc_var3;
    (param_1 + 0x8) = pc_var3 + 0x1;
    *(param_1 + -0x6) = c_var2;
    if ((c_var2 != '\0') & &(-0x1 < (param_1 + -0xc))) {
        if ((c_var2 - 0x20) < 0x59) {
            b_var4 = ((c_var2 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var4 = 0x0;
        }
        b_var4 = ((b_var4 * '\b' + *(param_1 + -0x9)) + 0x5ffe) >> 0x4;
        (param_1 + -0x9) = b_var4;
// WARNING: Could not recover jumptable at 0x1000310e. Too many
// branches
// WARNING: Treating indirect jump as call
        u_var5 = ((b_var4 * 0x2 + 0x30a4))();
        return u_var5;
    }
    return (param_1 + -0xc);
}


// WARNING (jumptable): Unable to track spacebase fully for stack

pub fn pass1_1000_3113(param_1: u16, param_2: u16) -> u16

{
    let c_var1: u8;
    let mut pc_var2: String;
    let b_var3: u8;
    let u_var4: u16;

    pass1_1000_3552(0x1, param_1, param_2);
    pc_var2 = (param_1 + 0xa);
    c_var1 = *pc_var2;
    (param_1 + 0xa) = pc_var2 + 0x1;
    *(param_1 - 0x4) = c_var1;
    if ((c_var1 != '\0') && (-0x1 < (param_1 - 0xa))) {
        if ((c_var1 - 0x20) < 0x59) {
            b_var3 = ((c_var1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var3 = 0x0;
        }
        b_var3 = ((b_var3 * '\b' + *(param_1 - 0x7)) + 0x5ffe) >> 0x4;
        (param_1 - 0x7) = b_var3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var4 = ((b_var3 * 0x2 + 0x30a4))();
        return u_var4;
    }
    return (param_1 - 0xa);
}


// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub fn pass1_1000_311e(param_1: i16, param_2: u16) -> u16

{
    let c_var1: u8;
    let mut pc_var2: String;
    let b_var3: u8;
    let u_var4: u16;

    (param_1 + -0x12) = 0x0;
    (param_1 + -0xc) = 0x0;
    (param_1 + -0x14) = 0x0;
    (param_1 + -0x6) = 0x20;
    (param_1 + -0xe) = 0xffff;
    pc_var2 = (param_1 + 0xa);
    c_var1 = *pc_var2;
    (param_1 + 0xa) = pc_var2 + 0x1;
    *(param_1 + -0x4) = c_var1;
    if ((c_var1 != '\0') && (-0x1 < (param_1 + -0xa))) {
        if ((c_var1 - 0x20) < 0x59) {
            b_var3 = ((c_var1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var3 = 0x0;
        }
        b_var3 = ((b_var3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        (param_1 + -0x7) = b_var3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var4 = ((b_var3 * 0x2 + 0x30a4))();
        return u_var4;
    }
    return (param_1 + -0xa);
}


// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub fn pass1_1000_3134(param_1: i16, param_2: u16) -> u16

{
    let pb_var1: *mut u8;
    let c_var2: u8;
    let mut pc_var3: String;
    let b_var4: u8;
    let u_var5: u16;

    c_var2 = *(param_1 + -0x4);
    if (c_var2 == '-') {
        pb_var1 = (param_1 + -0x6);
        *pb_var1 = *pb_var1 | 0x4;
    } else {
        if (c_var2 == '+') {
            pb_var1 = (param_1 + -0x6);
            *pb_var1 = *pb_var1 | 0x1;
        } else {
            if (c_var2 == ' ') {
                pb_var1 = (param_1 + -0x6);
                *pb_var1 = *pb_var1 | 0x2;
            } else {
                if (c_var2 == '#') {
                    pb_var1 = (param_1 + -0x6);
                    *pb_var1 = *pb_var1 | 0x80;
                } else {
                    pb_var1 = (param_1 + -0x6);
                    *pb_var1 = *pb_var1 | 0x8;
                }
            }
        }
    }
    pc_var3 = (param_1 + 0xa);
    c_var2 = *pc_var3;
    (param_1 + 0xa) = pc_var3 + 0x1;
    *(param_1 + -0x4) = c_var2;
    if ((c_var2 != '\0') && (-0x1 < (param_1 + -0xa))) {
        if ((c_var2 - 0x20) < 0x59) {
            b_var4 = ((c_var2 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var4 = 0x0;
        }
        b_var4 = ((b_var4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        (param_1 + -0x7) = b_var4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var5 = ((b_var4 * 0x2 + 0x30a4))();
        return u_var5;
    }
    return (param_1 + -0xa);
}


// WARNING (jumptable): Unable to track spacebase fully for stack

pub fn pass1_1000_3168(param_1: i16, param_2: u16) -> u16

{
    let pb_var1: *mut u8;
    let c_var2: u8;
    let mut pc_var3: String;
    let b_var4: u8;
    let u_var5: u16;

    c_var2 = *(param_1 + -0x4);
    if (c_var2 == '*') {
        u_var5 = pass1_1000_34cf(param_1, param_2);
        if (u_var5 < 0x0) {
            u_var5 = -u_var5;
            pb_var1 = (param_1 + -0x6);
            *pb_var1 = *pb_var1 | 0x4;
        }
    } else {
        u_var5 = (param_1 + -0xc) * 0xa + (c_var2 - 0x30);
    }
    (param_1 + -0xc) = u_var5;
    pc_var3 = (param_1 + 0xa);
    c_var2 = *pc_var3;
    (param_1 + 0xa) = pc_var3 + 0x1;
    *(param_1 + -0x4) = c_var2;
    if ((c_var2 != '\0') && (-0x1 < (param_1 + -0xa))) {
        if ((c_var2 - 0x20) < 0x59) {
            b_var4 = ((c_var2 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var4 = 0x0;
        }
        b_var4 = ((b_var4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        (param_1 + -0x7) = b_var4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var5 = ((b_var4 * 0x2 + 0x30a4))();
        return u_var5;
    }
    return (param_1 + -0xa);
}


// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub fn pass1_1000_3194(param_1: i16, param_2: u16) -> u16

{
    let c_var1: u8;
    let mut pc_var2: String;
    let b_var3: u8;
    let u_var4: u16;

    (param_1 + -0xe) = 0x0;
    pc_var2 = (param_1 + 0xa);
    c_var1 = *pc_var2;
    (param_1 + 0xa) = pc_var2 + 0x1;
    *(param_1 + -0x4) = c_var1;
    if ((c_var1 != '\0') && (-0x1 < (param_1 + -0xa))) {
        if ((c_var1 - 0x20) < 0x59) {
            b_var3 = ((c_var1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var3 = 0x0;
        }
        b_var3 = ((b_var3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        (param_1 + -0x7) = b_var3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var4 = ((b_var3 * 0x2 + 0x30a4))();
        return u_var4;
    }
    return (param_1 + -0xa);
}


// WARNING (jumptable): Unable to track spacebase fully for stack

pub fn pass1_1000_319c(param_1: i16, param_2: u16) -> u16

{
    let c_var1: u8;
    let mut pc_var2: String;
    let b_var3: u8;
    let u_var4: u16;

    c_var1 = *(param_1 + -0x4);
    if (c_var1 == '*') {
        u_var4 = pass1_1000_34cf(param_1, param_2);
        if (u_var4 < 0x0) {
            u_var4 = 0xffff;
        }
    } else {
        u_var4 = (param_1 + -0xe) * 0xa + (c_var1 - 0x30);
    }
    (param_1 + -0xe) = u_var4;
    pc_var2 = (param_1 + 0xa);
    c_var1 = *pc_var2;
    (param_1 + 0xa) = pc_var2 + 0x1;
    *(param_1 + -0x4) = c_var1;
    if ((c_var1 != '\0') && (-0x1 < (param_1 + -0xa))) {
        if ((c_var1 - 0x20) < 0x59) {
            b_var3 = ((c_var1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var3 = 0x0;
        }
        b_var3 = ((b_var3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        (param_1 + -0x7) = b_var3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var4 = ((b_var3 * 0x2 + 0x30a4))();
        return u_var4;
    }
    return (param_1 + -0xa);
}


// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub fn pass1_1000_31c5(param_1: i16, param_2: u16) -> u16

{
    let pb_var1: *mut u8;
    let c_var2: u8;
    let mut pc_var3: String;
    let b_var4: u8;
    let u_var5: u16;

    c_var2 = *(param_1 + -0x4);
    if (c_var2 == 'l') {
        pb_var1 = (param_1 + -0x6);
        *pb_var1 = *pb_var1 | 0x10;
    } else {
        if (c_var2 == 'F') {
            pb_var1 = (param_1 + -0x6);
            *pb_var1 = *pb_var1 | 0x20;
        } else {
            if (c_var2 == 'N') {
                pb_var1 = (param_1 + -0x5);
                *pb_var1 = *pb_var1 | 0x10;
            } else {
                if (c_var2 == 'L') {
                    pb_var1 = (param_1 + -0x5);
                    *pb_var1 = *pb_var1 | 0x4;
                } else {
                    pb_var1 = (param_1 + -0x5);
                    *pb_var1 = *pb_var1 | 0x8;
                }
            }
        }
    }
    pc_var3 = (param_1 + 0xa);
    c_var2 = *pc_var3;
    (param_1 + 0xa) = pc_var3 + 0x1;
    *(param_1 + -0x4) = c_var2;
    if ((c_var2 != '\0') && (-0x1 < (param_1 + -0xa))) {
        if ((c_var2 - 0x20) < 0x59) {
            b_var4 = ((c_var2 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var4 = 0x0;
        }
        b_var4 = ((b_var4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        (param_1 + -0x7) = b_var4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var5 = ((b_var4 * 0x2 + 0x30a4))();
        return u_var5;
    }
    return (param_1 + -0xa);
}


// WARNING (jumptable): Unable to track spacebase fully for stack

pub fn pass1_1000_31f7(param_1: u16, param_2: i16, param_3: *mut u16, param_4: i16, param_5: u16) -> u16

{
    let pi_var1: *mut i16;
    let pb_var2: *mut u8;
    let pu_var3: *mut u16;
    let c_var4: u8;
    let mut pc_var5: String;
    let b_var6: u8;
    let u_var7: u16;
    let i_var8: i16;
    let i_var9: i16;
    let i_var10: i16;
    let pu_var11: *mut u16;
    let u_var12: u16;
    let mut pc_var13: String;
    let mut pc_var14: String;
    let b_var15: bool;
    let u_var16: u32;

    c_var4 = *(param_2 + -0x4);
    if ((c_var4 == 'd') || (c_var4 == 'i')) {
        pb_var2 = (param_2 + -0x6);
        *pb_var2 = *pb_var2 | 0x40;
//LAB_1000_3399:
        *(param_2 + -0x8) = 0xa;
//LAB_1000_33d4:
        if (((param_2 + -0x6) & 0x10) == 0x0) {
            u_var7 = pass1_1000_34cf(param_2, param_5);
            if (((param_2 + -0x6) & 0x40) == 0x0) {
                u_var16 = u_var7;
            } else {
                u_var16 = SEXT24(u_var7);
            }
        } else {
            u_var16 = pass1_1000_34d8(param_2, param_5);
        }
        if ((((param_2 + -0x6) & 0x40) != 0x0) && (u_var16 < 0x0)) {
            pb_var2 = (param_2 + -0x5);
            *pb_var2 = *pb_var2 | 0x1;
            u_var16 = CONCAT22(-((u_var16 >> 0x10) + (u_var16 != 0x0)), -u_var16,
            );
        }
        if ((param_2 + -0xe) < 0x0) {
            (param_2 + -0xe) = 0x1;
        } else {
            pb_var2 = (param_2 + -0x6);
            *pb_var2 = *pb_var2 & 0xf7;
        }
        if (u_var16 == 0x0) {
            (param_2 + -0x12) = 0x0;
        }
        pu_var11 = (param_2 + -0x8);
        pass1_1000_356e(u_var16, pu_var11, (u_var16 >> 0x10), param_2,
                        (param_2 + -0xe), (param_2 + -0x17), param_5, param_5);
        if ((((param_2 + -0x5) & 0x2) != 0x0) && ((pu_var11 == 0x0 || ((param_2 + -0x17) != 0x30)))) {
            *(param_2 + -0x18) = 0x30;
            pu_var11 = (pu_var11 + 0x1);
        }
    } else {
        if (c_var4 == 'u') {
            // goto
            // LAB_1000_3399;
        }
        if (c_var4 == 'X') {
            *(param_2 + -0x3) = 0x7;
//LAB_1000_33a9:
            if (((param_2 + -0x6) & 0x80) != 0x0) {
                (param_2 + -0x12) = 0x2;
                *(param_2 + -0x10) = 0x30;
                *(param_2 + -0xf) = *(param_2 + -0x3) + 'Q';
            }
            *(param_2 + -0x8) = 0x10;
//       TODO: goto LAB_1000_33d4;
        }
        if (c_var4 == 'x') {
            *(param_2 + -0x3) = 0x27;
//       TODO: goto LAB_1000_33a9;
        }
        if (c_var4 == 'o') {
            if (((param_2 + -0x6) & 0x80) != 0x0) {
                pb_var2 = (param_2 + -0x5);
                *pb_var2 = *pb_var2 | 0x2;
            }
            *(param_2 + -0x8) = 0x8;
//       TODO: goto LAB_1000_33d4;
        }
        if (c_var4 == 'c') {
            u_var7 = pass1_1000_34cf(param_2, param_5);
            *(param_2 + -0x216) = u_var7;
            pu_var11 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
        } else {
            if (c_var4 == 's') {
                pass1_1000_34e6(param_1, param_2, param_5);
                if ((param_3 != 0x0) || (pu_var11 = ctx.DAT_1050_605d, param_4 != 0x0)) {
                    i_var10 = (param_2 + -0xe);
                    pu_var11 = param_3;
                    if (i_var10 != 0x0) {
                        b_var15 = true;
                        loop {
                            if (i_var10 == 0x0) { break; }
                            i_var10 += -0x1;
                            pu_var3 = pu_var11;
                            pu_var11 = (pu_var11 + 0x1);
                            b_var15 = *pu_var3 == '\0';
                            if b_var15 { break; }
                        }
                        if (b_var15) {
                            pu_var11 = (pu_var11 + -0x1);
                        }
                    }
                    pu_var11 = (pu_var11 - param_3);
                }
            } else {
                if (c_var4 == 'n') {
                    pass1_1000_34e6(param_1, param_2, param_5);
                    *param_3 = (param_2 + -0xa);
                    if (((param_2 + -0x6) & 0x10) != 0x0) {
                        param_3[0x1] = 0x0;
                    }
//           TODO: goto LAB_1000_30cf;
                }
                if (c_var4 == 'p') {
                    if (((param_2 + -0x6) & 0x30) == 0x0) {
                        u_var7 = pass1_1000_34cf(param_2, param_5);
                        u_var16 = u_var7;
                    } else {
                        u_var16 = pass1_1000_34d8(param_2, param_5);
                        // u_var12 = (u_var16 >> 0x10);
                        if (((param_2 + -0x5) & 0x18) == 0x0) {
                            *(param_2 + -0x3) = 0x7;
                            pass1_1000_356e(u_var16, 0x10, 0x0, param_2, 0x4, (param_2 + -0x20e), param_5, param_5);
                            pass1_1000_356e(u_var12, 0x10, 0x0, param_2, 0x4, (param_2 + -0x213),
                                            param_5, param_5);
                            *(param_2 + -0x212) = 0x3a;
                            pu_var11 = &DAT_1050_0009;
//               TODO: goto LAB_1000_3444;
                        }
                    }
                    *(param_2 + -0x3) = 0x7;
                    pass1_1000_356e(u_var16, 0x10, 0x0, param_2, 0x4, (param_2 + -0x213),
                                    param_5, param_5);
                    pu_var11 = &DAT_1050_0004;
                } else {
                    if ((c_var4 == 'E') || (c_var4 == 'G')) {
                        pi_var1 = (param_2 + -0x14);
                        *pi_var1 = *pi_var1 + 0x1;
                    }
                    pb_var2 = (param_2 + -0x6);
                    *pb_var2 = *pb_var2 | 0x40;
                    b_var6 = (param_2 + -0x4) | 0x20;
                    i_var10 = (param_2 + -0xe);
                    if (i_var10 < 0x1) {
                        if (i_var10 == 0x0) {
                            if (b_var6 == 0x67) {
                                (param_2 + -0xe) = 0x1;
                            }
                        } else {
                            (param_2 + -0xe) = 0x6;
                        }
                    }
                    pc_var13 = (param_2 + -0x216);
                    if (((param_2 + -0x5) & 0x4) == 0x0) {
                        (*PTR_s_3_wav_1050_25cc_1050_6068)();
                        pi_var1 = (param_2 + 0xe);
                        *pi_var1 = *pi_var1 + 0x8;
                    } else {
                        (*PTR_s_3_wav_1050_25cc_1050_607c)();
                        pi_var1 = (param_2 + 0xe);
                        *pi_var1 = *pi_var1 + 0xa;
                    }
                    if ((((param_2 + -0x6) & 0x80) != 0x0) && ((param_2 + -0xe) == 0x0)) {
                        (*PTR_s_3_wav_1050_25cc_1050_6074)();
                    }
                    if ((b_var6 == 0x67) && (((param_2 + -0x6) & 0x80) == 0x0)) {
                        (*PTR_s_3_wav_1050_25cc_1050_6070)();
                    }
                    if (*pc_var13 == '-') {
                        pc_var13 = (param_2 + -0x215);
                        pb_var2 = (param_2 + -0x5);
                        *pb_var2 = *pb_var2 | 0x1;
                    }
                    i_var10 = -0x1;
                    pc_var14 = pc_var13;
                    loop {
                        if (i_var10 == 0x0) { break; }
                        i_var10 += -0x1;
                        pc_var5 = pc_var14;
                        pc_var14 = pc_var14 + 0x1;
                        if *pc_var5 == '\0' { break; }
                    }
                    pu_var11 = (pc_var14 + (-0x1 - pc_var13));
                }
            }
        }
    }
//LAB_1000_3444:
    if (((param_2 + -0x6) & 0x40) != 0x0) {
        if (((param_2 + -0x5) & 0x1) == 0x0) {
            if (((param_2 + -0x6) & 0x1) == 0x0) {
                if (((param_2 + -0x6) & 0x2) != 0x0) {
                    *(param_2 + -0x10) = 0x20;
                    (param_2 + -0x12) = 0x1;
                }
            } else {
                *(param_2 + -0x10) = 0x2b;
                (param_2 + -0x12) = 0x1;
            }
        } else {
            *(param_2 + -0x10) = 0x2d;
            (param_2 + -0x12) = 0x1;
        }
    }
    i_var8 = (param_2 + -0xc) - pu_var11;
    i_var10 = (param_2 + -0x12);
    i_var9 = i_var8 - i_var10;
    if (i_var8 < i_var10) {
        i_var9 = 0x0;
    }
    if (((param_2 + -0x6) & 0xc) == 0x0) {
        pass1_1000_3552(i_var9, param_2, param_5);
    }
    pass1_1000_3534((param_2 + -0x12), param_2, param_5);
    if ((((param_2 + -0x6) & 0x8) != 0x0) && (((param_2 + -0x6) & 0x4) == 0x0)) {
        pass1_1000_3552(i_var9, param_2, param_5);
    }
    pass1_1000_3534(pu_var11, param_2, param_5);
    if (((param_2 + -0x6) & 0x4) != 0x0) {
        pass1_1000_3552(i_var9, param_2, param_5);
    }
//LAB_1000_30cf:
    pc_var5 = (param_2 + 0xa);
    c_var4 = *pc_var5;
    (param_2 + 0xa) = pc_var5 + 0x1;
    *(param_2 + -0x4) = c_var4;
    if ((c_var4 != '\0') && (-0x1 < (param_2 + -0xa))) {
        if ((c_var4 - 0x20) < 0x59) {
            b_var6 = ((c_var4 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var6 = 0x0;
        }
        b_var6 = ((b_var6 * '\b' + *(param_2 + -0x7)) + 0x5ffe) >> 0x4;
        (param_2 + -0x7) = b_var6;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var7 = ((b_var6 * 0x2 + 0x30a4))();
        return u_var7;
    }
    return (param_2 + -0xa);
}


pub fn pass1_1000_34cf(param_1: i16, param_2: u16) -> u16

{
    let u_var1: u16;
    let pu_var2: *mut u16;

    pu_var2 = (param_1 + 0xe);
    u_var1 = *pu_var2;
    (param_1 + 0xe) = pu_var2 + 0x2;
    return u_var1;
}


pub fn pass1_1000_34d8(param_1: i16, param_2: u16) -> u32

{
    let u_var1: u16;
    let u_var2: u16;
    let pu_var3: *mut u16;

    pu_var3 = (param_1 + 0xe);
    u_var1 = *pu_var3;
    u_var2 = (pu_var3 + 0x2);
    (param_1 + 0xe) = pu_var3 + 0x4;
    return CONCAT22(u_var2, u_var1);
}


pub fn pass1_1000_34e6(param_1: u16, param_2: i16, param_3: u16) -> u32

{
    let u_var1: u16;
    let u_var2: u32;

    if (((param_2 + -0x6) & 0x20) != 0x0) {
        u_var2 = pass1_1000_34d8(param_2, param_3);
        return u_var2;
    }
    u_var1 = pass1_1000_34cf(param_2, param_3);
    if (u_var1 == 0x0) {
        return param_1 << 0x10;
    }
    return CONCAT22(param_1, u_var1);
}


pub fn pass1_1000_3503(param_1: u8, param_2: u16, param_3: i16, param_4: u16, param_5: u16,
                       param_6: u8) -> i16

{
    let pi_var1: *mut i16;
    let mut pc_var2: String;
    char * *ppcVar3;
    let u_var4: u16;
    let pi_var5: *mut i16;
    let u_var6: u16;

    ppcVar3 = (param_3 + 0x6);
// u_var6 = (ppcVar3 >> 0x10);
    pi_var5 = ppcVar3;
    pi_var1 = pi_var5 + 0x2;
    *pi_var1 = *pi_var1 + -0x1;
    if (*pi_var1 < 0x0) {
        u_var4 = mem_1000_2bb6(param_1, pi_var5, param_3, u_var6, param_4, param_5, param_6,
                               param_2);
        if (u_var4 == 0xffff) {
            return -0x1;
        }
    } else {
        pc_var2 = *ppcVar3;
        *ppcVar3 = *ppcVar3 + 0x1;
        *pc_var2 = param_1;
    }
    return 0x0;
}


pub fn pass1_1000_3534(param_1: i16, param_2: i16, param_3: u16) {
    let pi_var1: *mut i16;
    let pu_var2: *mut u8;
    let u_var3: u16;
    let in_dx: u16;
    let unaff_di: *mut u8;
    let u_var4: u16;
    let unaff_es: u16;
    let unaff_cs: u16;
    let in_af: u8;

    if (param_1 != 0x0) {
        pi_var1 = (param_2 + -0xa);
        *pi_var1 = *pi_var1 + param_1;
        u_var4 = 0x0;
        loop {
            pu_var2 = unaff_di;
            unaff_di = unaff_di + 0x1;
            u_var3 = pass1_1000_3503(*pu_var2, in_dx, param_2, unaff_cs, param_3, in_af);
            u_var4 |= u_var3;
            param_1 += -0x1;
            if param_1 == 0 { break; }
        }
        if (u_var4 != 0x0) {
            (param_2 + -0xa) = 0xffff;
        }
    }
    return;
}


pub fn pass1_1000_3552(param_1: i16, param_2: i16, param_3: u16) {
    let pi_var1: *mut i16;
    let u_var2: u16;
    let in_dx: u16;
    let u_var3: u16;
    let unaff_cs: u16;
    let in_af: u8;

    if (param_1 != 0x0) {
        pi_var1 = (param_2 + -0xa);
        *pi_var1 = *pi_var1 + param_1;
        u_var3 = 0x0;
        loop {
            u_var2 = pass1_1000_3503(in_dx, in_dx, param_2, unaff_cs, param_3, in_af);
            u_var3 |= u_var2;
            param_1 += -0x1;
            if pram_1 == 0 { break; }
        }
        if (u_var3 != 0x0) {
            (param_2 + -0xa) = 0xffff;
        }
    }
    return;
}


pub fn pass1_1000_356e(param_1: u16, param_2: u16, param_3: u16, param_4: i16, param_5: i16,
                       param_6: *mut u8, param_7: u16, param_8: u16)

{
    let pb_var1: *mut u8;
    let u_var2: u32;
    let b_var3: u8;

    while (((0x0 < param_5 || (param_1 != 0x0)) || (param_3 != 0x0))) {
        u_var2 = param_3;
        param_3 /= param_2;
        u_var2 = u_var2 % param_2 << 0x10 | param_1;
        param_1 = (u_var2 / param_2);
        b_var3 = (u_var2 % param_2) + 0x30;
        if (0x39 < b_var3) {
            b_var3 += *(param_4 + -0x3);
        }
        pb_var1 = param_6;
        param_6 = param_6 + -0x1;
        *pb_var1 = b_var3;
        param_5 += -0x1;
    }
    return;
}


pub fn pass1_1000_35aa() -> u16

{
    let pu_var1: *mut u16;

    pu_var1 = &ctx.PTR_LOOP_1050_6210;
    loop {
        if (ctx.PTR_LOOP_1050_5ff0 < pu_var1) {
            return 0x0;
        }
        if (((pu_var1 + 0x5) & 0x83) == 0x0) { break; }
        pu_var1 = pu_var1 + 0x6;
    }
    *(pu_var1 + 0x5) = 0x0;
    pu_var1[0x2] = 0x0;
    pu_var1[0x4] = 0x0;
    pu_var1[0x3] = 0x0;
    pu_var1[0x1] = 0x0;
    *pu_var1 = 0x0;
    *(pu_var1 + 0xb) = 0xff;
    return pu_var1;
}


pub fn pass1_1000_39e1() {
    return;
}


pub fn pass1_1000_3bac() -> i16

{
    let i_var1: i16;

    if (ctx.PTR_LOOP_1050_5f48 < &stack0x0004) {
        i_var1 = -(ctx.PTR_LOOP_1050_5f48 + -&stack0x0004);
    } else {
        i_var1 = 0x0;
    }
    return i_var1;
}


pub fn pass1_1000_3bc0(param_1: i16, param_2: i16, param_3: *mut u16, param_4: u16, param_5: u16,
                       param_6: u16)

{
    let pi_var1: *mut i16;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let i_var5: i16;
    let pu_var6: *mut u16;
    let b_var7: bool;
    let u_var8: u32;

    if (((param_2 + 0x2) & 0x1) != 0x0) {
        pass1_1000_3cb7(param_2);
        u_var4 = *param_3;
        if ((u_var4 & 0x1) != 0x0) {
            param_1 = (param_1 - u_var4) + -0x1;
        }
        u_var4 = (param_2 + 0x4);
        if (u_var4 != 0x0) {
            u_var3 = param_1 + 0x2 + u_var4;
            if (!CARRY2(param_1 + 0x2, u_var4)) {
                param_4 = pass1_1000_29dc(param_6);
                u_var4 = &ctx.PTR_LOOP_1050_6066;
                if (u_var4 == 0x1000) {
                    // goto
                    // LAB_1000_3c12;
                }
                u_var2 = 0x8000;
                while (u_var4 <= u_var2) {
                    u_var2 >>= 0x1;
                    if (u_var2 == 0x0) {
                        // goto
                        // LAB_1000_3c2b;
                    }
                }
                if (u_var2 < 0x8) {
                    // goto
                    // LAB_1000_3c2b;
                }
                u_var4 = u_var2 << 0x1;
//         TODO: goto LAB_1000_3c12;
            }
            u_var2 = 0x0;
            u_var4 = 0xfff0;
            if (u_var3 == 0x0) {
                loop {
                    b_var7 = false;
                    u_var8 = mixed_mem_op_1000_3c51(u_var2, u_var3, param_2, param_4, param_5, 0x3c23);
                    if (!b_var7) { break; }
                    if (u_var4 == 0xfff0) {
                        return;
                    }
//LAB_1000_3c2b:
                    u_var4 = 0x10;
//LAB_1000_3c12:
                    u_var4 -= 0x1;
                    u_var2 = u_var4 + u_var3;
                    if (CARRY2(u_var4, u_var3)) {
                        u_var2 = 0x0;
                    }
                    u_var4 = ~u_var4;
                    u_var2 &= u_var4;
                }
                i_var5 = u_var8 - (param_2 + 0x4);
                (param_2 + 0x4) = u_var8;
                (param_2 + 0xa) = param_3;
                pi_var1 = (param_2 + 0xc);
                *pi_var1 = i_var5 + -0x1;
                pu_var6 = (pi_var1 + i_var5);
                *pu_var6 = 0xfffe;
                (param_2 + 0xc) = pu_var6;
            }
        }
    }
    return;
}


pub fn pass1_1000_3cb7(param_1: i16) {
    let u_var1: u16;
    let pu_var2: *mut u16;

    pu_var2 = (param_1 + 0xa);
    if (pu_var2 == (param_1 + 0xc)) {
        pu_var2 = (param_1 + 0x8);
    }
    loop {
        u_var1 = *pu_var2;
        if (u_var1 == 0xfffe) { break; }
        pu_var2 = (pu_var2 + (u_var1 & 0xfffe) + 0x2);
    }
    return;
}


pub fn pass1_1000_3cd8(param_1: u16, param_2: u16) {
    free_mem_1000_407a(param_1, param_2, &stack0xfffe);
    return;
}


pub fn pass1_1000_3cea(param_1: i32, param_2: i32) -> u16

{
    let p_uvar1: *mut u16;
    let mut pc_var2: String;
    let p_uvar3: *mut u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let p_uvar7: *mut u16;
    let mut pc_var8: String;
    let p_uvar9: *mut u16;
    let p_uvar10: *mut u16;
    let u_var11: u16;
    let u_var12: u16;
    let b_var13: bool;

    // u_var11 = (param_1 >> 0x10);
    b_var13 = true;
    i_var4 = -0x1;
    p_uvar7 = param_1;
    loop {
        if (i_var4 == 0x0) { break; }
        i_var4 += -0x1;
        p_uvar1 = p_uvar7;
        p_uvar7 = (p_uvar7 + 0x1);
        b_var13 = *p_uvar1 == '\0';
        if b_var13 { break; }
    }
    p_uvar10 = (p_uvar7 + -0x1);
    // u_var12 = (param_2 >> 0x10);
    pc_var8 = param_2;
    u_var5 = 0xffff;
    loop {
        if (u_var5 == 0x0) { break; }
        u_var5 -= 0x1;
        pc_var2 = pc_var8;
        pc_var8 = pc_var8 + 0x1;
        b_var13 = *pc_var2 == '\0';
        if b_var13 { break; }
    }
    u_var5 = ~u_var5;
    if (!b_var13) {
        pc_var8 = pc_var8 + -u_var5;
        u_var5 += 0x1;
    }
    p_uvar9 = (pc_var8 + -u_var5);
    if (u_var5 == 0x0) {
        p_uvar1 = p_uvar9;
        p_uvar9 = p_uvar9 + 0x1;
        *p_uvar10 = *p_uvar1;
        u_var5 = 0xfffe;
        p_uvar10 = (p_uvar7 + 0x1);
    } else {
        if ((p_uvar9 & 0x1) != 0x0) {
            p_uvar1 = p_uvar9;
            p_uvar9 = (p_uvar9 + 0x1);
            *p_uvar10 = *p_uvar1;
            u_var5 -= 0x1;
            p_uvar10 = p_uvar7;
        }
    }
    // TODO: refactor for loop
    // for (u_var6 = u_var5 >> 0x1; u_var6 != 0x0; u_var6 -= 0x1) {
    //   p_uvar3 = p_uvar10;
    //   p_uvar10 = p_uvar10 + 0x1;
    //   p_uvar1 = p_uvar9;
    //   p_uvar9 = p_uvar9 + 0x1;
    //   *p_uvar3 = *p_uvar1;
    // }
    // TODO: refactor for loop
    // for (u_var5 = ((u_var5 & 0x1) != 0x0); u_var5 != 0x0; u_var5 -= 0x1) {
    //   p_uvar3 = p_uvar10;
    //   p_uvar10 = (p_uvar10 + 0x1);
    //   p_uvar1 = p_uvar9;
    //   p_uvar9 = (p_uvar9 + 0x1);
    //   *p_uvar3 = *p_uvar1;
    // }
    return param_1;
}


pub fn pass1_1000_3d7a(param_1: u32, param_2: u32) -> i16

{
    let pb_var1: *mut u8;
    let mut pc_var2: String;
    let pb_var3: *mut u8;
    let i_var4: i16;
    let u_var5: u16;
    let mut pc_var6: String;
    let pb_var7: *mut u8;
    let mut pc_var8: String;
    let pb_var9: *mut u8;
    let u_var10: u16;
    let b_var11: bool;
    let b_var12: bool;

    pb_var7 = param_1;
// u_var10 = (param_2 >> 0x10);
    pc_var8 = param_2;
    i_var4 = 0x0;
    u_var5 = 0xffff;
    loop {
        if (u_var5 == 0x0) { break; }
        u_var5 -= 0x1;
        pc_var2 = pc_var8;
        pc_var8 = pc_var8 + 0x1;
        if *pc_var2 == '\0' { break; }
    }
    pc_var6 = ~u_var5;
    b_var11 = pc_var8 < pc_var6;
    pb_var9 = (pc_var8 + -pc_var6);
    b_var12 = pb_var9 == 0x0;
    loop {
        if (pc_var6 == 0x0) { break; }
        pc_var6 = pc_var6 + -0x1;
        pb_var3 = pb_var9;
        pb_var9 = pb_var9 + 0x1;
        pb_var1 = pb_var7;
        pb_var7 = pb_var7 + 0x1;
        b_var11 = *pb_var1 < *pb_var3;
        b_var12 = *pb_var1 == *pb_var3;
        if !b_var12 { breka; }
    }
    if (!b_var12) {
        i_var4 = (0x1 - b_var11) - (b_var11 != 0x0);
    }
    return i_var4;
}


pub fn pass1_1000_3de8(param_1: &mut String, param_2: &mut String, param_3: u16, param_4: u16, param_5: u16,
) -> u16

{
    let pb_var1: *mut u8;
    let mut pc_var2: String;
    let mut pc_var3: String;
    let b_var4: u8;
    let u_var5: u16;
    let i_var6: i16;
    let mut pc_var7: String;
    let mut pc_var8: String;
    let u_var9: u16;
    let u_var10: u16;
    let b_var11: bool;

    if (param_3 != 0x0) {
        // u_var9 = (param_1 >> 0x10);
        pc_var8 = param_1;
        u_var5 = param_3;
        pc_var7 = pc_var8;
        loop {
            if (u_var5 == 0x0) { break; }
            u_var5 -= 0x1;
            pc_var2 = pc_var7;
            pc_var7 = pc_var7 + 0x1;
            if *pc_var2 == '\0' { break; }
        }
        i_var6 = param_3 - u_var5;
        // u_var10 = (param_2 >> 0x10);
        pc_var7 = param_2;
        loop {
            if (i_var6 == 0x0) { break; }
            i_var6 += -0x1;
            pc_var3 = pc_var8;
            pc_var8 = pc_var8 + 0x1;
            pc_var2 = pc_var7;
            pc_var7 = pc_var7 + 0x1;
            if *pc_var2 != *pc_var3 { break; }
        }
        b_var4 = pc_var7[-0x1];
        u_var5 = 0x0;
        pb_var1 = (pc_var8 + -0x1);
        b_var11 = b_var4 == *pb_var1;
        if (b_var4 < *pb_var1 || b_var11) {
            if (b_var11) {
                return 0x0;
            }
            u_var5 = 0xfffe;
        }
        param_3 = ~u_var5;
    }
    return param_3;
}


pub fn pass1_1000_3e2c(param_1: u32) -> i16

{
    let pb_var1: *mut u8;
    let b_var2: u8;
    let b_var3: u8;
    let i_var4: i16;
    let pb_var5: *mut u8;
    let u_var6: u16;

    // u_var6 = (param_1 >> 0x10);
    pb_var5 = param_1;
    i_var4 = 0x0;
    loop {
        loop {
            pb_var1 = pb_var5;
            pb_var5 = pb_var5 + 0x1;
            b_var2 = *pb_var1;
            if b_var2 != 0x20 {
                break;
            }
        }
        if b_var2 != 0x9 {
            break;
        }
    }
    if ((b_var2 != 0x2d) && (b_var3 = b_var2, b_var2 != 0x2b)) {
        // goto LAB_1000_3e4d;
    }
    loop {
        pb_var1 = pb_var5;
        pb_var5 = pb_var5 + 0x1;
        b_var3 = *pb_var1;
//LAB_1000_3e4d:
        if ((0x39 < b_var3) || (b_var3 < 0x30)) { break; }
        i_var4 = i_var4 * 0xa + (b_var3 - 0x30);
    }
    if (b_var2 == 0x2d) {
        i_var4 = -i_var4;
    }
    return i_var4;
}


pub fn pass1_1000_3e2c(param_1: u32) -> i16

{
    let pb_var1: *mut u8;
    let b_var2: u8;
    let b_var3: u8;
    let i_var4: i16;
    let pb_var5: *mut u8;
    let u_var6: u16;

    // u_var6 = (param_1 >> 0x10);
    pb_var5 = param_1;
    i_var4 = 0x0;
    loop {
        loop {
            pb_var1 = pb_var5;
            pb_var5 = pb_var5 + 0x1;
            b_var2 = *pb_var1;
            if b_var2 != 20 {
                break;
            }
        }
        if b_var2 != 0x9 {
            break;
        }
    }
    if ((b_var2 != 0x2d) && (b_var3 = b_var2, b_var2 != 0x2b)) {
        // goto LAB_1000_3e4d;
    }
    loop {
        pb_var1 = pb_var5;
        pb_var5 = pb_var5 + 0x1;
        b_var3 = *pb_var1;
//LAB_1000_3e4d:
        if ((0x39 < b_var3) || (b_var3 < 0x30)) { break; }
        i_var4 = i_var4 * 0xa + (b_var3 - 0x30);
    }
    if (b_var2 == 0x2d) {
        i_var4 = -i_var4;
    }
    return i_var4;
}


pub fn pass1_1000_3e82(param_1: u16, param_2: *mut u8, param_3: u16) -> *mut u8

{
    let pb_var1: *mut u8;
    let u_var2: u32;
    let b_var3: u8;
    let u_var5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let pb_var8: *mut u8;
    let pb_var9: *mut u8;
    let pb_var10: *mut u8;
    let pb_var11: *mut u8;
    let u_var12: u16;
    let b_var13: bool;
    let c_var4: u8;

    u_var6 = 0x0;
    if (param_3 == 0xa) {
        u_var6 = param_1 > > 0xf;
    }
// u_var12 = (param_2 >> 0x10);
    pb_var9 = param_2;
    pb_var10 = pb_var9;
    pb_var8 = pb_var9;
    if (((true) & &(param_3 == 0xa)) & &(u_var6 < 0x0)) {
        pb_var10 = pb_var9 + 0x1;
        *param_2 = '-';
        b_var13 = param_1 != 0x0;
        param_1 = -param_1;
        u_var6 = -(u_var6 + b_var13);
        pb_var8 = pb_var10;
    }
    loop {
        u_var7 = 0x0;
        u_var5 = u_var6;
        if (u_var6 != 0x0) {
            u_var5 = u_var6 / param_3;
            u_var7 = u_var6 % param_3;
        }
        u_var2 = CONCAT22(u_var7, param_1);
        param_1 = (u_var2 / param_3);
        c_var4 = (u_var2 % param_3);
        b_var3 = c_var4 + 0x30;
        if (0x39 < b_var3) {
            b_var3 = c_var4 + 0x57;
        }
        pb_var11 = pb_var10 + 0x1;
        *pb_var10 = b_var3;
        u_var6 = u_var5;
        pb_var10 = pb_var11;
        if (u_var5 | param_1) == 0 {
            break;
        }
    }
    *pb_var11 = 0x0;
    loop {
        pb_var11 = pb_var11 + -0x1;
        pb_var1 = pb_var11;
        b_var3 = *pb_var1;
        *pb_var1 = *pb_var8;
        *pb_var8 = b_var3;
        pb_var10 = pb_var8 + 0x2;
        pb_var8 = pb_var8 + 0x1;
        if pb_var10 > = pb_var11 {
            break;
        }
    }
    return pb_var9;
}


pub fn pass1_1000_3ec0(param_1: u16, param_2: u16) -> i16

{
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let unaff_si: u16;
    let u_var4: u16;
    let pu_var4: u32;

    pu_var4 = CONCAT22(ctx.PTR_LOOP_1050_5fc0, PTR_LOOP_1050_5fbe);
    if (((ctx.PTR_LOOP_1050_5fc0 | ctx.PTR_LOOP_1050_5fbe) != 0x0) && ((param_2 | param_1) != 0x0)) {
        u_var1 = str_op_1000_3da4(CONCAT22(param_2, param_1));
        loop {
            // u_var4 = (pu_var4 >> 0x10);
            u_var3 = pu_var4;
            if (((u_var3 + 0x2) | pu_var4) == 0x0) { break; }
            u_var2 = str_op_1000_3da4(CONCAT22((u_var3 + 0x2),
                                               pu_var4));
            if (((u_var1 < u_var2) && (*(*pu_var4 + u_var1) == '=')) && (u_var2 = pass1_1000_3de8(CONCAT22((u_var3 + 0x2),
                                                                                                           pu_var4),
                                                                                                  CONCAT22(param_2, param_1), u_var1, unaff_si, u_var3),
                                                                         u_var2 == 0x0)) {
                return pu_var4 + u_var1 + 0x1;
            }
            pu_var4 = (pu_var4 & 0xffff0000 | (u_var3 + 0x4));
        }
    }
    return 0x0;
}


pub fn pass1_1000_3f5c(param_1: i16, param_2: u16, param_3: u16, param_4: u16, param_5: u8) -> i16

{
    let u_var1: u16;
    let pu_var2: *mut u16;
    let i_var3: i16;
    let i_stack2: i16;

    i_stack2 = param_1 + 0x1;
    i_var3 = 0x0;
    if (ctx.PTR_LOOP_1050_61ec == 0x0) {
        pu_var2 = &ctx.PTR_LOOP_1050_6210;
    } else {
        pu_var2 = 0x6234;
    }
    // TODO: refactor for loop
    // for (; pu_var2 <= ctx.PTR_LOOP_1050_5ff0; pu_var2 = pu_var2 + 0x6) {
    //   u_var1 = pass1_1000_2a00(pu_var2,&i_stack2,param_2,param_3,param_4,param_5);
    //   if (u_var1 != 0xffff) {
    //     i_var3 += 0x1;
    //   }
    // }
    return i_var3;
}


pub fn pass1_1000_400a(param_1: i16, param_2: u16) -> *mut u8

{
    let pu_var1: *mut u8;
    let i_stack2: i16;

    i_stack2 = param_2 + 0x1;
    if (param_1 < 0x0) || (ctx.PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1) {
        ctx.PTR_LOOP_1050_5f78 = &DAT_1050_0009;
        pu_var1 = 0xffff;
    } else {
        if (((ctx.PTR_LOOP_1050_61ec == 0x0) || ((param_1 < ctx.DAT_1050_5f8a && (0x2 < param_1)))) && (0x31d < CONCAT11(ctx.DAT_1050_5f83, DAT_1050_5f82))) {
            pu_var1 = ctx.PTR_LOOP_1050_5f88;
            if ((((param_1 + 0x5f90) & 0x1) == 0x0) || (pu_var1 = dos3_call_1000_5174(&i_stack2), pu_var1 != 0x0)
            ) {
                ctx.PTR_LOOP_1050_5f88 = pu_var1;
                ctx.PTR_LOOP_1050_5f78 = &DAT_1050_0009;
                pu_var1 = 0xffff;
            }
        } else {
            pu_var1 = 0x0;
        }
    }
    return pu_var1;
}


pub fn pass1_1000_41e0(param_1: i16) -> u16

{
    let pi_stack6: *mut i16;

    pi_stack6 = CONCAT22(ctx.PTR_LOOP_1050_6192, PTR_LOOP_1050_6190);
    loop {
        if (ctx.PTR_LOOP_1050_6190 + (ctx.PTR_LOOP_1050_6194 & 0xfffc) <= pi_stack6) {
            return 0x0;
        }
        if (*pi_stack6 == param_1) { break; }
        pi_stack6 = (pi_stack6 & 0xffff0000 | ZEXT24(pi_stack6 + 0x4));
    }
    *pi_stack6 = 0x0;
    return (pi_stack6 + 0x2);
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1000_422a(param_1: i16, param_2: u16, param_3: u16, param_4: u16) -> i16

{
    let pu_var1: *mut u8;
    let pu_var2: *mut u8;
    let pu_var3: *mut u8;
    let pu_var4: *mut u8;
    let pi_stack6: *mut i16;

    pi_stack6 = CONCAT22(ctx.PTR_LOOP_1050_6192, PTR_LOOP_1050_6190);
    loop {
        if (ctx.PTR_LOOP_1050_6190 + (ctx.PTR_LOOP_1050_6194 & 0xfffc) <= pi_stack6) {
            pu_var2 = ctx.PTR_LOOP_1050_6194 + 0x28;
            pu_var4 = ctx.PTR_LOOP_1050_6192;
            pu_var3 = pass1_1000_16aa(PTR_LOOP_1050_6190, PTR_LOOP_1050_6192,
                                      pu_var2, PTR_LOOP_1050_6192, param_3, param_4);
            if ((pu_var4 | pu_var3) == 0x0) {
                param_1 = 0x0;
            } else {
                pu_var1 = pu_var3 + (ctx.PTR_LOOP_1050_6194 & 0xfffc);
                pi_stack6 = CONCAT22(pu_var4, pu_var1);
                ctx.PTR_LOOP_1050_6190 = pu_var3;
                ctx.PTR_LOOP_1050_6192 = pu_var4;
                *pi_stack6 = param_1;
                (pu_var1 + 0x2) = param_2;
                ctx.PTR_LOOP_1050_6194 = pu_var2;
                pass1_1000_4906(CONCAT22(pu_var4, pu_var1 + 0x4), 0x0, 0x24,
                );
            }
            return param_1;
        }
        if (*pi_stack6 == 0x0) { break; }
        pi_stack6 = (pi_stack6 & 0xffff0000 | ZEXT24(pi_stack6 + 0x4));
    }
    (pi_stack6 + 0x2) = param_2;
    *pi_stack6 = param_1;
    return param_1;
}


pub fn pass1_1000_43f0(param_1: u16, param_2: u16) {
    if (ctx.PTR_LOOP_1050_68b4 == 0x0) {
        pass1_1000_440c(param_2);
        ctx.PTR_LOOP_1050_68b4 = ctx.PTR_LOOP_1050_68b4 + 0x1;
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1000_440c(param_1: u16) {
    let c_var1: u8;
    let mut pc_var2: String;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let l_var6: i32;
    let u_var7: u16;
    let u_var8: u16;
    let mut pc_stack8: String;

    u_var3 = pass1_1000_3ec0(0x61ca, ctx.data_seg);
    pc_stack8 = CONCAT22(param_1, u_var3);
    if (((param_1 | u_var3) != 0x0) && (_DAT_1050_61ce = CONCAT22(ctx.PTR_LOOP_1050_61d0, DAT_1050_61ce), *pc_stack8 != '\0')) {
        str_op_1000_3dbe(CONCAT13((ctx.PTR_USHORT_1050_1050_1050_61de >> 0x8),
                                  CONCAT12(ctx.PTR_USHORT_1050_1050_1050_61de,
                                           ctx.PTR_PTR_DAT_1050_5350_1050_61d4_1050_61dc)), CONCAT22(param_1, u_var3), 0x3);
        pc_stack8 = CONCAT22(param_1, u_var3 + 0x3);
        c_var1 = *pc_stack8;
        if (c_var1 == '-') {
            pc_stack8 = CONCAT22(param_1, u_var3 + 0x4);
        }
        u_var5 = 0x0;
        u_var8 = 0x0;
        u_var7 = 0xe10;
        i_var4 = pass1_1000_3e2c(pc_stack8 & 0xffff | param_1 << 0x10);
        _DAT_1050_61ce = pass1_1000_52be(i_var4, u_var5, u_var7, u_var8);
        // for (; (pc_var2 = pc_stack8, *pc_stack8 == '+' ||
        //        (('/' < *pc_stack8 && (*pc_stack8 < ':'))));
        //     pc_stack8 = (pc_stack8 & 0xffff0000 | (pc_stack8 + 0x1)))
        // {
    }
    if (*pc_stack8 == ':') {
        u_var5 = 0x0;
        u_var8 = 0x0;
        u_var7 = 0x3c;
        pc_stack8 = (pc_stack8 & 0xffff0000 | (pc_stack8 + 0x1));
        i_var4 = pass1_1000_3e2c(pc_var2 & 0xffff0000 | (pc_stack8 + 0x1));
        l_var6 = pass1_1000_52be(i_var4, u_var5, u_var7, u_var8);
        // u_var5 = (l_var6 >> 0x10);
        _DAT_1050_61ce += l_var6;
        // TODO: refactor for loop
        // for (; (pc_var2 = pc_stack8, '/' < *pc_stack8 && (*pc_stack8 < ':'));
        //     pc_stack8 = (pc_stack8 & 0xffff0000 | (pc_stack8 + 0x1))
        //     ) {
        // }
        if (*pc_stack8 == ':') {
            pc_stack8 = (pc_stack8 & 0xffff0000 | (pc_stack8 + 0x1));
            i_var4 = pass1_1000_3e2c(pc_var2 & 0xffff0000 | (pc_stack8 + 0x1));
            _DAT_1050_61ce += CONCAT22(u_var5, i_var4);
            // TODO: refactor for loop
            // for (; ('/' < *pc_stack8 && (*pc_stack8 < ':'));
            //     pc_stack8 = (pc_stack8 & 0xffff0000 |
            //                        (pc_stack8 + 0x1))) {
            // }
        }
    }
    ctx.PTR_LOOP_1050_61d0 = (_DAT_1050_61ce >> 0x10);
    if (c_var1 == '-') {
        _DAT_1050_61ce = CONCAT22(-(ctx.PTR_LOOP_1050_61d0 + (ctx.DAT_1050_61ce != 0x0)), -DAT_1050_61ce);
    }
    ctx.DAT_1050_61d2 = *pc_stack8;
    if (ctx.DAT_1050_61d2 == 0x0) {
        *_PTR_PTR_1050_61e0 = '\0';
    } else {
        str_op_1000_3dbe(ctx.PTR__PTR_1050_61e0, pc_stack8, 0x3);
    }
}
ctx.PTR_LOOP_1050_61d0 = (_DAT_1050_61ce > > 0x10); return; }



pub fn pass1_1000_455a(param_1: u32, param_2: u16) -> u16

{
    let pi_var1: *mut i16;
    let i_var2: i16;
    let u_var3: u16;
    let i_var4: i16;
    let uvar5: u16;
    let u_var6: u32;
    let i_stack6: i16;

    if ((((param_1 + 0xa) < 0x43) || ((param_1 + 0x8) < 0x3)) || (0x9 < (param_1 + 0x8))) {
        // goto
        // LAB_1000_4623;
    }
    if (((param_1 + 0x8) < 0x4) || (0x8 < (param_1 + 0x8))) {
        u_var3 = (param_1 + 0xa);
        if ((u_var3 < 0x57) || ((param_1 + 0x8) != 0x3)) {
            i_stack6 = ((param_1 + 0x8) * 0x2 + 0x61b2);
        } else {
            i_stack6 = ((param_1 + 0x8) * 0x2 + 0x61b0) + 0x7;
        }
        if ((u_var3 & 0x3) == 0x0) {
            i_stack6 += 0x1;
        }
        u_var3 = (u_var3 - 0x46) * 0x16d + ((u_var3 - 0x1) >> 0x2) + i_stack6;
        u_var6 = pass1_1000_52f0(u_var3 - 0xd, (u_var3 >> 0xf) - (u_var3 < 0xd), 0x7, 0x0);
        i_stack6 = u_var6 - i_stack6;
        i_var4 = -i_stack6;
        if ((param_1 + 0x8) == 0x3) {
            i_var2 = (param_1 + 0xe);
            if ((i_var4 < i_var2) || ((-i_var2 == i_stack6 && (0x1 < (param_1 + 0x4)))))
//       TODO: goto LAB_1000_460e;
        } else {
            pi_var1 = (param_1 + 0xe);
            i_var2 = *pi_var1;
            if ((SBORROW2(*pi_var1, i_var4) != i_var2 + i_stack6 < 0x0) || ((i_var2 == i_var4 && ((param_1 + 0x4) < 0x1)))) {
                // goto
                // LAB_1000_460e;
            }
        }
//LAB_1000_4623:
        uvar5 = 0x0;
    } else {
//LAB_1000_460e:
        uvar5 = 0x1;
    }
    return uvar5;
}


pub fn pass1_1000_462e(param_1: u16, param_2: i16, param_3: u16, param_4: u16, param_5: u16,
                       param_6: i16, param_7: i16, param_8: &mut String, param_9: u16) -> i16

{
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: i16;
    let uvar5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let u_var8: u16;
    let u_var9: u32;
    let i_stack26: i16;
    let local_16: [u8; 4];
    let u_stack18: u16;
    let i_stack14: i16;
    let i_stack12: i16;
    let i_stack8: i16;
    let local_4: u16;
    let i_stack2: i16;
    let u_var10: u16;
    let u_var11: u16;
    let u_var12: u16;
    let u_var13: u16;

    i_stack2 = param_7 + 0x1;
    local_4 = ctx.data_seg;
    u_var8 = (param_2 * 0x2 + 0x61ae);
    if (((param_1 & 0x3) == 0x0) && (0x2 < param_2)) {
        u_var8 += 0x1;
    }
    pass1_1000_43f0(&i_stack2, param_9);
    u_var13 = 0x0;
    u_var12 = 0x3c;
    u_var11 = 0x0;
    u_var10 = 0x3c;
    u_var1 = (param_1 * 0x16d);
    u_var2 = (param_1 + 0x3) >> 0x2;
    u_var3 = u_var2 + param_3;
    u_var6 = u_var1 + u_var3;
    u_var7 = u_var6 + u_var8;
    u_var9 = pass1_1000_52be(u_var7 + 0xe44,
                             ((param_1 * 0x16d) >> 0x10) + ((param_1 + 0x3) >> 0xf) + (param_3 >> 0xf) + CARRY2(u_var2, param_3) + CARRY2(u_var1, u_var3) + (u_var8 >> 0xf) + CARRY2(u_var6, u_var8) + (0xf1bb < u_var7), 0x18, 0x0);
    u_var9 = pass1_1000_52be(u_var9 + param_4,
                             (u_var9 >> 0x10) + (param_4 >> 0xf) + CARRY2(u_var9, param_4), u_var10, u_var11);
    i_var4 = pass1_1000_52be(u_var9 + param_5,
                             (u_var9 >> 0x10) + (param_5 >> 0xf) + CARRY2(u_var9, param_5), u_var12, u_var13);
    i_stack26 = i_var4 + param_6 + ctx.DAT_1050_61ce;
    i_stack8 = param_3 + u_var8;
    i_stack12 = param_1 + 0x50;
    i_stack14 = param_2 + -0x1;
    u_stack18 = param_4;
    if (ctx.DAT_1050_61d2 != 0x0) {
        uvar5 = pass1_1000_455a(local_16, param_8);
        if (uvar5 != 0x0) {
            i_stack26 += -0xe10;
        }
    }
    return i_stack26;
}


pub fn pass1_1000_472c(param_1: u32, param_2: u8) -> *mut u8

{
    let mut pc_var1: String;
    let u_var2: u16;
    let mut pc_var3: String;
    let mut pc_var4: String;
    let u_var5: u16;
    let b_var6: bool;

    // u_var5 = (param_1 >> 0x10);
    pc_var3 = param_1;
    b_var6 = true;
    u_var2 = 0xffff;
    pc_var4 = pc_var3;
    loop {
        if (u_var2 == 0x0) { break; }
        u_var2 -= 0x1;
        pc_var1 = pc_var4;
        pc_var4 = pc_var4 + 0x1;
        b_var6 = *pc_var1 == '\0';
        if b_var6 { break; }
    }
    u_var2 = !u_var2;
    loop {
        if (u_var2 == 0x0) { break; }
        u_var2 -= 0x1;
        pc_var1 = pc_var3;
        pc_var3 = pc_var3 + 0x1;
        b_var6 = param_2 == *pc_var1;
        if b_var6 { break; }
    }
    if (!b_var6) {
        if (param_2 != '\0') {
            return 0x0;
        }
        pc_var3 = pc_var3 + 0x1;
    }
    return pc_var3 + -0x1;
}


pub fn pass1_1000_475e(param_1: u32, param_2: u32) -> i16

{
    let mut pc_var1: String;
    let c_var2: u8;
    let c_var3: u8;
    let b_var4: u8;
    let b_var3: &mut Struct235;
    let b_var5: &mut Struct236;
    let mut pc_var5: String;
    let mut pc_var6: String;

    pc_var6 = param_2;
    pc_var5 = param_1;
    b_var5 = (s_You_may_not_run_a_turn__The_game_1050_00df + 0x20);
    loop {
        loop {
            c_var3 = b_var5;
            if (c_var3 == '\0') {
                // goto LAB_1000_479d;
            }
            pc_var1 = pc_var6;
            pc_var6 = pc_var6 + 0x1;
            c_var3 = *pc_var1;
            c_var2 = *pc_var5;
            b_var5 = CONCAT11(c_var2, c_var3);
            pc_var5 = pc_var5 + 0x1;
            if c_var2 != c_var3 { break; }
        }
        b_var4 = c_var3 + 0xbf + (-((c_var3 + 0xbf) < 0x1a) & 0x20) + 0x41;
        b_var3._0_1_ = c_var2 + 0xbf;
        b_var5._0_1_ = b_var3 + (-(b_var3 < 0x1a) & 0x20) + 0x41;
        b_var5 = CONCAT11(b_var4, b_var5);
    }
    while (b_var5 == b_var4);
    c_var3 = (b_var5 < b_var4) * -0x2 + '\x01';
//LAB_1000_479d:
    return c_var3;
}


pub fn pass1_1000_47a4(param_1: u32, param_2: u32, param_3: u16) -> u16

{
    let pb_var1: *mut u8;
    let b_var2: u8;
    let pu_var3: *mut u16;
    let pb_var4: *mut u8;
    let i_var5: i16;
    let pb_var6: *mut u8;
    let pu_var7: *mut u16;
    let u_var8: u16;
    let local_22: [u16; 0x10];

    pu_var7 = local_22;
    // TODO: refactor for loop
    // for (i_var5 = 0x10; i_var5 != 0x0; i_var5 += -0x1) {
    //   pu_var3 = pu_var7;
    //   pu_var7 = pu_var7 + 0x1;
    //   *pu_var3 = 0x0;
    // }
    pb_var6 = param_2;
    loop {
        pb_var1 = pb_var6;
        pb_var6 = pb_var6 + 0x1;
        b_var2 = *pb_var1;
        if (b_var2 == 0x0) { break; }
        pb_var1 = (local_22 + (b_var2 >> 0x3));
        *pb_var1 = *pb_var1 | '\x01' << (b_var2 & 0x7);
    }
    pb_var1 = param_1;
    if (param_1 == 0x0) {
        pb_var1 = pbRam105061e4;
    }
    loop {
        pbRam105061e4 = pb_var1;
        // u_var8 = (pbRam105061e4 >> 0x10);
        pb_var6 = (pbRam105061e4 + 0x1);
        b_var2 = *pbRam105061e4;
        if (b_var2 == 0x0) {
            return 0x0;
        }
        pb_var1 = (pbRam105061e4 & 0xffff0000 | ZEXT24(pb_var6));
        if ('\x01' << (b_var2 & 0x7) & (local_22 + (b_var2 >> 0x3))) != 0x0) == false
        { break; }
    }
    loop {
        pb_var4 = pb_var6;
        b_var2 = *pb_var4;
        if (b_var2 == 0x0) {
            // goto
            // LAB_1000_483c;
        }
        pb_var6 = pb_var4 + 0x1;
        if (('\x01' << (b_var2 & 0x7) & (local_22 + (b_var2 >> 0x3))) == 0x0) == false {
            break;
        }
    }
    *pb_var4 = 0x0;
    pb_var4 = pb_var4 + 0x1;
//LAB_1000_483c:
    pbRam105061e4 = (pbRam105061e4 & 0xffff0000 | ZEXT24(pb_var4));
    return pbRam105061e4;
}


pub fn pass1_1000_484c(param_1: u32, param_2: u32, param_3: u16) -> u16

{
    let pb_var1: *mut u8;
    let pb_var2: *mut u8;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let pb_var6: *mut u8;
    let pb_var7: *mut u8;
    let i_var8: i16;
    let b_var9: bool;
    let b_var10: bool;

    if (param_3 == 0x0) {
        return 0x0;
    }
    loop {
        // i_var8 = (param_2 >> 0x10);
        pb_var7 = param_2;
        // i_var3 = (param_1 >> 0x10);
        pb_var6 = param_1;
        u_var4 = ~pb_var7;
        u_var4 = ((param_3 - 0x1) - u_var4 & -(param_3 - 0x1 < u_var4)) + u_var4;
        u_var5 = ~pb_var6;
        u_var4 = (u_var4 - u_var5 & -(u_var4 < u_var5)) + u_var5 + 0x1;
        b_var9 = param_3 < u_var4;
        param_3 -= u_var4;
        b_var10 = param_3 == 0x0;
        loop {
            if (u_var4 == 0x0) { break; }
            u_var4 -= 0x1;
            pb_var2 = pb_var7;
            pb_var7 = pb_var7 + 0x1;
            pb_var1 = pb_var6;
            pb_var6 = pb_var6 + 0x1;
            b_var9 = *pb_var1 < *pb_var2;
            b_var10 = *pb_var1 == *pb_var2;
            if b_var10 == false { break; }
        }
        param_2 = param_2 & 0xffff0000 | ZEXT24(pb_var7);
        if (!b_var10) {
            return (0x1 - b_var9) - (b_var9 != 0x0);
        }
        if (param_3 == 0x0) {
            return u_var4;
        }
        if (pb_var6 == 0x0) {
            i_var3 += 0x6c;
        }
        param_1 = CONCAT22(i_var3, pb_var6);
        if (pb_var7 == 0x0) {
            param_2 = (i_var8 + 0x6c) << 0x10;
            param_1 = CONCAT22(i_var3, pb_var6);
        }
    }
}


pub fn pass1_1000_48a8(param_1: u32, param_2: u32, param_3: i16) -> u16

{
    let pu_var1: *mut u16;
    let pu_var2: *mut u16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let pu_var6: *mut u16;
    let pu_var7: *mut u16;
    let i_var8: i16;

    if (param_3 != 0x0) {
        loop {
            // i_var3 = (param_2 >> 0x10);
            pu_var6 = param_2;
            // i_var8 = (param_1 >> 0x10);
            pu_var7 = param_1;
            u_var4 = ~pu_var7;
            u_var4 = ((param_3 - 0x1) - u_var4 & -(param_3 - 0x1 < u_var4)) + u_var4;
            u_var5 = ~pu_var6;
            u_var4 = (u_var4 - u_var5 & -(u_var4 < u_var5)) + u_var5 + 0x1;
            param_3 -= u_var4;
            // TODO: refactor for loop
            // for (u_var5 = u_var4 >> 0x1; u_var5 != 0x0; u_var5 -= 0x1) {
            //   pu_var2 = pu_var7;
            //   pu_var7 = pu_var7 + 0x1;
            //   pu_var1 = pu_var6;
            //   pu_var6 = pu_var6 + 0x1;
            //   *pu_var2 = *pu_var1;
            // }
            // TODO: refactor for loop
            // for (u_var4 = ((u_var4 & 0x1) != 0x0); u_var4 != 0x0; u_var4 -= 0x1) {
            //   pu_var2 = pu_var7;
            //   pu_var7 = (pu_var7 + 0x1);
            //   pu_var1 = pu_var6;
            //   pu_var6 = (pu_var6 + 0x1);
            //   *pu_var2 = *pu_var1;
            // }
            if (param_3 == 0x0) { break; }
            if (pu_var6 == 0x0) {
                i_var3 += 0x6c;
            }
            param_1 = param_1 & 0xffff0000 | ZEXT24(pu_var7);
            param_2 = CONCAT22(i_var3, pu_var6);
            if (pu_var7 == 0x0) {
                param_1 = (i_var8 + 0x6c) << 0x10;
                param_2 = CONCAT22(i_var3, pu_var6);
            }
        }
    }
    return param_1;
}


pub fn pass1_1000_4906(param_1: &mut Struct20, in_wnd_class: Option<&WNDCLASS16>, param_3: u16) -> u16

{
    let pu_var1: *mut u16;
    let u_var2: u8;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let pu_var7: *mut u16;
    let i_var8: i16;

    if param_3 != 0x0 {
        // i_var8 = (param_1 >> 0x10);
        u_var5 = -param_1;
        u_var6 = param_3;
        if u_var5 != 0x0 {
            u_var6 = (u_var5 - param_3 & -(u_var5 < param_3)) + param_3;
            u_var5 = param_3 - u_var6;
        }
        u_var3 = in_wnd_class & 0xff | in_wnd_class << 0x8;
        pu_var7 = param_1;
        // TODO: refactor for loop
        // for (u_var4 = u_var6 >> 0x1; u_var4 != 0x0; u_var4 -= 0x1) {
        //   pu_var1 = pu_var7;
        //   pu_var7 = pu_var7 + 0x1;
        //   *pu_var1 = u_var3;
        // }
        // for (u_var6 = ((u_var6 & 0x1) != 0x0);
        //     u_var2 = (in_wnd_class & 0xff), u_var6 != 0x0; u_var6 -= 0x1) {
        //   pu_var1 = pu_var7;
        //   pu_var7 = (pu_var7 + 0x1);
        //   *pu_var1 = u_var2;
        // }
        if u_var5 != 0x0 {
            // TODO: refactor for loop
            // for (u_var6 = u_var5 >> 0x1; u_var6 != 0x0; u_var6 -= 0x1) {
            //   pu_var1 = pu_var7;
            //   pu_var7 = pu_var7 + 0x1;
            //   *pu_var1 = u_var3;
            // }
            // for (u_var6 = ((u_var5 & 0x1) != 0x0); u_var6 != 0x0; u_var6 -= 0x1) {
            //   pu_var1 = pu_var7;
            //   pu_var7 = (pu_var7 + 0x1);
            //   *pu_var1 = u_var2;
            // }
        }
    }
    return param_1;
}


pub fn pass1_1000_49b2(param_1: u16) -> i16

{
    return (param_1 ^ param_1 >> 0xf) - (param_1 >> 0xf);
}


pub fn pass1_1000_49c6(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16,
                       param_6: u16, param_7: *mut u8, param_8: i16) -> u16

{
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: i16;
    let l_var5: i32;
    let u_stack20: u16;
    let u_stack18: u16;
    let u_stack14: u16;
    let u_stack12: u16;
    let u_stack10: i16;
    let u_stack8: u16;
    let u_stack6: u16;
    let local_4: u16;
    let i_stack2: i16;

    i_stack2 = param_8 + 0x1;
    local_4 = SUB42(ctx.data_seg, 0x0);
    u_stack20 = param_3;
    u_stack18 = param_4;
    l_var5 = pass1_1000_52be(param_5 - 0x1, -(param_5 == 0x0), param_6, 0x0);
    u_stack8 = (l_var5 + 0x8);
    u_stack6 = ((l_var5 + 0x8) >> 0x10) * 0x100 + param_4;
    loop {
        if (u_stack6 < u_stack18) {
            return 0x0;
        }
        if ((u_stack6 <= u_stack18) && (u_stack8 < u_stack20)) {
            return 0x0;
        }
        u_stack14 = param_5 >> 0x1;
        if (u_stack14 == 0x0) {
            if ((param_5 != 0x0) && (i_var4 = (*param_7)(), i_var4 == 0x0)) {
                return u_stack20;
            }
            return 0x0;
        }
        u_var1 = u_stack14;
        if ((param_5 & 0x1) == 0x0) {
            u_var1 = u_stack14 - 0x1;
        }
        u_var2 = (u_var1 * param_6);
        u_var3 = u_var2 + u_stack20;
        iStack10 = ((u_var1 * param_6 >> 0x10) + CARRY2(u_var2, u_stack20)) * 0x100 + u_stack18;
        u_stack12 = u_var3;
        i_var4 = (*param_7)();
        if (i_var4 == 0x0) { break; }
        if (i_var4 < 0x0) {
            u_stack8 = -param_6 + u_stack12;
            u_stack6 = (CARRY2(-param_6, u_stack12) - (param_6 != 0x0)) * 0x100 + iStack10;
            u_var1 = param_5 & 0x1;
            param_5 = u_stack14;
            if (u_var1 == 0x0) {
                param_5 = u_stack14 - 0x1;
            }
        } else {
            u_stack20 = param_6 + u_stack12;
            u_stack18 = CARRY2(param_6, u_stack12) * 0x100 + iStack10;
            param_5 = u_stack14;
        }
    }
    return u_var3;
}


pub fn pass1_1000_4aea(param_1: u16, param_2: u16, param_3: i16, param_4: u16, param_5: *mut u8,
                       param_6: i16, param_7: i16, param_8: u16, param_9: u16, param_10: u16)

{
    let pu_var1: *mut u16;
    let ppc_var2: u32;
    let l_var3: i32;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let i_var8: i16;
    let u_var9: u16;
    let u_var10: u16;
    let pu_var11: &mut Struct171;
    let u_var11: u16;
    let u_var12: u16;
    let b_var13: bool;
    let u_var14: u16;
    let u_var15: u16;
    let u_var16: u16;
    let u_var17: u16;
    let u_var18: u16;
    let u_var19: u16;
    let u_stack4: u16;
    let i_stack2: i16;

    i_stack2 = param_6 + 0x1;
    u_stack4 = SUB42(ctx.data_seg, 0x0);
    u_var12 = SUB42(ctx.data_seg, 0x0);
    if ((param_4 != 0x0) && (param_3 != 0x0)) {
        // TODO: refactor for loop
//       for (i_var8 = param_3 + -0x1; i_var8 != 0x0; i_var8 += -0x1) {
//       i_var4 = (*param_5)(param_9);
//       if (i_var4 < 0x0) {
//         u_var5 = param_3 - 0x1;
//         i_var8 = 0x0;
//         loop {
//           u_var5 >>= 0x1;
//           i_var8 += -0x1;
//       if (i_var8 != 0x0 && u_var5 != 0x0) == false { break;}
//         }
//         if (((-i_var8 * 0x8 >> 0x10) != 0x0) ||
//            (u_var5 = pass1_1000_3bac(), u_var5 < (-i_var8 * 0x8))) {
//           exit_1000_25f2(0x4b7b,param_9,param_7,-0x4,param_8,param_9,param_10);
//           return;
//         }
//         pu_var11 = &stack0xfff6;
//         l_var3 = (param_3 - 0x1) * param_4;
//         u_var6 = l_var3;
//         u_var5 = u_var6 + param_1;
//         u_var6 = ((l_var3 >> 0x10) + CARRY2(u_var6,param_1)) * 0x100 +
//                 param_2;
// //LAB_1000_4b7d:
//         if (pu_var11 <= &stack0xffee) {
//           return;
//         }
// //LAB_1000_4b81:
//         if ((param_2 < u_var6) || ((param_2 <= u_var6 && (param_1 < u_var5)))) {
//           pu_var1 = &pu_var11.field_0x14;
//           u_var10 = u_var5 + *pu_var1;
//           u_var9 = u_var6 + (-CARRY2(u_var5,*pu_var1) & 0x6c);
//           u_var14 = param_1;
//           u_var15 = param_2;
//           u_var18 = u_var5;
//           u_var19 = u_var6;
//           u_var7 = param_1;
//           u_var11 = param_2;
// //LAB_1000_4bbc:
//           loop {
//             pu_var1 = &pu_var11.field_0x14;
//             b_var13 = CARRY2(param_1,*pu_var1);
//             param_1 += *pu_var1;
//             param_2 += -b_var13 & 0x6c;
//             if ((param_1 != u_var18) || (param_2 != u_var19)) {
//               ppc_var2 = &pu_var11.field_0x16;
//               i_var8 = (**ppc_var2)(param_9,param_1,param_2,u_var7,u_var11);
//               if (i_var8 < 0x1) {
//                 if (i_var8 != 0x0) {
//                   u_var14 = param_1;
//                   u_var15 = param_2;
//                 }
// //                 TODO: goto LAB_1000_4bbc;
//               }
//             }
//             loop {
//               u_var17 = u_var6;
//               u_var16 = u_var5;
//               pu_var1 = &pu_var11.field_0x14;
//               b_var13 = u_var10 < *pu_var1;
//               u_var10 -= *pu_var1;
//               u_var9 -= -b_var13 & 0x6c;
//               ppc_var2 = &pu_var11.field_0x16;
//               i_var8 = (**ppc_var2)(param_9,u_var7,u_var11,u_var10,u_var9);
//               if (0x0 < i_var8) break;
//               u_var5 = u_var10;
//               u_var6 = u_var9;
//             } while (((i_var8 != 0x0) || (u_var5 = u_var16, u_var6 = u_var17, u_var10 != u_var7))
//                     || (u_var9 != u_var11));
//             if ((u_var9 < param_2) || ((u_var9 <= param_2 && (u_var10 <= param_1))))
// //             TODO: goto LAB_1000_4c58;
//             pass1_1000_4ceb(pu_var11.field_0x14,param_1,u_var10,u_var9);
//             u_var14 = param_1;
//             u_var15 = param_2;
//             u_var5 = u_var10;
//             u_var6 = u_var9;
//           } while( true );
//         }
// //         TODO: goto LAB_1000_4b7d;
//       }
//     }
    }
    return;
//LAB_1000_4c58:
    param_1 = u_var7;
    param_2 = u_var11;
    pass1_1000_4ceb(pu_var11.field_0x14, u_var7, u_var10, u_var9);
    u_var11 = ((u_var19 - (-(u_var18 < u_var16) & 0x6c)) - u_var17) + (-CARRY2(u_var18 - u_var16, param_1) & 0x6c) + param_2;
    u_var7 = -((u_var18 - u_var16) + param_1 < u_var14) & 0x6c;
    u_var5 = u_var14;
    u_var6 = u_var15;
    if ((u_var7 <= u_var11) && (u_var15 <= u_var11 - u_var7)) {
        u_var5 = u_var18;
        u_var6 = u_var19;
        param_1 = u_var16;
        param_2 = u_var17;
    }
//   TODO: goto LAB_1000_4b81;
}


pub fn pass1_1000_4ceb(param_1: u16, param_2: i16, param_3: i16, param_4: u16) {
    let pu_var1: *mut u8;
    let pu_var2: *mut u16;
    let u_var3: u8;
    let u_var4: u16;

    if ((param_1 & 0x1) != 0x0) {
        param_1 -= 0x1;
        pu_var1 = (param_1 + param_3);
        u_var3 = *pu_var1;
        *pu_var1 = *(param_1 + param_2);
        *(param_1 + param_2) = u_var3;
        if (param_1 == 0x0) {
            return;
        }
    }
    loop {
        param_1 -= 0x2;
        pu_var2 = (param_1 + param_3);
        u_var4 = *pu_var2;
        *pu_var2 = (param_1 + param_2);
        (param_1 + param_2) = u_var4;
        if param_1 == 0 { break; }
    }
    return;
}


pub fn pass1_1000_4d0c(
    ctx: &mut AppContext,
    param_1: u16) {
    ctx.DAT_1050_61e8 = param_1;
    ctx.PTR_LOOP_1050_61ea = 0x0;
    return;
}


pub fn pass1_1000_4d24(
    ctx: &mut AppContext
) -> u16

{
    let l_var1: i32;

    l_var1 = pass1_1000_52be(ctx.DAT_1050_61e8, ctx.PTR_LOOP_1050_61ea,
                             (ctx.s_TPPOPMENU_1050_43fa + 0x3), 0x3);
    ctx.PTR_LOOP_1050_61ea = ((l_var1 + 0x269ec3) >> 0x10);
    ctx.DAT_1050_61e8 = (l_var1 + 0x269ec3);
    return ctx.PTR_LOOP_1050_61ea & 0x7fff;
}


pub fn pass1_1000_4f1a(param_1: i16, param_2: u16, param_3: u16) -> *mut i16

{
    let pi_var1: *mut i16;
    let mut pc_var2: String;
    let mut str: String;
    let pi_var3: *mut i16;
    let pi_var4: *mut i16;
    let mut pc_var5: String;
    let i_var6: i16;
    let i_var7: i16;

    i_var7 = 0x19;
    i_var6 = 0x19;
    pass1_1000_25a8(param_2, param_3);
    pass1_1000_2913(ctx, i_var6, param_2, param_3);
    str = poss_str_op_1000_28dc(ctx, i_var7);
    if (str != 0x0) {
        i_var6 = 0x9;
        if (*str == 'M') {
            i_var6 = 0xf;
        }
        str = str + i_var6;
        i_var6 = 0x22;
        pc_var5 = str;
        loop {
            if (i_var6 == 0x0) { break; }
            i_var6 += -0x1;
            pc_var2 = pc_var5;
            pc_var5 = pc_var5 + 0x1;
            if pc_var2 == '\r' { break; }
        }
        pc_var5[-0x1] = '\0';
    }
    FatalAppExit16(param_3, str);
    FatalExit();
    pi_var4 = &ctx.PTR_LOOP_1050_63fe;
    loop {
        pi_var1 = pi_var4;
        pi_var4 = pi_var4 + 0x1;
        i_var6 = *pi_var1;
        pi_var3 = pi_var4;
        if ((i_var6 == param_1) || (pi_var3 = (i_var6 + 0x1), pi_var3 == 0x0)) {
            return pi_var3;
        }
        i_var6 = -0x1;
        loop {
            if (i_var6 == 0x0) { break; }
            i_var6 += -0x1;
            pi_var1 = pi_var4;
            pi_var4 = (pi_var4 + 0x1);
            if *pi_var1 == '\0' { break; }
        }
    }
}


pub fn pass1_1000_4f2e(param_1: u16) -> u16

{
    let pc_var1: u32;
    let u_var2: u16;
    let u_var3: u8;

    u_var2 = 0x3b50;
    u_var3 = 0x0;
    if (true) {
        pc_var1 = swi(0x21);
        u_var2 = (*pc_var1)(ctx.data_seg, param_1 + 0x1);
    } else {
        DOS3Call(&ctx.PTR_LOOP_1050_1000);
    }
    if (!u_var3) {
        return 0x0;
    }
    pass1_1000_29b5(u_var2);
    return 0xffff;
}

pub fn pass1_1000_5008(param_1: u16, param_2: u16, param_3: u16, param_4: i16) {
    let unaff_cs: u16;
    let unaff_ss: u16;
    let i_stack2: i16;

    i_stack2 = param_4 + 0x1;
    pass1_1000_5026(0x0, param_1, param_2, param_3, &i_stack2, unaff_cs, unaff_ss);
    return;
}


pub fn pass1_1000_5224(param_1: u16, param_2: u16, param_3: u16, param_4: u16) -> u32

{
    let u_var1: u32;
    let l_var2: i32;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let u_var8: u16;
    let b_var10: bool;
    let c_var11: u8;
    let u_var9: u16;

    c_var11 = param_2 < 0x0;
    if (c_var11) {
        b_var10 = param_1 != 0x0;
        param_1 = -param_1;
        param_2 = -b_var10 - param_2;
    }
    if (param_4 < 0x0) {
        c_var11 += '\x01';
        b_var10 = param_3 != 0x0;
        param_3 = -param_3;
        param_4 = -b_var10 - param_4;
    }
    u_var3 = param_1;
    u_var5 = param_3;
    u_var6 = param_2;
    u_var9 = param_4;
    if (param_4 == 0x0) {
        u_var3 = param_2 / param_3;
        i_var4 = ((param_2 % param_3 << 0x10 | param_1) / param_3);
    } else {
        loop {
            u_var8 = u_var9 >> 0x1;
            u_var5 = u_var5 >> 0x1 | ((u_var9 & 0x1) != 0x0) << 0xf;
            u_var7 = u_var6 >> 0x1;
            u_var3 = u_var3 >> 0x1 | ((u_var6 & 0x1) != 0x0) << 0xf;
            u_var6 = u_var7;
            u_var9 = u_var8;
            if u_var8 == 0x0 { break; }
        }
        u_var1 = CONCAT22(u_var7, u_var3) / u_var5;
        i_var4 = u_var1;
        l_var2 = param_3 * (u_var1 & 0xffff);
        // u_var3 = (l_var2 >> 0x10);
        u_var5 = u_var3 + i_var4 * param_4;
        if (((CARRY2(u_var3, i_var4 * param_4)) || (param_2 < u_var5)) || ((param_2 <= u_var5 && (param_1 < l_var2)))) {
            i_var4 += -0x1;
        }
        u_var3 = 0x0;
    }
    if (c_var11 == '\x01') {
        b_var10 = i_var4 != 0x0;
        i_var4 = -i_var4;
        u_var3 = -b_var10 - u_var3;
    }
    return CONCAT22(u_var3, i_var4);
}


pub fn pass1_1000_52be(param_1: u16, param_2: u16, param_3: u16, param_4: u16) -> u32

{
    if (param_4 | param_2) == 0x0 {
        return param_1 * param_3;
    }
    return param_1 * param_3 & 0xffff | ((param_1 * param_3 >> 0x10) + param_2 * param_3 + param_1 * param_4) << 0x10;
}


pub fn pass1_1000_52f0(param_1: u16, param_2: u16, param_3: u16, param_4: u16) -> u32

{
    let u_var1: u32;
    let l_var2: i32;
    let u_var3: u16;
    let u_var4: u16;
    let i_var5: i16;
    let i_var6: i16;
    let u_var7: u16;
    let u_var8: u16;
    let u_var9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let b_var12: bool;
    let b_var13: bool;

    b_var13 = param_2 < 0x0;
    if (b_var13) {
        b_var12 = param_1 != 0x0;
        param_1 = -param_1;
        param_2 = -b_var12 - param_2;
    }
    u_var11 = b_var13;
    if (param_4 < 0x0) {
        b_var13 = param_3 != 0x0;
        param_3 = -param_3;
        param_4 = -b_var13 - param_4;
    }
    u_var3 = param_1;
    u_var4 = param_3;
    u_var8 = param_2;
    u_var9 = param_4;
    if (param_4 == 0x0) {
        i_var5 = ((param_2 % param_3 << 0x10 | param_1) % param_3);
        i_var6 = 0x0;
        if ((u_var11 - 0x1) < 0x0) {
            // goto
            // LAB_1000_538a;
        }
    } else {
        loop {
            u_var10 = u_var9 >> 0x1;
            u_var4 = u_var4 >> 0x1 | ((u_var9 & 0x1) != 0x0) << 0xf;
            u_var7 = u_var8 >> 0x1;
            u_var3 = u_var3 >> 0x1 | ((u_var8 & 0x1) != 0x0) << 0xf;
            u_var8 = u_var7;
            u_var9 = u_var10;
            if u_var10 == 0 { break; }
        }
        u_var1 = CONCAT22(u_var7, u_var3) / u_var4;
        u_var3 = u_var1 * param_4;
        l_var2 = (u_var1 & 0xffff) * param_3;
        // u_var8 = (l_var2 >> 0x10);
        u_var4 = l_var2;
        u_var9 = u_var8 + u_var3;
        if (((CARRY2(u_var8, u_var3)) || (param_2 < u_var9)) || ((param_2 <= u_var9 && (param_1 < u_var4)))) {
            b_var13 = u_var4 < param_3;
            u_var4 -= param_3;
            u_var9 = (u_var9 - param_4) - b_var13;
        }
        i_var5 = u_var4 - param_1;
        i_var6 = (u_var9 - param_2) - (u_var4 < param_1);
        if (-0x1 < (u_var11 - 0x1)) {
            // goto
            // LAB_1000_538a;
        }
    }
    b_var13 = i_var5 != 0x0;
    i_var5 = -i_var5;
    i_var6 = -b_var13 - i_var6;
//LAB_1000_538a:
    return CONCAT22(i_var6, i_var5);
}


pub fn pass1_1000_5390(param_1: u16, param_2: u16, param_3: u16, param_4: u16) -> u32

{
    let u_var1: u32;
    let l_var2: i32;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let u_var8: u16;
    let u_var9: u16;

    u_var3 = param_1;
    u_var8 = param_4;
    u_var6 = param_2;
    u_var9 = param_3;
    if param_4 == 0x0 {
        u_var3 = param_2 / param_3;
        i_var4 = ((param_2 % param_3 << 0x10 | param_1) / param_3);
    } else {
        loop {
            u_var5 = u_var8 >> 0x1;
            u_var9 = u_var9 >> 0x1 | ((u_var8 & 0x1) != 0x0) << 0xf;
            u_var7 = u_var6 >> 0x1;
            u_var3 = u_var3 >> 0x1 | ((u_var6 & 0x1) != 0x0) << 0xf;
            u_var8 = u_var5;
            u_var6 = u_var7;
            if u_var5 == 0 { break; }
        }
        u_var1 = CONCAT22(u_var7, u_var3) / u_var9;
        i_var4 = u_var1;
        l_var2 = param_3 * (u_var1 & 0xffff);
        // u_var3 = (l_var2 >> 0x10);
        u_var8 = u_var3 + i_var4 * param_4;
        if (((CARRY2(u_var3, i_var4 * param_4)) || (param_2 < u_var8)) || ((param_2 <= u_var8 && (param_1 < l_var2)))) {
            i_var4 += -0x1;
        }
        u_var3 = 0x0;
    }
    return CONCAT22(u_var3, i_var4);
}


pub fn pass1_1000_53f0(param_1: u16, param_2: u16, param_3: u16, param_4: u16) -> u32

{
    let u_var1: u32;
    let l_var2: i32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let i_var6: i16;
    let i_var7: i16;
    let u_var8: u16;
    let u_var9: u16;
    let u_var10: u16;
    let b_var11: bool;

    u_var3 = param_1;
    u_var4 = param_4;
    u_var9 = param_2;
    u_var10 = param_3;
    if (param_4 == 0x0) {
        i_var6 = ((param_2 % param_3 << 0x10 | param_1) % param_3);
        i_var7 = 0x0;
    } else {
        loop {
            u_var5 = u_var4 >> 0x1;
            u_var10 = u_var10 >> 0x1 | ((u_var4 & 0x1) != 0x0) << 0xf;
            u_var8 = u_var9 >> 0x1;
            u_var3 = u_var3 >> 0x1 | ((u_var9 & 0x1) != 0x0) << 0xf;
            u_var4 = u_var5;
            u_var9 = u_var8;
            if u_var5 == 0 { break; }
        }
        u_var1 = CONCAT22(u_var8, u_var3) / u_var10;
        u_var3 = u_var1 * param_4;
        l_var2 = (u_var1 & 0xffff) * param_3;
        // u_var9 = (l_var2 >> 0x10);
        u_var4 = l_var2;
        u_var10 = u_var9 + u_var3;
        if (((CARRY2(u_var9, u_var3)) || (param_2 < u_var10)) || ((param_2 <= u_var10 && (param_1 < u_var4)))) {
            b_var11 = u_var4 < param_3;
            u_var4 -= param_3;
            u_var10 = (u_var10 - param_4) - b_var11;
        }
        i_var6 = -(u_var4 - param_1);
        i_var7 = -(u_var4 - param_1 != 0x0) - ((u_var10 - param_2) - (u_var4 < param_1));
    }
    return CONCAT22(i_var7, i_var6);
}


pub fn pass1_1000_545a(param_1: u32, param_2: u32) -> i16

{
    let pb_var1: *mut u8;
    let b_var2: u8;
    let b_var3: u8;
    let b_var4: u8;
    let pb_var5: *mut u8;
    let pb_var6: *mut u8;

    pb_var6 = param_2;
    pb_var5 = param_1;
    b_var4 = 0xff;
    loop {
        loop {
            if (b_var4 == 0x0) {
                // goto
                // LAB_1000_5499;
            }
            pb_var1 = pb_var6;
            pb_var6 = pb_var6 + 0x1;
            b_var4 = *pb_var1;
            b_var3 = *pb_var5;
            pb_var5 = pb_var5 + 0x1;
            if b_var3 != b_var4 { break; }
        }
        b_var2 = b_var4 + 0xbf + (-((b_var4 + 0xbf) < 0x1a) & 0x20) + 0x41;
        b_var3 += 0xbf;
        b_var4 = b_var3 + (-(b_var3 < 0x1a) & 0x20) + 0x41;
        if b_var4 != b_var2 { break; }
    }
    b_var4 = (b_var4 < b_var2) * -0x2 + 0x1;
//LAB_1000_5499:
    return b_var4;
}


pub fn pass1_1000_54a0(param_1: u32, param_2: u16, param_3: u16) -> u16

{
    let pu_var1: *mut u16;
    let u_var2: u8;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let pu_var7: *mut u16;
    let i_var8: i16;

    if (param_3 != 0x0) {
        // i_var8 = (param_1 >> 0x10);
        u_var5 = -param_1;
        u_var6 = param_3;
        if (u_var5 != 0x0) {
            u_var6 = (u_var5 - param_3 & -(u_var5 < param_3)) + param_3;
            u_var5 = param_3 - u_var6;
        }
        u_var3 = param_2 & 0xff | param_2 << 0x8;
        pu_var7 = param_1;
        // TODO: refactor for loop
        // for (u_var4 = u_var6 >> 0x1; u_var4 != 0x0; u_var4 -= 0x1) {
        //   pu_var1 = pu_var7;
        //   pu_var7 = pu_var7 + 0x1;
        //   *pu_var1 = u_var3;
        // }
        // for (u_var6 = ((u_var6 & 0x1) != 0x0); u_var2 = (param_2 & 0xff),
        //     u_var6 != 0x0; u_var6 -= 0x1) {
        //   pu_var1 = pu_var7;
        //   pu_var7 = (pu_var7 + 0x1);
        //   *pu_var1 = u_var2;
        // }
        if u_var5 != 0x0 {
            // TODO: refactor for loop
            // for (u_var6 = u_var5 >> 0x1; u_var6 != 0x0; u_var6 -= 0x1) {
            //   pu_var1 = pu_var7;
            //   pu_var7 = pu_var7 + 0x1;
            //   *pu_var1 = u_var3;
            // }
            // for (u_var6 = ((u_var5 & 0x1) != 0x0); u_var6 != 0x0; u_var6 -= 0x1) {
            //   pu_var1 = pu_var7;
            //   pu_var7 = (pu_var7 + 0x1);
            //   *pu_var1 = u_var2;
            // }
        }
    }
    return param_1;
}


pub fn pass1_1000_54e8(param_1: *mut u8, param_2: u16, param_3: i16, param_4: i16, param_5: i16,
                       param_6: u16)

{
    let i_var1: i16;
    let i_var2: i16;
    let u_stack14: u16;
    let u_stack10: i16;
    let u_stack8: u16;

    i_var2 = param_5 + param_4 * param_3;
    i_var1 = param_3;
    while (i_var1 += -0x1, -0x1 < i_var1) {
        i_var2 -= param_4;
        u_stack8 = param_6;
        u_stack14 = 0x5506;
        iStack10 = i_var2;
        (*param_1)();
    }
    return;
}


pub fn pass1_1000_5512(param_1: *mut u8, param_2: u16, param_3: i16, param_4: i16, param_5: u16) {
    let b_var1: bool;
    let u_stack4: u16;

    pass1_1000_52be(param_3, param_4, param_5, 0x0);
    loop {
        b_var1 = param_3 == 0x0;
        param_3 += -0x1;
        param_4 -= b_var1;
        if param_4 < 0x0 { break; }
        u_stack4 = param_2;
        (*param_1)();
    }
    return;
}


pub fn pass1_1000_5586(param_1: *mut u8, param_2: u16, param_3: i16, param_4: i16, param_5: i16,
                       param_6: u16)

{
    let i_var1: i16;
    let i_var2: i16;
    let u_stack14: u16;
    let u_stack10: i16;
    let u_stack8: u16;

    i_var1 = param_5;
    i_var2 = param_3;
    while (i_var2 += -0x1, -0x1 < i_var2) {
        u_stack8 = param_6;
        u_stack14 = 0x559b;
        iStack10 = i_var1;
        (*param_1)();
        i_var1 += param_4;
    }
    return;
}


pub fn pass1_1000_55b1(param_1: i16, param_2: u16, param_3: u16) -> *mut i16

{
    let pi_var2: *mut i16;
    let mut pc_var3: String;
    let mut str: String;
    let pi_var4: *mut i16;
    let pi_var5: *mut i16;
    let mut pc_var6: String;
    let i_var7: i16;
    let i_var8: i16;
    let mut pi_var1: String;

    i_var8 = 0x14;
    i_var7 = 0x14;
    pass1_1000_25a8(param_2, param_3);
    pass1_1000_2913(ctx, i_var7, param_2, param_3);
    str = poss_str_op_1000_28dc(ctx, i_var8);
    if (str != 0x0) {
        i_var7 = 0x9;
        if (*str == 'M') {
            i_var7 = 0xf;
        }
        str = str + i_var7;
        i_var7 = 0x22;
        pc_var6 = str;
        loop {
            if (i_var7 == 0x0) { break; }
            i_var7 += -0x1;
            pc_var3 = pc_var6;
            pc_var6 = pc_var6 + 0x1;
            if *pc_var3 == '\r' { break; }
        }
        pc_var6[-0x1] = '\0';
    }
    FatalAppExit16(param_3, str);
    FatalExit();
    pi_var5 = &ctx.PTR_LOOP_1050_63fe;
    loop {
        pi_var2 = pi_var5;
        pi_var5 = pi_var5 + 0x1;
        i_var7 = *pi_var2;
        pi_var4 = pi_var5;
        if ((i_var7 == param_1) || (pi_var4 = (i_var7 + 0x1), pi_var4 == 0x0)) {
            return pi_var4;
        }
        i_var7 = -0x1;
        loop {
            if (i_var7 == 0x0) { break; }
            i_var7 += -0x1;
            pi_var1 = pi_var5;
            pi_var5 = (pi_var5 + 0x1);
            if *pi_var1 == '\0' { break; }
        }
    }
}
