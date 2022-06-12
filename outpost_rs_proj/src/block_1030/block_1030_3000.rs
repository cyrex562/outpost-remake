
pub fn pass1_1030_3006(mut param_1: u32,mut param_2: u32)

{
  (param_1 + 0x10) = param_2;
  return;
}
pub fn pass1_1030_301a(mut param_1: u16 ,mut param_2: u32,char *param_3)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut iVar4: i16;
  astruct_608 *iVar3;
  let mut uVar5: u16;

  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  if (*(i32 *)(iVar4 + 0x10) != 0x0) {
    uVar1 = (iVar4 + 0x10);
    fn_ptr_1000_17ce(*(char **)(uVar1 + 0x2));
    uVar2 = str_op_1008_60e8(param_1,param_3);
    uVar1 = (iVar4 + 0x10);
    uVar5 = (uVar1 >> 0x10);
    iVar3 = (astruct_608 *)uVar1;
    iVar3.field2_0x2 = uVar2;
    iVar3.field3_0x4 = param_1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_3058(mut param_1: u16 ,param_2: *mut astruct_57,param_3: *mut astruct_375)

{
  let mut puVar1: *mut u16;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  astruct_375 *iVar4;
  astruct_375 *uVar7;
  let mut uVar8: u32;
  let mut uStack4: u16;

  uVar7 = (astruct_375 *)(param_3 >> 0x10);
  iVar4 = (astruct_375 *)param_3;
  if (iVar4.field12_0xc == NULL) {
    mem_op_1000_179c(0x18,param_2);
    uVar5 = param_2;
    uVar6 = uVar5 | param_1;
    param_2 = (astruct_57 *)(param_2 & 0xffff0000 | uVar6);
    if (uVar6 == 0x0) {
      iVar4.field12_0xc = NULL;
    }
    else {
      uVar4 = struct_op_1030_1cd8((astruct_75 *)CONCAT22(uVar5,param_1),0x5,0x5);
      &iVar4.field12_0xc = uVar4;
      (&iVar4.field12_0xc + 0x2) = param_2;
    }
  }
  for (uStack4 = 0x0; uVar3 = iVar4.field13_0x10, puVar1 = (uVar3 + 0x22),
      uStack4 <= *puVar1 && *puVar1 != uStack4; uStack4 += 0x1) {
    uVar8 = pass1_1028_e2e0(param_2,_PTR_LOOP_1050_65e2,0x3);
    param_2 = (astruct_57 *)(param_2 & 0xffff0000 | uVar8 >> 0x10);
    ppcVar2 = (code **)(*iVar4.field12_0xc + 0x8);
    (**ppcVar2)(0x1028,iVar4.field12_0xc,uVar8,(uVar8 >> 0x10),uStack4,0x0);
  }
  return 0x1;
}



StructD * pass1_1030_310a(StructD *param_1,param_2: u8)

{
  pass1_1030_29e6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_314c(mut param_1: u16 ,param_2: *mut astruct_364,mut param_3: u32)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  astruct_364 *pstruct_1;
  let mut unaff_SI: u16;
  let mut uVar2: u16;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut iStack12: i16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar2 = (param_2 >> 0x10);
  pstruct_1 = (astruct_364 *)param_2;
  param_2.field0_0x0 = 0x389a;
  pstruct_1.field1_0x2 = 0x1008;
  pstruct_1.field366_0x170 = 0x0;
  pstruct_1.field409_0x1a4 = param_3;
  pstruct_1.field410_0x1a8 = 0x5;
  pstruct_1.field411_0x1aa = 0x0;
  pstruct_1.field412_0x1ae = 0x10;
  param_2.field0_0x0 = 0x3af2;
  pstruct_1.field1_0x2 = 0x1030;
  pass1_1000_4906((StructD *)(param_2 & 0xffff0000 | ZEXT24(&pstruct_1.field_0x4)),NULL,0x16c);
  pass1_1000_4906((StructD *)(param_2 & 0xffff0000 | ZEXT24(&pstruct_1.field_0x18c)),NULL,0x18);
  pass1_1000_4906((StructD *)(param_2 & 0xffff0000 | ZEXT24(&pstruct_1.field_0x174)),NULL,0xc);
  pass1_1000_4906((StructD *)(param_2 & 0xffff0000 | ZEXT24(&pstruct_1.field_0x180)),NULL,0xc);
  mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fe9a,in_stack_0000ffbe,
                  in_stack_0000ffc4,in_stack_0000ffc8);
  if (PTR_LOOP_1050_13ae < 0x2) {
    pass1_1030_34da(param_2);
  }
  else {
    pstruct_1.field369_0x176 = 0x1;
    pstruct_1.field370_0x178 = 0x2;
    pstruct_1.field371_0x17a = 0x2;
    pstruct_1.field372_0x17c = 0x60001;
    for (iStack12 = 0x1; iStack12 < 0x6; iStack12 += 0x1) {
      (&pstruct_1.field_0x180 + iStack12 * 0x2) = 0x64;
    }
  }
  return;
}
pub fn pass1_1030_3258(mut param_1: u32,mut param_2: u16 )

