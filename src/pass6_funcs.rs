pub fn pass1_1038_c410(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_be4a(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_be76(param_1: *mut u8, param_2: *mut u8)

{
    let paVar1: *mut astruct_318;
    let BVar2: bool;

    if (param_2._2_2_ == 0)
    {
        BVar2 = 0;
        paVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x2b);
        pass1_1010_038e(paVar1, BVar2);
    }
    destroy_win_1040_7b98(param_1, param_2);
    return;
}



pub fn pass1_1038_be4a(param_1: *mut astruct_599)

{
    let local_ES_3: *mut u8;

    local_ES_3 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xc436;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}


pub fn pass1_1038_bd4a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_b7f0(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_bca8(param_1: *mut u8)
{
    let mut uVar1: i32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let extraout_DX: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let mut extraout_DX_01: u16;
    let mut uVar6: u16;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    uVar3 = (iVar7 + 0x8e);
    puVar5 = (uVar3 + 10);
    unsafe {
    ppcVar2 = (*puVar5 + 0x14);}
    ppcVar2();
    struct_a = extraout_DX;
    puVar4 = puVar5;
    if ((iVar7 + 0x70) != 0)
    {
        puVar4 = (iVar7 + 0x70);
        uVar1 = (iVar7 + 0x72);
        struct_a = (uVar1 | puVar4);
        if (struct_a != 0x0)
        {
            unsafe{
            ppcVar2 = *puVar4;}
            ppcVar2();
            struct_a = extraout_DX_00;
        }
    }
    process_struct_1000_179c(0x14, struct_a);
    if ((struct_a | puVar4) == 0)
    {
        puVar4 = 0x0;
        uVar6 = 0;
    }
    else
    {
        process_struct_1008_4c58(CONCAT22(struct_a, puVar4));
        uVar6 = extraout_DX_01;
    }
    (iVar7 + 0x70) = puVar4;
    (iVar7 + 0x72) = uVar6;
    pass1_1008_4d84((iVar7 + 0x70), puVar5);
    return;
}



pub fn pass1_1038_b7f0(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xbd70;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}



pub fn pass1_1038_b6e0(in_astruct: *mut astruct_112, param_2: *mut u8)

{
    let mut u_var1: u32;
    let in_AX: *mut u8;
    let local_astruct: *mut astruct_112;
    let mut uVar2: u16;
    let mut local_4: u16;

    local_4 = 1;
    loop
    {
        if (0x2a < local_4)
        {
            return in_AX;
        }
        uVar2 = (in_astruct >> 0x10);
        local_astruct = in_astruct;
        in_AX = ((local_astruct + local_4 * 4 + 2) |
                       (local_astruct + local_4 * 4));
        let uvar1_6_val = unsafe{*(uVar1 + 6)};
        if ((in_AX != 0x0) &&
            (uVar1 = (local_astruct + local_4 * 4), in_AX = param_2,
             uvar1_6_val == param_2)) {
            break;}
        local_4 = local_4 + 1;
    }
    (local_astruct + local_4 * 4) = 0;
    return '\0';
}


pub fn pass1_1038_ae28(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_ae08(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1038_ad4c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_abb0(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1038_adde(param_1: i32, param_2: *mut u8, param_3: *mut u8, param_3_00: *mut u8)

{
    pass1_1038_9b72(param_1, param_2, param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xae4e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}



pub fn pass1_1038_abb0(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xad72;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}



pub fn pass1_1038_aaf0(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_a8cc(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_a80c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_a6c8(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1038_a89e(param_1: *mut u16, param_2: u16)

{
    let puVar1: *mut u8;

    puVar1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, puVar1, 1, 0, CONCAT22(param_2, 0xfca));
    unsafe{
    *param_1 = 0xab16;
    (param_1 + 2) = &PTR_LOOP_1050_1038;}
    return param_1;
}



pub fn pass1_1038_a8cc(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xab16;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}


pub fn pass1_1038_a608(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_a4c2(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1038_a69a(param_1: *mut u16, param_2: *mut u8)

{
    let puVar1: *mut u8;

    puVar1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, puVar1, 1, 0, CONCAT22(param_2, 0xfc9));
    unsafe{*param_1 = 0xa832;
    (param_1 + 2) = &PTR_LOOP_1050_1038;}
    return param_1;
}



pub fn pass1_1038_a6c8(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa832;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}


pub fn pass1_1038_a494(param_1: *mut u16, param_2: u16)

{
    let puVar1: *mut u8;

    puVar1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, puVar1, 1, 0, CONCAT22(param_2, 0xfc8));
    unsafe{*param_1 = 0xa62e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;}
    return param_1;
}



pub fn pass1_1038_a4c2(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa62e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}



pub fn pass1_1038_a402(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_a36a(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_a2aa(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_a156(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1038_a33c(param_1: *mut u16, param_2: *mut u8) -> *mut u16

{
    let puVar1: *mut u8;

    puVar1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, puVar1, 1, 0, CONCAT22(param_2, 0xfc7));
    unsafe{*param_1 = 0xa428;
    (param_1 + 2) = &PTR_LOOP_1050_1038;}
    return param_1;
}



pub fn pass1_1038_a36a(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa428;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}


pub fn pass1_1038_a174(param_1: *mut u8, param_2: i32)

{
    if (param_2 == 1)
    {
        (param_1 + 0x8e) = 0;
    }
    return;
}


pub fn pass1_1038_a090(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_9fa4(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_9fa4(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa0b6;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}


pub fn pass1_1038_9f56(param_1: *mut astruct_44) -> *mut astruct_44

{
    byte **ppbVar1;
    let pbVar2: *mut byte;
    let mut bVar3: u8;
    let mut bVar4: u8;
    let mut cVar5: u8;
    let mut uVar6: i32;
    let mut in_CL: u8;
    let mut bVar7: u8;
    let mut in_DX: i32;
    let mut bVar8: u8;
    let mut in_BX: i32;
    let mut bVar9: u8;
    let puVar10: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let in_struct_1: *mut astruct_44;
    let in_stack_0000bf2a: *mut u8;
    let mut bStack78: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    puVar10 = &stack0xfffe;
    cVar5 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar10 = puVar10 + -1;
        unsafe{*puVar10 = *unaff_BP};
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    bVar9 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar9;
    bVar8 = (in_DX + 1 >> 8);
    bVar3 = bVar8 + bVar9;
    bVar11 = CARRY1(bVar8, bVar9) || CARRY1(bVar3, in_CF);
    uVar6 = in_DX + 1 & 0xff;
    in_struct_1 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pbVar2 = unaff_SI + in_BX;
    bVar7 = uVar6;
    unsafe{bVar8 = *pbVar2;
    bVar4 = *pbVar2 - bVar7;
    bVar12 = *pbVar2 < bVar7 || bVar4 < bVar11;
    *pbVar2 = bVar4 - bVar11;
    if (*pbVar2 != 0 && (SBORROW1(bVar8, bVar7) != SBORROW1(bVar4, bVar11)) == *pbVar2 < '\0')
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
        process_struct_1040_7728(in_struct_1, (&PTR_LOOP_1050_0000 + 1),
                                 CONCAT22(uVar6 | (bVar3 + in_CF) << 8, in_BX), 0xfba,
                                 in_stack_0000bf2a);
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


pub fn pass1_1038_9f02(param_1: *mut astruct_44) -> *mut astruct_44

{
    let pbVar1: *mut byte;
    byte **ppbVar2;
    let pcVar3: *mut libc::c_char;
    let puVar4: *mut u32;
    let mut bVar5: u8;
    let mut uVar6: i32;
    let mut uVar7: i32;
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
    let puVar18: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
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
    let in_struct_1: *mut astruct_68;
    let local_4e: u8;
    let puStack34: *mut u8;
    let mut bVar11: u8;

    puStack34 = &stack0xfffe;
    puVar18 = &stack0xfffe;
    cVar12 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar18 = puVar18 + -1;
        unsafe{*puVar18 = *unaff_BP};
        cVar12 = cVar12 + -1;
        '\0' < cVar12
    } {}
    bVar11 = (in_AX >> 8);
    bVar13 = (in_CX >> 8);
    bVar17 = bVar11 + bVar13;
    bVar10 = bVar17 + in_CF;
    bVar14 = in_DX;
    bVar19 = CARRY1(bStack007d, bVar14) ||
             CARRY1(bStack007d + bVar14, unaff_SI[CONCAT11((in_BX >> 8), 0x40)] < bVar14);
    pbVar1 = &stack0x4079 + unaff_SI;
    bVar16 = (in_DX >> 8);
    unsafe{
    bVar20 = CARRY1(*pbVar1, bVar16) || CARRY1(*pbVar1 + bVar16, bVar19);
    *pbVar1 = *pbVar1 + bVar16 + bVar19;
    pbVar1 = unaff_SI;
    bVar5 = *pbVar1;
    bVar8 = *pbVar1 + 0x40;
    bVar19 = 0xbf < *pbVar1 || CARRY1(bVar8, bVar20);
    *pbVar1 = bVar8 + bVar20;
    cVar12 = in_CX;
    if (*pbVar1 == 0 || (SCARRY1(bVar5, '@') != SCARRY1(bVar8, bVar20)) != *pbVar1 < '\0')
    {
        pbVar1 = unaff_SI + 0x3fc8;
        *pbVar1 = *pbVar1 + cVar12 + bVar19;
        cVar9 = PTR_LOOP_1050_4080;
        PTR_LOOP_1050_4080._0_1_ = '@';
        puVar18 = CONCAT11(cVar9, 0x40);
        pbVar1 = unaff_SI;
        *pbVar1 = *pbVar1 + cVar12 + (unaff_SI[0x4040] < bVar14);
        pcVar3 = (puVar18 + unaff_SI + 0x10);
        *pcVar3 = *pcVar3 + 'T';
        pcVar3 = (puVar18 + unaff_SI + 0x10);
        *pcVar3 = *pcVar3 + -0x34;
        puVar4 = (puVar18 + unaff_SI + 0x10);
        uVar6 = *puVar4;
        *puVar4 = *puVar4 + 0x81b6;
        puVar4 = (puVar18 + unaff_SI + 0x10);
        uVar7 = *puVar4;
        *puVar4 = *puVar4 + 0x60ea;
        pcVar3 = (puVar18 + unaff_SI);
        *pcVar3 = (*pcVar3 - (in_DX & 0xff)) - (0x9f15 < uVar7);
        iVar15 = (in_DX & 0xff | (bVar16 + cVar9 + (0x7e49 < uVar6)) << 8) - 1;
        pcVar3 = (puVar18 + unaff_SI + 0x10);
        *pcVar3 = *pcVar3 + 'f';
        pbVar1 = (puVar18 + unaff_SI + 0x10);
        bVar5 = *pbVar1;
        *pbVar1 = *pbVar1 - 0x22;
        if (-1 < *pbVar1)
        {
            bVar17 = (iVar15 >> 8);
            pcVar3 = (puVar18 + unaff_SI);
            *pcVar3 = (*pcVar3 - iVar15) -
                      (CARRY1(bVar17, bVar13) || CARRY1(bVar17 + bVar13, 0x21 < bVar5));
            loop
            {
                // WARNING: Do nothing block with infinite loop
            } 
        }
    }
    else
    {
        bVar20 = 0xbf < bVar16 || CARRY1(bVar16 + 0x40, bVar19);
        in_struct_1 = CONCAT22(&stack0xc73f, &stack0xfffe);
        pbVar1 = unaff_SI + 0x4040;
        bVar14 = (in_DX & 0xff);
        bVar5 = *pbVar1;
        bVar8 = *pbVar1 - bVar14;
        bVar21 = *pbVar1 < bVar14 || bVar8 < bVar20;
        *pbVar1 = bVar8 - bVar20;
        if (*pbVar1 == 0 || (SBORROW1(bVar5, bVar14) != SBORROW1(bVar8, bVar20)) != *pbVar1 < '\0')
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
        process_struct_1040_7728(in_struct_1, (&PTR_LOOP_1050_0000 + 1),
                                 CONCAT22(in_DX & 0xff | (bVar16 + 0x40 + bVar19) << 8, 0x4040), 0xfba, ((in_NT & 1) * 0x4000 | (SCARRY1(bVar11, bVar13) != SCARRY1(bVar17, in_CF)) * 0x800 | (in_IF & 1) * 0x200 | (in_TF & 1) * 0x100 | ((in_AX & 0xff | bVar10 << 8) < 0) * 0x80 | (bVar10 == 0) * 0x40 | (in_AF & 1) * 0x10 | ((POPCOUNT(bVar10) & 1) == 0) * 4 | (CARRY1(bVar11, bVar13) || CARRY1(bVar17, in_CF))));
        unaff_ES = (in_struct_1 >> 0x10);
        puVar18 = in_struct_1;
    }
    *puVar18 = 0xa0b6;
}
    puVar18[1] = &PTR_LOOP_1050_1038;
    return CONCAT22(unaff_ES, puVar18);
}


pub fn pass1_1038_9b72(param_1: i32, param_2: *mut u8, param_3: *mut u8, param_3_00: *mut u8)

{
    let mut local_4: u16;

    pass1_1040_b040(CONCAT22(param_2, param_1), CONCAT22(param_3_00, param_3),
                    (param_3_00 >> 0x10));
    (param_1 + 0x128) = 0;
    CONCAT22(param_2, param_1) = 0x9efa;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    local_4 = 0;
    while
    {
        (param_1 + local_4 * 2 + 0x94) = 0;
        local_4 = local_4 + 1;
        local_4 < 0x4a
    } {}
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1038_9b52(param_1: *mut astruct_44) -> *mut astruct_44

{
    let pbVar1: *mut byte;
    let mut bVar2: u8;
    let mut bVar3: u8;
    let mut cVar4: u8;
    let mut uVar5: i32;
    let mut in_CX: u16;
    let mut bVar6: u8;
    let mut in_DX: i32;
    let mut bVar7: u8;
    let mut in_BX: i32;
    let mut bVar8: u8;
    let puVar9: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar10: bool;
    let mut bVar11: bool;
    let mut iVar12: i32;
    let puVar13: *mut u8;
    let puVar14: *mut u8;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    puVar9 = &stack0xfffe;
    puVar13 = &stack0xfffe;
    cVar4 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar9 = puVar9 + -1;
        unsafe{*puVar9 = *unaff_BP};
        cVar4 = cVar4 + -1;
        '\0' < cVar4
    } {}
    puVar14 = &stack0xbf2a;
    bVar8 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar8;
    bVar7 = (in_DX + 1 >> 8);
    bVar2 = bVar7 + bVar8;
    bVar10 = CARRY1(bVar7, bVar8) || CARRY1(bVar2, in_CF);
    uVar5 = in_DX + 1 & 0xff;
    pbVar1 = unaff_SI + in_BX;
    bVar6 = uVar5;
    unsafe{
    bVar7 = *pbVar1;
    bVar3 = *pbVar1 - bVar6;
    bVar11 = *pbVar1 < bVar6 || bVar3 < bVar10;
    *pbVar1 = bVar3 - bVar10;
    if (*pbVar1 != 0 && (SBORROW1(bVar7, bVar6) != SBORROW1(bVar3, bVar10)) == *pbVar1 < '\0')
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
        pass1_1040_b040(CONCAT22(&stack0xbf2a, &stack0xfffe),
                        CONCAT22(uVar5 | (bVar2 + in_CF) << 8, in_BX), in_CX);
        (puVar13 + 0x128) = 0;
        CONCAT22(puVar14, puVar13) = 0x9efa;
        (puVar13 + 2) = &PTR_LOOP_1050_1038;
        iVar12 = 0;
        while
        {
            (puVar13 + iVar12 * 2 + 0x94) = 0;
            iVar12 = iVar12 + 1;
            iVar12 < 0x4a
        } {}
        return CONCAT22(puVar14, puVar13);
    }
    if (*pbVar1 != 0)
    {
        error_check_1000_17ce(param_1);
    }
}
    return param_1;
}

pub fn pass1_1038_9ad0(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_9a48(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_9a1e(param_1: i32, param_2: *mut u8, param_3: *mut u8, param_3_00: *mut u8) -> *mut u32

{
    pass1_1040_b040(CONCAT22(param_2, param_1), CONCAT22(param_3_00, param_3),
                    (param_3_00 >> 0x10));
    *CONCAT22(param_2, param_1) = 0x9af6;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1038_99fe(param_1: *mut astruct_44) -> *mut astruct_44

{
    byte **ppbVar1;
    let pbVar2: *mut byte;
    let mut bVar3: u8;
    let mut bVar4: u8;
    let mut cVar5: u8;
    let mut uVar6: i32;
    let mut in_CX: u16;
    let mut bVar7: u8;
    let mut in_DX: i32;
    let mut bVar8: u8;
    let mut in_BX: i32;
    let mut bVar9: u8;
    let puVar10: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let puVar13: *mut u8;
    let puVar14: *mut u8;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    puVar10 = &stack0xfffe;
    puVar13 = &stack0xfffe;
    cVar5 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar10 = puVar10 + -1;
        unsafe{*puVar10 = *unaff_BP};
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    puVar14 = &stack0xbf2a;
    bVar9 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar9;
    bVar8 = (in_DX + 1 >> 8);
    bVar3 = bVar8 + bVar9;
    bVar11 = CARRY1(bVar8, bVar9) || CARRY1(bVar3, in_CF);
    uVar6 = in_DX + 1 & 0xff;
    pbVar2 = unaff_SI + in_BX;
    bVar7 = uVar6;
    unsafe{
    bVar8 = *pbVar2;
    bVar4 = *pbVar2 - bVar7;
    bVar12 = *pbVar2 < bVar7 || bVar4 < bVar11;
    *pbVar2 = bVar4 - bVar11;
    if (*pbVar2 != 0 && (SBORROW1(bVar8, bVar7) != SBORROW1(bVar4, bVar11)) == *pbVar2 < '\0')
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
        pass1_1040_b040(CONCAT22(&stack0xbf2a, &stack0xfffe),
                        CONCAT22(uVar6 | (bVar3 + in_CF) << 8, in_BX), in_CX);
        (CONCAT22(puVar14, puVar13)).ptr_a_lo = 0x9af6;
        (puVar13 + 2) = &PTR_LOOP_1050_1038;
        return CONCAT22(puVar14, puVar13);
    }
    ppbVar1 = (unaff_SI + 9);
    *ppbVar1 = unaff_SI + *ppbVar1;
}
    error_check_1000_17ce(param_1);
    return param_1;
}


pub fn  pass1_1038_993a(param_1: *mut u8, param_2: *mut u8, param_1_00: i32) -> i16

{
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = 0;
    loop
    {
        if (0xe < local_6)
        {
            return 0xffff;
        }
        if ((local_6 * 0xe + 0x5a70) == param_1_00) {
            break;
        }
        local_6 = local_6 + 1;
    }
    return local_6;
}


pub fn pass1_1038_90a2(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_8cf6(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_8cf6(param_1: *mut astruct_44)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x90c8;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_b0f8(param_1);
    return;
}


pub fn pass1_1038_8c8a(param_1: *mut astruct_44, param_2: *mut u8) -> *mut astruct_44

{
    let pbVar1: *mut byte;
    let pcVar2: *mut libc::c_char;
    let mut bVar3: u8;
    let mut cVar4: u8;
    let mut uVar5: i32;
    let mut bVar6: u8;
    let mut in_DX: i32;
    let mut bVar7: u8;
    let mut in_BX: i32;
    let mut bVar10: u8;
    let mut iVar9: i32;
    let puVar11: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut unaff_DI: i32;
    let mut uVar12: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar13: bool;
    let mut bVar14: bool;
    let ppVar15: *mut pass1_struct_1;
    let local_4e: u8;
    let mut bVar8: u8;

    puVar11 = &stack0xfffe;
    cVar4 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar11 = puVar11 + -1;
        unsafe{*puVar11 = *unaff_BP;}
        
        cVar4 = cVar4 + -1;
        '\0' < cVar4
    } {}
    bVar10 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar10;
    bVar8 = (in_DX + 1 >> 8);
    bVar7 = bVar8 + bVar10;
    bVar13 = CARRY1(bVar8, bVar10) || CARRY1(bVar7, in_CF);
    bVar7 = bVar7 + in_CF;
    uVar5 = in_DX + 1 & 0xff;
    pbVar1 = unaff_SI + in_BX;
    bVar6 = uVar5;
    unsafe {
    bVar8 = *pbVar1;
    bVar3 = *pbVar1 - bVar6;
    bVar14 = *pbVar1 < bVar6 || bVar3 < bVar13;
    *pbVar1 = bVar3 - bVar13;
    if (*pbVar1 != 0 && (SBORROW1(bVar8, bVar6) != SBORROW1(bVar3, bVar13)) == *pbVar1 < '\0')
    {
        pbVar1 = unaff_SI;
        bVar8 = *pbVar1;
        bVar3 = *pbVar1;
        *pbVar1 = bVar3 + bVar10 + bVar14;
        bVar13 = CARRY1(local_4e, in_BX) ||
                 CARRY1(local_4e + in_BX, CARRY1(bVar8, bVar10) || CARRY1(bVar3 + bVar10, bVar14));
        pbVar1 = unaff_SI + -0x4f;
        bVar14 = CARRY1(*pbVar1, bVar10) || CARRY1(*pbVar1 + bVar10, bVar13);
        *pbVar1 = *pbVar1 + bVar10 + bVar13;
        bVar8 = bVar7 + bVar10;
        pcVar2 = (unaff_DI + -0x75);
        *pcVar2 = *pcVar2 + bVar6 + (CARRY1(bVar7, bVar10) || CARRY1(bVar8, bVar14));
        _in(uVar5 | (bVar8 + bVar14) << 8);
        process_struct_1040_b082(param_1, CONCAT22(param_2, 0x185a));
        uVar12 = (param_1 >> 0x10);
        iVar9 = param_1;
        (iVar9 + 0x94) = 0;
        param_1.ptr_a_lo = 0x90c8;
        (iVar9 + 2) = &PTR_LOOP_1050_1038;
        ppVar15 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_DI, 0x3f));
        (iVar9 + 0x94) = ppVar15;
        (iVar9 + 0x96) = (ppVar15 >> 0x10);
        return param_1;
    }
    if (*pbVar1 != 0)
    {
        error_check_1000_17ce(param_1);
    }
}
    return param_1;
}


pub fn pass1_1038_8c08(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_893a(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_893a(param_1: *mut astruct_348)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x8c2e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_b0f8(param_1);
    return;
}


pub fn pass1_1038_8850(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_7d5c(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_801a(param_1: *mut u8)

{
    let mut uVar1: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut uVar3: u32;
    let puVar4: *mut u8;
    let mut in_stack_0000fff0: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff0, 0x30));
    uVar1 = (param_1 >> 0x10);
    uVar3 = pass1_1008_b340((param_1 + 0x94));
    puVar4 = (uVar3 & 0xffff | ((uVar3 >> 0x10) | uVar3) << 0x10);
    if (uVar3 != 0)
    {
        pass1_1010_3770(ppVar2, uVar3);
        puVar4 = pass1_1038_af40(_g_astruct_112_a, *(param_1 + 6), 3);
    }
    return puVar4;
}


pub fn pass1_1038_7a76(param_1: *mut *mut u8)

{
    let ppcVar1: fn();
    let BVar2: bool;
    let struct_a: *mut astruct_306;
    let lVar3: u32;
    let paVar4: *mut astruct_1152;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8;4];
    let mut local_6: u32;

    pass1_1008_5784(CONCAT22(struct_a, local_a), param_1);
    while (lVar3 = pass1_1008_5b12(CONCAT22(struct_a, local_a)), lVar3 != 0)
    {
        pass1_1038_6a0e(lVar3);
    }
    while (paVar4 = pass1_1008_5b12(CONCAT22(struct_a, local_a)),
           paVar4 != 0x0)
    {
        BVar2 = pass1_1038_6b3c(paVar4);
        if (BVar2 != 0)
        {
            ppcVar1 = (param_1 + 0xc);
            (**ppcVar1)(&PTR_LOOP_1050_1008);
        }
    }
    pass1_1008_5784(CONCAT22(struct_a, local_a), (param_1 + 4));
    while (lVar3 = pass1_1008_5b12(CONCAT22(struct_a, local_a)), lVar3 != 0)
    {
        pass1_1030_affc(lVar3);
    }
    return;
}


pub fn pass1_1038_79f2(param_1: *mut u8,param_2: u32)

{
    let ppcVar1: fn();
    let puVar2: *mut u8;
    let mut extraout_DX: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8;8];
    let mut local_6: u32;

    local_6 = (param_2 + 4);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    pass1_1008_5784(CONCAT22(unaff_SS, local_e), (iVar3 + 4));
    while
    {
        puVar2 = local_e;
        pass1_1008_5b12(CONCAT22(unaff_SS, puVar2));
        if ((extraout_DX | puVar2) == 0)
        {
            return;
        }
        (puVar2 + 4) != local_6
    } {}
    ppcVar1 = ((iVar3 + 4) + 0xc);
    (**ppcVar1)(&PTR_LOOP_1050_1008, (iVar3 + 4), puVar2, extraout_DX);
    return;
}


pub fn pass1_1038_79b2(param_1: *mut u8,param_2: u32)

{
    let ppcVar1: fn();
    let struct_a_hi: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut local_ES_41: u16;
    let mut local_CS_6: u16;
    let mut local_DXAX_29: u32;
    let mut local_8: u16;

    local_CS_6 = 0x1000;
    process_struct_1000_179c(0x14, struct_a);
    if ((struct_a | struct_a_hi) == 0)
    {
        local_DXAX_29 = 0;
    }
    else
    {
        local_CS_6 = 0x1030;
        local_DXAX_29 = pass1_1030_aefa(CONCAT22(struct_a, struct_a_hi), param_2);
    }
    local_ES_41 = (param_1 >> 0x10);
    ppcVar1 = ((param_1 + 4) + 4);
    (**ppcVar1)(local_CS_6, (param_1 + 4), local_DXAX_29);
    return;
}



pub fn pass1_1038_7964(struct_a: *mut *mut astruct_1170)

{
    let puVar1: *mut u32;
    let struct_b: *mut astruct_1170;
    let struct_b_hi: *mut astruct_1170;
    let fn_ptr_1: fn();
    let mut uint_a: i32;

    _PTR_LOOP_1050_5a64 = 0;
    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    uint_a = struct_b.field_0x2;
    let struct_a_val = unsafe{*struct_a};
    if ((uint_a | struct_a_val) != 0)
    {
        fn_ptr_1 = struct_a_val;
        (**fn_ptr_1)();
    }
    puVar1 = struct_b.field_0x4;
    uint_a = struct_b.field_0x6;
    if ((uint_a | puVar1) != 0)
    {
        unsafe {
        fn_ptr_1 = *puVar1;
        (**fn_ptr_1)();
        }
    }
    return;
}


pub fn pass1_1038_78b8(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_6912(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1038_6e1a(param_1: u16, param_2: u16, param_1_00: *mut long)

{
    let paVar1: *mut astruct_493;
    let local_AX_76: *mut astruct_1155;
    let BVar2: bool;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: u16;

    local_4 = 0;
    let param_1_00_val = unsafe{*param_1_00};
    if ((param_1_00_val == 0) && (param_1_00 == 0))
    {
        return 1;
    }
    uVar4 = (param_1_00 + 2);
    local_16._1_1_ = (uVar4 >> 8);
    if (local_16._1_1_ == '\0')
    {
        local_4 = param_1_00;
      // goto switchD_1038_6eab_caseD_9;
    }

    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_1_00_val, (param_1_00_val >> 0x10));
    local_AX_76._0_1_ = pass1_1030_6fa0(CONCAT22(uVar4, paVar1));
    local_AX_76 = CONCAT11(local_AX_76._1_1_, local_AX_76);
    if (local_AX_76 < 10)
    {
        match (local_AX_76)
        {
        0x1 =>{
            local_4 = 1;
            },
        0x2 |
        0x6 =>{
            local_4 = 2;
            },
        0x3 |
        0x7 =>{
            local_4 = 3;
            },
        0x4 |
        0x8 =>{
            local_4 = 4;
            },
        0x5 |
        0x9 =>{
          // goto switchD_1038_6eab_caseD_5;
        }}
    }
    else
    {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, local_AX_76, 0x41);
        if (BVar2 != 0)
        {
            local_4 = 10;
          // goto switchD_1038_6eab_caseD_9;
        }
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, local_AX_76, 0x42);
        if ((BVar2 != 0) ||
            (local_AX_76 == (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 10)))
        {
            local_4 = 0xb;
          // goto switchD_1038_6eab_caseD_9;
        }
// switchD_1038_6eab_caseD_5:
        local_4 = 5;
    }
// switchD_1038_6eab_caseD_9:
    match (local_4)
    {
    1 =>{
        return 0x14;},
    2 |
    7 =>{
        return 0x3c;},
    3 |
    8 =>{
        return 0x78;},
    4 |
    9 =>{
        return 0xf0;},
    5 |
    6 =>{
        return 0xf;},
    10 =>{
        uVar3 = 0xc},
    0xb =>{
        uVar3 = 10},
        _ => {
        uVar3 = 0xffff;
    }}
    return uVar3;
}


 
pub fn pass1_1038_6d24(param_1: *mut astruct_1153, param_2: *mut *mut u8, param_3: *mut u16, param_4: *mut astruct_1154,
param_5: u16) -> i32

{
    let mut iVar1: i32;
    let mut uVar2: u16;
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

    unsafe{*param_2 = 0x0;}
    local_8 = param_4.field_0xc;
    uStack4 = param_4.field_0x10;
    pass1_1008_3eb4(CONCAT22(unaff_SS, &local_8), CONCAT22(unaff_SS, &local_e),
                    CONCAT22(unaff_SS, &local_c), CONCAT22(unaff_SS, &local_a));
    pass1_1008_3eb4((param_1 & 0xffff0000 | (param_1 + 0x1a)),
                    CONCAT22(unaff_SS, &local_14), CONCAT22(unaff_SS, &local_12),
                    CONCAT22(unaff_SS, &local_10));
    iVar1 = local_c - local_12;
    uVar2 = local_e - local_14;
    iVar3 = local_a - local_10;
    if (((iVar3 == 0) && (iVar1 == 0)) && (uVar2 == 0))
    {
        return 0;
    }
    if ((iVar1 != 0) || (iVar3 == 0))
    {
        if ((iVar3 == 0) && (iVar1 != 0))
        {
            pass1_1038_6c1c(param_1, param_3, param_2, iVar1);
            return 1;
        }
        if (((iVar3 == 0) && (iVar1 == 0)) && (uVar2 != 0))
        {
            iVar1 = pass1_1038_6c68(param_1, param_3, param_2, uVar2);
            if (iVar1 != 0)
            {
                return 1;
            }
            return iVar1;
        }
    }
    pass1_1038_6bd4(param_1, param_3, param_2, iVar3);
    return 1;
}



pub fn pass1_1038_6c68(param_1: *mut astruct_1153,param_2: u32,param_3: u32, param_4: u16)

{
    let mut iVar1: i32;
    let local_AX_13: *mut astruct_1153;
    let paVar2: *mut astruct_493;
    let mut extraout_DX: i32;
    let mut uVar3: i32;
    let mut uVar4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut uVar6: u32;
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
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffd8, 0x2f));
    uVar6 = param_1 & 0xffff0000 | &local_AX_13.field_0x1a;
    pass1_1030_627e(_PTR_LOOP_1050_5740, uVar6, (ppVar5 + 0x20));
    uVar3 = extraout_DX | uVar6;
    if (uVar3 != 0)
    {
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar6, extraout_DX);
        uVar6 = pass1_1030_73a8(CONCAT22(uVar3, paVar2));
        iVar1 = (uVar6 + 0xc);
        if ((iVar1 == 0x47) || (iVar1 == 0x6a))
        {
            uVar4 = (param_1 >> 0x10);
            iVar1 = local_AX_13.field_0x1e;
            if (param_4 < 0)
            {
                local_1e = iVar1 - 1;
            }
            else
            {
                local_1e = iVar1 + 1;
            }
            (param_2 + 4) = local_1e;
            pass1_1038_6b88(local_AX_13, uVar4, param_2, param_3);
        }
    }
    return;
}


pub fn pass1_1038_6bd4(param_1: u32, param_2: *mut u16, param_3: *mut *mut u8, param_4: i32)

{
    let mut local_4: u16;

    modify_list_1008_3f62(param_2, param_1 & 0xffff0000 | (param_1 + 0x1a));
    if (param_4 < 0)
    {
        unsafe{local_4 = *param_2 - 1;}
    }
    else
    {
        unsafe{local_4 = *param_2 + 1;}
    }
    unsafe{*param_2 = local_4;}
    pass1_1038_6b88(param_1, (param_1 >> 0x10), param_2, param_3);
    return;
}

pub fn pass1_1038_6c1c(param_1: u32, param_2: *mut u16, param_3: *mut *mut u8, param_4: i32)

{
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut local_4: u16;

    modify_list_1008_3f62(param_2, param_1 & 0xffff0000 | (param_1 + 0x1a));
    uVar2 = (param_2 >> 0x10);
    iVar1 = (param_2 + 2);
    if (param_4 < 0)
    {
        local_4 = iVar1 - 1;
    }
    else
    {
        local_4 = iVar1 + 1;
    }
    (param_2 + 2) = local_4;
    pass1_1038_6b88(param_1, (param_1 >> 0x10), param_2, param_3);
    return;
}


pub fn pass1_1038_6b88(param_1: u16, param_2: u16,param_1_00: u32, param_2_00: *mut *mut u8)

{
// ppuVar1: *mut *mut u8;
    let mut extraout_DX: u16;
    let mut unaff_SS: u16;
    let mut in_stack_0000ffec: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0,
                                        CONCAT22((in_stack_0000ffec >> 0x10), 0x2f));
    local_a = (_local_6 + 0x20);
    ppuVar1 = &stack0xffee;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_1_00, local_a, (local_a >> 0x10), ppuVar1,
                    unaff_SS);
    unsafe{*param_2_00 = *ppuVar1;}
    return;
}


pub fn pass1_1038_6b3c(param_1: *mut astruct_1152) -> bool

{
    let local_BX_3: *mut astruct_1152;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if ((((local_BX_3.field_0xc == 0) && (local_BX_3.field_0x12 == 0)) &&
         (local_BX_3.field_0x14 == 0)) &&
        ((local_BX_3.field_0xe == 0 && (local_BX_3.field_0x16 != 0))))
    {
        local_BX_3.field_0x16 = 0;
    }
    if (local_BX_3.field_0x16 == 0)
    {
        return 1;
    }
    return 0;
}


pub fn pass1_1038_6912(param_1: *mut *mut astruct_1149)

{
    let mut uVar1: i32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let puVar4: *mut u32;
    let local_BX_5: *mut astruct_1149;
    let mut uVar5: u16;
    let mut local_a: u32;

    uVar5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    unsafe{*param_1 = 0x78de;}
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    uVar1 = &local_BX_5.field_0x6;
    puVar4 = &local_BX_5.field_0x4;
    if ((uVar1 | puVar4) != 0)
    {
        unsafe{ppcVar3 = *puVar4;}
        (**ppcVar3)();
    }
    uVar1 = local_BX_5.field_0xe;
    uVar2 = local_BX_5.field_0x10;
    local_a = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0)
    {
        pass1_1020_ba7e(CONCAT22(uVar2, uVar1));
        error_check_1000_17ce(local_a);
    }
    unsafe{*param_1 = s_1_1050_389a;}
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1038_6984(param_1: *mut astruct_1150)

{
    let local_BX_3: *mut astruct_1150;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0xc != 0)
    {
        ret_one_1020_c3ae();
        return;
    }
    if (local_BX_3.field_0xe != 0)
    {
        infinite_loop_1020_ba94(local_BX_3.field_0xe);
        return;
    }
    if (local_BX_3.field_0x12 == 0)
    {
        if (local_BX_3.field_0x14 == 0)
        {
            return;
        }
        pass1_1020_c42e(local_BX_3.field_0x14);
    }
    else
    {
        switch_statement_1020_c3b4(local_BX_3.field_0x12);
    }
    return;
}

pub fn pass1_1038_69fe(param_1: *mut u8)

{
    (param_1 + 0x28) = 0;
    return;
}




pub fn pass1_1038_6a0e(param_1: *mut astruct_1153)

{
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let paVar3: *mut astruct_493;
    let puVar4: *mut u8;
    let BVar5: bool;
    let mut uVar6: u16;
    let mut in_DX: u16;
    let mut extraout_DX: i32;
    let mut uVar7: i32;
    let local_BX_4: *mut astruct_1151;
    let mut uVar8: u16;
    let mut unaff_SS: u16;
    let mut uVar9: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: [u8;4];
    let mut local_c: [u8;6];
    let mut local_6: u16;
    let mut local_4: u16;

    uVar8 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x28 == 0)
    {
        uVar2 = local_BX_4.field_0x20;
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
        _local_6 = CONCAT22(in_DX, paVar3);
        piVar1 = &local_BX_4.field_0x24;
        unsafe{*piVar1 = *piVar1 + 0x3c;}
        zero_list_1008_3e38(CONCAT22(unaff_SS, local_c));
        loop
        {
            puVar4 = local_10;
            pass1_1038_6d24(param_1, CONCAT22(unaff_SS, puVar4), CONCAT22(unaff_SS, local_c),
                            _local_6, (_local_6 >> 0x10));
            if (puVar4 == 0x0)
            {
                pass1_1010_8fba(local_BX_4.field_0x4);
                _local_16 = CONCAT22(extraout_DX, puVar4);
                uVar7 = extraout_DX | puVar4;
                if (uVar7 == 0)
                {
                    uVar2 = local_BX_4.field_0x20;
                    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
                    pass1_1038_7356(param_1, paVar3, uVar7);
                    return;
                }
                uVar9 = pass1_1030_73a8(_local_6);
                BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar9 + 0xc), 0x40);
                if (BVar5 != 0)
                {
                    local_BX_4.field_0x28 = 1;
                    local_BX_4.field_0x20 = _local_16;
                    return;
                }
                local_BX_4.field_0x20 = _local_16;
                uVar7 = extraout_DX;
                paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, &local_BX_4.field_0x20, extraout_DX);
                _local_6 = CONCAT22(uVar7, paVar3);
            }
            uVar6 = pass1_1038_6e1a(local_BX_4, uVar8, CONCAT22(unaff_SS, local_10));
            if (local_BX_4.field_0x24 < uVar6) {
                break;
            }
            piVar1 = &local_BX_4.field_0x24;
            unsafe{*piVar1 = *piVar1 - uVar6;}
            modify_list_1008_3f62((param_1 & 0xffff0000 | &local_BX_4.field_0x1a),
                                  CONCAT22(unaff_SS, local_c));
        }
    }
    return;
}


pub fn pass1_1038_6838(param_1: *mut *mut astruct_1148,param_2: u32, param_3: u16, param_4: u16,param_5: u32) -> i32

{
    let paVar1: *mut astruct_493;
    let paVar2: *mut astruct_493;
    let mut uVar3: i32;
    let puVar4: *mut u16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let local_BX_5: *mut astruct_1148;
    let mut uVar5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    unsafe{*param_1 = s_1_1050_389a;}
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
    unsafe{*param_1 = 0x78de;}
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, (param_5 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_DX);
    local_BX_5.field_0x4 = paVar2;
    &local_BX_5.field_0x6 = extraout_DX;
    puVar4 = (param_1 & 0xffff0000 | &local_BX_5.field_0x1a);
    modify_list_1008_3f62(puVar4, CONCAT22(in_DX, &paVar1.field_0xc));
    uVar3 = puVar4;
    pass1_1010_8fba(&local_BX_5.field_0x4);
    local_BX_5.field_0x20 = uVar3;
    local_BX_5.field_0x22 = extraout_DX_00;
    return;
}

pub fn pass1_1038_6590(param_1: *mut *mut astruct_1145, param_2: *mut u8, param_3: *mut u8, param_4: *mut u8, param_5: *mut u8, param_6: *mut u8) -> i32

{
    let paVar1: *mut astruct_493;
    let paVar2: *mut astruct_493;
    let mut uVar3: i32;
    let puVar4: *mut u16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let local_BX_6: *mut astruct_1145;
    let mut uVar5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    unsafe{*param_1 = s_1_1050_389a;}
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
    unsafe{*param_1 = 0x78de;}
    local_BX_6.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_6, (param_6 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_DX);
    local_BX_6.field_0x4 = paVar2;
    &local_BX_6.field_0x6 = extraout_DX;
    puVar4 = (param_1 & 0xffff0000 | &local_BX_6.field_0x1a);
    modify_list_1008_3f62(puVar4, CONCAT22(in_DX, &paVar1.field_0xc));
    uVar3 = puVar4;
    pass1_1010_8fba(&local_BX_6.field_0x4);
    local_BX_6.field_0x20 = uVar3;
    local_BX_6.field_0x22 = extraout_DX_00;
    return;
}



pub fn pass1_1038_666e(param_1: *mut *mut astruct_1146,param_2: u32, param_3: u16,param_4: u32)

{
    let paVar1: *mut astruct_493;
    let paVar2: *mut astruct_493;
    let mut uVar3: i32;
    let puVar4: *mut u16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let local_BX_5: *mut astruct_1146;
    let mut uVar5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    unsafe{*param_1 = s_1_1050_389a};
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
    unsafe{*param_1 = 0x78de;}
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_4, (param_4 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_DX);
    local_BX_5.field_0x4 = paVar2;
    &local_BX_5.field_0x6 = extraout_DX;
    puVar4 = (param_1 & 0xffff0000 | &local_BX_5.field_0x1a);
    modify_list_1008_3f62(puVar4, CONCAT22(in_DX, &paVar1.field_0xc));
    uVar3 = puVar4;
    pass1_1010_8fba(&local_BX_5.field_0x4);
    local_BX_5.field_0x20 = uVar3;
    local_BX_5.field_0x22 = extraout_DX_00;
    infinite_loop_1020_ba94(param_2);
    local_BX_5.field_0x16 = uVar3;
    local_BX_5.field_0x18 = extraout_DX_01;
    return;
}

pub fn pass1_1038_675c(param_1: *mut *mut astruct_1147,param_2: u32, param_3: u16, param_4: u16,param_5: u32) -> i32

{
    let paVar1: *mut astruct_493;
    let paVar2: *mut astruct_493;
    let mut uVar3: i32;
    let puVar4: *mut u16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let local_BX_5: *mut astruct_1147;
    let mut uVar5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    unsafe{*param_1 = s_1_1050_389a;}
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
    unsafe{*param_1 = 0x78de;}
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, (param_5 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_DX);
    local_BX_5.field_0x4 = paVar2;
    &local_BX_5.field_0x6 = extraout_DX;
    puVar4 = (param_1 & 0xffff0000 | &local_BX_5.field_0x1a);
    modify_list_1008_3f62(puVar4, CONCAT22(in_DX, &paVar1.field_0xc));
    uVar3 = puVar4;
    pass1_1010_8fba(&local_BX_5.field_0x4);
    local_BX_5.field_0x20 = uVar3;
    local_BX_5.field_0x22 = extraout_DX_00;
    return;
}


pub fn pass1_1038_64de(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_33f8(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1038_5b3c(param_1: *mut u8, param_2: *mut u8,param_1_00: u32, param_2_00: *mut u8)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uVar3: u32;
    let BVar4: bool;
    let local_SI_152: *mut astruct_1138;
    let mut uVar5: u16;
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < param_1_00)
    {
        uVar5 = (param_2_00 >> 0x10);
        local_SI_152 = param_2_00;
        if ((((local_SI_152 + local_6 * 4) != 0) &&
             (uVar2 = (local_SI_152 + local_6 * 4),
              BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar2 + 0xc), 0x2d), BVar4 != 0)) &&
            (ppcVar1 = ((local_SI_152 + local_6 * 4) + 0x50),
             (**ppcVar1)(), BVar4 != 0))
        {
            uVar2 = (local_SI_152 + local_6 * 4);
            uVar3 = (local_SI_152 + local_6 * 4);
            (uVar3 + 0x1a) = (uVar2 + 0x1a) | 1;
            ppcVar1 = ((local_SI_152 + local_6 * 4) + 0x28);
            (**ppcVar1)();
        }
        local_6 = local_6 + 1;
    }
    return;
}




