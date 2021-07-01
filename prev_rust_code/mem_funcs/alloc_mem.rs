use crate::app_context::AppContext;
use crate::err_ops::{error_check_1000_0dc6, error_check_1000_16ee, error_check_1000_18d2, invoke_error_handler_1000_1e61};
use crate::mem_funcs;
use crate::mem_funcs::{free_mem, mem_size};
use crate::mem_funcs::mem_ops_1::{get_fn_ptr_at_address, StructuredData};
use crate::pass::pass_funcs::{pass1_fn_1000_0c32, pass1_fn_1000_25a8, pass1_fn_1000_2913, pass1_fn_1000_422a};
use crate::string_ops::misc::process_string_1000_28dc;
use crate::struct_ops::struct_ops_1;
use crate::struct_ops::struct_ops_2::{set_struct_1000_0782, set_struct_1000_09ca, struct_fn_1000_115c, struct_fn_1000_160a};
use crate::structs::prog_structs_13::Struct92;
use crate::structs::prog_structs_23::Struct210;
use crate::structs::prog_structs_24::Struct91;
use crate::structs::prog_structs_25::Struct89;
use crate::structs::prog_structs_28::Struct140;
use crate::structs::prog_structs_31::{Struct142, Struct143, Struct144};
use crate::structs::prog_structs_5::Struct93;
use crate::typedefs::{HGLOBAL16, SEGPTR};
use crate::util::{CARRY2, CONCAT12, CONCAT13, CONCAT22, SUB42};
use crate::winapi::{FatalAppExit16, FatalExit, GlobalDOSAlloc16, GlobalHandle16, GlobalLock16, GlobalPageLock16, GlobalPageUnlock16, GlobalReAlloc16};

pub unsafe fn alloc_mem_1000_010c(ctx: &mut AppContext,
                                  param_1: u16,
                                  param_2: u16,
                                  param_3: u16,
                                  param_4: u32) -> u32 {
    let mut u_var1: i32;
    // let mut in_eax: u32;
    // let mut u_var2: u16;
    // let mut i_var3: i32;
    // let mut u_var4: u16;
    // let mut unaff_cs: u16;
    let mut b_var5: u32;
    let mut alloc_addr: StructuredData = StructuredData::new();
    let mut u_var6: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut out_addr: u16;

    //// _var2 = (in_eax  >> 0x10);
    local_6 = 0;
    local_8 = 0;
  // u_var4 = (param_4  >> 0x10);
  //   i_var3 = param_4;
    if (param_4 + 0x14) == -0x4153 {
        WORD_1050_5f30 = 1;
        alloc_addr.full_address = ctx.eax_reg & 0xffff0000;
      // u_var2 = (alloc_addr  >> 0x10);
        if param_1 != 1 {
            if param_1 == 2 {
                local_4 = 2;
                // LAB_1000_016d:
                while local_6 <= param_3 && (local_6 < param_3 || (local_8 < param_2)) {
                    alloc_addr =
                        alloc_mem_1000_03c6(ctx,
                                            (ctx.s_version__d__d_1050_0012 + 8),
                                            local_4,
                                            None);
                    alloc_addr = alloc_addr & 0xffff0000;
                    if alloc_addr.full_address == 0 {
                        break;
                    }
                    u_var1 = (ctx.s_version__d__d_1050_0012 + 8);
                    b_var5 = CARRY2(local_8, u_var1 as u16);
                    local_8 = local_8 + u_var1;
                    local_6 = local_6 + b_var5;
                }
                return alloc_addr | local_8;
            }
            if param_1 == 4 {
                local_4 = 0;
                // goto LAB_1000_016d;
            }
            // goto LAB_1000_012c;
        }
        local_4 = 1;
        if (param_4 + 0x18) != 0 {}
        // goto LAB_1000_016d;
        u_var6 = CONCAT22(i_var3, 4);
    } else {
        u_var6 = 10;
    }
    invoke_error_handler_1000_1e61(ctx, unaff_cs, u_var6);
    // LAB_1000_012c:
    return CONCAT22(u_var2, 0xffff);
}

pub unsafe fn alloc_mem_1000_03c6(ctx: &mut AppContext,
    in_alloc_size: u32,
    param_2: u16,
    param_3: Option<Struct93>,
) -> StructuredData {
    let pu_var1: &mut  u32;
    let pi_var2: &mut  i32;
    let mut u_var3: u32;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut in_dx: i32;
    let mut b_var6: bool;
    let mut alloc_addr: u32;
    let mut alloc_size: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_alloc_flags: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut u_var3 = in_alloc_size + 0xfff & 0xf000;
    let mut pu_var1 = param_3.field_0x1e;
    let pu_var1_val = *pu_var1;
    u_var4 = u_var3 + pu_var1_val;
    u_var3 = in_alloc_size
        + (0xf000 < in_alloc_size)
        + param_3.field_0x20
        + CARRY2(u_var3 as u16, pu_var1_val);
    pu_var1 = param_3.field_0x28;
    b_var6 = u_var3 < pu_var1_val;
    pu_var1 = &param_3.field_0x26;
    if (b_var6)
        || (b_var6
            || u_var3 == pu_var1_val
                && (
                    u_var4 < pu_var1_val || u_var4 == pu_var1_val,
                ))
    {
        if param_2 == 3 {
            local_alloc_flags = ((-((ctx.dx_reg & 1) != 0) >> 8) & 1 | 0x20) << 8;
        } else {
            local_alloc_flags = 0x1000;
        }
        alloc_addr = alloc_mem_1000_131c(&param_3.field_0x16 | local_alloc_flags, in_alloc_size);
        alloc_addr = (alloc_addr >> 0x10);
        if alloc_addr != 0 {
            local_AX_131 = mem_size::get_mem_sz_1000_0308();
            if local_AX_131 != 0x0 {
                local_AX_131[4] = alloc_addr;
                local_AX_131[5] = alloc_addr;
                ctx.PTR_LOOP_1050_000c = param_2 | 0xcad0;
                // 0x0 = param_3;
                ctx.dos_alloc_addr_1050_0002 = ctx.g_alloc_addr_1050_1050.clone().into_bytes();
                ctx.PTR_DAT_0005_0000_1050_0004 = local_AX_131;
                (ctx.PTR_DAT_0005_0000_1050_0004 + 2) = ctx.g_alloc_addr_1050_1050.clone();
                ctx.PTR_LOOP_1050_000a = 0;
                alloc_size = mem_size::get_mem_sz_1000_1532(alloc_addr);
                if param_2 == 1 {
                    u_var5 = set_struct_1000_0782(ctx, param_3, ctx.g_alloc_addr_1050_1050);
                } else {
                    if param_2 == 3 {
                        u_var5 = struct_ops_1::set_struct_1000_05b4();
                    } else {
                        u_var5 = set_struct_1000_09ca(ctx);
                    }
                }
                unsafe { *local_AX_131 = u_var5 };
                local_AX_131[1] = 0x8000;
                pu_var1 = &param_3.field_0x1e;
                unsafe {
                    u_var3 = *pu_var1;
                    *pu_var1 = *pu_var1 + alloc_size;
                    pi_var2 = &param_3.field_0x20;
                   // pi_var2 = *pi_var2 + (alloc_size  >> 0x10) + CARRY2(u_var3, alloc_size);
                }
                return alloc_addr;
            }
            free_mem::free_mem_1000_13ce(alloc_addr);
        }
    } else {
        invoke_error_handler_1000_1e61(unaff_cs, 7, param_3);
    }
    return 0;
}

