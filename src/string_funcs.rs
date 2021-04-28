pub fn process_string_1000_28dc(in_string_1: &mut String) -> String {
    let mut i32_1: i32;
    let mut pch_2: u16;
    let out_string_1: *mut libc::c_char;
    let string_1: *mut libc::c_char;
    let string_2: *mut libc::c_char;
    let mut string_4: u32;

    pch_2 = (s___NMSG___1050_63f6 + 8);
    loop {
        string_2 = pch_2;
        pch_2 = pch_2 + 2;
        unsafe {
            string_1 = *string_2;
        }
        out_string_1 = pch_2;
        if ((string_1 == in_string_1) || (out_string_1 = string_1 + 1, out_string_1 == 0x0)) {
            return out_string_1;
        }
        i32_1 = -1;
        while {
            if (i32_1 == 0) {
                break;
            }
            i32_1 = i32_1 + -1;
            string_4 = pch_2;
            pch_2 = pch_2 + 1;
            *string_4 != '\0'
        } {}
    }
}

pub fn str_fn_1000_28e0(param_1: i32, param_2: u16) -> String {
    let piVar1: *mut i32;
    let mut iVar2: i32;
    let string_1: *mut libc::c_char;
    let string_2: *mut libc::c_char;
    let string_3: *mut libc::c_char;

    string_1 = (s___NMSG___1050_63f6 + 8);
    loop {
        piVar1 = string_1;
        string_1 = (string_1 + 2);
        unsafe {
            iVar2 = *piVar1;
        }
        string_2 = string_1;
        if ((iVar2 == param_1) || (string_2 = (iVar2 + 1), string_2 == 0x0)) {
            return string_2;
        }
        iVar2 = -1;
        while {
            if (iVar2 == 0) {
                break;
            }
            iVar2 = iVar2 + -1;
            piVar1 = string_1;
            string_1 = (string_1 + 1);
            unsafe { *piVar1 != '\0' }
        } {}
    }
}