pub fn pass1_1038_5be8(param_1: *mut u8, uparam_2: i32, param_3: u16,param_4: u32)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let paVar3: *mut astruct_493;
    let BVar4: bool;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let lVar7: u32;
    let mut uVar8: u32;
    let mut local_1e: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_4: u16;
    let temp_7ffe889d3b9: *mut astruct_1139;

    lVar7 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_4, (param_1 + 8));
    uVar5 = (lVar7 >> 0x10);
    uVar6 = uVar5 | lVar7;
    if (lVar7 != 0)
    {
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar7, uVar5);
        _local_a = CONCAT22(uVar6, paVar3);
        local_e = 0x7a;
        if (0 < (param_4 + 4))
        {
            if (param_3 == 0x7b)
            {
                param_3 = 0x7e;
            }
            else
            {
                if (param_3 == 0x7c)
                {
                    param_3 = 0x7d;
                }
            }
            local_e = 0x7f;
        }
        uVar8 = pass1_1030_73a8(_local_a);
        uVar2 = (uVar8 >> 0x10);
        temp_7ffe889d3b9 = uVar8;
        if ((((temp_7ffe889d3b9.field_0x1a & param_2) == 0) &&
             (((uVar1 = temp_7ffe889d3b9.field_0xc, uVar1 == local_e || (uVar1 == param_3)) ||
               (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x2b), BVar4 != 0)))) &&
            (temp_7ffe889d3b9.field_0x12 != 7))
        {
            temp_7ffe889d3b9.field_0x1a = temp_7ffe889d3b9.field_0x1a | param_2;
            return (&PTR_LOOP_1050_0000 + 1);
        }
    }
    return 0x0;
}

pub fn pass1_1038_5cc6(param_1: *mut u8,param_2: u32, param_3: *mut u8, param_4: *mut u8, param_5: u16, uparam_6: i32) -> i32

{
    let mut u_var1: u32;
    let mut uVar2: u32;
    let puVar3: *mut u8;
    let mut uVar4: u16;
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
    while
    {
        local_4 = 0;
        local_e = 0;
        while (local_e < param_2)
        {
            uVar4 = (param_4 >> 0x10);
            if ((local_e * 4 + param_4) != 0)
            {
                uVar1 = (local_e * 4 + param_4);
                modify_list_1008_3f62(CONCAT22(unaff_SS, &local_a), uVar1 & 0xffff0000 | (uVar1 + 0xc));
                pass1_1008_3eb4(CONCAT22(unaff_SS, &local_a), CONCAT22(unaff_SS, &local_14),
                                CONCAT22(unaff_SS, &local_12), CONCAT22(unaff_SS, &local_10));
                if (local_14 == param_5)
                {
                    uVar4 = (param_3 >> 0x10);
                    if (((local_e * 4 + param_3) != 0) &&
                        (uVar2 = (local_e * 4 + param_3),
                         ((uVar2 + 0x1a) & param_6) != 0))
                    {
                        local_8 = local_12 - 1;
                        puVar3 = pass1_1038_5be8(param_1, param_6, 0x7b, CONCAT22(unaff_SS, &local_a));
                        if (puVar3 != 0x0)
                        {
                            local_4 = 1;
                        }
                        local_8 = local_12 + 1;
                        puVar3 = pass1_1038_5be8(param_1, param_6, 0x7b, CONCAT22(unaff_SS, &local_a));
                        if (puVar3 != 0x0)
                        {
                            local_4 = 1;
                        }
                        local_8 = local_12;
                        local_a = local_10 - 1;
                        puVar3 = pass1_1038_5be8(param_1, param_6, 0x7c, CONCAT22(unaff_SS, &local_a));
                        if (puVar3 != 0x0)
                        {
                            local_4 = 1;
                        }
                        local_a = local_10 + 1;
                        puVar3 = pass1_1038_5be8(param_1, param_6, 0x7c, CONCAT22(unaff_SS, &local_a));
                        if (puVar3 != 0x0)
                        {
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



pub fn pass1_1038_5a96(param_1: *mut u8, param_2: *mut u8,param_1_00: u32, param_2_00: *mut u8)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let BVar3: bool;
    let local_SI_146: *mut astruct_1137;
    let mut local_ES_146: u16;
    let mut local_6: u32;
    let mut temp_5fa6353e1c: u32;

    local_6 = 0;
    while (local_6 < param_1_00)
    {
        local_ES_146 = (param_2_00 >> 0x10);
        local_SI_146 = param_2_00;
        if (((local_SI_146 + local_6 * 4) != 0) &&
            (temp_5fa6353e1c = (local_SI_146 + local_6 * 4),
             BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (temp_5fa6353e1c + 0xc), 0x2c),
             BVar3 != 0))
        {
            ppcVar1 = ((local_SI_146 + local_6 * 4) + 0x54);
            (**ppcVar1)();
            if (BVar3 != 0)
            {
                uVar2 = (local_SI_146 + local_6 * 4);
                (uVar2 + 0x1a) = 3;
                ppcVar1 = ((local_SI_146 + local_6 * 4) + 0x28);
                (**ppcVar1)();
                uVar2 = (local_SI_146 + local_6 * 4);
                (uVar2 + 0x1a) = 2;
            }
        }
        local_6 = local_6 + 1;
    }
    return;
}


pub fn pass1_1038_5a16(param_1: u16, param_2: u16,param_1_00: u32, param_2_00: *mut astruct_1135) -> i32

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let local_AX_36: *mut astruct_1136;
    let local_SI_109: *mut astruct_1135;
    let mut uvar3: u16;
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < param_1_00)
    {
        uVar3 = (param_2_00 >> 0x10);
        local_SI_109 = param_2_00;
        if (((local_SI_109 + local_6 * 4) != 0) &&
            (uVar2 = (local_SI_109 + local_6 * 4),
             local_AX_36 = 
                 pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar2 + 0xc), 0x2f),
             local_AX_36 != 0x0))
        {
            uVar2 = (local_SI_109 + local_6 * 4);
            (uVar2 + 0x1a) = 3;
            ppcVar1 = ((local_SI_109 + local_6 * 4) + 0x28);
            (**ppcVar1)();
        }
        local_6 = local_6 + 1;
    }
    return;
}



pub fn pass1_1038_58e6(param_1: u16, param_2: u16,param_1_00: u32, param_2_00: *mut astruct_1133, param_5: *mut u8, param_6: u16)

{
    let mut uVar1: i32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let BVar4: bool;
    let puVar5: *mut u32;
    let paVar6: *mut astruct_493;
    let mut extraout_DX: u16;
    let mut uVar7: u16;
    let local_SI_123: *mut astruct_1133;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let mut uVar10: u32;
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
    let temp_5fbadbcbba: *mut astruct_1134;

    local_6 = 0;
    while (local_6 < param_1_00)
    {
        uVar9 = (param_2_00 >> 0x10);
        local_SI_123 = param_2_00;
        if (((local_SI_123 + local_6 * 4) != 0) &&
            (uVar3 = (local_SI_123 + local_6 * 4),
             BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar3 + 0xc), 0x2e), BVar4 != 0))
        {
            uVar8 = (param_5 >> 0x10);
            temp_5fbadbcbba = (local_6 * 4 + param_5);
            uVar8 = (local_6 * 4 + param_5 + 2);
            local_12 = temp_5fbadbcbba.field_0xc;
            local_c = temp_5fbadbcbba.field_0x10;
            local_e = local_c;
            if (local_c == param_6)
            {
                local_e = local_c - 1;
                uVar10 = pass1_1028_bb24(*(local_SI_123 + local_6 * 4));
                puVar5 = &local_12;
                pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, puVar5), uVar10);
                uVar7 = extraout_DX;
                paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, puVar5, extraout_DX);
                if ((uVar7 | paVar6) != 0)
                {
                    uVar10 = pass1_1030_73a8(CONCAT22(uVar7, paVar6));
                    uVar1 = (uVar10 + 0x1a);
                    if (((uVar1 & 2) != 0) && ((uVar1 & 1) != 0))
                    {
                        uVar3 = (local_SI_123 + local_6 * 4);
                        (uVar3 + 0x1a) = 3;
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

pub fn pass1_1038_5860(param_1: *mut u8, param_2: u16, param_2_00: *mut u8, param_4: i32)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let puVar3: *mut u8;
    let mut uVar4: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let local_BX_19: *mut astruct_1132;
    let mut uVar5: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    if (param_4 == 0)
    {
        uVar5 = (param_1 >> 0x10);
        local_BX_19 = param_1;
        ppcVar1 = (local_BX_19.field_0xc + 0x10);
        puVar3 = param_2_00;
        (**ppcVar1)();
        uVar2 = puVar3 & 0xffff | extraout_DX << 0x10;
        local_e = 0;
        while (local_e < uVar2)
        {
            ppcVar1 = (local_BX_19.field_0xc + 4);
            uVar4 = uVar2;
            (**ppcVar1)();
            local_6._0_2_ = param_2_00;
            if ((uVar4 == local_6) &&
                (local_6._2_2_ = (param_2_00 >> 0x10), extraout_DX_00 == local_6._2_2_))
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


pub fn pass1_1038_57dc(param_1: *mut u8, param_2: *mut u8, param_3: *mut astruct_1129)

{
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    *(param_1 + param_3 * 4 + 0x1a2) =
        param_2 + (param_1 + 0x1a2 + param_3 * 4);
    return;
}

pub fn pass1_1038_5804(param_1: *mut u8, param_2: libc::c_long, param_3: *mut astruct_1130)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0x1a2) =
        (param_1 + 0x1a2 + param_3 * 4) - param_2;
    return;
}

pub fn pass1_1038_582c(param_1: *mut astruct_1131,param_2: u32)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_4: *mut astruct_1131;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    puVar1 = local_BX_4.field_0x14;
    uVar2 = &local_BX_4.field_0x16;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe {ppcVar3 = *puVar1;}
        (**ppcVar3)();
    }
    &local_BX_4.field_0x14 = param_2;
    return;
}

pub fn pass1_1038_57c0(param_1: *mut u8)

{
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x14e)), 0, 0x54);
    return;
}

pub fn pass1_1038_5770(param_1: *mut u8, param_2: libc::c_long, param_3: *mut astruct_1127)

{
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0xba) =
        (param_1 + 0xba + param_3 * 4) + param_2;
    return;
}

pub fn pass1_1038_5798(param_1: *mut u8,param_2: u32, param_3: *mut astruct_1128)

{
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0x14e) =
        (param_1 + 0x14e + param_3 * 4) + param_2;
    return;
}


pub fn pass1_1038_565e(param_1: *mut astruct_1124)

{
    let local_BX_4: *mut astruct_1124;
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut uVar2: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8;2];

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar2 = pass1_1030_8e3c(CONCAT22(unaff_SS, local_4), local_BX_4.field_0x4);
    pass1_1038_582c(param_1, uVar2);
    return CONCAT22(local_BX_4.field_0x16, local_BX_4.field_0x14);
}

pub fn pass1_1038_5694(param_1: u32,param_2: u32, param_3: *mut astruct_1125)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0x26) =
        (param_1 + 0x26 + param_3 * 4) + param_2;
    return;
}

pub fn pass1_1038_56ba(param_1: *mut u8)

{
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x26)), 0, 0x94);
    return;
}




pub fn pass1_1038_56d6(param_1: *mut u8, param_2: u16)

{
    let ppcVar1: fn();
    let uVar2: u8;
    let local_AX_14: *mut astruct_1126;
    let mut uvar3: u16;
    let paVar4: *mut astruct_493;
    let extraout_var: u32;
    let mut uVar5: u32;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let mut extraout_DX_00: i32;
    let mut uVar7: i32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_AX_14 = param_1;
    local_AX_14 = &local_AX_14.field_0xba;
    uVar9 = 0x1000;
    uVar2 = pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(local_AX_14)), 0, 0x94);
    uVar3 = CONCAT31(extraout_var, uVar2);
    if (param_2 != 0)
    {
        uVar8 = (param_1 >> 0x10);
        if (local_AX_14.field_0xc == 0)
        {
            uVar3 = 0;
            uVar6 = 0;
        }
        else
        {
            ppcVar1 = (local_AX_14.field_0xc + 0x10);
            (**ppcVar1)();
            uVar6 = extraout_DX;
        }
        _local_6 = CONCAT22(uVar6, uVar3);
        local_a = 0;
        while (local_a < _local_6)
        {
            ppcVar1 = (local_AX_14.field_0xc + 4);
            uVar5 = _local_6;
            (**ppcVar1)(uVar9, local_AX_14.field_0xc);
            uVar7 = extraout_DX_00 | uVar5;
            if (uVar7 != 0)
            {
                paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_00);
                uVar9 = 0x1030;
                pass1_1030_72d0(CONCAT22(uVar7, paVar4));
            }
            local_a = local_a + 1;
        }
    }
    return;
}


pub fn pass1_1038_53ba(param_1: *mut u8, param_2: *mut astruct_1122)

{
    let mut uVar1: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a2 + param_2 * 4) <
        (param_1 + 0x14e + param_2 * 4))
    {
        return;
    }
    return;
}

pub fn pass1_1038_540a()

{
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    return;
}





pub fn pass1_1038_5464(param_1: *mut astruct_1123)

{
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut in_AX: u16;
    let mut uVar3: i32;
    let mut uVar4: u16;
    let paVar5: *mut astruct_493;
    let mut uVar6: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut uVar7: i32;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: u16;
    let mut iVar8: i32;
    let mut iVar9: i32;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let local_42: *mut astruct_1123;
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
    let temp_3fa8023d0d: *mut astruct_1123;

    pass1_1038_56ba(param_1);
    pass1_1038_57c0(param_1);
    uVar10 = (param_1 >> 0x10);
    iVar8 = param_1;
    if ((iVar8 + 0xc) == 0)
    {
        in_AX = 0;
        uVar11 = 0;
    }
    else
    {
        ppcVar2 = ((iVar8 + 0xc) + 0x10);
        ppcVar2();
        uVar11 = extraout_DX;
    }
    _local_a = CONCAT22(uVar11, in_AX);
    local_e = 0;
    while (local_e < _local_a)
    {
        local_42 = local_e;
        uStack64 = (local_e >> 0x10);
        uVar1 = (iVar8 + 0xc);
        ppcVar2 = ((iVar8 + 0xc) + 4);
        uVar6 = _local_a;
        ppcVar2(unaff_CS, uVar1, (uVar1 >> 0x10), local_42, uStack64);
        local_12 = uVar6;
        uVar7 = extraout_DX_01 | local_12;
        local_10 = extraout_DX_01;
        if (uVar7 != 0)
        {
            unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
            paVar5 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_12, extraout_DX_01);
            local_16 = CONCAT22(uVar7, paVar5);
            local_1a = &paVar5[1].field_0x4;
            if ((&paVar5[1].field_0x6 | local_1a) == 0)
            {
                local_1c = 0;
            }
            else
            {
                local_1c = (local_1a + 4);
            }
            local_1e = 0;
            while (local_1e < local_1c)
            {
                unaff_CS = 0x1020;
                pass1_1020_bb16(local_1a,
                                CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_2e)),
                                CONCAT22(unaff_SS, &local_2a), local_1e);
                if (CONCAT22(local_2e._2_2_, local_2e) != 0)
                {
                    pass1_1038_5694(param_1, CONCAT22(local_2e._2_2_, local_2e),
                                    local_2a);
                }
                local_1e = local_1e + 1;
            }
            uVar11 = (local_16 >> 0x10);
            local_22 = (local_16 + 0x1e);
            uVar7 = (local_16 + 0x20);
            uVar3 = uVar7 | local_22;
            if (uVar3 == 0)
            {
                uVar3 = 0;
            }
            else
            {
                ppcVar2 = (local_22 + 0x10);
                ppcVar2(unaff_CS, local_22, uVar7);
            }
            local_1e = 0;
            local_1c = uVar3;
            while (local_1e < local_1c)
            {
                ppcVar2 = (local_22 + 4);
                uVar4 = local_1c;
                ppcVar2(unaff_CS, local_22, (local_22 >> 0x10), local_1e, 0);
                uVar7 = extraout_DX_00 | uVar4;
                local_2a = uVar4;
                local_28 = extraout_DX_00;
                if (uVar7 != 0)
                {
                    unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
                    paVar5 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, extraout_DX_00);
                    iVar9 = &paVar5.field_0xc * 4;
                    (iVar8 + iVar9 + 0x14e) = (iVar8 + 0x14e + iVar9) + 1;
                }
                local_1e = local_1e + 1;
            }
        }
        local_e = local_e + 1;
    }
    uVar1 = (iVar8 + 0x1f6);
    local_42 = (uVar1 >> 0x10);
    uVar6 = _local_a;
    pass1_1030_38f2(uVar1, local_42, 3);
    uVar7 = uVar6;
    uVar1 = (iVar8 + 0x1f6);
    local_42 = (uVar1 >> 0x10);
    local_6 = uVar7;
    local_4 = extraout_DX_02;
    pass1_1030_38f2(uVar1, local_42, 4);
    _local_6 = CONCAT22(local_4 + extraout_DX_03 + CARRY2(local_6, uVar7), local_6 + uVar7);
    if (_local_6 == 0)
    {
        uVar1 = (iVar8 + 0x1f6);
        local_42 = (uVar1 >> 0x10);
        pass1_1030_38f2(uVar1, local_42, 2);
        _local_6 = CONCAT22(extraout_DX_04, uVar7);
    }
    uVar1 = (iVar8 + 0x1f6);
    _local_6 = _local_6 + (uVar1 + 0x170);
    pass1_1038_5694(param_1, _local_6, (s_New_failed_in_Op__Op_1050_0020 + 4));
    return;
}



pub fn pass1_1038_45e4(param_1: *mut astruct_1109)

