
pub fn pass1_1028_737e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  u32 *puVar1;
  u32 *puVar2;
  u32 *puVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = paVar6;
  puStack10 = (u16 *)CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    (u32)(param_1 + 0x4) = (u32)((int)param_3 + 0x4);
    puVar3 = (u32 *)((int)param_3 + 0x8);
    puVar7 = (u32 *)(param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0x749e;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_740c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 *puVar5;
  let mut extraout_DX: u16;
  u32 *puVar6;
  i32 lStack14;
  u32 *puStack10;

  puVar6 = pass1_1008_c6fa(_u16_1050_06e0,param_3);
  puVar5 = ((u32)puVar6 >> 0x10);
  uVar3 = puVar6;
  pass1_1038_4d6e(uVar3,puVar5,(astruct_691 *)param_4,puVar6);
  puStack10 = (u32 *)CONCAT22(puVar5,uVar3);
  uVar2 = *puStack10;
  ppcVar1 = (code **)uVar2 + 0x8;
  uVar4 = uVar3;
  (**ppcVar1)((int)&u16_1050_1038,uVar3,puVar5);
  lStack14 = CONCAT22(extraout_DX,uVar4);
  if (puStack10 != NULL) {
    ppcVar1 = (code **)uVar2;
    (**ppcVar1)((int)&u16_1050_1038,uVar3,(char)puVar5,0x1);
  }
  if (lStack14 != 0x0) {
    return;
  }
  return;
}



StructD * pass1_1028_7472(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_74ae(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0x1387);
  param_1->offset_0x0 = 0x819a;
    // just 0x1028
  ((int)param_1 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCEvent_1050_4ff4);
  return param_1;
}



u16 pass1_1028_74e4(uchar param_1,param_2: *mut astruct_57,mut param_3: u32)

{
  let mut iVar1: i16;

  pass1_1028_7fb6(param_1,param_3);
  pass1_1028_7c4e(param_2,param_3);
  pass1_1028_7dfc(param_1,param_2,param_3);
  iVar1 = post_msg_1028_76da();
  pass1_1028_767e(iVar1,param_2);
  pass1_1028_75bc();
  pass1_1028_78b8(param_1,(long)param_2,param_3);
  return 0x1;
}
pub fn pass1_1028_752e(mut param_1: u16 ,u8 *param_2,mut param_3: u32)

