use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::block_1000::block_1000_1000::mem_op_1000_179c;
use crate::block_1000::block_1000_3000::{str_op_1000_3da4, sys_1000_3f9c};
use crate::block_1000::block_1000_4000::pass1_1000_484c;
use crate::block_1008::{block_1008_4000, block_1008_6000};
use crate::block_1008::block_1008_4000::{palette_op_1008_4e08, pass1_1008_4772, pass1_1008_4d72};
use crate::block_1008::block_1008_7000::switch_1008_73ea;
use crate::block_1010::block_1010_1000::pass1_1010_1f62;
use crate::block_1010::block_1010_6000::pass1_1010_6ca2;
use crate::block_1010::block_1010_7000::pass1_1010_715c;
use crate::block_1010::block_1010_8000::{FUN_1010_830a, pass1_1010_8170};
use crate::block_1018::block_1018_4000::{pass1_1018_4dce, struct_op_1018_4cda};
use crate::block_1018::block_1018_6000;
use crate::block_1018::block_1018_6000::{pass1_1018_642e, pass1_1018_6630};
use crate::block_1020::block_1020_2000::{pass1_1020_2286, pass1_1020_239c, pass1_1020_2488};
use crate::block_1020::{block_1020_3000, block_1020_6000, block_1020_7000};
use crate::block_1040::block_1040_9000;
use crate::dos_interrupt::swi;
use crate::{file_ops, win_ui, winapp};
use crate::file_ops::{read_file_1008_7dee, write_to_file_1008_7e1c};
use crate::globals::{DAT_1050_1050, DAT_1050_4216, DAT_1050_422c};
use crate::structs::struct_57::Struct57;
use crate::utils::{CONCAT11, CONCAT22};
use crate::winapi16::{CreateDC16, CreatePalette16, DeleteDC16, DeleteObject16, GetStockObject16, Rectangle16, SelectObject16, SelectPalette16, SetBkColor16, SetTextColor16, TextOut16, WritePrivateProfileString16};
use crate::windef16::{COLORREF, DEVMODEA, HDC16, HFILE16, HGDIOBJ16, HPALETTE16, HPEN16, HWND16, LOGPALETTE, RECT16};

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

pub unsafe fn draw_op_1040_a67e(struct750_param_1: *mut astruct_750, mut param_2: i16, mut param_3: u16, mut param_4: u16)

{
    let mut bVar1: bool;
    let mut brush_handle_var2: HBRUSH16;
    let mut struct750_var4: *mut astruct_750;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut iStack14: i16;
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut iVar2: *mut astruct_751;

    uVar3 = (struct750_param_1 >> 0x10);
    struct750_var4 = struct750_param_1;
    if (struct750_var4.hbrush16_field142_0x8e == 0) {
        brush_handle_var2 = CreateSolidBrush16(WHITE_BRUSH);
        struct750_var4.hbrush16_field142_0x8e = brush_handle_var2;
    }
    if (_u16_1050_5ee8 == 0) {
        uVar4 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
        uVar2 = (uVar4 >> 0x10);
        iVar2 = uVar4;
        _u16_1050_5ee8 = CONCAT12(iVar2.field_0x94, CONCAT11(iVar2.field_0x95, iVar2.field_0x96));
        u16_1050_5eec = CONCAT11(iVar2.field_0x3e5, iVar2.field_0x3e6);
        u16_1050_5eee = iVar2.field996_0x3e4;
    }
    if (0x5 < param_3) {
        if (param_3 != 0x6) {
            return;
        }
        bVar1 = false;
        // for (iStack14 = 0; piVar1 = &struct750_var4.field233_0xea, *piVar1 != iStack14 && iStack14 <= *piVar1;
        //     iStack14 += 1)
        iStack14 = 0;
        piVar1 = struct750_param_1.field233_0xea;
        while *piVar1 != iStack14 && iStack14 <= *piVar1 {
            if ((&struct750_var4.field_0x9a + iStack14 * 0x2) == param_2) {
                bVar1 = true;
                break;
            }
            iStack14 += 1;
        }
        if (bVar1) {
            u16_1050_5ee8 = u16_1050_5eec;
            u16_1050_5eea = u16_1050_5eee;
        }
    }
    SetTextColor16(CONCAT22(u16_1050_5eea, u16_1050_5ee8), param_4);
    SetBkColor16(0x1000000, param_4);
    return;
}


pub unsafe fn win_ui_op_1040_b372(mut param_1: u32, hwnd_param_2: HWND16, mut param_3: u16, hdc_param_4: HDC16)

{
    let mut uVar1: u16;
    let mut dlg_ctrl_id: i16;
    let mut local_brush_handle: HBRUSH16;
    let mut uVar4: u32;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut uVar2: u32;
    let mut iVar1: *mut astruct_798;
    let mut uVar3: u16;
    let mut uVar6: u16;

    uVar5 = (param_1 >> 0x10);
    if ((param_1 + 0x8e) == 0) {
        local_brush_handle = CreateSolidBrush16(WHITE_BRUSH);
        (param_1 + 0x8e) = local_brush_handle;
    }
    if (_PTR_LOOP_1050_5efa == 0) {
        uVar2 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
        uVar1 = (uVar2 >> 0x10);
        iVar1 = uVar2;
        _PTR_LOOP_1050_5efa = CONCAT12(iVar1.field_0x94, CONCAT11(iVar1.field_0x95, iVar1.field_0x96));
    }
    if (param_3 < 0x4) {//
// LAB_1040_b3ea:
        dlg_ctrl_id = GetDlgCtrlID16(hwnd_param_2);
        if (dlg_ctrl_id == 0x14c) {
            uVar3 = 0xffff;
            uVar6 = 0;
            // TODO: goto LAB_1040_b41a;
        }
        if (dlg_ctrl_id == 0x175) {
            uVar3 = 0xff;
            uVar6 = 0;
            // TODO: goto LAB_1040_b41a;
        }
    } else if (param_3 != 0x4) {
        if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
            return;
        }
// TODO: goto LAB_1040_b3ea;
    }
    uVar3 = _PTR_LOOP_1050_5efa;
    uVar6 = (_PTR_LOOP_1050_5efa >> 0x10);//
// LAB_1040_b41a:
    SetTextColor16(CONCAT22(uVar6, uVar3), hdc_param_4);
    SetBkColor16(0x1000000, hdc_param_4);
    return;
}

pub unsafe fn draw_ui_op_1040_27cc(param_1: *mut astruct_752, hwnd16_param_2: HWND16, mut param_3: u16, hdc_param_4: HDC16) -> u32

{
    let mut uVar1: u32;
    let mut brush_handle_var8: HBRUSH16;
    let mut IVar3: i16;
    let mut iVar3: *mut astruct_752;
    let mut uVar3: u16;
    let mut unaff_CS: u16;
    let mut uVar7: u16;
    let mut uVar4: u32;
    let mut hdc: HDC16;
    let mut iVar2: *mut astruct_753;
    let mut uVar2: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut fn_ptr_1: *mut *mut code;

    uVar7 = SUB42(&PTR_LOOP_1050_1040, 0x0);
    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (iVar3.brush_handle_field4_0x4 == 0) {
        uVar7 = SUB42(s_tile2_bmp_1050_1538, 0x0);
        brush_handle_var8 = CreateSolidBrush16(WHITE_BRUSH);
        iVar3.brush_handle_field4_0x4 = brush_handle_var8;
    }
    if (_u16_1050_5cf8 == 0) {
        fn_ptr_1 = (param_1 + 0x68);
        uVar1 = (**fn_ptr_1)(uVar7, param_1, iVar3.field109_0x6e);
        uVar4 = pass1_1008_4d72(uVar1);
        uVar2 = (uVar4 >> 0x10);
        iVar2 = uVar4;
        _u16_1050_5cf8 = CONCAT22(CONCAT11(0x2, iVar2.field_0x94), CONCAT11(iVar2.field_0x95, iVar2.field_0x96));
    }
    if (0x5 < param_3) {
        if (param_3 != 0x6) {
            return 0x0;
        }
        uVar5 = 0x284a;
        IVar3 = GetDlgCtrlID16(hwnd16_param_2);
        if ((iVar3.field146_0x94 != 0) && (IVar3 == 0xfb2)) {
            uVar6 = 0xff;
            hdc = 0;
            // TODO: goto LAB_1040_286e;
        }
    }
    uVar5 = _u16_1050_5cf8;
    uVar6 = (_u16_1050_5cf8 >> 0x10);
    hdc = hdc_param_4;//
// LAB_1040_286e:
    SetTextColor16(CONCAT22(uVar6, uVar5), hdc);
    SetBkColor16(0x1000000, hdc_param_4);
    return CONCAT22(0x1050, iVar3.brush_handle_field4_0x4);
}

pub unsafe fn draw_op_1020_7070(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    hdc_param_5: HDC16,
) -> u16 {
    let mut hgdi_obj: u16;

    GetStockObject16(BLACK_BRUSH);
    if (COLORREF_1050_441e == 0) {
        COLORREF_1050_441e = 0x1000002;
    }
    if (0x6 < param_4) {
        return 0x0;
    }
    SetTextColor16(COLORREF_1050_441e, hdc_param_5);
    hgdi_obj = 0x100;
    SetBkColor16(0x1000000, hdc_param_5);
    return hgdi_obj;
}

pub unsafe fn draw_polygon_1020_3602(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    Polygon16(param_3, param_4, param_5);
    return;
}

pub unsafe fn unk_draw_op_1020_3da4(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_24,
    param_4: *mut StructA,
) {
    let mut puVar2: *mut u32;
    let mut pUVar3: *mut u32;
    let mut i16: i16;
    let mut white_pen_handle: HGDIOBJ16;
    let mut pHVar4: *mut HDC16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut pstruct24_1: *mut astruct_24;
    let mut pstruct_24_hi: u16;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe92: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut hdc: HDC16;
    let mut hdc_00: HDC16;
    let mut hdc_4: HDC16;
    let mut iVar9: *mut astruct_24;
    let mut uVar8: *mut astruct_24;
    let mut puVar1: *mut u32;
    let mut uVar4: *mut u32;
    let mut in_stack_0000ffea: u16;
    let mut fn_ptr_1: *mut *mut code;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    get_sys_metrics_1020_7c1a(param_3, param_4);
    pstruct_24_hi = (param_3 >> 0x10);
    pstruct24_1 = param_3;
    pstruct24_1.field17_0x14 = null_mut();
    param_3.offset_0x0 = 0x408a;
    pstruct24_1.base_0x2 = 0x1020;
    puVar6 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x6),
        in_stack_0000fe92,
        in_stack_0000ffb6,
        in_stack_0000ffbc,
        in_stack_0000ffc0,
    );
    hdc = (puVar6 >> 0x10);
    pstruct24_1.field17_0x14 = puVar6;
    *(&pstruct24_1.field17_0x14 + 0x2) = hdc;
    hdc_00 = 0;
    fn_ptr_1 = (*pstruct24_1.field17_0x14 + 0x4);
    (**fn_ptr_1)(0x1010, &pstruct24_1.field17_0x14, hdc, 0x0, param_3);
    hdc_4 = GetDC16(pstruct24_1.field2_0x4);
    i16 = SetMapMode16(0x1, hdc_4);
    pstruct24_1.field21_0x1e = i16;
    white_pen_handle = GetStockObject16(WHITE_BRUSH);
    white_pen_handle = SelectObject16(white_pen_handle, hdc);
    pstruct24_1.field18_0x18 = white_pen_handle;
    white_pen_handle = GetStockObject16(WHITE_PEN);
    white_pen_handle = SelectObject16(white_pen_handle, hdc_00);
    pstruct24_1.obj_handle_0x1a = white_pen_handle;
    uVar4 = pstruct24_1.field17_0x14;
    puVar2 = (uVar4 + 0x24);
    pHVar4 = &hdc_4;
    // just 0x1538
    fn_ptr_1 = (*puVar2 + 0x8);
    (**fn_ptr_1)(
        s_tile2_bmp_1050_1538,
        puVar2,
        (puVar2 >> 0x10),
        pHVar4,
        &DAT_1050_1050,
    );
    pstruct24_1.field20_0x1c = pHVar4;
    pUVar3 = pstruct24_1.field17_0x14;
    *(pUVar3 + 0x4c) = hdc_4;
    return;
}

pub unsafe fn pass1_1020_3d08(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u16,
    mut param_9: u16,
    mut param_15: u32,
    mut param_16: u16,
    mut param_17: u16,
    mut param_18: u16,
    mut param_19: u16,
) {
    let mut pcVar1: *mut c_char;
    let mut pbVar2: *mut u8;
    let mut bVar3: bool;
    let mut bVar4: bool;
    let mut ppcVar5: *mut *mut code;
    let mut puVar6: *mut u16;
    let mut uVar7: u32;
    let mut puVar8: *mut u32;
    let mut hdc: HDC16;
    let pSVar9: *mut StructA;
    let mut bVar10: u8;
    let mut bVar12: u8;
    let mut iVar13: i16;
    let mut bVar17: u8;
    let mut cVar18: u8;
    let mut hdc_00: HDC16;
    let mut iVar14: i16;
    let mut HVar15: HGDIOBJ16;
    let mut puVar16: *mut u8;
    let mut uVar19: u16;
    let mut bVar20: u8;
    let mut bVar21: u8;
    let mut uVar22: u16;
    let mut in_register_0000000a: u16;
    let mut bVar24: u8;
    let mut pcVar25: *mut c_char;
    let mut uVar26: u16;
    let mut uVar27: u16;
    let mut bVar28: bool;
    let mut bVar29: bool;
    let mut puVar30: *mut u32;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb8: u16;
    let mut uStack30: u16;
    let mut pcStack4: *mut code;
    let mut bVar11: u8;
    let mut paVar23: *mut Struct57;

    pSVar9 = CONCAT22(param_19, param_18);
    bVar20 = param_2 + (param_1 >> 0x8) + param_10;
    *param_6 = 0x3c;
    paVar23 = CONCAT22(
        in_register_0000000a,
        CONCAT11(
            (param_2 >> 0x8) + '<' + (*(param_3 + param_5) < 0x20),
            bVar20,
        ),
    );
    pcStack4 = caseD_a7;
    iVar13 = 0x203d;
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 | bVar20;
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 & bVar20;
    pcVar25 = CONCAT11(0x79, param_5 - 0x37);
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 | bVar20;
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 | bVar20;
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 | bVar20;
    bVar10 = (param_6 + 2);
    bVar12 = 0x9 < (bVar10 & 0xf) | param_11;
    bVar11 = bVar10 + bVar12 * '\x06' & 0xf;
    pbVar2 = (param_3 + 0x203d);
    *pbVar2 = *pbVar2 | bVar20;
    bVar10 = 0x9 < bVar11 | bVar12;
    uVar19 = CONCAT11(
        (param_6 + 0x2 >> 0x8) + bVar12 + bVar10,
        bVar11 + bVar10 * '\x06',
    ) & 0xff0f;
    loop {
        pbVar2 = (param_3 + iVar13);
        bVar21 = paVar23;
        *pbVar2 = *pbVar2 | bVar21;
        bVar12 = (uVar19 - 1);
        bVar3 = 0x9 < (bVar12 & 0xf);
        bVar20 = bVar3 | bVar10;
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar21;
        bVar4 = 0x9 < (bVar12 + bVar20 * '\x06' & 0xf);
        bVar17 = (uVar19 - 0x1 >> 0x8) + bVar20 + (bVar4 | bVar20);
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar21;
        uVar19 = 0;
        bVar28 = &pcStack4 < (param_3 + iVar13);
        pbVar2 = (param_3 + iVar13 + 0x896);
        bVar24 = param_3;
        bVar29 = CARRY1(*pbVar2, bVar24) || CARRY1(*pbVar2 + bVar24, bVar28);
        *pbVar2 = *pbVar2 + bVar24 + bVar28;
        pbVar2 = (param_3 + iVar13 + 0x2038);
        bVar12 = *pbVar2;
        bVar11 = *pbVar2;
        *pbVar2 = bVar11 + bVar24 + bVar29;
        pcVar1 = (param_4 + iVar13);
        *pcVar1 = *pcVar1 + (paVar23 >> 0x8) + (CARRY1(bVar12, bVar24) || CARRY1(bVar11 + bVar24, bVar29));
        pcVar1 = (param_3 + iVar13 - 0x64);
        *pcVar1 = *pcVar1 + bVar17 + '\x01';
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar21;
        pcVar1 = pcVar25;
        pcVar25 = pcVar25 + 1;
        bVar28 = *pcVar1 != '\0';
        if (-*pcVar1 < '\0') {
            pcVar1 = (param_4 + 0x37);
            *pcVar1 = *pcVar1 + bVar24 + bVar28;
            pbVar2 = (param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar21;
            iVar13 += -0x1;
            pbVar2 = (param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar21;
            pbVar2 = (param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar21;
            uVar22 = paVar23 - 0x1;
            paVar23 = (paVar23 & 0xffff0000 | uVar22);
            pbVar2 = (param_3 + iVar13);
            bVar12 = uVar22;
            *pbVar2 = *pbVar2 | bVar12;
            if (*pbVar2 == 0) {
                pbVar2 = (param_3 + iVar13);
                *pbVar2 = *pbVar2 & bVar12;
                // code_r0x10203d96:
                pbVar2 = (param_3 + iVar13);
                bVar12 = paVar23;
                *pbVar2 = *pbVar2 | bVar12;
                pbVar2 = (param_3 + iVar13);
                *pbVar2 = *pbVar2 & bVar12;
                paVar23 = (paVar23 & 0xffff0000 | CONCAT11((paVar23 >> 0x8) * '\x02' + (uVar19 < 0x20), bVar12));
                pbVar2 = (param_3 + iVar13 + 1);
                *pbVar2 = *pbVar2 & bVar12;
                param_4 = &stack0xfff6;
                param_16 = pcStack4;
                param_17 = (pcStack4 >> 0x10);
                pSVar9 = param_15;
                // code_r0x10203db1:
                get_sys_metrics_1020_7c1a(CONCAT22(param_17, param_16), pSVar9);
                puVar6 = (param_4 + 0x6);
                uVar26 = (puVar6 >> 0x10);
                (puVar6 + 0x14) = 0;
                *puVar6 = 0x408a;
                (puVar6 + 0x2) = 0x1020;
                puVar30 = mixed_1010_20ba(
                    paVar23,
                    _u16_1050_0ed0,
                    CONCAT22(uStack30, 0x6),
                    in_stack_0000fe8a,
                    in_stack_0000ffae,
                    in_stack_0000ffb4,
                    in_stack_0000ffb8,
                );
                hdc = (puVar30 >> 0x10);
                uVar7 = (param_4 + 0x6);
                uVar26 = (uVar7 >> 0x10);
                iVar13 = uVar7;
                (iVar13 + 0x14) = puVar30;
                *(iVar13 + 0x16) = hdc;
                ppcVar5 = ((iVar13 + 0x14) + 0x4);
                (**ppcVar5)(0x1010, (iVar13 + 0x14), hdc, 0x0, iVar13, uVar26);
                uVar7 = (param_4 + 0x6);
                hdc_00 = GetDC16((uVar7 + 0x4));
                *(param_4 - 0x2) = hdc_00;
                iVar14 = SetMapMode16(0x1, hdc_00);
                uVar7 = (param_4 + 0x6);
                (uVar7 + 0x1e) = iVar14;
                HVar15 = GetStockObject16(0x0);
                HVar15 = SelectObject16(HVar15, hdc);
                uVar7 = (param_4 + 0x6);
                (uVar7 + 0x18) = HVar15;
                HVar15 = GetStockObject16(0x6);
                HVar15 = SelectObject16(HVar15, 0x0);
                uVar7 = (param_4 + 0x6);
                (uVar7 + 0x1a) = HVar15;
                uVar7 = (param_4 + 0x6);
                uVar7 = (uVar7 + 0x14);
                (param_4 - 0x6) = (uVar7 + 0x24);
                puVar16 = (param_4 - 0x2);
                puVar8 = (param_4 - 0x6);
                ppcVar5 = (*puVar8 + 0x8);
                (**ppcVar5)(
                    s_tile2_bmp_1050_1538,
                    puVar8,
                    (puVar8 >> 0x10),
                    puVar16,
                    &DAT_1050_1050,
                );
                uVar7 = (param_4 + 0x6);
                uVar27 = (uVar7 >> 0x10);
                iVar13 = uVar7;
                (iVar13 + 0x1c) = puVar16;
                uVar26 = (param_4 - 0x2);
                (param_4 - 0x14) = uVar26;
                uVar7 = (iVar13 + 0x14);
                (uVar7 + 0x4c) = uVar26;
                return;
            }
            pbVar2 = (param_3 + iVar13);
            *pbVar2 = *pbVar2 & bVar12;
            pcVar1 = (param_4 + iVar13 + 0x20);
            bVar11 = param_1 & 0x1f;
            cVar18 = *pcVar1;
            *pcVar1 = *pcVar1 >> bVar11;
            pcVar1 = (param_4 + iVar13 + 0x6a);
            *pcVar1 = *pcVar1 + param_1 + ((param_1 & 0x1f) != 0x0 && (cVar18 >> bVar11 - 0x1 & 1) != 0);
            pbVar2 = (param_3 + iVar13);
            *pbVar2 = *pbVar2 | bVar12;
            uVar19 = (param_3 + iVar13) * 0x10;
            pbVar2 = (pcVar25 + param_4 + 0x8);
            bVar12 = (uVar19 >> 0x8);
            uVar22 = uVar19 & 0xff | (bVar12 + *pbVar2) << 0x8;
            pcVar1 = (param_4 + iVar13 + 0x37);
            *pcVar1 = *pcVar1 + (param_3 >> 0x8) + CARRY1(bVar12, *pbVar2);
        } else {
            pcVar1 = (param_4 + iVar13);
            *pcVar1 = *pcVar1 + bVar28;
            uVar22 = (param_3 + iVar13) * 0x10;
            //      if ((POPCOUNT(*pcVar1) & 1) == 0) goto code_r0x10203db1;
        }
        pbVar2 = (param_3 + iVar13);
        bVar11 = paVar23;
        *pbVar2 = *pbVar2 | bVar11;
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar11;
        param_16 = (param_14 & 1) * 0x4000 | (param_13 & 1) * 0x200 | (param_12 & 1) * 0x100 | (*pbVar2 < '\0') * 0x80 | (*pbVar2 == 0) * 0x40 | (bVar4 | bVar3 | bVar10 & 1) * 0x10 | ((POPCOUNT(*pbVar2) & 1) == 0) * 0x4;
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar11;
        // TODO: bVar12 = in(0x79);
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 & bVar11;
        if (-0x1 < *pbVar2) {
            pSVar9 = CONCAT22(param_19, param_18);
            if ((bVar17 | *(param_4 - 1)) == 0) {
                param_16 = param_7;
                pSVar9 = CONCAT22(param_19, param_18);
            }
            // TODO: goto code_r0x10203db1;
        }
        pbVar2 = (param_4 + 0x89c);
        bVar28 = CARRY1(*pbVar2, bVar12);
        *pbVar2 = *pbVar2 + bVar12;
        bVar21 = bVar17 + bVar12;
        cVar18 = bVar21 + bVar28;
        uVar19 = CONCAT11(cVar18, bVar12);
        pcStack4._0_3_ = CONCAT21(
            (param_14 & 1) * 0x4000 | (SCARRY1(bVar17, bVar12) != SCARRY1(bVar21, bVar28)) * 0x800 | (param_13 & 1) * 0x200 | (param_12 & 1) * 0x100 | (cVar18 < '\0') * 0x80 | (cVar18 == '\0') * 0x40 | (bVar4 | bVar3 | bVar10 & 1) * 0x10 | ((POPCOUNT(cVar18) & 1) == 0) * 0x4 | (CARRY1(bVar17, bVar12) || CARRY1(bVar21, bVar28)),
            pcStack4._0_1_,
        );
        pcStack4 = (pcStack4 & 0xff000000 | pcStack4);
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 | bVar11;
        param_1 = uVar22 - 0x1;
        bVar10 = bVar4 | bVar20;
        //    if (param_1 == 0x0 || *pbVar2 == 0) goto code_r0x10203d96;
    }
}

pub unsafe fn validate_rect_1020_3f12(mut param_1: u32, mut param_2: i16) {
    let mut local_a: RECT16;
    let mut uStack6: u32;

    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    local_a = 0x8000e;
    uStack6 = 0x1100116;
    InvalidateRect16(0x0, &local_a, &DAT_1050_1050);
    local_a = 0xf10000;
    uStack6 = 0x1220030;
    ValidateRect16(&local_a, &DAT_1050_1050);
    local_a = 0xf100f5;
    uStack6 = 0x1220127;
    ValidateRect16(&local_a, &DAT_1050_1050);
    return;
}


pub unsafe fn draw_op_1008_8288(
    mut param_1: u16,
    mut param_2: u32,
    hdc16_param_1: HDC16,
    mut param_4: u32,
) {
    let mut paint_handle_1: HDC16;
    let mut pen_handle_1: HPEN16;
    let mut pen_handle_3: HPEN16;
    let mut brush_handle_1: HBRUSH16;
    let mut hgdiobj16_var1: HGDIOBJ16;
    let mut y: u16;
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut hdc: HDC16;
    let mut right_00: u16;
    let mut obj: HGDIOBJ16;
    let mut paintstruct_3c: [u8; 0x20] = [0; 0x20];
    let mut POpoint_1c: INT16;
    let mut iStack24: i16;
    let mut iStack22: i16;
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut POlocal_10: INT16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut iStack6: i16;
    let mut uStack4: u16;
    let mut in_stack_0000ffa6: u32;
    let mut left_01: i16;
    let mut top: i16;
    let mut left: i16;
    let mut x2: u16;
    let mut uVar1: u32;
    let mut right: u16;
    let mut in_stack_0000ffae: u16;
    let mut bottom: u16;

    paint_handle_1 = BeginPaint16(CONCAT22(0x1050, paintstruct_3c), hdc16_param_1);
    uStack4 = 0;
    pen_handle_1 = CreatePen16(COLORREF_1050_0368, 0x1, 0x0);
    pen_handle_3 = CreatePen16(COLORREF_1050_0364, 0x1, 0x0);
    brush_handle_1 = CreateSolidBrush16(COLORREF_1050_0364);
    uVar1 = CONCAT22(pen_handle_3, brush_handle_1);
    hdc = hdc16_param_1;
    GetClientRect16((&param_2 + 0x2), &DAT_1050_1050);
    y = param_1 >> 0x1;
    right_00 = x2;
    hgdiobj16_var1 = GetStockObject16(BLACK_PEN);
    hgdiobj16_var1 = SelectObject16(hgdiobj16_var1, hdc);
    param_2 = param_2 & 0xffff0000 | hgdiobj16_var1;
    hgdiobj16_var1 = GetStockObject16(BLACK_BRUSH);
    SelectObject16(hgdiobj16_var1, left);
    Rectangle16(param_1, right_00, top, (param_2 >> 0x10), paint_handle_1);
    MoveTo16(y, 0x0, paint_handle_1);
    param_2 = param_2 & 0xffff0000 | paint_handle_1;
    LineTo16(y, x2, paint_handle_1);
    uVar3 = (param_4 >> 0x10);
    if ((*(param_4 + 0x4) & 0x4) != 0) {
        uStack4 = 0x1;
    }
    local_10.x = (x2 >> 1) + uStack4;
    iVar1 = (param_1 >> 0x2) + uStack4;
    local_10.y = iVar1 -0x2;
    iStack12 = local_10.x -0x3;
    iStack10 = iVar1 + 1;
    iStack8 = local_10.x + 0x3;
    iStack6 = iStack10;
    param_2 = pen_handle_1;
    param_2 = paint_handle_1;
    SelectObject16(pen_handle_1, paint_handle_1);
    if (uStack4 == 0) {
        param_2 = CONCAT22(paint_handle_1, 1);
        MoveTo16(y - 0x2, 0x1, paint_handle_1);
        param_2 = 0x10001;
        LineTo16(0x1, 0x1, paint_handle_1);
        param_2 = 0x1;
        param_2 = s_tile2_bmp_1050_1538;
        LineTo16(0x1, x2 - 0x1, paint_handle_1);
    }
    uStack4 = ((*(param_4 + 0x4) & 0x8) != 0);
    point_1c.x = (x2 >> 1) + uStack4;
    iVar2 = (param_1 - (param_1 >> 0x2)) + uStack4;
    point_1c.y = iVar2 + 1;
    iStack24 = point_1c.x -0x3;
    iStack22 = iVar2 -0x2;
    iStack20 = point_1c.x + 0x3;
    iStack18 = iStack22;
    if (uStack4 == 0) {
        param_2 = 0x15388429;
        MoveTo16(paint_handle_1 - 0x2, 0x1, paint_handle_1);
        param_2 = 0x843a;
        LineTo16(y + 0x1, 0x1, paint_handle_1);
        uVar1 = CONCAT22(paint_handle_1, x2 - 1);
        LineTo16(y + 0x1, x2 - 0x1, paint_handle_1);
    }
    param_2 = CONCAT22(0x8453, param_2);
    SelectObject16((uVar1 >> 0x10), paint_handle_1);
    param_2 = CONCAT22(0x845e, param_2);
    SelectObject16(uVar1, paint_handle_1);
    obj = (uVar1 >> 0x10);
    param_2 = 0x31538;
    Polygon16(0x3, &local_10, &DAT_1050_1050);
    param_2 = 0x31538;
    hgdiobj16_var1 = 0x847c;
    Polygon16(0x3, &point_1c, &DAT_1050_1050);
    param_2 = CONCAT22(0x8487, param_2);
    SelectObject16(hgdiobj16_var1, paint_handle_1);
    param_2 = CONCAT22(0x8492, param_2);
    SelectObject16(param_2, paint_handle_1);
    DeleteObject16(pen_handle_1);
    DeleteObject16(obj);
    DeleteObject16(obj);
    EndPaint16(CONCAT22(0x1050, paintstruct_3c), hdc16_param_1);
    return;
}

// WARNING: Unable to use type for symbol uVar3
// WARNING: Unable to use type for symbol x1
// WARNING: Unable to use type for symbol uVar2
pub unsafe fn draw_line_1018_6444(mut param_1: u32, hdc_param_2: HDC16) {
    let mut iVar1: i16;
    let mut x: i16;
    let mut iVar4: *mut astruct_796;
    let mut piVar5: *mut i16;
    let mut piVar4: *mut i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut x_00: HDC16;
    let mut iStack10: i16;
    let mut uVar3: u32;
    let mut x1: *mut INT16 = null_mut();
    let mut uVar2: u32;

    uVar6 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0x6);
    iVar1 = (uVar2 + 0x30);
    uVar3 = (param_1 + 0x6);
    x1 = (uVar3 + 0x1a);
    MoveTo16(0x5, *x1, hdc_param_2);
    uVar7 = (x1 >> 0x10);
    iVar4 = x1;
    LineTo16(0x5, (iVar4 + iVar1 * 0x8 -0x4), hdc_param_2);
    for iStack10 in 0..iVar1 {
        piVar5 = (iVar4 + iStack10 * 0x8);
        x = (piVar5[0x2] - *piVar5 >> 1) + *piVar5;
        MoveTo16(0x5, x, hdc_param_2);
        LineTo16(0xa, x, hdc_param_2);
    }
    MoveTo16(0x5f, *x1, hdc_param_2);
    LineTo16(0x5f, (iVar4 + iVar1 * 0x8 -0x4), hdc_param_2);
    for iStack10 in 0..iVar1 {
        piVar4 = (iVar4 + iStack10 * 0x8);
        x_00 = hdc_param_2;
        MoveTo16(0x5f, (piVar4[0x2] - *piVar4 >> 1) + *piVar4, hdc_param_2);
        LineTo16(0x5a, x_00, hdc_param_2);
    }
    return;
}

pub unsafe fn draw_op_1018_6544(mut param_1: u32, param_2: *mut i16) {
    let mut puVar1: *mut u16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut local_a: [u8; 0x6] = [0; 0x6];
    let mut uStack4: u16;

    if (param_2.is_null() == false) {
        uStack4 = ((param_2 + 0x4) - *param_2 >> 1) + *param_2;
        puVar1 = pass1_1008_3e54(CONCAT22(0x1050, local_a), 0x0, 0x57, uStack4);
        uVar3 = (param_1 >> 0x10);
        uVar2 = block_1018_6000::pass1_1018_659a((puVar1 >> 0x10), param_1, uVar3, CONCAT22(0x1050, local_a));
        draw_polygon_1018_661c(param_1, uVar3, 0x3, uVar2, (uVar2 >> 0x10));
    }
    return;
}

pub unsafe fn draw_polygon_1018_661c(
    mut param_1: u16,
    mut param_2: u16,
    count_param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    Polygon16(count_param_3, param_4, param_5);
    return;
}


pub unsafe fn mixed_draw_op_1018_6a7a(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_28,
) {
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut uVar1: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe80: u16;
    let mut in_stack_0000ffa4: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb0: u16;
    let mut local_22: PAINTSTRUCT16 = PAINTSTRUCT16::default();
    let mut in_stack_0000ffd8: u16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    uVar1 = (param_3 >> 0x10);
    BeginPaint16(CONCAT22(0x1050, &local_22), (param_3 + 0x8));
    EndPaint16(CONCAT22(0x1050, &local_22), (param_3 + 0x8));
    puVar4 = mixed_1010_20ba(
        paVar3,
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x2),
        in_stack_0000fe80,
        in_stack_0000ffa4,
        in_stack_0000ffaa,
        in_stack_0000ffae,
    );
    uVar2 = (puVar4 >> 0x10);
    if ((puVar4 + 0x20) == 0) {
        win_ui::unk_destroy_window_op_1018_6bb6(param_3);
        return;
    }
    win_ui::mix_ui_op_1018_6adc(param_3, uVar2, in_stack_0000ffae, in_stack_0000ffb0);
    return;
}

// WARNING: Unable to use type for symbol pIVar1
// WARNING: Unable to use type for symbol uVar1
pub unsafe fn draw_line_1020_229c(mut param_1: u32, mut param_2: u16) {
    let mut uVar2: u32;
    let mut x: i16;
    let mut iVar3: i16;
    let mut piVar4: *mut i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut x_00: u16;
    let mut iStack10: i16;
    let mut iVar1: i16;
    let mut pIVar1: *mut INT16 = null_mut();
    let mut uVar1: u32;

    uVar5 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x6);
    iVar1 = (uVar1 + 0x30);
    uVar2 = (param_1 + 0x6);
    pIVar1 = (uVar2 + 0x1a);
    MoveTo16(0x5, *pIVar1, param_2);
    uVar6 = (pIVar1 >> 0x10);
    iVar3 = pIVar1;
    LineTo16(0x5, (iVar3 + iVar1 * 0x8 - 0x4), param_2);
    for iStack10 in 0..iVar1 {
        piVar4 = (iStack10 * 0x8 + iVar3);
        x = (piVar4[0x2] - *piVar4 >> 1) + *piVar4;
        MoveTo16(0x5, x, param_2);
        LineTo16(0xa, x, param_2);
    }
    MoveTo16(0x5f, *pIVar1, param_2);
    LineTo16(0x5f, (iVar3 + iVar1 * 0x8 - 0x4), param_2);
    for iStack10 in 0..iVar1 {
        piVar4 = (iStack10 * 0x8 + iVar3);
        x_00 = param_2;
        MoveTo16(0x5f, (piVar4[0x2] - *piVar4 >> 1) + *piVar4, param_2);
        LineTo16(0x5a, x_00, param_2);
    }
    return;
}

