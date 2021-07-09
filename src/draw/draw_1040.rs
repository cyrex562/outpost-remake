
pub fn mix_draw_op_1040_21d6(param_1: u32,param_2: HWND16,param_3: u16)
{
  let uVar1: u8;
  let uVar2: u8;
  let paVar3: &mut Struct13;
  let ppcVar4: u32;
  let iVar5: i16;
  let b_force_background: HPALETTE16;
  COLORREF color;
  COLORREF color_00;
  HANDLE16 handle;
  let in_DX: u16;
  let iVar6: i16;
  let rect: *mut RECT16;
  let uVar7: u32;
  let uVar8: u16;
  let HStack62: HGDIOBJ16;
  let local_24: HDC16;
  let local_22: PAINTSTRUCT16;
  
  rect = (param_1 >> 0x10);
  iVar6 = param_1;
  uVar8 = (iVar6 + 0x6);
  local_24 = BeginPaint16(param_2,&local_22);
  paVar3 = (ctx.PTR__LOOP_1050_4230 + 0xe);
  b_force_background = palette_op_1008_4e08(paVar3,&local_24,in_DX,0x1008);
  ppcVar4 = ((iVar6 + 0x8e) + 0x4);
  (**ppcVar4)(0x1008,(iVar6 + 0x8e),0xffffffff,&local_24,param_3,uVar8);
  uVar7 = pass1_1008_4d72(paVar3);
  uVar8 = (uVar7 >> 0x10);
  iVar5 = uVar7;
  uVar1 = *(iVar5 + 0x3e5);
  uVar2 = *(iVar5 + 0x3e6);
  color = SetBkColor16(0x1008,0x0);
  color_00 = SetTextColor16(ctx.s_tile2_bmp_1050_1538,CONCAT11(uVar1,uVar2));
  HStack62 = 0x0;
  handle = GetProp16(s_tile2_bmp_1050_1538,0x5ced);
  if (handle != 0x0) {
    HStack62 = SelectObject16(ctx.s_tile2_bmp_1050_1538,handle);
  }
  DrawText16(ctx.s_tile2_bmp_1050_1538,&ctx.PTR_LOOP_1050_0010,iVar6 + 0x92,rect,
             0xffff);
  SetTextColor16(ctx.s_tile2_bmp_1050_1538,
                 CONCAT11(*(iVar5 + 0x95),*(iVar5 + 0x96)));
  DrawText16(ctx.s_tile2_bmp_1050_1538,&ctx.PTR_LOOP_1050_0010,iVar6 + 0x9a,rect,
             0xffff);
  if (handle != 0x0) {
    SelectObject16(ctx.s_tile2_bmp_1050_1538,HStack62);
  }
  SetBkColor16(ctx.s_tile2_bmp_1050_1538,color);
  SetTextColor16(ctx.s_tile2_bmp_1050_1538,color_00);
  SelectPalette16(ctx.s_tile2_bmp_1050_1538,0x0,b_force_background);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  EndPaint16(s_tile2_bmp_1050_1538,&local_22);
  return;
}


pub fn draw_ui_op_1040_27cc(param_1: *mut u32,param_2: u16,param_3: u16,COLORREF param_4) -> u32

{
  let ppcVar1: u32;
  let uVar2: u16;
  let iVar3: i16;
  let HVar4: HBRUSH16;
  let IVar5: i16;
  let iVar6: i16;
  let uVar7: u16;
  COLORREF CVar8;
  let hdc: HWND16;
  let uVar9: u32;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  CVar8 = param_4;
  if ((iVar6 + 0x4) == 0x0) {
    CVar8 = s_tile2_bmp_1050_1538;
    HVar4 = CreateSolidBrush16(param_4);
    *(HBRUSH16 *)(iVar6 + 0x4) = HVar4;
  }
  if (ctx.PTR__LOOP_1050_5cf8 == 0x0) {
    ppcVar1 = (*param_1 + 0x68);
    uVar9 = (**ppcVar1)(CVar8,param_1,(iVar6 + 0x6e));
    CVar8 = 0x1008;
    uVar9 = pass1_1008_4d72(uVar9);
    uVar2 = (uVar9 >> 0x10);
    iVar3 = uVar9;
    ctx._PTR_LOOP_1050_5cf8 =
         CONCAT22(CONCAT11(0x2,*(iVar3 + 0x94)),
                  CONCAT11(*(iVar3 + 0x95),*(iVar3 + 0x96)));
  }
  hdc = CVar8;
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return 0x0;
    }
    hdc = ctx.s_tile2_bmp_1050_1538;
    IVar5 = GetDlgCtrlID16(CVar8);
    if (((iVar6 + 0x94) != 0x0) && (IVar5 == 0xfb2)) {
      CVar8 = 0x0;
//       TODO: goto LAB_1040_286e;
    }
  }
  CVar8 = _PTR_LOOP_1050_5cf8;
//LAB_1040_286e:
  SetTextColor16(hdc,CVar8);
  SetBkColor16(ctx.s_tile2_bmp_1050_1538,0x0);
  return CONCAT22(0x1050,(iVar6 + 0x4));
}


