#include <structs/structs_11.h>
#include <types.h>
#include <structs/structs_23.h>
#include <fn_ptr_defs.h>
#include <globals.h>
#include <structs/structs_40.h>
#include "structs/structs_45.h"
#include "unk/unk_17.h"

void pass1_1040_d1bc(Globals* globals, Struct18 *param_1)

{
    u32  *pu32_var_a;
    u16          u16_var_b;
    fn_ptr_4     fn_ptr_a;
    astruct_513 *iVar4;
    u16          uVar4;

    //uVar4              = (param_1 >> 0x10);
    //iVar4              = (astruct_513 *)param_1;
    param_1->field_0x0 = 0xd8c4;
    param_1->field_0x2 = &globals->PTR_LOOP_1050_1040;
    pass1_1038_b6e0(globals->_PTR_LOOP_1050_5b7c, param_1->field_0x6);
    pu32_var_a = param_1->field_0x9c;
    u16_var_b  = param_1->field_0x9e;
    if((u16_var_b | (u32)pu32_var_a) != 0x0)
    {
        fn_ptr_a = *pu32_var_a;
        (fn_ptr_a)(&globals->PTR_LOOP_1050_1038, pu32_var_a, u16_var_b, 0x1);
    }
    unk_draw_op_1040_b0f8(param_1);
    return;
}

void pass1_1040_ca74(Globals* globals, Struct18 *param_1)

{
    u16 u16_var_1;

    //uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0xd07c;
    (param_1->field_0x2)    = &globals->PTR_LOOP_1050_1040;
    pass1_1038_b6e0(globals->_PTR_LOOP_1050_5b7c, (param_1->field_0x6));
    globals->PTR_LOOP_1050_5f10 = 0x0;
    unk_draw_op_1040_b0f8(param_1);
    return;
}


void pass1_1040_c94a(Globals* globals, Struct380* param_1, u16 param_2, u8 *param_3, i16 param_4, u16 param_5, u16 param_6)

{
    u16        uVar1;
    u32 uVar2;
    u16        uVar3;
    u16        uVar4;
    u16       *pu16_var5;

    if((param_1 + 0x48) != 0x0)
    {
        pu16_var5  = mixed_1010_20ba(globals->_PTR_LOOP_1050_0ed0, 0x3, param_6, param_3, param_4);
        //uVar4   = (puVar5 >> 0x10);
        uVar2   = (param_1 + 0x42);
        uVar1   = (uVar2 + 0x12);
        param_5 = 0x1010;
        uVar3   = uVar1;
        pass1_1010_a5ca(pu16_var5, uVar4, uVar1, uVar1, uVar4);
        if(uVar3 == 0xffff)
        {
            (param_1 + 0x3c) = 0xf9;
        }
        else
        {
            if(uVar3 == 0x0)
            {
                (param_1 + 0x3c) = 0x25;
                if((uVar1 == 0x5b) || (uVar1 == 0x9))
                {
                    (param_1 + 0x3c) = 0xfe;
                }
            }
            else
            {
                (param_1 + 0x3c) = 0xfb;
            }
        }
    }
    draw_text_1040_94fc((astruct_37 *)CONCAT22(param_2, param_1), param_5);
    return;
}


void palette_op_1040_c886(u32 param_1, u8 param_2, u16 param_3, HDC16 param_4)

{
    u16         uVar1;
    code      **ppcVar2;
    i16         iVar3;
    u16         uVar4;
    u16         uVar5;
    u32 *puStack8;
    HPALETTE16  HStack4;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if(((iVar3 + 0xa) | (iVar3 + 0x8)) != 0x0)
    {
        HStack4 = 0x0;
        if((iVar3 + 0x46) == 0x0)
        {
            uVar5   = (_PTR_LOOP_1050_4230 >> 0x10);
            uVar1   = (_PTR_LOOP_1050_4230 + 0x10);
            param_4 = 0x1008;
            HStack4 = palette_op_1008_4e08((astruct_13 *)CONCAT22(uVar1, (_PTR_LOOP_1050_4230 + 0xe)), &param_3, uVar1, 0x1008);
        }
        puStack8 = (iVar3 + 0x8);
        uVar5    = (iVar3 + 0xa);
        if((((iVar3 + 0xe) | (iVar3 + 0xc)) != 0x0) && ((param_2 & 0x1) != 0x0))
        {
            puStack8 = (iVar3 + 0xc);
            uVar5    = (iVar3 + 0xe);
        }
        if(((iVar3 + 0x10) != 0x0) && ((param_2 & 0x4) != 0x0))
        {
            puStack8 = (iVar3 + 0x10);
            uVar5    = (iVar3 + 0x12);
        }
        ppcVar2 = (*puStack8 + 0x4);
        (**ppcVar2)(param_4, puStack8, uVar5, (iVar3 + 0x28), (iVar3 + 0x26), &param_3);
        if((iVar3 + 0x46) == 0x0)
        {
            SelectPalette16(param_4, 0x0, HStack4);
            DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        }
    }
    return;
}


void draw_op_1040_c74c(u32 *param_1, u32 param_2, u16 param_3)

{
    u16        uVar1;
    code     **ppcVar2;
    u32        uVar3;
    HPALETTE16 b_force_background;
    HGDIOBJ16  HVar4;
    HPEN16     handle;
    HGDIOBJ16  handle_00;
    i16        iVar5;
    u16        uVar6;

    uVar6              = (_PTR_LOOP_1050_4230 >> 0x10);
    uVar1              = (_PTR_LOOP_1050_4230 + 0x10);
    b_force_background = palette_op_1008_4e08((astruct_13 *)CONCAT22(uVar1, (_PTR_LOOP_1050_4230 + 0xe)), &param_2 + 0x2, uVar1, 0x1008);
    uVar6              = (param_1 >> 0x10);
    iVar5              = param_1;
    (iVar5 + 0x46)     = 0x1;
    HVar4              = GetStockObject16(0x1008);
    handle             = CreatePen16((i1616)s_tile2_bmp_1050_1538, 0x2, 0x100);
    HVar4              = SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar4);
    handle_00          = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
    Rectangle16((HDC16)s_tile2_bmp_1050_1538, (iVar5 + 0x24), (iVar5 + 0x22), 0x0, 0x0);
    MoveTo16((HDC16)s_tile2_bmp_1050_1538, 0x0, (iVar5 + 0x36) * 0x2 + (iVar5 + 0x2a));
    LineTo16((HDC16)s_tile2_bmp_1050_1538, (iVar5 + 0x24), (iVar5 + 0x36) * 0x2 + (iVar5 + 0x2a));
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, HVar4);
    HVar4 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_00);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    uVar3   = *param_1;
    ppcVar2 = (uVar3 + 0x10);
    (**ppcVar2)(s_tile2_bmp_1050_1538, param_1, param_2, HVar4, param_2._2_2_);
    ppcVar2 = (uVar3 + 0x14);
    (**ppcVar2)(s_tile2_bmp_1050_1538, param_1, param_2._2_2_);
    (iVar5 + 0x46) = 0x0;
    SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    return;
}


void unk_draw_op_1040_c226(u32 param_1, HWND16 param_2, u16 param_3)

