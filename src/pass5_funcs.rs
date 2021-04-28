pub fn pass1_1030_cac2(param_1: *mut astruct_44)

{
    let mut uVar1: u16;
    let mut uVar2: u32;
    let puVar3: *mut u32;
    let ppcVar4: fn();
    let mut uVar5: u32;
    let uVar6: u8;
    let paVar7: *mut astruct_493;
    let paVar8: *mut astruct_493;
    let extraout_var: u32;
    let puVar9: *mut u32;
    let mut uVar10: u32;
    let mut extraout_DX: u16;
    let mut uVar11: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut uVar12: i32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2c: u16;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    uVar12 = (param_1 >> 0x10);
    if (((param_1 + 0x12) == 5) && (PTR_LOOP_1050_5812 == 0x0))
    {
        PTR_LOOP_1050_5812 = (&PTR_LOOP_1050_0000 + 1);
        uVar11 = extraout_DX;
        uVar6 = pass1_1028_b58e((param_1 & 0xffff | uVar12 << 0x10));
        uVar2 = (CONCAT31(extraout_var, uVar6) + 0x2e);
        uVar2 = (uVar2 + 0x10);
        paVar7 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
        puVar3 = (paVar7 + 1);
        unsafe{ppcVar4 = (*puVar3 + 0x10)};
        puVar9 = puVar3;
        (**ppcVar4)(&PTR_LOOP_1050_1028, puVar3, &paVar7[1].field_0x2);
        uVar5 = puVar9 & 0xffff | extraout_DX_00 << 0x10;
        local_1c = 0;
        local_1e = pass1_1030_d144(param_1);
        local_22 = 0;
        while ((local_22 < uVar5 && (local_1e != 0)))
        {
            unsafe{ppcVar4 = (*puVar3 + 4)};
            uVar10 = uVar5;
            (**ppcVar4)(&PTR_LOOP_1050_1028, puVar3, (puVar3 >> 0x10), local_22,
                        (local_22 >> 0x10));
            uVar12 = extraout_DX_01 | uVar10;
            if (uVar12 != 0)
            {
                paVar8 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar10, extraout_DX_01);
                uVar1 = &paVar8.field_0xc;
                if ((0 < uVar1) && (!SBORROW2(uVar1, 1)))
                {
                    if (uVar1 != 3 && 0 < (uVar1 - 2))
                    {
                        if (uVar1 != 4) {}
                          // goto LAB_1030_cbbc;
                        local_1c = local_1c + 1;
                    }
                    pass1_1030_6e9c(CONCAT22(uVar11, paVar7), 1, uVar1);
                    pass1_1030_d180(param_1, uVar1);
                    local_1e = local_1e - 1;
                }
            }
// LAB_1030_cbbc:
            local_22 = local_22 + 1;
        }
        while (local_1c < 4)
        {
            pass1_1030_d180(param_1, 4);
            local_1c = local_1c + 1;
        }
    }
    return;
}

pub fn pass1_1030_cbf0(param_1: i32, param_2: u16, param_3: i32)

{
    let local_BX_39: *mut astruct_984;
    let local_4: *mut astruct_985;

    local_4 = 0x0;
    loop
    {
        if (9 < local_4)
        {
            return 0;
        }
        local_BX_39 = (local_4 * 0xc + param_1);
        if ((local_BX_39.field_0x24 == param_3) && (local_BX_39.field_0x26 == 3)) {
            break;}
        local_4 = &local_4.field_0x1;
    }
    local_BX_39.field_0x26 = 0;
    local_BX_39.field_0x28 = 0;
    return 1;
}


pub fn pass1_1030_cc44(param_1: *mut astruct_987, param_2: u16, param_3: i32,param_4: u32, param_5: u16) -> i32

{
    let ppcVar1: fn();
    let uVar2: u8;
    let puVar3: *mut u8;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let paVar6: *mut astruct_493;
    let mut uVar7: u32;
    let extraout_var: u32;
    let puVar8: *mut u8;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let local_BX_202: *mut astruct_986;
    let uVar9: u8;
    let mut local_3a: u32;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: [u8;8];
    let mut local_2a: u32;
    let mut local_26: u32;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let local_a: *mut astruct_988;
    let mut local_8: u32;
    let mut local_4: u16;

    local_4 = 0;
    local_8 = (param_4 + 4);
    local_a = 0x0;
    loop
    {
        if (9 < local_a)
        {
            return;
        }
        local_BX_202 = (param_1 + local_a * 0xc);
        if (((local_BX_202.field_0x28 == local_8) && (local_BX_202.field_0x2a == local_8._2_2_)) && (local_BX_202.field_0x24 == param_5))
        {
            if (local_BX_202.field_0x26 == 4)
            {
                uVar2 = pass1_1028_b58e(CONCAT22(param_2, param_1));
                local_e = CONCAT31(extraout_var, uVar2);
                local_c = local_8._2_2_;
                pass1_1030_6e9c(CONCAT13((local_8._2_2_ >> 8), CONCAT12(local_8._2_2_, local_e)), 1, local_BX_202.field_0x24);
                local_BX_202.field_0x20 = 0;
                local_BX_202.field_0x24 = 0;
                local_BX_202.field_0x26 = 0;
                local_2a = 0;
                local_12 = 0;
                _DAT_0000_0006 = param_5;
                uRam0000000a = 1;
                uVar4 = switch_statement_1020_c3b4(param_5);
                (local_12 + 0xc) = uVar4;
                puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 4);
                uVar5 = puVar8;
                uVar4 = local_8._2_2_;
                local_16 = uVar5;
                local_14 = local_8._2_2_;
                paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
                uVar9 = 0x38;
                local_1a = paVar6;
                local_18 = local_8._2_2_;
                pass1_1038_4d6e(CONCAT22(local_8._2_2_, paVar6), CONCAT22(uVar4, uVar5));
                local_1e = CONCAT22(extraout_DX_00, paVar6);
                ppcVar1 = (local_1e + 0x10);
                (**ppcVar1)(&PTR_LOOP_1050_1038, paVar6, extraout_DX_00);
                local_22 = CONCAT22(extraout_DX_01, paVar6);
                local_26 = 0;
                while (local_26 < local_22)
                {
                    uVar7 = local_22;
                    pass1_1030_1d7c(local_1e, (local_1e >> 0x10), local_26,
                                    (local_26 >> 0x10));
                    if ((extraout_DX | uVar7) != 0)
                    {
                        puVar3 = local_32;
                        ppcVar1 = ((uVar7 & 0xffff | extraout_DX << 0x10) +
                                            0x40);
                        (**ppcVar1)(0x38, uVar7, extraout_DX, puVar3);
                        if (puVar3 == 0x0)
                        {
                            uVar9 = 0x28;
                            pass1_1028_6408(uVar7 & 0xffff | extraout_DX << 0x10, local_12);
                            break;
                        }
                    }
                    local_26 = local_26 + 1;
                }
                local_2a = local_1e;
                if (local_1e != 0)
                {
                    ppcVar1 = local_1e;
                    (**ppcVar1)(uVar9, local_1e, (local_1e >> 0x10), 1);
                }
            }
            else
            {
                (param_1 + local_a * 0xc + 0x26) = 0;
                (param_1 + local_a * 0xc + 0x28) = 0;
            }
            local_4 = local_4 + 1;
            param_3 = param_3 + -1;
            if (param_3 == 0)
            {
                return;
            }
        }
        local_a = &local_a.field_0x1;
    } 
}

pub fn pass1_1030_cde8(param_1: i32, param_2: u16, param_3: i32)

{
    let mut iVar1: i32;
    let mut local_4: u16;

    local_4 = 0;
    loop
    {
        if (9 < local_4)
        {
            return 0xffff;
        }
        iVar1 = local_4 * 0xc + param_1;
        if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0)) {
            break;}
        local_4 = local_4 + 1;
    }
    return local_4;
}



pub fn pass1_1030_ce2e(param_1: i32, param_2: u16, param_3: i32) -> i32

{
    let mut iVar1: i32;
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < 10)
    {
        iVar1 = local_6 * 0xc + param_1;
        if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0))
        {
            local_6 = local_6 & 0xffff | (local_6._2_2_ + 1) << 0x10;
        }
        local_6 = local_6 & 0xffff0000 | (local_6 + 1);
    }
    return local_6._2_2_;
}

pub fn pass1_1030_ce72(param_1: u32, param_2: i32,param_3: u32, param_4: i32)

{
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    uVar1 = (param_3 + 4);
    local_a = 0;
    loop
    {
        if (9 < local_a)
        {
            return;
        }
        iVar2 = local_a * 0xc + param_1;
        if (((iVar2 + 0x24) == param_4) && ((iVar2 + 0x28) == 0))
        {
            (iVar2 + 0x28) = uVar1;
            if (param_4 == 4)
            {
                (iVar2 + 0x26) = 2;
            }
            else
            {
                (param_1 + local_a * 0xc + 0x26) = 1;
            }
            param_2 = param_2 + -1;
            if (param_2 == 0)
            {
                return;
            }
        }
        local_a = local_a + 1;
    } 
}

pub fn pass1_1030_cef8(param_1: *mut astruct_989,param_2: u32, param_3: u16, param_4: *mut astruct_990) -> i32

{
    let mut uVar1: u16;
    let local_BX_20: *mut astruct_989;
    let local_ES_20: *mut astruct_989;
    let mut uVar2: u16;

    local_ES_20 = (param_1 >> 0x10);
    local_BX_20 = param_1;
    (local_BX_20 + param_4 * 0xc + 0x26) = param_3;
    uVar2 = (param_2 >> 0x10);
    uVar1 = (param_2 + 6);
    (local_BX_20 + param_4 * 0xc + 0x28) = (param_2 + 4);
    (local_BX_20 + param_4 * 0xc + 0x2a) = uVar1;
    return;
}

pub fn pass1_1030_cf3a(param_1: u32, param_2: i32)

{
    let mut local_4: u16;

    local_4 = 0;
    loop
    {
        if (9 < local_4)
        {
            return 0;
        } 
        if ((param_1 + local_4 * 0xc + 0x24) == param_2) {
            break;}
        local_4 = local_4 + 1;
    }
    return 1;
}

pub fn pass1_1030_cf78(param_1: *mut astruct_44, param_3: i32)

{
    let uVar1: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let local_BX_40: *mut astruct_991;
    let mut uVar2: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: *mut astruct_992;

    local_4 = 0x0;
    loop
    {
        if (9 < local_4)
        {
            return;
        }
        uVar2 = (param_1 >> 0x10);
        if ((param_1 + local_4 * 0xc + 0x24) == param_3) {
            break;}
        local_4 = &local_4.field_0x1;
    }
    uVar1 = pass1_1028_b58e(param_1);
    if (param_3 == 5)
    {
        pass1_1038_4900(*(CONCAT31(extraout_var, uVar1) + 0x2e));
    }
    else
    {
        pass1_1030_6e9c(CONCAT31(extraout_var, uVar1) & 0xffff | in_DX << 0x10, 1, param_3);
    }
    local_BX_40 = (local_4 * 0xc + param_1);
    local_BX_40.field_0x20 = 0;
    local_BX_40.field_0x24 = 0;
    local_BX_40.field_0x26 = 0;
    return;
}

pub fn pass1_1030_d00c(param_1: *mut astruct_44, param_2: *mut astruct_44, param_3: i32)

{
    let uVar1: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let local_BX_40: *mut astruct_993;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: *mut astruct_994;

    local_4 = 0x0;
    loop
    {
        if (9 < local_4)
        {
            return;
        }
        if (((&param_1.ptr_a_lo + local_4 * 6)[0x13] == 0) &&
            ((&param_1.ptr_a_lo + local_4 * 6)[0x12] == param_3)) {
            break;}
        local_4 = &local_4.field_0x1;
    }
    uVar1 = pass1_1028_b58e(CONCAT22(param_2, param_1));
    if (param_3 == 5)
    {
        pass1_1038_4900(*(CONCAT31(extraout_var, uVar1) + 0x2e));
    }
    else
    {
        pass1_1030_6e9c(CONCAT31(extraout_var, uVar1) & 0xffff | in_DX << 0x10, 1, param_3);
    }
    local_BX_40 = (&param_1.ptr_a_lo + local_4 * 6);
    local_BX_40.field_0x20 = 0;
    local_BX_40.field_0x24 = 0;
    local_BX_40.field_0x26 = 0;
    return;
}

pub fn pass1_1030_d0a8(param_1: *mut astruct_44)

{
    let mut uVar1: u16;
    let mut uVar2: i32;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x98);
    pass1_1030_d56a((param_1 & 0xffff | uVar2 << 0x10));
    return uVar1;
}



pub fn pass1_1030_d0c6(param_1: u32) -> i32

{
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < 10)
    {
        if ((param_1 + local_6 * 0xc + 0x20) != 0)
        {
            local_6 = local_6 & 0xffff | (local_6._2_2_ + 1) << 0x10;
        }
        local_6 = local_6 & 0xffff0000 | (local_6 + 1);
    }
    return local_6._2_2_;
}



pub fn pass1_1030_d102(param_1: *mut astruct_997, param_2: u16) -> i32

{
    let local_BX_26: *mut astruct_996;
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < 10)
    {
        local_BX_26 = (param_1 + local_6 * 0xc);
        if ((local_BX_26.field_0x20 != 0) && (local_BX_26.field_0x26 != 0))
        {
            local_6 = local_6 & 0xffff | (local_6._2_2_ + 1) << 0x10;
        }
        local_6 = local_6 & 0xffff0000 | (local_6 + 1);
    }
    return local_6._2_2_;
}



pub fn pass1_1030_d144(param_1: *mut astruct_44) -> i32

{
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < 10)
    {
        if ((param_1 + local_6 * 0xc + 0x20) == 0)
        {
            local_6 = local_6 & 0xffff | (local_6._2_2_ + 1) << 0x10;
        }
        local_6 = local_6 & 0xffff0000 | (local_6 + 1);
    }
    return local_6._2_2_;
}



pub fn pass1_1030_d180(param_1: *mut astruct_44, param_2: u16)

{
    let uVar1: u8;
    let extraout_var: u32;
    let paVar2: *mut astruct_1002;
    let iVar5: *mut astruct_44;
    let local_BX_95: *mut astruct_1001;
    let local_ES_53: *mut astruct_44;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;
    let temp_5f4875ce46: *mut astruct_1002;
    let temp_5f2362e605: *mut astruct_1000;

    local_4 = 0;
    loop
    {
        if (9 < local_4)
        {
            return;
        }
        local_ES_53 = (param_1 >> 0x10);
        iVar5 = param_1;
        if (((&iVar5.field_0x22 + local_4 * 0xc) | (&iVar5.field_0x20)[local_4 * 6]) == 0) {
            break;}
        local_4 = local_4 + 1;
    }
    temp_5f2362e605 = *_PTR_LOOP_1050_65e2;
    temp_5f4875ce46 = (_PTR_LOOP_1050_65e2 + 2);
    paVar2 = temp_5f4875ce46 + (0xff37 < temp_5f2362e605);
    local_BX_95 = (&iVar5.ptr_a_lo + local_4 * 6);
    local_BX_95.field_0x20 = temp_5f2362e605 + 1;
    local_BX_95.field_0x22 = paVar2;
    local_BX_95.field_0x24 = param_2;
    pass1_1030_d340(iVar5, local_ES_53,
                    param_1 & 0xffff0000 | ZEXT24(&local_BX_95.field_0x20));
    uVar1 = pass1_1028_b58e(param_1);
    if (param_2 == 5)
    {
        pass1_1038_48e0(*(CONCAT31(extraout_var, uVar1) + 0x2e), 1);
        return;
    }
    pass1_1030_7c50(CONCAT31(extraout_var, uVar1) & 0xffff | ZEXT24(paVar2) << 0x10, 1, param_2);
    return;
}

pub fn pass1_1030_d230(param_1: *mut u8)

{
    let local_4: *mut astruct_1003;

    local_4 = 0x0;
    loop
    {
        if (9 < local_4)
        {
            return 1;
        }
        if ((param_1 + local_4 * 0xc + 0x20) == 0) {
            break;}
        local_4 = &local_4.field_0x1;
    }
    return 0;
}




pub fn pass1_1030_d26c(param_1: *mut astruct_44)

{
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let uVar3: u8;
    let mut uVar4: u32;
    let extraout_var: u32;
    let mut in_DX: i32;
    let local_BX_112: *mut astruct_1004;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar2 = *_PTR_LOOP_1050_65e2;
    local_8 = 0;
    while (local_8 < 10)
    {
        local_BX_112 = (local_8 * 0xc + param_1);
        let pu_var1_val = unsafe{*puVar1};
        if (((&local_BX_112.field_0x22 | local_BX_112.field_0x20) != 0) &&
            (puVar1 = &local_BX_112.field_0x20, pu_var1_val < uVar2 || pu_var1_val == uVar2))
        {
            uVar4 = uVar2;
            pass1_1030_d3b2(param_1, param_1._2_2_, local_8);
            if (uVar4 == 0)
            {
                uVar3 = pass1_1028_b58e(param_1);
                _local_c = CONCAT31(extraout_var, uVar3) & 0xffff;
                if (local_BX_112.field_0x24 == 5)
                {
                    pass1_1038_4900(*(_local_c + 0x2e));
                }
                else
                {
                    pass1_1030_6e9c(_local_c | in_DX << 0x10, 1,
                                    (param_1 + local_8 * 0xc + 0x24));
                }
                param_1._0_2_ = local_8 * 0xc + param_1;
                (param_1 + 0x20) = 0;
                (param_1 + 0x24) = 0;
                (param_1 + 0x26) = 0;
            }
        }
        local_8 = local_8 + 1;
    }
    return;
}

pub fn pass1_1030_d340(param_1: u16, param_2: u16,param_1_00: u32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut uvar3: u16;

    uVar3 = (param_1_00 >> 0x10);
    iVar2 = param_1_00;
    iVar1 = (iVar2 + 4);
    if (((0 < iVar1) && (!SBORROW2(iVar1, 1))) && ((iVar1 == 4 || iVar1 + -1 < 3 || (iVar1 == 0xc))))
    {
        (iVar2 + 6) = 0;
        return;
    }
    (iVar2 + 6) = 1;
    return;
}

pub fn pass1_1030_d36e(param_1: u32, param_2: *mut astruct_1005)

{
    let local_4: *mut astruct_1006;

    local_4 = 0x0;
    loop
    {
        if (9 < local_4)
        {
            return 0;
        }
        if ((local_4 != param_2) &&
            ((param_1 + local_4 * 0xc + 0x24) == 8)) {
            break;}
        local_4 = &local_4.field_0x1;
    }
    return 1;
}




pub fn pass1_1030_d3b2(param_1: i32, param_2: u16, param_2_00: *mut astruct_1005)

{
    let mut iVar1: i32;
    let paVar2: *mut astruct_1115;
    let ppcVar3: fn();
    let mut bVar4: bool;
    let uVar5: u8;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: i32;
    let extraout_var: u32;
    let puVar10: *mut u8;
    let mut uVar11: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: u16;
    let mut extraout_DX_04: u16;
    let mut extraout_DX_05: i32;
    let uVar12: u8;
    let mut uVar13: u32;
    let uVar14: u8;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = pass1_1028_b58e(CONCAT22(param_2, param_1));
    paVar2 = (CONCAT31(extraout_var, uVar5) + 0x2e);
    uVar6 = pass1_1030_d36e(CONCAT22(param_2, param_1), param_2_00);
    if (uVar6 == 0)
    {
        puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        uVar7 = SUB42(puVar10, 0);
        pass1_1038_4d6e(paVar2, puVar10 & 0xffff | in_DX << 0x10);
        _local_1a = CONCAT22(extraout_DX, uVar7);
        ppcVar3 = (*_local_1a + 0x10);
        uVar8 = uVar7;
        (**ppcVar3)(&PTR_LOOP_1050_1038, uVar7, extraout_DX);
        _local_12 = CONCAT22(extraout_DX_00, uVar8);
        bVar4 = false;
        local_e = 0;
        while (local_e < _local_12)
        {
            uVar11 = local_e;
            pass1_1030_1d7c(uVar7, extraout_DX, local_e, local_e._2_2_);
            local_e._2_2_ = extraout_DX_02 | uVar11;
            if (((local_e._2_2_ != 0) && ((uVar11 + 4) != (param_1 + 4))) &&
                (uVar6 = pass1_1030_cf3a(uVar11 & 0xffff | extraout_DX_02 << 0x10, 8), uVar6 != 0))
            {
                bVar4 = true;
                break;
            }
            local_e = local_e + 1;
        }
        if (_local_1a != 0x0)
        {
            ppcVar3 = *_local_1a;
            (**ppcVar3)(0x38, uVar7, extraout_DX, 1);
            local_e._2_2_ = extraout_DX_01;
        }
        in_DX = local_e._2_2_;
        if (!bVar4)
        {
            return;
        }
    }
    puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 4);
    uVar7 = SUB42(puVar10, 0);
    uVar12 = 0x38;
    pass1_1038_4d6e(paVar2, puVar10 & 0xffff | in_DX << 0x10);
    _local_1a = CONCAT22(extraout_DX_03, uVar7);
    ppcVar3 = (*_local_1a + 0x10);
    uVar8 = uVar7;
    (**ppcVar3)(&PTR_LOOP_1050_1038, uVar7, extraout_DX_03);
    _local_12 = CONCAT22(extraout_DX_04, uVar8);
    bVar4 = false;
    local_e = 0;
    loop
    {
        if (_local_12 <= local_e)
        {
// LAB_1030_d51b:
            if (_local_1a != 0x0)
            {
                ppcVar3 = *_local_1a;
                (**ppcVar3)(uVar12, uVar7, extraout_DX_03, 1);
            }
            if (!bVar4)
            {
                return;
            }
            uVar9 = *_PTR_LOOP_1050_65e2;
            iVar1 = (_PTR_LOOP_1050_65e2 + 2);
            (param_1 + param_2_00 * 0xc + 0x20) = uVar9 + 200;
            (param_1 + param_2_00 * 0xc + 0x22) = iVar1 + (0xff37 < uVar9);
            return;
        }
        uVar11 = local_e;
        pass1_1030_1d7c(uVar7, extraout_DX_03, local_e, local_e._2_2_);
        uVar9 = uVar11;
        if ((extraout_DX_05 | uVar9) != 0)
        {
            uVar14 = (extraout_DX_05 >> 8);
            uVar12 = 0x28;
            uVar13 = pass1_1028_6744(CONCAT13(uVar14, CONCAT12(extraout_DX_05, uVar9)), 7);
            if (((uVar13 >> 0x10) | uVar13) != 0)
            {
                uVar12 = 0x28;
                pass1_1028_6228(CONCAT13(uVar14, CONCAT12(extraout_DX_05, uVar9)), 1, 0, 7);
                bVar4 = true;
              // goto LAB_1030_d51b;
            }
        }
        local_e = local_e + 1;
    } 
}

pub fn pass1_1030_d56a(param_1: *mut astruct_1007) -> *mut astruct_1007

{
    let local_BX_3: *mut astruct_1007;
    let local_ES_3: *mut astruct_1007;

    local_ES_3 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    match (local_BX_3.field_0x98 + -1)
    {
    0 =>{
        local_BX_3.field_0x98 = 2},
    1 =>{
        local_BX_3.field_0x98 = 3},
    2 =>{
        local_BX_3.field_0x98 = 4},
    3 =>{
        local_BX_3.field_0x98 = 0xc},
// default:
    _ => {
        local_BX_3.field_0x98 = 1;
        return local_BX_3; },
    7 =>{
        local_BX_3.field_0x98 = 9;
        return local_BX_3; },
    8 =>{
        local_BX_3.field_0x98 = 0xb;
        return local_BX_3; },
    10 =>{
        local_BX_3.field_0x98 = 5;
        return local_BX_3; },
    0xb =>{
        local_BX_3.field_0x98 = 8;
        return local_BX_3;
    }
}
    return local_BX_3;
}



pub fn pass1_1030_c2fa(param_1: *mut astruct_44)

{
    let mut uVar1: i32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let uVar7: u8;
    let paVar8: *mut astruct_493;
    let mut iVar9: i32;
    let extraout_var: u32;
    let mut uVar10: u32;
    let mut uVar11: u32;
    let in_DX: *mut u8;
    let puVar12: *mut u8;
    let mut uVar13: u16;
    let mut uVar14: u32;
    let mut unaff_SI: u16;
    let mut uVar15: u16;
    let ppVar16: *mut pass1_struct_1;
    let mut uVar17: u16;
    let mut local_54: u16;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_38: u16;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_2e: u32;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_1e: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar15 = (param_1 >> 0x10);
    if ((param_1 + 0xc) != 0xb)
    {
        pass1_1028_bd38(param_1);
        uVar2 = (param_1 + 0x20);
        paVar8 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
        _local_6 = CONCAT22(in_DX, paVar8);
        puVar12 = in_DX;
        uVar7 = pass1_1028_b58e(param_1);
        uVar6 = CONCAT31(extraout_var, uVar7) & 0xffff | ZEXT24(puVar12) << 0x10;
        uVar3 = (CONCAT31(extraout_var, uVar7) + 0x2e);
        ppVar16 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2f));
        uVar14 = ppVar16 >> 0x10;
        uVar15 = (uVar3 >> 0x10);
        pass1_1010_ed22(ppVar16, (uVar3 + 4));
        uVar4 = paVar8[0x10].field_0x16;
        uVar13 = (uVar4 >> 0x10);
        uVar10 = uVar4;
        pass1_1030_3694(uVar4, uVar13, 3, 2, 0);
        uVar5 = (uVar3 + 0x1f6);
        pass1_1030_355c(uVar5, uVar10 & 0xffff | uVar14 << 0x10);
        uVar15 = (uVar5 >> 0x10);
        local_38 = 0;
        while
        {
            iVar9 = local_38 * 2;
            (iVar9 + uVar5 + 0x174) = (iVar9 + uVar4 + 0x174);
            uVar1 = (iVar9 + uVar4 + 0x180);
            uVar11 = uVar1;
            (iVar9 + uVar5 + 0x180) = uVar1;
            local_38 = local_38 + 1;
            local_38 < 6
        } {}
        local_54 = 0x11;
        loop
        {
            uVar13 = uVar14;
            uVar15 = uVar11;
            if (0x24 < local_54) {
                break;}
            if (0 < (local_54 * 2 + _PTR_LOOP_1050_580e))
            {
                pass1_1038_540a(paVar8, in_DX, local_54);
                _local_50 = CONCAT22(uVar13, uVar15);
                uVar15 = (_PTR_LOOP_1050_580e >> 0x10);
                iVar9 = (local_54 * 2 + _PTR_LOOP_1050_580e);
                uVar14 = iVar9 >> 0x10;
                uVar17 = local_54;
                if (_local_50 < iVar9)
                {
                    iVar9 = (local_54 * 2 + _PTR_LOOP_1050_580e);
                    uVar14 = (iVar9 >> 0xf);
                    uVar17 = 0x21;
                }
                pass1_1038_52b8(_local_6, CONCAT22(uVar14, iVar9), uVar17);
                pass1_1030_7ddc(uVar6, (local_54 * 2 + _PTR_LOOP_1050_580e), local_54);
                iVar9 = (_PTR_LOOP_1050_580e + local_54 * 2);
                uVar11 = SEXT24(iVar9);
                pass1_1038_5694(uVar3, iVar9, local_54);
            }
            local_54 = local_54 + 1;
        }
        pass1_1030_7c50(uVar6, 2, 1);
        pass1_1030_7c50(uVar6, 2, 2);
        pass1_1030_7c50(uVar6, 2, 3);
        pass1_1030_7c50(uVar6, 2, 4);
        pass1_1038_44d8(paVar8, in_DX, 2, (&PTR_LOOP_1050_0000 + 1));
        pass1_1038_44d8(paVar8, in_DX, 2, &dos_alloc_addr_1050_0002);
        pass1_1038_44d8(paVar8, in_DX, 2, (&dos_alloc_addr_1050_0002 + 1));
        pass1_1038_44d8(paVar8, in_DX, 2, &PTR_DAT_0005_0000_1050_0004);
        ppVar16 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2b));
        pass1_1010_043a(ppVar16, &paVar8.field_0x4, 7);
    }
    return;
}




pub fn pass1_1030_c52e(param_1: u32, param_2: *mut u16,param_3: u32,param_4: u32)

{
    let mut uVar1: u16;
    let puVar2: *mut u32;
    let puVar3: *mut u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut uVar4: i32;
    let mut unaff_SS: u16;
    let puVar5: *mut u16;
    let mut uVar6: u32;
    let mut local_38: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar1 = pass1_1028_c314(param_1, (param_1 >> 0x10), param_2, param_3,
                            (param_3 >> 0x10), param_4);
    if (uVar1 != 0)
    {
        puVar2 = &local_c;
        pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, param_4, puVar2, unaff_SS);
        unsafe{local_20 = *puVar2};
        local_20._3_1_ = (local_20 >> 0x18);
        local_8 = local_20._3_1_;
        if (local_20._3_1_ == 0)
        {
            local_16 = local_20;
            local_6 = local_20;
            pass1_1028_c7b6(param_1, param_2, param_4);
            if ((local_8 != 4) && (local_8 != 0))
            {
                pass1_1030_bcae(&local_20, unaff_SS);
                pass1_1028_dc52(CONCAT22(unaff_SS, &local_32),
                                (&PTR_LOOP_1050_0000 + 1), 0, 0x400);
                loop
                {
                    puVar3 = &local_32;
                    pass1_1028_e4ec(CONCAT22(unaff_SS, puVar3));
                    _local_1c = CONCAT22(extraout_DX_00, puVar3);
                    uVar4 = extraout_DX_00 | puVar3;
                    if (uVar4 == 0)
                    {
                        return;
                    }
                    local_16 = (puVar3 + 8);
                    uVar6 = param_4;
                    puVar5 = param_2;
                    local_12 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_16, (local_16 >> 0x10));
                    puVar2 = &local_20;
                    local_10 = uVar4;
                    pass1_1030_bcde(puVar2, unaff_SS, CONCAT22(uVar4, local_12), puVar5,
                                    uVar6);
                    if (puVar2 < 0) {
                        break;}
                    local_18 = puVar2;
                    if (puVar2 < 0x1f)
                    {
                        PTR_LOOP_1050_50ca = 0x6ae;
                        return;
                    }
                }
                PTR_LOOP_1050_50ca = 0x6af;
                return;
            }
            PTR_LOOP_1050_50ca = 0x6a8;
        }
    }
    return;
}


// WARNING: Restarted to delay deadcode elimination for space: stack

pub fn pass1_1030_c652()

{
    let in_struct_1: *mut pass1_struct_1;
    let mut in_stack_00000000: u16;

    in_struct_1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_00000000, 8));
    pass1_1010_9794(in_struct_1);
    return;
}

pub fn pass1_1030_c668(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_c6f6(param_1: *mut astruct_763) -> *mut astruct_763

{
    let mut uVar1: u16;

    pass1_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0;
    param_1.field_0x0 = 0xc940;
    (param_1 + 2) = 0x1030;
    return param_1;
}


pub fn pass1_1030_c71e(param_1: *mut astruct_764, param_2: *mut astruct_764, param_3: u16,param_3_00: u32) -> i32

{
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    &param_1.field_0x20 = 0;
    CONCAT22(param_2, param_1) = 0xc940;
    param_1.field_0x2 = 0x1030;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1030_c74e(param_1: *mut astruct_781, param_2: *mut u8)

{
    pass1_1028_b46e(param_1, param_2);
    (param_1 + 0x20) = 0x70;
    return;
}

pub fn pass1_1030_c76c(param_1: *mut astruct_833)

{
    let mut iVar1: i32;
    let local_BX_3: *mut astruct_833;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if ((local_BX_3.field_0x12 != 6) && (local_BX_3.field_0x12 != 5))
    {
        return;
    }
    iVar1 = local_BX_3.field_0x20;
    if (iVar1 != 0)
    {
        if (((iVar1 < 0x70) || (SBORROW2(iVar1, 0x70))) || (1 < iVar1 + -0x70))
        {
            pass1_1028_be2a();
            return;
        }
    }
    pass1_1028_bdac(param_1, 6);
    return;
}




pub fn pass1_1030_c7b0(param_1: *mut astruct_44)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let uVar3: u8;
    let paVar4: *mut astruct_493;
    let BVar5: bool;
    let extraout_var: u32;
    let mut in_DX: u16;
    let mut iVar6: i32;
    let mut uVar7: i32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    iVar1 = (iVar6 + 0x20);
    if (iVar1 != 0)
    {
        if (((iVar1 < 0x70) || (SBORROW2(iVar1, 0x70))) || (1 < iVar1 + -0x70))
        {
            uVar3 = pass1_1028_b58e((param_1 & 0xffff | uVar7 << 0x10));
            uVar2 = (CONCAT31(extraout_var, uVar3) + 0x2e);
            uVar2 = (uVar2 + 0x200);
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
            _local_12 = CONCAT22(in_DX, paVar4);
            BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (iVar6 + 0xc), 0x11);
            pass1_1030_23e2(_local_12, BVar5, (iVar6 + 0x20));
            if (BVar5 != 0)
            {
                if ((iVar6 + 0x20) == 1)
                {
                    pass1_1030_25d8(_local_12, 100, (iVar6 + 0x20));
                    return;
                }
                (iVar6 + 0x20) = 0x70;
            }
        }
    }
    return;
}


