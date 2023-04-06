// #include "draw_ops_2.h"

// #include "address_tables/function_tables.h"
// #include "draw_ops_3.h"
// #include "draw_ops_4.h"
// #include "fn_ptr_ops/fn_ptr_ops_7.h"
// #include "op_int.h"
// #include "op_winapi.h"
// #include "op_windef.h"
// #include "string_ops.h"
// #include "structs/structs_0xx/struct_20.h"
// #include "structs/structs_0xx/structs_2x.h"
// #include "structs/structs_1xx/structs_12x.h"
// #include "sys_ops/sys_ops_9.h"
// #include "ui_ops/ui_ops_1.h"
// #include "unk/unk_15.h"
// #include "unk/unk_17.h"
// #include "utils.h"
// #include "win_ops/win_ops_3.h"

// #include <stddef.h>

use crate::globals_h::Globals;
use crate::structs::structs_0xx::struct_18_h::Struct18;


pub fn pass1_1038_9a48(globals: &mut Globals, param_1: *mut Struct18)

{
    param_1.field_0x0 = addr_table_1038_9af6;//0x9af6;
    param_1.field_0x2 = SEG_1038;
    unk_draw_op_1040_b0f8(param_1);
}


pub fn pass1_1038_7d5c(globals: &mut Globals, param_1: *mut Struct18)

{
    param_1.field_0x0 = addr_table_1038_8876;//0x8876;
    param_1.field_0x2 = SEG_1038;
    pass1_1038_b6e0(globals.ptr_1050_5b7c, (param_1.field_0x6));
    unk_draw_op_1040_b0f8(globals, param_1);
}

pub fn unk_draw_op_1018_cda8(globals: &mut Globals, param_1: *mut Struct36, param_2: u16)

{
    let mut pi_var1: *mut i16;
    let mut iVar2: i16;
    let mut paVar3: *mut Struct76;
    let mut ppcVar4: *mut *mut c_void;
    let mut uVar5: u16;
    let mut b_force_background: *mut HDC16;
    let mut iVar6: i16;
    let mut iVar7: i16;
    let mut in_DX: *mut u8;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut extraout_DX: u16;
    let mut iVar10: i16;
    let mut unaff_DI: i16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut hwnd: HWND16;
    let mut uVar14: u32;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut pHVar17: *mut HDC16;
    let mut pRVar19: *mut RECT16;
    let mut uVar18: u32;
    let mut local_34: u32;
    let mut iStack48: i16;
    let mut iStack46: i16;
    let mut pRStack44: *mut RECT16;
    let mut local_2a: HDC16;
    let mut uStack40: u16;
    let mut puStack38: *mut u16;
    let mut local_22: PAINTSTRUCT16;

    hwnd = SEG_1010;
    puStack38
      = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    uVar8    = (puStack38 >> 0x10);
    uVar5    = (puStack38 + 0x20);
    iVar10   = param_1;
    uVar11   = (param_1 >> 0x10);
    uStack40 = uVar5;
    if(uVar5 == 0x0)
    {
        BeginPaint16(SEG_1010, &local_22);
        EndPaint16(LAST_SEGMENT, &local_22);
        PostMessage16(LAST_SEGMENT, 0x0, 0x0, str_var1(0x111, (iVar10 + 0xea)));
        return;
    }
    if((iVar10 + 0xf0) == 0x0)
    {
        (iVar10 + 0xf0) = 0x1;
        hwnd            = SEG_1008;
        win_1008_5c5c(param_2, uVar5, uVar8, globals._PTR_LOOP_1050_02a0, 0x1f3);
        uVar12 = (globals._PTR_LOOP_1050_02a0 >> 0x10);
        if((globals._PTR_LOOP_1050_02a0 + 0x12) == 0x0)
        {
            hwnd = SEG_1008;
            win_1008_5c5c(param_2,
                          uVar5,
                          uVar8,
                          globals._PTR_LOOP_1050_02a0 & 0xffff | uVar12 << 0x10,
                          0x1d3);
        }
    }
    local_2a  = BeginPaint16(hwnd, &local_22);
    pRStack44 = CreateSolidBrush16(LAST_SEGMENT);
    local_34  = 0x0;
    iStack48  = (iVar10 + 0xf6) + -0x1;
    iStack46  = (iVar10 + 0xf8) + -0x1;
    FillRect16(LAST_SEGMENT, pRStack44, (HBRUSH16)&local_34);
    pRVar19 = pRStack44;
    DeleteObject16(LAST_SEGMENT);
    uVar18             = (iVar10 + 0xe2);
    paVar3             = (uVar18 + 0xe);
    b_force_background = &local_2a;
    uVar18             = str_var1(pRVar19, param_2);
    uVar13             = (paVar3 >> 0x10);
    ppcVar4            = (paVar3 + 0x8);
    uVar15             = paVar3;
    uVar16             = uVar13;
    pHVar17            = b_force_background;
    unsafe{(**ppcVar4)()};
    uVar14           = pass1_1008_4772(paVar3);
    uVar9            = (uVar14 >> 0x10);
    iVar6            = (0x280 - (uVar14 + 0x4)) / 0x2;
    iVar2            = (uVar14 + 0x8);
    iVar7            = (0x1e0 - iVar2) / 0x2;
    (iVar10 + 0x10c) = iVar7 + iVar2 + (iVar10 + 0x110);
    if(((iVar10 + 0xfa) == 0x0) && (iVar6 == 0x0))
    {
        pi_var1  = (iVar10 + 0xfa);
        unsafe{*pi_var1 = *pi_var1 + 0x2;}
    }
    ppcVar4 = (paVar3 + 0x4);
    unsafe{(**ppcVar4)(SEG_1008,
                paVar3,
                uVar13,
                (iVar10 + 0xfc) + (iVar10 + 0xfe) + iVar7,
                (iVar10 + 0xfa) + iVar6,
                &local_2a,
                param_2,
                uVar15,
                uVar16,
                pHVar17,
                uVar18);}
    draw_text_1018_c742(param_1, SEG_1008, param_2, extraout_DX);
    SelectPalette16(SEG_1008, 0x0, b_force_background);
    DeleteObject16(LAST_SEGMENT);
    EndPaint16(LAST_SEGMENT, &local_22);
}

pub fn unk_draw_op_1018_cfc0(globals: &mut Globals, param_1: *mut Struct36, param_2: u16)

{
    let mut pi_var1: *mut i16;
    let mut iVar2: i16;
    let mut paVar3: *mut Struct76;
    let mut ppcVar4: *mut *mut c_void;
    let mut uVar5: u16;
    let mut b_force_background: *mut HDC16;
    let mut iVar6: i16;
    let mut iVar7: i16;
    let mut in_DX: *mut u8;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut extraout_DX: u16;
    let mut iVar10: i16;
    let mut unaff_DI: i16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut hwnd: HWND16;
    let mut uVar13: u32;
    let mut uVar14: u16;
    let mut uVar15: u16;
    let mut pHVar16: *mut HDC16;
    let mut pRVar18: *mut RECT16;
    let mut uVar17: u32;
    let mut HVar19: HDC16;
    let mut local_34: u32;
    let mut iStack48: i16;
    let mut iStack46: i16;
    let mut pRStack44: *mut RECT16;
    let mut local_2a: HDC16;
    let mut iStack40: i16;
    let mut puStack38: *mut u16;
    let mut local_22: PAINTSTRUCT16;

    hwnd = SEG_1010;
    puStack38
      = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    uVar8    = (puStack38 >> 0x10);
    iStack40 = (puStack38 + 0x20);
    iVar10   = param_1;
    uVar11   = (param_1 >> 0x10);
    if(iStack40 == 0x0)
    {
        BeginPaint16(SEG_1010, &local_22);
        EndPaint16(LAST_SEGMENT, &local_22);
        PostMessage16(LAST_SEGMENT, 0x0, 0x0, str_var1(0x111, (iVar10 + 0xea)));
        return;
    }
    if(((iVar10 + 0xf0) == 0x0) && ((iVar10 + 0xf4) != 0x0))
    {
        (iVar10 + 0xf0) = 0x1;
        uVar5           = iVar10 + 0xf2;
        hwnd            = SEG_1008;
        win_1008_5c9e(globals._PTR_LOOP_1050_02a0,
                      (param_1 & 0xffff0000 | uVar5),
                      uVar5,
                      uVar8,
                      param_2);
        if((globals._PTR_LOOP_1050_02a0 + 0x12) == 0x0)
        {
            hwnd = SEG_1008;
            win_1008_5c5c(param_2, uVar5, uVar8, globals._PTR_LOOP_1050_02a0, 0x1d3);
        }
    }
    local_2a  = BeginPaint16(hwnd, &local_22);
    pRStack44 = CreateSolidBrush16(LAST_SEGMENT);
    local_34  = 0x0;
    iStack48  = (iVar10 + 0xf6) + -0x1;
    iStack46  = (iVar10 + 0xf8) + -0x1;
    uVar8     = param_2;
    HVar19    = local_2a;
    FillRect16(LAST_SEGMENT, pRStack44, (HBRUSH16)&local_34);
    pRVar18 = pRStack44;
    DeleteObject16(LAST_SEGMENT);
    uVar17             = (iVar10 + 0xe2);
    paVar3             = (uVar17 + 0xe);
    b_force_background = &local_2a;
    uVar17             = str_var1(pRVar18, param_2);
    uVar12             = (paVar3 >> 0x10);
    ppcVar4            = (paVar3 + 0x8);
    uVar14             = paVar3;
    uVar15             = uVar12;
    pHVar16            = b_force_background;
    unsafe{(**ppcVar4)();}
    uVar13           = pass1_1008_4772(paVar3);
    uVar9            = (uVar13 >> 0x10);
    iVar6            = (0x280 - (uVar13 + 0x4)) / 0x2;
    iVar2            = (uVar13 + 0x8);
    iVar7            = (0x1e0 - iVar2) / 0x2;
    (iVar10 + 0x10c) = iVar7 + iVar2 + (iVar10 + 0x110);
    if(((iVar10 + 0xfa) == 0x0) && (iVar6 == 0x0))
    {
        pi_var1  = (iVar10 + 0xfa);
        unsafe{*pi_var1 = *pi_var1 + 0x2;}
    }
    ppcVar4 = (paVar3 + 0x4);
    unsafe{(**ppcVar4)(SEG_1008,
                paVar3,
                uVar12,
                (iVar10 + 0xfc) + (iVar10 + 0xfe) + iVar7,
                (iVar10 + 0xfa) + iVar6,
                &local_2a,
                param_2,
                uVar14,
                uVar15,
                pHVar16,
                uVar17,
                uVar8,
                HVar19);}
    draw_text_1018_c742(param_1, SEG_1008, param_2, extraout_DX);
    SelectPalette16(SEG_1008, 0x0, b_force_background);
    DeleteObject16(LAST_SEGMENT);
    EndPaint16(LAST_SEGMENT, &local_22);
}

pub fn palette_op_1020_92c4(param_1: *mut Struct587, param_2: HDC16)

{
    //    i16 iVar1;
    //    u16 u_var2;
    //    u_var2         = (param_1 >> 0x10);
    //    iVar1         = param_1;
    param_1.field_0x0 = addr_table_1020_96c8; // 0x96c8;
    param_1.field_0x2 = SEG_1020;
    if(param_1.field_0x12 != 0x0)
    {
        SelectPalette16(param_2, 0x0, (param_1.field_0x12));
        DeleteObject16(LAST_SEGMENT);
    }
    param_1.field_0x0   = addr_table_1008_3aa0[4]; // 0x3ab0;
    (param_1.field_0x2) = SEG_1008;
    param_1.field_0x0   = addr_table_1008_380a[36]; // 0x389a;
    (param_1.field_0x2) = SEG_1008;
}


pub fn mix_draw_op_1020_9312(param_1: u32, param_2: HWND16)

{
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut local_22: PAINTSTRUCT16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar6 = (iVar4 + 0x4);
    BeginPaint16(param_2, &local_22);
    uVar3   = (iVar4 + 0x6);
    puVar1  = (uVar3 + 0xa);
    unsafe{ppcVar2 = (*puVar1 + 0x4);}
    unsafe{(**ppcVar2)(LAST_SEGMENT,
                puVar1,
                (puVar1 >> 0x10),
                0x0,
                param_1 & 0xffff0000 | (iVar4 + 0xa),
                uVar6);}
    EndPaint16(LAST_SEGMENT, &local_22);
    return;
}

pub fn draw_op_1020_9364(param_1: *mut Struct7, in_win_handle_2: HWND16, param_3: u16)

