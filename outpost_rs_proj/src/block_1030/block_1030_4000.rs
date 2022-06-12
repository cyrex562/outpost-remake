pub fn struct_1030_44be(param_1: *mut astruct_57,param_2: *mut astruct_138,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,
                     mut param_6: u16 ,mut param_7: u16 )

{
  astruct_138 *iVar1;
  let mut uVar1: u16;
  let mut puVar2: *mut u32;
  let mut uStack18: u16;

  uVar1 = (param_2 >> 0x10);
  iVar1 = (astruct_138 *)param_2;
  param_2 = 0x0;
  iVar1->field8_0x8 = 0x0;
  iVar1->field15_0x12 = 0x0;
  iVar1->field332_0x152 = 0x0;
  iVar1->field333_0x154 = 0x0;
  iVar1->field334_0x156 = 0x0;
  iVar1->field335_0x158 = 0x0;
  iVar1->field336_0x15a = 0x0;
  iVar1->field337_0x15c = 0x0;
  iVar1->field338_0x160 = 0x0;
  iVar1->field339_0x164 = 0x0;
  iVar1->field1364_0x568 = 0x0;
  _param_7 = (u8 **)CONCAT22(uStack18,0x2);
  puVar2 = mixed_1010_20ba(param_1,_u16_1050_0ed0,_param_7,param_3,param_4,param_5,param_6);
  iVar1->field1364_0x568 = (puVar2 + 0x64);
  return;
}
pub fn pass1_1030_4538(u32 *param_1)

{
  let mut uVar1: u16;

  fn_ptr_1000_17ce(*param_1);
  uVar1 = (param_1 >> 0x10);
  fn_ptr_1000_17ce(*(char **)(param_1 + 0x12));
  fn_ptr_1000_17ce(*(char **)(param_1 + 0x15c));
  return;
}



pub fn struct_1030_4574(param_1: *mut astruct_159) -> u32

