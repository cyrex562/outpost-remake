pub fn unk_draw_op_1020_0000(param_1: *mut astruct_840)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  astruct_840 *iVar4;
  let mut uVar5: u16;
  let mut uVar4: u16;
  u8 local_c4 [0x6];
  let mut local_be: *mut i16;
  let mut piStack184: *mut i16;
  let mut uStack182: u16;
  let mut local_b4: i16;
  let mut iStack178: i16;
  i16 aiStack176 [0x3c];
  astruct_841 *iStack56;
  let mut iStack48: i16;
  astruct_76 *paStack46;
  let mut local_2a: i16;
  let mut local_28: i16;
  let mut puStack38: *mut u32;
  u8 local_22 [0x20];
  let mut uVar3: u32;
  let mut uVar2: u32;
  astruct_841 *iVar3;

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
  iVar4 = (astruct_840 *)param_1;
  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),iVar4.field4_0x4);
  uVar3 = iVar4.field19_0x14;
  puStack38 = (uVar3 + 0xa);
  pass1_1008_3e94((param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x18)),CONCAT22(0x1050,&local_2a),
                  CONCAT22(0x1050,&local_28));
  uVar4 = 0x1008;
  pass1_1008_4480((astruct_76 *)puStack38,(param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x18)),
                  iVar4.field32_0x24);
  paStack46 = NULL;
  for (iStack48 = 0x0; iStack48 < 0x6; iStack48 += 0x1) {
    uVar2 = iVar4.field19_0x14;
    uVar4 = 0x1010;
    pass1_1010_2b78(uVar2,(uVar2 >> 0x10),iStack48,CONCAT22(0x1050,&local_b4));
    if (local_b4 == 0x0) {
      for (iStack56 = NULL; iVar3 = iStack56, iStack56 <= iStack178; iStack56 = iStack56 + 0x1) {
        piVar1 = aiStack176 + iStack56 * 0x3;
        uStack182 = &DAT_1050_1050;
        piStack184 = piVar1;
        if (aiStack176[iStack56 * 0x3 + 0x2] != 0x0) {
          paStack46 = pass1_1010_2b98(iVar4.field19_0x14,aiStack176[iStack56 * 0x3 + 0x2]);
          pass1_1008_3e54(CONCAT22(0x1050,&local_be),0x0,aiStack176[iVar3 * 0x3 + 0x1] + local_2a,
                          *piVar1 + local_28);
          uVar4 = 0x1008;
          pass1_1008_4480((astruct_76 *)puStack38,CONCAT22(0x1050,&local_be),paStack46);
        }
      }
    }
    else {
      local_be = CONCAT22(0x1050,aiStack176 + iStack178 * 0x3);
      if (aiStack176[iStack178 * 0x3 + 0x2] != 0x0) {
        paStack46 = pass1_1010_2b98(iVar4.field19_0x14,aiStack176[iStack178 * 0x3 + 0x2]);
        pass1_1008_3e54(CONCAT22(0x1050,local_c4),0x0,(local_be + 0x2) + local_2a,
                        *local_be + local_28);
        uVar4 = 0x1008;
        pass1_1008_4480((astruct_76 *)puStack38,CONCAT22(0x1050,local_c4),paStack46);
      }
    }
  }
  ppcVar2 = (code **)(*puStack38 + 0x4);
  (**ppcVar2)(uVar4,puStack38,(puStack38 >> 0x10),0x0,0x0,&iVar4.field_0xa,uVar5);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),iVar4.field4_0x4);
  return;
}



StructD * pass1_1020_01a6(StructD *param_1,param_2: u8)

