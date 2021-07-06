

fn unk_draw_op_1020_0000(param_1: u32,HWND16 param_2,param_3: u16)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u32;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  HWND16 hwnd;
  let uVar7: u16;
  let local_c4: [u8;6];
  let local_be: [u8;2];
  let piStack184: *mut i16;
  let local_b4: i16;
  let iStack178: i16;
  i16 aiStack176 [0x3c];
  let iStack56: i16;
  let iStack48: i16;
  astruct_76 *paStack46;
  let local_2a: i16;
  let local_28: i16;
  let puStack38: u32;
  PAINTSTRUCT16 local_22;
  
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
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  uVar7 = (iVar5 + 0x4);
  BeginPaint16(param_2,&local_22);
  uVar3 = (iVar5 + 0x14);
  puStack38 = (uVar3 + 0xa);
  pass1_1008_3e94((param_1 & 0xffff0000 | (iVar5 + 0x18)),
                  CONCAT22(param_3,&local_2a),
                  CONCAT22(param_3,&local_28));
  hwnd = 0x1008;
  pass1_1008_4480(puStack38,
                  (param_1 & 0xffff0000 | (iVar5 + 0x18)),
                  *(astruct_76 **)(iVar5 + 0x24),param_3);
  paStack46 = (astruct_76 *)0x0;
  for (iStack48 = 0x0; iStack48 < 0x6; iStack48 += 0x1) {
    uVar3 = (iVar5 + 0x14);
    hwnd = 0x1010;
    pass1_1010_2b78(uVar3,(uVar3 >> 0x10),iStack48,
                    CONCAT22(param_3,&local_b4));
    if (local_b4 == 0x0) {
      for (iStack56 = 0x0; iVar4 = iStack56, iStack56 <= iStack178; iStack56 += 0x1) {
        piVar1 = aiStack176 + iStack56 * 0x3;
        piStack184 = piVar1;
        if (aiStack176[iStack56 * 0x3 + 0x2] != 0x0) {
          paStack46 = (astruct_76 *)
                      pass1_1010_2b98((iVar5 + 0x14),
                                      aiStack176[iStack56 * 0x3 + 0x2]);
          pass1_1008_3e54(CONCAT22(param_3,local_be),0x0,
                          aiStack176[iVar4 * 0x3 + 0x1] + local_2a,*piVar1 + local_28);
          hwnd = 0x1008;
          pass1_1008_4480(puStack38,CONCAT22(param_3,local_be),paStack46,
                          param_3);
        }
      }
    }
    else {
      _local_be = CONCAT22(param_3,aiStack176 + iStack178 * 0x3);
      if (aiStack176[iStack178 * 0x3 + 0x2] != 0x0) {
        paStack46 = (astruct_76 *)
                    pass1_1010_2b98((iVar5 + 0x14),
                                    aiStack176[iStack178 * 0x3 + 0x2]);
        pass1_1008_3e54(CONCAT22(param_3,local_c4),0x0,
                        (_local_be + 0x2) + local_2a,*_local_be + local_28);
        hwnd = 0x1008;
        pass1_1008_4480(puStack38,CONCAT22(param_3,local_c4),paStack46,
                        param_3);
      }
    }
  }
  ppcVar2 = (*puStack38 + 0x4);
  (**ppcVar2)(hwnd,puStack38,(puStack38 >> 0x10),0x0,0x0,iVar5 + 0xa,
              uVar6,uVar7);
  EndPaint16(hwnd,&local_22);
  return;
}


fn draw_op_1020_041e(param_1: u32,param_2: u16)
{
  fill_rect_1020_065e((param_1 + 0xe6),param_2);
  return;
}


fn fill_rect_1020_065e(param_1: u32,HWND16 in_win_handle_2)
{
  code **ppcVar1;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  let local_brush_handle: u16;
  let uStack50: u16;
  let iStack48: i16;
  let iStack46: i16;
  RECT16 *local_rect_1;
  HDC16 *pHStack42;
  let puStack40: u32;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  local_24 = BeginPaint16(in_win_handle_2,&local_22);
  if (0x280 < (iVar3 + 0xa)) {
    local_rect_1 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    local_brush_handle = 0x0;
    uStack50 = 0x0;
    iStack48 = (iVar3 + 0xa) + -0x1;
    iStack46 = (iVar3 + 0xc) + -0x1;
    FillRect16(s_tile2_bmp_1050_1538,local_rect_1,(HBRUSH16)&local_brush_handle);
    DeleteObject16(s_tile2_bmp_1050_1538);
  }
  uVar2 = (iVar3 + 0x6);
  puStack40 = (uVar2 + 0xe);
  pHStack42 = &local_24;
  uVar2 = *puStack40;
  ppcVar1 = (uVar2 + 0x8);
  (**ppcVar1)(s_tile2_bmp_1050_1538,puStack40,(puStack40 >> 0x10),
              pHStack42);
  ppcVar1 = (uVar2 + 0x4);
  (**ppcVar1)(s_tile2_bmp_1050_1538,puStack40,(iVar3 + 0x10),
              (iVar3 + 0xe),&local_24);
  pHStack42 = (HDC16 *)SelectPalette16(s_tile2_bmp_1050_1538,0x0,pHStack42)
  ;
  DeleteObject16(s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}


fn unk_draw_op_1020_0c3e(param_1: u32,HWND16 param_2)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  HDC16 *b_force_background;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  let uStack40: u16;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  local_24 = BeginPaint16(param_2,&local_22);
  uVar3 = (iVar4 + 0x6);
  uVar7 = (uVar3 >> 0x10);
  iVar5 = uVar3;
  puVar1 = (iVar5 + 0xa);
  uStack40 = puVar1;
  if (((iVar5 + 0xc) | uStack40) != 0x0) {
    b_force_background = &local_24;
    uVar3 = *puVar1;
    ppcVar2 = (uVar3 + 0x8);
    (**ppcVar2)(s_tile2_bmp_1050_1538,uStack40,(puVar1 >> 0x10),
                b_force_background);
    ppcVar2 = (uVar3 + 0x4);
    (**ppcVar2)(s_tile2_bmp_1050_1538,puVar1,(iVar4 + 0xc),
                (iVar4 + 0xa),&local_24);
    SelectPalette16(s_tile2_bmp_1050_1538,0x0,b_force_background);
    DeleteObject16(s_tile2_bmp_1050_1538);
  }
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}



fn win_ui_palette_op_1020_0cd2(param_1: u32,HWND16 param_2)
{
  let uVar1: u16;
  let puVar2: u32;
  code **ppcVar3;
  let uVar4: u32;
  let uVar5: u16;
  HDC16 hdc;
  HDC16 b_force_background;
  HPALETTE16 b_force_background_00;
  let UVar6: u16;
  let extraout_DX: u16;
  let iVar7: i16;
  let uVar8: u16;
  astruct_13 *paStack10;
  let uStack6: u16;
  
  uVar4 = (param_1 + 0x6);
  uVar8 = (uVar4 >> 0x10);
  iVar7 = uVar4;
  puVar2 = (iVar7 + 0xa);
  uVar1 = (iVar7 + 0xc);
  uStack6 = puVar2;
  uVar5 = uVar1 | uStack6;
  if (uVar5 != 0x0) {
    ppcVar3 = (*puVar2 + 0x14);
    (**ppcVar3)(param_2,uStack6,uVar1);
    paStack10 = (astruct_13 *)CONCAT22(extraout_DX,uVar5);
    uVar5 = extraout_DX | uVar5;
    if (uVar5 != 0x0) {
      hdc = GetDC16(param_2);
      b_force_background = hdc;
      create_palette_1008_4e38(paStack10,0x1008,uVar5);
      b_force_background_00 = SelectPalette16(0x1008,0x0,b_force_background);
      UVar6 = RealizePalette16(s_tile2_bmp_1050_1538);
      SelectPalette16(s_tile2_bmp_1050_1538,0x1,b_force_background_00);
      DeleteObject16(s_tile2_bmp_1050_1538);
      if (0x0 < UVar6) {
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,
                         (RECT16 *)(&ctx.PTR_LOOP_1050_0000 + 0x1),0x0);
      }
      ReleaseDC16((HWND16)s_tile2_bmp_1050_1538,hdc);
      return;
    }
  }
  return;
}


fn realize_palette_1020_0e46(param_1: u32,param_2: i16,HGDIOBJ16 param_3)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  
  if (param_2 != 0x0) {
    uVar3 = (param_1 + 0xf2);
    uVar5 = (uVar3 >> 0x10);
    iVar4 = uVar3;
    puVar1 = (iVar4 + 0x66);
    ppcVar2 = (*puVar1 + 0x18);
    (**ppcVar2)(param_3,puVar1,(iVar4 + 0x68));
    UnrealizeObject16(param_3);
    RealizePalette16(s_tile2_bmp_1050_1538);
  }
  return;
}


fn invalidate_rect_1020_157c(param_1: u32,param_2: i16,HWND16 param_3)
{
  let BVar1: bool;
  RECT16 local_a;
  let uStack4: u16;
  
  if (param_2 == 0x1) {
    (param_1 + 0x14) = 0x0;
    return;
  }
  if (param_2 == 0x2) {
    BVar1 = IsIconic16(param_3);
    if (BVar1 == 0x0) {
      GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_a);
      uStack4 = 0x9a;
      InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)0x0,&local_a);
      return;
    }
  }
  return;
}



fn draw_op_1020_15de(Uparam_1: i32,HWND16 in_win_handle_2)
{
  let uVar1: u32;
  code **ppcVar2;
  let BVar3: bool;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u16;
  HWND16 hwnd;
  let unaff_SS: u16;
  let uVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  uVar9 = (iVar5 + 0x4);
  local_24 = BeginPaint16(in_win_handle_2,&local_22);
  uVar8 = (iVar5 + 0x4);
  hwnd = s_tile2_bmp_1050_1538;
  BVar3 = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
  if (BVar3 == 0x0) {
    hwnd = 0x1010;
    uVar7 = pass1_1010_454a((iVar5 + 0x14));
    uVar4 = (uVar7 >> 0x10);
    if ((uVar4 | uVar7) != 0x0) {
      uVar1 = (iVar5 + 0x14);
      hwnd = 0x1008;
      pass1_1008_4480((iVar5 + 0x18),
                      (uVar1 & 0xffff0000 | (uVar1 + 0x76)),
                      (astruct_76 *)(uVar7 & 0xffff | uVar4 << 0x10),unaff_SS);
    }
    ppcVar2 = ((iVar5 + 0x18) + 0x4);
    (**ppcVar2)(hwnd,(iVar5 + 0x18),0x0,&local_24,unaff_SS,uVar8,uVar9);
  }
  else {
    draw_op_1020_1674(param_1,s_tile2_bmp_1050_1538);
  }
  EndPaint16(hwnd,&local_22);
  return;
}