{
    u32    uVar1;
    HPEN16        handle;
    HGDIOBJ16     handle_00;
    u16           uVar2;
    RECT16        local_32;
    i16           iStack46;
    i16           iStack44;
    u16           uStack42;
    i16           iStack40;
    RECT16       *pRStack38;
    HDC16         HStack36;
    PAi16STRUCT16 local_22;

    uVar2     = (param_1 >> 0x10);
    HStack36  = BeginPai1616(param_2, &local_22);
    pRStack38 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    GetClientRect16((HWND16)s_tile2_bmp_1050_1538, &local_32);
    uVar1      = (param_1 + 0x6);
    iStack40   = (uVar1 + 0x1a);
    uVar1      = (param_1 + 0x6);
    uStack42   = (uVar1 + 0x1c);
    local_32.y = local_32.y + 0x2;
    local_32.x = iStack40 + -0xa;
    iStack46   = iStack46 + -0x2;
    iStack44   = iStack44 + -0x2;
    FrameRect16((HDC16)s_tile2_bmp_1050_1538, pRStack38, (HBRUSH16)&local_32);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    handle    = CreatePen16((i1616)s_tile2_bmp_1050_1538, -0x7f80, 0x0);
    handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
    draw_line_1040_c302(param_1, s_tile2_bmp_1050_1538);
    draw_op_1040_c38e(param_1);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_00);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPai1616((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}


void draw_line_1040_c302(u32 param_1, HDC16 param_2)

{
    u32        uVar1;
    u32        uVar2;
    u32 uVar3;
    u16        uVar4;
    i16        iVar5;
    i16        iVar6;
    u16        uVar7;

    uVar7 = (param_1 >> 0x10);
    uVar3 = (param_1 + 0x6);
    iVar6 = (uVar3 + 0x16);
    if(0x1 < iVar6)
    {
        uVar1 = *(param_1 + 0x6);
        uVar4 = uVar1 + 0x2a;
        uVar1 = uVar1 & 0xffff0000;
        uVar2 = *(uVar1 | uVar4);
        iVar5 = uVar2;
        uVar2 = uVar2 & 0xffff0000;
        uVar7 = (uVar2 >> 0x10);
        MoveTo16(param_2, (iVar5 + 0x20) + (iVar5 + 0x24), (iVar5 + 0x22) / 0x2 + (uVar2 | iVar5 + 0x1e));
        uVar1 = *(uVar4 + iVar6 * 0x4 + -0x4);
        iVar6 = uVar1;
        uVar1 = uVar1 & 0xffff0000;
        uVar7 = (uVar1 >> 0x10);
        LineTo16((HDC16)s_tile2_bmp_1050_1538, (iVar6 + 0x20), (iVar6 + 0x22) / 0x2 + (uVar1 | iVar6 + 0x1e));
    }
    return;
}


void draw_op_1040_c38e(u32 param_1)

{
    u32        uVar1;
    u32 uVar2;
    u32 uVar3;
    i16        iVar4;
    i16        iVar5;
    i1616     *pIVar6;
    u16        in_DX;
    i16        iVar7;
    i16        iVar8;
    u16        uVar9;
    u16        uVar10;
    HDC16      hdc;
    u16        unaff_SS;
    DWORD      DVar11;
    i16        iStack26;
    i1616      IStack20;
    i16        iStack18;
    i1616      IStack16;
    i16        iStack14;

    uVar9 = (param_1 >> 0x10);
    iVar8 = param_1;
    uVar2 = (iVar8 + 0x6);
    iVar7 = (uVar2 + 0x18);
    if((iVar7 != 0x0) && (uVar2 = (iVar8 + 0x6), (uVar2 + 0x16) != 0x0))
    {
        hdc   = 0x1010;
        iVar4 = iVar7;
        pass1_1010_2ee2((iVar8 + 0x6), unaff_SS, 0x1010);
        for(iStack26 = 0x0; iStack26 < iVar7; iStack26 = iStack26 + 0x1)
        {
            uVar1  = *(iStack26 * 0x4 + iVar4);
            iVar5  = uVar1;
            uVar1  = uVar1 & 0xffff0000;
            pIVar6 = (i1616 *)(uVar1 | iVar5 + 0x1e);
            uVar10 = (uVar1 >> 0x10);
            iVar5  = (iVar5 + 0x24) / 0x2 + (iVar5 + 0x20);
            MoveTo16(hdc, iVar5, *pIVar6);
            LineTo16((HDC16)s_tile2_bmp_1050_1538, iVar5, *pIVar6 + -0xf);
            hdc      = (HDC16)s_tile2_bmp_1050_1538;
            DVar11   = GetCurrentPosition16((HDC16)s_tile2_bmp_1050_1538);
            iStack18 = (DVar11 >> 0x10);
            IStack20 = (i1616)DVar11;
            if(iStack26 == 0x0)
            {
                IStack16 = IStack20;
                iStack14 = iStack18;
            }
        }
        uVar2 = (iVar8 + 0x6);
        if((uVar2 + 0x24) != 0x0)
        {
            iStack14 = iStack14 + -0xd;
        }
        uVar2 = (iVar8 + 0x6);
        if((uVar2 + 0x26) != 0x0)
        {
            iStack18 = iStack18 + 0xd;
        }
        uVar2 = (iVar8 + 0x6);
        uVar3 = (iVar8 + 0x6);
        uVar1 = *(uVar2 + (uVar3 + 0x16) * 0x4 + 0x26);
        iVar7 = uVar1;
        uVar1 = uVar1 & 0xffff0000;
        uVar9 = (uVar1 >> 0x10);
        MoveTo16(hdc, (iVar7 + 0x24) / 0x2 + (iVar7 + 0x20), (iVar7 + 0x22) + (uVar1 | iVar7 + 0x1e));
        LineTo16((HDC16)s_tile2_bmp_1050_1538, (iVar7 + 0x24) / 0x2 + (iVar7 + 0x20), IStack16);
        DVar11 = GetCurrentPosition16((HDC16)s_tile2_bmp_1050_1538);
        iVar7  = (DVar11 >> 0x10);
        if(iVar7 < iStack14)
        {
            iStack14 = iVar7;
        }
        if(iStack18 < iVar7)
        {
            iStack18 = iVar7;
        }
        MoveTo16((HDC16)s_tile2_bmp_1050_1538, iStack14, IStack16);
        LineTo16((HDC16)s_tile2_bmp_1050_1538, iStack18, IStack20);
    }
    return;
}


void invalidate_rect_1040_c028(u32 param_1, i16 param_2, HWND16 param_3, RECT16 *param_4)

{
    u32 uVar1;
    u32        uVar2;
    u32 uVar3;
    i16        iVar4;
    i16        iVar5;
    u16        uVar6;
    i16        iVar7;
    u16        uVar9;
    RECT16    *rect;
    RECT16     local_a;
    i16        iStack6;
    i16        iStack4;
    i16       *piVar8;

    iVar7 = param_1;
    uVar9 = (param_1 >> 0x10);
    if(param_2 == 0x8)
    {
        GetClientRect16(param_3, &local_a);
        uVar1     = (iVar7 + 0x6);
        uVar3     = (iVar7 + 0x6);
        iVar5     = (uVar3 + 0x16);
        uVar3     = (iVar7 + 0x6);
        local_a.x = (uVar3 + 0x1a);
        uVar3     = (iVar7 + 0x6);
        local_a.y = (uVar3 + 0x1c);
        if(iVar5 != 0x0)
        {
            if(iVar5 < 0x2)
            {
                iVar4 = 0x1;
            }
            else
            {
                iVar4 = 0x2;
            }
            uVar2     = *((iVar5 - iVar4) * 0x4 + uVar1 + 0x2a);
            iVar5     = uVar2;
            uVar2     = uVar2 & 0xffff0000;
            local_a.x = (iVar5 + 0x22) + (uVar2 | iVar5 + 0x1e);
        }
        uVar1   = (iVar7 + 0x6);
        iStack6 = (uVar1 + 0x1e);
        iStack4 = iStack4 + -0x5;
    }
    else
    {
        if(param_2 != 0x9)
        {
            if(param_2 != 0xa)
            {
                return;
            }
            uVar1 = (iVar7 + 0x6);
            uVar6 = uVar1 + 0x2a;
            if(((iVar7 + 0x8) | uVar6) == 0x0)
            {
                return;
            }
            uVar3     = (iVar7 + 0x6);
            uVar2     = *(((uVar3 + 0x16) + -0x1) * 0x4 + uVar6);
            iVar7     = uVar2;
            uVar2     = uVar2 & 0xffff0000;
            piVar8    = (uVar2 | iVar7 + 0x1e);
            uVar9     = (uVar2 >> 0x10);
            local_a.y = (iVar7 + 0x20) + -0x8;
            local_a.x = *piVar8;
            iStack6   = (iVar7 + 0x22) + *piVar8;
            iStack4   = (iVar7 + 0x20);
            param_4   = &local_a;
            rect      = (RECT16 *)0x0;
            goto LAB_1040_c19d;
        }
        local_a.x = 0x0;
        local_a.y = 0x0;
        iStack6   = 0x0;
        iStack4   = 0x0;
        GetClientRect16(param_3, &local_a);
        uVar1     = (iVar7 + 0x6);
        local_a.x = (uVar1 + 0x1a);
        uVar1     = (iVar7 + 0x6);
        iStack6   = (uVar1 + 0x1e);
        iStack4   = iStack4 + -0x5;
        uVar1     = (iVar7 + 0x6);
        uVar3     = (iVar7 + 0x6);
        iVar7     = (uVar3 + 0x16);
        if(0x0 < iVar7)
        {
            uVar1     = (uVar1 + iVar7 * 0x4 + 0x26);
            iVar7     = uVar1;
            uVar9     = (uVar1 >> 0x10);
            local_a.y = (iVar7 + 0x20) + (iVar7 + 0x24);
        }
    }
    param_3 = (HWND16)s_tile2_bmp_1050_1538;
    rect    = &local_a;
LAB_1040_c19d:
    InvalidateRect16(param_3, rect, (BOOL16)param_4);
    return;
}

Struct18 *pass1_1040_be94(Struct18 *param_1, u8 param_2)

{
    unk_draw_op_1040_b0f8(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


u32 pass1_1040_b74c(Struct18 *param_1, u8 param_2)

{
    unk_draw_op_1040_b0f8(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_b372(u32 param_1, u16 param_2, u16 param_3, COLORREF in_colorref_4)

{
    u16      uVar1;
    i16      iVar2;
    HBRUSH16 local_brush_handle;
    i1616    IVar3;
    u32      uVar4;
    u16      extraout_DX;
    u16      uVar5;
    HWND16   local_win_handle;
    u32      uVar6;
    COLORREF local_colorref;

    uVar5          = (param_1 >> 0x10);
    local_colorref = in_colorref_4;
    if((param_1 + 0x8e) == 0x0)
    {
        local_colorref                = (COLORREF)s_tile2_bmp_1050_1538;
        local_brush_handle            = CreateSolidBrush16(in_colorref_4);
        *(HBRUSH16 *)(param_1 + 0x8e) = local_brush_handle;
    }
    if(_PTR_LOOP_1050_5efa == 0x0)
    {
        local_colorref               = 0x1008;
        uVar6                        = pass1_1008_4d72(*(_PTR_LOOP_1050_4230 + 0xe));
        uVar1                        = (uVar6 >> 0x10);
        iVar2                        = uVar6;
        globals->_PTR_LOOP_1050_5efa = CONCAT12(*(iVar2 + 0x94), CONCAT11(*(iVar2 + 0x95), *(iVar2 + 0x96)));
    }
    if(param_3 < 0x4)
    {
    LAB_1040_b3ea:
        local_win_handle = (HWND16)s_tile2_bmp_1050_1538;
        IVar3            = GetDlgCtrlID16(local_colorref);
        if(IVar3 == 0x14c)
        {
            local_colorref = 0x0;
            goto LAB_1040_b41a;
        }
        if(IVar3 == 0x175)
        {
            local_colorref = 0x0;
            goto LAB_1040_b41a;
        }
    }
    else
    {
        local_win_handle = local_colorref;
        if(param_3 != 0x4)
        {
            if((param_3 == 0x4) || (0x1 < param_3 - 0x5))
            {
                return;
            }
            goto LAB_1040_b3ea;
        }
    }
    local_colorref = (COLORREF)_PTR_LOOP_1050_5efa;
LAB_1040_b41a:
    SetTextColor16(local_win_handle, local_colorref);
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_ace8(Struct18 *param_1)

{
    u16 uVar1;

    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0xafc4;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    unk_draw_op_1040_b0f8(param_1);
    return;
}


Struct18 *pass1_1040_abe2(Struct18 *param_1, u8 param_2)

{
    unk_draw_op_1040_b0f8(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void draw_op_1040_a67e(u32 param_1, i16 param_2, u16 param_3, COLORREF param_4)

{
    i16     *piVar1;
    bool     bVar2;
    u16      uVar3;
    i16      iVar4;
    HBRUSH16 HVar5;
    i16      iVar6;
    u16      uVar7;
    COLORREF hdc;
    u32      uVar8;
    i16      iStack14;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    hdc   = param_4;
    if((iVar6 + 0x8e) == 0x0)
    {
        hdc                         = (COLORREF)s_tile2_bmp_1050_1538;
        HVar5                       = CreateSolidBrush16(param_4);
        *(HBRUSH16 *)(iVar6 + 0x8e) = HVar5;
    }
    if(_PTR_LOOP_1050_5ee8 == 0x0)
    {
        hdc                          = 0x1008;
        uVar8                        = pass1_1008_4d72(*(_PTR_LOOP_1050_4230 + 0xe));
        uVar3                        = (uVar8 >> 0x10);
        iVar4                        = uVar8;
        globals->_PTR_LOOP_1050_5ee8 = CONCAT12(*(iVar4 + 0x94), CONCAT11(*(iVar4 + 0x95), *(iVar4 + 0x96)));
        globals->PTR_LOOP_1050_5eec  = CONCAT11(*(iVar4 + 0x3e5), *(iVar4 + 0x3e6));
        globals->PTR_LOOP_1050_5eee  = *(u8 *)(iVar4 + 0x3e4);
    }
    if(0x5 < param_3)
    {
        if(param_3 != 0x6)
        {
            return;
        }
        bVar2 = false;
        for(iStack14 = 0x0; piVar1 = (iVar6 + 0xea), *piVar1 != iStack14 && iStack14 <= *piVar1; iStack14 = iStack14 + 0x1)
        {
            if((iVar6 + 0x9a + iStack14 * 0x2) == param_2)
            {
                bVar2 = true;
                break;
            }
        }
        if(bVar2)
        {
            globals->PTR_LOOP_1050_5ee8 = globals->PTR_LOOP_1050_5eec;
        }
    }
    SetTextColor16(hdc, (COLORREF)PTR_LOOP_1050_5ee8);
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
    return;
}


u16 *unk_win_ui_op_1040_9854(u16 *param_1, u16 param_2)

{
    HCURSOR16 HVar1;
    HGDIOBJ16 HVar2;
    i16       iVar3;
    u16       uVar4;

    uVar4         = (param_1 >> 0x10);
    iVar3         = param_1;
    *param_1      = 0x389a;
    (iVar3 + 0x2) = 0x1008;
    *param_1      = 0xa230;
    (iVar3 + 0x2) = &PTR_LOOP_1050_1040;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (iVar3 + 0x4)), s_OPButton_1050_5ece);
    (iVar3 + 0x54)               = 0x3;
    HVar1                        = LoadCursor16(0x1000, 0x7f00);
    *(HCURSOR16 *)(iVar3 + 0x58) = HVar1;
    HVar2                        = GetStockObject16((i1616)s_tile2_bmp_1050_1538);
    *(HGDIOBJ16 *)(iVar3 + 0x56) = HVar2;
    reg_class_1040_98c0(param_1 & 0xffff | uVar4 << 0x10, s_tile2_bmp_1050_1538, param_2);
    return param_1;
}


// WARNING: Could not reconcile some variable overlaps

void draw_op_1040_9948(u16 param_1, u32 param_2, HWND16 param_3, RECT16 *param_4)

{
    i16           iVar1;
    i16           iVar2;
    i1616_t       mode;
    u16           uVar3;
    HPEN16        handle;
    HPEN16        handle_00;
    HGDIOBJ16     handle_01;
    u8           *color;
    COLORREF      color_00;
    COLORREF      color_01;
    astruct_71   *iVar4;
    i1616         y;
    char         *x;
    i1616         cx;
    i1616         cy;
    i16           iStack88;
    i16           iStack86;
    i16           iStack84;
    i16           iStack82;
    i16           iStack80;
    i16           iStack78;
    PAi16STRUCT16 local_42;
    u16           uStack34;
    u16           uStack32;
    HGDIOBJ16     HStack30;
    i16           iStack28;
    i16           iStack26;
    i16           iStack24;
    i16           iStack22;
    i16           iStack20;
    RECT16        local_12;
    u32    uStack14;
    i16           local_a;
    i16           iStack8;
    i16           iStack6;
    i16           iStack4;

    iStack6  = 0x1;
    iStack4  = 0x1;
    local_a  = 0x0;
    iStack8  = 0x0;
    iStack28 = 0x0;
    HStack30 = 0x0;
    y        = (i1616)(param_2 >> 0x10);
    iVar4    = (astruct_71 *)param_2;
    uStack32 = iVar4->field_0x4 & 0x8;
    uStack34 = iVar4->field_0x56 & 0x1;
    BeginPai1616(param_3, &local_42);
    mode = SetMapMode16((HDC16)s_tile2_bmp_1050_1538, 0x1);
    GetClientRect16((HWND16)s_tile2_bmp_1050_1538, &local_12);
    iVar2    = (uStack14 >> 0x10);
    iVar1    = iVar2 + -0x1;
    uStack14 = CONCAT22(iVar1, uStack14 + -0x1);
    if(uStack34 != 0x0)
    {
        iStack26 = local_12;
        iStack24 = (local_12 >> 0x10);
        local_12 = CONCAT22(iStack24 + 0x2, iStack26 + 0x2);
        uStack14 = CONCAT22(iVar2 + -0x3, uStack14 + -0x3);
        iStack22 = uStack14 + -0x1;
        iStack20 = iVar1;
    }
    if(iVar4->field_0x6 != '\0')
    {
        iStack28 = 0x1;
        if(iVar4->field_0x5a != 0x0)
        {
            HStack30 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, iVar4->field_0x5a);
        }
        uVar3 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(&iVar4->field_0x6)));
        DrawText16(0x1000, 0x400, (i1616)&local_a, param_4, uVar3);
        iStack8 = ((uStack14._2_2_ - iStack4) + iStack8) / 0x2 + local_12.y;
        iStack4 = iStack4 + iStack8;
        local_a = ((uStack14 - iStack6) + local_a) / 0x2 + local_12.x;
        iStack6 = iStack6 + local_a;
    }
    handle    = CreatePen16((i1616)s_tile2_bmp_1050_1538, (i1616)DAT_1050_5ec2, (COLORREF)(DAT_1050_5ec2 >> 0x10));
    handle_00 = CreatePen16((i1616)s_tile2_bmp_1050_1538, (i1616)DAT_1050_5ebe, (COLORREF)(DAT_1050_5ebe >> 0x10));
    handle_01 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
    if(uStack34 != 0x0)
    {
        iStack78 = 0x0;
        do
        {
            MoveTo16((HDC16)s_tile2_bmp_1050_1538, iStack20, iStack26);
            LineTo16((HDC16)s_tile2_bmp_1050_1538, iStack20, iStack22);
            LineTo16((HDC16)s_tile2_bmp_1050_1538, iStack24, iStack22);
            LineTo16((HDC16)s_tile2_bmp_1050_1538, iStack24, iStack26);
            LineTo16((HDC16)s_tile2_bmp_1050_1538, iStack20, iStack26);
            iStack24 = iStack24 + 0x1;
            iStack26 = iStack26 + 0x1;
            iStack22 = iStack22 + -0x1;
            iStack20 = iStack20 + -0x1;
            iStack78 = iStack78 + 0x1;
        } while(iStack78 < 0x1);
    }
    if((iVar4->field_0x4 & 0x2) == 0x0)
    {
        iStack84 = (local_12 >> 0x10);
        iStack82 = uStack14;
        iStack78 = 0x0;
        iStack86 = local_12.x;
        iStack80 = uStack14._2_2_;
        do
        {
            SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
            MoveTo16((HDC16)s_tile2_bmp_1050_1538, iStack80, iStack86);
            LineTo16((HDC16)s_tile2_bmp_1050_1538, iStack80, iStack82);
            LineTo16((HDC16)s_tile2_bmp_1050_1538, iStack84, iStack82);
            iStack88 = 0x0;
            do
            {
                SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_00);
                LineTo16((HDC16)s_tile2_bmp_1050_1538, iStack84, iStack86);
                LineTo16((HDC16)s_tile2_bmp_1050_1538, iStack80, iStack86);
                iStack88 = iStack88 + 0x1;
            } while(iStack88 < 0x2);
            iStack84 = iStack84 + 0x1;
            iStack86 = iStack86 + 0x1;
            iStack82 = iStack82 + -0x1;
            iStack80 = iStack80 + -0x1;
            iStack78 = iStack78 + 0x1;
        } while(iStack78 < 0x2);
    }
    else
    {
        MoveTo16((HDC16)s_tile2_bmp_1050_1538, uStack14._2_2_, local_12.x);
        LineTo16((HDC16)s_tile2_bmp_1050_1538, local_12.y, local_12.x);
        LineTo16((HDC16)s_tile2_bmp_1050_1538, local_12.y, uStack14 + 0x1);
        if(iStack28 != 0x0)
        {
            iStack8 = iStack8 + 0x2;
            local_a = local_a + 0x2;
            iStack6 = iStack6 + 0x2;
            iStack4 = iStack4 + 0x2;
        }
    }
    MoveTo16((HDC16)s_tile2_bmp_1050_1538, 0x0, 0x0);
    if(iStack28 != 0x0)
    {
        if((iVar4->field_0x4 & 0x4) == 0x0)
        {
            color = globals->PTR_LOOP_1050_5ec6;
            if(uStack32 != 0x0)
            {
                color = DAT_1050_5eca;
            }
            color_00 = SetTextColor16((HDC16)s_tile2_bmp_1050_1538, (COLORREF)color);
            color_01 = SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
            uVar3    = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(&iVar4->field_0x6)));
            DrawText16(0x1000, (&PTR_LOOP_1050_0000 + 0x1), (i1616)&local_a, param_4, uVar3);
            SetTextColor16((HDC16)s_tile2_bmp_1050_1538, color_00);
            SetBkColor16((HDC16)s_tile2_bmp_1050_1538, color_01);
        }
        else
        {
            GetStockObject16((i1616)s_tile2_bmp_1050_1538);
            cx    = 0x0;
            cy    = 0x0;
            x     = &iVar4->field_0x6;
            uVar3 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(x)));
            GrayString16(0x1000, iStack4 - iStack8, (iStack6 - local_a), CONCAT22(local_a, iStack8), uVar3, (i1616)x, y, cx, cy);
        }
        if(HStack30 != 0x0)
        {
            SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack30);
        }
    }
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_01);
    SetMapMode16((HDC16)s_tile2_bmp_1050_1538, mode);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPai1616((HWND16)s_tile2_bmp_1050_1538, &local_42);
    return;
}