{
  (param_1 + 0x1ae) = param_2;
  return;
}
pub fn pass1_1030_326a(mut param_1: u32,mut param_2: u16 ,param_3: *mut astruct_692)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  astruct_692 *iVar4;
  let mut uVar4: u16;
  i32 lStack6;

  uVar4 = (param_3 >> 0x10);
  iVar4 = (astruct_692 *)param_3;
  if (iVar4.field426_0x1aa == 0x0) {
    iVar4.field426_0x1aa = 0x1;
  }
  else {
    param_1 = iVar4.field426_0x1aa * 0x2;
    iVar4.field426_0x1aa = param_1;
  }
  uVar1 = param_1;
  pass1_1030_38b8();
  lStack6 = CONCAT22(param_2,uVar1);
  uVar2 = iVar4.field426_0x1aa;
  uVar3 = (&iVar4.field426_0x1aa + 0x2);
  if (lStack6 < (long)uVar2) {
    uVar2 = uVar1;
    uVar3 = param_2;
  }
  &iVar4.field426_0x1aa = uVar2;
  (&iVar4.field426_0x1aa + 0x2) = uVar3;
  pass1_1030_375a(param_3 & 0xffff | uVar4 << 0x10,0x0,uVar2 & 0xffff | uVar3 << 0x10);
  return;
}
pub fn write_to_file_1030_32e4(mut param_1: u32,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut BVar2: bool;
  let mut uVar3: u16;
  HFILE16 in_stack_0000ffd0;
  u32 local_16 [0x2];
  let mut local_c: u16;
  u32 local_a [0x2];

  iVar1 = param_1;
  BVar2 = write_to_file_1008_7e1c
                    (param_2,param_1 & 0xffff0000 | (iVar1 + 0x4),0x16c,in_stack_0000ffd0);
  if (BVar2 != 0x0) {
    BVar2 = write_to_file_1008_7e1c
                      (param_2,param_1 & 0xffff0000 | (iVar1 + 0x174),&DAT_0000_000c,in_stack_0000ffd0);
    if (BVar2 != 0x0) {
      BVar2 = write_to_file_1008_7e1c
                        (param_2,param_1 & 0xffff0000 | (iVar1 + 0x180),&DAT_0000_000c,in_stack_0000ffd0);
      if (BVar2 != 0x0) {
        BVar2 = write_to_file_1008_7e1c
                          (param_2,param_1 & 0xffff0000 | (iVar1 + 0x18c),0x18,in_stack_0000ffd0);
        if (BVar2 != 0x0) {
          uVar3 = (param_1 >> 0x10);
          local_c = (iVar1 + 0x1a8);
          BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffd0);
          if (BVar2 != 0x0) {
            local_16[0] = (iVar1 + 0x1aa);
            BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_16),0x4,in_stack_0000ffd0);
            if (BVar2 != 0x0) {
              local_a[0] = (iVar1 + 0x170);
              BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_a),0x4,in_stack_0000ffd0);
              if (BVar2 != 0x0) {
                local_c = (iVar1 + 0x1ae);
                BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffd0);
                if (BVar2 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
  }
  u16_1050_0310 = 0x6d0;
  return;
}
pub fn read_file_1030_33f0(param_1: *mut astruct_430,HFILE16 *param_2)

{
  astruct_430 *iVar2;
  let mut BVar1: bool;

  iVar2 = (astruct_430 *)param_1;
  iVar2 = (astruct_430 *)&iVar2.field_0x4;
  BVar1 = read_file_1008_7dee(param_2,(param_1 & 0xffff0000 | ZEXT24(iVar2)),0x16c);
  if (((((BVar1 != 0x0) &&
        (BVar1 = read_file_1008_7dee(param_2,(param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x174)),0xc),
        BVar1 != 0x0)) &&
       (BVar1 = read_file_1008_7dee(param_2,(param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x180)),0xc),
       BVar1 != 0x0)) &&
      ((BVar1 = read_file_1008_7dee(param_2,(param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x18c)),0x18),
       BVar1 != 0x0 &&
       (BVar1 = read_file_1008_7dee(param_2,(param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x1a8)),0x2),
       BVar1 != 0x0)))) &&
     (BVar1 = read_file_1008_7dee(param_2,(param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x1aa)),0x4),
     BVar1 != 0x0)) {
    if (u16_1050_0312 < 0x2) {
      return;
    }
    BVar1 = read_file_1008_7dee(param_2,(param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x170)),0x4);
    if ((BVar1 != 0x0) &&
       (BVar1 = read_file_1008_7dee(param_2,(param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x1ae)),0x2),
       BVar1 != 0x0)) {
      return;
    }
  }
  u16_1050_0310 = 0x6d2;
  return;
}
pub fn pass1_1030_34da(param_1: *mut astruct_364)