{
  pass1_1018_ed98(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1020_01d8(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 ,
                    mut param_7: u32)

{
  let mut in_stack_0000ffd6: u16;

  unk_draw_op_1008_61b2(in_stack_0000ffd6,(StructA *)param_1,param_2,param_6,param_7);
  (param_1 + 0xe2) = 0x0;
  (param_1 + 0xe6) = 0x0;
  (param_1 + 0xea) = param_5;
  (param_1 + 0xec) = param_4;
  (param_1 + 0xee) = param_3;
  param_1.offset_0x0 = 0x45a;
  (param_1 + 0x2) = 0x1020;
  return;
}
pub fn pass1_1020_022c(astruct_29 *struct_param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_29 *iVar4;
  let mut uVar4: u16;

  uVar4 = (struct_param_1 >> 0x10);
  iVar4 = (astruct_29 *)struct_param_1;
  struct_param_1.field0_0x0 = 0x45a;
  iVar4.field1_0x2 = 0x1020;
  puVar1 = &iVar4.field228_0xe6;
  uVar2 = (&iVar4.field228_0xe6 + 0x2);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1008_57c4((struct_param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0xd2)));
  struct_param_1.field0_0x0 = 0x380a;
  iVar4.field1_0x2 = 0x1008;
  struct_param_1.field0_0x0 = 0x389a;
  iVar4.field1_0x2 = 0x1008;
  return;
}
pub fn pass1_1020_028c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16)

