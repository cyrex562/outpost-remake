use crate::err_funcs::invoke_error_handler_1000_1e61;

pub enum AddressType {
    NotSet,
    Invalid,
    Unknown,
    Struct112,
}

pub struct Address<T> {
    pub full_addr: u32,
    pub base: u16,
    pub offset: u16,
    pub len: usize,
    pub buffer: Vec<u8>,
    pub _type: T
}

impl Address<T> {
    pub fn new() -> Address<T> {
        Address {
            full_addr: 0,
            base: 0,
            offset: 0,
            len: 0,
            buffer: Vec::new(),
            _type: T,
        }
    }
}

// u32  free_mem_1000_0016(param_1: u32)
pub fn free_mem_1000_0016(param_1: u32) -> u32 {
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0x14) != -0x4153) {
        invoke_error_handler_1000_1e61(ctx, ctx.code_seg_reg, 10, 0);
        return 0xffffffff;
    }
    u_var1 = free_mem_1000_0052(&ctx.g_alloc_addr_1050_1050);
    return u_var1;
}




// u32  free_mem_1000_0052()
pub fn free_mem_1000_0052(param_1: *mut u32) -> u32 {
    // pu_var1: *mut u32;
    let mut pu_var1: u32;
    // let mut u_var2: i32;
    let mut u_var2: u32;
    // let mut i_var3: i32;
    let mut i_var3: i32;
    // let mut u_var4: u32;
    let mut u_var4: u32;
    // let b_var5: bool;
    let mut b_var5: bool;
    // local_BX__1: *mut Struct86;
    let mut local_BX__1: Struct86;
    // let mut local_16: u16;
    let mut local_16: u16;
    // let mut local_14: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let local_a: *mut Struct87;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = local_BX__1.field_0x1e;
    i_var3 = local_BX__1.field_0x20;
    local_8 = 0;
    while {
        local_a = (&local_BX__1.field_0x0 + local_8 * 2);
        if ((local_a != 0x0) && (local_8 != 3)) {
            local_e = 0;
            while {
                local_c = local_a.field_0x4;
                u_var4 = local_a.field_0x8;
                if ((u_var4 + 10) == 0) {
                    b_var5 = free_mem_1000_0510();
                    if (b_var5 == 0) {}
                    // goto LAB_1000_00f9;
                    if (local_c == local_a) {
                        local_c = 0;
                    }
                } else {
                    if (local_e == 0) {
                        local_e = local_a;
                    }
                }
                local_a = local_c;
                local_c != local_e
            } {}
        }
        local_8 = local_8 + 1;
        local_8 < 5
    } {}
    if (local_BX__1.field_0x32 != 0x0) {
        (*local_BX__1.field_0x32)();
    }
    // LAB_1000_00f9:
    pu_var1 = &local_BX__1.field_0x1e;
    return CONCAT22(
        (i_var3 - local_BX__1.field_0x20) - (u_var2 < *pu_var1),
        u_var2 - *pu_var1,
    );
}

pub fn alloc_mem_1000_010c(param_1: u16, param_2: u16, param_3: u16, param_4: u32) -> u32 {
    let mut u_var1: i32;
    let mut in_eax: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_cs: u16;
    let mut b_var5: bool;
    let mut alloc_addr: u32;
    let mut u_var6: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut out_addr: u16;

    u_var2 = (in_eax >> 0x10);
    local_6 = 0;
    local_8 = 0;
    u_var4 = (param_4 >> 0x10);
    i_var3 = param_4;
    if ((i_var3 + 0x14) == -0x4153) {
        WORD_1050_5f30 = 1;
        alloc_addr = in_eax & 0xffff0000;
        u_var2 = (alloc_addr >> 0x10);
        if (param_1 != 1) {
            if (param_1 == 2) {
                local_4 = 2;
                // LAB_1000_016d:
                while (local_6 <= param_3 && (local_6 < param_3 || (local_8 < param_2))) {
                    alloc_addr =
                        alloc_mem_1000_03c6(*(s_version__d__d_1050_0012 + 8), local_4, 0x0);
                    alloc_addr = alloc_addr & 0xffff0000;
                    if (((alloc_addr >> 0x10) | alloc_addr) == 0) {
                        break;
                    }
                    u_var1 = (s_version__d__d_1050_0012 + 8);
                    b_var5 = CARRY2(local_8, u_var1);
                    local_8 = local_8 + u_var1;
                    local_6 = local_6 + b_var5;
                }
                return alloc_addr | local_8;
            }
            if (param_1 == 4) {
                local_4 = 0;
                // goto LAB_1000_016d;
            }
            // goto LAB_1000_012c;
        }
        local_4 = 1;
        if ((i_var3 + 0x18) != 0) {}
        // goto LAB_1000_016d;
        u_var6 = CONCAT22(i_var3, 4);
    } else {
        u_var6 = 10;
    }
    invoke_error_handler_1000_1e61(unaff_cs, u_var6);
    // LAB_1000_012c:
    return CONCAT22(u_var2, 0xffff);
}

