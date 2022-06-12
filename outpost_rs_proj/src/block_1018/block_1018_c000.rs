
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_c402(mut param_1: u16 ,param_2: *mut astruct_20,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u32,
                    mut param_7: u32,mut param_8: u32,mut param_9: u32)

{
  let mut iVar1: i16;
  let mut puVar2: *mut u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  astruct_20 *iVar4;
  astruct_20 *uVar4;
  u32 *puVar4;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffe6: u16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  struct_1020_0762(param_2,CONCAT22((int)param_6,param_5),(u32 *)CONCAT22((int)param_7,(int)(param_6 >> 0x10)),
                   (param_7 >> 0x10),param_8,param_9);
  uVar4 = (astruct_20 *)((u32)param_2 >> 0x10);
  iVar4 = (astruct_20 *)param_2;
  iVar4[0x1].field8_0x14 = 0x0;
  iVar4[0x1].field9_0x16 = NULL;
  iVar4[0x1].field10_0x18 = 0x0;
  iVar4[0x1].field11_0x1a = 0x0;
  iVar4[0x1].field12_0x1c = 0x2;
  iVar4[0x1].field15_0x26 = 0x0;
  iVar4[0x1].field16_0x2a = 0x0;
  iVar4[0x1].field17_0x2c = 0x1e0190;
  iVar4[0x1].field18_0x30 = 0x0;
  param_2->offset_0x0 = 0xc8bc;
  iVar4->base_0x2 = 0x1018;
  puVar2 = pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar4[0x1].field13_0x1e)),NULL,0x8);
  if ((param_4 == 0x0) || (param_3 != 0x0)) {
    if ((param_3 & param_4) == 0x0) goto LAB_1018_c4bb;
    puVar2 = (u16 *)pass1_1008_5fd8(paVar3);
  }
  else {
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),param_4);
  }
  (u16*)&iVar4[0x1].field15_0x26 = puVar2;
  ((int)&iVar4[0x1].field15_0x26 + 0x2) = (int)paVar3;//
LAB_1018_c4bb:
  puVar4 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe6,0x48),in_stack_0000fe8e,
                           in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  iVar1 = (int)puVar4;
  iVar4[0x1].field8_0x14 = (iVar1 + 0xa);
  iVar4[0x1].field9_0x16 = *(astruct_19 **)(iVar1 + 0xc);
  pass1_1008_3e94((u16 *)((u32)puVar4 & 0xffff0000 | (u32)(iVar1 + 0xe)),
                  (u16 *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar4[0x1].field11_0x1a)),
                  (char *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar4[0x1].field10_0x18)));
  return;
}
pub fn destroy_window_1018_c518(param_1: *mut astruct_29)

{
  let mut is_window: bool;
  astruct_29 *pstruct_29_1;
  astruct_29 *pstruct_29_hi;

  pstruct_29_hi = (astruct_29 *)((u32)param_1 >> 0x10);
  pstruct_29_1 = (astruct_29 *)param_1;
  param_1->field0_0x0 = 0xc8bc;
  pstruct_29_1->field1_0x2 = 0x1018;
  fn_ptr_1000_17ce(pstruct_29_1->field259_0x108);
  if (pstruct_29_1->hwnd_0x112 != NULL) {
    is_window = IsWindow16((HWND16)pstruct_29_1->hwnd_0x112);
    if (is_window != 0x0) {
      DestroyWindow16((HWND16)pstruct_29_1->hwnd_0x112);
      pstruct_29_1->hwnd_0x112 = NULL;
    }
  }
  pass1_1020_022c(param_1);
  return;
}



// WARNING: Unable to use type for symbol iVar2
// WARNING: Unable to use type for symbol uVar3
// WARNING: Unable to use type for symbol puVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_draw_op_1018_c578(param_1: *mut astruct_57,param_2: *mut astruct_36)