{
    let mut pi_var1: *mut i16;
    let mut u_var2: u16;
    let mut iVar3: i16;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut pRVar6: *mut RECT16;
    let mut local_struct_1: *mut Struct7;
    let mut var7: u16;
    let mut uVar7: u16;
    let mut iStack62: i16;
    let mut uStack58: u16;
    let mut local_38: [u8;4] = [0;4];
    let mut HStack52: HGDIOBJ16;
    let mut HStack50: HPEN16;
    let mut uStack48: u16;
    let mut uStack46: u32;
    let mut uStack42: u32;
    let mut uStack38: u32;
    let mut uStack34: u32;
    let mut uStack30: u32;
    let mut puStack26: *mut u16;
    let mut iStack22: i16;
    let mut iStack20: i16;
    let mut local_12: u32;
    let mut uStack14: u32;
    let mut local_a: RECT16;
    let mut u_stack6: u32;

    var7           = (param_1 >> 0x10);
    local_struct_1 = param_1;
    GetClientRect16(in_win_handle_2, &local_a);
    local_12  = local_a;
    uStack14  = u_stack6;
    iStack20  = DAT_1050_4216;
    iStack22  = DAT_1050_422c;
    puStack26 = globals._PTR_PTR_DAT_1050_0009_1050_4172_1050_4212;
    uStack30  = globals._PTR_PTR_1050_4218;
    uStack34  = globals._PTR_PTR_s_ew_failed_in_Op_Op_1050_0021_1050_41da_1050_421c;
    uStack38  = globals._PTR_PTR_DAT_1050_0041_1050_4202_1050_4220;
    uStack42  = globals._PTR_DAT_1050_419a_1050_4224;
    uStack46  = globals._PTR_PTR_1050_4228;
    uVar4     = local_struct_1.field_0x6;
    uStack48  = (uVar4 + 0x12);
    uStack58  = 0x9;
    loop
    {
        uVar4    = (uStack58 * 0x4 + uStack34);
        HStack50 = CreatePen16(LAST_SEGMENT, uVar4, (uVar4 >> 0x10));
        HStack52 = SelectObject16(LAST_SEGMENT, HStack50);
        MoveToEx16(LAST_SEGMENT,
                   local_38,
                   param_3,
                  (uStack58 * 0x2 + puStack26));
        LineTo16(LAST_SEGMENT, (puStack26 + uStack58 * 0x2), u_stack6);
        iVar3 = (iStack20 - uStack58) * 0x2;
        MoveToEx16(
          LAST_SEGMENT, local_38, param_3,(iVar3 + puStack26));
        LineTo16(LAST_SEGMENT, (puStack26 + iVar3), u_stack6);
        SelectObject16(LAST_SEGMENT, HStack52);
        DeleteObject16(LAST_SEGMENT);
        uStack58 = uStack58 - 0x1;
            if !(uStack58 < 0x8000) {break;}
    }
    // while(uStack58 < 0x8000);
    pRVar6   = CreateSolidBrush16(LAST_SEGMENT);
    uVar7    = (puStack26 >> 0x10);
    local_a  = str_var1((puStack26 + 0x12) + 0x1, local_a.x);
    u_var2   = (puStack26 + 0x14);
    uStack14 = uStack14 & 0xffff | u_var2 << 0x10;
    u_stack6 = str_var1(u_var2, u_stack6);
    FillRect16(LAST_SEGMENT, pRVar6, (HBRUSH16)&local_a);
    DeleteObject16(LAST_SEGMENT);
    iStack62 = 0x8;
    // for(uStack58 = 0x1; uStack58 < 0xa; uStack58 = uStack58 + 0x1)
    for uStack58 in 0x1..0xa
    {
        pRVar6   = CreateSolidBrush16(LAST_SEGMENT);
        u_stack6 = u_stack6 & 0xffff | (local_a.y - 0x1) << 0x10;
        local_12 = local_12 & 0xffff | (uStack14 + 0x1) << 0x10;
        uVar7    = (puStack26 >> 0x10);
        local_a  = local_a & 0xffff | ((iStack62 * 0x2 + puStack26) + 0x1) << 0x10;
        uStack14 = uStack14 & 0xffff | (uStack58 * 0x2 + puStack26 + 0x14) << 0x10;
        FillRect16(LAST_SEGMENT, pRVar6, (HBRUSH16)&local_a);
        FillRect16(LAST_SEGMENT, pRVar6, (HBRUSH16)&local_12);
        DeleteObject16(LAST_SEGMENT);
        iStack62 = iStack62 + -0x1;
    }
    pRVar6   = CreateSolidBrush16(LAST_SEGMENT);
    local_a  = local_a & 0xffff;
    unsafe{u_stack6 = u_stack6 & 0xffff | *puStack26 << 0x10;}
    local_12 = local_12 & 0xffff | ((iStack20 * 0x2 + puStack26) + 0x1) << 0x10;
    uStack14 = uStack14 & 0xffff | local_struct_1.field_0xe << 0x10;
    FillRect16(LAST_SEGMENT, pRVar6, (HBRUSH16)&local_a);
    FillRect16(LAST_SEGMENT, pRVar6, (HBRUSH16)&local_12);
    DeleteObject16(LAST_SEGMENT);
    uStack58 = 0x3;
    loop
    {
        uVar4    = (uStack58 * 0x4 + uStack38);
        HStack50 = CreatePen16(LAST_SEGMENT, uVar4, (uVar4 >> 0x10));
        HStack52 = SelectObject16(LAST_SEGMENT, HStack50);
        iVar5    = uStack58 * 0x2;
        iVar3    = (iVar5 + uStack42);
        uVar7    = (uStack46 >> 0x10);
        pi_var1  = (iVar5 + uStack46);
        MoveToEx16(LAST_SEGMENT,
                   local_38,
                   param_3,
                  ((iVar5 + uStack46) * 0x2 + puStack26));
        unsafe{LineTo16(LAST_SEGMENT,
                 ((iStack20 - *pi_var1) * 0x2 + puStack26),
                 iVar3 + local_a.x);}
        iVar3 = ((iStack22 - uStack58) * 0x2 + uStack42);
        unsafe{MoveToEx16(LAST_SEGMENT,
                   local_38,
                   param_3,
                  (*pi_var1 * 0x2 + puStack26));}
        unsafe{LineTo16(LAST_SEGMENT,
                 ((iStack20 - *pi_var1) * 0x2 + puStack26),
                 iVar3 + local_a.x);}
        SelectObject16(LAST_SEGMENT, HStack52);
        DeleteObject16(LAST_SEGMENT);
        uStack58 = uStack58 - 0x1;
            if !(uStack58 < 0x8000) {break;}
    }
    // while(uStack58 < 0x8000);
    local_struct_1.field_0x10 = 0x0;
    return;
}


pub fn pass1_1020_96a2(param_1: *mut Struct18, param_2: u8, param_3: u16) -> *mut Struct18

{
    palette_op_1020_92c4(&param_1.field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, SEG_1000);
    }
    return param_1;
}

pub fn pass1_1020_7526(param_1: *mut Struct18, param_2: u8, param_3: u16) -> *mut Struct18

{
    palette_op_1020_7270(&param_1.field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, SEG_1000);
    }
    return param_1;
}

pub fn struct_1020_7554(globals: &mut Globals,
                      param_1: u16,
                      param_2: *mut Struct20,
                      param_3: u16,
                     param_4: u16)

{
    let mut extraout_DX: *mut u8;
    let mut uVar1: u16;
//    Struct129 *iVar2;
    let mut unaff_DI: i16;
    let mut u_var2: u16;
    let mut puVar3: *mut u16;

    unk_draw_op_1020_7f7a(globals, param_2, 0x5, str_var1(param_4, param_3));
//    u_var2             = (param_2 >> 0x10);
//    iVar2              = param_2;
    param_2.field_0xee  = 0x0;
    param_2.field_0xf2 = 0x0;
    param_2.field_0x0 = addr_table_1020_7780; //0x7780;
    param_2.field_0x2   = SEG_1020;
    param_2.field_0xe2  = addr_table_1020_7780[39];0x781c;
    param_2.field_0xe4  = SEG_1020;
    puVar3 = mixed_1010_20ba(globals.data_1050_0ed0, 0x25, param_1, extraout_DX, unaff_DI);
    uVar1  = (puVar3 >> 0x10);
    param_2.field_0xf2 = puVar3;
    param_2.field_0xf4 = uVar1;
    param_2.field_0xe6 = param_2.field_0xf2;
    param_2.field_0xe8 = uVar1;
}

pub fn pass1_1020_7824(globals: &mut Globals,
                     param_1: *mut Struct7,
                     param_3: u16,
                     param_4: i16,
                    param_5: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u32;
    let mut iVar3: i16;
    let mut extraout_DX: *mut u8;
    let mut uVar4: u16;
    let mut puVar5: *mut u16;

    set_struct_op_1020_921c(param_1, param_3);
    param_1.field_0x14       = 0x0;
    param_1.field_0x0 = addr_table_1020_7902;//0x7902;
    param_1.field_0x2         = SEG_1020;
    puVar5 = mixed_1010_20ba(globals.data_1050_0ed0, 0x25, param_5, extraout_DX, param_4);
    uVar4  = (puVar5 >> 0x10);
    param_1.field_0x14 = puVar5;
    param_1.field_0x16 = uVar4;
    param_1.field_0x6  = param_1.field_0x14;
    param_1.field_0x8  = uVar4;
    u_var2              = param_1.field_0x14;
    iVar3               = param_1.field_0xa;
    ppcVar1             = ((u_var2 + 0xa) + 0x8);
    unsafe{(**ppcVar1)();}
    param_1.field_0x12 = iVar3;
    draw_op_1020_9364(param_1, SEG_1010, param_5);
}

pub fn pass1_1020_78ac(pstruct_arg1: *mut Struct587, hdc_arg2: u16)

{
    //    Struct587 *iVar1;
    //    u16          uVar1;

    //    uVar1            = (param_1 >> 0x10);
    //    iVar1            = param_1;
    pstruct_arg1.field_0x0 = addr_table_1020_7902; // 0x7902;
    pstruct_arg1.field_0x2 = SEG_1020;
    if(pstruct_arg1.field_0x14 != 0x0)
    {
        hdc_arg2 = SEG_1010;
        pass1_1010_1dda(pstruct_arg1.field_0x14);
    }
    palette_op_1020_92c4(pstruct_arg1, hdc_arg2);
}

pub fn struct_1020_790e(param_1: *mut Struct271,
                      param_2: u32,
                      param_3: u16,
                      param_4: u32,
                     param_5: u16)

{
    //    Struct271 *iVar1;
    //    u16          uVar1;

    unk_draw_op_1008_7f62(param_1, param_3, param_4, param_5);
    //    uVar1             = (param_1 >> 0x10);
    //    iVar1             = param_1;
    param_1.field_0xe0 = 0x0;
    param_1.field_0xe4 = 0x0;
    param_1.field_0xe8 = 0x0;
    param_1.field_0xec = 0x0;
    param_1.field_0xee = param_2;
    param_1.field_0x0  = addr_table_1020_7b86; // 0x7b86;
    param_1.field_0x2  = SEG_1020;
}


pub fn string_1020_79b4(globals: &mut Globals,
                      param_1: u16,
                      param_2: *mut struct_1020_6e52_1,
                      param_3: i16,
                      param_4: *mut c_char)

{
    unk_str_op_1000_3d3e((param_2 + 0xa), param_4);
    if(param_3 != 0x0)
    {
        draw_op_1020_7cc8(globals, *(param_2 + 0xe8), SEG_1000, param_1);
    }
}


pub fn pass1_1020_79e4(globals: &mut Globals, param_1: u32, hwnd_arg2: u16, param_3: u16)

{
    draw_op_1020_7cc8(globals, *(param_1 + 0xe8), hwnd_arg2, param_3);
}

