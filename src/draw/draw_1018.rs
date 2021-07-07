
fn pt_in_rect_1018_1bda(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  let iVar3: i16;
  let Bvar4: bool;
  let iVar5: i16;
  let uVar6: u16;
  let iStack26: i16;
  POlet PStack24: i16;
  let local_14: i16;
  let local_12: i16;
  let uStack16: u16;
  let uStack14: u32;
  let local_a: i16;
  let local_8: i16;
  let iStack6: i16;
  let iStack4: i16;
  
  uStack14 = 0x0;
  iVar3 = param_1;
  pass1_1008_3e94((param_1 & 0xffff0000 | (iVar3 + 0x3a)),
                  CONCAT22(param_4,&local_14),
                  CONCAT22(param_4,&local_12));
  PStack24 = (POINT16)CONCAT22(param_2,param_3);
  uStack16 = 0x0;
  iStack26 = 0x0;
  while( true ) {
    uVar6 = (param_1 >> 0x10);
    piVar1 = (iVar3 + 0x44);
    if (*piVar1 == iStack26 || *piVar1 < iStack26) {
      return;
    }
    uVar2 = (iVar3 + 0x42);
    iVar5 = (iVar3 + 0x40) + iStack26 * 0x18;
    uStack14 = CONCAT22(uVar2,iVar5);
    pass1_1008_3e94(CONCAT22(uVar2,iVar5),CONCAT22(param_4,&local_8),
                    CONCAT22(param_4,&local_a));
    local_a += local_12 + -0x6;
    iStack6 = local_a + 0xc;
    local_8 += local_14 + -0x6;
    iStack4 = local_8 + 0xc;
    BVar4 = PtInRect16((RECT16 *)0x1008,PStack24);
    if (BVar4 != 0x0) break;
    iStack26 += 0x1;
  }
  pass1_1018_1eda(param_1,uStack14,param_4);
  return;
}


fn get_dc_1018_4db0(Uparam_1: i32,param_2: u16,HWND16 param_3)
{
  HDC16 HVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  (param_1 + 0x16) = param_2;
  HVar1 = GetDC16(param_3);
  *(HDC16 *)(param_1 + 0x14) = HVar1;
  return;
}


void 
create_dc_1018_4e04(astruct_8 **param_1,param_2: u16,param_3: i16,param_4: i16,
                   LPCSTR in_string_5,u16 in_string_6)

{
  code **ppcVar1;
  astruct_8 *paVar2;
  astruct_9 *iVar4;
  let uVar3: u16;
  let uVar4: u32;
  let iStack16: i16;
  
  uVar3 = (param_1 >> 0x10);
  iVar4 = (astruct_9 *)param_1;
  ppcVar1 = (*param_1 + 0x14);
  (**ppcVar1)();
  uVar4 = pass1_1008_4772((astruct_76 *)iVar4.field_0xa);
  pass1_1008_43cc(iVar4.field_0xa);
  paVar2 = (astruct_8 *)
           CreateDC16((LPCSTR)0x1008,(LPCSTR)uVar4,(LPCSTR)(uVar4 >> 0x10),(DEVMODEA *)0x0
                     );
  iVar4.field_0x12 = paVar2;
  paVar2 = (astruct_8 *)&iVar4.field_0x12;
  ppcVar1 = (*iVar4.field_0xa + 0x8);
  (**ppcVar1)();
  iVar4.field_0x1a = paVar2;
  if ((DAT_1050_422e != 0x0) && (0x280 < param_4)) {
    for (iStack16 = 0x0; iStack16 < DAT_1050_4216 + 0x1; iStack16 += 0x1) {
      (&ctx.PTR_DAT_1050_0009_1050_4172 + iStack16 * 0x2) =
           (((&ctx.PTR_DAT_1050_0009_1050_4172 + iStack16 * 0x2) *
                 (param_4 + 0x1)) / 0x280);
    }
    for (iStack16 = 0x0; iStack16 < DAT_1050_422c + 0x1; iStack16 += 0x1) {
      (&DAT_1050_419a + iStack16 * 0x2) =
           (((&DAT_1050_419a + iStack16 * 0x2) *
                 (param_3 + 0x1)) / 0x1e0);
    }
  }
  DAT_1050_422e = 0x0;
  return;
}


