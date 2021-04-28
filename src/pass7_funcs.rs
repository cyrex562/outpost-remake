pub fn pass1_1018_0bf4(param_1: *mut astruct_513, param_2: u16)

{
    let mut u_var1: u32;
    let mut uVar2: i32;
    let lVar3: u32;
    let mut extraout_DX: u16;
    let mut in_BX: i32;
    let mut unaff_SS: u16;
    let mut local_14: [u8;12];
    let mut local_8: u16;
    let mut local_6: u32;

    match (param_2)
    {
    1 =>{
        (param_1 + 0xc) = 0;
        (param_1 + 0x7e) = 0;
        return;},
    4 =>{
        pass1_1008_3eb4((param_1 & 0xffff0000 | (param_1 + 0x10)),
                        CONCAT22(unaff_SS, &local_8), CONCAT22(unaff_SS, &local_6),
                        CONCAT22(unaff_SS, &local_6 + 2));
        uVar1 = (param_1 + 0xc);
        local_8 = (uVar1 + 0x1e);
        pass1_1008_3e76((param_1 & 0xffff0000 | (param_1 + 0x10)), local_8,
                        local_6, (local_6 >> 0x10));
        zero_list_1008_6c90(local_14, unaff_SS);
        pass1_1018_0b1e(param_1 + -0x20, param_1._2_2_, local_14, unaff_SS);
      // goto LAB_1018_0c71;
    },
    5 |
    6 =>{
        uVar2 = param_1 - 0x20;
        pass1_1018_0dc6((param_1 & 0xffff0000 | uVar2));
        pass1_1018_10c4(uVar2, param_1._2_2_);
        pass1_1018_1346(uVar2, param_1._2_2_);
// LAB_1018_0c71:
        (param_1 + 0x2c) = 0;
        lVar3 = (param_1 + 0x1c);
        uVar1 = (param_1 + 0xc);
        if ((uVar1 + 0x20) == lVar3)
        {
            pass1_1018_028c((param_1 + 0xc), (param_1 + 0x1c));
            (param_1 + 0x2c) = lVar3;
            (param_1 + 0x2e) = extraout_DX;
            pass1_1010_1f62((param_1 & 0xffff0000 | (param_1 - 0x20)),
                            param_2);
            return;
        }
    },
    0x14 =>{
        uVar1 = (param_1 + 0xc);
        if ((uVar1 + 0x20) != (param_1 + 0x1c))
        {
            uVar1 = (param_1 + 0x60);
            post_win_msg_1020_291a(uVar1, (uVar1 >> 0x10));
            return;
        }
    },
    0x15 =>{
        pass1_1010_65d0((param_1 + 0x7e), 0x88);
        if (in_BX != 0)
        {
            (param_1 + 0x88) = 1;
            return;
        }
    }
}
    return;
}

pub fn pass1_1018_0d76(param_1: u32)

{
    let mut u_var1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x2c);
    if ((uVar1 + 0x20) == (param_1 + 0x3c))
    {
        return;
    }
    return;
}

pub fn pass1_1018_0d9a(param_1: u32,param_2: u32,param_3: u32)

{
    let mut u_var1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x7c);
    param_3 = (uVar1 + 0x16);
    uVar1 = (param_1 + 0x7c);
    param_2 = (uVar1 + 0x1a);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_0dc6(param_1: *mut astruct_513)

{
    let piVar1: *mut i32;
    let p_uvar2: *mut u16 ;
    let mut uvar3: u16;
    let paVar4: *mut astruct_493;
    let mut iVar5: i32;
    let local_AX_600: *mut u32;
    let mut uVar6: u32;
    let mut extraout_DX: i32;
    let in_u16_6: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut extraout_DX_00: i32;
    let mut uVar7: i32;
    let local_DX_546: *mut u8;
    let local_BX_24: *mut astruct_513;
    let local_ES_24: *mut u8;
    let local_SS__1: *mut u8;
    let mut local_36: u16;
    let mut local_32: u32;
    let local_2e: *mut u8;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_5ffec2cb35: *mut u8;

    pass1_1028_dc52(CONCAT22(local_SS__1, &local_14),
                    (&PTR_LOOP_1050_0000 + 1), 0, 0x400);
    local_ES_24 = (param_1 >> 0x10);
    local_BX_24 = param_1;
    local_24 = &local_BX_24.field_0x94;
    error_check_1000_17ce(local_24);
    local_28 = &local_BX_24.field_0x9a;
    local_20 = local_28;
    error_check_1000_17ce(local_28);
    &local_BX_24.field_0x94 = 0;
    &local_BX_24.field_0x9a = 0;
    local_BX_24.field_0x92 = 0;
    local_BX_24.field_0x98 = 0;
    loop
    {
        puVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(local_SS__1, puVar2));
        _local_18 = CONCAT22(extraout_DX, puVar2);
        in_u16_6 = (extraout_DX | puVar2);
        if (in_u16_6 == 0x0) {
            break;}
        uVar6 = (puVar2 + 0x100);
        local_28 = uVar6;
        if (uVar6 == 0x8000001)
        {
            piVar1 = &local_BX_24.field_0x92;
            unsafe{*piVar1 = *piVar1 + 1;}
        }
        else
        {
            if ((local_BX_24.field_0xa8 != 0) ||
                (pass1_1008_dfa6(local_BX_24.field_0xa2, (puVar2 + 2), 0x4000001), uVar6 != 0))
            {
                piVar1 = &local_BX_24.field_0x98;
                unsafe{*piVar1 = *piVar1 + 1;}
            }
        }
    }
    struct_a = in_u16_6;
    if (local_BX_24.field_0x92 != 0)
    {
        uVar7 = local_BX_24.field_0x92;
        local_2c = local_2c & 0xffff0000 | uVar7;
        uVar3 = uVar7 * 6;
        process_struct_1000_179c(uVar3, 0x0);
        local_20 = CONCAT22(in_u16_6, uVar3);
        struct_a = (in_u16_6 | uVar3);
        if (struct_a == 0x0)
        {
            &local_BX_24.field_0x94 = 0;
        }
        else
        {
            call_fn_ptr_1000_5586(0x3e38, &PTR_LOOP_1050_1008, local_2c, 6, uVar3,
                                  in_u16_6);
            &local_BX_24.field_0x94 = local_20;
        }
    }
    if (local_BX_24.field_0x98 != 0)
    {
        uVar7 = local_BX_24.field_0x98;
        local_2c = local_2c & 0xffff0000 | uVar7;
        uVar3 = uVar7 * 6;
        process_struct_1000_179c(uVar3, struct_a);
        local_20 = CONCAT22(struct_a, uVar3);
        if ((struct_a | uVar3) == 0)
        {
            &local_BX_24.field_0x9a = 0;
        }
        else
        {
            call_fn_ptr_1000_5586(0x3e38, &PTR_LOOP_1050_1008, local_2c, 6, uVar3,
                                  struct_a);
            &local_BX_24.field_0x9a = local_20;
        }
    }
    if (local_4 != 0)
    {
        local_8 = 1;
        local_6 = 0;
    }
    local_1c = 0;
    local_c = local_8;
    local_a = local_6;
// LAB_1018_0f74:
    puVar2 = &local_14;
    pass1_1028_e4ec(CONCAT22(local_SS__1, puVar2));
    _local_18 = CONCAT22(extraout_DX_00, puVar2);
    uVar7 = extraout_DX_00 | puVar2;
    if (uVar7 == 0)
    {
        return;
    }
    local_2c = (puVar2 + 0x100);
    local_28 = (puVar2 + 8);
    paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_28, (local_28 >> 0x10));
    local_24 = CONCAT22(uVar7, paVar4);
    local_32 = &paVar4.field_0xc;
    local_2e = *&paVar4.field_0x10;
    local_AX_600 = &local_32;
    if (local_2c != 0x8000001) {}
      // goto LAB_1018_0ffc;
    iVar5 = local_BX_24.field_0x94;
    local_DX_546 = local_BX_24.field_0x96;
    local_1c = local_1c & 0xffff | (local_1c._2_2_ + 1) << 0x10;
  // goto LAB_1018_0fe8;
// LAB_1018_0ffc:
    if ((local_BX_24.field_0xa8 != 0) ||
        (pass1_1008_dfa6(local_BX_24.field_0xa2, (_local_18 + 4), 0x4000001),
         local_AX_600 != 0x0))
    {
        iVar5 = local_BX_24.field_0x9a;
        local_DX_546 = local_BX_24.field_0x9c;
        local_1c = local_1c & 0xffff0000 | (local_1c + 1);
        local_1c._2_2_ = local_1c;
// LAB_1018_0fe8:
        modify_list_1008_3f62(CONCAT22(local_DX_546, iVar5 + local_1c._2_2_ * 6),
                              CONCAT22(local_SS__1, &local_32));
    }
  // goto LAB_1018_0f74;
}

pub fn pass1_1018_1054(param_1: *mut astruct_516, param_2: *mut u16,param_3: u32)

{
    let local_BX_3: *mut astruct_516;
    let mut uVar1: i32;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0x94 == 0)
    {
        pass1_1018_0dc6((param_1 & 0xffff | uVar1 << 0x10));
    }
    param_3 = local_BX_3.field_0x94;
    unsafe{*param_2 = local_BX_3.field_0x92;}
    return;
}

pub fn pass1_1018_108c(param_1: u32, param_2: *mut u16,param_3: u32)

{
    let local_BX_3: *mut astruct_517;
    let mut uVar1: i32;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0x9a == 0)
    {
        pass1_1018_0dc6((param_1 & 0xffff | uVar1 << 0x10));
    }
    param_3 = local_BX_3.field_0x9a;
    unsafe{*param_2 = local_BX_3.field_0x98;}
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_10c4(param_1: *mut astruct_318)

{
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let puVar4: *mut u16;
    let mut uVar5: i32;
    let mut in_i16_5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let puVar8: *mut u8;
    let mut uVar9: u32;
    let mut extraout_DX: i32;
    let mut uVar10: i32;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: u16;
    let mut extraout_DX_04: u16;
    let mut extraout_DX_05: u16;
    let mut iVar11: i32;
    let mut uVar12: u16;
    let uVar13: u8;
    let mut unaff_SS: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u32;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar12 = (param_1 >> 0x10);
    iVar11 = param_1;
    local_4 = (iVar11 + 0x86);
    error_check_1000_17ce((iVar11 + 0x88));
    (iVar11 + 0x86) = 0;
    (iVar11 + 0x88) = 0;
    pass1_1028_dc52(
                        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_16)),
                    (&PTR_LOOP_1050_0000 + 1), 0, 0x400);
    local_1e = 0;
    loop
    {
        puVar4 = &local_16;
        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar4));
        uVar10 = extraout_DX | puVar4;
        if (uVar10 == 0) {
            break;}
        if ((iVar11 + 0x3c) == (puVar4 + 4))
        {
            puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 2);
            uVar5 = puVar8;
            pass1_1038_4e78(CONCAT22(extraout_DX, puVar4), puVar8 & 0xffff | uVar10 << 0x10);
            _local_30 = CONCAT22(extraout_DX_00, uVar5);
            uVar3 = *_local_30;
            ppcVar2 = uVar3 + 8;
            uVar10 = uVar5;
            ppcVar2(&PTR_LOOP_1050_1038, uVar5, extraout_DX_00);
            local_1e = CONCAT22(local_1e._2_2_ + extraout_DX_01 + CARRY2(local_1e, uVar10),
                                local_1e + uVar10);
            if (_local_30 != 0x0)
            {
                ppcVar2 = uVar3;
                ppcVar2(0x38, uVar5, extraout_DX_00, 1);
            }
        }
    }
    if ((local_1e._2_2_ | local_1e) != 0)
    {
        (iVar11 + 0x86) = local_1e;
        in_i16_5 = local_1e * 6;
        process_struct_1000_179c(in_i16_5, 0x0);
        local_34 = CONCAT22(uVar10, in_i16_5);
        if ((uVar10 | in_i16_5) == 0)
        {
            (iVar11 + 0x88) = 0;
        }
        else
        {
            call_fn_ptr_1000_5586(0x3e38, &PTR_LOOP_1050_1008, local_1e, 6, in_i16_5, uVar10);
            (iVar11 + 0x88) = local_34;
        }
        if (local_6 != 0)
        {
            local_a = 1;
            local_8 = 0;
        }
        local_1e = 0;
        local_e = local_a;
        local_c = local_8;
        loop
        {
            puVar4 = &local_16;
            pass1_1028_e4ec(CONCAT22(unaff_SS, puVar4));
            uVar10 = extraout_DX_02 | puVar4;
            if (uVar10 == 0) {
                break;}
            if ((iVar11 + 0x3c) == (puVar4 + 4))
            {
                puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 2);
                uVar6 = SUB42(puVar8, 0);
                uVar13 = 0x38;
                pass1_1038_4e78(CONCAT22(extraout_DX_02, puVar4),
                                puVar8 & 0xffff | uVar10 << 0x10);
                local_28 = CONCAT22(extraout_DX_03, uVar6);
                ppcVar2 = (local_28 + 0x10);
                uVar7 = uVar6;
                ppcVar2(&PTR_LOOP_1050_1038, uVar6, extraout_DX_03);
                _local_38 = CONCAT22(extraout_DX_04, uVar7);
                local_3c = 0;
                while (local_3c < _local_38)
                {
                    uVar9 = _local_38;
                    pass1_1030_1d58(local_28);
                    uVar1 = (iVar11 + 0x88);
                    uVar13 = 8;
                    modify_list_1008_3f62((uVar1 & 0xffff0000 | (uVar1 + local_1e * 6)),
                                          CONCAT22(extraout_DX_05, uVar9 + 0xc));
                    local_1e = local_1e + 1;
                    local_3c = local_3c + 1;
                }
                if (local_28 != 0)
                {
                    ppcVar2 = local_28;
                    ppcVar2(uVar13, uVar6, extraout_DX_03, 1);
                }
            }
        }
        if ((iVar11 + 0x86) != local_4)
        {
            pass1_1010_1f62(param_1, 6);
        }
    }
    return;
}

pub fn pass1_1018_1320(param_1: u32,param_2: u32,param_3: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_3 = (param_1 + 0x88);
    param_2 = (param_1 + 0x86);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_1346(param_1: *mut astruct_318)

{
    let ppcVar1: fn();
    let p_uvar2: *mut u16 ;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut in_i16_5: u16;
    let puVar5: *mut u8;
    let mut uVar6: u32;
    let mut extraout_DX: i32;
    let mut uVar7: i32;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: u16;
    let mut extraout_DX_05: u16;
    let mut extraout_DX_06: i32;
    let local_BX_4: *mut astruct_518;
    let mut uVar8: u16;
    let uVar9: u8;
    let mut unaff_SS: u16;
    let mut uVar10: u32;
    let mut local_46: u32;
    let mut local_42: u32;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar8 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    local_4 = local_BX_4.field_0x8c;
    error_check_1000_17ce(local_BX_4.field_0x8e);
    local_BX_4.field_0x8c = 0;
    local_BX_4.field_0x8e = 0;
    pass1_1028_dc52(
                        CONCAT13((unaff_SS >> 8), CONCAT12(unaff_SS, &local_16)),
                    (&PTR_LOOP_1050_0000 + 1), 0, 0x400);
    local_1e = 0;
    loop
    {
        puVar2 = &local_16;
        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar2));
        uVar7 = extraout_DX | puVar2;
        if (uVar7 == 0) {
            break;}
        if (local_BX_4.field_0x3c == (puVar2 + 4))
        {
            puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 2);
            uVar3 = SUB42(puVar5, 0);
            uVar9 = 0x38;
            pass1_1038_4e78(CONCAT22(extraout_DX, puVar2), puVar5 & 0xffff | uVar7 << 0x10);
            _local_30 = CONCAT22(extraout_DX_00, uVar3);
            ppcVar1 = (*_local_30 + 0x10);
            uVar4 = uVar3;
            (**ppcVar1)(&PTR_LOOP_1050_1038, uVar3, extraout_DX_00);
            _local_34 = CONCAT22(extraout_DX_01, uVar4);
            local_38 = 0;
            while (local_38 < _local_34)
            {
                uVar9 = 0x30;
                uVar6 = _local_34;
                pass1_1030_1d7c(uVar3, extraout_DX_00, local_38, (local_38 >> 0x10));
                if ((uVar6 + 0x12) == 9)
                {
                    local_1e = local_1e + 1;
                }
                local_38 = local_38 + 1;
            }
            if (_local_30 != 0x0)
            {
                ppcVar1 = *_local_30;
                (**ppcVar1)(uVar9, uVar3, extraout_DX_00, 1);
            }
        }
    }
    if ((local_1e._2_2_ | local_1e) != 0)
    {
        local_BX_4.field_0x8c = local_1e;
        in_i16_5 = local_1e * 6;
        process_struct_1000_179c(in_i16_5, 0x0);
        local_46 = CONCAT22(uVar7, in_i16_5);
        if ((uVar7 | in_i16_5) == 0)
        {
            local_BX_4.field_0x8e = 0;
        }
        else
        {
            call_fn_ptr_1000_5586(0x3e38, &PTR_LOOP_1050_1008, local_1e, 6, in_i16_5, uVar7);
            local_BX_4.field_0x8e = local_46;
        }
        if (local_6 != 0)
        {
            local_a = 1;
            local_8 = 0;
        }
        local_1e = 0;
        local_e = local_a;
        local_c = local_8;
        loop
        {
            puVar2 = &local_16;
            pass1_1028_e4ec(CONCAT22(unaff_SS, puVar2));
            uVar7 = extraout_DX_03 | puVar2;
            if (uVar7 == 0) {
                break;}
            if (local_BX_4.field_0x3c == (puVar2 + 4))
            {
                puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 2);
                uVar3 = SUB42(puVar5, 0);
                uVar9 = 0x38;
                pass1_1038_4e78(CONCAT22(extraout_DX_03, puVar2),
                                puVar5 & 0xffff | uVar7 << 0x10);
                local_38 = CONCAT22(extraout_DX_04, uVar3);
                ppcVar1 = (local_38 + 0x10);
                uVar4 = uVar3;
                (**ppcVar1)(&PTR_LOOP_1050_1038, uVar3, extraout_DX_04);
                _local_34 = CONCAT22(extraout_DX_05, uVar4);
                _local_30 = 0x0;
                while (_local_30 < _local_34)
                {
                    uVar6 = _local_34;
                    pass1_1030_1d58(local_38);
                    uVar9 = 0x30;
                    uVar10 = pass1_1030_73a8((uVar6 & 0xffff | extraout_DX_06 << 0x10));
                    if ((uVar10 + 0x12) == 9)
                    {
                        uVar10 = local_BX_4.field_0x8e;
                        uVar9 = 8;
                        modify_list_1008_3f62((uVar10 & 0xffff0000 | (uVar10 + local_1e * 6)),
                                              CONCAT22(extraout_DX_06, uVar6 + 0xc));
                        local_1e = local_1e + 1;
                    }
                    _local_30 = (_local_30 + 1);
                }
                if (local_38 != 0)
                {
                    ppcVar1 = local_38;
                    (**ppcVar1)(uVar9, uVar3, extraout_DX_04, 1);
                }
            }
        }
        if (local_BX_4.field_0x8c != local_4)
        {
            pass1_1010_1f62(param_1, 6);
        }
    }
    return;
}

pub fn pass1_1018_15f6(param_1: u32,param_2: u32,param_3: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_3 = (param_1 + 0x8e);
    param_2 = (param_1 + 0x8c);
    return;
}

pub fn pass1_1018_161c(param_1: u32, param_2: *mut u16, param_3: i32, param_4: i32)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3e94((param_1 + 0x36), CONCAT22(unaff_SS, &local_6),
                    CONCAT22(unaff_SS, &local_4));
    uVar1 = local_4 + param_4 + -3;
    uVar2 = _local_6 + param_3 + -3;
    _local_6 = CONCAT22(uVar1, uVar2);
    pass1_1008_3e76(param_2, (param_1 + 0x44), uVar2, uVar1);
    return;
}

pub fn pass1_1018_1662(param_1: i32)

{
    let mut unaff_SS: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3e94((param_1 + 0x36), CONCAT22(unaff_SS, &local_6),
                    CONCAT22(unaff_SS, &local_4));
    pass1_1018_16b8(0, 0, 0);
    return;
}

pub fn pass1_1018_169e(param_1: u32,param_2: u32)

{
    pass1_1018_16b8(param_1, (param_1 + 0x44), param_2);
    return;
}

pub fn pass1_1018_16b8(param_1: *mut astruct_318, param_2: u16,param_3: u32)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let lVar3: u32;
    let mut extraout_DX: u16;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u16;
    let mut local_6: [u8;2];
    let mut local_4: [u8;2];

    if (param_3._2_2_ + -3 < 1)
    {
        param_3 = CONCAT22(3, param_3);
    }
    if (param_3 + -3 < 1)
    {
        param_3 = CONCAT22(param_3._2_2_, 3);
    }
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar2 = (iVar4 + 0x5a);
    iVar1 = (uVar2 + 4);
    if (iVar1 <= param_3._2_2_ + 2)
    {
        param_3 = param_3 & 0xffff | (iVar1 - 3) << 0x10;
    }
    uVar2 = (iVar4 + 0x5a);
    iVar1 = (uVar2 + 8);
    if (iVar1 <= param_3 + 2)
    {
        param_3 = param_3 & 0xffff0000 | (iVar1 - 3);
    }
    uVar6 = (param_3 >> 0x10);
    pass1_1008_3e76((param_1 & 0xffff0000 | (iVar4 + 0x30)), param_2, param_3,
                    uVar6);
    pass1_1008_3e94((iVar4 + 0x36), CONCAT22(unaff_SS, local_6),
                    CONCAT22(unaff_SS, local_4));
    pass1_1008_3e76((param_1 & 0xffff0000 | ZEXT24((iVar4 + 0x36))), 0,
                    param_3, uVar6);
    (iVar4 + 0x4c) = 0;
    lVar3 = (iVar4 + 0x3c);
    uVar2 = (iVar4 + 0x2c);
    if ((uVar2 + 0x20) == lVar3)
    {
        pass1_1018_028c((iVar4 + 0x2c), (iVar4 + 0x3c));
        (iVar4 + 0x4c) = lVar3;
        (iVar4 + 0x4e) = extraout_DX;
        pass1_1010_1f62(param_1, 4);
    }
    return;
}

pub fn pass1_1018_179e(param_1: u32,param_2: u32)

{
    let mut unaff_SS: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1008_3eb4(param_2, CONCAT22(unaff_SS, &local_8), CONCAT22(unaff_SS, &local_6),
                    CONCAT22(unaff_SS, &local_6 + 2));
    pass1_1018_16b8(param_1, local_8, local_6);
    return;
}

pub fn pass1_1018_17ce(param_1: u32,param_2: u32,param_3: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    pass1_1018_0412((param_1 + 0x2c), param_2,
                    CONCAT22(param_3, (param_2 >> 0x10)), (param_3 >> 0x10),
                    (param_1 + 0x3c));
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_17f0()

{
    let mut local_4: u16;

    local_4 = 0;
    while ((local_4 < 4 && ((local_4 * 2 + _PTR_LOOP_1050_3962) != 0)))
    {
        local_4 = local_4 + 1;
    }
    return local_4;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_181c(param_1: u32,param_2: u32)

{
    let ppVar1: *mut pass1_struct_2;
    let mut in_DX: u16;

    ppVar1 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10),
                             (param_1 + 0x3c));
    pass1_1030_5b6c(CONCAT22(in_DX, ppVar1), param_2);
    return;
}

pub fn pass1_1018_184e(param_1: u32, param_2: u8)

{
    pass1_1018_078e(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_18b8(param_1: *mut astruct_521, param_2: u16)

{
    let paVar1: *mut astruct_199;
    let mut extraout_DX: u16;
    let local_BX_18: *mut astruct_521;
    let mut unaff_SS: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut uVar3: u32;
    let uVar4: u8;
    let uVar5: u8;
    let mut uVar6: u16;
    astruct_522 **ppaVar7;
    let mut uVar8: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: *mut astruct_522;

    uVar6 = (param_1 >> 0x10);
    get_sys_metrics_1018_4b1e(param_1, uVar6, 0, param_2);
    local_BX_18 = param_1;
    local_BX_18.field_0x20 = s_1_1050_389a;
    local_BX_18.field_0x22 = &PTR_LOOP_1050_1008;
    local_BX_18.field_0x20 = (s_18_2_1050_3aa5 + 3);
    local_BX_18.field_0x22 = &PTR_LOOP_1050_1008;
    &local_BX_18.field_0x24 = 0;
    local_BX_18.field_0x28 = 4;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_BX_18.field_0x3a));
    local_BX_18.field_0x40 = 0;
    local_BX_18.field_0x44 = 0;
    local_BX_18.field_0x46 = 0;
    local_BX_18.field_0x4a = 0;
    local_BX_18.field_0x56 = 0;
    param_1 = (s_561_bmp_1050_1faf + 1);
    local_BX_18.field_0x2 = 0x1018;
    local_BX_18.field_0x20 = (s_568_bmp_1050_1fe7 + 5);
    local_BX_18.field_0x22 = 0x1018;
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_18.field_0x4e), 0, 8);
    ppaVar7 = &local_4;
    uVar4 = SUB21(&local_6, 0);
    uVar5 = (&local_6 >> 8);
    uVar8 = unaff_SS;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT13(uVar5, CONCAT12(uVar4, 0x48)));
    paVar1 = (ppVar2 + 0xe);
    pass1_1008_3e94(paVar1, CONCAT22(unaff_SS, CONCAT11(uVar5, uVar4)),
                    CONCAT22(uVar8, ppaVar7));
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x9a);
    local_BX_18.field_0x24 = paVar1;
    local_BX_18.field_0x26 = extraout_DX;
    uVar3 = process_struct_1008_4772(CONCAT22(extraout_DX, local_BX_18.field_0x24));
    uVar8 = (uVar3 >> 0x10);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_18.field_0x32), 0, 8);
    local_BX_18.field_0x36 = (uVar3 + 4);
    local_BX_18.field_0x38 = (uVar3 + 8);
    local_BX_18.field_0x2a = (local_4 + 1);
    local_BX_18.field_0x2c = local_6 + 0x14;
    get_sys_metrics_1018_1ea0(param_1);
    pass1_1008_3e76((param_1 & 0xffff0000 | &local_BX_18.field_0x3a), 0,
                    0x88, 0x99);
    return;
}

pub fn pass1_1018_1a04(param_1: *mut astruct_376)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let puVar4: *mut u16;
    let local_BX_5: *mut astruct_523;
    let mut uVar5: u16;
    let mut local_e: u32;

    uVar5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1.ptr_a_lo = (s_561_bmp_1050_1faf + 1);
    local_BX_5.field_0x2 = 0x1018;
    local_BX_5.field_0x20 = (s_568_bmp_1050_1fe7 + 5);
    local_BX_5.field_0x22 = 0x1018;
    puVar1 = local_BX_5.field_0x24;
    uVar2 = local_BX_5.field_0x26;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{
        ppcVar3 = *puVar1;}
        (**ppcVar3)();
    }
    error_check_1000_17ce(local_BX_5.field_0x40);
    if (param_1 == 0x0)
    {
        puVar4 = 0x0;
        uVar5 = 0;
    }
    else
    {
        puVar4 = &local_BX_5.field_0x20;
    }
    local_e = CONCAT22(uVar5, puVar4);
    local_e = s_1_1050_389a;
    puVar4[1] = &PTR_LOOP_1050_1008;
    pass1_1010_1d80(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_1a8e(param_1: u32)

{
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let local_BX_4: *mut astruct_524;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let ppVar4: *mut pass1_struct_1;
    let puVar5: *mut u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x44 != 0)
    {
        if (local_BX_4.field_0x46 != 0)
        {
            uVar2 = local_BX_4.field_0x46;
            (uVar2 + 0xe) = 0;
            local_BX_4.field_0x46 = 0;
        }
        piVar1 = &local_BX_4.field_0x4a;
        unsafe{*piVar1 = *piVar1 + 1;}
        return;
    }
    puVar5 = &local_8;
    ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar5, 3));
    pass1_1010_bf1e(ppVar4, CONCAT22(unaff_SS, puVar5));
    local_BX_4.field_0x44 = local_8;
    local_BX_4.field_0x40 = local_6;
    pass1_1018_1ce8(param_1);
    return;
}

pub fn pass1_1018_1b02(param_1: *mut astruct_318, param_2: i32)

{
    let piVar1: *mut i32;
    let p_uvar2: *mut u16 ;
    let mut uVar3: u32;
    let local_CX_30: *mut astruct_525;
    let local_BX_190: *mut astruct_318;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: [u8;2];

    local_c = 0;
    loop
    {
        uVar4 = (param_1 >> 0x10);
        local_BX_190 = param_1;
        puVar2 = &local_BX_190.f;
        if (*puVar2 == local_c || *puVar2 < local_c) {
            break;}
        uVar3 = &local_BX_190.e;
        local_CX_30 = uVar3;
        local_CX_30 = local_CX_30 + local_c;
        uVar3 = uVar3 & 0xffff0000;
        piVar1 = &local_CX_30.field_0x6;
        unsafe{*piVar1 = *piVar1 + param_2 * 2 + -1;}
        uVar4 = (uVar3 >> 0x10);
        if (0x23 < local_CX_30.field_0x6)
        {
            local_CX_30.field_0x6 = 0;
        }
        if (local_CX_30.field_0x6 < 0)
        {
            local_CX_30.field_0x6 = 0x23;
        }
        modify_list_1008_3f62((uVar3 | &local_CX_30.field_0x10), uVar3 | ZEXT24(local_CX_30));
        local_CX_30.field_0x16 = local_CX_30.field_0xa;
        pass1_1008_3e94(local_CX_30, CONCAT22(unaff_SS, &local_6),
                        CONCAT22(unaff_SS, local_4));
        pass1_1008_3e76((uVar3 | ZEXT24(local_CX_30)), 0, local_6,
                        ((local_CX_30.field_0x8 * 0x24 + local_CX_30.field_0x6) * 2 + 0x3c20));
        local_CX_30.field_0xa = (local_CX_30.field_0x6 * 2 + 0x3966);
        local_c = local_c + 1;
    }
    pass1_1010_1f62(param_1, 0xd);
    return;
}


pub fn pass1_1018_1c9a(param_1: *mut u8, param_2: u16)