pub fn draw_op_1020_7cc8(globals: &mut Globals,
                       struct_arg1: *mut Struct6,
                       hwnd_arg2: HWND16,
                       param_3: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut rect: *mut RECT16;
    let mut color: COLORREF;
    let mut handle: HPEN16;
    let mut handle_00: HGDIOBJ16;
    let mut count: *mut c_char;
    let mut str: *mut c_char;
    let mut pu_var2: *mut u32;
    let mut in_DX: u16;
    let mut str_00: *mut c_char;
    let mut iVar4: *mut Struct6;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut DVar6: DWORD;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut hbrush: HBRUSH16;
    let mut uVar9: u32;
    let mut HVar10: HDC16;
    let mut uVar11: u16;
    let mut iStack66: i16;
    let mut local_20: u16;
    let mut iStack30: i16;
    let mut iStack28: i16;
    let mut iStack26: i16;
    let mut iStack24: i16;
    let mut iStack22: i16;
    let mut local_rect_1: RECT16;
    let mut iStack16: i16;
    let mut iStack14: i16;
    let mut HStack12: HPALETTE16;
    let mut paStack10: *mut Struct13;
    let mut local_hdc_1: HDC16;
    let mut is_iconic: BOOL16;

    uVar4     = (struct_arg1 >> 0x10);
    iVar4     = struct_arg1;
    is_iconic = IsIconic16(hwnd_arg2);
    if((is_iconic == 0x0) || (globals.PTR_LOOP_1050_0010 != 0x0))
    {
        local_hdc_1 = GetWindowDC16(LAST_SEGMENT);
        paStack10   = (globals._PTR_LOOP_1050_423e);
        HStack12    = palette_op_1008_4e08(paStack10, &local_hdc_1, in_DX, SEG_1008);
        uVar11      = iVar4.field_0x4;
        GetWindowRect16(SEG_1008, &local_rect_1);
        iStack28 = (iStack16 - local_rect_1.x) + -0x1;
        iStack24 = (iStack14 - local_rect_1.y) + -0x1;
        local_20 = iVar4.field_0x10;
        iStack30 = iVar4.field_0x12;
        iStack26 = iStack24;
        if(is_iconic == 0x0)
        {
            iStack26 = iVar4.field_0xe - iVar4.field_0x12;
        }
        uVar9    = str_var1(param_3, &local_20);
        hbrush   = 0x4;
        HVar10   = local_hdc_1;
        iStack22 = iStack28;
        rect     = GetStockObject16(LAST_SEGMENT);
        FillRect16(LAST_SEGMENT, rect, hbrush);
        pu_var2 = iVar4.field_0x6;
        uVar5   = (pu_var2 >> 0x10);
        iVar3   = pu_var2;
        pu_var2 = (iVar3 + 0xe0);
        unsafe{ppcVar1 = (*pu_var2 + 0x24);}
        unsafe{(**ppcVar1)(LAST_SEGMENT, pu_var2, (iVar3 + 0xe2), 0x0, uVar9, HVar10, uVar11);}
        color     = (-(pu_var2 == 0x0) & 0x1e) + 0x25;
        handle    = CreatePen16(LAST_SEGMENT, color, 0x100);
        handle_00 = SelectObject16(LAST_SEGMENT, handle);
        MoveTo16(LAST_SEGMENT, 0x0, 0x0);
        LineTo16(LAST_SEGMENT, 0x0, iStack22);
        LineTo16(LAST_SEGMENT, iStack24, iStack22);
        uVar7 = local_hdc_1 << 0x10;
        LineTo16(LAST_SEGMENT, iStack24, 0x0);
        uVar8 = uVar7 & 0xffff0000 | local_hdc_1;
        uVar7 = 0x0;
        count = LineTo16(LAST_SEGMENT, 0x0, 0x0);
        if(is_iconic == 0x0)
        {
            iVar3 = iVar4.field_0xe - iVar4.field_0x12;
            uVar7 = local_hdc_1 << 0x10;
            MoveTo16(LAST_SEGMENT, iVar3, 0x0);
            uVar7 = uVar7 & 0xffff0000 | local_hdc_1;
            count = LineTo16(LAST_SEGMENT, iVar3, iStack22);
        }
        ppcVar1 = (*iVar4.field_0x6 + 0x18);
        unsafe{(**ppcVar1)(LAST_SEGMENT, iVar4.field_0x6, uVar7, uVar8);}
        if(unsafe{*count != '\0'})
        {
            SetBkColor16(LAST_SEGMENT, 0x0);
            SetTextColor16(LAST_SEGMENT, color);
            str   = lstrlen16(LAST_SEGMENT);
            DVar6 = GetTextExtent16(LAST_SEGMENT, str, count);
            iVar3 = (DVar6 >> 0x10);
            if(is_iconic == 0x0)
            {
                iStack66 = (iStack26 - iStack30) / 0x2 - iVar3 / 0x2;
            }
            else
            {
                iStack66 = iStack24 / 0x2 - iVar3 / 0x2;
            }
            TextOut16(LAST_SEGMENT, str, count, str_00, iStack66);
        }
        HStack12 = SelectPalette16(LAST_SEGMENT, 0x0, HStack12);
        DeleteObject16(LAST_SEGMENT);
        SelectObject16(LAST_SEGMENT, handle_00);
        DeleteObject16(LAST_SEGMENT);
        ReleaseDC16(LAST_SEGMENT, local_hdc_1);
    }
    return;
}

pub fn unk_draw_op_1020_7f7a(globals: &mut Globals, param_1: *mut Struct20, param_2: u16, param_3: u32)

{
    let mut uVar1: u16;
    let mut HVar2: HGDIOBJ16;
    let mut HVar3: HCURSOR16;
    let mut puVar4: *mut u8;
    //    Struct20 *iVar4;
    let mut unaff_DI: i16;
    //    u16         uVar5;
    let mut unaff_SS: u16;
    let mut paVar6: *mut Struct20;
    let mut puVar7: *mut u16;
    let mut in_stack_0000000e: u16;

    paVar6 = unk_draw_op_1008_61b2(
      NULL, param_1, param_2, param_3, str_var1(in_stack_0000000e, param_3), unaff_SS);
    puVar4 = (paVar6 >> 0x10);
    //    uVar5                      = (param_1 >> 0x10);
    //    iVar4                      = param_1;
    ((param_1 + 0x1)).field_0x0 = addr_table_1008_380a[36]; //0x389a;
    param_1[0x1].field_0x2       = SEG_1008;
    ((param_1 + 0x1)).field_0x0 = addr_table_1008_3aa0[2];//0x3aa8;
    param_1[0x1].field_0x2       = SEG_1008;
    param_1[0x1].field_0x4       = 0x0;
    param_1[0x1].field_0x8       = 0x0;
    param_1[0x1].field_0xa       = 0x0;
    param_1.field_0x0           = addr_table_1020_7f72[2];//0x82bc;
    param_1.field_0x2           = SEG_1020;
    ((param_1 + 0x1)).field_0x0 = addr_table_1020_7f72[41];//0x8358;
    param_1[0x1].field_0x2       = SEG_1020;
    unk_str_op_1000_3d3e(param_1.field_0x5b,
                         globals.s_VrMode_1050_4422);
    HVar2                       = GetStockObject16(SEG_1000);
    param_1.hgdiobj_field_0xc6 = HVar2;
    HVar3                       = LoadCursor16(LAST_SEGMENT, 0x7f00);
    param_1.hcursor_field_0xc4 = HVar3;
    param_1.field_0xc8         = 0x2028;
    param_1.field_0xac         = 0x47000000;
    param_1.field_0xbc         = (param_3 + 0x8);
    puVar7
      = mixed_1010_20ba(globals.data_1050_0ed0, 0x48, unaff_SS, puVar4, unaff_DI);
    uVar1               = (puVar7 >> 0x10);
    param_1.field_0xb4 = 0x0;
    param_1.field_0xb6 = 0x0;
    param_1.field_0xb8 = (puVar7 + 0xa);
    param_1.field_0xba = (puVar7 + 0xc);
    param_1.field_0xca = param_3;
    win_ui_reg_class_1008_96d2(param_1, SEG_1008, unaff_SS);
}

pub fn realize_palette_1020_8128(param_1: u32, param_2: i16, param_3: HGDIOBJ16, param_4: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u32;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut extraout_DX: u16;
    let mut iVar6: i16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut local_12: [u8;8] = [0;8];
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut pu_stack6: *mut u32;

    if(param_2 != 0x0)
    {
        uVar8     = (param_1 >> 0x10);
        iVar6     = param_1;
        u_var2    = (iVar6 + 0xe6);
        uVar9     = (u_var2 >> 0x10);
        iVar7     = u_var2;
        puVar5    = (iVar7 + 0xa);
        unsafe{ppcVar1   = (*puVar5 + 0x18);}
        pu_stack6 = puVar5;
        unsafe{(**ppcVar1)(param_3, puVar5, (iVar7 + 0xc));}
        uStack8 = SUB42(puVar5, 0x0);
        UnrealizeObject16(param_3);
        u_var2   = (iVar6 + 0xe6);
        uVar8    = (u_var2 + 0x14);
        uStack10 = uVar8;
        RealizePalette16(LAST_SEGMENT);
        pass1_1008_57a4(str_var1(param_4, local_12),
                        param_1 & 0xffff0000 | (iVar6 + 0xd2));
        while(true)
        {
            puVar3 = local_12;
            pass1_1008_5b12(puVar3, param_4);
            if((extraout_DX | puVar3) == 0x0){
                break;}
            uVar9   = (puVar3 + 0x6);
            puVar4  = (puVar3 + 0x4);
            unsafe{ppcVar1 = (*puVar4 + 0x90);}
            unsafe{(**ppcVar1)(SEG_1008, puVar4, uVar9, 0x1, uVar8);}
        }
    }
    return;
}

pub fn win_ui_palette_op_1020_81c0(param_1: HWND16)

{
    let mut in_struct_1: *mut Struct13;
    let mut b_force_background: BOOL16;
    let mut b_force_background_00: HPALETTE16;
    let mut UVar1: u16;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut u_stack6: u16;

    uVar3       = (globals._PTR_LOOP_1050_4230 >> 0x10);
    in_struct_1 = (globals._PTR_LOOP_1050_4230 + 0xe);
    u_var2      = (globals._PTR_LOOP_1050_4230 + 0x10);
    u_stack6    = in_struct_1;
    if((u_var2 | u_stack6) == 0x0)
    {
        return;
    }
    b_force_background = GetDC16(param_1);
    create_palette_1008_4e38(in_struct_1, SEG_1008, u_var2);
    b_force_background_00 = SelectPalette16(SEG_1008, 0x0, b_force_background);
    UVar1                 = RealizePalette16(LAST_SEGMENT);
    SelectPalette16(LAST_SEGMENT, 0x1, b_force_background_00);
    RealizePalette16(LAST_SEGMENT);
    DeleteObject16(LAST_SEGMENT);
    if(0x0 < UVar1)
    {
        InvalidateRect16(
          LAST_SEGMENT, (&PTR_LOOP_1050_0000 + 0x1), 0x0);
    }
    return;
}

pub fn pass1_1020_6466(param_1: *mut u16, param_2: u16, param_3: u16)

{
    let mut iVar1: *mut Struct585;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = addr_table_1020_67c2;//0x67c2;
    iVar1.field_0x2 = SEG_1020;
    if (iVar1.field_0x14 != 0x0) {
        param_2 = SEG_1010;
        pass1_1010_1ea6(iVar1.field_0x14, param_1 & 0xffff | uVar1 << 0x10, param_3);
    }
    palette_op_1020_92c4(param_1, param_2);
}
pub fn mix_draw_op_1020_650c(param_1: *mut Struct7, param_2: HWND16, param_3: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u32;
    let mut iVar3: i16;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut local_28: PAINTSTRUCT16;
    let mut iStack8: i16;
    let mut pu_stack6: *mut u32;

    uVar6     = (param_1 >> 0x10);
    iVar3     = param_1;
    u_var2    = (iVar3 + 0x14);
    pu_stack6 = (u_var2 + 0xa);
    if(((iVar3 + 0x10) != 0x0) || (u_var2 = (iVar3 + 0x14), (u_var2 + 0x24) != 0x0))
    {
        draw_op_1020_9364(param_1, param_2, param_3);
        if((iVar3 + 0x24) != 0x0)
        {
            u_var2  = (iVar3 + 0x24);
            ppcVar1 = ((iVar3 + 0x24) + 0x14);
            unsafe{(**ppcVar1)(param_2, u_var2, (u_var2 >> 0x10));}
        }
    }
    iStack8 = 0x0;
    loop
    {
        iVar4 = iVar3 + 0x18;
        iVar5 = iStack8 * 0x4;
        if((iVar4 + iVar5) != 0x0)
        {
            u_var2  = (iVar4 + iVar5);
            ppcVar1 = ((iVar4 + iVar5) + 0x8);
            unsafe{(**ppcVar1)(
              param_2, u_var2, (u_var2 >> 0x10), pu_stack6, (pu_stack6 >> 0x10));}
        }
        iStack8 = iStack8 + 0x1;
        if !(iStack8 < 0x5)
        {
            break;
        }
    }
    // while(iStack8 < 0x5);
    uVar7 = (iVar3 + 0x4);
    BeginPaint16(param_2, &local_28);
    unsafe{ppcVar1 = (*pu_stack6 + 0x4);}
    unsafe{(**ppcVar1)(
      LAST_SEGMENT, pu_stack6, (pu_stack6 >> 0x10), 0x0, 0x0, iVar3 + 0xa, uVar6, uVar7);}
    EndPaint16(LAST_SEGMENT, &local_28);
    return;
}
pub fn realize_palette_1020_6896(param_1: u32, param_2: i16, param_3: HGDIOBJ16)

{
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;

    if(param_2 != 0x0)
    {
        uVar3   = (param_1 + 0xf2);
        uVar5   = (uVar3 >> 0x10);
        iVar4   = uVar3;
        puVar1  = (iVar4 + 0x24);
        unsafe{ppcVar2 = (*puVar1 + 0x18);}
        unsafe{(**ppcVar2)(param_3, puVar1, (iVar4 + 0x26));}
        UnrealizeObject16(param_3);
        RealizePalette16(LAST_SEGMENT);
    }
    return;
}


pub fn pass1_1020_68de(param_1: u32, param_2: u16)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if((param_1 + 0xf6) != 0x0)
    {
        invalidate_rect_1020_735a(*(param_1 + 0xf6), param_2);
    }
    return;
}