fn invalidate_rect_1018_58e2(astruct_58 *param_1,param_2: i16,HWND16 param_3)
{
  let piVar1: *mut i16;
  astruct_58 *iVar2;
  let uVar2: u16;
  
  if (param_2 == 0x105) {
    uVar2 = (param_1 >> 0x10);
    iVar2 = (astruct_58 *)param_1;
    piVar1 = &iVar2.field_0xf6;
    *piVar1 = *piVar1 + 0x1;
    if (ctx.PTR_DAT_1050_0004_1050_4240 <= iVar2.field_0xf6) {
      PostMessage16(param_3,0x0,0x0,0x11100ca);
      return;
    }
    iVar2.field_0xea = 0x0;
    InvalidateRect16(param_3,(RECT16 *)0x0,0x0);
  }
  return;
}


fn invalidate_rect_1018_5d32(param_1: u32,param_2: i16,HWND16 param_3)
{
  if (param_2 == 0x1) {
    (param_1 + 0x14) = 0x0;
    return;
  }
  if (param_2 != 0x2) {
    return;
  }
  InvalidateRect16(param_3,(RECT16 *)0x0,param_1 + 0x22);
  return;
}



fn misc_draw_op_1018_5d6c(param_1: u32,HWND16 param_2)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  let unaff_SS: u16;
  astruct_76 *paVar6;
  let uVar7: u16;
  PAINTSTRUCT16 local_22;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar7 = (iVar4 + 0x4);
  BeginPaint16(param_2,&local_22);
  uVar3 = (iVar4 + 0x14);
  puVar1 = (uVar3 + 0xa);
  paVar6 = (astruct_76 *)pass1_1008_9f48((iVar4 + 0x14));
  pass1_1008_5236((iVar4 + 0x18));
  pass1_1008_4480(puVar1,(param_1 & 0xffff0000 | (iVar4 + 0x1c)),
                  paVar6,unaff_SS);
  ppcVar2 = (*puVar1 + 0x4);
  (**ppcVar2)(0x1008,puVar1,(puVar1 >> 0x10),0x0,
              param_1 & 0xffff0000 | (iVar4 + 0xa),uVar7);
  EndPaint16(0x1008,&local_22);
  return;
}