{
  astruct_76 *paVar1;
  let mut uVar2: u16;
  let mut uVar5: u16;
  HDC16 *hpal;
  let mut iVar5: i16;
  let mut iVar3: i16;
  let mut uVar6: u16;
  let mut uVar9: u16;
  let mut uVar7: u16;
  let mut extraout_DX: u16;
  HPALETTE16 obj;
  astruct_36 *iVar4;
  let mut uVar10: u16;
  let mut unaff_SI: u16;
  astruct_36 *uVar8;
  let mut uVar11: u16;
  let mut uVar12: u32;
  let mut in_stack_0000fe56: u16;
  let mut in_stack_0000ff7a: u16;
  let mut in_stack_0000ff80: u16;
  let mut in_stack_0000ff84: u16;
  RECT16 rect_34;
  let mut iStack48: i16;
  let mut iStack46: i16;
  HBRUSH16 hbrush_44;
  HDC16 hdc_2a;
  let mut uStack40: u16;
  u32 *puStack38;
  PAINTSTRUCT16 paintstruct_22;
  let mut uVar1: u16;
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u32;
  let mut uVar4: u32;
  u8 *puVar3;
  code **fn_ptr_1;

  puStack38 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fe56,
                              in_stack_0000ff7a,in_stack_0000ff80,in_stack_0000ff84);
  uVar9 = ((u32)puStack38 >> 0x10);
  uVar5 = ((int)puStack38 + 0x20);
  iVar4 = (astruct_36 *)param_2;
  uVar8 = (astruct_36 *)((u32)param_2 >> 0x10);
  uStack40 = uVar5;
  if (uVar5 == 0x0) {
    BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar4->hwnd_0x8);
    EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar4->hwnd_0x8);
    PostMessage16(0x0,iVar4->wparam_0xea,0x111,HWND16_1050_0396);
    return;
  }
  if ((iVar4->field235_0xf0 == 0x0) && (iVar4->field238_0xf4 != 0x0)) {
    iVar4->field235_0xf0 = 0x1;
    puVar3 = &iVar4->field_0xf2;
    win_1008_5c9e(puVar3,uVar9,_u16_1050_02a0,(u32 *)((u32)param_2 & 0xffff0000 | ZEXT24(puVar3)));
    uVar5 = puVar3;
  }
  if (((int)_u16_1050_02a0 + 0x12) == 0x0) {
    win_1008_5c5c(uVar5,uVar9,_u16_1050_02a0,0x1d3);
  }
  hdc_2a = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar4->hwnd_0x8);
  hbrush_44 = CreateSolidBrush16(0x2000000);
  rect_34 = (RECT16)0x0;
  iStack48 = iVar4->field239_0xf6 + -0x1;
  iStack46 = iVar4->field240_0xf8 + -0x1;
  FillRect16(hbrush_44,&rect_34,(HDC16)&DAT_1050_1050);
  DeleteObject16(hbrush_44);
  uVar3 = iVar4->field225_0xe2;
  paVar1 = *(astruct_76 **)((int)uVar3 + 0xe);
  hpal = &hdc_2a;
  uVar11 = ((u32)paVar1 >> 0x10);
  uVar10 = paVar1;
  uVar4 = (u32)paVar1;
  fn_ptr_1 = (code **)((int)uVar4 + 0x8);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,uVar10,uVar11,hpal,(int)&DAT_1050_1050);
  uVar12 = pass1_1008_4772(paVar1);
  uVar2 = (uVar12 >> 0x10);
  iVar1 = ((int)uVar12 + 0x4);
  iVar2 = ((int)uVar12 + 0x8);
  iVar5 = 0x1e0 - iVar2;
  extraout_DX = iVar5 >> 0xf;
  iVar3 = iVar5 / 0x2;
  iVar4->field249_0x10c = iVar3 + iVar2 + iVar4->field251_0x110;
  fn_ptr_1 = (code **)((int)uVar4 + 0x4);
  (**fn_ptr_1)(0x1008,uVar10,uVar11,iVar4->field242_0xfc + iVar4->field243_0xfe + iVar3,
               iVar4->field241_0xfa + (0x280 - iVar1) / 0x2,(char)&hdc_2a,(int)&DAT_1050_1050);
  draw_text_1018_c742(extraout_DX,param_2,(HDC16)&hdc_2a,(i16)&DAT_1050_1050,uVar10);
  obj = SelectPalette16(0x0,(HPALETTE16)hpal,hdc_2a);
  DeleteObject16(obj);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar4->hwnd_0x8);
  return;
}



// WARNING: Variable defined which should be unmapped: iStack22
// WARNING: Variable defined which should be unmapped: iStack20
pub fn draw_text_1018_c742(mut param_1: u16 ,astruct_36 *struct36_param_1,HDC16 hdc_2,i16 count_param_3,mut param_5: u16 )