pub fn pass1_1030_bbe6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_b96c(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_bc1c()

{
    let pbVar1: *mut byte;
    let pcVar2: *mut libc::c_char;
    let mut cVar3: u8;
    let mut in_DX: u16;
    let mut in_BX: i32;
    let puVar4: *mut u16;
    let unaff_BP: *mut u16;
    let mut unaff_SI: i32;
    let mut unaff_SS: u16;
    let mut in_ZF: bool;
    let mut in_SF: u8;
    let mut in_OF: u8;
    let mut in_stack_0000d730: i32;
    let mut in_stack_0000d732: u16;
    let mut in_stack_0000d734: u16;
    let mut in_stack_0000d736: u32;
    let paStack6: *mut astruct_44;

    puVar4 = &stack0xfffe;
    cVar3 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar4 = puVar4 + -1;
        unsafe{*puVar4 = *unaff_BP};
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    if (!in_ZF && in_OF == in_SF)
    {
        pcVar2 = (in_BX + unaff_SI);
        unsafe{*pcVar2 = *pcVar2 - in_DX};
        pass1_1028_b22c(CONCAT22(in_stack_0000d732, in_stack_0000d730), in_stack_0000d734,
                        in_stack_0000d736);
        CONCAT22(in_stack_0000d732, in_stack_0000d730) = 0xbc96;
        (in_stack_0000d730 + 2) = 0x1030;
        return CONCAT22(in_stack_0000d732, in_stack_0000d730);
    }
    pbVar1 = (in_BX + unaff_SI);
    unsafe{*pbVar1 = *pbVar1 & in_DX};
    error_check_1000_17ce(paStack6);
    return CONCAT22(in_DX, 1);
}

pub fn pass1_1030_bc24(param_1: i32, param_2: u16, param_3: u16,param_3_00: u32)

{
    pass1_1028_b22c(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xbc96;
    (param_1 + 2) = 0x1030;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1030_bc4e(param_1: *mut u16)

{
    unsafe{*param_1 = 0xbc96;
    (param_1 + 2) = 0x1030;}
    pass1_1028_b260(param_1);
    return;
}

pub fn pass1_1030_bc70(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_bc4e(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_bca6()

{
    let uVar1: u8;
    let mut cVar2: u8;
    let mut in_DX: u16;
    let puVar3: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_DI: *mut u8;
    let mut unaff_ES: u16;
    let mut unaff_SS: u16;

    puVar3 = &stack0xfffe;
    cVar2 = '\x0f';
    while
    {
        unaff_BP = unaff_BP + -1;
        puVar3 = puVar3 + -1;
        unsafe{*puVar3 = *unaff_BP};
        cVar2 = cVar2 + -1;
        '\0' < cVar2
    } {}
    uVar1 = _in(in_DX);
    unsafe{*unaff_DI = uVar1;
    *0x102e = &stack0xfffe;}
    return CONCAT22(0x1036, 0x1034);
}

pub fn pass1_1030_bcae(param_1: u16, param_2: u16)

{
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1030_bcbc(param_1: u32,param_2: u32,param_3: u32,param_4: u32)

{
    pass1_1030_bcde(param_1, (param_1 >> 0x10), param_2, param_3,
                    (param_4 + 4));
    return;
}


pub fn pass1_1030_bcde(param_1: u16, param_2: u16, param_1_00: *mut astruct_966,param_2_00: u32,param_5: u32) -> i32

{
    let local_BX_6: *mut astruct_966;
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut local_6: u32;

    uVar1 = (param_1_00 >> 0x10);
    local_BX_6 = param_1_00;
    local_6 = local_BX_6.field_0x8;
    if (local_6 != param_5)
    {
        return;
    }
    local_c = local_BX_6.field_0xc;
    uStack8 = local_BX_6.field_0x10;
    pass1_1008_3e94(param_2_00, CONCAT22(unaff_SS, &local_10),
                    CONCAT22(unaff_SS, &local_e));
    pass1_1008_3e94(&local_c, CONCAT22(unaff_SS, &local_14),
                    CONCAT22(unaff_SS, &local_12));
    xor_1000_49b2();
    xor_1000_49b2();
    return;
}

pub fn pass1_1030_bd74(param_1: u16, param_2: u16, param_1_00: *mut astruct_968, param_2_00: *mut astruct_967) -> i32

{
    let local_BX_6: *mut astruct_967;
    let local_BX_18: *mut astruct_968;
    let mut uVar1: u16;
    let mut local_ES_18: u16;
    let mut unaff_SS: u16;
    let mut local_2e: u16;
    let mut local_26: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut uStack18: u16;
    let mut local_10: u32;
    let mut uStack12: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar1 = (param_2_00 >> 0x10);
    local_BX_6 = param_2_00;
    local_6 = local_BX_6.field_0x8;
    local_ES_18 = (param_1_00 >> 0x10);
    local_BX_18 = param_1_00;
    local_a = local_BX_18.field_0x8;
    if (local_a != local_6)
    {
        return;
    }
    local_10 = local_BX_6.field_0xc;
    uStack12 = local_BX_6.field_0x10;
    local_16 = local_BX_18.field_0xc;
    uStack18 = local_BX_18.field_0x10;
    pass1_1008_3e94(&local_10, CONCAT22(unaff_SS, &local_1a),
                    CONCAT22(unaff_SS, &local_18));
    pass1_1008_3e94(&local_16, CONCAT22(unaff_SS, &local_1e),
                    CONCAT22(unaff_SS, &local_1c));
    xor_1000_49b2();
    xor_1000_49b2();
    return;
}

pub fn pass1_1030_be34(param_1: *mut astruct_763) -> *mut astruct_763

{
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0xc006;
    (param_1 + 2) = 0x1030;
    return param_1;
}

pub fn pass1_1030_be56(param_1: i32, param_2: u16, param_3: u16,param_3_00: u32)

{
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xc006;
    (param_1 + 2) = 0x1030;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1030_be80(param_1: *mut astruct_44)

{
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let uVar3: u8;
    let mut uVar4: u16;
    let extraout_var: u32;
    let puVar5: *mut u8;
    let mut in_DX: u16;
    let mut extraout_DX: i32;
    let local_BX_13: *mut astruct_969;
    let mut uVar6: u16;
    let mut iVar7: i32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_bf22(param_1);
    uVar6 = (param_1 >> 0x10);
    local_BX_13 = param_1;
    if (local_BX_13.field_0x12 == 5)
    {
        uVar2 = local_BX_13.field_0x14;
        (uVar2 + 0xa4) = 0x1e;
        uVar2 = local_BX_13.field_0x14;
        (uVar2 + 0xac) = 1;
        iVar7 = local_BX_13.field_0xc;
        if (iVar7 == 0x1b)
        {
            uVar2 = local_BX_13.field_0x14;
            (uVar2 + 0xaa) = 10;
        }
        else
        {
            if (iVar7 == 0x1c)
            {
                uVar2 = local_BX_13.field_0x14;
                (uVar2 + 0xaa) = 0xb;
            }
            else
            {
                if (iVar7 == 0x1d)
                {
                    uVar2 = local_BX_13.field_0x14;
                    (uVar2 + 0xaa) = 0xc;
                }
            }
        }
        uVar3 = pass1_1028_b58e(param_1);
        puVar5 = *(CONCAT31(extraout_var, uVar3) + 0x2e);
        iVar7 = 0xc;
        pass1_1038_3fb0(puVar5);
        uVar4 = pass1_1030_25b2(puVar5 & 0xffff | extraout_DX << 0x10, iVar7);
        if (uVar4 != 0)
        {
            uVar2 = local_BX_13.field_0x14;
            piVar1 = (uVar2 + 0xaa);
            unsafe{ *piVar1 = *piVar1 + 1 };
        }
        uVar4 = pass1_1030_25b2(puVar5 & 0xffff | extraout_DX << 0x10, 0xe);
        if (uVar4 != 0)
        {
            uVar2 = local_BX_13.field_0x14;
            piVar1 = (uVar2 + 0xaa);
            unsafe{ *piVar1 = *piVar1 + 1 };
        }
        uVar4 = pass1_1030_25b2(puVar5 & 0xffff | extraout_DX << 0x10, 0x76);
        if (uVar4 != 0)
        {
            uVar2 = local_BX_13.field_0x14;
            piVar1 = (uVar2 + 0xaa);
            unsafe{ *piVar1 = *piVar1 + 1 };
        }
    }
    return;
}




pub fn pass1_1030_bf6e(param_1: *mut astruct_44)

{
    let puVar1: *mut u32;
    let uVar2: u8;
    let extraout_AH: u8;
    let mut in_DX: u16;
    let local_BX_36: *mut astruct_970;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5ff0eedf2f: u32;
    let mut iVar3: i32;

    uVar5 = 0x1e;
    uVar2 = pass1_1028_b58e(param_1);
    iVar3 = CONCAT11(extraout_AH, uVar2);
    _local_6 = CONCAT22(in_DX, iVar3);
    pass1_1030_7c28(CONCAT22(in_DX, iVar3), uVar5);
    temp_5ff0eedf2f = (param_1 + 0x14);
    uVar4 = (temp_5ff0eedf2f >> 0x10);
    local_BX_36 = temp_5ff0eedf2f;
    puVar1 = &local_BX_36.field_0xaa;
    let pu_var1_val = unsafe{*puVar1};
    pass1_1030_7ddc(_local_6, (((1000 - iVar3) - pu_var1_val & -(1000 - iVar3 < pu_var1_val)) + local_BX_36.field_0xaa), 0x1e);
    return;
}

pub fn pass1_1030_bfb8(param_1: *mut astruct_44)

{
    let uVar1: u8;
    let extraout_AH: u8;
    let mut in_DX: i32;
    let mut uvar3: u16;
    let mut uVar2: i32;

    uVar3 = 0x1e;
    uVar1 = pass1_1028_b58e(param_1);
    uVar2 = CONCAT11(extraout_AH, uVar1);
    pass1_1030_7c28(CONCAT22(in_DX, uVar2), uVar3);
    return CONCAT22(-(1000 < uVar2) - in_DX, 1000 - uVar2);
}

pub fn pass1_1030_bfe0(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_c06e(param_1: *mut astruct_763)

{
    let local_BX_12: *mut astruct_763;
    let mut uVar1: u16;

    pass1_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    local_BX_12 = param_1;
    local_BX_12.field_0x20 = 0;
    &local_BX_12.field_0x24 = 0;
    param_1.field_0x0 = 0xc68e;
    local_BX_12.field_0x2 = 0x1030;
    return;
}

pub fn pass1_1030_c09c(param_1: i32, param_2: u16, param_3: u16,param_3_00: u32)

{
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    (param_1 + 0x20) = 0;
    (param_1 + 0x24) = 0;
    CONCAT22(param_2, param_1) = 0xc68e;
    (param_1 + 2) = 0x1030;
    return;
}

pub fn pass1_1030_c0d2(param_1: u32)

{
    if (0 < (param_1 + 0x24))
    {
        return 1;
    }
    return 0;
}

pub fn pass1_1030_c0ec(param_1: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if (((param_1 + 0xc) != 0xb) && ((param_1 + 0x24) < 1))
    {
        return 0;
    }
    return 1;
}

pub fn pass1_1030_c10e(param_1: u32)

{
    let piVar1: *mut i32;
    let local_BX_3: *mut astruct_972;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (0 < local_BX_3.field_0x24)
    {
        piVar1 = &local_BX_3.field_0x24;
        unsafe{ *piVar1 = *piVar1 + -1 };
        return;
    }
    local_BX_3.field_0xc = 0x37;
    return;
}

pub fn pass1_1030_c12e(param_1: *mut astruct_44, param_2: i32)

{
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let uVar4: u8;
    let extraout_var: u32;
    let mut in_DX: i32;
    let local_BX_31: *mut astruct_973;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = pass1_1028_b58e(param_1);
    uVar3 = CONCAT31(extraout_var, uVar4) & 0xffff | in_DX << 0x10;
    uVar2 = (CONCAT31(extraout_var, uVar4) + 0x2e);
    uVar5 = (param_1 >> 0x10);
    local_BX_31 = param_1;
    if (local_BX_31.field_0x24 < 1)
    {
        pass1_1030_7d1c(uVar3, 0, 0x230000);
    }
    else
    {
        if (param_2 == 0)
        {
            uVar6 = 0;
        }
        else
        {
            uVar6 = 0x32;
        }
        pass1_1030_7d1c(uVar3, uVar6, 0x230000);
        piVar1 = &local_BX_31.field_0x24;
        unsafe{ *piVar1 = *piVar1 + -1 };
    }
    if ((0 < local_BX_31.field_0x24) && (local_BX_31.field_0x24 < 0x19))
    {
        (uVar2 + 0x1fe) = 1;
    }
    return;
}




pub fn pass1_1030_c1b2(param_1: *mut astruct_974)

{
    let mut iVar1: i32;
    let ppVar2: *mut pass1_struct_1;
    let local_16: *mut astruct_974;
    let local_14: *mut astruct_974;
    let mut uStack18: u16;

    local_16 = param_1;
    local_14 = (param_1 >> 0x10);
    pass1_1028_be9e(param_1);
    if (local_16.field_0x12 == 5)
    {
        if (local_16.field_0xc == 0xb)
        {
            pass1_1030_c652(param_1);
            ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x820008);
            iVar1 = pass1_1010_9f8c(ppVar2, 0x82);
            local_16.field_0x24 = iVar1 * 3;
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(uStack18, 2));
            if (u16_1050_13ae < 3)
            {
                iVar1 = local_16.field_0x24;
                if (iVar1 < 0x32)
                {
                    iVar1 = 0x32;
                }
                local_16.field_0x24 = iVar1;
                return;
            }
        }
        else
        {
            local_16.field_0x24 = 100;
        }
    }
    return;
}


pub fn pass1_1030_b90c(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_afa6(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_b936(param_1: *mut astruct_963,param_2: u32,param_3: u32)

{
    let mut uVar1: u16;

    uVar1 = param_2;
    pass1_1028_b22c(CONCAT22(uVar1, param_1), (param_2 >> 0x10), param_3);
    param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    CONCAT22(uVar1, param_1) = 0xbc0c;
    param_1.field_0x2 = 0x1030;
    return;
}

pub fn pass1_1030_b96c(param_1: *mut astruct_44)

{
    let mut uVar1: i32;
    let in_struct_1: *mut astruct_44;
    let local_BX_4: *mut astruct_964;
    let mut uVar2: u16;
    let mut local_6: u32;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.ptr_a_lo = 0xbc0c;
    local_BX_4.field_0x2 = 0x1030;
    in_struct_1 = &local_BX_4.field_0xe;
    uVar1 = local_BX_4.field_0x10;
    if ((uVar1 | in_struct_1) != 0)
    {
        pass1_1020_ba7e((in_struct_1 & 0xffff | uVar1 << 0x10));
        error_check_1000_17ce(in_struct_1);
    }
    pass1_1028_b260(param_1);
    return;
}

pub fn pass1_1030_b9b2(param_1: u32)

{
    let mut uVar1: u16;
    let mut local_6: u32;

    uVar1 = (param_1 >> 0x10);
    (param_1 + 0xe) = 0;
    (param_1 + 0x12) = 0;
    return;
}




pub fn pass1_1030_b718(param_1: u16, param_2: u16,param_1_00: u32, param_2_00: *mut u32)

{
    let puVar1: *mut u32;
    let mut extraout_DX: u16;
    let mut unaff_SS: u16;
    let mut in_stack_0000ffec: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0,
                                        CONCAT22((in_stack_0000ffec >> 0x10), 0x2f));
    local_a = (_local_6 + 0x20);
    puVar1 = &stack0xffee;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_1_00, local_a, (local_a >> 0x10), puVar1, unaff_SS);
    unsafe{ *param_2_00 = *puVar1 };
    return;
}


pub fn pass1_1030_9ecc(param_1: *mut u32,param_2: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    unsafe{*param_1 = 0;
    (param_1 + 4) = param_2;
    (param_1 + 8) = 0;}
    return;
}

pub fn pass1_1030_9ef2(param_1: *mut *mut astruct_493)

{
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let param_1_val = unsafe{*param_1};
    if (param_1_val != 0x0)
    {
        uVar3 = pass1_1030_73a8(param_1_val);
        uVar2 = (uVar3 >> 0x10);
        iVar1 = (uVar3 + 0xc);
        if (((iVar1 != 5) && (iVar1 != 9)) && ((uVar3 + 0x12) < 5))
        {
            return 0;
        }
        pass1_1030_9f64(param_1);
    }
    return 1;
}



pub fn pass1_1030_9f40(param_1: u32, param_2: u16)

{
    let mut uVar1: u16;

    uVar1 = pass1_1008_c646(_PTR_LOOP_1050_06e0,
                            CONCAT22(param_2, (_PTR_LOOP_1050_06e0 >> 0x10)));
    (param_1 + 8) = uVar1;
    pass1_1030_9f7a(param_1, uVar1);
    return;
}

pub fn pass1_1030_9f64(param_1: *mut u32)

{
    (param_1 + 8) = 0;
    unsafe{*param_1 = 0};
    return;
}



pub fn pass1_1030_9f7a(param_1: *mut u16, param_2: u16)

{
    let mut u_var1: u32;
    let uVar2: u8;
    let BVar3: bool;
    let puVar4: *mut u32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut local_148: u16;
    let mut local_134: u32;
    let mut local_130: u16;
    let mut local_12e: u16;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

    zero_list_1008_3e38(CONCAT22(unaff_SS, &local_8));
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, param_2, 0x28);
    if (BVar3 != 0)
    {
        local_4 = 1;
    }
    puVar4 = &local_8;
    pass1_1030_a278(param_1, CONCAT22(unaff_SS, puVar4));
    if (puVar4 != 0x0)
    {
        uVar7 = (param_1 >> 0x10);
        uVar6 = param_1;
        uVar1 = (uVar6 + 4);
        local_c = (uVar1 + 8);
        uVar1 = (uVar6 + 4);
        pass1_1028_87f0(CONCAT22(unaff_SS, &local_130), 0, 0, param_2, &local_8, unaff_SS,
                        (uVar1 + 4), local_c);
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_130));
        uVar5 = extraout_DX;
        uVar2 = pass1_1028_b58e(local_10);
        unsafe{*param_1 = CONCAT31(extraout_var, uVar2)};
        (uVar6 + 2) = uVar5;
        if (0 < local_4)
        {
            pass1_1030_a044(uVar6, uVar7, CONCAT22(unaff_SS, &local_8), local_c);
        }
    }
    return;
}




pub fn pass1_1030_a044(param_1: u16, param_2: u16, param_1_00: *mut u16,param_2_00: u32)

{
    let ppcVar1: fn();
    let paVar2: *mut astruct_493;
    let puVar3: *mut u8;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut uVar6: i32;
    let mut extraout_DX_01: u16;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let puVar8: *mut u32;
    let mut uVar9: u16;
    let uVar10: u8;
    let uVar11: u8;
    let mut uVar12: u16;
    let mut local_17e: u16;
    let mut local_17c: u16;
    let mut local_5a: u16;
    let mut local_4e: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u32;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: [u8;12];
    let mut local_2e: [u8;6];
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u32;
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

    local_14 = &local_8;
    pass1_1008_3eb4(param_1_00, CONCAT22(unaff_SS, local_14), CONCAT22(unaff_SS, &local_6),
                    CONCAT22(unaff_SS, &local_4));
    pass1_1030_627e(_PTR_LOOP_1050_5740, param_1_00, param_2_00);
    local_26 = extraout_DX;
    local_12 = extraout_DX;
    paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_14, extraout_DX);
    _local_18 = CONCAT22(local_26, paVar2);
    local_1c = &paVar2[1].field_0x10;
    local_20 = (local_1c + 4);
    local_28 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    puVar8 = pass1_1030_5b5c(local_28, local_26);
    unsafe{local_2e._0_4_ = *puVar8;
    local_2e._4_2_ = (puVar8 + 4);}
    local_4e = local_2e;
    pass1_1008_3e94(local_2e, CONCAT22(unaff_SS, &local_24),
                    CONCAT22(unaff_SS, &local_22));
    iVar4 = local_4 + 1;
    _local_c = CONCAT22(local_4 - 1, local_6 - 1);
    iVar5 = local_6 + 1;
    if ((local_4 - 1) < 0)
    {
        _local_c = (local_6 - 1);
    }
    if (local_22 <= iVar4)
    {
        iVar4 = local_22 - 1;
    }
    if (local_c < 0)
    {
        _local_c = _local_c & 0xffff0000;
    }
    if (local_24 <= iVar5)
    {
        iVar5 = local_24 - 1;
    }
    _local_10 = CONCAT22(iVar4, iVar5);
    zero_list_1008_6c90(local_3a, unaff_SS);
    uVar7 = SUB42(&PTR_LOOP_1050_1008, 0);
    pass1_1008_6cec(CONCAT22(unaff_SS, local_3a), local_8, _local_10, local_8, _local_c);
    puVar3 = local_3a;
    uVar6 = extraout_DX_00;
    pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, puVar3), param_2_00);
    _local_3e = CONCAT22(uVar6, puVar3);
    if ((uVar6 | puVar3) != 0)
    {
        local_42 = 0;
        local_44 = 0;
        local_46 = local_c;
        while (local_46 <= local_10)
        {
            local_4e = local_a;
            while (local_4e <= local_e)
            {
                ppcVar1 = (*_local_3e + 4);
                uVar9 = local_44;
                local_44 = local_44 + 1;
                (**ppcVar1)(uVar7, _local_3e, (_local_3e >> 0x10));
                local_42 = CONCAT22(extraout_DX_01, uVar9);
                local_42._3_1_ = (extraout_DX_01 >> 8);
                if (local_42._3_1_ == '\0')
                {
                    local_5a = uVar9;
                    if (uVar9 == 7)
                    {
                        pass1_1008_3e76(param_1_00, local_8, local_46, local_4e);
                        uVar10 = local_20;
                        uVar11 = (local_20 >> 8);
                        uVar12 = (local_20 >> 0x10);
                        uVar9 = 6;
                    }
                    else
                    {
                        if (uVar9 == 8)
                        {
                            pass1_1008_3e76(param_1_00, local_8, local_46, local_4e);
                            uVar10 = local_20;
                            uVar11 = (local_20 >> 8);
                            uVar12 = (local_20 >> 0x10);
                            uVar9 = 7;
                        }
                        else
                        {
                            if (uVar9 != 9){}
                              // goto LAB_1030_a1d0;
                            pass1_1008_3e76(param_1_00, local_8, local_46, local_4e);
                            uVar10 = local_20;
                            uVar11 = (local_20 >> 8);
                            uVar12 = (local_20 >> 0x10);
                            uVar9 = 8;
                        }
                    }
                    uVar7 = SUB42(&PTR_LOOP_1050_1028, 0);
                    pass1_1028_87f0(CONCAT22(unaff_SS, &local_17e), 0, 0, uVar9, param_1_00,
                                    (param_1_00 >> 0x10), CONCAT22(uVar12, CONCAT11(uVar11, uVar10)),
                                    param_2_00);
                    pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_17e));
                    local_17e = s_1_1050_389a;
                    local_17c = &PTR_LOOP_1050_1008;
                }
// LAB_1030_a1d0:
                local_4e = local_4e + 1;
            }
            local_46 = local_46 + 1;
        }
    }
    return;
}



pub fn pass1_1030_a278(param_1: *mut u16,param_2: u32)

{
    let mut u_var1: u32;
    let uVar2: u8;
    let mut in_AX: u16;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let local_BX_46: *mut astruct_951;
    let local_BX_149: *mut astruct_952;
    let mut uVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_134: u16;
    let mut local_132: u16;
    let struct_a: *mut astruct_44;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 1;
    pass1_1030_a39a(param_1, param_2);
    if (in_AX != 0)
    {
        return;
    }
    local_6 = in_AX;
    pass1_1030_a3ae(param_1, param_2);
    local_BX_46 = param_2;
    uVar4 = (param_2 >> 0x10);
    if (in_AX == 0)
    {
        pass1_1030_a57e(param_1, param_2);
        if (in_AX == 0)
        {
            pass1_1030_a844(param_1, param_2);
            if (in_AX == 0)
            {
                local_4 = 0;
              // goto LAB_1030_a305;
            }
            uVar5 = local_BX_46.field_0x4;
        }
        else
        {
            uVar5 = local_BX_46.field_0x4;
        }
        if (uVar5 < 1)
        {
            local_6 = 0x73;
        }
        else
        {
            local_6 = 0x77;
        }
    }
    else
    {
        if (local_BX_46.field_0x4 < 1)
        {
            local_6 = 0x7a;
        }
        else
        {
            local_6 = 0x7f;
        }
    }
// LAB_1030_a305:
    if (local_6 != 0)
    {
        uVar5 = (param_1 >> 0x10);
        local_BX_149 = param_1;
        uVar1 = local_BX_149.field_0x4;
        local_10 = (uVar1 + 8);
        uVar1 = local_BX_149.field_0x4;
        pass1_1028_87f0(CONCAT22(unaff_SS, &local_134), 0, 0, local_6, local_BX_46,
                        uVar4, (uVar1 + 4), local_10);
        pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, &local_134));
        local_c = struct_a;
        uVar3 = extraout_DX;
        uVar2 = pass1_1028_b58e(struct_a);
        unsafe{*param_1 = CONCAT31(extraout_var, uVar2)};
        local_BX_149.field_0x2 = uVar3;
        if (0 < local_BX_46.field_0x4)
        {
            pass1_1030_a044(local_BX_149, uVar5, (param_2 & 0xffff | uVar4 << 0x10),
                            local_10);
        }
    }
    return;
}

pub fn pass1_1030_a39a(param_1: u32,param_2: u32)

{
    pass1_1030_aa18(param_1, param_2);
    return;
}





pub fn pass1_1030_a3ae(param_1: u32, param_2: *mut u16)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uvar3: u16;
    let BVar4: bool;
    let puVar5: *mut u8;
    let mut uVar6: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut uVar7: u16;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut unaff_SS: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut local_4e: u32;
    let mut local_4a: u32;
    let mut local_46: u16;
    let mut local_3c: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u32;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: [u8;6];
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    local_6 = (param_2 + 4);
    puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x45);
    uVar3 = puVar5;
    uVar9 = (param_1 >> 0x10);
    uVar7 = param_1;
    local_a = uVar3;
    pass1_1038_4e78((uVar7 + 4), puVar5 & 0xffff | in_DX << 0x10);
    _local_e = CONCAT22(extraout_DX, uVar3);
    ppcVar1 = (*_local_e + 0x10);
    uVar11 = uVar3;
    uVar12 = extraout_DX;
    (**ppcVar1)(&PTR_LOOP_1050_1038, uVar3, extraout_DX);
    _local_12 = CONCAT22(extraout_DX_00, uVar3);
    uVar2 = (uVar7 + 4);
    local_16 = (uVar2 + 8);
    zero_list_1008_3e38(CONCAT22(unaff_SS, &local_1c));
    zero_list_1008_3e38(CONCAT22(unaff_SS, local_22));
    local_2c = 0;
    loop
    {
        if (_local_12 <= local_2c)
        {
// LAB_1030_a4e7:
            if (_local_e != 0x0)
            {
                ppcVar1 = *_local_e;
                (**ppcVar1)(&PTR_LOOP_1050_1008, _local_e, (_local_e >> 0x10), 1, uVar11,
                            uVar12, _local_e, _local_e);
            }
            return;
        }
        uVar6 = _local_12;
        pass1_1030_1d58(_local_e);
        if ((extraout_DX_01 | uVar6) != 0)
        {
            modify_list_1008_3f62(CONCAT22(unaff_SS, &local_1c), CONCAT22(extraout_DX_01, uVar6 + 0xc));
            pass1_1008_3eb4(CONCAT22(unaff_SS, &local_1c), CONCAT22(unaff_SS, &local_28),
                            CONCAT22(unaff_SS, &local_26), CONCAT22(unaff_SS, &local_24));
            if ((local_28 == local_6) &&
                (uVar2 = (uVar7 + 4), uVar10 = (uVar2 >> 0x10),
                 iVar8 = uVar2, uVar2 = (iVar8 + 4),
                 uVar3 = pass1_1030_addc(uVar7, uVar9, CONCAT22(unaff_SS, &local_1c), uVar2,
                                         (uVar2 >> 0x10), (iVar8 + 8)),
                 uVar3 != 0))
            {
                modify_list_1008_3f62(CONCAT22(unaff_SS, local_22), CONCAT22(unaff_SS, &local_1c));
                local_1a = local_26 - 1;
                BVar4 = pass1_1030_ad22(uVar7, uVar9, CONCAT22(unaff_SS, &local_1c), local_16);
                if (BVar4 == 0)
                {
                    local_1a = local_26 + 1;
                    BVar4 = pass1_1030_ad22(uVar7, uVar9, CONCAT22(unaff_SS, &local_1c), local_16);
                    if (BVar4 == 0)
                    {
                        local_1a = local_26;
                        local_1c = local_24 - 1;
                        BVar4 = pass1_1030_ad22(uVar7, uVar9, CONCAT22(unaff_SS, &local_1c), local_16);
                        if (BVar4 == 0)
                        {
                            local_1c = local_24 + 1;
                            BVar4 = pass1_1030_ad22(uVar7, uVar9, CONCAT22(unaff_SS, &local_1c), local_16);
                            if (BVar4 == 0) {}
                              // goto LAB_1030_a45b;
                        }
                    }
                }
                modify_list_1008_3f62(param_2, CONCAT22(unaff_SS, local_22));
                local_4 = 1;
              // goto LAB_1030_a4e7;
            }
        }
// LAB_1030_a45b:
        local_2c = local_2c + 1;
    } 
}





pub fn pass1_1030_a57e(param_1: u32, param_2: *mut u16)

{
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut in_AX: i32;
    let mut uVar5: u16;
    let puVar6: *mut u16;
    let puVar7: *mut u8;
    let mut uVar8: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let local_BX_11: *mut astruct_953;
    let mut uVar9: u16;
    let mut iVar10: i32;
    let puVar11: *mut u32;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut uVar15: u16;
    let mut unaff_SS: u16;
    let mut uVar16: u32;
    let uVar17: u8;
    let mut local_4e: u32;
    let mut local_4a: u32;
    let mut local_46: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: [u8;2];
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    uVar12 = (param_1 >> 0x10);
    local_BX_11 = param_1;
    pass1_1038_53ba(local_BX_11.field_0x4, (&PTR_LOOP_1050_0000 + 1));
    if ((in_DX != 0) || (in_AX != 0))
    {
        local_6 = (param_2 + 4);
        local_8 = 8 - (local_6 == 0);
        puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, local_8);
        uVar5 = puVar7;
        local_c = uVar5;
        pass1_1038_4e78(local_BX_11.field_0x4, puVar7 & 0xffff | in_DX << 0x10);
        _local_10 = CONCAT22(extraout_DX, uVar5);
        uVar15 = SUB42(&PTR_LOOP_1050_1008, 0);
        zero_list_1008_3e38(CONCAT22(unaff_SS, &local_16));
        uVar3 = local_BX_11.field_0x4;
        uVar1 = (uVar3 + 8);
        uVar13 = (_local_10 >> 0x10);
        uVar9 = SUB42(_local_10, 0);
        ppcVar2 = (*_local_10 + 0x10);
        uVar8 = uVar1;
        ppcVar2(&PTR_LOOP_1050_1008, uVar9, uVar13);
        uVar4 = uVar8 & 0xffff | extraout_DX_00 << 0x10;
        local_28 = 0;
        while (local_28 < uVar4)
        {
            uVar16 = uVar4;
            pass1_1030_1d58(_local_10);
            if ((extraout_DX_01 | uVar16) != 0)
            {
                modify_list_1008_3f62(CONCAT22(unaff_SS, &local_16), CONCAT22(extraout_DX_01, uVar16 + 0xc));
                uVar15 = SUB42(&PTR_LOOP_1050_1008, 0);
                pass1_1008_3eb4(CONCAT22(unaff_SS, &local_16), CONCAT22(unaff_SS, local_1c),
                                CONCAT22(unaff_SS, &local_1a), CONCAT22(unaff_SS, &local_18));
                uVar3 = local_BX_11.field_0x4;
                uVar14 = (uVar3 >> 0x10);
                iVar10 = uVar3;
                uVar3 = (iVar10 + 4);
                uVar5 = pass1_1030_addc(local_BX_11, uVar12, CONCAT22(unaff_SS, &local_16),
                                        uVar3, (uVar3 >> 0x10), (iVar10 + 8));
                if (uVar5 == 0) {}
                  // goto LAB_1030_a660;
                uVar16 = pass1_1030_73a8((uVar16 & 0xffff | extraout_DX_01 << 0x10));
                iVar10 = (uVar16 + 0xc);
                if (5 < iVar10 - 0x7a) {}
                  // goto LAB_1030_a660;
                uVar15 = 0x1030;
                match (iVar10)
                {
// default:
 _ => {
                    local_14 = local_1a - 1;
                    puVar6 = &local_16;
                    pass1_1030_ad86(local_BX_11, uVar12, CONCAT22(unaff_SS, puVar6), uVar1);
                    if (puVar6 != 0x0) {}
                      // goto LAB_1030_a7df;
                    local_14 = local_1a + 1;
                    puVar6 = &local_16;
                    pass1_1030_ad86(local_BX_11, uVar12, CONCAT22(unaff_SS, puVar6), uVar1);
                    if (puVar6 == 0x0)
                    {
                        local_14 = local_1a;
                        local_16 = local_18 - 1;
                        puVar6 = &local_16;
                        pass1_1030_ad86(local_BX_11, uVar12, CONCAT22(unaff_SS, puVar6), uVar1);
                      // goto joined_r0x1030a722;
                    }
// LAB_1030_a748:
                    modify_list_1008_3f62(param_2, CONCAT22(unaff_SS, &local_16));
                },
                0x7b |
                0x7e =>{
                    local_14 = local_1a - 1;
                    puVar6 = &local_16;
                    pass1_1030_ad86(local_BX_11, uVar12, CONCAT22(unaff_SS, puVar6), uVar1);
                    if (puVar6 == 0x0)
                    {
                        local_14 = local_1a + 1;
                      // goto LAB_1030_a730;
                    }
                    modify_list_1008_3f62(param_2, CONCAT22(unaff_SS, &local_16));
                    if (_local_10 == 0x0)
                    {
                        return;
                    }
                    uVar15 = (_local_10 >> 0x10);
                    puVar11 = _local_10;
                    uVar17 = (_local_10 >> 0x10);
                  // goto LAB_1030_a6ea;
                },
                0x7c |
                0x7d =>{
                    local_16 = local_18 - 1;
                    puVar6 = &local_16;
                    pass1_1030_ad86(local_BX_11, uVar12, CONCAT22(unaff_SS, puVar6), uVar1);
// joined_r0x1030a722:
                    if (puVar6 == 0x0)
                    {
                        local_16 = local_18 + 1;
// LAB_1030_a730:
                        puVar6 = &local_16;
                        pass1_1030_ad86(local_BX_11, uVar12, CONCAT22(unaff_SS, puVar6), uVar1);
                        if (puVar6 != 0x0) {}
                          // goto LAB_1030_a748;
                      // goto LAB_1030_a660;
                    }
// LAB_1030_a7df:
                    modify_list_1008_3f62(param_2, CONCAT22(unaff_SS, &local_16));
                
                puVar11 = _local_10;
                if ((local_e | puVar11) != 0)
                {
                    uVar15 = (_local_10 >> 0x10);
                    uVar17 = (_local_10 >> 0x10);
// LAB_1030_a6ea:
                    unsafe{ppcVar2 = *puVar11;
                    ppcVar2(&PTR_LOOP_1050_1008, puVar11, uVar17, 1, uVar9, uVar13, _local_10, _local_10);}
                }
                return;
            } }
// LAB_1030_a660:
            local_28 = local_28 + 1;
        }
        if (_local_10 != 0x0)
        {
            ppcVar2 = *_local_10;
            ppcVar2(uVar15, _local_10, (_local_10 >> 0x10), 1, uVar9, uVar13, _local_10,
                        _local_10);
        }
    }
    return;
}




pub fn pass1_1030_a844(param_1: *mut astruct_954, param_2: *mut u16)

{
    let mut uVar1: u16;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut in_AX: i32;
    let mut uVar4: i32;
    let mut uVar5: u16;
    let puVar6: *mut u16;
    let puVar7: *mut u32;
    let mut uVar8: u32;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let local_BX_6: *mut astruct_954;
    let mut iVar9: i32;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut unaff_SS: u16;
    let mut local_46: u16;
    let mut local_3c: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar10 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    pass1_1038_53ba(local_BX_6.field_0x4, (&PTR_LOOP_1050_0000 + 1));
    if ((in_DX != 0) || (in_AX != 0))
    {
        uVar3 = local_BX_6.field_0x4;
        uVar11 = (uVar3 >> 0x10);
        iVar9 = uVar3;
        puVar7 = (iVar9 + 0xc);
        ppcVar2 = (*puVar7 + 0x10);
        local_6 = puVar7;
        ppcVar2(&PTR_LOOP_1050_1038, puVar7, (iVar9 + 0xe));
        _local_a = puVar7 & 0xffff | extraout_DX << 0x10;
        uVar3 = local_BX_6.field_0x4;
        local_e = (uVar3 + 8);
        local_10 = 0;
        zero_list_1008_3e38(CONCAT22(unaff_SS, &local_16));
        uVar1 = (param_2 + 4);
        local_22 = 0;
        while (local_22 < _local_a)
        {
            uVar8 = _local_a;
            pass1_1030_1d7c(local_6, local_22, (local_22 >> 0x10));
            if ((((extraout_DX_01 | uVar8) != 0) &&
                 (uVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar8 + 0xc), 0x46), uVar4 != 0)) &&
                (pass1_1030_1d58(local_6), (extraout_DX_00 | uVar4) != 0))
            {
                modify_list_1008_3f62(CONCAT22(unaff_SS, &local_16), CONCAT22(extraout_DX_00, uVar4 + 0xc));
                pass1_1008_3eb4(CONCAT22(unaff_SS, &local_16), CONCAT22(unaff_SS, &local_1c),
                                CONCAT22(unaff_SS, &local_1a), CONCAT22(unaff_SS, &local_18));
                if ((uVar1 == local_1c) &&
                    (uVar3 = local_BX_6.field_0x4, uVar11 = (uVar3 >> 0x10),
                     iVar9 = uVar3, uVar3 = (iVar9 + 4),
                     uVar5 = pass1_1030_addc(local_BX_6, uVar10, CONCAT22(unaff_SS, &local_16),
                                             uVar3, (uVar3 >> 0x10), (iVar9 + 8)),
                     uVar5 != 0))
                {
                    local_14 = local_1a - 1;
                    puVar6 = &local_16;
                    pass1_1030_ad86(local_BX_6, uVar10, CONCAT22(unaff_SS, puVar6), local_e);
                    if (puVar6 != 0x0)
                    {
// LAB_1030_a98e:
                        modify_list_1008_3f62(param_2, CONCAT22(unaff_SS, &local_16));
                        return;
                    }
                    local_14 = local_1a + 1;
                    puVar6 = &local_16;
                    pass1_1030_ad86(local_BX_6, uVar10, CONCAT22(unaff_SS, puVar6), local_e);
                    if (puVar6 != 0x0) {}
                      // goto LAB_1030_a98e;
                    local_14 = local_1a;
                    local_16 = local_18 - 1;
                    puVar6 = &local_16;
                    pass1_1030_ad86(local_BX_6, uVar10, CONCAT22(unaff_SS, puVar6), local_e);
                    if (puVar6 != 0x0) {}
                      // goto LAB_1030_a98e;
                    local_16 = local_18 + 1;
                    puVar6 = &local_16;
                    pass1_1030_ad86(local_BX_6, uVar10, CONCAT22(unaff_SS, puVar6), local_e);
                    if (puVar6 != 0x0) {}
                      // goto LAB_1030_a98e;
                }
            }
            local_22 = local_22 + 1;
        }
    }
    return;
}





pub fn pass1_1030_aa18(param_1: u32, param_2: *mut u16)

{
    let mut u_var1: u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut uVar4: u16;
    let puVar5: *mut u8;
    let mut uVar6: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut iVar9: i32;
    let puVar10: *mut u32;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut unaff_SS: u16;
    let mut uVar15: u32;
    let uVar16: u8;
    let mut local_4c: u32;
    let mut local_48: u32;
    let mut local_44: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u32;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: [u8;2];
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = (param_2 + 4);
    local_6 = 8 - (local_4 == 0);
    puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, local_6);
    uVar4 = puVar5;
    uVar11 = (param_1 >> 0x10);
    uVar7 = param_1;
    local_a = uVar4;
    pass1_1038_4e78((uVar7 + 4), puVar5 & 0xffff | in_DX << 0x10);
    _local_e = CONCAT22(extraout_DX, uVar4);
    uVar14 = SUB42(&PTR_LOOP_1050_1008, 0);
    zero_list_1008_3e38(CONCAT22(unaff_SS, &local_14));
    uVar3 = (uVar7 + 4);
    uVar1 = (uVar3 + 8);
    uVar12 = (_local_e >> 0x10);
    uVar8 = SUB42(_local_e, 0);
    ppcVar2 = (*_local_e + 0x10);
    uVar6 = uVar1;
    ppcVar2(&PTR_LOOP_1050_1008, uVar8, uVar12);
    uVar6 = uVar6 & 0xffff | extraout_DX_00 << 0x10;
    local_26 = 0;
    loop
    {
        if (uVar6 <= local_26)
        {
            if (_local_e != 0x0)
            {
                ppcVar2 = *_local_e;
                ppcVar2(uVar14, _local_e, (_local_e >> 0x10), 1, uVar8, uVar12, _local_e,
                            _local_e);
            }
            return;
        }
        uVar15 = uVar6;
        pass1_1030_1d58(_local_e);
        if ((extraout_DX_01 | uVar15) != 0) {
            break;}
// LAB_1030_aadc:
        local_26 = local_26 + 1;
    }
    modify_list_1008_3f62(CONCAT22(unaff_SS, &local_14), CONCAT22(extraout_DX_01, uVar15 + 0xc));
    uVar14 = SUB42(&PTR_LOOP_1050_1008, 0);
    pass1_1008_3eb4(CONCAT22(unaff_SS, &local_14), CONCAT22(unaff_SS, local_1a),
                    CONCAT22(unaff_SS, &local_18), CONCAT22(unaff_SS, &local_16));
    uVar3 = (uVar7 + 4);
    uVar13 = (uVar3 >> 0x10);
    iVar9 = uVar3;
    uVar3 = (iVar9 + 4);
    uVar4 = pass1_1030_addc(uVar7, uVar11, CONCAT22(unaff_SS, &local_14), uVar3,
                            (uVar3 >> 0x10), (iVar9 + 8));
    if (uVar4 == 0) {}
      // goto LAB_1030_aadc;
    uVar15 = pass1_1030_73a8((uVar15 & 0xffff | extraout_DX_01 << 0x10));
    iVar9 = (uVar15 + 0xc);
    if (5 < iVar9 - 0x7a) {}
      // goto LAB_1030_aadc;
    uVar14 = 0x1030;
    match (iVar9)
    {
// default:
     _ => {
        local_12 = local_18 - 1;
        uVar4 = pass1_1030_acbe(uVar7, uVar11, CONCAT22(unaff_SS, &local_14), uVar1);
        if (uVar4 != 0) {}
          // goto LAB_1030_ac5b;
        local_12 = local_18 + 1;
        uVar4 = pass1_1030_acbe(uVar7, uVar11, CONCAT22(unaff_SS, &local_14), uVar1);
        if (uVar4 == 0)
        {
            local_12 = local_18;
            local_14 = local_16 - 1;
            uVar4 = pass1_1030_acbe(uVar7, uVar11, CONCAT22(unaff_SS, &local_14), uVar1);
          // goto joined_r0x1030ab9e;
        }
    
// LAB_1030_abc4:
        modify_list_1008_3f62(param_2, CONCAT22(unaff_SS, &local_14))},
    0x7b |
    0x7e =>{
        local_12 = local_18 - 1;
        uVar4 = pass1_1030_acbe(uVar7, uVar11, CONCAT22(unaff_SS, &local_14), uVar1);
        if (uVar4 == 0)
        {
            local_12 = local_18 + 1;
          // goto LAB_1030_abac;
        }
        modify_list_1008_3f62(param_2, CONCAT22(unaff_SS, &local_14));
        if (_local_e == 0x0)
        {
            return;
        }
        uVar14 = (_local_e >> 0x10);
        puVar10 = _local_e;
        uVar16 = (_local_e >> 0x10);
      // goto LAB_1030_ab66;
    },
    0x7c |
    0x7d =>{
        local_14 = local_16 - 1;
        uVar4 = pass1_1030_acbe(uVar7, uVar11, CONCAT22(unaff_SS, &local_14), uVar1);
// joined_r0x1030ab9e:
        if (uVar4 == 0)
        {
            local_14 = local_16 + 1;
// LAB_1030_abac:
            uVar4 = pass1_1030_acbe(uVar7, uVar11, CONCAT22(unaff_SS, &local_14), uVar1);
            if (uVar4 != 0) {}
              // goto LAB_1030_abc4;
          // goto LAB_1030_aadc;
        }
// LAB_1030_ac5b:
        modify_list_1008_3f62(param_2, CONCAT22(unaff_SS, &local_14));
    
    
    puVar10 = _local_e;
    if ((local_c | puVar10) != 0)
    {
        uVar14 = (_local_e >> 0x10);
        uVar16 = (_local_e >> 0x10);
// LAB_1030_ab66:
        ppcVar2 = *puVar10;
        ppcVar2(&PTR_LOOP_1050_1008, puVar10, uVar16, 1, uVar8, uVar12, _local_e, _local_e);
    } }}
    return;
}



