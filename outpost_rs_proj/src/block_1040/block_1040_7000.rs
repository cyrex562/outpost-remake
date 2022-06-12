


u16 pass1_1040_7044(u32 *param_1,i16 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16)

{
  code **ppcVar1;
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
    ppcVar1 = (code **)((int)*param_1 + 0x80);
    (**ppcVar1)();
  }
  return 0x1;
}
pub fn pass1_1040_70a0(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)*param_1 + 0x7c);
  (**ppcVar1)();
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar14
// WARNING: Unable to use type for symbol uVar15
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn mixed_win_ui_op_1040_70b4
               (param_1: *mut astruct_57,mut param_2: u16 ,StructB *struct_b_param_1,mut param_4: u16 ,mut param_5: u16 )

{
  LPVOID pvVar1;
  astruct_57 *paVar2;
  let mut uVar3: u16;
  let mut count: u16;
  u32 *hwnd;
  astruct_792 *iVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  astruct_57 *paVar5;
  astruct_57 *paVar7;
  StructB *struct_b_5;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  u32 *puVar10;
  let mut puVar11: *mut u16;
  let mut DVar11: u32;
  let mut DVar12: u32;
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
  u8 uVar11;
  u8 uVar12;
  let mut BVar13: bool;
  let mut uVar16: u16;
  char *pcVar17;
  HDC16 hdc;
  let mut local_64: u32;
  let mut uStack96: u32;
  HWND16 HStack92;
  HMENlet mut HStack90: u16;
  char local_58 [0x50];
  HDC16 HStack8;
  astruct_57 *paStack6;
  let mut uStack4: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;
  u8 uVar14;
  u8 uVar15;
  let mut in_stack_0000ff8a: u16;
  astruct_57 *paVar6;
  let mut uVar9: u32;
  code **fn_ptr_1;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  puVar10 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x34),in_stack_0000fe32,
                            in_stack_0000ff56,in_stack_0000ff5c,in_stack_0000ff60);
  paVar5 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)puVar10 >> 0x10);
  paVar2 = (astruct_57 *)puVar10;
  uVar6 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_5 = (StructB *)struct_b_param_1;
  struct_b_5[0x7].max_count_field_0x10 = paVar2;
  struct_b_5[0x7].field5_0xa = (u32 *)((u32)puVar10 >> 0x10);
  fn_ptr_1 = (code **)((int)*(u32*)&struct_b_5[0x7].max_count_field_0x10 + 0x4);
  (**fn_ptr_1)(0x1010,struct_b_5[0x7].max_count_field_0x10,(char)((u32)puVar10 >> 0x10),0x0,struct_b_param_1);
  mem_op_1000_179c(0xa,paVar5);
  uVar4 = paVar5 | paVar2;
  paVar6 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)uVar4);
  if (uVar4 == 0x0) {
    (u32)&struct_b_5[0x7].field6_0xc = 0x0;
  }
  else {
    puVar11 = struct_1040_bf3e((astruct_442 *)CONCAT13((char)((u32)paVar5 >> 0x8),CONCAT12((char)paVar5,paVar2)),
                               struct_b_5->lpvoid_field_0x8);
    paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)puVar11 >> 0x10);
    paVar2 = (astruct_57 *)puVar11;
    struct_b_5[0x7].field6_0xc = paVar2;
    struct_b_5[0x7].field7_0xe = ((u32)puVar11 >> 0x10);
  }
    // WARNING: Load size is inaccurate
  pass1_1040_bfde(struct_b_5[0x7].field6_0xc,(u32*)&struct_b_5[0x7].max_count_field_0x10);
  mem_op_1000_179c(0x42,paVar6);
  uVar4 = (astruct_57 *)paVar6 | paVar2;
  paVar5 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)uVar4);
  if (uVar4 != 0x0) {
    pass1_1008_3bd6((u32)paVar5,paVar2,(astruct_57 *)paVar6,0x1,0xa000a,0x0,0x800081,
                    CONCAT22(struct_b_5->lpvoid_field_0x8,0x10a),param_5,in_stack_0000fdd6,in_stack_0000fdda,
                    in_stack_0000ff00,in_stack_0000ff04,in_stack_0000ff08);
  }
  mem_op_1000_179c(0x42,paVar5);
  uVar4 = (astruct_57 *)paVar5 | paVar2;
  paVar6 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)uVar4);
  if (uVar4 != 0x0) {
    pass1_1008_3bd6((u32)paVar6,paVar2,(astruct_57 *)paVar5,0x1,0xa0028,0x0,0x820083,
                    CONCAT22(struct_b_5->lpvoid_field_0x8,0x10c),param_5,in_stack_0000fdd6,in_stack_0000fdda,
                    in_stack_0000ff00,in_stack_0000ff04,in_stack_0000ff08);
  }
  BVar13 = 0x0;
  mem_op_1000_179c(0x42,paVar6);
  uVar4 = (astruct_57 *)paVar6 | paVar2;
  paVar5 = (astruct_57 *)((u32)paVar6 & 0xffff0000);
  paVar7 = (astruct_57 *)((u32)paVar5 | (u32)uVar4);
  if (uVar4 == 0x0) {
    paVar2 = NULL;
  }
  else {
    pvVar1 = struct_b_5->lpvoid_field_0x8;
    pass1_1008_3bd6((u32)paVar7,paVar2,(astruct_57 *)paVar6,0x1,0xa00aa,0x101,0xff0100,
                    CONCAT13((char)(pvVar1 >> 0x8),CONCAT12((char)pvVar1,0x107)),param_4,in_stack_0000fdd4,
                    in_stack_0000fdd8,in_stack_0000fefe,in_stack_0000ff02,in_stack_0000ff06);
    paVar5 = paVar7;
  }
  uStack4 = SUB42(paVar5,0x0);
  paStack6 = paVar2;
  enable_win_1040_9234(CONCAT13((char)((u32)paVar5 >> 0x8),CONCAT12((char)paVar5,paVar2)),BVar13);
  BVar13 = 0x0;
  mem_op_1000_179c(0x42,paVar5);
  uVar5 = (astruct_57 *)paVar5 | paVar2;
  uVar9 = (u32)paVar5 & 0xffff0000 | (u32)uVar5;
  if (uVar5 == 0x0) {
    paVar2 = NULL;
    uStack4 = 0x0;
  }
  else {
    pvVar1 = struct_b_5->lpvoid_field_0x8;
    pass1_1008_3bd6(uVar9,paVar2,(astruct_57 *)paVar5,0x1,0xa00c2,0x101,0xff0100,
                    CONCAT13((char)(pvVar1 >> 0x8),CONCAT12((char)pvVar1,0x108)),param_4,in_stack_0000fdd4,
                    in_stack_0000fdd8,in_stack_0000fefe,in_stack_0000ff02,in_stack_0000ff06);
    uStack4 = uVar9;
  }
  paStack6 = paVar2;
  enable_win_1040_9234(CONCAT13((char)(uStack4 >> 0x8),CONCAT12((char)uStack4,paVar2)),BVar13);
  HStack8 = GetDC16((HWND16)struct_b_5->lpvoid_field_0x8);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x50,local_58,(short)&DAT_1050_1050);
  uVar16 = SUB42(&DAT_1050_1050,0x0);
  uVar11 = SUB21(local_58,0x0);
  uVar12 = (u8)(local_58 >> 0x8);
  hdc = HStack8;
  uVar3 = str_op_1000_3da4((char *)CONCAT22(0x1050,local_58));
  DVar11 = GetTextExtent16(uVar3,(LPCSTR)CONCAT22(uVar16,CONCAT11(uVar12,uVar11)),hdc);
  HStack90 = (HMENU16)(DVar11 >> 0x10);
  HStack92 = (HWND16)DVar11;
  CreateWindow16(0x0,(void *)CONCAT22(0x7cd,HINSTANCE16_1050_038c),(HINSTANCE16)struct_b_5->lpvoid_field_0x8,HStack90,
                 HStack92,0xad,0x22,0x0,(int)s_Rebel_1050_4ffc + 0x4,(char *)CONCAT13(0x10,CONCAT12(0x50,local_58)),
                 s_static_1050_5d9a);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x50,local_58,(short)&DAT_1050_1050);
  uVar14 = (u8)HStack8;
  uVar15 = (u8)(HStack8 >> 0x8);
  pcVar17 = local_58;
  uVar16 = SUB42(&DAT_1050_1050,0x0);
  count = str_op_1000_3da4((char *)CONCAT13(0x10,CONCAT12(0x50,pcVar17)));
  DVar12 = GetTextExtent16(count,(LPCSTR)CONCAT22(uVar16,pcVar17),CONCAT11(uVar15,uVar14));
  HStack90 = (HMENU16)(DVar12 >> 0x10);
  HStack92 = (HWND16)DVar12;
  ReleaseDC16(HStack8,(HWND16)struct_b_5->lpvoid_field_0x8);
  CreateWindow16(0x0,(void *)CONCAT22(0x7ce,HINSTANCE16_1050_038c),(HINSTANCE16)struct_b_5->lpvoid_field_0x8,HStack90,
                 HStack92,0xc5,0x22,0x0,(int)s_Rebel_1050_4ffc + 0x4,(char *)CONCAT22(0x1050,local_58),
                 s_static_1050_5da1);
  local_64 = 0x5a000a;
  uStack96 = 0x140050;
  hwnd = &local_64;
  create_window_1040_7620(struct_b_param_1,0x1,(astruct_860 *)CONCAT22(0x1050,hwnd),0x5eb,0xfd);
  local_64 = local_64 & 0xffff | (u32)(local_64 + 0x14) << 0x10;
  create_window_1040_7620(struct_b_param_1,0x0,(astruct_860 *)CONCAT22(0x1050,&local_64),0x5ed,0xfe);
  local_64 = local_64 & 0xffff | (u32)(local_64 + 0x14) << 0x10;
  create_window_1040_7620(struct_b_param_1,0x0,(astruct_860 *)CONCAT22(0x1050,&local_64),0x5ef,0xff);
  SendMessage16(0x0,0x1,0x401,(HWND16)hwnd);
  uVar1 = (u32)&struct_b_5[0x7].max_count_field_0x10;
  iVar3 = (astruct_792 *)uVar1;
  iVar3 = (astruct_792 *)&iVar3->field_0xa;
  uVar16 = ((uVar1 & 0xffff0000) >> 0x10);
  SetWindowPos16(0x40,iVar3->field14_0x10,iVar3->field13_0xe,iVar3->field12_0xc,
                 *(INT16 *)(uVar1 & 0xffff0000 | ZEXT24(iVar3)),0x0,(HWND16)struct_b_5->lpvoid_field_0x8);
  DAT_1050_0ecc = 0x0;
  uVar2 = (u32)&struct_b_5[0x7].max_count_field_0x10;
  fn_ptr_1 = (code **)((int)*(u32*)&struct_b_5[0x7].max_count_field_0x10 + 0x10);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(int)uVar2,(int)((u32)uVar2 >> 0x10));
  pass1_1010_2ee2((u32*)&struct_b_5[0x7].max_count_field_0x10);
  PostMessage16(0x0,0x10a,0x111,(HWND16)struct_b_5->lpvoid_field_0x8);
  return;
}
pub fn pass1_1040_741e(mut param_1: u32)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1010_1ea6((u32)(iVar4 + 0x94),(StructD *)(param_1 & 0xffff | (u32)uVar5 << 0x10));
  puVar1 = (u32 *)(iVar4 + 0x98);
  uVar2 = (iVar4 + 0x9a);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1010,puVar1,uVar2,0x1);
  }
  (u32)(iVar4 + 0x98) = 0x0;
  (u32)(iVar4 + 0x94) = 0x0;
  return;
}



