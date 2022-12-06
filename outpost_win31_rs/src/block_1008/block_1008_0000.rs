pub fn struct_op_1008_0000(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;


  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x52a;
  (iVar1 + 0x2) = 0x1008;
  (iVar1 + 0x4) = 0;
  (iVar1 + 0x8) = 0;
  *param_1 = 0x51e;
  (iVar1 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_0036(param_1: *mut u16)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut pcVar3: *mut c_char;
  code **ppcVar4;
  let mut puVar5: *mut u32;
  let mut puVar6: *mut u16;
  astruct_449 *iVar6;
  let mut uVar7: u16;
  let mut unaff_CS: u16;

  uVar7 = (param_1 >> 0x10);
  iVar6 = (astruct_449 *)param_1;
  *param_1 = 0x51e;
  iVar6.field2_0x2 = 0x1008;
  pcVar3 = *&iVar6.field_0x8;
  if ((iVar6.field7_0xa | pcVar3) != 0) {
    pass1_1008_53aa();
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcVar3);
  }
  puVar6 = _u16_1050_5748;
  _PTR_LOOP_1050_0298 = 0;
  if (_u16_1050_5748 != NULL) {
    pass1_1030_8210(_u16_1050_5748);
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(puVar6);
  }
  pcVar3 = _u16_1050_0ed0;
  if (_u16_1050_0ed0 != NULL) {
    pass1_1010_2050(_u16_1050_0ed0);
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcVar3);
  }
  puVar5 = _u16_1050_14cc;
  if (_u16_1050_14cc != NULL) {
    pass1_1010_7efc(_u16_1050_14cc);
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(puVar5);
  }
  pcVar3 = _PTR_LOOP_1050_5b7c;
  if (_PTR_LOOP_1050_5b7c != NULL) {
    pass1_1038_af34();
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcVar3);
  }
  if (_u16_1050_5bc8 != NULL) {
    ppcVar4 = (code **)*_u16_1050_5bc8;
    (**ppcVar4)(unaff_CS,_u16_1050_5bc8,(_u16_1050_5bc8 >> 0x10),1);
  }
  if (_u16_1050_02a0 != NULL) {
    ppcVar4 = (code **)*_u16_1050_02a0;
    (**ppcVar4)(unaff_CS,_u16_1050_02a0,(_u16_1050_02a0 >> 0x10),1);
  }
  puVar1 = iVar6.field3_0x4;
  uVar2 = iVar6.field4_0x6;
  if ((uVar2 | puVar1) != 0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)(unaff_CS,puVar1,uVar2,1);
  }
  pass1_1008_9466(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar4
// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn mixed_win_sys_op_1008_016e(param_1: *mut astruct_823)

{
  let mut puVar1: *mut u16;
  let mut uVar6: u16;
  let mut iVar3: i16;
  let mut uVar5: u16;
  let mut uVar9: u16;
  let mut uVar11: u16;
  let mut uVar8: u32;
  let mut DVar10: u16;
  let mut puVar4: *mut u8;
  let mut puVar14: u16;
  let mut uVar13: u16;
  let mut puVar12: *mut u8;
  let mut puVar13: *mut u8;
  let mut uVar7: u16;
  let mut in_EDX: u32;
  astruct_823 *struct_1;
  let mut unaff_DI: i16;
  let mut uVar10: u16;
  let mut uVar12: u16;
  let mut DVar16: u32;
  let mut puVar17: *mut u32;
  StructD *pSVar18;
  let mut in_stack_0000fe46: u16;
  u8 local_13e [0xac];
  u8 local_92 [0x80];
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut puStack14: *mut u32;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u16;
  let mut uStack4: u16;
  let mut uVar1: u32;
  let mut puVar2: *mut u16;
  astruct_20 *uVar4;
  astruct_20 *uVar2;
  astruct_20 *uVar3;
  let mut paVar14: *mut Struct57;
  let mut paVar15: *mut Struct57;
  code **fn_ptr;

  uVar9 = (in_EDX >> 0x10);
  DVar16 = GetVersion16();
  DVar10 = (DVar16 >> 0x10);
  paVar14 = (astruct_57 *)CONCAT22(uVar9,DVar10);
  uStack6 = (DVar16 & 0xffff);
  uVar6 = DVar16 & 0xff;
  uStack10 = ((DVar16 & 0xffff) >> 0x8);
  uStack8 = uVar6;
  if ((uVar6 < 0x3) || ((uVar6 == 0x3 && (uStack10 < 0xa)))) {
    // 0x97
    uVar12 = 0x1000;
    mem_op_1000_179c(0xb4,paVar14);
    uStack16 = paVar14;
    puVar4 = (uStack16 | uVar6);
    paVar14 = (astruct_57 *)(paVar14 & 0xffff0000);
    paVar15 = (astruct_57 *)(paVar14 | ZEXT24(puVar4));
    uStack18 = uVar6;
    if (puVar4 == NULL) {
      iVar3 = 0;
    }
    else {
      uVar12 = &PTR_LOOP_1050_1040;
      iVar3 = string_1040_8520(paVar15,(astruct_57 *)CONCAT22(uStack16,uVar6),0x0,0x20010,0x5dd05de);
      paVar14 = paVar15;
    }
    puStack14 = CONCAT22(paVar14,iVar3);
    fn_ptr = (code **)(*puStack14 + 0x74);
    (**fn_ptr)(uVar12,iVar3,paVar14);
    fn_ptr_op_1000_24cd(1);
  }
  debug_print_1008_6048(paVar14,s_version__d__d_1050_0012);
  if ((uStack8 == 0x3) && (0xb < uStack10)) {
    PTR_LOOP_1050_0010 = (&PTR_LOOP_1050_0000 + 1);
  }
  LoadString16(0x80,CONCAT22(0x1050,local_92),0x578,HINSTANCE16_1050_038c);
  uVar5 = dos3_call_1000_51aa(local_92,&DAT_1050_1050,1);
  if (uVar5 != 0) {
    LoadString16(0x80,CONCAT13(0x10,CONCAT12(0x50,local_13e)),0x57b,HINSTANCE16_1050_038c);
    LoadString16(0x80,CONCAT13(0x10,CONCAT12(0x50,&stack0xfe42)),0x62e,HINSTANCE16_1050_038c);
    uVar5 = MessageBox16(0x10,CONCAT13(0x10,CONCAT12(0x50,local_13e)),CONCAT22(0x1050,&stack0xfe42),0x0)
    ;
    fn_ptr_op_1000_24cd(1);
  }
  mem_op_1000_179c(0x4,paVar14);
  uStack16 = paVar14;
  paVar14 = (astruct_57 *)(paVar14 & 0xffff0000);
  if ((uStack16 | uVar5) == 0) {
    uVar9 = 0;
    uStack18 = uVar5;
  }
  else {
    uStack18 = uVar5;
    puVar17 = pass1_1008_5394(CONCAT22(uStack16,uVar5));
    paVar14 = (astruct_57 *)(paVar14 & 0xffff0000 | puVar17 >> 0x10);
    uVar9 = SUB42(puVar17,0x0);
  }
  uVar10 = (param_1 >> 0x10);
  struct_1 = (astruct_823 *)param_1;
  &struct_1.field5_0x8 = uVar9;
  (&struct_1.field5_0x8 + 0x2) = paVar14;
  uVar8 = struct_1.field5_0x8;
  puVar1 = struct_1.field5_0x8;
  _PTR_LOOP_1050_0298 = uVar8;
  *puVar1 = 0x70;
    // 0x1538
  (puVar1 + 0x2) = s_tile2_bmp_1050_1538;
  mem_op_1000_179c(0x126,paVar14);
  uVar11 = uVar8;
  uStack16 = paVar14;
  paVar15 = (astruct_57 *)(paVar14 & 0xffff0000 | (uStack16 | uVar11));
  uStack18 = uVar11;
  if ((uStack16 | uVar11) != 0) {
    pSVar18 = pass1_1010_2024((uVar8 & 0xffff | paVar14 << 0x10));
    paVar15 = (astruct_57 *)(paVar15 & 0xffff0000 | pSVar18 >> 0x10);
    uVar11 = pSVar18;
  }
  if (_u16_1050_0ed0 == 0) {
    debug_print_1008_6048(paVar15,s_New_failed_in_Op__Op_1050_0020);
    fn_ptr_op_1000_24cd(1);
  }
  mem_op_1000_179c(0xe8c,paVar15);
  uStack16 = paVar15;
  puVar12 = (uStack16 | uVar11);
  paVar14 = (astruct_57 *)(paVar15 & 0xffff0000 | ZEXT24(puVar12));
  uStack18 = uVar11;
  if (puVar12 != NULL) {
    pass1_1010_7e40(puVar12,(astruct_652 *)CONCAT22(uStack16,uVar11));
  }
  if (_u16_1050_14cc == 0) {
    debug_print_1008_6048(paVar14,s_New_failed_in_Op__Op__ResLibr_1050_0035);
    fn_ptr_op_1000_24cd(1);
  }
  mem_op_1000_179c(0xb0,paVar14);
  uStack16 = paVar14;
  paVar14 = (astruct_57 *)(paVar14 & 0xffff0000 | (uStack16 | uVar11));
  uStack18 = uVar11;
  if ((uStack16 | uVar11) != 0) {
    pSVar18 = pass1_1038_aeca(CONCAT22(uStack16,uVar11));
    paVar14 = (astruct_57 *)(paVar14 & 0xffff0000 | pSVar18 >> 0x10);
    uVar11 = pSVar18;
  }
  if (_PTR_LOOP_1050_5b7c == 0) {
    debug_print_1008_6048(paVar14,s_New_failed_in_Op__Op__DialogCtr_1050_0053);
    fn_ptr_op_1000_24cd(1);
  }
  mem_op_1000_179c(0xa,paVar14);
  uStack16 = paVar14;
  puVar13 = (uStack16 | uVar11);
  paVar14 = (astruct_57 *)(paVar14 & 0xffff0000 | ZEXT24(puVar13));
  uStack18 = uVar11;
  if (puVar13 != NULL) {
    make_proc_inst_1038_cf6c(puVar13,(astruct_831 *)CONCAT22(uStack16,uVar11));
  }
  if (_u16_1050_5bc8 == 0) {
    debug_print_1008_6048(paVar14,s_New_failed_in_Op__Op__DialogHand_1050_0073);
    fn_ptr_op_1000_24cd(1);
  }
  mem_op_1000_179c(0x14,paVar14);
  uStack16 = paVar14;
  paVar14 = (astruct_57 *)(paVar14 & 0xffff0000 | (uStack16 | uVar11));
  uStack18 = uVar11;
  if ((uStack16 | uVar11) != 0) {
    pass1_1008_5bdc(CONCAT22(uStack16,uVar11));
  }
  if (_u16_1050_02a0 == 0) {
    debug_print_1008_6048(paVar14,s_New_failed_in_Op__Op__Simulator_1050_0097);
    fn_ptr_op_1000_24cd(1);
  }
  mem_op_1000_179c(0xfc,paVar14);
  uStack16 = paVar14;
  uVar7 = uStack16 | uVar11;
  uStack18 = uVar11;
  if (uVar7 == 0) {
    uVar11 = 0;
    uVar7 = 0;
  }
  else {
    set_struct_op_1008_0536((astruct_20 *)CONCAT22(uStack16,uVar11),in_stack_0000fe46);
  }
  &struct_1.field4_0x4 = uVar11;
  (&struct_1.field4_0x4 + 0x2) = uVar7;
  if (struct_1.field4_0x4 == NULL) {
    debug_print_1008_6048(uVar7,s_New_failed_in_Op__Op_1050_00b7);
    fn_ptr_op_1000_24cd(1);
  }
  win_ui_reg_class_1008_96d2((StructA *)struct_1.field4_0x4);
  fn_ptr = (code **)(struct_1.field4_0x4 + 0x8);
  (**fn_ptr)(0x1000);
  uVar4 = struct_1.field4_0x4;
  HWND16_1050_0396 = *(HWND16 *)(uVar4 + 0x8);
  uVar2 = struct_1.field4_0x4;
  fn_ptr = (code **)(struct_1.field4_0x4 + 0xc);
  (**fn_ptr)(0x1000,uVar2,(uVar2 >> 0x10),0x3);
  uVar3 = struct_1.field4_0x4;
  UpdateWindow16(*(HWND16 *)(uVar3 + 0x8));
  return;
}
pub fn pass1_1008_049c(mut param_1: u16 ,mut param_2: u16 ,char *param_3)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u8;

  if (param_3 != NULL) {
    uVar1 = str_op_1000_3da4(param_3);
    if (uVar1 != 0) {
      puVar2 =
               pass1_1000_545a(param_3 & 0xffff0000 | (param_3 + 1),s_nomono2_1050_00cc);
      if (puVar2 == NULL) {
        PTR_LOOP_1050_02ec = puVar2;
      }
    }
  }
  return;
}