pub fn pass1_1030_acbe(param_1: u16, param_2: u16,param_1_00: u32,param_2_00: u32)

{
    let mut iVar1: i32;
    let paVar2: *mut astruct_493;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let lVar5: u32;
    let mut uVar6: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    lVar5 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_1_00, param_2_00);
    uVar3 = (lVar5 >> 0x10);
    uVar4 = uVar3 | lVar5;
    if (lVar5 != 0)
    {
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar5, uVar3);
        if ((uVar4 | paVar2) != 0)
        {
            uVar6 = pass1_1030_73a8(CONCAT22(uVar4, paVar2));
            if ((uVar6 != 0) && ((iVar1 = (uVar6 + 0xc), iVar1 == 5 || (iVar1 == 9))))
            {
                return 1;
            }
        }
    }
    return 0;
}



pub fn pass1_1030_ad22(param_1: u16, param_2: u16,param_1_00: u32,param_2_00: u32) -> bool

{
    let paVar1: *mut astruct_493;
    let BVar2: bool;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let lVar5: u32;
    let mut uVar6: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    lVar5 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_1_00, param_2_00);
    uVar3 = (lVar5 >> 0x10);
    uVar4 = uVar3 | lVar5;
    if (lVar5 != 0)
    {
        paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar5, uVar3);
        if ((uVar4 | paVar1) != 0)
        {
            uVar6 = pass1_1030_73a8(CONCAT22(uVar4, paVar1));
            if (uVar6 != 0)
            {
                BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar6 + 0xc), 0x46);
                return BVar2;
            }
        }
    }
    return 0;
}




pub fn pass1_1030_ad86(param_1: u16, param_2: u16,param_1_00: u32,param_2_00: u32)

{
    let mut u_var1: u32;
    let puVar2: *mut u32;
    let mut extraout_DX: u16;
    let mut unaff_SS: u16;
    let mut local_14: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    puVar2 = &local_a;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_1_00, param_2_00, puVar2, unaff_SS);
    uVar1 = *puVar2;
    local_14._3_1_ = (uVar1 >> 0x18);
    if (local_14._3_1_ == '\0')
    {
        local_6._0_2_ = uVar1;
        if (((0 < local_6) && (!SBORROW2(local_6, 1))) &&
            ((local_6 == 3 || local_6 + -2 < 1 ||
              ((3 < local_6 + -3 && (local_6 + -7 < 2))))))
        {
            return;
        }
    }
    return;
}




pub fn pass1_1030_addc(param_1: u16, param_2: u16, param_1_00: *mut u16, param_4: u16, param_5: u16,param_2_00: u32) -> i32

{
    let mut in_DX: u16;
    let mut unaff_SS: u16;
    let puVar1: *mut u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    puVar1 = pass1_1030_5b5c(local_6, in_DX);
    local_c = *puVar1;
    uStack8 = (puVar1 + 4);
    pass1_1008_3e94(param_1_00, CONCAT22(unaff_SS, &local_10),
                    CONCAT22(unaff_SS, &local_e));
    pass1_1008_3e94(&local_c, CONCAT22(unaff_SS, &local_14),
                    CONCAT22(unaff_SS, &local_12));
    if ((((1 < local_e) && (1 < local_10)) && (local_e < (local_12 - 1))) &&
        (local_10 < (local_14 - 1)))
    {
        return 1;
    }
    return 0;
}

pub fn pass1_1030_ae6c(param_1: *mut u16)

{
    let mut u_var1: u32;
    let mut uVar2: i32;
    let paVar3: *mut astruct_199;
    let in_DX: *mut astruct_199;
    let mut uVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut local_8: u16;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = s_1_1050_389a;
    (iVar5 + 2) = &PTR_LOOP_1050_1008;
    (iVar5 + 4) = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | (iVar5 + 8)));
    uVar2 = 0;
    (iVar5 + 0xe) = 0;
    (iVar5 + 0x10) = 0;
    *param_1 = 0xb932;
    (iVar5 + 2) = 0x1030;
    process_struct_1000_179c(0xc, in_DX);
    uVar4 = in_DX | uVar2;
    if (uVar4 == 0)
    {
        (iVar5 + 0x10) = 0;
    }
    else
    {
        paVar3 = process_struct_1008_574a(CONCAT22(in_DX, uVar2));
        (iVar5 + 0x10) = paVar3;
        (iVar5 + 0x12) = uVar4;
    }
    uVar1 = (iVar5 + 0x10);
    (uVar1 + 10) = 0;
    return;
}

pub fn pass1_1030_aefa(param_1: *mut astruct_955,param_2: u32)

{
    let mut u_var1: u32;
    let mut uVar2: i32;
    let paVar3: *mut astruct_199;
    let puVar4: *mut u16;
    let struct_a: *mut astruct_199;
    let mut uVar5: i32;
    let local_BX_4: *mut astruct_955;
    let mut uVar6: u16;
    let mut local_c: u16;

    uVar6 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_BX_4.field_0x8));
    local_BX_4.field_0xe = 0;
    &local_BX_4.field_0x10 = 0;
    param_1 = 0xb932;
    local_BX_4.field_0x2 = 0x1030;
    local_BX_4.field_0x4 = (param_2 + 4);
    puVar4 = (param_1 & 0xffff0000 | &local_BX_4.field_0x8);
    modify_list_1008_3f62(puVar4, param_2 & 0xffff0000 | (param_2 + 0xc));
    uVar2 = puVar4;
    paVar3 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    uVar5 = paVar3 | uVar2;
    if (uVar5 == 0)
    {
        paVar3 = 0x0;
        uVar5 = 0;
    }
    else
    {
        paVar3 = process_struct_1008_574a(CONCAT22(paVar3, uVar2));
    }
    local_BX_4.field_0x10 = paVar3;
    &local_BX_4.field_0x12 = uVar5;
    uVar1 = &local_BX_4.field_0x10;
    (uVar1 + 10) = 0;
    return;
}

pub fn pass1_1030_afa6(param_1: *mut astruct_956)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut uVar4: u32;
    let local_BX_4: *mut astruct_956;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = 0xb932;
    local_BX_4.field_0x2 = 0x1030;
    if (&local_BX_4.field_0x10 != 0)
    {
        uVar4 = &local_BX_4.field_0x10;
        (uVar4 + 10) = 1;
    }
    puVar1 = local_BX_4.field_0x10;
    uVar2 = local_BX_4.field_0x12;
    if ((uVar2 | puVar1) != 0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}




pub fn pass1_1030_affc(param_1: *mut astruct_960)

{
    let mut iVar1: i32;
    let mut uVar2: u16;
    let paVar3: *mut astruct_493;
    let paVar4: *mut astruct_493;
    let mut uVar5: i32;
    let mut extraout_DX: i32;
    let mut unaff_SS: u16;
    let mut uVar6: u32;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let temp_5f46e7a27c: *mut astruct_957;

    pass1_1030_b718(param_1, param_1._2_2_, param_1 & 0xffff0000 | (param_1 + 8), CONCAT22(unaff_SS, &local_6));
    paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_6, (local_6 >> 0x10));
    _local_a = CONCAT22(param_1._2_2_, paVar3);
    uVar5 = param_1._2_2_ | paVar3;
    if (uVar5 != 0)
    {
        uVar6 = pass1_1030_73a8(CONCAT22(param_1._2_2_, paVar3));
        uVar5 = (uVar6 >> 0x10);
        temp_5f46e7a27c = (uVar6 + 0xc);
        paVar4 = (temp_5f46e7a27c + -0x16);
        paVar3 = paVar4;
        if (((0x15 < temp_5f46e7a27c) && (!SBORROW2(temp_5f46e7a27c, 0x16))) &&
            ((paVar3 = (temp_5f46e7a27c + -0x17),
              paVar3 == 0x0 || paVar4 < 1 ||
                  ((paVar3 = (temp_5f46e7a27c + -0x19), 0 < (temp_5f46e7a27c + -0x18) &&
                                                                           (paVar3 = (temp_5f46e7a27c + -0x1a),
                                                                            paVar3 == 0x0 || (temp_5f46e7a27c + -0x19) < 1))))))
        {
            (uVar6 + 0x20) = 0;
        }
    }
    local_c = 6;
    loop
    {
        if (local_c == 0)
        {
// LAB_1030_b0fc:
            if ((local_8 | local_a) != 0)
            {
                uVar6 = pass1_1030_73a8(_local_a);
                uVar2 = (uVar6 >> 0x10);
                iVar1 = (uVar6 + 0xc);
                if (((0x15 < iVar1) && (!SBORROW2(iVar1, 0x16))) &&
                    ((iVar1 == 0x17 || iVar1 + -0x16 < 1 || ((0 < iVar1 + -0x18 && (iVar1 + -0x19 < 2))))))
                {
                    (uVar6 + 0x20) = 1;
                }
            }
            return;
        }
        pass1_1030_b578(param_1);
        if ((uVar5 | paVar3) == 0) {}
          // goto LAB_1030_b0fc;
        _local_a = CONCAT22(uVar5, paVar3);
        uVar6 = pass1_1030_73a8(CONCAT22(uVar5, paVar3));
        iVar1 = (uVar6 + 0xc);
        modify_list_1008_3f62((param_1 & 0xffff0000 | (param_1 + 8)),
                              CONCAT22(uVar5, &paVar3.field_0xc));
        uVar5 = extraout_DX;
        if ((iVar1 == 0x18) || (iVar1 == 0x3f))
        {
            pass1_1030_b142(param_1, _local_a);
        }
        paVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar1, 0x40);
        if (paVar3 != 0x0)
        {
            pass1_1030_b454(param_1, _local_a);
          // goto LAB_1030_b0fc;
        }
        local_c = local_c - 1;
    } 
}

pub fn ret_1030_b13c()

{
    return;
}



pub fn pass1_1030_b142(param_1: *mut astruct_959, param_2: *mut astruct_493)

{
    let mut iVar1: i32;
    let mut uVar2: u16;
    let local_BX_100: *mut astruct_959;
    let local_ES_100: *mut astruct_959;
    let mut bVar3: bool;
    let mut uVar4: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff85ff504d: *mut astruct_958;

    uVar4 = pass1_1030_73a8(param_2);
    uVar2 = (uVar4 >> 0x10);
    temp_7ff85ff504d = uVar4;
    iVar1 = temp_7ff85ff504d.field_0xc;
    local_c = 0;
    if (iVar1 == 0x18)
    {
        local_c._2_2_ = return_false_1028_1c1c();
        uVar2 = temp_7ff85ff504d.field_0x22;
    }
    else
    {
        if (iVar1 != 0x3f) {}
          // goto LAB_1030_b1a6;
        local_c._2_2_ = return_false_1028_20b0();
        uVar2 = temp_7ff85ff504d.field_0x24;
    }
    local_c = CONCAT22(local_c._2_2_, uVar2);
// LAB_1030_b1a6:
    local_ES_100 = (param_1 >> 0x10);
    local_BX_100 = param_1;
    if (local_BX_100.field_0xe == 1)
    {
        bVar3 = (local_c & 0x10000) == 0;
    }
    else
    {
        if (local_BX_100.field_0xe == 2)
        {
            bVar3 = (local_c & 0x20000) == 0;
        }
        else
        {
            if (local_BX_100.field_0xe == 3)
            {
                bVar3 = (local_c & 0x40000) == 0;
            }
            else
            {
                bVar3 = (local_c & 0x80000) == 0;
            }
        }
    }
    if ((bVar3) || (local_c != 0))
    {
        bVar3 = false;
        loop
        {
            if (((local_c & 0x10000) != 0) && (local_c == 0)) {}
              // goto LAB_1030_b239;
            if (((local_c & 0x20000) != 0) && (local_c == 0)) {}
              // goto LAB_1030_b247;
            if (((local_c & 0x40000) != 0) && (local_c == 0)) {}
              // goto LAB_1030_b255;
            if (((local_c & 0x80000) != 0) && (local_c == 0)) {}
              // goto LAB_1030_b263;
            if (bVar3) {
                break; }
            local_c._1_3_ = (local_c >> 8) & 0xffff00;
            iVar1 = local_BX_100.field_0xe;
            if (iVar1 == 1)
            {
                local_c = CONCAT31(local_c._1_3_, 4);
            }
            else
            {
                if (iVar1 == 2)
                {
                    local_c = CONCAT31(local_c._1_3_, 8);
                }
                else
                {
                    if (iVar1 == 3)
                    {
                        local_c = CONCAT31(local_c._1_3_, 1);
                    }
                    else
                    {
                        local_c = CONCAT31(local_c._1_3_, 2);
                    }
                }
            }
            bVar3 = true;
        }
        if (local_BX_100.field_0xe == 1)
        {
// LAB_1030_b255:
            local_BX_100.field_0xe = 3;
            return;
        }
        if (local_BX_100.field_0xe == 2)
        {
// LAB_1030_b263:
            local_BX_100.field_0xe = 4;
            return;
        }
        if (local_BX_100.field_0xe == 3)
        {
// LAB_1030_b239:
            local_BX_100.field_0xe = 1;
            return;
        }
        if (local_BX_100.field_0xe == 4)
        {
// LAB_1030_b247:
            local_BX_100.field_0xe = 2;
            return;
        }
    }
    return;
}




pub fn pass1_1030_b2aa(param_1: u32,param_2: u32)

{
    let paVar1: *mut astruct_493;
    let BVar2: bool;
    let mut unaff_SS: u16;
    let mut uVar3: u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_6: u32;

    pass1_1030_b718(param_1, (param_1 >> 0x10), param_2, CONCAT22(unaff_SS, &local_6));
    local_1a._3_1_ = (local_6 >> 0x18);
    if (local_1a._3_1_ != '\0')
    {
        paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_6, local_6._2_2_);
        if ((local_6._2_2_ | paVar1) != 0)
        {
            uVar3 = pass1_1030_73a8(CONCAT22(local_6._2_2_, paVar1));
            BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar3 + 0xc), 0x42);
            if (BVar2 != 0)
            {
                modify_list_1008_3f62((param_1 & 0xffff0000 | (param_1 + 8)),
                                      CONCAT22(local_6._2_2_, &paVar1.field_0xc));
                return;
            }
        }
    }
    return;
}

pub fn pass1_1030_b344(param_1: *mut u8)

{
    let mut extraout_DX: i32;
    let mut uVar1: i32;
    let mut unaff_SS: u16;
    let mut local_1c: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8;2];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    _local_8 = (param_1 + 8);
    uStack4 = (param_1 + 0xc);
    pass1_1008_3eb4(CONCAT22(unaff_SS, &local_8), CONCAT22(unaff_SS, local_e),
                    CONCAT22(unaff_SS, &local_c), CONCAT22(unaff_SS, &local_a));
    _local_8 = _local_8 & 0xffff | (local_c - 1) << 0x10;
    local_12 = &local_8;
    uVar1 = extraout_DX;
    pass1_1030_b2aa(param_1, CONCAT22(unaff_SS, local_12));
    local_10 = uVar1 | local_12;
    if (local_10 == 0)
    {
        _local_8 = _local_8 & 0xffff | (local_c + 1) << 0x10;
        local_12 = &local_8;
        pass1_1030_b2aa(param_1, CONCAT22(unaff_SS, local_12));
        uVar1 = local_10 | local_12;
        if (uVar1 == 0)
        {
            local_8 = local_a - 1;
            local_6 = local_c;
            local_12 = &local_8;
            pass1_1030_b2aa(param_1, CONCAT22(unaff_SS, local_12));
            local_10 = uVar1 | local_12;
            if (local_10 == 0)
            {
                _local_8 = CONCAT22(local_6, local_a + 1);
                local_12 = &local_8;
                pass1_1030_b2aa(param_1, CONCAT22(unaff_SS, local_12));
                if ((local_10 | local_12) == 0)
                {
                    return 0;
                }
                (param_1 + 0xe) = 2;
            }
            else
            {
                (param_1 + 0xe) = 4;
                local_10 = uVar1;
            }
        }
        else
        {
            (param_1 + 0xe) = 3;
        }
    }
    else
    {
        (param_1 + 0xe) = 1;
        local_10 = uVar1;
    }
    return CONCAT22(local_10, local_12);
}




pub fn pass1_1030_b454(param_1: *mut astruct_960, param_2: *mut astruct_493)

{
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let local_AX_39: *mut u8;
    let puVar3: *mut u8;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let local_BX_16: *mut astruct_960;
    let mut local_ES_16: u16;
    let mut local_SS__1: u16;
    let mut uVar4: u32;
    let mut local_32: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: [u8;4];
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let temp_7e074184931: *mut u32;
    let mut temp_5f5d5590f3: u32;

    local_6 = (param_2 + 4);
    local_ES_16 = (param_1 >> 0x10);
    local_BX_16 = param_1;
    local_32 = CONCAT22(local_SS__1, local_12);
    pass1_1008_5784(local_32, local_BX_16.field_0x10);
    loop
    {
        local_AX_39 = local_12;
        pass1_1008_5b12(CONCAT22(local_SS__1, local_AX_39));
        _local_a = CONCAT22(extraout_DX, local_AX_39);
            break; }
        if ((local_AX_39 + 0x20) == local_6)
        {
            ppcVar2 = (local_BX_16.field_0x10 + 0xc);
            ppcVar2();
            local_e = 0;
            pass1_1038_69fe(_local_a);
        }
    }
    uVar4 = pass1_1030_73a8(param_2);
    puVar1 = (uVar4 + 0x20);
    puVar3 = local_12;
    local_32 = CONCAT22(local_SS__1, puVar3);
    pass1_1008_5784(local_32, puVar1);
    ret_1030_b13c(param_1);
    _local_1e = CONCAT22(-((s_Unsupported_FileStructType_in_Op_1050_01ca + 0x2a) < puVar3) - extraout_DX_00, 500 - puVar3);
    while
    {
        puVar3 = local_12;
        pass1_1008_5b12(CONCAT22(local_SS__1, puVar3));
        _local_a = CONCAT22(extraout_DX_01, puVar3);
        if ((extraout_DX_01 | puVar3) == 0)
        {
            return;
        }
        pass1_1038_6984(CONCAT22(extraout_DX_01, puVar3));
        _local_26 = CONCAT22(extraout_DX_02, puVar3);
        if ((extraout_DX_02 <= local_1c) &&
            ((extraout_DX_02 < local_1c || (puVar3 <= local_1e))))
        {
            temp_5f5d5590f3 = local_BX_16.field_0x10;
            ppcVar2 = (local_BX_16.field_0x10 + 8);
            ppcVar2();
            _local_1e = _local_1e - _local_26;
            ppcVar2 = (*puVar1 + 0xc);
            ppcVar2(&PTR_LOOP_1050_1038, puVar1, (puVar1 >> 0x10), _local_a,
                        temp_5f5d5590f3);
            local_e = 0;
            0 < _local_1e
        }
    } {}
    return;
}




pub fn pass1_1030_b578(param_1: u32)

{
    let mut iVar1: i32;
    let p_uvar2: *mut u16 ;
    let mut uVar3: i32;
    let mut extraout_DX: i32;
    let mut unaff_SS: u16;
    let mut bVar4: bool;
    let mut uVar5: u32;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u32;
    let mut local_1c: [u8;2];
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut uStack18: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1030_b718(param_1, param_1._2_2_, param_1 & 0xffff0000 | (param_1 + 8),
                    CONCAT22(unaff_SS, &local_6));
    local_30._3_1_ = (local_6 >> 0x18);
    if (local_30._3_1_ == '\0')
    {
        return;
    }
    local_a = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_6, local_6._2_2_);
    local_8 = local_6._2_2_;
    _local_e = pass1_1030_73a8(CONCAT22(local_6._2_2_, local_a));
    local_10 = (_local_e + 0xc);
    _local_16 = (param_1 + 8);
    uStack18 = (param_1 + 0xc);
    pass1_1008_3eb4(CONCAT22(unaff_SS, &local_16), CONCAT22(unaff_SS, local_1c),
                    CONCAT22(unaff_SS, &local_1a), CONCAT22(unaff_SS, &local_18));
    iVar1 = (param_1 + 0xe);
    if (iVar1 == 0)
    {
        pass1_1030_b344((param_1 & 0xffff | param_1._2_2_ << 0x10));
        return;
    }
    if (iVar1 == 1)
    {
        uVar3 = local_1a - 1;
// LAB_1030_b63e:
        _local_16 = _local_16 & 0xffff | uVar3 << 0x10;
        puVar2 = &local_16;
        uVar3 = extraout_DX;
        pass1_1030_b2aa(param_1 & 0xffff | param_1._2_2_ << 0x10, CONCAT22(unaff_SS, puVar2));
        local_30 = CONCAT22(uVar3, puVar2);
        if ((uVar3 | puVar2) == 0)
        {
            return;
        }
        uVar5 = pass1_1030_73a8(CONCAT22(uVar3, puVar2));
        uVar3 = (uVar5 + 0xc);
        if (uVar3 == 0x3f) {}
          // goto LAB_1030_b6e0;
        if (0x3f < uVar3)
        {
            return;
        }
        if (uVar3 == '\x16'){}
          // goto LAB_1030_b6e0;
        bVar4 = uVar3 == '\x18';
    }
    else
    {
        if (iVar1 == 2)
        {
            uVar3 = local_18 + 1;
        }
        else
        {
            if (iVar1 == 3)
            {
                uVar3 = local_1a + 1;
              // goto LAB_1030_b63e;
            }
            if (iVar1 != 4)
            {
                return;
            }
            uVar3 = local_18 - 1;
        }
        _local_16 = _local_16 & 0xffff0000 | uVar3;
        puVar2 = &local_16;
        uVar3 = extraout_DX;
        pass1_1030_b2aa(param_1 & 0xffff | param_1._2_2_ << 0x10, CONCAT22(unaff_SS, puVar2));
        local_30 = CONCAT22(uVar3, puVar2);
        if ((uVar3 | puVar2) == 0)
        {
            return;
        }
        uVar5 = pass1_1030_73a8(CONCAT22(uVar3, puVar2));
        iVar1 = (uVar5 + 0xc);
        if (iVar1 < 0x17)
        {
            return;
        }
        if (SBORROW2(iVar1, 0x17))
        {
            return;
        }
        if (iVar1 == 0x18 || iVar1 + -0x17 < 1){}
          // goto LAB_1030_b6e0;
        bVar4 = iVar1 == 0x3f;
    }
    if (!bVar4)
    {
        return;
    }
// LAB_1030_b6e0:
    modify_list_1008_3f62((param_1 & 0xffff0000 | (param_1 + 8)),
                          local_30 & 0xffff0000 | (local_30 + 0xc));
    return;
}



pub fn pass1_1030_8cd2(param_1: *mut astruct_104, param_2: u16)

{
    let mut local_6: u32;

    local_6._0_1_ = '\0';
    if ((-1 < param_2) && (param_2 < 6))
    {
        local_6._0_1_ =  * (param_1 + 0x38 + param_2 * 4);
    }
    return local_6;
}




pub fn pass1_1030_8d08(param_1: u32)

{
    let puVar1: *mut u16;
    let mut uVar2: u32;
    let mut uvar3: u16;
    let paVar4: *mut astruct_493;
    let mut in_DX: u16;
    let mut uVar5: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    loop
    {
        uVar5 = (param_1 >> 0x10);
        puVar1 = (param_1 + 0x1e);
        let pu_var1_val = unsafe{*puVar1};
        if (pu_var1_val == local_4 || pu_var1_val < local_4){
            break;}
        uVar3 = local_4 * 6;
        uVar2 = (param_1 + 0x1a);
        (uVar2 + uVar3 + 4) = 0;
        pass1_1028_e2ac(_PTR_LOOP_1050_65e2);
        paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, in_DX);
        _local_10 = CONCAT22(in_DX, paVar4);
        pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
        pass1_1030_7e5a(_local_10, CONCAT22(in_DX, paVar4));
        local_4 = local_4 + 1;
    }
    return;
}



pub fn pass1_1030_8d9e(param_1: *mut astruct_104, param_2: u16, param_3: u16)

{
    let mut unaff_SS: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: [u8;6];

    zero_list_1008_3e38(CONCAT22(unaff_SS, local_8));
    modify_list_1008_6d64(param_1 & 0xffff0000 | (param_1 + 0x28), local_8, unaff_SS);
    pass1_1008_3e94(local_8, CONCAT22(unaff_SS, &local_c),
                    CONCAT22(unaff_SS, &local_a));
    return (local_a * param_2 + param_3);
}

pub fn pass1_1030_8e12(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_8a2c(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1030_8e3c(param_1: u32,param_2: u32)

{
    let mut iVar1: i32;
    let mut in_AX: i32;
    let paVar2: *mut astruct_199;
    let paVar3: *mut astruct_493;
    let in_DX: *mut astruct_199;
    let mut uVar4: i32;
    let mut uVar5: u16;
    let ppVar6: *mut pass1_struct_1;
    let mut uVar7: u16;
    let mut in_stack_0000ffe2: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0xc, in_DX);
    uVar4 = in_DX | in_AX;
    if (uVar4 == 0)
    {
        paVar2 = 0x0;
        uVar4 = 0;
    }
    else
    {
        paVar2 = process_struct_1008_574a(CONCAT22(in_DX, in_AX));
    }
    if (param_2._3_1_ == 0x4)
    {
        ppVar6 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffe2, 0x2f));
        uVar5 = (ppVar6 >> 0x10);
        iVar1 = (ppVar6 + 0x1e);
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2, (param_2 >> 0x10));
        uVar7 = (param_1 >> 0x10);
        if (iVar1 < 1)
        {
            pass1_1030_9296(param_1, CONCAT22(uVar4, paVar2), CONCAT22(uVar5, paVar3));
            pass1_1030_951a(param_1, CONCAT22(uVar4, paVar2), CONCAT22(uVar5, paVar3));
        }
        else
        {
            pass1_1030_9adc(param_1, uVar7, CONCAT22(uVar4, paVar2), CONCAT22(uVar5, paVar3));
            pass1_1030_9c1c(param_1, CONCAT22(uVar4, paVar2), CONCAT22(uVar5, paVar3));
        }
        pass1_1030_9d42(param_1, uVar7, CONCAT22(uVar4, paVar2), CONCAT22(uVar5, paVar3));
    }
    return CONCAT22(uVar4, paVar2);
}




pub fn pass1_1030_8f04(param_1: u16, param_2: u16,param_1_00: u32)

{
    let mut uVar1: i32;
    let mut uVar2: i32;
    let mut in_EAX: u32;
    let mut in_DX: i32;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let mut uVar5: u32;
    let mut uVar6: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1038_53ba(param_1_00, (&PTR_LOOP_1050_0000 + 1));
    if ((((in_DX != 0) || (1 < in_EAX)) &&
         ((pass1_1038_53ba(param_1_00, &dos_alloc_addr_1050_0002), in_DX != 0 ||
                                                                                             (1 < in_EAX)))) &&
        ((pass1_1038_53ba(param_1_00, (&dos_alloc_addr_1050_0002 + 1)),
          in_DX != 0 || (1 < in_EAX))))
    {
        pass1_1038_53ba(param_1_00, &PTR_DAT_0005_0000_1050_0004);
        uVar5 = in_DX;
        if ((in_DX != 0) || (1 < in_EAX))
        {
            pass1_1038_540a(param_1_00, 0x21);
            _local_6 = in_EAX & 0xffff | uVar5 << 0x10;
            local_8 = 0;
            loop
            {
                uVar3 = uVar5;
                uVar2 = in_EAX;
                if (0 < (local_8 * 2 + _PTR_LOOP_1050_580e))
                {
                    pass1_1038_540a(param_1_00, local_8);
                    uVar6 = (_PTR_LOOP_1050_580e >> 0x10);
                    uVar1 = (local_8 * 2 + _PTR_LOOP_1050_580e);
                    in_EAX = uVar1;
                    uVar4 = uVar1 >> 0xf;
                    uVar5 = uVar4;
                    if ((uVar3 <= uVar4) && ((uVar3 < uVar4 || (uVar2 < uVar1))))
                    {
                        if (0x1c < local_8)
                        {
                            return;
                        }
                        uVar2 = (local_8 * 2 + _PTR_LOOP_1050_580e);
                        in_EAX = SEXT24(uVar2);
                        uVar5 = in_EAX >> 0x10;
                        if (_local_6 < in_EAX)
                        {
                            return;
                        }
                        _local_6 = CONCAT22(((_local_6 >> 0x10) - (uVar2 >> 0xf)) -
                                                (local_6 < uVar2),
                                            local_6 - uVar2);
                    }
                }
                local_8 = local_8 + 1;
                if (0x24 < local_8)
                {
                    return;
                }
            } 
        }
    }
    return;
}



pub fn pass1_1030_8fe4(param_1: u16, param_2: u16,param_1_00: u32,param_2_00: u32)

{
    let mut iVar1: i32;
    let struct_b: *mut astruct_493;
    let mut uVar2: i32;
    let mut uVar3: i32;
    let lVar4: u32;
    let mut uVar5: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    lVar4 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_1_00, param_2_00);
    uVar2 = (lVar4 >> 0x10);
    uVar3 = uVar2 | lVar4;
    if (lVar4 != 0)
    {
        struct_b = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar4, uVar2);
        if ((uVar3 | struct_b) != 0)
        {
            uVar5 = pass1_1030_73a8(CONCAT22(uVar3, struct_b));
            if ((uVar5 != 0) && ((iVar1 = (uVar5 + 0xc), iVar1 == 5 || (iVar1 == 9))))
            {
                return 1;
            }
        }
    }
    return 0;
}





pub fn pass1_1030_9048(param_1: u32, param_2: i32,param_3: u32)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let mut uVar4: u16;
    let mut uVar5: u16;
    let paVar6: *mut astruct_493;
    let puVar7: *mut u8;
    let mut uVar8: u32;
    let mut in_DX: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: u16;
    let mut uVar9: u16;
    let puVar10: *mut u32;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut unaff_SS: u16;
    let mut uVar13: u32;
    let mut uVar14: u16;
    let uVar15: u8;
    let mut local_46: u32;
    let mut local_42: u32;
    let mut local_3e: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: [u8;2];
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 8 - (param_2 == 0);
    puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, local_4);
    uVar4 = puVar7;
    local_8 = uVar4;
    pass1_1038_4e78(param_3, puVar7 & 0xffff | in_DX << 0x10);
    _local_c = CONCAT22(extraout_DX, uVar4);
    uVar12 = SUB42(&PTR_LOOP_1050_1008, 0);
    zero_list_1008_3e38(CONCAT22(unaff_SS, &local_12));
    uVar2 = (param_3 + 8);
    uVar11 = (_local_c >> 0x10);
    uVar9 = SUB42(_local_c, 0);
    ppcVar3 = (*_local_c + 0x10);
    uVar8 = uVar2;
    (**ppcVar3)(&PTR_LOOP_1050_1008, uVar9, uVar11);
    uVar8 = uVar8 & 0xffff | extraout_DX_00 << 0x10;
    local_24 = 0;
    loop
    {
        if (uVar8 <= local_24)
        {
            if (_local_c != 0x0)
            {
                ppcVar3 = *_local_c;
                (**ppcVar3)(uVar12, _local_c, (_local_c >> 0x10), 1, uVar9, uVar11, _local_c,
                            _local_c);
            }
            return;
        }
        ppcVar3 = (*_local_c + 4);
        uVar13 = uVar8;
        (**ppcVar3)();
        uVar4 = extraout_DX_01;
        paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar13, extraout_DX_01);
        modify_list_1008_3f62(CONCAT22(unaff_SS, &local_12), CONCAT22(uVar4, &paVar6.field_0xc));
        uVar12 = SUB42(&PTR_LOOP_1050_1008, 0);
        pass1_1008_3eb4(CONCAT22(unaff_SS, &local_12), CONCAT22(unaff_SS, local_18),
                        CONCAT22(unaff_SS, &local_16), CONCAT22(unaff_SS, &local_14));
        uVar13 = pass1_1030_73a8(CONCAT22(uVar4, paVar6));
        iVar1 = (uVar13 + 0xc);
        if (iVar1 - 0x7a < 6) {
            break;
        }
// LAB_1030_91fa:
        local_24 = local_24 + 1;
    }
    uVar12 = 0x1030;
    uVar4 = param_1;
    uVar14 = (param_1 >> 0x10);
    match (iVar1)
    {
    _ => {
        local_10 = local_16 - 1;
        uVar5 = pass1_1030_8fe4(uVar4, uVar14, CONCAT22(unaff_SS, &local_12), uVar2);
        if (uVar5 != 0) {}
          // goto LAB_1030_91cb;
        local_10 = local_16 + 1;
        uVar5 = pass1_1030_8fe4(uVar4, uVar14, CONCAT22(unaff_SS, &local_12), uVar2);
        if (uVar5 == 0)
        {
            local_10 = local_16;
            local_12 = local_14 - 1;
            uVar5 = pass1_1030_8fe4(uVar4, uVar14, CONCAT22(unaff_SS, &local_12), uVar2);
          // goto joined_r0x1030911e;
        }
// LAB_1030_9144:
    },
    0x7b |
    0x7e =>{
        local_10 = local_16 - 1;
        uVar5 = pass1_1030_8fe4(uVar4, uVar14, CONCAT22(unaff_SS, &local_12), uVar2);
        if (uVar5 == 0)
        {
            local_10 = local_16 + 1;
          // goto LAB_1030_912c;
        }
        if (_local_c == 0x0)
        {
            return;
        }
        uVar12 = (_local_c >> 0x10);
        puVar10 = _local_c;
        uVar15 = (_local_c >> 0x10);
      // goto LAB_1030_90e6; 
    },
    0x7c |
    0x7d =>{
        local_12 = local_14 - 1;
        uVar5 = pass1_1030_8fe4(uVar4, uVar14, CONCAT22(unaff_SS, &local_12), uVar2);
// joined_r0x1030911e:
        if (uVar5 == 0)
        {
            local_12 = local_14 + 1;
// LAB_1030_912c:
            uVar4 = pass1_1030_8fe4(uVar4, uVar14, CONCAT22(unaff_SS, &local_12), uVar2);
            if (uVar4 != 0) {}
              // goto LAB_1030_9144;
          // goto LAB_1030_91fa;
        }
// LAB_1030_91cb:
    }}

    puVar10 = _local_c;
    if ((local_a | puVar10) != 0)
    {
        uVar12 = (_local_c >> 0x10);
        uVar15 = (_local_c >> 0x10);
// LAB_1030_90e6:
        unsafe{
            ppcVar3 = *puVar10;
            (**ppcVar3)(0x1030, puVar10, uVar15, 1, uVar9, uVar11, _local_c, _local_c);
        }
    }
    return;
}




pub fn pass1_1030_9296(param_1: u32, param_2: *mut u32,param_3: u32)

