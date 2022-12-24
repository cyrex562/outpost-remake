use crate::block_1000::block_1000_3000::str_op_1000_3da4;
use crate::block_1008::block_1008_4000;
use crate::block_1008::block_1008_4000::pass1_1008_4d72;
use crate::block_1010::block_1010_8000::FUN_1010_830a;
use crate::file_ops;
use crate::globals::DAT_1050_1050;
use crate::structs::struct_57::Struct57;
use crate::utils::{CONCAT11, CONCAT22};
use crate::winbase::{CreateDC16, DeleteDC16, DeleteObject16, GetStockObject16, SelectPalette16, SetBkColor16, SetTextColor16, TextOut16};
use crate::windef::{COLORREF, HDC16, HGDIOBJ16, HPALETTE16, HWND16, RECT16};

pub unsafe fn draw_op_1020_7cc8(mut param_1: u16, struct_e_param_1: *mut StructE) {
    let mut y_00: i16;
    let mut str46_len: i16;
    let mut x: i16;
    let mut iVar3: i16;
    let mut brush_handle_2: HGDIOBJ16;
    let mut width: i16;
    let mut iVar9: i16;
    let mut puVar6: *mut u32;
    let mut iVar5: i16;
    let mut handle: HPEN16;
    let mut string_1: *mut c_char;
    let mut y: i16;
    let mut extraout_DX: u16;
    let mut iVar6: i16;
    let mut struct_e_1: *mut StructE;
    let mut iVar7: *mut astruct_781;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut DVar1: u32;
    let mut rect: *mut RECT16;
    let mut hdc: HDC16;
    let mut iVar2: i16;
    let mut string_46: *mut c_char;
    let mut local_rect_1: i16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut iStack14: i16;
    let mut hpalette_12: HPALETTE16;
    let mut paStack10: *mut astruct_13;
    let mut win_hdc_1: HDC16;
    let mut is_iconic: bool;
    let mut puVar2: *mut u32;
    let mut style: i16;
    let mut iVar8: i16;
    let mut iVar1: i16;
    let mut fn_ptr_1: *mut *mut code;

    uVar7 = (struct_e_param_1 >> 0x10);
    struct_e_1 = struct_e_param_1;
    is_iconic = IsIconic16(struct_e_1.hwnd16_field4_0x4);
    if ((is_iconic == 0) || (PTR_LOOP_1050_0010.is_null() == false)) {
        win_hdc_1 = GetWindowDC16(struct_e_1.hwnd16_field4_0x4);
        paStack10 = (_PTR_LOOP_1050_4230 + 0xe);
        hpalette_12 = palette_op_1008_4e08(&win_hdc_1, param_1, paStack10, CONCAT22(0x1050, &win_hdc_1));
        GetWindowRect16(
            CONCAT22(0x1050, &local_rect_1),
            struct_e_1.hwnd16_field4_0x4,
        );
        x = (iStack16 - local_rect_1) - 0x1;
        y = (iStack14 - iStack18) - 0x1;
        iVar1 = struct_e_1.field12_0x12;
        iVar3 = y;
        if (is_iconic == 0) {
            iVar3 = struct_e_1.field10_0xe - struct_e_1.field12_0x12;
        }
        rect = &DAT_1050_1050;
        hdc = win_hdc_1;
        iVar2 = y;
        brush_handle_2 = GetStockObject16(BLACK_BRUSH);
        FillRect16(brush_handle_2, rect, hdc);
        puVar2 = struct_e_1.field5_0x6;
        uVar8 = (puVar2 >> 0x10);
        iVar7 = puVar2;
        puVar6 = &iVar7.field_0xe0;
        style = iVar7.field226_0xe2;
        width = puVar6;
        fn_ptr_1 = (*puVar6 + 0x24);
        (**fn_ptr_1)(s_tile2_bmp_1050_1538, width, style, 0x0, 0x0, iVar2);
        iVar5 = (-(puVar6 == 0) & 0x1e) + 0x25;
        handle = CreatePen16(CONCAT22(0x100, iVar5), width, style);
        brush_handle_2 = SelectObject16(handle, win_hdc_1);
        MoveTo16(0x0, 0x0, win_hdc_1);
        LineTo16(0x0, x, win_hdc_1);
        LineTo16(y, x, win_hdc_1);
        LineTo16(y, 0x0, win_hdc_1);
        string_1 = LineTo16(0x0, 0x0, win_hdc_1);
        if (is_iconic == 0) {
            y_00 = struct_e_1.field10_0xe - struct_e_1.field12_0x12;
            MoveTo16(y_00, 0x0, win_hdc_1);
            string_1 = LineTo16(y_00, x, win_hdc_1);
        }
        fn_ptr_1 = (*struct_e_1.field5_0x6 + 0x18);
        (**fn_ptr_1)(s_tile2_bmp_1050_1538);
        string_46 = CONCAT22(extraout_DX, string_1);
        if (*string_1 != '\0') {
            SetBkColor16(0x0, win_hdc_1);
            SetTextColor16(CONCAT22(0x100, iVar5), win_hdc_1);
            str46_len = lstrlen16(string_46);
            DVar1 = GetTextExtent16(
                str46_len,
                CONCAT13((extraout_DX >> 0x8), CONCAT12(extraout_DX, string_1)),
                win_hdc_1,
            );
            iVar6 = (DVar1 >> 0x10);
            if (is_iconic == 0) {
                iVar9 = (iVar3 - iVar1) / 0x2 - iVar6 / 0x2;
                iVar8 = x / 0x2 - DVar1 / 0x2;
            } else {
                iVar9 = y / 0x2 - iVar6 / 0x2;
                iVar8 = 0x2;
            }
            TextOut16(
                iVar9,
                CONCAT22(extraout_DX, string_1),
                iVar9,
                iVar8,
                win_hdc_1,
            );
        }
        hpalette_12 = SelectPalette16(0x0, hpalette_12, win_hdc_1);
        DeleteObject16(hpalette_12);
        SelectObject16(brush_handle_2, win_hdc_1);
        DeleteObject16(handle);
        ReleaseDC16(win_hdc_1, struct_e_1.hwnd16_field4_0x4);
    }
    return;
}