{
  let mut piVar2: *mut i16;
  let mut iVar3: i16;
  u8 extraout_AH;
  u8 uVar3;
  let mut iVar5: i16;
  let mut iVar1: i16;
  astruct_36 *pstruct36_4;
  astruct_36 *pstruct36_hi;
  COLORREF color;
  let mut iStack22: i16;
  let mut iStack20: i16;
  RECT16 rect_12;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut piVar1: *mut i16;

  pstruct36_hi = (astruct_36 *)((u32)struct36_param_1 >> 0x10);
  pstruct36_4 = (astruct_36 *)struct36_param_1;
  if ((pstruct36_4->string_0x108 != NULL) && (*pstruct36_4->string_0x108 != '\0')) {
    uVar3 = SetTextColor16(0x8000,*_hdc_2);
    color = SetBkColor16(0x2000000,*_hdc_2);
    if (pstruct36_4->field247_0x106 == 0x0) {
      iVar3 = pstruct36_4->field250_0x10e;
      DrawText16(0x410,(RECT16 *)CONCAT22(0x1050,&stack0xffe6),-0x1,pstruct36_4->string_0x108,*_hdc_2);
      pstruct36_4->field244_0x100 = (0x280 - iVar3) / 0x2;
      pstruct36_4->field245_0x102 = pstruct36_4->field249_0x10c;
      pstruct36_4->field246_0x104 = pstruct36_4->field244_0x100 + iVar3;
      iVar3 = pstruct36_4->field245_0x102;
      pstruct36_4->field247_0x106 = iVar3;
      piVar1 = &pstruct36_4->field240_0xf8;
      if (*piVar1 == iVar3 || *piVar1 < iVar3) {
        iVar1 = iVar3 - pstruct36_4->field240_0xf8;
        piVar2 = &pstruct36_4->field245_0x102;
        *piVar2 = *piVar2 - iVar1;
        piVar2 = &pstruct36_4->field247_0x106;
        *piVar2 = *piVar2 - iVar1;
      }
    }
    rect_12.x = pstruct36_4->field241_0xfa + pstruct36_4->field244_0x100;
    rect_12.y = pstruct36_4->field242_0xfc + pstruct36_4->field245_0x102;
    DrawText16(0x10,(RECT16 *)CONCAT22(0x1050,&rect_12),-0x1,pstruct36_4->string_0x108,*_hdc_2);
    SetTextColor16(CONCAT22(param_1,CONCAT11(extraout_AH,uVar3)),*_hdc_2);
    SetBkColor16(color,*_hdc_2);
  }
  return;
}



astruct_29 * pass1_1018_c896(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_20 * pass1_1018_c958(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_6 [0x4];

  uVar3 = 0xf1;
  uVar4 = 0x9a;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x8d);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x732,0x26,CONCAT22((int)puVar2,0x1f40),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xdc5a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_c9a6(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_6 [0x4];

  uVar3 = 0xf2;
  uVar4 = 0xa0;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x8e);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x733,0x27,CONCAT22((int)puVar2,0x1b58),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd6de;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_c9f4(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut puVar4: *mut u16;
  let mut uVar5: u16;
  u8 local_6 [0x4];

  uVar3 = 0xf3;
  uVar5 = 0xa6;
  puVar4 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x8f);
  uVar2 = ((u32)puVar4 >> 0x10);
  pass1_1018_c402(uVar2,param_1,0x0,0x734,0x28,CONCAT22((int)puVar4,0x32c8),CONCAT22(uVar3,uVar2),
                  CONCAT22(param_2,uVar5),param_3);
  uVar3 = ((u32)param_1 >> 0x10);
  param_1->offset_0x0 = 0xda86;
  ((int)param_1 + 0x2) = 0x1018;
  piVar1 = ((int)param_1 + 0x10e);
  *piVar1 = *piVar1 + -0x19;
  return param_1;
}



astruct_20 * pass1_1018_ca48(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_6 [0x4];

  uVar3 = 0xf4;
  uVar4 = 0xa1;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x90);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x735,0x29,CONCAT22((int)puVar2,0x36b0),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd50a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_ca96(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut puVar4: *mut u16;
  let mut uVar5: u16;
  u8 local_6 [0x4];

  uVar3 = 0xf5;
  uVar5 = 0xbf;
  puVar4 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x92);
  uVar2 = ((u32)puVar4 >> 0x10);
  pass1_1018_c402(uVar2,param_1,0x737,0x736,0x2a,CONCAT22((int)puVar4,0x6590),CONCAT22(uVar3,uVar2),
                  CONCAT22(param_2,uVar5),param_3);
  uVar3 = ((u32)param_1 >> 0x10);
  param_1->offset_0x0 = 0xd8b2;
  ((int)param_1 + 0x2) = 0x1018;
  piVar1 = ((int)param_1 + 0x10e);
  *piVar1 = *piVar1 + 0x64;
  return param_1;
}



