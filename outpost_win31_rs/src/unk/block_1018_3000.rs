use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::unk::block_1008_8000::empty_1008_8fc4;
use crate::utils::CONCAT22;
use std::ptr;

pub unsafe fn pass1_1018_30ca(mut param_1: u16, param_2: *mut astruct_504, param_3: *mut c_char) {
    let mut uVar1: u16;
    let mut iVar3: *mut astruct_504;
    let mut uVar2: u16;

    uVar2 = (param_2 >> 0x10);
    iVar3 = param_2;
    fn_ptr_1000_17ce(*&iVar3.field390_0x186);
    uVar1 = str_op_1008_60e8(param_1, param_3);
    iVar3.field390_0x186 = uVar1;
    iVar3.field391_0x188 = param_1;
    return;
}

pub unsafe fn pass1_1018_30fc(mut param_1: *mut Struct57, mut param_2: u32, mut param_3: &mut u32) {
    let mut u16_var1: u16;
    let mut u32_var2: u32;
    let mut pu16_var3: *mut u16;
    let mut u16_var4: u16;
    let mut u16_var5: u16;
    let mut i16_var6: i16;
    let mut u16_var7: u16;
    // let mut in_register_0000000a: u16;
    let mut pstruct57_1: *mut Struct57;
    let mut pu32_var8: *mut u32;
    let mut i16_var9: i16;

    pstruct57_1 = param_1;
    param_3 = null_mut();
    u32_var2 = (param_2 + 0x17e);
    u16_var1 = (u32_var2 + 0xa);
    if u16_var1 != 0x0 {
        u16_var4 = u16_var1;
        mem_op_1000_179c(0x6, pstruct57_1);
        u16_var7 = pstruct57_1;
        pu32_var8 = CONCAT22(u16_var7, u16_var4);
        pstruct57_1 = (pstruct57_1 & 0xffff0000 | (u16_var7 | u16_var4));
        if ((u16_var7 | u16_var4) == 0) {
            *param_3 = null_mut();
        } else {
            *pu32_var8 = 0;
            (u16_var4 + 0x4) = 0;
            *param_3 = pu32_var8;
        }
        u16_var5 = u16_var1 * 0x2;
        mem_op_1000_179c(u16_var5, pstruct57_1);
        pu16_var3 = *param_3;
        *pu16_var3 = u16_var5;
        (pu16_var3 + 0x2) = pstruct57_1;
        (*param_3 + 0x4) = u16_var1;
        // for (iStack6 = 0; i16_var9 < uVar1; iStack6 += 1)
        for i16_var9 in 0..u16_var1 {
            i16_var6 = i16_var9;
            empty_1008_8fc4();
            (*param_3 + i16_var9 * 0x2) = (i16_var6 + 0x2e);
        }
    }
    return;
}

pub unsafe fn pass1_1018_31d0(param_1: *mut astruct_126) -> u16 {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if (((param_1 + 0x17e) != 0) && (uVar1 = (param_1 + 0x17e), (uVar1 + 0xa) != 0)) {
        return 0x1;
    }
    return 0x0;
}


pub unsafe fn pass1_1018_31fa(mut param_1: u16, mut param_2: u16, param_3: *mut astruct_126) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut iVar4: i16;
    let mut pstruct126_5: *mut astruct_126;
    let mut uVar5: u16;

    uVar5 = (param_3 >> 0x10);
    pstruct126_5 = param_3;
    pass1_1030_82f0(_u16_1050_5748, pstruct126_5.field373_0x17a);
    pstruct126_5.field374_0x17e = param_1;
    pstruct126_5.field375_0x180 = param_2;
    if (((param_2 | pstruct126_5.field374_0x17e) != 0) && (
        uVar2 = &pstruct126_5.field374_0x17e,
        iVar4 = (uVar2 + 0xa),
        iVar4 != 0,
    )) {
        pstruct126_5.field369_0x174 = 0;
        loop {
            if (iVar4 <= pstruct126_5.field369_0x174) {
                break;
            }
            iVar3 = pstruct126_5.field369_0x174;
            empty_1008_8fc4();
            pass1_1018_2e28(param_3);
            if (pstruct126_5.field370_0x176 == iVar3) {
                break;
            }
            piVar1 = &pstruct126_5.field369_0x174;
            *piVar1 = *piVar1 + 1;
        }
        if (iVar4 <= pstruct126_5.field369_0x174) {
            pstruct126_5.field369_0x174 = 0;
        }
        pass1_1018_2e28(param_3);
        pstruct126_5.field370_0x176 = iVar4;
    }
    return;
}



