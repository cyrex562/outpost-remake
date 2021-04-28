pub fn close_file_1008_496c(in_AStruct7: *mut AStruct7) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_BX_5: *mut AStruct7;
    let mut local_res6: u16;
    let temp_86216c208fd: *mut u32;
    let mut func_ptr: u32;
    let mut temp_5f096a4ace: u32;

    *_in_AStruct7 = &PTR_LOOP_1050_4c4c;
    in_AStruct7.u16_field_2 = &PTR_LOOP_1050_1008;
    puVar1 = in_AStruct7.func_ptr_0x4;
    uVar2 = in_AStruct7.i_field_4;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            func_ptr = *puVar1;
            (**func_ptr)();
        }
    }
    error_check_1000_17ce(in_AStruct7.file_path);
    if (&in_AStruct7.pv_buffer_0x1a != 0) {
        temp_5f096a4ace = &in_AStruct7.pv_buffer_0x1a;
        error_check_1000_0dc6(temp_5f096a4ace, (temp_5f096a4ace >> 0x10));
    }
    if (in_AStruct7.file != 0xffff) {
        _lclose16(in_AStruct7.file);
    }
    *_in_AStruct7 = s_1_1050_389a;
    in_AStruct7.u16_field_2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn read_from_file_1008_49e8(in_AStruct7: *mut AStruct7) {
    let paVar1: *mut AStruct131;
    let mut uVar2: u32;
    let mut file_handle: u16;
    let pvVar3: *mut void;
    let mut iVar4: i32;
    let paVar5: *mut AStruct131;
    let mut AStruct131_4: u16;
    let mut uVar6: u32;
    let mut extraout_DX: i32;
    let struct_a: *mut AStruct199;
    let mut extraout_DX_00: i32;
    let local_BX_4: *mut AStruct8;
    let mut unaff_SS: u16;
    let mut bytes_read: u32;
    let lVar8: u32;
    let mut in_stack_00000006: i32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut read_count: u32;
    let mut uVar7: u32;

    if (in_AStruct7.file_path != 0x0) {
        if (in_AStruct7.u16_field_0x1e != 0) {
            return;
        }
        if (in_AStruct7.file == 0xffff) {
            file_handle = _lopen16(0, in_AStruct7.file_path);
            in_AStruct7.file = file_handle;
            if (file_handle == 0xffff) {
                return;
            }
        }
        read_count = 0;
        bytes_read = _hread(0xe, CONCAT22(unaff_SS, &local_18), in_AStruct7.file);
        if ((bytes_read == 0xe) && ((bytes_read >> 0x10) == 0)) {
            read_count = local_16;
            if (local_18 == &PTR_LOOP_1050_4d42) {
                lVar8 = _llseek16(0, 0, in_AStruct7.file);
                pvVar3 = lVar8;
                local_1c = __g_AStruct94_ptr_1;
                local_1a = (__g_AStruct94_ptr_1 >> 0x10);
                alloc_mem_1000_0a48(1, read_count, (read_count >> 0x10), local_1c, local_1a);
                in_AStruct7.pv_buffer_0x1a = pvVar3;
                in_AStruct7.u16_field_0x1c = extraout_DX;
                if ((extraout_DX | in_AStruct7.pv_buffer_0x1a) == 0) {
                    return;
                }
                // WARNING: Load size is inaccurate
                lVar8 = _hread(read_count, in_AStruct7.pv_buffer_0x1a, in_AStruct7.file);
                struct_a = (lVar8 >> 0x10);
                local_a = lVar8;
                local_18 = in_AStruct7.file;
                local_8 = struct_a;
                _lclose16(local_18);
                in_AStruct7.file = 0xffff;
                in_AStruct7.u16_field_0x1e = 1;
                // WARNING: Load size is inaccurate
                in_AStruct7.pv_field_0xe = in_AStruct7.pv_buffer_0x1a;
                uVar6 = &in_AStruct7.pv_buffer_0x1a;
                iVar4 = uVar6;
                uVar6 = uVar6 & 0xffff0000;
                in_AStruct7.pv_field_0x12 = (uVar6 | iVar4 + 0xe);
                uVar6 = uVar6 | iVar4 + 0x436;
                in_AStruct7.u32_field_0x16 = uVar6;
                local_16 = CONCAT22(local_16._2_2_, 0x14);
                local_18 = offset;
                process_struct_1000_179c(0x14, struct_a);
                paVar5 = uVar6;
                if ((struct_a | paVar5) == 0) {
                    &in_AStruct7.func_ptr_0x4 = 0;
                } else {
                    local_16 = CONCAT22(local_16._2_2_, 0x100);
                    paVar1 = in_AStruct7.pv_field_0x12;
                    AStruct131_4 = paVar1;
                    AStruct131_4 = AStruct131_4 + 0x28;
                    uVar2 = paVar1 & 0xffff0000;
                    uVar7 = uVar2 | AStruct131_4;
                    local_18 = (uVar2 >> 0x10);
                    process_struct_1008_4c98((uVar6 & 0xffff | AStruct131_4 << 0x10), uVar7, 0x100);
                    paVar5 = uVar7;
                    in_AStruct7.func_ptr_0x4 = paVar5;
                    in_AStruct7.i_field_4 = extraout_DX_00;
                }
                if (in_AStruct7.pv_field_0x22 != 0) {
                    local_16 = local_16 & 0xffff0000 | in_stack_00000006;
                    local_18 = in_AStruct7;
                    pass1_1008_4b8e(in_AStruct7, paVar5);
                    return;
                }
                return;
            }
        }
        _lclose16(in_AStruct7.file);
        in_AStruct7.file = 0xffff;
    }
    return;
}

pub fn close_file_1008_4c26(param_1: *mut AStruct7, param_2: u8) -> *mut AStruct7 {
    close_file_1008_496c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn file_fn_1008_6414(in_AStruct117: *mut AStruct117, param_2: u32) {
    let ppcVar1: fn();
    let in_struct_a: *mut AStruct103;
    let paVar2: *mut AStruct7;
    let struct_a: *mut AStruct199;
    let paVar3: *mut AStruct199;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let local_AStruct117: *mut AStruct117;
    let mut local_ES_4: u16;
    let mut local_SS__1: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let local_26: u8;

    local_ES_4 = (in_AStruct117 >> 0x10);
    local_AStruct117 = in_AStruct117;
    in_AStruct117 = 0;
    &local_AStruct117.field_0x4 = 0;
    paVar2 = &local_26;
    process_struct_1008_48fe(
        CONCAT13((local_SS__1 >> 8), CONCAT12(local_SS__1, paVar2)),
        1,
        param_2,
    );
    paVar3 = struct_a;
    process_struct_1000_179c(0x1e, struct_a);
    if ((paVar3 | paVar2) == 0) {
        in_AStruct117 = 0;
    } else {
        in_struct_a = CONCAT22(paVar3, paVar2);
        paVar2 = &local_26;
        pass1_1008_3f92(in_struct_a, CONCAT22(local_SS__1, &local_26));
        in_AStruct117 = paVar2;
        local_AStruct117.field_0x2 = extraout_DX;
    }
    ppcVar1 = (in_AStruct117 + 0x14);
    (**ppcVar1)(0x1000, in_AStruct117);
    local_AStruct117.field_0x4 = paVar2;
    local_AStruct117.field_0x6 = extraout_DX_00;
    close_file_1008_496c(&local_26);
    return;
}

pub fn close_file_1008_6dd0(param_1: *mut AStruct9) {
    let local_BX_4: *mut AStruct9;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.file != 0xffff) {
        _lclose16(local_BX_4.file);
        local_BX_4.file = 0xffff;
    }
    error_check_1000_17ce(param_1);
    return;
}
pub fn write_to_file_1008_6e02(param_1: *mut file_object) -> u16 {
    let BVar1: bool;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_4: [u8; 2];

    g_u16_1050_0310 = 0;
    BVar1 = write_to_file_1008_70a6(param_1);
    if (BVar1 != 0) {
        uVar4 = (param_1 >> 0x10);
        uVar3 = param_1;
        set_array_val_1008_72a8(CONCAT22(unaff_SS, local_4), (uVar3 + 4));
        iVar2 = pass1_1008_7006(uVar3, uVar4, CONCAT22(unaff_SS, local_4));
        if ((iVar2 != 0)
            && (
                iVar2 = pass1_1008_6eee(uVar3, uVar4, CONCAT22(unaff_SS, local_4)),
                iVar2 != 0,
            ))
        {
            BVar1 = close_file_1008_726c(param_1);
            if (BVar1 == 0) {
                return 0;
            }
            return 1;
        }
        _lclose16((uVar3 + 4));
    }
    return 0;
}

pub fn close_file_1008_6e78(param_1: *mut void) -> bool {
    let BVar1: bool;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_4: [u8; 2];

    g_u16_1050_0310 = 0;
    BVar1 = read_from_file_1008_7146(param_1);
    if (BVar1 != 0) {
        uVar4 = (param_1 >> 0x10);
        uVar3 = param_1;
        set_array_val_1008_72a8(CONCAT22(unaff_SS, local_4), (uVar3 + 4));
        iVar2 = pass1_1008_7056(uVar3, uVar4, CONCAT22(unaff_SS, local_4));
        if ((iVar2 != 0)
            && (
                iVar2 = pass1_1008_6f7a(uVar3, uVar4, CONCAT22(unaff_SS, local_4)),
                iVar2 != 0,
            ))
        {
            BVar1 = close_file_1008_726c(param_1);
            if (BVar1 == 0) {
                return 0;
            }
            return 1;
        }
        _lclose16((uVar3 + 4));
    }
    return 0;
}
pub fn write_to_file_1008_70a6(param_1: *mut file_object) -> bool {
    let mut local_file: u16;
    let mut count: u16;
    let local_BX_6: *mut file_object;
    let mut uVar1: u16;
    let mut file_handle: u16;
    let mut bytes_written: u32;

    uVar1 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    if (local_BX_6.file != 0xffff) {
        _lclose16(local_BX_6.file);
        local_BX_6.file = 0xffff;
    }
    local_file = _lcreat16(0, param_1.path);
    local_BX_6.file = local_file;
    if (local_file == 0xffff) {
        g_u16_1050_0310 = 0x6cf;
    } else {
        PTR_LOOP_1050_0312 = &PTR_DAT_0005_0000_1050_0004;
        string_fn_1000_3f9c(
            s__1050_65a0,
            &g_alloc_addr_1050_1050,
            _PTR_s_SC_03d_1050_0314_1050_031c,
            (_PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),
            &PTR_DAT_0005_0000_1050_0004,
        );
        count = get_string_index_1000_3da4(s__1050_65a0);
        bytes_written = _hwrite16(count, s__1050_65a0, local_BX_6.file);
        if (bytes_written == count) {
            return 1;
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

// WARNING: Restarted to delay deadcode elimination for space: stack

pub fn read_from_file_1008_7146(param_1: *mut file_object) -> bool {
    let mut file: u16;
    let BVar1: bool;
    let local_file_obj: *mut file_object;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    local_file_obj = param_1;
    if (local_file_obj.file != 0xffff) {
        _lclose16(local_file_obj.file);
        local_file_obj.file = 0xffff;
    }
    file = _lopen16(0, param_1.path);
    local_file_obj.file = file;
    if (file == 0xffff) {
        g_u16_1050_0310 = 0x6cf;
    } else {
        BVar1 = read_file_func_1008_71a0((param_1 & 0xffff));
        if (BVar1 != 0) {
            return 1;
        }
    }
    return 0;
}

// WARNING: Removing unreachable block (ram,0x100871e6)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn read_file_func_1008_71a0(in_file_object: *mut file_object) -> bool {
    let mut count: u16;
    let mut unaff_SS: u16;
    let mut bytes_read: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 9];
    let local_5: u8;
    let mut local_4: u16;

    local_4 = 1;
    count = get_string_index_1000_3da4(s__1050_65a0);
    local_16 = 0;
    bytes_read = _hread(count, CONCAT22(unaff_SS, local_e), (in_file_object + 4));
    bytes_read._0_2_ = bytes_read;
    if (count < bytes_read) {
        bytes_read._0_2_ = count;
    }
    local_18 = bytes_read - 2;
    if (local_18 < 0) {
        local_18 = 0;
    }
    local_1a = 2;
    while (local_18 != 0) {
        local_16 = local_16 * 10 + local_e[local_1a] + -0x30;
        local_1a = local_1a + 1;
        local_18 = local_18 - 1;
    }
    if (local_16 == 1) {
        PTR_LOOP_1050_0312 = (&PTR_LOOP_1050_0000 + 1);
    } else {
        if (local_16 == 4) {
            PTR_LOOP_1050_0312 = &PTR_DAT_0005_0000_1050_0004;
        } else {
            local_5 = '\0';
            PTR_LOOP_1050_0312 = (&PTR_LOOP_1050_0000 + 1);
            local_4 = 0;
        }
    }
    string_fn_1000_3f9c(
        s__1050_65a0,
        &g_alloc_addr_1050_1050,
        _PTR_s_SC_03d_1050_0314_1050_031c,
        (_PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),
        PTR_LOOP_1050_0312,
    );
    return local_4;
}

pub fn close_file_1008_726c(param_1: *mut AStruct11) -> bool {
    let mut file_handle: u16;
    let local_BX_4: *mut AStruct11;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (local_BX_4.file != 0xffff) {
        file_handle = _lclose16(local_BX_4.file);
        if (file_handle == 0xffff) {
            g_u16_1050_0310 = 0x6d1;
            return 0;
        }
        local_BX_4.file = 0xffff;
        g_u16_1050_0310 = 0;
    }
    return 1;
}

pub fn read_file_1008_7548(param_1: *mut HFILE16, param_2: *mut long) {
    let ppcVar1: fn();
    let BVar2: bool;
    let mut uVar3: i32;
    let mut uVar4: u32;
    let mut local_DX_119: u16;
    let mut local_CS__1: u16;
    let mut local_SS__1: u16;
    let lVar5: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    BVar2 = read_file_1008_7dee(param_1, CONCAT22(local_SS__1, &local_6), 4);
    if (BVar2 == 0) {
        return;
    }
    if (local_6 != 0) {
        uVar4 = local_6;
        if (local_6 < 200) {
            local_6._2_2_ = 0x0;
            uVar4 = 200;
        }
        uVar3 = uVar4;
        _local_a = uVar4 & 0xffff | ZEXT24(local_6._2_2_) << 0x10;
        let param_2_val = unsafe { *param_2 };
        if (param_2_val == 0) {
            local_CS__1 = 0x1000;
            process_struct_1000_179c(0x1e, local_6._2_2_);
            if ((local_6._2_2_ | uVar3) == 0) {
                unsafe { *param_2 = 0 };
            } else {
                local_CS__1 = 0x1020;
                pass1_1020_c444(CONCAT22(local_6._2_2_, uVar3), 100, _local_a);
                param_2 = uVar3;
                (param_2 + 2) = local_DX_119;
            }
        }
        unsafe {
            lVar5 = *param_2;
            ppcVar1 = (*param_2 + 0x24);
            (**ppcVar1)();
        }
        local_e = 0;
        while (local_e < local_6) {
            BVar2 = read_file_1008_7dee(param_1, CONCAT22(local_SS__1, &local_1c), 4);
            if ((BVar2 == 0)
                || (
                    BVar2 = read_file_1008_7dee(param_1, CONCAT22(local_SS__1, &local_18), 2),
                    BVar2 == 0,
                ))
            {
                unsafe {
                    ppcVar1 = (*param_2 + 0x1c);
                    (**ppcVar1)(local_CS__1, *param_2);
                }
                return;
            }

            unsafe {
                ppcVar1 = (*param_2 + 0x28);
                (**ppcVar1)(
                    local_CS__1,
                    *param_2,
                    (*param_2 >> 0x10),
                    local_18,
                    local_1c,
                );
            }
            local_e = local_e + 1;
        }

        unsafe {
            ppcVar1 = (*param_2 + 0x1c);
            (**ppcVar1)(local_CS__1, *param_2, lVar5);
        }
    }
    return;
}

pub fn read_file_1008_76e4(param_1: *mut HFILE16, param_2: *mut long) {
    let ppcVar1: fn();
    let mut uVar2: i32;
    let BVar3: bool;
    let in_DX: *mut AStruct199;
    let mut local_DX_93: u16;
    let mut local_SS__1: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = 0;
    uVar2 = read_file_1008_7dee(param_1, CONCAT22(local_SS__1, &local_6), 4);
    if (uVar2 == 0) {
        return;
    }
    if (local_6 != 0) {
        let param_2_val = unsafe { *param_2 };
        if (val == 0) {
            process_struct_1000_179c(0x18, in_DX);
            if ((in_DX | uVar2) == 0) {
                unsafe {
                    *param_2 = 0;
                }
            } else {
                pass1_1030_1cd8(CONCAT22(in_DX, uVar2), 5, local_6);
                param_2 = uVar2;
                (param_2 + 2) = local_DX_93;
            }
        }
        let param_2_val = unsafe { *param_2 };
        ppcVar1 = (param_2_val + 0x14);
        (**ppcVar1)();
        local_a = 0;
        while (local_a < local_6) {
            BVar3 = read_file_1008_7dee(param_1, CONCAT22(local_SS__1, &local_18), 4);
            if (BVar3 == 0) {
                return;
            }
            let param_2_val = unsafe { *param_2 };
            ppcVar1 = (param_2_val + 0x18);
            (**ppcVar1)();
            local_a = local_a + 1;
        }
        let param_2_val = unsafe { *param_2 };
        ppcVar1 = (param_2_val + 0x1c);
        (**ppcVar1)();
    }
    return;
}

pub fn read_file_1008_77cc(in_hfile: *mut HFILE16, param_2: *mut long) -> u16 {
    let mut uVar1: i32;
    let BVar2: bool;
    let in_DX: *mut AStruct199;
    let mut local_SS__1: u16;
    let mut local_DXAX_90: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    uVar1 = read_file_1008_7dee(in_hfile, CONCAT22(local_SS__1, &local_4), 2);
    if (uVar1 == 0) {
        return 0;
    }
    if (local_4 != 0) {
        let param_2_val = unsafe { *param_2 };
        if (param_2_val == 0) {
            process_struct_1000_179c(10, in_DX);
            if ((in_DX | uVar1) == 0) {
                unsafe { *param_2 = 0 };
            } else {
                local_DXAX_90 = pass1_1020_ba3e(CONCAT22(in_DX, uVar1), 5, 5);
                param_2 = local_DXAX_90;
                (param_2 + 2) = (local_DXAX_90 >> 0x10);
            }
        }
        local_6 = 0;
        while (local_6 < local_4) {
            BVar2 = read_file_1008_7dee(in_hfile, CONCAT22(local_SS__1, &local_14), 2);
            if (BVar2 == 0) {
                return 0;
            }
            BVar2 = read_file_1008_7dee(in_hfile, CONCAT22(local_SS__1, &local_10), 4);
            if (BVar2 == 0) {
                return 0;
            }
            let param_2_val = unsafe { *param_2 };
            pass1_1020_bb8a(
                param_2_val,
                (param_2_val >> 0x10),
                local_10,
                (local_10 >> 0x10),
                local_14,
            );
            local_6 = local_6 + 1;
        }
    }
    return 1;
}

// WARNING: Could not reconcile some variable overlaps

pub fn write_to_file_1008_7898(in_file: u32, param_2: *mut u32) {
    let ppcVar1: fn();
    let mut local_AX__1: u16;
    let BVar2: bool;
    let mut local_DX_19: u16;
    let mut uvar3: u16;
    let mut local_SS__1: u16;
    let mut local_26: u16;
    let mut local_24: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0x0) {
        local_AX__1 = 0;
        uVar3 = 0;
    } else {
        let param_2_val = unsafe { *param_2 };
        ppcVar1 = (param_2_val + 0x10);
        (**ppcVar1)();
        uVar3 = local_DX_19;
    }
    _local_6 = CONCAT22(uVar3, local_AX__1);
    _local_18 = CONCAT22(uVar3, local_AX__1);
    BVar2 = write_to_file_1008_7e1c(in_file, CONCAT22(local_SS__1, &local_18), 4);
    if (BVar2 != 0) {
        local_a = 0;
        while (true) {
            if (_local_6 <= local_a) {
                return;
            }
            pass1_1020_c4a8(
                param_2,
                CONCAT22(local_SS__1, &local_14),
                CONCAT22(local_SS__1, &local_18),
                local_a,
            );
            local_24 = _local_18;
            BVar2 = write_to_file_1008_7e1c(in_file, CONCAT22(local_SS__1, &local_24), 4);
            if (BVar2 == 0) {
                break;
            }
            local_26 = local_14;
            BVar2 = write_to_file_1008_7e1c(in_file, CONCAT22(local_SS__1, &local_26), 2);
            if (BVar2 == 0) {
                g_u16_1050_0310 = 0x6d0;
                return;
            }
            local_a = local_a + 1;
        }
    }
    g_u16_1050_0310 = 0x6d0;
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn write_to_file_1008_7954(param_1: u32, param_2: *mut u32) {
    let ppcVar1: fn();
    let mut in_AX: u16;
    let BVar2: bool;
    let mut uVar3: u32;
    let mut local_DX_19: u16;
    let mut extraout_DX: u16;
    let mut local_SS__1: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0x0) {
        in_AX = 0;
        local_16 = 0;
    } else {
        let param_2_val = unsafe { *param_2 };
        ppcVar1 = (param_2_val + 0x10);
        (**ppcVar1)();
        local_16 = local_DX_19;
    }
    _local_6 = CONCAT22(local_16, in_AX);
    local_18 = in_AX;
    BVar2 = write_to_file_1008_7e1c(param_1, CONCAT22(local_SS__1, &local_18), 4);
    if (BVar2 != 0) {
        local_a = 0;
        while (true) {
            if (_local_6 <= local_a) {
                return;
            }
            let param_2_val = unsafe { *param_2 };
            ppcVar1 = (param_2_val + 4);
            uVar3 = _local_6;
            (**ppcVar1)();
            local_20 = uVar3;
            local_1e = extraout_DX;
            local_18 = local_20;
            local_16 = extraout_DX;
            BVar2 = write_to_file_1008_7e1c(param_1, CONCAT22(local_SS__1, &local_20), 4);
            if (BVar2 == 0) {
                break;
            }

            local_a = local_a + 1;
        }
    }
    g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn write_to_file_1008_79f0(in_file: u32, param_2: libc::c_long) {
    let mut local_AX_28: u16;
    let mut local_ES_11: u16;
    let mut local_4: u16;

    if (param_2 == 0) {
        local_AX_28 = 0;
        local_4 = 0;
    } else {
        local_ES_11 = (param_2 >> 0x10);
        local_AX_28 = (param_2 + 4);
        local_4 = (param_2 + 6);
    }
    write_to_file_1008_7954(in_file, CONCAT22(local_4, local_AX_28));
    return;
}

pub fn write_to_file_1008_7a22(param_1: *mut HFILE16, param_2: *mut *mut u8) {
    let BVar1: bool;
    let mut local_ES_9: u16;
    let mut local_SS__1: u16;
    let mut local_24: u32;
    let mut local_1c: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0x0) {
        local_4 = 0;
    } else {
        local_4 = (param_2 + 4);
    }
    local_12 = local_4;
    BVar1 = write_to_file_1008_7e1c(param_1, CONCAT22(local_SS__1, &local_12), 2);
    if (BVar1 == 0) {
        g_u16_1050_0310 = 0x6d0;
    } else {
        local_6 = 0;
        while (true) {
            if (local_4 <= local_6) {
                return;
            }
            pass1_1020_bb16(
                param_2,
                CONCAT22(local_SS__1, &local_10),
                CONCAT22(local_SS__1, &local_12),
                local_6,
            );
            local_a = local_12;
            local_1c = local_12;
            BVar1 = write_to_file_1008_7e1c(param_1, CONCAT22(local_SS__1, &local_1c), 2);
            if (BVar1 == 0) {
                break;
            }
            local_24 = local_10;
            BVar1 = write_to_file_1008_7e1c(param_1, CONCAT22(local_SS__1, &local_24), 4);
            if (BVar1 == 0) {
                return;
            }
            local_6 = local_6 + 1;
        }
    }
    return;
}

pub fn read_file_1008_7ad4(in_file: *mut HFILE16, param_2: u32) -> u16 {
    let BVar1: bool;
    let mut local_SS__1: u16;
    let mut local_14: u16;
    let mut local_10: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    BVar1 = read_file_1008_7dee(in_file, CONCAT22(local_SS__1, &local_4), 2);
    if (BVar1 != 0) {
        local_6 = 0;
        while (true) {
            if (local_4 <= local_6) {
                return 1;
            }
            BVar1 = read_file_1008_7dee(in_file, CONCAT22(local_SS__1, &local_14), 2);
            if ((BVar1 == 0)
                || (
                    BVar1 = read_file_1008_7dee(in_file, CONCAT22(local_SS__1, &local_10), 4),
                    BVar1 == 0,
                ))
            {
                break;
            }
            pass1_1020_bb8a(param_2, local_10, (local_10 >> 0x10), local_14);
            local_6 = local_6 + 1;
        }
    }
    return 0;
}

pub fn write_file_1008_7b4c(param_1: *mut HFILE16, param_2: *mut u8) -> u16 {
    let BVar1: bool;
    let mut local_SS__1: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3eb4(
        param_2,
        CONCAT22(local_SS__1, &local_8),
        CONCAT22(local_SS__1, &local_6),
        CONCAT22(local_SS__1, &local_4),
    );
    local_12 = local_4;
    BVar1 = write_to_file_1008_7e1c(param_1, CONCAT22(local_SS__1, &local_12), 2);
    if (BVar1 != 0) {
        local_c = local_6;
        BVar1 = write_to_file_1008_7e1c(param_1, CONCAT22(local_SS__1, &local_c), 2);
        if (BVar1 != 0) {
            local_c = local_8;
            BVar1 = write_to_file_1008_7e1c(param_1, CONCAT22(local_SS__1, &local_c), 2);
            if (BVar1 != 0) {
                return 1;
            }
        }
    }
    return 0;
}

pub fn read_file_1008_7bc8(in_file: *mut HFILE16, param_2: *mut u16) -> u16 {
    let BVar1: bool;
    let mut local_SS__1: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    BVar1 = read_file_1008_7dee(in_file, CONCAT22(local_SS__1, &local_6 + 2), 2);
    if (BVar1 != 0) {
        BVar1 = read_file_1008_7dee(in_file, CONCAT22(local_SS__1, &local_6), 2);
        if (BVar1 != 0) {
            BVar1 = read_file_1008_7dee(in_file, CONCAT22(local_SS__1, &local_8), 2);
            if (BVar1 != 0) {
                pass1_1008_3e76(param_2, local_8, local_6, (local_6 >> 0x10));
                return 1;
            }
        }
    }
    return 0;
}

pub fn write_to_file_1008_7c2a(param_1: *mut HFILE16, param_2: *mut char) -> bool {
    let mut uVar1: i32;
    let BVar2: bool;

    if (param_2 != 0x0) {
        uVar1 = get_string_index_1000_3da4(param_2);
        BVar2 = write_to_file_1008_7e1c(param_1, param_2, (uVar1 + 1));
        return BVar2;
    }
    write_to_file_1008_7e1c(param_1, s_playerName_1050_148e + 0xc, 1);
    return 1;
}

