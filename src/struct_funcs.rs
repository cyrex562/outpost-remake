// WARNING: Variable defined which should be unmapped: local_4

pub fn set_struct_1000_0782() -> u16 {
    let in_ax: *mut AStruct81;
    let local_BX__1: *mut AStruct79;
    let mut unaff_BP: u16;
    let mut unaff_SI: u16;
    let mut in_stack_00000000: u16;
    let mut local_4: u16;

    local_BX__1.field_0xe = 0;
    local_BX__1.field_0x10 = (local_BX__1 + 1);
    local_BX__1.field_0x8 = 0x9a0;
    struct_fn_1000_07ac(unaff_SI, in_ax, unaff_BP, in_stack_00000000);
    return 1;
}

pub fn struct_fn_1000_07ac(a: u16, in_AStruct81: *mut AStruct81, param_3: u16, param_4: u16) {
    let mut in_ax: i32;
    let paVar1: *mut AStruct81;
    let mut iVar2: i32;
    let mut in_dx: i32;
    let local_BX__1: *mut AStruct80;
    let mut uVar3: i32;
    let mut local_a: u16;
    let mut local_8: u16;
    let temp_79f3d97cb82: *mut AStruct81;

    temp_79f3d97cb82 = local_BX__1.field_0x10;
    local_BX__1.field_0xe = temp_79f3d97cb82;
    uVar3 = local_BX__1 + (in_dx - temp_79f3d97cb82);
    iVar2 = temp_79f3d97cb82 + (uVar3 - uVar3 % in_ax);
    local_BX__1.field_0x10 = iVar2;
    while (temp_79f3d97cb82 < (iVar2 - in_ax)) {
        paVar1 = (&temp_79f3d97cb82.field_0x0 + in_ax);
        (temp_79f3d97cb82).field_0x0 = paVar1;
        temp_79f3d97cb82 = paVar1;
    }
    (temp_79f3d97cb82).field_0x0 = 0x0;
    return;
}

pub fn set_struct_1000_09ca() -> u32 {
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let mut in_ax: i32;
    let local_BX__1: *mut AStruct84;
    let local_BX_17: *mut AStruct83;
    let temp_5f585a33fd: *mut AStruct85;

    puVar1 = &local_BX__1.field_0x14;
    local_BX_17 = ((local_BX__1 + (in_ax - puVar1) + -6 & 0xfffc) + puVar1);
    (local_BX_17).field_0x0 = 1;
    local_BX__1.field_0xe = puVar1;
    local_BX_17.field_0x4 = local_BX_17;
    local_BX_17.field_0x2 = local_BX_17;
    local_BX__1.field_0x10 = local_BX_17;
    if ((local_BX__1.field_0xc & 7) == 2) {
        local_BX__1.field_0x12 = 8;
    } else {
        uVar2 = (local_BX__1).field_0x0;
        temp_5f585a33fd = (uVar2 + 0x18);
        // local_BX__1.field_0x12 =
        //     (&temp_5f585a33fd[-2].field_0x1 &
        //      ~-(temp_5f585a33fd + 1 < &BYTE_1050_0008)) +
        //     8;
    }
    local_BX_17[-1].field_0x4 = (local_BX_17 - puVar1);
    puVar1 = (local_BX_17 - puVar1) | 2;
    local_BX__1.field_0x18 = local_BX_17;
    local_BX__1.field_0x16 = local_BX_17.field_0x2;
    (local_BX_17.field_0x2 + 1) = puVar1;
    local_BX_17.field_0x2 = puVar1;
    local_BX__1.field_0x8 = 0xe08;
    return puVar1 & 0xfffc;
}