fn draw_op_1020_1674(Uparam_1: i32,INT16 param_2)
{
  code **ppcVar1;
  let uVar2: u16;
  let local_1a: u16;
  let uStack24: u16;
  let iStack22: i16;
  let iStack20: i16;
  let iStack18: i16;
  let iStack16: i16;
  RECT16 local_e;
  let uStack10: i16;
  let iStack8: i16;
  RECT16 *pRStack6;
  let iStack4: i16;
  
  if (ctx.PTR_LOOP_1050_0010 == 0x0) {
    uVar2 = (param_1 >> 0x10);
    ppcVar1 = ((param_1 + 0x14) + 0x2c);
    iStack4 = (**ppcVar1)(param_2,(param_1 + 0x14));
    if (iStack4 != 0x0) {
      pRStack6 = (RECT16 *)GetStockObject16(param_2);
      GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_e);
      local_1a = 0x0;
      uStack24 = 0x0;
      iStack22 = (iStack10 - local_e.x) + -0x1;
      iStack20 = (iStack8 - local_e.y) + -0x1;
      iStack18 = iStack20;
      iStack16 = iStack22;
      FillRect16(s_tile2_bmp_1050_1538,pRStack6,(HBRUSH16)&local_1a);
      DrawIcon16(s_tile2_bmp_1050_1538,iStack4,0x2,0x2);
    }
  }
  return;
}


fn invalidate_rect_1020_1fb2(param_1: u32,param_2: i16,HWND16 param_3)
{
  let local_16: u16;
  let uStack20: u16;
  let iStack18: i16;
  let uStack16: u16;
  RECT16 local_e;
  let uStack10: i16;
  let uStack6: u16;
  let uStack4: u16;
  
  if (param_2 == 0x1) {
    (param_1 + 0x6) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  GetWindowRect16(param_3,&local_e);
  local_16 = 0x0;
  uStack6 = 0x46;
  uStack20 = 0x46;
  iStack18 = iStack10 - local_e.x;
  uStack4 = 0x5f;
  uStack16 = 0x5f;
  InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)0x0,&local_16);
  return;
}



// WARNING: Inlined function: struct_1010_4d5c

fn unk_draw_op_1020_2020(param_1: u32,HWND16 param_2,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let puVar3: u32;
  let uVar4: u16;
  HDC16 *pHVar5;
  let iVar6: i16;
  HPEN16 HVar7;
  HGDIOBJ16 HVar8;
  HBRUSH16 HVar9;
  let puVar10: *mut u8
  let extraout_DX: u16;
  let uVar11: u16;
  let iVar12: i16;
  let iVar13: i16;
  let puVar14: *mut u8;
  let uVar15: u16;
  let uVar16: u16;
  let style: i16;
  let uVar17: u32;
  let piVar18: *mut i16;
  let uVar19: u8;
  let uVar20: u8;
  let iVar21: i16;
  let uVar22: u8;
  let uVar23: u8;
  let local_38: [u8;6];
  let local_32: u16;
  let uStack48: u16;
  let uStack46: u32;
  let uStack42: u16;
  let puStack40: u32;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  puVar14 = &stack0xfffe;
  uVar15 = (param_1 >> 0x10);
  iVar12 = param_1;
  uVar16 = (iVar12 + 0x4);
  local_24 = BeginPaint16(param_2,&local_22);
  puStack40 = pass1_1010_4c2c((iVar12 + 0x6));
  pHVar5 = &local_24;
  ppcVar1 = (*puStack40 + 0x8);
  (**ppcVar1)(0x1010,puStack40,(puStack40 >> 0x10),pHVar5,param_3,uVar16)
  ;
  *(HDC16 **)(iVar12 + 0x10) = pHVar5;
  uVar2 = (iVar12 + 0x6);
  uStack42 = (uVar2 + 0x30);
  uVar2 = (iVar12 + 0x6);
  uStack46 = (uVar2 + 0x12);
  uStack48 = 0x14;
  local_32 = 0x0;
  style = 0x1008;
  pass1_1008_3e38(CONCAT22(param_3,local_38));
  while ((puVar14 + -0x38) < (puVar14 + -0x28)) {
    iVar12 = (puVar14 + -0x38) * 0x4;
    uVar2 = (puVar14 + -0x2c);
    uVar17 = pass1_1008_4772(*(astruct_76 **)(iVar12 + uVar2));
    puVar10 = (uVar17 >> 0x10);
    (puVar14 + -0x44) = uVar17;
    *(uchar **)(puVar14 + -0x42) = puVar10;
    uVar2 = (puVar14 + 0x6);
    pass1_1020_2286(uVar2,(uVar2 >> 0x10),
                    CONCAT13((param_3 >> 0x8),
                                    CONCAT12(param_3,puVar14 + -0x30)),
                    (uVar17 + 0x8));
    uVar2 = (puVar14 + -0x30);
    pass1_1008_3e76(CONCAT22(param_3,puVar14 + -0x36),0x0,uVar2,
                    (uVar2 >> 0x10));
    uVar2 = (puVar14 + -0x2c);
    pass1_1008_4480((puVar14 + -0x26),
                    CONCAT22(param_3,puVar14 + -0x36),
                    *(astruct_76 **)(uVar2 + iVar12),param_3);
    iVar12 = (puVar14 + -0x38);
    uVar2 = (puVar14 + -0x30);
    uVar15 = uVar2;
    uVar22 = (u8)(uVar2 >> 0x10);
    uVar23 = (u8)(uVar2 >> 0x18);
    uVar2 = (puVar14 + -0x44);
    uVar16 = (uVar2 >> 0x10);
    iVar13 = uVar2;
    iVar6 = (iVar13 + 0x4) + (puVar14 + -0x2e);
    iVar13 = (iVar13 + 0x8) + (puVar14 + -0x30);
    uVar2 = (puVar14 + 0x6);
    uVar2 = (uVar2 + 0x6);
    iVar21 = uVar2;
    uVar16 = (uVar2 >> 0x10);
    uVar19 = 0x8;
    uVar20 = 0x10;
    if (*(long *)(iVar21 + 0x1a) == 0x0) {
      uVar4 = (iVar21 + 0x30) << 0x3;
      mem_op_1000_179c(uVar4,puVar10,0x1000);
      (iVar21 + 0x1a) = uVar4;
      *(uchar **)(iVar21 + 0x1c) = puVar10;
    }
    uVar2 = (iVar21 + 0x1a);
    iVar12 *= 0x8;
    (uVar2 + iVar12) = CONCAT11(uVar23,uVar22);
    uVar2 = (iVar21 + 0x1a);
    (uVar2 + iVar12 + 0x2) = uVar15;
    uVar2 = (iVar21 + 0x1a);
    (uVar2 + iVar12 + 0x4) = iVar6;
    uVar2 = (iVar21 + 0x1a);
    (uVar2 + iVar12 + 0x6) = iVar13;
    style = CONCAT11(uVar20,uVar19);
    uVar2 = (puVar14 + -0x44);
    piVar18 = (puVar14 + -0x2e);
    *piVar18 = *piVar18 +
               (-((puVar14 + -0x38) == 0x0) & 0x5) + 0x14 +
               (uVar2 + 0x4);
    piVar18 = (puVar14 + -0x38);
    *piVar18 = *piVar18 + 0x1;
  }
  puVar3 = *(u32 **)(puVar14 + -0x26);
  ppcVar1 = (*puVar3 + 0x4);
  (**ppcVar1)(style,puVar3,(puVar3 >> 0x10),0x0,0x0,puVar14 + -0x22
              ,param_3);
  uVar11 = extraout_DX;
  HVar7 = CreatePen16(style,0x25,0x100);
  *(HPEN16 *)(puVar14 + -0x3a) = HVar7;
  HVar8 = SelectObject16(s_tile2_bmp_1050_1538,HVar7);
  *(HGDIOBJ16 *)(puVar14 + -0x3c) = HVar8;
  HVar9 = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  *(HBRUSH16 *)(puVar14 + -0x3e) = HVar9;
  HVar8 = SelectObject16(s_tile2_bmp_1050_1538,HVar9);
  *(HGDIOBJ16 *)(puVar14 + -0x40) = HVar8;
  draw_line_1020_229c((puVar14 + 0x6),s_tile2_bmp_1050_1538);
  uVar2 = (puVar14 + 0x6);
  pass1_1010_4df0((uVar2 + 0x6),uVar11,param_3);
  if (HVar8 == 0x0) {
    SelectObject16(0x1010,*(HGDIOBJ16 *)(puVar14 + -0x3c));
    DeleteObject16(s_tile2_bmp_1050_1538);
    SelectObject16(s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar14 + -0x40));
    DeleteObject16(s_tile2_bmp_1050_1538);
    HVar9 = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    *(HBRUSH16 *)(puVar14 + -0x3e) = HVar9;
    HVar7 = CreatePen16((INT16)s_tile2_bmp_1050_1538,0xff,0x0);
    *(HPEN16 *)(puVar14 + -0x3a) = HVar7;
    SelectObject16(s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar14 + -0x3e));
    SelectObject16(s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar14 + -0x3a));
  }
  uVar2 = (puVar14 + 0x6);
  piVar18 = pass1_1010_4dc8((uVar2 + 0x6));
  uVar15 = (piVar18 >> 0x10);
  uVar16 = SUB42(piVar18,0x0);
  pass1_1020_239c((puVar14 + 0x6),piVar18,param_3);
  uVar2 = (puVar14 + 0x6);
  uVar2 = (uVar2 + 0x6);
  if ((uVar2 + 0x2c) != 0x0) {
    pass1_1020_2488((puVar14 + 0x6),uVar16,uVar15);
  }
  uVar2 = (puVar14 + 0x6);
  SelectPalette16(0x1010,0x0,*(bool *)(uVar2 + 0x10));
  DeleteObject16(s_tile2_bmp_1050_1538);
  SelectObject16(s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar14 + -0x3c));
  DeleteObject16(s_tile2_bmp_1050_1538);
  SelectObject16(s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar14 + -0x40));
  DeleteObject16(s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,(PAINTSTRUCT16 *)(puVar14 + -0x20));
  return;
}


fn draw_line_1020_229c(param_1: u32,HDC16 param_2)
{
  let iVar1: i16;
  INT16 *pIVar2;
  let uVar3: u32;
  let iVar4: i16;
  let iVar5: i16;
  let piVar6: *mut i16;
  let uVar7: u16;
  let uStack10: i16;
  
  uVar7 = (param_1 >> 0x10);
  uVar3 = (param_1 + 0x6);
  iVar1 = (uVar3 + 0x30);
  uVar3 = (param_1 + 0x6);
  pIVar2 = *(INT16 **)(uVar3 + 0x1a);
  MoveTo16(param_2,0x5,*pIVar2);
  uVar7 = (pIVar2 >> 0x10);
  iVar5 = pIVar2;
  LineTo16(s_tile2_bmp_1050_1538,0x5,*(INT16 *)(iVar5 + iVar1 * 0x8 + -0x4));
  for (iStack10 = 0x0; iStack10 < iVar1; iStack10 += 0x1) {
    piVar6 = (iStack10 * 0x8 + iVar5);
    iVar4 = (piVar6[0x2] - *piVar6 >> 0x1) + *piVar6;
    MoveTo16(s_tile2_bmp_1050_1538,0x5,iVar4);
    LineTo16(s_tile2_bmp_1050_1538,0xa,iVar4);
  }
  MoveTo16(s_tile2_bmp_1050_1538,0x5f,*pIVar2);
  LineTo16(s_tile2_bmp_1050_1538,0x5f,*(INT16 *)(iVar5 + iVar1 * 0x8 + -0x4));
  for (iStack10 = 0x0; iStack10 < iVar1; iStack10 += 0x1) {
    piVar6 = (iStack10 * 0x8 + iVar5);
    iVar4 = (piVar6[0x2] - *piVar6 >> 0x1) + *piVar6;
    MoveTo16(s_tile2_bmp_1050_1538,0x5f,iVar4);
    LineTo16(s_tile2_bmp_1050_1538,0x5a,iVar4);
  }
  return;
}