astruct_20 * pass1_1018_caea(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_6 [0x4];

  uVar3 = 0xf6;
  uVar4 = 0x93;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x93);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x738,0x2b,CONCAT22((int)puVar2,0x2328),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xdbbe;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cb38(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_6 [0x4];

  uVar3 = 0xf7;
  uVar4 = 0x94;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x94);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x739,0x2c,CONCAT22((int)puVar2,0x32c8),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd642;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cb86(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut puVar4: *mut u16;
  let mut uVar5: u16;
  u8 local_6 [0x4];

  uVar3 = 0xf8;
  uVar5 = 0xc2;
  puVar4 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x96);
  uVar2 = ((u32)puVar4 >> 0x10);
  pass1_1018_c402(uVar2,param_1,0x0,0x73a,0x2d,CONCAT22((int)puVar4,0x2328),CONCAT22(uVar3,uVar2),
                  CONCAT22(param_2,uVar5),param_3);
  uVar3 = ((u32)param_1 >> 0x10);
  param_1->offset_0x0 = 0xd9ea;
  ((int)param_1 + 0x2) = 0x1018;
  piVar1 = ((int)param_1 + 0x10e);
  *piVar1 = *piVar1 + 0x64;
  return param_1;
}



astruct_20 * pass1_1018_cbda(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_6 [0x4];

  uVar3 = 0xf9;
  uVar4 = 0xc5;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x97);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x73b,0x2e,CONCAT22((int)puVar2,0x2af8),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd46e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cc28(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  u8 local_6 [0x4];
  let mut uVar3: u16;
  let mut uVar4: u16;

  uVar3 = 0xfa;
  uVar4 = 0xa3;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x98);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x73c,0x2f,CONCAT22((int)puVar2,0x2710),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd816;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cc76(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_6 [0x4];

  uVar3 = 0xfb;
  uVar4 = 0xa8;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x99);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x73e,0x73d,0x30,CONCAT22((int)puVar2,0x61a8),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xdb22;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_ccc4(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_6 [0x4];

  uVar3 = 0xfc;
  uVar4 = 0xa9;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x9b);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x740,0x73f,0x31,CONCAT22((int)puVar2,0x59d8),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd5a6;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cd12(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_6 [0x4];

  uVar3 = 0xfd;
  uVar4 = 0x7c;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x9c);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x741,0x32,CONCAT22((int)puVar2,0x2710),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd94e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cd60(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_6 [0x4];

  uVar3 = 0xfe;
  uVar4 = 0xc9;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x0);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x0,0x33,CONCAT22((int)puVar2,0x2710),CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),
                  param_3);
  param_1->offset_0x0 = 0xd3d2;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



// WARNING: Unable to use type for symbol iVar1
// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_draw_op_1018_cda8(param_1: *mut astruct_57,mut param_2: u16 ,mut param_3: u16 ,astruct_36 *struct36_param_1)

