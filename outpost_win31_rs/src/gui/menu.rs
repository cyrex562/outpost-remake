use std::ptr::null_mut;
use std::os::raw::c_char;
use crate::unk::block_1008_5000::win_1008_5c7c;
use crate::unk::block_1020_6000::{pass1_1020_6498, pass1_1020_64d4};
use crate::utils::CONCAT22;
use crate::{gui, winapp};
use crate::no_refs::l::pass1_1010_c3c2;
use crate::resources::load_string_1010_847e;
use crate::structs::struct_57::Struct57;
use crate::unk::block_1000_3000::unk_str_op_1000_3d3e;
use crate::unk::block_1008_c000::pass1_1008_c6ae;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1018_2000::pass1_1018_2504;
use crate::unk::block_1020_b000::pass1_1020_bd80;
use crate::unk::block_1030_7000::struct_op_1030_73a8;
use crate::unk::block_1030_8000::{pass1_1030_8308, pass1_1030_8344};
use crate::winapi16::{CheckMenuItem16, ClientToScreen16, DeleteMenu16, EnableMenuItem16, GetMenuState16, GetSubMenu16, InsertMenu16, LoadMenu16, ModifyMenu16, PostMessage16, WinAPI16_PtInRect16, TrackPopupMenu16};
use crate::winapp::winapp_e;
use crate::windef16::{BOOL16, HMENU16, HWND16, RECT16};

pub fn menu_ui_op_1020_5bf2(
    param_1: *mut astruct_52,
    param_2: INT16,
    param_3: INT16,
) -> BOOL16 {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut pIVar4: *mut INT16 = null_mut();
    //   HMENlet mut HVar5: u16;
    let mut HVar5: HMENU16;
    let mut in_DX: u16;
    let mut iVar5: *mut astruct_52;
    let mut uVar6: u16;
    let mut local_10: i16;
    let mut IStack14: i16;
    let mut iStack12: i16;
    let mut uStack10: u32;
    let mut local_6: i16;
    let mut IStack4: i16;

    local_6 = param_3;
    IStack4 = param_2;
    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    uVar2 = pass1_1020_64d4(iVar5.field243_0xf6, 0x2);
    if (uVar2 != 0) {
        uStack10 = pass1_1020_6498(iVar5.field243_0xf6, 0x2);
        in_DX = (uStack10 >> 0x10);
        BVar3 = WinAPI16_PtInRect16(CONCAT22(IStack4, local_6), uStack10);
        if (BVar3 != 0) {
            PostMessage16(0x0, 0x131, 0x111, HWND16_1050_0396);
            return 0x1;
        }
    }
    uVar2 = pass1_1020_64d4(iVar5.field243_0xf6, 0x3);
    if (uVar2 == 0) {
        return 0x0;
    }
    pIVar4 = &local_6;
    winapp_e::pt_in_rect_1020_5856(pIVar4, in_DX, param_1, CONCAT22(0x1050, pIVar4));
    iVar5.field257_0x108 = pIVar4;
    iVar5.field258_0x10a = in_DX;
    if ((in_DX | iVar5.field257_0x108) == 0) {
        return 0x0;
    }
    if (iVar5.field256_0x106 == 0) {
        HVar5 = LoadMenu16(s_TILEMENU_1050_43f0, HINSTANCE16_1050_038c);
        iVar5.field256_0x106 = HVar5;
        if (HVar5 == 0) {
            return 0x1;
        }
        HVar5 = GetSubMenu16(0x0, iVar5.field256_0x106);
        iVar5.field256_0x106 = HVar5;
        if (HVar5 == 0) {
            return 0x1;
        }
    }
    uVar1 = &iVar5.field257_0x108;
    uStack10 = (uVar1 + 0x2e);
    iStack12 = 0;
    if (uStack10 == 0x42) {
        iStack12 = 0xc9;
    } else if ((s_VrMode_1050_42ca + 0x8 + uStack10 * 0x2) == 0) {
        iStack12 = 0xc8;
    }
    if (iStack12 != 0) {
        win_1008_5c7c(uStack10, in_DX, _u16_1050_02a0, CONCAT22(iStack12, 1));
    }
    local_10 = param_3;
    IStack14 = param_2;
    ClientToScreen16(CONCAT22(0x1050, &local_10), iVar5.field8_0x8);
    TrackPopupMenu16(
        NULL,
        iVar5.field8_0x8,
        0x0,
        IStack14,
        local_10,
        0x0,
        iVar5.field256_0x106,
    );
    return 0x1;
}