{
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let mut uVar4: u32;
    let mut in_AX: i32;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let mut uVar7: i32;
    let mut iVar8: i32;
    let mut iVar9: i32;
    let puVar10: *mut u8;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut uVar11: i32;
    let mut extraout_DX_03: u16;
    let local_BX_8: *mut astruct_1109;
    let mut uVar12: u16;
    let mut bVar13: bool;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar12 = (param_1 >> 0x10);
    local_BX_8 = param_1;
    uVar2 = local_BX_8.field_0x1f6;
    pass1_1030_38f2(uVar2, (uVar2 >> 0x10), 2);
    uVar2 = local_BX_8.field_0x1f6;
    uVar5 = in_AX;
    pass1_1030_38f2(uVar2, (uVar2 >> 0x10), 1);
    bVar13 = in_AX < uVar5;
    in_AX = in_AX - uVar5;
    uVar2 = local_BX_8.field_0x1f6;
    pass1_1030_38f2(uVar2, (uVar2 >> 0x10), 4);
    uVar2 = local_BX_8.field_0x1f6;
    uVar6 = uVar5;
    pass1_1030_38f2(uVar2, (uVar2 >> 0x10), 3);
    uVar11 = local_BX_8.field_0x24;
    uVar7 = uVar11 + (uVar5 - uVar6);
    uVar11 = (uVar11 >> 0xf) + ((extraout_DX_01 - extraout_DX_02) - (uVar5 < uVar6)) +
             CARRY2(uVar11, uVar5 - uVar6) + ((extraout_DX - extraout_DX_00) - bVar13) +
             CARRY2(uVar7, in_AX);
    if ((uVar11 < 0) || ((uVar11 < 1 && (uVar7 + in_AX == 0))))
    {
        iVar9 = -1;
    }
    else
    {
        iVar9 = 1;
    }
    piVar1 = &local_BX_8.field_0x24;
    unsafe{*piVar1 = *piVar1 + iVar9;}
    puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x16);
    iVar8 = puVar10;
    pass1_1038_4d6e(param_1, puVar10 & 0xffff | uVar11 << 0x10);
    _local_16 = CONCAT22(extraout_DX_03, iVar8);
    uVar4 = *_local_16;
    ppcVar3 = uVar4 + 8;
    iVar9 = iVar8;
    (**ppcVar3)(&PTR_LOOP_1050_1008, iVar8, extraout_DX_03);
    if (_local_16 != 0x0)
    {
        ppcVar3 = uVar4;
        (**ppcVar3)(&PTR_LOOP_1050_1008, iVar8, extraout_DX_03, 1);
    }
    piVar1 = &local_BX_8.field_0x24;
    unsafe{*piVar1 = *piVar1 + iVar9 * 2;}
    iVar9 = local_BX_8.field_0x24;
    if (100 < iVar9)
    {
        iVar9 = 100;
    }
    local_BX_8.field_0x24 = iVar9;
    if (iVar9 < 0)
    {
        iVar9 = 0;
    }
    local_BX_8.field_0x24 = iVar9;
    iVar9 = iVar9 / 10;
    local_1c = 0x10;
    if (iVar9 < 0xb)
    {
        local_1c = 0x14;
    }
    else
    {
        if (iVar9 < 0x15)
        {
            local_1c = 0x13;
        }
        else
        {
            if (iVar9 < 0x1f)
            {
                local_1c = 0x12;
            }
            else
            {
                if (iVar9 < 0x29)
                {
                    local_1c = 0x11;
                }
                else
                {
                    if (iVar9 < 0x33)
                    {
                        local_1c = 0x10;
                    }
                    else
                    {
                        if (iVar9 < 0x3d)
                        {
                            local_1c = 0xf;
                        }
                        else
                        {
                            if (iVar9 < 0x47)
                            {
                                local_1c = 0xe;
                            }
                            else
                            {
                                if (iVar9 < 0x51)
                                {
                                    local_1c = 0xd;
                                }
                                else
                                {
                                    if (iVar9 < 0x5b)
                                    {
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




pub fn pass1_1038_4760(param_1: *mut astruct_1109)

{
    let piVar1: *mut i32;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut uVar4: i32;
    let mut iVar5: i32;
    let puVar6: *mut u8;
    let mut uVar7: u32;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut uVar8: i32;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: u16;
    let mut iVar9: i32;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let puVar12: *mut u32;
    let uVar13: u8;
    let mut uVar14: i32;
    let mut local_22: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar10 = (param_1 >> 0x10);
    iVar9 = param_1;
    piVar1 = (iVar9 + 0x22);
    unsafe{*piVar1 = *piVar1 + (iVar9 + 0x20c);}
    puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x26);
    uVar3 = SUB42(puVar6, 0);
    pass1_1038_4d6e(param_1, puVar6 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, uVar3);
    uVar11 = SUB42(&PTR_LOOP_1050_1008, 0);
    uVar8 = extraout_DX;
    puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1a);
    uVar4 = puVar6;
    pass1_1038_4d6e(param_1, puVar6 & 0xffff | uVar8 << 0x10);
    _local_e = CONCAT22(extraout_DX_00, uVar4);
    ppcVar2 = (*_local_e + 0x10);
    uVar8 = uVar4;
    ppcVar2(&PTR_LOOP_1050_1008, uVar4, extraout_DX_00);
    uVar13 = uVar3;
    uVar14 = extraout_DX;
    if ((extraout_DX_01 | uVar8) == 0)
    {
        ppcVar2 = (*_local_a + 0x10);
        ppcVar2();
        piVar1 = (iVar9 + 0x22);
        unsafe{*piVar1 = *piVar1 + uVar8;}
    }
    else
    {
        ppcVar2 = (*_local_a + 0x10);
        ppcVar2();
        _local_16 = CONCAT22(extraout_DX_02, uVar8);
        local_1a = 0;
        while (local_1a < _local_16)
        {
            uVar7 = _local_16;
            puVar12 = _local_e;
            pass1_1030_1d7c(_local_a, local_1a);
            iVar5 = uVar7;
            uVar11 = SUB42(&PTR_LOOP_1050_1028, 0);
            pass1_1028_5a94(iVar5, extraout_DX_03, puVar12);
            if (iVar5 == 2)
            {
                if ((*_PTR_LOOP_1050_65e2 & 1) == 0) {}
                  // goto LAB_1038_485e;
            }
            else
            {
                if (iVar5 != 3)
                {
// LAB_1038_485e:
                    piVar1 = (iVar9 + 0x22);
                    unsafe{*piVar1 = *piVar1 + 1;}
                }
            }
            local_1a = local_1a + 1;
        }
    }
    if (_local_a != 0x0)
    {
        ppcVar2 = *_local_a;
        ppcVar2(uVar11, uVar3, extraout_DX, 1, uVar13, uVar14);
    }
    if (_local_e != 0x0)
    {
        ppcVar2 = *_local_e;
        ppcVar2(uVar11, uVar4, extraout_DX_00, 1);
    }
    pass1_1038_45e4(param_1);
    unsafe{
    if (0x32 < (iVar9 + 0x24))
    {
        piVar1 = (iVar9 + 0x22);
        *piVar1 = *piVar1 + -1;
    }
    if ((iVar9 + 0x24) < 0x32)
    {
        piVar1 = (iVar9 + 0x22);
        *piVar1 = *piVar1 + 1;
    }
    if ((iVar9 + 0x18) < 0xfa)
    {
        piVar1 = (iVar9 + 0x22);
        *piVar1 = *piVar1 + 2;
    }
    else
    {
        if ((iVar9 + 0x18) < 0x1c2)
        {
            piVar1 = (iVar9 + 0x22);
            *piVar1 = *piVar1 + 1;
        }
        else
        {
            if (0x225 < (iVar9 + 0x18))
            {
                if ((iVar9 + 0x18) < 0x2ee)
                {
                    piVar1 = (iVar9 + 0x22);
                    *piVar1 = *piVar1 + -1;
                }
                else
                {
                    piVar1 = (iVar9 + 0x22);
                    *piVar1 = *piVar1 + -2;
                }
            }
        }
    }
}
    iVar5 = (iVar9 + 0x22);
    if (100 < iVar5)
    {
        iVar5 = 100;
    }
    (iVar9 + 0x22) = iVar5;
    if (iVar5 < 0)
    {
        iVar5 = 0;
    }
    (iVar9 + 0x22) = iVar5;
    return;
}

pub fn pass1_1038_48e0(param_1: *mut u8, param_2: i32)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = (param_1 + 0x20e) + param_2;
    if (10 < iVar1)
    {
        iVar1 = 10;
    }
    (param_1 + 0x20e) = iVar1;
    return;
}

pub fn pass1_1038_4900(param_1: *mut u8)

{
    let piVar1: *mut i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x20e);
    unsafe {
    *piVar1 = *piVar1 + -1;
    if (*piVar1 < 0)
    {
        (param_1 + 0x20e) = 0;
    }}
    return;
}




pub fn pass1_1038_4918(param_1: *mut astruct_1110)

{
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let mut uVar3: i32;
    let paVar4: *mut astruct_493;
    let mut iVar5: i32;
    let puVar6: *mut u32;
    let mut in_DX: u16;
    let mut uVar7: u16;
    let local_BX_6: *mut astruct_1110;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut unaff_SS: u16;
    let mut uVar11: u32;
    let mut local_15e: u32;
    let mut local_15a: u16;
    let mut local_158: u16;
    let mut local_156: u16;
    let mut local_154: u32;
    let mut local_14a: [u8;4];
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

    uVar9 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    if (local_BX_6.field_0x4 != 0x4000001)
    {
        return;
    }
    uVar2 = local_BX_6.field_0x8;
    paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
    _local_6 = CONCAT22(in_DX, paVar4);
    local_a = &paVar4.field_0x10;
    uVar10 = (local_a >> 0x10);
    iVar8 = local_a;
    if ((iVar8 + 0x1c) == 0)
    {
        return;
    }
    local_e = 0;
    match (local_BX_6.field_0x20e)
    {
    1 =>{
        local_e._0_2_ = 0x1e},
    2 =>{
        local_e._0_2_ = 0x1c},
    3 =>{
        local_e._0_2_ = 0x1a},
    4 =>{
        local_e._0_2_ = 0x18},
    5 =>{
        local_e._0_2_ = 0x16},
    6 =>{
        local_e._0_2_ = 0x14},
    7 =>{
        local_e._0_2_ = 0x12},
    8 =>{
        local_e._0_2_ = 0x10},
    9 =>{
        local_e._0_2_ = 0xe},
    10 =>{
        local_e._0_2_ = 0xc},
// default:
      // goto switchD_1038_49cf_caseD_a;
    }
    local_e = local_e;
// switchD_1038_49cf_caseD_a:
    local_12 = *_PTR_LOOP_1050_65e2;
    if ((local_e != 0) &&
        (((local_12 & 0xffff |  * (_PTR_LOOP_1050_65e2 + 2) << 0x10) % local_e) == 0))
    {
        piVar1 = (iVar8 + 0x1c);
        unsafe{
        *piVar1 = *piVar1 + -1;
        piVar1 = (iVar8 + 0x1a);
        *piVar1 = *piVar1 + 1;
        }
        iVar5 = (iVar8 + 0x1a) * 6 + (iVar8 + 0x16);
        uVar10 = (iVar8 + 0x18);
        local_20 = (iVar5 + -6);
        uStack28 = (iVar5 + -2);
        local_146 = &local_20;
        puVar6 = &local_20;
        pass1_1030_64ce(_PTR_LOOP_1050_5740, puVar6, unaff_SS, local_BX_6.field_0x8, local_14a, unaff_SS);
        unsafe{
        local_1a = *puVar6;}
        uVar7 = (puVar6 + 2);
        local_15e._3_1_ = (local_1a >> 0x18);
        if (local_15e._3_1_ != '\0')
        {
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_1a, uVar7);
            uVar11 = pass1_1030_73a8(CONCAT22(uVar7, paVar4));
            uVar3 = (uVar11 >> 0x10);
            if ((uVar3 | uVar11) != 0)
            {
                iVar8 = (uVar11 + 0xc);
                if (iVar8 < 1)
                {
                    return;
                }
                if (SBORROW2(iVar8, 1))
                {
                    return;
                }
                if (8 < iVar8 + -1)
                {
                    return;
                }
            }
        }
        pass1_1028_87f0(CONCAT22(unaff_SS, &local_144), 0, 0, 0x10, &local_20, unaff_SS,
                        local_BX_6.field_0x4, local_BX_6.field_0x8);
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_144));
    }
    return;
}

pub fn pass1_1038_4b20(param_1: *mut u8, param_2: *mut u8,param_3: u32)

{
    pass1_1020_c4f4((param_1 + 0xc), param_2, (param_2 >> 0x10), param_3);
    return;
}




pub fn pass1_1038_4b40(param_1: *mut astruct_1111)

{
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut astruct_493;
    let mut uVar3: u32;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut extraout_DX_00: i32;
    let mut uVar5: i32;
    let local_BX_12: *mut astruct_1111;
    let mut uVar6: u16;
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

    uVar6 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    if (local_BX_12.field_0xc == 0)
    {
        in_AX = 0;
        uVar4 = 0;
    }
    else
    {
        ppcVar1 = (local_BX_12.field_0xc + 0x10);
        (**ppcVar1)();
        uVar4 = extraout_DX;
    }
    _local_a = CONCAT22(uVar4, in_AX);
    local_e = 0;
    while (local_e < _local_a)
    {
        ppcVar1 = (local_BX_12.field_0xc + 4);
        uVar3 = _local_a;
        (**ppcVar1)(unaff_CS, local_BX_12.field_0xc);
        uVar5 = extraout_DX_00 | uVar3;
        if (uVar5 != 0)
        {
            paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, extraout_DX_00);
            unaff_CS = 0x1030;
            pass1_1030_73a8(CONCAT22(uVar5, paVar2));
        }
        local_e = local_e + 1;
    }
    return;
}




pub fn pass1_1038_4c1a(param_1: *mut astruct_1112)

{
    let ppcVar1: fn();
    let uVar2: u8;
    let mut in_AX: u16;
    let paVar3: *mut astruct_493;
    let mut uVar4: u32;
    let extraout_var: u32;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let mut in_EDX: u32;
    let local_BX_12: *mut astruct_1112;
    let local_ES_12: *mut astruct_1112;
    let mut unaff_CS: u16;
    let mut uVar7: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_ES_12 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    uVar7 = local_BX_12.field_0xc;
    ppcVar1 = (local_BX_12.field_0xc + 0x10);
    (**ppcVar1)();
    _local_a = CONCAT22(in_EDX, in_AX);
    local_e = 0;
    while (uVar5 = in_EDX, local_e < _local_a)
    {
        ppcVar1 = (local_BX_12.field_0xc + 4);
        uVar4 = _local_a;
        (**ppcVar1)(unaff_CS, local_BX_12.field_0xc, local_e, uVar7);
        uVar6 = uVar5 | uVar4;
        in_EDX = uVar6;
        if (uVar6 != 0)
        {
            paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, uVar5);
            uVar2 = pass1_1030_6fa0(CONCAT22(in_EDX, paVar3));
            unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
            pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, uVar2), 0xe);
        }
        local_e = local_e + 1;
    }
    return;
}

pub fn pass1_1038_4cba(param_1: *mut u8)

{
    pass1_1030_38b8((param_1 + 0x1f6));
    return;
}

pub fn pass1_1038_4cd0(param_1: *mut u8,param_2: u32, param_3: u16)

{
    let local_ES_6: *mut u8;

    local_ES_6 = (param_1 >> 0x10);
    (param_1 + 0x1c) = param_3;
    (param_1 + 0x1e) = param_2;
    return;
}

pub fn pass1_1038_4cea(param_1: *mut u8,param_2: u32,param_3: u32)

{
    let mut local_ES_3: u16;

    local_ES_3 = (param_1 >> 0x10);
    param_3 = (param_1 + 0x1c);
    param_2 = (param_1 + 0x1e);
    return;
}

pub fn pass1_1038_4d0e(param_1: *mut astruct_1113, param_2: u16)

{
    let local_BX_3: *mut astruct_1113;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    local_BX_3.field_0x1a = local_BX_3.field_0x18;
    local_BX_3.field_0x18 = param_2;
    return;
}

pub fn pass1_1038_4d28(param_1: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x1fc), (param_1 + 0x1fa));
}

pub fn pass1_1038_4d3c(param_1: *mut astruct_1114,param_2: u32)

{
    let uVar1: u8;
    let mut local_register1_15: u16;
    let mut local_DX__1: u16;
    let local_BX_4: *mut astruct_1114;
    let mut uvar3: u16;
    let paVar2: *mut astruct_44;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar1 = error_check_1000_17ce(&local_BX_4.field_0x1fa);
    paVar2 = CONCAT31(_local_register1_15, uVar1);
    pass1_fn_1008_60e8(param_2);
    local_BX_4.field_0x1fa = paVar2;
    local_BX_4.field_0x1fc = local_DX__1;
    return;
}




pub fn pass1_1038_4d6e(param_1: *mut astruct_1115,param_2: u32)

{
    let puVar1: *mut u16;
    let ppcVar2: fn();
    let uVar3: u8;
    let mut in_AX: i32;
    let paVar4: *mut astruct_493;
    let extraout_var: u32;
    let in_DX: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uVar5: u16;
    let mut extraout_DX_01: i32;
    let mut uVar6: i32;
    let local_BX_49: *mut astruct_1115;
    let mut uVar7: u16;
    let mut uVar8: u32;
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
    if ((in_DX | in_AX) == 0)
    {
        in_AX = 0;
        uVar7 = 0;
    }
    else
    {
        pass1_1030_1cd8(CONCAT22(in_DX, in_AX), 5, 5);
        uVar7 = extraout_DX;
    }
    _local_6 = CONCAT22(uVar7, in_AX);
    uVar7 = (param_1 >> 0x10);
    local_BX_49 = param_1;
    if (local_BX_49.field_0xc == 0)
    {
        in_AX = 0;
        uVar5 = 0;
    }
    else
    {
        ppcVar2 = (local_BX_49.field_0xc + 0x10);
        ppcVar2();
        uVar5 = extraout_DX_00;
    }
    _local_a = CONCAT22(uVar5, in_AX);
    local_e = 0;
    loop
    {
        if (_local_a <= local_e)
        {
            return;
        }
        ppcVar2 = (local_BX_49.field_0xc + 4);
        uVar8 = _local_a;
        ppcVar2();
        uVar6 = extraout_DX_01 | uVar8;
        if (uVar6 != 0)
        {
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar8, extraout_DX_01);
            local_1a = CONCAT22(uVar6, paVar4);
            uVar3 = pass1_1030_6fa0(CONCAT22(uVar6, paVar4));
            local_1e = 0;
            loop
            {
                puVar1 = (param_2 + 4);
                let pu_var1_val = unsafe{*puVar1};
                if (pu_var1_val == local_1e || pu_var1_val < local_1e) {
                    break;
                }
                if ((param_2 + local_1e * 2) ==
                    CONCAT31(extraout_var, uVar3))
                {
                    uVar8 = pass1_1030_73a8(local_1a);
                    if ((uVar8 + 0x12) == 5)
                    {
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



pub fn pass1_1038_4e78(param_1: u32,param_2: u32)

{
    let puVar1: *mut u16;
    let ppcVar2: fn();
    let uVar3: u8;
    let mut in_AX: i32;
    let local_AX_174: *mut astruct_515;
    let extraout_var: u32;
    let in_DX: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uVar5: u16;
    let mut extraout_DX_01: i32;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut uVar4: u32;

    process_struct_1000_179c(0x18, in_DX);
    if ((in_DX | in_AX) == 0)
    {
        in_AX = 0;
        uVar7 = 0;
    }
    else
    {
        pass1_1030_1cd8(CONCAT22(in_DX, in_AX), 5, 5);
        uVar7 = extraout_DX;
    }
    _local_6 = CONCAT22(uVar7, in_AX);
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0xc) == 0)
    {
        in_AX = 0;
        uVar5 = 0;
    }
    else
    {
        ppcVar2 = ((iVar6 + 0xc) + 0x10);
        ppcVar2();
        uVar5 = extraout_DX_00;
    }
    _local_a = CONCAT22(uVar5, in_AX);
    local_e = 0;
    loop
    {
        if (_local_a <= local_e)
        {
            return;
        }
        uVar4 = _local_a;
        pass1_1030_1d58((iVar6 + 0xc));
        if ((extraout_DX_01 | uVar4) != 0)
        {
            uVar3 = pass1_1030_6fa0((uVar4 & 0xffff | extraout_DX_01 << 0x10));
            local_1a = 0;
            loop
            {
                puVar1 = (param_2 + 4);
                let pu_var1_val = unsafe{*puVar1};
                if (pu_var1_val == local_1a || pu_var1_val < local_1a) {
                    break;
                }
                if ((param_2 + local_1a * 2) ==
                    CONCAT31(extraout_var, uVar3))
                {
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




pub fn pass1_1038_4f54(param_1: *mut astruct_1116, param_2: u16)

{
    let ppcVar1: fn();
    let uVar2: u8;
    let mut in_AX: u16;
    let BVar3: bool;
    let mut uVar4: u32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut extraout_DX_00: i32;
    let local_BX_4: *mut astruct_1116;
    let mut uVar6: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar6 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0xc == 0x0)
    {
        in_AX = 0;
        uVar5 = 0;
    }
    else
    {
        ppcVar1 = (*local_BX_4.field_0xc + 0x10);
        (**ppcVar1)();
        uVar5 = extraout_DX;
    }
    _local_6 = CONCAT22(uVar5, in_AX);
    local_a = 0;
    loop
    {
        if (_local_6 <= local_a)
        {
            return;
        }
        uVar4 = _local_6;
        pass1_1030_1d58(local_BX_4.field_0xc);
        if ((extraout_DX_00 | uVar4) != 0)
        {
            uVar2 = pass1_1030_6fa0((uVar4 & 0xffff | extraout_DX_00 << 0x10));
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, uVar2), param_2);
            if (BVar3 != 0)
            {
                return;
            }
        }
        local_a = local_a + 1;
    } 
}



pub fn pass1_1038_4fd8(param_1: *mut astruct_1117, param_2: u16)

{
    let ppcVar1: fn();
    let uVar2: u8;
    let mut in_AX: u16;
    let mut uVar3: u32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut extraout_DX_00: i32;
    let local_BX_4: *mut astruct_1117;
    let mut uVar5: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0xc == 0x0)
    {
        in_AX = 0;
        uVar4 = 0;
    }
    else
    {
        ppcVar1 = (*local_BX_4.field_0xc + 0x10);
        (**ppcVar1)();
        uVar4 = extraout_DX;
    }
    _local_6 = CONCAT22(uVar4, in_AX);
    local_a = 0;
    loop
    {
        if (_local_6 <= local_a)
        {
            return;
        }
        uVar3 = _local_6;
        pass1_1030_1d58(local_BX_4.field_0xc);
        if ((extraout_DX_00 | uVar3) != 0)
        {
            uVar2 = pass1_1030_6fa0((uVar3 & 0xffff | extraout_DX_00 << 0x10));
            if (CONCAT31(extraout_var, uVar2) == param_2)
            {
                return;
            }
        }
        local_a = local_a + 1;
    } 
}




pub fn pass1_1038_5050(param_1: *mut astruct_1118, param_2: u16)

{
    let ppcVar1: fn();
    let uVar2: u8;
    let mut in_AX: u16;
    let mut uVar3: u32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut extraout_DX_00: i32;
    let local_BX_12: *mut astruct_1118;
    let mut uVar5: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar5 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    if (local_BX_12.field_0xc == 0x0)
    {
        in_AX = 0;
        uVar4 = 0;
    }
    else
    {
        ppcVar1 = (*local_BX_12.field_0xc + 0x10);
        (**ppcVar1)();
        uVar4 = extraout_DX;
    }
    _local_a = CONCAT22(uVar4, in_AX);
    local_e = 0;
    while (local_e < _local_a)
    {
        uVar3 = _local_a;
        pass1_1030_1d58(local_BX_12.field_0xc);
        if ((extraout_DX_00 | uVar3) != 0)
        {
            uVar2 = pass1_1030_6fa0((uVar3 & 0xffff | extraout_DX_00 << 0x10));
            pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, uVar2), param_2);
        }
        local_e = local_e + 1;
    }
    return;
}




pub fn pass1_1038_50e0(param_1: *mut astruct_1119, param_2: u16)

{
    let ppcVar1: fn();
    let uVar2: u8;
    let mut in_AX: u16;
    let BVar3: bool;
    let mut uVar4: u32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut extraout_DX_00: i32;
    let local_BX_12: *mut astruct_1119;
    let mut uVar6: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar6 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    if (local_BX_12.field_0xc == 0x0)
    {
        in_AX = 0;
        uVar5 = 0;
    }
    else
    {
        ppcVar1 = (*local_BX_12.field_0xc + 0x10);
        (**ppcVar1)();
        uVar5 = extraout_DX;
    }
    _local_a = CONCAT22(uVar5, in_AX);
    local_e = 0;
    while (local_e < _local_a)
    {
        uVar4 = _local_a;
        pass1_1030_1d58(local_BX_12.field_0xc);
        if ((extraout_DX_00 | uVar4) != 0)
        {
            uVar2 = pass1_1030_6fa0((uVar4 & 0xffff | extraout_DX_00 << 0x10));
            BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, uVar2), param_2);
            if (BVar3 != 0)
            {
                pass1_1030_73a8((uVar4 & 0xffff | extraout_DX_00 << 0x10));
            }
        }
        local_e = local_e + 1;
    }
    return;
}




pub fn pass1_1038_518c(param_1: *mut astruct_1120)