pub unsafe fn alloc_mem_1000_05e2(ctx: &mut AppContext,
                                  struct_param_1: &Struct89,
                                  param_2: u16,
                                  param_3: &Struct144) {
    let pu_var1: &mut  u32;
    let mut i_var2: i32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let mut i_var5: i32;
    let mut bVar6: bool;
    let mut vec_var7: Vec<u8>;
    let mut lVar8: u32;

    i_var2 = param_2 + (0xffeb < struct_param_1);
    while {
        vec_var7 = alloc_mem_1000_03c6(ctx,struct_param_1.field_0x1, 3, local_BX__1);
        if vec_var7 != 0 {
            return vec_var7 & 0xffff0000 | (vec_var7 + 0x14);
        }
        lVar8 = free_mem::free_mem_1000_0052();
        u_var3 = struct_param_1[0xcd].field_0xf & 0xf000;
        pu_var1 = local_BX__1.field_0x1e;

        let pu_var1_val = unsafe { *pu_var1 };
        u_var4 = u_var3 + pu_var1_val;
        u_var3 =
            i_var2 + (0xf000 < struct_param_1 + 1) + local_BX__1.field_0x20 + CARRY2(u_var3, pu_var1_val);
        pu_var1 = local_BX__1.field_0x28;
        bVar6 = u_var3 < pu_var1_val;
        ((bVar6 || u_var3 == pu_var1_val)
            && (bVar6
                || (
                    pu_var1 = local_BX__1.field_0x26,
                    u_var4 < pu_var1_val || u_var4 == pu_var1_val,
                )))
            && (lVar8 != 0
                || (
                    i_var5 = invoke_error_handler_1000_1e61(ctx,ctx.code_seg_reg, 2, local_BX__1),
                    i_var5 != 0,
                ))
    } {}
    return 0;
}

pub unsafe fn alloc_mem_1000_0670(param_1: i32, param_2: &mut  i32) -> u16 {
    let pu_var1: &mut  u32;
    let pi_var2: &mut  i32;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: u32;
    let mut in_ax: i32;
    let mut u_var8: i32;
    let mut i_var9: i32;
    let mut iVar10: i32;
    let mut u_var11: i32;
    let mut in_dx: i32;
    let in_bx: &mut  u32;
    let mut unaff_cs: u16;
    let lVar12: u32;
    let lVar13: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    i_var3 = in_bx;
    i_var4 = (in_bx + 2);
    lVar12 = mem_size::get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
    iVar10 = in_dx + (0xffeb < in_ax);
    u_var7 = unsafe { *in_bx };
    u_var8 = -((param_1 & 1) != 0) & 0x100 | -((param_1 & 4) != 0) & 0x400 | (u_var7 + 0x16);
    if ((param_2 | param_2) == 0) {
        iVar10 = alloc_mem_1000_14f2(
            u_var8 | 0x2000,
            in_ax + 0x14,
            iVar10,
            in_bx,
            &ctx.g_alloc_addr_1050_1050,
        );
        if (iVar10 == 0) {
            return 0;
        }
        local_10 = &ctx.g_alloc_addr_1050_1050;
    } else {
        i_var5 = (in_bx + 1);
        i32_var6 = (in_bx + 6);
        while {
            lVar13 = alloc_mem_1000_1408(
                u_var8 | 0x2000,
                in_ax + 0x14,
                iVar10,
                in_bx,
                &ctx.g_alloc_addr_1050_1050,
            );
          // local_10 = (lVar13  >> 0x10);
            if (lVar13 != 0) {
                break;
            }
            i_var9 = invoke_error_handler_1000_1e61(ctx,unaff_cs, 2, i_var3);
            i_var9 != 0
        } {}
        if (lVar13 == 0) {
            (param_2 + 2) = 0;
            unsafe { *param_2 = 0 };
            return 0;
        }
        (i_var5 + 8) = lVar13;
        (i_var5 + 10) = local_10;
        unsafe { *param_2 = lVar13 + 0x14 };
        (param_2 + 2) = local_10;
    }
    lVar13 = mem_size::get_mem_sz_1000_1532(local_10);
    u_var11 = (lVar13 - lVar12);
    pu_var1 = (i_var3 + 0x1e);
    unsafe { u_var8 = *pu_var1 };
    unsafe { *pu_var1 = *pu_var1 + u_var11 };
    pi_var2 = (i_var3 + 0x20);
    unsafe {// pi_var2 = *pi_var2 + ((lVar13 - lVar12)  >> 0x10) + CARRY2(u_var8, u_var11) };
    return 1;
}

pub unsafe fn alloc_mem_1000_07fc(param_1: Vec<u8>) {
    let mut unaff_cs: u16;
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0x14) != -0x4153) {
        invoke_error_handler_1000_1e61(ctx,unaff_cs, 10, 0);
        return 0;
    }
    u_var1 = alloc_mem_1000_0838(0x0);
    return u_var1;
}