pub fn pt_in_rect_1020_68fc(param_1: *mut u32, param_2: u16, param_3: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u16;
    let mut BVar3: BOOL16;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut PStack6: POINT16;

    PStack6 = str_var1(param_2, param_3);
    uVar5   = (param_1 >> 0x10);
    u_var2  = pass1_1018_31d0(*(param_1 + 0xf2));
    if(u_var2 != 0x0)
    {
        uVar4 = *(param_1 + 0xf2);
        uVar4 = uVar4 & 0xffff0000 | (uVar4 + 0x16c);
        BVar3 = PtInRect16(SEG_1018, PStack6);
        if(BVar3 != 0x0)
        {
            unsafe{ppcVar1 = (*param_1 + 0x40);}
            unsafe{(**ppcVar1)(LAST_SEGMENT, param_1, 0xef, uVar4);}
        }
    }
    return;
}

pub fn destroy_icon_1020_6bd2(param_1: u32, param_2: u8, param_3: HICON16)

{
    let mut puVar1: *mut u32;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar6 = (iVar4 + 0xc2);
    DestroyIcon16(param_3);
    (iVar4 + 0xc2) = 0x0;
    (iVar4 + 0x8)  = 0x0;
    puVar1         = (iVar4 + 0xf6);
    u_var2         = (iVar4 + 0xf8);
    if((u_var2 | puVar1) != 0x0)
    {
        unsafe{ppcVar3 = *puVar1;}
        unsafe{(**ppcVar3)(LAST_SEGMENT, puVar1, u_var2, 0x1, uVar6);}
    }
    (iVar4 + 0xf6) = 0x0;
    pass1_1010_1dda(*(iVar4 + 0xf2));
    (iVar4 + 0xf2) = 0x0;
    return;
}

pub fn draw_op_1020_7070(param_1: u16, param_2: u16) -> HGDIOBJ16

{
    let mut HVar1: HGDIOBJ16;

    HVar1 = GetStockObject16(param_1);
    if(globals._PTR_LOOP_1050_441e == 0x0)
    {
        globals._PTR_LOOP_1050_441e = 0x1000002;
    }
    if(0x6 < param_2)
    {
        return 0x0;
    }
    SetTextColor16(LAST_SEGMENT, _PTR_LOOP_1050_441e);
    SetBkColor16(LAST_SEGMENT, 0x0);
    return HVar1;
}

pub fn palette_op_1020_7270(param_1: *mut u16, param_2: HDC16)

{
    let mut uVar1: u16;
    let mut u_var2: u16;
    let mut HVar3: HPALETTE16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut paStack8: *mut Struct18;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field_0x0 = addr_table_1020_754c;//0x754c;
    (iVar4 + 0x2) = SEG_1020;
    if ((iVar4 + 0x1c) != 0x0) {
        param_2 = SEG_1010;
        pass1_1010_1ea6(*(iVar4 + 0x1c), param_1 & 0xffff | uVar5 << 0x10, unaff_SS);
    }
    uVar1 = (iVar4 + 0x14);
    u_var2 = (iVar4 + 0x16);
    paStack8 = str_var1(u_var2, uVar1);
    if ((u_var2 | uVar1) != 0x0) {
        pass1_1008_5118(str_var1(u_var2, uVar1));
        param_2 = SEG_1000;
        fn_ptr_1000_17ce(paStack8, SEG_1000);
    }
    HVar3 = SelectPalette16(param_2, 0x0,  (iVar4 + 0x20));
     (iVar4 + 0x20) = HVar3;
    DeleteObject16( LAST_SEGMENT);
    param_1.field_0x0 = addr_table_1008_3aa0[4];//0x3ab0;
    (iVar4 + 0x2) = SEG_1008;
    param_1.field_0x0 = addr_table_1008_380a[36];//0x389a;
    (iVar4 + 0x2) = SEG_1008;
}

pub fn invalidate_rect_1020_735a(param_1: u32, param_2: HWND16)

{
    InvalidateRect16(param_2,
                     0x0,
                     (param_1 + 0x1c) + 0x16c);
}


pub fn win_ui_op_1020_737a(param_1: u32, param_2: HWND16, param_3: u16) -> BOOL16

{
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u32;
    let mut BVar4: BOOL16;
    let mut puVar5: *mut u8;
    let mut puVar6: *mut u32;
    let mut in_DX: u16;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut paStack78: *mut Struct18;
    let mut local_42: [u8;6] = [0;6];
    let mut local_brush_handle: *mut u32;
    let mut iStack56: i16;
    let mut HStack54: HWND16;
    let mut HStack52: HWND16;
    let mut iStack50: i16;
    let mut local_30: RECT16;
    let mut iStack44: i16;
    let mut iStack42: i16;
    let mut local_rect: *mut RECT16;
    let mut BStack38: BOOL16;
    let mut local_24: HDC16;
    let mut local_paint_struct: PAINTSTRUCT16;

    uVar9    = (param_1 >> 0x10);
    iVar8    = param_1;
    uVar11   = (iVar8 + 0x4);
    local_24 = BeginPaint16(param_2, &local_paint_struct);
    uVar10   = (iVar8 + 0x4);
    BVar4    = IsIconic16(LAST_SEGMENT);
    if(BVar4 == 0x0)
    {
        uVar3              = (iVar8 + 0x1c);
        puVar6             = (uVar3 + 0x24);
        local_brush_handle = puVar6;
        pass1_1018_2e5e(param_3, puVar6, in_DX, *(iVar8 + 0x1c));
        local_30 = puVar6 & 0xffff | in_DX << 0x10;
        pass1_1008_3e54(
          CONCAT13((param_3 >> 0x8), CONCAT12(param_3, local_42)), 0x0, 0x35, 0xc);
        if((iVar8 + 0x14) != 0x0)
        {
            pass1_1008_5236(*(iVar8 + 0x14));
        }
        if(local_30 != 0x0)
        {
            uVar1     = (iVar8 + 0x14);
            uVar7     = (iVar8 + 0x16);
            paStack78 = str_var1(uVar7, uVar1);
            if((uVar7 | uVar1) != 0x0)
            {
                pass1_1008_5118(str_var1(uVar7, uVar1));
                fn_ptr_1000_17ce(paStack78, SEG_1000);
            }
            puVar5 = local_42;
            pass1_1008_8ce4(
              local_30, str_var1(param_3, puVar5), local_brush_handle, param_3);
            (iVar8 + 0x14) = puVar5;
            (iVar8 + 0x16) = uVar7;
        }
        unsafe{ppcVar2 = (*local_brush_handle + 0x4);}
        unsafe{(**ppcVar2)(SEG_1008, local_brush_handle, (local_brush_handle >> 0x10), 0x0, 0x0);}
        ppcVar2 = ((iVar8 + 0x18) + 0x94);
        unsafe{(**ppcVar2)(SEG_1008, (iVar8 + 0x18), 0x1);}
        HStack52 = GetDlgItem16(SEG_1008, 0x1797);
        if(HStack52 != 0x0)
        {
            ShowWindow16(LAST_SEGMENT, 0x1);
        }
    }
    else
    {
        if(globals.PTR_LOOP_1050_0010 == 0x0)
        {
            ppcVar2 = ((iVar8 + 0x1c) + 0x2c);
            unsafe{(**ppcVar2)(LAST_SEGMENT, (iVar8 + 0x1c), uVar10, uVar11);}
            BStack38 = BVar4;
            if(BVar4 != 0x0)
            {
                local_rect = GetStockObject16(LAST_SEGMENT);
                GetClientRect16(LAST_SEGMENT, &local_30);
                local_brush_handle = 0x0;
                iStack56           = (iStack44 - local_30.x) + -0x1;
                HStack54           = (iStack42 - local_30.y) - 0x1;
                HStack52           = HStack54;
                iStack50           = iStack56;
                FillRect16(
                  LAST_SEGMENT, local_rect, (HBRUSH16)&local_brush_handle);
                DrawIcon16(LAST_SEGMENT, BStack38, 0x2, 0x2);
            }
        }
    }
    BVar4 = EndPaint16(LAST_SEGMENT, &local_paint_struct);
    return BVar4;
}

pub fn unk_draw_op_1020_3da4(param_1: *mut Struct24, param_2: u32)

{
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut HVar5: HGDIOBJ16;
    let mut pHVar6: *mut HDC16;
    let mut in_DX: *mut u8;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut unaff_DI: i16;
    let mut uVar9: u16;
    let mut unaff_CS: u16;
    let mut unaff_SS: u16;
    let mut puVar10: *mut u16;
    let mut local_4: HDC16;
    let mut iVar9: *mut Struct24;
    let mut uVar8: *mut Struct24;

    get_sys_metrics_1020_7c1a(&param_1.field_0x0, param_2, unaff_CS);
    uVar9              = (param_1 >> 0x10);
    iVar8              = param_1;
    (iVar8 + 0x14)     = 0x0;
    param_1.field_0x0 = addr_table_1020_408a;//0x408a;
    (iVar8 + 0x2)      = SEG_1020;
    puVar10        = mixed_1010_20ba(globals.data_1050_0ed0, 0x6, unaff_SS, in_DX, unaff_DI);
    uVar7          = (puVar10 >> 0x10);
    (iVar8 + 0x14) = puVar10;
    (iVar8 + 0x16) = uVar7;
    ppcVar2        = ((iVar8 + 0x14) + 0x4);
    unsafe{(**ppcVar2)(SEG_1010, (iVar8 + 0x14), uVar7, 0x0, param_1);}
    local_4                      = GetDC16(SEG_1010);
    iVar4                        = SetMapMode16(LAST_SEGMENT, 0x1);
    (iVar8 + 0x1e)       = iVar4;
    HVar5                        = GetStockObject16(LAST_SEGMENT);
    HVar5                        = SelectObject16(LAST_SEGMENT, HVar5);
    (iVar8 + 0x18) = HVar5;
    HVar5                        = GetStockObject16(LAST_SEGMENT);
    HVar5                        = SelectObject16(LAST_SEGMENT, HVar5);
    (iVar8 + 0x1a) = HVar5;
    uVar3                        = (iVar8 + 0x14);
    puVar1                       = (uVar3 + 0x24);
    pHVar6                       = &local_4;
    unsafe{ppcVar2                      = (*puVar1 + 0x8);}
    unsafe{(**ppcVar2)(LAST_SEGMENT, puVar1, (puVar1 >> 0x10), pHVar6);}
    (iVar8 + 0x1c) = pHVar6;
    uVar3                     = (iVar8 + 0x14);
    (uVar3 + 0x4c)  = local_4;
    return;
}

pub fn win_ui_palette_op_1020_3e84(param_1: *mut Struct16)

{
    let mut unaff_SS: u16;
    param_1            = addr_table_1020_408a; // 0x408a;
    param_1.field_0x2 = SEG_1020;
    pass1_1010_1ea6(param_1.field_0x14, param_1, unaff_SS);
    SelectObject16(SEG_1010, param_1.field_0x18);
    SelectObject16(LAST_SEGMENT, param_1.field_0x1a_addr_offset);
    SelectPalette16(LAST_SEGMENT, 0x0, param_1.field_0x1c_addr_base);
    DeleteObject16(LAST_SEGMENT);
    SetMapMode16(LAST_SEGMENT, param_1.field_0x1e);
    param_1.field_0x0 = addr_table_1008_3aa0[4]; // 0x3ab0;
    param_1.field_0x2 = SEG_1008;
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a;
    param_1.field_0x2 = SEG_1008;
}


pub fn validate_rect_1020_3f12(param_1: u32, param_2: i16, param_3: HWND16)

{
    let mut local_a: RECT16;
    let mut u_stack6: u32;

    if(param_2 == 0x1)
    {
        (param_1 + 0x14) = 0x0;
        return;
    }
    if(param_2 != 0xd)
    {
        return;
    }
    local_a  = 0x8000e;
    u_stack6 = 0x1100116;
    InvalidateRect16(param_3, 0x0, &local_a);
    local_a  = 0xf10000;
    u_stack6 = 0x1220030;
    ValidateRect16(LAST_SEGMENT, &local_a);
    local_a  = 0xf100f5;
    u_stack6 = 0x1220127;
    ValidateRect16(LAST_SEGMENT, &local_a);
    return;
}


pub fn mixed_draw_op_1020_3fa0(param_1: u32, param_2: HWND16, param_3: u16)