pub fn struct_fn_1000_115c(param_1: u16, param_2: u16) -> u16 {
    let paVar1: *mut AStruct127;
    let puVar2: *mut u32;
    let pbVar3: *mut u8;
    let local_AX__1: *mut AStruct118;
    let local_AX_12: *mut AStruct127;
    let paVar4: *mut AStruct127;
    let in_BX: *mut u32;
    let puVar5: *mut u32;
    let mut local_8: u16;
    let mut local_4: u16;

    unsafe {
        local_AX_12 = (*in_BX & 0x7ffc);
    }
    // paVar4 = ((((local_AX__1 + 1) & 0xfffc) - 8 &
    //                           ~-(((local_AX__1 + 1) & 0xfffc) < 8)) +
    //                          8);
    if (local_AX_12 < paVar4) {
        puVar5 = (local_AX_12 + in_BX);
        unsafe {
            if (((*puVar5 & 1) != 0) || (local_AX_12 + (*puVar5 & 0xfffc) < paVar4)) {
                return 0;
            }
            if (puVar5 == PTR_LOOP_1050_000e) {
                PTR_LOOP_1050_000e = puVar5[1];
            }
            (puVar5[2] + 2) = puVar5[1];
            (puVar5[1] + 4) = puVar5[2];
            local_4 = (local_AX_12 + ((*puVar5 & 0xfffc) - paVar4));
            if (local_4 < s_version__d__d_1050_0012._0_2_) {
                puVar2 = in_BX;
                *puVar2 = *puVar2 + (*puVar5 & 0xfffc);
                pbVar3 = (puVar5 + (*puVar5 & 0xfffc));
                *pbVar3 = *pbVar3 | 2;
                return 1;
            }
        }
    } else {
        local_4 = (local_AX_12 + -paVar4);
        if (local_4 < s_version__d__d_1050_0012._0_2_) {
            return 1;
        }
        unsafe {
            puVar5 = (local_AX_12 + in_BX);
            if ((*puVar5 & 1) == 0) {
                local_4 = local_4 + (*puVar5 & 0xfffc);
                if (puVar5 == PTR_LOOP_1050_000e) {
                    PTR_LOOP_1050_000e = puVar5[1];
                }
                (puVar5[2] + 2) = puVar5[1];
                (puVar5[1] + 4) = puVar5[2];
            }
        }
        if (PTR_DAT_0005_0000_1050_0004 < local_4) {
            PTR_DAT_0005_0000_1050_0004 = local_4;
        }
    }
    unsafe {
        *in_BX = *in_BX & 0x8003 | paVar4;
        (paVar4 + in_BX) = local_4 | 2;
        paVar4 = paVar4 + in_BX;
        *(paVar4 + 4) = PTR_LOOP_1050_0010;
        (paVar4 + 2) = (PTR_LOOP_1050_0010 + 2);
        ((PTR_LOOP_1050_0010 + 2) + 4) = paVar4;
        (PTR_LOOP_1050_0010 + 2) = paVar4;
        (paVar4 + local_4 + -2) = local_4;
        paVar1 = (paVar4 + local_4);
        *paVar1 = (AStruct127)(*paVar1 & 0xfd);
    }
    return 1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn struct_fn_1000_160a() -> *mut AStruct94 {
    let paVar1: *mut AStruct94;

    paVar1 = return_one_1000_2146();
    if (paVar1 == 0x0) {
        return paVar1;
    }
    if ((g_u16_ptr_1050_5f2e | _g_AStruct94_ptr_1) == 0) {
        WORD_1050_5f30 = 1;
        u16_1050_5f32 = 1;
        __g_AStruct94_ptr_1 = init_struct_1000_18ec(0);
        if (__g_AStruct94_ptr_1 != 0x0) {
            if (PTR_LOOP_1050_5f42 != 0x0) {
                process_struct_1000_1a54(
                    PTR_LOOP_1050_5f42,
                    __g_AStruct94_ptr_1,
                    (__g_AStruct94_ptr_1 >> 0x10),
                );
            }
            if (u16_1050_5f44 != 0xffff) {
                check_structs_1000_1afe(u16_1050_5f44, __g_AStruct94_ptr_1);
            }
        }
    }
    return_1000_214a();
    return _g_AStruct94_ptr_1;
}

pub fn process_struct_1000_179c(param_1: u16, struct_a: *mut AStruct199) {
    let u16_ptr_1: *mut u16;
    let mut u16_ptr_2: u16;
    let mut local_4: u16;

    u16_ptr_1 = _g_AStruct94_ptr_1;
    u16_ptr_2 = g_u16_ptr_1050_5f2e;
    if ((g_u16_ptr_1050_5f2e | _g_AStruct94_ptr_1) == 0) {
        u16_ptr_1 = struct_fn_1000_160a();
        u16_ptr_2 = struct_a;
    }
    alloc_mem_1000_1708(param_1, 0, 0, u16_ptr_1, u16_ptr_2);
    return;
}

pub fn init_struct_1000_18ec(param_1: *mut AStruct94) {
    init_struct_1000_1902(param_1, 0, 0, 0xc);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn init_struct_1000_1902(param_1: *mut AStruct94, param_2: u16, param_3: u16, param_4: u16) {
    let paVar1: *mut AStruct94;
    let uVar2: u8;
    let local_struct: *mut AStruct94;
    let mut uVar3: i32;
    let BVar4: bool;
    let extraout_var: u32;
    let mut iVar5: i32;
    let alloc_addr: *mut u8;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let paVar7: *mut AStruct94;
    let mut unaff_CS: u16;
    let mut uVar8: u32;
    let mut local_4: u16;

    if (((param_1 & 0x8000) != 0) && (_SHI_INVOKEERRORHANDLER1 != -0x6f70)) {
        param_1 = (param_1 | 1);
    }
    while {
        local_struct = (param_1 & 0xfffb | 0x1000);
        alloc_mem_1000_131c(local_struct, 0x100, 0);
        if ((alloc_addr | local_struct) != 0) {
            break;
        }
        paVar7 = local_struct;
        invoke_error_handler_1000_1e61(unaff_CS, 2, 0);
        paVar7 != 0x0
    } {}
    if ((alloc_addr | local_struct) != 0) {
        local_struct.field_0x2e = &PTR_s__1050_1f7e_1050_5f1a;
        local_struct.field_0x30 = &g_alloc_addr_1050_1050;
        local_struct.AStruct94_ptr_0x2a = g_AStruct94_1050_5f1e;
        local_struct.alloc_addr = g_alloc_addr_1050_5f20;
        iVar5 = 5;
        paVar7 = local_struct;
        g_AStruct94_1050_5f1e = local_struct;
        g_alloc_addr_1050_5f20 = alloc_addr;
        while (iVar5 != 0) {
            iVar5 = iVar5 + -1;
            paVar1 = paVar7;
            paVar7 = &paVar7.field_0x2;
            paVar1.field_0x0 = 0;
        }
        local_struct.field_0xa = 0;
        local_struct.field_0xe = 0;
        local_struct.field_0xc = 0;
        local_struct.field_0x12 = 0;
        local_struct.field_0x10 = 0;
        local_struct.field_0x14 = 0xbead;
        uVar3 = param_1 & 0xfffd;
        local_struct.field_0x16 = uVar3;
        local_struct.field_0x18 = 0;
        local_struct.field_0x1a = 0x2000;
        local_struct.field_0x1c = 0x800;
        get_mem_sz_1000_1532(alloc_addr);
        local_struct.field_0x1e = uVar3;
        local_struct.field_0x20 = extraout_DX;
        local_struct.field_0x24 = 0;
        local_struct.field_0x22 = 0;
        local_struct.field_0x26 = 0xfffe;
        local_struct.field_0x28 = 0xffff;
        local_struct.field_0x32 = 0;
        local_struct.field_0x34 = 0;
        local_struct.field_0x40 = 0;
        local_struct.field_0x3e = 0;
        uVar6 = extraout_DX;
        BVar4 = check_structs_1000_1afe(param_4, CONCAT22(alloc_addr, local_struct));
        if (BVar4 != 0) {
            if ((param_3 | param_2) != 0) {
                uVar8 = CONCAT22(alloc_addr, local_struct);
                uVar2 = pass1_fn_1000_52be(param_2, param_3, param_4, 0);
                alloc_mem_1000_010c(1, CONCAT31(extraout_var, uVar2), uVar6, uVar8);
            }
            return;
        }
        free_mem_1000_1b9a(0, CONCAT22(alloc_addr, local_struct));
    }
    return;
}

pub fn process_struct_1000_1a54(param_1: u16, param_2: *mut AStruct94, param_3: u16) -> u32 {
    let mut uVar1: i32;
    let mut uVar2: i32;
    let mut unaff_SI: u16;
    let mut unaff_CS: u16;

    if (param_2.field_0x14 != 0xbead) {
        invoke_error_handler_1000_1e61(unaff_CS, 10, 0);
        return 0;
    }
    uVar1 = check_flag_1000_1ab0(unaff_SI);
    if (uVar1 < param_2.field_0x18 + 0x14) {
        uVar2 = 0;
    } else {
        uVar2 = param_2.field_0x1a;
        param_2.field_0x1a = uVar1;
        param_2.field_0x1c = uVar1 >> 2;
    }
    return uVar2;
}

pub fn check_structs_1000_1afe(param_1: *mut AStruct96, param_2: *mut AStruct95) -> bool {
    let paVar1: *mut AStruct96;
    let local_SI_22: *mut AStruct95;
    let mut uVar2: u16;
    let mut unaff_CS: u16;

    if (param_1 == 0x0) {
        paVar1 = 0x0;
    } else {
        paVar1 = ((param_1 + 1) & 0xfffe);
    }
    uVar2 = (param_2 >> 0x10);
    local_SI_22 = param_2;
    if (local_SI_22.field_0x14 == -0x4153) {
        if ((paVar1 < param_1) || ((local_SI_22.field_0x1a - 0x14) < paVar1)) {
            invoke_error_handler_1000_1e61(unaff_CS, 3, local_SI_22);
        } else {
            if (local_SI_22.field_0x2 == 0) {
                local_SI_22.field_0x18 = paVar1;
                return 1;
            }
        }
        return 0;
    }
    invoke_error_handler_1000_1e61(unaff_CS, 10, 0);
    return 0;
}

pub fn process_struct_1000_2cb0(param_1: *mut AStruct151) {
    let pbVar1: *mut u8;
    let mut bVar2: u8;

    bVar2 = param_1.field_0xa;
    if (((bVar2 & 0x83) != 0) && ((bVar2 & 8) != 0)) {
        error_check_1000_16ee(param_1.field_0x6, param_1.field_0x8);
        pbVar1 = &param_1.field_0xa;
        unsafe {
            *pbVar1 = *pbVar1 & 0xf7;
        }
        param_1.field_0x6 = 0;
        param_1.field_0x8 = 0;
        (param_1).field_0x0 = 0;
        param_1.field_0x2 = 0;
        param_1.field_0x4 = 0;
    }
    return;
}

pub fn process_struct_1000_2ce8(param_1: *mut AStruct152, param_2: u16) {
    let pbVar1: *mut u8;
    let pvVar2: *mut void;

    pvVar2 = alloc_mem_1000_167a(0x200, param_2);
    if (param_2 == 0) {
        pbVar1 = &param_1.field_0xa;
        unsafe {
            *pbVar1 = *pbVar1 | 4;
        }
        param_1.field_0xf2 = 1;
        param_2 = &g_alloc_addr_1050_1050;
        pvVar2 = &param_1.field_0xf1;
    } else {
        pbVar1 = &param_1.field_0xa;
        unsafe {
            *pbVar1 = *pbVar1 | 8;
        }
        param_1.field_0xf2 = 0x200;
    }
    param_1.field_0x2 = param_2;
    (param_1).field_0x0 = pvVar2;
    param_1.field_0x8 = param_2;
    param_1.field_0x6 = pvVar2;
    param_1.field_0x4 = 0;
    return;
}

pub fn process_struct_1000_2e74(param_1: *mut AStruct154) -> bool {
    let pbVar1: *mut u8;
    let pvVar2: *mut void;
    let pvVar3: *mut void;
    void * *ppvVar4;
    let pbVar5: *mut u8;

    if (PTR_LOOP_1050_61ec != 0x0) {
        pbVar5 = &param_1.field_0xf0;
        ppvVar4 = 0x5ff2;
        if ((param_1 == 0x621c) || (ppvVar4 = &PTR_LOOP_1050_5ff6, param_1 == 0x6228)) {
            let pb_var5_val = unsafe { *pbVar5 };
            if (((param_1.field_0xa & 0xc) == 0) && ((pb_var5_val & 1) == 0)) {
                pvVar2 = *ppvVar4;
                pvVar3 = ppvVar4[1];
                if ((pvVar2 | pvVar3) == 0) {
                    pvVar2 = alloc_mem_1000_167a(0x200, pvVar3);
                    if (pvVar3 == 0) {
                        return 0;
                    }
                    *ppvVar4 = pvVar2;
                    ppvVar4[1] = pvVar3;
                }
                param_1.field_0x6 = pvVar2;
                param_1.field_0x8 = pvVar3;
                (param_1).field_0x0 = pvVar2;
                param_1.field_0x2 = pvVar3;
                param_1.field_0x4 = 0x200;
                param_1.field_0xf2 = 0x200;
                pbVar1 = &param_1.field_0xa;
                unsafe {
                    *pbVar1 = *pbVar1 | 2;
                    *pbVar5 = 0x11;
                }
                return 1;
            }
        } else {
            if (u16_1050_5f8a <= param_1.field_0xb) {
                pbVar1 = pbVar5;
                unsafe {
                    *pbVar1 = *pbVar1 | 0x10;
                }
            }
        }
    }
    return 0;
}

pub fn process_struct_1000_2f00(param_1: u16, param_2: *mut AStruct155) {
    if (((param_2.field_0xf0 & 0x10) != 0) && ((*(param_2.field_0xb + 0x5f90) & 0x40) != 0)) {
        pass1_fn_1000_2fa4(param_2);
        if (param_1 != 0) {
            param_2.field_0xf0 = 0;
            param_2.field_0xf2 = 0;
            (param_2).field_0x0 = 0;
            param_2.field_0x2 = 0;
            param_2.field_0x6 = 0;
            param_2.field_0x8 = 0;
        }
    }
    return;
}

pub fn set_struct_1008_0000(param_1: *mut AStruct97) {
    let local_BX_4: *mut AStruct97;
    let mut in_stack_00000006: u16;

    // Segment:    2
    // Offset:     000060e0
    // Length:     efe0
    // Min Alloc:  efe0
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    *_param_1 = 0x52a;
    param_1.field_0x2 = &PTR_LOOP_1050_1008;
    param_1.field_0x4 = 0;
    param_1.field_0x8 = 0;
    *_param_1 = 0x51e;
    param_1.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_3ab8(in_AStruct180: *mut AStruct180) -> *mut AStruct180 {
    let local_BX_15: *mut AStruct180;
    let mut local_ES_15: u16;

    pass1_1008_687a(in_AStruct180, 0);
    local_ES_15 = (in_AStruct180 >> 0x10);
    local_BX_15 = in_AStruct180;
    local_BX_15.field_0xde = 0;
    in_AStruct180 = s_0_000_1050_3b46;
    local_BX_15.field_0x2 = &PTR_LOOP_1050_1008;
    copy_string_1000_3d3e(
        (in_AStruct180 & 0xffff0000 | &local_BX_15.field_0x5b),
        s_SOLDefaultWindowClass_1050_01fe,
    );
    return in_AStruct180;
}

pub fn set_struct_1008_4016(param_1: *mut AStruct102) {
    let local_BX_12: *mut AStruct102;
    let mut in_stack_00000006: u16;

    set_struct_1008_56b4(param_1);
    param_1.field_0x6 = 0;
    param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x10 = 0;
    param_1.field_0x14 = 0;
    param_1.field_0x18 = 0;
    param_1.field_0x1c = 0;
    *_param_1 = &PTR_LOOP_1050_48de;
    param_1.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_41bc(param_1: *mut AStruct182) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let local_AStruct182: *mut AStruct182;
    let mut local_ES_5: u16;
    let temp_862134eb919: *mut u32;
    let mut temp_5f500f05be: u32;
    // fn_ptr_1: *mut *mut u8;

    local_ES_5 = (param_1 >> 0x10);
    local_AStruct182 = param_1;
    param_1 = &PTR_LOOP_1050_48de;
    local_AStruct182.field_0x2 = &PTR_LOOP_1050_1008;
    puVar1 = local_AStruct182.field_0xa;
    uVar2 = local_AStruct182.field_0xc;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            fn_ptr_1 = *puVar1;
            (**fn_ptr_1)();
        }
    }
    if (local_AStruct182.field_0x6 != 0) {
        temp_5f500f05be = local_AStruct182.field_0x6;
        error_check_1000_0dc6(temp_5f500f05be, (temp_5f500f05be >> 0x10));
    }
    param_1 = s_1_1050_389a;
    local_AStruct182.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_4544(param_1: *mut AStruct104) {
    let mut bVar1: bool;
    let local_AStruct104: *mut AStruct186;
    let mut uVar2: i32;
    let mut local_8: u16;

    uVar2 = (param_1 >> 0x10);
    local_AStruct104 = param_1;
    if (local_AStruct104.field_0x6 == 0) {
        process_struct_1008_47cc((param_1 & 0xffff | uVar2 << 0x10));
    }
    if (local_AStruct104.field_0x6 == 0) {
        bVar1 = false;
    } else {
        if (local_AStruct104.field_0xa == 0) {
            process_struct_1008_4834((param_1 & 0xffff | uVar2 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return;
    }
    return;
}

pub fn process_struct_1008_4772(in_struct_104_ptr: *mut AStruct104) {
    let mut bVar1: bool;
    let local_BX_4: *mut AStruct104;
    let mut uVar2: i32;
    let mut local_4: u16;

    uVar2 = (in_struct_104_ptr >> 0x10);
    local_BX_4 = in_struct_104_ptr;
    if (&local_BX_4.a == 0) {
        process_struct_1008_47cc((in_struct_104_ptr & 0xffff | uVar2 << 0x10));
    }
    if (&local_BX_4.a == 0) {
        bVar1 = false;
    } else {
        if (&local_BX_4.field_0xa == 0) {
            process_struct_1008_4834((in_struct_104_ptr & 0xffff | uVar2 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return 0;
    }
    return CONCAT22(local_BX_4.c, local_BX_4.b);
}

pub fn process_struct_1008_47cc(in_AStruct104: *mut AStruct103) -> u8 {
    let mut u_var1: u32;
    let mut uVar2: u32;
    let local_AX_23: *mut AStruct106;
    let in_EAX: u32;
    let local_AStruct104: *mut AStruct104;
    let local_BX_53: *mut AStruct105;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut uStack14: u32;
    let temp_5f4c330c45: *mut AStruct107;

    uVar3 = (in_AStruct104 >> 0x10);
    local_AStruct104 = in_AStruct104;
    if (&local_AStruct104.a != 0) {
        uVar1 = &local_AStruct104.a;
        temp_5f4c330c45 = &local_AStruct104.field_0x8;
        local_AX_23 = uVar1;
        local_AX_23 = &local_AX_23.field_0xe;
        &local_AStruct104.b = uVar1 & 0xffff0000 | ZEXT24(local_AX_23);
        local_AStruct104.d = &local_AX_23.field_0x436;
        local_AStruct104.e = (temp_5f4c330c45 + (-(0xfbd7 < local_AX_23) & 0x6c));
        uVar2 = &local_AStruct104.b;
        uVar4 = (uVar2 >> 0x10);
        local_BX_53 = uVar2;
        uStack14 = local_BX_53.field_0xe;
        in_EAX = (uStack14 * local_BX_53.field_0x4 + 0x1f) / 0x20 << 2;
        local_AStruct104.f = in_EAX;
    }
    return in_EAX;
}

// WARNING: Could not reconcile some variable overlaps

pub fn process_struct_1008_4834(in_AStruct108: *mut AStruct103) -> u8 {
    let ppcVar1: fn();
    let local_AX_43: *mut u32;
    let mut uVar2: u32;
    let extraout_DX: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let mut extraout_DX_00: u16;
    let local_AStruct108: *mut AStruct108;
    let mut local_ES_4: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    local_ES_4 = (in_AStruct108 >> 0x10);
    local_AStruct108 = in_AStruct108;
    local_AX_43 = local_AStruct108.field_0xa;
    struct_a = local_AStruct108.field_0xc;
    if ((struct_a | local_AX_43) != 0) {
        unsafe {
            ppcVar1 = *local_AX_43;
        }
        (**ppcVar1)();
        struct_a = extraout_DX;
    }
    process_struct_1000_179c(0x14, struct_a);
    _local_a = CONCAT22(struct_a, local_AX_43);
    if ((struct_a | local_AX_43) != 0) {
        uVar2 = local_AStruct108.field_0x10;
        uVar2 = uVar2 & 0xffff0000 | (uVar2 + 0x28);
        process_struct_1008_4c98(_local_a, uVar2, 0x100);
        local_AStruct108.field_0xa = uVar2;
        local_AStruct108.field_0xc = extraout_DX_00;
        return uVar2;
    }
    &local_AStruct108.field_0xa = 0;
    return local_AX_43;
}

pub fn get_struct_field_1008_48aa(param_1: u32) -> u16 {
    return (param_1 + 0xe);
}

pub fn process_struct_1008_48b8(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    process_struct_1008_41bc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1008_48fe(param_1: *mut AStruct188, param_2: u16, param_3: u32) {
    let mut local_DX__1: u16;
    let local_BX_4: *mut AStruct188;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = 0;
    &local_BX_4.field_0x8 = 0;
    local_BX_4.field_0xc = 0xffff;
    local_BX_4.field_0xe = 0;
    local_BX_4.field_0x12 = 0;
    local_BX_4.field_0x16 = 0;
    local_BX_4.field_0x1a = 0;
    local_BX_4.field_0x1e = 0;
    local_BX_4.field_0x22 = param_2;
    param_1 = &PTR_LOOP_1050_4c4c;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    pass1_fn_1008_60e8(param_3);
    local_BX_4.field_0x8 = param_2;
    local_BX_4.field_0xa = local_DX__1;
    return;
}

pub fn process_struct_1008_4c58(param_1: *mut AStruct190) {
    let local_struct: *mut AStruct190;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_struct = param_1;
    param_1 = s_1_1050_389a;
    local_struct.field_0x2 = &PTR_LOOP_1050_1008;
    local_struct.field_0x4 = 0;
    local_struct.field_0xc = 0;
    local_struct.field_0xe = 0;
    local_struct.field_0x12 = 1;
    param_1 = 0x4f1c;
    local_struct.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_4c98(param_1: *mut AStruct191, param_2: u32, param_3: u16) {
    let local_BX_4: *mut AStruct191;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = param_2;
    local_BX_4.field_0xc = param_3;
    local_BX_4.field_0xe = 0;
    local_BX_4.field_0x12 = 0;
    param_1 = 0x4f1c;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_4ef6(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    pass1_1008_4cdc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1008_50c2(
    param_1: *mut AStruct195,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) {
    let local_BX_23: *mut AStruct195;
    let mut local_ES_23: u16;

    param_1.field_0x0 = param_4;
    local_ES_23 = (param_1 >> 0x10);
    local_BX_23 = param_1;
    local_BX_23.field_0x2 = (param_4 + 2);
    local_BX_23.field_0x4 = param_3;
    local_BX_23.field_0x8 = param_2;
    local_BX_23.field_0xc = param_5;
    local_BX_23.field_0x10 = 0;
    pass1_1008_52fc((param_1 & 0xffff | local_ES_23 << 0x10));
    return;
}

pub fn set_struct_1008_56b4(param_1: *mut AStruct103) -> *mut AStruct103 {
    let local_BX_4: *mut AStruct103;
    let mut in_stack_00000006: u16;

    _param_1.ptr_1_lo = s_1_1050_389a;
    param_1.ptr_1_hi = &PTR_LOOP_1050_1008;
    param_1.field_0x4 = 0;
    _param_1.ptr_1_lo = s__s__d_1050_573a;
    param_1.ptr_1_hi = &PTR_LOOP_1050_1008;
    return _param_1;
}

pub fn process_struct_1008_574a(param_1: *mut AStruct199) -> *mut AStruct199 {
    let local_BX_4: *mut AStruct199;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.field_0x0 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = 0;
    local_BX_4.field_0x8 = 0;
    local_BX_4.field_0xa = 1;
    param_1.field_0x0 = (s__s__s__1050_5bc0 + 4);
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    return local_BX_4;
}

pub fn process_struct_1008_8d8a(in_struct_2: *mut AStruct209, in_struct_1: u32, param_3: u32) {
    let mut uVar1: i32;
    let mut cVar2: u8;
    let mut uVar3: i32;
    let struct_a: *mut AStruct199;
    let paVar4: *mut AStruct199;
    let mut local_DX_187: u16;
    let struct_2: *mut AStruct209;
    let mut local_ES_4: u16;
    let mut uVar5: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_ES_4 = (in_struct_2 >> 0x10);
    struct_2 = in_struct_2;
    uVar1 = struct_2.field_0x2e;
    if (uVar1 < 0x28) {
        if ((uVar1 < 0x25) && (uVar1 != 0x23)) {
            if (0x23 < uVar1) {
                return;
            }
            cVar2 = uVar1;
            if (((cVar2 != 0xb) && (cVar2 != 0xf)) && (cVar2 != '!')) {
                return;
            }
        }
    } else {
        if (uVar1 < 0x46) {
            if (uVar1 < 0x43) {
                if (uVar1 < 0x33) {
                    return;
                }
                if ((uVar1 != 0x34 && 0 < (uVar1 - 0x33)) && (uVar1 != 0x37)) {
                    return;
                }
            }
        } else {
            if (uVar1 != 0x49) {
                if ((uVar1 - 0x49) < 0x2a) {
                    return;
                }
                if (5 < (uVar1 - 0x73)) {
                    return;
                }
            }
        }
    }
    if (&struct_2.field_0x3a == 0) {
        uVar5 = process_struct_1008_4772(in_struct_1);
        struct_a = (uVar5 >> 0x10);
        uVar1 = uVar5;
        paVar4 = struct_a;
        uVar3 = uVar1;
        process_struct_1000_179c(0x14, struct_a);
        _local_a = CONCAT22(paVar4, uVar3);
        if ((paVar4 | uVar3) == 0) {
            &struct_2.field_0x3a = 0;
        } else {
            uVar5 = in_struct_2 & 0xffff0000 | &struct_2.field_0x28;
            process_struct_1008_50c2(_local_a, (uVar1 + 8), (uVar1 + 4), uVar5, param_3);
            struct_2.field_0x3a = uVar5;
            &struct_2.field_0x3c = local_DX_187;
        }
        pass1_1008_5134(&struct_2.field_0x3a);
        return;
    }
    pass1_1008_5236(&struct_2.field_0x3a);
    return;
}

pub fn process_struct_1008_8e9e(in_struct_1: *mut AStruct210, param_2: u32, param_3: u32) {
    let struc_1: *mut AStruct210;
    let mut uVar1: u16;

    uVar1 = (in_struct_1 >> 0x10);
    struc_1 = in_struct_1;
    in_struct_1 = s_1_1050_389a;
    struc_1.field_0x2 = &PTR_LOOP_1050_1008;
    struc_1.field_0x4 = 0;
    struc_1.field_0x6 = 0;
    struc_1.field_0xa = 0;
    struc_1.field_0xe = param_3;
    struc_1.field_0x12 = 0;
    struc_1.field_0x16 = param_2;
    struc_1.field_0x1a = 1;
    in_struct_1 = 0x9170;
    struc_1.field_0x2 = &PTR_LOOP_1050_1008;
    if (struc_1.field_0xe < 7) {
        struc_1.field_0xe = 6;
    }
    alloc_mem_1008_909c(in_struct_1);
    struc_1.field_0x6 = 0;
    return;
}

pub fn modify_struct_1008_9174(param_1: *mut u16, param_2: u32, param_3: u32) {
    let local_BX_4: *mut AStruct214;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    unsafe {
        *param_1 = s_1_1050_389a;
    }
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = param_3;
    local_BX_4.field_0x8 = param_2;
    local_BX_4.field_0xc = param_2;
    local_BX_4.field_0x10 = 0;
    unsafe {
        *param_1 = 0x9412;
    }
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_9262(param_1: i32, param_2: u16, param_3: u32, param_4: u32) {
    let ppcVar1: fn();
    let mut in_ax: i32;
    let in_dx: *mut AStruct199;
    let lVar2: u32;
    let mut local_8: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x12, in_dx);
    if ((in_dx | in_ax) == 0) {
        lVar2 = 0;
    } else {
        lVar2 = modify_struct_1008_9174(CONCAT22(in_dx, in_ax), param_3, param_4);
    }
    if (lVar2 != 0) {
        ppcVar1 = ((param_1 + 6) + 4);
        (**ppcVar1)();
    }
    return;
}

pub fn set_struct_1008_9584(param_1: *mut AStruct216, param_2: u32) {
    let local_BX_4: *mut AStruct216;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = param_2;
    param_1 = 0x9d2e;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x8 = 0;
    local_BX_4.field_0xac = 0x2000000;
    local_BX_4.field_0xb0 = 0;
    local_BX_4.field_0xb4 = 0x8000;
    local_BX_4.field_0xb6 = 0x8000;
    local_BX_4.field_0xb8 = 0x8000;
    local_BX_4.field_0xba = 0x8000;
    local_BX_4.field_0xbc = 0;
    local_BX_4.field_0xbe = 0;
    local_BX_4.field_0xc2 = 0;
    local_BX_4.field_0xc4 = 0;
    local_BX_4.field_0xc6 = 0;
    local_BX_4.field_0xc8 = (s_572_bmp_1050_2007 + 1);
    local_BX_4.field_0xca = 0;
    param_1 = 0x380a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    *&local_BX_4.field_0x5b = 0;
    *&local_BX_4.field_0xa = 0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn process_struct_1008_9d36(param_1: *mut u8, param_2: *mut u8, param_3: u16) {
    let mut uVar1: u16;
    let puVar2: *mut u8;
    let mut extraout_DX: u16;
    let mut uvar3: u16;
    let mut uVar4: u32;
    let mut local_4: u16;

    pass1_1018_4cda(param_1, CONCAT22(param_3, param_2));
    (param_1 + 0x1c) = s_1_1050_389a;
    (param_1 + 0x1e) = &PTR_LOOP_1050_1008;
    (param_1 + 0x1c) = (s_18_2_1050_3aa5 + 3);
    (param_1 + 0x1e) = &PTR_LOOP_1050_1008;
    (param_1 + 0x20) = 0;
    zero_list_1008_3e38(CONCAT22(param_2, param_1 + 0x52));
    CONCAT22(param_2, param_1) = 0x9fb2;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    (param_1 + 0x1c) = 0x9fca;
    (param_1 + 0x1e) = &PTR_LOOP_1050_1008;
    PTR_LOOP_1050_4230 = param_1;
    PTR_LOOP_1050_4232 = param_2;
    pass1_1000_4906(CONCAT22(param_2, param_1 + 0x22), 0, 0x30);
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x1c0);
    local_4 = 0;
    while {
        uVar1 = local_4 + 0x1c0;
        mixed_fn_1010_830a(_g_struct_73_1050_14cc, uVar1);
        (param_1 + local_4 * 4 + 0x22) = uVar1;
        (param_1 + local_4 * 4 + 0x24) = extraout_DX;
        local_4 = local_4 + 1;
        local_4 < 0xc
    } {}
    uVar4 = process_struct_1008_4772((param_1 + 0x22));
    uVar3 = (uVar4 >> 0x10);
    pass1_1008_3e76(
        CONCAT22(param_2, param_1 + 0x52),
        0,
        (0x1e0 - (uVar4 + 8)) / 2 - 0x32,
        (0x280 - (uVar4 + 4)) / 2,
    );
    if (CONCAT22(param_2, param_1) == 0) {
        puVar2 = 0x0;
        param_2 = 0x0;
    } else {
        puVar2 = param_1 + 0x1c;
    }
    process_struct_1008_9262(
        _PTR_LOOP_1050_0388,
        (_PTR_LOOP_1050_0388 >> 0x10),
        0x50,
        CONCAT22(param_2, puVar2),
    );
    return;
}

pub fn process_struct_1008_9fd2(param_1: *mut AStruct217, param_2: u32) {
    let paVar1: *mut AStruct199;
    let paVar2: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let struct_a_00: *mut AStruct199;
    let mut local_DX_149: u16;
    let mut uvar3: u16;
    let mut local_4: u16;

    uVar3 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar3, param_1), (param_2 >> 0x10));
    paVar1 = 0x0;
    &param_1.field_0xa = 0;
    &param_1.field_0x410 = 0;
    param_1.field_0x414 = 0;
    param_1.field_0x416 = 0;
    param_1.field_0x418 = 0;
    param_1.field_0x41a = 0;
    param_1.field_0x41c = 0;
    param_1.field_0x41e = 0;
    CONCAT22(uVar3, param_1) = 0xad92;
    param_1.field_0x2 = &PTR_LOOP_1050_1008;
    paVar2 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    struct_a_00 = (paVar2 | paVar1);
    if (struct_a_00 == 0x0) {
        &param_1.field_0xa = 0;
    } else {
        paVar1 = process_struct_1008_574a(CONCAT22(paVar2, paVar1));
        param_1.field_0xa = paVar1;
        param_1.field_0xc = struct_a_00;
    }
    process_struct_1000_179c(0xc, struct_a_00);
    local_DX_149 = struct_a_00 | paVar1;
    if (local_DX_149 == 0) {
        paVar2 = 0x0;
        local_DX_149 = 0;
    } else {
        paVar2 = process_struct_1008_574a(CONCAT22(struct_a_00, paVar1));
    }
    param_1.field_0x410 = paVar2;
    param_1.field_0x412 = local_DX_149;
    return;
}

pub fn process_struct_1008_c882(param_1: u32) {
    let puVar1: *mut u16;
    let mut switch_var: u16;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let puVar4: *mut u32;
    let paVar5: *mut AStruct199;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let puVar8: *mut u8;
    let extraout_DX: *mut AStruct199;
    let mut uVar9: i32;
    let paVar10: *mut AStruct199;
    let mut uVar11: u16;
    let extraout_DX_00: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let paVar12: *mut AStruct199;
    let mut extraout_DX_01: u16;
    let mut iVar13: i32;
    let mut unaff_SI: u16;
    let mut uVar14: u16;
    let uVar15: u8;
    let ppVar16: *mut pass1_struct_1;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar14 = (param_1 >> 0x10);
    iVar13 = param_1;
    puVar4 = (iVar13 + 10);
    paVar5 = (iVar13 + 0xc);
    if ((paVar5 | puVar4) != 0) {
        unsafe {
            ppcVar2 = *puVar4;
        }
        ppcVar2();
        paVar5 = extraout_DX;
    }
    process_struct_1000_179c(0xc, paVar5);
    uVar9 = paVar5 | puVar4;
    if (uVar9 == 0) {
        paVar5 = 0x0;
        uVar9 = 0;
    } else {
        paVar5 = process_struct_1008_574a(CONCAT22(paVar5, puVar4));
    }
    (iVar13 + 10) = paVar5;
    (iVar13 + 0xc) = uVar9;
    ppVar16 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_SI, 0x35));
    paVar10 = (ppVar16 >> 0x10);
    paVar5 = paVar10;
    puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x44);
    local_10 = 0;
    struct_a = paVar5;
    while (true) {
        puVar1 = ((puVar8 & 0xffff) + 4);
        let pu_var1_val = unsafe { *puVar1 };
        if (pu_var1_val == local_10 || pu_var1_val < local_10) {
            break;
        }
        uVar3 = (puVar8 & 0xffff | ZEXT24(paVar5) << 0x10);
        switch_var = (uVar3 + local_10 * 2);
        if ((switch_var * 2 + ppVar16 + 10) != 0) {
            uVar7 = switch_var;
            big_switch_statement_1020_bd80(switch_var);
            pass1_fn_1008_60e8(uVar7, struct_a);
            uVar15 = 0;
            paVar12 = struct_a;
            uVar6 = uVar7;
            process_struct_1000_179c(0x14, struct_a);
            if ((paVar12 | uVar6) == 0) {
                uVar6 = 0;
                uVar11 = 0;
            } else {
                uVar15 = 0x18;
                pass1_1018_47c8(
                    CONCAT22(paVar12, uVar6),
                    1,
                    CONCAT13((ZEXT24(struct_a) >> 8), CONCAT12(struct_a, uVar7)),
                    switch_var,
                    0,
                );
                uVar11 = extraout_DX_01;
            }
            uVar3 = (iVar13 + 10);
            ppcVar2 = ((iVar13 + 10) + 4);
            ppcVar2(uVar15, uVar3, (uVar3 >> 0x10), uVar6, uVar11);
            struct_a = extraout_DX_00;
        }
        local_10 = local_10 + 1;
    }
    return;
}

pub fn process_struct_1008_cbc4(param_1: *mut AStruct296, param_2: *mut u8) {
    let mut u_var1: u32;
    let uVar2: u8;
    let mut uvar3: u16;
    let mut bVar4: bool;
    let struct_a_lo: *mut AStruct199;
    let paVar5: *mut AStruct199;
    let mut uVar6: i32;
    let local_AX_382: *mut AStruct514;
    let paVar7: *mut AStruct514;
    let struct_c: *mut AStruct199;
    let paVar8: *mut AStruct199;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: u16;
    let struct_a: *mut AStruct199;
    let local_DX_427: *mut u8;
    let puVar9: *mut u8;
    let struct_b: *mut AStruct296;
    let struct_b_hi: *mut AStruct296;
    let local_CS_316: u8;
    let local_CS_427: *mut u8;
    let local_SS__1: *mut AStruct514;
    let mut uVar10: u32;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut temp_5fe001fdd0: u32;
    let temp_5ff61a9dbc: *mut u8;
    let fn_ptr_a: fn();

    struct_b_hi = (param_1 >> 0x10);
    struct_b = param_1;
    struct_a_lo = struct_b.field_0x1e;
    paVar5 = struct_b.field_0x20;
    if ((paVar5 | struct_a_lo) != 0) {
        fn_ptr_a = struct_a_lo;
        (**fn_ptr_a)();
        paVar5 = struct_c;
    }
    process_struct_1000_179c(0xc, paVar5);
    paVar8 = (paVar5 | struct_a_lo);
    if (paVar8 == 0x0) {
        paVar5 = 0x0;
        paVar8 = 0x0;
    } else {
        paVar5 = process_struct_1008_574a(CONCAT22(paVar5, struct_a_lo));
    }
    struct_b.field_0x1e = paVar5;
    struct_b.field_0x20 = paVar8;
    local_6 = (param_2 + 0x200);
    pass1_1028_dc52(
        CONCAT13((local_SS__1 >> 8), CONCAT12(local_SS__1, &local_18)),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    local_1e = 0;
    while (true) {
        local_AX_382 = &local_18;
        pass1_1028_e4ec(CONCAT22(local_SS__1, local_AX_382));
        if ((extraout_DX | local_AX_382) == 0) {
            break;
        }
        if (local_AX_382.field_0x200 == local_6) {
            local_1e = local_1e + 1;
        }
    }
    bVar4 = false;
    if (1 < local_1e) {
        local_10 = local_c;
        local_e = local_a;
        if (local_8 != 0) {
            local_10 = 1;
            local_e = 0;
        }
        while (true) {
            local_AX_382 = &local_18;
            pass1_1028_e4ec(CONCAT22(local_SS__1, local_AX_382));
            if ((extraout_DX_00 | local_AX_382) == 0) {
                break;
            }
            if ((local_AX_382.field_0x200 == local_6) && (local_AX_382.field_0x4 != 0x4000001)) {
                uVar10 = pass1_1038_4d28(CONCAT22(extraout_DX_00, local_AX_382));
                paVar8 = (uVar10 >> 0x10);
                uVar6 = uVar10;
                pass1_fn_1008_60e8(uVar6, paVar8);
                uVar2 = uVar6;
                local_CS_316 = '\0';
                paVar5 = paVar8;
                process_struct_1000_179c(0x12, paVar8);
                if ((paVar5 | uVar6) == 0) {
                    uVar6 = 0;
                    uVar3 = 0;
                } else {
                    temp_5fe001fdd0 = local_AX_382.field_0x4;
                    local_CS_316 = '\x18';
                    pass1_1018_4920(
                        uVar6,
                        paVar5,
                        1,
                        0,
                        uVar2,
                        paVar8,
                        temp_5fe001fdd0,
                        (temp_5fe001fdd0 >> 0x10),
                    );
                    uVar3 = extraout_DX_01;
                }
                uVar1 = &struct_b.field_0x1e;
                fn_ptr_a = (&struct_b.field_0x1e + 4);
                (**fn_ptr_a)(local_CS_316, uVar1, (uVar1 >> 0x10), uVar6, uVar3);
                bVar4 = true;
            }
        }
    }
    if (!bVar4) {
        load_str_1010_84ac(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x43d,
        );
        _local_40 = CONCAT22(struct_a, local_AX_382);
        local_CS_427._0_1_ = 0;
        paVar5 = struct_a;
        paVar7 = local_AX_382;
        process_struct_1000_179c(0x12, struct_a);
        if ((paVar5 | paVar7) == 0) {
            paVar7 = 0x0;
            puVar9 = 0x0;
        } else {
            local_CS_427._0_1_ = 0x18;
            pass1_1018_4920(paVar7, paVar5, 0, 0, local_AX_382, struct_a, 0, 0);
            puVar9 = local_DX_427;
        }
        uVar1 = &struct_b.field_0x1e;
        fn_ptr_a = (&struct_b.field_0x1e + 4);
        (**fn_ptr_a)(
            local_CS_427._0_1_,
            uVar1,
            (uVar1 >> 0x10),
            paVar7,
            puVar9,
            _local_40,
            paVar7,
            puVar9,
        );
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_36
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn process_struct_1008_cda2(param_1: *mut AStruct298, param_2: u32) {
    let plVar1: *mut long;
    let mut uVar2: u32;
    let ppcVar3: fn();
    let puVar4: *mut u32;
    let paVar5: *mut AStruct199;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let puVar8: *mut u8;
    let mut uVar9: i32;
    let mut uVar10: u32;
    let extraout_DX: *mut AStruct199;
    let mut uVar11: i32;
    let paVar12: *mut AStruct199;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: i32;
    let struct_a: *mut AStruct199;
    let mut extraout_DX_03: u16;
    let local_struct_1: *mut AStruct297;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let uVar15: u8;
    let mut unaff_SS: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: [u8; 8];
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    uVar13 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    puVar4 = local_struct_1.field_0x1a;
    paVar5 = local_struct_1.field_0x1c;
    _local_e = CONCAT22(paVar5, puVar4);
    local_12 = puVar4;
    local_10 = paVar5;
    if ((paVar5 | puVar4) != 0) {
        unsafe {
            ppcVar3 = *puVar4;
            (**ppcVar3)();
        }
        paVar5 = extraout_DX;
    }
    process_struct_1000_179c(0xc, paVar5);
    uVar11 = paVar5 | puVar4;
    local_12 = puVar4;
    local_10 = paVar5;
    if (uVar11 == 0) {
        paVar5 = 0x0;
        uVar11 = 0;
    } else {
        paVar5 = process_struct_1008_574a(CONCAT22(paVar5, puVar4));
    }
    local_struct_1.field_0x1a = paVar5;
    local_struct_1.field_0x1c = uVar11;
    local_4 = 0;
    uVar14 = (param_2 >> 0x10);
    local_8 = (param_2 + 0x210);
    uVar11 = (param_2 + 0x212) | local_8;
    uVar10 = uVar11;
    if (uVar11 != 0) {
        local_1a = (local_8 + 10);
        local_1e = 0;
        while (uVar10 = local_1a, local_1e < local_1a) {
            pass1_1030_1312(local_8, (local_8 >> 0x10), local_1e, (local_1e >> 0x10));
            local_22 = uVar10 & 0xffff | extraout_DX_02 << 0x10;
            if ((extraout_DX_02 | uVar10) != 0) {
                local_24 = 1;
                while (local_24 < 0x15) {
                    local_26 = pass1_1030_ce2e(local_22, (local_22 >> 0x10), local_24);
                    if (local_26 != 0) {
                        pass1_1008_5784(CONCAT22(unaff_SS, local_2e), &local_struct_1.field_0x1a);
                        while {
                            puVar8 = local_2e;
                            pass1_1008_5b12(CONCAT22(unaff_SS, puVar8));
                            _local_32 = CONCAT22(extraout_DX_01, puVar8);
                            paVar5 = (extraout_DX_01 | puVar8);
                            if (paVar5 == 0x0) {
                                break;
                            }
                            (puVar8 + 0xe) != local_24
                        } {}
                        if (_local_32 == 0) {
                            uVar6 = big_switch_statement_1020_c222(local_24);
                            pass1_fn_1008_60e8(uVar6, paVar5);
                            uVar15 = 0;
                            paVar12 = paVar5;
                            uVar7 = uVar6;
                            process_struct_1000_179c(0x10, paVar5);
                            _local_e = CONCAT22(paVar12, uVar7);
                            if ((paVar12 | uVar7) == 0) {
                                uVar7 = 0;
                                uVar14 = 0;
                            } else {
                                uVar15 = 0x18;
                                uVar7 = local_26;
                                pass1_1018_48b0(
                                    _local_e,
                                    CONCAT13(
                                        (local_26 >> 0xf),
                                        CONCAT12(
                                            (local_26 >> 0xf),
                                            local_26 & 0xff | (local_26 >> 8) << 8,
                                        ),
                                    ),
                                    CONCAT22(paVar5, uVar6),
                                    local_24,
                                );
                                uVar14 = extraout_DX_00;
                            }
                            uVar2 = &local_struct_1.field_0x1a;
                            // WARNING: Load size is inaccurate
                            ppcVar3 = (*local_struct_1.field_0x1a + 4);
                            (**ppcVar3)(uVar15, uVar2, (uVar2 >> 0x10), uVar7, uVar14);
                        } else {
                            plVar1 = (puVar8 + 8);
                            unsafe {
                                *plVar1 = *plVar1 + local_26;
                            }
                        }
                        local_4 = 1;
                    }
                    local_24 = local_24 + 1;
                }
            }
            local_1e = local_1e + 1;
        }
    }
    uVar11 = uVar10;
    local_a = 0;
    if (local_4 == 0) {
        load_str_1010_84ac(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x43f,
        );
        _local_36 = CONCAT22(struct_a, uVar11);
        uVar15 = 0;
        paVar5 = struct_a;
        uVar9 = uVar11;
        process_struct_1000_179c(0x10, struct_a);
        local_12 = uVar9;
        local_10 = paVar5;
        if ((paVar5 | uVar9) == 0) {
            uVar9 = 0;
            uVar14 = 0;
        } else {
            uVar15 = 0x18;
            pass1_1018_48b0(CONCAT22(paVar5, uVar9), 0, CONCAT22(struct_a, uVar11), 0);
            uVar14 = extraout_DX_03;
        }
        uVar2 = &local_struct_1.field_0x1a;
        // WARNING: Load size is inaccurate
        ppcVar3 = (*local_struct_1.field_0x1a + 4);
        (**ppcVar3)(
            uVar15,
            uVar2,
            (uVar2 >> 0x10),
            uVar9,
            uVar14,
            _local_36,
            uVar9,
            uVar14,
        );
    }
    return;
}

pub fn process_struct_1008_d3ae(
    in_struct_1: *mut AStruct298,
    in_struct_a_2: *mut AStruct298,
) -> u8 {
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let mut bVar3: bool;
    let uVar4: u8;
    let paVar5: *mut AStruct199;
    let mut uVar6: u16;
    let mut uVar7: i32;
    let mut uVar8: i32;
    let struct_a: *mut AStruct199;
    let local_struct_1: *mut AStruct298;
    let mut uVar9: u16;
    let uVar10: u8;
    let local_struct_2: *mut AStruct199;
    let mut uVar11: u32;
    let mut uVar12: u32;
    let paVar13: *mut AStruct568;
    let paVar14: *mut AStruct568;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let fn_ptr_2: fn();

    uVar9 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    puVar1 = local_struct_1.astruct99_0xa;
    uVar8 = local_struct_1.field_0xc;
    local_struct_2 = CONCAT22(uVar8, puVar1);
    if ((uVar8 | puVar1) != 0) {
        unsafe {
            fn_ptr_2 = *puVar1;
        }
        local_struct_2 = (**fn_ptr_2)();
    }
    process_struct_1000_179c(0xc, (local_struct_2 >> 0x10));
    uVar8 = (local_struct_2 >> 0x10) | local_struct_2;
    if (local_struct_2 == 0x0) {
        paVar5 = 0x0;
        uVar8 = 0;
    } else {
        paVar5 = process_struct_1008_574a(local_struct_2);
    }
    local_struct_1.astruct99_0xa = paVar5;
    local_struct_1.field_0xc = uVar8;
    bVar3 = false;
    local_6 = 0x21;
    while (uVar4 = paVar5, 0x10 < local_6) {
        uVar11 = pass1_1038_540a(in_struct_a_2, local_6);
        struct_a = (uVar11 >> 0x10);
        paVar5 = (struct_a | uVar11);
        if (uVar11 != 0) {
            bVar3 = true;
            uVar6 = pass1_1020_c0ca(local_6);
            uVar7 = pass1_fn_1008_60e8(uVar6, struct_a);
            uVar10 = 0;
            uVar8 = uVar7;
            paVar5 = struct_a;
            process_struct_1000_179c(0x10, struct_a);
            if ((paVar5 | uVar8) == 0) {
                uVar12 = 0;
            } else {
                uVar10 = 0x18;
                uVar12 = pass1_1018_4790(
                    CONCAT22(paVar5, uVar8),
                    uVar11,
                    CONCAT22(struct_a, uVar7),
                    local_6,
                );
            }
            uVar2 = &local_struct_1.astruct99_0xa;
            fn_ptr_2 = (&local_struct_1.astruct99_0xa + 4);
            paVar5 = (**fn_ptr_2)(uVar10, uVar2, (uVar2 >> 0x10), uVar12);
        }
        local_6 = local_6 - 1;
    }
    if (!bVar3) {
        paVar13 = load_str_1010_84ac(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x43e,
        );
        uVar10 = 0;
        paVar14 = paVar13;
        process_struct_1000_179c(0x10, (paVar13 >> 0x10));
        if (paVar14 == 0x0) {
            uVar12 = 0;
        } else {
            uVar10 = 0x18;
            uVar12 = pass1_1018_4790(paVar14, 0, paVar13, 0);
        }
        uVar2 = &local_struct_1.astruct99_0xa;
        fn_ptr_2 = (&local_struct_1.astruct99_0xa + 4);
        uVar4 = (**fn_ptr_2)(uVar10, uVar2, (uVar2 >> 0x10), uVar12);
    }
    return uVar4;
}

pub fn process_struct_1008_dcdc(in_struct_1: *mut AStruct302) {
    let local_struct_1: *mut AStruct302;
    let mut uVar1: u16;

    uVar1 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1 = s_1_1050_389a;
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1008;
    local_struct_1.field_0x4 = 0;
    local_struct_1.field_0x8 = 0;
    local_struct_1.field_0xc = 0;
    local_struct_1.field_0xe = 0;
    local_struct_1.field_0x12 = 0;
    in_struct_1 = 0xdd4a;
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_dd4e(param_1: *mut AStruct303, param_2: u32) {
    let mut uVar1: i32;
    let paVar2: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let mut uVar3: i32;
    let mut uVar4: u16;
    let mut local_4: u16;

    uVar4 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar4, param_1), (param_2 >> 0x10));
    uVar1 = 0;
    &param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    CONCAT22(uVar4, param_1) = 0xeaac;
    param_1.field_0x2 = &PTR_LOOP_1050_1008;
    paVar2 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    uVar3 = paVar2 | uVar1;
    if (uVar3 == 0) {
        &param_1.field_0xa = 0;
    } else {
        paVar2 = process_struct_1008_574a(CONCAT22(paVar2, uVar1));
        param_1.field_0xa = paVar2;
        param_1.field_0xc = uVar3;
    }
    return;
}

pub fn process_struct_1008_ddca(in_struct_1: *mut AStruct304) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_BX_5: *mut AStruct304;
    let mut uVar4: u16;

    uVar4 = (in_struct_1 >> 0x10);
    local_BX_5 = in_struct_1;
    in_struct_1 = 0xeaac;
    local_BX_5.field_0x2 = &PTR_LOOP_1050_1008;
    puVar1 = local_BX_5.field_0xe;
    uVar2 = local_BX_5.field_0x10;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    puVar1 = local_BX_5.field_0x12;
    uVar2 = local_BX_5.field_0x14;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    puVar1 = local_BX_5.field_0xa;
    uVar2 = local_BX_5.field_0xc;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    error_check_1000_17ce(local_BX_5.field_0x1e);
    pass1_1010_1d80(in_struct_1);
    return;
}

pub fn process_string_14f0_01d8(param_1: u8, param_2: u8, param_3: u8, param_4: u16) {
    let pbVar1: *mut u8;
    let pcVar2: *mut code;
    let mut bVar3: u8;
    let mut iVar4: i32;
    let local_SP: *mut u16;
    let local_BP__1: *mut u16;
    let unaff_SI: *mut libc::c_char;
    let mut unaff_DI: i32;
    let mut local_SS__1: u16;
    let mut local_DS__1: u16;
    let mut local_7790: u16;
    let mut uStack9930: u16;
    let puStack64: *mut u16;
    let temp_87f6a135679: *mut u8;
    let temp_87fd3b67608: *mut libc::c_char;
    let temp_2030414afc22f7: *mut libc::c_char;
    let mut char_2: u8;
    let string_1: *mut libc::c_char;

    local_SP = &stack0xfffe;
    local_SP = &stack0xfffe;
    local_SP = (register0x00000010 + -2);
    char_2 = '\x1e';
    while {
        local_BP__1 = local_BP__1 + -1;
        local_SP = local_SP + -1;
        unsafe {
            *local_SP = *local_BP__1;
        }
        char_2 = char_2 + -1;
        '\0' < char_2
    } {}
    uStack9930 = (local_SP + unaff_SI);
    unaff_SI[param_4] = param_2;
    string_1 = unaff_SI + param_4;
    unsafe {
        *string_1 = *string_1 + param_1;
        string_1 = unaff_SI + param_4;
        *string_1 = *string_1 + param_1;
        string_1 = unaff_SI;
        *string_1 = *string_1 + param_2;
        pbVar1 = (unaff_SI + param_4);
        *pbVar1 = *pbVar1 >> 2 | *pbVar1 << 6;
        temp_87fd3b67608 = (&local_7790 + unaff_DI);
        temp_87fd3b67608 =
            temp_87fd3b67608 + (*pbVar1 < '\0') * ((unaff_SI & 3) - (temp_87fd3b67608 & 3));
        string_1 = unaff_SI + param_4;
        *string_1 = *string_1 + param_1;
        string_1 = unaff_SI + param_4;
        *string_1 = *string_1 + param_1;
        string_1 = unaff_SI + param_4;
        *string_1 = *string_1 + param_1;
        string_1 = unaff_SI + param_4;
        *string_1 = *string_1 + param_1;
        bVar3 = param_1 | 0xc4;
        string_1 = unaff_SI + param_4;
        *string_1 = *string_1 + param_2;
        iVar4 = CONCAT11(0x28, param_4);
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI;
        *string_1 = *string_1 + param_2;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1;
        temp_87f6a135679 = &stack0x007e + unaff_SI;
        *temp_87f6a135679 = *temp_87f6a135679;
        pbVar1 = (unaff_SI + iVar4);
        *pbVar1 = *pbVar1 | bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI;
        *string_1 = *string_1 + param_2;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1;
        unaff_SI[iVar4 + 0x80] = '(';
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        bVar3 = param_1 | 0xc4;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        pbVar1 = (unaff_SI + iVar4 + 0xf00);
        *pbVar1 = *pbVar1 | 0x28;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        LOCK();
        string_1 = unaff_SI + iVar4 + 0xc00;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + param_3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        bVar3 = param_1 | 0xc4;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + param_3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        string_1 = unaff_SI + iVar4;
        *string_1 = *string_1 + bVar3;
        pcVar2 = swi(3);
        (*pcVar2)();
    }
    return;
}

pub fn process_struct_1040_c630(param_1: *mut AStruct336) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut local_DX_67: u16;
    let mut local_DX_87: u16;
    let local_BX_6: *mut AStruct336;
    let mut local_ES_6: u16;
    let mut local_CS__1: u16;
    let mut temp_5f342bbd88: u32;

    local_ES_6 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    temp_5f342bbd88 = local_BX_6.field_0x42;
    if ((temp_5f342bbd88 + 0x12) != 0x71) {
        local_BX_6.field_0x36 = 5;
        local_BX_6.field_0x26 = 5;
        local_BX_6.field_0x28 = 5;
        iVar3 = local_BX_6.field_0x36;
        local_BX_6.field_0x30 = iVar3;
        local_BX_6.field_0x2e = iVar3;
        if (PTR_LOOP_1050_5f02 == 0x0) {
            mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0xff);
            _PTR_LOOP_1050_5f04 = CONCAT22(local_DX_67, iVar3);
            local_CS__1 = 0x1010;
            mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x100);
            _PTR_LOOP_1050_5f08 = CONCAT22(local_DX_87, iVar3);
        }
        PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + 1;
        local_BX_6.field_0x8 = _PTR_LOOP_1050_5f04;
        local_BX_6.field_0xc = _PTR_LOOP_1050_5f08;
        process_struct_1040_9618(param_1);
        local_BX_6.field_0x20 = 0;
        local_BX_6.field_0x1e = 200;
        local_BX_6.field_0x22 = 0xa0;
        local_BX_6.field_0x24 = local_BX_6.field_0x2c + local_BX_6.field_0x36;
        local_BX_6.field_0x2e = local_BX_6.field_0x36 * 3 + local_BX_6.field_0x2a;
        local_BX_6.field_0x30 = local_BX_6.field_0x36;
        local_BX_6.field_0x32 = local_BX_6.field_0x22 - local_BX_6.field_0x36;
        local_BX_6.field_0x3c = 0x25;
        uVar2 = param_1;
        ppcVar1 = (uVar2 + 4);
        (**ppcVar1)(local_CS__1, param_1);
        ppcVar1 = (uVar2 + 8);
        (**ppcVar1)(local_CS__1, param_1, local_ES_6);
    }
    return;
}

pub fn process_struct_1040_bf3e(param_1: *mut AStruct341, param_2: u16) -> *mut AStruct341 {
    let local_BX_4: *mut AStruct341;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    param_1 = (s_18_2_1050_3aa5 + 3);
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = param_2;
    param_1 = s_0_020_1050_3ab0;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x6 = 0;
    param_1 = 0xc53e;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1040;
    return param_1;
}

pub fn process_struct_1040_b7ee(param_1: *mut u16, param_2: u32, param_3: u16) {
    let local_BX_23: *mut AStruct342;
    let mut uVar1: u16;
    let mut uVar2: u16;

    process_struct_1040_b0bc(param_1, 0, CONCAT22(param_3, 0xfab));
    uVar1 = (param_1 >> 0x10);
    local_BX_23 = param_1;
    local_BX_23.field_0x94 = 0;
    local_BX_23.field_0x98 = 0;
    local_BX_23.field_0xb0 = 0;
    local_BX_23.field_0xb4 = 0;
    local_BX_23.field_0xb6 = 0;
    unsafe {
        *param_1 = 0xbeba;
    }
    local_BX_23.field_0x2 = &PTR_LOOP_1050_1040;
    if (param_2 != 0) {
        uVar2 = (param_2 >> 0x10);
        local_BX_23.field_0xb0 = (param_2 + 6);
        local_BX_23.field_0xb4 = (param_2 + 0x14);
    }
    return;
}

pub fn process_struct_1010_02e0(param_1: *mut AStruct316, param_2: u16, param_3: u16) {
    let mut uVar1: i32;
    let paVar2: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let mut uVar3: i32;
    let mut local_4: u16;

    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    uVar1 = 0;
    &param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x10 = 0;
    param_1.field_0x18 = 0;
    CONCAT22(param_2, param_1) = 0xe98;
    param_1.field_0x2 = 0x1010;
    paVar2 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    uVar3 = paVar2 | uVar1;
    if (uVar3 == 0) {
        &param_1.field_0xa = 0;
    } else {
        paVar2 = process_struct_1008_574a(CONCAT22(paVar2, uVar1));
        param_1.field_0xa = paVar2;
        param_1.field_0xc = uVar3;
    }
    return;
}

pub fn modify_struct_1010_0f24(
    param_1: *mut AStruct314,
    param_2: *mut AStruct314,
    param_3: *mut u8,
) {
    let mut unaff_BP: u16;
    let ppVar1: *mut pass1_struct_1;

    process_struct_1010_2cd2(param_1, param_2, param_3);
    param_1.field_0x60 = 0;
    param_1.field_0x64 = 0;
    param_1.field_0x68 = 0;
    &param_1.field_0x6a = 0;
    CONCAT22(param_2, param_1) = (s_648_bmp_1050_1919 + 1);
    param_1.field_0x2 = 0x1010;
    ppVar1 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 3));
    param_1.field_0x6a = ppVar1;
    param_1.field_0x6c = (ppVar1 >> 0x10);
    return;
}

pub fn modify_struct_1010_195e(
    param_1: *mut AStruct314,
    param_2: *mut AStruct314,
    param_3: *mut u8,
) {
    let mut unaff_BP: u16;
    let ppVar1: *mut pass1_struct_1;

    modify_struct_1010_0f24(param_1, param_2, param_3);
    (param_1 + 1) = 0;
    CONCAT22(param_2, param_1) = s_35_flc_1050_1b2a;
    param_1.field_0x2 = 0x1010;
    ppVar1 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 3));
    (param_1 + 1) = ppVar1;
    param_1[1].field_0x2 = (ppVar1 >> 0x10);
    return CONCAT22(param_2, param_1);
}

pub fn process_struct_1010_1d48(param_1: *mut AStruct375, param_2: u16) -> *mut AStruct375 {
    let local_BX_4: *mut AStruct375;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.ptr_1_lo = s_1_1050_389a;
    local_BX_4.ptr_1_hi = &PTR_LOOP_1050_1008;
    local_BX_4.u32_x04 = 0;
    local_BX_4.u16_x08 = param_2;
    param_1.ptr_1_lo = (s_573_bmp_1050_200f + 5);
    local_BX_4.ptr_1_hi = 0x1010;
    return param_1;
}

pub fn process_struct_1010_1df2(param_1: *mut AStruct377, param_2: u16, param_3: u32) {
    let in_ax: *mut AStruct199;
    let in_dx: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let local_BX_4: *mut AStruct377;
    let mut uVar1: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let fn_ptr_1: fn();

    uVar1 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    struct_a = in_dx;
    if (&local_BX_4.field_0x4 == 0) {
        process_struct_1000_179c(0xc, in_dx);
        struct_a = (in_dx | in_ax);
        if (struct_a == 0x0) {
            &local_BX_4.field_0x4 = 0;
        } else {
            in_ax = process_struct_1008_574a(CONCAT22(in_dx, in_ax));
            local_BX_4.field_0x4 = in_ax;
            &local_BX_4.field_0x6 = struct_a;
        }
    }
    process_struct_1000_179c(10, struct_a);
    _local_a = CONCAT22(struct_a, in_ax);
    if ((struct_a | in_ax) == 0) {
        local_6 = 0;
    } else {
        *_local_a = s_1_1050_389a;
        in_ax.field_0x2 = &PTR_LOOP_1050_1008;
        in_ax.field_0x4 = param_3;
        in_ax.field_0x8 = param_2;
        *_local_a = (s_573_bmp_1050_200f + 1);
        in_ax.field_0x2 = 0x1010;
        local_6 = _local_a;
    }
    fn_ptr_1 = (&local_BX_4.field_0x4 + 4);
    (**fn_ptr_1)(0x1000, &local_BX_4.field_0x4, local_6);
    return;
}

pub fn process_struct_1010_2bfc(param_1: *mut AStruct384, param_2: u32) -> *mut AStruct384 {
    let mut local_EAX__1: u32;
    let mut local_register2: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    local_6 = param_2;
    uStack4 = (param_2 >> 0x10);
    process_struct_1010_1d48(CONCAT22(local_6, param_1), uStack4);
    param_1.field_0xa = 0;
    param_1.field_0xc = 0;
    param_1.field_0xe = 0;
    param_1.field_0x10 = 0;
    CONCAT22(local_6, param_1) = s_add39_wav_1050_2cc2;
    param_1.field_0x2 = 0x1010;
    return (ZEXT24(param_1) | local_EAX__1 & 0xffff0000);
}

pub fn process_struct_1010_2cd2(
    param_1: *mut AStruct314,
    param_2: *mut AStruct314,
    param_3: *mut u8,
) -> u8 {
    let mut unaff_SS: u16;
    let ppVar1: *mut pass1_struct_1;
    let p_uvar2: *mut u16;
    let puVar3: *mut u16;
    let mut uVar4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x18 = 0;
    param_1.field_0x22 = 0;
    param_1.field_0x24 = 0;
    param_1.field_0x26 = 0;
    param_1.field_0x28 = 0;
    param_1.field_0x52 = 0;
    param_1.field_0x56 = 0;
    param_1.field_0x5a = 0;
    param_1.field_0x5e = 0;
    param_1.field_0x5c = 0;
    CONCAT22(param_2, param_1) = 0x36da;
    param_1.field_0x2 = 0x1010;
    puVar3 = &local_4;
    puVar2 = &local_6;
    uVar4 = unaff_SS;
    ppVar1 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(puVar2, 0x48));
    pass1_1008_3e94(
        (ppVar1 + 0xe),
        CONCAT22(unaff_SS, puVar2),
        CONCAT22(uVar4, puVar3),
    );
    &param_1.field_0xe = 0x19001db;
    param_1.field_0xa = 0x140 - (param_1.field_0xe / 2 - local_4);
    param_1.field_0xc = 0xf0 - (param_1.field_0x10 / 2 - local_6);
    param_1.field_0x1a = 0xa006e;
    param_1.field_0x1e = 0xa012c;
    pass1_1000_4906(CONCAT22(param_2, &param_1.field_0x2a), 0, 0x28);
    return param_1;
}