pub unsafe fn draw_polygon_1020_2474(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    Polygon16(param_3, param_4, param_5);
    return;
}

pub unsafe fn palette_op_1020_92c4(struct_param_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut struct_1: *mut StructD;
    let mut uVar2: *mut StructD;

    uVar2 = (struct_param_1 >> 0x10);
    struct_1 = struct_param_1;
    struct_param_1.address_offset_field_0x0 = 0x96c8;
    struct_1.address_offset_field_0x2 = 0x1020;
    if (struct_1.field11_0x12 != 0) {
        obj = SelectPalette16(0x0, struct_1.field11_0x12, struct_1.field6_0xa);
        DeleteObject16(obj);
    }
    struct_param_1.address_offset_field_0x0 = 0x3ab0;
    struct_1.address_offset_field_0x2 = 0x1008;
    struct_param_1.address_offset_field_0x0 = 0x389a;
    struct_1.address_offset_field_0x2 = 0x1008;
    return;
}

pub unsafe fn draw_op_1020_9364(param_1: *mut StructA) {
    let mut piVar1: *mut i16;
    let mut puVar2: *mut u16;
    let mut brush_handle_var6: HBRUSH16;
    let local_struct_1: *mut StructA;
    let mut var7: u16;
    let mut uVar7: u16;
    let mut uVar3: u16;
    let mut uVar8: u16;
    let mut HVar3: HDC16;
    let mut iStack62: i16;
    let mut uStack58: *mut astruct_737;
    let mut POpoint16_38: INT16;
    let mut hgdiobj16_var_52: HGDIOBJ16;
    let mut HStack50: HPEN16;
    let mut hdc16_var_48: HDC16;
    let mut uStack46: u32;
    let mut uStack42: u32;
    let mut uStack38: u32;
    let mut uStack34: u32;
    let mut puStack30: *mut u32;
    let mut puStack26: *mut u16;
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut rect16_var_12: RECT16;
    let mut uStack14: u32;
    let mut rect16_a: RECT16;
    let mut x_var_6: u32;
    let mut uVar2: u16;
    let mut iVar2: i16;
    let mut piVar2: *mut i16;
    let mut uVar4: u32;
    let mut iVar4: i16;
    let mut uVar10: u8;
    let mut uVar11: u8;

    var7 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    GetClientRect16(&rect16_a, &DAT_1050_1050);
    rect16_var_12 = rect16_a;
    uStack14 = x_var_6;
    uStack20 = DAT_1050_4216;
    iStack22 = DAT_1050_422c;
    puStack26 = PTR_u16_1050_4172_1050_4212;
    puStack30 = PTR_u16_1050_41b2_1050_4218;
    uStack34 = PTR_u16_1050_41da_1050_421c;
    uStack38 = PTR_u16_1050_4202_1050_4220;
    uStack42 = PTR_u16_1050_419a_1050_4224;
    uStack46 = PTR_u16_1050_41aa_1050_4228;
    uVar4 = &local_struct_1.field3_0x6;
    hdc16_var_48 = *(uVar4 + 0x12);
    uStack58 = (&u16_1050_0008 + 1);
    loop {
        HVar3 = hdc16_var_48;
        HStack50 = CreatePen16(*(uStack58 * 0x4 + uStack34), 0x0, 0x0);
        hgdiobj16_var_52 = SelectObject16(HStack50, HVar3);
        MoveToEx16(
            &point16_38,
            &DAT_1050_1050,
            (uStack58 * 0x2 + puStack26),
            rect16_a.x,
        );
        LineTo16((puStack26 + uStack58 * 0x2), x_var_6, hdc16_var_48);
        iVar4 = (uStack20 - uStack58) * 0x2;
        MoveToEx16(&point16_38, &DAT_1050_1050, (iVar4 + puStack26), rect16_a.x);
        LineTo16((puStack26 + iVar4), x_var_6, hdc16_var_48);
        SelectObject16(hgdiobj16_var_52, hdc16_var_48);
        DeleteObject16(HStack50);
        uStack58 = (&uStack58[-0x1].field0_0x0 + 1);
        if uStack58 >= 0x8000 {
            break;
        }
    }
    brush_handle_var6 = CreateSolidBrush16(0x2000000);
    uVar7 = (puStack26 >> 0x10);
    rect16_a = CONCAT22((puStack26 + 0x12) + 0x1, rect16_a.x);
    uVar2 = (puStack26 + 0x14);
    uStack14 = uStack14 & 0xffff | uVar2 << 0x10;
    x_var_6 = CONCAT22(uVar2, x_var_6);
    FillRect16(brush_handle_var6, &rect16_a, &DAT_1050_1050);
    DeleteObject16(brush_handle_var6);
    iStack62 = 0x8;
    puVar2 = &PTR_LOOP_1050_0000;
    while (uStack58 = (puVar2 + 1), uStack58 < 0xa) {
        brush_handle_var6 = CreateSolidBrush16(*(puStack30 + iStack62 * 0x4 + 0x4));
        x_var_6 = x_var_6 & 0xffff | (rect16_a.y - 1) << 0x10;
        rect16_var_12 = (rect16_var_12 & 0xffff | (uStack14 + 1) << 0x10);
        uVar3 = (puStack26 >> 0x10);
        rect16_a = (rect16_a & 0xffff | ((iStack62 * 0x2 + puStack26) + 1) << 0x10);
        uStack14 = uStack14 & 0xffff | (uStack58 * 0x2 + puStack26 + 0x14) << 0x10;
        FillRect16(brush_handle_var6, &rect16_a, &DAT_1050_1050);
        FillRect16(brush_handle_var6, &rect16_var_12, &DAT_1050_1050);
        DeleteObject16(brush_handle_var6);
        iStack62 += -0x1;
        puVar2 = &uStack58.field0_0x0;
    }
    brush_handle_var6 = CreateSolidBrush16(*puStack30);
    rect16_a = (rect16_a & 0xffff);
    x_var_6 = x_var_6 & 0xffff | *puStack26 << 0x10;
    rect16_var_12 = (rect16_var_12 & 0xffff | ((uStack20 * 0x2 + puStack26) + 1) << 0x10);
    uStack14 = uStack14 & 0xffff | local_struct_1.field7_0xe << 0x10;
    FillRect16(brush_handle_var6, &rect16_a, &DAT_1050_1050);
    FillRect16(brush_handle_var6, &rect16_var_12, &DAT_1050_1050);
    DeleteObject16(brush_handle_var6);
    uStack58 = (&u16_1050_0002 + 1);
    loop {
        HVar3 = hdc16_var_48;
        HStack50 = CreatePen16(*(uStack58 * 0x4 + uStack38), 0x0, 0x0);
        hgdiobj16_var_52 = SelectObject16(HStack50, HVar3);
        iVar2 = uStack58 * 0x2;
        rect16_a.x = (iVar2 + uStack42) + rect16_a.x;
        uVar8 = (uStack46 >> 0x10);
        piVar1 = (iVar2 + uStack46);
        MoveToEx16(
            &point16_38,
            &DAT_1050_1050,
            ((iVar2 + uStack46) * 0x2 + puStack26),
            rect16_a.x,
        );
        LineTo16(
            ((uStack20 - *piVar1) * 0x2 + puStack26),
            rect16_a.x,
            hdc16_var_48,
        );
        rect16_a.x = ((iStack22 - uStack58) * 0x2 + uStack42) + rect16_a.x;
        MoveToEx16(
            &point16_38,
            &DAT_1050_1050,
            (*piVar1 * 0x2 + puStack26),
            rect16_a.x,
        );
        LineTo16(
            ((uStack20 - *piVar1) * 0x2 + puStack26),
            rect16_a.x,
            hdc16_var_48,
        );
        SelectObject16(hgdiobj16_var_52, hdc16_var_48);
        DeleteObject16(HStack50);
        uStack58 = (&uStack58[-0x1].field0_0x0 + 1);
        if uStack58 >= 0x8000 {
            break;
        }
    }

    local_struct_1.field8_0x10 = 0;
    return;
}


// WARNING: Unable to use type for symbol uVar4
pub unsafe fn draw_line_1040_c302(param_1: *mut astruct_772, param_2: HDC16)

{
    let mut uVar3: u32;
    let mut uVar5: u16;
    let mut iVar7: *mut astruct_794;
    let mut iVar6: *mut astruct_793;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar2: u32;
    let mut uVar4: u32;
    let mut iVar1: i16;
    let mut uVar1: u32;

    uVar6 = (param_1 >> 0x10);
    uVar4 = (param_1 + 0x6);
    iVar1 = (uVar4 + 0x16);
    if (0x1 < iVar1) {
        uVar2 = (param_1 + 0x6);
        uVar5 = uVar2;
        uVar5 = uVar5 + 0x2a;
        uVar1 = (uVar2 & 0xffff0000 | uVar5);
        iVar7 = uVar1;
        iVar7 = &iVar7.field_0x1e;
        uVar7 = ((uVar1 & 0xffff0000) >> 0x10);
        MoveTo16(iVar7.field32_0x20 + iVar7.field34_0x24,
                 iVar7.field33_0x22 / 0x2 + (uVar1 & 0xffff0000 | ZEXT24(iVar7)), param_2);
        uVar3 = (uVar5 + iVar1 * 0x4 - 0x4);
        iVar6 = uVar3;
        iVar6 = &iVar6.field_0x1e;
        uVar3 &= 0xffff0000;
        uVar8 = (uVar3 >> 0x10);
        LineTo16(iVar6.field32_0x20, iVar6.field33_0x22 / 0x2 + (uVar3 | ZEXT24(iVar6)), param_2);
    }
    return;
}


// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar5
// WARNING: Unable to use type for symbol uVar4
// WARNING: Unable to use type for symbol uVar6
// WARNING: Unable to use type for symbol uVar7
pub unsafe fn draw_op_1040_c38e(param_1: *mut astruct_772)

{
    let mut iVar1: i16;
    let mut uVar8: u32;
    let mut iVar5: i16;
    let mut iVar11: i16;
    let mut y1: i16;
    let mut iVar12: i16;
    let mut in_DX: u16;
    let mut iVar10: *mut astruct_772;
    let mut uVar10: u16;
    let mut uVar9: u16;
    let mut uVar11: u16;
    let mut unaff_SS: u16;
    let mut DVar10: u32;
    let mut DVar9: u32;
    let mut in_stack_00000008: HDC16;
    let mut iStack26: i16;
    let mut x3: i16;
    let mut y5: i16;
    let mut x2: i16;
    let mut y4: i16;
    let mut uVar2: u32;
    let mut uVar1: u32;
    let mut uVar5: u32;
    let mut x1: *mut i16;
    let mut uVar4: u32;
    let mut uVar3: u32;
    let mut uVar6: u32;
    let mut uVar7: u32;

    uVar10 = (param_1 >> 0x10);
    iVar10 = param_1;
    uVar2 = iVar10.field5_0x6;
    iVar1 = (uVar2 + 0x18);
    if ((iVar1 != 0) && (uVar4 = iVar10.field5_0x6, (uVar4 + 0x16) != 0)) {
        iVar5 = iVar1;
        pass1_1010_2ee2(iVar10.field5_0x6);
        for iStack26 in 0..iVar1 {
            uVar3 = (iStack26 * 0x4 + iVar5);
            iVar11 = uVar3;
            iVar11 = iVar11 + 0x1e;
            x1 = (uVar3 & 0xffff0000 | iVar11);
            uVar9 = ((uVar3 & 0xffff0000) >> 0x10);
            y1 = (iVar11 + 0x24) / 0x2 + (iVar11 + 0x20);
            MoveTo16(y1, *x1, in_stack_00000008);
            LineTo16(y1, *x1 - 0xf, in_stack_00000008);
            DVar10 = GetCurrentPosition16(in_stack_00000008);
            y5 = (DVar10 >> 0x10);
            x3 = DVar10;
            if (iStack26 == 0) {
                x2 = x3;
                y4 = y5;
            }
        }
        uVar6 = iVar10.field5_0x6;
        if ((uVar6 + 0x24) != 0) {
            y4 += -0xd;
        }
        uVar7 = iVar10.field5_0x6;
        if ((uVar7 + 0x26) != 0) {
            y5 += 0xd;
        }
        uVar8 = iVar10.field5_0x6;
        uVar5 = iVar10.field5_0x6;
        uVar1 = (uVar8 + (uVar5 + 0x16) * 0x4 + 0x26);
        iVar12 = uVar1;
        iVar12 = iVar12 + 0x1e;
        uVar11 = ((uVar1 & 0xffff0000) >> 0x10);
        MoveTo16((iVar12 + 0x24) / 0x2 + (iVar12 + 0x20),
                 (iVar12 + 0x22) + (uVar1 & 0xffff0000 | iVar12), in_stack_00000008);
        LineTo16((iVar12 + 0x24) / 0x2 + (iVar12 + 0x20), x2, in_stack_00000008);
        DVar9 = GetCurrentPosition16(in_stack_00000008);
        DVar9 = (DVar9 >> 0x10);
        if (DVar9 < y4) {
            y4 = DVar9;
        }
        if (y5 < DVar9) {
            y5 = DVar9;
        }
        MoveTo16(y4, x2, in_stack_00000008);
        LineTo16(y5, x3, in_stack_00000008);
    }
    return;
}


pub unsafe fn draw_op_1040_5a06(mut param_1: u32, struct741_param_1: *mut astruct_741)

{
    let mut uVar1: u16;
    let mut caption_height_px: i16;
    let mut IVar2: i16;
    let mut handle: HPEN16;
    let mut handle_00: HGDIOBJ16;
    let mut IVar3: i16;
    let mut y: i16;
    let mut IVar4: i16;
    let mut y_00: i16;
    let mut x: i16;
    let mut IVar5: i16;
    let mut in_DX: u16;
    let mut uVar6: u16;
    let mut palette_handle_7: HPALETTE16;
    let mut puVar2: *mut u32;
    let mut uVar8: u32;
    let mut struct_1: *mut astruct_741;
    let mut uVar9: u16;
    let mut base_addr: u16;
    let mut uVar11: u32;
    let mut uVar12: u32;
    let mut uVar10: u8;
    let mut uVar14: u16;
    let mut uStack82: u16;
    let mut iStack72: i16;
    let mut iStack68: i16;
    let mut puStack54: *mut u32;
    let mut hdc16_2c: HDC16;
    let mut paint_struct_2a: [u8; 0x20] = [0; 0x20];
    let mut rect_array_local_a: [u8; 0x8] = [0; 0x8];
    let mut uVar13: u16;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut uVar4: u32;
    let mut puVar1: *mut u16;
    let mut uVar5: u16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar7: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar9 = (struct741_param_1 >> 0x10);
    struct_1 = struct741_param_1;
    GetWindowRect16(CONCAT13(0x10, CONCAT12(0x50, rect_array_local_a)), struct_1.field6_0x6);
    hdc16_2c = BeginPaint16(CONCAT13(0x10, CONCAT12(0x50, paint_struct_2a)), struct_1.field6_0x6);
    base_addr = 0x1008;
    palette_handle_7 = palette_op_1008_4e08(&hdc16_2c, param_1, (_PTR_LOOP_1050_4230 + 0xe),
                                            CONCAT13(0x10, CONCAT12(0x50, &hdc16_2c)));
    puVar2 = null_mut();
    puStack54 = null_mut();
    uVar7 = param_1;
    if (struct_1.field149_0x98 != 0) {
        uVar1 = FUN_1010_830a(0x0, param_1, 0x1008, _u16_1050_14cc, struct_1.field149_0x98);
        uVar14 = param_1;
        puStack54 = CONCAT22(uVar14, uVar1);
        uVar7 = param_1;
        uVar11 = pass1_1008_4772(CONCAT22(uVar14, uVar1));
        uVar6 = (uVar11 >> 0x10) | (uVar11 & 0xffff);
        uVar7 = uVar7 & 0xffff0000 | uVar6;
        if (uVar6 == 0) {
            puVar2 = (uVar11 & 0xffff);
            if (puStack54.is_null() == false) {
                puVar2 = puStack54;
                if (puStack54.is_null() == false) {
                    fn_ptr_1 = *puStack54;
                    (**fn_ptr_1)(0x8, uVar1, param_1, 0x1, uVar14);
                    puVar2 = puStack54;
                }
            }
            uVar1 = FUN_1010_830a(puVar2, uVar7, 0x1008, _u16_1050_14cc, 0x4d);
            puStack54 = CONCAT22(uVar7, uVar1);
        }
        uVar13 = &DAT_1050_1050;
        uVar10 = SUB21(&hdc16_2c, 0x0);
        base_addr = s_tile2_bmp_1050_1538;
        caption_height_px = GetSystemMetrics16(SM_CYCAPTION);
        puVar2 = -(caption_height_px - 0x23);
        fn_ptr_1 = (*puStack54 + 0x4);
        (**fn_ptr_1)(0x38, puStack54, (puStack54 >> 0x10), -(caption_height_px - 0x23), uVar10, uVar13);
    }
    if (puStack54.is_null() == false) {
        uVar1 = (puStack54 >> 0x10);
        puVar2 = puStack54;
        if (puStack54.is_null() == false) {
            fn_ptr_1 = *puStack54;
            (**fn_ptr_1)(base_addr, puStack54, uVar1, 0x1, puStack54, uVar1);
            puVar2 = puStack54;
        }
    }
    uVar1 = FUN_1010_830a(puVar2, uVar7, base_addr, _u16_1050_14cc, struct_1.field148_0x96);
    puStack54 = CONCAT22(uVar7, uVar1);
    uVar14 = SUB42(&DAT_1050_1050, 0x0);
    uVar10 = SUB21(&hdc16_2c, 0x0);
    uVar8 = uVar7;
    IVar2 = GetSystemMetrics16(SM_CYCAPTION);
    uVar3 = *puStack54;
    fn_ptr_1 = uVar3 + 2;
    (**fn_ptr_1)(0x38, uVar1, uVar7, -(IVar2 - 0x23), uVar10, uVar14);
    if (puStack54.is_null() == false) {
        if (puStack54.is_null() == false) {
            fn_ptr_1 = uVar3;
            (**fn_ptr_1)(s_tile2_bmp_1050_1538, uVar1, uVar7, 1);
        }
    }
    handle = CreatePen16(0x1000025, 0x0, 0x0);
    handle_00 = SelectObject16(handle, hdc16_2c);
    uVar14 = FUN_1010_830a(handle_00, uVar8, s_tile2_bmp_1050_1538, _u16_1050_14cc, 0x4f);
    puStack54 = CONCAT22(uVar8, uVar14);
    uVar12 = pass1_1008_4772(CONCAT13((uVar8 >> 0x8), CONCAT12(uVar8, uVar14)));
    uVar1 = (uVar12 >> 0x10);
    uVar4 = (uVar12 + 0x4);
    uVar2 = (uVar12 + 0x8);
    IVar3 = GetSystemMetrics16(SM_CYCAPTION);
    y = -(IVar3 - 0xc1);
    IVar4 = GetSystemMetrics16(SM_CYCAPTION);
    iStack72 = uVar2;
    y_00 = 0xc5 - (IVar4 - iStack72);
    MoveTo16(y, 0x82, hdc16_2c);
    iStack68 = uVar4;
    x = iStack68 * 0xa + 0x85;
    LineTo16(y, x, hdc16_2c);
    LineTo16(y_00, x, hdc16_2c);
    LineTo16(y_00, 0x82, hdc16_2c);
    LineTo16(y, 0x82, hdc16_2c);
//   for (uStack82 = 0; puVar1 = &struct_1.field147_0x94, uStack82 <= *puVar1 && *puVar1 != uStack82; uStack82 += 1)
    uStack82 = 0;
    puVar1 = &struct_1.field147_0x94;
    while uStack82 <= *puVar1 && *puVar1 != uStack82 {
        IVar5 = GetSystemMetrics16(SM_CYCAPTION);
        fn_ptr_1 = (*puStack54 + 0x4);
        (**fn_ptr_1)(s_tile2_bmp_1050_1538, uVar14, uVar8, -(IVar5 - 0xc4));
        uStack82 += 1;
    }
    if (puStack54.is_null() == false) {
        if (puStack54.is_null() == false) {
            fn_ptr_1 = *puStack54;
            (**fn_ptr_1)(s_tile2_bmp_1050_1538, uVar14);
        }
    }
    SelectObject16(handle_00, hdc16_2c);
    DeleteObject16(handle);
    palette_handle_7 = SelectPalette16(0x0, palette_handle_7, hdc16_2c);
    DeleteObject16(palette_handle_7);
    EndPaint16(CONCAT22(0x1050, paint_struct_2a), struct_1.field6_0x6);
    return;
}


