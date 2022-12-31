use crate::app_context::AppContext;
use crate::dos_interrupt::{dos_set_interrupt_vector, swi};
use crate::draw_ops::draw_a::invalidate_rect_1020_735a;
use crate::file_ops::{close_file_1008_6dd0, read_file_1008_6e78};
use crate::globals::{u8_1050_5fc9, DAT_1050_5f82, DAT_1050_5f87, HINSTANCE16_1050_5f4c, PTR_LOOP_1050_5f48, PTR_LOOP_1050_5f4a, PTR_LOOP_1050_5f4e, PTR_LOOP_1050_5f50, PTR_LOOP_1050_5f7e, PTR_LOOP_1050_5f84, PTR_LOOP_1050_5fb8, PTR_LOOP_1050_5fc2, PTR_LOOP_1050_5fc4, PTR_LOOP_1050_5fd2, PTR_LOOP_1050_5fd4, PTR_LOOP_1050_63fe, WIN_VERSION_1050_5f80, DAT_1050_1050, PTR_LOOP_1050_0000, PTR_LOOP_1050_6066, PTR_LOOP_1050_1000, PTR_LOOP_1050_1040};
use crate::gui::window::win_e;
use crate::gui::{cursor, dialog};
use crate::no_refs::i::pass1_1010_6566;
use crate::no_refs::l::{pass1_1010_a58a, pass1_1010_a5ac};
use crate::resources::load_string_1010_847e;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_72::Struct72;
use crate::structs::struct_825::Struct825;
use crate::structs::struct_d::StructD;
use crate::sys_ops::{debug_print_1008_6048, pass1_1000_27d6};
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_1000_167a, mem_op_1000_179c};
use crate::unk::block_1000_2000::{
    fn_ptr_op_1000_2594, init_1000_23be, pass1_1000_25a8, pass1_1000_2913, poss_str_op_1000_28dc,
};
use crate::unk::block_1000_5000::{dos3_call_1000_51aa, dos_get_interrupt_vector_1000_23ea, ret_op_1000_55ac};
use crate::unk::block_1008_3000::{pass1_1008_3bd6, pass1_1008_3e94};
use crate::unk::block_1008_5000::{pass1_1008_5394, pass1_1008_5bdc, set_struct_1008_574a};
use crate::unk::block_1008_6000::{str_1008_6d8a, str_op_1008_60e8};
use crate::unk::block_1008_8000::empty_1008_8fc4;
use crate::unk::block_1010_0000::pass1_1010_043a;
use crate::unk::block_1010_2000::{mixed_1010_20ba, pass1_1010_2024, pass1_1010_209e};
use crate::unk::block_1010_a000::{pass1_1010_a5ca, pass1_1010_ac62};
use crate::unk::block_1018_0000::pass1_1018_0afa;
use crate::unk::block_1018_1000::{pass1_1018_179e, pass1_1018_1eda};
use crate::unk::block_1018_2000::{pass1_1018_2862, pass1_1018_2fe8};
use crate::unk::block_1018_3000::{pass1_1018_30ca, pass1_1018_31d0};
use crate::unk::block_1030_5000::pass1_1030_5b00;
use crate::unk::block_1030_6000::pass1_1030_6c1a;
use crate::unk::block_1030_8000::pass1_1030_8344;
use crate::unk::block_1038_3000::pass1_1038_387e;
use crate::unk::block_1040_8000::string_1040_8520;
use crate::unk::block_1040_b000::{pass1_1040_bfde, struct_1040_bf3e};
use crate::unk::{block_1000_2000, block_1000_4000, block_1018_1000};
use crate::utils::{CARRY2, CONCAT11, CONCAT22, SUB42};
use crate::winapi16::{DestroyWindow16, DispatchMessage16, FatalAppExit16, FatalExit, GetDlgItem16, GetDlgItemInt16, GetMessage16, GetModuleFileName16, GetVersion16, GlobalAlloc16, GlobalFree16, GlobalUnlock16, InitApp16, InitTask16, IsDialogMessage16, LoadString16, LockSegment16, MessageBeep16, WinAPI16_MessageBox16, PostMessage16, WinAPI16_PtInRect16, SendMessage16, SetWindowPos16, TranslateAccelerator16, TranslateMessage16, UpdateWindow16, WinAPI16_WaitEvent16};
use crate::winapp::{pass1_1000_29dc, winapp_b};
use crate::windef16::{HGLOBAL16, HWND16, LRESULT, MSG16, POINT16, WPARAM16};
use crate::{dos_interrupt, winapp};
use std::os::raw::c_char;
use std::ptr;
use std::ptr::{null, null_mut};
use crate::draw_ops::set_struct_op_1008_0536;
use crate::no_refs::j::pass1_1010_7e40;
use crate::structs::struct_20::Struct20;
use crate::structs::struct_b::StructB;
use crate::unk::block_1038_a000::pass1_1038_aeca;

pub fn pt_in_rect_1010_40f8(
    ctx: &mut AppContext,
    pstruct57_arg1: *mut Struct57,
    mut u32_arg2: u32,
    ppnt_arg3: *mut POINT16,
    mut u16_arg4: u16,
) -> i16 {
    let mut pi16_var1: *mut i16;
    let mut ppfn_var2: *mut *mut code;
    let mut b_var3: bool;
    let mut u16_var4: u16;
    let mut u16_var5: u16;
    let mut u16_var6: u16;
    let mut u16_var7: u16;
    let mut pstruct57_var8: *mut Struct57;
    let mut i16_var9: i16;
    let mut u16_var10: u16;
    let mut pu32_var11: *mut u32;
    let mut in_stack_0000fe94: u16 = 0;
    let mut in_stack_0000ffb8: u16 = 0;
    let mut in_stack_0000ffbe: u16 = 0;
    let mut in_stack_0000ffc2: u16 = 0;
    let mut pu32_var12: *mut u32;
    let mut i16_var13: i16;
    let mut u16_var14: u16;
    let mut u32_var15: u32;

    i16_var13 = 0;
    u16_var14 = 0;
    loop {
        u16_var10 = (u32_arg2 >> 0x10);
        i16_var9 = u32_arg2;
        pi16_var1 = (i16_var9 + 0x74);
        if *pi16_var1 == i16_var13 || *pi16_var1 < i16_var13 {
            // TODO:
            // LAB_1010_413e:
            if (u16_var14 != 0) && (0x3 < PTR_LOOP_1050_3960) {
                pu32_var11 = mixed_1010_20ba(
                    pstruct57_arg1,
                    _u16_1050_0ed0,
                    CONCAT22(u16_arg4, i16_var13 + 0xc),
                    in_stack_0000fe94,
                    in_stack_0000ffb8,
                    in_stack_0000ffbe,
                    in_stack_0000ffc2,
                );
                pstruct57_var8 = (pstruct57_arg1 & 0xffff0000 | pu32_var11 >> 0x10);
                u16_var4 = pass1_1018_0afa(pu32_var11);
                if u16_var4 == 0 {
                    u16_var10 = 0x1000;
                    u16_var5 = u16_var4;
                    mem_op_1000_179c(0xb4, pstruct57_var8);
                    u16_var6 = pstruct57_var8 | u16_var5;
                    u32_var15 = pstruct57_var8 & 0xffff0000 | u16_var6;
                    if u16_var6 == 0 {
                        i16_var9 = 0;
                        u16_var7 = 0;
                    } else {
                        u16_var10 = ctx.globals.PTR_LOOP_1050_1040;
                        i16_var9 = string_1040_8520(
                            u32_var15,
                            CONCAT22(pstruct57_var8, u16_var5),
                            HWND16_1050_0396,
                            0x20030,
                            0x6450643,
                        );
                        u16_var7 = u32_var15;
                    }
                    pu32_var12 = CONCAT22(u16_var7, i16_var9);
                    ppfn_var2 = (*pu32_var12 + 0x74);
                    (**ppfn_var2)(u16_var10, i16_var9, u16_var7);
                    pass1_1010_209e(_u16_1050_0ed0, i16_var13 + 0xc);
                    u16_var14 = u16_var4;
                }
            }
            if u16_var14 != 0 {
                return i16_var13;
            }
            return -0x1;
        }
        pstruct57_arg1 = (pstruct57_arg1 & 0xffff0000 | (i16_var9 + 0x72));
        b_var3 = WinAPI16_PtInRect16(
            *ppnt_arg3,
            ((i16_var13 * 0x10 + (i16_var9 + 0x24)) * 0x8 + (i16_var9 + 0x70)),
        );
        if b_var3 != 0 {
            u16_var14 = 0x1;
            // TODO: goto LAB_1010_413e;
        }
        i16_var13 += 0x1;
    }
}

