// #include "ui_ops_6.h"

// #include "address_tables/function_tables.h"
// #include "draw_ops/draw_ops_2.h"
// #include "draw_ops/draw_ops_3.h"
// #include "fn_ptr_ops/fn_ptr_ops_6.h"
// #include "fn_ptr_ops/fn_ptr_ops_7.h"
// #include "globals.h"
// #include "op_int.h"
// #include "op_winapi.h"
// #include "op_windef.h"
// #include "struct_43.h"
// #include "structs/structs_0xx/structs_3x.h"
// #include "structs/structs_5xx/struct_502.h"
// #include "sys_ops/sys_ops_6.h"
// #include "sys_ops/sys_ops_8.h"
// #include "sys_ops/sys_ops_9.h"
// #include "ui_ops_1.h"
// #include "ui_ops_4.h"
// #include "unk/unk_12.h"
// #include "unk/unk_14.h"
// #include "unk/unk_17.h"
// #include "unk/unk_5.h"
// #include "utils.h"
// #include "win_ops/win_ops_1.h"
// #include "win_ops/win_ops_3.h"
// #include "win_ops/win_ops_5.h"

// #include <stdbool.h>
// #include <stddef.h>

use std::ffi::c_void;

pub fn destroy_window_1020_3b3e(param_1: *mut Struct30, param_2: HWND16)

{
    let mut pu32_var1: *mut u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct30;
    let mut uVar5: *mut Struct30;
    let mut uVar6: *mut Struct30;
    let mut HVar5: HWND16;
    let mut ss_var1: u16;

    uVar6              = (param_1 >> 0x10);
    uVar5              = param_1;
    uVar5.field_0x10e = 0x0;
    HVar5              = param_2;
    if(uVar5.field_0x10a != 0x0)
    {
        HVar5 = LAST_SEGMENT;
        DestroyWindow16(param_2);
    }
    pu32_var1 = uVar5.field_0xf6;
    uVar3  = uVar5.field_0xf8;
    if((uVar3 | pu32_var1) != 0x0)
    {
        ppcVar2 = *pu32_var1;
        (**ppcVar2)(HVar5, pu32_var1);
    }
    &uVar5.field_0xf6 = 0x0;
    if(uVar5.field_0xfa != 0x0)
    {
        uVar3 = uVar6 | uVar5;
        if(param_1 == 0x0)
        {
            paVar4 = 0x0;
        }
        else
        {
            uVar3  = &uVar5.field_0xf2;
            paVar4 = uVar6;
        }
        pass1_1010_1ea6(uVar5.field_0xfa, str_var1(paVar4, uVar3), ss_var1);
    }
    uVar5.field_0xfa = 0x0;
}

pub fn pass1_1020_3c8c(param_1: u32, param_2: u32, param_3: u16)

{
    pt_in_rect_1018_1bda(*(param_1 + 0xfa), param_2, (param_2 >> 0x10), param_3);
}

Struct3 *pass1_1020_3ca6(param_1: *mut Struct3, param_2: u8, param_3: u16)

{
    let mut uVar1: u32;
    let mut puStack10: *mut u16;

    uVar1         = param_1 & 0xffff0000;
    param_1       = (uVar1 | param_1 - 0xf2);
    param_1 = (uVar1 >> 0x10);
    if(param_1 == 0x0)
    {
        param_1 = 0x0;
        param_1 = 0x0;
    }
    puStack10       = str_var1(param_1, param_1);
    *puStack10      = addr_table_1008_380a[36]; // 0x389a
    param_1.field_0x2 = SEG_1008;
    cleanup_menu_ui_op_1020_795c(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, SEG_1000);
    }
    return param_1;
}

pub fn enable_window_1020_1bd4(globals: &mut Globals, param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: HWND16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut bVar2: bool;
    let mut in_AX: u16;
    let mut iVar3: i16;
    let mut in_DX: *mut u8;
    let mut pu_var4: *mut u8;
    let mut uVar5: u16;
    let mut ss_var1: u16;
    let mut puStack12: *mut u32;

    bVar2 = false;
    pass1_1020_1d8e(str_var1(param_2, param_1), str_var1(param_4, param_3));
    if(in_AX != 0x0)
    {
        if(in_AX < 0x2)
        {
            bVar2 = true;
        }
        else
        {
            GetDlgItem16(param_5, 0x1);
            pass1_1010_4e8c(*(param_1 + 0x8e), ss_var1);
            in_AX = EnableWindow16(SEG_1010, 0x1);
            pass1_1010_4df0(*(param_1 + 0x8e), in_DX, ss_var1);
            if((in_AX == 0x0) && (bVar2 = true, (param_1 + 0x96) == 0x0))
            {
                in_AX = EnableWindow16(SEG_1010, 0x0);
            }
        }
    }
    if(bVar2)
    {
        uVar5 = SEG_1000;
        mem_op_1000_179c(0xb4, in_DX, 0);
        pu_var4 = (in_DX | in_AX);
        if(pu_var4 == 0x0)
        {
            iVar3  = 0x0;
            pu_var4 = 0x0;
        }
        else
        {
            uVar5 = SUB42(&SEG_1040, 0x0);
            iVar3 = string_1040_8520(NULL,
                                     str_var1(in_DX, in_AX),
                                     (param_1 + 0x6),
                                     0x30,
                                     0x2,
                                     0x57b,
                                     0x62a,
                                     pu_var4,
                                     ss_var1);
        }
        puStack12 = str_var1(pu_var4, iVar3);
        ppcVar1   = (*puStack12 + 0x74);
        (**ppcVar1)(uVar5, iVar3, pu_var4);
    }
}

pub fn set_win_tet_1020_1d2a(param_1: u16, param_2: u16, in_win_text_3: SEGPTR, param_4: u16, in_dlg_id_5: u16, in_hwnd_6: HWND16)

{
    GetDlgItem16(in_hwnd_6, in_dlg_id_5);
    SetWindowText16(LAST_SEGMENT, in_win_text_3);
    return;
}

pub fn pass1_1020_1d8e(param_1: u32, param_2: u32)

{
    pt_in_rect_1010_4e08(*(param_1 + 0x8e), param_2, (param_2 >> 0x10), SEG_1010);
    return;
}

BOOL16 destroy_win_1020_1dea(param_1: HWND16)

{
    let mut BVar1: BOOL16;
    WORD   WVar2;

    BVar1 = IsWindow16(param_1);
    if(BVar1 != 0x0)
    {
        WVar2 = GetWindowWord16(LAST_SEGMENT, -0xc);
        if(WVar2 == 0x175)
        {
            DestroyWindow16(LAST_SEGMENT);
            return 0x0;
        }
    }
    return 0x1;
}


u16 destroy_win_1020_1e1e(param_1: HWND16)

{
    let mut BVar1: BOOL16;
    WORD   WVar2;

    BVar1 = IsWindow16(param_1);
    if(BVar1 != 0x0)
    {
        WVar2 = GetWindowWord16(LAST_SEGMENT, -0xc);
        if((WVar2 != 0x1) && (WVar2 != 0x175))
        {
            DestroyWindow16(LAST_SEGMENT);
        }
    }
    return 0x1;
}

