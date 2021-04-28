pub unsafe fn pass1_1038_c410(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_be4a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_be76(param_1: *mut u8, param_2: *mut u8) {
    let paVar1: *mut AStruct318;
    let BVar2: bool;

    if (param_2._2_2_ == 0) {
        BVar2 = 0;
        paVar1 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, 0x2b);
        pass1_1010_038e(paVar1, BVar2);
    }
    destroy_win_1040_7b98(param_1, param_2);
    return;
}

pub unsafe fn pass1_1038_be4a(param_1: *mut AStruct599) {
    let local_ES_3: *mut u8;

    local_ES_3 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xc436;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_AStruct112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_bd4a(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_b7f0(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_bca8(param_1: *mut u8) {
    let mut u_var1: i32;
    let ppcVar2: fn();
    let mut u_var3: u32;
    let pu_var4: *mut u32;
    let pu_var5: *mut u32;
    let extraout_DX: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let extraout_DX_00: *mut AStruct199;
    let mut extraout_DX_01: u16;
    let mut u_var6: u16;
    let mut iVar7: i32;
    let mut u_var8: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var8 = (param_1 >> 0x10);
    iVar7 = param_1;
    u_var3 = (iVar7 + 0x8e);
    pu_var5 = (u_var3 + 10);
    unsafe {
        ppcVar2 = (*pu_var5 + 0x14);
    }
    ppcVar2();
    struct_a = extraout_DX;
    pu_var4 = pu_var5;
    if ((iVar7 + 0x70) != 0) {
        pu_var4 = (iVar7 + 0x70);
        u_var1 = (iVar7 + 0x72);
        struct_a = (u_var1 | pu_var4);
        if (struct_a != 0x0) {
            unsafe {
                ppcVar2 = *pu_var4;
            }
            ppcVar2();
            struct_a = extraout_DX_00;
        }
    }
    process_struct_1000_179c(0x14, struct_a);
    if ((struct_a | pu_var4) == 0) {
        pu_var4 = 0x0;
        u_var6 = 0;
    } else {
        process_struct_1008_4c58(CONCAT22(struct_a, pu_var4));
        u_var6 = extraout_DX_01;
    }
    (iVar7 + 0x70) = pu_var4;
    (iVar7 + 0x72) = u_var6;
    pass1_1008_4d84((iVar7 + 0x70), pu_var5);
    return;
}

pub unsafe fn pass1_1038_b7f0(param_1: *mut AStruct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xbd70;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_AStruct112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_b6e0(in_astruct: *mut AStruct112, param_2: *mut u8) {
    let mut u_var1: u32;
    let in_AX: *mut u8;
    let local_astruct: *mut AStruct112;
    let mut u_var2: u16;
    let mut local_4: u16;

    local_4 = 1;
    loop {
        if (0x2a < local_4) {
            return in_AX;
        }
        u_var2 = (in_astruct >> 0x10);
        local_astruct = in_astruct;
        in_AX = ((local_astruct + local_4 * 4 + 2) | (local_astruct + local_4 * 4));
        let uvar1_6_val = unsafe { *(u_var1 + 6) };
        if ((in_AX != 0x0)
            && (
                u_var1 = (local_astruct + local_4 * 4),
                in_AX = param_2,
                uvar1_6_val == param_2,
            ))
        {
            break;
        }
        local_4 = local_4 + 1;
    }
    (local_astruct + local_4 * 4) = 0;
    return '\0';
}

pub unsafe fn pass1_1038_ae28(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_ae08(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_ad4c(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_abb0(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_adde(
    param_1: i32,
    param_2: *mut u8,
    param_3: *mut u8,
    param_3_00: *mut u8,
) {
    pass1_1038_9b72(param_1, param_2, param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xae4e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1038_abb0(param_1: *mut AStruct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xad72;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_AStruct112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_aaf0(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_a8cc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_a80c(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_a6c8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_a89e(param_1: *mut u16, param_2: u16) {
    let pu_var1: *mut u8;

    pu_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, pu_var1, 1, 0, CONCAT22(param_2, 0xfca));
    unsafe {
        *param_1 = 0xab16;
        (param_1 + 2) = &PTR_LOOP_1050_1038;
    }
    return param_1;
}

pub unsafe fn pass1_1038_a8cc(param_1: *mut AStruct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xab16;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_AStruct112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}

pub unsafe fn pass1_1038_a608(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_a4c2(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_a69a(param_1: *mut u16, param_2: *mut u8) {
    let pu_var1: *mut u8;

    pu_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, pu_var1, 1, 0, CONCAT22(param_2, 0xfc9));
    unsafe {
        *param_1 = 0xa832;
        (param_1 + 2) = &PTR_LOOP_1050_1038;
    }
    return param_1;
}

pub unsafe fn pass1_1038_a6c8(param_1: *mut AStruct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa832;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_AStruct112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}

pub unsafe fn pass1_1038_a494(param_1: *mut u16, param_2: u16) {
    let pu_var1: *mut u8;

    pu_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, pu_var1, 1, 0, CONCAT22(param_2, 0xfc8));
    unsafe {
        *param_1 = 0xa62e;
        (param_1 + 2) = &PTR_LOOP_1050_1038;
    }
    return param_1;
}

pub unsafe fn pass1_1038_a4c2(param_1: *mut AStruct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa62e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_AStruct112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}

pub unsafe fn pass1_1038_a402(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_a36a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_a2aa(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_a156(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_a33c(param_1: *mut u16, param_2: *mut u8) -> *mut u16 {
    let pu_var1: *mut u8;

    pu_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, pu_var1, 1, 0, CONCAT22(param_2, 0xfc7));
    unsafe {
        *param_1 = 0xa428;
        (param_1 + 2) = &PTR_LOOP_1050_1038;
    }
    return param_1;
}

pub unsafe fn pass1_1038_a36a(param_1: *mut AStruct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa428;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_AStruct112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}

pub unsafe fn pass1_1038_a174(param_1: *mut u8, param_2: i32) {
    if (param_2 == 1) {
        (param_1 + 0x8e) = 0;
    }
    return;
}

pub unsafe fn pass1_1038_a090(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_9fa4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_9fa4(param_1: *mut AStruct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa0b6;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_AStruct112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_9f56(param_1: *mut AStruct44) -> *mut AStruct44 {
    byte * *ppbVar1;
    let pbVar2: *mut u8;
    let mut bVar3: u8;
    let mut bVar4: u8;
    let mut cVar5: u8;
    let mut u_var6: i32;
    let mut in_CL: u8;
    let mut bVar7: u8;
    let mut in_DX: i32;
    let mut bVar8: u8;
    let mut in_BX: i32;
    let mut bVar9: u8;
    let pu_var10: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut u8;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let in_struct_1: *mut AStruct44;
    let in_stack_0000bf2a: *mut u8;
    let mut bStack78: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    pu_var10 = &stack0xfffe;
    cVar5 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        pu_var10 = pu_var10 + -1;
        unsafe { *pu_var10 = *unaff_BP };
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    bVar9 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar9;
    bVar8 = (in_DX + 1 >> 8);
    bVar3 = bVar8 + bVar9;
    bVar11 = CARRY1(bVar8, bVar9) || CARRY1(bVar3, in_CF);
    u_var6 = in_DX + 1 & 0xff;
    in_struct_1 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pbVar2 = unaff_SI + in_BX;
    bVar7 = u_var6;
    unsafe {
        bVar8 = *pbVar2;
        bVar4 = *pbVar2 - bVar7;
        bVar12 = *pbVar2 < bVar7 || bVar4 < bVar11;
        *pbVar2 = bVar4 - bVar11;
        if ((*pbVar2 != 0)
            && (SBORROW1(bVar8, bVar7) != SBORROW1(bVar4, bVar11)) == (*pbVar2 < '\0'))
        {
            pbVar2 = unaff_SI;
            bVar11 = CARRY1(*pbVar2, bVar9) || CARRY1(*pbVar2 + bVar9, bVar12);
            *pbVar2 = *pbVar2 + bVar9 + bVar12;
            bVar8 = bStack78 + in_BX;
            bVar12 = CARRY1(bStack78, in_BX) || CARRY1(bVar8, bVar11);
            bStack78 = bVar8 + bVar11;
            pbVar2 = unaff_SI + -0x4f;
            bVar8 = *pbVar2;
            bVar4 = *pbVar2;
            *pbVar2 = bVar4 + bVar9 + bVar12;
            pbVar2 = unaff_SI + -0x78;
            *pbVar2 = *pbVar2 + in_CL + (CARRY1(bVar8, bVar9) || CARRY1(bVar4 + bVar9, bVar12));
            puStack34 = &stack0xfffe;
            process_struct_1040_7728(
                in_struct_1,
                (&PTR_LOOP_1050_0000 + 1),
                CONCAT22(u_var6 | (bVar3 + in_CF) << 8, in_BX),
                0xfba,
                in_stack_0000bf2a,
            );
            in_struct_1.ptr_a_lo = 0xa0b6;
            (in_struct_1 + 2) = &PTR_LOOP_1050_1038;
            return in_struct_1;
        }
        ppbVar1 = (unaff_SI + 9);
        *ppbVar1 = unaff_SI + *ppbVar1;
    }
    error_check_1000_17ce(param_1);
    return param_1;
}

pub unsafe fn pass1_1038_9f02(param_1: *mut AStruct44) -> *mut AStruct44 {
    let pbVar1: *mut u8;
    byte * *ppbVar2;
    let pcVar3: *mut libc::c_char;
    let pu_var4: *mut u32;
    let mut bVar5: u8;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut bVar8: u8;
    let mut cVar9: u8;
    let mut in_AX: i32;
    let mut bVar10: u8;
    let mut cVar12: u8;
    let mut in_CX: u16;
    let mut bVar13: u8;
    let mut bVar14: u8;
    let mut in_DX: i32;
    let mut bVar16: u8;
    let mut iVar15: i32;
    let mut bVar17: u8;
    let mut in_BX: u16;
    let pu_var18: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut u8;
    let mut unaff_ES: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar19: bool;
    let mut bVar20: bool;
    let mut bVar21: bool;
    let mut in_AF: u8;
    let mut in_TF: u8;
    let mut in_IF: u8;
    let mut in_NT: u8;
    let mut in_stack_0000007c: u16;
    let mut bStack007d: u8;
    let in_struct_1: *mut AStruct68;
    let local_4e: u8;
    let puStack34: *mut u8;
    let mut bVar11: u8;

    puStack34 = &stack0xfffe;
    pu_var18 = &stack0xfffe;
    cVar12 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        pu_var18 = pu_var18 + -1;
        unsafe { *pu_var18 = *unaff_BP };
        cVar12 = cVar12 + -1;
        '\0' < cVar12
    } {}
    bVar11 = (in_AX >> 8);
    bVar13 = (in_CX >> 8);
    bVar17 = bVar11 + bVar13;
    bVar10 = bVar17 + in_CF;
    bVar14 = in_DX;
    bVar19 = CARRY1(bStack007d, bVar14)
        || CARRY1(
            bStack007d + bVar14,
            unaff_SI[CONCAT11((in_BX >> 8), 0x40)] < bVar14,
        );
    pbVar1 = &stack0x4079 + unaff_SI;
    bVar16 = (in_DX >> 8);
    unsafe {
        bVar20 = CARRY1(*pbVar1, bVar16) || CARRY1(*pbVar1 + bVar16, bVar19);
        *pbVar1 = *pbVar1 + bVar16 + bVar19;
        pbVar1 = unaff_SI;
        bVar5 = *pbVar1;
        bVar8 = *pbVar1 + 0x40;
        bVar19 = 0xbf < *pbVar1 || CARRY1(bVar8, bVar20);
        *pbVar1 = bVar8 + bVar20;
        cVar12 = in_CX;
        if ((*pbVar1 == 0) || (SCARRY1(bVar5, '@') != SCARRY1(bVar8, bVar20)) != (*pbVar1 < '\0')) {
            pbVar1 = unaff_SI + 0x3fc8;
            *pbVar1 = *pbVar1 + cVar12 + bVar19;
            cVar9 = PTR_LOOP_1050_4080;
            PTR_LOOP_1050_4080._0_1_ = '@';
            pu_var18 = CONCAT11(cVar9, 0x40);
            pbVar1 = unaff_SI;
            *pbVar1 = *pbVar1 + cVar12 + (unaff_SI[0x4040] < bVar14);
            pcVar3 = (pu_var18 + unaff_SI + 0x10);
            *pcVar3 = *pcVar3 + 'T';
            pcVar3 = (pu_var18 + unaff_SI + 0x10);
            *pcVar3 = *pcVar3 + -0x34;
            pu_var4 = (pu_var18 + unaff_SI + 0x10);
            u_var6 = *pu_var4;
            *pu_var4 = *pu_var4 + 0x81b6;
            pu_var4 = (pu_var18 + unaff_SI + 0x10);
            u_var7 = *pu_var4;
            *pu_var4 = *pu_var4 + 0x60ea;
            pcVar3 = (pu_var18 + unaff_SI);
            *pcVar3 = (*pcVar3 - (in_DX & 0xff)) - (0x9f15 < u_var7);
            iVar15 = (in_DX & 0xff | (bVar16 + cVar9 + (0x7e49 < u_var6)) << 8) - 1;
            pcVar3 = (pu_var18 + unaff_SI + 0x10);
            *pcVar3 = *pcVar3 + 'f';
            pbVar1 = (pu_var18 + unaff_SI + 0x10);
            bVar5 = *pbVar1;
            *pbVar1 = *pbVar1 - 0x22;
            if (-1 < *pbVar1) {
                bVar17 = (iVar15 >> 8);
                pcVar3 = (pu_var18 + unaff_SI);
                *pcVar3 = (*pcVar3 - iVar15)
                    - (CARRY1(bVar17, bVar13) || CARRY1(bVar17 + bVar13, 0x21 < bVar5));
                loop {
                    // WARNING: Do nothing block with infinite loop
                }
            }
        } else {
            bVar20 = 0xbf < bVar16 || CARRY1(bVar16 + 0x40, bVar19);
            in_struct_1 = CONCAT22(&stack0xc73f, &stack0xfffe);
            pbVar1 = unaff_SI + 0x4040;
            bVar14 = (in_DX & 0xff);
            bVar5 = *pbVar1;
            bVar8 = *pbVar1 - bVar14;
            bVar21 = *pbVar1 < bVar14 || bVar8 < bVar20;
            *pbVar1 = bVar8 - bVar20;
            if ((*pbVar1 == 0)
                || (SBORROW1(bVar5, bVar14) != SBORROW1(bVar8, bVar20)) != (*pbVar1 < '\0'))
            {
                ppbVar2 = (unaff_SI + 9);
                *ppbVar2 = unaff_SI + *ppbVar2;
                error_check_1000_17ce(param_1);
                return param_1;
            }
            pbVar1 = unaff_SI;
            bVar20 = 0xbf < *pbVar1 || CARRY1(*pbVar1 + 0x40, bVar21);
            *pbVar1 = *pbVar1 + 0x40 + bVar21;
            bVar21 = 0xbf < local_4e || CARRY1(local_4e + 0x40, bVar20);
            local_4e = local_4e + 0x40 + bVar20;
            pbVar1 = unaff_SI + -0x4f;
            bVar5 = *pbVar1;
            bVar8 = *pbVar1;
            *pbVar1 = bVar8 + 0x40 + bVar21;
            pbVar1 = unaff_SI + -0x78;
            *pbVar1 = *pbVar1 + cVar12 + (0xbf < bVar5 || CARRY1(bVar8 + 0x40, bVar21));
            puStack34 = &stack0xfffe;
            process_struct_1040_7728(
                in_struct_1,
                (&PTR_LOOP_1050_0000 + 1),
                CONCAT22(in_DX & 0xff | (bVar16 + 0x40 + bVar19) << 8, 0x4040),
                0xfba,
                ((in_NT & 1) * 0x4000
                    | (SCARRY1(bVar11, bVar13) != SCARRY1(bVar17, in_CF)) * 0x800
                    | (in_IF & 1) * 0x200
                    | (in_TF & 1) * 0x100
                    | ((in_AX & 0xff | bVar10 << 8) < 0) * 0x80
                    | (bVar10 == 0) * 0x40
                    | (in_AF & 1) * 0x10
                    | ((POPCOUNT(bVar10) & 1) == 0) * 4
                    | (CARRY1(bVar11, bVar13) || CARRY1(bVar17, in_CF))),
            );
            unaff_ES = (in_struct_1 >> 0x10);
            pu_var18 = in_struct_1;
        }
        *pu_var18 = 0xa0b6;
    }
    pu_var18[1] = &PTR_LOOP_1050_1038;
    return CONCAT22(unaff_ES, pu_var18);
}

pub unsafe fn pass1_1038_9b72(
    param_1: i32,
    param_2: *mut u8,
    param_3: *mut u8,
    param_3_00: *mut u8,
) {
    let mut local_4: u16;

    pass1_1040_b040(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3_00, param_3),
        (param_3_00 >> 0x10),
    );
    (param_1 + 0x128) = 0;
    CONCAT22(param_2, param_1) = 0x9efa;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    local_4 = 0;
    while {
        (param_1 + local_4 * 2 + 0x94) = 0;
        local_4 = local_4 + 1;
        local_4 < 0x4a
    } {}
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1038_9b52(param_1: *mut AStruct44) -> *mut AStruct44 {
    let pbVar1: *mut u8;
    let mut bVar2: u8;
    let mut bVar3: u8;
    let mut cVar4: u8;
    let mut u_var5: i32;
    let mut in_CX: u16;
    let mut bVar6: u8;
    let mut in_DX: i32;
    let mut bVar7: u8;
    let mut in_BX: i32;
    let mut bVar8: u8;
    let pu_var9: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut u8;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar10: bool;
    let mut bVar11: bool;
    let mut iVar12: i32;
    let pu_var13: *mut u8;
    let pu_var14: *mut u8;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    pu_var9 = &stack0xfffe;
    pu_var13 = &stack0xfffe;
    cVar4 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        pu_var9 = pu_var9 + -1;
        unsafe { *pu_var9 = *unaff_BP };
        cVar4 = cVar4 + -1;
        '\0' < cVar4
    } {}
    pu_var14 = &stack0xbf2a;
    bVar8 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar8;
    bVar7 = (in_DX + 1 >> 8);
    bVar2 = bVar7 + bVar8;
    bVar10 = CARRY1(bVar7, bVar8) || CARRY1(bVar2, in_CF);
    u_var5 = in_DX + 1 & 0xff;
    pbVar1 = unaff_SI + in_BX;
    bVar6 = u_var5;
    unsafe {
        bVar7 = *pbVar1;
        bVar3 = *pbVar1 - bVar6;
        bVar11 = *pbVar1 < bVar6 || bVar3 < bVar10;
        *pbVar1 = bVar3 - bVar10;
        if ((*pbVar1 != 0)
            && (SBORROW1(bVar7, bVar6) != SBORROW1(bVar3, bVar10)) == (*pbVar1 < '\0'))
        {
            pbVar1 = unaff_SI;
            bVar10 = CARRY1(*pbVar1, bVar8) || CARRY1(*pbVar1 + bVar8, bVar11);
            *pbVar1 = *pbVar1 + bVar8 + bVar11;
            bVar7 = local_4e + in_BX;
            bVar11 = CARRY1(local_4e, in_BX) || CARRY1(bVar7, bVar10);
            local_4e = bVar7 + bVar10;
            pbVar1 = unaff_SI + -0x4f;
            bVar7 = *pbVar1;
            bVar3 = *pbVar1;
            *pbVar1 = bVar3 + bVar8 + bVar11;
            pbVar1 = unaff_SI + -0x78;
            *pbVar1 = *pbVar1 + in_CX + (CARRY1(bVar7, bVar8) || CARRY1(bVar3 + bVar8, bVar11));
            puStack34 = &stack0xfffe;
            pass1_1040_b040(
                CONCAT22(&stack0xbf2a, &stack0xfffe),
                CONCAT22(u_var5 | (bVar2 + in_CF) << 8, in_BX),
                in_CX,
            );
            (pu_var13 + 0x128) = 0;
            CONCAT22(pu_var14, pu_var13) = 0x9efa;
            (pu_var13 + 2) = &PTR_LOOP_1050_1038;
            iVar12 = 0;
            while {
                (pu_var13 + iVar12 * 2 + 0x94) = 0;
                iVar12 = iVar12 + 1;
                iVar12 < 0x4a
            } {}
            return CONCAT22(pu_var14, pu_var13);
        }
        if (*pbVar1 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return param_1;
}

pub unsafe fn pass1_1038_9ad0(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_9a48(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_9a1e(
    param_1: i32,
    param_2: *mut u8,
    param_3: *mut u8,
    param_3_00: *mut u8,
) -> *mut u32 {
    pass1_1040_b040(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3_00, param_3),
        (param_3_00 >> 0x10),
    );
    *CONCAT22(param_2, param_1) = 0x9af6;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1038_99fe(param_1: *mut AStruct44) -> *mut AStruct44 {
    byte * *ppbVar1;
    let pbVar2: *mut u8;
    let mut bVar3: u8;
    let mut bVar4: u8;
    let mut cVar5: u8;
    let mut u_var6: i32;
    let mut in_CX: u16;
    let mut bVar7: u8;
    let mut in_DX: i32;
    let mut bVar8: u8;
    let mut in_BX: i32;
    let mut bVar9: u8;
    let pu_var10: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut u8;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let pu_var13: *mut u8;
    let pu_var14: *mut u8;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    pu_var10 = &stack0xfffe;
    pu_var13 = &stack0xfffe;
    cVar5 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        pu_var10 = pu_var10 + -1;
        unsafe { *pu_var10 = *unaff_BP };
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    pu_var14 = &stack0xbf2a;
    bVar9 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar9;
    bVar8 = (in_DX + 1 >> 8);
    bVar3 = bVar8 + bVar9;
    bVar11 = CARRY1(bVar8, bVar9) || CARRY1(bVar3, in_CF);
    u_var6 = in_DX + 1 & 0xff;
    pbVar2 = unaff_SI + in_BX;
    bVar7 = u_var6;
    unsafe {
        bVar8 = *pbVar2;
        bVar4 = *pbVar2 - bVar7;
        bVar12 = *pbVar2 < bVar7 || bVar4 < bVar11;
        *pbVar2 = bVar4 - bVar11;
        if ((*pbVar2 != 0)
            && (SBORROW1(bVar8, bVar7) != SBORROW1(bVar4, bVar11)) == (*pbVar2 < '\0'))
        {
            pbVar2 = unaff_SI;
            bVar11 = CARRY1(*pbVar2, bVar9) || CARRY1(*pbVar2 + bVar9, bVar12);
            *pbVar2 = *pbVar2 + bVar9 + bVar12;
            bVar12 = CARRY1(local_4e, in_BX);
            bVar8 = local_4e + in_BX;
            local_4e = bVar8 + bVar11;
            pbVar2 = unaff_SI + -0x4f;
            *pbVar2 = *pbVar2 + bVar9 + (bVar12 || CARRY1(bVar8, bVar11));
            puStack34 = &stack0xfffe;
            pass1_1040_b040(
                CONCAT22(&stack0xbf2a, &stack0xfffe),
                CONCAT22(u_var6 | (bVar3 + in_CF) << 8, in_BX),
                in_CX,
            );
            (CONCAT22(pu_var14, pu_var13)).ptr_a_lo = 0x9af6;
            (pu_var13 + 2) = &PTR_LOOP_1050_1038;
            return CONCAT22(pu_var14, pu_var13);
        }
        ppbVar1 = (unaff_SI + 9);
        *ppbVar1 = unaff_SI + *ppbVar1;
    }
    error_check_1000_17ce(param_1);
    return param_1;
}

pub unsafe fn pass1_1038_993a(param_1: *mut u8, param_2: *mut u8, param_1_00: i32) -> i16 {
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = 0;
    loop {
        if (0xe < local_6) {
            return 0xffff;
        }
        if ((local_6 * 0xe + 0x5a70) == param_1_00) {
            break;
        }
        local_6 = local_6 + 1;
    }
    return local_6;
}

pub unsafe fn pass1_1038_90a2(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_8cf6(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_8cf6(param_1: *mut AStruct44) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x90c8;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_AStruct112_a, *(param_1 + 6));
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub unsafe fn pass1_1038_8c8a(param_1: *mut AStruct44, param_2: *mut u8) -> *mut AStruct44 {
    let pbVar1: *mut u8;
    let pcVar2: *mut libc::c_char;
    let mut bVar3: u8;
    let mut cVar4: u8;
    let mut u_var5: i32;
    let mut bVar6: u8;
    let mut in_DX: i32;
    let mut bVar7: u8;
    let mut in_BX: i32;
    let mut bVar10: u8;
    let mut iVar9: i32;
    let pu_var11: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut u8;
    let mut unaff_DI: i32;
    let mut u_var12: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar13: bool;
    let mut bVar14: bool;
    let ppVar15: *mut pass1_struct_1;
    let local_4e: u8;
    let mut bVar8: u8;

    pu_var11 = &stack0xfffe;
    cVar4 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        pu_var11 = pu_var11 + -1;
        unsafe {
            *pu_var11 = *unaff_BP;
        }

        cVar4 = cVar4 + -1;
        '\0' < cVar4
    } {}
    bVar10 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar10;
    bVar8 = (in_DX + 1 >> 8);
    bVar7 = bVar8 + bVar10;
    bVar13 = CARRY1(bVar8, bVar10) || CARRY1(bVar7, in_CF);
    bVar7 = bVar7 + in_CF;
    u_var5 = in_DX + 1 & 0xff;
    pbVar1 = unaff_SI + in_BX;
    bVar6 = u_var5;
    unsafe {
        bVar8 = *pbVar1;
        bVar3 = *pbVar1 - bVar6;
        bVar14 = *pbVar1 < bVar6 || bVar3 < bVar13;
        *pbVar1 = bVar3 - bVar13;
        if ((*pbVar1 != 0)
            && (SBORROW1(bVar8, bVar6) != SBORROW1(bVar3, bVar13)) == (*pbVar1 < '\0'))
        {
            pbVar1 = unaff_SI;
            bVar8 = *pbVar1;
            bVar3 = *pbVar1;
            *pbVar1 = bVar3 + bVar10 + bVar14;
            bVar13 = CARRY1(local_4e, in_BX)
                || CARRY1(
                    local_4e + in_BX,
                    CARRY1(bVar8, bVar10) || CARRY1(bVar3 + bVar10, bVar14),
                );
            pbVar1 = unaff_SI + -0x4f;
            bVar14 = CARRY1(*pbVar1, bVar10) || CARRY1(*pbVar1 + bVar10, bVar13);
            *pbVar1 = *pbVar1 + bVar10 + bVar13;
            bVar8 = bVar7 + bVar10;
            pcVar2 = (unaff_DI + -0x75);
            *pcVar2 = *pcVar2 + bVar6 + (CARRY1(bVar7, bVar10) || CARRY1(bVar8, bVar14));
            _in(u_var5 | (bVar8 + bVar14) << 8);
            process_struct_1040_b082(param_1, CONCAT22(param_2, 0x185a));
            u_var12 = (param_1 >> 0x10);
            iVar9 = param_1;
            (iVar9 + 0x94) = 0;
            param_1.ptr_a_lo = 0x90c8;
            (iVar9 + 2) = &PTR_LOOP_1050_1038;
            ppVar15 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_DI, 0x3f));
            (iVar9 + 0x94) = ppVar15;
            (iVar9 + 0x96) = (ppVar15 >> 0x10);
            return param_1;
        }
        if (*pbVar1 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return param_1;
}

pub unsafe fn pass1_1038_8c08(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_893a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_893a(param_1: *mut AStruct348) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x8c2e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_AStruct112_a, *(param_1 + 6));
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub unsafe fn pass1_1038_8850(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_7d5c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_801a(param_1: *mut u8) {
    let mut u_var1: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut u_var3: u32;
    let pu_var4: *mut u8;
    let mut in_stack_0000fff0: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar2 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(in_stack_0000fff0, 0x30));
    u_var1 = (param_1 >> 0x10);
    u_var3 = pass1_1008_b340((param_1 + 0x94));
    pu_var4 = (u_var3 & 0xffff | ((u_var3 >> 0x10) | u_var3) << 0x10);
    if (u_var3 != 0) {
        pass1_1010_3770(ppVar2, u_var3);
        pu_var4 = pass1_1038_af40(_g_AStruct112_a, *(param_1 + 6), 3);
    }
    return pu_var4;
}

pub unsafe fn pass1_1038_7a76(param_1: *mut *mut u8) {
    let ppcVar1: fn();
    let BVar2: bool;
    let struct_a: *mut AStruct306;
    let lVar3: u32;
    let paVar4: *mut AStruct1152;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let mut local_6: u32;

    pass1_1008_5784(CONCAT22(struct_a, local_a), param_1);
    while (
        lVar3 = pass1_1008_5b12(CONCAT22(struct_a, local_a)),
        lVar3 != 0,
    ) {
        pass1_1038_6a0e(lVar3);
    }
    while (
        paVar4 = pass1_1008_5b12(CONCAT22(struct_a, local_a)),
        paVar4 != 0x0,
    ) {
        BVar2 = pass1_1038_6b3c(paVar4);
        if (BVar2 != 0) {
            ppcVar1 = (param_1 + 0xc);
            (**ppcVar1)(&PTR_LOOP_1050_1008);
        }
    }
    pass1_1008_5784(CONCAT22(struct_a, local_a), (param_1 + 4));
    while (
        lVar3 = pass1_1008_5b12(CONCAT22(struct_a, local_a)),
        lVar3 != 0,
    ) {
        pass1_1030_affc(lVar3);
    }
    return;
}

pub unsafe fn pass1_1038_79f2(param_1: *mut u8, param_2: u32) {
    let ppcVar1: fn();
    let pu_var2: *mut u8;
    let mut extraout_DX: i32;
    let mut iVar3: i32;
    let mut u_var4: u16;
    let mut unaff_SS: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 8];
    let mut local_6: u32;

    local_6 = (param_2 + 4);
    u_var4 = (param_1 >> 0x10);
    iVar3 = param_1;
    pass1_1008_5784(CONCAT22(unaff_SS, local_e), (iVar3 + 4));
    while {
        pu_var2 = local_e;
        pass1_1008_5b12(CONCAT22(unaff_SS, pu_var2));
        if ((extraout_DX | pu_var2) == 0) {
            return;
        }
        (pu_var2 + 4) != local_6
    } {}
    ppcVar1 = ((iVar3 + 4) + 0xc);
    (**ppcVar1)(&PTR_LOOP_1050_1008, (iVar3 + 4), pu_var2, extraout_DX);
    return;
}

pub unsafe fn pass1_1038_79b2(param_1: *mut u8, param_2: u32) {
    let ppcVar1: fn();
    let struct_a_hi: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let mut local_ES_41: u16;
    let mut local_CS_6: u16;
    let mut local_DXAX_29: u32;
    let mut local_8: u16;

    local_CS_6 = 0x1000;
    process_struct_1000_179c(0x14, struct_a);
    if ((struct_a | struct_a_hi) == 0) {
        local_DXAX_29 = 0;
    } else {
        local_CS_6 = 0x1030;
        local_DXAX_29 = pass1_1030_aefa(CONCAT22(struct_a, struct_a_hi), param_2);
    }
    local_ES_41 = (param_1 >> 0x10);
    ppcVar1 = ((param_1 + 4) + 4);
    (**ppcVar1)(local_CS_6, (param_1 + 4), local_DXAX_29);
    return;
}

pub unsafe fn pass1_1038_7964(struct_a: *mut *mut AStruct1170) {
    let pu_var1: *mut u32;
    let struct_b: *mut AStruct1170;
    let struct_b_hi: *mut AStruct1170;
    let fn_ptr_1: fn();
    let mut uint_a: i32;

    _PTR_LOOP_1050_5a64 = 0;
    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    uint_a = struct_b.field_0x2;
    let struct_a_val = unsafe { *struct_a };
    if ((uint_a | struct_a_val) != 0) {
        fn_ptr_1 = struct_a_val;
        (**fn_ptr_1)();
    }
    pu_var1 = struct_b.field_0x4;
    uint_a = struct_b.field_0x6;
    if ((uint_a | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
            (**fn_ptr_1)();
        }
    }
    return;
}

pub unsafe fn pass1_1038_78b8(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_6912(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_6e1a(param_1: u16, param_2: u16, param_1_00: *mut long) {
    let paVar1: *mut AStruct493;
    let local_AX_76: *mut AStruct1155;
    let BVar2: bool;
    let mut uvar3: u16;
    let mut u_var4: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: u16;

    local_4 = 0;
    let param_1_00_val = unsafe { *param_1_00 };
    if ((param_1_00_val == 0) && (param_1_00 == 0)) {
        return 1;
    }
    u_var4 = (param_1_00 + 2);
    local_16._1_1_ = (u_var4 >> 8);
    if (local_16._1_1_ == '\0') {
        local_4 = param_1_00;
        // goto switchD_1038_6eab_caseD_9;
    }

    paVar1 = pass1_1028_e1ec(
        _PTR_LOOP_1050_65e2,
        param_1_00_val,
        (param_1_00_val >> 0x10),
    );
    local_AX_76._0_1_ = pass1_1030_6fa0(CONCAT22(u_var4, paVar1));
    local_AX_76 = CONCAT11(local_AX_76._1_1_, local_AX_76);
    if (local_AX_76 < 10) {
        match (local_AX_76) {
            0x1 => {
                local_4 = 1;
            }
            0x2 | 0x6 => {
                local_4 = 2;
            }
            0x3 | 0x7 => {
                local_4 = 3;
            }
            0x4 | 0x8 => {
                local_4 = 4;
            }
            0x5 | 0x9 => {
                // goto switchD_1038_6eab_caseD_5;
            }
        }
    } else {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, local_AX_76, 0x41);
        if (BVar2 != 0) {
            local_4 = 10;
            // goto switchD_1038_6eab_caseD_9;
        }
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, local_AX_76, 0x42);
        if ((BVar2 != 0) || (local_AX_76 == (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 10))) {
            local_4 = 0xb;
            // goto switchD_1038_6eab_caseD_9;
        }
        // switchD_1038_6eab_caseD_5:
        local_4 = 5;
    }
    // switchD_1038_6eab_caseD_9:
    match (local_4) {
        1 => {
            return 0x14;
        }
        2 | 7 => {
            return 0x3c;
        }
        3 | 8 => {
            return 0x78;
        }
        4 | 9 => {
            return 0xf0;
        }
        5 | 6 => {
            return 0xf;
        }
        10 => u_var3 = 0xc,
        0xb => u_var3 = 10,
        _ => {
            u_var3 = 0xffff;
        }
    }
    return u_var3;
}

pub unsafe fn pass1_1038_6d24(
    param_1: *mut AStruct1153,
    param_2: *mut *mut u8,
    param_3: *mut u16,
    param_4: *mut AStruct1154,
    param_5: u16,
) -> i32 {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let mut iVar3: i32;
    let mut unaff_SS: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut uStack4: u16;

    unsafe {
        *param_2 = 0x0;
    }
    local_8 = param_4.field_0xc;
    uStack4 = param_4.field_0x10;
    pass1_1008_3eb4(
        CONCAT22(unaff_SS, &local_8),
        CONCAT22(unaff_SS, &local_e),
        CONCAT22(unaff_SS, &local_c),
        CONCAT22(unaff_SS, &local_a),
    );
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | (param_1 + 0x1a)),
        CONCAT22(unaff_SS, &local_14),
        CONCAT22(unaff_SS, &local_12),
        CONCAT22(unaff_SS, &local_10),
    );
    iVar1 = local_c - local_12;
    u_var2 = local_e - local_14;
    iVar3 = local_a - local_10;
    if (((iVar3 == 0) && (iVar1 == 0)) && (u_var2 == 0)) {
        return 0;
    }
    if ((iVar1 != 0) || (iVar3 == 0)) {
        if ((iVar3 == 0) && (iVar1 != 0)) {
            pass1_1038_6c1c(param_1, param_3, param_2, iVar1);
            return 1;
        }
        if (((iVar3 == 0) && (iVar1 == 0)) && (u_var2 != 0)) {
            iVar1 = pass1_1038_6c68(param_1, param_3, param_2, u_var2);
            if (iVar1 != 0) {
                return 1;
            }
            return iVar1;
        }
    }
    pass1_1038_6bd4(param_1, param_3, param_2, iVar3);
    return 1;
}

pub unsafe fn pass1_1038_6c68(param_1: *mut AStruct1153, param_2: u32, param_3: u32, param_4: u16) {
    let mut iVar1: i32;
    let local_AX_13: *mut AStruct1153;
    let paVar2: *mut AStruct493;
    let mut extraout_DX: i32;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut u_var6: u32;
    let mut in_stack_0000ffd8: u16;
    let mut local_1e: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_AX_13 = param_1;
    local_AX_13 = &local_AX_13.field_0x1a;
    modify_list_1008_3f62(param_2, param_1 & 0xffff0000 | ZEXT24(local_AX_13));
    ppVar5 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(in_stack_0000ffd8, 0x2f));
    u_var6 = param_1 & 0xffff0000 | &local_AX_13.field_0x1a;
    pass1_1030_627e(_PTR_LOOP_1050_5740, u_var6, (ppVar5 + 0x20));
    u_var3 = extraout_DX | u_var6;
    if (u_var3 != 0) {
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var6, extraout_DX);
        u_var6 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
        iVar1 = (u_var6 + 0xc);
        if ((iVar1 == 0x47) || (iVar1 == 0x6a)) {
            u_var4 = (param_1 >> 0x10);
            iVar1 = local_AX_13.field_0x1e;
            if (param_4 < 0) {
                local_1e = iVar1 - 1;
            } else {
                local_1e = iVar1 + 1;
            }
            (param_2 + 4) = local_1e;
            pass1_1038_6b88(local_AX_13, u_var4, param_2, param_3);
        }
    }
    return;
}

pub unsafe fn pass1_1038_6bd4(
    param_1: u32,
    param_2: *mut u16,
    param_3: *mut *mut u8,
    param_4: i32,
) {
    let mut local_4: u16;

    modify_list_1008_3f62(param_2, param_1 & 0xffff0000 | (param_1 + 0x1a));
    if (param_4 < 0) {
        unsafe {
            local_4 = *param_2 - 1;
        }
    } else {
        unsafe {
            local_4 = *param_2 + 1;
        }
    }
    unsafe {
        *param_2 = local_4;
    }
    pass1_1038_6b88(param_1, (param_1 >> 0x10), param_2, param_3);
    return;
}

pub unsafe fn pass1_1038_6c1c(
    param_1: u32,
    param_2: *mut u16,
    param_3: *mut *mut u8,
    param_4: i32,
) {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let mut local_4: u16;

    modify_list_1008_3f62(param_2, param_1 & 0xffff0000 | (param_1 + 0x1a));
    u_var2 = (param_2 >> 0x10);
    iVar1 = (param_2 + 2);
    if (param_4 < 0) {
        local_4 = iVar1 - 1;
    } else {
        local_4 = iVar1 + 1;
    }
    (param_2 + 2) = local_4;
    pass1_1038_6b88(param_1, (param_1 >> 0x10), param_2, param_3);
    return;
}

pub unsafe fn pass1_1038_6b88(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: *mut *mut u8,
) {
    // ppu_var1: *mut *mut u8;
    let mut extraout_DX: u16;
    let mut unaff_SS: u16;
    let mut in_stack_0000ffec: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(
        _g_AStruct372_1050_0ed0,
        CONCAT22((in_stack_0000ffec >> 0x10), 0x2f),
    );
    local_a = (_local_6 + 0x20);
    ppu_var1 = &stack0xffee;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_1_00,
        local_a,
        (local_a >> 0x10),
        ppu_var1,
        unaff_SS,
    );
    unsafe {
        *param_2_00 = *ppu_var1;
    }
    return;
}

pub unsafe fn pass1_1038_6b3c(param_1: *mut AStruct1152) -> bool {
    let local_BX_3: *mut AStruct1152;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if ((((local_BX_3.field_0xc == 0) && (local_BX_3.field_0x12 == 0))
        && (local_BX_3.field_0x14 == 0))
        && (local_BX_3.field_0xe == 0 && (local_BX_3.field_0x16 != 0)))
    {
        local_BX_3.field_0x16 = 0;
    }
    if (local_BX_3.field_0x16 == 0) {
        return 1;
    }
    return 0;
}

pub unsafe fn pass1_1038_6912(param_1: *mut *mut AStruct1149) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let ppcVar3: fn();
    let pu_var4: *mut u32;
    let local_BX_5: *mut AStruct1149;
    let mut u_var5: u16;
    let mut local_a: u32;

    u_var5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    unsafe {
        *param_1 = 0x78de;
    }
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    u_var1 = &local_BX_5.field_0x6;
    pu_var4 = &local_BX_5.field_0x4;
    if ((u_var1 | pu_var4) != 0) {
        unsafe {
            ppcVar3 = *pu_var4;
        }
        (**ppcVar3)();
    }
    u_var1 = local_BX_5.field_0xe;
    u_var2 = local_BX_5.field_0x10;
    local_a = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0) {
        pass1_1020_ba7e(CONCAT22(u_var2, u_var1));
        error_check_1000_17ce(local_a);
    }
    unsafe {
        *param_1 = s_1_1050_389a;
    }
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1038_6984(param_1: *mut AStruct1150) {
    let local_BX_3: *mut AStruct1150;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0xc != 0) {
        ret_one_1020_c3ae();
        return;
    }
    if (local_BX_3.field_0xe != 0) {
        infinite_loop_1020_ba94(local_BX_3.field_0xe);
        return;
    }
    if (local_BX_3.field_0x12 == 0) {
        if (local_BX_3.field_0x14 == 0) {
            return;
        }
        pass1_1020_c42e(local_BX_3.field_0x14);
    } else {
        switch_statement_1020_c3b4(local_BX_3.field_0x12);
    }
    return;
}

pub unsafe fn pass1_1038_69fe(param_1: *mut u8) {
    (param_1 + 0x28) = 0;
    return;
}

pub unsafe fn pass1_1038_6a0e(param_1: *mut AStruct1153) {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let paVar3: *mut AStruct493;
    let pu_var4: *mut u8;
    let BVar5: bool;
    let mut u_var6: u16;
    let mut in_DX: u16;
    let mut extraout_DX: i32;
    let mut u_var7: i32;
    let local_BX_4: *mut AStruct1151;
    let mut u_var8: u16;
    let mut unaff_SS: u16;
    let mut u_var9: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: [u8; 4];
    let mut local_c: [u8; 6];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var8 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x28 == 0) {
        u_var2 = local_BX_4.field_0x20;
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
        _local_6 = CONCAT22(in_DX, paVar3);
        piVar1 = &local_BX_4.field_0x24;
        unsafe {
            *piVar1 = *piVar1 + 0x3c;
        }
        zero_list_1008_3e38(CONCAT22(unaff_SS, local_c));
        loop {
            pu_var4 = local_10;
            pass1_1038_6d24(
                param_1,
                CONCAT22(unaff_SS, pu_var4),
                CONCAT22(unaff_SS, local_c),
                _local_6,
                (_local_6 >> 0x10),
            );
            if (pu_var4 == 0x0) {
                pass1_1010_8fba(local_BX_4.field_0x4);
                _local_16 = CONCAT22(extraout_DX, pu_var4);
                u_var7 = extraout_DX | pu_var4;
                if (u_var7 == 0) {
                    u_var2 = local_BX_4.field_0x20;
                    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
                    pass1_1038_7356(param_1, paVar3, u_var7);
                    return;
                }
                u_var9 = pass1_1030_73a8(_local_6);
                BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (u_var9 + 0xc), 0x40);
                if (BVar5 != 0) {
                    local_BX_4.field_0x28 = 1;
                    local_BX_4.field_0x20 = _local_16;
                    return;
                }
                local_BX_4.field_0x20 = _local_16;
                u_var7 = extraout_DX;
                paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, &local_BX_4.field_0x20, extraout_DX);
                _local_6 = CONCAT22(u_var7, paVar3);
            }
            u_var6 = pass1_1038_6e1a(local_BX_4, u_var8, CONCAT22(unaff_SS, local_10));
            if (local_BX_4.field_0x24 < u_var6) {
                break;
            }
            piVar1 = &local_BX_4.field_0x24;
            unsafe {
                *piVar1 = *piVar1 - u_var6;
            }
            modify_list_1008_3f62(
                (param_1 & 0xffff0000 | &local_BX_4.field_0x1a),
                CONCAT22(unaff_SS, local_c),
            );
        }
    }
    return;
}

pub unsafe fn pass1_1038_6838(
    param_1: *mut *mut AStruct1148,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u32,
) -> i32 {
    let paVar1: *mut AStruct493;
    let paVar2: *mut AStruct493;
    let mut u_var3: i32;
    let pu_var4: *mut u16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let local_BX_5: *mut AStruct1148;
    let mut u_var5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    unsafe {
        *param_1 = s_1_1050_389a;
    }
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    &local_BX_5.field_0x4 = 0;
    local_BX_5.field_0x8 = param_5;
    local_BX_5.field_0xc = 0;
    local_BX_5.field_0xe = 0;
    local_BX_5.field_0x12 = 0;
    local_BX_5.field_0x14 = param_3;
    local_BX_5.field_0x16 = param_2;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_BX_5.field_0x1a));
    &local_BX_5.field_0x20 = 0;
    local_BX_5.field_0x24 = 0;
    local_BX_5.field_0x26 = param_4;
    local_BX_5.field_0x28 = 0;
    unsafe {
        *param_1 = 0x78de;
    }
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, (param_5 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_DX);
    local_BX_5.field_0x4 = paVar2;
    &local_BX_5.field_0x6 = extraout_DX;
    pu_var4 = (param_1 & 0xffff0000 | &local_BX_5.field_0x1a);
    modify_list_1008_3f62(pu_var4, CONCAT22(in_DX, &paVar1.field_0xc));
    u_var3 = pu_var4;
    pass1_1010_8fba(&local_BX_5.field_0x4);
    local_BX_5.field_0x20 = u_var3;
    local_BX_5.field_0x22 = extraout_DX_00;
    return;
}

pub unsafe fn pass1_1038_6590(
    param_1: *mut *mut AStruct1145,
    param_2: *mut u8,
    param_3: *mut u8,
    param_4: *mut u8,
    param_5: *mut u8,
    param_6: *mut u8,
) -> i32 {
    let paVar1: *mut AStruct493;
    let paVar2: *mut AStruct493;
    let mut u_var3: i32;
    let pu_var4: *mut u16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let local_BX_6: *mut AStruct1145;
    let mut u_var5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    unsafe {
        *param_1 = s_1_1050_389a;
    }
    local_BX_6.field_0x2 = &PTR_LOOP_1050_1008;
    &local_BX_6.field_0x4 = 0;
    local_BX_6.field_0x8 = param_6;
    local_BX_6.field_0xc = param_4;
    local_BX_6.field_0xe = 0;
    local_BX_6.field_0x12 = 0;
    local_BX_6.field_0x14 = 0;
    local_BX_6.field_0x16 = param_2;
    local_BX_6.field_0x18 = param_3;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_BX_6.field_0x1a));
    &local_BX_6.field_0x20 = 0;
    local_BX_6.field_0x24 = 0;
    local_BX_6.field_0x26 = param_5;
    local_BX_6.field_0x28 = 0;
    unsafe {
        *param_1 = 0x78de;
    }
    local_BX_6.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_6, (param_6 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_DX);
    local_BX_6.field_0x4 = paVar2;
    &local_BX_6.field_0x6 = extraout_DX;
    pu_var4 = (param_1 & 0xffff0000 | &local_BX_6.field_0x1a);
    modify_list_1008_3f62(pu_var4, CONCAT22(in_DX, &paVar1.field_0xc));
    u_var3 = pu_var4;
    pass1_1010_8fba(&local_BX_6.field_0x4);
    local_BX_6.field_0x20 = u_var3;
    local_BX_6.field_0x22 = extraout_DX_00;
    return;
}

pub unsafe fn pass1_1038_666e(
    param_1: *mut *mut AStruct1146,
    param_2: u32,
    param_3: u16,
    param_4: u32,
) {
    let paVar1: *mut AStruct493;
    let paVar2: *mut AStruct493;
    let mut u_var3: i32;
    let pu_var4: *mut u16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let local_BX_5: *mut AStruct1146;
    let mut u_var5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    unsafe { *param_1 = s_1_1050_389a };
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    &local_BX_5.field_0x4 = 0;
    local_BX_5.field_0x8 = param_4;
    local_BX_5.field_0xc = 0;
    local_BX_5.field_0xe = param_2;
    local_BX_5.field_0x12 = 0;
    local_BX_5.field_0x14 = 0;
    local_BX_5.field_0x18 = 0;
    local_BX_5.field_0x16 = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_BX_5.field_0x1a));
    &local_BX_5.field_0x20 = 0;
    local_BX_5.field_0x24 = 0;
    local_BX_5.field_0x26 = param_3;
    local_BX_5.field_0x28 = 0;
    unsafe {
        *param_1 = 0x78de;
    }
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_4, (param_4 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_DX);
    local_BX_5.field_0x4 = paVar2;
    &local_BX_5.field_0x6 = extraout_DX;
    pu_var4 = (param_1 & 0xffff0000 | &local_BX_5.field_0x1a);
    modify_list_1008_3f62(pu_var4, CONCAT22(in_DX, &paVar1.field_0xc));
    u_var3 = pu_var4;
    pass1_1010_8fba(&local_BX_5.field_0x4);
    local_BX_5.field_0x20 = u_var3;
    local_BX_5.field_0x22 = extraout_DX_00;
    infinite_loop_1020_ba94(param_2);
    local_BX_5.field_0x16 = u_var3;
    local_BX_5.field_0x18 = extraout_DX_01;
    return;
}

pub unsafe fn pass1_1038_675c(
    param_1: *mut *mut AStruct1147,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u32,
) -> i32 {
    let paVar1: *mut AStruct493;
    let paVar2: *mut AStruct493;
    let mut u_var3: i32;
    let pu_var4: *mut u16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let local_BX_5: *mut AStruct1147;
    let mut u_var5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    unsafe {
        *param_1 = s_1_1050_389a;
    }
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    &local_BX_5.field_0x4 = 0;
    local_BX_5.field_0x8 = param_5;
    local_BX_5.field_0xc = 0;
    local_BX_5.field_0xe = 0;
    local_BX_5.field_0x12 = param_3;
    local_BX_5.field_0x14 = 0;
    local_BX_5.field_0x16 = param_2;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_BX_5.field_0x1a));
    &local_BX_5.field_0x20 = 0;
    local_BX_5.field_0x24 = 0;
    local_BX_5.field_0x26 = param_4;
    local_BX_5.field_0x28 = 0;
    unsafe {
        *param_1 = 0x78de;
    }
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, (param_5 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_DX);
    local_BX_5.field_0x4 = paVar2;
    &local_BX_5.field_0x6 = extraout_DX;
    pu_var4 = (param_1 & 0xffff0000 | &local_BX_5.field_0x1a);
    modify_list_1008_3f62(pu_var4, CONCAT22(in_DX, &paVar1.field_0xc));
    u_var3 = pu_var4;
    pass1_1010_8fba(&local_BX_5.field_0x4);
    local_BX_5.field_0x20 = u_var3;
    local_BX_5.field_0x22 = extraout_DX_00;
    return;
}