pub unsafe fn draw_op_1040_c74c(param_1: *mut astruct_738, mut param_2: u16, hdc16_param_3: HDC16, mut param_4: u16)

{
    let mut uVar2: u16;
    let mut hdc_black_brush_1: HGDIOBJ16;
    let mut pen_handle_1: HPEN16;
    let mut handle: HGDIOBJ16;
    let mut hpalette_1: HPALETTE16;
    let mut struct_1: *mut astruct_738;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar3: u32;
    let mut uVar1: u16;
    let mut func_ptr_1: *mut *mut code;

    uVar4 = (_PTR_LOOP_1050_4230 >> 0x10);
    uVar2 = (_PTR_LOOP_1050_4230 + 0x10);
    hpalette_1 = palette_op_1008_4e08(&hdc16_param_3, uVar2,
                                      CONCAT22(uVar2, (_PTR_LOOP_1050_4230 + 0xe)),
                                      CONCAT13(0x10, CONCAT12(0x50, &hdc16_param_3)));
    uVar5 = (param_1 >> 0x10);
    struct_1 = param_1;
    struct_1.field66_0x46 = 0x1;
    hdc_black_brush_1 = GetStockObject16(BLACK_BRUSH);
    pen_handle_1 = CreatePen16(0x1000002, 0x1, 0x0);
    hdc_black_brush_1 = SelectObject16(hdc_black_brush_1, hdc16_param_3);
    handle = SelectObject16(pen_handle_1, hdc16_param_3);
    Rectangle16(struct_1.field35_0x24, struct_1.field34_0x22, 0x0, 0x0, hdc16_param_3);
    MoveTo16(0x0, struct_1.field51_0x36 * 0x2 + struct_1.field40_0x2a, hdc16_param_3);
    LineTo16(struct_1.field35_0x24, struct_1.field51_0x36 * 0x2 + struct_1.field40_0x2a, hdc16_param_3);
    SelectObject16(hdc_black_brush_1, hdc16_param_3);
    hdc_black_brush_1 = SelectObject16(handle, hdc16_param_3);
    DeleteObject16(hdc_black_brush_1);
    uVar3 = param_1;
    func_ptr_1 = (uVar3 + 0x10);
    (**func_ptr_1)(s_tile2_bmp_1050_1538, param_1, _param_2);
    func_ptr_1 = (uVar3 + 0x14);
    (**func_ptr_1)(s_tile2_bmp_1050_1538, struct_1, (param_1 >> 0x10), hdc16_param_3);
    struct_1.field66_0x46 = 0;
    hpalette_1 = SelectPalette16(0x0, hpalette_1, hdc16_param_3);
    DeleteObject16(hpalette_1);
    return;
}


pub unsafe fn draw_rect_1020_40ce(
    mut param_1: u32,
    mut param_2: i16,
    mut param_3: i16,
    hdc16_param_4: HDC16,
    mut param_5: u16,
) {
    let mut pen_handle: HPEN16;
    let mut brush_handle_1: HGDIOBJ16;
    let mut right: i16;
    let mut bottom: i16;
    let mut unaff_SS: u16;
    let mut hdc: HDC16;
    let mut local_6: i16;
    let mut local_4: i16;
    let mut hdc16_var_fff2: HDC16;
    let mut iVar1: i16;

    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (param_1 + 0x10)),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    pass1_1008_3e94(
        param_1,
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    iVar1 = (param_1 + 0xa);
    Ellipse16(
        iVar1 + local_6 + param_2,
        iVar1 + local_4 + param_3,
        (local_6 - (param_1 + 0xa)) + param_2,
        (local_4 - (param_1 + 0xa)) + param_3,
        hdc16_param_4,
    );
    if ((*(param_1 + 0xe) & 1) != 0) {
        brush_handle_1 = GetStockObject16(HOLLOW_BRUSH);
        SelectObject16(brush_handle_1, hdc16_var_fff2);
        hdc = hdc16_param_4;
        pen_handle = CreatePen16(0x10000f9, 0x1, 0x0);
        SelectObject16(pen_handle, hdc);
        right = local_4 + param_3 + 0x5;
        bottom = local_6 + param_2 + 0x5;
        Rectangle16(
            bottom,
            right,
            local_6 + param_2 -0x5,
            local_4 + param_3 -0x5,
            hdc16_param_4,
        );
        brush_handle_1 = GetStockObject16(WHITE_BRUSH);
        SelectObject16(brush_handle_1, right);
        brush_handle_1 = GetStockObject16(WHITE_PEN);
        brush_handle_1 = SelectObject16(brush_handle_1, bottom);
        DeleteObject16(brush_handle_1);
    }
    return;
}

pub unsafe fn draw_op_1010_47d0(param_1: *mut Struct27, mut param_2: u16, mut param_3: u16) {
    let mut piVar1: *mut i16;
    let mut puVar3: *mut u32;
    let mut iVar5: i16;
    let mut hpalette16_var6: HPALETTE16;
    let mut handle: HGDIOBJ16;
    let mut hgdiobj16_00: HGDIOBJ16;
    let mut iVar4: *mut astruct_797;
    let mut uVar5: u16;
    let mut extraout_DX: *mut u8;
    let mut puVar5: *mut u8;
    let mut iVar7: i16;
    let mut iVar8: *mut astruct_739;
    let mut uVar11: u16;
    let mut uVar12: u32;
    let mut iStack32: i16;
    let mut hdc16_var_1: HDC16;
    let mut devmodea_init_data: u16;
    let mut uStack16: u16;
    let mut local_e: u16;
    let mut uStack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut stock_obj_handle: HGDIOBJ16;
    let mut pen_handle: HPEN16;
    let mut uVar4: u32;
    let mut puVar2: *mut u32;
    let mut uVar13: u16;
    let mut uVar14: u8;
    let mut uVar15: u8;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut offset_1: u16;
    let mut base_addr_1: u16;
    let mut fn_ptr_1: *mut *mut code;

    pen_handle = CreatePen16(0x77d7fb, 0x1, 0x0);
    stock_obj_handle = GetStockObject16(HOLLOW_BRUSH);
    local_e = 0;
    uStack12 = 0;
    uStack10 = 0x1;
    uStack8 = 0x1;
    puVar3 = (&param_1.field_0x26 + param_3 * 0x4);
    puVar5 = (&param_1.field33_0x28)[param_3 * 0x2];
    if ((puVar5 | puVar3) != 0) {
        fn_ptr_1 = *puVar3;
        (**fn_ptr_1)(s_tile2_bmp_1050_1538, puVar3, puVar5, 1);
        puVar5 = extraout_DX;
    }
    iVar5 = param_3 + 0x105;
    pass1_1010_8170(puVar5, _u16_1050_14cc, iVar5);
    iVar8 = (param_3 * 0x4);
    (iVar8 + (&param_1.field_0x0 + 0x26)) = iVar5;
    (iVar8 + (&param_1.field_0x0 + 0x28)) = puVar5;
    base_addr_1 = &DAT_1050_1050;
    offset_1 = 0x1380;
    uVar16 = 0;
    uVar17 = 0;
    uVar13 = 0;
    uVar14 = '\0';
    uVar15 = '\0';
    uVar12 = pass1_1008_4772((&param_1.field_0x26 + iVar8));
    uVar12 = (uVar12 >> 0x10);
    devmodea_init_data = uVar12;
    uStack16 = uVar12;
    hdc16_var_1 = CreateDC16(
        (uVar12 & 0xffff | uVar12 << 0x10),
        CONCAT13(uVar15, CONCAT12(uVar14, uVar13)),
        CONCAT22(uVar17, uVar16),
        CONCAT22(base_addr_1, offset_1),
    );
    hpalette16_var6 = palette_op_1008_4e08(
        &hdc16_var_1,
        uVar12,
        (_PTR_LOOP_1050_4230 + 0xe),
        CONCAT22(0x1050, &hdc16_var_1),
    );
    handle = SelectObject16(pen_handle, hdc16_var_1);
    hgdiobj16_00 = SelectObject16(stock_obj_handle, hdc16_var_1);
    iStack32 = 0;
    loop {
        piVar1 = &param_1[0x1].field_0x1e;
        if (*piVar1 == iStack32 || *piVar1 < iStack32) {
            break;
        }
        iVar4 = ((iStack32 * 0x10 + param_3) * 0x8);
        uVar5 = pass1_1000_484c(
            CONCAT22(0x1050, &local_e),
            CONCAT22(&param_1[0x1].field_0x1c, iVar4 + param_1[0x1].field23_0x1a),
            0x8,
        );
        if (uVar5 != 0) {
            uVar4 = &param_1[0x1].field23_0x1a;
            uVar11 = (uVar4 >> 0x10);
            iVar7 = uVar4;
            Rectangle16(
                (iVar4 + iVar7 + 0x6),
                (iVar4 + iVar7 + 0x4),
                (iVar4 + iVar7 + 0x2),
                (iVar4 + iVar7),
                hdc16_var_1,
            );
        }
        iStack32 += 0x1;
    }
    hpalette16_var6 = SelectPalette16(0x0, hpalette16_var6, hdc16_var_1);
    DeleteObject16(hpalette16_var6);
    SelectObject16(handle, hdc16_var_1);
    SelectObject16(hgdiobj16_00, hdc16_var_1);
    DeleteDC16(hdc16_var_1);
    DeleteObject16(pen_handle);
    return;
}

// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
pub unsafe fn draw_op_1020_30be(struct_param_1: *mut astruct_762) {
    let mut is_iconic: bool;
    let mut iVar5: *mut astruct_762;
    let mut uVar5: *mut astruct_762;
    let mut pHVar1: *mut HDC16;
    let mut pHVar2: *mut HDC16;
    let mut rect_30: [RECT16; 2] = [RECT16::default(); 2];
    let mut hbrush_40: HGDIOBJ16;
    let mut hicon_38: HICON16;
    let mut local_24: HDC16;
    let mut local_22: [u8; 0x20] = [0; 0x20];
    let mut IVar4: i16;
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar5 = (struct_param_1 >> 0x10);
    iVar5 = struct_param_1;
    local_24 = BeginPaint16(CONCAT22(0x1050, local_22), iVar5.field4_0x4);
    is_iconic = IsIconic16(iVar5.field4_0x4);
    if (is_iconic == 0) {
        pass1_1018_0a50(iVar5.field19_0x14);
        pHVar2 = &local_24;
        IVar4 = &DAT_1050_1050;
        fn_ptr_1 = (CONCAT22(0x1050, pHVar2) + 0x8);
        pHVar1 = pHVar2;
        (**fn_ptr_1)(0x1018, pHVar2, &DAT_1050_1050);
        uVar2 = iVar5.field19_0x14;
        if ((uVar2 + 0x84) == 1) {
            unk_draw_op_1020_320e(struct_param_1, local_24);
        }
        fn_ptr_1 = (CONCAT22(IVar4, pHVar2) + 0x4);
        (**fn_ptr_1)(
            0x1018,
            pHVar2,
            IVar4,
            0x0,
            0x0,
            &local_24,
            &DAT_1050_1050,
            pHVar1,
        );
        uVar1 = iVar5.field19_0x14;
        if ((uVar1 + 0x84) != 1) {
            unk_draw_op_1020_320e(struct_param_1, local_24);
        }
        draw_op_1020_3488(struct_param_1);
        fn_ptr_1 = (CONCAT22(IVar4, pHVar2) + 0xc);
        (**fn_ptr_1)(0x1018, pHVar2, IVar4, &local_24, &DAT_1050_1050);
    } else if (PTR_LOOP_1050_0010.is_null()) {
        fn_ptr_1 = (iVar5.field19_0x14 + 0x2c);
        hicon_38 = (**fn_ptr_1)(s_tile2_bmp_1050_1538);
        if (hicon_38 != 0) {
            hbrush_40 = GetStockObject16(BLACK_BRUSH);
            GetClientRect16(rect_30, &DAT_1050_1050);
            FillRect16(hbrush_40, &stack0xffc4, &DAT_1050_1050);
            DrawIcon16(hicon_38, 0x2, 0x2, local_24);
        }
    }
    EndPaint16(CONCAT22(0x1050, local_22), iVar5.field4_0x4);
    return;
}

// WARNING: Unable to use type for symbol uVar4
pub unsafe fn unk_draw_op_1020_320e(astruct762_param_1: *mut astruct_762, hdc_param_2: HDC16) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut obj: HPALETTE16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut struct_a: *mut astruct_762;
    let mut iVar7: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: *mut DEVMODEA;
    let mut device: *mut c_char;
    let mut driver: *mut c_char;
    let mut local_c: i16;
    let mut local_a: u32;
    let mut pHStack6: *mut HDC16;
    let mut hdc_var4: HDC16;
    let mut puVar2: *mut u32;
    let mut uVar4: u32;
    let mut uVar9: u8;
    let mut uVar10: u8;
    let mut uVar11: u16;

    hdc_var4 = hdc_param_2;
    uVar6 = (astruct762_param_1 >> 0x10);
    struct_a = astruct762_param_1;
    uVar4 = struct_a.field19_0x14;
    if ((uVar4 + 0x84) == 1) {
        uVar3 = struct_a.field19_0x14;
        uVar7 = (uVar3 >> 0x10);
        iVar7 = uVar3;
        puVar1 = (iVar7 + 0x24);
        driver = s_dib_1050_42c6;
        device = null_mut();
        uVar9 = '\0';
        uVar10 = '\0';
        uVar11 = 0;
        uVar8 = pass1_1008_4772((puVar1 & 0xffff | (iVar7 + 0x26) << 0x10));
        hdc_var4 = CreateDC16(
            uVar8,
            CONCAT22(uVar11, CONCAT11(uVar10, uVar9)),
            device,
            driver,
        );
        pHStack6 = &hdc_var4;
        ppcVar2 = (*puVar1 + 0x8);
        (**ppcVar2)(s_tile2_bmp_1050_1538);
        in_DX = extraout_DX;
    }
    pass1_1018_0d9a(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    uVar3 = struct_a.field19_0x14;
    draw_op_1020_33c0(
        in_DX,
        astruct762_param_1,
        *(uVar3 + 0x6c),
        local_c,
        local_a,
        0x1,
        hdc_var4,
    );
    pass1_1018_1054(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    uVar3 = struct_a.field19_0x14;
    draw_op_1020_33c0(
        in_DX,
        astruct762_param_1,
        *(uVar3 + 0x74),
        local_c,
        local_a,
        0x2,
        hdc_var4,
    );
    pass1_1018_1320(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    uVar3 = struct_a.field19_0x14;
    draw_op_1020_33c0(
        in_DX,
        astruct762_param_1,
        *(uVar3 + 0x68),
        local_c,
        local_a,
        0x1,
        hdc_var4,
    );
    pass1_1018_15f6(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    if (local_c != 0) {
        uVar3 = struct_a.field19_0x14;
        draw_op_1020_33c0(
            in_DX,
            astruct762_param_1,
            *(uVar3 + 0x70),
            local_c,
            local_a,
            0x1,
            hdc_var4,
        );
    }
    pass1_1018_108c(
        struct_a.field19_0x14,
        CONCAT22(0x1050, &local_c),
        CONCAT22(0x1050, &local_a),
    );
    if (local_c != 0) {
        uVar3 = struct_a.field19_0x14;
        draw_op_1020_33c0(
            in_DX,
            astruct762_param_1,
            *(uVar3 + 0x78),
            local_c,
            local_a,
            0x0,
            hdc_var4,
        );
    }
    uVar3 = struct_a.field19_0x14;
    if ((uVar3 + 0x84) == 1) {
        obj = SelectPalette16(0x0, pHStack6, hdc_var4);
        DeleteObject16(obj);
        DeleteDC16(hdc_var4);
    }
    return;
}

pub unsafe fn draw_op_1020_33c0(
    mut param_1: u16,
    mut param_2: u32,
    colorref_param_2: COLORREF,
    mut param_4: i16,
    mut param_5: u32,
    mut param_6: i16,
    hdc16_param_6: HDC16,
) {
    let mut pen_handle: HPEN16;
    let mut object_handle: HGDIOBJ16;
    let mut brush_handle: HBRUSH16;
    let mut obj_handle_2: HGDIOBJ16;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_DX: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut uVar6: u16;
    let mut HVar7: HDC16;
    let mut iStack20: i16;
    let mut puStack14: *mut u16;
    let mut uVar5: u16;

    if (param_4 != 0) {
        HVar7 = hdc16_param_6;
        pen_handle = CreatePen16(colorref_param_2, 0x1, 0x0);
        object_handle = SelectObject16(pen_handle, HVar7);
        HVar7 = hdc16_param_6;
        brush_handle = CreateSolidBrush16(colorref_param_2);
        obj_handle_2 = SelectObject16(brush_handle, HVar7);
        puStack14 = param_5;
        for iStack20 in 0..param_4 {
            uVar6 = (param_2 >> 0x10);
            uVar1 = param_4;
            block_1020_3000::pass1_1020_3540(param_1, param_2, uVar6, param_6, puStack14);
            if (param_6 < 1) {
                uVar2 = 0x3;
            } else {
                uVar2 = 0x4;
            }
            uVar3 = param_1;
            draw_polygon_1020_3602(param_2, uVar6, uVar2, uVar1, param_1);
            fn_ptr_1000_17ce(CONCAT22(param_1, uVar1));
            puStack14 = (puStack14 & 0xffff0000 | (puStack14 + 0x6));
            param_1 = uVar3;
        }
        obj_handle_2 = SelectObject16(obj_handle_2, hdc16_param_6);
        DeleteObject16(obj_handle_2);
        SelectObject16(object_handle, hdc16_param_6);
        DeleteObject16(pen_handle);
    }
    return;
}

// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar3
pub unsafe fn draw_op_1020_3488(param_1: *mut astruct_762) {
    let mut uVar6: u32;
    let mut handle: HPEN16;
    let mut handle_00: HGDIOBJ16;
    let mut obj_handle_7: HGDIOBJ16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut in_stack_00000008: HDC16;
    let mut hdc: HDC16;
    let mut local_a: u32;
    let mut puStack6: *mut u16;
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut left: i16;
    let mut hdc16_ffe2: HDC16;

    uVar5 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0x14);
    puStack6 = (uVar2 & 0xffff0000 | (uVar2 + 0x36));
    pass1_1008_3e94(
        puStack6,
        CONCAT22(0x1050, &local_a),
        CONCAT22(0x1050, &local_a + 0x2),
    );
    uVar4 = (local_a - 0x3) << 0x10;
    if ((local_a - 0x3) < 0x0) {
        uVar4 = 0;
    }
    uVar1 = local_a - 0x3;
    uVar6 = uVar1;
    if (uVar1 < 0x0) {
        uVar6 = 0;
    }
    local_a = uVar4 | uVar6;
    uVar3 = (param_1 + 0x14);
    hdc = in_stack_00000008;
    handle = CreatePen16(*(uVar3 + 0x64), 0x1, 0x0);
    handle_00 = SelectObject16(handle, hdc);
    obj_handle_7 = GetStockObject16(HOLLOW_BRUSH);
    obj_handle_7 = SelectObject16(obj_handle_7, hdc16_ffe2);
    left = (local_a >> 0x10);
    Rectangle16(local_a + 0x6, left + 0x6, local_a, left, in_stack_00000008);
    SelectObject16(handle_00, in_stack_00000008);
    SelectObject16(obj_handle_7, in_stack_00000008);
    DeleteObject16(handle);
    return;
}

pub unsafe fn load_draw_op_1020_2ede(
    param_1: *mut Struct57,
    mut param_2: u16,
    param_3: *mut astruct_40,
    param_4: *mut StructA,
    mut param_5: u16,
) {
    let mut hdc_dev_ctx_1: HDC16;
    StructA * *ppSVar1;
    let mut handle: HPEN16;
    let mut pHVar2: *mut HDC16;
    let mut h_null_brush: HGDIOBJ16;
    let mut in_DX: *mut u8;
    let mut struct_1: *mut astruct_40;
    let mut unaff_DI: i16;
    let mut uVar5: u16;
    let mut unaff_SS: u16;
    let mut puVar5: *mut u16;
    let mut paVar3: *mut astruct_76;
    let mut devmodea_ptr_var11: *mut DEVMODEA;
    let mut in_stack_0000fe92: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut device: *mut c_char;
    let mut driver: *mut c_char;
    let mut in_stack_0000ffea: u16;
    let mut puVar1: *mut u32;
    let mut puVar2: u32;
    let mut uVar10: COLORREF;
    let mut output: LPCSTR = null_mut();
    let mut fn_ptr_1: *mut *mut code;

    get_sys_metrics_1020_7c1a(param_3, CONCAT22(param_5, param_4));
    uVar5 = (param_3 >> 0x10);
    struct_1 = param_3;
    struct_1.field17_0x14 = 0;
    struct_1.field20_0x18 = 0;
    (&struct_1.field20_0x18 + 0x2) = 0;
    struct_1.field21_0x1c = null_mut();
    struct_1.field_0x1e = 0;
    (&struct_1[0x1].field0_0x0 + 1) = 0;
    param_3.field0_0x0 = 0x363c;
    struct_1.field1_0x2 = 0x1020;
    puVar5 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_2, param_4[0x1].field26_0x30),
        in_stack_0000fe92,
        in_stack_0000ffb6,
        in_stack_0000ffbc,
        in_stack_0000ffc0,
    );
    struct_1.field17_0x14 = puVar5;
    struct_1.field_0x16 = (puVar5 >> 0x10);
    fn_ptr_1 = (*&struct_1.field17_0x14 + 0x4);
    (**fn_ptr_1)();
    driver = s_dib_1050_42c2;
    device = null_mut();
    output = null_mut();
    paVar3 = pass1_1018_0a50(&struct_1.field17_0x14);
    devmodea_ptr_var11 = pass1_1008_4772(paVar3);
    hdc_dev_ctx_1 = CreateDC16(devmodea_ptr_var11, output, device, driver);
    *&struct_1.field20_0x18 = hdc_dev_ctx_1;
    ppSVar1 = &struct_1.field20_0x18;
    fn_ptr_1 = (paVar3 + 0x8);
    (**fn_ptr_1)();
    (&struct_1[0x1].field0_0x0 + 1) = ppSVar1;
    puVar2 = &struct_1.field17_0x14;
    handle = CreatePen16(*(puVar2 + 0x64), 0x1, 0x0);
    (&struct_1.field20_0x18 + 0x2) = handle;
    pHVar2 = SelectObject16(handle, *&struct_1.field20_0x18);
    struct_1.field21_0x1c = pHVar2;
    h_null_brush = GetStockObject16(HOLLOW_BRUSH);
    h_null_brush = SelectObject16(h_null_brush, *&struct_1.field20_0x18);
    struct_1.field_0x1e = h_null_brush;
    return;
}

pub unsafe fn cleanup_win_ui_1020_2fea(in_struct_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut struct_1: *mut StructD;
    let mut var2: u16;
    let mut unaff_SS: u16;

    var2 = (in_struct_1 >> 0x10);
    struct_1 = in_struct_1;
    in_struct_1.address_offset_field_0x0 = 0x363c;
    struct_1.address_offset_field_0x2 = 0x1020;
    if (struct_1.field12_0x14 != 0) {
        pass1_1010_1ea6(struct_1.field12_0x14, (in_struct_1 & 0xffff | var2 << 0x10));
    }
    SelectObject16(&struct_1.field_0x1c, struct_1.field13_0x18);
    SelectObject16(&struct_1.field_0x1e, struct_1.field13_0x18);
    DeleteObject16(struct_1.field14_0x1a);
    obj = SelectPalette16(0x0, struct_1.field19_0x20, struct_1.field13_0x18);
    DeleteObject16(obj);
    DeleteDC16(struct_1.field13_0x18);
    in_struct_1.address_offset_field_0x0 = 0x3ab0;
    struct_1.address_offset_field_0x2 = 0x1008;
    in_struct_1.address_offset_field_0x0 = 0x389a;
    struct_1.address_offset_field_0x2 = 0x1008;
    return;
}

// WARNING: Unable to use type for symbol uVar2
pub unsafe fn win_ui_palette_op_1020_3e84(param_1: *mut StructD) {
    let mut hdc: HDC16;
    let mut obj: HPALETTE16;
    let mut struct_v1: *mut StructD;
    let mut uVar1: u16;
    let mut unaff_SS: u16;
    let mut uVar2: i32;

    uVar1 = (param_1 >> 0x10);
    struct_v1 = param_1;
    param_1.address_offset_field_0x0 = 0x408a;
    struct_v1.address_offset_field_0x2 = 0x1020;
    pass1_1010_1ea6(struct_v1.field12_0x14, (param_1 & 0xffff | uVar1 << 0x10));
    uVar2 = struct_v1.field12_0x14;
    hdc = *(uVar2 + 0x4c);
    SelectObject16(struct_v1.field13_0x18, hdc);
    SelectObject16(struct_v1.field14_0x1a, hdc);
    obj = SelectPalette16(0x0, &struct_v1.field_0x1c, hdc);
    DeleteObject16(obj);
    SetMapMode16(&struct_v1.field_0x1e, hdc);
    param_1.address_offset_field_0x0 = 0x3ab0;
    struct_v1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    struct_v1.address_offset_field_0x2 = 0x1008;
    return;
}

pub unsafe fn mixed_draw_op_1020_3fa0(mut param_1: u32) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
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
    let mut local_24: HDC16;
    let mut local_22: [u8; 0x20] = [0; 0x20];

    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(0x1050, local_22), (iVar4 + 0x4));
    uVar3 = (iVar4 + 0x14);
    local_24 = *(uVar3 + 0x4c);
    uVar3 = (iVar4 + 0x14);
    puStack40 = (uVar3 + 0x24);
    uVar5 = puStack40;
    ppcVar2 = (*puStack40 + 0x4);
    (**ppcVar2)(
        s_tile2_bmp_1050_1538,
        uVar5,
        (puStack40 >> 0x10),
        0x0,
        &local_24,
        &DAT_1050_1050,
    );
    uVar3 = (iVar4 + 0x14);
    iStack46 = (uVar3 + 0x44);
    uVar3 = (iVar4 + 0x14);
    uStack44 = (uVar3 + 0x40);
    uVar1 = (iVar4 + 0x14);
    pass1_1008_3e94(
        (uVar1 & 0xffff0000 | (uVar1 + 0x3a)),
        CONCAT22(0x1050, &local_32),
        CONCAT22(0x1050, &local_32 + 0x2),
    );
    uStack54 = uStack44;
    for iStack56 in 0..iStack46 {
        draw_rect_1020_40ce(uStack54, local_32, (local_32 >> 0x10), local_24, uVar5);
        uStack54 = uStack54 & 0xffff0000 | (uStack54 + 0x18);
    }
    EndPaint16(CONCAT13(0x10, CONCAT12(0x50, local_22)), (iVar4 + 0x4));
    return;
}

pub unsafe fn draw_text_1040_8d14(param_1: *mut Struct37, hdc_param_2: HDC16)

{
  let mut in_string: LPCSTR;
  let mut bVar1: u8;
  let mut IVar2: i16;
  let mut handle: HANDLE16;
   let mut struct_1: *mut Struct37;
  let mut uVar3: u16;
  let mut obj_handle_1: HGDIOBJ16;

  uVar3 = (param_1 >> 0x10);
  struct_1 = param_1;
  bVar1 = struct_1.field138_0x98 & 0xf0;
  if ((((bVar1 == 0x30) || (bVar1 == 0x10)) || (bVar1 == 0x40)) || (bVar1 == 0x20)) {
    struct_1.field145_0xa0 = 0xa;
    IVar2 = GetSystemMetrics16(SM_CXICON);
    struct_1.field144_0x9e = IVar2 + 0x28;
  }
  else {
    struct_1.field145_0xa0 = 0xa;
    struct_1.field144_0x9e = 0x14;
  }
  handle = GetProp16(s_hfont_1050_5e0f,(&struct_1.field1_0x4 + 0x2));
  if (handle != 0) {
    SelectObject16(handle,hdc_param_2);
  }
  in_string = struct_1.field133_0x90;
  obj_handle_1 = (in_string >> 0x10);
  DrawText16(0x410,(param_1 & 0xffff0000 | ZEXT24(&struct_1.field144_0x9e)),-0x1,in_string,hdc_param_2
            );
  if (obj_handle_1 != 0) {
    SelectObject16(obj_handle_1,hdc_param_2);
  }
  return;
}