pub unsafe fn unk_draw_op_1020_7f7a(
    param_1: *mut astruct_20,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u16,
) {
    let mut HVar1: HGDIOBJ16;
    let mut hcursor2: HCURSOR16;
    let mut in_EDX: u32;
    let mut uVar3: u16;
    let mut paVar2: *mut Struct57;
    let mut struct_1: *mut astruct_20;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut paVar4: *mut astruct_20;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe90: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffe8: u16;
    let mut uVar1: u16;

    uVar3 = (in_EDX >> 0x10);
    paVar4 = unk_draw_op_1008_61b2(
        in_stack_0000ffbc,
        param_1,
        param_2,
        param_3,
        CONCAT22(param_4, param_3),
    );
    paVar2 = CONCAT22(uVar3, (paVar4 >> 0x10));
    uVar4 = (param_1 >> 0x10);
    struct_1 = param_1;
    (struct_1 + 1).offset_0x0 = 0x389a;
    struct_1[0x1].base_0x2 = 0x1008;
    (struct_1 + 1).offset_0x0 = 0x3aa8;
    struct_1[0x1].base_0x2 = 0x1008;
    struct_1[0x1].field2_0x4 = 0;
    struct_1[0x1].field3_0x8 = 0;
    struct_1[0x1].field4_0xa = 0;
    param_1.offset_0x0 = 0x82bc;
    struct_1.base_0x2 = 0x1020;
    (struct_1 + 1).offset_0x0 = 0x8358;
    struct_1[0x1].base_0x2 = 0x1020;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&struct_1.field60_0x5b)),
        s_VrMode_1050_4422,
    );
    HVar1 = GetStockObject16(HOLLOW_BRUSH);
    struct_1.hgdiobj_field_0xc6 = HVar1;
    hcursor2 = LoadCursor16(0x7f00, 0x0);
    struct_1.hcursor_field_0xc4 = hcursor2;
    struct_1.field150_0xc8 = 0x2028;
    struct_1.field139_0xac = 0x47000000;
    struct_1.field145_0xbc = (param_3 + 0x8);
    puVar5 = mixed_1010_20ba(
        paVar2,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffe8, 0x48),
        in_stack_0000fe90,
        in_stack_0000ffb4,
        in_stack_0000ffba,
        in_stack_0000ffbe,
    );
    uVar1 = (puVar5 >> 0x10);
    struct_1.field141_0xb4 = 0;
    struct_1.field142_0xb6 = 0;
    struct_1.field143_0xb8 = (puVar5 + 0xa);
    struct_1.field144_0xba = (puVar5 + 0xc);
    struct_1.field151_0xca = param_3;
    win_ui_reg_class_1008_96d2(param_1);
    return;
}


pub unsafe fn draw_op_1040_7bb2(in_struct_1: *mut astruct_14)