Struct18 *pass1_1020_1e54(globals: &mut Globals,param_1: *mut Struct18, param_2: u8)

{
    ui_cleanup_op_1040_782c(param_1, SEG_1040);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, SEG_1000);
    }
    return param_1;
}


pub fn pass1_1020_26a6(param_1: u32)

{
    let mut pu32_var1: *mut u32;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut u_var4: u16;

    u_var4  = (param_1 >> 0x10);
    pu32_var1 = *(param_1 + 0xee);
    u_var2  = *(param_1 + 0xf0);
    if((u_var2 | pu32_var1) != 0x0)
    {
        ppcVar3 = *pu32_var1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1, SEG_1008);
}

pub fn pass1_1020_28fc(param_1: *mut Struct3, param_2: u16)

{
    param_1.address_offset_field_0x0 = addr_table_1020_2e4a;//0x2e4a;
    param_1.field_0x2 = SEG_1020;
    cleanup_menu_ui_op_1020_795c(param_1, param_2);
}

pub fn pass1_1020_2a6a(param_1: u32, param_2: u16)

{
    get_win_ui_info_op_1020_7a50(param_1, param_2);
    pass1_1018_0ae8(*(param_1 + 0xf2), 0x0);
    destroy_icon_1020_2c88(param_1, SEG_1018);
}

pub fn bring_window_to_top_1020_2aae(param_1: u32, param_2: u32)

{
    let mut ss_var1: u16;
    pass1_1008_3e0e(param_1);
    BringWindowToTop16(SEG_1008);
    pass1_1018_169e(*(param_1 + 0xf2), param_2, ss_var1);
}

pub fn pass1_1020_0aa6(param_1: u32, param_2: u16)

{
    win_ui_palette_op_1020_0cd2((param_1 + 0xe2), param_2);
    return;
}

pub fn win_ui_palette_op_1020_0cd2(param_1: u32, param_2: HWND16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u32;
    let mut ppcVar3: *mut *mut c_void;
    let mut u_var4: u32;
    let mut uVar5: u16;
    let mut hdc: HDC16;
    let mut b_force_background: HDC16;
    let mut b_force_background_00: HPALETTE16;
    let mut UVar6: u16;
    let mut dx_var1: u16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut paStack10: *mut Struct13;
    let mut u_stack6: u16;

    u_var4   = (param_1 + 0x6);
    uVar8   = (u_var4 >> 0x10);
    iVar7   = u_var4;
    pu_var2  = (iVar7 + 0xa);
    uVar1   = *(iVar7 + 0xc);
    u_stack6 = pu_var2;
    uVar5   = uVar1 | u_stack6;
    if(uVar5 != 0x0)
    {
        ppcVar3 = (*pu_var2 + 0x14);
        (**ppcVar3)(param_2, u_stack6, uVar1);
        paStack10 = str_var1(dx_var1, uVar5);
        uVar5     = dx_var1 | uVar5;
        if(uVar5 != 0x0)
        {
            hdc                = GetDC16(param_2);
            b_force_background = hdc;
            create_palette_1008_4e38(paStack10, SEG_1008, uVar5);
            b_force_background_00 = SelectPalette16(SEG_1008, 0x0, b_force_background);
            UVar6                 = RealizePalette16(LAST_SEGMENT);
            SelectPalette16(LAST_SEGMENT, 0x1, b_force_background_00);
            DeleteObject16(LAST_SEGMENT);
            if(0x0 < UVar6)
            {
                InvalidateRect16(LAST_SEGMENT, (&PTR_LOOP_1050_0000 + 0x1), 0x0);
            }
            ReleaseDC16(LAST_SEGMENT, hdc);
            return;
        }
    }
    return;
}

pub fn pass1_1020_0e2c(param_1: u32, param_2: u16)

{
    get_win_ui_info_op_1020_7a50(param_1, param_2);
    cleanup_ui_op_1020_1038(param_1);
}

pub fn pass1_1020_0e8e(param_1: i16, param_2: u16, param_3: i16, param_4: i16, param_5: i16, param_6: u16, param_7: u16)

{
    fn_ptr_1 *ppcVar1;

    win_ui_cursor_op_1020_1294(NULL, str_var1(param_2, param_1), param_3, param_4, param_6, param_7);
    if(param_5 == 0x0)
    {
        ppcVar1 = ((param_1 + 0x4) + 0x5c);
        (**ppcVar1)();
    }
}

pub fn enable_menu_1020_1000(HMENparam_1: u16, param_2: i16)

{
    if(param_2 != 0x0)
    {
        return;
    }
    EnableMenuItem16(param_1, 0x400, 0x3);
}

pub fn win_ui_cursor_op_1020_1294(globals: &mut Globals, param_1: u32, param_2: i16, param_3: i16, param_4: u16, param_5: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut in_AX: u16;
    let mut HVar2: HCURSOR16;
    let mut HVar3: HCURSOR16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut local_12: i16;
    let mut local_10: i16;
    let mut puStack14: *mut u16;
    let mut puStack10: *mut u32;
    let mut local_6: i16;
    let mut iStack4: i16;

    pass1_1030_8344(globals._PTR_LOOP_1050_5748, (globals._PTR_LOOP_1050_5748 >> 0x10), 0x4000001);
    if((param_4 | in_AX) == 0x0)
    {
        local_6   = param_3;
        iStack4   = param_2;
        uVar5     = (param_1 >> 0x10);
        iVar4     = param_1;
        puStack10 = pass1_1010_40cc(*(iVar4 + 0xf2), param_2, 0x0);
        uVar6     = *(iVar4 + 0xf2);
        puStack14 = (uVar6 & 0xffff0000 | (uVar6 + 0x76));
        pass1_1008_3e94(puStack14, str_var1(param_5, &local_12), str_var1(param_5, &local_10));
        local_6 = local_6 - local_10;
        iStack4 = iStack4 - local_12;
        iVar4   = pt_in_rect_1010_40f8(*(iVar4 + 0xf2), str_var1(param_5, &local_6), SEG_1010);
        if(iVar4 != -0x1)
        {
            uVar6   = 0x0;
            HVar2   = LoadCursor16(SEG_1010, 0x7f02);
            uVar6   = uVar6 & 0xffff0000 | HVar2;
            HVar3   = SetCursor16(LAST_SEGMENT);
            ppcVar1 = (*puStack10 + 0x4);
            (**ppcVar1)(LAST_SEGMENT, puStack10, (puStack10 >> 0x10), iVar4, iVar4 >> 0xf, iVar4, 0x2, uVar6, HVar3, HVar2);
            pass1_1008_3e0e(param_1);
            SetCursor16(SEG_1008);
        }
    }
}


Struct3 *pass1_1020_135e(param_1: *mut Struct18, param_2: u8, param_3: u16)

