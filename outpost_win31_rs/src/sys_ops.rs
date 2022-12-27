use std::ptr::null_mut;
use std::os::raw::c_char;
use crate::unk::block_1000_1000::{mem_op_1000_179c, msg_box_op_1000_1f24, pass1_1000_1f68};
use crate::unk::block_1000_2000::{mem_op_1000_21b6, pass1_1000_25a8, pass1_1000_2913, poss_str_op_1000_28dc};
use crate::unk::block_1000_3000::sys_1000_3f9c;
use crate::unk::block_1000_4000;
use crate::unk::block_1000_4000::pass1_1000_484c;
use crate::unk::block_1008_4000::pass1_1008_4772;
use crate::unk::block_1008_7000::switch_1008_73ea;
use crate::unk::block_1010_1000::pass1_1010_1f62;
use crate::unk::block_1010_6000::pass1_1010_6ca2;
use crate::unk::block_1010_7000::pass1_1010_715c;
use crate::unk::block_1010_8000::pass1_1010_8170;
use crate::unk::block_1018_4000::{pass1_1018_4dce, struct_op_1018_4cda};
use crate::dos_interrupt::swi;
use crate::draw_ops::draw_a::palette_op_1008_4e08;
use crate::file_ops::{read_file_1008_7dee, write_to_file_1008_7e1c};
use crate::globals::{DAT_1050_1050, PTR_LOOP_1050_1000, PTR_LOOP_1050_63fe};
use crate::structs::struct_57::Struct57;
use crate::structs::struct_825::Struct825;
use crate::utils::{CARRY2, CONCAT11, CONCAT22};
use crate::winapi16::{CreateDC16, DeleteDC16, DeleteObject16, FatalAppExit16, FatalExit, GLobalAlloc16, GlobalFree16, GlobalUnlock16, Rectangle16, SelectObject16, SelectPalette16, WritePrivateProfileString16};
use crate::windef16::{DEVMODEA, HDC16, HFILE16, HGDIOBJ16, HGLOBAL16, HPALETTE16};


pub fn set_err_mode_1010_8b14(mut param_1: u32, mut param_2: u32) -> u32 {
    let mut mode: u16;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut lVar4: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar3 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0xe84));
    mode = SetErrorMode16(SEM_FAILCRITICALERRORS);
    loop {
        lVar4 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        if (lVar4 == 0) {
            SetErrorMode16(mode);
            return param_2;
        }
        uVar1 = param_1 + 0xa82;
        unk_str_op_1000_3d3e((param_1 & 0xffff0000 | uVar1), *(lVar4 + 0x4));
        pass1_1000_3cea(param_1 & 0xffff0000 | uVar1, param_2);
        uVar2 = dos3_call_1000_51aa(uVar1, uVar3, 1);
        if uVar2 == 0 {
            break;
        }
    }
    SetErrorMode16(mode);
    return param_1 & 0xffff0000 | uVar1;
}

pub fn debug_print_1008_6048(mut param_1: u16, param_2: *mut c_char) {
    let mut uVar1: u16;
    let mut local_106: [u8; 0x100] = [0; 0x100];

    if (PTR_LOOP_1050_02ec.is_null() == false) {
        if (DAT_1050_02ee == 0xffff) {
            uVar1 = pass1_1000_3ec0(0x2f4, 0x1050);
            DAT_1050_02ee = ((param_1 | uVar1) != 0);
        }
        if (DAT_1050_02ee != 0) {
            wvsprintf16(
                &stack0x0008,
                CONCAT22(param_2, 0x1050),
                CONCAT22(local_106, (param_2 >> 0x10)),
            );
            OutputDebugString16(CONCAT22(0x1050, local_106));
            OutputDebugString16(s__1050_02fa);
            if (_PTR_LOOP_1050_02f0 != 0) {
                pass1_1000_2b5c(
                    _PTR_LOOP_1050_02f0,
                    (_PTR_LOOP_1050_02f0 >> 0x10),
                    0x2fd,
                    0x1050,
                );
                pass1_1000_2f48(_PTR_LOOP_1050_02f0);
            }
        }
    }
    return;
}