pub fn read_file_into_str_1008_7c6e(p_file_handle: *mut HFILE16, out_str: *mut char) {
    let pcVar1: *mut libc::c_char;
    let mut unaff_SS: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let local_c: u8;

    while (true) {
        pcVar1 = out_str;
        // read 1 byte i32o a stack buffer
        let p_file_handle_val = unsafe { *p_file_handle };
        _hread(1, CONCAT22(unaff_SS, &local_c), p_file_handle_val);
        // read until the terminator is reached; this might overflow if the string is
        // longer than 9 bytes
        if (local_c == '\0') {
            break;
        }
        // advance the p i32er param2 by 1
        out_str = (out_str & 0xffff0000 | (out_str + 1));
        unsafe { *pcVar1 = local_c };
    }
    unsafe { *out_str = '\0' };
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn write_to_file_1008_7cac(param_1: *mut HFILE16, param_2: u16) -> u8 {
    let mut buf_size: i32;
    let BVar1: bool;
    let local_SS__1: *mut libc::c_char;
    let mut char_buf: [u8; 10];

    string_fn_1000_3f9c(
        char_buf,
        local_SS__1,
        s__s_02x_1050_0340,
        &g_alloc_addr_1050_1050,
        _PTR_s_dcbSC_1050_0336_1050_033c,
    );
    buf_size = get_string_index_1000_3da4(CONCAT22(local_SS__1, char_buf));
    BVar1 = write_to_file_1008_7e1c(param_1, CONCAT22(local_SS__1, char_buf), buf_size);
    if (BVar1 == 0) {
        g_u16_1050_0310 = 0x6d0;
        return '\0';
    }
    return 0x1;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn read_file_1008_7cfe(param_1: *mut HFILE16) -> u8 {
    let mut bVar1: bool;
    let BVar2: bool;
    let mut unaff_SS: u16;
    let mut bytes_read: u32;
    let mut local_410: u32;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    // let mut local_406: [u8;1024];
    let mut local_406: [u8; 1024];
    let mut local_6: u32;

    local_6 = 0;
    bVar1 = false;
    loop {
        let param_1_val = unsafe { *param_1 };
        _llseek16(0, local_6, param_1_val);
        bytes_read = _hread(0x400, CONCAT22(unaff_SS, local_406), param_1_val);
        local_410 = 0;
        while (local_410 < bytes_read) {
            if (local_406[local_410] == *_PTR_s_dcbSC_1050_0336_1050_033c) {
                if (!bVar1) {
                    bVar1 = true;
                    local_6 = CONCAT22(
                        (local_6 >> 0x10) + local_410._2_2_ + CARRY2(local_6, local_410),
                        local_6 + local_410,
                    );
                    break;
                }
                bVar1 = false;
                BVar2 = process_string_1008_7e4a();
                if (BVar2 != 0) {
                    _llseek16(0, local_410 + local_6 + 7, param_1_val);
                    return 0x1;
                }
            }
            local_410 = local_410 + 1;
        }
        if (!bVar1) {
            if (bytes_read < 0x400) {
                return '\0';
            }
            local_6._0_2_ = CONCAT11(local_6._1_1_ + 4, local_6);
            local_6 = CONCAT22((local_6 >> 0x10) + (0xfb < local_6._1_1_), local_6);
        }
    }
}

pub fn read_file_1008_7dee(
    p_in_file_handle: *mut HFILE16,
    buffer: *mut char,
    count: libc::c_long,
) -> bool {
    let mut bytes_read: u32;
    let p_in_file_handle_val = unsafe { *p_in_file_handle };
    bytes_read = _hread(count, buffer, p_in_file_handle_val);
    if ((bytes_read == count) && ((bytes_read >> 0x10) == count._2_2_)) {
        return 1;
    }
    return 0;
}

pub fn write_to_file_1008_7e1c(
    file: *mut HFILE16,
    buffer: &mut string,
    count: libc::c_long,
) -> bool {
    let mut bytes_written: u32;
    let file_val = unsafe { *file };
    bytes_written = _hwrite16(count, buffer, file_val);
    if ((bytes_written == count) && ((bytes_written >> 0x10) == count._2_2_)) {
        return 1;
    }
    return 0;
}

pub fn read_file_1008_bb5e(in_AStruct120: &mut AStruct120, file_handle_ptr: &mut HFILE16) {
    let ppcVar1: fn();
    let mut b_read_result_1: u16;
    let b_read_result_2: bool;
    let paVar3: *mut AStruct199;
    let local_AX_168: *mut AStruct121;
    let local_AX_189: *mut libc::c_char;
    let uVar2: u8;
    let BVar4: bool;
    let pcVar5: *mut libc::c_char;
    let mut uVar6: i32;
    let extraout_var: u32;
    let local_AStruct120: *mut AStruct120;
    let extraout_var_00: u32;
    let in_DX: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let mut uVar7: u16;
    let extraout_DX: *mut AStruct199;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let mut in_stack_0000000a: u16;
    let offset2: u8;
    let base: u8;
    let mut local_13c: u32;
    let mut local_122: u16;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_11c: u16;
    let mut local_11a: u16;
    // let mut local_118: [u8;256];
    let local_118: [u8; 256];
    // let mut two_file_bytes_2: [u8;2];
    let two_file_bytes_2: [u8; 2];
    // let mut two_file_bytes: [u8;2];
    let two_file_bytes: [u8; 2];
    // let mut buffer_1: [u8;4];
    let buffer_1: [u8; 4];
    // let mut four_file_bytes: [u8;4];
    let four_file_bytes: [u8; 4];

    let mut func_ptr_1: u32;

    if (PTR_LOOP_1050_0312 < 2) {
        // bail if the read fails
        return;
    }
    uVar2 = read_file_1008_7cfe(CONCAT22(in_stack_0000000a, file_handle_ptr));
    if (CONCAT31(extraout_var, uVar2) == 0) {
        g_u16_1050_0310 = 0x6d4;
    } else {
        local_AStruct120._0_2_ = in_AStruct120;
        local_AStruct120._0_2_ = local_AStruct120 + 0x22;
        // read two bytes from the file i32o the structure passed in by param_1
        b_read_result_1 = read_file_1008_7dee(
            _file_handle_ptr,
            (in_AStruct120 & 0xffff0000 | local_AStruct120),
            2,
        );
    }
    if ((b_read_result_1 != 0)
        && (
            b_read_result_2 =
                read_file_1008_7dee(_file_handle_ptr, CONCAT22(unaff_SS, buffer_1), 2),
            b_read_result_2 != 0,
        ))
    {
        // read two bytes from the file i32o buffer_1)) {
        // bail if the file read fails
        if (buffer_1._0_2_ == 0) {
            return;
        }
        process_struct_1000_179c(0xc, in_DX);
        struct_a = (in_DX | b_read_result_2);
        if (struct_a == 0x0) {
            paVar3 = 0x0;
            struct_a = 0x0;
        } else {
            paVar3 = process_struct_1008_574a(CONCAT22(in_DX, b_read_result_2));
        }
        uVar9 = (in_AStruct120 >> 0x10);
        (local_AStruct120 + 10) = paVar3;
        (local_AStruct120 + 0xc) = struct_a;
        local_11e = 0;
        while (true) {
            if (buffer_1._0_2_ <= local_11e) {
                return;
            }
            uVar6 = buffer_1._0_2_;
            process_struct_1000_179c(0x12, struct_a);
            uVar8 = struct_a | uVar6;
            if (uVar8 == 0) {
                local_AX_168 = 0x0;
                uVar8 = 0;
            } else {
                uVar2 = pass1_1008_b0bc(CONCAT22(struct_a, uVar6));
                local_AX_168 = CONCAT31(extraout_var_00, uVar2);
            }
            _local_11c = CONCAT22(uVar8, local_AX_168);
            local_AX_189 = local_118;
            uVar7 = uVar8;
            read_file_into_str_1008_7c6e(_file_handle_ptr, CONCAT22(unaff_SS, local_AX_189));
            if (((local_AX_189 == 0x0)
                || (
                    BVar4 = read_file_1008_7dee(
                        _file_handle_ptr,
                        CONCAT22(unaff_SS, two_file_bytes),
                        2,
                    ),
                    BVar4 == 0,
                ))
                || (
                    BVar4 = read_file_1008_7dee(
                        _file_handle_ptr,
                        CONCAT22(unaff_SS, four_file_bytes),
                        4,
                    ),
                    BVar4
                        == 0
                    // read four bytes from file i32o stack buffer)) ||
           (BVar4 = read_file_1008_7dee(_file_handle_ptr,CONCAT22(unaff_SS,two_file_bytes_2) ,2), BVar4 == 0),
                ))
            {
                // read two bytes from file i32o local buffer))
                break;
            }
            pcVar5 = local_118;
            pass1_fn_1008_60e8(pcVar5);
            local_AX_168.field_0x4 = pcVar5;
            local_AX_168.field_0x6 = uVar7;
            local_AX_168.two_file_bytes_0x8 = two_file_bytes;
            local_AX_168.four_file_bytes_0xa = four_file_bytes;
            local_AX_168.two_file_bytes_0xe = two_file_bytes_2;
            func_ptr_1 = ((local_AStruct120 + 10) + 8);
            (**func_ptr_1)();
            local_11e = local_11e + 1;
            struct_a = extraout_DX;
        }
        if (_local_11c != 0x0) {
            ppcVar1 = *_local_11c;
            (**ppcVar1)(0, local_AX_168, uVar8, 1, _local_11c);
        }
    }
    g_u16_1050_0310 = 0x6d2;

    return;
}

pub fn write_to_file_1008_c98e(param_1: u32, param_2: u32) {
    let uVar1: u8;
    let BVar2: bool;
    let extraout_var: u32;
    let mut unaff_SS: u16;
    let mut local_10: u32;

    uVar1 = write_to_file_1008_7cac(param_2, 0x15);
    if (CONCAT31(extraout_var, uVar1) != 0) {
        local_10 = (param_1 + 0xe);
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_10), 4);
        if (BVar2 == 0) {
            g_u16_1050_0310 = 0x6d0;
            return;
        }
    }
    return;
}

pub fn read_file_1008_c9d4(param_1: u32, in_file: *mut HFILE16) {
    let uVar1: u8;
    let BVar2: bool;
    let extraout_var: u32;

    if (1 < PTR_LOOP_1050_0312) {
        uVar1 = read_file_1008_7cfe(in_file);
        if (CONCAT31(extraout_var, uVar1) == 0) {
            g_u16_1050_0310 = 0x6d4;
            return;
        }
        BVar2 = read_file_1008_7dee(in_file, (param_1 & 0xffff0000 | (param_1 + 0xe)), 4);
        if (BVar2 == 0) {
            g_u16_1050_0310 = 0x6d2;
            return;
        }
    }
    return;
}

pub fn write_to_file_1008_e5da(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let uVar2: u8;
    let BVar3: bool;
    let puVar4: *mut u8;
    let extraout_var: u32;
    let mut extraout_DX: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let mut local_30: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_1c: u16;
    let mut local_16: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    //let mut local_c: [u8;8];
    let local_c: [u8; 8];
    let mut local_4: u16;

    uVar2 = write_to_file_1008_7cac(param_2, 0x14);
    if (CONCAT31(extraout_var, uVar2) != 0) {
        uVar6 = (param_1 >> 0x10);
        iVar5 = param_1;
        if ((iVar5 + 10) == 0) {
            local_4 = 0;
        } else {
            uVar1 = (iVar5 + 10);
            local_4 = (uVar1 + 8);
        }
        local_1c = local_4;
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_1c), 2);
        if (BVar3 != 0) {
            pass1_1008_5784(CONCAT22(unaff_SS, local_c), (iVar5 + 10));
            while {
                puVar4 = local_c;
                pass1_1008_5b12(CONCAT22(unaff_SS, puVar4));
                _local_10 = CONCAT22(extraout_DX, puVar4);
                if ((extraout_DX | puVar4) == 0) {
                    return;
                }
                local_24 = (puVar4 + 4);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_24), 4);
                if (BVar3 == 0) {
                    break;
                }
                local_28 = (_local_10 + 8);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_28), 4);
                if (BVar3 == 0) {
                    break;
                }
                local_16 = (_local_10 + 0xc);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_16), 2);
                if (BVar3 == 0) {
                    break;
                }
                local_30 = (_local_10 + 0xe);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_30), 4);
                if (BVar3 == 0) {
                    break;
                }
                local_16 = (_local_10 + 0x12);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_16), 2);
                BVar3 != 0
            } {}
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_38
// WARNING: Could not reconcile some variable overlaps

pub fn read_file_1008_e70e(param_1: u32, in_file_1: *mut HFILE16) {
    let ppcVar1: fn();
    let uVar2: u8;
    let BVar3: bool;
    let mut uVar4: u16;
    let extraout_var: u32;
    let in_DX: *mut AStruct199;
    let mut uVar5: u16;
    let extraout_DX: *mut AStruct199;
    let mut extraout_DX_00: u16;
    let mut unaff_SS: u16;
    let mut local_38: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    if (PTR_LOOP_1050_0312 < 2) {
        return;
    }
    uVar2 = read_file_1008_7cfe(in_file_1);
    if (CONCAT31(extraout_var, uVar2) != 0) {
        BVar3 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_SS, &local_4), 2);
        if (BVar3 != 0) {
            if (local_4 == 0) {
                return;
            }
            local_a = 0;
            while (true) {
                if (local_4 <= local_a) {
                    return;
                }
                uVar4 = local_4;
                process_struct_1000_179c(0x14, in_DX);
                if ((in_DX | uVar4) == 0) {
                    uVar4 = 0;
                    uVar5 = 0;
                } else {
                    process_struct_1008_dcdc(CONCAT22(in_DX, uVar4));
                    uVar5 = extraout_DX_00;
                }
                _local_e = CONCAT22(uVar5, uVar4);
                BVar3 = read_file_1008_7dee(in_file_1, CONCAT22(uVar5, uVar4 + 4), 4);
                if ((((BVar3 == 0)
                    || (
                        BVar3 = read_file_1008_7dee(
                            in_file_1,
                            (_local_e & 0xffff0000 | (_local_e + 8)),
                            4,
                        ),
                        BVar3 == 0,
                    ))
                    || (
                        BVar3 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_SS, &local_12), 2),
                        BVar3 == 0,
                    ))
                    || ((
                        BVar3 = read_file_1008_7dee(
                            in_file_1,
                            (_local_e & 0xffff0000 | (_local_e + 0xe)),
                            4,
                        ),
                        BVar3 == 0
                            || (
                                BVar3 = read_file_1008_7dee(
                                    in_file_1,
                                    (_local_e & 0xffff0000 | (_local_e + 0x12)),
                                    2,
                                ),
                                BVar3 == 0,
                            ),
                    )))
                {
                    break;
                }
                (_local_e + 0xc) = local_12;
                ppcVar1 = ((param_1 + 10) + 4);
                (**ppcVar1)();
                local_a = local_a + 1;
                in_DX = extraout_DX;
            }
            if (_local_e != 0x0) {
                ppcVar1 = *_local_e;
                (**ppcVar1)(0x1000, _local_e, (_local_e >> 0x10), 1, _local_e);
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_file_1010_0ad2(in_struct_1: *mut AStruct235, param_2: u32) {
    let mut u_var1: u32;
    let uVar2: u8;
    let BVar3: bool;
    let pcVar4: *mut libc::c_char;
    let extraout_var: u32;
    let mut extraout_DX: i32;
    let local_BX_26: *mut AStruct325;
    let local_ES_26: *mut u8;
    let local_SS__1: *mut u8;
    let mut local_2a: u32;
    let mut local_22: u16;
    let mut local_1e: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 8];
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = write_to_file_1008_7cac(param_2, 6);
    if (CONCAT31(extraout_var, uVar2) == 0) {
        return;
    }
    local_ES_26 = (in_struct_1 >> 0x10);
    local_BX_26 = in_struct_1;
    if (local_BX_26.field_0xa == 0) {
        local_6 = 0;
    } else {
        uVar1 = local_BX_26.field_0xa;
        local_6 = (uVar1 + 8);
    }
    local_1e = local_6;
    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(local_SS__1, &local_1e), 2);
    if (BVar3 != 0) {
        pass1_1008_5784(CONCAT22(local_SS__1, local_e), local_BX_26.field_0xa);
        while {
            pcVar4 = local_e;
            pass1_1008_5b12(CONCAT22(local_SS__1, pcVar4));
            _local_12 = CONCAT22(extraout_DX, pcVar4);
            if ((extraout_DX | pcVar4) == 0) {
                local_22 = local_BX_26.field_0xe;
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(local_SS__1, &local_22), 2);
                if (BVar3 == 0) {
                    g_u16_1050_0310 = 0x6d0;
                    return;
                }
                local_22 = local_BX_26.field_0x10;
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(local_SS__1, &local_22), 2);
                if (BVar3 == 0) {
                    g_u16_1050_0310 = 0x6d0;
                    return;
                }
                if (local_BX_26.field_0x18 != 0) {
                    u16_1050_0e28 = local_BX_26.field_0x12;
                    PTR_LOOP_1050_0e30 = local_BX_26.field_0x14;
                    PTR_LOOP_1050_0ea8 = local_BX_26.field_0x16;
                }
                local_4 = 0;
                while (true) {
                    if (9 < local_4) {
                        local_4 = 0;
                        while (true) {
                            if (2 < local_4) {
                                if (local_BX_26.field_0x18 != 0) {
                                    u16_1050_0e28 = 0;
                                    PTR_LOOP_1050_0e30 = 0x0;
                                    PTR_LOOP_1050_0ea8 = 0x0;
                                }
                                return;
                            }
                            local_1e = (local_4 * 8 + 0xea8);
                            BVar3 = write_to_file_1008_7e1c(
                                param_2,
                                CONCAT22(local_SS__1, &local_1e),
                                2,
                            );
                            if (BVar3 == 0) {
                                break;
                            }
                            local_4 = local_4 + 1;
                        }
                        g_u16_1050_0310 = 0x6d0;
                        return;
                    }
                    local_1e = (local_4 * 8 + 0xe28);
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(local_SS__1, &local_1e), 2);
                    if (BVar3 == 0) {
                        break;
                    }
                    local_4 = local_4 + 1;
                }
                g_u16_1050_0310 = 0x6d0;
                return;
            }
            local_18 = (pcVar4 + 4);
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(local_SS__1, &local_18), 2);
            if (BVar3 == 0) {
                g_u16_1050_0310 = 0x6d0;
                return;
            }
            local_2a = (_local_12 + 6);
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(local_SS__1, &local_2a), 4);
            BVar3 != 0
        } {}
    }
    g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn read_file_1010_0c7c(param_1: u32, param_2: *mut HFILE16) {
    let ppcVar1: fn();
    let uVar2: u8;
    let BVar3: bool;
    let mut uVar4: u16;
    let BVar5: bool;
    let extraout_var: u32;
    let in_DX: *mut AStruct199;
    let extraout_DX: *mut AStruct199;
    let mut iVar6: i32;
    let mut unaff_SS: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_26: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, uVar2) == 0) {
        g_u16_1050_0310 = 0x6d4;
    } else {
        BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_6), 2);
        if (BVar3 != 0) {
            local_8 = 0;
            while (iVar6 = param_1, local_8 < local_6) {
                uVar4 = local_6;
                process_struct_1000_179c(10, in_DX);
                local_1a = CONCAT22(in_DX, uVar4);
                if ((in_DX | uVar4) == 0) {
                    local_16 = 0;
                } else {
                    local_1a = s_1_1050_389a;
                    (uVar4 + 2) = &PTR_LOOP_1050_1008;
                    local_1a = 0xea8;
                    (uVar4 + 2) = 0x1010;
                    local_16 = local_1a;
                }
                BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_12), 2);
                if ((BVar3 == 0)
                    || (
                        BVar3 = read_file_1008_7dee(
                            param_2,
                            (local_16 & 0xffff0000 | (local_16 + 6)),
                            4,
                        ),
                        BVar3 == 0,
                    ))
                {
                    local_1a = local_16;
                    if (local_16 != 0) {
                        ppcVar1 = local_16;
                        (**ppcVar1)(&PTR_LOOP_1050_1008, local_16, (local_16 >> 0x10), 1);
                    }
                    // goto LAB_1010_0cb1;
                }
                (local_16 + 4) = local_12;
                ppcVar1 = ((iVar6 + 10) + 8);
                (**ppcVar1)();
                local_8 = local_8 + 1;
                in_DX = extraout_DX;
            }
            BVar3 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar6 + 0xe)), 2);
            if ((BVar3 != 0)
                && (
                    BVar3 =
                        read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar6 + 0x10)), 2),
                    BVar3 != 0,
                ))
            {
                local_4 = 0;
                while (local_4 < 10) {
                    BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_2a), 2);
                    if (BVar3 == 0) {
                        // goto LAB_1010_0cb1;
                    }
                    BVar5 = local_4;
                    if (PTR_LOOP_1050_0312 < 2) {
                        switch_statement_1008_738c(param_2, (param_2 >> 0x10), local_4);
                        BVar5 = BVar3;
                    }
                    (BVar5 * 8 + 0xe28) = local_2a;
                    local_4 = local_4 + 1;
                    local_26 = BVar5;
                }
                if (2 < PTR_LOOP_1050_0312) {
                    local_4 = 0;
                    while {
                        BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_2a), 2);
                        if (BVar3 == 0) {}
                        // goto LAB_1010_0cb1;
                        (local_4 * 8 + 0xea8) = local_2a;
                        local_4 = local_4 + 1;
                        local_4 < 3
                    } {}
                }
                return;
            }
        }
        // LAB_1010_0cb1:
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1010_3fc2(in_struct_1: *mut AStruct390, in_file: *mut HFILE16) -> bool {
    let uVar1: u8;
    let extraout_AH: u8;
    let bool_rc: bool;
    let struct_a_lo: *mut AStruct390;
    let struct_a_hi: *mut AStruct390;
    let string_base: *mut libc::c_char;
    let string_offset_a: *mut libc::c_char;
    let string_offset_b: *mut libc::c_char;

    uVar1 = write_to_file_1008_7cac(in_file, 5);
    if (CONCAT11(extraout_AH, uVar1) != 0) {
        struct_a_hi = (in_struct_1 >> 0x10);
        struct_a_lo = in_struct_1;
        string_offset_a = struct_a_lo.string_buf_ptr_1;
        bool_rc = write_to_file_1008_7e1c(in_file, CONCAT22(string_base, &string_offset_a), 2);
        if (bool_rc != 0) {
            string_offset_b = struct_a_lo.string_buf_ptr_2;
            bool_rc = write_to_file_1008_7e1c(in_file, CONCAT22(string_base, &string_offset_b), 2);
            if (bool_rc != 0) {
                string_offset_b = struct_a_lo.string_buf_ptr_3;
                bool_rc =
                    write_to_file_1008_7e1c(in_file, CONCAT22(string_base, &string_offset_b), 2);
                if (bool_rc != 0) {
                    return 1;
                }
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn read_file_1010_404a(param_1: *mut AStruct407, param_2: u32) {
    let uVar1: u8;
    let local_AX_39: *mut AStruct407;
    let BVar2: bool;
    let extraout_var: u32;
    let mut unaff_SS: u16;
    let mut local_4: u16;

    uVar1 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, uVar1) == 0) {
        g_u16_1050_0310 = 0x6d4;
    } else {
        local_AX_39 = param_1;
        local_AX_39 = &local_AX_39.field_0x24;
        BVar2 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | ZEXT24(local_AX_39)), 2);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2);
            if (BVar2 != 0) {
                BVar2 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | ZEXT24(local_AX_39 + 1)),
                    2,
                );
                if (BVar2 != 0) {
                    local_AX_39.field_0x6a = local_4;
                    return;
                }
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn write_to_file_1010_5dc6(in_struct_1: *mut AStruct425, in_file_1: *mut HFILE16) -> bool {
    let mut u_var1: u32;
    let uVar2: u8;
    let extraout_AH: u8;
    let mut iVar3: i32;
    let BVar4: bool;
    let local_BX_26: *mut AStruct425;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_c: u16;
    let mut local_6: u16;

    uVar2 = write_to_file_1008_7cac(in_file_1, 4);
    if (CONCAT11(extraout_AH, uVar2) != 0) {
        uVar5 = (in_struct_1 >> 0x10);
        local_BX_26 = in_struct_1;
        uVar1 = local_BX_26.field_0x68;
        iVar3 = write_to_file_1008_7c2a(in_file_1, uVar1, (uVar1 >> 0x10));
        if (iVar3 != 0) {
            uVar1 = local_BX_26.field_0x6c;
            iVar3 = write_to_file_1008_7c2a(in_file_1, uVar1, (uVar1 >> 0x10));
            if (iVar3 != 0) {
                local_c = u16_1050_13ae;
                BVar4 = write_to_file_1008_7e1c(in_file_1, CONCAT22(unaff_SS, &local_c), 2);
                if (BVar4 != 0) {
                    local_6 = local_BX_26.field_0x82;
                    BVar4 = write_to_file_1008_7e1c(in_file_1, CONCAT22(unaff_SS, &local_6), 2);
                    if (BVar4 != 0) {
                        return 1;
                    }
                }
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn read_file_1010_5e56(param_1: u32, param_2: *mut HFILE16) {
    let uVar1: u8;
    let puVar2: *mut u8;
    let BVar3: bool;
    let extraout_var: u32;
    let mut in_DX: u16;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_404: u16;
    let mut local_402: [u8; 1024];

    uVar1 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, uVar1) == 0) {
        g_u16_1050_0310 = 0x6d4;
    } else {
        puVar2 = local_402;
        read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_SS, puVar2));
        if (puVar2 != 0x0) {
            puVar2 = local_402;
            pass1_fn_1008_60e8(puVar2);
            uVar5 = (param_1 >> 0x10);
            iVar4 = param_1;
            (iVar4 + 0x68) = puVar2;
            (iVar4 + 0x6a) = in_DX;
            puVar2 = local_402;
            read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_SS, puVar2));
            if (puVar2 != 0x0) {
                puVar2 = local_402;
                pass1_fn_1008_60e8(puVar2);
                (iVar4 + 0x6c) = puVar2;
                (iVar4 + 0x6e) = in_DX;
                BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_404), 2);
                if (BVar3 != 0) {
                    u16_1050_13ae = local_404;
                    if (PTR_LOOP_1050_0312 < 2) {
                        return;
                    }
                    BVar3 =
                        read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar4 + 0x82)), 2);
                    if (BVar3 != 0) {
                        return;
                    }
                }
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_file_1010_6372(param_1: u32, param_2: *mut HFILE16) {
    let uVar1: u8;
    let BVar2: bool;
    let extraout_var: u32;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_10: u32;
    let mut local_8: u32;

    uVar1 = write_to_file_1008_7cac(param_2, 7);
    if (CONCAT31(extraout_var, uVar1) != 0) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        local_10 = (iVar3 + 10);
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_10), 4);
        if (BVar2 != 0) {
            local_8 = (iVar3 + 0xe);
            BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_8), 4);
            if (BVar2 != 0) {
                local_8 = (iVar3 + 0x12);
                BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_8), 4);
                if (BVar2 != 0) {
                    local_8 = (iVar3 + 0x16);
                    BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_8), 4);
                    if (BVar2 != 0) {
                        local_8 = (iVar3 + 0x1a);
                        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_8), 4);
                        if (BVar2 != 0) {
                            local_8 = (iVar3 + 0x1e);
                            BVar2 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_8), 4);
                            if (BVar2 != 0) {
                                local_8 = (iVar3 + 0x22);
                                BVar2 = write_to_file_1008_7e1c(
                                    param_2,
                                    CONCAT22(unaff_SS, &local_8),
                                    4,
                                );
                                if (BVar2 != 0) {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn read_file_1010_648a(param_1: u32, param_2: *mut HFILE16) {
    let uVar1: u8;
    let mut iVar2: i32;
    let BVar3: bool;
    let extraout_var: u32;

    uVar1 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, uVar1) != 0) {
        iVar2 = param_1;
        BVar3 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar2 + 10)), 4);
        if (BVar3 != 0) {
            BVar3 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar2 + 0xe)), 4);
            if (BVar3 != 0) {
                BVar3 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar2 + 0x12)), 4);
                if (BVar3 != 0) {
                    BVar3 =
                        read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar2 + 0x16)), 4);
                    if (BVar3 != 0) {
                        BVar3 = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | (iVar2 + 0x1a)),
                            4,
                        );
                        if (BVar3 != 0) {
                            BVar3 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | (iVar2 + 0x1e)),
                                4,
                            );
                            if (BVar3 != 0) {
                                BVar3 = read_file_1008_7dee(
                                    param_2,
                                    (param_1 & 0xffff0000 | (iVar2 + 0x22)),
                                    4,
                                );
                                if (BVar3 != 0) {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1010_6846(param_1: u32, param_2: *mut HFILE16) {
    let uVar1: u8;
    let mut iVar2: i32;
    let BVar3: bool;
    let extraout_var: u32;
    let mut unaff_SS: u16;
    let mut local_c: u16;

    uVar1 = write_to_file_1008_7cac(param_2, 3);
    if (CONCAT31(extraout_var, uVar1) != 0) {
        iVar2 = param_1;
        BVar3 = write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (iVar2 + 10)), 0x114);
        if (BVar3 != 0) {
            BVar3 =
                write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (iVar2 + 0x11e)), 0x2a);
            if (BVar3 != 0) {
                local_c = (iVar2 + 0x148);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
                if (BVar3 != 0) {
                    return;
                }
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn read_file_1010_68c6(param_1: u32, param_2: *mut HFILE16) {
    let BVar1: bool;
    let uVar2: u8;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let BVar5: bool;
    let BVar6: bool;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let in_DX: *mut AStruct199;
    let extraout_DX: *mut AStruct199;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = read_file_1008_7cfe(param_2);
    iVar3 = CONCAT31(extraout_var, uVar2);
    if (iVar3 == 0) {
        g_u16_1050_0310 = 0x6d4;
        return;
    }
    iVar4 = param_1;
    uVar8 = param_2;
    uVar9 = (param_2 >> 0x10);
    uVar7 = (param_1 >> 0x10);
    if (PTR_LOOP_1050_0312 < 2) {
        uVar10 = 0x102;
        uVar11 = 0;
        process_struct_1000_179c(0x102, in_DX);
        _local_a = CONCAT22(in_DX, iVar3);
        BVar5 = read_file_1008_7dee(param_2, CONCAT22(in_DX, iVar3), CONCAT22(uVar11, uVar10));
        local_12 = _local_a;
        if (BVar5 == 0) {}
        // goto LAB_1010_692c;
        local_4 = 1;
        while {
            switch_statement_1008_73ea(uVar8, uVar9, local_4);
            BVar6 = (local_4 * 2 + iVar3);
            (iVar4 + BVar5 * 2 + 10) = BVar6;
            local_4 = local_4 + 1;
            BVar5 = BVar6;
            local_4 < 0x81
        } {}
        in_DX = extraout_DX;
        uVar2 = error_check_1000_17ce(_local_a);
        BVar5 = CONCAT31(extraout_var_00, uVar2);
    } else {
        BVar5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar4 + 10)), 0x114);
        if (BVar5 == 0) {
            g_u16_1050_0310 = 0x6d2;
            return;
        }
    }
    if (PTR_LOOP_1050_0312 < 2) {
        uVar10 = 0x2a;
        uVar11 = 0;
        process_struct_1000_179c(0x2a, in_DX);
        local_12 = CONCAT22(in_DX, BVar5);
        BVar6 = read_file_1008_7dee(param_2, CONCAT22(in_DX, BVar5), CONCAT22(uVar11, uVar10));
        if (BVar6 == 0) {
            // LAB_1010_692c:
            g_u16_1050_0310 = 0x6d2;
            error_check_1000_17ce((local_12 & 0xffff | ZEXT24(in_DX) << 0x10));
            return;
        }
        local_4 = 0;
        while {
            set_param_3_with_switch_1008_72bc(uVar8, uVar9, local_4);
            BVar1 = (local_4 * 2 + BVar5);
            (iVar4 + BVar6 * 2 + 0x11e) = BVar1;
            local_4 = local_4 + 1;
            BVar6 = BVar1;
            local_4 < 0x15
        } {}
        error_check_1000_17ce(local_12);
    } else {
        BVar5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar4 + 0x11e)), 0x2a);
        if (BVar5 == 0) {
            g_u16_1050_0310 = 0x6d2;
            return;
        }
    }
    BVar5 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_6), 2);
    if (BVar5 == 0) {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    switch_statement_1008_73ea(uVar8, uVar9, local_6);
    (iVar4 + 0x148) = BVar5;
    return;
}

pub fn write_to_file_1010_9900(param_1: u32, param_2: *mut HFILE16) -> bool {
    let mut u_var1: u32;
    let uVar2: u8;
    let extraout_AH: u8;
    let BVar3: bool;
    let mut uVar4: u16;
    let local_BX_26: *mut AStruct470;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let lVar6: u32;
    let mut local_24: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_16: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 8];
    let mut local_4: u16;

    uVar2 = write_to_file_1008_7cac(param_2, 1);
    if (CONCAT11(extraout_AH, uVar2) == 0) {
        return 0;
    }
    uVar5 = (param_1 >> 0x10);
    local_BX_26 = param_1;
    if (local_BX_26.field_0xa == 0) {
        local_4 = 0;
    } else {
        uVar1 = local_BX_26.field_0xa;
        local_4 = (uVar1 + 8);
    }
    local_1c = local_4;
    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_1c), 2);
    if (BVar3 == 0) {
        g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    pass1_1008_5784(CONCAT22(unaff_SS, local_c), local_BX_26.field_0xa);
    while {
        _local_10 = pass1_1008_5b12(CONCAT22(unaff_SS, local_c));
        if (_local_10 == 0) {
            if (local_BX_26.field_0xe == 0) {
                local_24 = 0;
            } else {
                uVar1 = local_BX_26.field_0xe;
                local_24 = (uVar1 + 8);
            }
            local_16 = local_24;
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_16), 2);
            if (BVar3 == 0) {
                g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            pass1_1008_5784(CONCAT22(unaff_SS, local_c), local_BX_26.field_0xe);
            while {
                lVar6 = pass1_1008_5b12(CONCAT22(unaff_SS, local_c));
                uVar4 = (lVar6 >> 0x10);
                if (lVar6 == 0) {
                    if (local_BX_26.field_0x12 == 0) {
                        local_24 = 0;
                    } else {
                        uVar1 = local_BX_26.field_0x12;
                        local_24 = (uVar1 + 8);
                    }
                    local_16 = local_24;
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_16), 2);
                    if (BVar3 == 0) {
                        g_u16_1050_0310 = 0x6d0;
                        return 0;
                    }
                    pass1_1008_5784(CONCAT22(unaff_SS, local_c), local_BX_26.field_0x12);
                    while {
                        lVar6 = pass1_1008_5b12(CONCAT22(unaff_SS, local_c));
                        uVar4 = (lVar6 >> 0x10);
                        if (lVar6 == 0) {
                            local_1c = local_BX_26.field_0x1a;
                            BVar3 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_1c), 2);
                            if (BVar3 == 0) {
                                g_u16_1050_0310 = 0x6d0;
                                return 0;
                            }
                            local_1c = local_BX_26.field_0x1c;
                            BVar3 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_1c), 2);
                            if (BVar3 == 0) {
                                g_u16_1050_0310 = 0x6d0;
                                return 0;
                            }
                            local_1c = local_BX_26.field_0x1e;
                            BVar3 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_1c), 2);
                            if (BVar3 == 0) {
                                g_u16_1050_0310 = 0x6d0;
                                return 0;
                            }
                            return 1;
                        }
                        _local_10 = _local_10 & 0xffff0000 | *(lVar6 + 4);
                        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_10), 2);
                        if (BVar3 == 0) {
                            g_u16_1050_0310 = 0x6d0;
                            return 0;
                        }
                        local_4 = (lVar6 + 6);
                        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_4), 2);
                        BVar3 != 0
                    } {}
                    g_u16_1050_0310 = 0x6d0;
                    return 0;
                }
                _local_10 = _local_10 & 0xffff0000 | *(lVar6 + 4);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_10), 2);
                if (BVar3 == 0) {
                    g_u16_1050_0310 = 0x6d0;
                    return 0;
                }
                local_4 = (lVar6 + 6);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_4), 2);
                BVar3 != 0
            } {}
            g_u16_1050_0310 = 0x6d0;
            return 0;
        }
        local_16 = (_local_10 + 4);
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_16), 2);
        if (BVar3 == 0) {
            g_u16_1050_0310 = 0x6d0;
            return 0;
        }
        local_16 = (_local_10 + 6);
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_16), 2);
        BVar3 != 0
    } {}
    g_u16_1050_0310 = 0x6d0;
    return 0;
}