{
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let local_BX_38: *mut astruct_527;
    let mut local_ES_22: u16;
    let mut local_ES_34: u16;
    let mut local_a: u32;
    let mut temp_5f8ef1f17c: u32;

    local_a = 0;
    loop
    {
        local_ES_22 = (param_1 >> 0x10);
        piVar1 = (param_1 + 0x44);
        let pi_var1_val = unsafe{*piVar1};
        if (pi_var1_val == local_a || pi_var1_val < local_a)
        {
// LAB_1018_1ce0:
            return local_a._2_2_;
        }
        uVar2 = (param_1 + 0x40);
        local_ES_34 = (uVar2 >> 0x10);
        local_BX_38 = (uVar2 + local_a * 0x18);
        if ((local_BX_38.field_0xc * 0x1e + 0x3c32) == param_2)
        {
            pass1_1018_1eda(param_1, uVar2 & 0xffff0000 | ZEXT24(local_BX_38));
            local_a = 0x10000;
          // goto LAB_1018_1ce0;
        }
        local_a = (local_a + 1);
    } 
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1018_1ce8(param_1: *mut astruct_528)

{
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let local_BX_6: *mut astruct_528;
    let local_BX_238: *mut astruct_529;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let uVar5: u8;
    let mut local_1a: u16;
    let mut local_18: [u8;2];
    let mut local_16: [u8;2];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar4 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    local_6 = local_BX_6.field_0x40;
    local_8 = 0;
    loop
    {
        puVar1 = &local_BX_6.field_0x44;
        let pu_var1_val = unsafe{*puVar1};
        if (pu_var1_val == local_8 || pu_var1_val < local_8)
        {
            return;
        }
        uVar5 = (unaff_SS >> 8);
        pass1_1008_3eb4((local_6 & 0xffff0000 | (local_8 * 0x18 + local_6)),
                        CONCAT22(unaff_SS, &local_e), CONCAT13(uVar5, CONCAT12(unaff_SS, &local_c)),
                        CONCAT22(unaff_SS, &local_a));
        local_a = local_a / 10;
        iVar3 = local_c / 10;
        local_10 = local_c % 10;
        if (local_10 != 0)
        {
            if (local_10 < 6)
            {
                local_c = local_c - local_10;
            }
            else
            {
                local_c = local_c + (10 - local_10);
            }
        }
        xor_1000_49b2();
        uVar2 = iVar3 / 5;
        local_12 = uVar2;
        if (0x14 < uVar2)
        {
            local_12 = 0x14;
            xor_1000_49b2();
            uVar2 = (local_e / uVar2) * 100;
            local_e = uVar2;
        }
        xor_1000_49b2();
        local_10 = uVar2 % 5;
        if (local_10 != 0)
        {
            if (local_e < 0)
            {
                uVar2 = local_10;
                if (2 < local_10)
                {
                    uVar2 = local_10 - 5;
                }
                local_e = local_e + uVar2;
            }
            else
            {
                if (local_10 < 3)
                {
                    local_e = local_e - local_10;
                }
                else
                {
                    local_e = local_e + (5 - local_10);
                }
            }
        }
        local_14 = (local_12 * 0x48 + 0x3c20);
        if (local_c < local_14)
        {
            local_1a = local_12;
            while (local_1a < 0x15)
            {
                puVar1 = (local_1a * 0x48 + 0x3c20);
                let pu_var1_val = unsafe{*puVar1};
                if (pu_var1_val == local_c || pu_var1_val < local_c)
                {
                    local_12 = local_1a;
                    break;
                }
                local_1a = local_1a + 1;
            }
        }
        pass1_1008_3e94(&local_BX_6.field_0x3a,
                        CONCAT13(uVar5, CONCAT12(unaff_SS, local_18)),
                        CONCAT22(unaff_SS, local_16));
        local_BX_238 = (local_8 * 0x18 + local_6);
        local_BX_238.field_0x6 = local_a;
        local_BX_238.field_0x8 = local_12;
        pass1_1008_3e76((local_6 & 0xffff0000 | ZEXT24(local_BX_238)), 0, local_e,
                        ((local_12 * 0x24 + local_a) * 2 + 0x3c20));
        local_BX_238.field_0xa = (local_a * 2 + 0x3966);
        local_8 = local_8 + 1;
    } 
}

pub fn pass1_1018_1e78(param_1: u32, param_2: u16)

{
    let mut u_var1: u32;
    let mut in_EAX: u32;
    let mut uVar2: u32;

    if (param_2 == 0xffff)
    {
        uVar1 = (param_1 + 0x46);
        uVar2 = in_EAX & 0xffff0000 |  * (uVar1 + 0xc);
    }
    else
    {
        uVar2 = in_EAX & 0xffff0000 | param_2;
    }
    return uVar2 & 0xffff0000 | (uVar2 * 0x1e + 0x3c18);
}


pub fn pass1_1018_1eda(param_1: *mut astruct_318,param_2: u32)

{
    let local_BX_3: *mut astruct_318;
    let mut uVar1: u16;
    let mut temp_5fe3fd8621: u32;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if ((local_BX_3 + 1) != 0)
    {
        temp_5fe3fd8621 = (local_BX_3 + 1);
        (temp_5fe3fd8621 + 0xe) = 2;
    }
    (local_BX_3 + 1) = param_2;
    (param_2 + 0xe) = 1;
    pass1_1010_1f62(param_1, 0xd);
    return;
}

pub fn pass1_1018_1f1a(param_1: *mut astruct_531, param_2: u16)

{
    astruct_532 **ppaVar1;
    let local_BX_10: *mut astruct_531;
    let mut uVar2: u16;
    let local_6: *mut astruct_532;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_10 = param_1;
    if (local_BX_10.field_0x56 == 0)
    {
        return 0;
    }
    local_6 = 0x0;
    loop
    {
        ppaVar1 = &local_BX_10.field_0x56;
        if (*ppaVar1 == local_6 || *ppaVar1 < local_6)
        {
            return 0;
        }
        if ((&local_BX_10.field_0x4e + local_6 * 2) == param_2) {
            break;}
        local_6 = &local_6.field_0x1;
    }
    return 1;
}

pub fn pass1_1018_1f7a(param_1: u16, param_2: u16)

{
    return CONCAT22(param_2, param_1 + 0x2a);
}

pub fn pass1_1018_1f8a(param_1: u32, param_2: u8)

{
    pass1_1018_1a04(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_1ff4(param_1: *mut astruct_533,param_2: u32)

{
    let puVar1: *mut u16;
    let extraout_var: uint32;
    let mut unaff_SS: u16;
    let ppVar2: *mut pass1_struct_1;
    let puVar3: *mut u16;
    let mut uVar4: u16;
    let puVar5: *mut u16;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar4, param_1), (param_2 >> 0x10));
    &param_1.field_0xa = 0xb9010b;
    param_1.field_0xe = 0x170035;
    CONCAT22(uVar4, param_1) = (s_556a_bmp_1050_21e5 + 3);
    param_1.field_0x2 = 0x1018;
    puVar5 = &local_8;
    puVar3 = &local_a;
    uVar6 = unaff_SS;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar3, 0x48));
    local_4 = (ppVar2 >> 0x10);
    local_6 = ppVar2;
    pass1_1008_3e94((local_6 + 0xe), CONCAT22(unaff_SS, puVar3),
                    CONCAT22(uVar6, puVar5));
    puVar1 = &param_1.field_0xa;
    unsafe{*puVar1 = *puVar1 + local_8;}
    puVar1 = &param_1.field_0xc;
    unsafe{*puVar1 = *puVar1 + local_a;}
    pass1_1000_4906(CONCAT22(uVar4, param_1 + 1), 0, 0x7f4);
    return (extraout_var & 0xffff00) << 8 | ZEXT24(param_1);
}

pub fn pass1_1018_2076(param_1: *mut astruct_376)

{
    let mut uVar1: i32;

    uVar1 = (param_1 >> 0x10);
    param_1.ptr_a_lo = (s_556a_bmp_1050_21e5 + 3);
    (param_1 + 2) = 0x1018;
    pass1_1018_209c(param_1 & 0xffff | uVar1 << 0x10);
    pass1_1010_1d80(param_1);
    return;
}

pub fn pass1_1018_209c(param_1: u32)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut local_4: u16;

    local_4 = 0;
    while
    {
        uVar5 = (param_1 >> 0x10);
        iVar4 = param_1 + 0x12;
        puVar1 = (iVar4 + local_4 * 4);
        uVar2 = (iVar4 + local_4 * 4 + 2);
        if ((uVar2 | puVar1) != 0)
        {
            unsafe{ppcVar3 = *puVar1;}
            (**ppcVar3)();
        }
        (param_1 + local_4 * 4 + 0x12) = 0;
        local_4 = local_4 + 1;
        local_4 < 0x1fd
    } {}
    return;
}

pub fn pass1_1018_20ee(param_1: u32,param_2: u32)

{
    let mut uVar1: u16;
    let mut uVar2: i32;
    let mut local_8: u32;

    uVar1 = pass1_1008_aed8(param_2);
    if (uVar1 == 0)
    {
        return;
    }
    uVar2 = (param_1 >> 0x10);
    if ((param_1 + param_2 * 4 + 0x12) == 0)
    {
        pass1_1018_216e(param_1 & 0xffff | uVar2 << 0x10, param_2);
    }
    pass1_1008_ae26(param_2);
    return;
}

pub fn pass1_1018_214e(param_1: u16, param_2: u16, param_1_00: *mut u16, param_2_00: i32)

{
    pass1_1008_3e76(param_1_00, 0, (param_2_00 * 4 + 0x3e90), (param_2_00 * 4 + 0x3e8e));
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_216e(param_1: u32,param_2: u32)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_8 = pass1_1008_adf2(param_2);
    uVar1 = pass1_1008_ae0c(param_2);
    while (local_8 <= uVar1)
    {
        uVar3 = string_fn_1010_8018(_g_struct_73_1050_14cc, local_8);
        uVar2 = (param_1 >> 0x10);
        (param_1 + local_8 * 4 + 0x12) = uVar3;
        (param_1 + local_8 * 4 + 0x14) = (uVar3 >> 0x10);
        local_8 = local_8 + 1;
    }
    return;
}

pub fn pass1_1018_21c2(param_1: u32, param_2: u8)

{
    pass1_1018_2076(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_229c(param_1: *mut astruct_534,param_2: u32)

{
    let uVar1: u8;
    let puVar2: *mut u8;
    let mut uvar3: u16;
    let puVar4: *mut u16;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: u16;
    let mut extraout_DX_04: u16;
    let mut extraout_DX_05: u16;
    let mut local_4: u16;

    pass1_1018_4cda(param_1, param_2);
    param_1.u16_x1c = s_1_1050_389a;
    param_1.u16_x1e = &PTR_LOOP_1050_1008;
    param_1.u16_x1c = (s_18_2_1050_3aa5 + 3);
    param_1.u16_x1e = &PTR_LOOP_1050_1008;
    uVar3 = 0;
    param_1.u16_x20 = 0;
    param_1.u16_x24 = 0;
    param_1.u16_x26 = 0;
    &param_1.u16_x2a = 0;
    param_1.u16_x3e = 0;
    param_1.u16_x40 = 0;
    param_1.u16_x42 = 0;
    param_1.u16_x44 = 0;
    &param_1.u8_ptr_x6e = 0;
    CONCAT22(param_2, param_1) = (s_fem130_wav_1050_2ad6 + 4);
    param_1.u16_x02 = 0x1018;
    param_1.u16_x1c = (s_fem132_wav_1050_2aec + 6);
    param_1.u16_x1e = 0x1018;
    PTR_LOOP_1050_4230 = param_1;
    PTR_LOOP_1050_4232 = param_2;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x105);
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1a8);
    param_1.u16_x2a = uVar3;
    param_1.u16_x2c = extraout_DX;
    pass1_1000_4906(CONCAT22(param_2, &param_1.u8_ptr_x2e), 0, 0x10);
    uVar1 = pass1_1000_4906((param_2 << 0x10 | &param_1.field_0x46), 0, 0x28);
    puVar2 = CONCAT31(extraout_var, uVar1);
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x6c);
    param_1.u8_ptr_x2e = puVar2;
    param_1.u16_x30 = extraout_DX_00;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x68);
    param_1.u8_ptr_x32 = puVar2;
    param_1.u16_x34 = extraout_DX_01;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x66);
    param_1.u8_ptr_x36 = puVar2;
    param_1.u16_x38 = extraout_DX_02;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x6a);
    param_1.u8_ptr_x3a = puVar2;
    param_1.u16_x3c = extraout_DX_03;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1cd);
    param_1.u8_ptr_x6e = puVar2;
    param_1.u16_x70 = extraout_DX_04;
    local_4 = 0;
    while
    {
        uVar3 = local_4 + 0x8f;
        mixed_fn_1010_830a(_g_struct_73_1050_14cc, uVar3);
        (&param_1.field_0x46 + local_4 * 4) = uVar3;
        (&param_1.field_0x48 + local_4 * 4) = extraout_DX_05;
        local_4 = local_4 + 1;
        local_4 < 10
    } {}
    if (CONCAT22(param_2, param_1) == 0)
    {
        puVar4 = 0x0;
        param_2._0_2_ = 0x0;
    }
    else
    {
        puVar4 = &param_1.u16_x1c;
    }
    process_struct_1008_9262(_PTR_LOOP_1050_0388, (_PTR_LOOP_1050_0388 >> 0x10), 0x73,
                             CONCAT22(param_2, puVar4));
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_2440(param_1: *mut astruct_535)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let puVar4: *mut u16;
    let mut uVar5: u16;
    let local_BX_4: *mut astruct_535;
    let mut uVar6: u16;
    let mut unaff_CS: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar6 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.a = (s_fem130_wav_1050_2ad6 + 4);
    local_BX_4.b = 0x1018;
    local_BX_4.c = (s_fem132_wav_1050_2aec + 6);
    local_BX_4.d = 0x1018;
    if (_PTR_LOOP_1050_0388 != 0)
    {
        if (param_1 == 0x0)
        {
            puVar4 = 0x0;
            uVar5 = 0;
        }
        else
        {
            puVar4 = &local_BX_4.c;
            uVar5 = uVar6;
        }
        unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
        pass1_1008_92b2(_PTR_LOOP_1050_0388, 0x73, puVar4, uVar5);
    }
    puVar1 = local_BX_4.e;
    uVar2 = local_BX_4.f;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;}
        (**ppcVar3)(unaff_CS, puVar1, uVar2, 1);
    }
    puVar1 = local_BX_4.g;
    uVar2 = local_BX_4.h;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;}
        (**ppcVar3)(unaff_CS, puVar1, uVar2, 1);
    }
    if (param_1 == 0x0)
    {
        puVar4 = 0x0;
        uVar6 = 0;
    }
    else
    {
        puVar4 = &local_BX_4.c;
    }
    _local_6 = CONCAT22(uVar6, puVar4);
    *_local_6 = s_1_1050_389a;
    puVar4[1] = &PTR_LOOP_1050_1008;
    win_cleanup_1018_4d22(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_2504()

{
    let ppVar1: *mut pass1_struct_2;
    let mut uVar2: u16;
    let mut in_DX: i32;
    let mut local_6: u32;

    ppVar1 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10),
                             0x4000001);
    if ((in_DX | ppVar1) != 0)
    {
        uVar2 = pass1_1028_d69e(**_g_bool_1050_5748);
        if (uVar2 == 0)
        {
            return;
        }
    }
    return;
}

pub fn pass1_1018_2548(param_1: u16, param_2: u16, param_1_00: *mut u16)

{
    modify_list_1008_3f62(param_1_00, &u32_1048_4228);
    return;
}



pub fn pass1_1018_255e(param_1: u32)

{
    let mut uVar1: u16;
    let mut temp_5f89fb61de: u32;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x26) != 0)
    {
        temp_5f89fb61de = (param_1 + 0x26);
        return (temp_5f89fb61de + 10);
    }
    return 0;
}

pub fn pass1_1018_2580(param_1: u32, param_2: u16,param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let local_BX_4: *mut astruct_536;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut local_8: [u8;6];

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x20 == 0)
    {
        return 0x6ad;
    }
    zero_list_1008_3e38(CONCAT22(unaff_SS, local_8));
    pass1_1018_161c(local_BX_4.field_0x20, local_8, unaff_SS, param_3);
    uVar1 = pass1_1018_17ce(local_BX_4.field_0x20, CONCAT22(local_8, param_2),
                            CONCAT22(param_4, unaff_SS));
    return uVar1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_25d2(param_1: u32, param_2: u16,param_3: u32)

{
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let ppVar3: *mut pass1_struct_1;
    let puVar4: *mut u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: [u8;6];

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x20) == 0)
    {
        return 0;
    }
    zero_list_1008_3e38(CONCAT22(unaff_SS, local_8));
    uVar1 = (param_1 + 0x20);
    pass1_1018_161c(uVar1, (uVar1 >> 0x10), local_8, unaff_SS, param_3);
    puVar4 = CONCAT22(unaff_SS, local_8);
    ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(param_2, 0x32));
    uVar2 = win_fn_1010_71d6(ppVar3, param_2, puVar4);
    return uVar2;
}

pub fn pass1_1018_262e(param_1: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x44) = 1;
    (param_1 + 0x3e) = 0;
    return;
}

pub fn pass1_1018_2646(param_1: u16, param_2: u16, param_1_00: *mut u16)

{
    modify_list_1008_3f62(param_1_00, &u32_1048_4222);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_265c()

{
    let mut u_var1: u32;

    uVar1 = pass1_1030_8326();
    return uVar1;
}

pub fn pass1_1018_266a(param_1: u32)

{
    return (param_1 + 0x44);
}

pub fn pass1_1018_2678(param_1: u16, param_2: u16, param_1_00: *mut u16)

{
    modify_list_1008_3f62(param_1_00, &u32_1048_4216);
    return;
}

pub fn pass1_1018_268e(param_1: *mut astruct_538) -> *mut astruct_104

{
    let local_BX_4: *mut astruct_538;
    let local_SI_36: *mut astruct_537;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x42 != 0)
    {
        &local_BX_4.field_0x40 = 0;
        local_BX_4.field_0x44 = 1;
    }
    local_SI_36 = (local_BX_4.field_0x3e * 4);
    return 
        CONCAT22((local_BX_4 + local_SI_36 + 0x48),
                 (local_BX_4 + local_SI_36 + 0x46));
}

pub fn pass1_1018_26c2(param_1: u16, param_2: u16, param_1_00: *mut u16)

{
    modify_list_1008_3f62(param_1_00, &u32_1048_421c);
    return;
}

pub fn pass1_1018_26d8(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: *mut u16)

{
    modify_list_1008_3f62(param_2_00, CONCAT22(0x1050, param_1_00 * 6 + 0x65ca));
    return;
}

pub fn pass1_1018_26f8(param_1: u16, param_2: u16, param_1_00: *mut u16)

{
    modify_list_1008_3f62(param_1_00, &u32_1048_4210);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_270e(param_1: *mut astruct_318, param_2: i32, param_3: *mut u16)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let puVar3: *mut u16;
    let mut uVar4: i32;
    let mut extraout_DX: i32;
    let local_BX_8: *mut astruct_318;
    let mut uVar5: i32;
    let ppVar6: *mut pass1_struct_1;
    let mut in_stack_0000fff4: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_BX_8 = param_1;
    uVar5 = (param_1 >> 0x10);
    if (param_2 == 0)
    {
        if ((local_BX_8.g == 0) || (uVar2 = local_BX_8.g, (uVar2 + 8) != param_3))
        {
            ppVar6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff4, param_3));
            if (local_BX_8.g != 0)
            {
                if ((uVar5 | local_BX_8) == 0)
                {
                    puVar3 = 0x0;
                    uVar4 = 0;
                }
                else
                {
                    puVar3 = &local_BX_8.j;
                    uVar4 = uVar5;
                }
                pass1_1010_1ea6(local_BX_8.g, CONCAT22(uVar4, puVar3));
            }
            local_BX_8.g = ppVar6;
            if ((uVar5 | local_BX_8) == 0)
            {
                param_3 = 0x0;
                uVar4 = 0;
            }
            else
            {
                param_3 = &local_BX_8.j;
                uVar4 = uVar5;
            }
            uVar2 = local_BX_8.g;
            ppcVar1 = (local_BX_8.g + 4);
            (**ppcVar1)(0x1010, uVar2, (uVar2 >> 0x10), 0, param_3, uVar4);
        }
        pass1_1018_2862(param_1);
        if ((extraout_DX | param_3) != 0)
        {
            local_BX_8.h = 1;
        }
        pass1_1010_1f62(param_1, 7);
    }
    else
    {
        if (((&local_BX_8.g + 2) | &local_BX_8.g) != 0)
        {
            if ((uVar5 | local_BX_8) == 0)
            {
                puVar3 = 0x0;
                uVar4 = 0;
            }
            else
            {
                puVar3 = &local_BX_8.j;
                uVar4 = uVar5;
            }
            pass1_1010_1ea6(local_BX_8.g, CONCAT22(uVar4, puVar3));
            local_BX_8.g = 0;
            return;
        }
    }
    return;
}

pub fn pass1_1018_280c(param_1: *mut astruct_318)

{
    let mut u_var1: u32;
    let local_BX_4: *mut astruct_540;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x24 == 0)
    {
        return;
    }
    local_BX_4.field_0x24 = 0;
    if (local_BX_4.field_0x20 == 0)
    {
        local_BX_4.field_0x26 = 0;
    }
    else
    {
        uVar1 = local_BX_4.field_0x20;
        local_BX_4.field_0x26 = (uVar1 + 0x4c);
    }
    return;
}

pub fn pass1_1018_2862(param_1: *mut astruct_318)

{
    let mut u_var1: u32;
    let local_BX_4: *mut astruct_541;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x20 == 0)
    {
        local_BX_4.field_0x26 = 0;
    }
    else
    {
        uVar1 = local_BX_4.field_0x20;
        local_BX_4.field_0x26 = (uVar1 + 0x4c);
    }
    return;
}

pub fn pass1_1018_289c(param_1: *mut astruct_318, param_2: u16)

{
    let mut uVar1: i32;
    let mut extraout_DX: i32;

    if (param_2 == 1)
    {
        (param_1 + 4) = 0;
        return;
    }
    if (param_2 == 2)
    {
        pass1_1018_2922((param_1 & 0xffff0000 | (param_1 - 0x1c)));
    }
    else
    {
        if (((((param_2 - 3) < 1) || (SBORROW2(param_2 - 3, 1))) || (1 < (param_2 - 5))) ||
            ((param_1 + 4) == 0))
        {
            return;
        }
        uVar1 = param_1 - 0x1c;
        pass1_1018_2862((param_1 & 0xffff0000 | uVar1));
        if ((extraout_DX | uVar1) != 0)
        {
            (param_1 + 8) = 1;
        }
    }
    pass1_1010_1f62((param_1 & 0xffff0000 | (param_1 - 0x1c)),
                    param_2);
    return;
}

pub fn pass1_1018_2922(param_1: *mut astruct_318)

{
    let piVar1: *mut i32;
    let local_BX_3: *mut astruct_318;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    let pi_var1_val = unsafe{*piVar1};
    if ((local_BX_3.e != 0) &&
        (piVar1 = &local_BX_3.field_0x3e, pi_var1_val = pi_var1_val + 1,
         9 < &local_BX_3.field_0x3e))
    {
        &local_BX_3.field_0x3e = 0;
        &local_BX_3.field_0x42 = 1;
    }
    return;
}

pub fn pass1_1018_294a(param_1: *mut astruct_116, param_2: u16, param_3: u16,param_2_00: u32)

{
    if ((param_1.field_0x18 != 0) && (param_2_00._2_2_ == 0x280))
    {
        param_1.field_0x18 = 0;
    }
    create_drawing_dc_1018_4e04(CONCAT22(param_2, param_1), param_3, param_2_00, param_2_00._2_2_);
    return;
}



pub fn pass1_1018_2ab4(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1018_2440(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_2afa(param_1: *mut astruct_535)

{
    error_check_1000_17ce(param_1);
    return;
}

// WARNING: Variable defined which should be unmapped: local_16
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_2b10(param_1: *mut astruct_393, param_2: u16)

{
    let mut uVar1: u16;
    let paVar2: *mut astruct_393;
    let puVar3: *mut u16;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut extraout_DX: u16;
    let ppVar6: *mut pass1_struct_1;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let local_16: *mut astruct_393;
    let mut uStack20: u16;
    let pcStack16: *mut libc::c_char;
    let mut local_6: u16;
    let mut local_4: u16;
    let fn_ptr_1: fn();

    uVar1 = (pcStack16 >> 0x10);
    local_16 = param_1;
    paVar2 = local_16;
    uStack20 = (param_1 >> 0x10);
    get_sys_metrics_1018_4b1e(param_1, 1, param_2);
    local_16.field_0x20 = s_1_1050_389a;
    local_16.field_0x22 = &PTR_LOOP_1050_1008;
    local_16.field_0x20 = (s_18_2_1050_3aa5 + 3);
    local_16.field_0x22 = &PTR_LOOP_1050_1008;
    &local_16.field_0x24 = 0;
    local_16.field_0x174 = 0;
    local_16.field_0x176 = 0;
    local_16.field_0x178 = 0;
    &local_16.field_0x17a = 0;
    &local_16.field_0x17e = 0;
    &local_16.field_0x182 = 0;
    &local_16.field_0x186 = 0;
    param_1.field_0x0 = 0x32d8;
    local_16.u16_0x2 = 0x1018;
    local_16.field_0x20 = 0x3314;
    local_16.field_0x22 = 0x1018;
    pcStack16 = CONCAT22(uVar1, 0x2f);
    ppVar6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, pcStack16);
    local_16.field_0x182 = ppVar6;
    local_16.field_0x184 = (ppVar6 >> 0x10);
    if (param_1 == 0x0)
    {
        puVar3 = 0x0;
        uVar5 = 0;
    }
    else
    {
        puVar3 = &local_16.field_0x20;
        uVar5 = uStack20;
    }
    uVar4 = &local_16.field_0x182;
    uVar8 = uVar4;
    local_16 = (uVar4 >> 0x10);
    fn_ptr_1 = (&paVar2.field_0x182 + 4);
    (**fn_ptr_1)(0x10, uVar8, local_16, 0, puVar3, uVar5, uVar1);
    uVar4 = &paVar2.field_0x182;
    uVar4 = (uVar4 + 0x24);
    &paVar2.field_0x17a = uVar4;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x6e);
    paVar2.field_0x24 = uVar4;
    &paVar2.field_0x26 = extraout_DX;
    paVar2.field_0x28 = 0;
    uVar7 = process_struct_1008_4772(&paVar2.field_0x24);
    uVar1 = (uVar7 >> 0x10);
    pass1_1018_4b78(param_1, uVar8);
    paVar2.field_0x16c = 1;
    paVar2.field_0x16e = 1;
    paVar2.field_0x170 = (uVar7 + 4) + paVar2.field_0x16c;
    paVar2.field_0x172 = (uVar7 + 8) + -0x19;
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1018_2c60(in_struct_a: *mut astruct_376)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let struct_b_2: *mut astruct_45;
    let puVar4: *mut u16;
    let struct_b_1: *mut astruct_45;
    let struct_a_2: *mut astruct_376;
    let struct_a_1: *mut astruct_376;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_8625d1596ee: *mut u32;
    let temp_5f646cfbe4: *mut astruct_376;

    struct_a_1 = (in_struct_a >> 0x10);
    struct_a_2 = in_struct_a;
    in_struct_a.ptr_a_lo = 0x32d8;
    struct_a_2.ptr_a_hi = 0x1018;
    struct_a_2.ptr_2_lo = 0x3314;
    struct_a_2.ptr_2_hi = 0x1018;
    if (struct_a_2.ptr_3 != 0x0)
    {
        if ((struct_a_1 | struct_a_2) == 0)
        {
            struct_b_2 = 0x0;
            struct_b_1 = 0x0;
        }
        else
        {
            struct_b_2 = &struct_a_2.ptr_2_lo;
            struct_b_1 = struct_a_1;
        }
        pass1_1010_1ea6(struct_a_2.ptr_3, CONCAT22(struct_b_1, struct_b_2));
    }
    error_check_1000_17ce(struct_a_2.u32_x186);
    puVar1 = struct_a_2.ptr_4;
    uVar2 = struct_a_2.u16_x26;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;}
        (**ppcVar3)(0x1000, puVar1, uVar2, 1);
    }
    if (in_struct_a == 0x0)
    {
        puVar4 = 0x0;
        struct_a_1 = 0x0;
    }
    else
    {
        puVar4 = &struct_a_2.ptr_2_lo;
    }
    _local_6 = CONCAT22(struct_a_1, puVar4);
    *_local_6 = s_1_1050_389a;
    puVar4[1] = &PTR_LOOP_1050_1008;
    pass1_1010_1d80(in_struct_a);
    return;
}

pub fn pass1_1018_2d22(param_1: *mut astruct_104,param_2: u32,param_3: u32, param_4: u16)

{
    let mut u_var1: u32;

    param_3 = 0;
    param_2 = 0;
    uVar1 = process_struct_1008_4772((param_1 + 0x24));
    param_2 = (uVar1 + 8) + -0x14;
    if (param_4 == 3000)
    {
        param_3 = 5;
    }
    if (param_4 == 0xbba)
    {
        param_3 = 0x23;
    }
    if (param_4 == 0xbb9)
    {
        param_3 = 0x75;
    }
    return;
}

pub fn pass1_1018_2d84(param_1: u32)

{
    let uVar1: u8;
    let extraout_AH: u8;

    uVar1 = pass1_1018_2e28(param_1);
    big_switch_statement_1020_bd80(CONCAT11(extraout_AH, uVar1));
    return;
}

pub fn pass1_1018_2d9a(param_1: *mut astruct_545)

{
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let uVar3: u8;
    let extraout_AH: u8;
    let local_BX_4: *mut u8;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (((local_BX_4 + 0x180) | (local_BX_4 + 0x17e)) != 0)
    {
        piVar1 = (local_BX_4 + 0x174);
        unsafe{*piVar1 = *piVar1 + -1;
        if (*piVar1 < 0)
        {
            uVar2 = (local_BX_4 + 0x17e);
            (local_BX_4 + 0x174) = (uVar2 + 10) + -1;
        }}
        uVar3 = pass1_1018_2e28(param_1);
        (local_BX_4 + 0x176) = CONCAT11(extraout_AH, uVar3);
    }
    return;
}



pub fn pass1_1018_2dde(param_1: *mut astruct_545)

{
    let piVar1: *mut i32;
// ppuVar2: *mut *mut u8;
    let u8_ptr_3: *mut u8;
    let u8_ptr_1: *mut u8;
    let u8_ptr_2: *mut u8;
    let temp_5ff449779a: *mut u8;
    let mut temp_5fe5c244c1: u32;
    let temp_4188645ad6: *mut u8;

    u8_ptr_2 = (param_1 >> 0x10);
    u8_ptr_1 = param_1;
    if (((u8_ptr_1 + 0x180) | (u8_ptr_1 + 0x17e)) != 0)
    {
        piVar1 = (u8_ptr_1 + 0x174);
        unsafe{*piVar1 = *piVar1 + 1;}
        temp_5ff449779a = *(u8_ptr_1 + 0x174);
        temp_5fe5c244c1 = (u8_ptr_1 + 0x17e);
        ppuVar2 = (temp_5fe5c244c1 + 10);
        if (*ppuVar2 == temp_5ff449779a || *ppuVar2 < temp_5ff449779a)
        {
            (u8_ptr_1 + 0x174) = 0;
        }
        u8_ptr_3._0_1_ = pass1_1018_2e28(param_1);
        u8_ptr_3 = CONCAT11(u8_ptr_3._1_1_, u8_ptr_3);
        *(u8_ptr_1 + 0x176) = u8_ptr_3;
    }
    return;
}

pub fn pass1_1018_2e28(param_1: *mut astruct_545)