StructD * pass1_1008_04d2(StructD *param_1,param_2: u8)

{
  pass1_1008_9466(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1008_04f8(param_1: *mut u16,param_2: u8)

{
  pass1_1008_0036(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack
pub fn set_struct_op_1008_0536(param_1: *mut astruct_20,mut param_2: u16 )

{
  HICON16 hicon_1;
  HCURSOR16 hcursor_1;
  HGDIOBJ16 hbrush_1;
  let mut in_EDX: u32;
  let mut uVar2: u16;
  let mut paVar1: *mut Struct57;
  astruct_20 *paVar3;
  let mut puVar4: *mut u32;
  let mut in_stack_0000feac: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd6: u16;
  let mut in_stack_0000ffda: u16;

  uVar2 = (in_EDX >> 0x10);
  paVar3 = pass1_1008_3ab8(param_1);
  paVar1 = (astruct_57 *)CONCAT22(uVar2,(paVar3 >> 0x10));
  (param_1 + 0xe0) = 0;
  (param_1 + 0xe4) = 0;
  (param_1 + 0xe8) = 0;
  (param_1 + 0xec) = 0;
  (param_1 + 0xee) = 0;
  (param_1 + 0xf2) = 0;
  (param_1 + 0xf4) = 0;
  (param_1 + 0xf8) = 0;
  param_1.offset_0x0 = 0x389e;
  (param_1 + 0x2) = 0x1008;
  (param_1 + 0xc8) = 0x2008;
  (param_1 + 0xac) = 0;
  (param_1 + 0xae) = 0x8700;
  hicon_1 = LoadIcon16(s_Op_1050_00d4,HINSTANCE16_1050_038c);
  *(HICON16 *)(param_1 + 0xc2) = hicon_1;
  hcursor_1 = LoadCursor16(0x7f00,0x0);
  *(HCURSOR16 *)(param_1 + 0xc4) = hcursor_1;
  hbrush_1 = GetStockObject16(BLACK_BRUSH);
  *(HGDIOBJ16 *)(param_1 + 0xc6) = hbrush_1;
  puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0 & 0xffff,(u8 **)CONCAT22(param_1,0x48),in_stack_0000feac,
                           in_stack_0000ffd0,in_stack_0000ffd6,in_stack_0000ffda);
  paVar1 = (astruct_57 *)(paVar1 & 0xffff0000 | puVar4 >> 0x10);
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0xa)),s_Outpost_1050_00d7);
  puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0 & 0xffff,(u8 **)CONCAT22(param_1,0x32),in_stack_0000feac,
                           in_stack_0000ffd0,in_stack_0000ffd6,in_stack_0000ffda);
  (param_1 + 0xf4) = puVar4;
  (param_1 + 0xf6) = (puVar4 >> 0x10);
  set_sys_color_1008_357e((astruct_53 *)param_1,0x1,paVar1 & 0xffff0000 | puVar4 >> 0x10);
  return;
}
pub fn cleanup_ui_op_1008_0618(param_1: *mut astruct_53)

