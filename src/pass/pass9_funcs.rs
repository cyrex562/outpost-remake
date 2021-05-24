use crate::err_ops::error_check_1000_17ce;
use crate::pass::{pass11_funcs, pass14_funcs, pass_funcs};
use crate::pass::pass17_funcs::pass1_1030_8326;
use crate::pass::pass6_funcs::pass1_1038_b6e0;
use crate::pass::pass8_funcs::pass1_1008_e9a4;
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1040_b082};
use crate::structs::prog_structs_2::{Struct199, Struct306};
use crate::structs::prog_structs_21::Struct350;
use crate::structs::prog_structs_24::Struct2111;
use crate::structs::prog_structs_31::{Struct305, Struct348};
use crate::sys_ops::win::win_cleanup_func_1040_b0f8;
use crate::util::{CARRY1, CONCAT22, SBORROW1};

pub unsafe fn pass1_1040_ac64(param_1: u32) {
    byte * *ppb_var1;
    let pu8_var2: Vec<u8>;
    let mut b_var3: u8;
    let mut cVar4: u8;
    let mut b_var5: u8;
    let mut in_CL: u8;
    let mut in_dx: i32;
    let mut b_var6: u8;
    let mut in_bx: i32;
    let mut b_var8: u8;
    let mut i_var7: i32;
    let pu_var9: &mut  u16;
    let unaff_bp: &mut  u16;
    let unaff_si: Vec<u8>;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut b_var11: bool;
    let mut b_var12: bool;
    let ppVar13: &mut  Struct2111;
    let pu_var14: Vec<u8>;
    let pu_var15: &mut  u16;
    let local_4e: u8;
    let puStack34: Vec<u8>;

    puStack34 = &stack0xfffe;
    pu_var9 = &stack0xfffe;
    pu_var14 = &stack0xfffe;
    cVar4 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var9 = pu_var9 + -1;
        unsafe { *pu_var9 = *unaff_bp };
        cVar4 = cVar4 + -1;
        '\0' < cVar4
    } {}
    b_var8 = (in_bx >> 8);
    unaff_si[in_bx] = b_var8;
    b_var6 = ((in_dx + 1) >> 8);
    b_var11 = CARRY1(b_var6, b_var8) || CARRY1(b_var6 + b_var8, in_CF);
    b_var5 = (in_dx + 1);
    pu_var15 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pu8_var2 = unaff_si + in_bx;
    unsafe { b_var6 = *pu8_var2 };
    unsafe { b_var3 = *pu8_var2 - b_var5 };
    unsafe { b_var12 = *pu8_var2 < b_var5 || b_var3 < b_var11 };
    unsafe { *pu8_var2 = b_var3 - b_var11 };
    let pu8_var2_val = unsafe { *pu8_var2 };
    if (pu8_var2_val != 0
        && (SBORROW1(b_var6, b_var5) != SBORROW1(b_var3, b_var11)) == (pu8_var2_val < '\0'))
    {
        pu8_var2 = unaff_si;
        b_var11 = CARRY1(pu8_var2_val, b_var8) || CARRY1(pu8_var2_val + b_var8, b_var12);
        unsafe { *pu8_var2 = *pu8_var2 + b_var8 + b_var12 };
        b_var6 = local_4e + in_bx;
        b_var12 = CARRY1(local_4e, in_bx) || CARRY1(b_var6, b_var11);
        local_4e = b_var6 + b_var11;
        pu8_var2 = unaff_si + -0x4f;
        unsafe { b_var6 = *pu8_var2 };
        unsafe { b_var3 = *pu8_var2 };
        unsafe { *pu8_var2 = b_var3 + b_var8 + b_var12 };
        pu8_var2 = unaff_si + -0x78;
        unsafe {
            *pu8_var2 =
                *pu8_var2 + in_CL + (CARRY1(b_var6, b_var8) || CARRY1(b_var3 + b_var8, b_var12))
        };
        puStack34 = &stack0xfffe;
        process_struct_1040_b082(CONCAT22(&stack0xbf2a, &stack0xfffe), CONCAT22(in_bx, 499));
      // u_var10 = (pu_var15  >> 0x10);
        i_var7 = pu_var15;
        (i_var7 + 0x94) = 0;
        (i_var7 + 0x98) = 0;
        unsafe { *pu_var15 = 0xafc4 };
        (i_var7 + 2) = &ctx.PTR_LOOP_1050_1040;
        (i_var7 + 0x94) = _PTR_LOOP_1050_5ef0;
        _PTR_LOOP_1050_5ef0 = 0;
        ppVar13 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(pu_var14, 0x3d));
      // u_var10 = (pu_var15  >> 0x10);
        (pu_var15 + 0x98) = ppVar13;
        (pu_var15 + 0x9a) = (ppVar13 >> 0x10);
        return;
    }
    ppb_var1 = (unaff_si + 9);
    unsafe { *ppb_var1 = unaff_si + *ppb_var1 };
    error_check_1000_17ce(param_1);
    return;
}