pub unsafe fn unk_draw_op_1040_c226(struct_param_1: *mut astruct_772)

{
    let mut handle: HPEN16;
    let mut obj_handle_var3: HGDIOBJ16;
    let mut iVar3: *mut astruct_772;
    let mut uVar4: u16;
    let mut hdc: HDC16;
    let mut rect_var_32: RECT16;
    let mut iStack46: i16;
    let mut iStack44: i16;
    let mut uStack42: u16;
    let mut iStack40: i16;
    let mut hbrush_var38: HBRUSH16;
    let mut hdc16_var36: HDC16;
//   PAINTSTRUCT16 *paintstruct_22;
    let mut paintstruct_22: *mut PAINTSTRUCT16;
    let mut uVar1: u32;
    let mut uVar2: u32;

    uVar4 = (struct_param_1 >> 0x10);
    iVar3 = struct_param_1;
    hdc16_var36 = BeginPaint16(CONCAT22(0x1050, &paintstruct_22), iVar3.hwnd_0x4);
    hbrush_var38 = CreateSolidBrush16(0x8000);
    GetClientRect16(&rect_var_32, &DAT_1050_1050);
    uVar1 = iVar3.field5_0x6;
    iStack40 = (uVar1 + 0x1a);
    uVar2 = iVar3.field5_0x6;
    uStack42 = (uVar2 + 0x1c);
    rect_var_32.y += 0x2;
    rect_var_32.x = iStack40 - 0xa;
    iStack46 += -0x2;
    iStack44 += -0x2;
    FrameRect16(hbrush_var38, &rect_var_32, &DAT_1050_1050);
    DeleteObject16(hbrush_var38);
    hdc = hdc16_var36;
    handle = CreatePen16(0x8080, 0x2, 0x0);
    obj_handle_var3 = SelectObject16(handle, hdc);
    draw_line_1040_c302(struct_param_1, hdc16_var36);
    draw_op_1040_c38e(struct_param_1);
    obj_handle_var3 = SelectObject16(obj_handle_var3, hdc16_var36);
    DeleteObject16(obj_handle_var3);
    EndPaint16(CONCAT22(0x1050, &paintstruct_22), iVar3.hwnd_0x4);
    return;
}

pub unsafe fn fn_xyz() {
    let mut piVar2: *mut i16;
    let mut uVar3_01: u32;
    let mut uVar6: u16;
    let mut pHVar14: *mut HDC16;
    let mut iVar7: i16;
    let mut iVar11: i16;
    let mut handle: HPEN16;
    let mut hgdiobj16_00: HBRUSH16;
    let mut uVar7: u16;
    let mut obj: HPALETTE16;
    let mut puVar7: *mut u8;
    let mut hgdiobj16_var7: HGDIOBJ16;
    let mut in_EDX: u32;
    let mut paVar25: *mut Struct57;
    let mut uVar27: u16;
    let mut uVar26: u32;
    let mut struct742_var8: *mut astruct_742;
    let mut iVar9: *mut astruct_755;
    let mut iVar10: *mut astruct_756;
    let mut puVar11: *mut astruct_734;
    let mut iVar12: i16;
    let mut uVar12: u16;
    let mut uVar14: u16;
    let mut uVar13: u16;
    let mut uVar28: u32;
    let mut piVar16: *mut i16;
    let mut iVar29: i16;
    let mut local_38: [u8; 0x6] = [0; 0x6];
    let mut local_32: u16;
    let mut uStack48: u16;
    let mut uStack46: u32;
    let mut uStack42: u16;
    let mut puStack40: *mut u32;
    let mut local_24: HDC16;
    let mut paintstruct16_22: [u8; 0x20] = [0; 0x20];
    let mut uVar2_01: u32;
    let mut piVar1: *mut i16;
    let mut uVar15: u32;
    let mut uVar5: u32;
    let mut uVar8: u32;
    let mut uVar9: u32;
    let mut uVar10: u32;
    let mut iVar13: *mut astruct_758;
    let mut uVar11: u32;
    let mut uVar2: u32;
    let mut puVar4: *mut u32;
    let mut uVar4: u32;
    let mut uVar16: u8;
    let mut uVar17: u8;
    let mut uVar18: u16;
    let mut iVar16: *mut astruct_757;
    let mut uVar19: u16;
    let mut uVar20: u8;
    let mut uVar21: u8;
    let mut uVar22: u16;
    let mut uVar3: u32;
    let mut uVar2_00: u32;
    let mut uVar3_00: u32;
    let mut uVar23: u32;
    let mut uVar24: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar22 = (in_EDX >> 0x10);
    puVar11 = &stack0xfffe;
    uVar12 = (param_1 >> 0x10);
    struct742_var8 = param_1;
    local_24 = BeginPaint16(
        CONCAT22(0x1050, paintstruct16_22),
        struct742_var8.field4_0x4,
    );
    puStack40 = pass1_1010_4c2c(struct742_var8.field5_0x6);
    pHVar14 = &local_24;
    fn_ptr_1 = (*puStack40 + 0x8);
    (**fn_ptr_1)(
        0x1010,
        puStack40,
        (puStack40 >> 0x10),
        pHVar14,
        &DAT_1050_1050,
    );
    struct742_var8.field12_0x10 = pHVar14;
    uVar2 = struct742_var8.field5_0x6;
    uStack42 = (uVar2 + 0x30);
    uVar28 = struct742_var8.field5_0x6;
    uStack46 = *(uVar28 + 0x12);
    uStack48 = 0x14;
    local_32 = 0;
    uVar13 = 0x1008;
    pass1_1008_3e38(CONCAT22(0x1050, local_38));
    paVar25 = (uVar22 << 0x10);
    while (
        uVar27 = (paVar25 >> 0x10),
        &puVar11[-0x6].field_0x4 < (puVar11 - 0x4),
    ) {
        iVar9 = (&puVar11[-0x6].field_0x4 * 0x4);
        uVar8 = puVar11[-0x5].field6_0x6;
        uVar28 = pass1_1008_4772((iVar9 + uVar8));
        puVar7 = (uVar28 >> 0x10);
        paVar25 = CONCAT22(uVar27, puVar7);
        puVar11[-0x7].field_0x2 = uVar28;
        puVar11[-0x7].field_0x4 = puVar7;
        uVar3 = puVar11.field6_0x6;
        pass1_1018_642e(
            uVar3,
            (uVar3 >> 0x10),
            CONCAT13(0x10, CONCAT12(0x50, &puVar11[-0x5].field_0x2)),
            (uVar28 + 0x8),
        );
        uVar9 = &puVar11[-0x5].field_0x2;
        pass1_1008_3e76(
            CONCAT22(0x1050, &puVar11[-0x6].field6_0x6),
            0x0,
            uVar9,
            (uVar9 >> 0x10),
        );
        uVar23 = puVar11[-0x5].field6_0x6;
        pass1_1008_4480(
            &puVar11[-0x4].field_0x2,
            CONCAT22(0x1050, &puVar11[-0x6].field6_0x6),
            (iVar9 + uVar23),
        );
        iVar29 = &puVar11[-0x6].field_0x4;
        uVar10 = &puVar11[-0x5].field_0x2;
        uVar19 = uVar10;
        uVar20 = (uVar10 >> 0x10);
        uVar21 = (uVar10 >> 0x18);
        uVar11 = &puVar11[-0x7].field_0x2;
        uVar14 = (uVar11 >> 0x10);
        iVar10 = uVar11;
        iVar7 = iVar10.field4_0x4 + &puVar11[-0x5].field_0x4;
        iVar11 = iVar10.field7_0x8 + &puVar11[-0x5].field_0x2;
        uVar26 = puVar11.field6_0x6;
        uVar2_00 = (uVar26 + 0x6);
        iVar16 = uVar2_00;
        uVar18 = (uVar2_00 >> 0x10);
        uVar16 = '\x0b';
        uVar17 = '\x10';
        if (&iVar16.field_0x1a == 0) {
            uVar6 = iVar16.field47_0x30 << 0x3;
            mem_op_1000_179c(uVar6, paVar25);
            iVar16.field_0x1a = uVar6;
            iVar16.field28_0x1c = paVar25;
        }
        uVar3_00 = &iVar16.field_0x1a;
        iVar13 = (iVar29 * 0x8);
        (iVar13 + uVar3_00) = CONCAT11(uVar21, uVar20);
        uVar3_01 = &iVar16.field_0x1a;
        (iVar13 + uVar3_01 + 0x2) = uVar19;
        uVar3_01 = &iVar16.field_0x1a;
        (iVar13 + uVar3_01 + 0x4) = iVar7;
        uVar3_01 = &iVar16.field_0x1a;
        (iVar13 + uVar3_01 + 0x6) = iVar11;
        uVar13 = CONCAT11(uVar17, uVar16);
        uVar2_01 = &puVar11[-0x7].field_0x2;
        piVar2 = &puVar11[-0x5].field_0x4;
        *piVar2 = *piVar2 + (-(&puVar11[-0x6].field_0x4 == 0) & 0x5) + 0x14 + (uVar2_01 + 0x4);
        piVar2 = &puVar11[-0x6].field_0x4;
        *piVar2 = *piVar2 + 1;
    }
    puVar4 = &puVar11[-0x4].field_0x2;
    fn_ptr_1 = (*puVar4 + 0x4);
    (**fn_ptr_1)(
        uVar13,
        puVar4,
        (puVar4 >> 0x10),
        0x0,
        0x0,
        puVar11 - 0x22,
        &DAT_1050_1050,
    );
    handle = CreatePen16(0x1000025, 0x1, 0x0);
    &puVar11[-0x6].field_0x2 = handle;
    hgdiobj16_var7 = SelectObject16(handle, *&puVar11[-0x4].field6_0x6);
    (puVar11 - 0x6) = hgdiobj16_var7;
    hgdiobj16_00 = CreateSolidBrush16(0x1000025);
    (&puVar11[-0x7].field6_0x6 + 0x2) = hgdiobj16_00;
    hgdiobj16_var7 = SelectObject16(hgdiobj16_00, *&puVar11[-0x4].field6_0x6);
    puVar11[-0x7].field6_0x6 = hgdiobj16_var7;
    draw_line_1018_6444(puVar11.field6_0x6, *&puVar11[-0x4].field6_0x6);
    uVar4 = puVar11.field6_0x6;
    piVar16 = pass1_1010_4dc8((uVar4 + 0x6));
    uVar26 = piVar16 >> 0x10;
    uVar24 = piVar16 & 0xffff;
    draw_op_1018_6544(
        puVar11.field6_0x6,
        (piVar16 & 0xff000000 | CONCAT12((piVar16 >> 0x10), uVar24)),
    );
    pass1_1018_6630((uVar26 & 0xffff | uVar24 << 0x10), puVar11.field6_0x6);
    uVar5 = puVar11.field6_0x6;
    obj = SelectPalette16(0x0, (uVar5 + 0x10), *&puVar11[-0x4].field6_0x6);
    DeleteObject16(obj);
    hgdiobj16_var7 = SelectObject16((puVar11 - 0x6), *&puVar11[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var7);
    hgdiobj16_var7 = SelectObject16(&puVar11[-0x7].field6_0x6, *&puVar11[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var7);
    uVar15 = puVar11.field6_0x6;
    EndPaint16(
        CONCAT22(0x1050, (&puVar11[-0x4].field6_0x6 + 0x2)),
        (uVar15 + 0x4),
    );
    return;
}

pub unsafe fn unk_draw_op_1020_2020(param_1: *mut astruct_743) {
    let mut piVar1: *mut i16;
    let mut uVar3: u32;
    let mut uVar6: u32;
    let mut iVar8: i16;
    let mut pHVar9: *mut HDC16;
    let mut iVar7: i16;
    let mut iVar9: i16;
    let mut hgdiobj16_var8: HGDIOBJ16;
    let mut hgdiobj16_var9: HGDIOBJ16;
    let mut hgdiobj16_var10: HGDIOBJ16;
    let mut HVar10: HBRUSH16;
    let mut HVar11: HPEN16;
    let mut uVar11: u16;
    let mut obj: HPALETTE16;
    let mut uVar13: u16;
    let mut extraout_DX: u16;
    let mut in_EDX: u32;
    let mut paVar14: *mut Struct57;
    let mut iVar10: *mut astruct_743;
    let mut iVar11: *mut astruct_744;
    let mut iVar12: *mut astruct_745;
    let mut puVar14: *mut astruct_735;
    let mut iVar13: i16;
    let mut uVar12: u16;
    let mut uVar15: u16;
    let mut uVar14: u16;
    let mut puVar15: *mut u16;
    let mut uVar16: u32;
    let mut piVar17: *mut i16;
    let mut local_38: [u8; 0x6] = [0; 0x6];
    let mut local_32: u16;
    let mut uStack48: u16;
    let mut uStack46: u32;
    let mut uStack42: u16;
    let mut puStack40: *mut u32;
    let mut hdc_24: HDC16;
    let mut paintstruct16_22: [u8; 0x20] = [0; 0x20];
    let mut uVar2: u32;
    let mut puVar4: *mut u32;
    let mut uVar18: u8;
    let mut uVar19: u8;
    let mut uVar20: u8;
    let mut uVar21: u8;
    let mut uVar22: u16;
    let mut uVar23: u16;
    let mut uVar24: u16;
    let mut iVar14: *mut astruct_746;
    let mut iVar15: *mut astruct_748;
    let mut uVar1: u32;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let mut iVar16: i16;
    let mut fn_ptr_1: *mut *mut code;

    uVar24 = (in_EDX >> 0x10);
    puVar14 = &stack0xfffe;
    uVar12 = (param_1 >> 0x10);
    iVar10 = param_1;
    hdc_24 = BeginPaint16(CONCAT22(0x1050, paintstruct16_22), iVar10.field4_0x4);
    puStack40 = pass1_1010_4c2c(iVar10.field5_0x6);
    pHVar9 = &hdc_24;
    fn_ptr_1 = (*puStack40 + 0x8);
    (**fn_ptr_1)(
        0x1010,
        puStack40,
        (puStack40 >> 0x10),
        pHVar9,
        &DAT_1050_1050,
    );
    iVar10.field12_0x10 = pHVar9;
    uVar2 = iVar10.field5_0x6;
    uStack42 = (uVar2 + 0x30);
    uVar16 = iVar10.field5_0x6;
    uStack46 = *(uVar16 + 0x12);
    uStack48 = 0x14;
    local_32 = 0;
    uVar14 = 0x1008;
    puVar15 = pass1_1008_3e38(CONCAT22(0x1050, local_38));
    paVar14 = CONCAT22(uVar24, (puVar15 >> 0x10));
    while (&puVar14[-0x6].field_0x4 < (puVar14 - 0x4)) {
        iVar11 = (&puVar14[-0x6].field_0x4 * 0x4);
        uVar6 = puVar14[-0x5].field6_0x6;
        uVar16 = pass1_1008_4772((iVar11 + uVar6));
        paVar14 = (paVar14 & 0xffff0000 | uVar16 >> 0x10);
        puVar14[-0x7].field_0x2 = uVar16;
        uVar13 = (uVar16 >> 0x10);
        puVar14[-0x7].field_0x4 = uVar13;
        uVar6 = puVar14.field6_0x6;
        pass1_1020_2286(
            uVar6,
            (uVar6 >> 0x10),
            CONCAT13(0x10, CONCAT12(0x50, &puVar14[-0x5].field_0x2)),
            (uVar16 + 0x8),
        );
        uVar3 = &puVar14[-0x5].field_0x2;
        pass1_1008_3e76(
            CONCAT22(0x1050, &puVar14[-0x6].field6_0x6),
            0x0,
            uVar3,
            (uVar3 >> 0x10),
        );
        uVar6 = puVar14[-0x5].field6_0x6;
        pass1_1008_4480(
            &puVar14[-0x4].field_0x2,
            CONCAT22(0x1050, &puVar14[-0x6].field6_0x6),
            (iVar11 + uVar6),
        );
        iVar16 = &puVar14[-0x6].field_0x4;
        uVar3 = &puVar14[-0x5].field_0x2;
        uVar23 = uVar3;
        uVar20 = (uVar3 >> 0x10);
        uVar21 = (uVar3 >> 0x18);
        uVar3 = &puVar14[-0x7].field_0x2;
        uVar15 = (uVar3 >> 0x10);
        iVar12 = uVar3;
        iVar7 = iVar12.field4_0x4 + &puVar14[-0x5].field_0x4;
        iVar9 = iVar12.field7_0x8 + &puVar14[-0x5].field_0x2;
        uVar6 = puVar14.field6_0x6;
        uVar3 = (uVar6 + 0x6);
        iVar14 = uVar3;
        uVar22 = (uVar3 >> 0x10);
        uVar18 = '\x0b';
        uVar19 = '\x10';
        if (&iVar14.field_0x1a == 0) {
            iVar8 = iVar14.field47_0x30 << 0x3;
            mem_op_1000_179c(iVar8, paVar14);
            iVar14.field_0x1a = iVar8;
            iVar14.field28_0x1c = paVar14;
        }
        uVar3 = &iVar14.field_0x1a;
        iVar15 = (iVar16 * 0x8);
        (iVar15 + uVar3) = CONCAT11(uVar21, uVar20);
        uVar3 = &iVar14.field_0x1a;
        (iVar15 + uVar3 + 0x2) = uVar23;
        uVar3 = &iVar14.field_0x1a;
        (iVar15 + uVar3 + 0x4) = iVar7;
        uVar3 = &iVar14.field_0x1a;
        (iVar15 + uVar3 + 0x6) = iVar9;
        uVar14 = CONCAT11(uVar19, uVar18);
        uVar3 = &puVar14[-0x7].field_0x2;
        piVar1 = &puVar14[-0x5].field_0x4;
        *piVar1 = *piVar1 + (-(&puVar14[-0x6].field_0x4 == 0) & 0x5) + 0x14 + (uVar3 + 0x4);
        piVar1 = &puVar14[-0x6].field_0x4;
        *piVar1 = *piVar1 + 1;
    }
    puVar4 = &puVar14[-0x4].field_0x2;
    fn_ptr_1 = (*puVar4 + 0x4);
    (**fn_ptr_1)(
        uVar14,
        puVar4,
        (puVar4 >> 0x10),
        0x0,
        0x0,
        puVar14 - 0x22,
        &DAT_1050_1050,
    );
    extraout_DX = paVar14;
    hgdiobj16_var8 = CreatePen16(0x1000025, 0x1, 0x0);
    puVar14[-0x6].field_0x2 = hgdiobj16_var8;
    hgdiobj16_var10 = SelectObject16(hgdiobj16_var8, *&puVar14[-0x4].field6_0x6);
    (puVar14 - 0x6) = hgdiobj16_var10;
    hgdiobj16_var9 = CreateSolidBrush16(0x1000025);
    (&puVar14[-0x7].field6_0x6 + 0x2) = hgdiobj16_var9;
    hgdiobj16_var10 = SelectObject16(hgdiobj16_var9, *&puVar14[-0x4].field6_0x6);
    puVar14[-0x7].field6_0x6 = hgdiobj16_var10;
    draw_line_1020_229c(puVar14.field6_0x6, &puVar14[-0x4].field6_0x6);
    uVar1 = puVar14.field6_0x6;
    pass1_1010_4df0(extraout_DX, (uVar1 + 0x6));
    if (hgdiobj16_var10 == 0) {
        hgdiobj16_var10 = SelectObject16((puVar14 - 0x6), *&puVar14[-0x4].field6_0x6);
        DeleteObject16(hgdiobj16_var10);
        hgdiobj16_var10 = SelectObject16(&puVar14[-0x7].field6_0x6, *&puVar14[-0x4].field6_0x6);
        DeleteObject16(hgdiobj16_var10);
        HVar10 = CreateSolidBrush16(0xff);
        (&puVar14[-0x7].field6_0x6 + 0x2) = HVar10;
        HVar11 = CreatePen16(0xff, 0x1, 0x0);
        puVar14[-0x6].field_0x2 = HVar11;
        SelectObject16(
            (&puVar14[-0x7].field6_0x6 + 0x2),
            *&puVar14[-0x4].field6_0x6,
        );
        SelectObject16(&puVar14[-0x6].field_0x2, *&puVar14[-0x4].field6_0x6);
    }
    uVar5 = puVar14.field6_0x6;
    piVar17 = pass1_1010_4dc8((uVar5 + 0x6));
    uVar13 = (piVar17 >> 0x10);
    uVar11 = piVar17;
    pass1_1020_239c(puVar14.field6_0x6, piVar17);
    uVar6 = puVar14.field6_0x6;
    uVar4 = (uVar6 + 0x6);
    if ((uVar4 + 0x2c) != 0) {
        pass1_1020_2488(uVar11, uVar13, puVar14.field6_0x6);
    }
    uVar6 = puVar14.field6_0x6;
    obj = SelectPalette16(0x0, (uVar6 + 0x10), *&puVar14[-0x4].field6_0x6);
    DeleteObject16(obj);
    hgdiobj16_var10 = SelectObject16((puVar14 - 0x6), *&puVar14[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var10);
    hgdiobj16_var10 = SelectObject16(&puVar14[-0x7].field6_0x6, *&puVar14[-0x4].field6_0x6);
    DeleteObject16(hgdiobj16_var10);
    uVar6 = puVar14.field6_0x6;
    EndPaint16(
        CONCAT22(0x1050, (&puVar14[-0x4].field6_0x6 + 0x2)),
        (uVar6 + 0x4),
    );
    return;
}

pub unsafe fn FUN_1010_2a32(
    buffer_param_2: *mut u8,
    mut param_2: u32,
    hfile_param: *mut HFILE16,
    mut param_4: u16,
) -> u16 {
    let mut piVar1: *mut i16;
    let mut pcVar2: *mut c_char;
    let mut pbVar3: *mut u8;
    let mut bVar4: u8;
    let mut puVar5: *mut u32;
    let mut puVar6: *mut u32;
    let mut ppcVar7: *mut *mut code;
    let mut pcVar8: *mut code;
    let mut uVar9: u32;
    let mut uVar10: u16;
    let mut HVar11: HDC16;
    let mut HVar12: HPALETTE16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut BVar15: bool;
    let mut iVar16: i16;
    let mut bVar17: u8;
    let mut puVar18: *mut u8;
    let mut puVar19: *mut u8;
    let mut uVar20: u16;
    let mut uVar21: u16;
    let mut in_EDX: u32;
    let mut in_BX: *mut u32;
    let mut unaff_BP: u16;
    let mut unaff_SI: i16;
    let mut iVar23: i16;
    let mut iVar24: i16;
    let mut unaff_ES: u16;
    let mut uVar25: u16;
    let mut bVar26: u8;
    let mut bVar27: bool;
    let mut init_data: *mut DEVMODEA;
    let mut in_stack_00000000: i16;
    let mut in_stack_00000002: u16;
    let mut uStack54: u16;
    let mut puStack46: *mut u16;
    let mut uStack42: u16;
    let mut read_buffer_38: *mut u8;
    let mut read_buffer_22: *mut u8;
    let mut HStack30: HGDIOBJ16;
    let mut HStack28: HGDIOBJ16;
    let mut in_stack_0000ffec: u8;
    let mut uVar28: u8;
    let mut in_stack_0000ffed: u8;
    let mut uVar29: u8;
    let mut uVar30: u8;
    let mut uVar31: u8;
    let mut uVar32: u8;
    let mut uVar33: u8;
    let mut uVar34: u8;
    let mut uVar35: u8;
    let mut uVar36: u8;
    let mut uVar37: u8;
    let mut uVar38: u8;
    let mut paVar22: *mut Struct57;

    uVar28 = unaff_BP;
    uVar29 = (unaff_BP >> 0x8);
    bVar26 = 0;
    uVar38 = 0;
    //   if ((param_2 + 0xec76 & 0x3) != 0) goto LAB_1010_2ad8;
    uVar10 = param_2 + 0xec76 >> 0x1;
    //   if (0x1c < uVar10) goto LAB_1010_2ad8;
    uVar14 = in_EDX;
    //   switch(uVar10)
    match uVar10 {
        //   _ =>
        _ => {}
        // TODO: goto switchD_1010_2ab5_caseD_0;
        0x1 | 0x3 | 0xb => {
            (uVar10 + 0xa) = in_BX;
        }
        0x9 | 0xf | 0x15 | 0x1b => {
            (uVar10 + 0xa) = in_BX;
            (uVar10 + 0x10) = in_BX;
            (uVar10 + 0xc) = in_BX;
            return uVar10;
        }
        0x5 => {
            BVar15 = write_to_file_1008_7e1c(
                param_2,
                ZEXT24(in_BX),
                CONCAT13(
                    (in_stack_00000000 >> 0x8),
                    CONCAT12(in_stack_00000000, unaff_BP),
                ),
                CONCAT11(in_stack_0000ffed, in_stack_0000ffec),
            );
            if (BVar15 != 0) {
                return 0x1;
            }
            u16_1050_0310 = 0x6d0;
            return 0x0;
        }
        0x6 => {
            bVar26 = 0;
        }
        // TODO: goto LAB_1010_2ad8;
        0x7 => {
            ppcVar7 = *in_BX;
            (**ppcVar7)();
            iVar16 = param_2 + 0x105;
            puVar18 = in_EDX;
            pass1_1010_8170(puVar18, _u16_1050_14cc, iVar16);
            iVar23 = param_2 * 0x4;
            (buffer_param_2 + iVar23 + 0x26) = iVar16;
            (buffer_param_2 + iVar23 + 0x28) = puVar18;
            uVar36 = 0x50;
            uVar37 = 0x10;
            uVar34 = 0x80;
            uVar35 = 0x13;
            uVar30 = 0;
            uVar31 = 0;
            uVar32 = 0;
            uVar33 = 0;
            uVar28 = 0;
            uVar29 = 0;
            init_data = pass1_1008_4772((buffer_param_2 + 0x26 + iVar23));
            uVar25 = (init_data >> 0x10);
            HVar11 = CreateDC16(
                init_data,
                CONCAT13(uVar29, CONCAT12(uVar28, uVar25)),
                CONCAT13(uVar33, CONCAT12(uVar32, CONCAT11(uVar31, uVar30))),
                CONCAT13(uVar37, CONCAT12(uVar36, CONCAT11(uVar35, uVar34))),
            );
            uVar28 = HVar11;
            uVar29 = (HVar11 >> 0x8);
            HVar12 = palette_op_1008_4e08(
                &stack0xffec,
                uVar25,
                (_PTR_LOOP_1050_4230 + 0xe),
                CONCAT13(0x10, CONCAT12(0x50, &stack0xffec)),
            );
            HStack28 = SelectObject16(CONCAT11(uVar38, bVar26), CONCAT11(uVar29, uVar28));
            HStack30 = SelectObject16(CONCAT11(uVar29, uVar28), CONCAT11(uVar29, uVar28));
            read_buffer_22 = 0;
            piVar1 = buffer_param_22 + 0x74;
            // for (read_buffer_22 = 0; piVar1 = (buffer_param_2 + 0x74),
            // *piVar1 != read_buffer_22 && read_buffer_22 <= *piVar1; read_buffer_22 += 1)
            while *piVar1 != read_buffer_22 && read_buffer_22 <= *piVar1 {
                iVar16 = (read_buffer_22 * 0x10 + param_2) * 0x8;
                uVar25 = (buffer_param_2 + 0x72);
                uVar13 = pass1_1000_484c(
                    CONCAT13(0x10, CONCAT12(0x50, &stack0xfff2)),
                    CONCAT13(
                        (uVar25 >> 0x8),
                        CONCAT12(uVar25, iVar16 + (buffer_param_2 + 0x70)),
                    ),
                    0x8,
                );
                if (uVar13 != 0) {
                    uVar9 = (buffer_param_2 + 0x70);
                    uVar25 = (uVar9 >> 0x10);
                    iVar23 = uVar9;
                    iVar24 = iVar16 + iVar23;
                    Rectangle16(
                        (iVar24 + 0x6),
                        (iVar24 + 0x4),
                        (iVar24 + 0x2),
                        (iVar23 + iVar16),
                        CONCAT11(uVar29, uVar28),
                    );
                }
                read_buffer_22 += 1;
            }
            HVar12 = SelectPalette16(0x0, HVar12, CONCAT11(uVar29, uVar28));
            DeleteObject16(HVar12);
            SelectObject16(HStack28, CONCAT11(uVar29, uVar28));
            SelectObject16(HStack30, CONCAT11(uVar29, uVar28));
            uVar38 = 0x38;
            uVar30 = 0x15;
            DeleteDC16(CONCAT11(uVar29, uVar28));
            BVar15 = DeleteObject16(CONCAT11(uVar30, uVar38));
            return BVar15;
        }
        0x8 => {
            bVar26 = 0x3;
        }
        // TODO: goto LAB_1010_2ad8;
        0xd => {
            pbVar3 = (uVar10 + unaff_SI);
            bVar26 = *pbVar3;
            bVar4 = *pbVar3 + in_EDX;
            *pbVar3 = bVar4 + (uVar10 < 0x1c);
            puVar5 = (CARRY1(bVar26, in_EDX) || CARRY1(bVar4, uVar10 < 0x1c));
            puVar6 = in_BX -0x404;
            bVar26 = in_BX < 0x1010 || puVar6 < puVar5;
            uVar21 = puVar6 - puVar5;
            pcVar8 = swi(0x4);
            if (SBORROW2(in_BX, 0x1010) != SBORROW2(puVar6, puVar5)) {
                (*pcVar8)();
            }
            puVar19 = in_EDX;
            bVar27 = uVar21 < 0x1010 || uVar21 + 0xeff0 < bVar26;
            pbVar3 = (uVar10 + unaff_SI);
            bVar26 = *pbVar3;
            bVar17 = in_EDX;
            bVar4 = *pbVar3;
            *pbVar3 = bVar4 + bVar17 + bVar27;
            pcVar2 = (uVar10 + unaff_SI);
            *pcVar2 = *pcVar2 + bVar17 + (CARRY1(bVar26, bVar17) || CARRY1(bVar4 + bVar17, bVar27));
            struct_op_1018_4cda(
                CONCAT13(
                    in_stack_00000000,
                    CONCAT12(uVar29, CONCAT11(uVar28, uVar38)),
                ),
                CONCAT11(in_stack_00000002, (in_stack_00000000 >> 0x8)),
            );
            iVar16 = CONCAT11(uVar28, uVar38);
            uVar30 = in_stack_00000000;
            piVar1 = CONCAT13(uVar30, CONCAT12(uVar29, iVar16));
            *piVar1 = s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 1;
            (iVar16 + 0x2) = 0x1010;
            pass1_1018_4dce(puVar19, CONCAT13(uVar30, CONCAT12(uVar29, iVar16)), 0x1b3);
            _PTR_LOOP_1050_4230 = CONCAT13(uVar30, CONCAT12(uVar29, CONCAT11(uVar28, uVar38)));
            return CONCAT11(uVar28, uVar38);
        }
        0xe => {
            (buffer_param_2 + 0x20) = param_2;
        }

        0x11 => {
            // loop {
            // // WARNING: Do nothing block with infinite loop
            // }
        }
        0x12 => {
            PTR_LOOP_1050_10c6 = (0x0 < param_2);
            PTR_LOOP_1050_1142 = (0x2 < param_2);
        }
        0x13 => {
            iVar16 = buffer_param_2 * 0x8 + in_stack_00000000;
            if ((((iVar16 + 0x22) != 0) || ((iVar16 + 0x24) != 0))
                || ((iVar16 + 0x26) != 0x0 || ((iVar16 + 0x28) != 0)))
            {
                HStack28 = 0x1010;
                HStack30 = 0x627c;
                sys_1000_3f9c(
                    *(in_stack_00000000 + 0xe),
                    s__d__d__d__d_1050_14ae,
                    (buffer_param_2 * 0x8 + in_stack_00000000 + 0x22),
                );
                HStack28 = 0x1000;
                HStack30 = 0x62a0;
                in_BX = WritePrivateProfileString16(
                    *(in_stack_00000000 + 0xa),
                    *(in_stack_00000000 + 0xe),
                    *(buffer_param_2 * 0x4 + 0x1446),
                    s_windows_1050_13b8,
                );
            }
            return in_BX;
        }
        0x14 => {
            (buffer_param_2 + 0x24) = param_2;
        }

        0x17 => {
            uVar21 = uVar14 - 0x1;
            paVar22 = (in_EDX & 0xffff0000 | uVar21);
            pbVar3 = (uVar10 + unaff_SI);
            *pbVar3 = *pbVar3 | uVar21;
            (buffer_param_2 + 0x12) = in_BX;
            (buffer_param_2 + 0x14) = uVar21;
            uStack42 = 0;
            loop {
                if (uStack54 <= uStack42) {
                    uVar38 = (buffer_param_2 >> 0x10);
                    BVar15 = read_file_1008_7dee(
                        param_2,
                        ((buffer_param_2 & 0xff00) << 0x10
                            | CONCAT12(uVar38, buffer_param_2 + 0x1a)),
                        0x2,
                    );
                    if (((BVar15 != 0)
                        && (
                            BVar15 = read_file_1008_7dee(
                                param_2,
                                ((buffer_param_2 & 0xff00) << 0x10
                                    | CONCAT12(uVar38, buffer_param_2 + 0x1c)),
                                0x2,
                            ),
                            BVar15 != 0,
                        ))
                        && (
                            BVar15 = read_file_1008_7dee(
                                param_2,
                                ((buffer_param_2 & 0xff00) << 0x10
                                    | CONCAT12(uVar38, buffer_param_2 + 0x1e)),
                                0x2,
                            ),
                            BVar15 != 0,
                        ))
                    {
                        return 0x1;
                    }
                    u16_1050_0310 = 0x6d2;
                    return 0x0;
                }
                uVar10 = uStack54;
                mem_op_1000_179c(0x8, paVar22);
                uVar21 = paVar22;
                puStack46 = CONCAT22(uVar21, uVar10);
                paVar22 = (paVar22 & 0xffff0000 | (uVar21 | uVar10));
                if ((uVar21 | uVar10) == 0) {
                    read_buffer_38 = null_mut();
                } else {
                    *puStack46 = 0x389a;
                    (uVar10 + 0x2) = 0x1008;
                    *puStack46 = 0xa1c4;
                    (uVar10 + 0x2) = 0x1010;
                    read_buffer_38 = puStack46;
                }
                BVar15 = read_file_1008_7dee(
                    param_2,
                    CONCAT13(0x10, CONCAT12(0x50, &read_buffer_22)),
                    0x2,
                );
                if (BVar15 == 0)
                    || (
                        BVar15 = read_file_1008_7dee(
                            param_2,
                            (read_buffer_38 & 0xff000000
                                | CONCAT12((read_buffer_38 >> 0x10), read_buffer_38 + 0x6)),
                            0x2,
                        ),
                        BVar15 == 0,
                    )
                {}
                iVar16 = switch_1008_73ea(param_2, param_2, read_buffer_22);
                (read_buffer_38 + 0x4) = iVar16;
                ppcVar7 = ((buffer_param_2 + 0x12) + 0x4);
                (**ppcVar7)();
                uStack42 += 0x1;
            }
            if read_buffer_38.is_null() {
                u16_1050_0310 = 0x6d2;
                return 0x0;
            }
            ppcVar7 = read_buffer_38;
            (**ppcVar7)();
            u16_1050_0310 = 0x6d2;
            return 0x0;
        }
        0x18 => {
            bVar26 = 0x2;
        }
        // TODO: goto LAB_1010_2ad8;
        0x19 => {
            uVar14 = pass1_1010_6ca2(uVar14, buffer_param_2, 0x8);
            uVar20 = in_EDX;
            if uVar14 != 0 {
                pass1_1010_715c(uVar14, uVar20, CONCAT22(0x1a, buffer_param_2), 0x1a);
                buffer_param_2 = 0x1a001a;
            }
            if param_2 == 0x2c {
                pass1_1010_715c(uVar14, uVar20, CONCAT22(0x1d, buffer_param_2), 0x1d);
            }
            uVar14 = pass1_1010_6ca2(uVar20, 0x5a, 0x2);
            if uVar14 != 0 {
                pass1_1010_715c(uVar14, uVar20, 0x1c005a, 0x1c);
            }
            return 0x1;
        }
        0x1a => {
            (buffer_param_2 + 0x26) = param_2;
        }
    };
    bVar26 = 0x1; //
                  // LAB_1010_2ad8:
    if (bVar26 == 1) || (bVar26 == 0x2) {
        if bVar26 == 1 {
            param_2 = ((buffer_param_2 + 0x20)
                + (buffer_param_2 + 0x22)
                + (buffer_param_2 + 0x24)
                + (buffer_param_2 + 0x26));
        }
        if param_2.is_null() == false {
            uVar10 = param_2 / 0x2 + 1;
            if 0x5 < uVar10 {
                uVar10 = 0x5;
            }
            param_2 = uVar10;
            if (bVar26 == 1) && (PTR_LOOP_1050_10c6.is_null() == false) {
                if 0x4 < uVar10 {
                    uVar10 = 0x4;
                }
                param_2 = uVar10;
            }
        }
    }
    (bVar26 * 0x7c + 0xed6) = param_2;
    in_BX = param_2;
    pass1_1010_1f62(buffer_param_2, 0xc);
    // switchD_1010_2ab5_caseD_0:
    return in_BX;
}

pub unsafe fn create_dc_1018_4e04(
    in_string_6: u16,
    in_string_5: u16,
    param_3: *mut astruct_8,
    mut param_4: u16,
    mut param_5: i16,
    mut param_6: i16,
) {
    let mut hvar1: HDC16;
    let mut p_hvar2: *mut HDC16;
    let mut i_var4: *mut astruct_8;
    let mut u_var3: u16;
    let mut devmodea_init_data: *mut DEVMODEA;
    let mut i_stack16: i16;
    let mut fn_ptr_1: *mut *mut code;

    u_var3 = (param_3 >> 0x10);
    i_var4 = param_3;
    fn_ptr_1 = (param_3 + 0x14);
    (**fn_ptr_1)();
    devmodea_init_data = pass1_1008_4772(i_var4.field10_0xa);
    pass1_1008_43cc(i_var4.field10_0xa);
    hvar1 = CreateDC16(devmodea_init_data, NULL, NULL, s_dib_1050_4234);
    i_var4.field15_0x12 = hvar1;
    p_hvar2 = &i_var4.field15_0x12;
    fn_ptr_1 = (i_var4.field10_0xa + 0x8);
    (**fn_ptr_1)();
    i_var4.field22_0x1a = p_hvar2;
    if ((DAT_1050_422e != 0) && (0x280 < param_6)) {
        // for (iStack16 = 0; iStack16 < DAT_1050_4216 + 1; iStack16 += 1)
        for iStack16 in 0..DAT_1050_4216 {
            (&u16_1050_4172 + iStack16 * 0x2) = (((&u16_1050_4172 + i_stack16 * 0x2) * (param_6 + 1)) / 0x280);
        }
        // for (iStack16 = 0; iStack16 < DAT_1050_422c + 1; iStack16 += 1)
        for iStack16 in 0..DAT_1050_422c {
            (&u16_1050_419a + iStack16 * 0x2) = (((&u16_1050_419a + i_stack16 * 0x2) * (param_5 + 1)) / 0x1e0);
        }
    }
    DAT_1050_422e = 0;
    return;
}

// l
pub unsafe fn fill_rect_1008_39ac(in_win_handle_1: *mut astruct_930, mut param_2: u16) {
    let mut hbrush: HBRUSH16;
    let mut local_paint_struct: [u8; 0x20] = [0; 0x20];

    BeginPaint16(
        CONCAT22(0x1050, local_paint_struct),
        in_win_handle_1.field4_0x4,
    );
    hbrush = CreateSolidBrush16(0x210070b);
    GetClientRect16(&stack0xffd2, &DAT_1050_1050);
    FillRect16(hbrush, &stack0xffd2, &DAT_1050_1050);
    EndPaint16(
        CONCAT22(0x1050, local_paint_struct),
        in_win_handle_1.field4_0x4,
    );
    DeleteObject16(hbrush);
    return;
}


// WARNING: Unable to use type for symbol uVar2
pub unsafe fn fill_rect_1020_065e(astruct754_param_1: *mut astruct_754) {
    let mut uVar1: u32;
    let mut struct754_var1: *mut astruct_754;
    let mut uVar4: u16;
    let mut rect_1: RECT16;
    let mut iStack48: i16;
    let mut iStack46: i16;
    let mut brush_handle_1: HBRUSH16;
    let mut palette_handle_var42: *mut HDC16;
    let mut puStack40: *mut u32;
    let mut hdc_var_24: HDC16;
    let mut paintstruct_22: [u8; 0x20] = [0; 0x20];
    let mut uVar3: u16;
    let mut uVar2: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar4 = (astruct754_param_1 >> 0x10);
    struct754_var1 = astruct754_param_1;
    hdc_var_24 = BeginPaint16(CONCAT22(0x1050, paintstruct_22), &struct754_var1.field_0x4);
    if (0x280 < struct754_var1.field7_0xa) {
        brush_handle_1 = CreateSolidBrush16(0x210070b);
        rect_1.x = 0;
        rect_1.y = 0;
        iStack48 = struct754_var1.field7_0xa - 0x1;
        iStack46 = struct754_var1.field8_0xc - 0x1;
        FillRect16(brush_handle_1, &rect_1, &DAT_1050_1050);
        DeleteObject16(brush_handle_1);
    }
    uVar2 = struct754_var1.field6_0x6;
    puStack40 = (uVar2 + 0xe);
    palette_handle_var42 = &hdc_var_24;
    uVar3 = puStack40;
    uVar1 = *puStack40;
    fn_ptr_1 = (uVar1 + 0x8);
    (**fn_ptr_1)(
        s_tile2_bmp_1050_1538,
        uVar3,
        (puStack40 >> 0x10),
        palette_handle_var42,
        &DAT_1050_1050,
    );
    fn_ptr_1 = (uVar1 + 0x4);
    (**fn_ptr_1)(
        s_tile2_bmp_1050_1538,
        puStack40,
        (puStack40 >> 0x10),
        struct754_var1.field10_0x10,
        struct754_var1.field9_0xe,
        &hdc_var_24,
        &DAT_1050_1050,
        uVar3,
    );
    palette_handle_var42 = SelectPalette16(0x0, palette_handle_var42, hdc_var_24);
    DeleteObject16(palette_handle_var42);
    EndPaint16(CONCAT22(0x1050, paintstruct_22), &struct754_var1.field_0x4);
    return;
}

pub unsafe fn draw_op_1040_82ee(astruct14_param_1: *mut astruct_14)

{
   let mut struct_1: *mut astruct_14;
  let mut struct_1_hi: HDC16;
  let mut local_1a: u32;
  let mut uStack22: u32;
  let mut rect_var_12: RECT16;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut brush_handle_1: HBRUSH16;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut iStack4: i16;

  struct_1_hi = (astruct14_param_1 >> 0x10);
  struct_1 = astruct14_param_1;
  iStack6 = (struct_1.field118_0x80 - struct_1.field116_0x7c) -0x2;
  iStack8 = (-(struct_1.field95_0x60 == 0) & 0x1e) + 0x25;
  iStack4 = iStack6;
  brush_handle_1 = CreateSolidBrush16(CONCAT22(0x100,iStack8));
  if (&struct_1[0x1].field_0x4 == 0) {
    local_1a = CONCAT22(struct_1.field98_0x66 + 0x2,struct_1.field97_0x64 + 2);
    uStack22 = CONCAT22(iStack4,iStack6);
    (struct_1 + 1) = local_1a;
    struct_1[0x1].field_0x4 = uStack22;
  }
  rect_var_12.x = (struct_1 + 1) + 2;
  rect_var_12.y =
       (struct_1[0x1].hwnd16_field6_0x6 - &struct_1[0x1].field_0x2) / 0x2 + &struct_1[0x1].field_0x2 +
       -0x2;
  iStack14 = &struct_1[0x1].field_0x4 -0x2;
  iStack12 = (struct_1[0x1].hwnd16_field6_0x6 - &struct_1[0x1].field_0x2) / 0x2 +
             &struct_1[0x1].field_0x2 + 2;
  FrameRect16(brush_handle_1,(struct_1 + 1),struct_1_hi);
  FrameRect16(brush_handle_1,&rect_var_12,&DAT_1050_1050);
  DeleteObject16(brush_handle_1);
  struct_1.field115_0x7a = &struct_1[0x1].field_0x4 + 2;
  return;
}


pub unsafe fn fill_rect_1008_62c0(param_1: *mut astruct_838, mut param_2: u16) {
    //   RECT16 rect_2e [0x2];
    let mut rect_2e: [RECt16; 2] = [RECT16::default(); 2];
    let mut hbrush_var38: HBRUSH16;
    let mut hbrush_var36: HDC16;
    let mut paintstruct_22: [u8; 0x20] = [0; 0x20];

    hbrush_var36 = BeginPaint16(CONCAT22(0x1050, paintstruct_22), param_1.field8_0x8);
    hbrush_var38 = CreateSolidBrush16(0x210070b);
    GetClientRect16(rect_2e, &DAT_1050_1050);
    FillRect16(hbrush_var38, rect_2e, &DAT_1050_1050);
    EndPaint16(CONCAT22(0x1050, paintstruct_22), param_1.field8_0x8);
    DeleteObject16(hbrush_var38);
    return;
}

pub unsafe fn unk_draw_op_1018_c578(param_1: *mut Struct57, param_2: *mut astruct_36) {
    let mut paVar1: *mut astruct_76;
    let mut uVar2: u16;
    let mut uVar5: u16;
    let mut hpal: *mut HDC16;
    let mut iVar5: i16;
    let mut iVar3: i16;
    let mut uVar6: u16;
    let mut uVar9: u16;
    let mut uVar7: u16;
    let mut extraout_DX: u16;
    let mut obj: HPALETTE16;
    let mut iVar4: *mut astruct_36;
    let mut uVar10: u16;
    let mut unaff_SI: u16;
    let mut uVar8: *mut astruct_36;
    let mut uVar11: u16;
    let mut uVar12: u32;
    let mut in_stack_0000fe56: u16;
    let mut in_stack_0000ff7a: u16;
    let mut in_stack_0000ff80: u16;
    let mut in_stack_0000ff84: u16;
    let mut rect_34: RECT16;
    let mut iStack48: i16;
    let mut iStack46: i16;
    let mut hbrush_44: HBRUSH16;
    let mut hdc_2a: HDC16;
    let mut uStack40: u16;
    let mut puStack38: *mut u32;
    let mut paintstruct_22: PAINTSTRUCT16 = PAINTSTRUCT16::default();
    let mut uVar1: u16;
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut puVar3: *mut u8;
    let mut fn_ptr_1: *mut *mut code;

    puStack38 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x2),
        in_stack_0000fe56,
        in_stack_0000ff7a,
        in_stack_0000ff80,
        in_stack_0000ff84,
    );
    uVar9 = (puStack38 >> 0x10);
    uVar5 = (puStack38 + 0x20);
    iVar4 = param_2;
    uVar8 = (param_2 >> 0x10);
    uStack40 = uVar5;
    if (uVar5 == 0) {
        BeginPaint16(CONCAT22(0x1050, &paintstruct_22), iVar4.hwnd_0x8);
        EndPaint16(CONCAT22(0x1050, &paintstruct_22), iVar4.hwnd_0x8);
        PostMessage16(0x0, iVar4.wparam_0xea, 0x111, HWND16_1050_0396);
        return;
    }
    if ((iVar4.field235_0xf0 == 0) && (iVar4.field238_0xf4 != 0)) {
        iVar4.field235_0xf0 = 0x1;
        puVar3 = &iVar4.field_0xf2;
        win_1008_5c9e(
            puVar3,
            uVar9,
            _u16_1050_02a0,
            (param_2 & 0xffff0000 | ZEXT24(puVar3)),
        );
        uVar5 = puVar3;
    }
    if ((_u16_1050_02a0 + 0x12) == 0) {
        win_1008_5c5c(uVar5, uVar9, _u16_1050_02a0, 0x1d3);
    }
    hdc_2a = BeginPaint16(CONCAT22(0x1050, &paintstruct_22), iVar4.hwnd_0x8);
    hbrush_44 = CreateSolidBrush16(0x2000000);
    rect_34 = 0x0;
    iStack48 = iVar4.field239_0xf6 - 0x1;
    iStack46 = iVar4.field240_0xf8 - 0x1;
    FillRect16(hbrush_44, &rect_34, &DAT_1050_1050);
    DeleteObject16(hbrush_44);
    uVar3 = iVar4.field225_0xe2;
    paVar1 = (uVar3 + 0xe);
    hpal = &hdc_2a;
    uVar11 = (paVar1 >> 0x10);
    uVar10 = paVar1;
    uVar4 = paVar1;
    fn_ptr_1 = (uVar4 + 0x8);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538, uVar10, uVar11, hpal, &DAT_1050_1050);
    uVar12 = pass1_1008_4772(paVar1);
    uVar2 = (uVar12 >> 0x10);
    iVar1 = (uVar12 + 0x4);
    iVar2 = (uVar12 + 0x8);
    iVar5 = 0x1e0 - iVar2;
    extraout_DX = iVar5 >> 0xf;
    iVar3 = iVar5 / 0x2;
    iVar4.field249_0x10c = iVar3 + iVar2 + iVar4.field251_0x110;
    fn_ptr_1 = (uVar4 + 0x4);
    (**fn_ptr_1)(
        0x1008,
        uVar10,
        uVar11,
        iVar4.field242_0xfc + iVar4.field243_0xfe + iVar3,
        iVar4.field241_0xfa + (0x280 - iVar1) / 0x2,
        &hdc_2a,
        &DAT_1050_1050,
    );
    draw_text_1018_c742(extraout_DX, param_2, &hdc_2a, &DAT_1050_1050, uVar10);
    obj = SelectPalette16(0x0, hpal, hdc_2a);
    DeleteObject16(obj);
    EndPaint16(CONCAT22(0x1050, &paintstruct_22), iVar4.hwnd_0x8);
    return;
}

