use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::draw_ops::draw_a;
use crate::structs::struct_57::Struct57;
use crate::unk::block_1008_4000::pass1_1008_4d72;
use crate::utils::{CONCAT11, CONCAT22};
use crate::winapi16::{DeleteObject16, GetStockObject16, InvalidateRect16, Rectangle16, SelectObject16, SelectPalette16, SetBkColor16, SetTextColor16};
use crate::windef16::{COLORREF, HDC16, HGDIOBJ16, HPALETTE16, HPEN16, HWND16, RECT16};

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
        uVar1 = (**fn_ptr_1)(0x1538, param_1, (param_1 + 0x6e));
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
    hpalette_7 = draw_a::palette_op_1008_4e08(&hdc_24, in_DX, paVar1, CONCAT22(0x1050, &hdc_24));
    uVar2 = iVar10.field141_0x8e;
    ppcVar2 = (*iVar10.field141_0x8e + 0x4);
    (**ppcVar2)(0x1008, uVar2, (uVar2 >> 0x10), 0xffff, 0xffff, &hdc_24, 0x1050);
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
    HVar7 = draw_a::palette_op_1008_4e08(&hdc_local_24, param_1, paVar1, CONCAT22(0x1050, &hdc_local_24));
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
        uVar7 = SUB42(0x1538, 0x0);
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
        0x1538,
        puVar2,
        (puVar2 >> 0x10),
        pHVar4,
        0x1050,
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
                    0x1538,
                    puVar8,
                    (puVar8 >> 0x10),
                    puVar16,
                    0x1050,
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
    InvalidateRect16(0x0, &local_a, 0x1050);
    local_a = 0xf10000;
    uStack6 = 0x1220030;
    ValidateRect16(&local_a, 0x1050);
    local_a = 0xf100f5;
    uStack6 = 0x1220127;
    ValidateRect16(&local_a, 0x1050);
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
    GetClientRect16((&param_2 + 0x2), 0x1050);
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
        param_2 = 0x1538;
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
    Polygon16(0x3, &local_10, 0x1050);
    param_2 = 0x31538;
    hgdiobj16_var1 = 0x847c;
    Polygon16(0x3, &point_1c, 0x1050);
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
