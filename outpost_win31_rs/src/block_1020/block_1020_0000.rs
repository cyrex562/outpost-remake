pub unsafe fn unk_draw_op_1020_0000(param_1: *mut astruct_840)

{
  let mut piVar1: *mut i16;
  let mut ppcVar2: *mut *mut code;
  let mut iVar4: *mut astruct_840;
  let mut uVar5: u16;
  let mut uVar4: u16;
  let mut local_c4: [u8;0x6] = [0;0x6];
  let mut local_be: *mut i16;
  let mut piStack184: *mut i16;
  let mut uStack182: u16;
  let mut local_b4: i16;
  let mut iStack178: i16;
  aiStack176: i16[0x3c];
  iStack56: *mut astruct_841;
  let mut iStack48: i16;
  let mut paStack46: *mut astruct_76;
  let mut local_2a: i16;
  let mut local_28: i16;
  let mut puStack38: *mut u32;
  let mut local_22: [u8;0x20] = [0;0x20];
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
  BeginPaint16(CONCAT22(0x1050,local_22),iVar4.field4_0x4);
  uVar3 = iVar4.field19_0x14;
  puStack38 = (uVar3 + 0xa);
  pass1_1008_3e94((param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x18)),CONCAT22(0x1050,&local_2a),
                  CONCAT22(0x1050,&local_28));
  uVar4 = 0x1008;
  pass1_1008_4480(puStack38,(param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x18)),
                  iVar4.field32_0x24);
  paStack46 = NULL;
  for (iStack48 = 0; iStack48 < 0x6; iStack48 += 1) {
    uVar2 = iVar4.field19_0x14;
    uVar4 = 0x1010;
    pass1_1010_2b78(uVar2,(uVar2 >> 0x10),iStack48,CONCAT22(0x1050,&local_b4));
    if (local_b4 == 0) {
      for (iStack56 = NULL; iVar3 = iStack56, iStack56 <= iStack178; iStack56 = iStack56 + 1) {
        piVar1 = aiStack176 + iStack56 * 0x3;
        uStack182 = &DAT_1050_1050;
        piStack184 = piVar1;
        if (aiStack176[iStack56 * 0x3 + 0x2] != 0) {
          paStack46 = pass1_1010_2b98(iVar4.field19_0x14,aiStack176[iStack56 * 0x3 + 0x2]);
          pass1_1008_3e54(CONCAT22(0x1050,&local_be),0x0,aiStack176[iVar3 * 0x3 + 0x1] + local_2a,
                          *piVar1 + local_28);
          uVar4 = 0x1008;
          pass1_1008_4480(puStack38,CONCAT22(0x1050,&local_be),paStack46);
        }
      }
    }
    else {
      local_be = CONCAT22(0x1050,aiStack176 + iStack178 * 0x3);
      if (aiStack176[iStack178 * 0x3 + 0x2] != 0) {
        paStack46 = pass1_1010_2b98(iVar4.field19_0x14,aiStack176[iStack178 * 0x3 + 0x2]);
        pass1_1008_3e54(CONCAT22(0x1050,local_c4),0x0,(local_be + 0x2) + local_2a,
                        *local_be + local_28);
        uVar4 = 0x1008;
        pass1_1008_4480(puStack38,CONCAT22(0x1050,local_c4),paStack46);
      }
    }
  }
  ppcVar2 = (*puStack38 + 0x4);
  (**ppcVar2)(uVar4,puStack38,(puStack38 >> 0x10),0x0,0x0,&iVar4.field_0xa,uVar5);
  EndPaint16(CONCAT22(0x1050,local_22),iVar4.field4_0x4);
  return;
}



pub unsafe fn pass1_1020_01a6(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1018_ed98(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub unsafe fn pass1_1020_01d8(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 ,
                    mut param_7: u32)

{
  let mut in_stack_0000ffd6: u16;

  unk_draw_op_1008_61b2(in_stack_0000ffd6,param_1,param_2,param_6,param_7);
  (param_1 + 0xe2) = 0;
  (param_1 + 0xe6) = 0;
  (param_1 + 0xea) = param_5;
  (param_1 + 0xec) = param_4;
  (param_1 + 0xee) = param_3;
  param_1.offset_0x0 = 0x45a;
  (param_1 + 0x2) = 0x1020;
  return;
}
pub unsafe fn pass1_1020_022c(struct_param_1: *mut astruct_29)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: *mut astruct_29;
  let mut uVar4: u16;

  uVar4 = (struct_param_1 >> 0x10);
  iVar4 = struct_param_1;
  struct_param_1.field0_0x0 = 0x45a;
  iVar4.field1_0x2 = 0x1020;
  puVar1 = &iVar4.field228_0xe6;
  uVar2 = (&iVar4.field228_0xe6 + 2);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  pass1_1008_57c4((struct_param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0xd2)));
  struct_param_1.field0_0x0 = 0x380a;
  iVar4.field1_0x2 = 0x1008;
  struct_param_1.field0_0x0 = 0x389a;
  iVar4.field1_0x2 = 0x1008;
  return;
}
pub unsafe fn pass1_1020_028c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16)