pub unsafe fn pass1_1038_64de(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_33f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_5b3c(
    param_1: *mut u8,
    param_2: *mut u8,
    param_1_00: u32,
    param_2_00: *mut u8,
) {
    let ppcVar1: fn();
    let mut u_var2: u32;
    let mut u_var3: u32;
    let BVar4: bool;
    let local_SI_152: *mut AStruct1138;
    let mut u_var5: u16;
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < param_1_00) {
        u_var5 = (param_2_00 >> 0x10);
        local_SI_152 = param_2_00;
        if ((((local_SI_152 + local_6 * 4) != 0)
            && (
                u_var2 = (local_SI_152 + local_6 * 4),
                BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (u_var2 + 0xc), 0x2d),
                BVar4 != 0,
            ))
            && (
                ppcVar1 = ((local_SI_152 + local_6 * 4) + 0x50),
                (**ppcVar1)(),
                BVar4 != 0,
            ))
        {
            u_var2 = (local_SI_152 + local_6 * 4);
            u_var3 = (local_SI_152 + local_6 * 4);
            (u_var3 + 0x1a) = (u_var2 + 0x1a) | 1;
            ppcVar1 = ((local_SI_152 + local_6 * 4) + 0x28);
            (**ppcVar1)();
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub unsafe fn pass1_1038_5be8(param_1: *mut u8, uparam_2: i32, param_3: u16, param_4: u32) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let paVar3: *mut AStruct493;
    let BVar4: bool;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let lVar7: u32;
    let mut u_var8: u32;
    let mut local_1e: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_4: u16;
    let temp_7ffe889d3b9: *mut AStruct1139;

    lVar7 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_4, (param_1 + 8));
    u_var5 = (lVar7 >> 0x10);
    u_var6 = u_var5 | lVar7;
    if (lVar7 != 0) {
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar7, u_var5);
        _local_a = CONCAT22(u_var6, paVar3);
        local_e = 0x7a;
        if (0 < (param_4 + 4)) {
            if (param_3 == 0x7b) {
                param_3 = 0x7e;
            } else {
                if (param_3 == 0x7c) {
                    param_3 = 0x7d;
                }
            }
            local_e = 0x7f;
        }
        u_var8 = pass1_1030_73a8(_local_a);
        u_var2 = (u_var8 >> 0x10);
        temp_7ffe889d3b9 = u_var8;
        if ((((temp_7ffe889d3b9.field_0x1a & param_2) == 0)
            && ((
                u_var1 = temp_7ffe889d3b9.field_0xc,
                u_var1 == local_e || (u_var1 == param_3),
            ) || (
                BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, u_var1, 0x2b),
                BVar4 != 0,
            )))
            && (temp_7ffe889d3b9.field_0x12 != 7))
        {
            temp_7ffe889d3b9.field_0x1a = temp_7ffe889d3b9.field_0x1a | param_2;
            return (&PTR_LOOP_1050_0000 + 1);
        }
    }
    return 0x0;
}

pub unsafe fn pass1_1038_5cc6(
    param_1: *mut u8,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_5: u16,
    uparam_6: i32,
) -> i32 {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let pu_var3: *mut u8;
    let mut u_var4: u16;
    let mut unaff_SS: u16;
    let mut local_26: u16;
    let mut local_20: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    zero_list_1008_3e38(CONCAT22(unaff_SS, &local_a));
    while {
        local_4 = 0;
        local_e = 0;
        while (local_e < param_2) {
            u_var4 = (param_4 >> 0x10);
            if ((local_e * 4 + param_4) != 0) {
                u_var1 = (local_e * 4 + param_4);
                modify_list_1008_3f62(
                    CONCAT22(unaff_SS, &local_a),
                    u_var1 & 0xffff0000 | (u_var1 + 0xc),
                );
                pass1_1008_3eb4(
                    CONCAT22(unaff_SS, &local_a),
                    CONCAT22(unaff_SS, &local_14),
                    CONCAT22(unaff_SS, &local_12),
                    CONCAT22(unaff_SS, &local_10),
                );
                if (local_14 == param_5) {
                    u_var4 = (param_3 >> 0x10);
                    if (((local_e * 4 + param_3) != 0)
                        && (
                            u_var2 = (local_e * 4 + param_3),
                            ((u_var2 + 0x1a) & param_6) != 0,
                        ))
                    {
                        local_8 = local_12 - 1;
                        pu_var3 =
                            pass1_1038_5be8(param_1, param_6, 0x7b, CONCAT22(unaff_SS, &local_a));
                        if (pu_var3 != 0x0) {
                            local_4 = 1;
                        }
                        local_8 = local_12 + 1;
                        pu_var3 =
                            pass1_1038_5be8(param_1, param_6, 0x7b, CONCAT22(unaff_SS, &local_a));
                        if (pu_var3 != 0x0) {
                            local_4 = 1;
                        }
                        local_8 = local_12;
                        local_a = local_10 - 1;
                        pu_var3 =
                            pass1_1038_5be8(param_1, param_6, 0x7c, CONCAT22(unaff_SS, &local_a));
                        if (pu_var3 != 0x0) {
                            local_4 = 1;
                        }
                        local_a = local_10 + 1;
                        pu_var3 =
                            pass1_1038_5be8(param_1, param_6, 0x7c, CONCAT22(unaff_SS, &local_a));
                        if (pu_var3 != 0x0) {
                            local_4 = 1;
                        }
                    }
                }
            }
            local_e = local_e + 1;
        }
        local_4 != 0
    } {}
    return;
}

pub unsafe fn pass1_1038_5a96(
    param_1: *mut u8,
    param_2: *mut u8,
    param_1_00: u32,
    param_2_00: *mut u8,
) {
    let ppcVar1: fn();
    let mut u_var2: u32;
    let BVar3: bool;
    let local_SI_146: *mut AStruct1137;
    let mut local_ES_146: u16;
    let mut local_6: u32;
    let mut temp_5fa6353e1c: u32;

    local_6 = 0;
    while (local_6 < param_1_00) {
        local_ES_146 = (param_2_00 >> 0x10);
        local_SI_146 = param_2_00;
        if (((local_SI_146 + local_6 * 4) != 0)
            && (
                temp_5fa6353e1c = (local_SI_146 + local_6 * 4),
                BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (temp_5fa6353e1c + 0xc), 0x2c),
                BVar3 != 0,
            ))
        {
            ppcVar1 = ((local_SI_146 + local_6 * 4) + 0x54);
            (**ppcVar1)();
            if (BVar3 != 0) {
                u_var2 = (local_SI_146 + local_6 * 4);
                (u_var2 + 0x1a) = 3;
                ppcVar1 = ((local_SI_146 + local_6 * 4) + 0x28);
                (**ppcVar1)();
                u_var2 = (local_SI_146 + local_6 * 4);
                (u_var2 + 0x1a) = 2;
            }
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub unsafe fn pass1_1038_5a16(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: *mut AStruct1135,
) -> i32 {
    let ppcVar1: fn();
    let mut u_var2: u32;
    let local_AX_36: *mut AStruct1136;
    let local_SI_109: *mut AStruct1135;
    let mut uvar3: u16;
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < param_1_00) {
        u_var3 = (param_2_00 >> 0x10);
        local_SI_109 = param_2_00;
        if (((local_SI_109 + local_6 * 4) != 0)
            && (
                u_var2 = (local_SI_109 + local_6 * 4),
                local_AX_36 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (u_var2 + 0xc), 0x2f),
                local_AX_36 != 0x0,
            ))
        {
            u_var2 = (local_SI_109 + local_6 * 4);
            (u_var2 + 0x1a) = 3;
            ppcVar1 = ((local_SI_109 + local_6 * 4) + 0x28);
            (**ppcVar1)();
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub unsafe fn pass1_1038_58e6(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: *mut AStruct1133,
    param_5: *mut u8,
    param_6: u16,
) {
    let mut u_var1: i32;
    let ppcVar2: fn();
    let mut u_var3: u32;
    let BVar4: bool;
    let pu_var5: *mut u32;
    let paVar6: *mut AStruct493;
    let mut extraout_DX: u16;
    let mut u_var7: u16;
    let local_SI_123: *mut AStruct1133;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_SS: u16;
    let mut u_var10: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u32;
    let temp_5fbadbcbba: *mut AStruct1134;

    local_6 = 0;
    while (local_6 < param_1_00) {
        u_var9 = (param_2_00 >> 0x10);
        local_SI_123 = param_2_00;
        if (((local_SI_123 + local_6 * 4) != 0)
            && (
                u_var3 = (local_SI_123 + local_6 * 4),
                BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (u_var3 + 0xc), 0x2e),
                BVar4 != 0,
            ))
        {
            u_var8 = (param_5 >> 0x10);
            temp_5fbadbcbba = (local_6 * 4 + param_5);
            u_var8 = (local_6 * 4 + param_5 + 2);
            local_12 = temp_5fbadbcbba.field_0xc;
            local_c = temp_5fbadbcbba.field_0x10;
            local_e = local_c;
            if (local_c == param_6) {
                local_e = local_c - 1;
                u_var10 = pass1_1028_bb24(*(local_SI_123 + local_6 * 4));
                pu_var5 = &local_12;
                pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, pu_var5), u_var10);
                u_var7 = extraout_DX;
                paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, pu_var5, extraout_DX);
                if ((u_var7 | paVar6) != 0) {
                    u_var10 = pass1_1030_73a8(CONCAT22(u_var7, paVar6));
                    u_var1 = (u_var10 + 0x1a);
                    if (((u_var1 & 2) != 0) && ((u_var1 & 1) != 0)) {
                        u_var3 = (local_SI_123 + local_6 * 4);
                        (u_var3 + 0x1a) = 3;
                        ppcVar2 = ((local_SI_123 + local_6 * 4) + 0x28);
                        ppcVar2();
                    }
                }
            }
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub unsafe fn pass1_1038_5860(param_1: *mut u8, param_2: u16, param_2_00: *mut u8, param_4: i32) {
    let ppcVar1: fn();
    let mut u_var2: u32;
    let pu_var3: *mut u8;
    let mut u_var4: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let local_BX_19: *mut AStruct1132;
    let mut u_var5: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    if (param_4 == 0) {
        u_var5 = (param_1 >> 0x10);
        local_BX_19 = param_1;
        ppcVar1 = (local_BX_19.field_0xc + 0x10);
        pu_var3 = param_2_00;
        (**ppcVar1)();
        u_var2 = pu_var3 & 0xffff | extraout_DX << 0x10;
        local_e = 0;
        while (local_e < u_var2) {
            ppcVar1 = (local_BX_19.field_0xc + 4);
            u_var4 = u_var2;
            (**ppcVar1)();
            local_6._0_2_ = param_2_00;
            if ((u_var4 == local_6)
                && (
                    local_6._2_2_ = (param_2_00 >> 0x10),
                    extraout_DX_00 == local_6._2_2_,
                ))
            {
                return;
            }
            local_e = local_e + 1;
        }
        ppcVar1 = (local_BX_19.field_0xc + 0xc);
        (**ppcVar1)();
    }
    return;
}

pub unsafe fn pass1_1038_57dc(param_1: *mut u8, param_2: *mut u8, param_3: *mut AStruct1129) {
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    *(param_1 + param_3 * 4 + 0x1a2) = param_2 + (param_1 + 0x1a2 + param_3 * 4);
    return;
}

pub unsafe fn pass1_1038_5804(param_1: *mut u8, param_2: libc::c_long, param_3: *mut AStruct1130) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0x1a2) = (param_1 + 0x1a2 + param_3 * 4) - param_2;
    return;
}

pub unsafe fn pass1_1038_582c(param_1: *mut AStruct1131, param_2: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppcVar3: fn();
    let local_BX_4: *mut AStruct1131;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    pu_var1 = local_BX_4.field_0x14;
    u_var2 = &local_BX_4.field_0x16;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppcVar3 = *pu_var1;
        }
        (**ppcVar3)();
    }
    &local_BX_4.field_0x14 = param_2;
    return;
}

pub unsafe fn pass1_1038_57c0(param_1: *mut u8) {
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x14e)), 0, 0x54);
    return;
}

pub unsafe fn pass1_1038_5770(param_1: *mut u8, param_2: libc::c_long, param_3: *mut AStruct1127) {
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0xba) = (param_1 + 0xba + param_3 * 4) + param_2;
    return;
}

pub unsafe fn pass1_1038_5798(param_1: *mut u8, param_2: u32, param_3: *mut AStruct1128) {
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0x14e) = (param_1 + 0x14e + param_3 * 4) + param_2;
    return;
}

pub unsafe fn pass1_1038_565e(param_1: *mut AStruct1124) {
    let local_BX_4: *mut AStruct1124;
    let mut u_var1: u16;
    let mut unaff_SS: u16;
    let mut u_var2: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    u_var1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    u_var2 = pass1_1030_8e3c(CONCAT22(unaff_SS, local_4), local_BX_4.field_0x4);
    pass1_1038_582c(param_1, u_var2);
    return CONCAT22(local_BX_4.field_0x16, local_BX_4.field_0x14);
}

pub unsafe fn pass1_1038_5694(param_1: u32, param_2: u32, param_3: *mut AStruct1125) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0x26) = (param_1 + 0x26 + param_3 * 4) + param_2;
    return;
}

pub unsafe fn pass1_1038_56ba(param_1: *mut u8) {
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x26)), 0, 0x94);
    return;
}

pub unsafe fn pass1_1038_56d6(param_1: *mut u8, param_2: u16) {
    let ppcVar1: fn();
    let u_var2: u8;
    let local_AX_14: *mut AStruct1126;
    let mut uvar3: u16;
    let paVar4: *mut AStruct493;
    let extraout_var: u32;
    let mut u_var5: u32;
    let mut extraout_DX: u16;
    let mut u_var6: u16;
    let mut extraout_DX_00: i32;
    let mut u_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_AX_14 = param_1;
    local_AX_14 = &local_AX_14.field_0xba;
    u_var9 = 0x1000;
    u_var2 = pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(local_AX_14)), 0, 0x94);
    u_var3 = CONCAT31(extraout_var, u_var2);
    if (param_2 != 0) {
        u_var8 = (param_1 >> 0x10);
        if (local_AX_14.field_0xc == 0) {
            u_var3 = 0;
            u_var6 = 0;
        } else {
            ppcVar1 = (local_AX_14.field_0xc + 0x10);
            (**ppcVar1)();
            u_var6 = extraout_DX;
        }
        _local_6 = CONCAT22(u_var6, u_var3);
        local_a = 0;
        while (local_a < _local_6) {
            ppcVar1 = (local_AX_14.field_0xc + 4);
            u_var5 = _local_6;
            (**ppcVar1)(u_var9, local_AX_14.field_0xc);
            u_var7 = extraout_DX_00 | u_var5;
            if (u_var7 != 0) {
                paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var5, extraout_DX_00);
                u_var9 = 0x1030;
                pass1_1030_72d0(CONCAT22(u_var7, paVar4));
            }
            local_a = local_a + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_53ba(param_1: *mut u8, param_2: *mut AStruct1122) {
    let mut u_var1: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a2 + param_2 * 4) < (param_1 + 0x14e + param_2 * 4)) {
        return;
    }
    return;
}

pub unsafe fn pass1_1038_540a() {
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    return;
}