{
  let mut in_DX: u16;

  pass1_1010_3c9e(*(i32 *)(param_1 + 0xe2));
  pass1_1008_68c6(in_DX,param_1,param_2,param_3);
  return;
}
pub fn pass1_1020_02ae(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  pass1_1010_3cd0(*(i32 *)(iVar4 + 0xe2));
  destroy_win_1008_628e(param_1);
  puVar1 = (iVar4 + 0xe6);
  uVar2 = (iVar4 + 0xe8);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  (iVar4 + 0xe6) = 0x0;
  pass1_1010_1dda((iVar4 + 0xe2));
  (iVar4 + 0xe2) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_1020_0316(mut param_1: u16 ,param_2: *mut StructA)

{
  let mut uVar1: u32;
  astruct_666 *paVar2;
  let mut puVar3: *mut u8;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let iVar1: *mut StructA;
  let mut uVar5: u16;
  let mut puVar6: *mut u32;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000fff2: u16;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  create_window_ex_1008_9760(param_2);
  puVar6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0x1),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  paVar4 = (astruct_57 *)(paVar4 & 0xffff0000 | puVar6 >> 0x10);
  uVar5 = (param_2 >> 0x10);
  iVar1 = (StructA *)param_2;
  iVar1[0x1].field11_0x16 = (i16)puVar6;
  iVar1[0x1].field12_0x18 = (i16)(puVar6 >> 0x10);
  uVar1 = &iVar1[0x1].field11_0x16;
  (uVar1 + 0x16) = iVar1[0x1].field15_0x1e;
  paVar2 = iVar1[0x1].field18_0x22;
  uVar1 = &iVar1[0x1].field11_0x16;
  *(astruct_666 **)(uVar1 + 0x12) = paVar2;
  struct_1010_3c52(paVar4,&iVar1[0x1].field11_0x16,&iVar1[0x1].field_0x20);
  mem_op_1000_179c(0x12,paVar4);
  puVar3 = (paVar4 | paVar2);
  if (puVar3 != NULL) {
    pass1_1020_04f6(puVar3,(astruct_662 *)CONCAT22(paVar4,paVar2),iVar1.field4_0x8);
    iVar1[0x1].field13_0x1a = paVar2;
    iVar1[0x1].field14_0x1c = puVar3;
    return;
  }
  &iVar1[0x1].field13_0x1a = 0x0;
  return;
}
pub fn post_msg_1020_03b2(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0xe2);
  PostMessage16(0x0,*(WPARAM16 *)(uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}
pub fn post_msg_1020_03d6(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0xe2);
  PostMessage16(0x0,*(WPARAM16 *)(uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}
pub fn post_msg_1020_03fa(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0xe2);
  PostMessage16(0x0,*(WPARAM16 *)(uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}
pub fn draw_op_1020_041e(mut param_1: u32)

{
  fill_rect_1020_065e(*(astruct_754 **)(param_1 + 0xe6));
  return;
}



astruct_29 * pass1_1020_0434(param_1: *mut astruct_29,param_2: u8)

{
  pass1_1020_022c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_04f6(u8 *param_1,param_2: *mut astruct_662,mut param_3: u16 )

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  astruct_662 *iVar4;
  let mut uVar5: u16;
  let mut puVar6: *mut u32;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000fe8a: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb8: u16;
  let mut uVar7: u16;
  let mut in_stack_0000ffe0: u32;
  u8 **ppuVar8;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar5 = (param_2 >> 0x10);
  iVar4 = (astruct_662 *)param_2;
  param_2 = 0x389a;
  iVar4.field2_0x2 = 0x1008;
  param_2 = 0x3aa8;
  iVar4.field2_0x2 = 0x1008;
  iVar4.field3_0x4 = param_3;
  param_2 = 0x3ab0;
  iVar4.field2_0x2 = 0x1008;
  iVar4.field4_0x6 = NULL;
  iVar4.field5_0xa = 0x0;
  iVar4.field6_0xc = 0x0;
  iVar4.field7_0xe = 0x0;
  iVar4.field8_0x10 = 0x0;
  param_2 = 0x75a;
  iVar4.field2_0x2 = 0x1020;
  ppuVar8 = (u8 **)CONCAT22((in_stack_0000ffe0 >> 0x10),0x1);
  puVar6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,ppuVar8,in_stack_0000fe8a,in_stack_0000ffae,in_stack_0000ffb4,
                           in_stack_0000ffb8);
  paVar4 = (astruct_57 *)(paVar4 & 0xffff0000 | puVar6 >> 0x10);
  &iVar4.field4_0x6 = puVar6;
  uVar3 = (puVar6 >> 0x10);
  (&iVar4.field4_0x6 + 0x2) = uVar3;
  uVar7 = &iVar4.field4_0x6;
  ppcVar1 = (code **)(*iVar4.field4_0x6 + 0x4);
  (**ppcVar1)(0x1010,uVar7,uVar3,0x0,param_2,(ppuVar8 >> 0x10));
  puVar6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(uVar7,0x48),in_stack_0000fe80,in_stack_0000ffa4,
                           in_stack_0000ffaa,in_stack_0000ffae);
  iVar2 = puVar6;
  iVar4.field5_0xa = (iVar2 + 0xa);
  iVar4.field6_0xc = (iVar2 + 0xc);
  pass1_1008_3e94((puVar6 & 0xffff0000 | (iVar2 + 0xe)),
                  (param_2 & 0xffff0000 | ZEXT24(&iVar4.field8_0x10)),
                  (param_2 & 0xffff0000 | ZEXT24(&iVar4.field7_0xe)));
  return;
}
pub fn pass1_1020_05d6(StructD *param_1)

{
  StructD *iVar1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x75a;
  iVar1.address_offset_field_0x2 = 0x1020;
  if (*(i32 *)&iVar1.field_0x6 != 0x0) {
    pass1_1010_1ea6(&iVar1.field_0x6,(param_1 & 0xffff | uVar1 << 0x10));
  }
  param_1.address_offset_field_0x0 = 0x3ab0;
  iVar1.address_offset_field_0x2 = 0x1008;
  param_1.address_offset_field_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}
pub fn post_win_msg_1020_061c(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (param_1 + 0x6) = 0x0;
    return;
  }
  if (param_2 != 0x2) {
    return;
  }
  uVar1 = (param_1 + 0x6);
  PostMessage16(0x0,*(WPARAM16 *)(uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}



// WARNING: Unable to use type for symbol uVar2
pub fn fill_rect_1020_065e(astruct_754 *astruct754_param_1)

{
  let mut uVar1: u32;
  astruct_754 *struct754_var1;
  let mut uVar4: u16;
  let mut rect_1: RECT16;
  let mut iStack48: i16;
  let mut iStack46: i16;
  HBRUSH16 brush_handle_1;
  HDC16 *palette_handle_var42;
  let mut puStack40: *mut u32;
  HDC16 hdc_var_24;
  u8 paintstruct_22 [0x20];
  let mut uVar3: u16;
  let mut uVar2: u32;
  code **fn_ptr_1;

  uVar4 = (astruct754_param_1 >> 0x10);
  struct754_var1 = (astruct_754 *)astruct754_param_1;
  hdc_var_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),*(HWND16 *)&struct754_var1.field_0x4);
  if (0x280 < struct754_var1.field7_0xa) {
    brush_handle_1 = CreateSolidBrush16(0x210070b);
    rect_1.x = 0x0;
    rect_1.y = 0x0;
    iStack48 = struct754_var1.field7_0xa + -0x1;
    iStack46 = struct754_var1.field8_0xc + -0x1;
    FillRect16(brush_handle_1,&rect_1,(HDC16)&DAT_1050_1050);
    DeleteObject16(brush_handle_1);
  }
  uVar2 = struct754_var1.field6_0x6;
  puStack40 = (uVar2 + 0xe);
  palette_handle_var42 = &hdc_var_24;
  uVar3 = puStack40;
  uVar1 = *puStack40;
  fn_ptr_1 = (code **)(uVar1 + 0x8);
  (**fn_ptr_1)(s_tile2_bmp_1050_1538,uVar3,(puStack40 >> 0x10),palette_handle_var42,&DAT_1050_1050
              );
  fn_ptr_1 = (code **)(uVar1 + 0x4);
  (**fn_ptr_1)(s_tile2_bmp_1050_1538,puStack40,(puStack40 >> 0x10),struct754_var1.field10_0x10,
               struct754_var1.field9_0xe,&hdc_var_24,&DAT_1050_1050,uVar3);
  palette_handle_var42 = (HDC16 *)SelectPalette16(0x0,(HPALETTE16)palette_handle_var42,hdc_var_24);
  DeleteObject16((HGDIOBJ16)palette_handle_var42);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),*(HWND16 *)&struct754_var1.field_0x4);
  return;
}



