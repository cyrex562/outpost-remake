use std::intrinsics::offset;

use crate::err_ops::error_check_1000_17ce;
use crate::other_funcs::mixed_fn_1010_830a;
use crate::pass::pass10_funcs::pass1_1040_b040;
use crate::pass::pass14_funcs::pass1_fn_1008_60e8;
use crate::string_ops::misc::fn_1008_6048;
use crate::structs::prog_structs_21::Struct74;
use crate::structs::prog_structs_23::Struct356;
use crate::structs::prog_structs_2::Struct7;
use crate::sys_ops::win_msg;
use crate::sys_structs::RECT16;
use crate::typedefs::{HANDLE16, LRESULT, WPARAM16};
use crate::util::{CONCAT12, CONCAT13, CONCAT22, SUB21};
use crate::winapi;
use crate::winapi::{FreeProcInstance16, GetClientRect16, GetProp16, MakeProcInstance16, PtInRect16, RemoveProp16};

pub unsafe fn call_win_proc_1040_a410(param_1: u16, param_2: u32, param_3: u32) {
    let win_proc: &mut Vec<u8>;
    let mut u_var1: i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_a: u16;
    let mut local_6: u32;

    local_6 = 0;
    if (param_3 == 0x19) {
        ppc_var2 = (&ctx.PTR_LOOP_1050_5ee0 + 0x34);
        local_6 = ppc_var2();
        ctx.dx_reg = (local_6 >> 0x10);
    } else {
        if (param_3 == 0x86) {
            ppc_var2 = (&ctx.PTR_LOOP_1050_5ee0 + 0x20);
            u_var5 = ppc_var2();
            return u_var5;
        }
        if (param_3 == 0x110) {
            u_var3 = &ctx.PTR_LOOP_1050_5ee0;
          // u_var5 = win_msg::send_win_msg_1040_a308(u_var3, (u_var3  >> 0x10), param_1, param_2);
            return u_var5;
        }
    }
    if (local_6 != 0) {
        return local_6 & 0xffff | ctx.dx_reg << 0x10;
    }
    u_var3 = PTR_LOOP_1050_5bc8;
    //// _var4 = (u_var3  >> 0x10);
    local_bx_122 = u_var3;
    win_proc = local_bx_122.fn_ptr_0x4;
    u_var1 = local_bx_122.field_0x6;
    if ((u_var1 | win_proc) == 0) {
        return u_var1 << 0x10;
    }
    u_var5 = CallWindowProc16(
        CONCAT22(param_2, param_1),
        (param_2 >> 0x10),
        param_3,
        (param_3 >> 0x10),
        win_proc,
    );
    return u_var5;
}

pub fn def_wn_proc_1008_9c30(param_1: u16, param_2: u32, param_3: u32) {
    def_wnd_proc_func_1008_9ce6(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, (param_2 >> 0x10)),
        CONCAT22(0x86, (param_3 >> 0x10)),
    );
    return;
}

pub fn def_wnd_proc_func_1008_9ce6(param_1: &mut Vec<u8>, param_2: u32, param_3: u32) -> LRESULT {
    let LVar1: LRESULT;

  // LVar1 = DefWindowProc16(param_2, param_3, (param_3  >> 0x10), (param_1 + 8));
    return LVar1;
}

pub unsafe fn free_proc_and_check_err_1008_3cd6(param_1: &mut Struct7, param_2: u8) -> &mut Struct7{
    free_proc_inst_1040_911e(param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(ctx, param_1);
    }
    return param_1;
}

