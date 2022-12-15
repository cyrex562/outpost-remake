// WARNING: Globals starting with '_' overlap smaller symbols at the same address

use std::ptr::null_mut;
use crate::file_ops;

pub unsafe fn pass1_1008_7006(
    param_1: *mut u8,
    param_2: *mut astruct_802,
    mut param_3: u16,
    mut param_4: u32,
    mut param_5: u16,
) -> i16 {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut in_stack_0000fff8: u16;
    let mut iStack4: i16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    iStack4 = 0;
    loop {
        if (PTR_LOOP_1050_0334 <= iStack4) {
            return 0x1;
        }
        puVar4 = mixed_1010_20ba(
            paVar3,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000fff8, (iStack4 * 0x2 + 0x320)),
            in_stack_0000fea0,
            in_stack_0000ffc4,
            in_stack_0000ffca,
            in_stack_0000ffce,
        );
        paVar3 = (paVar3 & 0xffff0000 | puVar4 >> 0x10);
        in_stack_0000fff8 = SUB42(puVar4, 0x0);
        ppcVar1 = (*puVar4 + 0x8);
        iVar2 = (**ppcVar1)(0x1010, in_stack_0000fff8, (puVar4 >> 0x10), param_4);
        if (iVar2 == 0) {
            break;
        }
        iStack4 += 0x1;
    }
    return iVar2;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_7056(
    param_1: *mut u8,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
) -> i16 {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut in_stack_0000fff8: u16;
    let mut iStack4: i16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    iStack4 = 0;
    loop {
        if (PTR_LOOP_1050_0334 <= iStack4) {
            return 0x1;
        }
        puVar4 = mixed_1010_20ba(
            paVar3,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000fff8, (iStack4 * 0x2 + 0x320)),
            in_stack_0000fea0,
            in_stack_0000ffc4,
            in_stack_0000ffca,
            in_stack_0000ffce,
        );
        paVar3 = (paVar3 & 0xffff0000 | puVar4 >> 0x10);
        in_stack_0000fff8 = SUB42(puVar4, 0x0);
        ppcVar1 = (*puVar4 + 0xc);
        iVar2 = (**ppcVar1)(0x1010, in_stack_0000fff8, (puVar4 >> 0x10), param_4);
        if (iVar2 == 0) {
            break;
        }
        iStack4 += 0x1;
    }
    return iVar2;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Restarted to delay deadcode elimination for space: stack

// WARNING: Removing unreachable block (ram,0x100871e6)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_72a8(param_1: *mut u16, mut param_2: u16) -> *mut u16 {
    *param_1 = param_2;
    return param_1;
}

pub unsafe fn switch_1008_72bc(param_1: *mut HFILE16, mut param_2: u16) -> u16 {
    if (u16_1050_0312 < 0x2) {
        // switch(param_2)
        match param_2 {
            // case 0x1:
            1 => {
                param_2 = 0x1;
            }
            //   break;
            // case 0x2:
            2 => {
                param_2 = 0x2;
            }
            //   break;
            // case 0x3:
            3 => {
                param_2 = 0x3;
            }
            //   break;
            // case 0x4:
            4 => {
                param_2 = 0x5;
            }
            //   break;
            // case 0x5:
            5 => {
                param_2 = 0x4;
            }
            //   break;
            // case 0x6:
            6 => {
                param_2 = 0x6;
            }
            //   break;
            // case 0x7:
            7 => {
                param_2 = 0x7;
            }
            //   break;
            // case 0x8:
            8 => {
                param_2 = 0x8;
            }
            //   break;
            // case 0x9:
            9 => {
                param_2 = 0x9;
            }
            //   break;
            // case 0xa:
            10 => {
                param_2 = 0xa;
            }
            //   break;
            // case 0xb:
            11 => {
                param_2 = 0xb;
            }
            //   break;
            // case 0xc:
            12 => {
                param_2 = 0xc;
            }
            //   break;
            // case 0xd:
            13 => {
                param_2 = 0xd;
            }
            //   break;
            // case 0xe:
            14 => {
                param_2 = 0xe;
            }
            //   break;
            // case 0xf:
            15 => {
                param_2 = 0xf;
            }
            //   break;
            // case 0x10:
            16 => {
                return 0x10;
            }
            // case 0x11:
            0x11 => {
                return 0x11;
            }
            // case 0x12:
            0x12 => {
                return 0x12;
            }
            // case 0x13:
            0x13 => {
                return 0x13;
            }
            // _ =>
            _ => {
                return 0x0;
            }
        }
    };
    return param_2;
}