{
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut iStack56: i16;
    let mut uStack54: u32;
    let mut local_32: u32;
    let mut iStack46: i16;
    let mut uStack44: u32;
    let mut puStack40: *mut u32;
    let mut local_24: u16;
    let mut local_22: PAINTSTRUCT16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar6 = (iVar4 + 0x4);
    BeginPaint16(param_2, &local_22);
    uVar3     = (iVar4 + 0x14);
    local_24  = (uVar3 + 0x4c);
    uVar3     = (iVar4 + 0x14);
    puStack40 = (uVar3 + 0x24);
    ppcVar2   = unsafe{(*puStack40 + 0x4)};
    unsafe{(**ppcVar2)(
      LAST_SEGMENT, puStack40, (puStack40 >> 0x10), 0x0, &local_24, param_3, uVar6);}
    uVar3    = (iVar4 + 0x14);
    iStack46 = (uVar3 + 0x44);
    uVar3    = (iVar4 + 0x14);
    uStack44 = *(uVar3 + 0x40);
    uVar1    = *(iVar4 + 0x14);
    pass1_1008_3e94((uVar1 & 0xffff0000 | (uVar1 + 0x3a)),
                    str_var1(param_3, &local_32),
                    str_var1(param_3, &local_32 + 0x2));
    uStack54 = uStack44;
    // for(iStack56 = 0x0; iStack56 < iStack46; iStack56 = iStack56 + 0x1)
    for iStack56 in 0..iStack46
    {
        draw_rect_1020_40ce(uStack54, local_32, (local_32 >> 0x10), param_3);
        uStack54 = uStack54 & 0xffff0000 | (uStack54 + 0x18);
    }
    EndPaint16(SEG_1008, &local_22);
    return;
}


pub fn pass1_1020_4064(param_1: *mut Struct16, param_2: u8) -> *mut Struct16

{
    win_ui_palette_op_1020_3e84(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, SEG_1000);
    }
    return param_1;
}

pub fn draw_rect_1020_40ce(param_1: u32, param_2: i16, param_3: i16, param_4: u16)