fn unk_draw_op_1018_623e(param_1: u32,HWND16 param_2,param_3: u16)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u32;
  let puVar4: u32;
  let uVar5: u16;
  HDC16 *pHVar6;
  let iVar7: i16;
  HPEN16 handle;
  HGDIOBJ16 HVar8;
  HBRUSH16 handle_00;
  let puVar9: *mut u8
  let uVar10: u16;
  let iVar11: i16;
  let iVar12: i16;
  let puVar13: *mut u8;
  let uVar14: u16;
  let uVar15: u16;
  let style: i16;
  let uVar16: u32;
  let uVar17: u8;
  let uVar18: u8;
  let iVar19: i16;
  let uVar20: u8;
  let uVar21: u8;
  let local_38: [u8;6];
  let local_32: u16;
  let uStack48: u16;
  let uStack46: u32;
  let uStack42: u16;
  let puStack40: u32;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  puVar13 = &stack0xfffe;
  uVar14 = (param_1 >> 0x10);
  iVar11 = param_1;
  uVar15 = (iVar11 + 0x4);
  local_24 = BeginPaint16(param_2,&local_22);
  puStack40 = pass1_1010_4c2c((iVar11 + 0x6));
  pHVar6 = &local_24;
  ppcVar2 = (*puStack40 + 0x8);
  (**ppcVar2)(0x1010,puStack40,(puStack40 >> 0x10),pHVar6,param_3,uVar15)
  ;
  *(HDC16 **)(iVar11 + 0x10) = pHVar6;
  uVar3 = (iVar11 + 0x6);
  uStack42 = (uVar3 + 0x30);
  uVar3 = (iVar11 + 0x6);
  uStack46 = (uVar3 + 0x12);
  uStack48 = 0x14;
  local_32 = 0x0;
  style = 0x1008;
  pass1_1008_3e38(CONCAT22(param_3,local_38));
  while ((puVar13 + -0x38) < (puVar13 + -0x28)) {
    iVar11 = (puVar13 + -0x38) * 0x4;
    uVar3 = (puVar13 + -0x2c);
    uVar16 = pass1_1008_4772(*(astruct_76 **)(iVar11 + uVar3));
    puVar9 = (uVar16 >> 0x10);
    (puVar13 + -0x44) = uVar16;
    *(uchar **)(puVar13 + -0x42) = puVar9;
    uVar3 = (puVar13 + 0x6);
    pass1_1018_642e(uVar3,(uVar3 >> 0x10),
                    CONCAT13((param_3 >> 0x8),
                                    CONCAT12(param_3,puVar13 + -0x30)),
                    (uVar16 + 0x8));
    uVar3 = (puVar13 + -0x30);
    pass1_1008_3e76(CONCAT22(param_3,puVar13 + -0x36),0x0,uVar3,
                    (uVar3 >> 0x10));
    uVar3 = (puVar13 + -0x2c);
    pass1_1008_4480((puVar13 + -0x26),
                    CONCAT22(param_3,puVar13 + -0x36),
                    *(astruct_76 **)(uVar3 + iVar11),param_3);
    iVar11 = (puVar13 + -0x38);
    uVar3 = (puVar13 + -0x30);
    uVar14 = uVar3;
    uVar20 = (uVar3 >> 0x10);
    uVar21 = (uVar3 >> 0x18);
    uVar3 = (puVar13 + -0x44);
    uVar15 = (uVar3 >> 0x10);
    iVar12 = uVar3;
    iVar7 = (iVar12 + 0x4) + (puVar13 + -0x2e);
    iVar12 = (iVar12 + 0x8) + (puVar13 + -0x30);
    uVar3 = (puVar13 + 0x6);
    uVar3 = (uVar3 + 0x6);
    iVar19 = uVar3;
    uVar15 = (uVar3 >> 0x10);
    uVar17 = 0x8;
    uVar18 = 0x10;
    if (*(long *)(iVar19 + 0x1a) == 0x0) {
      uVar5 = (iVar19 + 0x30) << 0x3;
      mem_op_1000_179c(uVar5,puVar9,0x1000);
      (iVar19 + 0x1a) = uVar5;
      *(uchar **)(iVar19 + 0x1c) = puVar9;
    }
    uVar3 = (iVar19 + 0x1a);
    iVar11 *= 0x8;
    (uVar3 + iVar11) = CONCAT11(uVar21,uVar20);
    uVar3 = (iVar19 + 0x1a);
    (uVar3 + iVar11 + 0x2) = uVar14;
    uVar3 = (iVar19 + 0x1a);
    (uVar3 + iVar11 + 0x4) = iVar7;
    uVar3 = (iVar19 + 0x1a);
    (uVar3 + iVar11 + 0x6) = iVar12;
    style = CONCAT11(uVar18,uVar17);
    uVar3 = (puVar13 + -0x44);
    piVar1 = (puVar13 + -0x2e);
    *piVar1 = *piVar1 + (-((puVar13 + -0x38) == 0x0) & 0x5) + 0x14 +
                        (uVar3 + 0x4);
    piVar1 = (puVar13 + -0x38);
    *piVar1 = *piVar1 + 0x1;
  }
  puVar4 = *(u32 **)(puVar13 + -0x26);
  ppcVar2 = (*puVar4 + 0x4);
  (**ppcVar2)(style,puVar4,(puVar4 >> 0x10),0x0,0x0,puVar13 + -0x22
              ,param_3);
  handle = CreatePen16(style,0x25,0x100);
  *(HPEN16 *)(puVar13 + -0x3a) = handle;
  HVar8 = SelectObject16(s_tile2_bmp_1050_1538,handle);
  *(HGDIOBJ16 *)(puVar13 + -0x3c) = HVar8;
  handle_00 = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  *(HBRUSH16 *)(puVar13 + -0x3e) = handle_00;
  HVar8 = SelectObject16(s_tile2_bmp_1050_1538,handle_00);
  *(HGDIOBJ16 *)(puVar13 + -0x40) = HVar8;
  draw_line_1018_6444((puVar13 + 0x6),s_tile2_bmp_1050_1538);
  uVar3 = (puVar13 + 0x6);
  uVar16 = pass1_1010_4dc8((uVar3 + 0x6));
  uVar10 = (uVar16 >> 0x10);
  uVar5 = uVar16;
  draw_op_1018_6544((puVar13 + 0x6),
                    (uVar16 & 0xffff | uVar10 << 0x10),param_3);
  pass1_1018_6630((puVar13 + 0x6),uVar5,uVar10);
  uVar3 = (puVar13 + 0x6);
  SelectPalette16(0x1010,0x0,*(bool *)(uVar3 + 0x10));
  DeleteObject16(s_tile2_bmp_1050_1538);
  SelectObject16(s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar13 + -0x3c));
  DeleteObject16(s_tile2_bmp_1050_1538);
  SelectObject16(s_tile2_bmp_1050_1538,*(HGDIOBJ16 *)(puVar13 + -0x40));
  DeleteObject16(s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,(PAINTSTRUCT16 *)(puVar13 + -0x20));
  return;
}