pub unsafe fn make_proc_inst_1040_8fb8(
    in_Struct1: &mut Struct74,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: u16,
) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut hinstance: u16;
    let mut in_stack_00000006: u16;

    *_in_Struct1 = ctx.s_1_1050_389a;
    in_Struct1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    &in_Struct1.field_0x4 = 0;
    &in_Struct1.field_0x8 = 0;
    &in_Struct1.field_0xc = 0;
    &in_Struct1.field_0x10 = 0;
    in_Struct1.field_0x14 = 0;
    in_Struct1.field_0x18 = 0;
    in_Struct1.field_0x1a = param_8;
    in_Struct1.field_0x1c = param_7;
    in_Struct1.field_0x36 = 5;
    u_var1 = 0;
    in_Struct1.field_0x38 = 0;
    in_Struct1.field_0x3a = 0;
    in_Struct1.field_0x3c = 2;
    in_Struct1.field_0x3e = 0;
    in_Struct1.field_0x40 = param_2;
    *_in_Struct1 = 0x9800;
    in_Struct1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    u_var2 = in_Struct1.field_0x36;
    in_Struct1.field_0x28 = u_var2;
    in_Struct1.field_0x26 = u_var2;
    in_Struct1.field_0x2c = 0;
    in_Struct1.field_0x2a = 0;
    if ((param_6 != 0) && (param_5 != 0)) {
        in_Struct1.field_0x38 = 1;
        mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, param_6);
        in_Struct1.field_0x8 = u_var1;
        in_Struct1.field_0xa = ctx.dx_reg;
        hinstance = 0x1010;
        mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, param_5);
        in_Struct1.field_0xc = u_var1;
        in_Struct1.field_0xe = ctx.dx_reg;
        if (param_4 == 0) {
            &in_Struct1.field_0x10 = 0;
            ctx.dx_reg = ctx.dx_reg;
        } else {
            hinstance = 0x1010;
            mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, param_4);
            in_Struct1.field_0x10 = u_var1;
            in_Struct1.field_0x12 = ctx.dx_reg;
            ctx.dx_reg = ctx.dx_reg;
        }
    }
    u_var2 = in_Struct1.field_0x36;
    in_Struct1.field_0x30 = u_var2;
    in_Struct1.field_0x2e = u_var2;
    in_Struct1.field_0x32 = 0;
    if (param_3 != 0) {
        hinstance = &ctx.PTR_LOOP_1050_1008;
        pass1_fn_1008_60e8(param_3);
        in_Struct1.field_0x4 = u_var2;
        in_Struct1.field_0x6 = ctx.dx_reg;
    }
    in_Struct1.field_0x22 = 0;
    in_Struct1.field_0x1e = 0;
    in_Struct1.field_0x20 = 0;
    if (_g_proc_inst_1050_5e18 == 0) {
        _g_proc_inst_1050_5e18 =
            winapi::MakeProcInstance16(hinstance, CONCAT22(0x9684, ctx.g_h_instance_1050_038c));
    }
  ctx.PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + 1;
    return;
}

pub fn make_proc_inst_1038_cf6c(param_1: &mut u16) {
    let pu_var1: Vec<u8>;
    let pu_var2: Vec<u8>;
    let unaff_cs: HANDLE16;
    let pvVar3: &mut Vec<u8>;

  // pu_var2 = (param_1  >> 0x10);
    pu_var1 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (pu_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    (pu_var1 + 4) = 0;
    (pu_var1 + 8) = 0;
    unsafe {
        *param_1 = 0xd23e;
    }
    (pu_var1 + 2) = &ctx.PTR_LOOP_1050_1038;
  ctx.PTR_LOOP_1050_5bc8 = pu_var1;
  ctx.PTR_LOOP_1050_5bca = pu_var2;
    pvVar3 = winapi::MakeProcInstance16(unaff_cs, CONCAT22(0xd116, ctx.g_h_instance_1050_038c));
    (pu_var1 + 4) = pvVar3;
    (pu_var1 + 6) = (pvVar3 >> 0x10);
    pvVar3 = winapi::MakeProcInstance16(offset, CONCAT22(0xd01e, ctx.g_h_instance_1050_038c));
  ctx.PTR_LOOP_1050_5bcc = pvVar3;
  // PTR_LOOP_1050_5bce = (pvVar3  >> 0x10);
    return;
}

pub fn make_proc_inst_1040_a234(param_1: Vec<u8>, param_2: Vec<u8>, param_3: u16, param_4: u32) {
    let unaff_cs: HANDLE16;

    pass1_1040_b040(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, param_3_00),
        (param_3 >> 0x10),
    );
    CONCAT22(param_2, param_1) = 0xa4e8;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    if (ctx._PTR_LOOP_1050_5edc == 0) {
      ctx._PTR_LOOP_1050_5edc =
            MakeProcInstance16(unaff_cs, CONCAT22(0xa40e, ctx.g_h_instance_1050_038c));
    }
    (param_1 + 0xc) = _PTR_LOOP_1050_5edc;
  ctx.PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + 1;
  ctx.PTR_LOOP_1050_5ee0 = param_1;
  ctx.PTR_LOOP_1050_5ee2 = param_2;
    return;
}

