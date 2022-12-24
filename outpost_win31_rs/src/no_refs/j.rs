use crate::block_1010::block_1010_7000::pass1_1010_7d7e;
use crate::block_1010::block_1010_8000::{pass1_1010_866c, pass1_1010_878c, pass1_1010_887a, pass1_1010_89f0};
use crate::sys_ops::set_err_mode_1010_8b14;


pub unsafe fn pass1_1010_7818(mut param_1: u32, param_2: *mut astruct_15) -> u16 {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut uStack6: u16;

    uVar4 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x14);
    uVar2 = pass1_1010_b028(uVar1, (uVar1 >> 0x10), param_2);
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x1e);
    if (BVar3 == 0) {
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0xb);
        if (((BVar3 == 0) && (
            BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x20),
            BVar3 == 0,
        )) && (
            BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x1c),
            BVar3 == 0,
        )) {
            BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x2);
            if ((BVar3 != 0) || (
                BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x13),
                BVar3 != 0,
            )) {
                return 0x5;
            }
            BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x11);
            if ((BVar3 != 0) || (
                BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x12),
                BVar3 != 0,
            )) {
                return 0x4;
            }
            BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x5);
            if (BVar3 != 0) {
                return 0x6;
            }
            BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x6);
            if (BVar3 != 0) {
                return 0x7;
            }
            BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x4);
            if (BVar3 != 0) {
                return 0x10;
            }
            BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x3);
            if (BVar3 != 0) {
                return 0x11;
            }
            BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x19);
            if (BVar3 != 0) {
                return 0x15;
            }
            BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar2, 0x1d);
            if (BVar3 != 0) {
                return 0x16;
            }
            uVar2 = pass1_1010_7d7e(param_1, uVar4, 0x1, uVar2);
            if (uVar2 == 0) {
                return 0x0;
            }
            return 0xc;
        }
        uStack6 = 0x1;
    } else {
        uStack6 = 0x18;
    }
    return uStack6;
}


pub unsafe fn show_window_1010_7ace(mut param_1: u32) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut lVar4: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x20) != 0) {
        (iVar2 + 0x20) = 0;
        pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar2 + 0x1c));
        loop {
            lVar4 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            if (lVar4 == 0) {
                break;
            }
            uVar1 = (lVar4 + 0x8);
            ShowWindow16(0x1, (uVar1 + 0x6));
        }
    }
    return;
}


pub unsafe fn pass1_1010_7b8c(mut param_1: u32, mut param_2: i16) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut puVar5: *mut u8;
    let mut extraout_DX: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uStack14: u32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if (((iVar6 + 0x1e) | (iVar6 + 0x1c)) != 0) {
        pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar6 + 0x1c));
        loop {
            puVar5 = local_a;
            pass1_1008_5b12(CONCAT22(0x1050, puVar5));
            uStack14 = CONCAT22(extraout_DX, puVar5);
            if ((extraout_DX | puVar5) == 0) {
                break;
            }
            uVar4 = (puVar5 + 0x8);
            if !((uVar4 + 0x6) != param_2) {
                break;
            }
        }
        if ((extraout_DX | puVar5) != 0) {
            ppcVar3 = ((iVar6 + 0x1c) + 0xc);
            (**ppcVar3)(0x1008, (iVar6 + 0x1c), uStack14);
        }
        uVar4 = (iVar6 + 0x1c);
        if ((uVar4 + 0x8) == 0) {
            puVar1 = (iVar6 + 0x1c);
            uVar2 = (iVar6 + 0x1e);
            if ((uVar2 | puVar1) != 0) {
                ppcVar3 = *puVar1;
                (**ppcVar3)(0x1008, puVar1, uVar2, 0x1, puVar1, uVar2, puVar1, uVar2);
            }
            (iVar6 + 0x1c) = 0;
        }
    }
    return;
}