{
  astruct_364 *iVar1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_364 *)param_1;
  iVar1->field369_0x176 = 0x1;
  iVar1->field370_0x178 = 0x1;
  iVar1->field371_0x17a = 0x1;
  &iVar1->field372_0x17c = 0x1;
  (&iVar1->field372_0x17c + 0x2) = 0x4;
  &iVar1->field_0x182 = 0x32;
  &iVar1->field_0x184 = 0xa;
  &iVar1->field_0x186 = 0xa;
  &iVar1->field_0x188 = 0xa;
  &iVar1->field_0x18a = 0x4b;
  pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | ZEXT24(&iVar1->field_0x18c)),NULL,0x18);
  return;
}
pub fn pass1_1030_3534(mut param_1: u32,mut param_2: u32)

{
  (param_1 + 0x4) = param_2;
  return;
}
pub fn pass1_1030_3548(mut param_1: u32,i32 param_2)

{
  i32 *plVar1;

  plVar1 = (i32 *)(param_1 + 0x4);
  *plVar1 = *plVar1 + param_2;
  return;
}
pub fn pass1_1030_355c(mut param_1: u32,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut iStack4: i16;

  iStack4 = 0x0;
  do {
    iVar1 = iStack4 * 0x4;
    uVar2 = (param_1 >> 0x10);
    *(i32 *)(param_1 + iVar1 + 0x4) = *(i32 *)(iVar1 + param_2) + *(i32 *)(param_1 + 0x4 + iVar1);
    iStack4 += 0x1;
  } while (iStack4 < 0x5b);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_35a4(u8 *param_1,mut param_2: u32,i32 param_3)

{
  let mut puVar1: *mut u16;
  let mut piVar2: *mut i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut iVar7: i16;
  let mut in_register_0000000a: u16;
  StructD *pSVar8;
  let mut uVar9: u16;
  u8 local_c [0x2];
  let mut local_a: u32;
  let mut uStack6: u32;

  pSVar8 = (StructD *)CONCAT22(in_register_0000000a,param_1);
  vsprintf_op_1030_840a(param_1,s_Pop_Leaving__ld_1050_516a);
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar8);
  }
  else {
    pSVar8 = (StructD *)(_PTR_LOOP_1050_5f2c >> 0x10);
  }
  uVar4 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar8);
  uStack6 = CONCAT22(pSVar8,uVar4);
  pass1_1030_3948(param_2,CONCAT22(0x1050,local_c),CONCAT13(0x10,CONCAT12(0x50,&local_a)),0x3);
  uVar6 = (&local_a + 0x2U);
  pass1_1030_3948(param_2,CONCAT22(0x1050,&local_a + 0x2U),CONCAT13(0x10,CONCAT12(0x50,local_c)),
                  0x4);
  do {
    uVar5 = uVar6;
    if (param_3 < 0x1) break;
    pass1_1008_612e(uVar5,local_a,(local_a >> 0x10));
    uVar6 = ZEXT24(&param_3);
    pass1_1030_3a3a(param_2,(i32 *)CONCAT13(0x10,CONCAT12(0x50,&param_3)),uVar5);
    uVar9 = (uStack6 >> 0x10);
    puVar1 = (uVar5 * 0x4 + uStack6);
    uVar3 = *puVar1;
    *puVar1 = *puVar1 + uVar6;
    piVar2 = (uVar5 * 0x4 + uStack6 + 0x2);
    *piVar2 = *piVar2 + pSVar8 + CARRY2(uVar3,uVar6);
    pass1_1030_38f2(param_2,0x3);
    uVar5 = uVar6;
    iVar7 = pSVar8;
    pass1_1030_38f2(param_2,0x4);
  } while ((iVar7 + pSVar8 + CARRY2(uVar5,uVar6) | uVar5 + uVar6) != 0x0);
  pass1_1000_4906((StructD *)(param_2 & 0xffff0000 | (param_2 + 0x18c)),NULL,0x18);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_3694(u8 *param_1,mut param_2: u32,mut param_3: i16,i32 param_4)

