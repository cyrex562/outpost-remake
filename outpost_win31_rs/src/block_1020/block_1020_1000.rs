
pub fn enable_menu_1020_1000(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,HMENmut param_5: u16 )

{
  if (param_4 != 0) {
    return;
  }
  EnableMenuItem16(0x400,0x3,param_5);
  return;
}
pub fn FUN_1020_101f()

{
  return;
}
pub fn pass1_1020_1022(mut param_1: u32)

{
  draw_op_1020_15de((param_1 + 0xf6));
  return;
}
pub fn cleanup_ui_op_1020_1038(param_1: *mut astruct_868)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  iVar4: *mut astruct_868;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  DestroyIcon16(iVar4.hicon_0xc2);
  iVar4.hicon_0xc2 = 0;
  iVar4.field8_0x8 = 0;
  puVar1 = iVar4.field241_0xf6;
  uVar2 = iVar4.field242_0xf8;
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)(s_tile2_bmp_1050_1538,puVar1,uVar2,1);
  }
  &iVar4.field241_0xf6 = 0;
  pass1_1010_1dda(iVar4.field240_0xf2);
  iVar4.field240_0xf2 = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn window_op_1020_10a0(param_1: *mut astruct_57,param_2: *mut astruct_57,struct_param_1: *mut StructA,mut param_4: u16 ,mut param_5: u16 ,
                        mut param_6: u16 ,mut param_7: u16 ,mut param_8: u16 ,mut param_9: u16 ,mut param_10: u16 ,
                        mut param_11: u16 ,mut param_12: u16 ,mut param_13: u16 ,mut param_14: u16 ,
                        mut param_15: u16 ,mut param_16: u16 )