{
  astruct_76 *paVar1;
  let mut uVar9: u16;
  let mut uVar4: u16;
  let mut uVar11: u16;
  let mut uVar7: u16;
  let mut uVar3: u16;
  HDC16 *hpalette_var1;
  let mut iVar4: i16;
  let mut iVar10: i16;
  let mut iVar2: i16;
  HPALETTE16 selected_obj;
  let mut uVar12: u16;
  astruct_36 *struct36_var3;
  let mut uVar13: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar8: u16;
  let mut uVar14: u32;
  let mut in_stack_0000fe5a: u16;
  let mut in_stack_0000ff7e: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff88: u16;
  RECT16 rect_var34;
  let mut iStack48: i16;
  let mut iStack46: i16;
  HBRUSH16 brush_handle_var44;
  HDC16 hdc_2a;
  let mut uStack40: u16;
  u32 *puStack38;
  u8 paintstruct_var_22 [0x20];
  let mut piVar1: *mut i16;
  let mut iVar1: i16;
  let mut uVar2: u32;
  let mut in_stack_0000ffb0: u32;
  let mut uVar10: u16;
  code **fn_ptr_2;

  puStack38 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x2),in_stack_0000fe5a,
                              in_stack_0000ff7e,in_stack_0000ff84,in_stack_0000ff88);
  uVar11 = ((u32)puStack38 >> 0x10);
  uVar3 = ((int)puStack38 + 0x20);
  struct36_var3 = (astruct_36 *)struct36_param_1;
  uVar5 = ((u32)struct36_param_1 >> 0x10);
  uStack40 = uVar3;
  if (uVar3 == 0x0) {
    BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_var_22),struct36_var3->hwnd_0x8);
    EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_var_22),struct36_var3->hwnd_0x8);
    PostMessage16(0x0,struct36_var3->wparam_0xea,0x111,HWND16_1050_0396);
    return;
  }
  if (struct36_var3->field235_0xf0 == 0x0) {
    struct36_var3->field235_0xf0 = 0x1;
    win_1008_5c5c(uVar3,uVar11,_u16_1050_02a0,0x1f3);
    uVar6 = (_u16_1050_02a0 >> 0x10);
    if (((int)_u16_1050_02a0 + 0x12) == 0x0) {
      win_1008_5c5c(uVar3,uVar11,_u16_1050_02a0,0x1d3);
    }
  }
  hdc_2a = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_var_22),struct36_var3->hwnd_0x8);
  brush_handle_var44 = CreateSolidBrush16(0x2000000);
  rect_var34 = (RECT16)0x0;
  iStack48 = struct36_var3->field239_0xf6 + -0x1;
  iStack46 = struct36_var3->field240_0xf8 + -0x1;
  FillRect16(brush_handle_var44,&rect_var34,(HDC16)&DAT_1050_1050);
  DeleteObject16(brush_handle_var44);
  uVar2 = struct36_var3->field225_0xe2;
  paVar1 = *(astruct_76 **)((int)uVar2 + 0xe);
  hpalette_var1 = &hdc_2a;
  uVar8 = ((u32)paVar1 >> 0x10);
  uVar13 = paVar1;
  fn_ptr_2 = (code **)((int)(u32)paVar1 + 0x8);
  (**fn_ptr_2)((int)s_tile2_bmp_1050_1538,uVar13,uVar8,hpalette_var1,(int)&DAT_1050_1050);
  uVar14 = pass1_1008_4772(paVar1);
  uVar9 = (uVar14 >> 0x10);
  iVar4 = (0x280 - ((int)uVar14 + 0x4)) / 0x2;
  iVar1 = ((int)uVar14 + 0x8);
  iVar10 = 0x1e0 - iVar1;
  uVar12 = iVar10 >> 0xf;
  iVar2 = iVar10 / 0x2;
  struct36_var3->field249_0x10c = iVar2 + iVar1 + struct36_var3->field251_0x110;
  if ((struct36_var3->field241_0xfa == 0x0) && (iVar4 == 0x0)) {
    piVar1 = &struct36_var3->field241_0xfa;
    *piVar1 = *piVar1 + 0x2;
  }
  fn_ptr_2 = (code **)((int)(u32)paVar1 + 0x4);
  (**fn_ptr_2)(0x1008,uVar13,uVar8,struct36_var3->field242_0xfc + struct36_var3->field243_0xfe + iVar2,
               struct36_var3->field241_0xfa + iVar4,(char)&hdc_2a,(int)&DAT_1050_1050);
  draw_text_1018_c742(uVar12,struct36_param_1,(HDC16)&hdc_2a,(i16)&DAT_1050_1050,uVar13);
  selected_obj = SelectPalette16(0x0,(HPALETTE16)hpalette_var1,hdc_2a);
  DeleteObject16(selected_obj);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_var_22),struct36_var3->hwnd_0x8);
  return;
}



astruct_20 * pass1_1018_cf74(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_6 [0x4];

  uVar3 = 0xfe;
  uVar4 = 0xcf;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x80);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x0,0x34,CONCAT22((int)puVar2,0x2710),CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),
                  param_3);
  param_1->offset_0x0 = 0xd77a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



// WARNING: Unable to use type for symbol iVar1
// WARNING: Unable to use type for symbol uVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_draw_op_1018_cfc0(param_1: *mut astruct_57,mut param_2: u16 ,astruct_36 *struct36_param_1)