fn draw_polygon_1020_2474(param_1: u16,param_2: u16,param_3: u32,HDC16 param_4)
{
  Polygon16(param_4,(POINT16 *)param_3,(param_3 >> 0x10));
  return;
}


fn realize_palette_1020_2992(Uparam_1: i32,param_2: i16)
{
  code **ppcVar1;
  let puVar2: u32;
  
  if (param_2 != 0x0) {
    puVar2 = pass1_1018_0a50((param_1 + 0xf2));
    ppcVar1 = (*puVar2 + 0x18);
    (**ppcVar1)(0x1018,puVar2,(puVar2 >> 0x10));
    UnrealizeObject16(0x1018);
    GetDC16((HWND16)s_tile2_bmp_1050_1538);
    RealizePalette16(s_tile2_bmp_1050_1538);
  }
  return;
}


fn invalidate_rect_1020_2ae4(param_1: *mut u32,param_2: u16,HWND16 param_3,param_4: u16)
{
  code **ppcVar1;
  let cVar2: u8;
  let iVar3: i16;
  let in_DX: *mut u8
  let uVar4: u16;
  let uVar5: u16;
  let unaff_DI: i16;
  let puVar6: *mut u16;
  let uVar7: u32;
  astruct_43 *paVar8;
  let uVar9: u16;
  let uVar10: u16;
  
  if (param_2 != 0x129) {
    uVar5 = param_1;
    uVar9 = (param_1 >> 0x10);
    if (0x129 < param_2) {
      if (param_2 == 0x12a) {
        uVar9 = 0xf012;
      }
      else {
        if (param_2 == 0x12b) {
          return;
        }
        if (param_2 == 0x12c) {
          uVar9 = 0xf020;
        }
        else {
          if (param_2 == 0x12d) {
            return;
          }
          if (param_2 != 0x12e) {
            return;
          }
          uVar9 = 0xf060;
        }
      }
      PostMessage16(param_3,0x0,0x0,CONCAT22(0x112,uVar9));
      return;
    }
    if (param_2 == 0xfb) {
      puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_4,in_DX,unaff_DI);
      pass1_1010_375e(puVar6);
      ppcVar1 = (*param_1 + 0x14);
      (**ppcVar1)();
      uVar7 = pass1_1010_375e(puVar6);
      uVar4 = (uVar7 >> 0x10);
      pass1_1018_181c((uVar5 + 0xf2),
                      (uVar7 & 0xffff | uVar4 << 0x10),
                      (uchar)(uVar7 & 0xffff),uVar4);
      return;
    }
    if (param_2 < 0xfc) {
      cVar2 = param_2;
      if (cVar2 == 'o') {
        paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,param_4);
        WinHelp16(0x1010,(LPCSTR)(s_New_failed_in_Op__Op_1050_0020 + 0xa),0x0,
                  CONCAT22(paVar8,0x1));
        return;
      }
      if (cVar2 == 'r') {
        iVar3 = uVar5 + 0xa;
        uVar10 = uVar9;
        puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_4,in_DX,unaff_DI);
        uVar4 = (puVar6 >> 0x10);
        pass1_1010_3770(puVar6,CONCAT22(uVar10,iVar3),uVar4);
        pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar5 + 0x8),0x3,uVar4,uVar5,
                        &ctx.PTR_LOOP_1050_1038,param_4);
        return;
      }
      if (cVar2 == 'u') {
        pass1_1018_0a76((uVar5 + 0xf2),param_4);
        InvalidateRect16(0x1018,(RECT16 *)0x0,0x0);
        return;
      }
    }
  }
  return;
}


fn load_draw_op_1020_2ede(param_1: *mut u16,param_2: u32,param_3: u16)
{
  let uVar1: u32;
  code **ppcVar2;
  HDC16 HVar3;
  let iVar4: i16;
  HPEN16 handle;
  HGDIOBJ16 HVar5;
  let in_DX: *mut u8
  let iVar6: i16;
  let unaff_DI: i16;
  let uVar7: u16;
  let unaff_SS: u16;
  let puVar8: *mut u16;
  astruct_76 *paVar9;
  let uVar10: u32;
  DEVMODEA *init_data;
  
  get_sys_metrics_1020_7c1a(param_1,param_2,param_3);
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  (iVar6 + 0x14) = 0x0;
  (iVar6 + 0x18) = 0x0;
  (iVar6 + 0x1a) = 0x0;
  (iVar6 + 0x1c) = 0x0;
  (iVar6 + 0x1e) = 0x0;
  (iVar6 + 0x20) = 0x0;
  *param_1 = 0x363c;
  (iVar6 + 0x2) = 0x1020;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,(param_2 + 0xfc),unaff_SS,
                           in_DX,unaff_DI);
  (iVar6 + 0x14) = puVar8;
  (iVar6 + 0x16) = (puVar8 >> 0x10);
  uVar1 = (iVar6 + 0x14);
  ppcVar2 = ((iVar6 + 0x14) + 0x4);
  (**ppcVar2)(0x1010,uVar1,(uVar1 >> 0x10),0x0,param_1);
  init_data = (DEVMODEA *)0x0;
  paVar9 = (astruct_76 *)pass1_1018_0a50((iVar6 + 0x14));
  uVar10 = pass1_1008_4772(paVar9);
  HVar3 = CreateDC16((LPCSTR)0x1008,(LPCSTR)uVar10,(LPCSTR)(uVar10 >> 0x10),init_data);
  *(HDC16 *)(iVar6 + 0x18) = HVar3;
  iVar4 = iVar6 + 0x18;
  ppcVar2 = (paVar9 + 0x8);
  (**ppcVar2)();
  (iVar6 + 0x20) = iVar4;
  uVar1 = (iVar6 + 0x14);
  uVar1 = (uVar1 + 0x64);
  handle = CreatePen16((INT16)s_tile2_bmp_1050_1538,uVar1,
                       (COLORREF)(uVar1 >> 0x10));
  *(HPEN16 *)(iVar6 + 0x1a) = handle;
  HVar5 = SelectObject16(s_tile2_bmp_1050_1538,handle);
  *(HGDIOBJ16 *)(iVar6 + 0x1c) = HVar5;
  HVar5 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar5 = SelectObject16(s_tile2_bmp_1050_1538,HVar5);
  *(HGDIOBJ16 *)(iVar6 + 0x1e) = HVar5;
  return;
}


fn invalidate_rect_1020_3080(param_1: u32,param_2: i16,HWND16 param_3)
{
  if (param_2 == 0x1) {
    (param_1 + 0x14) = 0x0;
    return;
  }
  if ((param_2 != 0x4) && (param_2 != 0x6)) {
    return;
  }
  InvalidateRect16(param_3,(RECT16 *)0x0,0x0);
  return;
}



fn draw_op_1020_30be(param_1: u32,HWND16 param_2,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let BVar3: bool;
  let iVar4: i16;
  let uVar5: u16;
  HWND16 hwnd;
  let uVar6: u16;
  let uVar7: u16;
  let local_3c: u32;
  let iStack56: i16;
  let iStack54: i16;
  let iStack52: i16;
  let iStack50: i16;
  RECT16 local_30;
  let iStack44: i16;
  let iStack42: i16;
  RECT16 *pRStack40;
  let iStack38: i16;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar7 = (iVar4 + 0x4);
  local_24 = BeginPaint16(param_2,&local_22);
  uVar6 = (iVar4 + 0x4);
  hwnd = s_tile2_bmp_1050_1538;
  BVar3 = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
  if (BVar3 == 0x0) {
    hwnd = 0x1018;
    local_3c = pass1_1018_0a50((iVar4 + 0x14));
    ppcVar1 = (*local_3c + 0x8);
    (**ppcVar1)(0x1018,local_3c,(local_3c >> 0x10),&local_24,param_3,
                uVar6,uVar7);
    uVar2 = (iVar4 + 0x14);
    if ((uVar2 + 0x84) == 0x1) {
      unk_draw_op_1020_320e(param_1,local_24,param_3);
    }
    ppcVar1 = (*local_3c + 0x4);
    (**ppcVar1)(0x1018,local_3c,(local_3c >> 0x10),0x0,0x0,0xdc,param_3);
    uVar2 = (iVar4 + 0x14);
    if ((uVar2 + 0x84) != 0x1) {
      unk_draw_op_1020_320e(param_1,local_24,param_3);
    }
    draw_op_1020_3488(param_1);
    ppcVar1 = (*local_3c + 0xc);
    (**ppcVar1)(0x1018,local_3c,(local_3c >> 0x10),&local_24,param_3);
  }
  else {
    if (ctx.PTR_LOOP_1050_0010 == 0x0) {
      ppcVar1 = ((iVar4 + 0x14) + 0x2c);
      iStack38 = (**ppcVar1)(s_tile2_bmp_1050_1538);
      if (iStack38 != 0x0) {
        pRStack40 = (RECT16 *)GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_30);
        local_3c = 0x0;
        iStack56 = (iStack44 - local_30.x) + -0x1;
        iStack54 = (iStack42 - local_30.y) + -0x1;
        iStack52 = iStack54;
        iStack50 = iStack56;
        FillRect16(s_tile2_bmp_1050_1538,pRStack40,(HBRUSH16)&local_3c);
        hwnd = s_tile2_bmp_1050_1538;
        DrawIcon16(s_tile2_bmp_1050_1538,iStack38,0x2,0x2);
      }
    }
  }
  EndPaint16(hwnd,&local_22);
  return;
}