{
  let mut uVar1: u32;
  let mut ppcVar2: *mut *mut code;
  paVar3: *mut astruct_160;
  INT16 *pIVar4;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut paVar9: *mut Struct57;
  let mut unaff_SI: u16;
  let mut puVar10: *mut u32;
  let mut uVar11: u32;
  let mut uVar12: u32;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let struct_1: *mut StructA;
  let mut paVar8: *mut Struct57;

  struct_1 = struct_param_1;
  uVar14 = (struct_param_1 >> 0x10);
  create_window_ex_1008_9760(struct_param_1);
  mem_op_1000_179c(0x42,param_2);
  uVar5 = param_2 | param_1;
  paVar8 = (param_2 & 0xffff0000 | uVar5);
  if (uVar5 != 0) {
    pass1_1008_3bd6(paVar8,param_1,param_2,0x0,0x1f009b,0x0,0x740075,
                    CONCAT22(struct_1.field4_0x8,0xf1),param_4,param_6,param_7,param_10,param_11,param_12);
  }
  mem_op_1000_179c(0x42,paVar8);
  uVar5 = paVar8 | param_1;
  paVar9 = (paVar8 & 0xffff0000 | uVar5);
  if (uVar5 != 0) {
    pass1_1008_3bd6(paVar9,param_1,paVar8,0x0,0x31009b,0x0,0x760077,
                    CONCAT22(struct_1.field4_0x8,0xf2),param_4,param_6,param_7,param_10,param_11,param_12);
  }
  mem_op_1000_179c(0x42,paVar9);
  uVar5 = paVar9 | param_1;
  paVar8 = (paVar9 & 0xffff0000 | uVar5);
  if (uVar5 != 0) {
    pass1_1008_3bd6(paVar8,param_1,paVar9,0x0,0x77009b,0x0,0x780079,
                    CONCAT22(struct_1.field4_0x8,0xf3),param_4,param_6,param_7,param_10,param_11,param_12);
  }
  puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2d),param_9,param_5,param_14,
                            param_15);
  paVar8 = (paVar8 & 0xffff0000 | puVar10 >> 0x10);
  struct_1[0x1].field20_0x26 = puVar10;
  uVar6 = (puVar10 >> 0x10);
  struct_1[0x1].field21_0x28 = uVar6;
  struct_1[0x1].field10_0x14 = struct_1[0x1].field20_0x26;
  struct_1[0x1].field11_0x16 = uVar6;
  paVar3 = LoadIcon16(s_PLNTICON_1050_4267,HINSTANCE16_1050_038c);
  *(astruct_160 **)&struct_1.field_0xc2 = paVar3;
  uVar1 = &struct_1[0x1].field20_0x26;
  uVar7 = uVar1;
  ppcVar2 = (*&struct_1[0x1].field20_0x26 + 0x30);
  (**ppcVar2)(s_tile2_bmp_1050_1538,uVar7,(uVar1 >> 0x10),paVar3);
  mem_op_1000_179c(0x24,paVar8);
  uVar5 = paVar8 | paVar3;
  paVar9 = (paVar8 & 0xffff0000 | uVar5);
  if (uVar5 == 0) {
    &struct_1[0x1].field22_0x2a = 0;
  }
  else {
    unk_win_ui_op_1020_1418(CONCAT22(paVar8,paVar3),struct_param_1,param_5);
    struct_1[0x1].field22_0x2a = paVar3;
    &struct_1[0x1].field_0x2c = paVar9;
  }
  &struct_1[0x1].field14_0x1c = &struct_1[0x1].field22_0x2a;
  puVar10 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,CONCAT22(uVar7,0x2f),param_16,param_4,param_13,param_14)
  ;
  uVar12 = paVar9 & 0xffff0000;
  uVar11 = pass1_1018_04b8(puVar10);
  paVar8 = (uVar12 & 0xffff0000 | uVar11 >> 0x10);
    // WARNING: Load size is inaccurate
  pass1_1010_41d6(paVar8,struct_1[0x1].field20_0x26,uVar11);
  uVar12 = pass1_1010_451a(paVar8,&struct_1[0x1].field20_0x26);
  uVar7 = (uVar12 >> 0x10);
  pIVar4 = uVar12;
  uVar1 = struct_param_1;
  ppcVar2 = (uVar1 + 0x14);
  (**ppcVar2)(0x1010,struct_param_1,0x0,uVar12,uVar7);
  uVar13 = 0x1;
  ppcVar2 = (uVar1 + 0x10);
  (**ppcVar2)();
  pass1_1010_459e(*(astruct_27 **)&struct_1[0x1].field20_0x26);
  ppcVar2 = (*&struct_1[0x1].field20_0x26 + 0x10);
  (**ppcVar2)(0x1010,&struct_1[0x1].field20_0x26,struct_param_1,uVar13);
  MoveWindow16(0x1,pIVar4[0x3],pIVar4[0x2],pIVar4[0x1],*pIVar4,struct_1.field4_0x8);
  UpdateWindow16(struct_1.field4_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_cursor_op_1020_1294(mut param_1: u16 ,mut param_2: u32,mut param_3: i16,mut param_4: i16)

{
  let mut uVar1: u32;
  let mut ppcVar2: *mut *mut code;
  let mut in_AX: u16;
  let mut hcursor: HCURSOR16;
  let mut hcursor_00: HCURSOR16;
  let mut in_register_0000000a: u16;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_stack_0000ffc2: u16;
  let mut local_12: i16;
  let mut local_10: i16;
  let mut puStack14: *mut u16;
  let mut puStack10: *mut u32;
  let mut local_6: i16;
  let mut iStack4: i16;

  pass1_1030_8344(_u16_1050_5748,0x4000001);
  if ((param_1 | in_AX) == 0) {
    local_6 = param_4;
    iStack4 = param_3;
    uVar5 = (param_2 >> 0x10);
    iVar4 = param_2;
    puStack10 = pass1_1010_40cc(param_3,0x0,(iVar4 + 0xf2));
    uVar3 = CONCAT22(in_register_0000000a,(puStack10 >> 0x10));
    uVar1 = (iVar4 + 0xf2);
    puStack14 = (uVar1 & 0xffff0000 | (uVar1 + 0x76));
    pass1_1008_3e94(puStack14,CONCAT22(0x1050,&local_12),CONCAT22(0x1050,&local_10));
    local_6 -= local_10;
    iStack4 -= local_12;
    hcursor = pt_in_rect_1010_40f8
                        (uVar3,(iVar4 + 0xf2),CONCAT22(0x1050,&local_6),in_stack_0000ffc2);
    if (hcursor != 0xffff) {
      hcursor_00 = LoadCursor16(0x7f02,0x0);
      SetCursor16(hcursor_00);
      ppcVar2 = (*puStack10 + 0x4);
      (**ppcVar2)();
      pass1_1008_3e0e(param_2);
      SetCursor16(hcursor);
    }
  }
  return;
}



pub fn pass1_1020_135e(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  cleanup_menu_ui_op_1020_795c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_ui_op_1020_1418(param_1: *mut astruct_40,param_2: *mut StructA,mut param_3: u16 )

{
  let mut uVar1: u32;
  paVar2: *mut astruct_13;
  let mut ppcVar3: *mut *mut code;
  pHVar4: *mut HDC16;
  let pSVar5: *mut StructA;
  let mut puVar6: *mut u8;
  let mut in_EDX: u32;
  let mut paVar7: *mut Struct57;
  iVar5: *mut astruct_40;
  let mut uVar9: u16;
  let mut puVar10: *mut u16;
  let mut puVar11: *mut u32;
  let mut in_stack_0000fe84: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb2: u16;
  local_8: HDC16;
  let mut puStack6: *mut u32;
  let mut uVar8: u16;

  uVar8 = (in_EDX >> 0x10);
  get_sys_metrics_1020_7c1a(param_1,param_2);
  uVar9 = (param_1 >> 0x10);
  iVar5 = param_1;
  &iVar5.field17_0x14 = 0;
  iVar5.field20_0x18 = NULL;
  puVar10 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar5.field_0x1e)));
  paVar7 = CONCAT22(uVar8,(puVar10 >> 0x10));
  param_1.field0_0x0 = 0x1730;
  iVar5.field1_0x2 = 0x1020;
  puVar11 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,CONCAT22(param_3,0x2d),in_stack_0000fe84,
                            in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
  iVar5.field17_0x14 = puVar11;
  &iVar5.field_0x16 = (puVar11 >> 0x10);
  puStack6 = mixed_1010_20ba((paVar7 & 0xffff0000 | puVar11 >> 0x10),_u16_1050_0ed0,
                             CONCAT22(param_3,0x29),in_stack_0000fe84,in_stack_0000ffa8,in_stack_0000ffae,
                             in_stack_0000ffb2);
  puVar6 = (puStack6 >> 0x10);
  uVar1 = &iVar5.field17_0x14;
  ppcVar3 = (*&iVar5.field17_0x14 + 0x4);
  (**ppcVar3)(0x1010,uVar1,(uVar1 >> 0x10),0x0,param_1);
  local_8 = GetDC16(iVar5.hwnd_0x4);
  uVar1 = &iVar5.field17_0x14;
  *(uVar1 + 0x7c) = local_8;
  uVar1 = &iVar5.field17_0x14;
  pSVar5 = *(StructA **)(uVar1 + 0x66);
  iVar5.field20_0x18 = pSVar5;
  ppcVar3 = (iVar5.field20_0x18 + 0x14);
  (**ppcVar3)();
  paVar2 = (puStack6 + 0xe);
  pass1_1008_4d84(puVar6,(pSVar5 & 0xffff | ZEXT24(puVar6) << 0x10),paVar2);
  pHVar4 = palette_op_1008_4e08(&local_8,puVar6,paVar2,CONCAT22(0x1050,&local_8));
  iVar5.field21_0x1c = pHVar4;
  return;
}