pub fn pt_in_rect_1018_1bda(u32_arg1: u32, u16_arg2: u16, u16_arg3: u16) {
    let mut pi16_var1: *mut i16;
    let mut u16_var2: u16;
    let mut i16_var3: i16;
    let mut pt_in_rect: bool;
    let mut i16_var5: i16;
    let mut u16_var6: u16;
    let mut i16_var7: i16;
    let mut i16_var8: i16;
    let mut i16_var9: i16;
    let mut i16_var10: i16;
    let mut u16_var11: u16;
    let mut u32_var12: u32;
    let mut u32_var13: u32;
    let mut i16_var14: i16;
    let mut i16_var15: i16;

    u32_var12 = 0;
    i16_var3 = u32_arg1;
    pass1_1008_3e94(
        u32_arg1 + 0x3a,
        &i16_var9,
        &i16_var10,
    );
    PStack24 = CONCAT22(u16_arg2, u16_arg3);
    u16_var11 = 0;
    i16_var7 = 0;
    loop {
        u16_var6 = (u32_arg1 >> 0x10);
        pi16_var1 = (i16_var3 + 0x44);
        if (*pi16_var1 == i16_var7 || *pi16_var1 < i16_var7) {
            return;
        }
        u16_var2 = (i16_var3 + 0x42);
        i16_var5 = (i16_var3 + 0x40) + i16_var7 * 0x18;
        u32_var12 = CONCAT22(u16_var2, i16_var5);
        pass1_1008_3e94(
            CONCAT22(u16_var2, i16_var5),
            &u32_var13 + 0x2,
            &u32_var13,
        );
        u32_var13 += i16_var10 - 0x6;
        i16_var14 = u32_var13 + 0xc;
        u32_var13 += i16_var9 - 0x6;
        i16_var15 = u32_var13 + 0xc;
        pt_in_rect = WinAPI16_PtInRect16(PStack24, &u32_var13);
        if pt_in_rect != 0 {
            break;
        }
        i16_var7 += 0x1;
    }
    pass1_1018_1eda(u32_arg1, u32_var12);
    return;
}

pub fn pt_in_rect_1020_5856(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    ppoint_arg4: *mut POINT16,
) {
    let mut pu32_var1: *mut u32;
    let mut b_var2: bool;
    let mut u32_var3: u32;
    let mut u16_var4: u16;
    let mut u32_var5: u32;

    pass1_1018_2862((param_3 + 0xfa));
    if (param_2 | param_1) != 0 {
        u32_var5 = 0;
        loop {
            pu32_var1 = (param_1 + 0xa);
            if *pu32_var1 < u32_var5 || *pu32_var1 == u32_var5 {
                break;
            }
            u32_var3 = u32_var5;
            empty_1008_8fc4();
            if (u16_var4 | u32_var3) != 0 {
                b_var2 = WinAPI16_PtInRect16(*ppoint_arg4, (u32_var3 + 0x14));
                if b_var2 != 0 {
                    return;
                }
            }
            u32_var5 += 0x1;
        }
    }
    return;
}

pub fn pt_in_rect_1010_4e08(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut piVar1: *mut i16;
    let mut bVar2: bool;
    let mut BVar3: bool;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut POPStack8: INT16;

    PStack8 = CONCAT22(param_2, param_3);
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    (iVar4 + 0x22) = (iVar4 + 0x20);
    bVar2 = false;
    (iVar4 + 0x24) = 0;
    iStack12 = 0;
    iStack10 = 0;
    loop {
        piVar1 = (iVar4 + 0x30);
        if *piVar1 == iStack12 || *piVar1 < iStack12 {
            //
            // LAB_1010_4e67:
            if iStack10 != 0 {
                (iVar4 + 0x20) = iStack10;
            }
            if bVar2 {
                return;
            }
            return;
        }
        BVar3 = WinAPI16_PtInRect16(PStack8, ((iVar4 + 0x1a) + iStack12 * 0x8));
        if (BVar3 != 0) {
            iStack10 = iStack12;
            bVar2 = true;
            // TODO: goto LAB_1010_4e67;
        }
        iStack12 += 0x1;
    }
}

pub fn pt_in_rect_1020_68fc(param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut POPStack6: INT16;

    PStack6 = CONCAT22(param_2, param_3);
    uVar4 = (param_1 >> 0x10);
    uVar2 = pass1_1018_31d0((param_1 + 0xf2));
    if (uVar2 != 0) {
        BVar3 = WinAPI16_PtInRect16(PStack6, ((param_1 + 0xf2) + 0x16c));
        if (BVar3 != 0) {
            ppcVar1 = (*param_1 + 0x40);
            (**ppcVar1)(0x1538, param_1, 0xef);
        }
    }
    return;
}

pub fn win_msg_op_1008_9498() -> WPARAM16 {
    let mut BVar1: bool;
    let mut IVar2: i16;
    let mut local_msg_1: MSG16; //
                                //
                                // LAB_1008_949c:
    BVar1 = GetMessage16(0x0, 0x0, 0x0, &local_msg_1);
    if BVar1 == 0 {
        return local_msg_1.wparam;
    }
    //   if ((_u16_1050_5bc8 + 0x8) != 0) goto code_r0x100894cd;
    //   goto LAB_1008_94dc;
    // code_r0x100894cd:
    BVar1 = IsDialogMessage16(&local_msg_1, 0x1050);
    if BVar1 == 0 {
        //
        // LAB_1008_94dc:
        if PTR_LOOP_1050_0398.is_null() == false {
            IVar2 = TranslateAccelerator16(&local_msg_1, &DAT_1050_1050, PTR_LOOP_1050_0398);
            //   if (IVar2 != 0) goto LAB_1008_949c;
        }
        TranslateMessage16(&local_msg_1);
        DispatchMessage16(&local_msg_1);
    }
    //   goto LAB_1008_949c;
}

pub fn win_ui_op_1020_6ae6(
    param_1: *mut astruct_877,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: i16,
    param_5: u16,
    param_6: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut hwnd: HWND16;
    let mut puVar2: *mut u8;
    let mut iVar3: *mut astruct_877;
    let mut uVar3: u16;
    let mut LVar4: LRESULT;

    if param_4 == 0x1797 {
        uVar3 = (param_1 >> 0x10);
        iVar3 = param_1;
        hwnd = GetDlgItem16(0x1797, iVar3.field8_0x8);
        if hwnd != 0 {
            if param_3 == 0x2 {
                LVar4 = SendMessage16(0x0, 0x0, 0x409, hwnd);
                if LVar4 != -1 {
                    LVar4 = SendMessage16(
                        CONCAT13(0x10, CONCAT12(0x50, &stack0xffa8)),
                        LVar4,
                        0x40a,
                        hwnd,
                    );
                    puVar2 = &stack0xffa8;
                    pass1_1018_30ca(
                        (LVar4 >> 0x10),
                        iVar3.field241_0xf2,
                        CONCAT22(0x1050, puVar2),
                    );
                    pass1_1018_2fe8(iVar3.field241_0xf2, param_5, param_6);
                    if puVar2.is_null() == false {
                        invalidate_rect_1020_735a(iVar3.field242_0xf6);
                        ppcVar1 = (param_1 + 0x40);
                        (**ppcVar1)(0x1018, iVar3);
                    }
                }
            } else if param_3 != 0x3 {
                return;
            }
            DestroyWindow16(hwnd);
        }
    }
    return;
}

pub fn unk_win_ui_op_1038_8afe(mut param_1: u16, param_2: *mut astruct_50) {
    let mut uVar1: u32;
    let mut dlg_item: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar4: *mut astruct_50;
    let mut uVar4: *mut astruct_50;
    let mut local_4: bool;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    uVar4 = (param_2 >> 0x10);
    iVar4 = param_2;
    dlg_item = GetDlgItemInt16(0x0, &mut local_4, 0x1050, HWND_1050_184d);
    pass1_1030_6c1a(iVar4.field148_0x94, dlg_item);
    uVar1 = iVar4.field148_0x94;
    pass1_1038_387e(
        paVar2,
        (uVar1 + 0x2e),
        dlg_item,
        iVar4.field153_0x9c,
        iVar4.field148_0x94,
    );
    return;
}