pub fn process_string_1000_2917(param_1: *mut libc::c_char, param_2: u16) {
    let mut string2: u16;
    let mut iVar1: i32;
    let mut unaff_ES: u16;
    let mut string1: u32;

    if (&PTR_LOOP_1050_61ec != 0) {
        string2 = process_string_1000_28dc(param_1);
        if (string2 != 0) {
            iVar1 = -1;
            while {
                if (iVar1 == 0) {
                    break;
                }
                iVar1 = iVar1 + -1;
                string1 = string2;
                string2 = string2 + 1;
                *string1 != '\0'
            } {}
            process_string_1000_55b1();
        }
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_12

pub fn process_string_1000_2a00(param_1: *mut astruct_150) -> u16 {
    let mut bVar1: bool;
    let mut iVar2: i32;
    let mut unaff_BP: i32;
    let mut uvar3: u16;
    let mut unaff_SS: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u8;
    let uStack15: u8;
    let mut local_e: [u8; 8];
    let mut local_6: u16;
    let mut local_4: u16;
    let mut iStack2: i32;

    iStack2 = unaff_BP + 1;
    local_4 = SUB42(&g_alloc_addr_1050_1050, 0);
    uVar3 = 0xffff;
    if ((*&param_1.field_0xa & 0x40) != 0) {
        *&param_1.field_0xa = 0;
        return 0xffff;
    }
    if ((*&param_1.field_0xa & 0x83) == 0) {}
    // goto LAB_1000_2af2;
    uVar3 = pass1_fn_1000_2fa4(param_1);
    local_6 = param_1.field_0xf4;
    process_struct_1000_2cb0(param_1);
    if (u16_1050_5f8a < param_1.field_0xb) {
        iVar2 = process_string_1000_55b1();
        if (iVar2 < 0) {}
        // goto LAB_1000_2a6a;
        // LAB_1000_2a82:
        bVar1 = false;
    } else {
        iVar2 = dos3_call_1000_35fe();
        if (-1 < iVar2) {}
        // goto LAB_1000_2a82;
        // LAB_1000_2a6a:
        bVar1 = true;
    }
    if (!bVar1) {
        if (local_6 == 0) {}
        // goto LAB_1000_2af2;
        copy_string_1000_3d3e(CONCAT22(unaff_SS, &local_10), s___1050_5fea);
        local_14 = local_e;
        if (local_10 == '\\') {
            local_14 = &uStack15;
        } else {
            process_string_1000_3cea(CONCAT22(unaff_SS, &local_10), s___1050_5fec);
        }
        pass1_fn_1000_3e82(local_6, local_14, unaff_SS, 10);
        iVar2 = dos3_call_1000_514e(&local_10, unaff_SS);
        if (iVar2 == 0) {}
        // goto LAB_1000_2af2;
    }
    uVar3 = 0xffff;
    // LAB_1000_2af2:
    *&param_1.field_0xa = 0;
    return uVar3;
}

pub fn process_string_1000_2ba0() {
    pass1_fn_1000_3024();
    if (PTR_LOOP_1050_5fc9 != '\0') {
        pass1_fn_1000_3f5c();
    }
    return;
}

pub fn process_string_1000_3cea(string_a: *mut libc::c_char, string_b: *mut libc::c_char) {
    let puVar1: *mut u16;
    let pcVar2: *mut libc::c_char;
    let puVar3: *mut u16;
    let mut iVar4: i32;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let local_str_1_1: *mut libc::c_char;
    let pcVar7: *mut libc::c_char;
    let puVar8: *mut u16;
    let puVar9: *mut u16;
    let local_str_1: *mut libc::c_char;
    let mut local_ES_21: u16;
    let mut bVar10: bool;
    let temp_1087f15098c83: *mut libc::c_char;
    let temp_87f9662d6e1: *mut u16;

    local_str_1 = (string_a >> 0x10);
    local_str_1_1 = string_a;
    bVar10 = true;
    iVar4 = -1;
    while {
        if (iVar4 == 0) {
            break;
        }
        iVar4 = iVar4 + -1;
        puVar1 = local_str_1_1;
        local_str_1_1 = (local_str_1_1 + 1);
        unsafe {
            bVar10 = *puVar1 == '\0';
        }
        !bVar10
    } {}
    puVar9 = (local_str_1_1 + -1);
    local_ES_21 = (string_b >> 0x10);
    pcVar7 = string_b;
    uVar5 = 0xffff;
    while {
        if (uVar5 == 0) {
            break;
        }
        uVar5 = uVar5 - 1;
        pcVar2 = pcVar7;
        pcVar7 = pcVar7 + 1;
        unsafe {
            bVar10 = *pcVar2 == '\0';
        }
        !bVar10
    } {}
    //uVar5 = ~uVar5;
    if (!bVar10) {
        pcVar7 = pcVar7 + -uVar5;
        uVar5 = uVar5 + 1;
    }
    puVar8 = (pcVar7 + -uVar5);
    if (uVar5 == 0) {
        puVar1 = puVar8;
        puVar8 = puVar8 + 1;
        unsafe {
            *puVar9 = *puVar1;
        }
        uVar5 = 0xfffe;
        puVar9 = (local_str_1_1 + 1);
    } else {
        if ((puVar8 & 1) != 0) {
            puVar1 = puVar8;
            puVar8 = (puVar8 + 1);
            unsafe {
                *puVar9 = *puVar1;
            }
            uVar5 = uVar5 - 1;
            puVar9 = local_str_1_1;
        }
    }
    uVar6 = uVar5 >> 1;
    while (uVar6 != 0) {
        uVar6 = uVar6 - 1;
        temp_87f9662d6e1 = puVar9;
        puVar9 = puVar9 + 1;
        puVar1 = puVar8;
        puVar8 = puVar8 + 1;
        unsafe {
            *temp_87f9662d6e1 = *puVar1;
        }
    }
    uVar5 = ((uVar5 & 1) != 0);
    while (uVar5 != 0) {
        uVar5 = uVar5 - 1;
        puVar3 = puVar9;
        puVar9 = (puVar9 + 1);
        puVar1 = puVar8;
        puVar8 = (puVar8 + 1);
        unsafe {
            *puVar3 = *puVar1;
        }
    }
    return;
}

pub fn copy_string_1000_3d3e(
    in_string_1: *mut libc::c_char,
    in_string_2: *mut libc::c_char,
) -> u16 {
    let puVar1: *mut u16;
    let paVar2: *mut astruct_166;
    let mut uVar3: i32;
    let mut uVar4: i32;
    let local_string_list_1: *mut libc::c_char;
    let local_string_list_2: *mut libc::c_char;
    let local_DI_26: *mut astruct_166;
    let mut local_ES_26: u16;
    let mut local_DS_8: u16;
    let mut bool_1: bool;
    // temp_1087faeaca0cc: *mut *mut u8;
    let string_3: *mut libc::c_char;

    local_DS_8 = (in_string_2 >> 0x10);
    local_string_list_1 = in_string_2;
    bool_1 = true;
    uVar3 = 0xffff;
    local_string_list_2 = local_string_list_1;
    while {
        if (uVar3 == 0) {
            break;
        }
        uVar3 = uVar3 - 1;
        string_3 = local_string_list_2;
        local_string_list_2 = local_string_list_2 + 1;
        unsafe {
            bool_1 = *string_3 == '\0';
        }
        !bool_1
    } {}
    //uVar3 = ~uVar3;
    local_ES_26 = (in_string_1 >> 0x10);
    local_DI_26 = in_string_1;
    if (bool_1) {
        if ((in_string_1 & 1) != 0) {
            local_DI_26 = &local_DI_26.field_0x1;
            local_string_list_1 = local_string_list_1 + 1;
            unsafe {
                *in_string_1 = *in_string_2;
            }
            uVar3 = uVar3 - 1;
        }
    } else {
        local_DI_26 = &local_DI_26.field_0x2;
        local_string_list_1 = local_string_list_1 + 2;
        in_string_1 = in_string_2;
        uVar3 = uVar3 - 1;
    }
    uVar4 = uVar3 >> 1;
    while (uVar4 != 0) {
        uVar4 = uVar4 - 1;
        paVar2 = local_DI_26;
        local_DI_26 = &local_DI_26.field_0x2;
        puVar1 = local_string_list_1;
        local_string_list_1 = (local_string_list_1 + 2);
        unsafe {
            paVar2 = *puVar1;
        }
    }
    uVar3 = ((uVar3 & 1) != 0);
    while (uVar3 != 0) {
        uVar3 = uVar3 - 1;
        paVar2 = local_DI_26;
        local_DI_26 = &local_DI_26.field_0x1;
        puVar1 = local_string_list_1;
        local_string_list_1 = (local_string_list_1 + 1);
        unsafe {
            *paVar2 = *puVar1;
        }
    }
    return local_ES_26;
}

pub fn get_string_index_1000_3da4(in_string_1: *mut libc::c_char) -> u32 {
    let mut uVar1: i32;
    let mut string3: u16;
    let bool2: u8;
    let mut string1: u32;

    string3 = in_string_1;
    bool2 = 0x1;
    uVar1 = 0xffff;
    while {
        if (uVar1 == 0) {
            break;
        }
        uVar1 = uVar1 - 1;
        string1 = string3;
        string3 = string3 + 1;
        unsafe {
            bool2 = *string1 == '\0';
        }
        !bool2
    } {}
    // uVar1 = ~uVar1;
    if (bool2) {
        uVar1 = uVar1 - 1;
    }
    return uVar1;
}

pub fn process_string_1000_3dbe(
    in_string_1: *mut libc::c_char,
    in_string_2: *mut libc::c_char,
    param_3: u16,
) -> *mut u8 {
    let puVar1: *mut u8;
    let puVar2: *mut u8;
    let mut string2: u16;
    let mut string3: u16;
    let mut uvar3: u16;
    let mut string1: u32;
    let char1: u8;

    uVar3 = (in_string_1 >> 0x10);
    string2 = in_string_2;
    string3 = in_string_1;
    if (param_3 != 0) {
        while {
            string1 = string2;
            string2 = string2 + 1;
            char1 = *string1;
            if (char1 == '\0') {
                break;
            }
            puVar1 = string3;
            string3 = string3 + 1;
            unsafe {
                *puVar1 = char1;
            }
            param_3 = param_3 - 1;
            param_3 != 0
        } {}
        while (param_3 != 0) {
            param_3 = param_3 - 1;
            puVar2 = string3;
            string3 = string3 + 1;
            unsafe {
                *puVar2 = 0;
            }
        }
    }
    return in_string_1;
}

pub fn process_string_1000_3de8(
    in_string_1: *mut libc::c_char,
    param_2: *mut libc::c_char,
    param_3: u16,
) -> u8 {
    let pbVar1: *mut byte;
    let pcVar2: *mut libc::c_char;
    let mut bVar3: u8;
    let mut uVar4: u16;
    let mut iVar5: i32;
    let mut string_3: u16;
    let mut string_4: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let bool1: u8;
    let mut string_1: u32;
    let mut string_2: u32;

    if (param_3 != 0) {
        uVar7 = (in_string_1 >> 0x10);
        string_4 = in_string_1;
        uVar4 = param_3;
        uVar6 = string_4;
        while {
            if (uVar4 == 0) {
                break;
            }
            uVar4 = uVar4 - 1;
            string_1 = uVar6;
            uVar6 = uVar6 + 1;
            unsafe { *string_1 != '\0' }
        } {}
        iVar5 = param_3 - uVar4;
        uVar8 = (param_2 >> 0x10);
        string_3 = param_2;
        while {
            if (iVar5 == 0) {
                break;
            }
            iVar5 = iVar5 + -1;
            string_2 = string_4;
            string_4 = string_4 + 1;
            pcVar2 = string_3;
            string_3 = string_3 + 1;
            unsafe { *pcVar2 == *string_2 }
        } {}
        bVar3 = *(string_3 + -1);
        param_3 = 0;
        pbVar1 = (string_4 - 1);
        unsafe {
            bool1 = bVar3 == *pbVar1;
        }
        let pb_var1_val = unsafe { *pbVar1 };
        if (bVar3 < pb_var1_val || bool1) {
            if (bool1) {}
            // goto LAB_1000_3e1a;
            param_3 = 0xfffe;
        }
        //param_3 = ~param_3;
    }
    // LAB_1000_3e1a:
    return param_3;
}

pub fn process_string_1000_3ec0(uparam_1: i32, param_2: *mut libc::c_char) -> u8 {
    let uVar1: u8;
    let mut str_index: u16;
    let mut uVar2: i32;
    let extraout_AH: u8;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    _local_8 = CONCAT22(PTR_LOOP_1050_5fc0, PTR_LOOP_1050_5fbe);
    if (((PTR_LOOP_1050_5fc0 | PTR_LOOP_1050_5fbe) != 0) && ((param_2 | param_1) != 0)) {
        str_index = get_string_index_1000_3da4(CONCAT22(param_2, param_1));
        loop {
            uVar4 = (_local_8 >> 0x10);
            iVar3 = _local_8;
            if (((iVar3 + 2) | _local_8) == 0) {
                break;
            }
            uVar2 = get_string_index_1000_3da4(CONCAT22((iVar3 + 2), _local_8));
            if (((str_index < uVar2) && (*(*_local_8 + str_index) == '='))
                && (
                    uVar1 = process_string_1000_3de8(
                        CONCAT22((iVar3 + 2), _local_8),
                        CONCAT22(param_2, param_1),
                        str_index,
                    ),
                    CONCAT11(extraout_AH, uVar1) == 0,
                ))
            {
                return _local_8 + str_index + 0x1;
            }
            _local_8 = (_local_8 & 0xffff0000 | (iVar3 + 4));
        }
    }
    return '\0';
}

pub fn string_fn_1000_3f9c(
    param_1: *mut libc::c_char,
    param_2: *mut libc::c_char,
    param_3: *mut libc::c_char,
    param_4: *mut libc::c_char,
    param_5: *mut libc::c_char,
) -> u8 {
    let puVar1: *mut u8;
    let mut uVar2: u16;
    let mut unaff_BP: i32;
    char * *ppcStack16;
    let mut local_4: u16;
    let mut iStack2: i32;

    iStack2 = unaff_BP + 1;
    local_4 = SUB42(&g_alloc_addr_1050_1050, 0);
    PTR_LOOP_1050_68b2._0_1_ = 0x42;
    PTR_LOOP_1050_68ae = param_1;
    PTR_LOOP_1050_68b0 = param_2;
    _PTR_LOOP_1050_68a8 = CONCAT22(param_2, param_1);
    PTR_LOOP_1050_68ac = 0x7fff;
    ppcStack16 = &param_5;
    uVar2 = pass1_fn_1000_30b4(
        &PTR_LOOP_1050_68a8,
        &g_alloc_addr_1050_1050,
        CONCAT22(param_4, param_3),
    );
    puVar1 = _PTR_LOOP_1050_68a8;
    PTR_LOOP_1050_68ac = PTR_LOOP_1050_68ac + -1;
    if (PTR_LOOP_1050_68ac < 0) {
        ppcStack16 = &PTR_LOOP_1050_68a8;
        dos3_call_1000_2bb6();
    } else {
        _PTR_LOOP_1050_68a8 = (_PTR_LOOP_1050_68a8 & 0xffff0000 | ZEXT24(PTR_LOOP_1050_68a8 + 1));
        unsafe {
            *puVar1 = 0;
        }
    }
    return uVar2;
}

pub fn process_string_1000_440c(param_1: u16) {
    let pcVar1: *mut libc::c_char;
    let mut cVar2: u8;
    let uVar3: u8;
    let extraout_AH: u8;
    let iVar4: u16;
    let extraout_AH_00: u8;
    let extraout_AH_01: u8;
    let mut uVar5: i32;
    let in_i16_2: *mut u8;
    let mut in_i16_2_00: i32;
    let mut bVar6: bool;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    cVar2 = process_string_1000_3ec0(0x61ca, &g_alloc_addr_1050_1050);
    uVar5 = CONCAT11(extraout_AH, cVar2);
    _local_8 = CONCAT22(param_1, uVar5);
    if (((param_1 | uVar5) != 0) && (*_local_8 != '\0')) {
        process_string_1000_3dbe(
            CONCAT22(
                PTR_LOOP_1050_61de,
                PTR_PTR_LAB_1050_534f_1_1050_61d4_1050_61dc,
            ),
            CONCAT22(param_1, uVar5),
            3,
        );
        _local_8 = CONCAT22(param_1, uVar5 + 3);
        cVar2 = *_local_8;
        if (cVar2 == '-') {
            _local_8 = CONCAT22(param_1, uVar5 + 4);
        }
        in_i16_2 = 0x0;
        uVar8 = 0;
        uVar7 = 0xe10;
        iVar4 = pass1_fn_1000_3e2c((_local_8 & 0xffff | param_1 << 0x10));
        uVar3 = pass1_fn_1000_52be(iVar4, in_i16_2, uVar7, uVar8);
        u16_1050_61ce = CONCAT11(extraout_AH_00, uVar3);
        while ((
            pcVar1 = _local_8,
            *_local_8 == '+' || ('/' < *_local_8 && (*_local_8 < ':')),
        )) {
            _local_8 = (_local_8 & 0xffff0000 | (local_8 + 1));
        }
        PTR_LOOP_1050_61d0 = in_i16_2;
        if (*_local_8 == ':') {
            in_i16_2_00 = 0;
            uVar8 = 0;
            uVar7 = 0x3c;
            _local_8 = (_local_8 & 0xffff0000 | (local_8 + 1));
            iVar4 = pass1_fn_1000_3e2c((pcVar1 & 0xffff0000 | (local_8 + 1)));
            uVar3 = pass1_fn_1000_52be(iVar4, in_i16_2_00, uVar7, uVar8);
            bVar6 = CARRY2(u16_1050_61ce, CONCAT11(extraout_AH_01, uVar3));
            u16_1050_61ce = u16_1050_61ce + CONCAT11(extraout_AH_01, uVar3);
            PTR_LOOP_1050_61d0 = PTR_LOOP_1050_61d0 + bVar6 + in_i16_2_00;
            while ((pcVar1 = _local_8, '/' < *_local_8 && (*_local_8 < ':'))) {
                _local_8 = (_local_8 & 0xffff0000 | (local_8 + 1));
            }
            if (*_local_8 == ':') {
                _local_8 = (_local_8 & 0xffff0000 | (local_8 + 1));
                uVar5 = pass1_fn_1000_3e2c((pcVar1 & 0xffff0000 | (local_8 + 1)));
                bVar6 = CARRY2(u16_1050_61ce, uVar5);
                u16_1050_61ce = u16_1050_61ce + uVar5;
                PTR_LOOP_1050_61d0 = PTR_LOOP_1050_61d0 + bVar6 + in_i16_2_00;
                while ('/' < *_local_8 && (*_local_8 < ':')) {
                    _local_8 = (_local_8 & 0xffff0000 | (local_8 + 1));
                }
            }
        }
        if (cVar2 == '-') {
            bVar6 = u16_1050_61ce != 0;
            u16_1050_61ce = -u16_1050_61ce;
            PTR_LOOP_1050_61d0 = -(PTR_LOOP_1050_61d0 + bVar6);
        }
        u16_1050_61d2 = SEXT12(*_local_8);
        if (u16_1050_61d2 == 0) {
            *_PTR_PTR_1050_61e0 = '\0';
        } else {
            process_string_1000_3dbe(_PTR_PTR_1050_61e0, _local_8, 3);
        }
    }
    return;
}

pub fn process_string_1000_472c(in_string_1: *mut libc::c_char, in_char_2: u8) -> *mut u8 {
    let puVar1: *mut u8;
    let mut uVar2: i32;
    let mut string2: u16;
    let mut string3: u16;
    let mut local_ES_4: u16;
    let bool1: u8;
    let mut string1: u32;

    local_ES_4 = (in_string_1 >> 0x10);
    string2 = in_string_1;
    bool1 = 0x1;
    uVar2 = 0xffff;
    string3 = string2;
    while {
        if (uVar2 == 0) {
            break;
        }
        uVar2 = uVar2 - 1;
        string1 = string3;
        string3 = string3 + 1;
        unsafe {
            bool1 = *string1 == '\0';
        }
        !bool1
    } {}
    uVar2 = !uVar2;
    while {
        if (uVar2 == 0) {
            break;
        }
        uVar2 = uVar2 - 1;
        puVar1 = string2;
        string2 = string2 + 1;
        unsafe {
            bool1 = in_char_2 == *puVar1;
        }
        !bool1
    } {}
    if (!bool1) {
        if (in_char_2 != '\0') {
            return 0x0;
        }
        string2 = string2 + 1;
    }
    return (string2 + -1);
}

pub fn process_string_1000_475e(
    in_string_1: *mut libc::c_char,
    in_string_2: *mut libc::c_char,
) -> i32 {
    let uVar1: u8;
    let char_2: u8;
    let byte_1: u8;
    let string_1: *mut libc::c_char;
    let mut string_3: u16;
    let mut string_4: u16;
    let mut string_2: u32;
    let char_1: u8;

    string_4 = in_string_2;
    string_3 = in_string_1;
    string_1 = (s_You_may_not_run_a_turn__The_game_1050_00df + 0x20);
    while {
        while {
            char_2 = string_1;
            if (char_2 == '\0') {}
            // goto LAB_1000_479d;
            string_2 = string_4;
            string_4 = string_4 + 1;
            uVar1 = *string_2;
            char_1 = *string_3;
            string_1 = CONCAT11(char_1, uVar1);
            string_3 = string_3 + 1;
            char_1 == uVar1
        } {}
        byte_1 = uVar1 + 0xbf + (-((uVar1 + 0xbf) < 0x1a) & 0x20) + 'A';
        string_1._0_1_ = char_1 + 0xbf + (-((char_1 + 0xbf) < 0x1a) & 0x20) + 0x41;
        string_1 = CONCAT11(byte_1, string_1);
        string_1 == byte_1
    } {}
    char_2 = (string_1 < byte_1) * -2 + 0x1;
    // LAB_1000_479d:
    return char_2;
}

pub fn process_string_1000_4d58(
    in_string_1: *mut libc::c_char,
    in_string_2: *mut libc::c_char,
    param_3: *mut u8,
    param_4: u32,
    param_5: *mut u8,
) {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut uvar3: u16;
    let mut uVar4: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    local_a = 0;
    local_c = 0;
    uVar3 = (in_string_1 >> 0x10);
    iVar1 = in_string_1;
    unsafe {
        if ((*in_string_1 == '\0') || (*(iVar1 + 1) != ':')) {
            if ((in_string_2._2_2_ | in_string_2) != 0) {
                *in_string_2 = '\0';
            }
        } else {
            if ((in_string_2._2_2_ | in_string_2) != 0) {
                *in_string_2 = *in_string_1;
                *(in_string_2 + 1) = *(iVar1 + 1);
                *(in_string_2 + 2) = 0;
            }
            in_string_1 = (in_string_1 & 0xffff0000 | (iVar1 + 2));
        }
    }
    local_6 = 0;
    local_8 = 0;
    _local_12 = in_string_1;
    loop {
        uVar4 = (_local_12 >> 0x10);
        uVar2 = _local_12;
        if (*_local_12 == '\0') {
            break;
        }
        if ((*_local_12 == '/') || (*_local_12 == '\\')) {
            local_8 = uVar2 + 1;
            local_6 = uVar4;
        } else {
            if (*_local_12 == '.') {
                local_c = uVar2;
                local_a = uVar4;
            }
        }
        _local_12 = (_local_12 & 0xffff0000 | (uVar2 + 1));
    }
    if ((local_6 | local_8) == 0) {
        if ((param_3._2_2_ | param_3) != 0) {
            unsafe {
                *param_3 = 0;
            }
        }
    } else {
        if ((param_3._2_2_ | param_3) != 0) {
            uVar4 = local_8 - in_string_1;
            if (0xff < uVar4) {
                uVar4 = 0xff;
            }
            process_string_1000_3dbe(
                (param_3 & 0xffff | param_3._2_2_ << 0x10),
                in_string_1,
                uVar4,
            );
            *(param_3 + uVar4) = 0;
        }
        in_string_1 = CONCAT22(local_6, local_8);
    }
    if (((local_a | local_c) != 0) && (in_string_1 <= local_c)) {
        if ((param_4._2_2_ | param_4) != 0) {
            uVar4 = local_c - in_string_1;
            if (0xff < uVar4) {
                uVar4 = 0xff;
            }
            process_string_1000_3dbe(
                (param_4 & 0xffff | param_4._2_2_ << 0x10),
                (in_string_1 & 0xffff | in_string_1._2_2_ << 0x10),
                uVar4,
            );
            *(param_4 + uVar4) = 0;
        }
        if ((param_5._2_2_ | param_5) == 0) {
            return;
        }
        uVar2 = uVar2 - local_c;
        if (0xff < uVar2) {
            uVar2 = 0xff;
        }
        process_string_1000_3dbe(
            (param_5 & 0xffff | param_5._2_2_ << 0x10),
            CONCAT22(local_a, local_c),
            uVar2,
        );
        *(param_5 + uVar2) = 0;
        return;
    }
    if ((param_4._2_2_ | param_4) != 0) {
        uVar2 = uVar2 - in_string_1;
        if (0xff < uVar2) {
            uVar2 = 0xff;
        }
        process_string_1000_3dbe(
            (param_4 & 0xffff | param_4._2_2_ << 0x10),
            (in_string_1 & 0xffff | in_string_1._2_2_ << 0x10),
            uVar2,
        );
        *(param_4 + uVar2) = 0;
    }
    if ((param_5._2_2_ | param_5) != 0) {
        unsafe {
            *param_5 = 0;
        }
    }
    return;
}

pub fn process_string_1000_545a(
    in_string_1: *mut libc::c_char,
    in_string_2: *mut libc::c_char,
) -> u8 {
    let mut char_3: u8;
    let string_1: *mut libc::c_char;
    let mut string_2: u16;
    let mut string_3: u16;
    let mut string_4: u32;
    let char_2: u8;
    let char_1: u8;

    string_3 = in_string_2;
    string_2 = in_string_1;
    string_1 = (s_You_may_not_run_a_turn__The_game_1050_00df + 0x20);
    while {
        while {
            if (string_1 == '\0') {
                return '\0';
            }
            string_4 = string_3;
            string_3 = string_3 + 1;
            char_2 = *string_4;
            char_1 = *string_2;
            string_1 = CONCAT11(char_1, char_2);
            string_2 = string_2 + 1;
            char_1 == char_2
        } {}
        char_3 = char_2 + 0xbf + (-((char_2 + 0xbf) < 0x1a) & 0x20) + 'A';
        string_1._0_1_ = char_1 + 0xbf + (-((char_1 + 0xbf) < 0x1a) & 0x20) + 0x41;
        string_1 = CONCAT11(char_3, string_1);
        string_1 == char_3
    } {}
    return (string_1 < char_3) * -2 + 0x1;
}

pub fn process_string_1000_55b1() -> u16 {
    let pcVar1: *mut libc::c_char;
    let mut string_2: u16;
    let mut i16_1: u16;
    let mut iVar2: i32;
    let mut p_i16_1: u16;
    let mut p_i16_2: u16;
    let mut string_3: u16;
    let mut in_stack_00000008: i32;
    let mut string_4: u16;
    let mut string_1: u32;
    let string_5: *mut libc::c_char;

    pass1_fn_1000_25a8();
    pass1_fn_1000_2913();
    string_2 = process_string_1000_28dc((s_version__d__d_1050_0012 + 2));
    if (string_2 != 0) {
        i16_1 = 9;
        if (*string_2 == 'M') {
            i16_1 = 0xf;
        }
        string_2 = string_2 + i16_1;
        iVar2 = 0x22;
        string_3 = string_2;
        while {
            if (iVar2 == 0) {
                break;
            }
            iVar2 = iVar2 + -1;
            string_1 = string_3;
            string_3 = string_3 + 1;
            unsafe { *string_1 != '\r' }
        } {}
        *(string_3 - 1) = 0;
    }
    FatalAppExit16(CONCAT22(0x1050, string_2), 0);
    FatalExit(0xff);
    p_i16_1 = (s___NMSG___1050_63f6 + 8);
    loop {
        string_5 = p_i16_1;
        p_i16_1 = p_i16_1 + 2;
        p_i16_2 = p_i16_1;
        if ((string_5 == in_stack_00000008) || (p_i16_2 = string_5 + 1, p_i16_2 == 0)) {
            return p_i16_2;
        }
        iVar2 = -1;
        while {
            if (iVar2 == 0) {
                break;
            }
            iVar2 = iVar2 + -1;
            pcVar1 = p_i16_1;
            p_i16_1 = p_i16_1 + 1;
            unsafe { *pcVar1 != '\0' }
        } {}
    }
}

pub fn process_string_1008_049c(param_1: u16, param_2: u16, in_string_1: *mut libc::c_char) {
    let mut str_index: u16;
    let string_1: *mut libc::c_char;
    let string_2: *mut libc::c_char;
    let extraout_var: u32;

    if (in_string_1 != 0x0) {
        str_index = get_string_index_1000_3da4(in_string_1);
        if (str_index != 0) {
            string_1._0_1_ = process_string_1000_545a(
                (in_string_1 & 0xffff0000 | (in_string_1 + 1)),
                s_nomono2_1050_00cc,
            );
            string_2 = CONCAT31(extraout_var, string_1);
            if (string_2 == 0x0) {
                g_string_1050_02ec = string_2;
            }
        }
    }
    return;
}

pub fn string_fn_1008_5fd8(param_1: *mut astruct_613, param_2: u8) -> u8 {
    let pcVar1: *mut libc::c_char;
    let piVar2: *mut i32;
    let in_DX: *mut astruct_199;
    let mut unaff_SS: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_a = &param_2;
    _local_6 = CONCAT22(unaff_SS, local_a);
    process_struct_1000_179c(0x1000, in_DX);
    local_8 = in_DX;
    pcVar1 = load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        param_1,
    );
    copy_string_1000_3d3e(CONCAT22(local_8, local_a), pcVar1);
    loop {
        piVar2 = _local_6;
        _local_6 = (_local_6 & 0xffff0000 | (local_6 + 2));
        let pi_var2_val = unsafe { *piVar2 };
        if (pi_var2_val == 0) {
            break;
        }
        pcVar1 = load_string_1010_847e(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            pi_var2_val,
        );
        process_string_1000_3cea(CONCAT22(local_8, local_a), pcVar1);
    }
    return local_a;
}

pub fn fn_1008_6048(in_string_1: *mut libc::c_char, param_2: u16, param_3: bool) -> u8 {
    let mut cVar1: u8;
    let mut local_AH_33: u8;
    let puVar2: *mut u8;
    let iVar3: u16;
    let mut iVar4: i32;
    let mut local_SS__1: i32;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_string_buf: [u8; 256];
    // va_list va_args;
    let mut local_4: i32;

    if (g_string_1050_02ec != 0x0) {
        puVar2 = &stack0x0008;
        if (u16_1050_02ee == 0xffff) {
            cVar1 = process_string_1000_3ec0(0x2f4, &g_alloc_addr_1050_1050);
            local_10a = CONCAT11(local_AH_33, cVar1);
            puVar2 = ((param_2 | local_10a) != 0);
            u16_1050_02ee = puVar2;
            local_108 = param_2;
        }
        param_3 = puVar2;
        if (u16_1050_02ee != 0) {
            wvsprintf16(
                &stack0x0008,
                CONCAT22(in_string_1, local_SS__1),
                CONCAT22(local_string_buf, (in_string_1 >> 0x10)),
            );
            OutputDebugString16(CONCAT22(local_SS__1, local_string_buf));
            iVar3 = OutputDebugString16(0x105002fa);
            param_3 = iVar3;
            if (_PTR_LOOP_1050_02f0 != 0) {
                pass1_fn_1000_2b5c(
                    _PTR_LOOP_1050_02f0,
                    (_PTR_LOOP_1050_02f0 >> 0x10),
                    0x2fd,
                    &g_alloc_addr_1050_1050,
                );
                iVar4 = pass1_fn_1000_2f48(_PTR_LOOP_1050_02f0, (_PTR_LOOP_1050_02f0 >> 0x10));
                param_3 = iVar4;
            }
        }
    }
    return param_3;
}

pub fn string_fn_1008_64c8(
    param_1: *mut libc::c_char,
    param_2: *mut libc::c_char,
    param_3: *mut libc::c_char,
    param_4: *mut libc::c_char,
) {
    let pcVar1: *mut libc::c_char;
    let mut in_AX: i32;
    let in_DX: *mut astruct_199;
    let mut local_DX_49: u16;
    let mut uVar2: u16;
    let mut local_DX_95: u16;
    let mut local_DX_121: u16;
    let mut uvar3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_1 == 0) {
        return;
    }
    process_struct_1000_179c(0x1e, in_DX);
    if ((in_DX | in_AX) == 0) {
        in_AX = 0;
        uVar2 = 0;
    } else {
        pass1_1008_6604(CONCAT22(in_DX, in_AX), param_2, (param_2 >> 0x10));
        uVar2 = local_DX_49;
    }
    _local_6 = CONCAT22(uVar2, in_AX);
    local_8 = 0;
    while (
        param_2 = (param_2 & 0xffff0000 | (param_2 - 1)),
        param_2 != 0,
    ) {
        pcVar1 = param_3 + 1;
        process_struct_1008_4544(param_1);
        uVar2 = local_8 + 1;
        uVar3 = local_DX_95;
        process_struct_1008_4544(_local_6);
        pass1_fn_1000_48a8(
            CONCAT22(local_DX_121, local_8),
            CONCAT22(uVar3, param_3),
            param_2._2_2_,
        );
        param_3 = pcVar1;
        local_8 = uVar2;
    }
    return;
}