// WARNING: Unable to use type for symbol uVar1
pub fn win_ui_op_1020_150e(param_1: *mut StructD)

{
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
    pass1_1010_1ea6(iVar1.field12_0x14,(param_1 & 0xffff | uVar2 << 0x10));
  }
  uVar1 = iVar1.field12_0x14;
  obj = SelectPalette16(0x0,&iVar1.field_0x1c,*(uVar1 + 0x7c));
  &iVar1.field_0x1c = obj;
  DeleteObject16(obj);
  param_1.address_offset_field_0x0 = 0x3ab0;
  iVar1.address_offset_field_0x2 = 0x1008;
  param_1.address_offset_field_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}
pub fn invalidate_rect_1020_157c(mut param_1: u32,mut param_2: i16)

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut local_a: RECT16;
  let mut uStack4: u16;

  iVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  if (param_2 == 1) {
    (iVar2 + 0x14) = 0;
    return;
  }
  if (param_2 == 0x2) {
    BVar1 = IsIconic16((iVar2 + 0x4));
    if (BVar1 == 0) {
      local_a.x = (iVar2 + 0x4);
      GetClientRect16(&local_a,&DAT_1050_1050);
      uStack4 = 0x9a;
      InvalidateRect16(0x0,&local_a,&DAT_1050_1050);
      return;
    }
  }
  return;
}



// WARNING: Unable to use type for symbol uVar2
pub fn draw_op_1020_15de(param_1: *mut astruct_779)