{
  let mut is_iconic: bool;
  let mut x: i16;
  let mut y1: i16;
  let mut iVar5: i16;
  let mut pen_handle: HPEN16;
  let mut obj_handle: HGDIOBJ16;
  let mut y: i16;
  let mut brush_handle: HGDIOBJ16;
  let mut handle: HANDLE16;
  let mut count: u16;
  let mut count_00: i16;
   let mut struct_1: *mut astruct_14;
   let mut pRVar1: *mut RECT16;
  let mut win_long: i32;
  let mut DVar2: u32;
   let mut rect: *mut RECT16;
  let mut count_01: i16;
  let mut local_rect_12: i16;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut HStack10: HPALETTE16;
  let mut uStack8: u32;
  let mut win_hdc16_4: HDC16;
  let mut uVar5: u8;
  let mut uVar6: u8;
  let mut uVar7: u32;
  let mut pcVar3: *mut u8;
  let mut uVar3: u16;
  let mut func_ptr_1: *mut *mut code;
  let mut hdc16_dev_ctx_1: HDC16;

  pRVar1 = (in_struct_1 >> 0x10);
  struct_1 = in_struct_1;
  is_iconic = IsIconic16(struct_1.hwnd16_field6_0x6);
  if (is_iconic == 0) {
    win_hdc16_4 = GetWindowDC16(struct_1.hwnd16_field6_0x6);
    func_ptr_1 = (in_struct_1 + 0x68);
    uStack8 = (**func_ptr_1)(s_tile2_bmp_1050_1538,in_struct_1);
    if (uStack8.is_null() == false) {
      HStack10 = palette_op_1008_4e08
                           (&win_hdc16_4,(uStack8 >> 0x10) | uStack8,uStack8,
                            CONCAT22(0x1050,&win_hdc16_4));
      GetWindowRect16(CONCAT13(0x10,CONCAT12(0x50,&local_rect_12)),struct_1.hwnd16_field6_0x6);
      x = (iStack14 - local_rect_12) -0x1;
      y1 = (iStack12 - iStack16) -0x1;
      iVar5 = (-(struct_1.field95_0x60 == 0) & 0x1e) + 0x25;
      pen_handle = CreatePen16(CONCAT13(0x1,iVar5),0x0,0x0);
      obj_handle = SelectObject16(pen_handle,win_hdc16_4);
      MoveTo16(0x0,0x0,win_hdc16_4);
      LineTo16(0x0,x,win_hdc16_4);
      LineTo16(y1,x,win_hdc16_4);
      LineTo16(y1,0x0,win_hdc16_4);
      LineTo16(0x0,0x0,win_hdc16_4);
      win_i32 = GetWindowLong16(-0x10,struct_1.hwnd16_field6_0x6);
      if (((win_i32 & 0x800000) != 0) && ((win_i32 & 0x400000) != 0)) {
        y = struct_1.field96_0x62 - struct_1.field98_0x66;
        MoveTo16(y,0x0,win_hdc16_4);
        LineTo16(y,x,win_hdc16_4);
        struct_1.field115_0x7a = struct_1.field97_0x64;
        struct_1.field116_0x7c = struct_1.field98_0x66;
        struct_1.field117_0x7e = x;
        struct_1.field118_0x80 = struct_1.field96_0x62 - struct_1.field98_0x66;
        rect = pRVar1;
        hdc16_dev_ctx_1 = win_hdc16_4;
        brush_handle = GetStockObject16(BLACK_BRUSH);
        FillRect16(brush_handle,rect,hdc16_dev_ctx_1);
        if (struct_1.field112_0x76 != 0) {
          draw_op_1040_82ee(in_struct_1);
        }
        if (struct_1.field15_0x10 != '\0') {
          handle = GetProp16(s_hfont_1050_5de9,struct_1.hwnd16_field6_0x6);
          if (handle != 0) {
            SelectObject16(handle,win_hdc16_4);
          }
          SetBkColor16(0x0,win_hdc16_4);
          count_01 = 0x100;
          hdc16_dev_ctx_1 = win_hdc16_4;
          SetTextColor16(CONCAT22(0x100,iVar5),win_hdc16_4);
          count = lstrlen16(CONCAT22(hdc16_dev_ctx_1,count_01));
          DVar2 = GetTextExtent16(count,CONCAT22(count_01,win_hdc16_4),win_hdc16_4);
          count_00 = ((struct_1.field117_0x7e - struct_1.field115_0x7a) / 0x2 - DVar2 / 0x2) +
                     struct_1.field115_0x7a;
          brush_handle = (struct_1.field118_0x80 - struct_1.field116_0x7c) / 0x2 - (DVar2 >> 0x10) / 0x2;
          TextOut16(count_01,CONCAT22(count_01,win_hdc16_4),brush_handle,count_00,win_hdc16_4);
          if (count_00 != 0) {
            SelectObject16(brush_handle,win_hdc16_4);
          }
        }
      }
      func_ptr_1 = (in_struct_1 + 0x64);
      (**func_ptr_1)(s_tile2_bmp_1050_1538,in_struct_1,uStack8,win_hdc16_4);
      HStack10 = SelectPalette16(0x0,HStack10,win_hdc16_4);
      DeleteObject16(HStack10);
      SelectObject16(obj_handle,win_hdc16_4);
      DeleteObject16(pen_handle);
    }
    ReleaseDC16(win_hdc16_4,struct_1.hwnd16_field6_0x6);
    return;
  }
  return;
}


pub unsafe fn set_text_bk_color_1040_7e5e(param_1: u32, mut param_2: u16, mut param_3: u16, param_4: HDC16) -> u32

{
  let mut HVar1: HGDIOBJ16;
  let mut iVar2: *mut astruct_936;
  let mut IVar2: i16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut hdc: HDC16;
  let mut fn_ptr_1: *mut *mut code;

  HVar1 = GetStockObject16(BLACK_BRUSH);
  if (COLORREF_1050_5df0 == 0) {
    fn_ptr_1 = (*param_1 + 0x68);
    uVar3 = (**fn_ptr_1)(s_tile2_bmp_1050_1538,param_1,(param_1 + 0x6e));
    if (uVar3 == 0) {
      return 0x0;
    }
    uVar3 = pass1_1008_4d72(uVar3);
    uVar4 = (uVar3 >> 0x10);
    iVar2 = uVar3;
    COLORREF_1050_5df0 = CONCAT22(CONCAT11(0x2,iVar2.field_0x94),CONCAT11(iVar2.field_0x95,iVar2.field_0x96));
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return 0x0;
    }
    uVar4 = 0x7ed5;
    IVar2 = GetDlgCtrlID16(param_2);
    if (IVar2 == 0x14c) {
      uVar5 = 0xffff;
      hdc = 0;
  // TODO: goto LAB_1040_7f00;
    }
    if (IVar2 == 0x175) {
      uVar5 = 0xff;
      hdc = 0;
  // TODO: goto LAB_1040_7f00;
    }
  }
  uVar4 = COLORREF_1050_5df0;
  uVar5 = (COLORREF_1050_5df0 >> 0x10);
  hdc = param_4;//
// LAB_1040_7f00:
  SetTextColor16(CONCAT22(uVar5,uVar4),hdc);
  SetBkColor16(0x1000000,param_4);
  return CONCAT22(0x1050,HVar1);
}