{
  let mut uVar1: u16;
  let mut uVar4: u16;
  HDC16 *hpal;
  let mut iVar3: i16;
  let mut iVar2: i16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar9: u16;
  let mut uVar6: u16;
  HPALETTE16 obj;
  let mut uVar10: u16;
  astruct_36 *struct36_var5;
  let mut uVar11: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar12: u32;
  let mut in_stack_0000fe58: u16;
  let mut in_stack_0000ff7c: u16;
  let mut in_stack_0000ff82: u16;
  let mut in_stack_0000ff86: u16;
  RECT16 rect_34;
  let mut iStack48: i16;
  let mut iStack46: i16;
  HBRUSH16 hbrush_44;
  HDC16 local_2a;
  let mut iStack40: i16;
  u32 *puStack38;
  u8 paintstruct_22 [0x20];
  let mut piVar1: *mut i16;
  let mut iVar1: i16;
  let mut uVar3: u32;
  let mut in_stack_0000ffb0: u16;
  code **fn_ptr_2;
  astruct_76 *struct76_var1;

  puStack38 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x2),in_stack_0000fe58,
                              in_stack_0000ff7c,in_stack_0000ff82,in_stack_0000ff86);
  uVar9 = ((u32)puStack38 >> 0x10);
  iStack40 = ((int)puStack38 + 0x20);
  struct36_var5 = (astruct_36 *)struct36_param_1;
  uVar7 = ((u32)struct36_param_1 >> 0x10);
  if (iStack40 == 0x0) {
    BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct36_var5->hwnd_0x8);
    EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct36_var5->hwnd_0x8);
    PostMessage16(0x0,struct36_var5->wparam_0xea,0x111,HWND16_1050_0396);
    return;
  }
  if ((struct36_var5->field235_0xf0 == 0x0) && (struct36_var5->field238_0xf4 != 0x0)) {
    struct36_var5->field235_0xf0 = 0x1;
    uVar4 = &struct36_var5->field_0xf2;
    win_1008_5c9e(uVar4,uVar9,_u16_1050_02a0,(u32 *)((u32)struct36_param_1 & 0xffff0000 | (u32)uVar4));
    if (((int)_u16_1050_02a0 + 0x12) == 0x0) {
      win_1008_5c5c(uVar4,uVar9,_u16_1050_02a0,0x1d3);
    }
  }
  local_2a = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct36_var5->hwnd_0x8);
  hbrush_44 = CreateSolidBrush16(0x2000000);
  rect_34 = (RECT16)0x0;
  iStack48 = struct36_var5->field239_0xf6 + -0x1;
  iStack46 = struct36_var5->field240_0xf8 + -0x1;
  FillRect16(hbrush_44,&rect_34,(HDC16)&DAT_1050_1050);
  DeleteObject16(hbrush_44);
  uVar3 = struct36_var5->field225_0xe2;
  struct76_var1 = *(astruct_76 **)((int)uVar3 + 0xe);
  hpal = &local_2a;
  uVar8 = ((u32)struct76_var1 >> 0x10);
  uVar11 = struct76_var1;
  fn_ptr_2 = (code **)((int)(u32)struct76_var1 + 0x8);
  (**fn_ptr_2)((int)s_tile2_bmp_1050_1538,uVar11,uVar8,hpal,(int)&DAT_1050_1050);
  uVar12 = pass1_1008_4772(struct76_var1);
  uVar1 = (uVar12 >> 0x10);
  iVar3 = (0x280 - ((int)uVar12 + 0x4)) / 0x2;
  iVar1 = ((int)uVar12 + 0x8);
  iVar2 = 0x1e0 - iVar1;
  uVar10 = iVar2 >> 0xf;
  iVar4 = iVar2 / 0x2;
  struct36_var5->field249_0x10c = iVar4 + iVar1 + struct36_var5->field251_0x110;
  if ((struct36_var5->field241_0xfa == 0x0) && (iVar3 == 0x0)) {
    piVar1 = &struct36_var5->field241_0xfa;
    *piVar1 = *piVar1 + 0x2;
  }
  fn_ptr_2 = (code **)((int)(u32)struct76_var1 + 0x4);
  (**fn_ptr_2)(0x1008,uVar11,uVar8,struct36_var5->field242_0xfc + struct36_var5->field243_0xfe + iVar4,
               struct36_var5->field241_0xfa + iVar3,(char)&local_2a,(int)&DAT_1050_1050);
  draw_text_1018_c742(uVar10,struct36_param_1,(HDC16)&local_2a,(i16)&DAT_1050_1050,uVar11);
  obj = SelectPalette16(0x0,(HPALETTE16)hpal,local_2a);
  DeleteObject16(obj);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct36_var5->hwnd_0x8);
  return;
}