fn unk_draw_op_1020_320e(param_1: u32,HDC16 param_2,param_3: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u32;
  DEVMODEA *init_data;
  let local_c: i16;
  let local_a: u32;
  HDC16 *pHStack6;
  HDC16 local_4;
  
  local_4 = param_2;
  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar3 = (iVar4 + 0x14);
  if ((uVar3 + 0x84) == 0x1) {
    uVar3 = (iVar4 + 0x14);
    uVar7 = (uVar3 >> 0x10);
    iVar5 = uVar3;
    puVar1 = (iVar5 + 0x24);
    init_data = (DEVMODEA *)0x0;
    uVar8 = pass1_1008_4772((astruct_76 *)
                            (puVar1 & 0xffff |
                            (iVar5 + 0x26) << 0x10));
    local_4 = CreateDC16((LPCSTR)0x1008,(LPCSTR)uVar8,(LPCSTR)(uVar8 >> 0x10),init_data);
    pHStack6 = &local_4;
    ppcVar2 = (*puVar1 + 0x8);
    (**ppcVar2)(s_tile2_bmp_1050_1538,puVar1,(puVar1 >> 0x10),
                pHStack6,param_3);
  }
  pass1_1018_0d9a((iVar4 + 0x14),CONCAT22(param_3,&local_c),
                  CONCAT22(param_3,&local_a));
  uVar3 = (iVar4 + 0x14);
  draw_op_1020_33c0(param_1,(uVar3 + 0x6c),local_c,local_a,0x1,local_4,
                    0x1018);
  pass1_1018_1054((iVar4 + 0x14),CONCAT22(param_3,&local_c),
                  CONCAT22(param_3,&local_a),param_3);
  uVar3 = (iVar4 + 0x14);
  draw_op_1020_33c0(param_1,(uVar3 + 0x74),local_c,local_a,0x2,local_4,
                    0x1018);
  pass1_1018_1320((iVar4 + 0x14),CONCAT22(param_3,&local_c),
                  CONCAT22(param_3,&local_a));
  uVar3 = (iVar4 + 0x14);
  draw_op_1020_33c0(param_1,(uVar3 + 0x68),local_c,local_a,0x1,local_4,
                    0x1018);
  pass1_1018_15f6((iVar4 + 0x14),CONCAT22(param_3,&local_c),
                  CONCAT22(param_3,&local_a));
  if (local_c != 0x0) {
    uVar3 = (iVar4 + 0x14);
    draw_op_1020_33c0(param_1,(uVar3 + 0x70),local_c,local_a,0x1,local_4,
                      0x1018);
  }
  pass1_1018_108c((iVar4 + 0x14),CONCAT22(param_3,&local_c),
                  CONCAT22(param_3,&local_a),param_3);
  if (local_c != 0x0) {
    uVar3 = (iVar4 + 0x14);
    draw_op_1020_33c0(param_1,(uVar3 + 0x78),local_c,local_a,0x0,local_4,
                      0x1018);
  }
  uVar3 = (iVar4 + 0x14);
  if ((uVar3 + 0x84) == 0x1) {
    SelectPalette16(0x1018,0x0,pHStack6);
    DeleteObject16(s_tile2_bmp_1050_1538);
    DeleteDC16(s_tile2_bmp_1050_1538);
  }
  return;
}



void 
draw_op_1020_33c0(param_1: u32,param_2: u32,param_3: i16,param_4: u32,param_5: i16,
                 param_6: u16,param_7: u16)

{
  HPEN16 pen_handle;
  HGDIOBJ16 object_handle;
  HBRUSH16 brush_handle;
  HGDIOBJ16 obj_handle_2;
  let iVar1: i16;
  let uVar2: u16;
  let in_DX: u16;
  let uVar3: u16;
  HDC16 hdc;
  let unaff_SS: u16;
  let uVar4: u16;
  let iStack20: i16;
  let puStack14: *mut u16;
  
  if (param_3 != 0x0) {
    pen_handle = CreatePen16(param_7,param_2,(COLORREF)(param_2 >> 0x10));
    object_handle = SelectObject16(s_tile2_bmp_1050_1538,pen_handle);
    brush_handle = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    hdc = s_tile2_bmp_1050_1538;
    obj_handle_2 = SelectObject16(s_tile2_bmp_1050_1538,brush_handle);
    puStack14 = param_4;
    for (iStack20 = 0x0; iStack20 < param_3; iStack20 += 0x1) {
      uVar4 = (param_1 >> 0x10);
      iVar1 = param_3;
      pass1_1020_3540(param_1,uVar4,param_5,puStack14,in_DX,unaff_SS);
      if (param_5 < 0x1) {
        uVar2 = 0x3;
      }
      else {
        uVar2 = 0x4;
      }
      uVar3 = in_DX;
      draw_polygon_1020_3602(param_1,uVar4,CONCAT22(iVar1,uVar2),hdc);
      hdc = 0x1000;
      fn_ptr_1000_17ce(CONCAT22(in_DX,iVar1),0x1000);
      puStack14 =
                  (puStack14 & 0xffff0000 | (puStack14 + 0x6));
      in_DX = uVar3;
    }
    SelectObject16(hdc,obj_handle_2);
    DeleteObject16(s_tile2_bmp_1050_1538);
    SelectObject16(s_tile2_bmp_1050_1538,object_handle);
    DeleteObject16(s_tile2_bmp_1050_1538);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn draw_op_1020_3488(Uparam_1: i32)
{
  let uVar1: u16;
  let uVar2: u32;
  let uVar3: u32;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  HGDIOBJ16 HVar4;
  let uVar5: u16;
  let unaff_SS: u16;
  let bottom: i16;
  let local_a: u32;
  let puStack6: *mut u16;
  
  uVar5 = (param_1 >> 0x10);
  uVar2 = (param_1 + 0x14);
  puStack6 = (uVar2 & 0xffff0000 | (uVar2 + 0x36));
  pass1_1008_3e94(puStack6,CONCAT22(unaff_SS,&local_a),
                  CONCAT22(unaff_SS,&local_a + 0x2));
  uVar2 = (local_a._2_2_ - 0x3) << 0x10;
  if ((local_a._2_2_ - 0x3) < 0x0) {
    uVar2 = 0x0;
  }
  uVar1 = local_a - 0x3;
  local_a = uVar1;
  if (uVar1 < 0x0) {
    local_a = 0x0;
  }
  local_a = uVar2 | local_a;
  uVar3 = (param_1 + 0x14);
  uVar3 = (uVar3 + 0x64);
  handle = CreatePen16(0x1008,uVar3,(COLORREF)(uVar3 >> 0x10));
  handle_00 = SelectObject16(s_tile2_bmp_1050_1538,handle);
  HVar4 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar4 = SelectObject16(s_tile2_bmp_1050_1538,HVar4);
  bottom = (local_a >> 0x10);
  Rectangle16(s_tile2_bmp_1050_1538,local_a + 0x6,bottom + 0x6,local_a,
              bottom);
  SelectObject16(s_tile2_bmp_1050_1538,handle_00);
  SelectObject16(s_tile2_bmp_1050_1538,HVar4);
  DeleteObject16(s_tile2_bmp_1050_1538);
  return;
}


fn draw_polygon_1020_3602(param_1: u16,param_2: u16,param_3: u32,HDC16 param_4)
{
  Polygon16(param_4,(POINT16 *)param_3,(param_3 >> 0x10));
  return;
}


fn unk_draw_op_1020_3da4(astruct_24 *param_1,Uparam_2: i32)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  int16_t iVar4;
  HGDIOBJ16 HVar5;
  HDC16 *pHVar6;
  let in_DX: *mut u8
  let uVar7: u16;
  let iVar8: i16;
  let unaff_DI: i16;
  let uVar9: u16;
  let unaff_CS: u16;
  let unaff_SS: u16;
  let puVar10: *mut u16;
  HDC16 local_4;
  astruct_24 *iVar9;
  astruct_24 *uVar8;
  
  get_sys_metrics_1020_7c1a(param_1,param_2,unaff_CS);
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  (iVar8 + 0x14) = 0x0;
  param_1.field_0x0 = 0x408a;
  (iVar8 + 0x2) = 0x1020;
  puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,unaff_SS,in_DX,unaff_DI);
  uVar7 = (puVar10 >> 0x10);
  (iVar8 + 0x14) = puVar10;
  (iVar8 + 0x16) = uVar7;
  ppcVar2 = ((iVar8 + 0x14) + 0x4);
  (**ppcVar2)(0x1010,(iVar8 + 0x14),uVar7,0x0,param_1);
  local_4 = GetDC16(0x1010);
  iVar4 = SetMapMode16(s_tile2_bmp_1050_1538,0x1);
  *(int16_t *)(iVar8 + 0x1e) = iVar4;
  HVar5 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar5 = SelectObject16(s_tile2_bmp_1050_1538,HVar5);
  *(HGDIOBJ16 *)(iVar8 + 0x18) = HVar5;
  HVar5 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar5 = SelectObject16(s_tile2_bmp_1050_1538,HVar5);
  *(HGDIOBJ16 *)(iVar8 + 0x1a) = HVar5;
  uVar3 = (iVar8 + 0x14);
  puVar1 = (uVar3 + 0x24);
  pHVar6 = &local_4;
  ppcVar2 = (*puVar1 + 0x8);
  (**ppcVar2)(s_tile2_bmp_1050_1538,puVar1,(puVar1 >> 0x10),pHVar6);
  *(HDC16 **)(iVar8 + 0x1c) = pHVar6;
  uVar3 = (iVar8 + 0x14);
  *(HDC16 *)(uVar3 + 0x4c) = local_4;
  return;
}


fn win_ui_palette_op_1020_3e84(astruct_16 *param_1)
{
  astruct_16 *iVar1;
  let uVar1: u16;
  let unaff_SS: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_16 *)param_1;
  param_1 = 0x408a;
  iVar1.field_0x2 = 0x1020;
  pass1_1010_1ea6(iVar1.field_0x14,param_1 & 0xffff | uVar1 << 0x10,
                  unaff_SS);
  SelectObject16(0x1010,iVar1.field_0x18);
  SelectObject16(s_tile2_bmp_1050_1538,iVar1.field_0x1a);
  SelectPalette16(s_tile2_bmp_1050_1538,0x0,iVar1.field_0x1c);
  DeleteObject16(s_tile2_bmp_1050_1538);
  SetMapMode16(s_tile2_bmp_1050_1538,iVar1.field_0x1e);
  param_1 = 0x3ab0;
  iVar1.field_0x2 = 0x1008;
  param_1 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  return;
}