fn draw_line_1018_6444(param_1: u32,HDC16 param_2)
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



fn draw_op_1018_6544(param_1: u32,i16 *param_2,param_3: u16)
{
  let puVar1: *mut u16;
  let uVar2: u32;
  let uVar3: u16;
  let local_a: [u8;6];
  let uStack4: u16;
  
  if (param_2 != 0x0) {
    uStack4 = ((param_2 + 0x4) - *param_2 >> 0x1) + *param_2;
    puVar1 = pass1_1008_3e54(CONCAT22(param_3,local_a),0x0,0x57,uStack4);
    uVar3 = (param_1 >> 0x10);
    uVar2 = pass1_1018_659a(param_1,uVar3,CONCAT22(param_3,local_a),
                            (puVar1 >> 0x10),param_3);
    draw_polygon_1018_661c(param_1,uVar3,CONCAT22(uVar2,0x3),0x1008);
  }
  return;
}


fn draw_polygon_1018_661c(param_1: u16,param_2: u16,param_3: u32,HDC16 param_4)
{
  Polygon16(param_4,(POINT16 *)param_3,(param_3 >> 0x10));
  return;
}


fn mixed_draw_op_1018_6a7a(Struct28 *param_1,param_2: u16)
{
  let in_DX: *mut u8
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar1: *mut u16;
  PAINTSTRUCT16 local_22;
  
  BeginPaint16(param_2,&local_22);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,unaff_SS,in_DX,unaff_DI);
  if ((puVar1 + 0x20) == 0x0) {
    unk_destroy_window_op_1018_6bb6(param_1,0x1010);
    return;
  }
  mix_ui_op_1018_6adc(param_1);
  return;
}


