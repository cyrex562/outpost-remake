
pub fn file_1038_6118(mut param_1: i16,param_2: *mut astruct_57,param_3: *mut astruct_373,HFILE16 *param_4)

{
  u32 *puVar1;
  let mut BVar2: bool;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  u8 *puVar6;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar8;
  astruct_373 *iVar9;
  astruct_373 *uVar10;
  let mut uVar9: u16;
  let mut uVar11: u16;
  u8 *puStack1046;
  let mut uStack1042: u16;
  u8 local_408 [0x400];
  let mut local_8: u16;
  let mut local_6: u32;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  file_1030_1730(param_3,param_4);
  if (param_1 == 0x0) {
    return;
  }
  local_6 = 0x0;
  puVar1 = &local_6;
  file_1008_7548(param_4,(i32 *)CONCAT22(0x1050,puVar1),paVar8);
  if (puVar1 != NULL) {
    uVar10 = (astruct_373 *)((u32)param_3 >> 0x10);
    iVar9 = (astruct_373 *)param_3;
    (u32)&iVar9->field_0xc = local_6;
    BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9->field13_0x10)),0x4);
    if (((((BVar2 != 0x0) &&
          (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9->field_0x18)),0x2),
          BVar2 != 0x0)) &&
         (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9->field19_0x1a)),0x2),
         BVar2 != 0x0)) &&
        ((BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_8),0x2), BVar2 != 0x0 &&
         (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0x2)),0x4),
         BVar2 != 0x0)))) &&
       (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 |
                                                  ZEXT24(((int)&iVar9[0x1].field4_0x4 + 0x2))),0x2),
       BVar2 != 0x0)) {
      (iVar9 + 0x1) = local_8;
      BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0x8)),0x2);
      if ((BVar2 != 0x0) &&
         (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0xa)),0x94),
         BVar2 != 0x0)) {
        if ((int)u16_1050_0312 < 0x2) {
          uVar9 = 0x54;
          uVar11 = 0x0;
          mem_op_1000_179c(0x54,paVar8);
          uVar7 = SUB42(paVar8,0x0);
          puStack1046 = CONCAT22(uVar7,BVar2);
          BVar3 = read_file_1008_7dee(param_4,CONCAT22(uVar7,BVar2),CONCAT22(uVar11,uVar9));
          if (BVar3 == 0x0) {//
LAB_1038_626a:
            u16_1050_0310 = 0x6d2;
            fn_ptr_1000_17ce((char *)puStack1046);
            return;
          }
          uStack1042 = 0x0;
          do {
            uVar4 = switch_1008_72bc(param_4,uStack1042);
            uVar9 = (uStack1042 * 0x4 + BVar2 + 0x2);
            (&iVar9[0xb].field19_0x1a)[uVar4 * 0x2] = *(astruct_430 **)(uStack1042 * 0x4 + BVar2);
            (&iVar9[0xc].field_0x0 + uVar4 * 0x4) = uVar9;
            uStack1042 += 0x1;
          } while ((int)uStack1042 < 0x15);
          BVar3 = read_file_1008_7dee(param_4,puStack1046,0x54);
          if (BVar3 == 0x0) goto LAB_1038_626a;
          uStack1042 = 0x0;
          do {
            uVar5 = switch_1008_72bc(param_4,uStack1042);
            uVar4 = (uStack1042 * 0x4 + BVar2 + 0x2);
            (&iVar9[0xe].field19_0x1a)[uVar5 * 0x2] = *(astruct_430 **)(uStack1042 * 0x4 + BVar2);
            (&iVar9[0xf].field_0x0 + uVar5 * 0x4) = uVar4;
            uStack1042 += 0x1;
          } while ((int)uStack1042 < 0x15);
          fn_ptr_1000_17ce((char *)puStack1046);
        }
        else {
          BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0xb].field19_0x1a)),
                                      0x54);
          uVar4 = paVar8;
          if (BVar2 == 0x0) {
            u16_1050_0310 = 0x6d2;
            return;
          }
          BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0xe].field19_0x1a)),
                                      0x54);
          if (BVar2 == 0x0) {
            u16_1050_0310 = 0x6d2;
            return;
          }
        }
    // WARNING: Load size is inaccurate
        read_file_1030_33f0(iVar9[0x11].field19_0x1a,param_4);
        puVar6 = local_408;
        read_file_1008_7c6e((HFILE16)param_4,((u32)param_4 >> 0x10),(char *)CONCAT22(0x1050,puVar6));
        if (puVar6 != NULL) {
          uVar5 = str_op_1008_60e8(uVar4,(char *)CONCAT22(0x1050,local_408));
          &iVar9[0x12].field_0x2 = uVar5;
          &iVar9[0x12].field4_0x4 = uVar4;
          BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 |
                                                    ZEXT24(((int)&iVar9[0x12].field4_0x4 + 0x2))),0x2);
          if (((((BVar2 != 0x0) &&
                (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 |
                                                           (u32)CONCAT11((char)((u32)param_3 >> 0x8) + '\x02',
                                                                           (char)param_3)),0x4), BVar2 != 0x0)) &&
               (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0x12].field_0xc))
                                            ,0x2), BVar2 != 0x0)) &&
              (((BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0x12].field_0xe)
                                                           ),0x2), BVar2 != 0x0 &&
                (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 |
                                                           ZEXT24(&iVar9[0x12].field13_0x10)),0x2), BVar2 != 0x0)) &&
               ((BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 |
                                                           ZEXT24(((int)&iVar9[0x12].field13_0x10 + 0x2))),
                                             0x2), BVar2 != 0x0 &&
                ((BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 |
                                                            ZEXT24(&iVar9[0x12].field_0x14)),0x2), BVar2 != 0x0 &&
                 (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 |
                                                            ZEXT24(&iVar9[0x12].field16_0x16)),0x2), BVar2 != 0x0)))))))
              ) && (((int)u16_1050_0312 < 0x2 ||
                    ((BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(iVar9 + 0x13)),0x2
                                                 ), BVar2 != 0x0 &&
                     (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 |
                                                                ZEXT24(&iVar9[0x13].field_0x2)),0x4), BVar2 != 0x0))))))
          {
            return;
          }
          u16_1050_0310 = 0x6d0;
          return;
        }
      }
    }
  }
  u16_1050_0310 = 0x6d2;
  return;
}