void mixed_draw_op_1040_8a06(u32 param_1, HWND16 param_2, u16 param_3)

{
    u8            uVar1;
    u8            uVar2;
    astruct_13   *paVar3;
    u16           uVar4;
    HPALETTE16    b_force_background;
    COLORREF      color;
    COLORREF      color_00;
    HANDLE16      handle;
    u16           in_DX;
    RECT16       *rect;
    u32           uVar5;
    HGDIOBJ16     HStack62;
    HDC16         local_24;
    PAi16STRUCT16 local_22;

    rect               = (RECT16 *)(param_1 >> 0x10);
    local_24           = BeginPai1616(param_2, &local_22);
    paVar3             = *(astruct_13 **)(_PTR_LOOP_1050_4230 + 0xe);
    b_force_background = palette_op_1008_4e08(paVar3, &local_24, in_DX, 0x1008);
    uVar5              = pass1_1008_4d72(paVar3);
    uVar4              = (uVar5 >> 0x10);
    uVar1              = *(uVar5 + 0x95);
    uVar2              = *(uVar5 + 0x96);
    DrawIcon16(0x1008, (param_1 + 0x8e), 0xa, 0x14);
    color    = SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
    color_00 = SetTextColor16((HDC16)s_tile2_bmp_1050_1538, CONCAT11(uVar1, uVar2));
    HStack62 = 0x0;
    handle   = GetProp16((HWND16)s_tile2_bmp_1050_1538, 0x5dfa);
    if(handle != 0x0)
    {
        HStack62 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
    }
    DrawText16((HDC16)s_tile2_bmp_1050_1538, &PTR_LOOP_1050_0010, param_1 + 0x9e, rect, 0xffff);
    if(handle != 0x0)
    {
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack62);
    }
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, color);
    SetTextColor16((HDC16)s_tile2_bmp_1050_1538, color_00);
    SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPai1616((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}


void pass1_1040_8e82(Struct18 *param_1)

{
    param_1->field_0x0 = 0x8f3c;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1040;
    unk_draw_op_1040_b0f8(param_1);
    return;
}


void pass1_1040_9252(u32 param_1, u16 param_2)

{
    i16         *piVar1;
    i16          iVar2;
    astruct_161 *iVar3;
    u16          uVar3;

    uVar3 = (param_1 >> 0x10);
    iVar3 = (astruct_161 *)param_1;
    if(iVar3->field_0x4 != 0x0)
    {
        draw_text_1040_9650(param_1 & 0xffff | uVar3 << 0x10, param_2);
    }
    if(iVar3->field_0x8 != 0x0)
    {
        pass1_1040_9618(param_1 & 0xffff | uVar3 << 0x10);
    }
    iVar2 = iVar3->field_0x32;
    if(iVar3->field_0x22 < iVar2)
    {
        iVar3->field_0x22 = iVar2;
    }
    iVar2 = iVar3->field_0x34;
    if(iVar3->field_0x24 < iVar2)
    {
        iVar3->field_0x24 = iVar2;
    }
    iVar2 = iVar3->field_0x26 + iVar3->field_0x2a;
    if(iVar3->field_0x22 < iVar2)
    {
        iVar3->field_0x22 = iVar2;
    }
    iVar2 = iVar3->field_0x28 + iVar3->field_0x2c;
    if(iVar3->field_0x24 < iVar2)
    {
        iVar3->field_0x24 = iVar2;
    }
    piVar1  = &iVar3->field_0x22;
    *piVar1 = *piVar1 + iVar3->field_0x36;
    piVar1  = &iVar3->field_0x24;
    *piVar1 = *piVar1 + iVar3->field_0x36;
    return;
}


void unk_draw_op_1040_9458(astruct_17 *param_1, u8 param_2, u16 param_3, HDC16 param_4)

{
    code      **ppcVar1;
    u32         UVar2;
    u16        *b_force_background;
    u16         uVar3;
    astruct_17 *iVar4;
    u16         uVar4;
    u16        *puStack8;
    u32        *puStack6;

    uVar4 = (param_1 >> 0x10);
    iVar4 = (astruct_17 *)param_1;
    if(iVar4->field_0x8 != 0x0)
    {
        puStack6 = iVar4->field_0x8;
        uVar3    = (&iVar4->field_0x8 + 0x2);
        if((((&iVar4->field_0xc + 0x2) | &iVar4->field_0xc) != 0x0) && ((param_2 & 0x1) != 0x0))
        {
            puStack6 = iVar4->field_0xc;
            uVar3    = (&iVar4->field_0xc + 0x2);
        }
        if((iVar4->field_0x10 != 0x0) && ((param_2 & 0x4) != 0x0))
        {
            puStack6 = iVar4->field_0x10;
            uVar3    = (&iVar4->field_0x10 + 0x2);
        }
        b_force_background = &param_3;
        UVar2              = *puStack6;
        ppcVar1            = (UVar2 + 0x8);
        (**ppcVar1)(param_4, puStack6, uVar3, b_force_background);
        ppcVar1 = (UVar2 + 0x4);
        (**ppcVar1)(param_4, puStack6, iVar4->field_0x28, iVar4->field_0x26, &param_3);
        SelectPalette16(param_4, 0x0, (BOOL16)b_force_background);
        DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    }
    return;
}


void draw_text_1040_94fc(astruct_37 *param_1, HDC16 param_2)

{
    COLORREF    color;
    COLORREF    color_00;
    astruct_38 *iVar1;
    RECT16     *rect;

    rect     = (RECT16 *)(param_1 >> 0x10);
    iVar1    = (astruct_38 *)param_1;
    color    = SetBkColor16(param_2, iVar1->field_0x3a);
    color_00 = SetTextColor16((HDC16)s_tile2_bmp_1050_1538, iVar1->field_0x3c);
    DrawText16((HDC16)s_tile2_bmp_1050_1538, &PTR_LOOP_1050_0010, &iVar1->field_0x2e, rect, 0xffff);
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, color);
    SetTextColor16((HDC16)s_tile2_bmp_1050_1538, color_00);
    return;
}