fn validate_rect_1020_3f12(param_1: u32,param_2: i16,HWND16 param_3)
{
  RECT16 local_a;
  let uStack6: u32;
  
  if (param_2 == 0x1) {
    (param_1 + 0x14) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  local_a = (RECT16)0x8000e;
  uStack6 = 0x1100116;
  InvalidateRect16(param_3,(RECT16 *)0x0,&local_a);
  local_a = (RECT16)0xf10000;
  uStack6 = 0x1220030;
  ValidateRect16((HWND16)s_tile2_bmp_1050_1538,&local_a);
  local_a = (RECT16)0xf100f5;
  uStack6 = 0x1220127;
  ValidateRect16((HWND16)s_tile2_bmp_1050_1538,&local_a);
  return;
}



fn mixed_draw_op_1020_3fa0(param_1: u32,HWND16 param_2,param_3: u16)
{
  let uVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let iStack56: i16;
  let uStack54: u32;
  let local_32: u32;
  let iStack46: i16;
  let uStack44: u32;
  let puStack40: u32;
  let local_24: u16;
  PAINTSTRUCT16 local_22;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar6 = (iVar4 + 0x4);
  BeginPaint16(param_2,&local_22);
  uVar3 = (iVar4 + 0x14);
  local_24 = (uVar3 + 0x4c);
  uVar3 = (iVar4 + 0x14);
  puStack40 = (uVar3 + 0x24);
  ppcVar2 = (*puStack40 + 0x4);
  (**ppcVar2)(s_tile2_bmp_1050_1538,puStack40,(puStack40 >> 0x10),
              0x0,&local_24,param_3,uVar6);
  uVar3 = (iVar4 + 0x14);
  iStack46 = (uVar3 + 0x44);
  uVar3 = (iVar4 + 0x14);
  uStack44 = (uVar3 + 0x40);
  uVar1 = (iVar4 + 0x14);
  pass1_1008_3e94((uVar1 & 0xffff0000 | (uVar1 + 0x3a)),
                  CONCAT22(param_3,&local_32),
                  CONCAT22(param_3,&local_32 + 0x2));
  uStack54 = uStack44;
  for (iStack56 = 0x0; iStack56 < iStack46; iStack56 += 0x1) {
    draw_rect_1020_40ce(uStack54,local_32,(local_32 >> 0x10),param_3);
    uStack54 = uStack54 & 0xffff0000 | (uStack54 + 0x18);
  }
  EndPaint16(0x1008,&local_22);
  return;
}



astruct_18 *  pass1_1020_4064(astruct_18 *param_1,param_2: u8)

{
  win_ui_palette_op_1020_3e84((astruct_16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn draw_rect_1020_40ce(param_1: u32,param_2: i16,param_3: i16,param_4: u16)
{
  let iVar1: i16;
  HGDIOBJ16 HVar2;
  HPEN16 handle;
  let local_6: i16;
  let local_4: i16;
  
  pass1_1008_3e94((param_1 & 0xffff0000 | (param_1 + 0x10)),
                  CONCAT22(param_4,&local_6),
                  CONCAT22(param_4,&local_4));
  pass1_1008_3e94(param_1,CONCAT22(param_4,&local_6),
                  CONCAT22(param_4,&local_4));
  iVar1 = (param_1 + 0xa);
  Ellipse16(0x1008,iVar1 + local_6 + param_2,iVar1 + local_4 + param_3,
            (local_6 - (param_1 + 0xa)) + param_2,
            (local_4 - (param_1 + 0xa)) + param_3);
  if (((param_1 + 0xe) & 0x1) != 0x0) {
    HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    SelectObject16(s_tile2_bmp_1050_1538,HVar2);
    handle = CreatePen16((INT16)s_tile2_bmp_1050_1538,0xf9,0x100);
    SelectObject16(s_tile2_bmp_1050_1538,handle);
    Rectangle16(s_tile2_bmp_1050_1538,local_6 + param_2 + 0x5,
                local_4 + param_3 + 0x5,local_6 + param_2 + -0x5,local_4 + param_3 + -0x5)
    ;
    HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    SelectObject16(s_tile2_bmp_1050_1538,HVar2);
    HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    SelectObject16(s_tile2_bmp_1050_1538,HVar2);
    DeleteObject16(s_tile2_bmp_1050_1538);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn unk_draw_op_1020_41c8(astruct_20 *param_1,param_2: u16,param_3: u16,param_4: u16)
{
  code **ppcVar1;
  HCURSOR16 HVar2;
  let puVar3: *mut u16;
  let extraout_DX: *mut u8
  let puVar4: *mut u8
  let uVar6: u16;
  astruct_64 *uVar5;
  let unaff_DI: i16;
  let uVar7: u16;
  let unaff_SS: u16;
  let puVar8: *mut u16;
  let puVar9: *mut u8;
  let puVar10: *mut u8;
  let puVar11: *mut u8;
  
  unk_draw_op_1020_7f7a(param_1,0x8,CONCAT22(param_3,param_2));
  uVar7 = (param_1 >> 0x10);
  uVar5 = (astruct_64 *)param_1;
  uVar5.field_0xee = 0x0;
  uVar5.field_0xf0 = 0x0;
  uVar5.field_0xf2 = 0x0;
  uVar5.field_0xf4 = 0x1;
  uVar5.field_0xf6 = 0x0;
  uVar5.field_0xfa = 0x0;
  uVar5.field_0xfe = 0x0;
  uVar5.field_0x102 = 0x0;
  uVar5.field_0x106 = 0x0;
  uVar5.field_0x10a = 0x0;
  uVar5.field_0x108 = 0x0;
  uVar5.field_0x10c = 0x0;
  uVar5.field_0x110 = 0x0;
  uVar5.field_0x10e = 0x0;
  uVar5.field_0x112 = 0x0;
  uVar5.field_0x114 = 0x0;
  uVar5.field_0x116 = 0x0;
  param_1.field_0x0 = 0x623c;
  uVar5.field_0x2 = 0x1020;
  uVar5.field_0xe2 = 0x62d8;
  uVar5.field_0xe4 = 0x1020;
  puVar4 = extraout_DX;
  puVar11 = ctx.PTR_LOOP_1050_038c;
  HVar2 = LoadCursor16(param_4,(LPCSTR)(s__s__ld_1050_019c + 0x2));
  uVar5.field_0xf0 = HVar2;
  puVar10 = ctx.PTR_LOOP_1050_038c;
  HVar2 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,
                       (LPCSTR)(s__s__ld_1050_019c + 0x3));
  uVar5.field_0xf2 = HVar2;
  puVar9 = ctx.PTR_LOOP_1050_038c;
  ctx.PTR_LOOP_1050_0398 =
       
       LoadAccelerators16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)s_OpAccel_1050_43e8);
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,unaff_SS,puVar4,unaff_DI);
  &uVar5.field_0xfa = puVar8;
  (&uVar5.field_0xfa + 0x2) = (puVar8 >> 0x10);
  if (param_1 == (astruct_20 *)0x0) {
    puVar3 = 0x0;
    uVar6 = 0x0;
  }
  else {
    puVar3 = &uVar5.field_0xe2;
    uVar6 = uVar7;
  }
  ppcVar1 = (*uVar5.field_0xfa + 0x4);
  (**ppcVar1)(0x1010,uVar5.field_0xfa,0x0,puVar3,uVar6,puVar9,puVar10,puVar11);
  uVar5.field_0xe6 = uVar5.field_0xfa;
  return;
}


fn set_cursor_1020_5764(param_1: u32,param_2: i16,param_3: u16)
{
  let uVar1: u16;
  let uVar2: u32;
  let in_DX: *mut u8
  let iVar3: i16;
  let unaff_DI: i16;
  let uVar4: u16;
  HINSTANCE16 h_instance;
  HCURSOR16 hcursor;
  let local_e: i16;
  let local_c: [u8;2];
  let uStack10: u32;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,in_DX,unaff_DI);
  uVar4 = (puStack6 >> 0x10);
  uStack10 = (puStack6 + 0x20);
  uVar1 = (puStack6 + 0x22);
  if ((uVar1 | uStack10) != 0x0) {
    h_instance = 0x1030;
    pass1_1030_8308(_PTR_LOOP_1050_5748,
                    (_PTR_LOOP_1050_5748 >> 0x10),
                    CONCAT22(param_3,&local_e),

                    CONCAT13((param_3 >> 0x8),CONCAT12(param_3,local_c)),
                    uStack10 & 0xffff | uVar1 << 0x10,&local_e,uVar1);
    if (param_2 <= local_e) {
      uVar4 = (param_1 >> 0x10);
      iVar3 = param_1;
      if ((iVar3 + 0xf4) != 0x1) {
        SetCursor16(0x1030);
        (iVar3 + 0xee) = 0x0;
        (iVar3 + 0xf4) = 0x1;
        (iVar3 + 0x10c) = 0x0;
        h_instance = (HINSTANCE16)s_tile2_bmp_1050_1538;
        ReleaseCapture16();
      }
      LoadCursor16(h_instance,(LPCSTR)0x7f02);
      SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
      hcursor = 0x1018;
      pass1_1018_017c(puStack6,param_2,param_3);
      uVar2 = (iVar3 + 0xf6);
      (uVar2 + 0x10) = 0x1;
      if (*(long *)(iVar3 + 0xfe) != 0x0) {
        pass1_1020_68de((iVar3 + 0xfe),0x1018);
        hcursor = (HCURSOR16)s_tile2_bmp_1050_1538;
        PostMessage16(0x1018,0x0,0x0,0x11100eb);
      }
      SetCursor16(hcursor);
    }
  }
  return;
}



fn pt_in_rect_1020_5856(param_1: u32,POINT16 *param_2,param_3: u16)
{
  let puVar1: u32;
  let BVar2: bool;
  let uVar3: u32;
  let in_DX: u16;
  let extraout_DX: u16;
  let uStack10: u32;
  
  pass1_1018_2862((param_1 + 0xfa));
  if ((in_DX | param_3) != 0x0) {
    uStack10 = 0x0;
    while( true ) {
      puVar1 = (param_3 + 0xa);
      if (*puVar1 < uStack10 || *puVar1 == uStack10) break;
      uVar3 = uStack10;
      empty_1008_8fc4(param_3,in_DX,uStack10,(uStack10 >> 0x10));
      if ((extraout_DX | uVar3) != 0x0) {
        BVar2 = PtInRect16((RECT16 *)0x1008,*param_2);
        if (BVar2 != 0x0) {
          return;
        }
      }
      uStack10 += 0x1;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pt_in_rect_op_1020_58ce
          (param_1: u32,param_2: u16,param_3: u16,param_4: u8,RECT16 *param_5,
          param_6: u16)

{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  let Bvar4: bool;
  let msg: *mut u16;
  let in_DX: *mut u8
  let uVar5: u16;
  let puVar6: *mut u8
  let iVar7: i16;
  let iVar8: i16;
  let unaff_DI: i16;
  let uVar9: u16;
  let uVar10: u16;
  RECT16 *rect;
  RECT16 *rect_00;
  let uVar11: u32;
  let puVar12: *mut u16;
  let local_34: [u8;6];
  let uStack46: u32;
  let puStack38: *mut u16;
  let uStack30: u32;
  let puStack26: *mut u16;
  let local_18: [u16;0x2];
  let uStack20: u16;
  let uStack18: u32;
  let uStack14: u16;
  let puStack12: *mut u8
  let uStack10: u16;
  let uStack8: u16;
  let local_6: u16;
  let uStack4: u16;
  
  local_6 = param_3;
  uStack4 = param_2;
  uStack8 = param_4 & 0x8;
  uStack10 = param_4 & 0x4;
  uVar9 = (param_1 >> 0x10);
  iVar7 = param_1;
  uVar3 = pass1_1020_64d4((iVar7 + 0xf6),0x2);
  uStack30._2_2_ = in_DX;
  rect = param_5;
  if (uVar3 == 0x0) {
//LAB_1020_5942:
    uVar3 = pass1_1020_64d4((iVar7 + 0xf6),0x4);
    rect_00 = rect;
    if (uVar3 == 0x0) {
//LAB_1020_5a16:
      uVar3 = pass1_1020_64d4((iVar7 + 0xf6),0x1);
      if (uVar3 != 0x0) {
        uStack30 = pass1_1020_6498((iVar7 + 0xf6),0x1);
        uStack30._2_2_ = (uStack30 >> 0x10);
        for (puStack26 = 0x0; puStack26 < 0x4;
            puStack26 = (puStack26 + 0x1)) {
          BVar4 = PtInRect16(rect_00,(POINT16)CONCAT22(uStack4,local_6));
          if (BVar4 != 0x0) {
            local_18[0] = 0x0;
            uStack20 = 0x0;
            if (puStack26 == 0x0) {
              uStack20 = (-(uStack10 == 0x0) & 0x4) - 0x5;
            }
            else {
              if (puStack26 == (&ctx.PTR_LOOP_1050_0000 + 0x1)) {
                uStack20 = (-(uStack10 == 0x0) & 0xfffc) + 0x5;
              }
              else {
                if (puStack26 == &ctx.PTR_LOOP_1050_0002) {
                  local_18[0] = (-(uStack10 == 0x0) & 0x4) - 0x5;
                }
                else {
                  if (puStack26 == (&ctx.PTR_LOOP_1050_0002 + 0x1)) {
                    local_18[0] = (-(uStack10 == 0x0) & 0xfffc) + 0x5;
                  }
                }
              }
            }
            pass1_1020_2a94((iVar7 + 0xce),CONCAT22(local_18[0],uStack20),
                            param_6);
            return;
          }
          rect_00 = (RECT16 *)s_tile2_bmp_1050_1538;
        }
      }
      uVar3 = pass1_1020_64d4((iVar7 + 0xf6),0x3);
      if (uVar3 != 0x0) {
        uStack30._0_2_ = &local_6;
        pt_in_rect_1020_5856
                  (param_1,(POINT16 *)CONCAT22(param_6,uStack30),
                   uStack30);
        uVar5 = uStack30._2_2_ | uStack30;
        if (uVar5 != 0x0) {
          puStack26 = (uStack30)[0x17];
          if (((uStack8 == 0x0) || (uStack10 == 0x0)) && (uStack10 == 0x0)) {
            local_18[0] = 0x1;
          }
          else {
            local_18[0] = 0x2;
          }
          uStack20 = (uStack30)[0x6];
          uStack18 = CONCAT22(uStack18._2_2_,(uStack30)[0x7]);
          if ((puStack26 == 0xb) || (puStack26 == 0x37)) {
            uVar2 = (iVar7 + 0xfa);
            uVar10 = (uVar2 >> 0x10);
            iVar8 = uVar2;
            uStack46 = (iVar8 + 0x20);
            uVar5 = (iVar8 + 0x22);
            if ((uVar5 | uStack46) != 0x0) {
              puVar12 = pass1_1008_3e38(CONCAT22(param_6,local_34));
              puVar6 = (puVar12 >> 0x10);
              pass1_1018_161c(param_6,uStack46,CONCAT22(param_6,local_34),
                              uStack18,uStack20);
              puStack38 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,puVar6,unaff_DI
                                         );
              uVar5 = (puStack38 >> 0x10);
              pass1_1010_ecc6(puStack38,CONCAT22(param_6,local_34),
                              *(long *)(uStack46 + 0x3c),local_34,uVar5,
                              param_6);
            }
          }
          uVar3 = pass1_1018_25d2((iVar7 + 0xfa),local_18[0],
                                  uStack18 & 0xffff | uStack20 << 0x10,
                                  unaff_DI,param_6);
          if (uVar3 != 0x0) {
            return;
          }
          uVar3 = pass1_1020_5d56(param_1,
                                  CONCAT22(uStack30._2_2_,uStack30),uVar5,
                                  unaff_DI,param_6);
          if (uVar3 != 0x0) {
            return;
          }
        }
      }
      return;
    }
    uVar11 = pass1_1020_6498((iVar7 + 0xf6),0x4);
    uStack30._2_2_ = (uVar11 >> 0x10);
    uVar10 = uVar11;
    rect_00 = (RECT16 *)s_tile2_bmp_1050_1538;
    puVar6 = uStack30._2_2_;
    uStack14 = uVar10;
    puStack12 = uStack30._2_2_;
    BVar4 = PtInRect16(rect,(POINT16)CONCAT22(uStack4,local_6));
    if (BVar4 == 0x0) goto LAB_1020_5a16;
    rect = (RECT16 *)0x1010;
    uStack18 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_6,uStack30._2_2_,unaff_DI);
    if ((uStack18 + 0x72) != 0x0) {
      (iVar7 + 0x116) = 0x1;
      if (param_1 == 0x0) {
        iVar7 = 0x0;
        uVar9 = 0x0;
      }
      else {
        iVar7 += 0xe2;
      }
      uStack30 = CONCAT22(uVar9,iVar7);
      ppcVar1 = (*_PTR_LOOP_1050_02a0 + 0x4);
      (**ppcVar1)(0x1010,_PTR_LOOP_1050_02a0,
                  (_PTR_LOOP_1050_02a0 >> 0x10),0x10,iVar7,uVar9,uVar10,puVar6
                 );
      puVar12 = pass1_1008_941a(CONCAT22(param_6,local_18),0x1,0x86);
      msg = local_18;
      rect = (RECT16 *)0x1008;
      win_1008_5c9e(_PTR_LOOP_1050_02a0,CONCAT22(param_6,msg),msg,
                    (puVar12 >> 0x10),param_6);
      if (msg != 0x0) {
        return;
      }
      uVar9 = 0xf6;
      puStack26 = msg;
      goto LAB_1020_5936;
    }
    uVar9 = 0xf6;
  }
  else {
    uVar11 = pass1_1020_6498((iVar7 + 0xf6),0x2);
    uStack30._2_2_ = (uVar11 >> 0x10);
    uStack14 = uVar11;
    rect = (RECT16 *)s_tile2_bmp_1050_1538;
    puStack12 = uStack30._2_2_;
    BVar4 = PtInRect16(param_5,(POINT16)CONCAT22(uStack4,local_6));
    if (BVar4 == 0x0) goto LAB_1020_5942;
    uVar9 = 0x68;
  }
  msg = 0x0;
//LAB_1020_5936:
  PostMessage16((HWND16)rect,msg,(WPARAM16)msg,CONCAT22(0x111,uVar9));
  return;
}


fn mix_draw_op_1020_650c(astruct_7 *param_1,HWND16 param_2,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let iVar3: i16;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  PAINTSTRUCT16 local_28;
  let iStack8: i16;
  let puStack6: u32;
  
  uVar6 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar2 = (iVar3 + 0x14);
  puStack6 = (uVar2 + 0xa);
  if (((iVar3 + 0x10) != 0x0) ||
     (uVar2 = (iVar3 + 0x14), (uVar2 + 0x24) != 0x0)) {
    draw_op_1020_9364(param_1,param_2,param_3);
    if (*(long *)(iVar3 + 0x24) != 0x0) {
      uVar2 = (iVar3 + 0x24);
      ppcVar1 = ((iVar3 + 0x24) + 0x14);
      (**ppcVar1)(param_2,uVar2,(uVar2 >> 0x10));
    }
  }
  iStack8 = 0x0;
  do {
    iVar4 = iVar3 + 0x18;
    iVar5 = iStack8 * 0x4;
    if (*(long *)(iVar4 + iVar5) != 0x0) {
      uVar2 = (iVar4 + iVar5);
      ppcVar1 = ((iVar4 + iVar5) + 0x8);
      (**ppcVar1)(param_2,uVar2,(uVar2 >> 0x10),puStack6,
                  (puStack6 >> 0x10));
    }
    iStack8 += 0x1;
  } while (iStack8 < 0x5);
  uVar7 = (iVar3 + 0x4);
  BeginPaint16(param_2,&local_28);
  ppcVar1 = (*puStack6 + 0x4);
  (**ppcVar1)(s_tile2_bmp_1050_1538,puStack6,(puStack6 >> 0x10),0x0,
              0x0,iVar3 + 0xa,uVar6,uVar7);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_28);
  return;
}


fn pt_in_rect_1020_68fc(param_1: *mut u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let BVar3: bool;
  let uVar4: u32;
  let uVar5: u16;
  POlet PStack6: i16;
  
  PStack6 = (POINT16)CONCAT22(param_2,param_3);
  uVar5 = (param_1 >> 0x10);
  uVar2 = pass1_1018_31d0((param_1 + 0xf2));
  if (uVar2 != 0x0) {
    uVar4 = (param_1 + 0xf2);
    uVar4 = uVar4 & 0xffff0000 | (uVar4 + 0x16c);
    BVar3 = PtInRect16((RECT16 *)0x1018,PStack6);
    if (BVar3 != 0x0) {
      ppcVar1 = (*param_1 + 0x40);
      (**ppcVar1)(s_tile2_bmp_1050_1538,param_1,0xef,uVar4);
    }
  }
  return;
}


HGDIOBJ16  draw_op_1020_7070(INT16 param_1,param_2: u16)

{
  HGDIOBJ16 HVar1;
  
  HVar1 = GetStockObject16(param_1);
  if (_PTR_LOOP_1050_441e == 0x0) {
    _PTR_LOOP_1050_441e = 0x1000002;
  }
  if (0x6 < param_2) {
    return 0x0;
  }
  SetTextColor16(s_tile2_bmp_1050_1538,(COLORREF)_PTR_LOOP_1050_441e);
  SetBkColor16(s_tile2_bmp_1050_1538,0x0);
  return HVar1;
}


fn palette_op_1020_7270(param_1: *mut u16,HDC16 param_2)
{
  let uVar1: u16;
  let uVar2: u16;
  HPALETTE16 HVar3;
  let iVar4: i16;
  let uVar5: u16;
  let unaff_SS: u16;
  astruct_18 *paStack8;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x754c;
  (iVar4 + 0x2) = 0x1020;
  if (*(long *)(iVar4 + 0x1c) != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1ea6((iVar4 + 0x1c),
                    param_1 & 0xffff | uVar5 << 0x10,unaff_SS);
  }
  uVar1 = (iVar4 + 0x14);
  uVar2 = (iVar4 + 0x16);
  paStack8 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar2,uVar1));
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paStack8,0x1000);
  }
  HVar3 = SelectPalette16(param_2,0x0,*(bool *)(iVar4 + 0x20));
  *(HPALETTE16 *)(iVar4 + 0x20) = HVar3;
  DeleteObject16(s_tile2_bmp_1050_1538);
  *param_1 = 0x3ab0;
  (iVar4 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar4 + 0x2) = 0x1008;
  return;
}