{
    let ppcVar1: fn();
    let mut uVar2: u16;
    let mut in_AX: i32;
    let mut uVar3: i32;
    let local_AX_355: *mut astruct_950;
    let puVar4: *mut u8;
    let puVar5: *mut u16;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: i32;
    let mut extraout_DX_05: i32;
    let mut extraout_DX_06: i32;
    let mut unaff_SS: u16;
    let mut in_stack_0000ffc2: u16;
    let mut local_3a: [u8;12];
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_24: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1038_53ba(param_3, (&PTR_LOOP_1050_0000 + 1));
    in_DX = in_DX | in_AX;
    if (in_DX != 0)
    {
        local_1e = _PTR_LOOP_1050_5768;
        puVar5 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        uVar3 = puVar5;
        _local_12 = (puVar5 & 0xffff | extraout_DX << 0x10);
        if ((extraout_DX | uVar3) == 0)
        {
            local_6 = 0;
        }
        else
        {
            *_local_12 = s_1_1050_389a;
            (uVar3 + 2) = &PTR_LOOP_1050_1008;
            (uVar3 + 4) = 0x73;
            *_local_12 = 0x9ec8;
            (uVar3 + 2) = 0x1030;
            puVar5 = _local_12;
            local_6 = _local_12;
        }
        in_AX = puVar5;
        unsafe {
            ppcVar1 = (*param_2 + 4);
            (**ppcVar1)(0x1000, param_2, local_6, (local_6 >> 0x10));
        }
        in_DX = extraout_DX_00;
    }
    pass1_1038_53ba(param_3, &dos_alloc_addr_1050_0002);
    in_DX = in_DX | in_AX;
    if (in_DX != 0)
    {
        local_1e = _PTR_LOOP_1050_5768;
        puVar5 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        uVar3 = puVar5;
        _local_12 = (puVar5 & 0xffff | extraout_DX_01 << 0x10);
        if ((extraout_DX_01 | uVar3) == 0)
        {
            local_6 = 0;
        }
        else
        {
            *_local_12 = s_1_1050_389a;
            (uVar3 + 2) = &PTR_LOOP_1050_1008;
            (uVar3 + 4) = 0x74;
            *_local_12 = 0x9ec8;
            (uVar3 + 2) = 0x1030;
            puVar5 = _local_12;
            local_6 = _local_12;
        }
        in_AX = puVar5;
        unsafe {
            ppcVar1 = (*param_2 + 8);
            (**ppcVar1)(0x1000, param_2, local_6, (local_6 >> 0x10));
        }
        in_DX = extraout_DX_02;
    }
    pass1_1038_53ba(param_3, (&dos_alloc_addr_1050_0002 + 1));
    if ((in_DX | in_AX) != 0)
    {
        local_24 = _PTR_LOOP_1050_5768;
        puVar5 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        uVar3 = puVar5;
        _local_12 = (puVar5 & 0xffff | extraout_DX_03 << 0x10);
        if ((extraout_DX_03 | uVar3) == 0)
        {
            local_6 = 0;
        }
        else
        {
            *_local_12 = s_1_1050_389a;
            (uVar3 + 2) = &PTR_LOOP_1050_1008;
            (uVar3 + 4) = 0x75;
            *_local_12 = 0x9ec8;
            (uVar3 + 2) = 0x1030;
            puVar5 = _local_12;
            local_6 = _local_12;
        }
        in_AX = puVar5;
        unsafe {
            ppcVar1 = (*param_2 + 8);
            (**ppcVar1)(0x1000, param_2, local_6, (local_6 >> 0x10));
        }
    }
    pass1_1030_8f04(param_1, (param_1 >> 0x10), param_3);
    if (in_AX != 0)
    {
        local_24 = _PTR_LOOP_1050_5768;
        puVar5 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        uVar3 = puVar5;
        _local_12 = (puVar5 & 0xffff | extraout_DX_04 << 0x10);
        if ((extraout_DX_04 | uVar3) == 0)
        {
            local_6 = 0;
        }
        else
        {
            *_local_12 = s_1_1050_389a;
            (uVar3 + 2) = &PTR_LOOP_1050_1008;
            (uVar3 + 4) = 0x37;
            *_local_12 = 0x9ec8;
            (uVar3 + 2) = 0x1030;
            local_6 = _local_12;
        }
        unsafe {
            ppcVar1 = (*param_2 + 8);
            (**ppcVar1)(0x1000, param_2, local_6, (local_6 >> 0x10));
        }
    }
    _local_a = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffc2, 8));
    uVar2 = (_local_a >> 0x10);
    local_e = (_local_a + 0xe);
    uVar3 = (_local_a + 0x10);
    if ((uVar3 | local_e) != 0)
    {
        pass1_1008_5784(CONCAT22(unaff_SS, local_3a), local_e & 0xffff | uVar3 << 0x10);
        loop
        {
            puVar4 = local_3a;
            pass1_1008_5b12(CONCAT22(unaff_SS, puVar4));
            _local_2e = CONCAT22(extraout_DX_05, puVar4);
            if ((extraout_DX_05 | puVar4) == 0) {
                break;
            }
            if (((puVar4 + 4) == 0x3e) || ((puVar4 + 4) == 0x41))
            {
                local_1e = _PTR_LOOP_1050_5768;
                puVar5 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                uVar3 = puVar5;
                _local_12 = (puVar5 & 0xffff | extraout_DX_06 << 0x10);
                if ((extraout_DX_06 | uVar3) == 0)
                {
                    local_6 = 0;
                }
                else
                {
                    local_1a = (_local_2e + 4);
                    *_local_12 = s_1_1050_389a;
                    (uVar3 + 2) = &PTR_LOOP_1050_1008;
                    (uVar3 + 4) = local_1a;
                    *_local_12 = 0x9ec8;
                    (uVar3 + 2) = 0x1030;
                    local_6 = _local_12;
                }
                unsafe {
                    ppcVar1 = (*param_2 + 8);
                    (**ppcVar1)(0x1000, param_2, local_6, (local_6 >> 0x10));
                }
            }
        }
    }
    return;
}




pub fn pass1_1030_951a(param_1: u32, param_2: *mut u32,param_3: u32)

{
    let ppcVar1: fn();
    let mut uVar2: i32;
    let puVar3: *mut u16;
    let puVar4: *mut u16;
    let mut uVar5: u16;
    let paVar6: *mut astruct_493;
    let ppVar7: *mut pass1_struct_1;
    let puVar8: *mut u8;
    let mut extraout_DX: i32;
    let mut uVar9: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: i32;
    let mut extraout_DX_05: i32;
    let mut extraout_DX_06: i32;
    let mut extraout_DX_07: i32;
    let mut extraout_DX_08: u16;
    let mut extraout_DX_09: u16;
    let mut extraout_DX_10: i32;
    let mut extraout_DX_11: i32;
    let mut extraout_DX_12: i32;
    let mut extraout_DX_13: i32;
    let mut extraout_DX_14: u16;
    let mut uVar10: u16;
    let mut extraout_DX_15: i32;
    let mut extraout_DX_16: i32;
    let mut iVar11: i32;
    let mut unaff_SI: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let uVar14: u8;
    let mut unaff_SS: u16;
    let mut uVar15: u32;
    let mut uVar16: u32;
    let puVar17: *mut u32;
    let uVar18: u8;
    let uVar19: u8;
    let uVar20: u8;
    let mut uVar21: u16;
    let mut local_5e: u32;
    let mut local_52: u16;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_46: u32;
    let mut local_3e: u32;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    _local_a = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x35));
    uVar9 = (_local_a >> 0x10);
    uVar2 = _local_a + 10;
    _local_e = _local_a & 0xffff0000 | uVar2;
    pass1_1030_9048(param_1, 0, param_3);
    uVar12 = (param_2 >> 0x10);
    uVar20 = SUB41(param_2, 0);
    if (uVar2 != 0)
    {
        local_1c = 0;
        _local_20 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 8));
        uVar13 = (_local_20 >> 0x10);
        local_24 = (_local_20 + 0xe);
        uVar9 = (_local_20 + 0x10);
        if ((uVar9 | local_24) != 0)
        {
            pass1_1008_5784(CONCAT22(unaff_SS, &local_36), local_24 & 0xffff | uVar9 << 0x10);
            loop
            {
                puVar3 = &local_36;
                pass1_1008_5b12(CONCAT22(unaff_SS, puVar3));
                _local_2e = CONCAT22(extraout_DX, puVar3);
                uVar9 = extraout_DX | puVar3;
                if (uVar9 == 0) {
                    break;
                }
                if ((puVar3[2] != 0x3e) && (puVar3[2] != 0x41))
                {
                    ppVar7 = _PTR_LOOP_1050_5768;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                    uVar9 = ppVar7;
                    local_6 = ppVar7 & 0xffff | extraout_DX_00 << 0x10;
                    if ((extraout_DX_00 | uVar9) == 0)
                    {
                        local_6 = 0;
                    }
                    else
                    {
                        uVar13 = (_local_2e + 4);
                        local_6 = s_1_1050_389a;
                        (uVar9 + 2) = &PTR_LOOP_1050_1008;
                        (uVar9 + 4) = uVar13;
                        local_6 = 0x9ec8;
                        (uVar9 + 2) = 0x1030;
                    }
                    unsafe {
                        ppcVar1 = (*param_2 + 8);
                        (**ppcVar1)(0, uVar20, uVar12, local_6, (local_6 >> 0x10));
                    }
                    if ((_local_2e + 4) == 0x13)
                    {
                        local_1c = 1;
                    }
                }
            }
        }
        local_26 = 10;
        while (local_26 < 0x41)
        {
            if ((((((local_26 != 0x37) && (local_26 != 0x35)) && (local_26 != 0x36)) &&
                  ((local_26 != 0x25 && (local_26 != 0x26)))) &&
                 ((local_26 != 0x27 &&
                   (((local_26 * 2 + _local_e) != 0 && (local_26 != 0x13)))))) &&
                ((local_26 != 0x14 || (local_1c == 0))))
            {
                ppVar7 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                uVar9 = ppVar7;
                local_6 = ppVar7 & 0xffff | extraout_DX_01 << 0x10;
                if ((extraout_DX_01 | uVar9) == 0)
                {
                    local_6 = 0;
                }
                else
                {
                    local_6 = s_1_1050_389a;
                    (uVar9 + 2) = &PTR_LOOP_1050_1008;
                    (uVar9 + 4) = local_26;
                    local_6 = 0x9ec8;
                    (uVar9 + 2) = 0x1030;
                }
                unsafe {
                    ppcVar1 = (*param_2 + 8);
                    (**ppcVar1)(0, uVar20, uVar12, local_6, (local_6 >> 0x10));
                }
                uVar9 = extraout_DX_02;
            }
            local_26 = local_26 + 1;
        }
    }
    uVar13 = (_local_e >> 0x10);
    if ((_local_e + 0x6a) == 0)
    {
        if ((_local_e + 0x6c) != 0)
        {
            ppVar7 = _PTR_LOOP_1050_5768;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
            puVar4 = ppVar7;
            _local_3a = (ppVar7 & 0xffff | extraout_DX_06 << 0x10);
            if ((extraout_DX_06 | puVar4) == 0) {}
              // goto LAB_1030_973e;
            unsafe {
              *_local_3a = s_1_1050_389a;
            }
            puVar4[1] = &PTR_LOOP_1050_1008;
            puVar4[2] = 0x36;
            uVar9 = extraout_DX_06;
          // goto LAB_1030_9728;
        }
    }
    else
    {
        ppVar7 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        puVar4 = ppVar7;
        _local_3a = (ppVar7 & 0xffff | extraout_DX_03 << 0x10);
        if ((extraout_DX_03 | puVar4) == 0)
        {
// LAB_1030_973e:
            local_6 = 0;
        }
        else
        {
            *_local_3a = s_1_1050_389a;
            puVar4[1] = &PTR_LOOP_1050_1008;
            puVar4[2] = 0x35;
            uVar9 = extraout_DX_03;
// LAB_1030_9728:
            unsafe {
                *puVar4 = 0x9ec8;
                puVar4[1] = 0x1030;
            }
            local_6 = _local_3a;
        }
        unsafe {
            ppcVar1 = (*param_2 + 8);
            (**ppcVar1)(0, uVar20, uVar12, local_6, (local_6 >> 0x10));
        }
        uVar9 = extraout_DX_04;
    }
    uVar13 = (_local_e >> 0x10);
    iVar11 = _local_e;
    if ((iVar11 + 0x4a) == 0)
    {
        if ((iVar11 + 0x4c) == 0)
        {
            if ((iVar11 + 0x4e) == 0) {}
              // goto LAB_1030_97e8;
            ppVar7 = _PTR_LOOP_1050_5768;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
            puVar4 = ppVar7;
            _local_3a = (ppVar7 & 0xffff | extraout_DX_11 << 0x10);
            if ((extraout_DX_11 | puVar4) != 0)
            {
                *_local_3a = s_1_1050_389a;
                puVar4[1] = &PTR_LOOP_1050_1008;
                puVar4[2] = 0x27;
                uVar9 = extraout_DX_11;
              // goto LAB_1030_9879;
            }
        }
        else
        {
            ppVar7 = _PTR_LOOP_1050_5768;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
            puVar4 = ppVar7;
            _local_3a = (ppVar7 & 0xffff | extraout_DX_10 << 0x10);
            if ((extraout_DX_10 | puVar4) != 0)
            {
                *_local_3a = s_1_1050_389a;
                puVar4[1] = &PTR_LOOP_1050_1008;
                puVar4[2] = 0x26;
                uVar9 = extraout_DX_10;
              // goto LAB_1030_9879;
            }
        }
// LAB_1030_97d0:
        local_6 = 0;
    }
    else
    {
        ppVar7 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        puVar4 = ppVar7;
        _local_3a = (ppVar7 & 0xffff | extraout_DX_05 << 0x10);
        if ((extraout_DX_05 | puVar4) == 0) {}
          // goto LAB_1030_97d0;
        unsafe {
            *_local_3a = s_1_1050_389a;
        }
        puVar4[1] = &PTR_LOOP_1050_1008;
        puVar4[2] = 0x25;
        
        uVar9 = extraout_DX_05;
// LAB_1030_9879:
        unsafe {
            *puVar4 = 0x9ec8;
            puVar4[1] = 0x1030;
        }
        local_6 = _local_3a;
    unsafe {
        ppcVar1 = (*param_2 + 8);
        (**ppcVar1)(0, uVar20, uVar12, local_6, (local_6 >> 0x10));
    }
    uVar9 = extraout_DX_07;
}
// LAB_1030_97e8:
    local_12 = _local_a & 0xffff0000 | (_local_a + 0x11e);
    if ((_local_a + 0x138) != 0)
    {
        puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 4);
        uVar5 = SUB42(puVar8, 0);
        uVar14 = 0x38;
        pass1_1038_4d6e(param_3, puVar8 & 0xffff | uVar9 << 0x10);
        _local_4c = CONCAT22(extraout_DX_08, uVar5);
        ppcVar1 = (*_local_4c + 0x10);
        uVar13 = uVar5;
        (**ppcVar1)(&PTR_LOOP_1050_1038, uVar5, extraout_DX_08);
        local_46 = CONCAT22(extraout_DX_09, uVar13);
        local_3e = 0;
        while (local_3e < local_46)
        {
            ppcVar1 = (*_local_4c + 4);
            uVar15 = local_46;
            (**ppcVar1)(uVar14, uVar5, extraout_DX_08, local_3e, (local_3e >> 0x10));
            local_36 = uVar15;
            uVar21 = 0xd;
            uVar10 = extraout_DX_14;
            local_34 = extraout_DX_14;
            paVar6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_36, extraout_DX_14);
            _local_2e = CONCAT22(uVar10, paVar6);
            uVar15 = pass1_1030_73a8(CONCAT22(uVar10, paVar6));
            local_28 = (uVar15 >> 0x10);
            local_2a = uVar15;
            uVar14 = 0x28;
            uVar16 = pass1_1028_6744(uVar15, uVar21);
            if (((uVar16 >> 0x10) | uVar16) != 0)
            {
                _local_20 = _PTR_LOOP_1050_5768;
                ppVar7 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                uVar9 = ppVar7;
                local_6 = ppVar7 & 0xffff | extraout_DX_15 << 0x10;
                if ((extraout_DX_15 | uVar9) == 0)
                {
                    local_6 = 0;
                }
                else
                {
                    local_6 = s_1_1050_389a;
                    (uVar9 + 2) = &PTR_LOOP_1050_1008;
                    (uVar9 + 4) = 0x4c;
                    local_6 = 0x9ec8;
                    (uVar9 + 2) = 0x1030;
                }
                unsafe {
                    ppcVar1 = (*param_2 + 8);
                    (**ppcVar1)(0, uVar20, uVar12, local_6, (local_6 >> 0x10));
                }
                local_24 = _PTR_LOOP_1050_5768;
                ppVar7 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                uVar9 = ppVar7;
                local_6 = ppVar7 & 0xffff | extraout_DX_12 << 0x10;
                if ((extraout_DX_12 | uVar9) == 0)
                {
                    local_6 = 0;
                }
                else
                {
                    local_6 = s_1_1050_389a;
                    (uVar9 + 2) = &PTR_LOOP_1050_1008;
                    (uVar9 + 4) = 0x4d;
                    local_6 = 0x9ec8;
                    (uVar9 + 2) = 0x1030;
                }
                uVar18 = local_6;
                uVar19 = (local_6 >> 0x10);
                unsafe {
                    ppcVar1 = (*param_2 + 8);
                    puVar17 = param_2;
                    (**ppcVar1)();
                }
                local_24 = _PTR_LOOP_1050_5768;
                uVar14 = 0;
                ppVar7 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                uVar9 = ppVar7;
                local_6 = ppVar7 & 0xffff | extraout_DX_13 << 0x10;
                if ((extraout_DX_13 | uVar9) == 0)
                {
                    local_6 = 0;
                }
                else
                {
                    local_6 = s_1_1050_389a;
                    (uVar9 + 2) = &PTR_LOOP_1050_1008;
                    (uVar9 + 4) = 0x4e;
                    local_6 = 0x9ec8;
                    (uVar9 + 2) = 0x1030;
                }
                unsafe {
                    ppcVar1 = (*param_2 + 8);
                    (**ppcVar1)(0x1000, param_2, local_6, puVar17, uVar18, uVar19);
                }
                break;
            }
            local_3e = local_3e + 1;
        }
        if (_local_4c != 0x0)
        {
            ppcVar1 = *_local_4c;
            (**ppcVar1)(uVar14, uVar5, extraout_DX_08, 1);
        }
    }
    local_14 = 0x7a;
    while (local_14 < 0x7d)
    {
        if ((local_14 * 2 + _local_e) != 0)
        {
            ppVar7 = _PTR_LOOP_1050_5768;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
            uVar9 = ppVar7;
            local_6 = ppVar7 & 0xffff | extraout_DX_16 << 0x10;
            if ((extraout_DX_16 | uVar9) == 0)
            {
                local_6 = 0;
            }
            else
            {
                local_6 = s_1_1050_389a;
                (uVar9 + 2) = &PTR_LOOP_1050_1008;
                (uVar9 + 4) = local_14;
                local_6 = 0x9ec8;
                (uVar9 + 2) = 0x1030;
            }
            unsafe {
                ppcVar1 = (*param_2 + 8);
                (**ppcVar1)(0, uVar20, uVar12, local_6, (local_6 >> 0x10));
            }
        }
        local_14 = local_14 + 1;
    }
    return;
}



pub fn pass1_1030_9adc(param_1: u16, param_2: u16, param_1_00: *mut u32,param_2_00: u32)

{
    let ppcVar1: fn();
    let mut in_AX: i32;
    let mut uVar2: i32;
    let puVar3: *mut u16;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut extraout_DX_03: i32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1038_53ba(param_2_00, (&PTR_LOOP_1050_0000 + 1));
    in_DX = in_DX | in_AX;
    if (in_DX != 0)
    {
        puVar3 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        uVar2 = puVar3;
        local_6 = puVar3 & 0xffff | extraout_DX << 0x10;
        if ((extraout_DX | uVar2) == 0)
        {
            local_6 = 0;
        }
        else
        {
            local_6 = s_1_1050_389a;
            (uVar2 + 2) = &PTR_LOOP_1050_1008;
            (uVar2 + 4) = 0x77;
            local_6 = 0x9ec8;
            (uVar2 + 2) = 0x1030;
            puVar3 = local_6;
        }
        in_AX = puVar3;
        ppcVar1 = (*param_1_00 + 4);
        (**ppcVar1)(0x1000, param_1_00, local_6, (local_6 >> 0x10));
        in_DX = extraout_DX_00;
    }
    pass1_1038_53ba(param_2_00, &dos_alloc_addr_1050_0002);
    in_DX = in_DX | in_AX;
    if (in_DX != 0)
    {
        puVar3 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        uVar2 = puVar3;
        local_6 = puVar3 & 0xffff | extraout_DX_01 << 0x10;
        if ((extraout_DX_01 | uVar2) == 0)
        {
            local_6 = 0;
        }
        else
        {
            local_6 = s_1_1050_389a;
            (uVar2 + 2) = &PTR_LOOP_1050_1008;
            (uVar2 + 4) = 0x78;
            local_6 = 0x9ec8;
            (uVar2 + 2) = 0x1030;
            puVar3 = local_6;
        }
        in_AX = puVar3;
        ppcVar1 = (*param_1_00 + 8);
        (**ppcVar1)(0x1000, param_1_00, local_6, (local_6 >> 0x10));
        in_DX = extraout_DX_02;
    }
    pass1_1038_53ba(param_2_00, (&dos_alloc_addr_1050_0002 + 1));
    if ((in_DX | in_AX) != 0)
    {
        puVar3 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        uVar2 = puVar3;
        local_6 = puVar3 & 0xffff | extraout_DX_03 << 0x10;
        if ((extraout_DX_03 | uVar2) == 0)
        {
            local_6 = 0;
        }
        else
        {
            local_6 = s_1_1050_389a;
            (uVar2 + 2) = &PTR_LOOP_1050_1008;
            (uVar2 + 4) = 0x75;
            local_6 = 0x9ec8;
            (uVar2 + 2) = 0x1030;
        }
        ppcVar1 = (*param_1_00 + 8);
        (**ppcVar1)(0x1000, param_1_00, local_6, (local_6 >> 0x10));
    }
    return;
}



pub fn pass1_1030_9c1c(param_1: u32, param_2: *mut u32,param_3: u32)

{
    let ppcVar1: fn();
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: i32;
    let puVar6: *mut u8;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut unaff_SI: u16;
    let ppVar7: *mut pass1_struct_1;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u32;

    ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x35));
    iVar3 = ppVar7 + 10;
    uVar2 = (ppVar7 >> 0x10);
    iVar4 = iVar3;
    pass1_1030_9048(param_1, 1, param_3);
    if (iVar4 != 0)
    {
        local_18 = 0x4f;
        while (local_18 < 0x70)
        {
            if ((local_18 * 2 + iVar3) != 0)
            {
                puVar6 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                uVar5 = puVar6;
                local_6 = puVar6 & 0xffff | extraout_DX << 0x10;
                if ((extraout_DX | uVar5) == 0)
                {
                    local_6 = 0;
                }
                else
                {
                    local_6 = s_1_1050_389a;
                    (uVar5 + 2) = &PTR_LOOP_1050_1008;
                    (uVar5 + 4) = local_18;
                    local_6 = 0x9ec8;
                    (uVar5 + 2) = 0x1030;
                }
                ppcVar1 = (*param_2 + 8);
                (**ppcVar1)(0x1000, param_2, local_6, (local_6 >> 0x10));
            }
            local_18 = local_18 + 1;
        }
    }
    local_10 = 0x7d;
    while (local_10 < 0x80)
    {
        if ((local_10 * 2 + iVar3) != 0)
        {
            puVar6 = _PTR_LOOP_1050_5768;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
            uVar5 = puVar6;
            local_6 = puVar6 & 0xffff | extraout_DX_00 << 0x10;
            if ((extraout_DX_00 | uVar5) == 0)
            {
                local_6 = 0;
            }
            else
            {
                local_6 = s_1_1050_389a;
                (uVar5 + 2) = &PTR_LOOP_1050_1008;
                (uVar5 + 4) = local_10;
                local_6 = 0x9ec8;
                (uVar5 + 2) = 0x1030;
            }
            ppcVar1 = (*param_2 + 8);
            (**ppcVar1)(0x1000, param_2, local_6, (local_6 >> 0x10));
        }
        local_10 = local_10 + 1;
    }
    return;
}




pub fn pass1_1030_9d42(param_1: u16, param_2: u16, param_1_00: *mut u32,param_2_00: u32)

{
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let ppcVar4: fn();
    let uVar5: u8;
    let mut uVar6: u16;
    let puVar7: *mut u8;
    let puVar8: *mut u8;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut uVar10: i32;
    let mut iVar11: i32;
    let mut uVar12: u16;
    let mut unaff_SS: u16;
    let mut local_b8: u16;
    let mut local_b6: u16;
    let mut local_b4: u32;
    let mut local_ae: u16;
    let mut local_ac: u16;
    let mut local_aa: u32;
    let mut local_a6: [u8;4];
    let mut local_a2: u32;
    let mut local_9e: u16;
    let mut local_9c: u16;
    let mut local_9a: u16;
    let mut local_98: u16;
    let mut local_96: u16;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;
    let mut uVar9: u32;

    uVar12 = (param_2_00 >> 0x10);
    if ((param_2_00 + 0x206) == 0)
    {
        local_4 = (param_2_00 + 0x204);
        uVar5 = pass1_1000_4906(CONCAT22(unaff_SS, &local_98), 0, 0x94);
        uVar9 = CONCAT31(extraout_var, uVar5);
        local_9a = 0x11;
        while
        {
            pass1_1038_540a(param_2_00, local_9a);
            uVar6 = uVar9;
            (&local_98)[local_9a * 2] = uVar6;
            (&local_96)[local_9a * 2] = extraout_DX;
            local_9a = local_9a + 1;
            local_9a < 0x25
        } {}
        pass1_1038_540a(param_2_00, 0x21);
        _local_9e = CONCAT22(extraout_DX_00, uVar6);
        pass1_1008_5784(CONCAT22(unaff_SS, local_a6), param_1_00);
        uVar2 = (_PTR_LOOP_1050_65e2 + 0x52);
        loop
        {
            puVar7 = local_a6;
            pass1_1008_5b12(CONCAT22(unaff_SS, puVar7));
            uVar10 = extraout_DX_01 | puVar7;
            if (uVar10 == 0) {
                break;
            }
            puVar8 = puVar7;
            pass1_1030_4bbe(uVar2, (puVar7 + 4));
            if (local_4 == 0)
            {
                local_9a = 0x11;
                while (local_9a < 0x25)
                {
                    iVar11 = local_9a * 4;
                    if (((puVar8 + iVar11) != 0) &&
                        (uVar3 = (&local_98 + local_9a * 2), puVar1 = (puVar8 + iVar11),
                         uVar3 <= *puVar1 && *puVar1 != uVar3))
                    {
                        puVar1 = (puVar8 + iVar11);
                        if (_local_9e <= *puVar1 && *puVar1 != _local_9e) {}
                          // goto LAB_1030_9e17;
                        _local_9e = _local_9e - (puVar8 + iVar11);
                    }
                    local_9a = local_9a + 1;
                }
            }
            else
            {
                puVar1 = (puVar8 + 0x8c);
                if ((local_c <= *puVar1 && *puVar1 != local_c) ||
                    (puVar1 = (puVar8 + 0x90), local_8 <= *puVar1 && *puVar1 != local_8))
                {
// LAB_1030_9e17:
                    ppcVar4 = (*param_1_00 + 0xc);
                    (**ppcVar4)(&PTR_LOOP_1050_1008, param_1_00, puVar7, extraout_DX_01);
                    local_a2 = 0;
                }
            }
        }
    }
    return;
}


pub fn pass1_1030_8480(param_1: *mut astruct_44)
{
    error_check_1000_17ce(*param_1);
    return;
}

pub fn pass1_1030_8496(param_1: *mut u8)

{
    error_check_1000_17ce((param_1 + 2));
    return;
}

pub fn pass1_1030_84ae(param_1: u32)

{
    zero_list_1008_3e38((param_1 & 0xffff0000 | (param_1 + 8)));
    (param_1 + 0x1e) = 1;
    return;
}

pub fn pass1_1030_84d0(param_1: *mut astruct_44)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let in_AL: u8;
    let local_BX_5: *mut astruct_941;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (local_BX_5.field_0x1e != 0)
    {
        puVar1 = local_BX_5.field_0xe;
        uVar2 = local_BX_5.field_0x10;
        if ((uVar2 | puVar1) != 0)
        {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        puVar1 = local_BX_5.field_0x12;
        uVar2 = local_BX_5.field_0x14;
        if ((uVar2 | puVar1) != 0)
        {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        error_check_1000_17ce(local_BX_5.field_0x4);
        in_AL = error_check_1000_17ce(local_BX_5.field_0x16);
    }
    return in_AL;
}

pub fn pass1_1030_8544(param_1: *mut astruct_942, param_2: *mut astruct_943)

{
    let local_BX_15: *mut astruct_943;
    let local_BX_23: *mut astruct_942;
    let mut uVar1: u16;
    let mut uVar2: u16;

    param_1.field_0x0 = param_2;
    uVar1 = (param_2 >> 0x10);
    local_BX_15 = param_2;
    uVar2 = (param_1 >> 0x10);
    local_BX_23 = param_1;
    local_BX_23.field_0x4 = local_BX_15.field_0x4;
    modify_list_1008_3f62((param_1 & 0xffff0000 | &local_BX_23.field_0x8),
                          param_2 & 0xffff0000 | &local_BX_15.field_0x8);
    local_BX_23.field_0xe = local_BX_15.field_0xe;
    local_BX_23.field_0x12 = local_BX_15.field_0x12;
    local_BX_23.field_0x16 = local_BX_15.field_0x16;
    local_BX_23.field_0x1a = local_BX_15.field_0x1a;
    local_BX_23.field_0x1e = 0;
    return;
}

pub fn pass1_1030_85be(param_1: *mut astruct_944, param_2: u16, param_3: u16)

{
    let local_BX_3: *mut astruct_944;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    param_1 = 0;
    local_BX_3.field_0x4 = 0;
    local_BX_3.field_0x6 = param_3;
    local_BX_3.field_0x8 = param_2;
    local_BX_3.field_0xe = 0;
    if (local_BX_3.field_0x6 == 0)
    {
        local_BX_3.field_0x6 = 5;
    }
    pass1_1030_878c(param_1);
    return;
}

pub fn pass1_1030_8604(param_1: *mut *mut astruct_44)
{
    error_check_1000_17ce(*param_1);
    return;
}



pub fn pass1_1030_861a(param_1: i32, param_2: u16, param_3: u16)
{
    let mut in_AX: i32;
    let mut in_DX: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_8854(CONCAT22(param_2, param_1), param_3);
    _local_6 = CONCAT22(in_DX, in_AX);
    if ((in_DX | in_AX) == 0)
    {
        (param_1 + 10) = 0;
    }
    else
    {
        (param_1 + 10) = *_local_6;
    }
    return;
}

pub fn pass1_1030_8660(param_1: u32, param_2: *mut u32, param_3: u16)

{
    let mut in_AX: i32;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut uVar1: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_8854(param_1, param_3);
    _local_6 = CONCAT22(in_DX, in_AX);
    in_DX = in_DX | in_AX;
    if (in_DX == 0)
    {
        pass1_1030_8854(param_1, 0);
        _local_6 = CONCAT22(in_DX, in_AX);
        if ((in_DX | in_AX) == 0)
        {
            pass1_1030_878c(param_1);
            uVar1 = extraout_DX;
            pass1_1030_8854(param_1, 0);
            _local_6 = CONCAT22(uVar1, in_AX);
            if ((uVar1 | in_AX) == 0)
            {
                return;
            }
        }
        (_local_6 + 4) = param_3;
        *_local_6 = *param_2;
        pass1_1030_8834(param_1);
    }
    else
    {
        *_local_6 = *param_2;
    }
    return;
}

pub fn pass1_1030_86ec(param_1: *mut astruct_44, param_2: u16)

{
    let local_BX_18: *mut astruct_44;
    let mut uVar1: u16;

    error_check_1000_17ce(param_1);
    uVar1 = (param_1 >> 0x10);
    local_BX_18 = param_1;
    param_1 = 0;
    local_BX_18.struct_ptr_1_0x4 = 0x0;
    &local_BX_18.field_0x6 = param_2;
    &local_BX_18.field_0xe = 0;
    return;
}

pub fn pass1_1030_871e(param_1: *mut astruct_946, param_2: *mut u32, param_3: u16)

{
    let piVar1: *mut i32;
    let local_BX_5: *mut astruct_946;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (param_1 == 0)
    {
        pass1_1030_878c(param_1);
    }
    piVar1 = &local_BX_5.field_0xe;
    unsafe{ *piVar1 = *piVar1 + 1 };
    (param_1 + local_BX_5.field_0xe * 6 + 4) = param_3;
    (local_BX_5.field_0xe * 6 + param_1) = *param_2;
    return;
}

pub fn pass1_1030_877c(param_1: u32)

{
    pass1_1030_8834(param_1);
    return;
}




pub fn pass1_1030_878c(param_1: *mut long)

{
    let puVar1: *mut u32;
    let paVar2: *mut astruct_94;
    let paVar3: *mut astruct_94;
    let mut uVar4: i32;
    let extraout_DX: *mut u16;
    let local_BX_4: *mut astruct_947;
    let mut uVar5: i32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x4 == 0x0)
    {
        g_u16_ptr_1050_5f2e = 0x0;
        paVar3 = local_BX_4.field_0x6;
    }
    else
    {
        paVar2 = local_BX_4.field_0x4;
        puVar1 = &local_BX_4.field_0x8;
        paVar3 = (&paVar2.field_0x0 + *puVar1);
        g_u16_ptr_1050_5f2e = CARRY2(paVar2, *puVar1);
    }
    if (g_u16_ptr_1050_5f2e == 0x0)
    {
        if (*param_1 == 0)
        {
            if (__g_astruct_94_ptr_1 == 0)
            {
                struct_fn_1000_160a();
            }
            else
            {
            }
            uVar4 = paVar3 * 6;
            alloc_mem_1000_1708(uVar4, 0, 1);
        }
        else
        {
            uVar4 = paVar3 * 6;
            alloc_mem_1000_0ed4(1, uVar4, 0, *param_1);
            g_u16_ptr_1050_5f2e = extraout_DX;
        }
        _local_c = CONCAT22(g_u16_ptr_1050_5f2e, uVar4);
        if ((g_u16_ptr_1050_5f2e | uVar4) != 0)
        {
            local_BX_4.field_0x4 = paVar3;
            *param_1 = _local_c;
            pass1_1030_8834((param_1 & 0xffff | uVar5 << 0x10));
        }
    }
    return;
}

pub fn pass1_1030_8834(param_1: *mut u16)

{
    pass1_1000_4aea(*param_1, (param_1 + 2), 6, 0x888e, 0x1030);
    return;
}

pub fn pass1_1030_8854(param_1: u32, param_2: u16)

{
    let mut u_var1: u32;
    let mut unaff_SS: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_8 = param_2;
    local_c = 0;
    uVar1 = (param_1 + 2);
    pass1_1000_49c6(&local_c, unaff_SS, param_1, uVar1, (uVar1 >> 0x10), 6,
                    0x888e);
    return;
}

pub fn pass1_1030_888e(param_1: u32,param_2: u32)

{
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = (param_1 + 4);
    uVar4 = (param_2 >> 0x10);
    piVar1 = (param_2 + 4);
    if (*piVar1 != iVar2 && iVar2 <= *piVar1)
    {
        return 0xffff;
    }
    if ((param_2 + 4) < (param_1 + 4))
    {
        return 1;
    }
    return 0;
}

pub fn pass1_1030_88ce(param_1: *mut astruct_942,param_2: u32,param_3: u32)

{
    let struct_a: *mut astruct_199;
    let paVar1: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let local_BX_6: *mut astruct_942;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut uVar3: u32;
    let uVar4: u8;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: [u8;2];
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: [u8;6];
    let mut local_14: [u8;6];
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    param_1.field_0x0 = s_1_1050_389a;
    local_BX_6.field_0x2 = &PTR_LOOP_1050_1008;
    pass1_1030_84ae(&local_BX_6.field_0x4, uVar2);
    local_BX_6.field_0x24 = param_3;
    zero_list_1008_6c90(&local_BX_6.field_0x28, uVar2);
    &local_BX_6.field_0x34 = 0;
    param_1.field_0x0 = 0x8e38;
    local_BX_6.field_0x2 = 0x1030;
    pass1_1030_8544((param_1 & 0xffff0000 | ZEXT24(&local_BX_6.field_0x4)),
                    param_2);
    uVar3 = process_struct_1008_4772(local_BX_6.field_0x12);
    local_4 = (uVar3 >> 0x10);
    local_6 = uVar3;
    local_a = (local_6 + 4);
    local_e = (local_6 + 8);
    uVar4 = (unaff_SS >> 8);
    pass1_1008_3e54(CONCAT13(uVar4, CONCAT12(unaff_SS, local_14)), 0, local_e - 1,
                    local_a - 1);
    pass1_1008_3e54(CONCAT13(uVar4, CONCAT12(unaff_SS, local_1a)), 0, 0, 0);
    uVar4 = &local_BX_6.field_0x28;
    modify_list_1008_6d18(uVar4, uVar2, 0xec, unaff_SS, local_1a, unaff_SS);
    modify_list_1008_6d64(uVar4, uVar2, local_1a, unaff_SS);
    pass1_1008_3eb4(CONCAT22(unaff_SS, local_1a), CONCAT22(unaff_SS, local_20),
                    CONCAT22(unaff_SS, &local_1e), CONCAT22(unaff_SS, &local_1c));
    struct_a = ((local_1e * local_1c) >> 0x10);
    uVar3 = local_1e * local_1c & 0xffff;
    local_BX_6.field_0x34 = uVar3;
    local_BX_6.field_0x36 = struct_a;
    local_22 = 0;
    while (local_22 < 5)
    {
        process_struct_1000_179c(0x10, struct_a);
        paVar1 = (struct_a | uVar3);
        if (paVar1 == 0x0)
        {
            (&local_BX_6.field_0x38 + local_22 * 4) = 0;
        }
        else
        {
            pass1_1030_85be((uVar3 & 0xffff | ZEXT24(struct_a) << 0x10), 0x19, 100);
            (&local_BX_6.field_0x38 + local_22 * 4) = uVar3;
            (&local_BX_6[1].field_0x0)[local_22 * 2] = extraout_DX;
            paVar1 = extraout_DX;
        }
        local_22 = local_22 + 1;
        struct_a = paVar1;
    }
    return;
}

pub fn pass1_1030_8a2c(param_1: *mut astruct_44)