pub unsafe fn alloc_mem_1000_0838(param_1: &mut Struct142) {
    let pu_var1: &mut  u32;
    Struct143 * *ppaVar2;
    let pi_var3: &mut  i32;
    let pu_var4: &mut  u16;
    let mut i_var5: i32;
    let mut u_var6: i32;
    let local_BX__1: &mut  Struct91;
    let pi_var7: &mut  i32;
    let mut unaff_cs: u16;
    let mut bVar8: bool;
    let pvVar9: &mut Vec<u8>;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_1802631433d: &mut  u16;
    let temp_5f1305d326: &mut  Struct143;

    pi_var7 = param_1.field_0x2;
    local_4 = pi_var7;
    if param_1.field_0x2 == 0x0 {}
    // goto LAB_1000_085b;
    loop {
        while {
            let pi_var7_val = unsafe { *pi_var7 };
            if pi_var7_val != 0 {
                i_var5 = pi_var7[5];
                pu_var4 = &ctx.PTR_LOOP_1050_000e;
                if pu_var4 != 0x0 {
                    &ctx.PTR_LOOP_1050_000e = unsafe { *pu_var4 };
                    pi_var3 = &ctx.PTR_LOOP_1050_000a;
                    unsafe {
                        *pi_var3 = *pi_var3 + 1;
                    }
                    param_1.field_0x2 = pi_var7;
                    return CONCAT22(i_var5, pu_var4);
                }
                unsafe { *pi_var7 = 0 };
            }
            pi_var7 = pi_var7[2];
            pi_var7 != local_4
        } {}
        // LAB_1000_085b:
        if param_1.field_0x18 == 0 {
            invoke_error_handler_1000_1e61(ctx,unaff_cs, 4, param_1);
            return 0;
        }
        u_var6 = param_1.field_0x1a;
        while (
            local_6 = u_var6,
            pvVar9 = alloc_mem_1000_03c6(ctx,local_6, 1, param_1),
            pvVar9 == 0,
        ) {
            temp_5f1305d326 = param_1.field_0x1e;
            u_var6 = param_1.field_0x20 + CARRY2(temp_5f1305d326, local_6);
            pu_var1 = &param_1.field_0x28;
            let pu_var1_val = unsafe { *pu_var1 };
            bVar8 = pu_var1_val <= u_var6;
            if (bVar8) {
                if (bVar8 && u_var6 != pu_var1_val) {
                    return 0;
                }
                ppaVar2 = &param_1.field_0x26;
                if (*ppaVar2 <= temp_5f1305d326 + local_6 && temp_5f1305d326 + local_6 != *ppaVar2)
                {
                    return 0;
                }
            }
            u_var6 = local_6 >> 1;
            if (local_6 >> 1 < param_1.field_0x18 + 0x14) {
                i_var5 = invoke_error_handler_1000_1e61(ctx,unaff_cs, 2, param_1);
                u_var6 = local_6 & 0xfffe;
                if (i_var5 == 0) {
                    return 0;
                }
            }
        }
        pi_var7 = param_1.field_0x2;
        local_4 = pi_var7[2];
    }
}

pub unsafe fn alloc_mem_1000_0a48(param_1: i32, param_2: &Struct89, param_3: u16, param_4: u32) {
    let paVar1: &mut  Struct140;
    let mut u_var2: i32;
    let in_cx: &mut  Struct92;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let local_DI_113: &mut  Struct140;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let mut u_var6: u32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var5 = (param_4  >> 0x10);
    if ((param_4 + 0x14) == -0x4153) {
        if (param_3 == 0) && (param_2 <= (s_version__d__d_1050_0012 + 6)) {
            if (param_2 == 0x0) {
                invoke_error_handler_1000_1e61(ctx,unaff_cs, 4, param_4);
                u_var6 = 0;
            } else {
                u_var6 = alloc_mem_1000_0838(0x0);
              // u_var4 = (u_var6  >> 0x10);
                local_DI_113 = u_var6;
                if (u_var6 != 0) && ((param_1 & 1) != 0) {
                    u_var2 = (s_version__d__d_1050_0012 + 6);
                    u_var3 = u_var2 >> 1;
                    while u_var3 != 0 {
                        u_var3 = u_var3 - 1;
                        paVar1 = local_DI_113;
                        local_DI_113 = local_DI_113.field_0x2;
                        paVar1 = 0;
                    }
                    if (u_var2 & 1) != 0 {
                        unsafe { *local_DI_113 = 0 };
                    }
                }
            }
        } else {
            if (param_3 == 0) && (param_2 <= (ctx.s_version__d__d_1050_0012 + 10)) {
                u_var6 = alloc_mem_1000_0b20(param_1 & 0xfffd, 0x0, in_cx, param_2);
            } else {
                u_var6 = alloc_mem_1000_05e2(ctx,param_2, param_3, 0x0);
            }
        }
        return u_var6;
    }
    invoke_error_handler_1000_1e61(ctx,unaff_cs, 10, 0);
    return 0;
}