pub fn draw_op_1040_5a06(param_1: u32,param_2: HWND16,param_3: u16)
{
  let puVar1: *mut u16;
  let uVar2: u32;
  let ppcVar3: u32;
  let uVar4: u32;
  let b_force_background: HPALETTE16;
  let iVar5: i16;
  let handle: HPEN16;
  let handle_00: HGDIOBJ16;
  let x: i16;
  let y: i16;
  let in_DX: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let IVar9: i16;
  let uVar10: u32;
  let paVar11: &mut Struct43;
  let paVar12: &mut Struct76;
  let uVar13: u16;
  HDC16 *pHVar14;
  let uVar15: u16;
  let HVar16: HDC16;
  let HVar17: HDC16;
  let HVar18: HDC16;
  let uVar19: u16;
  let uVar20: u16;
  let uVar21: u16;
  let uStack82: u16;
  let iStack72: i16;
  let iStack68: i16;
  let paStack54: &mut Struct76;
  let local_2c: HDC16;
  let local_2a: PAINTSTRUCT16;
  RECT16 local_a [0x2];
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  uVar21 = (iVar6 + 0x6);
  GetWindowRect16(param_2,local_a);
  uVar13 = (iVar6 + 0x6);
  local_2c = BeginPaint16(s_tile2_bmp_1050_1538,&local_2a);
  uVar8 = 0x1008;
  b_force_background =
       palette_op_1008_4e08
                 ((ctx.PTR__LOOP_1050_4230 + 0xe),&local_2c,in_DX,0x1008)
  ;
  paStack54 = 0x0;
  if ((iVar6 + 0x98) != 0x0) {
    paStack54 = 
                unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,(iVar6 + 0x98),param_3)
    ;
    IVar9 = 0x1008;
    uVar10 = pass1_1008_4772(paStack54);
    if (((uVar10 >> 0x10) | uVar10) == 0x0) {
      if (paStack54 != 0x0) {
        if (paStack54 != 0x0) {
          ppcVar3 = paStack54;
          (**ppcVar3)(0x1008,paStack54,(paStack54 >> 0x10),0x1,uVar13);
        }
      }
      IVar9 = 0x1010;
      paStack54 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,0x4d,param_3);
    }
    uVar8 = SUB42(ctx.s_tile2_bmp_1050_1538,0x0);
    GetSystemMetrics16(IVar9);
    ppcVar3 = (paStack54 + 0x4);
    (**ppcVar3)();
  }
  if (paStack54 != 0x0) {
    if (paStack54 != 0x0) {
      ppcVar3 = paStack54;
      (**ppcVar3)(uVar8,paStack54,(paStack54 >> 0x10),0x1,uVar13,uVar21);
    }
  }
  paVar11 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,(iVar6 + 0x96),param_3);
  uVar21 = (paVar11 >> 0x10);
  pHVar14 = &local_2c;
  uVar19 = 0x4;
  uVar20 = 0xd;
  uVar15 = param_3;
  IVar9 = GetSystemMetrics16(0x1010);
  iVar5 = -(IVar9 + -0x23);
  uVar4 = paVar11;
  ppcVar3 = uVar4 + 0x2;
  uVar13 = paVar11;
  uVar8 = uVar21;
  (**ppcVar3)();
  if (paVar11 != 0x0) {
    if (paVar11 != 0x0) {
      ppcVar3 = uVar4;
      (**ppcVar3)(ctx.s_tile2_bmp_1050_1538,paVar11,uVar21,0x1,uVar13,uVar8,iVar5,
                  uVar19,uVar20,pHVar14,uVar15);
    }
  }
  handle = CreatePen16(s_tile2_bmp_1050_1538,0x25,0x100);
  handle_00 = SelectObject16(ctx.s_tile2_bmp_1050_1538,handle);
  paVar12 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,0x4f,param_3);
  uVar21 = (paVar12 >> 0x10);
  uVar10 = pass1_1008_4772(paVar12);
  uVar13 = (uVar10 >> 0x10);
  uVar4 = (uVar10 + 0x4);
  uVar2 = (uVar10 + 0x8);
  IVar9 = GetSystemMetrics16(0x1008);
  iVar5 = -(IVar9 + -0xc1);
  IVar9 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
  iStack72 = uVar2;
  x = 0xc5 - (IVar9 - iStack72);
  MoveTo16(ctx.s_tile2_bmp_1050_1538,iVar5,0x82);
  iStack68 = uVar4;
  y = iStack68 * 0xa + 0x85;
  LineTo16(ctx.s_tile2_bmp_1050_1538,iVar5,y);
  HVar18 = local_2c;
  LineTo16(ctx.s_tile2_bmp_1050_1538,x,y);
  HVar17 = local_2c;
  LineTo16(ctx.s_tile2_bmp_1050_1538,x,0x82);
  HVar16 = local_2c;
  LineTo16(ctx.s_tile2_bmp_1050_1538,iVar5,0x82);
  for (uStack82 = 0x0; puVar1 = (iVar6 + 0x94),
      uStack82 <= *puVar1 && *puVar1 != uStack82; uStack82 += 0x1) {
    pHVar14 = &local_2c;
    iVar5 = iStack68 * uStack82 + 0x84;
    uVar13 = 0x4;
    uVar15 = param_3;
    IVar9 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
    ppcVar3 = (paVar12 + 0x4);
    (**ppcVar3)(ctx.s_tile2_bmp_1050_1538,paVar12,uVar21,-(IVar9 + -0xc4),uVar13,
                iVar5,pHVar14,uVar15,HVar16,HVar17);
  }
  if (paVar12 != 0x0) {
    if (paVar12 != 0x0) {
      ppcVar3 = paVar12;
      (**ppcVar3)(ctx.s_tile2_bmp_1050_1538,paVar12,uVar21,0x1,HVar16,HVar17,HVar18)
      ;
    }
  }
  SelectObject16(ctx.s_tile2_bmp_1050_1538,handle_00);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  SelectPalette16(ctx.s_tile2_bmp_1050_1538,0x0,b_force_background);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  EndPaint16(s_tile2_bmp_1050_1538,&local_2a);
  return;
}



// WARNING: Could not reconcile some variable overlaps