pub fn process_struct_1010_35a4(param_1: *mut AStruct388, param_2: u32) {
    let ppcVar1: fn();
    let mut uVar2: u32;
    let mut uVar3: i32;
    let paVar4: *mut AStruct114;
    let in_dx: *mut AStruct199;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let extraout_DX_00: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let extraout_DX_01: *mut AStruct199;
    let local_BX_5: *mut AStruct388;
    let mut uVar6: u16;
    let mut local_12: u16;
    let mut local_c: u32;
    let mut local_8: u32;

    uVar6 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    uVar2 = local_BX_5.field_0x56;
    uVar2 = (uVar2 + 8);
    local_8 = *(uVar2 + local_BX_5.field_0x5a * 4);
    local_c = param_2;
    if (param_2 != 0) {
        uVar6 = 0x1000;
        process_struct_1000_179c(0x4a, in_dx);
        uVar3 = param_2;
        if ((in_dx | uVar3) == 0) {
            uVar3 = 0;
            uVar5 = 0;
        } else {
            uVar6 = SUB42(&PTR_LOOP_1050_1040, 0);
            pass1_1040_c54a(param_2 & 0xffff | ZEXT24(in_dx) << 0x10, 1, local_8);
            uVar5 = extraout_DX;
        }
        ppcVar1 = (param_1 + 0x18);
        (**ppcVar1)(uVar6, param_1, 1, uVar3, uVar5);
        struct_a = extraout_DX_00;
        while ((local_c & 0xf) != 0) {
            uVar2 = (local_8 + 8);
            local_8 = *(((local_c & 0xf) - 1) * 4 + uVar2);
            uVar6 = 0x1000;
            paVar4 = local_8;
            process_struct_1000_179c(0x4a, struct_a);
            uVar3 = paVar4;
            if ((struct_a | uVar3) == 0) {
                uVar3 = 0;
            } else {
                uVar6 = SUB42(&PTR_LOOP_1050_1040, 0);
                pass1_1040_c54a(paVar4 & 0xffff | ZEXT24(struct_a) << 0x10, 1, local_8);
            }
            ppcVar1 = (param_1 + 0x18);
            (**ppcVar1)(uVar6, param_1, 1, uVar3);
            local_c = local_c >> 4;
            struct_a = extraout_DX_01;
        }
    }
    return;
}

