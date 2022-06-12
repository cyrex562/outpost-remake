
pub fn pass1_1018_30ca(mut param_1: u16 ,param_2: *mut astruct_504,char *param_3)

{
  let mut uVar1: u16;
  astruct_504 *iVar3;
  let mut uVar2: u16;

  uVar2 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_504 *)param_2;
  fn_ptr_1000_17ce(*(char **)&iVar3.field390_0x186);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3.field390_0x186 = uVar1;
  iVar3.field391_0x188 = param_1;
  return;
}
pub fn pass1_1018_30fc(mut param_1: u16 ,mut param_2: u32,u16 **param_3)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar8;
  u32 *puStack18;
  let mut iStack6: i16;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  *param_3 = NULL;
  uVar2 = (u32)((int)param_2 + 0x17e);
  uVar1 = ((int)uVar2 + 0xa);
  if (uVar1 != 0x0) {
    uVar4 = uVar1;
    mem_op_1000_179c(0x6,paVar8);
    uVar7 = paVar8;
    puStack18 = (u32 *)CONCAT22(uVar7,uVar4);
    paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)(uVar7 | uVar4));
    if ((uVar7 | uVar4) == 0x0) {
      *param_3 = NULL;
    }
    else {
      *puStack18 = 0x0;
      (uVar4 + 0x4) = 0x0;
      *param_3 = (u16 *)puStack18;
    }
    uVar5 = uVar1 * 0x2;
    mem_op_1000_179c(uVar5,paVar8);
    puVar3 = *param_3;
    *puVar3 = uVar5;
    ((int)puVar3 + 0x2) = (int)paVar8;
    ((int)*param_3 + 0x4) = uVar1;
    for (iStack6 = 0x0; iStack6 < (int)uVar1; iStack6 += 0x1) {
      iVar6 = iStack6;
      empty_1008_8fc4();
      ((int)(u32)*param_3 + iStack6 * 0x2) = (iVar6 + 0x2e);
    }
  }
  return;
}



u16 pass1_1018_31d0(param_1: *mut astruct_126)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  if ((*(i32 *)((int)param_1 + 0x17e) != 0x0) &&
     (uVar1 = (u32)((int)param_1 + 0x17e), *(i32 *)((int)uVar1 + 0xa) != 0x0)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_31fa(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_126)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut iVar4: i16;
  astruct_126 *pstruct126_5;
  let mut uVar5: u16;

  uVar5 = ((u32)param_3 >> 0x10);
  pstruct126_5 = (astruct_126 *)param_3;
  pass1_1030_82f0(_u16_1050_5748,pstruct126_5.field373_0x17a);
  pstruct126_5.field374_0x17e = param_1;
  pstruct126_5.field375_0x180 = param_2;
  if (((param_2 | pstruct126_5.field374_0x17e) != 0x0) &&
     (uVar2 = (u32)&pstruct126_5.field374_0x17e, iVar4 = ((int)uVar2 + 0xa), iVar4 != 0x0)) {
    pstruct126_5.field369_0x174 = 0x0;
    while( true ) {
      if (iVar4 <= pstruct126_5.field369_0x174) break;
      iVar3 = pstruct126_5.field369_0x174;
      empty_1008_8fc4();
      pass1_1018_2e28(param_3);
      if (pstruct126_5.field370_0x176 == iVar3) break;
      piVar1 = &pstruct126_5.field369_0x174;
      *piVar1 = *piVar1 + 0x1;
    }
    if (iVar4 <= pstruct126_5.field369_0x174) {
      pstruct126_5.field369_0x174 = 0x0;
    }
    pass1_1018_2e28(param_3);
    pstruct126_5.field370_0x176 = iVar4;
  }
  return;
}



u16 * pass1_1018_32a6(param_1: *mut u16,param_2: u8)