{
  let mut ppcVar1: *mut *mut code;
  let mut BVar2: bool;
  iVar3: *mut astruct_779;
  let mut uVar3: u16;
  let mut uVar4: u16;
  paVar3: *mut astruct_76;
  local_24: HDC16;
  let mut local_22: [u8;0x20] = [0;0x20];
  let mut uVar2: u32;
  let mut uVar1: u32;

  uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  local_24 = BeginPaint16(CONCAT22(0x1050,local_22),&iVar3.field_0x4);
  BVar2 = IsIconic16(&iVar3.field_0x4);
  if (BVar2 == 0) {
    uVar4 = 0x1010;
    paVar3 = pass1_1010_454a(iVar3.field20_0x14);
    if (((paVar3 >> 0x10) | paVar3) != 0) {
      uVar1 = iVar3.field20_0x14;
      uVar4 = 0x1008;
      pass1_1008_4480((iVar3 + 1),(uVar1 & 0xffff0000 | (uVar1 + 0x76)),paVar3);
    }
    uVar2 = (iVar3 + 1);
    ppcVar1 = (*(iVar3 + 1) + 0x4);
    (**ppcVar1)(uVar4,uVar2,(uVar2 >> 0x10),0x0,0x0,&local_24,&DAT_1050_1050);
  }
  else {
    draw_op_1020_1674(param_1);
  }
  EndPaint16(CONCAT22(0x1050,local_22),&iVar3.field_0x4);
  return;
}



// WARNING: Unable to use type for symbol puVar1
pub fn draw_op_1020_1674(param_1: *mut astruct_779)

{
  hdc: HDC16;
  struct_1: *mut astruct_779;
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
      GetClientRect16(&rect_e,&DAT_1050_1050);
      rect_1a.x = 0;
      rect_1a.y = 0;
      iStack22 = (iStack10 - rect_e.x) + -0x1;
      iStack20 = (iStack8 - rect_e.y) + -0x1;
      puVar1 = struct_1.field20_0x14;
      hdc = *(puVar1 + 0x7c);
      iStack18 = iStack20;
      iStack16 = iStack22;
      FillRect16(brush_handle,&rect_1a,(HDC16)&DAT_1050_1050);
      DrawIcon16(icon_handle,0x2,0x2,hdc);
    }
  }
  return;
}



pub fn pass1_1020_170a(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  win_ui_op_1020_150e(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1020_1738(param_1: *mut astruct_57,mut param_2: u16 ,mut param_3: u32)

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfcd,(param_3 + 0x8));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 1) = 0;
  &iVar1[0x1].field2_0x4 = 0;
  iVar1[0x1].field4_0x8 = 0;
  param_1.field0_0x0 = 0x1e7a;
  iVar1.field1_0x2 = 0x1020;
  return;
}
pub fn pass1_1020_1780(param_1: u32)

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = (*param_1 + 0x6c);
  (**ppcVar1)();
  destroy_win_1040_8212(param_1);
  return;
}



// WARNING: Unable to use type for symbol puVar2
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn mixed_ui_op_1020_179c(mut param_1: u16 ,mut param_2: u16 ,StructB *structb_param_1)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  LPVOID pvVar1;
  let mut IVar1: i16;
  let mut iVar2: i16;
  puVar11: *mut astruct_816;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut x: i16;
  let mut y_offset: i16;
  let mut IVar2: i16;
  let mut HVar3: HWND16;
  let mut uVar8: u16;
  let mut puVar4: *mut u8;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  StructB *struct_9;
  let mut iVar9: i16;
  StructB *uVar10;
  let mut uVar13: u16;
  let mut uVar11: u16;
  let mut uVar14: u16;
  let mut puVar10: *mut u32;
  let mut in_stack_0000fd2a: u16;
  let mut in_stack_0000fd2e: u16;
  let mut in_stack_0000fd30: u16;
  let mut in_stack_0000fe4e: u16;
  let mut in_stack_0000fe52: u16;
  let mut in_stack_0000fe54: u16;
  let mut in_stack_0000fe58: u16;
  let mut in_stack_0000fe5a: u16;
  let mut in_stack_0000fe5c: u16;
  let mut in_stack_0000fe5e: u16;
  let mut uVar16: u16;
  let mut in_buffer_4: *mut c_char;
  let mut in_stack_0000fe88_00: u16;
  short in_buf_len_5;
  let mut x_6e: i16;
  let mut y: i16;
  let mut x106: u16;
  let mut y4: i16;
  let mut hwnd_10: HWND16;
  let mut cx: i16;
  let mut cy: i16;
  let mut uStack78: u16;
  let mut uStack76: u16;
  let mut uStack74: u32;
  let mut HStack70: HWND16;
  let mut uStack68: u32;
  let mut uStack64: u32;