u16 pass1_1040_746c(mut param_1: u32)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x98) + 0x8);
  (**ppcVar1)();
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_cleanup_op_1040_748c(u8 *param_1,param_2: *mut astruct_898,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  u32 *puVar1;
  let mut iVar2: i16;
  RECT16 local_a;
  let mut iStack6: i16;
  let mut iStack4: i16;
  code **fn_ptr_1;

  switch(param_5) {
  case 0xfa:
    fn_ptr_1 = (code **)((int)*param_2->field144_0x94 + 0x18);
    (**fn_ptr_1)();
    break;
  default:
    pass1_1040_b54a(param_1,(astruct_903 *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,param_2)),param_4,
                    param_5);
    return;
  case 0xfd:
    if (DAT_1050_0ecc == 0x0) {
      return;
    }
    DAT_1050_0ecc = 0x0;
    goto LAB_1040_755d;
  case 0xfe:
    if (DAT_1050_0ecc == 0x1) {
      return;
    }
    DAT_1050_0ecc = 0x1;
    goto LAB_1040_755d;
  case 0xff:
    if (DAT_1050_0ecc == 0x2) {
      return;
    }
    DAT_1050_0ecc = 0x2;//
LAB_1040_755d:
    puVar1 = param_2->field144_0x94;
    fn_ptr_1 = (code **)((int)*param_2->field144_0x94 + 0x10);
    (**fn_ptr_1)((int)&PTR_LOOP_1050_1040,(int)puVar1,(int)((u32)puVar1 >> 0x10));
    pass1_1010_2ee2(param_2->field144_0x94);
    PostMessage16(0x0,0x10a,0x111,param_2->field6_0x6);
    break;
  case 0x107:
    iVar2 = 0x0;
    goto LAB_1040_75ba;
  case 0x108:
    iVar2 = 0x1;//
LAB_1040_75ba:
    win_ui_op_1010_3202((astruct_27 *)param_2->field144_0x94,iVar2);
    break;
  case 0x10a:
    GetClientRect16(&local_a,(HWND16)&DAT_1050_1050);
    puVar1 = param_2->field144_0x94;
    local_a.y += 0x3;
    local_a.x = ((int)puVar1 + 0x1a) + -0x9;
    iStack6 += -0x3;
    iStack4 += -0x3;
    InvalidateRect16(0x1,&local_a,(HWND16)&DAT_1050_1050);
    unk_destroy_win_op_1010_2fa0((astruct_873 *)param_2->field144_0x94);
    pass1_1010_32c0((u32)param_2->field144_0x94,0x0);
    pass1_1010_2ee2(param_2->field144_0x94);
    break;
  case 0x10c:
    DestroyWindow16(param_2->field6_0x6);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn create_window_1040_7620(mut param_1: u32,mut param_2: i16,astruct_860 *pstruct_param_3,mut param_4: u16 ,mut param_5: u16 )

{
  astruct_860 *iVar1;
  let mut uVar1: u16;
  char *window_name;
  HINSTANCE16 h_instance;

  window_name = load_string_1010_847e(_u16_1050_14cc,param_4);
  _h_instance = 0x50000009;
  if (param_2 != 0x0) {
    _h_instance = 0x50020009;
  }
  uVar1 = ((u32)pstruct_param_3 >> 0x10);
  iVar1 = (astruct_860 *)pstruct_param_3;
  CreateWindow16(0x0,(void *)CONCAT22(param_5,HINSTANCE16_1050_038c),*(HINSTANCE16 *)((int)param_1 + 0x6),
                 iVar1->field4_0x6,iVar1->field3_0x4,iVar1->field2_0x2,*(INT16 *)pstruct_param_3,(INT16)_h_instance,
                 (INT16)((u32)_h_instance >> 0x10),window_name,s_button_1050_5da8);
  return;
}



StructD * pass1_1040_767e(StructD *param_1,param_2: u8)

{
  let mut in_stack_0000ffda: u16;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn get_sys_metrics_1040_7728(param_1: *mut astruct_57,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 )

{
  INT16 IVar1;
  astruct_57 *iVar2;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_57 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar2->field1_0x2 = 0x1008;
  param_1->field0_0x0 = 0x3aa8;
  iVar2->field1_0x2 = 0x1008;
  iVar2->field2_0x4 = 0x0;
  iVar2->field3_0x6 = 0x0;
  iVar2->field4_0x8 = param_5;
  iVar2->field5_0xa = param_4;
  (u32)&iVar2->field6_0xc = 0x0;
  iVar2->field78_0x60 = 0x0;
  iVar2->field79_0x62 = 0x0;
  iVar2->field80_0x64 = 0x0;
  iVar2->field81_0x66 = 0x0;
  iVar2->field82_0x68 = 0x0;
  iVar2->field83_0x6a = param_3;
  iVar2->field84_0x6e = param_2;
  iVar2->field85_0x70 = 0x0;
  iVar2->field86_0x74 = 0x0;
  iVar2->field87_0x76 = 0x0;
  iVar2->field88_0x78 = 0x0;
  iVar2->field105_0x8a = 0x0;
  iVar2->field106_0x8c = 0x0;
  param_1->field0_0x0 = 0x840c;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field8_0x10)),(char *)0x10505db0);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x7a)),NULL,0x8);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x82)),NULL,0x8);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  iVar2->field79_0x62 = IVar1;
  IVar1 = GetSystemMetrics16(SM_CXBORDER);
  iVar2->field80_0x64 = IVar1;
  IVar1 = GetSystemMetrics16(SM_CYBORDER);
  iVar2->field81_0x66 = IVar1;
  return;
}
pub fn ui_cleanup_op_1040_782c(StructD *param_1)