{
    let mut u_var1: u32;
    let mut extraout_DX: i32;
    let mut uVar2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = SEXT24((param_1 + 0x174));
    bad_func_1008_8fc4(*(param_1 + 0x17e), uVar1);
    if ((extraout_DX | uVar1) != 0)
    {
        return  * (uVar1 + 0x2e);
    }
    return '\0';
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_2e5e(param_1: *mut astruct_545)

{
    let mut u_var1: u32;
    let u8_ptr_5: *mut u8;
    let u8_ptr_3: *mut u8;
    let u8_ptr_4: *mut u8;
    let u8_ptr_1: *mut u8;
    let u8_ptr_2: *mut u8;
    let mut local_8: u16;
    let mut local_6: u16;
    let extraout_var: u32;

    u8_ptr_2 = (param_1 >> 0x10);
    u8_ptr_1 = param_1;
    if ((u8_ptr_1 + 0x17e) == 0)
    {
        pass1_1030_82f0(_g_bool_1050_5748, (u8_ptr_1 + 0x17a));
        *(u8_ptr_1 + 0x17e) = u8_ptr_5;
        *(u8_ptr_1 + 0x180) = u8_ptr_4;
    }
    if (((u8_ptr_1 + 0x17e) != 0) &&
        (uVar1 = (u8_ptr_1 + 0x17e), (uVar1 + 10) != 0))
    {
        bad_func_1008_8fc4(*(u8_ptr_1 + 0x17e), (u8_ptr_1 + 0x174));
        u8_ptr_3._0_1_ = pass1_1018_2e28(param_1);
        (u8_ptr_1 + 0x176) = CONCAT31(extraout_var, u8_ptr_3);
        return;
    }
    return;
}

pub fn pass1_1018_2ee4(param_1: *mut u8, param_2: u16)

{
    let mut u_var1: u32;
    let mut cVar2: u8;

    if (param_2 != 0x12)
    {
        if (param_2 < 0x13)
        {
            cVar2 = param_2;
            if (cVar2 == 0x1)
            {
                (param_1 + 0x162) = 0;
                return;
            }
            if ((0x2 < (cVar2 + -1)) && ((cVar2 + -4) < '\x03')) {}
              // goto LAB_1018_2f06;
        }
        return;
    }
    uVar1 = (param_1 + 0x162);
    (param_1 + 0x15a) = (uVar1 + 0x24);
// LAB_1018_2f06:
    pass1_1018_31fa((param_1 & 0xffff0000 | (param_1 - 0x20)));
    pass1_1010_1f62((param_1 & 0xffff0000 | (param_1 - 0x20)),
                    param_2);
    return;
}


pub fn pass1_1018_2fe8(param_1: *mut astruct_545)

{
    let piVar1: *mut i32;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let puVar4: *mut u8;
    let uVar5: u8;
    let mut uVar6: i32;
    let mut switch_var: u16;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let extraout_var_01: u32;
    let extraout_var_02: u32;
    let mut extraout_DX: u16;
    let mut uVar7: u16;
    let local_struct_1: *mut astruct_546;
    let mut uVar8: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar8 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    uVar2 = local_struct_1.field_0x174;
    puVar4 = local_struct_1.field_0x17e;
    iVar3 = (puVar4 + 10);
    if (iVar3 != 0)
    {
        if (local_struct_1.char_ptr_186 != 0x0)
        {
            uVar6 = get_string_index_1000_3da4(local_struct_1.char_ptr_186);
            local_struct_1.field_0x174 = 0;
            loop
            {
                if (iVar3 <= local_struct_1.field_0x174) {
                    break;}
                bad_func_1008_8fc4(local_struct_1.field_0x17e, local_struct_1.field_0x174);
                uVar7 = extraout_DX;
                uVar5 = pass1_1018_2e28(param_1);
                switch_var = CONCAT31(extraout_var, uVar5);
                big_switch_statement_1020_bd80(switch_var);
                uVar5 = process_string_1000_3de8(CONCAT22(uVar7, switch_var), local_struct_1.char_ptr_186, uVar6);
                if (CONCAT31(extraout_var_00, uVar5) == 0) {
                    break;}
                piVar1 = &local_struct_1.field_0x174;
                unsafe{*piVar1 = *piVar1 + 1;}
            }
            if (local_struct_1.field_0x174 < iVar3)
            {
                uVar5 = pass1_1018_2e28(param_1);
                local_struct_1.field_0x176 = CONCAT31(extraout_var_02, uVar5);
                return;
            }
            local_struct_1.field_0x174 = uVar2;
            uVar5 = pass1_1018_2e28(param_1);
            local_struct_1.field_0x176 = CONCAT31(extraout_var_01, uVar5);
        }
    }
    return;
}

pub fn pass1_1018_30ca(param_1: *mut astruct_547,param_2: u32)

{
    let uVar1: u8;
    let extraout_var: u32;
    let mut in_DX: u16;
    let local_struct_1: *mut astruct_547;
    let mut uvar3: u16;
    let mut uVar2: u16;

    uVar3 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    uVar1 = error_check_1000_17ce(&local_struct_1.field_0x186);
    uVar2 = CONCAT31(extraout_var, uVar1);
    pass1_fn_1008_60e8(param_2);
    &local_struct_1.field_0x186 = uVar2;
    local_struct_1.field_0x188 = in_DX;
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1018_30fc(param_1: *mut u8,param_2: u32)

{
    let paVar1: *mut astruct_548;
    let mut uVar2: u32;
    let puVar3: *mut u16;
    let local_AX_39: *mut astruct_548;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let in_DX: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    param_2 = 0;
    uVar6 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0x17e);
    paVar1 = (uVar2 + 10);
    if (paVar1 != 0x0)
    {
        local_AX_39 = paVar1;
        process_struct_1000_179c(6, in_DX);
        _local_12 = CONCAT22(in_DX, local_AX_39);
        struct_a = (in_DX | local_AX_39);
        if (struct_a == 0x0)
        {
            param_2 = 0;
        }
        else
        {
            *_local_12 = 0;
            local_AX_39.field_0x4 = 0;
            param_2 = _local_12;
        }
        uVar4 = paVar1 * 2;
        process_struct_1000_179c(uVar4, struct_a);
        puVar3 = param_2;
        unsafe{*puVar3 = uVar4;}
        (puVar3 + 2) = struct_a;
        (param_2 + 4) = paVar1;
        local_6 = 0;
        while (local_6 < paVar1)
        {
            uVar5 = SEXT24(local_6);
            bad_func_1008_8fc4(*(param_1 + 0x17e), uVar5);
            (param_2 + local_6 * 2) =
                (uVar5 + 0x2e);
            local_6 = local_6 + 1;
        }
    }
    return;
}

pub fn pass1_1018_31d0(param_1: *mut u8)

{
    let mut u_var1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if (((param_1 + 0x17e) != 0) &&
        (uVar1 = (param_1 + 0x17e), (uVar1 + 10) != 0))
    {
        return 1;
    }
    return 0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_31fa(param_1: *mut astruct_545)

{
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let rc: u8;
    let local_AX__1: *mut u8;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut extraout_DX: i32;
    let local_BX_4: *mut astruct_545;
    let local_ES_4: *mut astruct_545;
    let mut local_4: u16;
    let mut temp_5f2e721914: u32;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    pass1_1030_82f0(_g_bool_1050_5748, local_BX_4.field_0x17a);
    local_BX_4.field_0x17e = local_AX__1;
    local_BX_4.field_0x180 = extraout_DX;
    if (((extraout_DX | local_BX_4.field_0x17e) != 0) &&
        (temp_5f2e721914 = &local_BX_4.field_0x17e,
         iVar2 = (temp_5f2e721914 + 10), iVar2 != 0))
    {
        local_BX_4.field_0x174 = 0;
        loop
        {
            if (iVar2 <= local_BX_4.field_0x174) {
                break;}
            // WARNING: Load size is inaccurate
            bad_func_1008_8fc4(local_BX_4.field_0x17e, local_BX_4.field_0x174);
            rc = pass1_1018_2e28(param_1);
            if (local_BX_4.field_0x176 == CONCAT31(extraout_var, rc)) {
                break;}
            piVar1 = &local_BX_4.field_0x174;
            unsafe{*piVar1 = *piVar1 + 1;}
        }
        if (iVar2 <= local_BX_4.field_0x174)
        {
            local_BX_4.field_0x174 = 0;
        }
        rc = pass1_1018_2e28(param_1);
        local_BX_4.field_0x176 = CONCAT31(extraout_var_00, rc);
    }
    return;
}

pub fn pass1_1018_32b2(in_struct_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    pass1_1018_2c60(in_struct_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_331c(in_struct_1: *mut astruct_295,param_2: u32)

{
    let mut uVar1: u16;
    let mut unaff_BP: u16;
    let ppVar2: *mut pass1_struct_1;

    pass1_1008_ca5a(in_struct_1, param_2);
    &in_struct_1.field_0x122 = 0;
    in_struct_1.field_0x126 = 0;
    in_struct_1.field_0x12a = 0;
    in_struct_1.field_0x12e = 0;
    in_struct_1.field_0x130 = 0;
    in_struct_1.field_0x132 = 0;
    in_struct_1.field_0x136 = 0;
    in_struct_1.field_0x13a = 0;
    in_struct_1.field_0x13c = 0;
    in_struct_1.field_0x13e = 0;
    in_struct_1.field_0x142 = 0;
    CONCAT22(param_2, in_struct_1) = &PTR_LOOP_1050_470c;
    in_struct_1.field_0x2 = 0x1018;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, 0x3b));
    uVar1 = SUB42(ppVar2, 0);
    in_struct_1.field_0x122 = uVar1;
    in_struct_1.field_0x124 = (ppVar2 >> 0x10);
    *&in_struct_1.field_0x22 = 0;
    pass1_fn_1008_612e(8, 0xc);
    in_struct_1.field_0x13c = uVar1;
    return;
}

pub fn pass1_1018_33b4(param_1: *mut astruct_376)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_struct_1: *mut astruct_376;
    let mut uvar3: u16;
    let local_struct_3: *mut astruct_376;
    let fn_ptr_1: fn();

    uVar3 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = &PTR_LOOP_1050_470c;
    local_struct_1.ptr_a_hi = 0x1018;
    puVar1 = &local_struct_1.field_0x136;
    uVar2 = &local_struct_1.field_0x138;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{fn_ptr_1 = *puVar1;}
        (**fn_ptr_1)();
    }
    &local_struct_1.field_0x136 = 0;
    error_check_1000_17ce(&local_struct_1.field_0x126);
    error_check_1000_17ce(&local_struct_1.field_0x12a);
    pass1_1008_caa0(param_1);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_3424(param_1: *mut astruct_550)

{
    let mut u_var1: u32;
    let mut in_AX: u16;
    let ppVar2: *mut pass1_struct_2;
    let local_AX_85: *mut astruct_551;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uvar3: u16;
    let local_struct_1: *mut astruct_550;
    let mut uVar4: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    uVar1 = local_struct_1.field_0x122;
    pass1_1008_e852(uVar1, (uVar1 >> 0x10),
                    local_struct_1.field_0x126);
    _local_6 = CONCAT22(extraout_DX, in_AX);
    uVar1 = local_struct_1.field_0x122;
    pass1_1008_e852(uVar1, (uVar1 >> 0x10),
                    local_struct_1.field_0x12a);
    _local_a = CONCAT22(extraout_DX_00, in_AX);
    uVar4 = extraout_DX_00;
    ppVar2 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10),
                             _local_6);
    uVar3 = uVar4;
    local_AX_85 = 
        pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), _local_a);
    if (local_AX_85.field_0x200 == &ppVar2[1].field_0x9e)
    {
        return;
    }
    return;
}

pub fn pass1_1018_34a6(param_1: u32)

{
    pass1_1018_3d6c(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn wsprintf_1018_34b6(param_1: *mut astruct_298)

{
    let mut iVar1: i32;
    let mut in_AX: u16;
    let mut iVar2: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    string_fn_1018_3b9e(param_1, (iVar3 + 0x12e));
    iVar1 = (iVar3 + 0x12e);
    iVar2 = iVar1 + -0x188;
    if (iVar2 == 0)
    {
        pass1_1008_57f0(in_AX, extraout_DX, (iVar3 + 0x130));
        big_switch_statement_1020_c0d8((iVar2 + 0xe));
        uVar6 =  * (iVar3 + 0x132);
        uVar5 = SUB42(s__ld__s_1050_4150, 0);
    }
    else
    {
        if (iVar1 == 0x18b)
        {
            pass1_1008_57f0(in_AX, extraout_DX, (iVar3 + 0x130));
            uVar6 =  * (iVar3 + 0x132);
            uVar5 = SUB42(s__ld__s_1050_415e, 0);
        }
        else
        {
            if (iVar1 != 0x18c)
            {
                load_string_1010_84e0(_g_struct_73_1050_14cc,
                                      (_g_struct_73_1050_14cc >> 0x10), 0x100,
                                      (param_1 & 0xffff0000 | (iVar3 + 0x22)), 0x424);
                return;
            }
            pass1_1008_57f0(in_AX, extraout_DX, (iVar3 + 0x130));
            uVar6 =  * (iVar3 + 0x132);
            uVar5 = SUB42(s__ld__s_1050_4157, 0);
        }
    }
    wsprintf16((iVar3 + 0x22), CONCAT22(uVar5, uVar4), CONCAT22(uVar6, 0x1050));
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address


pub fn pass1_1018_3710(in_struct_1: *mut astruct_553)

{
    let mut u_var1: u32;
    let mut uVar2: u32;
    let local_AX_136: *mut astruct_554;
    let local_AX_232: *mut astruct_556;
    let mut iVar3: i32;
    let local_AX_394: *mut astruct_555;
    let paVar4: *mut astruct_199;
    let local_struct_1: *mut astruct_553;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u32;
    let mut uVar7: u32;
    let paVar8: *mut astruct_568;
    let lVar9: u32;
    let mut local_132: u16;
    let mut local_130: u16;
    let mut local_12e: u16;
    let mut local_12c: u16;
    let mut string_1: [u8;280];
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let fn_ptr_1: fn();

    local_6 = 0;
    uVar5 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    uVar6 = string_fn_1018_3b9e(in_struct_1, local_struct_1.field_0x12e);
    match (local_struct_1.field_0x12e)
    {
    0x188 =>{
        paVar8 = pass1_1008_57f0(uVar6, local_struct_1.field_0x130);
        paVar4 = (paVar8 >> 0x10);
        iVar3 = paVar8;
        process_struct_1000_179c(0x10, paVar4);
        if (paVar8 != 0x0)
        {
            local_6 = pass1_1018_4790(paVar8, local_struct_1.field_0x132, 0, (iVar3 + 0xe));
          // goto LAB_1018_3950;
        }
    },
    0x189 =>{
        uVar7 = pass1_1008_57f0(uVar6, local_struct_1.field_0x130);
        paVar4 = (uVar7 >> 0x10);
        local_AX_136 = uVar7;
        process_struct_1000_179c(0x14, paVar4);
        if (uVar7 != 0)
        {
            local_6 = pass1_1018_47c8(uVar7, local_struct_1.field_0x132, 0, local_AX_136.field_0x12,
                                      local_AX_136.field_0xe);
          // goto LAB_1018_3950;
        }
    },
    0x18a =>{
        paVar8 = pass1_1008_57f0(uVar6, local_struct_1.field_0x130);
        paVar4 = (paVar8 >> 0x10);
        local_AX_232 = paVar8;
        process_struct_1000_179c(0x12, paVar4);
        if (paVar8 != 0x0)
        {
            local_6 = pass1_1018_4808(paVar8, local_struct_1.field_0x132, 0, local_AX_232.field_0xe);
          // goto LAB_1018_3950;
        }
    },
    0x18b =>{
        paVar8 = pass1_1008_57f0(uVar6, local_struct_1.field_0x130);
        paVar4 = (paVar8 >> 0x10);
        iVar3 = paVar8;
        process_struct_1000_179c(0x14, paVar4);
        if (paVar8 != 0x0)
        {
            local_6 = pass1_1018_4842(paVar8, local_struct_1.field_0x132, 0, (iVar3 + 0xe));
          // goto LAB_1018_3950;
        }
    },
    0x18c =>{
        paVar8 = pass1_1008_57f0(uVar6, local_struct_1.field_0x130);
        paVar4 = (paVar8 >> 0x10);
        iVar3 = paVar8;
        process_struct_1000_179c(0x10, paVar4);
        if (paVar8 != 0x0)
        {
            local_6 = pass1_1018_48b0(paVar8, local_struct_1.field_0x132, 0, (iVar3 + 0xe));
          // goto LAB_1018_3950;
        }
    },
    0x18d =>{
        lVar9 = pass1_1008_57f0(uVar6, local_struct_1.field_0x130);
        paVar4 = (lVar9 >> 0x10);
        local_AX_394 = lVar9;
        process_struct_1000_179c(0x12, paVar4);
        if (lVar9 != 0)
        {
            uVar6 = local_AX_394.field_0xe;
            uVar1 = local_struct_1.field_0x132;
            local_6 = pass1_1018_4920(lVar9, uVar1, (uVar1 >> 0x10), 0, 0, uVar6,
                                      (uVar6 >> 0x10));
        }
    },
    _ => {}
      // goto LAB_1018_3950;
    }
    local_6 = 0;
// LAB_1018_3950:
    uVar6 = local_struct_1.field_0x122;
    uVar2 = pass1_1008_e852(uVar6, (uVar6 >> 0x10),
                            local_struct_1.field_0x126);
    uVar6 = local_struct_1.field_0x122;
    uVar7 = pass1_1008_e852(uVar6, (uVar6 >> 0x10),
                            local_struct_1.field_0x12a);
    pass1_1038_2a0e(CONCAT22(unaff_SS, string_1), local_struct_1.field_0x136, local_6,
                    uVar7, uVar2);
    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, string_1));
    local_struct_1.field_0x136 = 0;
    fn_ptr_1 = (in_struct_1 + 0x10);
    (**fn_ptr_1)(0x1030, in_struct_1);
    pass1_1038_2a5c(CONCAT22(unaff_SS, string_1));
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_39d8(param_1: u32,param_2: u32,param_3: u32) -> bool

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let lVar3: u32;
    let mut uVar4: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar4 = param_2;
    uVar2 = load_string_1010_847e(_g_struct_73_1050_14cc, (_g_struct_73_1050_14cc >> 0x10), 0x531);
    iVar1 = pass1_1000_3d7a(uVar2, uVar4);
    if (iVar1 != 0)
    {
        iVar1 = pass1_1000_3d7a(param_3, param_2);
        if (iVar1 != 0)
        {
            lVar3 = pass1_1018_4608(param_1, param_2, param_3);
            if ((lVar3 != 0) && ((lVar3 + 0xc) == 1))
            {
                return 1;
            }
        }
    }
    return 0;
}



pub fn pass1_1018_3a42(param_1: u32,param_2: u32)

{
    let mut temp_5fce46d9ec: u32;

    temp_5fce46d9ec = (param_1 + 0x122);
    pass1_1008_e852(temp_5fce46d9ec, (temp_5fce46d9ec >> 0x10),
                    param_2);
    return;
}

pub fn pass1_1018_3a5c(param_1: *mut astruct_309,param_2: u32,param_3: u32)

{
    pass1_1008_e320((param_1 + 0x122), param_2, param_3);
    return;
}



pub fn pass1_1018_3a7a(param_1: *mut u8,param_2: u32)

{
    let mut local_DXAX_16: u32;
    let mut temp_5f60f00366: u32;

    temp_5f60f00366 = (param_1 + 0x122);
    local_DXAX_16 =
        process_struct_1008_e586(temp_5f60f00366, (temp_5f60f00366 >> 0x10), param_2);
    return local_DXAX_16;
}

pub fn pass1_1018_3a94(param_1: *mut u8,param_2: u32,param_3: u32)

{
    process_struct_1008_e3ec((param_1 + 0x122), param_2, param_3);
    return;
}

pub fn pass1_1018_3ab2(in_struct_1: *mut astruct_557, param_2: u16, param_3: u16)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let local_struct_1: *mut astruct_557;
    let mut unaff_SS: u16;
    let lVar3: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: [u8;8];
    let mut local_8: u16;
    let mut local_6: u32;

    if (5 < param_3 - 0x188)
    {
        return 0;
    }
    local_struct_1 = in_struct_1;
    uVar2 = (in_struct_1 >> 0x10);
    match (param_3)
    {
    0x188 =>{
        uVar1 = local_struct_1.field_0xa;
        uVar2 = local_struct_1.field_0xc},
    0x189 =>{
        uVar1 = local_struct_1.field_0xe;
        uVar2 = local_struct_1.field_0x10},
    0x18a =>{
        uVar1 = local_struct_1.field_0x12;
        uVar2 = local_struct_1.field_0x14},
    0x18b =>{
        uVar1 = local_struct_1.field_0x16;
        uVar2 = local_struct_1.field_0x18},
    0x18c =>{
        uVar1 = local_struct_1.field_0x1a;
        uVar2 = local_struct_1.field_0x1c},
    0x18d =>{
        uVar1 = local_struct_1.field_0x1e;
        uVar2 = local_struct_1.field_0x20;
    }}
    local_6 = CONCAT22(uVar2, uVar1);
    local_8 = 0;
    pass1_1008_5784(CONCAT22(unaff_SS, local_10), local_6);
    loop
    {
        lVar3 = pass1_1008_5b12(CONCAT22(unaff_SS, local_10));
        uVar2 = (lVar3 >> 0x10);
        if ((lVar3 == 0) || (local_8 == param_2)) {
            break;}
        local_8 = local_8 + 1;
    }
    local_16 = 0;
    if (lVar3 != 0)
    {
        if ((lVar3 + 10) == 0)
        {
            local_16 = (lVar3 + 8);
        }
        else
        {
            local_16 = 0xffff;
        }
    }
    return local_16;
}



pub fn pass1_1018_3cda(in_struct_1: *mut astruct_559,param_2: u32,param_3: u32)

{
    let uVar1: u8;
    let mut uVar2: u16;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let local_struct_1: *mut astruct_559;
    let mut uVar4: u16;
    let fn_ptr_1: fn();

    uVar4 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    fn_ptr_1 = (in_struct_1 + 0x10);
    (**fn_ptr_1)();
    uVar3 = extraout_DX;
    error_check_1000_17ce(&local_struct_1.field_0x126);
    uVar1 = error_check_1000_17ce(&local_struct_1.field_0x12a);
    uVar2 = CONCAT31(extraout_var, uVar1);
    pass1_fn_1008_60e8(param_3, in_struct_1);
    local_struct_1.field_0x126 = uVar2;
    local_struct_1.field_0x128 = uVar3;
    pass1_fn_1008_60e8(param_2);
    local_struct_1.field_0x12a = uVar2;
    local_struct_1.field_0x12c = uVar3;
    return;
}

pub fn pass1_1018_3d44(param_1: *mut u8,param_2: u32,param_3: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_3 = (param_1 + 0x126);
    param_2 = (param_1 + 0x12a);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_3d6c(param_1: u32)

{
    let mut uVar1: i32;
    let mut bVar2: u8;
    let mut uVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar8: u32;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    uVar1 = (iVar5 + 0x142);
    uVar3 = uVar1 + 0x1e;
    if ((iVar5 + 0x144) + 1 == (uVar1 < 0xffe2))
    {
        if (uVar3 != 0x3c)
        {
            if (0x3c < uVar3)
            {
                return;
            }
            bVar2 = uVar3;
            if (bVar2 == 0x14)
            {
                (iVar5 + 0x142) = 0xffec;
// LAB_1018_3e3d:
                (iVar5 + 0x144) = 0xffff;
                return;
            }
            if (0x14 < bVar2)
            {
                if (bVar2 == 0x1e)
                {
                    if (u16_1050_13ae < 1)
                    {
                        return;
                    }
                    if (SBORROW2(u16_1050_13ae, 1))
                    {
                        return;
                    }
                    if (u16_1050_13ae != 2 && 0 < (u16_1050_13ae - 1))
                    {
                        iVar4 = u16_1050_13ae - 3;
                        if (iVar4 == 0)
                        {
                            pass1_fn_1008_612e(0x640001);
                            if (iVar4 < 0x32)
                            {
                                iVar4 = 10;
                            }
                            else
                            {
                                iVar4 = -10;
                            }
                            (iVar5 + 0x142) = iVar4;
                            (iVar5 + 0x144) = iVar4 >> 0xf;
                            return;
                        }
                        if (u16_1050_13ae != 4)
                        {
                            return;
                        }
                        (iVar5 + 0x142) = 0xfff6;
                      // goto LAB_1018_3e3d;
                    }
                    (iVar5 + 0x142) = 10;
                }
                else
                {
                    if (bVar2 == 0x28)
                    {
                        (iVar5 + 0x142) = 0x14;
                    }
                    else
                    {
                        if (bVar2 != 0x32)
                        {
                            return;
                        }
                        (iVar5 + 0x142) = 0x1e;
                    }
                }
                (iVar5 + 0x144) = 0;
                return;
            }
            if (bVar2 != 0)
            {
                if (bVar2 != 10)
                {
                    return;
                }
                (iVar5 + 0x142) = 0xffe2;
              // goto LAB_1018_3e3d;
            }
        }
        uVar8 = 5;
        uVar7 = pass1_1030_8326();
        if (uVar7 % uVar8 == 0)
        {
            (iVar5 + 0x142) = 0;
            return;
        }
    }
    return;
}

pub fn pass1_1018_3e8c(param_1: u16, param_2: u16, param_1_00: *mut u16, param_2_00: *mut u16)

{
    unsafe{
    *param_2_00 = 1;
    *param_1_00 = 0x19;}
    return;
}

pub fn pass1_1018_3ea4(param_1: *mut astruct_560)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_14: *mut astruct_560;
    let mut uVar4: u16;

    pass1_1008_cac6(param_1);
    uVar4 = (param_1 >> 0x10);
    local_BX_14 = param_1;
    puVar1 = local_BX_14.field_0x136;
    uVar2 = &local_BX_14.field_0x138;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;}
        (**ppcVar3)(&PTR_LOOP_1050_1008, puVar1, uVar2, 1);
    }
    &local_BX_14.field_0x136 = 0;
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_3ee6(param_1: u32,param_2: u32, param_3: *mut astruct_561, in_switch_param: u16) -> i32

{
    let mut iVar1: i32;
    let mut switch_var: u16;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let mut iVar6: i32;
    let mut uVar7: u32;
    let in_DX: *mut astruct_199;
    let paVar8: *mut astruct_199;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let lVar11: u32;
    let in_struct_73_low: *mut astruct_73;
    let in_struct_73_hi: *mut astruct_73;
    let mut uVar12: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    match (in_switch_param)
    {
    1 =>{
        iVar2 = param_3 * 4 + 0x40b6},
        _ => {
        iVar2 = param_3 * 4 + 0x40ce},
    3 =>{
        iVar2 = param_3 * 4 + 0x40e2},
    4 =>{
        iVar2 = param_3 * 4 + 0x40ee},
    8 =>{
        iVar2 = param_3 * 4 + 0x40f2},
    9 =>{
        iVar2 = param_3 * 4 + 0x4106},
    10 =>{
        iVar2 = param_3 * 4 + 0x410a},
    0x14 =>{
        iVar2 = param_3 * 4 + 0x410e},
    0x16 =>{
        iVar2 = param_3 * 4 + 0x4112},
    0x17 =>{
        iVar2 = param_3 * 4 + 0x4116},
    0x19 =>{
        iVar2 = param_3 * 4 + 0x411a;
    }}
    local_6 = CONCAT22(0x1050, iVar2);
    if (local_6 == 0)
    {
        return;
    }
    local_a = 0;
    iVar1 = local_6;
    uVar5 = iVar1 - 1;
    uVar12 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (uVar5 == 0)
    {
        pass1_1018_456a(uVar12, uVar3, (iVar2 + 2));
        _local_e = CONCAT22(in_DX, uVar5);
        big_switch_statement_1020_c0d8((iVar2 + 2));
        pass1_fn_1008_60e8(uVar5, in_DX);
        paVar8 = in_DX;
        uVar4 = uVar5;
        process_struct_1000_179c(0x10, in_DX);
        _local_16 = CONCAT22(paVar8, uVar4);
        if ((paVar8 | uVar4) == 0)
        {
// LAB_1018_40bc:
            uVar10 = 0;
            uVar9 = 0;
        }
        else
        {
            uVar7 = param_2 / _local_e;
            uVar9 = (param_2 % _local_e);
            pass1_1018_4790(_local_16, uVar7, CONCAT22(in_DX, uVar5), (iVar2 + 2));
            uVar10 = uVar7;
        }
    }
    else
    {
        iVar6 = iVar1 + -2;
        if (iVar6 == 0)
        {
            pass1_1018_451e(uVar12, uVar3, (iVar2 + 2));
            _local_12 = CONCAT22(in_DX, iVar6);
            uVar3 = big_switch_statement_1020_c222((iVar2 + 2));
            pass1_fn_1008_60e8(uVar3, in_DX);
            paVar8 = in_DX;
            uVar12 = uVar3;
            process_struct_1000_179c(0x10, in_DX);
            _local_16 = CONCAT22(paVar8, uVar12);
            if ((paVar8 | uVar12) == 0) {}
              // goto LAB_1018_40bc;
            uVar7 = param_2 / _local_12;
            uVar9 = (param_2 % _local_12);
            pass1_1018_48b0(_local_16, uVar7, CONCAT22(in_DX, uVar3), (iVar2 + 2));
            uVar10 = uVar7;
        }
        else
        {
            if (iVar1 == 3)
            {
                uVar5 = pass1_1008_c646(_PTR_LOOP_1050_06e0,
                                        CONCAT22((iVar2 + 2),
                                                 (_PTR_LOOP_1050_06e0 >> 0x10)));
                switch_var = uVar5;
                if (uVar5 == 0)
                {
                    switch_var = 0x4f;
                }
                pass1_1018_43ec(uVar12, uVar3, switch_var);
                _local_e = CONCAT22(in_DX, uVar5);
                big_switch_statement_1020_bd80(switch_var);
                pass1_fn_1008_60e8(uVar5, in_DX);
                _local_1a = CONCAT22(in_DX, uVar5);
                process_struct_1000_179c(0x14, in_DX);
                _local_16 = CONCAT22(in_DX, uVar5);
                if ((in_DX | uVar5) == 0) {}
                  // goto LAB_1018_40bc;
                uVar7 = param_2 / _local_e;
                uVar9 = (param_2 % _local_e);
                pass1_1018_47c8(_local_16, uVar7, _local_1a, switch_var, 0);
                uVar10 = uVar7;
            }
            else
            {
                if (iVar1 != 4) {}
                  // goto LAB_1018_425e;
                iVar2 = (iVar2 + 2);
                uVar5 = iVar2 - 1;
                in_struct_73_low = _g_struct_73_1050_14cc;
                in_struct_73_hi = (_g_struct_73_1050_14cc >> 0x10);
                if (uVar5 == 0)
                {
                    load_str_1010_84ac(in_struct_73_low, in_struct_73_hi, 0x430);
                    uVar4 = uVar5;
                    paVar8 = in_DX;
                    process_struct_1000_179c(0x14, in_DX);
                    _local_16 = CONCAT22(paVar8, uVar4);
                    if ((paVar8 | uVar4) == 0) {}
                      // goto LAB_1018_40bc;
                    uVar12 = 2;
                    lVar11 = 0x14;
                }
                else
                {
                    uVar5 = iVar2 - 2;
                    if (uVar5 == 0)
                    {
                        load_str_1010_84ac(in_struct_73_low, in_struct_73_hi, 0x431);
                        uVar4 = uVar5;
                        paVar8 = in_DX;
                        process_struct_1000_179c(0x14, in_DX);
                        _local_16 = CONCAT22(paVar8, uVar4);
                        if ((paVar8 | uVar4) == 0) {}
                          // goto LAB_1018_40bc;
                        uVar12 = 3;
                        lVar11 = 0x16;
                    }
                    else
                    {
                        uVar5 = iVar2 - 3;
                        if (uVar5 == 0)
                        {
                            load_str_1010_84ac(in_struct_73_low, in_struct_73_hi, 0x432);
                            uVar4 = uVar5;
                            paVar8 = in_DX;
                            process_struct_1000_179c(0x14, in_DX);
                            _local_16 = CONCAT22(paVar8, uVar4);
                            if ((paVar8 | uVar4) == 0) {}
                              // goto LAB_1018_40bc;
                            uVar12 = 4;
                            lVar11 = 0x17;
                        }
                        else
                        {
                            uVar5 = iVar2 - 4;
                            if (uVar5 != 0) {}
                              // goto LAB_1018_425e;
                            load_str_1010_84ac(in_struct_73_low, in_struct_73_hi, 0x433);
                            uVar4 = uVar5;
                            paVar8 = in_DX;
                            process_struct_1000_179c(0x14, in_DX);
                            _local_16 = CONCAT22(paVar8, uVar4);
                            if ((paVar8 | uVar4) == 0) {}
                              // goto LAB_1018_40bc;
                            uVar12 = 4;
                            lVar11 = 10;
                        }
                    }
                }
                uVar7 = param_2 / lVar11;
                uVar9 = (param_2 % lVar11);
                pass1_1018_4842(_local_16, uVar7, CONCAT22(in_DX, uVar5), uVar12);
                uVar10 = uVar7;
            }
        }
    }
    local_a = CONCAT22(uVar9, uVar10);
// LAB_1018_425e:
    uVar10 = (local_a >> 0x10);
    if ((local_a + 8) == 0)
    {
        (local_a + 8) = 1;
    }
    return;
}

pub fn pass1_1018_427c(in_struct_298_ptr_1: *mut astruct_298)

{
    let local_AX__1: *mut u8;
    let local_AX_172: *mut astruct_563;
    let local_AX_179: *mut astruct_564;
    let local_AX_184: *mut astruct_565;
    let local_DX__1: *mut u8;
    let struct_298_ptr_1: *mut astruct_298;
    let mut uVar1: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let astruct_566_ptr_1: *mut astruct_566;

    uVar1 = (in_struct_298_ptr_1 >> 0x10);
    struct_298_ptr_1 = in_struct_298_ptr_1;
    string_fn_1018_3b9e(in_struct_298_ptr_1, struct_298_ptr_1.astruct_566_ptr_x12e);
    astruct_566_ptr_1 = struct_298_ptr_1.astruct_566_ptr_x12e;
    local_AX_172 = (astruct_566_ptr_1 + -0xc4);
    if (local_AX_172 == 0x0)
    {
        pass1_1008_57f0(local_AX__1, local_DX__1, struct_298_ptr_1.field_0x130);
        pass1_1018_456a(struct_298_ptr_1, uVar1, local_AX_172.field_0xe);
    }
    else
    {
        local_AX_179 = (&astruct_566_ptr_1[-0xc6].field_0x0 + 1);
        if (local_AX_179 == 0x0)
        {
            pass1_1008_57f0(local_AX__1, local_DX__1, struct_298_ptr_1.field_0x130);
            pass1_1018_45d4(struct_298_ptr_1, uVar1, local_AX_179.field_0xe);
        }
        else
        {
            local_AX_184 = (astruct_566_ptr_1 + -0xc6);
            if (local_AX_184 == 0x0)
            {
                pass1_1008_57f0(local_AX__1, local_DX__1, struct_298_ptr_1.field_0x130);
                pass1_1018_451e(struct_298_ptr_1, uVar1, local_AX_184.field_0xe);
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_435e(param_1: u32,param_2: u32, param_3: i32, param_4: i32)

{
    let mut u_var1: u32;
    let mut in_switch_param: u16;
    let local_AX_105: *mut astruct_567;
    let mut uVar2: u16;
    let lVar3: u32;
    let mut uVar4: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_4: u16;

    if (param_3 < param_4)
    {
        param_4 = param_3;
    }
    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x122);
    uVar4 = pass1_1008_e852(uVar1, (uVar1 >> 0x10),
                            (param_1 + 0x126));
    pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), uVar4);
    while
    {
        while
        {
            in_switch_param = pass1_fn_1008_612e(param_4, param_3);
            local_AX_105 = (in_switch_param * 2 + 0x411c);
            local_AX_105 == 0x0
        } {}
        if (local_AX_105 != (&PTR_LOOP_1050_0000 + 1))
        {
            local_AX_105 = pass1_fn_1008_612e();
        }
        lVar3 = pass1_1018_3ee6(param_1, param_2, (local_AX_105 + -1), in_switch_param);
        lVar3 == 0
    }{}
    return;
}

pub fn pass1_1018_43ec(param_1: u16, param_2: u16, param_1: u16_00)

{
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_1_00)
    {
    0xf |
    0x35 |
    0x36 =>{
        local_6 = 7},
    _ => {
        local_6 = 1},
    0x11 |
    0x13 |
    0x14 |
    0x15 |
    0x2d |
    0x2e |
    0x6e =>{
        local_6 = 9},
    0x12 |
    0x31 |
    0x32 |
    0x52 |
    0x53 |
    0x54 |
    0x55 |
    0x56 |
    0x5a |
    0x5b |
    0x5c |
    0x5d |
    0x5e |
    0x5f =>{
        local_6 = 4},
    0x1b |
    0x1c |
    0x1d |
    0x28 |
    0x29 |
    0x2c |
    0x2f |
    0x30 |
    0x68 |
    0x69 =>{
        local_6 = 5},
    0x1e |
    0x1f |
    0x20 |
    0x33 |
    0x34 => {
        local_6 = 6},
    0x22 |
    0x23 |
    0x24 => {
        local_6 = 8},
    0x25 |
    0x26 |
    0x27 =>{
        local_6 = 2},
    0x38 |
    0x39 |
    0x4f |
    0x50 |
    0x51 |
    0x57 |
    0x58 |
    0x59 |
    0x66 |
    0x67 |
    0x6c |
    0x6d =>{
        local_6 = 3;
    }}
    return local_6;
}