{
  astruct_159 *iVar1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_159 *)param_1;
  iVar1->field12_0xc = DAT_1050_518c;
  iVar1->field13_0xe = 0x518e;
  iVar1->field14_0x10 = &DAT_1050_1050;
  return param_1 & 0xffff0000 | ZEXT24(&iVar1->field12_0xc);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_4594(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut puVar8: *mut u16;
  let mut puStack8: *mut u16;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puVar2 = (u16 *)(param_4 - 0x1U);
  mem_op_1000_179c(0x10,paVar4);
  puStack8 = (u16 *)(puVar2 & 0xffff | (long)paVar4 << 0x10);
  uVar3 = paVar4 | puVar2;
  if (uVar3 == 0x0) {
    puStack8 = NULL;
  }
  else {
    puVar8 = pass1_1008_3e38((astruct_19 *)CONCAT22(paVar4,puVar2 + 0x4));
    uVar3 = (puVar8 >> 0x10);
    puVar2 = puStack8;
  }
  uVar1 = SUB42(puVar2,0x0);
  iVar5 = (param_4 - 0x1U) * 0x12;
  load_string_1010_84ac(_u16_1050_14cc,(INT16)(_u16_1050_14cc >> 0x10),(iVar5 + 0x51b8));
  uVar7 = (puStack8 >> 0x10);
  iVar6 = puStack8;
  *puStack8 = uVar1;
  (iVar6 + 0x2) = uVar3;
  (iVar6 + 0xa) = (iVar5 + 0x51ba);
  pass1_1008_3e76((u16 *)(puStack8 & 0xffff0000 | (iVar6 + 0x4)),(iVar5 + 0x51c0),
                  (iVar5 + 0x51be),(iVar5 + 0x51bc));
  (iVar6 + 0xc) = iVar5 + 0x51c2;
  (iVar6 + 0xe) = &DAT_1050_1050;
  return;
}
pub fn pass1_1030_4628(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut piVar8: *mut i16;
  let mut uVar9: u16;
  let mut iStack24: i16;
  let mut piStack20: *mut i16;
  let mut iStack10: i16;
  let mut piStack8: *mut i16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar2 = param_4 - 0x1;
  uVar3 = uVar2;
  mem_op_1000_179c(0x28,paVar5);
  uVar4 = paVar5;
  piStack20 = CONCAT22(uVar4,uVar3);
  if ((uVar4 | uVar3) == 0x0) {
    piStack8 = NULL;
  }
  else {
    pass1_1008_3e38((astruct_19 *)CONCAT22(uVar4,uVar3 + 0x6));
    piStack8 = piStack20;
  }
  uVar9 = (piStack8 >> 0x10);
  iVar6 = piStack8;
  (iVar6 + 0x2) = 0x0;
  iVar7 = uVar2 * 0x5e;
  pass1_1008_3e76((u16 *)(piStack8 & 0xffff0000 | (iVar6 + 0x6)),(iVar7 + 0x5336),
                  (iVar7 + 0x5334),(iVar7 + 0x5332));
  (iVar6 + 0xc) = (iVar7 + 0x5348);
  *piStack8 = param_4;
  (iVar6 + 0xe) = (iVar7 + 0x534a);
  iStack10 = 0x0;
  do {
    uVar3 = ((uVar2 * 0x2f + iStack10) * 0x2 + 0x5338);
    (iVar6 + iStack10 * 0x2 + 0x12) = uVar3;
    iStack10 += 0x1;
  } while (iStack10 < 0x8);
  uVar1 = (&DAT_1050_5350 + uVar2 * 0x5e);
  pass1_1008_612e(uVar3,uVar1,(uVar1 >> 0x10));
  (iVar6 + 0x22) = uVar3;
  piVar8 = (uVar2 * 0x5e + 0x5354);
  *(i16 **)(iVar6 + 0x24) = piVar8;
  (iVar6 + 0x26) = &DAT_1050_1050;
  iVar7 = *piVar8;
  pass1_1000_4906((StructD *)CONCAT22(0x1050,piVar8),NULL,0x1e);
  iStack10 = 0x0;//
LAB_1030_474c:
  if (uVar3 <= iStack10) {
    return;
  }
  do {
    uVar4 = ((uVar2 * 0x5e + 0x534e) + iVar7) - 0x1;
    pass1_1008_612e(uVar4,iVar7,uVar4);
    iStack24 = 0x0;
    while( true ) {
      if (iStack10 < iStack24) {
        uVar1 = (iVar6 + 0x24);
        (uVar1 + iStack10 * 0x2) = uVar4;
        iStack10 += 0x1;
        goto LAB_1030_474c;
      }
      uVar1 = (iVar6 + 0x24);
      if ((uVar1 + iStack24 * 0x2) == uVar4) break;
      iStack24 += 0x1;
    }
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_4782(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,mut param_5: i16,mut param_6: i16,mut param_7: i16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  u8 **ppuVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut paVar6: *mut Struct57;
  let mut uVar7: u32;
  let mut paVar8: *mut Struct57;
  StructD *pSVar9;
  let mut iVar10: i16;
  let mut unaff_SI: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut puVar13: *mut u32;
  let mut uVar14: u32;
  let mut in_stack_0000fdc8: u16;
  let mut in_stack_0000fdca: u16;
  let mut in_stack_0000fdcc: u16;
  let mut in_stack_0000feec: u16;
  let mut in_stack_0000feee: u16;
  let mut in_stack_0000fef0: u16;
  let mut in_stack_0000fef2: u16;
  let mut in_stack_0000fef4: u16;
  let mut in_stack_0000fef6: u16;
  let mut in_stack_0000fef8: u16;
  let mut in_stack_0000fefa: u16;
  u8 uVar15;
  u8 uVar16;
  u8 *local_c4;
  let mut uStack194: u16;
  u8 *local_c0;
  let mut uStack190: u16;
  let mut iStack188: i16;
  let mut pcStack184: *mut c_char;
  let mut iStack180: i16;
  let mut pcStack178: *mut c_char;
  let mut pcStack174: *mut c_char;
  let mut uStack170: u16;
  let mut uStack168: u16;
  let mut uStack166: u16;
  let mut uStack164: u16;
  let mut uStack162: u16;
  u8 **ppuStack160;
  let mut iStack158: i16;
  let mut iStack156: i16;
  let mut iStack154: i16;
  let mut uStack152: u16;
  let mut pcStack150: *mut c_char;
  u8 local_92 [0x80];
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut piStack6: *mut i16;

  uVar15 = (u8)unaff_SI;
  uVar16 = (u8)(unaff_SI >> 0x8);
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((StructD *)param_1);
  }
  else {
    param_1 = param_1 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10;
  }
  uStack194 = param_1;
  local_c4 = PTR_LOOP_1050_5f2c;
  uVar2 = fn_ptr_op_1000_1708(0x20,0x0,0x1,PTR_LOOP_1050_5f2c,uStack194);
  uVar4 = param_1;
  pcStack184 = CONCAT22(uVar4,uVar2);
  paVar8 = (astruct_57 *)(param_1 & 0xffff0000);
  paVar6 = (astruct_57 *)(paVar8 | (uVar4 | uVar2));
  if ((uVar4 | uVar2) == 0x0) {
    uVar2 = 0x0;
  }
  else {
    pass1_1030_84ae(CONCAT22(uVar4,uVar2));
    paVar8 = paVar6;
  }
  piStack6 = CONCAT22(paVar8,uVar2);
  *piStack6 = param_6;
  pass1_1008_3f62((u16 *)CONCAT22(paVar8,uVar2 + 0x8),
                  (u16 *)CONCAT22(0x1050,&u16_1050_65e6 + param_6 * 0xa));
  if (param_5 != 0x0) {
    puVar13 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT13(uVar16,CONCAT12(uVar15,0x2f)),
                              in_stack_0000fdc8,in_stack_0000feec,in_stack_0000fef2,in_stack_0000fef6);
    uVar7 = paVar8 & 0xffff0000;
    uStack10 = SUB42(puVar13,0x0);
    uStack8 = (puVar13 >> 0x10);
    uStack14 = pass1_1018_04b8(puVar13);
    uVar7 = uVar7 & 0xffff0000 | uStack14 >> 0x10;
    uVar11 = uStack14;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack14);
    uStack18 = CONCAT22(uVar7,uVar11);
    pcStack150 = load_string_1010_847e(_u16_1050_14cc,0x62d);
    uVar7 = uVar7 & 0xffff0000 | pcStack150 >> 0x10;
    uVar2 = pass1_1030_2a98(uStack18);
    (piStack6 + 0x2) = uVar2;
    sys_1000_3f9c(CONCAT22(0x1050,local_92),pcStack150,uVar2);
    uVar2 = str_op_1008_60e8(uVar7,CONCAT22(0x1050,local_92));
    uVar11 = (piStack6 >> 0x10);
    (piStack6 + 0x4) = uVar2;
    (piStack6 + 0x6) = uVar7;
    uVar11 = FUN_1010_830a(param_6,uVar7,0x1008,_u16_1050_14cc,(param_6 * 0xa + 0x65ec));
    uVar12 = (piStack6 >> 0x10);
    (piStack6 + 0xe) = uVar11;
    (piStack6 + 0x10) = uVar7;
    uVar11 = FUN_1010_830a(uVar11,uVar7,0x1010,_u16_1050_14cc,(param_6 * 0xa + 0x65ee));
    uVar12 = (piStack6 >> 0x10);
    iVar10 = piStack6;
    (iVar10 + 0x12) = uVar11;
    (iVar10 + 0x14) = uVar7;
    uVar14 = pass1_1008_4772(*(astruct_76 **)(iVar10 + 0xe));
    uVar4 = (uVar7 >> 0x10);
    iStack154 = uVar14;
    uStack152 = (uVar14 >> 0x10);
    iStack156 = (iStack154 + 0x4) + -0x1;
    iStack158 = (iStack154 + 0x8) + -0x1;
    if (param_4 != 0x0) {
      ppuStack160 = (u8 **)(&PTR_LOOP_1050_000e + 0x1);
      if (uStack14 == 0x0) {
        debug_print_1008_6048(uStack152,s_get_site_data_without_planet__1050_56de);
      }
      else {
        ppuVar3 = &local_c4;
        pass1_1030_2f1a(uStack18,(u16 *)CONCAT22(0x1050,&local_c0),(u16 *)CONCAT22(0x1050,ppuVar3));
        pass1_1008_612e(ppuVar3,local_c4,local_c0);
        ppuStack160 = ppuVar3;
      }
      iVar10 = ppuStack160 * 0xa;
      uVar11 = (piStack6 >> 0x10);
      (piStack6 + 0x1c) = iVar10;
      paVar8 = (astruct_57 *)((long)iVar10 % 0x64 & 0xffffU | uVar4 << 0x10);
      (piStack6 + 0x1c) = iVar10 / 0x64;
      puVar13 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x2),in_stack_0000fdcc,
                                in_stack_0000fef0,in_stack_0000fef6,in_stack_0000fefa);
      paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | puVar13 >> 0x10);
      local_c4 = puVar13;
      uStack194 = (puVar13 >> 0x10);
      local_c0 = PTR_LOOP_1050_13ae;
      uVar2 = 0x84;
      puVar13 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)0x840009,in_stack_0000fdca,in_stack_0000feee,
                                in_stack_0000fef4,in_stack_0000fef8);
      uVar7 = paVar8 & 0xffff0000;
      uStack190 = pass1_1010_65d0(puVar13,uVar2);
      iStack188 = 0x3c;
      if (local_c0 < 0x3) {
        if (0x0 < uStack190) {
          iStack188 = 0x5a;
        }
      }
      else if (uStack190 == 0x1) {
        iStack188 = 0x44;
      }
      else if (uStack190 == 0x2) {
        iStack188 = 0x4b;
      }
      else if (uStack190 == 0x3) {
        iStack188 = 0x53;
      }
      else if (uStack190 == 0x4) {
        iStack188 = 0x5a;
      }
      iVar10 = iStack188 * ppuStack160;
      ppuStack160 = (u8 **)(iVar10 / 0x64);
      paVar8 = (astruct_57 *)((long)iVar10 % 0x64 & 0xffffU | uVar7 & 0xffff0000);
      uVar11 = (piStack6 >> 0x10);
      (piStack6 + 0x1a) = ppuStack160;
      uStack164 = ppuStack160 + (piStack6 + 0x1c);
      uVar4 = uStack164 * 0x6;
      uStack162 = uStack164;
      mem_op_1000_179c(uVar4,paVar8);
      uVar5 = paVar8;
      pcStack184 = CONCAT22(uVar5,uVar4);
      pSVar9 = (StructD *)(paVar8 & 0xffff0000 | (uVar5 | uVar4));
      if ((uVar5 | uVar4) == 0x0) {
        (piStack6 + 0x16) = 0x0;
      }
      else {
        pass1_1000_5586(0x3e38,0x1008,uStack164,0x6,uVar4,uVar5);
        (piStack6 + 0x16) = pcStack184;
      }
      uStack170 = uStack162 * 0x2;
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar9);
      }
      else {
        pSVar9 = (StructD *)(pSVar9 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
      }
      uVar2 = fn_ptr_op_1000_1708(uStack170,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar9);
      pcStack174 = CONCAT22(pSVar9,uVar2);
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar9);
        PTR_LOOP_1050_5f2e = pSVar9;
      }
      else {
      }
      uVar2 = fn_ptr_op_1000_1708(uStack170,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
      pcStack178 = CONCAT22(PTR_LOOP_1050_5f2e,uVar2);
      iStack180 = 0x0;//
LAB_1030_4b57:
      uVar4 = uStack162;
      if (iStack180 < uStack162) {
        do {
          pass1_1008_612e(uVar4,0x0,iStack156);
          uStack166 = uVar4;
          pass1_1008_612e(uVar4,0x0,iStack158);
          param_7 = 0x0;
          while( true ) {
            iVar10 = pcStack174;
            uVar11 = (pcStack174 >> 0x10);
            uVar12 = (pcStack178 >> 0x10);
            uStack168 = uVar4;
            if (iStack180 <= param_7) {
              iVar1 = iStack180 * 0x2;
              (iVar1 + iVar10) = uStack166;
              (iVar1 + pcStack178) = uVar4;
              uVar7 = (piStack6 + 0x16);
              pass1_1008_3e76((u16 *)(uVar7 & 0xffff0000 | (uVar7 + iStack180 * 0x6)),0x0,uVar4,
                              (iVar1 + iVar10));
              iStack180 += 0x1;
              goto LAB_1030_4b57;
            }
            if (((param_7 * 0x2 + iVar10) == uStack166) && ((param_7 * 0x2 + pcStack178) == uVar4)
               ) break;
            param_7 += 0x1;
          }
        } while( true );
      }
      fn_ptr_1000_17ce(pcStack174);
      pcStack184 = pcStack178;
      fn_ptr_1000_17ce(pcStack178);
    }
  }
  return;
}
pub fn pass1_1030_4bbe(mut param_1: u16 ,param_2: *mut astruct_117,mut param_3: i16)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  astruct_117 *pstruct117_5;
  let mut puVar5: *mut u32;
  let mut puVar6: *mut u32;
  astruct_117 *uVar7;

  uVar7 = (astruct_117 *)(param_2 >> 0x10);
  pstruct117_5 = (astruct_117 *)param_2;
  if (pstruct117_5->field15_0x12 == 0x0) {
    pass1_1030_4f5a(param_1,(astruct_117 *)(param_2 & 0xffff | ZEXT24(uVar7) << 0x10));
  }
  puVar6 = &pstruct117_5->field16_0x16;
  uVar3 = (&pstruct117_5->field15_0x12 + 0x2);
  puVar5 = (&pstruct117_5->field15_0x12 + param_3 * 0x98);
  for (iVar4 = 0x26; iVar4 != 0x0; iVar4 += -0x1) {
    puVar2 = puVar6;
    puVar6 = puVar6 + 0x1;
    puVar1 = puVar5;
    puVar5 = puVar5 + 0x1;
    *puVar2 = *puVar1;
  }
  return;
}
pub fn pass1_1030_4c06(param_1: *mut astruct_117,mut param_2: i16,u16_t param_3)