{
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let mut in_AX: u16;
    let paVar4: *mut astruct_493;
    let mut uVar5: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut uVar6: i32;
    let mut extraout_DX_01: i32;
    let local_BX_5: *mut astruct_1120;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let unaff_CS: u8;
    let mut bVar11: bool;
    let mut uVar12: u32;
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

    uVar9 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (local_BX_5.field_0x206 == 0)
    {
        if (local_BX_5.field_0xc == 0)
        {
            in_AX = 0;
            uVar10 = 0;
        }
        else
        {
            ppcVar3 = (local_BX_5.field_0xc + 0x10);
            (**ppcVar3)();
            uVar10 = extraout_DX;
        }
        _local_6 = CONCAT22(uVar10, in_AX);
        local_a = 0;
        while (local_a < _local_6)
        {
            uVar2 = local_BX_5.field_0xc;
            ppcVar3 = (local_BX_5.field_0xc + 4);
            uVar5 = _local_6;
            (**ppcVar3)(unaff_CS, uVar2, (uVar2 >> 0x10), local_a,
                        (local_a >> 0x10));
            uVar6 = extraout_DX_00 | uVar5;
            if (uVar6 != 0)
            {
                paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_00);
                unaff_CS = 0x30;
                uVar12 = pass1_1030_73a8(CONCAT22(uVar6, paVar4));
                iVar7 = (uVar12 + 0x12);
                uVar6 = uVar12 + 0x14;
                uVar5 = uVar6;
                local_1c = uVar12 & 0xffff0000 | uVar6;
                local_20 = 0;
                if ((iVar7 == 4) || (iVar7 == 5))
                {
                    uVar5 = local_1c;
                    local_20 = uVar5;
                }
                if (local_20 != 0)
                {
                    local_22 = 0x11;
                    while (local_22 < 0x25)
                    {
                        if (((local_BX_5.field_0x204 == 0) || (local_22 == 0x23)) || (local_22 == 0x24))
                        {
                            pass1_1038_540a(param_1, local_22);
                            iVar7 = local_22 * 4;
                            uVar10 = (local_20 >> 0x10);
                            iVar8 = local_20;
                            puVar1 = (iVar7 + iVar8 + 2);
                            unsafe{
                            bVar11 = *puVar1 < extraout_DX_01;
                            if ((bVar11 || *puVar1 == extraout_DX_01) &&
                                ((bVar11 ||
                                  (puVar1 = (iVar7 + iVar8), *puVar1 < uVar5 || *puVar1 == uVar5))))
                            {
                                pass1_1038_5770(param_1, (iVar7 + iVar8), local_22);
                            }}
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





pub fn pass1_1038_52b8(param_1: *mut astruct_1121,param_2: u32, param_3: *mut astruct_1125)

{
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut iVar3: i32;
    let paVar4: *mut astruct_493;
    let mut uVar5: u32;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let mut extraout_DX_00: i32;
    let mut uVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut unaff_CS: u16;
    let local_24: *mut astruct_1121;
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
    pass1_1038_5694(param_1, CONCAT22(-(param_2._2_2_ + (param_2 != 0)), iVar3),
                    param_3);
    if (param_3 != (s_New_failed_in_Op__Op_1050_0020 + 4))
    {
        uVar9 = (param_1 >> 0x10);
        iVar8 = param_1;
        if ((iVar8 + 0xc) == 0)
        {
            iVar3 = 0;
            uVar6 = 0;
        }
        else
        {
            ppcVar2 = ((iVar8 + 0xc) + 0x10);
            ppcVar2();
            uVar6 = extraout_DX;
        }
        _local_a = CONCAT22(uVar6, iVar3);
        local_e = 0;
        while (local_e < _local_a)
        {
            uStack30 = local_e;
            uStack28 = (local_e >> 0x10);
            uVar1 = (iVar8 + 0xc);
            uStack34 = uVar1;
            ppcVar2 = ((iVar8 + 0xc) + 4);
            uVar5 = _local_a;
            ppcVar2(unaff_CS, uStack34, (uVar1 >> 0x10), uStack30, uStack28);
            uVar7 = extraout_DX_00 | uVar5;
            if (uVar7 != 0)
            {
                paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_00);
                local_16 = CONCAT22(uVar7, paVar4);
                unaff_CS = 0x1030;
                pass1_1030_7c28(CONCAT22(uVar7, paVar4), param_3);
                local_1a = CONCAT22(uVar7, paVar4);
                if ((uVar7 | paVar4) != 0)
                {
                    if (local_1a < param_2)
                    {
                        param_2 = param_2 - local_1a;
                        local_1a = 0;
                    }
                    else
                    {
                        local_1a = CONCAT22((uVar7 - param_2._2_2_) - (paVar4 < param_2),
                                            (paVar4 - param_2));
                        param_2 = 0;
                    }
                    uStack30 = (local_1a >> 0x10);
                    unaff_CS = 0x1030;
                    pass1_1030_7d1c(local_16, local_1a, CONCAT22(param_3, uStack30));
                    if (param_2 == 0)
                    {
                        return;
                    }
                }
            }
            local_e = local_e + 1;
        }
    }
    return;
}



pub fn pass1_1038_3f38(param_1: *mut u32, param_2: *mut u32,param_3: u32)

{
    let ppcVar1: fn();
    let uVar2: u8;
    let paVar3: *mut astruct_493;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut uVar4: i32;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let local_6: *mut astruct_1100;
    let mut local_4: u16;

    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_3, (param_3 >> 0x10));
    _local_6 = CONCAT22(in_DX, paVar3);
    uVar4 = in_DX;
    uVar2 = pass1_1028_b58e(CONCAT22(in_DX, paVar3));
    uVar7 = (CONCAT31(extraout_var, uVar2) + 4);
    unsafe{ppcVar1 = (*param_1 + 0x18);}
    (**ppcVar1)(&PTR_LOOP_1050_1028, param_1, uVar7);
    uVar8 = 0;
    uVar6 = 0;
    unsafe{ppcVar1 = (*param_2 + 8);}
    puVar5 = param_2;
    (**ppcVar1)();
    pass1_1030_73ee((CONCAT31(extraout_var, uVar2) & 0xffff | uVar4 << 0x10),
                    (param_2 + 4));
    ppcVar1 = (*_local_6 + 0x58);
    (**ppcVar1)(0x1030, paVar3, in_DX, param_2, puVar5, uVar6, uVar7, uVar8);
    return;
}



pub fn pass1_1038_3fb0(param_1: *mut u8)

{
    let mut u_var1: u32;

    uVar1 = (param_1 + 0x200);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    return;
}




pub fn pass1_1038_3fca(param_1: u32)

{
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut in_AX: i32;
    let local_AX_84: *mut astruct_1103;
    let local_AX_140: *mut astruct_1102;
    let paVar3: *mut astruct_493;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut uVar4: i32;
    let local_BX_5: *mut astruct_1101;
    let mut iVar5: i32;
    let mut unaff_SI: u16;
    let mut iVar6: i32;
    let puVar7: *mut u8;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let mut uVar10: u32;
    let ppVar11: *mut pass1_struct_1;
    let uVar12: u8;
    let uVar13: u8;
    let uVar14: u8;
    let uVar15: u8;
    let mut local_3a: u32;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: [u8;2];
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

    puVar7 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (local_BX_5.field_0xc == 0)
    {
        in_AX = 0;
        uVar4 = 0;
    }
    else
    {
        ppcVar2 = (local_BX_5.field_0xc + 0x10);
        ppcVar2();
        uVar4 = extraout_DX;
    }
    _local_6 = CONCAT22(uVar4, in_AX);
    g_u16_ptr_1050_5f2e = (uVar4 | in_AX);
    if (g_u16_ptr_1050_5f2e != 0x0)
    {
        if (__g_astruct_94_ptr_1 == 0)
        {
            struct_fn_1000_160a();
            _g_astruct_94_ptr_1._0_1_ = in_AX;
        }
        else
        {
        }
        local_AX_84 = (local_6 << 2);
        alloc_mem_1000_1708(local_AX_84, 0, 1, _g_astruct_94_ptr_1._0_1_, g_u16_ptr_1050_5f2e);
        _local_a = CONCAT22(g_u16_ptr_1050_5f2e, local_AX_84);
        if (__g_astruct_94_ptr_1 == 0)
        {
            struct_fn_1000_160a();
            _g_astruct_94_ptr_1._0_1_ = SUB21(local_AX_84, 0);
        }
        else
        {
        }
        local_AX_140 = (local_6 << 2);
        uVar9 = 0x1000;
        alloc_mem_1000_1708(local_AX_140, 0, 1, _g_astruct_94_ptr_1._0_1_, g_u16_ptr_1050_5f2e);
        _local_e = CONCAT22(g_u16_ptr_1050_5f2e, local_AX_140);
        local_16 = 0;
        while (local_16 < _local_6)
        {
            uVar1 = local_BX_5.field_0xc;
            ppcVar2 = (local_BX_5.field_0xc + 4);
            uVar10 = _local_6;
            ppcVar2(uVar9, uVar1, (uVar1 >> 0x10), local_16,
                        (local_16 >> 0x10));
            local_12 = uVar10;
            uVar4 = extraout_DX_00 | local_12;
            local_10 = extraout_DX_00;
            if (uVar4 != 0)
            {
                paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_12, extraout_DX_00);
                iVar5 = local_16 * 4;
                uVar8 = (_local_a >> 0x10);
                iVar6 = _local_a;
                (iVar5 + iVar6) = paVar3;
                (iVar5 + iVar6 + 2) = uVar4;
                uVar9 = 0x1030;
                uVar10 = pass1_1030_73a8(CONCAT22(uVar4, (iVar5 + iVar6)));
                uVar8 = (_local_e >> 0x10);
                (_local_e + iVar5) = uVar10;
                (_local_e + iVar5 + 2) = (uVar10 >> 0x10);
            }
            local_16 = local_16 + 1;
        }
        local_16 = 0;
        while (local_16 < _local_6)
        {
            uVar9 = (_local_e >> 0x10);
            iVar5 = _local_e;
            if (((local_16 * 4 + iVar5) != 0) &&
                (uVar1 = (local_16 * 4 + iVar5), (uVar1 + 0x1a) = 0, uVar1 = (local_16 * 4 + iVar5), (uVar1 + 0x12) == 5))
            {
                pass1_1028_bdac((local_16 * 4 + iVar5), 6);
            }
            local_16 = local_16 + 1;
        }
        local_BX_5.field_0x204 = 0;
        ppVar11 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
        local_1e = (ppVar11 >> 0x10);
        local_1a = ppVar11;
        local_1c = u16_1050_13ae;
        if (u16_1050_13ae == 1)
        {
            local_BX_5.field_0x204 = 1;
        }
        local_18 = local_1e;
        pass1_1038_5a96(local_BX_5, puVar7, _local_6, _local_e);
        pass1_1038_5cc6(param_1, _local_6, _local_e, _local_a, 0, 2);
        pass1_1038_5b3c(local_BX_5, puVar7, _local_6, _local_e);
        pass1_1038_5cc6(param_1, _local_6, _local_e, _local_a, 0, 1);
        uVar14 = SUB21(local_22, 0);
        uVar15 = (local_22 >> 8);
        uVar12 = SUB21(&local_24, 0);
        uVar13 = (&local_24 >> 8);
        uVar1 = local_BX_5.field_0x8;
        uVar9 = unaff_SS;
        local_20 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        pass1_1030_5b1c(CONCAT22(local_1e, local_20), CONCAT22(unaff_SS, CONCAT11(uVar13, uVar12)),
                        CONCAT22(uVar9, CONCAT11(uVar15, uVar14)));
        local_26 = 1;
        while (local_26 <= local_24)
        {
            pass1_1038_58e6(local_BX_5, puVar7, _local_6, _local_e, _local_a,
                            local_26);
            pass1_1038_5cc6(param_1, _local_6, _local_e, _local_a, local_26, 3);
            local_26 = local_26 + 1;
        }
        pass1_1038_5a16(local_BX_5, puVar7, _local_6, _local_e);
        local_16 = 0;
        while (local_16 < _local_6)
        {
            uVar9 = (_local_e >> 0x10);
            iVar5 = _local_e;
            if (((local_16 * 4 + iVar5) != 0) &&
                (uVar1 = (local_16 * 4 + iVar5), (uVar1 + 0x12) != 5))
            {
                uVar1 = (local_16 * 4 + iVar5);
                ppcVar2 = ((local_16 * 4 + iVar5) + 0x28);
                ppcVar2(0x1030, uVar1, (uVar1 >> 0x10));
            }
            local_16 = local_16 + 1;
        }
        error_check_1000_17ce(_local_a);
        error_check_1000_17ce(_local_e);
    }
    return;
}




pub fn pass1_1038_42cc(param_1: *mut astruct_1104)

{
    let paVar1: *mut astruct_872;
    let ppcVar2: fn();
    let mut bVar3: bool;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: i32;
    let puVar7: *mut u8;
    let mut uVar8: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let local_BX_4: *mut astruct_1104;
    let local_ES_4: *mut astruct_1104;
    let mut uVar9: u16;
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
    if (local_BX_4.field_0x1f6 == 0x0)
    {
        return;
    }
    uVar9 = SUB42(&PTR_LOOP_1050_1008, 0);
    puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2d);
    uVar4 = SUB42(puVar7, 0);
    pass1_1038_4d6e(param_1, puVar7 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, uVar4);
    ppcVar2 = (*_local_a + 0x10);
    uVar5 = uVar4;
    ppcVar2(&PTR_LOOP_1050_1008, uVar4, extraout_DX);
    _local_12 = CONCAT22(extraout_DX_00, uVar5);
    bVar3 = false;
    local_18 = 0;
    while (local_18 < _local_12)
    {
        uVar9 = 0x1030;
        uVar8 = _local_12;
        pass1_1030_1d7c(_local_a, local_18, (local_18 >> 0x10));
        uVar6 = uVar8;
        if ((extraout_DX_01 | uVar6) != 0)
        {
            ppcVar2 = ((uVar8 & 0xffff | extraout_DX_01 << 0x10) +
                                0x50);
            ppcVar2();
            if (uVar6 != 0)
            {
                bVar3 = true;
            }
        }
        local_18 = local_18 + 1;
    }
    if (bVar3)
    {
        paVar1 = local_BX_4.field_0x1f6;
        (paVar1 + 0x1aa) = 0;
    }
    else
    {
        paVar1 = local_BX_4.field_0x1f6;
        uVar9 = 0x1030;
        pass1_1030_38b8(paVar1, (paVar1 >> 0x10));
        if ((extraout_DX_02 | _local_12) != 0)
        {
            uVar9 = 0x1030;
            pass1_1030_326a(local_BX_4.field_0x1f6);
        }
    }
    if (_local_a != 0x0)
    {
        ppcVar2 = *_local_a;
        ppcVar2(uVar9, uVar4, extraout_DX, 1);
    }
    return;
}




pub fn pass1_1038_43cc(param_1: *mut astruct_1106,param_2: u32, param_3: u16)

{
    let ppcVar1: fn();
    let mut in_AX: i32;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut uVar4: i32;
    let puVar5: *mut u8;
    let mut uVar6: u32;
    let extraout_DL: u8;
    let mut in_DX: i32;
    let mut uVar7: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let local_SI_56: *mut astruct_1105;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;

    if (param_3 == 5)
    {
        pass1_1038_4900(CONCAT22(param_2, param_1));
        return;
    }
    pass1_1038_53ba(CONCAT22(param_2, param_1), param_3);
    if ((in_DX != 0) || (in_AX != 0))
    {
        local_SI_56 = (param_3 * 4);
        uVar4 = (param_1 + local_SI_56 + 0x14e);
        uVar7 = param_2._2_2_ >> 0xf;
        iVar8 = ((param_1 + local_SI_56 + 0x150) - uVar7) -
                (uVar4 < param_2._2_2_);
        (param_1 + local_SI_56 + 0x14e) = uVar4 - param_2._2_2_;
        (param_1 + local_SI_56 + 0x150) = iVar8;
        if (iVar8 < 0)
        {
            (param_1 + local_SI_56 + 0x14e) = 0;
        }
        uVar9 = SUB42(&PTR_LOOP_1050_1008, 0);
        puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        uVar2 = SUB42(puVar5, 0);
        pass1_1038_4e78(CONCAT22(param_2, param_1),
                        puVar5 & 0xffff | uVar7 << 0x10);
        _local_e = CONCAT22(extraout_DX, uVar2);
        ppcVar1 = (*_local_e + 0x10);
        uVar3 = uVar2;
        (**ppcVar1)(&PTR_LOOP_1050_1008, uVar2, extraout_DX);
        _local_12 = CONCAT22(extraout_DX_00, uVar3);
        local_16 = 0;
        while (local_16 < _local_12)
        {
            uVar6 = _local_12;
            pass1_1030_1d7c(uVar2, extraout_DX, local_16, (local_16 >> 0x10));
            param_2._2_2_ = (param_2 >> 0x10);
            uVar3 = uVar6;
            while (uVar4 = uVar6, param_2._2_2_ != 0)
            {
                pass1_1030_cf78(uVar3, extraout_DL, param_3);
                uVar6 = uVar4;
                if (uVar4 == 0) {
                    break;}
                param_2._2_2_ = param_2._2_2_ - 1;
            }
            param_2 = param_2._2_2_ << 0x10;
            uVar9 = 0x1030;
            if (param_2._2_2_ == 0) {
                break;}
            local_16 = local_16 + 1;
        }
        if (_local_e != 0x0)
        {
            ppcVar1 = *_local_e;
            (**ppcVar1)(uVar9, uVar2, extraout_DX, 1);
            return;
        }
    }
    return;
}




pub fn pass1_1038_44d8(param_1: *mut astruct_998, param_2: *mut u8, uparam_3: i32, param_3_00: *mut u8)

{
    let ppcVar1: fn();
    let paVar2: *mut astruct_44;
    let mut in_AX: i32;
    let local_AX_121: *mut u8;
    let puVar3: *mut u8;
    let mut uVar4: i32;
    let puVar5: *mut u8;
    let mut uVar6: u32;
    let mut in_DX: i32;
    let mut uVar7: i32;
    let local_DX_128: *mut u8;
    let local_DX_145: *mut u8;
    let extraout_DX: *mut astruct_44;
    let local_SI_56: *mut astruct_1107;
    let mut iVar8: i32;
    let local_CS_115: *mut u8;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let local_e: *mut astruct_1108;
    let mut local_c: u16;

    if (param_3_00 == (&PTR_DAT_0005_0000_1050_0004 + 1))
    {
        pass1_1038_4900(CONCAT22(param_2, param_1));
        return;
    }
    pass1_1038_53ba(CONCAT22(param_2, param_1), param_3_00);
    if ((in_DX != 0) || (in_AX != 0))
    {
        local_SI_56 = (param_3_00 * 4);
        uVar4 = (param_1 + local_SI_56 + 0x14e);
        uVar7 = param_3 >> 0xf;
        iVar8 = ((param_1 + local_SI_56 + 0x150) - uVar7) - (uVar4 < param_3);
        (param_1 + local_SI_56 + 0x14e) = uVar4 - param_3;
        (param_1 + local_SI_56 + 0x150) = iVar8;
        if (iVar8 < 0)
        {
            (param_1 + local_SI_56 + 0x14e) = 0;
        }
        local_CS_115 = &PTR_LOOP_1050_1008;
        puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        local_AX_121 = puVar5;
        pass1_1038_4e78(CONCAT22(param_2, param_1), puVar5 & 0xffff | uVar7 << 0x10);
        _local_e = CONCAT22(local_DX_128, local_AX_121);
        ppcVar1 = (*_local_e + 0x10);
        puVar3 = local_AX_121;
        (**ppcVar1)(&PTR_LOOP_1050_1008, local_AX_121, local_DX_128);
        _local_12 = CONCAT22(local_DX_145, puVar3);
        local_16 = 0;
        while (local_16 < _local_12)
        {
            uVar6 = _local_12;
            pass1_1030_1d7c(local_AX_121, local_DX_128, local_16, (local_16 >> 0x10));
            paVar2 = uVar6;
            while (uVar4 = uVar6, param_3 != 0)
            {
                pass1_1030_d00c(paVar2, extraout_DX, param_3_00);
                uVar6 = uVar4;
                if (uVar4 == 0) {
                    break;}
                param_3 = param_3 - 1;
            }
            local_CS_115 = 0x1030;
            if (param_3 == 0) {
                break;}
            local_16 = local_16 + 1;
        }
        if (_local_e != 0x0)
        {
            ppcVar1 = *_local_e;
            (**ppcVar1)(local_CS_115, local_AX_121, local_DX_128, 1);
            return;
        }
    }
    return;
}


pub fn pass1_1038_3698(param_1: *mut astruct_1091)

{
    let piVar1: *mut i32;
    astruct_1093 **ppaVar2;
    astruct_1094 **ppaVar3;
    let paVar4: *mut astruct_1093;
    let mut uVar5: u32;
    let ppcVar6: fn();
    let mut in_AX: u16;
    let mut uVar7: u16;
    let BVar8: bool;
    let mut uVar9: i32;
    let local_AX_338: *mut astruct_1092;
    let local_AX_462: *mut astruct_1092;
    let mut uVar10: i32;
    let mut uVar11: u32;
    let mut in_DX: u16;
    let mut uVar13: i32;
    let local_DX_239: *mut astruct_1094;
    let local_DX_299: *mut astruct_1093;
    let mut uVar14: i32;
    let mut uVar15: u32;
    let local_BX_4: *mut astruct_1091;
    let local_ES_4: *mut astruct_1091;
    let mut uVar16: u32;
    let uVar17: u8;
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
    let mut uVar12: u32;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x214 == 0)
    {
        return;
    }
    uVar5 = local_BX_4.field_0x1f6;
    pass1_1030_38b8(uVar5, (uVar5 >> 0x10));
    _local_6 = CONCAT22(in_DX, in_AX);
    _local_6 = _local_6 - &local_BX_4.field_0x216;
    if (0 < _local_6)
    {
        _local_6 = _local_6 + 3;
        local_a = _local_6 / 5;
        uVar15 = _local_6 % 5;
        if (local_BX_4.field_0xc == 0)
        {
            uVar7 = 0;
            uVar15 = 0;
        }
        else
        {
            uVar5 = local_BX_4.field_0xc;
            ppcVar6 = (local_BX_4.field_0xc + 0x10);
            uVar11 = local_a;
            (**ppcVar6)(0x1030, uVar5, (uVar5 >> 0x10));
            uVar7 = uVar11;
        }
        _local_e = CONCAT22(uVar15, uVar7);
        local_12 = 0;
        while (uVar14 = uVar15, uVar11 = _local_e, local_12 < _local_e)
        {
            uVar5 = local_BX_4.field_0xc;
            uVar12 = _local_e;
            pass1_1030_1d7c(uVar5, (uVar5 >> 0x10), local_12, (local_12 >> 0x10));
            uVar10 = uVar12;
            uVar15 = (uVar14 | uVar10);
            if ((uVar14 | uVar10) != 0)
            {
                BVar8 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar10 + 0xc), 4);
                uVar11 = BVar8;
                if (BVar8 != 0)
                {
                    uVar16 = pass1_1028_678c(uVar12 & 0xffff | uVar14 << 0x10, 0xf);
                    uVar9 = (uVar16 >> 0x10);
                    uVar13 = uVar9 | (uVar16 & 0xffff);
                    uVar15 = uVar13;
                    uVar11 = uVar16 & 0xffff;
                    if (uVar13 != 0)
                    {
                        uVar17 = (uVar14 >> 8);
                        if (local_a < uVar16)
                        {
                            uVar9 = local_a;
                            pass1_1028_6356(CONCAT13(uVar17, CONCAT12(uVar14, uVar10)), 0xf, uVar9, local_a._2_2_);
                            uVar14 = uVar9 * 5;
                            local_DX_239 = (local_a._2_2_ * 5 + CARRY2(uVar9, uVar9) * 2 +
                                                            CARRY2(uVar9 * 2, uVar9 * 2) + CARRY2(uVar9 * 4, uVar9));
                            uVar15 = ZEXT24(local_DX_239);
                            ppaVar2 = &local_BX_4.field_0x216;
                            paVar4 = *ppaVar2;
                            *ppaVar2 = *ppaVar2 + uVar14;
                            ppaVar3 = &local_BX_4.field_0x218;
                            *ppaVar3 = local_DX_239 + (*ppaVar3 + CARRY2(paVar4, uVar14));
                            local_a = 0;
                            uVar11 = uVar14;
                        }
                        else
                        {
                            uVar13 = uVar16;
                            pass1_1028_6356(CONCAT13(uVar17, CONCAT12(uVar14, uVar10)), 0xf, uVar13, uVar9);
                            local_DX_299 = (uVar9 * 5 + CARRY2(uVar13, uVar13) * 2 +
                                                            CARRY2(uVar13 * 2, uVar13 * 2) + CARRY2(uVar13 * 4, uVar13));
                            uVar15 = ZEXT24(local_DX_299);
                            ppaVar2 = &local_BX_4.field_0x216;
                            paVar4 = *ppaVar2;
                            *ppaVar2 = *ppaVar2 + uVar13 * 5;
                            ppaVar2 = &local_BX_4.field_0x218;
                            *ppaVar2 = local_DX_299 + (*ppaVar2 + CARRY2(paVar4, uVar13 * 5));
                            local_a = local_a - uVar16;
                            uVar11 = uVar16;
                        }
                    }
                }
                uVar14 = uVar15;
                if (local_a == 0) {
                    break;
                }
            }
            local_12 = local_12 + 1;
        }
        local_AX_338 = uVar11;
        uVar5 = local_BX_4.field_0x1f6;
        pass1_1030_38b8(uVar5, (uVar5 >> 0x10));
        _local_6 = CONCAT22(uVar14, local_AX_338);
        _local_6 = _local_6 - &local_BX_4.field_0x216;
        local_4 = (_local_6 >> 0x10);
        if ((local_4 | local_6) != 0)
        {
            _local_20 = _local_6 / local_BX_4.field_0x214;
            if (_local_20 < 1)
            {
                _local_20 = 1;
            }
            uVar5 = local_BX_4.field_0x1f6;
            pass1_1030_375a(uVar5, (uVar5 >> 0x10), 0, _local_20,
                            (_local_20 >> 0x10));
        }
    }
    piVar1 = &local_BX_4.field_0x214;
    unsafe{*piVar1 = *piVar1 + -1;}
    return;
}



pub fn pass1_1038_387e(param_1: *mut astruct_1095, param_2: u16, param_3: u16,param_4: u32)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let uVar3: u8;
    let mut uVar4: u16;
    let paVar5: *mut astruct_987;
    let mut uVar6: i32;
    let extraout_var: u32;
    let mut uVar7: u32;
    let extraout_var_00: u32;
    let mut uVar8: u32;
    let mut extraout_DX: u16;
    let mut uVar9: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: i32;
    let local_BX_14: *mut astruct_1095;
    let local_ES_14: *mut astruct_1095;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 != param_3)
    {
        local_BX_14 = param_1;
        local_ES_14 = (param_1 >> 0x10);
        if (param_2 < param_3)
        {
            local_c = param_3 - param_2;
            if ((local_BX_14.field_0x210 == 0) ||
                (uVar2 = local_BX_14.field_0x210, (uVar2 + 10) == 0))
            {
                if (local_BX_14.field_0xc == 0x0)
                {
                    uVar4 = 0;
                    uVar9 = 0;
                }
                else
                {
                    ppcVar1 = (*local_BX_14.field_0xc + 0x10);
                    uVar4 = local_c;
                    (**ppcVar1)();
                    uVar9 = extraout_DX;
                }
                _local_6 = CONCAT22(uVar9, uVar4);
                local_a = 0;
                while (local_a < _local_6)
                {
                    uVar7 = _local_6;
                    pass1_1030_1d58(local_BX_14.field_0xc);
                    if (((extraout_DX_00 | uVar7) != 0) &&
                        (uVar3 = pass1_1030_6fa0((uVar7 & 0xffff | extraout_DX_00 << 0x10)),
                         CONCAT31(extraout_var, uVar3) == 0xb))
                    {
                        pass1_1030_7c50(CONCAT13((extraout_DX_00 >> 8),
                                                 CONCAT12(extraout_DX_00, uVar7)),
                                        local_c,
                                        4);
                        return;
                    }
                    local_a = local_a + 1;
                }
            }
            else
            {
                uVar2 = local_BX_14.field_0x210;
                uVar7 = (uVar2 + 10);
                local_a = 0;
                while (local_a < uVar7)
                {
                    uVar2 = local_BX_14.field_0x210;
                    uVar8 = uVar7;
                    pass1_1030_1312(uVar2, (uVar2 >> 0x10), local_a,
                                    (local_a >> 0x10));
                    paVar5 = uVar8;
                    if ((((extraout_DX_01 | paVar5) != 0) &&
                         (pass1_1030_cc44(paVar5, extraout_DX_01, local_c, param_4, 4),
                          paVar5 != 0x0)) &&
                        (local_c = local_c - paVar5, local_c == 0))
                    {
                        return;
                    }
                    local_a = local_a + 1;
                }
            }
        }
        else
        {
            local_16 = param_2 - param_3;
            if ((local_BX_14.field_0x210 == 0) ||
                (uVar2 = local_BX_14.field_0x210, (uVar2 + 10) == 0))
            {
                if (local_BX_14.field_0xc == 0x0)
                {
                    uVar4 = 0;
                    uVar9 = 0;
                }
                else
                {
                    ppcVar1 = (*local_BX_14.field_0xc + 0x10);
                    uVar4 = local_16;
                    (**ppcVar1)();
                    uVar9 = extraout_DX_02;
                }
                _local_6 = CONCAT22(uVar9, uVar4);
                local_a = 0;
                while (local_a < _local_6)
                {
                    uVar7 = _local_6;
                    pass1_1030_1d58(local_BX_14.field_0xc);
                    if (((extraout_DX_03 | uVar7) != 0) &&
                        (uVar3 = pass1_1030_6fa0((uVar7 & 0xffff | extraout_DX_03 << 0x10)),
                         CONCAT31(extraout_var_00, uVar3) == 0xb))
                    {
                        pass1_1030_6e9c(CONCAT13((extraout_DX_03 >> 8),
                                                 CONCAT12(extraout_DX_03, uVar7)),
                                        local_16, 4);
                        return;
                    }
                    local_a = local_a + 1;
                }
            }
            else
            {
                uVar2 = local_BX_14.field_0x210;
                uVar7 = (uVar2 + 10);
                local_a = 0;
                while (local_a < uVar7)
                {
                    uVar2 = local_BX_14.field_0x210;
                    uVar8 = uVar7;
                    pass1_1030_1312(uVar2, (uVar2 >> 0x10), local_a,
                                    (local_a >> 0x10));
                    uVar6 = uVar8;
                    if ((extraout_DX_04 | uVar6) != 0)
                    {
                        pass1_1030_ce72(extraout_DX_04 << 0x10 | uVar8 & 0xffff, local_16, param_4, 4);
                        local_16 = local_16 - uVar6;
                        if (local_16 == 0)
                        {
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



pub fn pass1_1038_3aa6(param_1: *mut astruct_1096)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let uVar3: u8;
    let mut in_AX: u16;
    let mut uVar4: u32;
    let extraout_var: u32;
    let mut uVar5: u32;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let local_BX_9: *mut astruct_1096;
    let mut uVar7: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar7 = (param_1 >> 0x10);
    local_BX_9 = param_1;
    if ((local_BX_9.field_0x210 == 0) ||
        (uVar2 = local_BX_9.field_0x210, (uVar2 + 10) == 0))
    {
        if (local_BX_9.field_0xc == 0x0)
        {
            in_AX = 0;
            uVar6 = 0;
        }
        else
        {
            ppcVar1 = (*local_BX_9.field_0xc + 0x10);
            (**ppcVar1)();
            uVar6 = extraout_DX;
        }
        _local_8 = CONCAT22(uVar6, in_AX);
        local_c = 0;
        while (local_c < _local_8)
        {
            uVar4 = _local_8;
            pass1_1030_1d58(local_BX_9.field_0xc);
            if (((extraout_DX_00 | uVar4) != 0) &&
                (uVar3 = pass1_1030_6fa0((uVar4 & 0xffff | extraout_DX_00 << 0x10)),
                 CONCAT31(extraout_var, uVar3) == 0xb))
            {
                pass1_1030_6b86(uVar4 & 0xffff | extraout_DX_00 << 0x10);
                return;
            }
            local_c = local_c + 1;
        }
    }
    else
    {
        uVar2 = local_BX_9.field_0x210;
        uVar4 = (uVar2 + 10);
        local_c = 0;
        while (local_c < uVar4)
        {
            uVar2 = local_BX_9.field_0x210;
            uVar5 = uVar4;
            pass1_1030_1312(uVar2, (uVar2 >> 0x10), local_c, (local_c >> 0x10));
            if ((extraout_DX_01 | uVar5) != 0)
            {
                pass1_1030_ce2e(uVar5, extraout_DX_01, 4);
            }
            local_c = local_c + 1;
        }
    }
    return;
}



pub fn pass1_1038_3ba0(param_1: *mut astruct_1097)

{
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let puVar4: *mut u32;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let mut uVar7: i32;
    let mut uVar8: i32;
    let puVar9: *mut u8;
    let mut uVar10: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let struct_a: *mut astruct_199;
    let paVar11: *mut astruct_199;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: i32;
    let local_BX_5: *mut astruct_1097;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_14: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar12 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    puVar1 = local_BX_5.field_0x210;
    uVar7 = &local_BX_5.field_0x212;
    if ((uVar7 | puVar1) != 0)
    {
        unsafe{ppcVar2 = *puVar1;}
        ppcVar2();
        uVar7 = extraout_DX;
    }
    puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
    pass1_1038_4d6e(param_1, puVar9 & 0xffff | uVar7 << 0x10);
    uVar5 = puVar9 & 0xffff;
    puVar4 = (uVar5 | extraout_DX_00 << 0x10);
    unsafe{ppcVar2 = (*puVar4 + 0x10);}
    ppcVar2(&PTR_LOOP_1050_1008, puVar9, extraout_DX_00);
    uVar7 = puVar9;
    uVar6 = puVar9 & 0xffff | ZEXT24(struct_a) << 0x10;
    if ((struct_a == 0x0) && (uVar7 < 5))
    {
        uVar7 = 5;
    }
    uVar7 = uVar7 + 1;
    uVar13 = 0x1000;
    paVar11 = struct_a;
    uVar8 = uVar7;
    process_struct_1000_179c(0x1c, struct_a);
    if ((paVar11 | uVar8) == 0)
    {
        &local_BX_5.field_0x210 = 0;
    }
    else
    {
        uVar13 = 0x1030;
        pass1_1030_11aa(CONCAT22(paVar11, uVar8), 5, uVar7, uVar7 >> 0xf);
        local_BX_5.field_0x210 = uVar7;
        &local_BX_5.field_0x212 = extraout_DX_01;
    }
    uVar3 = &local_BX_5.field_0x210;
    (uVar3 + 0x1a) = 0;
    local_14 = 0;
    while (local_14 < uVar6)
    {
        uVar10 = uVar6;
        pass1_1030_1d7c(puVar4, local_14, (local_14 >> 0x10));
        if ((extraout_DX_02 | uVar10) != 0)
        {
            pass1_1030_1358(&local_BX_5.field_0x210, uVar10, extraout_DX_02, local_14 + 1);
        }
        uVar13 = 0x1030;
        local_14 = local_14 + 1;
    }
    if (puVar4 != 0x0)
    {
        unsafe{ppcVar2 = *puVar4;}
        ppcVar2(uVar13, uVar5, extraout_DX_00, 1);
    }
    return;
}




pub fn pass1_1038_3cc0(param_1: u32, param_2: u16, param_3: u16, param_3: u16_00)

{
    let lVar1: u32;
    let ppcVar2: fn();
    let uVar3: u8;
    let mut uVar4: u16;
    let puVar5: *mut u32;
    let extraout_AH: u8;
    let puVar6: *mut u8;
    let mut uVar7: u32;
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
    let mut uVar8: i32;
    let puVar9: *mut u32;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u32;
    let uVar13: u8;
    let uVar14: u8;
    let uVar15: u8;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let local_1a: *mut astruct_1099;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let local_a: *mut astruct_1098;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3_00 == 0x1e)
    {
        uVar11 = SUB42(&PTR_LOOP_1050_1008, 0);
        puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x27);
        puVar9 = puVar6;
        pass1_1038_4e78(param_1, puVar6 & 0xffff | in_DX << 0x10);
        _local_a = CONCAT22(extraout_DX_01, puVar9);
        ppcVar2 = (*_local_a + 0x10);
        puVar5 = puVar9;
        ppcVar2(&PTR_LOOP_1050_1008, puVar9, extraout_DX_01);
        _local_e = CONCAT22(extraout_DX_02, puVar5);
        local_12 = 0;
        while (local_12 < _local_e)
        {
            uVar7 = _local_e;
            pass1_1030_1d7c(puVar9, extraout_DX_01, local_12, (local_12 >> 0x10));
            if ((extraout_DX_03 | uVar7) != 0)
            {
                uVar12 = pass1_1030_bfb8((uVar7 & 0xffff | extraout_DX_03 << 0x10));
                uVar8 = (uVar12 >> 0x10) | uVar12;
                if (uVar8 != 0)
                {
                    uVar3 = pass1_1028_b58e((uVar7 & 0xffff | extraout_DX_03 << 0x10));
                    if (CONCAT22(param_3, param_2) <= uVar12)
                    {
                        uVar11 = 0x1030;
                        pass1_1030_7ddc(CONCAT31(extraout_var, uVar3) & 0xffff | uVar8 << 0x10,
                                        CONCAT13((param_3 >> 8), CONCAT12(param_3, param_2)), 0x1e);
                        break;
                    }
                    pass1_1030_7ddc(CONCAT31(extraout_var, uVar3) & 0xffff | uVar8 << 0x10, uVar12, 0x1e);
                    lVar1 = CONCAT22(param_3, param_2) - uVar12;
                    param_2 = lVar1;
                    param_3 = (lVar1 >> 0x10);
                }
            }
            uVar11 = 0x1030;
            local_12 = local_12 + 1;
        }
        _local_1a = _local_a;
        uVar10 = extraout_DX_01;
        if (_local_a == 0x0)
        {
            return;
        }
    }
    else
    {
        if (param_3_00 != 0x21)
        {
            uVar11 = SUB42(&PTR_LOOP_1050_1008, 0);
            puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 3);
            uVar4 = SUB42(puVar6, 0);
            pass1_1038_4e78(param_1, puVar6 & 0xffff | in_DX << 0x10);
            _local_1a = CONCAT22(extraout_DX, uVar4);
            ppcVar2 = (*_local_1a + 0x10);
            uVar10 = uVar4;
            ppcVar2(&PTR_LOOP_1050_1008, uVar4, extraout_DX);
            _local_16 = CONCAT22(extraout_DX_00, uVar10);
            local_12 = 0;
// LAB_1038_3e9c:
            if (local_12 < _local_16)
            {
                uVar11 = 0x1030;
                uVar7 = _local_16;
                pass1_1030_1d7c(uVar4, extraout_DX, local_12, (local_12 >> 0x10));
                if ((extraout_DX_07 | uVar7) == 0) {}
                  // goto LAB_1038_3e98;
                uVar11 = SUB42(&PTR_LOOP_1050_1028, 0);
                uVar12 = pass1_1028_45e2(uVar7 & 0xffff | extraout_DX_07 << 0x10);
                uVar8 = (uVar12 >> 0x10) | uVar12;
                if (uVar8 == 0) {}
                  // goto LAB_1038_3e98;
                uVar3 = pass1_1028_b58e((uVar7 & 0xffff | extraout_DX_07 << 0x10));
                if (uVar12 < CONCAT22(param_3, param_2))
                {
                    uVar11 = 0x1030;
                    pass1_1030_7ddc(CONCAT31(extraout_var_00, uVar3) & 0xffff | uVar8 << 0x10, uVar12,
                                    param_3_00);
                    lVar1 = CONCAT22(param_3, param_2) - uVar12;
                    param_2 = lVar1;
                    param_3 = (lVar1 >> 0x10);
                  // goto LAB_1038_3e98;
                }
                uVar14 = param_3;
                uVar15 = (param_3 >> 8);
                uVar13 = extraout_var_00;
// LAB_1038_3e67:
                uVar11 = 0x1030;
                pass1_1030_7ddc(CONCAT22(uVar8, CONCAT11(uVar13, uVar3)),
                                CONCAT13(uVar15, CONCAT12(uVar14, param_2)), param_3_00);
            }
          // goto LAB_1038_3e6c;
        }
        uVar11 = SUB42(&PTR_LOOP_1050_1008, 0);
        puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 10);
        uVar4 = SUB42(puVar6, 0);
        pass1_1038_4e78(param_1, puVar6 & 0xffff | in_DX << 0x10);
        _local_1a = CONCAT22(extraout_DX_04, uVar4);
        ppcVar2 = (*_local_1a + 0x10);
        uVar10 = uVar4;
        ppcVar2(&PTR_LOOP_1050_1008, uVar4, extraout_DX_04);
        _local_16 = CONCAT22(extraout_DX_05, uVar10);
        local_12 = 0;
        while (local_12 < _local_16)
        {
            uVar11 = 0x1030;
            uVar7 = _local_16;
            pass1_1030_1d7c(uVar4, extraout_DX_04, local_12, (local_12 >> 0x10));
            uVar8 = extraout_DX_06 | uVar7;
            if (uVar8 != 0)
            {
                uVar14 = param_3;
                uVar15 = (param_3 >> 8);
                uVar3 = pass1_1028_b58e((uVar7 & 0xffff | extraout_DX_06 << 0x10));
                uVar13 = extraout_AH;
              // goto LAB_1038_3e67;
            }
            local_12 = local_12 + 1;
        }
// LAB_1038_3e6c:
        if (_local_1a == 0x0)
        {
            return;
        }
        uVar10 = (_local_1a >> 0x10);
        puVar9 = _local_1a;
    }
    unsafe{ppcVar2 = *puVar9;}
    ppcVar2(uVar11, _local_1a, uVar10, 1);
    return;
// LAB_1038_3e98:
    local_12 = local_12 + 1;
  // goto LAB_1038_3e9c;
}




pub fn pass1_1038_3efc(param_1: u16, param_2: u16, param_1_00: *mut u8, param_2_00: *mut u8)

{
    let ppcVar1: fn();
    let paVar2: *mut astruct_493;
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



pub fn pass1_1038_3222(param_1: *mut astruct_848,param_2: u32,param_3: u32)

{
    let mut u_var1: u32;
    let uVar2: u8;
    let mut uVar3: i32;
    let pcVar4: *mut libc::c_char;
    let extraout_var: u32;
    let paVar5: *mut astruct_199;
    let mut uVar6: i32;
    let mut extraout_DX: u16;
    let local_BX_33: *mut astruct_1084;
    let mut uVar7: u16;
    let unaff_SS: *mut libc::c_char;
    let paVar8: *mut astruct_848;
    let mut local_26: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8;20];

    paVar8 = pass1_1030_183c(param_1, 0, 0, 0x4000000, param_3);
    paVar5 = (paVar8 >> 0x10);
    uVar7 = (param_1 >> 0x10);
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
    uVar2 = pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_33.field_0x1a2), 0,
                            0x54);
    uVar3 = CONCAT31(extraout_var, uVar2);
    process_struct_1000_179c(0x1b0, paVar5);
    uVar6 = paVar5 | uVar3;
    if (uVar6 == 0)
    {
        &local_BX_33.field_0x1f6 = 0;
    }
    else
    {
        pass1_1030_314c(CONCAT22(paVar5, uVar3), &local_BX_33.field_0x4);
        local_BX_33.field_0x1f6 = uVar3;
        local_BX_33.field_0x1f8 = uVar6;
    }
    uVar1 = &local_BX_33.field_0x4;
    paVar5 = (&local_BX_33.field_0x6 & 0xff);
    string_fn_1000_3f9c(local_16, unaff_SS, s__5lu_1050_5a1a, &g_alloc_addr_1050_1050,
                        uVar1);
    pcVar4 = local_16;
    pass1_fn_1008_60e8(pcVar4, unaff_SS, uVar1);
    local_BX_33.field_0x1fa = pcVar4;
    local_BX_33.field_0x1fc = paVar5;
    process_struct_1000_179c(0x1e, paVar5);
    if ((paVar5 | pcVar4) == 0)
    {
        &local_BX_33.field_0xc = 0;
    }
    else
    {
        pass1_1020_c444(CONCAT22(paVar5, pcVar4), 100, 200);
        local_BX_33.field_0xc = pcVar4;
        local_BX_33.field_0xe = extraout_DX;
    }
    return;
}