fn unk_draw_op_1018_c578(astruct_36 *param_1,param_2: u16)
{
  let uVar1: u16;
  let iVar2: i16;
  let iVar3: i16;
  astruct_76 *paVar4;
  code **ppcVar5;
  let uVar6: u32;
  let uVar7: u16;
  HDC16 *b_force_background;
  let iVar8: i16;
  let in_DX: *mut u8
  let uVar9: u16;
  let uVar10: u16;
  let extraout_DX: u16;
  let iVar11: i16;
  let uVar12: u16;
  let unaff_DI: i16;
  let uVar13: u16;
  let uVar14: u16;
  HWND16 hwnd;
  let uVar15: u32;
  HDC16 *pHVar16;
  RECT16 *pRVar18;
  let uVar17: u32;
  HDC16 HVar19;
  let local_34: u32;
  let iStack48: i16;
  let iStack46: i16;
  RECT16 *pRStack44;
  HDC16 local_2a;
  let uStack40: u16;
  let puStack38: *mut u16;
  PAINTSTRUCT16 local_22;
  
  hwnd = 0x1010;
  puStack38 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  uVar9 = (puStack38 >> 0x10);
  uVar7 = (puStack38 + 0x20);
  iVar11 = param_1;
  uVar13 = (param_1 >> 0x10);
  uStack40 = uVar7;
  if (uVar7 == 0x0) {
    BeginPaint16(0x1010,&local_22);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
    PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,
                  CONCAT22(0x111,(iVar11 + 0xea)));
    return;
  }
  if (((iVar11 + 0xf0) == 0x0) && ((iVar11 + 0xf4) != 0x0)) {
    (iVar11 + 0xf0) = 0x1;
    uVar1 = iVar11 + 0xf2;
    hwnd = 0x1008;
    win_1008_5c9e(_PTR_LOOP_1050_02a0,
                  (param_1 & 0xffff0000 | uVar1),uVar1,uVar9,
                  param_2);
    uVar7 = uVar1;
  }
  if ((_PTR_LOOP_1050_02a0 + 0x12) == 0x0) {
    hwnd = 0x1008;
    win_1008_5c5c((WNDCLASS16 *)param_2,uVar7,uVar9,_PTR_LOOP_1050_02a0,0x1d3);
  }
  local_2a = BeginPaint16(hwnd,&local_22);
  pRStack44 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  local_34 = 0x0;
  iStack48 = (iVar11 + 0xf6) + -0x1;
  iStack46 = (iVar11 + 0xf8) + -0x1;
  uVar7 = param_2;
  HVar19 = local_2a;
  FillRect16(s_tile2_bmp_1050_1538,pRStack44,(HBRUSH16)&local_34);
  pRVar18 = pRStack44;
  DeleteObject16(s_tile2_bmp_1050_1538);
  uVar6 = (iVar11 + 0xe2);
  paVar4 = *(astruct_76 **)(uVar6 + 0xe);
  b_force_background = &local_2a;
  uVar17 = CONCAT22(pRVar18,param_2);
  uVar14 = (paVar4 >> 0x10);
  uVar12 = SUB42(paVar4,0x0);
  uVar6 = paVar4;
  ppcVar5 = (uVar6 + 0x8);
  pHVar16 = b_force_background;
  (**ppcVar5)();
  uVar15 = pass1_1008_4772(paVar4);
  uVar10 = (uVar15 >> 0x10);
  iVar2 = (uVar15 + 0x4);
  iVar3 = (uVar15 + 0x8);
  iVar8 = (0x1e0 - iVar3) / 0x2;
  (iVar11 + 0x10c) = iVar8 + iVar3 + (iVar11 + 0x110);
  ppcVar5 = (uVar6 + 0x4);
  (**ppcVar5)(0x1008,paVar4,(iVar11 + 0xfc) + (iVar11 + 0xfe) + iVar8,
              (iVar11 + 0xfa) + (0x280 - iVar2) / 0x2,&local_2a,param_2,uVar12,
              uVar14,pHVar16,uVar17,uVar7,HVar19);
  draw_text_1018_c742(param_1,0x1008,param_2,extraout_DX);
  SelectPalette16(0x1008,0x0,b_force_background);
  DeleteObject16(s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}



