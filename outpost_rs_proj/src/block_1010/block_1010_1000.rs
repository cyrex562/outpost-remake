
pub fn pass1_1010_1146(mut param_1: u32,mut param_2: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  DAT_1050_0ecc = param_2;
  uVar2 = (param_1 >> 0x10);
  uVar1 = (u32)((int)param_1 + 0x66);
  pass1_1000_4aea(((int)param_1 + 0x64),uVar1,(int)((u32)uVar1 >> 0x10),0x4,
                  ((int)s_dibtext_bmp_1050_1844 + 0x6));
  return;
}
pub fn pass1_1010_116c(u32 *param_1,mut param_2: i16,mut param_3: u16 )

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut in_EDX: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uStack4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(i32 *)(iVar3 + 0x56) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x34);
    (**ppcVar1)();
  }
  ppcVar1 = (code **)((int)*param_1 + 0x28);
  iVar2 = (**ppcVar1)();
  if (iVar2 != 0x0) {
    uStack4 = DAT_1050_0ecc;
    iVar2 = DAT_1050_0ecc + 0x1;
    if (iVar2 == 0x0) {
      uStack4 = 0x0;
    }
    pass1_1010_1146((u32)param_1,uStack4);
    pass1_1010_11c6(iVar2,in_EDX,(astruct_234 *)param_1);
    (iVar3 + 0x56) = iVar2;
    (iVar3 + 0x58) = (int)in_EDX;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_11c6(mut param_1: u16 ,param_2: *mut astruct_57,param_3: *mut astruct_234)

{
  let mut piVar1: *mut i16;
  let mut puVar2: *mut u16;
  code **ppcVar3;
  let mut uVar4: u32;
  astruct_239 *iVar6;
  let mut iVar5: i16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  astruct_57 *paVar15;
  let mut uVar16: u32;
  u32 *puVar17;
  astruct_234 *iVar18;
  let mut iVar19: i16;
  let mut iVar21: i16;
  astruct_238 *iVar20;
  let mut uVar22: u16;
  let mut uVar23: u16;
  astruct_223 *paVar24;
  astruct_57 *paVar25;
  u32 *puStack50;
  let mut iStack42: i16;
  let mut iStack40: i16;
  StructD *pSStack38;
  let mut iStack28: i16;
  u32 *puStack26;
  u32 *puStack22;
  let mut uStack14: u32;
  let mut uStack10: u32;

  if (DAT_1050_0ecc == -0x1) {
    return;
  }
  mem_op_1000_179c(0x1a,param_2);
  paVar15 = (astruct_57 *)((u32)param_2 & 0xffff0000);
  if ((param_2 | param_1) == 0x0) {
    iVar6 = NULL;
  }
  else {
    paVar24 = pass1_1010_37d4((astruct_223 *)CONCAT22(param_2,param_1));
    paVar15 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)paVar24 >> 0x10);
    iVar6 = (astruct_239 *)paVar24;
  }
  uVar11 = SUB42(paVar15,0x0);
  uStack10 = 0x10500ece;
  uStack14 = 0x0;
  while( true ) {
    uVar22 = ((u32)param_3 >> 0x10);
    iVar18 = (astruct_234 *)param_3;
    piVar1 = &iVar18.field101_0x68;
    if (*piVar1 == (int)uStack14 || *piVar1 < (int)uStack14) break;
    uVar4 = iVar18.field100_0x64;
    uVar16 = (u32)((int)uVar4 + (int)uStack14 * 0x4);
    puVar17 = (u32 *)((int)uVar16 + DAT_1050_0ecc * 0x8);
    puStack50 = (u32 *)(uVar16 & 0xffff0000 | ZEXT24(puVar17));
    iVar5 = pass1_1000_475e(uStack10,*puVar17);
    if (iVar5 != 0x0) {
      uStack10 = *puStack50;
      uStack14 = uStack14 & 0xffff | (u32)(uStack14 + 0x1) << 0x10;
    }
    uStack14 = uStack14 & 0xffff0000 | (u32)((int)uStack14 + 0x1);
  }
  iVar6.field13_0x10 = uStack14;
  paVar25 = struct_1010_38f8(uStack14,paVar15,(astruct_240 *)CONCAT22(uVar11,iVar6),uStack14);
  paVar15 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)paVar25 >> 0x10);
  iVar7 = 0x0;
  mem_op_1000_179c(0x400,paVar15);
  uVar12 = SUB42(paVar15,0x0);
  iVar5 = iVar7;
  mem_op_1000_179c(0x400,paVar15);
  uVar13 = SUB42(paVar15,0x0);
  pSStack38 = (StructD *)CONCAT22(uVar13,iVar5);
  iStack28 = 0x0;
  pass1_1000_4906((StructD *)CONCAT22(uVar12,iVar7),NULL,0x400);
  pass1_1000_4906((StructD *)CONCAT22(uVar13,iVar5),NULL,0x400);
  iStack42 = 0x0;
  uVar10 = 0x0;
  do {
    puVar2 = &iVar6.field13_0x10;
    if (*puVar2 == uVar10 || (int)*puVar2 < (int)uVar10) {
      return;
    }
    uVar4 = iVar18.field100_0x64;
    uVar23 = ((u32)uVar4 >> 0x10);
    iVar19 = (int)uVar4;
    iVar21 = (iVar19 + iStack28 * 0x4);
    uVar9 = (iVar19 + iStack28 * 0x4 + 0x2);
    paVar25 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)uVar9);
    iVar19 = iVar21 + (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
    puStack22 = (u32 *)CONCAT22(uVar9,iVar19);
    uVar8 = iVar21 + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
    mem_op_1000_179c(0x1a,paVar25);
    uVar14 = paVar25 | uVar8;
    paVar15 = (astruct_57 *)((u32)paVar25 & 0xffff0000 | (u32)uVar14);
    if (uVar14 == 0x0) {
      uVar4 = iVar6.field8_0x8;
      (u32)((int)uVar4 + uVar10 * 0x4) = 0x0;
    }
    else {
      paVar24 = pass1_1010_37d4((astruct_223 *)CONCAT22(paVar25,uVar8));
      paVar15 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)paVar24 >> 0x10);
      uVar4 = iVar6.field8_0x8;
      uVar23 = ((u32)uVar4 >> 0x10);
      iVar21 = (int)uVar4;
      (iVar21 + uVar10 * 0x4) = (int)paVar24;
      (iVar21 + uVar10 * 0x4 + 0x2) = (int)((u32)paVar24 >> 0x10);
    }
    iStack42 += 0x1;
    uVar4 = iVar6.field8_0x8;
    uVar23 = ((u32)uVar4 >> 0x10);
    iVar21 = (int)uVar4;
    uVar4 = (u32)(iVar21 + uVar10 * 0x4);
    ppcVar3 = (code **)((int)(u32)(u32)(iVar21 + uVar10 * 0x4) + 0x1c);
    (**ppcVar3)(0x1000,(int)uVar4,(int)((u32)uVar4 >> 0x10),iStack42,iVar19,uVar9);
    uStack14 = (u32)uVar10;
    while( true ) {
      piVar1 = &iVar18.field101_0x68;
      if (*piVar1 == iStack28 || *piVar1 < iStack28) break;
      iVar19 = iStack28 * 0x4;
      uVar4 = iVar18.field100_0x64;
      uVar4 = (u32)((int)uVar4 + iVar19);
      iVar21 = pass1_1000_475e(*puStack22,(u32)((int)uVar4 + (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
      if (iVar21 != 0x0) break;
      uVar4 = iVar18.field100_0x64;
      uVar23 = ((u32)uVar4 >> 0x10);
      iVar21 = (int)uVar4;
      uVar10 = (iVar21 + iVar19 + 0x2);
      paVar15 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)uVar10);
      uVar9 = (iVar21 + iVar19) + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
      puStack26 = (u32 *)CONCAT22(uVar10,uVar9);
      mem_op_1000_179c(0x1a,paVar15);
      uVar16 = (u32)paVar15 & 0xffff0000;
      if ((paVar15 | uVar9) == 0x0) {
        uVar23 = 0x0;
      }
      else {
        paVar24 = pass1_1010_37d4((astruct_223 *)CONCAT22(paVar15,uVar9));
        uVar16 = uVar16 & 0xffff0000 | (u32)paVar24 >> 0x10;
        uVar23 = SUB42(paVar24,0x0);
      }
      (uStack14 * 0x4 + iVar7) = uVar23;
      (uStack14 * 0x4 + iVar7 + 0x2) = (int)uVar16;
      uVar4 = iVar18.field100_0x64;
      uVar23 = ((u32)uVar4 >> 0x10);
      iVar21 = (int)uVar4;
      uVar10 = (iVar21 + iStack28 * 0x4 + 0x2);
      paVar15 = (astruct_57 *)(uVar16 & 0xffff0000 | (u32)uVar10);
      iStack42 += 0x1;
      uVar4 = (u32)(uStack14 * 0x4 + iVar7);
      ppcVar3 = (code **)((int)(u32)(u32)(uStack14 * 0x4 + iVar7) + 0x1c);
      (**ppcVar3)(0x1000,(int)uVar4,(int)((u32)uVar4 >> 0x10),iStack42,
                  (iVar21 + iStack28 * 0x4) + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8,uVar10);
      iStack40 = 0x0;
      while( true ) {
        piVar1 = &iVar18.field101_0x68;
        if (*piVar1 == iStack28 || *piVar1 < iStack28) break;
        uVar4 = iVar18.field100_0x64;
        uVar4 = (u32)((int)uVar4 + iStack28 * 0x4);
        iVar21 = pass1_1000_475e(*puStack26,(u32)((int)uVar4 + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8));
        if (iVar21 != 0x0) break;
        uVar4 = iVar18.field100_0x64;
        uVar4 = (u32)((int)uVar4 + iStack28 * 0x4);
        uVar10 = pass1_1000_475e(*puStack22,(u32)((int)uVar4 + (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
        if (uVar10 != 0x0) break;
        mem_op_1000_179c(0x1a,paVar15);
        uVar16 = (u32)paVar15 & 0xffff0000;
        if ((paVar15 | uVar10) == 0x0) {
          uVar23 = 0x0;
        }
        else {
          paVar24 = pass1_1010_37d4((astruct_223 *)CONCAT22(paVar15,uVar10));
          uVar16 = uVar16 & 0xffff0000 | (u32)paVar24 >> 0x10;
          uVar23 = SUB42(paVar24,0x0);
        }
        (iStack40 * 0x4 + iVar5) = uVar23;
        (iStack40 * 0x4 + iVar5 + 0x2) = (int)uVar16;
        uVar4 = iVar18.field100_0x64;
        uVar23 = ((u32)uVar4 >> 0x10);
        iVar20 = (astruct_238 *)uVar4;
        uVar10 = (iVar20 + iStack28 * 0x4 + 0x2);
        paVar15 = (astruct_57 *)(uVar16 & 0xffff0000 | (u32)uVar10);
        iStack42 += 0x1;
        uVar4 = (u32)(iStack40 * 0x4 + iVar5);
        ppcVar3 = (code **)((int)(u32)(u32)(iStack40 * 0x4 + iVar5) + 0x1c);
        (**ppcVar3)(0x1000,(int)uVar4,(int)((u32)uVar4 >> 0x10),iStack42,
                    (iVar20 + iStack28 * 0x4) + (DAT_1050_0ecc * 0x6 + 0xebe) * 0x8,uVar10);
        iStack28 += 0x1;
        iStack40 += 0x1;
      }
      uVar4 = (u32)(uStack14 * 0x4 + iVar7);
      ((int)uVar4 + 0x10) = iStack40;
      uVar10 = iStack40 << 0x2;
      iVar21 = iVar5;
      uVar23 = uVar13;
      paVar25 = struct_1010_38f8(uVar10,paVar15,*(astruct_240 **)(uStack14 * 0x4 + iVar7),iStack40);
      paVar15 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)paVar25 >> 0x10);
      pass1_1000_48a8((u32)paVar25,CONCAT22(uVar23,iVar21),uVar10);
      pass1_1000_4906(pSStack38,NULL,0x400);
      uStack14 = uStack14 & 0xffff | (u32)(uStack14 + 0x1) << 0x10;
    }
    uVar4 = iVar6.field8_0x8;
    uVar4 = (u32)((int)uVar4 + (int)uStack14 * 0x4);
    ((int)uVar4 + 0x10) = uStack14;
    uVar10 = uStack14 << 0x2;
    uVar4 = iVar6.field8_0x8;
    iVar21 = iVar7;
    uVar23 = uVar12;
    paVar25 = struct_1010_38f8(uVar10,paVar15,*(astruct_240 **)((int)uVar4 + (int)uStack14 * 0x4),uStack14);
    paVar15 = (astruct_57 *)((u32)paVar15 & 0xffff0000);
    pass1_1000_48a8((u32)paVar25,CONCAT22(uVar23,iVar21),uVar10);
    pass1_1000_4906((StructD *)CONCAT22(uVar12,iVar7),NULL,0x400);
    uVar10 = (int)uStack14 + 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_1656(mut param_1: u16 ,u8 *param_2,mut param_3: u16 ,param_4: *mut astruct_27,mut param_5: u16 ,mut param_6: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  let mut uVar5: u32;
  let mut iVar6: i16;
  let mut uVar7: u16;
  astruct_57 *paVar8;
  astruct_15 *paVar9;
  let mut in_stack_0000fe88: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb6: u16;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  unk_destroy_win_op_1010_305a(param_1,param_4,param_5,param_6);
  if (((int)param_4 + 0x16) == 0x3) {
    paVar8 = (astruct_57 *)
             mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x32),in_stack_0000fe88,
                             in_stack_0000ffac,in_stack_0000ffb2,in_stack_0000ffb6);
    uVar5 = (u32)paVar4 & 0xffff0000;
    uVar1 = (u32)((int)param_4 + 0x32);
    uVar1 = (u32)((int)uVar1 + 0x42);
    uVar7 = ((u32)uVar1 >> 0x10);
    iVar6 = (int)uVar1;
    uVar1 = (u32)(iVar6 + 0x16);
    paVar9 = (astruct_15 *)struct_op_1030_73a8(*(astruct_419 **)((int)uVar1 + 0x4),iVar6,(int)((u32)paVar8 >> 0x10));
    uVar5 = uVar5 & 0xffff0000 | (u32)paVar9 >> 0x10;
    uVar2 = pass1_1010_7818((u32)paVar8,paVar9);
    uVar1 = (u32)(iVar6 + 0x16);
    uVar3 = uVar2;
    ui_op_1010_79aa(paVar8,0x0,*(i32 *)((int)uVar1 + 0x4));
    if (uVar3 == 0x0) {
      uVar1 = (u32)(iVar6 + 0x16);
      unk_win_op_1010_7300(uVar5,paVar8,0x0,uVar2,(u32)((int)uVar1 + 0x4));
    }
  }
  return;
}
pub fn pass1_1010_16ee(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  let mut in_stack_0000ffc0: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_6);
  mem_op_1000_179c(0x4a,paVar2);
  uVar1 = paVar2 | param_5;
  if (uVar1 != 0x0) {
    pass1_1040_c54a((astruct_65 *)CONCAT22(paVar2,param_5),0x0,(u32 *)CONCAT22(param_4,param_3),
                    in_stack_0000ffc0,(u32)paVar2 & 0xffff0000 | (u32)uVar1);
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn string_1010_1722(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut extraout_DX: u16;
  let mut uVar1: u16;
  char *pcVar2;
  let mut uVar3: u16;
  u8 local_52 [0x50];

  pass1_1028_b58e((astruct_15 *)param_4);
  if ((extraout_DX | param_1) == 0x0) {
    pcVar2 = load_string_1010_847e(_u16_1050_14cc,0x424);
    uVar1 = ((u32)pcVar2 >> 0x10);
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_52),pcVar2);
    pcVar2 = (char *)CONCAT22(uVar1,local_52);
    uVar3 = &DAT_1050_1050;
  }
  else {
    pcVar2 = pass1_1038_4d28(*(char **)(param_1 + 0x2e));
    uVar3 = ((u32)pcVar2 >> 0x10);
  }
  str_op_1008_60e8(((u32)pcVar2 >> 0x10),(char *)((u32)pcVar2 & 0xffff | (u32)uVar3 << 0x10));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_1788(char *param_1,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut astruct_15)