pub fn menu_ui_op_1008_09ba(param_1: *mut astruct_853, param_2: HWND16, param_3: *mut RECT16) {
    let mut HVar1: HMENU16;
    let mut iVar2: *mut astruct_853;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (iVar2.field235_0xec == 0) {
        HVar1 = LoadMenu16(s_OPPOPMENU_1050_0150, HINSTANCE16_1050_038c);
        iVar2.field235_0xec = HVar1;
        if (HVar1 == 0) {
            return;
        }
        HVar1 = GetSubMenu16(0x0, iVar2.field235_0xec);
        iVar2.field235_0xec = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    ClientToScreen16(CONCAT22(0x1050, &stack0xfffa), iVar2.field8_0x8);
    HVar1 = iVar2.field235_0xec;
    TrackPopupMenu16(NULL, HWND16_1050_0396, 0x0, HVar1, 0x0, 0x0, HVar1);
    return;
}


pub fn menu_ui_op_1040_7f86(param_1: *mut astruct_855)

{
//   HMENlet mut HVar1: u16;
    let mut HVar1: HMENU16;
    let mut iVar2: *mut astruct_855;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2.field104_0x6a.is_null() == false) && (iVar2.field103_0x68 == 0)) {
        HVar1 = LoadMenu16(iVar2.field104_0x6a, HINSTANCE16_1050_038c);
        iVar2.field103_0x68 = HVar1;
        if (HVar1 == 0) {
            return;
        }
        HVar1 = GetSubMenu16(0x0, iVar2.field103_0x68);
        iVar2.field103_0x68 = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    ClientToScreen16(CONCAT22(0x1050, &stack0xfffa), iVar2.field6_0x6);
    HVar1 = iVar2.field103_0x68;
    TrackPopupMenu16(NULL, iVar2.field6_0x6, 0x0, HVar1, 0x0, 0x0, HVar1);
    return;
}


pub fn enable_menu_item_1020_6b9a(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
    param_5: HMENU16,
) {
    if (param_4 != 0) {
        return;
    }
    EnableMenuItem16(0x400, 0x0, param_5);
    return;
}

pub fn win_ui_menu_op_1020_7ad2(
    param_1: *mut astruct_854,
    param_2: HWND16,
    param_3: *mut RECT16,
) {
    //   HMENlet mut HVar1: u16;
    let mut HVar1: HMENU16;
    let mut iVar2: *mut astruct_854;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (iVar2.field236_0xee.is_null() == false) && (iVar2.field235_0xec == 0) {
        HVar1 = LoadMenu16(iVar2.field236_0xee, HINSTANCE16_1050_038c);
        iVar2.field235_0xec = HVar1;
        if HVar1 == 0 {
            return;
        }
        HVar1 = GetSubMenu16(0x0, iVar2.field235_0xec);
        iVar2.field235_0xec = HVar1;
        if HVar1 == 0 {
            return;
        }
    }
    ClientToScreen16(CONCAT22(0x1050, &stack0xfffa), iVar2.field8_0x8);
    HVar1 = iVar2.field235_0xec;
    TrackPopupMenu16(NULL, iVar2.field8_0x8, 0x0, HVar1, 0x0, 0x0, HVar1);
    return;
}