pub unsafe fn file_and_draw_op_1008_4f20(
    param_1: *mut Struct57,
    param_2: *mut astruct_76,
    mut param_3: u32,
    mut param_4: u16,
    param_5: *mut c_char,
    mut param_6: u16,
    mut param_7: u16,
) {
    let mut uVar1: u16;
    let mut hdc: HDC16;
    let mut uVar2: u16;
    let mut count: i16;
    let mut uVar4: u16;
    let mut extraout_DX: u16;
    let mut hpalette_a: HPALETTE16;
    let mut struct_1: *mut astruct_76;
    let mut uVar6: u16;
    let mut unaff_CS: u16;
    let mut uVar3: u32;
    let mut color: COLORREF;
    let mut struct81_26: astruct_81;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut offset: u16;
    let mut segment: u16;
    let mut hdc_b: u16;
    let mut uVar14: u8;
    let mut uVar15: u8;
    struct uVar17;
    let mut hdc_a: HDC16;

    block_1008_4000::pass1_1008_4016(param_2);
    uVar6 = (param_2 >> 0x10);
    struct_1 = param_2;
    struct_1.lpcstr_field13_0x1e = param_5;
    struct_1.field15_0x22 = param_4;
    struct_1.field16_0x24 = param_3;
    // 0x50a2
    param_2.offset_0x0 = s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x9;
    struct_1.base_0x2 = 0x1008;
    uVar1 = FUN_1010_830a(param_3, param_1, unaff_CS, _u16_1050_14cc, 0x2);
    block_1008_4000::struct_op_1008_48fe(
        param_1,
        CONCAT22(0x1050, &struct81_26),
        0x1,
        CONCAT22(param_1, uVar1),
    );
    file_ops::read_file_1008_49e8(param_6, param_7, CONCAT22(0x1050, &struct81_26), param_1);
    block_1008_4000::pass1_1008_5068(param_2, CONCAT22(0x1050, &struct81_26));
    block_1008_4000::pass1_1008_47cc(param_2);
    block_1008_4000::pass1_1008_4834(param_2);
    segment = &DAT_1050_1050;
    offset = 0x27e;
    uVar10 = 0;
    uVar11 = 0;
    uVar8 = 0;
    uVar9 = 0;
    uVar3 = block_1008_4000::pass1_1008_4772(param_2);
    uVar4 = (uVar3 >> 0x10);
    hdc = CreateDC16(
        (uVar3 & 0xffff | uVar4 << 0x10),
        CONCAT22(uVar9, uVar8),
        CONCAT22(uVar11, uVar10),
        CONCAT22(segment, offset),
    );
    uVar2 = block_1008_4000::palette_op_1008_46e4(&stack0xffd4, uVar4, param_2, CONCAT22(0x1050, &stack0xffd4));
    color = SetBkColor16(0xffffff, hdc);
    SetTextColor16(CONCAT22(0x100, struct_1.field15_0x22), hdc);
    count = str_op_1000_3da4(struct_1.lpcstr_field13_0x1e);
    TextOut16(count, struct_1.lpcstr_field13_0x1e, 0x0, 0x0, hdc);
    uVar1 = (color >> 0x10);
    hdc_a = hdc;
    SetBkColor16(color, hdc);
    SetTextColor16(CONCAT22(hdc, uVar1), hdc_a);
    hpalette_a = SelectPalette16(0x0, uVar2, hdc_a);
    DeleteObject16(hpalette_a);
    DeleteDC16(hdc_a);
    file_ops::close_file_1008_496c(CONCAT22(0x1050, &struct81_26));
    return;
}


// WARNING: Variable defined which should be unmapped: iStack22
// WARNING: Variable defined which should be unmapped: iStack20
pub unsafe fn draw_text_1018_c742(
    mut param_1: u16,
    struct36_param_1: *mut astruct_36,
    hdc_2: HDC16,
    count_param_3: i16,
    mut param_5: u16,
) {
    let mut piVar2: *mut i16;
    let mut iVar3: i16;
    let mut extraout_AH: u8;
    let mut uVar3: u8;
    let mut iVar5: i16;
    let mut iVar1: i16;
    let mut pstruct36_4: *mut astruct_36;
    let mut pstruct36_hi: *mut astruct_36;
    let mut color: COLORREF;
    let mut iStack22: i16;
    let mut iStack20: i16;
    let mut rect_12: RECT16;
    let mut iStack14: i16;
    let mut iStack12: i16;
    let mut piVar1: *mut i16;

    pstruct36_hi = (struct36_param_1 >> 0x10);
    pstruct36_4 = struct36_param_1;
    if ((pstruct36_4.string_0x108.is_null() == false) && (*pstruct36_4.string_0x108 != '\0')) {
        uVar3 = SetTextColor16(0x8000, *_hdc_2);
        color = SetBkColor16(0x2000000, *_hdc_2);
        if (pstruct36_4.field247_0x106 == 0) {
            iVar3 = pstruct36_4.field250_0x10e;
            DrawText16(
                0x410,
                CONCAT22(0x1050, &stack0xffe6),
                -0x1,
                pstruct36_4.string_0x108,
                *_hdc_2,
            );
            pstruct36_4.field244_0x100 = (0x280 - iVar3) / 0x2;
            pstruct36_4.field245_0x102 = pstruct36_4.field249_0x10c;
            pstruct36_4.field246_0x104 = pstruct36_4.field244_0x100 + iVar3;
            iVar3 = pstruct36_4.field245_0x102;
            pstruct36_4.field247_0x106 = iVar3;
            piVar1 = &pstruct36_4.field240_0xf8;
            if (*piVar1 == iVar3 || *piVar1 < iVar3) {
                iVar1 = iVar3 - pstruct36_4.field240_0xf8;
                piVar2 = &pstruct36_4.field245_0x102;
                *piVar2 = *piVar2 - iVar1;
                piVar2 = &pstruct36_4.field247_0x106;
                *piVar2 = *piVar2 - iVar1;
            }
        }
        rect_12.x = pstruct36_4.field241_0xfa + pstruct36_4.field244_0x100;
        rect_12.y = pstruct36_4.field242_0xfc + pstruct36_4.field245_0x102;
        DrawText16(
            0x10,
            CONCAT22(0x1050, &rect_12),
            -0x1,
            pstruct36_4.string_0x108,
            *_hdc_2,
        );
        SetTextColor16(CONCAT22(param_1, CONCAT11(extraout_AH, uVar3)), *_hdc_2);
        SetBkColor16(color, *_hdc_2);
    }
    return;
}