{
  let mut uVar1: u16;
  code **ppcVar2;
  let mut in_EDX: u32;
  astruct_53 *iVar4;
  let mut uVar3: u16;
  let mut puVar1: *mut u32;

  uVar3 = (param_1 >> 0x10);
  iVar4 = (astruct_53 *)param_1;
  param_1 = 0x389e;
  &iVar4.field_0x2 = 0x1008;
  set_sys_color_1008_357e(param_1,0x0,in_EDX);
  fn_ptr_1000_17ce(*&iVar4.field248_0xf8);
  if (&iVar4.field_0xec != 0) {
    DestroyMenu16(*(HMENU16 *)&iVar4.field_0xec);
  }
  DestroyIcon16(*(HICON16 *)&iVar4.field_0xc2);
  &iVar4.field_0xc2 = 0;
  puVar1 = &iVar4.field_0xe0;
  uVar1 = &iVar4.field_0xe2;
  if ((uVar1 | puVar1) != 0) {
    ppcVar2 = (code **)*puVar1;
    (**ppcVar2)(s_tile2_bmp_1050_1538,puVar1,uVar1,1);
  }
  pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0xd2)));
  param_1 = 0x380a;
  &iVar4.field_0x2 = 0x1008;
  param_1 = 0x389a;
  &iVar4.field_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_cursor_op_1008_06c0(u32 *param_1,mut param_2: u32,mut param_3: u16 ,mut param_4: i16)