fn draw_text_1018_c742(astruct_36 *param_1,HDC16 param_2,RECT16 *param_3,param_4: u16)
{
  let piVar1: *mut i16;
  COLORREF CVar2;
  let iVar3: i16;
  astruct_36 *iVar4;
  astruct_36 *uVar4;
  let local_1a: u16;
  let uStack24: u16;
  let iStack22: i16;
  let iStack20: i16;
  let local_12: i16;
  let iStack16: i16;
  let iStack14: i16;
  let iStack12: i16;
  COLORREF CStack10;
  let uStack8: u16;
  let uStack6: u32;
  
  uVar4 = (astruct_36 *)(param_1 >> 0x10);
  iVar4 = (astruct_36 *)param_1;
  if ((iVar4.field_0x108 != 0x0) && (*iVar4.field_0x108 != '\0')) {
    CVar2 = SetTextColor16(param_2,0x8000);
    uStack6 = CONCAT22(param_4,CVar2);
    CStack10 = SetBkColor16(s_tile2_bmp_1050_1538,0x0);
    uStack8 = param_4;
    if (iVar4.field_0x106 == 0x0) {
      local_1a = 0x0;
      uStack24 = 0x0;
      iStack22 = iVar4.field_0x10e;
      iStack20 = 0x0;
      DrawText16(s_tile2_bmp_1050_1538,(LPCSTR)0x410,&local_1a,param_3,
                 0xffff);
      iVar4.field_0x100 = (0x280 - iStack22) / 0x2;
      iVar4.field_0x102 = iVar4.field_0x10c;
      iVar4.field_0x104 = iVar4.field_0x100 + iStack22;
      iVar3 = iVar4.field_0x102 + iStack20;
      iVar4.field_0x106 = iVar3;
      piVar1 = &iVar4.field_0xf8;
      if (*piVar1 == iVar3 || *piVar1 < iVar3) {
        iVar3 -= iVar4.field_0xf8;
        piVar1 = &iVar4.field_0x102;
        *piVar1 = *piVar1 - iVar3;
        piVar1 = &iVar4.field_0x106;
        *piVar1 = *piVar1 - iVar3;
      }
    }
    local_12 = iVar4.field_0xfa + iVar4.field_0x100;
    iStack16 = iVar4.field_0xfc + iVar4.field_0x102;
    iStack14 = iVar4.field_0xfa + iVar4.field_0x104;
    iStack12 = iVar4.field_0xfc + iVar4.field_0x106;
    DrawText16(s_tile2_bmp_1050_1538,(LPCSTR)&ctx.PTR_LOOP_1050_0010,&local_12,
               param_3,0xffff);
    SetTextColor16(s_tile2_bmp_1050_1538,(COLORREF)uStack6);
    SetBkColor16(s_tile2_bmp_1050_1538,CStack10);
  }
  return;
}