pub unsafe fn pass1_1040_ac84(param_1: &mut  Struct350, param_2: u16) {
    let local_bx_18: &mut  Struct350;
    let mut unaff_bp: u16;
    let mut u_var1: u16;
    let ppVar2: &mut  Struct2111;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 499));
  // u_var1 = (param_1  >> 0x10);
    local_bx_18 = param_1;
    local_bx_18.field_0x94 = 0;
    &local_bx_18.field_0x98 = 0;
    param_1 = 0xafc4;
    local_bx_18.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    local_bx_18.field_0x94 = _PTR_LOOP_1050_5ef0;
    _PTR_LOOP_1050_5ef0 = 0;
    ppVar2 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_bp, 0x3d));
    local_bx_18.field_0x98 = ppVar2;
    local_bx_18.field_0x9a = (ppVar2 >> 0x10);
    return;
}

pub unsafe fn pass1_1040_ace8(in_struct_1: &mut  Struct348) {
    let local_struct_1: &mut  Struct348;
    let mut u_var1: u16;

  // u_var1 = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.field_0x0 = 0xafc4;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.g_struct_112_001, local_struct_1.field_0x6);
    win_cleanup_func_1040_b0f8(in_struct_1);
    return;
}

pub unsafe fn pass1_1008_de58(param_1: u32, param_2: u32, param_3: u32) {
    let pp_var1: fn();
    let mut b_var2: bool;
    let local_AX_39: &mut  Struct305;

    let struct_a: &mut  Struct199;

    let paVar3: &mut  Struct305;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

  // u_var4 = (param_1  >> 0x10);
    pass14_funcs::pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    b_var2 = false;
    while {
        local_AX_39 = local_a;
        pass14_funcs::pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_39));
        struct_a = (ctx.dx_reg | local_AX_39);
        paVar3 = local_AX_39;
        if (struct_a == 0x0) {}
        // goto LAB_1008_dedb;
        ((local_AX_39.field_0x4 != param_3) || (local_AX_39.field_0x8 != param_2))
            && (local_AX_39.field_0x8 != param_3 || (local_AX_39.field_0x4 != param_2))
    } {}
    local_AX_39.field_0xc = 1;
    u_var6 = pass1_1030_8326();
  // struct_a = (u_var6  >> 0x10);
    paVar3 = u_var6;
    local_AX_39.field_0xe = paVar3;
    local_AX_39.field_0x10 = struct_a;
    b_var2 = true;
    // LAB_1008_dedb:
    if (!b_var2) {
        process_struct_1000_179c(0x14, struct_a);
        if ((struct_a | paVar3) == 0) {
            paVar3 = 0x0;
            u_var5 = 0;
        } else {
            pass11_funcs::pass1_1008_dc90(CONCAT22(struct_a, paVar3), param_2, param_3);
            u_var5 = ctx.dx_reg;
        }
        paVar3.field_0xc = 1;
        u_var6 = pass1_1030_8326();
        paVar3.field_0xe = u_var6;
        paVar3.field_0x10 = (u_var6 >> 0x10);
        pp_var1 = ((param_1 + 10) + 4);
        (**pp_var1)();
    }
    return;
}

pub unsafe fn pass1_1008_df4a(param_1: u32) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut u_var3: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

  // u_var2 = (param_1  >> 0x10);
    pass14_funcs::pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    while (true) {
        u_var3 = pass14_funcs::pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
      // u_var1 = (u_var3  >> 0x10);
        if (u_var3 == 0) {
            break;
        }
        if (((u_var3 + 0xc) == 2) || ((u_var3 + 0xc) == 3)) {
            pass1_1008_e9a4(param_1, u_var2, u_var3);
        }
    }
    return;
}

pub unsafe fn pass1_1008_dfa6(param_1: u32, param_2: u32, param_3: u32) {
    let local_AX_37: &mut  Struct306;

    let mut unaff_ss: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    pass14_funcs::pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    while {
        local_AX_37 = local_a;
        pass14_funcs::pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_37));
        if ((ctx.dx_reg | local_AX_37) == 0) {
            return;
        }
        ((local_AX_37.field_0x4 != param_3) || (local_AX_37.field_0x8 != param_2))
            && (local_AX_37.field_0x8 != param_3 || (local_AX_37.field_0x4 != param_2))
    } {}
    if (local_AX_37.field_0xc != 1) {
        return;
    }
    return;
}

pub unsafe fn pass1_1008_e01c(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    (param_1 + 0x16) = param_3;
    (param_1 + 0x1a) = param_2;
    return;
}

pub unsafe fn pass1_1008_e038(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    param_3 = (param_1 + 0x16);
    param_2 = (param_1 + 0x1a);
    return;
}