pub unsafe fn pass1_1038_5464(param_1: *mut AStruct1123) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut in_AX: u16;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let paVar5: *mut AStruct493;
    let mut u_var6: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut u_var7: i32;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: u16;
    let mut iVar8: i32;
    let mut iVar9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let local_42: *mut AStruct1123;
    let mut uStack64: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_3fa8023d0d: *mut AStruct1123;

    pass1_1038_56ba(param_1);
    pass1_1038_57c0(param_1);
    u_var10 = (param_1 >> 0x10);
    iVar8 = param_1;
    if ((iVar8 + 0xc) == 0) {
        in_AX = 0;
        u_var11 = 0;
    } else {
        ppcVar2 = ((iVar8 + 0xc) + 0x10);
        ppcVar2();
        u_var11 = extraout_DX;
    }
    _local_a = CONCAT22(u_var11, in_AX);
    local_e = 0;
    while (local_e < _local_a) {
        local_42 = local_e;
        uStack64 = (local_e >> 0x10);
        u_var1 = (iVar8 + 0xc);
        ppcVar2 = ((iVar8 + 0xc) + 4);
        u_var6 = _local_a;
        ppcVar2(unaff_CS, u_var1, (u_var1 >> 0x10), local_42, uStack64);
        local_12 = u_var6;
        u_var7 = extraout_DX_01 | local_12;
        local_10 = extraout_DX_01;
        if (u_var7 != 0) {
            unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
            paVar5 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_12, extraout_DX_01);
            local_16 = CONCAT22(u_var7, paVar5);
            local_1a = &paVar5[1].field_0x4;
            if ((&paVar5[1].field_0x6 | local_1a) == 0) {
                local_1c = 0;
            } else {
                local_1c = (local_1a + 4);
            }
            local_1e = 0;
            while (local_1e < local_1c) {
                unaff_CS = 0x1020;
                pass1_1020_bb16(
                    local_1a,
                    CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_2e)),
                    CONCAT22(unaff_SS, &local_2a),
                    local_1e,
                );
                if (CONCAT22(local_2e._2_2_, local_2e) != 0) {
                    pass1_1038_5694(param_1, CONCAT22(local_2e._2_2_, local_2e), local_2a);
                }
                local_1e = local_1e + 1;
            }
            u_var11 = (local_16 >> 0x10);
            local_22 = (local_16 + 0x1e);
            u_var7 = (local_16 + 0x20);
            u_var3 = u_var7 | local_22;
            if (u_var3 == 0) {
                u_var3 = 0;
            } else {
                ppcVar2 = (local_22 + 0x10);
                ppcVar2(unaff_CS, local_22, u_var7);
            }
            local_1e = 0;
            local_1c = u_var3;
            while (local_1e < local_1c) {
                ppcVar2 = (local_22 + 4);
                u_var4 = local_1c;
                ppcVar2(unaff_CS, local_22, (local_22 >> 0x10), local_1e, 0);
                u_var7 = extraout_DX_00 | u_var4;
                local_2a = u_var4;
                local_28 = extraout_DX_00;
                if (u_var7 != 0) {
                    unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
                    paVar5 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var4, extraout_DX_00);
                    iVar9 = &paVar5.field_0xc * 4;
                    (iVar8 + iVar9 + 0x14e) = (iVar8 + 0x14e + iVar9) + 1;
                }
                local_1e = local_1e + 1;
            }
        }
        local_e = local_e + 1;
    }
    u_var1 = (iVar8 + 0x1f6);
    local_42 = (u_var1 >> 0x10);
    u_var6 = _local_a;
    pass1_1030_38f2(u_var1, local_42, 3);
    u_var7 = u_var6;
    u_var1 = (iVar8 + 0x1f6);
    local_42 = (u_var1 >> 0x10);
    local_6 = u_var7;
    local_4 = extraout_DX_02;
    pass1_1030_38f2(u_var1, local_42, 4);
    _local_6 = CONCAT22(
        local_4 + extraout_DX_03 + CARRY2(local_6, u_var7),
        local_6 + u_var7,
    );
    if (_local_6 == 0) {
        u_var1 = (iVar8 + 0x1f6);
        local_42 = (u_var1 >> 0x10);
        pass1_1030_38f2(u_var1, local_42, 2);
        _local_6 = CONCAT22(extraout_DX_04, u_var7);
    }
    u_var1 = (iVar8 + 0x1f6);
    _local_6 = _local_6 + (u_var1 + 0x170);
    pass1_1038_5694(param_1, _local_6, (s_New_failed_in_Op__Op_1050_0020 + 4));
    return;
}

pub unsafe fn pass1_1038_45e4(param_1: *mut AStruct1109) {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let ppcVar3: fn();
    let mut u_var4: u32;
    let mut in_AX: i32;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut iVar8: i32;
    let mut iVar9: i32;
    let pu_var10: *mut u8;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut u_var11: i32;
    let mut extraout_DX_03: u16;
    let local_BX_8: *mut AStruct1109;
    let mut u_var12: u16;
    let mut bVar13: bool;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var12 = (param_1 >> 0x10);
    local_BX_8 = param_1;
    u_var2 = local_BX_8.field_0x1f6;
    pass1_1030_38f2(u_var2, (u_var2 >> 0x10), 2);
    u_var2 = local_BX_8.field_0x1f6;
    u_var5 = in_AX;
    pass1_1030_38f2(u_var2, (u_var2 >> 0x10), 1);
    bVar13 = in_AX < u_var5;
    in_AX = in_AX - u_var5;
    u_var2 = local_BX_8.field_0x1f6;
    pass1_1030_38f2(u_var2, (u_var2 >> 0x10), 4);
    u_var2 = local_BX_8.field_0x1f6;
    u_var6 = u_var5;
    pass1_1030_38f2(u_var2, (u_var2 >> 0x10), 3);
    u_var11 = local_BX_8.field_0x24;
    u_var7 = u_var11 + (u_var5 - u_var6);
    u_var11 = (u_var11 >> 0xf)
        + ((extraout_DX_01 - extraout_DX_02) - (u_var5 < u_var6))
        + CARRY2(u_var11, u_var5 - u_var6)
        + ((extraout_DX - extraout_DX_00) - bVar13)
        + CARRY2(u_var7, in_AX);
    if ((u_var11 < 0) || (u_var11 < 1 && (u_var7 + in_AX == 0))) {
        iVar9 = -1;
    } else {
        iVar9 = 1;
    }
    piVar1 = &local_BX_8.field_0x24;
    unsafe {
        *piVar1 = *piVar1 + iVar9;
    }
    pu_var10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x16);
    iVar8 = pu_var10;
    pass1_1038_4d6e(param_1, pu_var10 & 0xffff | u_var11 << 0x10);
    _local_16 = CONCAT22(extraout_DX_03, iVar8);
    u_var4 = *_local_16;
    ppcVar3 = u_var4 + 8;
    iVar9 = iVar8;
    (**ppcVar3)(&PTR_LOOP_1050_1008, iVar8, extraout_DX_03);
    if (_local_16 != 0x0) {
        ppcVar3 = u_var4;
        (**ppcVar3)(&PTR_LOOP_1050_1008, iVar8, extraout_DX_03, 1);
    }
    piVar1 = &local_BX_8.field_0x24;
    unsafe {
        *piVar1 = *piVar1 + iVar9 * 2;
    }
    iVar9 = local_BX_8.field_0x24;
    if (100 < iVar9) {
        iVar9 = 100;
    }
    local_BX_8.field_0x24 = iVar9;
    if (iVar9 < 0) {
        iVar9 = 0;
    }
    local_BX_8.field_0x24 = iVar9;
    iVar9 = iVar9 / 10;
    local_1c = 0x10;
    if (iVar9 < 0xb) {
        local_1c = 0x14;
    } else {
        if (iVar9 < 0x15) {
            local_1c = 0x13;
        } else {
            if (iVar9 < 0x1f) {
                local_1c = 0x12;
            } else {
                if (iVar9 < 0x29) {
                    local_1c = 0x11;
                } else {
                    if (iVar9 < 0x33) {
                        local_1c = 0x10;
                    } else {
                        if (iVar9 < 0x3d) {
                            local_1c = 0xf;
                        } else {
                            if (iVar9 < 0x47) {
                                local_1c = 0xe;
                            } else {
                                if (iVar9 < 0x51) {
                                    local_1c = 0xd;
                                } else {
                                    if (iVar9 < 0x5b) {
                                        local_1c = 0xc;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pass1_1030_3258(local_BX_8.field_0x1f6, local_1c);
    return;
}

pub unsafe fn pass1_1038_4760(param_1: *mut AStruct1109) {
    let piVar1: *mut i32;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut u_var4: i32;
    let mut iVar5: i32;
    let pu_var6: *mut u8;
    let mut u_var7: u32;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut u_var8: i32;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: u16;
    let mut iVar9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let pu_var12: *mut u32;
    let u_var13: u8;
    let mut u_var14: i32;
    let mut local_22: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var10 = (param_1 >> 0x10);
    iVar9 = param_1;
    piVar1 = (iVar9 + 0x22);
    unsafe {
        *piVar1 = *piVar1 + (iVar9 + 0x20c);
    }
    pu_var6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x26);
    u_var3 = SUB42(pu_var6, 0);
    pass1_1038_4d6e(param_1, pu_var6 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, u_var3);
    u_var11 = SUB42(&PTR_LOOP_1050_1008, 0);
    u_var8 = extraout_DX;
    pu_var6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1a);
    u_var4 = pu_var6;
    pass1_1038_4d6e(param_1, pu_var6 & 0xffff | u_var8 << 0x10);
    _local_e = CONCAT22(extraout_DX_00, u_var4);
    ppcVar2 = (*_local_e + 0x10);
    u_var8 = u_var4;
    ppcVar2(&PTR_LOOP_1050_1008, u_var4, extraout_DX_00);
    u_var13 = u_var3;
    u_var14 = extraout_DX;
    if ((extraout_DX_01 | u_var8) == 0) {
        ppcVar2 = (*_local_a + 0x10);
        ppcVar2();
        piVar1 = (iVar9 + 0x22);
        unsafe {
            *piVar1 = *piVar1 + u_var8;
        }
    } else {
        ppcVar2 = (*_local_a + 0x10);
        ppcVar2();
        _local_16 = CONCAT22(extraout_DX_02, u_var8);
        local_1a = 0;
        while (local_1a < _local_16) {
            u_var7 = _local_16;
            pu_var12 = _local_e;
            pass1_1030_1d7c(_local_a, local_1a);
            iVar5 = u_var7;
            u_var11 = SUB42(&PTR_LOOP_1050_1028, 0);
            pass1_1028_5a94(iVar5, extraout_DX_03, pu_var12);
            if (iVar5 == 2) {
                if ((*_PTR_LOOP_1050_65e2 & 1) == 0) {}
                // goto LAB_1038_485e;
            } else {
                if (iVar5 != 3) {
                    // LAB_1038_485e:
                    piVar1 = (iVar9 + 0x22);
                    unsafe {
                        *piVar1 = *piVar1 + 1;
                    }
                }
            }
            local_1a = local_1a + 1;
        }
    }
    if (_local_a != 0x0) {
        ppcVar2 = *_local_a;
        ppcVar2(u_var11, u_var3, extraout_DX, 1, u_var13, u_var14);
    }
    if (_local_e != 0x0) {
        ppcVar2 = *_local_e;
        ppcVar2(u_var11, u_var4, extraout_DX_00, 1);
    }
    pass1_1038_45e4(param_1);
    unsafe {
        if (0x32 < (iVar9 + 0x24)) {
            piVar1 = (iVar9 + 0x22);
            *piVar1 = *piVar1 + -1;
        }
        if ((iVar9 + 0x24) < 0x32) {
            piVar1 = (iVar9 + 0x22);
            *piVar1 = *piVar1 + 1;
        }
        if ((iVar9 + 0x18) < 0xfa) {
            piVar1 = (iVar9 + 0x22);
            *piVar1 = *piVar1 + 2;
        } else {
            if ((iVar9 + 0x18) < 0x1c2) {
                piVar1 = (iVar9 + 0x22);
                *piVar1 = *piVar1 + 1;
            } else {
                if (0x225 < (iVar9 + 0x18)) {
                    if ((iVar9 + 0x18) < 0x2ee) {
                        piVar1 = (iVar9 + 0x22);
                        *piVar1 = *piVar1 + -1;
                    } else {
                        piVar1 = (iVar9 + 0x22);
                        *piVar1 = *piVar1 + -2;
                    }
                }
            }
        }
    }
    iVar5 = (iVar9 + 0x22);
    if (100 < iVar5) {
        iVar5 = 100;
    }
    (iVar9 + 0x22) = iVar5;
    if (iVar5 < 0) {
        iVar5 = 0;
    }
    (iVar9 + 0x22) = iVar5;
    return;
}

pub unsafe fn pass1_1038_48e0(param_1: *mut u8, param_2: i32) {
    let mut iVar1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    iVar1 = (param_1 + 0x20e) + param_2;
    if (10 < iVar1) {
        iVar1 = 10;
    }
    (param_1 + 0x20e) = iVar1;
    return;
}

pub unsafe fn pass1_1038_4900(param_1: *mut u8) {
    let piVar1: *mut i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x20e);
    unsafe {
        *piVar1 = *piVar1 + -1;
        if (*piVar1 < 0) {
            (param_1 + 0x20e) = 0;
        }
    }
    return;
}

pub unsafe fn pass1_1038_4918(param_1: *mut AStruct1110) {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let mut u_var3: i32;
    let paVar4: *mut AStruct493;
    let mut iVar5: i32;
    let pu_var6: *mut u32;
    let mut in_DX: u16;
    let mut u_var7: u16;
    let local_BX_6: *mut AStruct1110;
    let mut iVar8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_SS: u16;
    let mut u_var11: u32;
    let mut local_15e: u32;
    let mut local_15a: u16;
    let mut local_158: u16;
    let mut local_156: u16;
    let mut local_154: u32;
    let mut local_14a: [u8; 4];
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_20: u32;
    let mut uStack28: u16;
    let mut local_1a: u32;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var9 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    if (local_BX_6.field_0x4 != 0x4000001) {
        return;
    }
    u_var2 = local_BX_6.field_0x8;
    paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
    _local_6 = CONCAT22(in_DX, paVar4);
    local_a = &paVar4.field_0x10;
    u_var10 = (local_a >> 0x10);
    iVar8 = local_a;
    if ((iVar8 + 0x1c) == 0) {
        return;
    }
    local_e = 0;
    match (local_BX_6.field_0x20e) {
        1 => local_e._0_2_ = 0x1e,
        2 => local_e._0_2_ = 0x1c,
        3 => local_e._0_2_ = 0x1a,
        4 => local_e._0_2_ = 0x18,
        5 => local_e._0_2_ = 0x16,
        6 => local_e._0_2_ = 0x14,
        7 => local_e._0_2_ = 0x12,
        8 => local_e._0_2_ = 0x10,
        9 => local_e._0_2_ = 0xe,
        10 => local_e._0_2_ = 0xc,
        // default:
        // goto switchD_1038_49cf_caseD_a;
    }
    local_e = local_e;
    // switchD_1038_49cf_caseD_a:
    local_12 = *_PTR_LOOP_1050_65e2;
    if ((local_e != 0)
        && (((local_12 & 0xffff | *(_PTR_LOOP_1050_65e2 + 2) << 0x10) % local_e) == 0))
    {
        piVar1 = (iVar8 + 0x1c);
        unsafe {
            *piVar1 = *piVar1 + -1;
            piVar1 = (iVar8 + 0x1a);
            *piVar1 = *piVar1 + 1;
        }
        iVar5 = (iVar8 + 0x1a) * 6 + (iVar8 + 0x16);
        u_var10 = (iVar8 + 0x18);
        local_20 = (iVar5 + -6);
        uStack28 = (iVar5 + -2);
        local_146 = &local_20;
        pu_var6 = &local_20;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            pu_var6,
            unaff_SS,
            local_BX_6.field_0x8,
            local_14a,
            unaff_SS,
        );
        unsafe {
            local_1a = *pu_var6;
        }
        u_var7 = (pu_var6 + 2);
        local_15e._3_1_ = (local_1a >> 0x18);
        if (local_15e._3_1_ != '\0') {
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_1a, u_var7);
            u_var11 = pass1_1030_73a8(CONCAT22(u_var7, paVar4));
            u_var3 = (u_var11 >> 0x10);
            if ((u_var3 | u_var11) != 0) {
                iVar8 = (u_var11 + 0xc);
                if (iVar8 < 1) {
                    return;
                }
                if (SBORROW2(iVar8, 1)) {
                    return;
                }
                if (8 < iVar8 + -1) {
                    return;
                }
            }
        }
        pass1_1028_87f0(
            CONCAT22(unaff_SS, &local_144),
            0,
            0,
            0x10,
            &local_20,
            unaff_SS,
            local_BX_6.field_0x4,
            local_BX_6.field_0x8,
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_144));
    }
    return;
}

pub unsafe fn pass1_1038_4b20(param_1: *mut u8, param_2: *mut u8, param_3: u32) {
    pass1_1020_c4f4((param_1 + 0xc), param_2, (param_2 >> 0x10), param_3);
    return;
}

pub unsafe fn pass1_1038_4b40(param_1: *mut AStruct1111) {
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut AStruct493;
    let mut u_var3: u32;
    let mut extraout_DX: u16;
    let mut u_var4: u16;
    let mut extraout_DX_00: i32;
    let mut u_var5: i32;
    let local_BX_12: *mut AStruct1111;
    let mut u_var6: u16;
    let mut unaff_CS: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var6 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    if (local_BX_12.field_0xc == 0) {
        in_AX = 0;
        u_var4 = 0;
    } else {
        ppcVar1 = (local_BX_12.field_0xc + 0x10);
        (**ppcVar1)();
        u_var4 = extraout_DX;
    }
    _local_a = CONCAT22(u_var4, in_AX);
    local_e = 0;
    while (local_e < _local_a) {
        ppcVar1 = (local_BX_12.field_0xc + 4);
        u_var3 = _local_a;
        (**ppcVar1)(unaff_CS, local_BX_12.field_0xc);
        u_var5 = extraout_DX_00 | u_var3;
        if (u_var5 != 0) {
            paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var3, extraout_DX_00);
            unaff_CS = 0x1030;
            pass1_1030_73a8(CONCAT22(u_var5, paVar2));
        }
        local_e = local_e + 1;
    }
    return;
}

pub unsafe fn pass1_1038_4c1a(param_1: *mut AStruct1112) {
    let ppcVar1: fn();
    let u_var2: u8;
    let mut in_AX: u16;
    let paVar3: *mut AStruct493;
    let mut u_var4: u32;
    let extraout_var: u32;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut in_EDX: u32;
    let local_BX_12: *mut AStruct1112;
    let local_ES_12: *mut AStruct1112;
    let mut unaff_CS: u16;
    let mut u_var7: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_ES_12 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    u_var7 = local_BX_12.field_0xc;
    ppcVar1 = (local_BX_12.field_0xc + 0x10);
    (**ppcVar1)();
    _local_a = CONCAT22(in_EDX, in_AX);
    local_e = 0;
    while (u_var5 = in_EDX, local_e < _local_a) {
        ppcVar1 = (local_BX_12.field_0xc + 4);
        u_var4 = _local_a;
        (**ppcVar1)(unaff_CS, local_BX_12.field_0xc, local_e, u_var7);
        u_var6 = u_var5 | u_var4;
        in_EDX = u_var6;
        if (u_var6 != 0) {
            paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var4, u_var5);
            u_var2 = pass1_1030_6fa0(CONCAT22(in_EDX, paVar3));
            unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
            pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, u_var2), 0xe);
        }
        local_e = local_e + 1;
    }
    return;
}

pub unsafe fn pass1_1038_4cba(param_1: *mut u8) {
    pass1_1030_38b8((param_1 + 0x1f6));
    return;
}

pub unsafe fn pass1_1038_4cd0(param_1: *mut u8, param_2: u32, param_3: u16) {
    let local_ES_6: *mut u8;

    local_ES_6 = (param_1 >> 0x10);
    (param_1 + 0x1c) = param_3;
    (param_1 + 0x1e) = param_2;
    return;
}

pub unsafe fn pass1_1038_4cea(param_1: *mut u8, param_2: u32, param_3: u32) {
    let mut local_ES_3: u16;

    local_ES_3 = (param_1 >> 0x10);
    param_3 = (param_1 + 0x1c);
    param_2 = (param_1 + 0x1e);
    return;
}

pub unsafe fn pass1_1038_4d0e(param_1: *mut AStruct1113, param_2: u16) {
    let local_BX_3: *mut AStruct1113;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    local_BX_3.field_0x1a = local_BX_3.field_0x18;
    local_BX_3.field_0x18 = param_2;
    return;
}

pub unsafe fn pass1_1038_4d28(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x1fc), (param_1 + 0x1fa));
}

pub unsafe fn pass1_1038_4d3c(param_1: *mut AStruct1114, param_2: u32) {
    let u_var1: u8;
    let mut local_register1_15: u16;
    let mut local_DX__1: u16;
    let local_BX_4: *mut AStruct1114;
    let mut uvar3: u16;
    let paVar2: *mut AStruct44;

    u_var3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    u_var1 = error_check_1000_17ce(&local_BX_4.field_0x1fa);
    paVar2 = CONCAT31(_local_register1_15, u_var1);
    pass1_fn_1008_60e8(param_2);
    local_BX_4.field_0x1fa = paVar2;
    local_BX_4.field_0x1fc = local_DX__1;
    return;
}

pub unsafe fn pass1_1038_4d6e(param_1: *mut AStruct1115, param_2: u32) {
    let pu_var1: *mut u16;
    let ppcVar2: fn();
    let u_var3: u8;
    let mut in_AX: i32;
    let paVar4: *mut AStruct493;
    let extraout_var: u32;
    let in_DX: *mut AStruct199;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut u_var5: u16;
    let mut extraout_DX_01: i32;
    let mut u_var6: i32;
    let local_BX_49: *mut AStruct1115;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x18, in_DX);
    if ((in_DX | in_AX) == 0) {
        in_AX = 0;
        u_var7 = 0;
    } else {
        pass1_1030_1cd8(CONCAT22(in_DX, in_AX), 5, 5);
        u_var7 = extraout_DX;
    }
    _local_6 = CONCAT22(u_var7, in_AX);
    u_var7 = (param_1 >> 0x10);
    local_BX_49 = param_1;
    if (local_BX_49.field_0xc == 0) {
        in_AX = 0;
        u_var5 = 0;
    } else {
        ppcVar2 = (local_BX_49.field_0xc + 0x10);
        ppcVar2();
        u_var5 = extraout_DX_00;
    }
    _local_a = CONCAT22(u_var5, in_AX);
    local_e = 0;
    loop {
        if (_local_a <= local_e) {
            return;
        }
        ppcVar2 = (local_BX_49.field_0xc + 4);
        u_var8 = _local_a;
        ppcVar2();
        u_var6 = extraout_DX_01 | u_var8;
        if (u_var6 != 0) {
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var8, extraout_DX_01);
            local_1a = CONCAT22(u_var6, paVar4);
            u_var3 = pass1_1030_6fa0(CONCAT22(u_var6, paVar4));
            local_1e = 0;
            loop {
                pu_var1 = (param_2 + 4);
                let pu_var1_val = unsafe { *pu_var1 };
                if (pu_var1_val == local_1e || pu_var1_val < local_1e) {
                    break;
                }
                if ((param_2 + local_1e * 2) == CONCAT31(extraout_var, u_var3)) {
                    u_var8 = pass1_1030_73a8(local_1a);
                    if ((u_var8 + 0x12) == 5) {
                        ppcVar2 = (*_local_6 + 0xc);
                        ppcVar2();
                    }
                    break;
                }
                local_1e = local_1e + 1;
            }
        }
        local_e = local_e + 1;
    }
}

pub unsafe fn pass1_1038_4e78(param_1: u32, param_2: u32) {
    let pu_var1: *mut u16;
    let ppcVar2: fn();
    let u_var3: u8;
    let mut in_AX: i32;
    let local_AX_174: *mut AStruct515;
    let extraout_var: u32;
    let in_DX: *mut AStruct199;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut u_var5: u16;
    let mut extraout_DX_01: i32;
    let mut iVar6: i32;
    let mut u_var7: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut u_var4: u32;

    process_struct_1000_179c(0x18, in_DX);
    if ((in_DX | in_AX) == 0) {
        in_AX = 0;
        u_var7 = 0;
    } else {
        pass1_1030_1cd8(CONCAT22(in_DX, in_AX), 5, 5);
        u_var7 = extraout_DX;
    }
    _local_6 = CONCAT22(u_var7, in_AX);
    u_var7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0xc) == 0) {
        in_AX = 0;
        u_var5 = 0;
    } else {
        ppcVar2 = ((iVar6 + 0xc) + 0x10);
        ppcVar2();
        u_var5 = extraout_DX_00;
    }
    _local_a = CONCAT22(u_var5, in_AX);
    local_e = 0;
    loop {
        if (_local_a <= local_e) {
            return;
        }
        u_var4 = _local_a;
        pass1_1030_1d58((iVar6 + 0xc));
        if ((extraout_DX_01 | u_var4) != 0) {
            u_var3 = pass1_1030_6fa0((u_var4 & 0xffff | extraout_DX_01 << 0x10));
            local_1a = 0;
            loop {
                pu_var1 = (param_2 + 4);
                let pu_var1_val = unsafe { *pu_var1 };
                if (pu_var1_val == local_1a || pu_var1_val < local_1a) {
                    break;
                }
                if ((param_2 + local_1a * 2) == CONCAT31(extraout_var, u_var3)) {
                    ppcVar2 = (*_local_6 + 0xc);
                    ppcVar2();
                    break;
                }
                local_1a = local_1a + 1;
            }
        }
        local_e = local_e + 1;
    }
}

pub unsafe fn pass1_1038_4f54(param_1: *mut AStruct1116, param_2: u16) {
    let ppcVar1: fn();
    let u_var2: u8;
    let mut in_AX: u16;
    let BVar3: bool;
    let mut u_var4: u32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut u_var5: u16;
    let mut extraout_DX_00: i32;
    let local_BX_4: *mut AStruct1116;
    let mut u_var6: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0xc == 0x0) {
        in_AX = 0;
        u_var5 = 0;
    } else {
        ppcVar1 = (*local_BX_4.field_0xc + 0x10);
        (**ppcVar1)();
        u_var5 = extraout_DX;
    }
    _local_6 = CONCAT22(u_var5, in_AX);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        u_var4 = _local_6;
        pass1_1030_1d58(local_BX_4.field_0xc);
        if ((extraout_DX_00 | u_var4) != 0) {
            u_var2 = pass1_1030_6fa0((u_var4 & 0xffff | extraout_DX_00 << 0x10));
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, u_var2), param_2);
            if (BVar3 != 0) {
                return;
            }
        }
        local_a = local_a + 1;
    }
}

pub unsafe fn pass1_1038_4fd8(param_1: *mut AStruct1117, param_2: u16) {
    let ppcVar1: fn();
    let u_var2: u8;
    let mut in_AX: u16;
    let mut u_var3: u32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut u_var4: u16;
    let mut extraout_DX_00: i32;
    let local_BX_4: *mut AStruct1117;
    let mut u_var5: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0xc == 0x0) {
        in_AX = 0;
        u_var4 = 0;
    } else {
        ppcVar1 = (*local_BX_4.field_0xc + 0x10);
        (**ppcVar1)();
        u_var4 = extraout_DX;
    }
    _local_6 = CONCAT22(u_var4, in_AX);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        u_var3 = _local_6;
        pass1_1030_1d58(local_BX_4.field_0xc);
        if ((extraout_DX_00 | u_var3) != 0) {
            u_var2 = pass1_1030_6fa0((u_var3 & 0xffff | extraout_DX_00 << 0x10));
            if (CONCAT31(extraout_var, u_var2) == param_2) {
                return;
            }
        }
        local_a = local_a + 1;
    }
}

pub unsafe fn pass1_1038_5050(param_1: *mut AStruct1118, param_2: u16) {
    let ppcVar1: fn();
    let u_var2: u8;
    let mut in_AX: u16;
    let mut u_var3: u32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut u_var4: u16;
    let mut extraout_DX_00: i32;
    let local_BX_12: *mut AStruct1118;
    let mut u_var5: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var5 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    if (local_BX_12.field_0xc == 0x0) {
        in_AX = 0;
        u_var4 = 0;
    } else {
        ppcVar1 = (*local_BX_12.field_0xc + 0x10);
        (**ppcVar1)();
        u_var4 = extraout_DX;
    }
    _local_a = CONCAT22(u_var4, in_AX);
    local_e = 0;
    while (local_e < _local_a) {
        u_var3 = _local_a;
        pass1_1030_1d58(local_BX_12.field_0xc);
        if ((extraout_DX_00 | u_var3) != 0) {
            u_var2 = pass1_1030_6fa0((u_var3 & 0xffff | extraout_DX_00 << 0x10));
            pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, u_var2), param_2);
        }
        local_e = local_e + 1;
    }
    return;
}

pub unsafe fn pass1_1038_50e0(param_1: *mut AStruct1119, param_2: u16) {
    let ppcVar1: fn();
    let u_var2: u8;
    let mut in_AX: u16;
    let BVar3: bool;
    let mut u_var4: u32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut u_var5: u16;
    let mut extraout_DX_00: i32;
    let local_BX_12: *mut AStruct1119;
    let mut u_var6: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var6 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    if (local_BX_12.field_0xc == 0x0) {
        in_AX = 0;
        u_var5 = 0;
    } else {
        ppcVar1 = (*local_BX_12.field_0xc + 0x10);
        (**ppcVar1)();
        u_var5 = extraout_DX;
    }
    _local_a = CONCAT22(u_var5, in_AX);
    local_e = 0;
    while (local_e < _local_a) {
        u_var4 = _local_a;
        pass1_1030_1d58(local_BX_12.field_0xc);
        if ((extraout_DX_00 | u_var4) != 0) {
            u_var2 = pass1_1030_6fa0((u_var4 & 0xffff | extraout_DX_00 << 0x10));
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, u_var2), param_2);
            if (BVar3 != 0) {
                pass1_1030_73a8((u_var4 & 0xffff | extraout_DX_00 << 0x10));
            }
        }
        local_e = local_e + 1;
    }
    return;
}

pub unsafe fn pass1_1038_518c(param_1: *mut AStruct1120) {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let ppcVar3: fn();
    let mut in_AX: u16;
    let paVar4: *mut AStruct493;
    let mut u_var5: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut u_var6: i32;
    let mut extraout_DX_01: i32;
    let local_BX_5: *mut AStruct1120;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let unaff_CS: u8;
    let mut bVar11: bool;
    let mut u_var12: u32;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var9 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (local_BX_5.field_0x206 == 0) {
        if (local_BX_5.field_0xc == 0) {
            in_AX = 0;
            u_var10 = 0;
        } else {
            ppcVar3 = (local_BX_5.field_0xc + 0x10);
            (**ppcVar3)();
            u_var10 = extraout_DX;
        }
        _local_6 = CONCAT22(u_var10, in_AX);
        local_a = 0;
        while (local_a < _local_6) {
            u_var2 = local_BX_5.field_0xc;
            ppcVar3 = (local_BX_5.field_0xc + 4);
            u_var5 = _local_6;
            (**ppcVar3)(
                unaff_CS,
                u_var2,
                (u_var2 >> 0x10),
                local_a,
                (local_a >> 0x10),
            );
            u_var6 = extraout_DX_00 | u_var5;
            if (u_var6 != 0) {
                paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var5, extraout_DX_00);
                unaff_CS = 0x30;
                u_var12 = pass1_1030_73a8(CONCAT22(u_var6, paVar4));
                iVar7 = (u_var12 + 0x12);
                u_var6 = u_var12 + 0x14;
                u_var5 = u_var6;
                local_1c = u_var12 & 0xffff0000 | u_var6;
                local_20 = 0;
                if ((iVar7 == 4) || (iVar7 == 5)) {
                    u_var5 = local_1c;
                    local_20 = u_var5;
                }
                if (local_20 != 0) {
                    local_22 = 0x11;
                    while (local_22 < 0x25) {
                        if (((local_BX_5.field_0x204 == 0) || (local_22 == 0x23))
                            || (local_22 == 0x24))
                        {
                            pass1_1038_540a(param_1, local_22);
                            iVar7 = local_22 * 4;
                            u_var10 = (local_20 >> 0x10);
                            iVar8 = local_20;
                            pu_var1 = (iVar7 + iVar8 + 2);
                            unsafe {
                                bVar11 = *pu_var1 < extraout_DX_01;
                                if ((bVar11 || *pu_var1 == extraout_DX_01)
                                    && (bVar11
                                        || (
                                            pu_var1 = (iVar7 + iVar8),
                                            *pu_var1 < u_var5 || *pu_var1 == u_var5,
                                        )))
                                {
                                    pass1_1038_5770(param_1, (iVar7 + iVar8), local_22);
                                }
                            }
                        }
                        local_22 = local_22 + 1;
                    }
                }
            }
            local_a = local_a + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_52b8(param_1: *mut AStruct1121, param_2: u32, param_3: *mut AStruct1125) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut iVar3: i32;
    let paVar4: *mut AStruct493;
    let mut u_var5: u32;
    let mut extraout_DX: u16;
    let mut u_var6: u16;
    let mut extraout_DX_00: i32;
    let mut u_var7: i32;
    let mut iVar8: i32;
    let mut u_var9: u16;
    let mut unaff_CS: u16;
    let local_24: *mut AStruct1121;
    let uStack34: u8;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    iVar3 = -param_2;
    pass1_1038_5694(
        param_1,
        CONCAT22(-(param_2._2_2_ + (param_2 != 0)), iVar3),
        param_3,
    );
    if (param_3 != (s_New_failed_in_Op__Op_1050_0020 + 4)) {
        u_var9 = (param_1 >> 0x10);
        iVar8 = param_1;
        if ((iVar8 + 0xc) == 0) {
            iVar3 = 0;
            u_var6 = 0;
        } else {
            ppcVar2 = ((iVar8 + 0xc) + 0x10);
            ppcVar2();
            u_var6 = extraout_DX;
        }
        _local_a = CONCAT22(u_var6, iVar3);
        local_e = 0;
        while (local_e < _local_a) {
            uStack30 = local_e;
            uStack28 = (local_e >> 0x10);
            u_var1 = (iVar8 + 0xc);
            uStack34 = u_var1;
            ppcVar2 = ((iVar8 + 0xc) + 4);
            u_var5 = _local_a;
            ppcVar2(unaff_CS, uStack34, (u_var1 >> 0x10), uStack30, uStack28);
            u_var7 = extraout_DX_00 | u_var5;
            if (u_var7 != 0) {
                paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var5, extraout_DX_00);
                local_16 = CONCAT22(u_var7, paVar4);
                unaff_CS = 0x1030;
                pass1_1030_7c28(CONCAT22(u_var7, paVar4), param_3);
                local_1a = CONCAT22(u_var7, paVar4);
                if ((u_var7 | paVar4) != 0) {
                    if (local_1a < param_2) {
                        param_2 = param_2 - local_1a;
                        local_1a = 0;
                    } else {
                        local_1a = CONCAT22(
                            (u_var7 - param_2._2_2_) - (paVar4 < param_2),
                            (paVar4 - param_2),
                        );
                        param_2 = 0;
                    }
                    uStack30 = (local_1a >> 0x10);
                    unaff_CS = 0x1030;
                    pass1_1030_7d1c(local_16, local_1a, CONCAT22(param_3, uStack30));
                    if (param_2 == 0) {
                        return;
                    }
                }
            }
            local_e = local_e + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_3f38(param_1: *mut u32, param_2: *mut u32, param_3: u32) {
    let ppcVar1: fn();
    let u_var2: u8;
    let paVar3: *mut AStruct493;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut u_var4: i32;
    let pu_var5: *mut u32;
    let mut u_var6: u16;
    let mut u_var7: u32;
    let mut u_var8: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let local_6: *mut AStruct1100;
    let mut local_4: u16;

    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_3, (param_3 >> 0x10));
    _local_6 = CONCAT22(in_DX, paVar3);
    u_var4 = in_DX;
    u_var2 = pass1_1028_b58e(CONCAT22(in_DX, paVar3));
    u_var7 = (CONCAT31(extraout_var, u_var2) + 4);
    unsafe {
        ppcVar1 = (*param_1 + 0x18);
    }
    (**ppcVar1)(&PTR_LOOP_1050_1028, param_1, u_var7);
    u_var8 = 0;
    u_var6 = 0;
    unsafe {
        ppcVar1 = (*param_2 + 8);
    }
    pu_var5 = param_2;
    (**ppcVar1)();
    pass1_1030_73ee(
        (CONCAT31(extraout_var, u_var2) & 0xffff | u_var4 << 0x10),
        (param_2 + 4),
    );
    ppcVar1 = (*_local_6 + 0x58);
    (**ppcVar1)(
        0x1030, paVar3, in_DX, param_2, pu_var5, u_var6, u_var7, u_var8,
    );
    return;
}

pub unsafe fn pass1_1038_3fb0(param_1: *mut u8) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0x200);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    return;
}