pub fn get_mem_sz_1000_01b0() -> bool {
    let pu_var1: *mut u32;
    let pi_var2: *mut i32;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let mut in_bx: i32;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut unaff_cs: u16;
    let lVar7: u32;
    let lVar8: u32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_e = 0;
    if (((in_bx + 0x40) | (in_bx + 0x3e)) == 0) {
        u_var5 = in_bx + 0x36;
        lVar8 = get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
        lVar7 = lVar8;
    } else {
        lVar7 = get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
        u_var5 = lVar7;
        if (((lVar7 >> 0x10) != 0) || (0xffef < u_var5)) {
            invoke_error_handler_1000_1e61(unaff_cs, 8, in_bx);
            return false;
        }
        if (0x1fff < u_var5) {
            u_var5 = 0x2000;
        }
        while (true) {
            u_var4 = u_var5;
            local_4 = (lVar7 >> 0x10);
            local_6 = lVar7;
            lVar8 = lVar7 + 0x18;
            if (((lVar8 >> 0x10) != 0) || (0xfff0 < lVar8)) {
                lVar8 = 0xfff0;
            }
            i_var3 = alloc_mem_1000_14f2(
                (in_bx + 0x16) | 0x1000,
                lVar8,
                in_bx,
                &ctx.g_alloc_addr_1050_1050,
            );
            if (i_var3 != 0) {
                break;
            }
            u_var5 = u_var4 >> 1;
            if (u_var5 < 0xc) {
                i_var3 = invoke_error_handler_1000_1e61(unaff_cs, 2, in_bx);
                if (i_var3 == 0) {
                    return (in_bx + 10) != 0;
                }
                lVar7 = get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
                u_var5 = u_var4 & 0xfffe;
            }
        }
        i_var3 = pass1_fn_1000_5390(local_6 - 0x42, local_4 - (local_6 < 0x42), 0xc, 0);
        u_var5 = i_var3 * 0xc + in_bx + 0x42;
    }
    pu_var1 = (in_bx + 0x1e);
    unsafe {
        u_var4 = *pu_var1;
        *pu_var1 = *pu_var1 - lVar7;
        pi_var2 = (in_bx + 0x20);
        *pi_var2 = (*pi_var2 - (lVar7 >> 0x10)) - (u_var4 < lVar7);
    }
    if (u_var5 != 0) {
        u_var10 = 0;
        u_var9 = 0xc;
        lVar8 = get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
        i_var3 = pass1_fn_1000_5390(
            lVar8 - 0x42,
            (lVar8 >> 0x10) - (lVar8 < 0x42),
            u_var9,
            u_var10,
        );
        local_e = i_var3 * 0xc + in_bx + 0x36;
    }
    local_a = (lVar8 >> 0x10);
    local_c = lVar8;
    pu_var1 = (in_bx + 0x1e);
    unsafe {
        u_var4 = *pu_var1;
        *pu_var1 = *pu_var1 + local_c;
        pi_var2 = (in_bx + 0x20);
        *pi_var2 = *pi_var2 + local_a + CARRY2(u_var4, local_c);
        u_var4 = (in_bx + 10);
    }
    while {
        u_var6 = u_var5;
        (u_var6 + 4) = u_var4;
        u_var4 = u_var6;
        u_var5 = u_var6 + 0xc;
        u_var6 < local_e
    } {}
    (in_bx + 10) = u_var6;
    return true;
}

pub fn get_mem_sz_1000_0308() -> i32 {
    let mut iVar1: i32;
    let mut in_ax: i32;
    let mut i_var2: i32;
    let mut in_bx: i32;
    let pi_var3: *mut i32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    if (((in_bx + 10) == 0) && (i_var2 = get_mem_sz_1000_01b0(), i_var2 == 0)) {
        return 0;
    }
    i_var2 = (in_bx + 10);
    (in_bx + 10) = (i_var2 + 4);
    pi_var3 = (in_ax * 2 + in_bx);
    let pi_var3_val = unsafe { *pi_var3 };
    if (pi_var3_val == 0) {
        (i_var2 + 6) = i_var2;
        (i_var2 + 4) = i_var2;
    } else {
        iVar1 = pi_var3_val;
        (i_var2 + 6) = iVar1;
        (i_var2 + 4) = (iVar1 + 4);
        ((iVar1 + 4) + 6) = i_var2;
        (iVar1 + 4) = i_var2;
    }
    unsafe { *pi_var3 = i_var2 };
    return i_var2;
}

pub fn struct_func_1000_0368() {
    let ctx.ax_reg: *mut Struct125;
    let mut in_dx: i32;
    let mut in_bx: i32;
    Struct125 * *ppaVar1;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    if (ctx.ax_reg.field_0x4 == ctx.ax_reg) {
        (in_bx + in_dx * 2) = 0;
    } else {
        (ctx.ax_reg.field_0x6 + 4) = ctx.ax_reg.field_0x4;
        (ctx.ax_reg.field_0x4 + 6) = ctx.ax_reg.field_0x6;
        ppaVar1 = (in_dx * 2 + in_bx);
        if (*ppaVar1 == ctx.ax_reg) {
            *ppaVar1 = ctx.ax_reg.field_0x4;
        }
    }
    ctx.ax_reg.field_0x4 = (in_bx + 10);
    (in_bx + 10) = ctx.ax_reg;
    return;
}

// WARNING: Variable defined which should be unmapped: local_1c

pub fn alloc_mem_1000_03c6(
    in_alloc_size: u32,
    param_2: u16,
    param_3: *mut Struct93,
) -> *mut libc::c_void {
    let pu_var1: *mut u32;
    let pi_var2: *mut i32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let local_AX_131: *mut u16;
    let mut u_var5: u16;
    let mut in_dx: i32;
    let local_BX__1: *mut Struct78;
    let mut unaff_cs: u16;
    let mut bVar6: bool;
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

    u_var3 = in_alloc_size + 0xfff & 0xf000;
    pu_var1 = &param_3.field_0x1e;
    let pu_var1_val = unsafe { *pu_var1 };
    u_var4 = u_var3 + pu_var1_val;
    u_var3 = in_alloc_size._2_2_
        + (0xf000 < in_alloc_size)
        + param_3.field_0x20
        + CARRY2(u_var3, pu_var1_val);
    pu_var1 = &param_3.field_0x28;
    bVar6 = u_var3 < pu_var1_val;
    if ((bVar6)
        || (bVar6
            || u_var3 == pu_var1_val
                && (
                    pu_var1 = &param_3.field_0x26,
                    u_var4 < pu_var1_val || u_var4 == pu_var1_val,
                )))
    {
        if (param_2 == 3) {
            local_alloc_flags = ((-((in_dx & 1) != 0) >> 8) & 1 | 0x20) << 8;
        } else {
            local_alloc_flags = 0x1000;
        }
        alloc_addr = alloc_mem_1000_131c(&param_3.field_0x16 | local_alloc_flags, in_alloc_size);
        alloc_addr._2_2_ = (alloc_addr >> 0x10);
        if (alloc_addr != 0) {
            local_AX_131 = get_mem_sz_1000_0308();
            if (local_AX_131 != 0x0) {
                local_AX_131[4] = alloc_addr;
                local_AX_131[5] = alloc_addr._2_2_;
                &PTR_LOOP_1050_000c = param_2 | 0xcad0;
                0x0 = param_3;
                &dos_alloc_addr_1050_0002 = &ctx.g_alloc_addr_1050_1050;
                &PTR_DAT_0005_0000_1050_0004 = local_AX_131;
                (&PTR_DAT_0005_0000_1050_0004 + 2) = &ctx.g_alloc_addr_1050_1050;
                &PTR_LOOP_1050_000a = 0;
                alloc_size = get_mem_sz_1000_1532(alloc_addr._2_2_);
                if (param_2 == 1) {
                    u_var5 = set_struct_1000_0782(param_3, &ctx.g_alloc_addr_1050_1050);
                } else {
                    if (param_2 == 3) {
                        u_var5 = set_struct_1000_05b4();
                    } else {
                        u_var5 = set_struct_1000_09ca();
                    }
                }
                unsafe { *local_AX_131 = u_var5 };
                local_AX_131[1] = 0x8000;
                pu_var1 = &param_3.field_0x1e;
                unsafe {
                    u_var3 = *pu_var1;
                    *pu_var1 = *pu_var1 + alloc_size;
                    pi_var2 = &param_3.field_0x20;
                    *pi_var2 = *pi_var2 + (alloc_size >> 0x10) + CARRY2(u_var3, alloc_size);
                }
                return alloc_addr;
            }
            free_mem_1000_13ce(alloc_addr);
        }
    } else {
        invoke_error_handler_1000_1e61(unaff_cs, 7, param_3);
    }
    return 0;
}