{
  param_1 = (u16 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0x20));
  pass1_1018_2c60((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1018_32b2(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1018_2c60(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_331c(u8 *param_1,param_2: *mut astruct_638,mut param_3: u16 ,mut param_4: u16 )

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  let mut unaff_BP: u16;
  u32 *puVar3;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1008_ca5a((astruct_19 *)CONCAT22(param_3,param_2),param_4);
  (u32)&param_2.field271_0x122 = 0x0;
  param_2.field273_0x126 = 0x0;
  param_2.field274_0x12a = 0x0;
  param_2.field275_0x12e = 0x0;
  param_2.field276_0x130 = 0x0;
  param_2.field277_0x132 = 0x0;
  param_2.field278_0x136 = 0x0;
  param_2.field279_0x13a = 0x0;
  param_2.field280_0x13c = 0x0;
  param_2.field281_0x13e = 0x0;
  param_2.field282_0x142 = 0x0;
  CONCAT22(param_3,param_2) = (int)&PTR_LOOP_1050_470c;
  param_2.field2_0x2 = 0x1018;
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  uVar1 = puVar3;
  param_2.field271_0x122 = uVar1;
  param_2.field272_0x124 = (int)((u32)puVar3 >> 0x10);
  param_2.field_0x22 = 0x0;
  pass1_1008_612e(uVar1,0x8,0xc);
  param_2.field280_0x13c = uVar1;
  return;
}
pub fn pass1_1018_33b4(param_1: *mut astruct_455)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_455 *iVar5;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_455 *)param_1;
  param_1.field0_0x0 = &PTR_LOOP_1050_470c;
  iVar5.field1_0x2 = 0x1018;
  puVar1 = (u32 *)iVar5[0x26].field3_0x6;
  uVar2 = (iVar5 + 0x27)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)&iVar5[0x26].field3_0x6 = 0x0;
  fn_ptr_1000_17ce(*(char **)&iVar5[0x24].field3_0x6);
  fn_ptr_1000_17ce(*(char **)&iVar5[0x25].field1_0x2);
  pass1_1008_caa0(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_3424(mut param_1: i16,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar4 = (param_3 >> 0x10);
  iVar3 = (int)param_3;
  uVar1 = (u32)(iVar3 + 0x122);
  pass1_1008_e852(param_2,uVar1,((u32)uVar1 >> 0x10),*(char **)(iVar3 + 0x126));
  uStack6 = CONCAT22(param_2,param_1);
  uVar1 = (u32)(iVar3 + 0x122);
  pass1_1008_e852(param_2,uVar1,((u32)uVar1 >> 0x10),*(char **)(iVar3 + 0x12a));
  uStack10 = CONCAT22(param_2,param_1);
  pass1_1030_8344(_u16_1050_5748,uStack6);
  uVar2 = param_2;
  iVar3 = param_1;
  pass1_1030_8344(_u16_1050_5748,uStack10);
  if (*(i32 *)(iVar3 + 0x200) == *(i32 *)(param_1 + 0x200)) {
    return;
  }
  return;
}
pub fn pass1_1018_34a6(param_1: *mut astruct_679)

{
  pass1_1018_3d6c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn sprintf_op_1018_34b6(uchar param_1,mut param_2: u16 ,param_3: *mut astruct_263)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  char *pcVar3;
  let mut in_register_00000001: u32;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  short in_buf_len_5;
  let mut uVar7: u32;
  i32 lVar8;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;

  in_buf_len_5 = (short)((u32)param_3 >> 0x10);
  iVar6 = (int)param_3;
  uVar7 = switch_1018_3b9e(CONCAT31(in_register_00000001,param_1),param_2,param_3,(iVar6 + 0x12e));
  uVar4 = (u32)param_3 & 0xffff0000 | (u32)(iVar6 + 0x22);
  iVar1 = (iVar6 + 0x12e);
  if (iVar1 == 0x188) {
    lVar8 = pass1_1008_57f0(uVar7,(iVar6 + 0x130));
    uVar5 = ((u32)lVar8 >> 0x10);
    pcVar3 = string_1020_c0d8(((int)lVar8 + 0xe));
    uVar2 = (u32)(iVar6 + 0x132);
    uVar10 = uVar2;
    uVar11 = ((u32)uVar2 >> 0x10);
    uVar9 = SUB42(s__ld__s_1050_4150,0x0);
  }
  else if (iVar1 == 0x18b) {
    lVar8 = pass1_1008_57f0(uVar7,(iVar6 + 0x130));
    uVar2 = (u32)((int)lVar8 + 0x4);
    pcVar3 = (char *)uVar2;
    uVar5 = ((u32)uVar2 >> 0x10);
    uVar2 = (u32)(iVar6 + 0x132);
    uVar10 = uVar2;
    uVar11 = ((u32)uVar2 >> 0x10);
    uVar9 = SUB42(s__ld__s_1050_415e,0x0);
  }
  else {
    if (iVar1 != 0x18c) {
      load_string_1010_84e0
                (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,(char *)(iVar6 + 0x22),
                 in_buf_len_5);
      return;
    }
    lVar8 = pass1_1008_57f0(uVar7,(iVar6 + 0x130));
    uVar2 = (u32)((int)lVar8 + 0x4);
    pcVar3 = (char *)uVar2;
    uVar5 = ((u32)uVar2 >> 0x10);
    uVar2 = (u32)(iVar6 + 0x132);
    uVar10 = uVar2;
    uVar11 = ((u32)uVar2 >> 0x10);
    uVar9 = SUB42(s__ld__s_1050_4157,0x0);
  }
  wsprintf16((WORD *)(iVar6 + 0x22),(char *)CONCAT22(uVar9,in_buf_len_5),(char *)CONCAT22(uVar10,0x1050),uVar11,pcVar3,
             uVar5,lVar8,uVar4);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_str_op_1018_35b0(mut param_1: u16 ,param_2: *mut astruct_263)

{
  i32 *plVar1;
  let mut piVar2: *mut i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  code **ppcVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut extraout_DX: u16;
  astruct_263 *pstruct263_1;
  let mut iVar10: i16;
  astruct_263 *pstruct263_2;
  let mut uVar11: u16;
  let mut bVar12: bool;
  let mut uVar13: u32;
  let mut uVar14: u32;
  let mut local_12: i16;
  let mut local_10: i16;
  i32 lStack14;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  uVar13 = pass1_1030_8326();
  uStack4 = (uVar13 >> 0x10);
  uStack6 = uVar13;
  pstruct263_2 = (astruct_263 *)((u32)param_2 >> 0x10);
  pstruct263_1 = (astruct_263 *)param_2;
  plVar1 = &pstruct263_1[0x1].field15_0x16;
  bVar12 = plVar1 < uStack4;
  if ((bVar12) || ((bVar12 || plVar1 == uStack4 && (pstruct263_1[0x1].field14_0x14 < uStack6)))) {
    uVar3 = pstruct263_1[0x1].field13_0x12;
    if (&pstruct263_1[0x1].field_0x10 < (int)uVar3) {
      uVar14 = switch_1018_3b9e(uVar3,uStack4,param_2,pstruct263_1[0x1].field4_0x4);
      uVar8 = ((u32)uVar14 >> 0x10);
      uVar11 = uVar14;
      uStack10 = uVar11;
      uStack8 = uVar8;
      pass1_1018_427c(param_2,uVar11,uVar8);
      lStack14 = CONCAT22(uVar8,uVar11);
      pass1_1018_3e8c(pstruct263_1,pstruct263_2,(u16 *)CONCAT22(0x1050,&local_12),
                      (u16 *)CONCAT22(0x1050,&local_10));
      if (lStack14 < local_12) {
        local_12 = (int)lStack14;
      }
      uVar4 = pstruct263_1[0x1].field10_0xe;
      puVar7 = (u32 *)(u32)&pstruct263_1[0x1].field9_0xc;
      uVar9 = uVar4 | puVar7;
      if (uVar9 != 0x0) {
        ppcVar6 = (code **)*puVar7;
        (**ppcVar6)(0x30,puVar7,uVar4,0x1);
        uVar9 = extraout_DX;
      }
      pass1_1018_435e(uVar9,(u32)param_2,lStack14,local_12,local_10);
      pstruct263_1[0x1].field9_0xc = puVar7;
      pstruct263_1[0x1].field10_0xe = uVar9;
      piVar2 = &pstruct263_1[0x1].field_0x10;
      *piVar2 = *piVar2 + 0x1;
      uVar14 = (u32)&pstruct263_1[0x1].field9_0xc;
      uVar11 = ((u32)uVar14 >> 0x10);
      iVar10 = (int)uVar14;
      uVar14 = (u32)(iVar10 + 0x4);
      uVar5 = (u32)(iVar10 + 0x8);
      wsprintf16((WORD *)&pstruct263_1.field_0x22,(char *)CONCAT22(0x4165,pstruct263_2),0x50,
                 CONCAT13((char)((u32)uVar5 >> 0x8),CONCAT12((char)uVar5,0x1050)),(char)uVar5,
                 (int)((u32)uVar5 >> 0x10),(int)uVar14,(int)((u32)uVar14 >> 0x10));
      return;
    }
    pstruct263_1[0x1].field14_0x14 = uStack6;
    &pstruct263_1[0x1].field15_0x16 = uStack4;
    &pstruct263_1[0x1].field_0x10 = 0x0;
    pass1_1008_612e(uStack6,0x8,0xc);
    pstruct263_1[0x1].field13_0x12 = uStack6;
  }
  return;
}
pub fn pass1_1018_36e6(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  (iVar1 + 0x12e) = param_4;
  (iVar1 + 0x130) = param_3;
  (iVar1 + 0x132) = param_2;
  (iVar1 + 0x134) = 0x0;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_3710(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_263)

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar7;
  astruct_263 *iVar8;
  let mut uVar8: u16;
  i32 lVar9;
  u8 local_12a [0x118];
  let mut uStack18: u32;
  astruct_203 *paStack14;
  let mut uStack10: u32;
  astruct_203 *paStack6;

  paStack6 = NULL;
  uVar8 = ((u32)param_3 >> 0x10);
  iVar8 = (astruct_263 *)param_3;
  uStack10 = switch_1018_3b9e(param_1,param_2,param_3,iVar8[0x1].field4_0x4);
  uVar4 = iVar8[0x1].field4_0x4 - 0x188;
  uStack18 = (astruct_203 *)(uStack10 & 0xffff0000 | (u32)uVar4);
  switch(uVar4) {
  case 0x0:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar4 = lVar9;
    mem_op_1000_179c(0x10,paVar7);
    uVar6 = paVar7 | uVar4;
    if (uVar6 != 0x0) {
      uVar3 = struct_1018_4790(CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                               (lVar9 + 0xe));
      uStack18 = (astruct_203 *)CONCAT22(uVar6,uVar3);
      paStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x1:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar6 = lVar9;
    uVar4 = uVar6;
    mem_op_1000_179c(0x14,paVar7);
    uVar5 = paVar7 | uVar4;
    if (uVar5 != 0x0) {
      struct_1018_47c8((astruct_203 *)CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                       (uVar6 + 0x12),(u32)(uVar6 + 0xe));
      uStack18 = (astruct_203 *)CONCAT22(uVar5,uVar4);
      paStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x2:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar4 = lVar9;
    mem_op_1000_179c(0x12,paVar7);
    uVar6 = paVar7 | uVar4;
    if (uVar6 != 0x0) {
      pass1_1018_4808((astruct_203 *)CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                      (u32)(lVar9 + 0xe));
      uStack18 = (astruct_203 *)CONCAT22(uVar6,uVar4);
      paStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x3:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar4 = lVar9;
    mem_op_1000_179c(0x14,paVar7);
    if ((paVar7 | uVar4) != 0x0) {
      uStack18 = struct_1018_4842((astruct_203 *)CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                                  (lVar9 + 0xe));
      paStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x4:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar4 = lVar9;
    mem_op_1000_179c(0x10,paVar7);
    if ((paVar7 | uVar4) != 0x0) {
      uStack18 = struct_1018_48b0((astruct_203 *)CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                                  (lVar9 + 0xe));
      paStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x5:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar4 = lVar9;
    mem_op_1000_179c(0x12,paVar7);
    uVar6 = paVar7 | uVar4;
    if (uVar6 != 0x0) {
      uVar3 = struct_1018_4920((astruct_203 *)CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                               (u32)(lVar9 + 0xe));
      uStack18 = (astruct_203 *)CONCAT22(uVar6,uVar3);
      paStack6 = uStack18;
    }
    break;
  default:
    goto switchD_1018_393f_caseD_6;
  }
  uStack18 = NULL;
  paStack6 = uStack18;
switchD_1018_393f_caseD_6:
  uVar1 = iVar8.field274_0x122;
  pass1_1008_e852(((u32)uStack18 >> 0x10),uVar1,((u32)uVar1 >> 0x10),
                  (char *)iVar8.field275_0x126);
  uVar1 = iVar8.field274_0x122;
  paStack14 = uStack18;
  pass1_1008_e852(((u32)uStack18 >> 0x10),uVar1,((u32)uVar1 >> 0x10),*(char **)(iVar8 + 0x1));
  pass1_1038_2a0e((astruct_97 *)CONCAT22(0x1050,local_12a),(u32)&iVar8[0x1].field9_0xc,(u32)paStack6,
                  (u32)uStack18,(u32)paStack14);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_12a));
  (u32)&iVar8[0x1].field9_0xc = 0x0;
  ppcVar2 = (code **)((int)(u32)param_3 + 0x10);
  (**ppcVar2)(0x1030,param_3);
  pass1_1038_2a5c((u16 *)CONCAT22(0x1050,local_12a));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 string_1018_39d8(mut param_1: u32,char *param_2,char *param_3)

{
  let mut iVar1: i16;
  char *pcVar2;
  i32 lVar3;
  char *pcVar4;

  pcVar4 = param_2;
  pcVar2 = load_string_1010_847e(_u16_1050_14cc,0x531);
  iVar1 = pass1_1000_3d7a(pcVar2,pcVar4);
  if (iVar1 != 0x0) {
    iVar1 = pass1_1000_3d7a(param_3,param_2);
    if (iVar1 != 0x0) {
      lVar3 = pass1_1018_4608(param_1,param_2,param_3);
      if ((lVar3 != 0x0) && (((int)lVar3 + 0xc) == 0x1)) {
        return 0x1;
      }
    }
  }
  return 0x0;
}
pub fn pass1_1018_3a42(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  let mut uVar1: u32;

  uVar1 = (u32)((int)param_2 + 0x122);
  pass1_1008_e852(param_1,uVar1,((u32)uVar1 >> 0x10),(char *)param_3);
  return;
}
pub fn pass1_1018_3a5c(mut param_1: u32,char *param_2,char *param_3)

{
  pass1_1008_e320(*(astruct_102 **)((int)param_1 + 0x122),param_2,param_3);
  return;
}



pub fn pass1_1018_3a7a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32) -> u32

{
  let mut uVar1: u32;
  let mut uVar2: u32;

  uVar1 = (u32)((int)param_3 + 0x122);
  uVar2 = string_1008_e586(uVar1,((u32)uVar1 >> 0x10),param_4,param_1,param_2);
  return uVar2;
}
pub fn pass1_1018_3a94(mut param_1: u32,u32 *param_2,u32 *param_3)

{
  pass1_1008_e3ec(*(astruct_218 **)((int)param_1 + 0x122),param_2,param_3);
  return;
}



u16 pass1_1018_3ab2(mut param_1: u32,mut param_2: i16,mut param_3: i16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  i32 lVar4;
  let mut uStack22: u16;
  u8 local_10 [0x8];
  let mut iStack8: i16;
  let mut uStack6: u32;

  if (0x5 < param_3 - 0x188U) {
    return 0x0;
  }
  iVar3 = (int)param_1;
  uVar2 = (param_1 >> 0x10);
  switch(param_3) {
  case 0x188:
    uVar1 = (iVar3 + 0xa);
    uVar2 = (iVar3 + 0xc);
    break;
  case 0x189:
    uVar1 = (iVar3 + 0xe);
    uVar2 = (iVar3 + 0x10);
    break;
  case 0x18a:
    uVar1 = (iVar3 + 0x12);
    uVar2 = (iVar3 + 0x14);
    break;
  case 0x18b:
    uVar1 = (iVar3 + 0x16);
    uVar2 = (iVar3 + 0x18);
    break;
  case 0x18c:
    uVar1 = (iVar3 + 0x1a);
    uVar2 = (iVar3 + 0x1c);
    break;
  case 0x18d:
    uVar1 = (iVar3 + 0x1e);
    uVar2 = (iVar3 + 0x20);
  }
  uStack6 = CONCAT22(uVar2,uVar1);
  iStack8 = 0x0;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_10),uStack6);
  while( true ) {
    lVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_10));
    uVar2 = ((u32)lVar4 >> 0x10);
    if ((lVar4 == 0x0) || (iStack8 == param_2)) break;
    iStack8 += 0x1;
  }
  uStack22 = 0x0;
  if (lVar4 != 0x0) {
    if (((int)lVar4 + 0xa) == 0x0) {
      uStack22 = ((int)lVar4 + 0x8);
    }
    else {
      uStack22 = 0xffff;
    }
  }
  return uStack22;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn switch_1018_3b9e(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_263,mut param_4: u16 ) -> u32

{
  let mut uVar1: u32;
  astruct_263 *pstruct_1_1;
  astruct_263 *pstruct_1_2;
  astruct_6 *paStack14;
  let mut uStack6: u16;
  let mut uStack4: u16;

  uStack6 = 0x0;
  uStack4 = 0x0;
  pstruct_1_2 = (astruct_263 *)((u32)param_3 >> 0x10);
  pstruct_1_1 = (astruct_263 *)param_3;
  uVar1 = pstruct_1_1.field274_0x122;
  pass1_1008_e852(param_2,uVar1,((u32)uVar1 >> 0x10),(char *)pstruct_1_1.field275_0x126);
  pass1_1030_8344(_u16_1050_5748,CONCAT22(param_2,param_1));
  paStack14 = (astruct_6 *)CONCAT22(param_2,param_1);
  switch(param_4) {
  case 0x188:
    if (*(i32 *)&pstruct_1_1.field8_0xa == 0x0) {
      pass1_1008_d3ae((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10));
    }
    uStack6 = pstruct_1_1.field8_0xa;
    uStack4 = pstruct_1_1.field9_0xc;
    break;
  case 0x189:
    if (*(i32 *)&pstruct_1_1.field10_0xe == 0x0) {
      unk_str_op_1008_d4f6((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10),paStack14);
    }
    uStack6 = pstruct_1_1.field10_0xe;
    uStack4 = &pstruct_1_1.field_0x10;
    break;
  case 0x18a:
    if (*(i32 *)&pstruct_1_1.field13_0x12 == 0x0) {
      unk_str_op_1008_d1c6((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10),(u32)paStack14);
    }
    uStack6 = pstruct_1_1.field13_0x12;
    uStack4 = pstruct_1_1.field14_0x14;
    break;
  case 0x18b:
    if (pstruct_1_1.field15_0x16 == 0x0) {
      pass1_1008_cfa0((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10),(u32)paStack14);
    }
    uStack6 = &pstruct_1_1.field15_0x16;
    uStack4 = ((int)&pstruct_1_1.field15_0x16 + 0x2);
    break;
  case 0x18c:
    if (pstruct_1_1.field16_0x1a == 0x0) {
      pass1_1008_cda2((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10),(u32)paStack14);
    }
    uStack6 = &pstruct_1_1.field16_0x1a;
    uStack4 = ((int)&pstruct_1_1.field16_0x1a + 0x2);
    break;
  case 0x18d:
    if (pstruct_1_1.field17_0x1e == 0x0) {
      pass1_1008_cbc4((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10),(u32)paStack14);
    }
    uStack6 = &pstruct_1_1.field17_0x1e;
    uStack4 = ((int)&pstruct_1_1.field17_0x1e + 0x2);
  }
  return CONCAT22(uStack4,uStack6);
}
pub fn pass1_1018_3cda(param_1: *mut astruct_506,char *param_2,char *param_3)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut extraout_DX: u16;
  let mut uVar3: u16;
  astruct_506 *iVar5;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_506 *)param_1;
  ppcVar1 = (code **)((int)(u32)param_1 + 0x10);
  (**ppcVar1)();
  uVar3 = extraout_DX;
  fn_ptr_1000_17ce(*(char **)&iVar5.field294_0x126);
  fn_ptr_1000_17ce(*(char **)&iVar5.field296_0x12a);
  uVar2 = str_op_1008_60e8(uVar3,param_3);
  iVar5.field294_0x126 = uVar2;
  iVar5.field295_0x128 = uVar3;
  uVar2 = str_op_1008_60e8(uVar3,param_2);
  iVar5.field296_0x12a = uVar2;
  iVar5.field297_0x12c = uVar3;
  return;
}
pub fn pass1_1018_3d44(mut param_1: u32,u32 *param_2,u32 *param_3)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_3 = (u32)((int)param_1 + 0x126);
  *param_2 = (u32)((int)param_1 + 0x12a);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_3d6c(param_1: *mut astruct_679)

{
  u8 bVar1;
  let mut uVar2: u16;
  u8 *puVar3;
  let mut uVar4: u16;
  astruct_679 *iVar6;
  astruct_679 *uVar5;
  let mut uVar6: u32;
  let mut uVar7: u32;

  uVar5 = (astruct_679 *)((u32)param_1 >> 0x10);
  iVar6 = (astruct_679 *)param_1;
  uVar4 = iVar6.field322_0x142;
  uVar2 = uVar4 + 0x1e;
  if (iVar6.field323_0x144 + 0x1U == (uVar4 < 0xffe2)) {
    if (uVar2 != 0x3c) {
      if (0x3c < uVar2) {
        return;
      }
      bVar1 = (u8)uVar2;
      if (bVar1 == 0x14) {
        iVar6.field322_0x142 = 0xffec;//
LAB_1018_3e3d:
        iVar6.field323_0x144 = -0x1;
        return;
      }
      if (0x14 < bVar1) {
        if (bVar1 == 0x1e) {
          if ((int)PTR_LOOP_1050_13ae < 0x1) {
            return;
          }
          if (SBORROW2((int)PTR_LOOP_1050_13ae,0x1)) {
            return;
          }
          if (PTR_LOOP_1050_13ae != &u16_1050_0002 && 0x0 < (int)(PTR_LOOP_1050_13ae + -0x1)) {
            puVar3 = PTR_LOOP_1050_13ae + -0x3;
            if (puVar3 == NULL) {
              pass1_1008_612e(0x0,0x1,0x64);
              if ((int)puVar3 < 0x32) {
                uVar4 = 0xa;
              }
              else {
                uVar4 = 0xfff6;
              }
              iVar6.field322_0x142 = uVar4;
              iVar6.field323_0x144 = (int)uVar4 >> 0xf;
              return;
            }
            if (puVar3 != ((int)&PTR_LOOP_1050_0000 + 0x1)) {
              return;
            }
            iVar6.field322_0x142 = 0xfff6;
            goto LAB_1018_3e3d;
          }
          iVar6.field322_0x142 = 0xa;
        }
        else if (bVar1 == 0x28) {
          iVar6.field322_0x142 = 0x14;
        }
        else {
          if (bVar1 != 0x32) {
            return;
          }
          iVar6.field322_0x142 = 0x1e;
        }
        iVar6.field323_0x144 = 0x0;
        return;
      }
      if (bVar1 != 0x0) {
        if (bVar1 != 0xa) {
          return;
        }
        iVar6->field322_0x142 = 0xffe2;
        goto LAB_1018_3e3d;
      }
    }
    uVar7 = 0x5;
    uVar6 = pass1_1030_8326();
    if (uVar6 % uVar7 == 0x0) {
      (u32)&iVar6->field322_0x142 = 0x0;
      return;
    }
  }
  return;
}
pub fn pass1_1018_3e8c(param_1: *mut astruct_263,param_2: *mut astruct_263,param_3: *mut u16,param_4: *mut u16)

{
  *param_4 = 0x1;
  *param_3 = 0x19;
  return;
}
pub fn pass1_1018_3ea4(param_1: *mut astruct_455)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_455 *iVar4;
  astruct_455 *uVar4;

  pass1_1008_cac6(param_1);
  uVar4 = (astruct_455 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  puVar1 = (u32 *)iVar4[0x26].field3_0x6;
  uVar2 = (iVar4 + 0x27)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  (u32)&iVar4[0x26].field3_0x6 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn switch_1018_3ee6(mut param_1: u16 ,mut param_2: u32,i32 param_3,mut param_4: i16,mut param_5: u16 )

{
  let mut iVar1: i16;
  char *pcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar8;
  astruct_203 *paVar9;
  i32 lVar10;
  let mut iVar11: i16;
  astruct_263 *paVar12;
  INT16 IVar13;
  astruct_263 *paVar14;
  let mut uVar15: u16;
  let mut uStack26: u32;
  astruct_203 *paStack22;
  i32 lStack18;
  i32 lStack14;
  let mut iStack10: i16;
  let mut uStack8: u16;
  let mut piStack6: *mut i16;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  switch(param_5) {
  case 0x1:
    iVar1 = param_4 * 0x4 + 0x40b6;
    break;
  default:
    iVar1 = param_4 * 0x4 + 0x40ce;
    break;
  case 0x3:
    iVar1 = param_4 * 0x4 + 0x40e2;
    break;
  case 0x4:
    iVar1 = param_4 * 0x4 + 0x40ee;
    break;
  case 0x8:
    iVar1 = param_4 * 0x4 + 0x40f2;
    break;
  case 0x9:
    iVar1 = param_4 * 0x4 + 0x4106;
    break;
  case 0xa:
    iVar1 = param_4 * 0x4 + 0x410a;
    break;
  case 0x14:
    iVar1 = param_4 * 0x4 + 0x410e;
    break;
  case 0x16:
    iVar1 = param_4 * 0x4 + 0x4112;
    break;
  case 0x17:
    iVar1 = param_4 * 0x4 + 0x4116;
    break;
  case 0x19:
    iVar1 = param_4 * 0x4 + 0x411a;
  }
  piStack6 = CONCAT22(0x1050,iVar1);
  if (piStack6 == NULL) {
    return;
  }
  iStack10 = 0x0;
  uStack8 = 0x0;
  iVar11 = *piStack6;
  paVar12 = (astruct_263 *)param_2;
  paVar14 = (astruct_263 *)(param_2 >> 0x10);
  if (iVar11 == 0x1) {
    uVar15 = pass1_1018_456a(paVar12,paVar14,(iVar1 + 0x2));
    lStack14 = CONCAT22((int)paVar8,uVar15);
    pcVar2 = string_1020_c0d8((iVar1 + 0x2));
    uVar3 = str_op_1008_60e8(paVar8,(char *)CONCAT22(paVar8,pcVar2));
    uVar7 = SUB42(paVar8,0x0);
    uVar15 = uVar3;
    mem_op_1000_179c(0x10,paVar8);
    paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar15);
    if ((paVar8 | uVar15) != 0x0) {
      lVar10 = param_3 / lStack14;
      uStack8 = (param_3 % lStack14);
      struct_1018_4790(paStack22,lVar10,CONCAT22(uVar7,uVar3),(iVar1 + 0x2));
      iStack10 = (int)lVar10;
      goto LAB_1018_425e;
    }
  }
  else if (iVar11 == 0x2) {
    uVar15 = pass1_1018_451e(paVar12,paVar14,(iVar1 + 0x2));
    lStack18 = CONCAT22((int)paVar8,uVar15);
    pcVar2 = string_op_1020_c222((iVar1 + 0x2));
    uVar3 = str_op_1008_60e8(paVar8,(char *)CONCAT22(paVar8,pcVar2));
    uVar7 = SUB42(paVar8,0x0);
    uVar15 = uVar3;
    mem_op_1000_179c(0x10,paVar8);
    paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar15);
    if ((paVar8 | uVar15) != 0x0) {
      paVar9 = struct_1018_48b0(paStack22,param_3 / lStack18,CONCAT22(uVar7,uVar3),(iVar1 + 0x2));
      uStack8 = ((u32)paVar9 >> 0x10);
      iStack10 = (int)paVar9;
      goto LAB_1018_425e;
    }
  }
  else if (iVar11 == 0x3) {
    uVar15 = pass1_1008_c646(_u16_1050_06e0,
                             CONCAT22((iVar1 + 0x2),(int)((u32)_u16_1050_06e0 >> 0x10)));
    if (uVar15 == 0x0) {
      uVar15 = 0x4f;
    }
    uVar3 = switch_1018_43ec(paVar12,paVar14,uVar15);
    lStack14 = CONCAT22((int)paVar8,uVar3);
    uVar3 = pass1_1020_bd80(uVar15);
    uVar4 = str_op_1008_60e8(paVar8,(char *)CONCAT22(paVar8,uVar3));
    uStack26 = CONCAT22((int)paVar8,uVar4);
    mem_op_1000_179c(0x14,paVar8);
    paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar4);
    if ((paVar8 | uVar4) != 0x0) {
      uVar6 = param_3 / lStack14;
      uStack8 = (param_3 % lStack14);
      struct_1018_47c8(paStack22,uVar6,uStack26,uVar15,0x0);
      iStack10 = (int)uVar6;
      goto LAB_1018_425e;
    }
  }
  else {
    if (iVar11 != 0x4) goto LAB_1018_425e;
    iVar1 = (iVar1 + 0x2);
    uVar4 = iVar1 - 0x1;
    iVar11 = (int)_u16_1050_14cc;
    IVar13 = (INT16)((u32)_u16_1050_14cc >> 0x10);
    if (uVar4 == 0x0) {
      load_string_1010_84ac(iVar11,IVar13,0x430);
      uVar7 = SUB42(paVar8,0x0);
      uVar5 = uVar4;
      mem_op_1000_179c(0x14,paVar8);
      paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar5);
      if ((paVar8 | uVar5) != 0x0) {
        uVar15 = 0x2;
        lVar10 = 0x14;//
LAB_1018_4230:
        paVar9 = struct_1018_4842(paStack22,param_3 / lVar10,CONCAT22(uVar7,uVar4),uVar15);
        uStack8 = ((u32)paVar9 >> 0x10);
        iStack10 = (int)paVar9;
        goto LAB_1018_425e;
      }
    }
    else {
      uVar4 = iVar1 - 0x2;
      if (uVar4 == 0x0) {
        load_string_1010_84ac(iVar11,IVar13,0x431);
        uVar7 = SUB42(paVar8,0x0);
        uVar5 = uVar4;
        mem_op_1000_179c(0x14,paVar8);
        paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar5);
        if ((paVar8 | uVar5) != 0x0) {
          uVar15 = 0x3;
          lVar10 = 0x16;
          goto LAB_1018_4230;
        }
      }
      else {
        uVar4 = iVar1 - 0x3;
        if (uVar4 == 0x0) {
          load_string_1010_84ac(iVar11,IVar13,0x432);
          uVar7 = SUB42(paVar8,0x0);
          uVar5 = uVar4;
          mem_op_1000_179c(0x14,paVar8);
          paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar5);
          if ((paVar8 | uVar5) != 0x0) {
            uVar15 = 0x4;
            lVar10 = 0x17;
            goto LAB_1018_4230;
          }
        }
        else {
          uVar4 = iVar1 - 0x4;
          if (uVar4 != 0x0) goto LAB_1018_425e;
          load_string_1010_84ac(iVar11,IVar13,0x433);
          uVar7 = SUB42(paVar8,0x0);
          uVar5 = uVar4;
          mem_op_1000_179c(0x14,paVar8);
          paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar5);
          if ((paVar8 | uVar5) != 0x0) {
            uVar15 = 0x4;
            lVar10 = 0xa;
            goto LAB_1018_4230;
          }
        }
      }
    }
  }
  iStack10 = 0x0;
  uStack8 = 0x0;//
LAB_1018_425e:
  if (*(i32 *)(iStack10 + 0x8) == 0x0) {
    (u32)(iStack10 + 0x8) = 0x1;
  }
  return;
}