pub fn draw_op_1040_7bb2(in_struct_1: &mut Struct14,in_win_handle_2: HWND16,param_3: u16)
{
  let ppcVar1: u32;
  let BVar2: bool;
  let y: i16;
  let iVar3: i16;
  COLORREF color;
  let handle: HPEN16;
  let handle_00: HGDIOBJ16;
  let rect: *mut RECT16;
  HANDLE16 handle_01;
  let mut str: String;
  let iVar4: &mut Struct14;
  let mut count: String; 
  let mut str_00: String; 
  let uVar4: u32;
  let DVar5: u32;
  let hbrush: HBRUSH16;
  let uVar6: u16;
  let uVar7: u16;
  let local_obj_handle_42: HGDIOBJ16;
  let local_rect_12: RECT16;
  let iStack14: i16;
  let iStack12: i16;
  let HStack10: HPALETTE16;
  let uStack8: u32;
  let local_hdc_4: HDC16;
  
  str_00 = (in_struct_1 >> 0x10);
  iVar4 = in_struct_1;
  uVar7 = iVar4.field_0x6;
  BVar2 = IsIconic16(in_win_handle_2);
  if (BVar2 == 0x0) {
    uVar6 = iVar4.field_0x6;
    local_hdc_4 = GetWindowDC16(s_tile2_bmp_1050_1538);
    ppcVar1 = (in_struct_1 + 0x68);
    uStack8 = 
              (**ppcVar1)(ctx.s_tile2_bmp_1050_1538,in_struct_1,iVar4.field_0x6e,uVar6,
                          uVar7);
    if (uStack8 != 0x0) {
      HStack10 = palette_op_1008_4e08
                           (uStack8,&local_hdc_4,
                            (uStack8 >> 0x10) | uStack8,0x1008);
      GetWindowRect16(0x1008,&local_rect_12);
      y = (iStack14 - local_rect_12.x) + -0x1;
      iVar3 = (iStack12 - local_rect_12.y) + -0x1;
      color = (-(iVar4.field_0x60 == 0x0) & 0x1e) + 0x25;
      handle = CreatePen16(s_tile2_bmp_1050_1538,color,0x100);
      handle_00 = SelectObject16(ctx.s_tile2_bmp_1050_1538,handle);
      MoveTo16(ctx.s_tile2_bmp_1050_1538,0x0,0x0);
      LineTo16(ctx.s_tile2_bmp_1050_1538,0x0,y);
      LineTo16(ctx.s_tile2_bmp_1050_1538,iVar3,y);
      LineTo16(ctx.s_tile2_bmp_1050_1538,iVar3,0x0);
      LineTo16(ctx.s_tile2_bmp_1050_1538,0x0,0x0);
      uVar4 = GetWindowLong16(s_tile2_bmp_1050_1538,-0x10);
      if (((uVar4 & 0x800000) != 0x0) && ((uVar4 & 0x400000) != 0x0)) {
        iVar3 = iVar4.field_0x62 - iVar4.field_0x66;
        MoveTo16(ctx.s_tile2_bmp_1050_1538,iVar3,0x0);
        LineTo16(ctx.s_tile2_bmp_1050_1538,iVar3,y);
        iVar4.field_0x7a = iVar4.field_0x64;
        iVar4.field_0x7c = iVar4.field_0x66;
        iVar4.field_0x7e = y;
        iVar4.field_0x80 = iVar4.field_0x62 - iVar4.field_0x66;
        hbrush = 0x4;
        rect = GetStockObject16(s_tile2_bmp_1050_1538);
        FillRect16(ctx.s_tile2_bmp_1050_1538,rect,hbrush);
        if (iVar4.field_0x76 != 0x0) {
          draw_op_1040_82ee(in_struct_1,s_tile2_bmp_1050_1538);
        }
        count = &iVar4.field_0x10;
        if (*count != '\0') {
          local_obj_handle_42 = 0x0;
          handle_01 = GetProp16(s_tile2_bmp_1050_1538,0x5de9);
          if (handle_01 != 0x0) {
            local_obj_handle_42 = SelectObject16(ctx.s_tile2_bmp_1050_1538,handle_01);
          }
          SetBkColor16(ctx.s_tile2_bmp_1050_1538,0x0);
          SetTextColor16(ctx.s_tile2_bmp_1050_1538,color);
          str = lstrlen16(ctx.s_tile2_bmp_1050_1538);
          DVar5 = GetTextExtent16(ctx.s_tile2_bmp_1050_1538,str,count);
          TextOut16(ctx.s_tile2_bmp_1050_1538,str,count,str_00,
                    (iVar4.field_0x80 - iVar4.field_0x7c) / 0x2 -
                    (DVar5 >> 0x10) / 0x2);
          if (handle_01 != 0x0) {
            SelectObject16(ctx.s_tile2_bmp_1050_1538,local_obj_handle_42);
          }
        }
      }
      ppcVar1 = (in_struct_1 + 0x64);
      (**ppcVar1)(ctx.s_tile2_bmp_1050_1538,in_struct_1,uStack8,local_hdc_4);
      HStack10 = SelectPalette16(ctx.s_tile2_bmp_1050_1538,0x0,HStack10);
      DeleteObject16(ctx.s_tile2_bmp_1050_1538);
      SelectObject16(ctx.s_tile2_bmp_1050_1538,handle_00);
      DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    }
    ReleaseDC16(s_tile2_bmp_1050_1538,local_hdc_4);
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn set_text_bk_color_1040_7e5e(param_1: *mut u32,param_2: u16,param_3: u16,param_4: i16) -> u32

{
  let ppcVar1: u32;
  let iVar2: i16;
  let HVar3: HGDIOBJ16;
  let IVar4: i16;
  let hwnd: HWND16;
  let hdc: HWND16;
  let uVar5: u32;
  COLORREF color;
  let uVar6: u16;
  
  uVar6 = 0x4;
  hwnd = ctx.s_tile2_bmp_1050_1538;
  HVar3 = GetStockObject16(param_4);
  if (ctx.PTR__LOOP_1050_5df0 == 0x0) {
    ppcVar1 = (*param_1 + 0x68);
    uVar5 = (**ppcVar1)(ctx.s_tile2_bmp_1050_1538,param_1,
                        (param_1 + 0x6e),uVar6);
    if (uVar5 == 0x0) {
      return 0x0;
    }
    hwnd = 0x1008;
    uVar5 = pass1_1008_4d72(uVar5);
    uVar6 = (uVar5 >> 0x10);
    iVar2 = uVar5;
    ctx._PTR_LOOP_1050_5df0 =
         CONCAT22(CONCAT11(0x2,*(iVar2 + 0x94)),
                  CONCAT11(*(iVar2 + 0x95),*(iVar2 + 0x96)));
  }
  hdc = hwnd;
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return 0x0;
    }
    hdc = ctx.s_tile2_bmp_1050_1538;
    IVar4 = GetDlgCtrlID16(hwnd);
    if (IVar4 == 0x14c) {
      color = 0x0;
//       TODO: goto LAB_1040_7f00;
    }
    if (IVar4 == 0x175) {
      color = 0x0;
//       TODO: goto LAB_1040_7f00;
    }
  }
  color = _PTR_LOOP_1050_5df0;
//LAB_1040_7f00:
  SetTextColor16(hdc,color);
  SetBkColor16(ctx.s_tile2_bmp_1050_1538,0x0);
  return CONCAT22(0x1050,HVar3);
}


pub fn draw_op_1040_82ee(param_1: &mut Struct15,COLORREF in_colorref_2)
{
  let iVar1: &mut Struct15;
  let uVar1: u16;
  let local_1a: u32;
  let uStack22: u32;
  let local_12: i16;
  let iStack16: i16;
  let iStack14: i16;
  let iStack12: i16;
  let l_brush_handle: *mut RECT16;
  let iStack8: i16;
  let iStack6: i16;
  let iStack4: i16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iStack6 = (iVar1.field_0x80 - iVar1.field_0x7c) + -0x2;
  iStack8 = (-(iVar1.field_0x60 == 0x0) & 0x1e) + 0x25;
  iStack4 = iStack6;
  l_brush_handle = CreateSolidBrush16(in_colorref_2);
  if (iVar1.field_0x86 == 0x0) {
    local_1a = CONCAT22(iVar1.field_0x66 + 0x2,iVar1.field_0x64 + 0x2);
    uStack22 = CONCAT22(iStack4,iStack6);
    &iVar1.field_0x82 = local_1a;
    &iVar1.field_0x86 = uStack22;
  }
  local_12 = iVar1.field_0x82 + 0x2;
  iStack16 = (iVar1.field_0x88 - iVar1.field_0x84) / 0x2 + iVar1.field_0x84 + -0x2;
  iStack14 = iVar1.field_0x86 - 0x2;
  iStack12 = (iVar1.field_0x88 - iVar1.field_0x84) / 0x2 + iVar1.field_0x84 + 0x2;
  FrameRect16(ctx.s_tile2_bmp_1050_1538,l_brush_handle,&iVar1.field_0x82);
  FrameRect16(ctx.s_tile2_bmp_1050_1538,l_brush_handle,&local_12);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  iVar1.field_0x7a = iVar1.field_0x86 + 0x2;
  return;
}


