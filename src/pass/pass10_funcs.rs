use crate::bad_funcs::bad1::{_in, halt_baddata, out};
use crate::bad_funcs::bad2::bad_fn_1050_335f;
use crate::draw::misc::process_struct_1040_9252;
use crate::err_ops::error_check_1000_17ce;
use crate::mem_funcs::Address;
use crate::mem_funcs::mem_ops_1::StructuredData;
use crate::pass::pass19_funcs::pass1_1040_79c0;
use crate::pass::pass20_funcs::pass1_1010_8ef2;
use crate::pass::pass6_funcs::pass1_1038_b6e0;
use crate::pass::pass7_funcs::{pass1_1018_5732, pass1_1018_5742};
use crate::pass::pass8_funcs::pass1_1010_2ee2;
use crate::pass::pass9_funcs;
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1040_b082, process_struct_1040_b0bc, process_struct_1040_c630};
use crate::struct_ops::struct_ops_2::process_struct_1040_7728;
use crate::structs::prog_structs_15::Struct337;
use crate::structs::prog_structs_18::Struct338;
use crate::structs::prog_structs_21::{Struct343, Struct74};
use crate::structs::prog_structs_23::Struct330;
use crate::structs::prog_structs_24::Struct2111;
use crate::structs::prog_structs_28::{Struct327, Struct331, Struct346};
use crate::structs::prog_structs_29::{Struct114, Struct328, Struct332};
use crate::structs::prog_structs_2::Struct199;
use crate::structs::prog_structs_30::Struct333;
use crate::structs::prog_structs_31::{Struct113, Struct329, Struct335};
use crate::structs::prog_structs_8::Struct68;
use crate::sys_ops::get_prop_1040_9566;
use crate::sys_ops::proc::free_proc_inst_1040_911e;
use crate::sys_ops::proc::make_proc_inst_1040_8fb8;
use crate::sys_ops::win::win_cleanup_func_1040_b0f8;
use crate::ui_ops::window::win_cleanup_1040_d1bc;
use crate::util::{CARRY1, CONCAT11, CONCAT22, SBORROW1, SUB42};