pub unsafe fn pass1_1018_331c(
    param_1: *mut u8,
    param_2: *mut Struct638,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut unaff_BP: u16;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    pass1_1008_ca5a(CONCAT22(param_3, param_2), param_4);
    param_2.field271_0x122 = 0;
    param_2.field273_0x126 = 0;
    param_2.field274_0x12a = 0;
    param_2.field275_0x12e = 0;
    param_2.field276_0x130 = 0;
    param_2.field277_0x132 = 0;
    param_2.field278_0x136 = 0;
    param_2.field279_0x13a = 0;
    param_2.field280_0x13c = 0;
    param_2.field281_0x13e = 0;
    param_2.field282_0x142 = 0;
    CONCAT22(param_3, param_2) = &PTR_LOOP_1050_470c;
    param_2.field2_0x2 = 0x1018;
    puVar3 = mixed_1010_20ba(
        paVar2,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x3b),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    uVar1 = puVar3;
    param_2.field271_0x122 = uVar1;
    param_2.field272_0x124 = (puVar3 >> 0x10);
    param_2.field_0x22 = 0;
    pass1_1008_612e(uVar1, 0x8, 0xc);
    param_2.field280_0x13c = uVar1;
    return;
}

pub unsafe fn pass1_1018_33b4(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar5: *mut astruct_455;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.field0_0x0 = &PTR_LOOP_1050_470c;
    iVar5.field1_0x2 = 0x1018;
    puVar1 = iVar5[0x26].field3_0x6;
    uVar2 = (iVar5 + 0x27).field0_0x0;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    iVar5[0x26].field3_0x6 = 0;
    fn_ptr_1000_17ce(*&iVar5[0x24].field3_0x6);
    fn_ptr_1000_17ce(*&iVar5[0x25].field1_0x2);
    pass1_1008_caa0(param_1);
    return;
}


pub unsafe fn pass1_1018_3424(mut param_1: i16, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uStack10: u32;
    let mut uStack6: u32;

    uVar4 = (param_3 >> 0x10);
    iVar3 = param_3;
    uVar1 = (iVar3 + 0x122);
    pass1_1008_e852(param_2, uVar1, (uVar1 >> 0x10), *(iVar3 + 0x126));
    uStack6 = CONCAT22(param_2, param_1);
    uVar1 = (iVar3 + 0x122);
    pass1_1008_e852(param_2, uVar1, (uVar1 >> 0x10), *(iVar3 + 0x12a));
    uStack10 = CONCAT22(param_2, param_1);
    pass1_1030_8344(_u16_1050_5748, uStack6);
    uVar2 = param_2;
    iVar3 = param_1;
    pass1_1030_8344(_u16_1050_5748, uStack10);
    if ((iVar3 + 0x200) == (param_1 + 0x200)) {
        return;
    }
    return;
}

pub unsafe fn pass1_1018_34a6(param_1: *mut astruct_679) {
    pass1_1018_3d6c(param_1);
    return;
}


pub unsafe fn sprintf_op_1018_34b6(param_1: u8, mut param_2: u16, param_3: *mut astruct_263) {
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut pcVar3: *mut c_char;
    let mut in_register_00000001: u32;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut in_buf_len_5: i16;
    let mut uVar7: u32;
    let mut lVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;

    in_buf_len_5 = (param_3 >> 0x10);
    iVar6 = param_3;
    uVar7 = switch_1018_3b9e(
        CONCAT31(in_register_00000001, param_1),
        param_2,
        param_3,
        (iVar6 + 0x12e),
    );
    uVar4 = param_3 & 0xffff0000 | (iVar6 + 0x22);
    iVar1 = (iVar6 + 0x12e);
    if (iVar1 == 0x188) {
        lVar8 = pass1_1008_57f0(uVar7, (iVar6 + 0x130));
        uVar5 = (lVar8 >> 0x10);
        pcVar3 = string_1020_c0d8((lVar8 + 0xe));
        uVar2 = (iVar6 + 0x132);
        uVar10 = uVar2;
        uVar11 = (uVar2 >> 0x10);
        uVar9 = SUB42(s__ld__s_1050_4150, 0x0);
    } else if (iVar1 == 0x18b) {
        lVar8 = pass1_1008_57f0(uVar7, (iVar6 + 0x130));
        uVar2 = (lVar8 + 0x4);
        pcVar3 = uVar2;
        uVar5 = (uVar2 >> 0x10);
        uVar2 = (iVar6 + 0x132);
        uVar10 = uVar2;
        uVar11 = (uVar2 >> 0x10);
        uVar9 = SUB42(s__ld__s_1050_415e, 0x0);
    } else {
        if (iVar1 != 0x18c) {
            load_string_1010_84e0(
                _u16_1050_14cc,
                (_u16_1050_14cc >> 0x10),
                0x100,
                (iVar6 + 0x22),
                in_buf_len_5,
            );
            return;
        }
        lVar8 = pass1_1008_57f0(uVar7, (iVar6 + 0x130));
        uVar2 = (lVar8 + 0x4);
        pcVar3 = uVar2;
        uVar5 = (uVar2 >> 0x10);
        uVar2 = (iVar6 + 0x132);
        uVar10 = uVar2;
        uVar11 = (uVar2 >> 0x10);
        uVar9 = SUB42(s__ld__s_1050_4157, 0x0);
    }
    wsprintf16(
        (iVar6 + 0x22),
        CONCAT22(uVar9, in_buf_len_5),
        CONCAT22(uVar10, 0x1050),
        uVar11,
        pcVar3,
        uVar5,
        lVar8,
        uVar4,
    );
    return;
}



