use std::ptr::null_mut;
use crate::block_1000::block_1000_0000::{call_fn_ptr_1000_0dc6, mem_op_1000_0a48};
use crate::block_1000::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::block_1008::{block_1008_4000, block_1008_7000};
use crate::structs::struct_57::Struct57;
use crate::utils::CONCAT22;
use crate::winbase::{_lclose16, _llseek16, _lopen16};
use crate::windef::HFILE16;

pub unsafe fn close_file_1008_496c(param_1: *mut astruct_803) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar5: *mut astruct_803;
    let mut uVar2: u16;
    let mut puVar1: *mut u32;
    let mut uVar1: u16;
    let mut lVar1: i32;

    uVar2 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.offset_0x0 = &u16_1050_4c4c;
    iVar5.base_0x2 = 0x1008;
    puVar1 = iVar5.field2_0x4;
    uVar1 = iVar5.field3_0x6;
    if (uVar1 | puVar1) != 0 {
        ppcVar1 = *puVar1;
        (**ppcVar1)();
    }
    fn_ptr_1000_17ce(iVar5.field4_0x8);
    if iVar5.field18_0x1a != 0 {
        call_fn_ptr_1000_0dc6(iVar5.field18_0x1a);
    }
    if iVar5.field5_0xc != 0xffff {
        _lclose16(iVar5.field5_0xc);
    }
    param_1.offset_0x0 = 0x389a;
    iVar5.base_0x2 = 0x1008;
    return;
}

pub unsafe fn read_file_1008_49e8(
    param_1: HFILE16,
    mut param_2: u16,
    struct_param_1: *mut astruct_81,
    mut param_4: u32,
) -> u16 {
    let mut uVar4: u32;
    let mut hfile_1: HFILE16;
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uVar1: u32;
    let mut lVar5: i16;
    let mut puVar5: *mut u8;
    let mut extraout_DX: *mut u8;
    let mut uVar8: u16;
    let mut paVar7: *mut Struct57;
    let mut struct_1: *mut astruct_81;
    let mut unaff_DI: i16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut iVar9: i32;
    let mut lVar10: i32;
    let mut uStack20: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut iStack6: i32;
    let mut in_stack_0000ffe8: i16;
    let mut uVar3: u32;
    let mut in_stack_0000ffea: u16;
    let mut in_stack_0000ffe8_00: u16;
    let mut uVar6: u32;

    uVar8 = (param_4 >> 0x10);
    uVar5 = (struct_param_1 >> 0x10);
    struct_1 = struct_param_1;
    if (&struct_1.field3_0x8 != 0) {
        if (struct_1.field10_0x1e != 0) {
            return 0x1;
        }
        if (struct_1.hfile_0xc == -1) {
            hfile_1 = _lopen16(0x0, *&struct_1.field3_0x8);
            struct_1.hfile_0xc = hfile_1;
            if (hfile_1 == 0xffff) {
                return 0x0;
            }
        }
        iStack6 = 0;
        iVar9 = WIN16_hread(0xe, CONCAT22(0x1050, &param_1), struct_1.hfile_0xc);
        if (((iVar9 == 0xe) && ((iVar9 >> 0x10) == 0))
            && (
                iStack6 = CONCAT22(uStack20, param_2),
                param_1 == &PTR_LOOP_1050_4d42,
            ))
        {
            _llseek16(0x0, 0x0, struct_1.hfile_0xc);
            lVar10 = mem_op_1000_0a48(0x1, iStack6, (iStack6 >> 0x10), _PTR_LOOP_1050_5f2c);
            lVar5 = (lVar10 >> 0x10);
            struct_1.buffer_0x1a = lVar10;
            (&struct_1.buffer_0x1a + 0x2) = lVar5;
            if ((lVar5 | &struct_1.buffer_0x1a) != 0) {
                iVar9 = WIN16_hread(iStack6, struct_1.buffer_0x1a, struct_1.hfile_0xc);
                uStack8 = (iVar9 >> 0x10);
                paVar7 = CONCAT22(uVar8, uStack8);
                uStack10 = iVar9;
                param_1 = struct_1.hfile_0xc;
                _lclose16(param_1);
                struct_1.hfile_0xc = 0xffff;
                struct_1.field10_0x1e = 0x1;
                struct_1.field6_0xe = struct_1.buffer_0x1a;
                uVar3 = struct_1.buffer_0x1a;
                iVar1 = uVar3;
                iVar1 = iVar1 + 0xe;
                struct_1.field7_0x12 = uVar3 & 0xffff0000 | iVar1;
                uVar1 = iVar1 + 0x436;
                uVar1 = uVar3 & 0xffff0000 | uVar1;
                struct_1.field8_0x16 = uVar1;
                param_2 = 0x14;
                param_1 = s_tile2_bmp_1050_1538;
                mem_op_1000_179c(0x14, paVar7);
                puVar5 = (paVar7 | uVar1);
                extraout_DX = puVar5;
                if (puVar5.is_null()) {
                    struct_1.field2_0x4 = 0;
                } else {
                    param_2 = 0x100;
                    uVar4 = struct_1.field7_0x12;
                    uVar2 = uVar4;
                    uVar2 = uVar2 + 0x28;
                    uVar4 &= 0xffff0000;
                    uVar6 = uVar4 | uVar2;
                    param_1 = (uVar4 >> 0x10);
                    block_1008_4000::struct_op_1008_4c98((uVar1 & 0xffff | uVar2 << 0x10), uVar6, 0x100);
                    struct_1.field2_0x4 = uVar6;
                    (&struct_1.field2_0x4 + 0x2) = extraout_DX;
                }
                if (struct_1.field13_0x22 == 0) {
                    return 0x1;
                }
                _param_1 = struct_param_1;
                block_1008_4000::pass1_1008_4b8e(extraout_DX, struct_param_1);
                return 0x1;
            }
        } else {
            _lclose16(struct_1.hfile_0xc);
            struct_1.hfile_0xc = 0xffff;
        }
    }
    return 0x0;
}