pub fn win_msg_op_1038_95fc(mut param_1: u16, mut param_2: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut UVar3: u16;
    let mut UVar4: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut paVar7: *mut Struct57;
    let mut iVar9: i16;
    let mut unaff_SI: u16;
    let mut uVar10: u16;
    let mut in_stack_0000fe80: u16;
    let mut in_stack_0000ffa4: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut puStack30: *mut u16;
    let mut puStack24: *mut u16;
    let mut iStack20: i16;
    let mut local_10: bool;
    let mut puStack14: *mut u32;
    let mut puStack10: *mut u32;
    let mut puStack6: *mut u32;
    let mut paVar8: *mut Struct57;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    puStack6 = mixed_1010_20ba(
        paVar6,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x8),
        in_stack_0000fe80,
        in_stack_0000ffa4,
        in_stack_0000ffaa,
        in_stack_0000ffae,
    );
    paVar6 = (paVar6 & 0xffff0000 | puStack6 >> 0x10);
    puStack10 = mixed_1010_20ba(
        paVar6,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x9),
        in_stack_0000fe80,
        in_stack_0000ffa4,
        in_stack_0000ffaa,
        in_stack_0000ffae,
    );
    paVar7 = (paVar6 & 0xffff0000 | puStack10 >> 0x10);
    uVar2 = puStack10;
    mem_op_1000_179c(0xc, paVar7);
    uVar5 = paVar7 | uVar2;
    paVar6 = (paVar7 & 0xffff0000);
    paVar8 = (paVar6 | uVar5);
    if (uVar5 == 0) {
        uVar2 = 0;
    } else {
        set_struct_1008_574a(CONCAT22(paVar7, uVar2));
        paVar6 = paVar8;
    }
    puStack14 = CONCAT22(paVar6, uVar2);
    for iStack20 in 0..0xf {
        uVar13 = (param_2 + 0x6);
        UVar3 = GetDlgItemInt16(0x1, &local_10, 0x1050, (iStack20 * 0xe + 0x5a72));
        if (UVar3 != 0) {
            if ((iStack20 * 0xe + 0x5a7c) < 0x83) {
                UVar4 = UVar3;
                mem_op_1000_179c(0x8, paVar6);
                uVar2 = paVar6;
                puStack24 = CONCAT22(uVar2, UVar4);
                paVar6 = (paVar6 & 0xffff0000 | (uVar2 | UVar4));
                if ((uVar2 | UVar4) == 0) {
                    puStack30 = null_mut();
                } else {
                    *puStack24 = 0x389a;
                    (UVar4 + 0x2) = 0x1008;
                    *puStack24 = 0xa1c4;
                    (UVar4 + 0x2) = 0x1010;
                    puStack30 = puStack24;
                }
                uVar10 = (puStack30 >> 0x10);
                iVar9 = puStack30;
                (iVar9 + 0x6) = UVar3;
                (iVar9 + 0x4) = (iStack20 * 0xe + 0x5a7c);
                ppcVar1 = (*puStack14 + 0x4);
                (**ppcVar1)(
                    0x1000,
                    puStack14,
                    (puStack14 >> 0x10),
                    iVar9,
                    uVar10,
                    uVar13,
                );
            } else {
                if ((iStack20 * 0xe + 0x5a7c) == 0x89) {
                    uVar12 = (iStack20 * 0xe + 0x5a7c);
                    uVar11 = UVar3;
                } else {
                    uVar12 = (iStack20 * 0xe + 0x5a7c);
                    uVar11 = 0;
                }
                pass1_1010_6566(puStack10, uVar11, UVar3, uVar12);
            }
        }
    }
    (puStack6 + 0xa) = puStack14;
    PostMessage16(0x0, 0xed, 0x111, HWND16_1050_0396);
    return;
}

pub fn win_ui_op_1008_1414(
    mut param_1: u32,
    param_2: *mut Struct20,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut puVar5: *mut u32;
    let mut extraout_DX: u16;
    let mut uVar6: u32;
    let mut paVar7: *mut Struct57;
    let mut uVar8: u32;
    let mut iVar17: *mut Struct20;
    let mut unaff_SI: u16;
    let mut unaff_DI: u16;
    let mut uVar16: *mut Struct72;
    let mut puVar9: *mut u32;
    let mut pcVar10: *mut c_char;
    let mut puVar11: *mut u32;
    let mut paVar12: *mut Struct27;
    let mut in_stack_0000fe3c: u16;
    let mut in_stack_0000fe3e: u16;
    let mut in_stack_0000fe4e: u16;
    let mut in_stack_0000ff60: u16;
    let mut in_stack_0000ff62: u16;
    let mut in_stack_0000ff66: u16;
    let mut in_stack_0000ff68: u16;
    let mut in_stack_0000ff6a: u16;
    let mut in_stack_0000ff6c: u16;
    let mut in_stack_0000ff72: u16;
    let mut in_stack_0000ff78: u16;
    let mut in_stack_0000ff7c: u16;
    let mut uVar13: u8;
    let mut uVar14: u8;
    let mut uVar15: u16;
    let mut local_2a: u32;
    let mut uStack38: u16;
    let mut iStack36: i16;
    let mut uStack34: u16;
    let mut uStack32: u32;
    let mut uStack28: u32;
    let mut uStack24: u32;
    let mut uStack20: u32;
    let mut uStack16: u32;
    let mut puStack12: *mut u32;
    let mut local_8: [u8; 0x6] = [0; 0x6];
    let mut uVar10: u16;

    puVar9 = str_1008_6d8a(param_1, CONCAT22(0x1050, local_8), param_3);
    paVar7 = (param_1 & 0xffff0000 | puVar9 >> 0x10);
    BVar3 = read_file_1008_6e78(CONCAT22(0x1050, local_8), unaff_SI, unaff_DI, param_4);
    iVar17 = param_2;
    uVar16 = (param_2 >> 0x10);
    if (BVar3 == 0) {
        if (u16_1050_0310 == 0) {
            u16_1050_0310 = 0x6d4;
        }
        pcVar10 = load_string_1010_847e(_u16_1050_14cc, u16_1050_0310);
        uVar8 = paVar7 & 0xffff0000 | pcVar10 >> 0x10;
        uVar4 = str_op_1008_60e8((pcVar10 >> 0x10), pcVar10);
        uVar15 = uVar8;
        pcVar10 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
        paVar7 = (uVar8 & 0xffff0000 | pcVar10 >> 0x10);
        MessageBeep16(0x10);
        WinAPI16_MessageBox16(0x10, pcVar10, CONCAT22(uVar15, uVar4), iVar17.field3_0x8);
        fn_ptr_1000_17ce(CONCAT22(uVar15, uVar4));
        fn_ptr_op_1000_24cd(1);
    }
    cursor::cursor_op_1008_2dcc(paVar7, param_2, 0x8, param_6, param_5);
    puStack12 = mixed_1010_20ba(
        paVar7,
        _u16_1050_0ed0,
        CONCAT22(unaff_DI, 0x2f),
        in_stack_0000fe4e,
        in_stack_0000ff72,
        in_stack_0000ff78,
        in_stack_0000ff7c,
    );
    paVar7 = (paVar7 & 0xffff0000 | puStack12 >> 0x10);
    uVar6 = (puStack12 + 0x20);
    uStack16 = uVar6;
    pass1_1030_8344(_u16_1050_5748, uVar6);
    uStack20 = uVar6 & 0xffff | paVar7 << 0x10;
    uStack24 = (uVar6 + 0x10);
    iVar5 = (uStack24 + 0x2) - 0x1;
    uVar1 = (&iVar17[0x1].field2_0x4 + 2);
    ppcVar2 = ((&iVar17[0x1].field2_0x4 + 0x2) + 0x4);
    (**ppcVar2)(
        0x1030,
        uVar1,
        (uVar1 >> 0x10),
        uStack16,
        (uStack16 >> 0x10),
        iVar5,
        0x2,
    );
    pass1_1030_8344(_u16_1050_5748, 0x4000001);
    uStack28 = CONCAT22(paVar7, iVar5);
    uVar6 = (iVar5 + 0x10);
    uStack32 = uVar6;
    pass1_1030_8344(_u16_1050_5748, uVar6);
    iStack36 = uVar6;
    uStack34 = SUB42(paVar7, 0x0);
    local_2a = (iStack36 + 0xc);
    uStack38 = (iStack36 + 0x10);
    puVar5 = pass1_1030_5b00(uStack20);
    uVar15 = SUB42(0x1050, 0x0);
    uVar13 = SUB21(&local_2a, 0x0);
    uVar14 = (&local_2a >> 0x8);
    puVar11 = mixed_1010_20ba(
        paVar7,
        _u16_1050_0ed0,
        CONCAT13(uVar14, CONCAT12(uVar13, puVar5)),
        in_stack_0000fe3e,
        in_stack_0000ff62,
        in_stack_0000ff68,
        in_stack_0000ff6c,
    );
    paVar7 = (paVar7 & 0xffff0000 | puVar11 >> 0x10);
    pass1_1018_179e(puVar11, CONCAT22(uVar15, CONCAT11(uVar14, uVar13)));
    uVar13 = 0;
    uVar14 = 0x4;
    iVar5 = 0x1b;
    paVar12 = mixed_1010_20ba(
        paVar7,
        _u16_1050_0ed0,
        0x1002b,
        in_stack_0000fe3c,
        in_stack_0000ff60,
        in_stack_0000ff66,
        in_stack_0000ff6a,
    );
    pass1_1010_043a(paVar12, CONCAT13(uVar14, CONCAT12(uVar13, 1)), iVar5);
    close_file_1008_6dd0(CONCAT22(0x1050, local_8));
    return;
}