pub fn process_string_1008_7e4a() -> bool {
    let uVar1: u8;
    let mut buf_size: i32;
    let local_AH_52: u8;
    let mut local_SS__1: u16;
    let in_stack_0000000a: *mut libc::c_char;
    let mut char_buf: u8;

    string_fn_1000_3f9c(
        &char_buf,
        local_SS__1,
        s__s_02x_1050_0347,
        &g_alloc_addr_1050_1050,
        _PTR_s_dcbSC_1050_0336_1050_033c,
    );
    buf_size = get_string_index_1000_3da4(CONCAT22(local_SS__1, &char_buf));
    uVar1 = process_string_1000_3de8(
        in_stack_0000000a,
        CONCAT22(local_SS__1, &char_buf),
        buf_size,
    );
    if (CONCAT11(local_AH_52, uVar1) == 0) {
        return 1;
    }
    return 0;
}

pub fn process_string_1008_9c86(param_1: u32, param_2: *mut libc::c_char, param_3: i32) {
    let mut uVar1: u16;
    let mut local_4: u16;

    uVar1 = get_string_index_1000_3da4((param_1 & 0xffff0000 | (param_1 + 10)));
    if (param_3 < uVar1) {
        uVar1 = param_3 - 1;
    }
    process_string_1000_3dbe(param_2, (param_1 & 0xffff0000 | (param_1 + 10)), uVar1);
    return;
}

pub fn load_string_switch_1008_a1f0(
    str_buffer_1: *mut libc::c_char,
    param_2: *mut u16,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) {
    let mut u_var1: u32;
    let mut b: u16;
    let mut uVar2: i32;
    let mut uvar3: u16;
    let paVar4: *mut astruct_493;
    let pcVar5: *mut libc::c_char;
    let pcVar6: *mut libc::c_char;
    let local_DL_217: u8;
    let mut extraout_DX: i32;
    let mut extraout_DX_00: u16;
    let mut local_DX_469: u16;
    let mut local_DX_507: u16;
    let mut local_DX_1061: u16;
    let string_base: *mut libc::c_char;
    let mut local_ES_32: u16;
    let mut local_ES_121: u16;
    let mut local_CS_1605: u16;
    let mut local_SS__1: u16;
    let mut uVar7: u32;
    let ppVar8: *mut pass1_struct_1;
    let mut a: u16;
    let local_11e: u8;
    let local_11d: u8;
    let mut resource_id_1: u16;
    let mut local_112: u32;
    let mut local_10e: u32;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f95f699dc: u32;
    let mut temp_5f759b5e79: u32;
    let fn_ptr_1: fn();

    uVar2 = 0;
    param_5 = 0;
    param_4 = 0;
    param_3 = 0;
    unsafe {
        *param_2 = 0;
    }
    local_ES_32 = (str_buffer_1 >> 0x10);
    string_base = str_buffer_1;
    string_base[0xe] = '\0';
    temp_5f759b5e79 = (string_base + 10);
    fn_ptr_1 = ((string_base + 10) + 0x10);
    (**fn_ptr_1)();
    _local_6 = CONCAT22(extraout_DX, uVar2);
    if ((extraout_DX | uVar2) == 0) {
        return;
    }
    param_5 = (uVar2 + 4);
    uVar3 = (uVar2 + 10);
    param_3 = uVar3;
    switch_stmt_1008_ab80(string_base, local_ES_32, param_5);
    unsafe {
        *param_2 = uVar3;
    }
    local_CS_1605 = &PTR_LOOP_1050_1008;
    a = _g_struct_73_1050_14cc;
    local_11e = (str_buffer_1 >> 0x10);
    b = (_g_struct_73_1050_14cc >> 0x10);
    match (uVar2 + 4) {
        1 => {
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x59a,
            );
            param_4 = 0xd1;
            // goto LAB_1008_a2b1;
        }
        2 => {
            uVar1 = (uVar2 + 6);
            resource_id_1 = (uVar1 >> 0x10);
            _local_DL_217 = extraout_DX_00;
            paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, resource_id_1);
            local_11e = local_SS__1;
            local_11d = (local_SS__1 >> 8);
            load_string_1010_84e0(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x100,
                CONCAT13(local_11d, CONCAT12(local_11e, local_106)),
                0x3ff,
            );
            local_11e = _local_DL_217;
            local_11d = (_local_DL_217 >> 8);
            uVar7 = pass1_1038_4d28(CONCAT13(local_11d, CONCAT12(local_11e, paVar4)));
            local_CS_1605 = 0x1000;
            string_fn_1000_3f9c(
                string_base + 0xe,
                local_ES_32,
                local_106,
                local_SS__1,
                uVar7,
            )
        }
        5 => {
            resource_id_1 = 0x59b;
            // goto LAB_1008_a277;
        }
        6 => {
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x631,
            );
            param_4 = 0xd4;
            // LAB_1008_a2b1:
            local_CS_1605 = 0x1010;
            param_3 = 1
        }
        7 => {
            resource_id_1 = 0x400;
            // LAB_1008_a277:
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                resource_id_1,
            )
        }
        9 => {
            if ((string_base + 0x416) == 0) {
                (string_base + 0x416) = 1;
            }
            // goto LAB_1008_a35a;
        }
        0xb => {
            if ((string_base + 0x41a) == 0) {
                (string_base + 0x41a) = 1;
            }
            // goto LAB_1008_a35a;
        }
        0xe => {
            if ((string_base + 0x41c) == 0) {
                (string_base + 0x41c) = 1;
            }
            // goto LAB_1008_a35a;
        }
        0x14 => {
            if ((string_base + 0x418) == 0) {
                (string_base + 0x418) = 1;
                pcVar6 = string_base + 0xe;
                pcVar5 = pcVar6;
                load_string_1010_84e0(
                    _g_struct_73_1050_14cc,
                    (_g_struct_73_1050_14cc >> 0x10),
                    0x3ff,
                    (str_buffer_1 & 0xff000000 | CONCAT12(local_11e, pcVar6)),
                    0x72a,
                );
                local_11e = (_g_struct_73_1050_14cc >> 0x10);
                load_string_1010_847e(_g_struct_73_1050_14cc, local_11e, 0x72b);
                process_string_1000_3cea(
                    (str_buffer_1 & 0xffff0000 | ZEXT24(pcVar6)),
                    CONCAT22(local_DX_469, pcVar5),
                );
                param_4 = 0x4c;
                ppVar8 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, 0x1002b);
                local_CS_1605 = 0x1010;
                pass1_1010_089e(ppVar8, 1, 10);
            }
            // goto LAB_1008_a35a;
        }
        0x16 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x713,
            );
            param_4 = 0x28
        }
        0x17 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x717,
            );
            param_4 = 0x2c
        }
        0x18 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x719,
            );
            param_4 = 0x2e
        }
        0x1b => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x71b,
            );
            param_4 = 0x30
        }
        0x1c => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x71d,
            );
            param_4 = 0x32
        }
        0x1f => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x720,
            );
            param_4 = 0x34
        }
        0x21 => {
            if ((string_base + 0x41e) == 0) {
                (string_base + 0x41e) = 1;
            }
            // LAB_1008_a35a:
            unsafe { *param_2 = 0 }
        }
        0x24 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x715,
            );
            param_4 = 0x2a
        }
        0x31 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x712,
            );
            param_4 = 0x27
        }
        0x32 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x714,
            );
            param_4 = 0x29
        }
        0x33 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x716,
            );
            param_4 = 0x2b
        }
        0x34 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x718,
            );
            param_4 = 0x2d
        }
        0x35 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x71a,
            );
            param_4 = 0x2f
        }
        0x36 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x71c,
            );
            param_4 = 0x31
        }
        0x37 => {
            pcVar5 = string_base + 0xe;
            pcVar6 = pcVar5;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xff000000 | CONCAT12(local_11e, pcVar5)),
                0x71e,
            );
            local_11e = (_g_struct_73_1050_14cc >> 0x10);
            load_string_1010_847e(_g_struct_73_1050_14cc, local_11e, 0x71f);
            local_CS_1605 = 0x1000;
            process_string_1000_3cea(
                (str_buffer_1 & 0xffff0000 | ZEXT24(pcVar5)),
                CONCAT22(local_DX_1061, pcVar6),
            );
            param_4 = 0x33
        }
        0x38 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x721,
            );
            param_4 = 0x35
        }
        0x39 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x722,
            );
            param_4 = 0x36
        }
        0x3a => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x723,
            );
            param_4 = 0x37
        }
        0x3b => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x724,
            );
            param_4 = 0x38
        }
        0x3c => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x725,
            );
            param_4 = 0x39
        }
        0x3d => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x742,
            );
            param_4 = 0xce
        }
        0x3e => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x743,
            );
            param_4 = 0xcf
        }
        0x3f => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x744,
            );
            param_4 = 0xd0
        }
        0x40 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x745,
            );
            param_4 = 0xd1
        }
        0x41 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x746,
            );
            param_4 = 0xd2
        }
        0x42 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x747,
            );
            param_4 = 0xd3
        }
        0x43 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x748,
            );
            param_4 = 0xd5
        }
        0x44 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x749,
            );
            param_4 = 0xd6
        }
        0x45 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x74a,
            );
            param_4 = 0xd7;
        }
    }
    if (_local_6 != 0x0) {
        local_11e = extraout_DX;
        fn_ptr_1 = *_local_6;
        (**fn_ptr_1)(local_CS_1605, uVar2, local_11e, 1, temp_5f759b5e79);
    }
    return;
}

pub fn load_string_1008_a8f4(
    param_1: *mut libc::c_char,
    param_2: *mut u16,
    param_3: *mut u16,
    param_4: u32,
) {
    let mut unaff_SS: u16;
    let mut u_var1: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    uVar1 = load_string_switch_1008_a1f0(
        param_1,
        param_2,
        CONCAT22(unaff_SS, &local_6),
        CONCAT22(unaff_SS, &local_6 + 2),
        param_4,
    );
    pass1_1008_944e(param_3, local_6, (local_6 >> 0x10));
    return uVar1;
}

pub fn load_string_1008_b1f0() {
    load_string_1010_847e(_g_struct_73_1050_14cc, 0x531, 0);
    return;
}

pub fn load_string_1008_b65a(param_1: u32, param_2: &mut string, param_3: u32) {
    let mut in_AX: i32;
    let mut in_DX: i32;
    let mut in_resource_id: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_b9ce(param_1, param_3);
    if (((in_DX | in_AX) == 0) || ((in_AX + 8) != 1)) {
        in_resource_id = 0x434;
    } else {
        in_resource_id = 0x435;
    }
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        param_2,
        in_resource_id,
    );
    return;
}

// WARNING: Variable defined which should be unmapped: u16_d
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn wsprintf_func_1008_b69c(struct_a: *mut pass1_struct_1) {
    let struct_e_lo: *mut astruct_199;
    let struct_e_a: *mut astruct_199;
    let struct_c: *mut astruct_915;
    let mut u16_j: u16;
    let struct_e_hi: *mut astruct_199;
    let struct_e_b: *mut astruct_199;
    let mut u16_a: u16;
    let mut u16_b: u16;
    let struct_b: *mut pass1_struct_1;
    let struct_b_hi: *mut pass1_struct_1;
    let mut u16_c: u16;
    let struct_d: *mut astruct_199;
    let mut u32_a: u32;
    let mut u16_d: u16;
    let mut u16_e: u16;
    let mut u16_f: u16;
    let mut u16_g: u16;
    let mut u16_h: u16;
    let mut va_args: [u8; 256];
    let fn_ptr_a: fn();

    struct_e_lo = &stack0xfdfe;
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(u16_c, struct_e_lo),
        0x6e7,
    );
    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    if (&struct_b.field_0xa == 0) {
        process_struct_1000_179c(0xc, struct_e_hi);
        struct_e_b = (struct_e_hi | struct_e_lo);
        if (struct_e_b == 0x0) {
            struct_e_a = 0x0;
            struct_e_b = 0x0;
        } else {
            struct_e_a = process_struct_1008_574a(CONCAT22(struct_e_hi, struct_e_lo));
        }
        struct_d = CONCAT22(struct_e_b, struct_e_a);
        struct_b.field_0xa = struct_e_a;
        struct_b.field_0xc = struct_e_b;
        u16_h = 1;
        while (u16_h < 6) {
            process_struct_1000_179c(0x12, (struct_d >> 0x10));
            u32_a._2_2_ = (struct_d >> 0x10) | struct_d;
            if (struct_d == 0x0) {
                struct_c = 0x0;
                u16_a = 0;
            } else {
                u32_a._0_1_ = pass1_1008_b0bc(struct_d);
                struct_c = CONCAT11(u32_a._1_1_, u32_a);
                u16_a = u32_a._2_2_;
            }
            u16_b = u16_a;
            wsprintf16(
                va_args,
                CONCAT22(&stack0xfdfe, u16_c),
                CONCAT22(u16_h, u16_c),
            );
            u16_j = pass1_fn_1008_60e8(va_args);
            struct_c.field_0x4 = u16_j;
            struct_c.field_0x6 = u16_b;
            fn_ptr_a = (&struct_b.field_0xa + 8);
            struct_d = (**fn_ptr_a)();
            u16_h = &struct_c.field_0x1;
        }
        struct_b.field_0x22 = 5;
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_20a
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn wsprintf_FUN_1008_b78a(param_1: u32) {
    let piVar1: *mut i32;
    let ppcVar2: fn();
    let uVar3: u8;
    let mut in_AX: i32;
    let extraout_AH: u8;
    let mut iVar4: i32;
    let mut uVar5: u16;
    let in_DX: *mut astruct_199;
    let mut uVar6: i32;
    let mut uVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: u32;
    let puStack514: *mut u8;
    let mut va_args: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x12, in_DX);
    uVar6 = in_DX | in_AX;
    if (uVar6 == 0) {
        iVar4 = 0;
        uVar6 = 0;
    } else {
        uVar3 = pass1_1008_b0bc(CONCAT22(in_DX, in_AX));
        iVar4 = CONCAT11(extraout_AH, uVar3);
    }
    uVar7 = uVar6;
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_SS, &local_206),
        0x6e7,
    );
    uVar9 = (param_1 >> 0x10);
    iVar8 = param_1;
    piVar1 = (iVar8 + 0x22);
    unsafe {
        *piVar1 = *piVar1 + 1;
    }
    wsprintf16(
        va_args,
        CONCAT22(&local_206, unaff_SS),
        CONCAT22((iVar8 + 0x22), unaff_SS),
    );
    puStack514 = va_args;
    local_206 = 0x1538b7f9;
    uVar5 = pass1_fn_1008_60e8();
    (iVar4 + 4) = uVar5;
    (iVar4 + 6) = uVar7;
    local_206 = (iVar8 + 10);
    ppcVar2 = ((iVar8 + 10) + 8);
    puStack514 = iVar4;
    ppcVar2(offset);
    return;
}