pub fn mixed_draw_op_1040_8a06(param_1: u32,param_2: HWND16,param_3: u16)
{
  let uVar1: u8;
  let uVar2: u8;
  let paVar3: &mut Struct13;
  let uVar4: u16;
  let b_force_background: HPALETTE16;
  COLORREF color;
  COLORREF color_00;
  HANDLE16 handle;
  let in_DX: u16;
  let rect: *mut RECT16;
  let uVar5: u32;
  let HStack62: HGDIOBJ16;
  let local_24: HDC16;
  let local_22: PAINTSTRUCT16;
  
  rect = (param_1 >> 0x10);
  local_24 = BeginPaint16(param_2,&local_22);
  paVar3 = (ctx.PTR__LOOP_1050_4230 + 0xe);
  b_force_background = palette_op_1008_4e08(paVar3,&local_24,in_DX,0x1008);
  uVar5 = pass1_1008_4d72(paVar3);
  uVar4 = (uVar5 >> 0x10);
  uVar1 = *(uVar5 + 0x95);
  uVar2 = *(uVar5 + 0x96);
  DrawIcon16(0x1008,(param_1 + 0x8e),0xa,0x14);
  color = SetBkColor16(ctx.s_tile2_bmp_1050_1538,0x0);
  color_00 = SetTextColor16(ctx.s_tile2_bmp_1050_1538,CONCAT11(uVar1,uVar2));
  HStack62 = 0x0;
  handle = GetProp16(s_tile2_bmp_1050_1538,0x5dfa);
  if (handle != 0x0) {
    HStack62 = SelectObject16(ctx.s_tile2_bmp_1050_1538,handle);
  }
  DrawText16(ctx.s_tile2_bmp_1050_1538,&ctx.PTR_LOOP_1050_0010,param_1 + 0x9e,
             rect,0xffff);
  if (handle != 0x0) {
    SelectObject16(ctx.s_tile2_bmp_1050_1538,HStack62);
  }
  SetBkColor16(ctx.s_tile2_bmp_1050_1538,color);
  SetTextColor16(ctx.s_tile2_bmp_1050_1538,color_00);
  SelectPalette16(ctx.s_tile2_bmp_1050_1538,0x0,b_force_background);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  EndPaint16(s_tile2_bmp_1050_1538,&local_22);
  return;
}


pub fn load_icon_1040_8b92(param_1: u32,HINSTANCE16 param_2)
{
  let bVar1: u8;
  HICON16 HVar2;
  let uVar3: u16;
  let mut name: String;
  
  uVar3 = (param_1 >> 0x10);
  bVar1 = (param_1 + 0x98) & 0xf0;
  if (bVar1 == 0x30) {
    name = 0x7f03;
  }
  else {
    if ((bVar1 == 0x10) || (bVar1 == 0x10)) {
      name = 0x7f01;
    }
    else {
      if ((bVar1 == 0x40) || (bVar1 == 0x40)) {
        name = 0x7f04;
      }
      else {
        if (bVar1 != 0x20) {
          return;
        }
        name = 0x7f02;
      }
    }
  }
  HVar2 = LoadIcon16(param_2,name);
  *(HICON16 *)(param_1 + 0x8e) = HVar2;
  return;
}



pub fn draw_text_1040_8d14(param_1: &mut Struct37,param_2: HWND16)
{
  let bVar1: u8;
  let IVar2: i16;
  HANDLE16 handle;
  let iVar3: &mut Struct37;
  let rect: *mut RECT16;
  let HStack8: HGDIOBJ16;
  
  rect = (param_1 >> 0x10);
  iVar3 = param_1;
  bVar1 = iVar3.field_0x98 & 0xf0;
  if ((((bVar1 == 0x30) || (bVar1 == 0x10)) || (bVar1 == 0x40)) || (bVar1 == 0x20)) {
    iVar3.field_0xa0 = 0xa;
    IVar2 = GetSystemMetrics16(param_2);
    iVar3.field_0x9e = IVar2 + 0x28;
    param_2 = ctx.s_tile2_bmp_1050_1538;
  }
  else {
    iVar3.field_0xa0 = 0xa;
    iVar3.field_0x9e = 0x14;
  }
  HStack8 = 0x0;
  handle = GetProp16(param_2,0x5e0f);
  if (handle != 0x0) {
    HStack8 = SelectObject16(ctx.s_tile2_bmp_1050_1538,handle);
  }
  DrawText16(ctx.s_tile2_bmp_1050_1538,0x410,&iVar3.field_0x9e,rect,
             0xffff);
  if (HStack8 != 0x0) {
    SelectObject16(ctx.s_tile2_bmp_1050_1538,HStack8);
  }
  return;
}



pub fn unk_draw_op_1040_9458(param_1: &mut Struct17,param_2: u8,param_3: u16,HDC16 param_4)
{
  let ppcVar1: u32;
  Ulet UVar2: i32;
  let uVar3: u16;
  let iVar4: &mut Struct17;
  let uVar4: u16;
  let puStack8: *mut u16;
  let puStack6: &mut u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (iVar4.field_0x8 != 0x0) {
    puStack6 = iVar4.field_0x8;
    uVar3 = (&iVar4.field_0x8 + 0x2);
    if ((((&iVar4.field_0xc + 0x2) | &iVar4.field_0xc) != 0x0) &&
       ((param_2 & 0x1) != 0x0)) {
      puStack6 = iVar4.field_0xc;
      uVar3 = (&iVar4.field_0xc + 0x2);
    }
    if ((iVar4.field_0x10 != 0x0) && ((param_2 & 0x4) != 0x0)) {
      puStack6 = iVar4.field_0x10;
      uVar3 = (&iVar4.field_0x10 + 0x2);
    }
    puStack8 = &param_3;
    UVar2 = *puStack6;
    ppcVar1 = (UVar2 + 0x8);
    (**ppcVar1)(param_4,puStack6,uVar3,puStack8);
    ppcVar1 = (UVar2 + 0x4);
    (**ppcVar1)(param_4,puStack6,iVar4.field_0x28,iVar4.field_0x26,&param_3);
    SelectPalette16(param_4,0x0,puStack8);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  }
  return;
}