{
    let mut uVar1: i32;
    let in_struct_1: *mut astruct_44;
    let mut uVar2: i32;
    let local_BX_5: *mut astruct_949;
    let mut uvar3: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1.ptr_a_lo = 0x8e38;
    local_BX_5.field_0x2 = 0x1030;
    local_4 = 0;
    while
    {
        in_struct_1 = (&local_BX_5.field_0x38 + local_4 * 4);
        uVar1 = (&local_BX_5.field_0x38 + local_4 * 4 + 2);
        uVar2 = in_struct_1;
        if ((uVar1 | uVar2) != 0)
        {
            pass1_1030_8604(uVar2, uVar1);
            error_check_1000_17ce(in_struct_1);
        }
        local_4 = local_4 + 1;
        local_4 < 5
    } {}
    pass1_1030_84d0((param_1 & 0xffff0000 | &local_BX_5.field_0x4));
    param_1.ptr_a_lo = s_1_1050_389a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1030_8aa0(param_1: *mut astruct_104,param_2: u32, param_3: *mut u16)

{
    let uVar1: u8;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut extraout_DX: i32;
    let mut unaff_SS: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1008_3eb4(param_3, CONCAT22(unaff_SS, &local_8), CONCAT22(unaff_SS, &local_6),
                    CONCAT22(unaff_SS, &local_6 + 2));
    local_a = extraout_DX;
    uVar1 = pass1_1030_8cd2(param_1, local_8);
    local_c = CONCAT31(extraout_var, uVar1);
    if ((local_a | local_c) != 0)
    {
        uVar1 = pass1_1030_8d9e(param_1, local_6, (local_6 >> 0x10));
        local_e = CONCAT31(extraout_var_00, uVar1);
        local_12 = param_2;
        pass1_1030_8660(CONCAT22(local_a, local_c), CONCAT22(unaff_SS, &local_12), local_e);
    }
    return;
}



pub fn pass1_1030_8b00(in_struct_1: *mut astruct_104,param_2: u32,param_3: u32)

{
    let uVar1: u8;
    let puVar2: *mut u32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut local_2a: u32;
    let mut local_26: u32;
    let mut local_1c: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let local_a: bool;
    let mut local_8: [u8;2];
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = 0;
    pass1_1008_3eb4(param_2, CONCAT22(unaff_SS, &local_c), CONCAT22(unaff_SS, &local_a),
                    CONCAT22(unaff_SS, local_8));
    local_e = extraout_DX;
    uVar1 = pass1_1030_8cd2(in_struct_1, local_c);
    local_10 = CONCAT31(extraout_var, uVar1);
    uVar1 = pass1_1030_8d9e(in_struct_1, _local_a, (_local_a >> 0x10));
    puVar2 = CONCAT31(extraout_var_00, uVar1);
    local_12 = puVar2;
    pass1_1030_861a(local_10, local_e, puVar2);
    local_26 = *puVar2;
    uVar3 = (puVar2 + 2);
    local_26._3_1_ = (local_26 >> 0x18);
    _local_6 = local_26;
    if (local_26._3_1_ == '\0')
    {
        puVar2 = &local_2a;
        local_1c = local_26;
        pass1_1030_8c66(in_struct_1, local_c, _local_a, (_local_a >> 0x10),
                        CONCAT22(unaff_SS, puVar2));
        _local_6 = *puVar2;
        uVar3 = (puVar2 + 2);
    }
    param_3 = local_6;
    (param_3 + 2) = uVar3;
    return;
}

pub fn pass1_1030_8bac(param_1: u32, param_2: u16)

{
    let mut local_4: u16;

    local_4 = 0;
    while
    {
        pass1_1030_86ec((param_1 + 0x38 + local_4 * 4), param_2);
        local_4 = local_4 + 1;
        local_4 < 5
    } {}
    return;
}

pub fn pass1_1030_8bdc(param_1: *mut astruct_104,param_2: u32, param_3: *mut u16)

{
    let uVar1: u8;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut extraout_DX: u16;
    let mut unaff_SS: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1008_3eb4(param_3, CONCAT22(unaff_SS, &local_8), CONCAT22(unaff_SS, &local_6),
                    CONCAT22(unaff_SS, &local_6 + 2));
    local_a = extraout_DX;
    uVar1 = pass1_1030_8cd2(param_1, local_8);
    local_c = CONCAT31(extraout_var, uVar1);
    uVar1 = pass1_1030_8d9e(param_1, local_6, (local_6 >> 0x10));
    local_e = CONCAT31(extraout_var_00, uVar1);
    local_12 = param_2;
    pass1_1030_871e(CONCAT22(local_a, local_c), CONCAT22(unaff_SS, &local_12),
                    local_e);
    return;
}

pub fn pass1_1030_8c38(param_1: u32)

{
    let mut local_4: u16;

    local_4 = 0;
    while
    {
        pass1_1030_877c((param_1 + 0x38 + local_4 * 4));
        local_4 = local_4 + 1;
        local_4 < 5
    } {}
    return;
}


pub fn pass1_1030_7bee(param_1: *mut astruct_493)

{
    let mut uVar1: u16;
    let local_BX_3: *mut astruct_493;
    let mut uVar2: i32;
    let fn_ptr_a: fn();

    uVar2 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0x16 == 0)
    {
        return 0;
    }
    if (&local_BX_3.field_0x1a == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar2 << 0x10));
    }
    fn_ptr_a = (&local_BX_3.field_0x1a + 0x44);
    uVar1 = (**fn_ptr_a)();
    return uVar1;
}



pub fn pass1_1030_7c28(param_1: u32, param_2: u16)

{
    let mut local_ES_3: u16;
    let mut temp_5ff154ccf8: u32;

    local_ES_3 = (param_1 >> 0x10);
    if ((param_1 + 0x22) == 0)
    {
        return;
    }
    temp_5ff154ccf8 = (param_1 + 0x22);
    pass1_1020_bae6(temp_5ff154ccf8, CONCAT22(param_2, (temp_5ff154ccf8 >> 0x10)));
    return;
}




pub fn pass1_1030_7c50(param_1: u32, param_2: libc::c_long, param_3: u16)

{
    let piVar1: *mut i32;
    let ppcVar2: fn();
    let mut in_AX: i32;
    let paVar3: *mut astruct_493;
    let mut uVar4: u32;
    let in_DX: *mut astruct_199;
    let paVar5: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let mut extraout_DX_00: u16;
    let mut uVar6: u16;
    let extraout_DX_01: *mut astruct_199;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut uVar9: u32;
    let mut uVar10: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_4: u16;

    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    paVar5 = in_DX;
    if ((iVar7 + 0x1e) == 0)
    {
        process_struct_1000_179c(0x18, in_DX);
        paVar5 = (in_DX | in_AX);
        if (paVar5 == 0x0)
        {
            (iVar7 + 0x1e) = 0;
        }
        else
        {
            pass1_1030_1cd8(CONCAT22(in_DX, in_AX), 5, 5);
            (iVar7 + 0x1e) = in_AX;
            (iVar7 + 0x20) = extraout_DX;
            paVar5 = extraout_DX;
        }
    }
    if (param_3 == 4)
    {
        piVar1 = (iVar7 + 0x34);
        *piVar1 = *piVar1 + param_2;
    }
    while (param_2 != 0)
    {
        uVar4 = SEXT24(param_3);
        pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
        uVar10 = uVar4 & 0xffff | ZEXT24(paVar5) << 0x10;
        uVar9 = (iVar7 + 0x1e);
        ppcVar2 = ((iVar7 + 0x1e) + 0xc);
        ppcVar2();
        uVar6 = extraout_DX_00;
        paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, paVar5);
        _local_12 = CONCAT22(uVar6, paVar3);
        ppcVar2 = (*_local_12 + 0x14);
        ppcVar2(&PTR_LOOP_1050_1028, paVar3, uVar6, param_1, uVar9, uVar10);
        paVar5 = extraout_DX_01;
        param_2 = param_2 + -1;
    }
    return;
}

pub fn pass1_1030_7d1c(param_1: u32, param_2: u16,param_3: u32)

{
    let mut in_AX: i32;
    let in_DX: *mut astruct_199;
    let local_BX_4: *mut astruct_938;
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut local_4: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x22 == 0)
    {
        process_struct_1000_179c(10, in_DX);
        if ((in_DX | in_AX) == 0)
        {
            &local_BX_4.field_0x22 = 0;
        }
        else
        {
            uVar2 = pass1_1020_ba3e(CONCAT22(in_DX, in_AX), 10, 2);
            local_BX_4.field_0x22 = uVar2;
            &local_BX_4.field_0x24 = (uVar2 >> 0x10);
        }
    }
    uVar2 = &local_BX_4.field_0x22;
    pass1_1020_bb8a(uVar2, (uVar2 >> 0x10), param_2, param_3);
    return;
}

pub fn pass1_1030_7d7c(param_1: *mut astruct_939, param_2: u16,param_3: u32)

{
    let mut in_AX: i32;
    let in_DX: *mut astruct_199;
    let local_BX_4: *mut astruct_939;
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut local_4: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x26 == 0)
    {
        process_struct_1000_179c(10, in_DX);
        if ((in_DX | in_AX) == 0)
        {
            &local_BX_4.field_0x26 = 0;
        }
        else
        {
            uVar2 = pass1_1020_ba3e(CONCAT22(in_DX, in_AX), 10, 2);
            local_BX_4.field_0x26 = uVar2;
            &local_BX_4.field_0x28 = (uVar2 >> 0x10);
        }
    }
    uVar2 = &local_BX_4.field_0x26;
    pass1_1020_bb8a(uVar2, (uVar2 >> 0x10), param_2, param_3);
    return;
}

pub fn pass1_1030_7ddc(param_1: u32, param_2: libc::c_long, param_3: u16)

{
    let mut u_var1: u32;
    let lVar2: u32;
    let in_AX: *mut astruct_199;
    let mut uVar3: u32;
    let in_DX: *mut astruct_199;
    let paVar4: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let local_BX_4: *mut astruct_940;
    let mut uVar5: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    paVar4 = in_DX;
    if (&local_BX_4.field_0x22 == 0)
    {
        process_struct_1000_179c(10, in_DX);
        paVar4 = (in_DX | in_AX);
        if (paVar4 == 0x0)
        {
            &local_BX_4.field_0x22 = 0;
        }
        else
        {
            pass1_1020_ba3e(CONCAT22(in_DX, in_AX), 10, 2);
            local_BX_4.field_0x22 = in_AX;
            &local_BX_4.field_0x24 = extraout_DX;
            paVar4 = extraout_DX;
        }
    }
    uVar1 = &local_BX_4.field_0x22;
    uVar3 = pass1_1020_bae6(uVar1, CONCAT22(param_3, (uVar1 >> 0x10)));
    lVar2 = (uVar3 & 0xffff | ZEXT24(paVar4) << 0x10) + param_2;
    uVar1 = &local_BX_4.field_0x22;
    pass1_1020_bb8a(uVar1, (uVar1 >> 0x10), lVar2, (lVar2 >> 0x10),
                    param_3);
    return;
}

pub fn pass1_1030_7e5a(param_1: u32,param_2: u32)

{
    let mut iVar1: i32;
    let mut uVar2: i32;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x16) = param_2;
    (iVar1 + 0x1a) = 0;
    pass1_1030_6fa0((param_1 & 0xffff | uVar2 << 0x10));
    if ((iVar1 + 0x2e) != 0)
    {
        pass1_1038_4b20(*(iVar1 + 0x2e), *(iVar1 + 0x16), (iVar1 + 4));
    }
    return;
}



pub fn pass1_1030_7ea0(param_1: *mut astruct_918) -> bool

{
    let mut u_var1: u32;
    let uVar2: u8;
    let extraout_AH: u8;
    let BVar3: bool;

    uVar2 = pass1_1030_6fa0(param_1);
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT11(extraout_AH, uVar2), 0xb);
    if (BVar3 != 0)
    {
        uVar1 = (param_1 + 0x1a);
        if ((uVar1 + 0x12) == 5)
        {
            return 1;
        }
        BVar3 = 0;
    }
    return BVar3;
}

pub fn pass1_1030_7eda(param_1: u32, param_2: u16)

{
    let mut uVar1: i32;
    let mut unaff_SS: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_c = 0;
    local_a = 0;
    local_6 = 0;
    local_4 = 0;
    local_8 = param_2;
    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar1 << 0x10));
    }
    pass1_1028_bb96((param_1 + 0x1a), &local_c, unaff_SS);
    return;
}

pub fn pass1_1030_7f1a(param_1: u32, param_2: u16)

{
    let mut uVar1: i32;
    let mut unaff_SS: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_c = 0;
    local_8 = 0;
    local_6 = 0;
    local_4 = 0;
    local_a = param_2;
    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar1 << 0x10));
    }
    pass1_1028_bb96((param_1 + 0x1a), &local_c, unaff_SS);
    return;
}

pub fn pass1_1030_7f5a(param_1: u32)

{
    let mut uVar1: i32;
    let mut uVar2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar1 << 0x10));
    }
    uVar2 = pass1_1028_bb6a((param_1 + 0x1a));
    if (uVar2 != 0)
    {
        return (uVar2 + 4);
    }
    return 0;
}

pub fn pass1_1030_7f98(param_1: u32)

{
    let mut uVar1: i32;
    let mut uVar2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar1 << 0x10));
    }
    uVar2 = pass1_1028_bb6a((param_1 + 0x1a));
    if (uVar2 != 0)
    {
        return (uVar2 + 2);
    }
    return 0;
}

pub fn pass1_1030_7fd6(param_1: u32)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let mut local_6: u32;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar4 << 0x10));
    }
    uVar2 = (iVar3 + 0x1a);
    iVar1 = (uVar2 + 0xc);
    if (((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33))) &&
        ((iVar1 == 0x34 || iVar1 + -0x33 < 1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 2))))))
    {
        pass1_1028_1416((iVar3 + 0x1a));
    }
    return;
}

pub fn pass1_1030_8030(param_1: u32)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar4 << 0x10));
    }
    uVar2 = (iVar3 + 0x1a);
    iVar1 = (uVar2 + 0xc);
    if (((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33))) &&
        ((iVar1 == 0x34 || iVar1 + -0x33 < 1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 2))))))
    {
        pass1_1028_1106((iVar3 + 0x1a));
    }
    return;
}

pub fn pass1_1030_8086(param_1: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x18), (param_1 + 0x16)) &
           0xffffff;
}

pub fn pass1_1030_809c(param_1: u32)

{
    let mut uVar1: i32;
    let mut local_a: u32;
    let mut local_4: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar1 << 0x10));
    }
    return;
}

pub fn pass1_1030_80ee(param_1: u32, param_2: u8)

{
    pass1_1030_68dc(param_1);
    if ((param_2 & 1) != 0)
    {
        free_mem_1000_093a(param_1);
    }
    return param_1;
}



pub fn pass1_1030_8128(param_1: u32) -> bool

{
    let mut uVar1: i32;
    let in_DX: *mut astruct_199;
    let paVar2: *mut astruct_199;
    let paVar3: *mut astruct_199;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar1 = 0;
    param_1 = 0;
    (iVar4 + 4) = 0;
    (iVar4 + 8) = 0;
    _g_bool_1050_5748 = param_1;
    process_struct_1000_179c(0x56, in_DX);
    paVar2 = (in_DX | uVar1);
    if (paVar2 != 0x0)
    {
        pass1_1028_d81c(CONCAT22(in_DX, uVar1), param_1);
    }
    process_struct_1000_179c(8, paVar2);
    paVar3 = (paVar2 | uVar1);
    if (paVar3 == 0x0)
    {
        param_1 = 0;
    }
    else
    {
        pass1_1028_d22e(CONCAT22(paVar2, uVar1), param_1);
        param_1 = uVar1;
        (iVar4 + 2) = paVar3;
    }
    process_struct_1000_179c(8, paVar3);
    paVar2 = (paVar3 | uVar1);
    if (paVar2 == 0x0)
    {
        (iVar4 + 4) = 0;
    }
    else
    {
        pass1_1028_cfd2(CONCAT22(paVar3, uVar1), param_1);
        (iVar4 + 4) = uVar1;
        (iVar4 + 6) = paVar2;
    }
    process_struct_1000_179c(0x24, paVar2);
    paVar3 = (paVar2 | uVar1);
    if (paVar3 != 0x0)
    {
        pass1_1030_5bec(CONCAT22(paVar2, uVar1));
    }
    process_struct_1000_179c(8, paVar3);
    if ((paVar3 | uVar1) != 0)
    {
        pass1_1038_78e2(CONCAT22(paVar3, uVar1));
    }
    PTR_LOOP_1050_574a = (_g_bool_1050_5748 >> 0x10);
    return SUB41(param_1, 0);
}



pub fn pass1_1030_8210(param_1: *mut u32)

{
    let mut uVar1: i32;
    let mut uVar2: i32;
    let paVar3: *mut astruct_44;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    paVar3 = _PTR_LOOP_1050_65e2;
    if (_PTR_LOOP_1050_65e2 != 0x0)
    {
        pass1_1028_daba(_PTR_LOOP_1050_65e2);
        error_check_1000_17ce(paVar3);
    }
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar1 = *param_1;
    uVar2 = (iVar4 + 2);
    local_a = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0)
    {
        pass1_1028_d282(CONCAT22(uVar2, uVar1));
        error_check_1000_17ce(local_a);
    }
    uVar1 = (iVar4 + 4);
    uVar2 = (iVar4 + 6);
    local_6 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0)
    {
        pass1_1028_cff2(CONCAT22(uVar2, uVar1));
        error_check_1000_17ce(local_6);
    }
    paVar3 = _PTR_LOOP_1050_5736;
    if (_PTR_LOOP_1050_5736 != 0x0)
    {
        pass1_1030_5c0e();
        error_check_1000_17ce(paVar3);
    }
    paVar3 = _PTR_LOOP_1050_5a64;
    if ((PTR_LOOP_1050_5a66 | _PTR_LOOP_1050_5a64) != 0)
    {
        pass1_1038_7964((_PTR_LOOP_1050_5a64 & 0xffff | ZEXT24(PTR_LOOP_1050_5a66) << 0x10));
        error_check_1000_17ce(paVar3);
    }
    _g_bool_1050_5748 = 0;
    return;
}

pub fn pass1_1030_82f0(param_1: *mut bool,param_2: u32)

{
    pass1_1028_d078((param_1 + 4), param_2);
    return;
}

pub fn pass1_1030_8308(param_1: u16, param_2: u16,param_1_00: u32,param_2_00: u32,param_5: u32) -> i32

{
    pass1_1028_e198(_PTR_LOOP_1050_65e2, param_1_00, param_2_00, param_5);
    return;
}



pub fn pass1_1030_8326()

{
    return CONCAT22((_PTR_LOOP_1050_65e2 + 2), *_PTR_LOOP_1050_65e2);
}



pub fn pass1_1030_8334()

{
    *_PTR_LOOP_1050_65e2 = 0;
    return;
}



pub fn pass1_1030_8344(in_bool_1: bool, in_bool_2: bool,param_1_00: u32) -> *mut pass1_struct_2

{
    let ppVar1: *mut pass1_struct_2;

    ppVar1 = 
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_1_00, (param_1_00 >> 0x10));
    return ppVar1;
}

pub fn pass1_1030_835a(param_1: *mut u32,param_2: u32)

{
    pass1_1028_d566(*param_1, param_2);
    return;
}

pub fn pass1_1030_8372(param_1: *mut *mut u32,param_2: u32, param_3: *mut u32)

{
    pass1_1028_d52c(*param_1, param_2, param_3);
    return;
}


pub fn pass1_1030_6118(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_5d78(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1030_615a(param_1: *mut u32)

{
    let mut uVar1: i32;
    let struct_b_1: *mut astruct_199;
    let struct_b_2: *mut astruct_199;
    let mut local_DX_40: u16;
    let local_BX_4: *mut astruct_914;
    let mut local_ES_4: u16;
    let mut local_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar1 = 0;
    *param_1 = 0;
    &local_BX_4.field_0x4 = 0;
    process_struct_1000_179c(0xc, struct_b_2);
    local_DX_40 = struct_b_2 | uVar1;
    if (local_DX_40 == 0)
    {
        &local_BX_4.field_0x4 = 0;
    }
    else
    {
        struct_b_1 = process_struct_1008_574a(CONCAT22(struct_b_2, uVar1));
        local_BX_4.field_0x4 = struct_b_1;
        local_BX_4.field_0x6 = local_DX_40;
    }
    _PTR_LOOP_1050_5740 = param_1;
    return;
}



pub fn pass1_1030_61b0(param_1: *mut u32)

{
    let mut uVar1: i32;
    let puVar2: *mut u32;
    let ppcVar3: fn();
    let mut iVar4: i32;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar1 = (iVar4 + 2);
    if ((uVar1 | *param_1) != 0)
    {
        ppcVar3 = *param_1;
        (**ppcVar3)();
    }
    puVar2 = (iVar4 + 4);
    uVar1 = (iVar4 + 6);
    if ((uVar1 | puVar2) != 0)
    {
        ppcVar3 = *puVar2;
        (**ppcVar3)();
    }
    _PTR_LOOP_1050_5740 = 0;
    return;
}

pub fn pass1_1030_61fe(param_1: u32,param_2: u32,param_3: u32,param_4: u32)

{
    let mut u_var1: u32;

    uVar1 = pass1_1030_677a(param_1, param_4);
    pass1_1030_8aa0(uVar1, param_2, param_3);
    return;
}

pub fn pass1_1030_6222(param_1: u32, param_2: i32,param_3: u32,param_4: u32)

{
    let ppcVar1: fn();
    let mut in_AX: i32;
    let in_DX: *mut astruct_199;
    let mut uVar2: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x4c, in_DX);
    if ((in_DX | in_AX) == 0)
    {
        uVar2 = 0;
    }
    else
    {
        uVar2 = pass1_1030_88ce(CONCAT22(in_DX, in_AX), param_3, param_4);
    }
    ppcVar1 = ((param_1 + 4) + 4);
    (**ppcVar1)();
    if (param_2 != 0)
    {
        pass1_1030_8d08(uVar2);
    }
    return 0;
}



pub fn pass1_1030_627e(param_1: u32,param_2: u32,param_3: u32)

{
    let mut in_AX: i32;
    let mut extraout_DX: i32;
    let mut unaff_SS: u16;
    let mut local_1c: u32;
    let mut local_12: u32;
    let mut local_e: u32;
    let local_a: *mut astruct_104;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    pass1_1030_677a(param_1, param_3);
    _local_a = CONCAT22(extraout_DX, in_AX);
    if ((extraout_DX | in_AX) != 0)
    {
        pass1_1030_8b00(_local_a, param_2, CONCAT22(unaff_SS, &local_12));
    }
    return;
}



pub fn pass1_1030_62e4(param_1: *mut u32,param_2: u32,param_3: u32)

{
    let ppcVar1: fn();
    let puVar2: *mut u32;
    let mut uVar3: i32;
    let extraout_DX: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut extraout_DX_00: u16;
    let mut uVar4: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let uVar6: u8;
    let uVar7: u8;
    let mut local_64: u16;
    let mut local_5e: u16;
    let mut local_5c: u16;
    let mut local_58: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: [u8;2];
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: [u8;2];
    let mut local_20: [u8;2];
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: [u8;6];
    let mut local_12: [u8;6];
    let mut local_c: [u8;6];
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    puVar2 = param_1;
    struct_a = (param_1 + 2);
    local_36 = puVar2;
    local_34 = struct_a;
    local_32 = puVar2;
    local_30 = struct_a;
    if ((struct_a | puVar2) != 0)
    {
        ppcVar1 = *puVar2;
        (**ppcVar1)();
        struct_a = extraout_DX;
    }
    process_struct_1000_179c(0x18, struct_a);
    local_36 = puVar2;
    local_34 = struct_a;
    if ((struct_a | puVar2) == 0)
    {
        puVar2 = 0x0;
        uVar4 = 0;
    }
    else
    {
        pass1_1030_1cd8(
                            CONCAT13((struct_a >> 8), CONCAT12(struct_a, puVar2)),
                        5, 5);
        uVar4 = extraout_DX_00;
    }
    param_1 = puVar2;
    (param_1 + 2) = uVar4;
    pass1_1030_677a(param_1, param_3);
    _local_6 = CONCAT22(extraout_DX_01, puVar2);
    if ((extraout_DX_01 | puVar2) != 0)
    {
        zero_list_1008_3e38(CONCAT22(unaff_SS, local_c));
        zero_list_1008_3e38(CONCAT22(unaff_SS, local_12));
        zero_list_1008_3e38(CONCAT22(unaff_SS, local_18));
        modify_list_1008_6d3e(param_2, (param_2 >> 0x10), 0xee, unaff_SS, local_c, unaff_SS);
        uVar6 = unaff_SS;
        uVar7 = (unaff_SS >> 8);
        pass1_1008_3eb4(CONCAT22(unaff_SS, local_c), CONCAT13(uVar7, CONCAT12(uVar6, &local_1e)),
                        CONCAT22(unaff_SS, &local_1c), CONCAT22(unaff_SS, &local_1a));
        pass1_1008_3eb4(CONCAT22(unaff_SS, local_12), CONCAT13(uVar7, CONCAT12(uVar6, &local_24)),
                        CONCAT22(unaff_SS, local_22), CONCAT22(unaff_SS, local_20));
        modify_list_1008_6d64(param_2, (param_2 >> 0x10), local_18, unaff_SS);
        pass1_1008_3eb4(CONCAT22(unaff_SS, local_18), CONCAT13(uVar7, CONCAT12(uVar6, local_2a)),
                        CONCAT22(unaff_SS, &local_28), CONCAT22(unaff_SS, &local_26));
        if (local_24 == local_1e)
        {
            local_2c = 0;
            local_2e = local_1c;
            while (uVar3 = local_28 + local_1c, local_2e < uVar3)
            {
                local_38 = local_1a;
                while (local_38 < (local_26 + local_1a))
                {
                    local_58 = local_1e;
                    pass1_1008_3e54(CONCAT13(uVar7, CONCAT12(uVar6, &local_64)), local_1e, local_2e,
                                    local_38);
                    pass1_1030_8b00(_local_6, CONCAT22(unaff_SS, &local_64), CONCAT22(unaff_SS, &local_40));
                    local_3c = local_40;
                    local_64 = local_2c;
                    local_3c._0_2_ = local_40;
                    local_4e = local_3c;
                    local_4c = local_40._2_2_;
                    local_4c._1_1_ = (local_40 >> 0x18);
                    if (local_4c._1_1_ == '\0')
                    {
                        local_3c._0_2_ = 0;
                        local_40._2_2_ = 0;
                    }
                    _local_5e = CONCAT22(local_40._2_2_, local_3c);
                    ppcVar1 = (*param_1 + 8);
                    local_2c = local_2c + 1;
                    (**ppcVar1)();
                    local_38 = local_38 + 1;
                }
                local_2e = local_2e + 1;
            }
            ppcVar1 = (*param_1 + 0x10);
            (**ppcVar1)(&PTR_LOOP_1050_1008, *param_1);
            if ((extraout_DX_02 | uVar3) != 0)
            {
                return;
            }
        }
    }
    return;
}



pub fn pass1_1030_64ce(param_1: u32,param_2: u32,param_3: u32, param_4: *mut u32)

{
    let mut in_AX: i32;
    let puVar1: *mut u32;
    let mut extraout_DX: i32;
    let mut local_DX_50: u16;
    let mut local_SS__1: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    pass1_1030_677a(param_1, param_3);
    _local_a = CONCAT22(extraout_DX, in_AX);
    if ((extraout_DX | in_AX) != 0)
    {
        puVar1 = &local_e;
        pass1_1030_8b00(_local_a, param_2, CONCAT22(local_SS__1, puVar1));
        local_6 = *puVar1;
    }
    *param_4 = local_6;
    return;
}



pub fn pass1_1030_6522(param_1: *mut u32,param_2: u32,param_3: u32)

{
    let ppcVar1: fn();
    let puVar2: *mut u32;
    let mut uVar3: i32;
    let extraout_DX: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut extraout_DX_00: u16;
    let mut uVar4: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let uVar6: u8;
    let uVar7: u8;
    let mut local_64: [u8;12];
    let mut local_58: u16;
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: [u8;2];
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: [u8;2];
    let mut local_20: [u8;2];
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: [u8;6];
    let mut local_12: [u8;6];
    let mut local_c: [u8;6];
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    puVar2 = param_1;
    struct_a = (param_1 + 2);
    local_36 = puVar2;
    local_34 = struct_a;
    local_32 = puVar2;
    local_30 = struct_a;
    if ((struct_a | puVar2) != 0)
    {
        ppcVar1 = *puVar2;
        (**ppcVar1)();
        struct_a = extraout_DX;
    }
    process_struct_1000_179c(0x18, struct_a);
    local_36 = puVar2;
    local_34 = struct_a;
    if ((struct_a | puVar2) == 0)
    {
        puVar2 = 0x0;
        uVar4 = 0;
    }
    else
    {
        pass1_1030_1cd8(
                            CONCAT13((struct_a >> 8), CONCAT12(struct_a, puVar2)),
                        5, 5);
        uVar4 = extraout_DX_00;
    }
    param_1 = puVar2;
    (param_1 + 2) = uVar4;
    pass1_1030_677a(param_1, param_3);
    _local_6 = CONCAT22(extraout_DX_01, puVar2);
    if ((extraout_DX_01 | puVar2) != 0)
    {
        zero_list_1008_3e38(CONCAT22(unaff_SS, local_c));
        zero_list_1008_3e38(CONCAT22(unaff_SS, local_12));
        zero_list_1008_3e38(CONCAT22(unaff_SS, local_18));
        modify_list_1008_6d3e(param_2, (param_2 >> 0x10), 0xee, unaff_SS, local_c, unaff_SS);
        uVar6 = unaff_SS;
        uVar7 = (unaff_SS >> 8);
        pass1_1008_3eb4(CONCAT22(unaff_SS, local_c), CONCAT13(uVar7, CONCAT12(uVar6, &local_1e)),
                        CONCAT22(unaff_SS, &local_1c), CONCAT22(unaff_SS, &local_1a));
        pass1_1008_3eb4(CONCAT22(unaff_SS, local_12), CONCAT13(uVar7, CONCAT12(uVar6, &local_24)),
                        CONCAT22(unaff_SS, local_22), CONCAT22(unaff_SS, local_20));
        modify_list_1008_6d64(param_2, (param_2 >> 0x10), local_18, unaff_SS);
        pass1_1008_3eb4(CONCAT22(unaff_SS, local_18), CONCAT13(uVar7, CONCAT12(uVar6, local_2a)),
                        CONCAT22(unaff_SS, &local_28), CONCAT22(unaff_SS, &local_26));
        if (local_24 == local_1e)
        {
            local_2c = 0;
            local_2e = local_1c;
            while (uVar3 = local_28 + local_1c, local_2e < uVar3)
            {
                local_38 = local_1a;
                while (local_38 < (local_26 + local_1a))
                {
                    local_58 = local_1e;
                    pass1_1008_3e54(CONCAT13(uVar7, CONCAT12(uVar6, local_64)), local_1e, local_2e, local_38);
                    pass1_1030_8b00(_local_6, CONCAT22(unaff_SS, local_64), CONCAT22(unaff_SS, &local_40));
                    local_3c = local_40;
                    local_2c = local_2c + 1;
                    ppcVar1 = (*param_1 + 8);
                    (**ppcVar1)();
                    local_38 = local_38 + 1;
                }
                local_2e = local_2e + 1;
            }
            ppcVar1 = (*param_1 + 0x10);
            (**ppcVar1)(&PTR_LOOP_1050_1008, *param_1);
            if ((extraout_DX_02 | uVar3) != 0)
            {
                return;
            }
        }
    }
    return;
}

pub fn pass1_1030_66de(param_1: u32,param_2: u32)

{
    let mut unaff_SS: u16;
    let mut u_var1: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8;8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 4));
    loop
    {
        uVar1 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        if (uVar1 == 0) {
            break;
        }
        pass1_1030_8bac(uVar1, param_2);
    }
    return;
}

pub fn pass1_1030_671c(param_1: u32,param_2: u32,param_3: u32,param_4: u32)

{
    let paVar1: *mut astruct_104;

    paVar1 = pass1_1030_677a(param_1, param_4);
    pass1_1030_8bdc(paVar1, param_2, param_3);
    return;
}

pub fn pass1_1030_6740(param_1: u32)

{
    let mut unaff_SS: u16;
    let mut u_var1: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8;8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 4));
    loop
    {
        uVar1 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
        if (uVar1 == 0) {
            break;
        }
        pass1_1030_8c38(uVar1);
    }
    return;
}

pub fn pass1_1030_677a(param_1: u32,param_2: u32)

{
    let puVar1: *mut u8;
    let mut extraout_DX: i32;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8;8];

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 4) == 0)
    {
        return;
    }
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 4));
    while
    {
        puVar1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_SS, puVar1));
        if ((extraout_DX | puVar1) == 0)
        {
            return;
        }
        (puVar1 + 0x24) != param_2
    } {}
    return;
}

pub fn pass1_1030_67cc(struct_a: *mut astruct_1)

{
    let struct_b: *mut astruct_1;
    let struct_c: *mut astruct_1;

    pass1_1030_1628(struct_a);
    struct_b = struct_a;
    struct_b = &struct_b.field_0xc;
    zero_list_1008_3e38((struct_a & 0xffff0000 | ZEXT24(struct_b)));
    struct_c = (struct_a >> 0x10);
    struct_b.field_0x12 = 0;
    struct_b.field_0x14 = 0;
    struct_b.field_0x16 = 0;
    struct_b.field_0x1a = 0;
    struct_b.field_0x1e = 0;
    struct_b.field_0x22 = 0;
    struct_b.field_0x26 = 0;
    struct_b.field_0x2a = 0;
    struct_b.field_0x2e = 0;
    struct_b.field_0x32 = 0;
    struct_b.field_0x34 = 0;
    struct_b.field_0x38 = 0;
    struct_b.field_0x36 = 0;
    struct_b.field_0x3c = 0;
    struct_b.field_0x3a = 0;
    struct_b.field_0x40 = 0;
    struct_b.field_0x3e = 0;
    struct_a.field_0x0 = 0x8114;
    struct_b.field_0x2 = 0x1030;
    return;
}

pub fn pass1_1030_684c(param_1: *mut astruct_848, param_2: *mut u32, param_3: u16, param_4: u16, param_5: u16,param_5_00: u32) -> i32

{
    let local_BX_24: *mut astruct_916;
    let mut uVar1: u16;

    pass1_1030_165e(param_1, 0x5000000, param_5_00);
    uVar1 = (param_1 >> 0x10);
    local_BX_24 = param_1;
    local_BX_24.field_0xc = *param_2;
    local_BX_24.field_0x10 = (param_2 + 1);
    local_BX_24.field_0x12 = param_4;
    local_BX_24.field_0x14 = param_4;
    local_BX_24.field_0x16 = 0;
    local_BX_24.field_0x1a = 0;
    local_BX_24.field_0x1e = 0;
    local_BX_24.field_0x22 = 0;
    local_BX_24.field_0x26 = 0;
    local_BX_24.field_0x2a = 0;
    local_BX_24.field_0x2e = 0;
    local_BX_24.field_0x32 = 0;
    local_BX_24.field_0x34 = 0;
    local_BX_24.field_0x36 = 0;
    local_BX_24.field_0x3a = 0;
    local_BX_24.field_0x3e = 0;
    param_1.field_0x0 = 0x8114;
    local_BX_24.field_0x2 = 0x1030;
    return;
}

pub fn pass1_1030_68dc(param_1: *mut u16)

{
    let mut uVar1: i32;
    let mut uVar2: i32;
    let puVar3: *mut u32;
    let in_struct_1: *mut astruct_44;
    let ppcVar4: fn();
    let local_BX_4: *mut astruct_917;
    let mut uVar5: u16;
    let mut unaff_CS: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar5 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    *param_1 = 0x8114;
    local_BX_4.field_0x2 = 0x1030;
    in_struct_1 = &local_BX_4.field_0x22;
    uVar1 = &local_BX_4.field_0x24;
    if ((uVar1 | in_struct_1) != 0)
    {
        pass1_1020_ba7e((in_struct_1 & 0xffff | uVar1 << 0x10));
        unaff_CS = 0x1000;
        error_check_1000_17ce(in_struct_1);
    }
    uVar1 = local_BX_4.field_0x26;
    uVar2 = local_BX_4.field_0x28;
    local_a = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0)
    {
        pass1_1020_ba7e(CONCAT22(uVar2, uVar1));
        unaff_CS = 0x1000;
        error_check_1000_17ce(local_a);
    }
    puVar3 = local_BX_4.field_0x1e;
    uVar1 = local_BX_4.field_0x20;
    if ((uVar1 | puVar3) != 0)
    {
        ppcVar4 = *puVar3;
        (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
    }
    puVar3 = local_BX_4.field_0x36;
    uVar1 = local_BX_4.field_0x38;
    if ((uVar1 | puVar3) != 0)
    {
        ppcVar4 = *puVar3;
        (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
    }
    puVar3 = local_BX_4.field_0x3a;
    uVar1 = local_BX_4.field_0x3c;
    if ((uVar1 | puVar3) != 0)
    {
        ppcVar4 = *puVar3;
        (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
    }
    puVar3 = local_BX_4.field_0x3e;
    uVar1 = local_BX_4.field_0x40;
    if ((uVar1 | puVar3) != 0)
    {
        ppcVar4 = *puVar3;
        (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
    }
    pass1_1030_16b2(param_1);
    return;
}



pub fn pass1_1030_69cc(struct_a: *mut astruct_918)

{
    let uVar1: u8;
    let mut in_AX: i32;
    let BVar2: bool;
    let extraout_var: u32;
    let mut extraout_DX: i32;
    let struct_b: *mut astruct_918;
    let struct_b_hi: *mut astruct_918;
    let mut uVar3: u32;

    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    if (struct_b.field_0x3e != 0)
    {
        return;
    }
    if ((struct_b.field_0x22 != 0) &&
        (infinite_loop_1020_ba94(struct_b.field_0x22), (extraout_DX | in_AX) != 0))
    {
        return;
    }
    uVar1 = pass1_1030_6fa0(struct_a);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, uVar1), 4);
    if ((BVar2 != 0) &&
        (uVar3 = pass1_1028_67d4(struct_b.field_0x1a), ((uVar3 >> 0x10) | uVar3) != 0))
    {
        return;
    }
    return;
}

pub fn pass1_1030_6a2c(struct_a: *mut astruct_921, param_2: libc::c_long)

{
    let mut in_AX: i32;
    let struct_c: *mut astruct_199;
    let struct_d: *mut astruct_920;
    let struct_e: *mut astruct_199;
    let mut uVar1: i32;
    let mut local_DX_93: u16;
    let struct_b: *mut astruct_921;
    let struct_f: *mut astruct_919;
    let mut struct_b_hi: u16;
    let mut local_ES_108: u16;
    let mut local_SS__1: u16;
    let lVar2: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8;8];
    let fn_ptr_a: fn();

    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    if (&struct_b.field_0x3e == 0)
    {
        process_struct_1000_179c(0xc, struct_e);
        uVar1 = struct_e | in_AX;
        if (uVar1 == 0)
        {
            &struct_b.field_0x3e = 0;
        }
        else
        {
            struct_c = process_struct_1008_574a(CONCAT22(struct_e, in_AX));
            struct_b.field_0x3e = struct_c;
            struct_b.field_0x40 = uVar1;
        }
    }
    pass1_1008_5784(CONCAT22(local_SS__1, local_a), &struct_b.field_0x3e);
    while
    {
        while
        {
            lVar2 = pass1_1008_5b12(CONCAT22(local_SS__1, local_a));
            local_DX_93 = (lVar2 >> 0x10);
            struct_d = lVar2;
            if (lVar2 == 0) {}
              // goto LAB_1030_6af4;
            local_ES_108 = (param_2 >> 0x10);
            struct_f = param_2;
            (struct_d.field_0x6 != struct_f.field_0x6) ||
                 (struct_d.field_0x4 != struct_f.field_0x4)
        } {}
        struct_d.field_0x8 != struct_f.field_0x8
    } {}
    struct_d.field_0xa = struct_d.field_0xa + struct_f.field_0xa;
    struct_d.field_0xc = struct_d.field_0xc + struct_f.field_0xc;
    param_2 = 0;
// LAB_1030_6af4:
    if (param_2 != 0)
    {
        fn_ptr_a = (&struct_b.field_0x3e + 8);
        (**fn_ptr_a)(&PTR_LOOP_1050_1008, &struct_b.field_0x3e, param_2);
    }
    return;
}