pub fn *pvStack60;
  let mut uStack56: u16;
  char **ppcStack54;
  let mut uStack50: u32;
  local_2e: *mut astruct_92;
  let mut local_1c: RECT16;
  let mut iStack22: i16;
  let mut iStack20: i16;
  let mut iStack18: i16;
  let mut puStack16: *mut u32;
  INT16 *pIStack12;
  let mut uStack8: u16;
  let mut puStack6: *mut u32;
  let mut puVar1: *mut u32;
  let mut puVar2: u32;
  let mut uVar12: u16;
  let mut in_resc_id_3: *mut u8;
  let mut uVar15: u16;
  let mut in_stack_0000fe88: u16;
  let mut uVar9: u32;
  let mut fn_ptr_1: *mut *mut code;

  paVar8 = CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(structb_param_1);
  uVar15 = 0x89;
  puStack6 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,0x890009,in_stack_0000fd2e,in_stack_0000fe52,
                             in_stack_0000fe58,in_stack_0000fe5c);
  paVar8 = (paVar8 & 0xffff0000 | puStack6 >> 0x10);
  uVar1 = pass1_1010_65d0(puStack6,uVar15);
  uStack8 = (uVar1 == 0);
  uVar2 = pass1_1010_65d0(puStack6,0x86);
  if (uVar2 != 0) {
    uStack8 = 0;
  }
  puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(in_stack_0000fe88_00,0x39),in_stack_0000fd30,
                            in_stack_0000fe54,in_stack_0000fe5a,in_stack_0000fe5e);
  paVar8 = (paVar8 & 0xffff0000 | puVar10 >> 0x10);
  pvVar1 = (LPVOID)puVar10;
  uVar10 = (StructB *)(structb_param_1 >> 0x10);
  struct_9 = (StructB *)structb_param_1;
  struct_9[0x7].field1_0x2 = pvVar1;
  HVar3 = (puVar10 >> 0x10);
  struct_9[0x7].hwnd_0x6 = HVar3;
  uVar16 = struct_9[0x7].field1_0x2;
  fn_ptr_1 = (*&struct_9[0x7].field1_0x2 + 0x10);
  (**fn_ptr_1)(0x1010,uVar16,HVar3,uStack8);
  mem_op_1000_179c(0x12,paVar8);
  uStack76 = paVar8;
  puVar4 = (uStack76 | pvVar1);
  paVar8 = (paVar8 & 0xffff0000 | ZEXT24(puVar4));
  uStack78 = pvVar1;
  if (puVar4.is_null()) {
    &struct_9[0x7].lpvoid_field_0x8 = 0;
  }
  else {
    pass1_1020_1eea(puVar4,CONCAT22(uStack76,pvVar1),structb_param_1,struct_9.lpvoid_field_0x8);
    struct_9[0x7].lpvoid_field_0x8 = pvVar1;
    struct_9[0x7].max_count_field_0x10 = paVar8;
  }
  puVar1 = &struct_9[0x7].field1_0x2;
  pIStack12 = (puVar1 & 0xffff0000 | (puVar1 + 0xa));
  puStack16 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(uVar16,0x48),in_stack_0000fd2a,
                              in_stack_0000fe4e,in_stack_0000fe54,in_stack_0000fe58);
  GetClientRect16(&local_1c,&DAT_1050_1050);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  uVar13 = (pIStack12 >> 0x10);
  iVar9 = pIStack12;
  (iVar9 + 0x6) = IVar1 + iStack22;
  uVar11 = (puStack16 >> 0x10);
  iStack18 = (puStack16 + 0xa);
  iStack20 = (puStack16 + 0xc);
  (iVar9 + 0x2) = (iStack20 - (iVar9 + 0x6)) / 0x2;
  iVar2 = iStack18 - (iVar9 + 0x4);
  uVar5 = iVar2 >> 0xf;
  uVar9 = uVar5;
  *pIStack12 = iVar2 / 0x2;
  pass1_1028_dc52(CONCAT22(0x1050,&local_2e),0x1,0x0,0x100);
  uStack56 = 0;
  while( true ) {
    uVar6 = uVar9;
    puVar11 = &local_2e;
    pass1_1028_e4ec(CONCAT22(0x1050,puVar11));
    uStack50 = CONCAT22(uVar6,puVar11);
    uVar7 = uVar6 | puVar11;
    uVar9 = uVar7;
    if (uVar7 == 0) break;
    ppcStack54 = puVar11.field16_0x10;
    if (ppcStack54.is_null() == false) {
      pass1_1000_3cea(structb_param_1 & 0xffff0000 | ZEXT24(&struct_9.field8_0x10),*ppcStack54);
    }
  }
  uVar3 = pass1_1020_1da8(puVar11,0x0,structb_param_1);
  struct_9[0x7].field5_0xa = uVar3;
  uVar4 = pass1_1010_65d0(puStack6,0x86);
  if ((uVar4 == 0) || (struct_9[0x7].field5_0xa.is_null() == false)) {
    puVar2 = &struct_9[0x7].field1_0x2;
    (puVar2 + 0x2c) = 0;
    hwnd_10 = GetDlgItem16(0x175,struct_9.lpvoid_field_0x8);
    if (uStack8 != 0) {
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,&stack0xfe88,(short)&DAT_1050_1050
                );
      SetWindowText16(CONCAT13(0x10,CONCAT12(0x50,&stack0xfe88)),hwnd_10);
    }
    pvStack60 = MakeProcInstance16(HINSTANCE16_1050_038c,destroy_win_1020_1e1e);
    GetWindowRect16(CONCAT22(0x1050,&x_6e),hwnd_10);
    cx = _x106 - x_6e;
    cy = y4 - y;
    x = cx - (pIStack12 + 0x4);
    y_offset = GetSystemMetrics16(SM_CYCAPTION);
    MoveWindow16(0x0,cy,cx,y - y_offset,-x / 0x2,hwnd_10);
  }
  else {
    win_1008_5c7c(uVar4,uVar7,_u16_1050_02a0,0x9d0001);
    (struct_9 + 0x7).field0_0x0 = uVar4;
    pvStack60 = MakeProcInstance16(HINSTANCE16_1050_038c,destroy_win_1020_1dea);
  }
  EnumChildWindows1(0x0,pvStack60,struct_9.lpvoid_field_0x8);
  FreeProcInstance16(pvStack60);
  HStack70 = GetDlgItem16(0x1,struct_9.lpvoid_field_0x8);
  GetWindowRect16(CONCAT22(0x1050,&local_1c),HStack70);
  uStack64 = _param_2;
  local_1c.x = _param_2 - local_1c.x;
  uStack74 = CONCAT22(local_1c.x,iStack22 - local_1c.y);
  uStack68 = local_1c & 0xffff0000 | (-(local_1c.x - (pIStack12 + 0x4)) / 0x2);
  IVar2 = GetSystemMetrics16(SM_CYCAPTION);
  uStack68 = uStack68 & 0xffff | (uStack68 - IVar2) << 0x10;
  if (struct_9[0x7].field5_0xa.is_null()) {
    if (uStack8 == 0) goto LAB_1020_1b24;
    in_buf_len_5 = 0x72e;
    in_resc_id_3 = &stack0xfe88;
    in_buffer_4 = &DAT_1050_1050;
  }
  else {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,&stack0xfe88,(short)&DAT_1050_1050);
    HVar3 = GetDlgItem16(0x175,struct_9.lpvoid_field_0x8);
    SetWindowText16(CONCAT22(0x1050,&stack0xfe88),HVar3);
    in_buffer_4 = &stack0xfe88;
    in_buf_len_5 = (short)&DAT_1050_1050;
    in_resc_id_3 = 0x3ff;
  }
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),in_resc_id_3,in_buffer_4,
             in_buf_len_5);
  SetWindowText16(CONCAT22(0x1050,&stack0xfe88),HStack70);//