StructD * pass1_1038_64de(StructD *param_1,param_2: u8)

{
  pass1_1038_33f8(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn struct_1038_6520(param_1: *mut astruct_308)

{
  astruct_308 *pstruct308_1;
  astruct_308 *pstruct308_2;

  pstruct308_2 = (astruct_308 *)((u32)param_1 >> 0x10);
  pstruct308_1 = (astruct_308 *)param_1;
  param_1->field0_0x0 = 0x389a;
  pstruct308_1->field1_0x2 = 0x1008;
  pstruct308_1->field2_0x4 = 0x0;
  pstruct308_1->field3_0x8 = 0x0;
  pstruct308_1->field4_0xc = 0x0;
  pstruct308_1->field5_0xe = 0x0;
  pstruct308_1->field6_0x12 = 0x0;
  pstruct308_1->field7_0x14 = 0x0;
  pstruct308_1->field8_0x16 = 0x0;
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&pstruct308_1->field9_0x1a)));
  pstruct308_1->field12_0x20 = 0x0;
  pstruct308_1->field13_0x24 = 0x0;
  pstruct308_1->field14_0x26 = 0x0;
  pstruct308_1->field15_0x28 = 0x0;
  param_1->field0_0x0 = 0x78de;
    // just 0x1038
  pstruct308_1->field1_0x2 = &u16_1050_1038;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6590(param_1: *mut astruct_410,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut iVar4: i16;
  astruct_410 *iVar3;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut uVar7: u32;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_410 *)param_1;
  param_1 = 0x389a;
  iVar3->field2_0x2 = 0x1008;
  (u32)&iVar3->field3_0x4 = 0x0;
  iVar3->field5_0x8 = param_6;
  iVar3->field6_0xc = param_4;
  iVar3->field7_0xe = 0x0;
  iVar3->field8_0x12 = 0x0;
  iVar3->field9_0x14 = 0x0;
  iVar3->field10_0x16 = param_2;
  iVar3->field11_0x18 = param_3;
  puVar6 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x1a)));
  uVar2 = ((u32)puVar6 >> 0x10);
  (u32)&iVar3->field18_0x20 = 0x0;
  iVar3->field20_0x24 = 0x0;
  iVar3->field21_0x26 = param_5;
  iVar3->field22_0x28 = 0x0;
  param_1 = 0x78de;
  iVar3->field2_0x2 = (int)&u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6);
  uVar7 = pass1_1030_6d4e(param_5,uVar2,CONCAT22(uVar2,param_5));
  iVar4 = (int)(uVar7 >> 0x10);
  iVar3->field3_0x4 = (astruct_411 *)uVar7;
  iVar3->field4_0x6 = iVar4;
  puVar1 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x1a));
  pass1_1008_3f62(puVar1,(u16 *)CONCAT22(uVar2,param_5 + 0xc));
    // WARNING: Load size is inaccurate
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3->field3_0x4);
  iVar3->field18_0x20 = uVar2;
  iVar3->field19_0x22 = iVar4;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_666e(param_1: *mut astruct_420,i32 *param_2,mut param_3: u16 ,mut param_4: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_420 *iVar3;
  astruct_420 *uVar4;
  let mut puVar4: *mut u16;
  let mut uVar5: u32;

  uVar4 = (astruct_420 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_420 *)param_1;
  param_1 = 0x389a;
  iVar3->field2_0x2 = 0x1008;
  iVar3->field3_0x4 = NULL;
  iVar3->field4_0x8 = param_4;
  iVar3->field5_0xc = 0x0;
  iVar3->field6_0xe = param_2;
  iVar3->field7_0x12 = 0x0;
  iVar3->field8_0x14 = 0x0;
  iVar3->field10_0x18 = 0x0;
  iVar3->field9_0x16 = 0x0;
  puVar4 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field11_0x1a)));
  uVar2 = ((u32)puVar4 >> 0x10);
  (u32)&iVar3->field16_0x20 = 0x0;
  iVar3->field18_0x24 = 0x0;
  iVar3->field19_0x26 = param_3;
  iVar3->field20_0x28 = 0x0;
  param_1 = 0x78de;
  iVar3->field2_0x2 = (int)&u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar5 = pass1_1030_6d4e(param_3,uVar2,CONCAT22(uVar2,param_3));
  uVar3 = (uVar5 >> 0x10);
  &iVar3->field3_0x4 = (int)uVar5;
  ((int)&iVar3->field3_0x4 + 0x2) = uVar3;
  puVar1 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field11_0x1a));
  pass1_1008_3f62(puVar1,(u16 *)CONCAT22(uVar2,param_3 + 0xc));
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3->field3_0x4);
  iVar3->field16_0x20 = uVar2;
  iVar3->field17_0x22 = uVar3;
  pass1_1020_ba94(param_2);
  iVar3->field9_0x16 = uVar2;
  iVar3->field10_0x18 = uVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_675c(param_1: *mut astruct_414,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_414 *iVar3;
  astruct_414 *uVar4;
  let mut puVar4: *mut u16;
  let mut uVar5: u32;

  uVar4 = (astruct_414 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_414 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  iVar3->field2_0x4 = NULL;
  iVar3->field3_0x8 = param_5;
  iVar3->field4_0xc = 0x0;
  iVar3->field5_0xe = 0x0;
  iVar3->field6_0x12 = param_3;
  iVar3->field7_0x14 = 0x0;
  iVar3->field8_0x16 = param_2;
  puVar4 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field9_0x1a)));
  uVar2 = ((u32)puVar4 >> 0x10);
  (u32)&iVar3->field14_0x20 = 0x0;
  iVar3->field16_0x24 = 0x0;
  iVar3->field17_0x26 = param_4;
  iVar3->field18_0x28 = 0x0;
  param_1->field0_0x0 = 0x78de;
  iVar3->field1_0x2 = (int)&u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  uVar5 = pass1_1030_6d4e(param_4,uVar2,CONCAT22(uVar2,param_4));
  uVar3 = (uVar5 >> 0x10);
  &iVar3->field2_0x4 = (int)uVar5;
  ((int)&iVar3->field2_0x4 + 0x2) = uVar3;
  puVar1 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field9_0x1a));
  pass1_1008_3f62(puVar1,(u16 *)CONCAT22(uVar2,param_4 + 0xc));
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3->field2_0x4);
  iVar3->field14_0x20 = uVar2;
  iVar3->field15_0x22 = uVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6838(param_1: *mut astruct_415,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_415 *iVar3;
  astruct_415 *uVar4;
  let mut puVar4: *mut u16;
  let mut uVar5: u32;

  uVar4 = (astruct_415 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_415 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  iVar3->field2_0x4 = NULL;
  iVar3->field3_0x8 = param_5;
  iVar3->field4_0xc = 0x0;
  iVar3->field5_0xe = 0x0;
  iVar3->field6_0x12 = 0x0;
  iVar3->field7_0x14 = param_3;
  iVar3->field8_0x16 = param_2;
  puVar4 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field9_0x1a)));
  uVar2 = ((u32)puVar4 >> 0x10);
  (u32)&iVar3->field14_0x20 = 0x0;
  iVar3->field16_0x24 = 0x0;
  iVar3->field17_0x26 = param_4;
  iVar3->field18_0x28 = 0x0;
  param_1->field0_0x0 = 0x78de;
    // 0x1038
  iVar3->field1_0x2 = (int)&u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  uVar5 = pass1_1030_6d4e(param_4,uVar2,CONCAT22(uVar2,param_4));
  uVar3 = (uVar5 >> 0x10);
  &iVar3->field2_0x4 = (int)uVar5;
  ((int)&iVar3->field2_0x4 + 0x2) = uVar3;
  puVar1 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field9_0x1a));
  pass1_1008_3f62(puVar1,(u16 *)CONCAT22(uVar2,param_4 + 0xc));
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3->field2_0x4);
  iVar3->field14_0x20 = uVar2;
  iVar3->field15_0x22 = uVar3;
  return;
}
pub fn pass1_1038_6912(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  code **ppcVar3;
  u32 *puVar4;
  let mut iVar5: i16;
  let mut uVar6: u16;
  char *pcStack10;

  uVar6 = ((u32)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *param_1 = 0x78de;
  (iVar5 + 0x2) = (int)&u16_1050_1038;
  uVar1 = (iVar5 + 0x6);
  puVar4 = (u32 *)(u32)(iVar5 + 0x4);
  if ((uVar1 | puVar4) != 0x0) {
    ppcVar3 = (code **)*puVar4;
    (**ppcVar3)();
  }
  uVar1 = (iVar5 + 0xe);
  uVar2 = (iVar5 + 0x10);
  pcStack10 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e((u32 *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack10);
  }
  *param_1 = 0x389a;
  (iVar5 + 0x2) = 0x1008;
  return;
}
pub fn pass1_1038_6984(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((iVar1 + 0xc) != 0x0) {
    pass1_1020_c3ae();
    return;
  }
  if (*(i32 *)(iVar1 + 0xe) != 0x0) {
    pass1_1020_ba94(*(i32 **)(iVar1 + 0xe));
    return;
  }
  if ((iVar1 + 0x12) == 0x0) {
    if ((iVar1 + 0x14) == 0x0) {
      return;
    }
    pass1_1020_c42e((iVar1 + 0x14));
  }
  else {
    switch_1020_c3b4((iVar1 + 0x12));
  }
  return;
}
pub fn pass1_1038_69fe(mut param_1: u32)

{
  ((int)param_1 + 0x28) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6a0e(param_1: *mut astruct_419,param_2: *mut astruct_615)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  astruct_615 *uVar7;
  let mut uVar8: u16;
  let mut puVar9: *mut u16;
  let mut uVar10: u32;
  let mut uStack22: u32;
  u8 local_10 [0x4];
  u8 local_c [0x6];
  astruct_419 *paStack6;

  uVar8 = ((u32)param_2 >> 0x10);
  uVar7 = (astruct_615 *)param_2;
  if ((uVar7 + 0x1) == 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)&uVar7->field_0x20);
    paStack6 = (astruct_419 *)CONCAT22((int)param_1,(int)((u32)param_1 >> 0x10));
    piVar1 = &uVar7->field_0x24;
    *piVar1 = *piVar1 + 0x3c;
    puVar9 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_c));
    uVar5 = ((u32)puVar9 >> 0x10);
    while( true ) {
      uVar2 = pass1_1038_6d24(param_2,(u32 *)CONCAT22(0x1050,local_10),(u16 *)CONCAT22(0x1050,local_c),
                              (int)paStack6,((u32)paStack6 >> 0x10));
      if (uVar2 == 0x0) {
        pass1_1010_8fba(0x0,*(astruct_411 **)&uVar7->field_0x4);
        uStack22 = CONCAT22(uVar5,uVar2);
        uVar6 = uVar5 | uVar2;
        if (uVar6 == 0x0) {
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)&uVar7->field_0x20);
          pass1_1038_7356(param_2,(astruct_419 *)CONCAT22(uVar6,uVar2));
          return;
        }
        uVar10 = struct_op_1030_73a8(paStack6,uVar2,uVar6);
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0,((int)uVar10 + 0xc),0x40);
        if (BVar3 != 0x0) {
          (uVar7 + 0x1) = 0x1;
          (u32)&uVar7->field_0x20 = uStack22;
          return;
        }
        (u32)&uVar7->field_0x20 = uStack22;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar5,&uVar7->field_0x20));
        paStack6 = (astruct_419 *)(uStack22 & 0xffff | (u32)uVar5 << 0x10);
      }
      uVar4 = pass1_1038_6e1a(uVar7,uVar8,(i32 *)CONCAT22(0x1050,local_10));
      if (&uVar7->field_0x24 < (int)uVar4) break;
      piVar1 = &uVar7->field_0x24;
      *piVar1 = *piVar1 - uVar4;
      pass1_1008_3f62((u16 *)((u32)param_2 & 0xffff0000 | ZEXT24(&uVar7->field_0x1a)),
                      (u16 *)CONCAT22(0x1050,local_c));
    }
  }
  return;
}