pub fn win_ui_1040_b8d2(mut param_1: u16, param_2: *mut StructB) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut in_register_0000000a: u16;
    let mut paVar10: *mut Struct57;
    let mut paVar12: *mut Struct57;
    let mut struct_b_10: *mut StructB;
    let mut struct_b_10_hi: *mut StructB;
    let mut uVar13: u16;
    let mut puVar14: *mut u32;
    let mut puVar15: *mut u16;
    let mut in_stack_0000fe3a: u16;
    let mut in_stack_0000fe3e: u16;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ff64: u16;
    let mut in_stack_0000ff68: u16;
    let mut in_stack_0000ff6c: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffe4: u16;
    let mut paVar11: *mut Struct57;

    paVar10 = CONCAT22(in_register_0000000a, param_1);
    dialog::dialog_ui_fn_1040_78e2(param_2);
    puVar14 = mixed_1010_20ba(
        paVar10,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffe4, 0x31),
        in_stack_0000fe8c,
        in_stack_0000ffb0,
        in_stack_0000ffb6,
        in_stack_0000ffba,
    );
    paVar10 = (paVar10 & 0xffff0000 | puVar14 >> 0x10);
    paVar4 = puVar14;
    struct_b_10_hi = (param_2 >> 0x10);
    struct_b_10 = param_2;
    struct_b_10[0x7].field6_0xc = paVar4;
    struct_b_10[0x7].field7_0xe = (puVar14 >> 0x10);
    mem_op_1000_179c(0xa, paVar10);
    uVar8 = paVar10 | paVar4;
    paVar11 = (paVar10 & 0xffff0000 | uVar8);
    if (uVar8 == 0) {
        struct_b_10[0x7].max_count_field_0x10 = 0;
    } else {
        puVar15 = struct_1040_bf3e(CONCAT22(paVar10, paVar4), struct_b_10.lpvoid_field_0x8);
        paVar11 = (paVar11 & 0xffff0000 | puVar15 >> 0x10);
        paVar4 = puVar15;
        struct_b_10[0x7].max_count_field_0x10 = paVar4;
        struct_b_10[0x7].field5_0xa = (puVar15 >> 0x10);
    }
    pass1_1040_bfde(
        &struct_b_10[0x7].max_count_field_0x10,
        &struct_b_10[0x7].field6_0xc,
    );
    mem_op_1000_179c(0x42, paVar11);
    uVar8 = paVar11 | paVar4;
    paVar10 = (paVar11 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar10,
            paVar4,
            paVar11,
            0x1,
            0xa000a,
            0x0,
            0x800081,
            CONCAT22(struct_b_10.lpvoid_field_0x8, 0x10a),
            in_stack_0000ffac,
            in_stack_0000fe3a,
            in_stack_0000fe3e,
            in_stack_0000ff64,
            in_stack_0000ff68,
            in_stack_0000ff6c,
        );
    }
    mem_op_1000_179c(0x42, paVar10);
    uVar8 = paVar10 | paVar4;
    paVar11 = (paVar10 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar11,
            paVar4,
            paVar10,
            0x1,
            0xa0028,
            0x0,
            0x840085,
            CONCAT22(struct_b_10.lpvoid_field_0x8, 0x10b),
            in_stack_0000ffac,
            in_stack_0000fe3a,
            in_stack_0000fe3e,
            in_stack_0000ff64,
            in_stack_0000ff68,
            in_stack_0000ff6c,
        );
    }
    mem_op_1000_179c(0x42, paVar11);
    uVar8 = paVar11 | paVar4;
    paVar10 = (paVar11 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar10,
            paVar4,
            paVar11,
            0x1,
            0xa0046,
            0x0,
            0x860087,
            CONCAT22(struct_b_10.lpvoid_field_0x8, 0x10d),
            in_stack_0000ffac,
            in_stack_0000fe3a,
            in_stack_0000fe3e,
            in_stack_0000ff64,
            in_stack_0000ff68,
            in_stack_0000ff6c,
        );
    }
    mem_op_1000_179c(0x42, paVar10);
    uVar8 = paVar10 | paVar4;
    paVar11 = (paVar10 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar11,
            paVar4,
            paVar10,
            0x1,
            0xa0064,
            0x0,
            0x880089,
            CONCAT22(struct_b_10.lpvoid_field_0x8, 0x10e),
            in_stack_0000ffac,
            in_stack_0000fe3a,
            in_stack_0000fe3e,
            in_stack_0000ff64,
            in_stack_0000ff68,
            in_stack_0000ff6c,
        );
    }
    mem_op_1000_179c(0x42, paVar11);
    uVar8 = paVar11 | paVar4;
    paVar10 = (paVar11 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar10,
            paVar4,
            paVar11,
            0x1,
            0xa0082,
            0x0,
            0x820083,
            CONCAT22(struct_b_10.lpvoid_field_0x8, 0x10c),
            in_stack_0000ffac,
            in_stack_0000fe3a,
            in_stack_0000fe3e,
            in_stack_0000ff64,
            in_stack_0000ff68,
            in_stack_0000ff6c,
        );
    }
    mem_op_1000_179c(0x42, paVar10);
    uVar8 = paVar10 | paVar4;
    paVar11 = (paVar10 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar11,
            paVar4,
            paVar10,
            0x1,
            0xa00d2,
            0x0,
            0x8a008b,
            CONCAT22(struct_b_10.lpvoid_field_0x8, 0xbbb),
            in_stack_0000ffac,
            in_stack_0000fe3a,
            in_stack_0000fe3e,
            in_stack_0000ff64,
            in_stack_0000ff68,
            in_stack_0000ff6c,
        );
    }
    mem_op_1000_179c(0x42, paVar11);
    uVar8 = paVar11 | paVar4;
    paVar10 = (paVar11 & 0xffff0000);
    paVar12 = (paVar10 | uVar8);
    if (uVar8 == 0) {
        paVar4 = null_mut();
    } else {
        pass1_1008_3bd6(
            paVar12,
            paVar4,
            paVar11,
            0x1,
            0xa00a0,
            0x8e,
            0x8c008d,
            CONCAT22(struct_b_10.lpvoid_field_0x8, 0xbbc),
            in_stack_0000ffac,
            in_stack_0000fe3a,
            in_stack_0000fe3e,
            in_stack_0000ff64,
            in_stack_0000ff68,
            in_stack_0000ff6c,
        );
        paVar10 = paVar12;
    }
    puVar14 = mixed_1010_20ba(
        paVar10,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffe4, 0x3),
        in_stack_0000fe8c,
        in_stack_0000ffb0,
        in_stack_0000ffb6,
        in_stack_0000ffba,
    );
    uVar3 = (puVar14 >> 0x10);
    uVar2 = puVar14;
    uVar9 = uVar3;
    uVar5 = pass1_1010_a5ac(uVar2, uVar3, struct_b_10[0x8].field8_0x10);
    uVar6 = pass1_1010_ac62(uVar5, uVar9, uVar2, uVar3, 0x1e);
    if (uVar6 != 0) {
        pass1_1010_a5ca(uVar6, uVar9, uVar2, uVar3, uVar5);
        if (0x0 < uVar6) {
            pass1_1010_a58a(uVar6, uVar9, uVar2, uVar3, uVar5);
            //      if (uVar6 == 0) goto LAB_1040_bb26;
        }
    }
    win_e::enable_win_1040_9234(CONCAT22(paVar10, paVar4), 0x0); //
                                                                 // LAB_1040_bb26:
    uVar1 = &struct_b_10[0x7].field6_0xc;
    iVar7 = uVar1;
    uVar1 &= 0xffff0000;
    uVar13 = (uVar1 >> 0x10);
    SetWindowPos16(
        0x40,
        (iVar7 + 0x10),
        (iVar7 + 0xe),
        (iVar7 + 0xc),
        (uVar1 | iVar7 + 0xa),
        0x0,
        struct_b_10.lpvoid_field_0x8,
    );
    return;
}