// WARNING: Could not reconcile some variable overlaps

pub fn read_file_1010_9b72(param_1: u32, param_2: *mut HFILE16) {
    let ppcVar1: fn();
    let uVar2: u8;
    let struct_a_2: *mut AStruct199;
    let paVar3: *mut AStruct199;
    let b_result: bool;
    let local_AX_165: *mut AStruct473;
    let struct_b: *mut AStruct1174;
    let mut uVar4: i32;
    let local_AX_571: *mut AStruct472;
    let mut uVar5: u16;
    let extraout_var: u32;
    let struct_a_1: *mut AStruct199;
    let paVar6: *mut AStruct199;
    let extraout_DX: *mut AStruct199;
    let extraout_DX_00: *mut AStruct199;
    let extraout_DX_01: *mut AStruct199;
    let local_BX_60: *mut AStruct471;
    let puVar7: *mut u32;
    let local_ES_60: *mut u8;
    let local_ES_165: *mut u8;
    let mut uVar8: u16;
    let string_a_1: *mut libc::c_char;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_36: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let paStack38: *mut AStruct473;
    let mut local_22: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let local_e: *mut AStruct473;
    let string_a_2: *mut libc::c_char;
    let mut local_4: u16;

    uVar2 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, uVar2) != 0) {
        struct_a_2 = read_file_1008_7dee(param_2, CONCAT22(string_a_1, &local_4), 2);
        if (struct_a_2 != 0x0) {
            local_BX_60 = param_1;
            local_ES_60 = (param_1 >> 0x10);
            uVar9 = param_2;
            uVar10 = (param_2 >> 0x10);
            if (local_4 != 0) {
                if (&local_BX_60.field_0xa == 0) {
                    process_struct_1000_179c(0xc, struct_a_1);
                    _local_16 = CONCAT22(struct_a_1, struct_a_2);
                    paVar6 = (struct_a_1 | struct_a_2);
                    if (paVar6 == 0x0) {
                        &local_BX_60.field_0xa = 0;
                        struct_a_1 = paVar6;
                    } else {
                        paVar3 = process_struct_1008_574a(CONCAT22(struct_a_1, struct_a_2));
                        local_BX_60.field_0xa = paVar3;
                        &local_BX_60.field_0xc = paVar6;
                        struct_a_1 = paVar6;
                    }
                }
                local_12 = 0;
                while (local_12 < local_4) {
                    struct_b = local_4;
                    process_struct_1000_179c(8, struct_a_1);
                    _local_16 = CONCAT22(struct_a_1, struct_b);
                    if ((struct_a_1 | struct_b) == 0) {
                        local_e = 0x0;
                    } else {
                        &_local_16.field_0x0 = s_1_1050_389a;
                        struct_b.field_0x2 = &PTR_LOOP_1050_1008;
                        &_local_16.field_0x0 = 0xa1c4;
                        struct_b.field_0x2 = 0x1010;
                        local_e = _local_16;
                    }
                    b_result = read_file_1008_7dee(param_2, CONCAT22(string_a_1, &string_a_2), 2);
                    if (b_result == 0) {
                        // LAB_1010_9c32:
                        _local_16 = local_e;
                        if (local_e == 0x0) {}
                        // goto LAB_1010_9ba1;
                        uVar8 = (local_e >> 0x10);
                        puVar7 = local_e;
                        // goto LAB_1010_9e9e;
                    }
                    b_result =
                        read_file_1008_7dee(param_2, (local_e & 0xffff0000 | (local_e + 6)), 2);
                    if (b_result == 0) {}
                    // goto LAB_1010_9c32;
                    switch_statement_1008_73ea(uVar9, uVar10, string_a_2);
                    (local_e + 4) = b_result;
                    ppcVar1 = (&local_BX_60.field_0xa + 4);
                    (**ppcVar1)();
                    local_12 = local_12 + 1;
                    struct_a_1 = extraout_DX;
                }
            }
            uVar4 = read_file_1008_7dee(param_2, CONCAT22(string_a_1, &local_2a), 2);
            if (uVar4 != 0) {
                if (local_2a != 0) {
                    if (&local_BX_60.field_0xe == 0) {
                        process_struct_1000_179c(0xc, struct_a_1);
                        local_2e = CONCAT22(struct_a_1, uVar4);
                        paVar6 = (struct_a_1 | uVar4);
                        if (paVar6 == 0x0) {
                            &local_BX_60.field_0xe = 0;
                            struct_a_1 = paVar6;
                        } else {
                            paVar3 = process_struct_1008_574a(CONCAT22(struct_a_1, uVar4));
                            &local_BX_60.field_0xe = paVar3;
                            local_BX_60.field_0x10 = paVar6;
                            struct_a_1 = paVar6;
                        }
                    }
                    local_22 = 0;
                    while (local_22 < local_2a) {
                        local_AX_571 = local_2a;
                        process_struct_1000_179c(8, struct_a_1);
                        _local_16 = CONCAT22(struct_a_1, local_AX_571);
                        if ((struct_a_1 | local_AX_571) == 0) {
                            local_1e = 0;
                        } else {
                            &_local_16.field_0x0 = s_1_1050_389a;
                            local_AX_571.field_0x2 = &PTR_LOOP_1050_1008;
                            &_local_16.field_0x0 = 0xa1c4;
                            local_AX_571.field_0x2 = 0x1010;
                            local_1e = _local_16;
                        }
                        b_result = read_file_1008_7dee(param_2, CONCAT22(string_a_1, &local_1a), 2);
                        if (b_result == 0) {
                            // LAB_1010_9d5c:
                            _local_16 = local_1e;
                            if (local_1e == 0) {}
                            // goto LAB_1010_9ba1;
                            uVar8 = (local_1e >> 0x10);
                            puVar7 = local_1e;
                            // goto LAB_1010_9e9e;
                        }
                        b_result = read_file_1008_7dee(
                            param_2,
                            (local_1e & 0xffff0000 | (local_1e + 6)),
                            2,
                        );
                        if (b_result == 0) {}
                        // goto LAB_1010_9d5c;
                        switch_statement_1008_73ea(uVar9, uVar10, local_1a);
                        (local_1e + 4) = b_result;
                        ppcVar1 = (&local_BX_60.field_0xe + 4);
                        (**ppcVar1)();
                        local_22 = local_22 + 1;
                        struct_a_1 = extraout_DX_00;
                    }
                }
                uVar4 = read_file_1008_7dee(param_2, CONCAT22(string_a_1, &local_36), 2);
                if (uVar4 != 0) {
                    if (local_36 != 0) {
                        paVar6 = struct_a_1;
                        if (&local_BX_60.field_0x12 == 0) {
                            process_struct_1000_179c(0xc, struct_a_1);
                            _local_16 = CONCAT22(struct_a_1, uVar4);
                            paVar6 = (struct_a_1 | uVar4);
                            if (paVar6 == 0x0) {
                                &local_BX_60.field_0x12 = 0;
                            } else {
                                paVar3 = process_struct_1008_574a(CONCAT22(struct_a_1, uVar4));
                                local_BX_60.field_0x12 = paVar3;
                                &local_BX_60.field_0x14 = paVar6;
                            }
                        }
                        local_2a = 0;
                        while (local_2a < local_36) {
                            uVar5 = local_36;
                            process_struct_1000_179c(8, paVar6);
                            local_2e = CONCAT22(paVar6, uVar5);
                            if ((paVar6 | uVar5) == 0) {
                                paStack38 = 0x0;
                            } else {
                                local_2e = s_1_1050_389a;
                                (uVar5 + 2) = &PTR_LOOP_1050_1008;
                                local_2e = 0xa1c4;
                                (uVar5 + 2) = 0x1010;
                                paStack38 = local_2e;
                            }
                            b_result =
                                read_file_1008_7dee(param_2, CONCAT22(string_a_1, &local_22), 2);
                            if (b_result == 0) {
                                // LAB_1010_9e86:
                                _local_16 = paStack38;
                                if (paStack38 != 0x0) {
                                    uVar8 = (paStack38 >> 0x10);
                                    puVar7 = paStack38;
                                    // LAB_1010_9e9e:
                                    ppcVar1 = unsafe { *puVar7 };
                                    local_2e = _local_16;
                                    (**ppcVar1)(&PTR_LOOP_1050_1008, _local_16, uVar8, 1);
                                }
                                // goto LAB_1010_9ba1;
                            }
                            b_result = read_file_1008_7dee(
                                param_2,
                                (paStack38 & 0xffff0000 | (paStack38 + 6)),
                                2,
                            );
                            if (b_result == 0) {}
                            // goto LAB_1010_9e86;
                            switch_statement_1008_73ea(uVar9, uVar10, local_22);
                            (paStack38 + 4) = b_result;
                            ppcVar1 = (&local_BX_60.field_0x12 + 4);
                            (**ppcVar1)();
                            local_2a = local_2a + 1;
                            paVar6 = extraout_DX_01;
                        }
                    }
                    b_result = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_BX_60.field_0x1a),
                        2,
                    );
                    if (b_result != 0) {
                        b_result = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | &local_BX_60.field_0x1c),
                            2,
                        );
                        if (b_result != 0) {
                            b_result = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | ZEXT24(local_BX_60 + 1)),
                                2,
                            );
                            if (b_result != 0) {
                                return;
                            }
                        }
                    }
                }
            }
        }
        // LAB_1010_9ba1:
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1010_ed58(param_1: u32, in_file: *mut HFILE16) {
    let puVar1: *mut u16;
    let uVar2: u8;
    let BVar3: bool;
    let extraout_var: u32;
    // ppuVar4: *mut *mut u8;
    let local_BX_30: *mut AStruct506;
    let local_ES_30: *mut u8;
    let string_base_a: *mut u8;
    let local_22: *mut u8;
    let mut uStack30: u16;
    let mut string_offset_a: u32;
    let mut string_offset_b: u32;
    let mut local_4: u16;
    let temp_5fb1d7bd90: *mut u8;

    uVar2 = write_to_file_1008_7cac(in_file, 2);
    if (CONCAT31(extraout_var, uVar2) != 0) {
        local_ES_30 = (param_1 >> 0x10);
        local_BX_30 = param_1;
        string_offset_a = local_BX_30.field_0x16;
        BVar3 = write_to_file_1008_7e1c(in_file, CONCAT22(string_base_a, &string_offset_a), 4);
        if (BVar3 != 0) {
            string_offset_b = local_BX_30.field_0x1a;
            BVar3 = write_to_file_1008_7e1c(in_file, CONCAT22(string_base_a, &string_offset_b), 4);
            if (BVar3 != 0) {
                string_offset_b = local_BX_30.field_0x20;
                BVar3 =
                    write_to_file_1008_7e1c(in_file, CONCAT22(string_base_a, &string_offset_b), 4);
                if (BVar3 != 0) {
                    string_offset_b = local_BX_30.field_0x24;
                    BVar3 = write_to_file_1008_7e1c(
                        in_file,
                        CONCAT22(string_base_a, &string_offset_b),
                        4,
                    );
                    if (BVar3 != 0) {
                        string_offset_b = string_offset_b & 0xffff0000 | local_BX_30.field_0x30;
                        BVar3 = write_to_file_1008_7e1c(
                            in_file,
                            CONCAT22(string_base_a, &string_offset_b),
                            2,
                        );
                        if (BVar3 != 0) {
                            string_offset_b = string_offset_b & 0xffff0000 | local_BX_30.field_0x32;
                            BVar3 = write_to_file_1008_7e1c(
                                in_file,
                                CONCAT22(string_base_a, &string_offset_b),
                                2,
                            );
                            if (BVar3 != 0) {
                                local_4 = 0;
                                while (true) {
                                    puVar1 = &local_BX_30.field_0x30;
                                    let pu_var1_val = unsafe { *puVar1 };
                                    if (pu_var1_val == local_4 || pu_var1_val < local_4) {
                                        return;
                                    }
                                    temp_5fb1d7bd90 = local_BX_30.field_0x2e;
                                    ppuVar4 = (local_BX_30.field_0x2c + local_4 * 6);
                                    local_22 = *ppuVar4;
                                    uStack30 = (ppuVar4 + 1);
                                    ppuVar4 = &local_22;
                                    string_offset_a =
                                        string_offset_a & 0xffff0000 | ZEXT24(ppuVar4);
                                    write_file_1008_7b4c(in_file, CONCAT22(string_base_a, ppuVar4));
                                    if (ppuVar4 == 0x0) {
                                        break;
                                    }
                                    local_4 = local_4 + 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn read_file_1018_0000(param_1: u32, param_2: *mut HFILE16) {
    let puVar1: *mut u16;
    let mut uVar2: u32;
    let uVar3: u8;
    let mut iVar4: i32;
    let BVar5: bool;
    let puVar6: *mut u8;
    let extraout_var: u32;
    let in_DX: *mut u16;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_24: u16;
    let mut local_20: [u8; 16];
    let mut local_10: u16;

    // Segment:    4
    // Offset:     00024460
    // Length:     ee6a
    // Min Alloc:  ee6a
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    uVar3 = read_file_1008_7cfe(param_2);
    if (CONCAT31(extraout_var, uVar3) == 0) {
        g_u16_1050_0310 = 0x6d4;
    } else {
        iVar4 = param_1;
        BVar5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar4 + 0x16)), 4);
        if ((((BVar5 != 0)
            && (
                BVar5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar4 + 0x1a)), 4),
                BVar5 != 0,
            ))
            && (
                BVar5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar4 + 0x20)), 4),
                BVar5 != 0,
            ))
            && ((
                BVar5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar4 + 0x24)), 4),
                BVar5 != 0
                    && (
                        BVar5 = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | (iVar4 + 0x30)),
                            2,
                        ),
                        BVar5 != 0,
                    ),
            ) && (
                BVar5 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar4 + 0x32)), 2),
                BVar5 != 0,
            )))
        {
            uVar7 = (param_1 >> 0x10);
            if ((iVar4 + 0x30) != 0) {
                _g_AStruct94_ptr_1 = ((iVar4 + 0x32) * 6);
                if (__g_AStruct94_ptr_1 == 0) {
                    struct_fn_1000_160a();
                    g_u16_ptr_1050_5f2e = in_DX;
                } else {
                }
                alloc_mem_1000_1708();
                (iVar4 + 0x2c) = _g_AStruct94_ptr_1;
                (iVar4 + 0x2e) = g_u16_ptr_1050_5f2e;
                zero_list_1008_3e38(CONCAT22(unaff_SS, local_20));
                local_10 = 0;
                let pu_var1_val = unsafe { *puVar1 };
                while (
                    puVar1 = (iVar4 + 0x30),
                    pu_var1_val != local_10 && local_10 <= pu_var1_val,
                ) {
                    puVar6 = local_20;
                    read_file_1008_7bc8(param_2, (param_2 >> 0x10), puVar6, unaff_SS);
                    if (puVar6 == 0x0) {
                        g_u16_1050_0310 = 0x6d0;
                        return;
                    }
                    uVar2 = (iVar4 + 0x2c);
                    modify_list_1008_3f62(
                        (uVar2 & 0xffff0000 | (uVar2 + local_10 * 6)),
                        CONCAT22(unaff_SS, local_20),
                    );
                    local_10 = local_10 + 1;
                }
            }
            return;
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1038_7b20(param_1: *mut u32, param_2: *mut HFILE16) -> u16 {
    let mut u_var1: u32;
    let uVar2: u8;
    let extraout_AH: u8;
    let BVar3: bool;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let lVar6: u32;
    let paVar7: *mut AStruct961;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 8];
    let mut local_4: u16;

    uVar2 = write_to_file_1008_7cac(param_2, 0x17);
    if (CONCAT11(extraout_AH, uVar2) != 0) {
        let param_1_val = unsafe { *param_1 };
        local_1c = (param_1_val + 8);
        local_4 = local_1c;
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_1c), 2);
        if (BVar3 != 0) {
            pass1_1008_5784(CONCAT22(unaff_SS, local_c), param_1_val);
            while {
                lVar6 = pass1_1008_5b12(CONCAT22(unaff_SS, local_c));
                _local_10 = lVar6;
                if (lVar6 == 0) {
                    uVar5 = (param_1 >> 0x10);
                    uVar1 = (param_1 + 4);
                    local_1c = (uVar1 + 8);
                    local_4 = local_1c;
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_4), 2);
                    if (BVar3 == 0) {
                        return 0;
                    }
                    pass1_1008_5784(CONCAT22(unaff_SS, local_c), (param_1 + 4));
                    while {
                        paVar7 = pass1_1008_5b12(CONCAT22(unaff_SS, local_c));
                        local_1a = paVar7;
                        if (paVar7 == 0x0) {
                            return 1;
                        }
                        write_to_file_1030_b768(paVar7, param_2);
                        local_18 = (paVar7 >> 0x10);
                        paVar7 != 0
                    } {}
                    return 0;
                }
                iVar4 = write_to_file_1038_75ca(lVar6, param_2, (param_2 >> 0x10));
                iVar4 != 0
            } {}
        }
    }
    return 0;
}

pub fn read_from_file_1038_7c02(param_1: *mut *mut u8, file_b: *mut HFILE16) -> u16 {
    let u8_a: u8;
    let extraout_AH: u8;
    let bool_a: bool;
    let mut uVar1: u16;
    let mut iVar2: i32;
    let in_DX: *mut AStruct199;
    let extraout_DX: *mut AStruct199;
    let mut uVar3: i32;
    let extraout_DX_00: *mut AStruct199;
    let mut unaff_SS: u16;
    let pHVar4: *mut HFILE16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_4: u16;
    let fn_ptr_a: fn();

    if (PTR_LOOP_1050_0312 < 2) {
        return 1;
    }
    u8_a = read_file_1008_7cfe(file_b);
    if ((CONCAT11(extraout_AH, u8_a) != 0)
        && (
            bool_a = read_file_1008_7dee(file_b, CONCAT22(unaff_SS, &local_4), 2),
            bool_a != 0,
        ))
    {
        while (local_4 != 0) {
            uVar1 = local_4;
            local_4 = local_4 - 1;
            pHVar4 = file_b;
            process_struct_1000_179c(0x2a, in_DX);
            if ((in_DX | uVar1) == 0) {
                _local_e = 0;
            } else {
                _local_e = pass1_1038_6520(CONCAT22(in_DX, uVar1));
            }
            iVar2 = read_from_file_1038_774e(_local_e, pHVar4);
            if (iVar2 == 0) {
                return 0;
            }
            fn_ptr_a = (param_1 + 4);
            (**fn_ptr_a)();
            in_DX = extraout_DX;
        }
        local_4 = local_4 - 1;
        bool_a = read_file_1008_7dee(file_b, CONCAT22(unaff_SS, &local_12), 2);
        if (bool_a != 0) {
            while (true) {
                if (local_12 == 0) {
                    return 1;
                }
                uVar1 = local_12;
                local_12 = local_12 - 1;
                pHVar4 = file_b;
                process_struct_1000_179c(0x14, in_DX);
                uVar3 = in_DX | uVar1;
                if (uVar3 == 0) {
                    uVar1 = 0;
                    uVar3 = 0;
                } else {
                    pass1_1030_ae6c(CONCAT22(in_DX, uVar1));
                }
                iVar2 = pass1_1030_b836(CONCAT22(uVar3, uVar1), pHVar4);
                if (iVar2 == 0) {
                    break;
                }
                fn_ptr_a = ((param_1 + 4) + 4);
                (**fn_ptr_a)();
                in_DX = extraout_DX_00;
            }
            return 0;
        }
    }
    return 0;
}