pub fn pass1_1030_6b16(struct_a: *mut astruct_922)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let struct_b: *mut astruct_922;
    let mut uVar3: u32;
    let mut local_res6: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f16f4ebe9: u32;
    let temp_86252203f71: *mut u32;
    let fn_ptr_1: fn();

    if (&struct_a.field_0x3a == 0)
    {
        return 0;
    }
    fn_ptr_1 = (&struct_a.field_0x3a + 0x10);
    uVar3 = (**fn_ptr_1)();
    temp_5f16f4ebe9 = &struct_a.field_0x3a;
    if ((temp_5f16f4ebe9 + 8) == 0)
    {
        puVar1 = struct_a.field_0x3a;
        uVar2 = &struct_a.field_0x3c;
        if ((uVar2 | puVar1) != 0)
        {
            fn_ptr_1 = *puVar1;
            (**fn_ptr_1)();
        }
        &struct_a.field_0x3a = 0;
    }
    return uVar3;
}




pub fn pass1_1030_6b86(param_1: u32)

{
    let ppcVar1: fn();
    let mut in_AX: u16;
    let mut uVar2: u32;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let mut extraout_DX_00: i32;
    let local_BX_9: *mut astruct_923;
    let mut uVar4: u16;
    let mut unaff_CS: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_9 = param_1;
    if (local_BX_9.field_0x1e == 0)
    {
        in_AX = 0;
        uVar3 = 0;
    }
    else
    {
        ppcVar1 = (local_BX_9.field_0x1e + 0x10);
        (**ppcVar1)();
        uVar3 = extraout_DX;
    }
    _local_8 = CONCAT22(uVar3, in_AX);
    local_c = 0;
    while (local_c < _local_8)
    {
        ppcVar1 = (local_BX_9.field_0x1e + 4);
        uVar2 = _local_8;
        (**ppcVar1)(unaff_CS, local_BX_9.field_0x1e);
        if ((extraout_DX_00 | uVar2) != 0)
        {
            unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00);
        }
        local_c = local_c + 1;
    }
    return;
}

pub fn pass1_1030_6c1a(param_1: *mut astruct_924, param_2: i32)

{
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let local_BX_7: *mut astruct_924;
    let mut uvar3: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_7 = param_1;
    iVar2 = local_BX_7.field_0x32;
    local_BX_7.field_0x32 = param_2;
    piVar1 = &local_BX_7.field_0x34;
    *piVar1 = *piVar1 + (param_2 - iVar2);
    iVar2 = local_BX_7.field_0x32;
    if (iVar2 < 0)
    {
        iVar2 = 0;
    }
    local_BX_7.field_0x32 = iVar2;
    return;
}

pub fn pass1_1030_6c4c(param_1: u32, param_2: i32)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = (param_1 + 0x32);
    if (param_2 < iVar1)
    {
        iVar1 = param_2;
    }
    (param_1 + 0x34) = iVar1;
    return;
}



pub fn pass1_1030_6c66(param_1: *mut astruct_918, param_2: i32,param_3: u32)

{
    let mut iVar1: i32;
    let ppcVar2: fn();
    let uVar3: u8;
    let mut in_AX: i32;
    let paVar4: *mut astruct_199;
    let BVar5: bool;
    let extraout_var: u32;
    let in_DX: *mut astruct_199;
    let mut uVar6: i32;
    let mut iVar7: i32;
    let local_BX_4: *mut astruct_925;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut unaff_CS: u16;
    let mut local_4: u16;

    uVar9 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x3a == 0)
    {
        unaff_CS = 0x1000;
        process_struct_1000_179c(0xc, in_DX);
        uVar6 = in_DX | in_AX;
        if (uVar6 == 0)
        {
            &local_BX_4.field_0x3a = 0;
        }
        else
        {
            unaff_CS = SUB42(&PTR_LOOP_1050_1008, 0);
            paVar4 = process_struct_1008_574a(CONCAT22(in_DX, in_AX));
            local_BX_4.field_0x3a = paVar4;
            &local_BX_4.field_0x3c = uVar6;
        }
    }
    ppcVar2 = (&local_BX_4.field_0x3a + 8);
    ppcVar2(unaff_CS, &local_BX_4.field_0x3a, param_3);
    if (param_2 != 0)
    {
        uVar10 = (param_3 >> 0x10);
        iVar8 = param_3;
        if ((iVar8 + 6) != 0)
        {
            pass1_1030_6e9c(param_1,  * (iVar8 + 10), (iVar8 + 6));
            return;
        }
        if ((iVar8 + 4) != 0)
        {
            iVar1 = (iVar8 + 10);
            iVar7 = -(iVar1 != 0);
            pass1_1030_7ddc(param_1, CONCAT13((iVar7 >> 8), CONCAT12(iVar7, -iVar1)),
                            (iVar8 + 4));
            return;
        }
        if ((iVar8 + 8) != 0)
        {
            uVar3 = pass1_1030_6fa0(param_1);
            BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, uVar3), 4);
            if (BVar5 != 0)
            {
                pass1_1028_6356(local_BX_4.field_0x1a, 0, (iVar8 + 10), 0);
            }
        }
    }
    return;
}

pub fn pass1_1030_6d4e(param_1: u32)

{
    let mut uVar1: u16;
    let mut local_6: u32;

    local_6 = 0;
    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x36) != 0)
    {
        local_6 = pass1_1010_9092((param_1 + 0x36));
    }
    return local_6;
}

pub fn pass1_1030_6d80(param_1: u32,param_2: u32)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_4: *mut astruct_926;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    puVar1 = local_BX_4.field_0x36;
    uVar2 = &local_BX_4.field_0x38;
    if ((uVar2 | puVar1) != 0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    &local_BX_4.field_0x36 = param_2;
    return;
}



pub fn pass1_1030_6db4()

{
    let ppVar1: *mut pass1_struct_1;
    let mut uVar2: u32;
    let mut in_stack_0000fff0: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    ppVar1 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff0, 0x2f));
    uVar2 = pass1_1010_ed3e(ppVar1);
    return (uVar2 + 0x18);
}



pub fn pass1_1030_6ddc(param_1: *mut astruct_918)

{
    let uVar1: u8;
    let BVar2: bool;
    let extraout_var: u32;

    uVar1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, uVar1), 0x1e);
    if (BVar2 != 0)
    {
        pass1_1030_d0c6((param_1 + 0x1a));
        return;
    }
    return;
}




pub fn pass1_1030_6e14(param_1: *mut astruct_918)

{
    let uVar1: u8;
    let BVar2: bool;
    let extraout_var: u32;
    let mut temp_5fbf6d6e8e: u32;

    uVar1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, uVar1), 0x1e);
    if (BVar2 != 0)
    {
        temp_5fbf6d6e8e = (param_1 + 0x1a);
        pass1_1030_d102(temp_5fbf6d6e8e, (temp_5fbf6d6e8e >> 0x10));
        return;
    }
    return;
}

pub fn pass1_1030_6e4c(param_1: u32)

{
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uVar3: i32;
    let mut local_6: u32;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar3 << 0x10));
    }
    if (((iVar2 + 0x1a) != 0) &&
        (uVar1 = (iVar2 + 0x1a), (uVar1 + 0x12) == 4))
    {
        return;
    }
    return;
}




pub fn pass1_1030_6e9c(param_1: u32, param_2: libc::c_long, param_3: i32)

{
    let ppcVar1: fn();
    let mut uVar2: i32;
    let paVar3: *mut astruct_493;
    let mut uVar4: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut uVar5: i32;
    let local_BX_4: *mut astruct_927;
    let mut uVar6: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar6 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar2 = local_BX_4.field_0x20 | &local_BX_4.field_0x1e;
    if (uVar2 != 0)
    {
        ppcVar1 = (&local_BX_4.field_0x1e + 0x10);
        (**ppcVar1)();
        _local_6 = CONCAT22(extraout_DX, uVar2);
        local_a = 0;
        while (local_a < _local_6)
        {
            ppcVar1 = (&local_BX_4.field_0x1e + 4);
            uVar4 = _local_6;
            (**ppcVar1)();
            uVar2 = uVar4;
            uVar5 = extraout_DX_00 | uVar2;
            if (uVar5 != 0)
            {
                paVar3 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00);
                if (&paVar3.field_0xc == param_3)
                {
                    param_2 = param_2 + -1;
                    pass1_1028_e332(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00);
                    ppcVar1 = (&local_BX_4.field_0x1e + 8);
                    (**ppcVar1)(&PTR_LOOP_1050_1028, &local_BX_4.field_0x1e, 0, local_a);
                }
                if ((param_2._2_2_ | param_2) == 0)
                {
                    return;
                }
            }
            local_a = local_a + 1;
        }
    }
    return;
}



pub fn pass1_1030_6f5a(param_1: *mut astruct_918)

{
    let uVar1: u8;
    let BVar2: bool;
    let extraout_var: u32;
    let mut local_6: u32;

    uVar1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, CONCAT31(extraout_var, uVar1), 4);
    if (BVar2 != 0)
    {
        pass1_1028_6302((param_1 + 0x1a));
    }
    return;
}

pub fn pass1_1030_6fa0(param_1: *mut astruct_918)

{
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uVar3: i32;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar3 << 0x10));
    }
    if ((iVar2 + 0x1a) != 0)
    {
        uVar1 = (iVar2 + 0x1a);
        return  * (uVar1 + 0xc);
    }
    return '\0';
}



pub fn pass1_1030_6fd4(param_1: *mut astruct_493)

{
    let mut uVar1: i32;
    let mut local_8: u32;
    let mut temp_5fccec0553: u32;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar1 << 0x10));
    }
    temp_5fccec0553 = (param_1 + 0x1a);
    if ((temp_5fccec0553 + 0x12) == 5)
    {
        return;
    }
    return;
}

pub fn pass1_1030_701c(param_1: *mut astruct_493)

{
    let mut u_var1: u32;
    let mut uVar2: i32;
    let mut local_8: u32;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar2 << 0x10));
    }
    uVar1 = (param_1 + 0x1a);
    if ((uVar1 + 0x12) == 5)
    {
        return;
    }
    return;
}

pub fn pass1_1030_7064(param_1: u32)

{
    let mut u_var1: u32;
    let mut uVar2: i32;
    let mut local_8: u32;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar2 << 0x10));
    }
    uVar1 = (param_1 + 0x1a);
    if ((uVar1 + 0x12) == 5)
    {
        return;
    }
    return;
}

pub fn pass1_1030_70ac(param_1: u32)

{
    let mut u_var1: u32;
    let mut uVar2: i32;
    let mut local_8: u32;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar2 << 0x10));
    }
    uVar1 = (param_1 + 0x1a);
    if ((uVar1 + 0x12) == 5)
    {
        return;
    }
    return;
}



pub fn pass1_1030_70f4(param_1: *mut astruct_493)

{
    let mut iVar1: i32;
    let BVar2: bool;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let mut uVar5: u32;
    let mut local_c: u32;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar4 << 0x10));
    }
    uVar5 = (iVar3 + 0x1a);
    iVar1 = (uVar5 + 0xc);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar1, 1);
    if (BVar2 == 0)
    {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar1, 2);
        if ((BVar2 == 0) || ((iVar3 + 0x22) == 0))
        {
            return;
        }
        uVar5 = (iVar3 + 0x22);
    }
    else
    {
        uVar5 = (iVar3 + 0x1a);
        uVar5 = (uVar5 + 0x28);
    }
    infinite_loop_1020_ba94(uVar5);
    return;
}



pub fn pass1_1030_7176(param_1: *mut astruct_493)

{
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x22) == 0)
    {
        return;
    }
    if ((iVar2 + 0x1a) == 0)
    {
        pass1_1030_73a8(param_1);
    }
    uVar1 = (iVar2 + 0x1a);
    local_8 = (uVar1 + 0xc);
    local_a = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, local_8, 3);
    if ((local_a != 0) && (uVar1 = (iVar2 + 0x1a), (uVar1 + 0x12) == 5))
    {
        uVar1 = (iVar2 + 0x22);
        local_e = (uVar1 + 4);
        local_12 = 0;
        while (local_12 < local_e)
        {
            pass1_1020_bb16((iVar2 + 0x22), CONCAT22(unaff_SS, &local_1a),
                            CONCAT22(unaff_SS, &local_16), local_12);
            if (0 < local_16)
            {
                local_6 = local_6 + local_1a;
            }
            local_12 = local_12 + 1;
        }
    }
    return;
}



pub fn pass1_1030_7226(param_1: *mut astruct_493)

{
    let mut u_var1: u32;
    let mut uVar2: u32;
    let BVar3: bool;
    let mut iVar4: i32;
    let mut uVar5: i32;
    let mut local_a: u32;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0x1a) == 0)
    {
        pass1_1030_73a8((param_1 & 0xffff | uVar5 << 0x10));
    }
    uVar2 = (iVar4 + 0x1a);
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar2 + 0xc), 0x10);
    if (((BVar3 != 0) && (uVar2 = (iVar4 + 0x1a), (uVar2 + 0x12) == 5)) &&
        (uVar1 = (iVar4 + 0x1a),
         uVar2 = (uVar1 & 0xffff0000 | (uVar1 + 0x14)),
         (uVar2 + 0xa4) == 0x1e))
    {
        return;
    }
    return;
}



pub fn pass1_1030_7296(param_1: *mut astruct_928)

{
    let mut uVar1: i32;
    let mut uVar2: i32;
    let local_BX_4: *mut astruct_928;
    let mut uvar3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar1 = local_BX_4.field_0x22;
    uVar2 = &local_BX_4.field_0x24;
    _local_6 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0)
    {
        pass1_1020_ba7e(CONCAT22(uVar2, uVar1));
        error_check_1000_17ce(_local_6);
    }
    &local_BX_4.field_0x22 = 0;
    return;
}



pub fn pass1_1030_72d0(param_1: *mut astruct_929)

{
    let mut uVar1: i32;
    let mut uVar2: i32;
    let local_BX_4: *mut astruct_929;
    let mut uvar3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar1 = local_BX_4.field_0x26;
    uVar2 = &local_BX_4.field_0x28;
    _local_6 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0)
    {
        pass1_1020_ba7e(CONCAT22(uVar2, uVar1));
        error_check_1000_17ce(_local_6);
    }
    &local_BX_4.field_0x26 = 0;
    return;
}




pub fn pass1_1030_730a(param_1: *mut astruct_930)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let mut in_AX: u16;
    let mut uVar4: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let local_BX_4: *mut astruct_930;
    let mut uVar5: u16;
    let mut unaff_CS: u16;
    let mut uVar6: u32;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x1e != 0)
    {
        uVar6 = &local_BX_4.field_0x1e;
        ppcVar3 = (&local_BX_4.field_0x1e + 0x10);
        (**ppcVar3)();
        _local_6 = CONCAT22(extraout_DX, in_AX);
        local_a = 0;
        while (local_a < _local_6)
        {
            ppcVar3 = (&local_BX_4.field_0x1e + 4);
            uVar4 = _local_6;
            (**ppcVar3)(unaff_CS);
            if ((extraout_DX_00 | uVar4) != 0)
            {
                unaff_CS = SUB42(&PTR_LOOP_1050_1028, 0);
                pass1_1028_e332(_PTR_LOOP_1050_65e2, uVar4, extraout_DX_00);
            }
            local_a = local_a + 1;
        }
        puVar1 = &local_BX_4.field_0x1e;
        uVar2 = local_BX_4.field_0x20;
        if ((uVar2 | puVar1) != 0)
        {
            ppcVar3 = *puVar1;
            (**ppcVar3)(unaff_CS, puVar1, uVar2, 1, uVar6);
        }
        &local_BX_4.field_0x1e = 0;
    }
    return;
}



pub fn pass1_1030_73a8(param_1: *mut astruct_493)

{
    let mut u_var1: u32;
    let paVar2: *mut astruct_493;
    let mut in_DX: u16;
    let local_BX_3: *mut astruct_493;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0x16 == 0)
    {
        return 0;
    }
    if (&local_BX_3.field_0x1a == 0)
    {
        uVar1 = local_BX_3.field_0x16;
        paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        local_BX_3.field_0x1a = paVar2;
        local_BX_3.field_0x1c = in_DX;
    }
    return CONCAT22(local_BX_3.field_0x1c, local_BX_3.field_0x1a);
}



pub fn pass1_1030_73ee(param_1: *mut astruct_931,param_2: u32)

{
    let paVar1: *mut astruct_493;
    let mut in_DX: u16;
    let local_BX_7: *mut astruct_931;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_7 = param_1;
    local_BX_7.field_0x2a = param_2;
    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2, (param_2 >> 0x10));
    local_BX_7.field_0x2e = paVar1;
    local_BX_7.field_0x30 = in_DX;
    return;
}


pub fn pass1_1030_5fe2(param_1: *mut u32,param_2: u32)

{
    (param_1 + 0x10) = param_2;
    return;
}



pub fn pass1_1030_5c8a(param_1: u32,param_2: u32)

{
    let plVar1: *mut long;
    let mut uVar2: i32;
    let local_AX_72: *mut astruct_910;
    let mut uVar4: i32;
    let local_BX_38: *mut astruct_908;
    let local_SI_43: *mut astruct_909;
    let mut uVar5: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut uVar3: u32;

    local_6 = 0;
    uVar2 = param_2._3_1_;
    if (uVar2 == 0xff)
    {
        return;
    }
    uVar5 = (_PTR_LOOP_1050_65e2 >> 0x10);
    local_BX_38 = (_PTR_LOOP_1050_65e2 + 10);
    local_SI_43 = (uVar2 * 4);
    uVar3 = (local_BX_38 + local_SI_43);
    uVar4 = (local_BX_38 + local_SI_43 + 2);
    if ((uVar3 + 4) != 0)
    {
        pass1_1030_12ca(uVar3 & 0xffff | uVar4 << 0x10);
        local_6 = uVar3 & 0xffff | uVar4 << 0x10;
    }
    if (local_6 == 0)
    {
        plVar1 = (uVar2 * 4 + param_1);
        *plVar1 = *plVar1 + 1;
    }
    return;
}

pub fn pass1_1030_5d0a(param_1: *mut astruct_393) -> *mut astruct_393

{
    let mut uVar1: u16;

    pass1_1030_17ce(param_1, 1, 4);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x10) = 0;
    param_1.field_0x0 = 0x613e;
    (param_1 + 2) = 0x1030;
    return param_1;
}

// l

pub fn pass1_1030_5d3c(struct_a: *mut astruct_848,param_2: u32) -> *mut astruct_848

{
    let struct_b_hi: *mut astruct_848;

    pass1_1030_183c(struct_a, 1, 4, 0x1000000, param_2);
    struct_b_hi = (struct_a >> 0x10);
    (struct_a + 0x10) = 0;
    struct_a.field_0x0 = 0x613e;
    (struct_a + 2) = 0x1030;
    return struct_a;
}

pub fn pass1_1030_5d78(struct_a: *mut astruct_850)

{
    let mut uVar1: i32;
    let mut uVar2: i32;
    let struct_c: *mut astruct_850;
    let mut uvar3: u16;
    let mut local_6: u32;
    let struct_b: *mut astruct_44;

    uVar3 = (struct_a >> 0x10);
    struct_c = struct_a;
    struct_a.field_0x0 = 0x613e;
    struct_c.field_0x2 = 0x1030;
    struct_b = struct_c.field_0x10;
    uVar1 = (&struct_c.field_0x10 + 2);
    uVar2 = struct_b;
    if ((uVar1 | uVar2) != 0)
    {
        pass1_1030_8480(uVar2, uVar1);
        error_check_1000_17ce(struct_b);
    }
    pass1_1030_18b2(struct_a);
    return;
}



pub fn pass1_1030_5a52(param_1: u32, param_2: *mut u32, param_3: *mut u32)

{
    let mut u_var1: u32;
    let mut local_ES_3: u16;
    let mut temp_5fe005333b: u32;

    local_ES_3 = (param_1 >> 0x10);
    temp_5fe005333b = (param_1 + 0x10);
    *param_3 = (temp_5fe005333b + 0xe);
    uVar1 = (param_1 + 0x10);
    *param_2 = (uVar1 + 0x12);
    return;
}

pub fn pass1_1030_5a80(param_1: u32,param_2: u32)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut uVar3: u32;
    let mut local_20: [u8;12];
    let mut local_14: [u8;6];
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    (param_1 + 0x10) = param_2;
    uVar3 = process_struct_1008_4772((param_2 + 0xe));
    local_4 = (uVar3 >> 0x10);
    local_6 = uVar3;
    local_a = (local_6 + 4);
    local_e = (local_6 + 8);
    pass1_1008_3e54(CONCAT22(unaff_SS, local_14), 0, local_e - 1, local_a - 1);
    uVar1 = param_1 + 0x14;
    modify_list_1008_6cb4(CONCAT22(unaff_SS, local_20), local_14, unaff_SS, uVar1, uVar2);
    modify_list_1008_6d64(local_20, unaff_SS, uVar1, uVar2);
    return;
}

pub fn pass1_1030_5b00(param_1: u32) -> i32

{
    let mut local_4: u16;

    return (param_1 + 4) + 0xb;
}

pub fn pass1_1030_5b1c(param_1: u32, param_2: *mut u16, param_3: *mut u16)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x1a);
    *param_2 = (param_1 + 0x1c);
    return;
}

pub fn pass1_1030_5b3e(param_1: u32, param_2: i32, param_3: u16)

{
    let mut iVar1: i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x1a) = param_3;
    if ((iVar1 + 0x1c) < param_2)
    {
        (iVar1 + 0x1c) = param_2;
    }
    return;
}

pub fn pass1_1030_5b5c(param_1: i32, param_2: u16)

{
    return CONCAT22(param_2, param_1 + 0x14);
}

pub fn pass1_1030_5b6c(param_1: *mut astruct_519,param_2: u32)

{
    let mut u_var1: u32;
    let uVar2: u8;
    let extraout_var: u32;
    let mut in_DX: u16;
    let local_BX_4: *mut astruct_519;
    let local_BX_46: *mut astruct_520;
    let mut uVar4: u16;
    let mut uvar3: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x10 != 0)
    {
        uVar1 = local_BX_4.field_0x10;
        uVar2 = error_check_1000_17ce((uVar1 + 4));
        uVar3 = CONCAT31(extraout_var, uVar2);
        pass1_fn_1008_60e8(param_2);
        uVar1 = local_BX_4.field_0x10;
        uVar4 = (uVar1 >> 0x10);
        local_BX_46 = uVar1;
        &local_BX_46.field_0x4 = uVar3;
        local_BX_46.field_0x6 = in_DX;
    }
    return;
}

pub fn pass1_1030_5baa(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_56b0(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1030_5bec(param_1: u32)

{
    _PTR_LOOP_1050_5736 = param_1;
    pass1_fn_1000_54a0(param_1, 0, 0x24);
    return;
}



pub fn pass1_1030_5c0e()

{
    _PTR_LOOP_1050_5736 = 0;
    return;
}



pub fn pass1_1030_538a(struct_a: *mut astruct_899)

{
    let mut uVar1: i32;
    let struct_b: *mut astruct_899;
    let struct_c: *mut astruct_899;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000fff0: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    struct_c = (struct_a >> 0x10);
    struct_b = struct_a;
    uVar1 = &struct_b.field_0x10a >> 8;
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff0, 0x2f));
    if (uVar1 == 1)
    {
        pass1_1018_04ca(ppVar2, &struct_b.field_0x108);
    }
    else
    {
        if (uVar1 == 2)
        {
            pass1_1018_04a4(ppVar2, &struct_b.field_0x108);
        }
    }
    return 1;
}




pub fn pass1_1030_53f4(struct_a: *mut astruct_900)

{
    let mut u_var1: u32;
    let local_AX_190: *mut astruct_901;
    let paVar2: *mut astruct_493;
    let mut in_DX: u16;
    let struct_b: *mut astruct_900;
    let struct_b_hi: *mut astruct_900;
    let mut unaff_SS: u16;
    let mut local_126: u32;
    let mut local_11e: u16;
    let mut local_11c: u16;
    let mut local_10: u32;
    let mut local_c: u32;

    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    local_c = struct_b.field_0x108;
    local_c._3_1_ = (local_c >> 0x18);
    if (local_c._3_1_ == -1)
    {
        pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
    }
    else
    {
        local_10 = struct_b.field_0x108;
        local_10._3_1_ = (local_10 >> 0x18);
        if (local_10._3_1_ == '\x03')
        {
            pass1_1028_e44a(_PTR_LOOP_1050_65e2, struct_b.field_0x108);
        }
        else
        {
            uVar1 = struct_b.field_0x108;
            pass1_1028_e372(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        }
    }
    local_c = struct_b.field_0x108;
    local_c._3_1_ = (local_c >> 0x18);
    if (local_c._3_1_ != '\x03')
    {
        pass1_1030_521c(CONCAT22(unaff_SS, &local_11e), struct_b.field_0x108);
        local_10 = *_g_bool_1050_5748;
        pass1_1028_d566(local_10, CONCAT22(unaff_SS, &local_11e));
        local_126._3_1_ = (struct_b.field_0x108 >> 0x18);
        if (local_126._3_1_ == 0x2)
        {
            uVar1 = struct_b.field_0x108;
            paVar2 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
            pass1_1010_82f8(_g_struct_73_1050_14cc, *&paVar2.field_0x10);
        }
    }
    return;
}



pub fn pass1_1030_54f8(param_1: u32)

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut astruct_902;
    let local_CX_80: *mut astruct_904;
    let in_DX: *mut astruct_199;
    let local_BX_41: *mut astruct_903;
    let puVar3: *mut u32;
    let puVar4: *mut u32;
    let mut uVar5: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x10c, in_DX);
    _local_a = CONCAT22(in_DX, local_AX__1);
    if ((in_DX | local_AX__1) != 0)
    {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar5 = (param_1 >> 0x10);
        local_BX_41 = param_1;
        local_AX__1.field_0x4 = local_BX_41.field_0x4;
        puVar3 = &local_BX_41.field_0x8;
        puVar4 = &local_AX__1.field_0x8;
        local_CX_80 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_80 != 0x0)
        {
            local_CX_80 = local_CX_80 + -1;
            puVar2 = puVar4;
            puVar4 = puVar4 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_41.field_0x108;
        *_local_a = 0x55ee;
        local_AX__1.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_5596(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_55c2(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_560e(struct_a: *mut astruct_393) -> *mut astruct_393

{
    let struct_b: *mut astruct_393;
    let struct_c: *mut astruct_393;

    pass1_1030_17ce(struct_a, 100, 500);
    struct_c = (struct_a >> 0x10);
    struct_b = struct_a;
    struct_b.field_0x10 = 0;
    zero_list_1008_3e38((struct_a & 0xffff0000 | ZEXT24(&struct_b.field_0x14)));
    struct_b.field_0x1a = 0;
    struct_b.field_0x1c = 0;
    struct_a.field_0x0 = s_procLo_1050_5bd0;
    struct_b.u16_0x2 = 0x1030;
    return struct_a;
}

pub fn pass1_1030_565a(struct_a: *mut astruct_393,param_2: u32) -> *mut astruct_393

{
    let struct_b: *mut astruct_906;
    let struct_b_hi: *mut astruct_393;

    pass1_1030_183c(struct_a, 100, 500, 0x3000000, param_2);
    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    struct_b.field_0x10 = 0;
    zero_list_1008_3e38((struct_a & 0xffff0000 | &struct_b.field_0x14));
    struct_b.field_0x1a = 0;
    struct_b.field_0x1c = 0;
    struct_a.field_0x0 = s_procLo_1050_5bd0;
    struct_b.field_0x2 = 0x1030;
    return struct_a;
}

pub fn pass1_1030_56b0(struct_a: *mut astruct_850)

{
    let struct_d: *mut astruct_44;
    let struct_b: *mut astruct_850;
    let struct_b_hi: *mut astruct_850;
    let mut local_6: u32;
    let struct_e_hi: *mut astruct_44;
    let struct_e: *mut astruct_44;

    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    struct_a.field_0x0 = s_procLo_1050_5bd0;
    struct_b.field_0x2 = 0x1030;
    struct_e = struct_b.field_0x10;
    struct_e_hi = (&struct_b.field_0x10 + 2);
    struct_d = struct_e;
    if ((struct_e_hi | struct_d) != 0)
    {
        pass1_1030_84d0((struct_e & 0xffff | ZEXT24(struct_e_hi) << 0x10));
        error_check_1000_17ce(struct_e);
    }
    pass1_1030_18b2(struct_a);
    return;
}


pub fn pass1_1030_5260(param_1: u32)

{
    let paVar1: *mut astruct_493;
    let mut in_DX: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5fc9f6a425: u32;
    let fn_ptr_a: fn();

    temp_5fc9f6a425 = (param_1 + 0x108);
    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, temp_5fc9f6a425,
                             (temp_5fc9f6a425 >> 0x10));
    _local_6 = CONCAT22(in_DX, paVar1);
    fn_ptr_a = (*_local_6 + 0x14);
    (**fn_ptr_a)();
    return 1;
}



pub fn pass1_1030_5290(struct_a: *mut astruct_895)

{
    let paVar1: *mut astruct_898;
    let paVar2: *mut astruct_897;
    let struct_e: *mut astruct_896;
    let mut iVar3: i32;
    let in_DX: *mut astruct_199;
    let struct_b: *mut astruct_895;
    let struct_c: *mut astruct_898;
    let struct_d: *mut astruct_897;
    let mut local_ES_41: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x10c, in_DX);
    _local_a = CONCAT22(in_DX, struct_e);
    if ((in_DX | struct_e) != 0)
    {
        *_local_a = s_1_1050_389a;
        struct_e.field_0x2 = &PTR_LOOP_1050_1008;
        local_ES_41 = (struct_a >> 0x10);
        struct_b = struct_a;
        struct_e.field_0x4 = struct_b.field_0x4;
        struct_c = &struct_b.field_0x8;
        struct_d = &struct_e.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0)
        {
            iVar3 = iVar3 + -1;
            paVar2 = struct_d;
            struct_d = &struct_d.field_0x4;
            paVar1 = struct_c;
            struct_c = &struct_c.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        struct_e.field_0x2 = &PTR_LOOP_1050_1028;
        struct_e.field_0x108 = struct_b.field_0x108;
        *_local_a = 0x55fe;
        struct_e.field_0x2 = 0x1030;
    }
    return;
}



pub fn pass1_1030_4f5a(param_1: u32)

{
    let mut u_var1: u32;
    let puVar2: *mut u32;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let extraout_DX: *mut u16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let local_32: *mut astruct_892;
    let mut uVar6: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    puVar2 = &local_a;
    read_from_file_1030_4e70(param_1, puVar2, unaff_SS, &local_6, unaff_SS, s_bldgbld_dat_1050_56fc,
                             &g_alloc_addr_1050_1050);
    uVar1 = local_6;
    if (puVar2 != 0x0)
    {
        local_32 = param_1;
        uVar6 = (param_1 >> 0x10);
        uVar4 = local_6;
        g_u16_ptr_1050_5f2e = extraout_DX;
        pass1_1030_4e34(local_32, uVar6, local_a, local_6);
        if (__g_astruct_94_ptr_1 == 0)
        {
            struct_fn_1000_160a();
        }
        else
        {
        }
        iVar3 = uVar4 * 0x98;
        alloc_mem_1000_1708();
        local_32.field_0x12 = iVar3;
        local_32.field_0x14 = g_u16_ptr_1050_5f2e;
        pass1_1030_4dbc(param_1, local_6, uVar4 & 0xffff);
        _local_14 = CONCAT22(g_u16_ptr_1050_5f2e, iVar3);
        local_16 = 0;
        while (local_16 < uVar4)
        {
            uVar5 = local_32.field_0x14;
            iVar3 = local_32.field_0x12 + local_16 * 0x98;
            pass1_1030_4d3a(local_32, uVar6, CONCAT22(uVar5, iVar3), _local_14);
            pass1_1030_4dbc(param_1, 0, 0);
            _local_14 = CONCAT22(uVar5, iVar3);
            local_16 = local_16 + 1;
        }
        local_e._2_2_ = (uVar1 >> 0x10);
        local_e._0_2_ = uVar1;
        if ((local_e._2_2_ | local_e) != 0)
        {
            error_check_1000_0dc6(local_e, local_e._2_2_);
        }
    }
    return;
}




pub fn pass1_1030_5044(param_1: u32)

{
    let mut u_var1: u32;
    let mut uVar2: u32;
    let puVar3: *mut u32;
    let mut in_i16_3: i32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let extraout_DX: *mut u16;
    let mut uVar7: i32;
    let mut uVar8: u16;
    let mut unaff_SS: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    puVar3 = &local_a;
    read_from_file_1030_4e70(param_1, puVar3, unaff_SS, &local_6, unaff_SS, s_bldgops_dat_1050_5708,
                             &g_alloc_addr_1050_1050);
    uVar2 = local_6;
    if (puVar3 != 0x0)
    {
        uVar9 = param_1;
        uVar10 = (param_1 >> 0x10);
        uVar5 = local_6;
        g_u16_ptr_1050_5f2e = extraout_DX;
        pass1_1030_4e34(uVar9, uVar10, local_a, local_6);
        in_i16_3 = uVar5;
        if (__g_astruct_94_ptr_1 == 0)
        {
            struct_fn_1000_160a();
        }
        else
        {
        }
        uVar6 = (in_i16_3 * 0xae);
        alloc_mem_1000_1708();
        uVar1 = uVar6 & 0xffff | ZEXT24(g_u16_ptr_1050_5f2e) << 0x10;
        uVar7 = g_u16_ptr_1050_5f2e | uVar6;
        if (uVar7 == 0)
        {
            (uVar9 + 0x15c) = 0;
        }
        else
        {
            call_fn_ptr_1000_5586(0x51f0, 0x1030, in_i16_3, 0xae, uVar6, g_u16_ptr_1050_5f2e);
            (uVar9 + 0x15c) = uVar1;
            uVar6 = uVar1;
        }
        uVar8 = uVar6;
        pass1_1030_4dbc(param_1, local_6, uVar5 & 0xffff);
        _local_16 = CONCAT22(uVar7, uVar8);
        local_18 = 0;
        while (local_18 < in_i16_3)
        {
            uVar8 = (uVar9 + 0x15e);
            iVar4 = (uVar9 + 0x15c) + local_18 * 0xae;
            pass1_1030_4c52(uVar9, uVar10, CONCAT22(uVar8, iVar4), _local_16);
            pass1_1030_4dbc(param_1, 0, 0);
            _local_16 = CONCAT22(uVar8, iVar4);
            local_18 = local_18 + 1;
        }
        local_e._2_2_ = (uVar2 >> 0x10);
        local_e._0_2_ = uVar2;
        if ((local_e._2_2_ | local_e) != 0)
        {
            error_check_1000_0dc6(local_e, local_e._2_2_);
        }
    }
    return;
}



pub fn pass1_1030_4e34(param_1: u16, param_2: u16, param_1_00: libc::c_long, param_2_00: *mut libc::c_char)

{
    let mut local_4: u16;

    while (param_1_00 != 0)
    {
        if ((*param_2_00 == '\r') || (*param_2_00 == '\n'))
        {
            *param_2_00 = '\0';
        }
        param_2_00 = (param_2_00 & 0xffff0000 | (param_2_00 + 1));
        param_1_00 = param_1_00 + -1;
    }
    return;
}


pub fn pass1_1030_4628(param_1: u16, param_2: u16, param_1_00: i32)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let local_AX_8: *mut astruct_886;
    let paVar3: *mut astruct_886;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let in_DX: *mut astruct_199;
    let local_BX_64: *mut astruct_884;
    let local_SI_76: *mut astruct_885;
    let piVar6: *mut i32;
    let mut uVar7: u16;
    let mut local_18: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    local_AX_8 = (param_1_00 + -1);
    paVar3 = local_AX_8;
    process_struct_1000_179c(0x28, in_DX);
    _local_14 = CONCAT22(in_DX, paVar3);
    if ((in_DX | paVar3) == 0)
    {
        local_8 = 0;
    }
    else
    {
        zero_list_1008_3e38(CONCAT22(in_DX, paVar3 + 1));
        local_8 = _local_14;
    }
    uVar7 = (local_8 >> 0x10);
    local_BX_64 = local_8;
    local_BX_64.field_0x2 = 0;
    local_SI_76 = (local_AX_8 * 0x5e);
    pass1_1008_3e76((local_8 & 0xff000000 |
                            CONCAT12((local_8 >> 0x10), &local_BX_64.field_0x6)),
                    (local_SI_76 + 0x5336), (local_SI_76 + 0x5334),
                    (local_SI_76 + 0x5332));
    local_BX_64.field_0xc = (local_SI_76 + 0x5348);
    local_8 = param_1_00;
    local_BX_64.field_0xe = (local_SI_76 + 0x534a);
    local_a = 0;
    while
    {
        iVar4 = ((local_AX_8 * 0x2f + local_a) * 2 + 0x5338);
        (&local_BX_64.field_0x12 + local_a * 2) = iVar4;
        local_a = local_a + 1;
        local_a < 8
    } {}
    pass1_fn_1008_612e((local_AX_8 * 0x5e + 0x5350));
    local_BX_64.field_0x22 = iVar4;
    piVar6 = (local_AX_8 * 0x5e + 0x5354);
    local_BX_64.field_0x24 = piVar6;
    &local_BX_64.field_0x26 = &g_alloc_addr_1050_1050;
    iVar1 = *piVar6;
    pass1_1000_4906(CONCAT22(0x1050, piVar6), 0, 0x1e);
    local_a = 0;
// LAB_1030_474c:
    if (iVar4 <= local_a)
    {
        return;
    }
    loop
    {
        iVar5 = (local_AX_8 * 0x5e + 0x534e) + iVar1 + -1;
        pass1_fn_1008_612e(iVar1, iVar5);
        local_18 = 0;
        loop
        {
            if (local_a < local_18)
            {
                uVar2 = &local_BX_64.field_0x24;
                (uVar2 + local_a * 2) = iVar5;
                local_a = local_a + 1;
              // goto LAB_1030_474c;
            }
            uVar2 = &local_BX_64.field_0x24;
            if ((uVar2 + local_18 * 2) == iVar5) {
                break;
            }
            local_18 = local_18 + 1;
        }
    } 
}