{
  let mut puVar1: *mut u16;
  let mut piVar2: *mut i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  StructD *pSVar8;

  pSVar8 = (StructD *)CONCAT22(in_register_0000000a,param_1);
  vsprintf_op_1030_840a(param_1,s_Pop_Leaving__ld_1050_517a);
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar8);
  }
  else {
    pSVar8 = (StructD *)(_PTR_LOOP_1050_5f2c >> 0x10);
  }
  uVar4 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar8);
  uVar7 = SUB42(pSVar8,0x0);
  uVar6 = (param_3 - 0x1U);
  if (((param_3 < 0x1) || (SBORROW2(param_3,0x1))) ||
     (uVar6 = (param_3 - 0x5U), param_3 - 0x5U != 0x0 && 0x3 < (param_3 - 0x1U))) {
    while (uVar5 = uVar6, 0x0 < param_4) {
      pass1_1008_612e(uVar5,0x0,0x5a);
      uVar6 = ZEXT24(&param_4);
      pass1_1030_3a3a(param_2,(i32 *)CONCAT13(0x10,CONCAT12(0x50,&param_4)),uVar5);
      puVar1 = (uVar5 * 0x4 + uVar4);
      uVar3 = *puVar1;
      *puVar1 = *puVar1 + uVar6;
      piVar2 = (uVar5 * 0x4 + uVar4 + 0x2);
      *piVar2 = *piVar2 + pSVar8 + CARRY2(uVar3,uVar6);
    }
  }
  else {
    pass1_1030_39dc(param_2,(i32 *)CONCAT22(0x1050,&param_4),
                    CONCAT13((char)(pSVar8 >> 0x8),CONCAT12((char)pSVar8,uVar4)),param_3);
  }
  pass1_1000_4906((StructD *)(param_2 & 0xffff0000 | (param_2 + 0x18c)),NULL,0x18);
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1030_375a(mut param_1: u32,mut param_2: i16,i32 param_3)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  i32 lVar4;
  i32 lVar5;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut iStack20: i16;
  let mut uStack18: u32;
  let mut local_6: i16;
  let mut local_4: i16;

  iVar6 = param_1;
  if (param_2 == 0x0) {
    local_4 = 0x5a;
    while ((-0x1 < local_4 && (pass1_1030_3a3a(param_1,(i32 *)CONCAT22(0x1050,&param_3),local_4), param_3 != 0x0))) {
      local_4 += -0x1;
    }
  }
  else {
    pass1_1030_3948(param_1,CONCAT22(0x1050,&local_4),CONCAT22(0x1050,&local_6),param_2);
    iVar2 = (local_4 - local_6) + 0x1;
    lVar4 = param_3 / (long)iVar2;
    lVar5 = lVar4 * iVar2;
    uVar3 = lVar5;
    uStack18 = CONCAT22(((param_3 >> 0x10) - (lVar5 >> 0x10)) - (param_3 < uVar3),
                        param_3 - uVar3);
    for (iStack20 = local_6; iStack20 <= local_4; iStack20 += 0x1) {
      iVar7 = iStack20 * 0x4;
      uVar8 = (param_1 >> 0x10);
      *(i32 *)(iVar6 + iVar7 + 0x4) = *(i32 *)(iVar6 + iVar7 + 0x4) - lVar4;
      iVar2 = (iVar6 + iVar7 + 0x6);
      if ((uStack18 | uStack18) != 0x0) {
        iVar1 = (iVar6 + iVar7 + 0x4);
        (iVar6 + iVar7 + 0x4) = iVar1 + -0x1;
        (iVar6 + iVar7 + 0x6) = iVar2 - (iVar1 == 0x0);
        uStack18 += -0x1;
      }
      if ((iVar6 + iStack20 * 0x4 + 0x6) < 0x0) {
        (iVar6 + iStack20 * 0x4 + 0x4) = 0x0;
      }
    }
  }
  pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | (iVar6 + 0x18c)),NULL,0x18);
  return;
}
pub fn pass1_1030_387c(mut param_1: u32)