{
  code **ppcVar1;
  let mut in_AX: u16;
  let mut in_EDX: u32;
  let mut paVar2: *mut Struct57;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  u8 in_AF;
  let mut pcVar6: *mut c_char;
  let mut puVar7: *mut u32;
  let mut in_stack_0000fe3a: u16;
  let mut in_stack_0000fe44: u16;
  let mut in_stack_0000ff5e: u16;
  let mut in_stack_0000ff64: u16;
  let mut in_stack_0000ff68: u16;
  let mut in_stack_0000ff6e: u16;
  let mut in_stack_0000ff72: u16;
  let mut iVar8: i16;
  let mut in_stack_0000ff9c: u16;
  u8 local_5a [0x50];
  let mut uStack10: u16;
  let mut uStack8: u16;
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;

  if (param_4 == 0x400) {
    pass1_1030_8344(_u16_1050_5748,0x4000001);
    in_AX = in_EDX | in_AX;
    paVar2 = (astruct_57 *)(in_EDX & 0xffff0000 | in_AX);
    if (in_AX != 0) {
      iVar4 = param_1;
      uVar5 = (param_1 >> 0x10);
      if (PTR_LOOP_1050_4fe8 != NULL) {
        pcVar6 = load_string_1010_847e(_u16_1050_14cc,0x57b);
        MessageBox16(0x10,pcVar6,s_You_may_not_run_a_turn__The_game_1050_00df,*(HWND16 *)(iVar4 + 0x8));
        return;
      }
      HStack4 = LoadCursor16(0x7f02,0x0);
      HStack6 = SetCursor16(HStack4);
      pass1_1030_83ba(in_AF,_u16_1050_5748,param_2);
      (_u16_1050_5748 + 0x8) = 0x1;
      puVar7 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ff9c,0x29),in_stack_0000fe44,
                               in_stack_0000ff68,in_stack_0000ff6e,in_stack_0000ff72);
      uVar3 = (paVar2 >> 0x10);
      uStack10 = SUB42(puVar7,0x0);
      uStack8 = (puVar7 >> 0x10);
      pass1_1018_262e(puVar7);
      pass1_1030_8326();
      pcVar6 = load_string_1010_847e(_u16_1050_14cc,0x5dc);
      paVar2 = (astruct_57 *)CONCAT22(uVar3,(pcVar6 >> 0x10));
      sys_1000_3f9c(CONCAT13(0x10,CONCAT12(0x50,local_5a)),s__s__ld_1050_0109,pcVar6);
      ppcVar1 = (code **)(*param_1 + 0x14);
      iVar8 = iVar4;
      (**ppcVar1)(0x1000,iVar4,(param_1 >> 0x10),0x0,local_5a,&DAT_1050_1050);
      puVar7 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(iVar8,0x37),in_stack_0000fe3a,
                               in_stack_0000ff5e,in_stack_0000ff64,in_stack_0000ff68);
      pass1_1008_a9ec(puVar7);
      SetCursor16(HStack6);
      PostMessage16(0x0,0xfc,0x111,*(HWND16 *)(iVar4 + 0x8));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1008_07d8(mut param_1: u16 ,param_2: *mut astruct_57,param_3: *mut astruct_72)

{
  let mut uVar2: u16;
  let mut uVar1: u16;
  let mut uVar4: u32;
  let mut paVar3: *mut Struct57;

  if (_u16_1050_5748 == NULL) {
    mem_op_1000_179c(0xa,param_2);
    uVar2 = param_2 | param_1;
    paVar3 = (astruct_57 *)(param_2 & 0xffff0000 | uVar2);
    if (uVar2 != 0) {
      struct_1030_8128(paVar3,(astruct_135 *)CONCAT22(param_2,param_1));
    }
    if (_u16_1050_5748 == NULL) {
      debug_print_1008_6048(paVar3,s_New_failed_in_Op__Op__Simulator_1050_0110);
      fn_ptr_op_1000_24cd(1);
    }
    uVar4 = pass1_1028_e2e0(paVar3,_PTR_LOOP_1050_65e2,0x8);
    paVar3 = (astruct_57 *)(paVar3 & 0xffff0000 | uVar4 >> 0x10);
    uVar4 = pass1_1028_e2e0(paVar3,_PTR_LOOP_1050_65e2,0x8);
    pass1_1028_e2e0((astruct_57 *)(paVar3 & 0xffff0000 | uVar4 >> 0x10),_PTR_LOOP_1050_65e2,0xff);
    pass1_1030_838e(_u16_1050_5748);
    param_1 = pass1_1030_8334();
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_087e(param_1: u8,mut param_2: u16 ,StructD *param_3)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u32;
  let mut local_112: u16;
  let mut uStack272: u16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_3);
  mem_op_1000_179c(0xa,paVar1);
  uStack4 = paVar1;
  paVar1 = (astruct_57 *)(paVar1 & 0xffff0000 | (uStack4 | param_2));
  uStack6 = param_2;
  if ((uStack4 | param_2) != 0) {
    struct_1030_8128(paVar1,(astruct_135 *)CONCAT22(uStack4,param_2));
  }
  if (_u16_1050_5748 == NULL) {
    debug_print_1008_6048(paVar1,s_New_failed_in_Op__Op__Simulator_1050_0130);
    fn_ptr_op_1000_24cd(1);
  }
  uVar2 = pass1_1028_e2e0(paVar1,_PTR_LOOP_1050_65e2,0x8);
  pass1_1028_e2e0((astruct_57 *)(paVar1 & 0xffff0000 | uVar2 >> 0x10),_PTR_LOOP_1050_65e2,0x8);
  pass1_1030_532e((astruct_97 *)CONCAT22(0x1050,&local_112),0xff000000);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_112));
  pass1_1030_838e(_u16_1050_5748);
  local_112 = 0x389a;
  uStack272 = 0x1008;
  pass1_1030_8334();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_0932() -> u32

{
  let mut uVar1: u32;

  if (_u16_1050_14cc != NULL) {
    pass1_1010_7fd6(_u16_1050_14cc);
  }
  mem_1000_0016(_PTR_LOOP_1050_03a0);
  mem_1000_0016(_PTR_LOOP_1050_029c);
  mem_1000_0016(_PTR_LOOP_1050_4fb8);
  mem_1000_0016(_PTR_LOOP_1050_68a2);
  mem_1000_0016(_PTR_LOOP_1050_5744);
  uVar1 = mem_1000_0016(_PTR_LOOP_1050_5f2c);
  return uVar1;
}
pub fn pass1_1008_0984(mut param_1: i16,mut param_2: u16 ,mut param_3: i16)

{
  code **ppcVar1;
  let mut in_EDX: u32;

  set_sys_color_1008_357e((astruct_53 *)CONCAT22(param_2,param_1),param_3,in_EDX);
  if (*(i32 *)(param_1 + 0xe8) != 0) {
    ppcVar1 = (code **)((param_1 + 0xe8) + 0x98);
    (**ppcVar1)();
  }
  return;
}
pub fn menu_ui_op_1008_09ba(param_1: *mut astruct_853,param_2: HWND16,RECT16 *param_3)