pub fn mixed_win_sys_op_1008_016e(param_1: *mut astruct_823) {
    let mut puVar1: *mut u16;
    let mut uVar6: u16;
    let mut iVar3: i16;
    let mut uVar5: u16;
    let mut uVar9: u16;
    let mut uVar11: u16;
    let mut uVar8: u32;
    let mut DVar10: u16;
    let mut puVar4: *mut u8;
    let mut puVar14: u16;
    let mut uVar13: u16;
    let mut puVar12: *mut u8;
    let mut puVar13: *mut u8;
    let mut uVar7: u16;
    let mut in_EDX: u32;
    let mut struct_1: *mut Struct832;
    let mut unaff_DI: i16;
    let mut uVar10: u16;
    let mut uVar12: u16;
    let mut DVar16: u32;
    let mut puVar17: *mut u32;
    let mut pSVar18: *mut StructD;
    let mut in_stack_0000fe46: u16;
    let mut string_buf_13e: [u8; 0xac] = [0; 0xac];
    let mut local_92: [u8; 0x80] = [0; 0x80];
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut puStack14: *mut u32;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut uStack6: u16;
    let mut uStack4: u16;
    let mut uVar1: u32;
    let mut puVar2: *mut u16;
    let mut uVar4: *mut Struct20;
    let mut uVar2: *mut Struct20;
    let mut uVar3: *mut Struct20;
    let mut paVar14: *mut Struct57;
    let mut paVar15: *mut Struct57;
    let mut fn_ptr: *mut *mut code;

    uVar9 = (in_EDX >> 0x10);
    DVar16 = GetVersion16();
    DVar10 = (DVar16 >> 0x10);
    paVar14 = CONCAT22(uVar9, DVar10);
    uStack6 = (DVar16 & 0xffff);
    uVar6 = DVar16 & 0xff;
    uStack10 = ((DVar16 & 0xffff) >> 0x8);
    uStack8 = uVar6;
    if (uVar6 < 0x3) || (uVar6 == 0x3 && (uStack10 < 0xa)) {
        // 0x97
        uVar12 = 0x1000;
        mem_op_1000_179c(0xb4, paVar14);
        uStack16 = paVar14;
        puVar4 = (uStack16 | uVar6);
        paVar14 = (paVar14 & 0xffff0000);
        paVar15 = (paVar14 | ZEXT24(puVar4));
        uStack18 = uVar6;
        if (puVar4.is_null()) {
            iVar3 = 0;
        } else {
            uVar12 = &ctx.globals.PTR_LOOP_1050_1040;
            iVar3 = string_1040_8520(paVar15, CONCAT22(uStack16, uVar6), 0x0, 0x20010, 0x5dd05de);
            paVar14 = paVar15;
        }
        puStack14 = CONCAT22(paVar14, iVar3);
        fn_ptr = (*puStack14 + 0x74);
        (**fn_ptr)(uVar12, iVar3, paVar14);
        fn_ptr_op_1000_24cd(1);
    }
    debug_print_1008_6048(paVar14, s_version__d__d_1050_0012);
    if ((uStack8 == 0x3) && (0xb < uStack10)) {
        PTR_LOOP_1050_0010 = (&PTR_LOOP_1050_0000 + 1);
    }
    LoadString16(
        0x80,
        CONCAT22(0x1050, local_92),
        0x578,
        HINSTANCE16_1050_038c,
    );
    uVar5 = dos3_call_1000_51aa(local_92, 0x1050, 1);
    if (uVar5 != 0) {
        LoadString16(
            0x80,
            string_buf_13e,
            0x57b,
            HINSTANCE16_1050_038c,
        );
        LoadString16(
            0x80,
            CONCAT13(0x10, CONCAT12(0x50, &stack0xfe42)),
            0x62e,
            HINSTANCE16_1050_038c,
        );
        uVar5 = WinAPI16_MessageBox16(
            0x10,
            string_buf_13e,
            &stack0xfe42,
            0x0,
        );
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0x4, paVar14);
    uStack16 = paVar14;
    paVar14 = (paVar14 & 0xffff0000);
    if ((uStack16 | uVar5) == 0) {
        uVar9 = 0;
        uStack18 = uVar5;
    } else {
        uStack18 = uVar5;
        puVar17 = pass1_1008_5394(CONCAT22(uStack16, uVar5));
        paVar14 = (paVar14 & 0xffff0000 | puVar17 >> 0x10);
        uVar9 = SUB42(puVar17, 0x0);
    }
    // uVar10 = (param_1 >> 0x10);
    struct_1 = param_1;
    struct_1.field5_0x8 = uVar9;
    (struct_1.field5_0x8 + 0x2) = paVar14;
    uVar8 = struct_1.field5_0x8;
    puVar1 = struct_1.field5_0x8;
    _PTR_LOOP_1050_0298 = uVar8;
    *puVar1 = 0x70;
    // 0x1538
    (puVar1 + 0x2) = 0x1538;
    mem_op_1000_179c(0x126, paVar14);
    uVar11 = uVar8;
    uStack16 = paVar14;
    paVar15 = (paVar14 & 0xffff0000 | (uStack16 | uVar11));
    uStack18 = uVar11;
    if ((uStack16 | uVar11) != 0) {
        pSVar18 = pass1_1010_2024((uVar8 & 0xffff | paVar14 << 0x10));
        paVar15 = (paVar15 & 0xffff0000 | pSVar18 >> 0x10);
        uVar11 = pSVar18;
    }
    if (_u16_1050_0ed0 == 0) {
        debug_print_1008_6048(paVar15, s_New_failed_in_Op__Op_1050_0020);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xe8c, paVar15);
    uStack16 = paVar15;
    puVar12 = (uStack16 | uVar11);
    paVar14 = (paVar15 & 0xffff0000 | ZEXT24(puVar12));
    uStack18 = uVar11;
    if puVar12.is_null() == false {
        pass1_1010_7e40(puVar12, CONCAT22(uStack16, uVar11));
    }
    if _u16_1050_14cc == 0 {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__ResLibr_1050_0035);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xb0, paVar14);
    uStack16 = paVar14;
    paVar14 = (paVar14 & 0xffff0000 | (uStack16 | uVar11));
    uStack18 = uVar11;
    if ((uStack16 | uVar11) != 0) {
        pSVar18 = pass1_1038_aeca(CONCAT22(uStack16, uVar11));
        paVar14 = (paVar14 & 0xffff0000 | pSVar18 >> 0x10);
        uVar11 = pSVar18;
    }
    if _PTR_LOOP_1050_5b7c == 0 {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__DialogCtr_1050_0053);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xa, paVar14);
    uStack16 = paVar14;
    puVar13 = (uStack16 | uVar11);
    paVar14 = (paVar14 & 0xffff0000 | ZEXT24(puVar13));
    uStack18 = uVar11;
    if puVar13.is_null() == false {
        winapp::make_proc_inst_1038_cf6c(puVar13, CONCAT22(uStack16, uVar11));
    }
    if ctx.globals.u16_1050_5bc8 == 0 {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__DialogHand_1050_0073);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0x14, paVar14);
    uStack16 = paVar14;
    paVar14 = (paVar14 & 0xffff0000 | (uStack16 | uVar11));
    uStack18 = uVar11;
    if ((uStack16 | uVar11) != 0) {
        pass1_1008_5bdc(CONCAT22(uStack16, uVar11));
    }
    if ctx.globals.u16_1050_02a0 == 0 {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__Simulator_1050_0097);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xfc, paVar14);
    uStack16 = paVar14;
    uVar7 = uStack16 | uVar11;
    uStack18 = uVar11;
    if (uVar7 == 0) {
        uVar11 = 0;
        uVar7 = 0;
    } else {
        set_struct_op_1008_0536(CONCAT22(uStack16, uVar11), in_stack_0000fe46);
    }
    struct_1.field4_0x4 = uVar11;
    (&struct_1.field4_0x4 + 0x2) = uVar7;
    if struct_1.field4_0x4.is_null() {
        debug_print_1008_6048(uVar7, s_New_failed_in_Op__Op_1050_00b7);
        fn_ptr_op_1000_24cd(1);
    }
    winapp_b::win_ui_reg_class_1008_96d2(struct_1.field4_0x4);
    fn_ptr = (struct_1.field4_0x4 + 0x8);
    (**fn_ptr)(0x1000);
    uVar4 = struct_1.field4_0x4;
    HWND16_1050_0396 = (uVar4 + 0x8);
    uVar2 = struct_1.field4_0x4;
    fn_ptr = (struct_1.field4_0x4 + 0xc);
    (**fn_ptr)(0x1000, uVar2, (uVar2 >> 0x10), 0x3);
    uVar3 = struct_1.field4_0x4;
    UpdateWindow16((uVar3 + 0x8));
    return;
}