pub fn win_sys_op_1010_5404(mut param_1: i16, param_2: *mut Struct19, mut param_3: u16) {
    let mut piVar1: *mut i16;
    let mut ppuVar2: *mut *mut u16;
    let mut uVar3: u32;
    let mut ppcVar4: *mut *mut code;
    let mut uVar5: u32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut puVar9: *mut u16;
    let mut IVar10: i16;
    let mut uVar4: *mut astruct_821;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut in_EDX: u32;
    let mut paVar14: *mut Struct57;
    let mut paVar15: *mut Struct57;
    let mut puVar16: *mut u8;
    let mut uVar17: u16;
    let mut uVar18: u16;
    let mut paVar19: *mut Struct19;
    let mut pcVar20: *mut c_char;
    let mut puVar21: *mut u32;
    let mut in_stack_0000fd74: u16;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000fe9e: u16;
    let mut in_stack_0000fea2: u16;
    let mut uVar22: u8;
    let mut uVar23: u8;
    let mut puStack50: *mut u16;
    let mut iStack42: i16;
    let mut uStack16: u32;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut paVar13: *mut Struct57;

    uVar18 = (in_EDX >> 0x10);
    paVar19 = struct_op_1010_1d48(param_2, param_3);
    uVar11 = (paVar19 >> 0x10);
    paVar13 = CONCAT22(uVar18, uVar11);
    uVar18 = 0;
    (param_2 + 0xa) = 0;
    (param_2 + 0xe) = 0;
    (param_2 + 0x12) = 0;
    (param_2 + 0x16) = 0;
    (param_2 + 0x1a) = 0;
    (param_2 + 0x62) = 0;
    (param_2 + 0x64) = 0;
    (param_2 + 0x68) = 0;
    (param_2 + 0x6c) = 0;
    (param_2 + 0x70) = 0x1;
    (param_2 + 0x7a) = 0;
    (param_2 + 0x7c) = 0;
    (param_2 + 0x7e) = 0;
    (param_2 + 0x80) = 0;
    (param_2 + 0x82) = 0x1;
    param_2.offset_0x0 = 0x6312;
    (param_2 + 0x2) = 0x1010;
    pass1_1010_6034(uVar11, param_2);
    mem_op_1000_179c(0x101, paVar13);
    (param_2 + 0xe) = uVar18;
    (param_2 + 0x10) = paVar13;
    pass1_1000_5008((param_2 + 0xe), paVar13, 0x100);
    uVar18 = (paVar13 >> 0x10);
    uVar6 = str_op_1000_3da4(*(param_2 + 0xe));
    uVar5 = (param_2 + 0xe);
    uVar17 = (uVar5 >> 0x10);
    puVar16 = (uVar5 + uVar6);
    if (puVar16[-0x1] != '\\') {
        *puVar16 = 0x5c;
        uVar5 = (param_2 + 0xe);
        *(uVar5 + uVar6 + 1) = 0;
    }
    pcVar20 = load_string_1010_847e(_u16_1050_14cc, 0x578);
    paVar13 = CONCAT22(uVar18, (pcVar20 >> 0x10));
    pass1_1000_3cea((param_2 + 0xe), pcVar20);
    uVar7 = str_op_1008_60e8(paVar13, *(param_2 + 0xe));
    (param_2 + 0xa) = uVar7;
    (param_2 + 0xc) = paVar13;
    GetPrivateProfileString16(
        CONCAT22(paVar13, (param_2 + 0xa)),
        0x100,
        *(param_2 + 0xe),
        s_playerName_1050_148e + 0xc,
        s_rez_1050_13c0,
        s_general_1050_13b0,
    );
    if (**(param_2 + 0xe) != '\0') {
        iVar8 = pass1_1000_3e2c((param_2 + 0xe));
        puVar21 = mixed_1010_20ba(
            paVar13,
            _u16_1050_0ed0,
            CONCAT22(param_1, 0x48),
            in_stack_0000fd74,
            in_stack_0000fe98,
            in_stack_0000fe9e,
            in_stack_0000fea2,
        );
        paVar13 = (paVar13 & 0xffff0000 | puVar21 >> 0x10);
        uVar18 = (puVar21 >> 0x10);
        iStack10 = (puVar21 + 0xa);
        iStack12 = (puVar21 + 0xc);
        (param_2 + 0x62) = (iVar8 != iStack10);
    }
    GetPrivateProfileString16(
        *(param_2 + 0xa),
        0x100,
        *(param_2 + 0xe),
        s_playerName_1050_148e + 0xc,
        s_falseMap_1050_13de,
        s_general_1050_13b0,
    );
    if (**(param_2 + 0xe) != '\0') {
        param_1 = 0x1050;
        iVar8 = pass1_1000_475e((param_2 + 0xe), s_on_1050_13c4);
        if (iVar8 == 0) {
            (param_2 + 0x80) = 0x1;
        }
    }
    param_1 = 0x1050;
    GetPrivateProfileString16(
        *(param_2 + 0xa),
        0x100,
        *(param_2 + 0xe),
        s_playerName_1050_148e + 0xc,
        s_music_1050_13d2,
        s_general_1050_13b0,
    );
    if (**(param_2 + 0xe) != '\0') {
        param_1 = s_off_1050_13c8;
        iVar8 = pass1_1000_475e((param_2 + 0xe), s_off_1050_13c8);
        if (iVar8 == 0) {
            (param_2 + 0x74) = 0;
        }
    }
    param_1 = s_general_1050_13b0;
    GetPrivateProfileString16(
        *(param_2 + 0xa),
        0x100,
        *(param_2 + 0xe),
        s_playerName_1050_148e + 0xc,
        s_sound_1050_13d8,
        s_general_1050_13b0,
    );
    if (**(param_2 + 0xe) != '\0') {
        uVar3 = (param_2 + 0xe);
        param_1 = (uVar3 >> 0x10);
        iVar8 = pass1_1000_475e(uVar3, s_off_1050_13c8);
        if (iVar8 == 0) {
            (param_2 + 0x72) = 0;
        }
    }
    param_1 = 0x1050;
    GetPrivateProfileString16(
        *(param_2 + 0xa),
        0x100,
        *(param_2 + 0xe),
        s_playerName_1050_148e + 0xc,
        s_anims_1050_13cc,
        s_general_1050_13b0,
    );
    if (**(param_2 + 0xe) != '\0') {
        uVar3 = (param_2 + 0xe);
        param_1 = uVar3;
        iVar8 = pass1_1000_475e(uVar3, s_off_1050_13c8);
        if (iVar8 == 0) {
            (param_2 + 0x1e) = 0;
        }
    }
    param_1 = s_movies_1050_13e8;
    GetPrivateProfileString16(
        *(param_2 + 0xa),
        0x100,
        *(param_2 + 0xe),
        s_playerName_1050_148e + 0xc,
        s_movies_1050_13e8,
        s_general_1050_13b0,
    );
    if (**(param_2 + 0xe) != '\0') {
        param_1 = 0x1538;
        iVar8 = pass1_1000_475e((param_2 + 0xe), s_off_1050_13c8);
        if (iVar8 == 0) {
            (param_2 + 0x20) = 0;
        }
    }
    param_1 = 0x1050;
    GetPrivateProfileString16(
        *(param_2 + 0xa),
        0x100,
        *(param_2 + 0xe),
        s_playerName_1050_148e + 0xc,
        s_turns_1050_1466,
        s_general_1050_13b0,
    );
    paVar14 = paVar13;
    if (**(param_2 + 0xe) != '\0') {
        uVar6 = pass1_1000_3e2c((param_2 + 0xe));
        uVar12 = paVar13 | uVar6;
        paVar14 = (paVar13 & 0xffff0000 | uVar12);
        if (uVar12 != 0) {
            (param_2 + 0x76) = uVar6;
            (param_2 + 0x78) = paVar13;
            paVar14 = paVar13;
        }
    }
    param_1 = s_playerName_1050_148e + 0xc;
    GetPrivateProfileString16(
        *(param_2 + 0xa),
        0x100,
        *(param_2 + 0xe),
        s_playerName_1050_148e + 0xc,
        s_turnsDlgStatus_1050_146c,
        s_general_1050_13b0,
    );
    if ((**(param_2 + 0xe) != '\0') && (
        iVar8 = pass1_1000_475e((param_2 + 0xe), s_on_1050_13c4),
        iVar8 == 0,
    )) {
        (param_2 + 0x7a) = 0x1;
    }
    pcVar20 = *(param_2 + 0xe);
    param_1 = (pcVar20 >> 0x10);
    GetPrivateProfileString16(
        *(param_2 + 0xa),
        0x100,
        pcVar20,
        s_playerName_1050_148e + 0xc,
        s_savePath_1050_147c,
        s_general_1050_13b0,
    );
    if (**(param_2 + 0xe) != '\0') {
        uVar7 = str_op_1008_60e8(paVar14, *(param_2 + 0xe));
        (param_2 + 0x1a) = uVar7;
        (param_2 + 0x1c) = paVar14;
    }
    pcVar20 = *(param_2 + 0xe);
    param_1 = pcVar20;
    GetPrivateProfileString16(
        *(param_2 + 0xa),
        0x100,
        pcVar20,
        s_playerName_1050_148e + 0xc,
        s_aiName_1050_1486,
        s_general_1050_13b0,
    );
    if (**(param_2 + 0xe) != '\0') {
        uVar7 = str_op_1008_60e8(paVar14, *(param_2 + 0xe));
        (param_2 + 0x68) = uVar7;
        (param_2 + 0x6a) = paVar14;
    }
    param_1 = 0x100;
    puVar9 = GetPrivateProfileString16(
        *(param_2 + 0xa),
        0x100,
        *(param_2 + 0xe),
        s_playerName_1050_148e + 0xc,
        s_playerName_1050_148e,
        s_general_1050_13b0,
    );
    if (**(param_2 + 0xe) != '\0') {
        puVar9 = str_op_1008_60e8(paVar14, *(param_2 + 0xe));
        (param_2 + 0x6c) = puVar9;
        (param_2 + 0x6e) = paVar14;
    }
    if ((param_2 + 0x62) == 0) {
        IVar10 = GetSystemMetrics16(SM_CYCAPTION);
        iStack42 = 0x1;
        loop {
            get_private_profile_string_1010_6132(param_2, iStack42);
            iVar8 = iStack42 * 0x8 + param_2;
            if (((((iVar8 + 0x22) < 0x0) || ((iVar8 + 0x24) < 0x0)) || (
                piVar1 = (iVar8 + 0x22),
                *piVar1 != iStack10 - IVar10 && iStack10 - IVar10 <= *piVar1,
            )) || (
                puVar9 = (iStack12 - IVar10),
                ppuVar2 = (iVar8 + 0x24),
                *ppuVar2 != puVar9 && puVar9 <= *ppuVar2,
            )) {
                puVar9 = pass1_1000_4906(
                    (param_2 & 0xffff0000 | (iStack42 * 0x8 + param_2 + 0x22)),
                    NULL,
                    0x8,
                );
            }
            iStack42 += 0x1;
            if iStack42 >= 0x8 {
                break;
            }
        }
    }
    mem_op_1000_179c(0xc, paVar14);
    uVar6 = paVar14 | puVar9;
    paVar13 = (paVar14 & 0xffff0000);
    paVar15 = (paVar13 | uVar6);
    if (uVar6 == 0) {
        puVar9 = null_mut();
    } else {
        set_struct_1008_574a(CONCAT22(paVar14, puVar9));
        paVar13 = paVar15;
    }
    (param_2 + 0x64) = puVar9;
    (param_2 + 0x66) = paVar13;
    uVar5 = (param_2 + 0xe);
    pass1_1000_5008(uVar5, (uVar5 >> 0x10), 0x100);
    uVar6 = str_op_1000_3da4(*(param_2 + 0xe));
    uVar5 = (param_2 + 0xe);
    uVar18 = (uVar5 >> 0x10);
    puVar16 = (uVar5 + uVar6);
    if (puVar16[-0x1] != '\\') {
        *puVar16 = 0x5c;
        uVar5 = (param_2 + 0xe);
        *(uVar5 + uVar6 + 1) = 0;
    }
    uVar4 = str_op_1008_60e8(paVar13, *(param_2 + 0xe));
    uStack16 = CONCAT22(paVar13, uVar4);
    mem_op_1000_179c(0x8, paVar13);
    uVar6 = paVar13;
    puStack50 = CONCAT22(uVar6, uVar4);
    paVar13 = (paVar13 & 0xffff0000 | (uVar6 | uVar4));
    if ((uVar6 | uVar4) != 0) {
        *puStack50 = 0x389a;
        uVar4.field2_0x2 = 0x1008;
        uVar4.field3_0x4 = uStack16;
        *puStack50 = 0x6322;
        uVar4.field2_0x2 = 0x1010;
    }
    ppcVar4 = ((param_2 + 0x64) + 0x4);
    (**ppcVar4)();
    pcVar20 = *(param_2 + 0xe);
    param_1 = (pcVar20 >> 0x10);
    GetPrivateProfileString16(
        *(param_2 + 0xa),
        0x100,
        pcVar20,
        s_playerName_1050_148e + 0xc,
        s_opimage_1050_13f0,
        s_general_1050_13b0,
    );
    if (**(param_2 + 0xe) != '\0') {
        uVar5 = (param_2 + 0xe);
        uVar18 = uVar5;
        uVar22 = (uVar5 >> 0x10);
        uVar23 = (uVar5 >> 0x18);
        loop {
            uVar7 = pass1_1000_47a4(CONCAT13(uVar23, CONCAT12(uVar22, uVar18)), s___1050_13f8);
            if ((paVar13 | uVar7) == 0) {
                break;
            }
            unk_str_op_1000_3d3e(
                CONCAT13(0x10, CONCAT12(0x50, &param_1)),
                CONCAT22(paVar13, uVar7),
            );
            uVar6 = str_op_1000_3da4(CONCAT22(0x1050, &param_1));
            if ((&stack0xfecb)[uVar6] != '\\') {
                *(&param_1 + uVar6) = 0x5c;
                *(&param_1 + uVar6 + 1) = 0;
            }
            uVar6 = str_op_1008_60e8(paVar13, CONCAT22(0x1050, &param_1));
            uStack16 = CONCAT22(paVar13, uVar6);
            mem_op_1000_179c(0x8, paVar13);
            uVar12 = paVar13;
            puStack50 = CONCAT22(uVar12, uVar6);
            paVar13 = (paVar13 & 0xffff0000 | (uVar12 | uVar6));
            if ((uVar12 | uVar6) != 0) {
                *puStack50 = 0x389a;
                (uVar6 + 0x2) = 0x1008;
                (uVar6 + 0x4) = uStack16;
                *puStack50 = 0x6322;
                (uVar6 + 0x2) = 0x1010;
            }
            ppcVar4 = ((param_2 + 0x64) + 0x8);
            (**ppcVar4)();
            uVar18 = 0;
            uVar22 = 0;
            uVar23 = 0;
        }
    }
    return;
}