pub unsafe fn unk_str_op_1018_35b0(mut param_1: u16, param_2: *mut astruct_263) {
    let mut plVar1: *mut i32;
    let mut piVar2: *mut i16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut ppcVar6: *mut *mut code;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut extraout_DX: u16;
    let mut pstruct263_1: *mut astruct_263;
    let mut iVar10: i16;
    let mut pstruct263_2: *mut astruct_263;
    let mut uVar11: u16;
    let mut bVar12: bool;
    let mut uVar13: u32;
    let mut uVar14: u32;
    let mut local_12: i16;
    let mut local_10: i16;
    let mut lStack14: i32;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    uVar13 = pass1_1030_8326();
    uStack4 = (uVar13 >> 0x10);
    uStack6 = uVar13;
    pstruct263_2 = (param_2 >> 0x10);
    pstruct263_1 = param_2;
    plVar1 = &pstruct263_1[0x1].field15_0x16;
    bVar12 = plVar1 < uStack4;
    if ((bVar12) || (bVar12 || plVar1 == uStack4 && (pstruct263_1[0x1].field14_0x14 < uStack6))) {
        uVar3 = pstruct263_1[0x1].field13_0x12;
        if (&pstruct263_1[0x1].field_0x10 < uVar3) {
            uVar14 = switch_1018_3b9e(uVar3, uStack4, param_2, pstruct263_1[0x1].field4_0x4);
            uVar8 = (uVar14 >> 0x10);
            uVar11 = uVar14;
            uStack10 = uVar11;
            uStack8 = uVar8;
            pass1_1018_427c(param_2, uVar11, uVar8);
            lStack14 = CONCAT22(uVar8, uVar11);
            pass1_1018_3e8c(
                pstruct263_1,
                pstruct263_2,
                CONCAT22(0x1050, &local_12),
                CONCAT22(0x1050, &local_10),
            );
            if (lStack14 < local_12) {
                local_12 = lStack14;
            }
            uVar4 = pstruct263_1[0x1].field10_0xe;
            puVar7 = &pstruct263_1[0x1].field9_0xc;
            uVar9 = uVar4 | puVar7;
            if (uVar9 != 0) {
                ppcVar6 = *puVar7;
                (**ppcVar6)(0x30, puVar7, uVar4, 1);
                uVar9 = extraout_DX;
            }
            pass1_1018_435e(uVar9, param_2, lStack14, local_12, local_10);
            pstruct263_1[0x1].field9_0xc = puVar7;
            pstruct263_1[0x1].field10_0xe = uVar9;
            piVar2 = &pstruct263_1[0x1].field_0x10;
            *piVar2 = *piVar2 + 1;
            uVar14 = &pstruct263_1[0x1].field9_0xc;
            uVar11 = (uVar14 >> 0x10);
            iVar10 = uVar14;
            uVar14 = (iVar10 + 0x4);
            uVar5 = (iVar10 + 0x8);
            wsprintf16(
                &pstruct263_1.field_0x22,
                CONCAT22(0x4165, pstruct263_2),
                0x50,
                CONCAT13((uVar5 >> 0x8), CONCAT12(uVar5, 0x1050)),
                uVar5,
                (uVar5 >> 0x10),
                uVar14,
                (uVar14 >> 0x10),
            );
            return;
        }
        pstruct263_1[0x1].field14_0x14 = uStack6;
        pstruct263_1[0x1].field15_0x16 = uStack4;
        pstruct263_1[0x1].field_0x10 = 0;
        pass1_1008_612e(uStack6, 0x8, 0xc);
        pstruct263_1[0x1].field13_0x12 = uStack6;
    }
    return;
}

pub unsafe fn pass1_1018_36e6(
    mut param_1: u32,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x12e) = param_4;
    (iVar1 + 0x130) = param_3;
    (iVar1 + 0x132) = param_2;
    (iVar1 + 0x134) = 0;
    return;
}



