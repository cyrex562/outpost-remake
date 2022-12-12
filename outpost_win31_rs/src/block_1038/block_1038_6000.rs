
pub unsafe fn file_1038_6118(mut param_1: i16,param_2: *mut astruct_57,param_3: *mut astruct_373,param_4: *mut HFILE16)

{
  let mut puVar1: *mut u32;
  let mut BVar2: bool;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u8;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut iVar9: *mut astruct_373;
  let mut uVar10: *mut astruct_373;
  let mut uVar9: u16;
  let mut uVar11: u16;
  let mut puStack1046: *mut u8;
  let mut uStack1042: u16;
  let mut local_408: [u8;0x400] = [0;0x400];
  let mut local_8: u16;
  let mut local_6: u32;

  paVar8 = CONCAT22(in_register_0000000a,param_2);
  file_1030_1730(param_3,param_4);
  if (param_1 == 0) {
    return;
  }
  local_6 = 0;
  puVar1 = &local_6;
  file_1008_7548(param_4,CONCAT22(0x1050,puVar1),paVar8);
  if (puVar1.is_null() == false) {
    uVar10 = (param_3 >> 0x10);
    iVar9 = param_3;
    iVar9.field_0xc = local_6;
    BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9.field13_0x10)),0x4);
    if (((((BVar2 != 0) &&
          (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9.field_0x18)),0x2),
          BVar2 != 0)) &&
         (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9.field19_0x1a)),0x2),
         BVar2 != 0)) &&
        ((BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_8),0x2), BVar2 != 0x0 &&
         (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0x2)),0x4),
         BVar2 != 0)))) &&
       (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                  ZEXT24((&iVar9[0x1].field4_0x4 + 0x2))),0x2),
       BVar2 != 0)) {
      (iVar9 + 1) = local_8;
      BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0x8)),0x2);
      if ((BVar2 != 0) &&
         (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0xa)),0x94),
         BVar2 != 0)) {
        if (u16_1050_0312 < 0x2) {
          uVar9 = 0x54;
          uVar11 = 0;
          mem_op_1000_179c(0x54,paVar8);
          uVar7 = SUB42(paVar8,0x0);
          puStack1046 = CONCAT22(uVar7,BVar2);
          BVar3 = read_file_1008_7dee(param_4,CONCAT22(uVar7,BVar2),CONCAT22(uVar11,uVar9));
          if (BVar3 == 0) {//
// LAB_1038_626a:
            u16_1050_0310 = 0x6d2;
            fn_ptr_1000_17ce(puStack1046);
            return;
          }
          uStack1042 = 0;
          loop {
            uVar4 = switch_1008_72bc(param_4,uStack1042);
            uVar9 = (uStack1042 * 0x4 + BVar2 + 2);
            (&iVar9[0xb].field19_0x1a)[uVar4 * 0x2] = (uStack1042 * 0x4 + BVar2);
            (&iVar9[0xc].field_0x0 + uVar4 * 0x4) = uVar9;
            uStack1042 += 0x1;
          } while (uStack1042 < 0x15);
          BVar3 = read_file_1008_7dee(param_4,puStack1046,0x54);
//          if (BVar3 == 0) goto LAB_1038_626a;
          uStack1042 = 0;
          loop {
            uVar5 = switch_1008_72bc(param_4,uStack1042);
            uVar4 = (uStack1042 * 0x4 + BVar2 + 2);
            (&iVar9[0xe].field19_0x1a)[uVar5 * 0x2] = (uStack1042 * 0x4 + BVar2);
            (&iVar9[0xf].field_0x0 + uVar5 * 0x4) = uVar4;
            uStack1042 += 0x1;
          } while (uStack1042 < 0x15);
          fn_ptr_1000_17ce(puStack1046);
        }
        else {
          BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0xb].field19_0x1a)),
                                      0x54);
          uVar4 = paVar8;
          if (BVar2 == 0) {
            u16_1050_0310 = 0x6d2;
            return;
          }
          BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0xe].field19_0x1a)),
                                      0x54);
          if (BVar2 == 0) {
            u16_1050_0310 = 0x6d2;
            return;
          }
        }
    // WARNING: Load size is inaccurate
        read_file_1030_33f0(iVar9[0x11].field19_0x1a,param_4);
        puVar6 = local_408;
        read_file_1008_7c6e(param_4,(param_4 >> 0x10),CONCAT22(0x1050,puVar6));
        if (puVar6.is_null() == false) {
          uVar5 = str_op_1008_60e8(uVar4,CONCAT22(0x1050,local_408));
          iVar9[0x12].field_0x2 = uVar5;
          iVar9[0x12].field4_0x4 = uVar4;
          BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                    ZEXT24((&iVar9[0x12].field4_0x4 + 0x2))),0x2);
          if (((((BVar2 != 0) &&
                (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                           CONCAT11((param_3 >> 0x8) + '\x02',
                                                                           param_3)),0x4), BVar2 != 0)) &&
               (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0x12].field_0xc))
                                            ,0x2), BVar2 != 0)) &&
              (((BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0x12].field_0xe)
                                                           ),0x2), BVar2 != 0x0 &&
                (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                           ZEXT24(&iVar9[0x12].field13_0x10)),0x2), BVar2 != 0)) &&
               ((BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                           ZEXT24((&iVar9[0x12].field13_0x10 + 0x2))),
                                             0x2), BVar2 != 0x0 &&
                ((BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                            ZEXT24(&iVar9[0x12].field_0x14)),0x2), BVar2 != 0x0 &&
                 (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                            ZEXT24(&iVar9[0x12].field16_0x16)),0x2), BVar2 != 0)))))))
              ) && ((u16_1050_0312 < 0x2 ||
                    ((BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(iVar9 + 0x13)),0x2
                                                 ), BVar2 != 0x0 &&
                     (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                                ZEXT24(&iVar9[0x13].field_0x2)),0x4), BVar2 != 0))))))
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