pub unsafe fn pass1_1050_309c() {
    let pb_var1: Vec<u8>;
    let pu_var2: &mut  u32;
    let pcVar3: String;
    let piVar4: &mut  i32;
    let mut b_var5: u8;
    let mut b_var6: u8;

    let mut b_var11: u8;
    let local_AX_15: &mut  Struct329;
    let mut cVar12: u8;
    let local_AX_111: &mut  Struct327;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let mut u_var9: i32;
    let pc_var10: String;
    let mut b_var13: u8;
    let mut in_cx: i32;
    let mut b_var14: u8;
    let mut b_var15: u8;
    let mut cVar16: u8;
    let mut b_var17: u8;
    let mut b_var18: u8;
    let mut in_dx: i32;
    let mut i_var19: i32;
    let mut b_var20: u8;
    let mut c_var21: u8;
    let mut b_var22: u8;
    let local_BX__1: &mut  Struct328;
    let mut b_var23: u8;
    let pu_var24: &mut  u16;
    let unaff_bp: &mut  u16;
    let unaff_si: String;
    let pc_var25: String;
    let unaff_DI: Vec<u8>;
    let mut unaff_ss: u16;
    let mut unaff_DS: u16;
    let mut in_CF: u8;
    let mut b_var26: bool;
    let mut b_var27: bool;
    let mut bStack002a: u8;
    let mut in_stack_0000002a: u16;
    let mut bStack002b: u8;
    let mut in_stack_0000502a: u8;
    let mut in_stack_0000502b: u8;
    let mut bStack3: u8;

    pu_var24 = &stack0xfffe;
    cVar12 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var24 = pu_var24 + -1;
        unsafe { *pu_var24 = *unaff_bp };
        cVar12 = cVar12 + -1;
        '\0' < cVar12
    } {}
    pb_var1 = &stack0xfffe + unaff_DI;
    b_var13 = in_cx;
    unsafe { b_var6 = *pb_var1 };
    unsafe { *pb_var1 = *pb_var1 >> (b_var13 & 0x1f) };
    b_var26 = (in_cx & 0x1f) != 0;
    b_var22 = local_BX__1;
    b_var11 = (in_ax >> 8)
        + b_var22
        + (!b_var26 & in_CF | (b_var26 && (b_var6 >> (b_var13 & 0x1f) - 1 & 1) != 0));
    pu_var2 = (local_BX__1 + unaff_si + 0x10);
    let pu_var2_val = unsafe { *pu_var2 };
    i_var19 = in_dx - pu_var2_val;
    out(0x2b, in_ax);
    local_AX_15._0_1_ = in_ax + (i_var19 >> 8) + (in_dx < pu_var2_val);
    i_var19 = (i_var19 - (local_BX__1 + unaff_si + 0x10)) - (local_BX__1 + unaff_si + 0x10);
    pb_var1 = 0x502c;
    unsafe { b_var6 = *pb_var1 };
    unsafe { b_var18 = *pb_var1 };
    unsafe { *pb_var1 = b_var18 + b_var13 + (0xd3 < local_AX_15) };
    pcVar3 = unaff_si + &local_BX__1.field_0x0;
    let pc_var3_val = unsafe { *pcVar3 };
    unsafe {
        *pcVar3 = *pcVar3
            + b_var22
            + (CARRY1(b_var6, b_var13) || CARRY1(b_var18 + b_var13, 0xd3 < local_AX_15))
    };
    pcVar3 = &stack0xfffe + unaff_si;
    unsafe { *pcVar3 = *pcVar3 + b_var11 + ((local_AX_15 + 0x2c) < 0x50) };
    pcVar3 = unaff_si;
    b_var14 = (in_cx >> 8);
    unsafe { *pcVar3 = *pcVar3 + b_var14 + ((local_AX_15 - 0x24) < 0x50) };
    b_var26 = (local_AX_15 + 0x8c) < 0x50;
    b_var5 = local_AX_15 + 0x3c;
    pb_var1 = 0x502c;
    b_var20 = (i_var19 >> 8);
    unsafe {
        b_var27 = CARRY1(*pb_var1, b_var20) || CARRY1(*pb_var1 + b_var20, b_var26);
        unsafe { *pb_var1 = *pb_var1 + b_var20 + b_var26 };
        pb_var1 = (local_BX__1 + unaff_si + 0x2c);
        b_var26 = CARRY1(*pb_var1, b_var5) || CARRY1(*pb_var1 + b_var5, b_var27);
        unsafe { *pb_var1 = *pb_var1 + b_var5 + b_var27 };
        pb_var1 = ((register0x00000010 + 0x2a) + unaff_si);
        b_var27 = CARRY1(*pb_var1, b_var13) || CARRY1(*pb_var1 + b_var13, b_var26);
    }
    unsafe { *pb_var1 = *pb_var1 + b_var13 + b_var26 };
    pb_var1 = (unaff_si + 0x2c);
    unsafe { b_var6 = *pb_var1 };
    b_var15 = i_var19;
    unsafe { b_var18 = *pb_var1 };
    unsafe { *pb_var1 = b_var18 + b_var15 + b_var27 };
    b_var26 = CARRY1(bStack002a, b_var22)
        || CARRY1(
            bStack002a + b_var22,
            CARRY1(b_var6, b_var15) || CARRY1(b_var18 + b_var15, b_var27),
        );
    pb_var1 = (local_BX__1 + unaff_si + 0x2c);
    unsafe {
        b_var27 = CARRY1(*pb_var1, b_var14) || CARRY1(*pb_var1 + b_var14, b_var26);
        unsafe { *pb_var1 = *pb_var1 + b_var14 + b_var26 };
        pb_var1 = ((register0x00000010 + 0x2a) + unaff_si);
        b_var26 = CARRY1(*pb_var1, b_var20) || CARRY1(*pb_var1 + b_var20, b_var27);
        unsafe { *pb_var1 = *pb_var1 + b_var20 + b_var27 };
        pb_var1 = (unaff_si + 0x2c);
        b_var23 = (local_BX__1 >> 8);
        b_var27 = CARRY1(*pb_var1, b_var23) || CARRY1(*pb_var1 + b_var23, b_var26);
        unsafe { *pb_var1 = *pb_var1 + b_var23 + b_var26 };
        b_var26 = CARRY1(in_stack_0000502a, b_var5) || CARRY1(in_stack_0000502a + b_var5, b_var27);
        b_var6 = in_stack_0000502a + b_var5 + b_var27;
        pb_var1 = (local_BX__1 + unaff_si + 0x502c);
        b_var27 = CARRY1(*pb_var1, b_var15) || CARRY1(*pb_var1 + b_var15, b_var26);
        unsafe { *pb_var1 = *pb_var1 + b_var15 + b_var26 };
        pb_var1 = &stack0x502a + unaff_si;
        b_var26 = CARRY1(*pb_var1, b_var22) || CARRY1(*pb_var1 + b_var22, b_var27);
    }
    unsafe { *pb_var1 = *pb_var1 + b_var22 + b_var27 };
    pb_var1 = (unaff_si + 0x502c);
    unsafe { b_var18 = *pb_var1 };
    unsafe { b_var17 = *pb_var1 };
    unsafe { *pb_var1 = b_var17 + b_var11 + b_var26 };
    b_var26 = CARRY1(b_var6, b_var14)
        || CARRY1(
            b_var6 + b_var14,
            CARRY1(b_var18, b_var11) || CARRY1(b_var17 + b_var11, b_var26),
        );
    pb_var1 = (local_BX__1 + unaff_si + 0x502c);
    unsafe { b_var6 = *pb_var1 };
    unsafe { b_var18 = *pb_var1 };
    unsafe { *pb_var1 = b_var18 + b_var23 + b_var26 };
    cVar16 = b_var15 + b_var5 + (CARRY1(b_var6, b_var23) || CARRY1(b_var18 + b_var23, b_var26));
    cVar12 = b_var11 + b_var13 + (b_var5 < 0x50);
    c_var21 = b_var20 + cVar16 + ((local_AX_15 - 0x14) < 0x50);
    local_AX_111._0_1_ = local_AX_15 + 0x4c + cVar12 + ((local_AX_15 + 0x9c) < 0x50);
    b_var17 = cVar16 + b_var14 + (local_AX_111 < 0x50);
    b_var11 = c_var21 + b_var23 + ((local_AX_111 + 0x60) < 0x50);
    u_var7 = CONCAT11(
        cVar12 + c_var21 + ((local_AX_111 + 0xb0) < 0x50),
        local_AX_111 - 0x40,
    );
    pcVar3 = unaff_si + &local_BX__1.field_0x0;
    unsafe { *pcVar3 = *pcVar3 + b_var13 + ((local_AX_111 + 0x10) < 0x50) };
    pb_var1 = unaff_DI;
    unsafe {
        b_var6 = b_var14 + *pb_var1;
    }
    b_var5 = b_var6 + (u_var7 < 0x1050);
    pcVar3 = unaff_si;
    unsafe {
        *pcVar3 = *pcVar3 + b_var22 + (CARRY1(b_var14, *pb_var1) || CARRY1(b_var6, u_var7 < 0x1050))
    };
    pb_var1 = unaff_DI;
    unsafe { *pb_var1 = *pb_var1 ^ b_var5 };
    pcVar3 = &stack0xfffe + unaff_si;
    unsafe { *pcVar3 = *pcVar3 + b_var23 };
    pc_var25 = unaff_si + -1;
    u_var7 = CONCAT11(local_AX_111 + 0x70, (u_var7 + 0xcf10 >> 8));
    u_var8 = u_var7 + 0xefb0;
    b_var14 = u_var8;
    cVar12 = (u_var8 >> 8);
    pb_var1 = (unaff_si + 0x2c);
    unsafe { b_var6 = *pb_var1 };
    unsafe { b_var18 = *pb_var1 };
    unsafe { *pb_var1 = b_var18 + b_var5 + (u_var7 < 0x1050) };
    b_var26 = CARRY1(bStack002b, b_var11)
        || CARRY1(
            bStack002b + b_var11,
            CARRY1(b_var6, b_var5) || CARRY1(b_var18 + b_var5, u_var7 < 0x1050),
        );
    pb_var1 = (local_BX__1 + pc_var25 + 0x502d);
    unsafe {
        b_var27 = CARRY1(*pb_var1, b_var14) || CARRY1(*pb_var1 + b_var14, b_var26);
        unsafe { *pb_var1 = *pb_var1 + b_var14 + b_var26 };
        pb_var1 = &stack0x502b + pc_var25;
        b_var26 = CARRY1(*pb_var1, b_var13) || CARRY1(*pb_var1 + b_var13, b_var27);
        unsafe { *pb_var1 = *pb_var1 + b_var13 + b_var27 };
        pb_var1 = (unaff_si + 0x502c);
        unsafe { b_var6 = *pb_var1 };
        unsafe { b_var18 = *pb_var1 };
        unsafe { *pb_var1 = b_var18 + b_var17 + b_var26 };
        b_var26 = CARRY1(in_stack_0000502b, b_var22)
            || CARRY1(
                in_stack_0000502b + b_var22,
                CARRY1(b_var6, b_var17) || CARRY1(b_var18 + b_var17, b_var26),
            );
        pb_var1 = (local_BX__1 + pc_var25 + 0x502d);
        b_var27 = CARRY1(*pb_var1, b_var5) || CARRY1(*pb_var1 + b_var5, b_var26);
    }
    unsafe { *pb_var1 = *pb_var1 + b_var5 + b_var26 };
    pb_var1 = &stack0x502b + pc_var25;
    unsafe { b_var6 = *pb_var1 };
    unsafe { b_var18 = *pb_var1 };
    unsafe { *pb_var1 = b_var18 + b_var11 + b_var27 };
    pcVar3 = unaff_si + 0x502c;
    unsafe {
        *pcVar3 =
            *pcVar3 + b_var23 + (CARRY1(b_var6, b_var11) || CARRY1(b_var18 + b_var11, b_var27))
    };
    i_var19 = u_var7 + 0xdf60;
    pb_var1 = unaff_DI;
    unsafe { b_var6 = *pb_var1 };
    unsafe { *pb_var1 = *pb_var1 >> 1 };
    b_var18 = b_var17 + b_var22 + (b_var6 & 1);
    b_var6 = _in(0x2d);
    u_var9 = (u_var7 + 0xcf10 & 0xff00 | b_var6) + 0xdf60;
    b_var5 = b_var5 + *0x1050;
    pc_var10 = (u_var9 | 0x2e);
    pb_var1 = 0x502e;
    unsafe {
        b_var26 = CARRY1(*pb_var1, b_var18);
        unsafe { *pb_var1 = *pb_var1 + b_var18 };
        pb_var1 = (pc_var25 + &local_BX__1.field_0x0);
        b_var17 = (u_var9 >> 8);
        b_var27 = CARRY1(*pb_var1, b_var17) || CARRY1(*pb_var1 + b_var17, b_var26);
    }
    unsafe { *pb_var1 = *pb_var1 + b_var17 + b_var26 };
    pb_var1 = &stack0xfffe + pc_var25;
    unsafe { b_var6 = *pb_var1 };
    unsafe { b_var18 = *pb_var1 + b_var5 };
    unsafe { *pb_var1 = b_var18 + b_var27 };
    pcVar3 = &stack0xfffe + pc_var25;
    unsafe { *pcVar3 = *pcVar3 + b_var17 + (CARRY1(b_var6, b_var5) || CARRY1(b_var18, b_var27)) };
    pcVar3 = pc_var10 + u_var7 + 0xe364;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + u_var7 + 0xdf64;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pu_var2 = (pc_var10 + i_var19);
    unsafe { *pu_var2 = *pu_var2 | u_var8 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    i_var19 = CONCAT11((i_var19 >> 8), 4);
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    b_var18 = b_var14 ^ pc_var10[i_var19];
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var18 };
    pb_var1 = (pc_var10 + i_var19);
    unsafe { b_var6 = *pb_var1 };
    unsafe { *pb_var1 = *pb_var1 + b_var18 };
    piVar4 = (pc_var10 + i_var19);
    unsafe { *piVar4 = (*piVar4 - CONCAT11(4, b_var18)) - CARRY1(b_var6, b_var18) };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var18 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var18 };
    b_var18 = b_var18 ^ pc_var10[i_var19];
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + b_var18 };
    pb_var1 = (pc_var10 + 0x404);
    unsafe { b_var6 = *pb_var1 };
    unsafe { *pb_var1 = *pb_var1 + b_var18 };
    piVar4 = (pc_var10 + 0x404);
    unsafe { *piVar4 = (*piVar4 - CONCAT11(4, b_var18)) - CARRY1(b_var6, b_var18) };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + b_var18 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x508;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pc_var10 * 2);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pc_var10 * 2 + 0x104);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pc_var10 * 2 + 4);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = (pc_var10 * 2);
    unsafe { *pcVar3 = *pcVar3 + b_var14 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    cVar12 = b_var14 + 4;
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + 0x104;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + 4;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    cVar12 = cVar12 + b_var14;
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = 0x608;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    cVar12 = cVar12 * 0x2;
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = (pc_var10 * 2);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + 0x404;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    c_var21 = cVar12 + 0x4;
    i_var19 = CONCAT11(4, c_var21);
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + c_var21 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = (pc_var10 * 2);
    unsafe { *pcVar3 = *pcVar3 + 0x4 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar12 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + c_var21 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar12 + 0x1 };
    cVar16 = cVar12 + 0x4;
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + c_var21 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    i_var19 = CONCAT11(cVar12 + 0x8, c_var21);
    pcVar3 = (pc_var10 * 2);
    unsafe { *pcVar3 = *pcVar3 + b_var14 + cVar12 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    unsafe { *0x2 = pc_var10 };
    i_var19 = i_var19 + -1;
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    pcVar3 = pc_var10;
    unsafe { *pcVar3 = *pcVar3 + cVar12 + 0x8 };
    pcVar3 = pc_var10 + i_var19;
    unsafe { *pcVar3 = *pcVar3 + cVar16 };
    return;
}

pub unsafe fn pass1_1050_3344() {
    let pc_var1: String;
    let pu8_var2: Vec<u8>;
    let pi_var3: &mut  i32;
    let mut b_var4: u8;
    let mut b_var5: u8;
    let mut in_ax: i32;
    let mut u_var6: u16;
    let mut extraout_DH: u8;
    let mut in_b_x: i32;
    let unaff_s_i: String;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;

    b_var4 = in_ax;
    out(4, b_var4);
    pc_var1 = unaff_s_i + in_b_x;
    unsafe { *pc_var1 = *pc_var1 + b_var4 };
    b_var4 = b_var4 ^ unaff_s_i[in_b_x];
    pc_var1 = unaff_s_i + in_b_x;
    unsafe { *pc_var1 = *pc_var1 + b_var4 };
    out(4, in_ax & 0xff00 | b_var4);
    pu8_var2 = (unaff_s_i + in_b_x);
    unsafe { b_var5 = b_var4 + *pu8_var2 };
    pi_var3 = (unaff_s_i + in_b_x);
    unsafe { *pi_var3 = (*pi_var3 - (in_ax & 0xff00 | b_var5)) - CARRY1(b_var4, *pu8_var2) };
    pc_var1 = unaff_s_i + in_b_x;
    unsafe { *pc_var1 = *pc_var1 + b_var5 };
    u_var6 = bad_fn_1050_335f();
    pc_var1 = &stack0xfffe + unaff_s_i;
    unsafe { *pc_var1 = *pc_var1 + extraout_DH };
    pc_var1 = unaff_s_i + in_b_x;
    unsafe { *pc_var1 = *pc_var1 + u_var6 };
    pc_var1 = unaff_s_i;
    unsafe { *pc_var1 = *pc_var1 + (u_var6 >> 8) };
    pc_var1 = unaff_s_i + in_b_x;
    unsafe { *pc_var1 = *pc_var1 + u_var6 };
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub unsafe fn pass1_1048_1e02() -> i32 {
    let pc_var1: String;
    let mut c_var2: u8;
    let mut in_a_x: i32;
    let mut in_b_x: i32;
    let pu_var3: &mut  u16;
    let unaff_b_p: &mut  u16;
    let mut unaff_s_i: i32;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;

    pu_var3 = &stack0xfffe;
    c_var2 = '\x0f';
    while {
        unaff_b_p = unaff_b_p + -1;
        pu_var3 = pu_var3 + -1;
        unsafe { *pu_var3 = *unaff_b_p };
        c_var2 = c_var2 + -1;
        '\0' < c_var2
    } {}
    pc_var1 = (in_b_x + unaff_s_i);
    c_var2 = in_a_x;
    unsafe { *pc_var1 = *pc_var1 + c_var2 };
    pc_var1 = (in_b_x + unaff_s_i);
    unsafe { *pc_var1 = *pc_var1 + c_var2 };
    pc_var1 = (in_b_x + unaff_s_i);
    unsafe { *pc_var1 = *pc_var1 + c_var2 };
    pc_var1 = (in_b_x + unaff_s_i);
    unsafe { *pc_var1 = *pc_var1 + c_var2 };
    pc_var1 = (in_b_x + unaff_s_i);
    unsafe { *pc_var1 = *pc_var1 + c_var2 };
    return in_a_x + (in_b_x + unaff_s_i);
}

pub unsafe fn pass1_1048_3c50() {
    let pi_var1: &mut  i32;
    let pc_var2: String;
    let mut c_var3: u8;
    let mut in_a_x: u16;
    let mut i_var4: i32;
    let mut in_c_x: u16;
    let mut in_d_x: u16;
    let in_b_x: String;
    let mut c_var5: u8;
    let pu_var6: &mut  u16;
    let unaff_b_p: &mut  u16;
    let unaff_s_i: String;
    let unaff_d_i: String;
    let mut unaff_e_s: u16;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;

    pu_var6 = &stack0xfffe;
    c_var3 = '\x1e';
    while {
        unaff_b_p = unaff_b_p + -1;
        pu_var6 = pu_var6 + -1;
        unsafe { *pu_var6 = *unaff_b_p };
        c_var3 = c_var3 + -1;
        '\0' < c_var3
    } {}
    out(in_d_x, in_a_x);
    pi_var1 = (in_b_x + unaff_d_i);
    unsafe { *pi_var1 = *pi_var1 + 1 };
    pc_var2 = unaff_d_i;
    unsafe { *pc_var2 = *pc_var2 + in_d_x };
    pc_var2 = in_b_x + unaff_s_i;
    unsafe { *pc_var2 = *pc_var2 + (in_c_x >> 8) };
    pc_var2 = &stack0xfffe + unaff_s_i;
    c_var5 = (in_b_x >> 8);
    unsafe { *pc_var2 = *pc_var2 + c_var5 };
    pc_var2 = &stack0xfffe + unaff_d_i;
    unsafe { *pc_var2 = *pc_var2 + in_c_x };
    pc_var2 = unaff_s_i;
    unsafe { *pc_var2 = *pc_var2 + (in_a_x >> 8) };
    c_var3 = _in(in_d_x);
    unsafe { *unaff_d_i = c_var3 };
    pc_var2 = in_b_x + (unaff_d_i + 1);
    unsafe { *pc_var2 = *pc_var2 + (in_d_x >> 8) };
    i_var4 = (in_b_x + unaff_s_i) * 0x62;
    pc_var2 = in_b_x;
    unsafe { *pc_var2 = *pc_var2 + in_d_x };
    pc_var2 = in_b_x + unaff_s_i;
    unsafe { *pc_var2 = *pc_var2 + c_var5 };
    pc_var2 = 0x1300;
    unsafe { *pc_var2 = *pc_var2 + (i_var4 >> 8) };
    pc_var2 = in_b_x + unaff_s_i;
    unsafe { *pc_var2 = *pc_var2 + i_var4 };
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub unsafe fn pass1_1048_3da0() {
    let ppc_var1: fn();
    let mut c_var2: u8;
    let in_a_l: u8;
    let mut in_b_x: i32;
    let pu_var3: &mut  u16;
    let unaff_b_p: &mut  u16;
    let mut unaff_s_i: i32;
    let unaff_d_i: Vec<u8>;
    let mut unaff_e_s: u16;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;
    let mut local_6e03: u16;
    let mut local_6703: u16;
    let pu_stack64: Vec<u8>;

    pu_stack64 = &stack0xfffe;
    pu_var3 = &stack0xfffe;
    c_var2 = '\x1e';
    while {
        unaff_b_p = unaff_b_p + -1;
        pu_var3 = pu_var3 + -1;
        unsafe { *pu_var3 = *unaff_b_p };
        c_var2 = c_var2 + -1;
        '\0' < c_var2
    } {}
    unsafe { *unaff_d_i = in_a_l };
    ppc_var1 = (in_b_x + -0x6801);
    (**ppc_var1)();
    (**(&local_6e03 + unaff_s_i))();
    (**(&local_6703 + unaff_s_i))();
    // WARNING: Could not recover jumptable at 0x10483db1. Too many branches
    // WARNING: Treating indirect jump as call
    (**(in_b_x + unaff_s_i + -0x5401))();
    return;
}

pub unsafe fn pass1_1048_3de8() {
    let mut c_var1: u8;
    let mut in_a_x: u16;
    let mut in_b_x: i32;
    let p_uvar2: &mut  u16;
    let unaff_b_p: &mut  u16;
    let mut unaff_s_i: i32;
    let unaff_d_i: &mut  u16;
    let mut unaff_e_s: u16;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;

    pu_var2 = &stack0xfffe;
    c_var1 = '\x1e';
    while {
        unaff_b_p = unaff_b_p + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_b_p };
        c_var1 = c_var1 + -1;
        '\0' < c_var1
    } {}
    unsafe { *unaff_d_i = in_a_x };
    // WARNING: Could not recover jumptable at 0x10483ded. Too many branches
    // WARNING: Treating indirect jump as call
    (**(in_b_x + unaff_s_i + -0x6701))();
    return;
}

pub unsafe fn pass1_1048_4072() {
    let mut c_var1: u8;
    let p_uvar2: &mut  u16;
    let unaff_b_p: &mut  u16;
    let mut unaff_d_i: i32;
    let mut unaff_s_s: u16;
    let mut unaff_d_s: u16;
    let mut local_5503: u16;
    let pu_stack64: Vec<u8>;

    pu_stack64 = &stack0xfffe;
    pu_var2 = &stack0xfffe;
    c_var1 = '\x1e';
    unsafe {
        unaff_b_p = unaff_b_p + -1;
        pu_var2 = pu_var2 + -1;
        unsafe { *pu_var2 = *unaff_b_p };
        c_var1 = c_var1 + -1;
        '\0' < c_var1
    }
    []
    // WARNING: Could not recover jumptable at 0x10484079. Too many branches
    // WARNING: Treating indirect jump as call
    (* * (&stack0xaaff + unaff_d_i))();
    return;
}

pub unsafe fn pass1_1040_d89e(param_1: u32, param_2: u8) {
    win_cleanup_1040_d1bc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_d76e(param_1: &mut  Struct330) {
    let mut u_var1: u32;
    let local_b_x_3: &mut  Struct330;
    let mut u_var2: u16;

  // u_var2 = (param_1  >> 0x10);
    local_b_x_3 = param_1;
    u_var1 = local_b_x_3.field_0x94;
    pass1_1018_5742(
        u_var1,
        (u_var1 >> 0x10),
        local_b_x_3.field_0x9c,
        local_b_x_3.field_0x98,
    );
    local_b_x_3.field_0x9c = 0;
    return;
}

pub unsafe fn pass1_1040_d056(param_1: u32, param_2: u8) {
    pass1_1040_ca74(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_d0d8(param_1: u32) {
    let pb_var1: Vec<u8>;
    let mut b_var2: u8;
    let mut b_var3: u8;
    let mut c_var4: u8;
    let mut u_var5: i32;
    let mut in_a_l: u8;
    let mut in_c_l: u8;
    let mut b_var6: u8;
    let mut in_d_x: i32;
    let mut b_var7: u8;
    let mut in_b_x: i32;
    let mut b_var8: u8;
    let pu_var9: &mut  u16;
    let unaff_b_p: &mut  u16;
    let unaff_s_i: Vec<u8>;
    let mut unaff_s_s: u16;
    let mut in_c_f: u8;
    let mut b_var10: bool;
    let mut b_var11: bool;
    let local_4e: u8;

    pu_var9 = &stack0xfffe;
    c_var4 = '\x0f';
    unsafe {
        unaff_b_p = unaff_b_p + -1;
        pu_var9 = pu_var9 + -1;
        unsafe { *pu_var9 = *unaff_b_p };
        c_var4 = c_var4 + -1;
        '\0' < c_var4
    }
    {}
    b_var8 = (in_b_x >> 8);
    unaff_s_i[in_b_x] = b_var8;
    b_var7 = (in_d_x + 1 >> 8);
    b_var2 = b_var7 + b_var8;
    b_var10 = CARRY1(b_var7, b_var8) || CARRY1(b_var2, in_c_f);
    u_var5 = in_d_x + 1 & 0xff;
    pb_var1 = unaff_s_i + in_b_x;
    b_var6 = u_var5;
    unsafe { b_var7 = *pb_var1 };
    unsafe { b_var3 = *pb_var1 - b_var6 };
    unsafe { b_var11 = *pb_var1 < b_var6 || b_var3 < b_var10 };
    unsafe { *pb_var1 = b_var3 - b_var10 };
    let pb_var1_val = unsafe { *pb_var1 };
    if (((pb_var1_val != 0) && SBORROW1(b_var7, b_var6))
        != (SBORROW1(b_var3, b_var10) == (pb_var1_val < '\0')))
    {
        pb_var1 = unaff_s_i;
        unsafe { b_var7 = *pb_var1 };
        unsafe { b_var3 = *pb_var1 };
        unsafe { *pb_var1 = b_var3 + b_var8 + b_var11 };
        b_var10 = CARRY1(local_4e, in_b_x)
            || CARRY1(
                local_4e + in_b_x,
                CARRY1(b_var7, b_var8) || CARRY1(b_var3 + b_var8, b_var11),
            );
        pb_var1 = unaff_s_i + -0x4f;
        unsafe { b_var7 = *pb_var1 };
        unsafe { b_var3 = *pb_var1 };
        unsafe { *pb_var1 = b_var3 + b_var8 + b_var10 };
        return CONCAT22(
            u_var5
                | (b_var2
                    + in_c_f
                    + in_c_l
                    + (CARRY1(b_var7, b_var8) || CARRY1(b_var3 + b_var8, b_var10)))
                    << 8,
            CONCAT11(0x40, in_a_l + 0x1) + 2,
        );
    }
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_d0f8(param_1: &mut  u16, param_2: u16) {
    let mut u_var1: u16;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let struct_a: &mut  Struct199;
    let pa_var4: &mut  Struct199;
    let mut extraout_d_x: u16;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let pp_var7: &mut  Struct2111;
    let mut in_stack_0000fffa: u16;
    let mut local_4: u16;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 0x1845));
  // u_var6 = (param_1  >> 0x10);
    i_var5 = param_1;
    (i_var5 + 0x94) = 0;
    (i_var5 + 0x98) = _ctx.PTR_LOOP_1050_5f16;
    (i_var5 + 0x9c) = 0;
    (i_var5 + 0xa0) = 0;
    unsafe { *param_1 = 0xd8c4 };
    (i_var5 + 2) = &ctx.PTR_LOOP_1050_1040;
    pp_var7 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000fffa, 0x47),
    );
  // u_var3 = (pp_var7  >> 0x10);
    u_var1 = SUB42(pp_var7, 0);
    (i_var5 + 0x94) = u_var1;
    (i_var5 + 0x96) = u_var3;
    pass1_1018_5732((i_var5 + 0x94), u_var3, (i_var5 + 0x98));
    (i_var5 + 0x9c) = u_var1;
    (i_var5 + 0x9e) = struct_a;
    u_var2 = struct_a | (i_var5 + 0x9c);
    if (u_var2 == 0) {
        pa_var4 = struct_a;
        process_struct_1000_179c(0xc, struct_a);
        if ((pa_var4 | u_var2) == 0) {
            (i_var5 + 0x9c) = 0;
        } else {
            pass1_1010_8ef2(CONCAT22(pa_var4, u_var2));
            (i_var5 + 0x9c) = u_var2;
            (i_var5 + 0x9e) = extraout_d_x;
        }
    }
    return;
}

pub unsafe fn pass1_1040_ca74(param_1: &mut  u16) {
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    unsafe { *param_1 = 0xd07c };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    PTR_LOOP_1050_5f10 = 0x0;
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub unsafe fn pass1_1040_ca16(param_1: &mut  Struct331, param_2: u16) {
    let mut i_var1: i32;
    let mut unaff_b_p: u16;
    let mut u_var2: u16;
    let pp_var3: &mut  Struct2111;
    let local_a: &mut  Struct331;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 0x1840));
  // u_var2 = (param_1  >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x94) = _ctx.PTR_LOOP_1050_5f0c;
    (i_var1 + 0x98) = 0;
    (i_var1 + 0x9c) = 0;
    (i_var1 + 0x9e) = 0;
    param_1.field_0x0 = 0xd07c;
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1040;
    pp_var3 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_b_p, 0x3e));
    (i_var1 + 0x98) = pp_var3;
    (i_var1 + 0x9a) = (pp_var3 >> 0x10);
    return;
}