pub fn wsprintf_1008_d1c6(in_struct_a: *mut pass1_struct_3, param_2: u32) {
    let mut iVar1: i32;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let uVar4: u8;
    let local_AL_436: u8;
    let struct_a_hi: *mut astruct_199;
    let paVar5: *mut astruct_199;
    let local_AX_93: *mut astruct;
    let mut struct_d: i32;
    let mut struct_e: i32;
    let struct_h: *mut astruct;
    let mut local_AX_234: u16;
    let mut struct_c: i32;
    let struct_f: *mut astruct_199;
    let struct_g: *mut astruct_199;
    let mut extraout_DX: i32;
    let mut local_DX_121: u16;
    let mut extraout_DX_00: i32;
    let struct_a: *mut astruct_199;
    let mut extraout_DX_01: u16;
    let mut local_DX_436: u16;
    let struct_b: *mut pass1_struct_3;
    let struct_b_hi: *mut pass1_struct_3;
    let mut uVar8: i32;
    let local_46: u8;
    let mut uStack68: i32;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_20: u32;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut temp_5fd6f85ef2: u32;
    u16 * *fn_ptr_1;
    let puVar6: *mut u8;
    let mut uVar7: u32;
    let mut bool_a: bool;

    struct_b_hi = (in_struct_a >> 0x10);
    struct_b = in_struct_a;
    struct_a_hi = struct_b.field_0x12;
    paVar5 = struct_b.field_0x14;
    if ((paVar5 | struct_a_hi) != 0) {
        fn_ptr_1 = struct_a_hi;
        (**fn_ptr_1)();
        paVar5 = struct_f;
    }
    process_struct_1000_179c(0xc, paVar5);
    struct_g = (paVar5 | struct_a_hi);
    if (struct_g == 0x0) {
        paVar5 = 0x0;
        struct_g = 0x0;
    } else {
        paVar5 = process_struct_1008_574a(CONCAT22(paVar5, struct_a_hi));
    }
    struct_b.field_0x12 = paVar5;
    struct_b.field_0x14 = struct_g;
    puVar6 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 2);
    struct_d = puVar6;
    uVar8 = &PTR_LOOP_1050_1038;
    pass1_1038_4e78(param_2, puVar6 & 0xffff | ZEXT24(struct_g) << 0x10);
    _local_a = CONCAT22(extraout_DX, struct_d);
    local_46 = struct_d;
    uVar4 = local_46;
    ppcVar2 = (*_local_a + 0x10);
    struct_e = struct_d;
    ppcVar2(&PTR_LOOP_1050_1038, local_46, extraout_DX);
    _local_e = CONCAT22(local_DX_121, struct_e);
    bool_a = false;
    local_14 = 0;
    uStack68 = extraout_DX;
    while (local_14 < _local_e) {
        uVar8 = 0x1030;
        uVar7 = _local_e;
        pass1_1030_1d7c(struct_d, extraout_DX, local_14, (local_14 >> 0x10));
        struct_e = uVar7;
        if ((((extraout_DX_00 | struct_e) != 0) && ((struct_e + 0x1c) != 0x8000002))
            && ((iVar1 = (struct_e + 0x12), iVar1 == 5 || (iVar1 == 6))))
        {
            local_AX_234 = (struct_e + 4);
            paVar5 = ((struct_e + 6) & 0xff);
            big_switch_statement_1020_bd80((struct_e + 0xc));
            wsprintf16(
                struct_b + 1,
                CONCAT13(0xc, CONCAT12(0xea, struct_b_hi)),
                CONCAT22(local_AX_234, 0x1050),
            );
            uVar7 = ZEXT24(struct_b + 1);
            pass1_fn_1008_60e8();
            struct_c = uVar7;
            uVar3 = ZEXT24(paVar5);
            uVar8 = 0x1000;
            process_struct_1000_179c(0x12, paVar5);
            if ((paVar5 | struct_c) != 0) {
                uVar8 = 0x1018;
                pass1_1018_4808(
                    CONCAT22(paVar5, struct_c),
                    1,
                    uVar7 & 0xffff | uVar3 << 0x10,
                    (struct_e + 4),
                );
            }
            local_46 = '0';
            ppcVar2 = (&struct_b.field_0x12 + 4);
            ppcVar2();
            bool_a = true;
            uStack68 = uVar8;
        }
        local_14 = local_14 + 1;
    }
    if (!bool_a) {
        load_str_1010_84ac(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x441,
        );
        uVar8 = 0x1000;
        paVar5 = struct_a;
        struct_e = _local_e;
        process_struct_1000_179c(0x12, struct_a);
        if ((paVar5 | struct_e) == 0) {
            local_AL_436 = '\0';
            local_DX_436 = 0;
        } else {
            uVar8 = 0x1018;
            pass1_1018_4808(
                CONCAT22(paVar5, struct_e),
                0,
                CONCAT13((struct_a >> 8), CONCAT12(struct_a, _local_e)),
                0,
            );
            local_AL_436 = struct_e;
            local_DX_436 = extraout_DX_01;
        }
        temp_5fd6f85ef2 = &struct_b.field_0x12;
        ppcVar2 = (&struct_b.field_0x12 + 4);
        ppcVar2(
            uVar8,
            temp_5fd6f85ef2,
            (temp_5fd6f85ef2 >> 0x10),
            local_AL_436,
            local_DX_436,
        );
    }
    if ((extraout_DX | struct_d) != 0) {
        ppcVar2 = *_local_a;
        ppcVar2(uVar8, uVar4, extraout_DX, 1, local_46, uStack68);
    }
    return;
}

pub fn wsprintf_1008_d4f6(param_1: *mut astruct_298, in_struct_b: *mut astruct_298) -> u8 {
    let mut iVar1: i32;
    let mut switch_var: u16;
    let lVar2: u32;
    let puVar3: *mut u32;
    let mut uVar4: u32;
    let ppcVar5: fn();
    let mut bVar6: bool;
    let puVar7: *mut u32;
    let paVar8: *mut astruct_199;
    let BVar9: bool;
    let mut uVar10: i32;
    let mut uVar11: i32;
    let puVar12: *mut u32;
    let mut uVar13: u32;
    let mut uVar14: u32;
    let extraout_DX: *mut astruct_199;
    let mut uVar15: i32;
    let mut extraout_DX_00: i32;
    let mut extraout_DX_01: i32;
    let paVar16: *mut astruct_199;
    let struct_a: *mut astruct_199;
    let mut extraout_DX_02: u16;
    let mut uVar17: u16;
    let local_BX_6: *mut astruct_3;
    let local_BX_19: *mut astruct_2;
    let mut uVar18: u16;
    let paVar19: *mut astruct_199;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_2c: u16;
    let mut local_1e: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar18 = (in_struct_b >> 0x10);
    local_BX_6 = in_struct_b;
    lVar2 = local_BX_6.field_0x200;
    paVar19 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    puVar7 = local_BX_19.field_0xe;
    paVar8 = local_BX_19.field_0x10;
    if ((paVar8 | puVar7) != 0) {
        unsafe {
            ppcVar5 = *puVar7;
        }
        (**ppcVar5)();
        paVar8 = extraout_DX;
    }
    process_struct_1000_179c(0xc, paVar8);
    uVar15 = paVar8 | puVar7;
    if (uVar15 == 0) {
        paVar8 = 0x0;
        uVar15 = 0;
    } else {
        paVar8 = process_struct_1008_574a(CONCAT22(paVar8, puVar7));
    }
    local_BX_19.field_0xe = paVar8;
    local_BX_19.field_0x10 = uVar15;
    puVar3 = &local_BX_6.field_0xc;
    unsafe {
        ppcVar5 = (*puVar3 + 0x10);
    }
    puVar12 = puVar3;
    (**ppcVar5)(0x1000, puVar3, &local_BX_6.field_0xe);
    uVar14 = puVar12 & 0xffff | extraout_DX_00 << 0x10;
    bVar6 = false;
    local_14 = 0;
    while (local_14 < uVar14) {
        uVar13 = uVar14;
        pass1_1030_1d7c(puVar3, (puVar3 >> 0x10), local_14, (local_14 >> 0x10));
        uVar15 = uVar13;
        if ((((extraout_DX_01 | uVar15) != 0) && ((uVar15 + 0x1c) != 0x8000002))
            && ((iVar1 = (uVar15 + 0x12), iVar1 == 5 || (iVar1 == 6))))
        {
            switch_var = (uVar15 + 0xc);
            BVar9 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, switch_var, 0x34);
            if ((BVar9 == 0) && ((uVar15 + 0x1c) != lVar2)) {
                uVar18 = (uVar15 + 4);
                big_switch_statement_1020_bd80(switch_var);
                paVar8 = paVar19;
                uVar10 = wsprintf16(
                    local_BX_19 + 1,
                    CONCAT22(0xcf4, paVar19),
                    CONCAT22(uVar18, 0x1050),
                );
                pass1_fn_1008_60e8();
                uVar18 = 0x1000;
                paVar16 = paVar8;
                uVar11 = uVar10;
                process_struct_1000_179c(0x14, paVar8);
                if ((paVar16 | uVar11) != 0) {
                    uVar18 = 0x1018;
                    pass1_1018_47c8(
                        CONCAT13((paVar16 >> 8), CONCAT12(paVar16, uVar11)),
                        1,
                        CONCAT13((paVar8 >> 8), CONCAT12(paVar8, uVar10)),
                        (uVar15 + 0xc),
                        (uVar15 + 4),
                    );
                }
                uVar4 = &local_BX_19.field_0xe;
                // WARNING: Load size is inaccurate
                ppcVar5 = (*local_BX_19.field_0xe + 4);
                (**ppcVar5)(uVar18, uVar4, (uVar4 >> 0x10));
                bVar6 = true;
            }
        }
        local_14 = local_14 + 1;
    }
    if (!bVar6) {
        load_str_1010_84ac(
            _g_struct_73_1050_14cc,
            (_g_struct_73_1050_14cc >> 0x10),
            0x442,
        );
        uVar18 = 0x1000;
        uVar13 = uVar14;
        paVar8 = struct_a;
        process_struct_1000_179c(0x14, struct_a);
        if ((paVar8 | uVar13) == 0) {
            uVar14 = 0;
            uVar17 = 0;
        } else {
            uVar18 = 0x1018;
            pass1_1018_47c8(
                uVar13 & 0xffff | ZEXT24(paVar8) << 0x10,
                0,
                uVar14 & 0xffff | ZEXT24(struct_a) << 0x10,
                0,
                0,
            );
            uVar14 = uVar13;
            uVar17 = extraout_DX_02;
        }
        uVar4 = &local_BX_19.field_0xe;
        // WARNING: Load size is inaccurate
        ppcVar5 = (*local_BX_19.field_0xe + 4);
        (**ppcVar5)(uVar18, uVar4, (uVar4 >> 0x10), uVar14, uVar17);
    }
    return uVar14;
}