pub unsafe fn pass1_1038_64de(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_33f8(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub unsafe fn struct_1038_6520(param_1: *mut astruct_308)

{
  pstruct308_1: *mut astruct_308;
  pstruct308_2: *mut astruct_308;

  pstruct308_2 = (param_1 >> 0x10);
  pstruct308_1 = param_1;
  param_1.field0_0x0 = 0x389a;
  pstruct308_1.field1_0x2 = 0x1008;
  pstruct308_1.field2_0x4 = 0;
  pstruct308_1.field3_0x8 = 0;
  pstruct308_1.field4_0xc = 0;
  pstruct308_1.field5_0xe = 0;
  pstruct308_1.field6_0x12 = 0;
  pstruct308_1.field7_0x14 = 0;
  pstruct308_1.field8_0x16 = 0;
  pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&pstruct308_1.field9_0x1a)));
  pstruct308_1.field12_0x20 = 0;
  pstruct308_1.field13_0x24 = 0;
  pstruct308_1.field14_0x26 = 0;
  pstruct308_1.field15_0x28 = 0;
  param_1.field0_0x0 = 0x78de;
    // just 0x1038
  pstruct308_1.field1_0x2 = &u16_1050_1038;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_6590(param_1: *mut astruct_410,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut iVar4: i16;
  let mut iVar3: *mut astruct_410;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut uVar7: u32;

  uVar5 = (param_1 >> 0x10);
  iVar3 = param_1;
  param_1 = 0x389a;
  iVar3.field2_0x2 = 0x1008;
  iVar3.field3_0x4 = 0;
  iVar3.field5_0x8 = param_6;
  iVar3.field6_0xc = param_4;
  iVar3.field7_0xe = 0;
  iVar3.field8_0x12 = 0;
  iVar3.field9_0x14 = 0;
  iVar3.field10_0x16 = param_2;
  iVar3.field11_0x18 = param_3;
  puVar6 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x1a)));
  uVar2 = (puVar6 >> 0x10);
  iVar3.field18_0x20 = 0;
  iVar3.field20_0x24 = 0;
  iVar3.field21_0x26 = param_5;
  iVar3.field22_0x28 = 0;
  param_1 = 0x78de;
  iVar3.field2_0x2 = &u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6);
  uVar7 = pass1_1030_6d4e(param_5,uVar2,CONCAT22(uVar2,param_5));
  iVar4 = (uVar7 >> 0x10);
  iVar3.field3_0x4 = uVar7;
  iVar3.field4_0x6 = iVar4;
  puVar1 = (param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x1a));
  pass1_1008_3f62(puVar1,CONCAT22(uVar2,param_5 + 0xc));
    // WARNING: Load size is inaccurate
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3.field3_0x4);
  iVar3.field18_0x20 = uVar2;
  iVar3.field19_0x22 = iVar4;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_666e(param_1: *mut astruct_420,param_2: *mut i32,mut param_3: u16 ,mut param_4: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar3: *mut astruct_420;
  let mut uVar4: *mut astruct_420;
  let mut puVar4: *mut u16;
  let mut uVar5: u32;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  param_1 = 0x389a;
  iVar3.field2_0x2 = 0x1008;
  iVar3.field3_0x4 = null_mut();
  iVar3.field4_0x8 = param_4;
  iVar3.field5_0xc = 0;
  iVar3.field6_0xe = param_2;
  iVar3.field7_0x12 = 0;
  iVar3.field8_0x14 = 0;
  iVar3.field10_0x18 = 0;
  iVar3.field9_0x16 = 0;
  puVar4 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar3.field11_0x1a)));
  uVar2 = (puVar4 >> 0x10);
  iVar3.field16_0x20 = 0;
  iVar3.field18_0x24 = 0;
  iVar3.field19_0x26 = param_3;
  iVar3.field20_0x28 = 0;
  param_1 = 0x78de;
  iVar3.field2_0x2 = &u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar5 = pass1_1030_6d4e(param_3,uVar2,CONCAT22(uVar2,param_3));
  uVar3 = (uVar5 >> 0x10);
  iVar3.field3_0x4 = uVar5;
  (&iVar3.field3_0x4 + 0x2) = uVar3;
  puVar1 = (param_1 & 0xffff0000 | ZEXT24(&iVar3.field11_0x1a));
  pass1_1008_3f62(puVar1,CONCAT22(uVar2,param_3 + 0xc));
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3.field3_0x4);
  iVar3.field16_0x20 = uVar2;
  iVar3.field17_0x22 = uVar3;
  pass1_1020_ba94(param_2);
  iVar3.field9_0x16 = uVar2;
  iVar3.field10_0x18 = uVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_675c(param_1: *mut astruct_414,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar3: *mut astruct_414;
  let mut uVar4: *mut astruct_414;
  let mut puVar4: *mut u16;
  let mut uVar5: u32;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  param_1.field0_0x0 = 0x389a;
  iVar3.field1_0x2 = 0x1008;
  iVar3.field2_0x4 = null_mut();
  iVar3.field3_0x8 = param_5;
  iVar3.field4_0xc = 0;
  iVar3.field5_0xe = 0;
  iVar3.field6_0x12 = param_3;
  iVar3.field7_0x14 = 0;
  iVar3.field8_0x16 = param_2;
  puVar4 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar3.field9_0x1a)));
  uVar2 = (puVar4 >> 0x10);
  iVar3.field14_0x20 = 0;
  iVar3.field16_0x24 = 0;
  iVar3.field17_0x26 = param_4;
  iVar3.field18_0x28 = 0;
  param_1.field0_0x0 = 0x78de;
  iVar3.field1_0x2 = &u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  uVar5 = pass1_1030_6d4e(param_4,uVar2,CONCAT22(uVar2,param_4));
  uVar3 = (uVar5 >> 0x10);
  iVar3.field2_0x4 = uVar5;
  (&iVar3.field2_0x4 + 0x2) = uVar3;
  puVar1 = (param_1 & 0xffff0000 | ZEXT24(&iVar3.field9_0x1a));
  pass1_1008_3f62(puVar1,CONCAT22(uVar2,param_4 + 0xc));
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3.field2_0x4);
  iVar3.field14_0x20 = uVar2;
  iVar3.field15_0x22 = uVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_6838(param_1: *mut astruct_415,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar3: *mut astruct_415;
  let mut uVar4: *mut astruct_415;
  let mut puVar4: *mut u16;
  let mut uVar5: u32;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  param_1.field0_0x0 = 0x389a;
  iVar3.field1_0x2 = 0x1008;
  iVar3.field2_0x4 = null_mut();
  iVar3.field3_0x8 = param_5;
  iVar3.field4_0xc = 0;
  iVar3.field5_0xe = 0;
  iVar3.field6_0x12 = 0;
  iVar3.field7_0x14 = param_3;
  iVar3.field8_0x16 = param_2;
  puVar4 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar3.field9_0x1a)));
  uVar2 = (puVar4 >> 0x10);
  iVar3.field14_0x20 = 0;
  iVar3.field16_0x24 = 0;
  iVar3.field17_0x26 = param_4;
  iVar3.field18_0x28 = 0;
  param_1.field0_0x0 = 0x78de;
    // 0x1038
  iVar3.field1_0x2 = &u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  uVar5 = pass1_1030_6d4e(param_4,uVar2,CONCAT22(uVar2,param_4));
  uVar3 = (uVar5 >> 0x10);
  iVar3.field2_0x4 = uVar5;
  (&iVar3.field2_0x4 + 0x2) = uVar3;
  puVar1 = (param_1 & 0xffff0000 | ZEXT24(&iVar3.field9_0x1a));
  pass1_1008_3f62(puVar1,CONCAT22(uVar2,param_4 + 0xc));
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3.field2_0x4);
  iVar3.field14_0x20 = uVar2;
  iVar3.field15_0x22 = uVar3;
  return;
}
pub unsafe fn pass1_1038_6912(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut puVar4: *mut u32;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut pcStack10: *mut c_char;

  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  *param_1 = 0x78de;
  (iVar5 + 0x2) = &u16_1050_1038;
  uVar1 = (iVar5 + 0x6);
  puVar4 = (iVar5 + 0x4);
  if ((uVar1 | puVar4) != 0) {
    ppcVar3 = *puVar4;
    (**ppcVar3)();
  }
  uVar1 = (iVar5 + 0xe);
  uVar2 = (iVar5 + 0x10);
  pcStack10 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0) {
    fn_ptr_1020_ba7e(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack10);
  }
  *param_1 = 0x389a;
  (iVar5 + 0x2) = 0x1008;
  return;
}
pub unsafe fn pass1_1038_6984(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0xc) != 0) {
    pass1_1020_c3ae();
    return;
  }
  if ((iVar1 + 0xe) != 0) {
    pass1_1020_ba94(*(i32 **)(iVar1 + 0xe));
    return;
  }
  if ((iVar1 + 0x12) == 0) {
    if ((iVar1 + 0x14) == 0) {
      return;
    }
    pass1_1020_c42e((iVar1 + 0x14));
  }
  else {
    switch_1020_c3b4((iVar1 + 0x12));
  }
  return;
}
pub unsafe fn pass1_1038_69fe(mut param_1: u32)