pub fn write_to_file_1038_75ca(param_1: u32, file_handle: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;
    let puVar2: *mut u8;
    let local_BX_4: *mut AStruct1167;
    let mut uvar3: u16;
    let mut base: u16;
    let mut offset_2: u32;
    let mut offset_1: u32;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    write_to_file_1008_79f0(file_handle, local_BX_4.field_0x4);
    if (in_AX != 0) {
        offset_2 = local_BX_4.field_0x8;
        BVar1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_2), 4);
        if (BVar1 != 0) {
            write_to_file_1008_7a22(file_handle, local_BX_4.field_0xe);
            if (BVar1 != 0) {
                offset_1 = CONCAT22(offset_1._2_2_, local_BX_4.field_0xc);
                BVar1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 2);
                if (BVar1 != 0) {
                    offset_1 = offset_1 & 0xffff0000 | local_BX_4.field_0x12;
                    BVar1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 2);
                    if (BVar1 != 0) {
                        offset_1 = offset_1 & 0xffff0000 | local_BX_4.field_0x14;
                        BVar1 = write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 2);
                        if (BVar1 != 0) {
                            offset_1 = local_BX_4.field_0x16;
                            BVar1 =
                                write_to_file_1008_7e1c(file_handle, CONCAT22(base, &offset_1), 4);
                            if (BVar1 != 0) {
                                puVar2 = (param_1 & 0xffff0000 | &local_BX_4.field_0x1a);
                                write_file_1008_7b4c(file_handle, puVar2);
                                if (puVar2 != 0) {
                                    offset_1 = local_BX_4.field_0x20;
                                    BVar1 = write_to_file_1008_7e1c(
                                        file_handle,
                                        CONCAT22(base, &offset_1),
                                        4,
                                    );
                                    if (BVar1 != 0) {
                                        offset_1 = offset_1 & 0xffff0000 | local_BX_4.field_0x24;
                                        BVar1 = write_to_file_1008_7e1c(
                                            file_handle,
                                            CONCAT22(base, &offset_1),
                                            2,
                                        );
                                        if (BVar1 != 0) {
                                            offset_1 =
                                                offset_1 & 0xffff0000 | local_BX_4.field_0x26;
                                            BVar1 = write_to_file_1008_7e1c(
                                                file_handle,
                                                CONCAT22(base, &offset_1),
                                                2,
                                            );
                                            if (BVar1 != 0) {
                                                offset_1 =
                                                    offset_1 & 0xffff0000 | local_BX_4.field_0x28;
                                                BVar1 = write_to_file_1008_7e1c(
                                                    file_handle,
                                                    CONCAT22(base, &offset_1),
                                                    2,
                                                );
                                                if (BVar1 != 0) {
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn read_from_file_1038_774e(param_1: u32, param_2: *mut HFILE16) {
    let mut uVar1: u16;
    let local_AX_22: *mut AStruct1168;
    let BVar2: bool;
    let mut iVar3: i32;
    let mut unaff_SS: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut uVar4: u32;

    if (PTR_LOOP_1050_0312 < 2) {
        return;
    }
    local_AX_22 = param_1;
    local_AX_22 = &local_AX_22.field_0x4;
    uVar4 = param_1 & 0xffff0000 | ZEXT24(local_AX_22);
    pass1_1008_766e(param_2, uVar4);
    if ((uVar4 != 0)
        && (
            BVar2 =
                read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | &local_AX_22.field_0x8), 4),
            BVar2 != 0,
        ))
    {
        iVar3 = &local_AX_22.field_0xe;
        uVar1 = (param_1 >> 0x10);
        uVar5 = param_2;
        uVar6 = (param_2 >> 0x10);
        read_file_1008_77cc(uVar5, uVar6, iVar3, uVar1);
        if ((iVar3 != 0)
            && (((
                BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2),
                BVar2 != 0
                    && (
                        BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_6), 2),
                        BVar2 != 0,
                    ),
            ) && (
                BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_8), 2),
                BVar2 != 0,
            )) && (
                BVar2 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | &local_AX_22.field_0x16),
                    4,
                ),
                BVar2 != 0,
            )))
        {
            iVar3 = &local_AX_22.field_0x1a;
            read_file_1008_7bc8(uVar5, uVar6, iVar3, uVar1);
            if ((((iVar3 != 0)
                && (
                    BVar2 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_AX_22.field_0x20),
                        4,
                    ),
                    BVar2 != 0,
                ))
                && (
                    BVar2 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_AX_22.field_0x24),
                        2,
                    ),
                    BVar2 != 0,
                ))
                && ((
                    BVar2 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_AX_22.field_0x26),
                        2,
                    ),
                    BVar2 != 0
                        && (
                            BVar2 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | &local_AX_22.field_0x28),
                                2,
                            ),
                            BVar2 != 0,
                        ),
                )))
            {
                local_AX_22.field_0xc = local_4;
                set_param_3_with_switch_1008_72bc(uVar5, uVar6, local_6);
                local_AX_22.field_0x12 = local_4;
                local_AX_22.field_0x14 = local_8;
                return;
            }
        }
    }
    g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn read_from_file_1038_6118(param_1: *mut AStruct933, param_2: *mut HFILE16) {
    let BVar1: bool;
    let uVar2: u8;
    let puVar3: *mut u32;
    let BVar4: bool;
    let local_AX_307: *mut AStruct1142;
    let puVar5: *mut u8;
    let extraout_var: u32;
    let extraout_DX: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let paVar6: *mut AStruct199;
    let local_BX_68: *mut AStruct1141;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_426: u32;
    let mut local_416: u16;
    let mut local_414: u16;
    let local_412: *mut AStruct1143;
    let mut local_40e: u32;
    let mut local_408: [u8; 1024];
    let mut local_8: u16;
    let mut local_6: u32;

    uVar2 = pass1_1030_1730(param_1, param_2);
    if (CONCAT31(extraout_var, uVar2) == 0) {
        return;
    }
    local_6 = 0;
    puVar3 = &local_6;
    uVar8 = (param_2 >> 0x10);
    read_file_1008_7548(param_2, uVar8, puVar3, unaff_SS);
    if (puVar3 != 0x0) {
        uVar7 = (param_1 >> 0x10);
        local_BX_68 = param_1;
        local_BX_68.field_0xc = local_6;
        struct_a = extraout_DX;
        BVar4 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | &local_BX_68.field_0x10), 4);
        if (((((BVar4 != 0)
            && (
                BVar4 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | &local_BX_68.field_0x18),
                    2,
                ),
                BVar4 != 0,
            ))
            && (
                BVar4 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | &local_BX_68.field_0x1a),
                    2,
                ),
                BVar4 != 0,
            ))
            && ((
                BVar4 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_8), 2),
                BVar4 != 0
                    && (
                        BVar4 = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | &local_BX_68.field_0x1e),
                            4,
                        ),
                        BVar4 != 0,
                    ),
            )))
            && (
                BVar4 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | &local_BX_68.field_0x22),
                    2,
                ),
                BVar4 != 0,
            ))
        {
            local_BX_68.field_0x1c = local_8;
            BVar4 =
                read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | &local_BX_68.field_0x24), 2);
            if ((BVar4 != 0)
                && (
                    local_AX_307 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_BX_68.field_0x26),
                        0x94,
                    ),
                    local_AX_307 != 0x0,
                ))
            {
                if (PTR_LOOP_1050_0312 < 2) {
                    uVar9 = 0x54;
                    uVar10 = 0;
                    process_struct_1000_179c(0x54, struct_a);
                    _local_416 = CONCAT22(struct_a, local_AX_307);
                    BVar4 = read_file_1008_7dee(
                        param_2,
                        CONCAT22(struct_a, local_AX_307),
                        CONCAT22(uVar10, uVar9),
                    );
                    if (BVar4 == 0) {
                        // LAB_1038_626a:
                        g_u16_1050_0310 = 0x6d2;
                        error_check_1000_17ce(_local_416);
                        return;
                    }
                    local_412 = 0x0;
                    while {
                        set_param_3_with_switch_1008_72bc(param_2, uVar8, local_412);
                        BVar1 = (local_AX_307 + local_412 * 4);
                        uVar9 = (local_AX_307 + local_412 * 4 + 2);
                        (&local_BX_68.field_0x14e + BVar4 * 4) = BVar1;
                        (&local_BX_68.field_0x150 + BVar4 * 4) = uVar9;
                        local_412 = &local_412.field_0x1;
                        BVar4 = BVar1;
                        local_412 < 0x15
                    } {}
                    BVar4 = read_file_1008_7dee(param_2, _local_416, 0x54);
                    if (BVar4 == 0) {}
                    // goto LAB_1038_626a;
                    local_412 = 0x0;
                    while {
                        set_param_3_with_switch_1008_72bc(param_2, uVar8, local_412);
                        BVar1 = (local_AX_307 + local_412 * 4);
                        paVar6 = (local_AX_307 + local_412 * 4 + 2);
                        (&local_BX_68.field_0x1a2 + BVar4 * 4) = BVar1;
                        (&local_BX_68.field_0x1a4 + BVar4 * 4) = paVar6;
                        local_412 = &local_412.field_0x1;
                        BVar4 = BVar1;
                        local_412 < 0x15
                    } {}
                    error_check_1000_17ce(_local_416);
                    struct_a = paVar6;
                } else {
                    BVar4 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_BX_68.field_0x14e),
                        0x54,
                    );
                    if (BVar4 == 0) {
                        g_u16_1050_0310 = 0x6d2;
                        return;
                    }
                    BVar4 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_BX_68.field_0x1a2),
                        0x54,
                    );
                    if (BVar4 == 0) {
                        g_u16_1050_0310 = 0x6d2;
                        return;
                    }
                }
                read_from_file_1030_33f0(local_BX_68.field_0x1f6, param_2);
                puVar5 = local_408;
                read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_SS, puVar5));
                if (puVar5 != 0x0) {
                    puVar5 = local_408;
                    pass1_fn_1008_60e8(puVar5);
                    local_BX_68.field_0x1fa = puVar5;
                    local_BX_68.field_0x1fc = struct_a;
                    BVar4 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_BX_68.field_0x1fe),
                        2,
                    );
                    if (((((BVar4 != 0)
                        && (
                            BVar4 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | CONCAT11((param_1 >> 8) + 0x2, param_1)),
                                4,
                            ),
                            BVar4 != 0,
                        ))
                        && (
                            BVar4 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | &local_BX_68.field_0x204),
                                2,
                            ),
                            BVar4 != 0,
                        ))
                        && ((
                            BVar4 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | &local_BX_68.field_0x206),
                                2,
                            ),
                            BVar4 != 0
                                && (
                                    BVar4 = read_file_1008_7dee(
                                        param_2,
                                        (param_1 & 0xffff0000 | &local_BX_68.field_0x208),
                                        2,
                                    ),
                                    BVar4 != 0,
                                ),
                        ) && ((
                            BVar4 = read_file_1008_7dee(
                                param_2,
                                (param_1 & 0xffff0000 | &local_BX_68.field_0x20a),
                                2,
                            ),
                            BVar4 != 0
                                && ((
                                    BVar4 = read_file_1008_7dee(
                                        param_2,
                                        (param_1 & 0xffff0000 | &local_BX_68.field_0x20c),
                                        2,
                                    ),
                                    BVar4 != 0
                                        && (
                                            BVar4 = read_file_1008_7dee(
                                                param_2,
                                                (param_1 & 0xffff0000 | &local_BX_68.field_0x20e),
                                                2,
                                            ),
                                            BVar4 != 0,
                                        ),
                                )),
                        ))))
                        && (PTR_LOOP_1050_0312 < 2
                            || ((
                                BVar4 = read_file_1008_7dee(
                                    param_2,
                                    (param_1 & 0xffff0000 | &local_BX_68.field_0x214),
                                    2,
                                ),
                                BVar4 != 0
                                    && (
                                        BVar4 = read_file_1008_7dee(
                                            param_2,
                                            (param_1 & 0xffff0000 | ZEXT24(local_BX_68 + 1)),
                                            4,
                                        ),
                                        BVar4 != 0,
                                    ),
                            ))))
                    {
                        return;
                    }
                    g_u16_1050_0310 = 0x6d0;
                    return;
                }
            }
        }
    }
    g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn write_to_file_1038_5e16(param_1: *mut AStruct771, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;
    let puVar2: *mut u32;
    let local_BX_28: *mut AStruct1140;
    let mut uvar3: u16;
    let mut string_base: u16;
    let mut offset_1: u32;
    let mut offset_2: u32;
    let mut local_6: u32;

    write_to_file_1030_16d6(param_1, param_2);
    if (in_AX != 0) {
        uVar3 = (param_1 >> 0x10);
        local_BX_28 = param_1;
        puVar2 = local_BX_28.field_0xc;
        local_6 = puVar2;
        write_to_file_1008_7898(param_2, puVar2);
        if (puVar2 != 0) {
            offset_1 = local_BX_28.field_0x10;
            BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_1), 4);
            if (BVar1 != 0) {
                offset_2 = CONCAT22(offset_2._2_2_, local_BX_28.field_0x18);
                BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                if (BVar1 != 0) {
                    offset_2 = offset_2 & 0xffff0000 | local_BX_28.field_0x1a;
                    BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                    if (BVar1 != 0) {
                        offset_2 = offset_2 & 0xffff0000 | local_BX_28.field_0x1c;
                        BVar1 =
                            write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                        if (BVar1 != 0) {
                            offset_2 = local_BX_28.field_0x1e;
                            BVar1 = write_to_file_1008_7e1c(
                                param_2,
                                CONCAT22(string_base, &offset_2),
                                4,
                            );
                            if (BVar1 != 0) {
                                offset_2 = offset_2 & 0xffff0000 | local_BX_28.field_0x22;
                                BVar1 = write_to_file_1008_7e1c(
                                    param_2,
                                    CONCAT22(string_base, &offset_2),
                                    2,
                                );
                                if (BVar1 != 0) {
                                    offset_2 = offset_2 & 0xffff0000 | local_BX_28.field_0x24;
                                    BVar1 = write_to_file_1008_7e1c(
                                        param_2,
                                        CONCAT22(string_base, &offset_2),
                                        2,
                                    );
                                    if (BVar1 != 0) {
                                        BVar1 = write_to_file_1008_7e1c(
                                            param_2,
                                            (param_1 & 0xffff0000 | &local_BX_28.field_0x26),
                                            0x94,
                                        );
                                        if (BVar1 != 0) {
                                            BVar1 = write_to_file_1008_7e1c(
                                                param_2,
                                                (param_1 & 0xffff0000 | &local_BX_28.field_0x14e),
                                                0x54,
                                            );
                                            if (BVar1 != 0) {
                                                BVar1 = write_to_file_1008_7e1c(
                                                    param_2,
                                                    (param_1 & 0xffff0000
                                                        | &local_BX_28.field_0x1a2),
                                                    0x54,
                                                );
                                                if (BVar1 != 0) {
                                                    write_to_file_1030_32e4(
                                                        local_BX_28.field_0x1f6,
                                                        param_2,
                                                    );
                                                    write_to_file_1008_7c2a(
                                                        param_2,
                                                        local_BX_28.field_0x1fa,
                                                    );
                                                    if (BVar1 != 0) {
                                                        offset_2 = offset_2 & 0xffff0000
                                                            | local_BX_28.field_0x1fe;
                                                        BVar1 = write_to_file_1008_7e1c(
                                                            param_2,
                                                            CONCAT22(string_base, &offset_2),
                                                            2,
                                                        );
                                                        if (BVar1 != 0) {
                                                            offset_2 = local_BX_28.field_0x200;
                                                            BVar1 = write_to_file_1008_7e1c(
                                                                param_2,
                                                                CONCAT22(string_base, &offset_2),
                                                                4,
                                                            );
                                                            if (BVar1 != 0) {
                                                                offset_2 = offset_2 & 0xffff0000
                                                                    | local_BX_28.field_0x204;
                                                                BVar1 = write_to_file_1008_7e1c(
                                                                    param_2,
                                                                    CONCAT22(
                                                                        string_base,
                                                                        &offset_2,
                                                                    ),
                                                                    2,
                                                                );
                                                                if (BVar1 != 0) {
                                                                    offset_2 = offset_2
                                                                        & 0xffff0000
                                                                        | local_BX_28.field_0x206;
                                                                    BVar1 = write_to_file_1008_7e1c(
                                                                        param_2,
                                                                        CONCAT22(
                                                                            string_base,
                                                                            &offset_2,
                                                                        ),
                                                                        2,
                                                                    );
                                                                    if (BVar1 != 0) {
                                                                        offset_2 = offset_2
                                                                            & 0xffff0000
                                                                            | local_BX_28
                                                                                .field_0x208;
                                                                        BVar1 =
                                                                            write_to_file_1008_7e1c(
                                                                                param_2,
                                                                                CONCAT22(
                                                                                    string_base,
                                                                                    &offset_2,
                                                                                ),
                                                                                2,
                                                                            );
                                                                        if (BVar1 != 0) {
                                                                            offset_2 = offset_2
                                                                                & 0xffff0000
                                                                                | local_BX_28
                                                                                    .field_0x20a;
                                                                            BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                                                                            if (BVar1 != 0) {
                                                                                offset_2 = offset_2 & 0xffff0000 |
                                                                                           local_BX_28.field_0x20c;
                                                                                BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                                                                                if (BVar1 != 0) {
                                                                                    offset_2 = offset_2 & 0xffff0000 |
                                                                                               local_BX_28.field_0x20e;
                                                                                    BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                                                                                    if (BVar1 != 0)
                                                                                    {
                                                                                        offset_2 = offset_2 & 0xffff0000 |
                                                                                                   local_BX_28.field_0x214;
                                                                                        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 2);
                                                                                        if (BVar1
                                                                                            != 0)
                                                                                        {
                                                                                            offset_2 = local_BX_28.field_0x216;
                                                                                            BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &offset_2), 4);
                                                                                            if (BVar1 != 0)
                                                                                            {
                                                                                                return;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn read_from_file_1030_dec4(param_1: u32, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;

    read_file_fn_1028_b81a(param_1, param_2);
    if (((in_AX != 0) && (1 < PTR_LOOP_1050_0312))
        && (
            BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x20)), 4),
            BVar1 == 0,
        ))
    {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub fn write_to_file_1030_de7c(param_1: *mut AStruct771, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;
    let mut unaff_SS: u16;
    let mut local_10: u32;

    write_to_file_1028_b5ec(param_1, param_2);
    if (in_AX != 0) {
        local_10 = (param_1 + 0x20);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_10), 4);
        if (BVar1 == 0) {
            g_u16_1050_0310 = 0x6d0;
            return;
        }
    }
    return;
}

pub fn write_to_file_1030_d61c(param_1: *mut AStruct771, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let b_success: bool;
    let mut iVar1: i32;
    let mut local_ES_61: u16;
    let mut string_base: u16;
    let mut string_off_4: u32;
    let mut string_off_3: u16;
    let mut string_off_2: u16;
    let mut string_off_1: u32;
    let mut local_4: u16;

    write_to_file_1028_b5ec(param_1, param_2);
    if (in_AX != 0) {
        local_4 = 0;
        while (local_4 < 10) {
            local_ES_61 = (param_1 >> 0x10);
            iVar1 = param_1;
            string_off_1 = (iVar1 + local_4 * 0xc + 0x20);
            b_success = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &string_off_1), 4);
            if (b_success == 0) {}
            // goto LAB_1030_d701;
            string_off_2 = (iVar1 + local_4 * 0xc + 0x24);
            b_success = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &string_off_2), 2);
            if (b_success == 0) {}
            // goto LAB_1030_d701;
            string_off_3 = (iVar1 + local_4 * 0xc + 0x26);
            b_success = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &string_off_3), 2);
            if (b_success == 0) {}
            // goto LAB_1030_d701;
            string_off_4 = (iVar1 + local_4 * 0xc + 0x28);
            b_success = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &string_off_4), 4);
            if (b_success == 0) {}
            // goto LAB_1030_d701;
            local_4 = local_4 + 1;
        }
        string_off_3 = PTR_LOOP_1050_5812;
        b_success = write_to_file_1008_7e1c(param_2, CONCAT22(string_base, &string_off_3), 2);
        if (b_success != 0) {
            return;
        }
        // LAB_1030_d701:
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn pass1_1030_d72e(param_1: u32, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;
    let mut iVar2: i32;
    let mut unaff_SS: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    read_file_fn_1028_b81a(param_1, param_2);
    if (in_AX == 0) {
        return;
    }
    local_a = 0;
    while (true) {
        if (9 < local_a) {
            if ((3 < PTR_LOOP_1050_0312)
                && (
                    BVar1 = read_file_1008_7dee(param_2, &PTR_LOOP_1050_5812, 2),
                    BVar1 == 0,
                ))
            {
                g_u16_1050_0310 = 0x6d2;
                return;
            }
            return;
        }
        BVar1 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_8), 4);
        if (BVar1 == 0) {
            g_u16_1050_0310 = 0x6d2;
            return;
        }
        BVar1 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2);
        if (BVar1 == 0) {
            break;
        }
        iVar2 = local_a * 0xc + param_1;
        (iVar2 + 0x20) = local_8;
        (iVar2 + 0x22) = local_6;
        set_param_3_with_switch_1008_72bc(param_2, (param_2 >> 0x10), local_4);
        (iVar2 + 0x24) = local_8;
        if (PTR_LOOP_1050_0312 < 2) {
            iVar2 = local_a * 0xc + param_1;
            (iVar2 + 0x26) = 3;
            (iVar2 + 0x28) = 0;
        } else {
            BVar1 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2);
            if (BVar1 == 0) {
                g_u16_1050_0310 = 0x6d2;
                return;
            }
            BVar1 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_8), 4);
            if (BVar1 == 0) {
                g_u16_1050_0310 = 0x6d2;
                return;
            }
            iVar2 = local_a * 0xc + param_1;
            (iVar2 + 0x26) = local_4;
            (iVar2 + 0x28) = _local_8;
        }
        local_a = local_a + 1;
    }
    g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn pass1_1030_c84e(param_1: *mut AStruct771, param_2: *mut HFILE16) -> bool {
    let BVar1: bool;
    let mut unaff_SS: u16;
    let mut local_c: u16;

    BVar1 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar1 != 0) {
        local_c = (param_1 + 0x20);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
        if (BVar1 == 0) {
            g_u16_1050_0310 = 0x6d0;
            return BVar1;
        }
        BVar1 = 1;
    }
    return BVar1;
}

pub fn read_from_file_1030_c894(param_1: u32, param_2: *mut HFILE16) -> bool {
    let BVar1: bool;
    let mut unaff_SS: u16;
    let mut local_4: u16;

    BVar1 = read_file_fn_1028_b81a(param_1, param_2);
    if (BVar1 != 0) {
        BVar1 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2);
        if (BVar1 == 0) {
            g_u16_1050_0310 = 0x6d2;
            return BVar1;
        }
        (param_1 + 0x20) = local_4;
        BVar1 = 1;
    }
    return BVar1;
}

pub fn write_to_file_1030_c230(param_1: *mut AStruct771, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut local_10: u32;
    let mut local_8: u16;

    write_to_file_1028_b5ec(param_1, param_2);
    if (in_AX != 0) {
        uVar2 = (param_1 >> 0x10);
        local_10 = (param_1 + 0x20);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_10), 4);
        if (BVar1 != 0) {
            local_8 = (param_1 + 0x24);
            BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_8), 2);
            if (BVar1 != 0) {
                return;
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn read_from_file_1030_c29c(param_1: u32, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;

    read_file_fn_1028_b81a(param_1, param_2);
    if (in_AX != 0) {
        BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x20)), 4);
        if (BVar1 != 0) {
            BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x24)), 2);
            if (BVar1 != 0) {
                return;
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1030_b768(param_1: *mut AStruct961, param_2: *mut HFILE16) {
    let mut u_var1: u32;
    let BVar2: bool;
    let puVar3: *mut u8;
    let puVar4: *mut u8;
    let mut extraout_DX: i32;
    let local_BX_4: *mut AStruct961;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_22: u16;
    let mut local_1a: [u8; 10];
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar5 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    local_10 = local_BX_4.field_0x4;
    BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_10), 4);
    if (BVar2 != 0) {
        puVar4 = (param_1 & 0xffff0000 | &local_BX_4.field_0x8);
        write_file_1008_7b4c(param_2, puVar4);
        if (puVar4 != 0) {
            local_8 = local_BX_4.field_0xe;
            BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_8), 2);
            if (BVar2 != 0) {
                uVar1 = local_BX_4.field_0x10;
                local_22 = (uVar1 + 8);
                local_10 = local_10 & 0xffff0000 | local_22;
                BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_22), 2);
                if (BVar2 == 0) {
                    return;
                }
                pass1_1008_5784(CONCAT22(unaff_SS, local_1a), local_BX_4.field_0x10);
                while {
                    puVar3 = local_1a;
                    pass1_1008_5b12(CONCAT22(unaff_SS, puVar3));
                    if ((extraout_DX | puVar3) == 0) {
                        return;
                    }
                    local_c = puVar3;
                    local_a = extraout_DX;
                    write_to_file_1038_75ca(puVar3, extraout_DX, param_2);
                    puVar3 != 0x0
                } {}
                return;
            }
        }
    }
    g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn pass1_1030_b836(struct_b: *mut AStruct962, file_a: *mut HFILE16) {
    let ppcVar1: fn();
    let local_AX_11: *mut AStruct962;
    let BVar2: bool;
    let mut uvar3: u16;
    let mut uVar4: u32;
    let extraout_DX: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let mut extraout_DX_00: u16;
    let mut uVar5: u16;
    let extraout_DX_01: *mut AStruct199;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let pHVar7: *mut HFILE16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_4: u16;

    local_AX_11 = struct_b;
    local_AX_11 = &local_AX_11.field_0x4;
    BVar2 = read_file_1008_7dee(file_a, (struct_b & 0xffff0000 | ZEXT24(local_AX_11)), 4);
    if (BVar2 != 0) {
        uVar4 = struct_b & 0xffff0000 | &local_AX_11.field_0x8;
        read_file_1008_7bc8(file_a, uVar4);
        if ((uVar4 != 0)
            && (
                struct_a = extraout_DX,
                BVar2 = read_file_1008_7dee(file_a, CONCAT22(unaff_SS, &local_4), 2),
                BVar2 != 0,
            ))
        {
            uVar6 = (struct_b >> 0x10);
            local_AX_11.field_0xe = local_4;
            BVar2 = read_file_1008_7dee(file_a, CONCAT22(unaff_SS, &local_12), 2);
            if (BVar2 == 0) {
                return;
            }
            while (true) {
                if (local_12 == 0) {
                    return;
                }
                uVar3 = local_12;
                pHVar7 = file_a;
                local_12 = local_12 - 1;
                process_struct_1000_179c(0x2a, struct_a);
                if ((struct_a | uVar3) == 0) {
                    uVar3 = 0;
                    uVar5 = 0;
                } else {
                    pass1_1038_6520(CONCAT22(struct_a, uVar3));
                    uVar5 = extraout_DX_00;
                }
                read_from_file_1038_774e(uVar3, uVar5, pHVar7);
                if (uVar3 == 0) {
                    break;
                }
                ppcVar1 = (local_AX_11.field_0x10 + 4);
                (**ppcVar1)();
                struct_a = extraout_DX_01;
            }
            return;
        }
    }
    g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn write_to_file_1030_7418(struct_a: *mut AStruct771, file_a: *mut HFILE16) {
    let mut u_var1: u32;
    let mut in_AX: i32;
    let struct_b: *mut AStruct771;
    let b_success: bool;
    let pcVar2: *mut libc::c_char;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut local_ES_52: u16;
    let char_ptr_a: *mut libc::c_char;
    let mut local_3e: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_2a: u16;
    let mut char_array_14_a: [u8; 14];
    let char_ptr_a_4: *mut libc::c_char;
    let mut local_14: u16;
    let mut local_12: u16;
    let char_ptr_a_2: *mut libc::c_char;
    let char_ptr_a_5: *mut libc::c_char;
    let char_ptr_a_3: *mut libc::c_char;
    let puVar3: *mut u8;

    write_to_file_1030_16d6(struct_a, file_a);
    if (in_AX == 0) {
        return;
    }
    struct_b = struct_a;
    struct_b = &struct_b.field_0xc;
    puVar3 = (struct_a & 0xffff0000 | ZEXT24(struct_b));
    write_file_1008_7b4c(file_a, puVar3);
    if (puVar3 == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    local_ES_52 = (struct_a >> 0x10);
    char_ptr_a_2 = struct_b.field_0x12;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
    if (b_success == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_3 = *&struct_b.field_0x14;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
    if (b_success == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_4 = *&struct_b.field_0x16;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_4), 4);
    if (b_success == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7954(file_a, &struct_b.field_0x1e);
    if (b_success == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(file_a, (&struct_b.field_0x20 + 2));
    if (b_success == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_7a22(file_a, (&struct_b.field_0x24 + 2));
    if (b_success == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_5 = *(&struct_b.field_0x28 + 2);
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_5), 4);
    if (b_success == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_2 = *&struct_b[1].field_0x6;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
    if (b_success == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    char_ptr_a_2 = *&struct_b[1].field_0x8;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
    if (b_success == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    write_to_file_1008_79f0(file_a, &struct_b[1].field_0xa);
    if (b_success == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    if (&struct_b[1].field_0xe == 0) {
        char_ptr_a_4 = (char_ptr_a_4 & 0xffff0000);
    } else {
        uVar1 = &struct_b[1].field_0xe;
        char_ptr_a_4 = (char_ptr_a_4 & 0xffff0000 | *(uVar1 + 8));
    }
    char_ptr_a_3 = char_ptr_a_4;
    b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
    if (b_success == 0) {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    pass1_1008_5784(
        CONCAT22(char_ptr_a, char_array_14_a),
        &struct_b[1].field_0xe,
    );
    while (true) {
        pcVar2 = char_array_14_a;
        pass1_1008_5b12(CONCAT22(char_ptr_a, pcVar2));
        _local_14 = CONCAT22(extraout_DX, pcVar2);
        if ((extraout_DX | pcVar2) == 0) {
            if (&struct_b[1].field_0x12 == 0) {
                local_3e = 0;
            } else {
                uVar1 = &struct_b[1].field_0x12;
                local_3e = (uVar1 + 8);
            }
            local_2a = local_3e;
            b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &local_2a), 2);
            if (b_success == 0) {
                g_u16_1050_0310 = 0x6d0;
                return;
            }
            pass1_1008_5784(
                CONCAT22(char_ptr_a, char_array_14_a),
                &struct_b[1].field_0x12,
            );
            while (true) {
                pcVar2 = char_array_14_a;
                pass1_1008_5b12(CONCAT22(char_ptr_a, pcVar2));
                if ((extraout_DX_00 | pcVar2) == 0) {
                    return;
                }
                char_ptr_a_4 = (char_ptr_a_4 & 0xffff0000 | *(pcVar2 + 4));
                b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_4), 2);
                if (b_success == 0) {
                    g_u16_1050_0310 = 0x6d0;
                    return;
                }
                _local_14 = _local_14 & 0xffff0000 | *(pcVar2 + 6);
                b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &local_14), 2);
                if (b_success == 0) {
                    g_u16_1050_0310 = 0x6d0;
                    return;
                }
                char_ptr_a_2 = *(pcVar2 + 8);
                b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
                if (b_success == 0) {
                    break;
                }
                char_ptr_a_2 = *(pcVar2 + 10);
                b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_2), 2);
                if (b_success == 0) {
                    g_u16_1050_0310 = 0x6d0;
                    return;
                }
                char_ptr_a_3 = *(pcVar2 + 0xc);
                b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
                if (b_success == 0) {
                    g_u16_1050_0310 = 0x6d0;
                    return;
                }
            }
            g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(pcVar2 + 4);
        b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(_local_14 + 6);
        b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            break;
        }
        char_ptr_a_3 = *(_local_14 + 8);
        b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(_local_14 + 10);
        b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            g_u16_1050_0310 = 0x6d0;
            return;
        }
        char_ptr_a_3 = *(_local_14 + 0xc);
        b_success = write_to_file_1008_7e1c(file_a, CONCAT22(char_ptr_a, &char_ptr_a_3), 2);
        if (b_success == 0) {
            g_u16_1050_0310 = 0x6d0;
            return;
        }
    }
    g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn read_from_file_1030_778c(struct_b: *mut AStruct933, file_a: u32) {
    let mut u_var1: u32;
    let uVar2: u8;
    let local_AX_32: *mut AStruct933;
    let BVar3: bool;
    let paVar4: *mut AStruct493;
    let local_AX_482: *mut AStruct935;
    let mut uVar5: u16;
    let local_AX_662: *mut AStruct934;
    let local_AX_1036: *mut AStruct936;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uVar8: i32;
    let mut extraout_DX_00: i32;
    let paVar9: *mut AStruct199;
    let mut extraout_DX_01: i32;
    let mut iVar10: i32;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let base_ptr: *mut u8;
    let mut local_56: u16;
    let mut local_52: u16;
    let mut local_4a: u32;
    let mut local_46: u16;
    let mut local_42: u16;
    let mut local_3e: u32;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let local_2c: *mut u8;
    let mut local_28: u16;
    let mut local_24: u16;
    let mut local_20: u16;
    let mut local_e: u16;
    let offset_ptr: *mut u8;
    let mut uVar6: u32;
    let puVar7: *mut u8;
    let fn_ptr_a: fn();

    uVar2 = pass1_1030_1730(struct_b, file_a);
    if (CONCAT31(extraout_var, uVar2) != 0) {
        local_AX_32 = struct_b;
        local_AX_32 = &local_AX_32.field_0xc;
        uVar6 = struct_b & 0xffff0000 | ZEXT24(local_AX_32);
        read_file_1008_7bc8(file_a, uVar6);
        if ((uVar6 != 0)
            && (
                BVar3 = read_file_1008_7dee(file_a, CONCAT22(base_ptr, &offset_ptr), 2),
                BVar3 != 0,
            ))
        {
            uVar11 = (struct_b >> 0x10);
            local_AX_32.field_0x12 = offset_ptr;
            BVar3 = read_file_1008_7dee(file_a, CONCAT22(base_ptr, &offset_ptr), 2);
            if (BVar3 != 0) {
                local_AX_32.field_0x14 = offset_ptr;
                BVar3 = read_file_1008_7dee(
                    file_a,
                    (struct_b & 0xffff0000 | &local_AX_32.field_0x16),
                    4,
                );
                if (BVar3 != 0) {
                    uVar6 = struct_b & 0xffff0000 | &local_AX_32.field_0x1e;
                    read_file_1008_76e4(file_a, uVar6);
                    if (uVar6 != 0) {
                        uVar6 = struct_b & 0xffff0000 | &local_AX_32.field_0x22;
                        read_file_1008_77cc(file_a, uVar6);
                        if (uVar6 != 0) {
                            uVar6 = struct_b & 0xffff0000 | &local_AX_32.field_0x26;
                            read_file_1008_77cc(file_a, uVar6);
                            if ((uVar6 != 0)
                                && (
                                    uVar12 = extraout_DX,
                                    BVar3 = read_file_1008_7dee(
                                        file_a,
                                        (struct_b & 0xffff0000 | ZEXT24(&local_AX_32.field_0x2a)),
                                        4,
                                    ),
                                    BVar3 != 0,
                                ))
                            {
                                if (local_AX_32.field_0x2a != 0) {
                                    uVar1 = local_AX_32.field_0x2a;
                                    paVar4 = pass1_1028_e1ec(
                                        _PTR_LOOP_1050_65e2,
                                        uVar1,
                                        (uVar1 >> 0x10),
                                    );
                                    local_AX_32.field_0x2e = paVar4;
                                    local_AX_32.field_0x30 = uVar12;
                                }
                                if (PTR_LOOP_1050_0312 < 2) {
                                    return;
                                }
                                BVar3 = read_file_1008_7dee(
                                    file_a,
                                    (struct_b & 0xffff0000 | &local_AX_32.field_0x32),
                                    2,
                                );
                                if ((BVar3 != 0)
                                    && (
                                        BVar3 = read_file_1008_7dee(
                                            file_a,
                                            (struct_b & 0xffff0000 | &local_AX_32.field_0x34),
                                            2,
                                        ),
                                        BVar3 != 0,
                                    ))
                                {
                                    uVar6 = struct_b & 0xffff0000 | &local_AX_32.field_0x36;
                                    pass1_1008_766e(file_a, uVar6);
                                    if ((uVar6 != 0)
                                        && (
                                            BVar3 = read_file_1008_7dee(
                                                file_a,
                                                CONCAT22(base_ptr, &local_20),
                                                2,
                                            ),
                                            BVar3 != 0,
                                        ))
                                    {
                                        local_e = 0;
                                        while (local_e < local_20) {
                                            local_3e = _PTR_LOOP_1050_68a2;
                                            puVar7 = _PTR_LOOP_1050_68a2;
                                            alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                                            uVar8 = puVar7;
                                            _local_32 = (puVar7 & 0xffff | extraout_DX_00 << 0x10);
                                            paVar9 = (extraout_DX_00 | uVar8);
                                            if (paVar9 == 0x0) {
                                                local_2c = 0x0;
                                            } else {
                                                *_local_32 = s_1_1050_389a;
                                                (uVar8 + 2) = &PTR_LOOP_1050_1008;
                                                (uVar8 + 4) = 0;
                                                (uVar8 + 6) = 0;
                                                (uVar8 + 8) = 0;
                                                (uVar8 + 10) = 0;
                                                (uVar8 + 0xc) = 0;
                                                *_local_32 = 0x56ce;
                                                (uVar8 + 2) = 0x1018;
                                                local_2c = _local_32;
                                            }
                                            BVar3 = read_file_1008_7dee(
                                                file_a,
                                                CONCAT22(base_ptr, &local_28),
                                                2,
                                            );
                                            if ((((BVar3 == 0)
                                                || (
                                                    BVar3 = read_file_1008_7dee(
                                                        file_a,
                                                        CONCAT22(base_ptr, &local_24),
                                                        2,
                                                    ),
                                                    BVar3 == 0,
                                                ))
                                                || (
                                                    BVar3 = read_file_1008_7dee(
                                                        file_a,
                                                        CONCAT22(base_ptr, &local_2e),
                                                        2,
                                                    ),
                                                    BVar3 == 0,
                                                ))
                                                || ((
                                                    local_AX_482 = local_2c,
                                                    local_AX_482 = &local_AX_482.field_0xa,
                                                    BVar3 = read_file_1008_7dee(
                                                        file_a,
                                                        (local_2c & 0xffff0000
                                                            | ZEXT24(local_AX_482)),
                                                        2,
                                                    ),
                                                    BVar3 == 0
                                                        || (
                                                            BVar3 = read_file_1008_7dee(
                                                                file_a,
                                                                (local_2c & 0xffff0000
                                                                    | (local_2c + 0xc)),
                                                                2,
                                                            ),
                                                            BVar3 == 0,
                                                        ),
                                                )))
                                            {
                                                // goto LAB_1030_77be;
                                            }
                                            uVar12 = (local_2c >> 0x10);
                                            iVar10 = local_2c;
                                            (iVar10 + 4) = local_28;
                                            (iVar10 + 6) = local_24;
                                            (iVar10 + 8) = local_2e;
                                            if (&local_AX_32.field_0x3a == 0) {
                                                uVar5 = local_2e;
                                                process_struct_1000_179c(0xc, paVar9);
                                                _local_32 = CONCAT22(paVar9, uVar5);
                                                uVar8 = paVar9 | uVar5;
                                                if (uVar8 == 0) {
                                                    &local_AX_32.field_0x3a = 0;
                                                } else {
                                                    paVar9 = process_struct_1008_574a(CONCAT22(
                                                        paVar9, uVar5,
                                                    ));
                                                    local_AX_32.field_0x3a = paVar9;
                                                    &local_AX_32.field_0x3c = uVar8;
                                                }
                                            }
                                            fn_ptr_a = (&local_AX_32.field_0x3a + 8);
                                            (**fn_ptr_a)();
                                            local_e = local_e + 1;
                                        }
                                        BVar3 = read_file_1008_7dee(
                                            file_a,
                                            CONCAT22(base_ptr, &local_56),
                                            2,
                                        );
                                        if (BVar3 != 0) {
                                            local_52 = 0;
                                            while (true) {
                                                if (local_56 <= local_52) {
                                                    return;
                                                }
                                                local_2c = _PTR_LOOP_1050_68a2;
                                                puVar7 = _PTR_LOOP_1050_68a2;
                                                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                                                uVar8 = puVar7;
                                                _local_32 =
                                                    (puVar7 & 0xffff | extraout_DX_01 << 0x10);
                                                paVar9 = (extraout_DX_01 | uVar8);
                                                if (paVar9 == 0x0) {
                                                    local_4a = 0;
                                                } else {
                                                    *_local_32 = s_1_1050_389a;
                                                    (uVar8 + 2) = &PTR_LOOP_1050_1008;
                                                    (uVar8 + 4) = 0;
                                                    (uVar8 + 6) = 0;
                                                    (uVar8 + 8) = 0;
                                                    (uVar8 + 10) = 0;
                                                    (uVar8 + 0xc) = 0;
                                                    *_local_32 = 0x56ce;
                                                    (uVar8 + 2) = 0x1018;
                                                    local_4a = _local_32;
                                                }
                                                BVar3 = read_file_1008_7dee(
                                                    file_a,
                                                    CONCAT22(base_ptr, &local_46),
                                                    2,
                                                );
                                                if (((BVar3 == 0)
                                                    || (
                                                        BVar3 = read_file_1008_7dee(
                                                            file_a,
                                                            CONCAT22(base_ptr, &local_42),
                                                            2,
                                                        ),
                                                        BVar3 == 0,
                                                    ))
                                                    || ((
                                                        BVar3 = read_file_1008_7dee(
                                                            file_a,
                                                            CONCAT22(base_ptr, &local_3e),
                                                            2,
                                                        ),
                                                        BVar3 == 0
                                                            || ((
                                                                BVar3 = read_file_1008_7dee(
                                                                    file_a,
                                                                    (local_4a & 0xffff0000
                                                                        | (local_4a + 10)),
                                                                    2,
                                                                ),
                                                                BVar3 == 0
                                                                    || (
                                                                        BVar3 = read_file_1008_7dee(
                                                                            file_a,
                                                                            (local_4a & 0xffff0000
                                                                                | (local_4a + 0xc)),
                                                                            2,
                                                                        ),
                                                                        BVar3 == 0,
                                                                    ),
                                                            )),
                                                    )))
                                                {
                                                    break;
                                                }
                                                uVar12 = (local_4a >> 0x10);
                                                iVar10 = local_4a;
                                                (iVar10 + 4) = local_46;
                                                (iVar10 + 6) = local_42;
                                                (iVar10 + 8) = local_3e;
                                                if (&local_AX_32.field_0x3e == 0) {
                                                    process_struct_1000_179c(0xc, paVar9);
                                                    _local_32 = CONCAT22(paVar9, local_3e);
                                                    uVar8 = paVar9 | local_3e;
                                                    if (uVar8 == 0) {
                                                        &local_AX_32.field_0x3e = 0;
                                                    } else {
                                                        paVar9 = process_struct_1008_574a(
                                                            CONCAT22(paVar9, local_3e),
                                                        );
                                                        &local_AX_32.field_0x3e = paVar9;
                                                        local_AX_32.field_0x40 = uVar8;
                                                    }
                                                }
                                                fn_ptr_a = (&local_AX_32.field_0x3e + 8);
                                                (**fn_ptr_a)();
                                                local_52 = local_52 + 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // LAB_1030_77be:
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1030_5dbe(param_1: *mut AStruct771, param_2: *mut HFILE16) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let BVar4: bool;
    let puVar5: *mut u8;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut local_c: u16;

    iVar3 = write_to_file_1030_1978(param_1, param_2);
    if (iVar3 != 0) {
        uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        write_to_file_1008_7c2a(param_2, (iVar6 + 0x10));
        if (iVar3 != 0) {
            uVar1 = (iVar6 + 0x10);
            puVar5 = (uVar1 & 0xffff0000 | (uVar1 + 4));
            write_file_1008_7b4c(param_2, puVar5);
            if (puVar5 != 0) {
                uVar2 = (iVar6 + 0x10);
                local_c = (uVar2 + 10);
                BVar4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
                if (BVar4 != 0) {
                    uVar2 = (iVar6 + 0x10);
                    if ((uVar2 + 10) == 0) {
                        return;
                    }
                    uVar2 = (iVar6 + 0x10);
                    uVar7 = (uVar2 >> 0x10);
                    iVar3 = uVar2;
                    BVar4 = write_to_file_1008_7e1c(param_2, (iVar3 + 0xc), ((iVar3 + 10) * 2));
                    if (BVar4 != 0) {
                        return;
                    }
                }
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn read_from_file_1030_5e70(param_1: u32, param_2: *mut HFILE16) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let piVar3: *mut i32;
    let in_AX: *mut AStruct94;
    let puVar4: *mut u8;
    let mut iVar5: i32;
    let BVar6: bool;
    let mut uVar7: i32;
    let in_DX: *mut u16;
    let puVar8: *mut u16;
    let extraout_DX: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let mut iVar9: i32;
    let mut uVar10: u16;
    let mut unaff_SS: u16;
    let ppVar11: *mut pass1_struct_1;
    let mut uVar12: u16;
    let mut local_420: u32;
    let mut local_416: u16;
    let mut local_414: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    let mut local_406: u16;
    let mut local_404: u16;
    let mut local_402: [u8; 1024];

    uVar12 = (param_1 >> 0x10);
    pass1_1030_19b4(param_1, param_2);
    if (in_AX != 0x0) {
        if (__g_AStruct94_ptr_1 == 0) {
            _g_AStruct94_ptr_1 = in_AX;
            struct_fn_1000_160a();
            g_u16_ptr_1050_5f2e = in_DX;
        } else {
        }
        alloc_mem_1000_1708();
        _local_40a = CONCAT22(g_u16_ptr_1050_5f2e, _g_AStruct94_ptr_1);
        puVar8 = (g_u16_ptr_1050_5f2e | _g_AStruct94_ptr_1);
        iVar9 = param_1;
        if (puVar8 == 0x0) {
            (iVar9 + 0x10) = 0;
        } else {
            zero_list_1008_3e38(CONCAT22(g_u16_ptr_1050_5f2e, &_g_AStruct94_ptr_1.field_0x4));
            (iVar9 + 0x10) = _local_40a;
            puVar8 = g_u16_ptr_1050_5f2e;
        }
        puVar4 = local_402;
        read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_SS, puVar4));
        if (puVar4 != 0x0) {
            puVar4 = local_402;
            pass1_fn_1008_60e8(puVar4);
            piVar3 = (iVar9 + 0x10);
            let pi_var3_val = unsafe { *piVar3 };
            unsafe { *piVar3 = puVar4 };
            (piVar3 + 2) = puVar8;
            uVar1 = (iVar9 + 0x10);
            iVar5 = uVar1 + 4;
            read_file_1008_7bc8(param_2, (param_2 >> 0x10), iVar5, (uVar1 >> 0x10));
            if (iVar5 != 0) {
                uVar2 = (iVar9 + 0x10);
                local_420._0_2_ = uVar2 + 10;
                struct_a = extraout_DX;
                BVar6 = read_file_1008_7dee(param_2, (uVar2 & 0xffff0000 | local_420), 2);
                if (BVar6 != 0) {
                    uVar1 = (iVar9 + 0x10);
                    uVar10 = (uVar1 >> 0x10);
                    iVar5 = uVar1;
                    if ((iVar5 + 10) == 0) {
                        // LAB_1030_5fb7:
                        ppVar11 = process_struct_1010_20ba(
                            _g_AStruct372_1050_0ed0,
                            CONCAT22(local_420, 0x2f),
                        );
                        pass1_1018_04ca(ppVar11, (iVar9 + 4));
                        return;
                    }
                    local_420._0_2_ = (iVar5 + 10) * 2;
                    uVar7 = local_420;
                    process_struct_1000_179c(local_420, struct_a);
                    uVar1 = (iVar9 + 0x10);
                    uVar10 = (uVar1 >> 0x10);
                    iVar5 = uVar1;
                    (iVar5 + 0xc) = uVar7;
                    (iVar5 + 0xe) = struct_a;
                    uVar1 = (iVar9 + 0x10);
                    BVar6 = read_file_1008_7dee(param_2, *(uVar1 + 0xc), local_420);
                    if (BVar6 != 0) {}
                    // goto LAB_1030_5fb7;
                }
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1030_5c1a(param_1: &mut string, param_2: *mut HFILE16) -> bool {
    let uVar1: u8;
    let extraout_AH: u8;
    let BVar2: bool;

    uVar1 = write_to_file_1008_7cac(param_2, 9);
    BVar2 = CONCAT11(extraout_AH, uVar1);
    if (BVar2 != 0) {
        BVar2 = write_to_file_1008_7e1c(param_2, param_1, 0x24);
        if (BVar2 == 0) {
            g_u16_1050_0310 = 0x6d0;
            return BVar2;
        }
        BVar2 = 1;
    }
    return BVar2;
}

pub fn read_file_1030_5c52(buf_a: *mut u8, file_a: *mut HFILE16) -> bool {
    let uVar1: u8;
    let extraout_AH: u8;
    let BVar2: bool;

    uVar1 = read_file_1008_7cfe(file_a);
    BVar2 = CONCAT11(extraout_AH, uVar1);
    if (BVar2 != 0) {
        BVar2 = read_file_1008_7dee(file_a, buf_a, 0x24);
        if (BVar2 == 0) {
            g_u16_1050_0310 = 0x6d2;
            return BVar2;
        }
        BVar2 = 1;
    }
    return BVar2;
}

pub fn write_to_file_1030_56f6(param_1: *mut AStruct771, param_2: *mut HFILE16) {
    let puVar1: *mut u16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let BVar5: bool;
    let mut iVar6: i32;
    let puVar7: *mut u8;
    let mut uVar8: u16;
    let mut unaff_SS: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    iVar4 = write_to_file_1030_1978(param_1, param_2);
    if (iVar4 != 0) {
        uVar8 = (param_1 >> 0x10);
        iVar4 = param_1;
        local_e = *(iVar4 + 0x10);
        BVar5 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_e), 2);
        if (BVar5 != 0) {
            uVar3 = (iVar4 + 0x10);
            local_8 = (uVar3 + 2);
            BVar5 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_8), 2);
            if ((BVar5 != 0)
                && (
                    uVar3 = (iVar4 + 0x10),
                    write_to_file_1008_7c2a(param_2, (uVar3 + 4)),
                    BVar5 != 0,
                ))
            {
                uVar3 = (iVar4 + 0x10);
                local_8 = (uVar3 + 0x1a);
                BVar5 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_8), 2);
                if (BVar5 != 0) {
                    local_4 = 0;
                    let pu_var1_val = unsafe { *puVar1 };
                    while (
                        uVar3 = (iVar4 + 0x10),
                        puVar1 = (uVar3 + 0x1a),
                        pu_var1_val != local_4 && local_4 <= pu_var1_val,
                    ) {
                        iVar6 = local_4 * 6;
                        uVar3 = (iVar4 + 0x10);
                        uVar2 = (uVar3 + 0x16);
                        write_file_1008_7b4c(param_2, (uVar2 & 0xffff0000 | (uVar2 + iVar6)));
                        if (iVar6 == 0) {}
                        // goto LAB_1030_5734;
                        local_4 = local_4 + 1;
                    }
                    puVar7 = (param_1 & 0xffff0000 | (iVar4 + 0x14));
                    write_file_1008_7b4c(param_2, puVar7);
                    if (puVar7 != 0) {
                        local_8 = (iVar4 + 0x1c);
                        BVar5 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_8), 2);
                        if (BVar5 != 0) {
                            return;
                        }
                    }
                }
            }
        }
        // LAB_1030_5734:
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn read_from_file_1030_581e(param_1: u32, param_2: *mut HFILE16) {
    let puVar1: *mut u16;
    let mut uVar2: u32;
    let in_AX: *mut AStruct94;
    let BVar3: bool;
    let puVar4: *mut u8;
    let mut in_i16_5: u16;
    let mut uVar5: u32;
    let in_DX: *mut u16;
    let extraout_DX: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let mut iVar6: i32;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut unaff_SS: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_432: u32;
    let mut local_426: u32;
    let mut local_41c: u16;
    let mut local_41a: u16;
    let mut local_410: u16;
    let mut local_40e: u16;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: [u8; 1024];
    let mut local_8: u32;
    let mut local_4: u16;

    uVar10 = (param_2 >> 0x10);
    uVar9 = (param_1 >> 0x10);
    pass1_1030_19b4(param_1, param_2);
    if (in_AX != 0x0) {
        if (__g_AStruct94_ptr_1 == 0) {
            _g_AStruct94_ptr_1 = in_AX;
            struct_fn_1000_160a();
            g_u16_ptr_1050_5f2e = in_DX;
        } else {
        }
        alloc_mem_1000_1708(0x20, 0, 1, _g_AStruct94_ptr_1, g_u16_ptr_1050_5f2e);
        if ((g_u16_ptr_1050_5f2e | _g_AStruct94_ptr_1) == 0) {
            _g_AStruct94_ptr_1 = 0x0;
            struct_a = 0x0;
        } else {
            pass1_1030_84ae(_g_AStruct94_ptr_1, g_u16_ptr_1050_5f2e);
            struct_a = extraout_DX;
        }
        iVar6 = param_1;
        (iVar6 + 0x10) = _g_AStruct94_ptr_1;
        (iVar6 + 0x12) = struct_a;
        BVar3 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2);
        if (BVar3 != 0) {
            uVar5 = (_PTR_LOOP_1050_65e2 + 0x52);
            local_8 = uVar5;
            pass1_1030_4782(uVar5, (uVar5 >> 0x10), 0, 1, local_4);
            (iVar6 + 0x10) = uVar5;
            (iVar6 + 0x12) = struct_a;
            BVar3 = read_file_1008_7dee(param_2, CONCAT22(struct_a, (iVar6 + 0x10) + 2), 2);
            if (BVar3 != 0) {
                puVar4 = local_408;
                read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_SS, puVar4));
                if (puVar4 != 0x0) {
                    uVar2 = (iVar6 + 0x10);
                    error_check_1000_17ce((uVar2 + 4));
                    puVar4 = local_408;
                    pass1_fn_1008_60e8(puVar4);
                    uVar2 = (iVar6 + 0x10);
                    uVar8 = (uVar2 >> 0x10);
                    iVar7 = uVar2;
                    (iVar7 + 4) = puVar4;
                    (iVar7 + 6) = struct_a;
                    uVar5 = (iVar6 + 0x10);
                    BVar3 = read_file_1008_7dee(param_2, (uVar5 & 0xffff0000 | (uVar5 + 0x1a)), 2);
                    if (BVar3 != 0) {
                        uVar2 = (iVar6 + 0x10);
                        iVar7 = (uVar2 + 0x1a);
                        in_i16_5 = iVar7 * 6;
                        process_struct_1000_179c(in_i16_5, struct_a);
                        _local_410 = CONCAT22(struct_a, in_i16_5);
                        if ((struct_a | in_i16_5) == 0) {
                            uVar2 = (iVar6 + 0x10);
                            (uVar2 + 0x16) = 0;
                        } else {
                            call_fn_ptr_1000_5586(
                                0x3e38,
                                &PTR_LOOP_1050_1008,
                                iVar7,
                                6,
                                in_i16_5,
                                struct_a,
                            );
                            uVar2 = (iVar6 + 0x10);
                            (uVar2 + 0x16) = _local_410;
                        }
                        local_40c = 0;
                        let pu_var1_val = unsafe { *puVar1 };
                        while (
                            uVar2 = (iVar6 + 0x10),
                            puVar1 = (uVar2 + 0x1a),
                            pu_var1_val != local_40c && local_40c <= pu_var1_val,
                        ) {
                            iVar7 = local_40c * 6;
                            uVar2 = (iVar6 + 0x10);
                            uVar2 = (uVar2 + 0x16);
                            read_file_1008_7bc8(param_2, uVar10, uVar2 + iVar7, (uVar2 >> 0x10));
                            if (iVar7 == 0) {}
                            // goto LAB_1030_58a7;
                            local_40c = local_40c + 1;
                        }
                        iVar7 = iVar6 + 0x14;
                        read_file_1008_7bc8(param_2, uVar10, iVar7, uVar9);
                        if ((iVar7 != 0)
                            && (
                                BVar3 = read_file_1008_7dee(
                                    param_2,
                                    (param_1 & 0xffff0000 | (iVar6 + 0x1c)),
                                    2,
                                ),
                                BVar3 != 0,
                            ))
                        {
                            return;
                        }
                    }
                }
            }
        }
        // LAB_1030_58a7:
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn read_from_file_1030_4e70(
    param_1: *mut char,
    param_2: *mut u32,
    param_3: *mut *mut libc::c_void,
    param_4: *mut char,
) {
    let file: HFILE16;
    let mut iVar1: i32;
    let mut extraout_DX: i32;
    let path: *mut libc::c_char;
    let mut uVar2: u16;
    let puVar3: *mut u8;
    let mut local_3c: u32;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: [u8; 26];
    let mut local_14: u32;

    unsafe {
        *param_3 = 0;
        *param_2 = 0;
    }
    if (param_4 != 0x0) {
        puVar3 = local_2e;
        uVar2 = 0;
        path = pass1_1030_5164(param_1, param_4);
        iVar1 = path;
        dos3_call_1000_51aa(iVar1, (path >> 0x10), uVar2, puVar3);
        if (iVar1 == 0) {
            unsafe { *param_2 = local_14 };
            file = _lopen16(0, path);
            iVar1 = file + 1;
            if (iVar1 != 0) {
                let param_2_val = unsafe { *param_2 };
                let param_3_val = unsafe { *param_3 };

                alloc_mem_1000_0a48(
                    1,
                    param_2_val,
                    (param_2_val >> 0x10),
                    __g_AStruct94_ptr_1,
                    (__g_AStruct94_ptr_1 >> 0x10),
                );
                param_3 = iVar1;
                (param_3 + 2) = extraout_DX;
                if ((extraout_DX | param_3) != 0) {
                    _local_38 = _hread(param_2_val, param_3_val, file);
                    _lclose16(file);
                    local_3c = param_3_val;
                    while (_local_38 != 0) {
                        if ((*(*local_3c + 0x608b) & 0x20) == 0) {
                            *local_3c = *local_3c + -0x80;
                        }
                        local_3c = local_3c & 0xffff0000 | (local_3c + 1);
                        _local_38 = _local_38 + -1;
                    }
                    return;
                }
            }
        }
    }
    return;
}

pub fn read_from_file_1030_33f0(param_1: u32, param_2: *mut HFILE16) {
    let local_AX_14: *mut AStruct874;
    let BVar1: bool;

    local_AX_14 = param_1;
    local_AX_14 = &local_AX_14.field_0x4;
    BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | ZEXT24(local_AX_14)), 0x16c);
    if (((((BVar1 != 0)
        && (
            BVar1 = read_file_1008_7dee(
                param_2,
                (param_1 & 0xffff0000 | &local_AX_14.field_0x174),
                0xc,
            ),
            BVar1 != 0,
        ))
        && (
            BVar1 = read_file_1008_7dee(
                param_2,
                (param_1 & 0xffff0000 | &local_AX_14.field_0x180),
                0xc,
            ),
            BVar1 != 0,
        ))
        && ((
            BVar1 = read_file_1008_7dee(
                param_2,
                (param_1 & 0xffff0000 | &local_AX_14.field_0x18c),
                0x18,
            ),
            BVar1 != 0
                && (
                    BVar1 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | &local_AX_14.field_0x1a8),
                        2,
                    ),
                    BVar1 != 0,
                ),
        )))
        && (
            BVar1 = read_file_1008_7dee(
                param_2,
                (param_1 & 0xffff0000 | &local_AX_14.field_0x1aa),
                4,
            ),
            BVar1 != 0,
        ))
    {
        if (PTR_LOOP_1050_0312 < 2) {
            return;
        }
        BVar1 = read_file_1008_7dee(
            param_2,
            (param_1 & 0xffff0000 | &local_AX_14.field_0x170),
            4,
        );
        if ((BVar1 != 0)
            && (
                BVar1 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | ZEXT24(local_AX_14 + 1)),
                    2,
                ),
                BVar1 != 0,
            ))
        {
            return;
        }
    }
    g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn write_to_file_1030_32e4(param_1: u32, param_2: *mut HFILE16) {
    let local_AX_14: *mut AStruct873;
    let BVar1: bool;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut local_16: u32;
    let mut local_c: u16;
    let mut local_a: u32;

    local_AX_14 = param_1;
    local_AX_14 = &local_AX_14.field_0x4;
    BVar1 = write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | ZEXT24(local_AX_14)), 0x16c);
    if (BVar1 != 0) {
        BVar1 = write_to_file_1008_7e1c(
            param_2,
            (param_1 & 0xffff0000 | &local_AX_14.field_0x174),
            0xc,
        );
        if (BVar1 != 0) {
            BVar1 = write_to_file_1008_7e1c(
                param_2,
                (param_1 & 0xffff0000 | &local_AX_14.field_0x180),
                0xc,
            );
            if (BVar1 != 0) {
                BVar1 = write_to_file_1008_7e1c(
                    param_2,
                    (param_1 & 0xffff0000 | &local_AX_14.field_0x18c),
                    0x18,
                );
                if (BVar1 != 0) {
                    uVar2 = (param_1 >> 0x10);
                    local_c = local_AX_14.field_0x1a8;
                    BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
                    if (BVar1 != 0) {
                        local_16 = local_AX_14.field_0x1aa;
                        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_16), 4);
                        if (BVar1 != 0) {
                            local_a = local_AX_14.field_0x170;
                            BVar1 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_a), 4);
                            if (BVar1 != 0) {
                                local_c = local_AX_14.field_0x1ae;
                                BVar1 = write_to_file_1008_7e1c(
                                    param_2,
                                    CONCAT22(unaff_SS, &local_c),
                                    2,
                                );
                                if (BVar1 != 0) {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn write_to_file_1030_2aca(param_1: *mut AStruct771, param_2: *mut HFILE16) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let BVar4: bool;
    let puVar5: *mut u8;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut unaff_SS: u16;
    let mut local_18: u32;
    let mut local_c: u16;
    let mut local_6: u16;

    iVar3 = write_to_file_1030_1978(param_1, param_2);
    if (iVar3 == 0) {
        return;
    }
    uVar7 = (param_1 >> 0x10);
    iVar3 = param_1;
    local_c = *(iVar3 + 0x10);
    BVar4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
    if ((BVar4 != 0)
        && (
            uVar2 = (iVar3 + 0x10),
            write_to_file_1008_7c2a(param_2, (uVar2 + 2)),
            BVar4 != 0,
        ))
    {
        uVar1 = (iVar3 + 0x10);
        puVar5 = (uVar1 & 0xffff0000 | (uVar1 + 6));
        write_file_1008_7b4c(param_2, puVar5);
        if (puVar5 != 0) {
            uVar2 = (iVar3 + 0x10);
            local_6 = (uVar2 + 0xc);
            BVar4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_6), 2);
            if (BVar4 != 0) {
                uVar2 = (iVar3 + 0x10);
                local_18 = (uVar2 + 0xe);
                BVar4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_18), 4);
                if ((BVar4 != 0)
                    && (
                        uVar1 = (iVar3 + 0x10),
                        BVar4 = write_to_file_1008_7e1c(
                            param_2,
                            (uVar1 & 0xffff0000 | (uVar1 + 0x12)),
                            0x10,
                        ),
                        BVar4 != 0,
                    ))
                {
                    uVar2 = (iVar3 + 0x10);
                    local_c = (uVar2 + 0x22);
                    BVar4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
                    if ((BVar4 != 0)
                        && ((
                            uVar2 = (iVar3 + 0x10),
                            (uVar2 + 0x22) == 0
                                || (
                                    uVar2 = (iVar3 + 0x10),
                                    uVar8 = (uVar2 >> 0x10),
                                    iVar6 = uVar2,
                                    BVar4 = write_to_file_1008_7e1c(
                                        param_2,
                                        (iVar6 + 0x24),
                                        ((iVar6 + 0x22) * 2),
                                    ),
                                    BVar4 != 0,
                                ),
                        )))
                    {
                        local_c = (iVar3 + 0x14);
                        BVar4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
                        if (BVar4 != 0) {
                            local_c = (iVar3 + 0x16);
                            BVar4 =
                                write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
                            if (BVar4 != 0) {
                                local_c = (iVar3 + 0x18);
                                BVar4 = write_to_file_1008_7e1c(
                                    param_2,
                                    CONCAT22(unaff_SS, &local_c),
                                    2,
                                );
                                if (BVar4 != 0) {
                                    local_c = (iVar3 + 0x1a);
                                    BVar4 = write_to_file_1008_7e1c(
                                        param_2,
                                        CONCAT22(unaff_SS, &local_c),
                                        2,
                                    );
                                    if (BVar4 != 0) {
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn read_file_1030_2c8a(param_1: u32, param_2: *mut HFILE16) {
    let puVar1: *mut u16;
    let in_AX: *mut AStruct94;
    let BVar2: bool;
    let local_AX_184: *mut u8;
    let puVar3: *mut u8;
    let mut uVar4: u16;
    let in_DX: *mut u16;
    let puVar5: *mut u16;
    let extraout_DX: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let local_BX_91: *mut AStruct866;
    let mut iVar6: i32;
    let mut local_SI__1: u16;
    let mut local_ES_210: u16;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let ppVar8: *mut pass1_struct_1;
    let mut uVar9: u16;
    let mut local_424: u32;
    let mut local_41a: u16;
    let mut local_418: u16;
    let mut local_40e: u16;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    let mut local_406: u16;
    let mut local_404: u16;
    let mut local_402: [u8; 1024];

    uVar9 = (param_1 >> 0x10);
    pass1_1030_19b4(param_1, param_2);
    if (in_AX == 0x0) {
        return;
    }
    if (__g_AStruct94_ptr_1 == 0) {
        _g_AStruct94_ptr_1 = in_AX;
        struct_fn_1000_160a();
        g_u16_ptr_1050_5f2e = in_DX;
    } else {
    }
    alloc_mem_1000_1708();
    _local_40e = CONCAT22(g_u16_ptr_1050_5f2e, _g_AStruct94_ptr_1);
    puVar5 = (g_u16_ptr_1050_5f2e | _g_AStruct94_ptr_1);
    local_BX_91 = param_1;
    if (puVar5 == 0x0) {
        local_BX_91.field_0x10 = 0x0;
    } else {
        zero_list_1008_3e38(CONCAT22(g_u16_ptr_1050_5f2e, &_g_AStruct94_ptr_1.field_0x6));
        local_BX_91.field_0x10 = _local_40e;
        puVar5 = g_u16_ptr_1050_5f2e;
    }
    BVar2 = read_file_1008_7dee(param_2, local_BX_91.field_0x10, 2);
    if (BVar2 != 0) {
        local_AX_184 = local_402;
        read_file_into_str_1008_7c6e(param_2, CONCAT22(unaff_SS, local_AX_184));
        if (local_AX_184 != 0x0) {
            puVar3 = local_402;
            pass1_fn_1008_60e8(puVar3);
            puVar1 = local_BX_91.field_0x10;
            local_ES_210 = (puVar1 >> 0x10);
            iVar6 = puVar1;
            *(iVar6 + 2) = puVar3;
            (iVar6 + 4) = puVar5;
            puVar1 = local_BX_91.field_0x10;
            iVar6 = puVar1 + 6;
            read_file_1008_7bc8(param_2, (param_2 >> 0x10), iVar6, (puVar1 >> 0x10));
            if ((((iVar6 != 0)
                && (
                    puVar1 = local_BX_91.field_0x10,
                    struct_a = extraout_DX,
                    BVar2 = read_file_1008_7dee(param_2, (puVar1 & 0xffff0000 | (puVar1 + 0xc)), 2),
                    BVar2 != 0,
                ))
                && (
                    puVar1 = local_BX_91.field_0x10,
                    BVar2 = read_file_1008_7dee(param_2, (puVar1 & 0xffff0000 | (puVar1 + 0xe)), 4),
                    BVar2 != 0,
                ))
                && ((
                    puVar1 = local_BX_91.field_0x10,
                    BVar2 =
                        read_file_1008_7dee(param_2, (puVar1 & 0xffff0000 | (puVar1 + 0x12)), 0x10),
                    BVar2 != 0
                        && (
                            puVar1 = local_BX_91.field_0x10,
                            BVar2 = read_file_1008_7dee(
                                param_2,
                                (puVar1 & 0xffff0000 | (puVar1 + 0x22)),
                                2,
                            ),
                            BVar2 != 0,
                        ),
                )))
            {
                puVar1 = local_BX_91.field_0x10;
                if ((puVar1 + 0x22) != 0) {
                    puVar1 = local_BX_91.field_0x10;
                    uVar7 = (puVar1 >> 0x10);
                    iVar6 = puVar1;
                    uVar4 = (iVar6 + 0x22) * 2;
                    process_struct_1000_179c(uVar4, struct_a);
                    (iVar6 + 0x24) = uVar4;
                    (iVar6 + 0x26) = struct_a;
                    puVar1 = local_BX_91.field_0x10;
                    uVar7 = (puVar1 >> 0x10);
                    iVar6 = puVar1;
                    BVar2 = read_file_1008_7dee(param_2, *(iVar6 + 0x24), ((iVar6 + 0x22) * 2));
                    if (BVar2 == 0) {
                        g_u16_1050_0310 = 0x6d2;
                        return;
                    }
                }
                BVar2 = read_file_1008_7dee(
                    param_2,
                    (param_1 & 0xffff0000 | &local_BX_91.field_0x14),
                    2,
                );
                if (((BVar2 != 0)
                    && (
                        BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_404), 2),
                        BVar2 != 0,
                    ))
                    && ((
                        BVar2 = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | &local_BX_91.field_0x18),
                            2,
                        ),
                        BVar2 != 0
                            && (
                                BVar2 =
                                    read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_406), 2),
                                BVar2 != 0,
                            ),
                    )))
                {
                    local_BX_91.field_0x16 = local_404;
                    local_BX_91.field_0x1a = local_406;
                    ppVar8 = process_struct_1010_20ba(
                        _g_AStruct372_1050_0ed0,
                        CONCAT22(local_SI__1, 0x2f),
                    );
                    pass1_1018_04a4(ppVar8, local_BX_91.field_0x4);
                    pass1_1010_82f8(_g_struct_73_1050_14cc, *local_BX_91.field_0x10);
                    return;
                }
            }
        }
    }
    g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn pass1_1030_227a(param_1: *mut AStruct771, param_2: *mut HFILE16) {
    let mut iVar1: i32;
    let BVar2: bool;

    iVar1 = write_to_file_1030_1978(param_1, param_2);
    if (iVar1 != 0) {
        iVar1 = param_1;
        BVar2 = write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (iVar1 + 0x10)), 0x106);
        if (BVar2 != 0) {
            BVar2 =
                write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (iVar1 + 0x116)), 0x86);
            if (BVar2 != 0) {
                BVar2 =
                    write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (iVar1 + 0x19c)), 10);
                if (BVar2 != 0) {
                    BVar2 = write_to_file_1008_7e1c(
                        param_2,
                        (param_1 & 0xffff0000 | (iVar1 + 0x1a6)),
                        0x106,
                    );
                    if (BVar2 != 0) {
                        BVar2 = write_to_file_1008_7e1c(
                            param_2,
                            (param_1 & 0xffff0000 | (iVar1 + 0x2ac)),
                            0x106,
                        );
                        if (BVar2 != 0) {
                            return;
                        }
                    }
                }
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn read_file_1030_232e(param_1: u32, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let mut iVar1: i32;
    let BVar2: bool;

    pass1_1030_19b4(param_1, param_2);
    if (in_AX != 0) {
        iVar1 = param_1;
        BVar2 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar1 + 0x10)), 0x106);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar1 + 0x116)), 0x86);
            if (BVar2 != 0) {
                BVar2 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (iVar1 + 0x19c)), 10);
                if (BVar2 != 0) {
                    BVar2 = read_file_1008_7dee(
                        param_2,
                        (param_1 & 0xffff0000 | (iVar1 + 0x1a6)),
                        0x106,
                    );
                    if (BVar2 != 0) {
                        BVar2 = read_file_1008_7dee(
                            param_2,
                            (param_1 & 0xffff0000 | (iVar1 + 0x2ac)),
                            0x106,
                        );
                        if (BVar2 != 0) {
                            return;
                        }
                    }
                }
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1030_1a9c(param_1: *mut AStruct771, param_2: *mut HFILE16) -> bool {
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let BVar3: bool;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_c: u16;

    iVar2 = write_to_file_1030_1978(param_1, param_2);
    if (iVar2 != 0) {
        uVar4 = (param_1 >> 0x10);
        iVar2 = param_1;
        local_c = *(iVar2 + 0x10);
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
        if (BVar3 != 0) {
            if ((iVar2 + 0x10) == 0) {
                return 1;
            }
            piVar1 = (iVar2 + 0x10);
            let pi_var1_val = unsafe { *piVar1 };
            BVar3 = write_to_file_1008_7e1c(param_2, (piVar1 + 2), (pi_var1_val * 2));
            if (BVar3 != 0) {
                return 1;
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn pass1_1030_1b18(param_1: u32, param_2: *mut HFILE16) -> u16 {
    let piVar1: *mut i32;
    let mut uVar2: u32;
    let mut uVar3: i32;
    let mut in_AX: i32;
    let mut uVar4: u16;
    let BVar5: bool;
    let mut uVar6: i32;
    let in_DX: *mut u16;
    let struct_a: *mut AStruct199;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut local_18: u16;

    pass1_1030_19b4(param_1, param_2);
    if (in_AX != 0) {
        if (__g_AStruct94_ptr_1 == 0) {
            struct_fn_1000_160a();
            g_u16_ptr_1050_5f2e = in_DX;
        } else {
        }
        uVar4 = alloc_mem_1000_1708();
        uVar9 = (param_1 >> 0x10);
        iVar7 = param_1;
        (iVar7 + 0x10) = uVar4;
        (iVar7 + 0x12) = g_u16_ptr_1050_5f2e;
        struct_a = (iVar7 + 0x12);
        BVar5 = read_file_1008_7dee(param_2, CONCAT22(struct_a, (iVar7 + 0x10)), 2);
        if (BVar5 != 0) {
            piVar1 = (iVar7 + 0x10);
            let pi_var1_val = unsafe { *piVar1 };
            if (pi_var1_val == 0) {
                return 1;
            }
            uVar3 = pi_var1_val * 2;
            uVar6 = uVar3;
            process_struct_1000_179c(uVar3, struct_a);
            uVar2 = (iVar7 + 0x10);
            uVar4 = (uVar2 >> 0x10);
            iVar8 = uVar2;
            (iVar8 + 2) = uVar6;
            (iVar8 + 4) = struct_a;
            uVar2 = (iVar7 + 0x10);
            BVar5 = read_file_1008_7dee(param_2, *(uVar2 + 2), uVar3);
            if (BVar5 != 0) {
                return 1;
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return 0;
}

pub fn write_to_file_1030_1978(param_1: *mut AStruct771, param_2: *mut HFILE16) -> i32 {
    let mut iVar1: i32;

    iVar1 = write_to_file_1030_16d6(param_1, param_2);
    if (iVar1 != 0) {
        iVar1 = write_to_file_1008_7954(param_2, (param_1 + 0xc));
        if (iVar1 == 0) {
            g_u16_1050_0310 = 0x6d0;
            return iVar1;
        }
        iVar1 = 1;
    }
    return iVar1;
}

pub fn pass1_1030_19b4(param_1: u32, param_2: u32) {
    let uVar1: u8;
    let extraout_var: u32;
    let mut uVar2: u32;

    uVar1 = pass1_1030_1730(param_1, param_2);
    if (CONCAT31(extraout_var, uVar1) != 0) {
        uVar2 = param_1 & 0xffff0000 | (param_1 + 0xc);
        read_file_1008_76e4(param_2, uVar2);
        if (uVar2 == 0) {
            g_u16_1050_0310 = 0x6d2;
            return;
        }
    }
    return;
}

pub fn write_to_file_1030_16d6(param_1: *mut AStruct771, param_2: *mut HFILE16) {
    let BVar1: bool;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut local_10: u32;
    let mut local_8: u32;

    uVar2 = (param_1 >> 0x10);
    local_10 = (param_1 + 4);
    BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_10), 4);
    if (BVar1 != 0) {
        local_8 = (param_1 + 8);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_8), 4);
        if (BVar1 != 0) {
            return;
        }
    }
    g_u16_1050_0310 = 0x6d0;
    return;
}

pub fn pass1_1030_1730(param_1: *mut AStruct933, param_2: *mut HFILE16) -> u8 {
    let BVar1: bool;

    BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 4)), 4);
    if (BVar1 != 0) {
        BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 8)), 4);
        if (BVar1 != 0) {
            return 0x1;
        }
    }
    g_u16_1050_0310 = 0x6d2;
    return '\0';
}