pub unsafe fn unk_draw_op_1018_cda8(
    param_1: *mut Struct57,
    mut param_2: u16,
    mut param_3: u16,
    struct36_param_1: *mut astruct_36,
) {
    let mut paVar1: *mut astruct_76;
    let mut uVar9: u16;
    let mut uVar4: u16;
    let mut uVar11: u16;
    let mut uVar7: u16;
    let mut uVar3: u16;
    let mut hpalette_var1: *mut HDC16;
    let mut iVar4: i16;
    let mut iVar10: i16;
    let mut iVar2: i16;
    let mut selected_obj: HPALETTE16;
    let mut uVar12: u16;
    let mut struct36_var3: *mut astruct_36;
    let mut uVar13: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar8: u16;
    let mut uVar14: u32;
    let mut in_stack_0000fe5a: u16;
    let mut in_stack_0000ff7e: u16;
    let mut in_stack_0000ff84: u16;
    let mut in_stack_0000ff88: u16;
    let mut rect_var34: RECT16;
    let mut iStack48: i16;
    let mut iStack46: i16;
    let mut brush_handle_var44: HBRUSH16;
    let mut hdc_2a: HDC16;
    let mut uStack40: u16;
    let mut puStack38: *mut u32;
    let mut paintstruct_var_22: [u8; 0x20] = [0; 0x20];
    let mut piVar1: *mut i16;
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut in_stack_0000ffb0: u32;
    let mut uVar10: u16;
    let mut fn_ptr_2: *mut *mut code;

    puStack38 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x2),
        in_stack_0000fe5a,
        in_stack_0000ff7e,
        in_stack_0000ff84,
        in_stack_0000ff88,
    );
    uVar11 = (puStack38 >> 0x10);
    uVar3 = (puStack38 + 0x20);
    struct36_var3 = struct36_param_1;
    uVar5 = (struct36_param_1 >> 0x10);
    uStack40 = uVar3;
    if (uVar3 == 0) {
        BeginPaint16(CONCAT22(0x1050, paintstruct_var_22), struct36_var3.hwnd_0x8);
        EndPaint16(CONCAT22(0x1050, paintstruct_var_22), struct36_var3.hwnd_0x8);
        PostMessage16(0x0, struct36_var3.wparam_0xea, 0x111, HWND16_1050_0396);
        return;
    }
    if (struct36_var3.field235_0xf0 == 0) {
        struct36_var3.field235_0xf0 = 0x1;
        win_1008_5c5c(uVar3, uVar11, _u16_1050_02a0, 0x1f3);
        uVar6 = (_u16_1050_02a0 >> 0x10);
        if ((_u16_1050_02a0 + 0x12) == 0) {
            win_1008_5c5c(uVar3, uVar11, _u16_1050_02a0, 0x1d3);
        }
    }
    hdc_2a = BeginPaint16(CONCAT22(0x1050, paintstruct_var_22), struct36_var3.hwnd_0x8);
    brush_handle_var44 = CreateSolidBrush16(0x2000000);
    rect_var34 = 0x0;
    iStack48 = struct36_var3.field239_0xf6 - 0x1;
    iStack46 = struct36_var3.field240_0xf8 - 0x1;
    FillRect16(brush_handle_var44, &rect_var34, &DAT_1050_1050);
    DeleteObject16(brush_handle_var44);
    uVar2 = struct36_var3.field225_0xe2;
    paVar1 = (uVar2 + 0xe);
    hpalette_var1 = &hdc_2a;
    uVar8 = (paVar1 >> 0x10);
    uVar13 = paVar1;
    fn_ptr_2 = (paVar1 + 0x8);
    (**fn_ptr_2)(
        s_tile2_bmp_1050_1538,
        uVar13,
        uVar8,
        hpalette_var1,
        &DAT_1050_1050,
    );
    uVar14 = pass1_1008_4772(paVar1);
    uVar9 = (uVar14 >> 0x10);
    iVar4 = (0x280 - (uVar14 + 0x4)) / 0x2;
    iVar1 = (uVar14 + 0x8);
    iVar10 = 0x1e0 - iVar1;
    uVar12 = iVar10 >> 0xf;
    iVar2 = iVar10 / 0x2;
    struct36_var3.field249_0x10c = iVar2 + iVar1 + struct36_var3.field251_0x110;
    if ((struct36_var3.field241_0xfa == 0) && (iVar4 == 0)) {
        piVar1 = &struct36_var3.field241_0xfa;
        *piVar1 = *piVar1 + 2;
    }
    fn_ptr_2 = (paVar1 + 0x4);
    (**fn_ptr_2)(
        0x1008,
        uVar13,
        uVar8,
        struct36_var3.field242_0xfc + struct36_var3.field243_0xfe + iVar2,
        struct36_var3.field241_0xfa + iVar4,
        &hdc_2a,
        &DAT_1050_1050,
    );
    draw_text_1018_c742(uVar12, struct36_param_1, &hdc_2a, &DAT_1050_1050, uVar13);
    selected_obj = SelectPalette16(0x0, hpalette_var1, hdc_2a);
    DeleteObject16(selected_obj);
    EndPaint16(CONCAT22(0x1050, paintstruct_var_22), struct36_var3.hwnd_0x8);
    return;
}