// LAB_1020_1b24:
  MoveWindow16(0x0,uStack74,(uStack74 >> 0x10),uStack68,uStack68,HStack70);
  uVar14 = (pIStack12 >> 0x10);
  iVar2 = pIStack12;
  SetWindowPos16(0x44,(iVar2 + 0x6),(iVar2 + 0x4),(iVar2 + 0x2),*pIStack12,0x0,
                 struct_9.lpvoid_field_0x8);
  return;
}
pub fn pass1_1020_1b68(astruct *param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  astruct *iVar4;
  astruct *uVar4;

  uVar4 = (astruct *)(param_1 >> 0x10);
  iVar4 = (astruct *)param_1;
  puVar1 = iVar4.field143_0x92;
  uVar2 = iVar4.field144_0x94;
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  &iVar4.field143_0x92 = 0;
  pass1_1010_4f48(iVar4.field142_0x8e);
  iVar4.field142_0x8e = NULL;
  return;
}



pub unsafe fn pass1_1020_1bb6(mut param_1: u32) -> u16

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = ((param_1 + 0x92) + 0x8);
  (**ppcVar1)();
  return 0x0;
}
pub fn enable_window_1020_1bd4
               (mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_901,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut bVar2: bool;
  let mut hwnd: HWND16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut uVar8: u16;
  let mut puStack12: *mut u32;
  let mut uVar7: u32;

  paVar6 = CONCAT22(in_register_0000000a,param_2);
  bVar2 = false;
  pass1_1020_1d8e(CONCAT13((param_4 >> 0x8),CONCAT12(param_4,param_3)),CONCAT22(param_6,param_5));
  if (param_1 != 0) {
    if (param_1 < 0x2) {
      bVar2 = true;
    }
    else {
      hwnd = GetDlgItem16(0x1,param_3.field6_0x6);
      pass1_1010_4e8c(param_3.field141_0x8e);
      param_1 = EnableWindow16(0x1,hwnd);
      pass1_1010_4df0(paVar6,param_3.field141_0x8e);
      if ((param_1 == 0) && (bVar2 = true, param_3.field146_0x96 == 0)) {
        param_1 = EnableWindow16(0x0,hwnd);
      }
    }
  }
  if (bVar2) {
    uVar8 = 0x1000;
    mem_op_1000_179c(0xb4,paVar6);
    uVar4 = paVar6 | param_1;
    uVar7 = paVar6 & 0xffff0000 | uVar4;
    if (uVar4 == 0) {
      iVar3 = 0;
      uVar5 = 0;
    }
    else {
      uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar3 = string_1040_8520(uVar7,CONCAT22(paVar6,param_1),param_3.field6_0x6,0x20030,0x62a057b)
      ;
      uVar5 = uVar7;
    }
    puStack12 = CONCAT22(uVar5,iVar3);
    ppcVar1 = (*puStack12 + 0x74);
    (**ppcVar1)(uVar8,iVar3,uVar5);
  }
  return;
}