pub unsafe fn pass1_1038_3fca(param_1: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut in_AX: i32;
    let local_AX_84: *mut AStruct1103;
    let local_AX_140: *mut AStruct1102;
    let paVar3: *mut AStruct493;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut u_var4: i32;
    let local_BX_5: *mut AStruct1101;
    let mut iVar5: i32;
    let mut unaff_SI: u16;
    let mut iVar6: i32;
    let pu_var7: *mut u8;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_SS: u16;
    let mut u_var10: u32;
    let ppVar11: *mut pass1_struct_1;
    let u_var12: u8;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let mut local_3a: u32;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: [u8; 2];
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var7 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (local_BX_5.field_0xc == 0) {
        in_AX = 0;
        u_var4 = 0;
    } else {
        ppcVar2 = (local_BX_5.field_0xc + 0x10);
        ppcVar2();
        u_var4 = extraout_DX;
    }
    _local_6 = CONCAT22(u_var4, in_AX);
    g_u16_ptr_1050_5f2e = (u_var4 | in_AX);
    if (g_u16_ptr_1050_5f2e != 0x0) {
        if (__g_AStruct94_ptr_1 == 0) {
            struct_fn_1000_160a();
            _g_AStruct94_ptr_1._0_1_ = in_AX;
        } else {
        }
        local_AX_84 = (local_6 << 2);
        alloc_mem_1000_1708(
            local_AX_84,
            0,
            1,
            _g_AStruct94_ptr_1._0_1_,
            g_u16_ptr_1050_5f2e,
        );
        _local_a = CONCAT22(g_u16_ptr_1050_5f2e, local_AX_84);
        if (__g_AStruct94_ptr_1 == 0) {
            struct_fn_1000_160a();
            _g_AStruct94_ptr_1._0_1_ = SUB21(local_AX_84, 0);
        } else {
        }
        local_AX_140 = (local_6 << 2);
        u_var9 = 0x1000;
        alloc_mem_1000_1708(
            local_AX_140,
            0,
            1,
            _g_AStruct94_ptr_1._0_1_,
            g_u16_ptr_1050_5f2e,
        );
        _local_e = CONCAT22(g_u16_ptr_1050_5f2e, local_AX_140);
        local_16 = 0;
        while (local_16 < _local_6) {
            u_var1 = local_BX_5.field_0xc;
            ppcVar2 = (local_BX_5.field_0xc + 4);
            u_var10 = _local_6;
            ppcVar2(
                u_var9,
                u_var1,
                (u_var1 >> 0x10),
                local_16,
                (local_16 >> 0x10),
            );
            local_12 = u_var10;
            u_var4 = extraout_DX_00 | local_12;
            local_10 = extraout_DX_00;
            if (u_var4 != 0) {
                paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_12, extraout_DX_00);
                iVar5 = local_16 * 4;
                u_var8 = (_local_a >> 0x10);
                iVar6 = _local_a;
                (iVar5 + iVar6) = paVar3;
                (iVar5 + iVar6 + 2) = u_var4;
                u_var9 = 0x1030;
                u_var10 = pass1_1030_73a8(CONCAT22(u_var4, (iVar5 + iVar6)));
                u_var8 = (_local_e >> 0x10);
                (_local_e + iVar5) = u_var10;
                (_local_e + iVar5 + 2) = (u_var10 >> 0x10);
            }
            local_16 = local_16 + 1;
        }
        local_16 = 0;
        while (local_16 < _local_6) {
            u_var9 = (_local_e >> 0x10);
            iVar5 = _local_e;
            if (((local_16 * 4 + iVar5) != 0)
                && (
                    u_var1 = (local_16 * 4 + iVar5),
                    (u_var1 + 0x1a) = 0,
                    u_var1 = (local_16 * 4 + iVar5),
                    (u_var1 + 0x12) == 5,
                ))
            {
                pass1_1028_bdac((local_16 * 4 + iVar5), 6);
            }
            local_16 = local_16 + 1;
        }
        local_BX_5.field_0x204 = 0;
        ppVar11 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_SI, 2));
        local_1e = (ppVar11 >> 0x10);
        local_1a = ppVar11;
        local_1c = u16_1050_13ae;
        if (u16_1050_13ae == 1) {
            local_BX_5.field_0x204 = 1;
        }
        local_18 = local_1e;
        pass1_1038_5a96(local_BX_5, pu_var7, _local_6, _local_e);
        pass1_1038_5cc6(param_1, _local_6, _local_e, _local_a, 0, 2);
        pass1_1038_5b3c(local_BX_5, pu_var7, _local_6, _local_e);
        pass1_1038_5cc6(param_1, _local_6, _local_e, _local_a, 0, 1);
        u_var14 = SUB21(local_22, 0);
        u_var15 = (local_22 >> 8);
        u_var12 = SUB21(&local_24, 0);
        u_var13 = (&local_24 >> 8);
        u_var1 = local_BX_5.field_0x8;
        u_var9 = unaff_SS;
        local_20 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        pass1_1030_5b1c(
            CONCAT22(local_1e, local_20),
            CONCAT22(unaff_SS, CONCAT11(u_var13, u_var12)),
            CONCAT22(u_var9, CONCAT11(u_var15, u_var14)),
        );
        local_26 = 1;
        while (local_26 <= local_24) {
            pass1_1038_58e6(local_BX_5, pu_var7, _local_6, _local_e, _local_a, local_26);
            pass1_1038_5cc6(param_1, _local_6, _local_e, _local_a, local_26, 3);
            local_26 = local_26 + 1;
        }
        pass1_1038_5a16(local_BX_5, pu_var7, _local_6, _local_e);
        local_16 = 0;
        while (local_16 < _local_6) {
            u_var9 = (_local_e >> 0x10);
            iVar5 = _local_e;
            if (((local_16 * 4 + iVar5) != 0)
                && (u_var1 = (local_16 * 4 + iVar5), (u_var1 + 0x12) != 5))
            {
                u_var1 = (local_16 * 4 + iVar5);
                ppcVar2 = ((local_16 * 4 + iVar5) + 0x28);
                ppcVar2(0x1030, u_var1, (u_var1 >> 0x10));
            }
            local_16 = local_16 + 1;
        }
        error_check_1000_17ce(_local_a);
        error_check_1000_17ce(_local_e);
    }
    return;
}

pub unsafe fn pass1_1038_42cc(param_1: *mut AStruct1104) {
    let paVar1: *mut AStruct872;
    let ppcVar2: fn();
    let mut bVar3: bool;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: i32;
    let pu_var7: *mut u8;
    let mut u_var8: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let local_BX_4: *mut AStruct1104;
    let local_ES_4: *mut AStruct1104;
    let mut u_var9: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x1f6 == 0x0) {
        return;
    }
    u_var9 = SUB42(&PTR_LOOP_1050_1008, 0);
    pu_var7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2d);
    u_var4 = SUB42(pu_var7, 0);
    pass1_1038_4d6e(param_1, pu_var7 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, u_var4);
    ppcVar2 = (*_local_a + 0x10);
    u_var5 = u_var4;
    ppcVar2(&PTR_LOOP_1050_1008, u_var4, extraout_DX);
    _local_12 = CONCAT22(extraout_DX_00, u_var5);
    bVar3 = false;
    local_18 = 0;
    while (local_18 < _local_12) {
        u_var9 = 0x1030;
        u_var8 = _local_12;
        pass1_1030_1d7c(_local_a, local_18, (local_18 >> 0x10));
        u_var6 = u_var8;
        if ((extraout_DX_01 | u_var6) != 0) {
            ppcVar2 = ((u_var8 & 0xffff | extraout_DX_01 << 0x10) + 0x50);
            ppcVar2();
            if (u_var6 != 0) {
                bVar3 = true;
            }
        }
        local_18 = local_18 + 1;
    }
    if (bVar3) {
        paVar1 = local_BX_4.field_0x1f6;
        (paVar1 + 0x1aa) = 0;
    } else {
        paVar1 = local_BX_4.field_0x1f6;
        u_var9 = 0x1030;
        pass1_1030_38b8(paVar1, (paVar1 >> 0x10));
        if ((extraout_DX_02 | _local_12) != 0) {
            u_var9 = 0x1030;
            pass1_1030_326a(local_BX_4.field_0x1f6);
        }
    }
    if (_local_a != 0x0) {
        ppcVar2 = *_local_a;
        ppcVar2(u_var9, u_var4, extraout_DX, 1);
    }
    return;
}

pub unsafe fn pass1_1038_43cc(param_1: *mut AStruct1106, param_2: u32, param_3: u16) {
    let ppcVar1: fn();
    let mut in_AX: i32;
    let mut u_var2: u16;
    let mut uvar3: u16;
    let mut u_var4: i32;
    let pu_var5: *mut u8;
    let mut u_var6: u32;
    let extraout_DL: u8;
    let mut in_DX: i32;
    let mut u_var7: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let local_SI_56: *mut AStruct1105;
    let mut iVar8: i32;
    let mut u_var9: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;

    if (param_3 == 5) {
        pass1_1038_4900(CONCAT22(param_2, param_1));
        return;
    }
    pass1_1038_53ba(CONCAT22(param_2, param_1), param_3);
    if ((in_DX != 0) || (in_AX != 0)) {
        local_SI_56 = (param_3 * 4);
        u_var4 = (param_1 + local_SI_56 + 0x14e);
        u_var7 = param_2._2_2_ >> 0xf;
        iVar8 = ((param_1 + local_SI_56 + 0x150) - u_var7) - (u_var4 < param_2._2_2_);
        (param_1 + local_SI_56 + 0x14e) = u_var4 - param_2._2_2_;
        (param_1 + local_SI_56 + 0x150) = iVar8;
        if (iVar8 < 0) {
            (param_1 + local_SI_56 + 0x14e) = 0;
        }
        u_var9 = SUB42(&PTR_LOOP_1050_1008, 0);
        pu_var5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        u_var2 = SUB42(pu_var5, 0);
        pass1_1038_4e78(
            CONCAT22(param_2, param_1),
            pu_var5 & 0xffff | u_var7 << 0x10,
        );
        _local_e = CONCAT22(extraout_DX, u_var2);
        ppcVar1 = (*_local_e + 0x10);
        u_var3 = u_var2;
        (**ppcVar1)(&PTR_LOOP_1050_1008, u_var2, extraout_DX);
        _local_12 = CONCAT22(extraout_DX_00, u_var3);
        local_16 = 0;
        while (local_16 < _local_12) {
            u_var6 = _local_12;
            pass1_1030_1d7c(u_var2, extraout_DX, local_16, (local_16 >> 0x10));
            param_2._2_2_ = (param_2 >> 0x10);
            u_var3 = u_var6;
            while (u_var4 = u_var6, param_2._2_2_ != 0) {
                pass1_1030_cf78(u_var3, extraout_DL, param_3);
                u_var6 = u_var4;
                if (u_var4 == 0) {
                    break;
                }
                param_2._2_2_ = param_2._2_2_ - 1;
            }
            param_2 = param_2._2_2_ << 0x10;
            u_var9 = 0x1030;
            if (param_2._2_2_ == 0) {
                break;
            }
            local_16 = local_16 + 1;
        }
        if (_local_e != 0x0) {
            ppcVar1 = *_local_e;
            (**ppcVar1)(u_var9, u_var2, extraout_DX, 1);
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1038_44d8(
    param_1: *mut AStruct998,
    param_2: *mut u8,
    uparam_3: i32,
    param_3_00: *mut u8,
) {
    let ppcVar1: fn();
    let paVar2: *mut AStruct44;
    let mut in_AX: i32;
    let local_AX_121: *mut u8;
    let pu_var3: *mut u8;
    let mut u_var4: i32;
    let pu_var5: *mut u8;
    let mut u_var6: u32;
    let mut in_DX: i32;
    let mut u_var7: i32;
    let local_DX_128: *mut u8;
    let local_DX_145: *mut u8;
    let extraout_DX: *mut AStruct44;
    let local_SI_56: *mut AStruct1107;
    let mut iVar8: i32;
    let local_CS_115: *mut u8;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let local_e: *mut AStruct1108;
    let mut local_c: u16;

    if (param_3_00 == (&PTR_DAT_0005_0000_1050_0004 + 1)) {
        pass1_1038_4900(CONCAT22(param_2, param_1));
        return;
    }
    pass1_1038_53ba(CONCAT22(param_2, param_1), param_3_00);
    if ((in_DX != 0) || (in_AX != 0)) {
        local_SI_56 = (param_3_00 * 4);
        u_var4 = (param_1 + local_SI_56 + 0x14e);
        u_var7 = param_3 >> 0xf;
        iVar8 = ((param_1 + local_SI_56 + 0x150) - u_var7) - (u_var4 < param_3);
        (param_1 + local_SI_56 + 0x14e) = u_var4 - param_3;
        (param_1 + local_SI_56 + 0x150) = iVar8;
        if (iVar8 < 0) {
            (param_1 + local_SI_56 + 0x14e) = 0;
        }
        local_CS_115 = &PTR_LOOP_1050_1008;
        pu_var5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        local_AX_121 = pu_var5;
        pass1_1038_4e78(
            CONCAT22(param_2, param_1),
            pu_var5 & 0xffff | u_var7 << 0x10,
        );
        _local_e = CONCAT22(local_DX_128, local_AX_121);
        ppcVar1 = (*_local_e + 0x10);
        pu_var3 = local_AX_121;
        (**ppcVar1)(&PTR_LOOP_1050_1008, local_AX_121, local_DX_128);
        _local_12 = CONCAT22(local_DX_145, pu_var3);
        local_16 = 0;
        while (local_16 < _local_12) {
            u_var6 = _local_12;
            pass1_1030_1d7c(local_AX_121, local_DX_128, local_16, (local_16 >> 0x10));
            paVar2 = u_var6;
            while (u_var4 = u_var6, param_3 != 0) {
                pass1_1030_d00c(paVar2, extraout_DX, param_3_00);
                u_var6 = u_var4;
                if (u_var4 == 0) {
                    break;
                }
                param_3 = param_3 - 1;
            }
            local_CS_115 = 0x1030;
            if (param_3 == 0) {
                break;
            }
            local_16 = local_16 + 1;
        }
        if (_local_e != 0x0) {
            ppcVar1 = *_local_e;
            (**ppcVar1)(local_CS_115, local_AX_121, local_DX_128, 1);
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1038_3698(param_1: *mut AStruct1091) {
    let piVar1: *mut i32;
    AStruct1093 * *ppaVar2;
    AStruct1094 * *ppaVar3;
    let paVar4: *mut AStruct1093;
    let mut u_var5: u32;
    let ppcVar6: fn();
    let mut in_AX: u16;
    let mut u_var7: u16;
    let BVar8: bool;
    let mut u_var9: i32;
    let local_AX_338: *mut AStruct1092;
    let local_AX_462: *mut AStruct1092;
    let mut u_var10: i32;
    let mut u_var11: u32;
    let mut in_DX: u16;
    let mut u_var13: i32;
    let local_DX_239: *mut AStruct1094;
    let local_DX_299: *mut AStruct1093;
    let mut u_var14: i32;
    let mut u_var15: u32;
    let local_BX_4: *mut AStruct1091;
    let local_ES_4: *mut AStruct1091;
    let mut u_var16: u32;
    let u_var17: u8;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut u_var12: u32;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x214 == 0) {
        return;
    }
    u_var5 = local_BX_4.field_0x1f6;
    pass1_1030_38b8(u_var5, (u_var5 >> 0x10));
    _local_6 = CONCAT22(in_DX, in_AX);
    _local_6 = _local_6 - &local_BX_4.field_0x216;
    if (0 < _local_6) {
        _local_6 = _local_6 + 3;
        local_a = _local_6 / 5;
        u_var15 = _local_6 % 5;
        if (local_BX_4.field_0xc == 0) {
            u_var7 = 0;
            u_var15 = 0;
        } else {
            u_var5 = local_BX_4.field_0xc;
            ppcVar6 = (local_BX_4.field_0xc + 0x10);
            u_var11 = local_a;
            (**ppcVar6)(0x1030, u_var5, (u_var5 >> 0x10));
            u_var7 = u_var11;
        }
        _local_e = CONCAT22(u_var15, u_var7);
        local_12 = 0;
        while (u_var14 = u_var15, u_var11 = _local_e, local_12 < _local_e) {
            u_var5 = local_BX_4.field_0xc;
            u_var12 = _local_e;
            pass1_1030_1d7c(u_var5, (u_var5 >> 0x10), local_12, (local_12 >> 0x10));
            u_var10 = u_var12;
            u_var15 = (u_var14 | u_var10);
            if ((u_var14 | u_var10) != 0) {
                BVar8 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (u_var10 + 0xc), 4);
                u_var11 = BVar8;
                if (BVar8 != 0) {
                    u_var16 = pass1_1028_678c(u_var12 & 0xffff | u_var14 << 0x10, 0xf);
                    u_var9 = (u_var16 >> 0x10);
                    u_var13 = u_var9 | (u_var16 & 0xffff);
                    u_var15 = u_var13;
                    u_var11 = u_var16 & 0xffff;
                    if (u_var13 != 0) {
                        u_var17 = (u_var14 >> 8);
                        if (local_a < u_var16) {
                            u_var9 = local_a;
                            pass1_1028_6356(
                                CONCAT13(u_var17, CONCAT12(u_var14, u_var10)),
                                0xf,
                                u_var9,
                                local_a._2_2_,
                            );
                            u_var14 = u_var9 * 5;
                            local_DX_239 = (local_a._2_2_ * 5
                                + CARRY2(u_var9, u_var9) * 2
                                + CARRY2(u_var9 * 2, u_var9 * 2)
                                + CARRY2(u_var9 * 4, u_var9));
                            u_var15 = ZEXT24(local_DX_239);
                            ppaVar2 = &local_BX_4.field_0x216;
                            paVar4 = *ppaVar2;
                            *ppaVar2 = *ppaVar2 + u_var14;
                            ppaVar3 = &local_BX_4.field_0x218;
                            *ppaVar3 = local_DX_239 + (*ppaVar3 + CARRY2(paVar4, u_var14));
                            local_a = 0;
                            u_var11 = u_var14;
                        } else {
                            u_var13 = u_var16;
                            pass1_1028_6356(
                                CONCAT13(u_var17, CONCAT12(u_var14, u_var10)),
                                0xf,
                                u_var13,
                                u_var9,
                            );
                            local_DX_299 = (u_var9 * 5
                                + CARRY2(u_var13, u_var13) * 2
                                + CARRY2(u_var13 * 2, u_var13 * 2)
                                + CARRY2(u_var13 * 4, u_var13));
                            u_var15 = ZEXT24(local_DX_299);
                            ppaVar2 = &local_BX_4.field_0x216;
                            paVar4 = *ppaVar2;
                            *ppaVar2 = *ppaVar2 + u_var13 * 5;
                            ppaVar2 = &local_BX_4.field_0x218;
                            *ppaVar2 = local_DX_299 + (*ppaVar2 + CARRY2(paVar4, u_var13 * 5));
                            local_a = local_a - u_var16;
                            u_var11 = u_var16;
                        }
                    }
                }
                u_var14 = u_var15;
                if (local_a == 0) {
                    break;
                }
            }
            local_12 = local_12 + 1;
        }
        local_AX_338 = u_var11;
        u_var5 = local_BX_4.field_0x1f6;
        pass1_1030_38b8(u_var5, (u_var5 >> 0x10));
        _local_6 = CONCAT22(u_var14, local_AX_338);
        _local_6 = _local_6 - &local_BX_4.field_0x216;
        local_4 = (_local_6 >> 0x10);
        if ((local_4 | local_6) != 0) {
            _local_20 = _local_6 / local_BX_4.field_0x214;
            if (_local_20 < 1) {
                _local_20 = 1;
            }
            u_var5 = local_BX_4.field_0x1f6;
            pass1_1030_375a(u_var5, (u_var5 >> 0x10), 0, _local_20, (_local_20 >> 0x10));
        }
    }
    piVar1 = &local_BX_4.field_0x214;
    unsafe {
        *piVar1 = *piVar1 + -1;
    }
    return;
}

pub unsafe fn pass1_1038_387e(param_1: *mut AStruct1095, param_2: u16, param_3: u16, param_4: u32) {
    let ppcVar1: fn();
    let mut u_var2: u32;
    let u_var3: u8;
    let mut u_var4: u16;
    let paVar5: *mut AStruct987;
    let mut u_var6: i32;
    let extraout_var: u32;
    let mut u_var7: u32;
    let extraout_var_00: u32;
    let mut u_var8: u32;
    let mut extraout_DX: u16;
    let mut u_var9: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: i32;
    let local_BX_14: *mut AStruct1095;
    let local_ES_14: *mut AStruct1095;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 != param_3) {
        local_BX_14 = param_1;
        local_ES_14 = (param_1 >> 0x10);
        if (param_2 < param_3) {
            local_c = param_3 - param_2;
            if ((local_BX_14.field_0x210 == 0)
                || (u_var2 = local_BX_14.field_0x210, (u_var2 + 10) == 0))
            {
                if (local_BX_14.field_0xc == 0x0) {
                    u_var4 = 0;
                    u_var9 = 0;
                } else {
                    ppcVar1 = (*local_BX_14.field_0xc + 0x10);
                    u_var4 = local_c;
                    (**ppcVar1)();
                    u_var9 = extraout_DX;
                }
                _local_6 = CONCAT22(u_var9, u_var4);
                local_a = 0;
                while (local_a < _local_6) {
                    u_var7 = _local_6;
                    pass1_1030_1d58(local_BX_14.field_0xc);
                    if (((extraout_DX_00 | u_var7) != 0)
                        && (
                            u_var3 = pass1_1030_6fa0((u_var7 & 0xffff | extraout_DX_00 << 0x10)),
                            CONCAT31(extraout_var, u_var3) == 0xb,
                        ))
                    {
                        pass1_1030_7c50(
                            CONCAT13((extraout_DX_00 >> 8), CONCAT12(extraout_DX_00, u_var7)),
                            local_c,
                            4,
                        );
                        return;
                    }
                    local_a = local_a + 1;
                }
            } else {
                u_var2 = local_BX_14.field_0x210;
                u_var7 = (u_var2 + 10);
                local_a = 0;
                while (local_a < u_var7) {
                    u_var2 = local_BX_14.field_0x210;
                    u_var8 = u_var7;
                    pass1_1030_1312(u_var2, (u_var2 >> 0x10), local_a, (local_a >> 0x10));
                    paVar5 = u_var8;
                    if ((((extraout_DX_01 | paVar5) != 0)
                        && (
                            pass1_1030_cc44(paVar5, extraout_DX_01, local_c, param_4, 4),
                            paVar5 != 0x0,
                        ))
                        && (local_c = local_c - paVar5, local_c == 0))
                    {
                        return;
                    }
                    local_a = local_a + 1;
                }
            }
        } else {
            local_16 = param_2 - param_3;
            if ((local_BX_14.field_0x210 == 0)
                || (u_var2 = local_BX_14.field_0x210, (u_var2 + 10) == 0))
            {
                if (local_BX_14.field_0xc == 0x0) {
                    u_var4 = 0;
                    u_var9 = 0;
                } else {
                    ppcVar1 = (*local_BX_14.field_0xc + 0x10);
                    u_var4 = local_16;
                    (**ppcVar1)();
                    u_var9 = extraout_DX_02;
                }
                _local_6 = CONCAT22(u_var9, u_var4);
                local_a = 0;
                while (local_a < _local_6) {
                    u_var7 = _local_6;
                    pass1_1030_1d58(local_BX_14.field_0xc);
                    if (((extraout_DX_03 | u_var7) != 0)
                        && (
                            u_var3 = pass1_1030_6fa0((u_var7 & 0xffff | extraout_DX_03 << 0x10)),
                            CONCAT31(extraout_var_00, u_var3) == 0xb,
                        ))
                    {
                        pass1_1030_6e9c(
                            CONCAT13((extraout_DX_03 >> 8), CONCAT12(extraout_DX_03, u_var7)),
                            local_16,
                            4,
                        );
                        return;
                    }
                    local_a = local_a + 1;
                }
            } else {
                u_var2 = local_BX_14.field_0x210;
                u_var7 = (u_var2 + 10);
                local_a = 0;
                while (local_a < u_var7) {
                    u_var2 = local_BX_14.field_0x210;
                    u_var8 = u_var7;
                    pass1_1030_1312(u_var2, (u_var2 >> 0x10), local_a, (local_a >> 0x10));
                    u_var6 = u_var8;
                    if ((extraout_DX_04 | u_var6) != 0) {
                        pass1_1030_ce72(
                            extraout_DX_04 << 0x10 | u_var8 & 0xffff,
                            local_16,
                            param_4,
                            4,
                        );
                        local_16 = local_16 - u_var6;
                        if (local_16 == 0) {
                            return;
                        }
                    }
                    local_a = local_a + 1;
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1038_3aa6(param_1: *mut AStruct1096) {
    let ppcVar1: fn();
    let mut u_var2: u32;
    let u_var3: u8;
    let mut in_AX: u16;
    let mut u_var4: u32;
    let extraout_var: u32;
    let mut u_var5: u32;
    let mut extraout_DX: u16;
    let mut u_var6: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let local_BX_9: *mut AStruct1096;
    let mut u_var7: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = (param_1 >> 0x10);
    local_BX_9 = param_1;
    if ((local_BX_9.field_0x210 == 0) || (u_var2 = local_BX_9.field_0x210, (u_var2 + 10) == 0)) {
        if (local_BX_9.field_0xc == 0x0) {
            in_AX = 0;
            u_var6 = 0;
        } else {
            ppcVar1 = (*local_BX_9.field_0xc + 0x10);
            (**ppcVar1)();
            u_var6 = extraout_DX;
        }
        _local_8 = CONCAT22(u_var6, in_AX);
        local_c = 0;
        while (local_c < _local_8) {
            u_var4 = _local_8;
            pass1_1030_1d58(local_BX_9.field_0xc);
            if (((extraout_DX_00 | u_var4) != 0)
                && (
                    u_var3 = pass1_1030_6fa0((u_var4 & 0xffff | extraout_DX_00 << 0x10)),
                    CONCAT31(extraout_var, u_var3) == 0xb,
                ))
            {
                pass1_1030_6b86(u_var4 & 0xffff | extraout_DX_00 << 0x10);
                return;
            }
            local_c = local_c + 1;
        }
    } else {
        u_var2 = local_BX_9.field_0x210;
        u_var4 = (u_var2 + 10);
        local_c = 0;
        while (local_c < u_var4) {
            u_var2 = local_BX_9.field_0x210;
            u_var5 = u_var4;
            pass1_1030_1312(u_var2, (u_var2 >> 0x10), local_c, (local_c >> 0x10));
            if ((extraout_DX_01 | u_var5) != 0) {
                pass1_1030_ce2e(u_var5, extraout_DX_01, 4);
            }
            local_c = local_c + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_3ba0(param_1: *mut AStruct1097) {
    let pu_var1: *mut u32;
    let ppcVar2: fn();
    let mut u_var3: u32;
    let pu_var4: *mut u32;
    let mut u_var5: u32;
    let mut u_var6: u32;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let pu_var9: *mut u8;
    let mut u_var10: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let struct_a: *mut AStruct199;
    let paVar11: *mut AStruct199;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: i32;
    let local_BX_5: *mut AStruct1097;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_14: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var12 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    pu_var1 = local_BX_5.field_0x210;
    u_var7 = &local_BX_5.field_0x212;
    if ((u_var7 | pu_var1) != 0) {
        unsafe {
            ppcVar2 = *pu_var1;
        }
        ppcVar2();
        u_var7 = extraout_DX;
    }
    pu_var9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
    pass1_1038_4d6e(param_1, pu_var9 & 0xffff | u_var7 << 0x10);
    u_var5 = pu_var9 & 0xffff;
    pu_var4 = (u_var5 | extraout_DX_00 << 0x10);
    unsafe {
        ppcVar2 = (*pu_var4 + 0x10);
    }
    ppcVar2(&PTR_LOOP_1050_1008, pu_var9, extraout_DX_00);
    u_var7 = pu_var9;
    u_var6 = pu_var9 & 0xffff | ZEXT24(struct_a) << 0x10;
    if ((struct_a == 0x0) && (u_var7 < 5)) {
        u_var7 = 5;
    }
    u_var7 = u_var7 + 1;
    u_var13 = 0x1000;
    paVar11 = struct_a;
    u_var8 = u_var7;
    process_struct_1000_179c(0x1c, struct_a);
    if ((paVar11 | u_var8) == 0) {
        &local_BX_5.field_0x210 = 0;
    } else {
        u_var13 = 0x1030;
        pass1_1030_11aa(CONCAT22(paVar11, u_var8), 5, u_var7, u_var7 >> 0xf);
        local_BX_5.field_0x210 = u_var7;
        &local_BX_5.field_0x212 = extraout_DX_01;
    }
    u_var3 = &local_BX_5.field_0x210;
    (u_var3 + 0x1a) = 0;
    local_14 = 0;
    while (local_14 < u_var6) {
        u_var10 = u_var6;
        pass1_1030_1d7c(pu_var4, local_14, (local_14 >> 0x10));
        if ((extraout_DX_02 | u_var10) != 0) {
            pass1_1030_1358(
                &local_BX_5.field_0x210,
                u_var10,
                extraout_DX_02,
                local_14 + 1,
            );
        }
        u_var13 = 0x1030;
        local_14 = local_14 + 1;
    }
    if (pu_var4 != 0x0) {
        unsafe {
            ppcVar2 = *pu_var4;
        }
        ppcVar2(u_var13, u_var5, extraout_DX_00, 1);
    }
    return;
}

pub unsafe fn pass1_1038_3cc0(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let lVar1: u32;
    let ppcVar2: fn();
    let u_var3: u8;
    let mut u_var4: u16;
    let pu_var5: *mut u32;
    let extraout_AH: u8;
    let pu_var6: *mut u8;
    let mut u_var7: u32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: u16;
    let mut extraout_DX_05: u16;
    let mut extraout_DX_06: i32;
    let mut extraout_DX_07: i32;
    let mut u_var8: i32;
    let pu_var9: *mut u32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u32;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let local_1a: *mut AStruct1099;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let local_a: *mut AStruct1098;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3_00 == 0x1e) {
        u_var11 = SUB42(&PTR_LOOP_1050_1008, 0);
        pu_var6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x27);
        pu_var9 = pu_var6;
        pass1_1038_4e78(param_1, pu_var6 & 0xffff | in_DX << 0x10);
        _local_a = CONCAT22(extraout_DX_01, pu_var9);
        ppcVar2 = (*_local_a + 0x10);
        pu_var5 = pu_var9;
        ppcVar2(&PTR_LOOP_1050_1008, pu_var9, extraout_DX_01);
        _local_e = CONCAT22(extraout_DX_02, pu_var5);
        local_12 = 0;
        while (local_12 < _local_e) {
            u_var7 = _local_e;
            pass1_1030_1d7c(pu_var9, extraout_DX_01, local_12, (local_12 >> 0x10));
            if ((extraout_DX_03 | u_var7) != 0) {
                u_var12 = pass1_1030_bfb8((u_var7 & 0xffff | extraout_DX_03 << 0x10));
                u_var8 = (u_var12 >> 0x10) | u_var12;
                if (u_var8 != 0) {
                    u_var3 = pass1_1028_b58e((u_var7 & 0xffff | extraout_DX_03 << 0x10));
                    if (CONCAT22(param_3, param_2) <= u_var12) {
                        u_var11 = 0x1030;
                        pass1_1030_7ddc(
                            CONCAT31(extraout_var, u_var3) & 0xffff | u_var8 << 0x10,
                            CONCAT13((param_3 >> 8), CONCAT12(param_3, param_2)),
                            0x1e,
                        );
                        break;
                    }
                    pass1_1030_7ddc(
                        CONCAT31(extraout_var, u_var3) & 0xffff | u_var8 << 0x10,
                        u_var12,
                        0x1e,
                    );
                    lVar1 = CONCAT22(param_3, param_2) - u_var12;
                    param_2 = lVar1;
                    param_3 = (lVar1 >> 0x10);
                }
            }
            u_var11 = 0x1030;
            local_12 = local_12 + 1;
        }
        _local_1a = _local_a;
        u_var10 = extraout_DX_01;
        if (_local_a == 0x0) {
            return;
        }
    } else {
        if (param_3_00 != 0x21) {
            u_var11 = SUB42(&PTR_LOOP_1050_1008, 0);
            pu_var6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 3);
            u_var4 = SUB42(pu_var6, 0);
            pass1_1038_4e78(param_1, pu_var6 & 0xffff | in_DX << 0x10);
            _local_1a = CONCAT22(extraout_DX, u_var4);
            ppcVar2 = (*_local_1a + 0x10);
            u_var10 = u_var4;
            ppcVar2(&PTR_LOOP_1050_1008, u_var4, extraout_DX);
            _local_16 = CONCAT22(extraout_DX_00, u_var10);
            local_12 = 0;
            // LAB_1038_3e9c:
            if (local_12 < _local_16) {
                u_var11 = 0x1030;
                u_var7 = _local_16;
                pass1_1030_1d7c(u_var4, extraout_DX, local_12, (local_12 >> 0x10));
                if ((extraout_DX_07 | u_var7) == 0) {}
                // goto LAB_1038_3e98;
                u_var11 = SUB42(&PTR_LOOP_1050_1028, 0);
                u_var12 = pass1_1028_45e2(u_var7 & 0xffff | extraout_DX_07 << 0x10);
                u_var8 = (u_var12 >> 0x10) | u_var12;
                if (u_var8 == 0) {}
                // goto LAB_1038_3e98;
                u_var3 = pass1_1028_b58e((u_var7 & 0xffff | extraout_DX_07 << 0x10));
                if (u_var12 < CONCAT22(param_3, param_2)) {
                    u_var11 = 0x1030;
                    pass1_1030_7ddc(
                        CONCAT31(extraout_var_00, u_var3) & 0xffff | u_var8 << 0x10,
                        u_var12,
                        param_3_00,
                    );
                    lVar1 = CONCAT22(param_3, param_2) - u_var12;
                    param_2 = lVar1;
                    param_3 = (lVar1 >> 0x10);
                    // goto LAB_1038_3e98;
                }
                u_var14 = param_3;
                u_var15 = (param_3 >> 8);
                u_var13 = extraout_var_00;
                // LAB_1038_3e67:
                u_var11 = 0x1030;
                pass1_1030_7ddc(
                    CONCAT22(u_var8, CONCAT11(u_var13, u_var3)),
                    CONCAT13(u_var15, CONCAT12(u_var14, param_2)),
                    param_3_00,
                );
            }
            // goto LAB_1038_3e6c;
        }
        u_var11 = SUB42(&PTR_LOOP_1050_1008, 0);
        pu_var6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 10);
        u_var4 = SUB42(pu_var6, 0);
        pass1_1038_4e78(param_1, pu_var6 & 0xffff | in_DX << 0x10);
        _local_1a = CONCAT22(extraout_DX_04, u_var4);
        ppcVar2 = (*_local_1a + 0x10);
        u_var10 = u_var4;
        ppcVar2(&PTR_LOOP_1050_1008, u_var4, extraout_DX_04);
        _local_16 = CONCAT22(extraout_DX_05, u_var10);
        local_12 = 0;
        while (local_12 < _local_16) {
            u_var11 = 0x1030;
            u_var7 = _local_16;
            pass1_1030_1d7c(u_var4, extraout_DX_04, local_12, (local_12 >> 0x10));
            u_var8 = extraout_DX_06 | u_var7;
            if (u_var8 != 0) {
                u_var14 = param_3;
                u_var15 = (param_3 >> 8);
                u_var3 = pass1_1028_b58e((u_var7 & 0xffff | extraout_DX_06 << 0x10));
                u_var13 = extraout_AH;
                // goto LAB_1038_3e67;
            }
            local_12 = local_12 + 1;
        }
        // LAB_1038_3e6c:
        if (_local_1a == 0x0) {
            return;
        }
        u_var10 = (_local_1a >> 0x10);
        pu_var9 = _local_1a;
    }
    unsafe {
        ppcVar2 = *pu_var9;
    }
    ppcVar2(u_var11, _local_1a, u_var10, 1);
    return;
    // LAB_1038_3e98:
    local_12 = local_12 + 1;
    // goto LAB_1038_3e9c;
}

pub unsafe fn pass1_1038_3efc(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut u8,
    param_2_00: *mut u8,
) {
    let ppcVar1: fn();
    let paVar2: *mut AStruct493;
    let mut in_DX: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    _local_6 = CONCAT22(in_DX, paVar2);
    &paVar2.field_0x1c = (param_1_00 + 4);
    ppcVar1 = (*_local_6 + 0x58);
    (**ppcVar1)(&PTR_LOOP_1050_1028, paVar2, in_DX, param_1_00);
    return;
}

pub unsafe fn pass1_1038_3222(param_1: *mut AStruct848, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    let mut u_var3: i32;
    let pcVar4: *mut libc::c_char;
    let extraout_var: u32;
    let paVar5: *mut AStruct199;
    let mut u_var6: i32;
    let mut extraout_DX: u16;
    let local_BX_33: *mut AStruct1084;
    let mut u_var7: u16;
    let unaff_SS: *mut libc::c_char;
    let paVar8: *mut AStruct848;
    let mut local_26: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8; 20];

    paVar8 = pass1_1030_183c(param_1, 0, 0, 0x4000000, param_3);
    paVar5 = (paVar8 >> 0x10);
    u_var7 = (param_1 >> 0x10);
    local_BX_33 = param_1;
    local_BX_33.field_0x10 = param_2;
    local_BX_33.field_0x14 = 0;
    local_BX_33.field_0x18 = 600;
    local_BX_33.field_0x1a = 600;
    local_BX_33.field_0x1c = 0;
    local_BX_33.field_0x1e = 0;
    local_BX_33.field_0x22 = 0;
    local_BX_33.field_0x24 = 0x32;
    &local_BX_33.field_0x1f6 = 0;
    &local_BX_33.field_0x1fa = 0;
    local_BX_33.field_0x1fe = 0;
    local_BX_33.field_0x200 = 0x8000001;
    local_BX_33.field_0x204 = 0;
    local_BX_33.field_0x206 = 0;
    local_BX_33.field_0x208 = 1;
    local_BX_33.field_0x20a = 0;
    local_BX_33.field_0x20c = 0;
    local_BX_33.field_0x20e = 0;
    local_BX_33.field_0x210 = 0;
    local_BX_33.field_0x214 = 0;
    local_BX_33.field_0x216 = 0;
    local_BX_33.field_0x21a = 0;
    param_1.field_0x0 = 0x6504;
    local_BX_33.field_0x2 = &PTR_LOOP_1050_1038;
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_33.field_0x26), 0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_33.field_0xba), 0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_33.field_0x14e), 0, 0x54);
    u_var2 = pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_33.field_0x1a2), 0, 0x54);
    u_var3 = CONCAT31(extraout_var, u_var2);
    process_struct_1000_179c(0x1b0, paVar5);
    u_var6 = paVar5 | u_var3;
    if (u_var6 == 0) {
        &local_BX_33.field_0x1f6 = 0;
    } else {
        pass1_1030_314c(CONCAT22(paVar5, u_var3), &local_BX_33.field_0x4);
        local_BX_33.field_0x1f6 = u_var3;
        local_BX_33.field_0x1f8 = u_var6;
    }
    u_var1 = &local_BX_33.field_0x4;
    paVar5 = (&local_BX_33.field_0x6 & 0xff);
    string_fn_1000_3f9c(
        local_16,
        unaff_SS,
        s__5lu_1050_5a1a,
        &g_alloc_addr_1050_1050,
        u_var1,
    );
    pcVar4 = local_16;
    pass1_fn_1008_60e8(pcVar4, unaff_SS, u_var1);
    local_BX_33.field_0x1fa = pcVar4;
    local_BX_33.field_0x1fc = paVar5;
    process_struct_1000_179c(0x1e, paVar5);
    if ((paVar5 | pcVar4) == 0) {
        &local_BX_33.field_0xc = 0;
    } else {
        pass1_1020_c444(CONCAT22(paVar5, pcVar4), 100, 200);
        local_BX_33.field_0xc = pcVar4;
        local_BX_33.field_0xe = extraout_DX;
    }
    return;
}