pub fn process_struct_1010_3680(param_1: u16, param_2: u16, param_3: u16, param_4: u16) {
    let mut in_ax: i32;
    let in_dx: *mut AStruct199;
    let mut local_4: u16;

    process_struct_1000_179c(0x4a, in_dx);
    if ((in_dx | in_ax) != 0) {
        pass1_1040_c54a(CONCAT22(in_dx, in_ax), 1, CONCAT22(param_2_00, param_1_00));
        return;
    }
    return;
}

pub fn process_struct_1010_38f8(param_1: *mut AStruct405, param_2: u16) -> u16 {
    let mut in_ax: i32;
    let mut uVar1: u16;
    let in_dx: *mut AStruct199;
    let local_BX_25: *mut AStruct405;
    let mut uVar2: u16;
    let puVar3: *mut u16;
    let mut local_4: u16;

    if (param_2 != 0) {
        uVar1 = param_2 << 2;
        process_struct_1000_179c(uVar1, in_dx);
        uVar2 = (param_1 >> 0x10);
        local_BX_25 = param_1;
        local_BX_25.field_0x8 = uVar1;
        local_BX_25.field_0xa = in_dx;
        return CONCAT22(in_dx, local_BX_25.field_0x8);
    }
    process_struct_1000_179c(0x1a, in_dx);
    if ((in_dx | in_ax) != 0) {
        puVar3 = pass1_1010_37d4(CONCAT22(in_dx, in_ax));
        return puVar3;
    }
    return 0x0;
}

pub fn process_struct_1010_394a(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut in_ax: i32;
    let local_struct_1: *mut AStruct402;
    let mut local_4: u16;

    if (param_1_00 != 0) {
        process_struct_1000_179c(param_1_00 << 2, local_struct_1);
        return;
    }
    process_struct_1000_179c(0x16, local_struct_1);
    if ((local_struct_1 | in_ax) != 0) {
        pass1_1010_383a(CONCAT22(local_struct_1, in_ax));
        return;
    }
    return;
}

pub fn process_struct_1010_3b7a(param_1: *mut AStruct64, param_2: u32) {
    let mut uVar1: u16;

    uVar1 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar1, param_1), (param_2 >> 0x10));
    param_1.field_0xa = s_1_1050_389a;
    &param_1.field_0xc = &PTR_LOOP_1050_1008;
    param_1.field_0xa = (s_18_2_1050_3aa5 + 3);
    &param_1.field_0xc = &PTR_LOOP_1050_1008;
    &param_1.field_0xe = 0;
    &param_1.field_0x12 = 0;
    (&param_1.field_0x12 + 2) = 0;
    &param_1.field_0x16 = 0;
    CONCAT22(uVar1, param_1) = 0x3d6a;
    param_1.field_0x2 = 0x1010;
    param_1.field_0xa = 0x3d7a;
    &param_1.field_0xc = 0x1010;
    return;
}

pub fn process_struct_1010_3e3c(in_struct_1: *mut AStruct393, param_2: u16) {
    let uVar1: u8;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let local_struct_1: *mut AStruct393;
    let local_struct_2: *mut AStruct393;
    let mut uStack16: u16;
    let mut uStack10: u16;
    let mut uVar2: i32;

    local_struct_2 = in_struct_1;
    uStack16 = (in_struct_1 >> 0x10);
    get_sys_metrics_1018_4b1e(in_struct_1, 6, param_2);
    local_struct_2.field_0x20 = s_1_1050_389a;
    local_struct_2.field_0x22 = &PTR_LOOP_1050_1008;
    local_struct_2.field_0x20 = (s_18_2_1050_3aa5 + 3);
    local_struct_2.field_0x22 = &PTR_LOOP_1050_1008;
    local_struct_2.field_0x24 = 0;
    &local_struct_2.field_0x66 = 0;
    local_struct_2.field_0x6a = 4;
    local_struct_2.field_0x6c = 0;
    local_struct_2.field_0x70 = 0;
    local_struct_2.field_0x74 = 0;
    pass1_1008_3e54(
        (in_struct_1 & 0xffff0000 | &local_struct_2.field_0x76),
        0,
        3,
        5,
    );
    local_struct_2.field_0x7c = 0;
    in_struct_1.field_0x0 = &PTR_LOOP_1050_4a46;
    local_struct_2.u16_0x2 = 0x1010;
    local_struct_2.field_0x20 = &PTR_LOOP_1050_4a82;
    local_struct_2.field_0x22 = 0x1010;
    uVar1 = pass1_1000_4906(
        (in_struct_1 & 0xffff0000 | &local_struct_2.field_0x26),
        0,
        0x40,
    );
    uVar2 = CONCAT31(extraout_var, uVar1);
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1a1);
    local_struct_2.field_0x66 = uVar2;
    local_struct_2.field_0x68 = extraout_DX;
    pass1_1018_4b78(in_struct_1, uStack10);
    return;
}

pub fn pass1_1010_3f00(param_1: *mut AStruct376) {
    let puVar1: *mut u32;
    let mut uVar2: i32;
    let ppcVar3: fn();
    let local_AX_159: *mut AStruct391;
    let local_BX_5: *mut AStruct392;
    let mut uVar4: u16;
    let mut local_10: u32;
    let mut local_4: u16;

    uVar4 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    param_1.ptr_a_lo = &PTR_LOOP_1050_4a46;
    local_BX_5.field_0x2 = 0x1010;
    local_BX_5.field_0x20 = &PTR_LOOP_1050_4a82;
    local_BX_5.field_0x22 = 0x1010;
    local_4 = 0;
    while {
        puVar1 = (&local_BX_5.field_0x26 + local_4 * 4);
        uVar2 = (&local_BX_5.field_0x26 + local_4 * 4 + 2);
        if ((uVar2 | puVar1) != 0) {
            unsafe {
                ppcVar3 = *puVar1;
                (**ppcVar3)();
            }
        }
        local_4 = local_4 + 1;
        local_4 < 0x10
    } {}
    puVar1 = local_BX_5.field_0x66;
    uVar2 = local_BX_5.field_0x68;
    if ((uVar2 | puVar1) != 0) {
        unsafe {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    error_check_1000_17ce(local_BX_5.field_0x70);
    if (param_1 == 0x0) {
        local_AX_159 = 0x0;
        uVar4 = 0;
    } else {
        local_AX_159 = &local_BX_5.field_0x20;
    }
    local_10 = CONCAT22(uVar4, local_AX_159);
    local_10 = s_1_1050_389a;
    local_AX_159.field_0x2 = &PTR_LOOP_1050_1008;
    pass1_1010_1d80(param_1);
    return;
}

pub fn process_struct_1010_4a8a(param_1: *mut AStruct375, param_2: *mut AStruct375, param_3: u16) {
    let local_AX_19: *mut u8;
    let local_DX_102: *mut u8;
    let ppVar1: *mut pass1_struct_1;
    let local_6: *mut u8;

    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    local_AX_19 = 0x0;
    param_1.u16_x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    param_1.field_0x20 = 1;
    param_1.field_0x22 = 0;
    param_1.field_0x24 = 0;
    &param_1.field_0x26 = 0;
    param_1.field_0x2a = 0;
    param_1.field_0x2c = 1;
    param_1.field_0x2e = 0;
    param_1.field_0x30 = 0;
    &param_1.field_0x32 = 0;
    CONCAT22(param_2, param_1) = (s_SCForceMorale__s_for_colony__08l_1050_5024 + 6);
    param_1.ptr_1_hi = 0x1010;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1b3);
    *&param_1.u16_x16 = local_AX_19;
    *(&param_1.u16_x16 + 2) = local_DX_102;
    ppVar1 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(local_6, 3));
    param_1.field_0x26 = ppVar1;
    &param_1.field_0x28 = (ppVar1 >> 0x10);
    process_struct_1008_4772(param_1.u16_x16);
    param_1.u32_x0e = 0x13c;
    param_1.ptr_2_lo = 0;
    param_1.field_0x10 = 0;
    param_1.ptr_2_hi = 0;
    return;
}

pub fn process_struct_1010_4d5c(
    param_1: *mut AStruct412,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let mut u_var1: u32;
    let p_uvar2: *mut u16;
    let in_dx: *mut AStruct199;
    let local_BX_4: *mut AStruct412;
    let local_SI_55: *mut AStruct413;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    if (&local_BX_4.field_0x1a == 0) {
        puVar2 = (local_BX_4.field_0x30 << 3);
        process_struct_1000_179c(puVar2, in_dx);
        local_BX_4.field_0x1a = puVar2;
        &local_BX_4.field_0x1c = in_dx;
    }
    uVar1 = &local_BX_4.field_0x1a;
    local_SI_55 = (param_6 * 8);
    (local_SI_55 + uVar1) = param_5;
    uVar1 = &local_BX_4.field_0x1a;
    (local_SI_55 + uVar1 + 2) = param_4;
    uVar1 = &local_BX_4.field_0x1a;
    (local_SI_55 + uVar1 + 4) = param_3;
    uVar1 = &local_BX_4.field_0x1a;
    (local_SI_55 + uVar1 + 6) = param_2;
    return;
}

pub fn process_struct_1010_5d9c(param_1: u32, button_state: u16) {
    let ppVar1: *mut pass1_struct_1;
    let mut in_stack_0000fffa: u16;

    (param_1 + 0x1e) = button_state;
    if (button_state == 0) {
        ppVar1 =
            process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(in_stack_0000fffa, 0x2e));
        pass1_1018_209c(ppVar1);
    }
    return;
}

pub fn process_struct_1010_60fa(param_1: *mut AStruct434) {
    let local_BX_3: *mut AStruct434;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    local_BX_3.field_0x7e = 1;
    local_BX_3.field_0x7c = local_BX_3.field_0x20;
    local_BX_3.field_0x20 = 1;
    return;
}

pub fn process_struct_1010_6118(param_1: *mut AStruct435) {
    let local_BX_3: *mut AStruct435;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    if (local_BX_3.field_0x7e != 0) {
        local_BX_3.field_0x20 = local_BX_3.field_0x7c;
    }
    return;
}

pub fn modify_struct_1010_6326(param_1: *mut AStruct436, param_2: u32) {
    let mut uVar1: u16;

    uVar1 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar1, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    param_1.field_0x22 = 0;
    CONCAT22(uVar1, param_1) = 0x66f0;
    param_1.field_0x2 = 0x1010;
    return;
}

pub fn modify_struct_1010_6700(param_1: *mut AStruct437, param_2: u32) {
    let extraout_var: u32;
    let mut uVar1: u16;

    uVar1 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar1, param_1), (param_2 >> 0x10));
    param_1.field_0x148 = 0x33;
    CONCAT22(uVar1, param_1) = 0x6aac;
    param_1.field_0x2 = 0x1010;
    pass1_1000_4906(CONCAT22(uVar1, &param_1.field_0xa), 0, 0x114);
    param_1.field_0x32 = 1;
    param_1.field_0x40 = 1;
    param_1.field_0x46 = 1;
    param_1.field_0x4e = 1;
    param_1.field_0x54 = 1;
    param_1.field_0x5e = 1;
    param_1.field_0x68 = 1;
    param_1.field_0x6c = 1;
    param_1.field_0x74 = 1;
    param_1.field_0x78 = 1;
    param_1.field_0x7a = 1;
    param_1.field_0x7e = 1;
    param_1.field_0x82 = 1;
    param_1.field_0xa2 = 1;
    param_1.field_0xa4 = 1;
    param_1.field_0xa6 = 1;
    param_1.field_0xa8 = 1;
    param_1.field_0xae = 1;
    param_1.field_0xb2 = 1;
    param_1.field_0xb8 = 1;
    param_1.field_0xbe = 1;
    param_1.field_0xc0 = 1;
    param_1.field_0xc4 = 1;
    param_1.field_0xd4 = 1;
    param_1.field_0xda = 1;
    param_1.field_0xe2 = 1;
    param_1.field_0xfe = 1;
    param_1.field_0x100 = 1;
    param_1.field_0x102 = 1;
    param_1.field_0x104 = 1;
    param_1.field_0x106 = 1;
    param_1.field_0x108 = 1;
    pass1_1000_4906(CONCAT22(uVar1, &param_1.field_0x11e), 0, 0x2a);
    param_1.field_0x120 = 1;
    param_1.field_0x122 = 1;
    param_1.field_0x124 = 1;
    param_1.field_0x126 = 1;
    param_1.field_0x128 = 1;
    param_1.field_0x12c = 1;
    param_1.field_0x138 = 1;
    return (extraout_var & 0xffff00) << 8 | ZEXT24(param_1);
}

pub fn process_struct_1010_6abc(param_1: *mut AStruct438, param_2: u32) {
    let ppcVar1: fn();
    let ppVar2: *mut pass1_struct_1;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut in_stack_0000fff6: u16;

    uVar4 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar4, param_1), (param_2 >> 0x10));
    param_1.field_0xa = s_1_1050_389a;
    param_1.field_0xc = &PTR_LOOP_1050_1008;
    param_1.field_0xa = (s_18_2_1050_3aa5 + 3);
    param_1.field_0xc = &PTR_LOOP_1050_1008;
    param_1.field_0xe = 0;
    param_1.field_0x10 = 0;
    &param_1.field_0x14 = 0;
    param_1.field_0x1c = 0;
    param_1.field_0x20 = 0;
    &param_1.field_0x22 = 0;
    CONCAT22(uVar4, param_1) = 0x7e28;
    param_1.field_0x2 = 0x1010;
    param_1.field_0xa = 0x7e38;
    param_1.field_0xc = 0x1010;
    ppVar2 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(in_stack_0000fff6, 3));
    param_1.field_0x14 = ppVar2;
    &param_1.field_0x16 = (ppVar2 >> 0x10);
    uVar3 = *&param_1.field_0x14;
    ppcVar1 = (&param_1.field_0x14 + 4);
    (**ppcVar1)();
    ppVar2 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(uVar3, 0x2f));
    param_1.field_0x22 = ppVar2;
    &param_1.field_0x24 = (ppVar2 >> 0x10);
    ppcVar1 = (&param_1.field_0x22 + 4);
    (**ppcVar1)();
    return;
}