StructD * pass1_1020_0734(StructD *param_1,param_2: u8)

{
  pass1_1020_05d6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1020_0762(param_1: *mut astruct_20,mut param_2: u32,u32 *param_3,mut param_4: u16 ,mut param_5: u32,mut param_6: u32)

{
  astruct_20 *iVar1;
  astruct_20 *uVar1;

  pass1_1020_01d8(param_1,param_2,(param_2 >> 0x10),param_4,param_5,(param_5 >> 0x10),
                  param_6);
  uVar1 = (astruct_20 *)(param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  iVar1[0x1].field6_0xe = 0x0;
  iVar1[0x1].field7_0x10 = (astruct_20 *)*param_3;
  param_1.offset_0x0 = 0x81a;
  iVar1.base_0x2 = 0x1020;
  return;
}
pub fn pass1_1020_07aa(mut param_1: u16 ,param_2: *mut astruct_19)

{
  astruct_19 *iVar1;
  astruct_19 *uVar2;
  char local_16 [0x14];

  draw_op_1020_041e(param_2);
  uVar2 = (astruct_19 *)(param_2 >> 0x10);
  iVar1 = (astruct_19 *)param_2;
  if (iVar1[0x2].field24_0x30 == 0x0) {
    iVar1[0x2].field24_0x30 = 0x1;
    pass1_1008_5bdc(CONCAT22(0x1050,local_16));
    win_1008_5c9e(local_16,param_1,CONCAT22(0x1050,local_16),
                  (param_2 & 0xffff0000 | ZEXT24(&iVar1[0x2].field25_0x32)));
    pass1_1008_5c34(CONCAT22(0x1050,local_16));
  }
  return;
}



astruct_29 * pass1_1020_07f4(param_1: *mut astruct_29,param_2: u8)

{
  pass1_1020_022c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_08b6(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  astruct_20 *iVar1;
  let mut uVar1: u16;
  astruct_20 *paVar2;
  let mut in_stack_0000ffd6: u16;

  paVar2 = unk_draw_op_1008_61b2(in_stack_0000ffd6,(StructA *)param_1,0x1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  &iVar1[0x1].field2_0x4 = 0x0;
  (&iVar1[0x1].field2_0x4 + 0x2) = 0x0;
  param_1.offset_0x0 = 0xb0e;
  iVar1.base_0x2 = 0x1020;
  win_1008_5c5c(0x0,(paVar2 >> 0x10),_u16_1050_02a0,0x1d4);
  return;
}
pub fn send_win_msg_1020_08fe(param_1: *mut astruct_63)

{
  let mut hwnd: HWND16;
  let mut lVar1: i32;
  let mut BVar2: bool;
  astruct_63 *iVar2;
  astruct_63 *uVar2;

  uVar2 = (astruct_63 *)(param_1 >> 0x10);
  iVar2 = (astruct_63 *)param_1;
  param_1.field0_0x0 = 0xb0e;
  iVar2.field1_0x2 = 0x1020;
  if (iVar2.field229_0xe8 != 0x0) {
    lVar1 = iVar2.field229_0xe8;
    hwnd = *(HWND16 *)(lVar1 + 0x6);
    BVar2 = IsWindow16(hwnd);
    if (BVar2 != 0x0) {
      SendMessage16(0x0,0x1,0x111,hwnd);
    }
    iVar2.field229_0xe8 = 0x0;
  }
  pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(&iVar2.field208_0xd2)));
  param_1.field0_0x0 = 0x380a;
  iVar2.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x389a;
  iVar2.field1_0x2 = 0x1008;
  return;
}
pub fn send_msg_1020_097e(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (((iVar2 + 0xea) | (iVar2 + 0xe8)) != 0x0) {
    uVar1 = (iVar2 + 0xe8);
    SendMessage16(0x0,0x1,0x111,*(HWND16 *)(uVar1 + 0x6));
    (iVar2 + 0xe8) = 0x0;
  }
  return;
}
pub fn win_1020_09ba(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut StructA)

{
  let mut puVar1: *mut u8;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let iVar1: *mut StructA;
  let mut uVar3: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  create_window_ex_1008_9760(param_3);
  mem_op_1000_179c(0xe,paVar2);
  puVar1 = (paVar2 | param_1);
  iVar1 = (StructA *)param_3;
  uVar3 = (param_3 >> 0x10);
  if (puVar1 != NULL) {
    struct_1020_0baa(puVar1,CONCAT22(paVar2,param_1),iVar1.field4_0x8);
    iVar1[0x1].field11_0x16 = param_1;
    iVar1[0x1].field12_0x18 = (i16)puVar1;
    return;
  }
  &iVar1[0x1].field11_0x16 = 0x0;
  return;
}
pub fn pass1_1020_0a0c(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  destroy_win_1008_628e(param_1);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0xe2);
  uVar2 = (iVar4 + 0xe4);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  (iVar4 + 0xe2) = 0x0;
  (iVar4 + 0xe6) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_0a52(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  uVar2 = (param_2 >> 0x10);
  uVar1 = param_2;
  unk_draw_op_1020_0c3e(*(astruct_771 **)(uVar1 + 0xe2));
  if ((uVar1 + 0xe6) == 0x0) {
    (uVar1 + 0xe6) = 0x1;
    (_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
    uVar3 = pass1_1038_af40(uVar1,param_1,_PTR_LOOP_1050_5b7c,(uVar1 + 0x8),0x6);
    (uVar1 + 0xe8) = uVar3;
    (uVar1 + 0xea) = (uVar3 >> 0x10);
  }
  return;
}
pub fn pass1_1020_0aa6(mut param_1: u32)

{
  win_ui_palette_op_1020_0cd2(*(astruct_775 **)(param_1 + 0xe2));
  return;
}
pub fn pass1_1020_0abc(mut param_1: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0xe6) != 0x0) {
    ppcVar1 = (code **)((param_1 + 0xe8) + 0x10);
    (**ppcVar1)();
  }
  return;
}



astruct_63 * pass1_1020_0ae8(param_1: *mut astruct_63,param_2: u8)

{
  send_win_msg_1020_08fe(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1020_0baa(u8 *param_1,param_2: *mut u16,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  astruct_276 *iVar2;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe82: u16;
  let mut in_stack_0000fe8a: u16;
  let mut in_stack_0000ffa6: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb8: u16;
  let mut puVar4: *mut u16;
  let mut puVar5: *mut u16;
  let mut uVar6: u16;
  let mut in_stack_0000ffe2: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar2 = (param_2 >> 0x10);
  iVar2 = (astruct_276 *)param_2;
  *param_2 = 0x389a;
  iVar2.field2_0x2 = 0x1008;
  *param_2 = 0x3aa8;
  iVar2.field2_0x2 = 0x1008;
  iVar2.field3_0x4 = param_3;
  *param_2 = 0x3ab0;
  iVar2.field2_0x2 = 0x1008;
  &iVar2.field4_0x6 = 0x0;
  iVar2.field6_0xa = 0x0;
  iVar2.field7_0xc = 0x0;
  *param_2 = 0xdbc;
  iVar2.field2_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe2,0x7),in_stack_0000fe8a,
                           in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
  iVar2.field4_0x6 = puVar3;
  iVar2.field5_0x8 = (puVar3 >> 0x10);
  puVar5 = &iVar2.field6_0xa;
  puVar4 = &iVar2.field7_0xc;
  uVar6 = uVar2;
  puVar3 = mixed_1010_20ba((astruct_57 *)(paVar1 & 0xffff0000 | puVar3 >> 0x10),_u16_1050_0ed0,
                           (u8 **)CONCAT22(puVar4,0x48),in_stack_0000fe82,in_stack_0000ffa6,in_stack_0000ffac,
                           in_stack_0000ffb0);
  pass1_1008_3e94((puVar3 & 0xffff0000 | (puVar3 + 0xe)),CONCAT22(uVar2,puVar4),
                  CONCAT22(uVar6,puVar5));
  return;
}



// WARNING: Unable to use type for symbol uVar3
pub fn unk_draw_op_1020_0c3e(param_1: *mut astruct_771)

{
  let mut puVar2: *mut u32;
  HDC16 *hpal;
  HPALETTE16 obj;
  astruct_771 *struct_1;
  astruct_842 *iVar5;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uStack40: u16;
  HDC16 local_24;
  u8 paintstruct_22 [0x20];
  let mut uVar3: u32;
  let mut puVar1: *mut u32;
  let mut uVar4: u32;
  code **fn_ptr_1;

  uVar5 = (param_1 >> 0x10);
  struct_1 = (astruct_771 *)param_1;
  local_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct_1.hwnd_0x4);
  uVar3 = struct_1.field5_0x6;
  uVar6 = (uVar3 >> 0x10);
  iVar5 = (astruct_842 *)uVar3;
  puVar2 = &iVar5.field_0xa;
  uStack40 = puVar2;
  if ((iVar5.field12_0xc | uStack40) != 0x0) {
    hpal = &local_24;
    uVar4 = *puVar2;
    fn_ptr_1 = (code **)(uVar4 + 0x8);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,uStack40,(puVar2 >> 0x10),hpal,&DAT_1050_1050);
    fn_ptr_1 = (code **)(uVar4 + 0x4);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,puVar2,struct_1.field7_0xc,struct_1.field6_0xa,&local_24,
                 &DAT_1050_1050);
    obj = SelectPalette16(0x0,(HPALETTE16)hpal,local_24);
    DeleteObject16(obj);
  }
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct_1.hwnd_0x4);
  return;
}