pub unsafe fn pass1_1040_c9cc(param_1: u32, param_2: u8) {
    pass1_1040_c5ac(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_ca03() {
    let pb_var1: Vec<u8>;
    let pc_var2: String;
    let mut b_var3: u8;
    let mut b_var4: u8;
    let pu_var5: &mut  u16;
    let mut c_var6: u8;
    let pu_var7: &mut  u16;
    let mut u_var8: u32;
    let mut in_a_l: u8;
    let mut b_var9: u8;
    let mut in_d_x: i32;
    let mut in_b_l: u8;
    let local_b_x_41: &mut  Struct332;
    let local_b_x_92: &mut  Struct333;
    let pu_var10: &mut  u16;
    let unaff_b_p: &mut  u16;
    let mut unaff_s_i: i32;
    let mut unaff_d_i: i32;
    let mut u_var11: u16;
    let mut unaff_s_s: u16;
    let mut in_c_f: u8;
    let mut b_var12: bool;
    let mut b_var13: bool;
    let pp_var14: &mut  Struct2111;
    let mut in_stack_00000000: u16;

    pu_var10 = &stack0xfffe;
    c_var6 = '\t';
    pu_var5 = unaff_b_p;
    while {
        pu_var5 = pu_var5 + -1;
        pu_var10 = pu_var10 + -1;
        unsafe { *pu_var10 = *pu_var5 };
        c_var6 = c_var6 + -1;
        '\0' < c_var6
    } {}
    b_var9 = in_d_x * 0x2 + in_c_f;
    b_var12 = (in_a_l + 1) < 8;
    pb_var1 = (unaff_b_p + unaff_s_i + -0x38);
    unsafe {
        b_var13 = CARRY1(*pb_var1, in_b_l) || CARRY1(*pb_var1 + in_b_l, b_var12);
    }
    unsafe { *pb_var1 = *pb_var1 + in_b_l + b_var12 };
    pb_var1 = (unaff_b_p + unaff_s_i + 0x40c8);
    unsafe { b_var3 = *pb_var1 };
    unsafe { b_var4 = *pb_var1 + in_a_l + 2 };
    unsafe { *pb_var1 = b_var4 + b_var13 };
    pc_var2 = (unaff_d_i + -0x75);
    unsafe {
        *pc_var2 = *pc_var2 + b_var9 + (CARRY1(b_var3, in_a_l + 2) || CARRY1(b_var4, b_var13))
    };
    _in(in_d_x & 0xff00 | b_var9);
    process_struct_1040_b082((unaff_b_p + 3), CONCAT22(unaff_b_p[5], 0x1840));
    pu_var7 = (unaff_b_p + 3);
  // u_var11 = (pu_var7  >> 0x10);
    local_b_x_41 = pu_var7;
    local_b_x_41.field_0x94 = _ctx.PTR_LOOP_1050_5f0c;
    local_b_x_41.field_0x98 = 0;
    local_b_x_41.field_0x9c = 0;
    local_b_x_41.field_0x9e = 0;
    unsafe { *pu_var7 = 0xd07c };
    local_b_x_41.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pp_var14 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_00000000, 0x3e),
    );
    u_var8 = (unaff_b_p + 3);
  // u_var11 = (u_var8  >> 0x10);
    local_b_x_92 = u_var8;
    local_b_x_92.field_0x98 = pp_var14;
    local_b_x_92.field_0x9a = (pp_var14 >> 0x10);
    return;
}