pub fn write_private_profile_str_1010_5b10(mut param_1: u16, param_2: *mut StructD) {
    let mut puVar2: *mut u32;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut in_register_0000000a: u16;
    let mut iVar10: *mut StructD;
    let mut unaff_SI: u16;
    let mut uVar10: u16;
    let mut puVar10: *mut u32;
    let mut in_stack_0000fe76: u16;
    let mut in_stack_0000ff9a: u16;
    let mut in_stack_0000ffa0: u16;
    let mut in_stack_0000ffa4: u16;
    let mut iStack12: i16;
    let mut puVar1: *mut u32;
    let mut uVar3: u16;

    uVar10 = (param_2 >> 0x10);
    iVar10 = param_2;
    param_2.address_offset_field_0x0 = 0x6312;
    iVar10.address_offset_field_0x2 = 0x1010;
    puVar10 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x48),
        in_stack_0000fe76,
        in_stack_0000ff9a,
        in_stack_0000ffa0,
        in_stack_0000ffa4,
    );
    sys_1000_3f9c(*&iVar10.field8_0xe, s__d__d_1050_149c, (puVar10 + 0xa));
    if (&iVar10.field_0x80 == 0) {
        // actualy just 0x13c8
        uVar4 = s_off_1050_13c8;
    } else {
        // actually just 0x13c4
        uVar4 = s_on_1050_13c4;
    }
    WritePrivateProfileString16(
        *&iVar10.field6_0xa,
        CONCAT22(0x1050, uVar4),
        s_falseMap_1050_13de,
        s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        *&iVar10.field6_0xa,
        *&iVar10.field8_0xe,
        s_rez_1050_13c0,
        s_general_1050_13b0,
    );
    if (&iVar10.field_0x1e == 0) {
        // actually just 0x13c8
        uVar5 = s_off_1050_13c8;
    } else {
        // actually just 0x13c4
        uVar5 = s_on_1050_13c4;
    }
    WritePrivateProfileString16(
        *&iVar10.field6_0xa,
        CONCAT22(0x1050, uVar5),
        s_anims_1050_13cc,
        s_general_1050_13b0,
    );
    if (&iVar10.field_0x74 == 0) {
        // 0x13c8
        uVar6 = s_off_1050_13c8;
    } else {
        // 0x13c4
        uVar6 = s_on_1050_13c4;
    }
    WritePrivateProfileString16(
        *&iVar10.field6_0xa,
        CONCAT22(0x1050, uVar6),
        s_music_1050_13d2,
        s_general_1050_13b0,
    );
    if (&iVar10.field_0x72 == 0) {
        // 0x13c8
        uVar7 = s_off_1050_13c8;
    } else {
        // 0x13c4
        uVar7 = s_on_1050_13c4;
    }
    WritePrivateProfileString16(
        *&iVar10.field6_0xa,
        CONCAT22(0x1050, uVar7),
        s_sound_1050_13d8,
        s_general_1050_13b0,
    );
    if (iVar10.field19_0x20 == 0) {
        // 0x13c8
        uVar8 = s_off_1050_13c8;
    } else {
        // 0x13c4
        uVar8 = s_on_1050_13c4;
    }
    WritePrivateProfileString16(
        *&iVar10.field6_0xa,
        CONCAT22(0x1050, uVar8),
        s_movies_1050_13e8,
        s_general_1050_13b0,
    );
    sys_1000_3f9c(*&iVar10.field8_0xe, s__lu_1050_14a2, &iVar10.field_0x76);
    WritePrivateProfileString16(
        *&iVar10.field6_0xa,
        *&iVar10.field8_0xe,
        s_turns_1050_1466,
        s_general_1050_13b0,
    );
    if (&iVar10.field_0x7a == 0) {
        // 0x13c8
        uVar9 = SUB42(s_off_1050_13c8, 0x0);
    } else {
        // 0x13c4
        uVar9 = SUB42(s_on_1050_13c4, 0x0);
    }
    WritePrivateProfileString16(
        *&iVar10.field6_0xa,
        CONCAT22(0x1050, uVar9),
        s_turnsDlgStatus_1050_146c,
        s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        *&iVar10.field6_0xa,
        *&iVar10.field14_0x1a,
        s_savePath_1050_147c,
        s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        *&iVar10.field6_0xa,
        *&iVar10.field_0x68,
        s_aiName_1050_1486,
        s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        *&iVar10.field6_0xa,
        *&iVar10.field_0x6c,
        s_playerName_1050_148e,
        s_general_1050_13b0,
    );
    iStack12 = 0x1;
    loop {
        caseD_13(param_2, iStack12);
        iStack12 += 0x1;
        if iStack12 >= 0x8 {
            break;
        }
    }
    fn_ptr_1000_17ce(*&iVar10.field6_0xa);
    fn_ptr_1000_17ce(*&iVar10.field8_0xe);
    fn_ptr_1000_17ce(*&iVar10.field11_0x12);
    fn_ptr_1000_17ce(*(&iVar10.field12_0x14 + 0x2));
    fn_ptr_1000_17ce(*&iVar10.field14_0x1a);
    puVar2 = &iVar10.field_0x64;
    uVar3 = &iVar10.field_0x66;
    if ((uVar3 | puVar2) != 0) {
        ppcVar3 = *puVar2;
        (**ppcVar3)(0x1000, puVar2, uVar3, 1);
    }
    fn_ptr_1000_17ce(*&iVar10.field_0x68);
    fn_ptr_1000_17ce(*&iVar10.field_0x6c);
    pass1_1010_1d80(param_2);
    return;
}