pub unsafe fn draw_text_1040_94fc(struct_param_1: *mut Struct37, hdc_param_2: HDC16)

{
   let mut struct_1: *mut Struct37;
  let mut struct_1_lo: u16;
  let mut colorref_2: COLORREF;
  let mut u16_var_1: u16;
  let mut u16_var3: u16;

  struct_1_lo = (struct_param_1 >> 0x10);
  struct_1 = struct_param_1;
  colorref_2 = SetBkColor16(CONCAT22(0x100,struct_1.field49_0x3a),hdc_param_2);
  SetTextColor16(CONCAT22(0x100,struct_1.field50_0x3c),hdc_param_2);
  DrawText16(0x10,(struct_param_1 & 0xffff0000 | ZEXT24(&struct_1.field40_0x2e)),-0x1,
             struct_1.field1_0x4,hdc_param_2);
  u16_var_1 = ((colorref_2 & 0xffff0000) >> 0x10);
  u16_var3 = hdc_param_2;
  SetBkColor16(colorref_2 & 0xffff0000 | hdc_param_2,hdc_param_2);
  SetTextColor16(CONCAT22(u16_var_1,u16_var3),hdc_param_2);
  return;
}

pub unsafe fn draw_text_1040_9650(param_1: *mut astruct_65)

{
  let mut hdc: HDC16;

  hdc = GetDC16(0x0);
  DrawText16(0x410,(param_1 & 0xffff0000 | (param_1 + 0x2e)),-0x1,
             (param_1 + 0x4),hdc);
  ReleaseDC16(hdc,0x0);
  return;
}

pub unsafe fn draw_op_1040_9948(mut param_1: u16, param_2: *mut astruct_71)