pub fn draw_text_1040_94fc(param_1: &mut Struct37,HDC16 param_2)
{
  COLORREF color;
  COLORREF color_00;
  let iVar1: &mut Struct38;
  let rect: *mut RECT16;
  
  rect = (param_1 >> 0x10);
  iVar1 = param_1;
  color = SetBkColor16(param_2,iVar1.field_0x3a);
  color_00 = SetTextColor16(ctx.s_tile2_bmp_1050_1538,iVar1.field_0x3c);
  DrawText16(ctx.s_tile2_bmp_1050_1538,&ctx.PTR_LOOP_1050_0010,&iVar1.field_0x2e,
             rect,0xffff);
  SetBkColor16(ctx.s_tile2_bmp_1050_1538,color);
  SetTextColor16(ctx.s_tile2_bmp_1050_1538,color_00);
  return;
}



pub fn draw_text_1040_9650(Uparam_1: i32,param_2: HWND16)
{
  let hdc: HDC16;
  
  hdc = GetDC16(param_2);
  DrawText16(ctx.s_tile2_bmp_1050_1538,0x410,param_1 + 0x2e,
             (param_1 >> 0x10),0xffff);
  ReleaseDC16(s_tile2_bmp_1050_1538,hdc);
  return;
}




// WARNING: Could not reconcile some variable overlaps

pub fn draw_op_1040_9948(param_1: u16,param_2: u32,param_3: HWND16,param_4: &RECT16)
{
  let iVar1: i16;
  let iVar2: i16;
  int16_t mode;
  let uVar3: u16;
  let handle: HPEN16;
  let handle_00: HPEN16;
  let handle_01: HGDIOBJ16;
  let color: *mut u8;
  COLORREF color_00;
  COLORREF color_01;
  let iVar4: &mut Struct71;
  let y: i16;
  let mut x: String; 
  let cx: i16;
  let cy: i16;
  let iStack88: i16;
  let iStack86: i16;
  let iStack84: i16;
  let iStack82: i16;
  let iStack80: i16;
  let iStack78: i16;
  let local_42: PAINTSTRUCT16;
  let uStack34: u16;
  let uStack32: u16;
  let HStack30: HGDIOBJ16;
  let iStack28: i16;
  let iStack26: i16;
  let iStack24: i16;
  let iStack22: i16;
  let iStack20: i16;
  let local_12: RECT16;
  let uStack14: u32;
  let local_a: i16;
  let iStack8: i16;
  let iStack6: i16;
  let iStack4: i16;
  
  iStack6 = 0x1;
  iStack4 = 0x1;
  local_a = 0x0;
  iStack8 = 0x0;
  iStack28 = 0x0;
  HStack30 = 0x0;
  y = (param_2 >> 0x10);
  iVar4 = param_2;
  uStack32 = iVar4.field_0x4 & 0x8;
  uStack34 = iVar4.field_0x56 & 0x1;
  BeginPaint16(param_3,&local_42);
  mode = SetMapMode16(ctx.s_tile2_bmp_1050_1538,0x1);
  GetClientRect16(s_tile2_bmp_1050_1538,&local_12);
  iVar2 = (uStack14 >> 0x10);
  iVar1 = iVar2 + -0x1;
  uStack14 = CONCAT22(iVar1,uStack14 + -0x1);
  if (uStack34 != 0x0) {
    iStack26 = local_12;
    iStack24 = (local_12 >> 0x10);
    local_12 = CONCAT22(iStack24 + 0x2,iStack26 + 0x2);
    uStack14 = CONCAT22(iVar2 + -0x3,uStack14 + -0x3);
    iStack22 = uStack14 + -0x1;
    iStack20 = iVar1;
  }
  if (iVar4.field_0x6 != '\0') {
    iStack28 = 0x1;
    if (iVar4.field_0x5a != 0x0) {
      HStack30 = SelectObject16(ctx.s_tile2_bmp_1050_1538,iVar4.field_0x5a);
    }
    uVar3 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(&iVar4.field_0x6)));
    DrawText16(0x1000,0x400,&local_a,param_4,uVar3);
    iStack8 = ((uStack14._2_2_ - iStack4) + iStack8) / 0x2 + local_12.y;
    iStack4 += iStack8;
    local_a = ((uStack14 - iStack6) + local_a) / 0x2 + local_12.x;
    iStack6 += local_a;
  }
  handle = CreatePen16(s_tile2_bmp_1050_1538,DAT_1050_5ec2,
                       (ctx.DAT_1050_5ec2 >> 0x10));
  handle_00 = CreatePen16(s_tile2_bmp_1050_1538,DAT_1050_5ebe,
                          (ctx.DAT_1050_5ebe >> 0x10));
  handle_01 = SelectObject16(ctx.s_tile2_bmp_1050_1538,handle);
  if (uStack34 != 0x0) {
    iStack78 = 0x0;
    do {
      MoveTo16(ctx.s_tile2_bmp_1050_1538,iStack20,iStack26);
      LineTo16(ctx.s_tile2_bmp_1050_1538,iStack20,iStack22);
      LineTo16(ctx.s_tile2_bmp_1050_1538,iStack24,iStack22);
      LineTo16(ctx.s_tile2_bmp_1050_1538,iStack24,iStack26);
      LineTo16(ctx.s_tile2_bmp_1050_1538,iStack20,iStack26);
      iStack24 += 0x1;
      iStack26 += 0x1;
      iStack22 += -0x1;
      iStack20 += -0x1;
      iStack78 += 0x1;
    } while (iStack78 < 0x1);
  }
  if ((iVar4.field_0x4 & 0x2) == 0x0) {
    iStack84 = (local_12 >> 0x10);
    iStack82 = uStack14;
    iStack78 = 0x0;
    iStack86 = local_12.x;
    iStack80 = uStack14._2_2_;
    do {
      SelectObject16(ctx.s_tile2_bmp_1050_1538,handle);
      MoveTo16(ctx.s_tile2_bmp_1050_1538,iStack80,iStack86);
      LineTo16(ctx.s_tile2_bmp_1050_1538,iStack80,iStack82);
      LineTo16(ctx.s_tile2_bmp_1050_1538,iStack84,iStack82);
      iStack88 = 0x0;
      do {
        SelectObject16(ctx.s_tile2_bmp_1050_1538,handle_00);
        LineTo16(ctx.s_tile2_bmp_1050_1538,iStack84,iStack86);
        LineTo16(ctx.s_tile2_bmp_1050_1538,iStack80,iStack86);
        iStack88 += 0x1;
      } while (iStack88 < 0x2);
      iStack84 += 0x1;
      iStack86 += 0x1;
      iStack82 += -0x1;
      iStack80 += -0x1;
      iStack78 += 0x1;
    } while (iStack78 < 0x2);
  }
  else {
    MoveTo16(ctx.s_tile2_bmp_1050_1538,uStack14._2_2_,local_12.x);
    LineTo16(ctx.s_tile2_bmp_1050_1538,local_12.y,local_12.x);
    LineTo16(ctx.s_tile2_bmp_1050_1538,local_12.y,uStack14 + 0x1);
    if (iStack28 != 0x0) {
      iStack8 += 0x2;
      local_a += 0x2;
      iStack6 += 0x2;
      iStack4 += 0x2;
    }
  }
  MoveTo16(ctx.s_tile2_bmp_1050_1538,0x0,0x0);
  if (iStack28 != 0x0) {
    if ((iVar4.field_0x4 & 0x4) == 0x0) {
      color = ctx.PTR_LOOP_1050_5ec6;
      if (uStack32 != 0x0) {
        color = ctx.DAT_1050_5eca;
      }
      color_00 = SetTextColor16(ctx.s_tile2_bmp_1050_1538,color);
      color_01 = SetBkColor16(ctx.s_tile2_bmp_1050_1538,0x0);
      uVar3 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(&iVar4.field_0x6)))
      ;
      DrawText16(0x1000,(&ctx.PTR_LOOP_1050_0000 + 0x1),&local_a,param_4,
                 uVar3);
      SetTextColor16(ctx.s_tile2_bmp_1050_1538,color_00);
      SetBkColor16(ctx.s_tile2_bmp_1050_1538,color_01);
    }
    else {
      GetStockObject16(s_tile2_bmp_1050_1538);
      cx = 0x0;
      cy = 0x0;
      x = &iVar4.field_0x6;
      uVar3 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(x)));
      GrayString16(0x1000,iStack4 - iStack8,(LPVOID)(iStack6 - local_a),
                   CONCAT22(local_a,iStack8),uVar3,x,y,cx,cy);
    }
    if (HStack30 != 0x0) {
      SelectObject16(ctx.s_tile2_bmp_1050_1538,HStack30);
    }
  }
  SelectObject16(ctx.s_tile2_bmp_1050_1538,handle_01);
  SetMapMode16(ctx.s_tile2_bmp_1050_1538,mode);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  EndPaint16(s_tile2_bmp_1050_1538,&local_42);
  return;
}