pub fn post_win_msg_1020_1ca4(mut param_1: u32,param_2: *mut astruct_57,mut param_3: u16 ) -> BOOL16

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar6: u16;
  let mut puStack10: *mut u32;
  let mut uVar5: u32;

  uVar6 = (param_1 >> 0x10);
  if ((param_1 + 0x96) == 0) {
    pass1_1010_4df0(param_2,(param_1 + 0x8e));
    if (param_3 == 0) {
      uVar6 = 0x1000;
      mem_op_1000_179c(0xb4,param_2);
      uVar3 = param_2 | param_3;
      uVar5 = param_2 & 0xffff0000 | uVar3;
      if (uVar3 == 0) {
        iVar2 = 0;
        uVar4 = 0;
      }
      else {
        uVar6 = SUB42(&PTR_LOOP_1050_1040,0x0);
        iVar2 = string_1040_8520(uVar5,CONCAT22(param_2,param_3),HWND16_1050_0396,0x20030,0x62a057b)
        ;
        uVar4 = uVar5;
      }
      puStack10 = CONCAT22(uVar4,iVar2);
      ppcVar1 = (*puStack10 + 0x74);
      (**ppcVar1)(uVar6,iVar2,uVar4);
      return 0x0;
    }
    PostMessage16(0x0,0xde,0x111,HWND16_1050_0396);
  }
  return 0x1;
}
pub fn set_win_tet_1020_1d2a(param_1: *mut astruct_938,mut param_2: u16 ,void *in_win_text_3,mut param_4: u16 ,in_dlg_id_5: INT16)

{
  let mut hwnd: HWND16;

  hwnd = GetDlgItem16(param_4,param_1.field6_0x6);
  SetWindowText16(in_win_text_3,hwnd);
  return;
}
pub fn destroy_window_1020_1d4a(mut param_1: u32,mut param_2: i16)

{
  let mut in_AX: u16;
  let mut BVar1: bool;
  let mut in_EDX: u32;
  let mut uVar2: u16;

  if (param_2 != 0) {
    BVar1 = post_win_msg_1020_1ca4(param_1,in_EDX,in_AX);
    if (BVar1 != 0) {
      uVar2 = (param_1 >> 0x10);
      if ((param_1 + 0x96) != 0) {
        PostMessage16(0x0,0xee,0x111,HWND16_1050_0396);
      }
      DestroyWindow16((param_1 + 0x6));
    }
  }
  return;
}
pub fn pass1_1020_1d8e(mut param_1: u32,mut param_2: u32)

{
  pt_in_rect_1010_4e08((param_1 + 0x8e),param_2,(param_2 >> 0x10));
  return;
}