void draw_text_1040_9650(u32 param_1, HWND16 param_2)

{
    HDC16 hdc;

    hdc = GetDC16(param_2);
    DrawText16((HDC16)s_tile2_bmp_1050_1538, 0x410, param_1 + 0x2e, (RECT16 *)(param_1 >> 0x10), 0xffff);
    ReleaseDC16((HWND16)s_tile2_bmp_1050_1538, hdc);
    return;
}


void draw_op_1040_82ee(astruct_15 *param_1, COLORREF in_colorref_2)

{
    astruct_15 *iVar1;
    u16         uVar1;
    u32  local_1a;
    u32  uStack22;
    i16         local_12;
    i16         iStack16;
    i16         iStack14;
    i16         iStack12;
    RECT16     *l_brush_handle;
    i16         iStack8;
    i16         iStack6;
    i16         iStack4;

    uVar1          = (param_1 >> 0x10);
    iVar1          = (astruct_15 *)param_1;
    iStack6        = (iVar1->field_0x80 - iVar1->field_0x7c) + -0x2;
    iStack8        = (-(iVar1->field_0x60 == 0x0) & 0x1e) + 0x25;
    iStack4        = iStack6;
    l_brush_handle = (RECT16 *)CreateSolidBrush16(in_colorref_2);
    if(iVar1->field_0x86 == 0x0)
    {
        local_1a           = CONCAT22(iVar1->field_0x66 + 0x2, iVar1->field_0x64 + 0x2);
        uStack22           = CONCAT22(iStack4, iStack6);
        &iVar1->field_0x82 = local_1a;
        &iVar1->field_0x86 = uStack22;
    }
    local_12 = iVar1->field_0x82 + 0x2;
    iStack16 = (iVar1->field_0x88 - iVar1->field_0x84) / 0x2 + iVar1->field_0x84 + -0x2;
    iStack14 = iVar1->field_0x86 - 0x2;
    iStack12 = (iVar1->field_0x88 - iVar1->field_0x84) / 0x2 + iVar1->field_0x84 + 0x2;
    FrameRect16((HDC16)s_tile2_bmp_1050_1538, l_brush_handle, (HBRUSH16)&iVar1->field_0x82);
    FrameRect16((HDC16)s_tile2_bmp_1050_1538, l_brush_handle, (HBRUSH16)&local_12);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    iVar1->field_0x7a = iVar1->field_0x86 + 0x2;
    return;
}