pub unsafe fn pass1_1040_c71e(param_1: &mut  Struct335) {
    let local_b_x_12: &mut  Struct335;
    let mut u_var1: u16;

    process_struct_1040_9252(param_1);
  // u_var1 = (param_1  >> 0x10);
    local_b_x_12 = param_1;
    local_b_x_12.field_0x28 = local_b_x_12.field_0x24 / 2 - local_b_x_12.field_0x2c / 2;
    return;
}

pub unsafe fn pass1_1040_c54a(param_1: u32, param_2: u16, in_Struct3: &mut  Struct114) {
    let ppc_var1: fn();
    let mut i_var2: i32;
    let mut extraout_d_x: u16;
    let local_b_x_7: &mut  Struct114;
    let local_b_x_49: &mut  Struct113;
    let mut u_var3: i32;
    let in_Struct1: &mut  Struct74;
    let pa_var4: &mut  Struct114;
    let mut u_var5: u16;
    let mut u_var6: u32;

    local_b_x_7 = in_Struct3;
    i_var2 = local_b_x_7.field_0x12 + 200;
    u_var6 = 0;
    u_var5 = 0;
    ppc_var1 = (in_Struct3 + 0x14);
    pa_var4 = in_Struct3;
    (**ppc_var1)();
    in_Struct1 = param_1;
    make_proc_inst_1040_8fb8(
        in_Struct1,
        0,
        CONCAT22(extraout_d_x, i_var2),
        pa_var4,
        (pa_var4 >> 0x10),
        u_var5,
        u_var6,
        (u_var6 >> 0x10),
    );
  // u_var3 = (param_1  >> 0x10);
    (in_Struct1 + 1) = in_Struct3;
    in_Struct1[1].field_0x4 = 0;
    in_Struct1[1].field_0x6 = param_2;
    param_1 = 0xc9f2;
    in_Struct1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    process_struct_1040_c630((param_1 & 0xffff | u_var3 << 0x10));
    return;
}