pub fn draw_op_1040_a67e(param_1: u32,param_2: i16,param_3: u16,COLORREF param_4)
{
  let piVar1: *mut i16;
  let bVar2: bool;
  let uVar3: u16;
  let iVar4: i16;
  let HVar5: HBRUSH16;
  let iVar6: i16;
  let uVar7: u16;
  COLORREF hdc;
  let uVar8: u32;
  let iStack14: i16;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  hdc = param_4;
  if ((iVar6 + 0x8e) == 0x0) {
    hdc = s_tile2_bmp_1050_1538;
    HVar5 = CreateSolidBrush16(param_4);
    *(HBRUSH16 *)(iVar6 + 0x8e) = HVar5;
  }
  if (ctx.PTR__LOOP_1050_5ee8 == 0x0) {
    hdc = 0x1008;
    uVar8 = pass1_1008_4d72((ctx.PTR__LOOP_1050_4230 + 0xe));
    uVar3 = (uVar8 >> 0x10);
    iVar4 = uVar8;
    ctx._PTR_LOOP_1050_5ee8 =
         CONCAT12(*(iVar4 + 0x94),
                         CONCAT11(*(iVar4 + 0x95),
                                  *(iVar4 + 0x96)));
    ctx.PTR_LOOP_1050_5eec =
         
         CONCAT11(*(iVar4 + 0x3e5),*(iVar4 + 0x3e6));
    ctx.PTR_LOOP_1050_5eee = (iVar4 + 0x3e4);
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return;
    }
    bVar2 = false;
    for (iStack14 = 0x0; piVar1 = (iVar6 + 0xea),
        *piVar1 != iStack14 && iStack14 <= *piVar1; iStack14 += 0x1) {
      if ((iVar6 + 0x9a + iStack14 * 0x2) == param_2) {
        bVar2 = true;
        break;
      }
    }
    if (bVar2) {
      ctx.PTR_LOOP_1050_5ee8 = ctx.PTR_LOOP_1050_5eec;
    }
  }
  SetTextColor16(hdc,PTR_LOOP_1050_5ee8);
  SetBkColor16(ctx.s_tile2_bmp_1050_1538,0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn unk_draw_op_1040_b0f8(param_1: &mut Struct18)
{
  let uVar1: u16;
  let uVar2: u16;
  let in_DX: *mut u8;
  let iVar3: &mut Struct18;
  let unaff_DI: i16;
  let uVar3: u16;
  let uVar4: u16;
  let unaff_SS: u16;
  let puVar5: *mut u16;
  let paStack10: &mut Struct18;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  param_1.field_0x0 = 0xb772;
  iVar3.field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x32,unaff_SS,in_DX,unaff_DI);
  uVar4 = 0x1010;
  pass1_1010_7b8c(puVar5,iVar3.field_0x6,unaff_SS);
  if (iVar3.field_0x8e != 0x0) {
    uVar4 = SUB42(ctx.s_tile2_bmp_1050_1538,0x0);
    DeleteObject16(0x1010);
    iVar3.field_0x8e = 0x0;
  }
  uVar1 = iVar3.field_0x90;
  uVar2 = iVar3.field_0x92;
  paStack10 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1040_a5d0(CONCAT22(uVar2,uVar1));
    uVar4 = 0x1000;
    fn_ptr_1000_17ce(paStack10,0x1000);
  }
  ui_cleanup_op_1040_782c(param_1,uVar4);
  return;
}


