
u16 * pass1_1010_5004(StructD *param_1,param_2: u8,mut param_3: u16 )

{
  free_rsrc_1010_4b3e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return &param_1->address_offset_field_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_503e(u8 *param_1,param_2: *mut astruct_19,mut param_3: u16 )

{
  struct_op_1018_4cda(param_2,param_3);
    // just 0x5099
    // 0x1010:509a = ptr to fn ptr in table
  param_2->offset_0x0 = s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1;
  (param_2 + 0x2) = 0x1010;
  pass1_1018_4dce(param_1,param_2,0x1b3);
  _PTR_LOOP_1050_4230 = param_2;
  return;
}



StructD * pass1_1010_5074(StructD *param_1,param_2: u8)

{
  clenaup_win_ui_1018_4d22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1010_50b2(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0x0;
  (param_1 + 0xc) = 0x0;
  (param_1 + 0x10) = 0x0;
  (param_1 + 0x12) = 0x0;
  (param_1 + 0x16) = 0x0;
  param_1->offset_0x0 = 0x53f4;
  (param_1 + 0x2) = 0x1010;
  return;
}
pub fn pass1_1010_50f2(param_1: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x53f4;
  (param_1 + 0x2) = 0x1010;
  fn_ptr_1000_17ce(*(param_1 + 0xc));
  pass1_1010_1d80(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_5120(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut iVar8: i16;
  let mut uVar9: u16;

  uVar9 = (param_3 >> 0x10);
  iVar8 = param_3;
  if (*(i32 *)(iVar8 + 0x16) != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(iVar8 + 0x16));
    uVar5 = param_2 | param_1;
    if (uVar5 != 0x0) {
      uVar1 = (param_1 + 0x1f6);
      uVar4 = uVar1;
      pass1_1030_38f2(uVar1,0x3);
      uVar2 = uVar4;
      uVar6 = uVar5;
      uVar3 = uVar2;
      pass1_1030_38f2(uVar1,0x4);
      iVar7 = uVar6 + uVar5 + CARRY2(uVar3,uVar2);
      if ((0x0 < iVar7) || ((-0x1 < iVar7 && (param_4 <= uVar3 + uVar2)))) {
        (iVar8 + 0xa) = param_4;
        return;
      }
    }
  }
  return;
}
pub fn pass1_1010_519a(u8 *param_1,mut param_2: u32,char *param_3)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  astruct_92 *paVar4;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  astruct_246 *iVar5;
  astruct_247 *iVar6;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut piStack44: *mut i16;
  astruct_92 local_18;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_18),0x1,0x0,0x400);
  uVar8 = (param_2 >> 0x10);
  iVar5 = (astruct_246 *)param_2;
  iVar5->field14_0x10 = local_18.field5_0xc;
  fn_ptr_1000_17ce(*&iVar5->field12_0xc);
  uVar3 = iVar5->field14_0x10 << 0x2;
  mem_op_1000_179c(uVar3,paVar6);
  iVar5->field12_0xc = uVar3;
  iVar5->field13_0xe = paVar6;
  iVar5->field14_0x10 = 0x0;
  while( true ) {
    uVar5 = paVar6;
    paVar4 = &local_18;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
    paVar6 = (astruct_57 *)(uVar5 | paVar4);
    if ((uVar5 | paVar4) == 0x0) break;
    if (paVar4[0x1c].field4_0x8 != 0x8000002) {
      uVar1 = (&paVar4->field3_0x4 + 0x2);
      paVar6 = (astruct_57 *)uVar1;
      uVar2 = &iVar5->field12_0xc;
      uVar9 = (uVar2 >> 0x10);
      iVar7 = uVar2;
      iVar6 = (astruct_247 *)(iVar5->field14_0x10 * 0x4);
      piStack44 = (param_2 & 0xffff0000 | ZEXT24(&iVar5->field14_0x10));
      (iVar6 + iVar7) = &paVar4->field3_0x4;
      (iVar6 + iVar7 + 0x2) = uVar1;
      *piStack44 = *piStack44 + 0x1;
    }
  }
  param_3 = iVar5->field14_0x10;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn string_1010_5286(char *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32) -> u32

{
  short in_buf_len_5;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut pcVar2: *mut c_char;
  let mut UStack10: u32;
  let mut pcStack6: *mut c_char;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  pcStack6 = CONCAT22(param_2,param_1);
  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2 | param_1);
  if ((param_2 | param_1) == 0x0) {
    return 0x0;
  }
  mem_op_1000_179c(0x80,paVar1);
  in_buf_len_5 = (short)paVar1;
  UStack10 = CONCAT22(in_buf_len_5,param_1);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x80,param_1,in_buf_len_5);
  pass1_1000_3cea(UStack10,0x105013ac);
  pcVar2 = pass1_1038_4d28(pcStack6);
  pass1_1000_3cea(UStack10,pcVar2);
  return CONCAT22(in_buf_len_5,param_1);
}
pub fn pass1_1010_52fc(mut param_1: u16 ,u8 *param_2,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u16;

  pass1_1010_533c(param_2,param_3,param_4);
  uVar1 = (param_3 >> 0x10);
  (param_3 + 0x12) = param_1;
  (param_3 + 0x14) = param_2;
  return;
}
pub fn pass1_1010_531c(mut param_1: u16 ,u8 *param_2,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u16;

  pass1_1010_533c(param_2,param_3,param_4);
  uVar1 = (param_3 >> 0x10);
  (param_3 + 0x16) = param_1;
  (param_3 + 0x18) = param_2;
  return;
}
pub fn pass1_1010_533c(u8 *param_1,mut param_2: u32,char *param_3)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut pcVar7: *mut c_char;
  let mut uStack6: u16;
  char local_4 [0x2];

  pass1_1010_519a(param_1,param_2,CONCAT22(0x1050,local_4));
  uStack6 = 0x0;
  while( true ) {
    uVar6 = (param_2 >> 0x10);
    uVar5 = param_2;
    puVar1 = (uVar5 + 0x10);
    if (*puVar1 < uStack6 || *puVar1 == uStack6) {
      return;
    }
    uVar3 = (uVar5 + 0xc);
    uVar2 = (uVar3 + uStack6 * 0x4);
    pcVar7 = string_1010_5286(uVar2,param_1,uVar5,uVar6,uVar2);
    param_1 = (pcVar7 >> 0x10);
    iVar4 = pass1_1000_3d7a(param_3,(pcVar7 & 0xffff | ZEXT24(param_1) << 0x10));
    if (iVar4 == 0x0) break;
    fn_ptr_1000_17ce(pcVar7);
    uStack6 += 0x1;
  }
  return;
}