u16 pass1_1038_6b3c(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (((((iVar1 + 0xc) == 0x0) && ((iVar1 + 0x12) == 0x0)) && ((iVar1 + 0x14) == 0x0)) &&
     ((*(i32 *)(iVar1 + 0xe) == 0x0 && (*(i32 *)(iVar1 + 0x16) != 0x0)))) {
    (u32)(iVar1 + 0x16) = 0x0;
  }
  if (*(i32 *)(iVar1 + 0x16) == 0x0) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6b88(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,u32 *param_5)

{
  u32 *puVar1;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  u32 *puVar3;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffee: u16;

  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000ffee,0x2f),in_stack_0000fe96,in_stack_0000ffba,
                           in_stack_0000ffc0,in_stack_0000ffc4);
  uVar2 = ((u32)puVar3 >> 0x10);
  puVar1 = (u32 *)&stack0xffee;
  pass1_1030_64ce(puVar1,uVar2,_PTR_LOOP_1050_5740,param_4,*(i32 *)((int)puVar3 + 0x20),
                  (u32 *)CONCAT22(0x1050,puVar1));
  *param_5 = *puVar1;
  return;
}
pub fn pass1_1038_6bd4(u8 *param_1,param_2: *mut astruct_615,param_3: *mut u16,u32 *param_4,mut param_5: i16)