pub unsafe fn alloc_mem_1000_0b20(
    param_1: u16,
    param_2: Struct93,
    param_3: &mut  Struct92,
    param_4: u16,
) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let paVar3: &Struct89;
    let mut u_var4: i32;
    let mut alloc_data: u16;
    let local_CX_30: &mut  Struct92;
    let pa_var5: &Struct89;
    let local_BX__1: Struct93;
    let mut u_var6: u16;
    let mut bVar7: bool;
    let mut u_var8: u32;
    let pvVar9: &mut Vec<u8>;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var2 = param_1 & 2;
    u_var4 = param_4 + 5 & 0xfffc;
    // local_CX_30 = (u_var4 - 8 & ~(-(u_var4 < 8)));
    pa_var5 = &local_CX_30.field_0x8;
    paVar3 = (&local_BX__1.field_0x0 + u_var2 * 2);
    local_14 = param_1;
    local_6 = paVar3;
    if (paVar3 == 0x0) {}
    // goto LAB_1000_0b64;
    loop {
        while {
            if (pa_var5 <= paVar3)
                && (
                    pvVar9 = pass1_fn_1000_0c32(pa_var5, None, local_14),
                    pvVar9 != 0,
                )
            {
                (&local_BX__1.field_0x0 + u_var2 * 2) = paVar3;
                return pvVar9;
            }
            paVar3 = &paVar3.field_0x4;
            paVar3 != local_6
        } {}
        // LAB_1000_0b64:
        if ((((local_14 & 2) == 0) || ((local_14 & 0x40) != 0))
            || (local_BX__1.field_0x32 == 0x0))
            || (paVar3 = (*local_BX__1.field_0x32)(), paVar3 < pa_var5)
        {
            if ((local_14 & 0x10) != 0)
                || (
                    pvVar9 = alloc_mem_1000_03c6(ctx,local_BX__1.field_0x1a, u_var2, local_BX__1),
                    pvVar9 == 0,
                )
            {
                if (local_14 & 0x20) == 0 {
                    u_var2 = &local_CX_30[1].field_0x7 & 0xf000;
                    pu_var1 = &local_BX__1.field_0x1e;
                    unsafe {
                        u_var4 = u_var2 + *pu_var1;
                        u_var2 = local_BX__1.field_0x20 + CARRY2(u_var2, *pu_var1);
                        pu_var1 = &local_BX__1.field_0x28;
                        bVar7 = u_var2 < *pu_var1;
                        if ((bVar7 || u_var2 == *pu_var1)
                            && (bVar7
                                || (
                                    pu_var1 = &local_BX__1.field_0x26,
                                    u_var4 < *pu_var1 || u_var4 == *pu_var1,
                                )))
                        {
                            u_var8 = alloc_mem_1000_05e2(ctx,pa_var5, 0, local_BX__1);
                            return u_var8;
                        }
                    }
                }
                return 0;
            }
        } else {
            local_14 = local_14 | 0x40;
        }
        paVar3 = (&local_BX__1.field_0x0 + u_var2 * 2);
        local_6 = &paVar3.field_0x4;
    }
    {}
}

pub unsafe fn alloc_mem_1000_0ed4(
    param_1: i32,
    param_2: i32,
    uparam_3: i32,
    param_4: &mut  u16,
    param_5: u16,
) -> libc::c_long {
    let pu_var1: &mut  u16;
    let p_uvar2: &mut  u16;
    let pu_var3: &mut  u16;
    let mut u_var4: i32;
    let pu_var5: &mut  u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((&ctx.PTR_LOOP_1050_000c & 0xfff8) == 0xcad0) {
        local_6 = 0x0;
        local_4 = &dos_alloc_addr_1050_0002;
        if ((param_1 & 8) == 0) {
            pu_var3 = &param_4;
        } else {
            pu_var3 = 0x0;
            unaff_ss = 0;
        }
        _local_c = CONCAT22(unaff_ss, pu_var3);
        local_8 = alloc_mem_1000_0fb8(param_1, pu_var3, unaff_ss);
        if (local_8 == 0) {
            return CONCAT22(param_5, param_4);
        }
        if ((param_1 & 8) == 0) {
            _local_c = alloc_mem_1000_0a48(param_1, param_2, param_3, local_6, local_4);
          // u_var6 = (_local_c  >> 0x10);
            pu_var3 = _local_c;
            if (_local_c != 0) {
                u_var4 = local_8 >> 1;
                pu_var5 = param_4;
                while (u_var4 != 0) {
                    u_var4 = u_var4 - 1;
                    pu_var2 = pu_var3;
                    pu_var3 = pu_var3 + 1;
                    pu_var1 = pu_var5;
                    pu_var5 = pu_var5 + 1;
                    unsafe { *pu_var2 = *pu_var1 };
                }
                u_var4 = ((local_8 & 1) != 0);
                while u_var4 != 0 {
                    u_var4 = u_var4 - 1;
                    pu_var2 = pu_var3;
                    pu_var3 = (pu_var3 + 1);
                    pu_var1 = pu_var5;
                    pu_var5 = (pu_var5 + 1);
                    unsafe { *pu_var2 = *pu_var1 };
                }
                error_check_1000_0dc6(ctx, param_4, param_5);
            }
            return _local_c;
        }
        if (param_3 | param_2) == 0 {
            return 0;
        }
        u_var6 = 5;
        u_var7 = local_6;
    } else {
        u_var6 = 0xe;
        u_var7 = 0;
    }
    invoke_error_handler_1000_1e61(ctx,unaff_cs, u_var6, u_var7);
    return 0;
}