{
  i32 *plVar1;
  i32 *plVar2;
  let mut uVar3: u16;
  let mut iVar4: i16;
  astruct_117 *iVar5;
  i32 *plVar5;
  i32 *plVar6;
  astruct_117 *uVar7;

  uVar7 = (astruct_117 *)(param_1 >> 0x10);
  iVar5 = (astruct_117 *)param_1;
  if (iVar5[0xd].field10_0xa == 0x0) {
    pass1_1030_5044(param_3,(astruct_117 *)(param_1 & 0xffff | ZEXT24(uVar7) << 0x10));
  }
  plVar6 = &iVar5[0x6].field15_0x12;
  uVar3 = (&iVar5[0xd].field10_0xa + 0x2);
  plVar5 = (i32 *)(&iVar5[0xd].field10_0xa + param_2 * 0xae);
  for (iVar4 = 0x2b; iVar4 != 0x0; iVar4 += -0x1) {
    plVar2 = plVar6;
    plVar6 = plVar6 + 0x1;
    plVar1 = plVar5;
    plVar5 = plVar5 + 0x1;
    *plVar2 = *plVar1;
  }
  plVar6 = plVar5;
  return;
}
pub fn pass1_1030_4c52(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,mut param_5: u16 )

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut pcStack8: *mut c_char;
  let mut iStack4: i16;

  iStack4 = 0x0;
  while( true ) {
    uVar1 = pass1_1000_47a4(param_4,s___1050_518a);
    pcStack8 = CONCAT22(param_5,uVar1);
    if ((param_5 | uVar1) == 0x0) break;
    if (*pcStack8 != '\"') {
      iVar2 = pass1_1000_3e2c(CONCAT22(param_5,uVar1));
      iVar3 = param_3;
      uVar4 = (param_3 >> 0x10);
      if (iStack4 < 0x25) {
        (iStack4 * 0x4 + iVar3) = iVar2;
        (iStack4 * 0x4 + iVar3 + 0x2) = param_5;
      }
      else if (iStack4 == 0x25) {
        (iVar3 + 0x94) = iVar2;
      }
      else if (iStack4 == 0x26) {
        (iVar3 + 0x96) = iVar2;
      }
      else if (iStack4 == 0x27) {
        (iVar3 + 0x98) = iVar2;
      }
      else if (iStack4 == 0x28) {
        (iVar3 + 0x9a) = iVar2;
      }
      else if (iStack4 == 0x29) {
        (iVar3 + 0x9c) = iVar2;
      }
      else if (iStack4 == 0x2a) {
        (iVar3 + 0x9e) = iVar2;
      }
      else if (iStack4 == 0x2b) {
        (iVar3 + 0xa0) = iVar2;
      }
      else if (iStack4 == 0x2c) {
        (iVar3 + 0xa2) = iVar2;
      }
      iStack4 += 0x1;
    }
    param_4 = 0x0;
  }
  return;
}
pub fn pass1_1030_4d3a(mut param_1: u16 ,param_2: *mut astruct_117,param_3: *mut astruct_118,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  astruct_118 *pstruct118_3;
  astruct_118 *pstruct118_4;
  let mut pcStack8: *mut c_char;
  let mut iStack4: i16;

  iStack4 = 0x0;
  while( true ) {
    uVar1 = pass1_1000_47a4(param_4,s___1050_518a);
    pcStack8 = CONCAT22(param_1,uVar1);
    if ((param_1 | uVar1) == 0x0) break;
    if (*pcStack8 != '\"') {
      iVar2 = pass1_1000_3e2c(CONCAT22(param_1,uVar1));
      pstruct118_3 = (astruct_118 *)param_3;
      pstruct118_4 = (astruct_118 *)(param_3 >> 0x10);
      if (iStack4 < 0x25) {
        (&pstruct118_3->field_0x0 + iStack4 * 0x4) = iVar2;
        (&pstruct118_3->field_0x2 + iStack4 * 0x4) = param_1;
      }
      else if (iStack4 == 0x25) {
        pstruct118_3->field148_0x94 = iVar2;
      }
      else if (iStack4 == 0x26) {
        pstruct118_3->field149_0x96 = iVar2;
      }
      iStack4 += 0x1;
    }
    param_4 = 0x0;
  }
  return;
}
pub fn pass1_1030_4dbc(param_1: *mut astruct_117,mut param_2: u32,i32 param_3)