pub fn pass1_1018_451e(param_1: u16, param_2: u16, param_1_00: i32)

{
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_1_00 == 7)
    {
        local_6 = 9;
    }
    else
    {
        if (param_1_00 == 8)
        {
            local_6 = 10;
        }
        else
        {
            if (param_1_00 == 0xc)
            {
                local_6 = 0x19;
            }
            else
            {
                if (param_1_00 == 0xd)
                {
                    local_6 = 3;
                }
                else
                {
                    local_6 = 8;
                }
            }
        }
    }
    return local_6;
}

pub fn pass1_1018_456a(param_1: u16, param_2: u16, param_1: u16_00)

{
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_1_00)
    {
    0x11 |
    0x12 |
    0x13 |
    0x14 |
    0x15 =>{
        local_6 = 2},
    0x16 |
    0x1e =>{
        local_6 = 3},
_ => {
        local_6 = 1},
    0x1d |
    0x21 =>{
        local_6 = 4;
    }}
    return local_6;
}

pub fn pass1_1018_45d4(param_1: u16, param_2: u16, param_1_00: i32)

{
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_1_00 == 3)
    {
        local_6 = 0x16;
    }
    else
    {
        if (param_1_00 == 4)
        {
            local_6 = 0x17;
        }
        else
        {
            local_6 = 0x14;
        }
    }
    return local_6;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_4608(param_1: u32,param_2: u32,param_3: u32) -> libc::c_long

{
    let mut u_var1: u32;
    let mut uVar2: i32;
    let paVar3: *mut astruct_493;
    let mut iVar4: i32;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let mut unaff_SS: u16;
    let lVar7: u32;
    let mut uVar8: u32;
    let mut uVar9: u32;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8;8];

    uVar1 = (param_1 + 0x122);
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (uVar1 + 10));
    loop
    {
        lVar7 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        uVar5 = (lVar7 >> 0x10);
        uVar2 = lVar7;
        uVar6 = uVar5 | uVar2;
        if (lVar7 == 0)
        {
            return 0;
        }
        uVar1 = (uVar2 + 4);
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        _local_16 = CONCAT22(uVar6, paVar3);
        uVar1 = (uVar2 + 8);
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        _local_1a = CONCAT22(uVar6, paVar3);
        uVar8 = pass1_1038_4d28(_local_16);
        uVar9 = pass1_1038_4d28(_local_1a);
        iVar4 = pass1_1000_3d7a(param_3, uVar8);
        if ((iVar4 == 0) && (iVar4 = pass1_1000_3d7a(param_2, uVar9), iVar4 == 0)) {
            break;}
        iVar4 = pass1_1000_3d7a(param_2);
        if ((iVar4 == 0) && (iVar4 = pass1_1000_3d7a(param_3), iVar4 == 0))
        {
            return lVar7;
        }
    }
    return lVar7;
}

pub fn pass1_1018_46e6(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    pass1_1018_33b4(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_4720(param_1: *mut astruct_568,param_2: u32,param_3: u32)

{
    let local_BX_4: *mut astruct_568;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.u16_x0 = s_1_1050_389a;
    local_BX_4.u16_x2 = &PTR_LOOP_1050_1008;
    local_BX_4.u32_x4 = param_3;
    local_BX_4.u32_x8 = param_2;
    local_BX_4.u32_xc = 0;
    param_1.u16_x0 = &PTR_LOOP_1050_4aa6;
    local_BX_4.u16_x2 = 0x1018;
    return;
}

pub fn pass1_1018_4760(in_struct_569_1: *mut astruct_569)

{
    let local_struct_569_ptr_1: *mut astruct_569;
    let mut uVar1: u16;

    uVar1 = (in_struct_569_1 >> 0x10);
    local_struct_569_ptr_1 = in_struct_569_1;
    in_struct_569_1.u16_x0 = &PTR_LOOP_1050_4aa6;
    local_struct_569_ptr_1.u16_x2 = 0x1018;
    error_check_1000_17ce(local_struct_569_ptr_1.astruct_376_ptr_x4);
    in_struct_569_1.u16_x0 = s_1_1050_389a;
    local_struct_569_ptr_1.u16_x2 = &PTR_LOOP_1050_1008;
    return;
}


pub fn pass1_1018_4790(param_1: *mut astruct_568,param_2: u32,param_3: u32, param_4: u16) -> *mut astruct_568
{
    let local_BX_23: *mut astruct_568;
    let mut uVar1: u16;

    pass1_1018_4720(param_1, param_2, param_3);
    uVar1 = (param_1 >> 0x10);
    local_BX_23 = param_1;
    &local_BX_23.u32_xe = param_4;
    param_1.u16_x0 = 0x4a92;
    local_BX_23.u16_x2 = 0x1018;
    local_BX_23.u32_xc = 1;
    return param_1;
}

pub fn pass1_1018_47c8(in_astruct_568_ptr_1: u32,param_2: u32,param_3: u32, param_4: u16,param_5: u32) -> i32

{
    let astruct_568_ptr_1: *mut astruct_568;
    let mut uVar1: u16;

    pass1_1018_4720(in_astruct_568_ptr_1, param_2, param_3);
    uVar1 = (in_astruct_568_ptr_1 >> 0x10);
    astruct_568_ptr_1 = in_astruct_568_ptr_1;
    astruct_568_ptr_1.u32_xe = param_5;
    astruct_568_ptr_1.u16_x12 = param_4;
    in_astruct_568_ptr_1 = &PTR_LOOP_1050_4a9a;
    astruct_568_ptr_1.u16_x2 = 0x1018;
    astruct_568_ptr_1.u32_xc = 2;
    return;
}

pub fn pass1_1018_4808(param_1: *mut astruct_568,param_2: u32,param_3: u32,param_4: u32)

{
    let astruct_568_ptr_1: *mut astruct_568;
    let mut uVar1: u16;

    pass1_1018_4720(param_1, param_2, param_3);
    uVar1 = (param_1 >> 0x10);
    astruct_568_ptr_1 = param_1;
    astruct_568_ptr_1.u32_xe = param_4;
    param_1.u16_x0 = &PTR_LOOP_1050_4aa2;
    astruct_568_ptr_1.u16_x2 = 0x1018;
    astruct_568_ptr_1.u32_xc = 3;
    return;
}


pub fn pass1_1018_4842(in_astruct_568_ptr_1: *mut astruct_568,param_2: u32,param_3: u32, param_4: u16) -> *mut astruct_568

{
    let astruct_568_ptr_1: *mut astruct_568;
    let mut uVar1: u16;

    pass1_1018_4720(in_astruct_568_ptr_1, param_2, param_3);
    uVar1 = (in_astruct_568_ptr_1 >> 0x10);
    astruct_568_ptr_1 = in_astruct_568_ptr_1;
    &astruct_568_ptr_1.u32_xe = param_4;
    (&astruct_568_ptr_1.u32_xe + 2) = 0;
    in_astruct_568_ptr_1.u16_x0 = &PTR_LOOP_1050_4a8e;
    astruct_568_ptr_1.u16_x2 = 0x1018;
    astruct_568_ptr_1.u32_xc = 4;
    return in_astruct_568_ptr_1;
}

pub fn pass1_1018_4882(param_1: *mut astruct_569)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.u16_x0 = &PTR_LOOP_1050_4a8e;
    (param_1 + 2) = 0x1018;
    error_check_1000_17ce((param_1 + 0x10));
    pass1_1018_4760(param_1);
    return;
}


pub fn pass1_1018_48b0(param_1: *mut astruct_568,param_2: u32,param_3: u32, param_4: u16) -> *mut astruct_568

{
    let local_BX_23: *mut astruct_568;
    let mut uVar1: u16;

    pass1_1018_4720(param_1, param_2, param_3);
    uVar1 = (param_1 >> 0x10);
    local_BX_23 = param_1;
    &local_BX_23.u32_xe = param_4;
    param_1.u16_x0 = &PTR_LOOP_1050_4a96;
    local_BX_23.u16_x2 = 0x1018;
    local_BX_23.u32_xc = 5;
    return param_1;
}

pub fn pass1_1018_48e8(param_1: *mut astruct_568,param_2: u32,param_3: u32, param_4: u16) -> *mut astruct_568

{
    let local_BX_23: *mut astruct_568;
    let mut uVar1: u16;

    pass1_1018_4720(param_1, param_2, param_3);
    uVar1 = (param_1 >> 0x10);
    local_BX_23 = param_1;
    &local_BX_23.u32_xe = param_4;
    param_1.u16_x0 = 0x4a9e;
    local_BX_23.u16_x2 = 0x1018;
    local_BX_23.u32_xc = 6;
    return param_1;
}

pub fn pass1_1018_4920(param_1: *mut astruct_568,param_2: u32,param_3: u32,param_4: u32)

{
    let local_BX_24: *mut astruct_568;
    let mut uVar1: u16;

    pass1_1018_4720(param_1, param_2, param_3);
    uVar1 = (param_1 >> 0x10);
    local_BX_24 = param_1;
    local_BX_24.u32_xe = param_4;
    param_1.u16_x0 = &PTR_LOOP_1050_4a8a;
    local_BX_24.u16_x2 = 0x1018;
    local_BX_24.u32_xc = 7;
    return;
}

pub fn pass1_1018_495a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_4980(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_49a6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_49cc(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_49f2(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1018_4882(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_4a18(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_4a3e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_4a64(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1018_4760(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_4aaa(param_1: *mut astruct_534, param_2: u16, param_3: u16)

{
    pass1_1018_4cda(param_1, CONCAT22(param_3, param_2));
    CONCAT22(param_2, param_1) = 0x4b06;
    param_1.u16_x02 = 0x1018;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x9a);
    _PTR_LOOP_1050_4230 = CONCAT22(param_2, param_1);
    return;
}

pub fn pass1_1018_4ae0(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    win_cleanup_1018_4d22(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1018_4b78(param_1: *mut astruct_393, param_2: u16)

{
    let ppcVar1: fn();
    let puVar2: *mut u32;
    let mut extraout_DX: i32;
    let mut unaff_SI: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut local_8: u16;

    pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24((param_1 + 10))), 0, 8);
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x18)), 0, 8);
    ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
    puVar2 = ppVar3;
    pass1_1010_5f7a(puVar2, (ppVar3 >> 0x10), 0, (param_1 + 0x12));
    if ((extraout_DX | puVar2) != 0)
    {
        unsafe{(param_1 + 10) = *puVar2;
        (param_1 + 0xe) = puVar2[1];}
    }
    ppcVar1 = (param_1 + 0x20);
    (**ppcVar1)(0x1010, param_1);
    if (((param_1 + 0xe) == 0) && ((param_1 + 0x10) == 0))
    {
        (param_1 + 10) = (param_1 + 0x18);
        (param_1 + 0xc) = (param_1 + 0x1a);
    }
    (param_1 + 0xe) = (param_1 + 0x1c);
    (param_1 + 0x10) = (param_1 + 0x1e);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_4c2c(param_1: u32, param_2: *mut u32, param_3: u16)

{
    let mut unaff_SI: u16;
    let ppVar1: *mut pass1_struct_1;

    unsafe{(param_1 + 10) = *param_2;
    (param_1 + 0xe) = param_2[1];}
    ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
    pass1_1010_5fb0(ppVar1, 0, (param_1 + 10), param_1._2_2_,
                    (param_1 + 0x12));
    return;
}

pub fn pass1_1018_4c78(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_4cda(param_1: *mut astruct_534,param_2: u32)

{
    let mut uVar1: u16;

    uVar1 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar1, param_1), (param_2 >> 0x10));
    param_1.u16_x0a = 0;
    param_1.u16_x0e = 0;
    param_1.u16_x12 = 0;
    param_1.u16_x14 = 0;
    param_1.u16_x16 = 0;
    param_1.u16_x18 = 1;
    param_1.u16_x1a = 0;
    CONCAT22(uVar1, param_1) = (s_SinternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12);
    param_1.u16_x02 = 0x1018;
    return;
}


pub fn pass1_1018_4dce(param_1: *mut u32, param_2: u16)

{
    let mut uVar1: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000ffe0: u32;
    let in_string_1: *mut libc::c_char;
    let mut local_8: u16;
    let fn_ptr_1: fn();

    in_string_1 = CONCAT22((in_stack_0000ffe0 >> 0x10), 0x48);
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, in_string_1);
    uVar1 = (ppVar2 >> 0x10);
    unsafe{fn_ptr_1 = (*param_1 + 0x10);}
    (**fn_ptr_1)(0x1010, param_1, param_2, (ppVar2 + 0xc),
                 (ppVar2 + 10), (in_string_1 >> 0x10));
    return;
}



pub fn pass1_1018_5032(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    win_cleanup_1018_4d22(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Variable defined which should be unmapped: u16_1

pub fn pass1_1018_5070(param_1: *mut astruct_375,param_2: u32)

{
    let mut u16_1: u16;
    let mut uStack4: u16;

    u16_1 = param_2;
    uStack4 = (param_2 >> 0x10);
    process_struct_1010_1d48(CONCAT22(u16_1, param_1), uStack4);
    &param_1.ptr_2_lo = 0;
    &param_1.u32_x0e = 0;
    param_1.u32_x12 = 0;
    &param_1.u16_x16 = 0;
    CONCAT22(u16_1, param_1) = 0x56d2;
    param_1.ptr_1_hi = 0x1018;
    return;
}

pub fn pass1_1018_50ac(param_1: *mut astruct_376)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_BX_5: *mut astruct_376;
    let mut local_ES_5: u16;
    let temp_86210a0b78d: *mut u32;
    let fn_ptr_1: fn();

    local_ES_5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1.ptr_a_lo = 0x56d2;
    local_BX_5.ptr_a_hi = 0x1018;
    puVar1 = local_BX_5.u16_x0e;
    uVar2 = local_BX_5.u16_x10;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{fn_ptr_1 = *puVar1;}
        (**fn_ptr_1)();
    }
    pass1_1010_1d80(param_1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_50ea(param_1: u32, param_2: u16,param_3: u32)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let struct_579_ptr_1: *mut astruct_579;
    let mut extraout_DX: i32;
    let struct_580_ptr_1: *mut astruct_580;
    let struct_581_ptr_1: *mut astruct_581;
    let mut uVar4: i32;
    let mut local_ES_104: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut temp_5f37dc446a: u32;
    let puVar3: *mut u8;
    let fn_ptr_1: fn();

    puVar3 = _PTR_LOOP_1050_68a2;
    alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
    uVar4 = puVar3;
    local_6 = puVar3 & 0xffff | extraout_DX << 0x10;
    if ((extraout_DX | uVar4) == 0)
    {
        local_6 = 0;
    }
    else
    {
        local_6 = s_1_1050_389a;
        (uVar4 + 2) = &PTR_LOOP_1050_1008;
        (uVar4 + 4) = 0;
        (uVar4 + 6) = 0;
        (uVar4 + 8) = 0;
        (uVar4 + 10) = 0;
        (uVar4 + 0xc) = 0;
        local_6 = 0x56ce;
        (uVar4 + 2) = 0x1018;
    }
    uVar4 = (local_6 >> 0x10);
    struct_580_ptr_1 = local_6;
    struct_580_ptr_1.field_0xa = param_2;
    local_ES_104 = (param_1 >> 0x10);
    struct_581_ptr_1 = param_1;
    temp_5f37dc446a = struct_581_ptr_1.field_0xa;
    iVar1 = (temp_5f37dc446a + 0xc);
    if (iVar1 == 1)
    {
        uVar2 = struct_581_ptr_1.field_0xa;
        struct_580_ptr_1.field_0x4 = (uVar2 + 0xe);
    }
    else
    {
        if (iVar1 == 5)
        {
            uVar2 = struct_581_ptr_1.field_0xa;
            struct_580_ptr_1.field_0x6 = (uVar2 + 0xe);
        }
        else
        {
            if (iVar1 != 6)
            {
                if ((uVar4 | struct_580_ptr_1) == 0)
                {
                    return;
                }
                fn_ptr_1 = local_6;
                (**fn_ptr_1)();
                return;
            }
            uVar2 = struct_581_ptr_1.field_0xa;
            struct_580_ptr_1.field_0x8 = (uVar2 + 0xe);
        }
    }
    pass1_1030_6c66(param_3, 1, local_6);
    return;
}

pub fn pass1_1018_51d2(param_1: *mut astruct_582)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_BX_4: *mut astruct_582;
    let mut uvar3: u16;
    let fn_ptr_1: fn();

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    puVar1 = &local_BX_4.field_0xe;
    uVar2 = local_BX_4.field_0x10;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{fn_ptr_1 = *puVar1;}
        (**fn_ptr_1)();
    }
    &local_BX_4.field_0xe = 0;
    return;
}

pub fn pass1_1018_5206(param_1: *mut astruct_583)

{
    let mut iVar1: i32;
    let mut uVar2: i32;
    let local_BX_4: *mut astruct_583;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut uVar4: u32;
    let mut local_a: [u8;8];

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    &local_BX_4.field_0xa = 0;
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), local_BX_4.field_0xe);
    while
    {
        uVar4 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        uVar2 = (uVar4 >> 0x10);
        local_BX_4.field_0xa = uVar4;
        local_BX_4.field_0xc = uVar2;
        if ((uVar2 | local_BX_4.field_0xa) == 0) {
            break;}
        uVar4 = &local_BX_4.field_0xa;
        iVar1 = pass1_1000_3d7a((uVar4 + 4));
        iVar1 != 0
    } {}
    return CONCAT22(local_BX_4.field_0xc, local_BX_4.field_0xa);
}

pub fn pass1_1018_526a(param_1: *mut astruct_584,param_2: u32)

{
    let local_BX_3: *mut astruct_584;
    let mut uVar1: i32;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (&local_BX_3.field_0xe == 0)
    {
        pass1_1018_5292((param_1 & 0xffff | uVar1 << 0x10), param_2);
    }
    return CONCAT22(local_BX_3.field_0x10, local_BX_3.field_0xe);
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_5292(param_1: *mut astruct_585,param_2: u32)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let puVar4: *mut u32;
    let paVar5: *mut astruct_199;
    let BVar6: bool;
    let puVar7: *mut u8;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let local_AX_626: *mut astruct_586;
    let paVar11: *mut astruct_586;
    let extraout_DX: *mut astruct_199;
    let paVar13: *mut astruct_199;
    let mut uVar14: i32;
    let extraout_DX_00: *mut astruct_199;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: u16;
    let extraout_DX_04: *mut astruct_199;
    let mut extraout_DX_05: u16;
    let mut extraout_DX_06: i32;
    let mut extraout_DX_07: u16;
    let extraout_DX_08: *mut astruct_199;
    let mut extraout_DX_09: u16;
    let extraout_DX_10: *mut astruct_199;
    let mut extraout_DX_11: u16;
    let local_BX_4: *mut astruct_585;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut unaff_SS: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_26: [u8;8];
    let mut local_1e: u16;
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
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut uVar12: u32;

    uVar15 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    puVar4 = &local_BX_4.field_0xe;
    paVar5 = local_BX_4.field_0x10;
    local_12 = puVar4;
    local_10 = paVar5;
    local_e = puVar4;
    local_c = paVar5;
    if ((paVar5 | puVar4) != 0)
    {
        unsafe{ppcVar3 = *puVar4;}
        (**ppcVar3)();
        paVar5 = extraout_DX;
    }
    process_struct_1000_179c(0xc, paVar5);
    paVar13 = (paVar5 | puVar4);
    local_12 = puVar4;
    local_10 = paVar5;
    if (paVar13 == 0x0)
    {
        uVar12 = 0;
        paVar13 = 0x0;
    }
    else
    {
        paVar5 = process_struct_1008_574a(CONCAT22(paVar5, puVar4));
        uVar12 = ZEXT24(paVar5);
    }
    &local_BX_4.field_0xe = uVar12;
    local_BX_4.field_0x10 = paVar13;
    local_4 = 0x21;
    while (-1 < local_4)
    {
        pass1_1030_7c28(param_2, local_4);
        _local_16 = uVar12 & 0xffff | ZEXT24(paVar13) << 0x10;
        paVar13 = (paVar13 | uVar12);
        if (paVar13 != 0x0)
        {
            pass1_1020_c0ca(local_4);
            pass1_fn_1008_60e8(uVar12, paVar13);
            _local_1a = uVar12 & 0xffff | ZEXT24(paVar13) << 0x10;
            process_struct_1000_179c(0x10, paVar13);
            local_e = uVar12;
            local_c = paVar13;
            if ((paVar13 | local_e) == 0)
            {
                uVar12 = 0;
                uVar14 = 0;
            }
            else
            {
                pass1_1018_4790((uVar12 & 0xffff | ZEXT24(paVar13) << 0x10), _local_16,
                                _local_1a, local_4);
                uVar14 = extraout_DX_01;
            }
            _local_1e = uVar12 & 0xffff | uVar14 << 0x10;
            uVar2 = &local_BX_4.field_0xe;
            ppcVar3 = (&local_BX_4.field_0xe + 4);
            (**ppcVar3)(0, uVar2, (uVar2 >> 0x10), uVar12, uVar14);
            paVar13 = extraout_DX_00;
        }
        local_4 = local_4 - 1;
    }
    _local_8 = pass1_1030_73a8(param_2);
    local_a = (_local_8 + 0xc);
    BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, local_a, 4);
    if (BVar6 != 0)
    {
        _local_1e = _local_8;
        _local_1a = (_local_8 + 0x20);
        pass1_1008_5784(CONCAT22(unaff_SS, local_26), _local_1a);
        loop
        {
            puVar7 = local_26;
            pass1_1008_5b12(CONCAT22(unaff_SS, puVar7));
            _local_16 = CONCAT22(extraout_DX_02, puVar7);
            paVar5 = (extraout_DX_02 | puVar7);
            if (paVar5 == 0x0) {
                break;}
            iVar1 = (puVar7 + 6);
            iVar8 = iVar1 + -7;
            if (iVar8 == 0)
            {
// LAB_1018_53f0:
                uVar9 = big_switch_statement_1020_c222((puVar7 + 6));
                pass1_fn_1008_60e8(uVar9, paVar5);
                paVar13 = paVar5;
                uVar10 = uVar9;
                process_struct_1000_179c(0x10, paVar5);
                local_12 = uVar10;
                local_10 = paVar13;
                if ((paVar13 | uVar10) == 0)
                {
                    uVar14 = 0;
                    uVar16 = 0;
                }
                else
                {
                    uVar16 = (_local_16 >> 0x10);
                    uVar14 = uVar10;
                    pass1_1018_48b0(CONCAT22(paVar13, uVar10),
                                     * (_local_16 + 10), CONCAT22(paVar5, uVar9),
                                    (_local_16 + 6));
                    uVar16 = extraout_DX_03;
                }
                uVar2 = &local_BX_4.field_0xe;
                ppcVar3 = (&local_BX_4.field_0xe + 4);
                (**ppcVar3)(0, uVar2, (uVar2 >> 0x10), uVar14, uVar16);
                paVar5 = extraout_DX_04;
            }
            else
            {
                if (((5 < iVar8) && (!SBORROW2(iVar8, 6))) && (iVar1 + -0xd < 2)) {}
                  // goto LAB_1018_53f0;
            }
            uVar16 = (_local_16 >> 0x10);
            if ((_local_16 + 8) != 0)
            {
                uVar9 = big_switch_statement_1020_c2f8(*(_local_16 + 8));
                pass1_fn_1008_60e8(uVar9, paVar5);
                paVar13 = paVar5;
                uVar10 = uVar9;
                process_struct_1000_179c(0x10, paVar5);
                local_e = uVar10;
                local_c = paVar13;
                if ((paVar13 | uVar10) == 0)
                {
                    uVar14 = 0;
                    uVar16 = 0;
                }
                else
                {
                    uVar16 = (_local_16 >> 0x10);
                    uVar14 = uVar10;
                    pass1_1018_48e8(CONCAT22(paVar13, uVar10),
                                     * (_local_16 + 10), CONCAT22(paVar5, uVar9),
                                    (_local_16 + 8));
                    uVar16 = extraout_DX_05;
                }
                uVar2 = &local_BX_4.field_0xe;
                ppcVar3 = (&local_BX_4.field_0xe + 4);
                (**ppcVar3)(0, uVar2, (uVar2 >> 0x10), uVar14, uVar16);
            }
        }
    }
    uVar16 = (param_2 >> 0x10);
    uVar12 = (param_2 + 0x3e);
    uVar14 = (param_2 + 0x40);
    local_32 = uVar12;
    if ((uVar14 | local_32) != 0)
    {
        pass1_1008_5784(CONCAT22(unaff_SS, local_26), uVar12 & 0xffff | uVar14 << 0x10);
        loop
        {
            local_AX_626 = local_26;
            pass1_1008_5b12(CONCAT22(unaff_SS, local_AX_626));
            paVar5 = (extraout_DX_06 | local_AX_626);
            if (paVar5 == 0x0) {
                break;}
            if (local_AX_626.field_0x4 != 0)
            {
                paVar11 = local_AX_626;
                big_switch_statement_1020_c0d8(local_AX_626.field_0x4);
                pass1_fn_1008_60e8(paVar11, paVar5);
                _local_1e = CONCAT22(paVar5, paVar11);
                process_struct_1000_179c(0x10, paVar5);
                local_12 = paVar11;
                local_10 = paVar5;
                if ((paVar5 | paVar11) == 0)
                {
                    paVar11 = 0x0;
                    uVar16 = 0;
                }
                else
                {
                    pass1_1018_4790(CONCAT22(paVar5, paVar11), local_AX_626.field_0xa,
                                    _local_1e, local_AX_626.field_0x4);
                    uVar16 = extraout_DX_07;
                }
                _local_1a = CONCAT22(uVar16, paVar11);
                uVar2 = &local_BX_4.field_0xe;
                ppcVar3 = (&local_BX_4.field_0xe + 4);
                (**ppcVar3)(0, uVar2, (uVar2 >> 0x10), paVar11, uVar16);
                paVar5 = extraout_DX_08;
            }
            if (local_AX_626.field_0x6 != 0)
            {
                uVar14 = big_switch_statement_1020_c222(local_AX_626.field_0x6);
                pass1_fn_1008_60e8(uVar14, paVar5);
                _local_1e = CONCAT22(paVar5, uVar14);
                process_struct_1000_179c(0x10, paVar5);
                local_e = uVar14;
                local_c = paVar5;
                if ((paVar5 | uVar14) == 0)
                {
                    uVar14 = 0;
                    uVar16 = 0;
                }
                else
                {
                    pass1_1018_48b0(CONCAT22(paVar5, uVar14), local_AX_626.field_0xa,
                                    _local_1e, local_AX_626.field_0x6);
                    uVar16 = extraout_DX_09;
                }
                _local_1a = CONCAT22(uVar16, uVar14);
                uVar2 = &local_BX_4.field_0xe;
                ppcVar3 = (&local_BX_4.field_0xe + 4);
                (**ppcVar3)(0, uVar2, (uVar2 >> 0x10), uVar14, uVar16);
                paVar5 = extraout_DX_10;
            }
            if (local_AX_626.field_0x8 != 0)
            {
                uVar14 = big_switch_statement_1020_c2f8(local_AX_626.field_0x8);
                pass1_fn_1008_60e8(uVar14, paVar5);
                _local_1e = CONCAT22(paVar5, uVar14);
                process_struct_1000_179c(0x10, paVar5);
                local_12 = uVar14;
                local_10 = paVar5;
                if ((paVar5 | uVar14) == 0)
                {
                    uVar14 = 0;
                    uVar16 = 0;
                }
                else
                {
                    pass1_1018_48e8(CONCAT22(paVar5, uVar14), local_AX_626.field_0xa,
                                    _local_1e, local_AX_626.field_0x8);
                    uVar16 = extraout_DX_11;
                }
                _local_1a = CONCAT22(uVar16, uVar14);
                uVar2 = &local_BX_4.field_0xe;
                ppcVar3 = (&local_BX_4.field_0xe + 4);
                (**ppcVar3)(0, uVar2, (uVar2 >> 0x10), uVar14, uVar16);
            }
        }
    }
    return;
}

pub fn pass1_1018_567c(param_1: *mut u16, param_2: u8)

{
    unsafe{*param_1 = s_1_1050_389a;}
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        free_mem_1000_093a(param_1);
    }
    return param_1;
}