// WARNING: Unable to use type for symbol uVar4
pub fn win_ui_palette_op_1020_0cd2(astruct_775 *struct_param_1)

{
  let mut puVar2: *mut u32;
  code **ppcVar3;
  let mut uVar7: u16;
  HDC16 hdc;
  HDC16 hpal;
  HPALETTE16 hpal_00;
  let mut UVar4: u16;
  let mut extraout_DX: u16;
  let mut uVar8: u16;
  astruct_775 *struct_1;
  astruct_774 *iVar5;
  let mut uVar5: u16;
  let mut uVar6: u16;
  HDC16 hdc_00;
  astruct_13 *paStack10;
  let mut uStack6: u16;
  let mut puVar1: *mut u32;
  let mut uVar4: u32;

  uVar5 = (struct_param_1 >> 0x10);
  struct_1 = (astruct_775 *)struct_param_1;
  uVar4 = struct_1.field5_0x6;
  uVar6 = (uVar4 >> 0x10);
  iVar5 = (astruct_774 *)uVar4;
  puVar2 = &iVar5.field_0xa;
  uStack6 = puVar2;
  uVar7 = iVar5.field12_0xc | uStack6;
  if (uVar7 != 0x0) {
    ppcVar3 = (code **)(*puVar2 + 0x14);
    (**ppcVar3)();
    paStack10 = (astruct_13 *)CONCAT22(extraout_DX,uVar7);
    uVar8 = extraout_DX | uVar7;
    if (uVar8 != 0x0) {
      hdc = GetDC16(struct_1.field4_0x4);
      hpal = hdc;
      hdc_00 = hdc;
      create_palette_1008_4e38(paStack10,uVar8);
      hpal_00 = SelectPalette16(0x0,hpal,hdc_00);
      UVar4 = RealizePalette16(hdc);
      SelectPalette16(0x1,hpal_00,hdc);
      DeleteObject16(hpal);
      if (0x0 < UVar4) {
        InvalidateRect16(0x1,NULL,0x0);
      }
      ReleaseDC16(hdc,struct_1.field4_0x4);
      return;
    }
  }
  return;
}