{
  i32 *plVar1;
  let mut piVar2: *mut i16;
  i32 lVar3;
  let mut uVar4: u16;
  astruct_117 *iVar5;
  astruct_117 *uVar5;

  iVar5 = (astruct_117 *)param_1;
  uVar5 = (astruct_117 *)(param_1 >> 0x10);
  if (0x0 < param_3) {
    &iVar5[0xd].field_0xe = param_2;
    iVar5[0xd].field15_0x12 = param_3;
  }
  if ((*(i32 *)&iVar5[0xd].field_0xe == 0x0) ||
     (lVar3 = iVar5[0xd].field15_0x12, plVar1 = &iVar5[0xd].field15_0x12, *plVar1 = *plVar1 + -0x1, lVar3 == 0x0)) {
    &iVar5[0xd].field_0xe = 0x0;
  }
  else {
    uVar4 = str_op_1000_3da4(*(char **)&iVar5[0xd].field_0xe);
    piVar2 = &iVar5[0xd].field_0xe;
    *piVar2 = *piVar2 + uVar4 + 0x2;
  }
  return;
}
pub fn pass1_1030_4e34(param_1: *mut astruct_117,param_2: *mut astruct_117,i32 param_3,char *param_4)

{
  while (param_3 != 0x0) {
    if ((*param_4 == '\r') || (*param_4 == '\n')) {
      *param_4 = '\0';
    }
    param_4 = (param_4 & 0xffff0000 | (param_4 + 0x1));
    param_3 = param_3 + -0x1;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 read_file_1030_4e70(param_1: *mut astruct_117,u32 *param_2,u8 **param_3,i32 param_4,mut param_5: u16 )

{
  let mut uVar1: u16;
  HFILE16 h_file;
  let mut unaff_SS: u16;
  let mut path: *mut c_char;
  i32 lVar1;
  let mut uVar2: u16;
  u8 *pbStack60;
  i32 iStack56;
  let mut uStack20: u32;

  *param_3 = NULL;
  *param_2 = 0x0;
  if (param_4 != 0x0) {
    uVar2 = 0x0;
    path = pass1_1030_5164(param_1,param_4);
    param_5 = (path >> 0x10);
    uVar1 = dos3_call_1000_51aa(path,param_5,uVar2);
    if (uVar1 == 0x0) {
      *param_2 = uStack20;
      h_file = _lopen16(0x0,path);
      if (h_file != 0xffff) {
        lVar1 = mem_op_1000_0a48(0x1,*param_2,(*param_2 >> 0x10),_PTR_LOOP_1050_5f2c);
        lVar1 = (lVar1 >> 0x10);
        param_3 = lVar1;
        (param_3 + 0x2) = lVar1;
        param_5 = lVar1;
        if ((lVar1 | param_3) != 0x0) {
          iStack56 = WIN16_hread(*param_2,*param_3,h_file);
          uVar2 = (iStack56 >> 0x10);
          _lclose16(h_file);
          pbStack60 = *param_3;
          while (iStack56 != 0x0) {
            if ((*(*pbStack60 + 0x608b) & 0x20) == 0x0) {
              *pbStack60 = *pbStack60 + 0x80;
            }
            pbStack60 = (pbStack60 & 0xffff0000 | (pbStack60 + 0x1));
            iStack56 = iStack56 + -0x1;
          }
          return uVar2;
        }
      }
    }
  }
  return param_5;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_4f5a(mut param_1: u16 ,param_2: *mut astruct_117)

{
  let mut pcVar1: *mut c_char;
  i32 *plVar2;
  let mut uVar3: u16;
  astruct_118 *iVar4;
  let mut pcVar4: *mut c_char;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut extraout_var: u16;
  astruct_117 *paVar8;
  astruct_117 *paVar9;
  let mut uStack22: u16;
  let mut uStack20: u32;
  let mut uStack14: u16;
  let mut uStack12: u16;
  i32 local_a;
  let mut local_6: *mut c_char;
  astruct_118 *uVar1;
  astruct_117 *pstruct117_7;
  astruct_117 *pstruct117_8;
  StructD *pSVar7;

  plVar2 = &local_a;
  uVar5 = read_file_1030_4e70(param_2,CONCAT22(0x1050,plVar2),(u8 **)CONCAT22(0x1050,&local_6),
                              (i32)s_bldgbld_dat_1050_56fc,param_1);
  pcVar1 = local_6;
  pSVar7 = (StructD *)CONCAT22(extraout_var,uVar5);
  if (plVar2 != NULL) {
    paVar8 = (astruct_117 *)param_2;
    paVar9 = (astruct_117 *)(param_2 >> 0x10);
    pcVar4 = local_6;
    pass1_1030_4e34(paVar8,paVar9,local_a,local_6);
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
      PTR_LOOP_1050_5f2e = pSVar7;
    }
    else {
    }
    uVar3 = fn_ptr_op_1000_1708(pcVar4 * 0x98,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
    &paVar8->field15_0x12 = uVar3;
    (&paVar8->field15_0x12 + 0x2) = PTR_LOOP_1050_5f2e;
    pass1_1030_4dbc(param_2,local_6,pcVar4 & 0xffff);
    uStack20 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    for (uStack22 = 0x0; uStack22 < pcVar4; uStack22 += 0x1) {
      uVar6 = (&paVar8->field15_0x12 + 0x2);
      iVar4 = (astruct_118 *)(&paVar8->field15_0x12 + uStack22 * 0x98);
      pass1_1030_4d3a(uVar6,(astruct_117 *)(param_2 & 0xffff | ZEXT24(paVar9) << 0x10),
                      (astruct_118 *)CONCAT22(uVar6,iVar4),uStack20);
      pass1_1030_4dbc(param_2,0x0,0x0);
      uStack20 = CONCAT22(uVar6,iVar4);
    }
    uStack12 = (pcVar1 >> 0x10);
    uStack14 = pcVar1;
    if ((uStack12 | uStack14) != 0x0) {
      call_fn_ptr_1000_0dc6(pcVar1);
    }
  }
  return;
}