pub unsafe fn pass1_1018_3710(mut param_1: u16, mut param_2: u16, param_3: *mut astruct_263) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut paVar7: *mut Struct57;
    let mut iVar8: *mut astruct_263;
    let mut uVar8: u16;
    let mut lVar9: i32;
    let mut local_12a: [u8; 0x118] = [0; 0x118];
    let mut uStack18: u32;
    let mut paStack14: *mut astruct_203;
    let mut uStack10: u32;
    let mut paStack6: *mut astruct_203;

    paStack6 = null_mut();
    uVar8 = (param_3 >> 0x10);
    iVar8 = param_3;
    uStack10 = switch_1018_3b9e(param_1, param_2, param_3, iVar8[0x1].field4_0x4);
    uVar4 = iVar8[0x1].field4_0x4 - 0x188;
    uStack18 = (uStack10 & 0xffff0000 | uVar4);
    match uVar4 {
        0x0 => {
            lVar9 = pass1_1008_57f0(uStack10, iVar8[0x1].field5_0x6);
            uVar3 = (lVar9 >> 0x10);
            paVar7 = CONCAT22(in_register_0000000a, uVar3);
            uVar4 = lVar9;
            mem_op_1000_179c(0x10, paVar7);
            uVar6 = paVar7 | uVar4;
            if (uVar6 != 0) {
                uVar3 = struct_1018_4790(
                    CONCAT22(paVar7, uVar4),
                    &iVar8[0x1].field_0x8,
                    0x0,
                    (lVar9 + 0xe),
                );
                uStack18 = CONCAT22(uVar6, uVar3);
                paStack6 = uStack18;
                // TODO: goto switchD_1018_393f_caseD_6;
            }
        }
        // break;
        0x1 => {
            lVar9 = pass1_1008_57f0(uStack10, iVar8[0x1].field5_0x6);
            uVar3 = (lVar9 >> 0x10);
            paVar7 = CONCAT22(in_register_0000000a, uVar3);
            uVar6 = lVar9;
            uVar4 = uVar6;
            mem_op_1000_179c(0x14, paVar7);
            uVar5 = paVar7 | uVar4;
            if (uVar5 != 0) {
                struct_1018_47c8(
                    CONCAT22(paVar7, uVar4),
                    &iVar8[0x1].field_0x8,
                    0x0,
                    (uVar6 + 0x12),
                    (uVar6 + 0xe),
                );
                uStack18 = CONCAT22(uVar5, uVar4);
                paStack6 = uStack18;
                // TODO: goto switchD_1018_393f_caseD_6;
            }
        }
        // break;
        0x2 => {
            lVar9 = pass1_1008_57f0(uStack10, iVar8[0x1].field5_0x6);
            uVar3 = (lVar9 >> 0x10);
            paVar7 = CONCAT22(in_register_0000000a, uVar3);
            uVar4 = lVar9;
            mem_op_1000_179c(0x12, paVar7);
            uVar6 = paVar7 | uVar4;
            if (uVar6 != 0) {
                pass1_1018_4808(
                    CONCAT22(paVar7, uVar4),
                    &iVar8[0x1].field_0x8,
                    0x0,
                    (lVar9 + 0xe),
                );
                uStack18 = CONCAT22(uVar6, uVar4);
                paStack6 = uStack18;
                // TODO: goto switchD_1018_393f_caseD_6;
            }
        }
        // break;
        0x3 => {
            lVar9 = pass1_1008_57f0(uStack10, iVar8[0x1].field5_0x6);
            uVar3 = (lVar9 >> 0x10);
            paVar7 = CONCAT22(in_register_0000000a, uVar3);
            uVar4 = lVar9;
            mem_op_1000_179c(0x14, paVar7);
            if ((paVar7 | uVar4) != 0) {
                uStack18 = struct_1018_4842(
                    CONCAT22(paVar7, uVar4),
                    &iVar8[0x1].field_0x8,
                    0x0,
                    (lVar9 + 0xe),
                );
                paStack6 = uStack18;
                // TODO: goto switchD_1018_393f_caseD_6;
            }
        }
        // break;
        0x4 => {
            lVar9 = pass1_1008_57f0(uStack10, iVar8[0x1].field5_0x6);
            uVar3 = (lVar9 >> 0x10);
            paVar7 = CONCAT22(in_register_0000000a, uVar3);
            uVar4 = lVar9;
            mem_op_1000_179c(0x10, paVar7);
            if ((paVar7 | uVar4) != 0) {
                uStack18 = struct_1018_48b0(
                    CONCAT22(paVar7, uVar4),
                    &iVar8[0x1].field_0x8,
                    0x0,
                    (lVar9 + 0xe),
                );
                paStack6 = uStack18;
                // TODO: goto switchD_1018_393f_caseD_6;
            }
        }
        // break;
        0x5 => {
            lVar9 = pass1_1008_57f0(uStack10, iVar8[0x1].field5_0x6);
            uVar3 = (lVar9 >> 0x10);
            paVar7 = CONCAT22(in_register_0000000a, uVar3);
            uVar4 = lVar9;
            mem_op_1000_179c(0x12, paVar7);
            uVar6 = paVar7 | uVar4;
            if (uVar6 != 0) {
                uVar3 = struct_1018_4920(
                    CONCAT22(paVar7, uVar4),
                    &iVar8[0x1].field_0x8,
                    0x0,
                    (lVar9 + 0xe),
                );
                uStack18 = CONCAT22(uVar6, uVar3);
                paStack6 = uStack18;
            }
        }
        // break;
        _ => {} // TODO: goto switchD_1018_393f_caseD_6;
    };
    uStack18 = null_mut();
    paStack6 = uStack18;
    // switchD_1018_393f_caseD_6:
    uVar1 = iVar8.field274_0x122;
    pass1_1008_e852(
        (uStack18 >> 0x10),
        uVar1,
        (uVar1 >> 0x10),
        iVar8.field275_0x126,
    );
    uVar1 = iVar8.field274_0x122;
    paStack14 = uStack18;
    pass1_1008_e852((uStack18 >> 0x10), uVar1, (uVar1 >> 0x10), *(iVar8 + 1));
    pass1_1038_2a0e(
        CONCAT22(0x1050, local_12a),
        &iVar8[0x1].field9_0xc,
        paStack6,
        uStack18,
        paStack14,
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_12a));
    iVar8[0x1].field9_0xc = 0;
    ppcVar2 = (param_3 + 0x10);
    (**ppcVar2)(0x1030, param_3);
    pass1_1038_2a5c(CONCAT22(0x1050, local_12a));
    return;
}