pub fn modify_string_11d8_024f(param_1: u8, param_2: u16, param_3: u16) {
    let pbVar1: *mut byte;
    let piVar2: *mut i32;
    char * *ppcVar3;
    let puVar4: *mut u32;
    let puVar5: *mut u8;
    byte * *ppbVar6;
    let pcVar7: *mut libc::c_char;
    let puVar8: *mut u16;
    char * *ppcVar9;
    let mut cVar10: u8;
    let pcVar11: *mut code;
    let mut char8: u8;
    let mut bVar12: u8;
    let mut cVar13: u8;
    let mut bVar14: u8;
    let mut bVar15: u8;
    let mut cVar16: u8;
    let mut cVar18: u8;
    let mut cVar19: u8;
    let mut bVar20: u8;
    let mut uVar21: i32;
    let mut extraout_DL: u8;
    let mut extraout_DL_00: u8;
    let mut cVar22: u8;
    let mut cVar23: u8;
    let mut uVar25: i32;
    let puVar26: *mut u32;
    let piVar27: *mut i32;
    let mut local_BP__1: u16;
    let unaff_SI: *mut libc::c_char;
    let pcVar29: *mut libc::c_char;
    let unaff_DI: *mut libc::c_char;
    let mut local_ES__1: u16;
    let mut local_DS__1: u16;
    let mut local_FS__1: u16;
    let mut local_DXAX_60: u32;
    let mut local_resc: u16;
    let mut local_res18: u32;
    let mut in_stack_00000063: u8;
    let mut uStack23: i32;
    let mut cStack17: u8;
    let local_3: u8;
    let mut uVar31: i32;
    let temp_86276906b60: *mut u16;
    let mut iVar17: i32;
    let mut uVar24: i32;
    let mut bVar28: u8;
    let mut uVar30: u32;
    let string_1: *mut libc::c_char;

    _local_3 = CONCAT21(local_BP__1, local_3);
    uVar30 = _local_3;
    string_1 = unaff_SI + param_3;
    unsafe {
        *string_1 = *string_1 + param_1;
        pcVar11 = swi(0);
        char8 = (*pcVar11)();
        string_1 = unaff_SI;
        *string_1 = *string_1 + extraout_DL;
        string_1 = unaff_SI + param_3;
        *string_1 = *string_1 + param_2;
        uVar21 = param_2 & 0xff00 | (param_2 * 0x2);
        string_1 = unaff_SI + param_3;
        *string_1 = *string_1 + char8;
        string_1 = unaff_SI + param_3;
        *string_1 = *string_1 + char8;
        pcVar11 = swi(0);
        uStack23 = param_2;
        char8 = (*pcVar11)();
        string_1 = unaff_SI;
        *string_1 = *string_1 + cStack17;
        string_1 = unaff_SI + param_3;
        *string_1 = *string_1 + uVar21;
        string_1 = unaff_SI + param_3;
        *string_1 = *string_1 + char8;
        string_1 = unaff_SI + param_3;
        *string_1 = *string_1 + char8;
        pcVar11 = swi(0);
        char8 = (*pcVar11)();
        string_1 = unaff_SI;
        *string_1 = *string_1 + extraout_DL_00;
        string_1 = unaff_SI + param_3;
        *string_1 = *string_1 + uVar21;
        uVar25 = param_3 & 0xff00 | (param_3 + uVar21);
        string_1 = unaff_SI + uVar25;
        *string_1 = *string_1 + char8;
        string_1 = unaff_SI + uVar25;
        *string_1 = *string_1 + char8;
        pcVar11 = swi(0);
        local_DXAX_60 = (*pcVar11)();
        pcVar7 = unaff_SI + 1;
        out(*unaff_SI, (local_DXAX_60 >> 0x10));
        string_1 = pcVar7;
        *string_1 = *string_1 + (local_DXAX_60 >> 0x10);
        string_1 = pcVar7 + uVar25;
        cVar19 = uVar21;
        *string_1 = *string_1 + cVar19;
        cVar18 = (local_DXAX_60 >> 8) + cVar19;
        bVar12 = local_DXAX_60;
        _local_3 = uVar30;
        string_1 = pcVar7 + uVar25;
        *string_1 = *string_1 + bVar12;
        pbVar1 = (pcVar7 + uVar25);
        bVar15 = *pbVar1;
        *pbVar1 = *pbVar1 + bVar12;
        bVar12 = bVar12 + CARRY1(bVar15, bVar12);
        iVar17 = CONCAT11(cVar18, bVar12);
        string_1 = pcVar7 + uVar25;
        *string_1 = *string_1 - bVar12;
        pbVar1 = (pcVar7 + uVar25);
        *pbVar1 = *pbVar1 | bVar12;
        *0x17 = bVar12;
        string_1 = &stack0xfffe + pcVar7;
        *string_1 = *string_1 + bVar12;
        string_1 = pcVar7 + uVar25;
        *string_1 = *string_1;
        string_1 = pcVar7 + uVar25;
        cVar22 = (uVar21 >> 8);
        *string_1 = *string_1 + cVar22;
        string_1 = pcVar7 + uVar25;
        *string_1 = *string_1 + cVar19;
        string_1 = &stack0x0015 + unaff_DI;
        *string_1 = *string_1 + cVar18;
        string_1 = &stack0xfffe + pcVar7;
        *string_1 = *string_1 + bVar12;
        string_1 = pcVar7 + uVar25;
        *string_1 = *string_1;
        piVar2 = (pcVar7 + uVar25);
        *piVar2 = *piVar2 - iVar17;
        string_1 = pcVar7 + uVar25;
        *string_1 = *string_1 - bVar12;
        pbVar1 = (pcVar7 + uVar25);
        *pbVar1 = *pbVar1 | bVar12;
        puVar8 = (unaff_SI + 2);
        *unaff_DI = *pcVar7;
        string_1 = (uVar25 + puVar8);
        *string_1 = *string_1 + bVar12;
        string_1 = (uVar25 + puVar8);
        *string_1 = *string_1 + bVar12;
        bVar12 = bVar12 ^ *(uVar25 + puVar8);
        string_1 = (uVar25 + puVar8);
        *string_1 = *string_1 - bVar12;
        pbVar1 = (uVar25 + puVar8);
        *pbVar1 = *pbVar1 | bVar12;
        pcVar7 = unaff_SI + 4;
        (unaff_DI + 1) = *puVar8;
        string_1 = pcVar7 + uVar25;
        *string_1 = *string_1 + bVar12;
        string_1 = pcVar7 + uVar25;
        *string_1 = *string_1 + bVar12;
        string_1 = pcVar7 + uVar25;
        *string_1 = *string_1 - bVar12;
        pbVar1 = (pcVar7 + uVar25);
        *pbVar1 = *pbVar1 | bVar12;
        pcVar7 = unaff_DI + 4;
        string_1 = unaff_SI + 5 + uVar25;
        *string_1 = *string_1 + bVar12;
        string_1 = unaff_SI + 5 + uVar25;
        *string_1 = *string_1 + bVar12;
        pcVar29 = unaff_SI + 6;
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar22;
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar19;
        string_1 = (uVar25 + 0x17);
        *string_1 = *string_1 + cVar18;
        string_1 = &stack0xfffe + pcVar29;
        *string_1 = *string_1 + bVar12;
        uVar31 = (uVar30 >> 8) & 0xff00 | (CONCAT11(cVar18, bVar12) >> 8);
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1;
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar22;
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar19;
        string_1 = pcVar29 + uVar25 + 0x17;
        *string_1 = *string_1 + cVar22;
        string_1 = &stack0xfffe + pcVar29;
        *string_1 = *string_1 + bVar12;
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1;
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar22;
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar19;
        cVar13 = bVar12 * 0x2;
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar13;
        char8 = pcVar29[uVar25 - 0x7e];
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar13;
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar22;
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar19;
        cVar19 = cVar19 + cVar13;
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar13;
        char8 = bVar12 + char8 + pcVar29[uVar25 - 0x7e];
        string_1 = pcVar29 + uVar25;
        *string_1 = *string_1 + cVar13;
        ppcVar9 = (unaff_SI + 7);
        out(*pcVar29, CONCAT11(cVar18, char8));
        string_1 = (uVar25 + ppcVar9);
        *string_1 = *string_1 + cVar22;
        string_1 = (uVar25 + ppcVar9);
        *string_1 = *string_1 + cVar19;
        string_1 = (uVar25 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        cVar22 = char8 + cVar13 + *(ppcVar9 + (uVar25 - 0x7e));
        string_1 = (uVar25 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        ppcVar3 = ppcVar9;
        *ppcVar3 = *ppcVar3 + CONCAT11(cVar18, cVar22);
        ppcVar3 = ppcVar9;
        *ppcVar3 = *ppcVar3 + cVar22;
        string_1 = (uVar25 + ppcVar9);
        *string_1 = *string_1 + cVar19;
        string_1 = (uVar25 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        char8 = *(ppcVar9 + (uVar25 - 0x7e));
        string_1 = (uVar25 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        piVar2 = 0x1400;
        *piVar2 = *piVar2 + uVar25;
        string_1 = (uVar25 + ppcVar9);
        *string_1 = *string_1 + cVar19;
        string_1 = (uVar25 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        cVar22 = cVar22 + char8 + *(ppcVar9 + (uVar25 - 0x7e));
        string_1 = (uVar25 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        piVar2 = (pcVar7 + uVar25);
        *piVar2 = (&stack0xfffe + *piVar2);
        ppcVar3 = ppcVar9;
        *ppcVar3 = *ppcVar3 + cVar22;
        string_1 = (uVar25 + ppcVar9);
        *string_1 = *string_1 + cVar19;
        bVar28 = (uVar25 >> 8) + cVar19;
        uVar25 = uVar25 & 0xff;
        puVar26 = (uVar25 | bVar28 << 8);
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        cVar22 = cVar22 + *(puVar26 + ppcVar9 + -0x7e);
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        piVar2 = (&stack0xfffe + ppcVar9);
        *piVar2 = *piVar2 + ppcVar9;
        ppcVar3 = ppcVar9;
        *ppcVar3 = *ppcVar3 + cVar22;
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + cVar19;
        cVar13 = cVar13 + cVar22;
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        cVar22 = cVar22 + *(puVar26 + ppcVar9 + -0x7e);
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        ppcVar3 = ppcVar9;
        *ppcVar3 = pcVar7 + *ppcVar3;
        ppcVar3 = ppcVar9;
        *ppcVar3 = *ppcVar3 + cVar22;
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + cVar19;
        bVar20 = cVar19 + cVar22;
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        char8 = *(puVar26 + ppcVar9 + -0x7e);
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + cVar13;
        bVar14 = cVar13 + CARRY2(uVar31, CONCAT11(cVar18, cVar13));
        pbVar1 = (puVar26 + ppcVar9);
        *pbVar1 = *pbVar1 | bVar14;
        puVar4 = puVar26;
        bVar12 = (bVar20 & 0x1f) % 9;
        bVar15 = *puVar4;
        *puVar4 = bVar15 << bVar12 | bVar15 >> 9 - bVar12;
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + bVar14;
        cVar23 = cVar22 + char8 + *(puVar26 + ppcVar9 + -0x7e);
        uVar24 = CONCAT11(cVar18 + cVar19, cVar23);
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + bVar14;
        puVar4 = (puVar26 + pcVar7);
        uVar21 = *puVar4;
        *puVar4 = *puVar4 + uVar24;
        bVar14 = bVar14 + CARRY2(uVar21, uVar24);
        pbVar1 = (puVar26 + ppcVar9);
        *pbVar1 = *pbVar1 | bVar14;
        puVar4 = puVar26;
        bVar15 = (bVar20 & 0x1f) % 0x11;
        uVar21 = *puVar4;
        *puVar4 = uVar21 << bVar15 | uVar21 >> 0x11 - bVar15;
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + bVar14;
        char8 = *(puVar26 + ppcVar9 + -0x7e);
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + bVar14;
        puVar4 = (&stack0xfffe + ppcVar9);
        uVar21 = *puVar4;
        *puVar4 = *puVar4 + puVar26;
        bVar14 = bVar14 + CARRY2(uVar21, puVar26);
        pbVar1 = (puVar26 + ppcVar9);
        *pbVar1 = *pbVar1 | bVar14;
        bVar15 = bVar14 % 0x17;
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + bVar15;
        cVar22 = *(puVar26 + ppcVar9 + -0x7e);
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + bVar15;
        ppcVar3 = ppcVar9;
        pcVar29 = *ppcVar3;
        *ppcVar3 = &stack0x001d + *ppcVar3;
        bVar15 = bVar15 + CARRY2(pcVar29, &stack0x001d);
        pbVar1 = (puVar26 + ppcVar9);
        *pbVar1 = *pbVar1 | bVar15;
        cVar16 = bVar15 + (bVar14 / 0x17) * '\x17';
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + cVar16;
        cVar10 = *(puVar26 + ppcVar9 + -0x7e);
        string_1 = (puVar26 + ppcVar9);
        *string_1 = *string_1 + cVar16;
        pbVar1 = (puVar26 + ppcVar9);
        *pbVar1 = *pbVar1 | cVar16 + CARRY2(uVar31 + CONCAT11(cVar18, cVar13), &stack0xfffe);
        puVar5 = (puVar26 + ppcVar9);
        *puVar5 = *puVar5;
        iVar17 = CONCAT11(
            cVar18 + cVar19,
            cVar23 + char8 + cVar22 + cVar10 + *(puVar26 + ppcVar9 + -0x7e),
        );
        puVar5 = (puVar26 + ppcVar9);
        *puVar5 = *puVar5;
        string_1 = pcVar7;
        bVar12 = ppcVar9;
        *string_1 = *string_1 + bVar12;
        puVar4 = puVar26 + 0x3c00;
        *puVar4 = *puVar4 + uVar25;
        piVar27 = (uVar25 | (bVar28 * 0x2) << 8);
        piVar2 = piVar27;
        *piVar2 = *piVar2 + 1;
        piVar2 = piVar27;
        bVar15 = *piVar2;
        *piVar2 = *piVar2 + bVar12;
        out(0x0, iVar17);
        if ((in_stack_00000063 + 0x73 + CARRY1(bVar15, bVar12)) != '\0') {
            // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
        }
        puVar4 = (unaff_DI + 0x77);
        *puVar4 = *puVar4
            + (0x8c < in_stack_00000063
                || CARRY1(in_stack_00000063 + 0x73, CARRY1(bVar15, bVar12)))
                * ((&stack0x001f & 3) - (*puVar4 & 3));
        piVar2 = piVar27 + 1;
        *piVar2 = *piVar2 + bVar12;
        bVar12 = bVar12 ^ *(piVar27 + 1);
        string_1 = (piVar27 + pcVar7);
        *string_1 = *string_1 + bVar12;
        string_1 = (piVar27 + pcVar7);
        *string_1 = *string_1 + bVar12;
        string_1 = (piVar27 + pcVar7);
        *string_1 = *string_1 + bVar12;
        if (iVar17 == -1) {
            if (iVar17 != -1) {
                out(*0x2, 0);
                pbVar1 = (piVar27 + 0x6b);
                *pbVar1 = *pbVar1 + bVar20;
                pbVar1 = (piVar27 + 3);
                *pbVar1 = *pbVar1 + bVar12;
                piVar2 = piVar27 + 1;
                *piVar2 = *piVar2 + (bVar12 ^ *(piVar27 + 1)) + (piVar27 + -1);
                ppbVar6 = (piVar27 + 0x51);
                *ppbVar6 = (piVar27 + -1) + *ppbVar6;
                // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
            }
            // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
        }
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn modify_string_11b8_02b9(param_1: u8, param_2: u16, param_3: u16) {
    let mut byte_2: u8;
    let mut byte_3: u8;
    let mut unaff_BP: i32;
    let mut unaff_SI: i32;
    let mut unaff_DI: i32;
    let local_DS__1: *mut u8;
    let local_res0: *mut u8;
    let mut byte_1: u8;
    let bytes_1: *mut byte;
    let mut char_3: u8;
    let string_1: *mut libc::c_char;

    byte_3 = param_2 + *(param_3 + unaff_SI + -0x7e);
    string_1 = (param_3 + unaff_SI);
    unsafe {
        *string_1 = *string_1 + param_1;
        out(param_2 & 0xff00 | byte_3, param_1);
        bytes_1 = (param_3 + unaff_DI);
        byte_1 = *bytes_1;
        *bytes_1 = *bytes_1 + byte_3;
        byte_2 = param_1 + CARRY1(byte_1, byte_3);
        bytes_1 = (param_3 + unaff_SI);
        *bytes_1 = *bytes_1 | byte_2;
        string_1 = (param_3 + unaff_SI);
        *string_1 = *string_1 + byte_2;
        char_3 = *(param_3 + unaff_SI + -0x7e);
        string_1 = (param_3 + unaff_SI);
        *string_1 = *string_1 + byte_2;
        out(param_2 & 0xff00 | (byte_3 + char_3), byte_2);
        bytes_1 = (unaff_BP + unaff_SI);
        byte_1 = *bytes_1;
        *bytes_1 = *bytes_1 + param_3;
        bytes_1 = (param_3 + unaff_SI);
        *bytes_1 = *bytes_1 | byte_2 + CARRY1(byte_1, param_3);
    }
    return;
}

pub fn string_fn_1008_e2a4(param_1: u32, param_2: u32, param_3: u32) -> u16 {
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let lVar4: u32;
    let mut uVar5: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    uVar5 = param_2;
    uVar3 = load_string_1010_847e(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    iVar1 = pass1_1000_3d7a(uVar3, uVar5);
    if ((iVar1 == 0) || (iVar1 = pass1_1000_3d7a(param_3, param_2), iVar1 == 0)) {
        return 0;
    }
    lVar4 = pass1_1008_e8cc(param_1, param_2, param_3);
    if (lVar4 != 0) {
        iVar1 = (lVar4 + 0xc);
        iVar2 = iVar1 + -1;
        if (iVar2 == 0) {
            return 2;
        }
        if (iVar2 < 1) {
            return 0;
        }
        if (SBORROW2(iVar2, 1)) {
            return 0;
        }
        if (1 < iVar1 + -2) {
            return 0;
        }
    }
    return 1;
}

pub fn load_string_1008_ee56(param_1: u32) {
    load_string_1010_847e(_g_struct_73_1050_14cc, (param_1 + 0x16));
    return;
}

pub fn process_string_1010_184a(param_1: *mut u32, param_2: u32) {
    let mut uVar1: u16;
    let mut iVar2: i32;
    let mut local_6: u16;

    uVar1 = u16_1050_0ecc;
    iVar2 = (u16_1050_0ecc * 6 + 0xeba) * 8;
    unsafe {
        iVar2 = process_string_1000_475e(*(iVar2 + *param_1), *(iVar2 + param_2));
    }
    if (iVar2 == 0) {
        iVar2 = (uVar1 * 6 + 0xebc) * 8;
        unsafe {
            iVar2 = process_string_1000_475e(*(iVar2 + *param_1), *(iVar2 + param_2));
        }
        if (iVar2 == 0) {
            iVar2 = (uVar1 * 6 + 0xebe) * 8;
            unsafe {
                process_string_1000_475e(*(iVar2 + *param_1), *(iVar2 + param_2));
            }
        }
    }
    return;
}

pub fn string_fn_1010_2c34() -> *mut pass1_struct_1 {
    let string_b: *mut libc::c_char;
    let mut uVar1: u16;
    let struct_a: *mut astruct_199;
    let out_buffer: *mut pass1_struct_1;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    out_buffer = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(local_a, 3));
    struct_a = (out_buffer >> 0x10);
    uVar1 = SUB42(out_buffer, 0);
    process_struct_1000_179c(0x80, struct_a);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x80,
        out_buffer,
        0x5eb,
    );
    process_string_1000_3cea(out_buffer, 0x105011c8);
    string_b = pass1_1010_e964(uVar1, struct_a);
    process_string_1000_3cea(out_buffer, string_b);
    return out_buffer;
}

pub fn str_fn_1010_5286(param_1: u16, param_2: u16, param_1_00: u32) {
    let paVar1: *mut astruct_493;
    let paVar2: *mut astruct_493;
    let mut in_DX: i32;
    let struct_a: *mut astruct_199;
    let string_b: *mut libc::c_char;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_1_00, (param_1_00 >> 0x10));
    struct_a = (in_DX | paVar1);
    if (struct_a == 0x0) {
        return 0;
    }
    paVar2 = paVar1;
    process_struct_1000_179c(0x80, struct_a);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x80,
        CONCAT22(struct_a, paVar2),
        0x5eb,
    );
    process_string_1000_3cea(CONCAT22(struct_a, paVar2), s__1050_13ac);
    string_b = pass1_1038_4d28(CONCAT22(in_DX, paVar1));
    process_string_1000_3cea(CONCAT22(struct_a, paVar2), string_b);
    return CONCAT22(struct_a, paVar2);
}

pub fn str_fn_1010_6034(param_1: *mut astruct_432) {
    let uVar1: u8;
    let mut uVar2: i32;
    let extraout_var: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let local_BX_7: *mut astruct_432;
    let mut uvar3: u16;

    uVar3 = (param_1 >> 0x10);
    local_BX_7 = param_1;
    local_BX_7.field_0x1e = 1;
    local_BX_7.field_0x20 = 1;
    local_BX_7.field_0x72 = 1;
    local_BX_7.field_0x74 = 1;
    pass1_1010_60a0(param_1);
    uVar1 = pass1_1000_4906((param_1 & 0xffff0000 | &local_BX_7.field_0x22), 0, 0x40);
    uVar2 = CONCAT31(extraout_var, uVar1);
    load_str_1010_84ac(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x630,
    );
    local_BX_7.field_0x68 = uVar2;
    local_BX_7.field_0x6a = extraout_DX;
    load_str_1010_84ac(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x62f,
    );
    local_BX_7.field_0x6c = uVar2;
    local_BX_7.field_0x6e = extraout_DX_00;
    return;
}

pub fn write_private_profile_str_1010_62ec(param_1: u32, param_2: u8) {
    write_private_profile_str_1010_5b10(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn string_fn_1010_8018(param_1: *mut astruct_446, param_2: u16) {
    let mut iVar1: i32;
    let local_c: *mut astruct_446;
    let mut uStack10: u16;

    if (((s_559_bmp_1050_1f9f + 1) + param_2 * 10) != 0) {
        local_c = param_1;
        uStack10 = (param_1 >> 0x10);
        pass1_1010_878c(param_1, ((s_559_bmp_1050_1f9f + 1) + param_2 * 10));
        if (local_c.field_0x67c != 0x0) {
            iVar1 = param_2 * 10;
            string_fn_1008_64c8(
                local_c.field_0x67c,
                CONCAT22(
                    ((s_559_bmp_1050_1f9f + 7) + iVar1),
                    ((s_560_bmp_1050_1fa7 + 1) + iVar1),
                ),
                *((s_559_bmp_1050_1f9f + 5) + iVar1),
                *((s_559_bmp_1050_1f9f + 3) + iVar1),
            );
            return;
        }
    }
    return;
}

pub fn load_string_1010_847e(param_1: i32, param_2: u16, resource_id: u16) {
    LoadString16(
        0x3ff,
        CONCAT22(param_2, param_1 + 0x682),
        resource_id,
        g_h_instance_1050_038c,
    );
    return CONCAT22(param_2, param_1 + 0x682);
}

pub fn load_str_1010_84ac(
    in_struct_73_low: *mut astruct_73,
    in_struct_73_hi: *mut astruct_73,
    resource_id: u16,
) {
    LoadString16(
        0x3ff,
        CONCAT22(in_struct_73_hi, in_struct_73_low + 1),
        resource_id,
        g_h_instance_1050_038c,
    );
    pass1_fn_1008_60e8(in_struct_73_low + 1, in_struct_73_hi);
    return;
}

pub fn load_string_1010_84e0(a: u16, b: u16, buf_lenout_buffer: &mut string, in_resource_id: u16) {
    let mut resource_id: u16;

    LoadString16(buf_len, out_buffer, in_resource_id, g_h_instance_1050_038c);
    return;
}

pub fn wsprintf_1010_8c96(param_1: u32, param_2: &mut string, param_3: u32) {
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut uvar3: u16;
    let paVar4: *mut astruct_493;
    let puVar5: *mut u32;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: i32;
    let mut extraout_DX_02: u16;
    let mut uVar6: i32;
    let mut extraout_DX_03: i32;
    let mut extraout_DX_04: u16;
    let mut iVar7: i32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let puVar10: *mut u8;
    let mut uVar11: u32;
    // va_list valist;
    let mut uVar12: u16;
    let mut local_38: u32;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar8 = (param_3 >> 0x10);
    iVar7 = param_3;
    iVar1 = (iVar7 + 6);
    valist = param_2;
    if (iVar1 == 0) {
        uVar12 = 0x436;
    } else {
        uVar9 = (param_1 >> 0x10);
        uVar3 = (param_2 >> 0x10);
        if (iVar1 == 1) {
            match (iVar7 + 4) {
                1 | 2 => {
                    uVar2 = (iVar7 + 8);
                    local_a = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
                    local_10 = (local_a + 0xc);
                    local_c = (local_a + 0x10);
                    local_6 = &local_10;
                    if (0 < local_c) {
                        load_string_1010_847e(
                            _g_struct_73_1050_14cc,
                            (_g_struct_73_1050_14cc >> 0x10),
                            0x437,
                        );
                        local_4 = extraout_DX;
                        wsprintf16(
                            valist,
                            CONCAT22(local_6, uVar3),
                            CONCAT22(local_c, extraout_DX),
                        );
                        return;
                    }
                }
                3 => {
                    uVar2 = (iVar7 + 8);
                    paVar4 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
                    local_10 = &paVar4.field_0xc;
                    local_c = &paVar4.field_0x10;
                    if (0 < local_c) {
                        local_c = 0;
                        puVar10 = pass1_1030_73a8(CONCAT22(in_DX, paVar4));
                        uVar11 = pass1_1028_bb24(puVar10);
                        local_8 = (uVar11 >> 0x10);
                        local_a = uVar11;
                        puVar5 = &local_10;
                        pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_SS, puVar5), uVar11);
                        uVar12 = extraout_DX_00;
                        local_6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, puVar5, extraout_DX_00);
                        uVar2 = (param_1 + 10);
                        pass1_1010_c3c2(uVar2, (uVar2 >> 0x10), 0, CONCAT22(uVar12, local_6));
                        _local_2e = CONCAT22(extraout_DX_01, local_6);
                        paVar4 = local_6;
                        load_string_1010_847e(
                            _g_struct_73_1050_14cc,
                            (_g_struct_73_1050_14cc >> 0x10),
                            0x439,
                        );
                        local_4 = extraout_DX_02;
                        wsprintf16(
                            valist,
                            CONCAT22(local_6, uVar3),
                            CONCAT22(paVar4, extraout_DX_02),
                        );
                        uVar6 = extraout_DX_01;
                        // goto LAB_1010_8def;
                    }
                }
                _ => {}
                // goto switchD_1010_8e11_caseD_4;
                5 | 8 | 9 | 0xb => {
                    uVar12 = 0x43a;
                    // goto LAB_1010_8ea5;
                }
            }
            uVar12 = 0x438;
        } else {
            if (iVar1 == 2) {
                iVar1 = (iVar7 + 4);
                if ((iVar1 == 4) || (iVar1 == 0xc)) {
                    uVar2 = (iVar7 + 8);
                    local_6 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
                    uVar2 = (param_1 + 10);
                    pass1_1010_c3c2(uVar2, (uVar2 >> 0x10), 0, CONCAT22(in_DX, local_6));
                    _local_2e = CONCAT22(extraout_DX_03, local_6);
                    paVar4 = local_6;
                    load_string_1010_847e(
                        _g_struct_73_1050_14cc,
                        (_g_struct_73_1050_14cc >> 0x10),
                        0x43b,
                    );
                    local_4 = extraout_DX_04;
                    wsprintf16(
                        valist,
                        CONCAT22(local_6, uVar3),
                        CONCAT22(paVar4, extraout_DX_04),
                    );
                    uVar6 = extraout_DX_03;
                    // LAB_1010_8def:
                    error_check_1000_17ce((_local_2e & 0xffff | uVar6 << 0x10));
                    return;
                }
                uVar12 = 0x43c;
            } else {
                uVar12 = 0x5d9;
            }
        }
    }
    // LAB_1010_8ea5:
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        param_2,
        uVar12,
    );
    // switchD_1010_8e11_caseD_4:
    return;
}

pub fn str_fn_1010_c446(param_1: u32, param_2: u32, param_3: u32) {
    let mut iVar1: i32;
    let pcVar2: *mut libc::c_char;
    let in_DX: *mut astruct_199;
    let mut uVar3: u32;
    let pcVar4: *mut libc::c_char;
    let mut a: u16;
    let mut b: u16;
    let mut in_resource_id: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = param_2;
    if (param_2 == 0) {
        process_struct_1000_179c(0x100, in_DX);
        local_6 = param_2 & 0xffff | ZEXT24(in_DX) << 0x10;
    }
    uVar3 = pass1_1030_73a8(param_3);
    pass1_1010_dd5e(param_1, (param_1 >> 0x10), param_3);
    iVar1 = (uVar3 + 0x12);
    if (6 < iVar1 - 3) {
        return;
    }
    a = _g_struct_73_1050_14cc;
    b = (_g_struct_73_1050_14cc >> 0x10);
    match (iVar1) {
        _ => in_resource_id = 0x5f4,
        6 => {
            load_string_1010_84e0(a, b, 0x3ff, local_6, 0x531);
            local_16 = get_string_index_1000_3da4(local_6);
            pcVar2 = local_16;
            load_string_1010_847e(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x61e,
            );
            pcVar4 = s_____s__lu_1050_38d7;
            // goto LAB_1010_c4f9;
        }
        7 | 9 => in_resource_id = 0x531,
        8 => {
            load_string_1010_84e0(a, b, 0x3ff, local_6, 0x5f5);
            local_16 = get_string_index_1000_3da4(local_6);
            pcVar2 = local_16;
            load_string_1010_847e(
                _g_struct_73_1050_14cc,
                (_g_struct_73_1050_14cc >> 0x10),
                0x61e,
            );
            pcVar4 = s_____s__lu_1050_38cd;
            // LAB_1010_c4f9:
            string_fn_1000_3f9c(
                (local_6 + local_16),
                (local_6 >> 0x10),
                pcVar4,
                &g_alloc_addr_1050_1050,
                pcVar2,
            );
            return;
        }
    }
    load_string_1010_84e0(a, b, 0x3ff, local_6, in_resource_id);
    return;
}

pub fn load_string_1010_de78(param_1: *mut libc::c_char, param_2: u32) {
    let mut in_AX: i32;
    let mut in_resource_id: u16;

    *(param_1 + 0x13c) = 0;
    pass1_1030_809c(param_2);
    if (in_AX == 0) {
        in_resource_id = 0x531;
    } else {
        in_resource_id = 0x5f4;
    }
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        (param_1 & 0xffff0000 | (param_1 + 0x13c)),
        in_resource_id,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn wsprintf_1018_35b0(param_1: *mut astruct_298) {
    let puVar1: *mut u32;
    let piVar2: *mut i32;
    let mut uVar3: i32;
    let ppcVar4: fn();
    let mut uVar5: u32;
    let mut uVar6: u16;
    let puVar7: *mut u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut unaff_SS: u16;
    let mut bVar10: bool;
    let mut uVar11: u32;
    let mut local_1c: u32;
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

    uVar11 = pass1_1030_8326();
    local_4 = (uVar11 >> 0x10);
    local_6 = uVar11;
    uVar9 = (param_1 >> 0x10);
    uVar8 = param_1;
    puVar1 = (uVar8 + 0x140);
    unsafe {
        bVar10 = *puVar1 < local_4;
    }
    let pu_var1_val = unsafe { *puVar1 };
    if ((bVar10) || (bVar10 || pu_var1_val == local_4 && ((uVar8 + 0x13e) < local_6))) {
        uVar6 = (uVar8 + 0x13c);
        if ((uVar8 + 0x13a) < uVar6) {
            string_fn_1018_3b9e(param_1, (uVar8 + 0x12e));
            local_a = uVar6;
            local_8 = extraout_DX;
            pass1_1018_427c(uVar8, uVar9);
            _local_e = CONCAT22(extraout_DX_00, uVar6);
            pass1_1018_3e8c(
                uVar8,
                uVar9,
                CONCAT22(unaff_SS, &local_12),
                CONCAT22(unaff_SS, &local_10),
            );
            if (_local_e < local_12) {
                local_12 = local_e;
            }
            uVar3 = (uVar8 + 0x138);
            puVar7 = (uVar8 + 0x136);
            if ((uVar3 | puVar7) != 0) {
                unsafe {
                    ppcVar4 = *puVar7;
                }
                (**ppcVar4)(0x30, puVar7, uVar3, 1);
            }
            pass1_1018_435e(
                uVar8,
                (param_1 >> 0x10),
                _local_e,
                (_local_e >> 0x10),
                local_12,
                local_10,
            );
            (uVar8 + 0x136) = puVar7;
            (uVar8 + 0x138) = extraout_DX_01;
            piVar2 = (uVar8 + 0x13a);
            unsafe {
                *piVar2 = *piVar2 + 1;
            }
            uVar5 = (uVar8 + 0x136);
            uVar5 = (uVar5 + 8);
            wsprintf16(
                (uVar8 + 0x22),
                CONCAT22(0x4165, uVar9),
                CONCAT13((uVar5 >> 8), CONCAT12(uVar5, 0x1050)),
            );
            return;
        }
        (uVar8 + 0x13e) = local_6;
        (uVar8 + 0x140) = local_4;
        (uVar8 + 0x13a) = 0;
        pass1_fn_1008_612e(8, 0xc);
        (uVar8 + 0x13c) = local_6;
    }
    return;
}

pub fn string_fn_1018_3b9e(in_struct_a: *mut astruct_298, in_struct_b: *mut astruct_566) {
    let ppVar1: *mut pass1_struct_2;
    let local_AX_89: *mut u8;
    let ptr_a_1: *mut u8;
    let mut uVar2: u16;
    let struct_a_2: *mut astruct_298;
    let struct_a_1: *mut astruct_298;
    let mut uVar3: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u32;
    let struct_b_1: *mut astruct_199;

    local_6 = 0;
    struct_a_1 = (in_struct_a >> 0x10);
    struct_a_2 = in_struct_a;
    struct_b_1 = struct_a_2.astruct_199_ptr_x122;
    uVar3 = pass1_1008_e852(struct_b_1, (struct_b_1 >> 0x10), struct_a_2.u32_x126);
    ptr_a_1 = (uVar3 >> 0x10);
    ppVar1 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), uVar3);
    _local_e = CONCAT22(ptr_a_1, ppVar1);
    match (in_struct_b) {
        0x188 => {
            if (&struct_a_2.astruct99_0xa == 0) {
                process_struct_1008_d3ae(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.astruct99_0xa;
            uVar2 = struct_a_2.field_0xc
        }
        0x189 => {
            if (&struct_a_2.field_0xe == 0) {
                wsprintf_1008_d4f6(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.field_0xe;
            uVar2 = struct_a_2.field_0x10
        }
        0x18a => {
            if (&struct_a_2.field_0x12 == 0) {
                wsprintf_1008_d1c6(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.field_0x12;
            uVar2 = struct_a_2.field_0x14
        }
        0x18b => {
            if (&struct_a_2.field_0x16 == 0) {
                pass1_1008_cfa0(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.field_0x16;
            uVar2 = struct_a_2.field_0x18
        }
        0x18c => {
            if (&struct_a_2.field_0x1a == 0) {
                process_struct_1008_cda2(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.field_0x1a;
            uVar2 = struct_a_2.field_0x1c
        }
        0x18d => {
            if (&struct_a_2.field_0x1e == 0) {
                process_struct_1008_cbc4(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.field_0x1e;
            uVar2 = struct_a_2.field_0x20
        }
        // default:
        // goto switchD_1018_3cbf_caseD_6;
    }
    local_6 = CONCAT22(uVar2, local_AX_89);
    // switchD_1018_3cbf_caseD_6:
    return local_6;
}

pub fn pass1_1040_29c2(
    param_1: *mut astruct_346,
    param_2: u32,
    param_3: *mut u8,
) -> *mut astruct_346 {
    let mut iVar1: i32;
    let mut uVar2: u16;
    let mut uVar3: u32;

    process_struct_1040_b0bc(param_1, param_2, CONCAT22(param_3, 0x157));
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1 = (s_add74_wav_1050_2e20 + 6);
    (iVar1 + 2) = &PTR_LOOP_1050_1040;
    uVar3 = load_str_1010_84ac(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x64b,
    );
    (iVar1 + 0x94) = uVar3;
    (iVar1 + 0x96) = (uVar3 >> 0x10);
    uVar3 = load_str_1010_84ac(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x64a,
    );
    (iVar1 + 0x98) = uVar3;
    (iVar1 + 0x9a) = (uVar3 >> 0x10);
    return param_1;
}

pub fn load_str_1038_81be(param_1: u32) {
    let mut in_AX: u16;
    let in_DX: *mut astruct_199;
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: [u8; 258];
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_DX);
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_DX, in_AX),
        0x80b,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x80c,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x80d,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x80e,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x80f,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    uVar1 = (param_1 >> 0x10);
    MessageBox16(
        0,
        CONCAT22(unaff_SS, local_206),
        CONCAT22(in_DX, in_AX),
        (param_1 + 6),
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_DX, in_AX),
        0x810,
    );
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x811,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    load_string_1010_84e0(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_SS, local_104),
        0x812,
    );
    process_string_1000_3cea(CONCAT22(in_DX, in_AX), CONCAT22(unaff_SS, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_SS, local_206),
        CONCAT22(in_DX, in_AX),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_DX, in_AX));
    return;
}

pub fn pass1_1038_0ba6(param_1: *mut astruct_500) -> *mut astruct_500 {
    let local_BX_15: *mut astruct_1053;
    let mut unaff_BP: u16;
    let mut uVar1: u16;
    let ppVar2: *mut pass1_struct_1;

    pass1_1028_d1dc(param_1, (s_fem36_wav_1050_270c + 3));
    uVar1 = (param_1 >> 0x10);
    local_BX_15 = param_1;
    &local_BX_15.field_0x108 = 0;
    param_1.a = s_198_flc_1050_1c2e;
    local_BX_15.field_0x2 = &PTR_LOOP_1050_1038;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | &local_BX_15.field_0x8),
        s_SCMove_1050_59d8,
    );
    ppVar2 = process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(unaff_BP, 2));
    local_BX_15.field_0x108 = ppVar2;
    local_BX_15.field_0x10a = (ppVar2 >> 0x10);
    return param_1;
}

pub fn pass1_1030_eb50(struct_a: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(struct_a, s_547_bmp_1050_1f3f);
    struct_a.a = 0xecb2;
    (struct_a + 2) = 0x1030;
    copy_string_1000_3d3e(
        (struct_a & 0xffff0000 | (struct_a + 8)),
        s_SCMines_1050_59c6,
    );
    return struct_a;
}

pub fn pas1_1030_e8a0(param_1: *mut astruct_500, param_2: u32, param_3: u16, param_4: u32) {
    let local_BX_19: *mut astruct_1035;
    let pcVar1: *mut libc::c_char;

    pass1_1028_d1dc(param_1, (s_fem36_wav_1050_270c + 4));
    pcVar1 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    local_BX_19.field_0x108 = param_2;
    local_BX_19.field_0x10c = param_4;
    local_BX_19.field_0x110 = param_3;
    param_1.a = 0xeb40;
    local_BX_19.field_0x2 = 0x1030;
    string_fn_1000_3f9c(
        &local_BX_19.field_0x8,
        pcVar1,
        s_SCMoveBas_to_0x_08lx_1050_59b0,
        &g_alloc_addr_1050_1050,
        local_BX_19.field_0x10c,
    );
    return;
}

pub fn pass1_1030_e79a(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, 0xf9f);
    param_1.a = 0xe890;
    (param_1 + 2) = 0x1030;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCKillRebelColony_1050_599e,
    );
    return param_1;
}

pub fn pass1_1030_e63e(param_1: *mut astruct_500, param_2: u16) -> *mut astruct_500 {
    let local_8: *mut astruct_500;
    let local_6: *mut astruct_500;

    local_8 = param_1;
    local_6 = (param_1 >> 0x10);
    pass1_1028_d1dc(param_1, 0xf9f);
    &local_8.field_0x108 = param_2;
    param_1.a = 0xe78a;
    local_8.b = 0x1030;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&local_8.field_0x8)),
        s_SCKillColony_1050_5990,
    );
    return param_1;
}

pub fn pass1_1030_e4fa(param_1: *mut astruct_500, param_2: u32) {
    let local_BX_19: *mut astruct_500;
    let pcVar1: *mut libc::c_char;

    pass1_1028_d1dc(param_1, 0x3e80);
    pcVar1 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    local_BX_19.field_0x108 = param_2;
    param_1.a = 0xe62e;
    local_BX_19.b = 0x1030;
    string_fn_1000_3f9c(
        &local_BX_19.field_0x8,
        pcVar1,
        s_SCKillBldg__0x_08lx_1050_597c,
        &g_alloc_addr_1050_1050,
        local_BX_19.field_0x108,
    );
    return;
}

pub fn pass1_1030_e09e(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, s_fem133_wav_1050_2af7);
    param_1.a = 0xe2ae;
    (param_1 + 2) = 0x1030;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCAiInput_1050_5972,
    );
    return param_1;
}