{
  u32 *puVar2;
  let mut uVar3: u16;
  StructD *struct_1;
  let mut uVar2: u16;
  let mut uVar1: u16;
  u32 *puVar1;
  code **fn_ptr_1;

  uVar2 = ((u32)param_1 >> 0x10);
  struct_1._0_2_ = (int)param_1;
  param_1->address_offset_field_0x0 = 0x840c;
  ((int)struct_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  puVar2 = (u32 *)((int)struct_1 + 0x70);
  uVar3 = ((int)struct_1 + 0x72);
  if ((uVar3 | puVar2) != 0x0) {
    fn_ptr_1 = (code **)*puVar2;
    (**fn_ptr_1)();
  }
  if (((int)struct_1 + 0x4) != 0x0) {
    DeleteObject16(*(HGDIOBJ16 *)((int)struct_1 + 0x4));
    ((int)struct_1 + 0x4) = 0x0;
  }
  if (((int)struct_1 + 0x68) != 0x0) {
    DestroyMenu16(*(HMENU16 *)((int)struct_1 + 0x68));
  }
  RemoveProp16(s_thisLo_1050_5db1,*(HWND16 *)((int)struct_1 + 0x6));
  RemoveProp16(s_thisHi_1050_5db8,*(HWND16 *)((int)struct_1 + 0x6));
  RemoveProp16(s_procLo_1050_5dbf,*(HWND16 *)((int)struct_1 + 0x6));
  RemoveProp16(s_procHi_1050_5dc6,*(HWND16 *)((int)struct_1 + 0x6));
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)struct_1 + 0x2) = 0x1008;
  return;
}
pub fn pass1_1040_78de(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn dialog_ui_fn_1040_78e2(StructB *in_struct_1)

{
  u8 *puVar1;
  LPVOID dialog_handle;
  let mut uVar2: u16;
  StructB *struct_b_1;
  StructB *local_string_1;
  let mut uVar3: u16;
  i32 lVar4;
  HANDLE16 local_string_2;
  HANDLE16 HStack8;
pub fn *pvStack6;
  code **fn_ptr_1;

  local_string_1 = (StructB *)((u32)in_struct_1 >> 0x10);
  struct_b_1 = (StructB *)in_struct_1;
  if (*(i32 *)&struct_b_1->field6_0xc == 0x0) {
    uVar3 = ((u32)_u16_1050_5bc8 >> 0x10);
    puVar1 = *(u8 **)((int)_u16_1050_5bc8 + 0x4);
    uVar2 = ((int)_u16_1050_5bc8 + 0x6);
  }
  else {
    puVar1 = struct_b_1->field6_0xc;
    uVar2 = struct_b_1->field7_0xe;
  }
  pvStack6 = (void *)CONCAT22(uVar2,puVar1);
  dialog_handle =
       (LPVOID)CreateDialog16(pvStack6,struct_b_1->max_count_field_0x10,(char *)ZEXT24(struct_b_1->field5_0xa),
                              HINSTANCE16_1050_038c);
  struct_b_1->lpvoid_field_0x8 = dialog_handle;
  GetWindowText16(0x50,(u32)in_struct_1 & 0xffff0000 | ZEXT24(&struct_b_1->field8_0x10),(HWND16)dialog_handle);
  lVar4 = GetWindowLong16(-0x4,(HWND16)struct_b_1->lpvoid_field_0x8);
  SetWindowLong16(_u16_1050_5bcc,-0x4,(HWND16)struct_b_1->lpvoid_field_0x8);
  SetProp16((HANDLE16)struct_b_1,s_thisLo_1050_5dcd,(HWND16)struct_b_1->lpvoid_field_0x8);
  SetProp16((HANDLE16)local_string_1,s_thisHi_1050_5dd4,(HWND16)struct_b_1->lpvoid_field_0x8);
  local_string_2 = (HANDLE16)lVar4;
  SetProp16(local_string_2,s_procLo_1050_5ddb,(HWND16)struct_b_1->lpvoid_field_0x8);
  HStack8 = (HANDLE16)((u32)lVar4 >> 0x10);
  SetProp16(HStack8,s_procHi_1050_5de2,(HWND16)struct_b_1->lpvoid_field_0x8);
  fn_ptr_1 = (code **)((int)(u32)in_struct_1 + 0x50);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,in_struct_1);
  return;
}