pub fn free_mem_1000_0510() {
    let pu_var1: *mut u32;
    let pi_var2: *mut i32;
    let paVar3: *mut Struct88;
    let mut u_var4: i32;
    let mut in_ax: i32;
    let mut u_var5: i32;
    let mut trunc_alloc_size: u16;
    let mut u_var6: i32;
    Struct88 * *in_bx;
    let mut bVar7: bool;
    let mut u_var8: u32;
    // ppu_var9: *mut *mut u8;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_5f5b47e88b: *mut Struct88;

    temp_5f5b47e88b = *in_bx;
    paVar3 = in_bx[1];
    u_var8 = get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
    trunc_alloc_size = (u_var8 >> 0x10);
    u_var5 = u_var8;
    ppu_var9 = &ctx.PTR_LOOP_1050_0000;
    if (in_ax != 0) {
        u_var4 = temp_5f5b47e88b.field_0x1e;
        u_var6 = (temp_5f5b47e88b.field_0x20 - trunc_alloc_size) - (u_var4 < u_var5);
        pu_var1 = &temp_5f5b47e88b.field_0x24;
        let pu_var1_val = unsafe { *pu_var1 };
        bVar7 = u_var6 < unsafe { *pu_var1 };
        if ((bVar7 || u_var6 == pu_var1_val)
            && (bVar7 || (u_var4 - u_var5 < temp_5f5b47e88b.field_0x22)))
        {
            bVar7 = false;
            // goto LAB_1000_0595;
        }
    }
    ppu_var9 = 0x1050057f;
    struct_func_1000_0368();
    pu_var1 = (s_version__d__d_1050_0012 + 0xc);
    unsafe {
        u_var4 = *pu_var1;
        *pu_var1 = *pu_var1 - u_var5;
        pi_var2 = s_New_failed_in_Op__Op_1050_0020;
        *pi_var2 = (*pi_var2 - trunc_alloc_size) - (u_var4 < u_var5);
    }
    bVar7 = true;
    // LAB_1000_0595:
    if (bVar7) {
        in_bx[6] = 0x0;
        free_mem_1000_13ce(ppu_var9 & 0xffff0000 | ZEXT24(in_bx));
        return 1;
    }
    return 0;
}

pub fn set_struct_1000_05b4() {
    let mut in_AL: u8;
    let local_BX__1: *mut Struct82;
    let local_4: u8;

    local_BX__1.field_0xa = 1;
    local_BX__1.field_0x8 = 0x668;
    local_BX__1.field_0x13 = -((in_AL & 2) != 0) & 2;
    local_BX__1.field_0x10 = 0;
    local_BX__1.field_0xe = 0;
    return;
}

pub fn alloc_mem_1000_05e2(param_1: *mut Struct89, param_2: u16, param_3: *mut Struct144) {
    let pu_var1: *mut u32;
    let mut i_var2: i32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let mut i_var5: i32;
    let local_BX__1: *mut Struct90;
    let mut unaff_cs: u16;
    let mut bVar6: bool;
    let pvVar7: &mut Vec<u8>;
    let lVar8: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    i_var2 = param_2 + (0xffeb < param_1);
    while {
        pvVar7 = alloc_mem_1000_03c6(CONCAT22(i_var2, param_1 + 1), 3, local_BX__1);
        if (pvVar7 != 0) {
            return pvVar7 & 0xffff0000 | (pvVar7 + 0x14);
        }
        lVar8 = free_mem_1000_0052();
        u_var3 = &param_1[0xcd].field_0xf & 0xf000;
        pu_var1 = &local_BX__1.field_0x1e;

        let pu_var1_val = unsafe { *pu_var1 };
        u_var4 = u_var3 + pu_var1_val;
        u_var3 =
            i_var2 + (0xf000 < param_1 + 1) + local_BX__1.field_0x20 + CARRY2(u_var3, pu_var1_val);
        pu_var1 = &local_BX__1.field_0x28;
        bVar6 = u_var3 < pu_var1_val;
        ((bVar6 || u_var3 == pu_var1_val)
            && (bVar6
                || (
                    pu_var1 = &local_BX__1.field_0x26,
                    u_var4 < pu_var1_val || u_var4 == pu_var1_val,
                )))
            && (lVar8 != 0
                || (
                    i_var5 = invoke_error_handler_1000_1e61(unaff_cs, 2, local_BX__1),
                    i_var5 != 0,
                ))
    } {}
    return 0;
}