pub fn invalidate_rect_1040_c028(param_1: u32,param_2: i16,param_3: HWND16,param_4: &RECT16)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u32;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  let iVar7: i16;
  let uVar9: u16;
  let rect: *mut RECT16;
  let local_a: RECT16;
  let iStack6: i16;
  let iStack4: i16;
  let piVar8: *mut i16;
  
  iVar7 = param_1;
  uVar9 = (param_1 >> 0x10);
  if (param_2 == 0x8) {
    GetClientRect16(param_3,&local_a);
    uVar1 = (iVar7 + 0x6);
    uVar3 = (iVar7 + 0x6);
    iVar5 = (uVar3 + 0x16);
    uVar3 = (iVar7 + 0x6);
    local_a.x = (uVar3 + 0x1a);
    uVar3 = (iVar7 + 0x6);
    local_a.y = (uVar3 + 0x1c);
    if (iVar5 != 0x0) {
      if (iVar5 < 0x2) {
        iVar4 = 0x1;
      }
      else {
        iVar4 = 0x2;
      }
      uVar2 = ((iVar5 - iVar4) * 0x4 + uVar1 + 0x2a);
      iVar5 = uVar2;
      uVar2 &= 0xffff0000;
      local_a.x = (iVar5 + 0x22) + (uVar2 | iVar5 + 0x1e);
    }
    uVar1 = (iVar7 + 0x6);
    iStack6 = (uVar1 + 0x1e);
    iStack4 += -0x5;
  }
  else {
    if (param_2 != 0x9) {
      if (param_2 != 0xa) {
        return;
      }
      uVar1 = (iVar7 + 0x6);
      uVar6 = uVar1 + 0x2a;
      if (((iVar7 + 0x8) | uVar6) == 0x0) {
        return;
      }
      uVar3 = (iVar7 + 0x6);
      uVar2 = (((uVar3 + 0x16) + -0x1) * 0x4 + uVar6);
      iVar7 = uVar2;
      uVar2 &= 0xffff0000;
      piVar8 = (uVar2 | iVar7 + 0x1e);
      uVar9 = (uVar2 >> 0x10);
      local_a.y = (iVar7 + 0x20) + -0x8;
      local_a.x = *piVar8;
      iStack6 = (iVar7 + 0x22) + *piVar8;
      iStack4 = (iVar7 + 0x20);
      param_4 = &local_a;
      rect = 0x0;
//       TODO: goto LAB_1040_c19d;
    }
    local_a.x = 0x0;
    local_a.y = 0x0;
    iStack6 = 0x0;
    iStack4 = 0x0;
    GetClientRect16(param_3,&local_a);
    uVar1 = (iVar7 + 0x6);
    local_a.x = (uVar1 + 0x1a);
    uVar1 = (iVar7 + 0x6);
    iStack6 = (uVar1 + 0x1e);
    iStack4 += -0x5;
    uVar1 = (iVar7 + 0x6);
    uVar3 = (iVar7 + 0x6);
    iVar7 = (uVar3 + 0x16);
    if (0x0 < iVar7) {
      uVar1 = (uVar1 + iVar7 * 0x4 + 0x26);
      iVar7 = uVar1;
      uVar9 = (uVar1 >> 0x10);
      local_a.y = (iVar7 + 0x20) + (iVar7 + 0x24);
    }
  }
  param_3 = ctx.s_tile2_bmp_1050_1538;
  rect = &local_a;
//LAB_1040_c19d:
  InvalidateRect16(param_3,rect,param_4);
  return;
}



pub fn unk_draw_op_1040_c226(param_1: u32,param_2: HWND16,param_3: u16)
{
  let uVar1: u32;
  let handle: HPEN16;
  let handle_00: HGDIOBJ16;
  let uVar2: u16;
  let local_32: RECT16;
  let iStack46: i16;
  let iStack44: i16;
  let uStack42: u16;
  let iStack40: i16;
  let pRStack38: *mut RECT16;
  let HStack36: HDC16;
  let local_22: PAINTSTRUCT16;
  
  uVar2 = (param_1 >> 0x10);
  HStack36 = BeginPaint16(param_2,&local_22);
  pRStack38 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
  GetClientRect16(s_tile2_bmp_1050_1538,&local_32);
  uVar1 = (param_1 + 0x6);
  iStack40 = (uVar1 + 0x1a);
  uVar1 = (param_1 + 0x6);
  uStack42 = (uVar1 + 0x1c);
  local_32.y += 0x2;
  local_32.x = iStack40 + -0xa;
  iStack46 += -0x2;
  iStack44 += -0x2;
  FrameRect16(ctx.s_tile2_bmp_1050_1538,pRStack38,&local_32);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  handle = CreatePen16(s_tile2_bmp_1050_1538,-0x7f80,0x0);
  handle_00 = SelectObject16(ctx.s_tile2_bmp_1050_1538,handle);
  draw_line_1040_c302(param_1,s_tile2_bmp_1050_1538);
  draw_op_1040_c38e(param_1);
  SelectObject16(ctx.s_tile2_bmp_1050_1538,handle_00);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  EndPaint16(s_tile2_bmp_1050_1538,&local_22);
  return;
}



pub fn draw_line_1040_c302(param_1: u32,HDC16 param_2)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u32;
  let uVar4: u16;
  let iVar5: i16;
  let iVar6: i16;
  let uVar7: u16;
  
  uVar7 = (param_1 >> 0x10);
  uVar3 = (param_1 + 0x6);
  iVar6 = (uVar3 + 0x16);
  if (0x1 < iVar6) {
    uVar1 = (param_1 + 0x6);
    uVar4 = uVar1 + 0x2a;
    uVar1 &= 0xffff0000;
    uVar2 = (uVar1 | uVar4);
    iVar5 = uVar2;
    uVar2 &= 0xffff0000;
    uVar7 = (uVar2 >> 0x10);
    MoveTo16(param_2,(iVar5 + 0x20) + (iVar5 + 0x24),
             (iVar5 + 0x22) / 0x2 + (uVar2 | iVar5 + 0x1e));
    uVar1 = (uVar4 + iVar6 * 0x4 + -0x4);
    iVar6 = uVar1;
    uVar1 &= 0xffff0000;
    uVar7 = (uVar1 >> 0x10);
    LineTo16(ctx.s_tile2_bmp_1050_1538,(iVar6 + 0x20),
             (iVar6 + 0x22) / 0x2 + (uVar1 | iVar6 + 0x1e));
  }
  return;
}



