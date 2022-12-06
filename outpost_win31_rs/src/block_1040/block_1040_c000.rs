
// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
pub fn unk_draw_op_1040_c226(astruct_772 *struct_param_1)

{
  HPEN16 handle;
  HGDIOBJ16 obj_handle_var3;
  astruct_772 *iVar3;
  let mut uVar4: u16;
  HDC16 hdc;
  let mut rect_var_32: RECT16;
  let mut iStack46: i16;
  let mut iStack44: i16;
  let mut uStack42: u16;
  let mut iStack40: i16;
  HBRUSH16 hbrush_var38;
  HDC16 hdc16_var36;
  PAINTSTRUCT16 *paintstruct_22;
  let mut uVar1: u32;
  let mut uVar2: u32;

  uVar4 = (struct_param_1 >> 0x10);
  iVar3 = (astruct_772 *)struct_param_1;
  hdc16_var36 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar3->hwnd_0x4);
  hbrush_var38 = CreateSolidBrush16(0x8000);
  GetClientRect16(&rect_var_32,(HWND16)&DAT_1050_1050);
  uVar1 = iVar3->field5_0x6;
  iStack40 = (uVar1 + 0x1a);
  uVar2 = iVar3->field5_0x6;
  uStack42 = (uVar2 + 0x1c);
  rect_var_32.y += 0x2;
  rect_var_32.x = iStack40 + -0xa;
  iStack46 += -0x2;
  iStack44 += -0x2;
  FrameRect16(hbrush_var38,&rect_var_32,(HDC16)&DAT_1050_1050);
  DeleteObject16(hbrush_var38);
  hdc = hdc16_var36;
  handle = CreatePen16(0x8080,0x2,0x0);
  obj_handle_var3 = SelectObject16(handle,hdc);
  draw_line_1040_c302(struct_param_1,hdc16_var36);
  draw_op_1040_c38e(struct_param_1);
  obj_handle_var3 = SelectObject16(obj_handle_var3,hdc16_var36);
  DeleteObject16(obj_handle_var3);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar3->hwnd_0x4);
  return;
}



// WARNING: Unable to use type for symbol uVar4
pub fn draw_line_1040_c302(param_1: *mut astruct_772,HDC16 param_2)

{
  let mut uVar3: u32;
  let mut uVar5: u16;
  astruct_794 *iVar7;
  astruct_793 *iVar6;
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
    iVar7 = (astruct_794 *)uVar1;
    iVar7 = (astruct_794 *)&iVar7->field_0x1e;
    uVar7 = ((uVar1 & 0xffff0000) >> 0x10);
    MoveTo16(iVar7->field32_0x20 + iVar7->field34_0x24,
             iVar7->field33_0x22 / 0x2 + (uVar1 & 0xffff0000 | ZEXT24(iVar7)),param_2);
    uVar3 = (uVar5 + iVar1 * 0x4 + -0x4);
    iVar6 = (astruct_793 *)uVar3;
    iVar6 = (astruct_793 *)&iVar6->field_0x1e;
    uVar3 &= 0xffff0000;
    uVar8 = (uVar3 >> 0x10);
    LineTo16(iVar6->field32_0x20,iVar6->field33_0x22 / 0x2 + (uVar3 | ZEXT24(iVar6)),param_2);
  }
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar5
// WARNING: Unable to use type for symbol uVar4
// WARNING: Unable to use type for symbol uVar6
// WARNING: Unable to use type for symbol uVar7
pub fn draw_op_1040_c38e(param_1: *mut astruct_772)