pub unsafe fn pass1_1008_738c(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> u16 {
    let mut u_var1: u16;

    //   switch(param_3)
    match param_3 {
        //   case 0x1:
        1 => {
            u_var1 = 0x3;
        }
        // break;
        //   case 0x2:
        2 => {
            u_var1 = 0x4;
        }
        // break;
        //   case 0x3:
        3 => {
            return 0x5;
        }
        //   case 0x4:
        4 => {
            return 0x6;
        }
        //   case 0x5:
        5 => {
            return 0x8;
        }
        //   case 0x6:
        6 => {
            return 0x9;
        }
        //   case 0x7:
        7 => {
            return 0xa;
        }
        //   _ =>
        _ => {
            u_var1 = 0;
        }
    };
    return u_var1;
}

pub unsafe fn switch_1008_73ea(mut param_1: u16, mut param_2: u16, mut param_3: i16) -> i16 {
    let mut i_stack4: i16;

    i_stack4 = param_3;
    if (u16_1050_0312 < 0x2) {
        // switch(param_3) {
        match param_3 {
            0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f | 0x20 | 0x21 | 0x22 | 0x23
            | 0x24 | 0x25 | 0x26 | 0x27 | 0x28 | 0x29 | 0x2a | 0x2b | 0x2c | 0x2d | 0x2e | 0x2f
            | 0x30 | 0x31 | 0x32 | 0x33 | 0x34 | 0x35 | 0x36 | 0x37 | 0x38 | 0x39 | 0x3a | 0x3b
            | 0x3c => {
                i_stack4 = param_3 + 0x3;
            }
            //   break;
            0x3d | 0x3e => {
                i_stack4 = param_3 + 0x4;
            }
            //   break;
            0x3f | 0x40 | 0x41 | 0x42 | 0x43 | 0x44 | 0x45 | 0x46 | 0x47 | 0x48 | 0x49 | 0x4a
            | 0x4b | 0x4c | 0x4d | 0x4e | 0x4f | 0x50 | 0x51 | 0x52 | 0x53 | 0x54 | 0x55 | 0x56
            | 0x57 | 0x58 | 0x59 | 0x5a | 0x5b | 0x5c | 0x5d | 0x5e | 0x5f | 0x60 | 0x61 | 0x62
            | 0x63 | 0x64 | 0x65 | 0x66 => {
                i_stack4 = param_3 + 0x8;
            }
            //   break;
            0x67 | 0x68 | 0x69 | 0x6a | 0x6b | 0x6c | 0x6d | 0x6e | 0x6f | 0x70 | 0x71 | 0x72
            | 0x73 | 0x74 | 0x75 | 0x76 | 0x77 | 0x78 | 0x79 | 0x7a | 0x7b | 0x7c | 0x7d | 0x7e
            | 0x7f | 0x80 => {
                i_stack4 = param_3 + 0x9;
            }
        };
    }
    return i_stack4;
}

pub unsafe fn pass1_1008_766e(param_1: *mut u8, mut param_2: u32, param_3: *mut astruct_169) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffe4: u16;
    let mut local_6: u32;
    let mut paVar5: *mut Struct57;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    param_3 = 0;
    local_6 = 0;
    puVar1 = &local_6;
    file_ops::file_1008_76e4(paVar4, param_2, CONCAT22(0x1050, puVar1));
    if (puVar1.is_null() == false) {
        if (local_6 != 0) {
            mem_op_1000_179c(0xc, paVar4);
            uVar2 = paVar4 | puVar1;
            paVar5 = (paVar4 & 0xffff0000 | uVar2);
            if (uVar2 == 0) {
                puVar1 = null_mut();
                uVar3 = 0;
            } else {
                pass1_1010_8ef2(
                    paVar5,
                    CONCAT22(paVar4, puVar1),
                    in_stack_0000ffe4,
                    in_stack_0000fe8c,
                    in_stack_0000ffb0,
                    in_stack_0000ffb6,
                    in_stack_0000ffba,
                );
                uVar3 = SUB42(paVar5, 0x0);
            }
            param_3 = puVar1;
            (param_3 + 0x2) = uVar3;
            fn_ptr_1010_905e(param_3, local_6);
        }
        return;
    }
    return;
}