{
  let mut uStack4: u16;

  pass1_1008_3f62(param_3,(u16 *)((u32)param_2 & 0xffff0000 | (u32)(param_2 + 0x1a)));
  if (param_5 < 0x0) {
    uStack4 = *param_3 - 0x1;
  }
  else {
    uStack4 = *param_3 + 0x1;
  }
  *param_3 = uStack4;
  pass1_1038_6b88(param_1,param_2,((u32)param_2 >> 0x10),param_3,param_4);
  return;
}
pub fn pass1_1038_6c1c(u8 *param_1,param_2: *mut astruct_615,param_3: *mut u16,u32 *param_4,mut param_5: i16)

{
  let mut uVar1: u16;
  let mut iStack4: i16;

  pass1_1008_3f62(param_3,(u16 *)((u32)param_2 & 0xffff0000 | (u32)(param_2 + 0x1a)));
  uVar1 = ((u32)param_3 >> 0x10);
  iStack4 = ((int)param_3 + 0x2);
  if (param_5 < 0x0) {
    iStack4 += -0x1;
  }
  else {
    iStack4 += 0x1;
  }
  ((int)param_3 + 0x2) = iStack4;
  pass1_1038_6b88(param_1,param_2,((u32)param_2 >> 0x10),param_3,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6c68(u8 *param_1,param_2: *mut astruct_615,param_3: *mut u16,u32 *param_4,mut param_5: i16)

{
  let mut iVar1: i16;
  u8 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut puVar5: *mut u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar8;
  let mut uVar9: u16;
  u32 *puVar10;
  let mut uVar11: u32;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffd8: u16;
  let mut iStack30: i16;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar3 = param_2;
  pass1_1008_3f62(param_3,(u16 *)((u32)param_2 & 0xffff0000 | (u32)(uVar3 + 0x1a)));
  puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffd8,0x2f),in_stack_0000fe80,
                            in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
  uVar6 = ((u32)puVar10 >> 0x10);
  puVar5 = (u16 *)((u32)param_2 & 0xffff0000 | (u32)(uVar3 + 0x1a));
  pass1_1030_627e(uVar3 + 0x1a,uVar6,_PTR_LOOP_1050_5740,puVar5,*(i32 *)((int)puVar10 + 0x20));
  uVar4 = puVar5;
  uVar7 = uVar6 | uVar4;
  if (uVar7 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)puVar5 & 0xffff | (u32)uVar6 << 0x10);
    uVar11 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar7,uVar4),uVar4,uVar7);
    puVar2 = (uVar11 >> 0x10);
    iVar1 = ((int)uVar11 + 0xc);
    if ((iVar1 == 0x47) || (iVar1 == 0x6a)) {
      uVar9 = ((u32)param_2 >> 0x10);
      iStack30 = (uVar3 + 0x1e);
      if (param_5 < 0x0) {
        iStack30 += -0x1;
      }
      else {
        iStack30 += 0x1;
      }
      ((int)param_3 + 0x4) = iStack30;
      pass1_1038_6b88(puVar2,uVar3,uVar9,param_3,param_4);
    }
  }
  return;
}