fn unk_draw_op_1018_cda8(astruct_36 *param_1,param_2: u16)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  astruct_76 *paVar3;
  code **ppcVar4;
  let uVar5: u16;
  HDC16 *b_force_background;
  let iVar6: i16;
  let iVar7: i16;
  let in_DX: *mut u8
  let uVar8: u16;
  let uVar9: u16;
  let extraout_DX: u16;
  let iVar10: i16;
  let unaff_DI: i16;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  HWND16 hwnd;
  let uVar14: u32;
  let uVar15: u16;
  let uVar16: u16;
  HDC16 *pHVar17;
  RECT16 *pRVar19;
  let uVar18: u32;
  let local_34: u32;
  let iStack48: i16;
  let iStack46: i16;
  RECT16 *pRStack44;
  HDC16 local_2a;
  let uStack40: u16;
  let puStack38: *mut u16;
  PAINTSTRUCT16 local_22;
  
  hwnd = 0x1010;
  puStack38 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  uVar8 = (puStack38 >> 0x10);
  uVar5 = (puStack38 + 0x20);
  iVar10 = param_1;
  uVar11 = (param_1 >> 0x10);
  uStack40 = uVar5;
  if (uVar5 == 0x0) {
    BeginPaint16(0x1010,&local_22);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
    PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,
                  CONCAT22(0x111,(iVar10 + 0xea)));
    return;
  }
  if ((iVar10 + 0xf0) == 0x0) {
    (iVar10 + 0xf0) = 0x1;
    hwnd = 0x1008;
    win_1008_5c5c((WNDCLASS16 *)param_2,uVar5,uVar8,_PTR_LOOP_1050_02a0,0x1f3);
    uVar12 = (_PTR_LOOP_1050_02a0 >> 0x10);
    if ((_PTR_LOOP_1050_02a0 + 0x12) == 0x0) {
      hwnd = 0x1008;
      win_1008_5c5c((WNDCLASS16 *)param_2,uVar5,uVar8,
                    _PTR_LOOP_1050_02a0 & 0xffff | uVar12 << 0x10,0x1d3);
    }
  }
  local_2a = BeginPaint16(hwnd,&local_22);
  pRStack44 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  local_34 = 0x0;
  iStack48 = (iVar10 + 0xf6) + -0x1;
  iStack46 = (iVar10 + 0xf8) + -0x1;
  FillRect16(s_tile2_bmp_1050_1538,pRStack44,(HBRUSH16)&local_34);
  pRVar19 = pRStack44;
  DeleteObject16(s_tile2_bmp_1050_1538);
  uVar18 = (iVar10 + 0xe2);
  paVar3 = *(astruct_76 **)(uVar18 + 0xe);
  b_force_background = &local_2a;
  uVar18 = CONCAT22(pRVar19,param_2);
  uVar13 = (paVar3 >> 0x10);
  ppcVar4 = (paVar3 + 0x8);
  uVar15 = paVar3;
  uVar16 = uVar13;
  pHVar17 = b_force_background;
  (**ppcVar4)();
  uVar14 = pass1_1008_4772(paVar3);
  uVar9 = (uVar14 >> 0x10);
  iVar6 = (0x280 - (uVar14 + 0x4)) / 0x2;
  iVar2 = (uVar14 + 0x8);
  iVar7 = (0x1e0 - iVar2) / 0x2;
  (iVar10 + 0x10c) = iVar7 + iVar2 + (iVar10 + 0x110);
  if (((iVar10 + 0xfa) == 0x0) && (iVar6 == 0x0)) {
    piVar1 = (iVar10 + 0xfa);
    *piVar1 = *piVar1 + 0x2;
  }
  ppcVar4 = (paVar3 + 0x4);
  (**ppcVar4)(0x1008,paVar3,uVar13,
              (iVar10 + 0xfc) + (iVar10 + 0xfe) + iVar7,
              (iVar10 + 0xfa) + iVar6,&local_2a,param_2,uVar15,uVar16,pHVar17,
              uVar18);
  draw_text_1018_c742(param_1,0x1008,param_2,extraout_DX);
  SelectPalette16(0x1008,0x0,b_force_background);
  DeleteObject16(s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}