pub unsafe fn unk_draw_op_1018_cfc0(
    param_1: *mut Struct57,
    mut param_2: u16,
    struct36_param_1: *mut astruct_36,
) {
    let mut uVar1: u16;
    let mut uVar4: u16;
    let mut hpal: *mut HDC16;
    let mut iVar3: i16;
    let mut iVar2: i16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar9: u16;
    let mut uVar6: u16;
    let mut obj: HPALETTE16;
    let mut uVar10: u16;
    let mut struct36_var5: *mut astruct_36;
    let mut uVar11: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar12: u32;
    let mut in_stack_0000fe58: u16;
    let mut in_stack_0000ff7c: u16;
    let mut in_stack_0000ff82: u16;
    let mut in_stack_0000ff86: u16;
    let mut rect_34: RECT16;
    let mut iStack48: i16;
    let mut iStack46: i16;
    let mut hbrush_44: HBRUSH16;
    let mut local_2a: HDC16;
    let mut iStack40: i16;
    let mut puStack38: *mut u32;
    let mut paintstruct_22: [u8; 0x20] = [0; 0x20];
    let mut piVar1: *mut i16;
    let mut iVar1: i16;
    let mut uVar3: u32;
    let mut in_stack_0000ffb0: u16;
    let mut fn_ptr_2: *mut *mut code;
    let mut struct76_var1: *mut astruct_76;

    puStack38 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x2),
        in_stack_0000fe58,
        in_stack_0000ff7c,
        in_stack_0000ff82,
        in_stack_0000ff86,
    );
    uVar9 = (puStack38 >> 0x10);
    iStack40 = (puStack38 + 0x20);
    struct36_var5 = struct36_param_1;
    uVar7 = (struct36_param_1 >> 0x10);
    if (iStack40 == 0) {
        BeginPaint16(CONCAT22(0x1050, paintstruct_22), struct36_var5.hwnd_0x8);
        EndPaint16(CONCAT22(0x1050, paintstruct_22), struct36_var5.hwnd_0x8);
        PostMessage16(0x0, struct36_var5.wparam_0xea, 0x111, HWND16_1050_0396);
        return;
    }
    if ((struct36_var5.field235_0xf0 == 0) && (struct36_var5.field238_0xf4 != 0)) {
        struct36_var5.field235_0xf0 = 0x1;
        uVar4 = &struct36_var5.field_0xf2;
        win_1008_5c9e(
            uVar4,
            uVar9,
            _u16_1050_02a0,
            (struct36_param_1 & 0xffff0000 | uVar4),
        );
        if ((_u16_1050_02a0 + 0x12) == 0) {
            win_1008_5c5c(uVar4, uVar9, _u16_1050_02a0, 0x1d3);
        }
    }
    local_2a = BeginPaint16(CONCAT22(0x1050, paintstruct_22), struct36_var5.hwnd_0x8);
    hbrush_44 = CreateSolidBrush16(0x2000000);
    rect_34 = 0x0;
    iStack48 = struct36_var5.field239_0xf6 - 0x1;
    iStack46 = struct36_var5.field240_0xf8 - 0x1;
    FillRect16(hbrush_44, &rect_34, &DAT_1050_1050);
    DeleteObject16(hbrush_44);
    uVar3 = struct36_var5.field225_0xe2;
    struct76_var1 = (uVar3 + 0xe);
    hpal = &local_2a;
    uVar8 = (struct76_var1 >> 0x10);
    uVar11 = struct76_var1;
    fn_ptr_2 = (struct76_var1 + 0x8);
    (**fn_ptr_2)(s_tile2_bmp_1050_1538, uVar11, uVar8, hpal, &DAT_1050_1050);
    uVar12 = pass1_1008_4772(struct76_var1);
    uVar1 = (uVar12 >> 0x10);
    iVar3 = (0x280 - (uVar12 + 0x4)) / 0x2;
    iVar1 = (uVar12 + 0x8);
    iVar2 = 0x1e0 - iVar1;
    uVar10 = iVar2 >> 0xf;
    iVar4 = iVar2 / 0x2;
    struct36_var5.field249_0x10c = iVar4 + iVar1 + struct36_var5.field251_0x110;
    if ((struct36_var5.field241_0xfa == 0) && (iVar3 == 0)) {
        piVar1 = &struct36_var5.field241_0xfa;
        *piVar1 = *piVar1 + 2;
    }
    fn_ptr_2 = (struct76_var1 + 0x4);
    (**fn_ptr_2)(
        0x1008,
        uVar11,
        uVar8,
        struct36_var5.field242_0xfc + struct36_var5.field243_0xfe + iVar4,
        struct36_var5.field241_0xfa + iVar3,
        &local_2a,
        &DAT_1050_1050,
    );
    draw_text_1018_c742(uVar10, struct36_param_1, &local_2a, &DAT_1050_1050, uVar11);
    obj = SelectPalette16(0x0, hpal, local_2a);
    DeleteObject16(obj);
    EndPaint16(CONCAT22(0x1050, paintstruct_22), struct36_var5.hwnd_0x8);
    return;
}


pub unsafe fn clenaup_win_ui_1018_4d22(in_struct_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut local_struct_1: *mut StructD;
    let mut uVar4: *mut StructD;
    let mut uVar3: u16;
    let mut unaff_SS: u16;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut puVar2: *mut u32;
    let mut puVar1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar3 = 0x1018;
    uVar4 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    // just 0x5058
    in_struct_1.address_offset_field_0x0 = s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
    local_struct_1.address_offset_field_0x2 = 0x1018;
    if (local_struct_1.field11_0x12 != 0) {
        obj = SelectPalette16(
            0x0,
            local_struct_1.field14_0x1a,
            local_struct_1.field11_0x12,
        );
        DeleteObject16(obj);
        uVar3 = SUB42(s_tile2_bmp_1050_1538, 0x0);
        DeleteDC16(local_struct_1.field11_0x12);
    }
    puVar1 = local_struct_1.field6_0xa;
    uVar1 = local_struct_1.field7_0xc;
    if ((uVar1 | puVar1) != 0) {
        fn_ptr_1 = *puVar1;
        (**fn_ptr_1)(uVar3, puVar1, uVar1, 1);
    }
    puVar2 = local_struct_1.field8_0xe;
    uVar2 = &local_struct_1.field_0x10;
    if ((uVar2 | puVar2) != 0) {
        fn_ptr_1 = *puVar2;
        (**fn_ptr_1)(uVar3, puVar2, uVar2, 1);
    }
    _PTR_LOOP_1050_4230 = 0;
    pass1_1010_1d80(in_struct_1);
    return;
}


pub unsafe fn win_ui_op_1008_3c34(mut param_1: u32, param_2: u8, hdc_param_3: HDC16) {
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut HVar3: HPALETTE16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puStack6: *mut u32;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if (((iVar4 + 0xa) | (iVar4 + 0x8)) != 0) {
        puStack6 = (iVar4 + 0x8);
        if (((iVar4 + 0xc) != 0) && ((param_2 & 1) != 0)) {
            puStack6 = (iVar4 + 0xc);
        }
        if (((iVar4 + 0x10) != 0) && ((param_2 & 0x4) != 0)) {
            puStack6 = (iVar4 + 0x10);
        }
        uVar6 = (_PTR_LOOP_1050_4230 >> 0x10);
        uVar1 = (_PTR_LOOP_1050_4230 + 0x10);
        HVar3 = palette_op_1008_4e08(
            &hdc_param_3,
            uVar1,
            CONCAT22(uVar1, (_PTR_LOOP_1050_4230 + 0xe)),
            CONCAT22(0x1050, &hdc_param_3),
        );
        ppcVar2 = (*puStack6 + 0x4);
        (**ppcVar2)();
        HVar3 = SelectPalette16(0x0, HVar3, hdc_param_3);
        DeleteObject16(HVar3);
    }
    return;
}

// WARNING: Unable to use type for symbol uVar3
pub unsafe fn delete_palette_1018_e16c(param_1: *mut astruct_795) {
    let mut puVar2: *mut u32;
    let mut hpal: *mut HDC16;
    let mut hpalette_a: HPALETTE16;
    let mut iVar5: *mut astruct_795;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut hwnd16_ss: u16;
    let mut hdc_var24: HDC16;
    let mut paintstruct_22: [u8; 0x20] = [0; 0x20];
    let mut uVar4: u32;
    let mut uVar3: u32;
    let mut puVar3: *mut u32;
    let mut puVar1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar5 = (param_1 >> 0x10);
    iVar5 = param_1;
    hdc_var24 = BeginPaint16(CONCAT22(0x1050, paintstruct_22), iVar5.hwnd_0x4);
    uVar3 = iVar5.field5_0x6;
    puVar2 = (uVar3 + 0xa);
    hpal = &hdc_var24;
    uVar6 = (puVar2 >> 0x10);
    uVar4 = *puVar2;
    fn_ptr_1 = (uVar4 + 0x8);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538, puVar2, uVar6, hpal, &DAT_1050_1050);
    fn_ptr_1 = (uVar4 + 0x4);
    (**fn_ptr_1)(
        s_tile2_bmp_1050_1538,
        puVar2,
        uVar6,
        0x0,
        &hdc_var24,
        &DAT_1050_1050,
    );
    hpalette_a = SelectPalette16(0x0, hpal, hdc_var24);
    DeleteObject16(hpalette_a);
    EndPaint16(CONCAT22(0x1050, paintstruct_22), iVar5.hwnd_0x4);
    return;
}

// WARNING: Unable to use type for symbol uVar3
pub unsafe fn unk_draw_op_1020_0c3e(param_1: *mut astruct_771) {
    let mut puVar2: *mut u32;
    let mut hpal: *mut HDC16;
    let mut obj: HPALETTE16;
    let mut struct_1: *mut astruct_771;
    let mut iVar5: *mut astruct_842;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uStack40: u16;
    let mut local_24: HDC16;
    let mut paintstruct_22: [u8; 0x20] = [0; 0x20];
    let mut uVar3: u32;
    let mut puVar1: *mut u32;
    let mut uVar4: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar5 = (param_1 >> 0x10);
    struct_1 = param_1;
    local_24 = BeginPaint16(CONCAT22(0x1050, paintstruct_22), struct_1.hwnd_0x4);
    uVar3 = struct_1.field5_0x6;
    uVar6 = (uVar3 >> 0x10);
    iVar5 = uVar3;
    puVar2 = &iVar5.field_0xa;
    uStack40 = puVar2;
    if ((iVar5.field12_0xc | uStack40) != 0) {
        hpal = &local_24;
        uVar4 = *puVar2;
        fn_ptr_1 = (uVar4 + 0x8);
        (**fn_ptr_1)(
            s_tile2_bmp_1050_1538,
            uStack40,
            (puVar2 >> 0x10),
            hpal,
            &DAT_1050_1050,
        );
        fn_ptr_1 = (uVar4 + 0x4);
        (**fn_ptr_1)(
            s_tile2_bmp_1050_1538,
            puVar2,
            struct_1.field7_0xc,
            struct_1.field6_0xa,
            &local_24,
            &DAT_1050_1050,
        );
        obj = SelectPalette16(0x0, hpal, local_24);
        DeleteObject16(obj);
    }
    EndPaint16(CONCAT22(0x1050, paintstruct_22), struct_1.hwnd_0x4);
    return;
}

// WARNING: Unable to use type for symbol uVar1
pub unsafe fn win_ui_op_1020_150e(param_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut iVar1: *mut StructD;
    let mut uVar2: u16;
    let mut unaff_SS: u16;
    let mut uVar1: i32;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x1730;
    iVar1.address_offset_field_0x2 = 0x1020;
    if (iVar1.field12_0x14 != 0) {
        pass1_1010_1ea6(iVar1.field12_0x14, (param_1 & 0xffff | uVar2 << 0x10));
    }
    uVar1 = iVar1.field12_0x14;
    obj = SelectPalette16(0x0, &iVar1.field_0x1c, *(uVar1 + 0x7c));
    iVar1.field_0x1c = obj;
    DeleteObject16(obj);
    param_1.address_offset_field_0x0 = 0x3ab0;
    iVar1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    iVar1.address_offset_field_0x2 = 0x1008;
    return;
}

// WARNING: Unable to use type for symbol uVar2
pub unsafe fn draw_op_1020_15de(param_1: *mut astruct_779) {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut iVar3: *mut astruct_779;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut paVar3: *mut astruct_76;
    let mut local_24: HDC16;
    let mut local_22: [u8; 0x20] = [0; 0x20];
    let mut uVar2: u32;
    let mut uVar1: u32;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    local_24 = BeginPaint16(CONCAT22(0x1050, local_22), &iVar3.field_0x4);
    BVar2 = IsIconic16(&iVar3.field_0x4);
    if (BVar2 == 0) {
        uVar4 = 0x1010;
        paVar3 = pass1_1010_454a(iVar3.field20_0x14);
        if (((paVar3 >> 0x10) | paVar3) != 0) {
            uVar1 = iVar3.field20_0x14;
            uVar4 = 0x1008;
            pass1_1008_4480((iVar3 + 1), (uVar1 & 0xffff0000 | (uVar1 + 0x76)), paVar3);
        }
        uVar2 = (iVar3 + 1);
        ppcVar1 = (*(iVar3 + 1) + 0x4);
        (**ppcVar1)(
            uVar4,
            uVar2,
            (uVar2 >> 0x10),
            0x0,
            0x0,
            &local_24,
            &DAT_1050_1050,
        );
    } else {
        draw_op_1020_1674(param_1);
    }
    EndPaint16(CONCAT22(0x1050, local_22), &iVar3.field_0x4);
    return;
}

// WARNING: Unable to use type for symbol puVar1
pub unsafe fn draw_op_1020_1674(param_1: *mut astruct_779) {
    let mut hdc: HDC16;
    let mut struct_1: *mut astruct_779;
    let mut uVar2: u16;
    let mut rect_1a: RECT16;
    let mut iStack22: i16;
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut rect_e: RECT16;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut brush_handle: HGDIOBJ16;
    let mut icon_handle: HICON16;
    let mut puVar1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    if (PTR_LOOP_1050_0010.is_null()) {
        uVar2 = (param_1 >> 0x10);
        struct_1 = param_1;
        fn_ptr_1 = (*struct_1.field20_0x14 + 0x2c);
        icon_handle = (**fn_ptr_1)();
        if (icon_handle != 0) {
            brush_handle = GetStockObject16(BLACK_BRUSH);
            GetClientRect16(&rect_e, &DAT_1050_1050);
            rect_1a.x = 0;
            rect_1a.y = 0;
            iStack22 = (iStack10 - rect_e.x) - 0x1;
            iStack20 = (iStack8 - rect_e.y) - 0x1;
            puVar1 = struct_1.field20_0x14;
            hdc = *(puVar1 + 0x7c);
            iStack18 = iStack20;
            iStack16 = iStack22;
            FillRect16(brush_handle, &rect_1a, &DAT_1050_1050);
            DrawIcon16(icon_handle, 0x2, 0x2, hdc);
        }
    }
    return;
}

// WARNING: Unable to use type for symbol uVar3
pub unsafe fn palette_op_1020_7270(pstruct_param_1: *mut StructD) {
    let mut obj: HPALETTE16;
    let mut struct767_var1: *mut StructD;
    let mut uVar4: u16;
    let mut unaff_SS: u16;
    let mut pcStack8: *mut c_char;
    let mut uVar3: u32;
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar4 = (pstruct_param_1 >> 0x10);
    struct767_var1 = pstruct_param_1;
    pstruct_param_1.address_offset_field_0x0 = 0x754c;
    struct767_var1.address_offset_field_0x2 = 0x1020;
    if (&struct767_var1.field_0x1c != 0) {
        pass1_1010_1ea6(
            &struct767_var1.field_0x1c,
            (pstruct_param_1 & 0xffff | uVar4 << 0x10),
        );
    }
    uVar1 = &struct767_var1.field12_0x14;
    uVar2 = (&struct767_var1.field12_0x14 + 2);
    pcStack8 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0) {
        pass1_1008_5118(CONCAT22(uVar2, uVar1));
        fn_ptr_1000_17ce(pcStack8);
    }
    uVar3 = &struct767_var1.field_0x1c;
    obj = SelectPalette16(0x0, struct767_var1.field19_0x20, *(uVar3 + 0x178));
    struct767_var1.field19_0x20 = obj;
    DeleteObject16(obj);
    pstruct_param_1.address_offset_field_0x0 = 0x3ab0;
    struct767_var1.address_offset_field_0x2 = 0x1008;
    pstruct_param_1.address_offset_field_0x0 = 0x389a;
    struct767_var1.address_offset_field_0x2 = 0x1008;
    return;
}

pub unsafe fn ui_cleanup_op_1040_782c(param_1: *mut StructD)

{
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut struct_1: *mut StructD;
  let mut uVar2: u16;
  let mut uVar1: u16;
  let mut puVar1: *mut u32;
  let mut fn_ptr_1: *mut *mut code;

  uVar2 = (param_1 >> 0x10);
  struct_1 = param_1;
  param_1.address_offset_field_0x0 = 0x840c;
  (struct_1 + 0x2) = &PTR_LOOP_1050_1040;
  puVar2 = (struct_1 + 0x70);
  uVar3 = (struct_1 + 0x72);
  if ((uVar3 | puVar2) != 0) {
    fn_ptr_1 = *puVar2;
    (**fn_ptr_1)();
  }
  if ((struct_1 + 0x4) != 0) {
    DeleteObject16((struct_1 + 0x4));
    (struct_1 + 0x4) = 0;
  }
  if ((struct_1 + 0x68) != 0) {
    DestroyMenu16((struct_1 + 0x68));
  }
  RemoveProp16(s_thisLo_1050_5db1,(struct_1 + 0x6));
  RemoveProp16(s_thisHi_1050_5db8,(struct_1 + 0x6));
  RemoveProp16(s_procLo_1050_5dbf,(struct_1 + 0x6));
  RemoveProp16(s_procHi_1050_5dc6,(struct_1 + 0x6));
  param_1.address_offset_field_0x0 = 0x389a;
  (struct_1 + 0x2) = 0x1008;
  return;
}


pub unsafe fn unk_draw_op_1040_b0f8(mut param_1: u16, param_2: *mut StructD)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_EDX: *mut Struct57;
  let mut iVar3: *mut StructD;
  let mut uVar3: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe92: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc0: u16;
  let mut uStack22: u16;
  let mut pcStack10: *mut c_char;
  let mut in_stack_0000ffe8: u32;

  uVar3 = (param_2 >> 0x10);
  iVar3 = param_2;
  param_2.address_offset_field_0x0 = 0xb772;
  iVar3.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  _param_1 = CONCAT22(uStack22,0x32);
  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,_param_1,in_stack_0000fe92,in_stack_0000ffb6,
                           in_stack_0000ffbc,in_stack_0000ffc0);
  pass1_1010_7b8c(puVar3,&iVar3.field_0x6);
  if (&iVar3.field_0x8e != 0) {
    DeleteObject16(&iVar3.field_0x8e);
    iVar3.field_0x8e = 0;
  }
  uVar1 = &iVar3.field_0x90;
  uVar2 = &iVar3.field_0x92;
  pcStack10 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0) {
    _param_1 = CONCAT22(uVar2,uVar1);
    pass1_1040_a5d0(_param_1);
    fn_ptr_1000_17ce(pcStack10);
  }
  ui_cleanup_op_1040_782c(param_2);
  return;
}

pub unsafe fn win_prop_op_1038_d118(base_addr_param_4: u16, mut param_2: u32, mut param_3: u32, hwnd_param_3: HWND16)

{
  let mut uVar1: u32;
  let mut cVar2: u8;
  let mut HVar3: HANDLE16;
  let mut HVar4: HANDLE16;
  let mut puStack6: *mut u32;
  let mut uVar2: u32;
  let mut fn_ptr_1: *mut *mut code;

  HVar3 = GetProp16(CONCAT22(base_addr_param_4,0x5bf3),hwnd_param_3);
  HVar4 = GetProp16(CONCAT22(base_addr_param_4,0x5bec),hwnd_param_3);
  puStack6 = CONCAT22(HVar3,HVar4);
  if (param_3 == 0x30) {
    if (param_3 == 0) {
      return;
    }
    SetProp16(param_3,CONCAT22(base_addr_param_4,0x5c06),hwnd_param_3);
    return;
  }
  if (param_3 < 0x310000) {
    cVar2 = (param_3 >> 0x10);
    if (cVar2 == '\x02') {
      if ((HVar3 | HVar4) != 0) {
        uVar1 = *puStack6;
        fn_ptr_1 = uVar1 + 0x6;
        (**fn_ptr_1)(s_tile2_bmp_1050_1538,HVar4,HVar3,param_2,param_3);
        if (puStack6.is_null() == false) {
          fn_ptr_1 = uVar1;
          (**fn_ptr_1)(s_tile2_bmp_1050_1538,HVar4,HVar3,1);
        }
      }
      HVar3 = GetProp16(CONCAT22(base_addr_param_4,0x5bfa),hwnd_param_3);
      if (HVar3 == 0) {
        return;
      }
      DeleteObject16(HVar3);
      RemoveProp16(CONCAT22(base_addr_param_4,0x5c00),hwnd_param_3);
      return;
    }
    if (cVar2 == '\x06') {
      if ((param_3 != 1) && (param_3 != 0x2)) {
        uVar1 = &u16_1050_5bc8;
        (uVar1 + 0x8) = 0;
        return;
      }
      uVar2 = &u16_1050_5bc8;
      (uVar2 + 0x8) = hwnd_param_3;
      return;
    }
  }
  if ((HVar3 | HVar4) != 0) {
    fn_ptr_1 = (*puStack6 + 0xc);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,HVar4,HVar3,param_2,param_3);
  }
  return;
}


pub unsafe fn unk_draw_op_1040_9458(param_1: *mut astruct_17, param_2: u8, mut param_3: u16)

{
    let mut ppcVar1: *mut *mut code;
    let mut hpal: *mut u16;
    let mut obj: HPALETTE16;
    let mut uVar3: u16;
    let mut iVar4: *mut astruct_17;
    let mut uVar4: u16;
    let mut puStack8: *mut u16;
    let mut puStack6: *mut u32;
    let mut UVar2: u32;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    if (iVar4.field8_0x8.is_null() == false) {
        puStack6 = iVar4.field8_0x8;
        if ((((&iVar4.field9_0xc + 0x2) | &iVar4.field9_0xc) != 0) && ((param_2 & 1) != 0)) {
            puStack6 = iVar4.field9_0xc;
        }
        if ((iVar4.field10_0x10.is_null() == false) && ((param_2 & 0x4) != 0)) {
            puStack6 = iVar4.field10_0x10;
        }
        hpal = &param_3;
        UVar2 = *puStack6;
        ppcVar1 = (UVar2 + 0x8);
        (**ppcVar1)();
        ppcVar1 = (UVar2 + 0x4);
        (**ppcVar1)();
        obj = SelectPalette16(0x0, hpal, param_3);
        DeleteObject16(obj);
    }
    return;
}


pub unsafe fn palette_op_1040_c886(struct_param_1: *mut astruct_769, param_2: u8, hdc_param_3: HDC16)

{
    let mut uVar1: u16;
    let mut palette_2: HPALETTE16;
    let mut uVar4: u16;
    let mut struct_2: *mut astruct_769;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut unaff_CS: u16;
    let mut puStack8: *mut u32;
    let mut palette: HPALETTE16;
    let mut fn_ptr_1: *mut *mut code;

    uVar2 = (struct_param_1 >> 0x10);
    struct_2 = struct_param_1;
    if (((&struct_2.field8_0x8 + 0x2) | &struct_2.field8_0x8) != 0) {
        palette = 0;
        if (struct_2.field59_0x46 == 0) {
            uVar3 = (_PTR_LOOP_1050_4230 >> 0x10);
            uVar1 = (_PTR_LOOP_1050_4230 + 0x10);
            unaff_CS = 0x1008;
            palette = palette_op_1008_4e08(&hdc_param_3, uVar1,
                                           CONCAT22(uVar1, (_PTR_LOOP_1050_4230 + 0xe)),
                                           CONCAT22(0x1050, &hdc_param_3));
        }
        puStack8 = struct_2.field8_0x8;
        uVar4 = (&struct_2.field8_0x8 + 2);
        if ((((&struct_2.field9_0xc + 0x2) | &struct_2.field9_0xc) != 0) && ((param_2 & 1) != 0)) {
            puStack8 = struct_2.field9_0xc;
            uVar4 = (&struct_2.field9_0xc + 2);
        }
        if ((struct_2.field10_0x10.is_null() == false) && ((param_2 & 0x4) != 0)) {
            puStack8 = struct_2.field10_0x10;
            uVar4 = (&struct_2.field10_0x10 + 2);
        }
        fn_ptr_1 = (*puStack8 + 0x4);
        (**fn_ptr_1)(unaff_CS, puStack8, uVar4, struct_2.field30_0x28, struct_2.field29_0x26, &hdc_param_3,
                     &DAT_1050_1050);
        if (struct_2.field59_0x46 == 0) {
            palette_2 = SelectPalette16(0x0, palette, hdc_param_3);
            DeleteObject16(palette_2);
        }
    }
    return;
}


pub unsafe fn cleanup_palette_1008_56e2(mut param_1: u32, mut param_2: u32) -> BOOL16

{
  let mut hpalette_a: HPALETTE16;
  let mut u16_a: u16;

  u16_a = (param_1 >> 0x10);
  hpalette_a = SelectPalette16(0x0,(param_1 + 0x4),*param_2);
  (param_1 + 0x4) = hpalette_a;
  DeleteObject16(hpalette_a);
  return 0x1;
}

pub unsafe fn win_ui_palette_op_1020_81c0(mut param_1: u32) {
    let mut in_struct_1: *mut astruct_13;
    let mut hdc: HDC16;
    let mut hpal: HDC16;
    let mut hpal_00: HPALETTE16;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut hdc_00: HDC16;
    let mut uStack6: u16;

    uVar2 = (_PTR_LOOP_1050_4230 >> 0x10);
    in_struct_1 = (_PTR_LOOP_1050_4230 + 0xe);
    uVar1 = (_PTR_LOOP_1050_4230 + 0x10);
    uStack6 = in_struct_1;
    if ((uVar1 | uStack6) == 0) {
        return;
    }
    hdc = GetDC16((param_1 + 0x8));
    hpal = hdc;
    hdc_00 = hdc;
    create_palette_1008_4e38(in_struct_1, uVar1);
    hpal_00 = SelectPalette16(0x0, hpal, hdc_00);
    RealizePalette16(hdc);
    SelectPalette16(0x1, hpal_00, hdc);
    RealizePalette16(hdc);
    DeleteObject16(hpal);
    if (0x0 < hpal) {
        InvalidateRect16(0x1, NULL, 0x0);
    }
    return;
}

