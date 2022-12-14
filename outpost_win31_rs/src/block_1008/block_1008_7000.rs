// WARNING: Globals starting with '_' overlap smaller symbols at the same address

use std::ptr::null_mut;

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

pub unsafe fn write_to_file_1008_70a6(struct802_param_1: *mut astruct_802) -> u16 {
    let mut hfile_1: HFILE16;
    let mut uVar1: u16;
    let mut i16_var2: i16;
    let mut i16_var4: *mut astruct_802;
    let mut i16_var5: u16;
    let mut i16_var6: u16;
    let mut in_AF: u8;
    let mut uVar2: u32;
    let hfile_2: HFILE16;

    i16_var5 = (struct802_param_1 >> 0x10);
    i16_var4 = struct802_param_1;
    if (i16_var4.hfile_0x4 != 0xffff) {
        _lclose16(i16_var4.hfile_0x4);
        i16_var4.hfile_0x4 = 0xffff;
    }
    hfile_2 = 0;
    hfile_1 = _lcreat16(0x0, struct802_param_1.filename_0x0);
    i16_var4.hfile_0x4 = hfile_1;
    if (hfile_1 == 0xffff) {
        u16_1050_0310 = 0x6cf;
    } else {
        u16_1050_0312 = 0x4;
        sys_1000_3f9c(s__1050_65a0, _PTR_s_SC_03d_1050_0314_1050_031c, 0x4);
        uVar1 = str_op_1000_3da4(s__1050_65a0);
        i16_var2 = uVar1 >> 0xf;
        uVar2 = _hwrite16(
            CONCAT22(0x65a0, i16_var2),
            CONCAT22(i16_var4.hfile_0x4, 0x1050),
            hfile_2,
        );
        if (uVar2 == CONCAT22(uVar1, i16_var2)) {
            return 0x1;
        }
        u16_1050_0310 = 0x6d0;
    }
    return 0x0;
}

// WARNING: Restarted to delay deadcode elimination for space: stack

pub unsafe fn read_file_1008_7146(
    mut param_1: u16,
    struct_param_1: *mut astruct_806,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) -> BOOL16 {
    let mut hfile_1: HFILE16;
    let mut uVar1: u16;
    let mut struct_1: *mut astruct_806;
    let mut uVar2: u16;

    uVar2 = (struct_param_1 >> 0x10);
    struct_1 = struct_param_1;
    if (struct_1.hfile_0x4 != 0xffff) {
        _lclose16(struct_1.hfile_0x4);
        struct_1.hfile_0x4 = 0xffff;
    }
    hfile_1 = _lopen16(0x0, struct_param_1.path_0x0);
    struct_1.hfile_0x4 = hfile_1;
    if (hfile_1 == 0xffff) {
        u16_1050_0310 = 0x6cf;
    } else {
        uVar1 = read_file_1008_71a0((struct_param_1 & 0xffff), param_1);
        if (uVar1 != 0) {
            return 0x1;
        }
    }
    return 0x0;
}

// WARNING: Removing unreachable block (ram,0x100871e6)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn read_file_1008_71a0(param_1: *mut astruct_806, mut param_2: u16) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut iStack26: i16;
    let mut iStack24: i16;
    let mut iStack22: i16;
    let mut local_e: [u8; 0x9] = [0; 0x9];
    let mut uStack5: u8;
    let mut uStack4: u16;

    uStack4 = 0x1;
    uVar1 = str_op_1000_3da4(s__1050_65a0);
    iStack22 = 0;
    iVar3 = WIN16_hread(uVar1, CONCAT22(0x1050, local_e), *(param_1 + 0x4));
    uVar2 = iVar3;
    if (uVar1 < iVar3) {
        uVar2 = uVar1;
    }
    iStack24 = uVar2 - 0x2;
    if (iStack24 < 0x0) {
        iStack24 = 0;
    }
    iStack26 = 0x2;
    while (iStack24 != 0) {
        iStack22 = iStack22 * 0xa + local_e[iStack26] -0x30;
        iStack26 += 0x1;
        iStack24 = iStack24 -0x1;
    }
    if (iStack22 == 1) {
        u16_1050_0312 = 0x1;
    } else if (iStack22 == 0x4) {
        u16_1050_0312 = 0x4;
    } else {
        uStack5 = '\0';
        u16_1050_0312 = 0x1;
        uStack4 = 0;
    }
    sys_1000_3f9c(
        s__1050_65a0,
        _PTR_s_SC_03d_1050_0314_1050_031c,
        u16_1050_0312,
    );
    return uStack4;
}