pub unsafe fn string_1018_39d8(
    mut param_1: u32,
    param_2: *mut c_char,
    param_3: *mut c_char,
) -> BOOL16 {
    let mut iVar1: i16;
    let mut pcVar2: *mut c_char;
    let mut lVar3: i32;
    let mut pcVar4: *mut c_char;

    pcVar4 = param_2;
    pcVar2 = load_string_1010_847e(_u16_1050_14cc, 0x531);
    iVar1 = pass1_1000_3d7a(pcVar2, pcVar4);
    if (iVar1 != 0) {
        iVar1 = pass1_1000_3d7a(param_3, param_2);
        if (iVar1 != 0) {
            lVar3 = pass1_1018_4608(param_1, param_2, param_3);
            if ((lVar3 != 0) && ((lVar3 + 0xc) == 1)) {
                return 0x1;
            }
        }
    }
    return 0x0;
}

pub unsafe fn pass1_1018_3a42(mut param_1: u16, mut param_2: u32, mut param_3: u32) {
    let mut uVar1: u32;

    uVar1 = (param_2 + 0x122);
    pass1_1008_e852(param_1, uVar1, (uVar1 >> 0x10), param_3);
    return;
}

pub unsafe fn pass1_1018_3a5c(mut param_1: u32, param_2: *mut c_char, param_3: *mut c_char) {
    pass1_1008_e320((param_1 + 0x122), param_2, param_3);
    return;
}

pub unsafe fn pass1_1018_3a7a(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u32,
) -> u32 {
    let mut uVar1: u32;
    let mut uVar2: u32;

    uVar1 = (param_3 + 0x122);
    uVar2 = string_1008_e586(uVar1, (uVar1 >> 0x10), param_4, param_1, param_2);
    return uVar2;
}

pub unsafe fn pass1_1018_3a94(mut param_1: u32, param_2: *mut u32, param_3: *mut u32) {
    pass1_1008_e3ec((param_1 + 0x122), param_2, param_3);
    return;
}

pub unsafe fn pass1_1018_3ab2(mut param_1: u32, mut param_2: i16, mut param_3: i16) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut lVar4: i32;
    let mut uStack22: u16;
    let mut local_10: [u8; 0x8] = [0; 0x8];
    let mut iStack8: i16;
    let mut uStack6: u32;

    if (0x5 < param_3 - 0x188) {
        return 0x0;
    }
    iVar3 = param_1;
    uVar2 = (param_1 >> 0x10);
    match param_3 {
        0x188 => {
            uVar1 = (iVar3 + 0xa);
            uVar2 = (iVar3 + 0xc);
        }
        // break;
        0x189 => {
            uVar1 = (iVar3 + 0xe);
            uVar2 = (iVar3 + 0x10);
        }
        // break;
        0x18a => {
            uVar1 = (iVar3 + 0x12);
            uVar2 = (iVar3 + 0x14);
        }
        // break;
        0x18b => {
            uVar1 = (iVar3 + 0x16);
            uVar2 = (iVar3 + 0x18);
        }
        // break;
        0x18c => {
            uVar1 = (iVar3 + 0x1a);
            uVar2 = (iVar3 + 0x1c);
        }
        // break;
        0x18d => {
            uVar1 = (iVar3 + 0x1e);
            uVar2 = (iVar3 + 0x20);
        }
    };
    uStack6 = CONCAT22(uVar2, uVar1);
    iStack8 = 0;
    pass1_1008_5784(CONCAT22(0x1050, local_10), uStack6);
    loop {
        lVar4 = pass1_1008_5b12(CONCAT22(0x1050, local_10));
        uVar2 = (lVar4 >> 0x10);
        if ((lVar4 == 0) || (iStack8 == param_2)) {
            break;
        }
        iStack8 += 0x1;
    }
    uStack22 = 0;
    if (lVar4 != 0) {
        if ((lVar4 + 0xa) == 0) {
            uStack22 = (lVar4 + 0x8);
        } else {
            uStack22 = 0xffff;
        }
    }
    return uStack22;
}