fn unk_draw_op_1018_cfc0(astruct_36 *param_1,param_2: u16)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  astruct_76 *paVar3;
  code **ppcVar4;
  let uVar5: u16;
  HDC16 *b_force_background;
  let iVar6: i16;
  let iVar7: i16;
  let in_DX: *mut u8
  let uVar8: u16;
  let uVar9: u16;
  let extraout_DX: u16;
  let iVar10: i16;
  let unaff_DI: i16;
  let uVar11: u16;
  let uVar12: u16;
  HWND16 hwnd;
  let uVar13: u32;
  let uVar14: u16;
  let uVar15: u16;
  HDC16 *pHVar16;
  RECT16 *pRVar18;
  let uVar17: u32;
  HDC16 HVar19;
  let local_34: u32;
  let iStack48: i16;
  let iStack46: i16;
  RECT16 *pRStack44;
  HDC16 local_2a;
  let iStack40: i16;
  let puStack38: *mut u16;
  PAINTSTRUCT16 local_22;
  
  hwnd = 0x1010;
  puStack38 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  uVar8 = (puStack38 >> 0x10);
  iStack40 = (puStack38 + 0x20);
  iVar10 = param_1;
  uVar11 = (param_1 >> 0x10);
  if (iStack40 == 0x0) {
    BeginPaint16(0x1010,&local_22);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
    PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,
                  CONCAT22(0x111,(iVar10 + 0xea)));
    return;
  }
  if (((iVar10 + 0xf0) == 0x0) && ((iVar10 + 0xf4) != 0x0)) {
    (iVar10 + 0xf0) = 0x1;
    uVar5 = iVar10 + 0xf2;
    hwnd = 0x1008;
    win_1008_5c9e(_PTR_LOOP_1050_02a0,
                  (param_1 & 0xffff0000 | uVar5),uVar5,uVar8,
                  param_2);
    if ((_PTR_LOOP_1050_02a0 + 0x12) == 0x0) {
      hwnd = 0x1008;
      win_1008_5c5c((WNDCLASS16 *)param_2,uVar5,uVar8,_PTR_LOOP_1050_02a0,0x1d3);
    }
  }
  local_2a = BeginPaint16(hwnd,&local_22);
  pRStack44 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  local_34 = 0x0;
  iStack48 = (iVar10 + 0xf6) + -0x1;
  iStack46 = (iVar10 + 0xf8) + -0x1;
  uVar8 = param_2;
  HVar19 = local_2a;
  FillRect16(s_tile2_bmp_1050_1538,pRStack44,(HBRUSH16)&local_34);
  pRVar18 = pRStack44;
  DeleteObject16(s_tile2_bmp_1050_1538);
  uVar17 = (iVar10 + 0xe2);
  paVar3 = *(astruct_76 **)(uVar17 + 0xe);
  b_force_background = &local_2a;
  uVar17 = CONCAT22(pRVar18,param_2);
  uVar12 = (paVar3 >> 0x10);
  ppcVar4 = (paVar3 + 0x8);
  uVar14 = paVar3;
  uVar15 = uVar12;
  pHVar16 = b_force_background;
  (**ppcVar4)();
  uVar13 = pass1_1008_4772(paVar3);
  uVar9 = (uVar13 >> 0x10);
  iVar6 = (0x280 - (uVar13 + 0x4)) / 0x2;
  iVar2 = (uVar13 + 0x8);
  iVar7 = (0x1e0 - iVar2) / 0x2;
  (iVar10 + 0x10c) = iVar7 + iVar2 + (iVar10 + 0x110);
  if (((iVar10 + 0xfa) == 0x0) && (iVar6 == 0x0)) {
    piVar1 = (iVar10 + 0xfa);
    *piVar1 = *piVar1 + 0x2;
  }
  ppcVar4 = (paVar3 + 0x4);
  (**ppcVar4)(0x1008,paVar3,uVar12,
              (iVar10 + 0xfc) + (iVar10 + 0xfe) + iVar7,
              (iVar10 + 0xfa) + iVar6,&local_2a,param_2,uVar14,uVar15,pHVar16,
              uVar17,uVar8,HVar19);
  draw_text_1018_c742(param_1,0x1008,param_2,extraout_DX);
  SelectPalette16(0x1008,0x0,b_force_background);
  DeleteObject16(s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}


fn invalidate_rect_1018_edd8(param_1: u32,param_2: i16,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u32;
  let local_16: i16;
  let iStack20: i16;
  let iStack18: i16;
  let iStack16: i16;
  let uStack14: u32;
  let uStack10: u16;
  let uStack8: u16;
  let local_6: i16;
  let local_4: i16;
  
  iVar1 = param_1;
  uVar2 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (iVar1 + 0x14) = 0x0;
    return;
  }
  if (param_2 != 0xc) {
    return;
  }
  pass1_1008_3e94((param_1 & 0xffff0000 | (iVar1 + 0x18)),
                  CONCAT22(param_3,&local_6),
                  CONCAT22(param_3,&local_4));
  uVar3 = pass1_1010_2b66((iVar1 + 0x14));
  uStack8 = (uVar3 >> 0x10);
  uStack10 = uVar3;
  uStack14 = pass1_1008_4772((astruct_76 *)(uVar3 & 0xffff | uStack8 << 0x10));
  uVar2 = (uStack14 >> 0x10);
  local_16 = local_4;
  iStack20 = local_6;
  iStack18 = local_4 + (uStack14 + 0x4);
  iStack16 = local_6 + (uStack14 + 0x8);
  InvalidateRect16(0x1008,(RECT16 *)0x0,&local_16);
  return;
}