u16 * pass1_1010_53ce(param_1: *mut u16,param_2: u8)

{
  pass1_1010_50f2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_sys_op_1010_5404(mut param_1: i16,param_2: *mut astruct_19,mut param_3: u16 )

{
  let mut piVar1: *mut i16;
  u16 **ppuVar2;
  let mut uVar3: u32;
  code **ppcVar4;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut puVar9: *mut u16;
  let mut IVar10: i16;
  astruct_821 *uVar4;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut in_EDX: u32;
  let mut paVar14: *mut Struct57;
  let mut paVar15: *mut Struct57;
  let mut puVar16: *mut u8;
  let mut uVar17: u16;
  let mut uVar18: u16;
  astruct_19 *paVar19;
  let mut pcVar20: *mut c_char;
  let mut puVar21: *mut u32;
  let mut in_stack_0000fd74: u16;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000fea2: u16;
  u8 uVar22;
  u8 uVar23;
  let mut puStack50: *mut u16;
  let mut iStack42: i16;
  let mut uStack16: u32;
  let mut iStack12: i16;
  let mut iStack10: i16;
  let mut paVar13: *mut Struct57;

  uVar18 = (in_EDX >> 0x10);
  paVar19 = struct_op_1010_1d48(param_2,param_3);
  uVar11 = (paVar19 >> 0x10);
  paVar13 = (astruct_57 *)CONCAT22(uVar18,uVar11);
  uVar18 = 0x0;
  (param_2 + 0xa) = 0x0;
  (param_2 + 0xe) = 0x0;
  (param_2 + 0x12) = 0x0;
  (param_2 + 0x16) = 0x0;
  (param_2 + 0x1a) = 0x0;
  (param_2 + 0x62) = 0x0;
  (param_2 + 0x64) = 0x0;
  (param_2 + 0x68) = 0x0;
  (param_2 + 0x6c) = 0x0;
  (param_2 + 0x70) = 0x1;
  (param_2 + 0x7a) = 0x0;
  (param_2 + 0x7c) = 0x0;
  (param_2 + 0x7e) = 0x0;
  (param_2 + 0x80) = 0x0;
  (param_2 + 0x82) = 0x1;
  param_2->offset_0x0 = 0x6312;
  (param_2 + 0x2) = 0x1010;
  pass1_1010_6034(uVar11,param_2);
  mem_op_1000_179c(0x101,paVar13);
  (param_2 + 0xe) = uVar18;
  (param_2 + 0x10) = paVar13;
  pass1_1000_5008((param_2 + 0xe),paVar13,0x100);
  uVar18 = (paVar13 >> 0x10);
  uVar6 = str_op_1000_3da4(*(param_2 + 0xe));
  uVar5 = (param_2 + 0xe);
  uVar17 = (uVar5 >> 0x10);
  puVar16 = (uVar5 + uVar6);
  if (puVar16[-0x1] != '\\') {
    *puVar16 = 0x5c;
    uVar5 = (param_2 + 0xe);
    *(uVar5 + uVar6 + 0x1) = 0x0;
  }
  pcVar20 = load_string_1010_847e(_u16_1050_14cc,0x578);
  paVar13 = (astruct_57 *)CONCAT22(uVar18,(pcVar20 >> 0x10));
  pass1_1000_3cea((param_2 + 0xe),pcVar20);
  uVar7 = str_op_1008_60e8(paVar13,*(param_2 + 0xe));
  (param_2 + 0xa) = uVar7;
  (param_2 + 0xc) = paVar13;
  GetPrivateProfileString16
            (CONCAT22(paVar13,(param_2 + 0xa)),0x100,*(param_2 + 0xe),
             s_playerName_1050_148e + 0xc,s_rez_1050_13c0,s_general_1050_13b0);
  if (**(param_2 + 0xe) != '\0') {
    iVar8 = pass1_1000_3e2c((param_2 + 0xe));
    puVar21 = mixed_1010_20ba(paVar13,_u16_1050_0ed0,(u8 **)CONCAT22(param_1,0x48),in_stack_0000fd74,
                              in_stack_0000fe98,in_stack_0000fe9e,in_stack_0000fea2);
    paVar13 = (astruct_57 *)(paVar13 & 0xffff0000 | puVar21 >> 0x10);
    uVar18 = (puVar21 >> 0x10);
    iStack10 = (puVar21 + 0xa);
    iStack12 = (puVar21 + 0xc);
    (param_2 + 0x62) = (iVar8 != iStack10);
  }
  GetPrivateProfileString16
            (*(param_2 + 0xa),0x100,*(param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_falseMap_1050_13de,s_general_1050_13b0);
  if (**(param_2 + 0xe) != '\0') {
    param_1 = &DAT_1050_1050;
    iVar8 = pass1_1000_475e((param_2 + 0xe),s_on_1050_13c4);
    if (iVar8 == 0x0) {
      (param_2 + 0x80) = 0x1;
    }
  }
  param_1 = &DAT_1050_1050;
  GetPrivateProfileString16
            (*(param_2 + 0xa),0x100,*(param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_music_1050_13d2,s_general_1050_13b0);
  if (**(param_2 + 0xe) != '\0') {
    param_1 = s_off_1050_13c8;
    iVar8 = pass1_1000_475e((param_2 + 0xe),s_off_1050_13c8);
    if (iVar8 == 0x0) {
      (param_2 + 0x74) = 0x0;
    }
  }
  param_1 = s_general_1050_13b0;
  GetPrivateProfileString16
            (*(param_2 + 0xa),0x100,*(param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_sound_1050_13d8,s_general_1050_13b0);
  if (**(param_2 + 0xe) != '\0') {
    uVar3 = (param_2 + 0xe);
    param_1 = (uVar3 >> 0x10);
    iVar8 = pass1_1000_475e(uVar3,s_off_1050_13c8);
    if (iVar8 == 0x0) {
      (param_2 + 0x72) = 0x0;
    }
  }
  param_1 = &DAT_1050_1050;
  GetPrivateProfileString16
            (*(param_2 + 0xa),0x100,*(param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_anims_1050_13cc,s_general_1050_13b0);
  if (**(param_2 + 0xe) != '\0') {
    uVar3 = (param_2 + 0xe);
    param_1 = uVar3;
    iVar8 = pass1_1000_475e(uVar3,s_off_1050_13c8);
    if (iVar8 == 0x0) {
      (param_2 + 0x1e) = 0x0;
    }
  }
  param_1 = s_movies_1050_13e8;
  GetPrivateProfileString16
            (*(param_2 + 0xa),0x100,*(param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_movies_1050_13e8,s_general_1050_13b0);
  if (**(param_2 + 0xe) != '\0') {
    param_1 = s_tile2_bmp_1050_1538;
    iVar8 = pass1_1000_475e((param_2 + 0xe),s_off_1050_13c8);
    if (iVar8 == 0x0) {
      (param_2 + 0x20) = 0x0;
    }
  }
  param_1 = &DAT_1050_1050;
  GetPrivateProfileString16
            (*(param_2 + 0xa),0x100,*(param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_turns_1050_1466,s_general_1050_13b0);
  paVar14 = paVar13;
  if (**(param_2 + 0xe) != '\0') {
    uVar6 = pass1_1000_3e2c((param_2 + 0xe));
    uVar12 = paVar13 | uVar6;
    paVar14 = (astruct_57 *)(paVar13 & 0xffff0000 | uVar12);
    if (uVar12 != 0x0) {
      (param_2 + 0x76) = uVar6;
      (param_2 + 0x78) = paVar13;
      paVar14 = paVar13;
    }
  }
  param_1 = s_playerName_1050_148e + 0xc;
  GetPrivateProfileString16
            (*(param_2 + 0xa),0x100,*(param_2 + 0xe),s_playerName_1050_148e + 0xc,
             s_turnsDlgStatus_1050_146c,s_general_1050_13b0);
  if ((**(param_2 + 0xe) != '\0') &&
     (iVar8 = pass1_1000_475e((param_2 + 0xe),s_on_1050_13c4), iVar8 == 0x0)) {
    (param_2 + 0x7a) = 0x1;
  }
  pcVar20 = *(param_2 + 0xe);
  param_1 = (pcVar20 >> 0x10);
  GetPrivateProfileString16
            (*(param_2 + 0xa),0x100,pcVar20,s_playerName_1050_148e + 0xc,s_savePath_1050_147c,
             s_general_1050_13b0);
  if (**(param_2 + 0xe) != '\0') {
    uVar7 = str_op_1008_60e8(paVar14,*(param_2 + 0xe));
    (param_2 + 0x1a) = uVar7;
    (param_2 + 0x1c) = paVar14;
  }
  pcVar20 = *(param_2 + 0xe);
  param_1 = pcVar20;
  GetPrivateProfileString16
            (*(param_2 + 0xa),0x100,pcVar20,s_playerName_1050_148e + 0xc,s_aiName_1050_1486,
             s_general_1050_13b0);
  if (**(param_2 + 0xe) != '\0') {
    uVar7 = str_op_1008_60e8(paVar14,*(param_2 + 0xe));
    (param_2 + 0x68) = uVar7;
    (param_2 + 0x6a) = paVar14;
  }
  param_1 = 0x100;
  puVar9 = GetPrivateProfileString16
                             (*(param_2 + 0xa),0x100,*(param_2 + 0xe),
                              s_playerName_1050_148e + 0xc,s_playerName_1050_148e,s_general_1050_13b0);
  if (**(param_2 + 0xe) != '\0') {
    puVar9 = str_op_1008_60e8(paVar14,*(param_2 + 0xe));
    (u16*)(param_2 + 0x6c) = puVar9;
    (param_2 + 0x6e) = paVar14;
  }
  if ((param_2 + 0x62) == 0x0) {
    IVar10 = GetSystemMetrics16(SM_CYCAPTION);
    iStack42 = 0x1;
    loop {
      get_private_profile_string_1010_6132(param_2,iStack42);
      iVar8 = iStack42 * 0x8 + param_2;
      if (((((iVar8 + 0x22) < 0x0) || ((iVar8 + 0x24) < 0x0)) ||
          (piVar1 = (iVar8 + 0x22), *piVar1 != iStack10 - IVar10 && iStack10 - IVar10 <= *piVar1)) ||
         (puVar9 = (iStack12 - IVar10), ppuVar2 = (iVar8 + 0x24),
         *ppuVar2 != puVar9 && puVar9 <= *ppuVar2)) {
        puVar9 = pass1_1000_4906(
                                 (param_2 & 0xffff0000 | (iStack42 * 0x8 + param_2 + 0x22)),NULL,0x8)
        ;
      }
      iStack42 += 0x1;
    } while (iStack42 < 0x8);
  }
  mem_op_1000_179c(0xc,paVar14);
  uVar6 = paVar14 | puVar9;
  paVar13 = (astruct_57 *)(paVar14 & 0xffff0000);
  paVar15 = (astruct_57 *)(paVar13 | uVar6);
  if (uVar6 == 0x0) {
    puVar9 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar14,puVar9));
    paVar13 = paVar15;
  }
  (u16*)(param_2 + 0x64) = puVar9;
  (param_2 + 0x66) = paVar13;
  uVar5 = (param_2 + 0xe);
  pass1_1000_5008(uVar5,(uVar5 >> 0x10),0x100);
  uVar6 = str_op_1000_3da4(*(param_2 + 0xe));
  uVar5 = (param_2 + 0xe);
  uVar18 = (uVar5 >> 0x10);
  puVar16 = (uVar5 + uVar6);
  if (puVar16[-0x1] != '\\') {
    *puVar16 = 0x5c;
    uVar5 = (param_2 + 0xe);
    *(uVar5 + uVar6 + 0x1) = 0x0;
  }
  uVar4 = (astruct_821 *)str_op_1008_60e8(paVar13,*(param_2 + 0xe));
  uStack16 = CONCAT22(paVar13,uVar4);
  mem_op_1000_179c(0x8,paVar13);
  uVar6 = paVar13;
  puStack50 = CONCAT22(uVar6,uVar4);
  paVar13 = (astruct_57 *)(paVar13 & 0xffff0000 | (uVar6 | uVar4));
  if ((uVar6 | uVar4) != 0x0) {
    *puStack50 = 0x389a;
    uVar4->field2_0x2 = 0x1008;
    uVar4->field3_0x4 = uStack16;
    *puStack50 = 0x6322;
    uVar4->field2_0x2 = 0x1010;
  }
  ppcVar4 = (code **)((param_2 + 0x64) + 0x4);
  (**ppcVar4)();
  pcVar20 = *(param_2 + 0xe);
  param_1 = (pcVar20 >> 0x10);
  GetPrivateProfileString16
            (*(param_2 + 0xa),0x100,pcVar20,s_playerName_1050_148e + 0xc,s_opimage_1050_13f0,
             s_general_1050_13b0);
  if (**(param_2 + 0xe) != '\0') {
    uVar5 = (param_2 + 0xe);
    uVar18 = uVar5;
    uVar22 = (uVar5 >> 0x10);
    uVar23 = (uVar5 >> 0x18);
    while( true ) {
      uVar7 = pass1_1000_47a4(CONCAT13(uVar23,CONCAT12(uVar22,uVar18)),s___1050_13f8);
      if ((paVar13 | uVar7) == 0x0) break;
      unk_str_op_1000_3d3e(CONCAT13(0x10,CONCAT12(0x50,&param_1)),CONCAT22(paVar13,uVar7));
      uVar6 = str_op_1000_3da4(CONCAT22(0x1050,&param_1));
      if ((&stack0xfecb)[uVar6] != '\\') {
        *(&param_1 + uVar6) = 0x5c;
        *(&param_1 + uVar6 + 0x1) = 0x0;
      }
      uVar6 = str_op_1008_60e8(paVar13,CONCAT22(0x1050,&param_1));
      uStack16 = CONCAT22(paVar13,uVar6);
      mem_op_1000_179c(0x8,paVar13);
      uVar12 = paVar13;
      puStack50 = CONCAT22(uVar12,uVar6);
      paVar13 = (astruct_57 *)(paVar13 & 0xffff0000 | (uVar12 | uVar6));
      if ((uVar12 | uVar6) != 0x0) {
        *puStack50 = 0x389a;
        (uVar6 + 0x2) = 0x1008;
        (uVar6 + 0x4) = uStack16;
        *puStack50 = 0x6322;
        (uVar6 + 0x2) = 0x1010;
      }
      ppcVar4 = (code **)((param_2 + 0x64) + 0x8);
      (**ppcVar4)();
      uVar18 = 0x0;
      uVar22 = 0x0;
      uVar23 = 0x0;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn write_private_profile_str_1010_5b10(mut param_1: u16 ,StructD *param_2)

{
  let mut puVar2: *mut u32;
  code **ppcVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut in_register_0000000a: u16;
  StructD *iVar10;
  let mut unaff_SI: u16;
  let mut uVar10: u16;
  let mut puVar10: *mut u32;
  let mut in_stack_0000fe76: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ffa0: u16;
  let mut in_stack_0000ffa4: u16;
  let mut iStack12: i16;
  let mut puVar1: *mut u32;
  let mut uVar3: u16;

  uVar10 = (param_2 >> 0x10);
  iVar10 = param_2;
  param_2->address_offset_field_0x0 = 0x6312;
  iVar10->address_offset_field_0x2 = 0x1010;
  puVar10 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                            (u8 **)CONCAT22(unaff_SI,0x48),in_stack_0000fe76,in_stack_0000ff9a,in_stack_0000ffa0,
                            in_stack_0000ffa4);
  sys_1000_3f9c(*&iVar10->field8_0xe,s__d__d_1050_149c,(puVar10 + 0xa));
  if (&iVar10->field_0x80 == 0x0) {
    // actualy just 0x13c8
    uVar4 = s_off_1050_13c8;
  }
  else {
    // actually just 0x13c4
    uVar4 = s_on_1050_13c4;
  }
  WritePrivateProfileString16
            (*&iVar10->field6_0xa,CONCAT22(0x1050,uVar4),s_falseMap_1050_13de,s_general_1050_13b0);
  WritePrivateProfileString16
            (*&iVar10->field6_0xa,*&iVar10->field8_0xe,s_rez_1050_13c0,s_general_1050_13b0);
  if (&iVar10->field_0x1e == 0x0) {
    // actually just 0x13c8
    uVar5 = s_off_1050_13c8;
  }
  else {
    // actually just 0x13c4
    uVar5 = s_on_1050_13c4;
  }
  WritePrivateProfileString16
            (*&iVar10->field6_0xa,CONCAT22(0x1050,uVar5),s_anims_1050_13cc,s_general_1050_13b0);
  if (&iVar10->field_0x74 == 0x0) {
    // 0x13c8
    uVar6 = s_off_1050_13c8;
  }
  else {
    // 0x13c4
    uVar6 = s_on_1050_13c4;
  }
  WritePrivateProfileString16
            (*&iVar10->field6_0xa,CONCAT22(0x1050,uVar6),s_music_1050_13d2,s_general_1050_13b0);
  if (&iVar10->field_0x72 == 0x0) {
    // 0x13c8
    uVar7 = s_off_1050_13c8;
  }
  else {
    // 0x13c4
    uVar7 = s_on_1050_13c4;
  }
  WritePrivateProfileString16
            (*&iVar10->field6_0xa,CONCAT22(0x1050,uVar7),s_sound_1050_13d8,s_general_1050_13b0);
  if (iVar10->field19_0x20 == 0x0) {
    // 0x13c8
    uVar8 = s_off_1050_13c8;
  }
  else {
    // 0x13c4
    uVar8 = s_on_1050_13c4;
  }
  WritePrivateProfileString16
            (*&iVar10->field6_0xa,CONCAT22(0x1050,uVar8),s_movies_1050_13e8,s_general_1050_13b0);
  sys_1000_3f9c(*&iVar10->field8_0xe,s__lu_1050_14a2,&iVar10->field_0x76);
  WritePrivateProfileString16
            (*&iVar10->field6_0xa,*&iVar10->field8_0xe,s_turns_1050_1466,s_general_1050_13b0);
  if (&iVar10->field_0x7a == 0x0) {
    // 0x13c8
    uVar9 = SUB42(s_off_1050_13c8,0x0);
  }
  else {
    // 0x13c4
    uVar9 = SUB42(s_on_1050_13c4,0x0);
  }
  WritePrivateProfileString16
            (*&iVar10->field6_0xa,CONCAT22(0x1050,uVar9),s_turnsDlgStatus_1050_146c,s_general_1050_13b0
            );
  WritePrivateProfileString16
            (*&iVar10->field6_0xa,*&iVar10->field14_0x1a,s_savePath_1050_147c,s_general_1050_13b0);
  WritePrivateProfileString16
            (*&iVar10->field6_0xa,*&iVar10->field_0x68,s_aiName_1050_1486,s_general_1050_13b0);
  WritePrivateProfileString16
            (*&iVar10->field6_0xa,*&iVar10->field_0x6c,s_playerName_1050_148e,s_general_1050_13b0);
  iStack12 = 0x1;
  loop {
    caseD_13(param_2,iStack12);
    iStack12 += 0x1;
  } while (iStack12 < 0x8);
  fn_ptr_1000_17ce(*&iVar10->field6_0xa);
  fn_ptr_1000_17ce(*&iVar10->field8_0xe);
  fn_ptr_1000_17ce(*&iVar10->field11_0x12);
  fn_ptr_1000_17ce(*(&iVar10->field12_0x14 + 0x2));
  fn_ptr_1000_17ce(*&iVar10->field14_0x1a);
  puVar2 = &iVar10->field_0x64;
  uVar3 = &iVar10->field_0x66;
  if ((uVar3 | puVar2) != 0x0) {
    ppcVar3 = (code **)*puVar2;
    (**ppcVar3)(0x1000,puVar2,uVar3,0x1);
  }
  fn_ptr_1000_17ce(*&iVar10->field_0x68);
  fn_ptr_1000_17ce(*&iVar10->field_0x6c);
  pass1_1010_1d80(param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_5d9c(u8 *param_1,mut param_2: u32,mut param_3: i16)

{
  let mut in_register_0000000a: u16;
  let mut puVar1: *mut u32;
  let mut in_stack_0000fea2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000fffa: u16;

  (param_2 + 0x1e) = param_3;
  if (param_3 == 0x0) {
    puVar1 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000fffa,0x2e),in_stack_0000fea2,in_stack_0000ffc6,
                             in_stack_0000ffcc,in_stack_0000ffd0);
    pass1_1018_209c(puVar1);
  }
  return;
}



u16 pass1_1010_5dc6(mut param_1: u32,mut param_2: u32)

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  let mut uVar3: u16;
  HFILE16 in_stack_0000ffdc;
  u8 *local_c [0x3];
  u16 local_6 [0x2];

  BVar1 = write_to_file_1008_7cac(param_2);
  if (BVar1 != 0x0) {
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    BVar1 = pass1_1008_7c2a(param_2,*(iVar2 + 0x68));
    if (BVar1 != 0x0) {
      BVar1 = pass1_1008_7c2a(param_2,*(iVar2 + 0x6c));
      if (BVar1 != 0x0) {
        local_c[0] = PTR_LOOP_1050_13ae;
        BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffdc);
        if (BVar1 != 0x0) {
          local_6[0] = (iVar2 + 0x82);
          BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_6),0x2,in_stack_0000ffdc);
          if (BVar1 != 0x0) {
            return 0x1;
          }
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}
pub fn pass1_1010_5e56(mut param_1: i16,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut puVar1: *mut u8;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut iVar4: i16;
  let mut uVar5: u16;
  HFILE16 HVar6;
  let mut uVar7: u16;
  let mut local_404: *mut u8;
  u8 local_402 [0x400];

  HVar6 = (HFILE16)param_4;
  uVar7 = (param_4 >> 0x10);
  read_file_1008_7cfe(HVar6,uVar7,0x4);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
  }
  else {
    puVar1 = local_402;
    read_file_1008_7c6e(HVar6,uVar7,CONCAT22(0x1050,puVar1));
    if (puVar1 != NULL) {
      uVar2 = str_op_1008_60e8(param_2,CONCAT22(0x1050,local_402));
      uVar5 = (param_3 >> 0x10);
      iVar4 = param_3;
      (iVar4 + 0x68) = uVar2;
      (iVar4 + 0x6a) = param_2;
      puVar1 = local_402;
      read_file_1008_7c6e(HVar6,uVar7,CONCAT22(0x1050,puVar1));
      if (puVar1 != NULL) {
        uVar2 = str_op_1008_60e8(param_2,CONCAT22(0x1050,local_402));
        (iVar4 + 0x6c) = uVar2;
        (iVar4 + 0x6e) = param_2;
        BVar3 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(0x1050,&local_404),0x2);
        if (BVar3 != 0x0) {
          PTR_LOOP_1050_13ae = local_404;
          if (u16_1050_0312 < 0x2) {
            return;
          }
          BVar3 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | (iVar4 + 0x82)),0x2);
          if (BVar3 != 0x0) {
            return;
          }
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}
pub fn struct_1010_5f1e(mut param_1: u16 ,param_2: *mut astruct_73,char *param_3)

{
  let mut uVar1: u16;
  astruct_73 *iVar3;
  astruct_73 *uVar3;

  uVar3 = (astruct_73 *)(param_2 >> 0x10);
  iVar3 = (astruct_73 *)param_2;
  fn_ptr_1000_17ce(iVar3->field22_0x16);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  &iVar3->field22_0x16 = uVar1;
  (&iVar3->field22_0x16 + 0x2) = param_1;
  return;
}
pub fn pass1_1010_5f4c(mut param_1: u16 ,param_2: *mut astruct_484,char *param_3)

{
  let mut uVar1: u16;
  astruct_484 *iVar3;
  astruct_484 *uVar2;

  uVar2 = (astruct_484 *)(param_2 >> 0x10);
  iVar3 = (astruct_484 *)param_2;
  fn_ptr_1000_17ce(*&iVar3->field18_0x12);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3->field18_0x12 = uVar1;
  iVar3->field19_0x14 = param_1;
  return;
}



pub fn pass1_1010_5f7a(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16) -> u32

{
  let mut iVar1: i16;

  iVar1 = param_4 * 0x8 + param_1;
  if (((iVar1 + 0x26) == 0x0) && ((iVar1 + 0x28) == 0x0)) {
    return 0x0;
  }
  return CONCAT22(param_2,param_4 * 0x8 + param_1 + 0x22);
}
pub fn pass1_1010_5fb0(mut param_1: u32,mut param_2: u16 ,u32 *param_3,mut param_4: u16 ,mut param_5: i16)

{
  let mut uVar1: u16;
  astruct_656 *iVar1;

  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_656 *)(param_1 + param_5 * 0x8);
  iVar1->field34_0x22 = *param_3;
  iVar1->field35_0x26 = param_3[0x1];
  return;
}
pub fn pass1_1010_5fd8(mut param_1: u16 ,param_2: *mut astruct_485,char *param_3)

{
  let mut uVar1: u16;
  astruct_485 *iVar3;
  astruct_485 *uVar2;

  uVar2 = (astruct_485 *)(param_2 >> 0x10);
  iVar3 = (astruct_485 *)param_2;
  fn_ptr_1000_17ce(*&iVar3->field104_0x68);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3->field104_0x68 = uVar1;
  iVar3->field105_0x6a = param_1;
  return;
}