pub fn alloc_mem_1000_0670(param_1: i32, param_2: *mut i32) -> u16 {
    let pu_var1: *mut u32;
    let pi_var2: *mut i32;
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
    let in_bx: *mut u32;
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
    lVar12 = get_mem_sz_1000_1532(&ctx.g_alloc_addr_1050_1050);
    iVar10 = in_dx + (0xffeb < in_ax);
    u_var7 = unsafe { *in_bx };
    u_var8 = -((param_1 & 1) != 0) & 0x100 | -((param_1 & 4) != 0) & 0x400 | (u_var7 + 0x16);
    if ((param_2._2_2_ | param_2) == 0) {
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
            local_10 = (lVar13 >> 0x10);
            if (lVar13 != 0) {
                break;
            }
            i_var9 = invoke_error_handler_1000_1e61(unaff_cs, 2, i_var3);
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
    lVar13 = get_mem_sz_1000_1532(local_10);
    u_var11 = (lVar13 - lVar12);
    pu_var1 = (i_var3 + 0x1e);
    unsafe { u_var8 = *pu_var1 };
    unsafe { *pu_var1 = *pu_var1 + u_var11 };
    pi_var2 = (i_var3 + 0x20);
    unsafe { *pi_var2 = *pi_var2 + ((lVar13 - lVar12) >> 0x10) + CARRY2(u_var8, u_var11) };
    return 1;
}

pub fn alloc_mem_1000_07fc(param_1: *mut u8) {
    let mut unaff_cs: u16;
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0x14) != -0x4153) {
        invoke_error_handler_1000_1e61(unaff_cs, 10, 0);
        return 0;
    }
    u_var1 = alloc_mem_1000_0838(0x0);
    return u_var1;
}