{
  let mut iVar1: i16;
  let mut uVar8: u32;
  let mut iVar5: i16;
  let mut iVar11: i16;
  let mut y1: i16;
  let mut iVar12: i16;
  let mut in_DX: u16;
  astruct_772 *iVar10;
  let mut uVar10: u16;
  let mut uVar9: u16;
  let mut uVar11: u16;
  let mut unaff_SS: u16;
  let mut DVar10: u32;
  let mut DVar9: u32;
  HDC16 in_stack_00000008;
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
  iVar10 = (astruct_772 *)param_1;
  uVar2 = iVar10->field5_0x6;
  iVar1 = (uVar2 + 0x18);
  if ((iVar1 != 0) && (uVar4 = iVar10->field5_0x6, (uVar4 + 0x16) != 0)) {
    iVar5 = iVar1;
    pass1_1010_2ee2(iVar10->field5_0x6);
    for (iStack26 = 0; iStack26 < iVar1; iStack26 += 1) {
      uVar3 = (iStack26 * 0x4 + iVar5);
      iVar11 = uVar3;
      iVar11 = iVar11 + 0x1e;
      x1 = (uVar3 & 0xffff0000 | iVar11);
      uVar9 = ((uVar3 & 0xffff0000) >> 0x10);
      y1 = (iVar11 + 0x24) / 0x2 + (iVar11 + 0x20);
      MoveTo16(y1,*x1,in_stack_00000008);
      LineTo16(y1,*x1 + -0xf,in_stack_00000008);
      DVar10 = GetCurrentPosition16(in_stack_00000008);
      y5 = (DVar10 >> 0x10);
      x3 = DVar10;
      if (iStack26 == 0) {
        x2 = x3;
        y4 = y5;
      }
    }
    uVar6 = iVar10->field5_0x6;
    if ((uVar6 + 0x24) != 0) {
      y4 += -0xd;
    }
    uVar7 = iVar10->field5_0x6;
    if ((uVar7 + 0x26) != 0) {
      y5 += 0xd;
    }
    uVar8 = iVar10->field5_0x6;
    uVar5 = iVar10->field5_0x6;
    uVar1 = (uVar8 + (uVar5 + 0x16) * 0x4 + 0x26);
    iVar12 = uVar1;
    iVar12 = iVar12 + 0x1e;
    uVar11 = ((uVar1 & 0xffff0000) >> 0x10);
    MoveTo16((iVar12 + 0x24) / 0x2 + (iVar12 + 0x20),
             (iVar12 + 0x22) + (uVar1 & 0xffff0000 | iVar12),in_stack_00000008);
    LineTo16((iVar12 + 0x24) / 0x2 + (iVar12 + 0x20),x2,in_stack_00000008);
    DVar9 = GetCurrentPosition16(in_stack_00000008);
    DVar9 = (DVar9 >> 0x10);
    if (DVar9 < y4) {
      y4 = DVar9;
    }
    if (y5 < DVar9) {
      y5 = DVar9;
    }
    MoveTo16(y4,x2,in_stack_00000008);
    LineTo16(y5,x3,in_stack_00000008);
  }
  return;
}



pub fn pass1_1040_c518(mut param_1: u32,param_2: u8) -> u32