pub fn process_struct_1010_9298(
    in_struct_1: *mut AStruct314,
    param_2: *mut AStruct314,
    param_3: u16,
) {
    let uVar1: u8;
    let extraout_AH: u8;
    let in_dx: *mut AStruct199;
    let mut uVar2: i32;

    uVar1 = process_struct_1010_2cd2(in_struct_1, param_2, param_3);
    uVar2 = CONCAT11(extraout_AH, uVar1);
    CONCAT22(param_2, in_struct_1) = 0x9566;
    in_struct_1.field_0x2 = 0x1010;
    process_struct_1000_179c(0x20c, in_dx);
    in_struct_1.field_0x5c = uVar2;
    in_struct_1.field_0x5e = in_dx;
    pass1_1000_4906(CONCAT22(in_dx, in_struct_1.field_0x5c), 0, 0x20c);
    return CONCAT22(param_2, in_struct_1);
}

pub fn process_struct_1040_b082(param_1: *mut u16, param_2: u32) {
    let local_BX_21: *mut AStruct344;
    let mut uVar1: u16;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        0,
        param_2,
        (param_2 >> 0x10),
    );
    uVar1 = (param_1 >> 0x10);
    local_BX_21 = param_1;
    local_BX_21.field_0x8e = 0;
    local_BX_21.field_0x90 = 0;
    unsafe {
        *param_1 = 0xb772;
    }
    local_BX_21.field_0x2 = &PTR_LOOP_1050_1040;
    return;
}

pub fn process_struct_1040_b0bc(param_1: *mut AStruct346, param_2: u32, param_3: u32) {
    let local_BX_21: *mut AStruct346;
    let mut uVar1: u16;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        0,
        param_3,
        (param_3 >> 0x10),
    );
    uVar1 = (param_1 >> 0x10);
    local_BX_21 = param_1;
    local_BX_21.field_0x8e = 0;
    local_BX_21.field_0x90 = param_2;
    param_1 = 0xb772;
    local_BX_21.field_0x2 = &PTR_LOOP_1050_1040;
    return;
}

pub fn process_struct_1008_e3ec(in_struct_a: *mut AStruct310, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let local_AX_88: *mut u32;
    let p_uvar2: *mut u16;
    let struct_c_lo: *mut AStruct199;
    let extraout_DX: *mut AStruct199;
    let local_DX_78: *mut u8;
    let mut extraout_DX_00: i32;
    let paVar3: *mut AStruct199;
    let extraout_DX_01: *mut AStruct199;
    let local_DX_273: *mut u8;
    let puVar4: *mut u8;
    let mut extraout_DX_02: i32;
    let struct_b: *mut AStruct310;
    let struct_b_hi: *mut AStruct310;
    let local_SS__1: *mut u8;
    let mut local_20: u16;
    let mut local_1e: u16;
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
    let mut temp_5fd014a0be: u32;
    let struct_c_hi: *mut AStruct199;
    let fn_ptr_a: fn();

    struct_b_hi = (in_struct_a >> 0x10);
    struct_b = in_struct_a;
    local_AX_88 = struct_b.field_0xe;
    paVar3 = struct_b.field_0x10;
    if ((paVar3 | local_AX_88) != 0) {
        unsafe {
            fn_ptr_a = *local_AX_88;
            (**fn_ptr_a)();
        }
        paVar3 = extraout_DX;
    }
    process_struct_1000_179c(0x18, paVar3);
    if ((paVar3 | local_AX_88) == 0) {
        local_AX_88 = 0x0;
        puVar4 = 0x0;
    } else {
        pass1_1030_1cd8(CONCAT13((paVar3 >> 8), CONCAT12(paVar3, local_AX_88)), 5, 5);
        puVar4 = local_DX_78;
    }
    struct_b.field_0xe = local_AX_88;
    struct_b.field_0x10 = puVar4;
    pass1_1028_dc52(
        CONCAT22(local_SS__1, &local_14),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        puVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(local_SS__1, puVar2));
        if ((extraout_DX_00 | puVar2) == 0) {
            break;
        }
        if ((puVar2 + 0x100) != 0x8000002) {
            temp_5fd014a0be = &struct_b.field_0xe;
            fn_ptr_a = (&struct_b.field_0xe + 0xc);
            (**fn_ptr_a)(0x28, temp_5fd014a0be, (temp_5fd014a0be >> 0x10));
        }
    }
    param_3 = &struct_b.field_0xe;
    struct_c_hi = struct_b.field_0x14;
    struct_c_lo = &struct_b.field_0x12;
    paVar3 = (struct_c_hi | struct_c_lo);
    if (paVar3 != 0x0) {
        fn_ptr_a = struct_c_lo;
        (**fn_ptr_a)(&PTR_LOOP_1050_1028, struct_c_lo);
        paVar3 = extraout_DX_01;
    }
    process_struct_1000_179c(0x18, paVar3);
    if ((paVar3 | struct_c_lo) == 0) {
        struct_c_lo = 0x0;
        puVar4 = 0x0;
    } else {
        pass1_1030_1cd8(CONCAT13((paVar3 >> 8), CONCAT12(paVar3, struct_c_lo)), 5, 5);
        puVar4 = local_DX_273;
    }
    struct_b.field_0x12 = struct_c_lo;
    struct_b.field_0x14 = puVar4;
    local_c = local_8;
    local_a = local_6;
    if (local_4 != 0) {
        local_c = 1;
        local_a = 0;
    }
    while (true) {
        puVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(local_SS__1, puVar2));
        if ((extraout_DX_02 | puVar2) == 0) {
            break;
        }
        uVar1 = &struct_b.field_0x12;
        fn_ptr_a = (&struct_b.field_0x12 + 0xc);
        (**fn_ptr_a)(0x28, uVar1, (uVar1 >> 0x10));
    }
    param_2 = &struct_b.field_0x12;
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn process_struct_1008_e586(param_1: *mut u8, param_2: u16, param_1_00: u32) {
    let paVar1: *mut AStruct493;
    let mut in_dx: i32;
    let struct_a: *mut AStruct199;
    let in_string_2: *mut libc::c_char;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_1_00, (param_1_00 >> 0x10));
    _local_6 = CONCAT22(in_dx, paVar1);
    struct_a = (in_dx | paVar1);
    if (struct_a == 0x0) {
        return 0;
    }
    process_struct_1000_179c(0x80, struct_a);
    _local_a = CONCAT22(struct_a, paVar1);
    in_string_2 = pass1_1038_4d28(_local_6);
    copy_string_1000_3d3e(_local_a, in_string_2);
    return CONCAT22(struct_a, paVar1);
}

pub fn process_struct_1008_ea86(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    process_struct_1008_ddca(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1008_ed1e(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut in_ax: i32;
    let in_dx: *mut AStruct199;
    let mut local_4: u16;

    if (param_1_00 != 0) {
        process_struct_1000_179c(param_1_00 << 2, in_dx);
        return;
    }
    process_struct_1000_179c(0x1a, in_dx);
    if ((in_dx | in_ax) != 0) {
        pass1_1008_ec72(CONCAT22(in_dx, in_ax));
        return;
    }
    return;
}

pub fn process_struct_1010_9348(in_struct_1: *mut AStruct460, param_2: u16) {
    let local_struct_1: *mut AStruct460;
    let mut local_ES_21: u16;

    (param_2 * 8 + 0x319e) = param_2;
    local_ES_21 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.b = param_2 * 8 + 0x3198;
    local_struct_1.c = &g_alloc_addr_1050_1050;
    local_struct_1.a = param_2;
    return;
}

pub fn process_struct_1010_95aa(param_1: *mut AStruct464, param_2: u32) {
    let mut uVar1: u16;

    uVar1 = param_2;
    process_struct_1010_1d48(CONCAT22(uVar1, param_1), (param_2 >> 0x10));
    param_1.b = 0;
    param_1.c = 0;
    param_1.d = 0;
    param_1.e = 0;
    param_1.f = 0;
    param_1.g = 0;
    param_1.h = 10;
    param_1.j = 0;
    CONCAT22(uVar1, param_1) = 0xa1c8;
    param_1.a = 0x1010;
    return;
}

pub fn process_struct_1010_9fee(param_1: u32, param_2: u16, param_3: u16) {
    let ppcVar1: fn();
    let in_ax: *mut AStruct199;
    let in_dx: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let local_BX_4: *mut AStruct474;
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    struct_a = in_dx;
    if (&local_BX_4.field_0x12 == 0) {
        process_struct_1000_179c(0xc, in_dx);
        struct_a = (in_dx | in_ax);
        if (struct_a == 0x0) {
            &local_BX_4.field_0x12 = 0;
        } else {
            in_ax = process_struct_1008_574a(CONCAT22(in_dx, in_ax));
            local_BX_4.field_0x12 = in_ax;
            &local_BX_4.field_0x14 = struct_a;
        }
    }
    process_struct_1000_179c(8, struct_a);
    _local_a = CONCAT22(struct_a, in_ax);
    if ((struct_a | in_ax) == 0) {
        local_6 = 0;
    } else {
        *_local_a = s_1_1050_389a;
        in_ax.field_0x2 = &PTR_LOOP_1050_1008;
        *_local_a = 0xa1c4;
        in_ax.field_0x2 = 0x1010;
        local_6 = _local_a;
    }
    uVar4 = (local_6 >> 0x10);
    iVar2 = local_6;
    (iVar2 + 4) = param_3;
    (iVar2 + 6) = param_2;
    ppcVar1 = (&local_BX_4.field_0x12 + 4);
    (**ppcVar1)(0x1000, &local_BX_4.field_0x12, iVar2, uVar4);
    return;
}

pub fn process_struct_1010_a1d8(
    in_struct_a_1: *mut AStruct475,
    in_struct_a_2: *mut AStruct475,
    param_3: u16,
) {
    let ppcVar1: fn();
    let string_ptr_base: *mut libc::c_char;
    let struct_a: *mut pass1_struct_1;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: u32;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u32;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u32;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_27fdf014d48: *mut AStruct1175;

    process_struct_1010_1d48(CONCAT22(in_struct_a_2, in_struct_a_1), param_3);
    &in_struct_a_1.field_0xa = s_1_1050_389a;
    (&in_struct_a_1.field_0xa + 2) = &PTR_LOOP_1050_1008;
    &in_struct_a_1.field_0xa = (s_18_2_1050_3aa5 + 3);
    (&in_struct_a_1.field_0xa + 2) = &PTR_LOOP_1050_1008;
    &in_struct_a_1.field_0x138 = 0;
    CONCAT22(in_struct_a_2, in_struct_a_1) = 0xe9cc;
    in_struct_a_1.field_0x2 = 0x1010;
    &in_struct_a_1.field_0xa = 0xe9dc;
    (&in_struct_a_1.field_0xa + 2) = 0x1010;
    struct_a = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(string_ptr_base, 0x2f));
    &in_struct_a_1.field_0x138 = struct_a;
    (in_struct_a_1 + 1).field_0x0 = (struct_a >> 0x10);
    ppcVar1 = (&in_struct_a_1.field_0x138 + 4);
    (**ppcVar1)();
    pass1_1000_4906(CONCAT22(in_struct_a_2, &in_struct_a_1.field_0xa4), 0, 0x94);
    pass1_1000_4906(CONCAT22(in_struct_a_2, &in_struct_a_1.field_0xe), 0, 0x96);
    local_4 = 0;
    while {
        temp_27fdf014d48 = (&in_struct_a_1.field_0x0 + local_4 * 3);
        temp_27fdf014d48.field_0xe = call_big_fn_1010_c7e2;
        temp_27fdf014d48.field_0x12 = 0;
        local_4 = local_4 + 1;
        local_4 < 0x19
    } {}
    *(&in_struct_a_1.field_0x48 + 2) = pass1_1010_c864;
    &in_struct_a_1.field_0x4e = 0;
    *(&in_struct_a_1.field_0x4e + 2) = pass1_1010_cc56;
    &in_struct_a_1.field_0x54 = 0;
    *(&in_struct_a_1.field_0x54 + 2) = pass1_1010_cf36;
    &in_struct_a_1.field_0x5a = 0;
    *(&in_struct_a_1.field_0x2a + 2) = pass1_1010_d24a;
    &in_struct_a_1.field_0x30 = 0;
    *(&in_struct_a_1.field_0x6c + 2) = pass1_1010_d448;
    &in_struct_a_1.field_0x72 = 0;
    *(&in_struct_a_1.field_0x72 + 2) = pass1_1010_d5ae;
    &in_struct_a_1.field_0x78 = 0;
    *(&in_struct_a_1.field_0x96 + 2) = pass1_1010_d710;
    &in_struct_a_1.field_0x9c = 0;
    return;
}

pub fn process_struct_1018_0570(param_1: *mut u16, param_2: u16) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let puVar3: *mut u16;
    let mut iVar4: i32;
    let struct_a: *mut AStruct199;
    let paVar5: *mut AStruct199;
    let mut uVar6: u16;
    let local_BX_18: *mut AStruct508;
    let ppVar7: *mut pass1_struct_1;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut in_stack_0000fff0: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar9 = (param_1 >> 0x10);
    get_sys_metrics_1018_4b1e(param_1, uVar9, 0, param_2);
    local_BX_18 = param_1;
    local_BX_18.field_0x20 = s_1_1050_389a;
    local_BX_18.field_0x22 = &PTR_LOOP_1050_1008;
    local_BX_18.field_0x20 = (s_18_2_1050_3aa5 + 3);
    local_BX_18.field_0x22 = &PTR_LOOP_1050_1008;
    local_BX_18.field_0x24 = 0;
    &local_BX_18.field_0x2c = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_BX_18.field_0x30));
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_BX_18.field_0x36));
    local_BX_18.field_0x3c = 0;
    zero_list_1008_6c90(&local_BX_18.field_0x40, uVar9);
    uVar6 = 0;
    local_BX_18.field_0x4c = 0;
    local_BX_18.field_0x5a = 0;
    local_BX_18.field_0x5e = 0;
    local_BX_18.field_0x60 = 0;
    local_BX_18.field_0x64 = 0xff00;
    local_BX_18.field_0x66 = 0;
    local_BX_18.field_0x68 = 0x10000fb;
    local_BX_18.field_0x6c = 0x10000f9;
    local_BX_18.field_0x70 = 0x10000ff;
    local_BX_18.field_0x74 = 0x10000fe;
    local_BX_18.field_0x78 = 0x10000fc;
    local_BX_18.field_0x7c = 0;
    local_BX_18.field_0x80 = 0;
    local_BX_18.field_0x84 = 1;
    local_BX_18.field_0x86 = 0;
    local_BX_18.field_0x88 = 0;
    local_BX_18.field_0x8c = 0;
    local_BX_18.field_0x8e = 0;
    local_BX_18.field_0x92 = 0;
    local_BX_18.field_0x94 = 0;
    local_BX_18.field_0x98 = 0;
    local_BX_18.field_0x9a = 0;
    &local_BX_18.field_0xa2 = 0;
    local_BX_18.field_0xa6 = 0xffff;
    local_BX_18.field_0xa8 = 0;
    unsafe {
        *param_1 = (s_582_bmp_1050_1871 + 3);
    }
    local_BX_18.field_0x2 = 0x1018;
    local_BX_18.field_0x20 = (s_589_bmp_1050_18a9 + 7);
    local_BX_18.field_0x22 = 0x1018;
    if ((PTR_LOOP_1050_3960 == 0x0) && (_PTR_LOOP_1050_3962 == 0)) {
        paVar5 = struct_a;
        process_struct_1000_179c(8, struct_a);
        _PTR_LOOP_1050_3962 = CONCAT22(paVar5, uVar6);
        pass1_1000_4906(CONCAT22(paVar5, uVar6), 0, 8);
    }
    PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + 1;
    ppVar7 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(in_stack_0000fff0, 0x2f));
    local_BX_18.field_0x2c = ppVar7;
    &local_BX_18.field_0x2e = (ppVar7 >> 0x10);
    if (param_1 == 0x0) {
        puVar3 = 0x0;
        uVar6 = 0;
    } else {
        puVar3 = &local_BX_18.field_0x20;
        uVar6 = uVar9;
    }
    uVar1 = &local_BX_18.field_0x2c;
    uVar8 = uVar1;
    ppcVar2 = (&local_BX_18.field_0x2c + 4);
    ppcVar2(0x10, uVar8, (uVar1 >> 0x10), 0, puVar3, uVar6);
    ppVar7 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(uVar8, 2));
    if ((ppVar7 + 0x80) != 0) {
        local_BX_18.field_0x84 = 2;
    }
    ppVar7 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(uVar8, 9));
    iVar4 = ppVar7;
    local_BX_18.field_0x9e = iVar4;
    local_BX_18.field_0xa0 = (ppVar7 >> 0x10);
    pass1_1010_65d0(ppVar7 & 0xffff0000 | local_BX_18.field_0x9e, 0x88);
    if (iVar4 != 0) {
        local_BX_18.field_0xa8 = 1;
    }
    ppVar7 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(uVar8, 0x3b));
    local_BX_18.field_0xa2 = ppVar7;
    local_BX_18.field_0xa4 = (ppVar7 >> 0x10);
    return;
}

pub fn set_struct_1018_36e6(
    in_struct_1: *mut AStruct552,
    param_2: u16,
    param_3: u16,
    param_4: u16,
) {
    let struct_a_2: *mut AStruct552;
    let struct_a_1: *mut AStruct552;

    struct_a_1 = (in_struct_1 >> 0x10);
    struct_a_2 = in_struct_1;
    struct_a_2.field_0x12e = param_4;
    struct_a_2.field_0x130 = param_3;
    struct_a_2.field_0x132 = param_2;
    struct_a_2.field_0x134 = 0;
    return;
}

pub fn process_struct_1018_e2a0(in_AStruct376: *mut AStruct376) {
    let local_AStruct376_lo: *mut AStruct376;
    let local_AStruct376_hi: *mut AStruct376;

    local_AStruct376_hi = (in_AStruct376 >> 0x10);
    local_AStruct376_lo = in_AStruct376;
    in_AStruct376.ptr_a_lo = 0xe44e;
    local_AStruct376_lo.ptr_a_hi = 0x1018;
    local_AStruct376_lo.u16_xe2 = 0xe4ea;
    local_AStruct376_lo.u16_xe4 = 0x1018;
    process_struct_1020_808e(in_AStruct376);
    return;
}

pub fn process_struct_1018_e428(
    in_AStruct376_ptr_1: *mut AStruct376,
    param_2: u8,
) -> *mut AStruct376 {
    process_struct_1018_e2a0(in_AStruct376_ptr_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_AStruct376_ptr_1);
    }
    return in_AStruct376_ptr_1;
}

pub fn process_struct_1018_e5dc(in_struct_1: *mut AStruct65, param_2: u16, param_3: u32) {
    let mut uVar1: u16;
    let mut iVar2: i32;
    let mut unaff_BP: u16;
    let local_struct_1: *mut AStruct65;
    let ppVar3: *mut pass1_struct_1;

    load_cursor_1020_7f7a(in_struct_1, CONCAT22(param_2, 9), param_3);
    local_struct_1 = (in_struct_1 >> 0x10);
    iVar2 = in_struct_1;
    (iVar2 + 0xee) = 0;
    (iVar2 + 0xf2) = 0;
    in_struct_1.ptr_a_lo = 0xe790;
    (iVar2 + 2) = 0x1018;
    (iVar2 + 0xe2) = 0xe82c;
    (iVar2 + 0xe4) = 0x1018;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 10));
    uVar1 = (ppVar3 >> 0x10);
    (iVar2 + 0xf2) = ppVar3;
    (iVar2 + 0xf4) = uVar1;
    (iVar2 + 0xe6) = (iVar2 + 0xf2);
    (iVar2 + 0xe8) = uVar1;
    return;
}

pub fn process_struct_1018_e64c(param_1: *mut AStruct44) {
    let local_BX_3: *mut AStruct376;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    param_1.ptr_a_lo = 0xe790;
    local_BX_3.ptr_a_hi = 0x1018;
    local_BX_3.u16_xe2 = 0xe82c;
    local_BX_3.u16_xe4 = 0x1018;
    process_struct_1020_808e(param_1);
    return;
}

pub fn process_struct_1018_e76a(in_struct_1: *mut AStruct376, param_2: u8) -> *mut AStruct376 {
    process_struct_1018_e64c(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn process_struct_1018_e91e(in_struct_a: *mut AStruct65, param_2: u16, param_3: u32) {
    let mut u_var1: u32;
    let paVar2: *mut AStruct65;
    let puVar3: *mut u16;
    let mut uVar4: u16;
    let paVar5: *mut AStruct65;
    let ppVar6: *mut pass1_struct_1;
    let struct_a_1: *mut AStruct65;
    let struct_a_2: *mut AStruct65;
    let mut uStack14: u16;
    let char_ptr_1: *mut libc::c_char;
    // fn_ptr_1: *mut *mut u8;

    struct_a_1 = in_struct_a;
    struct_a_2 = (in_struct_a >> 0x10);
    paVar2 = struct_a_2;
    load_cursor_1020_7f7a(in_struct_a, CONCAT22(param_2, 3), param_3);
    &struct_a_1.u16_xee = 0;
    &struct_a_1.u16_xf2 = 0;
    &struct_a_1.field_0xf6 = 0;
    in_struct_a.ptr_a_lo = 0xebd0;
    struct_a_1.ptr_a_hi = 0x1018;
    struct_a_1.ptr_b_lo = 0xec6c;
    struct_a_1.ptr_b_hi = 0x1018;
    ppVar6 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(char_ptr_1, 0x28));
    uVar4 = (ppVar6 >> 0x10);
    struct_a_1.u16_xf2 = ppVar6;
    struct_a_1.u16_xf4 = uVar4;
    struct_a_1.u16_xe6 = struct_a_1.u16_xf2;
    struct_a_1.u16_xe8 = uVar4;
    ppVar6 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(char_ptr_1, 0x32));
    struct_a_1.field_0xf6 = ppVar6;
    struct_a_1.field_0xf8 = (ppVar6 >> 0x10);
    if (in_struct_a == 0x0) {
        puVar3 = 0x0;
        paVar5 = 0x0;
    } else {
        puVar3 = &struct_a_1.ptr_b_lo;
        paVar5 = struct_a_2;
    }
    uVar1 = &struct_a_1.field_0xf6;
    struct_a_2 = uVar1;
    uStack14 = (uVar1 >> 0x10);
    fn_ptr_1 = (&struct_a_1.field_0xf6 + 4);
    (**fn_ptr_1)(0x1010, struct_a_2, uStack14, 0xb, puVar3, paVar5);
    return;
}