fn invalidate_rect_1020_735a(param_1: u32,HWND16 param_2)
{
  InvalidateRect16(param_2,(RECT16 *)0x0,(param_1 + 0x1c) + 0x16c
                  );
  return;
}


fn draw_op_1020_7cc8(Uparam_1: i32,HWND16 in_win_handle_2,param_3: u16)
{
  code **ppcVar1;
  RECT16 *rect;
  COLORREF color;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  char *count;
  LPCSTR str;
  let puVar2: u32;
  let in_DX: u16;
  char *str_00;
  astruct_6 *iVar4;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let DVar6: u32;
  let uVar7: u32;
  let uVar8: u32;
  HBRUSH16 hbrush;
  let uVar9: u32;
  HDC16 HVar10;
  let uVar11: u16;
  let iStack66: i16;
  let local_20: u16;
  let iStack30: i16;
  let iStack28: i16;
  let iStack26: i16;
  let iStack24: i16;
  let iStack22: i16;
  RECT16 local_rect_1;
  let iStack16: i16;
  let iStack14: i16;
  HPALETTE16 HStack12;
  astruct_13 *paStack10;
  HDC16 local_hdc_1;
  let is_iconic: bool;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_6 *)param_1;
  is_iconic = IsIconic16(in_win_handle_2);
  if ((is_iconic == 0x0) || (ctx.PTR_LOOP_1050_0010 != 0x0)) {
    local_hdc_1 = GetWindowDC16((HWND16)s_tile2_bmp_1050_1538);
    paStack10 = *(astruct_13 **)(_PTR_LOOP_1050_4230 + 0xe);
    HStack12 = palette_op_1008_4e08(paStack10,&local_hdc_1,in_DX,0x1008);
    uVar11 = iVar4.field_0x4;
    GetWindowRect16(0x1008,&local_rect_1);
    iStack28 = (iStack16 - local_rect_1.x) + -0x1;
    iStack24 = (iStack14 - local_rect_1.y) + -0x1;
    local_20 = iVar4.field_0x10;
    iStack30 = iVar4.field_0x12;
    iStack26 = iStack24;
    if (is_iconic == 0x0) {
      iStack26 = iVar4.field_0xe - iVar4.field_0x12;
    }
    uVar9 = CONCAT22(param_3,&local_20);
    hbrush = 0x4;
    HVar10 = local_hdc_1;
    iStack22 = iStack28;
    rect = (RECT16 *)GetStockObject16((INT16)s_tile2_bmp_1050_1538);
    FillRect16(s_tile2_bmp_1050_1538,rect,hbrush);
    puVar2 = iVar4.field_0x6;
    uVar5 = (puVar2 >> 0x10);
    iVar3 = puVar2;
    puVar2 = (iVar3 + 0xe0);
    ppcVar1 = (*puVar2 + 0x24);
    (**ppcVar1)(s_tile2_bmp_1050_1538,puVar2,(iVar3 + 0xe2),0x0,
                uVar9,HVar10,uVar11);
    color = (-(puVar2 == 0x0) & 0x1e) + 0x25;
    handle = CreatePen16((INT16)s_tile2_bmp_1050_1538,color,0x100);
    handle_00 = SelectObject16(s_tile2_bmp_1050_1538,handle);
    MoveTo16(s_tile2_bmp_1050_1538,0x0,0x0);
    LineTo16(s_tile2_bmp_1050_1538,0x0,iStack22);
    LineTo16(s_tile2_bmp_1050_1538,iStack24,iStack22);
    uVar7 = local_hdc_1 << 0x10;
    LineTo16(s_tile2_bmp_1050_1538,iStack24,0x0);
    uVar8 = uVar7 & 0xffff0000 | local_hdc_1;
    uVar7 = 0x0;
    count = LineTo16(s_tile2_bmp_1050_1538,0x0,0x0);
    if (is_iconic == 0x0) {
      iVar3 = iVar4.field_0xe - iVar4.field_0x12;
      uVar7 = local_hdc_1 << 0x10;
      MoveTo16(s_tile2_bmp_1050_1538,iVar3,0x0);
      uVar7 = uVar7 & 0xffff0000 | local_hdc_1;
      count = LineTo16(s_tile2_bmp_1050_1538,iVar3,iStack22);
    }
    ppcVar1 = (*iVar4.field_0x6 + 0x18);
    (**ppcVar1)(s_tile2_bmp_1050_1538,iVar4.field_0x6,uVar7,uVar8);
    if (*count != '\0') {
      SetBkColor16(s_tile2_bmp_1050_1538,0x0);
      SetTextColor16(s_tile2_bmp_1050_1538,color);
      str = (LPCSTR)lstrlen16((LPCSTR)s_tile2_bmp_1050_1538);
      DVar6 = GetTextExtent16(s_tile2_bmp_1050_1538,str,count);
      iVar3 = (DVar6 >> 0x10);
      if (is_iconic == 0x0) {
        iStack66 = (iStack26 - iStack30) / 0x2 - iVar3 / 0x2;
      }
      else {
        iStack66 = iStack24 / 0x2 - iVar3 / 0x2;
      }
      TextOut16(s_tile2_bmp_1050_1538,str,count,str_00,iStack66);
    }
    HStack12 = SelectPalette16(s_tile2_bmp_1050_1538,0x0,HStack12);
    DeleteObject16(s_tile2_bmp_1050_1538);
    SelectObject16(s_tile2_bmp_1050_1538,handle_00);
    DeleteObject16(s_tile2_bmp_1050_1538);
    ReleaseDC16((HWND16)s_tile2_bmp_1050_1538,local_hdc_1);
  }
  return;
}