pub fn pass1_1030_dfcc(param_1: *mut u8) -> u16 {
    let mut local_4: u16;
    let temp_5f890d5eff: *mut astruct_1016;

    temp_5f890d5eff = (param_1 + 0xc);
    if (temp_5f890d5eff == s_New_failed_in_Op__Op__DialogHand_1050_0073) {
        // LAB_1030_dfde:
        local_4 = 1;
    } else {
        if (temp_5f890d5eff != (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 1)) {
            if (temp_5f890d5eff == (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 2)) {
                return 3;
            }
            if (temp_5f890d5eff == (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 4)) {}
            // goto LAB_1030_dfde;
            if (temp_5f890d5eff != (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 5)) {
                return 0;
            }
        }
        local_4 = 2;
    }
    return local_4;
}

pub fn pass1_1030_b9da(param_1: *mut astruct_965, param_2: u32) {
    let plVar1: *mut long;
    let mut uVar2: u32;
    let mut in_EAX: u32;
    let mut uVar3: u32;
    let struct_a: *mut astruct_199;
    let mut uVar4: i32;
    let mut uVar5: i32;
    let mut iVar6: i32;
    let mut in_EDX: u32;
    let local_BX_5: *mut astruct_965;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    struct_a = in_EDX;
    uVar7 = (param_1 >> 0x10);
    local_BX_5 = param_1;
    if (&local_BX_5.field_0xe == 0) {
        process_struct_1000_179c(10, struct_a);
        uVar4 = struct_a | in_EAX;
        in_EDX = uVar4;
        if (uVar4 == 0) {
            &local_BX_5.field_0xe = 0;
        } else {
            pass1_1020_ba3e((in_EAX & 0xffff | ZEXT24(struct_a) << 0x10), 5, 5);
            &local_BX_5.field_0xe = in_EAX;
            local_BX_5.field_0x10 = in_EDX;
        }
        local_BX_5.field_0x12 = 0;
    }
    local_4 = 4;
    while (uVar4 = in_EDX, local_4 < 0xe) {
        pass1_1030_7c28(param_2, local_4);
        uVar5 = uVar4 | in_EAX;
        in_EDX = uVar5;
        if (uVar5 != 0) {
            uVar3 = 100 - local_BX_5.field_0x12;
            in_EDX = uVar3 >> 0x10;
            local_c = (in_EAX & 0xffff);
            if ((in_EAX & 0xffff | uVar4 << 0x10) < uVar3) {
                uVar3 = in_EAX & 0xffff;
                in_EDX = uVar4;
            }
            uVar5 = uVar3;
            iVar6 = in_EDX;
            in_EAX = uVar3 & 0xffff | in_EDX << 0x10;
            pass1_1030_7d1c(
                param_2,
                local_c - uVar5,
                CONCAT22(local_4, (uVar4 - iVar6) - (local_c < uVar5)),
            );
            uVar2 = &local_BX_5.field_0xe;
            pass1_1020_bb8a(uVar2, (uVar2 >> 0x10), uVar5, iVar6, local_4);
            plVar1 = &local_BX_5.field_0x12;
            unsafe {
                *plVar1 = *plVar1 + in_EAX;
            }
            uVar2 = (param_2 + 4);
            uVar8 = uVar2;
            uVar9 = (uVar2 >> 0x10);
            pass1_1020_c0ca(local_4);
            wvsprintf_FUN_1030_840a(
                s_truck_0x_08lx_loaded__ld_of__s_f_1050_576c,
                &g_alloc_addr_1050_1050,
                local_BX_5.field_0x4,
                uVar5,
                iVar6,
                in_EAX,
                in_EDX,
                uVar8,
                uVar9,
            );
            if (99 < local_BX_5.field_0x12) {
                break;
            }
        }
        local_4 = local_4 + 1;
    }
    if (local_BX_5.field_0x12 != 0) {
        return;
    }
    return;
}