u32 set_text_bk_color_1040_7e5e(u32 *param_1, u16 param_2, u16 param_3, i1616 param_4)

{
    code    **ppcVar1;
    i16       iVar2;
    HGDIOBJ16 HVar3;
    i1616     IVar4;
    HWND16    hwnd;
    HWND16    hdc;
    u32       uVar5;
    COLORREF  color;
    u16       uVar6;

    uVar6 = 0x4;
    hwnd  = (HWND16)s_tile2_bmp_1050_1538;
    HVar3 = GetStockObject16(param_4);
    if(_PTR_LOOP_1050_5df0 == 0x0)
    {
        ppcVar1 = (*param_1 + 0x68);
        uVar5   = (**ppcVar1)(s_tile2_bmp_1050_1538, param_1, (param_1 + 0x6e), uVar6);
        if(uVar5 == 0x0)
        {
            return 0x0;
        }
        hwnd                         = 0x1008;
        uVar5                        = pass1_1008_4d72(uVar5);
        uVar6                        = (uVar5 >> 0x10);
        iVar2                        = uVar5;
        globals->_PTR_LOOP_1050_5df0 = CONCAT22(CONCAT11(0x2, *(iVar2 + 0x94)), CONCAT11(*(iVar2 + 0x95), *(iVar2 + 0x96)));
    }
    hdc = hwnd;
    if(0x5 < param_3)
    {
        if(param_3 != 0x6)
        {
            return 0x0;
        }
        hdc   = (HWND16)s_tile2_bmp_1050_1538;
        IVar4 = GetDlgCtrlID16(hwnd);
        if(IVar4 == 0x14c)
        {
            color = 0x0;
            goto LAB_1040_7f00;
        }
        if(IVar4 == 0x175)
        {
            color = 0x0;
            goto LAB_1040_7f00;
        }
    }
    color = (COLORREF)_PTR_LOOP_1050_5df0;
LAB_1040_7f00:
    SetTextColor16(hdc, color);
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
    return CONCAT22(0x1050, HVar3);
}


void draw_op_1040_7bb2(astruct_14 *in_struct_1, HWND16 in_win_handle_2, u16 param_3)

{
    code      **ppcVar1;
    BOOL16      BVar2;
    i16         y;
    i16         iVar3;
    COLORREF    color;
    HPEN16      handle;
    HGDIOBJ16   handle_00;
    RECT16     *rect;
    HANDLE16    handle_01;
    LPCSTR      str;
    astruct_14 *iVar4;
    char       *count;
    char       *str_00;
    u32         uVar4;
    DWORD       DVar5;
    HBRUSH16    hbrush;
    u16         uVar6;
    u16         uVar7;
    HGDIOBJ16   local_obj_handle_42;
    RECT16      local_rect_12;
    i16         iStack14;
    i16         iStack12;
    HPALETTE16  HStack10;
    u32  uStack8;
    HDC16       local_hdc_4;

    str_00 = (in_struct_1 >> 0x10);
    iVar4  = (astruct_14 *)in_struct_1;
    uVar7  = iVar4->field_0x6;
    BVar2  = IsIconic16(in_win_handle_2);
    if(BVar2 == 0x0)
    {
        uVar6       = iVar4->field_0x6;
        local_hdc_4 = GetWindowDC16((HWND16)s_tile2_bmp_1050_1538);
        ppcVar1     = (in_struct_1 + 0x68);
        uStack8     = (astruct_13 *)(**ppcVar1)(s_tile2_bmp_1050_1538, in_struct_1, iVar4->field_0x6e, uVar6, uVar7);
        if(uStack8 != (astruct_13 *)0x0)
        {
            HStack10 = palette_op_1008_4e08(uStack8, &local_hdc_4, (uStack8 >> 0x10) | uStack8, 0x1008);
            GetWindowRect16(0x1008, &local_rect_12);
            y         = (iStack14 - local_rect_12.x) + -0x1;
            iVar3     = (iStack12 - local_rect_12.y) + -0x1;
            color     = (-(iVar4->field_0x60 == 0x0) & 0x1e) + 0x25;
            handle    = CreatePen16((i1616)s_tile2_bmp_1050_1538, color, 0x100);
            handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
            MoveTo16((HDC16)s_tile2_bmp_1050_1538, 0x0, 0x0);
            LineTo16((HDC16)s_tile2_bmp_1050_1538, 0x0, y);
            LineTo16((HDC16)s_tile2_bmp_1050_1538, iVar3, y);
            LineTo16((HDC16)s_tile2_bmp_1050_1538, iVar3, 0x0);
            LineTo16((HDC16)s_tile2_bmp_1050_1538, 0x0, 0x0);
            uVar4 = GetWindowLong16((HWND16)s_tile2_bmp_1050_1538, -0x10);
            if(((uVar4 & 0x800000) != 0x0) && ((uVar4 & 0x400000) != 0x0))
            {
                iVar3 = iVar4->field_0x62 - iVar4->field_0x66;
                MoveTo16((HDC16)s_tile2_bmp_1050_1538, iVar3, 0x0);
                LineTo16((HDC16)s_tile2_bmp_1050_1538, iVar3, y);
                iVar4->field_0x7a = iVar4->field_0x64;
                iVar4->field_0x7c = iVar4->field_0x66;
                iVar4->field_0x7e = y;
                iVar4->field_0x80 = iVar4->field_0x62 - iVar4->field_0x66;
                hbrush            = 0x4;
                rect              = (RECT16 *)GetStockObject16((i1616)s_tile2_bmp_1050_1538);
                FillRect16((HDC16)s_tile2_bmp_1050_1538, rect, hbrush);
                if(iVar4->field_0x76 != 0x0)
                {
                    draw_op_1040_82ee((astruct_15 *)in_struct_1, s_tile2_bmp_1050_1538);
                }
                count = &iVar4->field_0x10;
                if(*count != '\0')
                {
                    local_obj_handle_42 = 0x0;
                    handle_01           = GetProp16((HWND16)s_tile2_bmp_1050_1538, 0x5de9);
                    if(handle_01 != 0x0)
                    {
                        local_obj_handle_42 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_01);
                    }
                    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
                    SetTextColor16((HDC16)s_tile2_bmp_1050_1538, color);
                    str   = lstrlen16(s_tile2_bmp_1050_1538);
                    DVar5 = GetTextExtent16((HDC16)s_tile2_bmp_1050_1538, str, (i1616)count);
                    TextOut16((HDC16)s_tile2_bmp_1050_1538, (i1616)str, (i1616)count, str_00, (iVar4->field_0x80 - iVar4->field_0x7c) / 0x2 - (DVar5 >> 0x10) / 0x2);
                    if(handle_01 != 0x0)
                    {
                        SelectObject16((HDC16)s_tile2_bmp_1050_1538, local_obj_handle_42);
                    }
                }
            }
            ppcVar1 = (in_struct_1 + 0x64);
            (**ppcVar1)(s_tile2_bmp_1050_1538, in_struct_1, uStack8, local_hdc_4);
            HStack10 = SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, HStack10);
            DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
            SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_00);
            DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
        }
        ReleaseDC16((HWND16)s_tile2_bmp_1050_1538, local_hdc_4);
        return;
    }
    return;
}