pub fn pass1_1030_4782(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: i32, param_5: u16) -> i32

{
    let mut iVar1: i32;
    let in_AX: *mut astruct_94;
    let pcVar2: *mut libc::c_char;
    let mut uvar3: u16;
    let in_DX: *mut u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let struct_a: *mut astruct_199;
    let mut iVar4: i32;
    let mut unaff_SI: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let unaff_SS: *mut libc::c_char;
    let ppVar7: *mut pass1_struct_1;
    let mut uVar8: u32;
    let mut local_dc: u16;
    let mut local_da: u16;
    let mut local_d4: u16;
    let mut local_d2: u16;
    let mut local_c8: u16;
    let mut local_c6: u16;
    let mut local_c4: u16;
    let mut local_c2: u16;
    let mut local_c0: u16;
    let mut local_be: u16;
    let mut local_bc: u16;
    let mut local_b8: u16;
    let mut local_b6: u16;
    let mut local_b4: u16;
    let mut local_b2: u16;
    let mut local_b0: u16;
    let mut local_ae: u16;
    let mut local_ac: u16;
    let mut local_aa: u16;
    let mut local_a8: u16;
    let mut local_a6: u16;
    let mut local_a4: u16;
    let mut local_a2: u16;
    let mut local_a0: u16;
    let mut local_9e: u16;
    let mut local_9c: u16;
    let mut local_9a: u16;
    let mut local_98: u16;
    let mut local_96: u16;
    let mut local_94: u16;
    let mut local_92: [u8;128];
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (__g_astruct_94_ptr_1 == 0)
    {
        _g_astruct_94_ptr_1 = in_AX;
        struct_fn_1000_160a();
        g_u16_ptr_1050_5f2e = in_DX;
    }
    else
    {
    }
    local_c4 = _g_astruct_94_ptr_1;
    local_c2 = g_u16_ptr_1050_5f2e;
    alloc_mem_1000_1708(0x20, 0, 1, _g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
    _local_b8 = CONCAT22(g_u16_ptr_1050_5f2e, _g_astruct_94_ptr_1);
    if ((g_u16_ptr_1050_5f2e | _g_astruct_94_ptr_1) == 0)
    {
        _g_astruct_94_ptr_1 = 0x0;
        uVar5 = 0;
    }
    else
    {
        pass1_1030_84ae(_g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
        uVar5 = extraout_DX;
    }
    _local_6 = CONCAT22(uVar5, _g_astruct_94_ptr_1);
    *_local_6 = param_5;
    modify_list_1008_3f62(CONCAT22(uVar5, &_g_astruct_94_ptr_1.field_0x8),
                          CONCAT22(0x1050, param_5 * 10 + 0x65e6));
    if (param_2_00 != 0)
    {
        ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x2f));
        local_8 = (ppVar7 >> 0x10);
        local_a = ppVar7;
        _local_e = pass1_1018_04b8(ppVar7);
        uVar3 = (_local_e >> 0x10);
        local_96 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, _local_e, uVar3);
        _local_12 = CONCAT22(uVar3, local_96);
        load_string_1010_847e(_g_struct_73_1050_14cc, (_g_struct_73_1050_14cc >> 0x10), 0x62d);
        uVar3 = extraout_DX_00;
        local_94 = extraout_DX_00;
        pcVar2 = pass1_1030_2a98(_local_12);
        *(_local_6 + 2) = pcVar2;
        string_fn_1000_3f9c(local_92, unaff_SS, local_96, local_94, pcVar2);
        pcVar2 = local_92;
        pass1_fn_1008_60e8(pcVar2);
        uVar5 = (_local_6 >> 0x10);
        *(_local_6 + 4) = pcVar2;
        (_local_6 + 6) = uVar3;
        iVar4 = param_5 * 10;
        mixed_fn_1010_830a(_g_struct_73_1050_14cc, (iVar4 + 0x65ec));
        uVar5 = (_local_6 >> 0x10);
        (_local_6 + 0xe) = param_5;
        (_local_6 + 0x10) = extraout_DX_01;
        mixed_fn_1010_830a(_g_struct_73_1050_14cc, (iVar4 + 0x65ee));
        uVar5 = (_local_6 >> 0x10);
        iVar4 = _local_6;
        (iVar4 + 0x12) = param_5;
        (iVar4 + 0x14) = extraout_DX_02;
        uVar8 = process_struct_1008_4772((iVar4 + 0xe));
        local_98 = (uVar8 >> 0x10);
        local_9a = uVar8;
        local_9c = (local_9a + 4) - 1;
        local_9e = (local_9a + 8) - 1;
        if (param_1_00 != 0)
        {
            local_a0 = 0xf;
            if (_local_e == 0)
            {
                fn_1008_6048(s_get_site_data_without_planet__1050_56de, local_98, SUB21(local_9e, 0));
            }
            else
            {
                uVar3 = pass1_1030_2f1a(_local_12, CONCAT22(unaff_SS, &local_c0),
                                        CONCAT22(unaff_SS, &local_c4));
                pass1_fn_1008_612e(local_c4, local_c0);
                local_a0 = uVar3;
            }
            uVar5 = (_local_6 >> 0x10);
            (_local_6 + 0x1c) = local_a0 * 10;
            (_local_6 + 0x1c) = (local_a0 * 10) / 100;
            ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
            local_c2 = (ppVar7 >> 0x10);
            local_c4 = ppVar7;
            local_c0 = u16_1050_13ae;
            uVar3 = 0x84;
            ppVar7 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x840009);
            local_be = ppVar7;
            pass1_1010_65d0(ppVar7, uVar3);
            local_bc = 0x3c;
            if (local_c0 < 3)
            {
                if (0 < local_be)
                {
                    local_bc = 0x5a;
                }
            }
            else
            {
                if (local_be == 1)
                {
                    local_bc = 0x44;
                }
                else
                {
                    if (local_be == 2)
                    {
                        local_bc = 0x4b;
                    }
                    else
                    {
                        if (local_be == 3)
                        {
                            local_bc = 0x53;
                        }
                        else
                        {
                            if (local_be == 4)
                            {
                                local_bc = 0x5a;
                            }
                        }
                    }
                }
            }
            iVar4 = local_bc * local_a0;
            local_a0 = iVar4 / 100;
            struct_a = (iVar4 % 100);
            uVar5 = (_local_6 >> 0x10);
            (_local_6 + 0x1a) = local_a0;
            local_a4 = local_a0 + (_local_6 + 0x1c);
            uVar3 = local_a4 * 6;
            local_a2 = local_a4;
            process_struct_1000_179c(uVar3, struct_a);
            _local_b8 = CONCAT22(struct_a, uVar3);
            g_u16_ptr_1050_5f2e = (struct_a | uVar3);
            if (g_u16_ptr_1050_5f2e == 0x0)
            {
                (_local_6 + 0x16) = 0;
            }
            else
            {
                call_fn_ptr_1000_5586(0x3e38, &PTR_LOOP_1050_1008, local_a4, 6, uVar3, struct_a);
                (_local_6 + 0x16) = _local_b8;
            }
            local_aa = local_a2 * 2;
            if (__g_astruct_94_ptr_1 == 0)
            {
                _g_astruct_94_ptr_1 = local_aa;
                struct_fn_1000_160a();
            }
            else
            {
            }
            alloc_mem_1000_1708(local_aa, 0, 1, _g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
            _local_ae = CONCAT22(g_u16_ptr_1050_5f2e, _g_astruct_94_ptr_1);
            if (__g_astruct_94_ptr_1 == 0)
            {
                struct_fn_1000_160a();
            }
            else
            {
            }
            alloc_mem_1000_1708(local_aa, 0, 1, _g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
            _local_b2 = CONCAT22(g_u16_ptr_1050_5f2e, _g_astruct_94_ptr_1);
            local_b4 = 0;
// LAB_1030_4b57:
            uVar3 = local_a2;
            if (local_b4 < local_a2)
            {
                loop
                {
                    pass1_fn_1008_612e(0, local_9c);
                    local_a6 = uVar3;
                    pass1_fn_1008_612e(0, local_9e);
                    local_dc = 0;
                    loop
                    {
                        iVar4 = _local_ae;
                        uVar5 = (_local_ae >> 0x10);
                        uVar6 = (_local_b2 >> 0x10);
                        local_a8 = uVar3;
                        if (local_b4 <= local_dc)
                        {
                            iVar1 = local_b4 * 2;
                            (iVar1 + iVar4) = local_a6;
                            (iVar1 + _local_b2) = uVar3;
                            uVar8 = (_local_6 + 0x16);
                            pass1_1008_3e76((uVar8 & 0xff000000 |
                                                    CONCAT12((uVar8 >> 0x10), uVar8 + local_b4 * 6)),
                                            0, uVar3, (iVar1 + iVar4));
                            local_b4 = local_b4 + 1;
                          // goto LAB_1030_4b57;
                        }
                        if (((local_dc * 2 + iVar4) == local_a6) &&
                            ((local_dc * 2 + _local_b2) == uVar3)) {
                            break;
                            }
                        local_dc = local_dc + 1;
                    }
                } 
            }
            error_check_1000_17ce(_local_ae);
            _local_b8 = _local_b2;
            error_check_1000_17ce(_local_b2);
        }
    }
    return;
}

pub fn pass1_1030_4bbe(param_1: u32, param_2: i32)

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut uvar3: u16;
    let mut iVar4: i32;
    let local_BX_5: *mut astruct_887;
    let puVar5: *mut u32;
    let puVar6: *mut u32;
    let mut uVar7: i32;

    uVar7 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (&local_BX_5.field_0x12 == 0)
    {
        pass1_1030_4f5a(param_1 & 0xffff | uVar7 << 0x10);
    }
    puVar6 = &local_BX_5.field_0x16;
    uVar3 = local_BX_5.field_0x14;
    puVar5 = (local_BX_5.field_0x12 + param_2 * 0x98);
    iVar4 = 0x26;
    while (iVar4 != 0)
    {
        iVar4 = iVar4 + -1;
        puVar2 = puVar6;
        puVar6 = puVar6 + 1;
        puVar1 = puVar5;
        puVar5 = puVar5 + 1;
        *puVar2 = *puVar1;
    }
    return;
}

pub fn pass1_1030_4c06(param_1: u32, param_2: i32)

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut uvar3: u16;
    let mut iVar4: i32;
    let local_BX_5: *mut astruct_888;
    let puVar5: *mut u32;
    let puVar6: *mut u32;
    let mut uVar7: i32;

    uVar7 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (&local_BX_5.field_0x15c == 0)
    {
        pass1_1030_5044(param_1 & 0xffff | uVar7 << 0x10);
    }
    puVar6 = &local_BX_5.field_0xae;
    uVar3 = local_BX_5.field_0x15e;
    puVar5 = (local_BX_5.field_0x15c + param_2 * 0xae);
    iVar4 = 0x2b;
    while (iVar4 != 0)
    {
        iVar4 = iVar4 + -1;
        puVar2 = puVar6;
        puVar6 = puVar6 + 1;
        puVar1 = puVar5;
        puVar5 = puVar5 + 1;
        *puVar2 = *puVar1;
    }
    puVar6 = puVar5;
    return;
}



pub fn pass1_1030_4c52(param_1: u16, param_2: u16,param_1_00: u32,param_2_00: u32)

{
    let mut uVar1: i32;
    let mut uVar2: u16;
    let mut in_DX: i32;
    let local_SI_204: *mut astruct_890;
    let mut uvar3: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: *mut astruct_889;

    local_4 = 0x0;
    loop
    {
        uVar1 = pass1_fn_1000_47a4(param_2_00, &PTR_s_n_Op_Op_1050_002c_1050_518a);
        _local_8 = CONCAT22(in_DX, uVar1);
        if ((in_DX | uVar1) == 0) {
            break;
        }
        if (*_local_8 != '\"')
        {
            uVar2 = pass1_fn_1000_3e2c(CONCAT22(in_DX, uVar1));
            local_SI_204 = param_1_00;
            uVar3 = (param_1_00 >> 0x10);
            if (local_4 < 0x25)
            {
                (&local_SI_204.field_0x0 + local_4 * 4) = uVar2;
                (&local_SI_204.field_0x2 + local_4 * 4) = in_DX;
            }
            else
            {
                if (local_4 == (s_New_failed_in_Op__Op_1050_0020 + 5))
                {
                    local_SI_204.field_0x94 = uVar2;
                }
                else
                {
                    if (local_4 == (s_New_failed_in_Op__Op_1050_0020 + 6))
                    {
                        local_SI_204.field_0x96 = uVar2;
                    }
                    else
                    {
                        if (local_4 == (s_New_failed_in_Op__Op_1050_0020 + 7))
                        {
                            local_SI_204.field_0x98 = uVar2;
                        }
                        else
                        {
                            if (local_4 == (s_New_failed_in_Op__Op_1050_0020 + 8))
                            {
                                local_SI_204.field_0x9a = uVar2;
                            }
                            else
                            {
                                if (local_4 == (s_New_failed_in_Op__Op_1050_0020 + 9))
                                {
                                    local_SI_204.field_0x9c = uVar2;
                                }
                                else
                                {
                                    if (local_4 == (s_New_failed_in_Op__Op_1050_0020 + 10))
                                    {
                                        local_SI_204.field_0x9e = uVar2;
                                    }
                                    else
                                    {
                                        if (local_4 == (s_New_failed_in_Op__Op_1050_0020 + 0xb))
                                        {
                                            local_SI_204.field_0xa0 = uVar2;
                                        }
                                        else
                                        {
                                            if (local_4 == (s_New_failed_in_Op__Op_1050_0020 + 0xc))
                                            {
                                                local_SI_204.field_0xa2 = uVar2;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            local_4 = &local_4.field_0x1;
        }
        param_2_00 = 0;
    }
    return;
}



pub fn pass1_1030_4d3a(param_1: u16, param_2: u16,param_1_00: u32,param_2_00: u32)

{
    let mut uVar1: i32;
    let mut uVar2: u16;
    let mut in_DX: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    loop
    {
        uVar1 = pass1_fn_1000_47a4(param_2_00, &PTR_s_n_Op_Op_1050_002c_1050_518a);
        _local_8 = CONCAT22(in_DX, uVar1);
        if ((in_DX | uVar1) == 0) {
            break;
        }
        if (*_local_8 != '\"')
        {
            uVar2 = pass1_fn_1000_3e2c(CONCAT22(in_DX, uVar1));
            iVar3 = param_1_00;
            uVar4 = (param_1_00 >> 0x10);
            if (local_4 < 0x25)
            {
                (local_4 * 4 + iVar3) = uVar2;
                (local_4 * 4 + iVar3 + 2) = in_DX;
            }
            else
            {
                if (local_4 == 0x25)
                {
                    (iVar3 + 0x94) = uVar2;
                }
                else
                {
                    if (local_4 == 0x26)
                    {
                        (iVar3 + 0x96) = uVar2;
                    }
                }
            }
            local_4 = local_4 + 1;
        }
        param_2_00 = 0;
    }
    return;
}


pub fn pass1_1030_3b28()

{
    let mut local_SS__1: u16;
    let mut local_DXAX_17: u32;
    let mut u_var1: u32;
    let struct_a: *mut astruct_880;
    let mut u8_array_a: [u8;6];

    local_DXAX_17 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffc4, 0);
    pass1_1030_3af6(0x105065e6, 0x115, 0x15b, local_DXAX_17,
                    (local_DXAX_17 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0x19);
    pass1_1030_3af6(0x105065f0, 0x116, 0x15c, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffdd, 0x32);
    pass1_1030_3af6(0x105065fa, 0x117, 0x15d, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0x4b);
    pass1_1030_3af6(0x10506604, 0x118, 0x15e, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xf, 100);
    pass1_1030_3af6(0x1050660e, 0x119, 0x15f, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x28, 0x7d);
    pass1_1030_3af6(0x10506618, 0x11a, 0x160, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffec, 0x96);
    pass1_1030_3af6(0x10506622, 0x11b, 0x161, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x14, 0xaf);
    pass1_1030_3af6(0x1050662c, 0x11c, 0x162, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x1e, 200);
    pass1_1030_3af6(0x10506636, 0x11d, 0x163, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xfffb, 0xe1);
    pass1_1030_3af6(0x10506640, 0x11e, 0x164, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x32, 0xfa);
    pass1_1030_3af6(0x1050664a, 0x11f, 0x165, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x1e, 0xe1);
    pass1_1030_3af6(0x10506654, 0x120, 0x166, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffe7, 0xfa);
    pass1_1030_3af6(0x1050665e, 0x121, 0x167, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0x113);
    pass1_1030_3af6(0x10506668, 0x122, 0x168, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x28, 300);
    pass1_1030_3af6(0x10506672, 0x123, 0x169, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xf, 0x145);
    pass1_1030_3af6(0x1050667c, 0x124, 0x16a, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffec, 0x15e);
    pass1_1030_3af6(0x10506686, 0x125, 0x16b, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0);
    pass1_1030_3af6(0x10506690, 0x126, 0x16c, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x2d, 0x19);
    pass1_1030_3af6(0x1050669a, 0x127, 0x16d, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 10, 0x32);
    pass1_1030_3af6(0x105066a4, 0x128, 0x16e, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffe2, 0x4b);
    pass1_1030_3af6(0x105066ae, 0x129, 0x16f, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 5, 100);
    pass1_1030_3af6(0x105066b8, 0x12a, 0x170, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x32, 0x7d);
    pass1_1030_3af6(0x105066c2, 299, 0x171, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffc9, 0x96);
    pass1_1030_3af6(0x105066cc, 300, 0x172, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xfffb, 0xaf);
    pass1_1030_3af6(0x105066d6, 0x12d, 0x173, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffe7, 200);
    pass1_1030_3af6(0x105066e0, 0x12e, 0x174, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x32, 0x32);
    pass1_1030_3af6(0x105066ea, 0x12f, 0x175, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x3c, 100);
    pass1_1030_3af6(0x105066f4, 0x130, 0x176, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffc4, 0xe1);
    pass1_1030_3af6(0x105066fe, 0x131, 0x177, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0x19);
    pass1_1030_3af6(0x10506708, 0x132, 0x178, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 5, 0xaf);
    pass1_1030_3af6(0x10506712, 0x133, 0x179, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0x19);
    pass1_1030_3af6(0x1050671c, 0x134, 0x17a, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x23, 0x19);
    pass1_1030_3af6(0x10506726, 0x135, 0x17b, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xfffb, 0x32);
    pass1_1030_3af6(0x10506730, 0x136, 0x17c, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xf, 0x32);
    pass1_1030_3af6(0x1050673a, 0x137, 0x17d, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x2d, 0x4b);
    pass1_1030_3af6(0x10506744, 0x138, 0x17e, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x1e, 0x4b);
    pass1_1030_3af6(0x1050674e, 0x139, 0x17f, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x2d, 100);
    pass1_1030_3af6(0x10506758, 0x13a, 0x180, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffe7, 0x7d);
    pass1_1030_3af6(0x10506762, 0x13b, 0x181, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 5, 0xaf);
    pass1_1030_3af6(0x1050676c, 0x13c, 0x182, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 200);
    pass1_1030_3af6(0x10506776, 0x13d, 0x183, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffce, 200);
    pass1_1030_3af6(0x10506780, 0x13e, 0x184, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xf, 0xfa);
    pass1_1030_3af6(0x1050678a, 0x13f, 0x185, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x1e, 0x113);
    pass1_1030_3af6(0x10506794, 0x140, 0x186, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffe2, 300);
    pass1_1030_3af6(0x1050679e, 0x141, 0x187, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 100, 300);
    pass1_1030_3af6(0x105067a8, 0x142, 0x188, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x32, 0x145);
    pass1_1030_3af6(0x105067b2, 0x143, 0x189, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 100, 0x145);
    pass1_1030_3af6(0x105067bc, 0x144, 0x18a, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x1e, 0x15e);
    pass1_1030_3af6(0x105067c6, 0x145, 0x18b, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffd3, 0x15e);
    pass1_1030_3af6(0x105067d0, 0x146, 0x18c, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x32, 0xfa);
    pass1_1030_3af6(0x105067da, 0x147, 0x18d, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xf, 0x19);
    pass1_1030_3af6(0x105067e4, 0x148, 0x18e, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0x32);
    pass1_1030_3af6(0x105067ee, 0x149, 399, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0xaf);
    pass1_1030_3af6(0x105067f8, 0x14a, 400, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xfffb, 0xe1);
    pass1_1030_3af6(0x10506802, 0x14b, 0x191, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 10, 0x15e);
    pass1_1030_3af6(0x1050680c, 0x14c, 0x192, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0x19);
    pass1_1030_3af6(0x10506816, 0x14d, 0x193, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x1e, 0x32);
    pass1_1030_3af6(0x10506820, 0x14e, 0x194, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xfffb, 100);
    pass1_1030_3af6(0x1050682a, 0x14f, 0x195, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xf, 100);
    pass1_1030_3af6(0x10506834, 0x150, 0x196, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x1e, 0x7d);
    pass1_1030_3af6(0x1050683e, 0x151, 0x197, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffdd, 0xe1);
    pass1_1030_3af6(0x10506848, 0x152, 0x198, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0x113);
    pass1_1030_3af6(0x10506852, 0x153, 0x199, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0x2d, 300);
    pass1_1030_3af6(0x1050685c, 0x154, 0x19a, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffe7, 0x145);
    pass1_1030_3af6(0x10506866, 0x155, 0x19b, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 10, 0x15e);
    pass1_1030_3af6(0x10506870, 0x156, 0x19c, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0x4b);
    pass1_1030_3af6(0x1050687a, 0x157, 0x19d, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 5, 100);
    pass1_1030_3af6(0x10506884, 0x158, 0x19e, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0xffec, 0x96);
    pass1_1030_3af6(0x1050688e, 0x159, 0x19f, uVar1, (uVar1 >> 0x10));
    uVar1 = pass1_1008_3e54(CONCAT22(local_SS__1, u8_array_a), 0, 0, 0x113);
    struct_a = pass1_1030_3af6(0x10506898, 0x15a, 0x1a0, uVar1,
                               (uVar1 >> 0x10));
    return struct_a;
}


pub fn pass1_1030_34da(struct_1: *mut astruct_393)

{
    let struct_2_2: *mut astruct_393;
    let mut struct_2: u16;
    let mut local_res6: u16;

    struct_2 = (struct_1 >> 0x10);
    struct_2_2 = struct_1;
    struct_2_2.field_0x176 = 1;
    struct_2_2.field_0x178 = 1;
    struct_2_2.field_0x17a = 1;
    struct_2_2.field_0x17c = 1;
    struct_2_2.field_0x17e = 4;
    struct_2_2.field_0x182 = 0x32;
    struct_2_2.field_0x184 = 10;
    struct_2_2.field_0x186 = 10;
    struct_2_2.field_0x188 = 10;
    struct_2_2.field_0x18a = 0x4b;
    pass1_1000_4906((struct_1 & 0xffff0000 | ZEXT24(struct_2_2 + 1)), 0, 0x18);
    return;
}

pub fn pass1_1030_3534(param_1: u32,param_2: u32)

{
    (param_1 + 4) = param_2;
    return;
}

pub fn pass1_1030_3548(param_1: u32, param_2: libc::c_long)

{
    let plVar1: *mut long;

    plVar1 = (param_1 + 4);
    *plVar1 = *plVar1 + param_2;
    return;
}

pub fn pass1_1030_355c(param_1: u32,param_2: u32)

{
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut local_4: u16;

    local_4 = 0;
    while
    {
        iVar1 = local_4 * 4;
        uVar2 = (param_1 >> 0x10);
        (param_1 + iVar1 + 4) =
            (iVar1 + param_2) + (param_1 + 4 + iVar1);
        local_4 = local_4 + 1;
        local_4 < 0x5b
    } {}
    return;
}





pub fn pass1_1030_35a4(struct_a: *mut astruct_393, param_2: libc::c_long)

{
    let puVar1: *mut u32;
    let piVar2: *mut i32;
    let paVar3: *mut astruct_393;
    let uVar4: u8;
    let uVar5: u8;
    let uVar6: u8;
    let uVar7: u8;
    let struct_b: *mut astruct_94;
    let struct_c: *mut astruct_876;
    let struct_f: *mut astruct_876;
    let mut uVar8: i32;
    let in_DX: *mut u16;
    let mut extraout_DX: i32;
    let mut iVar10: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut uVar11: u16;
    let paVar12: *mut astruct_393;
    let mut local_SS__1: u16;
    let struct_d: *mut astruct_393;
    let struct_e: *mut astruct_393;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut u8_array_a: [u8;2];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_3f701da5c8: u8;
    let mut uVar9: u32;
    let extraout_var: u32;

    struct_e._0_1_ = param_2;
    struct_e._1_1_ = (param_2 >> 8);
    struct_d = &g_alloc_addr_1050_1050;
    _g_astruct_94_ptr_1 = struct_b;
    wvsprintf_FUN_1030_840a(0x6a, &g_alloc_addr_1050_1050, struct_e._0_1_, (param_2 >> 0x10));
    if (__g_astruct_94_ptr_1 == 0)
    {
        struct_e._0_1_ = 0xd1;
        struct_e._1_1_ = 0x35;
        struct_fn_1000_160a();
        g_u16_ptr_1050_5f2e = in_DX;
    }
    else
    {
    }
    struct_e._0_1_ = SUB21(_g_astruct_94_ptr_1, 0);
    struct_e._1_1_ = (_g_astruct_94_ptr_1 >> 8);
    struct_d = (&PTR_LOOP_1050_0000 + 1);
    local_18 = _g_astruct_94_ptr_1;
    local_16 = g_u16_ptr_1050_5f2e;
    alloc_mem_1000_1708(0x6c, 0, 1, struct_e._0_1_, g_u16_ptr_1050_5f2e);
    _local_6 = CONCAT22(g_u16_ptr_1050_5f2e, _g_astruct_94_ptr_1);
    struct_d = &local_a;
    struct_e._0_1_ = local_SS__1;
    uVar4 = struct_e._0_1_;
    struct_e._1_1_ = (local_SS__1 >> 8);
    uVar6 = struct_e._1_1_;
    pass1_1030_3948(struct_a, CONCAT13(struct_e._1_1_, CONCAT12(struct_e._0_1_, u8_array_a)),
                    CONCAT13(struct_e._1_1_, CONCAT12(struct_e._0_1_, struct_d)), 3);
    struct_d = u8_array_a;
    uVar9 = (&local_a + 2);
    struct_e._0_1_ = uVar4;
    struct_e._1_1_ = uVar6;
    pass1_1030_3948(struct_a, CONCAT13(uVar6, CONCAT12(uVar4, &local_a + 2)),
                    CONCAT13(uVar6, CONCAT12(uVar4, struct_d)), 4);
    iVar10 = extraout_DX;
    paVar12 = &PTR_LOOP_1050_1000;
    while
    {
        struct_c = uVar9;
        struct_d = struct_a;
        paVar3 = struct_d;
        if (param_2 < 1) {
            break;
        }
        struct_e._0_1_ = local_a;
        struct_e._1_1_ = (local_a >> 8);
        struct_d = paVar12;
        pass1_fn_1008_612e(struct_e._0_1_, (local_a >> 0x10));
        struct_d = &param_2;
        struct_e._0_1_ = uVar4;
        struct_e._1_1_ = uVar6;
        local_18 = struct_c;
        struct_f._0_1_ =
            pass1_1030_3a3a(struct_a, CONCAT13(uVar6, CONCAT12(uVar4, struct_d)), struct_c);
        uVar9 = CONCAT31(extraout_var, struct_f);
        uVar11 = (_local_6 >> 0x10);
        puVar1 = (local_18 * 4 + _local_6);
        uVar8 = *puVar1;
        *puVar1 = *puVar1 + uVar9;
        piVar2 = (local_18 * 4 + _local_6 + 2);
        *piVar2 = *piVar2 + iVar10 + CARRY2(uVar8, uVar9);
        struct_e._0_1_ = (struct_a >> 0x10);
        uVar5 = struct_e._0_1_;
        struct_e._1_1_ = (struct_a >> 0x18);
        uVar7 = struct_e._1_1_;
        struct_d = paVar3;
        pass1_1030_38f2(paVar3, struct_e._0_1_, 3);
        uVar8 = uVar9;
        struct_d = paVar3;
        struct_e._0_1_ = uVar5;
        struct_e._1_1_ = uVar7;
        pass1_1030_38f2(paVar3, uVar5, 4);
        iVar10 = extraout_DX_01;
        paVar12 = &PTR_LOOP_1050_1008;
        (extraout_DX_00 + extraout_DX_01 + CARRY2(uVar8, uVar9) | uVar8 + uVar9) != 0
    } {}
    struct_e._0_1_ = 0;
    struct_e._1_1_ = 0;
    struct_d = ((struct_a & 0xffff0000) >> 0x10);
    pass1_1000_4906((struct_a & 0xffff0000 | ZEXT24(paVar3 + 1)), 0, 0x18);
    return;
}



pub fn pass1_1030_3694(param_1: *mut astruct_393, param_2: u16, param_3: libc::c_long)

{
    let puVar1: *mut u32;
    let piVar2: *mut i32;
    let mut uVar3: i32;
    let uVar4: u8;
    let in_AX: *mut astruct_94;
    let mut uVar5: u16;
    let mut local_register1_120: u16;
    let in_DX: *mut u16;
    let mut local_SS__1: u16;
    let uVar7: u8;
    let mut local_12: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut uVar6: u32;

    _g_astruct_94_ptr_1 = in_AX;
    wvsprintf_FUN_1030_840a(0x7a, &g_alloc_addr_1050_1050, param_3, (param_3 >> 0x10));
    if (__g_astruct_94_ptr_1 == 0)
    {
        struct_fn_1000_160a();
        g_u16_ptr_1050_5f2e = in_DX;
    }
    else
    {
    }
    local_12 = _g_astruct_94_ptr_1;
    alloc_mem_1000_1708(0x16c, 0, 1, _g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
    uVar6 = (param_2 - 1);
    uVar7 = (local_SS__1 >> 8);
    local_6 = _g_astruct_94_ptr_1;
    local_4 = g_u16_ptr_1050_5f2e;
    if (((param_2 < 1) || (SBORROW2(param_2, 1))) ||
        (uVar6 = (param_2 - 5), param_2 - 5 != 0 && 3 < (param_2 - 1)))
    {
        while (uVar5 = uVar6, 0 < param_3)
        {
            pass1_fn_1008_612e(0, 0x5a);
            local_12 = uVar5;
            uVar4 = pass1_1030_3a3a(param_1, CONCAT13(uVar7, CONCAT12(local_SS__1, &param_3)), uVar5);
            uVar6 = CONCAT31(_local_register1_120, uVar4);
            puVar1 = (local_12 * 4 + local_6);
            uVar3 = *puVar1;
            *puVar1 = *puVar1 + uVar6;
            piVar2 = (local_12 * 4 + local_6 + 2);
            *piVar2 = g_u16_ptr_1050_5f2e + CARRY2(uVar3, uVar6) + *piVar2;
        }
    }
    else
    {
        pass1_1030_39dc(param_1, CONCAT13(uVar7, CONCAT12(local_SS__1, &param_3)),
                        CONCAT13((g_u16_ptr_1050_5f2e >> 8),
                                 CONCAT12(g_u16_ptr_1050_5f2e, _g_astruct_94_ptr_1)),
                        param_2);
    }
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x18c)), 0, 0x18);
    return;
}



pub fn pass1_1030_375a(param_1: *mut astruct_393, param_2: u16, param_3: libc::c_long)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut uVar3: i32;
    let mut iVar4: i32;
    let local_SI_165: *mut astruct_879;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    iVar4 = param_1;
    if (param_2 == 0)
    {
        local_4 = 0x5a;
        while ((-1 < local_4 &&
                (pass1_1030_3a3a(param_1, CONCAT22(unaff_SS, &param_3), local_4), param_3 != 0)))
        {
            local_4 = local_4 - 1;
        }
    }
    else
    {
        pass1_1030_3948(param_1, CONCAT22(unaff_SS, &local_4), CONCAT22(unaff_SS, &local_6), param_2);
        local_a = (local_4 - local_6) + 1;
        local_8 = local_a >> 0xf;
        local_e = param_3 / local_a;
        uVar3 = (local_e * local_a);
        _local_12 = CONCAT22(((param_3 >> 0x10) -
                              (local_e * local_a >> 0x10)) -
                                 (param_3 < uVar3),
                             param_3 - uVar3);
        local_14 = local_6;
        while (local_14 <= local_4)
        {
            local_SI_165 = (local_14 * 4);
            uVar5 = (param_1 >> 0x10);
            (local_SI_165 + iVar4 + 4) = (local_SI_165 + iVar4 + 4) - local_e;
            iVar1 = (local_SI_165 + iVar4 + 6);
            if ((local_10 | local_12) != 0)
            {
                iVar2 = (local_SI_165 + iVar4 + 4);
                (local_SI_165 + iVar4 + 4) = iVar2 + -1;
                (local_SI_165 + iVar4 + 6) = iVar1 - (iVar2 == 0);
                _local_12 = _local_12 + -1;
            }
            if ((iVar4 + local_14 * 4 + 6) < 0)
            {
                (iVar4 + local_14 * 4 + 4) = 0;
            }
            local_14 = local_14 + 1;
        }
    }
    pass1_1000_4906((param_1 & 0xffff0000 | (iVar4 + 0x18c)), 0, 0x18);
    return;
}

pub fn pass1_1030_387c(param_1: u32)

{
    let mut local_4: u16;

    local_4 = 0x5a;
    while
    {
        (local_4 * 4 + param_1 + 4) = (local_4 * 4 + param_1);
        local_4 = local_4 - 1;
        0 < local_4
    } {}
    (param_1 + 4) = 0;
    return;
}

pub fn pass1_1030_38b8()

{
    let mut local_8: u16;
    let mut local_6: u32;

    local_8 = 0;
    while
    {
        local_8 = local_8 + 1;
        local_8 < 0x5b
    } {}
    return;
}

pub fn pass1_1030_38f2(param_1: *mut astruct_393, param_2: u16)

{
    let mut unaff_SS: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    pass1_1030_3948(param_1, CONCAT22(unaff_SS, &local_a), CONCAT22(unaff_SS, &local_8), param_2);
    local_c = local_8;
    while (local_c <= local_a)
    {
        local_c = local_c + 1;
    }
    return;
}

pub fn pass1_1030_3948(struct_a: *mut astruct_393,param_2: u32,param_3: u32, param_4: u16)

{
    let struct_a_hi: *mut astruct_393;

    if (param_4 == 1)
    {
        param_3 = 0;
        param_2 = 3;
        return;
    }
    struct_a_hi = (struct_a >> 0x10);
    if (param_4 == 2)
    {
        param_3 = 4;
        param_2 = (struct_a + 0x1ae);
        return;
    }
    if (param_4 == 3)
    {
        param_3 = (struct_a + 0x1ae) + 1;
        param_2 = 0x27;
        return;
    }
    if (param_4 != 4)
    {
        if (param_4 == 5)
        {
            param_3 = 0x4c;
        }
        else
        {
            param_3 = 0;
        }
        param_2 = 0x5a;
        return;
    }
    param_3 = 0x28;
    param_2 = 0x4b;
    return;
}

pub fn pass1_1030_39dc(param_1: *mut astruct_393, param_2: *mut long,param_3: u32, param_4: u16)

{
    let uVar1: u8;
    let extraout_AH: u8;
    let mut extraout_DX: u16;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_3948(param_1, CONCAT22(unaff_SS, &local_6), CONCAT22(unaff_SS, &local_4), param_4);
    local_8 = local_6;
    uVar2 = extraout_DX;
    loop
    {
        if (local_8 < local_4)
        {
            return;
        }
        uVar1 = pass1_1030_3a3a(param_1, param_2, local_8);
        uVar3 = (param_3 >> 0x10);
        (local_8 * 4 + param_3) = CONCAT11(extraout_AH, uVar1);
        (local_8 * 4 + param_3 + 2) = uVar2;
        if (*param_2 == 0) {
            break;
        }
        local_8 = local_8 - 1;
    }
    return;
}

pub fn pass1_1030_3a3a(struct_a: *mut astruct_393,param_2: u32, param_3: u16)

{
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let struct_a_2: *mut astruct_393;
    let puVar6: *mut u16;
    let local_SI_30: *mut astruct_877;
    let struct_a_hi: *mut astruct_393;
    let mut local_6: u32;
    let mut temp_5f7d4063d1: u32;

    temp_5f7d4063d1 = param_2;
    iVar2 = (param_2 + 2);
    struct_a_hi = (struct_a >> 0x10);
    struct_a_2 = struct_a;
    puVar6 = &struct_a_2.u16_0x4;
    local_SI_30 = (param_3 * 4);
    piVar1 = (puVar6 + local_SI_30 + 2);
    iVar3 = *piVar1;
    uVar5 = temp_5f7d4063d1;
    if ((iVar3 < iVar2) || ((*piVar1 == iVar2 || iVar3 < iVar2 && (puVar6[param_3 * 2] < uVar5))))
    {
        param_2 = param_2 - (&struct_a_2.u16_0x4 + param_3 * 2);
        (&struct_a_2.u16_0x4 + param_3 * 2) = 0;
    }
    else
    {
        uVar4 = puVar6[param_3 * 2];
        iVar3 = (puVar6 + local_SI_30 + 2);
        (struct_a_2 + local_SI_30 + 4) = uVar4 - uVar5;
        (struct_a_2 + local_SI_30 + 6) = (iVar3 - iVar2) - (uVar4 < uVar5);
        param_2 = 0;
    }
    return (temp_5f7d4063d1 - param_2);
}

pub fn pass1_1030_3ac6(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1030_2f1a(param_1: u32, param_2: *mut u16, param_3: *mut u16) -> i32

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut iVar3: i32;

    uVar2 = (param_1 + 0x10);
    iVar3 = uVar2;
    iVar1 = (iVar3 + 0xc);
    if (iVar1 - 1 < 9)
    {
        match (iVar1)
        {
// default:
       _ => {
            *param_3 = 0x19;
            *param_2 = 0x2d;
            return iVar3;
       },
        3 |
        4 |
        5 =>{
            *param_3 = 10;
            *param_2 = 0xf;
            return iVar3;
        },
        6 =>{
            *param_3 = 10;
            *param_2 = 0x19;
            return iVar3;
        }, 7 =>{
            *param_3 = 0x19;
            *param_2 = 0x37;
            return iVar3;
        }
    }
    *param_3 = 0;
    *param_2 = 0;
    return 0;
}

pub fn pass1_1030_2fac(param_1: u32)

{
    let mut u_var1: u32;
    let local_BX_3: *mut astruct_867;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0x10 == 0)
    {
        return 0;
    }
    uVar1 = local_BX_3.field_0x10;
    if ((uVar1 + 0xc) < 2)
    {
        return 4;
    }
    uVar1 = local_BX_3.field_0x10;
    if ((uVar1 + 0xc) < 5)
    {
        return 3;
    }
    uVar1 = local_BX_3.field_0x10;
    if ((uVar1 + 0xc) < 8)
    {
        return 2;
    }
    return 1;
}

pub fn pass1_1030_3006(param_1: u32,param_2: u32)

{
    (param_1 + 0x10) = param_2;
    return;
}

pub fn pass1_1030_301a(param_1: u32,param_2: u32)

{
    let mut u_var1: u32;
    let uVar2: u8;
    let extraout_var: u32;
    let mut in_DX: u16;
    let local_BX_4: *mut astruct_869;
    let local_BX_46: *mut astruct_868;
    let mut uVar4: u16;
    let mut uvar3: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.field_0x10 != 0)
    {
        uVar1 = local_BX_4.field_0x10;
        uVar2 = error_check_1000_17ce((uVar1 + 2));
        uVar3 = CONCAT31(extraout_var, uVar2);
        pass1_fn_1008_60e8(param_2);
        uVar1 = local_BX_4.field_0x10;
        uVar4 = (uVar1 >> 0x10);
        local_BX_46 = uVar1;
        &local_BX_46.field_0x2 = uVar3;
        local_BX_46.field_0x4 = in_DX;
    }
    return;
}



pub fn pass1_1030_3058(param_1: u32)

{
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut in_AX: i32;
    let mut uvar3: u16;
    let in_DX: *mut astruct_199;
    let paVar4: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let local_BX_5: *mut astruct_870;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    paVar4 = in_DX;
    if (&local_BX_5.field_0xc == 0)
    {
        process_struct_1000_179c(0x18, in_DX);
        paVar4 = (in_DX | in_AX);
        if (paVar4 == 0x0)
        {
            &local_BX_5.field_0xc = 0;
        }
        else
        {
            uVar6 = pass1_1030_1cd8(CONCAT22(in_DX, in_AX), 5, 5);
            paVar4 = (uVar6 >> 0x10);
            local_BX_5.field_0xc = uVar6;
            &local_BX_5.field_0xe = paVar4;
        }
    }
    local_4 = 0;
    while (uVar6 = local_BX_5.field_0x10, puVar1 = (uVar6 + 0x22),
           local_4 <= *puVar1 && *puVar1 != local_4)
    {
        uVar3 = local_4;
        pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
        ppcVar2 = (&local_BX_5.field_0xc + 8);
        ppcVar2(&PTR_LOOP_1050_1028, &local_BX_5.field_0xc, uVar3, paVar4, local_4,
                    0);
        local_4 = local_4 + 1;
        paVar4 = extraout_DX;
    }
    return 1;
}

pub fn pass1_1030_310a(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_29e6(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1030_314c(param_1: *mut u16,param_2: u32)

{
    let local_BX_5: *mut astruct_871;
    let mut unaff_SI: u16;
    let mut uVar1: u16;
    let mut local_c: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    *param_1 = s_1_1050_389a;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_5.field_0x170 = 0;
    local_BX_5.field_0x1a4 = param_2;
    local_BX_5.field_0x1a8 = 5;
    local_BX_5.field_0x1aa = 0;
    local_BX_5.field_0x1ae = 0x10;
    *param_1 = s_G1_1050_3af2;
    local_BX_5.field_0x2 = 0x1030;
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_5.field_0x4),
                    0, 0x16c);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_5.field_0x18c), 0, 0x18);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_5.field_0x174), 0, 0xc);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_5.field_0x180), 0, 0xc);
    process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 2));
    if (u16_1050_13ae < 2)
    {
        pass1_1030_34da(param_1);
    }
    else
    {
        local_BX_5.field_0x176 = 1;
        local_BX_5.field_0x178 = 2;
        local_BX_5.field_0x17a = 2;
        local_BX_5.field_0x17c = 0x60001;
        local_c = 1;
        while (local_c < 6)
        {
            (&local_BX_5.field_0x180 + local_c * 2) = 100;
            local_c = local_c + 1;
        }
    }
    return;
}