pub fn pass1_1018_56a8(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    pass1_1018_50ac(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_56e6(param_1: *mut astruct_587,param_2: u32)

{
    let mut in_EAX: u32;
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar1 = (in_EAX >> 0x10);
    uVar2 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar2, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    CONCAT22(uVar2, param_1) = 0x5830;
    param_1.field_0x2 = 0x1018;
    return CONCAT22(uVar1, param_1);
}

pub fn pass1_1018_5714(param_1: *mut astruct_376)

{
    param_1.ptr_a_lo = 0x5830;
    (param_1 + 2) = 0x1018;
    pass1_1010_1d80(param_1);
    return;
}

pub fn pass1_1018_5732(param_1: u16, param_2: u16,param_1_00: u32)

{
    pass1_1030_6d4e(param_1_00);
    return;
}

pub fn pass1_1018_5742(param_1: u16, param_2: u16, param_1_00: *mut u32,param_2_00: u32)

{
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut bVar4: bool;
    let puVar5: *mut u32;
    let mut uVar6: u32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    bVar4 = false;
    puVar1 = (param_1_00 + 4);
    unsafe{ppcVar2 = (*puVar1 + 0x10);}
    puVar5 = puVar1;
    ppcVar2();
    uVar3 = puVar5 & 0xffff | extraout_DX << 0x10;
    local_10 = 0;
    loop
    {
        if (uVar3 <= local_10)
        {
// LAB_1018_579f:
            if (!bVar4)
            {
                if (param_1_00 != 0x0)
                {
                    unsafe{ppcVar2 = *param_1_00;}
                    ppcVar2();
                }
                param_1_00 = 0x0;
            }
            pass1_1030_6d80(param_2_00, param_1_00);
            return;
        }
        unsafe{ppcVar2 = (*puVar1 + 4);}
        uVar6 = uVar3;
        ppcVar2();
        if ((extraout_DX_00 | uVar6) != 0)
        {
            bVar4 = true;
          // goto LAB_1018_579f;
        }
        local_10 = local_10 + 1;
    } 
}

pub fn pass1_1018_57d2(param_1: u32,param_2: u32)

{
    (param_1 + 10) = param_2;
    return;
}

pub fn pass1_1018_57e6(param_1: u32,param_2: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    send_dialog_item_msg_1040_d20c((param_1 + 10), param_2);
    (param_1 + 10) = 0;
    return;
}

pub fn pass1_1018_580a(in_struct_376_ptr_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    pass1_1018_5714(in_struct_376_ptr_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(in_struct_376_ptr_1);
    }
    return in_struct_376_ptr_1;
}


pub fn pass1_1018_58b6(param_1: *mut astruct_591)

{
    let local_BX_3: *mut astruct_591;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    param_1.u16_x00 = (s_Alloc__s_1050_5a5b + 7);
    local_BX_3.u16_x02 = 0x1018;
    local_BX_3.u16_xe2 = 0x5afe;
    local_BX_3.u16_xe4 = 0x1018;
    process_struct_1020_808e(param_1);
    return;
}

pub fn invalidate_rect_1018_58e2(param_1: u32, param_2: i32)

{
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let mut uvar3: u16;

    if (param_2 == 0x105)
    {
        uVar3 = (param_1 >> 0x10);
        iVar2 = param_1;
        piVar1 = (iVar2 + 0xf6);
        unsafe{*piVar1 = *piVar1 + 1;}
        if (PTR_PTR_DAT_0005_0000_1050_0004_1050_4240 <= (iVar2 + 0xf6))
        {
            PostMessage16(0, 0xca, 0x111, g_h_window);
            return;
        }
        (iVar2 + 0xea) = 0;
        InvalidateRect16(0, 0x0, 0);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_5932(param_1: *mut astruct_592) -> i32

{
    let ppcVar1: fn();
    let mut uVar2: i32;
    let local_BX_5: *mut astruct_592;
    let mut uvar3: u16;
    let puVar4: *mut u8;

    uVar3 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    uVar2 = local_BX_5.field_0xf0 | &local_BX_5.field_0xee;
    if (uVar2 != 0)
    {
        ppcVar1 = (&local_BX_5.field_0xee + 8);
        uVar2 = (**ppcVar1)();
    }
    if (local_BX_5.field_0xea == 0)
    {
        local_BX_5.field_0xea = 1;
        puVar4 = pass1_1038_af40(_g_astruct_112_a, local_BX_5.field_0x8,
                                 (local_BX_5.field_0xf6 * 2 + 0x4238));
        uVar2 = puVar4;
    }
    return uVar2;
}

pub fn pass1_1018_598c(in_struct_a: *mut win_struct_42)

{
    let paVar1: *mut astruct_595;
    let struct_b_2: *mut astruct_199;
    let struct_a_1: *mut win_struct_42;
    let struct_a_2: *mut win_struct_42;
    let struct_b: *mut astruct_199;
    let local_DXAX_61: *mut u8;
    let mut local_4: u16;

    struct_b = create_win_1008_9760(in_struct_a);
    struct_b_2 = (struct_b >> 0x10);
    struct_a_2 = (in_struct_a >> 0x10);
    struct_a_1 = in_struct_a;
    paVar1 = 
        get_gui_dc_1018_4db0(*&struct_a_1.u32_xf2, struct_a_1.win_handle_0x8);
    process_struct_1000_179c(0x2a, struct_b_2);
    if ((struct_b_2 | paVar1) != 0)
    {
        local_DXAX_61 = pass1_1018_5b06(paVar1, CONCAT22(struct_a_1.win_handle_0x8, struct_b_2));
        struct_a_1.char_ptr_16_0xee = local_DXAX_61;
        struct_a_1.field_0xf0 = (local_DXAX_61 >> 0x10);
        return;
    }
    &struct_a_1.char_ptr_16_0xee = 0;
    return;
}

pub fn pass1_1018_59f0(in_struct_594: *mut astruct_594)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_4: *mut astruct_594;
    let mut uVar4: i32;
    let mut in_stack_0000fff6: u16;
    let temp_8628572018a: *mut astruct_593;

    uVar4 = (in_struct_594 >> 0x10);
    local_BX_4 = in_struct_594;
    puVar1 = local_BX_4.u8_ptr_16_xee;
    uVar2 = local_BX_4.field_0xf0;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;}
        (**ppcVar3)();
    }
    &local_BX_4.u8_ptr_16_xee = 0;
    destroy_win_1008_628e((in_struct_594 & 0xffff | uVar4 << 0x10), in_stack_0000fff6);
    return;
}

pub fn pass1_1018_5a3c(param_1: *mut astruct_591, param_2: u8) -> *mut astruct_591

{
    pass1_1018_58b6(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_5b06(param_1: *mut astruct_595,param_2: u32)

{
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: i32;
    let struct_a: *mut astruct_199;
    let paVar6: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut unaff_SI: u16;
    let mut unaff_SS: u16;
    let ppVar7: *mut pass1_struct_1;
    let local_struct_104_ptr: *mut astruct_104;
    let mut uVar8: u32;
    let mut uVar9: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: [u8;6];
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_7ff4519c874: i32;

    uVar9 = param_2;
    get_dc_1020_921c(CONCAT22(uVar9, param_1), (param_2 >> 0x10));
    &param_1.field_0x14 = 0;
    &param_1.field_0x18 = 0;
    zero_list_1008_3e38(CONCAT22(uVar9, &param_1.field_0x1c));
    CONCAT22(uVar9, param_1) = &PTR_LOOP_1050_5e1a;
    param_1.field_0x2 = 0x1018;
    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x48));
    zero_list_1008_3e38(CONCAT22(unaff_SS, local_c));
    modify_list_1008_3f62(CONCAT22(unaff_SS, local_c),
                          _local_6 & 0xffff0000 | (_local_6 + 0xe));
    ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x27));
    param_1.field_0x14 = ppVar7;
    param_1.field_0x16 = (ppVar7 >> 0x10);
    ppcVar2 = (&param_1.field_0x14 + 4);
    ppcVar2();
    param_1.field_0x6 = &param_1.field_0x14;
    uVar3 = &param_1.field_0x14;
    puVar1 = (uVar3 + 10);
    iVar4 = &param_1.field_0xa;
    unsafe{ppcVar2 = (*puVar1 + 8);}
    ppcVar2();
    param_1.field_0x12 = iVar4;
    draw_1020_9364(CONCAT22(uVar9, param_1));
    uVar8 = &param_1.field_0x14;
    modify_list_1008_3f62(CONCAT22(uVar9, &param_1.field_0x1c),
                          uVar8 & 0xffff0000 | (uVar8 + 0x52));
    pass1_1008_3f32(&param_1.field_0x1c, uVar9, local_c, unaff_SS);
    local_struct_104_ptr = pass1_1008_9f48(&param_1.field_0x14);
    uVar8 = process_struct_1008_4772(local_struct_104_ptr);
    struct_a = (uVar8 >> 0x10);
    temp_7ff4519c874 = uVar8;
    paVar6 = struct_a;
    uVar5 = temp_7ff4519c874;
    process_struct_1000_179c(0x14, struct_a);
    if ((paVar6 | uVar5) == 0)
    {
        &param_1.field_0x18 = 0;
    }
    else
    {
        process_struct_1008_50c2(CONCAT22(paVar6, uVar5), (temp_7ff4519c874 + 8),
                                 (temp_7ff4519c874 + 4), CONCAT22(uVar9, &param_1.field_0x1c), puVar1);
        param_1.field_0x18 = uVar5;
        &param_1.field_0x1a = extraout_DX;
    }
    pass1_1008_5134(&param_1.field_0x18);
    param_1.field_0x22 = param_1.field_0x1c;
    param_1.field_0x24 = param_1.field_0x1e;
    param_1.field_0x26 = (temp_7ff4519c874 + 4) + param_1.field_0x22 + 1;
    param_1.field_0x28 = (temp_7ff4519c874 + 8) + param_1.field_0x24 + 1;
    return;
}

pub fn pass1_1018_5cc8(param_1: *mut *mut astruct_598)

{
    let mut uVar1: i32;
    let in_struct_1: *mut astruct_44;
    let struct_598_ptr_1: *mut astruct_598;
    let mut uVar2: i32;
    let mut local_6: u32;
    let struct_376_ptr_1: *mut astruct_376;

    uVar2 = (param_1 >> 0x10);
    struct_598_ptr_1 = param_1;
    unsafe{*param_1 = &PTR_LOOP_1050_5e1a;}
    struct_598_ptr_1.field_0x2 = 0x1018;
    in_struct_1 = &struct_598_ptr_1.astruct_376_ptr_x18;
    uVar1 = &struct_598_ptr_1.field_0x1a;
    if ((uVar1 | in_struct_1) != 0)
    {
        pass1_1008_5118((in_struct_1 & 0xffff | uVar1 << 0x10));
        error_check_1000_17ce(in_struct_1);
    }
    if (struct_598_ptr_1.field_0x14 != 0)
    {
        pass1_1010_1ea6(struct_598_ptr_1.field_0x14,
                        (param_1 & 0xffff | uVar2 << 0x10));
        pass1_1010_1dda(struct_598_ptr_1.field_0x14);
    }
    select_and_delete_palette_1020_92c4(param_1);
    return;
}


pub fn pass1_1018_5df4(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1018_5cc8(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_5e26(param_1: *mut u16, param_2: u16)

{
    let mut uVar1: u16;

    process_struct_1040_7728(param_1, (&PTR_LOOP_1050_0000 + 1), 0, 0xfd0, param_2);
    uVar1 = (param_1 >> 0x10);
    unsafe{*param_1 = 0x6128;
    (param_1 + 2) = 0x1018;}
    (param_1 + 0x74) = 1;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_5e5a(in_astruct_599_ptr_1: *mut astruct_599)

{
    let local_BX_3: *mut astruct_599;
    let mut uVar1: u16;

    uVar1 = (in_astruct_599_ptr_1 >> 0x10);
    local_BX_3 = in_astruct_599_ptr_1;
    in_astruct_599_ptr_1.field_0x0 = 0x6128;
    local_BX_3.field_0x2 = 0x1018;
    pass1_1038_b6e0(_g_astruct_112_a, local_BX_3.field_0x6);
    win_cleanup_func_1040_782c(in_astruct_599_ptr_1);
    return;
}

pub fn pass1_1018_5e86(param_1: *mut u32)

{
    let fn_ptr_1: fn();

    unsafe{fn_ptr_1 = (*param_1 + 0x6c);
    (**fn_ptr_1)();}
    return;
}


pub fn pass1_1018_5ffa(param_1: *mut astruct_600)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_4: *mut astruct_600;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    puVar1 = local_BX_4.field_0x92;
    uVar2 = &local_BX_4.field_0x94;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;
        (**ppcVar3)();}
    }
    &local_BX_4.field_0x92 = 0;
    pass1_1010_1dda(local_BX_4.field_0x8e);
    local_BX_4.field_0x8e = 0;
    return;
}

pub fn pass1_1018_6048(param_1: u32) -> bool

{
    let fn_ptr_1: fn();

    fn_ptr_1 = ((param_1 + 0x92) + 8);
    (**fn_ptr_1)();
    return 0;
}


pub fn pass1_1018_6102(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1018_5e5a(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_6198(in_astruct_601_ptr: *mut astruct_601,param_2: u32, param_3: u16)

{
    let local_struct_601_ptr: *mut astruct_601;
    let mut uVar1: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000ffec: u32;

    uVar1 = (in_astruct_601_ptr >> 0x10);
    local_struct_601_ptr = in_astruct_601_ptr;
    in_astruct_601_ptr = s_1_1050_389a;
    local_struct_601_ptr.field_0x2 = &PTR_LOOP_1050_1008;
    in_astruct_601_ptr = (s_18_2_1050_3aa5 + 3);
    local_struct_601_ptr.field_0x2 = &PTR_LOOP_1050_1008;
    local_struct_601_ptr.field_0x4 = param_3;
    in_astruct_601_ptr = s_0_020_1050_3ab0;
    local_struct_601_ptr.field_0x2 = &PTR_LOOP_1050_1008;
    &local_struct_601_ptr.field_0x6 = 0;
    local_struct_601_ptr.field_0xa = param_2;
    in_astruct_601_ptr = 0x66c0;
    local_struct_601_ptr.field_0x2 = 0x1018;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0,
                                      CONCAT22((in_stack_0000ffec >> 0x10), 0x39));
    local_struct_601_ptr.field_0x6 = ppVar2;
    local_struct_601_ptr.field_0x8 = (ppVar2 >> 0x10);
    return;
}

pub fn pass1_1018_620c(param_1: *mut astruct_376)

{
    let local_BX_4: *mut astruct_602;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.ptr_a_lo = 0x66c0;
    local_BX_4.u16_x01 = 0x1018;
    param_1.ptr_a_lo = s_0_020_1050_3ab0;
    local_BX_4.u16_x01 = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    local_BX_4.u16_x01 = &PTR_LOOP_1050_1008;
    return;
}



pub fn pass1_1018_642e(param_1: u16, param_2: u16, param_1_00: *mut i32, param_2_00: i32)

{
    unsafe{*param_1_00 = 100 - param_2_00 >> 1;}
    return;
}

// WARNING: Variable defined which should be unmapped: local_e

pub fn draw_1018_6444(param_1: u32, HDC16: param_2)

{
    let mut iVar1: i32;
    let piVar2: *mut u16;
    let mut uVar3: u32;
    let mut x: i32;
    let mut iVar4: i32;
    let piVar5: *mut i32;
    let mut uVar6: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    uVar6 = (param_1 >> 0x10);
    uVar3 = (param_1 + 6);
    iVar1 = (uVar3 + 0x30);
    uVar3 = (param_1 + 6);
    piVar2 = (uVar3 + 0x1a);
    unsafe{
    MoveTo16(5, *piVar2, param_2);
    uVar6 = (piVar2 >> 0x10);
    iVar4 = piVar2;
    LineTo16(5, (iVar4 + iVar1 * 8 + -4), param_2);
    local_a = 0;
    while (local_a < iVar1)
    {
        piVar5 = (local_a * 8 + iVar4);
        x = (piVar5[2] - *piVar5 >> 1) + *piVar5;
        MoveTo16(5, x, param_2);
        LineTo16(10, x, param_2);
        local_a = local_a + 1;
    }
    MoveTo16(0x5f, *piVar2, param_2);
    LineTo16(0x5f, (iVar4 + iVar1 * 8 + -4), param_2);
    local_a = 0;
    while (local_a < iVar1)
    {
        piVar5 = (local_a * 8 + iVar4);
        MoveTo16(0x5f, (piVar5[2] - *piVar5 >> 1) + *piVar5, param_2);
        LineTo16(0x5a, param_2, param_2);
        local_a = local_a + 1;
    }
}
    return;
}

pub fn pass1_1018_6544(param_1: u32,param_2: u32, param_3: u16)

{
    let mut unaff_SS: u16;
    let mut u_var1: u32;
    let mut uVar2: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8;6];
    let mut local_4: u16;

    if (param_2 != 0)
    {
        local_4 = ((param_2 + 4) - param_2 >> 1) + param_2;
        pass1_1008_3e54(CONCAT22(unaff_SS, local_a), 0, 0x57, local_4);
        uVar2 = (param_1 >> 0x10);
        uVar1 = pass1_1018_659a(param_1, uVar2, CONCAT22(unaff_SS, local_a));
        polygon_1018_661c(param_1, uVar2, 3, uVar1);
    }
    return;
}

pub fn pass1_1018_659a(param_1: u16, param_2: u16,param_1_00: u32)

{
    let mut uVar1: u16;
    let local_BX_65: *mut astruct_603;
    let mut unaff_SS: u16;
    let mut uVar2: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = pass1_1008_3e94(param_1_00, CONCAT22(unaff_SS, &local_6),
                            CONCAT22(unaff_SS, &local_4));
    process_struct_1000_179c(0xc, (uVar2 >> 0x10));
    uVar1 = (uVar2 >> 0x10);
    local_12 = 0;
    while (local_12 < 3)
    {
        local_BX_65 = (local_12 * 4);
        (local_BX_65 + uVar2) = (local_BX_65 + 0x4248) + local_4;
        (local_BX_65 + uVar2 + 2) = (local_BX_65 + 0x424a) + local_6;
        local_12 = local_12 + 1;
    }
    return uVar2;
}


pub fn pass1_1018_669a(in_struct_376_ptr: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    pass1_1018_620c(in_struct_376_ptr);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(in_struct_376_ptr);
    }
    return in_struct_376_ptr;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_66cc(in_struct_65_1: u32, param_2: u16,param_3: u32)

{
    let mut local_AX_24: u16;
    let mut local_DX_71: u16;
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut local_ES_21: u16;
    let ppVar2: *mut pass1_struct_1;

    load_cursor_1020_7f7a(in_struct_65_1, CONCAT22(param_2, 10), param_3);
    local_ES_21 = (in_struct_65_1 >> 0x10);
    iVar1 = in_struct_65_1;
    (iVar1 + 0xee) = 0;
    (iVar1 + 0xf2) = 0;
    in_struct_65_1 = 0x6880;
    (iVar1 + 2) = 0x1018;
    (iVar1 + 0xe2) = 0x691c;
    (iVar1 + 0xe4) = 0x1018;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, 0xb));
    local_DX_71 = (ppVar2 >> 0x10);
    (iVar1 + 0xf2) = ppVar2;
    (iVar1 + 0xf4) = local_DX_71;
    (iVar1 + 0xe6) = (iVar1 + 0xf2);
    (iVar1 + 0xe8) = local_DX_71;
    return;
}

pub fn pass1_1018_673c(in_struct_591_ptr: *mut astruct_376)

{
    let uVar1: u8;
    let local_struct_591_ptr: *mut astruct_591;
    let mut uVar2: u16;

    uVar2 = (in_struct_591_ptr >> 0x10);
    local_struct_591_ptr = in_struct_591_ptr;
    in_struct_591_ptr.ptr_a_lo = 0x6880;
    local_struct_591_ptr.u16_x02 = 0x1018;
    local_struct_591_ptr.u16_xe2 = 0x691c;
    local_struct_591_ptr.u16_xe4 = 0x1018;
    uVar1 = process_struct_1020_808e(in_struct_591_ptr);
    return uVar1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_6768(in_struct_608_ptr: *mut astruct_608) -> i32

{
    let mut uVar1: i32;
    let local_struct_608_ptr: *mut astruct_608;
    let mut local_struct_608_ptr_hi: u16;
    let puVar2: *mut u8;
    let fn_ptr_1: fn();

    local_struct_608_ptr_hi = (in_struct_608_ptr >> 0x10);
    local_struct_608_ptr = in_struct_608_ptr;
    uVar1 = local_struct_608_ptr.field_0xf0 | &local_struct_608_ptr.field_0xee;
    if (uVar1 != 0)
    {
        fn_ptr_1 = (&local_struct_608_ptr.field_0xee + 8);
        uVar1 = (**fn_ptr_1)();
    }
    if (local_struct_608_ptr.field_0xea == 0)
    {
        local_struct_608_ptr.field_0xea = 1;
        puVar2 = pass1_1038_af40(_g_astruct_112_a, local_struct_608_ptr.field_0x8, 0x16);
        uVar1 = puVar2;
    }
    return uVar1;
}

pub fn pass1_1018_67b6(in_win_struct_42: *mut win_struct_42)

{
    let paVar1: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let local_win_struct_42: *mut win_struct_42;
    let local_win_struct_42_hi: *mut win_struct_42;
    let paVar2: *mut astruct_199;
    let mut uVar3: u32;
    let mut local_4: u16;

    paVar2 = create_win_1008_9760(in_win_struct_42);
    struct_a = (paVar2 >> 0x10);
    local_win_struct_42_hi = (in_win_struct_42 >> 0x10);
    local_win_struct_42 = in_win_struct_42;
    paVar1 = get_gui_dc_1018_4db0(*&local_win_struct_42.u32_xf2, local_win_struct_42.win_handle_0x8);
    process_struct_1000_179c(0x18, struct_a);
    if ((struct_a | paVar1) != 0)
    {
        uVar3 = pass1_1018_6924(paVar1, struct_a);
        local_win_struct_42.char_ptr_16_0xee = uVar3;
        local_win_struct_42.field_0xf0 = (uVar3 >> 0x10);
        return;
    }
    &local_win_struct_42.char_ptr_16_0xee = 0;
    return;
}

pub fn pass1_1018_681a(param_1: *mut astruct_594)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let mut uvar3: u16;
    let mut in_stack_0000fff6: u16;
    let fn_ptr_1: fn();

    uVar3 = (param_1 >> 0x10);
    puVar1 = (param_1 + 0xee);
    uVar2 = (param_1 + 0xf0);
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{
        fn_ptr_1 = *puVar1;
        (**fn_ptr_1)();}
    }
    destroy_win_1008_628e(param_1, in_stack_0000fff6);
    return;
}

pub fn pass1_1018_685a(in_struct_376_ptr: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    pass1_1018_673c(in_struct_376_ptr);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(in_struct_376_ptr);
    }
    return in_struct_376_ptr;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_6924(param_1: u16, HANDLE16: param_2)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_00000008: u16;
    let mut in_stack_0000fff2: u16;
    let mut local_6: u32;

    get_dc_1020_921c(CONCAT22(param_2, param_1), in_stack_00000008);
    (param_1 + 0x14) = 0;
    CONCAT22(param_2, param_1) = 0x6a02;
    (param_1 + 2) = 0x1018;
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff2, 0xb));
    uVar4 = (ppVar5 >> 0x10);
    (param_1 + 0x14) = ppVar5;
    (param_1 + 0x16) = uVar4;
    (param_1 + 6) = (param_1 + 0x14);
    (param_1 + 8) = uVar4;
    uVar2 = (param_1 + 0x14);
    iVar3 = param_1 + 10;
    ppcVar1 = ((uVar2 + 10) + 8);
    (**ppcVar1)();
    (param_1 + 0x12) = iVar3;
    draw_1020_9364(CONCAT22(param_2, param_1));
    return;
}



pub fn pass1_1018_6c1e(in_struct_a: *mut astruct_215, param_2: u8)

{
    let struct_a_1: *mut astruct_215;
    let struct_a_2: *mut astruct_215;

    struct_a_1 = in_struct_a;
    struct_a_1 = &struct_a_1.field_0xd2;
    pass1_1008_57c4((in_struct_a & 0xffff0000 | ZEXT24(struct_a_1)));
    struct_a_2 = (in_struct_a >> 0x10);
    in_struct_a.ptr_a_lo = 0x380a;
    struct_a_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    in_struct_a.ptr_a_lo = s_1_1050_389a;
    struct_a_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(in_struct_a);
    }
    return;
}