pub unsafe fn pass1_1040_c5ac(param_1: &mut  Struct337) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_b_x_5: &mut  Struct337;
    let mut u_var4: u16;

  // u_var4 = (param_1  >> 0x10);
    local_b_x_5 = param_1;
    param_1 = 0xc9f2;
    local_b_x_5.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + -1;
    if (PTR_LOOP_1050_5f02 == 0x0) {
        pu_var1 = local_b_x_5.field_0x8;
        u_var2 = local_b_x_5.field_0xa;
        if ((u_var2 | pu_var1) != 0) {
            unsafe { ppc_var3 = *pu_var1 };
            (**ppc_var3)();
        }
        pu_var1 = local_b_x_5.field_0xc;
        u_var2 = local_b_x_5.field_0xe;
        if ((u_var2 | pu_var1) != 0) {
            unsafe { ppc_var3 = *pu_var1 };
            (**ppc_var3)();
        }
    }
    free_proc_inst_1040_911e(param_1);
    return;
}

pub unsafe fn pass1_1040_c60e(param_1: &mut StructuredData) -> u32 {
    let mut u_var1  = param_1.get_u32(0x42).unwrap();
    if (u_var1) != 0 {
        // return u_var1 + 0x12;
        return
    }
    return 0;
}

pub unsafe fn pass1_1040_bfde(param_1: &mut  Struct338, param_2: u32) {
    let ppc_var1: fn();
    let mut u_var2: u32;
    let local_b_x_15: &mut  Struct338;
    let mut u_var3: u16;

  // u_var3 = (param_1  >> 0x10);
    local_b_x_15 = param_1;
    local_b_x_15.field_0x6 = param_2;
    ppc_var1 = (param_2 + 4);
    (**ppc_var1)();
    u_var2 = local_b_x_15.field_0x6;
    (u_var2 + 0x22) = local_b_x_15.field_0x4;
    pass1_1010_2ee2(local_b_x_15.field_0x6);
    return;
}