{
    cleanup_menu_ui_op_1020_795c(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, SEG_1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn unk_win_ui_op_1020_1418(param_1: *mut Struct40, param_2: u32, param_3: u16)

{
    let mut uVar1: u32;
    let mut p_var2: *mut Struct13;
    let mut ppcVar3: *mut *mut c_void;
    let mut pHVar4: *mut HDC16;
    let mut puVar5: *mut u32;
    let mut puVar6: *mut u8;
    let mut dx_var1: *mut u8;
    let mut iVar5: *mut Struct40;
    let mut unaff_DI: i16;
    let mut uVar7: u16;
    let mut unaff_CS: u16;
    let mut puVar8: *mut u16;
    let mut local_8: HDC16;
    let mut pu_stack6: *mut u16;

    get_sys_metrics_1020_7c1a(param_1, param_2, unaff_CS);
    uVar7              = (param_1 >> 0x10);
    iVar5              = param_1;
    &iVar5.field_0x14 = 0x0;
    iVar5.field_0x18  = 0x0;
    puVar8             = pass1_1008_3e38((param_1 & 0xffff0000 | &iVar5.field_0x1e));
    param_1            = addr_table_1020_1730;//0x1730;
    iVar5.field_0x2   = SEG_1020;
    puVar8             = mixed_1010_20ba(globals.data_1050_0ed0, 0x2d, param_3, (puVar8 >> 0x10), unaff_DI);
    puVar6             = (puVar8 >> 0x10);
    iVar5.field_0x14  = puVar8;
    &iVar5.field_0x16 = puVar6;
    pu_stack6           = mixed_1010_20ba(globals.data_1050_0ed0, 0x29, param_3, puVar6, unaff_DI);
    uVar1              = &iVar5.field_0x14;
    ppcVar3            = (**(u32 **)&iVar5.field_0x14 + 0x4);
    (**ppcVar3)(SEG_1010, uVar1, (uVar1 >> 0x10), 0x0, param_1);
    local_8                  = GetDC16(SEG_1010);
    uVar1                    = &iVar5.field_0x14;
    (uVar1 + 0x7c) = local_8;
    uVar1                    = &iVar5.field_0x14;
    puVar5                   = *(uVar1 + 0x66);
    iVar5.field_0x18        = puVar5;
    ppcVar3                  = (*iVar5.field_0x18 + 0x14);
    (**ppcVar3)();
    p_var2 = (pu_stack6 + 0xe);
    puVar6 = dx_var1;
    pass1_1008_4d84((puVar5 & 0xffff | ZEXT24(dx_var1) << 0x10), p_var2, dx_var1);
    pHVar4            = (HDC16 *)palette_op_1008_4e08(p_var2, &local_8, puVar6, SEG_1008);
    iVar5.field_0x1c_addr_base = pHVar4;
    return;
}


pub fn win_ui_op_1020_150e(param_1: *mut u16, param_2: HDC16)

{
    let mut HVar1: HPALETTE16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut ss_var1: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    param_1.field_0x0 = addr_table_1020_1730;//0x1730;
    (iVar2 + 0x2) = SEG_1020;
    if ((iVar2 + 0x14) != 0x0) {
        param_2 = SEG_1010;
        pass1_1010_1ea6(*(iVar2 + 0x14), param_1 & 0xffff | uVar3 << 0x10, ss_var1);
    }
    HVar1 = SelectPalette16(param_2, 0x0,  (iVar2 + 0x1c));
     (iVar2 + 0x1c) = HVar1;
    DeleteObject16( LAST_SEGMENT);
    param_1.field_0x0 = addr_table_1008_3aa0[4]; // 0x3ab0;
    (iVar2 + 0x2) = SEG_1008;
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a
    (iVar2 + 0x2) = SEG_1008;
    return;
}

Struct18 *pass1_1020_170a(param_1: *mut Struct18, param_2: u8, param_3: u16)

{
    win_ui_op_1020_150e(&param_1.field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, SEG_1000);
    }
    return param_1;
}

pub fn pass1_1020_1780(u32 *param_1)

{
    let mut address_table: *mut *mut c_void;

    address_table = (param_1 + 0x6c);
    (**address_table)();
    destroy_win_1040_8212(param_1, SEG_1040);
    return;
}


pub fn mixed_ui_op_1020_179c(param_1: *mut Struct1)

{
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u32;
    let mut u_var4: u16;
    let mut IVar5: u16;
    let mut puVar6: *mut u8;
    let mut in_DX: *mut u8;
    let mut dx_var1: *mut u8;
    let mut puVar7: *mut u8;
    let mut uVar8: u16;
    let mut iVar9: i16;
    let mut iVar10: i16;
    let mut unaff_DI: i16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    WNDCLASS16 *ss_var1;
    let mut puVar14: *mut u16;
    WNDCLASS16 *in_resc_id_3;
    WNDCLASS16 *in_buffer_4;
    WNDCLASS16  local_178[0xc];
    let mut uStack118: u32;
    let mut uStack114: u32;
    let mut local_6e: RECT16;
    let mut uStack106: u32;
    let mut HStack102: HWND16;
    let mut iStack98: i16;
    let mut iStack94: i16;
    let mut uStack78: u16;
    let mut puStack76: *mut u8;
    let mut uStack74: u32;
    let mut HStack70: HWND16;
    let mut u_stack68: u32;
    let mut u_stack64: u32;
    LPVOID      pvStack60;
    let mut uStack58: u16;
    let mut uStack56: u16;
    let mut pUStack54: *mut u32;
    let mut uStack50: u32;
    let mut local_2e: [u8;12] = [0;12];
    let mut local_1c: RECT16;
    let mut uStack24: u32;
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut puStack16: *mut u16;
    let mut pIStack12: *mut u16;
    let mut uStack8: u16;
    let mut pu_stack6: *mut u16;

    dialog_ui_fn_1040_78e2(NULL, param_1, SEG_1040);
    u_var4    = 0x89;
    pu_stack6 = mixed_1010_20ba(globals.data_1050_0ed0, 0x9, ss_var1, in_DX, unaff_DI);
    puVar7   = (pu_stack6 >> 0x10);
    u_var4    = pass1_1010_65d0(ss_var1, pu_stack6, u_var4);
    uStack8  = (u_var4 == 0x0);
    u_var4    = pass1_1010_65d0(ss_var1, pu_stack6, 0x86);
    if(u_var4 != 0x0)
    {
        uStack8 = 0x0;
    }
    puVar14         = mixed_1010_20ba(globals.data_1050_0ed0, 0x39, ss_var1, puVar7, unaff_DI);
    uVar12          = (puVar14 >> 0x10);
    uVar8           = puVar14;
    uVar11          = (param_1 >> 0x10);
    iVar9           = param_1;
    *(iVar9 + 0x8e) = uVar8;
    (iVar9 + 0x90)  = uVar12;
    ppcVar2         = ((iVar9 + 0x8e) + 0x10);
    (**ppcVar2)(SEG_1010, (iVar9 + 0x8e), uVar12, uStack8);
    puStack76 = dx_var1;
    mem_op_1000_179c(0x12, dx_var1, 0);
    puVar7 = (puStack76 | uVar8);
    uStack78 = uVar8;
    if(puVar7 == 0x0)
    {
        (iVar9 + 0x92) = 0x0;
    }
    else
    {
        pass1_1020_1eea(
          str_var1(puStack76, uVar8), param_1, (iVar9 + 0x6), puVar7, unaff_DI, ss_var1);
        *(iVar9 + 0x92) = uVar8;
        (iVar9 + 0x94)  = puVar7;
    }
    uVar1     = *(iVar9 + 0x8e);
    pIStack12 = (uVar1 & 0xffff0000 | (uVar1 + 0xa));
    puStack16 = mixed_1010_20ba(globals.data_1050_0ed0, 0x48, ss_var1, puVar7, unaff_DI);
    GetClientRect16(SEG_1010, &local_1c);
    IVar5          = GetSystemMetrics16(LAST_SEGMENT);
    uVar12         = (pIStack12 >> 0x10);
    iVar10         = pIStack12;
    (iVar10 + 0x6) = IVar5 + uStack24;
    uVar13         = (puStack16 >> 0x10);
    iStack18       = (puStack16 + 0xa);
    iStack20       = (puStack16 + 0xc);
    (iVar10 + 0x2) = (iStack20 - (iVar10 + 0x6)) / 0x2;
    iVar10         = iStack18 - (iVar10 + 0x4);
    uVar8          = iVar10 >> 0xf;
    *pIStack12     = iVar10 / 0x2;
    pass1_1028_dc52(str_var1(ss_var1, local_2e), 0x1, 0x0, 0x100);
    uStack56 = 0x0;
    while(true)
    {
        puVar6 = local_2e;
        pass1_1028_e4ec(str_var1(ss_var1, puVar6));
        uStack50 = str_var1(uVar8, puVar6);
        uStack58 = uVar8 | puVar6;
        if(uStack58 == 0x0)
            break;
        pUStack54 = (puVar6 + 0x10);
        uVar8     = uStack58;
        if(pUStack54 != 0x0)
        {
            pass1_1000_3cea(param_1 & 0xffff0000 | (iVar9 + 0x10), *pUStack54);
            uVar8 = uStack58;
        }
    }
    u_var4          = pass1_1020_1da8(param_1, puVar6, 0x0, ss_var1);
    (iVar9 + 0x96) = u_var4;
    u_var4          = pass1_1010_65d0(ss_var1, pu_stack6, 0x86);
    if((u_var4 == 0x0) || ((iVar9 + 0x96) != 0x0))
    {
        uVar3          = (iVar9 + 0x8e);
        (uVar3 + 0x2c) = 0x0;
        HStack102      = GetDlgItem16(SEG_1010, 0x175);
        if(uStack8 != 0x0)
        {
            load_string_1010_84e0(SEG_1010, globals.dat_1050_14cc, globals.dat_1050_14cc >> 0x10,NULL), 0x100, local_178, (short)ss_var1);
            SetWindowText16(SEG_1010, (SEGPTR)local_178);
        }
        pvStack60 = MakeProcInstance16(LAST_SEGMENT, globals.hinst_1050_038c);
        GetWindowRect16(LAST_SEGMENT, &local_6e);
        uStack114       = uStack106;
        iStack98        = uStack106 - local_6e.x;
        iStack94        = uStack106 - local_6e.y;
        uStack118       = local_6e & 0xffff0000 | (-(iStack98 - (pIStack12 + 0x4)) / 0x2);
        IVar5           = GetSystemMetrics16(LAST_SEGMENT);
        uVar1           = uStack118 & 0xffff;
        uStack118       = uVar1 | (uStack118 - IVar5) << 0x10;
        uStack118 = uVar1;
        MoveWindow16(LAST_SEGMENT, 0x0, iStack94, iStack98, uStack118 - IVar5, uStack118);
    }
    else
    {
        win_1008_5c7c(NULL, _PTR_LOOP_1050_02a0, 0x9d0001, ss_var1, u_var4);
        (iVar9 + 0x8c) = u_var4;
        pvStack60      = MakeProcInstance16(SEG_1008, globals.hinst_1050_038c);
    }
    EnumChildWindows1(LAST_SEGMENT, 0x0, ZEXT24(pvStack60) << 0x10);
    FreeProcInstance16(LAST_SEGMENT);
    HStack70 = GetDlgItem16(LAST_SEGMENT, 0x1);
    GetWindowRect16(LAST_SEGMENT, &local_1c);
    u_stack64   = uStack24;
    local_1c.x = uStack24 - local_1c.x;
    uStack74   = str_var1(local_1c.x, uStack24 - local_1c.y);
    u_stack68   = local_1c & 0xffff0000 | (-(local_1c.x - (pIStack12 + 0x4)) / 0x2);
    IVar5      = GetSystemMetrics16(LAST_SEGMENT);
    u_stack68   = u_stack68 & 0xffff | (u_stack68 - IVar5) << 0x10;
    if((iVar9 + 0x96) == 0x0)
    {
        if(uStack8 == 0x0)
            //goto LAB_1020_1b24;
        in_buffer_4  = local_178;
        in_resc_id_3 = (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21);
    }
    else
    {
        load_string_1010_84e0(SEG_1010, globals.dat_1050_14cc, globals.dat_1050_14cc >> 0x10,NULL), 0x100, local_178, (short)ss_var1);
        GetDlgItem16(SEG_1010, 0x175);
        SetWindowText16(LAST_SEGMENT, (SEGPTR)local_178);
        in_resc_id_3 = local_178;
        in_buffer_4  = ss_var1;
        ss_var1     = 0x3fe;
    }
    load_string_1010_84e0(SEG_1010, globals.dat_1050_14cc, globals.dat_1050_14cc >> 0x10,NULL), in_resc_id_3, in_buffer_4, (short)ss_var1);
    SetWindowText16(SEG_1010, (SEGPTR)local_178);