pub unsafe fn switch_1018_3b9e(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_263,
    mut param_4: u16,
) -> u32 {
    let mut uVar1: u32;
    let mut pstruct_1_1: *mut astruct_263;
    let mut pstruct_1_2: *mut astruct_263;
    let mut paStack14: *mut astruct_6;
    let mut uStack6: u16;
    let mut uStack4: u16;

    uStack6 = 0;
    uStack4 = 0;
    pstruct_1_2 = (param_3 >> 0x10);
    pstruct_1_1 = param_3;
    uVar1 = pstruct_1_1.field274_0x122;
    pass1_1008_e852(param_2, uVar1, (uVar1 >> 0x10), pstruct_1_1.field275_0x126);
    pass1_1030_8344(_u16_1050_5748, CONCAT22(param_2, param_1));
    paStack14 = CONCAT22(param_2, param_1);
    match param_4 {
        0x188 => {
            if (&pstruct_1_1.field8_0xa == 0) {
                pass1_1008_d3ae((param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10));
            }
            uStack6 = pstruct_1_1.field8_0xa;
            uStack4 = pstruct_1_1.field9_0xc;
        }
        // break;
        0x189 => {
            if (&pstruct_1_1.field10_0xe == 0) {
                unk_str_op_1008_d4f6((param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10), paStack14);
            }
            uStack6 = pstruct_1_1.field10_0xe;
            uStack4 = &pstruct_1_1.field_0x10;
        }
        // break;
        0x18a => {
            if (&pstruct_1_1.field13_0x12 == 0) {
                unk_str_op_1008_d1c6((param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10), paStack14);
            }
            uStack6 = pstruct_1_1.field13_0x12;
            uStack4 = pstruct_1_1.field14_0x14;
        }
        // break;
        0x18b => {
            if (pstruct_1_1.field15_0x16 == 0) {
                pass1_1008_cfa0((param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10), paStack14);
            }
            uStack6 = &pstruct_1_1.field15_0x16;
            uStack4 = (&pstruct_1_1.field15_0x16 + 2);
        }
        // break;
        0x18c => {
            if (pstruct_1_1.field16_0x1a == 0) {
                pass1_1008_cda2((param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10), paStack14);
            }
            uStack6 = &pstruct_1_1.field16_0x1a;
            uStack4 = (&pstruct_1_1.field16_0x1a + 2);
        }
        // break;
        0x18d => {
            if (pstruct_1_1.field17_0x1e == 0) {
                pass1_1008_cbc4((param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10), paStack14);
            }
            uStack6 = &pstruct_1_1.field17_0x1e;
            uStack4 = (&pstruct_1_1.field17_0x1e + 2);
        }
    };
    return CONCAT22(uStack4, uStack6);
}

pub unsafe fn pass1_1018_3cda(
    param_1: *mut astruct_506,
    param_2: *mut c_char,
    param_3: *mut c_char,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut extraout_DX: u16;
    let mut uVar3: u16;
    let mut iVar5: *mut astruct_506;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar5 = param_1;
    ppcVar1 = (param_1 + 0x10);
    (**ppcVar1)();
    uVar3 = extraout_DX;
    fn_ptr_1000_17ce(*&iVar5.field294_0x126);
    fn_ptr_1000_17ce(*&iVar5.field296_0x12a);
    uVar2 = str_op_1008_60e8(uVar3, param_3);
    iVar5.field294_0x126 = uVar2;
    iVar5.field295_0x128 = uVar3;
    uVar2 = str_op_1008_60e8(uVar3, param_2);
    iVar5.field296_0x12a = uVar2;
    iVar5.field297_0x12c = uVar3;
    return;
}

pub unsafe fn pass1_1018_3d44(mut param_1: u32, param_2: *mut u32, param_3: *mut u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x126);
    *param_2 = (param_1 + 0x12a);
    return;
}