{
  (param_1 + 0x28) = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_6a0e(param_1: *mut astruct_419,param_2: *mut astruct_615)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: *mut astruct_615;
  let mut uVar8: u16;
  let mut puVar9: *mut u16;
  let mut uVar10: u32;
  let mut uStack22: u32;
  let mut local_10: [u8;0x4] = [0;0x4];
  let mut local_c: [u8;0x6] = [0;0x6];
  let mut paStack6: *mut astruct_419;

  uVar8 = (param_2 >> 0x10);
  uVar7 = param_2;
  if ((uVar7 + 1) == 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,&uVar7.field_0x20);
    paStack6 = CONCAT22(param_1,(param_1 >> 0x10));
    piVar1 = &uVar7.field_0x24;
    *piVar1 = *piVar1 + 0x3c;
    puVar9 = pass1_1008_3e38(CONCAT22(0x1050,local_c));
    uVar5 = (puVar9 >> 0x10);
    loop {
      uVar2 = pass1_1038_6d24(param_2,CONCAT22(0x1050,local_10),CONCAT22(0x1050,local_c),
                              paStack6,(paStack6 >> 0x10));
      if (uVar2 == 0) {
        pass1_1010_8fba(0x0,&uVar7.field_0x4);
        uStack22 = CONCAT22(uVar5,uVar2);
        uVar6 = uVar5 | uVar2;
        if (uVar6 == 0) {
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,&uVar7.field_0x20);
          pass1_1038_7356(param_2,CONCAT22(uVar6,uVar2));
          return;
        }
        uVar10 = struct_op_1030_73a8(paStack6,uVar2,uVar6);
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0,(uVar10 + 0xc),0x40);
        if (BVar3 != 0) {
          (uVar7 + 1) = 0x1;
          uVar7.field_0x20 = uStack22;
          return;
        }
        uVar7.field_0x20 = uStack22;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar5,&uVar7.field_0x20));
        paStack6 = (uStack22 & 0xffff | uVar5 << 0x10);
      }
      uVar4 = pass1_1038_6e1a(uVar7,uVar8,CONCAT22(0x1050,local_10));
      if (&uVar7.field_0x24 < uVar4) { break; }
      piVar1 = &uVar7.field_0x24;
      *piVar1 = *piVar1 - uVar4;
      pass1_1008_3f62((param_2 & 0xffff0000 | ZEXT24(&uVar7.field_0x1a)),
                      CONCAT22(0x1050,local_c));
    }
  }
  return;
}