{
  HMENlet mut HVar1: u16;
  astruct_853 *iVar2;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_853 *)param_1;
  if (iVar2.field235_0xec == 0) {
    HVar1 = LoadMenu16(s_OPPOPMENU_1050_0150,HINSTANCE16_1050_038c);
    iVar2.field235_0xec = HVar1;
    if (HVar1 == 0) {
      return;
    }
    HVar1 = GetSubMenu16(0x0,iVar2.field235_0xec);
    iVar2.field235_0xec = HVar1;
    if (HVar1 == 0) {
      return;
    }
  }
  ClientToScreen16((POINT16 *)CONCAT22(0x1050,&stack0xfffa),iVar2.field8_0x8);
  HVar1 = iVar2.field235_0xec;
  TrackPopupMenu16(NULL,HWND16_1050_0396,0x0,HVar1,0x0,0x0,HVar1);
  return;
}



u16 unk_win_msg_op_1008_0a3c(mut param_1: u32,mut param_2: u16 )

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  let mut uVar3: u16;

  iVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  if ((param_2 & 0xfff0) == 0xf140) {
    return (iVar2 + 0xde);
  }
  if ((param_2 & 0xfff0) == 0xf060) {
    BVar1 = IsIconic16(*(HWND16 *)(iVar2 + 0x8));
    if (BVar1 == 0) {
      PostMessage16(0x0,0x67,0x111,*(HWND16 *)(iVar2 + 0x8));
    }
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_0a92(mut param_1: u32)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(i32 *)(iVar2 + 0xee) != 0) {
    ppcVar1 = (code **)((iVar2 + 0xee) + 0x90);
    (**ppcVar1)();
  }
  if (*(i32 *)(iVar2 + 0xe8) != 0) {
    ppcVar1 = (code **)((iVar2 + 0xe8) + 0x90);
    (**ppcVar1)();
  }
  if (_PTR_LOOP_1050_0388 != NULL) {
    ppcVar1 = (code **)*_PTR_LOOP_1050_0388;
    (**ppcVar1)();
  }
  post_quit_msg_1008_3af4();
  return;
}
pub fn window_op_1008_0af8(mut param_1: u16 ,StructA *struct_param_1)

{
  let pSVar1: *mut StructA;
  let mut HVar2: HWND16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut puVar5: *mut u8;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let iVar8: *mut StructA;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut puVar10: *mut u16;
  let mut uVar11: u16;
  u8 uVar12;
  let mut uVar13: u16;
  astruct_20 *struct_20_v6;
  let mut paVar7: *mut Struct57;
  code **fn_ptr_1;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  create_window_ex_1008_9760(struct_param_1);
  uVar8 = (struct_param_1 >> 0x10);
  iVar8 = (StructA *)struct_param_1;
  HVar2 = iVar8.field4_0x8;
  HWND16_1050_0396 = HVar2;
  mem_op_1000_179c(0x12,paVar6);
  uVar4 = paVar6 | HVar2;
  paVar7 = (astruct_57 *)(paVar6 & 0xffff0000 | uVar4);
  if (uVar4 != 0) {
    puVar10 = pass1_1008_91ba((astruct_3 *)CONCAT22(paVar6,HVar2));
    paVar7 = (astruct_57 *)(paVar7 & 0xffff0000 | puVar10 >> 0x10);
    HVar2 = (HWND16)puVar10;
  }
  mem_op_1000_179c(0x6,paVar7);
  uVar4 = paVar7 | HVar2;
  paVar6 = (astruct_57 *)(paVar7 & 0xffff0000 | uVar4);
  if (uVar4 == 0) {
    &iVar8[0x1].field10_0x14 = 0;
  }
  else {
    pass1_1008_392e(CONCAT22(paVar7,HVar2),iVar8.field4_0x8);
    iVar8[0x1].field10_0x14 = HVar2;
    iVar8[0x1].field11_0x16 = paVar6;
  }
  fn_ptr_1 = (code **)(struct_param_1 + 0x14);
  (**fn_ptr_1)(0x1000,struct_param_1,0x0,0x15a,&DAT_1050_1050);
  uVar9 = 0x1000;
  mem_op_1000_179c(0xec,paVar6);
  struct_20_v6 = (astruct_20 *)CONCAT22(paVar6,HVar2);
  uVar4 = paVar6 | HVar2;
  if (uVar4 == 0) {
    &iVar8[0x1].field12_0x18 = 0;
  }
  else {
    pSVar1 = iVar8 + 1;
    pSVar1.field0_0x0 = pSVar1.field0_0x0 + 1;
    uVar9 = 0x1020;
    pass1_1020_08b6(struct_20_v6,(iVar8 + 1)->field0_0x0,struct_param_1);
    iVar8[0x1].field12_0x18 = HVar2;
    iVar8[0x1].field13_0x1a = uVar4;
  }
  if (*(i32 *)&iVar8[0x1].field1_0x2 != 0) {
    fn_ptr_1 = (code **)(*&iVar8[0x1].field1_0x2 + 0x10);
    (**fn_ptr_1)();
  }
  &iVar8[0x1].field1_0x2 = &iVar8[0x1].field12_0x18;
  uVar13 = 0x1;
  uVar3 = &iVar8[0x1].field12_0x18;
  uVar11 = uVar3;
  uVar12 = (uVar3 >> 0x10);
  fn_ptr_1 = (code **)(*&iVar8[0x1].field12_0x18 + 0x10);
  (**fn_ptr_1)();
  uVar3 = &iVar8[0x1].field12_0x18;
  puVar5 = iVar8[0x1].field13_0x1a;
  &iVar8[0x1].field14_0x1c = uVar3;
  fn_ptr_1 = (code **)(*&iVar8[0x1].field14_0x1c + 0x8);
  (**fn_ptr_1)(uVar9,iVar8[0x1].field14_0x1c,puVar5,uVar11,uVar12,uVar13);
  uVar4 = uVar3;
  fn_ptr_1 = (code **)(*&iVar8[0x1].field14_0x1c + 0xc);
  (**fn_ptr_1)();
  pass1_1008_6978(uVar4,puVar5,struct_param_1 & 0xffff | uVar8 << 0x10,0x0,
                  &iVar8[0x1].field14_0x1c);
  return;
}