pub unsafe fn pass1_1038_33f8(param_1: *mut AStruct44) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppcVar3: fn();
    let local_BX_5: *mut AStruct850;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1.ptr_a_lo = 0x6504;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    pu_var1 = (local_BX_5 + 1).field_0x0;
    u_var2 = local_BX_5[1].field_0x2;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppcVar3 = *pu_var1;
        }
        (**ppcVar3)();
    }
    pu_var1 = local_BX_5[0x19].field_0x2;
    u_var2 = &local_BX_5[0x19].field_0x4;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppcVar3 = *pu_var1;
        }
        (**ppcVar3)();
    }
    error_check_1000_17ce(&local_BX_5[0x19].field_0x6);
    pu_var1 = &local_BX_5[0x1a].field_0x8;
    u_var2 = &local_BX_5[0x1a].field_0xa;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppcVar3 = *pu_var1;
        }
        (**ppcVar3)(0x1000, pu_var1, u_var2, 1);
    }
    error_check_1000_17ce((&local_BX_5[0x1a].field_0x10 + 2));
    pass1_1030_18b2(param_1);
    return;
}

pub unsafe fn pass1_1038_349e(param_1: *mut AStruct1086, param_2: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut u_var4: i32;
    let mut u_var5: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let local_BX_8: *mut AStruct1086;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let local_16: *mut AStruct1087;
    let mut local_14: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    local_BX_8 = param_1;
    local_BX_8.field_0x200 = param_2;
    pass1_1038_4d0e(param_1, 600);
    u_var3 = param_2;
    pass1_1038_4d0e(param_1, 600);
    local_BX_8.field_0x204 = 0;
    local_BX_8.field_0x206 = 0;
    u_var1 = local_BX_8.field_0xc;
    u_var7 = u_var1;
    u_var8 = (u_var1 >> 0x10);
    ppcVar2 = (local_BX_8.field_0xc + 0x10);
    ppcVar2();
    _local_6 = CONCAT22(extraout_DX, u_var3);
    local_a = 0;
    while (local_a < _local_6) {
        u_var5 = _local_6;
        pass1_1030_1d7c(local_BX_8.field_0xc, local_a, (local_a >> 0x10));
        u_var4 = u_var5;
        if ((extraout_DX_00 | u_var4) != 0) {
            ppcVar2 = ((u_var5 & 0xffff | extraout_DX_00 << 0x10) + 0x58);
            ppcVar2(
                0x1030,
                u_var4,
                extraout_DX_00,
                param_1,
                u_var6,
                u_var7,
                u_var8,
            );
            (u_var4 + 0x1c) = 0;
        }
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn pass1_1038_354a(param_1: *mut AStruct1088) {
    let in_AX: *mut AStruct493;
    let in_DX: *mut AStruct199;
    let mut u_var1: i32;
    let local_BX_4: *mut AStruct1088;
    let mut u_var2: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x21a == 0) {
        process_struct_1000_179c(10, in_DX);
        u_var1 = in_DX | in_AX;
        if (u_var1 == 0) {
            &local_BX_4.field_0x21a = 0;
        } else {
            pass1_1030_9ecc(CONCAT22(in_DX, in_AX), param_1);
            local_BX_4.field_0x21a = in_AX;
            &local_BX_4.field_0x21c = u_var1;
        }
    }
    pass1_1030_9ef2(&local_BX_4.field_0x21a);
    return;
}

pub unsafe fn pass1_1038_35a8(param_1: *mut AStruct1089, param_2: u16) {
    let mut in_AX: i32;
    let in_DX: *mut AStruct199;
    let mut u_var1: i32;
    let local_BX_4: *mut AStruct1089;
    let mut u_var2: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x21a == 0) {
        process_struct_1000_179c(10, in_DX);
        u_var1 = in_DX | in_AX;
        if (u_var1 == 0) {
            &local_BX_4.field_0x21a = 0;
        } else {
            pass1_1030_9ecc(CONCAT22(in_DX, in_AX), param_1);
            local_BX_4.field_0x21a = in_AX;
            &local_BX_4.field_0x21c = u_var1;
        }
    }
    pass1_1030_9f40(&local_BX_4.field_0x21a, param_2);
    return;
}

pub unsafe fn pass1_1038_3608(param_1: *mut u8) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    error_check_1000_17ce((param_1 + 0x21a));
    (param_1 + 0x21a) = 0;
    return;
}

pub unsafe fn pass1_1038_2c82(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let pu_var1: *mut u32;
    let piVar2: *mut i32;
    let mut u_var3: i32;
    let mut u_var4: u32;
    let ppcVar5: fn();
    let mut u_var6: i32;
    let paVar7: *mut AStruct493;
    let ppVar8: *mut pass1_struct_2;
    let mut iVar9: i32;
    let pu_var10: *mut u32;
    let mut extraout_DX: u16;
    let local_BX_4: *mut AStruct1079;
    let mut iVar11: i32;
    let local_BX_35: *mut AStruct1080;
    let mut iVar12: i32;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut u_var15: u16;
    let mut u_var16: u16;
    let mut unaff_SS: u16;
    let ppVar17: *mut pass1_struct_1;
    let u_var18: u8;
    let mut in_stack_0000ffce: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    u_var13 = (param_5 >> 0x10);
    local_BX_4 = param_5;
    local_6 = local_BX_4.field_0x200;
    u_var14 = (param_2_00 >> 0x10);
    iVar11 = param_2_00;
    local_a = (iVar11 + 0x200);
    u_var16 = (iVar11 + 0x202);
    u_var15 = (param_1_00 >> 0x10);
    local_BX_35 = param_1_00;
    iVar9 = local_BX_35.field_0xc;
    if (iVar9 == 1) {
        _local_e = param_1_00;
        pass1_1038_52b8(param_5, &local_BX_35.field_0x8, &local_BX_35.field_0xe);
        return;
    }
    if (iVar9 == 2) {
        _local_e = param_1_00;
        if (&local_BX_35.field_0xe != 0) {
            pass1_1038_3efc(local_BX_4, u_var13, param_2_00, *&local_BX_35.field_0xe);
            return;
        }
        pass1_1020_a43e(CONCAT22(unaff_SS, &local_12));
        local_16 = (_local_e + 8);
        while (local_16 = local_16 - 1, (local_16._2_2_ | local_16) != 0) {
            pass1_1020_a6ee(
                CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_12)),
                (_local_e + 0x12),
            );
        }
    } else {
        if (iVar9 == 3) {
            pass1_1038_3f38(param_5, param_2_00, &local_BX_35.field_0xe);
            return;
        }
        local_6._2_2_ = (local_6 >> 0x10);
        if (iVar9 == 4) {
            g_u16_ptr_1050_5f2e = (local_6._2_2_ & 0xff);
            if ((local_6 == (&PTR_LOOP_1050_0000 + 1)) && ((local_6 & 0xff0000) == 0)) {
                local_12 = local_BX_4.field_0x1f6;
                u_var4 = &local_BX_35.field_0x8;
                pass1_1030_3694(
                    local_12,
                    (local_12 >> 0x10),
                    &local_BX_35.field_0xe,
                    u_var4,
                    (u_var4 >> 0x10),
                );
                local_BX_35.field_0x10 = local_12;
                local_BX_35.field_0x12 = extraout_DX;
            } else {
                if (__g_AStruct94_ptr_1 == 0) {
                    _g_AStruct94_ptr_1 = local_6;
                    struct_fn_1000_160a();
                } else {
                }
                alloc_mem_1000_1708(0x6c, 0, 1, _g_AStruct94_ptr_1, g_u16_ptr_1050_5f2e);
                local_BX_35.field_0x10 = _g_AStruct94_ptr_1;
                local_BX_35.field_0x12 = g_u16_ptr_1050_5f2e;
                iVar9 = &local_BX_35.field_0xe;
                if (iVar9 != 3) {
                    if (iVar9 != 4) {
                        u_var4 = &local_BX_35.field_0x10;
                        (u_var4 + 0x28) = &local_BX_35.field_0x8;
                        return;
                    }
                    u_var4 = &local_BX_35.field_0x10;
                    (u_var4 + 0xdc) = &local_BX_35.field_0x8;
                    return;
                }
                u_var4 = &local_BX_35.field_0x10;
                (u_var4 + 100) = &local_BX_35.field_0x8;
            }
        } else {
            if (iVar9 == 5) {
                if (&local_BX_35.field_0xe == 0xc) {
                    if ((local_6 == (&PTR_LOOP_1050_0000 + 1)) && ((local_6 & 0xff0000) == 0)) {
                        u_var4 = local_BX_4.field_0x1f6;
                        iVar9 = local_BX_35.field_0x8;
                        iVar11 = local_BX_35.field_0xa;
                        u_var6 = -iVar9;
                        u_var16 = (u_var4 >> 0x10);
                        iVar12 = u_var4;
                        pu_var1 = (iVar12 + 0x170);
                        unsafe {
                            u_var3 = *pu_var1;
                            *pu_var1 = *pu_var1 + u_var6;
                            piVar2 = (iVar12 + 0x172);
                            *piVar2 = (*piVar2 - (iVar11 + (iVar9 != 0))) + CARRY2(u_var3, u_var6);
                        }
                    }
                } else {
                    u_var16 = local_BX_35.field_0x8;
                    pass1_1038_43cc(
                        local_BX_4,
                        CONCAT13((u_var16 >> 8), CONCAT12(u_var16, u_var13)),
                        &local_BX_35.field_0xe,
                    );
                }
            } else {
                if (iVar9 == 7) {
                    u_var4 = &local_BX_35.field_0xe;
                    paVar7 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var4, (u_var4 >> 0x10));
                    pass1_1038_349e(CONCAT22(u_var16, paVar7), (iVar11 + 0x200));
                    u_var18 = (u_var16 >> 8);
                    pass1_1038_4d0e(CONCAT13(u_var18, CONCAT12(u_var16, paVar7)), 600);
                    pass1_1038_4d0e(CONCAT13(u_var18, CONCAT12(u_var16, paVar7)), 600);
                    ppVar17 = process_struct_1010_20ba(
                        _g_AStruct372_1050_0ed0,
                        CONCAT22(in_stack_0000ffce, 0x3b),
                    );
                    pass1_1008_de58(ppVar17, &local_BX_35.field_0xe, 0x4000001);
                    ppVar17 = process_struct_1010_20ba(
                        _g_AStruct372_1050_0ed0,
                        CONCAT22(in_stack_0000ffce, 0x2f),
                    );
                    u_var16 = (ppVar17 >> 0x10);
                    ppVar8 = pass1_1030_8344(
                        _g_bool_1050_5748,
                        (_g_bool_1050_5748 >> 0x10),
                        (ppVar17 + 0x20),
                    );
                    local_12 = CONCAT22(u_var16, ppVar8);
                    iVar9 = pass1_1030_5b00(CONCAT22(u_var16, ppVar8));
                    _local_e = process_struct_1010_20ba(
                        _g_AStruct372_1050_0ed0,
                        CONCAT22(in_stack_0000ffce, iVar9),
                    );
                    pu_var10 = (_local_e + 0x20);
                    unsafe {
                        ppcVar5 = (*pu_var10 + 4);
                    }
                    (**ppcVar5)(0x1010, pu_var10, (_local_e >> 0x10), 6);
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1038_2f92(param_1: u16, param_2: u16, param_3: u16, param_2_00: u32) {
    let pu_var1: *mut u32;
    AStruct1082 * *ppaVar2;
    let mut iVar3: i32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut u_var6: u32;
    let mut u_var7: u32;
    let local_BX_189: *mut AStruct1081;
    let mut u_var8: u16;
    let mut in_stack_0000000e: u16;
    let mut local_1a: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let temp_5f8f06f4df: *mut AStruct1082;

    u_var6 = (param_2_00._2_2_ + 0x200);
    iVar3 = (param_1_00 + 0xc);
    if (iVar3 == 1) {
        u_var7 = (param_1_00 + 8);
        pass1_1038_3cc0(
            CONCAT22(in_stack_0000000e, param_2_00._2_2_),
            u_var7,
            (u_var7 >> 0x10),
            (param_1_00 + 0xe),
        );
        return;
    }
    if (iVar3 == 4) {
        pass1_1030_355c((param_2_00._2_2_ + 0x1f6), (param_1_00 + 0x10));
        return;
    }
    if (iVar3 == 5) {
        if ((param_1_00 + 0xe) != 0xc) {
            pass1_1038_5798(
                CONCAT22(in_stack_0000000e, param_2_00._2_2_),
                (param_1_00 + 8),
                (param_1_00 + 0xe),
            );
            return;
        }
        local_a._0_2_ = u_var6;
        if ((local_a == 1) && ((u_var6 & 0xff0000) == 0)) {
            u_var7 = (param_2_00._2_2_ + 0x1f6);
            u_var4 = (param_1_00 + 8);
            temp_5f8f06f4df = (param_1_00 + 10);
            u_var8 = (u_var7 >> 0x10);
            local_BX_189 = u_var7;
            pu_var1 = &local_BX_189.field_0x170;
            u_var5 = *pu_var1;
            *pu_var1 = *pu_var1 + u_var4;
            ppaVar2 = &local_BX_189.field_0x172;
            *ppaVar2 = temp_5f8f06f4df + (*ppaVar2 + CARRY2(u_var5, u_var4));
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1038_3074(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1038_2a5c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_30aa(param_1: *mut AStruct393) {
    let u_var1: u8;
    let mut u_var2: i32;
    let extraout_var: u32;
    let in_DX: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let local_BX_19: *mut AStruct1083;
    let mut u_var4: u16;
    let mut local_4: u16;

    pass1_1030_17ce(param_1, 0, 0);
    u_var4 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    local_BX_19.field_0x10 = 0;
    local_BX_19.field_0x14 = 0;
    local_BX_19.field_0x18 = 600;
    local_BX_19.field_0x1a = 600;
    local_BX_19.field_0x1c = 0;
    local_BX_19.field_0x1e = 0;
    local_BX_19.field_0x22 = 0;
    local_BX_19.field_0x24 = 0x32;
    &local_BX_19.field_0x1f6 = 0;
    local_BX_19.field_0x1fa = 0;
    local_BX_19.field_0x1fe = 0;
    local_BX_19.field_0x200 = 0x8000001;
    local_BX_19.field_0x204 = 0;
    local_BX_19.field_0x206 = 0;
    local_BX_19.field_0x208 = 1;
    local_BX_19.field_0x20a = 0;
    local_BX_19.field_0x20c = 0;
    local_BX_19.field_0x20e = 0;
    local_BX_19.field_0x210 = 0;
    local_BX_19.field_0x214 = 0;
    local_BX_19.field_0x216 = 0;
    local_BX_19.field_0x21a = 0;
    param_1.field_0x0 = 0x6504;
    local_BX_19.field_0x2 = &PTR_LOOP_1050_1038;
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_19.field_0x26), 0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_19.field_0xba), 0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_19.field_0x14e), 0, 0x54);
    u_var1 = pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_19.field_0x1a2), 0, 0x54);
    u_var2 = CONCAT31(extraout_var, u_var1);
    process_struct_1000_179c(0x1b0, in_DX);
    struct_a = (in_DX | u_var2);
    if (struct_a == 0x0) {
        &local_BX_19.field_0x1f6 = 0;
    } else {
        pass1_1030_314c(CONCAT22(in_DX, u_var2), local_BX_19.field_0x4);
        local_BX_19.field_0x1f6 = u_var2;
        local_BX_19.field_0x1f8 = struct_a;
    }
    process_struct_1000_179c(0x1e, struct_a);
    if ((struct_a | u_var2) == 0) {
        u_var2 = 0;
        u_var3 = 0;
    } else {
        pass1_1020_c444(CONCAT22(struct_a, u_var2), 100, 200);
        u_var3 = extraout_DX;
    }
    local_BX_19.field_0xc = u_var2;
    local_BX_19.field_0xe = u_var3;
    return;
}

pub unsafe fn pass1_1038_29d2(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_2a0e(
    param_1: *mut AStruct500,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) -> i32 {
    let local_BX_19: *mut AStruct500;
    let mut local_ES_19: u16;

    pass1_1028_d1dc(param_1, s_fem133_wav_1050_2af7);
    local_ES_19 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    local_BX_19.field_0x108 = param_5;
    local_BX_19.field_0x10c = param_4;
    local_BX_19.field_0x110 = param_3;
    local_BX_19.field_0x114 = param_2;
    param_1.a = 0x309a;
    local_BX_19.b = &PTR_LOOP_1050_1038;
    return;
}

pub unsafe fn pass1_1038_2a5c(param_1: *mut u16) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppcVar3: fn();
    let local_BX_5: *mut AStruct1076;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    *param_1 = 0x309a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    pu_var1 = local_BX_5.field_0x114;
    u_var2 = local_BX_5.field_0x116;
    if ((u_var2 | pu_var1) != 0) {
        ppcVar3 = *pu_var1;
        (**ppcVar3)();
    }
    pu_var1 = local_BX_5.field_0x110;
    u_var2 = local_BX_5.field_0x112;
    if ((u_var2 | pu_var1) != 0) {
        ppcVar3 = *pu_var1;
        (**ppcVar3)();
    }
    *param_1 = s_1_1050_389a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1038_2ac2(param_1: *mut AStruct1077) {
    let mut u_var1: u32;
    let paVar2: *mut AStruct493;
    let mut in_DX: u16;
    let local_BX_4: *mut AStruct1077;
    let mut uvar3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    u_var1 = local_BX_4.field_0x108;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    _local_6 = CONCAT22(in_DX, paVar2);
    u_var1 = local_BX_4.field_0x10c;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    _local_a = CONCAT22(in_DX, paVar2);
    pass1_1038_2c82(
        local_BX_4,
        u_var3,
        local_BX_4.field_0x110,
        CONCAT22(in_DX, paVar2),
        _local_6,
    );
    pass1_1038_2c82(
        local_BX_4,
        u_var3,
        local_BX_4.field_0x114,
        _local_6,
        _local_a,
    );
    return 1;
}

pub unsafe fn pass1_1038_2b2e(param_1: *mut AStruct1078) {
    let mut u_var1: u32;
    let paVar2: *mut AStruct493;
    let paVar3: *mut AStruct493;
    let local_BX_4: *mut AStruct1078;
    let local_ES_4: *mut AStruct1078;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    u_var1 = local_BX_4.field_0x108;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    u_var1 = local_BX_4.field_0x10c;
    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    u_var1 = local_BX_4.field_0x110;
    pass1_1038_2f92(
        local_BX_4,
        local_ES_4,
        u_var1,
        CONCAT22(paVar3, (u_var1 >> 0x10)),
    );
    u_var1 = local_BX_4.field_0x114;
    pass1_1038_2f92(
        local_BX_4,
        local_ES_4,
        u_var1,
        CONCAT22(paVar2, (u_var1 >> 0x10)),
    );
    return 1;
}

pub unsafe fn pass1_1038_0d8e(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: *mut AStruct44,
) {
    let mut u_var1: u16;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1030_d0a8(param_2_00);
    u_var2 = pass1_1030_d144(param_2_00);
    _local_a = u_var2;
    local_4 = u_var1;
    if ((u_var2 >> 0xf | u_var2) != 0) {
        while {
            u_var3 = pass1_1028_6744(param_1_00, local_4);
            if (u_var3 != 0) {
                pass1_1028_6228(param_1_00, 1, 0, local_4);
                _local_a = _local_a + -1;
                pass1_1030_d180(param_2_00, local_4);
            }
            if (_local_a == 0) {
                return;
            }
            local_4 = pass1_1030_d0a8(param_2_00);
            local_4 != u_var1
        } {}
    }
    return;
}

pub unsafe fn pass1_1038_0e00(param_1: u32, param_2: u32, param_3: u32) {
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut AStruct493;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut u_var3: i32;
    let mut u_var4: u32;
    let mut local_14: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppcVar1 = (param_2 + 0x10);
    (**ppcVar1)();
    _local_6 = CONCAT22(extraout_DX, in_AX);
    local_a = 0;
    while (local_a < _local_6) {
        ppcVar1 = (param_2 + 4);
        u_var4 = _local_6;
        (**ppcVar1)();
        u_var3 = extraout_DX_00 | u_var4;
        if (u_var3 != 0) {
            paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var4, extraout_DX_00);
            u_var4 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
            u_var3 = (u_var4 >> 0x10);
            if ((u_var3 | u_var4) != 0) {
                pass1_1038_0d8e(
                    param_1,
                    (param_1 >> 0x10),
                    u_var4 & 0xffff | u_var3 << 0x10,
                    param_3,
                );
            }
        }
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn pass1_1038_0e78(param_1: u32, param_2: u32) {
    let ppcVar1: fn();
    let mut u_var2: u32;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let paVar6: *mut AStruct493;
    let pu_var7: *mut u8;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut u_var8: i32;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: i32;
    let mut u_var9: u16;
    let mut u_var10: u32;
    let mut local_28: u16;
    let mut local_20: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let local_e: *mut AStruct1058;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    pu_var7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 4);
    u_var3 = pu_var7;
    pass1_1038_4d6e(param_2, pu_var7 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, u_var3);
    u_var2 = *_local_a;
    ppcVar1 = u_var2 + 8;
    u_var8 = u_var3;
    (**ppcVar1)(&PTR_LOOP_1050_1008, u_var3, extraout_DX);
    u_var8 = extraout_DX_00 | u_var8;
    if (u_var8 == 0) {
        if (_local_a != 0x0) {
            ppcVar1 = u_var2;
            (**ppcVar1)(8, u_var3, extraout_DX, 1);
            return;
        }
    } else {
        u_var9 = SUB42(&PTR_LOOP_1050_1008, 0);
        pu_var7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        u_var4 = SUB42(pu_var7, 0);
        pass1_1038_4d6e(param_2, pu_var7 & 0xffff | u_var8 << 0x10);
        _local_e = CONCAT22(extraout_DX_01, u_var4);
        ppcVar1 = (*_local_e + 0x10);
        u_var5 = u_var4;
        (**ppcVar1)(&PTR_LOOP_1050_1008, u_var4, extraout_DX_01);
        _local_12 = CONCAT22(extraout_DX_02, u_var5);
        local_16 = 0;
        while (local_16 < _local_12) {
            ppcVar1 = (*_local_e + 4);
            u_var10 = _local_12;
            (**ppcVar1)();
            u_var8 = extraout_DX_03 | u_var10;
            if (u_var8 != 0) {
                paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var10, extraout_DX_03);
                u_var9 = 0x1030;
                u_var10 = pass1_1030_73a8(CONCAT22(u_var8, paVar6));
                if (((u_var10 >> 0x10) | u_var10) != 0) {
                    pass1_1038_0e00(param_1, _local_a, u_var10);
                }
            }
            local_16 = local_16 + 1;
        }
        if (_local_a != 0x0) {
            ppcVar1 = *_local_a;
            (**ppcVar1)(u_var9, u_var3, extraout_DX, 1);
        }
        if (_local_e != 0x0) {
            ppcVar1 = *_local_e;
            (**ppcVar1)(u_var9, u_var4, extraout_DX_01, 1);
        }
    }
    return;
}

pub unsafe fn pass1_1038_0f8c(param_1: u16, param_2: u16, param_1_00: *mut u32, param_2_00: u32) {
    let pu_var1: *mut u16;
    let mut u_var2: u16;
    let mut uvar3: u16;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let ppcVar6: fn();
    let mut u_var7: u32;
    let Var8: u16;
    let mut in_AX: u16;
    let local_AX_95: *mut AStruct1059;
    let pu_var9: *mut u8;
    let local_AX_405: *mut AStruct1060;
    let mut u_var10: u32;
    let mut u_var12: i32;
    let mut u_var13: i32;
    let mut in_EDX: u32;
    let local_BX_491: *mut AStruct1061;
    let mut u_var14: u16;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let pu_var15: *mut u32;
    let mut local_60: u32;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_50: u32;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: [u8; 4];
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: [u8; 4];
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let pu_var11: *mut u8;

    local_6 = 100;
    local_8 = 0;
    ppcVar6 = (*param_1_00 + 0x10);
    pu_var15 = param_1_00;
    (**ppcVar6)();
    _local_c = CONCAT22(in_EDX, in_AX);
    local_10 = 0;
    loop {
        if (_local_c <= local_10) {
            return;
        }
        ppcVar6 = (*param_1_00 + 4);
        u_var10 = _local_c;
        u_var13 = in_EDX;
        (**ppcVar6)(unaff_CS, param_1_00, local_10, (local_10 >> 0x10), pu_var15);
        local_12 = u_var13;
        local_14 = u_var10;
        u_var13 = local_12 | local_14;
        in_EDX = u_var13;
        if (u_var13 != 0) {
            local_18 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_14, local_12);
            local_16 = u_var13;
            unaff_CS = 0x1030;
            local_1c = pass1_1030_73a8(CONCAT22(local_16, local_18));
            in_EDX = local_1c >> 0x10;
            pu_var9 = local_20;
            ppcVar6 = (local_1c + 0x40);
            (**ppcVar6)(0x1030, local_1c, (local_1c >> 0x10), pu_var9);
            if (pu_var9 == 0x0) {
                _local_24 = pass1_1028_62c8(local_1c);
                u_var10 = _local_24 >> 0x10;
                local_8 = 1;
                local_28 = (param_2_00 + 0x22);
                pass1_1008_5784(CONCAT22(unaff_SS, local_30), local_28);
                loop {
                    u_var13 = u_var10;
                    local_AX_95 = local_30;
                    unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                    pass1_1008_5b12(CONCAT22(unaff_SS, local_AX_95));
                    in_EDX = (u_var13 | local_AX_95);
                    if ((u_var13 | local_AX_95) == 0) {
                        break;
                    }
                    u_var2 = local_AX_95.field_0x4;
                    u_var3 = local_AX_95.field_0x6;
                    u_var4 = local_AX_95.field_0x8;
                    u_var5 = local_AX_95.field_0xa;
                    u_var7 = local_AX_95.field_0xc / u_var5;
                    u_var10 = _local_24;
                    if (local_6 < _local_24) {
                        u_var10 = local_6 & 0xffff;
                        local_22 = local_6._2_2_;
                    }
                    u_var12 = local_22 | u_var10;
                    in_EDX = u_var12;
                    if (u_var12 == 0) {
                        break;
                    }
                    qVar8 = (u_var10 & 0xffff | local_22 << 0x10) / u_var7;
                    in_EDX = qVar8 >> 0x10;
                    u_var12 = (qVar8 >> 0x10);
                    local_4c = qVar8;
                    if (local_4c == 0) {
                        break;
                    }
                    if (local_4c < u_var5) {
                        pu_var1 = &local_AX_95.field_0xc;
                        *pu_var1 = *pu_var1 - u_var10;
                        pu_var1 = &local_AX_95.field_0xa;
                        *pu_var1 = *pu_var1 - local_4c;
                    } else {
                        ppcVar6 = (local_28 + 0xc);
                        (**ppcVar6)(
                            &PTR_LOOP_1050_1008,
                            local_28,
                            (local_28 >> 0x10),
                            local_AX_95,
                            u_var13,
                        );
                        local_2c = 0;
                        local_4c = u_var5;
                    }
                    pu_var11 = _PTR_LOOP_1050_68a2;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                    u_var13 = pu_var11;
                    local_50 = pu_var11 & 0xffff | u_var12 << 0x10;
                    if ((u_var12 | u_var13) == 0) {
                        local_50 = 0;
                    } else {
                        local_50 = s_1_1050_389a;
                        (u_var13 + 2) = &PTR_LOOP_1050_1008;
                        (u_var13 + 4) = 0;
                        (u_var13 + 6) = 0;
                        (u_var13 + 8) = 0;
                        (u_var13 + 10) = 0;
                        (u_var13 + 0xc) = 0;
                        local_50 = 0x56ce;
                        (u_var13 + 2) = 0x1018;
                    }
                    u_var14 = (local_50 >> 0x10);
                    local_BX_491 = local_50;
                    local_BX_491.field_0xa = local_4c;
                    u_var7 = local_4c * u_var7;
                    u_var10 = u_var7 >> 0x10;
                    local_BX_491.field_0xc = u_var7;
                    local_BX_491.field_0x4 = u_var2;
                    local_BX_491.field_0x6 = u_var3;
                    local_BX_491.field_0x8 = u_var4;
                    pass1_1028_6408(local_1c, local_50);
                }
            } else {
                ppcVar6 = (*param_1_00 + 8);
                (**ppcVar6)(0x1030, param_1_00, 0, 0, local_10, (local_10 >> 0x10));
            }
        }
        local_10 = local_10 + 1;
    }
}