pub fn entry(
    ctx: &mut AppContext,
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut u8,
    mut param_5: u16,
    param_6: u16,
) -> *mut c_char {
    let mut win_version: u32 = 0u32;
    let mut var10: u32 = 0u32;
    let mut pstruct_825_var13: *mut Struct825 = ptr::null_mut();

    let mut result2 = CONCAT22(param_6, ctx.globals.PTR_LOOP_1050_5f84 as u16);
    loop {
        InitTask16(null_mut());
        ctx.globals.PTR_LOOP_1050_5f84 = result2;
        if param_3 != 0x0 {
            ctx.globals.PTR_LOOP_1050_5f7e = 0x1050;
            let mut var9 = param_5 < 0xff00;
            param_5 = param_5 + 0x100;
            if var9 {
                ctx.globals.PTR_LOOP_1050_5f50 = 0x1050;
                ctx.globals.PTR_LOOP_1050_5f48 = param_5;
                ctx.globals.PTR_LOOP_1050_5f4a = ctx.SI_REG;
                ctx.globals.HINSTANCE16_1050_5f4c = ctx.DI_REG;
                ctx.globals.PTR_LOOP_1050_5f4e = param_4;
                LockSegment16(0xffff);
                ctx.globals.PTR_LOOP_1050_5f84 = result2;
                win_version = GetVersion16();
                ctx.globals.PTR_LOOP_1050_5f84 = result2;
                ctx.globals.WIN_VERSION_1050_5f80 = win_version;
                ctx.AH_REG = 0x30; // get dos version
                let mut result = swi(ctx, 0x21);
                // let mut u32_var12 = u32_var11;
                // u32_var11 = func_ptr_3::CODE8();
                result.call(ctx);
                result2 = result.u32_call_result();
                // PTR_LOOP_1050_5f84 = u32_var12;
                ctx.globals.DAT_1050_5f82 = result2;
                ctx.globals.DAT_1050_5f87 = 0;
                let wait_result = WinAPI16_WaitEvent16(0x0);
                ctx.globals.PTR_LOOP_1050_5f84 = result2;
                param_3 = InitApp16(ctx.globals.HINSTANCE16_1050_5f4c) as u16;
                ctx.globals.PTR_LOOP_1050_5f84 = result2;
                if param_3 != 0x0 {
                    break;
                }
            }
        }
        // param_3 = ((param_3 >> 0x8) << 0x8) | 0xff;
        dos_exit_program_1000_24db(ctx, param_3);
        ctx.globals.PTR_LOOP_1050_5f84 = result2;
    }
    dos_get_interrupt_vector_1000_23ea(param_4, 0x1050, 0x0, 0x1050);
    ctx.globals.PTR_LOOP_1050_5f84 = result2;
    entry_1000_262c(ctx, result2, 0x238d, 0x1538);
    ctx.globals.PTR_LOOP_1050_5f84 = result2;
    pass1_1000_27d6(result2);
    win_version = ret_op_1000_55ac();
    let var4 = win_version as u16;
    init_1000_23be(param_5, (var10 >> 0x10) as u16);
    exit_op_1000_24cd(ctx, var4);
    // TODO
    // paVar13 =  CONCAT22(uVar4, 0x15);
    pass1_1000_25a8();
    pass1_1000_2913(ctx, 0x15);
    let mut string_var4 = poss_str_op_1000_28dc(pstruct_825_var13);
    if string_var4.is_null() == false {
        let mut var5 = 0x9;
        if string_var4[0] == 'M' {
            var5 = 0xf;
        }
        string_var4 = string_var4 + var5;
        let mut varb = 0x22;
        let mut string_var8 = string_var4;
        loop {
            if varb == 0x0 {
                break;
            }
            varb += -0x1;
            let var1 = string_var8;
            string_var8 = string_var8 + 1;
            if !(var1.field0_0x0 != '\r') {
                break;
            }
        }
        (string_var8 - 1) = '\0';
    }
    FatalAppExit16(0x0, string_var4);
    FatalExit();
    let mut string_var7 = ctx.globals.PTR_LOOP_1050_63fe;
    loop {
        let mut string_var1 = string_var7;
        string_var7 = string_var7 + 1;
        let mut string_var5 = string_var7;
        string_var5 = string_var1 + 1;
        // TODO: string_var1 == param_2 where param_2 is a u16 possibly an address
        if string_var5.is_null() {
            return string_var5;
        }
        let mut var6 = -0x1;
        loop {
            if var6 == 0x0 {
                break;
            }
            var6 += -0x1;
            let var2 = string_var7;
            string_var7 = (string_var7 + 1);
            if *var2 == 0 {
                break;
            }
        }
    }
}

pub fn dos_exit_program_1000_24db(ctx: &mut AppContext, mut exit_code: u16) {
    //        1000:254c b4 4c           MOV        AH,0x4c

    ctx.globals.u8_1050_5fc9 = 0;
    fn_ptr_op_1000_2594(ctx);
    fn_ptr_op_1000_2594(ctx);
    dos_set_interrupt_vector(ctx);
    ctx.AH_REG = 0x4c; // Exit Program
    let result = swi(ctx, 0x21);
    // TODO: marshal argument into context
    // sys_call_ptr::CODE(exit_code);
    result.call(ctx);
    return;
}