{
  let mut iStack4: i16;

  iStack4 = 0x5a;
  do {
    (iStack4 * 0x4 + param_1 + 0x4) = (iStack4 * 0x4 + param_1);
    iStack4 += -0x1;
  } while (0x0 < iStack4);
  (param_1 + 0x4) = 0x0;
  return;
}
pub fn pass1_1030_38b8(void)

{
  let mut iStack8: i16;

  iStack8 = 0x0;
  do {
    iStack8 += 0x1;
  } while (iStack8 < 0x5b);
  return;
}
pub fn pass1_1030_38f2(mut param_1: u32,mut param_2: i16)

{
  let mut iStack12: i16;
  let mut local_a: i16;
  let mut local_8: i16;
  let mut uStack6: u32;

  uStack6 = 0x0;
  pass1_1030_3948(param_1,CONCAT22(0x1050,&local_a),CONCAT22(0x1050,&local_8),param_2);
  for (iStack12 = local_8; iStack12 <= local_a; iStack12 += 0x1) {
  }
  return;
}
pub fn pass1_1030_3948(mut param_1: u32,param_2: *mut u16,i16 *param_3,mut param_4: i16)

{
  let mut uVar1: u16;

  if (param_4 == 0x1) {
    *param_3 = 0x0;
    *param_2 = 0x3;
    return;
  }
  uVar1 = (param_1 >> 0x10);
  if (param_4 == 0x2) {
    *param_3 = 0x4;
    *param_2 = (param_1 + 0x1ae);
    return;
  }
  if (param_4 == 0x3) {
    *param_3 = (param_1 + 0x1ae) + 0x1;
    *param_2 = 0x27;
    return;
  }
  if (param_4 != 0x4) {
    if (param_4 == 0x5) {
      *param_3 = 0x4c;
    }
    else {
      *param_3 = 0x0;
    }
    *param_2 = 0x5a;
    return;
  }
  *param_3 = 0x28;
  *param_2 = 0x4b;
  return;
}
pub fn pass1_1030_39dc(mut param_1: u32,i32 *param_2,mut param_3: u32,mut param_4: i16)

{
  let mut iVar1: i16;
  let mut in_DX: u16;
  let mut uVar2: u16;
  let mut iStack8: i16;
  let mut local_6: i16;
  let mut local_4: i16;

  pass1_1030_3948(param_1,CONCAT22(0x1050,&local_6),CONCAT22(0x1050,&local_4),param_4);
  iStack8 = local_6;
  while( true ) {
    if (iStack8 < local_4) {
      return;
    }
    iVar1 = local_4;
    pass1_1030_3a3a(param_1,param_2,iStack8);
    uVar2 = (param_3 >> 0x10);
    (iStack8 * 0x4 + param_3) = iVar1;
    (iStack8 * 0x4 + param_3 + 0x2) = in_DX;
    if (*param_2 == 0x0) break;
    iStack8 += -0x1;
  }
  return;
}
pub fn pass1_1030_3a3a(mut param_1: u32,i32 *param_2,mut param_3: i16)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut iVar8: i16;
  let mut uVar9: u16;

  iVar2 = (param_2 + 0x2);
  uVar9 = (param_1 >> 0x10);
  iVar6 = param_1;
  iVar7 = iVar6 + 0x4;
  iVar8 = param_3 * 0x4;
  piVar1 = (iVar7 + iVar8 + 0x2);
  iVar3 = *piVar1;
  if ((iVar3 < iVar2) ||
     ((uVar5 = *param_2, *piVar1 == iVar2 || iVar3 < iVar2 && ((iVar7 + iVar8) < uVar5)))) {
    *param_2 = *param_2 - *(i32 *)(iVar6 + 0x4 + param_3 * 0x4);
    (iVar6 + param_3 * 0x4 + 0x4) = 0x0;
  }
  else {
    uVar4 = (iVar7 + iVar8);
    iVar3 = (iVar7 + iVar8 + 0x2);
    (iVar6 + iVar8 + 0x4) = uVar4 - uVar5;
    (iVar6 + iVar8 + 0x6) = (iVar3 - iVar2) - (uVar4 < uVar5);
    *param_2 = 0x0;
  }
  return;
}



