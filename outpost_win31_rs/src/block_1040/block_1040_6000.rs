
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1040_6402(param_1: *mut StructD,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut iVar2: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar2: *mut Struct57;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x1850,param_3);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  (iVar2 + 1) = 0;
  iVar2[0x1].field2_0x4 = 0;
  param_2.field0_0x0 = 0x67ba;
  iVar2.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(unaff_BP,0x2b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar3;
  iVar2[0x1].field3_0x6 = (puVar3 >> 0x10);
  ppcVar1 = (*&iVar2[0x1].field2_0x4 + 0x4);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1040_6470(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x67ba;
  iVar1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (&iVar1.field_0x92 != 0) {
    pass1_1010_1ea6(&iVar1.field_0x92,param_1);
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1.field_0x6);
  fn_ptr_1000_17ce(*&iVar1.field_0x8e);
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn msg_box_ui_op_1040_64ca(param_1: *mut c_char,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn show_win_1040_65ba(param_1: *mut StructD,StructB *struct_b_param_1,mut param_3: u16 )

{
  LPVOID pvVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut rect: *mut Struct57;
  let mut uVar4: u16;
  let mut uVar5: *mut StructD;
  let mut paVar5: *mut Struct57;
  StructB *struct_b_4;
  let mut iVar6: i16;
  let mut unaff_SI: u16;
  let mut unaff_DI: i16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut in_stack_0000fe2a: u16;
  let mut in_stack_0000fe2e: u16;
  let mut in_stack_0000fe7e: u16;
  let mut in_stack_0000ff54: u16;
  let mut in_stack_0000ff58: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffac: u16;
  let mut local_22: u16;
  let mut uStack32: u16;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut puStack26: *mut u16;
  let mut iStack10: i16;
  let mut uStack8: u16;
  let mut puStack6: *mut u32;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar5 = param_1;
  puStack6 = mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2b),
                             in_stack_0000fe7e,in_stack_0000ffa2,in_stack_0000ffa8,in_stack_0000ffac);
  uVar5 = (uVar5 & 0xffff0000 | puStack6 >> 0x10);
  uStack8 = pass1_1010_0898();
  if (_PTR_LOOP_1050_5f2c == 0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(uVar5);
  }
  else {
    uVar5 = (uVar5 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
  }
  puStack26 = CONCAT22(uVar5,PTR_LOOP_1050_5f2c);
  uVar3 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,uVar5);
  uVar7 = (struct_b_param_1 >> 0x10);
  struct_b_4 = (StructB *)struct_b_param_1;
  struct_b_4[0x7].field1_0x2 = uVar3;
  struct_b_4[0x7].hwnd_0x6 = uVar5;
  for (iStack10 = 0x1; iStack10 <= uStack8; iStack10 += 1) {
    puStack26 = pass1_1010_0946(puStack6,(puStack6 >> 0x10),iStack10,uVar5,unaff_DI,
                                       &DAT_1050_1050);
    paVar5 = (uVar5 & 0xffff0000 | puStack26 >> 0x10);
    local_22 = *puStack26;
    uStack32 = (puStack26 + 2);
    uStack30 = 0x1;
    uStack28 = 0x1;
    rect = &local_22;
    MapDialogRect16(rect,&DAT_1050_1050);
    mem_op_1000_179c(0x42,paVar5);
    uVar4 = paVar5 | rect;
    uVar5 = (paVar5 & 0xffff0000 | uVar4);
    if (uVar4 == 0) {
      uVar2 = &struct_b_4[0x7].field1_0x2;
      (uVar2 + iStack10 * 0x4) = 0;
    }
    else {
      pvVar1 = struct_b_4.lpvoid_field_0x8;
      pass1_1008_3bd6(uVar5,rect,paVar5,0x0,CONCAT22(local_22,uStack32),0x101,0xff0100,
                      CONCAT13((pvVar1 >> 0x8),CONCAT12(pvVar1,(puStack26 + 0x4)))
                      ,param_3,in_stack_0000fe2a,in_stack_0000fe2e,in_stack_0000ff54,in_stack_0000ff58,in_stack_0000ff5c
                     );
      uVar2 = &struct_b_4[0x7].field1_0x2;
      uVar8 = (uVar2 >> 0x10);
      iVar6 = uVar2;
      *(astruct_57 **)(iVar6 + iStack10 * 0x4) = rect;
      (iVar6 + iStack10 * 0x4 + 0x2) = uVar5;
    }
    uVar2 = &struct_b_4[0x7].field1_0x2;
    uVar8 = (uVar2 >> 0x10);
    iVar6 = uVar2;
    if ((iVar6 + iStack10 * 0x4) != 0) {
      unaff_DI = puStack26;
      enable_win_1040_9234((iVar6 + iStack10 * 0x4),*(BOOL16 *)(unaff_DI + 0x6));
    }
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  ShowWindow16(0x5,struct_b_4.lpvoid_field_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn post_win_msg_1040_672e(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut iVar1: i16;

  if (param_5 == s_vrpal_bmp_1050_183a + 0x7) {
    msg_box_ui_op_1040_64ca(0x0,param_1,CONCAT22(param_3,param_2));
  }
  else {
    if (param_5 == 0x1851) {
      iVar1 = 0x2a;
    }
    else {
      if (param_5 != 0x1852) {
        post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
        return;
      }
      iVar1 = 0x29;
    }
    pass1_1038_af40(param_2,param_1,_PTR_LOOP_1050_5b7c,(param_2 + 0x8),iVar1);
    PostMessage16(0x0,0x2,0x111,(param_2 + 0x6));
  }
  return;
}



pub unsafe fn pass1_1040_6794(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1040_6470(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub unsafe fn pass1_1040_6826(param_1: *mut astruct_57,mut param_2: u16 )

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;

  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_2,0xfda));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field3_0x6 = 0;
  iVar1[0x1].field5_0xa = 0;
  param_1.field0_0x0 = 0x6f32;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}
pub unsafe fn pass1_1040_6862(param_1: *mut StructD)

{
  let mut in_stack_0000ffde: u16;

  param_1.address_offset_field_0x0 = 0x6f32;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar1
pub unsafe fn enable_win_1040_6880(param_1: *mut astruct_925,mut param_2: i16)

{
  let mut uVar2: u32;
  let mut HVar3: HWND16;
  let mut iVar3: *mut astruct_925;
  let mut uVar4: u16;
  let mut uVar1: u32;

  if (param_2 == 0x8) {
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    HVar3 = GetDlgItem16(0x107,iVar3.field6_0x6);
    uVar1 = iVar3.field147_0x94;
    EnableWindow16(*(BOOL16 *)(uVar1 + 0x24),HVar3);
    HVar3 = GetDlgItem16(0x108,iVar3.field6_0x6);
    uVar2 = iVar3.field147_0x94;
    EnableWindow16(*(BOOL16 *)(uVar2 + 0x26),HVar3);
  }
  return;
}



pub unsafe fn pass1_1040_68d2(param_1: u32,param_2: *mut i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;

  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566(CONCAT22(param_3,param_2));
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (*param_1 + 0x80);
    (**ppcVar1)();
  }
  return 0x1;
}
pub unsafe fn pass1_1040_692e(param_1: u32)

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = (*param_1 + 0x7c);
  (**ppcVar1)();
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar18
// WARNING: Unable to use type for symbol uVar19
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn mixed_win_ui_op_1040_6942
               (param_1: *mut astruct_57,mut param_2: u16 ,StructB *struct_b_param_1,mut param_4: u16 ,mut param_5: u16 )

{
  LPVOID pvVar1;
  let mut ppcVar2: *mut *mut code;
  let mut paVar3: *mut Struct57;
  let mut hwnd: *mut u32;
  let mut iVar3: *mut astruct_790;
  let mut uVar4: u16;
  let mut uVar10: u16;
  let mut uVar5: u16;
  let mut paVar13: *mut Struct57;
  StructB *struct_b_6;
  let mut uVar6: u16;
  let mut uVar9: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut puVar15: *mut u32;
  let mut puVar14: *mut u16;
  let mut DVar16: u32;
  let mut in_stack_0000fdd4: u16;
  let mut in_stack_0000fdd6: u16;
  let mut in_stack_0000fdd8: u16;
  let mut in_stack_0000fdda: u16;
  let mut in_stack_0000fe32: u16;
  let mut in_stack_0000fefe: u16;
  let mut in_stack_0000ff00: u16;
  let mut in_stack_0000ff02: u16;
  let mut in_stack_0000ff04: u16;
  let mut in_stack_0000ff06: u16;
  let mut in_stack_0000ff08: u16;
  let mut in_stack_0000ff56: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ff60: u16;
  let mut uVar17: u8;
  let mut uVar20: u8;
  let mut BVar21: bool;
  let mut uVar22: u16;
  let mut pcVar23: *mut c_char;
  let mut hdc: HDC16;
  let mut local_64: u32;
  let mut uStack96: u32;
  let mut HStack92: HWND16;
//   HMENlet mut HStack90: u16;
let mut HStack90: HMENU16;
let mut local_58: [u8;0x50] = [0;0x50];
  let mut hdc_8: HDC16;
  let mut paStack6: *mut Struct57;
  let mut uStack4: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar18: u8;
  let mut uVar19: u8;
  let mut in_stack_0000ff8a: u16;
  let mut paVar11: *mut Struct57;
  let mut paVar12: *mut Struct57;
  let mut uVar14: u32;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  puVar15 = mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(param_2,0x33),in_stack_0000fe32,
                            in_stack_0000ff56,in_stack_0000ff5c,in_stack_0000ff60);
  paVar11 = (param_1 & 0xffff0000 | puVar15 >> 0x10);
  paVar3 = puVar15;
  uVar6 = (struct_b_param_1 >> 0x10);
  struct_b_6 = (StructB *)struct_b_param_1;
  struct_b_6[0x7].max_count_field_0x10 = paVar3;
  struct_b_6[0x7].field5_0xa = (puVar15 >> 0x10);
  ppcVar2 = (*&struct_b_6[0x7].max_count_field_0x10 + 0x4);
  (**ppcVar2)(0x1010,struct_b_6[0x7].max_count_field_0x10,(puVar15 >> 0x10),0x0,struct_b_param_1);
  mem_op_1000_179c(0xa,paVar11);
  uVar10 = paVar11 | paVar3;
  paVar12 = (paVar11 & 0xffff0000 | uVar10);
  if (uVar10 == 0) {
    &struct_b_6[0x7].field6_0xc = 0;
  }
  else {
    puVar14 = struct_1040_bf3e(CONCAT13((paVar11 >> 0x8),CONCAT12(paVar11,paVar3)),
                               struct_b_6.lpvoid_field_0x8);
    paVar12 = (paVar12 & 0xffff0000 | puVar14 >> 0x10);
    paVar3 = puVar14;
    struct_b_6[0x7].field6_0xc = paVar3;
    struct_b_6[0x7].field7_0xe = (puVar14 >> 0x10);
  }
    // WARNING: Load size is inaccurate
  pass1_1040_bfde(struct_b_6[0x7].field6_0xc,&struct_b_6[0x7].max_count_field_0x10);
  mem_op_1000_179c(0x42,paVar12);
  uVar10 = paVar12 | paVar3;
  paVar11 = (paVar12 & 0xffff0000 | uVar10);
  if (uVar10 != 0) {
    pass1_1008_3bd6(paVar11,paVar3,paVar12,0x1,0xa000a,0x0,0x800081,
                    CONCAT22(struct_b_6.lpvoid_field_0x8,0x10a),param_5,in_stack_0000fdd6,in_stack_0000fdda,
                    in_stack_0000ff00,in_stack_0000ff04,in_stack_0000ff08);
  }
  mem_op_1000_179c(0x42,paVar11);
  uVar10 = paVar11 | paVar3;
  paVar12 = (paVar11 & 0xffff0000 | uVar10);
  if (uVar10 != 0) {
    pass1_1008_3bd6(paVar12,paVar3,paVar11,0x1,0xa0028,0x0,0x820083,
                    CONCAT22(struct_b_6.lpvoid_field_0x8,0x10c),param_5,in_stack_0000fdd6,in_stack_0000fdda,
                    in_stack_0000ff00,in_stack_0000ff04,in_stack_0000ff08);
  }
  BVar21 = 0;
  mem_op_1000_179c(0x42,paVar12);
  uVar10 = paVar12 | paVar3;
  paVar11 = (paVar12 & 0xffff0000);
  paVar13 = (paVar11 | uVar10);
  if (uVar10 == 0) {
    paVar3 = NULL;
  }
  else {
    pvVar1 = struct_b_6.lpvoid_field_0x8;
    pass1_1008_3bd6(paVar13,paVar3,paVar12,0x1,0xa00aa,0x101,0xff0100,
                    CONCAT13((pvVar1 >> 0x8),CONCAT12(pvVar1,0x107)),param_4,in_stack_0000fdd4,
                    in_stack_0000fdd8,in_stack_0000fefe,in_stack_0000ff02,in_stack_0000ff06);
    paVar11 = paVar13;
  }
  uStack4 = SUB42(paVar11,0x0);
  paStack6 = paVar3;
  enable_win_1040_9234(CONCAT13((paVar11 >> 0x8),CONCAT12(paVar11,paVar3)),BVar21);
  BVar21 = 0;
  mem_op_1000_179c(0x42,paVar11);
  uVar5 = paVar11 | paVar3;
  uVar14 = paVar11 & 0xffff0000 | uVar5;
  if (uVar5 == 0) {
    paVar3 = NULL;
    uStack4 = 0;
  }
  else {
    pvVar1 = struct_b_6.lpvoid_field_0x8;
    pass1_1008_3bd6(uVar14,paVar3,paVar11,0x1,0xa00c2,0x101,0xff0100,
                    CONCAT13((pvVar1 >> 0x8),CONCAT12(pvVar1,0x108)),param_4,in_stack_0000fdd4,
                    in_stack_0000fdd8,in_stack_0000fefe,in_stack_0000ff02,in_stack_0000ff06);
    uStack4 = uVar14;
  }
  paStack6 = paVar3;
  enable_win_1040_9234(CONCAT13((uStack4 >> 0x8),CONCAT12(uStack4,paVar3)),BVar21);
  hdc_8 = GetDC16(struct_b_6.lpvoid_field_0x8);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x50,local_58,&DAT_1050_1050);
  uVar22 = SUB42(&DAT_1050_1050,0x0);
  uVar17 = SUB21(local_58,0x0);
  uVar20 = (local_58 >> 0x8);
  hdc = hdc_8;
  uVar10 = str_op_1000_3da4(CONCAT22(0x1050,local_58));
  DVar16 = GetTextExtent16(uVar10,(LPCSTR)CONCAT22(uVar22,CONCAT11(uVar20,uVar17)),hdc);
  HStack90 = (HMENU16)(DVar16 >> 0x10);
  HStack92 = DVar16;
  CreateWindow16(0x0,CONCAT22(0x7cd,HINSTANCE16_1050_038c),struct_b_6.lpvoid_field_0x8,HStack90,
                 HStack92,0xad,0x22,0x0,s_Rebel_1050_4ffc + 0x4,CONCAT13(0x10,CONCAT12(0x50,local_58)),
                 s_static_1050_5d84);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x50,local_58,&DAT_1050_1050);
  uVar18 = hdc_8;
  uVar19 = (hdc_8 >> 0x8);
  pcVar23 = local_58;
  uVar22 = SUB42(&DAT_1050_1050,0x0);
  uVar10 = str_op_1000_3da4(CONCAT13(0x10,CONCAT12(0x50,pcVar23)));
  DVar16 = GetTextExtent16(uVar10,(LPCSTR)CONCAT22(uVar22,pcVar23),CONCAT11(uVar19,uVar18));
  HStack90 = (HMENU16)(DVar16 >> 0x10);
  HStack92 = DVar16;
  ReleaseDC16(hdc_8,struct_b_6.lpvoid_field_0x8);
  CreateWindow16(0x0,CONCAT22(0x7ce,HINSTANCE16_1050_038c),struct_b_6.lpvoid_field_0x8,HStack90,
                 HStack92,0xc5,0x22,0x0,s_Rebel_1050_4ffc + 0x4,CONCAT22(0x1050,local_58),
                 s_static_1050_5d8b);
  local_64 = 0x5a000a;
  uStack96 = 0x140050;
  hwnd = &local_64;
  create_window_1040_6eae(struct_b_param_1,0x1,CONCAT22(0x1050,hwnd),0x5eb,0xfd);
  local_64 = local_64 & 0xffff | (local_64 + 0x14) << 0x10;
  create_window_1040_6eae(struct_b_param_1,0x0,CONCAT22(0x1050,&local_64),0x5ec,0xfe);
  local_64 = local_64 & 0xffff | (local_64 + 0x14) << 0x10;
  create_window_1040_6eae(struct_b_param_1,0x0,CONCAT22(0x1050,&local_64),0x5ee,0xff);
  SendMessage16(0x0,0x1,0x401,hwnd);
  uVar1 = &struct_b_6[0x7].max_count_field_0x10;
  iVar3 = uVar1;
  iVar3 = &iVar3.field_0xa;
  uVar9 = ((uVar1 & 0xffff0000) >> 0x10);
  SetWindowPos16(0x40,iVar3.field14_0x10,iVar3.field13_0xe,iVar3.field12_0xc,
                 (uVar1 & 0xffff0000 | ZEXT24(iVar3)),0x0,struct_b_6.lpvoid_field_0x8);
  DAT_1050_0ecc = 0;
  uVar2 = &struct_b_6[0x7].max_count_field_0x10;
  ppcVar2 = (*&struct_b_6[0x7].max_count_field_0x10 + 0x10);
  (**ppcVar2)(s_tile2_bmp_1050_1538,uVar2,(uVar2 >> 0x10));
  pass1_1010_2ee2(&struct_b_6[0x7].max_count_field_0x10);
  PostMessage16(0x0,0x10a,0x111,struct_b_6.lpvoid_field_0x8);
  return;
}
pub unsafe fn pass1_1040_6cac(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  pass1_1010_1ea6((iVar4 + 0x94),(param_1 & 0xffff | uVar5 << 0x10));
  puVar1 = (iVar4 + 0x98);
  uVar2 = (iVar4 + 0x9a);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)(0x1010,puVar1,uVar2,1);
  }
  (iVar4 + 0x98) = 0;
  (iVar4 + 0x94) = 0;
  return;
}