pub unsafe fn pass1_1008_7898(mut param_1: u16, mut param_2: u32, param_3: *mut u32) {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut extraout_DX: u16;
    let mut uVar3: u16;
    let mut in_stack_0000ffc4: HFILE16;
    let mut local_26: u16;
    let mut local_24: [u32; 3] = [0; 3];
    let mut local_18: u32;
    let mut local_14: [u16; 0x5] = [0; 0x5];
    let mut uStack10: u32;
    let mut uStack6: u32;

    if (param_3.is_null()) {
        param_1 = 0;
        uVar3 = 0;
    } else {
        ppcVar1 = (*param_3 + 0x10);
        (**ppcVar1)();
        uVar3 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar3, param_1);
    local_18 = CONCAT22(uVar3, param_1);
    BVar2 = file_ops::write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, &local_18), 0x4, in_stack_0000ffc4);
    if (BVar2 != 0) {
        uStack10 = 0;
        loop {
            if (uStack6 <= uStack10) {
                return;
            }
            pass1_1020_c4a8(
                param_3,
                CONCAT22(0x1050, local_14),
                CONCAT22(0x1050, &local_18),
                uStack10,
            );
            local_24[0] = local_18;
            BVar2 = file_ops::write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, local_24),
                0x4,
                in_stack_0000ffc4,
            );
            if (BVar2 == 0) {
                break;
            }
            local_26 = local_14[0];
            BVar2 = file_ops::write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, &local_26),
                0x2,
                in_stack_0000ffc4,
            );
            if (BVar2 == 0) {
                u16_1050_0310 = 0x6d0;
                return;
            }
            uStack10 += 0x1;
        }
    }
    u16_1050_0310 = 0x6d0;
    return;
}

pub unsafe fn pass1_1008_79f0(mut param_1: u32, param_2: i32) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uStack4: u16;

    if (param_2 == 0) {
        uVar1 = 0;
        uStack4 = 0;
    } else {
        uVar2 = (param_2 >> 0x10);
        uVar1 = *(param_2 + 0x4);
        uStack4 = (param_2 + 0x6);
    }
    file_ops::write_to_file_1008_7954(uVar1, param_1, CONCAT22(uStack4, uVar1));
    return;
}

pub unsafe fn pass1_1008_7ad4(mut param_1: u32, param_2: *mut i32) -> u16 {
    let mut BVar1: bool;
    let mut local_14: [u16; 0x2] = [0; 0x2];
    let mut local_10: [u32; 0x2] = [0; 0x2];
    let mut uStack6: u16;
    let mut local_4: u16;

    BVar1 = file_ops::read_file_1008_7dee(param_1, CONCAT22(0x1050, &local_4), 0x2);
    if (BVar1 != 0) {
        uStack6 = 0;
        loop {
            if (local_4 <= uStack6) {
                return 0x1;
            }
            BVar1 = file_ops::read_file_1008_7dee(param_1, CONCAT22(0x1050, local_14), 0x2);
            if ((BVar1 == 0)
                || (
                BVar1 = file_ops::read_file_1008_7dee(param_1, CONCAT22(0x1050, local_10), 0x4),
                BVar1 == 0,
                ))
            {
                break;
            }
            pass1_1020_bb8a(
                param_2,
                local_10[0],
                CONCAT22(local_14[0], (local_10[0] >> 0x10)),
            );
            uStack6 += 0x1;
        }
    }
    return 0x0;
}