{
  u32 *puVar1;
  u32 *puVar2;
  u32 *puVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = paVar6;
  puStack10 = (u16 *)CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    (u32)(param_1 + 0x4) = (u32)((int)param_3 + 0x4);
    puVar3 = (u32 *)((int)param_3 + 0x8);
    puVar7 = (u32 *)(param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0x819a;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_75bc(void)

{
  astruct_92 *paVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut in_stack_0000ffcc: u16;
  let mut uVar5: u32;
  let mut uStack28: u32;
  astruct_92 local_18;

  uVar2 = ((qword)*_PTR_LOOP_1050_65e2 % 0x7b);
  uVar4 = (u32)uVar2;
  if ((uVar2 == 0x0) && (0x95 < *_PTR_LOOP_1050_65e2)) {
    uVar5 = CONCAT22(0x7603,in_stack_0000ffcc);
    pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_18),0x1,0x0,0x400);
    while( true ) {
      paVar1 = &local_18;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
      uStack28 = CONCAT22(uVar4,paVar1);
      uVar2 = uVar4 | paVar1;
      uVar4 = uVar4 & 0xffff0000 | (u32)uVar2;
      if (uVar2 == 0x0) break;
      pass1_1008_612e(paVar1,0x1,0x64);
      if ((int)paVar1 < 0x6) {
        pass1_1038_362e(uStack28,paVar1,uVar5,uVar4);
      }
    }
    if (local_18.field6_0x10 != 0x0) {
      local_18.field5_0xc._0_2_ = 0x1;
      local_18.field5_0xc = 0x0;
    }
    uVar4 = (u32)local_18.field5_0xc;
    local_18.field4_0x8._0_2_ = local_18.field5_0xc;
    local_18.field4_0x8 = local_18.field5_0xc;
    while( true ) {
      uVar2 = uVar4;
      paVar1 = &local_18;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
      uVar3 = uVar2 | paVar1;
      uVar4 = (u32)uVar3;
      if (uVar3 == 0x0) break;
      pass1_1038_3698(paVar1,uVar3,CONCAT22(uVar2,paVar1));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_767e(mut param_1: i16,mut param_2: u16 )

{
  let mut uVar1: u16;
  u32 *puVar2;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffec: u16;

  pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x8000001);
  if (((param_1 + 0x152) != 0x0) && (uVar1 = ((qword)*_PTR_LOOP_1050_65e2 % 0x64), uVar1 == 0x0)) {
    puVar2 = mixed_1010_20ba((astruct_57 *)(u32)uVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffec,0x40),
                             in_stack_0000fe94,in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
    load_str_and_sprintf_1008_b78a((StructD *)CONCAT22((int)puVar2,(int)((u32)puVar2 >> 0x10)),(u32)puVar2);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn post_msg_1028_76da(void)

{
  i32 lVar1;
  let mut uVar2: u16;
  astruct_57 *in_EDX;
  u32 *puVar3;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffe4: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;

  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22((int)((u32)in_stack_0000ffe4 >> 0x10),0x2c),
                           in_stack_0000fe8e,in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar2 = ((u32)puVar3 >> 0x10);
  lVar1 = *(i32 *)((int)puVar3 + 0xc);
  uStack8 = ((u32)lVar1 >> 0x10);
  uStack10 = lVar1;
  if (((uStack8 | uStack10) != 0x0) && (*_PTR_LOOP_1050_65e2 == lVar1)) {
    PostMessage16(0x0,0x106,0x111,HWND16_1050_0396);
    (u32)((int)puVar3 + 0xc) = 0x0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_7742(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,param_4: *mut astruct_15)

{
  code **ppcVar1;
  astruct_670 *paVar2;
  let mut uVar3: u16;
  u8 *puVar4;
  let mut uVar5: u16;
  u8 *puVar6;
  let mut extraout_DX: u16;
  let mut uVar7: u16;
  let mut extraout_DX_00: u16;
  u32 *puVar8;
  astruct_691 *paVar9;
  let mut uVar10: u32;
  u8 uVar11;
  u8 uVar12;
  let mut uVar13: u16;
  let mut uStack26: u32;
  u8 local_16 [0x2];
  let mut uStack20: u32;
  let mut uStack16: u16;
  u32 *puStack14;
  let mut uStack10: u16;
  u8 *puStack8;
  let mut uStack6: u16;
  let mut uStack4: u16;

  puVar8 = pass1_1008_c6fa(_u16_1050_06e0,0x18);
  uVar5 = ((u32)puVar8 >> 0x10);
  uVar7 = SUB42(puVar8,0x0);
  uStack6 = uVar7;
  uStack4 = uVar5;
  paVar9 = (astruct_691 *)pass1_1028_b4f2(param_4);
  puVar6 = ((u32)paVar9 >> 0x10);
  uVar3 = paVar9;
  uStack10 = uVar3;
  puStack8 = puVar6;
  pass1_1038_4d6e(uVar3,puVar6,paVar9,(u32 *)CONCAT22(uVar5,uVar7));
  puStack14 = (u32 *)CONCAT22(puVar6,uVar3);
  uStack16 = 0x0;
  ppcVar1 = (code **)((int)*puStack14 + 0x10);
  uVar13 = uVar3;
  (**ppcVar1)((int)&u16_1050_1038,uVar3,puVar6);
  uStack20 = CONCAT22(extraout_DX,uVar3);
  uVar10 = pass1_1030_bcae(local_16,&DAT_1050_1050);
  uVar7 = (uVar10 >> 0x10);
  uStack26 = 0x0;
  do {
    if (uStack20 <= uStack26) {//
LAB_1028_77e7:
      if (puStack14 != NULL) {
        ppcVar1 = (code **)*puStack14;
        (**ppcVar1)(0x1030,(int)puStack14,(char)((u32)puStack14 >> 0x10),0x1,uVar13,puVar6,puStack14,puStack14);
      }
      return;
    }
    uVar10 = uStack20;
    pass1_1030_1d58((u32)puStack14);
    uVar5 = uVar10;
    uVar11 = (u8)uVar10;
    uVar12 = (u8)(uVar10 >> 0x8);
    pass1_1028_b58e(param_4);
    puVar4 = local_16;
    paVar2 = (astruct_670 *)CONCAT22(uVar7,CONCAT11(uVar12,uVar11));
    uVar7 = extraout_DX_00;
    pass1_1030_bd74(puVar4,&DAT_1050_1050,CONCAT22(extraout_DX_00,uVar5),paVar2);
    if ((int)puVar4 <= param_3) {
      uStack16 = 0x1;
      goto LAB_1028_77e7;
    }
    uStack26 += 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_780c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar7;
  let mut uVar8: u16;
  u32 *puVar9;
  u32 *puVar10;
  let mut uStack18: u32;
  let mut uStack14: u32;
  u32 *puStack10;

  uVar8 = ((u32)in_EDX >> 0x10);
  puVar9 = pass1_1008_c6fa(_u16_1050_06e0,0x25);
  paVar7 = (astruct_57 *)CONCAT22(uVar8,(int)((u32)puVar9 >> 0x10));
  uVar2 = puVar9;
  uVar8 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4e78(uVar2,paVar7,param_3,puVar9);
  uVar6 = paVar7;
  puStack10 = (u32 *)CONCAT22(uVar6,uVar2);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  uVar3 = uVar2;
  (**ppcVar1)((int)&u16_1050_1038,uVar2,uVar6);
  uStack14 = CONCAT22(uVar6,uVar3);
  uVar5 = (u32)(uVar6 | uVar3);
  if ((uVar6 | uVar3) == 0x0) {
    return;
  }
  uStack18 = 0x0;
  while( true ) {
    uVar3 = uVar5;
    if (uStack14 <= uStack18) break;
    ppcVar1 = (code **)((int)*puStack10 + 0x4);
    uVar5 = uStack14;
    (**ppcVar1)();
    uVar4 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5 & 0xffff | (u32)uVar3 << 0x10);
    uVar8 = 0x1030;
    puVar10 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,uVar4),uVar4,uVar3);
    uVar5 = (u32)puVar10 >> 0x10;
    ppcVar1 = (code **)((int)*puVar10 + 0x24);
    (**ppcVar1)();
    uStack18 += 0x1;
  }
  if (puStack10 != NULL) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(uVar8,uVar2,(char)paVar7,0x1);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_78b8(uchar param_1,i32 param_2,mut param_3: u32)

{
  u8 *puVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  astruct_92 *paVar4;
  u8 *puVar5;
  let mut puVar6: *mut u16;
  let mut puVar7: *mut u16;
  let mut uVar8: u32;
  u8 *puVar9;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut iVar12: i16;
  astruct_57 *paVar13;
  let mut unaff_SI: u16;
  let mut bVar15: bool;
  let mut bVar16: bool;
  let mut puVar17: *mut u16;
  let mut puVar18: *mut u16;
  astruct_27 *paVar19;
  u32 *puVar20;
  let mut in_stack_0000fd48: u16;
  let mut in_stack_0000fd4e: u16;
  let mut in_stack_0000fd52: u16;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000fe76: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000fe7c: u16;
  let mut uVar21: u16;
  let mut uVar22: u16;
  let mut uVar23: u16;
  let mut iVar24: i16;
  let mut uStack340: u16;
  let mut uStack338: u16;
  u32 *puStack74;
  u8 *puStack70;
  let mut uStack68: u16;
  astruct_92 local_42;
  astruct_92 local_30;
  u8 *local_1e [0x3];
  let mut local_18: u32;
  u8 *puStack20;
  let mut uStack18: u16;
  astruct_364 *paStack16;
  u8 *puStack12;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u32;
  astruct_57 *paVar14;

  puVar9 = param_2;
  uVar8 = *_PTR_LOOP_1050_65e2;
  uStack6 = uVar8;
  if (uVar8 == 0x98) {
    pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x4000002);
    puVar9 = param_2;
    paStack16 = (astruct_364 *)(uVar8 & 0xffff | param_2 << 0x10);
    if (*(i32 *)((int)uVar8 + 0x200) == 0x8000002) {
      pass1_1020_a43e(puVar9,(u16 *)CONCAT22(0x1050,&local_18));
      puVar17 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_1e));
      puVar9 = ((u32)puVar17 >> 0x10);
      puVar2 = &local_18;
      pass1_1020_a49a(puVar9,in_stack_0000fd52,CONCAT22(0x1050,puVar2),CONCAT22(0x1050,local_1e),0x7a);
      pass1_1038_4f54(puVar2,(u32)paStack16,0x1);
      if (puVar2 == NULL) {
        pass1_1020_a49a(puVar9,in_stack_0000fd52,CONCAT22(0x1050,&local_18),NULL,0x35);
      }
    }
  }
  if ((0xe < uStack6) && (uStack6 < 0x16)) {
    puVar18 = pass1_1020_a43e(puVar9,(u16 *)CONCAT22(0x1050,local_1e));
    local_18 = uStack6 - 0xf;
    pass1_1020_a54c(((u32)puVar18 >> 0x10),local_1e,&DAT_1050_1050,(i16)local_18);
  }
  paVar13 = (astruct_57 *)((qword)uStack6 % 0x7d);
  paVar14 = paVar13;
  if (paVar13 == NULL) {
    pass1_1008_612e(((qword)uStack6 % 0x7d),0x1,0x64);
    local_1e[0] = paVar13;
    if ((int)local_1e[0] < 0x1a) {
      pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_30),0x1,0x0,0x400);
      do {
        paVar13 = (astruct_57 *)ZEXT24(&local_30);
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,&local_30));
        uVar3 = paVar13;
        uVar10 = paVar14;
        local_18 = (u32)paVar13 & 0xffff | (long)paVar14 << 0x10;
        uVar11 = uVar10 | uVar3;
        paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)uVar11);
        if (uVar11 == 0x0) goto LAB_1028_79d6;
      } while (*(i32 *)(uVar3 + 0x200) == 0x8000002);
      pass1_1038_43cc(uVar3,uVar11,uVar3,uVar10,0x1,0x4);//