Struct18 *pass1_1040_767e(Struct18 *param_1, u8 param_2)

{
    unk_draw_op_1040_b0f8(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 *pass1_1040_6360(Struct18 *param_1, u8 param_2)

{
    unk_draw_op_1040_b0f8(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void pass1_1040_6862(Struct18 *param_1)

{
    param_1->field_0x0 = 0x6f32;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1040;
    unk_draw_op_1040_b0f8(param_1);
    return;
}


Struct18 *pass1_1040_4df2(Struct18 *param_1, u8 param_2)

{
    unk_draw_op_1040_b0f8(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void pass1_1040_4f0a(Struct18 *param_1)

{
    param_1->field_0x0 = 0x55a2;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1040;
    unk_draw_op_1040_b0f8(param_1);
    return;
}


void draw_op_1040_5a06(u32 param_1, HWND16 param_2, u16 param_3)

{
    u16          *puVar1;
    u32    uVar2;
    code        **ppcVar3;
    u32    uVar4;
    HPALETTE16    b_force_background;
    i16           iVar5;
    HPEN16        handle;
    HGDIOBJ16     handle_00;
    i16           x;
    i16           y;
    u16           in_DX;
    i16           iVar6;
    u16           uVar7;
    u16           uVar8;
    i1616         IVar9;
    u32           uVar10;
    astruct_43   *paVar11;
    astruct_76   *paVar12;
    u16           uVar13;
    HDC16        *pHVar14;
    u16           uVar15;
    HDC16         HVar16;
    HDC16         HVar17;
    HDC16         HVar18;
    u16           uVar19;
    u16           uVar20;
    u16           uVar21;
    u16           uStack82;
    i16           iStack72;
    i16           iStack68;
    astruct_76   *paStack54;
    HDC16         local_2c;
    PAi16STRUCT16 local_2a;
    RECT16        local_a[0x2];

    uVar7  = (param_1 >> 0x10);
    iVar6  = param_1;
    uVar21 = (iVar6 + 0x6);
    GetWindowRect16(param_2, local_a);
    uVar13             = (iVar6 + 0x6);
    local_2c           = BeginPai1616((HWND16)s_tile2_bmp_1050_1538, &local_2a);
    uVar8              = 0x1008;
    b_force_background = palette_op_1008_4e08(*(astruct_13 **)(_PTR_LOOP_1050_4230 + 0xe), &local_2c, in_DX, 0x1008);
    paStack54          = (astruct_76 *)0x0;
    if((iVar6 + 0x98) != 0x0)
    {
        paStack54 = (astruct_76 *)unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, (iVar6 + 0x98), param_3);
        IVar9     = 0x1008;
        uVar10    = pass1_1008_4772(paStack54);
        if(((uVar10 >> 0x10) | uVar10) == 0x0)
        {
            if(paStack54 != (astruct_76 *)0x0)
            {
                if(paStack54 != (astruct_76 *)0x0)
                {
                    ppcVar3 = paStack54;
                    (**ppcVar3)(0x1008, paStack54, (paStack54 >> 0x10), 0x1, uVar13);
                }
            }
            IVar9     = 0x1010;
            paStack54 = (astruct_76 *)unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x4d, param_3);
        }
        uVar8 = SUB42(s_tile2_bmp_1050_1538, 0x0);
        GetSystemMetrics16(IVar9);
        ppcVar3 = (paStack54 + 0x4);
        (**ppcVar3)();
    }
    if(paStack54 != (astruct_76 *)0x0)
    {
        if(paStack54 != (astruct_76 *)0x0)
        {
            ppcVar3 = paStack54;
            (**ppcVar3)(uVar8, paStack54, (paStack54 >> 0x10), 0x1, uVar13, uVar21);
        }
    }
    paVar11 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, (iVar6 + 0x96), param_3);
    uVar21  = (paVar11 >> 0x10);
    pHVar14 = &local_2c;
    uVar19  = 0x4;
    uVar20  = 0xd;
    uVar15  = param_3;
    IVar9   = GetSystemMetrics16(0x1010);
    iVar5   = -(IVar9 + -0x23);
    uVar4   = paVar11;
    ppcVar3 = uVar4 + 0x2;
    uVar13  = paVar11;
    uVar8   = uVar21;
    (**ppcVar3)();
    if(paVar11 != (astruct_43 *)0x0)
    {
        if(paVar11 != (astruct_43 *)0x0)
        {
            ppcVar3 = uVar4;
            (**ppcVar3)(s_tile2_bmp_1050_1538, paVar11, uVar21, 0x1, uVar13, uVar8, iVar5, uVar19, uVar20, pHVar14, uVar15);
        }
    }
    handle    = CreatePen16((i1616)s_tile2_bmp_1050_1538, 0x25, 0x100);
    handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
    paVar12   = (astruct_76 *)unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x4f, param_3);
    uVar21    = (paVar12 >> 0x10);
    uVar10    = pass1_1008_4772(paVar12);
    uVar13    = (uVar10 >> 0x10);
    uVar4     = (uVar10 + 0x4);
    uVar2     = (uVar10 + 0x8);
    IVar9     = GetSystemMetrics16(0x1008);
    iVar5     = -(IVar9 + -0xc1);
    IVar9     = GetSystemMetrics16((i1616)s_tile2_bmp_1050_1538);
    iStack72  = uVar2;
    x         = 0xc5 - (IVar9 - iStack72);
    MoveTo16((HDC16)s_tile2_bmp_1050_1538, iVar5, 0x82);
    iStack68 = uVar4;
    y        = iStack68 * 0xa + 0x85;
    LineTo16((HDC16)s_tile2_bmp_1050_1538, iVar5, y);
    HVar18 = local_2c;
    LineTo16((HDC16)s_tile2_bmp_1050_1538, x, y);
    HVar17 = local_2c;
    LineTo16((HDC16)s_tile2_bmp_1050_1538, x, 0x82);
    HVar16 = local_2c;
    LineTo16((HDC16)s_tile2_bmp_1050_1538, iVar5, 0x82);
    for(uStack82 = 0x0; puVar1 = (iVar6 + 0x94), uStack82 <= *puVar1 && *puVar1 != uStack82; uStack82 = uStack82 + 0x1)
    {
        pHVar14 = &local_2c;
        iVar5   = iStack68 * uStack82 + 0x84;
        uVar13  = 0x4;
        uVar15  = param_3;
        IVar9   = GetSystemMetrics16((i1616)s_tile2_bmp_1050_1538);
        ppcVar3 = (paVar12 + 0x4);
        (**ppcVar3)(s_tile2_bmp_1050_1538, paVar12, uVar21, -(IVar9 + -0xc4), uVar13, iVar5, pHVar14, uVar15, HVar16, HVar17);
    }
    if(paVar12 != (astruct_76 *)0x0)
    {
        if(paVar12 != (astruct_76 *)0x0)
        {
            ppcVar3 = paVar12;
            (**ppcVar3)(s_tile2_bmp_1050_1538, paVar12, uVar21, 0x1, HVar16, HVar17, HVar18);
        }
    }
    SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle_00);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPai1616((HWND16)s_tile2_bmp_1050_1538, &local_2a);
    return;
}


u16 get_dc_op_1040_3d5e(u32 param_1, HWND16 param_2, u16 param_3)

{
    code      **ppcVar1;
    i16         iVar2;
    u16         uVar3;
    astruct_43 *paVar4;
    u16         uVar5;
    HDC16       local_4;

    uVar3   = (param_1 >> 0x10);
    uVar5   = (param_1 + 0x6);
    local_4 = GetDC16(param_2);
    paVar4  = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, (param_1 + 0xa4), param_3);
    iVar2   = paVar4;
    ppcVar1 = (iVar2 + 0x8);
    (**ppcVar1)(0x1010, paVar4, (paVar4 >> 0x10), &local_4, param_3, uVar5);
    ppcVar1 = (iVar2 + 0x4);
    (**ppcVar1)(0x1010, paVar4, 0x50078, &local_4, param_3);
    ppcVar1 = (iVar2 + 0xc);
    (**ppcVar1)(0x1010, paVar4, &local_4, param_3);
    ReleaseDC16(0x1010, local_4);
    return 0x0;
}


void invalidate_rect_1040_3ddc(astruct_2 *in_struct_1, HWND16 in_win_handle_2)

{
    u32 local_b_erase;
    u32 uStack6;

    local_b_erase = 0x780005;
    uStack6       = 0xdc0069;
    InvalidateRect16(in_win_handle_2, (RECT16 *)0x0, (BOOL16)&local_b_erase);
    return;
}