pub unsafe fn pass1_1040_6cfa(mut param_1: u32) -> u16

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = ((param_1 + 0x98) + 0x8);
  (**ppcVar1)();
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_ui_op_1040_6d1a(param_1: *mut astruct_897,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut paVar2: *mut astruct_27;
  let mut in_DX: *mut u8;
  let mut local_a: RECT16;
  let mut iStack6: i16;
  iStack4: *mut astruct_896;
  let mut iVar3: *mut astruct_895;

  match param_4 {
  0xfa =>
    ppcVar1 = (param_1.field144_0x94 + 0x18);
    (**ppcVar1)();
    break;
  _ =>
    pass1_1040_b54a(in_DX,CONCAT13((param_2 >> 0x8),CONCAT12(param_2,param_1)),param_3,
                    param_4);
    return;
  0xfd =>
    if (DAT_1050_0ecc == 0) {
      return;
    }
    DAT_1050_0ecc = 0;
// TODO: goto LAB_1040_6deb;
  0xfe =>
    if (DAT_1050_0ecc == 1) {
      return;
    }
    DAT_1050_0ecc = 0x1;
// TODO: goto LAB_1040_6deb;
  0xff =>
    if (DAT_1050_0ecc == 0x2) {
      return;
    }
    DAT_1050_0ecc = 0x2;//
// LAB_1040_6deb:
    paVar2 = param_1.field144_0x94;
    ppcVar1 = (param_1.field144_0x94 + 0x10);
    (**ppcVar1)(&PTR_LOOP_1050_1040,paVar2,(paVar2 >> 0x10));
    pass1_1010_2ee2(param_1.field144_0x94);
    PostMessage16(0x0,0x10a,0x111,param_1.field6_0x6);
    break;
  0x107 =>
    iVar3 = NULL;
// TODO: goto LAB_1040_6e48;
  0x108 =>
    iVar3 = (&PTR_LOOP_1050_0000 + 1);//
// LAB_1040_6e48:
    win_ui_op_1010_3202(param_1.field144_0x94,iVar3);
    break;
  0x10a =>
    GetClientRect16(&local_a,&DAT_1050_1050);
    paVar2 = param_1.field144_0x94;
    local_a.y += 0x3;
    local_a.x = (paVar2 + 0x1a) + -0x9;
    iStack6 += -0x3;
    iStack4 = iStack4 + -0x3;
    InvalidateRect16(0x1,&local_a,&DAT_1050_1050);
    unk_destroy_win_op_1010_2fa0(param_1.field144_0x94);
    pass1_1010_32c0(param_1.field144_0x94,0x0);
    pass1_1010_2ee2(param_1.field144_0x94);
    break;
  0x10c =>
    DestroyWindow16(param_1.field6_0x6);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn create_window_1040_6eae(mut param_1: u32,mut param_2: i16,pstruct_param_3: *mut astruct_859,mut param_4: u16 ,mut param_5: u16 )

{
  pstruct_1: *mut astruct_859;
  let mut uVar1: u16;
  let mut window_name: *mut c_char;
  let mut h_instance: HISTANCE16;

  window_name = load_string_1010_847e(_u16_1050_14cc,param_4);
  _h_instance = 0x50000009;
  if (param_2 != 0) {
    _h_instance = 0x50020009;
  }
  uVar1 = (pstruct_param_3 >> 0x10);
  pstruct_1 = pstruct_param_3;
  CreateWindow16(0x0,CONCAT22(param_5,HINSTANCE16_1050_038c),*(HINSTANCE16 *)(param_1 + 0x6),
                 pstruct_1.field4_0x6,pstruct_1.field3_0x4,pstruct_1.field2_0x2,pstruct_param_3,
                 _h_instance,(_h_instance >> 0x10),window_name,s_button_1050_5d92);
  return;
}



pub unsafe fn pass1_1040_6f0c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1040_6862(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub unsafe fn pass1_1040_6fb6(param_1: *mut astruct_57,mut param_2: u16 )

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;

  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_2,0xfd9));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field3_0x6 = 0;
  iVar1[0x1].field5_0xa = 0;
  param_1.field0_0x0 = 0x76a4;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}
pub unsafe fn enable_win_1040_6ff2(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut HVar2: HWND16;
  let mut iVar3: *mut astruct_926;
  let mut uVar3: u16;

  if (param_2 == 0x8) {
    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    HVar2 = GetDlgItem16(0x107,iVar3.field6_0x6);
    uVar1 = iVar3.field147_0x94;
    EnableWindow16(*(BOOL16 *)(uVar1 + 0x24),HVar2);
    HVar2 = GetDlgItem16(0x108,iVar3.field6_0x6);
    uVar1 = iVar3.field147_0x94;
    EnableWindow16(*(BOOL16 *)(uVar1 + 0x26),HVar2);
  }
  return;
}