pub unsafe fn send_msg_1010_7c9e(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut lVar7: i32;
    let mut uVar8: u32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((((iVar5 + 0x1e) | (iVar5 + 0x1c)) != 0) && (param_2 != 0)) {
        pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar5 + 0x1c));
        loop {
            lVar7 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            uVar4 = (lVar7 >> 0x10);
            uVar2 = lVar7;
            if (lVar7 == 0) {
                break;
            }
            if ((uVar2 + 0x4) != 0) {
                uVar8 = struct_op_1030_73a8((uVar2 + 0x4), uVar2, uVar4 | uVar2);
                BVar3 = pass1_1008_c6ae(_u16_1050_06e0, (uVar8 + 0xc), param_2);
                if (BVar3 != 0) {
                    uVar1 = (uVar2 + 0x8);
                    SendMessage16(0x0, 0xeb, 0x111, (uVar1 + 0x6));
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1010_7d38(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: u16,
) -> u16 {
    let mut local_e: u32;
    let mut uStack10: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 0x2] = [0; 0x2];
    let mut local_4: [u8; 0x2] = [0; 0x2];

    local_e = (param_3 + 0xc);
    uStack10 = (param_3 + 0x10);
    pass1_1008_3eb4(
        CONCAT22(0x1050, &local_e),
        CONCAT22(0x1050, &local_8),
        CONCAT22(0x1050, local_6),
        CONCAT22(0x1050, local_4),
    );
    return local_8;
}


pub unsafe fn pass1_1010_7d7e(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: i16,
) -> u16 {
    let mut BVar1: bool;

    if (param_3 != 0x3) {
        if ((param_4 < 0xa) || (0x7f < param_4)) {
            return 0x0;
        }
        BVar1 = pass1_1008_c6ae(_u16_1050_06e0, param_4, 0x3c);
        if (BVar1 != 0) {
            return 0x0;
        }
        if (((param_4 == 0x6a) && (param_3 != 0x4)) && (param_3 != 0x5)) {
            return 0x0;
        }
    }
    return 0x1;
}

pub unsafe fn pass1_1010_7e40(param_1: *mut u8, param_2: *mut astruct_652) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut struct_1: *mut astruct_652;
    let mut uVar2: u16;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000fff6: u16;
    let mut uVar1: u32;

    paVar1 = CONCAT22(in_register_0000000a, param_1);
    uVar2 = (param_2 >> 0x10);
    struct_1 = param_2;
    param_2.field0_0x0 = 0;
    struct_1.field1648_0x67c = 0;
    struct_1.field1649_0x680 = 0;
    struct_1.field3698_0xe82 = 0;
    struct_1.field3699_0xe84 = 0;
    struct_1.field3700_0xe88 = 0;
    pass1_1000_4906(
        (param_2 & 0xffff0000 | ZEXT24(&struct_1.field1_0x4)),
        NULL,
        0x228,
    );
    pass1_1000_4906(
        (param_2 & 0xffff0000 | ZEXT24(&struct_1.field550_0x22c)),
        NULL,
        0x228,
    );
    pass1_1000_4906(
        (param_2 & 0xffff0000 | ZEXT24(&struct_1.field1099_0x454)),
        NULL,
        0x228,
    );
    struct_1.field_0x682 = 0;
    struct_1.field_0xa82 = 0;
    _u16_1050_14cc = param_2;
    puVar3 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff6, 0x2),
        in_stack_0000fe9e,
        in_stack_0000ffc2,
        in_stack_0000ffc8,
        in_stack_0000ffcc,
    );
    struct_1.field3700_0xe88 = puVar3;
    struct_1.field3701_0xe8a = (puVar3 >> 0x10);
    uVar1 = &struct_1.field3700_0xe88;
    struct_1.field3699_0xe84 = (uVar1 + 0x64);
    return;
}

pub unsafe fn pass1_1010_7efc(param_1: u32) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut puVar3: *mut u32;
    let mut ppcVar4: *mut *mut code;
    let mut iVar5: *mut astruct_448;
    let mut uVar5: u16;
    let mut unaff_CS: u16;
    let mut pcStack8: *mut c_char;
    let mut iStack4: i16;

    uVar5 = (param_1 >> 0x10);
    iVar5 = param_1;
    uVar1 = iVar5.field1660_0x67c;
    uVar2 = iVar5.field1661_0x67e;
    pcStack8 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0) {
        pass1_1008_64a2(CONCAT22(uVar2, uVar1));
        unaff_CS = 0x1000;
        fn_ptr_1000_17ce(pcStack8);
    }
    //   for (iStack4 = 0; iStack4 < 0x8a; iStack4 += 1)
    for iStack4 in 0..0x8a {
        puVar3 = (&iVar5.field_0x4 + iStack4 * 0x4);
        uVar1 = (&iVar5.field_0x6 + iStack4 * 0x4);
        if ((uVar1 | puVar3) != 0) {
            ppcVar4 = *puVar3;
            (**ppcVar4)(unaff_CS, puVar3, uVar1, 1);
        }
        puVar3 = (&iVar5.field_0x22c + iStack4 * 0x4);
        uVar1 = (&iVar5.field_0x22e + iStack4 * 0x4);
        if ((uVar1 | puVar3) != 0) {
            ppcVar4 = *puVar3;
            (**ppcVar4)(unaff_CS, puVar3);
        }
        puVar3 = (&iVar5.field_0x454 + iStack4 * 0x4);
        uVar1 = (&iVar5.field_0x456 + iStack4 * 0x4);
        if ((uVar1 | puVar3) != 0) {
            ppcVar4 = *puVar3;
            (**ppcVar4)(unaff_CS, puVar3);
        }
    }
    fn_ptr_1000_17ce(*param_1);
    _u16_1050_14cc = 0;
    return;
}

pub unsafe fn pass1_1010_7fd6(param_1: *mut astruct_489) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar3: *mut astruct_489;
    let mut uVar3: *mut astruct_489;
    let mut pcStack6: *mut c_char;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar1 = iVar3.field1660_0x67c;
    uVar2 = iVar3.field1661_0x67e;
    pcStack6 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0) {
        pass1_1008_64a2(CONCAT22(uVar2, uVar1));
        fn_ptr_1000_17ce(pcStack6);
    }
    iVar3.field1660_0x67c = 0;
    iVar3.field1662_0x680 = 0;
    return;
}