{
  let mut in_DX: u16;

  pass1_1010_3c9e((param_1 + 0xe2));
  pass1_1008_68c6(in_DX,param_1,param_2,param_3);
  return;
}
pub unsafe fn pass1_1020_02ae(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  pass1_1010_3cd0((iVar4 + 0xe2));
  destroy_win_1008_628e(param_1);
  puVar1 = (iVar4 + 0xe6);
  uVar2 = (iVar4 + 0xe8);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,1);
  }
  (iVar4 + 0xe6) = 0;
  pass1_1010_1dda((iVar4 + 0xe2));
  (iVar4 + 0xe2) = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_1020_0316(mut param_1: u16 ,param_2: *mut StructA)

{
  let mut uVar1: u32;
  let mut paVar2: *mut astruct_666;
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

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  create_window_ex_1008_9760(param_2);
  puVar6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(in_stack_0000fff2,1),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  paVar4 = (paVar4 & 0xffff0000 | puVar6 >> 0x10);
  uVar5 = (param_2 >> 0x10);
  iVar1 = param_2;
  iVar1[0x1].field11_0x16 = puVar6;
  iVar1[0x1].field12_0x18 = (puVar6 >> 0x10);
  uVar1 = &iVar1[0x1].field11_0x16;
  (uVar1 + 0x16) = iVar1[0x1].field15_0x1e;
  paVar2 = iVar1[0x1].field18_0x22;
  uVar1 = &iVar1[0x1].field11_0x16;
  *(astruct_666 **)(uVar1 + 0x12) = paVar2;
  struct_1010_3c52(paVar4,&iVar1[0x1].field11_0x16,&iVar1[0x1].field_0x20);
  mem_op_1000_179c(0x12,paVar4);
  puVar3 = (paVar4 | paVar2);
  if (puVar3.is_null() == false) {
    pass1_1020_04f6(puVar3,CONCAT22(paVar4,paVar2),iVar1.field4_0x8);
    iVar1[0x1].field13_0x1a = paVar2;
    iVar1[0x1].field14_0x1c = puVar3;
    return;
  }
  iVar1[0x1].field13_0x1a = 0;
  return;
}
pub unsafe fn post_msg_1020_03b2(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0xe2);
  PostMessage16(0x0,(uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}
pub unsafe fn post_msg_1020_03d6(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0xe2);
  PostMessage16(0x0,(uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}
pub unsafe fn post_msg_1020_03fa(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0xe2);
  PostMessage16(0x0,(uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}
pub unsafe fn draw_op_1020_041e(mut param_1: u32)

{
  fill_rect_1020_065e(*(astruct_754 **)(param_1 + 0xe6));
  return;
}



astruct_29 * pass1_1020_0434(param_1: *mut astruct_29,param_2: u8)

{
  pass1_1020_022c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_04f6(param_1: *mut u8,param_2: *mut astruct_662,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar4: *mut astruct_662;
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

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  param_2 = 0x389a;
  iVar4.field2_0x2 = 0x1008;
  param_2 = 0x3aa8;
  iVar4.field2_0x2 = 0x1008;
  iVar4.field3_0x4 = param_3;
  param_2 = 0x3ab0;
  iVar4.field2_0x2 = 0x1008;
  iVar4.field4_0x6 = NULL;
  iVar4.field5_0xa = 0;
  iVar4.field6_0xc = 0;
  iVar4.field7_0xe = 0;
  iVar4.field8_0x10 = 0;
  param_2 = 0x75a;
  iVar4.field2_0x2 = 0x1020;
  ppuVar8 = CONCAT22((in_stack_0000ffe0 >> 0x10),1);
  puVar6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,ppuVar8,in_stack_0000fe8a,in_stack_0000ffae,in_stack_0000ffb4,
                           in_stack_0000ffb8);
  paVar4 = (paVar4 & 0xffff0000 | puVar6 >> 0x10);
  iVar4.field4_0x6 = puVar6;
  uVar3 = (puVar6 >> 0x10);
  (&iVar4.field4_0x6 + 0x2) = uVar3;
  uVar7 = &iVar4.field4_0x6;
  ppcVar1 = (*iVar4.field4_0x6 + 0x4);
  (**ppcVar1)(0x1010,uVar7,uVar3,0x0,param_2,(ppuVar8 >> 0x10));
  puVar6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(uVar7,0x48),in_stack_0000fe80,in_stack_0000ffa4,
                           in_stack_0000ffaa,in_stack_0000ffae);
  iVar2 = puVar6;
  iVar4.field5_0xa = (iVar2 + 0xa);
  iVar4.field6_0xc = (iVar2 + 0xc);
  pass1_1008_3e94((puVar6 & 0xffff0000 | (iVar2 + 0xe)),
                  (param_2 & 0xffff0000 | ZEXT24(&iVar4.field8_0x10)),
                  (param_2 & 0xffff0000 | ZEXT24(&iVar4.field7_0xe)));
  return;
}
pub unsafe fn pass1_1020_05d6(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x75a;
  iVar1.address_offset_field_0x2 = 0x1020;
  if (&iVar1.field_0x6 != 0) {
    pass1_1010_1ea6(&iVar1.field_0x6,(param_1 & 0xffff | uVar1 << 0x10));
  }
  param_1.address_offset_field_0x0 = 0x3ab0;
  iVar1.address_offset_field_0x2 = 0x1008;
  param_1.address_offset_field_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}
pub unsafe fn post_win_msg_1020_061c(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (param_2 == 1) {
    (param_1 + 0x6) = 0;
    return;
  }
  if (param_2 != 0x2) {
    return;
  }
  uVar1 = (param_1 + 0x6);
  PostMessage16(0x0,(uVar1 + 0x16),0x111,HWND16_1050_0396);
  return;
}



// WARNING: Unable to use type for symbol uVar2
pub unsafe fn fill_rect_1020_065e(astruct754_param_1: *mut astruct_754)

{
  let mut uVar1: u32;
  struct754_var1: *mut astruct_754;
  let mut uVar4: u16;
  let mut rect_1: RECT16;
  let mut iStack48: i16;
  let mut iStack46: i16;
  let mut brush_handle_1: HBRUSH16;
  palette_handle_var42: *mut HDC16;
  let mut puStack40: *mut u32;
  let mut hdc_var_24: HDC16;
  let mut paintstruct_22: [u8;0x20] = [0;0x20];
  let mut uVar3: u16;
  let mut uVar2: u32;
  let mut fn_ptr_1: *mut *mut code;

  uVar4 = (astruct754_param_1 >> 0x10);
  struct754_var1 = astruct754_param_1;
  hdc_var_24 = BeginPaint16(CONCAT22(0x1050,paintstruct_22),&struct754_var1.field_0x4);
  if (0x280 < struct754_var1.field7_0xa) {
    brush_handle_1 = CreateSolidBrush16(0x210070b);
    rect_1.x = 0;
    rect_1.y = 0;
    iStack48 = struct754_var1.field7_0xa + -0x1;
    iStack46 = struct754_var1.field8_0xc + -0x1;
    FillRect16(brush_handle_1,&rect_1,&DAT_1050_1050);
    DeleteObject16(brush_handle_1);
  }
  uVar2 = struct754_var1.field6_0x6;
  puStack40 = (uVar2 + 0xe);
  palette_handle_var42 = &hdc_var_24;
  uVar3 = puStack40;
  uVar1 = *puStack40;
  fn_ptr_1 = (uVar1 + 0x8);
  (**fn_ptr_1)(s_tile2_bmp_1050_1538,uVar3,(puStack40 >> 0x10),palette_handle_var42,&DAT_1050_1050
              );
  fn_ptr_1 = (uVar1 + 0x4);
  (**fn_ptr_1)(s_tile2_bmp_1050_1538,puStack40,(puStack40 >> 0x10),struct754_var1.field10_0x10,
               struct754_var1.field9_0xe,&hdc_var_24,&DAT_1050_1050,uVar3);
  palette_handle_var42 = SelectPalette16(0x0,palette_handle_var42,hdc_var_24);
  DeleteObject16(palette_handle_var42);
  EndPaint16(CONCAT22(0x1050,paintstruct_22),&struct754_var1.field_0x4);
  return;
}



pub unsafe fn pass1_1020_0734(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1020_05d6(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub unsafe fn struct_1020_0762(param_1: *mut astruct_20,mut param_2: u32,param_3: *mut u32,mut param_4: u16 ,mut param_5: u32,mut param_6: u32)

{
  let mut iVar1: *mut astruct_20;
  let mut uVar1: *mut astruct_20;

  pass1_1020_01d8(param_1,param_2,(param_2 >> 0x10),param_4,param_5,(param_5 >> 0x10),
                  param_6);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field6_0xe = 0;
  iVar1[0x1].field7_0x10 = *param_3;
  param_1.offset_0x0 = 0x81a;
  iVar1.base_0x2 = 0x1020;
  return;
}
pub unsafe fn pass1_1020_07aa(mut param_1: u16 ,param_2: *mut astruct_19)

{
  let mut iVar1: *mut astruct_19;
  let mut uVar2: *mut astruct_19;
  let mut local_16: [u8;0x14] = [0;0x14];

  draw_op_1020_041e(param_2);
  uVar2 = (param_2 >> 0x10);
  iVar1 = param_2;
  if (iVar1[0x2].field24_0x30 == 0) {
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
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_08b6(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  let mut iVar1: *mut astruct_20;
  let mut uVar1: u16;
  let mut paVar2: *mut astruct_20;
  let mut in_stack_0000ffd6: u16;

  paVar2 = unk_draw_op_1008_61b2(in_stack_0000ffd6,param_1,0x1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field2_0x4 = 0;
  (&iVar1[0x1].field2_0x4 + 0x2) = 0;
  param_1.offset_0x0 = 0xb0e;
  iVar1.base_0x2 = 0x1020;
  win_1008_5c5c(0x0,(paVar2 >> 0x10),_u16_1050_02a0,0x1d4);
  return;
}
pub unsafe fn send_win_msg_1020_08fe(param_1: *mut astruct_63)

{
  let mut hwnd: HWND16;
  let mut lVar1: i32;
  let mut BVar2: bool;
  let mut iVar2: *mut astruct_63;
  let mut uVar2: *mut astruct_63;

  uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  param_1.field0_0x0 = 0xb0e;
  iVar2.field1_0x2 = 0x1020;
  if (iVar2.field229_0xe8 != 0) {
    lVar1 = iVar2.field229_0xe8;
    hwnd = (lVar1 + 0x6);
    BVar2 = IsWindow16(hwnd);
    if (BVar2 != 0) {
      SendMessage16(0x0,0x1,0x111,hwnd);
    }
    iVar2.field229_0xe8 = 0;
  }
  pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(&iVar2.field208_0xd2)));
  param_1.field0_0x0 = 0x380a;
  iVar2.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x389a;
  iVar2.field1_0x2 = 0x1008;
  return;
}
pub unsafe fn send_msg_1020_097e(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (((iVar2 + 0xea) | (iVar2 + 0xe8)) != 0) {
    uVar1 = (iVar2 + 0xe8);
    SendMessage16(0x0,0x1,0x111,(uVar1 + 0x6));
    (iVar2 + 0xe8) = 0;
  }
  return;
}
pub unsafe fn win_1020_09ba(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut StructA)

{
  let mut puVar1: *mut u8;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let iVar1: *mut StructA;
  let mut uVar3: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_2);
  create_window_ex_1008_9760(param_3);
  mem_op_1000_179c(0xe,paVar2);
  puVar1 = (paVar2 | param_1);
  iVar1 = param_3;
  uVar3 = (param_3 >> 0x10);
  if (puVar1.is_null() == false) {
    struct_1020_0baa(puVar1,CONCAT22(paVar2,param_1),iVar1.field4_0x8);
    iVar1[0x1].field11_0x16 = param_1;
    iVar1[0x1].field12_0x18 = puVar1;
    return;
  }
  iVar1[0x1].field11_0x16 = 0;
  return;
}
pub unsafe fn pass1_1020_0a0c(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  destroy_win_1008_628e(param_1);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0xe2);
  uVar2 = (iVar4 + 0xe4);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,1);
  }
  (iVar4 + 0xe2) = 0;
  (iVar4 + 0xe6) = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_0a52(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  uVar2 = (param_2 >> 0x10);
  uVar1 = param_2;
  unk_draw_op_1020_0c3e(*(astruct_771 **)(uVar1 + 0xe2));
  if ((uVar1 + 0xe6) == 0) {
    (uVar1 + 0xe6) = 0x1;
    (_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
    uVar3 = pass1_1038_af40(uVar1,param_1,_PTR_LOOP_1050_5b7c,(uVar1 + 0x8),0x6);
    (uVar1 + 0xe8) = uVar3;
    (uVar1 + 0xea) = (uVar3 >> 0x10);
  }
  return;
}
pub unsafe fn pass1_1020_0aa6(mut param_1: u32)

{
  win_ui_palette_op_1020_0cd2(*(astruct_775 **)(param_1 + 0xe2));
  return;
}
pub unsafe fn pass1_1020_0abc(mut param_1: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0xe6) != 0) {
    ppcVar1 = ((param_1 + 0xe8) + 0x10);
    (**ppcVar1)();
  }
  return;
}



astruct_63 * pass1_1020_0ae8(param_1: *mut astruct_63,param_2: u8)

{
  send_win_msg_1020_08fe(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn struct_1020_0baa(param_1: *mut u8,param_2: *mut u16,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar2: *mut astruct_276;
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

  paVar1 = CONCAT22(in_register_0000000a,param_1);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  *param_2 = 0x389a;
  iVar2.field2_0x2 = 0x1008;
  *param_2 = 0x3aa8;
  iVar2.field2_0x2 = 0x1008;
  iVar2.field3_0x4 = param_3;
  *param_2 = 0x3ab0;
  iVar2.field2_0x2 = 0x1008;
  iVar2.field4_0x6 = 0;
  iVar2.field6_0xa = 0;
  iVar2.field7_0xc = 0;
  *param_2 = 0xdbc;
  iVar2.field2_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe2,0x7),in_stack_0000fe8a,
                           in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
  iVar2.field4_0x6 = puVar3;
  iVar2.field5_0x8 = (puVar3 >> 0x10);
  puVar5 = &iVar2.field6_0xa;
  puVar4 = &iVar2.field7_0xc;
  uVar6 = uVar2;
  puVar3 = mixed_1010_20ba((paVar1 & 0xffff0000 | puVar3 >> 0x10),_u16_1050_0ed0,
                           CONCAT22(puVar4,0x48),in_stack_0000fe82,in_stack_0000ffa6,in_stack_0000ffac,
                           in_stack_0000ffb0);
  pass1_1008_3e94((puVar3 & 0xffff0000 | (puVar3 + 0xe)),CONCAT22(uVar2,puVar4),
                  CONCAT22(uVar6,puVar5));
  return;
}



// WARNING: Unable to use type for symbol uVar3
pub unsafe fn unk_draw_op_1020_0c3e(param_1: *mut astruct_771)

{
  let mut puVar2: *mut u32;
  hpal: *mut HDC16;
  let mut obj: HPALETTE16;
  struct_1: *mut astruct_771;
  let mut iVar5: *mut astruct_842;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uStack40: u16;
  let mut local_24: HDC16;
  let mut paintstruct_22: [u8;0x20] = [0;0x20];
  let mut uVar3: u32;
  let mut puVar1: *mut u32;
  let mut uVar4: u32;
  let mut fn_ptr_1: *mut *mut code;

  uVar5 = (param_1 >> 0x10);
  struct_1 = param_1;
  local_24 = BeginPaint16(CONCAT22(0x1050,paintstruct_22),struct_1.hwnd_0x4);
  uVar3 = struct_1.field5_0x6;
  uVar6 = (uVar3 >> 0x10);
  iVar5 = uVar3;
  puVar2 = &iVar5.field_0xa;
  uStack40 = puVar2;
  if ((iVar5.field12_0xc | uStack40) != 0) {
    hpal = &local_24;
    uVar4 = *puVar2;
    fn_ptr_1 = (uVar4 + 0x8);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,uStack40,(puVar2 >> 0x10),hpal,&DAT_1050_1050);
    fn_ptr_1 = (uVar4 + 0x4);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,puVar2,struct_1.field7_0xc,struct_1.field6_0xa,&local_24,
                 &DAT_1050_1050);
    obj = SelectPalette16(0x0,hpal,local_24);
    DeleteObject16(obj);
  }
  EndPaint16(CONCAT22(0x1050,paintstruct_22),struct_1.hwnd_0x4);
  return;
}