Struct18 *pass1_1040_47fe(Struct18 *param_1, u8 param_2)

{
    unk_draw_op_1040_b0f8(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


u32 draw_ui_op_1040_27cc(u32 *param_1, u16 param_2, u16 param_3, COLORREF param_4)

{
    code   **ppcVar1;
    u16      uVar2;
    i16      iVar3;
    HBRUSH16 HVar4;
    i1616    IVar5;
    i16      iVar6;
    u16      uVar7;
    COLORREF CVar8;
    HWND16   hdc;
    u32      uVar9;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    CVar8 = param_4;
    if((iVar6 + 0x4) == 0x0)
    {
        CVar8                      = (COLORREF)s_tile2_bmp_1050_1538;
        HVar4                      = CreateSolidBrush16(param_4);
        *(HBRUSH16 *)(iVar6 + 0x4) = HVar4;
    }
    if(_PTR_LOOP_1050_5cf8 == 0x0)
    {
        ppcVar1                      = (*param_1 + 0x68);
        uVar9                        = (**ppcVar1)(CVar8, param_1, (iVar6 + 0x6e));
        CVar8                        = 0x1008;
        uVar9                        = pass1_1008_4d72(uVar9);
        uVar2                        = (uVar9 >> 0x10);
        iVar3                        = uVar9;
        globals->_PTR_LOOP_1050_5cf8 = CONCAT22(CONCAT11(0x2, *(iVar3 + 0x94)), CONCAT11(*(iVar3 + 0x95), *(iVar3 + 0x96)));
    }
    hdc = CVar8;
    if(0x5 < param_3)
    {
        if(param_3 != 0x6)
        {
            return 0x0;
        }
        hdc   = (HWND16)s_tile2_bmp_1050_1538;
        IVar5 = GetDlgCtrlID16(CVar8);
        if(((iVar6 + 0x94) != 0x0) && (IVar5 == 0xfb2))
        {
            CVar8 = 0x0;
            goto LAB_1040_286e;
        }
    }
    CVar8 = (COLORREF)_PTR_LOOP_1050_5cf8;
LAB_1040_286e:
    SetTextColor16(hdc, CVar8);
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
    return CONCAT22(0x1050, (iVar6 + 0x4));
}


void pass1_1040_2a22(Struct18 *param_1)

{
    astruct_625 *iVar1;
    u16          uVar1;

    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_625 *)param_1;
    param_1->field_0x0 = 0x2e26;
    iVar1->field_0x2   = &PTR_LOOP_1050_1040;
    fn_ptr_1000_17ce(iVar1->field_0x94, 0x1000);
    fn_ptr_1000_17ce(iVar1->field_0x98, 0x1000);
    unk_draw_op_1040_b0f8(param_1);
    return;
}


void mix_draw_op_1040_21d6(u32 param_1, HWND16 param_2, u16 param_3)

{
    u8            uVar1;
    u8            uVar2;
    astruct_13   *paVar3;
    code        **ppcVar4;
    i16           iVar5;
    HPALETTE16    b_force_background;
    COLORREF      color;
    COLORREF      color_00;
    HANDLE16      handle;
    u16           in_DX;
    i16           iVar6;
    RECT16       *rect;
    u32           uVar7;
    u16           uVar8;
    HGDIOBJ16     HStack62;
    HDC16         local_24;
    PAi16STRUCT16 local_22;

    rect               = (RECT16 *)(param_1 >> 0x10);
    iVar6              = param_1;
    uVar8              = (iVar6 + 0x6);
    local_24           = BeginPai1616(param_2, &local_22);
    paVar3             = *(astruct_13 **)(_PTR_LOOP_1050_4230 + 0xe);
    b_force_background = palette_op_1008_4e08(paVar3, &local_24, in_DX, 0x1008);
    ppcVar4            = ((iVar6 + 0x8e) + 0x4);
    (**ppcVar4)(0x1008, (iVar6 + 0x8e), 0xffffffff, &local_24, param_3, uVar8);
    uVar7    = pass1_1008_4d72(paVar3);
    uVar8    = (uVar7 >> 0x10);
    iVar5    = uVar7;
    uVar1    = *(iVar5 + 0x3e5);
    uVar2    = *(iVar5 + 0x3e6);
    color    = SetBkColor16(0x1008, 0x0);
    color_00 = SetTextColor16((HDC16)s_tile2_bmp_1050_1538, CONCAT11(uVar1, uVar2));
    HStack62 = 0x0;
    handle   = GetProp16((HWND16)s_tile2_bmp_1050_1538, 0x5ced);
    if(handle != 0x0)
    {
        HStack62 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
    }
    DrawText16((HDC16)s_tile2_bmp_1050_1538, &PTR_LOOP_1050_0010, iVar6 + 0x92, rect, 0xffff);
    SetTextColor16((HDC16)s_tile2_bmp_1050_1538, CONCAT11(*(iVar5 + 0x95), *(iVar5 + 0x96)));
    DrawText16((HDC16)s_tile2_bmp_1050_1538, &PTR_LOOP_1050_0010, iVar6 + 0x9a, rect, 0xffff);
    if(handle != 0x0)
    {
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack62);
    }
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, color);
    SetTextColor16((HDC16)s_tile2_bmp_1050_1538, color_00);
    SelectPalette16((HDC16)s_tile2_bmp_1050_1538, 0x0, b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    EndPai1616((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}


u32 set_text_bk_color_1040_0cc0(u32 *param_1, u16 param_2, u16 param_3, i1616 param_4)

{
    code    **ppcVar1;
    i16       iVar2;
    HDC16     obj;
    HDC16     hdc;
    u32       uVar3;
    u16       uVar4;
    HGDIOBJ16 HStack4;

    uVar4   = 0x4;
    obj     = (HDC16)s_tile2_bmp_1050_1538;
    HStack4 = GetStockObject16(param_4);
    if(_PTR_LOOP_1050_5cd0 == 0x0)
    {
        ppcVar1                      = (*param_1 + 0x68);
        uVar3                        = (**ppcVar1)(s_tile2_bmp_1050_1538, param_1, (param_1 + 0x6e), uVar4);
        obj                          = 0x1008;
        uVar3                        = pass1_1008_4d72(uVar3);
        uVar4                        = (uVar3 >> 0x10);
        iVar2                        = uVar3;
        globals->_PTR_LOOP_1050_5cd0 = CONCAT22(CONCAT11(0x2, *(iVar2 + 0x94)), CONCAT11(*(iVar2 + 0x95), *(iVar2 + 0x96)));
    }
    hdc = obj;
    if(0x3 < param_3)
    {
        if(param_3 == 0x4)
        {
            hdc     = (HDC16)s_tile2_bmp_1050_1538;
            HStack4 = GetStockObject16(obj);
        }
        else
        {
            if((param_3 == 0x4) || (0x1 < param_3 - 0x5))
            {
                return 0x0;
            }
        }
    }
    SetTextColor16(hdc, (COLORREF)_PTR_LOOP_1050_5cd0);
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
    return CONCAT22(0x1050, HStack4);
}


void draw_op_1038_9dcc(astruct_10 *in_struct_1, i16 param_2, u16 param_3, COLORREF in_colorref_4, u16 param_5)

{
    u16        *puVar1;
    bool        bVar2;
    u16         uVar3;
    i16         iVar4;
    HBRUSH16    local_brush_handle;
    u32         uVar5;
    u16         extraout_DX;
    astruct_10 *local_struct_5;
    astruct_10 *var5;
    COLORREF    hdc;
    u32         uVar6;
    u16         uStack14;

    var5           = (astruct_10 *)(in_struct_1 >> 0x10);
    local_struct_5 = (astruct_10 *)in_struct_1;
    hdc            = in_colorref_4;
    if(local_struct_5->brush_handle_field_0x8e == 0x0)
    {
        hdc                                     = (COLORREF)s_tile2_bmp_1050_1538;
        local_brush_handle                      = CreateSolidBrush16(in_colorref_4);
        local_struct_5->brush_handle_field_0x8e = local_brush_handle;
    }
    if(_PTR_LOOP_1050_5b64 == 0x0)
    {
        hdc                          = 0x1008;
        uVar6                        = pass1_1008_4d72(*(_PTR_LOOP_1050_4230 + 0xe));
        uVar3                        = (uVar6 >> 0x10);
        iVar4                        = uVar6;
        globals->_PTR_LOOP_1050_5b64 = CONCAT12(*(iVar4 + 0x94), CONCAT11(*(iVar4 + 0x95), *(iVar4 + 0x96)));
        globals->PTR_LOOP_1050_5b68  = CONCAT11(*(iVar4 + 0x3e5), *(iVar4 + 0x3e6));
        globals->PTR_LOOP_1050_5b6a  = *(u8 *)(iVar4 + 0x3e4);
    }
    if(0x5 < param_3)
    {
        if(param_3 != 0x6)
        {
            return;
        }
        bVar2 = false;
        for(uStack14 = 0x0; puVar1 = &local_struct_5->field_0x128, uStack14 <= *puVar1 && *puVar1 != uStack14; uStack14 = uStack14 + 0x1)
        {
            if((&local_struct_5->field_0x94 + uStack14 * 0x2) == param_2)
            {
                bVar2 = true;
                break;
            }
        }
        if(bVar2)
        {
            globals->PTR_LOOP_1050_5b64 = globals->PTR_LOOP_1050_5b68;
        }
    }
    SetTextColor16(hdc, (COLORREF)PTR_LOOP_1050_5b64);
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
    return;
}


u16 call_fn_ptr_1038_9ffa(HWND16 win_handle, u16 param_2, astruct_733 *struct_1, u16 param_4)

{
    code      **ppcVar1;
    astruct_43 *var_2;
    astruct_43 *var_3;
    HDC16       dev_ctx;
    u16         var_5;

    var_5   = struct_1->field_0x6;
    dev_ctx = GetDC16(win_handle);
    var_3   = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x3, param_2);
    var_2   = (astruct_43 *)var_3;
    ppcVar1 = &var_2->fn_ptr_field_0x8;
    (**ppcVar1)(0x1010, var_3, (var_3 >> 0x10), &dev_ctx, param_2, var_5);
    ppcVar1 = &var_2->fn_ptr_field_0x4;
    (**ppcVar1)(0x1010, var_3, 0x50005, &dev_ctx, param_2);
    ppcVar1 = &var_2->fn_ptr_field_0xc;
    (**ppcVar1)(0x1010, var_3, &dev_ctx, param_2);
    ReleaseDC16(0x1010, dev_ctx);
    return 0x0;
}


void unk_win_ui_op_1038_ac38(i1616 param_1, u16 param_2)

{
    u16         uVar1;
    i16         iVar2;
    i1616       IVar3;
    u32         uVar3;
    u16         extraout_DX;
    HWND16      hwnd;
    HWND16      hdc;
    u32         uVar5;
    COLORREF    color;
    u8          uVar4;
    astruct_46 *iVar1;

    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    GetStockObject16(param_1);
    if(_PTR_LOOP_1050_5b78 == 0x0)
    {
        hwnd                         = 0x1008;
        uVar5                        = pass1_1008_4d72(*(_PTR_LOOP_1050_4230 + 0xe));
        uVar1                        = (uVar5 >> 0x10);
        iVar2                        = uVar5;
        globals->_PTR_LOOP_1050_5b6c = CONCAT12(*(iVar2 + 0x3ec), CONCAT11(*(iVar2 + 0x3ed), *(iVar2 + 0x3ee)));
        globals->_PTR_LOOP_1050_5b70 = CONCAT12(*(iVar2 + 0x3e4), CONCAT11(*(iVar2 + 0x3e5), *(iVar2 + 0x3e6)));
        globals->_PTR_LOOP_1050_5b74 = CONCAT12(*(iVar2 + 0x3f8), CONCAT11(*(iVar2 + 0x3f9), *(iVar2 + 0x3fa)));
        globals->_PTR_LOOP_1050_5b78 = CONCAT12(*(iVar2 + 0x94), CONCAT11(*(iVar2 + 0x95), *(iVar2 + 0x96)));
    }
    if(param_2 < 0x4)
    {
    LAB_1038_acf0:
        hdc   = (HWND16)s_tile2_bmp_1050_1538;
        IVar3 = GetDlgCtrlID16(hwnd);
        if(IVar3 == 0xfd4)
        {
            color = (COLORREF)_PTR_LOOP_1050_5b70;
            goto LAB_1038_ad0e;
        }
        if(IVar3 != 0xfd5)
        {
            if(IVar3 == 0xfd6)
            {
                color = (COLORREF)_PTR_LOOP_1050_5b6c;
                goto LAB_1038_ad0e;
            }
            if(IVar3 == 0xfd7)
            {
                color = (COLORREF)_PTR_LOOP_1050_5b74;
                goto LAB_1038_ad0e;
            }
        }
    }
    else
    {
        hdc = hwnd;
        if(param_2 != 0x4)
        {
            if((param_2 == 0x4) || (0x1 < param_2 - 0x5))
            {
                return;
            }
            goto LAB_1038_acf0;
        }
    }
    color = (COLORREF)_PTR_LOOP_1050_5b78;
LAB_1038_ad0e:
    SetTextColor16(hdc, color);
    SetBkColor16((HDC16)s_tile2_bmp_1050_1538, 0x0);
    return;
}


void pass1_1038_ae08(Struct18 *param_1)

{
    param_1->field_0x0 = 0xae4e;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1038;
    unk_draw_op_1040_b0f8(param_1);
    return;
}


void pass1_1038_893a(Struct18 *param_1)

{
    u16 uVar1;

    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x8c2e;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    unk_draw_op_1040_b0f8(param_1);
    return;
}


void pass1_1038_8cf6(Struct18 *param_1)

{
    u16 uVar1;

    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x90c8;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    unk_draw_op_1040_b0f8(param_1);
    return;
}


void draw_op_1038_92f6(u16 param_1, u16 param_2, u16 param_3, u32 param_4, HWND16 param_5, u16 param_6)

{
    u32  uVar1;
    code      **ppcVar2;
    u16         uVar3;
    i16         iVar4;
    Struct18 *paVar5;
    u8         *in_DX;
    u8         *puVar6;
    u8         *puVar7;
    i16         unaff_DI;
    u16         uVar8;
    BOOL16      local_1a[0x2];
    u16         UStack22;
    Struct18 *paStack20;
    Struct18 *paStack16;
    i16         iStack12;
    Struct18 *paStack10;
    astruct_20 *paStack6;

    if(param_4._2_2_ == 0xeb)
    {
        paStack6 = (astruct_20 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, in_DX, unaff_DI);
        puVar6   = (paStack6 >> 0x10);
        paVar5   = *(Struct18 **)(param_1 + 0x90);
        if(paVar5 != (Struct18 *)0x0)
        {
            paStack10 = paVar5;
            mem_op_1000_179c(0x18, puVar6, 0x1000);
            uVar3     = paVar5;
            paStack16 = (Struct18 *)(paVar5 & 0xffff | ZEXT24(puVar6) << 0x10);
            puVar7    = (puVar6 | uVar3);
            if(puVar7 == 0x0)
            {
                uVar3  = 0x0;
                puVar7 = 0x0;
            }
            else
            {
                struct_1040_a598((paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
            }
            (param_1 + 0x90) = uVar3;
            (param_1 + 0x92) = puVar7;
            (param_1 + 0x90) = 0x11;
            iStack12         = (param_1 + 0x90);
            uVar3            = iStack12 * 0xa + 0x2;
            mem_op_1000_179c(uVar3, puVar7, 0x1000);
            paStack16 = (Struct18 *)CONCAT22(puVar7, uVar3);
            if((puVar7 | uVar3) == 0x0)
            {
                uVar1         = (param_1 + 0x90);
                (uVar1 + 0x2) = 0x0;
            }
            else
            {
                paStack16 = iStack12;
                pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iStack12, 0xa, uVar3 + 0x2, puVar7);
                uVar1         = (param_1 + 0x90);
                uVar8         = (uVar1 >> 0x10);
                iVar4         = uVar1;
                (iVar4 + 0x2) = uVar3 + 0x2;
                (iVar4 + 0x4) = puVar7;
            }
            uVar8          = (paStack10 >> 0x10);
            uVar1          = (param_1 + 0x90);
            (uVar1 + 0x6)  = (paStack10 + 0x6);
            uVar1          = (param_1 + 0x90);
            (uVar1 + 0xa)  = (paStack10 + 0xa);
            uVar1          = (param_1 + 0x90);
            (uVar1 + 0x12) = (param_1 + 0xa);
            uVar8          = 0x1010;
            pass1_1010_a50c(paStack6, 0x10505b42, *(param_1 + 0x90));
            paStack20 = paStack10;
            paStack16 = paStack10;
            if(paStack10 != (Struct18 *)0x0)
            {
                pass1_1040_a5d0(paStack10);
                uVar8 = 0x1000;
                fn_ptr_1000_17ce(paStack10, 0x1000);
            }
            ppcVar2 = (CONCAT22(param_2, param_1) + 0x70);
            (**ppcVar2)(uVar8, param_1, param_2);
        }
    }
    else
    {
        if(param_4._2_2_ != 0xf9)
        {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, &PTR_LOOP_1050_1040, param_6);
            return;
        }
        iVar4 = pass1_1038_993a(param_1, param_2, param_3);
        if(-0x1 < iVar4)
        {
            uVar8    = (param_1 + 0x6);
            UStack22 = GetDlgItemi1616(param_5, 0x1, local_1a, param_6);
            if(local_1a[0] != 0x0)
            {
                uVar1 = (param_1 + 0x98);
                draw_fn_1010_2a32(0x94be, s_tile2_bmp_1050_1538, uVar1, (uVar1 >> 0x10), UStack22, CONCAT22(uVar8, (iVar4 * 0xe + 0x5a72)), in_DX, param_1, &stack0xfffe, param_2);
            }
        }
    }
    return;
}


Struct18 *pass1_1038_997c(Struct18 *param_1, u8 param_2)

{
    unk_draw_op_1040_b0f8(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}