{
    let mut iVar1: i16;
    let mut HVar2: HGDIOBJ16;
    let mut handle: HPEN16;
    let mut local_6: i16;
    let mut local_4: i16;

    pass1_1008_3e94((param_1 & 0xffff0000 | (param_1 + 0x10)),
                    str_var1(param_4, &local_6),
                    str_var1(param_4, &local_4));
    pass1_1008_3e94(param_1, str_var1(param_4, &local_6), str_var1(param_4, &local_4));
    iVar1 = (param_1 + 0xa);
    Ellipse16(SEG_1008,
              iVar1 + local_6 + param_2,
              iVar1 + local_4 + param_3,
              (local_6 - (param_1 + 0xa)) + param_2,
              (local_4 - (param_1 + 0xa)) + param_3);
    if(((param_1 + 0xe) & 0x1) != 0x0)
    {
        HVar2 = GetStockObject16(LAST_SEGMENT);
        SelectObject16(LAST_SEGMENT, HVar2);
        handle = CreatePen16(LAST_SEGMENT, 0xf9, 0x100);
        SelectObject16(LAST_SEGMENT, handle);
        Rectangle16(LAST_SEGMENT,
                    local_6 + param_2 + 0x5,
                    local_4 + param_3 + 0x5,
                    local_6 + param_2 + -0x5,
                    local_4 + param_3 + -0x5);
        HVar2 = GetStockObject16(LAST_SEGMENT);
        SelectObject16(LAST_SEGMENT, HVar2);
        HVar2 = GetStockObject16(LAST_SEGMENT);
        SelectObject16(LAST_SEGMENT, HVar2);
        DeleteObject16(LAST_SEGMENT);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn unk_draw_op_1020_41c8(globals: &mut Globals,
                           param_1: &mut Struct20,
                           param_2: u16,
                           param_3: u16,
                          param_4: u16)

{
    let mut ppcVar1: *mut c_void;
    let mut cursor_handle_1: HCURSOR16;
    let mut puVar3: *mut u16;
    let mut extraout_DX: *mut u8;
    let mut puVar4: *mut u8;
    let mut uVar6: u16;
//    Struct64 *uVar5;
    let mut unaff_DI: i16;
//    u16       uVar7;
    let mut unaff_SS: u16;
    let mut puVar8: *mut u16;
    let mut hinst_var9: HINSTANCE16;
    let mut hinst_var10: HINSTANCE16;
    let mut puVar11: u16;

    unk_draw_op_1020_7f7a(globals, param_1, 0x8, str_var1(param_3, param_2));
//    uVar7              = (param_1 >> 0x10);
//    uVar5              = param_1;
    param_1.field_0xee  = 0x0;
    param_1.field_0xf0  = 0x0;
    param_1.field_0xf2  = 0x0;
    param_1.field_0xf4  = 0x1;
    param_1.field_0xf6  = 0x0;
    param_1.field_0xfa  = 0x0;
    param_1.field_0xfe  = 0x0;
    param_1.field_0x102 = 0x0;
    param_1.field_0x106 = 0x0;
    param_1.field_0x10a = 0x0;
    param_1.field_0x108 = 0x0;
    param_1.field_0x10c = 0x0;
    param_1.field_0x110 = 0x0;
    param_1.field_0x10e = 0x0;
    param_1.field_0x112 = 0x0;
    param_1.field_0x114 = 0x0;
    param_1.field_0x116 = 0x0;
    param_1.field_0x0 = addr_table_1020_623c;//0x623c;
    param_1.field_0x2   = SEG_1020;
    param_1.field_0xe2  = addr_table_1020_623c[39];//0x62d8;
    param_1.field_0xe4  = SEG_1020;
    puVar4             = extraout_DX;
    puVar11            = globals.hinst_1050_038c;
    cursor_handle_1
      = LoadCursor16(param_4, get_rsrc_string(0x019e)); // s__s__ld_1050_019c + 0x2
    param_1.field_0xf0 = cursor_handle_1;
    hinst_var10         = globals.hinst_1050_038c;
    cursor_handle_1   = LoadCursor16(LAST_SEGMENT,
                                   get_rsrc_string(0x019f)); // s__s__ld_1050_019c + 0x3
    param_1.field_0xf2 = cursor_handle_1;
    hinst_var9          = globals.hinst_1050_038c;
    globals.PTR_LOOP_1050_0398
      = LoadAccelerators16(LAST_SEGMENT, globals.s_OpAccel_1050_43e8);
    puVar8
      = mixed_1010_20ba(globals.data_1050_0ed0, 0x29, unaff_SS, puVar4, unaff_DI);
    param_1.field_0xfa         = puVar8;
    (param_1.field_0xfa + 0x2) = (puVar8 >> 0x10);
    if(param_1 == 0x0)
    {
        puVar3 = 0x0;
        uVar6  = 0x0;
    }
    else
    {
        puVar3 = &param_1.field_0xe2;
        uVar6  = uVar7;
    }
    ppcVar1 = (param_1.field_0xfa + 0x4);
    (ppcVar1)(SEG_1010, param_1.field_0xfa, 0x0, puVar3, uVar6,
              hinst_var9, hinst_var10, puVar11);
    param_1.field_0xe6 = param_1.field_0xfa;
}


pub fn destroy_cursor_1020_42f4(param_1: *mut u16, HMENparam_2: u16) {
    let mut iVar1: i16;
    let mut u_var2: u16;
    let mut h_cursor: HMENU16;

    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field_0x0 = addr_table_1020_623c;//0x623c;
    (iVar1 + 0x2) = SEG_1020;
    (iVar1 + 0xe2) = addr_table_1020_623c[39];//0x62d8;
    (iVar1 + 0xe4) = SEG_1020;
    h_cursor = param_2;
    if ((iVar1 + 0x106) != 0x0) {
        h_cursor =  LAST_SEGMENT;
        DestroyMenu16(param_2);
    }
    DestroyCursor16(h_cursor);
    DestroyCursor16(LAST_SEGMENT);
    pass1_1020_808e(param_1);
    return;
}

pub fn pass1_1020_2c72(param_1: u32, param_2: u16, param_3: u16)

{
    draw_op_1020_30be(*(param_1 + 0xf6), param_2, param_3);
    return;
}

pub fn destroy_icon_1020_2c88(param_1: u32, param_2: HICON16)

{
    let mut puVar1: *mut u32;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar6 = (iVar4 + 0xc2);
    DestroyIcon16(param_2);
    (iVar4 + 0xc2) = 0x0;
    (iVar4 + 0x8)  = 0x0;
    puVar1         = (iVar4 + 0xf6);
    u_var2         = (iVar4 + 0xf8);
    if((u_var2 | puVar1) != 0x0)
    {
        unsafe{ppcVar3 = *puVar1};
        unsafe{(**ppcVar3)(LAST_SEGMENT, puVar1, u_var2, 0x1, uVar6)};
    }
    (iVar4 + 0xf6) = 0x0;
    pass1_1010_1dda(*(iVar4 + 0xf2));
    (iVar4 + 0xf2) = 0x0;
    return;
}

pub fn load_draw_op_1020_2ede(param_1: *mut u16, param_2: u32, param_3: u16)

{
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut HVar3: HDC16;
    let mut iVar4: i16;
    let mut handle: HPEN16;
    let mut HVar5: HGDIOBJ16;
    let mut in_DX: *mut u8;
    let mut iVar6: i16;
    let mut unaff_DI: i16;
    let mut uVar7: u16;
    let mut unaff_SS: u16;
    let mut puVar8: *mut u16;
    let mut paVar9: *mut Struct76;
    let mut uVar10: u32;

    get_sys_metrics_1020_7c1a(param_1, param_2, param_3);
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    (iVar6 + 0x14) = 0x0;
    (iVar6 + 0x18) = 0x0;
    (iVar6 + 0x1a) = 0x0;
    (iVar6 + 0x1c) = 0x0;
    (iVar6 + 0x1e) = 0x0;
    (iVar6 + 0x20) = 0x0;
    param_1.field_0x0 = addr_table_1020_363c;//0x363c;
    (iVar6 + 0x2) = SEG_1020;
    puVar8
            = mixed_1010_20ba(globals.data_1050_0ed0, (param_2 + 0xfc), unaff_SS, in_DX, unaff_DI);
    (iVar6 + 0x14) = puVar8;
    (iVar6 + 0x16) = (puVar8 >> 0x10);
    uVar1 = (iVar6 + 0x14);
    ppcVar2 = ((iVar6 + 0x14) + 0x4);
    unsafe{(**ppcVar2)(SEG_1010, uVar1, (uVar1 >> 0x10), 0x0, param_1)};
    paVar9 =  pass1_1018_0a50(*(iVar6 + 0x14));
    uVar10 = pass1_1008_4772(paVar9);
    HVar3  = CreateDC16(SEG_1008, uVar10, (uVar10 >> 0x10), 0x0);
    (iVar6 + 0x18) = HVar3;
    iVar4                    = iVar6 + 0x18;
    ppcVar2                  = (paVar9 + 0x8);
    unsafe{(**ppcVar2)()};
    (iVar6 + 0x20) = iVar4;
    uVar1          = (iVar6 + 0x14);
    uVar1          = (uVar1 + 0x64);
    handle = CreatePen16(LAST_SEGMENT, uVar1, (uVar1 >> 0x10));
    (iVar6 + 0x1a)    = handle;
    HVar5                        = SelectObject16(LAST_SEGMENT, handle);
    (iVar6 + 0x1c) = HVar5;
    HVar5                        = GetStockObject16(LAST_SEGMENT);
    HVar5                        = SelectObject16(LAST_SEGMENT, HVar5);
    (iVar6 + 0x1e) = HVar5;
    return;
}

pub fn invalidate_rect_1020_3080(param_1: u32, param_2: i16, param_3: HWND16)

{
    if(param_2 == 0x1)
    {
        (param_1 + 0x14) = 0x0;
        return;
    }
    if((param_2 != 0x4) && (param_2 != 0x6))
    {
        return;
    }
    InvalidateRect16(param_3, 0x0, 0x0);
    return;
}

pub fn draw_op_1020_30be(param_1: u32, param_2: HWND16, param_3: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u32;
    let mut BVar3: BOOL16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut hwnd: HWND16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut local_3c: *mut u32;
    let mut iStack56: i16;
    let mut iStack54: i16;
    let mut iStack52: i16;
    let mut iStack50: i16;
    let mut local_30: RECT16;
    let mut iStack44: i16;
    let mut iStack42: i16;
    let mut pRStack40: *mut RECT16;
    let mut iStack38: i16;
    let mut local_24: HDC16;
    let mut local_22: PAINTSTRUCT16;

    uVar5    = (param_1 >> 0x10);
    iVar4    = param_1;
    uVar7    = (iVar4 + 0x4);
    local_24 = BeginPaint16(param_2, &local_22);
    uVar6    = (iVar4 + 0x4);
    hwnd     = LAST_SEGMENT;
    BVar3    = IsIconic16(LAST_SEGMENT);
    if(BVar3 == 0x0)
    {
        hwnd     = SEG_1018;
        local_3c = pass1_1018_0a50(*(iVar4 + 0x14));
        unsafe{ppcVar1  = (*local_3c + 0x8)};
        unsafe{(**ppcVar1)(
          SEG_1018, local_3c, (local_3c >> 0x10), &local_24, param_3, uVar6, uVar7)};
        u_var2 = (iVar4 + 0x14);
        if((u_var2 + 0x84) == 0x1)
        {
            unk_draw_op_1020_320e(param_1, local_24, param_3);
        }
        ppcVar1 = unsafe{(*local_3c + 0x4)};
        unsafe{(**ppcVar1)(SEG_1018, local_3c, (local_3c >> 0x10), 0x0, 0x0, 0xdc, param_3)};
        u_var2 = (iVar4 + 0x14);
        if((u_var2 + 0x84) != 0x1)
        {
            unk_draw_op_1020_320e(param_1, local_24, param_3);
        }
        draw_op_1020_3488(param_1);
        ppcVar1 = unsafe{(*local_3c + 0xc)};
        unsafe{(**ppcVar1)(SEG_1018, local_3c, (local_3c >> 0x10), &local_24, param_3)};
    }
    else
    {
        if(globals.PTR_LOOP_1050_0010 == 0x0)
        {
            ppcVar1  = ((iVar4 + 0x14) + 0x2c);
            unsafe{iStack38 = (**ppcVar1)(LAST_SEGMENT)};
            if(iStack38 != 0x0)
            {
                pRStack40 = GetStockObject16(LAST_SEGMENT);
                GetClientRect16(LAST_SEGMENT, &local_30);
                local_3c = 0x0;
                iStack56 = (iStack44 - local_30.x) + -0x1;
                iStack54 = (iStack42 - local_30.y) + -0x1;
                iStack52 = iStack54;
                iStack50 = iStack56;
                FillRect16(LAST_SEGMENT, pRStack40, (HBRUSH16)&local_3c);
                hwnd = LAST_SEGMENT;
                DrawIcon16(LAST_SEGMENT, iStack38, 0x2, 0x2);
            }
        }
    }
    EndPaint16(hwnd, &local_22);
    return;
}


pub fn unk_draw_op_1020_320e(param_1: u32, param_2: HDC16, param_3: u16)

{
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    DEVMODEA *init_data;
    let mut local_c: i16;
    let mut local_a: u32;
    let mut pHStack6: *mut HDC16;
    let mut local_4: HDC16;

    local_4 = param_2;
    uVar6   = (param_1 >> 0x10);
    iVar4   = param_1;
    uVar3   = (iVar4 + 0x14);
    if((uVar3 + 0x84) == 0x1)
    {
        uVar3     = (iVar4 + 0x14);
        uVar7     = (uVar3 >> 0x10);
        iVar5     = uVar3;
        puVar1    = *(iVar5 + 0x24);
        init_data = 0x0;
        uVar8   = pass1_1008_4772((puVar1 & 0xffff | (iVar5 + 0x26) << 0x10));
        local_4 = CreateDC16(SEG_1008, uVar8, (uVar8 >> 0x10), init_data);
        pHStack6 = &local_4;
        ppcVar2  = unsafe{(*puVar1 + 0x8)};
        unsafe{(**ppcVar2)(LAST_SEGMENT, puVar1, (puVar1 >> 0x10), pHStack6, param_3)};
    }
    pass1_1018_0d9a(
      *(iVar4 + 0x14), str_var1(param_3, &local_c), str_var1(param_3, &local_a));
    uVar3 = (iVar4 + 0x14);
    draw_op_1020_33c0(param_1, *(uVar3 + 0x6c), local_c, local_a, 0x1, local_4, SEG_1018);
    pass1_1018_1054(
      *(iVar4 + 0x14), str_var1(param_3, &local_c), str_var1(param_3, &local_a), param_3);
    uVar3 = (iVar4 + 0x14);
    draw_op_1020_33c0(param_1, *(uVar3 + 0x74), local_c, local_a, 0x2, local_4, SEG_1018);
    pass1_1018_1320(
      *(iVar4 + 0x14), str_var1(param_3, &local_c), str_var1(param_3, &local_a));
    uVar3 = (iVar4 + 0x14);
    draw_op_1020_33c0(param_1, *(uVar3 + 0x68), local_c, local_a, 0x1, local_4, SEG_1018);
    pass1_1018_15f6(
      *(iVar4 + 0x14), str_var1(param_3, &local_c), str_var1(param_3, &local_a));
    if(local_c != 0x0)
    {
        uVar3 = (iVar4 + 0x14);
        draw_op_1020_33c0(
          param_1, *(uVar3 + 0x70), local_c, local_a, 0x1, local_4, SEG_1018);
    }
    pass1_1018_108c(
      *(iVar4 + 0x14), str_var1(param_3, &local_c), str_var1(param_3, &local_a), param_3);
    if(local_c != 0x0)
    {
        uVar3 = (iVar4 + 0x14);
        draw_op_1020_33c0(
          param_1, *(uVar3 + 0x78), local_c, local_a, 0x0, local_4, SEG_1018);
    }
    uVar3 = (iVar4 + 0x14);
    if((uVar3 + 0x84) == 0x1)
    {
        SelectPalette16(SEG_1018, 0x0, pHStack6);
        DeleteObject16(LAST_SEGMENT);
        DeleteDC16(LAST_SEGMENT);
    }
    return;
}


pub fn draw_op_1020_33c0(param_1: u32,
                       param_2: u32,
                       param_3: i16,
                       param_4: u32,
                       param_5: i16,
                       param_6: u16,
                       param_7: u16)

{
    let mut pen_handle: HPEN16;
    let mut object_handle: HGDIOBJ16;
    let mut brush_handle: HBRUSH16;
    let mut obj_handle_2: HGDIOBJ16;
    let mut iVar1: i16;
    let mut u_var2: u16;
    let mut in_DX: u16;
    let mut uVar3: u16;
    let mut hdc: HDC16;
    let mut unaff_SS: u16;
    let mut uVar4: u16;
    let mut iStack20: i16;
    let mut puStack14: *mut u16;

    if(param_3 != 0x0)
    {
        pen_handle    = CreatePen16(param_7, param_2, (param_2 >> 0x10));
        object_handle = SelectObject16(LAST_SEGMENT, pen_handle);
        brush_handle  = CreateSolidBrush16(LAST_SEGMENT);
        hdc           = LAST_SEGMENT;
        obj_handle_2  = SelectObject16(LAST_SEGMENT, brush_handle);
        puStack14     = param_4;
        // for(iStack20 = 0x0; iStack20 < param_3; iStack20 = iStack20 + 0x1)
        for iStack20 in 0..param_3
        {
            uVar4 = (param_1 >> 0x10);
            iVar1 = param_3;
            pass1_1020_3540(param_1, uVar4, param_5, puStack14, in_DX, unaff_SS);
            if(param_5 < 0x1)
            {
                u_var2 = 0x3;
            }
            else
            {
                u_var2 = 0x4;
            }
            uVar3 = in_DX;
            draw_polygon_1020_3602(param_1, uVar4, str_var1(iVar1, u_var2), hdc);
            hdc = SEG_1000;
            fn_ptr_1000_17ce(str_var1(in_DX, iVar1), SEG_1000);
            puStack14 = (puStack14 & 0xffff0000 | (puStack14 + 0x6));
            in_DX     = uVar3;
        }
        SelectObject16(hdc, obj_handle_2);
        DeleteObject16(LAST_SEGMENT);
        SelectObject16(LAST_SEGMENT, object_handle);
        DeleteObject16(LAST_SEGMENT);
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn draw_op_1020_3488(param_1: u32)

{
    let mut uVar1: u16;
    let mut u_var2: u32;
    let mut uVar3: u32;
    let mut handle: HPEN16;
    let mut handle_00: HGDIOBJ16;
    let mut HVar4: HGDIOBJ16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut bottom: i16;
    let mut local_a: u32;
    let mut pu_stack6: *mut u16;

    uVar5     = (param_1 >> 0x10);
    u_var2    = *(param_1 + 0x14);
    pu_stack6 = (u_var2 & 0xffff0000 | (u_var2 + 0x36));
    pass1_1008_3e94(
      pu_stack6, str_var1(unaff_SS, &local_a), str_var1(unaff_SS, &local_a + 0x2));
    u_var2 = (local_a - 0x3U) << 0x10;
    if((local_a - 0x3U) < 0x0)
    {
        u_var2 = 0x0;
    }
    uVar1   = local_a - 0x3;
    local_a = uVar1;
    if(uVar1 < 0x0)
    {
        local_a = 0x0;
    }
    local_a   = u_var2 | local_a;
    uVar3     = (param_1 + 0x14);
    uVar3     = (uVar3 + 0x64);
    handle    = CreatePen16(SEG_1008, uVar3, (uVar3 >> 0x10));
    handle_00 = SelectObject16(LAST_SEGMENT, handle);
    HVar4     = GetStockObject16(LAST_SEGMENT);
    HVar4     = SelectObject16(LAST_SEGMENT, HVar4);
    bottom    = (local_a >> 0x10);
    Rectangle16(LAST_SEGMENT, local_a + 0x6, bottom + 0x6, local_a, bottom);
    SelectObject16(LAST_SEGMENT, handle_00);
    SelectObject16(LAST_SEGMENT, HVar4);
    DeleteObject16(LAST_SEGMENT);
    return;
}

pub fn draw_polygon_1020_3602(param_1: u16, param_2: u16, param_3: u32, param_4: HDC16)

{
    Polygon16(param_4, param_3, (param_3 >> 0x10));
    return;
}


pub fn pass1_1020_3bd6(param_1: u32, param_2: u16, param_3: u16, param_4: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut uVar4: u32;

    uVar3  = (param_1 >> 0x10);
    u_var2 = param_1;
    mixed_draw_op_1020_3fa0(*(u_var2 + 0xf6), param_3, param_4);
    if((u_var2 + 0x100) == 0x0)
    {
        (u_var2 + 0x100) = 0x1;
        uVar4            = (u_var2 + 0xfa);
        if((uVar4 + 0x56) == 0x0)
        {
            iVar1 = 0x5;
        }
        else
        {
            iVar1 = 0x8;
        }
        uVar4            = pass1_1038_af40(globals.ptr_1050_5b7c,
                                (u_var2 + 0x8),
                                iVar1,
                                param_2,
                                u_var2,
                                SEG_1038,
                                param_4);
        (u_var2 + 0x10e) = uVar4;
        (u_var2 + 0x110) = (uVar4 >> 0x10);
    }
    return;
}

pub fn pass1_1020_3d08(param_1: u16,
                     param_2: u16,
                     param_3: u16,
                     param_4: u16,
                     param_5: u16,
                     param_6: u16,
                     param_7: u16,
                     param_8: u16,
                     param_9: u16,
                     param_10: u8,
                     param_11: u8,
                     param_12: u8,
                     param_13: u8,
                     param_14: u8,
                     param_15: u32,
                     param_16: u16,
                     param_17: u16,
                     param_18: u16,
                     param_19: u16)

{
    let mut pcVar1: *mut c_char;
    let mut pbVar2: *mut u8;
    let mut bVar3: bool;
    let mut bVar4: bool;
    let mut ppcVar5: *mut *mut c_void;
    let mut puVar6: *mut u16;
    let mut uVar7: u32;
    let mut puVar8: *mut u32;
    let mut uVar9: u32;
    let mut bVar10: u8;
    let mut bVar12: u8;
    let mut iVar13: i16;
    let mut bVar18: u8;
    let mut cVar19: char;
    let mut HVar14: HDC16;
    let mut iVar15: i16;
    let mut HVar16: HGDIOBJ16;
    let mut puVar17: *mut u8;
    let mut u_var20: u16;
    let mut u_var21: u16;
    let mut bVar22: u8;
    let mut bVar23: u8;
    let mut pu_var24: *mut u8;
    let mut bVar25: u8;
    let mut pcVar26: *mut c_char;
    let mut pcVar27: *mut c_char;
    let mut u_var28: u16;
    let mut u_var29: u16;
    let mut bVar30: bool;
    let mut bVar31: bool;
    let mut puVar32: *mut u16;
    let mut pcStack4: *mut c_void;
    let mut bVar11: u8;

    uVar9    = str_var1(param_19, param_18);
    bVar22   = param_2 + (param_1 >> 0x8) + param_10;
    *param_6 = 0x3c;
    pu_var24
      = CONCAT11((param_2 >> 0x8) + '<' + ((param_3 + param_5) < 0x20), bVar22);
    // pcStack4 = switchD_1008: 1091 ::caseD_a7;
    iVar13   = 0x203d;
    pbVar2   = (param_3 + 0x203d);
    unsafe{*pbVar2  = *pbVar2 | bVar22};
    pbVar2   = (param_3 + 0x203d);
    unsafe{*pbVar2  = *pbVar2 & bVar22};
    pbVar2   = (param_3 + 0x203d);
    unsafe{*pbVar2  = *pbVar2 | bVar22};
    pbVar2   = (param_3 + 0x203d);
    unsafe{*pbVar2  = *pbVar2 | bVar22};
    pbVar2   = (param_3 + 0x203d);
    unsafe{*pbVar2  = *pbVar2 | bVar22};
    bVar10   = (param_6 + 0x2);
    bVar12   = 0x9 < (bVar10 & 0xf) | param_11;
    bVar11   = bVar10 + bVar12 * '\x06' & 0xf;
    pbVar2   = (param_3 + 0x203d);
    unsafe{*pbVar2  = *pbVar2 | bVar22};
    bVar10   = 0x9 < bVar11 | bVar12;
    u_var20 = CONCAT11((param_6 + 0x2 >> 0x8) + bVar12 + bVar10, bVar11 + bVar10 * '\x06')
            & 0xff0f;
    pcVar27 = CONCAT11(0x79, param_5 + -0x37);
    loop
    {
        pcVar26 = pcVar27;
        pbVar2  = (param_3 + iVar13);
        bVar23  = pu_var24;
        unsafe{*pbVar2 = *pbVar2 | bVar23};
        bVar12  = (u_var20 - 0x1);
        bVar3   = 0x9 < (bVar12 & 0xf);
        bVar22  = bVar3 | bVar10;
        pbVar2  = (param_3 + iVar13);
        unsafe{*pbVar2 = *pbVar2 | bVar23};
        bVar4   = 0x9 < (bVar12 + bVar22 * '\x06' & 0xf);
        bVar18  = (u_var20 - 0x1 >> 0x8) + bVar22 + (bVar4 | bVar22);
        pbVar2  = (param_3 + iVar13);
        unsafe{*pbVar2 = *pbVar2 | bVar23};
        u_var20 = 0x0;
        bVar30  = &pcStack4 < (param_3 + iVar13);
        pbVar2  = (param_3 + iVar13 + 0x896);
        bVar25  = param_3;
        bVar31  = unsafe{CARRY1(*pbVar2, bVar25) || CARRY1(*pbVar2 + bVar25, bVar30)};
        unsafe{*pbVar2 = *pbVar2 + bVar25 + bVar30};
        pbVar2  = (param_3 + iVar13 + 0x2038);
        unsafe{bVar12  = *pbVar2};
        unsafe{bVar11  = *pbVar2};
        unsafe{*pbVar2 = bVar11 + bVar25 + bVar31};
        pcVar1  = (param_4 + iVar13);
        unsafe{*pcVar1 = *pcVar1 + (pu_var24 >> 0x8)
                + (CARRY1(bVar12, bVar25) || CARRY1(bVar11 + bVar25, bVar31))};
        pcVar1  = (param_3 + iVar13 + -0x64);
        unsafe{*pcVar1 = *pcVar1 + bVar18 + '\x01'};
        pbVar2  = (param_3 + iVar13);
        unsafe { *pbVar2 = *pbVar2 | bVar23 };
        pcVar1  = pcVar26;
        pcVar27 = pcVar26 + 0x1;
        bVar30  = unsafe { *pcVar1 } != '\0';
        unsafe{
            if(-*pcVar1 < '\0')
        {
            pcVar1   = (param_4 + 0x37);
            *pcVar1  = *pcVar1 + bVar25 + bVar30;
            pbVar2   = (param_3 + iVar13);
            *pbVar2  = *pbVar2 | bVar23;
            iVar13   = iVar13 + -0x1;
            pbVar2   = (param_3 + iVar13);
            *pbVar2  = *pbVar2 | bVar23;
            pbVar2   = (param_3 + iVar13);
            *pbVar2  = *pbVar2 | bVar23;
            pu_var24 = pu_var24 + -0x1;
            pbVar2   = (param_3 + iVar13);
            bVar12   = pu_var24;
            *pbVar2  = *pbVar2 | bVar12;
            if(*pbVar2 == 0x0)
            {
                pbVar2  = (param_3 + iVar13);
                *pbVar2 = *pbVar2 & bVar12;
            // code_rSEG_10203d96:
                pbVar2  = (param_3 + iVar13);
                *pbVar2 = *pbVar2 | pu_var24;
                pbVar2  = (param_3 + iVar13);
                *pbVar2 = *pbVar2 & pu_var24;
                pcVar27 = pcVar26 + 0x2;
                u_var21 = pu_var24 & 0xff;
                pu_var24
                  = (u_var21
                     | ((pu_var24 >> 0x8) * '\x02' + (u_var20 < 0x20)) << 0x8);
                pbVar2   = (param_3 + iVar13 + 0x1);
                *pbVar2  = *pbVar2 & u_var21;
                param_4  = &stack0xfff6;
                param_16 = pcStack4;
                param_17 = (pcStack4 >> 0x10);
                uVar9    = param_15;
            // code_rSEG_10203db1:
                get_sys_metrics_1020_7c1a(str_var1(param_17, param_16), uVar9, param_8);
                puVar6          = (param_4 + 0x6);
                u_var28         = (puVar6 >> 0x10);
                (puVar6 + 0x14) = 0x0;
                *puVar6         = addr_table_1020_408a;//0x408a;
                (puVar6 + 0x2)  = SEG_1020;
                puVar32
                  = mixed_1010_20ba(globals.data_1050_0ed0, 0x6, param_9, pu_var24, pcVar27);
                u_var28         = (puVar32 >> 0x10);
                uVar7           = (param_4 + 0x6);
                u_var29         = (uVar7 >> 0x10);
                iVar13          = uVar7;
                (iVar13 + 0x14) = puVar32;
                (iVar13 + 0x16) = u_var28;
                ppcVar5         = ((iVar13 + 0x14) + 0x4);
                (**ppcVar5)(SEG_1010, (iVar13 + 0x14), u_var28, 0x0, iVar13, u_var29);
                HVar14                     = GetDC16(SEG_1010);
                (param_4 + -0x2) = HVar14;
                iVar15                     = SetMapMode16(LAST_SEGMENT, 0x1);
                uVar7                      = (param_4 + 0x6);
                (uVar7 + 0x1e)     = iVar15;
                u_var28                    = (param_4 + -0x2);
                HVar16                     = GetStockObject16(LAST_SEGMENT);
                HVar16                     = SelectObject16(LAST_SEGMENT, HVar16);
                uVar7                      = (param_4 + 0x6);
                (uVar7 + 0x18) = HVar16;
                u_var29                      = (param_4 + -0x2);
                HVar16                       = GetStockObject16(LAST_SEGMENT);
                HVar16 = SelectObject16(LAST_SEGMENT, HVar16);
                uVar7  = (param_4 + 0x6);
                (uVar7 + 0x1a) = HVar16;
                uVar7                        = (param_4 + 0x6);
                uVar7                        = (uVar7 + 0x14);
                (param_4 + -0x6)             = (uVar7 + 0x24);
                puVar17                      = (param_4 + -0x2);
                puVar8                       = (param_4 + -0x6);
                ppcVar5                      = (*puVar8 + 0x8);
                (**ppcVar5)(LAST_SEGMENT,
                            puVar8,
                            (puVar8 >> 0x10),
                            puVar17,
                            param_9,
                            u_var29,
                            u_var28);
                uVar7             = (param_4 + 0x6);
                u_var29           = (uVar7 >> 0x10);
                iVar13            = uVar7;
                (iVar13 + 0x1c)   = puVar17;
                u_var28           = (param_4 + -0x2);
                (param_4 + -0x14) = u_var28;
                uVar7             = (iVar13 + 0x14);
                (uVar7 + 0x4c)    = u_var28;
                return;
            }
            pbVar2  = (param_3 + iVar13);
            *pbVar2 = *pbVar2 & bVar12;
            pcVar1  = (param_4 + iVar13 + 0x20);
            bVar11  = param_1 & 0x1f;
            cVar19  = *pcVar1;
            *pcVar1 = *pcVar1 >> bVar11;
            pcVar1  = (param_4 + iVar13 + 0x6a);
            *pcVar1 = *pcVar1 + param_1
                    + ((param_1 & 0x1f) != 0x0 && (cVar19 >> bVar11 - 0x1 & 0x1U) != 0x0);
            pbVar2  = (param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar12;
            param_8 = SEG_1020;
            u_var20 = (param_3 + iVar13) * 0x10;
            pbVar2  = (pcVar27 + param_4 + 0x8);
            bVar12  = (u_var20 >> 0x8);
            u_var21 = u_var20 & 0xff | (bVar12 + *pbVar2) << 0x8;
            pcVar1  = (param_4 + iVar13 + 0x37);
            *pcVar1 = *pcVar1 + (param_3 >> 0x8) + CARRY1(bVar12, *pbVar2);
        }
        else
        {
            pcVar1  = (param_4 + iVar13);
            *pcVar1 = *pcVar1 + bVar30;
            u_var21 = (param_3 + iVar13) * 0x10;
            if((POPCOUNT(*pcVar1) & 0x1U) == 0x0){
                // goto code_rSEG_10203db1;}
             }
    }}
        pbVar2   = (param_3 + iVar13);
        bVar11   = pu_var24;
        unsafe{*pbVar2  = *pbVar2 | bVar11};
        pbVar2   = (param_3 + iVar13);
        unsafe{*pbVar2  = *pbVar2 | bVar11};
        param_16 = (param_14 & 0x1) * 0x4000 | (param_13 & 0x1) * 0x200
                 | (param_12 & 0x1) * 0x100 | (unsafe{*pbVar2 < '\0'}) * 0x80
                 | (unsafe{*pbVar2 == 0x0}) * 0x40 | (bVar4 | bVar3 | bVar10 & 0x1) * 0x10
                 | ((POPCOUNT(unsafe { *pbVar2 }) & 0x1U) == 0x0) * 0x4;
        pbVar2  = (param_3 + iVar13);
        unsafe { *pbVar2 = *pbVar2 | bVar11 };
//        bVar12  = in(0x79);
        pbVar2  = (param_3 + iVar13);
        unsafe { *pbVar2 = *pbVar2 & bVar11 };
        if(-0x1 < unsafe { *pbVar2 })
        {
            uVar9 = str_var1(param_19, param_18);
            if((bVar18 | (param_4 - 0x1)) == 0x0)
            {
                param_16 = param_7;
                uVar9    = str_var1(param_19, param_18);
            }
            // goto code_rSEG_10203db1;
        }
        pbVar2         = (param_4 + 0x89c);
        bVar30         = CARRY1(unsafe { *pbVar2 }, bVar12);
        unsafe { *pbVar2        = *pbVar2 + bVar12 };
        bVar23         = bVar18 + bVar12;
        cVar19         = bVar23 + bVar30;
        u_var20        = CONCAT11(cVar19, bVar12);
        pcStack4._0_3_ = CONCAT21(
          (param_14 & 0x1) * 0x4000
            | (SCARRY1(bVar18, bVar12) != SCARRY1(bVar23, bVar30)) * 0x800
            | (param_13 & 0x1) * 0x200 | (param_12 & 0x1) * 0x100 | (cVar19 < '\0') * 0x80
            | (cVar19 == '\0') * 0x40 | (bVar4 | bVar3 | bVar10 & 0x1) * 0x10
            | ((POPCOUNT(cVar19) & 0x1U) == 0x0) * 0x4
            | (CARRY1(bVar18, bVar12) || CARRY1(bVar23, bVar30)),
          pcStack4._0_1_);
        pcStack4 = (fn_ptr_1)(pcStack4 & 0xff000000 | pcStack4);
        pbVar2   = (param_3 + iVar13);
        unsafe { *pbVar2  = *pbVar2 | bVar11 };
        param_1  = u_var21 - 0x1;
        bVar10   = bVar4 | bVar22;
        if(param_1 == 0x0 || unsafe { *pbVar2 } == 0x0){
            // goto code_rSEG_10203d96;
        }
    }
    // while(true);
}

pub fn invalidate_rect_1020_1fb2(param_1: u32, param_2: i16, param_3: HWND16)

{
    let mut local_16: u16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut local_e: RECT16;
    let mut iStack10: i16;
    let mut u_stack6: u16;
    let mut uStack4: u16;

    if(param_2 == 0x1)
    {
        (param_1 + 0x6) = 0x0;
        return;
    }
    if(param_2 != 0xd)
    {
        return;
    }
    GetWindowRect16(param_3, &local_e);
    local_16 = 0x0;
    u_stack6 = 0x46;
    uStack20 = 0x46;
    iStack18 = iStack10 - local_e.x;
    uStack4  = 0x5f;
    uStack16 = 0x5f;
    InvalidateRect16(LAST_SEGMENT, 0x0, &local_16);
    return;
}

pub fn unk_draw_op_1020_2020(param_1: u32, param_2: HWND16, param_3: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u32;
    let mut puVar3: *mut u32;
    let mut uVar4: u16;
    let mut pHVar5: *mut HDC16;
    let mut iVar6: i16;
    let mut HVar7: HPEN16;
    let mut HVar8: HGDIOBJ16;
    let mut HVar9: HBRUSH16;
    let mut puVar10: *mut u8;
    let mut extraout_DX: u16;
    let mut uVar11: u16;
    let mut iVar12: i16;
    let mut iVar13: i16;
    let mut puVar14: *mut u8;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut uVar17: u32;
    let mut pi_var18: *mut i16;
    let mut iVar19: i16;
    let mut u_var20: u8;
    let mut u_var21: u8;
    let mut local_38: [u8;6] = [0;6];
    let mut local_32: u16;
    let mut uStack48: u16;
    let mut uStack46: u32;
    let mut uStack42: u16;
    let mut puStack40: *mut u32;
    let mut local_24: HDC16;
    let mut local_22: PAINTSTRUCT16;

    puVar14   = &stack0xfffe;
    uVar15    = (param_1 >> 0x10);
    iVar12    = param_1;
    uVar16    = (iVar12 + 0x4);
    local_24  = BeginPaint16(param_2, &local_22);
    puStack40 = pass1_1010_4c2c(*(iVar12 + 0x6));
    pHVar5    = &local_24;
    ppcVar1   = (unsafe { *puStack40 } + 0x8);
    (unsafe { **ppcVar1 })(SEG_1010, puStack40, (puStack40 >> 0x10), pHVar5, param_3, uVar16);
    (iVar12 + 0x10) = pHVar5;
    u_var2                     = (iVar12 + 0x6);
    uStack42                   = (u_var2 + 0x30);
    u_var2                     = (iVar12 + 0x6);
    uStack46                   = (u_var2 + 0x12);
    uStack48                   = 0x14;
    local_32                   = 0x0;
    pass1_1008_3e38(str_var1(param_3, local_38));
    while((puVar14 + -0x38) < (puVar14 + -0x28))
    {
        iVar12            = (puVar14 + -0x38) * 0x4;
        u_var2            = (puVar14 + -0x2c);
        uVar17            = pass1_1008_4772((iVar12 + u_var2));
        puVar10           = (uVar17 >> 0x10);
        (puVar14 + -0x44) = uVar17;
        (puVar14 + -0x42) = puVar10;
        u_var2            = (puVar14 + 0x6);
        pass1_1020_2286(u_var2,
                        (u_var2 >> 0x10),
                        CONCAT13((param_3 >> 0x8), CONCAT12(param_3, puVar14 + -0x30)),
                        (uVar17 + 0x8));
        u_var2 = (puVar14 + -0x30);
        pass1_1008_3e76(
          str_var1(param_3, puVar14 + -0x36), 0x0, u_var2, (u_var2 >> 0x10));
        u_var2 = (puVar14 + -0x2c);
        pass1_1008_4480(*(puVar14 + -0x26),
                        str_var1(param_3, puVar14 + -0x36),
                        (u_var2 + iVar12),
                        param_3);
        iVar12  = (puVar14 + -0x38);
        u_var2  = (puVar14 + -0x30);
        uVar15  = u_var2;
        u_var20 = (u_var2 >> 0x10);
        u_var21 = (u_var2 >> 0x18);
        u_var2  = (puVar14 + -0x44);
        uVar16  = (u_var2 >> 0x10);
        iVar13  = u_var2;
        iVar6   = (iVar13 + 0x4) + (puVar14 + -0x2e);
        iVar13  = (iVar13 + 0x8) + (puVar14 + -0x30);
        u_var2  = (puVar14 + 0x6);
        u_var2  = (u_var2 + 0x6);
        iVar19  = u_var2;
        uVar16  = (u_var2 >> 0x10);
        if((iVar19 + 0x1a) == 0x0)
        {
            uVar4 = (iVar19 + 0x30) << 0x3;
            mem_op_1000_179c(uVar4, puVar10, 0);
            (iVar19 + 0x1a) = uVar4;
            (iVar19 + 0x1c) = puVar10;
        }
        u_var2                  = (iVar19 + 0x1a);
        iVar12                  = iVar12 * 0x8;
        (u_var2 + iVar12)       = CONCAT11(u_var21, u_var20);
        u_var2                  = (iVar19 + 0x1a);
        (u_var2 + iVar12 + 0x2) = uVar15;
        u_var2                  = (iVar19 + 0x1a);
        (u_var2 + iVar12 + 0x4) = iVar6;
        u_var2                  = (iVar19 + 0x1a);
        (u_var2 + iVar12 + 0x6) = iVar13;
        u_var2                  = (puVar14 + -0x44);
        pi_var18                = (puVar14 + -0x2e);
        unsafe { *pi_var18
          = *pi_var18 + (-((puVar14 + -0x38) == 0x0) & 0x5) + 0x14 + (u_var2 + 0x4) };
        pi_var18  = (puVar14 + -0x38);
        unsafe { *pi_var18 = *pi_var18 + 0x1 };
    }
    puVar3  = (puVar14 + -0x26);
    ppcVar1 = (unsafe { *puVar3 } + 0x4);
    (unsafe { **ppcVar1 })(SEG_1008, puVar3, (puVar3 >> 0x10), 0x0, 0x0, puVar14 + -0x22, param_3);
    uVar11                          = extraout_DX;
    HVar7                           = CreatePen16(SEG_1008, 0x25, 0x100);
    (puVar14 + -0x3a)    = HVar7;
    HVar8                           = SelectObject16(LAST_SEGMENT, HVar7);
    (puVar14 + -0x3c) = HVar8;
    HVar9                           = CreateSolidBrush16(LAST_SEGMENT);
    (puVar14 + -0x3e)  = HVar9;
    HVar8                           = SelectObject16(LAST_SEGMENT, HVar9);
    (puVar14 + -0x40) = HVar8;
    draw_line_1020_229c(*(puVar14 + 0x6), LAST_SEGMENT);
    u_var2 = (puVar14 + 0x6);
    pass1_1010_4df0(*(u_var2 + 0x6), uVar11, param_3);
    if(HVar8 == 0x0)
    {
        SelectObject16(SEG_1010, (puVar14 + -0x3c));
        DeleteObject16(LAST_SEGMENT);
        SelectObject16(LAST_SEGMENT, (puVar14 + -0x40));
        DeleteObject16(LAST_SEGMENT);
        HVar9                          = CreateSolidBrush16(LAST_SEGMENT);
        (puVar14 + -0x3e) = HVar9;
        HVar7                          = CreatePen16(LAST_SEGMENT, 0xff, 0x0);
        (puVar14 + -0x3a)   = HVar7;
        SelectObject16(LAST_SEGMENT, (puVar14 + -0x3e));
        SelectObject16(LAST_SEGMENT, (puVar14 + -0x3a));
    }
    u_var2   = (puVar14 + 0x6);
    pi_var18 = pass1_1010_4dc8(*(u_var2 + 0x6));
    uVar15   = (pi_var18 >> 0x10);
    uVar16   = SUB42(pi_var18, 0x0);
    pass1_1020_239c(*(puVar14 + 0x6), pi_var18, param_3);
    u_var2 = (puVar14 + 0x6);
    u_var2 = (u_var2 + 0x6);
    if((u_var2 + 0x2c) != 0x0)
    {
        pass1_1020_2488(*(puVar14 + 0x6), uVar16, uVar15);
    }
    u_var2 = (puVar14 + 0x6);
    SelectPalette16(SEG_1010, 0x0, (u_var2 + 0x10));
    DeleteObject16(LAST_SEGMENT);
    SelectObject16(LAST_SEGMENT, (puVar14 + -0x3c));
    DeleteObject16(LAST_SEGMENT);
    SelectObject16(LAST_SEGMENT, (puVar14 + -0x40));
    DeleteObject16(LAST_SEGMENT);
    EndPaint16(LAST_SEGMENT, (puVar14 + -0x20));
    return;
}

pub fn draw_line_1020_229c(param_1: u32, param_2: HDC16)

{
    let mut iVar1: i16;
    let mut pIVar2: *mut u16;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut piVar6: *mut i16;
    let mut uVar7: u16;
    let mut iStack10: i16;

    uVar7  = (param_1 >> 0x10);
    uVar3  = (param_1 + 0x6);
    iVar1  = (uVar3 + 0x30);
    uVar3  = (param_1 + 0x6);
    pIVar2 = (uVar3 + 0x1a);
    MoveTo16(param_2, 0x5, unsafe { *pIVar2 });
    uVar7 = (pIVar2 >> 0x10);
    iVar5 = pIVar2;
    LineTo16(LAST_SEGMENT, 0x5, (iVar5 + iVar1 * 0x8 + -0x4));
    // for(iStack10 = 0x0; iStack10 < iVar1; iStack10 = iStack10 + 0x1)
    for iStack10 in 0..iVar1
    {
        piVar6 = (iStack10 * 0x8 + iVar5);
        iVar4  = (piVar6[0x2] - unsafe { *piVar6 } >> 0x1) + unsafe { *piVar6 };
        MoveTo16(LAST_SEGMENT, 0x5, iVar4);
        LineTo16(LAST_SEGMENT, 0xa, iVar4);
    }
    MoveTo16(LAST_SEGMENT, 0x5f, unsafe { *pIVar2 });
    LineTo16(LAST_SEGMENT, 0x5f, (iVar5 + iVar1 * 0x8 + -0x4));
    // for(iStack10 = 0x0; iStack10 < iVar1; iStack10 = iStack10 + 0x1)
    for iStack10 in 0..iVar1
    {
        piVar6 = (iStack10 * 0x8 + iVar5);
        iVar4  = (piVar6[0x2] - unsafe { *piVar6 } >> 0x1) + unsafe { *piVar6 };
        MoveTo16(LAST_SEGMENT, 0x5f, iVar4);
        LineTo16(LAST_SEGMENT, 0x5a, iVar4);
    }
    return;
}

pub fn pass1_1020_239c(param_1: u32, param_2: *mut i16, param_3: u16)

{
    let mut puVar1: *mut u16;
    let mut u_var2: u32;
    let mut uVar3: u16;
    let mut local_a: [u8;6] = [0;6];
    let mut uStack4: u16;

    if(param_2 != 0x0)
    {
        uStack4 = ((param_2 + 0x4) - unsafe { *param_2 } >> 0x1) + unsafe { *param_2 };
        puVar1  = pass1_1008_3e54(str_var1(param_3, local_a), 0x0, 0x57, uStack4);
        uVar3   = (param_1 >> 0x10);
        u_var2  = pass1_1020_23f2(
          param_1, uVar3, str_var1(param_3, local_a), (puVar1 >> 0x10), param_3);
        draw_polygon_1020_2474(param_1, uVar3, str_var1(u_var2, 0x3), SEG_1008);
    }
    return;
}

pub fn draw_polygon_1020_2474(param_1: u16, param_2: u16, param_3: u32, param_4: HDC16)

{
    Polygon16(param_4, param_3, (param_3 >> 0x10));
    return;
}

pub fn struct_1020_2524(param_1: &mut Struct20, param_2: u16, param_3: u16, param_4: u16)

{
    let mut extraout_DX: *mut u8;
    let mut uVar1: u16;
    let mut iVar2: *mut Struct20;
    let mut unaff_DI: i16;
    let mut u_var2: u16;
    let mut puVar3: *mut u16;

    unk_draw_op_1020_7f7a(NULL, param_1, 0x7, str_var1(param_3, param_2));
    u_var2                     = (param_1 >> 0x10);
    iVar2                      = param_1;
    iVar2[0x1].field_0xc      = 0x0;
    iVar2[0x1].field_0x10      = 0x0;
    param_1.field_0x0         = addr_table_1020_270c;//0x270c;
    iVar2.field_0x2           = SEG_1020;
    ((iVar2 + 0x1)).field_0x0 = addr_table_1020_270c[39];//0x27a8;
    iVar2[0x1].field_0x2       = SEG_1020;
    puVar3 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2a, param_4, extraout_DX, unaff_DI);
    uVar1  = (puVar3 >> 0x10);
    iVar2[0x1].field_0x10         = puVar3;
    (iVar2[0x1].field_0x10 + 0x2) = uVar1;
    iVar2[0x1].field_0x4          = &iVar2[0x1].field_0x10;
    (iVar2[0x1].field_0x4 + 0x2)  = uVar1;
    return;
}

pub fn pass1_1020_27b0(param_1: *mut Struct664,
                     param_2: u16,
                     param_3: u16,
                     param_4: i16,
                    param_5: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u32;
    let mut iVar3: i16;
    let mut extraout_DX: *mut u8;
    let mut uVar4: u16;
    let mut puVar5: *mut u16;

    set_struct_op_1020_921c(str_var1(param_2, param_1), param_3);
    param_1.field_0x14       = 0x0;
    param_1 =  addr_table_1020_288e;//0x288e;
    param_1.field_0x2         = SEG_1020;
    puVar5 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2a, param_5, extraout_DX, param_4);
    uVar4  = (puVar5 >> 0x10);
    param_1.field_0x14 = puVar5;
    param_1.field_0x16 = uVar4;
    param_1.field_0x6  = param_1.field_0x14;
    param_1.field_0x8  = uVar4;
    u_var2              = &param_1.field_0x14;
    iVar3               = &param_1.field_0xa;
    ppcVar1             = ((u_var2 + 0xa) + 0x8);
    (unsafe { **ppcVar1 })();
    param_1.field_0x12 = iVar3;
    draw_op_1020_9364(str_var1(param_2, param_1), SEG_1010, param_5);
    return;
}

pub fn pass1_1020_2838(param_1: *mut u16, param_2: u16) {
    let mut iVar1: *mut Struct584;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = addr_table_1020_288e;//0x288e;
    iVar1.field_0x2 = SEG_1020;
    if (iVar1.field_0x14 != 0x0) {
        param_2 = SEG_1010;
        pass1_1010_1dda(iVar1.field_0x14);
    }
    palette_op_1020_92c4(param_1, param_2);
}