{
  let mut hdc16_dev_ctx_1: HDC16;
  let mut mode: i16;
  let mut uVar3: u16;
  let mut handle: HPEN16;
  let mut hgdiobj16_00: HPEN16;
  let mut hgdiobj_2: HGDIOBJ16;
  let mut hdc_lt_gray_brush_1: HGDIOBJ16;
  let mut cch_1: u16;
  let mut puVar1: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
   let mut struct71_var4: *mut astruct_71;
  let mut uVar7: u16;
  let mut pcVar1: *mut c_char;
  let mut uVar2: u16;
  let mut HVar3: HDC16;
  let mut iStack88: i16;
  let mut x1: i16;
  let mut y2: i16;
  let mut x2: i16;
  let mut y1: i16;
  let mut iStack78: i16;
  let mut paintstruct_42: [u8;0x20] = [0;0x20];
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut hgdiobj_1: HGDIOBJ16;
  let mut iStack28: i16;
  let mut x4: i16;
  let mut y6: i16;
  let mut x5: i16;
  let mut y4: i16;
  let mut rect_12: RECT16;
  let mut x3: i16;
  let mut y3: i16;
  let mut rect_a: i16;
  let mut iStack8: i16;
  let mut x6: i16;
  let mut y7: i16;
  let mut iVar1: i16;
  let mut iVar2: *mut astruct_782;
  let mut uVar8: u8;
  let mut uVar9: u8;
  let mut uVar14: u16;
  let mut uVar10: u8;
  let mut uVar11: u8;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar4_00: u16;

  x6 = 0x1;
  y7 = 0x1;
  rect_a = 0;
  iStack8 = 0;
  iStack28 = 0;
  hgdiobj_1 = 0;
  uVar7 = (param_2 >> 0x10);
  struct71_var4 = param_2;
  uStack32 = struct71_var4.field4_0x4 & 0x8;
  uStack34 = struct71_var4.field86_0x56 & 0x1;
  hdc16_dev_ctx_1 = BeginPaint16(CONCAT22(0x1050,paintstruct_42),param_1);
  mode = SetMapMode16(0x1,hdc16_dev_ctx_1);
  GetClientRect16(&rect_12,&DAT_1050_1050);
  iVar2 = (_x3 >> 0x10);
  _x3 = CONCAT22(iVar2 -0x1,x3 -1);
  if (uStack34 != 0) {
    x4 = rect_12;
    y6 = (rect_12 >> 0x10);
    rect_12 = CONCAT22(y6 + 0x2,x4 + 2);
    _x3 = CONCAT22(iVar2 -0x3,x3 -0x3);
    x5 = x3 -0x1;
    y4 = (iVar2 -1);
  }
  if (struct71_var4.field6_0x6 != '\0') {
    iStack28 = 0x1;
    if (struct71_var4.hgdiobj_field90_0x5a != 0) {
      hgdiobj_1 = SelectObject16(struct71_var4.hgdiobj_field90_0x5a,hdc16_dev_ctx_1);
    }
    pcVar1 = &struct71_var4.field6_0x6;
    uVar2 = uVar7;
    HVar3 = hdc16_dev_ctx_1;
    uVar3 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(pcVar1)));
    DrawText16(0x400,CONCAT22(0x1050,&rect_a),uVar3,CONCAT22(uVar2,pcVar1),HVar3);
    iStack8 = ((y3 - y7) + iStack8) / 0x2 + rect_12.y;
    y7 += iStack8;
    rect_a = ((x3 - x6) + rect_a) / 0x2 + rect_12.x;
    x6 += rect_a;
  }
    // 1050:5ec2
  handle = CreatePen16(COLORREF_1050_5ec2,0x1,0x0);
    // 1050:5ebe
  hgdiobj16_00 = CreatePen16(COLORREF_1050_5ebe,0x1,0x0);
  hgdiobj_2 = SelectObject16(handle,hdc16_dev_ctx_1);
  if (uStack34 != 0) {
    iStack78 = 0;
    loop {
      MoveTo16(y4,x4,hdc16_dev_ctx_1);
      LineTo16(y4,x5,hdc16_dev_ctx_1);
      LineTo16(y6,x5,hdc16_dev_ctx_1);
      LineTo16(y6,x4,hdc16_dev_ctx_1);
      LineTo16(y4,x4,hdc16_dev_ctx_1);
      y6 += 0x1;
      x4 += 0x1;
      x5 += -0x1;
      y4 += -0x1;
      iStack78 += 0x1;
      if iStack78 >= 1 {break;}
    }
  }
  if ((struct71_var4.field4_0x4 & 0x2) == 0) {
    y2 = (rect_12 >> 0x10);
    x2 = _x3;
    iStack78 = 0;
    x1 = rect_12.x;
    y1 = y3;
    loop {
      SelectObject16(handle,hdc16_dev_ctx_1);
      MoveTo16(y1,x1,hdc16_dev_ctx_1);
      LineTo16(y1,x2,hdc16_dev_ctx_1);
      LineTo16(y2,x2,hdc16_dev_ctx_1);
      loop {
        SelectObject16(hgdiobj16_00,hdc16_dev_ctx_1);
        LineTo16(y2,x1,hdc16_dev_ctx_1);
        LineTo16(y1,x1,hdc16_dev_ctx_1);
        if !((hdc16_dev_ctx_1 + 1) < 0x2) {break;}
      }
      y2 += 0x1;
      x1 += 0x1;
      x2 += -0x1;
      y1 += -0x1;
      iStack78 += 0x1;
      if iStack78 >= 0x2 {break;}
    }
  }
  else {
    MoveTo16(y3,rect_12.x,hdc16_dev_ctx_1);
    LineTo16(rect_12.y,rect_12.x,hdc16_dev_ctx_1);
    LineTo16(rect_12.y,x3 + 0x1,hdc16_dev_ctx_1);
    if (iStack28 != 0) {
      iStack8 += 0x2;
      rect_a += 0x2;
      x6 += 0x2;
      y7 += 0x2;
    }
  }
  MoveTo16(0x0,0x0,hdc16_dev_ctx_1);
  if (iStack28 != 0) {
    if ((struct71_var4.field4_0x4 & 0x4) == 0) {
      uVar4 = u16_1050_5ec8;
      puVar1 = u16_1050_5ec6;
      if (uStack32 != 0) {
        uVar4 = u16_1050_5ecc;
        puVar1 = u16_1050_5eca;
      }
      SetTextColor16(CONCAT22(uVar4,puVar1),hdc16_dev_ctx_1);
      SetBkColor16(0x1000000,hdc16_dev_ctx_1);
      pcVar1 = &struct71_var4.field6_0x6;
      HVar3 = hdc16_dev_ctx_1;
      uVar6 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(pcVar1)));
      DrawText16(0x1,CONCAT22(0x1050,&rect_a),uVar6,CONCAT22(uVar7,pcVar1),HVar3);
      uVar13 = s_tile2_bmp_1050_1538;
      uVar14 = 0x9c8d;
      SetTextColor16(CONCAT22(HVar3,uVar7),hdc16_dev_ctx_1);
      SetBkColor16(CONCAT22(uVar13,uVar14),hdc16_dev_ctx_1);
    }
    else {
      hdc_lt_gray_brush_1 = GetStockObject16(LTGRAY_BRUSH);
      uVar4_00 = 0;
      uVar12 = 0;
      pcVar1 = &struct71_var4.field6_0x6;
      HVar3 = hdc16_dev_ctx_1;
      cch_1 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(pcVar1)));
      GrayString16(y7 - iStack8,x6 - rect_a,iStack8,rect_a,cch_1,CONCAT22(uVar7,pcVar1),
                   CONCAT22(uVar12,uVar4_00),hdc_lt_gray_brush_1,HVar3);
    }
    if (hgdiobj_1 != 0) {
      SelectObject16(hgdiobj_1,hdc16_dev_ctx_1);
    }
  }
  SelectObject16(hgdiobj_2,hdc16_dev_ctx_1);
  SetMapMode16(mode,hdc16_dev_ctx_1);
  DeleteObject16(handle);
  DeleteObject16(hgdiobj16_00);
  EndPaint16(CONCAT22(0x1050,paintstruct_42),param_1);
  return;
}


pub unsafe fn draw_op_1038_9dcc(in_struct_1: *mut astruct_10, mut param_2: i16, mut param_3: u16, in_hdc_param_4: u16, mut param_5: u16 )

{
  let mut bVar1: bool;
  let mut local_brush_handle: HBRUSH16;
   let mut struct10_5: *mut astruct_10;
   let mut struct10_5_hi: *mut astruct_10;
  let mut uVar3: u32;
  let mut uStack14: u16;
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut iVar2: *mut astruct_749;

  struct10_5_hi = (in_struct_1 >> 0x10);
  struct10_5 = in_struct_1;
  if (struct10_5.brush_handle_field_0x8e == 0) {
    local_brush_handle = CreateSolidBrush16(WHITE_BRUSH);
    struct10_5.brush_handle_field_0x8e = local_brush_handle;
  }
  if (_u16_1050_5b64 == 0) {
    uVar3 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
    uVar2 = (uVar3 >> 0x10);
    iVar2 = uVar3;
    _u16_1050_5b64 = CONCAT12(iVar2.field_0x94,CONCAT11(iVar2.field_0x95,iVar2.field_0x96));
    u16_1050_5b68 = CONCAT11(iVar2.field_0x3e5,iVar2.field_0x3e6);
    u16_1050_5b6a = iVar2.field996_0x3e4;
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return;
    }
    bVar1 = false;
    // for (uStack14 = 0; puVar1 = &struct10_5.field295_0x128, uStack14 <= *puVar1 && *puVar1 != uStack14;
    //     uStack14 += 1)
    uStack14 = 0;
    puVar1 = struct10_5.fiel295_0x128;
    while uStack14 <= *puvar1 && *puvar1 != uStack14

        {
      if ((&struct10_5.field_0x94 + uStack14 * 0x2) == param_2) {
        bVar1 = true;
        break;
      }
      uStack14 += 1;
    }
    if (bVar1) {
      u16_1050_5b64 = u16_1050_5b68;
      u16_1050_5b66 = u16_1050_5b6a;
    }
  }
  SetTextColor16(CONCAT22(u16_1050_5b66,u16_1050_5b64),in_hdc_param_4);
  SetBkColor16(0x1000000,in_hdc_param_4);
  return;
}