pub fn mixed_menu_op_1020_44ec(
    param_1: *mut astruct_850,
    mut param_2: u16,
    mut param_3: i16,
    param_4: HMENU16,
    mut param_5: u16,
    param_6: u8,
) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut UVar4: u16;
    let mut BVar5: bool;
    //   HMENlet mut HVar6: u16;
    let mut HVar6: HMENU16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut puVar9: *mut u8;
    let mut uVar10: u16;
    let mut in_register_0000000a: u16;
    let mut paVar11: *mut Struct57;
    let mut iVar9: *mut astruct_850;
    let mut iVar12: i16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut data: *mut c_char;
    let mut puVar15: *mut u32;
    let mut in_stack_0000fd70: u16;
    let mut in_stack_0000fe94: u16;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000fe9e: u16;
    let mut w_flags: u16;
    let mut in_stack_0000fec8: u16;
    //   HMENlet mut w_item_id: u16;
    let mut w_ite_id: HMENU16;
    let mut uStack300: u16;
    let mut bStack293: u8;
    let mut uStack278: u16;
    let mut uStack268: u32;
    let mut local_108: [u32; 0x40] = [0; 0x40];
    let mut uStack8: u16;
    let mut puStack6: *mut u32;

    paVar11 = CONCAT22(in_register_0000000a, param_5);
    uVar13 = (param_1 >> 0x10);
    iVar9 = param_1;
    if (iVar9.hmenu_0x106 != 0) {
        if (iVar9.hmenu_0x106 == param_4) {
            puStack6 = mixed_1010_20ba(
                paVar11,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000fec8, 0x3),
                in_stack_0000fd70,
                in_stack_0000fe94,
                in_stack_0000fe9a,
                in_stack_0000fe9e,
            );
            uVar2 = iVar9.field257_0x108;
            uStack8 = (uVar2 + 0x2e);
            uVar2 = iVar9.field257_0x108;
            uVar14 = (uVar2 >> 0x10);
            iVar12 = uVar2;
            uVar1 = (iVar12 + 0x42);
            puVar9 = (iVar12 + 0x44);
            bStack293 = (uVar1 >> 0x18);
            uVar7 = bStack293;
            if (bStack293 == 0) {
                uVar3 = pass1_1020_bd80(uStack8);
                unk_str_op_1000_3d3e(CONCAT22(0x1050, local_108), CONCAT22(puVar9, uVar3));
            } else {
                pass1_1030_8344(_u16_1050_5748, uVar1 & 0xffff | ZEXT24(puVar9) << 0x10);
                pass1_1010_c3c2(
                    puVar9,
                    puStack6,
                    (puStack6 >> 0x10),
                    CONCAT22(0x1050, local_108),
                    CONCAT22(puVar9, uVar7),
                );
            }
            ModifyMenu16(
                CONCAT22(0x1050, local_108),
                0x76,
                0x0,
                0x76,
                iVar9.hmenu_0x106,
            );
            UVar4 = GetMenuState16(0x0, 0x13c, iVar9.hmenu_0x106);
            if UVar4 != 0xffff {
                DeleteMenu16(0x0, 0x13c, iVar9.hmenu_0x106);
            }
            BVar5 = pass1_1008_c6ae(_u16_1050_06e0, uStack8, 0x20);
            if BVar5 != 0 {
                data = load_string_1010_847e(_u16_1050_14cc, 0x74b);
                InsertMenu16(data, 0x13c, 0x400, 0xffff, iVar9.hmenu_0x106);
            }
            if (s_VrMode_1050_42ca + 0x8 + uStack8 * 0x2) == 0 {
                HVar6 = iVar9.hmenu_0x106;
                w_flags = 0x1;
                UVar4 = 0x77;
                // TODO: goto LAB_1020_464c;
            }
            HVar6 = iVar9.hmenu_0x106;
            UVar4 = 0x77;
        } else {
            HVar6 = GetSubMenu16(0x1, iVar9.hmenu_0x106);
            //      if (HVar6 != param_4) goto LAB_1020_479e;
            EnableMenuItem16(0x1, 0x200, HVar6);
            uVar10 = paVar11;
            EnableMenuItem16(0x1, 0x201, HVar6);
            EnableMenuItem16(0x1, 0x202, HVar6);
            uVar2 = iVar9.field257_0x108;
            uVar8 = (uVar2 + 0x42);
            pass1_1030_8344(_u16_1050_5748, uVar8);
            uVar7 = uVar8;
            if (uVar10 | uVar7) == 0 {
                return;
            }
            uVar2 = (uVar7 + 0x2e);
            uVar7 = (uVar7 + 0x30);
            uStack278 = uVar2;
            if (uVar7 | uStack278) == 0 {
                return;
            }
            uStack268 = (uStack278 + 0x200);
            local_108[0] = struct_op_1030_73a8((uVar8 & 0xffff | uVar10 << 0x10), uStack268, uVar7);
            uVar13 = (local_108[0] >> 0x10);
            puStack6 = (local_108[0] + 0x1c);
            uVar7 = (local_108[0] + 0x1e);
            if (uVar7 | puStack6) != 0 {
                uStack268 = puStack6 & 0xffff | uVar7 << 0x10;
            }
            uStack268 &= 0xff;
            if uStack268 != 1 {
                return;
            }
            if (uStack268 & 0xff0000) != 0 {
                return;
            }
            uVar3 = pass1_1030_6fa0(uVar8 & 0xffff | uVar10 << 0x10);
            BVar5 = pass1_1008_c6ae(_u16_1050_06e0, uVar3, 0x3f);
            if BVar5 != 0 {
                BVar5 = EnableMenuItem16(0x0, 0x201, HVar6);
            }
            if ((uVar8 & 0xffff) + 0x36) != 0 {
                BVar5 = EnableMenuItem16(0x0, 0x202, HVar6);
            }
            pass1_1030_69cc(BVar5, uStack268, uVar8 & 0xffff | uVar10 << 0x10);
            if BVar5 == 0 {
                return;
            }
            UVar4 = 0x200;
        }
        w_flags = 0;
        // TODO: goto LAB_1020_464c;
    } //
    // LAB_1020_479e:
    iVar12 = param_3 - 0x1;
    if iVar12 == 0 {
        pass1_1018_2504(0x0, paVar11);
        if iVar12 == 0 {
            UVar4 = 0;
            EnableMenuItem16(0x401, 0x0, param_4);
            HVar6 = 0x1; //
            // LAB_1020_47e3:
            w_flags = 0x401;
            // TODO: goto LAB_1020_464c;
        }
        UVar4 = 0;
        EnableMenuItem16(0x400, 0x0, param_4);
        HVar6 = 0x1;
    } else if param_3 == 0x2 {
        uVar3 = pass1_1020_64d4(iVar9.field246_0xf6, 0x2);
        if uVar3 == 0 {
            EnableMenuItem16(0x401, 0x0, param_4);
            UVar4 = 0x401;
        } else {
            EnableMenuItem16(0x400, 0x0, param_4);
            UVar4 = 0x400;
        }
        HVar6 = 0x1;
        EnableMenuItem16(UVar4, 0x1, param_4);
        //    if ((PTR_LOOP_1050_0010.is_null() == false) || (iVar9.field255_0x102 == 0)) goto LAB_1020_47e3;
    } else {
        if param_3 == 0x3 {
            HVar6 = 0;
            puVar15 = mixed_1010_20ba(
                paVar11,
                _u16_1050_0ed0,
                0x2f,
                in_stack_0000fd70,
                in_stack_0000fe94,
                in_stack_0000fe9a,
                in_stack_0000fe9e,
            );
            uVar13 = (puVar15 >> 0x10);
            uVar1 = (puVar15 + 0x20);
            uVar7 = (puVar15 + 0x22);
            uStack300 = uVar1;
            if (uVar7 | uStack300) != 0 {
                pass1_1030_8308(
                    &stack0xfecc,
                    uVar7,
                    _u16_1050_5748,
                    (_u16_1050_5748 >> 0x10),
                    CONCAT22(0x1050, &stack0xfecc),
                    CONCAT22(0x1050, &stack0xfec8),
                    uVar1 & 0xffff | uVar7 << 0x10,
                );
            }
            UVar4 = 0;
            loop {
                CheckMenuItem16(0x400, UVar4, param_4);
                w_item_id = param_4;
                EnableMenuItem16(0x401, UVar4, param_4);
                UVar4 += 0x1;
                if UVar4 >= 5 {
                    break;
                }
            }
            CheckMenuItem16(0x408, w_item_id, param_4);
            //   for (UVar4 = 0; UVar4 <= HVar6; UVar4 += 1)
            for UVar4 in 0..HVar6 {
                HVar6 = param_4;
                EnableMenuItem16(0x400, UVar4, param_4);
            }
            return;
        }
        if param_3 != 0x4 {
            return;
        }
        UVar4 = 0x2;
        HVar6 = param_4;
    }
    w_flags = 0x400; //
    // LAB_1020_464c:
    EnableMenuItem16(w_flags, UVar4, HVar6);
    return;
}

pub fn enable_menu_1020_1000(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
    param_5: HMENU,
) {
    if (param_4 != 0) {
        return;
    }
    EnableMenuItem16(0x400, 0x3, param_5);
    return;
}


pub fn enable_menu_item_1020_2c2a(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    param_5: HMENU16,
) -> BOOL16 {
    let mut BVar1: bool;
    let mut w_flags: u16;

    if param_4 != 0 {
        return param_4 - 0x1;
    }
    EnableMenuItem16(0x400, 0x3, param_5);
    if PTR_LOOP_1050_3960 < 0x2 {
        w_flags = 0x401;
    } else {
        w_flags = 0x400;
    }
    BVar1 = EnableMenuItem16(w_flags, 0x5, param_5);
    return BVar1;
}