pub fn pass1_1028_e628(param_1: u32, param_2: u16, param_3: u16, param_3_00: i32, param_5: i32) {
    let pcVar1: *mut libc::c_char;
    let piVar2: *mut i32;
    let mut uVar3: u32;
    let mut cVar4: u8;
    let puVar5: *mut u32;
    let mut uVar6: u32;
    let paVar7: *mut AStruct844;
    let mut uVar8: u32;
    let lVar9: u32;
    let ppcVar10: fn();
    let puVar11: *mut u16;
    let uVar12: u8;
    let uVar13: u8;
    let uVar14: u8;
    let uVar15: u8;
    let uVar16: u8;
    let uVar17: u8;
    let paVar18: *mut AStruct493;
    let puVar19: *mut u16;
    let mut uVar20: u16;
    let BVar21: bool;
    let mut uVar22: i32;
    let puVar23: *mut u8;
    let mut uVar24: u32;
    let extraout_DX: *mut AStruct199;
    let mut iVar25: i32;
    let extraout_DX_00: *mut AStruct199;
    let extraout_DX_01: *mut AStruct199;
    let extraout_DX_02: *mut AStruct199;
    let mut extraout_DX_03: u16;
    let mut extraout_DX_04: u16;
    let mut extraout_DX_05: u16;
    let mut extraout_DX_06: u16;
    let mut extraout_DX_07: u16;
    let extraout_DX_08: *mut AStruct199;
    let mut extraout_DX_09: u16;
    let mut extraout_DX_10: u16;
    let paVar26: *mut AStruct199;
    let mut uVar27: u16;
    let mut unaff_SI: u16;
    let unaff_DI: *mut u8;
    let mut unaff_ES: u16;
    let mut uVar28: u16;
    let mut unaff_CS: u16;
    let mut uVar29: u16;
    let mut uVar30: u16;
    let mut unaff_SS: u16;
    let mut bVar31: bool;
    let mut bVar32: bool;
    let ppVar33: *mut pass1_struct_1;
    let puVar34: *mut u16;
    let paVar35: *mut AStruct763;
    let mut local_154: u16;
    let mut local_152: u16;
    let mut local_14c: u16;
    let mut local_14a: u16;
    let mut uStack80: u16;
    let mut uStack78: u16;
    let uStack76: u8;
    let uStack75: u8;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut uVar36: u16;
    let mut local_3a: u16;
    let mut local_38: u32;
    let mut uStack52: u16;
    let mut uStack50: u16;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u32;
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

    uVar36 = SUB42(&g_alloc_addr_1050_1050, 0);
    local_42._0_1_ = SUB21(&local_6, 0);
    local_42._1_1_ = (&local_6 >> 8);
    local_46._0_1_ = param_2;
    uVar12 = local_46;
    local_46._1_1_ = (param_2 >> 8);
    uVar15 = local_46._1_1_;
    local_44 = param_3;
    local_4a._0_1_ = 0x3f;
    local_4a._1_1_ = 0xe6;
    local_48 = unaff_CS;
    local_3a = unaff_SI;
    local_38._0_2_ = unaff_DI;
    BVar21 = read_file_1008_7dee(CONCAT22(param_3, param_2), CONCAT22(unaff_SS, &local_6), 4);
    uVar29 = local_44;
    uVar16 = local_46._1_1_;
    uVar13 = local_46;
    if (BVar21 == 0) {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    local_a = 0;
    if (((param_3_00 == 0) && ((param_5 - 0x100) == '\0'))
        && (
            paVar26 = (param_5 - 0x100 >> 7),
            paVar26 < (&PTR_LOOP_1050_000e + 1),
        ))
    {
        uVar30 = &PTR_LOOP_1050_1028;
        local_46._0_1_ = param_1;
        uVar14 = local_46;
        local_46._1_1_ = (param_1 >> 8);
        uVar17 = local_46._1_1_;
        local_44 = (param_1 >> 0x10);
        uVar28 = local_44;
        uVar27 = param_1;
        match paVar26 {
            0x0 => {
                paVar7 = (uVar27 + 0xe);
                local_42._0_1_ = SUB41(paVar7, 0);
                local_42._1_1_ = (paVar7 >> 8);
                local_40 = (paVar7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x73;
                local_46._1_1_ = 0xe6;
                pass1_1030_145a(paVar7, local_6);
                local_1e._2_2_ = 0;
                local_1a = 0;
                while (CONCAT22(local_1a, local_1e._2_2_) < local_6) {
                    uVar29 = 0x1000;
                    local_40 = 0xe6d5;
                    uVar24 = local_6;
                    process_struct_1000_179c(0x14, paVar26);
                    local_20 = uVar24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        uVar22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        uVar29 = 0x1030;
                        local_42._0_1_ = 0xeb;
                        local_42._1_1_ = 0xe6;
                        uVar22 = local_20;
                        pass1_1030_5d0a();
                        local_40 = extraout_DX_04;
                    }
                    local_42._0_1_ = uVar22;
                    local_42._1_1_ = (uVar22 >> 8);
                    local_46._0_1_ = 0x97;
                    local_46._1_1_ = 0xe6;
                    ppcVar10 = (CONCAT22(local_40, uVar22) + 0x10);
                    local_44 = uVar29;
                    local_18 = uVar22;
                    local_16 = local_40;
                    (**ppcVar10)();
                    if (uVar22 == 0) {
                        return;
                    }
                    uVar24 = (local_18 + 4);
                    uVar22 = (local_18 + 6);
                    local_e = uVar24;
                    local_c = (uVar24 >> 0x10);
                    paVar26 = (uVar22 & 0xff);
                    local_40 = local_16;
                    local_42._0_1_ = local_18;
                    local_42._1_1_ = (local_18 >> 8);
                    uVar8 = (uVar27 + 0xe);
                    local_46._0_1_ = uVar8;
                    local_46._1_1_ = (uVar8 >> 8);
                    local_44 = (uVar8 >> 0x10);
                    local_4a._0_1_ = 0xc0;
                    local_4a._1_1_ = 0xe6;
                    local_48 = uVar29;
                    pass1_1030_14b4(
                        uVar8,
                        local_18,
                        local_16,
                        uVar24 & 0xffff | (uVar22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(local_1a, local_1e._2_2_) + 1;
                    local_1e._2_2_ = lVar9;
                    local_1a = (lVar9 >> 0x10);
                }
            }
            0x1 => {
                // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
            }
            0x2 => {
                paVar7 = (uVar27 + 0x12);
                local_42._0_1_ = SUB41(paVar7, 0);
                local_42._1_1_ = (paVar7 >> 8);
                local_40 = (paVar7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xf;
                local_46._1_1_ = 0xe7;
                pass1_1030_145a(paVar7, local_6);
                local_2a._2_2_ = 0;
                local_26 = 0;
                while (CONCAT22(local_26, local_2a._2_2_) < local_6) {
                    uVar29 = 0x1000;
                    local_40 = 0xe771;
                    uVar24 = local_6;
                    process_struct_1000_179c(0x1c, paVar26);
                    local_20 = uVar24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        uVar22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        uVar29 = 0x1030;
                        local_42._0_1_ = 0x87;
                        local_42._1_1_ = 0xe7;
                        uVar22 = local_20;
                        pass1_1030_2958((uVar24 & 0xffff | ZEXT24(paVar26) << 0x10));
                        local_40 = extraout_DX_05;
                    }
                    _local_24 = CONCAT22(local_40, uVar22);
                    local_42._0_1_ = uVar22;
                    local_42._1_1_ = (uVar22 >> 8);
                    local_46._0_1_ = 0x33;
                    local_46._1_1_ = 0xe7;
                    ppcVar10 = (*_local_24 + 0x10);
                    local_44 = uVar29;
                    (**ppcVar10)();
                    if (uVar22 == 0) {
                        return;
                    }
                    local_40 = (_local_24 >> 0x10);
                    uVar30 = _local_24;
                    uVar24 = (uVar30 + 4);
                    uVar22 = (uVar30 + 6);
                    local_e = uVar24;
                    local_c = (uVar24 >> 0x10);
                    paVar26 = (uVar22 & 0xff);
                    local_42._0_1_ = SUB41(_local_24, 0);
                    local_42._1_1_ = (_local_24 >> 8);
                    uVar8 = (uVar27 + 0x12);
                    local_46._0_1_ = uVar8;
                    local_46._1_1_ = (uVar8 >> 8);
                    local_44 = (uVar8 >> 0x10);
                    local_4a._0_1_ = 0x5c;
                    local_4a._1_1_ = 0xe7;
                    local_48 = uVar29;
                    pass1_1030_14b4(
                        uVar8,
                        uVar30,
                        local_40,
                        uVar24 & 0xffff | (uVar22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(local_26, local_2a._2_2_) + 1;
                    local_2a._2_2_ = lVar9;
                    local_26 = (lVar9 >> 0x10);
                }
            }
            0x3 => {
                local_42._0_1_ = 0;
                local_42._1_1_ = 0;
                local_40 = 0x500;
                uVar22 = uVar27 + 0x114;
                local_46._0_1_ = uVar22;
                uVar12 = local_46;
                local_46._1_1_ = (uVar22 >> 8);
                uVar13 = local_46._1_1_;
                local_48 = 0;
                uStack76 = _PTR_LOOP_1050_65e2;
                uStack75 = (_PTR_LOOP_1050_65e2 >> 8);
                local_4a._0_1_ = (_PTR_LOOP_1050_65e2 >> 0x10);
                local_4a._1_1_ = (_PTR_LOOP_1050_65e2 >> 0x18);
                uStack78 = SUB42(&PTR_LOOP_1050_1028, 0);
                uStack80 = 0x970b;
                local_16 = uVar22;
                pass1_1028_e2ac(_PTR_LOOP_1050_65e2);
                local_46._0_1_ = local_16;
                local_46._1_1_ = (local_16 >> 8);
                local_4a._0_1_ = _PTR_LOOP_1050_5740;
                local_4a._1_1_ = (_PTR_LOOP_1050_5740 >> 8);
                local_48 = (_PTR_LOOP_1050_5740 >> 0x10);
                uStack76 = 0x28;
                uStack75 = 0x10;
                uVar29 = 0x1030;
                uStack78 = 0x9728;
                local_44 = paVar26;
                local_42._0_1_ = uVar12;
                local_42._1_1_ = uVar13;
                local_40 = uVar28;
                local_14 = paVar26;
                pass1_1030_61fe(
                    _PTR_LOOP_1050_5740,
                    CONCAT22(paVar26, local_16),
                    param_1 & 0xffff0000 | uVar22,
                    (uVar27 + 0x108),
                );
                if (((uVar27 + 0x11a) == 10) || ((uVar27 + 0x11a) == 0x37)) {
                    if ((uVar27 + 0x11a) == 0x37) {
                        uVar3 = (uVar27 + 0x11e);
                        paVar26 = (uVar27 + 0x120);
                        local_38._0_2_ = uVar3;
                        local_38._2_2_ = (uVar3 >> 0x10);
                        uVar3 = (uVar27 + 0x10c);
                        local_2a._0_2_ = uVar3;
                        local_2a._2_2_ = (uVar3 >> 0x10);
                        (local_38 + 0x20) = uVar3;
                    }
                    local_42._0_1_ = 0;
                    local_42._1_1_ = 0;
                    local_40 = 0x400;
                    iVar25 = uVar27 + 0x114;
                    local_46._0_1_ = iVar25;
                    local_46._1_1_ = (iVar25 >> 8);
                    local_48 = 0;
                    uStack76 = _PTR_LOOP_1050_65e2;
                    uStack75 = (_PTR_LOOP_1050_65e2 >> 8);
                    local_4a._0_1_ = (_PTR_LOOP_1050_65e2 >> 0x10);
                    local_4a._1_1_ = (_PTR_LOOP_1050_65e2 >> 0x18);
                    uStack78 = 0x1030;
                    uStack80 = 0x9788;
                    local_44 = uVar28;
                    pass1_1028_e2ac(_PTR_LOOP_1050_65e2);
                    (uVar27 + 0x10c) = iVar25;
                    (uVar27 + 0x10e) = paVar26;
                    uVar20 = (uVar27 + 0x10c);
                    local_42._0_1_ = uVar20;
                    local_42._1_1_ = (uVar20 >> 8);
                    local_46._0_1_ = local_6;
                    local_46._1_1_ = (local_6 >> 8);
                    local_44 = (local_6 >> 0x10);
                    local_48 = 0x1030;
                    uVar29 = 0x1018;
                    local_4a._0_1_ = 0xaa;
                    local_4a._1_1_ = 0x97;
                    local_40 = paVar26;
                    pass1_1018_0196(local_6, CONCAT22(paVar26, uVar20), (uVar27 + 0x108));
                    paVar26 = extraout_DX_00;
                    if ((uVar27 + 0x11a) == 10) {
                        local_42._0_1_ = local_6;
                        local_42._1_1_ = (local_6 >> 8);
                        local_40 = (local_6 >> 0x10);
                        local_44 = 0x1018;
                        uVar29 = 0x1010;
                        local_46._0_1_ = 0xc4;
                        local_46._1_1_ = 0x97;
                        pass1_1010_ed22(local_6, (uVar27 + 0x10c));
                        paVar26 = extraout_DX_01;
                    }
                }
                uVar3 = (uVar27 + 0x10c);
                local_42._0_1_ = _PTR_LOOP_1050_65e2;
                local_42._1_1_ = (_PTR_LOOP_1050_65e2 >> 8);
                local_40 = (_PTR_LOOP_1050_65e2 >> 0x10);
                local_46._0_1_ = 0xd7;
                local_46._1_1_ = 0x97;
                local_44 = uVar29;
                paVar18 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, (uVar3 >> 0x10));
                (uVar27 + 0x110) = paVar18;
                (uVar27 + 0x112) = paVar26;
                if ((paVar26 | (uVar27 + 0x110)) != 0) {
                    local_40 = local_16;
                    local_42._0_1_ = 0;
                    local_42._1_1_ = 0;
                    uVar20 = (uVar27 + 0x110);
                    local_46._0_1_ = uVar20;
                    local_46._1_1_ = (uVar20 >> 8);
                    local_4a._0_1_ = 7;
                    local_4a._1_1_ = 0x98;
                    ppcVar10 = ((uVar27 + 0x110) + 8);
                    local_48 = uVar29;
                    local_44 = paVar26;
                    (**ppcVar10)();
                    paVar26 = extraout_DX_02;
                }
                local_42._0_1_ = _PTR_LOOP_1050_65e2;
                local_42._1_1_ = (_PTR_LOOP_1050_65e2 >> 8);
                local_40 = (_PTR_LOOP_1050_65e2 >> 0x10);
                local_46._0_1_ = 0x15;
                local_46._1_1_ = 0x98;
                local_44 = uVar29;
                local_1a = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, local_16, local_14);
                local_42._0_1_ = local_1a;
                local_42._1_1_ = (local_1a >> 8);
                local_46._0_1_ = 0x2b;
                local_46._1_1_ = 0x98;
                local_44 = uVar29;
                local_40 = paVar26;
                local_18 = paVar26;
                pass1_1030_73ee(CONCAT22(paVar26, local_1a), (uVar27 + 0x10c));
                local_42._0_1_ = _PTR_LOOP_1050_06e0;
                local_42._1_1_ = (_PTR_LOOP_1050_06e0 >> 8);
                local_40 = (_PTR_LOOP_1050_06e0 >> 0x10);
                local_44 = 0x1030;
                uVar29 = &PTR_LOOP_1050_1008;
                local_46._0_1_ = 0x3f;
                local_46._1_1_ = 0x98;
                local_20 = extraout_DX_03;
                BVar21 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar27 + 0x11a), 0x31);
                if ((BVar21 == 0) && ((uVar27 + 0x122) == 0)) {
                    uVar36 = (*(local_1a + 0xc) >> 0x10);
                    local_3a = (local_1a + 0x10);
                    local_38._0_2_ = &stack0xffc2;
                    if (local_3a < 1) {
                        uVar20 = 5;
                    } else {
                        uVar20 = 6;
                    }
                    (local_1a + 0x14) = uVar20;
                    local_20 = local_18;
                }
                uVar3 = (local_1a + 0x16);
                local_1e._0_2_ = uVar3;
                local_1e._2_2_ = (uVar3 >> 0x10);
                uVar8 = &PTR_LOOP_1050_65e2;
                local_42._0_1_ = uVar8;
                local_42._1_1_ = (uVar8 >> 8);
                local_40 = (uVar8 >> 0x10);
                local_44 = &PTR_LOOP_1050_1008;
                local_46._0_1_ = 0x9b;
                local_46._1_1_ = 0x98;
                paVar18 = pass1_1028_e1ec(uVar8, local_1e, local_1e._2_2_);
                _local_24 = CONCAT22(paVar18, local_24);
                if (CONCAT22(local_1e._2_2_, local_1e) != 0) {
                    local_42._0_1_ = SUB21(&local_14c, 0);
                    local_42._1_1_ = (&local_14c >> 8);
                    local_44 = &PTR_LOOP_1050_1008;
                    local_46._0_1_ = 0xb7;
                    local_46._1_1_ = 0x98;
                    pass1_1030_e4fa(
                        CONCAT22(unaff_SS, &local_14c),
                        CONCAT22(local_1e._2_2_, local_1e),
                    );
                    puVar5 = &g_bool_1050_5748;
                    local_42._0_1_ = SUB41(puVar5, 0);
                    local_42._1_1_ = (puVar5 >> 8);
                    local_40 = (puVar5 >> 0x10);
                    local_44 = 0x1030;
                    uVar29 = 0x1030;
                    local_46._0_1_ = 199;
                    local_46._1_1_ = 0x98;
                    pass1_1030_835a(puVar5, CONCAT22(unaff_SS, &local_14c));
                    local_14c = s_1_1050_389a;
                    local_14a = &PTR_LOOP_1050_1008;
                }
                uVar3 = (uVar27 + 0x11e);
                local_42._0_1_ = uVar3;
                local_42._1_1_ = (uVar3 >> 8);
                local_40 = (uVar3 >> 0x10);
                local_46._0_1_ = 0xec;
                local_46._1_1_ = 0x98;
                ppcVar10 = ((uVar27 + 0x11e) + 4);
                local_44 = uVar29;
                (**ppcVar10)();
                uVar3 = (uVar27 + 0x11e);
                uVar8 = (uVar3 + 4);
                local_46._0_1_ = uVar8;
                local_46._1_1_ = (uVar8 >> 8);
                local_44 = (uVar8 >> 0x10);
                local_4a._0_1_ = local_1a;
                local_4a._1_1_ = (local_1a >> 8);
                local_48 = local_18;
                uStack76 = uVar29;
                uStack75 = (uVar29 >> 8);
                uStack78 = 0x9902;
                pass1_1030_7e5a(CONCAT22(local_18, local_1a), uVar8);
                return;
            }
            0x4 => {
                paVar7 = (uVar27 + 0x16);
                local_42._0_1_ = SUB41(paVar7, 0);
                local_42._1_1_ = (paVar7 >> 8);
                local_40 = (paVar7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xa7;
                local_46._1_1_ = 0xe7;
                pass1_1030_145a(paVar7, local_6);
                local_2a._2_2_ = 0;
                local_26 = 0;
                while (CONCAT22(local_26, local_2a._2_2_) < local_6) {
                    uVar29 = 0x1000;
                    local_40 = 0xe828;
                    uVar24 = local_6;
                    process_struct_1000_179c(0x1e, paVar26);
                    local_20 = uVar24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        uVar22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        uVar29 = 0x1030;
                        local_42._0_1_ = 0x40;
                        local_42._1_1_ = 0xe8;
                        uVar22 = local_20;
                        pass1_1030_560e((uVar24 & 0xffff | ZEXT24(paVar26) << 0x10));
                        local_40 = extraout_DX_06;
                    }
                    _local_24 = CONCAT22(local_40, uVar22);
                    local_42._0_1_ = uVar22;
                    local_42._1_1_ = (uVar22 >> 8);
                    local_46._0_1_ = 0xcb;
                    local_46._1_1_ = 0xe7;
                    ppcVar10 = (*_local_24 + 0x10);
                    local_44 = uVar29;
                    (**ppcVar10)();
                    if (uVar22 == 0) {
                        return;
                    }
                    uVar36 = (_local_24 >> 0x10);
                    uVar8 = (_local_24 + 4);
                    local_e = uVar8;
                    local_c = (uVar8 >> 0x10);
                    uVar6 = (_local_24 + 0x10);
                    local_1e._2_2_ = uVar6;
                    local_1a = (uVar6 >> 0x10);
                    local_42._0_1_ = uVar6;
                    local_42._1_1_ = (uVar6 >> 8);
                    local_44 = 0;
                    local_48 = _PTR_LOOP_1050_5740;
                    local_46._0_1_ = (_PTR_LOOP_1050_5740 >> 0x10);
                    local_46._1_1_ = (_PTR_LOOP_1050_5740 >> 0x18);
                    local_4a._0_1_ = uVar29;
                    local_4a._1_1_ = (uVar29 >> 8);
                    uStack76 = 0xf8;
                    uStack75 = 0xe7;
                    local_40 = local_1a;
                    pass1_1030_6222(_PTR_LOOP_1050_5740, 0, uVar6, uVar8);
                    paVar26 = (local_c & 0xff);
                    local_42._0_1_ = SUB41(_local_24, 0);
                    local_42._1_1_ = (_local_24 >> 8);
                    local_40 = (_local_24 >> 0x10);
                    uVar8 = (uVar27 + 0x16);
                    local_46._0_1_ = uVar8;
                    local_46._1_1_ = (uVar8 >> 8);
                    local_44 = (uVar8 >> 0x10);
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0x13;
                    local_4a._1_1_ = 0xe8;
                    pass1_1030_14b4(
                        uVar8,
                        _local_24,
                        local_40,
                        CONCAT22(local_c, local_e) & 0xffffff,
                    );
                    lVar9 = CONCAT22(local_26, local_2a._2_2_) + 1;
                    local_2a._2_2_ = lVar9;
                    local_26 = (lVar9 >> 0x10);
                }
            }
            0x5 => {
                (paVar26).field_0x0 = "\x02";
                paVar26.field_0x2 = &PTR_LOOP_1050_1028;
                return;
            }
            0x6 => {
                paVar7 = (uVar27 + 0x1a);
                local_42._0_1_ = SUB41(paVar7, 0);
                local_42._1_1_ = (paVar7 >> 8);
                local_40 = (paVar7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x61;
                local_46._1_1_ = 0xe8;
                pass1_1030_145a(paVar7, local_6);
                local_30 = 0;
                while (local_30 < local_6) {
                    uVar29 = 0x1000;
                    local_40 = 0xe8c4;
                    uVar24 = local_6;
                    process_struct_1000_179c(0x21e, paVar26);
                    local_20 = uVar24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        uVar22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        uVar29 = &PTR_LOOP_1050_1038;
                        local_42._0_1_ = 0xda;
                        local_42._1_1_ = 0xe8;
                        uVar22 = local_20;
                        pass1_1038_30aa();
                        local_40 = extraout_DX_07;
                    }
                    local_42._0_1_ = uVar22;
                    local_42._1_1_ = (uVar22 >> 8);
                    local_46._0_1_ = 0x85;
                    local_46._1_1_ = 0xe8;
                    ppcVar10 = (CONCAT22(local_40, uVar22) + 0x10);
                    local_44 = uVar29;
                    local_2c = uVar22;
                    local_2a._0_2_ = local_40;
                    (**ppcVar10)();
                    if (uVar22 == 0) {
                        return;
                    }
                    uVar24 = (local_2c + 4);
                    uVar22 = (local_2c + 6);
                    local_e = uVar24;
                    local_c = (uVar24 >> 0x10);
                    paVar26 = (uVar22 & 0xff);
                    local_40 = local_2a;
                    local_42._0_1_ = local_2c;
                    local_42._1_1_ = (local_2c >> 8);
                    uVar8 = (uVar27 + 0x1a);
                    local_46._0_1_ = uVar8;
                    local_46._1_1_ = (uVar8 >> 8);
                    local_44 = (uVar8 >> 0x10);
                    local_4a._0_1_ = 0xae;
                    local_4a._1_1_ = 0xe8;
                    local_48 = uVar29;
                    pass1_1030_14b4(
                        uVar8,
                        local_2c,
                        local_2a,
                        uVar24 & 0xffff | (uVar22 & 0xff) << 0x10,
                    );
                    local_30 = local_30 + 1;
                }
            }
            // default:
            _ => {
                paVar7 = (uVar27 + 0x1e);
                local_42._0_1_ = SUB41(paVar7, 0);
                local_42._1_1_ = (paVar7 >> 8);
                local_40 = (paVar7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xf9;
                local_46._1_1_ = 0xe8;
                pass1_1030_145a(paVar7, local_6);
                local_42._0_1_ = _PTR_LOOP_1050_5740;
                local_42._1_1_ = (_PTR_LOOP_1050_5740 >> 8);
                local_40 = (_PTR_LOOP_1050_5740 >> 0x10);
                local_44 = 0x1030;
                local_46._0_1_ = 7;
                local_46._1_1_ = 0xe9;
                pass1_1030_66de(_PTR_LOOP_1050_5740, local_6);
                local_30 = 0;
                while (true) {
                    if (local_6 <= local_30) {
                        local_40 = 0x1030;
                        local_42._0_1_ = 0xd5;
                        local_42._1_1_ = 0xe9;
                        ret_1030_154c();
                        local_40 = 0x1030;
                        local_42._0_1_ = 0xdf;
                        local_42._1_1_ = 0xe9;
                        pass1_1030_6740(_PTR_LOOP_1050_5740);
                        return;
                    }
                    local_14 = _PTR_LOOP_1050_5744;
                    local_12 = (_PTR_LOOP_1050_5744 >> 0x10);
                    local_40 = 0x1030;
                    uVar29 = 0x1000;
                    local_42._0_1_ = 0xaf;
                    local_42._1_1_ = 0xe9;
                    puVar23 = _PTR_LOOP_1050_5744;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_5744);
                    local_20 = puVar23;
                    local_1e._0_2_ = extraout_DX_08;
                    if ((extraout_DX_08 | local_20) == 0) {
                        uVar22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        uVar29 = 0x1030;
                        local_42._0_1_ = 0xc4;
                        local_42._1_1_ = 0xe9;
                        uVar22 = local_20;
                        pass1_1030_67cc((puVar23 & 0xffff | ZEXT24(extraout_DX_08) << 0x10));
                        local_40 = extraout_DX_09;
                    }
                    local_42._0_1_ = uVar22;
                    local_42._1_1_ = (uVar22 >> 8);
                    local_46._0_1_ = 0x2d;
                    local_46._1_1_ = 0xe9;
                    ppcVar10 = (CONCAT22(local_40, uVar22) + 0x10);
                    local_44 = uVar29;
                    local_2c = uVar22;
                    local_2a._0_2_ = local_40;
                    (**ppcVar10)();
                    if (uVar22 == 0) {
                        break;
                    }
                    uVar8 = (local_2c + 4);
                    local_e = uVar8;
                    local_c = (uVar8 >> 0x10);
                    uVar24 = (local_2c + 8);
                    local_2a._2_2_ = uVar24;
                    local_26 = (uVar24 >> 0x10);
                    uVar3 = (local_2c + 0xc);
                    local_38._2_2_ = uVar3;
                    uStack52 = (uVar3 >> 0x10);
                    uStack50 = (local_2c + 0x10);
                    uVar22 = &local_38 + 2;
                    _local_24 = (_local_24 & 0xffff0000 | uVar22);
                    local_42._0_1_ = uVar22;
                    local_42._1_1_ = (uVar22 >> 8);
                    local_46._0_1_ = uVar8;
                    local_46._1_1_ = (uVar8 >> 8);
                    local_44 = local_c;
                    local_4a._0_1_ = _PTR_LOOP_1050_5740;
                    local_4a._1_1_ = (_PTR_LOOP_1050_5740 >> 8);
                    local_48 = (_PTR_LOOP_1050_5740 >> 0x10);
                    uStack76 = uVar29;
                    uStack75 = (uVar29 >> 8);
                    uStack78 = 0xe977;
                    pass1_1030_671c(
                        _PTR_LOOP_1050_5740,
                        uVar8,
                        CONCAT22(unaff_SS, uVar22),
                        uVar24,
                    );
                    local_42._0_1_ = local_2c;
                    local_42._1_1_ = (local_2c >> 8);
                    local_40 = local_2a;
                    uVar8 = (uVar27 + 0x1e);
                    local_46._0_1_ = uVar8;
                    local_46._1_1_ = (uVar8 >> 8);
                    local_44 = (uVar8 >> 0x10);
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0x92;
                    local_4a._1_1_ = 0xe9;
                    pass1_1030_14b4(
                        uVar8,
                        local_2c,
                        local_2a,
                        CONCAT22(local_c, local_e) & 0xffffff,
                    );
                    local_30 = local_30 + 1;
                }
                return;
            }
            0x9 => {
                local_6 = local_6 & 0xffff;
                local_42._0_1_ = param_3;
                local_42._1_1_ = (param_3 >> 8);
                local_40 = param_3_00;
                local_44 = param_2;
                local_46._0_1_ = (param_1 >> 0x10);
                local_46._1_1_ = (param_1 >> 0x18);
                local_c = *(uVar27 + 0x2e);
                local_a = *(uVar27 + 0x30);
                local_4a._0_1_ = 0x28;
                local_4a._1_1_ = 0x10;
                uStack76 = 0x2d;
                uStack75 = 0xe3;
                local_48 = uVar27;
                (*local_c)();
                return;
            }
            0xa => {
                paVar7 = (uVar27 + 0x22);
                local_42._0_1_ = SUB41(paVar7, 0);
                local_42._1_1_ = (paVar7 >> 8);
                local_40 = (paVar7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xf3;
                local_46._1_1_ = 0xe9;
                pass1_1030_145a(paVar7, local_6);
                local_38._2_2_ = 0;
                uStack52 = 0;
                while (CONCAT22(uStack52, local_38._2_2_) < local_6) {
                    local_40 = 0xea55;
                    uVar24 = local_6;
                    process_struct_1000_179c(0xe, paVar26);
                    local_20 = uVar24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        iVar25 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        local_42._0_1_ = 0x6b;
                        local_42._1_1_ = 0xea;
                        puVar34 = pass1_1028_b204((uVar24 & 0xffff | ZEXT24(paVar26) << 0x10));
                        local_40 = (puVar34 >> 0x10);
                        iVar25 = puVar34;
                    }
                    local_30 = CONCAT22(local_40, iVar25);
                    local_42._0_1_ = iVar25;
                    local_42._1_1_ = (iVar25 >> 8);
                    local_44 = 0x1000;
                    local_46._0_1_ = 0x17;
                    local_46._1_1_ = 0xea;
                    ppcVar10 = (local_30 + 0x10);
                    (**ppcVar10)();
                    if (iVar25 == 0) {
                        return;
                    }
                    local_40 = (local_30 >> 0x10);
                    uVar29 = local_30;
                    uVar24 = (uVar29 + 4);
                    uVar22 = (uVar29 + 6);
                    local_e = uVar24;
                    local_c = (uVar24 >> 0x10);
                    paVar26 = (uVar22 & 0xff);
                    local_42._0_1_ = local_30;
                    local_42._1_1_ = (local_30 >> 8);
                    uVar8 = (uVar27 + 0x22);
                    local_46._0_1_ = uVar8;
                    local_46._1_1_ = (uVar8 >> 8);
                    local_44 = (uVar8 >> 0x10);
                    local_48 = 0x1000;
                    local_4a._0_1_ = 0x40;
                    local_4a._1_1_ = 0xea;
                    pass1_1030_14b4(
                        uVar8,
                        uVar29,
                        local_40,
                        uVar24 & 0xffff | (uVar22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(uStack52, local_38._2_2_) + 1;
                    local_38._2_2_ = lVar9;
                    uStack52 = (lVar9 >> 0x10);
                }
            }
            0xb => {
                if (paVar26 < (&PTR_LOOP_1050_000e + 1)) {
                    pcVar1 = (unaff_SI + 0x23);
                    pv_var1_val = unsafe { *pcVar1 };
                    cVar4 = pv_var1_val;
                    unsafe { *pcVar1 = *pcVar1 << 6 };
                    uVar36 = 0x2b;
                    piVar2 = (&paVar26.field_0x0 + unaff_SI);
                    let pi_var_2_val = unsafe { piVar2 };
                    unsafe {
                        *piVar2 = *piVar2 + (-0x6600 - ((cVar4 << 5) < '\0'));
                    }
                } else {
                    uVar36 = 0x7af0;
                    local_46._0_1_ = uVar13;
                    local_46._1_1_ = uVar16;
                    local_44 = uVar29;
                    pass1_1028_780c(local_3a, local_38, CONCAT22(uStack52, local_38._2_2_));
                    if (param_3_00 == 0) {}
                    // goto code_r0x10287b17;
                }
                local_42._0_1_ = 0;
                local_42._1_1_ = 4;
                local_40 = 0x1d;
                local_46._0_1_ = 0x2b;
                local_46._1_1_ = 0;
                local_44 = 1;
                local_4a._0_1_ = SUB41(_g_AStruct372_1050_0ed0, 0);
                local_4a._1_1_ = (_g_AStruct372_1050_0ed0 >> 8);
                local_48 = (_g_AStruct372_1050_0ed0 >> 0x10);
                uStack76 = 0x28;
                uStack75 = 0x10;
                uStack78 = 0x7b0a;
                ppVar33 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, 0x1002b);
                local_44 = (ppVar33 >> 0x10);
                local_48 = ppVar33;
                local_46._0_1_ = (ppVar33 >> 0x10);
                local_46._1_1_ = (ppVar33 >> 0x18);
                local_4a._0_1_ = 0x10;
                local_4a._1_1_ = 0x10;
                uVar30 = 0x1010;
                uStack76 = 0x17;
                uStack75 = 0x7b;
                pass1_1010_043a(
                    ppVar33,
                    CONCAT13(local_42._1_1_, CONCAT12(local_42, local_44)),
                    local_40,
                );
                paVar26 = extraout_DX;
                // code_r0x10287b17:
                local_42._0_1_ = 2;
                local_42._1_1_ = 0;
                local_40 = 0x400;
                local_46._0_1_ = _PTR_LOOP_1050_65e2;
                local_46._1_1_ = (_PTR_LOOP_1050_65e2 >> 8);
                local_44 = (_PTR_LOOP_1050_65e2 >> 0x10);
                local_4a._0_1_ = 0x27;
                local_4a._1_1_ = 0x7b;
                local_48 = uVar30;
                paVar18 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 2, 0x400);
                local_42._0_1_ = SUB21(paVar18, 0);
                local_42._1_1_ = (paVar18 >> 8);
                local_4a._0_1_ = 0x38;
                local_4a._1_1_ = 0x7b;
                local_48 = uVar30;
                local_46._0_1_ = uVar14;
                local_46._1_1_ = uVar17;
                local_44 = uVar28;
                local_40 = paVar26;
                pass1_1028_780c(uVar27, uVar28, CONCAT22(paVar26, paVar18));
                local_40 = 2;
                local_44 = _g_AStruct372_1050_0ed0;
                local_42._0_1_ = (_g_AStruct372_1050_0ed0 >> 0x10);
                local_42._1_1_ = (_g_AStruct372_1050_0ed0 >> 0x18);
                local_46._0_1_ = uVar30;
                local_46._1_1_ = (uVar30 >> 8);
                local_48 = 0x7b44;
                local_a = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(uVar36, 2));
                local_c = u16_1050_13ae;
                if (2 < u16_1050_13ae) {
                    local_40 = 0x2f;
                    local_44 = _g_AStruct372_1050_0ed0;
                    local_42._0_1_ = (_g_AStruct372_1050_0ed0 >> 0x10);
                    local_42._1_1_ = (_g_AStruct372_1050_0ed0 >> 0x18);
                    local_46._0_1_ = 0x10;
                    local_46._1_1_ = 0x10;
                    uVar29 = 0x1010;
                    local_48 = 0x7b63;
                    ppVar33 =
                        process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(uVar36, 0x2f));
                    local_48 = (ppVar33 >> 0x10);
                    local_4a._0_1_ = SUB41(ppVar33, 0);
                    local_4a._1_1_ = (ppVar33 >> 8);
                    local_46._0_1_ = 1;
                    local_46._1_1_ = 0;
                    while (CONCAT11(local_46._1_1_, local_46) < 9) {
                        if ((CONCAT11(local_4a._1_1_, local_4a)
                            + 0x34
                            + CONCAT11(local_46._1_1_, local_46) * 4)
                            == local_6)
                        {
                            local_40 = 100;
                            paVar26 = (&PTR_LOOP_1050_0000 + 1);
                            local_30 = CONCAT22(local_30._2_2_, 1);
                            local_42._0_1_ = 1;
                            local_42._1_1_ = 0;
                            uVar28 = &PTR_LOOP_1050_1008;
                            local_46._0_1_ = 0xd7;
                            local_46._1_1_ = 0x7b;
                            local_44 = uVar29;
                            pass1_fn_1008_612e();
                            puVar19 = (CONCAT11(local_46._1_1_, local_46) - 7);
                            if (puVar19 == 0x0) {
                                bVar32 = SBORROW2(paVar26, 0x32);
                                puVar11 = &paVar26[-5].field_0xa;
                                bVar31 = paVar26 == (s_New_failed_in_Op__Op_1050_0020 + 0x12);
                                // LAB_1028_7b74:
                                if ((!bVar31 && bVar32) == (puVar11 < 0)) {
                                    local_30 = local_30 & 0xffff0000;
                                }
                            } else {
                                puVar19 = (CONCAT11(local_46._1_1_, local_46) - 8);
                                if (puVar19 == 0x0) {
                                    bVar32 = SBORROW2(paVar26, 0x19);
                                    puVar11 = (&paVar26[-3].field_0xa + 1);
                                    bVar31 = puVar11 == 0x0;
                                    // goto LAB_1028_7b74;
                                }
                            }
                            local_1e._0_2_ = paVar26;
                            if (local_30 != 0) {
                                local_40 = CONCAT11(local_46._1_1_, local_46);
                                local_44 = &local_154;
                                local_42._0_1_ = unaff_SS;
                                local_42._1_1_ = (unaff_SS >> 8);
                                local_46._0_1_ = 8;
                                local_46._1_1_ = 0x10;
                                local_48 = 0x7bfc;
                                pass1_1028_90e6(
                                    CONCAT13(local_42._1_1_, CONCAT12(local_42, local_44)),
                                    local_40,
                                );
                                puVar19 = &local_154;
                                local_42._0_1_ = SUB21(puVar19, 0);
                                local_42._1_1_ = (puVar19 >> 8);
                                local_46._0_1_ = SUB41(_g_bool_1050_5748, 0);
                                local_46._1_1_ = (_g_bool_1050_5748 >> 8);
                                local_44 = (_g_bool_1050_5748 >> 0x10);
                                local_48 = &PTR_LOOP_1050_1008;
                                uVar28 = 0x1030;
                                local_4a._0_1_ = 0xc;
                                local_4a._1_1_ = 0x7c;
                                pass1_1030_835a(_g_bool_1050_5748, CONCAT22(unaff_SS, puVar19));
                                local_154 = s_1_1050_389a;
                                local_152 = &PTR_LOOP_1050_1008;
                            }
                            local_42._0_1_ = 0;
                            local_42._1_1_ = 0;
                            local_40 = 10;
                            uVar29 = &PTR_LOOP_1050_1008;
                            local_46._0_1_ = 0x23;
                            local_46._1_1_ = 0x7c;
                            local_44 = uVar28;
                            pass1_fn_1008_612e();
                            local_18 = puVar19;
                            if (CONCAT11(local_46._1_1_, local_46) == 7) {
                                local_40 = 7;
                                puVar19 = puVar19 + 0x37;
                                iVar25 = puVar19 >> 0xf;
                            } else {
                                if (CONCAT11(local_46._1_1_, local_46) != 8) {}
                                // goto LAB_1028_7ba0;
                                local_40 = 8;
                                puVar19 = puVar19 + 0x32;
                                iVar25 = (puVar19 >> 0xf) + (0xff9b < puVar19);
                            }
                            local_44 = CONCAT11(local_42._1_1_, local_42) + puVar19;
                            local_40 = local_40
                                + iVar25
                                + CARRY2(CONCAT11(local_42._1_1_, local_42), puVar19);
                            local_42._0_1_ = local_40;
                            local_42._1_1_ = (local_40 >> 8);
                            uVar28 = CONCAT11(local_4a._1_1_, local_4a);
                            local_46._0_1_ = local_48;
                            local_46._1_1_ = (local_48 >> 8);
                            local_4a._0_1_ = 8;
                            local_4a._1_1_ = 0x10;
                            uVar29 = 0x1010;
                            uStack76 = 0xa0;
                            uStack75 = 0x7b;
                            local_48 = uVar28;
                            pass1_1010_ebf8(
                                CONCAT13(local_46._1_1_, CONCAT12(local_46, uVar28)),
                                local_44,
                                local_40,
                                local_40,
                            );
                        }
                        // LAB_1028_7ba0:
                        iVar25 = CONCAT11(local_46._1_1_, local_46) + 1;
                        local_46._0_1_ = iVar25;
                        local_46._1_1_ = (iVar25 >> 8);
                    }
                }
                return;
            }
            0xc => {
                paVar7 = (uVar27 + 0x26);
                local_42._0_1_ = SUB41(paVar7, 0);
                local_42._1_1_ = (paVar7 >> 8);
                local_40 = (paVar7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x8b;
                local_46._1_1_ = 0xea;
                pass1_1030_145a(paVar7, local_6);
                local_38._2_2_ = 0;
                uStack52 = 0;
                while (CONCAT22(uStack52, local_38._2_2_) < local_6) {
                    local_42._0_1_ = SUB21(&local_30, 0);
                    local_42._1_1_ = (&local_30 >> 8);
                    local_44 = param_3;
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0xd;
                    local_4a._1_1_ = 0xeb;
                    local_46._0_1_ = uVar12;
                    local_46._1_1_ = uVar15;
                    uVar29 = read_file_1008_7dee(
                        CONCAT22(param_3, param_2),
                        CONCAT22(unaff_SS, &local_30),
                        2,
                    );
                    if (uVar29 == 0) {
                        g_u16_1050_0310 = 0x6d2;
                        return;
                    }
                    local_40 = param_2;
                    local_42._0_1_ = 8;
                    local_42._1_1_ = 0x10;
                    local_44 = 0xeaa2;
                    switch_statement_1008_73ea(param_2, param_3, local_30);
                    local_42._0_1_ = 8;
                    local_42._1_1_ = 0x10;
                    local_44 = 0xeaaf;
                    local_40 = uVar27;
                    local_2c = uVar29;
                    paVar35 = pass1_1030_0000(uVar27, uVar28, uVar29);
                    local_40 = (paVar35 >> 0x10);
                    uVar29 = paVar35;
                    local_42._0_1_ = SUB41(paVar35, 0);
                    local_42._1_1_ = (paVar35 >> 8);
                    local_44 = 0x1030;
                    local_46._0_1_ = 0xc5;
                    local_46._1_1_ = 0xea;
                    ppcVar10 = (paVar35 + 0x10);
                    local_2a._2_2_ = uVar29;
                    local_26 = local_40;
                    (**ppcVar10)();
                    if (uVar29 == 0) {
                        return;
                    }
                    uVar24 = (local_2a._2_2_ + 4);
                    local_e = uVar24;
                    local_c = (uVar24 >> 0x10);
                    local_40 = local_26;
                    local_42._0_1_ = local_2a._2_2_;
                    local_42._1_1_ = (local_2a._2_2_ >> 8);
                    uVar8 = (uVar27 + 0x26);
                    local_46._0_1_ = uVar8;
                    local_46._1_1_ = (uVar8 >> 8);
                    local_44 = (uVar8 >> 0x10);
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0xee;
                    local_4a._1_1_ = 0xea;
                    pass1_1030_14b4(
                        uVar8,
                        local_2a._2_2_,
                        local_26,
                        uVar24 & 0xffff | (*(local_2a._2_2_ + 6) & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(uStack52, local_38._2_2_) + 1;
                    local_38._2_2_ = lVar9;
                    uStack52 = (lVar9 >> 0x10);
                }
            }
            0xd => {
                local_a = ZEXT24(paVar26) << 0x10;
                uVar3 = &PTR_LOOP_1050_000c;
                local_10 = uVar3;
                local_e = (uVar3 >> 0x10);
                local_c = &PTR_LOOP_1050_0010;
                local_18 = &local_10;
                local_42._0_1_ = SUB21(&local_14, 0);
                local_42._1_1_ = (&local_14 >> 8);
                local_46._0_1_ = SUB21(&local_16, 0);
                local_46._1_1_ = (&local_16 >> 8);
                local_4a._0_1_ = SUB21(&local_10, 0);
                local_4a._1_1_ = (&local_10 >> 8);
                uStack76 = 0x28;
                uStack75 = 0x10;
                uStack78 = SUB42(s_523a_bmp_1050_2116 + 7, 0);
                pass1_1008_3eb4(
                    CONCAT22(unaff_SS, &local_10),
                    CONCAT22(unaff_SS, &local_16),
                    CONCAT22(unaff_SS, &local_14),
                    CONCAT22(unaff_SS, &local_12),
                );
                local_18 = local_14 - 1;
                local_42._0_1_ = SUB21(&local_10, 0);
                local_42._1_1_ = (&local_10 >> 8);
                local_48 = &PTR_LOOP_1050_1008;
                local_4a._0_1_ = 0x39;
                local_4a._1_1_ = 0x21;
                local_46._0_1_ = uVar14;
                local_46._1_1_ = uVar17;
                local_e = local_18;
                BVar21 = pass1_1028_21ba(uVar27, local_44, CONCAT22(unaff_SS, &local_10), local_6);
                if (BVar21 == 0) {
                    local_18 = local_14 + 1;
                    local_42._0_1_ = SUB21(&local_10, 0);
                    local_42._1_1_ = (&local_10 >> 8);
                    local_48 = &PTR_LOOP_1050_1008;
                    local_4a._0_1_ = 0x59;
                    local_4a._1_1_ = 0x21;
                    local_46._0_1_ = uVar14;
                    local_46._1_1_ = uVar17;
                    local_44 = uVar28;
                    local_e = local_18;
                    BVar21 =
                        pass1_1028_21ba(uVar27, uVar28, CONCAT22(unaff_SS, &local_10), local_6);
                    if (BVar21 == 0) {
                        local_e = local_14;
                        local_18 = local_12 - 1;
                        local_42._0_1_ = SUB21(&local_10, 0);
                        local_42._1_1_ = (&local_10 >> 8);
                        local_48 = &PTR_LOOP_1050_1008;
                        local_4a._0_1_ = 0x82;
                        local_4a._1_1_ = 0x21;
                        local_46._0_1_ = uVar14;
                        local_46._1_1_ = uVar17;
                        local_44 = uVar28;
                        local_10 = local_18;
                        BVar21 =
                            pass1_1028_21ba(uVar27, uVar28, CONCAT22(unaff_SS, &local_10), local_6);
                        if (BVar21 == 0) {
                            local_18 = local_12 + 1;
                            local_42._0_1_ = SUB21(&local_10, 0);
                            local_42._1_1_ = (&local_10 >> 8);
                            local_48 = &PTR_LOOP_1050_1008;
                            local_4a._0_1_ = 0xa2;
                            local_4a._1_1_ = 0x21;
                            local_46._0_1_ = uVar14;
                            local_46._1_1_ = uVar17;
                            local_44 = uVar28;
                            local_10 = local_18;
                            BVar21 = pass1_1028_21ba(
                                uVar27,
                                uVar28,
                                CONCAT22(unaff_SS, &local_10),
                                local_6,
                            );
                            if (BVar21 == 0) {
                                return;
                            }
                        }
                    }
                }
                local_42._0_1_ = SUB41(_PTR_LOOP_1050_5a64, 0);
                local_42._1_1_ = (_PTR_LOOP_1050_5a64 >> 8);
                local_40 = (_PTR_LOOP_1050_5a64 >> 0x10);
                local_44 = &PTR_LOOP_1050_1008;
                local_46._0_1_ = 0xb4;
                local_46._1_1_ = 0x21;
                pass1_1038_79b2(_PTR_LOOP_1050_5a64, local_a);
                return;
            }
            0xe => {
                paVar7 = (uVar27 + 0x2a);
                local_42._0_1_ = SUB41(paVar7, 0);
                local_42._1_1_ = (paVar7 >> 8);
                local_40 = (paVar7 >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x31;
                local_46._1_1_ = 0xeb;
                pass1_1030_145a(paVar7, local_6);
                local_38._2_2_ = 0;
                uStack52 = 0;
                while (CONCAT22(uStack52, local_38._2_2_) < local_6) {
                    uVar29 = 0x1000;
                    local_40 = 0xeb94;
                    uVar24 = local_6;
                    process_struct_1000_179c(0x3b2, paVar26);
                    local_20 = uVar24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        uVar22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        uVar29 = 0x1030;
                        local_42._0_1_ = 0xaa;
                        local_42._1_1_ = 0xeb;
                        uVar22 = local_20;
                        pass1_1030_2068((uVar24 & 0xffff | ZEXT24(paVar26) << 0x10));
                        local_40 = extraout_DX_10;
                    }
                    local_30 = CONCAT22(local_40, uVar22);
                    local_42._0_1_ = uVar22;
                    local_42._1_1_ = (uVar22 >> 8);
                    local_46._0_1_ = 0x55;
                    local_46._1_1_ = 0xeb;
                    ppcVar10 = (local_30 + 0x10);
                    local_44 = uVar29;
                    (**ppcVar10)();
                    if (uVar22 == 0) {
                        return;
                    }
                    local_40 = (local_30 >> 0x10);
                    uVar30 = local_30;
                    uVar24 = (uVar30 + 4);
                    uVar22 = (uVar30 + 6);
                    local_e = uVar24;
                    local_c = (uVar24 >> 0x10);
                    paVar26 = (uVar22 & 0xff);
                    local_42._0_1_ = local_30;
                    local_42._1_1_ = (local_30 >> 8);
                    uVar8 = (uVar27 + 0x2a);
                    local_46._0_1_ = uVar8;
                    local_46._1_1_ = (uVar8 >> 8);
                    local_44 = (uVar8 >> 0x10);
                    local_4a._0_1_ = 0x7e;
                    local_4a._1_1_ = 0xeb;
                    local_48 = uVar29;
                    pass1_1030_14b4(
                        uVar8,
                        uVar30,
                        local_40,
                        uVar24 & 0xffff | (uVar22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(uStack52, local_38._2_2_) + 1;
                    local_38._2_2_ = lVar9;
                    uStack52 = (lVar9 >> 0x10);
                }
            }
        }
        local_40 = 0x1030;
        local_42._0_1_ = 0xfb;
        local_42._1_1_ = 0xe6;
        ret_1030_154c();
    }
    return;
}

pub fn write_to_file_1028_e56c(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut HFILE16,
    param_2_00: u32,
) {
    let ppcVar1: fn();
    let p_uvar2: *mut u16;
    let BVar3: bool;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut unaff_SS: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_14),
        (&PTR_LOOP_1050_0000 + 1),
        param_2_00,
        (param_2_00 >> 0x10),
    );
    local_18 = 0;
    while (true) {
        puVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar2));
        _local_1c = CONCAT22(extraout_DX, puVar2);
        if ((extraout_DX | puVar2) == 0) {
            break;
        }
        local_18 = local_18 + 1;
    }
    local_2a = local_18;
    BVar3 = write_to_file_1008_7e1c(param_1_00, CONCAT22(unaff_SS, &local_2a), 4);
    if (BVar3 == 0) {
        g_u16_1050_0310 = 0x6d0;
    } else {
        local_c = local_8;
        local_a = local_6;
        if (local_4 != 0) {
            local_c = 1;
            local_a = 0;
        }
        while {
            puVar2 = &local_14;
            pass1_1028_e4ec(CONCAT22(unaff_SS, puVar2));
            _local_1c = CONCAT22(extraout_DX_00, puVar2);
            if ((extraout_DX_00 | puVar2) == 0) {
                return;
            }
            ppcVar1 = (*_local_1c + 0xc);
            (**ppcVar1)(&PTR_LOOP_1050_1008, puVar2, extraout_DX_00);
            local_2a = local_2a & 0xffff0000 | ZEXT24(puVar2);
            puVar2 != 0x0
        } {}
    }
    return;
}

