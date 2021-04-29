// u32  free_mem_1000_0016(param_1: u32)
pub fn free_mem_1000_0016(param_1: u32) -> u32 {
    // let mut unaff_CS: u16;
    let mut unaff_CS: u16;
    // let mut u_var1: u32;
    let mut uVar1: u32;
    // let mut local_6: u16;
    let mut local_6: u16;
    // let mut local_4: u16;
    let mut local_4: u16;

    if ((param_1 + 0x14) != -0x4153) {
        invoke_error_handler_1000_1e61(unaff_CS, 10, 0);
        return 0xffffffff;
    }
    uVar1 = free_mem_1000_0052(&g_alloc_addr_1050_1050);
    return uVar1;
}

// u32  free_mem_1000_0052()
pub fn free_mem_1000_0052(param_1: *mut u32) -> u32 {
    // puVar1: *mut u32;
    let mut puVar1: u32;
    // let mut uVar2: i32;
    let mut uVar2: u32;
    // let mut iVar3: i32;
    let mut iVar3: i32;
    // let mut uVar4: u32;
    let mut uVar4: u32;
    // let BVar5: bool;
    let mut BVar5: bool;
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

    uVar2 = local_BX__1.field_0x1e;
    iVar3 = local_BX__1.field_0x20;
    local_8 = 0;
    while {
        local_a = (&local_BX__1.field_0x0 + local_8 * 2);
        if ((local_a != 0x0) && (local_8 != 3)) {
            local_e = 0;
            while {
                local_c = local_a.field_0x4;
                uVar4 = local_a.field_0x8;
                if ((uVar4 + 10) == 0) {
                    BVar5 = free_mem_1000_0510();
                    if (BVar5 == 0) {}
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
    puVar1 = &local_BX__1.field_0x1e;
    return CONCAT22(
        (iVar3 - local_BX__1.field_0x20) - (uVar2 < *puVar1),
        uVar2 - *puVar1,
    );
}

pub fn alloc_mem_1000_010c(param_1: u16, param_2: u16, param_3: u16, param_4: u32) -> u32 {
    let mut uVar1: i32;
    let mut in_EAX: u32;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_CS: u16;
    let mut bVar5: bool;
    let mut alloc_addr: u32;
    let mut uVar6: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut out_addr: u16;

    uVar2 = (in_EAX >> 0x10);
    local_6 = 0;
    local_8 = 0;
    uVar4 = (param_4 >> 0x10);
    iVar3 = param_4;
    if ((iVar3 + 0x14) == -0x4153) {
        WORD_1050_5f30 = 1;
        alloc_addr = in_EAX & 0xffff0000;
        uVar2 = (alloc_addr >> 0x10);
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
                    uVar1 = (s_version__d__d_1050_0012 + 8);
                    bVar5 = CARRY2(local_8, uVar1);
                    local_8 = local_8 + uVar1;
                    local_6 = local_6 + bVar5;
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
        if ((iVar3 + 0x18) != 0) {}
        // goto LAB_1000_016d;
        uVar6 = CONCAT22(iVar3, 4);
    } else {
        uVar6 = 10;
    }
    invoke_error_handler_1000_1e61(unaff_CS, uVar6);
    // LAB_1000_012c:
    return CONCAT22(uVar2, 0xffff);
}

pub fn get_mem_sz_1000_01b0() -> bool {
    let puVar1: *mut u32;
    let piVar2: *mut i32;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let mut in_BX: i32;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let mut unaff_CS: u16;
    let lVar7: u32;
    let lVar8: u32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_e = 0;
    if (((in_BX + 0x40) | (in_BX + 0x3e)) == 0) {
        uVar5 = in_BX + 0x36;
        lVar8 = get_mem_sz_1000_1532(&g_alloc_addr_1050_1050);
        lVar7 = lVar8;
    } else {
        lVar7 = get_mem_sz_1000_1532(&g_alloc_addr_1050_1050);
        uVar5 = lVar7;
        if (((lVar7 >> 0x10) != 0) || (0xffef < uVar5)) {
            invoke_error_handler_1000_1e61(unaff_CS, 8, in_BX);
            return false;
        }
        if (0x1fff < uVar5) {
            uVar5 = 0x2000;
        }
        while (true) {
            uVar4 = uVar5;
            local_4 = (lVar7 >> 0x10);
            local_6 = lVar7;
            lVar8 = lVar7 + 0x18;
            if (((lVar8 >> 0x10) != 0) || (0xfff0 < lVar8)) {
                lVar8 = 0xfff0;
            }
            iVar3 = alloc_mem_1000_14f2(
                (in_BX + 0x16) | 0x1000,
                lVar8,
                in_BX,
                &g_alloc_addr_1050_1050,
            );
            if (iVar3 != 0) {
                break;
            }
            uVar5 = uVar4 >> 1;
            if (uVar5 < 0xc) {
                iVar3 = invoke_error_handler_1000_1e61(unaff_CS, 2, in_BX);
                if (iVar3 == 0) {
                    return (in_BX + 10) != 0;
                }
                lVar7 = get_mem_sz_1000_1532(&g_alloc_addr_1050_1050);
                uVar5 = uVar4 & 0xfffe;
            }
        }
        iVar3 = pass1_fn_1000_5390(local_6 - 0x42, local_4 - (local_6 < 0x42), 0xc, 0);
        uVar5 = iVar3 * 0xc + in_BX + 0x42;
    }
    puVar1 = (in_BX + 0x1e);
    unsafe {
        uVar4 = *puVar1;
        *puVar1 = *puVar1 - lVar7;
        piVar2 = (in_BX + 0x20);
        *piVar2 = (*piVar2 - (lVar7 >> 0x10)) - (uVar4 < lVar7);
    }
    if (uVar5 != 0) {
        uVar10 = 0;
        uVar9 = 0xc;
        lVar8 = get_mem_sz_1000_1532(&g_alloc_addr_1050_1050);
        iVar3 = pass1_fn_1000_5390(
            lVar8 - 0x42,
            (lVar8 >> 0x10) - (lVar8 < 0x42),
            uVar9,
            uVar10,
        );
        local_e = iVar3 * 0xc + in_BX + 0x36;
    }
    local_a = (lVar8 >> 0x10);
    local_c = lVar8;
    puVar1 = (in_BX + 0x1e);
    unsafe {
        uVar4 = *puVar1;
        *puVar1 = *puVar1 + local_c;
        piVar2 = (in_BX + 0x20);
        *piVar2 = *piVar2 + local_a + CARRY2(uVar4, local_c);
        uVar4 = (in_BX + 10);
    }
    while {
        uVar6 = uVar5;
        (uVar6 + 4) = uVar4;
        uVar4 = uVar6;
        uVar5 = uVar6 + 0xc;
        uVar6 < local_e
    } {}
    (in_BX + 10) = uVar6;
    return true;
}

pub fn get_mem_sz_1000_0308() -> i32 {
    let mut iVar1: i32;
    let mut in_AX: i32;
    let mut iVar2: i32;
    let mut in_BX: i32;
    let piVar3: *mut i32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    if (((in_BX + 10) == 0) && (iVar2 = get_mem_sz_1000_01b0(), iVar2 == 0)) {
        return 0;
    }
    iVar2 = (in_BX + 10);
    (in_BX + 10) = (iVar2 + 4);
    piVar3 = (in_AX * 2 + in_BX);
    let pi_var3_val = unsafe { *piVar3 };
    if (pi_var3_val == 0) {
        (iVar2 + 6) = iVar2;
        (iVar2 + 4) = iVar2;
    } else {
        iVar1 = pi_var3_val;
        (iVar2 + 6) = iVar1;
        (iVar2 + 4) = (iVar1 + 4);
        ((iVar1 + 4) + 6) = iVar2;
        (iVar1 + 4) = iVar2;
    }
    unsafe { *piVar3 = iVar2 };
    return iVar2;
}

pub fn struct_func_1000_0368() {
    let local_AX__1: *mut Struct125;
    let mut in_DX: i32;
    let mut in_BX: i32;
    Struct125 * *ppaVar1;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    if (local_AX__1.field_0x4 == local_AX__1) {
        (in_BX + in_DX * 2) = 0;
    } else {
        (local_AX__1.field_0x6 + 4) = local_AX__1.field_0x4;
        (local_AX__1.field_0x4 + 6) = local_AX__1.field_0x6;
        ppaVar1 = (in_DX * 2 + in_BX);
        if (*ppaVar1 == local_AX__1) {
            *ppaVar1 = local_AX__1.field_0x4;
        }
    }
    local_AX__1.field_0x4 = (in_BX + 10);
    (in_BX + 10) = local_AX__1;
    return;
}

// WARNING: Variable defined which should be unmapped: local_1c

pub fn alloc_mem_1000_03c6(
    in_alloc_size: u32,
    param_2: u16,
    param_3: *mut Struct93,
) -> *mut libc::c_void {
    let puVar1: *mut u32;
    let piVar2: *mut i32;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let local_AX_131: *mut u16;
    let mut uVar5: u16;
    let mut in_DX: i32;
    let local_BX__1: *mut Struct78;
    let mut unaff_CS: u16;
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

    uVar3 = in_alloc_size + 0xfff & 0xf000;
    puVar1 = &param_3.field_0x1e;
    let pu_var1_val = unsafe { *puVar1 };
    uVar4 = uVar3 + pu_var1_val;
    uVar3 = in_alloc_size._2_2_
        + (0xf000 < in_alloc_size)
        + param_3.field_0x20
        + CARRY2(uVar3, pu_var1_val);
    puVar1 = &param_3.field_0x28;
    bVar6 = uVar3 < pu_var1_val;
    if ((bVar6)
        || (bVar6
            || uVar3 == pu_var1_val
                && (
                    puVar1 = &param_3.field_0x26,
                    uVar4 < pu_var1_val || uVar4 == pu_var1_val,
                )))
    {
        if (param_2 == 3) {
            local_alloc_flags = ((-((in_DX & 1) != 0) >> 8) & 1 | 0x20) << 8;
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
                &dos_alloc_addr_1050_0002 = &g_alloc_addr_1050_1050;
                &PTR_DAT_0005_0000_1050_0004 = local_AX_131;
                (&PTR_DAT_0005_0000_1050_0004 + 2) = &g_alloc_addr_1050_1050;
                &PTR_LOOP_1050_000a = 0;
                alloc_size = get_mem_sz_1000_1532(alloc_addr._2_2_);
                if (param_2 == 1) {
                    uVar5 = set_struct_1000_0782(param_3, &g_alloc_addr_1050_1050);
                } else {
                    if (param_2 == 3) {
                        uVar5 = set_struct_1000_05b4();
                    } else {
                        uVar5 = set_struct_1000_09ca();
                    }
                }
                unsafe { *local_AX_131 = uVar5 };
                local_AX_131[1] = 0x8000;
                puVar1 = &param_3.field_0x1e;
                unsafe {
                    uVar3 = *puVar1;
                    *puVar1 = *puVar1 + alloc_size;
                    piVar2 = &param_3.field_0x20;
                    *piVar2 = *piVar2 + (alloc_size >> 0x10) + CARRY2(uVar3, alloc_size);
                }
                return alloc_addr;
            }
            free_mem_1000_13ce(alloc_addr);
        }
    } else {
        invoke_error_handler_1000_1e61(unaff_CS, 7, param_3);
    }
    return 0;
}

pub fn free_mem_1000_0510() {
    let puVar1: *mut u32;
    let piVar2: *mut i32;
    let paVar3: *mut Struct88;
    let mut uVar4: i32;
    let mut in_AX: i32;
    let mut uVar5: i32;
    let mut trunc_alloc_size: u16;
    let mut uVar6: i32;
    Struct88 * *in_BX;
    let mut bVar7: bool;
    let mut uVar8: u32;
    // ppuVar9: *mut *mut u8;
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

    temp_5f5b47e88b = *in_BX;
    paVar3 = in_BX[1];
    uVar8 = get_mem_sz_1000_1532(&g_alloc_addr_1050_1050);
    trunc_alloc_size = (uVar8 >> 0x10);
    uVar5 = uVar8;
    ppuVar9 = &PTR_LOOP_1050_0000;
    if (in_AX != 0) {
        uVar4 = temp_5f5b47e88b.field_0x1e;
        uVar6 = (temp_5f5b47e88b.field_0x20 - trunc_alloc_size) - (uVar4 < uVar5);
        puVar1 = &temp_5f5b47e88b.field_0x24;
        let pu_var1_val = unsafe { *puVar1 };
        bVar7 = uVar6 < unsafe { *puVar1 };
        if ((bVar7 || uVar6 == pu_var1_val)
            && (bVar7 || (uVar4 - uVar5 < temp_5f5b47e88b.field_0x22)))
        {
            bVar7 = false;
            // goto LAB_1000_0595;
        }
    }
    ppuVar9 = 0x1050057f;
    struct_func_1000_0368();
    puVar1 = (s_version__d__d_1050_0012 + 0xc);
    unsafe {
        uVar4 = *puVar1;
        *puVar1 = *puVar1 - uVar5;
        piVar2 = s_New_failed_in_Op__Op_1050_0020;
        *piVar2 = (*piVar2 - trunc_alloc_size) - (uVar4 < uVar5);
    }
    bVar7 = true;
    // LAB_1000_0595:
    if (bVar7) {
        in_BX[6] = 0x0;
        free_mem_1000_13ce(ppuVar9 & 0xffff0000 | ZEXT24(in_BX));
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
    let puVar1: *mut u32;
    let mut iVar2: i32;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let mut iVar5: i32;
    let local_BX__1: *mut Struct90;
    let mut unaff_CS: u16;
    let mut bVar6: bool;
    let pvVar7: *mut void;
    let lVar8: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    iVar2 = param_2 + (0xffeb < param_1);
    while {
        pvVar7 = alloc_mem_1000_03c6(CONCAT22(iVar2, param_1 + 1), 3, local_BX__1);
        if (pvVar7 != 0) {
            return pvVar7 & 0xffff0000 | (pvVar7 + 0x14);
        }
        lVar8 = free_mem_1000_0052();
        uVar3 = &param_1[0xcd].field_0xf & 0xf000;
        puVar1 = &local_BX__1.field_0x1e;

        let pu_var1_val = unsafe { *puVar1 };
        uVar4 = uVar3 + pu_var1_val;
        uVar3 =
            iVar2 + (0xf000 < param_1 + 1) + local_BX__1.field_0x20 + CARRY2(uVar3, pu_var1_val);
        puVar1 = &local_BX__1.field_0x28;
        bVar6 = uVar3 < pu_var1_val;
        ((bVar6 || uVar3 == pu_var1_val)
            && (bVar6
                || (
                    puVar1 = &local_BX__1.field_0x26,
                    uVar4 < pu_var1_val || uVar4 == pu_var1_val,
                )))
            && (lVar8 != 0
                || (
                    iVar5 = invoke_error_handler_1000_1e61(unaff_CS, 2, local_BX__1),
                    iVar5 != 0,
                ))
    } {}
    return 0;
}

pub fn alloc_mem_1000_0670(uparam_1: i32, param_2: *mut i32) -> u16 {
    let puVar1: *mut u32;
    let piVar2: *mut i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut uVar7: u32;
    let mut in_AX: i32;
    let mut uVar8: i32;
    let mut iVar9: i32;
    let mut iVar10: i32;
    let mut uVar11: i32;
    let mut in_DX: i32;
    let in_BX: *mut u32;
    let mut unaff_CS: u16;
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

    iVar3 = in_BX;
    iVar4 = (in_BX + 2);
    lVar12 = get_mem_sz_1000_1532(&g_alloc_addr_1050_1050);
    iVar10 = in_DX + (0xffeb < in_AX);
    uVar7 = unsafe { *in_BX };
    uVar8 = -((param_1 & 1) != 0) & 0x100 | -((param_1 & 4) != 0) & 0x400 | (uVar7 + 0x16);
    if ((param_2._2_2_ | param_2) == 0) {
        iVar10 = alloc_mem_1000_14f2(
            uVar8 | 0x2000,
            in_AX + 0x14,
            iVar10,
            in_BX,
            &g_alloc_addr_1050_1050,
        );
        if (iVar10 == 0) {
            return 0;
        }
        local_10 = &g_alloc_addr_1050_1050;
    } else {
        iVar5 = (in_BX + 1);
        iVar6 = (in_BX + 6);
        while {
            lVar13 = alloc_mem_1000_1408(
                uVar8 | 0x2000,
                in_AX + 0x14,
                iVar10,
                in_BX,
                &g_alloc_addr_1050_1050,
            );
            local_10 = (lVar13 >> 0x10);
            if (lVar13 != 0) {
                break;
            }
            iVar9 = invoke_error_handler_1000_1e61(unaff_CS, 2, iVar3);
            iVar9 != 0
        } {}
        if (lVar13 == 0) {
            (param_2 + 2) = 0;
            unsafe { *param_2 = 0 };
            return 0;
        }
        (iVar5 + 8) = lVar13;
        (iVar5 + 10) = local_10;
        unsafe { *param_2 = lVar13 + 0x14 };
        (param_2 + 2) = local_10;
    }
    lVar13 = get_mem_sz_1000_1532(local_10);
    uVar11 = (lVar13 - lVar12);
    puVar1 = (iVar3 + 0x1e);
    unsafe { uVar8 = *puVar1 };
    unsafe { *puVar1 = *puVar1 + uVar11 };
    piVar2 = (iVar3 + 0x20);
    unsafe { *piVar2 = *piVar2 + ((lVar13 - lVar12) >> 0x10) + CARRY2(uVar8, uVar11) };
    return 1;
}

pub fn alloc_mem_1000_07fc(param_1: *mut u8) {
    let mut unaff_CS: u16;
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0x14) != -0x4153) {
        invoke_error_handler_1000_1e61(unaff_CS, 10, 0);
        return 0;
    }
    uVar1 = alloc_mem_1000_0838(0x0);
    return uVar1;
}

pub fn alloc_mem_1000_0838(param_1: *mut Struct142) {
    let puVar1: *mut u32;
    Struct143 * *ppaVar2;
    let piVar3: *mut i32;
    let puVar4: *mut u16;
    let mut iVar5: i32;
    let mut uVar6: i32;
    let local_BX__1: *mut Struct91;
    let piVar7: *mut i32;
    let mut unaff_CS: u16;
    let mut bVar8: bool;
    let pvVar9: *mut void;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_1802631433d: *mut u16;
    let temp_5f1305d326: *mut Struct143;

    piVar7 = param_1.field_0x2;
    local_4 = piVar7;
    if (param_1.field_0x2 == 0x0) {}
    // goto LAB_1000_085b;
    loop {
        while {
            let pi_var7_val = unsafe { *piVar7 };
            if (pi_var7_val != 0) {
                iVar5 = piVar7[5];
                puVar4 = &PTR_LOOP_1050_000e;
                if (puVar4 != 0x0) {
                    &PTR_LOOP_1050_000e = unsafe { *puVar4 };
                    piVar3 = &PTR_LOOP_1050_000a;
                    unsafe {
                        *piVar3 = *piVar3 + 1;
                    }
                    param_1.field_0x2 = piVar7;
                    return CONCAT22(iVar5, puVar4);
                }
                unsafe { *piVar7 = 0 };
            }
            piVar7 = piVar7[2];
            piVar7 != local_4
        } {}
        // LAB_1000_085b:
        if (param_1.field_0x18 == 0) {
            invoke_error_handler_1000_1e61(unaff_CS, 4, param_1);
            return 0;
        }
        uVar6 = param_1.field_0x1a;
        while (
            local_6 = uVar6,
            pvVar9 = alloc_mem_1000_03c6(local_6, 1, param_1),
            pvVar9 == 0,
        ) {
            temp_5f1305d326 = param_1.field_0x1e;
            uVar6 = param_1.field_0x20 + CARRY2(temp_5f1305d326, local_6);
            puVar1 = &param_1.field_0x28;
            let pu_var1_val = unsafe { *puVar1 };
            bVar8 = pu_var1_val <= uVar6;
            if (bVar8) {
                if (bVar8 && uVar6 != pu_var1_val) {
                    return 0;
                }
                ppaVar2 = &param_1.field_0x26;
                if (*ppaVar2 <= temp_5f1305d326 + local_6 && temp_5f1305d326 + local_6 != *ppaVar2)
                {
                    return 0;
                }
            }
            uVar6 = local_6 >> 1;
            if (local_6 >> 1 < param_1.field_0x18 + 0x14) {
                iVar5 = invoke_error_handler_1000_1e61(unaff_CS, 2, param_1);
                uVar6 = local_6 & 0xfffe;
                if (iVar5 == 0) {
                    return 0;
                }
            }
        }
        piVar7 = param_1.field_0x2;
        local_4 = piVar7[2];
    }
}

pub fn free_mem_1000_093a(param_1: *mut i32, param_2: u16) -> u16 {
    let piVar1: *mut i32;
    let mut unaff_CS: u16;

    if (&PTR_LOOP_1050_000c != -0x352f) {
        invoke_error_handler_1000_1e61(unaff_CS, 0xe, 0);
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

pub fn alloc_mem_1000_0a48(uparam_1: i32, param_2: *mut Struct89, param_3: u16, param_4: u32) {
    let paVar1: *mut Struct140;
    let mut uVar2: i32;
    let in_CX: *mut Struct92;
    let mut uVar3: i32;
    let mut uVar4: u16;
    let local_DI_113: *mut Struct140;
    let mut uVar5: u16;
    let mut unaff_CS: u16;
    let mut uVar6: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_4 >> 0x10);
    if ((param_4 + 0x14) == -0x4153) {
        if ((param_3 == 0) && (param_2 <= (s_version__d__d_1050_0012 + 6))) {
            if (param_2 == 0x0) {
                invoke_error_handler_1000_1e61(unaff_CS, 4, param_4);
                uVar6 = 0;
            } else {
                uVar6 = alloc_mem_1000_0838(0x0);
                uVar4 = (uVar6 >> 0x10);
                local_DI_113 = uVar6;
                if ((uVar6 != 0) && ((param_1 & 1) != 0)) {
                    uVar2 = (s_version__d__d_1050_0012 + 6);
                    uVar3 = uVar2 >> 1;
                    while (uVar3 != 0) {
                        uVar3 = uVar3 - 1;
                        paVar1 = local_DI_113;
                        local_DI_113 = &local_DI_113.field_0x2;
                        paVar1 = 0;
                    }
                    if ((uVar2 & 1) != 0) {
                        unsafe { *local_DI_113 = 0 };
                    }
                }
            }
        } else {
            if ((param_3 == 0) && (param_2 <= (s_version__d__d_1050_0012 + 10))) {
                uVar6 = alloc_mem_1000_0b20(param_1 & 0xfffd, 0x0, in_CX, param_2);
            } else {
                uVar6 = alloc_mem_1000_05e2(param_2, param_3, 0x0);
            }
        }
        return uVar6;
    }
    invoke_error_handler_1000_1e61(unaff_CS, 10, 0);
    return 0;
}

// WARNING: Type propagation algorithm not settling

pub fn alloc_mem_1000_0b20(
    param_1: u16,
    param_2: *mut Struct93,
    param_3: *mut Struct92,
    param_4: u16,
) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let paVar3: *mut Struct89;
    let mut uVar4: i32;
    let mut alloc_data: u16;
    let local_CX_30: *mut Struct92;
    let paVar5: *mut Struct89;
    let local_BX__1: *mut Struct93;
    let mut uVar6: u16;
    let mut bVar7: bool;
    let mut uVar8: u32;
    let pvVar9: *mut void;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar6 = SUB42(&g_alloc_addr_1050_1050, 0);
    uVar2 = param_1 & 2;
    uVar4 = param_4 + 5 & 0xfffc;
    // local_CX_30 = (uVar4 - 8 & ~(-(uVar4 < 8)));
    paVar5 = &local_CX_30.field_0x8;
    paVar3 = (&local_BX__1.field_0x0 + uVar2 * 2);
    local_14 = param_1;
    local_6 = paVar3;
    if (paVar3 == 0x0) {}
    // goto LAB_1000_0b64;
    loop {
        while {
            if ((paVar5 <= paVar3)
                && (
                    pvVar9 = pass1_fn_1000_0c32(paVar5, 0x0, local_14),
                    pvVar9 != 0,
                ))
            {
                (&local_BX__1.field_0x0 + uVar2 * 2) = paVar3;
                return pvVar9;
            }
            paVar3 = &paVar3.field_0x4;
            paVar3 != local_6
        } {}
        // LAB_1000_0b64:
        if (((((local_14 & 2) == 0) || ((local_14 & 0x40) != 0))
            || (local_BX__1.field_0x32 == 0x0))
            || (paVar3 = (*local_BX__1.field_0x32)(), paVar3 < paVar5))
        {
            if (((local_14 & 0x10) != 0)
                || (
                    pvVar9 = alloc_mem_1000_03c6(local_BX__1.field_0x1a, uVar2, local_BX__1),
                    pvVar9 == 0,
                ))
            {
                if ((local_14 & 0x20) == 0) {
                    uVar2 = &local_CX_30[1].field_0x7 & 0xf000;
                    puVar1 = &local_BX__1.field_0x1e;
                    unsafe {
                        uVar4 = uVar2 + *puVar1;
                        uVar2 = local_BX__1.field_0x20 + CARRY2(uVar2, *puVar1);
                        puVar1 = &local_BX__1.field_0x28;
                        bVar7 = uVar2 < *puVar1;
                        if ((bVar7 || uVar2 == *puVar1)
                            && (bVar7
                                || (
                                    puVar1 = &local_BX__1.field_0x26,
                                    uVar4 < *puVar1 || uVar4 == *puVar1,
                                )))
                        {
                            uVar8 = alloc_mem_1000_05e2(paVar5, 0, local_BX__1);
                            return uVar8;
                        }
                    }
                }
                return 0;
            }
        } else {
            local_14 = local_14 | 0x40;
        }
        paVar3 = (&local_BX__1.field_0x0 + uVar2 * 2);
        local_6 = &paVar3.field_0x4;
    }
    {}
}

// WARNING: Could not reconcile some variable overlaps

pub fn alloc_mem_1000_0ed4(
    uparam_1: i32,
    uparam_2: i32,
    uparam_3: i32,
    param_4: *mut u16,
    param_5: u16,
) -> libc::c_long {
    let puVar1: *mut u16;
    let p_uvar2: *mut u16;
    let puVar3: *mut u16;
    let mut uVar4: i32;
    let puVar5: *mut u16;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((&PTR_LOOP_1050_000c & 0xfff8) == 0xcad0) {
        local_6 = 0x0;
        local_4 = &dos_alloc_addr_1050_0002;
        if ((param_1 & 8) == 0) {
            puVar3 = &param_4;
        } else {
            puVar3 = 0x0;
            unaff_SS = 0;
        }
        _local_c = CONCAT22(unaff_SS, puVar3);
        local_8 = alloc_mem_1000_0fb8(param_1, puVar3, unaff_SS);
        if (local_8 == 0) {
            return CONCAT22(param_5, param_4);
        }
        if ((param_1 & 8) == 0) {
            _local_c = alloc_mem_1000_0a48(param_1, param_2, param_3, local_6, local_4);
            uVar6 = (_local_c >> 0x10);
            puVar3 = _local_c;
            if (_local_c != 0) {
                uVar4 = local_8 >> 1;
                puVar5 = param_4;
                while (uVar4 != 0) {
                    uVar4 = uVar4 - 1;
                    puVar2 = puVar3;
                    puVar3 = puVar3 + 1;
                    puVar1 = puVar5;
                    puVar5 = puVar5 + 1;
                    unsafe { *puVar2 = *puVar1 };
                }
                uVar4 = ((local_8 & 1) != 0);
                while (uVar4 != 0) {
                    uVar4 = uVar4 - 1;
                    puVar2 = puVar3;
                    puVar3 = (puVar3 + 1);
                    puVar1 = puVar5;
                    puVar5 = (puVar5 + 1);
                    unsafe { *puVar2 = *puVar1 };
                }
                error_check_1000_0dc6(param_4, param_5);
            }
            return _local_c;
        }
        if ((param_3 | param_2) == 0) {
            return 0;
        }
        uVar6 = 5;
        uVar7 = local_6;
    } else {
        uVar6 = 0xe;
        uVar7 = 0;
    }
    invoke_error_handler_1000_1e61(unaff_CS, uVar6, uVar7);
    return 0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn alloc_mem_1000_0fb8(uparam_1: i32, param_2: *mut u16, uparam_3: i32) -> u32 {
    let puVar1: *mut u16;
    let mut bVar2: u8;
    let mut in_AX: i32;
    let mut uVar3: i32;
    let mut uVar4: u16;
    let mut iVar5: i32;
    let mut uVar6: i32;
    let mut in_DX: i32;
    let mut in_BX: i32;
    let puVar7: *mut u16;
    let mut unaff_SI: u16;
    let mut unaff_DI: u16;
    let mut uVar8: u16;
    let mut unaff_CS: u16;
    let mut uVar9: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((in_DX | in_AX) == 0) {
        invoke_error_handler_1000_1e61(unaff_CS, 4, PTR_LOOP_1050_0000);
        if ((param_3 | param_2) != 0) {
            param_2[1] = 0;
            unsafe { *param_2 = 0 };
            return 0;
        }
        return 1;
    }
    bVar2 = PTR_LOOP_1050_000c & 7;
    uVar8 = (_PTR_LOOP_1050_0000 >> 0x10);
    if ((PTR_LOOP_1050_000c & 7) != 0) {
        if (bVar2 == 1) {
            uVar3 = (PTR_LOOP_1050_0000 + 0x18);
            if (in_DX != 0) {
                return uVar3;
            }
            if (in_AX <= uVar3) {
                return 0;
            }
            return uVar3;
        }
        if (bVar2 != 2) {
            if (bVar2 != 3) {
                if ((param_3 | param_2) != 0) {
                    param_2[1] = 0;
                    unsafe { *param_2 = 0 };
                    return 0;
                }
                return 1;
            }
            if ((((param_3 | param_2) != 0) && (in_DX == 0))
                && (in_AX <= (PTR_LOOP_1050_0000 + 0x1c)))
            {
                uVar9 = get_mem_sz_1000_1284(in_BX, &g_alloc_addr_1050_1050);
                if (((uVar9 >> 0x10) == 0) && (uVar9 <= in_AX)) {
                    return uVar9;
                }
                return in_AX;
            }
            iVar5 = alloc_mem_1000_0670(param_1, param_2, param_3);
            if (iVar5 != 0) {
                return 0;
            }
            if ((param_3 | param_2) != 0) {
                return 0;
            }
            return 1;
        }
    }
    uVar3 = (in_BX + -2) & 0x7ffc;
    local_4 = uVar3 - 2;
    if ((*(in_BX + -1) & 0x80) != 0) {
        local_4 = uVar3 - 6;
    }
    if ((((in_DX == 0) && (in_AX <= local_4))
        || (in_DX == 0 && (in_AX <= (PTR_LOOP_1050_0000 + 0x1c))))
        && (uVar4 = struct_fn_1000_115c(unaff_SI, unaff_DI), uVar4 != 0))
    {
        if ((param_1 & 1) != 0) {
            uVar3 = ((in_BX + -2) & 0x7ffc) - 2;
            if (local_4 < in_AX) {
                puVar7 = (local_4 + in_BX);
                iVar5 = -local_4;
            } else {
                if (uVar3 <= in_AX) {
                    return 0;
                }
                puVar7 = (in_AX + in_BX);
                iVar5 = -in_AX;
            }
            uVar3 = uVar3 + iVar5;
            uVar6 = uVar3 >> 1;
            while (uVar6 != 0) {
                uVar6 = uVar6 - 1;
                puVar1 = puVar7;
                puVar7 = puVar7 + 1;
                unsafe {
                    *puVar1 = 0;
                }
            }
            if ((uVar3 & 1) != 0) {
                unsafe {
                    *puVar7 = 0;
                }
            }
        }
        return 0;
    }
    return local_4;
}

pub fn get_mem_sz_1000_1284(param_1: u32) {
    let mut bVar1: u8;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let mut bVar4: u8;
    let mut uVar5: i32;
    let mut unaff_CS: u16;
    let mut bVar6: bool;
    let mut alloc_size: u32;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
        invoke_error_handler_1000_1e61(unaff_CS, 0xe, 0);
        return 0xffffffff;
    }
    bVar1 = *&PTR_LOOP_1050_000c;
    bVar4 = bVar1 & 7;
    if ((bVar1 & 7) != 0) {
        if (bVar4 == 1) {
            uVar3 = 0x0;
            return *(uVar3 + 0x18);
        }
        if (bVar4 != 2) {
            if (bVar4 != 3) {
                return 0xffffffff;
            }
            alloc_size = get_mem_sz_1000_1532(param_1._2_2_);
            return CONCAT22(
                (alloc_size >> 0x10) - (alloc_size < 0x14),
                alloc_size - 0x14,
            );
        }
    }
    uVar2 = (param_1 + -2);
    uVar5 = uVar2 & 0x7ffc;
    local_6 = uVar5 - 2;
    local_4 = 0;
    if ((uVar2 & 0x8000) != 0) {
        bVar6 = local_6 < 4;
        local_6 = uVar5 - 6;
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
    uparam_1: i32,
    uparam_2: i32,
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
    uparam_1: i32,
    uparam_2: i32,
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
    let mut uVar1: i32;
    let mut in_AX: i32;
    let mut alloc_addr_cpy: u16;
    let mut in_DX: i32;
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
    if ((in_DX < 9) && (in_DX < 8 || (in_AX == 0))) {
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
            uVar1 = local_8 & 1;
            local_8 = local_8 >> 1;
            local_a = local_a >> 1 | (uVar1 != 0) << 0xf;
            (in_DX < local_8) || (in_DX <= local_8 && (in_AX <= local_a))
        } {}
    }
    return out_alloc_addr << 0x10;
}

pub fn free_mem_1000_15ce(param_1: *mut u32, param_2: u16) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let mut uVar3: i32;
    let mut local_8: u16;
    let mut local_4: u16;

    uVar3 = param_2 | param_1;
    while (uVar3 != 0) {
        unsafe { puVar1 = *param_1 };
        uVar2 = param_1[1];
        GlobalDOSFree16(param_2);
        uVar3 = uVar2 | puVar1;
        param_2 = uVar2;
        param_1 = puVar1;
    }
    return;
}

pub fn alloc_mem_1000_167a(param_1: u16, param_2: u16) -> *mut u8 {
    let mut uVar1: i32;
    let mut mem_buf: u16;

    if ((g_u16_ptr_1050_5f2e | _g_Struct94_ptr_1) == 0) {
        uVar1 = struct_fn_1000_160a();
        if ((param_2 | uVar1) == 0) {
            return 0;
        }
    }
    mem_buf = alloc_mem_1000_0a48(0, param_1, 0, _g_Struct94_ptr_1, g_u16_ptr_1050_5f2e);
    return mem_buf;
}

pub fn alloc_mem_1000_16aa(uparam_1: i32, uparam_2: i32, param_3: u16) -> *mut libc::c_void {
    let pvVar1: *mut void;
    let mut in_DX: u16;

    if ((param_2 | param_1) == 0) {
        pvVar1 = alloc_mem_1000_167a(param_3, in_DX);
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
    uparam_1: i32,
    uparam_2: i32,
    uparam_3: i32,
    uparam_4: i32,
    uparam_5: i32,
) -> u16 {
    let mut iVar1: i32;
    let mut bVar2: bool;
    let lVar3: u32;
    let mut local_4: u16;

    if ((param_2 | param_1) == 0) {
        bVar2 = 0xfffe < param_1;
        param_1 = param_1 + 1;
        param_2 = param_2 + bVar2;
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
            if ((PTR_LOOP_1050_5f36 | func_ptr_1050_5f34) == 0) {
                if ((PTR_LOOP_1050_5f3e | func_ptr_1050_5f3c) == 0) {
                    return 0;
                }
                (*func_ptr_1050_5f3c)();
                // goto LAB_1000_1724;
            }
            iVar1 = (*func_ptr_1050_5f34)();
        } else {
            iVar1 = (*func_ptr_1050_5f38)();
        }
        if (iVar1 == 0) {
            return 0;
        }
    }
}

pub fn alloc_mem_1000_180c(param_1: u16) -> u16 {
    let mut uVar1: i32;
    let mut uVar2: u16;
    let mut in_DX: i32;

    if ((g_u16_ptr_1050_5f2e | _g_Struct94_ptr_1) == 0) {
        uVar1 = struct_fn_1000_160a();
        if ((in_DX | uVar1) == 0) {
            return 0;
        }
    }
    uVar2 = alloc_mem_1000_0a48(0, param_1, 0, _g_Struct94_ptr_1, g_u16_ptr_1050_5f2e);
    return uVar2;
}

// WARNING: Variable defined which should be unmapped: local_6

pub fn alloc_mem_1000_183c(uparam_1: i32, uparam_2: i32) -> u16 {
    let mut uVar1: u16;
    let puVar2: *mut u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = (param_2 * param_1);
    puVar2 = 0x0;
    if ((param_2 * param_1 >> 0x10) != 0) {
        return 0;
    }
    if (((g_u16_ptr_1050_5f2e | _g_Struct94_ptr_1) == 0)
        && (
            _g_Struct94_ptr_1 = struct_fn_1000_160a(uVar1, 0),
            g_u16_ptr_1050_5f2e = puVar2,
            (puVar2 | _g_Struct94_ptr_1) == 0,
        ))
    {
        return 0;
    }
    uVar1 = alloc_mem_1000_0a48(1, uVar1, 0, _g_Struct94_ptr_1, g_u16_ptr_1050_5f2e);
    return uVar1;
}

pub fn alloc_mem_1000_188e(uparam_1: i32, uparam_2: i32, param_3: i32) -> u16 {
    let mut uVar1: u16;

    if ((param_2 | param_1) == 0) {
        uVar1 = alloc_mem_1000_180c(param_3);
        return uVar1;
    }
    if (param_3 == 0) {
        error_check_1000_18d2(param_1, param_2);
        return 0;
    }
    uVar1 = alloc_mem_1000_0ed4(0, param_3, 0, param_1, param_2);
    return uVar1;
}

pub fn free_mem_1000_1b68(param_1: *mut Struct98, param_2: u16) -> bool {
    let BVar1: bool;
    let mut unaff_CS: u16;

    if (param_1.field_0x14 != -0x4153) {
        invoke_error_handler_1000_1e61(unaff_CS, 10, 0);
        return 0;
    }
    BVar1 = free_mem_1000_1b9a(0, CONCAT22(param_2, param_1));
    return BVar1;
}

// WARNING: Could not reconcile some variable overlaps

pub fn free_mem_1000_1b9a(param_1: u16, param_1_00: *mut Struct99) -> bool {
    let mut uVar1: i32;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let local_BX_5: *mut Struct99;
    let local_SI_34: *mut Struct100;
    let mut uVar6: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar6 = (param_1_00 >> 0x10);
    local_BX_5 = param_1_00;
    local_BX_5.field_0x14 = 0;
    local_4 = 0;
    while {
        local_SI_34 = (local_4 * 2);
        if (local_SI_34 != 0x0) {
            while {
                uVar3 = &local_SI_34.field_0x8;
                (uVar3 + 0xc) = 0;
                free_mem_1000_13ce(CONCAT22(local_SI_34.field_0xa, local_SI_34.field_0x8));
                local_SI_34 = local_SI_34.field_0x4;
                (local_4 * 2) != local_SI_34
            } {}
        }
        local_4 = local_4 + 1;
        local_4 < 5
    } {}
    uVar4 = local_BX_5.field_0x10;
    uVar5 = local_BX_5.field_0x12;
    while (true) {
        _local_8 = CONCAT22(uVar5, uVar4);
        if ((uVar5 | uVar4) == 0) {
            break;
        }
        uVar1 = *_local_8;
        uVar2 = (uVar4 + 2);
        free_mem_1000_13ce(CONCAT22(uVar5, uVar4));
        uVar4 = uVar1;
        uVar5 = uVar2;
    }
    pass1_fn_1000_20a2(param_1_00);
    free_mem_1000_13ce(param_1_00);
    return 1;
}

pub fn alloc_mem_1000_3c51() {
    let piVar1: *mut i32;
    let pcVar2: *mut libc::c_char;
    let handle: HGLOBAL16;
    let mut iVar3: i32;
    let piVar4: *mut i32;
    let in_AX: HGLOBAL16;
    let HVar5: HGLOBAL16;
    let mut in_CX: i32;
    let mut in_BX: i32;
    let piVar6: *mut i32;
    let pcVar7: *mut libc::c_char;
    let dVar8: u32;
    let in_stack_00000000: HGLOBAL16;
    let mut in_stack_00000008: i32;
    let pcVar9: *mut libc::c_char;

    if ((*(in_BX + 2) & 4) == 0) {
        handle = (in_BX + 6);
        HVar5 = GlobalReAlloc16(0x20, CONCAT12(in_AX == 0, in_AX), handle);
        if (HVar5 != 0) {
            if ((HVar5 != handle)
                || (
                    HVar5 = in_stack_00000000,
                    dVar8 = GlobalSize16(handle),
                    dVar8 == 0,
                ))
            {}
            // goto LAB_1000_289a;
            in_AX = HVar5;
            if ((*(in_CX + 2) & 4) != 0) {
                in_AX = HVar5 - 1;
                (in_CX + -2) = in_AX;
            }
        }
        return CONCAT22(in_AX, HVar5);
    }
    // LAB_1000_289a:
    pcVar9 = s_version__d__d_1050_0012;
    pass1_fn_1000_25a8();
    pass1_fn_1000_2913();
    pcVar9 = process_string_1000_28dc(pcVar9);
    if (pcVar9 != 0x0) {
        iVar3 = 9;
        if (unsafe { *pcVar9 } == 'M') {
            iVar3 = 0xf;
        }
        pcVar9 = pcVar9 + iVar3;
        iVar3 = 0x22;
        pcVar7 = pcVar9;
        while {
            if (iVar3 == 0) {
                break;
            }
            iVar3 = iVar3 + -1;
            pcVar2 = pcVar7;
            pcVar7 = pcVar7 + 1;
            let pc_var2_val = unsafe { *pcVar2 };
            pc_var2_val != '\r'
        } {}
        pcVar7[-1] = '\0';
    }
    FatalAppExit16(CONCAT22(0x1050, pcVar9), 0);
    FatalExit(0xff);
    piVar6 = (s___NMSG___1050_63f6 + 8);
    loop {
        piVar1 = piVar6;
        piVar6 = piVar6 + 1;
        unsafe { iVar3 = *piVar1 };
        piVar4 = piVar6;
        if ((iVar3 == in_stack_00000008) || (piVar4 = (iVar3 + 1), piVar4 == 0x0)) {
            return CONCAT22(in_stack_00000008, piVar4);
        }
        iVar3 = -1;
        while {
            if (iVar3 == 0) {
                break;
            }
            iVar3 = iVar3 + -1;
            piVar1 = piVar6;
            piVar6 = (piVar6 + 1);
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
    GlobalFree16(&g_alloc_addr_1050_1050);
    return;
}

pub fn alloc_mem_func_1000_40af(uparam_1: i32, param_2: i32, uparam_3: i32) -> *mut u16 {
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let pcVar3: *mut libc::c_char;
    let puVar4: *mut u16;
    let puVar5: *mut u16;
    let mut uVar6: i32;
    let mut handle: i32;
    let handle_00: HGLOBAL16;
    let mut iVar7: i32;
    let mut unaff_SI: i32;
    let puVar8: *mut u16;
    let pcVar9: *mut libc::c_char;
    let unaff_DI: HGLOBAL16;
    let puVar10: *mut u16;
    let unaff_CS: u8;
    let mut bVar11: bool;
    let SVar12: SEGPTR;
    let uVar13: u8;
    let pcVar14: *mut libc::c_char;
    let mut local_6: u16;

    loop {
        uVar6 = (param_1 * param_3);
        handle = param_2 * param_3 + (param_1 * param_3 >> 0x10);
        if ((handle | uVar6) != 0) {
            puVar8 = 0x0;
            if ((handle < 3) && (handle < 2 || (uVar6 == 0))) {
                if (handle == 0) {
                    uVar6 = uVar6 + 0xfff & 0xf000;
                    if (uVar6 == 0) {
                        handle = 1;
                    }
                } else {
                    if ((param_3 - 1 & param_3) != 0) {
                        puVar8 = ((handle << 0x10) % param_3);
                        bVar11 = CARRY2(uVar6, puVar8);
                        uVar6 = uVar6 + puVar8;
                        if (bVar11) {}
                        // goto LAB_1000_41aa;
                        handle = 1;
                    }
                }
            } else {
                if ((param_3 - 1 & param_3) != 0) {}
                // goto LAB_1000_41aa;
            }
            handle_00 = GlobalAlloc16(CONCAT13((handle >> 8), CONCAT12(handle, uVar6)), 0x20);
            if ((handle_00 != 0) && ((uVar6 & 1) != 0)) {
                SVar12 = GlobalLock16(handle_00);
                if ((SVar12 != 0) || (SVar12 == 0)) {
                    pcVar14 = s_version__d__d_1050_0012;
                    uVar13 = 0x12;
                    pass1_fn_1000_25a8(0x12);
                    pass1_fn_1000_2913(uVar13);
                    pcVar14 = process_string_1000_28dc(pcVar14);
                    if (pcVar14 == 0x0) {}
                    // goto LAB_1000_28cb;
                    iVar7 = 9;
                    let pc_var14_val = unsafe { *pcVar14 };
                    if (pc_var14_val == 'M') {
                        iVar7 = 0xf;
                    }
                    pcVar14 = pcVar14 + iVar7;
                    iVar7 = 0x22;
                    pcVar9 = pcVar14;
                    break;
                }
                handle_00 = pass1_fn_1000_422a((SVar12 >> 0x10), handle_00);
                if (handle_00 == 0) {
                    GlobalUnlock16(handle);
                    GlobalFree16(unaff_DI);
                    handle_00 = 0;
                }
            }
            unaff_CS = 0x38;
            if (handle_00 != 0) {
                puVar10 = 0x0;
                while (unaff_SI != 0) {
                    iVar7 = -0x8000;
                    while (iVar7 != 0) {
                        iVar7 = iVar7 + -1;
                        puVar4 = puVar10;
                        puVar10 = puVar10 + 1;
                        unsafe { *puVar4 = 0 };
                    }
                    handle_00 = handle_00 + 0x100;
                    unaff_SI = unaff_SI + -1;
                }
                if (unaff_DI != 0) {
                    while (unaff_DI != 0) {
                        unaff_DI = unaff_DI - 1;
                        puVar4 = puVar10;
                        puVar10 = (puVar10 + 1);
                        unsafe { *puVar4 = 0 };
                    }
                }
                return puVar8;
            }
        }
        // LAB_1000_41aa:
        if ((PTR_LOOP_1050_618e | PTR_LOOP_1050_618c) == 0) {
            return 0x0;
        }
        iVar7 = (*PTR_LOOP_1050_618c)(unaff_CS, param_3, param_1, param_2);
        if (iVar7 == 0) {
            return 0x0;
        }
    }
    while (true) {
        iVar7 = iVar7 + -1;
        pcVar3 = pcVar9;
        pcVar9 = pcVar9 + 1;
        let pc_var3_val = unsafe { *pcVar3 };
        if (pc_var3_val == '\r') {
            break;
        }
        if (iVar7 == 0) {
            break;
        }
    }
    pcVar9[-1] = '\0';
    // LAB_1000_28cb:
    FatalAppExit16(CONCAT13(0x10, CONCAT12(0x50, pcVar14)), 0);
    FatalExit(0xff);
    puVar8 = (s___NMSG___1050_63f6 + 8);
    loop {
        puVar1 = puVar8;
        puVar8 = puVar8 + 1;
        unsafe { uVar2 = *puVar1 };
        puVar5 = puVar8;
        if ((uVar2 == local_6) || (puVar5 = (uVar2 + 1), puVar5 == 0x0)) {
            return puVar5;
        }
        iVar7 = -1;
        while {
            if (iVar7 == 0) {
                break;
            }
            iVar7 = iVar7 + -1;
            puVar1 = puVar8;
            puVar8 = (puVar8 + 1);
            let pi_var1_val = unsafe { *puVar1 };
            pi_var1_val != '\0'
        } {}
    }
}

pub fn copy_mem_1008_4274(in_Struct101: *mut Struct101) {
    let mut u_var1: u32;
    let in_AX: *mut Struct102;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let struct_a: *mut Struct199;
    let mut extraout_DX_01: u16;
    let local_Struct101: *mut Struct101;
    let mut a_struct_102: u16;
    let mut uVar2: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (in_Struct101 >> 0x10);
    local_Struct101 = in_Struct101;
    if (local_Struct101.field_0x6 != 0) {
        get_mem_sz_1000_1284(local_Struct101.field_0x6);
        a_struct_102 = in_AX;
        alloc_mem_1000_0a48(1, in_AX, extraout_DX, __g_Struct94_ptr_1);
        _local_a = CONCAT22(extraout_DX_00, a_struct_102);
        struct_a = (extraout_DX_00 | a_struct_102);
        if (struct_a != 0x0) {
            uVar1 = local_Struct101.field_0x6;
            hmemcpy16(
                CONCAT22(in_AX, 0x1000),
                CONCAT13((uVar1 >> 8), CONCAT12(uVar1, extraout_DX)),
            );
            process_struct_1000_179c(0x1e, struct_a);
            if ((struct_a | a_struct_102) == 0) {
                a_struct_102 = 0;
                uVar2 = 0;
            } else {
                set_struct_1008_4016(a_struct_102);
                uVar2 = extraout_DX_01;
            }
            (a_struct_102 + 6) = _local_a;
            process_struct_1008_47cc(CONCAT22(uVar2, a_struct_102));
            process_struct_1008_4834(CONCAT22(uVar2, a_struct_102));
            (a_struct_102 + 0x1c) = 1;
            return;
        }
    }
    return;
}

pub fn copy_mem_1008_676e(param_1: u32) {
    let mut u_var1: u32;
    let local_AX__1: *mut Struct111;
    let mut count: u32;
    let in_DX: *mut Struct199;
    let mut uVar2: i32;
    let local_BX_4: *mut Struct109;
    let local_BX_42: *mut Struct110;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x6 == 0) {
        return;
    }
    process_struct_1000_179c(0x1e, in_DX);
    uVar2 = in_DX | local_AX__1;
    if (uVar2 == 0) {
        local_AX__1 = 0x0;
        uVar2 = 0;
    } else {
        uVar1 = local_BX_4.field_0x10;
        uVar4 = (uVar1 >> 0x10);
        local_BX_42 = uVar1;
        pass1_1008_6604(
            CONCAT22(in_DX, local_AX__1),
            local_BX_42.field_0x8,
            local_BX_42.field_0x4,
        );
    }
    pass1_fn_1000_48a8(local_AX__1.field_0x10, local_BX_4.field_0x10, 0x28);
    uVar1 = local_AX__1.field_0x10;
    count = (uVar1 + 8) * local_BX_4.field_0x18;
    hmemcpy16(
        CONCAT22(count, 0x1000),
        CONCAT22(local_BX_4.field_0x6, (count >> 0x10)),
    );
    local_AX__1.field_0x1c = 1;
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn alloc_mem_1008_909c(in_struct_1: *mut Struct210) {
    let puVar1: *mut u32;
    let local_AX_23: *mut u8;
    let mut uVar2: i32;
    let extraout_DX: *mut u16;
    let mut iVar3: i32;
    let mut local_ES_4: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_ES_4 = (in_struct_1 >> 0x10);
    iVar3 = in_struct_1;
    if ((iVar3 + 0x12) == 0) {
        local_AX_23 = (iVar3 + 0xe);
        g_u16_ptr_1050_5f2e = (iVar3 + 0x10);
    } else {
        uVar2 = (iVar3 + 0x12);
        puVar1 = (iVar3 + 0x16);
        let pu_var1_val = unsafe { *puVar1 };
        local_AX_23 = (uVar2 + pu_var1_val);
        g_u16_ptr_1050_5f2e = ((iVar3 + 0x14) + (iVar3 + 0x18) + CARRY2(uVar2, pu_var1_val));
    }
    _local_6 = CONCAT22(g_u16_ptr_1050_5f2e, local_AX_23);
    if ((iVar3 + 6) == 0) {
        if (__g_Struct94_ptr_1 == 0) {
            _g_Struct94_ptr_1 = local_AX_23;
            struct_fn_1000_160a();
        } else {
        }
        uVar2 = local_AX_23 << 2;
        alloc_mem_1000_1708(uVar2, 0, 1, _g_Struct94_ptr_1, g_u16_ptr_1050_5f2e);
    } else {
        uVar2 = local_AX_23 * 4;
        alloc_mem_1000_0ed4(
            1,
            uVar2,
            (g_u16_ptr_1050_5f2e * 2 + CARRY2(local_AX_23, local_AX_23)) * 2
                + CARRY2(local_AX_23 * 2, local_AX_23 * 2),
            (iVar3 + 6),
        );
        g_u16_ptr_1050_5f2e = extraout_DX;
    }
    local_a = CONCAT22(g_u16_ptr_1050_5f2e, uVar2);
    if ((g_u16_ptr_1050_5f2e | uVar2) != 0) {
        (iVar3 + 0x12) = _local_6;
        (iVar3 + 6) = local_a;
    }
    return;
}

pub fn free_mem_1008_ad0c(param_1: u16, param_2: u8) {
    let in_stack_00000007: u8;
    let mut in_stack_00000008: u8;

    _param_1 = CONCAT13(in_stack_00000007, CONCAT12(param_2, param_1));
    *_param_1 = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((in_stack_00000008 & 1) != 0) {
        free_mem_1000_093a(_param_1);
    }
    return _param_1 & 0xffff0000 | CONCAT12(param_2, param_1) & 0xffff;
}

pub fn pass1_1030_9e9c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    unsafe { *param_1 = s_1_1050_389a };
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        free_mem_1000_093a(param_1);
    }
    return param_1;
}