pub unsafe fn alloc_mem_1000_0fb8(param_1: i32, param_2: &mut  u16, uparam_3: i32) -> u32 {
    let pu_var1: &mut  u16;
    let mut b_var2: u8;
    let mut in_ax: i32;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut i_var5: i32;
    let mut u_var6: i32;
    let mut in_dx: i32;
    let mut in_bx: i32;
    let pu_var7: &mut  u16;
    let mut unaff_si: u16;
    let mut unaff_DI: u16;
    let mut u_var8: u16;
    let mut unaff_cs: u16;
    let mut u_var9: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((in_dx | in_ax) == 0) {
        invoke_error_handler_1000_1e61(ctx,unaff_cs, 4, PTR_LOOP_1050_0000);
        if ((param_3 | param_2) != 0) {
            param_2[1] = 0;
            unsafe { *param_2 = 0 };
            return 0;
        }
        return 1;
    }
    b_var2 = PTR_LOOP_1050_000c & 7;
  // u_var8 = (ctx._PTR_LOOP_1050_0000  >> 0x10);
    if ((ctx.PTR_LOOP_1050_000c & 7) != 0) {
        if (b_var2 == 1) {
            u_var3 = (ctx.PTR_LOOP_1050_0000 + 0x18);
            if (in_dx != 0) {
                return u_var3;
            }
            if (in_ax <= u_var3) {
                return 0;
            }
            return u_var3;
        }
        if (b_var2 != 2) {
            if (b_var2 != 3) {
                if ((param_3 | param_2) != 0) {
                    param_2[1] = 0;
                    unsafe { *param_2 = 0 };
                    return 0;
                }
                return 1;
            }
            if ((((param_3 | param_2) != 0) && (in_dx == 0))
                && (in_ax <= (ctx.PTR_LOOP_1050_0000 + 0x1c)))
            {
                u_var9 = mem_size::get_mem_sz_1000_1284(in_bx, &ctx.g_alloc_addr_1050_1050);
                if (((u_var9 >> 0x10) == 0) && (u_var9 <= in_ax)) {
                    return u_var9;
                }
                return in_ax;
            }
            i_var5 = alloc_mem_1000_0670(param_1, param_2, param_3);
            if (i_var5 != 0) {
                return 0;
            }
            if ((param_3 | param_2) != 0) {
                return 0;
            }
            return 1;
        }
    }
    u_var3 = (in_bx + -2) & 0x7ffc;
    local_4 = u_var3 - 2;
    if ((*(in_bx + -1) & 0x80) != 0) {
        local_4 = u_var3 - 6;
    }
    if ((((in_dx == 0) && (in_ax <= local_4))
        || (in_dx == 0 && (in_ax <= (ctx.PTR_LOOP_1050_0000 + 0x1c))))
        && (
            u_var4 = struct_fn_1000_115c(unaff_si, unaff_DI),
            u_var4 != 0,
        ))
    {
        if ((param_1 & 1) != 0) {
            u_var3 = ((in_bx + -2) & 0x7ffc) - 2;
            if (local_4 < in_ax) {
                pu_var7 = (local_4 + in_bx);
                i_var5 = -local_4;
            } else {
                if (u_var3 <= in_ax) {
                    return 0;
                }
                pu_var7 = (in_ax + in_bx);
                i_var5 = -in_ax;
            }
            u_var3 = u_var3 + i_var5;
            u_var6 = u_var3 >> 1;
            while (u_var6 != 0) {
                u_var6 = u_var6 - 1;
                pu_var1 = pu_var7;
                pu_var7 = pu_var7 + 1;
                unsafe {
                    *pu_var1 = 0;
                }
            }
            if ((u_var3 & 1) != 0) {
                unsafe {
                    *pu_var7 = 0;
                }
            }
        }
        return 0;
    }
    return local_4;
}

pub fn alloc_mem_1000_131c(alloc_options: u32, mut alloc_size_1: u32, mut alloc_size_2: i32) {
    let handle: HGLOBAL16;
    let mut flags: i32;
    let mut bVar1: bool;
    let mut memory_block_byte: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    let mut dos_alloc_addr = CONCAT22(local_8, local_a);
    // GMEM_NODISCARD, GMEM_NOCOMPACT, GMEM_MOVEABLE
    flags = 0x32;
    local_6 = 1;
    if ((alloc_options & 0x1000) != 0) && (alloc_size_2 != 0 || (0xfff0 < alloc_size_1)) {
        alloc_size_1 = 0xfff0;
        alloc_size_2 = 0;
    }
    if (alloc_options & 0x100) != 0 {
        flags = 0x72;
    }
    if (alloc_options & 1) != 0 {
        // mark memory shared
        flags = flags | 0x2000;
    }
    if (alloc_options & 4) != 0 {
        flags = flags & 0xfffd;
        dos_alloc_addr = alloc_mem_1000_1558();
    }
    while {
        handle = GlobalAlloc16(CONCAT22(alloc_size_2, alloc_size_1), flags);
        if (handle != 0) {
            break;
        }
        flags = flags & 0xffcf;
        bVar1 = local_6 != 0;
        local_6 = local_6 - 1;
        bVar1
    } {}
    if ((alloc_options & 4) != 0) {
        if (handle != 0) {
            GlobalPageLock(handle);
        }
        free_mem::free_mem_1000_15ce(dos_alloc_addr);
    }
    if (handle == 0) {
        return 0;
    }
    memory_block_byte = GlobalLock16(handle);
    return memory_block_u8;
}

pub fn alloc_mem_1000_1408(
    param_1: i32,
    mut param_2: i32,
    mut param_3: i32,
    param_4: u16,
    param_5: u16,
) -> SEGPTR {
    let handle: HGLOBAL16;
    let handle_00: HGLOBAL16;
    let mut bVar1: bool;
    let dVar2: u32;
    let SVar3: SEGPTR;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    dVar2 = GlobalHandle16(param_4);
    handle_00 = dVar2;
    local_8 = 0x32;
    local_c = 1;
    if ((param_1 & 0x1000) != 0) && (param_3 != 0 || (0xfff0 < param_2)) {
        param_2 = 0xfff0;
        param_3 = 0;
    }
    if (param_1 & 0x100) != 0 {
        local_8 = 0x72;
    }
    if (param_1 & 0x804) != 0 {
        local_8 = local_8 & 0xfffd;
    }
    if handle_00 != 0 {
        if (param_1 & 4) != 0 {
            GlobalPageUnlock16();
        }
        while {
            handle = GlobalReAlloc16(local_8, CONCAT22(param_3, param_2), handle_00);
            if handle != 0 {
                break;
            }
            local_8 = local_8 & 0xffcf;
            bVar1 = local_c != 0;
            local_c = local_c - 1;
            bVar1
        } {}
        if (handle != 0) && ((param_1 & 4) != 0) {
            GlobalPageLock16(handle);
        }
        if handle != 0 {
            SVar3 = GlobalLock16(handle);
            return SVar3;
        }
        handle_00 = 0;
    }
    return handle_00;
}