pub unsafe fn pass1_1008_7c2a(mut param_1: u32, param_2: *mut c_char) -> BOOL16 {
    let mut uVar1: u16;
    let mut BVar2: bool;
    let mut in_stack_0000ffe6: HFILE16;

    if (param_2.is_null() == false) {
        uVar1 = str_op_1000_3da4(param_2);
        BVar2 = file_ops::write_to_file_1008_7e1c(param_1, param_2, (uVar1 + 1), in_stack_0000ffe6);
        return BVar2;
    }
    file_ops::write_to_file_1008_7e1c(
        param_1,
        (s_playerName_1050_148e + 0xc),
        0x1,
        in_stack_0000ffe6,
    );
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_7e4a(param_1: *mut c_char, mut param_2: u16, param_3: u16) -> u16 {
    let mut uVar1: u16;

    sys_1000_3f9c(
        CONCAT22(0x1050, &param_2),
        s__s_02x_1050_0347,
        _PTR_s_dcbSC_1050_0336_1050_033c,
    );
    uVar1 = str_op_1000_3da4(CONCAT22(0x1050, &param_2));
    uVar1 = pass1_1000_3de8(param_1, CONCAT22(0x1050, &param_2), uVar1, param_2, param_3);
    if (uVar1 == 0) {
        return 0x1;
    }
    return 0x0;
}

pub unsafe fn pass1_1008_7e98(param_1: *mut astruct_460, param_2: u8) -> *mut u16 {
    let mut uVar1: *mut astruct_460;
    let mut uVar2: *mut astruct_460;

    uVar2 = (param_1 >> 0x10);
    uVar1 = param_1;
    param_1 = 0x380a;
    uVar1.field2_0x2 = 0x1008;
    param_1 = 0x389a;
    uVar1.field2_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn unk_draw_op_1008_7f62(
    param_1: *mut astruct_20,
    param_2: u16,
    param_3: u32,
) -> *mut astruct_20 {
    let mut HVar1: HGDIOBJ16;
    let mut HVar2: HCURSOR16;
    let mut iVar4: *mut astruct_20;
    let mut uVar3: u16;
    let mut uVar4: *mut astruct_20;
    let mut iVar3: *mut astruct_20;

    set_struct_1008_687a(param_1, param_3);
    uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    iVar4.field164_0xde = param_2;
    param_1.offset_0x0 = 0x8042;
    iVar4.base_0x2 = 0x1008;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar4.field60_0x5b)),
        s_SOLChildPar_1050_0358,
    );
    HVar1 = GetStockObject16(HOLLOW_BRUSH);
    iVar4.hgdiobj_field_0xc6 = HVar1;
    HVar2 = LoadCursor16(0x7f00, 0x0);
    iVar4.hcursor_field_0xc4 = HVar2;
    iVar4.field150_0xc8 = 0x2008;
    iVar4.field139_0xac = 0x44000000;
    iVar4.field145_0xbc = (param_3 + 0x8);
    iVar4.field151_0xca = iVar4.field164_0xde;
    win_ui_reg_class_1008_96d2(param_1);
    return param_1;
}
pub unsafe fn pass1_1008_7ffa(param_1: *mut astruct_461, param_2: u8) {
    let mut uVar1: *mut astruct_461;
    let mut uVar2: u16;

    uVar1 = param_1;
    uVar1 = uVar1 + 1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    uVar2 = (param_1 >> 0x10);
    param_1 = 0x380a;
    uVar1.field2_0x2 = 0x1008;
    param_1 = 0x389a;
    uVar1.field2_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}