pub fn alloc_mem_1000_0838(param_1: *mut Struct142) {
    let pu_var1: *mut u32;
    Struct143 * *ppaVar2;
    let pi_var3: *mut i32;
    let pu_var4: *mut u16;
    let mut i_var5: i32;
    let mut u_var6: i32;
    let local_BX__1: *mut Struct91;
    let pi_var7: *mut i32;
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
    let temp_1802631433d: *mut u16;
    let temp_5f1305d326: *mut Struct143;

    pi_var7 = param_1.field_0x2;
    local_4 = pi_var7;
    if (param_1.field_0x2 == 0x0) {}
    // goto LAB_1000_085b;
    loop {
        while {
            let pi_var7_val = unsafe { *pi_var7 };
            if (pi_var7_val != 0) {
                i_var5 = pi_var7[5];
                pu_var4 = &PTR_LOOP_1050_000e;
                if (pu_var4 != 0x0) {
                    &PTR_LOOP_1050_000e = unsafe { *pu_var4 };
                    pi_var3 = &PTR_LOOP_1050_000a;
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
        if (param_1.field_0x18 == 0) {
            invoke_error_handler_1000_1e61(unaff_cs, 4, param_1);
            return 0;
        }
        u_var6 = param_1.field_0x1a;
        while (
            local_6 = u_var6,
            pvVar9 = alloc_mem_1000_03c6(local_6, 1, param_1),
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
                i_var5 = invoke_error_handler_1000_1e61(unaff_cs, 2, param_1);
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

pub fn free_mem_1000_093a(param_1: *mut i32, param_2: u16) -> u16 {
    let piVar1: *mut i32;
    let mut unaff_cs: u16;

    if (&PTR_LOOP_1050_000c != -0x352f) {
        invoke_error_handler_1000_1e61(unaff_cs, 0xe, 0);
        return 0;
    }
    let param_1_val = unsafe { *param_1 };
    param_1_val = &PTR_LOOP_1050_000e;
    if (param_1_val == 0) {
        &PTR_DAT_0005_0000_1050_0004 = 1;
    }
    &PTR_LOOP_1050_000e = param_1;
    piVar1 = &PTR_LOOP_1050_000a;
    unsafe {
        *piVar1 = *piVar1 + -1;
        if (*piVar1 == 0) {
            free_mem_1000_0510();
        }
    }
    return 1;
}

pub fn alloc_mem_1000_0a48(param_1: i32, param_2: *mut Struct89, param_3: u16, param_4: u32) {
    let paVar1: *mut Struct140;
    let mut u_var2: i32;
    let in_cx: *mut Struct92;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let local_DI_113: *mut Struct140;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let mut u_var6: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_4 >> 0x10);
    if ((param_4 + 0x14) == -0x4153) {
        if ((param_3 == 0) && (param_2 <= (s_version__d__d_1050_0012 + 6))) {
            if (param_2 == 0x0) {
                invoke_error_handler_1000_1e61(unaff_cs, 4, param_4);
                u_var6 = 0;
            } else {
                u_var6 = alloc_mem_1000_0838(0x0);
                u_var4 = (u_var6 >> 0x10);
                local_DI_113 = u_var6;
                if ((u_var6 != 0) && ((param_1 & 1) != 0)) {
                    u_var2 = (s_version__d__d_1050_0012 + 6);
                    u_var3 = u_var2 >> 1;
                    while (u_var3 != 0) {
                        u_var3 = u_var3 - 1;
                        paVar1 = local_DI_113;
                        local_DI_113 = &local_DI_113.field_0x2;
                        paVar1 = 0;
                    }
                    if ((u_var2 & 1) != 0) {
                        unsafe { *local_DI_113 = 0 };
                    }
                }
            }
        } else {
            if ((param_3 == 0) && (param_2 <= (s_version__d__d_1050_0012 + 10))) {
                u_var6 = alloc_mem_1000_0b20(param_1 & 0xfffd, 0x0, in_cx, param_2);
            } else {
                u_var6 = alloc_mem_1000_05e2(param_2, param_3, 0x0);
            }
        }
        return u_var6;
    }
    invoke_error_handler_1000_1e61(unaff_cs, 10, 0);
    return 0;
}

// WARNING: Type propagation algorithm not settling

pub fn alloc_mem_1000_0b20(
    param_1: u16,
    param_2: *mut Struct93,
    param_3: *mut Struct92,
    param_4: u16,
) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let paVar3: *mut Struct89;
    let mut u_var4: i32;
    let mut alloc_data: u16;
    let local_CX_30: *mut Struct92;
    let pa_var5: *mut Struct89;
    let local_BX__1: *mut Struct93;
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
            if ((pa_var5 <= paVar3)
                && (
                    pvVar9 = pass1_fn_1000_0c32(pa_var5, 0x0, local_14),
                    pvVar9 != 0,
                ))
            {
                (&local_BX__1.field_0x0 + u_var2 * 2) = paVar3;
                return pvVar9;
            }
            paVar3 = &paVar3.field_0x4;
            paVar3 != local_6
        } {}
        // LAB_1000_0b64:
        if (((((local_14 & 2) == 0) || ((local_14 & 0x40) != 0))
            || (local_BX__1.field_0x32 == 0x0))
            || (paVar3 = (*local_BX__1.field_0x32)(), paVar3 < pa_var5))
        {
            if (((local_14 & 0x10) != 0)
                || (
                    pvVar9 = alloc_mem_1000_03c6(local_BX__1.field_0x1a, u_var2, local_BX__1),
                    pvVar9 == 0,
                ))
            {
                if ((local_14 & 0x20) == 0) {
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
                            u_var8 = alloc_mem_1000_05e2(pa_var5, 0, local_BX__1);
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

// WARNING: Could not reconcile some variable overlaps

pub fn alloc_mem_1000_0ed4(
    param_1: i32,
    param_2: i32,
    uparam_3: i32,
    param_4: *mut u16,
    param_5: u16,
) -> libc::c_long {
    let pu_var1: *mut u16;
    let p_uvar2: *mut u16;
    let pu_var3: *mut u16;
    let mut u_var4: i32;
    let pu_var5: *mut u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((&PTR_LOOP_1050_000c & 0xfff8) == 0xcad0) {
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
            u_var6 = (_local_c >> 0x10);
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
                while (u_var4 != 0) {
                    u_var4 = u_var4 - 1;
                    pu_var2 = pu_var3;
                    pu_var3 = (pu_var3 + 1);
                    pu_var1 = pu_var5;
                    pu_var5 = (pu_var5 + 1);
                    unsafe { *pu_var2 = *pu_var1 };
                }
                error_check_1000_0dc6(param_4, param_5);
            }
            return _local_c;
        }
        if ((param_3 | param_2) == 0) {
            return 0;
        }
        u_var6 = 5;
        u_var7 = local_6;
    } else {
        u_var6 = 0xe;
        u_var7 = 0;
    }
    invoke_error_handler_1000_1e61(unaff_cs, u_var6, u_var7);
    return 0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn alloc_mem_1000_0fb8(param_1: i32, param_2: *mut u16, uparam_3: i32) -> u32 {
    let pu_var1: *mut u16;
    let mut b_var2: u8;
    let mut in_ax: i32;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut i_var5: i32;
    let mut u_var6: i32;
    let mut in_dx: i32;
    let mut in_bx: i32;
    let pu_var7: *mut u16;
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
        invoke_error_handler_1000_1e61(unaff_cs, 4, PTR_LOOP_1050_0000);
        if ((param_3 | param_2) != 0) {
            param_2[1] = 0;
            unsafe { *param_2 = 0 };
            return 0;
        }
        return 1;
    }
    b_var2 = PTR_LOOP_1050_000c & 7;
    u_var8 = (_PTR_LOOP_1050_0000 >> 0x10);
    if ((PTR_LOOP_1050_000c & 7) != 0) {
        if (b_var2 == 1) {
            u_var3 = (PTR_LOOP_1050_0000 + 0x18);
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
                && (in_ax <= (PTR_LOOP_1050_0000 + 0x1c)))
            {
                u_var9 = get_mem_sz_1000_1284(in_bx, &ctx.g_alloc_addr_1050_1050);
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
        || (in_dx == 0 && (in_ax <= (PTR_LOOP_1050_0000 + 0x1c))))
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

pub fn get_mem_sz_1000_1284(param_1: u32) {
    let mut bVar1: u8;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let mut u8_var4: u8;
    let mut u_var5: i32;
    let mut unaff_cs: u16;
    let mut bVar6: bool;
    let mut alloc_size: u32;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
        invoke_error_handler_1000_1e61(unaff_cs, 0xe, 0);
        return 0xffffffff;
    }
    bVar1 = *&PTR_LOOP_1050_000c;
    u8_var4 = bVar1 & 7;
    if ((bVar1 & 7) != 0) {
        if (u8_var4 == 1) {
            u_var3 = 0x0;
            return *(u_var3 + 0x18);
        }
        if (u8_var4 != 2) {
            if (u8_var4 != 3) {
                return 0xffffffff;
            }
            alloc_size = get_mem_sz_1000_1532(param_1._2_2_);
            return CONCAT22(
                (alloc_size >> 0x10) - (alloc_size < 0x14),
                alloc_size - 0x14,
            );
        }
    }
    u_var2 = (param_1 + -2);
    u_var5 = u_var2 & 0x7ffc;
    local_6 = u_var5 - 2;
    local_4 = 0;
    if ((u_var2 & 0x8000) != 0) {
        bVar6 = local_6 < 4;
        local_6 = u_var5 - 6;
        local_4 = -bVar6;
    }
    return CONCAT22(local_4, local_6);
}

pub fn alloc_mem_1000_131c(alloc_options: u32, alloc_size_1: u32, alloc_size_2: i32) {
    let handle: HGLOBAL16;
    let mut flags: i32;
    let mut bVar1: bool;
    let mut dos_alloc_addr: u32;
    let mut memory_block_byte: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    dos_alloc_addr = CONCAT22(local_8, local_a);
    // GMEM_NODISCARD, GMEM_NOCOMPACT, GMEM_MOVEABLE
    flags = 0x32;
    local_6 = 1;
    if (((alloc_options & 0x1000) != 0) && (alloc_size_2 != 0 || (0xfff0 < alloc_size_1))) {
        alloc_size_1 = 0xfff0;
        alloc_size_2 = 0;
    }
    if ((alloc_options & 0x100) != 0) {
        flags = 0x72;
    }
    if ((alloc_options & 1) != 0) {
        // mark memory shared
        flags = flags | 0x2000;
    }
    if ((alloc_options & 4) != 0) {
        flags = flags & 0xfffd;
        dos_alloc_addr = alloc_dos_mem_1000_1558();
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
        free_mem_1000_15ce(dos_alloc_addr);
    }
    if (handle == 0) {
        return 0;
    }
    memory_block_byte = GlobalLock16(handle);
    return memory_block_u8;
}

// WARNING: Variable defined which should be unmapped: local_4

pub fn free_mem_1000_13ce(alloc_addr: u32) -> bool {
    let mut out_handle: u16;
    let mut alloc_handle: u32;
    let mut local_4: u16;

    alloc_handle = GlobalHandle16(alloc_addr._2_2_);
    if (alloc_handle != 0) {
        out_handle = GlobalFree16(alloc_handle);
        return (out_handle == 0);
    }
    return 0;
}

pub fn alloc_mem_1000_1408(
    param_1: i32,
    param_2: i32,
    param_3: i32,
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
    if (((param_1 & 0x1000) != 0) && (param_3 != 0 || (0xfff0 < param_2))) {
        param_2 = 0xfff0;
        param_3 = 0;
    }
    if ((param_1 & 0x100) != 0) {
        local_8 = 0x72;
    }
    if ((param_1 & 0x804) != 0) {
        local_8 = local_8 & 0xfffd;
    }
    if (handle_00 != 0) {
        if ((param_1 & 4) != 0) {
            GlobalPageUnlock();
        }
        while {
            handle = GlobalReAlloc16(local_8, CONCAT22(param_3, param_2), handle_00);
            if (handle != 0) {
                break;
            }
            local_8 = local_8 & 0xffcf;
            bVar1 = local_c != 0;
            local_c = local_c - 1;
            bVar1
        } {}
        if ((handle != 0) && ((param_1 & 4) != 0)) {
            GlobalPageLock(handle);
        }
        if (handle != 0) {
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

    if (((param_1 & 0x1000) != 0) || (param_3 == 0 && (param_2 < 0xfff1))) {
        lVar1 = alloc_mem_1000_1408(param_1 & 0xfdff | 0x800, param_2, param_3, param_4, param_5);
        if (lVar1 != 0) {
            return 1;
        }
    }
    return 0;
}

pub fn get_mem_sz_1000_1532(alloc_addr: u32) -> u32 {
    let mut alloc_handle: u32;
    let dVar1: u32;

    alloc_handle = GlobalHandle16(alloc_addr);
    if (alloc_handle != 0) {
        dVar1 = GlobalSize16(alloc_handle);
        return dVar1;
    }
    return 0;
}

// Allocate memory in first MB, return rsult

pub fn alloc_dos_mem_1000_1558() -> libc::c_long {
    let mut u_var1: i32;
    let mut in_ax: i32;
    let mut alloc_addr_cpy: u16;
    let mut in_dx: i32;
    let dVar2: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut out_alloc_addr: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    out_alloc_addr = 0;
    local_a = 0;
    local_8 = 8;
    if ((in_dx < 9) && (in_dx < 8 || (in_ax == 0))) {
        while {
            while (true) {
                dVar2 = GlobalDOSAlloc16(CONCAT22(local_8, local_a));
                alloc_addr_cpy = dVar2;
                if (alloc_addr_cpy == 0) {
                    break;
                }
                0x0 = 0;
                &dos_alloc_addr_1050_0002 = out_alloc_addr;
                out_alloc_addr = alloc_addr_cpy;
            }
            u_var1 = local_8 & 1;
            local_8 = local_8 >> 1;
            local_a = local_a >> 1 | (u_var1 != 0) << 0xf;
            (in_dx < local_8) || (in_dx <= local_8 && (in_ax <= local_a))
        } {}
    }
    return out_alloc_addr << 0x10;
}

pub fn free_mem_1000_15ce(param_1: *mut u32, param_2: u16) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let mut local_8: u16;
    let mut local_4: u16;

    u_var3 = param_2 | param_1;
    while (u_var3 != 0) {
        unsafe { pu_var1 = *param_1 };
        u_var2 = param_1[1];
        GlobalDOSFree16(param_2);
        u_var3 = u_var2 | pu_var1;
        param_2 = u_var2;
        param_1 = pu_var1;
    }
    return;
}

pub fn alloc_mem_1000_167a(param_1: u16, param_2: u16) -> Vec<u8> {
    let mut u_var1: i32;
    let mut mem_buf: u16;

    if ((ctx.g_u16_ptr_1050_5f2e | _g_Struct94_ptr_1) == 0) {
        u_var1 = struct_fn_1000_160a();
        if ((param_2 | u_var1) == 0) {
            return 0;
        }
    }
    mem_buf = alloc_mem_1000_0a48(0, param_1, 0, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    return mem_buf;
}

pub fn alloc_mem_1000_16aa(param_1: i32, param_2: i32, param_3: u16) -> *mut libc::c_void {
    let pvVar1: &mut Vec<u8>;
    let mut in_dx: u16;

    if ((param_2 | param_1) == 0) {
        pvVar1 = alloc_mem_1000_167a(param_3, in_dx);
        return pvVar1;
    }
    if (param_3 == 0) {
        error_check_1000_16ee(param_1, param_2);
        return 0;
    }
    pvVar1 = alloc_mem_1000_0ed4(0, param_3, 0, param_1, param_2);
    return pvVar1;
}

pub fn alloc_mem_1000_1708(
    param_1: i32,
    param_2: i32,
    uparam_3: i32,
    param_4: i32,
    uparam_5: i32,
) -> u16 {
    let mut iVar1: i32;
    let mut b_var2: bool;
    let lVar3: u32;
    let mut local_4: u16;

    if ((param_2 | param_1) == 0) {
        b_var2 = 0xfffe < param_1;
        param_1 = param_1 + 1;
        param_2 = param_2 + b_var2;
    }
    // LAB_1000_1724:
    loop {
        if ((param_5 | param_4) != 0) {
            lVar3 = alloc_mem_1000_0a48(param_3, param_1, param_2, param_4, param_5);
            if (lVar3 != 0) {
                return lVar3;
            }
        }
        if (((param_3 & 0x8000) == 0) || ((PTR_LOOP_1050_5f3a | func_ptr_1050_5f38) == 0)) {
            if ((ctx.PTR_LOOP_1050_5f36 | ctx.func_ptr_1050_5f34) == 0) {
                if ((PTR_LOOP_1050_5f3e | func_ptr_1050_5f3c) == 0) {
                    return 0;
                }
                (*func_ptr_1050_5f3c)();
                // goto LAB_1000_1724;
            }
            iVar1 = (*ctx.func_ptr_1050_5f34)();
        } else {
            iVar1 = (*func_ptr_1050_5f38)();
        }
        if (iVar1 == 0) {
            return 0;
        }
    }
}

pub fn alloc_mem_1000_180c(param_1: u16) -> u16 {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let mut in_dx: i32;

    if ((ctx.g_u16_ptr_1050_5f2e | _g_Struct94_ptr_1) == 0) {
        u_var1 = struct_fn_1000_160a();
        if ((in_dx | u_var1) == 0) {
            return 0;
        }
    }
    u_var2 = alloc_mem_1000_0a48(0, param_1, 0, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    return u_var2;
}

// WARNING: Variable defined which should be unmapped: local_6

pub fn alloc_mem_1000_183c(param_1: i32, param_2: i32) -> u16 {
    let mut u_var1: u16;
    let pu_var2: *mut u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = (param_2 * param_1);
    pu_var2 = 0x0;
    if ((param_2 * param_1 >> 0x10) != 0) {
        return 0;
    }
    if (((ctx.g_u16_ptr_1050_5f2e | _g_Struct94_ptr_1) == 0)
        && (
            _g_Struct94_ptr_1 = struct_fn_1000_160a(u_var1, 0),
            ctx.g_u16_ptr_1050_5f2e = pu_var2,
            (pu_var2 | _g_Struct94_ptr_1) == 0,
        ))
    {
        return 0;
    }
    u_var1 = alloc_mem_1000_0a48(1, u_var1, 0, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    return u_var1;
}

pub fn alloc_mem_1000_188e(param_1: i32, param_2: i32, param_3: i32) -> u16 {
    let mut u_var1: u16;

    if ((param_2 | param_1) == 0) {
        u_var1 = alloc_mem_1000_180c(param_3);
        return u_var1;
    }
    if (param_3 == 0) {
        error_check_1000_18d2(param_1, param_2);
        return 0;
    }
    u_var1 = alloc_mem_1000_0ed4(0, param_3, 0, param_1, param_2);
    return u_var1;
}

pub fn free_mem_1000_1b68(param_1: *mut Struct98, param_2: u16) -> bool {
    let b_var1: bool;
    let mut unaff_cs: u16;

    if (param_1.field_0x14 != -0x4153) {
        invoke_error_handler_1000_1e61(unaff_cs, 10, 0);
        return 0;
    }
    b_var1 = free_mem_1000_1b9a(0, CONCAT22(param_2, param_1));
    return b_var1;
}

// WARNING: Could not reconcile some variable overlaps

pub fn free_mem_1000_1b9a(param_1: u16, param_1_00: *mut Struct99) -> bool {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let local_bx_5: *mut Struct99;
    let local_SI_34: *mut Struct100;
    let mut u_var6: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = (param_1_00 >> 0x10);
    local_bx_5 = param_1_00;
    local_bx_5.field_0x14 = 0;
    local_4 = 0;
    while {
        local_SI_34 = (local_4 * 2);
        if (local_SI_34 != 0x0) {
            while {
                u_var3 = &local_SI_34.field_0x8;
                (u_var3 + 0xc) = 0;
                free_mem_1000_13ce(CONCAT22(local_SI_34.field_0xa, local_SI_34.field_0x8));
                local_SI_34 = local_SI_34.field_0x4;
                (local_4 * 2) != local_SI_34
            } {}
        }
        local_4 = local_4 + 1;
        local_4 < 5
    } {}
    u_var4 = local_bx_5.field_0x10;
    u_var5 = local_bx_5.field_0x12;
    while (true) {
        _local_8 = CONCAT22(u_var5, u_var4);
        if ((u_var5 | u_var4) == 0) {
            break;
        }
        u_var1 = *_local_8;
        u_var2 = (u_var4 + 2);
        free_mem_1000_13ce(CONCAT22(u_var5, u_var4));
        u_var4 = u_var1;
        u_var5 = u_var2;
    }
    pass1_fn_1000_20a2(param_1_00);
    free_mem_1000_13ce(param_1_00);
    return 1;
}

pub fn alloc_mem_1000_3c51() {
    let piVar1: *mut i32;
    let pc_var2: *mut libc::c_char;
    let handle: HGLOBAL16;
    let mut i_var3: i32;
    let piVar4: *mut i32;
    let in_ax: HGLOBAL16;
    let HVar5: HGLOBAL16;
    let mut in_cx: i32;
    let mut in_bx: i32;
    let pi_var6: *mut i32;
    let pcVar7: *mut libc::c_char;
    let dVar8: u32;
    let in_stack_00000000: HGLOBAL16;
    let mut in_stack_00000008: i32;
    let pc_var9: *mut libc::c_char;

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
    pc_var9 = process_string_1000_28dc(pc_var9);
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

pub fn unlock_mem_1000_3cd8(param_1: u16, param_2: u16) {
    unlock_mem_1000_407a(param_1, param_2);
    return;
}

pub fn unlock_mem_1000_407a(a: u16, b: u16) {
    GlobalFree16(&ctx.g_alloc_addr_1050_1050);
    return;
}

pub fn alloc_mem_func_1000_40af(param_1: i32, param_2: i32, uparam_3: i32) -> *mut u16 {
    let pu_var1: *mut u16;
    let mut u_var2: u16;
    let pcVar3: *mut libc::c_char;
    let pu_var4: *mut u16;
    let pu_var5: *mut u16;
    let mut u_var6: i32;
    let mut handle: i32;
    let handle_00: HGLOBAL16;
    let mut i_var7: i32;
    let mut unaff_si: i32;
    let puVar8: *mut u16;
    let pc_var9: *mut libc::c_char;
    let unaff_DI: HGLOBAL16;
    let pu_var10: *mut u16;
    let unaff_cs: u8;
    let mut bVar11: bool;
    let SVar12: SEGPTR;
    let u_var13: u8;
    let pc_var14: *mut libc::c_char;
    let mut local_6: u16;

    loop {
        u_var6 = (param_1 * param_3);
        handle = param_2 * param_3 + (param_1 * param_3 >> 0x10);
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
                    pass1_fn_1000_25a8(0x12);
                    pass1_fn_1000_2913(u_var13);
                    pc_var14 = process_string_1000_28dc(pc_var14);
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
                handle_00 = pass1_fn_1000_422a((SVar12 >> 0x10), handle_00);
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
        if ((PTR_LOOP_1050_618e | PTR_LOOP_1050_618c) == 0) {
            return 0x0;
        }
        i_var7 = (*PTR_LOOP_1050_618c)(unaff_cs, param_3, param_1, param_2);
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

pub fn copy_mem_1008_4274(in_Struct101: *mut Struct101) {
    let mut u_var1: u32;
    let in_ax: *mut Struct102;
    
    
    let struct_a: *mut Struct199;
    
    let local_Struct101: *mut Struct101;
    let mut a_struct_102: u16;
    let mut u_var2: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = (in_Struct101 >> 0x10);
    local_Struct101 = in_Struct101;
    if (local_Struct101.field_0x6 != 0) {
        get_mem_sz_1000_1284(local_Struct101.field_0x6);
        a_struct_102 = in_ax;
        alloc_mem_1000_0a48(1, in_ax, ctx.dx_reg, ctx.__g_Struct94_ptr_1);
        _local_a = CONCAT22(ctx.dx_reg, a_struct_102);
        struct_a = (ctx.dx_reg | a_struct_102);
        if (struct_a != 0x0) {
            u_var1 = local_Struct101.field_0x6;
            hmemcpy16(
                CONCAT22(in_ax, 0x1000),
                CONCAT13((u_var1 >> 8), CONCAT12(u_var1, ctx.dx_reg)),
            );
            process_struct_1000_179c(0x1e, struct_a);
            if ((struct_a | a_struct_102) == 0) {
                a_struct_102 = 0;
                u_var2 = 0;
            } else {
                set_struct_1008_4016(a_struct_102);
                u_var2 = ctx.dx_reg;
            }
            (a_struct_102 + 6) = _local_a;
            process_struct_1008_47cc(CONCAT22(u_var2, a_struct_102));
            process_struct_1008_4834(CONCAT22(u_var2, a_struct_102));
            (a_struct_102 + 0x1c) = 1;
            return;
        }
    }
    return;
}

pub fn copy_mem_1008_676e(param_1: u32) {
    let mut u_var1: u32;
    let ctx.ax_reg: *mut Struct111;
    let mut count: u32;
    let in_dx: *mut Struct199;
    let mut u_var2: i32;
    let local_bx_4: *mut Struct109;
    let local_bx_42: *mut Struct110;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x6 == 0) {
        return;
    }
    process_struct_1000_179c(0x1e, in_dx);
    u_var2 = in_dx | ctx.ax_reg;
    if (u_var2 == 0) {
        ctx.ax_reg = 0x0;
        u_var2 = 0;
    } else {
        u_var1 = local_bx_4.field_0x10;
        u_var4 = (u_var1 >> 0x10);
        local_bx_42 = u_var1;
        pass1_1008_6604(
            CONCAT22(in_dx, ctx.ax_reg),
            local_bx_42.field_0x8,
            local_bx_42.field_0x4,
        );
    }
    pass1_fn_1000_48a8(ctx.ax_reg.field_0x10, local_bx_4.field_0x10, 0x28);
    u_var1 = ctx.ax_reg.field_0x10;
    count = (u_var1 + 8) * local_bx_4.field_0x18;
    hmemcpy16(
        CONCAT22(count, 0x1000),
        CONCAT22(local_bx_4.field_0x6, (count >> 0x10)),
    );
    ctx.ax_reg.field_0x1c = 1;
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn alloc_mem_1008_909c(in_struct_1: *mut Struct210) {
    let pu_var1: *mut u32;
    let local_AX_23: *mut u8;
    let mut u_var2: i32;
    let ctx.dx_reg: *mut u16;
    let mut i_var3: i32;
    let mut local_es_4: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_es_4 = (in_struct_1 >> 0x10);
    i_var3 = in_struct_1;
    if ((i_var3 + 0x12) == 0) {
        local_AX_23 = (i_var3 + 0xe);
        ctx.g_u16_ptr_1050_5f2e = (i_var3 + 0x10);
    } else {
        u_var2 = (i_var3 + 0x12);
        pu_var1 = (i_var3 + 0x16);
        let pu_var1_val = unsafe { *pu_var1 };
        local_AX_23 = (u_var2 + pu_var1_val);
        ctx.g_u16_ptr_1050_5f2e = ((i_var3 + 0x14) + (i_var3 + 0x18) + CARRY2(u_var2, pu_var1_val));
    }
    _local_6 = CONCAT22(ctx.g_u16_ptr_1050_5f2e, local_AX_23);
    if ((i_var3 + 6) == 0) {
        if (ctx.__g_Struct94_ptr_1 == 0) {
            _g_Struct94_ptr_1 = local_AX_23;
            struct_fn_1000_160a();
        } else {
        }
        u_var2 = local_AX_23 << 2;
        alloc_mem_1000_1708(u_var2, 0, 1, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    } else {
        u_var2 = local_AX_23 * 4;
        alloc_mem_1000_0ed4(
            1,
            u_var2,
            (ctx.g_u16_ptr_1050_5f2e * 2 + CARRY2(local_AX_23, local_AX_23)) * 2
                + CARRY2(local_AX_23 * 2, local_AX_23 * 2),
            (i_var3 + 6),
        );
        ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
    }
    local_a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, u_var2);
    if ((ctx.g_u16_ptr_1050_5f2e | u_var2) != 0) {
        (i_var3 + 0x12) = _local_6;
        (i_var3 + 6) = local_a;
    }
    return;
}

pub fn free_mem_1008_ad0c(param_1: u16, param_2: u8) {
    let in_stack_00000007: u8;
    let mut in_stack_00000008: u8;

    _param_1 = CONCAT13(in_stack_00000007, CONCAT12(param_2, param_1));
    *_param_1 = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((in_stack_00000008 & 1) != 0) {
        free_mem_1000_093a(_param_1);
    }
    return _param_1 & 0xffff0000 | CONCAT12(param_2, param_1) & 0xffff;
}

pub fn pass1_1030_9e9c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        free_mem_1000_093a(param_1);
    }
    return param_1;
}