i16 pass1_1038_6d24(param_1: *mut astruct_615,u32 *param_2,param_3: *mut u16,mut param_4: i16,mut param_5: u16 )

{
  u8 *puVar1;
  let mut local_14: i16;
  let mut local_12: i16;
  let mut local_10: i16;
  let mut local_e: i16;
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  *param_2 = 0x0;
  local_8 = (u32)(param_4 + 0xc);
  uStack4 = (param_4 + 0x10);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_e),
                  (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
  pass1_1008_3eb4((astruct_615 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x1a)),
                  (u16 *)CONCAT22(0x1050,&local_14),(u16 *)CONCAT22(0x1050,&local_12),
                  (u16 *)CONCAT22(0x1050,&local_10));
  local_c -= local_12;
  local_e -= local_14;
  puVar1 = (local_a - local_10);
  if (((puVar1 == NULL) && (local_c == 0x0)) && (local_e == 0x0)) {
    return 0x0;
  }
  if ((local_c != 0x0) || (puVar1 == NULL)) {
    if ((puVar1 == NULL) && (local_c != 0x0)) {
      pass1_1038_6c1c(NULL,param_1,param_3,param_2,local_c);
      return 0x1;
    }
    if (((puVar1 == NULL) && (local_c == 0x0)) && (local_e != 0x0)) {
      pass1_1038_6c68(NULL,param_1,param_3,param_2,local_e);
      if (local_c != 0x0) {
        return 0x1;
      }
      return local_c;
    }
  }
  pass1_1038_6bd4(puVar1,param_1,param_3,param_2,(int)puVar1);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1038_6e1a(mut param_1: u16 ,mut param_2: u16 ,i32 *param_3)

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 bStack21;
  let mut uStack4: u16;

  uStack4 = 0x0;
  if ((*param_3 == 0x0) && (param_3 == 0x0)) {
    return 0x1;
  }
  uVar4 = ((int)param_3 + 0x2);
  bStack21 = (u8)(uVar4 >> 0x8);
  uVar1 = bStack21;
  if (bStack21 == 0x0) {
    uStack4 = param_3;
    goto switchD_1038_6eab_caseD_9;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*param_3);
  uVar3 = pass1_1030_6fa0(CONCAT22(uVar4,uVar1));
  if ((int)uVar3 < 0xa) {
    switch(uVar3) {
    case 0x1:
      uStack4 = 0x1;
      break;
    case 0x2:
    case 0x6:
      uStack4 = 0x2;
      break;
    case 0x3:
    case 0x7:
      uStack4 = 0x3;
      break;
    case 0x4:
    case 0x8:
      uStack4 = 0x4;
      break;
    case 0x5:
    case 0x9:
      goto switchD_1038_6eab_caseD_5;
    }
  }
  else {
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x41);
    if (BVar2 != 0x0) {
      uStack4 = 0xa;
      goto switchD_1038_6eab_caseD_9;
    }
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x42);
    if ((BVar2 != 0x0) || (uVar3 == 0x3f)) {
      uStack4 = 0xb;
      goto switchD_1038_6eab_caseD_9;
    }