fn unk_draw_op_1020_7f7a(astruct_20 *param_1,param_2: u16,Uparam_3: i32)
{
  let uVar1: u16;
  HGDIOBJ16 HVar2;
  HCURSOR16 HVar3;
  let puVar4: *mut u8
  astruct_20 *iVar4;
  let unaff_DI: i16;
  let uVar5: u16;
  let unaff_SS: u16;
  astruct_20 *paVar6;
  let puVar7: *mut u16;
  let in_stack_0000000e: u16;
  
  paVar6 = unk_draw_op_1008_61b2
                     (param_1,param_2,param_3,
                      CONCAT22(in_stack_0000000e,param_3._2_2_),unaff_SS);
  puVar4 = (paVar6 >> 0x10);
  uVar5 = (param_1 >> 0x10);
  iVar4 = (astruct_20 *)param_1;
  ((astruct_20 *)(iVar4 + 0x1)).field_0x0 = 0x389a;
  iVar4[0x1].field_0x2 = 0x1008;
  ((astruct_20 *)(iVar4 + 0x1)).field_0x0 = 0x3aa8;
  iVar4[0x1].field_0x2 = 0x1008;
  iVar4[0x1].field_0x4 = 0x0;
  iVar4[0x1].field_0x8 = 0x0;
  iVar4[0x1].field_0xa = 0x0;
  param_1.field_0x0 = 0x82bc;
  iVar4.field_0x2 = 0x1020;
  ((astruct_20 *)(iVar4 + 0x1)).field_0x0 = 0x8358;
  iVar4[0x1].field_0x2 = 0x1020;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x5b)),
             s_VrMode_1050_4422);
  HVar2 = GetStockObject16(0x1000);
  iVar4.hgdiobj_field_0xc6 = HVar2;
  HVar3 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)0x7f00);
  iVar4.hcursor_field_0xc4 = HVar3;
  iVar4.field_0xc8 = 0x2028;
  iVar4.field_0xac = 0x47000000;
  iVar4.field_0xbc = (param_3._2_2_ + 0x8);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,puVar4,unaff_DI);
  uVar1 = (puVar7 >> 0x10);
  iVar4.field_0xb4 = 0x0;
  iVar4.field_0xb6 = 0x0;
  iVar4.field_0xb8 = (puVar7 + 0xa);
  iVar4.field_0xba = (puVar7 + 0xc);
  iVar4.field_0xca = param_3;
  win_ui_reg_class_1008_96d2(param_1,0x1008,unaff_SS);
  return;
}