pub unsafe fn unk_win_ui_op_1038_ac38(mut param_1: u16, mut param_2: u16, mut param_3: u16, mut param_4: u16, hdc_param_5: HDC16)

{
  let mut IVar1: i16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar6: u32;
  let mut uVar4: u8;
  let mut iVar1: *mut astruct_46;
  let mut iVar2: *mut astruct_786;
  let mut uVar2: u16;
  let mut uVar5: u16;
  let mut uVar1: u16;

  GetStockObject16(BLACK_BRUSH);
  if (_u16_1050_5b78 == 0) {
    uVar6 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
    uVar1 = (uVar6 >> 0x10);
    iVar2 = uVar6;
    _u16_1050_5b6c = CONCAT12(iVar2.field_0x3ec,CONCAT11(iVar2.field_0x3ed,iVar2.field_0x3ee));
    _u16_1050_5b70 = CONCAT12(iVar2.field_0x3e4,CONCAT11(iVar2.field_0x3e5,iVar2.field_0x3e6));
    _u16_1050_5b74 = CONCAT12(iVar2.field_0x3f8,CONCAT11(iVar2.field_0x3f9,iVar2.field_0x3fa));
    _u16_1050_5b78 = CONCAT12(iVar2.field_0x94,CONCAT11(iVar2.field_0x95,iVar2.field_0x96));
  }
  if (param_4 < 0x4) {//
// LAB_1038_acf0:
    IVar1 = GetDlgCtrlID16(param_3);
    if (IVar1 == 0xfd4) {
      uVar2 = _u16_1050_5b70;
      uVar5 = (_u16_1050_5b70 >> 0x10);
  // TODO: goto LAB_1038_ad0e;
    }
    if (IVar1 != 0xfd5) {
      if (IVar1 == 0xfd6) {
        uVar2 = _u16_1050_5b6c;
        uVar5 = (_u16_1050_5b6c >> 0x10);
    // TODO: goto LAB_1038_ad0e;
      }
      if (IVar1 == 0xfd7) {
        uVar2 = _u16_1050_5b74;
        uVar5 = (_u16_1050_5b74 >> 0x10);
    // TODO: goto LAB_1038_ad0e;
      }
    }
  }
  else if (param_4 != 0x4) {
    if ((param_4 == 0x4) || (0x1 < param_4 - 0x5)) {
      return;
    }
// TODO: goto LAB_1038_acf0;
  }
  uVar2 = _u16_1050_5b78;
  uVar5 = (_u16_1050_5b78 >> 0x10);//
// LAB_1038_ad0e:
  SetTextColor16(CONCAT22(uVar5,uVar2),hdc_param_5);
  SetBkColor16(0x1000000,hdc_param_5);
  return;
}

pub unsafe fn set_text_bk_color_1040_0cc0(
    param_1: u32,
    mut param_2: u16,
    mut param_3: u16,
    hwnd_param_4: HWND16,
) -> u32 {
    let mut iVar1: *mut astruct_783;
    let mut uVar3: u16;
    let mut uVar1: u32;
    let mut hobject: HGDIOBJ16;
    let mut fn_ptr_1: *mut *mut code;

    hobject = GetStockObject16(BLACK_BRUSH);
    if (_u16_1050_5cd0 == 0) {
        fn_ptr_1 = (*param_1 + 0x68);
        uVar1 = (**fn_ptr_1)(s_tile2_bmp_1050_1538, param_1, (param_1 + 0x6e));
        uVar1 = pass1_1008_4d72(uVar1);
        uVar3 = (uVar1 >> 0x10);
        iVar1 = uVar1;
        _u16_1050_5cd0 = CONCAT22(
            CONCAT11(0x2, iVar1.field_0x94),
            CONCAT11(iVar1.field_0x95, iVar1.field_0x96),
        );
    }
    if (0x3 < param_3) {
        if (param_3 == 0x4) {
            hobject = GetStockObject16(HOLLOW_BRUSH);
        } else if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
            return 0x0;
        }
    }
    SetTextColor16(_u16_1050_5cd0, hwnd_param_4);
    SetBkColor16(0x1000000, hwnd_param_4);
    return CONCAT22(0x1050, hobject);
}


pub unsafe fn mix_draw_op_1040_21d6(param_1: *mut astruct_763)