StructD * pass1_1020_0d82(StructD *param_1,param_2: u8)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x3ab0;
  (param_1 + 0x2) = 0x1008;
  param_1.address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1020_0dc4(param_1: *mut StructA,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,mut param_5: u16 ,
                    mut param_6: u16 )

{
  char **ppcVar1;
  let iVar1: *mut StructA;
  let mut uVar2: u16;
  let mut in_stack_0000fe16: u16;
  let mut in_stack_0000fe1a: u16;
  let mut in_stack_0000fe59: u16;
  let mut in_stack_0000fe64: u16;
  let mut in_stack_0000fe68: u16;
  let mut in_stack_0000ff40: u16;
  let mut in_stack_0000ff44: u16;
  let mut in_stack_0000ff48: u16;
  let mut in_stack_0000ff8e: u16;
  let mut in_stack_0000ff92: u16;
  let mut in_stack_0000ff96: u16;

  struct_1020_790e(&param_1.field0_0x0,s_PCPOPMENU_1050_4256,param_2,param_3);
  uVar2 = (param_1 >> 0x10);
  iVar1 = (StructA *)param_1;
  &iVar1[0x1].field20_0x26 = 0x0;
  &iVar1[0x1].field22_0x2a = 0x0;
  iVar1[0x1].field25_0x2e = 0x0;
  param_1.field0_0x0 = 0x1384;
  iVar1.field1_0x2 = 0x1020;
  ppcVar1 = &iVar1.field60_0x5b;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(ppcVar1)),s_VrMode_1050_4260);
  iVar1.field140_0xac = 0x44c00000;
  window_op_1020_10a0(ppcVar1,param_4,(StructA *)(param_1 & 0xffff | uVar2 << 0x10),param_5,param_6,
                      in_stack_0000fe16,in_stack_0000fe1a,in_stack_0000fe59,in_stack_0000fe68,in_stack_0000ff40,
                      in_stack_0000ff44,in_stack_0000ff48,in_stack_0000ff8e,in_stack_0000ff92,in_stack_0000ff96,
                      in_stack_0000fe64);
  return;
}
pub fn pass1_1020_0e2c(param_1: *mut astruct_868)