// LAB_1020_1b24:
    MoveWindow16(LAST_SEGMENT, 0x0, uStack74, (uStack74 >> 0x10), u_stack68, u_stack68);
    uVar12 = (pIStack12 >> 0x10);
    iVar9  = pIStack12;
    SetWindowPos16(LAST_SEGMENT, 0x44, (iVar9 + 0x6), (iVar9 + 0x4), (iVar9 + 0x2), *pIStack12, 0x0);
    return;
}

pub fn pass1_1018_5e5a(param_1: *mut u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = addr_table_1018_6128;//0x6128;
    param_1.field_0x2 = SEG_1018;
    pass1_1038_b6e0(globals.ptr_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c( param_1, SEG_1040);
}

pub fn win_ui_op_1018_5e9a(param_1: *mut Struct1, param_2: u16)

{
    let mut uVar1: u32;
    let mut pu_var2: *mut u32;
    let mut IVar3: u16;
    let mut pu_var4: *mut u8;
    let mut in_DX: *mut u8;
    let mut puVar5: *mut u8;
    let mut puVar6: *mut u8;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut iVar9: i16;
    let mut unaff_DI: i16;
    let mut uVar10: u16;
    let mut puVar11: *mut u16;
    let mut local_28: [u8;12] = [0;12];
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut local_e: RECT16;
    let mut iStack8: i16;
    let mut pIStack6: *mut u16;

    dialog_ui_fn_1040_78e2(NULL, param_1, SEG_1040);
    puVar11         = mixed_1010_20ba(globals.data_1050_0ed0, 0x39, param_2, in_DX, unaff_DI);
    puVar5          = (puVar11 >> 0x10);
    uVar7           = puVar11;
    uVar10          = (param_1 >> 0x10);
    iVar9           = param_1;
    *(iVar9 + 0x8e) = uVar7;
    (iVar9 + 0x90) = puVar5;
    mem_op_1000_179c(0x12, puVar5, 0);
    puVar6 = (puVar5 | uVar7);
    if(puVar6 == 0x0)
    {
        (iVar9 + 0x92) = 0x0;
    }
    else
    {
        pass1_1018_6198(
          str_var1(puVar5, uVar7), param_1, (iVar9 + 0x6), puVar6, unaff_DI, param_2);
        *(iVar9 + 0x92) = uVar7;
        (iVar9 + 0x94)  = puVar6;
    }
    uVar1    = *(iVar9 + 0x8e);
    pIStack6 = (uVar1 & 0xffff0000 | (uVar1 + 0xa));
    GetClientRect16(SEG_1000, &local_e);
    IVar3            = GetSystemMetrics16(LAST_SEGMENT);
    (pIStack6 + 0x6) = IVar3 + iStack8;
    puVar11          = mixed_1010_20ba(globals.data_1050_0ed0, 0x48, param_2, puVar6, unaff_DI);
    uStack20         = (puVar11 >> 0x10);
    iStack22         = puVar11;
    iStack16         = (iStack22 + 0xa);
    iStack18         = (iStack22 + 0xc);
    uVar10           = (pIStack6 >> 0x10);
    (pIStack6 + 0x2) = (iStack18 - (pIStack6 + 0x6)) / 0x2;
    uVar7            = iStack16 >> 0xf;
    *pIStack6        = iStack16 / 0x2 + 0x3;
    pass1_1028_dc52(str_var1(param_2, local_28), 0x1, 0x0, 0x100);
    while(true)
    {
        pu_var4 = local_28;
        pass1_1028_e4ec(str_var1(param_2, pu_var4));
        uVar8 = uVar7 | pu_var4;
        if(uVar8 == 0x0)
            break;
        pu_var2 = (pu_var4 + 0x10);
        uVar7  = uVar8;
        if(pu_var2 != 0x0)
        {
            pass1_1000_3cea(param_1 & 0xffff0000 | (iVar9 + 0x10), *pu_var2);
            uVar7 = uVar8;
        }
    }
    uVar10 = (pIStack6 >> 0x10);
    iVar9  = pIStack6;
    SetWindowPos16(SEG_1028, 0x44, (iVar9 + 0x6), (iVar9 + 0x4), (iVar9 + 0x2), *pIStack6, 0x0);
    return;
}

pub fn mix_ui_op_1018_6adc(param_1: *mut Struct28)

{
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut in_DX: *mut u8;
    let mut u_var4: u16;
    let mut iVar5: i16;
    let mut unaff_DI: i16;
    let mut uVar6: u16;
    WNDCLASS16 *ss_var1;
    let mut puVar7: *mut u16;
    let mut paVar8: *mut Struct43;

    puVar7 = mixed_1010_20ba(globals.data_1050_0ed0, 0x48, ss_var1, in_DX, unaff_DI);
    u_var4  = (puVar7 >> 0x10);
    iVar1  = (puVar7 + 0xa);
    iVar2  = (puVar7 + 0xc);
    uVar6  = (param_1 >> 0x10);
    iVar5  = param_1;
    if(0x1 < iVar1 - (iVar5 + 0xe6))
    {
        u_var4          = iVar1 >> 0xf;
        (iVar5 + 0xe2) = iVar1 / 0x2 - ((iVar5 + 0xe6) + 0x1) / 0x2;
    }
    if(0x1 < iVar2 - (iVar5 + 0xe8))
    {
        u_var4          = iVar2 >> 0xf;
        (iVar5 + 0xe4) = iVar2 / 0x2 - ((iVar5 + 0xe8) + 0x1) / 0x2;
    }
    uVar3 = ShowCursor16(SEG_1010);
    if((iVar5 + 0xee) != 0x0)
    {
        win_1008_5c5c(ss_var1, uVar3, u_var4, globals._PTR_LOOP_1050_02a0, (iVar5 + 0xee));
        (iVar5 + 0xf0) = uVar3;
    }
    paVar8 = unk_io_op_1010_830a(globals.dat_1050_14cc, (iVar5 + 0xec), ss_var1);
    mci_send_command_1008_53ae(paVar8, (iVar5 + 0x8), SEG_1008, ss_var1);
    ShowCursor16(SEG_1008);
    unk_destroy_window_op_1018_6bb6(param_1, LAST_SEGMENT);
    return;
}

Struct11 *pass1_1018_4ae0(param_1: *mut Struct11, param_2: u8, param_3: u16)

{
    clenaup_win_ui_1018_4d22(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, SEG_1000);
    }
    return param_1;
}