pub unsafe fn pass1_1010_8018(param_1: *mut u8, mut param_2: u32, mut param_3: u16) {
    let mut iVar1: i16;
    let mut uVar2: *mut u16;

    if ((param_3 * 0xa + 0x1fa0) != 0) {
        pass1_1010_878c(param_2, (param_3 * 0xa + 0x1fa0));
        uVar2 = (param_2 >> 0x10);
        if ((param_2 + 0x67c) != 0) {
            iVar1 = param_3 * 0xa;
            pass1_1008_64c8(
                param_3,
                param_1,
                (param_2 + 0x67c),
                CONCAT22((iVar1 + 0x1fa6), (iVar1 + 0x1fa8)),
                (iVar1 + 0x1fa4),
            );
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1010_81f6(param_1: *mut *mut astruct_87, param_2: i32, mut param_3: i16) {
    let mut in_EDX: u32;
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uStack12: u16;
    let mut uStack10: u32;

    if (param_2 == 0x8000001) {
        uVar1 = in_EDX & 0xffff0000 | param_1;
        uStack10 = param_1 & 0xffff0000 | (param_1 + 0x22c);
        uStack12 = 0xfa;
    } else if (param_2 == 0x8000002) {
        uVar1 = in_EDX & 0xffff0000 | param_1;
        uStack10 = param_1 & 0xffff0000 | (param_1 + 0x454);
        uStack12 = 0xfc;
    } else {
        uVar1 = in_EDX & 0xffff0000 | param_1;
        uStack10 = param_1 & 0xffff0000 | (param_1 + 0x4);
        uStack12 = 0x2;
    }
    uVar2 = (uStack10 >> 0x10);
    uVar3 = uVar1;
    if ((param_3 * 0x4 + uStack10) == 0) {
        if ((0x0 < param_3) && (param_3 < 0xa)) {
            pass1_1010_89f0(param_1, uVar3, uStack12, uStack10);
            return;
        }
        if (param_3 < 0xa) {
            return;
        }
        if (0x7f < param_3) {
            return;
        }
        if ((uStack10 + 0x14) == 0) {
            pass1_1010_89f0(param_1, uVar3, uStack12, uStack10);
        }
        pass1_1010_887a(param_1, uStack10, param_3, uVar1);
        uVar3 = param_1;
    }
    pass1_1010_866c(uVar1, param_1, uVar3, uStack10, param_3);
    return;
}

pub unsafe fn pass1_1010_82f8(mut param_1: u32, mut param_2: u16) {
    (param_1 + 0xe82) = param_2;
    return;
}

pub unsafe fn pass1_1010_84f8(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u32;
    let mut pcStack780: *mut c_char;
    let mut local_308: [u8; 0x100] = [0; 0x100];
    let mut local_208: [u8; 0x100] = [0; 0x100];
    let mut local_108: [u8; 0x104] = [0; 0x104];
    let mut iStack4: i16;

    if ((param_2 * 0x10 + 0x10) == 0x3) {
        uVar1 = (param_1 + 0xe88);
        iStack4 = (uVar1 + 0x70);
        str_1000_4d58(
            *(param_2 * 0x10 + 0x12),
            NULL,
            0x0,
            CONCAT22(0x1050, local_208),
            CONCAT22(0x1050, local_308),
        );
        unk_str_op_1000_3d3e(CONCAT22(0x1050, local_108), CONCAT22(0x1050, local_208));
        if (local_308[0] == '\0') {
            if (iStack4 == 0) {
                pcStack780 = s__mid_1050_14c0;
            } else {
                pcStack780 = s__wav_1050_14ba;
            }
        } else {
            pcStack780 = local_308;
        }
        pcStack780 = CONCAT22(0x1050, pcStack780);
        pass1_1000_3cea(CONCAT22(0x1050, local_108), pcStack780);
        set_err_mode_1010_8b14(param_1, CONCAT22(0x1050, local_108));
        return;
    }
    return;
}

pub unsafe fn pass1_1010_85be(mut param_1: u32, mut param_2: i16, mut param_3: i16) {
    let mut uVar1: u32;
    let mut local_30a: [u8; 0x100] = [0; 0x100];
    let mut local_20a: [u8; 0x100] = [0; 0x100];
    let mut local_10a: [u8; 0x108] = [0; 0x108];

    if (param_2 == 0x2) {
        uVar1 = (param_3 * 0x4 + 0x2e34);
        str_1000_4d58(
            (uVar1 & 0xffff0000 | (uVar1 + 0x3)),
            NULL,
            0x0,
            CONCAT22(0x1050, local_20a),
            CONCAT22(0x1050, local_30a),
        );
        unk_str_op_1000_3d3e(CONCAT22(0x1050, local_10a), s_male_1050_14c6);
        pass1_1000_3cea(CONCAT22(0x1050, local_10a), CONCAT22(0x1050, local_20a));
        pass1_1000_3cea(CONCAT22(0x1050, local_10a), CONCAT22(0x1050, local_30a));
        set_err_mode_1010_8b14(param_1, CONCAT22(0x1050, local_10a));
        return;
    }
    set_err_mode_1010_8b14(param_1, (param_3 * 0x4 + 0x2e34));
    return;
}