// WARNING: Restarted to delay deadcode elimination for space: ram
pub fn entry_1000_262c(ctx: &mut AppContext, param_1: u16, mut param_2: u16, mut param_3: u16) {
    let mut var3: i16;
    let mut var4: u16;
    let mut var5: u16;
    let mut var6: u16;
    let mut var7: u16;
    let mut var8: u16;
    let mut var10: u16;
    let mut var11: *mut *mut c_char;
    let mut var12: *mut c_char;
    let mut var13: *mut c_char;
    let mut var14: u16;
    let mut var15: u16;
    let mut var16: *mut u8;

    // PTR_LOOP_1050_5fd2 = param_2;
    ctx.data_map[0x1050][0x5fd2].store_u16(param_2);
    // PTR_LOOP_1050_5fd4 = param_3;
    ctx.data_map[0x1050][0x5fd4].store_u16(param_3);
    param_3 = 0x263d;
    param_2 = exit_op_1000_2950(ctx, 0x8, param_1, 0x104);
    param_3 = param_1;
    // PTR_LOOP_1050_5fc2 = param_2;
    ctx.data_map[0x1050][0x5fc2] = param_2;
    // PTR_LOOP_1050_5fc4 = param_1;
    ctx.data_map[0x1050][0x5fc4] = param_1;
    CONCAT22(param_1, param_2);
    let mut out_module_fname: Vec<u8> = Vec::with_capacity(512);
    let mut result_str_len = GetModuleFileName16(
        0x104,
        out_module_fname.as_mut_ptr() as *mut c_char,
        ctx.globals.HINSTANCE16_1050_5f4c,
    );
    param_2[result_str_len] = '\0';
    let mut i_var4 = 0x1;
    ctx.globals.PTR_LOOP_1050_5fb8 = (ctx.globals.PTR_LOOP_1050_0000 + 1);
    // TODO:
    // LAB_1000_266c:
    let mut var1 = 0u16;
    let mut var2 = 0u16;
    let mut var7 = ctx.globals.PTR_LOOP_1050_5f7e;
    loop {
        loop {
            let mut var17 = var7;
            var7 = var7 + 1;
            var2 = var17;
            if var2 != ' ' {
                break;
            }
        }
        if var2 != '\t' {
            break;
        }
    }
    if (var2 != '\r') && (var2 != '\0') {
        ctx.globals.PTR_LOOP_1050_5fb8 = ctx.globals.PTR_LOOP_1050_5fb8 + 1;
        loop {
            var7 -= 1;
            // TODO:
            // LAB_1000_267f:
            var1 = var7;
            var7 += 1;
            var2 = *var1;
            if (var2 == ' ') || (var2 == '\t') {
                // TODO:
                // goto LAB_1000_266c;
            }
            if (var2 == '\r') || (var2 == '\0') {
                break;
            }
            if var2 == '\"' {
                // LAB_1000_26b8:
                loop {
                    loop {
                        loop {
                            var1 = var7;
                            var7 = var7 + 1;
                            var2 = *var1;
                            if (var2 == '\r') || (var2 == '\0') {
                                // TODO:
                                // goto LAB_1000_26e8;
                            }
                            if var2 == '\"' {
                                // TODO:
                                // goto LAB_1000_267f;
                            }
                            if var2 == '\\' {
                                break;
                            }
                            i_var4 += 0x1;
                        }
                        var7 = 0;
                        loop {
                            var13 = var7;
                            var7 += 0x1;
                            var7 = var13 + 1;
                            var2 = *var13;
                            if var2 != '\\' {
                                break;
                            }
                        }
                        if var2 == '\"' {
                            break;
                        }
                        i_var4 += var7;
                        var7 = var13;
                    }
                    i_var4 = i_var4 + (var7 >> 1) + ((var7 & 1) != 0);
                    if (var7 & 1) == 0 {
                        break;
                    }
                }
                // TODO:
                // goto LAB_1000_267f;
            }
            if var2 != '\\' {
                i_var4 += 0x1;
                // TODO:
                // goto LAB_1000_267f;
            }
            var6 = 0;
            loop {
                var6 += 0x1;
                var1 = var7;
                var7 = var7 + 1;
                var2 = *var1;
                if var2 != '\\' {
                    break;
                }
            }
            if var2 == '\"' {
                i_var4 = i_var4 + (var6 >> 1) + ((var6 & 1) != 0);
                if (var6 & 1) == 0 {
                    // TODO:
                    // goto LAB_1000_26b8;
                }
                // TODO:
                // goto LAB_1000_267f;
            }
            i_var4 += var6;
        }
    }
    // TODO:
    // LAB_1000_26e8:
    param_3 = 0x1050u16;
    var3 = -((ctx.globals.PTR_LOOP_1050_5fb8 + (ctx.globals.PTR_LOOP_1050_5fb8 + 1) * 0x4 + i_var4 + 1) & 0xfffe);
    PTR_LOOP_1050_5fba = &stack0x0004 + var3;
    PTR_LOOP_1050_5fbc = 0x1050u16;
    var13 = stack0x0004 + (ctx.globals.PTR_LOOP_1050_5fb8 + 1) * 0x4 + var3;
    param_3 + var3 = 0x1050u16;
    var16 = ctx.globals.PTR_LOOP_1050_5fc4;
    var14 = (param_3 + var3);
    (&stack0x0004 + var3) = ctx.globals.PTR_LOOP_1050_5fc2;
    (&stack0x0006 + var3) = var16;
    var11 = (&stack0x0008 + var3);
    param_3 + var3 = stack0x0004 + var3;
    param_2 + var3 = 0x1538;
    stack0xfffe + var3 = 0x271f;
    var4 = winapp::pass1_1000_29dc(0x1050);
    var15 = &ctx.globals.PTR_LOOP_1050_5f7e;
    var7 = 0x0081;
    // TODO:
    // LAB_1000_272e:
    loop {
        loop {
            var1 = var7;
            var7 = var7 + 1;
            var2 = *var1;
            if var2 != ' ' {
                break;
            }
        }
        if var2 != '\t' {
            break;
        }
    }
    if (var2 == '\r') || (var2 == '\0') {
        //  TODO:
        //  LAB_1000_27c1:
        (param_3 + var3) = 0x1538;
        (param_2 + var3) = 0x27c5;
        var5 = pass1_1000_29dc(0x1050);
        *var11 = null_mut();
        var11[0x1] = null_mut();
        // WARNING: Could not recover jumptable at 0x100027d2. Too many branches
        // WARNING: Treating indirect jump as call
        ctx.globals.PTR_LOOP_1050_5fd2();
        ctx.globals._PTR_LOOP_1050_5fc2 = CONCAT22(ctx.globals.PTR_LOOP_1050_5fc4, ctx.globals.PTR_LOOP_1050_5fc2);
        return;
    }
    *var11 = var13;
    var11[0x1] = 0x1050;
    var11 = var11 + 2;
    loop {
        var7 = var7 - 0x1;
        //  TODO:
        //  LAB_1000_274f:
        var1 = var7;
        var7 = var7 + 1;
        var2 = *var1;
        if (var2 == ' ') || (var2 == '\t') {
            var1 = var13;
            var13 = var13 + 1;
            *var1 = '\0';
            // TODO:
            // goto LAB_1000_272e;
        }
        if (var2 == '\r') || (var2 == '\0') {
            //
            //            LAB_1000_27be:
            *var13 = '\0';
            // TODO:
            // goto LAB_1000_27c1;
        }
        var12 = var7;
        if var2 == '\"' {
            // TODO:
            // LAB_1000_278b:
            loop {
                var7 = var12 + 1;
                var2 = *var12;
                if (var2 == '\r') || (var2 == '\0') {
                    // TODO:
                    // goto LAB_1000_27be;
                }
                if var2 == '\"' {
                    break;
                }
                if var2 == '\\' {
                    var10 = 0;
                    loop {
                        var12 = var7;
                        var10 += 0x1;
                        var7 = var12 + 1;
                        var2 = *var12;
                        if var2 != '\\' {
                            break;
                        }
                    }
                    if var2 == '\"' {
                        // for (uVar11 = uVar10 >> 0x1; uVar11 != 0; uVar11 -= 1)
                        for uVar11 in uVar10 >> 1..0 {
                            var1 = var13;
                            var13 = var13 + 1;
                            *var1 = '\\';
                        }
                        if (var10 & 1) == 0 {
                            break;
                        }
                        var1 = var13;
                        var13 = var13 + 1;
                        *var1 = '\"';
                        var12 = var7;
                    } else {
                        // for (; uVar10 != 0; uVar10 -= 1) {
                        while uVar10 != 0 {
                            var1 = var13;
                            var13 = var13 + 1;
                            *var1 = '\\';
                            uVar10 -= 1;
                        }
                    }
                } else {
                    var1 = var13;
                    var13 = var13 + 1;
                    *var1 = var2;
                    var12 = var7;
                }
            }
            // TODO:
            // goto LAB_1000_274f;
        }
        if var2 != '\\' {
            var1 = var13;
            var13 = var13 + 1;
            *var1 = var2;
            // TODO:
            // goto LAB_1000_274f;
        }
        var8 = 0;
        loop {
            var8 += 0x1;
            var1 = var7;
            var7 = var7 + 1;
            var2 = *var1;
            if var2 != '\\' {
                break;
            }
        }
        if var2 == '\"' {
            // for (uVar9 = uVar8 >> 0x1; uVar9 != 0; uVar9 -= 1)
            for uVar9 in uVar8 >> 0x1..0 {
                var1 = var13;
                var13 = var13 + 1;
                *var1 = '\\';
            }
            var12 = var7;
            if (var8 & 1) == 0 {
                // TODO:
                // goto LAB_1000_278b;
            }
            var1 = var13;
            var13 = var13 + 1;
            *var1 = '\"';
            // TODO: goto LAB_1000_274f;
        }
        // for (; uVar8 != 0; uVar8 -= 1)
        while uVar8 != 0 {
            var1 = var13;
            var13 = var13 + 1;
            *var1 = '\\';
            uVar8 -= 1;
        }
    }
}

pub fn exit_op_1000_24cd(ctx: &mut AppContext, mut param_1: u16) {
    let mut var2: i16;
    let mut var3: u16;
    let mut var4: u16;
    let mut var5: u16;
    let mut var6: u16;
    let mut var7: u16;

    ctx.globals.u8_1050_5fc9 = u8::try_from('\0').unwrap();
    fn_ptr_op_1000_2594();
    fn_ptr_op_1000_2594();
    ret_op_1000_55ac();
    fn_ptr_op_1000_2594();
    fn_ptr_op_1000_2594();
    dos_set_interrupt_vector(ctx);
    // terminate with return code
    ctx.AH_REG = 0x4c;
    let result = swi(ctx, 0x21);
    result.call(ctx);
}