pub unsafe fn pass1_1038_6b3c(mut param_1: u32) -> u16

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (((((iVar1 + 0xc) == 0) && ((iVar1 + 0x12) == 0)) && ((iVar1 + 0x14) == 0)) &&
     (((iVar1 + 0xe) == 0x0 && ((iVar1 + 0x16) != 0)))) {
    (iVar1 + 0x16) = 0;
  }
  if ((iVar1 + 0x16) == 0) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_6b88(param_1: *mut u8,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,param_5: *mut u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffee: u16;

  puVar3 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           CONCAT22(in_stack_0000ffee,0x2f),in_stack_0000fe96,in_stack_0000ffba,
                           in_stack_0000ffc0,in_stack_0000ffc4);
  uVar2 = (puVar3 >> 0x10);
  puVar1 = &stack0xffee;
  pass1_1030_64ce(puVar1,uVar2,_PTR_LOOP_1050_5740,param_4,(puVar3 + 0x20),
                  CONCAT22(0x1050,puVar1));
  *param_5 = *puVar1;
  return;
}
pub unsafe fn pass1_1038_6bd4(param_1: *mut u8,param_2: *mut astruct_615,param_3: *mut u16,param_4: *mut u32,mut param_5: i16)

{
  let mut uStack4: u16;

  pass1_1008_3f62(param_3,(param_2 & 0xffff0000 | (param_2 + 0x1a)));
  if (param_5 < 0x0) {
    uStack4 = *param_3 - 0x1;
  }
  else {
    uStack4 = *param_3 + 1;
  }
  *param_3 = uStack4;
  pass1_1038_6b88(param_1,param_2,(param_2 >> 0x10),param_3,param_4);
  return;
}
pub unsafe fn pass1_1038_6c1c(param_1: *mut u8,param_2: *mut astruct_615,param_3: *mut u16,param_4: *mut u32,mut param_5: i16)