LAB_1028_79d6:
      local_30._0_2_ = 0x389a;
      local_30.field2_0x2 = 0x1008;
    }
  }
  puVar5 = paVar13;
  if (uStack6 == 0x5) {
    uVar23 = SUB42(&DAT_1050_1050,0x0);
    uVar22 = SUB42(s_Rebel_1050_4ffc,0x0);
    pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x4000002);
    local_30.field2_0x2 = paVar14;
    local_30._0_2_ = puVar5;
    pass1_1038_4d3c(CONCAT22(local_30.field2_0x2,puVar5),(char *)CONCAT22(uVar23,uVar22),local_30.field2_0x2);
  }
  if (uStack6 == 0x12c) {
    uVar23 = 0x400;
    iVar12 = 0xf;
    uVar22 = 0x1;
    paVar19 = (astruct_27 *)
              mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fd48,in_stack_0000fe6c,
                              in_stack_0000fe72,in_stack_0000fe76);
    paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)paVar19 >> 0x10);
    puVar5 = paVar19;
    local_30.field2_0x2 = ((u32)paVar19 >> 0x10);
    local_30._0_2_ = puVar5;
    pass1_1010_043a(paVar19,CONCAT22(uVar23,uVar22),iVar12);
  }
  if (uStack6 == 0x3d) {
    puVar20 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fd4e,
                              in_stack_0000fe72,in_stack_0000fe78,in_stack_0000fe7c);
    paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)puVar20 >> 0x10);
    local_30._0_2_ = puVar20;
    local_30.field2_0x2 = ((u32)puVar20 >> 0x10);
    local_1e[0] = PTR_LOOP_1050_13ae;
    puVar5 = PTR_LOOP_1050_13ae;
    if (PTR_LOOP_1050_13ae != ((int)&PTR_LOOP_1050_0000 + 0x1)) {
      pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_42),0x1,0x0,0x400);
      while( true ) {
        paVar4 = &local_42;
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
        uVar3 = paVar14;
        local_18 = CONCAT22(uVar3,paVar4);
        paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)(uVar3 | paVar4));
        if ((uVar3 | paVar4) == 0x0) break;
        paStack16 = *(astruct_364 **)&paVar4[0x1b].field6_0x10;
        pass1_1030_34da(paStack16);
      }
      uVar23 = 0x400;
      iVar12 = 0x10;
      uVar22 = 0x1;
      paVar19 = (astruct_27 *)
                mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fd48,in_stack_0000fe6c,
                                in_stack_0000fe72,in_stack_0000fe76);
      paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)paVar19 >> 0x10);
      puVar5 = paVar19;
      uStack18 = ((u32)paVar19 >> 0x10);
      puStack20 = puVar5;
      pass1_1010_043a(paVar19,CONCAT22(uVar23,uVar22),iVar12);
      local_42._0_4_ = &PTR_pass1_1008_377e_1008_389a;
    }
  }
  if (uStack6 == 0x96) {
    pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x4000001);
    puStack74 = (u32 *)CONCAT22((int)paVar14,puVar5);
    uVar21 = (param_3 >> 0x10);
    pass1_1028_780c(param_3,uVar21,CONCAT22((int)paVar14,puVar5));
    if (puVar5 != NULL) {
      uVar23 = 0x400;
      iVar12 = 0x1d;
      uVar22 = 0x1;
      paVar19 = (astruct_27 *)
                mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fd48,in_stack_0000fe6c,
                                in_stack_0000fe72,in_stack_0000fe76);
      paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)paVar19 >> 0x10);
      puVar5 = paVar19;
      uStack68 = ((u32)paVar19 >> 0x10);
      puStack70 = puVar5;
      pass1_1010_043a(paVar19,CONCAT22(uVar23,uVar22),iVar12);
    }
    pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x4000002);
    puStack74 = (u32 *)CONCAT22((int)paVar14,puVar5);
    pass1_1028_780c(param_3,uVar21,CONCAT22((int)paVar14,puVar5));
  }
  puVar20 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fd4e,
                            in_stack_0000fe72,in_stack_0000fe78,in_stack_0000fe7c);
  uStack10 = SUB42(puVar20,0x0);
  uStack8 = ((u32)puVar20 >> 0x10);
  puStack12 = PTR_LOOP_1050_13ae;
  if (0x2 < (int)PTR_LOOP_1050_13ae) {
    puStack74 = mixed_1010_20ba((astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)puVar20 >> 0x10),_u16_1050_0ed0,
                                (u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fd4e,in_stack_0000fe72,
                                in_stack_0000fe78,in_stack_0000fe7c);
    for (puStack70 = 0x1; (int)puStack70 < 0x9; puStack70 = ((int)puStack70 + 0x1)) {
      local_42._0_4_ = (u8 **)(u32)((int)puStack74 + 0x34 + (int)puStack70 * 0x4);
      if (local_42._0_4_ == (u8 **)uStack6) {
        puVar5 = ((int)&PTR_LOOP_1050_0000 + 0x1);
        local_30._0_2_ = 0x1;
        pass1_1008_612e(0x1,0x1,0x64);
        puVar7 = (u16 *)((int)puStack70 - 0x7);
        if (puVar7 == NULL) {
          bVar16 = SBORROW2((int)puVar5,0x32);
          puVar1 = puVar5 + -0x32;
          bVar15 = puVar5 == ((int)s_New_failed_in_Op__Op_1050_0020 + 0x12);//
LAB_1028_7b74:
          if (!bVar15 && bVar16 == (int)puVar1 < 0x0) {
            local_30._0_2_ = NULL;
          }
        }
        else {
          puVar7 = (u16 *)((int)puStack70 - 0x8);
          if (puVar7 == NULL) {
            bVar16 = SBORROW2((int)puVar5,0x19);
            puVar1 = puVar5 + -0x19;
            bVar15 = puVar1 == NULL;
            goto LAB_1028_7b74;
          }
        }
        local_1e[0] = puVar5;
        if (local_30._0_2_ != NULL) {
          pass1_1028_90e6((astruct_97 *)CONCAT22(0x1050,&uStack340),puStack70);
          puVar7 = &uStack340;
          fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,puVar7));
          uStack340 = 0x389a;
          uStack338 = 0x1008;
        }
        pass1_1008_612e(puVar7,0x0,0xa);
        local_18 = local_18 & 0xffff0000 | ZEXT24(puVar7);
        if (puStack70 == 0x7) {
          iVar24 = 0x7;
          puVar6 = puVar7 + 0x37;
          iVar12 = (int)puVar6 >> 0xf;
        }
        else {
          if (puStack70 != 0x8) goto LAB_1028_7ba0;
          iVar24 = 0x8;
          puVar6 = puVar7 + 0x32;
          iVar12 = ((int)puVar7 >> 0xf) + ((u16 *)0xff9b < puVar7);
        }
        uVar21 = (int)((u32)local_42._0_4_ >> 0x10) + iVar12 + CARRY2(local_42._0_2_,puVar6);
        local_42._0_4_ = (u8 **)CONCAT22(uVar21,local_42._0_2_ + (int)puVar6);
        pass1_1010_ebf8((u32)puStack74,local_42._0_2_ + (int)puVar6,uVar21,iVar24);
      }//