pub fn process_struct_1020_04f6(in_struct_1: *mut AStruct629, param_2: u16) {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let local_struct_1: *mut AStruct629;
    let mut local_struct_1_hi: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut uVar6: u16;
    let local_1e: *mut libc::c_char;
    let pcStack32: *mut libc::c_char;
    let mut local_6: u16;
    let mut local_4: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.u16_x00 = s_1_1050_389a;
    local_struct_1.u16_x02 = &PTR_LOOP_1050_1008;
    in_struct_1.u16_x00 = (s_18_2_1050_3aa5 + 3);
    local_struct_1.u16_x02 = &PTR_LOOP_1050_1008;
    local_struct_1.u16_x04 = param_2;
    in_struct_1.u16_x00 = s_0_020_1050_3ab0;
    local_struct_1.u16_x02 = &PTR_LOOP_1050_1008;
    &local_struct_1.u16_x06 = 0;
    local_struct_1.u16_x0A = 0;
    local_struct_1.u16_x0C = 0;
    local_struct_1.u16_x0E = 0;
    local_struct_1.u16_x10 = 0;
    in_struct_1.u16_x00 = 0x75a;
    local_struct_1.u16_x02 = 0x1020;
    pcStack32 = CONCAT22(local_1e, 1);
    ppVar5 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, pcStack32);
    uVar3 = (ppVar5 >> 0x10);
    local_struct_1.u16_x06 = ppVar5;
    local_struct_1.u16_x08 = uVar3;
    uVar6 = local_struct_1.u16_x06;
    ppcVar1 = (&local_struct_1.u16_x06 + 4);
    (**ppcVar1)(0x1010, uVar6, uVar3, 0, in_struct_1, local_1e);
    ppVar5 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(uVar6, 0x48));
    uVar4 = (ppVar5 >> 0x10);
    iVar2 = ppVar5;
    local_struct_1.u16_x0A = (iVar2 + 10);
    local_struct_1.u16_x0C = (iVar2 + 0xc);
    pass1_1008_3e94(
        (iVar2 + 0xe),
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.u16_x10)),
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.u16_x0E)),
    );
    return;
}

pub fn process_struct_1020_0baa(in_struct_1: *mut AStruct636, param_2: u16) {
    let mut local_DX_123: u16;
    let local_struct_1: *mut AStruct636;
    let local_struct_1_hi: *mut AStruct636;
    let ppVar1: *mut pass1_struct_1;
    let p_uvar2: *mut u16;
    let puVar3: *mut u16;
    let paVar4: *mut AStruct636;
    let local_string_1: *mut libc::c_char;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.u16_0x0 = s_1_1050_389a;
    local_struct_1.u16_0x02 = &PTR_LOOP_1050_1008;
    in_struct_1.u16_0x0 = (s_18_2_1050_3aa5 + 3);
    local_struct_1.u16_0x02 = &PTR_LOOP_1050_1008;
    local_struct_1.u16_0x04 = param_2;
    in_struct_1.u16_0x0 = s_0_020_1050_3ab0;
    local_struct_1.u16_0x02 = &PTR_LOOP_1050_1008;
    &local_struct_1.u16_0x06 = 0;
    local_struct_1.u16_0x0a = 0;
    local_struct_1.u16_0x0c = 0;
    in_struct_1.u16_0x0 = 0xdbc;
    local_struct_1.u16_0x02 = 0x1020;
    ppVar1 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(local_string_1, 7));
    local_struct_1.u16_0x06 = ppVar1;
    local_struct_1.u16_0x08 = (ppVar1 >> 0x10);
    puVar3 = &local_struct_1.u16_0x0a;
    puVar2 = &local_struct_1.u16_0x0c;
    paVar4 = local_struct_1_hi;
    ppVar1 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(puVar2, 0x48));
    pass1_1008_3e94(
        (ppVar1 + 0xe),
        CONCAT22(local_struct_1_hi, puVar2),
        CONCAT22(paVar4, puVar3),
    );
    return;
}

pub fn process_struct_1040_a640(in_struct_1: *mut AStruct352, param_2: u32, param_3: u16) {
    let local_struct_1: *mut AStruct352;
    let mut local_ES_22: u16;

    process_struct_1040_b082(in_struct_1, CONCAT22(param_3, 0x1f1));
    local_ES_22 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.field_0x94 = param_2;
    local_struct_1.field_0x98 = 0;
    local_struct_1.field_0xea = 0;
    in_struct_1 = 0xac08;
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1040;
    return;
}

pub fn process_struct_1040_a598(param_1: *mut AStruct354) {
    let local_BX_3: *mut AStruct354;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    param_1 = 0;
    local_BX_3.field_0x2 = 0;
    local_BX_3.field_0x6 = 0;
    local_BX_3.field_0xa = 0;
    local_BX_3.field_0xc = 0;
    local_BX_3.field_0x10 = 0;
    local_BX_3.field_0x12 = 0;
    local_BX_3.field_0x14 = 0;
    local_BX_3.field_0x16 = 0;
    return;
}

pub fn process_struct_1040_9824(param_1: *mut u32) {
    let local_BX_3: *mut AStruct355;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_BX_3 = param_1;
    unsafe {
        *param_1 = 0;
    }
    local_BX_3.field_0x4 = 0;
    local_BX_3.field_0x56 = 0;
    local_BX_3.field_0x5a = 0;
    local_BX_3.field_0x5c = 0;
    *&local_BX_3.field_0x6 = 0;
    return;
}

pub fn process_struct_1040_9618(param_1: *mut AStruct357) {
    let mut uVar1: u16;
    let local_BX_4: *mut AStruct357;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    uVar3 = process_struct_1008_4772(local_BX_4.field_0x8);
    uVar1 = (uVar3 >> 0x10);
    local_BX_4.field_0x2a = (uVar3 + 4);
    local_BX_4.field_0x2c = (uVar3 + 8);
    return;
}

pub fn process_struct_1040_807e(param_1: u32, param_2: u16) {
    let mut uVar1: i32;
    let ppcVar2: fn();
    let in_ax: *mut u32;
    let puVar3: *mut u32;
    let puVar4: *mut u32;
    let mut extraout_DX: i32;
    let extraout_DX_00: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let extraout_DX_01: *mut AStruct199;
    let mut extraout_DX_02: u16;
    let mut uVar5: u16;
    let mut iVar6: i32;
    let mut uVar7: u16;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 1) {
        pass1_1040_805a();
        return;
    }
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, param_2);
    _local_6 = CONCAT22(extraout_DX, in_ax);
    if ((extraout_DX | in_ax) != 0) {
        ppcVar2 = (*_local_6 + 0x14);
        puVar3 = in_ax;
        ppcVar2(0x1010, in_ax, extraout_DX);
        uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        struct_a = extraout_DX_00;
        puVar4 = puVar3;
        if ((iVar6 + 0x70) != 0) {
            puVar4 = (iVar6 + 0x70);
            uVar1 = (iVar6 + 0x72);
            struct_a = (uVar1 | puVar4);
            if (struct_a != 0x0) {
                unsafe {
                    ppcVar2 = *puVar4;
                }
                ppcVar2(0x1010, puVar4, uVar1, 1);
                struct_a = extraout_DX_01;
            }
        }
        process_struct_1000_179c(0x14, struct_a);
        if ((struct_a | puVar4) == 0) {
            puVar4 = 0x0;
            uVar5 = 0;
        } else {
            process_struct_1008_4c58(CONCAT22(struct_a, puVar4));
            uVar5 = extraout_DX_02;
        }
        (iVar6 + 0x70) = puVar4;
        (iVar6 + 0x72) = uVar5;
        pass1_1008_4d84((iVar6 + 0x70), puVar3);
        if (_local_6 != 0x0) {
            ppcVar2 = *_local_6;
            ppcVar2(&PTR_LOOP_1050_1008, in_ax, extraout_DX, 1);
        }
        return;
    }
    return;
}

pub fn process_struct_1040_7728(
    in_struct_1: *mut AStruct68,
    param_2: *mut u8,
    param_3: u32,
    param_4: *mut u8,
    param_5: *mut u8,
) -> u8 {
    let iVar1: u16;
    let local_struct_1: *mut AStruct68;
    let local_struct_1_hi: *mut AStruct68;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.field_0x0 = s_1_1050_389a;
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1008;
    in_struct_1.field_0x0 = (s_18_2_1050_3aa5 + 3);
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1008;
    local_struct_1.field_0x4 = 0;
    local_struct_1.field_0x6 = 0;
    local_struct_1.field_0x8 = param_5;
    local_struct_1.field_0xa = param_4;
    local_struct_1.field_0xc = 0;
    local_struct_1.field_0x60 = 0;
    local_struct_1.field_0x62 = 0;
    local_struct_1.field_0x64 = 0;
    local_struct_1.field_0x66 = 0;
    local_struct_1.field_0x68 = 0;
    local_struct_1.field_0x6a = param_3;
    local_struct_1.field_0x6e = param_2;
    local_struct_1.field_0x70 = 0;
    local_struct_1.field_0x74 = 0;
    local_struct_1.field_0x76 = 0;
    local_struct_1.field_0x78 = 0;
    local_struct_1.field_0x8a = 0;
    local_struct_1.field_0x8c = 0;
    in_struct_1.field_0x0 = 0x840c;
    local_struct_1.field_0x2 = &PTR_LOOP_1050_1040;
    copy_string_1000_3d3e(
        (in_struct_1 & 0xffff0000 | &local_struct_1.field_0x10),
        0x10505db0,
    );
    pass1_1000_4906(
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.struct_ptr_0x7a)),
        0,
        8,
    );
    pass1_1000_4906(
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.struct_ptr_0x82)),
        0,
        8,
    );
    iVar1 = GetSystemMetrics16(4);
    local_struct_1.field_0x62 = iVar1;
    iVar1 = GetSystemMetrics16(5);
    local_struct_1.field_0x64 = iVar1;
    iVar1 = GetSystemMetrics16(6);
    local_struct_1.field_0x66 = iVar1;
    return in_struct_1;
}

pub fn pass1_1040_6fb6(param_1: *mut AStruct346, param_2: *mut u8) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    process_struct_1040_b0bc(param_1, 0, CONCAT22(param_2, 0xfd9));
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = 0;
    (iVar1 + 0x98) = 0;
    param_1 = 0x76a4;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    return;
}

pub fn pass1_1040_5626(param_1: *mut u16, param_2: *mut u8, param_3: u16) {
    let mut u_var1: u32;
    let mut uVar2: i32;
    let struct_a: *mut AStruct199;
    let paVar3: *mut AStruct199;
    let struct_a_00: *mut AStruct199;
    let extraout_DX: *mut AStruct199;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1040_b082(param_1, CONCAT22(param_3, 0xfa3));
    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar2 = 0;
    (iVar4 + 0x94) = 0;
    (iVar4 + 0x96) = 0;
    (iVar4 + 0x98) = 0;
    (iVar4 + 0x9c) = 0;
    unsafe {
        *param_1 = 0x6386;
    }
    (iVar4 + 2) = &PTR_LOOP_1050_1040;
    paVar3 = struct_a;
    process_struct_1000_179c(0x18, struct_a);
    struct_a_00 = (paVar3 | uVar2);
    if (struct_a_00 == 0x0) {
        (iVar4 + 0x90) = 0;
    } else {
        process_struct_1040_a598(CONCAT22(paVar3, uVar2));
        (iVar4 + 0x90) = uVar2;
        (iVar4 + 0x92) = extraout_DX;
        struct_a_00 = extraout_DX;
    }
    (iVar4 + 0x90) = 6;
    iVar5 = (iVar4 + 0x90);
    uVar2 = iVar5 * 10 + 2;
    process_struct_1000_179c(uVar2, struct_a_00);
    _local_c = CONCAT22(struct_a_00, uVar2);
    if ((struct_a_00 | uVar2) == 0) {
        uVar1 = (iVar4 + 0x90);
        (uVar1 + 2) = 0;
    } else {
        *_local_c = iVar5;
        call_fn_ptr_1000_5586(
            0xa564,
            &PTR_LOOP_1050_1040,
            iVar5,
            10,
            uVar2 + 2,
            struct_a_00,
        );
        uVar1 = (iVar4 + 0x90);
        uVar7 = (uVar1 >> 0x10);
        iVar5 = uVar1;
        (iVar5 + 2) = uVar2 + 2;
        (iVar5 + 4) = struct_a_00;
    }
    uVar1 = (iVar4 + 0x90);
    *(uVar1 + 6) = param_2;
    uVar1 = (iVar4 + 0x90);
    (uVar1 + 10) = 4;
    uVar1 = (iVar4 + 0x90);
    (uVar1 + 0x12) = (iVar4 + 10);
    uVar8 = pass1_1040_5d12(param_1);
    uVar2 = (uVar8 >> 0x10);
    if ((uVar2 | uVar8) == 0) {
        (iVar4 + 0x9a) = 0;
    } else {
        (iVar4 + 0x9a) = (uVar8 + 0x20);
    }
    return;
}

pub fn pass1_1040_44d2(param_1: *mut u16, param_2: *mut u8, param_3: *mut u8) {
    let mut u_var1: u32;
    let mut in_ax: i32;
    let mut uVar2: i32;
    let struct_a: *mut AStruct199;
    let paVar3: *mut AStruct199;
    let struct_a_00: *mut AStruct199;
    let extraout_DX: *mut AStruct199;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1040_b082(param_1, CONCAT22(param_3, 0xfa2));
    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    unsafe {
        *param_1 = 0x4824;
    }
    (iVar4 + 2) = &PTR_LOOP_1050_1040;
    paVar3 = struct_a;
    process_struct_1000_179c(0x18, struct_a);
    struct_a_00 = (paVar3 | in_ax);
    if (struct_a_00 == 0x0) {
        (iVar4 + 0x90) = 0;
    } else {
        process_struct_1040_a598(CONCAT22(paVar3, in_ax));
        (iVar4 + 0x90) = in_ax;
        (iVar4 + 0x92) = extraout_DX;
        struct_a_00 = extraout_DX;
    }
    (iVar4 + 0x90) = 0x14;
    iVar5 = (iVar4 + 0x90);
    uVar2 = iVar5 * 10 + 2;
    process_struct_1000_179c(uVar2, struct_a_00);
    _local_8 = CONCAT22(struct_a_00, uVar2);
    if ((struct_a_00 | uVar2) == 0) {
        uVar1 = (iVar4 + 0x90);
        (uVar1 + 2) = 0;
    } else {
        *_local_8 = iVar5;
        call_fn_ptr_1000_5586(
            0xa564,
            &PTR_LOOP_1050_1040,
            iVar5,
            10,
            uVar2 + 2,
            struct_a_00,
        );
        uVar1 = (iVar4 + 0x90);
        uVar7 = (uVar1 >> 0x10);
        iVar5 = uVar1;
        (iVar5 + 2) = uVar2 + 2;
        (iVar5 + 4) = struct_a_00;
    }
    uVar1 = (iVar4 + 0x90);
    *(uVar1 + 6) = param_2;
    uVar1 = (iVar4 + 0x90);
    (uVar1 + 10) = 1;
    uVar1 = (iVar4 + 0x90);
    (uVar1 + 0x12) = (iVar4 + 10);
    return;
}

pub fn pass1_1040_34a2(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        (s_You_may_not_run_a_turn__The_game_1050_0172 + 0x20),
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x94) = 0;
    (iVar1 + 0x96) = 0;
    (iVar1 + 0x98) = 0;
    param_1.field_0x0 = (s_Null_Ptr_1050_38f3 + 7);
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x3c));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_2ea2(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        (s_You_may_not_run_a_turn__The_game_1050_0172 + 0xe),
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x90) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x94) = 0;
    (iVar1 + 0x96) = 0;
    param_1.field_0x0 = 0x3436;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x3c));
    (iVar1 + 0x96) = ppVar3;
    (iVar1 + 0x98) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_1f5a(param_1: *mut AStruct68, param_2: *mut u8) {
    let piVar1: *mut i32;
    let mut uVar2: u16;
    let mut extraout_DX: u16;
    let mut iVar3: i32;
    let mut unaff_SI: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;
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

    process_struct_1040_7728(param_1, (&PTR_LOOP_1050_0000 + 1), 0, 0xfcf, param_2);
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar2 = 0;
    (iVar3 + 0x8e) = 0;
    (iVar3 + 0xa2) = 0;
    (iVar3 + 0xa6) = 0;
    param_1.field_0x0 = (s_alarm_m_1050_2377 + 7);
    (iVar3 + 2) = &PTR_LOOP_1050_1040;
    mixed_fn_1010_830a(_g_struct_73_1050_14cc, 0x1cc);
    (iVar3 + 0x8e) = uVar2;
    (iVar3 + 0x90) = extraout_DX;
    uVar5 = process_struct_1008_4772(CONCAT22(extraout_DX, (iVar3 + 0x8e)));
    uVar2 = (uVar5 >> 0x10);
    process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_SI, 0x48));
    _local_16 = CONCAT22((uVar5 + 8) + 10, 10);
    _local_12 = CONCAT22(0x1d6, (uVar5 + 4) + -10);
    (iVar3 + 0x92) = _local_16;
    (iVar3 + 0x96) = _local_12;
    (iVar3 + 0x9a) = _local_16;
    (iVar3 + 0x9e) = _local_12;
    piVar1 = (iVar3 + 0x9c);
    unsafe {
        *piVar1 = *piVar1 + 0x14;
    }
    process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(iVar3 + 0xa6, 0x2b));
    pass1_1010_0538();
    return;
}

pub fn pass1_1040_1cb4(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;
    let in_string_1: *mut libc::c_char;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        (s_You_may_not_run_a_turn__The_game_1050_00df + 9),
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    param_1.field_0x0 = (s_526_bmp_1050_1ee7 + 7);
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    in_string_1 = CONCAT22(unaff_BP, 2);
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, in_string_1);
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    ppVar3 = process_struct_1010_20ba(
        _g_AStruct372_1050_0ed0,
        CONCAT22((in_string_1 >> 0x10), 0x37),
    );
    (iVar1 + 0x92) = ppVar3;
    (iVar1 + 0x94) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_181c(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfbb,
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x94) = 0;
    param_1.field_0x0 = (s_202_flc_1050_1c46 + 2);
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 2));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_123e(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) -> *mut AStruct68 {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfd1,
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1.field_0x0 = 0x17b0;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x46));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return param_1;
}

pub fn pass1_1040_0e1c(
    param_1: *mut AStruct68,
    param_2: *mut u8,
    param_3: *mut u8,
    param_4: *mut u8,
) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        0,
        (s_OpWnd__getKid__Unknown_target_mo_1050_01a3 + 0x1d),
        param_4,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    *(iVar1 + 0x92) = param_3;
    (iVar1 + 0x96) = 0;
    *(iVar1 + 0x98) = param_2;
    param_1.field_0x0 = (s_overflow_on_node__d_1050_11ca + 8);
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x3a));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1040_0bfc(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) -> *mut AStruct68 {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfcd,
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1.field_0x0 = 0xdb0;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x39));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    (iVar1 + 0x74) = 1;
    return param_1;
}

pub fn pass1_1040_0a1a(param_1: *mut u8) {
    let mut uVar1: i32;
    let puVar2: *mut u32;
    let ppcVar3: fn();
    let mut uVar4: u32;
    let puVar5: *mut u32;
    let puVar6: *mut u32;
    let extraout_DX: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let extraout_DX_00: *mut AStruct199;
    let mut extraout_DX_01: u16;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar9 = (param_1 >> 0x10);
    iVar7 = param_1;
    uVar4 = (iVar7 + 0x8e);
    uVar10 = (uVar4 >> 0x10);
    iVar8 = uVar4;
    puVar2 = (iVar8 + 10);
    local_6._0_2_ = puVar2;
    puVar5 = ((iVar8 + 0xc) | local_6);
    if (puVar5 == 0x0) {
        return;
    }
    unsafe {
        ppcVar3 = (*puVar2 + 0x14);
    }
    (**ppcVar3)();
    struct_a = extraout_DX;
    puVar6 = puVar5;
    if ((iVar7 + 0x70) != 0) {
        puVar6 = (iVar7 + 0x70);
        uVar1 = (iVar7 + 0x72);
        struct_a = (uVar1 | puVar6);
        if (struct_a != 0x0) {
            unsafe {
                ppcVar3 = *puVar6;
            }
            (**ppcVar3)();
            struct_a = extraout_DX_00;
        }
    }
    process_struct_1000_179c(0x14, struct_a);
    if ((struct_a | puVar6) == 0) {
        puVar6 = 0x0;
        uVar10 = 0;
    } else {
        process_struct_1008_4c58(CONCAT22(struct_a, puVar6));
        uVar10 = extraout_DX_01;
    }
    (iVar7 + 0x70) = puVar6;
    (iVar7 + 0x72) = uVar10;
    pass1_1008_4d84((iVar7 + 0x70), puVar5);
    return;
}