// WARNING: Unable to use type for symbol puVar1
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 mixed_win_op_1008_0c60
                 (mut param_1: u16 ,StructD *param_2,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut astruct_72,mut param_6: u16 ,
                 mut param_7: u16 ,mut param_8: u16 )

{
  code **ppcVar1;
  HINSTANCE16 HVar2;
  let mut iVar3: i16;
  let mut BVar4: bool;
  let mut uVar7: u16;
  StructD *pSVar8;
  let mut uVar15: u16;
  astruct_72 *struct_var5;
  let mut uVar6: u16;
  uchar in_AF;
  let mut uVar5: u32;
  let mut lresult_6: LRESULT;
  let mut pcVar16: *mut c_char;
  let mut puVar17: *mut u32;
  let mut in_stack_0000fcd2: u16;
  let mut in_stack_0000fce4: u16;
  let mut in_stack_0000fcf8: u16;
  WNDCLASS16 *in_stack_0000fd58;
  let mut in_stack_0000fe34: u16;
  let mut in_stack_0000fe3a: u16;
  let mut in_stack_0000ff58: u16;
  let mut in_stack_0000ff5e: u16;
  let mut in_stack_0000ff62: u16;
  let mut in_stack_0000ff64: u16;
  let mut in_stack_0000ff68: u16;
  let mut WVar18: WPARAM16;
  let mut uVar19: u16;
  u8 local_64 [0x50];
  let mut uStack20: u32;
  HCURSOR16 HStack16;
  HCURSOR16 HStack14;
  let mut uStack6: u32;
  let mut puVar1: *mut u32;
  u8 uVar9;
  u8 uVar10;
  let mut iVar12: i16;
  let mut uVar13: u16;
  astruct_72 *struct_var15;
  let mut uVar14: u16;
  let mut in_stack_0000ff92: u16;
  u8 uVar11;
  let mut uVar12: u16;
  let mut paVar9: *mut Struct57;

  struct_var5 = (astruct_72 *)param_5;
  uVar6 = (param_5 >> 0x10);
  pSVar8 = param_2;
  switch(param_6) {
  case 0x64:
    BVar4 = pass1_1008_07d8(param_1,(astruct_57 *)param_2,struct_var5);
    win_ui_cursor_op_1008_2e9a(param_2,param_5,in_stack_0000fd58,in_stack_0000fcd2,in_stack_0000fce4,in_stack_0000fcf8);
    return BVar4;
  case 0x65:
    pass1_1008_3018(pSVar8,param_5);
    return param_1;
  case 0x66:
    pass1_1008_30cc(param_1,pSVar8,param_5);
    return param_1;
  case 0x67:
    iVar3 = win_ui_op_1008_2b54(param_1,pSVar8,param_5);
    if (iVar3 == 0) {
      return 0x0;
    }
  case 0xee:
    uVar19 = struct_var5.field7_0x8;
    WVar18 = 0;
    uVar9 = '\x10';
    uVar10 = '\0';
// TODO: goto LAB_1008_0d18;
  case 0x68:
    pass1_1030_8344(_u16_1050_5748,0x4000001);
    uVar7 = param_2 | param_1;
    paVar9 = (astruct_57 *)(param_2 & 0xffff0000 | uVar7);
    if (uVar7 == 0) {
      return param_1;
    }
    if (PTR_LOOP_1050_4fe8 != NULL) {
      pcVar16 = load_string_1010_847e(_u16_1050_14cc,0x57b);
      BVar4 = MessageBox16(0x10,pcVar16,s_You_may_not_run_a_turn__The_game_1050_0172,struct_var5.field7_0x8);
      return BVar4;
    }
    HStack14 = LoadCursor16(0x7f02,0x0);
    HStack16 = SetCursor16(HStack14);
    puVar17 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x29),in_stack_0000fe3a,
                              in_stack_0000ff5e,in_stack_0000ff64,in_stack_0000ff68);
    uVar15 = (paVar9 >> 0x10);
    uStack20 = SUB42(puVar17,0x0);
    uStack20 = (puVar17 >> 0x10);
    pass1_1018_262e(puVar17);
    pass1_1030_838e(_u16_1050_5748);
    (_u16_1050_5748 + 0x8) = 0x1;
    pass1_1030_8326();
    pcVar16 = load_string_1010_847e(_u16_1050_14cc,0x5dc);
    paVar9 = (astruct_57 *)CONCAT22(uVar15,(pcVar16 >> 0x10));
    sys_1000_3f9c(CONCAT13(0x10,CONCAT12(0x50,local_64)),s__s__ld_1050_019c,pcVar16);
    uVar12 = 0;
    ppcVar1 = (code **)(param_5 + 0x14);
    (**ppcVar1)(0x0,struct_var5,(param_5 >> 0x10),0x0,local_64,&DAT_1050_1050);
    puVar17 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(uVar12,0x37),in_stack_0000fe34,
                              in_stack_0000ff58,in_stack_0000ff5e,in_stack_0000ff62);
    pass1_1008_a9ec(puVar17);
    SetCursor16(HStack16);
    uVar19 = struct_var5.field7_0x8;
    WVar18 = 0xfc;
    uVar11 = '\x11';
// TODO: goto LAB_1008_0e3d;
  default:
    if (((&struct_var5.field227_0xe8 + 0x2) | &struct_var5.field227_0xe8) == 0) {
      return 0x0;
    }
    puVar1 = struct_var5.field227_0xe8;
    ppcVar1 = (code **)(*struct_var5.field227_0xe8 + 0x40);
    BVar4 = (**ppcVar1)(0x8,puVar1,(puVar1 >> 0x10),param_6);
    return BVar4;
  case 0x6e:
    iVar12 = 0x2;
// TODO: goto LAB_1008_0cba;
  case 0x6f:
    uStack6 = FUN_1010_830a(param_1,param_2,0x1008,_u16_1050_14cc,0x1f8);
    uStack6 = SUB42(param_2,0x0);
    BVar4 = WinHelp16(0x0,0x3,CONCAT22(uStack6,uStack6),struct_var5.field7_0x8);
    return BVar4;
  case 0x70:
    iVar12 = 0x1;//