pub unsafe fn unk_draw_op_1008_da12(param_1: *mut Struct19, mut param_2: u16) {
    let mut bVar1: u8;
    let mut hdc: HDC16;
    let mut horiz_res: i16;
    let mut vert_res: i16;
    let mut iVar2: i16;
    let mut raster_caps: u16;
    let mut sz_palette: i16;
    let mut num_reserved: i16;
    PALETTEENTRY * entries;
    let mut uVar4: u16;
    let mut start: u16;
    let mut count: *mut u8;
    let mut count_00: u16;
    let mut in_EDX: u32;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut puStack32: *mut u16;
    let mut iStack16: i16;
    let mut lStack8: i32;
    let mut uVar3: u32;
    let mut piVar1: *mut i16;
    let mut puVar2: *mut u16;
    let mut pSVar2: *mut StructD;

    uVar5 = (in_EDX >> 0x10);
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xc) = 0;
    pass1_1008_3e38((param_1 & 0xff000000 | CONCAT12((param_1 >> 0x10), param_1 + 0xe)));
    (param_1 + 0x14) = 0;
    (param_1 + 0x16) = 0;
    (param_1 + 0x18) = 0;
    param_1.offset_0x0 = 0xdc80;
    (param_1 + 0x2) = 0x1008;
    hdc = GetDC16(0x0);
    horiz_res = GetDeviceCaps16(HORZRES, hdc);
    (param_1 + 0xa) = horiz_res;
    vert_res = GetDeviceCaps16(VERTRES, hdc);
    (param_1 + 0xc) = vert_res;
    iVar2 = (param_1 + 0xc) -0x1e0;
    count = (iVar2 >> 0xf);
    pSVar2 = CONCAT22(uVar5, count);
    pass1_1008_3e76(
        (param_1 & 0xffff0000 | (param_1 + 0xe)),
        0x0,
        iVar2 / 0x2,
        ((param_1 + 0xa) -0x280) / 0x2,
    );
    raster_caps = GetDeviceCaps16(RASTERCAPS, hdc);
    if ((raster_caps & 0x100) != 0) {
        sz_palette = GetDeviceCaps16(SIZEPALETTE, hdc);
        (param_1 + 0x14) = sz_palette;
        num_reserved = GetDeviceCaps16(NUMRESERVED, hdc);
        (param_1 + 0x16) = num_reserved;
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar2);
        } else {
            pSVar2 = (pSVar2 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
        }
        entries = fn_ptr_op_1000_1708(
            (num_reserved + 1) * 0x4,
            0x0,
            0x1,
            PTR_LOOP_1050_5f2c,
            pSVar2,
        );
        count_00 = pSVar2;
        lStack8 = CONCAT22(count_00, entries);
        iVar6 = (param_1 + 0x16);
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar2);
            PTR_LOOP_1050_5f2e = pSVar2;
        } else {
        }
        uVar4 = fn_ptr_op_1000_1708(
            (iVar6 + 1) * 0x4,
            0x0,
            0x1,
            PTR_LOOP_1050_5f2c,
            PTR_LOOP_1050_5f2e,
        );
        (param_1 + 0x18) = uVar4;
        (param_1 + 0x1a) = PTR_LOOP_1050_5f2e;
        if (lStack8 != 0) {
            if ((param_1 + 0x18) != 0) {
                start = (param_1 + 0x16) / 0x2;
                GetSystemPaletteEntries(entries, count_00, start, 0x0);
                GetSystemPaletteEntries(entries + start, count_00, start, (param_1 + 0x14) - start);
                puStack32 = (param_1 + 0x18);
                // for (iStack16 = 0; puVar2 = puStack32, piVar1 = (param_1 + 0x16),
                // *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 += 1)
                iStack16 = 0;
                puVar2 = (param_1 + 0x16);
                while (*piVar1 != iStack16) && (iStack16 <= *pivar1) {
                    bVar1 = (entries + iStack16).pe_red;
                    uVar3 = puStack32 >> 0x10;
                    iVar6 = puStack32;
                    puStack32 = (puStack32 & 0xffff0000 | (iVar6 + 0x4));
                    *puVar2 = CONCAT11(entries[iStack16].pe_green, entries[iStack16].pe_blue);
                    (iVar6 + 0x2) = bVar1;
                    iStack16 += 1;
                }
            }
        }
        fn_ptr_1000_17ce(CONCAT22(count_00, entries));
    }
    ReleaseDC16(hdc, 0x0);
    return;
}


pub unsafe fn unk_win_ui_op_1038_9bc8(mut param_1: u16, mut param_2: u16, mut param_3: u16, param_4: *mut StructB)

{
  let mut IVar2: i16;
  let mut iVar2: i16;
  let mut hdc: HDC16;
  let mut IVar1: i16;
  let mut HVar2: HWND16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
   let mut struct_b_7: *mut StructB;
  let mut uVar4: u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe60: u16;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff8a: u16;
  let mut in_stack_0000ff8e: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  let mut piVar6: *mut i16;
  let mut uVar7: u16;
  let mut piVar8: *mut i16;
  let mut uVar9: u16;
  let mut iStack36: i16;
  let mut local_16: [u8;0x2] = [0;0x2];
  let mut iStack20: i16;
  let mut iStack16: i16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut local_6: i16;
  let mut local_4: i16;
  let mut iVar3: *mut astruct_778;
  let mut piVar1: *mut i16;
  let mut in_stack_0000ffc6: u32;
  let mut uVar16: u16;
  let mut fn_ptr_1: *mut *mut code;

  paVar3 = CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(param_4);
  if (PTR_LOOP_1050_5ef8 == (&u32_1050_0004 + 1)) {
    PTR_LOOP_1050_5ef8 = null_mut();
  }
  piVar8 = &local_4;
  uVar9 = SUB42(&DAT_1050_1050,0x0);
  piVar6 = &local_6;
  uVar7 = SUB42(&DAT_1050_1050,0x0);
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,CONCAT22(piVar6,0x48),in_stack_0000fe60,in_stack_0000ff84
                           ,in_stack_0000ff8a,in_stack_0000ff8e);
  uVar4 = (paVar3 >> 0x10);
  uStack10 = puVar5;
  uStack10 = (puVar5 >> 0x10);
  pass1_1008_3e94((puVar5 & 0xffff0000 | (uStack10 + 0xe)),CONCAT22(uVar7,piVar6),
                  CONCAT22(uVar9,piVar8));
  IVar2 = GetSystemMetrics16(SM_CYCAPTION);
  paVar3 = CONCAT22(uVar4,((IVar2 * PTR_LOOP_1050_5ef8) >> 0x10));
  iVar2 = (IVar2 * PTR_LOOP_1050_5ef8) + 0xa;
  PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 1;
  uStack14 = CONCAT22(iVar2 + local_4,iVar2 + local_6);
  uVar4 = (param_4 >> 0x10);
  struct_b_7 = param_4;
  GetWindowRect16(CONCAT22(0x1050,local_16),struct_b_7.lpvoid_field_0x8);
  hdc = GetDC16(0x0);
  IVar1 = GetDeviceCaps16(VERTRES,hdc);
  ReleaseDC16(hdc,0x0);
  if (IVar1 < iStack16) {
    uStack14 = uStack14 & 0xffff0000 | ((iStack20 - (iStack16 - IVar1)) + 1);
  }
  SetWindowPos16(0x1,0x0,0x0,uStack14,(uStack14 >> 0x10),0x0,struct_b_7.lpvoid_field_0x8);
  _param_3 = CONCAT22(param_2,0x3);
  uVar9 = 0x1010;
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,_param_3,in_stack_0000fe70,in_stack_0000ff94,in_stack_0000ff9a,
                           in_stack_0000ff9e);
  uVar7 = (puVar5 >> 0x10);
  iStack36 = 0;
  while (iVar3 = (iStack36 * 0x2), (&iVar3[0x52].field_0x0 + puVar5) != 0) {
    _param_3 = (_param_3 & 0xffff0000);
    uVar9 = SUB42(s_tile2_bmp_1050_1538,0x0);
    HVar2 = GetDlgItem16((&iVar3[0x52].field_0x0 + puVar5),struct_b_7.lpvoid_field_0x8);
    (&iVar3[0x4a].field_0x0 + &struct_b_7.field0_0x0) = HVar2;
    iStack36 += 0x1;
    piVar1 = &struct_b_7[0xe].field8_0x10;
    *piVar1 = *piVar1 + 1;
  }
  fn_ptr_1 = (param_4 + 0x6c);
  (**fn_ptr_1)(uVar9,param_4,param_2);
  return;
}

pub unsafe fn set_struct_op_1008_0536(param_1: *mut astruct_20, mut param_2: u16) {
    let mut hicon_1: HICON16;
    let mut hcursor_1: HCURSOR16;
    let mut hbrush_1: HGDIOBJ16;
    let mut in_EDX: u32;
    let mut uVar2: u16;
    let mut paVar1: *mut Struct57;
    let mut paVar3: *mut astruct_20;
    let mut puVar4: *mut u32;
    let mut in_stack_0000feac: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd6: u16;
    let mut in_stack_0000ffda: u16;

    uVar2 = (in_EDX >> 0x10);
    paVar3 = pass1_1008_3ab8(param_1);
    paVar1 = CONCAT22(uVar2, (paVar3 >> 0x10));
    (param_1 + 0xe0) = 0;
    (param_1 + 0xe4) = 0;
    (param_1 + 0xe8) = 0;
    (param_1 + 0xec) = 0;
    (param_1 + 0xee) = 0;
    (param_1 + 0xf2) = 0;
    (param_1 + 0xf4) = 0;
    (param_1 + 0xf8) = 0;
    param_1.offset_0x0 = 0x389e;
    (param_1 + 0x2) = 0x1008;
    (param_1 + 0xc8) = 0x2008;
    (param_1 + 0xac) = 0;
    (param_1 + 0xae) = 0x8700;
    hicon_1 = LoadIcon16(s_Op_1050_00d4, HINSTANCE16_1050_038c);
    (param_1 + 0xc2) = hicon_1;
    hcursor_1 = LoadCursor16(0x7f00, 0x0);
    (param_1 + 0xc4) = hcursor_1;
    hbrush_1 = GetStockObject16(BLACK_BRUSH);
    (param_1 + 0xc6) = hbrush_1;
    puVar4 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0 & 0xffff,
        CONCAT22(param_1, 0x48),
        in_stack_0000feac,
        in_stack_0000ffd0,
        in_stack_0000ffd6,
        in_stack_0000ffda,
    );
    paVar1 = (paVar1 & 0xffff0000 | puVar4 >> 0x10);
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0xa)),
        s_Outpost_1050_00d7,
    );
    puVar4 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0 & 0xffff,
        CONCAT22(param_1, 0x32),
        in_stack_0000feac,
        in_stack_0000ffd0,
        in_stack_0000ffd6,
        in_stack_0000ffda,
    );
    (param_1 + 0xf4) = puVar4;
    (param_1 + 0xf6) = (puVar4 >> 0x10);
    set_sys_color_1008_357e(param_1, 0x1, paVar1 & 0xffff0000 | puVar4 >> 0x10);
    return;
}


pub unsafe fn unk_draw_op_1008_61b2(
    mut param_1: u16,
    param_2: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u32,
) -> *mut astruct_20 {
    let mut l_hgdiobj_1: HGDIOBJ16;
    let mut l_hcursor_1: HCURSOR16;
    let mut in_EDX: *mut Struct57;
    let mut iVar2: *mut astruct_20;
    let mut uVar2: u16;
    let mut l_struct_2: *mut u16;
    let mut in_stack_0000fe90: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffbe: u16;
    let mut uVar5: *mut astruct_20;
    let mut in_stack_0000ffe8: u16;
    let mut iVar1: *mut astruct_20;
    let mut iVar4: *mut astruct_20;
    let mut uVar1: *mut u16;

    block_1008_6000::set_struct_1008_687a(param_2, param_5);
    uVar2 = (param_2 >> 0x10);
    iVar2 = param_2;
    iVar2.field164_0xde = param_3;
    iVar2.field165_0xe0 = 0;
    param_2.field0_0x0 = 0x6378;
    iVar2.base_0x2 = 0x1008;
    unk_str_op_1000_3d3e(
        (param_2 & 0xffff0000 | ZEXT24(&iVar2.field60_0x5b)),
        s_DanBrotherton_1050_0302,
    );
    l_hgdiobj_1 = GetStockObject16(BLACK_BRUSH);
    iVar2.hgdiobj_field_0xc6 = l_hgdiobj_1;
    l_hcursor_1 = LoadCursor16(0x7f00, 0x0);
    iVar2.hcursor_field_0xc4 = l_hcursor_1;
    iVar2.field150_0xc8 = 0x200b;
    iVar2.field139_0xac = 0x45000000;
    iVar2.field145_0xbc = (param_5 + 0x8);
    l_struct_2 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(param_1, 0x48),
        in_stack_0000fe90,
        in_stack_0000ffb4,
        in_stack_0000ffba,
        in_stack_0000ffbe,
    );
    uVar1 = (l_struct_2 >> 0x10);
    iVar2.field141_0xb4 = 0;
    iVar2.field142_0xb6 = 0;
    iVar2.field143_0xb8 = (l_struct_2 + 0xa);
    iVar2.field144_0xba = (l_struct_2 + 0xc);
    iVar2.field151_0xca = param_4;
    win_ui_reg_class_1008_96d2(param_2);
    return param_2;
}

pub unsafe fn unk_draw_op_1008_7f62(
    param_1: *mut astruct_20,
    param_2: u16,
    param_3: u32,
) -> *mut astruct_20 {
    let mut HVar1: HGDIOBJ16;
    let mut HVar2: HCURSOR16;
    let mut iVar4: *mut astruct_20;
    let mut uVar3: u16;
    let mut uVar4: *mut astruct_20;
    let mut iVar3: *mut astruct_20;

    set_struct_1008_687a(param_1, param_3);
    uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    iVar4.field164_0xde = param_2;
    param_1.offset_0x0 = 0x8042;
    iVar4.base_0x2 = 0x1008;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar4.field60_0x5b)),
        s_SOLChildPar_1050_0358,
    );
    HVar1 = GetStockObject16(HOLLOW_BRUSH);
    iVar4.hgdiobj_field_0xc6 = HVar1;
    HVar2 = LoadCursor16(0x7f00, 0x0);
    iVar4.hcursor_field_0xc4 = HVar2;
    iVar4.field150_0xc8 = 0x2008;
    iVar4.field139_0xac = 0x44000000;
    iVar4.field145_0xbc = (param_3 + 0x8);
    iVar4.field151_0xca = iVar4.field164_0xde;
    win_ui_reg_class_1008_96d2(param_1);
    return param_1;
}


pub unsafe fn unk_win_ui_op_1020_67ce(
    in_struct_1: *mut StructA,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut HVar1: HGDIOBJ16;
    let mut hcursor_2: HCURSOR16;
    let struct_a_1_lo: *mut StructA;
    let struct_a_1_hi: *mut StructA;
    let mut in_stack_0000fe10: u16;
    let mut in_stack_0000fe14: u16;
    let mut in_stack_0000fe66: u16;
    let mut in_stack_0000ff3a: u16;
    let mut in_stack_0000ff3e: u16;
    let mut in_stack_0000ff42: u16;
    let mut in_stack_0000ff82: u16;
    let mut in_stack_0000ff8a: u16;
    let mut in_stack_0000ff90: u16;
    let mut in_stack_0000ff94: u16;

    struct_1020_790e(
        &in_struct_1.field0_0x0,
        s_TPPOPMENU_1050_43fa,
        param_2,
        param_3,
    );
    struct_a_1_hi = (in_struct_1 >> 0x10);
    struct_a_1_lo = in_struct_1;
    struct_a_1_lo[0x1].field20_0x26 = 0;
    struct_a_1_lo[0x1].field22_0x2a = 0;
    in_struct_1.field0_0x0 = 0x70e6;
    struct_a_1_lo.field1_0x2 = 0x1020;
    unk_str_op_1000_3d3e(
        (in_struct_1 & 0xffff0000 | ZEXT24(&struct_a_1_lo.field60_0x5b)),
        s_VrMode2_1050_4404,
    );
    HVar1 = GetStockObject16(HOLLOW_BRUSH);
    struct_a_1_lo.field157_0xc6 = HVar1;
    hcursor_2 = LoadCursor16(0x7f00, 0x0);
    struct_a_1_lo.field156_0xc4 = hcursor_2;
    struct_a_1_lo.field140_0xac = 0x44c00000;
    struct_a_1_lo.field158_0xc8 = 0x2020;
    struct_a_1_lo.field149_0xbc = (param_3 + 0x8);
    struct_a_1_lo.field159_0xca = param_2;
    win_ui_reg_class_1008_96d2(in_struct_1);
    win_ui::window_op_1020_6c3a(
        param_4,
        in_struct_1,
        in_stack_0000fe66,
        in_stack_0000ff82,
        in_stack_0000ff8a,
        in_stack_0000ff90,
        in_stack_0000ff94,
        in_stack_0000fe10,
        in_stack_0000fe14,
        in_stack_0000ff3a,
        in_stack_0000ff3e,
        in_stack_0000ff42,
    );
    return;
}

pub unsafe fn win_ui_op_1020_737a(mut param_1: u16, param_2: *mut astruct_788) -> BOOL16 {
    let mut uVar8: u16;
    let mut is_iconic: bool;
    let mut puVar3: *mut c_char;
    let mut RVar9: RECT16;
    let mut in_DX: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut uVar10: u32;
    let mut struct_1: *mut astruct_788;
    let mut uVar6: u16;
    let mut local_42: [u8; 0x6] = [0; 0x6];
    let mut local_brush_handle: RECT16;
    let mut iStack56: i16;
    let mut HStack54: HWND16;
    let mut HStack52: HWND16;
    let mut iStack50: i16;
    let mut rect_30: RECT16;
    let mut iStack44: i16;
    let mut iStack42: i16;
    let mut local_rect: HGDIOBJ16;
    let mut hicon_38: bool;
    let mut hdc_24: HDC16;
    let mut local_paint_struct: [u8; 0x20] = [0; 0x20];
    let mut uVar3: *mut astruct_126;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar4: *mut astruct_126;
    let mut uVar7: u8;
    let mut fn_ptr_1: *mut *mut code;

    uVar6 = (param_2 >> 0x10);
    struct_1 = param_2;
    hdc_24 = BeginPaint16(
        CONCAT13(0x10, CONCAT12(0x50, local_paint_struct)),
        struct_1.hwnd_0x4,
    );
    is_iconic = IsIconic16(struct_1.hwnd_0x4);
    if (is_iconic == 0) {
        uVar4 = struct_1.field22_0x1c;
        RVar9 = *(uVar4 + 0x24);
        local_brush_handle = RVar9;
        pass1_1018_2e5e(SUB42(RVar9, 0x0), in_DX, struct_1.field22_0x1c);
        rect_30 = RVar9 & 0xffff | in_DX << 0x10;
        pass1_1008_3e54(CONCAT13(0x10, CONCAT12(0x50, local_42)), 0x0, 0x35, 0xc);
        if (&struct_1.field19_0x14 != 0) {
            pass1_1008_5236(&struct_1.field19_0x14);
        }
        if (rect_30 != 0) {
            uVar1 = struct_1.field19_0x14;
            uVar8 = struct_1.field20_0x16;
            uVar10 = CONCAT22(in_register_0000000a, uVar8);
            if ((uVar8 | uVar1) != 0) {
                pass1_1008_5118(CONCAT22(uVar8, uVar1));
                fn_ptr_1000_17ce(CONCAT22(uVar8, uVar1));
            }
            puVar3 = local_42;
            pass1_1008_8ce4(
                rect_30,
                CONCAT22(0x1050, puVar3),
                local_brush_handle,
                uVar10,
            );
            struct_1.field19_0x14 = puVar3;
            struct_1.field20_0x16 = uVar10;
        }
        fn_ptr_1 = (local_brush_handle + 0x4);
        (**fn_ptr_1)(
            0x1008,
            SUB42(local_brush_handle, 0x0),
            (local_brush_handle >> 0x10),
            0x0,
            0x0,
            &hdc_24,
            &DAT_1050_1050,
        );
        fn_ptr_1 = (*struct_1.field21_0x18 + 0x94);
        (**fn_ptr_1)(0x1008, struct_1.field21_0x18, 1);
        HStack52 = GetDlgItem16(0x1797, struct_1.hwnd_0x4);
        if (HStack52 != 0) {
            ShowWindow16(0x1, HStack52);
        }
    } else if (PTR_LOOP_1050_0010.is_null()) {
        uVar3 = struct_1.field22_0x1c;
        fn_ptr_1 = (struct_1.field22_0x1c + 0x2c);
        (**fn_ptr_1)(s_tile2_bmp_1050_1538, uVar3, (uVar3 >> 0x10));
        hicon_38 = is_iconic;
        if (is_iconic != 0) {
            local_rect = GetStockObject16(BLACK_BRUSH);
            GetClientRect16(&rect_30, &DAT_1050_1050);
            local_brush_handle = 0x0;
            iStack56 = (iStack44 - rect_30.x) - 0x1;
            HStack54 = (iStack42 - rect_30.y) - 0x1;
            HStack52 = HStack54;
            iStack50 = iStack56;
            FillRect16(local_rect, &local_brush_handle, &DAT_1050_1050);
            DrawIcon16(hicon_38, 0x2, 0x2, hdc_24);
        }
    }
    is_iconic = EndPaint16(CONCAT22(0x1050, local_paint_struct), struct_1.hwnd_0x4);
    return is_iconic;
}


pub unsafe fn unk_win_ui_op_1040_9854(param_1: *mut astruct_787) -> *mut u16

{
  let mut HVar1: HCURSOR16;
  let mut HVar2: HGDIOBJ16;
   let mut struct_1: *mut astruct_787;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  struct_1 = param_1;
  param_1.offset = 0x389a;
  struct_1.base = 0x1008;
  param_1.offset = 0xa230;
  struct_1.base = &PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&struct_1.field_0x4)),s_OPButton_1050_5ece);
  struct_1.field82_0x54 = 0x3;
  HVar1 = LoadCursor16(0x7f00,0x0);
  struct_1.field84_0x58 = HVar1;
  HVar2 = GetStockObject16(BLACK_BRUSH);
  struct_1.field83_0x56 = HVar2;
  winapp::reg_class_1040_98c0(param_1 & 0xffff | uVar3 << 0x10);
  return &param_1.offset;
}

pub unsafe fn get_stock_obj_1008_9c56() {
    GetStockObject16(HOLLOW_BRUSH);
    return;
}

pub unsafe fn realize_palette_1020_0e46(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u32;
    let mut puVar4: HGDIOBJ16;
    let mut iVar4: *mut astruct_800;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar2: u32;
    let mut puVar2: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    if (param_2 != 0) {
        uVar4 = (param_1 >> 0x10);
        uVar2 = (param_1 + 0xf2);
        uVar5 = (uVar2 >> 0x10);
        iVar4 = uVar2;
        puVar2 = iVar4.field102_0x66;
        fn_ptr_1 = (*puVar2 + 0x18);
        (**fn_ptr_1)();
        UnrealizeObject16(puVar2);
        uVar1 = (param_1 + 0xf2);
        RealizePalette16(*(uVar1 + 0x7c));
    }
    return;
}

pub unsafe fn realize_palette_1020_2992(mut param_1: u32, mut param_2: i16) {
    let mut obj: HGDIOBJ16;
    let mut hdc: HDC16;
    let mut uVar1: u16;
    let mut puVar1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    if (param_2 != 0) {
        uVar1 = (param_1 >> 0x10);
        puVar1 = pass1_1018_0a50((param_1 + 0xf2));
        fn_ptr_1 = (*puVar1 + 0x18);
        obj = (**fn_ptr_1)(0x1018);
        UnrealizeObject16(obj);
        hdc = GetDC16((param_1 + 0x8));
        RealizePalette16(hdc);
    }
    return;
}

pub unsafe fn realize_palette_1020_6896(mut param_1: u32, mut param_2: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar3: u32;
    let mut puVar4: u32;
    let mut iVar4: *mut astruct_801;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar2: u32;

    if (param_2 != 0) {
        uVar4 = (param_1 >> 0x10);
        uVar2 = (param_1 + 0xf2);
        uVar5 = (uVar2 >> 0x10);
        iVar4 = uVar2;
        puVar4 = iVar4.field36_0x24;
        ppcVar1 = (puVar4 + 0x18);
        (**ppcVar1)();
        UnrealizeObject16(puVar4);
        uVar3 = (param_1 + 0xf2);
        RealizePalette16(*(uVar3 + 0x178));
    }
    return;
}

// WARNING: Unable to use type for symbol uVar3
pub unsafe fn create_palette_1008_4e38(in_struct_1: *mut astruct_13, mut param_2: u16) {
    let mut piVar1: *mut i16;
    let mut pLVar2: *mut LOGPALETTE;
    let mut iVar3: i16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut local_struct_1: *mut astruct_13;
    let mut iVar5: i16;
    let mut uVar8: *mut astruct_13;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut iStack14: i16;
    let mut UpuStack12: *mut c_char;
    let mut UpuStack8: *mut c_char;
    let mut uVar3: *mut LOGPALETTE;

    paVar4 = CONCAT22(in_register_0000000a, param_2);
    uVar8 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    iVar3 = (local_struct_1.field9_0xc + 0x2) * 0x4;
    if (local_struct_1.field10_0xe.is_null()) {
        mem_op_1000_179c(iVar3, paVar4);
        local_struct_1.field10_0xe = iVar3;
        (&local_struct_1.field10_0xe + 0x2) = paVar4;
        local_struct_1.field10_0xe.pal_version = 0x300;
        uVar3 = local_struct_1.field10_0xe;
        (uVar3 + 0x2) = local_struct_1.field9_0xc;
        pLVar2 = local_struct_1.field10_0xe;
        puStack8 = (pLVar2 & 0xffff0000 | (pLVar2 + 0x4));
        puStack12 = local_struct_1.field4_0x4;
        iStack14 = 0;
        loop {
            piVar1 = &local_struct_1.field9_0xc;
            if (*piVar1 == iStack14 || *piVar1 < iStack14) {
                break;
            }
            uVar9 = (puStack12 >> 0x10);
            iVar3 = puStack12;
            *puStack8 = (iVar3 + 2);
            uVar10 = (puStack8 >> 0x10);
            iVar5 = puStack8;
            *(iVar5 + 1) = *(iVar3 + 1);
            (iVar5 + 0x2) = *puStack12;
            *(iVar5 + 0x3) = 0;
            iStack14 += 0x1;
            puStack8 = (puStack8 & 0xffff0000 | (iVar5 + 0x4));
            puStack12 = (puStack12 & 0xffff0000 | (iVar3 + 0x4));
        }
    }
    CreatePalette16(local_struct_1.field10_0xe);
    return;
}

pub unsafe fn begin_end_paint_1008_97c8(param_1: *mut astruct_837, mut param_2: u16) {
    BeginPaint16(CONCAT22(0x1050, &stack0xffde), param_1.field8_0x8);
    EndPaint16(CONCAT22(0x1050, &stack0xffde), param_1.field8_0x8);
    return;
}

pub unsafe fn misc_draw_op_1018_5d6c(param_1: *mut astruct_839) {
    let mut pa_var1: *mut astruct_76;
    let mut struct_4: *mut astruct_839;
    let mut u_var5: u16;
    let mut pa_var2: *mut astruct_76;
    let mut local_22: PAINTSTRUCT16 = PAINTSTRUCT16::default();
    let mut pu_var1: *mut u32;
    let mut u_var4: *mut astruct_134;
    let mut fn_ptr_1: *mut *mut code;

    u_var5 = (param_1 >> 0x10);
    struct_4 = param_1;
    BeginPaint16(CONCAT22(0x1050, &local_22), struct_4.field4_0x4);
    u_var4 = struct_4.pstruct134_0x14;
    pa_var1 = (u_var4 + 0xa);
    pa_var2 = pass1_1008_9f48(struct_4.pstruct134_0x14);
    pass1_1008_5236(struct_4.field20_0x18);
    pass1_1008_4480(
        pa_var1,
        (param_1 & 0xffff0000 | ZEXT24(struct_4 + 1)),
        pa_var2,
    );
    fn_ptr_1 = (pa_var1 + 0x4);
    (**fn_ptr_1)(
        0x1008,
        pa_var1,
        (pa_var1 >> 0x10),
        0x0,
        param_1 & 0xffff0000 | ZEXT24(&struct_4.field_0xa),
    );
    EndPaint16(CONCAT22(0x1050, &local_22), struct_4.field4_0x4);
    return;
}