pub fn pass1_1040_06e8(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) -> *mut AStruct68 {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfbc,
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1.field_0x0 = 0xb90;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 7));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return param_1;
}

pub fn pass1_1038_eeda(param_1: *mut AStruct68, param_2: *mut u8) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        0,
        (s_notepad_read_me_1050_0162 + 4),
        param_2,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x94) = 0;
    param_1.field_0x0 = 0x67c;
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 9));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    (iVar1 + 0x74) = 1;
    return;
}

pub fn pass1_1038_eb9e(param_1: *mut AStruct68, param_2: *mut u8) -> *mut AStruct68 {
    let mut uVar1: u16;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        0,
        (s_OpWnd__getKid__Unknown_target_mo_1050_01a3 + 0x24),
        param_2,
    );
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x8e) = 0;
    param_1.field_0x0 = 0xee6e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return param_1;
}

pub fn pass1_1038_e99a(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) -> *mut AStruct68 {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfb9,
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1.field_0x0 = 0xeb32;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x30));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return param_1;
}

pub fn pass1_1038_e69a(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfcb,
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    param_1.field_0x0 = 0xe92e;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x43));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1038_e2d0(param_1: *mut AStruct68, param_2: *mut u8) -> *mut AStruct68 {
    let mut uVar1: u16;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        0,
        (s_OpWnd__getKid__Unknown_target_mo_1050_01a3 + 0x20),
        param_2,
    );
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x8e) = 0;
    param_1.field_0x0 = 0xe62e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return param_1;
}

pub fn pass1_1038_e140(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) -> *mut AStruct68 {
    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfc2,
        param_3_00,
    );
    param_1.field_0x0 = 0xe264;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return param_1;
}

pub fn pass1_1038_df86(param_1: *mut u8) {
    let mut u_var1: u32;
    let ppcVar2: fn();
    let BVar3: bool;
    let BVar4: bool;
    let puVar5: *mut u16;
    let struct_a: *mut AStruct199;
    let paVar6: *mut AStruct199;
    let mut uVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let ppVar11: *mut pass1_struct_1;
    let mut in_stack_0000ffe6: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar11 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(in_stack_0000ffe6, 2));
    uVar1 = (ppVar11 + 0x68);
    uVar9 = (param_1 >> 0x10);
    iVar8 = param_1;
    BVar3 = pass1_1010_041a();
    if (BVar3 != 0) {
        pass1_1010_038e((iVar8 + 0x92), 1);
        pass1_1038_af40(_g_AStruct112_a, *(iVar8 + 8), 0x1e);
        return;
    }
    load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x7d5,
    );
    uVar10 = 0x1000;
    paVar6 = struct_a;
    BVar4 = BVar3;
    process_struct_1000_179c(0xb4, struct_a);
    uVar7 = paVar6 | BVar4;
    if (uVar7 == 0) {
        uVar9 = 0;
        uVar7 = 0;
    } else {
        uVar10 = SUB42(&PTR_LOOP_1050_1040, 0);
        puVar5 = process_struct_1040_8478(
            CONCAT22(paVar6, BVar4),
            0x20,
            uVar1,
            CONCAT22(struct_a, BVar3),
            (iVar8 + 6),
        );
        uVar9 = SUB42(puVar5, 0);
    }
    _local_16 = CONCAT22(uVar7, uVar9);
    ppcVar2 = (*_local_16 + 0x74);
    ppcVar2(uVar10, uVar9, uVar7);
    return;
}

pub fn pass1_1038_d756(param_1: *mut AStruct68, param_2: *mut u8) -> *mut AStruct68 {
    let ppcVar1: fn();
    let mut iVar2: i32;
    let mut unaff_BP: u16;
    let mut uvar3: u16;
    let ppVar4: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        0,
        (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0xb),
        param_2,
    );
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    (iVar2 + 0x8e) = 0;
    (iVar2 + 0x90) = 0;
    (iVar2 + 0x92) = 0;
    (iVar2 + 0x96) = 0;
    param_1.field_0x0 = 0xe0d4;
    (iVar2 + 2) = &PTR_LOOP_1050_1038;
    ppVar4 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x2b));
    (iVar2 + 0x92) = ppVar4;
    (iVar2 + 0x94) = (ppVar4 >> 0x10);
    ppcVar1 = ((iVar2 + 0x92) + 4);
    (**ppcVar1)();
    return param_1;
}

pub fn pass1_1038_d242(param_1: *mut AStruct68, param_2: *mut u8) -> *mut AStruct68 {
    let mut uVar1: u16;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        0,
        (s_New_failed_in_Op__Op__Simulator_1050_0130 + 0xe),
        param_2,
    );
    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xd6ea;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    (param_1 + 0x74) = 1;
    return param_1;
}

pub fn pass1_1038_cd06(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfcc,
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    param_1.field_0x0 = 0xcf00;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x42));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1038_cad8(param_1: u32, param_2: *mut u8) -> u8 {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        0,
        (s_Unsupported_FileStructType_in_Op_1050_01ca + 1),
        param_2,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1 = 0xcc9a;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x2c));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    (iVar1 + 0x74) = 0;
    return param_1;
}

pub fn pass1_1038_c7b8(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) -> *mut AStruct68 {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfb8,
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    param_1.field_0x0 = 0xca6c;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 5));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return param_1;
}

pub fn pass1_1038_c4a2(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        (s_You_may_not_run_a_turn__The_game_1050_0172 + 10),
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x96) = 0;
    param_1.field_0x0 = 0xc74c;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x3b));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1038_bddc(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        (s_You_may_not_run_a_turn__The_game_1050_0172 + 4),
        param_3_00,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x94) = 0;
    (iVar1 + 0x96) = 0;
    (iVar1 + 0x98) = 0;
    (iVar1 + 0x9a) = 0;
    (iVar1 + 0x9c) = 0;
    param_1.field_0x0 = 0xc436;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x3b));
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1038_b772(param_1: *mut AStruct68) {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;
    let in_stack_00000010: *mut u8;
    let in_string_1: *mut libc::c_char;

    process_struct_1040_7728(
        param_1,
        (s_New_failed_in_Op__Op__Simulator_1050_0097 + 3),
        0,
        0xfbf,
        in_stack_00000010,
    );
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0;
    (iVar1 + 0x92) = 0;
    (iVar1 + 0x96) = 1;
    (iVar1 + 0x98) = 0;
    param_1.field_0x0 = 0xbd70;
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    in_string_1 = CONCAT22(unaff_BP, 0x36);
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, in_string_1);
    (iVar1 + 0x8e) = ppVar3;
    (iVar1 + 0x90) = (ppVar3 >> 0x10);
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22((in_string_1 >> 0x10), 6));
    (iVar1 + 0x92) = ppVar3;
    (iVar1 + 0x94) = (ppVar3 >> 0x10);
    return;
}

pub fn pass1_1038_ab82(param_1: *mut AStruct68, param_2: *mut u8) -> *mut AStruct68 {
    process_struct_1040_7728(param_1, (&PTR_LOOP_1050_0000 + 1), 0, 0xfd3, param_2);
    param_1.field_0x0 = 0xad72;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return param_1;
}

pub fn pass1_1038_a122(
    param_1: i32,
    param_2: *mut u8,
    param_3: *mut u8,
    param_3_00: u32,
    param_5: u32,
) -> *mut *mut u8 {
    let local_e: *mut u8;
    let puStack6: *mut u8;
    let puStack4: *mut u8;

    puStack6 = param_5;
    puStack4 = (param_5 >> 0x10);
    process_struct_1040_7728(
        CONCAT22(param_2, param_1),
        param_3,
        param_3_00,
        puStack6,
        puStack4,
    );
    (param_1 + 0x8e) = 0;
    *CONCAT22(param_2, param_1) = 0xa2d0;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1038_9f76(
    param_1: *mut AStruct68,
    param_2: u32,
    param_3: *mut u8,
    param_4: *mut u8,
    param_3_00: *mut u8,
) -> *mut AStruct68 {
    process_struct_1040_7728(
        param_1,
        (&PTR_LOOP_1050_0000 + 1),
        param_2,
        0xfba,
        param_3_00,
    );
    param_1.field_0x0 = 0xa0b6;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return param_1;
}

pub fn pass1_1038_9144(param_1: *mut u8, param_2: *mut u8) {
    let mut u_var1: u32;
    let mut uVar2: i32;
    let struct_a: *mut AStruct199;
    let struct_a_00: *mut AStruct199;
    let extraout_DX: *mut AStruct199;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut unaff_SI: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let ppVar7: *mut pass1_struct_1;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 0xfaa));
    uVar5 = (param_1 >> 0x10);
    iVar3 = param_1;
    (iVar3 + 0x94) = 0;
    (iVar3 + 0x96) = 0;
    (iVar3 + 0x98) = 0;
    param_1 = 0x99a2;
    (iVar3 + 2) = &PTR_LOOP_1050_1038;
    (iVar3 + 0x8a) = 0x27;
    ppVar7 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_SI, 0x28));
    struct_a = (ppVar7 >> 0x10);
    uVar2 = ppVar7;
    (iVar3 + 0x98) = uVar2;
    (iVar3 + 0x9a) = struct_a;
    process_struct_1000_179c(0x18, struct_a);
    struct_a_00 = (struct_a | uVar2);
    if (struct_a_00 == 0x0) {
        (iVar3 + 0x90) = 0;
    } else {
        process_struct_1040_a598(CONCAT22(struct_a, uVar2));
        (iVar3 + 0x90) = uVar2;
        (iVar3 + 0x92) = extraout_DX;
        struct_a_00 = extraout_DX;
    }
    (iVar3 + 0x90) = 0x11;
    iVar4 = (iVar3 + 0x90);
    uVar2 = iVar4 * 10 + 2;
    process_struct_1000_179c(uVar2, struct_a_00);
    _local_8 = CONCAT22(struct_a_00, uVar2);
    if ((struct_a_00 | uVar2) == 0) {
        uVar1 = (iVar3 + 0x90);
        (uVar1 + 2) = 0;
    } else {
        *_local_8 = iVar4;
        call_fn_ptr_1000_5586(
            0xa564,
            &PTR_LOOP_1050_1040,
            iVar4,
            10,
            uVar2 + 2,
            struct_a_00,
        );
        uVar1 = (iVar3 + 0x90);
        uVar6 = (uVar1 >> 0x10);
        iVar4 = uVar1;
        (iVar4 + 2) = uVar2 + 2;
        (iVar4 + 4) = struct_a_00;
    }
    uVar1 = (iVar3 + 0x90);
    (uVar1 + 10) = 0x18;
    uVar1 = (iVar3 + 0x90);
    (uVar1 + 0x12) = (iVar3 + 10);
    return;
}

pub fn pass1_1038_9124(param_1: *mut AStruct44, param_2: *mut u8) {
    let pbVar1: *mut u8;
    let mut bVar2: u8;
    let mut cVar3: u8;
    let mut uVar4: u32;
    let mut in_AL: u8;
    let mut uVar5: i32;
    let mut bVar6: u8;
    let mut in_CL: u8;
    let mut in_dx: i32;
    let mut bVar7: u8;
    let struct_a: *mut AStruct199;
    let struct_a_00: *mut AStruct199;
    let extraout_DX: *mut AStruct199;
    let mut in_BX: i32;
    let mut bVar10: u8;
    let mut iVar8: i32;
    let mut iVar9: i32;
    let puVar11: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut u8;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar14: bool;
    let mut bVar15: bool;
    let ppVar16: *mut pass1_struct_1;
    let local_4e: u8;
    let piStack8: *mut i32;

    puVar11 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar11 = puVar11 + -1;
        unsafe {
            *puVar11 = *unaff_BP;
        }
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    bVar10 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar10;
    bVar7 = ((in_dx + 1) >> 8);
    bVar14 = CARRY1(bVar7, bVar10) || CARRY1(bVar7 + bVar10, in_CF);
    bVar6 = (in_dx + 1);
    pbVar1 = unaff_SI + in_BX;
    unsafe {
        bVar7 = *pbVar1;
        bVar2 = *pbVar1 - bVar6;
        bVar15 = *pbVar1 < bVar6 || bVar2 < bVar14;
        *pbVar1 = bVar2 - bVar14;
        if (*pbVar1 != 0 && (SBORROW1(bVar7, bVar6) != SBORROW1(bVar2, bVar14)) == (*pbVar1 < '\0'))
        {
            pbVar1 = unaff_SI;
            bVar7 = *pbVar1;
            bVar2 = *pbVar1;
            *pbVar1 = bVar2 + bVar10 + bVar15;
            bVar14 = CARRY1(local_4e, in_BX)
                || CARRY1(
                    local_4e + in_BX,
                    CARRY1(bVar7, bVar10) || CARRY1(bVar2 + bVar10, bVar15),
                );
            pbVar1 = unaff_SI + -0x4f;
            bVar15 = CARRY1(*pbVar1, bVar10) || CARRY1(*pbVar1 + bVar10, bVar14);
            *pbVar1 = *pbVar1 + bVar10 + bVar14;
            uVar5 = CONCAT11(0x40, in_AL + 0x1) + 2;
            pbVar1 = &stack0xfffe + unaff_SI;
            bVar7 = *pbVar1;
            bVar6 = (uVar5 >> 8);
            bVar2 = *pbVar1 + bVar6;
            *pbVar1 = bVar2 + bVar15;
            pbVar1 = unaff_SI + in_BX;
            *pbVar1 = *pbVar1 + uVar5 + in_CL + (CARRY1(bVar7, bVar6) || CARRY1(bVar2, bVar15));
            process_struct_1040_b082(param_1, CONCAT22(param_2, 0xfaa));
            uVar12 = (param_1 >> 0x10);
            iVar8 = param_1;
            (iVar8 + 0x94) = 0;
            (iVar8 + 0x96) = 0;
            (iVar8 + 0x98) = 0;
            param_1.ptr_a_lo = 0x99a2;
            (iVar8 + 2) = &PTR_LOOP_1050_1038;
            (iVar8 + 0x8a) = 0x27;
            ppVar16 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_SI, 0x28));
            struct_a = (ppVar16 >> 0x10);
            uVar5 = ppVar16;
            (iVar8 + 0x98) = uVar5;
            (iVar8 + 0x9a) = struct_a;
            process_struct_1000_179c(0x18, struct_a);
            struct_a_00 = (struct_a | uVar5);
            if (struct_a_00 == 0x0) {
                (iVar8 + 0x90) = 0;
            } else {
                process_struct_1040_a598(CONCAT22(struct_a, uVar5));
                (iVar8 + 0x90) = uVar5;
                (iVar8 + 0x92) = extraout_DX;
                struct_a_00 = extraout_DX;
            }
            (iVar8 + 0x90) = 0x11;
            iVar9 = (iVar8 + 0x90);
            uVar5 = iVar9 * 10 + 2;
            process_struct_1000_179c(uVar5, struct_a_00);
            piStack8 = CONCAT22(struct_a_00, uVar5);
            if ((struct_a_00 | uVar5) == 0) {
                uVar4 = (iVar8 + 0x90);
                (uVar4 + 2) = 0;
            } else {
                *piStack8 = iVar9;
                call_fn_ptr_1000_5586(
                    0xa564,
                    &PTR_LOOP_1050_1040,
                    iVar9,
                    10,
                    uVar5 + 2,
                    struct_a_00,
                );
                uVar4 = (iVar8 + 0x90);
                uVar13 = (uVar4 >> 0x10);
                iVar9 = uVar4;
                (iVar9 + 2) = uVar5 + 2;
                (iVar9 + 4) = struct_a_00;
            }
            uVar4 = (iVar8 + 0x90);
            (uVar4 + 10) = 0x18;
            uVar4 = (iVar8 + 0x90);
            (uVar4 + 0x12) = (iVar8 + 10);
            return;
        }
    }
    if (unsafe { *pbVar1 != 0 }) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub fn pass1_1038_8caa(param_1: *mut *mut u8, param_2: *mut u8) -> *mut *mut u8 {
    let mut iVar1: i32;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let ppVar3: *mut pass1_struct_1;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 0x185a));
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = 0;
    unsafe {
        *param_1 = 0x90c8;
    }
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    ppVar3 = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x3f));
    (iVar1 + 0x94) = ppVar3;
    (iVar1 + 0x96) = (ppVar3 >> 0x10);
    return param_1;
}

pub fn pass1_1038_88f2(param_1: *mut *mut u8, param_2: *mut u8) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 0x184c));
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = _PTR_LOOP_1050_5a68;
    (iVar1 + 0x98) = 0;
    (iVar1 + 0x9a) = 0;
    (iVar1 + 0x9c) = 0;
    (iVar1 + 0x9e) = 0;
    unsafe {
        *param_1 = 0x8c2e;
    }
    (iVar1 + 2) = &PTR_LOOP_1050_1038;
    return;
}

pub fn pass1_1038_88d2(param_1: *mut AStruct44) {
    let pbVar1: *mut u8;
    let mut bVar2: u8;
    let mut cVar3: u8;
    let mut bVar4: u8;
    let mut in_CL: u8;
    let mut in_dx: i32;
    let mut bVar5: u8;
    let mut in_BX: i32;
    let mut bVar7: u8;
    let mut iVar6: i32;
    let puVar8: *mut u16;
    let unaff_BP: *mut u16;
    let unaff_SI: *mut u8;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let mut in_CF: u8;
    let mut bVar10: bool;
    let mut bVar11: bool;
    let puVar12: *mut u16;
    let local_4e: u8;
    let puStack34: *mut u8;

    puStack34 = &stack0xfffe;
    puVar8 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_BP = unaff_BP + -1;
        puVar8 = puVar8 + -1;
        unsafe {
            *puVar8 = *unaff_BP;
        }
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    bVar7 = (in_BX >> 8);
    unaff_SI[in_BX] = bVar7;
    bVar5 = ((in_dx + 1) >> 8);
    bVar10 = CARRY1(bVar5, bVar7) || CARRY1(bVar5 + bVar7, in_CF);
    bVar4 = (in_dx + 1);
    puVar12 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pbVar1 = unaff_SI + in_BX;
    unsafe {
        bVar5 = *pbVar1;
        bVar2 = *pbVar1 - bVar4;
        bVar11 = *pbVar1 < bVar4 || bVar2 < bVar10;
        *pbVar1 = bVar2 - bVar10;
        if (*pbVar1 != 0 && (SBORROW1(bVar5, bVar4) != SBORROW1(bVar2, bVar10)) == (*pbVar1 < '\0'))
        {
            pbVar1 = unaff_SI;
            bVar10 = CARRY1(*pbVar1, bVar7) || CARRY1(*pbVar1 + bVar7, bVar11);
            *pbVar1 = *pbVar1 + bVar7 + bVar11;
            bVar5 = local_4e + in_BX;
            bVar11 = CARRY1(local_4e, in_BX) || CARRY1(bVar5, bVar10);
            local_4e = bVar5 + bVar10;
            pbVar1 = unaff_SI + -0x4f;
            bVar5 = *pbVar1;
            bVar2 = *pbVar1;
            *pbVar1 = bVar2 + bVar7 + bVar11;
            pbVar1 = unaff_SI + -0x78;
            *pbVar1 = *pbVar1 + in_CL + (CARRY1(bVar5, bVar7) || CARRY1(bVar2 + bVar7, bVar11));
            puStack34 = &stack0xfffe;
            process_struct_1040_b082(puVar12, CONCAT22(in_BX, 0x184c));
            uVar9 = (puVar12 >> 0x10);
            iVar6 = puVar12;
            (iVar6 + 0x94) = _PTR_LOOP_1050_5a68;
            (iVar6 + 0x98) = 0;
            (iVar6 + 0x9a) = 0;
            (iVar6 + 0x9c) = 0;
            (iVar6 + 0x9e) = 0;
            *puVar12 = 0x8c2e;
            (iVar6 + 2) = &PTR_LOOP_1050_1038;
            return;
        }
        if (*pbVar1 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return;
}

pub fn pass1_1038_7d10(param_1: u32, param_2: *mut u8) {
    let local_BX_18: *mut AStruct1172;
    let mut unaff_BP: u16;
    let mut uVar1: u16;
    let struct_a: *mut pass1_struct_1;

    process_struct_1040_b082(param_1, CONCAT22(param_2, 0x1853));
    uVar1 = (param_1 >> 0x10);
    local_BX_18 = param_1;
    local_BX_18.field_0x94 = 0x0;
    param_1 = 0x8876;
    local_BX_18.field_0x2 = &PTR_LOOP_1050_1038;
    struct_a = process_struct_1010_20ba(_g_AStruct372_1050_0ed0, CONCAT22(unaff_BP, 0x40));
    &local_BX_18.field_0x94 = struct_a;
    (&local_BX_18.field_0x94 + 2) = (struct_a >> 0x10);
    return param_1;
}

pub fn pass1_1038_78e2(param_1: *mut *mut u8) {
    let paVar1: *mut AStruct199;
    let in_dx: *mut AStruct199;
    let struct_a: *mut AStruct199;
    let mut uVar2: i32;
    let local_BX_4: *mut AStruct1169;
    let mut uvar3: u16;
    let mut local_4: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    paVar1 = 0x0;
    unsafe {
        *param_1 = 0x0;
    }
    &local_BX_4.field_0x4 = 0;
    _PTR_LOOP_1050_5a64 = param_1;
    process_struct_1000_179c(0xc, in_dx);
    struct_a = (in_dx | paVar1);
    if (struct_a == 0x0) {
        unsafe {
            *param_1 = 0x0;
        }
    } else {
        paVar1 = process_struct_1008_574a(CONCAT22(in_dx, paVar1));
        param_1 = paVar1;
        local_BX_4.field_0x2 = struct_a;
    }
    process_struct_1000_179c(0xc, struct_a);
    uVar2 = struct_a | paVar1;
    if (uVar2 == 0) {
        paVar1 = 0x0;
        uVar2 = 0;
    } else {
        paVar1 = process_struct_1008_574a(CONCAT22(struct_a, paVar1));
    }
    local_BX_4.field_0x4 = paVar1;
    local_BX_4.field_0x6 = uVar2;
    return;
}

pub fn pass1_1038_6520(param_1: *mut AStruct1144) {
    let local_BX_4: *mut AStruct1144;
    let mut local_ES_4: u16;

    local_ES_4 = (param_1 >> 0x10);
    local_BX_4 = param_1;
    param_1.field_0x0 = s_1_1050_389a;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1008;
    local_BX_4.field_0x4 = 0x0;
    local_BX_4.field_0x8 = 0x0;
    local_BX_4.field_0xc = 0;
    local_BX_4.field_0xe = 0x0;
    local_BX_4.field_0x12 = 0;
    local_BX_4.field_0x14 = 0;
    local_BX_4.field_0x16 = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_BX_4.field_0x1a));
    local_BX_4.field_0x20 = 0;
    local_BX_4.field_0x24 = 0;
    local_BX_4.field_0x26 = 0;
    local_BX_4.field_0x28 = 0;
    param_1.field_0x0 = 0x78de;
    local_BX_4.field_0x2 = &PTR_LOOP_1050_1038;
    return;
}