{
  let mut uVar1: u16;
  let mut iStack4: i16;

  pass1_1008_3f62(param_3,(param_2 & 0xffff0000 | (param_2 + 0x1a)));
  uVar1 = (param_3 >> 0x10);
  iStack4 = (param_3 + 2);
  if (param_5 < 0x0) {
    iStack4 += -0x1;
  }
  else {
    iStack4 += 0x1;
  }
  (param_3 + 0x2) = iStack4;
  pass1_1038_6b88(param_1,param_2,(param_2 >> 0x10),param_3,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_6c68(param_1: *mut u8,param_2: *mut astruct_615,param_3: *mut u16,param_4: *mut u32,mut param_5: i16)

{
  let mut iVar1: i16;
  let mut puVar2: *mut u8;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut puVar5: *mut u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u16;
  let mut puVar10: *mut u32;
  let mut uVar11: u32;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffd8: u16;
  let mut iStack30: i16;

  paVar8 = CONCAT22(in_register_0000000a,param_1);
  uVar3 = param_2;
  pass1_1008_3f62(param_3,(param_2 & 0xffff0000 | (uVar3 + 0x1a)));
  puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(in_stack_0000ffd8,0x2f),in_stack_0000fe80,
                            in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
  uVar6 = (puVar10 >> 0x10);
  puVar5 = (param_2 & 0xffff0000 | (uVar3 + 0x1a));
  pass1_1030_627e(uVar3 + 0x1a,uVar6,_PTR_LOOP_1050_5740,puVar5,(puVar10 + 0x20));
  uVar4 = puVar5;
  uVar7 = uVar6 | uVar4;
  if (uVar7 != 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,puVar5 & 0xffff | uVar6 << 0x10);
    uVar11 = struct_op_1030_73a8(CONCAT22(uVar7,uVar4),uVar4,uVar7);
    puVar2 = (uVar11 >> 0x10);
    iVar1 = (uVar11 + 0xc);
    if ((iVar1 == 0x47) || (iVar1 == 0x6a)) {
      uVar9 = (param_2 >> 0x10);
      iStack30 = (uVar3 + 0x1e);
      if (param_5 < 0x0) {
        iStack30 += -0x1;
      }
      else {
        iStack30 += 0x1;
      }
      (param_3 + 0x4) = iStack30;
      pass1_1038_6b88(puVar2,uVar3,uVar9,param_3,param_4);
    }
  }
  return;
}



pub unsafe fn pass1_1038_6d24(param_1: *mut astruct_615,param_2: *mut u32,param_3: *mut u16,mut param_4: i16,mut param_5: u16 ) -> i16

{
  let mut puVar1: *mut u8;
  let mut local_14: i16;
  let mut local_12: i16;
  let mut local_10: i16;
  let mut local_e: i16;
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  *param_2 = 0;
  local_8 = (param_4 + 0xc);
  uStack4 = (param_4 + 0x10);
  pass1_1008_3eb4(CONCAT22(0x1050,&local_8),CONCAT22(0x1050,&local_e),
                  CONCAT22(0x1050,&local_c),CONCAT22(0x1050,&local_a));
  pass1_1008_3eb4((param_1 & 0xffff0000 | (param_1 + 0x1a)),
                  CONCAT22(0x1050,&local_14),CONCAT22(0x1050,&local_12),
                  CONCAT22(0x1050,&local_10));
  local_c -= local_12;
  local_e -= local_14;
  puVar1 = (local_a - local_10);
  if (((puVar1.is_null()) && (local_c == 0)) && (local_e == 0)) {
    return 0x0;
  }
  if ((local_c != 0) || (puVar1.is_null())) {
    if ((puVar1.is_null()) && (local_c != 0)) {
      pass1_1038_6c1c(NULL,param_1,param_3,param_2,local_c);
      return 0x1;
    }
    if (((puVar1.is_null()) && (local_c == 0)) && (local_e != 0)) {
      pass1_1038_6c68(NULL,param_1,param_3,param_2,local_e);
      if (local_c != 0) {
        return 0x1;
      }
      return local_c;
    }
  }
  pass1_1038_6bd4(puVar1,param_1,param_3,param_2,puVar1);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1038_6e1a(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut i32) -> u16

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut bStack21: u8;
  let mut uStack4: u16;

  uStack4 = 0;
  if ((*param_3 == 0) && (param_3 == 0)) {
    return 0x1;
  }
  uVar4 = (param_3 + 2);
  bStack21 = (uVar4 >> 0x8);
  uVar1 = bStack21;
  if (bStack21 == 0) {
    uStack4 = param_3;
// TODO: goto switchD_1038_6eab_caseD_9;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*param_3);
  uVar3 = pass1_1030_6fa0(CONCAT22(uVar4,uVar1));
  if (uVar3 < 0xa) {
    match uVar3 {
    0x1 =>
      uStack4 = 0x1;
      break;
    0x2 =>
    0x6 =>
      uStack4 = 0x2;
      break;
    0x3 =>
    0x7 =>
      uStack4 = 0x3;
      break;
    0x4 =>
    0x8 =>
      uStack4 = 0x4;
      break;
    0x5 =>
    0x9 =>
  // TODO: goto switchD_1038_6eab_caseD_5;
    }
  }
  else {
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x41);
    if (BVar2 != 0) {
      uStack4 = 0xa;
  // TODO: goto switchD_1038_6eab_caseD_9;
    }
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x42);
    if ((BVar2 != 0) || (uVar3 == 0x3f)) {
      uStack4 = 0xb;
  // TODO: goto switchD_1038_6eab_caseD_9;
    }
switchD_1038_6eab_caseD_5:
    uStack4 = 0x5;
  }
switchD_1038_6eab_caseD_9:
  match uStack4 {
  0x1 =>
    return 0x14;
  0x2 =>
  0x7 =>
    return 0x3c;
  0x3 =>
  0x8 =>
    return 0x78;
  0x4 =>
  0x9 =>
    return 0xf0;
  0x5 =>
  0x6 =>
    return 0xf;
  0xa =>
    uVar3 = 0xc;
    break;
  0xb =>
    uVar3 = 0xa;
    break;
  _ =>
    uVar3 = 0xffff;
  }
  return uVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_6f5a(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32,param_4: *mut astruct_419)

{
  let mut uVar1: u32;
  let mut lVar2: i32;
  let mut uVar4: u16;
  let mut puVar5: *mut u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut paStack16: *mut astruct_99;
  let mut uStack12: u16;
  let mut local_a: u16;
  let mut uStack8: u16;
  let mut local_6: u16;
  let mut uStack4: u16;
  let mut uVar3: *mut astruct_623;

  paVar7 = CONCAT22(in_register_0000000a,param_2);
  uVar10 = (param_3 >> 0x10);
  iVar8 = param_3;
  if ((iVar8 + 0xe) == 0) {
    if ((iVar8 + 0xc) != 0) {
      pass1_1030_7ddc(param_1,paVar7,param_4,(iVar8 + 0x16),(iVar8 + 0xc));
      return;
    }
    if ((iVar8 + 0x12) != 0) {
      pass1_1030_7c50(param_1,paVar7,param_4,(iVar8 + 0x16),(iVar8 + 0x12));
      return;
    }
    paStack16 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
    uVar4 = (paStack16 >> 0x10);
    uVar3 = paStack16;
    if ((uVar4 | uVar3) == 0) {
      paStack16 = null_mut();
    }
    else {
      paStack16.field0_0x0 = 0x389a;
      uVar3.field2_0x2 = 0x1008;
      uVar3.field3_0x4 = 0;
      uVar3.field4_0x6 = 0;
      uVar3.field5_0x8 = 0;
      uVar3.field6_0xa = 0;
      uVar3.field7_0xc = 0;
      paStack16.field0_0x0 = 0x56ce;
      uVar3.field2_0x2 = 0x1018;
    }
    uVar11 = (paStack16 >> 0x10);
    iVar9 = paStack16;
    (iVar9 + 0x8) = (iVar8 + 0x14);
    (iVar9 + 0xa) = (iVar8 + 0x16);
    uVar6 = pass1_1020_c42e((iVar8 + 0x14));
    lVar2 = uVar6 * (iVar9 + 0xa);
    uVar4 = lVar2;
    (iVar9 + 0xc) = uVar4;
    pass1_1030_6a2c(uVar4,(lVar2 >> 0x10),param_4,paStack16);
  }
  else {
    uVar1 = (iVar8 + 0xe);
    uStack4 = (uVar1 + 0x4);
    for uStack12 in 0 .. uStack4 {
      puVar5 = &local_6;
      pass1_1020_bb16((iVar8 + 0xe),CONCAT22(0x1050,&local_a),CONCAT22(0x1050,puVar5),
                      uStack12);
      if (CONCAT22(uStack8,local_a) != 0) {
        pass1_1030_7ddc(puVar5,paVar7,param_4,CONCAT22(uStack8,local_a),local_6);
      }
    }
  }
  return;
}