pub unsafe fn unk_draw_op_1020_0000(param_1: *mut astruct_840) {
    let mut piVar1: *mut i16;
    let mut ppcVar2: *mut *mut code;
    let mut iVar4: *mut astruct_840;
    let mut uVar5: u16;
    let mut uVar4: u16;
    let mut local_c4: [u8; 0x6] = [0; 0x6];
    let mut local_be: *mut i16;
    let mut piStack184: *mut i16;
    let mut uStack182: u16;
    let mut local_b4: i16;
    let mut iStack178: i16;
    let mut aiStack176: [i16; 0x3c] = [0; 0x3c];
    let mut iStack56: *mut astruct_841;
    let mut iStack48: i16;
    let mut paStack46: *mut astruct_76;
    let mut local_2a: i16;
    let mut local_28: i16;
    let mut puStack38: *mut u32;
    let mut local_22: [u8; 0x20] = [0; 0x20];
    let mut uVar3: u32;
    let mut uVar2: u32;
    let mut iVar3: *mut astruct_841;

    // Segment:    5
    // Offset:     00033420
    // Length:     efba
    // Min Alloc:  efba
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(0x1050, local_22), iVar4.field4_0x4);
    uVar3 = iVar4.field19_0x14;
    puStack38 = (uVar3 + 0xa);
    pass1_1008_3e94(
        (param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x18)),
        CONCAT22(0x1050, &local_2a),
        CONCAT22(0x1050, &local_28),
    );
    uVar4 = 0x1008;
    pass1_1008_4480(
        puStack38,
        (param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x18)),
        iVar4.field32_0x24,
    );
    paStack46 = null_mut();
    for iStack48 in 0..0x6 {
        uVar2 = iVar4.field19_0x14;
        uVar4 = 0x1010;
        pass1_1010_2b78(
            uVar2,
            (uVar2 >> 0x10),
            iStack48,
            CONCAT22(0x1050, &local_b4),
        );
        if (local_b4 == 0) {
            //   for (iStack56 = null_mut(); iVar3 = iStack56, iStack56 <= iStack178; iStack56 = iStack56 + 1)
            iStack56 = null_mut();
            iVar3 = iStack56;
            while iStack56 <= iStack178 {
                piVar1 = aiStack176 + iStack56 * 0x3;
                uStack182 = &DAT_1050_1050;
                piStack184 = piVar1;
                if (aiStack176[iStack56 * 0x3 + 0x2] != 0) {
                    paStack46 = pass1_1010_2b98(iVar4.field19_0x14, aiStack176[iStack56 * 0x3 + 0x2]);
                    pass1_1008_3e54(
                        CONCAT22(0x1050, &local_be),
                        0x0,
                        aiStack176[iVar3 * 0x3 + 0x1] + local_2a,
                        *piVar1 + local_28,
                    );
                    uVar4 = 0x1008;
                    pass1_1008_4480(puStack38, CONCAT22(0x1050, &local_be), paStack46);
                }
                iStack56 += 1;
            }
        } else {
            local_be = CONCAT22(0x1050, aiStack176 + iStack178 * 0x3);
            if (aiStack176[iStack178 * 0x3 + 0x2] != 0) {
                paStack46 = pass1_1010_2b98(iVar4.field19_0x14, aiStack176[iStack178 * 0x3 + 0x2]);
                pass1_1008_3e54(
                    CONCAT22(0x1050, local_c4),
                    0x0,
                    (local_be + 0x2) + local_2a,
                    *local_be + local_28,
                );
                uVar4 = 0x1008;
                pass1_1008_4480(puStack38, CONCAT22(0x1050, local_c4), paStack46);
            }
        }
    }
    ppcVar2 = (*puStack38 + 0x4);
    (**ppcVar2)(
        uVar4,
        puStack38,
        (puStack38 >> 0x10),
        0x0,
        0x0,
        &iVar4.field_0xa,
        uVar5,
    );
    EndPaint16(CONCAT22(0x1050, local_22), iVar4.field4_0x4);
    return;
}

pub unsafe fn mix_draw_op_1020_650c(param_1: *mut StructA) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let struct_1: *mut StructA;
    let mut uVar3: u16;
    let mut local_28: [u8; 0x20] = [0; 0x20];
    let mut iStack8: i16;
    let mut puStack6: *mut u32;

    uVar3 = (param_1 >> 0x10);
    struct_1 = param_1;
    uVar2 = &struct_1.field10_0x14;
    puStack6 = (uVar2 + 0xa);
    if ((struct_1.field8_0x10 != 0) || (uVar2 = &struct_1.field10_0x14, (uVar2 + 0x24) != 0)) {
        draw_op_1020_9364(param_1);
        if (&struct_1.field19_0x24 != 0) {
            uVar2 = &struct_1.field19_0x24;
            ppcVar1 = (*&struct_1.field19_0x24 + 0x14);
            (**ppcVar1)(0x1020, uVar2, (uVar2 >> 0x10));
        }
    }
    iStack8 = 0;
    loop {
        if ((&struct_1.field12_0x18 + iStack8 * 0x2) != 0) {
            uVar2 = (&struct_1.field12_0x18 + iStack8 * 0x2);
            ppcVar1 = (*(&struct_1.field12_0x18 + iStack8 * 0x2) + 0x8);
            (**ppcVar1)(0x1020, uVar2, (uVar2 >> 0x10), puStack6, (puStack6 >> 0x10));
        }
        iStack8 += 0x1;
        if iStack8 >= 0x5 {
            break;
        }
    }
    BeginPaint16(CONCAT22(0x1050, local_28), struct_1.field2_0x4);
    ppcVar1 = (*puStack6 + 0x4);
    (**ppcVar1)(
        s_tile2_bmp_1050_1538,
        puStack6,
        (puStack6 >> 0x10),
        0x0,
        0x0,
        &struct_1.field5_0xa,
        uVar3,
    );
    EndPaint16(CONCAT22(0x1050, local_28), struct_1.field2_0x4);
    return;
}

pub unsafe fn mix_draw_op_1020_9312(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut local_22: [u8; 0x20] = [0; 0x20];

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(0x1050, local_22), (iVar4 + 0x4));
    uVar3 = (iVar4 + 0x6);
    puVar1 = (uVar3 + 0xa);
    ppcVar2 = (*puVar1 + 0x4);
    (**ppcVar2)(
        s_tile2_bmp_1050_1538,
        puVar1,
        (puVar1 >> 0x10),
        0x0,
        param_1 & 0xffff0000 | (iVar4 + 0xa),
    );
    EndPaint16(CONCAT22(0x1050, local_22), (iVar4 + 0x4));
    return;
}

pub unsafe fn get_dc_1018_4db0(param_1: *mut astruct_126, mut param_2: u16) {
    let mut HVar1: HDC16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    (param_1 + 0x16) = param_2;
    HVar1 = GetDC16(param_2);
    *(param_1 + 0x14) = HVar1;
    return;
}

// WARNING: Unable to use type for symbol uVar4
pub unsafe fn win_ui_palette_op_1020_0cd2(struct_param_1: *mut astruct_775) {
    let mut puVar2: *mut u32;
    let mut ppcVar3: *mut *mut code;
    let mut uVar7: u16;
    let mut hdc: HDC16;
    let mut hpal: HDC16;
    let mut hpal_00: HPALETTE16;
    let mut UVar4: u16;
    let mut extraout_DX: u16;
    let mut uVar8: u16;
    let mut struct_1: *mut astruct_775;
    let mut iVar5: *mut astruct_774;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut hdc_00: HDC16;
    let mut paStack10: *mut astruct_13;
    let mut uStack6: u16;
    let mut puVar1: *mut u32;
    let mut uVar4: u32;

    uVar5 = (struct_param_1 >> 0x10);
    struct_1 = struct_param_1;
    uVar4 = struct_1.field5_0x6;
    uVar6 = (uVar4 >> 0x10);
    iVar5 = uVar4;
    puVar2 = &iVar5.field_0xa;
    uStack6 = puVar2;
    uVar7 = iVar5.field12_0xc | uStack6;
    if (uVar7 != 0) {
        ppcVar3 = (*puVar2 + 0x14);
        (**ppcVar3)();
        paStack10 = CONCAT22(extraout_DX, uVar7);
        uVar8 = extraout_DX | uVar7;
        if (uVar8 != 0) {
            hdc = GetDC16(struct_1.field4_0x4);
            hpal = hdc;
            hdc_00 = hdc;
            create_palette_1008_4e38(paStack10, uVar8);
            hpal_00 = SelectPalette16(0x0, hpal, hdc_00);
            UVar4 = RealizePalette16(hdc);
            SelectPalette16(0x1, hpal_00, hdc);
            DeleteObject16(hpal);
            if (0x0 < UVar4) {
                InvalidateRect16(0x1, NULL, 0x0);
            }
            ReleaseDC16(hdc, struct_1.field4_0x4);
            return;
        }
    }
    return;
}

pub unsafe fn unk_win_ui_op_1020_717e(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_40,
    param_4: *mut StructA,
) {
    let mut paVar1: *mut astruct_13;
    let mut ppcVar2: *mut *mut code;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut HVar6: HPALETTE16;
    let mut puVar6: *mut u32;
    let mut uVar6: u16;
    let mut uVar9: u16;
    let mut puVar7: *mut u8;
    let mut puVar10: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar11: *mut Struct57;
    let mut iVar7: *mut astruct_40;
    let mut iVar8: *mut astruct_933;
    let mut uVar7: *mut astruct_40;
    let mut uVar8: u16;
    let mut puVar12: *mut u32;
    let mut in_stack_0000fe78: u16;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000ff9c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut local_4: HDC16;
    let mut uVar3: u32;
    let mut iVar9: *mut astruct_41;
    let mut in_stack_0000ffdc: u16;

    paVar11 = CONCAT22(in_register_0000000a, param_1);
    win_ui::get_sys_metrics_1020_7c1a(param_3, param_4);
    uVar7 = (param_3 >> 0x10);
    iVar7 = param_3;
    iVar7.field17_0x14 = 0;
    iVar7.field20_0x18 = param_4;
    iVar7.field21_0x1c = 0;
    (&iVar7[0x1].field0_0x0 + 1) = 0;
    param_3.field0_0x0 = 0x754c;
    iVar7.field1_0x2 = 0x1020;
    puVar12 = mixed_1010_20ba(
        paVar11,
        _u16_1050_0ed0,
        CONCAT22(param_2, 0x4),
        in_stack_0000fe84,
        in_stack_0000ffa8,
        in_stack_0000ffae,
        in_stack_0000ffb2,
    );
    uVar5 = (paVar11 >> 0x10);
    iVar7.field21_0x1c = puVar12;
    uVar9 = (puVar12 >> 0x10);
    iVar7.field_0x1e = uVar9;
    ppcVar2 = (*&iVar7.field21_0x1c + 0x4);
    (**ppcVar2)(0x1010, iVar7.field21_0x1c, uVar9, 0x0, param_3);
    local_4 = GetDC16(iVar7.hwnd_0x4);
    uVar3 = &iVar7.field21_0x1c;
    *(uVar3 + 0x178) = local_4;
    uVar4 = &iVar7.field21_0x1c;
    uVar8 = (uVar4 >> 0x10);
    iVar8 = uVar4;
    puVar6 = iVar8.field36_0x24;
    uVar9 = (&iVar8.field36_0x24 + 2);
    paVar11 = CONCAT22(uVar5, uVar9);
    uVar5 = SUB42(puVar6, 0x0);
    ppcVar2 = (*puVar6 + 0x14);
    (**ppcVar2)(0x38, uVar5, uVar9);
    puVar12 = mixed_1010_20ba(
        paVar11,
        _u16_1050_0ed0,
        CONCAT22(uVar5, 0x29),
        in_stack_0000fe78,
        in_stack_0000ff9c,
        in_stack_0000ffa2,
        in_stack_0000ffa6,
    );
    puVar10 = (puVar12 >> 0x10);
    paVar1 = (puVar12 + 0xe);
    pass1_1008_4d84(puVar10, (puVar6 & 0xffff | paVar11 << 0x10), paVar1);
    HVar6 = palette_op_1008_4e08(
        &local_4,
        puVar10,
        paVar1,
        CONCAT13(0x10, CONCAT12(0x50, &local_4)),
    );
    (&iVar7[0x1].field0_0x0 + 1) = HVar6;
    return;
}


pub unsafe fn set_struct_op_1020_921c(
    mut param_1: u16,
    pstructa_param_2: *mut StructA,
    mut param_3: u16,
    param_4: *mut *mut u8,
) {
    let mut uVar1: u16;
    let mut HVar2: HDC16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let iVar3: *mut StructA;
    let uVar3: *mut StructA;
    let mut pUVar3: *mut u16;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffe2: u32;
    let mut uVar2: u16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    uVar3 = (pstructa_param_2 >> 0x10);
    iVar3 = pstructa_param_2;
    pstructa_param_2.field0_0x0 = 0x389a;
    iVar3.field1_0x2 = 0x1008;
    pstructa_param_2.field0_0x0 = 0x3aa8;
    iVar3.field1_0x2 = 0x1008;
    iVar3.field2_0x4 = param_3;
    pstructa_param_2.field0_0x0 = 0x3ab0;
    iVar3.field1_0x2 = 0x1008;
    iVar3.field3_0x6 = 0;
    iVar3.field5_0xa = 0;
    iVar3.field6_0xc = 0;
    iVar3.field7_0xe = 0;
    iVar3.field8_0x10 = 0;
    iVar3.field9_0x12 = 0;
    pstructa_param_2.field0_0x0 = 0x96c8;
    iVar3.field1_0x2 = 0x1020;
    HVar2 = GetDC16(iVar3.field2_0x4);
    iVar3.field5_0xa = HVar2;
    param_4 = CONCAT22(param_4, 0x48);
    pUVar3 = mixed_1010_20ba(
        paVar3,
        _u16_1050_0ed0,
        param_4,
        in_stack_0000fe8c,
        in_stack_0000ffb0,
        in_stack_0000ffb6,
        in_stack_0000ffba,
    );
    uVar1 = (pUVar3 >> 0x10);
    iVar3.field6_0xc = (pUVar3 + 0xa);
    iVar3.field7_0xe = (pUVar3 + 0xc);
    return;
}

pub unsafe fn get_sys_metrics_1040_8c66(param_1: *mut Struct37)

{
  let mut piVar1: *mut i16;
  let mut bVar2: u8;
  let mut HVar3: HDC16;
  let mut IVar4: i16;
   let mut struct_1: *mut Struct37;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  struct_1 = param_1;
  HVar3 = GetDC16((&struct_1.field1_0x4 + 0x2));
  draw_text_1040_8d14(param_1, HVar3);
  struct_1[0x1].field1_0x4 = *&struct_1.field144_0x9e;
  &struct_1[0x1].field_0x8 = (struct_1 + 1).field0_0x0;
  IVar4 = GetSystemMetrics16(SM_CYCAPTION);
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + IVar4;
  bVar2 = struct_1.field138_0x98 & 0xf0;
  if ((((bVar2 == 0x30) || (bVar2 == 0x10)) || (bVar2 == 0x40)) || (bVar2 == 0x20)) {
    IVar4 = GetSystemMetrics16(SM_CYICON);
    if (&struct_1[0x1].field_0xa < IVar4) {
      IVar4 = GetSystemMetrics16(SM_CYICON);
      struct_1[0x1].field_0xa = IVar4;
    }
  }
  piVar1 = &struct_1[0x1].field_0x8;
  *piVar1 = *piVar1 + 0x14;
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + 0xa;
  struct_1[0x1].field_0xe = &struct_1[0x1].field_0xa;
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + 0x30;
  HVar3 = *(&struct_1.field1_0x4 + 2);
  ReleaseDC16(HVar3,HVar3);
  return;
}


pub unsafe fn call_fn_ptr_1038_9ffa(mut param_1: u32, pstruct_param_2: *mut astruct_733, mut param_3: u16 ) -> u16

{
  let mut ppcVar1: *mut *mut code;
   let mut struct_3: *mut astruct_43;
   let mut struct_2: *mut astruct_43;
  let mut puStack8: *mut u32;
  let mut hdc: HDC16;
  let mut var_5: u16;

  hdc = GetDC16(pstruct_param_2.hwnd_0x6);
  struct_2 = FUN_1010_830a(hdc,param_1,s_tile2_bmp_1050_1538,_u16_1050_14cc,0x3);
  puStack8 = CONCAT22(param_1,struct_2);
  struct_3 = *puStack8;
  ppcVar1 = &struct_3.fn_ptr_field_0x8;
  (**ppcVar1)(0x1010,struct_2,param_1,&hdc,&DAT_1050_1050);
  ppcVar1 = &struct_3.fn_ptr_field_0x4;
  (**ppcVar1)(0x1010,puStack8,0x50005,&hdc,&DAT_1050_1050);
  ppcVar1 = &struct_3.fn_ptr_field_0xc;
  (**ppcVar1)(0x1010,puStack8,&hdc,&DAT_1050_1050);
  ReleaseDC16(hdc,pstruct_param_2.hwnd_0x6);
  return 0x0;
}


pub unsafe fn get_dc_op_1040_3d5e(param_1: *mut astruct_1) -> u16

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut in_EDX: u32;
    let mut iVar4: *mut astruct_1;
    let mut iVar3: *mut astruct_934;
    let mut uVar3: u16;
    let mut puStack8: *mut u32;
    let mut local_4: HDC16;

    uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    local_4 = GetDC16(iVar4.field6_0x6);
    uVar2 = FUN_1010_830a(local_4, in_EDX, s_tile2_bmp_1050_1538, _u16_1050_14cc, iVar4.field163_0xa4);
    puStack8 = CONCAT22(in_EDX, uVar2);
    iVar3 = *puStack8;
    ppcVar1 = &iVar3.field6_0x8;
    (**ppcVar1)(0x1010, uVar2, in_EDX, &local_4, &DAT_1050_1050);
    ppcVar1 = &iVar3.field4_0x4;
    (**ppcVar1)(0x1010, puStack8, 0x50078, &local_4, &DAT_1050_1050);
    ppcVar1 = &iVar3.field8_0xc;
    (**ppcVar1)(0x1010, puStack8, &local_4, &DAT_1050_1050);
    ReleaseDC16(local_4, iVar4.field6_0x6);
    return 0x0;
}


pub unsafe fn invalidate_rect_1020_8d90(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: u16,
    mut param_7: u16,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut local_48: RECT16;
    let mut iStack68: i16;
    let mut iStack66: i16;
    let mut local_40: i16;
    let mut local_3e: i16;
    let mut uStack60: u32;
    let mut local_38: astruct_288 = astruct_288::default();
    let mut local_10: [u8; 0xa] = [0; 0xa];
    let mut uStack6: u16;
    let mut uStack4: u16;

    uVar5 = (param_3 >> 0x10);
    iVar4 = param_3;
    uVar2 = pass1_1018_266a((iVar4 + 0x22));
    if (uVar2 != 0) {
        uVar6 = pass1_1018_265c();
        uStack4 = (uVar6 >> 0x10);
        uStack6 = uVar6;
        uVar3 = CONCAT22(in_register_0000000a, uStack4 | uStack6);
        if ((uStack4 | uStack6) != 0) {
            sys_1000_3f9c(CONCAT22(0x1050, local_10), s__03ld_1050_442a, uStack6);
            uVar1 = (iVar4 + 0x22);
            file_and_draw_op_1008_4f20(
                uVar3,
                CONCAT22(0x1050, &local_38),
                (uVar1 + 0xe),
                0x25,
                CONCAT22(0x1050, local_10),
                param_6,
                param_7,
            );
            pass1_1008_4480(
                param_5,
                (param_3 & 0xffff0000 | (iVar4 + 0x1c)),
                CONCAT22(0x1050, &local_38),
            );
            uStack60 = pass1_1008_4772(CONCAT22(0x1050, &local_38));
            pass1_1008_3e94(
                (param_3 & 0xffff0000 | (iVar4 + 0x1c)),
                CONCAT22(0x1050, &local_40),
                CONCAT22(0x1050, &local_3e),
            );
            local_48.x = local_3e;
            local_48.y = local_40;
            uVar5 = (uStack60 >> 0x10);
            iStack68 = local_3e + (uStack60 + 0x4);
            iStack66 = local_40 + (uStack60 + 0x8);
            // just 0x0
            InvalidateRect16(0x0, &local_48, &DAT_1050_1050);
            pass1_1008_41bc(CONCAT22(0x1050, &local_38));
        }
    }
    return;
}


pub unsafe fn invalidate_rect_1040_3ddc(in_struct_1: *mut StructC)

{
  let mut rect: RECT16;
  let mut uStack6: u32;

  rect = 0x780005;
  uStack6 = 0xdc0069;
  InvalidateRect16(0x0,&rect,&DAT_1050_1050);
  return;
}


pub unsafe fn invalidate_rect_1018_5d32(mut param_1: u32, mut param_2: i16) {
    let mut hwnd: HWND16;

    hwnd = (param_1 >> 0x10);
    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0x2) {
        return;
    }
    InvalidateRect16(0x0, (param_1 + 0x22), hwnd);
    return;
}

pub unsafe fn invalidate_rect_1018_edd8(mut param_1: u32, mut param_2: i16) {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut local_16: RECT16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut uStack14: u32;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut local_6: i16;
    let mut local_4: i16;

    iVar1 = param_1;
    uVar2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (iVar1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0xc) {
        return;
    }
    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (iVar1 + 0x18)),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    uVar3 = pass1_1010_2b66((iVar1 + 0x14));
    uStack8 = (uVar3 >> 0x10);
    uStack10 = uVar3;
    uStack14 = pass1_1008_4772((uVar3 & 0xffff | uStack8 << 0x10));
    uVar2 = (uStack14 >> 0x10);
    local_16.x = local_4;
    local_16.y = local_6;
    iStack18 = local_4 + (uStack14 + 0x4);
    iStack16 = local_6 + (uStack14 + 0x8);
    InvalidateRect16(0x0, &local_16, &DAT_1050_1050);
    return;
}

pub unsafe fn invalidate_rect_1020_3080(mut param_1: u32, mut param_2: i16) {
    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if ((param_2 != 0x4) && (param_2 != 0x6)) {
        return;
    }
    InvalidateRect16(0x0, NULL, 0x0);
    return;
}

pub unsafe fn invalidate_rect_1020_8fb4(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u32;
    let mut rect: *mut RECT16;
    let mut uVar2: u32;
    let mut extraout_DX: u16;
    let mut hwnd: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut iStack8: i16;

    uVar4 = (param_3 >> 0x10);
    iVar3 = param_3;
    uVar1 = (iVar3 + 0xba);
    if ((uVar1 + 0x1e) != 0) {
        pass1_1018_2862((iVar3 + 0x16));
        (iVar3 + 0xaa) = param_1;
        (iVar3 + 0xac) = param_2;
        if ((param_2 | (iVar3 + 0xaa)) != 0) {
            uVar1 = (iVar3 + 0xaa);
            iVar3 = (uVar1 + 0xa);
            for iStack8 in 0..iVar3 {
                uVar2 = iStack8;
                empty_1008_8fc4();
                rect = uVar2;
                hwnd = extraout_DX | rect;
                if (((hwnd != 0) && (0x9 < rect[0xb].y)) && (
                    pass1_1008_8b20(uVar2 & 0xffff | extraout_DX << 0x10),
                    (hwnd | rect) != 0,
                )) {
                    InvalidateRect16(0x0, rect, hwnd);
                }
            }
        }
    }
    return;
}

pub unsafe fn unk_draw_op_1008_80ee(param_1: *mut astruct_23) -> *mut astruct_23 {
    let mut HVar1: HCURSOR16;
    let mut HVar2: HGDIOBJ16;
    let mut iVar3: *mut astruct_23;
    let mut uVar3: *mut astruct_23;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar3.field1_0x2 = 0x1008;
    iVar3.field79_0x54 = 0;
    iVar3.field80_0x56 = 0;
    iVar3.field81_0x58 = 0;
    param_1.field0_0x0 = 0x87c8;
    iVar3.field1_0x2 = 0x1008;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar3.field2_0x4)),
        s_MicroSpinControl_1050_0370,
    );
    iVar3.field79_0x54 = 0x3;
    HVar1 = LoadCursor16(0x7f00, 0x0);
    iVar3.field81_0x58 = HVar1;
    HVar2 = GetStockObject16(0x4);
    iVar3.field80_0x56 = HVar2;
    winapp::pass1_1008_818c((param_1 & 0xffff | ZEXT24(uVar3) << 0x10));
    return param_1;
}


pub unsafe fn set_sys_color_1008_357e(param_1: *mut astruct_53, mut param_2: i16, mut param_3: u32) {
    let mut uVar1: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar3: *mut astruct_53;
    let mut iVar4: i16;
    let mut uVar6: *mut astruct_53;
    let mut uVar5: u16;
    let mut CVar6: COLORREF;
    let mut uVar7: u32;
    let mut iStack132: i16;
    let mut local_80: u32;
    let mut uStack124: u16;
    let mut uStack122: u16;
    let mut uStack120: u16;
    let mut uStack118: u16;
    let mut uStack116: u16;
    let mut uStack114: u16;
    let mut uStack112: u32;
    let mut uStack108: u32;
    let mut uStack104: u16;
    let mut uStack102: u16;
    let mut uStack100: u16;
    let mut uStack98: u16;
    let mut uStack96: u16;
    let mut uStack94: u16;
    let mut uStack92: u16;
    let mut uStack90: u16;
    let mut uStack88: u32;
    let mut uStack84: u32;
    let mut uStack80: u16;
    let mut uStack78: u16;
    let mut uStack76: u32;
    let mut uStack72: u32;
    let mut uStack68: u32;
    let mut uStack64: u32;
    let mut uStack60: u32;
    let mut uStack56: u32;
    let mut uStack52: u32;
    let mut uStack48: u32;
    let mut local_2c: u32;
    let mut uStack40: u32;
    let mut uStack36: u32;
    let mut uStack32: u32;
    let mut uStack28: u32;
    let mut uStack24: u32;
    let mut uStack20: u32;
    let mut uStack16: u32;
    let mut uStack12: u32;
    let mut uStack8: u32;
    let mut uStack4: u16;

    local_2c = 0x70004;
    uStack40 = 0xf0000;
    uStack36 = 0x100014;
    uStack32 = 0xd0012;
    uStack28 = 0x6000e;
    uStack24 = 0x80005;
    uStack20 = 0x10011;
    uStack16 = 0x30002;
    uStack12 = 0xa0009;
    uStack8 = 0xc000b;
    uStack4 = 0x13;
    local_80 = 0;
    uStack108 = 0x808080;
    paVar2 = CONCAT22((param_3 >> 0x10), 0x100);
    uStack116 = 0;
    uStack114 = 0x100;
    uStack100 = 0;
    uStack98 = 0x100;
    uStack96 = 0xffff;
    uStack94 = 0;
    uStack124 = 0x2;
    uStack122 = 0x100;
    uStack120 = 0x2;
    uStack118 = 0x100;
    uStack104 = 0x2;
    uStack102 = 0x100;
    uStack92 = 0x2;
    uStack90 = 0x100;
    uStack88 = 0;
    uStack80 = 0xc0c0;
    uStack78 = 0;
    uStack76 = 0;
    uStack72 = 0;
    uStack68 = 0;
    uStack52 = 0;
    uVar1 = 0x8000;
    uStack112 = 0x8000;
    uStack84 = 0x8000;
    uStack64 = 0x8000;
    uStack60 = 0x8000;
    uStack56 = 0x8000;
    uStack48 = 0x8000;
    uVar6 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (&iVar3.field248_0xf8 == 0) {
        mem_op_1000_179c(0x54, paVar2);
        iVar3.field248_0xf8 = uVar1;
        iVar3.field249_0xfa = paVar2;
        // for (iStack132 = 0; iStack132 < 0x15; iStack132 += 1)
        for iStack132 in 0..0x15 {
            CVar6 = GetSysColor16((&local_2c + iStack132 * 0x2));
            uVar7 = &iVar3.field248_0xf8;
            uVar5 = (uVar7 >> 0x10);
            iVar4 = uVar7;
            (iVar4 + iStack132 * 0x4) = CVar6;
            (iVar4 + iStack132 * 0x4 + 0x2) = (CVar6 >> 0x10);
        }
    }
    if (param_2 != 0) {
        CVar6 = GetSysColor16(local_2c);
        if ((CVar6 == local_80) && ((CVar6 >> 0x10) == local_80)) {
            return;
        }
    }
    if (PTR_LOOP_1050_0010.is_null()) {
        uStack112 = 0xc0c0c0;
    }
    if (param_2 == 0) {
        uVar7 = &iVar3.field248_0xf8;
    } else {
        uVar7 = CONCAT22(0x1050, &local_80);
    }
    SetSysColors16(uVar7, (uVar7 >> 0x10), &local_2c);
    return;
}