pub fn pass1_1038_33f8(param_1: *mut astruct_44)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_5: *mut astruct_850;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1.ptr_a_lo = 0x6504;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    puVar1 = ((local_BX_5 + 1)).field_0x0;
    uVar2 = local_BX_5[1].field_0x2;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;}
        (**ppcVar3)();
    }
    puVar1 = local_BX_5[0x19].field_0x2;
    uVar2 = &local_BX_5[0x19].field_0x4;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;}
        (**ppcVar3)();
    }
    error_check_1000_17ce(&local_BX_5[0x19].field_0x6);
    puVar1 = &local_BX_5[0x1a].field_0x8;
    uVar2 = &local_BX_5[0x1a].field_0xa;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;}
        (**ppcVar3)(0x1000, puVar1, uVar2, 1);
    }
    error_check_1000_17ce((&local_BX_5[0x1a].field_0x10 + 2));
    pass1_1030_18b2(param_1);
    return;
}



pub fn pass1_1038_349e(param_1: *mut astruct_1086,param_2: u32)

{
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut uVar4: i32;
    let mut uVar5: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let local_BX_8: *mut astruct_1086;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let local_16: *mut astruct_1087;
    let mut local_14: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar6 = (param_1 >> 0x10);
    local_BX_8 = param_1;
    local_BX_8.field_0x200 = param_2;
    pass1_1038_4d0e(param_1, 600);
    uVar3 = param_2;
    pass1_1038_4d0e(param_1, 600);
    local_BX_8.field_0x204 = 0;
    local_BX_8.field_0x206 = 0;
    uVar1 = local_BX_8.field_0xc;
    uVar7 = uVar1;
    uVar8 = (uVar1 >> 0x10);
    ppcVar2 = (local_BX_8.field_0xc + 0x10);
    ppcVar2();
    _local_6 = CONCAT22(extraout_DX, uVar3);
    local_a = 0;
    while (local_a < _local_6)
    {
        uVar5 = _local_6;
        pass1_1030_1d7c(local_BX_8.field_0xc, local_a, (local_a >> 0x10));
        uVar4 = uVar5;
        if ((extraout_DX_00 | uVar4) != 0)
        {
            ppcVar2 = ((uVar5 & 0xffff | extraout_DX_00 << 0x10) +
                                0x58);
            ppcVar2(0x1030, uVar4, extraout_DX_00, param_1, uVar6, uVar7, uVar8);
            (uVar4 + 0x1c) = 0;
        }
        local_a = local_a + 1;
    }
    return;
}

pub fn pass1_1038_354a(param_1: *mut astruct_1088)

{
    let in_AX: *mut astruct_493;
    let in_DX: *mut astruct_199;
    let mut uVar1: i32;
    let local_BX_4: *mut astruct_1088;
    let mut uVar2: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x21a == 0)
    {
        process_struct_1000_179c(10, in_DX);
        uVar1 = in_DX | in_AX;
        if (uVar1 == 0)
        {
            &local_BX_4.field_0x21a = 0;
        }
        else
        {
            pass1_1030_9ecc(CONCAT22(in_DX, in_AX), param_1);
            local_BX_4.field_0x21a = in_AX;
            &local_BX_4.field_0x21c = uVar1;
        }
    }
    pass1_1030_9ef2(&local_BX_4.field_0x21a);
    return;
}

pub fn pass1_1038_35a8(param_1: *mut astruct_1089, param_2: u16)

{
    let mut in_AX: i32;
    let in_DX: *mut astruct_199;
    let mut uVar1: i32;
    let local_BX_4: *mut astruct_1089;
    let mut uVar2: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x21a == 0)
    {
        process_struct_1000_179c(10, in_DX);
        uVar1 = in_DX | in_AX;
        if (uVar1 == 0)
        {
            &local_BX_4.field_0x21a = 0;
        }
        else
        {
            pass1_1030_9ecc(CONCAT22(in_DX, in_AX), param_1);
            local_BX_4.field_0x21a = in_AX;
            &local_BX_4.field_0x21c = uVar1;
        }
    }
    pass1_1030_9f40(&local_BX_4.field_0x21a, param_2);
    return;
}

pub fn pass1_1038_3608(param_1: *mut u8)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    error_check_1000_17ce((param_1 + 0x21a));
    (param_1 + 0x21a) = 0;
    return;
}




pub fn pass1_1038_2c82(param_1: u16, param_2: u16,param_1_00: u32,param_2_00: u32,param_5: u32) -> i32

{
    let puVar1: *mut u32;
    let piVar2: *mut i32;
    let mut uVar3: i32;
    let mut uVar4: u32;
    let ppcVar5: fn();
    let mut uVar6: i32;
    let paVar7: *mut astruct_493;
    let ppVar8: *mut pass1_struct_2;
    let mut iVar9: i32;
    let puVar10: *mut u32;
    let mut extraout_DX: u16;
    let local_BX_4: *mut astruct_1079;
    let mut iVar11: i32;
    let local_BX_35: *mut astruct_1080;
    let mut iVar12: i32;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut unaff_SS: u16;
    let ppVar17: *mut pass1_struct_1;
    let uVar18: u8;
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

    uVar13 = (param_5 >> 0x10);
    local_BX_4 = param_5;
    local_6 = local_BX_4.field_0x200;
    uVar14 = (param_2_00 >> 0x10);
    iVar11 = param_2_00;
    local_a = (iVar11 + 0x200);
    uVar16 = (iVar11 + 0x202);
    uVar15 = (param_1_00 >> 0x10);
    local_BX_35 = param_1_00;
    iVar9 = local_BX_35.field_0xc;
    if (iVar9 == 1)
    {
        _local_e = param_1_00;
        pass1_1038_52b8(param_5, &local_BX_35.field_0x8,
                        &local_BX_35.field_0xe);
        return;
    }
    if (iVar9 == 2)
    {
        _local_e = param_1_00;
        if (&local_BX_35.field_0xe != 0)
        {
            pass1_1038_3efc(local_BX_4, uVar13, param_2_00, *&local_BX_35.field_0xe);
            return;
        }
        pass1_1020_a43e(CONCAT22(unaff_SS, &local_12));
        local_16 = (_local_e + 8);
        while (local_16 = local_16 - 1, (local_16._2_2_ | local_16) != 0)
        {
            pass1_1020_a6ee(CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_12)), (_local_e + 0x12));
        }
    }
    else
    {
        if (iVar9 == 3)
        {
            pass1_1038_3f38(param_5, param_2_00, &local_BX_35.field_0xe);
            return;
        }
        local_6._2_2_ = (local_6 >> 0x10);
        if (iVar9 == 4)
        {
            g_u16_ptr_1050_5f2e = (local_6._2_2_ & 0xff);
            if ((local_6 == (&PTR_LOOP_1050_0000 + 1)) &&
                ((local_6 & 0xff0000) == 0))
            {
                local_12 = local_BX_4.field_0x1f6;
                uVar4 = &local_BX_35.field_0x8;
                pass1_1030_3694(local_12, (local_12 >> 0x10),
                                &local_BX_35.field_0xe, uVar4,
                                (uVar4 >> 0x10));
                local_BX_35.field_0x10 = local_12;
                local_BX_35.field_0x12 = extraout_DX;
            }
            else
            {
                if (__g_astruct_94_ptr_1 == 0)
                {
                    _g_astruct_94_ptr_1 = local_6;
                    struct_fn_1000_160a();
                }
                else
                {
                }
                alloc_mem_1000_1708(0x6c, 0, 1, _g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
                local_BX_35.field_0x10 = _g_astruct_94_ptr_1;
                local_BX_35.field_0x12 = g_u16_ptr_1050_5f2e;
                iVar9 = &local_BX_35.field_0xe;
                if (iVar9 != 3)
                {
                    if (iVar9 != 4)
                    {
                        uVar4 = &local_BX_35.field_0x10;
                        (uVar4 + 0x28) = &local_BX_35.field_0x8;
                        return;
                    }
                    uVar4 = &local_BX_35.field_0x10;
                    (uVar4 + 0xdc) = &local_BX_35.field_0x8;
                    return;
                }
                uVar4 = &local_BX_35.field_0x10;
                (uVar4 + 100) = &local_BX_35.field_0x8;
            }
        }
        else
        {
            if (iVar9 == 5)
            {
                if (&local_BX_35.field_0xe == 0xc)
                {
                    if ((local_6 == (&PTR_LOOP_1050_0000 + 1)) &&
                        ((local_6 & 0xff0000) == 0))
                    {
                        uVar4 = local_BX_4.field_0x1f6;
                        iVar9 = local_BX_35.field_0x8;
                        iVar11 = local_BX_35.field_0xa;
                        uVar6 = -iVar9;
                        uVar16 = (uVar4 >> 0x10);
                        iVar12 = uVar4;
                        puVar1 = (iVar12 + 0x170);
                        unsafe{
                        uVar3 = *puVar1;
                        *puVar1 = *puVar1 + uVar6;
                        piVar2 = (iVar12 + 0x172);
                        *piVar2 = (*piVar2 - (iVar11 + (iVar9 != 0))) + CARRY2(uVar3, uVar6);
                        }
                    }
                }
                else
                {
                    uVar16 = local_BX_35.field_0x8;
                    pass1_1038_43cc(local_BX_4,
                                    CONCAT13((uVar16 >> 8), CONCAT12(uVar16, uVar13)),
                                    &local_BX_35.field_0xe);
                }
            }
            else
            {
                if (iVar9 == 7)
                {
                    uVar4 = &local_BX_35.field_0xe;
                    paVar7 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, (uVar4 >> 0x10));
                    pass1_1038_349e(CONCAT22(uVar16, paVar7), (iVar11 + 0x200));
                    uVar18 = (uVar16 >> 8);
                    pass1_1038_4d0e(CONCAT13(uVar18, CONCAT12(uVar16, paVar7)), 600);
                    pass1_1038_4d0e(CONCAT13(uVar18, CONCAT12(uVar16, paVar7)), 600);
                    ppVar17 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffce, 0x3b));
                    pass1_1008_de58(ppVar17, &local_BX_35.field_0xe, 0x4000001);
                    ppVar17 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffce, 0x2f));
                    uVar16 = (ppVar17 >> 0x10);
                    ppVar8 = pass1_1030_8344(_g_bool_1050_5748,
                                             (_g_bool_1050_5748 >> 0x10),
                                             (ppVar17 + 0x20));
                    local_12 = CONCAT22(uVar16, ppVar8);
                    iVar9 = pass1_1030_5b00(CONCAT22(uVar16, ppVar8));
                    _local_e = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffce, iVar9));
                    puVar10 = (_local_e + 0x20);
                    unsafe{ppcVar5 = (*puVar10 + 4);}
                    (**ppcVar5)(0x1010, puVar10, (_local_e >> 0x10), 6);
                }
            }
        }
    }
    return;
}



pub fn pass1_1038_2f92(param_1: u16, param_2: u16, param_1: u16_00,param_2_00: u32)

{
    let puVar1: *mut u32;
    astruct_1082 **ppaVar2;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let mut uVar6: u32;
    let mut uVar7: u32;
    let local_BX_189: *mut astruct_1081;
    let mut uVar8: u16;
    let mut in_stack_0000000e: u16;
    let mut local_1a: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let temp_5f8f06f4df: *mut astruct_1082;

    uVar6 = (param_2_00._2_2_ + 0x200);
    iVar3 = (param_1_00 + 0xc);
    if (iVar3 == 1)
    {
        uVar7 = (param_1_00 + 8);
        pass1_1038_3cc0(CONCAT22(in_stack_0000000e, param_2_00._2_2_), uVar7,
                        (uVar7 >> 0x10), (param_1_00 + 0xe));
        return;
    }
    if (iVar3 == 4)
    {
        pass1_1030_355c((param_2_00._2_2_ + 0x1f6), (param_1_00 + 0x10));
        return;
    }
    if (iVar3 == 5)
    {
        if ((param_1_00 + 0xe) != 0xc)
        {
            pass1_1038_5798(CONCAT22(in_stack_0000000e, param_2_00._2_2_), (param_1_00 + 8),
                            (param_1_00 + 0xe));
            return;
        }
        local_a._0_2_ = uVar6;
        if ((local_a == 1) && ((uVar6 & 0xff0000) == 0))
        {
            uVar7 = (param_2_00._2_2_ + 0x1f6);
            uVar4 = (param_1_00 + 8);
            temp_5f8f06f4df = (param_1_00 + 10);
            uVar8 = (uVar7 >> 0x10);
            local_BX_189 = uVar7;
            puVar1 = &local_BX_189.field_0x170;
            unsafe{uVar5 = *puVar1;
            *puVar1 = *puVar1 + uVar4;
            ppaVar2 = &local_BX_189.field_0x172;
            *ppaVar2 = temp_5f8f06f4df + (*ppaVar2 + CARRY2(uVar5, uVar4));
            return;
        }
    }
    return;
}

pub fn pass1_1038_3074(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_2a5c(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1038_30aa(param_1: *mut astruct_393)

{
    let uVar1: u8;
    let mut uVar2: i32;
    let extraout_var: u32;
    let in_DX: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let local_BX_19: *mut astruct_1083;
    let mut uVar4: u16;
    let mut local_4: u16;

    pass1_1030_17ce(param_1, 0, 0);
    uVar4 = (param_1 >> 0x10);
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
    uVar1 = pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_19.field_0x1a2), 0,
                            0x54);
    uVar2 = CONCAT31(extraout_var, uVar1);
    process_struct_1000_179c(0x1b0, in_DX);
    struct_a = (in_DX | uVar2);
    if (struct_a == 0x0)
    {
        &local_BX_19.field_0x1f6 = 0;
    }
    else
    {
        pass1_1030_314c(CONCAT22(in_DX, uVar2), local_BX_19.field_0x4);
        local_BX_19.field_0x1f6 = uVar2;
        local_BX_19.field_0x1f8 = struct_a;
    }
    process_struct_1000_179c(0x1e, struct_a);
    if ((struct_a | uVar2) == 0)
    {
        uVar2 = 0;
        uVar3 = 0;
    }
    else
    {
        pass1_1020_c444(CONCAT22(struct_a, uVar2), 100, 200);
        uVar3 = extraout_DX;
    }
    local_BX_19.field_0xc = uVar2;
    local_BX_19.field_0xe = uVar3;
    return;
}


pub fn pass1_1038_29d2(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_2a0e(param_1: *mut astruct_500,param_2: u32,param_3: u32,param_4: u32,param_5: u32) -> i32

{
    let local_BX_19: *mut astruct_500;
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

pub fn pass1_1038_2a5c(param_1: *mut u16)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_5: *mut astruct_1076;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    *param_1 = 0x309a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1038;
    puVar1 = local_BX_5.field_0x114;
    uVar2 = local_BX_5.field_0x116;
    if ((uVar2 | puVar1) != 0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    puVar1 = local_BX_5.field_0x110;
    uVar2 = local_BX_5.field_0x112;
    if ((uVar2 | puVar1) != 0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    *param_1 = s_1_1050_389a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}




pub fn pass1_1038_2ac2(param_1: *mut astruct_1077)

{
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let mut in_DX: u16;
    let local_BX_4: *mut astruct_1077;
    let mut uvar3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar1 = local_BX_4.field_0x108;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    _local_6 = CONCAT22(in_DX, paVar2);
    uVar1 = local_BX_4.field_0x10c;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    _local_a = CONCAT22(in_DX, paVar2);
    pass1_1038_2c82(local_BX_4, uVar3, local_BX_4.field_0x110, CONCAT22(in_DX, paVar2), _local_6);
    pass1_1038_2c82(local_BX_4, uVar3, local_BX_4.field_0x114, _local_6, _local_a);
    return 1;
}



pub fn pass1_1038_2b2e(param_1: *mut astruct_1078)

{
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let paVar3: *mut astruct_493;
    let local_BX_4: *mut astruct_1078;
    let local_ES_4: *mut astruct_1078;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar1 = local_BX_4.field_0x108;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    uVar1 = local_BX_4.field_0x10c;
    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    uVar1 = local_BX_4.field_0x110;
    pass1_1038_2f92(local_BX_4, local_ES_4, uVar1,
                    CONCAT22(paVar3, (uVar1 >> 0x10)));
    uVar1 = local_BX_4.field_0x114;
    pass1_1038_2f92(local_BX_4, local_ES_4, uVar1,
                    CONCAT22(paVar2, (uVar1 >> 0x10)));
    return 1;
}


pub fn pass1_1038_0d8e(param_1: u16, param_2: u16,param_1_00: u32, param_2_00: *mut astruct_44)

{
    let mut uVar1: u16;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = pass1_1030_d0a8(param_2_00);
    uVar2 = pass1_1030_d144(param_2_00);
    _local_a = uVar2;
    local_4 = uVar1;
    if ((uVar2 >> 0xf | uVar2) != 0)
    {
        while
        {
            uVar3 = pass1_1028_6744(param_1_00, local_4);
            if (uVar3 != 0)
            {
                pass1_1028_6228(param_1_00, 1, 0, local_4);
                _local_a = _local_a + -1;
                pass1_1030_d180(param_2_00, local_4);
            }
            if (_local_a == 0)
            {
                return;
            }
            local_4 = pass1_1030_d0a8(param_2_00);
            local_4 != uVar1
        } {}
    }
    return;
}




pub fn pass1_1038_0e00(param_1: u32,param_2: u32,param_3: u32)

{
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut astruct_493;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut uVar3: i32;
    let mut uVar4: u32;
    let mut local_14: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppcVar1 = (param_2 + 0x10);
    (**ppcVar1)();
    _local_6 = CONCAT22(extraout_DX, in_AX);
    local_a = 0;
    while (local_a < _local_6)
    {
        ppcVar1 = (param_2 + 4);
        uVar4 = _local_6;
        (**ppcVar1)();
        uVar3 = extraout_DX_00 | uVar4;
        if (uVar3 != 0)
        {
            paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, extraout_DX_00);
            uVar4 = pass1_1030_73a8(CONCAT22(uVar3, paVar2));
            uVar3 = (uVar4 >> 0x10);
            if ((uVar3 | uVar4) != 0)
            {
                pass1_1038_0d8e(param_1, (param_1 >> 0x10), uVar4 & 0xffff | uVar3 << 0x10,
                                param_3);
            }
        }
        local_a = local_a + 1;
    }
    return;
}




pub fn pass1_1038_0e78(param_1: u32,param_2: u32)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uVar3: i32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let paVar6: *mut astruct_493;
    let puVar7: *mut u8;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut uVar8: i32;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: i32;
    let mut uVar9: u16;
    let mut uVar10: u32;
    let mut local_28: u16;
    let mut local_20: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let local_e: *mut astruct_1058;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 4);
    uVar3 = puVar7;
    pass1_1038_4d6e(param_2, puVar7 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, uVar3);
    uVar2 = *_local_a;
    ppcVar1 = uVar2 + 8;
    uVar8 = uVar3;
    (**ppcVar1)(&PTR_LOOP_1050_1008, uVar3, extraout_DX);
    uVar8 = extraout_DX_00 | uVar8;
    if (uVar8 == 0)
    {
        if (_local_a != 0x0)
        {
            ppcVar1 = uVar2;
            (**ppcVar1)(8, uVar3, extraout_DX, 1);
            return;
        }
    }
    else
    {
        uVar9 = SUB42(&PTR_LOOP_1050_1008, 0);
        puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        uVar4 = SUB42(puVar7, 0);
        pass1_1038_4d6e(param_2, puVar7 & 0xffff | uVar8 << 0x10);
        _local_e = CONCAT22(extraout_DX_01, uVar4);
        ppcVar1 = (*_local_e + 0x10);
        uVar5 = uVar4;
        (**ppcVar1)(&PTR_LOOP_1050_1008, uVar4, extraout_DX_01);
        _local_12 = CONCAT22(extraout_DX_02, uVar5);
        local_16 = 0;
        while (local_16 < _local_12)
        {
            ppcVar1 = (*_local_e + 4);
            uVar10 = _local_12;
            (**ppcVar1)();
            uVar8 = extraout_DX_03 | uVar10;
            if (uVar8 != 0)
            {
                paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar10, extraout_DX_03);
                uVar9 = 0x1030;
                uVar10 = pass1_1030_73a8(CONCAT22(uVar8, paVar6));
                if (((uVar10 >> 0x10) | uVar10) != 0)
                {
                    pass1_1038_0e00(param_1, _local_a, uVar10);
                }
            }
            local_16 = local_16 + 1;
        }
        if (_local_a != 0x0)
        {
            ppcVar1 = *_local_a;
            (**ppcVar1)(uVar9, uVar3, extraout_DX, 1);
        }
        if (_local_e != 0x0)
        {
            ppcVar1 = *_local_e;
            (**ppcVar1)(uVar9, uVar4, extraout_DX_01, 1);
        }
    }
    return;
}




pub fn pass1_1038_0f8c(param_1: u16, param_2: u16, param_1_00: *mut u32,param_2_00: u32)

{
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: i32;
    let ppcVar6: fn();
    let mut uVar7: u32;
    let Var8: u16;
    let mut in_AX: u16;
    let local_AX_95: *mut astruct_1059;
    let puVar9: *mut u8;
    let local_AX_405: *mut astruct_1060;
    let mut uVar10: u32;
    let mut uVar12: i32;
    let mut uVar13: i32;
    let mut in_EDX: u32;
    let local_BX_491: *mut astruct_1061;
    let mut uVar14: u16;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let puVar15: *mut u32;
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
    let mut local_30: [u8;4];
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: [u8;4];
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
    let puVar11: *mut u8;

    local_6 = 100;
    local_8 = 0;
    ppcVar6 = (*param_1_00 + 0x10);
    puVar15 = param_1_00;
    (**ppcVar6)();
    _local_c = CONCAT22(in_EDX, in_AX);
    local_10 = 0;
    loop
    {
        if (_local_c <= local_10)
        {
            return;
        }
        ppcVar6 = (*param_1_00 + 4);
        uVar10 = _local_c;
        uVar13 = in_EDX;
        (**ppcVar6)(unaff_CS, param_1_00, local_10, (local_10 >> 0x10), puVar15);
        local_12 = uVar13;
        local_14 = uVar10;
        uVar13 = local_12 | local_14;
        in_EDX = uVar13;
        if (uVar13 != 0)
        {
            local_18 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_14, local_12);
            local_16 = uVar13;
            unaff_CS = 0x1030;
            local_1c = pass1_1030_73a8(CONCAT22(local_16, local_18));
            in_EDX = local_1c >> 0x10;
            puVar9 = local_20;
            ppcVar6 = (local_1c + 0x40);
            (**ppcVar6)(0x1030, local_1c, (local_1c >> 0x10), puVar9);
            if (puVar9 == 0x0)
            {
                _local_24 = pass1_1028_62c8(local_1c);
                uVar10 = _local_24 >> 0x10;
                local_8 = 1;
                local_28 = (param_2_00 + 0x22);
                pass1_1008_5784(CONCAT22(unaff_SS, local_30), local_28);
                loop
                {
                    uVar13 = uVar10;
                    local_AX_95 = local_30;
                    unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                    pass1_1008_5b12(CONCAT22(unaff_SS, local_AX_95));
                    in_EDX = (uVar13 | local_AX_95);
                    if ((uVar13 | local_AX_95) == 0) {
                        break;}
                    uVar2 = local_AX_95.field_0x4;
                    uVar3 = local_AX_95.field_0x6;
                    uVar4 = local_AX_95.field_0x8;
                    uVar5 = local_AX_95.field_0xa;
                    uVar7 = local_AX_95.field_0xc / uVar5;
                    uVar10 = _local_24;
                    if (local_6 < _local_24)
                    {
                        uVar10 = local_6 & 0xffff;
                        local_22 = local_6._2_2_;
                    }
                    uVar12 = local_22 | uVar10;
                    in_EDX = uVar12;
                    if (uVar12 == 0) {
                        break;}
                    qVar8 = (uVar10 & 0xffff | local_22 << 0x10) / uVar7;
                    in_EDX = qVar8 >> 0x10;
                    uVar12 = (qVar8 >> 0x10);
                    local_4c = qVar8;
                    if (local_4c == 0) {
                        break;}
                    if (local_4c < uVar5)
                    {
                        puVar1 = &local_AX_95.field_0xc;
                        *puVar1 = *puVar1 - uVar10;
                        puVar1 = &local_AX_95.field_0xa;
                        *puVar1 = *puVar1 - local_4c;
                    }
                    else
                    {
                        ppcVar6 = (local_28 + 0xc);
                        (**ppcVar6)(&PTR_LOOP_1050_1008, local_28, (local_28 >> 0x10),
                                    local_AX_95, uVar13);
                        local_2c = 0;
                        local_4c = uVar5;
                    }
                    puVar11 = _PTR_LOOP_1050_68a2;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                    uVar13 = puVar11;
                    local_50 = puVar11 & 0xffff | uVar12 << 0x10;
                    if ((uVar12 | uVar13) == 0)
                    {
                        local_50 = 0;
                    }
                    else
                    {
                        local_50 = s_1_1050_389a;
                        (uVar13 + 2) = &PTR_LOOP_1050_1008;
                        (uVar13 + 4) = 0;
                        (uVar13 + 6) = 0;
                        (uVar13 + 8) = 0;
                        (uVar13 + 10) = 0;
                        (uVar13 + 0xc) = 0;
                        local_50 = 0x56ce;
                        (uVar13 + 2) = 0x1018;
                    }
                    uVar14 = (local_50 >> 0x10);
                    local_BX_491 = local_50;
                    local_BX_491.field_0xa = local_4c;
                    uVar7 = local_4c * uVar7;
                    uVar10 = uVar7 >> 0x10;
                    local_BX_491.field_0xc = uVar7;
                    local_BX_491.field_0x4 = uVar2;
                    local_BX_491.field_0x6 = uVar3;
                    local_BX_491.field_0x8 = uVar4;
                    pass1_1028_6408(local_1c, local_50);
                }
            }
            else
            {
                ppcVar6 = (*param_1_00 + 8);
                (**ppcVar6)(0x1030, param_1_00, 0, 0, local_10, (local_10 >> 0x10));
            }
        }
        local_10 = local_10 + 1;
    } 
}




pub fn pass1_1038_11b0(param_1: u32,param_2: u32,param_3: u32)

{
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut astruct_493;
    let mut iVar3: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppcVar1 = (param_3 + 0x10);
    (**ppcVar1)();
    _local_6 = CONCAT22(extraout_DX, in_AX);
    local_a = 0;
    loop
    {
        if (_local_6 <= local_a)
        {
            return;
        }
        ppcVar1 = (param_3 + 4);
        uVar5 = _local_6;
        (**ppcVar1)();
        uVar4 = extraout_DX_00;
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_00);
        uVar5 = pass1_1030_73a8(CONCAT22(uVar4, paVar2));
        iVar3 = uVar5;
        pass1_1038_0f8c(param_1, (param_1 >> 0x10), param_2, uVar5);
        if (iVar3 == 0) {
            break;}
        local_a = local_a + 1;
    }
    return;
}




pub fn pass1_1038_1220(param_1: u32,param_2: u32)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let puVar6: *mut u8;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut uVar7: i32;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: i32;
    let mut extraout_DX_05: u16;
    let mut extraout_DX_06: i32;
    let uVar8: u8;
    let mut local_e: u16;
    let mut local_c: u16;
    let local_a: *mut astruct_1062;
    let mut local_8: u16;

    puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 4);
    iVar3 = puVar6;
    pass1_1038_4d6e(param_2, puVar6 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, iVar3);
    ppcVar1 = (*_local_a + 0x10);
    iVar4 = iVar3;
    (**ppcVar1)(&PTR_LOOP_1050_1008, iVar3, extraout_DX);
    if ((extraout_DX_00 != 0) || (iVar4 != 0))
    {
        uVar7 = extraout_DX_00;
        puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 5);
        iVar5 = puVar6;
        pass1_1038_4d6e(param_2, puVar6 & 0xffff | uVar7 << 0x10);
        _local_e = CONCAT22(extraout_DX_01, iVar5);
        uVar8 = iVar5;
        uVar2 = *_local_e;
        ppcVar1 = uVar2 + 8;
        iVar4 = iVar5;
        (**ppcVar1)(&PTR_LOOP_1050_1008, uVar8, extraout_DX_01);
        if (((extraout_DX_02 != 0) || (uVar7 = extraout_DX_02, iVar4 != 0)) &&
            (pass1_1038_11b0(param_1, CONCAT13((extraout_DX >> 8), CONCAT12(extraout_DX, iVar3)),
                             CONCAT22(extraout_DX_01, iVar5)),
             uVar7 = extraout_DX_03, iVar4 == 0))
        {
            if (_local_e == 0x0)
            {
                return;
            }
            ppcVar1 = uVar2;
            (**ppcVar1)(8, uVar8, extraout_DX_01, 1);
            return;
        }
        if (_local_e != 0x0)
        {
            ppcVar1 = *_local_e;
            (**ppcVar1)(8, uVar8, extraout_DX_01, 1);
            uVar7 = extraout_DX_04;
        }
        puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 6);
        iVar5 = puVar6;
        pass1_1038_4d6e(param_2, puVar6 & 0xffff | uVar7 << 0x10);
        _local_e = CONCAT22(extraout_DX_05, iVar5);
        ppcVar1 = (*_local_e + 0x10);
        iVar4 = iVar5;
        (**ppcVar1)(8, iVar5, extraout_DX_05);
        if ((extraout_DX_06 != 0) || (iVar4 != 0))
        {
            pass1_1038_11b0(param_1, CONCAT22(extraout_DX, iVar3), CONCAT22(extraout_DX_05, iVar5));
        }
        if (_local_e != 0x0)
        {
            ppcVar1 = *_local_e;
            (**ppcVar1)(8, iVar5, extraout_DX_05, 1);
        }
    }
    if (_local_a != 0x0)
    {
        ppcVar1 = *_local_a;
        (**ppcVar1)(8, iVar3, extraout_DX, 1);
    }
    return;
}