u16 pass1_1040_79c0(u32 *param_1,i16 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  code **ppcVar1;
  char cVar2;
  let mut uVar3: u16;

  if (param_5 == 0xa1) {
    ppcVar1 = (code **)((int)*param_1 + 0x38);
    uVar3 = (**ppcVar1)();
    return uVar3;
  }
  if (param_5 < 0xa2) {
    if (param_5 == 0x85) {
      ppcVar1 = (code **)((int)*param_1 + 0x1c);
      (**ppcVar1)();
      return 0x1;
    }
    if (param_5 < 0x86) {
      cVar2 = (char)param_5;
      if (cVar2 == '\x02') {
        ppcVar1 = (code **)((int)*param_1 + 0x24);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\f') {
        ppcVar1 = (code **)((int)*param_1 + 0x18);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\x0f') {
        ppcVar1 = (code **)((int)*param_1 + 0x60);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (cVar2 == '+') {
        if (*param_2 != 0x4) {
          return 0x1;
        }
        win_ui_get_prop_op_1040_9566(CONCAT22(param_3,param_2));
        return 0x1;
      }
    }
  }
  else {
    if (param_5 == 0x114) {
      ppcVar1 = (code **)((int)*param_1 + 0x58);
      uVar3 = (**ppcVar1)();
      return uVar3;
    }
    if (param_5 < 0x115) {
      if (param_5 == 0x104) {
        ppcVar1 = (code **)((int)*param_1 + 0x30);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x111) {
        ppcVar1 = (code **)((int)*param_1 + 0x10);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
    }
    else {
      if (param_5 == 0x115) {
        ppcVar1 = (code **)((int)*param_1 + 0x54);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x201) {
        ppcVar1 = (code **)((int)*param_1 + 0x44);
        (**ppcVar1)();
        return 0x1;
      }
      if (param_5 == 0x204) {
        ppcVar1 = (code **)((int)*param_1 + 0x28);
        (**ppcVar1)();
        return 0x1;
      }
    }
  }
  return 0x0;
}



BOOL16 post_win_msg_1040_7b3c(StructC *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  code **ppcVar1;

  if ((param_4 == 0x1) || (param_4 == 0x2)) {
    ppcVar1 = (code **)((int)(u32)param_1 + 0x14);
    (**ppcVar1)();
  }
  else if (param_4 == 0x6f) {
    ppcVar1 = (code **)((int)(u32)param_1 + 0x2c);
    (**ppcVar1)();
  }
  else {
    if (param_4 != 0x12e) {
      return 0x0;
    }
    PostMessage16(0x0,0xf060,0x112,*(HWND16 *)((int)param_1 + 0x6));
  }
  return 0x1;
}
pub fn destroy_win_1040_7b98(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if (((int)param_1 + 0x74) == 0x0) {
    DestroyWindow16(*(HWND16 *)((int)param_1 + 0x6));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn draw_op_1040_7bb2(astruct_14 *in_struct_1)

{
  let mut is_iconic: bool;
  let mut x: i16;
  let mut y1: i16;
  let mut iVar5: i16;
  HPEN16 pen_handle;
  HGDIOBJ16 obj_handle;
  let mut y: i16;
  HGDIOBJ16 brush_handle;
  HANDLE16 handle;
  let mut count: u16;
  let mut count_00: i16;
  astruct_14 *struct_1;
  RECT16 *pRVar1;
  i32 win_long;
  let mut DVar2: u32;
  RECT16 *rect;
  let mut count_01: i16;
  let mut local_rect_12: i16;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut iStack12: i16;
  HPALETTE16 HStack10;
  let mut uStack8: u32;
  HDC16 win_hdc16_4;
  u8 uVar5;
  u8 uVar6;
  let mut uVar7: u32;
  u8 *pcVar3;
  let mut uVar3: u16;
  code **func_ptr_1;
  HDC16 hdc16_dev_ctx_1;

  pRVar1 = (RECT16 *)((u32)in_struct_1 >> 0x10);
  struct_1 = (astruct_14 *)in_struct_1;
  is_iconic = IsIconic16(struct_1->hwnd16_field6_0x6);
  if (is_iconic == 0x0) {
    win_hdc16_4 = GetWindowDC16(struct_1->hwnd16_field6_0x6);
    func_ptr_1 = (code **)((int)(u32)in_struct_1 + 0x68);
    uStack8 = (**func_ptr_1)((int)s_tile2_bmp_1050_1538,in_struct_1);
    if ((astruct_13 *)uStack8 != NULL) {
      HStack10 = palette_op_1008_4e08
                           ((HPALETTE16)&win_hdc16_4,(uStack8 >> 0x10) | uStack8,(astruct_13 *)uStack8,
                            (HDC16 *)CONCAT22(0x1050,&win_hdc16_4));
      GetWindowRect16((RECT16 *)CONCAT13(0x10,CONCAT12(0x50,&local_rect_12)),struct_1->hwnd16_field6_0x6);
      x = (iStack14 - local_rect_12) + -0x1;
      y1 = (iStack12 - iStack16) + -0x1;
      iVar5 = (-(struct_1->field95_0x60 == 0x0) & 0x1e) + 0x25;
      pen_handle = CreatePen16(CONCAT13(0x1,(u16)iVar5),0x0,0x0);
      obj_handle = SelectObject16(pen_handle,win_hdc16_4);
      MoveTo16(0x0,0x0,win_hdc16_4);
      LineTo16(0x0,x,win_hdc16_4);
      LineTo16(y1,x,win_hdc16_4);
      LineTo16(y1,0x0,win_hdc16_4);
      LineTo16(0x0,0x0,win_hdc16_4);
      win_i32 = GetWindowLong16(-0x10,struct_1->hwnd16_field6_0x6);
      if (((win_i32 & 0x800000U) != 0x0) && ((win_i32 & 0x400000U) != 0x0)) {
        y = struct_1->field96_0x62 - struct_1->field98_0x66;
        MoveTo16(y,0x0,win_hdc16_4);
        LineTo16(y,x,win_hdc16_4);
        struct_1->field115_0x7a = struct_1->field97_0x64;
        struct_1->field116_0x7c = struct_1->field98_0x66;
        struct_1->field117_0x7e = x;
        struct_1->field118_0x80 = struct_1->field96_0x62 - struct_1->field98_0x66;
        rect = pRVar1;
        hdc16_dev_ctx_1 = win_hdc16_4;
        brush_handle = GetStockObject16(BLACK_BRUSH);
        FillRect16(brush_handle,rect,hdc16_dev_ctx_1);
        if (struct_1->field112_0x76 != 0x0) {
          draw_op_1040_82ee(in_struct_1);
        }
        if (struct_1->field15_0x10 != '\0') {
          handle = GetProp16(s_hfont_1050_5de9,struct_1->hwnd16_field6_0x6);
          if (handle != 0x0) {
            SelectObject16(handle,win_hdc16_4);
          }
          SetBkColor16(0x0,win_hdc16_4);
          count_01 = 0x100;
          hdc16_dev_ctx_1 = win_hdc16_4;
          SetTextColor16(CONCAT22(0x100,iVar5),win_hdc16_4);
          count = lstrlen16((char *)CONCAT22(hdc16_dev_ctx_1,count_01));
          DVar2 = GetTextExtent16(count,(LPCSTR)CONCAT22(count_01,win_hdc16_4),win_hdc16_4);
          count_00 = ((struct_1->field117_0x7e - struct_1->field115_0x7a) / 0x2 - (int)DVar2 / 0x2) +
                     struct_1->field115_0x7a;
          brush_handle = (struct_1->field118_0x80 - struct_1->field116_0x7c) / 0x2 - (int)(DVar2 >> 0x10) / 0x2;
          TextOut16(count_01,(char *)CONCAT22(count_01,win_hdc16_4),brush_handle,count_00,win_hdc16_4);
          if (count_00 != 0x0) {
            SelectObject16(brush_handle,win_hdc16_4);
          }
        }
      }
      func_ptr_1 = (code **)((int)(u32)in_struct_1 + 0x64);
      (**func_ptr_1)((int)s_tile2_bmp_1050_1538,in_struct_1,uStack8,win_hdc16_4);
      HStack10 = SelectPalette16(0x0,HStack10,win_hdc16_4);
      DeleteObject16(HStack10);
      SelectObject16(obj_handle,win_hdc16_4);
      DeleteObject16(pen_handle);
    }
    ReleaseDC16(win_hdc16_4,struct_1->hwnd16_field6_0x6);
    return;
  }
  return;
}



pub fn set_text_bk_color_1040_7e5e(u32 *param_1,mut param_2: u16 ,mut param_3: u16 ,HDC16 param_4) -> u32

{
  HGDIOBJ16 HVar1;
  astruct_936 *iVar2;
  INT16 IVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  HDC16 hdc;
  code **fn_ptr_1;

  HVar1 = GetStockObject16(BLACK_BRUSH);
  if (COLORREF_1050_5df0 == 0x0) {
    fn_ptr_1 = (code **)((int)*param_1 + 0x68);
    uVar3 = (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,param_1,((int)param_1 + 0x6e));
    if (uVar3 == 0x0) {
      return 0x0;
    }
    uVar3 = pass1_1008_4d72(uVar3);
    uVar4 = (uVar3 >> 0x10);
    iVar2 = (astruct_936 *)uVar3;
    COLORREF_1050_5df0 = CONCAT22(CONCAT11(0x2,iVar2->field_0x94),CONCAT11(iVar2->field_0x95,iVar2->field_0x96));
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return 0x0;
    }
    uVar4 = 0x7ed5;
    IVar2 = GetDlgCtrlID16(param_2);
    if (IVar2 == 0x14c) {
      uVar5 = 0xffff;
      hdc = 0x0;
      goto LAB_1040_7f00;
    }
    if (IVar2 == 0x175) {
      uVar5 = 0xff;
      hdc = 0x0;
      goto LAB_1040_7f00;
    }
  }
  uVar4 = COLORREF_1050_5df0;
  uVar5 = (COLORREF_1050_5df0 >> 0x10);
  hdc = param_4;//
LAB_1040_7f00:
  SetTextColor16(CONCAT22(uVar5,uVar4),hdc);
  SetBkColor16(0x1000000,param_4);
  return CONCAT22(0x1050,HVar1);
}



BOOL16 post_win_msg_1040_7f1c(mut param_1: u32,mut param_2: i16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((iVar1 + 0x78) != 0x0) {
    return 0x0;
  }
  if ((iVar1 + 0x60) != param_2) {
    (iVar1 + 0x60) = param_2;
    PostMessage16(0x0,0x0,0x85,*(HWND16 *)(iVar1 + 0x6));
  }
  return 0x1;
}
pub fn post_win_msg_1040_7f56(mut param_1: u32,char *param_2)

{
  unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x10)),param_2);
  PostMessage16(0x0,0x0,0x85,*(HWND16 *)((int)param_1 + 0x6));
  return;
}
pub fn menu_ui_op_1040_7f86(param_1: *mut astruct_855)

{
  HMENlet mut HVar1: u16;
  astruct_855 *iVar2;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_855 *)param_1;
  if ((iVar2->field104_0x6a != NULL) && (iVar2->field103_0x68 == 0x0)) {
    HVar1 = LoadMenu16(iVar2->field104_0x6a,HINSTANCE16_1050_038c);
    iVar2->field103_0x68 = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
    HVar1 = GetSubMenu16(0x0,iVar2->field103_0x68);
    iVar2->field103_0x68 = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
  }
  ClientToScreen16((POINT16 *)CONCAT22(0x1050,&stack0xfffa),iVar2->field6_0x6);
  HVar1 = iVar2->field103_0x68;
  TrackPopupMenu16(NULL,iVar2->field6_0x6,0x0,HVar1,0x0,0x0,HVar1);
  return;
}