LAB_1028_7ba0:
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_7c4e(param_1: *mut astruct_57,mut param_2: u32)

{
  code **ppcVar1;
  u8 *puVar2;
  astruct_92 *paVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  u8 uVar7;
  u32 *puVar8;
  astruct_97 *paVar9;
  let mut in_stack_0000fd32: u16;
  let mut in_stack_0000fe56: u16;
  let mut in_stack_0000fe5c: u16;
  let mut in_stack_0000fe60: u16;
  let mut in_stack_0000fe8a: u16;
  let mut uVar10: u16;
  let mut local_156: u16;
  let mut uStack340: u16;
  let mut uStack70: u16;
  let mut uStack68: u16;
  let mut iStack66: i16;
  astruct_15 *paStack64;
  let mut uStack56: u32;
  let mut uStack52: u16;
  let mut uStack50: u32;
  u32 *puStack46;
  let mut uStack42: u16;
  u8 *puStack40;
  astruct_691 *paStack38;
  astruct_92 local_22;
  let mut iStack10: i16;

  mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fe8a,0x2),in_stack_0000fd32,
                  in_stack_0000fe56,in_stack_0000fe5c,in_stack_0000fe60);
  puVar2 = PTR_LOOP_1050_13ae;
  if (0x2 < (int)PTR_LOOP_1050_13ae) {
    uVar6 = *_PTR_LOOP_1050_65e2;
    iStack10 = (int)(uVar6 >> 0x10);
    if ((0x2 < uVar6) && (uVar6 = CONCAT22(iStack10 - (uVar6 < 0x2),uVar6 - 0x2) % 0x14, uVar6 == 0x0)
       ) {
      pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_22)),0x1,0x0,0x400);
      while( true ) {
        uVar5 = uVar6;
        paVar3 = &local_22;
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
        paStack38 = (astruct_691 *)CONCAT22(uVar5,paVar3);
        uVar6 = (u32)(uVar5 | paVar3);
        if ((uVar5 | paVar3) == 0x0) break;
        if (paVar3[0x1c].field4_0x8 != 0x8000002) {
          puVar8 = pass1_1008_c6fa(_u16_1050_06e0,0x2a);
          uVar6 = (u32)puVar8 >> 0x10;
          uVar5 = puVar8;
          puStack40 = ((u32)puVar8 >> 0x10);
          uVar7 = 0x38;
          uStack42 = uVar5;
          pass1_1038_4d6e(uVar5,puStack40,paStack38,puVar8);
          puStack46 = (u32 *)CONCAT22((int)uVar6,uVar5);
          ppcVar1 = (code **)((int)*puStack46 + 0x10);
          (**ppcVar1)((int)&u16_1050_1038,uVar5,(int)uVar6);
          uStack50 = CONCAT22((int)uVar6,uVar5);
          if (puVar2 == ((int)&u16_1050_0002 + 0x1)) {
            uStack52 = 0x6;
          }
          else {
            uStack52 = 0xc;
          }
          for (uStack56 = 0x0; uStack56 < uStack50; uStack56 += 0x1) {
            paStack64 = (astruct_15 *)pass1_1030_1d7c((int)uStack50,(int)uVar6,(u32)puStack46);
            uVar6 = (u32)paStack64 >> 0x10;
            iVar4 = (int)paStack64;
            pass1_1028_7742(param_2,(param_2 >> 0x10),0x4,paStack64);
            uVar5 = uStack52;
            if (iVar4 == 0x0) {
              uVar5 = 0x19;
            }
            uVar7 = 0x8;
            uStack68 = uVar5;
            iStack66 = iVar4;
            pass1_1008_612e(uVar5,0x1,0x64);
            uStack70 = uVar5;
            if ((int)uVar5 <= (int)uStack68) {
              paVar9 = pass1_1028_8fc0((astruct_97 *)CONCAT13(0x10,CONCAT12(0x50,&local_156)),
                                       (u32)((int)paStack64 + 0x4),(u32)((int)paStack38 + 0x4));
              uVar6 = (u32)paVar9 >> 0x10;
              uVar7 = 0x30;
              fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_156));
              local_156 = 0x389a;
              uStack340 = 0x1008;
            }
          }
          uVar10 = ((u32)puStack46 >> 0x10);
          if (puStack46 != NULL) {
            ppcVar1 = (code **)*puStack46;
            (**ppcVar1)(uVar7,(int)puStack46,uVar10,0x1,(int)puStack46,uVar10,puStack46);
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_7dfc(undefined1 param_1,u8 *param_2,mut param_3: u32)

{
  code **ppcVar1;
  u8 *puVar2;
  astruct_92 *paVar3;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut uVar5: u32;
  astruct_57 *paVar6;
  u8 uVar7;
  u32 *puVar8;
  u32 *puVar9;
  astruct_97 *paVar10;
  let mut in_stack_0000fd30: u16;
  let mut in_stack_0000fe54: u16;
  let mut in_stack_0000fe5a: u16;
  let mut in_stack_0000fe5e: u16;
  let mut in_stack_0000fe88: u16;
  let mut uVar11: u16;
  let mut local_158: u16;
  let mut uStack342: u16;
  let mut uStack72: u16;
  let mut uStack70: u16;
  astruct_15 *paStack68;
  let mut uStack60: u32;
  let mut uStack56: u16;
  let mut uStack54: u16;
  let mut iStack52: i16;
  let mut uStack50: u32;
  u32 *puStack46;
  let mut uStack42: u16;
  u8 *puStack40;
  astruct_691 *paStack38;
  astruct_92 local_22;

  mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_2),_u16_1050_0ed0,
                  (u8 **)CONCAT22(in_stack_0000fe88,0x2),in_stack_0000fd30,in_stack_0000fe54,in_stack_0000fe5a,
                  in_stack_0000fe5e);
  puVar2 = PTR_LOOP_1050_13ae;
  if (((0x2 < (int)PTR_LOOP_1050_13ae) && (0x3 < *_PTR_LOOP_1050_65e2)) &&
     (uVar5 = *_PTR_LOOP_1050_65e2 % 0x14, uVar5 == 0x0)) {
    pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_22)),0x1,0x0,0x400);
    while( true ) {
      paVar3 = &local_22;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
      uVar4 = uVar5;
      paStack38 = (astruct_691 *)CONCAT22(uVar4,paVar3);
      uVar5 = uVar5 & 0xffff0000 | (u32)(uVar4 | paVar3);
      if ((uVar4 | paVar3) == 0x0) break;
      if (paVar3[0x1c].field4_0x8 != 0x8000002) {
        puVar8 = pass1_1008_c6fa(_u16_1050_06e0,0x29);
        paVar6 = (astruct_57 *)(uVar5 & 0xffff0000 | (u32)puVar8 >> 0x10);
        uVar4 = puVar8;
        puStack40 = ((u32)puVar8 >> 0x10);
        uStack42 = uVar4;
        pass1_1038_4d6e(uVar4,puStack40,paStack38,puVar8);
        puStack46 = (u32 *)CONCAT22((int)paVar6,uVar4);
        ppcVar1 = (code **)((int)*puStack46 + 0x10);
        (**ppcVar1)((int)&u16_1050_1038,uVar4,(int)paVar6);
        uStack50 = CONCAT22((int)paVar6,uVar4);
        uVar7 = 0x10;
        puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fe88,0x2),in_stack_0000fd30,
                                 in_stack_0000fe54,in_stack_0000fe5a,in_stack_0000fe5e);
        uVar5 = (u32)paVar6 & 0xffff0000 | (u32)puVar9 >> 0x10;
        uStack56 = SUB42(puVar9,0x0);
        uStack54 = ((u32)puVar9 >> 0x10);
        if (puVar2 == ((int)&u16_1050_0002 + 0x1)) {
          iStack52 = 0x5;
        }
        else {
          iStack52 = 0x1e;
        }
        for (uStack60 = 0x0; uStack60 < uStack50; uStack60 += 0x1) {
          paStack68 = (astruct_15 *)pass1_1030_1d7c((int)uStack50,(int)uVar5,(u32)puStack46);
          uVar5 = uVar5 & 0xffff0000 | (u32)paStack68 >> 0x10;
          uVar4 = paStack68;
          uVar7 = 0x8;
          pass1_1008_612e(uVar4,0x1,0x64);
          uStack70 = uVar4;
          if (((int)uVar4 <= iStack52) &&
             (pass1_1028_7742(param_3,(param_3 >> 0x10),0x4,paStack68), uStack72 = uVar4, uVar4 == 0x0)) {
            paVar10 = pass1_1028_b0de((astruct_97 *)CONCAT13(0x10,CONCAT12(0x50,&local_158)),
                                      (u32)((int)paStack68 + 0x4),(u32)((int)paStack38 + 0x4));
            uVar5 = uVar5 & 0xffff0000 | (u32)paVar10 >> 0x10;
            uVar7 = 0x30;
            fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_158));
            local_158 = 0x389a;
            uStack342 = 0x1008;
          }
        }
        in_stack_0000fe88 = SUB42(puStack46,0x0);
        uVar11 = ((u32)puStack46 >> 0x10);
        if (puStack46 != NULL) {
          ppcVar1 = (code **)*puStack46;
          (**ppcVar1)(uVar7,in_stack_0000fe88,uVar11,0x1,in_stack_0000fe88,uVar11,puStack46);
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_7fb6(uchar param_1,mut param_2: u32)

{
  code **ppcVar1;
  astruct_92 *paVar2;
  let mut uVar3: u16;
  let mut uVar4: u32;
  astruct_57 *paVar5;
  u8 uVar6;
  u32 *puVar7;
  u32 *puVar8;
  astruct_97 *paVar9;
  let mut in_stack_0000fd30: u16;
  let mut in_stack_0000fe54: u16;
  let mut in_stack_0000fe5a: u16;
  let mut in_stack_0000fe5e: u16;
  let mut in_stack_0000fe88: u16;
  let mut uVar10: u16;
  let mut local_158: u16;
  let mut uStack342: u16;
  let mut uStack72: u16;
  let mut uStack68: u16;
  let mut uStack66: u16;
  astruct_15 *paStack64;
  let mut uStack56: u32;
  let mut iStack52: i16;
  u8 *puStack50;
  let mut uStack48: u16;
  let mut uStack46: u16;
  let mut uStack44: u32;
  u32 *puStack40;
  let mut uStack36: u16;
  u8 *puStack34;
  astruct_691 *paStack32;
  astruct_92 local_1c;

  if ((0xb < *_PTR_LOOP_1050_65e2) && (uVar4 = *_PTR_LOOP_1050_65e2 % 0x32, uVar4 == 0x0)) {
    pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_1c)),0x1,0x0,0x400);
    while( true ) {
      paVar2 = &local_1c;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
      uVar3 = uVar4;
      paStack32 = (astruct_691 *)CONCAT22(uVar3,paVar2);
      uVar4 = uVar4 & 0xffff0000 | (u32)(uVar3 | paVar2);
      if ((uVar3 | paVar2) == 0x0) break;
      if (paVar2[0x1c].field4_0x8 != 0x8000002) {
        puVar7 = pass1_1008_c6fa(_u16_1050_06e0,0x11);
        paVar5 = (astruct_57 *)(uVar4 & 0xffff0000 | (u32)puVar7 >> 0x10);
        uVar3 = puVar7;
        puStack34 = ((u32)puVar7 >> 0x10);
        uStack36 = uVar3;
        pass1_1038_4d6e(uVar3,puStack34,paStack32,puVar7);
        puStack40 = (u32 *)CONCAT22((int)paVar5,uVar3);
        ppcVar1 = (code **)((int)*puStack40 + 0x10);
        (**ppcVar1)((int)&u16_1050_1038,uVar3,(int)paVar5);
        uStack44 = CONCAT22((int)paVar5,uVar3);
        uVar6 = 0x10;
        puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fe88,0x2),in_stack_0000fd30,
                                 in_stack_0000fe54,in_stack_0000fe5a,in_stack_0000fe5e);
        uVar4 = (u32)paVar5 & 0xffff0000 | (u32)puVar8 >> 0x10;
        uStack48 = SUB42(puVar8,0x0);
        uStack46 = ((u32)puVar8 >> 0x10);
        puStack50 = PTR_LOOP_1050_13ae;
        if ((int)PTR_LOOP_1050_13ae < 0x3) {
          iStack52 = 0x5;
        }
        else {
          iStack52 = 0x14;
        }
        for (uStack56 = 0x0; uStack56 < uStack44; uStack56 += 0x1) {
          uVar6 = 0x30;
          paStack64 = (astruct_15 *)pass1_1030_1d7c((int)uStack44,(int)uVar4,(u32)puStack40);
          uVar4 = uVar4 & 0xffff0000 | (u32)paStack64 >> 0x10;
          uVar3 = ((int)paStack64 + 0x20);
          uStack66 = uVar3;
          if (((uVar3 != 0x0) && (uVar3 != 0x70)) && (uVar3 != 0x71)) {
            uVar6 = 0x8;
            pass1_1008_612e(uVar3,0x1,0x64);
            uStack68 = uVar3;
            if (((int)uVar3 <= iStack52) &&
               (pass1_1028_7742(param_2,(param_2 >> 0x10),0x4,paStack64), uStack72 = uVar3, uVar3 == 0x0)) {
              paVar9 = pass1_1028_8698((astruct_97 *)CONCAT13(0x10,CONCAT12(0x50,&local_158)),
                                       (u32)((int)paStack64 + 0x4),(u32)((int)paStack32 + 0x4));
              uVar4 = uVar4 & 0xffff0000 | (u32)paVar9 >> 0x10;
              uVar6 = 0x30;
              fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_158));
              local_158 = 0x389a;
              uStack342 = 0x1008;
            }
          }
        }
        in_stack_0000fe88 = SUB42(puStack40,0x0);
        uVar10 = ((u32)puStack40 >> 0x10);
        if (puStack40 != NULL) {
          ppcVar1 = (code **)*puStack40;
          (**ppcVar1)(uVar6,in_stack_0000fe88,uVar10,0x1,in_stack_0000fe88,uVar10,puStack40);
        }
      }
    }
  }
  return;
}