pub fn draw_op_1040_c38e(param_1: u32)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u32;
  let iVar4: i16;
  let iVar5: i16;
  INT16 *pIVar6;
  let in_DX: u16;
  let iVar7: i16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let hdc: HDC16;
  let unaff_SS: u16;
  let DVar11: u32;
  let iStack26: i16;
  let IStack20: i16;
  let iStack18: i16;
  let IStack16: i16;
  let iStack14: i16;
  
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  uVar2 = (iVar8 + 0x6);
  iVar7 = (uVar2 + 0x18);
  if ((iVar7 != 0x0) &&
     (uVar2 = (iVar8 + 0x6), (uVar2 + 0x16) != 0x0)) {
    hdc = 0x1010;
    iVar4 = iVar7;
    pass1_1010_2ee2(*(u32 **)(iVar8 + 0x6),unaff_SS,0x1010);
    for (iStack26 = 0x0; iStack26 < iVar7; iStack26 += 0x1) {
      uVar1 = (iStack26 * 0x4 + iVar4);
      iVar5 = uVar1;
      uVar1 &= 0xffff0000;
      pIVar6 = (INT16 *)(uVar1 | iVar5 + 0x1e);
      uVar10 = (uVar1 >> 0x10);
      iVar5 = (iVar5 + 0x24) / 0x2 + (iVar5 + 0x20);
      MoveTo16(hdc,iVar5,*pIVar6);
      LineTo16(ctx.s_tile2_bmp_1050_1538,iVar5,*pIVar6 + -0xf);
      hdc = ctx.s_tile2_bmp_1050_1538;
      DVar11 = GetCurrentPosition16(ctx.s_tile2_bmp_1050_1538);
      iStack18 = (DVar11 >> 0x10);
      IStack20 = DVar11;
      if (iStack26 == 0x0) {
        IStack16 = IStack20;
        iStack14 = iStack18;
      }
    }
    uVar2 = (iVar8 + 0x6);
    if ((uVar2 + 0x24) != 0x0) {
      iStack14 += -0xd;
    }
    uVar2 = (iVar8 + 0x6);
    if ((uVar2 + 0x26) != 0x0) {
      iStack18 += 0xd;
    }
    uVar2 = (iVar8 + 0x6);
    uVar3 = (iVar8 + 0x6);
    uVar1 = (uVar2 + (uVar3 + 0x16) * 0x4 + 0x26);
    iVar7 = uVar1;
    uVar1 &= 0xffff0000;
    uVar9 = (uVar1 >> 0x10);
    MoveTo16(hdc,(iVar7 + 0x24) / 0x2 + (iVar7 + 0x20),
             (iVar7 + 0x22) + (uVar1 | iVar7 + 0x1e));
    LineTo16(ctx.s_tile2_bmp_1050_1538,
             (iVar7 + 0x24) / 0x2 + (iVar7 + 0x20),IStack16);
    DVar11 = GetCurrentPosition16(ctx.s_tile2_bmp_1050_1538);
    iVar7 = (DVar11 >> 0x10);
    if (iVar7 < iStack14) {
      iStack14 = iVar7;
    }
    if (iStack18 < iVar7) {
      iStack18 = iVar7;
    }
    MoveTo16(ctx.s_tile2_bmp_1050_1538,iStack14,IStack16);
    LineTo16(ctx.s_tile2_bmp_1050_1538,iStack18,IStack20);
  }
  return;
}


pub fn draw_op_1040_c74c(param_1: *mut u32,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  let ppcVar2: u32;
  let uVar3: u32;
  let HVar4: HGDIOBJ16;
  let iVar5: i16;
  let uVar6: u16;
  let HStack16: HGDIOBJ16;
  let HStack14: HGDIOBJ16;
  let HStack12: HPEN16;
  let HStack10: HGDIOBJ16;
  let HStack8: HPALETTE16;
  let uStack6: u16;
  
  uVar6 = (ctx.PTR__LOOP_1050_4230 >> 0x10);
  uStack6 = (ctx.PTR__LOOP_1050_4230 + 0xe);
  uVar1 = (ctx.PTR__LOOP_1050_4230 + 0x10);
  HStack8 = palette_op_1008_4e08
                      (CONCAT22(uVar1,uStack6),&param_2 + 0x2,uVar1,
                       0x1008);
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  (iVar5 + 0x46) = 0x1;
  HStack10 = GetStockObject16(0x1008);
  HStack12 = CreatePen16(s_tile2_bmp_1050_1538,0x2,0x100);
  HStack14 = SelectObject16(ctx.s_tile2_bmp_1050_1538,HStack10);
  HStack16 = SelectObject16(ctx.s_tile2_bmp_1050_1538,HStack12);
  Rectangle16(ctx.s_tile2_bmp_1050_1538,(iVar5 + 0x24),
              (iVar5 + 0x22),0x0,0x0);
  MoveTo16(ctx.s_tile2_bmp_1050_1538,0x0,
           (iVar5 + 0x36) * 0x2 + (iVar5 + 0x2a));
  LineTo16(ctx.s_tile2_bmp_1050_1538,(iVar5 + 0x24),
           (iVar5 + 0x36) * 0x2 + (iVar5 + 0x2a));
  SelectObject16(ctx.s_tile2_bmp_1050_1538,HStack14);
  HVar4 = SelectObject16(ctx.s_tile2_bmp_1050_1538,HStack16);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  uVar3 = *param_1;
  ppcVar2 = (uVar3 + 0x10);
  (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,param_1,param_2,HVar4,param_2._2_2_);
  ppcVar2 = (uVar3 + 0x14);
  (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,param_1,param_2._2_2_);
  (iVar5 + 0x46) = 0x0;
  SelectPalette16(ctx.s_tile2_bmp_1050_1538,0x0,HStack8);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  return;
}


pub fn palette_op_1040_c886(param_1: u32,param_2: u8,param_3: u16,HDC16 param_4)
{
  let uVar1: u16;
  let ppcVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let uStack12: u16;
  let puStack8: u32;
  let HStack4: HPALETTE16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (((iVar3 + 0xa) | (iVar3 + 0x8)) != 0x0) {
    HStack4 = 0x0;
    if ((iVar3 + 0x46) == 0x0) {
      uVar5 = (ctx.PTR__LOOP_1050_4230 >> 0x10);
      uStack12 = (ctx.PTR__LOOP_1050_4230 + 0xe);
      uVar1 = (ctx.PTR__LOOP_1050_4230 + 0x10);
      param_4 = 0x1008;
      HStack4 = palette_op_1008_4e08
                          (CONCAT22(uVar1,uStack12),&param_3,uVar1,0x1008);
    }
    puStack8 = (iVar3 + 0x8);
    uVar5 = (iVar3 + 0xa);
    if ((((iVar3 + 0xe) | (iVar3 + 0xc)) != 0x0) &&
       ((param_2 & 0x1) != 0x0)) {
      puStack8 = (iVar3 + 0xc);
      uVar5 = (iVar3 + 0xe);
    }
    if (((iVar3 + 0x10) != 0x0) && ((param_2 & 0x4) != 0x0)) {
      puStack8 = (iVar3 + 0x10);
      uVar5 = (iVar3 + 0x12);
    }
    ppcVar2 = (*puStack8 + 0x4);
    (**ppcVar2)(param_4,puStack8,uVar5,(iVar3 + 0x28),
                (iVar3 + 0x26),&param_3);
    if ((iVar3 + 0x46) == 0x0) {
      SelectPalette16(param_4,0x0,HStack4);
      DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    }
  }
  return;
}