StructD * pass1_1030_3ac6(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u32 * pass1_1030_3af6(u32 *param_1,mut param_2: u16 ,mut param_3: u16 ,u32 *param_4,mut param_5: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = *param_4;
  (iVar1 + 0x4) = (param_4 + 0x1);
  (iVar1 + 0x6) = param_3;
  (iVar1 + 0x8) = param_2;
  return param_1;
}



u16 pass1_1030_3b28(void)

{
  let mut puVar1: *mut u16;
  let mut puVar2: *mut u32;
  u8 local_8 [0x6];

  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffc4,0x0);
  pass1_1030_3af6(&u16_1050_65e6,0x115,0x15b,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0x19);
  pass1_1030_3af6(&u16_1050_65f0,0x116,0x15c,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffdd,0x32);
  pass1_1030_3af6(&u16_1050_65fa,0x117,0x15d,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0x4b);
  pass1_1030_3af6(&u16_1050_6604,0x118,0x15e,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xf,0x64);
  pass1_1030_3af6(&u16_1050_660e,0x119,0x15f,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x28,0x7d);
  pass1_1030_3af6(&u16_1050_6618,0x11a,0x160,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffec,0x96);
  pass1_1030_3af6(&u16_1050_6622,0x11b,0x161,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x14,0xaf);
  pass1_1030_3af6(&u16_1050_662c,0x11c,0x162,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x1e,0xc8);
  pass1_1030_3af6(&u16_1050_6636,0x11d,0x163,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xfffb,0xe1);
  pass1_1030_3af6(&u16_1050_6640,0x11e,0x164,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x32,0xfa);
  pass1_1030_3af6(&u16_1050_664a,0x11f,0x165,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x1e,0xe1);
  pass1_1030_3af6(&u16_1050_6654,0x120,0x166,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffe7,0xfa);
  pass1_1030_3af6(&u16_1050_665e,0x121,0x167,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0x113);
  pass1_1030_3af6(&u16_1050_6668,0x122,0x168,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x28,0x12c);
  pass1_1030_3af6(&u16_1050_6672,0x123,0x169,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xf,0x145);
  pass1_1030_3af6(&u16_1050_667c,0x124,0x16a,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffec,0x15e);
  pass1_1030_3af6(&u16_1050_6686,0x125,0x16b,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0x0);
  pass1_1030_3af6(&u16_1050_6690,0x126,0x16c,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x2d,0x19);
  pass1_1030_3af6(&u16_1050_669a,0x127,0x16d,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xa,0x32);
  pass1_1030_3af6(&u16_1050_66a4,0x128,0x16e,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffe2,0x4b);
  pass1_1030_3af6(&u16_1050_66ae,0x129,0x16f,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x5,0x64);
  pass1_1030_3af6(&u16_1050_66b8,0x12a,0x170,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x32,0x7d);
  pass1_1030_3af6(&u16_1050_66c2,0x12b,0x171,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffc9,0x96);
  pass1_1030_3af6(&u16_1050_66cc,0x12c,0x172,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xfffb,0xaf);
  pass1_1030_3af6(&u16_1050_66d6,0x12d,0x173,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffe7,0xc8);
  pass1_1030_3af6(&u16_1050_66e0,0x12e,0x174,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x32,0x32);
  pass1_1030_3af6(&u16_1050_66ea,0x12f,0x175,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x3c,0x64);
  pass1_1030_3af6(&u16_1050_66f4,0x130,0x176,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffc4,0xe1);
  pass1_1030_3af6(&u16_1050_66fe,0x131,0x177,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0x19);
  pass1_1030_3af6(&u16_1050_6708,0x132,0x178,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x5,0xaf);
  pass1_1030_3af6(&u16_1050_6712,0x133,0x179,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0x19);
  pass1_1030_3af6(&u16_1050_671c,0x134,0x17a,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x23,0x19);
  pass1_1030_3af6(&u16_1050_6726,0x135,0x17b,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xfffb,0x32);
  pass1_1030_3af6(&u16_1050_6730,0x136,0x17c,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xf,0x32);
  pass1_1030_3af6(&u16_1050_673a,0x137,0x17d,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x2d,0x4b);
  pass1_1030_3af6(&u16_1050_6744,0x138,0x17e,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x1e,0x4b);
  pass1_1030_3af6(&u16_1050_674e,0x139,0x17f,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x2d,0x64);
  pass1_1030_3af6(&u16_1050_6758,0x13a,0x180,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffe7,0x7d);
  pass1_1030_3af6(&u16_1050_6762,0x13b,0x181,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x5,0xaf);
  pass1_1030_3af6(&u16_1050_676c,0x13c,0x182,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0xc8);
  pass1_1030_3af6(&u16_1050_6776,0x13d,0x183,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffce,0xc8);
  pass1_1030_3af6(&u16_1050_6780,0x13e,0x184,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xf,0xfa);
  pass1_1030_3af6(&u16_1050_678a,0x13f,0x185,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x1e,0x113);
  pass1_1030_3af6(&u16_1050_6794,0x140,0x186,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffe2,0x12c);
  pass1_1030_3af6(&u16_1050_679e,0x141,0x187,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x64,0x12c);
  pass1_1030_3af6(&u16_1050_67a8,0x142,0x188,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x32,0x145);
  pass1_1030_3af6(&u16_1050_67b2,0x143,0x189,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x64,0x145);
  pass1_1030_3af6(&u16_1050_67bc,0x144,0x18a,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x1e,0x15e);
  pass1_1030_3af6(&u16_1050_67c6,0x145,0x18b,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffd3,0x15e);
  pass1_1030_3af6(&u16_1050_67d0,0x146,0x18c,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x32,0xfa);
  pass1_1030_3af6(&u16_1050_67da,0x147,0x18d,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xf,0x19);
  pass1_1030_3af6(&u16_1050_67e4,0x148,0x18e,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0x32);
  pass1_1030_3af6(&u16_1050_67ee,0x149,0x18f,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0xaf);
  pass1_1030_3af6(&u16_1050_67f8,0x14a,0x190,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xfffb,0xe1);
  pass1_1030_3af6(&u16_1050_6802,0x14b,0x191,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xa,0x15e);
  pass1_1030_3af6(&u16_1050_680c,0x14c,0x192,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0x19);
  pass1_1030_3af6(&u16_1050_6816,0x14d,0x193,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x1e,0x32);
  pass1_1030_3af6(&u16_1050_6820,0x14e,0x194,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xfffb,0x64);
  pass1_1030_3af6(&u16_1050_682a,0x14f,0x195,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xf,0x64);
  pass1_1030_3af6(&u16_1050_6834,0x150,0x196,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x1e,0x7d);
  pass1_1030_3af6(&u16_1050_683e,0x151,0x197,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffdd,0xe1);
  pass1_1030_3af6(&u16_1050_6848,0x152,0x198,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0x113);
  pass1_1030_3af6(&u16_1050_6852,0x153,0x199,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x2d,0x12c);
  pass1_1030_3af6(&u16_1050_685c,0x154,0x19a,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffe7,0x145);
  pass1_1030_3af6(&u16_1050_6866,0x155,0x19b,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xa,0x15e);
  pass1_1030_3af6(&u16_1050_6870,0x156,0x19c,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0x4b);
  pass1_1030_3af6(&u16_1050_687a,0x157,0x19d,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x5,0x64);
  pass1_1030_3af6(&u16_1050_6884,0x158,0x19e,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0xffec,0x96);
  pass1_1030_3af6(&u16_1050_688e,0x159,0x19f,puVar1,(puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54(CONCAT22(0x1050,local_8),0x0,0x0,0x113);
  puVar2 = pass1_1030_3af6(&u16_1050_6898,0x15a,0x1a0,puVar1,(puVar1 >> 0x10));
  return puVar2;
}