LAB_1008_0cba:
    uVar5 = pass1_1038_af40(struct_var5,pSVar8,_PTR_LOOP_1050_5b7c,struct_var5.field7_0x8,iVar12);
    return (BOOL16)uVar5;
  case 0x71:
    HVar2 = WinExec16(0x3,s_notepad_read_me_1050_0162);
    return HVar2;
  case 0x79:
    BVar4 = post_msg_1008_2d22(param_5);
    return BVar4;
  case 0x7a:
    uVar13 = 0xb;
// TODO: goto LAB_1008_0f3e;
  case 0x7b:
    uVar13 = 0x1e;
// TODO: goto LAB_1008_0f3e;
  case 0x7c:
    uVar13 = 0x1f;
// TODO: goto LAB_1008_0f3e;
  case 0x7d:
    uVar13 = 0x21;
// TODO: goto LAB_1008_0f3e;
  case 0x7e:
    uVar13 = 0x35;
// TODO: goto LAB_1008_0f3e;
  case 0x7f:
    uVar14 = 0x39;
    break;
  case 0x80:
    uVar13 = 0x22;
// TODO: goto LAB_1008_0f3e;
  case 0x81:
    uVar14 = 0x36;
    break;
  case 0x82:
    uVar14 = 0x37;
    break;
  case 0x83:
    uVar14 = 0x38;
    break;
  case 0x84:
    uVar14 = 0x3a;
    break;
  case 0x85:
    uVar14 = 0x3b;
    break;
  case 0x86:
    uVar14 = 0x3c;
    break;
  case 0x87:
    uVar14 = 0x3d;
    break;
  case 0x88:
    uVar14 = 0x3e;
    break;
  case 0x89:
    uVar14 = 0x3f;
    break;
  case 0x8a:
    uVar14 = 0x40;
    break;
  case 0x8b:
    uVar13 = 0xc;
// TODO: goto LAB_1008_0f3e;
  case 0x8c:
    uVar14 = 0x41;
    break;
  case 0x8d:
    uVar14 = 0x42;
    break;
  case 0x8e:
    uVar14 = 0x43;
    break;
  case 0x8f:
    uVar14 = 0x44;
    break;
  case 0x90:
    uVar14 = 0x45;
    break;
  case 0x91:
    uVar14 = 0x46;
    break;
  case 0x92:
    uVar14 = 0x47;
    break;
  case 0x93:
    uVar13 = 0x23;
// TODO: goto LAB_1008_0f3e;
  case 0x94:
    uVar13 = 0x24;
// TODO: goto LAB_1008_0f3e;
  case 0x95:
    uVar14 = 0x48;
    break;
  case 0x96:
    uVar14 = 0x49;
    break;
  case 0x97:
    uVar14 = 0x4a;
    break;
  case 0x98:
    uVar14 = 0x4b;
    break;
  case 0x99:
    uVar14 = 0x4c;
    break;
  case 0x9a:
    uVar13 = 0xd;
// TODO: goto LAB_1008_0f3e;
  case 0x9b:
    uVar14 = 0x4d;
    break;
  case 0x9c:
    uVar14 = 0x4e;
    break;
  case 0x9d:
    uVar14 = 0x4f;
    break;
  case 0x9e:
    uVar14 = 0x50;
    break;
  case 0x9f:
    uVar14 = 0x51;
    break;
  case 0xa0:
    uVar13 = 0xe;
// TODO: goto LAB_1008_0f3e;
  case 0xa1:
    uVar13 = 0xf;
// TODO: goto LAB_1008_0f3e;
  case 0xa2:
    uVar14 = 0x52;
    break;
  case 0xa3:
    uVar13 = 0x10;
// TODO: goto LAB_1008_0f3e;
  case 0xa4:
    uVar14 = 0x53;
    break;
  case 0xa5:
    uVar13 = 0x11;
// TODO: goto LAB_1008_0f3e;
  case 0xa6:
    uVar13 = 0x12;
// TODO: goto LAB_1008_0f3e;
  case 0xa7:
    uVar14 = 0x57;
    break;
  case 0xa8:
    uVar13 = 0x13;
// TODO: goto LAB_1008_0f3e;
  case 0xa9:
    uVar13 = 0x14;
// TODO: goto LAB_1008_0f3e;
  case 0xaa:
    uVar14 = 0x58;
    break;
  case 0xab:
    uVar14 = 0x63;
    break;
  case 0xac:
    uVar14 = 0x59;
    break;
  case 0xad:
    uVar14 = 0x5a;
    break;
  case 0xae:
    uVar14 = 0x5b;
    break;
  case 0xaf:
    uVar14 = 0x15;
    break;
  case 0xb0:
    uVar13 = 0x25;
// TODO: goto LAB_1008_0f3e;
  case 0xb1:
    uVar14 = 0x5c;
    break;
  case 0xb2:
    uVar14 = 0x16;
    break;
  case 0xb3:
    uVar14 = 0x5d;
    break;
  case 0xb4:
    uVar13 = 0x5e;
// TODO: goto LAB_1008_0f3e;
  case 0xb5:
    uVar14 = 0x5f;
    break;
  case 0xb6:
    uVar14 = 0x60;
    break;
  case 0xb7:
    uVar14 = 0x61;
    break;
  case 0xb8:
    uVar14 = 0x62;
    break;
  case 0xb9:
    uVar14 = 0x64;
    break;
  case 0xba:
    uVar14 = 0x65;
    break;
  case 0xbb:
    uVar14 = 0x66;
    break;
  case 0xbc:
    uVar14 = 0x67;
    break;
  case 0xbd:
    uVar14 = 0x68;
    break;
  case 0xbe:
    uVar14 = 0x69;
    break;
  case 0xbf:
    uVar13 = 0x17;
// TODO: goto LAB_1008_0f3e;
  case 0xc0:
    uVar13 = 0x18;
// TODO: goto LAB_1008_0f3e;
  case 0xc1:
    uVar13 = 0x19;