pub fn pass1_1038_2b9a(param_1: *mut u8) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut in_ax: i32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let in_dx: *mut AStruct199;
    let mut iVar5: i32;
    let puVar6: *mut u32;
    let mut uVar7: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x118, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    iVar5 = param_1;
    uVar7 = (param_1 >> 0x10);
    if ((in_dx | in_ax) != 0) {
        *_local_a = s_1_1050_389a;
        (in_ax + 2) = &PTR_LOOP_1050_1008;
        (in_ax + 4) = (iVar5 + 4);
        puVar3 = (iVar5 + 8);
        puVar6 = (in_ax + 8);
        iVar4 = 0x40;
        while (iVar4 != 0) {
            iVar4 = iVar4 + -1;
            puVar2 = puVar6;
            puVar6 = puVar6 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe {
                *puVar2 = *puVar1;
            }
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        (in_ax + 0x108) = (iVar5 + 0x108);
        (in_ax + 0x10c) = (iVar5 + 0x10c);
        (in_ax + 0x110) = (iVar5 + 0x110);
        (in_ax + 0x114) = (iVar5 + 0x114);
        *_local_a = 0x309a;
        (in_ax + 2) = &PTR_LOOP_1050_1038;
    }
    (iVar5 + 0x114) = 0;
    (iVar5 + 0x110) = 0;
    return;
}

pub fn pass1_1038_2944(param_1: u32) {
    let paVar1: *mut AStruct1074;
    let paVar2: *mut AStruct1073;
    let local_AX__1: *mut AStruct1071;
    let local_CX_80: *mut AStruct1072;
    let in_dx: *mut AStruct199;
    let local_SI_80: *mut AStruct1074;
    let local_DI_80: *mut AStruct1073;
    let mut uvar3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, local_AX__1);
    if ((in_dx | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar3 = (param_1 >> 0x10);
        local_AX__1.field_0x4 = (param_1 + 4);
        local_SI_80 = (param_1 + 8);
        local_DI_80 = &local_AX__1.field_0x8;
        local_CX_80 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_80 != 0x0) {
            local_CX_80 = local_CX_80 + -1;
            paVar2 = local_DI_80;
            local_DI_80 = &local_DI_80.field_0x4;
            paVar1 = local_SI_80;
            local_SI_80 = &local_SI_80.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = (s_fem110_wav_1050_29fa + 4);
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1038;
    }
    return;
}

pub fn pass1_1038_0cf0(param_1: *mut AStruct1054) {
    let paVar1: *mut AStruct1057;
    let paVar2: *mut AStruct1056;
    let local_AX__1: *mut AStruct1055;
    let mut iVar3: i32;
    let in_dx: *mut AStruct199;
    let local_BX_41: *mut AStruct1054;
    let local_SI_80: *mut AStruct1057;
    let local_DI_80: *mut AStruct1056;
    let local_ES_41: *mut AStruct1054;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x10c, in_dx);
    _local_a = CONCAT22(in_dx, local_AX__1);
    if ((in_dx | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        local_ES_41 = (param_1 >> 0x10);
        local_BX_41 = param_1;
        local_AX__1.field_0x4 = local_BX_41.field_0x4;
        local_SI_80 = &local_BX_41.field_0x8;
        local_DI_80 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            paVar2 = local_DI_80;
            local_DI_80 = &local_DI_80.field_0x4;
            paVar1 = local_SI_80;
            local_SI_80 = &local_SI_80.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_41.field_0x108;
        *_local_a = s_198_flc_1050_1c2e;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1038;
    }
    return;
}

pub fn pass1_1038_0000(param_1: *mut u8) {
    let paVar1: *mut AStruct1049;
    let paVar2: *mut AStruct1048;
    let local_AX__1: *mut AStruct1046;
    let local_CX_80: *mut AStruct1047;
    let in_dx: *mut AStruct199;
    let local_SI_80: *mut AStruct1049;
    let local_DI_80: *mut AStruct1048;
    let mut uvar3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    // Segment:    8
    // Offset:     000606c0
    // Length:     ef91
    // Min Alloc:  ef91
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, local_AX__1);
    if ((in_dx | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar3 = (param_1 >> 0x10);
        local_AX__1.field_0x4 = (param_1 + 4);
        local_SI_80 = (param_1 + 8);
        local_DI_80 = &local_AX__1.field_0x8;
        local_CX_80 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_80 != 0x0) {
            local_CX_80 = local_CX_80 + -1;
            paVar2 = local_DI_80;
            local_DI_80 = &local_DI_80.field_0x4;
            paVar1 = local_SI_80;
            local_SI_80 = &local_SI_80.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xb96;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1038;
    }
    return;
}

pub fn pass1_1030_ebf8(param_1: *mut u8) {
    let paVar1: *mut AStruct1042;
    let paVar2: *mut AStruct1043;
    let local_AX__1: *mut AStruct1044;
    let mut iVar3: i32;
    let in_dx: *mut AStruct199;
    let local_SI_80: *mut AStruct1042;
    let local_DI_80: *mut AStruct1043;
    let mut uVar4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, local_AX__1);
    if ((in_dx | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar4 = (param_1 >> 0x10);
        local_AX__1.field_0x4 = (param_1 + 4);
        local_SI_80 = (param_1 + 8);
        local_DI_80 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            paVar2 = local_DI_80;
            local_DI_80 = &local_DI_80.field_0x4;
            paVar1 = local_SI_80;
            local_SI_80 = &local_SI_80.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xecb2;
        local_AX__1.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e98e(param_1: *mut AStruct1037) {
    let paVar1: *mut AStruct1041;
    let paVar2: *mut AStruct1040;
    let local_AX__1: *mut AStruct1038;
    let local_CX_82: *mut AStruct1039;
    let in_dx: *mut AStruct199;
    let local_BX_43: *mut AStruct1037;
    let local_SI_82: *mut AStruct1041;
    let local_DI_82: *mut AStruct1040;
    let local_ES_43: *mut AStruct1037;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x112, in_dx);
    _local_a = CONCAT22(in_dx, local_AX__1);
    if ((in_dx | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        local_ES_43 = (param_1 >> 0x10);
        local_BX_43 = param_1;
        local_AX__1.field_0x4 = local_BX_43.field_0x4;
        local_SI_82 = &local_BX_43.field_0x8;
        local_DI_82 = &local_AX__1.field_0x8;
        local_CX_82 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_82 != 0x0) {
            local_CX_82 = local_CX_82 + -1;
            paVar2 = local_DI_82;
            local_DI_82 = &local_DI_82.field_0x4;
            paVar1 = local_SI_82;
            local_SI_82 = &local_SI_82.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_43.field_0x108;
        local_AX__1.field_0x10c = local_BX_43.field_0x10c;
        local_AX__1.field_0x110 = local_BX_43.field_0x110;
        *_local_a = 0xeb40;
        local_AX__1.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e7d6(param_1: *mut u8) {
    let paVar1: *mut AStruct1034;
    let paVar2: *mut AStruct1033;
    let local_AX__1: *mut AStruct1031;
    let local_CX_80: *mut AStruct1032;
    let in_dx: *mut AStruct199;
    let local_SI_80: *mut AStruct1034;
    let local_DI_80: *mut AStruct1033;
    let mut uvar3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, local_AX__1);
    if ((in_dx | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar3 = (param_1 >> 0x10);
        local_AX__1.field_0x4 = (param_1 + 4);
        local_SI_80 = (param_1 + 8);
        local_DI_80 = &local_AX__1.field_0x8;
        local_CX_80 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_80 != 0x0) {
            local_CX_80 = local_CX_80 + -1;
            paVar2 = local_DI_80;
            local_DI_80 = &local_DI_80.field_0x4;
            paVar1 = local_SI_80;
            local_SI_80 = &local_SI_80.field_0x4;
            paVar2.field_0x0 = paVar1.field_0x0;
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xe890;
        local_AX__1.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e6c2(param_1: *mut AStruct1030) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut AStruct1029;
    let mut iVar3: i32;
    let in_dx: *mut AStruct199;
    let local_BX_41: *mut AStruct1030;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut uVar6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x10a, in_dx);
    _local_a = CONCAT22(in_dx, local_AX__1);
    if ((in_dx | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar6 = (param_1 >> 0x10);
        local_BX_41 = param_1;
        local_AX__1.field_0x4 = local_BX_41.field_0x4;
        puVar4 = &local_BX_41.field_0x8;
        puVar5 = &local_AX__1.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 1;
            unsafe {
                *puVar2 = *puVar1;
            }
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_41.field_0x108;
        *_local_a = 0xe78a;
        local_AX__1.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e564(param_1: *mut AStruct1026) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_AX__1: *mut AStruct1027;
    let local_CX_80: *mut AStruct1025;
    let in_dx: *mut AStruct199;
    let local_BX_41: *mut AStruct1026;
    let puVar3: *mut u32;
    let puVar4: *mut u32;
    let mut uVar5: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x10c, in_dx);
    _local_a = CONCAT22(in_dx, local_AX__1);
    if ((in_dx | local_AX__1) != 0) {
        *_local_a = s_1_1050_389a;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1008;
        uVar5 = (param_1 >> 0x10);
        local_BX_41 = param_1;
        local_AX__1.field_0x4 = local_BX_41.field_0x4;
        puVar3 = &local_BX_41.field_0x8;
        puVar4 = &local_AX__1.field_0x8;
        local_CX_80 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_80 != 0x0) {
            local_CX_80 = local_CX_80 + -1;
            puVar2 = puVar4;
            puVar4 = puVar4 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe {
                *puVar2 = *puVar1;
            }
        }
        *_local_a = 0x6ad2;
        local_AX__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_AX__1.field_0x108 = local_BX_41.field_0x108;
        *_local_a = 0xe62e;
        local_AX__1.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e34e(struct_a: *mut AStruct1022) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let struct_b: *mut AStruct1023;
    let mut iVar3: i32;
    let struct_c: *mut AStruct199;
    let struct_d: *mut AStruct1022;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let struct_e: *mut AStruct1022;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x112, struct_c);
    _local_a = CONCAT22(struct_c, struct_b);
    if ((struct_c | struct_b) != 0) {
        *_local_a = s_1_1050_389a;
        struct_b.field_0x2 = &PTR_LOOP_1050_1008;
        struct_e = (struct_a >> 0x10);
        struct_d = struct_a;
        struct_b.field_0x4 = struct_d.field_0x4;
        puVar4 = &struct_d.field_0x8;
        puVar5 = &struct_b.field_0x8;
        iVar3 = 0x40;
        while (iVar3 != 0) {
            iVar3 = iVar3 + -1;
            puVar2 = puVar5;
            puVar5 = puVar5 + 1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 1;
            unsafe {
                *puVar2 = *puVar1;
            }
        }
        *_local_a = 0x6ad2;
        struct_b.field_0x2 = &PTR_LOOP_1050_1028;
        struct_b.field_0x108 = struct_d.field_0x108;
        struct_b.field_0x10c = struct_d.field_0x10c;
        struct_b.field_0x110 = struct_d.field_0x110;
        *_local_a = 0xe4ea;
        struct_b.field_0x2 = 0x1030;
    }
    return;
}

pub fn pass1_1030_e1f4(param_1: u32) {
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut in_ax: i32;
    let puVar3: *mut u32;
    let local_CX_80: *mut AStruct1019;
    let in_dx: *mut AStruct199;
    let puVar4: *mut u32;
    let mut uVar5: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) != 0) {
        *_local_a = s_1_1050_389a;
        (in_ax + 2) = &PTR_LOOP_1050_1008;
        uVar5 = (param_1 >> 0x10);
        (in_ax + 4) = (param_1 + 4);
        puVar3 = (param_1 + 8);
        puVar4 = (in_ax + 8);
        local_CX_80 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
        while (local_CX_80 != 0x0) {
            local_CX_80 = local_CX_80 + -1;
            puVar2 = puVar4;
            puVar4 = puVar4 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            unsafe {
                *puVar2 = *puVar1;
            }
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0xe2ae;
        (in_ax + 2) = 0x1030;
    }
    return;
}

pub fn pass1_1030_8c66(
    param_1: *mut AStruct104,
    param_2: u16,
    param_3: bool,
    param_4: u16,
    param_5: u32,
) {
    let mut bVar1: u8;
    let mut uVar2: i32;
    let mut extraout_DX: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1008_4544((param_1 + 0x12));
    bVar1 = *param_3;
    uVar2 = bVar1;
    _local_6 = (uVar2 + 1);
    if (0 < param_2) {
        if (uVar2 == 0) {
            _local_6 = 7;
        } else {
            if (((bVar1 == 0) || (SBORROW2(uVar2, 1))) || (1 < (uVar2 - 1))) {
                _local_6 = 9;
            } else {
                _local_6 = 8;
            }
        }
    }
    param_5 = _local_6;
    return;
}

pub fn pass1_1030_51f0(struct_a: *mut AStruct893) -> *mut AStruct893 {
    let local_BX_6: *mut AStruct893;
    let mut uVar1: u16;

    uVar1 = (struct_a >> 0x10);
    local_BX_6 = struct_a;
    local_BX_6.field_0xa4 = 0;
    local_BX_6.field_0xa6 = 0;
    local_BX_6.field_0xa8 = 0;
    local_BX_6.field_0xaa = 0;
    local_BX_6.field_0xac = 0;
    return struct_a;
}

pub fn pass1_1030_4574(param_1: *mut AStruct882) {
    let local_BX_6: *mut AStruct882;
    let mut local_ES_6: u16;

    local_ES_6 = (param_1 >> 0x10);
    local_BX_6 = param_1;
    local_BX_6.field_0xc = u16_1050_518c;
    local_BX_6.field_0xe = 0x518e;
    local_BX_6.field_0x10 = &g_alloc_addr_1050_1050;
    return param_1 & 0xffff0000 | ZEXT24(&local_BX_6.field_0xc);
}

pub fn pass1_1030_44be(struct_a: *mut AStruct881) {
    let struct_b: *mut AStruct881;
    let struct_b_hi: *mut AStruct881;
    let struct_c: *mut pass1_struct_1;
    let mut in_stack_0000ffec: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    struct_a = 0;
    struct_b.field_0x8 = 0;
    struct_b.field_0x12 = 0;
    struct_b.field_0x152 = 0;
    struct_b.field_0x154 = 0;
    struct_b.field_0x156 = 0;
    struct_b.field_0x158 = 0;
    struct_b.field_0x15a = 0;
    struct_b.field_0x15c = 0;
    struct_b.field_0x160 = 0;
    struct_b.field_0x164 = 0;
    struct_b.field_0x568 = 0x0;
    struct_c = process_struct_1010_20ba(
        _g_AStruct372_1050_0ed0,
        CONCAT22((in_stack_0000ffec >> 0x10), 2),
    );
    struct_b.field_0x568 = (struct_c + 100);
    return;
}

pub fn pass1_1030_3af6(
    param_1: *mut AStruct880,
    param_2: u16,
    param_3: u16,
    param_4: *mut u32,
    param_5: u16,
) -> *mut AStruct880 {
    let local_DI_16: *mut AStruct880;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_DI_16 = param_1;
    unsafe {
        param_1 = *param_4;
    }
    local_DI_16.field_0x4 = (param_4 + 1);
    local_DI_16.field_0x6 = param_3;
    local_DI_16.field_0x8 = param_2;
    return param_1;
}

pub fn process_struct_1020_7f38(param_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.ptr_a_lo = s_0_020_1050_3ab0;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    param_1.ptr_a_lo = s_1_1050_389a;
    (param_1 + 2) = &PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1020_775a(in_struct_1: *mut AStruct44, param_2: u8) -> *mut AStruct44 {
    process_struct_1020_75c4(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn process_struct_1020_75c4(param_1: *mut AStruct44) {
    let local_struct_1: *mut AStruct376;
    let local_struct_1_hi: *mut AStruct376;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = 0x7780;
    local_struct_1.ptr_a_hi = 0x1020;
    local_struct_1.u16_xe2 = 0x781c;
    local_struct_1.u16_xe4 = 0x1020;
    process_struct_1020_808e(param_1);
    return;
}

pub fn process_struct_1020_4092(in_struct_1: *mut AStruct660) -> *mut AStruct660 {
    let local_struct_1: *mut AStruct660;
    let local_struct_1_hi: *mut AStruct660;

    zero_list_1008_3e38(in_struct_1);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.field_0x6 = 0;
    local_struct_1.field_0x8 = 0;
    local_struct_1.field_0xa = 1;
    local_struct_1.field_0xc = 0;
    local_struct_1.field_0xe = 0;
    zero_list_1008_3e38((in_struct_1 & 0xffff0000 | ZEXT24(local_struct_1 + 1)));
    return in_struct_1;
}

pub fn process_struct_1020_2594(param_1: *mut AStruct44) {
    let local_struct_1: *mut AStruct376;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = s_fem36_wav_1050_270c;
    local_struct_1.ptr_a_hi = 0x1020;
    local_struct_1.u16_xe2 = (s_fem51_wav_1050_27a2 + 6);
    local_struct_1.u16_xe4 = 0x1020;
    process_struct_1020_808e(param_1);
    return;
}

pub fn pass1_1020_25c0(in_struct_1_1: *mut AStruct649) {
    let puVar1: *mut u16;
    let mut in_ax: u16;
    let mut in_dx: u16;
    let local_struct_1: *mut AStruct649;
    let local_struct_1_hi: *mut AStruct649;
    let in_struct_1: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let fn_ptr_1: fn();

    in_struct_1 = CONCAT22(in_dx, in_ax);
    local_struct_1_hi = (in_struct_1_1 >> 0x10);
    local_struct_1 = in_struct_1_1;
    if (local_struct_1.field_0xee != 0) {
        fn_ptr_1 = (local_struct_1.field_0xee + 8);
        in_struct_1 = (**fn_ptr_1)();
    }
    if (local_struct_1.field_0xea == 0) {
        local_struct_1.field_0xea = 1;
        process_struct_1000_179c(0x98, (in_struct_1 >> 0x10));
        if (in_struct_1 == 0) {
            _local_6 = 0x0;
        } else {
            puVar1 = &local_struct_1.field_0xcc;
            unsafe {
                *puVar1 = *puVar1 + 1;
            }
            _local_6 = process_struct_1020_1738(
                in_struct_1,
                local_struct_1.field_0xcc,
                in_struct_1_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10,
            );
        }
        fn_ptr_1 = (*_local_6 + 8);
        (**fn_ptr_1)(0x1000, _local_6, (_local_6 >> 0x10));
    }
    return;
}

// pass1_struct_1 *32 process_struct_1010_20ba (AStruct372 *32 in_struct_372_ptr, char *32 in_string_1)
pub fn process_struct_1010_20ba(a: *mut AStruct372, b: *mut libc::c_char) -> *mut pass1_struct1 {
    let mut out: pass1_struct1 = pass1_struct1 {};

    &out
}

pub fn process_struct_1020_808e(in_struct_1: &mut AStruct44) {
    let puVar1: *mut u16;
    let local_AStruct591_ptr_2: *mut AStruct376;
    let local_struct_1: *mut AStruct376;
    let local_struct_1_hi: *mut AStruct376;
    let mut local_6: u16;
    let mut local_4: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x82bc;
    local_struct_1.ptr_a_hi = 0x1020;
    local_struct_1.u16_xe2 = 0x8358;
    local_struct_1.u16_xe4 = 0x1020;
    if (in_struct_1 == 0x0) {
        puVar1 = 0x0;
        local_AStruct591_ptr_2 = 0x0;
    } else {
        puVar1 = &local_struct_1.u16_xe2;
        local_AStruct591_ptr_2 = local_struct_1_hi;
    }
    _local_6 = CONCAT22(local_AStruct591_ptr_2, puVar1);
    *_local_6 = s_1_1050_389a;
    puVar1[1] = &PTR_LOOP_1050_1008;
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.struct_215_ptr_0xd2)));
    in_struct_1.ptr_a_lo = 0x380a;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    in_struct_1.ptr_a_lo = s_1_1050_389a;
    local_struct_1.ptr_a_hi = &PTR_LOOP_1050_1008;
    return;
}