pub fn get_private_profile_string_1010_6132(param_1: *mut Struct19, mut param_2: i16)

{
  let mut uVar2: u16;
  let mut iVar1: i16;
  let mut uVar3: u16;
  let mut iVar2: i16;
  let mut uVar4: u16;
  let mut iVar3: i16;
  let mut uVar5: u16;
  let mut iVar4: i16;
  let mut in_DX: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut iVar5: *mut Struct19;
  let mut iVar6: i16;
  let mut uVar9: u16;
  let mut unaff_SS: u16;
  let mut uVar1: u32;

  uVar9 = (param_1 >> 0x10);
  iVar5 = param_1;
  GetPrivateProfileString16
            (*&iVar5.horiz_res_0xa,0x100,*&iVar5.field_0xe,s_playerName_1050_148e + 0xc,
             *(param_2 * 0x4 + 0x1446),s_windows_1050_13b8);
  if (**&iVar5.field_0xe != '\0') {
    uVar2 = pass1_1000_47a4(&iVar5.field_0xe,s___1050_14a6);
    uVar6 = in_DX | uVar2;
    if (uVar6 != 0) {
      iVar1 = pass1_1000_3e2c(CONCAT22(in_DX,uVar2));
      (&iVar5.field18_0x22)[param_2 * 0x4] = iVar1;
      uVar3 = pass1_1000_47a4(0x0,s___1050_14a8);
      uVar7 = uVar6 | uVar3;
      if (uVar7 != 0) {
        iVar2 = pass1_1000_3e2c(CONCAT22(uVar6,uVar3));
        (&iVar5.field19_0x24)[param_2 * 0x4] = iVar2;
        uVar4 = pass1_1000_47a4(0x0,s___1050_14aa);
        uVar8 = uVar7 | uVar4;
        if (uVar8 != 0) {
          iVar3 = pass1_1000_3e2c(CONCAT22(uVar7,uVar4));
          (&iVar5.field20_0x26 + param_2 * 0x2) = iVar3;
          uVar5 = pass1_1000_47a4(0x0,s___1050_14ac);
          if ((uVar8 | uVar5) != 0) {
            iVar4 = pass1_1000_3e2c(CONCAT22(uVar8,uVar5));
            (&iVar5.field20_0x26 + param_2 * 0x8 + 0x2) = iVar4;
          }
        }
      }
    }
  }
  return;
}