pub fn pass1_1038_134a(param_1: u16, param_2: u16, param_1_00: *mut u32, param_2_00: *mut u32, param_5: *mut u32) -> i32

{
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut astruct_493;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uvar3: u16;
    let mut uVar4: i32;
    let mut unaff_CS: u16;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let puVar7: *mut u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppcVar1 = (*param_5 + 0x10);
    puVar7 = param_5;
    (**ppcVar1)();
    _local_6 = CONCAT22(extraout_DX, in_AX);
    *param_1_00 = 0;
    while
    {
        if (_local_6 <= *param_2_00)
        {
            return;
        }
        uVar5 = *param_2_00;
        *param_2_00 = *param_2_00 + 1;
        ppcVar1 = (*param_5 + 4);
        (**ppcVar1)(unaff_CS, param_5, uVar5, puVar7);
        uVar3 = extraout_DX_00;
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_00);
        uVar5 = pass1_1030_73a8(CONCAT22(uVar3, paVar2));
        unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
        uVar6 = pass1_1028_45e2(uVar5);
        uVar4 = (uVar6 >> 0x10);
        param_1_00 = uVar6;
        (param_1_00 + 2) = uVar4;
        (uVar4 | param_1_00) == 0
    } {}
    return;
}





pub fn pass1_1038_13da(param_1: u16, param_2: u16, param_1_00: *mut u32, param_2_00: *mut u32, param_5: *mut u32) -> i32

{
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut astruct_493;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uvar3: u16;
    let mut uVar4: i32;
    let mut unaff_CS: u16;
    let mut uVar5: u32;
    let puVar6: *mut u8;
    let puVar7: *mut u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppcVar1 = (*param_5 + 0x10);
    puVar7 = param_5;
    (**ppcVar1)();
    _local_6 = CONCAT22(extraout_DX, in_AX);
    *param_1_00 = 0;
    while
    {
        if (_local_6 <= *param_2_00)
        {
            return;
        }
        uVar5 = *param_2_00;
        *param_2_00 = *param_2_00 + 1;
        ppcVar1 = (*param_5 + 4);
        (**ppcVar1)(unaff_CS, param_5, uVar5, puVar7);
        uVar3 = extraout_DX_00;
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_00);
        if ((uVar3 | paVar2) == 0)
        {
            return;
        }
        uVar5 = pass1_1030_73a8(CONCAT22(uVar3, paVar2));
        uVar4 = (uVar5 >> 0x10);
        if ((uVar4 | uVar5) == 0)
        {
            return;
        }
        unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
        puVar6 = pass1_1028_3c32((uVar5 & 0xffff | uVar4 << 0x10));
        uVar4 = (puVar6 >> 0x10);
        param_1_00 = puVar6;
        (param_1_00 + 2) = uVar4;
        (uVar4 | param_1_00) == 0
    } {}
    return;
}




pub fn pass1_1038_1482(param_1: u32,param_2: u32,param_3: u32)

{
    let ppcVar1: fn();
    let Var2: u16;
    let mut uvar3: u16;
    let uVar4: u8;
    let puVar5: *mut u32;
    let mut uVar6: i32;
    let mut uVar7: u16;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut uVar8: u32;
    let mut in_DX: u16;
    let paVar9: *mut astruct_199;
    let paVar10: *mut astruct_199;
    let mut uVar11: i32;
    let mut unaff_SS: u16;
    let mut uVar12: u16;
    let uVar13: u8;
    let uVar14: u8;
    let mut uVar15: u16;
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
    puVar5 = &local_a;
    uVar12 = (param_1 >> 0x10);
    uVar3 = param_1;
    pass1_1038_134a(uVar3, uVar12, CONCAT22(unaff_SS, puVar5), CONCAT22(unaff_SS, &local_6),
                    param_3);
    _local_e = CONCAT22(in_DX, puVar5);
    ppcVar1 = (param_2 + 0x10);
    (**ppcVar1)();
    _local_12 = CONCAT22(in_DX, puVar5);
    local_16 = 0;
    loop
    {
        if (_local_12 <= local_16)
        {
            return;
        }
        if ((local_c | local_e) == 0)
        {
            return;
        }
        uVar4 = pass1_1028_b58e(_local_e);
        uVar11 = CONCAT31(extraout_var_00, uVar4);
        local_1a = uVar11;
        local_18 = local_10;
        pass1_1038_1a30(uVar3, uVar12, CONCAT31(extraout_var_00, uVar4) & 0xffff | local_10 << 0x10);
        local_1e = uVar11;
        local_1c = local_10;
        if ((local_10 | uVar11) != 0)
        {
            sVar2 = CONCAT22(local_10, uVar11) * 100;
            uVar7 = (sVar2 >> 0x20);
            uVar8 = sVar2 >> 1;
            ppcVar1 = (param_2 + 4);
            local_22 = uVar8;
            (**ppcVar1)(&PTR_LOOP_1050_1028, param_2, local_16, (local_16 >> 0x10));
            local_26 = uVar8;
            local_24 = uVar7;
            local_2a = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_26, uVar7);
            local_28 = uVar7;
            _local_2e = pass1_1030_73a8(CONCAT22(uVar7, local_2a));
            local_32 = (_local_2e + 0x28);
            local_36 = 0;
            local_38 = (local_32 + 4);
            local_3a = 0;
            while (local_3a < local_38)
            {
                pass1_1020_bb16(local_32, CONCAT22(unaff_SS, &local_46),
                                CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_42)),
                                local_3a);
                if (((local_46 != 0) && (0xd < local_42)) && (local_42 < 0x1d))
                {
                    uVar8 = local_46;
                    if (local_22 < local_46)
                    {
                        uVar8 = local_22 & 0xffff;
                        local_46._2_2_ = local_22._2_2_;
                    }
                    uVar11 = uVar8;
                    if ((local_a._2_2_ <= local_46._2_2_) &&
                        ((local_a._2_2_ < local_46._2_2_ || (local_a < uVar11))))
                    {
                        uVar11 = local_a;
                        local_46._2_2_ = local_a._2_2_;
                    }
                    _local_4a = CONCAT22(local_46._2_2_, uVar11);
                    local_22 = CONCAT22(local_22._2_2_ +
                                            (-(local_22 < uVar11) - local_46._2_2_),
                                        local_22 - uVar11);
                    local_a = CONCAT22(local_a._2_2_ +
                                           (-(local_a < uVar11) - local_46._2_2_),
                                       local_a - uVar11);
                    paVar10 = local_46._2_2_;
                    if (local_36 == 0)
                    {
                        paVar9 = local_46._2_2_;
                        uVar6 = uVar11;
                        process_struct_1000_179c(10, local_46._2_2_);
                        paVar10 = (paVar9 | uVar6);
                        if (paVar10 == 0x0)
                        {
                            uVar6 = 0;
                            paVar10 = 0x0;
                        }
                        else
                        {
                            pass1_1020_ba3e(CONCAT22(paVar9, uVar6), 5, 5);
                        }
                        local_36 = CONCAT22(paVar10, uVar6);
                    }
                    pass1_1020_bb8a(local_36, (local_36 >> 0x10), uVar11, local_46._2_2_, local_42);
                    pass1_1020_bb8a(local_32, (local_32 >> 0x10), (local_46 - _local_4a),
                                    (local_46 - _local_4a >> 0x10), local_42);
                    if (local_a == 0)
                    {
                        pass1_1038_1b3a(uVar3, uVar12, _local_e, local_36);
                        local_36 = 0;
                        puVar5 = &local_a;
                        pass1_1038_134a(uVar3, uVar12, CONCAT22(unaff_SS, puVar5),
                                        CONCAT22(unaff_SS, &local_6), param_3);
                        _local_e = CONCAT22(paVar10, puVar5);
                        uVar11 = paVar10 | puVar5;
                        if (uVar11 != 0)
                        {
                            uVar13 = 100;
                            uVar14 = 0;
                            uVar15 = 0;
                            uVar4 = pass1_1028_b58e(CONCAT22(paVar10, puVar5));
                            uVar7 = CONCAT31(extraout_var, uVar4);
                            local_1a = uVar7;
                            local_18 = uVar11;
                            pass1_1038_1a30(uVar3, uVar12,
                                            CONCAT31(extraout_var, uVar4) & 0xffff | uVar11 << 0x10);
                            local_22 = (CONCAT22(uVar11, uVar7) * CONCAT22(uVar15, CONCAT11(uVar14, uVar13))) >> 1;
                            local_1e = uVar7;
                            local_1c = uVar11;
                        }
                    }
                    if ((local_22 == 0) || (local_a == 0)) {
                        break;}
                }
                local_3a = local_3a + 1;
            }
            if (local_36 != 0)
            {
                pass1_1038_1b3a(uVar3, uVar12, _local_e, local_36);
                local_36 = 0;
            }
        }
        local_16 = local_16 + 1;
    } 
}




pub fn pass1_1038_16f2(param_1: u32,param_2: u32,param_3: u32)

{
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut uVar4: u16;
    let puVar5: *mut u32;
    let mut uVar6: i32;
    let paVar7: *mut astruct_493;
    let mut uVar8: u16;
    let mut uVar9: i32;
    let lVar10: u32;
    let mut uVar11: u32;
    let mut in_DX: i32;
    let mut uVar12: i32;
    let struct_a: *mut astruct_199;
    let mut uVar13: u32;
    let mut uVar14: u32;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let mut uVar15: u32;
    let mut uVar16: u16;
    let mut uVar17: u32;
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
    puVar5 = &local_a;
    uVar16 = (param_1 >> 0x10);
    uVar3 = param_1;
    pass1_1038_13da(uVar3, uVar16, CONCAT22(unaff_SS, puVar5), CONCAT22(unaff_SS, &local_6),
                    param_3);
    _local_e = CONCAT22(in_DX, puVar5);
    in_DX = in_DX | puVar5;
    if (in_DX != 0)
    {
        ppcVar2 = (param_2 + 0x10);
        uVar17 = param_2;
        ppcVar2();
        _local_12 = CONCAT22(in_DX, puVar5);
        local_16 = 0;
        while (local_16 < _local_12)
        {
            ppcVar2 = (param_2 + 4);
            uVar15 = _local_12;
            uVar6 = in_DX;
            ppcVar2(unaff_CS, param_2, local_16, (local_16 >> 0x10), uVar17);
            paVar7 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar15, uVar6);
            unaff_CS = 0x1030;
            uVar15 = pass1_1030_73a8(CONCAT22(uVar6, paVar7));
            uVar12 = (uVar15 >> 0x10);
            uVar9 = uVar15;
            pass1_1038_1a30(uVar3, uVar16, CONCAT22(uVar6, paVar7));
            if ((uVar12 | uVar9) != 0)
            {
                local_2a = (CONCAT22(uVar12, uVar9) * 100) >> 1;
                uVar1 = &paVar7[1].field_0x4;
                uVar9 = &paVar7[1].field_0x6;
                uVar13 = uVar9;
                local_2e._0_2_ = uVar1;
                if ((uVar9 | local_2e) != 0)
                {
                    _local_32 = 0;
                    uVar8 = pass1_1028_0d80(uVar15);
                    local_38 = 0;
                    local_34 = uVar8;
                    loop
                    {
                        uVar12 = uVar13;
                        uVar4 = (uVar1 >> 0x10);
                        uVar11 = pass1_1020_bae6(local_2e, CONCAT22(local_34, uVar4));
                        uVar9 = uVar11;
                        uVar13 = (uVar12 | uVar9);
                        if ((uVar12 | uVar9) != 0)
                        {
                            uVar14 = uVar12;
                            if ((local_2a._2_2_ <= uVar12) &&
                                ((local_2a._2_2_ < uVar12 || (local_2a < uVar9))))
                            {
                                uVar14 = local_2a._2_2_;
                                uVar9 = local_2a;
                            }
                            if ((local_a._2_2_ <= uVar14) &&
                                ((local_a._2_2_ < uVar14 || (local_a < uVar9))))
                            {
                                uVar14 = local_a._2_2_;
                                uVar9 = local_a;
                            }
                            struct_a = uVar14;
                            local_44 = CONCAT22(struct_a, uVar9);
                            local_2a = CONCAT22((local_2a._2_2_ - struct_a) - (local_2a < uVar9),
                                                local_2a - uVar9);
                            local_a = CONCAT22((local_a._2_2_ - struct_a) - (local_a < uVar9),
                                               local_a - uVar9);
                            uVar13 = uVar14;
                            if (_local_32 == 0)
                            {
                                uVar6 = uVar9;
                                process_struct_1000_179c(10, struct_a);
                                uVar13 = (struct_a | uVar6);
                                if ((struct_a | uVar6) == 0)
                                {
                                    uVar6 = 0;
                                    uVar13 = 0;
                                }
                                else
                                {
                                    pass1_1020_ba3e(CONCAT22(struct_a, uVar6), 5, 5);
                                }
                                _local_32 = CONCAT22(uVar13, uVar6);
                            }
                            pass1_1020_bb8a(_local_32, (_local_32 >> 0x10), uVar9, uVar14,
                                            local_34);
                            lVar10 = (uVar11 & 0xffff | uVar12 << 0x10) - local_44;
                            pass1_1020_bb8a(local_2e, uVar4, lVar10, (lVar10 >> 0x10),
                                            local_34);
                            uVar9 = uVar13;
                            local_38 = local_34;
                            if (local_a == 0)
                            {
                                pass1_1038_1ac6(uVar3, uVar16, _local_e, _local_32);
                                _local_32 = 0;
                                puVar5 = &local_a;
                                pass1_1038_13da(uVar3, uVar16, CONCAT22(unaff_SS, puVar5),
                                                CONCAT22(unaff_SS, &local_6), param_3);
                                _local_e = CONCAT22(uVar9, puVar5);
                                uVar13 = (uVar9 | puVar5);
                                if ((uVar9 | puVar5) == 0)
                                {
                                    return;
                                }
                            }
                        }
                        unaff_CS = 0x1020;
                        if ((local_2a == 0) || (local_a == 0)) {
                            break;}
                        unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
                        local_34 = pass1_1028_0d80(uVar15);
                        if ((local_34 == local_38) || ((local_38 == 0 && (local_34 == uVar8)))) {
                            break;}
                    }
                    if (_local_32 != 0)
                    {
                        pass1_1038_1ac6(uVar3, uVar16, _local_e, _local_32);
                    }
                }
            }
            local_16 = local_16 + 1;
        }
    }
    return;
}




pub fn pass1_1038_1940(param_1: u32,param_2: u32,param_3: u32)

{
    let ppcVar1: fn();
    let mut uVar2: i32;
    let mut uVar3: i32;
    let puVar4: *mut u8;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let local_a: *mut astruct_1063;
    let mut local_8: u16;

    puVar4 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 3);
    uVar2 = puVar4;
    pass1_1038_4d6e(param_3, puVar4 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, uVar2);
    ppcVar1 = (*_local_a + 0x10);
    uVar3 = uVar2;
    (**ppcVar1)(&PTR_LOOP_1050_1008, uVar2, extraout_DX);
    if ((extraout_DX_00 | uVar3) != 0)
    {
        pass1_1038_1482(param_1, param_2, _local_a);
    }
    if (_local_a != 0x0)
    {
        ppcVar1 = *_local_a;
        (**ppcVar1)(&PTR_LOOP_1050_1008, uVar2, extraout_DX, 1);
    }
    return;
}




pub fn pass1_1038_19a0(param_1: u32,param_2: u32,param_3: u32)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let puVar5: *mut u8;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut local_a: u16;
    let mut local_8: u16;

    puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 2);
    uVar3 = puVar5;
    pass1_1038_4d6e(param_3, puVar5 & 0xffff | in_DX << 0x10);
    _local_a = CONCAT22(extraout_DX, uVar3);
    uVar2 = *_local_a;
    ppcVar1 = uVar2 + 8;
    uVar4 = uVar3;
    (**ppcVar1)(&PTR_LOOP_1050_1008, uVar3, extraout_DX);
    if ((extraout_DX_00 | uVar4) == 0)
    {
        wvsprintf_FUN_1030_840a(0xdf, &g_alloc_addr_1050_1050);
        if (_local_a != 0x0)
        {
            ppcVar1 = uVar2;
            (**ppcVar1)(0x1030, uVar3, extraout_DX, 1);
            return;
        }
    }
    else
    {
        pass1_1038_16f2(param_1, _local_a, param_2);
        if (_local_a != 0x0)
        {
            ppcVar1 = *_local_a;
            (**ppcVar1)(&PTR_LOOP_1050_1008, uVar3, extraout_DX, 1);
        }
    }
    return;
}




pub fn pass1_1038_1a30(param_1: u16, param_2: u16,param_1_00: u32)

{
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: i32;
    let mut uVar4: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut uVar5: u16;
    let mut unaff_CS: u16;
    let mut uVar6: i32;
    let mut uVar7: i32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar5 = (param_1_00 >> 0x10);
    puVar1 = (param_1_00 + 0x1e);
    uVar7 = (param_1_00 + 0x20);
    local_6._0_2_ = puVar1;
    uVar3 = uVar7 | local_6;
    if (uVar3 != 0)
    {
        ppcVar2 = (*puVar1 + 0x10);
        uVar6 = local_6;
        ppcVar2();
        _local_a = CONCAT22(extraout_DX, uVar3);
        local_12 = 0;
        while (local_12 < _local_a)
        {
            ppcVar2 = (*puVar1 + 4);
            uVar4 = _local_a;
            ppcVar2(unaff_CS, local_6, (puVar1 >> 0x10), local_12, uVar6, uVar7);
            if ((extraout_DX_00 | uVar4) != 0)
            {
                unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, extraout_DX_00);
            }
            local_12 = local_12 + 1;
        }
        return;
    }
    return;
}




pub fn pass1_1038_1ac6(param_1: u16, param_2: u16, param_1_00: *mut astruct_44,param_2_00: u32)

{
    let uVar1: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut local_120: u32;
    let mut local_11c: u32;
    let mut local_118: u16;
    let mut local_116: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = pass1_1028_b58e(param_1_00);
    _local_6 = CONCAT31(extraout_var, uVar1) & 0xffff | in_DX << 0x10;
    pas1_1030_e8a0(CONCAT22(unaff_SS, &local_118), param_2_00,
                   (param_1_00 + 0xc), (CONCAT31(extraout_var, uVar1) + 4));
    pass1_1028_d52c(*_g_bool_1050_5748, *_PTR_LOOP_1050_65e2 + 1, CONCAT22(unaff_SS, &local_118));
    return;
}




pub fn pass1_1038_1b3a(param_1: u16, param_2: u16, param_1_00: *mut astruct_44, param_2_00: *mut astruct_44) -> i32

{
    let uVar1: u8;
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

    uVar1 = pass1_1028_b58e(param_1_00);
    _local_6 = CONCAT31(extraout_var, uVar1) & 0xffff | in_DX << 0x10;
    local_a = param_1_00;
    _local_e = pass1_1028_45e2(param_1_00);
    local_10 = (param_2_00 + 4);
    local_12 = 0;
    while (local_12 < local_10)
    {
        pass1_1020_bb16(param_2_00, CONCAT22(unaff_SS, &local_1a), CONCAT22(unaff_SS, &local_16),
                        local_12);
        if (_local_e < local_1a)
        {
            pass1_1030_7ddc(_local_6, _local_e, local_16);
            _local_e = 0;
        }
        else
        {
            _local_e = _local_e - local_1a;
            pass1_1030_7ddc(_local_6, local_1a, local_16);
        }
        if (_local_e == 0){
            break;}
        local_12 = local_12 + 1;
    }
    if (param_2_00 != 0x0)
    {
        pass1_1020_ba7e(param_2_00);
        error_check_1000_17ce(param_2_00);
    }
    return;
}

pub fn pass1_1038_1c02(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}




pub fn pass1_1038_1c3e(param_1: u32,param_2: u32)

{
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let paVar4: *mut astruct_493;
    let mut iVar5: i32;
    let BVar6: bool;
    let puVar7: *mut u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut uVar8: i32;
    let mut uVar9: u16;
    let mut unaff_CS: u16;
    let mut uVar10: u32;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
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

    uVar9 = (param_2 >> 0x10);
    puVar1 = (param_2 + 0xc);
    uVar9 = (param_2 + 0xe);
    ppcVar2 = (*puVar1 + 0x10);
    puVar7 = puVar1;
    uVar13 = puVar1;
    ppcVar2();
    uVar3 = puVar7 & 0xffff | extraout_DX << 0x10;
    local_e = 0;
    loop
    {
        if (uVar3 <= local_e)
        {
            return;
        }
        ppcVar2 = (*puVar1 + 4);
        uVar10 = uVar3;
        ppcVar2(unaff_CS, puVar1, (puVar1 >> 0x10), local_e, uVar13, uVar9);
        uVar8 = extraout_DX_00 | uVar10;
        if (uVar8 != 0)
        {
            unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar10, extraout_DX_00);
            _local_1a = CONCAT22(uVar8, paVar4);
            iVar5 = &paVar4[1].field_0x16;
            if ((iVar5 != 0) && ((&paVar4[1].field_0x16 + 2) != 0))
            {
                uVar11 = param_1;
                uVar12 = (param_1 >> 0x10);
                pass1_1038_201a(uVar11, uVar12, CONCAT22(uVar8, paVar4));
                if (iVar5 == 0)
                {
                    uVar10 = pass1_1030_73a8(_local_1a);
                    iVar5 = (uVar10 + 0xc);
                    unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                    BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar5, 1);
                    if (BVar6 == 0)
                    {
                        unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                        BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar5, 2);
                        if (BVar6 == 0)
                        {
                            BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar5, 5);
                            if (BVar6 == 0)
                            {
                                unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                                BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar5, 6);
                                if (BVar6 == 0) {}
                                  // goto LAB_1038_1c76;
                            }
                            unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                            pass1_1038_2306(uVar11, uVar12, _local_1a);
                        }
                        else
                        {
                            pass1_1038_26ee(uVar11, uVar12, _local_1a);
                        }
                    }
                    else
                    {
                        pass1_1038_24e8(uVar11, uVar12, _local_1a);
                    }
                }
            }
        }
// LAB_1038_1c76:
        local_e = local_e + 1;
    } 
}




pub fn pass1_1038_1d68(param_1: u16, param_2: u16, param_1_00: *mut u32,param_2_00: u32)

{
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut uVar4: i32;
    let ppcVar5: fn();
    let Var6: u16;
    let mut uVar7: u32;
    let mut uVar8: i32;
    let mut bVar9: bool;
    let mut in_AX: u16;
    let local_AX_95: *mut astruct_1064;
    let puVar10: *mut u8;
    let local_AX_435: *mut astruct_1066;
    let mut uVar11: u32;
    let mut uVar13: i32;
    let mut uVar14: i32;
    let mut in_EDX: u32;
    let local_BX_521: *mut astruct_1067;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let puVar15: *mut u32;
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
    let mut local_30: [u8;4];
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: [u8;4];
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
    let temp_5f69f6c76a: *mut astruct_1065;
    let puVar12: *mut u8;

    local_6 = 100;
    local_8 = 0;
    ppcVar5 = (*param_1_00 + 0x10);
    puVar15 = param_1_00;
    (**ppcVar5)();
    _local_c = CONCAT22(in_EDX, in_AX);
    local_10 = 0;
    loop
    {
        if (_local_c <= local_10)
        {
            return;
        }
        ppcVar5 = (*param_1_00 + 4);
        uVar11 = _local_c;
        uVar14 = in_EDX;
        (**ppcVar5)(unaff_CS, param_1_00, local_10, puVar15);
        local_12 = uVar14;
        local_14 = uVar11;
        uVar14 = local_12 | local_14;
        in_EDX = uVar14;
        if (uVar14 != 0)
        {
            local_18 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_14, local_12);
            local_16 = uVar14;
            unaff_CS = 0x1030;
            local_1c = pass1_1030_73a8(CONCAT22(local_16, local_18));
            in_EDX = local_1c >> 0x10;
            puVar10 = local_20;
            ppcVar5 = (local_1c + 0x40);
            (**ppcVar5)(0x1030, local_1c, (local_1c >> 0x10), puVar10, unaff_SS);
            if (puVar10 == 0x0)
            {
                _local_24 = pass1_1028_62c8(local_1c);
                uVar11 = _local_24 >> 0x10;
                local_8 = 1;
                local_28 = (param_2_00 + 0x22);
                pass1_1008_5784(CONCAT22(unaff_SS, local_30), local_28);
                loop
                {
                    uVar14 = uVar11;
                    local_AX_95 = local_30;
                    unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
                    pass1_1008_5b12(CONCAT22(unaff_SS, local_AX_95));
                    _local_34 = CONCAT22(uVar14, local_AX_95);
                    in_EDX = (uVar14 | local_AX_95);
                    if ((uVar14 | local_AX_95) == 0) {
                        break;}
                    uVar2 = local_AX_95.field_0x4;
                    temp_5f69f6c76a = local_AX_95.field_0x6;
                    uVar3 = local_AX_95.field_0x8;
                    uVar13 = local_AX_95.field_0xc;
                    uVar4 = local_AX_95.field_0xa;
                    uVar8 = uVar13 / uVar4;
                    uVar11 = uVar13 % uVar4;
                    bVar9 = false;
                    if (((0 < temp_5f69f6c76a) && (!SBORROW2(temp_5f69f6c76a, 1))) &&
                        ((temp_5f69f6c76a == (&PTR_DAT_0005_0000_1050_0004 + 1) ||
                          (temp_5f69f6c76a + -1) < 4 ||
                          (temp_5f69f6c76a == &BYTE_1050_0008))))
                    {
                        bVar9 = true;
                    }
                    if (bVar9)
                    {
                        uVar11 = _local_24;
                        if (local_6 < _local_24)
                        {
                            uVar11 = local_6 & 0xffff;
                            local_22 = local_6._2_2_;
                        }
                        uVar13 = local_22 | uVar11;
                        in_EDX = uVar13;
                        if (uVar13 == 0) {
                            break;}
                        qVar6 = (uVar11 & 0xffff | local_22 << 0x10) / uVar8;
                        uVar13 = (qVar6 >> 0x10);
                        local_4e = qVar6;
                        if (local_4e < uVar4)
                        {
                            puVar1 = &local_AX_95.field_0xc;
                            *puVar1 = *puVar1 - uVar11;
                            puVar1 = &local_AX_95.field_0xa;
                            *puVar1 = *puVar1 - local_4e;
                        }
                        else
                        {
                            ppcVar5 = (local_28 + 0xc);
                            (**ppcVar5)(&PTR_LOOP_1050_1008, local_28, (local_28 >> 0x10), _local_34);
                            local_2c = 0;
                            local_4e = uVar4;
                        }
                        puVar12 = _PTR_LOOP_1050_68a2;
                        alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                        uVar14 = puVar12;
                        local_52 = puVar12 & 0xffff | uVar13 << 0x10;
                        if ((uVar13 | uVar14) == 0)
                        {
                            local_52 = 0;
                        }
                        else
                        {
                            local_52 = s_1_1050_389a;
                            (uVar14 + 2) = &PTR_LOOP_1050_1008;
                            (uVar14 + 4) = 0;
                            (uVar14 + 6) = 0;
                            (uVar14 + 8) = 0;
                            (uVar14 + 10) = 0;
                            (uVar14 + 0xc) = 0;
                            local_52 = 0x56ce;
                            (uVar14 + 2) = 0x1018;
                        }
                        uVar14 = (local_52 >> 0x10);
                        local_BX_521 = local_52;
                        local_BX_521.field_0xa = local_4e;
                        uVar7 = local_4e * uVar8;
                        uVar11 = uVar7 >> 0x10;
                        local_BX_521.field_0xc = uVar7;
                        local_BX_521.field_0x4 = uVar2;
                        local_BX_521.field_0x6 = temp_5f69f6c76a;
                        local_BX_521.field_0x8 = uVar3;
                        pass1_1028_6408(local_1c, (local_52 & 0xffff | uVar14 << 0x10));
                    }
                }
            }
            else
            {
                ppcVar5 = (*param_1_00 + 8);
                (**ppcVar5)(0x1030, param_1_00, 0, local_10);
            }
        }
        local_10 = local_10 + 1;
    } 
}




pub fn pass1_1038_1faa(param_1: u32,param_2: u32,param_3: u32)

{
    let ppcVar1: fn();
    let mut in_AX: u16;
    let paVar2: *mut astruct_493;
    let mut iVar3: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppcVar1 = (param_3 + 0x10);
    (**ppcVar1)();
    _local_6 = CONCAT22(extraout_DX, in_AX);
    local_a = 0;
    loop
    {
        if (_local_6 <= local_a)
        {
            return;
        }
        ppcVar1 = (param_3 + 4);
        uVar5 = _local_6;
        (**ppcVar1)();
        uVar4 = extraout_DX_00;
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_00);
        uVar5 = pass1_1030_73a8(CONCAT22(uVar4, paVar2));
        iVar3 = uVar5;
        pass1_1038_1d68(param_1, (param_1 >> 0x10), param_2, uVar5);
        if (iVar3 == 0) {
            break;}
        local_a = local_a + 1;
    }
    return;
}




pub fn pass1_1038_201a(param_1: u16, param_2: u16, param_1_00: *mut astruct_918)