pub fn write_to_file_1028_dce2(param_1: *mut u32, param_2: *mut HFILE16) {
    let ppcVar1: fn();
    let uVar2: u8;
    let BVar3: bool;
    let mut iVar4: i32;
    let puVar5: *mut u16;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let extraout_var_01: u32;
    let extraout_var_02: u32;
    let extraout_var_03: u32;
    let extraout_var_04: u32;
    let extraout_var_05: u32;
    let extraout_var_06: u32;
    let extraout_var_07: u32;
    let mut extraout_DX: i32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut local_2a: u16;
    let mut local_26: u32;
    let mut local_1e: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    uVar2 = write_to_file_1008_7cac(param_2, 10);
    if (CONCAT31(extraout_var, uVar2) != 0) {
        let param1_val = unsafe { *param_1 };
        local_26 = param1_val;
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_26), 4);
        if (BVar3 != 0) {
            uVar7 = (param_1 >> 0x10);
            uVar6 = param_1;
            local_1e = (uVar6 + 8);
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_1e), 2);
            if (BVar3 != 0) {
                ppcVar1 = (*_PTR_LOOP_1050_5166 + 0xc);
                (**ppcVar1)(
                    &PTR_LOOP_1050_1008,
                    _PTR_LOOP_1050_5166,
                    (_PTR_LOOP_1050_5166 >> 0x10),
                    param_2,
                );
                if (BVar3 != 0) {
                    uVar2 = write_to_file_1008_7cac(param_2, 0xc);
                    iVar4 = CONCAT31(extraout_var_00, uVar2);
                    if (iVar4 != 0) {
                        write_to_file_1028_e56c(uVar6, uVar7, param_2, 0x1000000);
                        if (iVar4 != 0) {
                            uVar2 = write_to_file_1008_7cac(param_2, 0xd);
                            iVar4 = CONCAT31(extraout_var_01, uVar2);
                            if (iVar4 != 0) {
                                write_to_file_1028_e56c(uVar6, uVar7, param_2, 0x2000000);
                                if (iVar4 != 0) {
                                    uVar2 = write_to_file_1008_7cac(param_2, 0xe);
                                    iVar4 = CONCAT31(extraout_var_02, uVar2);
                                    if (iVar4 != 0) {
                                        write_to_file_1028_e56c(uVar6, uVar7, param_2, 0x3000000);
                                        if (iVar4 != 0) {
                                            uVar2 = write_to_file_1008_7cac(param_2, 0xf);
                                            iVar4 = CONCAT31(extraout_var_03, uVar2);
                                            if (iVar4 != 0) {
                                                write_to_file_1028_e56c(
                                                    uVar6, uVar7, param_2, 0x4000000,
                                                );
                                                if (iVar4 != 0) {
                                                    uVar2 = write_to_file_1008_7cac(param_2, 0x10);
                                                    iVar4 = CONCAT31(extraout_var_04, uVar2);
                                                    if (iVar4 != 0) {
                                                        write_to_file_1028_e56c(
                                                            uVar6, uVar7, param_2, 0x5000000,
                                                        );
                                                        if (iVar4 != 0) {
                                                            uVar2 = write_to_file_1008_7cac(
                                                                param_2, 0x11,
                                                            );
                                                            iVar4 =
                                                                CONCAT31(extraout_var_05, uVar2);
                                                            if (iVar4 != 0) {
                                                                write_to_file_1028_e56c(
                                                                    uVar6, uVar7, param_2,
                                                                    0x6000000,
                                                                );
                                                                if (iVar4 != 0) {
                                                                    uVar2 = write_to_file_1008_7cac(
                                                                        param_2, 0x12,
                                                                    );
                                                                    iVar4 = CONCAT31(
                                                                        extraout_var_06,
                                                                        uVar2,
                                                                    );
                                                                    if (iVar4 != 0) {
                                                                        write_to_file_1028_e56c(
                                                                            uVar6, uVar7, param_2,
                                                                            0x7000000,
                                                                        );
                                                                        if (iVar4 != 0) {
                                                                            uVar2 = write_to_file_1008_7cac(param_2, 0x13);
                                                                            iVar4 = CONCAT31(
                                                                                extraout_var_07,
                                                                                uVar2,
                                                                            );
                                                                            if (iVar4 != 0) {
                                                                                write_to_file_1028_e56c(uVar6, uVar7, param_2, 0x8000000);
                                                                                if (iVar4 != 0) {
                                                                                    pass1_1028_dc52(
                                                                                                        CONCAT22(unaff_SS, &local_14),
                                                                                                    (&PTR_LOOP_1050_0000 + 1), 0,
                                                                                                    0x400);
                                                                                    while (true) {
                                                                                        puVar5 = &local_14;
                                                                                        pass1_1028_e4ec(CONCAT22(unaff_SS, puVar5));
                                                                                        _local_18 = CONCAT22(extraout_DX, puVar5);
                                                                                        if ((extraout_DX | puVar5) == 0) {
                                                                                            break;
                                                                                        }
                                                                                        if ((puVar5 + 0x100) != 0x8000002)
                                                                                        {
                                                                                            pass1_1038_3ba0(
                                                                                                                CONCAT22(extraout_DX, puVar5));
                                                                                        }
                                                                                    }
                                                                                    return;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return;
}

pub fn read_file_1028_def2(param_1: *mut u8, file_handle_1: *mut HFILE16) {
    let ppcVar1: fn();
    let uVar2: u8;
    let BVar3: bool;
    let mut iVar4: i32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let extraout_var_01: u32;
    let extraout_var_02: u32;
    let extraout_var_03: u32;
    let extraout_var_04: u32;
    let extraout_var_05: u32;
    let extraout_var_06: u32;
    let extraout_var_07: u32;
    let mut uVar5: u16;
    let mut uVar6: u16;

    uVar5 = (file_handle_1 >> 0x10);
    uVar2 = read_file_1008_7cfe(file_handle_1);
    if (CONCAT31(extraout_var, uVar2) != 0) {
        BVar3 = read_file_1008_7dee(file_handle_1, param_1, 4);
        if (BVar3 != 0) {
            BVar3 = read_file_1008_7dee(file_handle_1, (param_1 & 0xffff0000 | (param_1 + 8)), 2);
            if (BVar3 != 0) {
                uVar6 = file_handle_1;
                ppcVar1 = (*_PTR_LOOP_1050_5166 + 0x10);
                (**ppcVar1)(
                    &PTR_LOOP_1050_1008,
                    _PTR_LOOP_1050_5166,
                    (_PTR_LOOP_1050_5166 >> 0x10),
                    file_handle_1,
                );
                if (BVar3 != 0) {
                    uVar2 = read_file_1008_7cfe(file_handle_1);
                    iVar4 = CONCAT31(extraout_var_00, uVar2);
                    if (iVar4 != 0) {
                        pass1_1028_e628(param_1, uVar6, uVar5, 0, 0x100);
                        if (iVar4 != 0) {
                            uVar2 = read_file_1008_7cfe(file_handle_1);
                            iVar4 = CONCAT31(extraout_var_01, uVar2);
                            if (iVar4 != 0) {
                                pass1_1028_e628(param_1, uVar6, uVar5, 0, 0x200);
                                if (iVar4 != 0) {
                                    uVar2 = read_file_1008_7cfe(file_handle_1);
                                    iVar4 = CONCAT31(extraout_var_02, uVar2);
                                    if (iVar4 != 0) {
                                        pass1_1028_e628(param_1, uVar6, uVar5, 0, 0x300);
                                        if (iVar4 != 0) {
                                            uVar2 = read_file_1008_7cfe(file_handle_1);
                                            iVar4 = CONCAT31(extraout_var_03, uVar2);
                                            if (iVar4 != 0) {
                                                pass1_1028_e628(param_1, uVar6, uVar5, 0, 0x400);
                                                if (iVar4 != 0) {
                                                    uVar2 = read_file_1008_7cfe(file_handle_1);
                                                    iVar4 = CONCAT31(extraout_var_04, uVar2);
                                                    if (iVar4 != 0) {
                                                        pass1_1028_e628(
                                                            param_1, uVar6, uVar5, 0, 0x500,
                                                        );
                                                        if (iVar4 != 0) {
                                                            uVar2 =
                                                                read_file_1008_7cfe(file_handle_1);
                                                            iVar4 =
                                                                CONCAT31(extraout_var_05, uVar2);
                                                            if (iVar4 != 0) {
                                                                pass1_1028_e628(
                                                                    param_1, uVar6, uVar5, 0, 0x600,
                                                                );
                                                                if (iVar4 != 0) {
                                                                    uVar2 = read_file_1008_7cfe(
                                                                        file_handle_1,
                                                                    );
                                                                    iVar4 = CONCAT31(
                                                                        extraout_var_06,
                                                                        uVar2,
                                                                    );
                                                                    if (iVar4 != 0) {
                                                                        pass1_1028_e628(
                                                                            param_1, uVar6, uVar5,
                                                                            0, 0x700,
                                                                        );
                                                                        if (iVar4 != 0) {
                                                                            uVar2 =
                                                                                read_file_1008_7cfe(
                                                                                    file_handle_1,
                                                                                );
                                                                            iVar4 = CONCAT31(
                                                                                extraout_var_07,
                                                                                uVar2,
                                                                            );
                                                                            if (iVar4 != 0) {
                                                                                pass1_1028_e628(
                                                                                    param_1, uVar6,
                                                                                    uVar5, 0,
                                                                                    0x800,
                                                                                );
                                                                                if (iVar4 != 0) {
                                                                                    return;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return;
}

pub fn write_to_file_1028_d7a0(param_1: u16, param_2: u16, param_1_00: u32) -> i32 {
    let uVar1: u8;
    let extraout_AH: u8;
    let mut iVar2: i32;

    uVar1 = write_to_file_1008_7cac(param_1_00, 8);
    iVar2 = CONCAT11(extraout_AH, uVar1);
    if (iVar2 != 0) {
        iVar2 = 1;
    }
    return iVar2;
}

pub fn read_file_1028_d7ba(param_1: u16, param_2: u16, param_1_00: *mut HFILE16) -> i32 {
    let uVar1: u8;
    let extraout_AH: u8;

    uVar1 = read_file_1008_7cfe(param_1_00);
    if (CONCAT11(extraout_AH, uVar1) == 0) {
        g_u16_1050_0310 = 0x6d4;
        return CONCAT11(extraout_AH, uVar1);
    }
    return 1;
}

pub fn write_to_file_1028_b5ec(in_struct_1: *mut AStruct771, in_file_handle: *mut HFILE16) -> u16 {
    let mut u_var1: u32;
    let BVar2: bool;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    uVar5 = (in_struct_1 >> 0x10);
    iVar4 = in_struct_1;
    local_e = (iVar4 + 0xc);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_e), 2);
    if (BVar2 == 0) {
        g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    iVar3 = write_to_file_1030_16d6(in_struct_1, in_file_handle);
    if (iVar3 == 0) {
        return 0;
    }
    local_8 = (iVar4 + 0xc);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
    if (BVar2 == 0) {
        g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (iVar4 + 0xe);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
    if (BVar2 == 0) {
        g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (iVar4 + 0x10);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
    if (BVar2 == 0) {
        g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (iVar4 + 0x12);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
    if (BVar2 == 0) {
        g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (iVar4 + 0x18);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
    if (BVar2 == 0) {
        g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_8 = (iVar4 + 0x1a);
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
    if (BVar2 == 0) {
        g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    local_4 = (iVar4 + 0x12);
    if (local_4 == 6) {
        local_4 = (iVar4 + 0x18);
    }
    if (local_4 < 1) {
        return 1;
    }
    if (SBORROW2(local_4, 1)) {
        return 1;
    }
    if (local_4 == 3 || (local_4 - 2) < 1) {
        local_8 = (iVar4 + 0x14);
    } else {
        if (local_4 == 4) {
            if ((iVar4 + 0x14) == 0) {
                local_8 = 0;
                BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
                // goto joined_r0x1028b766;
            }
            uVar1 = (iVar4 + 0x14);
            local_8 = (uVar1 + 0x94);
        } else {
            if (local_4 != 5) {
                return 1;
            }
            uVar1 = (iVar4 + 0x14);
            local_8 = (uVar1 + 0xa4);
            BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
            if (BVar2 == 0) {
                g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            uVar1 = (iVar4 + 0x14);
            local_8 = (uVar1 + 0xa6);
            BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
            if (BVar2 == 0) {
                g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            uVar1 = (iVar4 + 0x14);
            local_8 = (uVar1 + 0xa8);
            BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
            if (BVar2 == 0) {
                g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            uVar1 = (iVar4 + 0x14);
            local_8 = (uVar1 + 0xaa);
            BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
            if (BVar2 == 0) {
                g_u16_1050_0310 = 0x6d0;
                return 0;
            }
            uVar1 = (iVar4 + 0x14);
            local_8 = (uVar1 + 0xac);
        }
    }
    BVar2 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_8), 2);
    // joined_r0x1028b766:
    if (BVar2 == 0) {
        g_u16_1050_0310 = 0x6d0;
        return 0;
    }
    return 1;
}

pub fn read_file_fn_1028_b81a(param_1: u32, in_file_1: *mut HFILE16) {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let uVar3: u8;
    let BVar4: bool;
    let mut uVar5: u16;
    let mut iVar6: i32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut uVar7: i32;
    let mut unaff_SS: u16;
    let mut uVar8: u32;
    let mut iVar9: i32;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut local_3a: u32;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_2a: u16;
    let mut local_26: [u8; 22];
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    iVar9 = param_1;
    uVar7 = (param_1 >> 0x10);
    uVar3 = pass1_1030_1730(param_1, in_file_1);
    if (CONCAT31(extraout_var, uVar3) == 0) {
        return;
    }
    BVar4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_SS, &local_4), 2);
    if (BVar4 == 0) {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    BVar4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_SS, &local_6), 2);
    if (BVar4 == 0) {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    BVar4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_SS, &local_8), 2);
    if (BVar4 == 0) {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    uVar10 = in_file_1;
    uVar11 = (in_file_1 >> 0x10);
    switch_statement_1008_73ea(uVar10, uVar11, local_4);
    (iVar9 + 0xc) = BVar4;
    switch_statement_1008_73ea(uVar10, uVar11, local_6);
    (iVar9 + 0xe) = BVar4;
    switch_statement_1008_73ea(uVar10, uVar11, local_8);
    (iVar9 + 0x10) = BVar4;
    uVar5 = extraout_DX;
    BVar4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_SS, &local_4), 2);
    if (BVar4 == 0) {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    BVar4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_SS, &local_6), 2);
    if (BVar4 == 0) {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    BVar4 = read_file_1008_7dee(in_file_1, (param_1 & 0xffff0000 | (iVar9 + 0x1a)), 2);
    if (BVar4 == 0) {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    (iVar9 + 0x12) = local_4;
    (iVar9 + 0x18) = local_6;
    local_a = (iVar9 + 0x12);
    if (local_a == 6) {
        local_a = (iVar9 + 0x18);
    }
    match (local_a) {
        1 | 2 | 3 => {
            uVar5 = iVar9 + 0x14;
            // LAB_1028_b968:
            BVar4 = read_file_1008_7dee(in_file_1, CONCAT22(uVar7, uVar5), 2);
        }
        4 => {
            uVar8 = pass1_1028_e0bc(_PTR_LOOP_1050_65e2, CONCAT22(local_3a, (iVar9 + 0xc)));
            local_e = (uVar8 >> 0x10);
            (iVar9 + 0x14) = uVar8;
            (iVar9 + 0x16) = local_e;
            if ((local_e | (iVar9 + 0x14)) != 0) {
                uVar5 = (iVar9 + 0x14) + 0x94;
                uVar7 = local_e;
                local_10 = uVar5;
                // goto LAB_1028_b968;
            }
            BVar4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_SS, local_26), 2);
        }
        5 => {
            iVar6 = iVar9;
            pass1_1028_e100(_PTR_LOOP_1050_65e2, (iVar9 + 0xc));
            (iVar9 + 0x14) = iVar6;
            (iVar9 + 0x16) = uVar5;
            local_10 = (iVar9 + 0x14) + 0xa4;
            local_e = uVar5;
            BVar4 = read_file_1008_7dee(in_file_1, CONCAT22(uVar5, local_10), 2);
            if (BVar4 == 0) {
                g_u16_1050_0310 = 0x6d2;
                return;
            }
            BVar4 = read_file_1008_7dee(in_file_1, CONCAT22(unaff_SS, &local_2a), 2);
            if (BVar4 == 0) {
                g_u16_1050_0310 = 0x6d2;
                return;
            }
            uVar1 = (iVar9 + 0x14);
            BVar4 = read_file_1008_7dee(in_file_1, (uVar1 & 0xffff0000 | (uVar1 + 0xa8)), 2);
            if (BVar4 == 0) {
                g_u16_1050_0310 = 0x6d2;
                return;
            }
            uVar1 = (iVar9 + 0x14);
            BVar4 = read_file_1008_7dee(in_file_1, (uVar1 & 0xffff0000 | (uVar1 + 0xaa)), 2);
            if (BVar4 == 0) {
                g_u16_1050_0310 = 0x6d2;
                return;
            }
            uVar1 = (iVar9 + 0x14);
            BVar4 = read_file_1008_7dee(in_file_1, (uVar1 & 0xffff0000 | (uVar1 + 0xac)), 2);
            if (BVar4 == 0) {
                g_u16_1050_0310 = 0x6d2;
                return;
            }
            set_param_3_with_switch_1008_72bc(uVar10, uVar11, local_2a);
            uVar2 = (iVar9 + 0x14);
            (uVar2 + 0xa6) = BVar4;
            return;
        }
        _ => {}
        // default:
        // goto switchD_1028_ba97_caseD_6;
        9 => {
            iVar6 = iVar9;
            pass1_1028_e100(_PTR_LOOP_1050_65e2, (iVar9 + 0xc));
            (iVar9 + 0x14) = iVar6;
            (iVar9 + 0x16) = uVar5;
            // goto switchD_1028_ba97_caseD_6;

            if (BVar4 == 0) {
                g_u16_1050_0310 = 0x6d2;
                return;
            }
        }
        6 => {
            // switchD_1028_ba97_caseD_6:
            return;
        }
    }
}

pub fn pass1_1028_b282(param_1: *mut AStruct771, param_2: *mut HFILE16) -> bool {
    let BVar1: bool;
    let mut unaff_SS: u16;
    let mut local_c: u16;

    BVar1 = write_to_file_1030_16d6(param_1, param_2);
    if (BVar1 != 0) {
        local_c = (param_1 + 0xc);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
        if (BVar1 == 0) {
            g_u16_1050_0310 = 0x6d0;
            return BVar1;
        }
        BVar1 = 1;
    }
    return BVar1;
}

pub fn pass1_1028_b2c8(param_1: u32, param_2: *mut HFILE16) -> bool {
    let uVar1: u8;
    let extraout_AH: u8;
    let BVar2: bool;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut local_4: u16;

    uVar1 = pass1_1030_1730(param_1, param_2);
    BVar2 = CONCAT11(extraout_AH, uVar1);
    if (BVar2 != 0) {
        BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2);
        if (BVar2 == 0) {
            g_u16_1050_0310 = 0x6d2;
            return BVar2;
        }
        uVar3 = set_param_3_with_switch_1008_72bc(param_2, (param_2 >> 0x10), local_4);
        (param_1 + 0xc) = uVar3;
        BVar2 = 1;
    }
    return BVar2;
}

pub fn read_from_file_1028_65e2(param_1: u32, param_2: *mut HFILE16) {
    let ppcVar1: fn();
    let mut in_AX: i32;
    let BVar2: bool;
    let mut uvar3: u16;
    let mut uVar4: i32;
    let puVar5: *mut u8;
    let mut extraout_DX: i32;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let mut local_26: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    read_file_fn_1028_b81a(param_1, param_2);
    if (in_AX != 0) {
        BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2);
        if (BVar2 != 0) {
            local_6 = 0;
            while (true) {
                if (local_4 <= local_6) {
                    return;
                }
                puVar5 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                uVar4 = puVar5;
                local_14 = puVar5 & 0xffff | extraout_DX << 0x10;
                if ((extraout_DX | uVar4) == 0) {
                    local_14 = 0;
                } else {
                    local_14 = s_1_1050_389a;
                    (uVar4 + 2) = &PTR_LOOP_1050_1008;
                    (uVar4 + 4) = 0;
                    (uVar4 + 6) = 0;
                    (uVar4 + 8) = 0;
                    (uVar4 + 10) = 0;
                    (uVar4 + 0xc) = 0;
                    local_14 = 0x56ce;
                    (uVar4 + 2) = 0x1018;
                }
                BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_10), 2);
                if (BVar2 == 0) {
                    break;
                }
                BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_c), 2);
                if (BVar2 == 0) {
                    break;
                }
                BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_16), 2);
                if (BVar2 == 0) {
                    break;
                }
                BVar2 = read_file_1008_7dee(param_2, (local_14 & 0xffff0000 | (local_14 + 10)), 2);
                if (BVar2 == 0) {
                    break;
                }
                BVar2 = read_file_1008_7dee(param_2, (local_14 & 0xffff0000 | (local_14 + 0xc)), 2);
                if (BVar2 == 0) {
                    break;
                }
                (local_14 + 4) = local_10;
                uVar3 = local_10;
                set_param_3_with_switch_1008_72bc(param_2, (param_2 >> 0x10), local_c);
                uVar6 = (local_14 >> 0x10);
                (local_14 + 6) = uVar3;
                (local_14 + 8) = local_16;
                ppcVar1 = ((param_1 + 0x20) + 8);
                (**ppcVar1)();
                local_6 = local_6 + 1;
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_to_file_1028_64d6(param_1: *mut AStruct771, param_2: *mut HFILE16) -> u16 {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let puVar3: *mut u16;
    let BVar4: bool;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    iVar2 = write_to_file_1028_b5ec(param_1, param_2);
    if (iVar2 != 0) {
        uVar5 = (param_1 >> 0x10);
        pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0x20));
        uVar1 = (param_1 + 0x20);
        local_1c = (uVar1 + 8);
        puVar3 = &local_1c;
        local_10 = local_1c;
        while (true) {
            BVar4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, puVar3), 2);
            if (BVar4 == 0) {
                break;
            }
            _local_e = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
            if (_local_e == 0) {
                return 1;
            }
            local_1e = (_local_e + 4);
            BVar4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_1e), 2);
            if (BVar4 == 0) {
                break;
            }
            local_20 = (_local_e + 6);
            BVar4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_20), 2);
            if (BVar4 == 0) {
                break;
            }
            local_22 = (_local_e + 8);
            BVar4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_22), 2);
            if (BVar4 == 0) {
                break;
            }
            local_24 = (_local_e + 10);
            BVar4 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_24), 2);
            if (BVar4 == 0) {
                break;
            }
            local_26 = (_local_e + 0xc);
            puVar3 = &local_26;
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn write_to_file_1028_5f82(param_1: *mut AStruct771, param_2: *mut HFILE16) -> bool {
    let BVar1: bool;
    let mut unaff_SS: u16;
    let mut local_c: u16;

    BVar1 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar1 != 0) {
        local_c = (param_1 + 0x20);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
        if (BVar1 == 0) {
            g_u16_1050_0310 = 0x6d0;
            return BVar1;
        }
        BVar1 = 1;
    }
    return BVar1;
}

pub fn read_from_file_1028_5fc8(param_1: u32, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;

    read_file_fn_1028_b81a(param_1, param_2);
    if ((in_AX != 0)
        && (
            BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x20)), 2),
            BVar1 == 0,
        ))
    {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub fn write_file_func_1028_4a1a(param_1: *mut AStruct771, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;

    write_to_file_1028_b5ec(param_1, param_2);
    if ((in_AX != 0)
        && (
            BVar1 = write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (param_1 + 0x20)), 10),
            BVar1 == 0,
        ))
    {
        g_u16_1050_0310 = 0x6d0;
        return;
    }
    return;
}

pub fn pass1_1028_4a5a(param_1: u32, in_file_1: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;

    read_file_fn_1028_b81a(param_1, in_file_1);
    if ((in_AX != 0)
        && (
            BVar1 = read_file_1008_7dee(in_file_1, (param_1 & 0xffff0000 | (param_1 + 0x20)), 10),
            BVar1 == 0,
        ))
    {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub fn write_to_file_1028_3d0e(in_struct_1: *mut AStruct771, in_file_handle: *mut HFILE16) {
    let mut in_AX: i32;
    let bool_res: bool;
    let local_struct_1: *mut AStruct771;
    let mut local_struct_1_hi: u16;
    let mut local_SS__1: u16;
    let local_10: *mut u8;
    let local_8: *mut u8;

    write_to_file_1028_b5ec(in_struct_1, in_file_handle);
    if (in_AX != 0) {
        local_struct_1_hi = (in_struct_1 >> 0x10);
        local_struct_1 = in_struct_1;
        local_10 = local_struct_1.field_0x20;
        bool_res = write_to_file_1008_7e1c(in_file_handle, CONCAT22(local_SS__1, &local_10), 4);
        if (bool_res != 0) {
            local_8 = local_struct_1.field_0x24;
            bool_res = write_to_file_1008_7e1c(in_file_handle, CONCAT22(local_SS__1, &local_8), 4);
            if (bool_res != 0) {
                write_to_file_1008_7a22(in_file_handle, local_struct_1.field_0x28);
                if (bool_res != 0) {
                    return;
                }
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub fn read_file_1028_3d92(in_struct: *mut AStruct772, in_file_handle: *mut HFILE16) {
    let mut in_AX: i32;
    let local_struct_1: *mut AStruct772;
    let BVar1: bool;
    let mut uVar2: u16;

    read_file_fn_1028_b81a(in_struct, in_file_handle);
    if (in_AX != 0) {
        local_struct_1 = in_struct;
        local_struct_1 = &local_struct_1.field_0x20;
        BVar1 = read_file_1008_7dee(
            in_file_handle,
            (in_struct & 0xffff0000 | ZEXT24(local_struct_1)),
            4,
        );
        if (BVar1 != 0) {
            BVar1 = read_file_1008_7dee(
                in_file_handle,
                (in_struct & 0xffff0000 | &local_struct_1.field_0x24),
                4,
            );
            if (BVar1 != 0) {
                uVar2 = read_file_1008_7ad4(in_file_handle, local_struct_1.field_0x28);
                if (uVar2 != 0) {
                    return;
                }
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn write_file_fn_1028_2418(param_1: u32, param_2: *mut HFILE16) -> bool {
    let mut u_var1: u32;
    let BVar2: bool;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let lVar4: u32;
    let mut local_1c: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    BVar2 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar2 != 0) {
        uVar3 = (param_1 >> 0x10);
        pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0x20));
        uVar1 = (param_1 + 0x20);
        local_1c = (uVar1 + 8);
        local_10 = local_1c;
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_1c), 2);
        if (BVar2 == 0) {
            g_u16_1050_0310 = 0x6d0;
            return BVar2;
        }
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
            if (lVar4 == 0) {
                break;
            }
            _local_e = lVar4;
            BVar2 = write_to_file_1038_75ca(lVar4, param_2);
            if (BVar2 == 0) {
                return BVar2;
            }
        }
        BVar2 = 1;
    }
    return BVar2;
}

pub fn read_file_fn_1028_24a2(param_1: u32, param_2: *mut HFILE16) -> bool {
    let ppcVar1: fn();
    let BVar2: bool;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let struct_a: *mut AStruct199;
    let extraout_DX: *mut AStruct199;
    let mut unaff_SS: u16;
    let mut uVar5: u32;
    let pHVar6: *mut HFILE16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar5 = read_file_fn_1028_b81a(param_1, param_2);
    struct_a = (uVar5 >> 0x10);
    if (uVar5 == 0) {
        return 0;
    }
    if (1 < PTR_LOOP_1050_0312) {
        BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2);
        if (BVar2 == 0) {
            g_u16_1050_0310 = 0x6d2;
            return 0;
        }
        local_6 = 0;
        while (local_6 < local_4) {
            uVar4 = local_4;
            pHVar6 = param_2;
            process_struct_1000_179c(0x2a, struct_a);
            if ((struct_a | uVar4) == 0) {
                uVar5 = 0;
            } else {
                uVar5 = pass1_1038_6520(CONCAT22(struct_a, uVar4));
            }
            iVar3 = read_from_file_1038_774e(uVar5, pHVar6);
            if (iVar3 == 0) {
                return 0;
            }
            ppcVar1 = ((param_1 + 0x20) + 8);
            (**ppcVar1)();
            local_6 = local_6 + 1;
            struct_a = extraout_DX;
        }
    }
    return 1;
}

pub fn file_write_fn_1028_1452(param_1: u32, param_2: *mut HFILE16) -> bool {
    let mut iVar1: i32;
    let BVar2: bool;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut local_c: u16;
    let mut local_6: u16;

    iVar1 = write_to_file_1028_b5ec(param_1, param_2);
    if (iVar1 != 0) {
        uVar3 = (param_1 >> 0x10);
        local_c = (param_1 + 0x22);
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
        if (BVar2 != 0) {
            local_6 = (param_1 + 0x20);
            BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_6), 2);
            if (BVar2 != 0) {
                local_6 = PTR_LOOP_1050_4fbc;
                BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_6), 2);
                if (BVar2 != 0) {
                    return 1;
                }
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn read_file_fn_1028_14d8(param_1: u32, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;
    let mut unaff_SS: u16;
    let mut local_4: u16;

    read_file_fn_1028_b81a(param_1, param_2);
    if (in_AX != 0) {
        BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x22)), 2);
        if ((BVar1 != 0)
            && (
                BVar1 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2),
                BVar1 != 0,
            ))
        {
            (param_1 + 0x20) = local_4;
            if (PTR_LOOP_1050_0312 < 2) {
                return;
            }
            BVar1 = read_file_1008_7dee(param_2, &PTR_LOOP_1050_4fbc, 2);
            if (BVar1 != 0) {
                return;
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn file_write_fn_1028_0234(param_1: *mut AStruct731, param_2: u32) -> bool {
    let mut u_var1: u32;
    let mut iVar2: i32;
    let BVar3: bool;
    let local_BX_28: *mut AStruct731;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut local_1a: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    iVar2 = write_to_file_1028_b5ec(param_1, param_2);
    if (iVar2 != 0) {
        uVar4 = (param_1 >> 0x10);
        local_BX_28 = param_1;
        local_1a = local_BX_28.field_0x20;
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_1a), 2);
        if (BVar3 != 0) {
            pass1_1008_5784(CONCAT22(unaff_SS, local_a), local_BX_28.field_0x22);
            uVar1 = local_BX_28.field_0x22;
            local_14 = (uVar1 + 8);
            local_10 = local_14;
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_14), 2);
            while (BVar3 != 0) {
                _local_e = pass1_1008_5b12(CONCAT22(unaff_SS, local_a));
                if (_local_e == 0) {
                    return 1;
                }
                local_14 = (_local_e + 4);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_14), 2);
                if (BVar3 == 0) {
                    break;
                }
                local_14 = (_local_e + 6);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_14), 2);
                if (BVar3 == 0) {
                    break;
                }
                local_14 = (_local_e + 8);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_14), 2);
                if (BVar3 == 0) {
                    break;
                }
                local_14 = (_local_e + 10);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_14), 2);
                if (BVar3 == 0) {
                    break;
                }
                local_14 = (_local_e + 0xc);
                BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_14), 2);
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn file_read_fn_1028_0374(param_1: u32, param_2: *mut HFILE16) {
    let ppcVar1: fn();
    let mut in_AX: i32;
    let BVar2: bool;
    let mut uvar3: u16;
    let local_AX_291: *mut AStruct732;
    let mut uVar4: i32;
    let mut extraout_DX: i32;
    let mut uVar6: u16;
    let mut unaff_SS: u16;
    let mut local_28: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let puVar5: *mut u8;

    read_file_fn_1028_b81a(param_1, param_2);
    if (in_AX != 0) {
        BVar2 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x20)), 2);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2);
            if (BVar2 != 0) {
                local_6 = 0;
                while (true) {
                    if (local_4 <= local_6) {
                        return;
                    }
                    puVar5 = _PTR_LOOP_1050_68a2;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                    uVar4 = puVar5;
                    local_14 = puVar5 & 0xffff | extraout_DX << 0x10;
                    if ((extraout_DX | uVar4) == 0) {
                        local_14 = 0;
                    } else {
                        local_14 = s_1_1050_389a;
                        (uVar4 + 2) = &PTR_LOOP_1050_1008;
                        (uVar4 + 4) = 0;
                        (uVar4 + 6) = 0;
                        (uVar4 + 8) = 0;
                        (uVar4 + 10) = 0;
                        (uVar4 + 0xc) = 0;
                        local_14 = 0x56ce;
                        (uVar4 + 2) = 0x1018;
                    }
                    BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_10), 2);
                    if (BVar2 == 0) {
                        break;
                    }
                    BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_c), 2);
                    if (BVar2 == 0) {
                        break;
                    }
                    BVar2 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_18), 2);
                    if (BVar2 == 0) {
                        break;
                    }
                    BVar2 =
                        read_file_1008_7dee(param_2, (local_14 & 0xffff0000 | (local_14 + 10)), 2);
                    if (BVar2 == 0) {
                        break;
                    }
                    BVar2 =
                        read_file_1008_7dee(param_2, (local_14 & 0xffff0000 | (local_14 + 0xc)), 2);
                    if (BVar2 == 0) {
                        break;
                    }
                    (local_14 + 4) = local_10;
                    uVar3 = local_10;
                    set_param_3_with_switch_1008_72bc(param_2, (param_2 >> 0x10), local_c);
                    uVar6 = (local_14 >> 0x10);
                    (local_14 + 6) = uVar3;
                    (local_14 + 8) = local_18;
                    ppcVar1 = ((param_1 + 0x22) + 8);
                    (**ppcVar1)();
                    local_6 = local_6 + 1;
                }
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn file_write_fn_1020_e94e(param_1: u32, param_2: *mut HFILE16) -> bool {
    let BVar1: bool;
    let mut unaff_SS: u16;
    let mut local_c: u16;

    BVar1 = write_to_file_1030_de7c(param_1, param_2);
    if (BVar1 != 0) {
        local_c = (param_1 + 0x24);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
        if (BVar1 == 0) {
            g_u16_1050_0310 = 0x6d0;
            return BVar1;
        }
        BVar1 = 1;
    }
    return BVar1;
}

pub fn file_read_fn_1020_e994(param_1: u32, param_2: *mut HFILE16) {
    let mut in_AX: i32;
    let BVar1: bool;

    read_from_file_1030_dec4(param_1, param_2);
    if ((in_AX != 0)
        && (
            BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x24)), 2),
            BVar1 == 0,
        ))
    {
        g_u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub fn file_write_fn_1020_e6a4(param_1: u32, param_2: *mut HFILE16) -> u16 {
    let mut iVar1: i32;
    let BVar2: bool;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut local_c: u16;
    let mut local_6: u16;

    iVar1 = write_to_file_1030_de7c(param_1, param_2);
    if (iVar1 != 0) {
        uVar3 = (param_1 >> 0x10);
        local_c = (param_1 + 0x24);
        BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_c), 2);
        if (BVar2 != 0) {
            local_6 = (param_1 + 0x26);
            BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_SS, &local_6), 2);
            if (BVar2 != 0) {
                return 1;
            }
        }
        g_u16_1050_0310 = 0x6d0;
    }
    return 0;
}

pub fn file_read_fn_1020_e70e(param_1: *mut u8, param_2: u32) {
    let mut in_AX: i32;
    let BVar1: bool;

    read_from_file_1030_dec4(param_1, param_2);
    if (in_AX != 0) {
        BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x24)), 2);
        if (BVar1 != 0) {
            BVar1 = read_file_1008_7dee(param_2, (param_1 & 0xffff0000 | (param_1 + 0x26)), 2);
            if (BVar1 != 0) {
                return;
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return;
}

pub fn call_write_to_file_1020_d3d4(param_1: *mut u8, in_file_handle: *mut HFILE16) -> bool {
    let BVar1: bool;
    let mut unaff_SS: u16;
    let mut local_c: u16;

    BVar1 = write_to_file_1028_b5ec(param_1, in_file_handle);
    if (BVar1 != 0) {
        local_c = (param_1 + 0x20);
        BVar1 = write_to_file_1008_7e1c(in_file_handle, CONCAT22(unaff_SS, &local_c), 2);
        if (BVar1 == 0) {
            g_u16_1050_0310 = 0x6d0;
            return BVar1;
        }
        BVar1 = 1;
    }
    return BVar1;
}

pub fn call_read_file_1020_d41a(param_1: *mut u8, param_2: *mut HFILE16) -> bool {
    let BVar1: bool;
    let mut unaff_SS: u16;
    let local_4: *mut u8;

    BVar1 = read_file_fn_1028_b81a(param_1, param_2);
    if (BVar1 != 0) {
        BVar1 = read_file_1008_7dee(param_2, CONCAT22(unaff_SS, &local_4), 2);
        if (BVar1 == 0) {
            g_u16_1050_0310 = 0x6d2;
            return BVar1;
        }
        *(param_1 + 0x20) = local_4;
        BVar1 = 1;
    }
    return BVar1;
}

pub fn call_write_to_file_1020_a644(param_1: u16, param_2: u16, param_1_00: u32) -> i32 {
    let uVar1: u8;
    let mut write_file_result: i32;
    let extraout_AH: u8;

    uVar1 = write_to_file_1008_7cac(param_1_00, 0xb);
    write_file_result = CONCAT11(extraout_AH, uVar1);
    if (write_file_result != 0) {
        write_file_result = 1;
    }
    return write_file_result;
}

pub fn call_read_file_1020_a65e(param_1: u32, in_file_handle: *mut HFILE16) -> u16 {
    let uVar1: u8;
    let extraout_AH: u8;
    let BVar2: bool;
    let mut local_SS__1: u16;
    let mut local_a: [u8; 2];
    let mut local_8: [u8; 2];
    let mut local_6: [u8; 2];
    let mut local_4: [u8; 2];

    uVar1 = read_file_1008_7cfe(in_file_handle);
    if (CONCAT11(extraout_AH, uVar1) != 0) {
        if (1 < PTR_LOOP_1050_0312) {
            // LAB_1020_a6dc:
            pass1_1020_b97e(param_1, (param_1 >> 0x10), 0);
            return 1;
        }
        BVar2 = read_file_1008_7dee(in_file_handle, CONCAT22(local_SS__1, local_4), 2);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(in_file_handle, CONCAT22(local_SS__1, local_8), 2);
            if (BVar2 != 0) {
                BVar2 = read_file_1008_7dee(in_file_handle, CONCAT22(local_SS__1, local_6), 2);
                if (BVar2 != 0) {
                    BVar2 = read_file_1008_7dee(in_file_handle, CONCAT22(local_SS__1, local_a), 2);
                    if (BVar2 != 0) {}
                    // goto LAB_1020_a6dc;
                }
            }
        }
        g_u16_1050_0310 = 0x6d2;
    }
    return 0;
}