pub fn unk_win_ui_op_1018_4f18(param_1: *mut Struct39, param_2: u16, param_3: u32)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut pu_var2: *mut u32;
    let mut rect: *mut RECT16;
    let mut iVar3: i16;
    let mut u_var4: u32;
    let mut dx_var1: *mut u8;
    let mut puVar5: *mut u8;
    let mut dx_var1_00: *mut u8;
    let mut puVar6: *mut u8;
    let mut uVar7: u16;
    let mut iVar6: *mut Struct39;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut ss_var1: u16;
    let mut paStack22: *mut Struct76;
    let mut local_12: RECT16;
    let mut iStack14: i16;
    let mut iStack12: i16;
    let mut uStack10: u32;
    let mut paStack6: *mut Struct43;

    paStack6 = unk_io_op_1010_830a(globals.dat_1050_14cc, param_2, ss_var1);
    u_var4    = paStack6 & 0xffff;
    ppcVar1  = (paStack6 + 0x14);
    (**ppcVar1)(SEG_1010, u_var4, (paStack6 >> 0x10));
    pu_var2   = u_var4;
    uStack10 = u_var4 & 0xffff | ZEXT24(dx_var1) << 0x10;
    uVar8    = (param_1 >> 0x10);
    iVar6    = param_1;
    puVar5   = dx_var1;
    if(&iVar6.field_0xe != 0x0)
    {
        uVar7  = iVar6.field_0x10;
        pu_var2 = &iVar6.field_0xe;
        puVar5 = (uVar7 | pu_var2);
        if(puVar5 != 0x0)
        {
            ppcVar1 = *pu_var2;
            (**ppcVar1)();
            puVar5 = dx_var1_00;
        }
    }
    mem_op_1000_179c(0x14, puVar5, 0);
    puVar6 = (puVar5 | pu_var2);
    if(puVar6 == 0x0)
    {
        pu_var2 = 0x0;
        puVar6 = 0x0;
    }
    else
    {
        struct_1008_4c58(str_var1(puVar5, pu_var2));
    }
    iVar6.field_0xe  = pu_var2;
    iVar6.field_0x10 = puVar6;
    pass1_1008_4d84(&iVar6.field_0xe, uStack10, puVar6);
    rect = &local_12;
    GetClientRect16(SEG_1008, rect);
    uVar9 = SEG_1000;
    mem_op_1000_179c(0x1e, puVar6, 0);
    paStack22 = str_var1(puVar6, rect);
    uVar7     = puVar6 | rect;
    if(uVar7 == 0x0)
    {
        &iVar6.field_0xa = 0x0;
    }
    else
    {
        iVar3 = (iStack12 - local_12.y) + 0x1;
        uVar9 = SEG_1008;
        pass1_1008_405c(paStack22, *&iVar6.field_0xe, iVar3, (iStack14 - local_12.x) + 0x1);
        iVar6.field_0xa = iVar3;
        iVar6.field_0xc = uVar7;
    }
    if(paStack6 != 0x0)
    {
        ppcVar1 = paStack6;
        (**ppcVar1)(uVar9, paStack6, (paStack6 >> 0x10), 0x1);
    }
    return;
}