pub unsafe fn pass1_1040_be94(param_1: u32, param_2: u8) {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_bb5a(param_1: u32) {
    let fn_ptr_1: fn();

    fn_ptr_1 = ((param_1 + 0x94) + 8);
    (**fn_ptr_1)();
    return 0;
}

pub unsafe fn pass1_1040_b864(
    param_1: &mut  u32,
    param_2: &mut  i32,
    param_3: u16,
    param_4: u16,
    param_5: Vec<u8>,
) {
    let ppc_var1: fn();
    let u_var2: u8;
    let extraout_a_h: u8;

    if (param_5 == (s_New_failed_in_Op__Op_1050_0020 + 0xb)) {
        let param_2_val = unsafe { *param_2 };
        if (param_2_val == 4) {
            get_prop_1040_9566(param_2, param_3);
        }
    } else {
        if (param_5 != (s_New_failed_in_Op__Op__Simulator_1050_0110 + 1)) {
            u_var2 = pass1_1040_b316(param_1, param_2, param_3, param_3_00, param_5);
            return CONCAT11(extraout_a_h, u_var2);
        }
        let param_1_val = unsafe { *param_1 };
        ppc_var1 = (param_1_val + 0x7c);
        (**ppc_var1)();
    }
    return 1;
}

pub unsafe fn pass1_1040_b74c(param_1: u32, param_2: u8) {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_b7ce(param_1: u32) {
    let pb_var1: Vec<u8>;
    let mut b_var2: u8;
    let mut c_var3: u8;
    let mut u_var4: i32;
    let mut in_c_x: u16;
    let mut b_var5: u8;
    let mut in_d_x: i32;
    let mut b_var6: u8;
    let mut in_b_x: i32;
    let mut b_var7: u8;
    let local_bx_55: &mut  Struct343;
    let pu_var8: &mut  u16;
    let unaff_bp: &mut  u16;
    let unaff_si: Vec<u8>;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut b_var11: bool;
    let mut b_var12: bool;
    let paVar13: &mut  Struct346;
    let lVar14: u32;
    let local_4e: u8;
    let puStack34: Vec<u8>;

    puStack34 = &stack0xfffe;
    pu_var8 = &stack0xfffe;
    c_var3 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var8 = pu_var8 + -1;
        unsafe { *pu_var8 = *unaff_bp };
        c_var3 = c_var3 + -1;
        '\0' < c_var3
    } {}
    b_var7 = (in_b_x >> 8);
    unaff_si[in_b_x] = b_var7;
    b_var6 = (in_d_x + 1 >> 8);
    b_var2 = b_var6 + b_var7;
    b_var11 = CARRY1(b_var6, b_var7) || CARRY1(b_var2, in_CF);
    u_var4 = in_d_x + 1 & 0xff;
    lVar14 = CONCAT22(u_var4 | (b_var2 + in_CF) << 8, in_b_x);
    paVar13 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pb_var1 = unaff_si + in_b_x;
    b_var5 = u_var4;
    unsafe { b_var2 = *pb_var1 };
    unsafe { b_var6 = *pb_var1 - b_var5 };
    unsafe { b_var12 = *pb_var1 < b_var5 || b_var6 < b_var11 };
    unsafe { *pb_var1 = b_var6 - b_var11 };
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val != 0
        && (SBORROW1(b_var2, b_var5) != SBORROW1(b_var6, b_var11)) == (pb_var1_val < '\0'))
    {
        pb_var1 = unaff_si;
        b_var11 = CARRY1(pb_var1_val, b_var7) || CARRY1(pb_var1_val + b_var7, b_var12);
        unsafe { *pb_var1 = *pb_var1 + b_var7 + b_var12 };
        b_var2 = local_4e + in_b_x;
        b_var12 = CARRY1(local_4e, in_b_x) || CARRY1(b_var2, b_var11);
        local_4e = b_var2 + b_var11;
        pb_var1 = unaff_si + -0x4f;
        unsafe { b_var2 = *pb_var1 };
        unsafe { b_var6 = *pb_var1 };
        unsafe { *pb_var1 = b_var6 + b_var7 + b_var12 };
        pb_var1 = unaff_si + -0x78;
        unsafe {
            *pb_var1 =
                *pb_var1 + in_c_x + (CARRY1(b_var2, b_var7) || CARRY1(b_var6 + b_var7, b_var12))
        };
        puStack34 = &stack0xfffe;
        process_struct_1040_b0bc(paVar13, 0, CONCAT22(in_c_x, 0xfab));
      // u_var9 = (paVar13  >> 0x10);
        local_bx_55 = paVar13;
        local_bx_55.field_0x94 = 0;
        local_bx_55.field_0x98 = 0;
        local_bx_55.field_0xb0 = 0;
        local_bx_55.field_0xb4 = 0;
        local_bx_55.field_0xb6 = 0;
        paVar13 = 0xbeba;
        local_bx_55.field_0x2 = &ctx.PTR_LOOP_1050_1040;
        if (lVar14 != 0) {
          // u_var10 = (lVar14  >> 0x10);
            local_bx_55.field_0xb0 = (lVar14 + 6);
            local_bx_55.field_0xb4 = (lVar14 + 0x14);
        }
        return;
    }
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1040_b316(
    param_1: &mut  u32,
    param_2: Vec<u8>,
    param_3: Vec<u8>,
    param_3_00: Vec<u8>,
    param_5: Vec<u8>,
) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let mut local_4: u16;

    if (param_5 == (&PTR_LOOP_1050_000e + 1)) {
        let param_1_val = unsafe { *param_1 };
        pp_var1 = (param_1_val + 0x60);
        local_4._0_1_ = (**pp_var1)();
    } else {
        if (param_5 == (s_New_failed_in_Op__Op__Simulator_1050_0110 + 1)) {
            let param_1_val = unsafe { *param_1 };
            pp_var1 = (param_1_val + 0x10);
            (**pp_var1)();
            local_4._0_1_ = 0x1;
        } else {
            u_var2 = pass1_1040_79c0(param_1, param_2, param_3, param_3_00, param_5);
            local_4._0_1_ = u_var2;
        }
    }
    return local_4;
}