switchD_1038_6eab_caseD_5:
    uStack4 = 0x5;
  }
switchD_1038_6eab_caseD_9:
  switch(uStack4) {
  case 0x1:
    return 0x14;
  case 0x2:
  case 0x7:
    return 0x3c;
  case 0x3:
  case 0x8:
    return 0x78;
  case 0x4:
  case 0x9:
    return 0xf0;
  case 0x5:
  case 0x6:
    return 0xf;
  case 0xa:
    uVar3 = 0xc;
    break;
  case 0xb:
    uVar3 = 0xa;
    break;
  default:
    uVar3 = 0xffff;
  }
  return uVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6f5a(mut param_1: u16 ,u8 *param_2,mut param_3: u32,param_4: *mut astruct_419)

{
  let mut uVar1: u32;
  i32 lVar2;
  let mut uVar4: u16;
  let mut puVar5: *mut u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar7;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  astruct_99 *paStack16;
  let mut uStack12: u16;
  let mut local_a: u16;
  let mut uStack8: u16;
  let mut local_6: u16;
  let mut uStack4: u16;
  astruct_623 *uVar3;

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar10 = (param_3 >> 0x10);
  iVar8 = (int)param_3;
  if (*(i32 *)(iVar8 + 0xe) == 0x0) {
    if ((iVar8 + 0xc) != 0x0) {
      pass1_1030_7ddc(param_1,paVar7,(u32)param_4,*(i32 *)(iVar8 + 0x16),(iVar8 + 0xc));
      return;
    }
    if ((iVar8 + 0x12) != 0x0) {
      pass1_1030_7c50(param_1,paVar7,(astruct_305 *)param_4,*(i32 *)(iVar8 + 0x16),(iVar8 + 0x12));
      return;
    }
    paStack16 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
    uVar4 = ((u32)paStack16 >> 0x10);
    uVar3 = (astruct_623 *)paStack16;
    if ((uVar4 | uVar3) == 0x0) {
      paStack16 = NULL;
    }
    else {
      paStack16->field0_0x0 = 0x389a;
      uVar3->field2_0x2 = 0x1008;
      uVar3->field3_0x4 = 0x0;
      uVar3->field4_0x6 = 0x0;
      uVar3->field5_0x8 = 0x0;
      uVar3->field6_0xa = 0x0;
      uVar3->field7_0xc = 0x0;
      paStack16->field0_0x0 = 0x56ce;
      uVar3->field2_0x2 = 0x1018;
    }
    uVar11 = ((u32)paStack16 >> 0x10);
    iVar9 = (int)paStack16;
    (iVar9 + 0x8) = (iVar8 + 0x14);
    (iVar9 + 0xa) = (iVar8 + 0x16);
    uVar6 = pass1_1020_c42e((iVar8 + 0x14));
    lVar2 = (u32)uVar6 * (u32)(iVar9 + 0xa);
    uVar4 = lVar2;
    (iVar9 + 0xc) = uVar4;
    pass1_1030_6a2c(uVar4,(StructD *)((u32)lVar2 >> 0x10),(astruct_382 *)param_4,(astruct_383 *)paStack16);
  }
  else {
    uVar1 = (u32)(iVar8 + 0xe);
    uStack4 = ((int)uVar1 + 0x4);
    for (uStack12 = 0x0; uStack12 < uStack4; uStack12 += 0x1) {
      puVar5 = &local_6;
      pass1_1020_bb16((u32*)(iVar8 + 0xe),(u32 *)CONCAT22(0x1050,&local_a),(u16 *)CONCAT22(0x1050,puVar5),
                      uStack12);
      if (CONCAT22(uStack8,local_a) != 0x0) {
        pass1_1030_7ddc(puVar5,paVar7,(u32)param_4,CONCAT22(uStack8,local_a),local_6);
      }
    }
  }
  return;
}