Struct11 *pass1_1018_5032(param_1: *mut Struct11, param_2: u8, param_3: u16)

{
    clenaup_win_ui_1018_4d22(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, SEG_1000);
    }
    return param_1;
}

pub fn pass1_1018_57e6(param_1: u32, long param_2, param_3: u16)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    send_dlg_item_msg_1040_d20c(*(param_1 + 0xa), param_2, SEG_1040, param_3);
    (param_1 + 0xa) = 0x0;
    return;
}

pub fn pt_in_rect_1018_1bda(param_1: u32, param_2: u16, param_3: u16, param_4: u16)

{
    let mut pi_var1: *mut i16;
    let mut u_var2: u16;
    let mut iVar3: i16;
    let mut BVar4: BOOL16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut iStack26: i16;
    let mut PStack24: POINT16;
    let mut local_14: i16;
    let mut local_12: i16;
    let mut uStack16: u16;
    let mut uStack14: u32;
    let mut local_a: i16;
    let mut local_8: i16;
    let mut iStack6: i16;
    let mut iStack4: i16;

    uStack14 = 0x0;
    iVar3    = param_1;
    pass1_1008_3e94((param_1 & 0xffff0000 | (iVar3 + 0x3a)),
                    str_var1(param_4, &local_14),
                    str_var1(param_4, &local_12));
    PStack24 = str_var1(param_2, param_3);
    uStack16 = 0x0;
    iStack26 = 0x0;
    while(true)
    {
        uVar6  = (param_1 >> 0x10);
        pi_var1 = (iVar3 + 0x44);
        if(*pi_var1 == iStack26 || *pi_var1 < iStack26)
        {
            return;
        }
        u_var2    = (iVar3 + 0x42);
        iVar5    = (iVar3 + 0x40) + iStack26 * 0x18;
        uStack14 = str_var1(u_var2, iVar5);
        pass1_1008_3e94(str_var1(u_var2, iVar5),
                        str_var1(param_4, &local_8),
                        str_var1(param_4, &local_a));
        local_a = local_a + local_12 + -0x6;
        iStack6 = local_a + 0xc;
        local_8 = local_8 + local_14 + -0x6;
        iStack4 = local_8 + 0xc;
        BVar4   = PtInRect16(SEG_1008, PStack24);
        if(BVar4 != 0x0)
            break;
        iStack26 = iStack26 + 0x1;
    }
    pass1_1018_1eda(param_1, uStack14, param_4);
    return;
}
pub fn pass1_1018_2440(param_1: *mut Struct11, param_2: u16)

{
    let mut pu32_var1: *mut u32;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut piVar4: *mut i16;
    let mut uVar6: u16;
    let mut uVar5: *mut Struct502;
    let mut uVar7: u16;
    let mut pu_stack6: *mut u16;

    uVar7             = (param_1 >> 0x10);
    uVar5             = param_1;
    param_1           = addr_table_1018_2ada;//0x2ada;
    uVar5.field_0x2  = SEG_1018;
    uVar5.field_0x1c_addr_base = addr_table_1018_2ada[6];//0x2af2;  // s_fem132_wav_1050_2aec + 0x6;
    uVar5.field_0x1e = SEG_1018;
    if(globals._PTR_LOOP_1050_0388 != 0x0)
    {
        if(param_1 == 0x0)
        {
            piVar4 = 0x0;
            uVar6  = 0x0;
        }
        else
        {
            piVar4 = &uVar5.field_0x1c_addr_base;
            uVar6  = uVar7;
        }
        param_2 = SEG_1008;
        pass1_1008_92b2(globals._PTR_LOOP_1050_0388, 0x73, str_var1(uVar6, piVar4));
    }
    pu32_var1 = uVar5.field_0x2a;
    u_var2  = uVar5.field_0x2c;
    if((u_var2 | pu32_var1) != 0x0)
    {
        ppcVar3 = *pu32_var1;
        (**ppcVar3)(param_2, p_var1: u32, u_var2, 0x1);
    }
    pu32_var1 = uVar5.field_0x6e;
    u_var2  = uVar5.field_0x70;
    if((u_var2 | pu32_var1) != 0x0)
    {
        ppcVar3 = *pu32_var1;
        (**ppcVar3)(param_2, p_var1: u32, u_var2, 0x1);
    }
    if(param_1 == 0x0)
    {
        piVar4 = 0x0;
        uVar7  = 0x0;
    }
    else
    {
        piVar4 = &uVar5.field_0x1c_addr_base;
    }
    pu_stack6    = str_var1(uVar7, piVar4);
    *pu_stack6   = addr_table_1008_380a[36]; // 0x389a
    piVar4[0x1] = SEG_1008;
    clenaup_win_ui_1018_4d22(param_1, param_2);
    return;
}

pub fn msg_box_op_1010_8bb4(param_1: *mut c_void, param_2: u16, param_3: u32, param_4: HINSTANCE16, param_5: u16)

{
    let mut pcVar1: *mut c_char;
    let mut local_402: [u8;400] = [0;400];

    pcVar1 = load_string_1010_847eglobals.dat_1050_14cc, param_4);
    unk_str_op_1000_3d3e(str_var1(param_5, local_402), pcVar1);
    pass1_1000_3cea(str_var1(param_5, local_402), param_3);
    pcVar1 = load_string_1010_847eglobals.dat_1050_14cc, SEG_1000);
    MessageBox16(SEG_1000, SEG_1010, pcVar1, (pcVar1 >> 0x10));
    PostMessage16(LAST_SEGMENT, 0x0, 0x0, 0x11100ee);
    return;
}

pub fn ui_op_1010_79aa(param_1: u32, param_2: i16, long param_3, param_4: u16)

{
    let mut uVar1: u32;
    let mut pu_var2: *mut u8;
    let mut dx_var1: u16;
    let mut uVar3: u16;
    let mut lStack18 = 0i32;
    let mut lStack14 = 0i32;
    let mut local_a: [u8;8] = [0;8];

    uVar3 = (param_1 >> 0x10);
    if(((param_1 + 0x1c) != 0x0) && ((param_3 != 0x0 || (param_2 != 0x0))))
    {
        pass1_1008_5784(str_var1(param_4, local_a), *(param_1 + 0x1c));
        lStack18 = 0x0;
        do
        {
            pu_var2 = local_a;
            pass1_1008_5b12(pu_var2, param_4);
            lStack14 = str_var1(dx_var1, pu_var2);
            if((dx_var1 | pu_var2) == 0x0)
                //goto LAB_1010_7a49;
            if(((param_2 == 0x0) && ((pu_var2 + 0x4) == param_3)) || ((param_3 == 0x0 && (uVar1 = (pu_var2 + 0x8), (uVar1 + 0xa) == param_2))))
                break;
        } while(((pu_var2 + 0x4) != param_3) || (uVar1 = (pu_var2 + 0x8), (uVar1 + 0xa) != param_2));
        lStack18 = lStack14;
    // LAB_1010_7a49:
        if(lStack18 != 0x0)
        {
            SetFocus16(SEG_1008);
            BringWindowToTop16(LAST_SEGMENT);
            return;
        }
    }
    return;
}