// WARNING: Unable to use type for symbol uVar4
pub unsafe fn win_ui_palette_op_1020_0cd2(struct_param_1: *mut astruct_775)

{
  let mut puVar2: *mut u32;
  let mut ppcVar3: *mut *mut code;
  let mut uVar7: u16;
  let mut hdc: HDC16;
  let mut hpal: HDC16;
  let mut hpal_00: HPALETTE16;
  let mut UVar4: u16;
  let mut extraout_DX: u16;
  let mut uVar8: u16;
  struct_1: *mut astruct_775;
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
    paStack10 = CONCAT22(extraout_DX,uVar7);
    uVar8 = extraout_DX | uVar7;
    if (uVar8 != 0) {
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



pub unsafe fn pass1_1020_0d82(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x3ab0;
  (param_1 + 0x2) = 0x1008;
  param_1.address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub unsafe fn pass1_1020_0dc4(param_1: *mut StructA,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,mut param_5: u16 ,
                    mut param_6: u16 )

{
  ppcVar1: *mut *mut c_char;
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
  iVar1 = param_1;
  iVar1[0x1].field20_0x26 = 0;
  iVar1[0x1].field22_0x2a = 0;
  iVar1[0x1].field25_0x2e = 0;
  param_1.field0_0x0 = 0x1384;
  iVar1.field1_0x2 = 0x1020;
  ppcVar1 = &iVar1.field60_0x5b;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(ppcVar1)),s_VrMode_1050_4260);
  iVar1.field140_0xac = 0x44c00000;
  window_op_1020_10a0(ppcVar1,param_4,(param_1 & 0xffff | uVar2 << 0x10),param_5,param_6,
                      in_stack_0000fe16,in_stack_0000fe1a,in_stack_0000fe59,in_stack_0000fe68,in_stack_0000ff40,
                      in_stack_0000ff44,in_stack_0000ff48,in_stack_0000ff8e,in_stack_0000ff92,in_stack_0000ff96,
                      in_stack_0000fe64);
  return;
}
pub unsafe fn pass1_1020_0e2c(param_1: *mut astruct_868)

{
  get_win_ui_info_op_1020_7a50(param_1);
  cleanup_ui_op_1020_1038(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar2
pub unsafe fn realize_palette_1020_0e46(mut param_1: u32,mut param_2: i16)

{
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
pub unsafe fn pass1_1020_0e8e(mut param_1: i16,mut param_2: u16 ,mut param_3: i16,mut param_4: u16 ,mut param_5: i16,mut param_6: i16)

{
  let mut ppcVar1: *mut *mut code;

  win_ui_cursor_op_1020_1294(param_2,CONCAT22(param_4,param_3),param_5,param_6);
  if (param_1 == 0) {
    ppcVar1 = ((param_3 + 0x4) + 0x5c);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_help_op_1020_0ec4(mut param_1: u16 ,param_2: *mut u32,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut cVar2: u8;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  let mut paVar8: *mut astruct_477;
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

  paVar6 = CONCAT22(in_register_0000000a,param_1);
  uVar7 = (param_2 >> 0x10);
  uVar3 = param_2;
  if (param_3 == 0xfb) {
    puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,CONCAT22(in_stack_0000fff4,0x30),in_stack_0000fe9c,
                             in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
    uVar12 = puVar9;
    pass1_1010_375e(puVar9);
    ppcVar1 = (*param_2 + 0x14);
    (**ppcVar1)();
    uVar10 = pass1_1010_375e(puVar9 & 0xffff0000 | uVar12);
    uVar12 = (uVar10 >> 0x10);
    pass1_1010_4788(uVar10,uVar12,(uVar3 + 0xf2),(uVar10 & 0xffff | uVar12 << 0x10));
    return;
  }
  if (0xfb < param_3) {
    switch(param_3) {
    _ =>
      return;
    0x12a =>
      hwnd = (uVar3 + 0x8);
      wparam = 0xf012;
      break;
    0x12c =>
      hwnd = (uVar3 + 0x8);
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
    cVar2 = param_3;
    if ((cVar2 + 0x91U) == 0) {
      uVar4 = FUN_1010_830a(param_3 & 0xff00 | (cVar2 + 0x91U),paVar6,unaff_CS,_u16_1050_14cc,0x1f8);
      WinHelp16(0x28,0x1,CONCAT22(paVar6,uVar4),(uVar3 + 0x8));
      return;
    }
    if (cVar2 == 'r') {
      iVar11 = uVar3 + 0xa;
      uVar4 = uVar7;
      paVar8 =
               mixed_1010_20ba(paVar6,_u16_1050_0ed0,CONCAT22(iVar11,0x30),in_stack_0000fe98,
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