{
  let mut uVar1: u16;
  char *pcVar2;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  u32 *puVar4;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffcc: u16;
  u8 *puVar5;
  let mut in_stack_0000fff4: u32;
  u8 **ppuVar6;
  let mut iVar7: i16;

  ppuVar6 = (u8 **)CONCAT22((int)((u32)in_stack_0000fff4 >> 0x10),0x3);
  puVar4 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,ppuVar6,in_stack_0000fe9e
                           ,in_stack_0000ffc2,in_stack_0000ffc8,in_stack_0000ffcc);
  uVar3 = ((u32)puVar4 >> 0x10);
  iVar7 = (int)((u32)ppuVar6 >> 0x10);
  puVar5 = 0x1778;
  uVar1 = pass1_1028_b58e(param_4);
  pcVar2 = pass1_1010_b038(puVar4,uVar1,uVar3,puVar5,iVar7);
  str_op_1008_60e8(uVar3,(char *)CONCAT22(uVar3,pcVar2));
  return;
}
pub fn pass1_1010_17c0(param_1: *mut astruct_455)

{
  u32 *puVar1;
  let mut uVar2: u16;
  astruct_455 *struct_1;
  astruct_455 *struct_1_hi;
  code **fn_ptr_1;

  unk_destroy_win_op_1010_2fa0((astruct_873 *)param_1);
  struct_1_hi = (astruct_455 *)((u32)param_1 >> 0x10);
  struct_1 = (astruct_455 *)param_1;
  puVar1 = (u32 *)struct_1[0xa].field3_0x6;
  uVar2 = (struct_1 + 0xb)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    fn_ptr_1 = (code **)*puVar1;
    (**fn_ptr_1)();
  }
  (u32)&struct_1[0xa].field3_0x6 = 0x0;
  fn_ptr_1000_17ce(*(char **)(struct_1 + 0xc));
  pass1_1000_4906(*(StructD **)&struct_1[0xc].field2_0x4,NULL,(struct_1 + 0xd)->field0_0x0 << 0x2);
  fn_ptr_1000_17ce(*(char **)&struct_1[0xc].field2_0x4);
  (u32)(struct_1 + 0xc) = 0x0;
  (u32)&struct_1[0xc].field2_0x4 = 0x0;
  return;
}
pub fn pass1_1010_184a(u32 *param_1,u32 *param_2)