pub unsafe fn file_fn_1008_726c(param_1: *mut astruct_802, mut param_2: u16) -> BOOL16 {
    let hfile_1: HFILE16;

    if (param_1.hfile_0x4 != 0xffff) {
        hfile_1 = _lclose16(param_1.hfile_0x4);
        if (hfile_1 == 0xffff) {
            u16_1050_0310 = 0x6d1;
            return 0x0;
        }
        param_1.hfile_0x4 = 0xffff;
        u16_1050_0310 = 0;
    }
    return 0x1;
}

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
pub unsafe fn file_1008_7548(hfile_param: *mut HFILE16, param_2: *mut i32, mut param_3: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut file_read_ok: bool;
    let mut uVar2: u16;
    let mut buffer_3: *mut u8;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut buffer_4: *mut Struct57;
    let mut unaff_CS: u16;
    let mut lVar5: i32;
    let mut read_buffer_1c: *mut u8;
    let mut local_18: [u16; 0x5] = [0; 0x5];
    let mut puStack14: *mut u8;
    let mut uStack10: u32;
    let mut read_buffer: *mut u8;

    uVar4 = (param_3 >> 0x10);
    read_buffer = null_mut();
    file_read_ok = read_file_1008_7dee(hfile_param, CONCAT22(0x1050, &read_buffer), 0x4);
    if (file_read_ok == 0) {
        return;
    }
    if (read_buffer.is_null() == false) {
        buffer_3 = read_buffer;
        if (read_buffer < 0xc8) {
            read_buffer = 0;
            buffer_3 = 0xc8;
        }
        buffer_4 = CONCAT22(uVar4, read_buffer);
        uVar2 = buffer_3;
        uStack10 = buffer_3 & 0xffff | read_buffer << 0x10;
        if (*param_2 == 0) {
            unaff_CS = 0x1000;
            mem_op_1000_179c(0x1e, buffer_4);
            uVar3 = buffer_4 | uVar2;
            if (uVar3 == 0) {
                *param_2 = 0;
            } else {
                unaff_CS = 0x1020;
                struct_1020_c444(CONCAT22(buffer_4, uVar2), 0x64, uStack10);
                param_2 = uVar2;
                (param_2 + 0x2) = uVar3;
            }
        }
        lVar5 = *param_2;
        ppcVar1 = (*param_2 + 0x24);
        (**ppcVar1)();
        // for (puStack14 = null_mut(); puStack14 < read_buffer; puStack14 = puStack14 + 1)
        puStack14 = null_mut();
        while puStack14 < read_buffer {
            file_read_ok = read_file_1008_7dee(hfile_param, CONCAT22(0x1050, &read_buffer_1c), 0x4);
            if ((file_read_ok == 0)
                || (
                    file_read_ok =
                        read_file_1008_7dee(hfile_param, CONCAT22(0x1050, local_18), 0x2),
                    file_read_ok == 0,
                ))
            {
                ppcVar1 = (*param_2 + 0x1c);
                (**ppcVar1)(unaff_CS, *param_2);
                return;
            }
            ppcVar1 = (*param_2 + 0x28);
            (**ppcVar1)(
                unaff_CS,
                *param_2,
                (*param_2 >> 0x10),
                local_18[0],
                read_buffer_1c,
            );
            puStack14 += 1;
        }
        ppcVar1 = (*param_2 + 0x1c);
        (**ppcVar1)(unaff_CS, *param_2, lVar5);
    }
    return;
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
    file_1008_76e4(paVar4, param_2, CONCAT22(0x1050, puVar1));
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
pub unsafe fn file_1008_76e4(param_1: *mut astruct_57, param_2: *mut HFILE16, param_3: *mut i32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut local_18: [u8; 0xe] = [0; 0xe];
    let mut puStack10: *mut u8;
    let mut buffer_6: *mut u8;

    buffer_6 = null_mut();
    uVar2 = read_file_1008_7dee(param_2, CONCAT22(0x1050, &buffer_6), 0x4);
    if (uVar2 == 0) {
        return;
    }
    if (buffer_6.is_null() == false) {
        if (*param_3 == 0) {
            mem_op_1000_179c(0x18, param_1);
            uVar4 = param_1 | uVar2;
            if (uVar4 == 0) {
                *param_3 = 0;
            } else {
                struct_op_1030_1cd8(CONCAT22(param_1, uVar2), 0x5, buffer_6);
                param_3 = uVar2;
                (param_3 + 0x2) = uVar4;
            }
        }
        ppcVar1 = (*param_3 + 0x14);
        (**ppcVar1)();
        // for (puStack10 = null_mut(); puStack10 < buffer_6; puStack10 = puStack10 + 1)
        puStack10 = null_mut();
        while puStack10 < buffer_6 {
            BVar3 = read_file_1008_7dee(param_2, CONCAT22(0x1050, local_18), 0x4);
            if (BVar3 == 0) {
                return;
            }
            ppcVar1 = (*param_3 + 0x18);
            (**ppcVar1)();
            puStack10 += 1;
        }
        ppcVar1 = (*param_3 + 0x1c);
        (**ppcVar1)();
    }
    return;
}

pub unsafe fn file_1008_77cc(mut param_1: u16, mut param_2: u32, param_3: *mut i32) -> u16 {
    let mut u_var1: u16;
    let mut bvar2: bool;
    let mut u_var3: u16;
    let mut in_register_0000000a: u16;
    let mut pa_var4: *mut Struct57;
    let mut local_14: [u16; 0x2] = [0; 0x2];
    let mut local_10: [u32; 2] = [0; 2];
    let mut u_stack6: u16;
    let mut local_4: u16;

    pa_var4 = CONCAT22(in_register_0000000a, param_1);
    local_4 = 0;
    u_var1 = read_file_1008_7dee(param_2, CONCAT22(0x1050, &local_4), 0x2);
    if (u_var1 == 0) {
        return 0x0;
    }
    if (local_4 != 0) {
        if (*param_3 == 0) {
            mem_op_1000_179c(0xa, pa_var4);
            u_var3 = pa_var4 | u_var1;
            if (u_var3 == 0) {
                *param_3 = 0;
            } else {
                pass1_1020_ba3e(CONCAT22(pa_var4, u_var1), 0x5, 0x5);
                param_3 = u_var1;
                (param_3 + 0x2) = u_var3;
            }
        }
        // for (uStack6 = 0; u_stack6 < local_4; u_stack6 += 1)
        for uStack6 in 0..local_4 {
            bvar2 = read_file_1008_7dee(param_2, CONCAT22(0x1050, local_14), 0x2);
            if (bvar2 == 0) {
                return 0x0;
            }
            bvar2 = read_file_1008_7dee(param_2, CONCAT22(0x1050, local_10), 0x4);
            if (bvar2 == 0) {
                return 0x0;
            }
            pass1_1020_bb8a(
                *param_3,
                local_10[0],
                CONCAT22(local_14[0], (local_10[0] >> 0x10)),
            );
        }
    }
    return 0x1;
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
    BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, &local_18), 0x4, in_stack_0000ffc4);
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
            BVar2 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, local_24),
                0x4,
                in_stack_0000ffc4,
            );
            if (BVar2 == 0) {
                break;
            }
            local_26 = local_14[0];
            BVar2 = write_to_file_1008_7e1c(
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

pub unsafe fn write_to_file_1008_7954(param_1: u16, param_2: *mut u8, param_3: *mut u32) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut uVar3: u32;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut extraout_DX_00: u16;
    let mut in_stack_0000ffca: HFILE16;
    let mut local_20: u16;
    let mut uStack30: u16;
    let mut local_18: u16;
    let mut uStack22: u16;
    let mut uStack10: u32;
    let mut uStack6: u32;

    if (param_3.is_null()) {
        param_1 = 0;
        uVar4 = 0;
    } else {
        ppcVar1 = (*param_3 + 0x10);
        (**ppcVar1)();
        uVar4 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar4, param_1);
    local_18 = param_1;
    uStack22 = uVar4;
    BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, &local_18), 0x4, in_stack_0000ffca);
    if (BVar2 != 0) {
        uStack10 = 0;
        loop {
            if (uStack6 <= uStack10) {
                return uVar4;
            }
            ppcVar1 = (*param_3 + 0x4);
            uVar3 = uStack6;
            (**ppcVar1)();
            local_20 = uVar3;
            uVar4 = extraout_DX_00;
            uStack30 = extraout_DX_00;
            local_18 = local_20;
            uStack22 = extraout_DX_00;
            BVar2 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, &local_20),
                0x4,
                in_stack_0000ffca,
            );
            if (BVar2 == 0) {
                break;
            }
            uStack10 += 0x1;
        }
    }
    u16_1050_0310 = 0x6d0;
    return uVar4;
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
    write_to_file_1008_7954(uVar1, param_1, CONCAT22(uStack4, uVar1));
    return;
}
pub unsafe fn write_to_file_1008_7a22(param_1: *mut u8, param_2: i32) {
    let mut BVar1: bool;
    let mut in_stack_0000ffc6: HFILE16;
    let mut local_24: [u32; 0x2] = [0; 0x2];
    let mut local_1c: [u16; 0x5] = [0; 0x5];
    let mut local_12: u16;
    let mut local_10: u32;
    let mut uStack10: u16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    if (param_2 == 0) {
        uStack4 = 0;
    } else {
        uStack4 = (param_2 + 0x4);
    }
    local_12 = uStack4;
    BVar1 = write_to_file_1008_7e1c(param_1, CONCAT22(0x1050, &local_12), 0x2, in_stack_0000ffc6);
    if (BVar1 == 0) {
        u16_1050_0310 = 0x6d0;
    } else {
        uStack6 = 0;
        loop {
            if (uStack4 <= uStack6) {
                return;
            }
            pass1_1020_bb16(
                param_2,
                CONCAT22(0x1050, &local_10),
                CONCAT22(0x1050, &local_12),
                uStack6,
            );
            uStack10 = local_12;
            local_1c[0] = local_12;
            BVar1 = write_to_file_1008_7e1c(
                param_1,
                CONCAT22(0x1050, local_1c),
                0x2,
                in_stack_0000ffc6,
            );
            if (BVar1 == 0) {
                break;
            }
            local_24[0] = local_10;
            BVar1 = write_to_file_1008_7e1c(
                param_1,
                CONCAT22(0x1050, local_24),
                0x4,
                in_stack_0000ffc6,
            );
            if (BVar1 == 0) {
                return;
            }
            uStack6 += 0x1;
        }
    }
    return;
}