pub unsafe fn pass1_1038_11b0(param_1: u32, param_2: u32, param_3: u32) {
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut AStruct493;
    let mut iVar3: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppcVar1 = (param_3 + 0x10);
    (**ppcVar1)();
    _local_6 = CONCAT22(extraout_DX, in_AX);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        ppcVar1 = (param_3 + 4);
        u_var5 = _local_6;
        (**ppcVar1)();
        u_var4 = extraout_DX_00;
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var5, extraout_DX_00);
        u_var5 = pass1_1030_73a8(CONCAT22(u_var4, paVar2));
        iVar3 = u_var5;
        pass1_1038_0f8c(param_1, (param_1 >> 0x10), param_2, u_var5);
        if (iVar3 == 0) {
            break;
        }
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn pass1_1038_1220(param_1: u32, param_2: u32) {
    let ppcVar1: fn();
    let mut u_var2: u32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let pu_var6: *mut u8;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut u_var7: i32;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: i32;
    let mut extraout_DX_05: u16;
    let mut extraout_DX_06: i32;
    let u_var8: u8;
    let mut local_e: u16;
    let mut local_c: u16;
    let local_a: *mut AStruct1062;
    let mut local_8: u16;

    pu_var6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 4);
    iVar3 = pu_var6;
    pass1_1038_4d6e(param_2, pu_var6 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, iVar3);
    ppcVar1 = (*_local_a + 0x10);
    iVar4 = iVar3;
    (**ppcVar1)(&PTR_LOOP_1050_1008, iVar3, extraout_DX);
    if ((extraout_DX_00 != 0) || (iVar4 != 0)) {
        u_var7 = extraout_DX_00;
        pu_var6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 5);
        iVar5 = pu_var6;
        pass1_1038_4d6e(param_2, pu_var6 & 0xffff | u_var7 << 0x10);
        _local_e = CONCAT22(extraout_DX_01, iVar5);
        u_var8 = iVar5;
        u_var2 = *_local_e;
        ppcVar1 = u_var2 + 8;
        iVar4 = iVar5;
        (**ppcVar1)(&PTR_LOOP_1050_1008, u_var8, extraout_DX_01);
        if (((extraout_DX_02 != 0) || (u_var7 = extraout_DX_02, iVar4 != 0))
            && (
                pass1_1038_11b0(
                    param_1,
                    CONCAT13((extraout_DX >> 8), CONCAT12(extraout_DX, iVar3)),
                    CONCAT22(extraout_DX_01, iVar5),
                ),
                u_var7 = extraout_DX_03,
                iVar4 == 0,
            ))
        {
            if (_local_e == 0x0) {
                return;
            }
            ppcVar1 = u_var2;
            (**ppcVar1)(8, u_var8, extraout_DX_01, 1);
            return;
        }
        if (_local_e != 0x0) {
            ppcVar1 = *_local_e;
            (**ppcVar1)(8, u_var8, extraout_DX_01, 1);
            u_var7 = extraout_DX_04;
        }
        pu_var6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 6);
        iVar5 = pu_var6;
        pass1_1038_4d6e(param_2, pu_var6 & 0xffff | u_var7 << 0x10);
        _local_e = CONCAT22(extraout_DX_05, iVar5);
        ppcVar1 = (*_local_e + 0x10);
        iVar4 = iVar5;
        (**ppcVar1)(8, iVar5, extraout_DX_05);
        if ((extraout_DX_06 != 0) || (iVar4 != 0)) {
            pass1_1038_11b0(
                param_1,
                CONCAT22(extraout_DX, iVar3),
                CONCAT22(extraout_DX_05, iVar5),
            );
        }
        if (_local_e != 0x0) {
            ppcVar1 = *_local_e;
            (**ppcVar1)(8, iVar5, extraout_DX_05, 1);
        }
    }
    if (_local_a != 0x0) {
        ppcVar1 = *_local_a;
        (**ppcVar1)(8, iVar3, extraout_DX, 1);
    }
    return;
}

pub unsafe fn pass1_1038_134a(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut u32,
    param_2_00: *mut u32,
    param_5: *mut u32,
) -> i32 {
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut AStruct493;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uvar3: u16;
    let mut u_var4: i32;
    let mut unaff_CS: u16;
    let mut u_var5: u32;
    let mut u_var6: u32;
    let pu_var7: *mut u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppcVar1 = (*param_5 + 0x10);
    pu_var7 = param_5;
    (**ppcVar1)();
    _local_6 = CONCAT22(extraout_DX, in_AX);
    *param_1_00 = 0;
    while {
        if (_local_6 <= *param_2_00) {
            return;
        }
        u_var5 = *param_2_00;
        *param_2_00 = *param_2_00 + 1;
        ppcVar1 = (*param_5 + 4);
        (**ppcVar1)(unaff_CS, param_5, u_var5, pu_var7);
        u_var3 = extraout_DX_00;
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var5, extraout_DX_00);
        u_var5 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
        unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
        u_var6 = pass1_1028_45e2(u_var5);
        u_var4 = (u_var6 >> 0x10);
        param_1_00 = u_var6;
        (param_1_00 + 2) = u_var4;
        (u_var4 | param_1_00) == 0
    } {}
    return;
}

pub unsafe fn pass1_1038_13da(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut u32,
    param_2_00: *mut u32,
    param_5: *mut u32,
) -> i32 {
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut AStruct493;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uvar3: u16;
    let mut u_var4: i32;
    let mut unaff_CS: u16;
    let mut u_var5: u32;
    let pu_var6: *mut u8;
    let pu_var7: *mut u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppcVar1 = (*param_5 + 0x10);
    pu_var7 = param_5;
    (**ppcVar1)();
    _local_6 = CONCAT22(extraout_DX, in_AX);
    *param_1_00 = 0;
    while {
        if (_local_6 <= *param_2_00) {
            return;
        }
        u_var5 = *param_2_00;
        *param_2_00 = *param_2_00 + 1;
        ppcVar1 = (*param_5 + 4);
        (**ppcVar1)(unaff_CS, param_5, u_var5, pu_var7);
        u_var3 = extraout_DX_00;
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var5, extraout_DX_00);
        if ((u_var3 | paVar2) == 0) {
            return;
        }
        u_var5 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
        u_var4 = (u_var5 >> 0x10);
        if ((u_var4 | u_var5) == 0) {
            return;
        }
        unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
        pu_var6 = pass1_1028_3c32((u_var5 & 0xffff | u_var4 << 0x10));
        u_var4 = (pu_var6 >> 0x10);
        param_1_00 = pu_var6;
        (param_1_00 + 2) = u_var4;
        (u_var4 | param_1_00) == 0
    } {}
    return;
}

pub unsafe fn pass1_1038_1482(param_1: u32, param_2: u32, param_3: u32) {
    let ppcVar1: fn();
    let Var2: u16;
    let mut uvar3: u16;
    let u_var4: u8;
    let pu_var5: *mut u32;
    let mut u_var6: i32;
    let mut u_var7: u16;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut u_var8: u32;
    let mut in_DX: u16;
    let paVar9: *mut AStruct199;
    let paVar10: *mut AStruct199;
    let mut u_var11: i32;
    let mut unaff_SS: u16;
    let mut u_var12: u16;
    let u_var13: u8;
    let u_var14: u8;
    let mut u_var15: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u32;
    let mut local_42: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = 0;
    local_a = 0;
    pu_var5 = &local_a;
    u_var12 = (param_1 >> 0x10);
    u_var3 = param_1;
    pass1_1038_134a(
        u_var3,
        u_var12,
        CONCAT22(unaff_SS, pu_var5),
        CONCAT22(unaff_SS, &local_6),
        param_3,
    );
    _local_e = CONCAT22(in_DX, pu_var5);
    ppcVar1 = (param_2 + 0x10);
    (**ppcVar1)();
    _local_12 = CONCAT22(in_DX, pu_var5);
    local_16 = 0;
    loop {
        if (_local_12 <= local_16) {
            return;
        }
        if ((local_c | local_e) == 0) {
            return;
        }
        u_var4 = pass1_1028_b58e(_local_e);
        u_var11 = CONCAT31(extraout_var_00, u_var4);
        local_1a = u_var11;
        local_18 = local_10;
        pass1_1038_1a30(
            u_var3,
            u_var12,
            CONCAT31(extraout_var_00, u_var4) & 0xffff | local_10 << 0x10,
        );
        local_1e = u_var11;
        local_1c = local_10;
        if ((local_10 | u_var11) != 0) {
            sVar2 = CONCAT22(local_10, u_var11) * 100;
            u_var7 = (sVar2 >> 0x20);
            u_var8 = sVar2 >> 1;
            ppcVar1 = (param_2 + 4);
            local_22 = u_var8;
            (**ppcVar1)(&PTR_LOOP_1050_1028, param_2, local_16, (local_16 >> 0x10));
            local_26 = u_var8;
            local_24 = u_var7;
            local_2a = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_26, u_var7);
            local_28 = u_var7;
            _local_2e = pass1_1030_73a8(CONCAT22(u_var7, local_2a));
            local_32 = (_local_2e + 0x28);
            local_36 = 0;
            local_38 = (local_32 + 4);
            local_3a = 0;
            while (local_3a < local_38) {
                pass1_1020_bb16(
                    local_32,
                    CONCAT22(unaff_SS, &local_46),
                    CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_42)),
                    local_3a,
                );
                if (((local_46 != 0) && (0xd < local_42)) && (local_42 < 0x1d)) {
                    u_var8 = local_46;
                    if (local_22 < local_46) {
                        u_var8 = local_22 & 0xffff;
                        local_46._2_2_ = local_22._2_2_;
                    }
                    u_var11 = u_var8;
                    if ((local_a._2_2_ <= local_46._2_2_)
                        && (local_a._2_2_ < local_46._2_2_ || (local_a < u_var11)))
                    {
                        u_var11 = local_a;
                        local_46._2_2_ = local_a._2_2_;
                    }
                    _local_4a = CONCAT22(local_46._2_2_, u_var11);
                    local_22 = CONCAT22(
                        local_22._2_2_ + (-(local_22 < u_var11) - local_46._2_2_),
                        local_22 - u_var11,
                    );
                    local_a = CONCAT22(
                        local_a._2_2_ + (-(local_a < u_var11) - local_46._2_2_),
                        local_a - u_var11,
                    );
                    paVar10 = local_46._2_2_;
                    if (local_36 == 0) {
                        paVar9 = local_46._2_2_;
                        u_var6 = u_var11;
                        process_struct_1000_179c(10, local_46._2_2_);
                        paVar10 = (paVar9 | u_var6);
                        if (paVar10 == 0x0) {
                            u_var6 = 0;
                            paVar10 = 0x0;
                        } else {
                            pass1_1020_ba3e(CONCAT22(paVar9, u_var6), 5, 5);
                        }
                        local_36 = CONCAT22(paVar10, u_var6);
                    }
                    pass1_1020_bb8a(
                        local_36,
                        (local_36 >> 0x10),
                        u_var11,
                        local_46._2_2_,
                        local_42,
                    );
                    pass1_1020_bb8a(
                        local_32,
                        (local_32 >> 0x10),
                        (local_46 - _local_4a),
                        (local_46 - _local_4a >> 0x10),
                        local_42,
                    );
                    if (local_a == 0) {
                        pass1_1038_1b3a(u_var3, u_var12, _local_e, local_36);
                        local_36 = 0;
                        pu_var5 = &local_a;
                        pass1_1038_134a(
                            u_var3,
                            u_var12,
                            CONCAT22(unaff_SS, pu_var5),
                            CONCAT22(unaff_SS, &local_6),
                            param_3,
                        );
                        _local_e = CONCAT22(paVar10, pu_var5);
                        u_var11 = paVar10 | pu_var5;
                        if (u_var11 != 0) {
                            u_var13 = 100;
                            u_var14 = 0;
                            u_var15 = 0;
                            u_var4 = pass1_1028_b58e(CONCAT22(paVar10, pu_var5));
                            u_var7 = CONCAT31(extraout_var, u_var4);
                            local_1a = u_var7;
                            local_18 = u_var11;
                            pass1_1038_1a30(
                                u_var3,
                                u_var12,
                                CONCAT31(extraout_var, u_var4) & 0xffff | u_var11 << 0x10,
                            );
                            local_22 = (CONCAT22(u_var11, u_var7)
                                * CONCAT22(u_var15, CONCAT11(u_var14, u_var13)))
                                >> 1;
                            local_1e = u_var7;
                            local_1c = u_var11;
                        }
                    }
                    if ((local_22 == 0) || (local_a == 0)) {
                        break;
                    }
                }
                local_3a = local_3a + 1;
            }
            if (local_36 != 0) {
                pass1_1038_1b3a(u_var3, u_var12, _local_e, local_36);
                local_36 = 0;
            }
        }
        local_16 = local_16 + 1;
    }
}

pub unsafe fn pass1_1038_16f2(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut u_var4: u16;
    let pu_var5: *mut u32;
    let mut u_var6: i32;
    let paVar7: *mut AStruct493;
    let mut u_var8: u16;
    let mut u_var9: i32;
    let lVar10: u32;
    let mut u_var11: u32;
    let mut in_DX: i32;
    let mut u_var12: i32;
    let struct_a: *mut AStruct199;
    let mut u_var13: u32;
    let mut u_var14: u32;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let mut u_var15: u32;
    let mut u_var16: u16;
    let mut u_var17: u32;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = 0;
    local_a = 0;
    pu_var5 = &local_a;
    u_var16 = (param_1 >> 0x10);
    u_var3 = param_1;
    pass1_1038_13da(
        u_var3,
        u_var16,
        CONCAT22(unaff_SS, pu_var5),
        CONCAT22(unaff_SS, &local_6),
        param_3,
    );
    _local_e = CONCAT22(in_DX, pu_var5);
    in_DX = in_DX | pu_var5;
    if (in_DX != 0) {
        ppcVar2 = (param_2 + 0x10);
        u_var17 = param_2;
        ppcVar2();
        _local_12 = CONCAT22(in_DX, pu_var5);
        local_16 = 0;
        while (local_16 < _local_12) {
            ppcVar2 = (param_2 + 4);
            u_var15 = _local_12;
            u_var6 = in_DX;
            ppcVar2(unaff_CS, param_2, local_16, (local_16 >> 0x10), u_var17);
            paVar7 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var15, u_var6);
            unaff_CS = 0x1030;
            u_var15 = pass1_1030_73a8(CONCAT22(u_var6, paVar7));
            u_var12 = (u_var15 >> 0x10);
            u_var9 = u_var15;
            pass1_1038_1a30(u_var3, u_var16, CONCAT22(u_var6, paVar7));
            if ((u_var12 | u_var9) != 0) {
                local_2a = (CONCAT22(u_var12, u_var9) * 100) >> 1;
                u_var1 = &paVar7[1].field_0x4;
                u_var9 = &paVar7[1].field_0x6;
                u_var13 = u_var9;
                local_2e._0_2_ = u_var1;
                if ((u_var9 | local_2e) != 0) {
                    _local_32 = 0;
                    u_var8 = pass1_1028_0d80(u_var15);
                    local_38 = 0;
                    local_34 = u_var8;
                    loop {
                        u_var12 = u_var13;
                        u_var4 = (u_var1 >> 0x10);
                        u_var11 = pass1_1020_bae6(local_2e, CONCAT22(local_34, u_var4));
                        u_var9 = u_var11;
                        u_var13 = (u_var12 | u_var9);
                        if ((u_var12 | u_var9) != 0) {
                            u_var14 = u_var12;
                            if ((local_2a._2_2_ <= u_var12)
                                && (local_2a._2_2_ < u_var12 || (local_2a < u_var9)))
                            {
                                u_var14 = local_2a._2_2_;
                                u_var9 = local_2a;
                            }
                            if ((local_a._2_2_ <= u_var14)
                                && (local_a._2_2_ < u_var14 || (local_a < u_var9)))
                            {
                                u_var14 = local_a._2_2_;
                                u_var9 = local_a;
                            }
                            struct_a = u_var14;
                            local_44 = CONCAT22(struct_a, u_var9);
                            local_2a = CONCAT22(
                                (local_2a._2_2_ - struct_a) - (local_2a < u_var9),
                                local_2a - u_var9,
                            );
                            local_a = CONCAT22(
                                (local_a._2_2_ - struct_a) - (local_a < u_var9),
                                local_a - u_var9,
                            );
                            u_var13 = u_var14;
                            if (_local_32 == 0) {
                                u_var6 = u_var9;
                                process_struct_1000_179c(10, struct_a);
                                u_var13 = (struct_a | u_var6);
                                if ((struct_a | u_var6) == 0) {
                                    u_var6 = 0;
                                    u_var13 = 0;
                                } else {
                                    pass1_1020_ba3e(CONCAT22(struct_a, u_var6), 5, 5);
                                }
                                _local_32 = CONCAT22(u_var13, u_var6);
                            }
                            pass1_1020_bb8a(
                                _local_32,
                                (_local_32 >> 0x10),
                                u_var9,
                                u_var14,
                                local_34,
                            );
                            lVar10 = (u_var11 & 0xffff | u_var12 << 0x10) - local_44;
                            pass1_1020_bb8a(local_2e, u_var4, lVar10, (lVar10 >> 0x10), local_34);
                            u_var9 = u_var13;
                            local_38 = local_34;
                            if (local_a == 0) {
                                pass1_1038_1ac6(u_var3, u_var16, _local_e, _local_32);
                                _local_32 = 0;
                                pu_var5 = &local_a;
                                pass1_1038_13da(
                                    u_var3,
                                    u_var16,
                                    CONCAT22(unaff_SS, pu_var5),
                                    CONCAT22(unaff_SS, &local_6),
                                    param_3,
                                );
                                _local_e = CONCAT22(u_var9, pu_var5);
                                u_var13 = (u_var9 | pu_var5);
                                if ((u_var9 | pu_var5) == 0) {
                                    return;
                                }
                            }
                        }
                        unaff_CS = 0x1020;
                        if ((local_2a == 0) || (local_a == 0)) {
                            break;
                        }
                        unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
                        local_34 = pass1_1028_0d80(u_var15);
                        if ((local_34 == local_38) || (local_38 == 0 && (local_34 == u_var8))) {
                            break;
                        }
                    }
                    if (_local_32 != 0) {
                        pass1_1038_1ac6(u_var3, u_var16, _local_e, _local_32);
                    }
                }
            }
            local_16 = local_16 + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_1940(param_1: u32, param_2: u32, param_3: u32) {
    let ppcVar1: fn();
    let mut u_var2: i32;
    let mut u_var3: i32;
    let pu_var4: *mut u8;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let local_a: *mut AStruct1063;
    let mut local_8: u16;

    pu_var4 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 3);
    u_var2 = pu_var4;
    pass1_1038_4d6e(param_3, pu_var4 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, u_var2);
    ppcVar1 = (*_local_a + 0x10);
    u_var3 = u_var2;
    (**ppcVar1)(&PTR_LOOP_1050_1008, u_var2, extraout_DX);
    if ((extraout_DX_00 | u_var3) != 0) {
        pass1_1038_1482(param_1, param_2, _local_a);
    }
    if (_local_a != 0x0) {
        ppcVar1 = *_local_a;
        (**ppcVar1)(&PTR_LOOP_1050_1008, u_var2, extraout_DX, 1);
    }
    return;
}

pub unsafe fn pass1_1038_19a0(param_1: u32, param_2: u32, param_3: u32) {
    let ppcVar1: fn();
    let mut u_var2: u32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let pu_var5: *mut u8;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut local_a: u16;
    let mut local_8: u16;

    pu_var5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 2);
    u_var3 = pu_var5;
    pass1_1038_4d6e(param_3, pu_var5 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, u_var3);
    u_var2 = *_local_a;
    ppcVar1 = u_var2 + 8;
    u_var4 = u_var3;
    (**ppcVar1)(&PTR_LOOP_1050_1008, u_var3, extraout_DX);
    if ((extraout_DX_00 | u_var4) == 0) {
        wvsprintf_FUN_1030_840a(0xdf, &g_alloc_addr_1050_1050);
        if (_local_a != 0x0) {
            ppcVar1 = u_var2;
            (**ppcVar1)(0x1030, u_var3, extraout_DX, 1);
            return;
        }
    } else {
        pass1_1038_16f2(param_1, _local_a, param_2);
        if (_local_a != 0x0) {
            ppcVar1 = *_local_a;
            (**ppcVar1)(&PTR_LOOP_1050_1008, u_var3, extraout_DX, 1);
        }
    }
    return;
}

pub unsafe fn pass1_1038_1a30(param_1: u16, param_2: u16, param_1_00: u32) {
    let pu_var1: *mut u32;
    let ppcVar2: fn();
    let mut u_var3: i32;
    let mut u_var4: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut u_var5: u16;
    let mut unaff_CS: u16;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var5 = (param_1_00 >> 0x10);
    pu_var1 = (param_1_00 + 0x1e);
    u_var7 = (param_1_00 + 0x20);
    local_6._0_2_ = pu_var1;
    u_var3 = u_var7 | local_6;
    if (u_var3 != 0) {
        ppcVar2 = (*pu_var1 + 0x10);
        u_var6 = local_6;
        ppcVar2();
        _local_a = CONCAT22(extraout_DX, u_var3);
        local_12 = 0;
        while (local_12 < _local_a) {
            ppcVar2 = (*pu_var1 + 4);
            u_var4 = _local_a;
            ppcVar2(
                unaff_CS,
                local_6,
                (pu_var1 >> 0x10),
                local_12,
                u_var6,
                u_var7,
            );
            if ((extraout_DX_00 | u_var4) != 0) {
                unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var4, extraout_DX_00);
            }
            local_12 = local_12 + 1;
        }
        return;
    }
    return;
}

pub unsafe fn pass1_1038_1ac6(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut AStruct44,
    param_2_00: u32,
) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut local_120: u32;
    let mut local_11c: u32;
    let mut local_118: u16;
    let mut local_116: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1028_b58e(param_1_00);
    _local_6 = CONCAT31(extraout_var, u_var1) & 0xffff | in_DX << 0x10;
    pas1_1030_e8a0(
        CONCAT22(unaff_SS, &local_118),
        param_2_00,
        (param_1_00 + 0xc),
        (CONCAT31(extraout_var, u_var1) + 4),
    );
    pass1_1028_d52c(
        *_g_bool_1050_5748,
        *_PTR_LOOP_1050_65e2 + 1,
        CONCAT22(unaff_SS, &local_118),
    );
    return;
}

pub unsafe fn pass1_1038_1b3a(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut AStruct44,
    param_2_00: *mut AStruct44,
) -> i32 {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1028_b58e(param_1_00);
    _local_6 = CONCAT31(extraout_var, u_var1) & 0xffff | in_DX << 0x10;
    local_a = param_1_00;
    _local_e = pass1_1028_45e2(param_1_00);
    local_10 = (param_2_00 + 4);
    local_12 = 0;
    while (local_12 < local_10) {
        pass1_1020_bb16(
            param_2_00,
            CONCAT22(unaff_SS, &local_1a),
            CONCAT22(unaff_SS, &local_16),
            local_12,
        );
        if (_local_e < local_1a) {
            pass1_1030_7ddc(_local_6, _local_e, local_16);
            _local_e = 0;
        } else {
            _local_e = _local_e - local_1a;
            pass1_1030_7ddc(_local_6, local_1a, local_16);
        }
        if (_local_e == 0) {
            break;
        }
        local_12 = local_12 + 1;
    }
    if (param_2_00 != 0x0) {
        pass1_1020_ba7e(param_2_00);
        error_check_1000_17ce(param_2_00);
    }
    return;
}

pub unsafe fn pass1_1038_1c02(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_1c3e(param_1: u32, param_2: u32) {
    let pu_var1: *mut u32;
    let ppcVar2: fn();
    let mut u_var3: u32;
    let paVar4: *mut AStruct493;
    let mut iVar5: i32;
    let BVar6: bool;
    let pu_var7: *mut u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut u_var8: i32;
    let mut u_var9: u16;
    let mut unaff_CS: u16;
    let mut u_var10: u32;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_14: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var9 = (param_2 >> 0x10);
    pu_var1 = (param_2 + 0xc);
    u_var9 = (param_2 + 0xe);
    ppcVar2 = (*pu_var1 + 0x10);
    pu_var7 = pu_var1;
    u_var13 = pu_var1;
    ppcVar2();
    u_var3 = pu_var7 & 0xffff | extraout_DX << 0x10;
    local_e = 0;
    loop {
        if (u_var3 <= local_e) {
            return;
        }
        ppcVar2 = (*pu_var1 + 4);
        u_var10 = u_var3;
        ppcVar2(
            unaff_CS,
            pu_var1,
            (pu_var1 >> 0x10),
            local_e,
            u_var13,
            u_var9,
        );
        u_var8 = extraout_DX_00 | u_var10;
        if (u_var8 != 0) {
            unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var10, extraout_DX_00);
            _local_1a = CONCAT22(u_var8, paVar4);
            iVar5 = &paVar4[1].field_0x16;
            if ((iVar5 != 0) && ((&paVar4[1].field_0x16 + 2) != 0)) {
                u_var11 = param_1;
                u_var12 = (param_1 >> 0x10);
                pass1_1038_201a(u_var11, u_var12, CONCAT22(u_var8, paVar4));
                if (iVar5 == 0) {
                    u_var10 = pass1_1030_73a8(_local_1a);
                    iVar5 = (u_var10 + 0xc);
                    unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                    BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar5, 1);
                    if (BVar6 == 0) {
                        unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                        BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar5, 2);
                        if (BVar6 == 0) {
                            BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar5, 5);
                            if (BVar6 == 0) {
                                unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                                BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar5, 6);
                                if (BVar6 == 0) {}
                                // goto LAB_1038_1c76;
                            }
                            unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                            pass1_1038_2306(u_var11, u_var12, _local_1a);
                        } else {
                            pass1_1038_26ee(u_var11, u_var12, _local_1a);
                        }
                    } else {
                        pass1_1038_24e8(u_var11, u_var12, _local_1a);
                    }
                }
            }
        }
        // LAB_1038_1c76:
        local_e = local_e + 1;
    }
}

pub unsafe fn pass1_1038_1d68(param_1: u16, param_2: u16, param_1_00: *mut u32, param_2_00: u32) {
    let pu_var1: *mut u16;
    let mut u_var2: u16;
    let mut uvar3: u16;
    let mut u_var4: i32;
    let ppcVar5: fn();
    let Var6: u16;
    let mut u_var7: u32;
    let mut u_var8: i32;
    let mut bVar9: bool;
    let mut in_AX: u16;
    let local_AX_95: *mut AStruct1064;
    let pu_var10: *mut u8;
    let local_AX_435: *mut AStruct1066;
    let mut u_var11: u32;
    let mut u_var13: i32;
    let mut u_var14: i32;
    let mut in_EDX: u32;
    let local_BX_521: *mut AStruct1067;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let pu_var15: *mut u32;
    let mut local_62: u32;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_52: u32;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: [u8; 4];
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: [u8; 4];
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let temp_5f69f6c76a: *mut AStruct1065;
    let pu_var12: *mut u8;

    local_6 = 100;
    local_8 = 0;
    ppcVar5 = (*param_1_00 + 0x10);
    pu_var15 = param_1_00;
    (**ppcVar5)();
    _local_c = CONCAT22(in_EDX, in_AX);
    local_10 = 0;
    loop {
        if (_local_c <= local_10) {
            return;
        }
        ppcVar5 = (*param_1_00 + 4);
        u_var11 = _local_c;
        u_var14 = in_EDX;
        (**ppcVar5)(unaff_CS, param_1_00, local_10, pu_var15);
        local_12 = u_var14;
        local_14 = u_var11;
        u_var14 = local_12 | local_14;
        in_EDX = u_var14;
        if (u_var14 != 0) {
            local_18 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_14, local_12);
            local_16 = u_var14;
            unaff_CS = 0x1030;
            local_1c = pass1_1030_73a8(CONCAT22(local_16, local_18));
            in_EDX = local_1c >> 0x10;
            pu_var10 = local_20;
            ppcVar5 = (local_1c + 0x40);
            (**ppcVar5)(0x1030, local_1c, (local_1c >> 0x10), pu_var10, unaff_SS);
            if (pu_var10 == 0x0) {
                _local_24 = pass1_1028_62c8(local_1c);
                u_var11 = _local_24 >> 0x10;
                local_8 = 1;
                local_28 = (param_2_00 + 0x22);
                pass1_1008_5784(CONCAT22(unaff_SS, local_30), local_28);
                loop {
                    u_var14 = u_var11;
                    local_AX_95 = local_30;
                    unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                    pass1_1008_5b12(CONCAT22(unaff_SS, local_AX_95));
                    _local_34 = CONCAT22(u_var14, local_AX_95);
                    in_EDX = (u_var14 | local_AX_95);
                    if ((u_var14 | local_AX_95) == 0) {
                        break;
                    }
                    u_var2 = local_AX_95.field_0x4;
                    temp_5f69f6c76a = local_AX_95.field_0x6;
                    u_var3 = local_AX_95.field_0x8;
                    u_var13 = local_AX_95.field_0xc;
                    u_var4 = local_AX_95.field_0xa;
                    u_var8 = u_var13 / u_var4;
                    u_var11 = u_var13 % u_var4;
                    bVar9 = false;
                    if (((0 < temp_5f69f6c76a) && (!SBORROW2(temp_5f69f6c76a, 1)))
                        && (temp_5f69f6c76a == (&PTR_DAT_0005_0000_1050_0004 + 1)
                            || (temp_5f69f6c76a + -1) < 4
                            || (temp_5f69f6c76a == &BYTE_1050_0008)))
                    {
                        bVar9 = true;
                    }
                    if (bVar9) {
                        u_var11 = _local_24;
                        if (local_6 < _local_24) {
                            u_var11 = local_6 & 0xffff;
                            local_22 = local_6._2_2_;
                        }
                        u_var13 = local_22 | u_var11;
                        in_EDX = u_var13;
                        if (u_var13 == 0) {
                            break;
                        }
                        qVar6 = (u_var11 & 0xffff | local_22 << 0x10) / u_var8;
                        u_var13 = (qVar6 >> 0x10);
                        local_4e = qVar6;
                        if (local_4e < u_var4) {
                            pu_var1 = &local_AX_95.field_0xc;
                            *pu_var1 = *pu_var1 - u_var11;
                            pu_var1 = &local_AX_95.field_0xa;
                            *pu_var1 = *pu_var1 - local_4e;
                        } else {
                            ppcVar5 = (local_28 + 0xc);
                            (**ppcVar5)(
                                &PTR_LOOP_1050_1008,
                                local_28,
                                (local_28 >> 0x10),
                                _local_34,
                            );
                            local_2c = 0;
                            local_4e = u_var4;
                        }
                        pu_var12 = _PTR_LOOP_1050_68a2;
                        alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                        u_var14 = pu_var12;
                        local_52 = pu_var12 & 0xffff | u_var13 << 0x10;
                        if ((u_var13 | u_var14) == 0) {
                            local_52 = 0;
                        } else {
                            local_52 = s_1_1050_389a;
                            (u_var14 + 2) = &PTR_LOOP_1050_1008;
                            (u_var14 + 4) = 0;
                            (u_var14 + 6) = 0;
                            (u_var14 + 8) = 0;
                            (u_var14 + 10) = 0;
                            (u_var14 + 0xc) = 0;
                            local_52 = 0x56ce;
                            (u_var14 + 2) = 0x1018;
                        }
                        u_var14 = (local_52 >> 0x10);
                        local_BX_521 = local_52;
                        local_BX_521.field_0xa = local_4e;
                        u_var7 = local_4e * u_var8;
                        u_var11 = u_var7 >> 0x10;
                        local_BX_521.field_0xc = u_var7;
                        local_BX_521.field_0x4 = u_var2;
                        local_BX_521.field_0x6 = temp_5f69f6c76a;
                        local_BX_521.field_0x8 = u_var3;
                        pass1_1028_6408(local_1c, (local_52 & 0xffff | u_var14 << 0x10));
                    }
                }
            } else {
                ppcVar5 = (*param_1_00 + 8);
                (**ppcVar5)(0x1030, param_1_00, 0, local_10);
            }
        }
        local_10 = local_10 + 1;
    }
}