fn realize_palette_1020_8128(param_1: u32,param_2: i16,HGDIOBJ16 param_3,param_4: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let puVar3: *mut u8;
  let puVar4: u32;
  let puVar5: u32;
  let extraout_DX: u16;
  let iVar6: i16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let local_12: [u8;8];
  let uStack10: u16;
  let uStack8: u16;
  let puStack6: u32;
  
  if (param_2 != 0x0) {
    uVar8 = (param_1 >> 0x10);
    iVar6 = param_1;
    uVar2 = (iVar6 + 0xe6);
    uVar9 = (uVar2 >> 0x10);
    iVar7 = uVar2;
    puVar5 = (iVar7 + 0xa);
    ppcVar1 = (*puVar5 + 0x18);
    puStack6 = puVar5;
    (**ppcVar1)(param_3,puVar5,(iVar7 + 0xc));
    uStack8 = SUB42(puVar5,0x0);
    UnrealizeObject16(param_3);
    uVar2 = (iVar6 + 0xe6);
    uVar8 = (uVar2 + 0x14);
    uStack10 = uVar8;
    RealizePalette16(s_tile2_bmp_1050_1538);
    pass1_1008_57a4(CONCAT22(param_4,local_12),
                    param_1 & 0xffff0000 | (iVar6 + 0xd2));
    while( true ) {
      puVar3 = local_12;
      pass1_1008_5b12(puVar3,param_4);
      if ((extraout_DX | puVar3) == 0x0) break;
      uVar9 = (puVar3 + 0x6);
      puVar4 = (puVar3 + 0x4);
      ppcVar1 = (*puVar4 + 0x90);
      (**ppcVar1)(0x1008,puVar4,uVar9,0x1,uVar8);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_ui_palette_op_1020_81c0(HWND16 param_1)
{
  astruct_13 *in_struct_1;
  let b_force_background: bool;
  HPALETTE16 b_force_background_00;
  let UVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let uStack6: u16;
  
  uVar3 = (_PTR_LOOP_1050_4230 >> 0x10);
  in_struct_1 = *(astruct_13 **)(_PTR_LOOP_1050_4230 + 0xe);
  uVar2 = (_PTR_LOOP_1050_4230 + 0x10);
  uStack6 = in_struct_1;
  if ((uVar2 | uStack6) == 0x0) {
    return;
  }
  b_force_background = GetDC16(param_1);
  create_palette_1008_4e38(in_struct_1,0x1008,uVar2);
  b_force_background_00 = SelectPalette16(0x1008,0x0,b_force_background);
  UVar1 = RealizePalette16(s_tile2_bmp_1050_1538);
  SelectPalette16(s_tile2_bmp_1050_1538,0x1,b_force_background_00);
  RealizePalette16(s_tile2_bmp_1050_1538);
  DeleteObject16(s_tile2_bmp_1050_1538);
  if (0x0 < UVar1) {
    InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,
                     (RECT16 *)(&ctx.PTR_LOOP_1050_0000 + 0x1),0x0);
  }
  return;
}


void 
invalidate_rect_1020_8d90
          (param_1: u32,param_2: u16,param_3: u32,param_4: u16,param_5: u16,
          param_6: u16)

{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  let in_AF: u8;
  let local_48: i16;
  let iStack70: i16;
  let iStack68: i16;
  let iStack66: i16;
  let local_40: i16;
  let local_3e: i16;
  let uStack60: u32;
  let local_38: [u8;28];
  uchar local_10 [0xa];
  let uStack6: u16;
  let uStack4: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  uStack6 = pass1_1018_266a((iVar2 + 0x22));
  if (uStack6 != 0x0) {
    pass1_1018_265c((iVar2 + 0x22));
    if ((param_5 | uStack6) != 0x0) {
      uStack4 = param_5;
      sys_1000_3f9c(local_10,param_6,s__03ld_1050_442a,
                    ctx.data_seg,uStack6,&stack0xfffe,uVar3,0x1000,param_6,
                    in_AF);
      uVar1 = (iVar2 + 0x22);
      file_and_draw_op_1008_4f20
                (CONCAT22(param_6,local_38),(uVar1 + 0xe),0x25,
                 CONCAT22(param_6,local_10),param_6);
      pass1_1008_4480(param_3,(param_1 & 0xffff0000 | (iVar2 + 0x1c)),
                      (astruct_76 *)CONCAT22(param_6,local_38),param_6);
      uStack60 = pass1_1008_4772((astruct_76 *)CONCAT22(param_6,local_38));
      pass1_1008_3e94((param_1 & 0xffff0000 | (iVar2 + 0x1c)),
                      CONCAT22(param_6,&local_40),
                      CONCAT22(param_6,&local_3e));
      local_48 = local_3e;
      iStack70 = local_40;
      uVar3 = (uStack60 >> 0x10);
      iStack68 = local_3e + (uStack60 + 0x4);
      iStack66 = local_40 + (uStack60 + 0x8);
      InvalidateRect16(0x1008,(RECT16 *)0x0,&local_48);
      pass1_1008_41bc(CONCAT22(param_6,local_38));
    }
  }
  return;
}


fn invalidate_rect_1020_8fb4(param_1: u32,param_2: u16)
{
  let iVar1: i16;
  let uVar2: u32;
  let erase: u16;
  let uVar3: u32;
  let in_DX: u16;
  let extraout_DX: u16;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u16;
  let unaff_SS: u16;
  let iStack8: i16;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  uVar2 = (iVar5 + 0xba);
  if ((uVar2 + 0x1e) != 0x0) {
    pass1_1018_2862((iVar5 + 0x16));
    (iVar5 + 0xaa) = param_2;
    (iVar5 + 0xac) = in_DX;
    if ((in_DX | (iVar5 + 0xaa)) != 0x0) {
      uVar2 = (iVar5 + 0xaa);
      iVar1 = (uVar2 + 0xa);
      for (iStack8 = 0x0; iStack8 < iVar1; iStack8 += 0x1) {
        uVar3 = SEXT24(iStack8);
        empty_1008_8fc4((iVar5 + 0xaa),uVar3);
        erase = uVar3;
        uVar4 = extraout_DX | erase;
        if (((uVar4 != 0x0) && (0x9 < (erase + 0x2e))) &&
           (pass1_1008_8b20(uVar3 & 0xffff | extraout_DX << 0x10,unaff_SS),
           (uVar4 | erase) != 0x0)) {
          InvalidateRect16(0x1008,(RECT16 *)0x0,erase);
        }
      }
    }
  }
  return;
}


fn palette_op_1020_92c4(param_1: *mut u16,HDC16 param_2)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x96c8;
  (iVar1 + 0x2) = 0x1020;
  if ((iVar1 + 0x12) != 0x0) {
    SelectPalette16(param_2,0x0,*(bool *)(iVar1 + 0x12));
    DeleteObject16(s_tile2_bmp_1050_1538);
  }
  *param_1 = 0x3ab0;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  return;
}



fn mix_draw_op_1020_9312(param_1: u32,HWND16 param_2)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  PAINTSTRUCT16 local_22;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar6 = (iVar4 + 0x4);
  BeginPaint16(param_2,&local_22);
  uVar3 = (iVar4 + 0x6);
  puVar1 = (uVar3 + 0xa);
  ppcVar2 = (*puVar1 + 0x4);
  (**ppcVar2)(s_tile2_bmp_1050_1538,puVar1,(puVar1 >> 0x10),0x0,
              param_1 & 0xffff0000 | (iVar4 + 0xa),uVar6);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}


fn draw_op_1020_9364(astruct_7 *param_1,HWND16 in_win_handle_2,INT16 param_3)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u32;
  let iVar5: i16;
  RECT16 *pRVar6;
  astruct_7 *local_struct_1;
  let var7: u16;
  let uVar7: u16;
  let iStack62: i16;
  let uStack58: u16;
  let local_38: [u8;4];
  HGDIOBJ16 HStack52;
  HPEN16 HStack50;
  let uStack48: u16;
  let uStack46: u32;
  let uStack42: u32;
  let uStack38: u32;
  let uStack34: u32;
  let uStack30: u32;
  let puStack26: *mut u16;
  let iStack22: i16;
  let iStack20: i16;
  let local_12: u32;
  let uStack14: u32;
  RECT16 local_a;
  let uStack6: u32;
  
  var7 = (param_1 >> 0x10);
  local_struct_1 = (astruct_7 *)param_1;
  GetClientRect16(in_win_handle_2,&local_a);
  local_12 = local_a;
  uStack14 = uStack6;
  iStack20 = DAT_1050_4216;
  iStack22 = DAT_1050_422c;
  puStack26 = _PTR_PTR_DAT_1050_0009_1050_4172_1050_4212;
  uStack30 = _PTR_PTR_1050_4218;
  uStack34 = _PTR_PTR_s_ew_failed_in_Op_Op_1050_0021_1050_41da_1050_421c;
  uStack38 = _PTR_PTR_DAT_1050_0041_1050_4202_1050_4220;
  uStack42 = _PTR_DAT_1050_419a_1050_4224;
  uStack46 = _PTR_PTR_1050_4228;
  uVar4 = local_struct_1.field_0x6;
  uStack48 = (uVar4 + 0x12);
  uStack58 = 0x9;
  do {
    uVar4 = (uStack58 * 0x4 + uStack34);
    HStack50 = CreatePen16((INT16)s_tile2_bmp_1050_1538,uVar4,
                           (COLORREF)(uVar4 >> 0x10));
    HStack52 = SelectObject16(s_tile2_bmp_1050_1538,HStack50);
    MoveToEx16(s_tile2_bmp_1050_1538,local_38,param_3,
               *(POINT16 **)(uStack58 * 0x2 + puStack26));
    LineTo16(s_tile2_bmp_1050_1538,*(INT16 *)(puStack26 + uStack58 * 0x2),
             uStack6);
    iVar3 = (iStack20 - uStack58) * 0x2;
    MoveToEx16(s_tile2_bmp_1050_1538,local_38,param_3,
               *(POINT16 **)(iVar3 + puStack26));
    LineTo16(s_tile2_bmp_1050_1538,*(INT16 *)(puStack26 + iVar3),
             uStack6);
    SelectObject16(s_tile2_bmp_1050_1538,HStack52);
    DeleteObject16(s_tile2_bmp_1050_1538);
    uStack58 -= 0x1;
  } while (uStack58 < 0x8000);
  pRVar6 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  uVar7 = (puStack26 >> 0x10);
  local_a = CONCAT22((puStack26 + 0x12) + 0x1,local_a.x);
  uVar2 = (puStack26 + 0x14);
  uStack14 = uStack14 & 0xffff | uVar2 << 0x10;
  uStack6 = CONCAT22(uVar2,uStack6);
  FillRect16(s_tile2_bmp_1050_1538,pRVar6,(HBRUSH16)&local_a);
  DeleteObject16(s_tile2_bmp_1050_1538);
  iStack62 = 0x8;
  for (uStack58 = 0x1; uStack58 < 0xa; uStack58 += 0x1) {
    pRVar6 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    uStack6 = uStack6 & 0xffff | (local_a.y - 0x1) << 0x10;
    local_12 = local_12 & 0xffff | (uStack14._2_2_ + 0x1) << 0x10;
    uVar7 = (puStack26 >> 0x10);
    local_a = local_a & 0xffff |
              ((iStack62 * 0x2 + puStack26) + 0x1) << 0x10;
    uStack14 = uStack14 & 0xffff |
               (uStack58 * 0x2 + puStack26 + 0x14) << 0x10;
    FillRect16(s_tile2_bmp_1050_1538,pRVar6,(HBRUSH16)&local_a);
    FillRect16(s_tile2_bmp_1050_1538,pRVar6,(HBRUSH16)&local_12);
    DeleteObject16(s_tile2_bmp_1050_1538);
    iStack62 += -0x1;
  }
  pRVar6 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  local_a &= 0xffff;
  uStack6 = uStack6 & 0xffff | *puStack26 << 0x10;
  local_12 = local_12 & 0xffff |
             ((iStack20 * 0x2 + puStack26) + 0x1) << 0x10;
  uStack14 = uStack14 & 0xffff | local_struct_1.field_0xe << 0x10;
  FillRect16(s_tile2_bmp_1050_1538,pRVar6,(HBRUSH16)&local_a);
  FillRect16(s_tile2_bmp_1050_1538,pRVar6,(HBRUSH16)&local_12);
  DeleteObject16(s_tile2_bmp_1050_1538);
  uStack58 = 0x3;
  do {
    uVar4 = (uStack58 * 0x4 + uStack38);
    HStack50 = CreatePen16((INT16)s_tile2_bmp_1050_1538,uVar4,
                           (COLORREF)(uVar4 >> 0x10));
    HStack52 = SelectObject16(s_tile2_bmp_1050_1538,HStack50);
    iVar5 = uStack58 * 0x2;
    iVar3 = (iVar5 + uStack42);
    uVar7 = (uStack46 >> 0x10);
    piVar1 = (iVar5 + uStack46);
    MoveToEx16(s_tile2_bmp_1050_1538,local_38,param_3,
               *(POINT16 **)((iVar5 + uStack46) * 0x2 + puStack26));
    LineTo16(s_tile2_bmp_1050_1538,
             *(INT16 *)((iStack20 - *piVar1) * 0x2 + puStack26),iVar3 + local_a.x);
    iVar3 = ((iStack22 - uStack58) * 0x2 + uStack42);
    MoveToEx16(s_tile2_bmp_1050_1538,local_38,param_3,
               *(POINT16 **)(*piVar1 * 0x2 + puStack26));
    LineTo16(s_tile2_bmp_1050_1538,
             *(INT16 *)((iStack20 - *piVar1) * 0x2 + puStack26),iVar3 + local_a.x);
    SelectObject16(s_tile2_bmp_1050_1538,HStack52);
    DeleteObject16(s_tile2_bmp_1050_1538);
    uStack58 -= 0x1;
  } while (uStack58 < 0x8000);
  local_struct_1.field_0x10 = 0x0;
  return;
}