pub unsafe fn file_1008_4c26(param_1: *mut astruct_803, param_2: u8) -> *mut astruct_803 {
    close_file_1008_496c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn close_file_1008_6dd0(param_1: *mut StructD) {
    let mut struct_1: *mut StructD;
    let mut struct_1_hi: u16;

    struct_1_hi = (param_1 >> 0x10);
    struct_1 = param_1;
    if (struct_1.hfile_0x4 != 0xffff) {
        _lclose16(struct_1.hfile_0x4);
        struct_1.hfile_0x4 = 0xffff;
    }
    fn_ptr_1000_17ce(*param_1);
    return;
}

pub unsafe fn file_fn_1008_6e02(param_1: *mut astruct_802) -> BOOL16 {
    let mut var1: u16;
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut var2: bool;
    let mut extraout_DX: *mut u8;
    let mut struct_1: *mut astruct_802;
    let mut unaff_DI: i16;
    let mut uVar1: u16;
    let mut puVar1: *mut u16;
    let mut local_4: [u8; 0x2] = [0; 0x2];

    u16_1050_0310 = 0;
    var1 = write_to_file_1008_70a6(param_1);
    if (var1 != 0) {
        uVar1 = (param_1 >> 0x10);
        struct_1 = param_1;
        puVar1 = pass1_1008_72a8(CONCAT22(0x1050, local_4), struct_1.hfile_0x4);
        extraout_DX = (puVar1 >> 0x10);
        iVar1 = pass1_1008_7006(
            extraout_DX,
            struct_1,
            uVar1,
            CONCAT22(0x1050, local_4),
            &DAT_1050_1050,
        );
        if ((iVar1 != 0)
            && (
                iVar2 = file_fn_1008_6eee(struct_1, uVar1, CONCAT22(0x1050, local_4)),
                iVar2 != 0,
            ))
        {
            var2 = file_fn_1008_726c(struct_1, uVar1);
            if (var2 == 0) {
                return 0x0;
            }
            return 0x1;
        }
        _lclose16(struct_1.hfile_0x4);
    }
    return 0x0;
}

pub unsafe fn read_file_1008_6e78(
    param_1: *mut astruct_806,
    mut param_2: u16,
    in_string: u16,
    mut param_4: u16,
) -> BOOL16 {
    let mut b_var1: bool;
    let mut i_var2: i16;
    let mut var3: *mut u8;
    let mut puVar1: *mut u8;
    let mut paVar2: *mut astruct_802;
    let mut unaff_BP: u16;
    let mut uVar3: u16;
    let mut unaff_CS: u16;
    let mut puVar4: *mut u16;
    let mut in_stack_00000000: u16;
    let mut in_stack_0000fffc: u16;

    u16_1050_0310 = 0;
    b_var1 = read_file_1008_7146(
        unaff_CS,
        param_1,
        in_stack_0000fffc,
        unaff_BP,
        in_stack_00000000,
    );
    if (b_var1 != 0) {
        uVar3 = (param_1 >> 0x10);
        paVar2 = param_1;
        puVar4 = pass1_1008_72a8(CONCAT22(0x1050, &stack0xfffc), paVar2.hfile_0x4);
        puVar1 = (puVar4 >> 0x10);
        i_var2 = pass1_1008_7056(puVar1, paVar2, uVar3, CONCAT22(0x1050, &stack0xfffc));
        if (i_var2 != 0) {
            var3 = &stack0xfffc;
            read_file_1008_6f7a(paVar2, uVar3, CONCAT22(0x1050, var3), puVar1);
            if (var3.is_null() == false) {
                b_var1 = file_fn_1008_726c(paVar2, uVar3);
                if (b_var1 == 0) {
                    return 0x0;
                }
                return 0x1;
            }
        }
        _lclose16(paVar2.hfile_0x4);
    }
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn file_fn_1008_6eee(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut in_DX: *mut u8;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut uVar5: u32;
    let mut local_e: [u8; 0x4] = [0; 0x4];
    let mut uStack10: u32;
    let mut puStack6: *mut u32;

    puStack6 = *_u16_1050_5748;
    uStack10 = *puStack6;
    puVar4 = pass1_1020_a43e(in_DX, CONCAT22(0x1050, local_e));
    uVar3 = (puVar4 >> 0x10);
    BVar1 = pass1_1028_d7a0(uStack10, (uStack10 >> 0x10), param_3);
    if (BVar1 != 0) {
        BVar1 = pass1_1030_5c1a(_PTR_LOOP_1050_5736, param_3);
        if (BVar1 != 0) {
            uVar5 = write_to_file_1028_dce2(uVar3, _PTR_LOOP_1050_65e2, param_3);
            if ((uVar5 >> 0x10) != 0) {
                uVar2 = pass1_1038_7b20(_PTR_LOOP_1050_5a64, param_3);
                if (uVar2 != 0) {
                    BVar1 = pass1_1020_a644(local_e, &DAT_1050_1050, param_3);
                    if (BVar1 != 0) {
                        return;
                    }
                }
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn read_file_1008_6f7a(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u8,
) {
    let mut var5: u16;
    let mut i_var3: i16;
    let mut b_var4: bool;
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut local_e: [u8; 0x4] = [0; 0x4];
    let mut uStack10: u32;
    let mut puStack6: *mut u32;

    puStack6 = *_u16_1050_5748;
    uStack10 = *puStack6;
    puVar2 = pass1_1020_a43e(param_4, CONCAT22(0x1050, local_e));
    uVar1 = (puVar2 >> 0x10);
    var5 = read_file_1028_d7ba(puVar2, uStack10, (uStack10 >> 0x10), param_3);
    if (var5 != 0) {
        var5 = read_file_1030_5c52(var5, _PTR_LOOP_1050_5736, param_3);
        if (var5 != 0) {
            read_file_1028_def2(var5, _PTR_LOOP_1050_65e2, param_3);
            if (var5 != 0) {
                i_var3 = read_file_1038_7c02(var5, uVar1, _PTR_LOOP_1050_5a64, param_3);
                if (i_var3 != 0) {
                    b_var4 = read_file_1020_a65e(local_e, CONCAT22(0x1050, local_e), param_3);
                    if (b_var4 != 0) {
                        return;
                    }
                }
            }
        }
    }
    return;
}

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

pub unsafe fn file_1008_76e4(param_1: *mut Struct57, param_2: *mut HFILE16, param_3: *mut i32) {
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
                uVar2 = block_1008_7000::pass1_1008_7e4a(
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

pub unsafe fn read_file_1030_4e70(
    param_1: *mut astruct_117,
    param_2: *mut u32,
    param_3: *mut *mut u8,
    param_4: i32,
    mut param_5: u16,
) -> u16 {
    let mut uVar1: u16;
    let mut h_file: HFILE16;
    let mut unaff_SS: u16;
    let mut path: *mut c_char;
    let mut lVar1: i32;
    let mut uVar2: u16;
    let mut pbStack60: *mut u8;
    let mut iStack56: i32;
    let mut uStack20: u32;

    *param_3 = null_mut();
    *param_2 = 0;
    if (param_4 != 0) {
        uVar2 = 0;
        path = pass1_1030_5164(param_1, param_4);
        param_5 = (path >> 0x10);
        uVar1 = dos3_call_1000_51aa(path, param_5, uVar2);
        if (uVar1 == 0) {
            *param_2 = uStack20;
            h_file = _lopen16(0x0, path);
            if (h_file != 0xffff) {
                lVar1 = mem_op_1000_0a48(0x1, *param_2, (*param_2 >> 0x10), _PTR_LOOP_1050_5f2c);
                lVar1 = (lVar1 >> 0x10);
                param_3 = lVar1;
                (param_3 + 0x2) = lVar1;
                param_5 = lVar1;
                if ((lVar1 | param_3) != 0) {
                    iStack56 = WIN16_hread(*param_2, *param_3, h_file);
                    uVar2 = (iStack56 >> 0x10);
                    _lclose16(h_file);
                    pbStack60 = *param_3;
                    while (iStack56 != 0) {
                        if ((*(*pbStack60 + 0x608b) & 0x20) == 0) {
                            *pbStack60 = *pbStack60 + 0x80;
                        }
                        pbStack60 = (pbStack60 & 0xffff0000 | (pbStack60 + 1));
                        iStack56 = iStack56 -0x1;
                    }
                    return uVar2;
                }
            }
        }
    }
    return param_5;
}