// TODO: goto LAB_1008_0f3e;
  case 0xc2:
    uVar13 = 0x1a;
// TODO: goto LAB_1008_0f3e;
  case 0xc3:
    uVar13 = 0x1b;
// TODO: goto LAB_1008_0f3e;
  case 0xc4:
    uVar13 = 0x1c;
// TODO: goto LAB_1008_0f3e;
  case 0xc5:
    uVar13 = 0x1d;
// TODO: goto LAB_1008_0f3e;
  case 0xc6:
    uVar13 = 0x4;
// TODO: goto LAB_1008_0f3e;
  case 0xc8:
    uVar13 = 0x3;
// TODO: goto LAB_1008_0f3e;
  case 0xc9:
    uVar13 = 0x1;
// TODO: goto LAB_1008_0f3e;
  case 0xca:
    uVar13 = 0x5;
// TODO: goto LAB_1008_0f3e;
  case 0xcb:
    pass1_1008_087e(in_AF,param_1,pSVar8);
    uVar13 = 0x6;
// TODO: goto LAB_1008_0f3e;
  case 0xcc:
    uVar13 = 0x7;
// TODO: goto LAB_1008_0f3e;
  case 0xcd:
    uVar13 = 0x8;
// TODO: goto LAB_1008_0f3e;
  case 0xce:
    uVar13 = 0x9;
// TODO: goto LAB_1008_0f3e;
  case 0xcf:
    uVar13 = 0xa;
// TODO: goto LAB_1008_0f3e;
  case 0xd0:
    uVar13 = 0x26;
// TODO: goto LAB_1008_0f3e;
  case 0xd1:
    uVar13 = 0x27;
// TODO: goto LAB_1008_0f3e;
  case 0xd2:
    uVar13 = 0x28;
// TODO: goto LAB_1008_0f3e;
  case 0xd3:
    uVar13 = 0x29;
// TODO: goto LAB_1008_0f3e;
  case 0xd4:
    uVar13 = 0x2a;
// TODO: goto LAB_1008_0f3e;
  case 0xd5:
    uVar13 = 0x2b;
// TODO: goto LAB_1008_0f3e;
  case 0xd6:
    uVar13 = 0x2c;
// TODO: goto LAB_1008_0f3e;
  case 0xd7:
    uVar13 = 0x2d;
// TODO: goto LAB_1008_0f3e;
  case 0xd8:
    uVar13 = 0x2e;
// TODO: goto LAB_1008_0f3e;
  case 0xd9:
    uVar13 = 0x2f;
// TODO: goto LAB_1008_0f3e;
  case 0xda:
    uVar13 = 0x30;
// TODO: goto LAB_1008_0f3e;
  case 0xdb:
    uVar13 = 0x31;
// TODO: goto LAB_1008_0f3e;
  case 0xdc:
    uVar13 = 0x32;
// TODO: goto LAB_1008_0f3e;
  case 0xdd:
    uVar13 = 0x33;
// TODO: goto LAB_1008_0f3e;
  case 0xde:
    uVar13 = 0x34;//
LAB_1008_0f3e:
    cursor_op_1008_2dcc(param_2,(astruct_20 *)param_5,uVar13,param_7,param_8);
    return param_1;
  case 0xdf:
    uVar14 = 0x55;
    break;
  case 0xe0:
    uVar14 = 0x56;
    break;
  case 0x100:
    win_1008_5c5c(param_1,pSVar8,_u16_1050_02a0,0x1dc);
    return param_1;
  case 0x12c:
    uVar19 = struct_var5.field7_0x8;
    WVar18 = 0xf020;
    uVar9 = '\x12';
    uVar10 = '\x01';//
LAB_1008_0d18:
    lresult_6 = SendMessage16(0x0,WVar18,CONCAT11(uVar10,uVar9),uVar19);
    return (BOOL16)lresult_6;
  case 0x12e:
    uVar19 = struct_var5.field7_0x8;
    WVar18 = 0xf060;
    uVar11 = '\x12';//
LAB_1008_0e3d:
    BVar4 = PostMessage16(0x0,WVar18,CONCAT11(0x1,uVar11),uVar19);
    return BVar4;
  }
  ui_op_1008_2c4e(pSVar8,param_8,param_5,uVar14);
  return param_1;
}
pub fn caseD_a7(mut param_1: u16 ,mut param_2: u16 )

{
  let mut unaff_BP: i16;
  astruct_72 *uVar1;

  ui_op_1008_2c4e(param_1,param_2,*(astruct_72 **)(unaff_BP + 0x6),0x57);
  return;
}
pub fn caseD_aa()

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x58);
  return;
}
pub fn caseD_ac()

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x59);
  return;
}
pub fn caseD_ad()

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x5a);
  return;
}
pub fn caseD_ae()

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x5b);
  return;
}
pub fn caseD_b1()

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x5c);
  return;
}
pub fn caseD_b3()

{
  let mut in_DX: u16;
  let mut unaff_BP: i16;
  let mut in_stack_0000ffee: u16;

  ui_op_1008_2c4e(in_DX,in_stack_0000ffee,*(astruct_72 **)(unaff_BP + 0x6),0x5d);
  return;
}
pub fn draw_op_1008_1230(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0xe0);
  fill_rect_1008_39ac((astruct_930 *)uVar1,(uVar1 >> 0x10));
  return;
}
pub fn pass1_1008_1246(mut param_1: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0xe8) != 0) {
    ppcVar1 = (code **)((param_1 + 0xe8) + 0x4c);
    (**ppcVar1)();
  }
  return;
}
pub fn pass1_1008_1272(mut param_1: u32,mut param_2: i16)

{
  code **ppcVar1;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0xe8) != 0) {
    ppcVar1 = (code **)((param_1 + 0xe8) + 0x88);
    (**ppcVar1)();
    return;
  }
  pass1_1008_9cc4(param_1 & 0xffff | uVar2 << 0x10,param_2);
  return;
}
pub fn pass1_1008_12aa(mut param_1: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0xe8) != 0) {
    ppcVar1 = (code **)((param_1 + 0xe8) + 0x8c);
    (**ppcVar1)();
    return;
  }
  pass1_1008_9ce0();
  return;
}