pub fn pass1_1030_bb0e(param_1: u32, param_2: *mut astruct_493) {
    let mut u_var1: u32;
    let mut uVar2: i32;
    let mut uvar3: u16;
    let mut uVar4: u32;
    let mut in_DX: i32;
    let mut extraout_DX: i32;
    let mut uVar5: i32;
    let mut uVar6: i32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uVar2 = pass1_1030_7bee(param_2);
    if (uVar2 != 0) {
        return;
    }
    pass1_1030_b9b2(param_1);
    _local_6 = CONCAT22(in_DX, uVar2);
    if ((in_DX | uVar2) != 0) {
        local_8 = 4;
        uVar5 = in_DX | uVar2;
        while (local_8 < 0x25) {
            uVar4 = pass1_1020_bae6(uVar2, CONCAT22(local_8, in_DX));
            uVar6 = uVar5 | uVar4;
            if (uVar6 != 0) {
                pass1_1030_7ddc(param_2, uVar4 & 0xffff | uVar5 << 0x10, local_8);
                uVar3 = pass1_1030_7bee(param_2);
                if (uVar3 != 0) {
                    return;
                }
                uVar1 = (param_2 + 4);
                uVar7 = uVar1;
                uVar8 = (uVar1 >> 0x10);
                pass1_1020_c0ca(local_8);
                wvsprintf_FUN_1030_840a(
                    s_truck_0x_08lx_unloaded__ld_of__s_1050_5798,
                    &g_alloc_addr_1050_1050,
                    (param_1 + 4),
                    uVar4,
                    uVar5,
                    uVar3,
                    uVar6,
                    uVar7,
                    uVar8,
                );
                pass1_1020_bb8a(uVar2, in_DX, 0, 0, local_8);
                uVar6 = extraout_DX;
            }
            local_8 = local_8 + 1;
            uVar5 = uVar6;
        }
        if (_local_6 != 0x0) {
            pass1_1020_ba7e(_local_6);
            error_check_1000_17ce(_local_6);
        }
    }
    return;
}

pub fn wvsprintf_FUN_1030_840a(param_1: u32) {
    let puVar1: *mut u8;
    let mut in_DX: u16;
    let mut unaff_SS: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    if (PTR_LOOP_1050_574c != 0x0) {
        puVar1 = &stack0x0008;
        local_6 = puVar1;
        local_4 = unaff_SS;
        if (PTR_LOOP_1050_5750 == 0x0) {
            pass1_fn_1000_2b3c(
                s_simres_out_1050_5758,
                &g_alloc_addr_1050_1050,
                0x5756,
                &g_alloc_addr_1050_1050,
                in_DX,
            );
            _PTR_LOOP_1050_5752 = CONCAT22(in_DX, puVar1);
            PTR_LOOP_1050_5750 = (&PTR_LOOP_1050_0000 + 1);
        }
        wvsprintf16(
            local_6,
            CONCAT22(param_1, local_4),
            CONCAT22(local_106, (param_1 >> 0x10)),
        );
        pass1_fn_1000_2b5c(
            _PTR_LOOP_1050_5752,
            (_PTR_LOOP_1050_5752 >> 0x10),
            s__s_1050_5763,
            &g_alloc_addr_1050_1050,
        );
        pass1_fn_1000_2f48(_PTR_LOOP_1050_5752, (_PTR_LOOP_1050_5752 >> 0x10));
    }
    return;
}

pub fn pass1_1030_5ff6(struct_a: *mut astruct_912) {
    astruct_913 * *ppaVar1;
    let ppcVar2: fn();
    let mut uVar3: u32;
    let in_AX: *mut u16;
    let ppVar4: *mut pass1_struct_2;
    let pcVar5: *mut libc::c_char;
    let mut uVar6: u32;
    let in_DX: *mut astruct_199;
    let paVar7: *mut astruct_199;
    let extraout_DX: *mut astruct_199;
    let extraout_DX_00: *mut astruct_199;
    let struct_b: *mut astruct_912;
    let struct_b_hi: *mut astruct_912;
    let mut uVar8: u16;
    let unaff_SS: *mut libc::c_char;
    let mut local_6c: [u8; 88];
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: *mut astruct_913;

    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    paVar7 = in_DX;
    if (&struct_b.field_0xc == 0) {
        process_struct_1000_179c(0x18, in_DX);
        paVar7 = (in_DX | in_AX);
        if (paVar7 == 0x0) {
            &struct_b.field_0xc = 0;
        } else {
            pass1_1030_1cd8(CONCAT22(in_DX, in_AX), 5, 5);
            struct_b.field_0xc = in_AX;
            &struct_b.field_0xe = extraout_DX;
            paVar7 = extraout_DX;
        }
    }
    local_4 = 0x0;
    while (
        uVar3 = struct_b.field_0x10,
        ppaVar1 = (uVar3 + 10),
        local_4 <= *ppaVar1 && *ppaVar1 != local_4,
    ) {
        uVar3 = struct_b.field_0x10;
        uVar3 = (uVar3 + 0xc);
        uVar6 = SEXT24((uVar3 + local_4 * 2));
        pass1_1028_e2e0(_PTR_LOOP_1050_65e2);
        _local_c = uVar6 & 0xffff | ZEXT24(paVar7) << 0x10;
        ppcVar2 = (&struct_b.field_0xc + 8);
        ppcVar2(
            &PTR_LOOP_1050_1028,
            &struct_b.field_0xc,
            uVar6,
            paVar7,
            local_4,
            0,
        );
        paVar7 = extraout_DX_00;
        ppVar4 = pass1_1030_8344(_g_bool_1050_5748, (_g_bool_1050_5748 >> 0x10), _local_c);
        _local_10 = CONCAT22(paVar7, ppVar4);
        local_14 = &ppVar4.field_0x10;
        if ((local_14 + 2) == 0) {
            string_fn_1000_3f9c(
                local_6c,
                unaff_SS,
                s__s__d_1050_573a,
                &g_alloc_addr_1050_1050,
                struct_b.field_0x10,
            );
            pcVar5 = local_6c;
            pass1_fn_1008_60e8(pcVar5);
            uVar8 = (local_14 >> 0x10);
            *(local_14 + 2) = pcVar5;
            (local_14 + 4) = paVar7;
        }
        local_4 = &local_4.field_0x1;
    }
    return;
}

pub fn pass1_1030_532e(param_1: *mut astruct_500, param_2: u32) {
    let local_struct_1: *mut astruct_500;
    let pcVar1: *mut libc::c_char;
    let mut local_a: u16;

    pass1_1028_d1dc(param_1, 0x32c7);
    pcVar1 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    local_struct_1.field_0x108 = param_2;
    param_1.a = 0x55ee;
    local_struct_1.b = 0x1030;
    string_fn_1000_3f9c(
        &local_struct_1.field_0x8,
        pcVar1,
        s_SCSelect__u___d_1050_5726,
        &g_alloc_addr_1050_1050,
        &local_struct_1.c,
    );
    return;
}

pub fn pass1_1030_521c(struct_a: *mut astruct_500, param_2: u32) {
    let struct_b: *mut astruct_894;
    let pcVar1: *mut libc::c_char;

    pass1_1028_d1dc(struct_a, 0x32c7);
    pcVar1 = (struct_a >> 0x10);
    struct_b = struct_a;
    struct_b.field_0x108 = param_2;
    struct_a.a = 0x55fe;
    struct_b.field_0x2 = 0x1030;
    string_fn_1000_3f9c(
        &struct_b.field_0x8,
        pcVar1,
        s_SCGenKids_0x_08lx_1050_5714,
        &g_alloc_addr_1050_1050,
        param_2,
    );
    return;
}

pub fn pass1_1030_5164(
    string_a: *mut libc::c_char,
    string_b: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut uVar1: i32;
    let mut iVar2: i32;
    let mut local_SS__1: u16;
    let lVar3: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut array_a: [u8; 8];

    pass1_1008_5784(CONCAT22(local_SS__1, array_a), (string_a + 0x568));
    while {
        lVar3 = pass1_1008_5b12(CONCAT22(local_SS__1, array_a));
        if (lVar3 == 0) {
            return string_b;
        }
        uVar1 = string_a + 0x168;
        copy_string_1000_3d3e((string_a & 0xffff0000 | uVar1), *(lVar3 + 4));
        process_string_1000_3cea((string_a & 0xffff0000 | uVar1), string_b);
        iVar2 = dos3_call_1000_51aa(uVar1);
        iVar2 != 0
    } {}
    return (string_a & 0xffff0000 | uVar1);
}

pub fn pass1_1030_4dbc(param_1: u32, param_2: u32, param_3: libc::c_long) {
    let plVar1: *mut long;
    let puVar2: *mut u32;
    let lVar3: u32;
    let mut uVar4: i32;
    let local_BX_9: *mut astruct_891;
    let mut uVar5: u16;
    let mut local_6: u32;

    local_BX_9 = param_1;
    uVar5 = (param_1 >> 0x10);
    if (0 < param_3) {
        &local_BX_9.field_0x160 = param_2;
        local_BX_9.field_0x164 = param_3;
    }
    if ((&local_BX_9.field_0x160 == 0)
        || (
            lVar3 = local_BX_9.field_0x164,
            plVar1 = &local_BX_9.field_0x164,
            unsafe { *plVar1 = *plVar1 + -1 },
            lVar3 == 0,
        ))
    {
        &local_BX_9.field_0x160 = 0;
    } else {
        uVar4 = get_string_index_1000_3da4(*&local_BX_9.field_0x160);
        puVar2 = &local_BX_9.field_0x160;
        unsafe {
            *puVar2 = *puVar2 + uVar4 + 2;
        }
    }
    return;
}

pub fn pass1_1030_4594(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let in_DX: *mut astruct_199;
    let mut extraout_DX: u16;
    let local_BX_64: *mut astruct_883;
    let mut iVar3: i32;
    let mut uVar4: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    uVar2 = (param_1_00 - 1);
    process_struct_1000_179c(0x10, in_DX);
    local_8 = uVar2 & 0xffff | ZEXT24(in_DX) << 0x10;
    if ((in_DX | uVar2) == 0) {
        local_8 = 0;
    } else {
        zero_list_1008_3e38(CONCAT22(in_DX, uVar2 + 4));
        uVar2 = local_8;
    }
    uVar1 = uVar2;
    local_BX_64 = ((param_1_00 - 1) * 0x12);
    load_str_1010_84ac(
        _g_struct_73_1050_14cc,
        (_g_struct_73_1050_14cc >> 0x10),
        (local_BX_64 + 0x51b8),
    );
    uVar4 = (local_8 >> 0x10);
    iVar3 = local_8;
    local_8 = uVar1;
    (iVar3 + 2) = extraout_DX;
    (iVar3 + 10) = (local_BX_64 + 0x51ba);
    pass1_1008_3e76(
        (local_8 & 0xffff0000 | (iVar3 + 4)),
        (local_BX_64 + 0x51c0),
        (local_BX_64 + 0x51be),
        (local_BX_64 + 0x51bc),
    );
    (iVar3 + 0xc) = local_BX_64 + 0x51c2;
    (iVar3 + 0xe) = &g_alloc_addr_1050_1050;
    return;
}