pub fn pass1_1020_8a9c(param_1: *mut AStruct393) {
    let mut u_var1: u32;
    let mut uVar2: i32;
    let mut uvar3: u16;
    let mut extraout_DX: u16;
    let mut unaff_SI: u16;
    let mut unaff_SS: u16;
    let ppVar4: *mut pass1_struct_1;
    let paVar5: *mut AStruct104;
    let local_struct_1: *mut AStruct393;
    let local_struct_1_hi: *mut AStruct393;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: [u8; 30];
    let mut local_2a: [u8; 36];
    let mut local_6: u16;
    let mut local_4: u16;
    let local_struct_1_1: *mut AStruct393;

    local_struct_1 = param_1;
    local_struct_1_1 = local_struct_1;
    local_struct_1_hi = (param_1 >> 0x10);
    process_struct_1020_847a(param_1, 2);
    uVar2 = &local_struct_1.field_0x14 + 2;
    zero_list_1008_3e38((param_1 & 0xffff0000 | uVar2));
    _local_4c = param_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0x1c);
    zero_list_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0x1c)));
    &local_struct_1.field_0x22 = 0;
    param_1.field_0x0 = 0x8e92;
    local_struct_1.u16_0x2 = 0x1020;
    ppVar4 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_SI, 0x29));
    uVar3 = (ppVar4 >> 0x10);
    local_struct_1.field_0x22 = ppVar4;
    local_struct_1.field_0x24 = uVar3;
    pass1_1018_2678(
        local_struct_1.field_0x22,
        uVar3,
        param_1 & 0xffff0000 | uVar2,
    );
    paVar5 = pass1_1018_268e(&local_struct_1.field_0x22);
    local_4 = (paVar5 >> 0x10);
    uVar3 = paVar5;
    uVar1 = &local_struct_1.field_0x8;
    local_6 = uVar3;
    pass1_1020_8712(
        (param_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        uVar1,
        (uVar1 >> 0x10),
        paVar5,
        param_1 & 0xffff0000 | uVar2,
    );
    uVar1 = &local_struct_1.field_0x22;
    local_struct_1 = (uVar1 >> 0x10);
    pass1_1018_26c2(uVar1, local_struct_1, _local_4c);
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 2);
    process_struct_1008_48fe(
        CONCAT22(unaff_SS, local_2a),
        1,
        CONCAT22(extraout_DX, uVar3),
    );
    pass1_1008_3f92(CONCAT22(unaff_SS, local_48), CONCAT22(unaff_SS, local_2a));
    uVar1 = &local_struct_1_1.field_0x8;
    pass1_1020_8712(
        (param_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        (uVar1 + 8),
        (uVar1 >> 0x10),
        CONCAT22(unaff_SS, local_48),
        _local_4c,
    );
    process_struct_1008_41bc(CONCAT22(unaff_SS, local_48));
    close_file_1008_496c(local_2a);
    return;
}