pub fn show_win_1010_7a76(param_1: u32, param_2: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u16;
    let mut ss_var1: u16;
    let mut lVar3 = 0i32;
    let mut local_a: [u8;8] = [0;8];

    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if((iVar1 + 0x20) == 0x0)
    {
        (iVar1 + 0x20) = 0x1;
        pass1_1008_5784(str_var1(ss_var1, local_a), *(iVar1 + 0x1c));
        while(true)
        {
            lVar3 = pass1_1008_5b12(local_a, ss_var1);
            if(lVar3 == 0x0)
                break;
            ShowWindow16(SEG_1008, 0x0);
        }
    }
    return;
}

pub fn show_window_1010_7ace(param_1: u32, param_2: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u16;
    let mut lVar3 = 0i32;
    let mut local_a: [u8;8] = [0;8];

    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if((iVar1 + 0x20) != 0x0)
    {
        (iVar1 + 0x20) = 0x0;
        pass1_1008_5784(str_var1(param_2, local_a), *(iVar1 + 0x1c));
        while(true)
        {
            lVar3 = pass1_1008_5b12(local_a, param_2);
            if(lVar3 == 0x0)
                break;
            ShowWindow16(SEG_1008, 0x1);
        }
    }
    return;
}


u32 destroy_window_1010_7b26(param_1: u32, long param_2, param_3: u16, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u8;
    let mut dx_var1: u16;
    let mut iVar2: i16;
    let mut u_var4: u16;
    let mut local_a: [u8;8] = [0;8];

    u_var4 = (param_1 >> 0x10);
    iVar2 = param_1;
    uVar1 = *(iVar2 + 0x1e) | *(iVar2 + 0x1c);
    if(uVar1 != 0x0)
    {
        pass1_1008_5784(str_var1(param_3, local_a), *(iVar2 + 0x1c));
        do
        {
            pu_var2 = local_a;
            pass1_1008_5b12(pu_var2, param_3);
            param_4 = dx_var1 | pu_var2;
            if(param_4 == 0x0)
                break;
        } while((pu_var2 + 0x4) != param_2);
        uVar1 = dx_var1 | pu_var2;
        if(uVar1 != 0x0)
        {
            uVar1 = DestroyWindow16(SEG_1008);
        }
    }
    return str_var1(uVar1, param_4);
}

pub fn pass1_1010_8096(param_1: *mut u32, param_2: i16)

{
    let mut uVar1: u16;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut ss_var1: u16;
    let mut pcVar5: *mut c_char;
    let mut puVar6: *mut u16;
    let mut local_306: [u8;100] = [0;100];
    let mut local_206: [u8;100] = [0;100];
    let mut local_106: [u8;104] = [0;104];

    u_var4 = (param_1 >> 0x10);
    uVar3 = param_1;
    str_1000_4d58(((uVar3 + 0xe82) * 0x4 + 0x2526), 0x0, 0x0,
                  str_var1(ss_var1, local_206),
                  str_var1(ss_var1, local_306));
    unk_str_op_1000_3d3e(str_var1(ss_var1, local_106), str_var1(ss_var1, local_206));
    if(param_2 == 0x2)
    {
        puVar6 = &USHORT_1050_3194;
    }
    else
    {
        puVar6 = &USHORT_1050_3196;
    }
    pass1_1000_3cea(str_var1(ss_var1, local_106), puVar6);
    pass1_1000_3cea(str_var1(ss_var1, local_106), str_var1(ss_var1, local_306));
    pcVar5 = set_err_mode_1010_8b14(param_1, str_var1(ss_var1, local_106), ss_var1);
    u_var2  = (pcVar5 >> 0x10);
    if((pcVar5 == local_106) && (u_var2 == ss_var1))
    {
        msg_box_op_1010_8bb4(uVar3, u_var4, pcVar5 & 0xffff | u_var2 << 0x10, SEG_1000, ss_var1);
    }
    fn_ptr_1000_17ce(*param_1, SEG_1000);
    uVar1         = str_op_1008_60e8(pcVar5);
    param_1       = uVar1;
    (uVar3 + 0x2) = u_var2;
    return;
}

Struct43 *unk_io_op_1010_830a(param_1: u32, param_2: u16, param_3: u16)

{
    let mut in_AX: u16;
    let mut pu32_var1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut in_DX: *mut u8;
    let mut uVar3: u16;
    let mut iVar3: *mut Struct45;
    let mut iVar2: *mut Struct44;
    let mut iVar4: i16;
    let mut unaff_CS: HINSTANCE16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut local_2e: u32;
    let mut uStack10: u32;
    let mut paStack6: *mut Struct43;

    paStack6 = 0x0;
    iVar3    = (param_2 * 0x10);
    uVar5    = param_1;
    uVar6    = (param_1 >> 0x10);
    if(iVar3.field_0x10 == 0x1)
    {
        uStack10       = set_err_mode_1010_8b14(param_1, *&iVar3.field_0x12, param_3);
        uStack10 = (uStack10 >> 0x10);
        if((iVar3.field_0x12 == uStack10) && (iVar3.field_0x14 == uStack10))
        {
            msg_box_op_1010_8bb4(uVar5, uVar6, uStack10, unaff_CS, param_3);
            return 0x0;
        }
        pu32_var1 = &local_2e;
        struct_op_1008_48fe(str_var1(param_3, pu32_var1), 0x1, uStack10, uStack10);
        mem_op_1000_179c(0x1e, (uStack10 >> 0x10), 0);
        uVar3 = (uStack10 >> 0x10) | pu32_var1;
        if(uVar3 == 0x0)
        {
            pu_var2 = 0x0;
            uVar3  = 0x0;
        }
        else
        {
            pu_var2 = &local_2e;
            struct_op_1008_3f92((uStack10 & 0xffff0000 | ZEXT24(pu32_var1)), str_var1(param_3, pu_var2));
        }
        paStack6 = str_var1(uVar3, pu_var2);
        close_file_1008_496c(&local_2e, param_3);
        local_2e = paStack6;
    }
    else
    {
        if((param_2 * 0x10 + 0x10) == 0x2)
        {
            pass1_1010_878c((Struct87 **)param_1, (param_2 * 0x10 + 0x16), unaff_CS);
            if((uVar5 + 0x67c) == 0x0)
            {
                return 0x0;
            }
            iVar2 = (param_2 * 0x10);
            pass1_1008_6562((uVar5 + 0x67c),
                            str_var1(iVar2.field_0x1c_addr_base, iVar2.field_0x1e), iVar2.field_0x1a_addr_offset, in_AX, in_DX);
            local_2e = str_var1(in_DX, in_AX);
        }
        else
        {
            iVar4 = param_2 * 0x10;
            if((iVar4 + 0x10) == 0x3)
            {
                local_2e = set_err_mode_1010_8b14(param_1, *(iVar4 + 0x12), param_3);
                if(((iVar4 + 0x12) == local_2e) && ((iVar4 + 0x14) == (local_2e >> 0x10)))
                {
                    msg_box_op_1010_8bb4(uVar5, uVar6, local_2e, unaff_CS, param_3);
                    local_2e = local_2e;
                }
            }
            else
            {
                local_2e = paStack6;
                if((param_2 * 0x10 + 0x10) == 0x4)
                {
                    local_2e = set_err_mode_1010_8b14(param_1, *(param_2 * 0x10 + 0x12), param_3);
                }
            }
        }
    }
    paStack6 = local_2e;
    return paStack6;
}