pub fn alloc_mem_1000_14f2(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: u16,
    param_5: u16,
) -> u16 {
    let lVar1: u32;

    if ((param_1 & 0x1000) != 0) || (param_3 == 0 && (param_2 < 0xfff1)) {
        lVar1 = alloc_mem_1000_1408(param_1 & 0xfdff | 0x800, param_2, param_3, param_4, param_5);
        if lVar1 != 0 {
            return 1;
        }
    }
    return 0;
}

pub fn alloc_mem_1000_1558(ctx: &mut AppContext) -> u32 {
    let mut u_var1: u16;
    let mut alloc_addr_cpy: u32;
    let d_var2: u32;
    let mut out_alloc_addr: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    out_alloc_addr = 0;
    local_a = 0;
    local_8 = 8;
    if (ctx.dx_reg < 9) && (ctx.dx_reg < 8 || (ctx.ax_reg == 0)) {
        while {
            loop {
                d_var2 = GlobalDOSAlloc16(CONCAT22(local_8, local_a));
                alloc_addr_cpy = d_var2;
                if alloc_addr_cpy == 0 {
                    break;
                }
                0x0 = 0;
                ctx.dos_alloc_addr_1050_0002 = out_alloc_addr;
                out_alloc_addr = alloc_addr_cpy;
            }
            u_var1 = local_8 & 1;
            local_8 = local_8 >> 1;
            local_a = local_a >> 1 | (u_var1 != 0) << 0xf;
            (ctx.dx_reg < local_8) || (ctx.dx_reg <= local_8 && (ctx.ax_reg <= local_a))
        } {}
    }
    return out_alloc_addr << 0x10;
}