{
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let ppcVar3: fn();
    let lVar4: u32;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let puVar7: *mut u8;
    let mut uVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u32;
    let mut uVar11: i32;
    let mut uVar12: u32;
    let mut uVar13: u32;
    let puVar14: *mut u8;
    let puVar15: *mut u8;
    let mut uVar16: u16;
    let puVar17: *mut u32;
    let struct_a: *mut astruct_922;
    let mut uVar18: u16;
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
    uVar18 = (param_1_00 >> 0x10);
    uVar16 = 0x1030;
    puVar17 = pass1_1030_6b16(struct_a);
    uVar6 = (puVar17 >> 0x10);
    uVar5 = puVar17;
    if ((uVar6 | uVar5) == 0)
    {
        return;
    }
    iVar2 = &struct_a.field_0x34;
    lVar4 = iVar2;
    uVar13 = lVar4 * 100;
    puVar7 = (uVar13 >> 0x10);
    local_a = 0;
    _local_14 = 0;
    if ((uVar5 + 4) == 0)
    {
        if ((uVar5 + 6) == 0)
        {
            if ((uVar5 + 8) == 0) {}
              // goto LAB_1038_2102;
            uVar9 = pass1_1020_c42e((uVar5 + 8));
            uVar12 =  * (uVar5 + 10) * uVar9;
            puVar14 = (uVar12 >> 0x10);
            if (uVar12 + lVar4 * -100 != 0 && uVar13 <= uVar12)
            {
                uVar12 = uVar13 & 0xffff;
                puVar14 = puVar7;
            }
            uVar13 = uVar12 & 0xffff | ZEXT24(puVar14) << 0x10;
            uVar10 = (uVar12 & 0xffff | ZEXT24(puVar14) << 0x10) / uVar9;
            piVar1 = (uVar5 + 10);
            *piVar1 = *piVar1 - uVar10;
            local_a = (uVar13 / 100);
            uVar13 = uVar13 % 100;
            uVar12 = uVar13;
            if (uVar13 != 0)
            {
                local_a = local_a + 1;
                uVar12 = local_a;
            }
            uVar8 = uVar12;
            process_struct_1000_179c(0x2a, uVar13);
            uVar11 = uVar13 | uVar8;
            if (uVar11 == 0) {}
              // goto LAB_1038_20fa;
            pass1_1038_6838(CONCAT22(uVar13, uVar8), uVar10, (uVar5 + 8),
                            local_a, &struct_a.field_0x4);
        }
        else
        {
            uVar9 = switch_statement_1020_c3b4((uVar5 + 6));
            uVar12 =  * (uVar5 + 10) * uVar9;
            puVar14 = (uVar12 >> 0x10);
            if (uVar12 + lVar4 * -100 != 0 && uVar13 <= uVar12)
            {
                uVar12 = uVar13 & 0xffff;
                puVar14 = puVar7;
            }
            uVar13 = uVar12 & 0xffff | ZEXT24(puVar14) << 0x10;
            uVar10 = (uVar12 & 0xffff | ZEXT24(puVar14) << 0x10) / uVar9;
            piVar1 = (uVar5 + 10);
            *piVar1 = *piVar1 - uVar10;
            local_a = (uVar13 / 100);
            uVar13 = uVar13 % 100;
            uVar12 = uVar13;
            if (uVar13 != 0)
            {
                local_a = local_a + 1;
                uVar12 = local_a;
            }
            uVar8 = uVar12;
            process_struct_1000_179c(0x2a, uVar13);
            uVar11 = uVar13 | uVar8;
            if (uVar11 == 0) {}
              // goto LAB_1038_20fa;
            pass1_1038_675c(CONCAT22(uVar13, uVar8), uVar10, (uVar5 + 6),
                            local_a, &struct_a.field_0x4);
        }
    }
    else
    {
        puVar14 = *(uVar5 + 10);
        puVar15 = 0x0;
        if ((puVar7 < 1) && ((0x7fff < puVar7 || (uVar13 < puVar14))))
        {
            puVar14 = uVar13;
            puVar15 = puVar7;
        }
        _local_18 = CONCAT22(puVar15, puVar14);
        piVar1 = (uVar5 + 10);
        *piVar1 = *piVar1 - puVar14;
        local_a = (_local_18 / 100);
        uVar12 = _local_18 % 100;
        uVar13 = uVar12;
        if (uVar12 != 0)
        {
            local_a = local_a + 1;
            uVar13 = local_a;
        }
        uVar8 = uVar13;
        process_struct_1000_179c(0x2a, uVar12);
        uVar11 = uVar12 | uVar8;
        if (uVar11 == 0)
        {
// LAB_1038_20fa:
            uVar16 = 0x1000;
            _local_14 = 0;
          // goto LAB_1038_2102;
        }
        pass1_1038_6590(CONCAT22(uVar12, uVar8), puVar14, puVar15,
                        *(uVar5 + 4), local_a, *&struct_a.field_0x4);
    }
    uVar16 = 0x1000;
    _local_14 = CONCAT22(uVar11, uVar8);
// LAB_1038_2102:
    if (_local_14 != 0)
    {
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        uVar16 = 0x1030;
        pass1_1030_6c4c(param_1_00, iVar2 - local_a);
    }
    if ((uVar5 + 10) == 0)
    {
        if ((uVar6 | uVar5) != 0)
        {
            ppcVar3 = *puVar17;
            (**ppcVar3)(uVar16, uVar5, uVar6, 1);
        }
    }
    else
    {
        pass1_1030_6c66(param_1_00, 0, puVar17);
    }
    return;
}




pub fn pass1_1038_2306(param_1: u16, param_2: u16, param_1_00: *mut astruct_493)

{
    let piVar1: *mut i32;
    let puVar2: *mut u32;
    let ppcVar3: fn();
    let Var4: u16;
    let puVar5: *mut u32;
    let mut uVar6: u32;
    let puVar7: *mut u32;
    let mut uVar8: i32;
    let mut uVar9: i32;
    let mut uVar10: u32;
    let local_BX_19: *mut astruct_1068;
    let mut iVar11: i32;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u32;
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

    uVar13 = 0x1030;
    uVar14 = pass1_1030_73a8(param_1_00);
    uVar10 = uVar14 >> 0x10;
    uVar12 = (param_1_00 >> 0x10);
    local_BX_19 = param_1_00;
    local_8 = local_BX_19.field_0x34;
    local_c = 100;
    puVar2 = (uVar14 + 0x22);
    puVar7 = puVar2;
    loop
    {
        uVar8 = uVar10;
        ppcVar3 = (*puVar2 + 0x10);
        (**ppcVar3)(uVar13, puVar2, (puVar2 >> 0x10));
        uVar9 = puVar7;
        uVar14 = puVar7 & 0xffff;
        puVar5 = (uVar14 | uVar8 << 0x10);
        if ((uVar8 | uVar9) == 0) {
            break;}
        if ((uVar9 + 10) == 0)
        {
            uVar10 = (uVar8 | uVar9);
            if ((uVar8 | uVar9) != 0)
            {
                ppcVar3 = *puVar5;
                (**ppcVar3)(uVar13, uVar9, uVar8, 1);
            }
        }
        else
        {
            local_18 = 0;
            local_1e = 0;
            if ((uVar9 + 6) == 0)
            {
                if ((uVar9 + 8) != 0)
                {
                    local_1e = pass1_1020_c42e((uVar9 + 8));
                  // goto LAB_1038_2385;
                }
            }
            else
            {
                local_1e = switch_statement_1020_c3b4((uVar9 + 6));
// LAB_1038_2385:
                uVar13 = 0x1020;
                local_18 = ((uVar9 + 10) * local_1e);
            }
            local_c._2_2_ = 0;
            if (local_c < local_18)
            {
                local_18 = local_c & 0xffff;
            }
            _local_22 = local_18 | local_c._2_2_ << 0x10;
            uVar10 = local_18 | local_c._2_2_ << 0x10;
            qVar4 = uVar10 / local_1e;
            uVar6 = qVar4;
            uVar10 = uVar10 % local_1e;
            piVar1 = (uVar9 + 10);
            *piVar1 = *piVar1 - qVar4;
            if (*piVar1 == 0)
            {
                uVar10 = (uVar8 | uVar9);
                if ((uVar8 | uVar9) != 0)
                {
                    ppcVar3 = *puVar5;
                    (**ppcVar3)(uVar13, uVar9, uVar8, 1);
                }
            }
            else
            {
                ppcVar3 = (*puVar2 + 8);
                (**ppcVar3)();
            }
            local_c = local_c - _local_22;
            puVar7 = 0x0;
            local_2a = 0;
            iVar11 = uVar14;
            if ((iVar11 + 6) == 0)
            {
                if ((iVar11 + 8) != 0)
                {
                    process_struct_1000_179c(0x2a, uVar10);
                    uVar9 = uVar10 | puVar7;
                    uVar14 = uVar9;
                    if (uVar9 == 0) {}
                      // goto LAB_1038_244e;
                    pass1_1038_6838((puVar7 & 0xffff | uVar10 << 0x10), uVar6,
                                    (iVar11 + 8), 1, local_BX_19.field_0x4);
                  // goto LAB_1038_24b3;
                }
            }
            else
            {
                process_struct_1000_179c(0x2a, uVar10);
                uVar9 = uVar10 | puVar7;
                uVar14 = uVar9;
                if (uVar9 == 0)
                {
// LAB_1038_244e:
                    uVar13 = 0x1000;
                    local_2a = 0;
                    uVar10 = uVar14;
                }
                else
                {
                    pass1_1038_675c((puVar7 & 0xffff | uVar10 << 0x10), uVar6,
                                    (iVar11 + 6), 1, local_BX_19.field_0x4);
// LAB_1038_24b3:
                    uVar13 = 0x1000;
                    local_2a = puVar7 & 0xffff | uVar14 << 0x10;
                    uVar10 = uVar14;
                }
            }
            if (local_2a != 0)
            {
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                local_8 = local_8 - 1;
                if (local_8 == 0) {
                    break;}
                local_c = 100;
            }
        }
    }
    pass1_1030_6c4c(param_1_00, local_8);
    return;
}




pub fn pass1_1038_24e8(param_1: u16, param_2: u16, param_1_00: *mut astruct_493)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let struct_a: *mut astruct_199;
    let mut extraout_DX_00: i32;
    let mut uVar4: i32;
    let struct_a_00: *mut astruct_199;
    let mut extraout_DX_01: i32;
    let paVar5: *mut astruct_199;
    let local_BX_19: *mut astruct_1069;
    let mut uVar6: u16;
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
    uVar6 = (param_1_00 >> 0x10);
    local_BX_19 = param_1_00;
    local_8 = local_BX_19.field_0x34;
    local_c = (_local_6 + 0x28);
    local_10 = 100;
    local_12 = (local_c + 4);
    uVar2 = local_12;
    process_struct_1000_179c(10, paVar5);
    uVar4 = uVar2;
    if ((paVar5 | uVar4) == 0)
    {
        uVar4 = 0;
        uVar3 = 0;
    }
    else
    {
        pass1_1020_ba3e((uVar2 & 0xffff | ZEXT24(paVar5) << 0x10), 5, 5);
        uVar3 = extraout_DX;
    }
    _local_1c = CONCAT22(uVar3, uVar4);
    local_1e = 0;
    while (uVar2 = local_12, local_1e < local_12)
    {
        pass1_1020_bb16(local_c,
                        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_18)),
                        CONCAT22(unaff_SS, &local_14), local_1e);
        if (local_18 != 0)
        {
            uVar2 = local_18;
            local_10._2_2_ = local_18._2_2_;
            if (local_10 < local_18)
            {
                uVar2 = local_10 & 0xffff;
            }
            uVar4 = uVar2;
            uVar2 = uVar2 & 0xffff | local_10._2_2_ << 0x10;
            iVar1 = (local_18._2_2_ - local_10._2_2_) - (local_18 < uVar4);
            local_18 = CONCAT22(iVar1, local_18 - uVar4);
            pass1_1020_bb8a(local_c, (local_c >> 0x10), local_18 - uVar4, iVar1, local_14);
            pass1_1020_bb70(_local_1c, uVar4, CONCAT22(local_14, local_10._2_2_));
            local_10 = local_10 - uVar2;
            if (local_10 == 0)
            {
                paVar5 = struct_a_00;
                process_struct_1000_179c(0x2a, struct_a_00);
                if ((paVar5 | uVar2) == 0)
                {
                    uVar2 = 0;
                }
                else
                {
                    pass1_1038_666e((uVar2 & 0xffff | ZEXT24(paVar5) << 0x10),
                                    _local_1c, 1, local_BX_19.field_0x4);
                }
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                paVar5 = struct_a;
                process_struct_1000_179c(10, struct_a);
                if ((paVar5 | uVar2) == 0)
                {
                    uVar2 = 0;
                    uVar4 = 0;
                }
                else
                {
                    pass1_1020_ba3e((uVar2 & 0xffff | ZEXT24(paVar5) << 0x10), 5, 5);
                    uVar4 = extraout_DX_00;
                }
                _local_1c = (uVar2 & 0xffff | uVar4 << 0x10);
                local_8 = local_8 - 1;
                if (local_8 == 0){
                    break;}
                local_10 = 100;
            }
        }
        local_1e = local_1e + 1;
    }
    infinite_loop_1020_ba94(_local_1c, (_local_1c >> 0x10));
    paVar5 = (extraout_DX_01 | uVar2);
    if (paVar5 == 0x0)
    {
        if (_local_1c != 0x0)
        {
            pass1_1020_ba7e(_local_1c);
            error_check_1000_17ce(_local_1c);
        }
    }
    else
    {
        process_struct_1000_179c(0x2a, paVar5);
        if ((paVar5 | uVar2) != 0)
        {
            pass1_1038_666e((uVar2 & 0xffff | ZEXT24(paVar5) << 0x10), _local_1c, 1,
                            local_BX_19.field_0x4);
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    }
    pass1_1030_6c4c(param_1_00, local_8);
    return;
}




pub fn pass1_1038_26ee(param_1: u16, param_2: u16, param_1_00: *mut astruct_493)

{
    let mut uVar1: i32;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let extraout_DX: *mut astruct_199;
    let paVar4: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let struct_a_00: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let mut extraout_DX_01: i32;
    let paVar5: *mut astruct_199;
    let local_BX_19: *mut astruct_1070;
    let mut uVar6: u16;
    let mut uVar7: u32;
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

    uVar7 = pass1_1030_73a8(param_1_00);
    paVar5 = (uVar7 >> 0x10);
    uVar6 = (param_1_00 >> 0x10);
    local_BX_19 = param_1_00;
    local_8 = local_BX_19.field_0x34;
    local_c = pass1_1028_0d80(uVar7);
    uVar3 = local_c;
    local_10 = 100;
    process_struct_1000_179c(10, paVar5);
    if ((paVar5 | uVar3) == 0)
    {
        uVar3 = 0;
        paVar5 = 0x0;
    }
    else
    {
        pass1_1020_ba3e((uVar3 & 0xffff | ZEXT24(paVar5) << 0x10), 5, 5);
        paVar5 = extraout_DX;
    }
    _local_14 = (uVar3 & 0xffff | ZEXT24(paVar5) << 0x10);
    local_a = local_c;
    while
    {
        uVar1 = uVar3;
        pass1_1030_7c28(param_1_00, local_a);
        paVar4 = (paVar5 | uVar1);
        if (paVar4 != 0x0)
        {
            paVar4 = paVar5;
            uVar2 = uVar1;
            if ((local_10._2_2_ <= paVar5) && ((local_10._2_2_ < paVar5 || (local_10 < uVar1))))
            {
                paVar4 = local_10._2_2_;
                uVar2 = local_10;
            }
            _local_24 = CONCAT22(paVar4, uVar2);
            pass1_1030_7d1c(param_1_00, uVar1 - uVar2,
                            CONCAT22(local_a, paVar5 + (-(uVar1 < uVar2) - paVar4)));
            pass1_1020_bb70(_local_14, uVar2, CONCAT22(local_a, paVar4));
            local_10 = local_10 - _local_24;
            paVar4 = struct_a;
            if (local_10 == 0)
            {
                paVar5 = struct_a;
                process_struct_1000_179c(0x2a, struct_a);
                local_a = _local_24;
                if ((paVar5 | local_a) == 0)
                {
                    local_a = 0;
                }
                else
                {
                    pass1_1038_666e((_local_24 & 0xffff | ZEXT24(paVar5) << 0x10),
                                    _local_14, 1, local_BX_19.field_0x4);
                }
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                paVar5 = struct_a_00;
                process_struct_1000_179c(10, struct_a_00);
                if ((paVar5 | local_a) == 0)
                {
                    local_a = 0;
                    paVar4 = 0x0;
                }
                else
                {
                    pass1_1020_ba3e(CONCAT22(paVar5, local_a), 5, 5);
                    paVar4 = extraout_DX_00;
                }
                _local_14 = CONCAT22(paVar4, local_a);
                local_8 = local_8 - 1;
                if (local_8 == 0) {
                    break;}
                local_10 = 100;
            }
        }
        local_a = pass1_1028_0d80(uVar7);
        uVar3 = local_a;
        if (local_c == 0)
        {
            local_c = local_a;
        }
        paVar5 = paVar4;
        local_c != local_a
    } {}
    uVar1 = (_local_14 >> 0x10);
    infinite_loop_1020_ba94(_local_14, uVar1);
    paVar5 = (extraout_DX_01 | local_a);
    if (paVar5 == 0x0)
    {
        if ((uVar1 | _local_14) != 0)
        {
            pass1_1020_ba7e(_local_14);
            error_check_1000_17ce(_local_14);
        }
    }
    else
    {
        process_struct_1000_179c(0x2a, paVar5);
        if ((paVar5 | local_a) != 0)
        {
            pass1_1038_666e(CONCAT22(paVar5, local_a), _local_14, 1,
                            local_BX_19.field_0x4);
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    }
    pass1_1030_6c4c(param_1_00, local_8);
    return;
}

pub fn pass1_1038_28d8(param_1: *mut astruct_500) -> *mut astruct_500

{
    pass1_1028_d1dc(param_1, (s_0_023_1050_3a93 + 4));
    param_1.a = (s_fem110_wav_1050_29fa + 4);
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)),
                          s_SCRoboMove_1050_59f8);
    return param_1;
}



pub fn pass1_1038_290e()

{
    let paVar1: *mut astruct_493;
    let mut in_DX: i32;
    let mut local_4: u16;

    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
    if ((in_DX | paVar1) != 0)
    {
        pass1_1038_4918(CONCAT22(in_DX, paVar1));
    }
    pass1_1038_7a76(_PTR_LOOP_1050_5a64);
    return 1;
}


pub fn pass1_1038_0c00(param_1: u32)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let puVar3: *mut u16;
    let mut uVar4: i32;
    let puVar5: *mut u8;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut uVar6: i32;
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
                    (&PTR_LOOP_1050_0000 + 1), 0, 0x400);
    loop
    {
        puVar3 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar3));
        _local_18 = CONCAT22(extraout_DX, puVar3);
        if ((extraout_DX | puVar3) == 0) {
            break;}
        pass1_1038_0e78(param_1, CONCAT22(extraout_DX, puVar3));
        pass1_1038_1220(param_1, CONCAT22(extraout_DX, puVar3));
        uVar6 = extraout_DX_00;
        puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 1);
        uVar4 = puVar5;
        pass1_1038_4d6e(CONCAT22(extraout_DX, puVar3),
                        puVar5 & 0xffff | uVar6 << 0x10);
        _local_20 = CONCAT22(extraout_DX_01, uVar4);
        ppcVar1 = (*_local_20 + 0x10);
        uVar6 = uVar4;
        (**ppcVar1)(&PTR_LOOP_1050_1008, uVar4, extraout_DX_01);
        if ((extraout_DX_02 | uVar6) != 0)
        {
            uVar2 = (param_1 + 0x108);
            if ((uVar2 + 0x82) != 0)
            {
                pass1_1038_19a0(param_1, CONCAT22(extraout_DX_01, uVar4), CONCAT22(extraout_DX, puVar3));
            }
            pass1_1038_1940(param_1, CONCAT22(extraout_DX_01, uVar4), _local_18);
        }
        if (_local_20 != 0x0)
        {
            ppcVar1 = *_local_20;
            (**ppcVar1)(8, uVar4, extraout_DX_01, 1);
        }
        pass1_1038_1c3e(param_1, _local_18);
    }
    return;
}


pub fn pass1_1038_0b6a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1038_008e(param_1: u16, param_2: u16, param_1_00: *mut astruct_1050)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut uvar3: u16;
    let mut iVar4: i32;
    let local_AX_270: *mut astruct_1052;
    let mut uVar5: i32;
    let local_BX_4: *mut astruct_1050;
    let local_ES_4: *mut astruct_1050;
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
    let temp_7ff08964032: *mut astruct_1052;
    let mut temp_7ff94c183f6: i32;
    let mut uVar6: u32;

    local_ES_4 = (param_1_00 >> 0x10);
    local_BX_4 = param_1_00;
    if (local_BX_4.field_0x4 != 0x4000001)
    {
        return;
    }
    ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffd4, 0x2c));
    temp_7ff94c183f6 = ppVar7;
    uVar3 = (ppVar7 >> 0x10);
    iVar4 = temp_7ff94c183f6;
    pass1_fn_1008_612e(1, 100);
    local_c = 0;
    iVar1 = (temp_7ff94c183f6 + 10);
    if (iVar1 == 1)
    {
        local_c = 0x15;
    }
    else
    {
        if (iVar1 != 2)
        {
            if (iVar1 == 3)
            {
                local_c = 0x16;
            }
            else
            {
                if (iVar1 == 4)
                {
                    if (iVar4 < 0x32)
                    {
                        local_c = 0x17;
                    }
                    else
                    {
                        local_c = 0x18;
                    }
                }
                else
                {
                    if (iVar1 == 5)
                    {
                        local_c = 0x19;
                    }
                }
            }
        }
    }
    if (local_c != 0)
    {
        ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffd4, 0x2b));
        pass1_1010_043a(ppVar8, local_BX_4.field_0x4, local_c);
    }
    pass1_1008_eb74(ppVar7, 0);
    if ((((temp_7ff94c183f6 + 0xe) | (temp_7ff94c183f6 + 0xc)) == 0) &&
        (local_BX_4.field_0x18 < 0xc9))
    {
        uVar2 = *_PTR_LOOP_1050_65e2;
        uVar6 = uVar2;
        pass1_fn_1008_612e(0, 8);
        uVar5 = uVar6;
        local_22._0_2_ = uVar2;
        local_22._2_2_ = (uVar2 >> 0x10);
        (temp_7ff94c183f6 + 0xc) = uVar5 + local_22 + 0x1e;
        (temp_7ff94c183f6 + 0xe) =
            (uVar5 >> 0xf) + local_22._2_2_ + CARRY2(uVar5, local_22) +
            (0xffe1 < uVar5 + local_22);
    }
    return;
}




pub fn pass1_1038_01c0(param_1: u16, param_2: u16,param_1_00: u32)

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let ppcVar3: fn();
    let mut uVar4: u32;
    let paVar5: *mut astruct_493;
    let mut uVar6: u16;
    let BVar7: bool;
    let puVar8: *mut u8;
    let puVar9: *mut u8;
    let puVar10: *mut u32;
    let mut uVar11: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut unaff_SS: u16;
    let mut uVar14: u32;
    let mut uVar15: u32;
    let uVar16: u8;
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
    let mut local_e: [u8;2];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x29);
    uVar6 = puVar9;
    local_8 = uVar6;
    pass1_1038_4e78(param_1_00, puVar9 & 0xffff | in_DX << 0x10);
    _local_c = CONCAT22(extraout_DX, uVar6);
    uVar13 = 0x1030;
    uVar14 = pass1_1030_bcae(local_e, unaff_SS);
    uVar12 = uVar14;
    ppcVar3 = (*_local_c + 0x10);
    (**ppcVar3)(0x1030, _local_c, (_local_c >> 0x10));
    _local_12 = CONCAT22(extraout_DX_00, uVar12);
    uVar12 = (param_1_00 >> 0x10);
    puVar2 = (param_1_00 + 0xc);
    uVar12 = (param_1_00 + 0xe);
    uVar16 = SUB41(puVar2, 0);
    ppcVar3 = (*puVar2 + 0x10);
    puVar10 = puVar2;
    (**ppcVar3)();
    uVar4 = puVar10 & 0xffff | extraout_DX_01 << 0x10;
    local_1e = 0;
    loop
    {
        if (_local_12 <= local_1e)
        {
            if (_local_c != 0x0)
            {
                ppcVar3 = *_local_c;
                (**ppcVar3)(uVar13, _local_c, (_local_c >> 0x10), 1, uVar16, uVar12);
            }
            return;
        }
        uVar13 = 0x1030;
        uVar11 = _local_12;
        pass1_1030_1d58(_local_c);
        iVar1 = (uVar11 + 0x10);
        local_32 = 0;
        while (local_32 < uVar4)
        {
            uVar13 = 0x1030;
            uVar15 = uVar4;
            pass1_1030_1d58(puVar2);
            paVar5 = (uVar15 & 0xffff | extraout_DX_02 << 0x10);
            if (((extraout_DX_02 | uVar15) != 0) && ((uVar15 + 0x10) == iVar1))
            {
                uVar15 = pass1_1030_73a8(paVar5);
                uVar13 = SUB42(&PTR_LOOP_1050_1008, 0);
                BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar15 + 0xc), 0x30);
                if (BVar7 == 0)
                {
                    puVar8 = local_e;
                    uVar13 = 0x1030;
                    pass1_1030_bd74(puVar8, unaff_SS, paVar5,
                                    (uVar11 & 0xffff | extraout_DX_03 << 0x10));
                    if (puVar8 < 6)
                    {
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




pub fn pass1_1038_0340(param_1: u16, param_2: u16, param_1_00: i32,param_2_00: u32)

{
    let mut uVar1: i32;
    let mut uVar2: u32;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_13a: [u8;284];
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
    uVar4 = (param_2_00 >> 0x10);
    pass1_1038_4cea(param_2_00, CONCAT22(unaff_SS, &local_12), CONCAT22(unaff_SS, &local_e));
    uVar2 = (iVar3 + 0x1f6);
    local_16 = uVar2;
    pass1_1030_38b8(uVar2, (uVar2 >> 0x10));
    uVar1 = uVar2;
    _local_1a = uVar2 & 0xffff | in_DX << 0x10;
    if (param_1_00 == 0)
    {
        if (local_e != 8)
        {
            local_a = (uVar2 & 0xffff | in_DX << 0x10) / 4;
            local_c = 8;
          // goto LAB_1038_054b;
        }
    }
    else
    {
        if (param_1_00 < 0xb)
        {
            if (local_e != 7)
            {
                local_a = (uVar2 & 0xffff | in_DX << 0x10) / 10;
                local_c = 7;
              // goto LAB_1038_054b;
            }
        }
        else
        {
            if (param_1_00 < 0x1a)
            {
                if (local_e != 6)
                {
                    local_a = (uVar2 & 0xffff | in_DX << 0x10) / 0x14;
                    local_c = 6;
                  // goto LAB_1038_054b;
                }
            }
            else
            {
                if (param_1_00 < 0x33)
                {
                    if (local_e != 5)
                    {
                        local_a = (uVar2 & 0xffff | in_DX << 0x10) / 100;
                        local_c = 5;
                      // goto LAB_1038_054b;
                    }
                }
                else
                {
                    if (param_1_00 < 0x4c)
                    {
                        if (local_6 % 3 != 0) {}
                          // goto LAB_1038_054b;
                        if (local_e != 4)
                        {
                            local_a = _local_1a / 100;
                            local_c = 4;
                          // goto LAB_1038_054b;
                        }
                    }
                    else
                    {
                        if (param_1_00 < 0x65)
                        {
                            if (local_6 % 5 != 0) {}
                              // goto LAB_1038_054b;
                            if (local_e != 3)
                            {
                                local_a = _local_1a / 100;
                                local_c = 3;
                              // goto LAB_1038_054b;
                            }
                        }
                        else
                        {
                            if (param_1_00 < 0x97)
                            {
                                if (local_6 % 10 != 0) {}
                                  // goto LAB_1038_054b;
                                if (local_e != 2)
                                {
                                    local_a = _local_1a / 100;
                                    local_c = 2;
                                  // goto LAB_1038_054b;
                                }
                            }
                            else
                            {
                                if ((200 < param_1_00) || (local_6 % 0x14 != 0)) {}
                                  // goto LAB_1038_054b;
                                if (local_e != 1)
                                {
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
    if ((local_10 <= in_DX) && ((local_10 < in_DX || (local_12 < uVar1))))
    {
        uVar1 = local_12;
        in_DX = local_10;
    }
    local_a = CONCAT22(in_DX, uVar1);
// LAB_1038_054b:
    if (local_c != 0)
    {
        if ((_local_1a != 0) && (local_a == 0))
        {
            local_a = 1;
        }
        pass1_1038_4cd0(param_2_00, local_a, local_c);
    }
    if ((local_a._2_2_ | local_a) != 0)
    {
        if ((iVar3 + 0x200) == 0x8000001)
        {
            local_1e = 2;
        }
        else
        {
            local_1e = 1;
        }
        _local_1e = CONCAT22(0x400, local_1e);
        pass1_1028_9944(CONCAT22(unaff_SS, local_13a), local_a, CONCAT22(0x400, local_1e),
                        (iVar3 + 4));
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, local_13a));
        pass1_1028_9992(CONCAT22(unaff_SS, local_13a));
    }
    return;
}




pub fn pass1_1038_05d8(param_1: u16, param_2: u16, param_1_00: i32,param_2_00: u32)

{
    let puVar1: *mut u16;
    let mut uVar2: u32;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let mut in_EDX: u32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_158: [u8;280];
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
    pass1_1038_4cea(param_2_00, CONCAT22(unaff_SS, &local_12), CONCAT22(unaff_SS, &local_e));
    local_16 = 0;
    local_1a = 0;
    local_1e = 0;
    pass1_1028_dc52(
                        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_34)),
                    (&PTR_LOOP_1050_0000 + 1), 0, 0x400);
    while
    {
        while
        {
            uVar3 = in_EDX;
            puVar1 = &local_34;
            pass1_1028_e4ec(CONCAT22(unaff_SS, puVar1));
            _local_22 = CONCAT22(uVar3, puVar1);
            uVar4 = uVar3 | puVar1;
            in_EDX = uVar4;
            if (uVar4 == 0){}
              // goto LAB_1038_0668;
            (puVar1 + 0x100) != 0x8000002
        } {}
        local_16 = CONCAT22(uVar3, puVar1);
        uVar2 = (puVar1 + 0xfb);
        local_1a = uVar2;
        pass1_1030_38b8(uVar2, (uVar2 >> 0x10));
        local_1e = uVar2 & 0xffff | uVar4 << 0x10;
        uVar4 = uVar4 | uVar2;
        in_EDX = uVar4;
        uVar4 == 0
    } {}
// LAB_1038_0668:
    local_34 = s_1_1050_389a;
    local_32 = &PTR_LOOP_1050_1008;
    if ((local_16._2_2_ | local_16) == 0)
    {
        return;
    }
    if (param_1_00 == 1000)
    {
        if (local_e != 0x10)
        {
            local_a = local_1e / 4;
            local_c = 0x10;
          // goto LAB_1038_0841;
        }
    }
    else
    {
        if (param_1_00 < 0x3de)
        {
            if (param_1_00 < 0x3cf)
            {
                if (param_1_00 < 0x3b6)
                {
                    if (param_1_00 < 0x39d)
                    {
                        if (param_1_00 < 900)
                        {
                            if (param_1_00 < 0x352)
                            {
                                if ((param_1_00 < 800) || (local_6 % 0x14 != 0)) {}
                                  // goto LAB_1038_0841;
                                if (local_e != 9)
                                {
                                    local_a = local_1e / 100;
                                    local_c = 9;
                                  // goto LAB_1038_0841;
                                }
                            }
                            else
                            {
                                if (local_6 % 10 != 0) {}
                                  // goto LAB_1038_0841;
                                if (local_e != 10)
                                {
                                    local_a = local_1e / 100;
                                    local_c = 10;
                                  // goto LAB_1038_0841;
                                }
                            }
                        }
                        else
                        {
                            if (local_6 % 5 != 0) {}
                              // goto LAB_1038_0841;
                            if (local_e != 0xb)
                            {
                                local_a = local_1e / 100;
                                local_c = 0xb;
                              // goto LAB_1038_0841;
                            }
                        }
                    }
                    else
                    {
                        if (local_6 % 3 != 0) {}
                          // goto LAB_1038_0841;
                        if (local_e != 0xc)
                        {
                            local_a = local_1e / 100;
                            local_c = 0xc;
                          // goto LAB_1038_0841;
                        }
                    }
                }
                else
                {
                    if (local_e != 0xd)
                    {
                        local_a = local_1e / 100;
                        local_c = 0xd;
                      // goto LAB_1038_0841;
                    }
                }
            }
            else
            {
                if (local_e != 0xe)
                {
                    local_a = local_1e / 0x14;
                    local_c = 0xe;
                  // goto LAB_1038_0841;
                }
            }
        }
        else
        {
            if (local_e != 0xf)
            {
                local_a = local_1e / 10;
                local_c = 0xf;
              // goto LAB_1038_0841;
            }
        }
    }
    uVar2 = local_1e;
    if (local_12 < local_1e)
    {
        uVar2 = local_12 & 0xffff;
        local_1e._2_2_ = local_12._2_2_;
    }
    local_a = uVar2 & 0xffff | local_1e._2_2_ << 0x10;
// LAB_1038_0841:
    if (local_c != 0)
    {
        if ((local_1e != 0) && (local_a == 0))
        {
            local_a = 1;
        }
        pass1_1038_4cd0(param_2_00, local_a, local_c);
    }
    if ((local_a._2_2_ | local_a) != 0)
    {
        uVar5 = (param_2_00 >> 0x10);
        if ((param_2_00 + 0x200) == 0x8000001)
        {
            local_40 = (local_16 + 4);
        }
        else
        {
            local_40 = 0x4000001;
        }
        pass1_1028_9944(CONCAT22(unaff_SS, local_158), local_a,
                        (param_2_00 + 4), local_40);
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, local_158));
        pass1_1028_9992(CONCAT22(unaff_SS, local_158));
    }
    return;
}

pub fn pass1_1038_08d4(param_1: u16,param_2: u32,param_3: u32)

{
    let mut unaff_SS: u16;
    let lVar1: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: u16;

    local_4 = 0;
    pass1_1028_dc52(CONCAT22(unaff_SS, &local_16), (&PTR_LOOP_1050_0000 + 1), 0, 0x400);
    while
    {
        lVar1 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_16));
        if (lVar1 == 0) {}
          // goto LAB_1038_0917;
          (lVar1 + 0x200) != 0x8000002
    } {}
    local_4 = 1;
// LAB_1038_0917:
    local_16 = s_1_1050_389a;
    local_14 = &PTR_LOOP_1050_1008;
    if (local_4 != 0)
    {
        if (param_2 < 0xc90000)
        {
            pass1_1038_0340(param_1, param_2, param_2._2_2_, param_3);
            return;
        }
        if (0x31fffff < param_2)
        {
            pass1_1038_05d8(param_1, param_2, param_2._2_2_, param_3);
        }
    }
    return;
}


pub fn pass1_1030_ecc2(param_1: *mut astruct_500) -> *mut astruct_500

{
    pass1_1028_d1dc(param_1, 0xf9f);
    param_1.a = 0xb96;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCMorale_1050_59ce);
    return param_1;
}





pub fn pass1_1030_ecf8(param_1: u32)

{
    let mut uVar1: u16;
    let mut iVar2: i32;
    let puVar3: *mut u32;
    let ppcVar4: fn();
    let mut uVar5: u16;
    let mut uVar6: u32;
    let local_AX_122: *mut astruct_1045;
    let paVar7: *mut astruct_493;
    let mut uVar8: u16;
    let mut iVar9: i32;
    let mut uVar10: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut uVar11: i32;
    let mut uVar12: i32;
    let mut uVar13: u16;
    let mut unaff_SS: u16;
    let mut bVar14: bool;
    let ppVar15: *mut pass1_struct_1;
    let puVar16: *mut u32;
    let mut uVar17: u16;
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
    ppVar15 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_68, 0x2f));
    local_4 = (ppVar15 >> 0x10);
    local_a = ppVar15;
    local_6 = local_a;
    pass1_1010_ed3e(ppVar15);
    local_8 = extraout_DX;
    if ((extraout_DX | local_a) != 0)
    {
        local_c = pass1_1030_2aaa(CONCAT22(extraout_DX, local_a));
    }
    if (local_c < 2)
    {
        local_c = 0;
    }
    ppVar15 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_68, 2));
    local_e = (ppVar15 >> 0x10);
    local_10 = ppVar15;
    if ((0 < u16_1050_13ae) && (!SBORROW2(u16_1050_13ae, 1)))
    {
        if (u16_1050_13ae == 2 || (u16_1050_13ae - 1) < 1)
        {
            if (6 < local_c)
            {
                local_c = local_c - 2;
              // goto LAB_1030_ed5b;
            }
            bVar14 = SBORROW2(local_c, 4);
            iVar2 = local_c - 4;
        }
        else
        {
            if (u16_1050_13ae != 3) {}
              // goto LAB_1030_ed5b;
            bVar14 = SBORROW2(local_c, 7);
            iVar2 = local_c - 7;
        }
        if (bVar14 == iVar2 < 0)
        {
            local_c = local_c - 1;
        }
    }