pub fn pass1_1010_71d6(param_1: u32, param_2: i16,param_3: *mut u16, param_4: u16, param_5: u16, param_6: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar9: u32;
    let mut uStack20: u16;
    let mut uStack14: u16;
    let mut u_stack6: u32;

    uVar8 = (param_1 >> 0x10);
    u_var2 = (param_1 + 0x14);
    pass1_1010_ad22(u_var2, param_5, param_6, (u_var2 >> 0x10), *param_3);
    u_stack6 = str_var1(param_5, param_4);
    if((param_5 | param_4) == 0x0)
    {
        return;
    }
    uVar9 = struct_op_1030_73a8(str_var1(param_5, param_4));
    uVar7 = (uVar9 >> 0x10);
    uVar3 = uVar9;
    if(((uVar7 | uVar3) != 0x0) && ((uVar3 + 0x1c) == 0x8000002))
    {
        return;
    }
    u_var2    = (param_4 + 0x2e);
    uStack14 = u_var2;
    if(((*(param_4 + 0x30) | uStack14) != 0x0) && ((uStack14 + 0x200) == 0x8000002))
    {
        return;
    }
    u_var2 = (param_1 + 0x14);
    uVar5 = pass1_1010_b028(u_var2, (u_var2 >> 0x10), uVar9);
    iVar1 = (uVar3 + 0x12);
    iVar4 = iVar1;
    if((iVar1 != 0x4) && (iVar4 = param_2, iVar1 == 0x7))
    {
        param_2 = 0x5;
        iVar4   = param_2;
    }
    param_2 = iVar4;
    uVar6   = param_2 - 0x2;
    if(uVar6 != 0x0)
    {
        if(param_2 == 0x3)
        {
            uVar6 = uVar5 - 0xb;
            if((uVar6 == 0x0) || (uVar6 = uVar5 - 0x37, uVar6 == 0x0))
            {
                uStack20 = 0xb;
            }
            else
            {
                uStack20 = 0xa;
            }
            //goto LAB_1010_72a7;
        }
        uVar6 = param_2 - 0x4;
        if(uVar6 == 0x0)
        {
            uStack20 = 0x17;
            //goto LAB_1010_72a7;
        }
        uVar6 = param_2 - 0x5;
        if(uVar6 != 0x0)
        {
            uVar6    = pass1_1010_7818(param_1, uVar9);
            uStack20 = uVar6;
            //goto LAB_1010_72a7;
        }
    }
    uStack20 = 0xc;
// LAB_1010_72a7:
    if(uStack20 == 0x0)
    {
        return;
    }
    ui_op_1010_79aa(param_1, 0x0, u_stack6, param_6);
    if(uVar6 == 0x0)
    {
        unk_win_op_1010_7300(param_1, 0x0, uStack20, u_stack6);
    }
    return;
}

Struct11 *pass1_1010_5074(param_1: *mut Struct11, param_2: u8)

{
    clenaup_win_ui_1018_4d22(param_1, SEG_1018);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, SEG_1000);
    }
    return param_1;
}

pub fn pass1_1010_29c6(param_1: *mut Struct11) {
    u32 * pu32_var1;
    let mut u_var2: u16;
    let mut ppv_var3: *mut *mut c_void;
//    Struct476 *iVar5;
//    u16          u_var4;
//    u_var4            = (param_1 >> 0x10);
//    iVar5            = param_1;

    param_1 = addr_table_1010_2be4;//0x2bec;//s_add16_wav_1050_2bdc + 0x8;
    iVar5->fld2_segment = SEG_1010;
    if (&iVar5.field_0x1c_addr_base != 0x0) {
        pu32_var1 = *&iVar5.field_0x1c_addr_base;
        u_var2 = iVar5.field_0x1e;
        if ((u_var2 | pu32_var1) != 0x0) {
            ppv_var3 = *pu32_var1;
            (**ppv_var3)();
        }
        &iVar5.field_0x1c_addr_base = 0x0;
        fn_ptr_1000_17ce(iVar5.field_0x28, SEG_1000);
        iVar5.field_0x28 = 0x0;
    }
    clenaup_win_ui_1018_4d22(param_1, SEG_1018);
}


pub fn win_ui_op_1010_3202(param_1: u32, param_2: i16, param_3: HWND16)

{
    let mut pi_var1: *mut i16;
    let mut u_var2: u32;
    let mut iVar3: i16;
    let mut u_var4: u16;
    let mut hwnd: HWND16;
    let mut ss_var1: u16;
    let mut iStack4: i16;

    iVar3 = param_1;
    u_var4 = (param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        pi_var1  = (iVar3 + 0x28);
        *pi_var1 = *pi_var1 + -0xa;
        if(*pi_var1 < 0x0)
        {
            (iVar3 + 0x28) = 0x0;
        }
    }
    else
    {
        pi_var1  = (iVar3 + 0x28);
        *pi_var1 = *pi_var1 + (iVar3 + 0x18);
    }
    if((iVar3 + 0x52) != 0x0)
    {
        iStack4 = 0x0;
        hwnd    = param_3;
        do
        {
            u_var2   = (iVar3 + 0x52);
            param_3 = hwnd;
            if((u_var2 + iStack4 * 0x4) != 0x0)
            {
                param_3 = LAST_SEGMENT;
                DestroyWindow16(hwnd);
                u_var2                   = (iVar3 + 0x52);
                (u_var2 + iStack4 * 0x4) = 0x0;
            }
            iStack4 = iStack4 + 0x1;
            hwnd    = param_3;
        } while(iStack4 < 0xa);
    }
    if((iVar3 + 0x16) == 0x0)
    {
        pass1_1010_32f4(param_1, (iVar3 + 0x56), ss_var1, param_3);
    }
    else
    {
        pass1_1010_32da(param_1, *(iVar3 + (iVar3 + 0x16) * 0x4 + 0x26), param_3, ss_var1);
    }
    pass1_1010_1f62(ss_var1, param_1, 0x8);
}


Struct11 *pass1_1010_0ee6(param_1: *mut Struct11, param_2: u8)

{
    clenaup_win_ui_1018_4d22(param_1, SEG_1018);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, SEG_1000);
    }
    return param_1;
}