pub fn caseD_13(mut param_1: u32, mut param_2: i16)

{
  let mut iVar1: *mut astruct_833;

  iVar1 = (param_2 * 0x8 + param_1);
  if ((((iVar1.field34_0x22 != 0) || (iVar1.field35_0x24 != 0)) || (iVar1.field36_0x26 != 0)) ||
     (iVar1.field37_0x28 != 0)) {
    sys_1000_3f9c(*(param_1 + 0xe),s__d__d__d__d_1050_14ae,
                  (param_2 * 0x8 + param_1 + 0x22));
    WritePrivateProfileString16
              (*(param_1 + 0xa),*(param_1 + 0xe),*(param_2 * 0x4 + 0x1446),
               s_windows_1050_13b8);
  }
  return;
}

pub fn pass1_1000_27d6(mut param_1: u16) {
    let mut piVar2: *mut i16;
    let mut pcVar3: *mut c_char;
    let mut cVar4: u8;
    let mut pu_var5: *mut u16;
    u16 * *ppuVar6;
    let mut i_var7: i16;
    let mut u_var7: u16;
    let mut i_var8: i16;
    let mut pu_var8: *mut u16;
    let mut i_var9: i16;
    let mut piVar9: *mut i16;
    let mut piVar10: *mut i16;
    let mut pcVar11: *mut c_char;
    let mut piVar12: *mut i16;
    let mut b_var13: bool;
    let mut pu_var14: *mut u16;
    let mut piVar1: *mut i16;
    let mut pu_var4: *mut u16;
    let mut piVar4: *mut i16;

    let dos_env = GetDOSEnvironment16();
    let pu_var7 = (dos_env >> 0x10);
    if (dos_env != 0) {
        pu_var7 = null_mut();
    }
    i_var9 = 0;
    pcVar11 = null_mut();
    i_var7 = -0x1;
    if (pu_var7.is_null() == false) {
        cVar4 = *NULL;
        while (cVar4 != '\0') {
            loop {
                if (i_var7 == 0) {
                    break;
                }
                i_var7 += -0x1;
                pcVar3 = pcVar11;
                pcVar11 = pcVar11 + 1;
                if *pcVar3 == '\0' {
                    break;
                }
            }
            i_var9 += 0x1;
            pcVar3 = pcVar11;
            pcVar11 = pcVar11 + 1;
            cVar4 = *pcVar3;
        }
    }
    u_var7 = 0x9;
    pu_var8 = pu_var7;
    pu_var5 = exit_op_1000_2950(ctx, 0x9, pu_var7, (pcVar11 + 1) & 0xfffe);
    pu_var14 = pu_var8;
    ppuVar6 = exit_op_1000_2950(ctx, u_var7, pu_var8, (i_var9 + 1) * 0x4);
    piVar9 = null_mut();
    PTR_LOOP_1050_5fbe = ppuVar6;
    PTR_LOOP_1050_5fc0 = pu_var8;
    loop {
        if (i_var9 == 0) {
            *ppuVar6 = null_mut();
            ppuVar6[0x1] = null_mut();
            return;
        }
        b_var13 = *piVar9 == s__C_FILE_INFO__1050_5f5c;
        if (b_var13) {
            piVar12 = s__C_FILE_INFO__1050_5f5c;
            i_var8 = 0x6;
            piVar10 = piVar9;
            loop {
                if (i_var8 == 0) {
                    break;
                }
                i_var8 += -0x1;
                piVar4 = piVar12;
                piVar12 = piVar12 + 1;
                piVar1 = piVar10;
                piVar10 = piVar10 + 1;
                b_var13 = *piVar1 == *piVar4;
                if b_var13 == false {
                    break;
                }
            }
            if (!b_var13) {
                // TODO: goto LAB_1000_2867;
            }
        } else {
            //
            //            LAB_1000_2867:
            *ppuVar6 = pu_var5;
            ppuVar6[0x1] = pu_var14;
            ppuVar6 = ppuVar6 + 2;
        }
        loop {
            piVar2 = piVar9;
            piVar9 = (piVar9 + 1);
            cVar4 = piVar2;
            pu_var4 = pu_var5;
            pu_var5 = (pu_var5 + 1);
            pu_var4 = cVar4;
            if cVar4 == '\0' {
                break;
            }
        }
        i_var9 += -0x1;
    }
}


pub fn pass1_1008_91ba(param_1: *mut astruct_3) -> *mut u16 {
    let mut UVar1: u16;
    let mut iVar2: *mut astruct_3;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar2.field1_0x2 = 0x1008;
    iVar2.field2_0x4 = 0;
    set_struct_1008_574a((param_1 & 0xffff0000 | ZEXT24(iVar2 + 1)));
    param_1.field0_0x0 = 0x9416;
    iVar2.field1_0x2 = 0x1008;
    _PTR_LOOP_1050_0388 = param_1;
    UVar1 = SetTimer16(NULL, 0x1, 0x1, HWND16_1050_0396);
    if (UVar1 == 0) {
        fn_ptr_op_1000_24cd(1);
    }
    PTR_LOOP_1050_038a = (_PTR_LOOP_1050_0388 >> 0x10);
    return &param_1.field0_0x0;
}


pub fn kill_timer_1008_921c(param_1: *mut u16) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0x9416;
    (iVar1 + 0x2) = 0x1008;
    KillTimer16(0x1, HWND16_1050_0396);
    _PTR_LOOP_1050_0388 = 0;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0x6)));
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    return;
}