pub unsafe fn pass1_1018_3d6c(param_1: *mut astruct_679) {
    let mut bVar1: u8;
    let mut uVar2: u16;
    let mut puVar3: *mut u8;
    let mut uVar4: u16;
    let mut iVar6: *mut astruct_679;
    let mut uVar5: *mut astruct_679;
    let mut uVar6: u32;
    let mut uVar7: u32;

    uVar5 = (param_1 >> 0x10);
    iVar6 = param_1;
    uVar4 = iVar6.field322_0x142;
    uVar2 = uVar4 + 0x1e;
    if (iVar6.field323_0x144 + 0x1 == (uVar4 < 0xffe2)) {
        if (uVar2 != 0x3c) {
            if (0x3c < uVar2) {
                return;
            }
            bVar1 = uVar2;
            if (bVar1 == 0x14) {
                iVar6.field322_0x142 = 0xffec; //
                // LAB_1018_3e3d:
                iVar6.field323_0x144 = -0x1;
                return;
            }
            if (0x14 < bVar1) {
                if (bVar1 == 0x1e) {
                    if (PTR_LOOP_1050_13ae < 1) {
                        return;
                    }
                    if (SBORROW2(PTR_LOOP_1050_13ae, 1)) {
                        return;
                    }
                    if (PTR_LOOP_1050_13ae != &u16_1050_0002 && 0x0 < (PTR_LOOP_1050_13ae - 1)) {
                        puVar3 = PTR_LOOP_1050_13ae - 0x3;
                        if (puVar3.is_null()) {
                            pass1_1008_612e(0x0, 0x1, 0x64);
                            if (puVar3 < 0x32) {
                                uVar4 = 0xa;
                            } else {
                                uVar4 = 0xfff6;
                            }
                            iVar6.field322_0x142 = uVar4;
                            iVar6.field323_0x144 = uVar4 >> 0xf;
                            return;
                        }
                        if (puVar3 != (&PTR_LOOP_1050_0000 + 1)) {
                            return;
                        }
                        iVar6.field322_0x142 = 0xfff6;
                        // TODO: goto LAB_1018_3e3d;
                    }
                    iVar6.field322_0x142 = 0xa;
                } else if (bVar1 == 0x28) {
                    iVar6.field322_0x142 = 0x14;
                } else {
                    if (bVar1 != 0x32) {
                        return;
                    }
                    iVar6.field322_0x142 = 0x1e;
                }
                iVar6.field323_0x144 = 0;
                return;
            }
            if (bVar1 != 0) {
                if (bVar1 != 0xa) {
                    return;
                }
                iVar6.field322_0x142 = 0xffe2;
                // TODO: goto LAB_1018_3e3d;
            }
        }
        uVar7 = 0x5;
        uVar6 = pass1_1030_8326();
        if (uVar6 % uVar7 == 0) {
            iVar6.field322_0x142 = 0;
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1018_3e8c(
    param_1: *mut astruct_263,
    param_2: *mut astruct_263,
    param_3: *mut u16,
    param_4: *mut u16,
) {
    *param_4 = 0x1;
    *param_3 = 0x19;
    return;
}



pub unsafe fn switch_1018_3ee6(
    mut param_1: u16,
    mut param_2: u32,
    param_3: i32,
    mut param_4: i16,
    mut param_5: u16,
) {
    let mut iVar1: i16;
    let mut pcVar2: *mut c_char;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut paVar9: *mut astruct_203;
    let mut lVar10: i32;
    let mut iVar11: i16;
    let mut paVar12: *mut astruct_263;
    let mut IVar13: i16;
    let mut paVar14: *mut astruct_263;
    let mut uVar15: u16;
    let mut uStack26: u32;
    let mut paStack22: *mut astruct_203;
    let mut lStack18: i32;
    let mut lStack14: i32;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut piStack6: *mut i16;

    paVar8 = CONCAT22(in_register_0000000a, param_1);
    match param_5 {
        0x1 => {
            iVar1 = param_4 * 0x4 + 0x40b6;
        }
        // break;
        _ => {
            iVar1 = param_4 * 0x4 + 0x40ce;
        }
        // break;
        0x3 => {
            iVar1 = param_4 * 0x4 + 0x40e2;
        }
        // break;
        0x4 => {
            iVar1 = param_4 * 0x4 + 0x40ee;
        }
        // break;
        0x8 => {
            iVar1 = param_4 * 0x4 + 0x40f2;
        }
        // break;
        0x9 => {
            iVar1 = param_4 * 0x4 + 0x4106;
        }
        // break;
        0xa => {
            iVar1 = param_4 * 0x4 + 0x410a;
        }
        // break;
        0x14 => {
            iVar1 = param_4 * 0x4 + 0x410e;
        }
        // break;
        0x16 => {
            iVar1 = param_4 * 0x4 + 0x4112;
        }
        // break;
        0x17 => {
            iVar1 = param_4 * 0x4 + 0x4116;
        }
        // break;
        0x19 => {
            iVar1 = param_4 * 0x4 + 0x411a;
        }
    };
    piStack6 = CONCAT22(0x1050, iVar1);
    if (piStack6.is_null()) {
        return;
    }
    iStack10 = 0;
    uStack8 = 0;
    iVar11 = *piStack6;
    paVar12 = param_2;
    paVar14 = (param_2 >> 0x10);
    if (iVar11 == 1) {
        uVar15 = pass1_1018_456a(paVar12, paVar14, (iVar1 + 0x2));
        lStack14 = CONCAT22(paVar8, uVar15);
        pcVar2 = string_1020_c0d8((iVar1 + 0x2));
        uVar3 = str_op_1008_60e8(paVar8, CONCAT22(paVar8, pcVar2));
        uVar7 = SUB42(paVar8, 0x0);
        uVar15 = uVar3;
        mem_op_1000_179c(0x10, paVar8);
        paStack22 = CONCAT22(paVar8, uVar15);
        if ((paVar8 | uVar15) != 0) {
            lVar10 = param_3 / lStack14;
            uStack8 = (param_3 % lStack14);
            struct_1018_4790(paStack22, lVar10, CONCAT22(uVar7, uVar3), (iVar1 + 0x2));
            iStack10 = lVar10;
            // TODO: goto LAB_1018_425e;
        }
    } else if (iVar11 == 0x2) {
        uVar15 = pass1_1018_451e(paVar12, paVar14, (iVar1 + 0x2));
        lStack18 = CONCAT22(paVar8, uVar15);
        pcVar2 = string_op_1020_c222((iVar1 + 0x2));
        uVar3 = str_op_1008_60e8(paVar8, CONCAT22(paVar8, pcVar2));
        uVar7 = SUB42(paVar8, 0x0);
        uVar15 = uVar3;
        mem_op_1000_179c(0x10, paVar8);
        paStack22 = CONCAT22(paVar8, uVar15);
        if ((paVar8 | uVar15) != 0) {
            paVar9 = struct_1018_48b0(
                paStack22,
                param_3 / lStack18,
                CONCAT22(uVar7, uVar3),
                (iVar1 + 0x2),
            );
            uStack8 = (paVar9 >> 0x10);
            iStack10 = paVar9;
            // TODO: goto LAB_1018_425e;
        }
    } else if (iVar11 == 0x3) {
        uVar15 = pass1_1008_c646(
            _u16_1050_06e0,
            CONCAT22((iVar1 + 0x2), (_u16_1050_06e0 >> 0x10)),
        );
        if (uVar15 == 0) {
            uVar15 = 0x4f;
        }
        uVar3 = switch_1018_43ec(paVar12, paVar14, uVar15);
        lStack14 = CONCAT22(paVar8, uVar3);
        uVar3 = pass1_1020_bd80(uVar15);
        uVar4 = str_op_1008_60e8(paVar8, CONCAT22(paVar8, uVar3));
        uStack26 = CONCAT22(paVar8, uVar4);
        mem_op_1000_179c(0x14, paVar8);
        paStack22 = CONCAT22(paVar8, uVar4);
        if ((paVar8 | uVar4) != 0) {
            uVar6 = param_3 / lStack14;
            uStack8 = (param_3 % lStack14);
            struct_1018_47c8(paStack22, uVar6, uStack26, uVar15, 0x0);
            iStack10 = uVar6;
            // TODO: goto LAB_1018_425e;
        }
    } else {
        //    if (iVar11 != 0x4) goto LAB_1018_425e;
        iVar1 = (iVar1 + 2);
        uVar4 = iVar1 - 0x1;
        iVar11 = _u16_1050_14cc;
        IVar13 = (_u16_1050_14cc >> 0x10);
        if (uVar4 == 0) {
            load_string_1010_84ac(iVar11, IVar13, 0x430);
            uVar7 = SUB42(paVar8, 0x0);
            uVar5 = uVar4;
            mem_op_1000_179c(0x14, paVar8);
            paStack22 = CONCAT22(paVar8, uVar5);
            if ((paVar8 | uVar5) != 0) {
                uVar15 = 0x2;
                lVar10 = 0x14; //
                // LAB_1018_4230:
                paVar9 = struct_1018_4842(paStack22, param_3 / lVar10, CONCAT22(uVar7, uVar4), uVar15);
                uStack8 = (paVar9 >> 0x10);
                iStack10 = paVar9;
                // TODO: goto LAB_1018_425e;
            }
        } else {
            uVar4 = iVar1 - 0x2;
            if (uVar4 == 0) {
                load_string_1010_84ac(iVar11, IVar13, 0x431);
                uVar7 = SUB42(paVar8, 0x0);
                uVar5 = uVar4;
                mem_op_1000_179c(0x14, paVar8);
                paStack22 = CONCAT22(paVar8, uVar5);
                if ((paVar8 | uVar5) != 0) {
                    uVar15 = 0x3;
                    lVar10 = 0x16;
                    // TODO: goto LAB_1018_4230;
                }
            } else {
                uVar4 = iVar1 - 0x3;
                if (uVar4 == 0) {
                    load_string_1010_84ac(iVar11, IVar13, 0x432);
                    uVar7 = SUB42(paVar8, 0x0);
                    uVar5 = uVar4;
                    mem_op_1000_179c(0x14, paVar8);
                    paStack22 = CONCAT22(paVar8, uVar5);
                    if ((paVar8 | uVar5) != 0) {
                        uVar15 = 0x4;
                        lVar10 = 0x17;
                        // TODO: goto LAB_1018_4230;
                    }
                } else {
                    uVar4 = iVar1 - 0x4;
                    //          if (uVar4 != 0) goto LAB_1018_425e;
                    load_string_1010_84ac(iVar11, IVar13, 0x433);
                    uVar7 = SUB42(paVar8, 0x0);
                    uVar5 = uVar4;
                    mem_op_1000_179c(0x14, paVar8);
                    paStack22 = CONCAT22(paVar8, uVar5);
                    if ((paVar8 | uVar5) != 0) {
                        uVar15 = 0x4;
                        lVar10 = 0xa;
                        // TODO: goto LAB_1018_4230;
                    }
                }
            }
        }
    }
    iStack10 = 0;
    uStack8 = 0; //
    // LAB_1018_425e:
    if ((iStack10 + 0x8) == 0) {
        (iStack10 + 0x8) = 0x1;
    }
    return;
}