pub fn pass1_1030_3258(param_1: u32, param_2: u16)

{
    (param_1 + 0x1ae) = param_2;
    return;
}



pub fn pass1_1030_326a(param_1: *mut astruct_872)

{
    let mut uVar1: i32;
    let in_EAX: u32;
    let mut uVar2: u32;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let local_BX_4: *mut astruct_872;
    let mut uVar4: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x1aa == 0)
    {
        &local_BX_4.field_0x1aa = 1;
    }
    else
    {
        in_EAX = &local_BX_4.field_0x1aa * 2;
        &local_BX_4.field_0x1aa = in_EAX;
    }
    uVar1 = in_EAX;
    pass1_1030_38b8(param_1);
    _local_6 = CONCAT22(extraout_DX, uVar1);
    uVar2 = &local_BX_4.field_0x1aa;
    uVar3 = local_BX_4.field_0x1ac;
    if (_local_6 < uVar2)
    {
        uVar2 = uVar1;
        uVar3 = extraout_DX;
    }
    local_BX_4.field_0x1aa = uVar2;
    local_BX_4.field_0x1ac = uVar3;
    pass1_1030_375a(param_1, 0, uVar2, uVar3);
    return;
}



pub fn pass1_1030_23e2(param_1: u32, param_2: i32, param_3: i32)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut bVar3: bool;
    let mut uVar4: u16;
    let mut iVar5: i32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut uVar6: u16;
    let local_BX_6: *mut astruct_859;
    let mut unaff_SI: u16;
    let mut uVar7: u16;
    let ppVar8: *mut pass1_struct_1;
    let mut iVar9: i32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar7 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    if ((&local_BX_6.field_0x10 + param_3 * 2) < 0)
    {
        iVar9 = param_3;
        if (param_2 == 0)
        {
            ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x31));
            ppcVar1 = (ppVar8 + 0x14);
            (**ppcVar1)(0x1010, ppVar8, (ppVar8 >> 0x10), param_3, param_3 >> 0xf);
            uVar6 = extraout_DX_00;
        }
        else
        {
            ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_SI, 0x41));
            ppcVar1 = (ppVar8 + 0x14);
            (**ppcVar1)(0x1010, ppVar8, (ppVar8 >> 0x10), param_3, param_3 >> 0xf);
            uVar6 = extraout_DX;
        }
        uVar2 = (iVar9 + 0x16);
        (&local_BX_6.field_0x10 + param_3 * 2) = (uVar2 + 4);
    }
    if ((&local_BX_6.field_0x10 + param_3 * 2) != 0)
    {
        local_8 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x400);
        pass1_1030_2852();
        bVar3 = false;
        if (local_BX_6.field_0x152 != 0)
        {
            uVar4 = pass1_1030_266c(local_BX_6, CONCAT22(param_3, uVar7));
            if (uVar4 != 0)
            {
                local_8 = local_8 + 1;
                bVar3 = true;
            }
        }
        iVar9 = param_3 * 2;
        iVar5 = (&local_BX_6.field_0x10 + iVar9) - local_8;
        (&local_BX_6.field_0x10 + iVar9) = iVar5;
        if (iVar5 < 0)
        {
            (&local_BX_6.field_0x10 + iVar9) = 0;
        }
        iVar9 = param_3 * 2;
        if ((&local_BX_6[1].field_0x0 + iVar9) == 0)
        {
            iVar5 = &local_BX_6.field_0x0 + iVar9;
            (iVar5 + 0x2ac) = 1;
            (iVar5 + 0x1a6) = (&local_BX_6.field_0x1a6 + iVar9) + -1;
            if ((&local_BX_6.field_0x1a6 + iVar9) < 0)
            {
                (iVar5 + 0x1a6) = 0;
            }
        }
        if (((&local_BX_6.field_0x10 + param_3 * 2) != 0) ||
            ((&local_BX_6.field_0x1a6 + param_3 * 2) != 0))
        {
            if ((&local_BX_6.field_0x10 + param_3 * 2) == 0)
            {
                (&local_BX_6.field_0x10 + param_3 * 2) = 1;
            }
            return;
        }
        iVar9 = param_3;
        ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(param_3, 0x32));
        process_switch_stmt_1010_6cf8(0x1010, ppVar8, iVar9);
        pass1_1030_26ac(param_1, param_3);
        if (bVar3)
        {
            uVar4 = pass1_1030_28dc(param_1, param_3);
            (&local_BX_6.field_0x19c + uVar4 * 2) = 0;
        }
    }
    return;
}

pub fn pass1_1030_25b2(param_1: u32, param_2: i32)

{
    if ((param_1 + 0x10 + param_2 * 2) == 0)
    {
        return 1;
    }
    return 0;
}

pub fn pass1_1030_25d8(struct_a: *mut astruct_858, param_2: u16, param_3: i32)

{
    (struct_a + param_3 * 2 + 0x10) = param_2;
    return;
}

pub fn pass1_1030_25f0(param_1: *mut astruct_858, param_2: i32, param_3: i32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if (param_2 == 0)
    {
        param_2 = (param_1 + 0x116 + param_3 * 2) + 1;
    }
    (param_1 + param_3 * 2 + 0x116) = param_2;
    return;
}

pub fn pass1_1030_2622(param_1: u32, param_2: i32)

{
    let local_AX_38: *mut astruct_861;
    let mut local_4: u16;

    if ((param_2 != 0x70) && (param_2 != 1))
    {
        local_AX_38 = pass1_1030_28dc(param_1, 0);
        if (-1 < local_AX_38)
        {
            (param_1 + local_AX_38 * 2 + 0x19c) = param_2;
        }
        local_4 = (-1 < local_AX_38);
        return local_4;
    }
    return 0;
}

pub fn pass1_1030_266c(param_1: u16,param_2: u32)

{
    let mut uVar1: u16;
    let mut local_4: u16;

    uVar1 = pass1_1030_28dc(CONCAT22(param_2, param_1), (param_2 >> 0x10));
    local_4 = (uVar1 != 0xffff);
    return local_4;
}

pub fn pass1_1030_2690(param_1: *mut astruct_393)

{
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x2ac)), 0,
                    0x106);
    return;
}

pub fn pass1_1030_26ac(param_1: u32, uparam_2: i32)

{
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut cVar5: u8;
    let puVar6: *mut u16;
    let mut uVar7: u16;
    let mut iVar8: i32;
    let mut iVar9: i32;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let mut iVar10: i32;
    let mut uVar11: u16;
    let mut unaff_SS: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    iVar10 = param_1;
    uVar11 = (param_1 >> 0x10);
    if (param_2 != 0x13)
    {
        if (0x13 < param_2)
        {
            if (param_2 != 0x5f)
            {
                if ((param_2 - 0x5f) < 6)
                {
                    return;
                }
                if (param_2 != 0x66 && 0 < (param_2 - 0x65))
                {
                    if ((param_2 - 0x66) < 5)
                    {
                        return;
                    }
                    if (4 < (param_2 - 0x6b))
                    {
                        return;
                    }
                }
            }
            pass1_1028_dc52(CONCAT22(unaff_SS, &local_14),
                            (&PTR_LOOP_1050_0000 + 1), 0, 0x400);
            loop
            {
                puVar6 = &local_14;
                pass1_1028_e4ec(CONCAT22(unaff_SS, puVar6));
                if ((extraout_DX | puVar6) == 0) {
                    break;
                }
                if ((iVar10 + 4) == (puVar6 + 0x100))
                {
                    uVar7 = puVar6[0xc] + 0x19;
                    if (1000 < uVar7)
                    {
                        uVar7 = 1000;
                    }
                    pass1_1038_4d0e(CONCAT22(extraout_DX, puVar6), uVar7);
                }
            }
            return;
        }
        if (param_2 == 0x12)
        {
            pass1_1028_dc52(CONCAT22(unaff_SS, &local_14),
                            (&PTR_LOOP_1050_0000 + 1), 0, 0x400);
            loop
            {
                puVar6 = &local_14;
                pass1_1028_e4ec(CONCAT22(unaff_SS, puVar6));
                if ((extraout_DX_01 | puVar6) == 0) {
                    break;
                }
                if ((iVar10 + 4) == (puVar6 + 0x100))
                {
                    uVar2 = (puVar6 + 0xfb);
                    iVar9 = uVar2;
                    uVar4 = (uVar2 >> 0x10);
                    piVar1 = (iVar9 + 0x182);
                    *piVar1 = *piVar1 + -0x19;
                    iVar8 = (iVar9 + 0x182);
                    if (iVar8 < 1)
                    {
                        iVar8 = 1;
                    }
                    (iVar9 + 0x182) = iVar8;
                }
            }
            return;
        }
        if (0x12 < param_2)
        {
            return;
        }
        cVar5 = param_2;
        if (cVar5 != '\n')
        {
            if ((cVar5 + -10) < '\x06')
            {
                return;
            }
            if (0x1 < (cVar5 + -0x10))
            {
                return;
            }
        }
    }
    pass1_1028_dc52(CONCAT22(unaff_SS, &local_14), (&PTR_LOOP_1050_0000 + 1), 0, 0x400);
    loop
    {
        puVar6 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar6));
        if ((extraout_DX_00 | puVar6) == 0) {
            break;
        }
        if ((iVar10 + 4) == (puVar6 + 0x100))
        {
            uVar2 = (puVar6 + 0xfb);
            iVar8 = uVar2 + 0x180;
            uVar4 = (uVar2 >> 0x10);
            local_26 = 1;
            while
            {
                iVar3 = local_26 * 2;
                piVar1 = (iVar3 + iVar8);
                unsafe{ *piVar1 = *piVar1 + -1 };
                iVar9 = (iVar3 + iVar8);
                if (iVar9 < 1)
                {
                    iVar9 = 1;
                }
                (iVar3 + iVar8) = iVar9;
                local_26 = local_26 + 1;
                local_26 < 6
            } {}
        }
    }
    return;
}



pub fn pass1_1030_2852()

{
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    return;
}

pub fn pass1_1030_28dc(param_1: u32, param_2: i32)

{
    let mut local_4: u16;

    local_4 = 0;
    loop
    {
        if (4 < local_4)
        {
            return 0xffff;
        }
        if ((param_1 + 0x19c + local_4 * 2) == param_2) {
            break;
        }
        local_4 = local_4 + 1;
    }
    return local_4;
}

pub fn pass1_1030_2916(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_18b2(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_2958(param_1: *mut astruct_862)

{
    let local_BX_18: *mut astruct_862;
    let mut uVar1: u16;

    pass1_1030_17ce(param_1, 5, 0xf);
    uVar1 = (param_1 >> 0x10);
    local_BX_18 = param_1;
    local_BX_18.field_0x10 = 0;
    local_BX_18.field_0x14 = 0;
    local_BX_18.field_0x16 = 0;
    local_BX_18.field_0x18 = (s_fem36_wav_1050_270c + 4);
    local_BX_18.field_0x1a = 0;
    param_1 = 0x3130;
    local_BX_18.field_0x2 = 0x1030;
    return;
}

pub fn pass1_1030_299a(param_1: *mut astruct_848,param_2: u32)

{
    let local_BX_28: *mut astruct_848;
    let mut uVar1: u16;

    pass1_1030_183c(param_1, 5, 0xf, 0x2000000, param_2);
    uVar1 = (param_1 >> 0x10);
    local_BX_28 = param_1;
    local_BX_28.field_0x10 = 0;
    local_BX_28.field_0x14 = 0;
    local_BX_28.field_0x16 = 0;
    local_BX_28.field_0x18 = (s_fem36_wav_1050_270c + 4);
    local_BX_28.field_0x1a = 0;
    param_1.field_0x0 = 0x3130;
    local_BX_28.field_0x2 = 0x1030;
    return;
}

pub fn pass1_1030_29e6(param_1: *mut astruct_850)

{
    let mut uVar1: i32;
    let in_struct_1: *mut astruct_44;
    let local_BX_4: *mut astruct_850;
    let mut uVar2: u16;
    let mut local_6: u32;
    let temp_5f367f3cdc: *mut astruct_44;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.field_0x0 = 0x3130;
    local_BX_4.field_0x2 = 0x1030;
    in_struct_1 = local_BX_4.field_0x10;
    uVar1 = (&local_BX_4.field_0x10 + 2);
    if ((uVar1 | in_struct_1) != 0)
    {
        pass1_1030_8496((in_struct_1 & 0xffff | uVar1 << 0x10));
        error_check_1000_17ce(in_struct_1);
    }
    pass1_1030_18b2(param_1);
    return;
}



pub fn pass1_1030_2a2c(param_1: *mut astruct_865)

{
    let piVar1: *mut i32;
    let local_BX_4: *mut astruct_865;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (0 < local_BX_4.field_0x18)
    {
        piVar1 = &local_BX_4.field_0x18;
        unsafe{ *piVar1 = *piVar1 + -1 };
    }
    if (local_BX_4.field_0x16 == 0)
    {
        local_BX_4.field_0x16 = 1;
    }
    if (local_BX_4.field_0x1a == 0)
    {
        local_BX_4.field_0x1a = 2;
    }
    if (local_BX_4.field_0x18 < 1)
    {
        local_BX_4.field_0x16 = 2;
        local_BX_4.field_0x1a = 1;
        uVar8 = 0;
        uVar9 = 0x21;
        uVar6 = 1;
        uVar7 = 0;
        uVar4 = 0;
        uVar5 = 0;
        uVar2 = 0;
        ppVar3 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x37);
        post_win_msg_1008_a0e4(ppVar3, CONCAT22(uVar4, uVar2), uVar5, uVar6, CONCAT22(uVar8, uVar7), uVar9);
    }
    return;
}

pub fn pass1_1030_2a98(param_1: u32)

{
    let piVar1: *mut i32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x14);
    unsafe{ *piVar1 = *piVar1 + 1 };
    return (param_1 + 0x14);
}

pub fn pass1_1030_2aaa(param_1: u32)

{
    let mut u_var1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x10) == 0)
    {
        return 0;
    }
    uVar1 = (param_1 + 0x10);
    return (uVar1 + 0xc);
}



pub fn pass1_1030_1be2(param_1: *mut astruct_851)

{
    let ppcVar1: fn();
    let mut uVar2: u32;
    let puVar3: *mut u32;
    let in_AX: *mut u16;
    let mut uVar4: u16;
    let in_DX: *mut astruct_199;
    let paVar5: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let local_BX_5: *mut astruct_851;
    let mut uVar6: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar6 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    paVar5 = in_DX;
    if (&local_BX_5.field_0xc == 0)
    {
        process_struct_1000_179c(0x18, in_DX);
        paVar5 = (in_DX | in_AX);
        if (paVar5 == 0x0)
        {
            &local_BX_5.field_0xc = 0;
        }
        else
        {
            pass1_1030_1cd8(CONCAT22(in_DX, in_AX), 5, 5);
            local_BX_5.field_0xc = in_AX;
            &local_BX_5.field_0xe = extraout_DX;
            paVar5 = extraout_DX;
        }
    }
    local_4 = 0;
    while (puVar3 = local_BX_5.field_0x10, local_4 <= *puVar3 && *puVar3 != local_4)
    {
        uVar2 = local_BX_5.field_0x10;
        uVar2 = (uVar2 + 2);
        uVar4 = (uVar2 + local_4 * 2);
        pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
        ppcVar1 = (&local_BX_5.field_0xc + 8);
        (**ppcVar1)(&PTR_LOOP_1050_1028, &local_BX_5.field_0xc, uVar4, paVar5, local_4,
                    0);
        local_4 = local_4 + 1;
        paVar5 = extraout_DX_00;
    }
    return;
}

pub fn pass1_1030_1c96(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_1a74(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_1cd8(param_1: *mut *mut astruct_706,param_2: u32,param_3: u32)

{
    let local_BX_4: *mut astruct_707;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    *param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = 0;
    local_BX_4.field_0x8 = 0;
    local_BX_4.field_0xc = param_3;
    local_BX_4.field_0x10 = 0;
    local_BX_4.field_0x14 = param_2;
    *param_1 = (s_579_bmp_1050_203f + 5);
    local_BX_4.field_0x2 = 0x1030;
    return;
}

pub fn pass1_1030_1d28(param_1: *mut astruct_708)

{
    let local_BX_4: *mut astruct_708;
    let local_ES_4: *mut astruct_708;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = (s_579_bmp_1050_203f + 5);
    local_BX_4.field_0x2 = 0x1030;
    error_check_1000_17ce(local_BX_4.field_0x4);
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}



pub fn pass1_1030_1d58(param_1: *mut u32)

{
    let ppcVar1: fn();
    let mut uVar2: u32;

    ppcVar1 = (*param_1 + 4);
    uVar2 = (**ppcVar1)();
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
    return;
}

pub fn pass1_1030_1d7c(param_1: *mut u32)

{
    let paVar1: *mut astruct_493;
    let mut uVar2: u32;
    let mut local_4: u16;

    paVar1 = pass1_1030_1d58(param_1);
    if (paVar1 != 0x0)
    {
        uVar2 = pass1_1030_73a8(paVar1);
        return uVar2;
    }
    return 0;
}

pub fn pass1_1030_1daa(param_1: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 10), (param_1 + 8));
}

pub fn pass1_1030_1dbc()

{
    let mut local_6: u32;

    return;
}

pub fn pass1_1030_1dfc(param_1: *mut u32, param_2: u16, param_3: u16,param_4: u32)

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let ppcVar3: fn();
    let mut uVar4: u32;
    let mut iVar5: i32;
    let local_BX_128: *mut astruct_852;
    let mut uVar6: u16;
    let mut bVar7: bool;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    puVar1 = (iVar5 + 8);
    if ((*puVar1 < param_4 || *puVar1 == param_4) || ((iVar5 + 4) == 0))
    {
        puVar2 = (iVar5 + 0x12);
        bVar7 = *puVar2 < param_4._2_2_;
        if ((bVar7 || *puVar2 == param_4._2_2_) &&
            ((bVar7 || (puVar2 = (iVar5 + 0x10),
                        *puVar2 < param_4 || *puVar2 == param_4))))
        {
            ppcVar3 = (*param_1 + 0x20);
            (**ppcVar3)();
        }
        puVar1 = (iVar5 + 0x10);
        if ((*puVar1 < param_4 || *puVar1 == param_4) || ((iVar5 + 4) == 0))
        {
            return;
        }
        puVar2 = (iVar5 + 10);
        bVar7 = *puVar2 < param_4._2_2_;
        if ((bVar7 || *puVar2 == param_4._2_2_) &&
            ((bVar7 || (puVar2 = (iVar5 + 8), *puVar2 < param_4 || *puVar2 == param_4))))
        {
            (iVar5 + 8) = (param_4 + 1);
            (iVar5 + 10) = (param_4 + 1 >> 0x10);
        }
    }
    uVar4 = (iVar5 + 4);
    uVar6 = (uVar4 >> 0x10);
    local_BX_128 = uVar4;
    (local_BX_128 + param_4 * 4) = param_2;
    (local_BX_128 + param_4 * 4 + 2) = param_3;
    return;
}

pub fn pass1_1030_1e96(param_1: *mut u32)

{
    let puVar1: *mut u32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut local_6: u32;

    local_6 = 0;
    loop
    {
        uVar4 = (param_1 >> 0x10);
        puVar1 = (param_1 + 8);
        if ((*puVar1 < local_6 || *puVar1 == local_6) ||
            (uVar3 = (param_1 + 4), (uVar3 + local_6 * 4) == 0)) {
            break;
            }
        local_6 = local_6 + 1;
    }
    ppcVar2 = (*param_1 + 8);
    ppcVar2();
    return;
}

pub fn pass1_1030_1eee(param_1: u32,param_2: u32)

{
    let mut u_var1: u32;
    let mut iVar2: i32;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    uVar1 = (iVar2 + 0xc);
    param_2._2_2_ = (iVar2 + 0xe);
    if (uVar1 < param_2)
    {
        uVar1 = param_2 & 0xffff;
    }
    (iVar2 + 0xc) = uVar1;
    (iVar2 + 0xe) = param_2._2_2_;
    return;
}

pub fn pass1_1030_1f16(param_1: *mut u32,param_2: u32)

{
    let plVar1: *mut long;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let local_BX_5: *mut astruct_853;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if ((local_BX_5.field_0x4 == 0) || (local_BX_5.field_0x10 <= &local_BX_5.field_0x8))
    {
        ppcVar2 = (*param_1 + 0x20);
        ppcVar2();
    }
    uVar3 = local_BX_5.field_0x4;
    (local_BX_5.field_0x8 * 4 + uVar3) = param_2;
    plVar1 = &local_BX_5.field_0x8;
    *plVar1 = *plVar1 + 1;
    return;
}




pub fn pass1_1030_1f70(param_1: u32)

{
    let puVar1: *mut u32;
    let paVar2: *mut astruct_94;
    let mut uVar3: i32;
    let extraout_DX: *mut u16;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0x10) == 0)
    {
        paVar2 = (iVar4 + 0xc);
        g_u16_ptr_1050_5f2e = (iVar4 + 0xe);
    }
    else
    {
        uVar3 = (iVar4 + 0x10);
        puVar1 = (iVar4 + 0x14);
        paVar2 = (uVar3 + *puVar1);
        g_u16_ptr_1050_5f2e =
            ((iVar4 + 0x12) + (iVar4 + 0x16) + CARRY2(uVar3, *puVar1));
    }
    _local_6 = CONCAT22(g_u16_ptr_1050_5f2e, paVar2);
    if ((iVar4 + 4) == 0)
    {
        if (__g_astruct_94_ptr_1 == 0)
        {
            _g_astruct_94_ptr_1 = paVar2;
            struct_fn_1000_160a();
        }
        else
        {
        }
        uVar3 = paVar2 << 2;
        alloc_mem_1000_1708(uVar3, 0, 1, _g_astruct_94_ptr_1, g_u16_ptr_1050_5f2e);
    }
    else
    {
        uVar3 = paVar2 * 4;
        alloc_mem_1000_0ed4(1, uVar3, (g_u16_ptr_1050_5f2e * 2 + CARRY2(paVar2, paVar2)) * 2 + CARRY2(paVar2 * 2, paVar2 * 2),
                            (iVar4 + 4));
        g_u16_ptr_1050_5f2e = extraout_DX;
    }
    local_a = CONCAT22(g_u16_ptr_1050_5f2e, uVar3);
    if ((g_u16_ptr_1050_5f2e | uVar3) != 0)
    {
        (iVar4 + 0x10) = _local_6;
        (iVar4 + 4) = local_a;
    }
    return;
}

pub fn pass1_1030_201e(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_1d28(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_2068(param_1: *mut astruct_854)

{
    let local_BX_19: *mut astruct_854;
    let local_BX_134: *mut astruct_855;
    let mut uVar1: u16;
    let mut local_4: u16;

    pass1_1030_17ce(param_1, 0, 0);
    uVar1 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    param_1 = s_fem92_wav_1050_293c;
    local_BX_19.field_0x2 = 0x1030;
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_19.field_0x10), 0, 0x106);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_19.field_0x116), 0, 0x86);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_19.field_0x19c), 0, 10);
    pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(local_BX_19 + 1)), 0, 0x106);
    local_4 = 0;
    while
    {
        local_BX_134 = (&local_BX_19.field_0x0 + local_4 * 2);
        local_BX_134.field_0x10 = 0xffff;
        local_BX_134.field_0x1a6 = 0x19;
        local_4 = local_4 + 1;
        local_4 < 0x83
    } {}
    return;
}

pub fn pass1_1030_2112(param_1: *mut astruct_848,param_2: u32)

{
    let local_BX_29: *mut astruct_856;
    let local_BX_56: *mut astruct_857;
    let mut uVar1: u16;
    let mut local_4: u16;

    pass1_1030_183c(param_1, 1, 1, 0x8000000, param_2);
    uVar1 = (param_1 >> 0x10);
    local_BX_29 = param_1;
    param_1.field_0x0 = s_fem92_wav_1050_293c;
    local_BX_29.field_0x2 = 0x1030;
    local_4 = 0;
    while
    {
        local_BX_56 = (&local_BX_29.field_0x0 + local_4 * 2);
        local_BX_56.field_0x10 = 0xffff;
        local_BX_56.field_0x1a6 = 0x19;
        local_4 = local_4 + 1;
        local_4 < 0x83
    } {}
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_29.field_0x116), 0, 0x86);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_29.field_0x19c), 0, 10);
    pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(local_BX_29 + 1)), 0, 0x106);
    local_BX_29.field_0x10 = 0;
    local_BX_29.field_0x14 = 0;
    local_BX_29.field_0x16 = 0;
    local_BX_29.field_0x20 = 0;
    local_BX_29.field_0x44 = 0;
    local_BX_29.field_0x50 = 0;
    local_BX_29.field_0x6a = 0;
    local_BX_29.field_0x84 = 0;
    local_BX_29.field_0xc8 = 0;
    local_BX_29.field_0xe4 = 0;
    local_BX_29.field_0xf0 = 0;
    local_BX_29.field_0xf4 = 0;
    local_BX_29.field_0xf6 = 0;
    local_BX_29.field_0x102 = 0;
    local_BX_29.field_0xfe = 0;
    local_BX_29.field_0x1a6 = 0;
    local_BX_29.field_0x1aa = 0;
    local_BX_29.field_0x1ac = 0;
    local_BX_29.field_0x1b6 = 0;
    local_BX_29.field_0x1da = 0;
    local_BX_29.field_0x1e6 = 0;
    local_BX_29.field_0x200 = 0;
    local_BX_29.field_0x21a = 0;
    local_BX_29.field_0x25e = 0;
    local_BX_29.field_0x27a = 0;
    local_BX_29.field_0x286 = 0;
    local_BX_29.field_0x28a = 0;
    local_BX_29.field_0x28c = 0;
    local_BX_29.field_0x298 = 0;
    local_BX_29.field_0x294 = 0;
    return;
}

pub fn pass1_1030_2242(param_1: *mut pass1_struct_2, param_2: u16)

{
    let mut uVar1: u16;
    let local_BX_4: *mut astruct_858;
    let paVar2: *mut astruct_858;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    paVar2 = &local_BX_4.field_0x10;
    if (-1 < (&paVar2.field_0x0 + param_2 * 2))
    {
        uVar1 = (&local_BX_4.field_0x10 + param_2 * 2);
        paVar2 = local_BX_4 + 1;
        if ((&paVar2.field_0x0 + param_2 * 2) <= uVar1)
        {
            return uVar1;
        }
    }
    return (&paVar2.field_0x0 + param_2 * 2);
}



pub fn pass1_1030_19f0(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_18b2(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_1a32(param_1: *mut astruct_848) -> *mut astruct_848

{
    pass1_1030_183c(param_1, 1, 0x16, 0xff000000, 0);
    PTR_LOOP_1050_5168 = (param_1 >> 0x10);
    PTR_LOOP_1050_5166 = param_1;
    (PTR_LOOP_1050_5166 + 0x10) = 0;
    param_1.field_0x0 = (s_288_flc_1050_1cb6 + 6);
    (PTR_LOOP_1050_5166 + 2) = 0x1030;
    return param_1;
}



pub fn pass1_1030_1a74(param_1: *mut astruct_850)

{
    param_1.field_0x0 = (s_288_flc_1050_1cb6 + 6);
    (param_1 + 2) = 0x1030;
    _PTR_LOOP_1050_5166 = 0;
    pass1_1030_18b2(param_1);
    return;
}


pub fn pass1_1030_177a(param_1: u32,param_2: u32)

{
    (param_1 + 8) = param_2;
    return;
}

pub fn ret_1030_178e()

{
    return;
}

pub fn pass1_1030_1794(param_1: *mut astruct_44, param_2: u8) -> *mut astruct_44

{
    pass1_1030_16b2(param_1);
    if ((param_2 & 1) != 0)
    {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_17ce(param_1: *mut astruct_393,param_2: u32,param_3: u32)

{
    let uVar1: u8;
    let extraout_AH: u8;
    let in_DX: *mut astruct_199;
    let local_BX_13: *mut astruct_849;
    let mut uvar3: u16;
    let mut uVar4: u32;
    let mut local_4: u16;
    let mut uVar2: i32;

    uVar1 = pass1_1030_1628(param_1);
    uVar2 = CONCAT11(extraout_AH, uVar1);
    uVar3 = (param_1 >> 0x10);
    local_BX_13 = param_1;
    &local_BX_13.field_0xc = 0;
    param_1.field_0x0 = (s_638_bmp_1050_1a11 + 5);
    local_BX_13.field_0x2 = 0x1030;
    if ((param_3 != 0) || (param_2 != 0))
    {
        process_struct_1000_179c(0x18, in_DX);
        if ((in_DX | uVar2) == 0)
        {
            uVar4 = 0;
        }
        else
        {
            uVar4 = pass1_1030_1cd8(CONCAT22(in_DX, uVar2), param_2, param_3);
        }
        local_BX_13.field_0xc = uVar4;
        local_BX_13.field_0xe = (uVar4 >> 0x10);
    }
    return param_1;
}


pub fn pass1_1030_183c(param_1: *mut astruct_848,param_2: u32,param_3: u32,param_4: u32,param_5: u32) -> *mut astruct_848

{
    let mut in_AX: i32;
    let in_DX: *mut astruct_199;
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut local_4: u16;

    pass1_1030_165e(param_1, param_4, param_5);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xc) = 0;
    param_1.field_0x0 = (s_638_bmp_1050_1a11 + 5);
    (iVar1 + 2) = 0x1030;
    if ((param_3 != 0) || (param_2 != 0))
    {
        process_struct_1000_179c(0x18, in_DX);
        if ((in_DX | in_AX) == 0)
        {
            uVar3 = 0;
        }
        else
        {
            uVar3 = pass1_1030_1cd8(CONCAT22(in_DX, in_AX), param_2, param_3);
        }
        (iVar1 + 0xc) = uVar3;
        (iVar1 + 0xe) = (uVar3 >> 0x10);
    }
    return param_1;
}

pub fn pass1_1030_18b2(param_1: *mut astruct_850)

{
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_5: *mut astruct_850;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1.field_0x0 = (s_638_bmp_1050_1a11 + 5);
    local_BX_5.field_0x2 = 0x1030;
    puVar1 = local_BX_5.field_0xc;
    uVar2 = local_BX_5.field_0xe;
    if ((uVar2 | puVar1) != 0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1030_16b2(param_1);
    return;
}



pub fn pass1_1030_18f0(param_1: u32, param_2: i32, param_3: i32)

{
    let ppcVar1: fn();
    let mut in_AX: u16;
    let mut uVar2: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: i32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0xc) != 0)
    {
        ppcVar1 = ((iVar3 + 0xc) + 0x10);
        (**ppcVar1)();
        _local_6 = CONCAT22(extraout_DX, in_AX);
        local_a = 0;
        while (local_a < _local_6)
        {
            ppcVar1 = ((iVar3 + 0xc) + 4);
            uVar2 = _local_6;
            (**ppcVar1)();
            if ((uVar2 == param_2) && (extraout_DX_00 == param_3))
            {
                ppcVar1 = ((iVar3 + 0xc) + 8);
                (**ppcVar1)();
            }
            local_a = local_a + 1;
        }
    }
    return;
}



pub fn pass1_1028_ebee(param_1: u32)

{
    let mut in_AX: i32;
    let in_DX: *mut astruct_199;
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut local_c: u16;

    process_struct_1000_179c(0x14, in_DX);
    if ((in_DX | in_AX) != 0)
    {
        pass1_1030_1a32(CONCAT22(in_DX, in_AX));
    }
    uVar3 = pass1_1030_4574((param_1 + 0x52));
    uVar2 = (_PTR_LOOP_1050_5166 >> 0x10);
    iVar1 = _PTR_LOOP_1050_5166;
    (iVar1 + 0x10) = uVar3;
    (iVar1 + 0x12) = (uVar3 >> 0x10);
    uVar2 = (_PTR_LOOP_1050_5166 >> 0x10);
    return CONCAT22((_PTR_LOOP_1050_5166 + 6),
                    (_PTR_LOOP_1050_5166 + 4));
}

pub fn pass1_1028_ec36(param_1: u32, param_2: u16, param_2: u16_00, param_4: u16,param_5: u32)

{
    let mut u_var1: u32;
    let mut in_AX: u16;
    let mut uVar2: u16;
    let in_DX: *mut astruct_199;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x14, in_DX);
    if ((in_DX | in_AX) == 0)
    {
        in_AX = 0;
        uVar3 = 0;
    }
    else
    {
        pass1_1030_5d3c(in_AX, in_DX, param_5);
        uVar3 = extraout_DX;
    }
    uVar5 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x52);
    uVar4 = uVar3;
    uVar2 = in_AX;
    pass1_1030_4594(uVar1, (uVar1 >> 0x10), param_2_00);
    pass1_1030_5fe2(CONCAT22(uVar3, in_AX), CONCAT22(uVar4, uVar2));
    pass1_1030_1358((param_1 + 0xe), in_AX, uVar3,
                    (in_AX + 4) & 0xffff | ( * (in_AX + 6) & 0xff) << 0x10);
    return;
}

pub fn pass1_1028_ecac(param_1: u32, param_2: u16, param_2_00: *mut u16, param_4: u16,param_5: u32)

{
    let mut u_var1: u32;
    let mut in_AX: u16;
    u16 **ppuVar2;
    let in_DX: *mut astruct_199;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x1c, in_DX);
    uVar3 = in_DX | in_AX;
    if (uVar3 == 0)
    {
        in_AX = 0;
        uVar3 = 0;
    }
    else
    {
        pass1_1030_299a(CONCAT22(in_DX, in_AX), param_5);
    }
    uVar5 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x52);
    uVar4 = uVar3;
    ppuVar2 = param_2_00;
    pass1_1030_4628(uVar1, (uVar1 >> 0x10), param_2_00);
    *ppuVar2 = param_2_00;
    pass1_1030_3006(CONCAT22(uVar3, in_AX), CONCAT22(uVar4, ppuVar2));
    pass1_1030_1358((param_1 + 0x12), in_AX, uVar3,
                    (in_AX + 4) & 0xffff | ( * (in_AX + 6) & 0xff) << 0x10);
    return;
}