pub fn pass1_1028_ae66(param_1: *mut astruct_500, param_2: u32, param_3: u32, param_4: u32) {
    let local_BX_19: *mut astruct_500;
    let mut uVar1: u16;

    pass1_1028_d1dc(param_1, 0x1387);
    uVar1 = (param_1 >> 0x10);
    local_BX_19 = param_1;
    local_BX_19.field_0x108 = param_4;
    local_BX_19.field_0x10c = param_3;
    local_BX_19.field_0x110 = param_2;
    &local_BX_19.field_0x114 = 0;
    param_1.a = 0xb0ce;
    local_BX_19.b = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&local_BX_19.field_0x8)),
        s_SCStarve_1050_5156,
    );
    return;
}

pub fn pass1_1028_acb6(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, 0x3e7f);
    param_1.a = 0xae56;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCSetup_1050_5124);
    return param_1;
}

pub fn pass1_1028_acec() -> u16 {
    let mut unaff_SS: u16;
    let paVar1: *mut astruct_1123;
    let paVar2: *mut astruct_393;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_14),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    loop {
        paVar1 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_14));
        if (paVar1 == 0x0) {
            break;
        }
        wvsprintf_FUN_1030_840a();
        if ((paVar1 + 0x200) != 0x8000002) {
            pass1_1038_5464(paVar1);
            pass1_1038_56d6(paVar1, 1);
        }
    }
    local_14 = s_1_1050_389a;
    local_12 = &PTR_LOOP_1050_1008;
    pass1_1028_dc52(
        CONCAT22(unaff_SS, &local_14),
        (&PTR_LOOP_1050_0000 + 1),
        0,
        0x800,
    );
    loop {
        paVar2 = pass1_1028_e4ec(CONCAT22(unaff_SS, &local_14));
        if (paVar2 == 0x0) {
            break;
        }
        pass1_1030_2690(paVar2);
    }
    return 1;
}

pub fn pass1_1028_ab32(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(
        param_1,
        (s__P__P__P__P__P__P__P__P__P__P__P_1050_2e35 + 0xaa),
    );
    param_1.a = 0xaca6;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCRchSched_1050_5118,
    );
    return param_1;
}

pub fn pass1_1028_a9be(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, 0x176f);
    param_1.a = 0xab22;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCPower_1050_5110);
    return param_1;
}

pub fn pass1_1028_a866(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, 0x36af);
    param_1.a = 0xa9ae;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCProdSched_1050_5104,
    );
    return param_1;
}

pub fn pass1_1028_a706(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, 0xbb7);
    param_1.a = 0xa856;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCPrelimAlloc_1050_50f6,
    );
    return param_1;
}

pub fn pass1_1028_9ec6(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, (s_noth_bmp_1050_2321 + 6));
    param_1.a = 0xa6f6;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCPop_1050_50f0);
    return param_1;
}

pub fn pass1_1028_933c(
    param_1: *mut astruct_500,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: *mut u32,
    param_6: u16,
    param_7: u32,
    param_8: u32,
) {
    let local_BX_24: *mut astruct_500;
    let pcVar1: *mut libc::c_char;

    pass1_1028_d1dc(param_1, 0x3e8);
    pcVar1 = (param_1 >> 0x10);
    local_BX_24 = param_1;
    local_BX_24.field_0x108 = param_8;
    local_BX_24.field_0x10c = param_7;
    local_BX_24.field_0x110 = 0;
    unsafe {
        local_BX_24.field_0x114 = *param_5;
    }
    local_BX_24.field_0x118 = (param_5 + 1);
    local_BX_24.field_0x11a = param_4;
    local_BX_24.field_0x11c = param_2;
    local_BX_24.field_0x120 = 0;
    local_BX_24.field_0x11e = 0;
    local_BX_24.field_0x122 = param_3;
    param_1.a = 0x9934;
    local_BX_24.b = &PTR_LOOP_1050_1028;
    string_fn_1000_3f9c(
        &local_BX_24.field_0x8,
        pcVar1,
        s_SCPutBldg_site_0x_08lx__bldg__u__1050_50ce,
        &g_alloc_addr_1050_1050,
        param_8,
    );
    return;
}

pub fn pass1_1028_87f0(
    param_1: *mut astruct_500,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: *mut u32,
    param_6: u16,
    param_7: u32,
    param_8: u32,
) {
    let local_BX_24: *mut astruct_500;
    let pcVar1: *mut libc::c_char;

    pass1_1028_d1dc(param_1, 0x3e8);
    pcVar1 = (param_1 >> 0x10);
    local_BX_24 = param_1;
    local_BX_24.field_0x108 = param_8;
    local_BX_24.field_0x10c = param_7;
    local_BX_24.field_0x110 = 0;
    unsafe {
        local_BX_24.field_0x114 = *param_5;
    }
    local_BX_24.field_0x118 = (param_5 + 1);
    local_BX_24.field_0x11a = param_4;
    local_BX_24.field_0x11c = param_3;
    local_BX_24.field_0x11e = param_2;
    local_BX_24.field_0x122 = 0;
    local_BX_24.field_0x120 = 0;
    param_1.a = 0x8d8e;
    local_BX_24.b = &PTR_LOOP_1050_1028;
    string_fn_1000_3f9c(
        &local_BX_24.field_0x8,
        pcVar1,
        s_SinternalPutBldg_site_0x_08lx__b_1050_5046,
        &g_alloc_addr_1050_1050,
        param_8,
    );
    return;
}

pub fn pass1_1028_8888(
    param_1: *mut astruct_500,
    param_2: u16,
    param_3: u16,
    param_4: *mut u32,
    param_5: u16,
    param_6: u32,
    param_7: u32,
    param_8: u32,
) {
    let local_BX_24: *mut astruct_500;
    let pcVar1: *mut libc::c_char;

    pass1_1028_d1dc(param_1, 0x3e8);
    pcVar1 = (param_1 >> 0x10);
    local_BX_24 = param_1;
    local_BX_24.field_0x108 = param_8;
    local_BX_24.field_0x10c = param_7;
    local_BX_24.field_0x110 = param_6;
    unsafe {
        local_BX_24.field_0x114 = *param_4;
    }
    local_BX_24.field_0x118 = (param_4 + 1);
    local_BX_24.field_0x11a = param_3;
    local_BX_24.field_0x11c = 0;
    local_BX_24.field_0x11e = param_2;
    local_BX_24.field_0x122 = 0;
    local_BX_24.field_0x120 = 0;
    param_1.a = 0x8d8e;
    local_BX_24.b = &PTR_LOOP_1050_1028;
    string_fn_1000_3f9c(
        &local_BX_24.field_0x8,
        pcVar1,
        s_SinternalPutBldg2_site_0x_08lx__1050_506f,
        &g_alloc_addr_1050_1050,
        param_8,
    );
    return;
}

pub fn pass1_1028_837e(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, 0xf9f);
    param_1.a = 0x84ba;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCFillResources_1050_500c,
    );
    return param_1;
}

pub fn pass1_1028_81aa(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, (s_42_flc_1050_1b54 + 3));
    param_1.a = 0x836e;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCFactory_1050_5002,
    );
    return param_1;
}

pub fn pass1_1028_767e() {
    let paVar1: *mut astruct_493;
    let mut in_DX: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000ffec: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar1 = pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 1, 0x800);
    if ((&paVar1[0xb].field_0x8 != 0) && ((*_PTR_LOOP_1050_65e2 % 100) == 0)) {
        ppVar2 =
            process_struct_1010_20ba(_g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000ffec, 0x40));
        wsprintf_FUN_1008_b78a(ppVar2, (ppVar2 >> 0x10));
    }
    return;
}

pub fn pass1_1028_74ae(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, 0x1387);
    param_1.a = 0x819a;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCEvent_1050_4ff4);
    return param_1;
}

pub fn pass1_1028_6fc0(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, 0x3e7);
    param_1.a = 0x749e;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCEndSim_1050_4fea);
    return param_1;
}

pub fn pass1_1028_6e60(param_1: *mut astruct_500) -> *mut astruct_500 {
    pass1_1028_d1dc(param_1, 0x32c7);
    param_1.a = 0x6fb0;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCConstruct_1050_4fdc,
    );
    return param_1;
}
pub fn pass1_1028_68de(param_1: *mut astruct_500, param_2: u16, param_3: u32) {
    let mut iVar1: i32;
    let mut uVar2: u16;

    pass1_1028_d1dc(param_1, 0x3e8);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x108) = param_3;
    (iVar1 + 0x10c) = param_2;
    param_1.a = 0x6ae2;
    (iVar1 + 2) = &PTR_LOOP_1050_1028;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (iVar1 + 8)), s_SCAddSpew_1050_4fd2);
    return;
}

pub fn big_switch_statement_1020_bd80(switch_var: u16) -> u16 {
    let mut uVar1: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (switch_var) {
        1 | 6 => local_8 = 0x427,
        2 => local_8 = 0x428,
        3 | 7 => local_8 = 0x429,
        4 | 8 => local_8 = 0x425,
        5 | 9 => local_8 = 0x426,
        10 => local_8 = 0x402,
        0xb | 0x37 => local_8 = 0x418,
        0xc | 0x35 | 0x36 => local_8 = 0x42a,
        0xd => local_8 = 0x5f7,
        0xe => local_8 = 0x5f6,
        0xf => local_8 = 0x403,
        0x10 => local_8 = 0x5f8,
        0x11 => local_8 = 0x404,
        0x12 => local_8 = 0x405,
        0x13 | 0x14 | 0x15 => local_8 = 0x406,
        0x16 | 0x19 => local_8 = 0x5f9,
        0x17 | 0x1a => local_8 = 0x5fa,
        0x18 => local_8 = 0x5fb,
        0x1b | 0x1c | 0x1d => local_8 = 0x408,
        0x1e | 0x1f | 0x20 => local_8 = 0x409,
        0x21 => local_8 = 0x42c,
        0x22 | 0x23 | 0x24 => local_8 = 0x40a,
        0x25 | 0x26 | 0x27 => local_8 = 0x40b,
        0x28 | 0x29 => local_8 = 0x40c,
        0x2a | 0x2b => local_8 = 0x40d,
        0x2c => local_8 = 0x40e,
        0x2d | 0x2e => local_8 = 0x40f,
        0x2f | 0x30 => local_8 = 0x410,
        0x31 | 0x32 => local_8 = 0x411,
        0x33 | 0x34 => local_8 = 0x416,
        0x38 | 0x39 => local_8 = 0x5fc,
        0x3a | 0x3b => local_8 = 0x419,
        0x3c | 0x3d => local_8 = 0x5fd,
        0x3e => local_8 = 0x5fe,
        0x3f => local_8 = 0x5ff,
        0x40 => local_8 = 0x600,
        0x41 => local_8 = 0x601,
        0x42 | 0x46 | 0x6b => local_8 = 0x602,
        0x43 => {
            local_6 = s_bidLRoadConst_1050_4e7a;
            return local_6;
        }
        0x44 => {
            local_6 = s_bidRRoadConst_1050_4e88;
            return local_6;
        }
        0x45 => {
            local_6 = s_bidXRoadConst_1050_4e96;
            return local_6;
        }
        0x47 => local_8 = 0x42b,
        0x48 | 0x49 | 0x4a | 0x70 | 0x71 | 0x72 => local_8 = 0x603,
        0x4b => local_8 = 0x42d,
        0x4c => local_8 = 0x604,
        0x4d => local_8 = 0x605,
        0x4e => local_8 = 0x606,
        0x4f | 0x50 | 0x51 => local_8 = 0x41a,
        0x52 | 0x53 => local_8 = 0x41b,
        0x54 | 0x55 | 0x56 => local_8 = 0x41d,
        0x57 | 0x58 | 0x59 => local_8 = 0x41e,
        0x5a => local_8 = 0x41f,
        0x5b | 0x5c => local_8 = 0x421,
        0x5d | 0x5e | 0x5f => local_8 = 0x420,
        0x60 | 0x61 => local_8 = 0x607,
        0x62 | 99 => local_8 = 0x608,
        100 => local_8 = 0x609,
        0x65 => local_8 = 0x422,
        0x66 | 0x67 => local_8 = 0x423,
        0x68 | 0x69 => local_8 = 0x60a,
        0x6a => local_8 = 0x60b,
        0x6c | 0x6d => local_8 = 0x41c,
        0x6e => local_8 = 0x60c,
        0x6f => local_8 = 0x60d,
        0x73 | 0x77 => local_8 = 0x415,
        0x74 | 0x78 | 0x79 => local_8 = 0x412,
        0x75 => local_8 = 0x413,
        0x76 => local_8 = 0x414,
        0x7a => local_8 = 0x60e,
        0x7b => local_8 = 0x60f,
        0x7c => local_8 = 0x610,
        0x7d => local_8 = 0x611,
        0x7e => local_8 = 0x612,
        0x7f => local_8 = 0x613,
        0x80 => local_8 = 0x614,
        0x81 => local_8 = 0x615,
        0x82 => local_8 = 0x616,
        0x83 => local_8 = 0x617,
        0x84 => local_8 = 0x618,
        0x85 => local_8 = 0x619,
        0x86 => local_8 = 0x61a,
        0x87 => local_8 = 0x61b,
        0x88 => local_8 = 0x61c,
        0x89 => local_8 = 0x61d,
        _ => {
            local_8 = 0x424;
        }
    }
    uVar1 = load_string_1010_847e(_g_struct_73_1050_14cc, local_8);
    return uVar1;
}

pub fn pass1_1020_c0ca(param_1: u16) {
    big_switch_statement_1020_c0d8(param_1);
    return;
}

pub fn big_switch_statement_1020_c0d8(switch_var: u16) -> u16 {
    let mut uVar1: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (switch_var) {
        1 => uVar1 = 0x5b7,
        2 => uVar1 = 0x5b8,
        3 => uVar1 = 0x5b9,
        4 => uVar1 = 0x5ba,
        5 => uVar1 = 0x5bb,
        6 => uVar1 = 0x5bc,
        7 => uVar1 = 0x5bd,
        8 => uVar1 = 0x5be,
        9 => uVar1 = 0x5bf,
        10 => uVar1 = 0x5c0,
        0xb => uVar1 = 0x5c1,
        0xc => uVar1 = 0x5c2,
        0xd => uVar1 = 0x5c3,
        0xe => uVar1 = 0x5c4,
        0xf => uVar1 = 0x5c5,
        0x10 => uVar1 = 0x5c6,
        0x11 => uVar1 = 0x5c7,
        0x12 => uVar1 = 0x5c8,
        0x13 => uVar1 = 0x5c9,
        0x14 => uVar1 = 0x5ca,
        0x15 => uVar1 = 0x5cb,
        0x16 => uVar1 = 0x5cc,
        0x17 => uVar1 = 0x5cd,
        0x18 => uVar1 = 0x5ce,
        0x19 => uVar1 = 0x5cf,
        0x1a => uVar1 = 0x5d0,
        0x1b => uVar1 = 0x5d1,
        0x1c => uVar1 = 0x5d2,
        0x1d => uVar1 = 0x5d3,
        0x1e => uVar1 = 0x5d4,
        0x1f => uVar1 = 0x5d5,
        _ => uVar1 = 0x5d9,
        0x21 => uVar1 = 0x5d6,
        0x23 => uVar1 = 0x5d7,
        0x24 => {
            uVar1 = 0x5e5;
        }
    }
    uVar1 = load_string_1010_847e(_g_struct_73_1050_14cc, uVar1);
    return uVar1;
}

pub fn big_switch_statement_1020_c222(param_1: u16) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_1) {
        1 => uVar2 = 0x57f,
        2 => uVar2 = 0x580,
        3 => uVar2 = 0x581,
        4 => uVar2 = 0x5e7,
        5 => uVar2 = 0x57e,
        6 => uVar2 = 0x582,
        7 => uVar2 = 1000,
        8 => uVar2 = 0x3e9,
        9 => uVar2 = 0x3ea,
        10 => uVar2 = 0x3eb,
        0xb => uVar2 = 0x3ec,
        0xc => uVar2 = 0x3ed,
        0xd => uVar2 = 0x3ee,
        0xe => uVar2 = 0x3ef,
        0xf => uVar2 = 0x3f0,
        0x10 => uVar2 = 0x3f1,
        0x11 => uVar2 = 0x3f2,
        0x12 => uVar2 = 0x3f4,
        0x13 => uVar2 = 0x3f5,
        0x14 => uVar2 = 0x532,
        _ => {
            uVar2 = 0x5d9;
        }
    }
    uVar1 = load_string_1010_847e(_g_struct_73_1050_14cc, uVar2);
    return uVar1;
}