pub unsafe fn pass1_1038_1faa(param_1: u32, param_2: u32, param_3: u32) {
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut AStruct493;
    let mut iVar3: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppcVar1 = (param_3 + 0x10);
    (**ppcVar1)();
    _local_6 = CONCAT22(extraout_DX, in_AX);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        ppcVar1 = (param_3 + 4);
        u_var5 = _local_6;
        (**ppcVar1)();
        u_var4 = extraout_DX_00;
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var5, extraout_DX_00);
        u_var5 = pass1_1030_73a8(CONCAT22(u_var4, paVar2));
        iVar3 = u_var5;
        pass1_1038_1d68(param_1, (param_1 >> 0x10), param_2, u_var5);
        if (iVar3 == 0) {
            break;
        }
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn pass1_1038_201a(param_1: u16, param_2: u16, param_1_00: *mut AStruct918) {
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let ppcVar3: fn();
    let lVar4: u32;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let pu_var7: *mut u8;
    let mut u_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u32;
    let mut u_var11: i32;
    let mut u_var12: u32;
    let mut u_var13: u32;
    let pu_var14: *mut u8;
    let pu_var15: *mut u8;
    let mut u_var16: u16;
    let pu_var17: *mut u32;
    let struct_a: *mut AStruct922;
    let mut u_var18: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
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

    struct_a = param_1_00;
    u_var18 = (param_1_00 >> 0x10);
    u_var16 = 0x1030;
    pu_var17 = pass1_1030_6b16(struct_a);
    u_var6 = (pu_var17 >> 0x10);
    u_var5 = pu_var17;
    if ((u_var6 | u_var5) == 0) {
        return;
    }
    iVar2 = &struct_a.field_0x34;
    lVar4 = iVar2;
    u_var13 = lVar4 * 100;
    pu_var7 = (u_var13 >> 0x10);
    local_a = 0;
    _local_14 = 0;
    if ((u_var5 + 4) == 0) {
        if ((u_var5 + 6) == 0) {
            if ((u_var5 + 8) == 0) {}
            // goto LAB_1038_2102;
            u_var9 = pass1_1020_c42e((u_var5 + 8));
            u_var12 = *(u_var5 + 10) * u_var9;
            pu_var14 = (u_var12 >> 0x10);
            if (u_var12 + lVar4 * -100 != 0 && u_var13 <= u_var12) {
                u_var12 = u_var13 & 0xffff;
                pu_var14 = pu_var7;
            }
            u_var13 = u_var12 & 0xffff | ZEXT24(pu_var14) << 0x10;
            u_var10 = (u_var12 & 0xffff | ZEXT24(pu_var14) << 0x10) / u_var9;
            piVar1 = (u_var5 + 10);
            *piVar1 = *piVar1 - u_var10;
            local_a = (u_var13 / 100);
            u_var13 = u_var13 % 100;
            u_var12 = u_var13;
            if (u_var13 != 0) {
                local_a = local_a + 1;
                u_var12 = local_a;
            }
            u_var8 = u_var12;
            process_struct_1000_179c(0x2a, u_var13);
            u_var11 = u_var13 | u_var8;
            if (u_var11 == 0) {}
            // goto LAB_1038_20fa;
            pass1_1038_6838(
                CONCAT22(u_var13, u_var8),
                u_var10,
                (u_var5 + 8),
                local_a,
                &struct_a.field_0x4,
            );
        } else {
            u_var9 = switch_statement_1020_c3b4((u_var5 + 6));
            u_var12 = *(u_var5 + 10) * u_var9;
            pu_var14 = (u_var12 >> 0x10);
            if (u_var12 + lVar4 * -100 != 0 && u_var13 <= u_var12) {
                u_var12 = u_var13 & 0xffff;
                pu_var14 = pu_var7;
            }
            u_var13 = u_var12 & 0xffff | ZEXT24(pu_var14) << 0x10;
            u_var10 = (u_var12 & 0xffff | ZEXT24(pu_var14) << 0x10) / u_var9;
            piVar1 = (u_var5 + 10);
            *piVar1 = *piVar1 - u_var10;
            local_a = (u_var13 / 100);
            u_var13 = u_var13 % 100;
            u_var12 = u_var13;
            if (u_var13 != 0) {
                local_a = local_a + 1;
                u_var12 = local_a;
            }
            u_var8 = u_var12;
            process_struct_1000_179c(0x2a, u_var13);
            u_var11 = u_var13 | u_var8;
            if (u_var11 == 0) {}
            // goto LAB_1038_20fa;
            pass1_1038_675c(
                CONCAT22(u_var13, u_var8),
                u_var10,
                (u_var5 + 6),
                local_a,
                &struct_a.field_0x4,
            );
        }
    } else {
        pu_var14 = *(u_var5 + 10);
        pu_var15 = 0x0;
        if ((pu_var7 < 1) && (0x7fff < pu_var7 || (u_var13 < pu_var14))) {
            pu_var14 = u_var13;
            pu_var15 = pu_var7;
        }
        _local_18 = CONCAT22(pu_var15, pu_var14);
        piVar1 = (u_var5 + 10);
        *piVar1 = *piVar1 - pu_var14;
        local_a = (_local_18 / 100);
        u_var12 = _local_18 % 100;
        u_var13 = u_var12;
        if (u_var12 != 0) {
            local_a = local_a + 1;
            u_var13 = local_a;
        }
        u_var8 = u_var13;
        process_struct_1000_179c(0x2a, u_var12);
        u_var11 = u_var12 | u_var8;
        if (u_var11 == 0) {
            // LAB_1038_20fa:
            u_var16 = 0x1000;
            _local_14 = 0;
            // goto LAB_1038_2102;
        }
        pass1_1038_6590(
            CONCAT22(u_var12, u_var8),
            pu_var14,
            pu_var15,
            *(u_var5 + 4),
            local_a,
            *&struct_a.field_0x4,
        );
    }
    u_var16 = 0x1000;
    _local_14 = CONCAT22(u_var11, u_var8);
    // LAB_1038_2102:
    if (_local_14 != 0) {
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        u_var16 = 0x1030;
        pass1_1030_6c4c(param_1_00, iVar2 - local_a);
    }
    if ((u_var5 + 10) == 0) {
        if ((u_var6 | u_var5) != 0) {
            ppcVar3 = *pu_var17;
            (**ppcVar3)(u_var16, u_var5, u_var6, 1);
        }
    } else {
        pass1_1030_6c66(param_1_00, 0, pu_var17);
    }
    return;
}

pub unsafe fn pass1_1038_2306(param_1: u16, param_2: u16, param_1_00: *mut AStruct493) {
    let piVar1: *mut i32;
    let pu_var2: *mut u32;
    let ppcVar3: fn();
    let Var4: u16;
    let pu_var5: *mut u32;
    let mut u_var6: u32;
    let pu_var7: *mut u32;
    let mut u_var8: i32;
    let mut u_var9: i32;
    let mut u_var10: u32;
    let local_BX_19: *mut AStruct1068;
    let mut iVar11: i32;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u32;
    let mut local_30: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_26: u32;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var13 = 0x1030;
    u_var14 = pass1_1030_73a8(param_1_00);
    u_var10 = u_var14 >> 0x10;
    u_var12 = (param_1_00 >> 0x10);
    local_BX_19 = param_1_00;
    local_8 = local_BX_19.field_0x34;
    local_c = 100;
    pu_var2 = (u_var14 + 0x22);
    pu_var7 = pu_var2;
    loop {
        u_var8 = u_var10;
        ppcVar3 = (*pu_var2 + 0x10);
        (**ppcVar3)(u_var13, pu_var2, (pu_var2 >> 0x10));
        u_var9 = pu_var7;
        u_var14 = pu_var7 & 0xffff;
        pu_var5 = (u_var14 | u_var8 << 0x10);
        if ((u_var8 | u_var9) == 0) {
            break;
        }
        if ((u_var9 + 10) == 0) {
            u_var10 = (u_var8 | u_var9);
            if ((u_var8 | u_var9) != 0) {
                ppcVar3 = *pu_var5;
                (**ppcVar3)(u_var13, u_var9, u_var8, 1);
            }
        } else {
            local_18 = 0;
            local_1e = 0;
            if ((u_var9 + 6) == 0) {
                if ((u_var9 + 8) != 0) {
                    local_1e = pass1_1020_c42e((u_var9 + 8));
                    // goto LAB_1038_2385;
                }
            } else {
                local_1e = switch_statement_1020_c3b4((u_var9 + 6));
                // LAB_1038_2385:
                u_var13 = 0x1020;
                local_18 = ((u_var9 + 10) * local_1e);
            }
            local_c._2_2_ = 0;
            if (local_c < local_18) {
                local_18 = local_c & 0xffff;
            }
            _local_22 = local_18 | local_c._2_2_ << 0x10;
            u_var10 = local_18 | local_c._2_2_ << 0x10;
            qVar4 = u_var10 / local_1e;
            u_var6 = qVar4;
            u_var10 = u_var10 % local_1e;
            piVar1 = (u_var9 + 10);
            *piVar1 = *piVar1 - qVar4;
            if (*piVar1 == 0) {
                u_var10 = (u_var8 | u_var9);
                if ((u_var8 | u_var9) != 0) {
                    ppcVar3 = *pu_var5;
                    (**ppcVar3)(u_var13, u_var9, u_var8, 1);
                }
            } else {
                ppcVar3 = (*pu_var2 + 8);
                (**ppcVar3)();
            }
            local_c = local_c - _local_22;
            pu_var7 = 0x0;
            local_2a = 0;
            iVar11 = u_var14;
            if ((iVar11 + 6) == 0) {
                if ((iVar11 + 8) != 0) {
                    process_struct_1000_179c(0x2a, u_var10);
                    u_var9 = u_var10 | pu_var7;
                    u_var14 = u_var9;
                    if (u_var9 == 0) {}
                    // goto LAB_1038_244e;
                    pass1_1038_6838(
                        (pu_var7 & 0xffff | u_var10 << 0x10),
                        u_var6,
                        (iVar11 + 8),
                        1,
                        local_BX_19.field_0x4,
                    );
                    // goto LAB_1038_24b3;
                }
            } else {
                process_struct_1000_179c(0x2a, u_var10);
                u_var9 = u_var10 | pu_var7;
                u_var14 = u_var9;
                if (u_var9 == 0) {
                    // LAB_1038_244e:
                    u_var13 = 0x1000;
                    local_2a = 0;
                    u_var10 = u_var14;
                } else {
                    pass1_1038_675c(
                        (pu_var7 & 0xffff | u_var10 << 0x10),
                        u_var6,
                        (iVar11 + 6),
                        1,
                        local_BX_19.field_0x4,
                    );
                    // LAB_1038_24b3:
                    u_var13 = 0x1000;
                    local_2a = pu_var7 & 0xffff | u_var14 << 0x10;
                    u_var10 = u_var14;
                }
            }
            if (local_2a != 0) {
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                local_8 = local_8 - 1;
                if (local_8 == 0) {
                    break;
                }
                local_c = 100;
            }
        }
    }
    pass1_1030_6c4c(param_1_00, local_8);
    return;
}

pub unsafe fn pass1_1038_24e8(param_1: u16, param_2: u16, param_1_00: *mut AStruct493) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let struct_a: *mut AStruct199;
    let mut extraout_DX_00: i32;
    let mut u_var4: i32;
    let struct_a_00: *mut AStruct199;
    let mut extraout_DX_01: i32;
    let paVar5: *mut AStruct199;
    let local_BX_19: *mut AStruct1069;
    let mut u_var6: u16;
    let mut unaff_SS: u16;
    let mut local_3e: u32;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = pass1_1030_73a8(param_1_00);
    paVar5 = (_local_6 >> 0x10);
    u_var6 = (param_1_00 >> 0x10);
    local_BX_19 = param_1_00;
    local_8 = local_BX_19.field_0x34;
    local_c = (_local_6 + 0x28);
    local_10 = 100;
    local_12 = (local_c + 4);
    u_var2 = local_12;
    process_struct_1000_179c(10, paVar5);
    u_var4 = u_var2;
    if ((paVar5 | u_var4) == 0) {
        u_var4 = 0;
        u_var3 = 0;
    } else {
        pass1_1020_ba3e((u_var2 & 0xffff | ZEXT24(paVar5) << 0x10), 5, 5);
        u_var3 = extraout_DX;
    }
    _local_1c = CONCAT22(u_var3, u_var4);
    local_1e = 0;
    while (u_var2 = local_12, local_1e < local_12) {
        pass1_1020_bb16(
            local_c,
            CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_18)),
            CONCAT22(unaff_SS, &local_14),
            local_1e,
        );
        if (local_18 != 0) {
            u_var2 = local_18;
            local_10._2_2_ = local_18._2_2_;
            if (local_10 < local_18) {
                u_var2 = local_10 & 0xffff;
            }
            u_var4 = u_var2;
            u_var2 = u_var2 & 0xffff | local_10._2_2_ << 0x10;
            iVar1 = (local_18._2_2_ - local_10._2_2_) - (local_18 < u_var4);
            local_18 = CONCAT22(iVar1, local_18 - u_var4);
            pass1_1020_bb8a(
                local_c,
                (local_c >> 0x10),
                local_18 - u_var4,
                iVar1,
                local_14,
            );
            pass1_1020_bb70(_local_1c, u_var4, CONCAT22(local_14, local_10._2_2_));
            local_10 = local_10 - u_var2;
            if (local_10 == 0) {
                paVar5 = struct_a_00;
                process_struct_1000_179c(0x2a, struct_a_00);
                if ((paVar5 | u_var2) == 0) {
                    u_var2 = 0;
                } else {
                    pass1_1038_666e(
                        (u_var2 & 0xffff | ZEXT24(paVar5) << 0x10),
                        _local_1c,
                        1,
                        local_BX_19.field_0x4,
                    );
                }
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                paVar5 = struct_a;
                process_struct_1000_179c(10, struct_a);
                if ((paVar5 | u_var2) == 0) {
                    u_var2 = 0;
                    u_var4 = 0;
                } else {
                    pass1_1020_ba3e((u_var2 & 0xffff | ZEXT24(paVar5) << 0x10), 5, 5);
                    u_var4 = extraout_DX_00;
                }
                _local_1c = (u_var2 & 0xffff | u_var4 << 0x10);
                local_8 = local_8 - 1;
                if (local_8 == 0) {
                    break;
                }
                local_10 = 100;
            }
        }
        local_1e = local_1e + 1;
    }
    infinite_loop_1020_ba94(_local_1c, (_local_1c >> 0x10));
    paVar5 = (extraout_DX_01 | u_var2);
    if (paVar5 == 0x0) {
        if (_local_1c != 0x0) {
            pass1_1020_ba7e(_local_1c);
            error_check_1000_17ce(_local_1c);
        }
    } else {
        process_struct_1000_179c(0x2a, paVar5);
        if ((paVar5 | u_var2) != 0) {
            pass1_1038_666e(
                (u_var2 & 0xffff | ZEXT24(paVar5) << 0x10),
                _local_1c,
                1,
                local_BX_19.field_0x4,
            );
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    }
    pass1_1030_6c4c(param_1_00, local_8);
    return;
}

pub unsafe fn pass1_1038_26ee(param_1: u16, param_2: u16, param_1_00: *mut AStruct493) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let extraout_DX: *mut AStruct199;
    let paVar4: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let struct_a_00: *mut AStruct199;
    let extraout_DX_00: *mut AStruct199;
    let mut extraout_DX_01: i32;
    let paVar5: *mut AStruct199;
    let local_BX_19: *mut AStruct1070;
    let mut u_var6: u16;
    let mut u_var7: u32;
    let mut local_32: u16;
    let mut local_2e: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = pass1_1030_73a8(param_1_00);
    paVar5 = (u_var7 >> 0x10);
    u_var6 = (param_1_00 >> 0x10);
    local_BX_19 = param_1_00;
    local_8 = local_BX_19.field_0x34;
    local_c = pass1_1028_0d80(u_var7);
    u_var3 = local_c;
    local_10 = 100;
    process_struct_1000_179c(10, paVar5);
    if ((paVar5 | u_var3) == 0) {
        u_var3 = 0;
        paVar5 = 0x0;
    } else {
        pass1_1020_ba3e((u_var3 & 0xffff | ZEXT24(paVar5) << 0x10), 5, 5);
        paVar5 = extraout_DX;
    }
    _local_14 = (u_var3 & 0xffff | ZEXT24(paVar5) << 0x10);
    local_a = local_c;
    while {
        u_var1 = u_var3;
        pass1_1030_7c28(param_1_00, local_a);
        paVar4 = (paVar5 | u_var1);
        if (paVar4 != 0x0) {
            paVar4 = paVar5;
            u_var2 = u_var1;
            if ((local_10._2_2_ <= paVar5) && (local_10._2_2_ < paVar5 || (local_10 < u_var1))) {
                paVar4 = local_10._2_2_;
                u_var2 = local_10;
            }
            _local_24 = CONCAT22(paVar4, u_var2);
            pass1_1030_7d1c(
                param_1_00,
                u_var1 - u_var2,
                CONCAT22(local_a, paVar5 + (-(u_var1 < u_var2) - paVar4)),
            );
            pass1_1020_bb70(_local_14, u_var2, CONCAT22(local_a, paVar4));
            local_10 = local_10 - _local_24;
            paVar4 = struct_a;
            if (local_10 == 0) {
                paVar5 = struct_a;
                process_struct_1000_179c(0x2a, struct_a);
                local_a = _local_24;
                if ((paVar5 | local_a) == 0) {
                    local_a = 0;
                } else {
                    pass1_1038_666e(
                        (_local_24 & 0xffff | ZEXT24(paVar5) << 0x10),
                        _local_14,
                        1,
                        local_BX_19.field_0x4,
                    );
                }
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                paVar5 = struct_a_00;
                process_struct_1000_179c(10, struct_a_00);
                if ((paVar5 | local_a) == 0) {
                    local_a = 0;
                    paVar4 = 0x0;
                } else {
                    pass1_1020_ba3e(CONCAT22(paVar5, local_a), 5, 5);
                    paVar4 = extraout_DX_00;
                }
                _local_14 = CONCAT22(paVar4, local_a);
                local_8 = local_8 - 1;
                if (local_8 == 0) {
                    break;
                }
                local_10 = 100;
            }
        }
        local_a = pass1_1028_0d80(u_var7);
        u_var3 = local_a;
        if (local_c == 0) {
            local_c = local_a;
        }
        paVar5 = paVar4;
        local_c != local_a
    } {}
    u_var1 = (_local_14 >> 0x10);
    infinite_loop_1020_ba94(_local_14, u_var1);
    paVar5 = (extraout_DX_01 | local_a);
    if (paVar5 == 0x0) {
        if ((u_var1 | _local_14) != 0) {
            pass1_1020_ba7e(_local_14);
            error_check_1000_17ce(_local_14);
        }
    } else {
        process_struct_1000_179c(0x2a, paVar5);
        if ((paVar5 | local_a) != 0) {
            pass1_1038_666e(
                CONCAT22(paVar5, local_a),
                _local_14,
                1,
                local_BX_19.field_0x4,
            );
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    }
    pass1_1030_6c4c(param_1_00, local_8);
    return;
}

pub unsafe fn pass1_1038_28d8(param_1: *mut AStruct500) -> *mut AStruct500 {
    pass1_1028_d1dc(param_1, (s_0_023_1050_3a93 + 4));
    param_1.a = (s_fem110_wav_1050_29fa + 4);
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCRoboMove_1050_59f8,
    );
    return param_1;
}

pub unsafe fn pass1_1038_290e() {
    let paVar1: *mut AStruct493;
    let mut in_DX: i32;
    let mut local_4: u16;

    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
    if ((in_DX | paVar1) != 0) {
        pass1_1038_4918(CONCAT22(in_DX, paVar1));
    }
    pass1_1038_7a76(_PTR_LOOP_1050_5a64);
    return 1;
}

pub unsafe fn pass1_1038_0c00(param_1: u32) {
    let ppcVar1: fn();
    let mut u_var2: u32;
    let pu_var3: *mut u16;
    let mut u_var4: i32;
    let pu_var5: *mut u8;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut u_var6: i32;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: i32;
    let mut unaff_SS: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_14)),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    loop {
        pu_var3 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_SS, pu_var3));
        _local_18 = CONCAT22(extraout_DX, pu_var3);
        if ((extraout_DX | pu_var3) == 0) {
            break;
        }
        pass1_1038_0e78(param_1, CONCAT22(extraout_DX, pu_var3));
        pass1_1038_1220(param_1, CONCAT22(extraout_DX, pu_var3));
        u_var6 = extraout_DX_00;
        pu_var5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 1);
        u_var4 = pu_var5;
        pass1_1038_4d6e(
            CONCAT22(extraout_DX, pu_var3),
            pu_var5 & 0xffff | u_var6 << 0x10,
        );
        _local_20 = CONCAT22(extraout_DX_01, u_var4);
        ppcVar1 = (*_local_20 + 0x10);
        u_var6 = u_var4;
        (**ppcVar1)(&PTR_LOOP_1050_1008, u_var4, extraout_DX_01);
        if ((extraout_DX_02 | u_var6) != 0) {
            u_var2 = (param_1 + 0x108);
            if ((u_var2 + 0x82) != 0) {
                pass1_1038_19a0(
                    param_1,
                    CONCAT22(extraout_DX_01, u_var4),
                    CONCAT22(extraout_DX, pu_var3),
                );
            }
            pass1_1038_1940(param_1, CONCAT22(extraout_DX_01, u_var4), _local_18);
        }
        if (_local_20 != 0x0) {
            ppcVar1 = *_local_20;
            (**ppcVar1)(8, u_var4, extraout_DX_01, 1);
        }
        pass1_1038_1c3e(param_1, _local_18);
    }
    return;
}

pub unsafe fn pass1_1038_0b6a(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_008e(param_1: u16, param_2: u16, param_1_00: *mut AStruct1050) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let mut uvar3: u16;
    let mut iVar4: i32;
    let local_AX_270: *mut AStruct1052;
    let mut u_var5: i32;
    let local_BX_4: *mut AStruct1050;
    let local_ES_4: *mut AStruct1050;
    let ppVar7: *mut pass1_struct_1;
    let ppVar8: *mut pass1_struct_1;
    let mut in_stack_0000ffd4: u16;
    let mut local_22: u32;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff08964032: *mut AStruct1052;
    let mut temp_7ff94c183f6: i32;
    let mut u_var6: u32;

    local_ES_4 = (param_1_00 >> 0x10);
    local_BX_4 = param_1_00;
    if (local_BX_4.field_0x4 != 0x4000001) {
        return;
    }
    ppVar7 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(in_stack_0000ffd4, 0x2c));
    temp_7ff94c183f6 = ppVar7;
    u_var3 = (ppVar7 >> 0x10);
    iVar4 = temp_7ff94c183f6;
    pass1_fn_1008_612e(1, 100);
    local_c = 0;
    iVar1 = (temp_7ff94c183f6 + 10);
    if (iVar1 == 1) {
        local_c = 0x15;
    } else {
        if (iVar1 != 2) {
            if (iVar1 == 3) {
                local_c = 0x16;
            } else {
                if (iVar1 == 4) {
                    if (iVar4 < 0x32) {
                        local_c = 0x17;
                    } else {
                        local_c = 0x18;
                    }
                } else {
                    if (iVar1 == 5) {
                        local_c = 0x19;
                    }
                }
            }
        }
    }
    if (local_c != 0) {
        ppVar8 =
            process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(in_stack_0000ffd4, 0x2b));
        pass1_1010_043a(ppVar8, local_BX_4.field_0x4, local_c);
    }
    pass1_1008_eb74(ppVar7, 0);
    if ((((temp_7ff94c183f6 + 0xe) | (temp_7ff94c183f6 + 0xc)) == 0)
        && (local_BX_4.field_0x18 < 0xc9))
    {
        u_var2 = *_PTR_LOOP_1050_65e2;
        u_var6 = u_var2;
        pass1_fn_1008_612e(0, 8);
        u_var5 = u_var6;
        local_22._0_2_ = u_var2;
        local_22._2_2_ = (u_var2 >> 0x10);
        (temp_7ff94c183f6 + 0xc) = u_var5 + local_22 + 0x1e;
        (temp_7ff94c183f6 + 0xe) = (u_var5 >> 0xf)
            + local_22._2_2_
            + CARRY2(u_var5, local_22)
            + (0xffe1 < u_var5 + local_22);
    }
    return;
}

pub unsafe fn pass1_1038_01c0(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut iVar1: i32;
    let pu_var2: *mut u32;
    let ppcVar3: fn();
    let mut u_var4: u32;
    let paVar5: *mut AStruct493;
    let mut u_var6: u16;
    let BVar7: bool;
    let pu_var8: *mut u8;
    let pu_var9: *mut u8;
    let pu_var10: *mut u32;
    let mut u_var11: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut unaff_SS: u16;
    let mut u_var14: u32;
    let mut u_var15: u32;
    let u_var16: u8;
    let mut local_5c: u32;
    let mut local_58: u32;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_34: u16;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_28: u16;
    let mut local_26: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 2];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    pu_var9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x29);
    u_var6 = pu_var9;
    local_8 = u_var6;
    pass1_1038_4e78(param_1_00, pu_var9 & 0xffff | in_DX << 0x10);
    _local_c = CONCAT22(extraout_DX, u_var6);
    u_var13 = 0x1030;
    u_var14 = pass1_1030_bcae(local_e, unaff_SS);
    u_var12 = u_var14;
    ppcVar3 = (*_local_c + 0x10);
    (**ppcVar3)(0x1030, _local_c, (_local_c >> 0x10));
    _local_12 = CONCAT22(extraout_DX_00, u_var12);
    u_var12 = (param_1_00 >> 0x10);
    pu_var2 = (param_1_00 + 0xc);
    u_var12 = (param_1_00 + 0xe);
    u_var16 = SUB41(pu_var2, 0);
    ppcVar3 = (*pu_var2 + 0x10);
    pu_var10 = pu_var2;
    (**ppcVar3)();
    u_var4 = pu_var10 & 0xffff | extraout_DX_01 << 0x10;
    local_1e = 0;
    loop {
        if (_local_12 <= local_1e) {
            if (_local_c != 0x0) {
                ppcVar3 = *_local_c;
                (**ppcVar3)(u_var13, _local_c, (_local_c >> 0x10), 1, u_var16, u_var12);
            }
            return;
        }
        u_var13 = 0x1030;
        u_var11 = _local_12;
        pass1_1030_1d58(_local_c);
        iVar1 = (u_var11 + 0x10);
        local_32 = 0;
        while (local_32 < u_var4) {
            u_var13 = 0x1030;
            u_var15 = u_var4;
            pass1_1030_1d58(pu_var2);
            paVar5 = (u_var15 & 0xffff | extraout_DX_02 << 0x10);
            if (((extraout_DX_02 | u_var15) != 0) && ((u_var15 + 0x10) == iVar1)) {
                u_var15 = pass1_1030_73a8(paVar5);
                u_var13 = SUB42(&PTR_LOOP_1050_1008, 0);
                BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (u_var15 + 0xc), 0x30);
                if (BVar7 == 0) {
                    pu_var8 = local_e;
                    u_var13 = 0x1030;
                    pass1_1030_bd74(
                        pu_var8,
                        unaff_SS,
                        paVar5,
                        (u_var11 & 0xffff | extraout_DX_03 << 0x10),
                    );
                    if (pu_var8 < 6) {
                        local_4 = local_4 + 1;
                        break;
                    }
                }
            }
            local_32 = local_32 + 1;
        }
        local_1e = local_1e + 1;
    }
}

pub unsafe fn pass1_1038_0340(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: u32) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut iVar3: i32;
    let mut u_var4: u16;
    let mut local_13a: [u8; 284];
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = *_PTR_LOOP_1050_65e2;
    local_a = 0;
    local_c = 0;
    iVar3 = param_2_00;
    u_var4 = (param_2_00 >> 0x10);
    pass1_1038_4cea(
        param_2_00,
        CONCAT22(unaff_SS, &local_12),
        CONCAT22(unaff_SS, &local_e),
    );
    u_var2 = (iVar3 + 0x1f6);
    local_16 = u_var2;
    pass1_1030_38b8(u_var2, (u_var2 >> 0x10));
    u_var1 = u_var2;
    _local_1a = u_var2 & 0xffff | in_DX << 0x10;
    if (param_1_00 == 0) {
        if (local_e != 8) {
            local_a = (u_var2 & 0xffff | in_DX << 0x10) / 4;
            local_c = 8;
            // goto LAB_1038_054b;
        }
    } else {
        if (param_1_00 < 0xb) {
            if (local_e != 7) {
                local_a = (u_var2 & 0xffff | in_DX << 0x10) / 10;
                local_c = 7;
                // goto LAB_1038_054b;
            }
        } else {
            if (param_1_00 < 0x1a) {
                if (local_e != 6) {
                    local_a = (u_var2 & 0xffff | in_DX << 0x10) / 0x14;
                    local_c = 6;
                    // goto LAB_1038_054b;
                }
            } else {
                if (param_1_00 < 0x33) {
                    if (local_e != 5) {
                        local_a = (u_var2 & 0xffff | in_DX << 0x10) / 100;
                        local_c = 5;
                        // goto LAB_1038_054b;
                    }
                } else {
                    if (param_1_00 < 0x4c) {
                        if (local_6 % 3 != 0) {}
                        // goto LAB_1038_054b;
                        if (local_e != 4) {
                            local_a = _local_1a / 100;
                            local_c = 4;
                            // goto LAB_1038_054b;
                        }
                    } else {
                        if (param_1_00 < 0x65) {
                            if (local_6 % 5 != 0) {}
                            // goto LAB_1038_054b;
                            if (local_e != 3) {
                                local_a = _local_1a / 100;
                                local_c = 3;
                                // goto LAB_1038_054b;
                            }
                        } else {
                            if (param_1_00 < 0x97) {
                                if (local_6 % 10 != 0) {}
                                // goto LAB_1038_054b;
                                if (local_e != 2) {
                                    local_a = _local_1a / 100;
                                    local_c = 2;
                                    // goto LAB_1038_054b;
                                }
                            } else {
                                if ((200 < param_1_00) || (local_6 % 0x14 != 0)) {}
                                // goto LAB_1038_054b;
                                if (local_e != 1) {
                                    local_a = _local_1a / 100;
                                    local_c = 1;
                                    // goto LAB_1038_054b;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ((local_10 <= in_DX) && (local_10 < in_DX || (local_12 < u_var1))) {
        u_var1 = local_12;
        in_DX = local_10;
    }
    local_a = CONCAT22(in_DX, u_var1);
    // LAB_1038_054b:
    if (local_c != 0) {
        if ((_local_1a != 0) && (local_a == 0)) {
            local_a = 1;
        }
        pass1_1038_4cd0(param_2_00, local_a, local_c);
    }
    if ((local_a._2_2_ | local_a) != 0) {
        if ((iVar3 + 0x200) == 0x8000001) {
            local_1e = 2;
        } else {
            local_1e = 1;
        }
        _local_1e = CONCAT22(0x400, local_1e);
        pass1_1028_9944(
            CONCAT22(unaff_SS, local_13a),
            local_a,
            CONCAT22(0x400, local_1e),
            (iVar3 + 4),
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, local_13a));
        pass1_1028_9992(CONCAT22(unaff_SS, local_13a));
    }
    return;
}

pub unsafe fn pass1_1038_05d8(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: u32) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let mut in_EDX: u32;
    let mut u_var5: u16;
    let mut unaff_SS: u16;
    let mut local_158: [u8; 280];
    let mut local_40: u32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = *_PTR_LOOP_1050_65e2;
    local_a = 0;
    local_c = 0;
    pass1_1038_4cea(
        param_2_00,
        CONCAT22(unaff_SS, &local_12),
        CONCAT22(unaff_SS, &local_e),
    );
    local_16 = 0;
    local_1a = 0;
    local_1e = 0;
    pass1_1028_dc52(
        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_34)),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while {
        while {
            u_var3 = in_EDX;
            pu_var1 = &local_34;
            pass1_1028_e4ec(CONCAT22(unaff_SS, pu_var1));
            _local_22 = CONCAT22(u_var3, pu_var1);
            u_var4 = u_var3 | pu_var1;
            in_EDX = u_var4;
            if (u_var4 == 0) {}
            // goto LAB_1038_0668;
            (pu_var1 + 0x100) != 0x8000002
        } {}
        local_16 = CONCAT22(u_var3, pu_var1);
        u_var2 = (pu_var1 + 0xfb);
        local_1a = u_var2;
        pass1_1030_38b8(u_var2, (u_var2 >> 0x10));
        local_1e = u_var2 & 0xffff | u_var4 << 0x10;
        u_var4 = u_var4 | u_var2;
        in_EDX = u_var4;
        u_var4 == 0
    } {}
    // LAB_1038_0668:
    local_34 = s_1_1050_389a;
    local_32 = &PTR_LOOP_1050_1008;
    if ((local_16._2_2_ | local_16) == 0) {
        return;
    }
    if (param_1_00 == 1000) {
        if (local_e != 0x10) {
            local_a = local_1e / 4;
            local_c = 0x10;
            // goto LAB_1038_0841;
        }
    } else {
        if (param_1_00 < 0x3de) {
            if (param_1_00 < 0x3cf) {
                if (param_1_00 < 0x3b6) {
                    if (param_1_00 < 0x39d) {
                        if (param_1_00 < 900) {
                            if (param_1_00 < 0x352) {
                                if ((param_1_00 < 800) || (local_6 % 0x14 != 0)) {}
                                // goto LAB_1038_0841;
                                if (local_e != 9) {
                                    local_a = local_1e / 100;
                                    local_c = 9;
                                    // goto LAB_1038_0841;
                                }
                            } else {
                                if (local_6 % 10 != 0) {}
                                // goto LAB_1038_0841;
                                if (local_e != 10) {
                                    local_a = local_1e / 100;
                                    local_c = 10;
                                    // goto LAB_1038_0841;
                                }
                            }
                        } else {
                            if (local_6 % 5 != 0) {}
                            // goto LAB_1038_0841;
                            if (local_e != 0xb) {
                                local_a = local_1e / 100;
                                local_c = 0xb;
                                // goto LAB_1038_0841;
                            }
                        }
                    } else {
                        if (local_6 % 3 != 0) {}
                        // goto LAB_1038_0841;
                        if (local_e != 0xc) {
                            local_a = local_1e / 100;
                            local_c = 0xc;
                            // goto LAB_1038_0841;
                        }
                    }
                } else {
                    if (local_e != 0xd) {
                        local_a = local_1e / 100;
                        local_c = 0xd;
                        // goto LAB_1038_0841;
                    }
                }
            } else {
                if (local_e != 0xe) {
                    local_a = local_1e / 0x14;
                    local_c = 0xe;
                    // goto LAB_1038_0841;
                }
            }
        } else {
            if (local_e != 0xf) {
                local_a = local_1e / 10;
                local_c = 0xf;
                // goto LAB_1038_0841;
            }
        }
    }
    u_var2 = local_1e;
    if (local_12 < local_1e) {
        u_var2 = local_12 & 0xffff;
        local_1e._2_2_ = local_12._2_2_;
    }
    local_a = u_var2 & 0xffff | local_1e._2_2_ << 0x10;
    // LAB_1038_0841:
    if (local_c != 0) {
        if ((local_1e != 0) && (local_a == 0)) {
            local_a = 1;
        }
        pass1_1038_4cd0(param_2_00, local_a, local_c);
    }
    if ((local_a._2_2_ | local_a) != 0) {
        u_var5 = (param_2_00 >> 0x10);
        if ((param_2_00 + 0x200) == 0x8000001) {
            local_40 = (local_16 + 4);
        } else {
            local_40 = 0x4000001;
        }
        pass1_1028_9944(
            CONCAT22(unaff_SS, local_158),
            local_a,
            (param_2_00 + 4),
            local_40,
        );
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, local_158));
        pass1_1028_9992(CONCAT22(unaff_SS, local_158));
    }
    return;
}

pub unsafe fn pass1_1038_08d4(param_1: u16, param_2: u32, param_3: u32) {
    let mut unaff_SS: u16;
    let lVar1: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: u16;

    local_4 = 0;
    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_16),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while {
        lVar1 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_16));
        if (lVar1 == 0) {}
        // goto LAB_1038_0917;
        (lVar1 + 0x200) != 0x8000002
    } {}
    local_4 = 1;
    // LAB_1038_0917:
    local_16 = s_1_1050_389a;
    local_14 = &PTR_LOOP_1050_1008;
    if (local_4 != 0) {
        if (param_2 < 0xc90000) {
            pass1_1038_0340(param_1, param_2, param_2._2_2_, param_3);
            return;
        }
        if (0x31fffff < param_2) {
            pass1_1038_05d8(param_1, param_2, param_2._2_2_, param_3);
        }
    }
    return;
}