{
    let mut paVar1: *mut astruct_13;
    let mut ppcVar2: *mut *mut code;
    let mut uVar4: u8;
    let mut hpalette_7: HPALETTE16;
    let mut uVar7: u16;
    let mut handle: HANDLE16;
    let mut extraout_var: u32;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut iVar10: *mut astruct_763;
    let mut count: i16;
    let mut uVar5: u32;
    let mut color: COLORREF;
    let mut handle_00: HGDIOBJ16;
    let mut hdc_24: HDC16;
    PAINTSTRUCT16 * paintstruct_22;
    let mut uVar1: u8;
    let mut uVar2: *mut u32;
    let mut uVar3: u16;
    let mut iVar5: *mut astruct_764;
    let mut uVar11: u16;

    count = (param_1 >> 0x10);
    iVar10 = param_1;
    hdc_24 = BeginPaint16(CONCAT22(0x1050, &paintstruct_22), iVar10.field6_0x6);
    paVar1 = (_PTR_LOOP_1050_4230 + 0xe);
    hpalette_7 = palette_op_1008_4e08(&hdc_24, in_DX, paVar1, CONCAT22(0x1050, &hdc_24));
    uVar2 = iVar10.field141_0x8e;
    ppcVar2 = (*iVar10.field141_0x8e + 0x4);
    (**ppcVar2)(0x1008, uVar2, (uVar2 >> 0x10), 0xffff, 0xffff, &hdc_24, &DAT_1050_1050);
    uVar5 = pass1_1008_4d72(paVar1);
    uVar3 = (uVar5 >> 0x10);
    iVar5 = uVar5;
    uVar7 = CONCAT11(iVar5.field_0x3e5, iVar5.field_0x3e6);
    uVar1 = iVar5.field996_0x3e4;
    color = SetBkColor16(0x0, hdc_24);
    extraout_DX = (color >> 0x10);
    uVar4 = SetTextColor16(CONCAT22(CONCAT11(0x2, uVar1), uVar7), hdc_24);
    handle_00 = 0;
    handle = GetProp16(s_hfont_1050_5ced, iVar10.field6_0x6);
    if (handle != 0) {
        handle_00 = SelectObject16(handle, hdc_24);
    }
    DrawText16(0x10,
               (param_1 & 0xff000000 | CONCAT12((param_1 >> 0x10), &iVar10.rect_0x92)),
               -0x1, iVar10.field152_0xa2, hdc_24);
    SetTextColor16(CONCAT22(CONCAT11(0x2, iVar5.field_0x94), CONCAT11(iVar5.field_0x95, iVar5.field_0x96)), hdc_24);
    DrawText16(0x10, (param_1 & 0xffff0000 | ZEXT24(&iVar10.field147_0x9a)), -0x1,
               iVar10.field153_0xa6, hdc_24);
    if (handle != 0) {
        SelectObject16(handle_00, hdc_24);
    }
    SetBkColor16(color, hdc_24);
    SetTextColor16(CONCAT31(extraout_var, uVar4) & 0xffff | extraout_DX << 0x10, hdc_24);
    hpalette_7 = SelectPalette16(0x0, hpalette_7, hdc_24);
    DeleteObject16(hpalette_7);
    EndPaint16(CONCAT22(0x1050, &paintstruct_22), iVar10.field6_0x6);
    return;
}


pub unsafe fn mixed_draw_op_1040_8a06(mut param_1: u16, param_2: *mut astruct_765)

{
    let mut paVar1: *mut astruct_13;
    let mut uVar6: u8;
    let mut HVar7: HPALETTE16;
    let mut handle: HANDLE16;
    let mut extraout_var: u32;
    let mut extraout_DX: u16;
    let mut iVar10: *mut astruct_765;
    let mut count: i16;
    let mut uVar8: u32;
    let mut color: COLORREF;
    let mut color_00: u32;
    let mut hdc_local_24: HDC16;
    let mut paintstruct_22: PAINTSTRUCT16 = PAINTSTRUCT16::default();
    let mut uVar1: u8;
    let mut uVar2: u8;
    let mut uVar3: u8;
    let mut uVar4: LPCSTR;
    let mut uVar5: u16;
    let mut iVar2: *mut astruct_766;

    count = (param_2 >> 0x10);
    iVar10 = param_2;
    hdc_local_24 = BeginPaint16(CONCAT22(0x1050, &paintstruct_22), iVar10.hwnd_field6_0x6);
    paVar1 = (_PTR_LOOP_1050_4230 + 0xe);
    HVar7 = palette_op_1008_4e08(&hdc_local_24, param_1, paVar1, CONCAT22(0x1050, &hdc_local_24));
    uVar8 = pass1_1008_4d72(paVar1);
    uVar5 = (uVar8 >> 0x10);
    iVar2 = uVar8;
    uVar1 = iVar2.field149_0x95;
    uVar2 = iVar2.field150_0x96;
    uVar3 = iVar2.field148_0x94;
    DrawIcon16(iVar10.field141_0x8e, 0xa, 0x14, hdc_local_24);
    color = SetBkColor16(0x0, hdc_local_24);
    extraout_DX = (color >> 0x10);
    uVar6 = SetTextColor16(CONCAT22(CONCAT11(0x2, uVar3), CONCAT11(uVar1, uVar2)), hdc_local_24);
    color_00 = CONCAT31(extraout_var, uVar6) & 0xffff | extraout_DX << 0x10;
    handle = GetProp16(s_hfont_1050_5dfa, iVar10.hwnd_field6_0x6);
    if (handle != 0) {
        SelectObject16(handle, hdc_local_24);
    }
    DrawText16(0x10, (param_2 & 0xffff0000 | ZEXT24(&iVar10.rect_0x9e)), -0x1,
               iVar10.field142_0x90, hdc_local_24);
    if (handle != 0) {
        SelectObject16(hdc_local_24, hdc_local_24);
    }
    SetBkColor16(color, hdc_local_24);
    SetTextColor16(color_00, hdc_local_24);
    HVar7 = SelectPalette16(0x0, HVar7, hdc_local_24);
    DeleteObject16(HVar7);
    EndPaint16(CONCAT22(0x1050, &paintstruct_22), iVar10.hwnd_field6_0x6);
    return;
}
