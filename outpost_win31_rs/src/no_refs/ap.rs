use crate::winapp::winapp_c::send_msg_1038_c228;
use crate::unk::block_1038_e000::{pass1_1038_e16e, pass1_1038_e308, pass1_1038_e6f0, pass1_1038_e9ec, pass1_1038_ebd6};
use crate::gui::window::set_win_pos_1038_c31a;

pub unsafe fn pass1_1038_d6c4(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1038_d276(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1038_e0ae(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1038_d7d0(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1038_e23a()

{
    return;
}

pub unsafe fn pass1_1038_e23e(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1038_e16e(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1038_e4bc(param_1: *mut u8, mut param_2: u16, mut param_3: u32, mut param_4: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut unaff_SI: u16;
    let mut puVar9: *mut u32;
    let mut puVar10: *mut u32;
    let mut paVar11: *mut Struct57;
    let mut in_stack_0000fe66: u16;
    let mut in_stack_0000fe68: u16;
    let mut in_stack_0000fe72: u16;
    let mut in_stack_0000ff8a: u16;
    let mut in_stack_0000ff8c: u16;
    let mut in_stack_0000ff90: u16;
    let mut in_stack_0000ff92: u16;
    let mut in_stack_0000ff94: u16;
    let mut in_stack_0000ff96: u16;
    let mut in_stack_0000ff9c: u16;
    let mut in_stack_0000ffa0: u16;
    let mut uVar12: u16;
    let mut uVar13: u8;
    let mut uVar14: u8;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut puStack22: *mut u32;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    if (param_4 == 0x1c4) {
        puVar9 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x2f), in_stack_0000fe72,
                                 in_stack_0000ff96, in_stack_0000ff9c, in_stack_0000ffa0);
        uVar15 = (puVar9 >> 0x10);
        uVar4 = (puVar9 + 0x24);
        uVar5 = (puVar9 + 0x26);
        uVar7 = paVar6 & 0xffff0000 | uVar5;
        uVar3 = uVar5 | uVar4;
        if (uVar3 != 0) {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(uVar5, uVar4));
            uVar8 = uVar7 & 0xffff0000;
            if ((uVar7 | uVar3) != 0) {
                puVar10 = pass1_1008_c6fa(_u16_1050_06e0, 0x20);
                paVar11 = (uVar8 & 0xffff0000 | puVar10 >> 0x10);
                uVar4 = puVar10;
                pass1_1038_4e78(uVar4, paVar11, CONCAT22(uVar7, uVar3), puVar10);
                uVar15 = SUB42(paVar11, 0x0);
                puStack22 = CONCAT22(uVar15, uVar4);
                uVar2 = *puStack22;
                ppcVar1 = uVar2 + 0x8;
                paVar6 = paVar11;
                uVar5 = uVar4;
                (**ppcVar1)(0x1008, uVar4, uVar15);
                uVar3 = paVar6 | uVar5;
                paVar6 = (paVar6 & 0xffff0000 | uVar3);
                if (uVar3 == 0) {
                    if (puStack22.is_null() == false) {
                        ppcVar1 = uVar2;
                        (**ppcVar1)(0x1008, uVar4, paVar11, 1);
                    }
                } else {
                    ppcVar1 = (*puStack22 + 0x4);
                    uVar3 = uVar4;
                    (**ppcVar1)(0x8, uVar4, uVar15, 0x0, 0x0);
                    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT13((paVar6 >> 0x8), CONCAT12(paVar6, uVar5)));
                    puVar9 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(uVar3, 0x32), in_stack_0000fe66,
                                             in_stack_0000ff8a, in_stack_0000ff90, in_stack_0000ff94);
                    pass1_1010_71d6(uVar5 + 0xc, (puVar9 >> 0x10), puVar9, 0x1,
                                    ((paVar6 & 0xff00) << 0x10 | CONCAT12(paVar6, uVar5 + 0xc)),
                                    0x1050);
                    if (puStack22.is_null() == false) {
                        ppcVar1 = *puStack22;
                        (**ppcVar1)(0x1010, uVar4, paVar11, 1);
                    }
                }
            }
        }
    } else {
        if (param_4 == 0x1c5) {
            uVar15 = 0xe;
        } else {
            if (param_4 != 0x1c6) {
                post_win_msg_1040_7b3c(CONCAT13((param_3 >> 0x8), CONCAT12(param_3, param_2)), (param_3 >> 0x10), param_4, param_4);
                return;
            }
            uVar15 = 0xd;
        }
        uVar17 = 0;
        uVar16 = 0;
        uVar12 = 0;
        uVar13 = 0;
        uVar14 = 0;
        paVar11 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, 0x32, in_stack_0000fe68, in_stack_0000ff8c,
                                  in_stack_0000ff92, in_stack_0000ff96);
        unk_win_op_1010_7300(paVar6 & 0xffff0000 | paVar11 >> 0x10, paVar11, CONCAT13(uVar14, CONCAT12(uVar13, uVar12)),
                             uVar15, CONCAT22(uVar17, uVar16));
    }
    return;
}


pub unsafe fn pass1_1038_e608(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1038_e308(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1038_e904()

{
    return;
}


pub unsafe fn pass1_1038_e908(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1038_e6f0(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