// WARNING: Unable to use type for symbol uVar1

pub unsafe fn pass1_1020_1da8(mut param_1: i16,mut param_2: u16 ,StructB *param_3) -> u16

{
  let mut uVar2: u32;
  StructB *struct_b_1;
  let mut uVar3: u16;
  let mut uVar1: u32;

  uVar3 = (param_3 >> 0x10);
  struct_b_1 = (StructB *)param_3;
  uVar1 = &struct_b_1[0x7].field1_0x2;
  if ((uVar1 + 0x30) == 1) {
    return 0x1;
  }
  uVar2 = &struct_b_1[0x7].field1_0x2;
  if (((uVar2 + 0x30) < 0x3) &&
     (pass1_1010_4df0(param_2,&struct_b_1[0x7].field1_0x2), param_1 == 0)) {
    return 0x1;
  }
  return 0x0;
}



pub fn destroy_win_1020_1dea(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ) -> BOOL16

{
  let mut BVar1: bool;
  WVar2: u16;

  BVar1 = IsWindow16(param_3);
  if (BVar1 != 0) {
    WVar2 = GetWindowWord16(-0xc,param_3);
    if (WVar2 == 0x175) {
      DestroyWindow16(param_3);
      return 0x0;
    }
  }
  return 0x1;
}



pub unsafe fn destroy_win_1020_1e1e(mut param_1: u16 ,mut param_2: u16 ,param_3: HWND16) -> u16

{
  let mut BVar1: bool;
  WVar2: u16;

  BVar1 = IsWindow16(param_3);
  if (BVar1 != 0) {
    WVar2 = GetWindowWord16(-0xc,param_3);
    if ((WVar2 != 1) && (WVar2 != 0x175)) {
      DestroyWindow16(param_3);
    }
  }
  return 0x1;
}



pub fn pass1_1020_1e54(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  ui_cleanup_op_1040_782c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_1eea(param_1: *mut u8,param_2: *mut astruct_663,StructB *param_3,param_4: HWND16)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  iVar3: *mut astruct_663;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffec: u32;
  u8 **ppuVar5;

  uVar3 = (param_2 >> 0x10);
  iVar3 = param_2;
  param_2 = 0x389a;
  iVar3.field2_0x2 = 0x1008;
  param_2 = 0x3aa8;
  iVar3.field2_0x2 = 0x1008;
  iVar3.field3_0x4 = param_4;
  param_2 = 0x3ab0;
  iVar3.field2_0x2 = 0x1008;
  iVar3.field4_0x6 = NULL;
  iVar3.field5_0xa = param_3;
  param_2 = 0x2518;
  iVar3.field2_0x2 = 0x1020;
  ppuVar5 = CONCAT22((in_stack_0000ffec >> 0x10),0x39);
  puVar4 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,ppuVar5,in_stack_0000fe96
                           ,in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
  uVar2 = (puVar4 >> 0x10);
  &iVar3.field4_0x6 = puVar4;
  (&iVar3.field4_0x6 + 0x2) = uVar2;
  ppcVar1 = (*iVar3.field4_0x6 + 0x4);
  (**ppcVar1)(0x1010,&iVar3.field4_0x6,uVar2,0x0,param_2,(ppuVar5 >> 0x10));
  return;
}
pub fn pass1_1020_1f74(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x2518;
  iVar1.address_offset_field_0x2 = 0x1020;
  pass1_1010_1ea6(&iVar1.field_0x6,(param_1 & 0xffff | uVar1 << 0x10));
  param_1.address_offset_field_0x0 = 0x3ab0;
  iVar1.address_offset_field_0x2 = 0x1008;
  param_1.address_offset_field_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}
pub fn invalidate_rect_1020_1fb2(mut param_1: u32,mut param_2: i16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut local_16: u16;
  let mut uStack20: u16;
  let mut iStack18: i16;
  let mut uStack16: u16;
  local_e: i16[0x2];
  let mut iStack10: i16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  iVar1 = param_1;
  uVar2 = (param_1 >> 0x10);
  if (param_2 == 1) {
    (iVar1 + 0x6) = 0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  GetWindowRect16(CONCAT22(0x1050,local_e),(iVar1 + 0x4));
  uStack6 = 0x46;
  uStack20 = 0x46;
  iStack18 = iStack10 - local_e[0];
  uStack4 = 0x5f;
  uStack16 = 0x5f;
  local_16 = (iVar1 + 0x4);
  InvalidateRect16(0x0,&local_16,&DAT_1050_1050);
  return;
}