pub unsafe fn pass1_1040_af9e(param_1: u32, param_2: u8) {
    pass9_funcs::pass1_1040_ace8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_b020(param_1: u32) {
    let pb_var1: Vec<u8>;
    let mut b_var2: u8;
    let mut cVar3: u8;
    let mut u_var4: i32;
    let in_cx: Vec<u8>;
    let mut b_var5: u8;
    let mut in_dx: i32;
    let mut u_var6: i32;
    let mut in_bx: i32;
    let mut b_var9: u8;
    let mut i_var8: i32;
    let pu_var10: &mut  u16;
    let unaff_bp: &mut  u16;
    let unaff_si: Vec<u8>;
    let mut u_var11: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut b_var12: bool;
    let mut b_var13: bool;
    let in_struct_1: &mut  Struct68;
    let mut u_var14: u32;
    let local_4e: u8;
    let puStack34: Vec<u8>;
    let mut b_var7: u8;

    puStack34 = &stack0xfffe;
    pu_var10 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var10 = pu_var10 + -1;
        unsafe { *pu_var10 = *unaff_bp };
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    b_var9 = (in_bx >> 8);
    unaff_si[in_bx] = b_var9;
    b_var7 = (in_dx + 1 >> 8);
    b_var2 = b_var7 + b_var9;
    b_var12 = CARRY1(b_var7, b_var9) || CARRY1(b_var2, in_CF);
    u_var4 = in_dx + 1 & 0xff;
    u_var6 = u_var4 | (b_var2 + in_CF) << 8;
    u_var14 = CONCAT22(u_var6, in_bx);
    in_struct_1 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pb_var1 = unaff_si + in_bx;
    b_var5 = u_var4;
    unsafe { b_var2 = *pb_var1 };
    unsafe { b_var7 = *pb_var1 - b_var5 };
    unsafe { b_var13 = *pb_var1 < b_var5 || b_var7 < b_var12 };
    unsafe { *pb_var1 = b_var7 - b_var12 };
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val != 0
        && (SBORROW1(b_var2, b_var5) != SBORROW1(b_var7, b_var12)) == (pb_var1_val < '\0'))
    {
        pb_var1 = unaff_si;
        b_var12 = CARRY1(pb_var1_val, b_var9) || CARRY1(pb_var1_val + b_var9, b_var13);
        unsafe { *pb_var1 = *pb_var1 + b_var9 + b_var13 };
        b_var2 = local_4e + in_bx;
        b_var13 = CARRY1(local_4e, in_bx) || CARRY1(b_var2, b_var12);
        local_4e = b_var2 + b_var12;
        pb_var1 = unaff_si + -0x4f;
        unsafe { b_var2 = *pb_var1 };
        unsafe { b_var7 = *pb_var1 };
        unsafe { *pb_var1 = b_var7 + b_var9 + b_var13 };
        pb_var1 = unaff_si + -0x78;
        unsafe {
            *pb_var1 =
                *pb_var1 + in_cx + (CARRY1(b_var2, b_var9) || CARRY1(b_var7 + b_var9, b_var13))
        };
        puStack34 = &stack0xfffe;
        process_struct_1040_7728(
            in_struct_1,
            (&ctx.PTR_LOOP_1050_0000 + 1),
            0,
            *(in_bx + 0x12),
            in_cx,
        );
      // u_var11 = (in_struct_1  >> 0x10);
        i_var8 = in_struct_1;
        (i_var8 + 0x8e) = 0;
        (i_var8 + 0x90) = u_var14;
        in_struct_1.field_0x0 = 0xb772;
        (i_var8 + 2) = &ctx.PTR_LOOP_1050_1040;
        return;
    }
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1040_b040(param_1: &mut  u16, param_2: u32, param_3: u16) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        *(param_2 + 0x12),
        param_3,
    );
  // u_var2 = (param_1  >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x8e) = 0;
    (i_var1 + 0x90) = param_2;
    unsafe { *param_1 = 0xb772 };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1040;
    return;
}