{
  pass1_1040_bf92(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1040_c54a(param_1: *mut astruct_65,mut param_2: u16 ,u32 *param_3,mut param_4: u16 ,mut param_5: u32)

{
  code **ppcVar1;
  let mut iVar3: i16;
  astruct_65 *iVar2;
  astruct_65 *uVar4;
  let mut puVar4: *mut u32;
  let mut uVar5: u16;
  let mut uVar6: u32;

  iVar3 = (param_3 + 0x12) + 0xc8;
  uVar6 = 0;
  uVar5 = 0;
  ppcVar1 = (code **)(*param_3 + 0x14);
  puVar4 = param_3;
  (**ppcVar1)();
  mixed_struct_op_1040_8fb8
            (param_5,param_1,0x0,CONCAT22(param_5,iVar3),puVar4,(puVar4 >> 0x10),uVar5,
             uVar6,(uVar6 >> 0x10),param_4);
  uVar4 = (astruct_65 *)(param_1 >> 0x10);
  iVar2 = (astruct_65 *)param_1;
  &iVar2[0x1].field13_0x1c = param_3;
  iVar2[0x1].field15_0x20 = 0;
  iVar2[0x1].field16_0x22 = param_2;
  param_1->field0_0x0 = 0xc9f2;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  pass1_1040_c630((astruct_65 *)(param_1 & 0xffff | ZEXT24(uVar4) << 0x10),param_5);
  return;
}
pub fn pass1_1040_c5ac(StructD *param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  StructD *iVar4;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  param_1->address_offset_field_0x0 = 0xc9f2;
  iVar4->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + -0x1;
  if (PTR_LOOP_1050_5f02 == NULL) {
    puVar1 = iVar4->field5_0x8;
    uVar2 = iVar4->field6_0xa;
    if ((uVar2 | puVar1) != 0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    puVar1 = iVar4->field7_0xc;
    uVar2 = iVar4->field8_0xe;
    if ((uVar2 | puVar1) != 0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
  }
  mix_win_ui_op_1040_911e(param_1);
  return;
}



u16 pass1_1040_c60e(param_1: *mut astruct_65)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0x42) != 0) {
    uVar1 = (param_1 + 0x42);
    return (uVar1 + 0x12);
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_c630(param_1: *mut astruct_65,mut param_2: u32)

{
  let mut uVar1: u16;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  astruct_65 *iVar4;
  let mut uVar5: u16;
  let mut unaff_CS: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (astruct_65 *)param_1;
  uVar3 = &iVar4[0x1].field13_0x1c;
  if ((uVar3 + 0x12) != 0x71) {
    iVar4[0x1].field8_0x10 = 0x5;
    (iVar4 + 1)->field0_0x0 = 0x5;
    iVar4[0x1].field1_0x2 = 0x5;
    uVar1 = iVar4[0x1].field8_0x10;
    iVar4[0x1].field5_0xa = uVar1;
    iVar4[0x1].field4_0x8 = uVar1;
    if (PTR_LOOP_1050_5f02 == NULL) {
      uVar4 = FUN_1010_830a(uVar1,param_2,unaff_CS,_u16_1050_14cc,0xff);
      _PTR_LOOP_1050_5f04 = CONCAT22(param_2,uVar4);
      unaff_CS = 0x1010;
      uVar4 = FUN_1010_830a(uVar4,param_2,0x1010,_u16_1050_14cc,0x100);
      _PTR_LOOP_1050_5f08 = CONCAT22(param_2,uVar4);
    }
    PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + 1;
    &iVar4->field4_0x8 = _PTR_LOOP_1050_5f04;
    &iVar4->field6_0xc = _PTR_LOOP_1050_5f08;
    pass1_1040_9618((astruct_65 *)(param_1 & 0xffff | uVar5 << 0x10));
    iVar4->field15_0x20 = 0;
    iVar4->field14_0x1e = 0xc8;
    iVar4->field16_0x22 = 0xa0;
    iVar4->field17_0x24 = iVar4[0x1].field3_0x6 + iVar4[0x1].field8_0x10;
    iVar4[0x1].field4_0x8 = iVar4[0x1].field8_0x10 * 0x3 + iVar4[0x1].field2_0x4;
    iVar4[0x1].field5_0xa = iVar4[0x1].field8_0x10;
    iVar4[0x1].field6_0xc = iVar4->field16_0x22 - iVar4[0x1].field8_0x10;
    (&iVar4[0x1].field10_0x14 + 0x2) = 0x25;
    uVar3 = param_1;
    ppcVar2 = (code **)(uVar3 + 0x4);
    (**ppcVar2)(unaff_CS,param_1);
    ppcVar2 = (code **)(uVar3 + 0x8);
    (**ppcVar2)(unaff_CS,param_1,uVar5);
  }
  return;
}
pub fn pass1_1040_c71e(param_1: *mut astruct_65)

{
  astruct_65 *iVar1;
  astruct_65 *uVar1;

  pass1_1040_9252(param_1);
  uVar1 = (astruct_65 *)(param_1 >> 0x10);
  iVar1 = (astruct_65 *)param_1;
  iVar1[0x1].field1_0x2 = iVar1->field17_0x24 / 0x2 - iVar1[0x1].field3_0x6 / 0x2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn draw_op_1040_c74c(param_1: *mut astruct_738,mut param_2: u16 ,HDC16 hdc16_param_3,mut param_4: u16 )

{
  let mut uVar2: u16;
  HGDIOBJ16 hdc_black_brush_1;
  HPEN16 pen_handle_1;
  HGDIOBJ16 handle;
  HPALETTE16 hpalette_1;
  astruct_738 *struct_1;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar3: u32;
  let mut uVar1: u16;
  code **func_ptr_1;

  uVar4 = (_PTR_LOOP_1050_4230 >> 0x10);
  uVar2 = (_PTR_LOOP_1050_4230 + 0x10);
  hpalette_1 = palette_op_1008_4e08
                         ((HPALETTE16)&hdc16_param_3,uVar2,
                          (astruct_13 *)CONCAT22(uVar2,(_PTR_LOOP_1050_4230 + 0xe)),
                          (HDC16 *)CONCAT13(0x10,CONCAT12(0x50,&hdc16_param_3)));
  uVar5 = (param_1 >> 0x10);
  struct_1 = (astruct_738 *)param_1;
  struct_1->field66_0x46 = 0x1;
  hdc_black_brush_1 = GetStockObject16(BLACK_BRUSH);
  pen_handle_1 = CreatePen16(0x1000002,0x1,0x0);
  hdc_black_brush_1 = SelectObject16(hdc_black_brush_1,hdc16_param_3);
  handle = SelectObject16(pen_handle_1,hdc16_param_3);
  Rectangle16(struct_1->field35_0x24,struct_1->field34_0x22,0x0,0x0,hdc16_param_3);
  MoveTo16(0x0,struct_1->field51_0x36 * 0x2 + struct_1->field40_0x2a,hdc16_param_3);
  LineTo16(struct_1->field35_0x24,struct_1->field51_0x36 * 0x2 + struct_1->field40_0x2a,hdc16_param_3);
  SelectObject16(hdc_black_brush_1,hdc16_param_3);
  hdc_black_brush_1 = SelectObject16(handle,hdc16_param_3);
  DeleteObject16(hdc_black_brush_1);
  uVar3 = param_1;
  func_ptr_1 = (code **)(uVar3 + 0x10);
  (**func_ptr_1)(s_tile2_bmp_1050_1538,param_1,_param_2);
  func_ptr_1 = (code **)(uVar3 + 0x14);
  (**func_ptr_1)(s_tile2_bmp_1050_1538,struct_1,(param_1 >> 0x10),hdc16_param_3);
  struct_1->field66_0x46 = 0;
  hpalette_1 = SelectPalette16(0x0,hpalette_1,hdc16_param_3);
  DeleteObject16(hpalette_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn send_msg_1040_c85a(mut param_1: u32)

{
  _PTR_LOOP_1050_5efe = param_1;
  SendMessage16(0x0,0xfa,0x111,*(HWND16 *)(param_1 + 0x1a));
  return;
}
pub fn FUN_1040_c882()

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn palette_op_1040_c886(astruct_769 *struct_param_1,param_2: u8,HDC16 hdc_param_3)

{
  let mut uVar1: u16;
  HPALETTE16 palette_2;
  let mut uVar4: u16;
  astruct_769 *struct_2;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut unaff_CS: u16;
  let mut puStack8: *mut u32;
  HPALETTE16 palette;
  code **fn_ptr_1;

  uVar2 = (struct_param_1 >> 0x10);
  struct_2 = (astruct_769 *)struct_param_1;
  if (((&struct_2->field8_0x8 + 0x2) | &struct_2->field8_0x8) != 0) {
    palette = 0;
    if (struct_2->field59_0x46 == 0) {
      uVar3 = (_PTR_LOOP_1050_4230 >> 0x10);
      uVar1 = (_PTR_LOOP_1050_4230 + 0x10);
      unaff_CS = 0x1008;
      palette = palette_op_1008_4e08
                          ((HPALETTE16)&hdc_param_3,uVar1,
                           (astruct_13 *)CONCAT22(uVar1,(_PTR_LOOP_1050_4230 + 0xe)),
                           (HDC16 *)CONCAT22(0x1050,&hdc_param_3));
    }
    puStack8 = struct_2->field8_0x8;
    uVar4 = (&struct_2->field8_0x8 + 2);
    if ((((&struct_2->field9_0xc + 0x2) | &struct_2->field9_0xc) != 0) &&
       ((param_2 & 1) != 0)) {
      puStack8 = struct_2->field9_0xc;
      uVar4 = (&struct_2->field9_0xc + 2);
    }
    if ((struct_2->field10_0x10 != NULL) && ((param_2 & 0x4) != 0)) {
      puStack8 = struct_2->field10_0x10;
      uVar4 = (&struct_2->field10_0x10 + 2);
    }
    fn_ptr_1 = (code **)(*puStack8 + 0x4);
    (**fn_ptr_1)(unaff_CS,puStack8,uVar4,struct_2->field30_0x28,struct_2->field29_0x26,&hdc_param_3,
                 &DAT_1050_1050);
    if (struct_2->field59_0x46 == 0) {
      palette_2 = SelectPalette16(0x0,palette,hdc_param_3);
      DeleteObject16(palette_2);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_c94a(param_1: *mut u8,param_2: *mut astruct_37,HDC16 param_3,mut param_4: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;

  if ((param_2 + 0x48) != 0) {
    puVar5 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(param_4,0x3),in_stack_0000fe98,in_stack_0000ffbc,in_stack_0000ffc2,
                             in_stack_0000ffc6);
    uVar3 = (puVar5 >> 0x10);
    uVar2 = (param_2 + 0x42);
    uVar1 = (uVar2 + 0x12);
    uVar4 = uVar1;
    pass1_1010_a5ca(uVar1,uVar3,puVar5,uVar3,uVar1);
    if (uVar4 == 0xffff) {
      (param_2 + 0x3c) = 0xf9;
    }
    else if (uVar4 == 0) {
      (param_2 + 0x3c) = 0x25;
      if ((uVar1 == 0x5b) || (uVar1 == 0x9)) {
        (param_2 + 0x3c) = 0xfe;
      }
    }
    else {
      (param_2 + 0x3c) = 0xfb;
    }
  }
  draw_text_1040_94fc(param_2,param_3);
  return;
}



StructD * pass1_1040_c9cc(StructD *param_1,param_2: u8)

{
  pass1_1040_c5ac(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_ca16(param_1: *mut u8,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar1: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082(param_2,CONCAT22(param_3,0x1840));
  uVar2 = (param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  &iVar1[0x1].field3_0x6 = _PTR_LOOP_1050_5f0c;
  &iVar1[0x1].field5_0xa = 0;
  iVar1[0x1].field7_0xe = 0;
  iVar1[0x1].field8_0x10 = 0;
  param_2->field0_0x0 = 0xd07c;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3e),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field5_0xa = puVar3;
  iVar1[0x1].field6_0xc = (puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_ca74(StructD *param_1)

{
  let mut uVar1: u16;
  let mut in_stack_0000ffde: u16;

  uVar1 = (param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xd07c;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  PTR_LOOP_1050_5f10 = NULL;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_caa6(param_1: *mut u8,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_register_0000000a: u16;
  astruct_27 *paVar1;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  iVar2 = 0;
  paVar1 = (astruct_27 *)
           mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,(u8 **)0x2b,
                           in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
  pass1_1010_038e(paVar1,iVar2);
  destroy_window_1040_b726(CONCAT22(param_3,param_2),(param_3 >> 0x10));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_cace(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut bVar2: bool;
  let mut iVar3: i16;
  let mut IVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut bVar7: bool;
  let mut uVar8: u16;
  char local_208 [0x100];
  char local_108 [0x100];
  let mut UStack8: u16;
  let mut uStack6: u16;
  let mut local_4: bool;

  uVar6 = (param_2 >> 0x10);
  uVar5 = param_2;
  uStack6 = GetDlgItemInt16(0x0,&local_4,(INT16)&DAT_1050_1050,0x18e);
  UStack8 = GetDlgItemInt16(0x0,&local_4,(INT16)&DAT_1050_1050,0x191);
  if (uStack6 == 0) {
    return;
  }
  pass1_1018_50ea((uVar5 + 0x98),uStack6,(uVar5 + 0x94));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_208,(short)&DAT_1050_1050);
  uVar1 = (uVar5 + 0x94);
  uVar8 = (_u16_1050_14cc >> 0x10);
  if (*(i32 *)(uVar1 + 0x36) == 0) {
    load_string_1010_84e0(_u16_1050_14cc,uVar8,0x3ff,local_108,(short)&DAT_1050_1050);
    iVar3 = MessageBox16(0x34,CONCAT22(0x1050,local_208),CONCAT22(0x1050,local_108),
                         *(HWND16 *)(uVar5 + 0x8));
  }
  else {
    load_string_1010_84e0(_u16_1050_14cc,uVar8,0x3ff,local_108,(short)&DAT_1050_1050);
    iVar3 = MessageBox16(0x34,CONCAT22(0x1050,local_208),CONCAT22(0x1050,local_108),
                         *(HWND16 *)(uVar5 + 0x8));
  }
  bVar2 = iVar3 == 0x6;
  bVar7 = false;
  if ((!bVar2) && (uVar1 = (uVar5 + 0x94), (uVar1 + 0x34) < 1)) {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_108,(short)&DAT_1050_1050);
    IVar4 = MessageBox16(0x34,CONCAT22(0x1050,local_208),CONCAT22(0x1050,local_108),
                         *(HWND16 *)(uVar5 + 0x8));
    bVar7 = IVar4 == 0x6;
    bVar2 = false;
  }
  if (bVar2) {
    _PTR_LOOP_1050_5f16 = (uVar5 + 0x94);
    iVar3 = 0x26;
  }
  else {
    if (!bVar7) {
      return;
    }
    _u16_1050_5a68 = (uVar5 + 0x94);
    iVar3 = 0x27;
  }
  pass1_1038_af40(uVar5,param_1,_PTR_LOOP_1050_5b7c,(uVar5 + 0x8),iVar3);
  return;
}



LRESULT pass1_1040_cc66(mut param_1: u32)

{
  code **ppcVar1;
  let mut extraout_DX: u16;
  let mut LVar2: LRESULT;

  ppcVar1 = (code **)((param_1 + 0x98) + 0x10);
  (**ppcVar1)();
  LVar2 = send_dlg_msg_1040_cf1c(extraout_DX,(astruct_903 *)param_1);
  return LVar2;
}
pub fn pass1_1040_cc8c(param_1: *mut u8,param_2: *mut astruct_903,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  if (param_5 == 0xeb) {
    send_dlg_msg_1040_cf1c(param_1,param_2);
  }
  else {
    // just 0x1756
    if (param_5 == s_vrpal_bmp_1050_183a + 0x7U) {
      msg_box_op_1040_cce4(0x0,param_1,param_2);
    }
    else {
      if (param_5 != s_vrpal_bmp_1050_183a + 0x8U) {
        pass1_1040_b54a(param_1,param_2,param_3,_param_4);
        return;
      }
      if (param_4 == 1) {
        send_dlg_item_1040_ce76(param_2);
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_op_1040_cce4(param_1: *mut c_char,mut param_2: u16 ,param_3: *mut astruct_903)

{
  short in_buf_len_5;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uStack522: u32;
  char local_206 [0x102];
  char local_104 [0x102];
  let mut uVar2: u16;
  let mut uVar3: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,(short)&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)(param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}



u16 pass1_1040_cdac(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut bVar3: bool;
  let mut iVar4: i16;
  let mut uVar5: u16;

  bVar3 = false;
  iVar4 = param_1;
  uVar5 = (param_1 >> 0x10);
  if (param_4 == 0) {
    iVar2 = (iVar4 + 0x9e);
    piVar1 = (iVar4 + 0x9c);
    if (*piVar1 == iVar2 || *piVar1 < iVar2) goto LAB_1040_cdef;
    piVar1 = (iVar4 + 0x9e);
    *piVar1 = *piVar1 + 1;
  }
  else {
    if (param_4 != 1) goto LAB_1040_cdef;
    if ((iVar4 + 0x9e) < 1) goto LAB_1040_cdef;
    piVar1 = (iVar4 + 0x9e);
    *piVar1 = *piVar1 + -0x1;
  }
  bVar3 = true;//
LAB_1040_cdef:
  if (bVar3) {
    SetDlgItemInt16(0x0,(iVar4 + 0x9e),0x18e,*(HWND16 *)(iVar4 + 0x6));
  }
  return 0x0;
}
pub fn send_dlg_item_msg_1040_ce12(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  let mut uVar1: u32;
  let mut lVar2: i32;
  WORD local_10a [0x80];
  u8 local_a [0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),param_3);
  while( true ) {
    lVar2 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (lVar2 == 0) break;
    uVar1 = (lVar2 + 0x4);
    wsprintf16(local_10a,0x5f121050,CONCAT22(uVar1,0x1050),(uVar1 >> 0x10));
    SendDlgItemMessage16(CONCAT22(0x1050,local_10a),0x0,0x401,param_4,*(HWND16 *)(param_1 + 0x6));
  }
  return;
}
pub fn send_dlg_item_1040_ce76(param_1: *mut astruct_903)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut LVar3: LRESULT;
  let mut uVar4: u32;
  u8 local_106 [0x100];
  let mut WStack6: WPARAM16;
  let mut iStack4: i16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  LVar3 = SendDlgItemMessage16(0x0,0x0,0x409,s_vrpal_bmp_1050_183a + 0x8,*(HWND16 *)(iVar1 + 0x6));
  WStack6 = LVar3;
  iStack4 = WStack6 >> 0xf;
  if (WStack6 != 0xffff) {
    SendDlgItemMessage16
              (CONCAT22(0x1050,local_106),WStack6,0x40a,s_vrpal_bmp_1050_183a + 0x8,*(HWND16 *)(iVar1 + 0x6));
    uVar4 = pass1_1018_5206((iVar1 + 0x98),CONCAT22(0x1050,local_106));
    if (uVar4 != 0) {
      (iVar1 + 0x9c) = (uVar4 + 0x8);
      (iVar1 + 0x9e) = 0;
      SetDlgItemInt16(0x0,0x0,0x18e,*(HWND16 *)(iVar1 + 0x6));
      SetDlgItemInt16(0x0,(iVar1 + 0x9c),0x191,*(HWND16 *)(iVar1 + 0x6));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT send_dlg_msg_1040_cf1c(mut param_1: u16 ,param_2: *mut astruct_903)

{
  let mut puVar1: *mut u8;
  let mut hwnd: HWND16;
  let mut in_register_0000000a: u16;
  astruct_928 *uVar1;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut uVar4: u32;
  let mut LVar5: LRESULT;
  let mut in_stack_0000f99c: u16;
  let mut in_stack_0000fac0: u16;
  let mut in_stack_0000fac6: u16;
  let mut in_stack_0000faca: u16;
  let mut enable: bool;
  let mut uVar6: u16;
  let mut in_stack_0000faf4: u16;
  char local_106 [0x100];

  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000faf4,0x3),in_stack_0000f99c,in_stack_0000fac0,
                           in_stack_0000fac6,in_stack_0000faca);
  puVar1 = (puVar3 >> 0x10);
  uVar2 = (param_2 >> 0x10);
  uVar1 = (astruct_928 *)param_2;
  pass1_1010_c3c2(puVar1,puVar3,puVar1,CONCAT22(0x1050,local_106),uVar1->field147_0x94);
  SendDlgItemMessage16(CONCAT22(0x1050,local_106),0x0,0xc,s_dibtext_bmp_1050_1844 + 0x2,uVar1->field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0xb,s_vrpal_bmp_1050_183a + 0x8,uVar1->field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,s_vrpal_bmp_1050_183a + 0x8,uVar1->field6_0x6);
  uVar6 = s_vrpal_bmp_1050_183a + 0x8;
  uVar4 = pass1_1018_526a(uVar1->field148_0x98,uVar1->field147_0x94);
  send_dlg_item_msg_1040_ce12(uVar1,uVar2,uVar4,uVar6);
  LVar5 = SendDlgItemMessage16(0x0,0x0,0x40c,s_vrpal_bmp_1050_183a + 0x8,uVar1->field6_0x6);
  if (((LVar5 >> 0x10) < 1) && ((LVar5 < 0x0 || (LVar5 == 0)))) {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,&stack0xfaf4,(short)&DAT_1050_1050);
    SendDlgItemMessage16(CONCAT22(0x1050,&stack0xfaf4),0x0,0x401,s_vrpal_bmp_1050_183a + 0x8,uVar1->field6_0x6);
    hwnd = GetDlgItem16(0x1,uVar1->field6_0x6);
    enable = 0;
  }
  else {
    hwnd = GetDlgItem16(0x1,uVar1->field6_0x6);
    enable = 0x1;
  }
  EnableWindow16(enable,hwnd);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0xb,s_vrpal_bmp_1050_183a + 0x8,uVar1->field6_0x6);
  return LVar5;
}