pub unsafe fn pass1_1008_7ad4(mut param_1: u32, param_2: *mut i32) -> u16 {
    let mut BVar1: bool;
    let mut local_14: [u16; 0x2] = [0; 0x2];
    let mut local_10: [u32; 0x2] = [0; 0x2];
    let mut uStack6: u16;
    let mut local_4: u16;

    BVar1 = read_file_1008_7dee(param_1, CONCAT22(0x1050, &local_4), 0x2);
    if (BVar1 != 0) {
        uStack6 = 0;
        loop {
            if (local_4 <= uStack6) {
                return 0x1;
            }
            BVar1 = read_file_1008_7dee(param_1, CONCAT22(0x1050, local_14), 0x2);
            if ((BVar1 == 0)
                || (
                    BVar1 = read_file_1008_7dee(param_1, CONCAT22(0x1050, local_10), 0x4),
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

pub unsafe fn write_to_file_1008_7b4c(param_1: *mut u8, param_2: *mut astruct_615) -> u16 {
    let mut BVar1: bool;
    let mut in_stack_0000ffd4: HFILE16;
    let mut local_12: [u16; 0x3] = [0; 0x3];
    let mut local_c: [u16; 0x2] = [0; 0x2];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3eb4(
        param_2,
        CONCAT22(0x1050, &local_8),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    local_12[0] = local_4;
    BVar1 = write_to_file_1008_7e1c(param_1, CONCAT22(0x1050, local_12), 0x2, in_stack_0000ffd4);
    if (BVar1 != 0) {
        local_c[0] = local_6;
        BVar1 = write_to_file_1008_7e1c(param_1, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffd4);
        if (BVar1 != 0) {
            local_c[0] = local_8;
            BVar1 =
                write_to_file_1008_7e1c(param_1, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffd4);
            if (BVar1 != 0) {
                return 0x1;
            }
        }
    }
    return 0x0;
}

pub unsafe fn read_file_1008_7bc8(mut param_1: u32, param_2: *mut u16) -> BOOL16 {
    let mut BVar1: bool;
    let mut local_8: u16;
    let mut local_6: u32;

    BVar1 = read_file_1008_7dee(param_1, CONCAT22(0x1050, &local_6 + 0x2), 0x2);
    if (BVar1 != 0) {
        BVar1 = read_file_1008_7dee(param_1, CONCAT22(0x1050, &local_6), 0x2);
        if (BVar1 != 0) {
            BVar1 = read_file_1008_7dee(param_1, CONCAT22(0x1050, &local_8), 0x2);
            if (BVar1 != 0) {
                pass1_1008_3e76(param_2, local_8, local_6, (local_6 >> 0x10));
                return 0x1;
            }
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
        BVar2 = write_to_file_1008_7e1c(param_1, param_2, (uVar1 + 1), in_stack_0000ffe6);
        return BVar2;
    }
    write_to_file_1008_7e1c(
        param_1,
        (s_playerName_1050_148e + 0xc),
        0x1,
        in_stack_0000ffe6,
    );
    return 0x1;
}
pub unsafe fn read_file_1008_7c6e(param_1: HFILE16, mut param_2: u16, param_3: *mut c_char) {
    let mut pcVar1: *mut c_char;
    let mut local_c: [u8; 0xa] = [0; 0xa];

    loop {
        pcVar1 = param_3;
        WIN16_hread(0x1, CONCAT22(0x1050, local_c), *_param_1);
        if (local_c[0] == '\0') {
            break;
        }
        param_3 = (param_3 & 0xffff0000 | (param_3 + 1));
        *pcVar1 = local_c[0];
    }
    *param_3 = '\0';
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn write_to_file_1008_7cac(param_1: *mut u8) -> BOOL16 {
    let mut uVar1: u16;
    let mut BVar2: bool;
    let mut in_stack_0000ffde: HFILE16;
    let mut local_c: [u8; 0xa] = [0; 0xa];

    sys_1000_3f9c(
        CONCAT22(0x1050, local_c),
        s__s_02x_1050_0340,
        _PTR_s_dcbSC_1050_0336_1050_033c,
    );
    uVar1 = str_op_1000_3da4(CONCAT22(0x1050, local_c));
    BVar2 = write_to_file_1008_7e1c(param_1, CONCAT22(0x1050, local_c), uVar1, in_stack_0000ffde);
    if (BVar2 == 0) {
        u16_1050_0310 = 0x6d0;
        return BVar2;
    }
    return 0x1;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn read_file_1008_7cfe(mut param_1: u16, mut param_2: u16, mut param_3: u16) {
    let mut bVar1: bool;
    let mut uVar2: u16;
    let mut iVar3: i32;
    let mut in_stack_0000fbd6: u16;
    let mut in_stack_0000fbd8: u16;
    let mut uStack1040: u32;
    let mut local_406: [u8; 0x400] = [0; 0x400];
    let mut uStack6: u32;

    uStack6 = 0;
    bVar1 = false;
    loop {
        _llseek16(0x0, uStack6, *CONCAT22(param_2, param_1));
        iVar3 = WIN16_hread(
            0x400,
            CONCAT22(0x1050, local_406),
            *CONCAT22(param_2, param_1),
        );
        // for (uStack1040 = 0; uStack1040 < iVar3; uStack1040 += 1)
        for uStack1040 in 0..iVar3 {
            if (local_406[uStack1040] == *_PTR_s_dcbSC_1050_0336_1050_033c) {
                if (!bVar1) {
                    bVar1 = true;
                    uStack6 = CONCAT22(
                        (uStack6 >> 0x10) + uStack1040 + CARRY2(uStack6, uStack1040),
                        uStack6 + uStack1040,
                    );
                    break;
                }
                bVar1 = false;
                uVar2 = pass1_1008_7e4a(
                    CONCAT22(0x1050, local_406 + uStack1040),
                    in_stack_0000fbd6,
                    in_stack_0000fbd8,
                );
                if (uVar2 != 0) {
                    _llseek16(0x0, uStack1040 + uStack6 + 0x7, *CONCAT22(param_2, param_1));
                    return;
                }
            }
        }
        if (!bVar1) {
            if (iVar3 < 0x400) {
                return;
            }
            uStack6 = CONCAT11(uStack6._1_1_ + 0x4, uStack6);
            uStack6 = CONCAT22((uStack6 >> 0x10) + (0xfb < uStack6._1_1_), uStack6);
        }
    }
}

pub unsafe fn read_file_1008_7dee(
    hfile_param_1: *mut HFILE16,
    buffer_param_2: *mut u8,
    count_param_3: u32,
) -> BOOL16 {
    let mut read_count: i32;

    read_count = WIN16_hread(count_param_3, buffer_param_2, *hfile_param_1);
    if ((read_count == count_param_3) && ((read_count >> 0x10) == count_param_3)) {
        return 0x1;
    }
    return 0x0;
}

pub unsafe fn write_to_file_1008_7e1c(
    buffer: *mut u8,
    count: u32,
    buf_to_write: *mut c_char,
    param_4: HFILE16,
) -> BOOL16 {
    let mut uVar1: u32;
    let mut uStackY16: u16;
    let mut uVar2: u16;

    uStackY16 = SUB42(buf_to_write, 0x0);
    uVar2 = (buf_to_write >> 0x10);
    uVar1 = _hwrite16(
        CONCAT22(count, uVar2),
        CONCAT22(buffer, (count >> 0x10)),
        param_4,
    );
    if (uVar1 != CONCAT22(uStackY16, uVar2)) {
        return 0x0;
    }
    return 0x1;
}

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