pub fn call_win_proc_1040_9686(param_1: u16, param_2: u16, param_3_00: WPARAM16, param_3: u32) {
    let pp_var1: fn();

    let HVar2: HANDLE16;
    let HVar3: HANDLE16;
    let b_var4: bool;
    let mut in_dx: u16;
    let mut unaff_ss: u16;
    let u_var5: u8;
    let local_1a: RECT16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (in_ax >> 8);
    HVar2 = GetProp16(CONCAT13(u_var5, CONCAT12(in_ax, 0x5e7d)), param_3);
    HVar3 = GetProp16(CONCAT13(u_var5, CONCAT12(in_ax, 0x5e76)), param_3);
    _local_6 = CONCAT22(HVar2, HVar3);
    HVar2 = GetProp16(CONCAT22(in_ax, 0x5e8b), param_3);
    HVar3 = GetProp16(CONCAT22(in_ax, 0x5e84), param_3);
    _local_a = CONCAT22(HVar2, HVar3);
    if ((HVar2 | HVar3) != 0) {
        if (param_3 == 2) {
            local_12 = _local_a;
            local_e = _local_a;
            if (_local_a != 0x0) {
                pp_var1 = *_local_a;
                (**pp_var1)(offset, HVar3, HVar2, 1);
            }
        } else {
            if (param_3 == 0x201) {
                HVar2 = GetProp16(CONCAT22(in_ax, 0x5e92), param_3);
                if (HVar2 == 0) {
                    GetClientRect16(CONCAT22(unaff_ss, &local_1a), (_local_a + 0x18));
                    b_var4 = PtInRect16(CONCAT22(param_2, param_1), &local_1a);
                    if (b_var4 == 0) {
                        return;
                    }
                    fn_1008_6048(CONCAT22(in_ax, 0x5e98), in_dx, SUB21(b_var4, 0));
                    pp_var1 = (*_local_a + 0x1c);
                    (**pp_var1)(
                        &ctx.PTR_LOOP_1050_1008,
                        _local_a,
                        (_local_a >> 0x10),
                        param_2,
                        param_1,
                        param_3_00,
                    );
                    return;
                }
            } else {
                if (param_3 == 0x204) {
                    GetClientRect16(CONCAT22(unaff_ss, &local_1a), (HVar3 + 0x18));
                    b_var4 = PtInRect16(CONCAT22(param_2, param_1), &local_1a);
                    if (b_var4 == 0) {
                        return;
                    }
                    fn_1008_6048(CONCAT22(in_ax, 0x5eab), in_dx, SUB21(b_var4, 0));
                    pp_var1 = (*_local_a + 0x20);
                    (**pp_var1)(
                        8,
                        _local_a,
                        (_local_a >> 0x10),
                        param_2,
                        param_1,
                        param_3_00,
                    );
                    return;
                }
            }
        }
    }
    if (_local_6 != 0) {
        CallWindowProc16(
            CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)),
            param_3_00,
            param_3,
            param_3,
            _local_6,
        );
    }
    return;
}

pub fn free_proc_inst_1040_911e(in_struct_1: &mut Struct7) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let local_struct_1: &mut  Struct356;
    let mut u_var5: u16;

  // u_var5 = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1 = 0x9800;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    if (local_struct_1.field_0x38 != 0) {
        pu_var1 = local_struct_1.field_0x8;
        u_var2 = local_struct_1.field_0xa;
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppcVar4 = *pu_var1;
            }
            (**ppcVar4)();
        }
        pu_var1 = local_struct_1.field_0xc;
        u_var2 = local_struct_1.field_0xe;
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppcVar4 = *pu_var1;
            }
            (**ppcVar4)();
        }
        pu_var1 = local_struct_1.field_0x10;
        u_var2 = local_struct_1.field_0x12;
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppcVar4 = *pu_var1;
            }
            (**ppcVar4)();
        }
    }
    error_check_1000_17ce(local_struct_1.field_0x4);
    u_var3 = local_struct_1.field_0x14;
    SetWindowLong16(u_var3, (u_var3 >> 0x10));
    RemoveProp16(s_thisLo_1050_5e1c, local_struct_1.field_0x18);
    RemoveProp16(s_thisHi_1050_5e23, local_struct_1.field_0x18);
    RemoveProp16(s_procLo_1050_5e2a, local_struct_1.field_0x18);
    RemoveProp16(s_procHi_1050_5e31, local_struct_1.field_0x18);
    RemoveProp16(s_IsDlg_1050_5e38, local_struct_1.field_0x18);
  ctx.PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + -1;
    if (ctx.PTR_LOOP_1050_5e16 == 0x0) {
        FreeProcInstance16(CONCAT22(_g_proc_inst_1050_5e18, 0x1538));
        _g_proc_inst_1050_5e18 = 0;
    }
    in_struct_1 = ctx.s_1_1050_389a;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}