pub unsafe fn pass1_1030_ecc2(param_1: *mut AStruct500) -> *mut AStruct500 {
    pass1_1028_d1dc(param_1, 0xf9f);
    param_1.a = 0xb96;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCMorale_1050_59ce);
    return param_1;
}

pub unsafe fn pass1_1030_ecf8(param_1: u32) {
    let mut u_var1: u16;
    let mut iVar2: i32;
    let pu_var3: *mut u32;
    let ppcVar4: fn();
    let mut u_var5: u16;
    let mut u_var6: u32;
    let local_AX_122: *mut AStruct1045;
    let paVar7: *mut AStruct493;
    let mut u_var8: u16;
    let mut iVar9: i32;
    let mut u_var10: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut u_var11: i32;
    let mut u_var12: i32;
    let mut u_var13: u16;
    let mut unaff_SS: u16;
    let mut bVar14: bool;
    let ppVar15: *mut pass1_struct_1;
    let pu_var16: *mut u32;
    let mut u_var17: u16;
    let mut local_68: u16;
    let mut local_66: u16;
    let mut local_60: u16;
    let mut local_5e: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u32;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_c = 0;
    ppVar15 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(local_68, 0x2f));
    local_4 = (ppVar15 >> 0x10);
    local_a = ppVar15;
    local_6 = local_a;
    pass1_1010_ed3e(ppVar15);
    local_8 = extraout_DX;
    if ((extraout_DX | local_a) != 0) {
        local_c = pass1_1030_2aaa(CONCAT22(extraout_DX, local_a));
    }
    if (local_c < 2) {
        local_c = 0;
    }
    ppVar15 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(local_68, 2));
    local_e = (ppVar15 >> 0x10);
    local_10 = ppVar15;
    if ((0 < u16_1050_13ae) && (!SBORROW2(u16_1050_13ae, 1))) {
        if (u16_1050_13ae == 2 || (u16_1050_13ae - 1) < 1) {
            if (6 < local_c) {
                local_c = local_c - 2;
                // goto LAB_1030_ed5b;
            }
            bVar14 = SBORROW2(local_c, 4);
            iVar2 = local_c - 4;
        } else {
            if (u16_1050_13ae != 3) {}
            // goto LAB_1030_ed5b;
            bVar14 = SBORROW2(local_c, 7);
            iVar2 = local_c - 7;
        }
        if (bVar14 == (iVar2 < 0)) {
            local_c = local_c - 1;
        }
    }
    // LAB_1030_ed5b:
    pass1_1028_dc52(
        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_22)),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    loop {
        local_AX_122 = &local_22;
        pass1_1028_e4ec(CONCAT22(unaff_SS, local_AX_122));
        _local_26 = CONCAT22(extraout_DX_00, local_AX_122);
        if ((extraout_DX_00 | local_AX_122) == 0) {
            break;
        }
        u_var11 = *&local_AX_122.field_0x1f6;
        if (((local_AX_122.field_0x1fe != 0) && (local_AX_122.field_0x200 != 0x8000002))
            && (
                pass1_1030_38b8(u_var11, &local_AX_122.field_0x1f8),
                (extraout_DX_01 | u_var11) != 0,
            ))
        {
            pu_var3 = &local_AX_122.field_0xc;
            ppcVar4 = (*pu_var3 + 0x10);
            pu_var16 = pu_var3;
            (**ppcVar4)(&PTR_LOOP_1050_1028, pu_var3, &local_AX_122.field_0xe);
            u_var6 = pu_var16 & 0xffff | extraout_DX_02 << 0x10;
            local_36 = local_AX_122.field_0x18;
            u_var13 = SUB42(&PTR_LOOP_1050_1038, 0);
            pass1_1038_4760(local_AX_122, extraout_DX_00);
            local_38 = local_AX_122.field_0x22 / 10;
            u_var1 = local_AX_122.field_0x24;
            if (u_var1 < 0x33) {
                if (u_var1 < 0x32) {
                    local_38 = local_38 - 1;
                }
            } else {
                local_36 = local_36 + 1;
            }
            local_40 = 0;
            while (local_40 < u_var6) {
                ppcVar4 = (*pu_var3 + 4);
                u_var10 = u_var6;
                (**ppcVar4)(
                    u_var13,
                    pu_var3,
                    (pu_var3 >> 0x10),
                    local_40,
                    (local_40 >> 0x10),
                );
                u_var11 = extraout_DX_03 | u_var10;
                if (u_var11 != 0) {
                    u_var13 = SUB42(&PTR_LOOP_1050_1028, 0);
                    paVar7 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var10, extraout_DX_03);
                    pu_var16 = pass1_1030_73a8(CONCAT22(u_var11, paVar7));
                    u_var12 = (pu_var16 >> 0x10);
                    u_var11 = pu_var16;
                    if (((u_var12 | u_var11) != 0) && ((u_var11 + 0x12) == 5)) {
                        ppcVar4 = (*pu_var16 + 0x48);
                        (**ppcVar4)(&PTR_LOOP_1050_1028, u_var11, u_var12);
                        if (u_var11 < 0) {
                            local_38 = local_38 + u_var11;
                        } else {
                            local_36 = local_36 + u_var11;
                        }
                    }
                }
                local_40 = local_40 + 1;
            }
            iVar2 = local_38 - local_c;
            u_var1 = local_AX_122.field_0x20a;
            u_var17 = (param_1 >> 0x10);
            u_var5 = param_1;
            u_var8 = u_var1;
            pass1_1038_01c0(u_var5, u_var17, _local_26);
            iVar9 = u_var8 - u_var1;
            local_38 = iVar2 - iVar9;
            pass1_1038_008e(u_var5, u_var17, _local_26);
            if (iVar9 < 0) {
                local_38 = local_38 + iVar9;
            } else {
                local_36 = local_36 + iVar9;
            }
            if (1000 < local_36) {
                local_36 = 1000;
            }
            if (local_36 < 0) {
                local_36 = 0;
            }
            local_36 = local_36 + local_38;
            if (1000 < local_36) {
                local_36 = 1000;
            }
            if (local_36 < 0) {
                local_36 = 0;
            }
            pass1_1038_4d0e(_local_26, local_36);
            if (local_AX_122.field_0x4 == 0x4000001) {
                pass1_1038_08d4(u_var5, CONCAT22(local_36, u_var17), _local_26);
            }
            pass1_1038_095e(u_var5, u_var17, local_36, _local_26);
        }
    }
    return;
}

pub unsafe fn pass1_1030_ec86(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_eb86() {
    let mut iVar1: i32;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let pu_var4: *mut u32;
    let mut local_22: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_14),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    loop {
        pu_var4 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_14));
        u_var3 = (pu_var4 >> 0x10);
        if (pu_var4 == 0x0) {
            break;
        }
        if ((pu_var4 + 0x12) == 5) {
            iVar1 = (pu_var4 + 0xc);
            if (((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33)))
                && (iVar1 == 0x34
                    || iVar1 + -0x33 < 1
                    || (0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 2))))
            {
                ppcVar2 = (*pu_var4 + 0x2c);
                ppcVar2(&PTR_LOOP_1050_1028);
            }
        }
    }
    return 1;
}

pub unsafe fn pass1_1030_ea50(param_1: u32, param_2: *mut AStruct493) {
    let mut u_var1: u32;
    let BVar2: bool;
    let mut iVar3: i32;
    let mut u_var4: u16;
    let mut unaff_SS: u16;
    let mut u_var5: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 99999;
    u_var4 = (param_1 >> 0x10);
    iVar3 = param_1;
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (iVar3 + 0x110), 3);
    if (BVar2 != 0) {
        u_var5 = pass1_1030_73a8(param_2);
        local_c = (u_var5 >> 0x10);
        local_e = u_var5;
        local_6 = pass1_1028_45e2(u_var5);
    }
    u_var1 = (iVar3 + 0x108);
    local_8 = (u_var1 + 4);
    local_a = 0;
    loop {
        if (local_8 <= local_a) {
            return;
        }
        pass1_1020_bb16(
            (iVar3 + 0x108),
            CONCAT22(unaff_SS, &local_12),
            CONCAT22(unaff_SS, &local_e),
            local_a,
        );
        if (local_6 < local_12) {
            pass1_1030_7ddc(param_2, local_6, local_e);
            local_6 = 0;
        } else {
            local_6 = local_6 - local_12;
            pass1_1030_7ddc(param_2, local_12, local_e);
        }
        if ((local_6._2_2_ | local_6) == 0) {
            break;
        }
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn pass1_1030_eb14(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_e8f8(param_1: *mut AStruct1036) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let paVar4: *mut AStruct493;
    let mut in_DX: u16;
    let local_BX_4: *mut AStruct1036;
    let local_ES_4: *mut AStruct1036;
    let mut u_var5: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x108 != 0) {
        u_var3 = local_BX_4.field_0x10c;
        paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var3, (u_var3 >> 0x10));
        _local_6 = CONCAT22(in_DX, paVar4);
        u_var5 = pass1_1030_73a8(CONCAT22(in_DX, paVar4));
        if ((u_var5 + 0xc) == local_BX_4.field_0x110) {
            pass1_1030_ea50(param_1, _local_6);
        }
        u_var1 = local_BX_4.field_0x108;
        u_var2 = &local_BX_4.field_0x10a;
        _local_14 = CONCAT22(u_var2, u_var1);
        if ((u_var2 | u_var1) != 0) {
            pass1_1020_ba7e(CONCAT22(u_var2, u_var1));
            error_check_1000_17ce(_local_14);
        }
        &local_BX_4.field_0x108 = 0;
    }
    return 1;
}

pub unsafe fn pass1_1030_e864(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_e75e(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_e602(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_e546(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0x108);
    pass1_1028_e332(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    return 1;
}

pub unsafe fn pass1_1030_e410(param_1: *mut AStruct1021) {
    let mut u_var1: u32;
    let paVar2: *mut AStruct493;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut local_10: [u8; 6];
    let mut local_a: [u8; 4];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = (param_1 + 0x10c);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    if ((in_DX | paVar2) != 0) {
        local_6 = paVar2;
        paVar2 = pass1_1038_4fd8(CONCAT22(in_DX, paVar2), 0x21);
        if (paVar2 == 0x0) {
            pass1_1020_a43e(CONCAT22(unaff_SS, local_a));
            pass1_1008_3e54(CONCAT22(unaff_SS, local_10), 0, 2, 0xfffd);
            pass1_1020_a49a(
                CONCAT22(unaff_SS, local_a),
                CONCAT22(unaff_SS, local_10),
                0x7a,
            );
            pass1_1008_3e76(CONCAT22(unaff_SS, local_10), 0, 3, 0xfffe);
            pass1_1020_a49a(
                CONCAT22(unaff_SS, local_a),
                CONCAT22(unaff_SS, local_10),
                0x7a,
            );
            pass1_1008_3e76(CONCAT22(unaff_SS, local_10), 0, 3, 0xfffd);
            paVar2 = pass1_1020_a49a(
                CONCAT22(unaff_SS, local_a),
                CONCAT22(unaff_SS, local_10),
                0x21,
            );
        }
    }
    return paVar2;
}

pub unsafe fn pass1_1030_e2be(param_1: *mut AStruct500, param_2: u16, param_3: u32, param_4: u32) {
    let local_BX_19: *mut AStruct500;
    let mut u_var1: u16;

    pass1_1028_d1dc(param_1, s_fem133_wav_1050_2af7);
    u_var1 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    local_BX_19.field_0x108 = param_4;
    local_BX_19.field_0x10c = param_3;
    &local_BX_19.field_0x110 = param_2;
    param_1.a = 0xe4ea;
    local_BX_19.b = 0x1030;
    return;
}

pub unsafe fn pass1_1030_e300(param_1: u32) {
    let ppVar1: *mut pass1_struct_1;
    let mut in_stack_0000fff8: u32;

    ppVar1 = process_struct_1010_20ba(
        _g_AStruct372_1050_0ed0,
        CONCAT22((in_stack_0000fff8 >> 0x10), 0x2b),
    );
    pass1_1010_089e(ppVar1, (param_1 + 0x110), 2);
    return 1;
}

pub unsafe fn pass1_1030_e328(param_1: *mut AStruct1021) {
    let local_BX_3: *mut AStruct1021;
    let local_ES_3: *mut AStruct1021;

    local_ES_3 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0x110 == 0) {
        ret_1030_e4ba(param_1);
    } else {
        pass1_1030_e410((param_1 & 0xffff | ZEXT24(local_ES_3) << 0x10));
    }
    return 1;
}

pub unsafe fn pass1_1030_e0d4() {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let paVar3: *mut AStruct493;
    let local_AX_104: *mut AStruct1017;
    let paVar4: *mut AStruct493;
    let mut u_var5: i32;
    let mut extraout_DX: i32;
    let mut iVar6: i32;
    let local_BX_211: *mut AStruct1018;
    let mut iVar7: i32;
    let mut unaff_SI: u16;
    let mut u_var8: u16;
    let mut unaff_SS: u16;
    let ppVar9: *mut pass1_struct_1;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1c: [u8; 8];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar9 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_SI, 0x40));
    local_4 = (ppVar9 >> 0x10);
    local_6 = ppVar9;
    _local_a = pass1_1008_b820(ppVar9);
    u_var5 = (_local_a >> 0x10) | _local_a;
    if (u_var5 != 0) {
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x800);
        _local_e = CONCAT22(u_var5, paVar3);
        local_10 = (&paVar3[0xb].field_0xa != 0);
        pass1_1008_5784(CONCAT22(unaff_SS, local_1c), _local_a);
        loop {
            local_AX_104 = local_1c;
            pass1_1008_5b12(CONCAT22(unaff_SS, local_AX_104));
            _local_14 = CONCAT22(extraout_DX, local_AX_104);
            u_var5 = extraout_DX | local_AX_104;
            if (u_var5 == 0) {
                break;
            }
            if (local_AX_104.field_0x8 != 0) {
                u_var2 = local_AX_104.field_0xa;
                paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
                paVar3 = paVar4;
                pass1_1038_354a(CONCAT22(u_var5, paVar4));
                if (paVar3 != 0x0) {
                    u_var8 = (_local_14 >> 0x10);
                    if (local_10 == 0) {
                        local_BX_211 = ((local_14 + 0xe) * 0xc);
                        local_2a = (local_BX_211 + 0x58c4);
                        iVar6 = (local_BX_211 + 0x58c8);
                    } else {
                        iVar6 = (local_14 + 0xe) * 0xc;
                        local_2a = (iVar6 + 0x58be);
                        iVar6 = (iVar6 + 0x58c2);
                    }
                    iVar7 = iVar6;
                    pass1_1038_35a8(CONCAT22(u_var5, paVar4), ((local_14 + 0x10) * 2 + local_2a));
                    if (iVar7 != 0) {
                        u_var8 = (_local_14 >> 0x10);
                        iVar7 = _local_14;
                        piVar1 = (iVar7 + 0x10);
                        *piVar1 = *piVar1 + 1;
                        if (iVar6 <= (iVar7 + 0x10)) {
                            (iVar7 + 0x10) = 0;
                        }
                    }
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1030_e010(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1030_dcf4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_df0c(param_1: *mut AStruct44) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let lVar3: u32;
    let u_var4: u8;
    let mut u_var5: u16;
    let paVar6: *mut AStruct990;
    let extraout_var: u32;
    let mut u_var7: u32;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff07747202: *mut AStruct1015;

    u_var4 = pass1_1028_b58e(param_1);
    temp_7ff07747202 = CONCAT31(extraout_var, u_var4);
    u_var1 = &temp_7ff07747202.field_0x2e;
    local_a._0_2_ = u_var1;
    if ((temp_7ff07747202.field_0x30 | local_a) != 0) {
        u_var8 = (u_var1 >> 0x10);
        u_var1 = (local_a + 0x210);
        local_e._0_2_ = u_var1;
        if (((local_a + 0x212) | local_e) != 0) {
            u_var8 = (u_var1 >> 0x10);
            u_var2 = (local_e + 10);
            u_var5 = pass1_1030_dfcc(param_1);
            if (u_var5 != 0) {
                local_18._0_2_ = 1;
                local_18._2_2_ = 0;
                while (CONCAT22(local_18._2_2_, local_18) < u_var2) {
                    u_var7 = u_var2;
                    u_var9 = u_var5;
                    pass1_1030_1312(local_e, u_var8, local_18);
                    paVar6 = pass1_1030_cde8(u_var7, extraout_DX, u_var9);
                    if (-1 < paVar6) {
                        pass1_1030_cef8(
                            (u_var7 & 0xffff | extraout_DX << 0x10),
                            CONCAT31(extraout_var, u_var4) & 0xffff | in_DX << 0x10,
                            1,
                            paVar6,
                        );
                        (param_1 + 0x20) = (u_var7 + 4);
                        return;
                    }
                    lVar3 = CONCAT22(local_18._2_2_, local_18) + 1;
                    local_18._0_2_ = lVar3;
                    local_18._2_2_ = (lVar3 >> 0x10);
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1030_dc08(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_dc96(param_1: *mut AStruct763) -> *mut AStruct763 {
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0;
    param_1.field_0x0 = 0xe036;
    (param_1 + 2) = 0x1030;
    return param_1;
}

pub unsafe fn pass1_1030_dcc2(
    param_1: *mut AStruct1012,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> i32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    param_1.field_0x20 = 0;
    CONCAT22(param_2, param_1) = 0xe036;
    param_1.field_0x2 = 0x1030;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1030_dcf4(param_1: *mut AStruct44) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let u_var3: u8;
    let paVar4: *mut AStruct493;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let extraout_var: u32;
    let pu_var7: *mut u8;
    let mut u_var8: u32;
    let mut in_DX: i32;
    let mut u_var9: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut iVar10: i32;
    let mut u_var11: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let local_14: *mut AStruct1014;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff4f6878d8: *mut AStruct1013;

    u_var11 = (param_1 >> 0x10);
    iVar10 = param_1;
    param_1.ptr_a_lo = 0xe036;
    (iVar10 + 2) = 0x1030;
    if (_PTR_LOOP_1050_65e2 != 0) {
        u_var3 = pass1_1028_b58e(param_1);
        temp_7ff4f6878d8 = CONCAT31(extraout_var, u_var3);
        if ((iVar10 + 0x20) == 0) {
            if ((in_DX | temp_7ff4f6878d8) == 0) {
                paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
            } else {
                paVar4 = temp_7ff4f6878d8.field_0x2e;
                in_DX = temp_7ff4f6878d8.field_0x30;
            }
            u_var9 = in_DX;
            pu_var7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
            u_var5 = SUB42(pu_var7, 0);
            pass1_1038_4d6e(CONCAT22(in_DX, paVar4), pu_var7 & 0xffff | u_var9 << 0x10);
            _local_14 = CONCAT22(extraout_DX, u_var5);
            ppcVar2 = (*_local_14 + 0x10);
            u_var11 = u_var5;
            ppcVar2(&PTR_LOOP_1050_1038, u_var5, extraout_DX);
            _local_18 = CONCAT22(extraout_DX_00, u_var11);
            local_1c = 0;
            while (local_1c < _local_18) {
                u_var8 = _local_18;
                pass1_1030_1d7c(u_var5, extraout_DX, local_1c, (local_1c >> 0x10));
                if ((extraout_DX_01 | u_var8) != 0) {
                    u_var6 = pass1_1030_dfcc(param_1);
                    u_var6 = pass1_1030_cbf0(u_var8, extraout_DX_01, u_var6);
                    if (u_var6 != 0) {
                        break;
                    }
                }
                local_1c = local_1c + 1;
            }
            if (_local_14 != 0x0) {
                ppcVar2 = *_local_14;
                ppcVar2(0x38, u_var5, extraout_DX, 1);
            }
        } else {
            u_var1 = (iVar10 + 0x20);
            u_var9 = in_DX;
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
            if ((u_var9 | paVar4) != 0) {
                local_c = 0;
                match (iVar10 + 0xc) {
                    0x73 | 0x77 => {
                        local_c = 1;
                    }
                    0x74 | 0x78 => {
                        local_c = 2;
                    }
                    0x75 => {
                        local_c = 3;
                    }
                    0x76 => {
                        local_c = 5;
                    }
                }
                if (local_c != 0) {
                    pass1_1030_cc44(
                        paVar4,
                        u_var9,
                        1,
                        CONCAT31(extraout_var, u_var3) & 0xffff | in_DX << 0x10,
                        local_c,
                    );
                }
            }
        }
    }
    pass1_1028_b418(param_1);
    return;
}

pub unsafe fn pass1_1030_db78(param_1: u32) {
    let mut u_var1: i32;

    u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 6) {
        pass1_1028_bdac((param_1 & 0xffff | u_var1 << 0x10), 5);
    }
    return;
}

pub unsafe fn pass1_1030_db92(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u32;
    let paVar2: *mut AStruct493;
    let pu_var3: *mut u8;
    let mut u_var4: u16;
    let mut unaff_SS: u16;
    let mut u_var5: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    u_var5 = pass1_1030_bcae(local_4, unaff_SS);
    u_var4 = (u_var5 >> 0x10);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    u_var1 = &paVar2.field_0x10;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    pu_var3 = local_4;
    pass1_1030_bcde(
        pu_var3,
        unaff_SS,
        CONCAT22(u_var4, paVar2),
        param_1_00,
        param_5,
    );
    if (pu_var3 < 0) {
        PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    return;
}

pub unsafe fn pass1_1030_d868(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_d8f6(param_1: *mut AStruct763) -> *mut AStruct763 {
    let local_BX_12: *mut AStruct1008;
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    param_1.field_0x0 = 0xdc2e;
    local_BX_12.field_0x2 = 0x1030;
    if (local_BX_12.field_0xc == 0x4c) {
        local_BX_12.field_0xe = 0x43;
    } else {
        if (local_BX_12.field_0xc == 0x4d) {
            local_BX_12.field_0xe = 0x44;
        } else {
            local_BX_12.field_0xe = 0x45;
        }
    }
    return param_1;
}

pub unsafe fn pass1_1030_d942(
    param_1: *mut AStruct1009,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xdc2e;
    param_1.field_0x2 = 0x1030;
    if (param_1.field_0xc == 0x4c) {
        param_1.field_0xe = 0x43;
    } else {
        if (param_1.field_0xc == 0x4d) {
            param_1.field_0xe = 0x44;
        } else {
            param_1.field_0xe = 0x45;
        }
    }
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1030_d994(param_1: *mut AStruct44) {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let mut iVar3: i32;
    let local_BX_4: *mut AStruct1010;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x12 != 4) {
        return;
    }
    u_var5 = pass1_1028_b4f2(param_1);
    if ((u_var5 + 0x200) == 0x8000002) {
        u_var2 = local_BX_4.field_0x14;
        piVar1 = (u_var2 + 0x94);
        *piVar1 = *piVar1 + -1;
    } else {
        iVar3 = pass1_1028_cb04(param_1);
        if (iVar3 == 0) {
            return;
        }
        pass1_1030_dace(param_1);
        if (iVar3 == 0) {
            return;
        }
        u_var2 = local_BX_4.field_0x14;
        piVar1 = (u_var2 + 0x94);
        *piVar1 = *piVar1 + -1;
        pass1_1028_c952(param_1);
        pass1_1030_da22(param_1);
    }
    u_var2 = local_BX_4.field_0x14;
    if ((u_var2 + 0x94) < 1) {
        pass1_1028_bdac(param_1, 5);
    }
    return;
}

pub unsafe fn pass1_1030_da22(param_1: *mut AStruct44) {
    let pu_var1: *mut u32;
    let ppcVar2: fn();
    let mut u_var3: u32;
    let mut u_var4: u16;
    let BVar5: bool;
    let mut u_var6: i32;
    let pu_var7: *mut u32;
    let mut u_var8: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut u_var9: u32;
    let mut local_1a: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var9 = pass1_1028_b4f2(param_1);
    u_var4 = (u_var9 >> 0x10);
    pu_var1 = (u_var9 + 0xc);
    ppcVar2 = (*pu_var1 + 0x10);
    pu_var7 = pu_var1;
    ppcVar2(&PTR_LOOP_1050_1028, pu_var1, (u_var9 + 0xe));
    u_var3 = pu_var7 & 0xffff | extraout_DX << 0x10;
    local_12 = 0;
    loop {
        if (u_var3 <= local_12) {
            return;
        }
        u_var8 = u_var3;
        pass1_1030_1d7c(pu_var1, (pu_var1 >> 0x10), local_12, (local_12 >> 0x10));
        u_var6 = u_var8;
        if ((((extraout_DX_00 | u_var6) != 0)
            && (
                BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (u_var6 + 0xc), 4),
                BVar5 != 0,
            ))
            && (
                u_var9 = pass1_1028_6744(
                    CONCAT13((extraout_DX_00 >> 8), CONCAT12(extraout_DX_00, u_var6)),
                    0xd,
                ),
                ((u_var9 >> 0x10) | u_var9) != 0,
            ))
        {
            break;
        }
        local_12 = local_12 + 1;
    }
    pass1_1028_6228(u_var8 & 0xffff | extraout_DX_00 << 0x10, 1, 0, 0xd);
    return;
}

pub unsafe fn pass1_1030_dace(param_1: *mut AStruct44) {
    let pu_var1: *mut u32;
    let ppcVar2: fn();
    let mut u_var3: u32;
    let mut u_var4: u16;
    let BVar5: bool;
    let mut u_var6: i32;
    let pu_var7: *mut u32;
    let mut u_var8: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut u_var9: u32;
    let mut local_1c: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var9 = pass1_1028_b4f2(param_1);
    u_var4 = (u_var9 >> 0x10);
    pu_var1 = (u_var9 + 0xc);
    ppcVar2 = (*pu_var1 + 0x10);
    pu_var7 = pu_var1;
    ppcVar2(&PTR_LOOP_1050_1028, pu_var1, (u_var9 + 0xe));
    u_var3 = pu_var7 & 0xffff | extraout_DX << 0x10;
    local_14 = 0;
    loop {
        if (u_var3 <= local_14) {
            return;
        }
        u_var8 = u_var3;
        pass1_1030_1d7c(pu_var1, (pu_var1 >> 0x10), local_14, (local_14 >> 0x10));
        u_var6 = u_var8;
        if ((((extraout_DX_00 | u_var6) != 0)
            && (
                BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (u_var6 + 0xc), 4),
                BVar5 != 0,
            ))
            && (
                u_var9 = pass1_1028_6744(
                    CONCAT13((extraout_DX_00 >> 8), CONCAT12(extraout_DX_00, u_var6)),
                    0xd,
                ),
                ((u_var9 >> 0x10) | u_var9) != 0,
            ))
        {
            break;
        }
        local_14 = local_14 + 1;
    }
    return;
}

pub unsafe fn pass1_1030_c8da(param_1: u32, param_2: u32, param_3: u32) {
    let mut local_6: u32;

    local_6 = 0;
    if (param_3._2_2_ == 1) {
        (param_1 + 0x20) = param_2._2_2_;
    } else {
        local_6 = ret_1030_178e(param_1, param_2, param_3);
    }
    return local_6;
}

pub unsafe fn pass1_1030_c91a(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: This function may have set the stack p i32er

pub unsafe fn bad_fn_1030_c949() {
    let pcVar1: *mut libc::c_char;
    let u_var2: u8;
    let mut in_DX: u16;
    let mut in_BX: i32;
    let mut cVar3: u8;
    let pu_var4: *mut u16;
    // ppu_var6: *mut *mut u8;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let unaff_DI: *mut u8;
    let mut unaff_ES: u16;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    // appuStack8362: *mut *mut u8[3];
    let mut uStack8356: u32;
    let mut bStack8352: u8;
    // ppuStack4216: *mut *mut u8;
    let apuStack4176: [u8; 2073];
    let puStack30: *mut u8;
    let pu_var5: *mut u16;
    // ppu_var7: *mut *mut u8;

    puStack30 = &stack0xfffe;
    pu_var4 = &stack0xfffe;
    apuStack4176[0] = &stack0xfffe;
    pu_var5 = &stack0xfffe;
    cVar3 = '\r';
    while {
        unaff_BP = unaff_BP + -1;
        pu_var4 = pu_var4 + -1;
        *pu_var4 = *unaff_BP;
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    ppuStack4216 = apuStack4176;
    ppu_var6 = apuStack4176;
    appuStack8362[0] = apuStack4176;
    ppu_var7 = apuStack4176;
    cVar3 = '\x13';
    while {
        pu_var5 = pu_var5 + -1;
        ppu_var6 = ppu_var6 + -1;
        *ppu_var6 = *pu_var5;
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    ppu_var6 = appuStack8362;
    cVar3 = '\x1b';
    while {
        ppu_var7 = ppu_var7 + -1;
        ppu_var6 = ppu_var6 + -1;
        *ppu_var6 = *ppu_var7;
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    pcVar1 = unaff_DI + 0x1028;
    cVar3 = (in_BX >> 8);
    if (*pcVar1 != cVar3 && cVar3 <= *pcVar1) {
        pcVar1 = (in_BX + unaff_SI);
        *pcVar1 = *pcVar1 - in_DX;
        pcVar1 = (in_BX + unaff_SI);
        *pcVar1 = *pcVar1 - in_DX;
        u_var2 = _in(in_DX);
        *unaff_DI = u_var2;
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    0x1024 = uStack8356;
    0x1022 = unaff_CS;
    0x1020 = 0xc927;
    pass1_1028_b418(paRam00001024);
    if ((bStack8352 & 1) != 0) {
        0x1024 = uStack8356;
        0x1022 = &PTR_LOOP_1050_1028;
        0x1020 = 0xc936;
        error_check_1000_17ce(paRam00001024);
    }
    return uStack8356;
}

pub unsafe fn pass1_1030_c9a8(param_1: *mut AStruct763) -> *mut AStruct763 {
    let mut iVar1: i32;
    let mut u_var2: u16;

    pass1_1028_b354(param_1);
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x98) = 1;
    param_1.field_0x0 = 0xd88e;
    (iVar1 + 2) = 0x1030;
    pass1_1000_4906((param_1 & 0xffff0000 | (iVar1 + 0x20)), 0, 0x78);
    return param_1;
}

pub unsafe fn pass1_1030_c9e4(
    param_1: *mut AStruct764,
    param_2: *mut AStruct764,
    param_3: u16,
    param_3_00: u32,
) -> i32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    param_1.field_0x98 = 1;
    CONCAT22(param_2, param_1) = 0xd88e;
    param_1.field_0x2 = 0x1030;
    pass1_1000_4906(CONCAT22(param_2, &param_1.field_0x20), 0, 0x78);
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1030_ca26(param_1: *mut AStruct44, param_3: *mut u8) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let local_BX_39: *mut AStruct981;
    let local_BX_87: *mut AStruct982;
    let mut u_var2: u16;
    let mut local_8: u16;
    let local_4: *mut AStruct983;

    local_4 = 0x0;
    while (
        local_BX_87 = param_1,
        u_var2 = (param_1 >> 0x10),
        local_4 < 10,
    ) {
        if (((local_BX_87 + local_4 * 0xc + 0x26) == 2)
            || ((local_BX_87 + local_4 * 0xc + 0x26) == 1))
        {
            (local_BX_87 + local_4 * 0xc + 0x26) = 4;
        } else {
            u_var1 = pass1_1028_b58e(param_1);
            local_BX_39 = (local_BX_87 + local_4 * 0xc);
            pass1_1030_6e9c(
                CONCAT31(extraout_var, u_var1) & 0xffff | in_DX << 0x10,
                1,
                local_BX_39.field_0x24,
            );
            local_BX_39.field_0x20 = 0;
            local_BX_39.field_0x24 = 0;
            local_BX_39.field_0x26 = 0;
        }
        local_4 = &local_4.field_0x1;
    }
    pass1_1028_b46e(param_1, param_3);
    return;
}