{
  let mut iVar1: i16;
  let mut iVar2: i16;

  iVar2 = DAT_1050_0ecc;
  iVar1 = (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
  iVar1 = pass1_1000_475e((u32)(iVar1 + (int)*param_1),(u32)(iVar1 + (int)*param_2));
  if (iVar1 == 0x0) {
    iVar1 = (iVar2 * 0x6 + 0xebc) * 0x8;
    iVar1 = pass1_1000_475e((u32)(iVar1 + (int)*param_1),(u32)(iVar1 + (int)*param_2));
    if (iVar1 == 0x0) {
      iVar2 = (iVar2 * 0x6 + 0xebe) * 0x8;
      pass1_1000_475e((u32)(iVar2 + (int)*param_1),(u32)(iVar2 + (int)*param_2));
    }
  }
  return;
}



u16 FUN_1010_18e8(void)

{
  return 0x0;
}



u16 FUN_1010_18ee(void)

{
  return 0x1;
}



u16 * pass1_1010_18f4(param_1: *mut u16,param_2: u8,mut param_3: u16 )

{
  pass1_1010_0f76((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_19 * pass1_1010_195e(param_1: *mut astruct_57,param_2: *mut astruct_19,param_3: *mut astruct_19,mut param_4: u16 )

{
  let mut unaff_BP: u16;
  u32 *puVar1;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  pass1_1010_0f24(param_1,(astruct_19 *)CONCAT22(param_3,param_2),param_4);
  (u32)&param_2[0x1].field_0xe = 0x0;
  CONCAT22(param_3,param_2) = 0x1b2a;
  param_2.segment_0x2 = 0x1010;
  puVar1 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  &param_2[0x1].field_0xe = (int)puVar1;
  param_2[0x1].field8_0x10 = ((u32)puVar1 >> 0x10);
  return (astruct_19 *)CONCAT22(param_3,param_2);
}
pub fn pass1_1010_19a4(mut param_1: u16 ,u32 *param_2)

{
  code **ppcVar1;
  astruct_92 *paVar2;
  let mut extraout_DX: u16;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
  while( true ) {
    paVar2 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
    if ((param_1 | paVar2) == 0x0) break;
    ppcVar1 = (code **)((int)*param_2 + 0x40);
    (**ppcVar1)(0x1028,param_2);
    param_1 = extraout_DX;
  }
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_1a06(mut param_1: u32,param_2: *mut astruct_15,mut param_3: i16,mut param_4: u16 )

{
  char *pcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar2: u16;
  astruct_57 *in_EDX;
  let mut uVar5: u16;
  let mut uVar6: u32;
  u32 *puVar3;
  char *pcVar4;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffee: i16;
  let mut uVar1: u32;

  uVar6._0_2_ = pass1_1028_b58e(param_2);
  uVar5 = (param_1 >> 0x10);
  pcVar1 = pass1_1010_b038(*(u8 **)((int)param_1 + 0x6e),uVar6,in_EDX,0x1770,
                           in_stack_0000ffee);
  iVar2 = pass1_1000_3e2c(CONCAT22((int)in_EDX,pcVar1));
  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22((int)param_2,0x32),in_stack_0000fe8e,
                           in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar2 = ((u32)puVar3 >> 0x10);
  uVar3 = pass1_1010_7818((u32)puVar3,param_2);
  uVar1 = (u32)((int)param_1 + 0x6e);
  pcVar4 = string_op_1010_ada6(uVar2,uVar1,((u32)uVar1 >> 0x10),iVar2,uVar3);
  str_op_1008_60e8(((u32)pcVar4 >> 0x10),pcVar4);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uchar pass1_1010_1a66(mut param_1: u32,param_2: *mut astruct_15)

{
  let mut uVar2: u32;
  uchar uVar3;
  let mut uVar4: u16;
  let mut BVar4: bool;
  astruct_15 *uVar5;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut uVar1: u32;

  uVar5 = (astruct_15 *)((u32)param_2 >> 0x10);
  if ((((int)param_2 + 0x1c) != 0x2) || ((((int)param_2 + 0x1e) & 0xff) != 0x0)) {
    uVar7 = pass1_1028_b58e((astruct_15 *)((u32)param_2 & 0xffff | ZEXT24(uVar5) << 0x10));
    uVar6 = (param_1 >> 0x10);
    uVar1 = (u32)((int)param_1 + 0x6e);
    pass1_1010_c2d8(uVar1,((u32)uVar1 >> 0x10),uVar7);
    if (((int)uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0x0)) {
      uVar2 = (u32)((int)param_1 + 0x6e);
      uVar4 = pass1_1010_b028(uVar2,((u32)uVar2 >> 0x10),param_2);
      BVar4 = pass1_1008_c6ae(_u16_1050_06e0,uVar4,0x5);
      if ((BVar4 == 0x0) && (BVar4 = pass1_1008_c6ae(_u16_1050_06e0,uVar4,0x6), BVar4 == 0x0)) {
        uVar3 = '\0';
      }
      else {
        uVar3 = '\x01';
      }
      return uVar3;
    }
  }
  return '\0';
}



u16 * pass1_1010_1b04(param_1: *mut u16,param_2: u8)

{
  pass1_1010_0f76((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_19 * pass1_1010_1b6e(StructD *param_1,param_2: *mut astruct_19,param_3: *mut astruct_19,mut param_4: u16 )

{
  let mut unaff_BP: u16;
  u32 *puVar1;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  pass1_1010_0f24(param_1,(astruct_19 *)CONCAT22(param_3,param_2),param_4);
  (u32)&param_2[0x1].field_0xe = 0x0;
  CONCAT22(param_3,param_2) = 0x1d04;
  param_2.segment_0x2 = 0x1010;
  puVar1 = mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  &param_2[0x1].field_0xe = (int)puVar1;
  param_2[0x1].field8_0x10 = ((u32)puVar1 >> 0x10);
  return (astruct_19 *)CONCAT22(param_3,param_2);
}
pub fn pass1_1010_1bb4(mut param_1: u16 ,u32 *param_2)

{
  code **ppcVar1;
  astruct_92 *paVar2;
  let mut extraout_DX: u16;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
  while( true ) {
    paVar2 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
    if ((param_1 | paVar2) == 0x0) break;
    ppcVar1 = (code **)((int)*param_2 + 0x40);
    (**ppcVar1)(0x1028,param_2);
    param_1 = extraout_DX;
  }
  return;
}
pub fn pass1_1010_1c16(mut param_1: u32,param_2: *mut astruct_15,mut param_3: i16)

{
  char *pcVar1;
  astruct_15 *uVar3;
  astruct_15 *uVar2;
  let mut uVar4: u32;

  uVar4 = pass1_1028_b58e(param_2);
  uVar3 = (astruct_15 *)((u32)uVar4 >> 0x10);
  uVar2 = uVar3;
  pcVar1 = pass1_1010_b038(*(u8 **)((int)param_1 + 0x6e),uVar4,uVar3,0x178a,param_3);
  str_op_1008_60e8(uVar2,(char *)CONCAT22(uVar2,pcVar1));
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u8 pass1_1010_1c40(mut param_1: u32,param_2: *mut astruct_15)

{
  let mut uVar4: u32;
  let mut uVar3: u16;
  let mut BVar5: bool;
  astruct_15 *uVar5;
  let mut uVar6: u16;
  let mut uVar7: u32;
  u8 uVar2;
  let mut uVar1: u32;

  uVar5 = (astruct_15 *)((u32)param_2 >> 0x10);
  if ((((int)param_2 + 0x1c) != 0x2) || ((((int)param_2 + 0x1e) & 0xff) != 0x0)) {
    uVar7 = pass1_1028_b58e((astruct_15 *)((u32)param_2 & 0xffff | ZEXT24(uVar5) << 0x10));
    uVar6 = (param_1 >> 0x10);
    uVar1 = (u32)((int)param_1 + 0x6e);
    pass1_1010_c2d8(uVar1,((u32)uVar1 >> 0x10),uVar7);
    if (((int)uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0x0)) {
      uVar4 = (u32)((int)param_1 + 0x6e);
      uVar3 = pass1_1010_b028(uVar4,((u32)uVar4 >> 0x10),param_2);
      BVar5 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x11);
      if ((BVar5 == 0x0) && (BVar5 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x12), BVar5 == 0x0)) {
        uVar2 = '\0';
      }
      else {
        uVar2 = '\x01';
      }
      return uVar2;
    }
  }
  return '\0';
}



u16 * pass1_1010_1cde(param_1: *mut u16,param_2: u8)

{
  pass1_1010_0f76((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_19 * struct_op_1010_1d48(param_1: *mut astruct_19,mut param_2: u16 )

{
  astruct_19 *iVar1;
  astruct_19 *uVar1;

  uVar1 = (astruct_19 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_19 *)param_1;
  param_1.offset_0x0 = 0x389a;
  iVar1.segment_0x2 = 0x1008;
  iVar1.field2_0x4 = 0x0;
  iVar1.field3_0x8 = param_2;
  param_1.offset_0x0 = 0x2014;
  iVar1.segment_0x2 = 0x1010;
  return param_1;
}
pub fn pass1_1010_1d80(StructD *param_1)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_455 *iVar4;
  astruct_455 *uVar4;

  uVar4 = (astruct_455 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1.address_offset_field_0x0 = 0x2014;
  iVar4.field1_0x2 = 0x1010;
  pass1_1010_1f62((astruct_27 *)((u32)param_1 & 0xffff | ZEXT24(uVar4) << 0x10),0x1);
  puVar1 = iVar4.field2_0x4;
  uVar2 = iVar4.field3_0x6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  param_1.address_offset_field_0x0 = 0x389a;
  iVar4.field1_0x2 = 0x1008;
  return;
}



u16 pass1_1010_1dce(void)

{
  return 0x0;
}



u16 pass1_1010_1dd4(void)

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_1dda(mut param_1: u32)

{
  pass1_1010_209e(_u16_1050_0ed0,((int)param_1 + 0x8));
  return;
}
pub fn pass1_1010_1df2(param_1: *mut astruct_242,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,param_5: *mut astruct_57)

{
  code **ppcVar1;
  astruct_241 *in_AX;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_242 *iVar3;
  let mut uVar4: u16;
  let mut puStack10: *mut u16;
  let mut puStack6: *mut u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_242 *)param_1;
  if (iVar3.field4_0x4 == NULL) {
    mem_op_1000_179c(0xc,param_5);
    uVar2 = param_5;
    uVar3 = uVar2 | in_AX;
    param_5 = (astruct_57 *)((u32)param_5 & 0xffff0000 | (u32)uVar3);
    if (uVar3 == 0x0) {
      iVar3.field4_0x4 = NULL;
    }
    else {
      set_struct_1008_574a((astruct_57 *)CONCAT22(uVar2,in_AX));
      *(astruct_241 **)&iVar3.field4_0x4 = in_AX;
      ((int)&iVar3.field4_0x4 + 0x2) = (int)param_5;
    }
  }
  mem_op_1000_179c(0xa,param_5);
  uVar2 = param_5;
  puStack10 = (u16 *)CONCAT22(uVar2,in_AX);
  if ((uVar2 | in_AX) == 0x0) {
    puStack6 = NULL;
  }
  else {
    *puStack10 = 0x389a;
    in_AX.field2_0x2 = 0x1008;
    in_AX.field3_0x4 = param_3;
    in_AX.field4_0x8 = param_2;
    *puStack10 = 0x2010;
    in_AX.field2_0x2 = 0x1010;
    puStack6 = puStack10;
  }
  ppcVar1 = (code **)((int)*iVar3.field4_0x4 + 0x4);
  (**ppcVar1)(0x1000,iVar3.field4_0x4,puStack6);
  return;
}
pub fn pass1_1010_1ea6(mut param_1: u32,StructD *param_2)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  u32 *puVar4;
  u8 *puVar5;
  let mut extraout_DX: u16;
  astruct_498 *iVar6;
  let mut uVar6: u16;
  u8 local_c [0x4];
  let mut uStack8: u32;
  let mut uStack4: u16;

  uVar6 = (param_1 >> 0x10);
  iVar6 = (astruct_498 *)param_1;
  if (iVar6.field4_0x4 == NULL) {
    return;
  }
  uStack4 = 0x0;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_c),(u32)iVar6.field4_0x4);
  while( true ) {
    puVar5 = local_c;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar5));
    if ((extraout_DX | puVar5) == 0x0) break;
    if (*(StructD **)(puVar5 + 0x4) == param_2) {
      uStack4 = 0x1;
      ppcVar3 = (code **)((int)*iVar6.field4_0x4 + 0xc);
      (**ppcVar3)(0x1008);
      uStack8 = 0x0;
    }
  }
  puVar4 = iVar6.field4_0x4;
  if (((int)puVar4 + 0x8) == 0x0) {
    // WARNING: Load size is inaccurate
    puVar1 = iVar6.field4_0x4;
    uVar2 = ((int)&iVar6.field4_0x4 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)(0x1008,puVar1,uVar2,0x1,puVar1,uVar2,puVar1,uVar2);
    }
    iVar6.field4_0x4 = NULL;
  }
  return;
}
pub fn pass1_1010_1f62(param_1: *mut astruct_27,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  i32 lVar4;
  u8 local_a [0x8];
  code **fn_ptr_1;

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x4));
  while( true ) {
    lVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    uVar3 = ((u32)lVar4 >> 0x10);
    iVar2 = (int)lVar4;
    if (lVar4 == 0x0) break;
    if ((((iVar2 + 0x8) == 0x0) || (param_2 == 0x0)) || ((iVar2 + 0x8) == param_2)) {
      uVar1 = (u32)(iVar2 + 0x4);
      fn_ptr_1 = (code **)((int)(u32)(u32)(iVar2 + 0x4) + 0x4);
      (**fn_ptr_1)(0x1008,(int)uVar1,(int)((u32)uVar1 >> 0x10),param_2);
    }
  }
  return;
}



u16 * pass1_1010_1fbe(param_1: *mut u16,param_2: u8)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1010_1fea(param_1: *mut u16,param_2: u8)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}