{
  get_win_ui_info_op_1020_7a50(param_1);
  cleanup_ui_op_1020_1038(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar2
pub fn realize_palette_1020_0e46(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  HGDIOBJ16 puVar4;
  astruct_800 *iVar4;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar2: u32;
  let mut puVar2: *mut u32;
  code **fn_ptr_1;

  if (param_2 != 0x0) {
    uVar4 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0xf2);
    uVar5 = (uVar2 >> 0x10);
    iVar4 = (astruct_800 *)uVar2;
    puVar2 = iVar4.field102_0x66;
    fn_ptr_1 = (code **)(*puVar2 + 0x18);
    (**fn_ptr_1)();
    UnrealizeObject16((HGDIOBJ16)puVar2);
    uVar1 = (param_1 + 0xf2);
    RealizePalette16(*(HDC16 *)(uVar1 + 0x7c));
  }
  return;
}
pub fn pass1_1020_0e8e(mut param_1: i16,mut param_2: u16 ,mut param_3: i16,mut param_4: u16 ,mut param_5: i16,mut param_6: i16)

{
  code **ppcVar1;

  win_ui_cursor_op_1020_1294(param_2,CONCAT22(param_4,param_3),param_5,param_6);
  if (param_1 == 0x0) {
    ppcVar1 = (code **)((param_3 + 0x4) + 0x5c);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_help_op_1020_0ec4(mut param_1: u16 ,u32 *param_2,mut param_3: u16 )

{
  code **ppcVar1;
  char cVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  astruct_477 *paVar8;
  let mut puVar9: *mut u32;
  let mut uVar10: u32;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut wparam: WPARAM16;
  let mut hwnd: HWND16;
  let mut iVar11: i16;
  let mut in_stack_0000fff4: u16;
  let mut uVar12: u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar7 = (param_2 >> 0x10);
  uVar3 = param_2;
  if (param_3 == 0xfb) {
    puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff4,0x30),in_stack_0000fe9c,
                             in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
    uVar12 = puVar9;
    pass1_1010_375e(puVar9);
    ppcVar1 = (code **)(*param_2 + 0x14);
    (**ppcVar1)();
    uVar10 = pass1_1010_375e(puVar9 & 0xffff0000 | uVar12);
    uVar12 = (uVar10 >> 0x10);
    pass1_1010_4788(uVar10,uVar12,(uVar3 + 0xf2),(uVar10 & 0xffff | uVar12 << 0x10));
    return;
  }
  if (0xfb < param_3) {
    switch(param_3) {
    default:
      return;
    case 0x12a:
      hwnd = *(HWND16 *)(uVar3 + 0x8);
      wparam = 0xf012;
      break;
    case 0x12c:
      hwnd = *(HWND16 *)(uVar3 + 0x8);
      wparam = 0xf020;
    }
    PostMessage16(0x0,wparam,0x112,hwnd);
    return;
  }
  if (param_3 == 0xf3) {
    iVar11 = 0x3;
  }
  else {
    if (0xf3 < param_3) {
      return;
    }
    cVar2 = (char)param_3;
    if ((cVar2 + 0x91U) == 0x0) {
      uVar4 = FUN_1010_830a(param_3 & 0xff00 | (cVar2 + 0x91U),paVar6,unaff_CS,_u16_1050_14cc,0x1f8);
      WinHelp16(0x28,0x1,CONCAT22(paVar6,uVar4),*(HWND16 *)(uVar3 + 0x8));
      return;
    }
    if (cVar2 == 'r') {
      iVar11 = uVar3 + 0xa;
      uVar4 = uVar7;
      paVar8 = (astruct_477 *)
               mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(iVar11,0x30),in_stack_0000fe98,
                               in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
      uVar5 = (paVar8 >> 0x10);
      pass1_1010_3770(uVar5,paVar8,CONCAT22(uVar4,iVar11));
      pass1_1038_af40(uVar3,uVar5,_PTR_LOOP_1050_5b7c,(uVar3 + 0x8),0x3);
      return;
    }
    if (cVar2 == -0xf) {
      iVar11 = 0x1;
    }
    else {
      if (cVar2 != -0xe) {
        return;
      }
      iVar11 = 0x2;
    }
  }
  pass1_1010_4674(*(astruct_27 **)(uVar3 + 0xf2),iVar11,0x1010,&DAT_1050_1050);
  return;
}