pub fn pass1_1018_7da6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_7dee(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_7e36(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_7e7e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_7ec6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_7f0e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_7f56(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_7f9e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_7fe6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_802e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8076(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_80be(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8106(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_814e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8196(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_81de(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8226(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_826e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_82b6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_82fe(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8346(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_838e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_83d6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_841e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8466(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_84ae(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_84f6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_853e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8586(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_85ce(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8616(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_865e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_86a6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_86ee(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8736(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_877e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_87c6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_880e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8856(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_889e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_88e6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_892e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8976(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_89be(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8a06(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8a4e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8a96(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8ade(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8b26(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8b6e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8bb6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8bfe(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8c46(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8c8e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8cd6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8d1e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8d66(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8dae(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8df6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8e3e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8e86(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8ece(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8f16(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8f5e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8fa6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_8fee(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_9036(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_907e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_90c6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_910e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_9156(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_919e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_91e6(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_922e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_9276(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_92be(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_9306(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_934e(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1018_9396(param_1: *mut astruct_376, param_2: u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x380a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (iVar1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return;
}

// WARNING: Variable defined which should be unmapped: string_a
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn 
pass1_1018_c402(in_struct_a: *mut astruct_65, param_2: u16, in_struct_b: u16, param_4: u16, param_5: *mut u8, param_6: *mut u8, param_7: *mut u8,param_8: u32)
{
    let uVar1: u8;
    let struct_b: *mut astruct_613;
    let extraout_var: u32;
    let mut local_DX_27: u16;
    let mut uVar2: u16;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let struct_a_2: *mut astruct_65;
    let struct_a_1: *mut astruct_65;
    let struct_result: *mut pass1_struct_1;
    let string_a: *mut libc::c_char;
    let mut local_6: u16;
    let mut local_4: u16;
    let struct_c: *mut pass1_struct_1;
    let extraout_var_00: u32;

    win_gui_fn_1020_0762(in_struct_a, CONCAT22(param_5, param_4),
                         CONCAT22(param_6, (param_5 >> 0x10)), (param_6 >> 0x10),
                         param_7, param_8);
    struct_a_1 = (in_struct_a >> 0x10);
    struct_a_2 = in_struct_a;
    struct_a_2.field_0xf6 = 0x0;
    struct_a_2.field_0xf8 = 0x0;
    struct_a_2.field_0xfa = 0x0;
    struct_a_2.field_0xfc = 0x0;
    struct_a_2.field_0xfe = 2;
    &struct_a_2.field_0x108 = 0;
    struct_a_2.field_0x10c = 0;
    struct_a_2.field_0x10e = 0x1e0190;
    struct_a_2.field_0x112 = 0;
    in_struct_a.ptr_a_lo = 0xc8bc;
    struct_a_2.ptr_a_hi = 0x1018;
    uVar2 = local_DX_27;
    uVar1 = pass1_1000_4906((in_struct_a & 0xffff0000 | &struct_a_2.field_0x100),
                            0, 8);
    struct_b = CONCAT31(extraout_var, uVar1);
    if ((in_struct_b == 0) || (param_2 != 0))
    {
        if ((param_2 & in_struct_b) == 0) {}
          // goto LAB_1018_c4bb;
        struct_b._0_1_ = string_fn_1008_5fd8(in_struct_b, param_2);
        struct_b = CONCAT31(extraout_var_00, struct_b);
    }
    else
    {
        load_str_1010_84ac(_g_struct_73_1050_14cc,
                           (_g_struct_73_1050_14cc >> 0x10), in_struct_b);
        uVar2 = extraout_DX;
    }
    struct_a_2.field_0x108 = struct_b;
    struct_a_2.field_0x10a = uVar2;
// LAB_1018_c4bb:
    struct_result = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(string_a, 0x48));
    uVar3 = (struct_result >> 0x10);
    struct_c = struct_result;
    struct_a_2.field_0xf6 = struct_c.field_0xa;
    struct_a_2.field_0xf8 = struct_c.field_0xc;
    pass1_1008_3e94(&struct_c.field_0xe,
                    (in_struct_a & 0xffff0000 | ZEXT24(&struct_a_2.field_0xfc)),
                    (in_struct_a & 0xffff0000 | ZEXT24(&struct_a_2.field_0xfa)));
    return;
}


pub fn pass1_1018_c958(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let mut local_SS__1: u16;
    let puVar1: *mut u16;
    let mut local_10: u16;
    let mut local_6: [u8;4];

    puVar1 = pass1_1008_941a(CONCAT22(local_SS__1, local_6), 1, 0x8d);
    pass1_1018_c402(param_1, 0, 0x732, 0x26, CONCAT22(puVar1, 8000),
                    CONCAT22(0xf1, (puVar1 >> 0x10)), CONCAT22(param_2, 0x9a),
                    param_3);
    param_1.u16_x00 = 0xdc5a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_c9a6(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let mut local_SS__1: u16;
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xf2;
    uVar3 = 0xa0;
    puVar1 = pass1_1008_941a(CONCAT22(local_SS__1, local_6), 1, 0x8e);
    pass1_1018_c402(param_1, 0, 0x733, 0x27, CONCAT22(puVar1, 7000),
                    CONCAT22(uVar2, (puVar1 >> 0x10)), CONCAT22(param_2, uVar3),
                    param_3);
    param_1.u16_x00 = 0xd6de;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_c9f4(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let piVar1: *mut i32;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let puVar3: *mut u16;
    let mut uVar4: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xf3;
    uVar4 = 0xa6;
    puVar3 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x8f);
    pass1_1018_c402(param_1, 0, 0x734, 0x28, CONCAT22(puVar3, 13000),
                    CONCAT22(uVar2, (puVar3 >> 0x10)), CONCAT22(param_2, uVar4),
                    param_3);
    uVar2 = (param_1 >> 0x10);
    param_1.u16_x00 = 0xda86;
    (param_1 + 2) = 0x1018;
    piVar1 = (param_1 + 0x10e);
    unsafe{*piVar1 = *piVar1 + -0x19;}
    return param_1;
}

pub fn pass1_1018_ca48(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let mut unaff_SS: u16;
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xf4;
    uVar3 = 0xa1;
    puVar1 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x90);
    pass1_1018_c402(param_1, 0, 0x735, 0x29, CONCAT22(puVar1, 14000),
                    CONCAT22(uVar2, (puVar1 >> 0x10)), CONCAT22(param_2, uVar3),
                    param_3);
    param_1.u16_x00 = 0xd50a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_ca96(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let piVar1: *mut i32;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let puVar3: *mut u16;
    let mut uVar4: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xf5;
    uVar4 = 0xbf;
    puVar3 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x92);
    pass1_1018_c402(param_1, 0x737, 0x736, 0x2a, CONCAT22(puVar3, 26000),
                    CONCAT22(uVar2, (puVar3 >> 0x10)), CONCAT22(param_2, uVar4),
                    param_3);
    uVar2 = (param_1 >> 0x10);
    param_1.u16_x00 = 0xd8b2;
    (param_1 + 2) = 0x1018;
    piVar1 = (param_1 + 0x10e);
    unsafe{*piVar1 = *piVar1 + 100;}
    return param_1;
}

pub fn pass1_1018_caea(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let mut unaff_SS: u16;
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xf6;
    uVar3 = 0x93;
    puVar1 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x93);
    pass1_1018_c402(param_1, 0, 0x738, 0x2b, CONCAT22(puVar1, 9000),
                    CONCAT22(uVar2, (puVar1 >> 0x10)), CONCAT22(param_2, uVar3),
                    param_3);
    param_1.u16_x00 = 0xdbbe;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cb38(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let mut unaff_SS: u16;
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xf7;
    uVar3 = 0x94;
    puVar1 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x94);
    pass1_1018_c402(param_1, 0, 0x739, 0x2c, CONCAT22(puVar1, 13000),
                    CONCAT22(uVar2, (puVar1 >> 0x10)), CONCAT22(param_2, uVar3),
                    param_3);
    param_1.u16_x00 = 0xd642;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cb86(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let piVar1: *mut i32;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let puVar3: *mut u16;
    let mut uVar4: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xf8;
    uVar4 = 0xc2;
    puVar3 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x96);
    pass1_1018_c402(param_1, 0, 0x73a, 0x2d, CONCAT22(puVar3, 9000),
                    CONCAT22(uVar2, (puVar3 >> 0x10)), CONCAT22(param_2, uVar4),
                    param_3);
    uVar2 = (param_1 >> 0x10);
    param_1.u16_x00 = 0xd9ea;
    (param_1 + 2) = 0x1018;
    piVar1 = (param_1 + 0x10e);
    unsafe{*piVar1 = *piVar1 + 100;}
    return param_1;
}

// WARNING: Variable defined which should be unmapped: local_10

pub fn pass1_1018_cbda(param_1: *mut astruct_65, param_2: u16,param_3: u32) -> *mut astruct_65

{
    let string_base_a: *mut libc::c_char;
    let puVar1: *mut u16;
    let local_10: *mut u8;
    let uVar3: *mut u8;
    let mut string_offset_a: [u8;4];

    puVar1 = pass1_1008_941a(CONCAT22(string_base_a, string_offset_a), 1, 0x97);
    pass1_1018_c402(param_1, 0, 0x73b, 0x2e, CONCAT22(puVar1, 11000),
                    CONCAT22(0xf9, (puVar1 >> 0x10)), CONCAT22(param_2, 0xc5),
                    param_3);
    param_1.ptr_a_lo = 0xd46e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cc28(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let mut unaff_SS: u16;
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xfa;
    uVar3 = 0xa3;
    puVar1 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x98);
    pass1_1018_c402(param_1, 0, 0x73c, 0x2f, CONCAT22(puVar1, 10000),
                    CONCAT22(uVar2, (puVar1 >> 0x10)), CONCAT22(param_2, uVar3),
                    param_3);
    param_1.u16_x00 = 0xd816;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cc76(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let mut unaff_SS: u16;
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xfb;
    uVar3 = 0xa8;
    puVar1 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x99);
    pass1_1018_c402(param_1, 0x73e, 0x73d, 0x30, CONCAT22(puVar1, 25000),
                    CONCAT22(uVar2, (puVar1 >> 0x10)), CONCAT22(param_2, uVar3),
                    param_3);
    param_1.u16_x00 = 0xdb22;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_ccc4(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let mut unaff_SS: u16;
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xfc;
    uVar3 = 0xa9;
    puVar1 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x9b);
    pass1_1018_c402(param_1, 0x740, 0x73f, 0x31, CONCAT22(puVar1, 23000),
                    CONCAT22(uVar2, (puVar1 >> 0x10)), CONCAT22(param_2, uVar3),
                    param_3);
    param_1.u16_x00 = 0xd5a6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cd12(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let mut unaff_SS: u16;
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xfd;
    uVar3 = 0x7c;
    puVar1 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x9c);
    pass1_1018_c402(param_1, 0, 0x741, 0x32, CONCAT22(puVar1, 10000),
                    CONCAT22(uVar2, (puVar1 >> 0x10)), CONCAT22(param_2, uVar3),
                    param_3);
    param_1.u16_x00 = 0xd94e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cd60(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let mut unaff_SS: u16;
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xfe;
    uVar3 = 0xc9;
    puVar1 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0);
    pass1_1018_c402(param_1, 0, 0, 0x33, CONCAT22(puVar1, 10000),
                    CONCAT22(uVar2, (puVar1 >> 0x10)), CONCAT22(param_2, uVar3),
                    param_3);
    param_1.u16_x00 = 0xd3d2;
    (param_1 + 2) = 0x1018;
    return param_1;
}



pub fn pass1_1018_cf74(param_1: *mut astruct_612, param_2: u16,param_3: u32) -> *mut astruct_612

{
    let mut unaff_SS: u16;
    let puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut local_6: [u8;4];

    uVar2 = 0xfe;
    uVar3 = 0xcf;
    puVar1 = pass1_1008_941a(CONCAT22(unaff_SS, local_6), 1, 0x80);
    pass1_1018_c402(param_1, 0, 0, 0x34, CONCAT22(puVar1, 10000),
                    CONCAT22(uVar2, (puVar1 >> 0x10)), CONCAT22(param_2, uVar3),
                    param_3);
    param_1.u16_x00 = 0xd77a;
    (param_1 + 2) = 0x1018;
    return param_1;
}



pub fn pass1_1018_d198(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d1be(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d1e4(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d20a(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d230(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d256(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d27c(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d2a2(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d2c8(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d2ee(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d314(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d33a(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d360(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d386(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1018_d3ac(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1018_dcf6(param_1: *mut u16)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    unsafe{*param_1 = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    *param_1 = 0xdf3c;
    (param_1 + 2) = 0x1018;}
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_dd1e(param_1: u16, param_2: u16, param_1: u16_00,param_2_00: u32)

{
    let paVar1: *mut astruct_204;
    let paVar2: *mut astruct_204;
    let mut uVar3: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar1 = 
        pass1_1010_81f6(_g_struct_73_1050_14cc, (_g_struct_73_1050_14cc >> 0x10), 0, 0, param_2_00._2_2_);
    paVar2 = paVar1;
    process_struct_1000_179c(0x46, (paVar1 >> 0x10));
    if (paVar2 == 0x0)
    {
        uVar3 = 0;
    }
    else
    {
        uVar3 = pass1_1008_87cc(paVar2, param_1_00, param_2_00, param_2_00._2_2_, paVar1, 0);
    }
    pass1_1008_8bc6(uVar3);
    return uVar3;
}

// WARNING: Variable defined which should be unmapped: local_30

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_dd7c(param_1: u16, param_2: u16,param_1_00: u32, in_astruct_617_1: *mut astruct_617) -> i32

{
    let mut uVar1: i32;
    let local_AL_380: u8;
    let mut uVar2: i32;
    let local_astruct_617_ptr_1: *mut astruct_617;
    let mut uVar3: i32;
    let mut uVar4: u16;
    let mut local_DX_35: u16;
    let struct_a: *mut astruct_199;
    let mut local_DX_195: u16;
    let struct_a_00: *mut astruct_199;
    let paVar5: *mut astruct_199;
    let mut local_DX_371: u16;
    let ppVar6: *mut pass1_struct_1;
    let local_DXAX_208: *mut i32;
    let mut uVar7: u16;
    let mut local_30: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_20: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u32;
    let local_astruct_616_ptr_1: *mut astruct_616;
    let mut local_a: u16;
    let mut local_6: u32;
    let local_astruct_615_ptr_1: *mut astruct_615;
    let mut temp_5f29fbf093: u32;
    let fn_ptr_1: fn();
    let mut local_astruct_617_ptr_1_hi: u16;

    local_astruct_617_ptr_1_hi = (param_1_00 >> 0x10);
    if (in_astruct_617_1._3_1_ == '\0')
    {
        ppVar6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_30, 0x2f));
        if ((ppVar6 + 0x1e) == 0)
        {
            local_14 = in_astruct_617_1;
            uVar7 = in_astruct_617_1;
        }
        else
        {
            if (in_astruct_617_1 - 7 == 0)
            {
                local_14 = 6;
                in_astruct_617_1._0_2_ = in_astruct_617_1 - 7;
            }
            else
            {
                if (in_astruct_617_1 - 8 == 0)
                {
                    local_14 = 7;
                    in_astruct_617_1._0_2_ = in_astruct_617_1 - 8;
                }
                else
                {
                    local_14 = 8;
                    in_astruct_617_1._0_2_ = in_astruct_617_1 - 9;
                }
            }
            uVar7 = 6;
        }
        pass1_1010_81f6(_g_struct_73_1050_14cc, (_g_struct_73_1050_14cc >> 0x10), 0, 0,
                        uVar7);
        uVar2 = struct_a | in_astruct_617_1;
        if ((uVar2 != 0) &&
            (paVar5 = struct_a, process_struct_1000_179c(0x46, struct_a), (paVar5 | uVar2) != 0))
        {
            pass1_1008_87cc(CONCAT22(paVar5, uVar2), param_1_00,
                            local_astruct_617_ptr_1_hi, local_14, CONCAT22(struct_a, in_astruct_617_1),
                            in_astruct_617_1);
        }
    }
    else
    {
        local_astruct_617_ptr_1 =
            
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, in_astruct_617_1,
                                (in_astruct_617_1 >> 0x10));
        local_DXAX_208 = pass1_1030_73a8(CONCAT22(local_DX_195, local_astruct_617_ptr_1));
        local_DXAX_208._2_2_ = (local_DXAX_208 >> 0x10);
        local_astruct_615_ptr_1 = local_DXAX_208;
        if ((local_DXAX_208._2_2_ | local_astruct_615_ptr_1) != 0)
        {
            temp_5f29fbf093 = &local_astruct_617_ptr_1.field_0x2e;
            local_astruct_616_ptr_1 = temp_5f29fbf093;
            if ((local_astruct_617_ptr_1.field_0x30 | local_astruct_616_ptr_1) == 0)
            {
                local_20 = 0;
            }
            else
            {
                local_20 = local_astruct_616_ptr_1.field_0x200;
            }
            uVar2 = local_astruct_615_ptr_1.field_0x1c;
            uVar1 = local_astruct_615_ptr_1.field_0x1e;
            uVar3 = uVar1 | uVar2;
            if (uVar3 != 0)
            {
                local_20 = CONCAT22(uVar1, uVar2);
                uVar3 = uVar2;
            }
            unsafe{
            fn_ptr_1 = (*local_DXAX_208 + 0x14);}
            (**fn_ptr_1)(0x1030, local_astruct_615_ptr_1, local_DXAX_208._2_2_);
            uVar4 = uVar3;
            pass1_1010_81f6(_g_struct_73_1050_14cc, (_g_struct_73_1050_14cc >> 0x10),
                            local_20, (local_20 >> 0x10), uVar3);
            paVar5 = struct_a_00;
            uVar7 = uVar4;
            process_struct_1000_179c(0x46, struct_a_00);
            if ((paVar5 | uVar7) == 0)
            {
                local_AL_380 = '\0';
                local_astruct_617_ptr_1_hi = 0;
            }
            else
            {
                pass1_1008_87cc(CONCAT22(paVar5, uVar7), param_1_00,
                                local_astruct_617_ptr_1_hi, uVar3, CONCAT22(struct_a_00, uVar4),
                                in_astruct_617_1);
                local_AL_380 = uVar7;
                local_astruct_617_ptr_1_hi = local_DX_371;
            }
            pass1_1008_8bc6(local_AL_380, local_astruct_617_ptr_1_hi);
        }
    }
    return;
}

pub fn pass1_1018_df10(param_1: *mut astruct_376, param_2: u8)

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1018_e01c(in_struct_a: *mut astruct_44, param_2: u8)

{
    let struct_a: *mut astruct_44;
    let struct_a_1: *mut astruct_44;

    struct_a = in_struct_a;
    struct_a = &struct_a[1].field_0x1a;
    pass1_1008_57c4((in_struct_a & 0xffff0000 | ZEXT24(struct_a)));
    struct_a_1 = (in_struct_a >> 0x10);
    in_struct_a.ptr_a_lo = 0x380a;
    struct_a.ptr_a_hi = &PTR_LOOP_1050_1008;
    in_struct_a.ptr_a_lo = s_1_1050_389a;
    struct_a.ptr_a_hi = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(in_struct_a);
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_14
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_e100(in_astruct_619_ptr_1: *mut astruct_619, param_2: u16) -> *mut astruct_619

{
    let local_astruct_619_ptr_1: *mut astruct_619;
    let mut local_ES_4: u16;
    let mut local_DXAX_83: u32;
    let mut local_14: u32;

    local_ES_4 = (in_astruct_619_ptr_1 >> 0x10);
    local_astruct_619_ptr_1 = in_astruct_619_ptr_1;
    in_astruct_619_ptr_1.u16_x00 = s_1_1050_389a;
    local_astruct_619_ptr_1.u16_x02 = &PTR_LOOP_1050_1008;
    in_astruct_619_ptr_1.u16_x00 = (s_18_2_1050_3aa5 + 3);
    local_astruct_619_ptr_1.u16_x02 = &PTR_LOOP_1050_1008;
    local_astruct_619_ptr_1.u16_x04 = param_2;
    in_astruct_619_ptr_1.u16_x00 = s_0_020_1050_3ab0;
    local_astruct_619_ptr_1.u16_x02 = &PTR_LOOP_1050_1008;
    &local_astruct_619_ptr_1.u16_x06 = 0;
    in_astruct_619_ptr_1.u16_x00 = 0xe228;
    local_astruct_619_ptr_1.u16_x02 = 0x1018;
    local_14 = CONCAT22(local_14._2_2_, 0x36);
    local_DXAX_83 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, local_14);
    local_astruct_619_ptr_1.u16_x06 = local_DXAX_83;
    local_astruct_619_ptr_1.u16_x08 = (local_DXAX_83 >> 0x10);
    return in_astruct_619_ptr_1;
}



pub fn pass1_1018_e1ee(in_struct_a: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    let struct_a: *mut astruct_44;

    struct_a = (in_struct_a >> 0x10);
    in_struct_a.ptr_a_lo = s_0_020_1050_3ab0;
    (in_struct_a + 2) = &PTR_LOOP_1050_1008;
    in_struct_a.ptr_a_lo = s_1_1050_389a;
    (in_struct_a + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(in_struct_a);
    }
    return in_struct_a;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_e230(param_1: *mut astruct_65, param_2: u16,param_3: u32)

{
    let mut uVar1: u16;
    let mut iVar2: i32;
    let mut unaff_BP: u16;
    let mut uvar3: u16;
    let ppVar4: *mut pass1_struct_1;

    load_cursor_1020_7f7a(param_1, CONCAT22(param_2, 4), param_3);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    (iVar2 + 0xee) = 0;
    (iVar2 + 0xf2) = 0;
    param_1.ptr_a_lo = 0xe44e;
    (iVar2 + 2) = 0x1018;
    (iVar2 + 0xe2) = 0xe4ea;
    (iVar2 + 0xe4) = 0x1018;
    ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, 0x26));
    uVar1 = (ppVar4 >> 0x10);
    (iVar2 + 0xf2) = ppVar4;
    (iVar2 + 0xf4) = uVar1;
    (iVar2 + 0xe6) = (iVar2 + 0xf2);
    (iVar2 + 0xe8) = uVar1;
    return;
}



pub fn pass1_1018_e3e8(param_1: *mut astruct_594)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut uVar4: u16;
    let mut in_stack_0000fff6: u16;
    let temp_8621841cd79: *mut astruct_594;

    uVar4 = (param_1 >> 0x10);
    puVar1 = (param_1 + 0xee);
    uVar2 = (param_1 + 0xf0);
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;
        (**ppcVar3)();}
    }
    destroy_win_1008_628e(param_1, in_stack_0000fff6);
    return;
}


pub fn pass1_1018_e5aa(in_struct_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    select_and_delete_palette_fn_1018_e57a(in_struct_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}


pub fn pass1_1018_e678(in_struct_1: *mut astruct_623)

{
    let local_AX_12: *mut u8;
    let struct_a_2: *mut astruct_623;
    let struct_a_1: *mut astruct_623;
    let local_DXAX_69: *mut u8;
    let fn_ptr_1: fn();

    struct_a_1 = (in_struct_1 >> 0x10);
    struct_a_2 = in_struct_1;
    local_AX_12 = (struct_a_2.field_0xf0 | struct_a_2.field_0xee);
    local_DXAX_69._0_2_ = local_AX_12;
    if (local_AX_12 != 0x0)
    {
        fn_ptr_1 = (&struct_a_2.field_0xee + 8);
        local_DXAX_69._0_2_ = (**fn_ptr_1)();
    }
    if (struct_a_2.field_0xea == 0)
    {
        struct_a_2.field_0xea = 1;
        local_DXAX_69 = pass1_1038_af40(_g_astruct_112_a, struct_a_2.field_0x8, 0x15);
        local_DXAX_69._0_2_ = local_DXAX_69;
    }
    return local_DXAX_69;
}
// WARNING: Globals starting with '_' overlap smaller symbols at the same address




pub fn pass1_1018_e834(param_1: *mut astruct_199, param_2: *mut HWND16)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let puVar3: *mut u16;
    let mut uVar4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut uVar6: u16;
    let mut in_stack_0000fff2: u16;
    let mut local_6: u32;

    uVar6 = SUB42(param_2, 0);
    get_dc_1020_921c(CONCAT22(uVar6, param_1), (param_2 >> 0x10));
    &param_1[1].field_0x8 = 0;
    CONCAT22(uVar6, param_1) = 0xe912;
    param_1.field_0x2 = 0x1018;
    ppVar5 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff2, 10));
    uVar4 = (ppVar5 >> 0x10);
    param_1[1].field_0x8 = ppVar5;
    param_1[1].field_0xa = uVar4;
    (&param_1.field_0x4 + 2) = param_1[1].field_0x8;
    param_1.field_0x8 = uVar4;
    uVar2 = &param_1[1].field_0x8;
    puVar3 = &param_1.field_0xa;
    ppcVar1 = ((uVar2 + 10) + 8);
    (**ppcVar1)();
    (&param_1[1].field_0x4 + 2) = puVar3;
    draw_1020_9364(CONCAT22(uVar6, param_1));
    return param_1;
}


pub fn pass1_1018_e9de(param_1: *mut astruct_44)

{
    let local_struct_1: *mut astruct_376;
    let mut local_struct_1_hi: u16;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = 0xebd0;
    local_struct_1.ptr_a_hi = 0x1018;
    local_struct_1.u16_xe2 = 0xec6c;
    local_struct_1.u16_xe4 = 0x1018;
    process_struct_1020_808e(param_1);
    return;
}

pub fn post_win_msg_1018_ea0a(param_1: u16_00, param_2: u16, param_1: i32)

{
    if (param_1 == 0xed)
    {
        PostMessage16(0, 0xc6, 0x111, g_h_window);
    }
    return;
}



pub fn pass1_1018_ea2c(param_1: *mut astruct_625, param_2: u16)

{
    let local_struct_1: *mut astruct_625;
    let local_struct_1_hi: *mut astruct_625;
    let mut temp_5f8c9843ed: u32;

    local_struct_1 = param_1;
    local_struct_1_hi = (param_1 >> 0x10);
    if (param_2 == 1)
    {
        local_struct_1.field_0x14 = 0;
        return;
    }
    if (param_2 != 0xb)
    {
        return;
    }
    temp_5f8c9843ed = local_struct_1.field_0x14;
    (temp_5f8c9843ed + 0xe) =
        (&local_struct_1[-10].field_0x14 + 2);
    return;
}

pub fn pass1_1018_ebaa(in_struct_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    pass1_1018_e9de(in_struct_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}


pub fn pass1_1020_022c(in_struct_1: *mut astruct_376)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let uVar3: u8;
    let local_struct_1: *mut astruct_215;
    let local_struct_1_hi: *mut astruct_215;
    let temp_862254695c9: *mut u32;
// fn_ptr_1: *mut *mut u8;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x45a;
    local_struct_1.ptr_a_hi = 0x1020;
    puVar1 = local_struct_1.field_0xe6;
    uVar2 = local_struct_1.field_0xe8;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{fn_ptr_1 = *puVar1;
        (**fn_ptr_1)();}
    }
    uVar3 = pass1_1008_57c4((in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0xd2)));
    in_struct_1.ptr_a_lo = 0x380a;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    in_struct_1.ptr_a_lo = s_1_1050_389a;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    return uVar3;
}


pub fn pass1_1020_0434(in_struct_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    pass1_1020_022c(in_struct_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}


pub fn pass1_1020_05d6(in_struct_1: *mut astruct_376)

{
    let local_struct_1: *mut astruct_376;
    let local_struct_1_hi: *mut astruct_376;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x75a;
    local_struct_1.ptr_a_hi = 0x1020;
    if (&local_struct_1.u16_x6 != 0)
    {
        pass1_1010_1ea6(*&local_struct_1.u16_x6,
                        (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10));
    }
    in_struct_1.ptr_a_lo = s_0_020_1050_3ab0;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    in_struct_1.ptr_a_lo = s_1_1050_389a;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1020_0734(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    pass1_1020_05d6(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1020_07f4(param_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    pass1_1020_022c(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1020_0a0c(param_1: *mut astruct_594)

{
    let pvVar1: *mut void;
    let local_struct_1: *mut astruct_594;
    let mut unaff_SI: u16;
    let local_struct_1_hi: *mut astruct_594;
    let _fn_ptr_1: *mut u32;
    let fn_ptr_1: fn();

    destroy_win_1008_628e(param_1, unaff_SI);
    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    _fn_ptr_1 = local_struct_1.u32_xE2;
    pvVar1 = local_struct_1.vptr_xE4;
    if ((pvVar1 | _fn_ptr_1) != 0)
    {
        unsafe{fn_ptr_1 = *_fn_ptr_1;}
        (**fn_ptr_1)(&PTR_LOOP_1050_1008, _fn_ptr_1, pvVar1, 1);
    }
    &local_struct_1.u32_xE2 = 0;
    &local_struct_1.u8_ptr_32_xE6 = 0;
    return;
}


pub fn pass1_1020_0d82(in_struct_1: *mut astruct_376, param_2: u8) -> *mut astruct_376

{
    let mut uVar1: u16;

    uVar1 = (in_struct_1 >> 0x10);
    in_struct_1.ptr_a_lo = s_0_020_1050_3ab0;
    (in_struct_1 + 2) = &PTR_LOOP_1050_1008;
    in_struct_1.ptr_a_lo = s_1_1050_389a;
    (in_struct_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn pass1_1040_a626(param_1: *mut u16, param_2: *mut libc::c_char)

{
    let mut uVar1: u16;
    let mut local_DX__1: u16;

    uVar1 = pass1_fn_1008_60e8(param_2);
    unsafe{*param_1 = uVar1;}
    (param_1 + 2) = local_DX__1;
    return;
}

pub fn pass1_1040_a5d0(param_1: *mut astruct_353)

{
    let mut uVar1: i32;
    let mut uVar2: i32;
    let local_BX_4: *mut astruct_353;
    let mut uvar3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar1 = local_BX_4.field_0x2;
    uVar2 = local_BX_4.field_0x4;
    if ((uVar2 | uVar1) != 0)
    {
        call_fn_ptr_in_loop_1000_54e8(0xa582, &PTR_LOOP_1050_1040, (uVar1 - 2), 10, uVar1, uVar2);
        error_check_1000_17ce(CONCAT22(uVar2, uVar1 - 2));
    }
    error_check_1000_17ce(local_BX_4.field_0xc);
    return;
}


pub fn pass1_1040_a544(param_1: u32)

{
    let puVar1: *mut u32;
    let pbVar2: *mut byte;
    let mut bVar3: u8;
    let mut uVar4: i32;
    let mut cVar5: u8;
    let mut in_AL: u8;
    let mut bVar6: u8;
    let mut bVar7: u8;
    let mut in_CX: u16;
    let mut iVar8: i32;
    let mut in_DX: i32;
    let mut uVar9: i32;
    let mut bVar10: u8;
    let mut iVar12: i32;
    let mut in_BX: i32;
    let mut bVar13: u8;
    let puVar14: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar15: bool;
    let mut bVar16: bool;
    let mut in_stack_0000407d: u8;
    let mut in_stack_0000407f: u8;
    let puStack34: *mut u8;
    let mut bVar11: u8;

    puStack34 = &stack0xfffe;
    puVar14 = &stack0xfffe;
    cVar5 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar14 = puVar14 + -1;
        unsafe{*puVar14 = *unaff_BP;}
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    uVar9 = in_DX + 1;
    bVar13 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar13;
    bVar11 = (uVar9 >> 8);
    bVar10 = bVar11 + bVar13;
    bVar15 = CARRY1(bVar11, bVar13) || CARRY1(bVar10, in_CF);
    bVar10 = bVar10 + in_CF;
    bVar3 = uVar9;
    pbVar2 = unaff_SI + in_BX;
    unsafe{bVar11 = *pbVar2;
    bVar6 = *pbVar2 - bVar3;
    bVar16 = *pbVar2 < bVar3 || bVar6 < bVar15;
    *pbVar2 = bVar6 - bVar15;
    if (*pbVar2 == 0 || (SBORROW1(bVar11, bVar3) != SBORROW1(bVar6, bVar15)) != *pbVar2 < '\0')
    {
        if (*pbVar2 != 0)
        {
            error_check_1000_17ce(param_1);
        }
        return;
    }
    bVar6 = in_AL + 1;
    pbVar2 = unaff_SI;
    bVar15 = CARRY1(*pbVar2, bVar13) || CARRY1(*pbVar2 + bVar13, bVar16);
    *pbVar2 = *pbVar2 + bVar13 + bVar16;
    bVar11 = bVar6 + bVar3;
    bVar16 = CARRY1(bVar6, bVar3) || CARRY1(bVar11, bVar15);
    bVar7 = bVar11 + bVar15;
    PTR_LOOP_1050_1040 = (bVar7 | 0x4000);
    if ((SCARRY1(bVar6, bVar3) != SCARRY1(bVar11, bVar15)) == bVar7 < '\0')
    {
        pbVar2 = unaff_SI + -0x78;
        *pbVar2 = *pbVar2 + in_CX + bVar16;
        CONCAT22(&stack0xbf2a, &stack0xfffe) = 0;
        return;
    }
    pbVar2 = unaff_SI;
    *pbVar2 = *pbVar2 + in_CX +
              (CARRY1(in_stack_0000407d, bVar7) || CARRY1(in_stack_0000407d + bVar7, bVar16));
    pbVar2 = unaff_SI + in_BX + 0x10;
    *pbVar2 = *pbVar2 + 0x54;
    pbVar2 = unaff_SI + in_BX + 0x10;
    bVar3 = *pbVar2;
    *pbVar2 = *pbVar2 + 0x72;
    iVar12 = CONCAT11(bVar13, 0x40);
    pbVar2 = unaff_SI + iVar12 + -0x7f;
    bVar11 = *pbVar2;
    bVar6 = *pbVar2;
    *pbVar2 = bVar6 + 0x40 + (0x8d < bVar3);
    puVar1 = (unaff_SI + iVar12 + 0x10);
    uVar4 = *puVar1;
    *puVar1 = *puVar1 + 0x60ea;
    pbVar2 = unaff_SI + iVar12;
    *pbVar2 = (*pbVar2 - (uVar9 & 0xff)) - (0x9f15 < uVar4);
    iVar8 = (uVar9 & 0xff |
             (bVar10 + bVar13 +
                          (CARRY1(in_stack_0000407f, bVar10) ||
                           CARRY1(in_stack_0000407f + bVar10,
                                  0xbf < bVar11 || CARRY1(bVar6 + 0x40, 0x8d < bVar3))))
                 << 8) -
            1;
    pbVar2 = unaff_SI + iVar12 + 0x10;
    *pbVar2 = *pbVar2 + 0x66;
    pbVar2 = unaff_SI + iVar12 + 0x10;
    bVar11 = *pbVar2;
    *pbVar2 = *pbVar2 - 0x22;
    if (-1 < *pbVar2)
    {
        bVar10 = (in_CX >> 8);
        bVar6 = (iVar8 >> 8);
        pbVar2 = unaff_SI + iVar12;
        *pbVar2 = (*pbVar2 - iVar8) -
                  (CARRY1(bVar6, bVar10) || CARRY1(bVar6 + bVar10, 0x21 < bVar11));
        loop
        {
            // WARNING: Do nothing block with infinite loop
        } 
    }
}
    return;
}

pub fn pass1_1040_a564(param_1: *mut u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    unsafe{*param_1 = 0;}
    (param_1 + 4) = 0;
    (param_1 + 6) = 0;
    return;
}

pub fn pass1_1040_a204(param_1: *mut u16, param_2: u8)

{
    unsafe{*param_1 = s_1_1050_389a;}
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_9422(param_1: *mut u32)

{
    let mut uVar1: u16;
    let fn_ptr_1: fn();

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 8) != 0)
    {
        unsafe{fn_ptr_1 = (*param_1 + 0x10);}
        (**fn_ptr_1)();
    }
    if ((param_1 + 4) != 0)
    {
        unsafe{fn_ptr_1 = (*param_1 + 0x14);}
        (**fn_ptr_1)();
    }
    return;
}


pub fn pass1_1040_8f16(param_1: u32, param_2: u8)

{
    pass1_1040_8e82(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_8e58(param_1: *mut astruct_360,param_2: u32,param_3: u32)

{
    let mut in_EAX: u32;
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar1 = (in_EAX >> 0x10);
    uVar2 = param_2;
    pass1_1040_b040(CONCAT22(uVar2, param_1), CONCAT22(param_3, (param_2 >> 0x10)),
                    (param_3 >> 0x10));
    CONCAT22(uVar2, param_1) = 0x8f3c;
    param_1.field_0x2 = &PTR_LOOP_1050_1040;
    return CONCAT22(uVar1, param_1);
}

pub fn pass1_1040_8e82(param_1: *mut astruct_44)

{
    param_1.ptr_a_lo = 0x8f3c;
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn pass1_1040_824a(param_1: u32, param_2: u16)

{
    if ((param_1 + 6) != param_2)
    {
        return 1;
    }
    return 0;
}



pub fn pass1_1040_79c0(param_1: *mut u32, param_2: *mut i32, param_3: u16, param_4: u16, uparam_4_00: i32) -> i32

{
    let ppcVar1: fn();
    let mut cVar2: u8;
    let mut uvar3: u16;
    let mut local_4: u16;

    if (param_4_00 == 0xa1)
    {
        unsafe{ppcVar1 = (*param_1 + 0x38);}
        uVar3 = (**ppcVar1)();
        return uVar3;
    }
    if (param_4_00 < 0xa2)
    {
        if (param_4_00 == 0x85)
        {
            unsafe{ppcVar1 = (*param_1 + 0x1c);}
            (**ppcVar1)();
            return 1;
        }
        if (param_4_00 < 0x86)
        {
            cVar2 = param_4_00;
            if (cVar2 == 0x2)
            {
                unsafe{ppcVar1 = (*param_1 + 0x24);}
                (**ppcVar1)();
                return 1;
            }
            if (cVar2 == 0xc)
            {
                unsafe{ppcVar1 = (*param_1 + 0x18);}
                (**ppcVar1)();
                return 1;
            }
            if (cVar2 == 0xf)
            {
                unsafe{ppcVar1 = (*param_1 + 0x60);}
                uVar3 = (**ppcVar1)();
                return uVar3;
            }
            if (cVar2 == '+')
            {
                unsafe{if (*param_2 != 4)
                {
                    return 1;
                }}
                get_prop_1040_9566(param_2, param_3);
                return 1;
            }
        }
    }
    else
    {
        if (param_4_00 == 0x114)
        {
            unsafe{ppcVar1 = (*param_1 + 0x58);
            uVar3 = (**ppcVar1)();}
            return uVar3;
        }
        if (param_4_00 < 0x115)
        {
            if (param_4_00 == 0x104)
            {
                unsafe{ppcVar1 = (*param_1 + 0x30);
                uVar3 = (**ppcVar1)();}
                return uVar3;
            }
            if (param_4_00 == 0x111)
            {
                unsafe{ppcVar1 = (*param_1 + 0x10);
                uVar3 = (**ppcVar1)();}
                return uVar3;
            }
        }
        else
        {
            if (param_4_00 == 0x115)
            {
                unsafe{ppcVar1 = (*param_1 + 0x54);
                uVar3 = (**ppcVar1)();}
                return uVar3;
            }
            if (param_4_00 == 0x201)
            {
                unsafe{ppcVar1 = (*param_1 + 0x44);
                (**ppcVar1)();}
                return 1;
            }
            if (param_4_00 == 0x204)
            {
                unsafe{ppcVar1 = (*param_1 + 0x28);
                (**ppcVar1)();}
                return 1;
            }
        }
    }
    return 0;
}


pub fn pass1_1040_7700(param_1: *mut astruct_44)

{
    let puVar1: *mut u16;
    byte **ppbVar2;
    let pbVar3: *mut byte;
    let uVar4: u8;
    let mut uVar5: u32;
    let mut bVar6: u8;
    let puVar7: *mut u16;
    let mut uVar8: u32;
    let mut in_AL: u8;
    let mut cVar9: u8;
    let Var10: u16;
    let mut in_CL: u8;
    let mut bVar11: u8;
    let mut bVar12: u8;
    let mut in_EDX: u32;
    let in_BX: *mut u16;
    let mut bVar13: u8;
    let local_BX_75: *mut astruct_977;
    let puVar14: *mut u16;
    let unaff_BP: *mut u16;
    let puVar15: *mut u16;
    let unaff_SI: *mut byte;
    let unaff_DI: *mut u8;
    let mut unaff_ES: u16;
    let mut uVar16: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar17: bool;
    let mut bVar18: bool;
    let mut in_ZF: bool;
    let local_4e: u8;
    let pcStack8: *mut libc::c_char;

    puVar14 = &stack0xfffe;
    puVar15 = &stack0xfffe;
    cVar9 = '\x0f';
    puVar7 = unaff_BP;
    while
    {
        puVar7 = puVar7 + -1;
        puVar14 = puVar14 + -1;
        unsafe{*puVar14 = *puVar7;}
        cVar9 = cVar9 + -1;
        '\0' < cVar9
    } {}
    uVar4 = _in(in_EDX);
    unsafe{*unaff_DI = uVar4;}
    bVar11 = in_EDX;
    if (in_ZF)
    {
        pbVar3 = (in_BX + unaff_SI);
        bVar17 = false;
        unsafe{*pbVar3 = *pbVar3 | bVar11;}
        cVar9 = in_AL;
    }
    else
    {
        bVar12 = (in_EDX >> 8);
        bVar13 = (in_BX >> 8);
        bVar17 = CARRY1(bVar12, bVar13) || CARRY1(bVar12 + bVar13, in_CF);
        in_EDX = in_EDX & 0xff;
        pbVar3 = (in_BX + unaff_SI);
        unsafe{bVar12 = *pbVar3;
        bVar6 = *pbVar3 - bVar11;
        bVar18 = *pbVar3 < bVar11 || bVar6 < bVar17;
        *pbVar3 = bVar6 - bVar17;
        if (*pbVar3 == 0 || (SBORROW1(bVar12, bVar11) != SBORROW1(bVar6, bVar17)) != *pbVar3 < '\0')
        {
            ppbVar2 = (unaff_SI + 9);
            *ppbVar2 = unaff_SI + *ppbVar2;
            error_check_1000_17ce(param_1);
            return;
        }
        pbVar3 = unaff_SI;
        bVar12 = *pbVar3;
        bVar6 = *pbVar3;
        *pbVar3 = bVar6 + bVar13 + bVar18;
        bVar17 = CARRY1(local_4e, in_BX) ||
                 CARRY1(local_4e + in_BX, CARRY1(bVar12, bVar13) || CARRY1(bVar6 + bVar13, bVar18));
        pbVar3 = unaff_SI + -0x4f;
        bVar12 = *pbVar3;
        bVar6 = *pbVar3;
        *pbVar3 = bVar6 + bVar13 + bVar17;
        pbVar3 = unaff_SI + -0x78;
        *pbVar3 = *pbVar3 + in_CL + (CARRY1(bVar12, bVar13) || CARRY1(bVar6 + bVar13, bVar17));
        pbVar3 = (in_BX + unaff_SI);
        bVar17 = *pbVar3 < bVar11;
        puVar15 = &stack0xfffe;
        cVar9 = in_AL + '\x03';
        if (*pbVar3 == bVar11)
        {
            bVar17 = false;
            puVar15 = unaff_BP;
            cVar9 = in_AL + '\x03';
        }}
    }
    pbVar3 = unaff_SI + 0x4074;
    unsafe{
    bVar11 = *pbVar3;
    bVar12 = *pbVar3;
    *pbVar3 = bVar12 + in_CL + bVar17;
    pbVar3 = (in_BX + unaff_SI);
    *pbVar3 = *pbVar3 | cVar9 + in_CL + (CARRY1(bVar11, in_CL) || CARRY1(bVar12 + in_CL, bVar17));
    puVar1 = puVar15 + -0x1e;
    *puVar1 = *puVar1 + in_EDX;
    *in_BX = s_1_1050_389a;
    in_BX[1] = &PTR_LOOP_1050_1008;
    puVar1 = (puVar15 + 3);
    *puVar1 = (s_18_2_1050_3aa5 + 3);
    (puVar1 + 2) = &PTR_LOOP_1050_1008;
    puVar1 = (puVar15 + 3);
    uVar16 = (puVar1 >> 0x10);
    local_BX_75 = puVar1;
    local_BX_75.field_0x4 = 0;
    local_BX_75.field_0x6 = 0;
    local_BX_75.field_0x8 = puVar15[9];
    local_BX_75.field_0xa = puVar15[8];
    local_BX_75.field_0xc = 0;
    local_BX_75.field_0x60 = 0;
    local_BX_75.field_0x62 = 0;
    local_BX_75.field_0x64 = 0;
    local_BX_75.field_0x66 = 0;
    local_BX_75.field_0x68 = 0;
    local_BX_75.field_0x6a = (puVar15 + 6);
    local_BX_75.field_0x6e = puVar15[5];
    local_BX_75.field_0x70 = 0;
    local_BX_75.field_0x74 = 0;
    local_BX_75.field_0x76 = 0;
    local_BX_75.field_0x78 = 0;
    local_BX_75.field_0x8a = 0;
    local_BX_75.field_0x8c = 0;
    *puVar1 = 0x840c;}
    local_BX_75.field_0x2 = &PTR_LOOP_1050_1040;
    pcStack8 = (puVar1 & 0xffff0000 | &local_BX_75.field_0x10);
    copy_string_1000_3d3e(pcStack8, 0x10505db0);
    uVar5 = (puVar15 + 3);
    pass1_1000_4906((uVar5 & 0xffff0000 | (uVar5 + 0x7a)), 0, 8);
    uVar5 = (puVar15 + 3);
    pass1_1000_4906((uVar5 & 0xffff0000 | (uVar5 + 0x82)), 0, 8);
    iVar10 = GetSystemMetrics16(4);
    uVar8 = (puVar15 + 3);
    (uVar8 + 0x62) = iVar10;
    iVar10 = GetSystemMetrics16(5);
    uVar8 = (puVar15 + 3);
    (uVar8 + 100) = iVar10;
    iVar10 = GetSystemMetrics16(6);
    uVar8 = (puVar15 + 3);
    (uVar8 + 0x66) = iVar10;
    return;
}


pub fn pass1_1040_767e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_741e(param_1: u32)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_5: *mut astruct_978;
    let mut uVar4: i32;

    uVar4 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    pass1_1010_1ea6(local_BX_5.field_0x94,
                    (param_1 & 0xffff | uVar4 << 0x10));
    puVar1 = local_BX_5.field_0x98;
    uVar2 = &local_BX_5.field_0x9a;
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;
        (**ppcVar3)(0x1010, puVar1, uVar2, 1);}
    }
    &local_BX_5.field_0x98 = 0;
    local_BX_5.field_0x94 = 0;
    return;
}


pub fn pass1_1040_7044(param_1: *mut u32, param_2: *mut i32, param_3_00: *mut u8, param_3: *mut u8, param_5: *mut u8)

{
    let ppcVar1: fn();
    let uVar2: u8;

    if (param_5 == (s_New_failed_in_Op__Op_1050_0020 + 0xb))
    {
        let param_2_val = unsafe{*param_2};
        if (param_2_val == 4)
        {
            get_prop_1040_9566(param_2, param_3_00);
        }
    }
    else
    {
        if (param_5 != (s_New_failed_in_Op__Op__Simulator_1050_0110 + 1))
        {
            uVar2 = pass1_1040_b316(param_1, param_2, param_3_00, param_3, param_5);
            return uVar2;
        }
        unsafe{
        ppcVar1 = (*param_1 + 0x80);
        (**ppcVar1)();}
    }
    return 0x1;
}


pub fn pass1_1040_6f0c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_6862(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_6f8e(param_1: *mut astruct_44, param_2: *mut u8)

{
    let pbVar1: *mut byte;
    let pcVar2: *mut libc::c_char;
    let uVar3: u8;
    let mut bVar4: u8;
    let mut cVar5: u8;
    let mut in_CL: u8;
    let mut bVar6: u8;
    let mut in_DX: i32;
    let mut uVar7: i32;
    let mut bVar9: u8;
    let mut in_BX: i32;
    let mut bVar11: u8;
    let mut iVar10: i32;
    let puVar12: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let unaff_DI: *mut u8;
    let mut unaff_ES: u16;
    let mut uVar13: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar14: bool;
    let mut bVar15: bool;
    let local_4e: u8;
    let mut bVar8: u8;

    puVar12 = &stack0xfffe;
    cVar5 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar12 = puVar12 + -1;
        unsafe{*puVar12 = *unaff_BP;}
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    uVar3 = _in(in_DX);
    unsafe{*unaff_DI = uVar3;}
    bVar8 = (in_DX >> 8);
    bVar11 = (in_BX >> 8);
    bVar4 = bVar8 + bVar11;
    bVar14 = CARRY1(bVar8, bVar11) || CARRY1(bVar4, in_CF);
    uVar7 = in_DX & 0xff | (bVar4 + in_CF) << 8;
    pbVar1 = unaff_SI + in_BX;
    bVar6 = (in_DX & 0xff);
    unsafe{bVar4 = *pbVar1;
    bVar8 = *pbVar1 - bVar6;
    bVar15 = *pbVar1 < bVar6 || bVar8 < bVar14;
    *pbVar1 = bVar8 - bVar14;
    if (*pbVar1 != 0 && (SBORROW1(bVar4, bVar6) != SBORROW1(bVar8, bVar14)) == *pbVar1 < '\0')
    {
        pbVar1 = unaff_SI;
        bVar4 = *pbVar1;
        bVar8 = *pbVar1;
        *pbVar1 = bVar8 + bVar11 + bVar15;
        bVar9 = in_BX;
        bVar14 = CARRY1(local_4e, bVar9) ||
                 CARRY1(local_4e + bVar9, CARRY1(bVar4, bVar11) || CARRY1(bVar8 + bVar11, bVar15));
        pbVar1 = unaff_SI + -0x4f;
        bVar4 = *pbVar1;
        bVar8 = *pbVar1;
        *pbVar1 = bVar8 + bVar11 + bVar14;
        pbVar1 = unaff_SI + -0x78;
        *pbVar1 = *pbVar1 + in_CL + (CARRY1(bVar4, bVar11) || CARRY1(bVar8 + bVar11, bVar14));
        bVar14 = unaff_SI[in_BX] < bVar6;
        uVar3 = _in(uVar7);
        unaff_DI[1] = uVar3;
        pbVar1 = unaff_SI + -1 + (&stack0xffff + 1);
        bVar4 = *pbVar1;
        bVar8 = *pbVar1;
        *pbVar1 = bVar8 + bVar9 + bVar14;
        uVar13 = _in(uVar7);
        (unaff_DI + 2) = uVar13;
        pcVar2 = unaff_DI + -0x71;
        *pcVar2 = *pcVar2 + bVar6 + (CARRY1(bVar4, bVar9) || CARRY1(bVar8 + bVar9, bVar14));
        _in(uVar7);
        process_struct_1040_b0bc(param_1, 0, CONCAT22(param_2, 0xfd9));
        uVar13 = (param_1 >> 0x10);
        iVar10 = param_1;
        (iVar10 + 0x94) = 0;
        (iVar10 + 0x98) = 0;
        param_1.ptr_a_lo = 0x76a4;
        (iVar10 + 2) = &PTR_LOOP_1050_1040;
        return;
    }
    if (*pbVar1 != 0)
    {
        error_check_1000_17ce(param_1);
    }}
    return;
}


pub fn pass1_1040_6cac(param_1: u32)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: i32;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_1ea6(*(iVar4 + 0x94), (param_1 & 0xffff | uVar5 << 0x10));
    puVar1 = (iVar4 + 0x98);
    uVar2 = (iVar4 + 0x9a);
    if ((uVar2 | puVar1) != 0)
    {
        unsafe{ppcVar3 = *puVar1;
        (**ppcVar3)(0x1010, puVar1, uVar2, 1);}
    }
    (iVar4 + 0x98) = 0;
    (iVar4 + 0x94) = 0;
    return;
}

pub fn pass1_1040_6cfa(param_1: *mut u8)

{
    let ppcVar1: fn();

    ppcVar1 = ((param_1 + 0x98) + 8);
    (**ppcVar1)();
    return (&PTR_LOOP_1050_0000 + 1);
}


pub fn pass1_1040_68d2(param_1: *mut u32, param_2: *mut i32, param_3: *mut u8, param_3_00: *mut u8, param_5: *mut u8)

{
    let ppcVar1: fn();
    let uVar2: u8;
    let extraout_AH: u8;

    if (param_5 == (s_New_failed_in_Op__Op_1050_0020 + 0xb))
    {
        let param_2_val = unsafe{*param_2};
        if (param_2_val == 4)
        {
            get_prop_1040_9566(param_2, param_3);
        }
    }
    else
    {
        if (param_5 != (s_New_failed_in_Op__Op__Simulator_1050_0110 + 1))
        {
            uVar2 = pass1_1040_b316(param_1, param_2, param_3, param_3_00, param_5);
            return CONCAT11(extraout_AH, uVar2);
        }
        let param_2_val = unsafe{*param_2};
        ppcVar1 = (param_2_val + 0x80);
        (**ppcVar1)();
    }
    return 1;
}


pub fn pass1_1040_6794(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_6470(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_6826(param_1: *mut astruct_346, param_2: *mut u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    process_struct_1040_b0bc(param_1, 0, CONCAT22(param_2, 0xfda));
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = 0;
    (iVar1 + 0x98) = 0;
    param_1 = 0x6f32;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    return;
}

pub fn pass1_1040_6862(param_1: *mut astruct_44)

{
    param_1.ptr_a_lo = 0x6f32;
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    win_cleanup_func_1040_b0f8(param_1);
    return;
}



pub fn pass1_1040_6402(param_1: *mut astruct_68, param_2: *mut u8)

{
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut unaff_BP: u16;
    let mut uvar3: u16;
    let ppVar4: *mut pass1_struct_1;

    process_struct_1040_7728(param_1, (&PTR_LOOP_1050_0000 + 1), 0, s_logo_bmp_1050_1850, param_2);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    (iVar2 + 0x8e) = 0;
    (iVar2 + 0x92) = 0;
    param_1.field_0x0 = 0x67ba;
    (iVar2 + 2) = &PTR_LOOP_1050_1040;
    ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, 0x2b));
    (iVar2 + 0x92) = ppVar4;
    (iVar2 + 0x94) = (ppVar4 >> 0x10);
    ppcVar1 = ((iVar2 + 0x92) + 4);
    (**ppcVar1)();
    return;
}



pub fn pass1_1040_6470(param_1: *mut astruct_45)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.u16_0x0 = 0x67ba;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    if ((iVar1 + 0x92) != 0)
    {
        pass1_1010_1ea6(*(iVar1 + 0x92), param_1);
    }
    pass1_1038_b6e0(_g_astruct_112_a, *(iVar1 + 6));
    error_check_1000_17ce((iVar1 + 0x8e));
    win_cleanup_func_1040_782c(param_1);
    return;
}


pub fn pass1_1040_5eaa(param_1: *mut u8) -> i32

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    match ((iVar1 + 0x9a))
    {
    0 |
    0x70 |
    0x71 =>{
        (iVar1 + 0x98) = 0;
        return iVar1;},
    1 |
    2 =>{
        (iVar1 + 0x98) = 0xd;
        return iVar1;},
    3 =>{
        (iVar1 + 0x98) = 0xe;
        return iVar1;},
    4 |
    0x4b =>{
        (iVar1 + 0x98) = 0xf},
    5 =>{
        (iVar1 + 0x98) = 0x10;
        return iVar1;},
    6 =>{
        (iVar1 + 0x98) = 0x11;
        return iVar1;},
    7 =>{
        (iVar1 + 0x98) = 0x12},
    8 =>{
        (iVar1 + 0x98) = 0x13},
    9 |
    10 |
    0xb =>{
        (iVar1 + 0x98) = 0x14},
    0xc =>{
        (iVar1 + 0x98) = 0x18},
    0xd =>{
        (iVar1 + 0x98) = 0x19},
    0xe |
    0x76 =>{
        (iVar1 + 0x98) = 0x17},
    0xf |
    0x10 |
    0x11 =>{
        (iVar1 + 0x98) = 0x1a},
    0x12 =>{
        (iVar1 + 0x98) = 0x1b},
    0x13 =>{
        (iVar1 + 0x98) = 0x1c},
    0x14 =>{
        (iVar1 + 0x98) = 0x1d},
    0x15 |
    0x16 |
    0x17 |
    0x18 |
    0x19 =>{
        (iVar1 + 0x98) = 0x1e},
    0x1a =>{
        (iVar1 + 0x98) = 0x1f},
    0x1b =>{
        (iVar1 + 0x98) = 0x20},
    0x1c |
    0x1d |
    0x1e =>{
        (iVar1 + 0x98) = 0x21},
    0x1f =>{
        (iVar1 + 0x98) = 0x22},
    0x20 =>{
        (iVar1 + 0x98) = 0x23},
    0x21 =>{
        (iVar1 + 0x98) = 0x24},
    0x22 =>{
        (iVar1 + 0x98) = 0x25},
    0x23 |
    0x24 |
    0x25 |
    0x26 |
    0x27 |
    0x28 |
    0x29 |
    0x2a |
    0x2b =>{
        (iVar1 + 0x98) = 0x26},
    0x2c =>{
        (iVar1 + 0x98) = 0x27},
    0x2d =>{
        (iVar1 + 0x98) = 0x28},
    0x2e |
    0x2f |
    0x30 |
    0x31 =>{
        (iVar1 + 0x98) = 0x29},
    0x32 |
    0x33 |
    0x34 |
    0x35 |
    0x4d =>{
        (iVar1 + 0x98) = 0x2a},
    0x36 =>{
        (iVar1 + 0x98) = 0x2b},
    0x37 |
    0x38 |
    0x39 =>{
        (iVar1 + 0x98) = 0x2c},
    0x3a =>{
        (iVar1 + 0x98) = 0x2d},
    0x3b |
    0x3c =>{
        (iVar1 + 0x98) = 0x2e},
    0x3d =>{
        (iVar1 + 0x98) = 0x2f},
    0x3e =>{
        (iVar1 + 0x98) = 0x30},
    0x3f =>{
        (iVar1 + 0x98) = 0x31},
    0x40 =>{
        (iVar1 + 0x98) = 0x32},
    0x41 =>{
        (iVar1 + 0x98) = 0x33},
    0x42 =>{
        (iVar1 + 0x98) = 0x34},
    0x43 =>{
        (iVar1 + 0x98) = 0x35},
    0x44 =>{
        (iVar1 + 0x98) = 0x36},
    0x45 =>{
        (iVar1 + 0x98) = 0x37},
    0x46 =>{
        (iVar1 + 0x98) = 0x38},
    0x47 =>{
        (iVar1 + 0x98) = 0x39},
    0x48 |
    0x49 |
    0x4a =>{
        (iVar1 + 0x98) = 0x3a},
    0x4c =>{
        (iVar1 + 0x98) = 0x3b},
    0x4e =>{
        (iVar1 + 0x98) = 0x3c},
    0x4f |
    0x50 =>{
        (iVar1 + 0x98) = 0x3d},
    0x51 |
    0x52 |
    0x53 |
    0x54 |
    0x55 =>{
        (iVar1 + 0x98) = 0x3e},
    0x56 |
    0x57 |
    0x58 |
    0x59 |
    0x5a =>{
        (iVar1 + 0x98) = 0x3f},
    0x5b =>{
        (iVar1 + 0x98) = 0x40},
    0x5c |
    0x5d |
    0x5e =>{
        (iVar1 + 0x98) = 0x41},
    0x5f |
    0x60 |
    0x61 =>{
        (iVar1 + 0x98) = 0x42},
    0x62 |
    99 |
    100 |
    0x65 |
    0x66 =>{
        (iVar1 + 0x98) = 0x43},
    0x67 |
    0x68 =>{
        (iVar1 + 0x98) = 0x44},
    0x69 =>{
        (iVar1 + 0x98) = 0x45},
    0x6a =>{
        (iVar1 + 0x98) = 0x46},
    0x6b =>{
        (iVar1 + 0x98) = 0x47},
    0x6c =>{
        (iVar1 + 0x98) = 0x48},
    0x6d =>{
        (iVar1 + 0x98) = 0x49},
    0x6e =>{
        (iVar1 + 0x98) = 0x4a},
    0x6f =>{
        (iVar1 + 0x98) = 0x4b},
    0x74 =>{
        (iVar1 + 0x98) = 0x15},
    0x75 =>{
        (iVar1 + 0x98) = 0x16},
    0x78 |
    0x7a |
    0x7b |
    0x7c |
    0x7d |
    0x7e |
    0x7f |
    0x80 |
    0x81 |
    0x82 =>{
        (iVar1 + 0x98) = 0x4c;
    }}
    return iVar1;
}

pub fn pass1_1040_6360(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1040_63e2(param_1: *mut astruct_44)

{
    byte **ppbVar1;
    let pbVar2: *mut byte;
    let mut bVar3: u8;
    let ppcVar4: fn();
    let mut cVar5: u8;
    let mut bVar6: u8;
    let mut in_CL: u8;
    let mut bVar7: u8;
    let in_BX: *mut u8;
    let mut bVar9: u8;
    let mut iVar8: i32;
    let puVar10: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut unaff_ES: u16;
    let mut uVar11: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar12: bool;
    let mut bVar13: bool;
    let ppVar14: *mut pass1_struct_1;
    let puVar15: *mut u8;
    let in_struct_1: *mut astruct_68;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    puVar10 = &stack0xfffe;
    puVar15 = &stack0xfffe;
    cVar5 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar10 = puVar10 + -1;
        unsafe{*puVar10 = *unaff_BP;}
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    bVar7 = (unaff_ES >> 8);
    bVar9 = (in_BX >> 8);
    bVar12 = CARRY1(bVar7, bVar9) || CARRY1(bVar7 + bVar9, in_CF);
    bVar6 = unaff_ES;
    in_struct_1 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pbVar2 = in_BX + unaff_SI;
    unsafe{
    bVar7 = *pbVar2;
    bVar3 = *pbVar2 - bVar6;
    bVar13 = *pbVar2 < bVar6 || bVar3 < bVar12;
    *pbVar2 = bVar3 - bVar12;
    if (*pbVar2 != 0 && (SBORROW1(bVar7, bVar6) != SBORROW1(bVar3, bVar12)) == *pbVar2 < '\0')
    {
        pbVar2 = unaff_SI;
        bVar12 = CARRY1(*pbVar2, bVar9) || CARRY1(*pbVar2 + bVar9, bVar13);
        *pbVar2 = *pbVar2 + bVar9 + bVar13;
        bVar7 = local_4e + in_BX;
        bVar13 = CARRY1(local_4e, in_BX) || CARRY1(bVar7, bVar12);
        local_4e = bVar7 + bVar12;
        pbVar2 = unaff_SI + -0x4f;
        bVar7 = *pbVar2;
        bVar3 = *pbVar2;
        *pbVar2 = bVar3 + bVar9 + bVar13;
        pbVar2 = unaff_SI + -0x78;
        *pbVar2 = *pbVar2 + in_CL + (CARRY1(bVar7, bVar9) || CARRY1(bVar3 + bVar9, bVar13));
        puStack34 = &stack0xfffe;
        process_struct_1040_7728(in_struct_1, (&PTR_LOOP_1050_0000 + 1), 0, s_logo_bmp_1050_1850, in_BX);
        uVar11 = (in_struct_1 >> 0x10);
        iVar8 = in_struct_1;
        (iVar8 + 0x8e) = 0;
        (iVar8 + 0x92) = 0;
        in_struct_1.field_0x0 = 0x67ba;
        (iVar8 + 2) = &PTR_LOOP_1050_1040;
        ppVar14 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar15, 0x2b));
        uVar11 = (in_struct_1 >> 0x10);
        iVar8 = in_struct_1;
        (iVar8 + 0x92) = ppVar14;
        (iVar8 + 0x94) = (ppVar14 >> 0x10);
        ppcVar4 = ((iVar8 + 0x92) + 4);
        (**ppcVar4)();
        return;
    }
    ppbVar1 = (unaff_SI + 9);
    *ppbVar1 = unaff_SI + *ppbVar1;
}
    error_check_1000_17ce(param_1);
    return;
}



pub fn pass1_1040_5dc4(param_1: *mut u8)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uvar3: u16;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let mut extraout_DX: i32;
    let mut iVar6: i32;
    let mut unaff_SI: u16;
    let mut uVar7: u16;
    let ppVar8: *mut pass1_struct_1;
    let mut uVar9: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;

    ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 3));
    uVar3 = ppVar8;
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pass1_1010_a5ca(uVar3, (ppVar8 >> 0x10), (iVar6 + 0x9a));
    if (uVar3 == 0)
    {
        (iVar6 + 0x94) = 0;
        (iVar6 + 0x9c) = 1;
    }
    if (-1 < uVar3)
    {
        if ((iVar6 + 0x9a) < 0x72)
        {
            uVar9 = 0x31;
        }
        else
        {
            uVar9 = 0x41;
        }
        ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, uVar9));
        uVar4 = (iVar6 + 0x9a);
        ppcVar1 = (ppVar8 + 0x14);
        (**ppcVar1)(0x1010, ppVar8, (ppVar8 >> 0x10), uVar4, uVar4 >> 0xf);
        if ((extraout_DX | uVar4) == 0)
        {
            local_12 = 0;
        }
        else
        {
            uVar2 = (uVar4 + 0x16);
            local_12 = (uVar2 + 4);
        }
        if ((local_12 != 0) && (uVar3 != 0))
        {
            uVar5 = ((local_12 - uVar3) * 100) / local_12;
            uVar4 = uVar5 / 10;
            (iVar6 + 0x94) = uVar4;
            if (4 < uVar5 % 10)
            {
                (iVar6 + 0x94) = uVar4 + 1;
            }
        }
    }
    return;
}


pub fn pass1_1040_5cd6(param_1: *mut u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = pass1_1040_5d12(param_1);
    if (uVar3 != 0)
    {
        iVar1 = (uVar3 + 0x20);
        uVar2 = (param_1 >> 0x10);
        if ((param_1 + 0x9a) != iVar1)
        {
            (param_1 + 0x9a) = iVar1;
            return (&PTR_LOOP_1050_0000 + 1);
        }
    }
    return 0x0;
}

pub fn pass1_1040_5d12(param_1: *mut u8)

{
    let mut uVar1: i32;
    let mut uVar2: i32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut local_4: u16;

    uVar3 = (param_1 + 0x90);
    uVar5 = (uVar3 >> 0x10);
    iVar4 = uVar3;
    uVar1 = (iVar4 + 6);
    uVar2 = (iVar4 + 8);
    if ((uVar2 | uVar1) != 0)
    {
        uVar6 = pass1_1030_73a8(CONCAT22(uVar2, uVar1));
        return uVar6;
    }
    return 0;
}

pub fn pass1_1040_5d42(param_1: *mut u8)

{
    let mut uVar1: i32;
    let mut cVar2: u8;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = pass1_1040_5d12(param_1);
    if (uVar5 != 0)
    {
        uVar1 = (uVar5 + 0xc);
        iVar3 = param_1;
        uVar4 = (param_1 >> 0x10);
        if (uVar1 == 0x5f)
        {
            (iVar3 + 0x96) = 0x53;
            return;
        }
        if (uVar1 < 0x60)
        {
            cVar2 = uVar1;
            if (cVar2 == '(')
            {
                (iVar3 + 0x96) = 0x54;
                return;
            }
            if (cVar2 == ')')
            {
                (iVar3 + 0x96) = 0x55;
                return;
            }
            if (cVar2 == ']')
            {
                (iVar3 + 0x96) = 0x51;
                return;
            }
            if (cVar2 == '^')
            {
                (iVar3 + 0x96) = 0x52;
                return;
            }
        }
    }
    return;
}


pub fn pass1_1040_557c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_4f0a(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_55fe(param_1: *mut astruct_44, param_2: *mut u8, param_3: *mut u8)

{
    let pbVar1: *mut byte;
    let mut bVar2: u8;
    let mut cVar3: u8;
    let mut uVar4: u32;
    let mut in_AL: u8;
    let mut uVar5: i32;
    let mut in_CL: u8;
    let mut bVar6: u8;
    let mut in_DX: u16;
    let mut bVar8: u8;
    let struct_a: *mut astruct_199;
    let paVar7: *mut astruct_199;
    let struct_a_00: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let mut in_BX: i32;
    let mut bVar11: u8;
    let mut iVar9: i32;
    let mut iVar10: i32;
    let puVar12: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut unaff_SS: u16;
    let mut bVar15: bool;
    let mut bVar16: bool;
    let mut uVar17: u32;
    let local_4e: u8;
    let puStack34: *mut u8;
    let piStack12: *mut i32;
    let mut iStack4: i32;

    puStack34 = &stack0xfffe;
    puVar12 = &stack0xfffe;
    cVar3 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar12 = puVar12 + -1;
        unsafe{*puVar12 = *unaff_BP;}
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {} 
    bVar6 = in_DX;
    bVar8 = (in_DX >> 8);
    bVar11 = (in_BX >> 8);
    bVar15 = CARRY1(bVar8, bVar11) ||
             CARRY1(bVar8 + bVar11, (&stack0x003e)[unaff_SI] < bVar6);
    pbVar1 = unaff_SI + in_BX;
    unsafe{bVar8 = *pbVar1;
    bVar2 = *pbVar1 - bVar6;
    bVar16 = *pbVar1 < bVar6 || bVar2 < bVar15;
    *pbVar1 = bVar2 - bVar15;
    if (*pbVar1 != 0 && (SBORROW1(bVar8, bVar6) != SBORROW1(bVar2, bVar15)) == *pbVar1 < '\0')
    {
        pbVar1 = unaff_SI;
        bVar15 = CARRY1(*pbVar1, bVar11) || CARRY1(*pbVar1 + bVar11, bVar16);
        *pbVar1 = *pbVar1 + bVar11 + bVar16;
        bVar8 = local_4e + in_BX;
        bVar16 = CARRY1(local_4e, in_BX) || CARRY1(bVar8, bVar15);
        local_4e = bVar8 + bVar15;
        pbVar1 = unaff_SI + -0x4f;
        bVar8 = *pbVar1;
        bVar2 = *pbVar1;
        *pbVar1 = bVar2 + bVar11 + bVar16;
        pbVar1 = unaff_SI + -0x78;
        *pbVar1 = *pbVar1 + in_CL + (CARRY1(bVar8, bVar11) || CARRY1(bVar2 + bVar11, bVar16));
        pbVar1 = &stack0x003e + unaff_SI;
        bVar8 = *pbVar1;
        *pbVar1 = bVar8 << 7 | (CONCAT11(unaff_SI[in_BX] < bVar6, bVar8) >> 2);
        pbVar1 = unaff_SI + in_BX;
        *pbVar1 = *pbVar1 + in_AL + 0x4 + in_CL + ((bVar8 & 2) != 0);
        puStack34 = &stack0xfffe;
        process_struct_1040_b082(param_1, CONCAT22(param_3, 0xfa3));
        uVar13 = (param_1 >> 0x10);
        iVar9 = param_1;
        uVar5 = 0;
        (iVar9 + 0x94) = 0;
        (iVar9 + 0x96) = 0;
        (iVar9 + 0x98) = 0;
        (iVar9 + 0x9c) = 0;
        param_1.ptr_a_lo = 0x6386;
        (iVar9 + 2) = &PTR_LOOP_1050_1040;
        paVar7 = struct_a;
        process_struct_1000_179c(0x18, struct_a);
        piStack12 = CONCAT22(paVar7, uVar5);
        struct_a_00 = (paVar7 | uVar5);
        if (struct_a_00 == 0x0)
        {
            (iVar9 + 0x90) = 0;
        }
        else
        {
            process_struct_1040_a598(CONCAT22(paVar7, uVar5));
            (iVar9 + 0x90) = uVar5;
            (iVar9 + 0x92) = extraout_DX;
            struct_a_00 = extraout_DX;
        }
        (iVar9 + 0x90) = 6;
        iStack4 = (iVar9 + 0x90);
        uVar5 = iStack4 * 10 + 2;
        process_struct_1000_179c(uVar5, struct_a_00);
        piStack12 = CONCAT22(struct_a_00, uVar5);
        if ((struct_a_00 | uVar5) == 0)
        {
            uVar4 = (iVar9 + 0x90);
            (uVar4 + 2) = 0;
        }
        else
        {
            *piStack12 = iStack4;
            call_fn_ptr_1000_5586(0xa564, &PTR_LOOP_1050_1040, iStack4, 10, uVar5 + 2,
                                  struct_a_00);
            uVar4 = (iVar9 + 0x90);
            uVar14 = (uVar4 >> 0x10);
            iVar10 = uVar4;
            (iVar10 + 2) = uVar5 + 2;
            (iVar10 + 4) = struct_a_00;
        }
        uVar4 = (iVar9 + 0x90);
        *(uVar4 + 6) = param_2;
        uVar4 = (iVar9 + 0x90);
        (uVar4 + 10) = 4;
        uVar4 = (iVar9 + 0x90);
        (uVar4 + 0x12) = (iVar9 + 10);
        uVar17 = pass1_1040_5d12(param_1);
        uVar5 = (uVar17 >> 0x10);
        if ((uVar5 | uVar17) == 0)
        {
            (iVar9 + 0x9a) = 0;
        }
        else
        {
            (iVar9 + 0x9a) = (uVar17 + 0x20);
        }
        return;
    }
    if (*pbVar1 != 0)
    {
        error_check_1000_17ce(param_1);
    }}
    return;
}


pub fn pass1_1040_4f0a(param_1: *mut astruct_44)

{
    param_1.ptr_a_lo = 0x55a2;
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    win_cleanup_func_1040_b0f8(param_1);
    return;
}


pub fn pass1_1040_4f28(param_1: *mut u32, param_2: *mut i32, param_3: *mut u8, param_3_00: *mut u8, param_5: i32) -> i32

{
    let ppcVar1: fn();
    let uVar2: u8;
    let extraout_AH: u8;

    if (param_5 == 0x2b)
    {
        let param_2_val = unsafe{*param_2};
        if (param_2_val)
        {
            get_prop_1040_9566(param_2, param_3);
        }
    }
    else
    {
        if (param_5 != 0x111)
        {
            uVar2 = pass1_1040_b316(param_1, param_2, param_3, param_3_00, param_5);
            return CONCAT11(extraout_AH, uVar2);
        }
        
        unsafe{ppcVar1 = (*param_1 + 0x7c);}
        (**ppcVar1)();
    }
    return (&PTR_LOOP_1050_0000 + 1);
}

pub fn pass1_1040_4f82(param_1: *mut u32)

{
    let ppcVar1: fn();

    unsafe{ppcVar1 = (*param_1 + 0x80);}
    (**ppcVar1)();
    return;
}


pub fn pass1_1040_4d7e(param_1: *mut u8)

{
    let mut u_var1: u32;
    let p_uvar2: *mut u16 ;
    let mut uvar3: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar3 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x90);
    local_6 = (uVar1 + 2);
    local_8 = 0;
    while ((puVar2 = (param_1 + 0x90),
            *puVar2 != local_8 && local_8 <= *puVar2 && ((local_6 + 4) != 6000)))
    {
        local_8 = local_8 + 1;
        local_6 = local_6 & 0xffff0000 | (local_6 + 10);
    }
    pass1_fn_1000_3e2c(local_6);
    return;
}

pub fn pass1_1040_4dcc(param_1: *mut u8, param_2: *mut u8)

{
    let mut u_var1: u32;
    let mut uVar2: u32;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0x90);
    uVar1 = (param_1 + 0x94);
    pass1_1010_ada6(uVar1, (uVar1 >> 0x10), param_2, (uVar2 + 10));
    return;
}

pub fn pass1_1040_4df2(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_4e74(param_1: *mut astruct_44, param_2: libc::c_long, param_3: *mut u8)

{
    byte **ppbVar1;
    let pbVar2: *mut byte;
    let pcVar3: *mut libc::c_char;
    let mut bVar4: u8;
    let mut cVar5: u8;
    let mut uVar6: i32;
    let mut bVar7: u8;
    let mut in_AL: u8;
    let mut bVar8: u8;
    let mut in_DX: i32;
    let mut bVar9: u8;
    let mut in_BX: i32;
    let mut bVar12: u8;
    let mut iVar11: i32;
    let puVar13: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut unaff_DI: i32;
    let mut uVar14: u16;
    let mut uVar15: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar16: bool;
    let mut bVar17: bool;
    let puStack34: *mut u8;
    let mut bVar10: u8;

    puStack34 = &stack0xfffe;
    puVar13 = &stack0xfffe;
    cVar5 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar13 = puVar13 + -1;
        unsafe{*puVar13 = *unaff_BP;}
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    bVar12 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar12;
    bVar10 = (in_DX + 1 >> 8);
    bVar9 = bVar10 + bVar12;
    bVar16 = CARRY1(bVar10, bVar12) || CARRY1(bVar9, in_CF);
    bVar9 = bVar9 + in_CF;
    uVar6 = in_DX + 1 & 0xff;
    pbVar2 = unaff_SI + in_BX;
    bVar8 = uVar6;
    unsafe{
    bVar10 = *pbVar2;
    bVar4 = *pbVar2 - bVar8;
    bVar17 = *pbVar2 < bVar8 || bVar4 < bVar16;
    *pbVar2 = bVar4 - bVar16;
    if (*pbVar2 != 0 && (SBORROW1(bVar10, bVar8) != SBORROW1(bVar4, bVar16)) == *pbVar2 < '\0')
    {
        pbVar2 = unaff_SI;
        bVar16 = CARRY1(*pbVar2, bVar12) || CARRY1(*pbVar2 + bVar12, bVar17);
        *pbVar2 = *pbVar2 + bVar12 + bVar17;
        pbVar2 = &stack0x404a + unaff_SI;
        bVar17 = CARRY1(*pbVar2, bVar9) || CARRY1(*pbVar2 + bVar9, bVar16);
        *pbVar2 = *pbVar2 + bVar9 + bVar16;
        pbVar2 = unaff_SI + -0x4f;
        bVar10 = *pbVar2;
        bVar4 = *pbVar2;
        *pbVar2 = bVar4 + bVar12 + bVar17;
        bVar7 = ((CONCAT11(0x40, in_AL + 0x1) + 1) >> 8);
        pcVar3 = (unaff_DI + -0x75);
        *pcVar3 = *pcVar3 + bVar8 +
                  (CARRY1(bVar7, bVar9) ||
                   CARRY1(bVar7 + bVar9, CARRY1(bVar10, bVar12) || CARRY1(bVar4 + bVar12, bVar17)));
        _in(uVar6 | bVar9 << 8);
        puStack34 = &stack0xfffe;
        process_struct_1040_b0bc(param_1, 0, CONCAT22(param_3, 0xfab));
        uVar14 = (param_1 >> 0x10);
        iVar11 = param_1;
        (iVar11 + 0x94) = 0;
        (iVar11 + 0x98) = 0;
        (iVar11 + 0xb0) = 0;
        (iVar11 + 0xb4) = 0;
        (iVar11 + 0xb6) = 0;
        param_1.ptr_a_lo = 0x55a2;
        (iVar11 + 2) = &PTR_LOOP_1050_1040;
        if (param_2 != 0)
        {
            uVar15 = (param_2 >> 0x10);
            (iVar11 + 0xb0) = (param_2 + 6);
            (iVar11 + 0xb4) = (param_2 + 0x14);
        }
        return;
    }
    ppbVar1 = (unaff_SI + 9);
    *ppbVar1 = unaff_SI + *ppbVar1;
}
    error_check_1000_17ce(param_1);
    return;
}

pub fn pass1_1040_4e94(param_1: *mut astruct_346, param_2: libc::c_long, param_3: *mut u8)

{
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut uvar3: u16;

    process_struct_1040_b0bc(param_1, 0, CONCAT22(param_3, 0xfab));
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = 0;
    (iVar1 + 0x98) = 0;
    (iVar1 + 0xb0) = 0;
    (iVar1 + 0xb4) = 0;
    (iVar1 + 0xb6) = 0;
    param_1 = 0x55a2;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    if (param_2 != 0)
    {
        uVar3 = (param_2 >> 0x10);
        (iVar1 + 0xb0) = (param_2 + 6);
        (iVar1 + 0xb4) = (param_2 + 0x14);
    }
    return;
}


pub fn pass1_1040_47fe(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1040_4880(param_1: *mut astruct_44)

{
    byte **ppbVar1;
    let pbVar2: *mut byte;
    let mut bVar3: u8;
    let mut cVar4: u8;
    let mut uVar5: u32;
    let mut in_AX: u16;
    let mut uVar6: i32;
    let mut in_CX: u16;
    let mut bVar7: u8;
    let mut in_DX: i32;
    let mut bVar8: u8;
    let struct_a: *mut astruct_199;
    let struct_a_00: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let mut in_BX: i32;
    let mut bVar11: u8;
    let mut iVar9: i32;
    let mut iVar10: i32;
    let puVar12: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar15: bool;
    let mut bVar16: bool;
    let ppVar17: *mut pass1_struct_1;
    let puVar18: *mut u16;
    let mut uVar19: u32;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    puVar12 = &stack0xfffe;
    cVar4 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar12 = puVar12 + -1;
        unsafe{*puVar12 = *unaff_BP;}
        cVar4 = cVar4 + -1;
        '\0' < cVar4
    } {}
    bVar11 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar11;
    bVar8 = (in_DX + 1 >> 8);
    bVar3 = bVar8 + bVar11;
    bVar15 = CARRY1(bVar8, bVar11) || CARRY1(bVar3, in_CF);
    uVar6 = in_DX + 1 & 0xff;
    uVar19 = CONCAT22(in_CX, uVar6 | (bVar3 + in_CF) << 8);
    puVar18 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pbVar2 = unaff_SI + in_BX;
    bVar7 = uVar6;
    unsafe{
    bVar3 = *pbVar2;
    bVar8 = *pbVar2 - bVar7;
    bVar16 = *pbVar2 < bVar7 || bVar8 < bVar15;
    *pbVar2 = bVar8 - bVar15;
    if (*pbVar2 != 0 && (SBORROW1(bVar3, bVar7) != SBORROW1(bVar8, bVar15)) == *pbVar2 < '\0')
    {
        pbVar2 = unaff_SI;
        bVar15 = CARRY1(*pbVar2, bVar11) || CARRY1(*pbVar2 + bVar11, bVar16);
        *pbVar2 = *pbVar2 + bVar11 + bVar16;
        bVar3 = local_4e + in_BX;
        bVar16 = CARRY1(local_4e, in_BX) || CARRY1(bVar3, bVar15);
        local_4e = bVar3 + bVar15;
        pbVar2 = unaff_SI + -0x4f;
        bVar3 = *pbVar2;
        bVar8 = *pbVar2;
        *pbVar2 = bVar8 + bVar11 + bVar16;
        pbVar2 = unaff_SI + -0x78;
        *pbVar2 = *pbVar2 + in_CX + (CARRY1(bVar3, bVar11) || CARRY1(bVar8 + bVar11, bVar16));
        puStack34 = &stack0xfffe;
        process_struct_1040_b082(CONCAT22(&stack0xbf2a, &stack0xfffe), CONCAT22(in_AX, 0xfa1));
        uVar13 = (puVar18 >> 0x10);
        (puVar18 + 0x94) = 0;
        *puVar18 = &PTR_LOOP_1050_4e18;
        (puVar18 + 2) = &PTR_LOOP_1050_1040;
        ppVar17 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 3));
        struct_a = (ppVar17 >> 0x10);
        uVar6 = ppVar17;
        uVar13 = (puVar18 >> 0x10);
        (puVar18 + 0x94) = uVar6;
        (puVar18 + 0x96) = struct_a;
        process_struct_1000_179c(0x18, struct_a);
        struct_a_00 = (struct_a | uVar6);
        if (struct_a_00 == 0x0)
        {
            (puVar18 + 0x90) = 0;
        }
        else
        {
            process_struct_1040_a598(CONCAT22(struct_a, uVar6));
            uVar13 = (puVar18 >> 0x10);
            (puVar18 + 0x90) = uVar6;
            (puVar18 + 0x92) = extraout_DX;
            struct_a_00 = extraout_DX;
        }
        uVar13 = (puVar18 >> 0x10);
        (puVar18 + 0x90) = 7;
        iVar9 = (puVar18 + 0x90);
        uVar6 = iVar9 * 10 + 2;
        process_struct_1000_179c(uVar6, struct_a_00);
        if ((struct_a_00 | uVar6) == 0)
        {
            uVar5 = (puVar18 + 0x90);
            (uVar5 + 2) = 0;
        }
        else
        {
            CONCAT22(struct_a_00, uVar6) = iVar9;
            call_fn_ptr_1000_5586(0xa564, &PTR_LOOP_1050_1040, iVar9, 10, uVar6 + 2,
                                  struct_a_00);
            uVar5 = (puVar18 + 0x90);
            uVar13 = (uVar5 >> 0x10);
            iVar9 = uVar5;
            (iVar9 + 2) = uVar6 + 2;
            (iVar9 + 4) = struct_a_00;
        }
        uVar14 = (puVar18 >> 0x10);
        iVar10 = puVar18;
        uVar5 = (iVar10 + 0x90);
        (uVar5 + 6) = uVar19;
        uVar5 = (iVar10 + 0x90);
        (uVar5 + 10) = in_BX;
        uVar5 = (iVar10 + 0x90);
        (uVar5 + 0x12) = (iVar10 + 10);
        iVar9 = (iVar10 + 0x90);
        uVar13 = (iVar10 + 0x92);
        pass1_1010_debe((iVar10 + 0x94), (iVar9 + 10), CONCAT22(uVar13, iVar9 + 0x10),
                        CONCAT22(uVar13, iVar9 + 0xc), uVar19);
        return;
    }
    ppbVar1 = (unaff_SI + 9);}
    *ppbVar1 = unaff_SI + *ppbVar1;
    error_check_1000_17ce(param_1);
    return;
}




pub fn pass1_1040_48a0(param_1: *mut u16, param_2: *mut u8,param_3: u32, param_4: *mut u8)

{
    let mut u_var1: u32;
    let mut uVar2: i32;
    let struct_a: *mut astruct_199;
    let struct_a_00: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut unaff_SI: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let ppVar7: *mut pass1_struct_1;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1040_b082(param_1, CONCAT22(param_4, 0xfa1));
    uVar5 = (param_1 >> 0x10);
    iVar3 = param_1;
    (iVar3 + 0x94) = 0;
    unsafe{*param_1 = &PTR_LOOP_1050_4e18;}
    (iVar3 + 2) = &PTR_LOOP_1050_1040;
    ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 3));
    struct_a = (ppVar7 >> 0x10);
    uVar2 = ppVar7;
    (iVar3 + 0x94) = uVar2;
    (iVar3 + 0x96) = struct_a;
    process_struct_1000_179c(0x18, struct_a);
    struct_a_00 = (struct_a | uVar2);
    if (struct_a_00 == 0x0)
    {
        (iVar3 + 0x90) = 0;
    }
    else
    {
        process_struct_1040_a598(CONCAT22(struct_a, uVar2));
        (iVar3 + 0x90) = uVar2;
        (iVar3 + 0x92) = extraout_DX;
        struct_a_00 = extraout_DX;
    }
    (iVar3 + 0x90) = 7;
    iVar4 = (iVar3 + 0x90);
    uVar2 = iVar4 * 10 + 2;
    process_struct_1000_179c(uVar2, struct_a_00);
    _local_8 = CONCAT22(struct_a_00, uVar2);
    if ((struct_a_00 | uVar2) == 0)
    {
        uVar1 = (iVar3 + 0x90);
        (uVar1 + 2) = 0;
    }
    else
    {
        *_local_8 = iVar4;
        call_fn_ptr_1000_5586(0xa564, &PTR_LOOP_1050_1040, iVar4, 10, uVar2 + 2, struct_a_00);
        uVar1 = (iVar3 + 0x90);
        uVar6 = (uVar1 >> 0x10);
        iVar4 = uVar1;
        (iVar4 + 2) = uVar2 + 2;
        (iVar4 + 4) = struct_a_00;
    }
    uVar1 = (iVar3 + 0x90);
    (uVar1 + 6) = param_3;
    uVar1 = (iVar3 + 0x90);
    *(uVar1 + 10) = param_2;
    uVar1 = (iVar3 + 0x90);
    (uVar1 + 0x12) = (iVar3 + 10);
    iVar4 = (iVar3 + 0x90);
    uVar6 = (iVar3 + 0x92);
    pass1_1010_debe((iVar3 + 0x94), (iVar4 + 10), CONCAT22(uVar6, iVar4 + 0x10),
                    CONCAT22(uVar6, iVar4 + 0xc), param_3);
    return;
}


pub fn pass1_1040_4766(param_1: *mut u32)

{
    let ppcVar1: fn();

    unsafe{ppcVar1 = (*param_1 + 0x74);
    (**ppcVar1)();}
    return;
}


pub fn pass1_1040_4440(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_40e2(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_40e2(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x4466;
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}


pub fn pass1_1040_4068(param_1: *mut astruct_68,param_2: u32, param_3: *mut u8, param_4: *mut u8, param_3_00: *mut u8) -> i32

{
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;
    let in_string_1: *mut libc::c_char;

    process_struct_1040_7728(param_1, (&PTR_LOOP_1050_0000 + 1), param_2, 0xfb7, param_3_00);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x9a) = 0;
    param_1.field_0x0 = 0x4466;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    (iVar1 + 0x76) = 1;
    in_string_1 = CONCAT22(unaff_BP, 2);
    ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, in_string_1);
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0,
                                      CONCAT22((in_string_1 >> 0x10), 0x29));
    (iVar1 + 0x96) = ppVar3;
    (iVar1 + 0x98) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_3fd6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_39e2(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}




pub fn pass1_1040_3966(param_1: *mut astruct_68,param_2: u32, param_3: *mut u8, param_4: *mut u8, param_3_00: *mut u8) -> i32

{
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(param_1, (&PTR_LOOP_1050_0000 + 1), param_2,
                             (s_You_may_not_run_a_turn__The_game_1050_0172 + 0x13), param_3_00);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x96) = 0;
    (iVar1 + 0x9a) = 0;
    (iVar1 + 0x9c) = 0;
    (iVar1 + 0x9e) = 0;
    (iVar1 + 0xa0) = 0;
    (iVar1 + 0xa2) = 0;
    (iVar1 + 0xa4) = 5;
    param_1.field_0x0 = 0x3ffc;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, 0x3c));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}



pub fn pass1_1040_39e2(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x3ffc;
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}



pub fn pass1_1040_3a0e(param_1: *mut u8, param_2: *mut u8)

{
    let paVar1: *mut astruct_318;
    let BVar2: bool;

    BVar2 = 0;
    paVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x2b);
    pass1_1010_038e(paVar1, BVar2);
    destroy_win_1040_7b98(param_1, param_2);
    return;
}


pub fn pass1_1040_38d4(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_3506(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1040_3410(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_2f06(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}





pub fn pass1_1040_2e82(param_1: *mut astruct_44,param_2: u32, param_3: *mut u8, param_4: *mut u8, param_5: *mut u8) -> i32

{
    let pbVar1: *mut byte;
    let pcVar2: *mut libc::c_char;
    let mut bVar3: u8;
    let mut bVar4: u8;
    let mut cVar5: u8;
    let mut uVar6: i32;
    let mut in_CX: u16;
    let mut bVar7: u8;
    let mut bVar8: u8;
    let mut in_DX: i32;
    let mut bVar9: u8;
    let mut in_BX: i32;
    let mut iVar10: i32;
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

    puVar11 = &stack0xfffe;
    cVar5 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar11 = puVar11 + -1;
        unsafe{*puVar11 = *unaff_BP;}
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    bVar7 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar7;
    bVar9 = (in_DX + 1 >> 8);
    bVar3 = bVar9 + bVar7;
    bVar13 = CARRY1(bVar9, bVar7) || CARRY1(bVar3, in_CF);
    uVar6 = in_DX + 1 & 0xff;
    pbVar1 = unaff_SI + in_BX;
    bVar8 = uVar6;
    unsafe{bVar9 = *pbVar1;
    bVar4 = *pbVar1 - bVar8;
    bVar14 = *pbVar1 < bVar8 || bVar4 < bVar13;
    *pbVar1 = bVar4 - bVar13;
    if (*pbVar1 != 0 && (SBORROW1(bVar9, bVar8) != SBORROW1(bVar4, bVar13)) == *pbVar1 < '\0')
    {
        pbVar1 = unaff_SI;
        bVar13 = CARRY1(*pbVar1, bVar7) || CARRY1(*pbVar1 + bVar7, bVar14);
        *pbVar1 = *pbVar1 + bVar7 + bVar14;
        pbVar1 = unaff_SI + in_BX + 0x2d;
        bVar9 = *pbVar1 + in_CX;
        bVar14 = CARRY1(*pbVar1, in_CX) || CARRY1(bVar9, bVar13);
        *pbVar1 = bVar9 + bVar13;
        pbVar1 = unaff_SI + -0x4f;
        bVar13 = CARRY1(*pbVar1, bVar7) || CARRY1(*pbVar1 + bVar7, bVar14);
        *pbVar1 = *pbVar1 + bVar7 + bVar14;
        pbVar1 = unaff_SI + 0x402d;
        bVar9 = *pbVar1;
        bVar7 = (in_CX >> 8);
        bVar4 = *pbVar1 + bVar7;
        *pbVar1 = bVar4 + bVar13;
        pcVar2 = (unaff_DI + -0x75);
        *pcVar2 = *pcVar2 + bVar8 + (CARRY1(bVar9, bVar7) || CARRY1(bVar4, bVar13));
        _in(uVar6 | (bVar3 + in_CF) << 8);
        process_struct_1040_7728(param_1, (&PTR_LOOP_1050_0000 + 1), param_2,
                                 (s_You_may_not_run_a_turn__The_game_1050_0172 + 0xe), param_5);
        uVar12 = (param_1 >> 0x10);
        iVar10 = param_1;
        (iVar10 + 0x8e) = 0;
        (iVar10 + 0x90) = 0;
        (iVar10 + 0x92) = 0;
        (iVar10 + 0x94) = 0;
        (iVar10 + 0x96) = 0;
        param_1.ptr_a_lo = 0x3436;
        (iVar10 + 2) = &PTR_LOOP_1050_1040;
        ppVar15 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_DI, 0x3c));
        (iVar10 + 0x96) = ppVar15;
        (iVar10 + 0x98) = (ppVar15 >> 0x10);
        return;
    }
    if (*pbVar1 != 0)
    {
        error_check_1000_17ce(param_1);
    }}
    return;
}


pub fn pass1_1040_2dac(param_1: *mut u8)

{
    let mut u_var1: u32;
    let mut uVar2: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = (param_1 + 0x90);
    uVar2 = pass1_1030_73a8((uVar1 + 6));
    local_a = 0;
    while (local_a < 5)
    {
        pass1_1028_4ab2(uVar2, (&PTR_LOOP_1050_5d04 + local_a * 0xc),
                        (local_a * 0xc + 0x5d02));
        local_a = local_a + 1;
    }
    return;
}

pub fn pass1_1040_2e00(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_2a22(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_288e(param_1: *mut u8)

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
    puVar5 = (uVar3 + 0x24);
    unsafe{ppcVar2 = (*puVar5 + 0x14);}
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
            unsafe{ppcVar2 = *puVar4;
            ppcVar2();}
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

pub fn pass1_1040_2930(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_2464(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1040_2358(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_205e(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_23ea(param_1: *mut astruct_68,param_2: u32, param_3: *mut u8, param_4: *mut u8, param_3_00: *mut u8) -> i32

{
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut in_stack_0000fff8: u32;
    let mut uVar5: u16;

    uVar5 = (in_stack_0000fff8 >> 0x10);
    process_struct_1040_7728(param_1, (s_New_failed_in_Op__Op__Simulator_1050_0097 + 3), param_2, 0xfbd,
                             param_3_00);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    (iVar2 + 0x8e) = 0;
    (iVar2 + 0x92) = 0;
    (iVar2 + 0x94) = 0;
    param_1.field_0x0 = (s_fem94_wav_1050_2950 + 6);
    (iVar2 + 2) = &PTR_LOOP_1050_1040;
    (iVar2 + 0x8a) = 0x26;
    ppVar4 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uVar5, 6));
    (iVar2 + 0x8e) = ppVar4;
    (iVar2 + 0x90) = (ppVar4 >> 0x10);
    uVar1 = (iVar2 + 0x8e);
    (iVar2 + 0x92) = (uVar1 + 0x28);
    return;
}



pub fn pass1_1040_1e80(param_1: i32, param_2: *mut u8, param_3: *mut u8, param_2_00: *mut u8)

{
    let mut local_6: u32;

    local_6 = 0;
    if (param_2_00._2_2_ == 0xe4)
    {
        pass1_1008_a9ec((param_1 + 0x92));
    }
    else
    {
        local_6 = post_win_msg_1040_7b3c(param_1, CONCAT22(param_3, param_2), param_2_00);
    }
    return local_6;
}

pub fn pass1_1040_1ec8(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_1d24(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Instruction at (ram,0x10401f52) overlaps instruction at (ram,0x10401f51)
//



pub fn pass1_1040_1eee(param_1: *mut astruct_44, param_2: *mut u8)

{
    let pcVar1: *mut libc::c_char;
    let pbVar2: *mut byte;
    let puVar3: *mut u32;
    let puVar4: *mut u16;
    let piVar5: *mut i32;
    let mut bVar6: u8;
    let mut cVar7: u8;
    let mut uVar8: i32;
    let mut uVar9: i32;
    let mut uVar10: i32;
    let mut cVar11: u8;
    let mut bVar12: u8;
    let mut bVar13: u8;
    let mut uVar14: i32;
    let mut in_EAX: u32;
    let mut bVar16: u8;
    let mut uVar15: u32;
    let mut in_CX: u16;
    let mut in_DX: i32;
    let mut iVar17: i32;
    let mut bVar18: u8;
    let mut extraout_DX: u16;
    let mut bVar19: u8;
    let in_BX: *mut u16;
    let mut bVar21: u8;
    let puVar20: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut byte;
    let unaff_DI: *mut byte;
    let mut unaff_ES: u16;
    let mut uVar22: u16;
    let mut uVar23: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar24: bool;
    let mut bVar25: bool;
    let ppVar26: *mut pass1_struct_1;
    let mut in_stack_0000407d: u8;
    let mut in_stack_0000407f: u8;
    let in_stack_0000bfac: *mut byte;
    let puStack34: *mut u8;
    let mut uStack26: u16;
    let mut uStack24: u16;
    let mut uStack22: u32;
    let mut uStack18: u32;
    let mut uStack14: u16;
    let mut uStack12: u16;
    let mut iStack10: i32;
    let mut uStack8: u16;
    let mut uStack6: u32;

    puStack34 = &stack0xfffe;
    puVar20 = &stack0xfffe;
    cVar11 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar20 = puVar20 + -1;
        unsafe{*puVar20 = *unaff_BP;}
        cVar11 = cVar11 + -1;
        '\0' < cVar11
    } {}
    pcVar1 = &stack0xfffe + unaff_SI;
    bVar21 = (in_BX >> 8);
    unsafe{*pcVar1 = *pcVar1 + bVar21 + in_CF;
    pbVar2 = (in_BX + unaff_SI);
    bVar16 = in_DX;
    *pbVar2 = *pbVar2 | bVar16;
    uVar9 = in_EAX + 0xefc0;
    pcVar1 = (in_BX + unaff_DI + 0x40);
    cVar11 = *pcVar1;
    *pcVar1 = *pcVar1 >> 0x10;}
    bVar25 = (cVar11 >> 0xf & 1) != 0;
    bVar24 = PTR_LOOP_1050_1040 < 0x7a;
    bVar12 = PTR_LOOP_1050_1040 + 0x86;
    PTR_LOOP_1050_1040._0_1_ = bVar12 - bVar25;
    uVar8 = (bVar24 || bVar12 < bVar25);
    uVar10 = in_EAX + 0xdf80;
    bVar25 = uVar9 < &PTR_LOOP_1050_1040 || uVar10 < uVar8;
    uVar14 = uVar10 - uVar8;
    bVar18 = (in_DX >> 8);
    bVar12 = uVar14;
    if (uVar14 != 0 && (SBORROW2(uVar9, 0x1040) != SBORROW2(uVar10, uVar8)) == uVar14 < 0)
    {
        pcVar1 = (in_BX + unaff_SI);
        unsafe{
        *pcVar1 = (*pcVar1 - (bVar16 + bVar21 + bVar25)) -
                  (CARRY1(bVar16, bVar21) || CARRY1(bVar16 + bVar21, bVar25));
        *unaff_DI = *unaff_SI;}
        PTR_LOOP_1050_1038._0_1_ = bVar12;
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    pbVar2 = &stack0x4079 + unaff_SI;
    unsafe{bVar24 = CARRY1(*pbVar2, bVar18) || CARRY1(*pbVar2 + bVar18, bVar25);
    *pbVar2 = *pbVar2 + bVar18 + bVar25;
    pbVar2 = unaff_SI;
    bVar19 = in_BX;
    bVar6 = *pbVar2;
    bVar13 = *pbVar2 + bVar19;
    bVar25 = CARRY1(*pbVar2, bVar19) || CARRY1(bVar13, bVar24);
    *pbVar2 = bVar13 + bVar24;
    uVar22 = (param_1 >> 0x10);
    puVar20 = param_1;
    if (*pbVar2 == 0 || (SCARRY1(bVar6, bVar19) != SCARRY1(bVar13, bVar24)) != *pbVar2 < '\0')
    {
        bVar16 = (uVar14 >> 8);
        bVar6 = bVar16 + bVar12;
        bVar13 = (uVar14 & 0xff) + 1;
        pbVar2 = unaff_SI;
        *pbVar2 = *pbVar2 + in_CX +
                  (CARRY1(in_stack_0000407d, bVar13) ||
                   CARRY1(in_stack_0000407d + bVar13, CARRY1(bVar16, bVar12) || CARRY1(bVar6, bVar25)));
        pcVar1 = (in_BX + unaff_SI + 0x10);
        *pcVar1 = *pcVar1 + 'T';
        pbVar2 = (in_BX + unaff_SI + 0x10);
        bVar12 = *pbVar2;
        bVar16 = *pbVar2;
        *pbVar2 = *pbVar2 + 0x5e;
        if (*pbVar2 == 0 || SCARRY1(bVar16, '^') != *pbVar2 < '\0')
        {
            pcVar1 = (in_BX + unaff_SI);
            *pcVar1 = *pcVar1 + bVar13;
            return;
        }
        pbVar2 = (in_BX + unaff_SI + -0x7f);
        bVar16 = *pbVar2;
        bVar13 = *pbVar2;
        *pbVar2 = bVar13 + bVar19 + (0xa1 < bVar12);
        uVar15 = in_EAX & 0xffff0000 | ((uVar14 & 0xff | (bVar6 + bVar25) << 8) + 2);
        puVar3 = (in_BX + unaff_SI + 0x10);
        uVar8 = *puVar3;
        *puVar3 = *puVar3 + 0x60ea;
        pcVar1 = (in_BX + unaff_SI);
        *pcVar1 = (*pcVar1 - (in_DX & 0xff)) - (0x9f15 < uVar8);
        iVar17 = (in_DX & 0xff |
                  (bVar18 + bVar21 +
                               (CARRY1(in_stack_0000407f, bVar18) ||
                                CARRY1(in_stack_0000407f + bVar18,
                                       CARRY1(bVar16, bVar19) || CARRY1(bVar13 + bVar19, 0xa1 < bVar12))))
                      << 8) -
                 1;
        pcVar1 = (in_BX + unaff_SI + 0x10);
        *pcVar1 = *pcVar1 + 'f';
        pbVar2 = (in_BX + unaff_SI + 0x10);
        bVar12 = *pbVar2;
        *pbVar2 = *pbVar2 - 0x22;
        unaff_DI = in_stack_0000bfac;
        puStack34 = &stack0xfffe;
        if (-1 < *pbVar2)
        {
            bVar16 = (in_CX >> 8);
            bVar18 = (iVar17 >> 8);
            pcVar1 = (in_BX + unaff_SI);
            *pcVar1 = (*pcVar1 - iVar17) -
                      (CARRY1(bVar18, bVar16) || CARRY1(bVar18 + bVar16, 0x21 < bVar12));
            loop
            {
                // WARNING: Do nothing block with infinite loop
            } 
        }
    }
    else
    {
        bVar25 = CARRY1(bVar18, bVar21) || CARRY1(bVar18 + bVar21, bVar25);
        pcVar1 = (in_BX + unaff_SI);
        cVar11 = *pcVar1;
        cVar7 = *pcVar1;
        *pcVar1 = (cVar7 - bVar16) - bVar25;
        if (*pcVar1 == '\0' ||
            (SBORROW1(cVar11, bVar16) != SBORROW1(cVar7 - bVar16, bVar25)) != *pcVar1 < '\0')
        {
            if (*pcVar1 != '\0')
            {
                error_check_1000_17ce(param_1);
            }
            return;
        }
        puVar4 = in_BX + 0x2b;
        *puVar4 = *puVar4 + bVar16;
        puStack34 = &stack0xfffe;
        process_struct_1040_7728(param_1, (&PTR_LOOP_1050_0000 + 1), 0, 0xfcf, param_2);
        uVar15 = 0;
        (puVar20 + 0x47) = 0;
        in_BX = puVar20;
        unaff_ES = uVar22;
    }}
    (in_BX + 0x51) = uVar15;
    (in_BX + 0x53) = uVar15;
    unsafe{*in_BX = (s_alarm_m_1050_2377 + 7);}
    in_BX[1] = &PTR_LOOP_1050_1040;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1cc);
    puVar20[0x47] = uVar15;
    puVar20[0x48] = extraout_DX;
    uStack6 = process_struct_1008_4772(CONCAT22(extraout_DX, puVar20[0x47]));
    ppVar26 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_DI, 0x48));
    uStack8 = (ppVar26 >> 0x10);
    iStack10 = ppVar26;
    uStack12 = (iStack10 + 10);
    uStack14 = (iStack10 + 0xc);
    uVar23 = (uStack6 >> 0x10);
    uStack22 = CONCAT22((uStack6 + 8) + 10, 10);
    uStack18 = CONCAT22(0x1d6, (uStack6 + 4) + -10);
    (puVar20 + 0x49) = uStack22;
    (puVar20 + 0x4b) = uStack18;
    (puVar20 + 0x4d) = uStack22;
    (puVar20 + 0x4f) = uStack18;
    piVar5 = puVar20 + 0x4e;
    unsafe{*piVar5 = *piVar5 + 0x14;}
    ppVar26 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(puVar20 + 0x53, 0x2b));
    uStack24 = (ppVar26 >> 0x10);
    uStack26 = SUB42(ppVar26, 0);
    pass1_1010_0538();
    return;
}



pub fn pass1_1040_1d24(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = (s_526_bmp_1050_1ee7 + 7);
    (param_1 + 2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}


pub fn pass1_1040_1c22(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_1876(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_178a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_1290(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_11ac(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_0e86(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_0d8a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_0c54(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_0b6a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1040_073a(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1038_ee48(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_ebd6(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_eb0c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_e9ec(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_e9ec(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xeb32;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}


pub fn pass1_1038_e908(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_e6f0(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1038_e6f0(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xe92e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}


pub fn pass1_1038_e608(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_e308(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_e23e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_e16e(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_e0ae(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_d7d0(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1038_df5c(param_1: *mut u8)

{
    let mut uVar1: u16;
    let puVar2: *mut u8;

    uVar1 = (param_1 >> 0x10);
    pass1_1010_038e((param_1 + 0x92), 1);
    puVar2 = pass1_1038_af40(_g_astruct_112_a, *(param_1 + 8), 0x20);
    return puVar2;
}



pub fn pass1_1038_d6c4(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_d276(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_ceda(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_cd5c(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1038_cc74(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_cb30(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1038_ca46(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_c80a(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1038_c80a(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xca6c;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}


pub fn pass1_1038_c726(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1038_c4fe(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn set_focus_1038_c558(param_1: *mut astruct_20)

{
    let mut uVar1: u16;

    win_gui_func_1040_78e2(param_1);
    move_window_1040_826c(param_1, 0xffffffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}



pub fn pass1_1038_c52a(param_1: *mut u8, param_2: *mut u8)

{
    let local_DXAX_18: *mut pass1_struct_1;
    let BVar1: bool;

    if (param_2._2_2_ == 0)
    {
        BVar1 = 0;
        local_DXAX_18 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x2b);
        pass1_1010_038e(local_DXAX_18, BVar1);
    }
    destroy_win_1040_7b98(param_1, param_2);
    return;
}


pub fn pass1_1038_c4fe(param_1: *mut astruct_599)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xc74c;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}