// LAB_1030_ed5b:
    pass1_1028_dc52(
                        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_22)),
                    (&PTR_LOOP_1050_0000 + 1), 0, 0x400);
    loop
    {
        local_AX_122 = &local_22;
        pass1_1028_e4ec(CONCAT22(unaff_SS, local_AX_122));
        _local_26 = CONCAT22(extraout_DX_00, local_AX_122);
        if ((extraout_DX_00 | local_AX_122) == 0) {
            break;}
        uVar11 =  * &local_AX_122.field_0x1f6;
        if (((local_AX_122.field_0x1fe != 0) && (local_AX_122.field_0x200 != 0x8000002)) &&
            (pass1_1030_38b8(uVar11, &local_AX_122.field_0x1f8),
             (extraout_DX_01 | uVar11) != 0))
        {
            puVar3 = &local_AX_122.field_0xc;
            ppcVar4 = (*puVar3 + 0x10);
            puVar16 = puVar3;
            (**ppcVar4)(&PTR_LOOP_1050_1028, puVar3, &local_AX_122.field_0xe);
            uVar6 = puVar16 & 0xffff | extraout_DX_02 << 0x10;
            local_36 = local_AX_122.field_0x18;
            uVar13 = SUB42(&PTR_LOOP_1050_1038, 0);
            pass1_1038_4760(local_AX_122, extraout_DX_00);
            local_38 = local_AX_122.field_0x22 / 10;
            uVar1 = local_AX_122.field_0x24;
            if (uVar1 < 0x33)
            {
                if (uVar1 < 0x32)
                {
                    local_38 = local_38 - 1;
                }
            }
            else
            {
                local_36 = local_36 + 1;
            }
            local_40 = 0;
            while (local_40 < uVar6)
            {
                ppcVar4 = (*puVar3 + 4);
                uVar10 = uVar6;
                (**ppcVar4)(uVar13, puVar3, (puVar3 >> 0x10), local_40,
                            (local_40 >> 0x10));
                uVar11 = extraout_DX_03 | uVar10;
                if (uVar11 != 0)
                {
                    uVar13 = SUB42(&PTR_LOOP_1050_1028, 0);
                    paVar7 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar10, extraout_DX_03);
                    puVar16 = pass1_1030_73a8(CONCAT22(uVar11, paVar7));
                    uVar12 = (puVar16 >> 0x10);
                    uVar11 = puVar16;
                    if (((uVar12 | uVar11) != 0) && ((uVar11 + 0x12) == 5))
                    {
                        ppcVar4 = (*puVar16 + 0x48);
                        (**ppcVar4)(&PTR_LOOP_1050_1028, uVar11, uVar12);
                        if (uVar11 < 0)
                        {
                            local_38 = local_38 + uVar11;
                        }
                        else
                        {
                            local_36 = local_36 + uVar11;
                        }
                    }
                }
                local_40 = local_40 + 1;
            }
            iVar2 = local_38 - local_c;
            uVar1 = local_AX_122.field_0x20a;
            uVar17 = (param_1 >> 0x10);
            uVar5 = param_1;
            uVar8 = uVar1;
            pass1_1038_01c0(uVar5, uVar17, _local_26);
            iVar9 = uVar8 - uVar1;
            local_38 = iVar2 - iVar9;
            pass1_1038_008e(uVar5, uVar17, _local_26);
            if (iVar9 < 0)
            {
                local_38 = local_38 + iVar9;
            }
            else
            {
                local_36 = local_36 + iVar9;
            }
            if (1000 < local_36)
            {
                local_36 = 1000;
            }
            if (local_36 < 0)
            {
                local_36 = 0;
            }
            local_36 = local_36 + local_38;
            if (1000 < local_36)
            {
                local_36 = 1000;
            }
            if (local_36 < 0)
            {
                local_36 = 0;
            }
            pass1_1038_4d0e(_local_26, local_36);
            if (local_AX_122.field_0x4 == 0x4000001)
            {
                pass1_1038_08d4(uVar5, CONCAT22(local_36, uVar17), _local_26);
            }
            pass1_1038_095e(uVar5, uVar17, local_36, _local_26);
        }
    }
    return;
}


pub fn pass1_1030_ec86(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1030_eb86()

{
    let mut iVar1: i32;
    let ppcVar2: fn();
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let puVar4: *mut u32;
    let mut local_22: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(CONCAT22(unaff_SS, &local_14), (&PTR_LOOP_1050_0000 + 1), 0, 0x700);
    loop
    {
        puVar4 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_14));
        uVar3 = (puVar4 >> 0x10);
        if (puVar4 == 0x0) {
            break;}
        if ((puVar4 + 0x12) == 5)
        {
            iVar1 = (puVar4 + 0xc);
            if (((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33))) &&
                ((iVar1 == 0x34 || iVar1 + -0x33 < 1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 2))))))
            {
                ppcVar2 = (*puVar4 + 0x2c);
                ppcVar2(&PTR_LOOP_1050_1028);
            }
        }
    }
    return 1;
}



pub fn pass1_1030_ea50(param_1: u32, param_2: *mut astruct_493)

{
    let mut u_var1: u32;
    let BVar2: bool;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 99999;
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (iVar3 + 0x110), 3);
    if (BVar2 != 0)
    {
        uVar5 = pass1_1030_73a8(param_2);
        local_c = (uVar5 >> 0x10);
        local_e = uVar5;
        local_6 = pass1_1028_45e2(uVar5);
    }
    uVar1 = (iVar3 + 0x108);
    local_8 = (uVar1 + 4);
    local_a = 0;
    loop
    {
        if (local_8 <= local_a)
        {
            return;
        }
        pass1_1020_bb16((iVar3 + 0x108), CONCAT22(unaff_SS, &local_12),
                        CONCAT22(unaff_SS, &local_e), local_a);
        if (local_6 < local_12)
        {
            pass1_1030_7ddc(param_2, local_6, local_e);
            local_6 = 0;
        }
        else
        {
            local_6 = local_6 - local_12;
            pass1_1030_7ddc(param_2, local_12, local_e);
        }
        if ((local_6._2_2_ | local_6) == 0) {
            break;}
        local_a = local_a + 1;
    }
    return;
}

pub fn pass1_1030_eb14(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}




pub fn pass1_1030_e8f8(param_1: *mut astruct_1036)

{
    let mut uVar1: i32;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let paVar4: *mut astruct_493;
    let mut in_DX: u16;
    let local_BX_4: *mut astruct_1036;
    let local_ES_4: *mut astruct_1036;
    let mut uVar5: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x108 != 0)
    {
        uVar3 = local_BX_4.field_0x10c;
        paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, (uVar3 >> 0x10));
        _local_6 = CONCAT22(in_DX, paVar4);
        uVar5 = pass1_1030_73a8(CONCAT22(in_DX, paVar4));
        if ((uVar5 + 0xc) == local_BX_4.field_0x110)
        {
            pass1_1030_ea50(param_1, _local_6);
        }
        uVar1 = local_BX_4.field_0x108;
        uVar2 = &local_BX_4.field_0x10a;
        _local_14 = CONCAT22(uVar2, uVar1);
        if ((uVar2 | uVar1) != 0)
        {
            pass1_1020_ba7e(CONCAT22(uVar2, uVar1));
            error_check_1000_17ce(_local_14);
        }
        &local_BX_4.field_0x108 = 0;
    }
    return 1;
}


pub fn pass1_1030_e864(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1030_e75e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1030_e602(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1030_e546(param_1: u32)

{
    let mut u_var1: u32;

    uVar1 = (param_1 + 0x108);
    pass1_1028_e332(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    return 1;
}



pub fn pass1_1030_e410(param_1: *mut astruct_1021)

{
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let mut in_DX: i32;
    let mut unaff_SS: u16;
    let mut local_10: [u8;6];
    let mut local_a: [u8;4];
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = (param_1 + 0x10c);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    if ((in_DX | paVar2) != 0)
    {
        local_6 = paVar2;
        paVar2 = pass1_1038_4fd8(CONCAT22(in_DX, paVar2), 0x21);
        if (paVar2 == 0x0)
        {
            pass1_1020_a43e(CONCAT22(unaff_SS, local_a));
            pass1_1008_3e54(CONCAT22(unaff_SS, local_10), 0, 2, 0xfffd);
            pass1_1020_a49a(CONCAT22(unaff_SS, local_a), CONCAT22(unaff_SS, local_10), 0x7a);
            pass1_1008_3e76(CONCAT22(unaff_SS, local_10), 0, 3, 0xfffe);
            pass1_1020_a49a(CONCAT22(unaff_SS, local_a), CONCAT22(unaff_SS, local_10), 0x7a);
            pass1_1008_3e76(CONCAT22(unaff_SS, local_10), 0, 3, 0xfffd);
            paVar2 = 
                pass1_1020_a49a(CONCAT22(unaff_SS, local_a), CONCAT22(unaff_SS, local_10), 0x21);
        }
    }
    return paVar2;
}


pub fn pass1_1030_e2be(param_1: *mut astruct_500, param_2: u16,param_3: u32,param_4: u32)

{
    let local_BX_19: *mut astruct_500;
    let mut uVar1: u16;

    pass1_1028_d1dc(param_1, s_fem133_wav_1050_2af7);
    uVar1 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    local_BX_19.field_0x108 = param_4;
    local_BX_19.field_0x10c = param_3;
    &local_BX_19.field_0x110 = param_2;
    param_1.a = 0xe4ea;
    local_BX_19.b = 0x1030;
    return;
}



pub fn pass1_1030_e300(param_1: u32)

{
    let ppVar1: *mut pass1_struct_1;
    let mut in_stack_0000fff8: u32;

    ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0,
                                      CONCAT22((in_stack_0000fff8 >> 0x10), 0x2b));
    pass1_1010_089e(ppVar1, (param_1 + 0x110), 2);
    return 1;
}

pub fn pass1_1030_e328(param_1: *mut astruct_1021)

{
    let local_BX_3: *mut astruct_1021;
    let local_ES_3: *mut astruct_1021;

    local_ES_3 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0x110 == 0)
    {
        ret_1030_e4ba(param_1);
    }
    else
    {
        pass1_1030_e410((param_1 & 0xffff | ZEXT24(local_ES_3) << 0x10));
    }
    return 1;
}


pub fn pass1_1030_e0d4()

{
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let paVar3: *mut astruct_493;
    let local_AX_104: *mut astruct_1017;
    let paVar4: *mut astruct_493;
    let mut uVar5: i32;
    let mut extraout_DX: i32;
    let mut iVar6: i32;
    let local_BX_211: *mut astruct_1018;
    let mut iVar7: i32;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let mut unaff_SS: u16;
    let ppVar9: *mut pass1_struct_1;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1c: [u8;8];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar9 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x40));
    local_4 = (ppVar9 >> 0x10);
    local_6 = ppVar9;
    _local_a = pass1_1008_b820(ppVar9);
    uVar5 = (_local_a >> 0x10) | _local_a;
    if (uVar5 != 0)
    {
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x800);
        _local_e = CONCAT22(uVar5, paVar3);
        local_10 = (&paVar3[0xb].field_0xa != 0);
        pass1_1008_5784(CONCAT22(unaff_SS, local_1c), _local_a);
        loop
        {
            local_AX_104 = local_1c;
            pass1_1008_5b12(CONCAT22(unaff_SS, local_AX_104));
            _local_14 = CONCAT22(extraout_DX, local_AX_104);
            uVar5 = extraout_DX | local_AX_104;
            if (uVar5 == 0) {
                break;}
            if (local_AX_104.field_0x8 != 0)
            {
                uVar2 = local_AX_104.field_0xa;
                paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
                paVar3 = paVar4;
                pass1_1038_354a(CONCAT22(uVar5, paVar4));
                if (paVar3 != 0x0)
                {
                    uVar8 = (_local_14 >> 0x10);
                    if (local_10 == 0)
                    {
                        local_BX_211 = ((local_14 + 0xe) * 0xc);
                        local_2a = (local_BX_211 + 0x58c4);
                        iVar6 = (local_BX_211 + 0x58c8);
                    }
                    else
                    {
                        iVar6 = (local_14 + 0xe) * 0xc;
                        local_2a = (iVar6 + 0x58be);
                        iVar6 = (iVar6 + 0x58c2);
                    }
                    iVar7 = iVar6;
                    pass1_1038_35a8(CONCAT22(uVar5, paVar4),
                                    ((local_14 + 0x10) * 2 + local_2a));
                    if (iVar7 != 0)
                    {
                        uVar8 = (_local_14 >> 0x10);
                        iVar7 = _local_14;
                        piVar1 = (iVar7 + 0x10);
                        *piVar1 = *piVar1 + 1;
                        if (iVar6 <= (iVar7 + 0x10))
                        {
                            (iVar7 + 0x10) = 0;
                        }
                    }
                }
            }
        }
    }
    return;
}


pub fn pass1_1030_e010(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_dcf4(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1030_df0c(param_1: *mut astruct_44)

{
    let mut u_var1: u32;
    let mut uVar2: u32;
    let lVar3: u32;
    let uVar4: u8;
    let mut uVar5: u16;
    let paVar6: *mut astruct_990;
    let extraout_var: u32;
    let mut uVar7: u32;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff07747202: *mut astruct_1015;

    uVar4 = pass1_1028_b58e(param_1);
    temp_7ff07747202 = CONCAT31(extraout_var, uVar4);
    uVar1 = &temp_7ff07747202.field_0x2e;
    local_a._0_2_ = uVar1;
    if ((temp_7ff07747202.field_0x30 | local_a) != 0)
    {
        uVar8 = (uVar1 >> 0x10);
        uVar1 = (local_a + 0x210);
        local_e._0_2_ = uVar1;
        if (((local_a + 0x212) | local_e) != 0)
        {
            uVar8 = (uVar1 >> 0x10);
            uVar2 = (local_e + 10);
            uVar5 = pass1_1030_dfcc(param_1);
            if (uVar5 != 0)
            {
                local_18._0_2_ = 1;
                local_18._2_2_ = 0;
                while (CONCAT22(local_18._2_2_, local_18) < uVar2)
                {
                    uVar7 = uVar2;
                    uVar9 = uVar5;
                    pass1_1030_1312(local_e, uVar8, local_18);
                    paVar6 = pass1_1030_cde8(uVar7, extraout_DX, uVar9);
                    if (-1 < paVar6)
                    {
                        pass1_1030_cef8((uVar7 & 0xffff | extraout_DX << 0x10),
                                        CONCAT31(extraout_var, uVar4) & 0xffff | in_DX << 0x10, 1, paVar6);
                        (param_1 + 0x20) = (uVar7 + 4);
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


pub fn pass1_1030_dc08(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_dc96(param_1: *mut astruct_763) -> *mut astruct_763

{
    let mut uVar1: u16;

    pass1_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0;
    param_1.field_0x0 = 0xe036;
    (param_1 + 2) = 0x1030;
    return param_1;
}


pub fn pass1_1030_dcc2(param_1: *mut astruct_1012, param_2: u16, param_3: u16,param_3_00: u32) -> i32

{
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    param_1.field_0x20 = 0;
    CONCAT22(param_2, param_1) = 0xe036;
    param_1.field_0x2 = 0x1030;
    return CONCAT22(param_2, param_1);
}




pub fn pass1_1030_dcf4(param_1: *mut astruct_44)

{
    let mut u_var1: u32;
    let ppcVar2: fn();
    let uVar3: u8;
    let paVar4: *mut astruct_493;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let extraout_var: u32;
    let puVar7: *mut u8;
    let mut uVar8: u32;
    let mut in_DX: i32;
    let mut uVar9: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut iVar10: i32;
    let mut uVar11: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let local_14: *mut astruct_1014;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff4f6878d8: *mut astruct_1013;

    uVar11 = (param_1 >> 0x10);
    iVar10 = param_1;
    param_1.ptr_a_lo = 0xe036;
    (iVar10 + 2) = 0x1030;
    if (_PTR_LOOP_1050_65e2 != 0)
    {
        uVar3 = pass1_1028_b58e(param_1);
        temp_7ff4f6878d8 = CONCAT31(extraout_var, uVar3);
        if ((iVar10 + 0x20) == 0)
        {
            if ((in_DX | temp_7ff4f6878d8) == 0)
            {
                paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
            }
            else
            {
                paVar4 = temp_7ff4f6878d8.field_0x2e;
                in_DX = temp_7ff4f6878d8.field_0x30;
            }
            uVar9 = in_DX;
            puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
            uVar5 = SUB42(puVar7, 0);
            pass1_1038_4d6e(CONCAT22(in_DX, paVar4),
                            puVar7 & 0xffff | uVar9 << 0x10);
            _local_14 = CONCAT22(extraout_DX, uVar5);
            ppcVar2 = (*_local_14 + 0x10);
            uVar11 = uVar5;
            ppcVar2(&PTR_LOOP_1050_1038, uVar5, extraout_DX);
            _local_18 = CONCAT22(extraout_DX_00, uVar11);
            local_1c = 0;
            while (local_1c < _local_18)
            {
                uVar8 = _local_18;
                pass1_1030_1d7c(uVar5, extraout_DX, local_1c, (local_1c >> 0x10));
                if ((extraout_DX_01 | uVar8) != 0)
                {
                    uVar6 = pass1_1030_dfcc(param_1);
                    uVar6 = pass1_1030_cbf0(uVar8, extraout_DX_01, uVar6);
                    if (uVar6 != 0) {
                        break;}
                }
                local_1c = local_1c + 1;
            }
            if (_local_14 != 0x0)
            {
                ppcVar2 = *_local_14;
                ppcVar2(0x38, uVar5, extraout_DX, 1);
            }
        }
        else
        {
            uVar1 = (iVar10 + 0x20);
            uVar9 = in_DX;
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
            if ((uVar9 | paVar4) != 0)
            {
                local_c = 0;
                match ((iVar10 + 0xc))
                {
                0x73 |
                0x77 =>{
                    local_c = 1;
                    },
                0x74 |
                0x78 =>{
                    local_c = 2;
                },
                0x75 =>{
                    local_c = 3;
                },
                0x76 =>{
                    local_c = 5;
                }}
                if (local_c != 0)
                {
                    pass1_1030_cc44(paVar4, uVar9, 1,
                                    CONCAT31(extraout_var, uVar3) & 0xffff | in_DX << 0x10, local_c);
                }
            }
        }
    }
    pass1_1028_b418(param_1);
    return;
}


pub fn pass1_1030_db78(param_1: u32)

{
    let mut uVar1: i32;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 6)
    {
        pass1_1028_bdac((param_1 & 0xffff | uVar1 << 0x10), 5);
    }
    return;
}




pub fn pass1_1030_db92(param_1: u16, param_2: u16,param_1_00: u32,param_2_00: u32,param_5: u32) -> i32

{
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let puVar3: *mut u8;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8;2];

    uVar5 = pass1_1030_bcae(local_4, unaff_SS);
    uVar4 = (uVar5 >> 0x10);
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    uVar1 = &paVar2.field_0x10;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    puVar3 = local_4;
    pass1_1030_bcde(puVar3, unaff_SS, CONCAT22(uVar4, paVar2), param_1_00, param_5);
    if (puVar3 < 0)
    {
        PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    return;
}


pub fn pass1_1030_d868(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_d8f6(param_1: *mut astruct_763) -> *mut astruct_763

{
    let local_BX_12: *mut astruct_1008;
    let mut uVar1: u16;

    pass1_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    param_1.field_0x0 = 0xdc2e;
    local_BX_12.field_0x2 = 0x1030;
    if (local_BX_12.field_0xc == 0x4c)
    {
        local_BX_12.field_0xe = 0x43;
    }
    else
    {
        if (local_BX_12.field_0xc == 0x4d)
        {
            local_BX_12.field_0xe = 0x44;
        }
        else
        {
            local_BX_12.field_0xe = 0x45;
        }
    }
    return param_1;
}

pub fn pass1_1030_d942(param_1: *mut astruct_1009, param_2: u16, param_3: u16,param_3_00: u32)

{
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xdc2e;
    param_1.field_0x2 = 0x1030;
    if (param_1.field_0xc == 0x4c)
    {
        param_1.field_0xe = 0x43;
    }
    else
    {
        if (param_1.field_0xc == 0x4d)
        {
            param_1.field_0xe = 0x44;
        }
        else
        {
            param_1.field_0xe = 0x45;
        }
    }
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1030_d994(param_1: *mut astruct_44)

{
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let local_BX_4: *mut astruct_1010;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x12 != 4)
    {
        return;
    }
    uVar5 = pass1_1028_b4f2(param_1);
    if ((uVar5 + 0x200) == 0x8000002)
    {
        uVar2 = local_BX_4.field_0x14;
        piVar1 = (uVar2 + 0x94);
        *piVar1 = *piVar1 + -1;
    }
    else
    {
        iVar3 = pass1_1028_cb04(param_1);
        if (iVar3 == 0)
        {
            return;
        }
        pass1_1030_dace(param_1);
        if (iVar3 == 0)
        {
            return;
        }
        uVar2 = local_BX_4.field_0x14;
        piVar1 = (uVar2 + 0x94);
        *piVar1 = *piVar1 + -1;
        pass1_1028_c952(param_1);
        pass1_1030_da22(param_1);
    }
    uVar2 = local_BX_4.field_0x14;
    if ((uVar2 + 0x94) < 1)
    {
        pass1_1028_bdac(param_1, 5);
    }
    return;
}



pub fn pass1_1030_da22(param_1: *mut astruct_44)

{
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut uVar4: u16;
    let BVar5: bool;
    let mut uVar6: i32;
    let puVar7: *mut u32;
    let mut uVar8: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut uVar9: u32;
    let mut local_1a: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar9 = pass1_1028_b4f2(param_1);
    uVar4 = (uVar9 >> 0x10);
    puVar1 = (uVar9 + 0xc);
    ppcVar2 = (*puVar1 + 0x10);
    puVar7 = puVar1;
    ppcVar2(&PTR_LOOP_1050_1028, puVar1, (uVar9 + 0xe));
    uVar3 = puVar7 & 0xffff | extraout_DX << 0x10;
    local_12 = 0;
    loop
    {
        if (uVar3 <= local_12)
        {
            return;
        }
        uVar8 = uVar3;
        pass1_1030_1d7c(puVar1, (puVar1 >> 0x10), local_12, (local_12 >> 0x10));
        uVar6 = uVar8;
        if ((((extraout_DX_00 | uVar6) != 0) &&
             (BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar6 + 0xc), 4), BVar5 != 0)) &&
            (uVar9 = pass1_1028_6744(CONCAT13((extraout_DX_00 >> 8),
                                              CONCAT12(extraout_DX_00, uVar6)),
                                     0xd),
             ((uVar9 >> 0x10) | uVar9) != 0)) {
            break;}
        local_12 = local_12 + 1;
    }
    pass1_1028_6228(uVar8 & 0xffff | extraout_DX_00 << 0x10, 1, 0, 0xd);
    return;
}



pub fn pass1_1030_dace(param_1: *mut astruct_44)

{
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut uVar4: u16;
    let BVar5: bool;
    let mut uVar6: i32;
    let puVar7: *mut u32;
    let mut uVar8: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut uVar9: u32;
    let mut local_1c: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar9 = pass1_1028_b4f2(param_1);
    uVar4 = (uVar9 >> 0x10);
    puVar1 = (uVar9 + 0xc);
    ppcVar2 = (*puVar1 + 0x10);
    puVar7 = puVar1;
    ppcVar2(&PTR_LOOP_1050_1028, puVar1, (uVar9 + 0xe));
    uVar3 = puVar7 & 0xffff | extraout_DX << 0x10;
    local_14 = 0;
    loop
    {
        if (uVar3 <= local_14)
        {
            return;
        }
        uVar8 = uVar3;
        pass1_1030_1d7c(puVar1, (puVar1 >> 0x10), local_14, (local_14 >> 0x10));
        uVar6 = uVar8;
        if ((((extraout_DX_00 | uVar6) != 0) &&
             (BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar6 + 0xc), 4), BVar5 != 0)) &&
            (uVar9 = pass1_1028_6744(CONCAT13((extraout_DX_00 >> 8),
                                              CONCAT12(extraout_DX_00, uVar6)),
                                     0xd),
             ((uVar9 >> 0x10) | uVar9) != 0)) {
            break;}
        local_14 = local_14 + 1;
    }
    return;
}


pub fn pass1_1030_c8da(param_1: u32,param_2: u32,param_3: u32)

{
    let mut local_6: u32;

    local_6 = 0;
    if (param_3._2_2_ == 1)
    {
        (param_1 + 0x20) = param_2._2_2_;
    }
    else
    {
        local_6 = ret_1030_178e(param_1, param_2, param_3);
    }
    return local_6;
}

pub fn pass1_1030_c91a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


// WARNING: This function may have set the stack p i32er

pub fn bad_fn_1030_c949()

{
    let pcVar1: *mut libc::c_char;
    let uVar2: u8;
    let mut in_DX: u16;
    let mut in_BX: i32;
    let mut cVar3: u8;
    let puVar4: *mut u16;
// ppuVar6: *mut *mut u8;
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
    let apuStack4176:[u8;2073];
    let puStack30: *mut u8;
    let puVar5: *mut u16;
// ppuVar7: *mut *mut u8;

    puStack30 = &stack0xfffe;
    puVar4 = &stack0xfffe;
    apuStack4176[0] = &stack0xfffe;
    puVar5 = &stack0xfffe;
    cVar3 = '\r';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar4 = puVar4 + -1;
        *puVar4 = *unaff_BP;
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    ppuStack4216 = apuStack4176;
    ppuVar6 = apuStack4176;
    appuStack8362[0] = apuStack4176;
    ppuVar7 = apuStack4176;
    cVar3 = '\x13';
    while
    {
        puVar5 = puVar5 + -1;
        ppuVar6 = ppuVar6 + -1;
        *ppuVar6 = *puVar5;
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    ppuVar6 = appuStack8362;
    cVar3 = '\x1b';
    while
    {
        ppuVar7 = ppuVar7 + -1;
        ppuVar6 = ppuVar6 + -1;
        *ppuVar6 = *ppuVar7;
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    pcVar1 = unaff_DI + 0x1028;
    cVar3 = (in_BX >> 8);
    if (*pcVar1 != cVar3 && cVar3 <= *pcVar1)
    {
        pcVar1 = (in_BX + unaff_SI);
        *pcVar1 = *pcVar1 - in_DX;
        pcVar1 = (in_BX + unaff_SI);
        *pcVar1 = *pcVar1 - in_DX;
        uVar2 = _in(in_DX);
        *unaff_DI = uVar2;
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    0x1024 = uStack8356;
    0x1022 = unaff_CS;
    0x1020 = 0xc927;
    pass1_1028_b418(paRam00001024);
    if ((bStack8352 & 1) != 0)
    {
        0x1024 = uStack8356;
        0x1022 = &PTR_LOOP_1050_1028;
        0x1020 = 0xc936;
        error_check_1000_17ce(paRam00001024);
    }
    return uStack8356;
}

pub fn pass1_1030_c9a8(param_1: *mut astruct_763) -> *mut astruct_763

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    pass1_1028_b354(param_1);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x98) = 1;
    param_1.field_0x0 = 0xd88e;
    (iVar1 + 2) = 0x1030;
    pass1_1000_4906((param_1 & 0xffff0000 | (iVar1 + 0x20)), 0, 0x78);
    return param_1;
}

pub fn pass1_1030_c9e4(param_1: *mut astruct_764, param_2: *mut astruct_764, param_3: u16,param_3_00: u32) -> i32

{
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    param_1.field_0x98 = 1;
    CONCAT22(param_2, param_1) = 0xd88e;
    param_1.field_0x2 = 0x1030;
    pass1_1000_4906(CONCAT22(param_2, &param_1.field_0x20), 0, 0x78);
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1030_ca26(param_1: *mut astruct_44, param_3: *mut u8)

{
    let uVar1: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let local_BX_39: *mut astruct_981;
    let local_BX_87: *mut astruct_982;
    let mut uVar2: u16;
    let mut local_8: u16;
    let local_4: *mut astruct_983;

    local_4 = 0x0;
    while (local_BX_87 = param_1, uVar2 = (param_1 >> 0x10),
           local_4 < 10)
    {
        if (((local_BX_87 + local_4 * 0xc + 0x26) == 2) ||
            ((local_BX_87 + local_4 * 0xc + 0x26) == 1))
        {
            (local_BX_87 + local_4 * 0xc + 0x26) = 4;
        }
        else
        {
            uVar1 = pass1_1028_b58e(param_1);
            local_BX_39 = (local_BX_87 + local_4 * 0xc);
            pass1_1030_6e9c(CONCAT31(extraout_var, uVar1) & 0xffff | in_DX << 0x10, 1,
                            local_BX_39.field_0x24);
            local_BX_39.field_0x20 = 0;
            local_BX_39.field_0x24 = 0;
            local_BX_39.field_0x26 = 0;
        }
        local_4 = &local_4.field_0x1;
    }
    pass1_1028_b46e(param_1, param_3);
    return;
}