pub fn mixed_sys_op_1000_40af(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
) -> *mut i16 {
    let mut ppa_var1: *mut *mut struct_824;
    let mut pc_var2: *mut c_char;
    let mut pu_var4: *mut u16;
    let mut pc_var5: *mut c_char;
    let mut i_var6: i16;
    let mut ppa_var7: *mut *mut struct_824;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut i_var8: i16;
    let mut hglobal_7: HGLOBAL16;
    // pub fn *SVar8;
    let mut ppa_var8: *mut *mut struct_824;
    let mut unaff_si: i16;
    let mut pi_var9: *mut i16;
    let mut pc_var10: *mut c_char;
    let mut hglobal_di: *mut astruct_824;
    let mut pu_var11: *mut u16;
    let mut unaff_cs: u8;
    let mut unaff_ss: u16;
    let mut b_var12: bool;
    // pub fn *pvVar13;
    let mut pa_var14: *mut Struct825;
    let mut pu_var3: *mut u16;
    let mut u_var13: u8;
    let mut u_var14: u8;
    let mut i_var12: i16;
    let mut temp_5fa27366cb: *mut astruct_824;

    loop {
        u_var7 = (param_1 * param_3);
        u_var8 = param_2 * param_3 + (param_1 * param_3 >> 0x10);
        if ((u_var8 | u_var7) != 0) {
            pi_var9 = null_mut();
            if ((u_var8 < 0x3) && (u_var8 < 0x2 || (u_var7 == 0))) {
                if (u_var8 == 0) {
                    u_var7 = u_var7 + 0xfff & 0xf000;
                    if (u_var7 == 0) {
                        u_var8 = 0x1;
                    }
                } else if ((param_3 - 0x1 & param_3) != 0) {
                    pi_var9 = ((u_var8 << 0x10) % param_3);
                    b_var12 = CARRY2(u_var7, pi_var9);
                    u_var7 += pi_var9;
                    if (b_var12) {
                        // TODO: goto LAB_1000_41aa;
                    }
                    u_var8 = 0x1;
                }
            } else if ((param_3 - 0x1 & param_3) != 0) {
                // TODO: goto LAB_1000_41aa;
            }
            hglobal_7 = GlobalAlloc16(CONCAT13((u_var8 >> 0x8), CONCAT12(u_var8, u_var7)), 0x20);
            if ((hglobal_7 != 0) && ((u_var7 & 1) != 0)) {
                pvVar13 = WIN16_GlobalLock16(hglobal_7);
                SVar8 = pvVar13;
                if ((SVar8 != 0) || (pvVar13.is_null())) {
                    pa_var14 = CONCAT22(hglobal_7, 0x12);
                    u_var13 = '\x12';
                    u_var14 = '\0';
                    pass1_1000_25a8();
                    pass1_1000_2913(ctx, CONCAT11(u_var14, u_var13));
                    pc_var5 = poss_str_op_1000_28dc(pa_var14);
                    if (pc_var5.is_null()) {
                        // TODO: goto LAB_1000_28cb;
                    }
                    i_var6 = 0x9;
                    if (*pc_var5 == 'M') {
                        i_var6 = 0xf;
                    }
                    pc_var5 = pc_var5 + i_var6;
                    i_var6 = 0x22;
                    pc_var10 = pc_var5;
                    break;
                }
                hglobal_7 = block_1000_4000::pass1_1000_422a((pvVar13 >> 0x10), hglobal_7);
                if (hglobal_7 == 0) {
                    GlobalUnlock16(u_var8);
                    GlobalFree16(hglobal_di);
                    hglobal_7 = 0;
                }
            }
            unaff_cs = 0x38;
            if (hglobal_7 != 0) {
                pu_var11 = null_mut();
                // for (; unaff_si != 0; unaff_si += -1)
                while unaff_si != 0 {
                    // for (iVar6 = -0x8000; i_var6 != 0; i_var6 += -1)
                    for ivar6 in -0x8000..0 {
                        pu_var3 = pu_var11;
                        pu_var11 = pu_var11 + 1;
                        *pu_var3 = 0;
                    }
                    hglobal_7 += 0x100;
                    unaff_si -= 1;
                }
                if (hglobal_di.is_null() == false) {
                    // for (; hglobal_di.is_null() == false; hglobal_di = hglobal_di -1)
                    while hglobal_di.is_null() == false {
                        pu_var4 = pu_var11;
                        pu_var11 = (pu_var11 + 1);
                        *pu_var4 = 0;
                        hglobal_di -= 1;
                    }
                }
                return pi_var9;
            }
        } //
          //        LAB_1000_41aa:
        if ((u16_1050_618e | PTR_LOOP_1050_618c) == 0) {
            return NULL;
        }
        i_var8 = (PTR_LOOP_1050_618c)(unaff_cs, param_3, param_1, param_2);
        if (i_var8 == 0) {
            return NULL;
        }
    }
    loop {
        i_var6 += -0x1;
        pc_var2 = pc_var10;
        pc_var10 = pc_var10 + 1;
        if (*pc_var2 == '\r') {
            break;
        }
        if (i_var6 == 0) {
            break;
        }
    }
    pc_var10[-0x1] = '\0'; //
                           //    LAB_1000_28cb:
    FatalAppExit16(CONCAT13(0x10, CONCAT12(0x50, pc_var5)), 0x0);
    FatalExit();
    ppa_var8 = &PTR_LOOP_1050_63fe;
    loop {
        ppa_var1 = ppa_var8;
        ppa_var8 = ppa_var8 + 1;
        temp_5fa27366cb = *ppa_var1;
        ppa_var7 = ppa_var8;
        if ((temp_5fa27366cb == hglobal_di)
            || (ppa_var7 = (temp_5fa27366cb + 1), ppa_var7.is_null()))
        {
            return ppa_var7;
        }
        i_var6 = -0x1;
        loop {
            if (i_var6 == 0) {
                break;
            }
            i_var6 += -0x1;
            ppa_var1 = ppa_var8;
            ppa_var8 = (ppa_var8 + 1);
            if ppa_var1.is_null() {
                break;
            }
        }
    }
}

pub fn exit_op_1000_2950(
    ctx: &mut AppContext,
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
) -> u16 {
    let mut var2: u16;
    let mut var3: *mut c_char;
    let mut var6: i16;

    let mut var9: *mut c_char;

    // let mut paVar10: *mut Struct825;

    // let mut var4 = PTR_LOOP_1050_6066;
    let mut var4: u16 = ctx.data_map[0x1050][0x6066].u16_value();

    // PTR_LOOP_1050_6066 = PTR_LOOP_1050_1000;
    ctx.data_map[0x1050][0x6066].set_u16(0x1000);

    let mut var8 = mem_1000_167a(param_2, param_3);
    PTR_LOOP_1050_6066 = var4;
    if (param_2 | var8) != 0 {
        return var8;
    }
    let mut var10 = CONCAT22(ctx.ES_REG, param_1);
    pass1_1000_25a8();
    pass1_1000_2913(ctx, param_1);
    let mut var5 = poss_str_op_1000_28dc(var10);
    if var5.is_null() == false {
        var6 = 0x9;
        if *var5 == 'M' as i8 {
            var6 = 0xf;
        }
        var5 = var5 + var6;
        var6 = 0x22;
        var9 = var5;
        loop {
            if var6 == 0 {
                break;
            }
            var6 += -0x1;
            var3 = var9;
            var9 = var9 + 1;
            if *var3 == '\r' as i8 {
                break;
            }
        }
        var9[-0x1] = '\0';
    }
    FatalAppExit16(0, var5);
    FatalExit();
    var8 = PTR_LOOP_1050_63fe;
    loop {
        let mut var1 = var8;
        var8 = var8 + 1;
        var2 = var1;
        let mut var7 = var8;
        var7 = var2 + 1;
        if var2 == ctx.BP_REG || var7.is_null() {
            return var7;
        }
        var6 = -0x1;
        loop {
            if var6 == 0 {
                break;
            }
            var6 += -0x1;
            var1 = var8;
            var8 = (var8 + 1);
            if var1 == '\0' {
                break;
            }
        }
    }
}