pub unsafe fn alloc_mem_1000_167a(ctx: &mut AppContext, param_1: u16, param_2: u16) -> Vec<u8> {
    let mut u_var1: i32;
    let mut mem_buf: u16;

    if (ctx.g_u16_ptr_1050_5f2e | ctx.g_struct_ptr_1) == 0 {
        u_var1 = struct_fn_1000_160a(ctx);
        if (param_2 | u_var1) == 0 {
            return 0;
        }
    }
    mem_buf = alloc_mem_1000_0a48(0, param_1, 0, g_struct_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    return mem_buf;
}

pub unsafe fn alloc_mem_1000_16aa(param_1: i32, param_2: i32, param_3: u16) -> &mut  libc::c_void {
    let pvVar1: &mut Vec<u8>;
    let mut in_dx: u16;

    if (param_2 | param_1) == 0 {
        pvVar1 = alloc_mem_1000_167a(ctx,param_3, in_dx);
        return pvVar1;
    }
    if param_3 == 0 {
        error_check_1000_16ee(ctx,param_1, param_2);
        return 0;
    }
    pvVar1 = alloc_mem_1000_0ed4(0, param_3, 0, param_1, param_2);
    return pvVar1;
}

pub unsafe fn alloc_mem_1000_1708(
    param_1: &mut Struct89,
    param_2: &mut i32,
    uparam_3: i32,
    param_4: i32,
    uparam_5: i32,
) -> u32 {
    let mut iVar1: i32;
    let mut b_var2: bool;
    let lVar3: u32;
    let mut local_4: u16;

    if (*param_2 | *param_1) == 0 {
        b_var2 = 0xfffe < *param_1;
        *param_1 = *param_1 + 1;
        *param_2 = *param_2 + b_var2;
    }
    // LAB_1000_1724:
    loop {
        if (param_5 | param_4) != 0 {
            lVar3 = alloc_mem_1000_0a48(param_3, param_1, param_2, param_4, param_5);
            if lVar3 != 0 {
                return lVar3;
            }
        }
        if ((param_3 & 0x8000) == 0) || ((ctx.PTR_LOOP_1050_5f3a | ctx.func_ptr_1050_5f38) == 0) {
            if (ctx.PTR_LOOP_1050_5f36 | ctx.func_ptr_1050_5f34) == 0 {
                if (ctx.PTR_LOOP_1050_5f3e | ctx.func_ptr_1050_5f3c) == 0 {
                    return 0;
                }
                get_fn_ptr_at_address(ctx.func_ptr_1050_5f3c)();
                // goto LAB_1000_1724;
            }
            // iVar1 = get_fn_ptr_at_address(ctx.func_ptr_1050_5f34)();
        } else {
            iVar1 = (*func_ptr_1050_5f38)();
        }
        if iVar1 == 0 {
            return 0;
        }
    }
}

pub unsafe fn alloc_mem_1000_180c(param_1: u16) -> u16 {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let mut in_dx: i32;

    if ((ctx.g_u16_ptr_1050_5f2e | g_struct_ptr_1) == 0) {
        u_var1 = struct_fn_1000_160a();
        if ((in_dx | u_var1) == 0) {
            return 0;
        }
    }
    u_var2 = alloc_mem_1000_0a48(0, param_1, 0, g_struct_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    return u_var2;
}

pub unsafe fn alloc_mem_1000_183c(param_1: i32, param_2: i32) -> u16 {
    let mut u_var1: u16;
    let pu_var2: &mut  u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = (param_2 * param_1);
    pu_var2 = 0x0;
    if ((param_2 * param_1 >> 0x10) != 0) {
        return 0;
    }
    if (((ctx.g_u16_ptr_1050_5f2e | g_struct_ptr_1) == 0)
        && (
            g_struct_ptr_1 = struct_fn_1000_160a(ctx,u_var1, 0),
            ctx.g_u16_ptr_1050_5f2e = pu_var2,
            (pu_var2 | g_struct_ptr_1) == 0,
        ))
    {
        return 0;
    }
    u_var1 = alloc_mem_1000_0a48(1, u_var1, 0, g_struct_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    return u_var1;
}

pub unsafe fn alloc_mem_1000_188e(param_1: i32, param_2: i32, param_3: i32) -> u16 {
    let mut u_var1: u16;

    if ((param_2 | param_1) == 0) {
        u_var1 = alloc_mem_1000_180c(param_3);
        return u_var1;
    }
    if (param_3 == 0) {
        error_check_1000_18d2(ctx,param_1, param_2);
        return 0;
    }
    u_var1 = alloc_mem_1000_0ed4(0, param_3, 0, param_1, param_2);
    return u_var1;
}

pub unsafe fn alloc_mem_1000_3c51() {
    let piVar1: &mut  i32;
    let pc_var2: String;
    let handle: HGLOBAL16;
    let mut i_var3: i32;
    let piVar4: &mut  i32;
    let in_ax: HGLOBAL16;
    let HVar5: HGLOBAL16;
    let mut in_cx: i32;
    let mut in_bx: i32;
    let pi_var6: &mut  i32;
    let pcVar7: String;
    let dVar8: u32;
    let in_stack_00000000: HGLOBAL16;
    let mut in_stack_00000008: i32;
    let pc_var9: String;

    if ((*(in_bx + 2) & 4) == 0) {
        handle = (in_bx + 6);
        HVar5 = GlobalReAlloc16(0x20, CONCAT12(in_ax == 0, in_ax), handle);
        if (HVar5 != 0) {
            if ((HVar5 != handle)
                || (
                    HVar5 = in_stack_00000000,
                    dVar8 = GlobalSize16(handle),
                    dVar8 == 0,
                ))
            {}
            // goto LAB_1000_289a;
            in_ax = HVar5;
            if ((*(in_cx + 2) & 4) != 0) {
                in_ax = HVar5 - 1;
                (in_cx + -2) = in_ax;
            }
        }
        return CONCAT22(in_ax, HVar5);
    }
    // LAB_1000_289a:
    pc_var9 = s_version__d__d_1050_0012;
    pass1_fn_1000_25a8();
    pass1_fn_1000_2913();
    pc_var9 = process_string_1000_28dc(ctx,pc_var9);
    if (pc_var9 != 0x0) {
        i_var3 = 9;
        if (unsafe { *pc_var9 } == 'M') {
            i_var3 = 0xf;
        }
        pc_var9 = pc_var9 + i_var3;
        i_var3 = 0x22;
        pcVar7 = pc_var9;
        while {
            if (i_var3 == 0) {
                break;
            }
            i_var3 = i_var3 + -1;
            pc_var2 = pcVar7;
            pcVar7 = pcVar7 + 1;
            let pc_var2_val = unsafe { *pc_var2 };
            pc_var2_val != '\r'
        } {}
        pcVar7[-1] = '\0';
    }
    FatalAppExit16(CONCAT22(0x1050, pc_var9), 0);
    FatalExit(0xff);
    pi_var6 = (s___NMSG___1050_63f6 + 8);
    loop {
        piVar1 = pi_var6;
        pi_var6 = pi_var6 + 1;
        unsafe { i_var3 = *piVar1 };
        piVar4 = pi_var6;
        if ((i_var3 == in_stack_00000008) || (piVar4 = (i_var3 + 1), piVar4 == 0x0)) {
            return CONCAT22(in_stack_00000008, piVar4);
        }
        i_var3 = -1;
        while {
            if (i_var3 == 0) {
                break;
            }
            i_var3 = i_var3 + -1;
            piVar1 = pi_var6;
            pi_var6 = (pi_var6 + 1);
            let pi_var1_val = unsafe { *piVar1 };
            pi_var1_val != '\0'
        } {}
    }
}

pub unsafe fn alloc_mem_func_1000_40af(param_1: i32, param_2: i32, uparam_3: i32) -> &mut  u16 {
    let pu_var1: &mut  u16;
    let mut u_var2: u16;
    let pcVar3: String;
    let pu_var4: &mut  u16;
    let pu_var5: &mut  u16;
    let mut u_var6: i32;
    let mut handle: i32;
    let handle_00: HGLOBAL16;
    let mut i_var7: i32;
    let mut unaff_si: i32;
    let puVar8: &mut  u16;
    let pc_var9: String;
    let unaff_DI: HGLOBAL16;
    let pu_var10: &mut  u16;
    let unaff_cs: u8;
    let mut bVar11: bool;
    let SVar12: SEGPTR;
    let u_var13: u8;
    let pc_var14: String;
    let mut local_6: u16;

    loop {
        u_var6 = (param_1 * param_3);
      // handle = param_2 * param_3 + (param_1 * param_3  >> 0x10);
        if ((handle | u_var6) != 0) {
            puVar8 = 0x0;
            if ((handle < 3) && (handle < 2 || (u_var6 == 0))) {
                if (handle == 0) {
                    u_var6 = u_var6 + 0xfff & 0xf000;
                    if (u_var6 == 0) {
                        handle = 1;
                    }
                } else {
                    if ((param_3 - 1 & param_3) != 0) {
                        puVar8 = ((handle << 0x10) % param_3);
                        bVar11 = CARRY2(u_var6, puVar8);
                        u_var6 = u_var6 + puVar8;
                        if (bVar11) {}
                        // goto LAB_1000_41aa;
                        handle = 1;
                    }
                }
            } else {
                if ((param_3 - 1 & param_3) != 0) {}
                // goto LAB_1000_41aa;
            }
            handle_00 = GlobalAlloc16(CONCAT13((handle >> 8), CONCAT12(handle, u_var6)), 0x20);
            if ((handle_00 != 0) && ((u_var6 & 1) != 0)) {
                SVar12 = GlobalLock16(handle_00);
                if ((SVar12 != 0) || (SVar12 == 0)) {
                    pc_var14 = s_version__d__d_1050_0012;
                    u_var13 = 0x12;
                    pass1_fn_1000_25a8(ctx,0x12);
                    pass1_fn_1000_2913(ctx,u_var13);
                    pc_var14 = process_string_1000_28dc(ctx,pc_var14);
                    if (pc_var14 == 0x0) {}
                    // goto LAB_1000_28cb;
                    i_var7 = 9;
                    let pc_var14_val = unsafe { *pc_var14 };
                    if (pc_var14_val == 'M') {
                        i_var7 = 0xf;
                    }
                    pc_var14 = pc_var14 + i_var7;
                    i_var7 = 0x22;
                    pc_var9 = pc_var14;
                    break;
                }
              // handle_00 = pass1_fn_1000_422a((SVar12  >> 0x10), handle_00);
                if (handle_00 == 0) {
                    GlobalUnlock16(handle);
                    GlobalFree16(unaff_DI);
                    handle_00 = 0;
                }
            }
            unaff_cs = 0x38;
            if (handle_00 != 0) {
                pu_var10 = 0x0;
                while (unaff_si != 0) {
                    i_var7 = -0x8000;
                    while (i_var7 != 0) {
                        i_var7 = i_var7 + -1;
                        pu_var4 = pu_var10;
                        pu_var10 = pu_var10 + 1;
                        unsafe { *pu_var4 = 0 };
                    }
                    handle_00 = handle_00 + 0x100;
                    unaff_si = unaff_si + -1;
                }
                if (unaff_DI != 0) {
                    while (unaff_DI != 0) {
                        unaff_DI = unaff_DI - 1;
                        pu_var4 = pu_var10;
                        pu_var10 = (pu_var10 + 1);
                        unsafe { *pu_var4 = 0 };
                    }
                }
                return puVar8;
            }
        }
        // LAB_1000_41aa:
        if ((ctx.PTR_LOOP_1050_618e | ctx.PTR_LOOP_1050_618c) == 0) {
            return 0x0;
        }
        i_var7 = (*ctx.PTR_LOOP_1050_618c)(unaff_cs, param_3, param_1, param_2);
        if (i_var7 == 0) {
            return 0x0;
        }
    }
    while (true) {
        i_var7 = i_var7 + -1;
        pcVar3 = pc_var9;
        pc_var9 = pc_var9 + 1;
        let pc_var3_val = unsafe { *pcVar3 };
        if (pc_var3_val == '\r') {
            break;
        }
        if (i_var7 == 0) {
            break;
        }
    }
    pc_var9[-1] = '\0';
    // LAB_1000_28cb:
    FatalAppExit16(CONCAT13(0x10, CONCAT12(0x50, pc_var14)), 0);
    FatalExit(0xff);
    puVar8 = (s___NMSG___1050_63f6 + 8);
    loop {
        pu_var1 = puVar8;
        puVar8 = puVar8 + 1;
        unsafe { u_var2 = *pu_var1 };
        pu_var5 = puVar8;
        if ((u_var2 == local_6) || (pu_var5 = (u_var2 + 1), pu_var5 == 0x0)) {
            return pu_var5;
        }
        i_var7 = -1;
        while {
            if (i_var7 == 0) {
                break;
            }
            i_var7 = i_var7 + -1;
            pu_var1 = puVar8;
            puVar8 = (puVar8 + 1);
            let pi_var1_val = unsafe { *pu_var1 };
            pi_var1_val != '\0'
        } {}
    }
}

pub unsafe fn alloc_mem_1008_909c(struct_param_1: &mut Struct210) {
    let pu_var1: &mut  u32;
    let mut var2: &mut Struct89;
    // let mut i_var3: i32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

  // local_es_4 = (in_struct_1  >> 0x10);
  //   i_var3 = in_struct_1;
    if (struct_param_1.field_0x12) == 0 {
        local_AX_23 = (i_var3 + 0xe);
        ctx.g_u16_ptr_1050_5f2e = (i_var3 + 0x10);
    } else {
        var2 = &mut struct_param_1.field_0x12;
        pu_var1 = &mut struct_param_1.field_0x16;
        let pu_var1_val = *pu_var1;
        local_AX_23 = (var2 + pu_var1_val);
        ctx.g_u16_ptr_1050_5f2e = ((struct_param_1 + 0x14) + (struct_param_1 + 0x18) + CARRY2(var2, pu_var1_val));
    }
    _local_6 = CONCAT22(ctx.g_u16_ptr_1050_5f2e, local_AX_23);
    if (i_var3 + 6) == 0 {
        if ctx.g_struct_ptr_1 == 0 {
            ctx.g_struct_ptr_1 = local_AX_23;
            struct_fn_1000_160a();
        } else {
        }
        var2 = local_AX_23 << 2;
        alloc_mem_1000_1708(var2, 0, 1, g_struct_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    } else {
        var2 = local_AX_23 * 4;
        // alloc_mem_1000_0ed4(
        //     1,
        //     u_var2,
        //     (ctx.g_u16_ptr_1050_5f2e * 2 + CARRY2(local_AX_23, local_AX_23)) * 2
        //         + CARRY2(local_AX_23 * 2, local_AX_23 * 2),
        //     (i_var3 + 6),
        // );
        ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
    }
    local_a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, var2 as u16);
    if (ctx.g_u16_ptr_1050_5f2e | var2) != 0 {
        (i_var3 + 0x12) = _local_6;
        (i_var3 + 6) = local_a as i32;
    }
    return;
}
