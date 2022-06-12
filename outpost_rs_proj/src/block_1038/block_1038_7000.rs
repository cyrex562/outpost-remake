
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_709c(u8 *param_1,param_2: *mut astruct_618,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  i32 lVar2;
  let mut uVar7: u16;
  let mut uVar8: u16;
  StructD *pSVar9;
  let mut in_register_0000000a: u16;
  astruct_618 *iVar8;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  astruct_99 *paStack40;
  astruct_99 *paStack16;
  let mut uStack12: u16;
  i32 local_a;
  let mut local_6: u16;
  let mut uStack4: u16;
  astruct_617 *uVar3;
  astruct_619 *uVar4;
  astruct_620 *uVar5;
  astruct_621 *uVar6;

  uVar11 = (param_2 >> 0x10);
  iVar8 = (astruct_618 *)param_2;
  if (((&iVar8->field13_0xe + 0x2) | &iVar8->field13_0xe) == 0x0) {
    if (iVar8->field12_0xc == 0x0) {
      if (iVar8->field14_0x12 == 0x0) {
        if (iVar8->field15_0x14 == 0x0) {
          return;
        }
        paStack40 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar8 = (paStack40 >> 0x10);
        uVar3 = (astruct_617 *)paStack40;
        if ((uVar8 | uVar3) == 0x0) {
          paStack40 = NULL;
        }
        else {
          paStack40->field0_0x0 = 0x389a;
          uVar3->field2_0x2 = 0x1008;
          uVar3->field3_0x4 = 0x0;
          uVar3->field4_0x6 = 0x0;
          uVar3->field5_0x8 = 0x0;
          uVar3->field6_0xa = 0x0;
          uVar3->field7_0xc = 0x0;
          paStack40->field0_0x0 = 0x56ce;
          uVar3->field2_0x2 = 0x1018;
        }
        uVar12 = (paStack40 >> 0x10);
        (paStack40 + 0x8) = iVar8->field15_0x14;
        (paStack40 + 0xa) = &iVar8->field16_0x16;
        uVar8 = pass1_1020_c42e(iVar8->field15_0x14);
      }
      else {
        pass1_1030_7c50(0x0,(astruct_57 *)CONCAT22(in_register_0000000a,param_1),(astruct_305 *)param_3,
                        iVar8->field16_0x16,iVar8->field14_0x12);
        paStack40 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar8 = (paStack40 >> 0x10);
        uVar4 = (astruct_619 *)paStack40;
        if ((uVar8 | uVar4) == 0x0) {
          paStack40 = NULL;
        }
        else {
          paStack40->field0_0x0 = 0x389a;
          uVar4->field2_0x2 = 0x1008;
          uVar4->field3_0x4 = 0x0;
          uVar4->field4_0x6 = 0x0;
          uVar4->field5_0x8 = 0x0;
          uVar4->field6_0xa = 0x0;
          uVar4->field7_0xc = 0x0;
          paStack40->field0_0x0 = 0x56ce;
          uVar4->field2_0x2 = 0x1018;
        }
        uVar12 = (paStack40 >> 0x10);
        (paStack40 + 0x6) = iVar8->field14_0x12;
        (paStack40 + 0xa) = &iVar8->field16_0x16;
        uVar8 = switch_1020_c3b4(iVar8->field14_0x12);
      }
      uVar12 = (paStack40 >> 0x10);
      iVar10 = paStack40;
      lVar2 = uVar8 * (iVar10 + 0xa);
      pSVar9 = (StructD *)(lVar2 >> 0x10);
      uVar8 = lVar2;
    }
    else {
      paStack40 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
      uVar8 = (paStack40 >> 0x10);
      uVar5 = (astruct_620 *)paStack40;
      pSVar9 = (StructD *)(uVar8 | uVar5);
      if (pSVar9 == NULL) {
        paStack40 = NULL;
      }
      else {
        paStack40->field0_0x0 = 0x389a;
        uVar5->field2_0x2 = 0x1008;
        uVar5->field3_0x4 = 0x0;
        uVar5->field4_0x6 = 0x0;
        uVar5->field5_0x8 = 0x0;
        uVar5->field6_0xa = 0x0;
        uVar5->field7_0xc = 0x0;
        paStack40->field0_0x0 = 0x56ce;
        uVar5->field2_0x2 = 0x1018;
      }
      uVar12 = (paStack40 >> 0x10);
      iVar10 = paStack40;
      (iVar10 + 0x4) = iVar8->field12_0xc;
      uVar8 = &iVar8->field16_0x16;
      (iVar10 + 0xa) = uVar8;
    }
    (iVar10 + 0xc) = uVar8;
    pass1_1030_6a2c(uVar8,pSVar9,(astruct_382 *)param_3,(astruct_383 *)CONCAT22(uVar12,iVar10));
  }
  else {
    puVar1 = iVar8->field13_0xe;
    uStack4 = (puVar1 + 0x4);
    for (uStack12 = 0x0; uStack12 < uStack4; uStack12 += 0x1) {
      pass1_1020_bb16(iVar8->field13_0xe,CONCAT22(0x1050,&local_a),(u16 *)CONCAT22(0x1050,&local_6),uStack12
                     );
      if (local_a != 0x0) {
        paStack16 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar8 = (paStack16 >> 0x10);
        uVar6 = (astruct_621 *)paStack16;
        if ((uVar8 | uVar6) == 0x0) {
          paStack16 = NULL;
        }
        else {
          paStack16->field0_0x0 = 0x389a;
          uVar6->field2_0x2 = 0x1008;
          uVar6->field3_0x4 = 0x0;
          uVar6->field4_0x6 = 0x0;
          uVar6->field5_0x8 = 0x0;
          uVar6->field6_0xa = 0x0;
          uVar6->field7_0xc = 0x0;
          paStack16->field0_0x0 = 0x56ce;
          uVar6->field2_0x2 = 0x1018;
        }
        uVar12 = (paStack16 >> 0x10);
        iVar10 = paStack16;
        (iVar10 + 0x4) = local_6;
        (iVar10 + 0xa) = local_a;
        uVar7 = pass1_1020_c3ae();
        lVar2 = uVar7 * (iVar10 + 0xa);
        uVar8 = lVar2;
        (iVar10 + 0xc) = uVar8;
        pass1_1030_6a2c(uVar8,(StructD *)(lVar2 >> 0x10),(astruct_382 *)param_3,(astruct_383 *)paStack16);
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_7356(param_1: *mut astruct_615,param_2: *mut astruct_419)

{
  u8 **ppuVar1;
  let mut puVar2: *mut u16;
  let mut pcVar3: *mut c_char;
  i32 lVar4;
  let mut in_AX: u16;
  let mut BVar5: bool;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_DX: u16;
  u8 *puVar8;
  u8 *puVar9;
  astruct_615 *iVar9;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut bVar13: bool;
  let mut uVar14: u32;
  let mut uVar15: u32;
  astruct_99 *paStack50;
  astruct_99 *paStack26;
  astruct_616 *uVar8;
  astruct_622 *uVar10;

  uVar14 = struct_op_1030_73a8(param_2,in_AX,in_DX);
  puVar8 = (uVar14 >> 0x10);
  uVar6 = uVar14;
  puVar9 = puVar8;
  BVar5 = pass1_1008_c6ae(_u16_1050_06e0,(uVar6 + 0xc),0x4);
  iVar9 = (astruct_615 *)param_1;
  uVar11 = (param_1 >> 0x10);
  if (BVar5 == 0x0) {
    uVar7 = pass1_1008_c6ae(_u16_1050_06e0,(uVar6 + 0xc),0x3);
    if (uVar7 == 0x0) {
code_r0x10387545:
      pass1_1038_6f5a(uVar7,puVar9,param_1,param_2);
      goto LAB_1038_7549;
    }
    if ((iVar9->field9_0xc != 0x0) || (*(i32 *)&iVar9->field_0xe != 0x0)) {
      uVar14 = pass1_1028_45e2(uVar6,puVar9,uVar14);
      puVar9 = (uVar14 >> 0x10);
      uVar7 = uVar14;
      ppuVar1 = (u8 **)&iVar9->field16_0x18;
      bVar13 = *ppuVar1 < puVar9;
      if ((bVar13 || *ppuVar1 == puVar9) &&
         ((bVar13 || (puVar2 = &iVar9->field15_0x16, *puVar2 < uVar7 || *puVar2 == uVar7)))) goto code_r0x10387545;
    }
  }
  else {
    uVar15 = pass1_1028_62c8(uVar14);
    puVar9 = (uVar15 >> 0x10);
    uVar7 = uVar15;
    ppuVar1 = (u8 **)&iVar9->field16_0x18;
    bVar13 = *ppuVar1 < puVar9;
    if ((bVar13 || *ppuVar1 == puVar9) &&
       ((bVar13 || (puVar2 = &iVar9->field15_0x16, *puVar2 < uVar7 || *puVar2 == uVar7)))) {
      if (iVar9->field13_0x12 == 0x0) {
        if (iVar9->field14_0x14 == 0x0) goto LAB_1038_74e0;
        paStack50 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar6 = (paStack50 >> 0x10);
        uVar10 = (astruct_622 *)paStack50;
        if ((uVar6 | uVar10) == 0x0) {
          paStack50 = NULL;
        }
        else {
          paStack50->field0_0x0 = 0x389a;
          uVar10->field2_0x2 = 0x1008;
          uVar10->field3_0x4 = 0x0;
          uVar10->field4_0x6 = 0x0;
          uVar10->field5_0x8 = 0x0;
          uVar10->field6_0xa = 0x0;
          uVar10->field7_0xc = 0x0;
          paStack50->field0_0x0 = 0x56ce;
          uVar10->field2_0x2 = 0x1018;
        }
        uVar12 = (paStack50 >> 0x10);
        iVar10 = paStack50;
        (iVar10 + 0x8) = iVar9->field14_0x14;
        (iVar10 + 0xa) = iVar9->field15_0x16;
        uVar6 = pass1_1020_c42e(iVar9->field14_0x14);
      }
      else {
        paStack26 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar6 = (paStack26 >> 0x10);
        uVar8 = (astruct_616 *)paStack26;
        if ((uVar6 | uVar8) == 0x0) {
          paStack26 = NULL;
        }
        else {
          paStack26->field0_0x0 = 0x389a;
          uVar8->field2_0x2 = 0x1008;
          uVar8->field3_0x4 = 0x0;
          uVar8->field4_0x6 = 0x0;
          uVar8->field5_0x8 = 0x0;
          uVar8->field6_0xa = 0x0;
          uVar8->field7_0xc = 0x0;
          paStack26->field0_0x0 = 0x56ce;
          uVar8->field2_0x2 = 0x1018;
        }
        uVar12 = (paStack26 >> 0x10);
        iVar10 = paStack26;
        (iVar10 + 0x6) = iVar9->field13_0x12;
        (iVar10 + 0xa) = iVar9->field15_0x16;
        uVar6 = switch_1020_c3b4(iVar9->field13_0x12);
      }
      lVar4 = uVar6 * (iVar10 + 0xa);
      puVar9 = (lVar4 >> 0x10);
      uVar7 = lVar4;
      (iVar10 + 0xc) = uVar7;
      pass1_1028_6408(uVar14,CONCAT22(uVar12,iVar10));
      goto LAB_1038_7549;
    }
  }//
LAB_1038_74e0:
  pass1_1038_709c(puVar9,(astruct_618 *)param_1,param_2);//
LAB_1038_7549:
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar9->field8_0x8);
  pass1_1030_6c4c(CONCAT22(puVar9,uVar7),(uVar7 + 0x34) + iVar9->field29_0x26);
  iVar9->field9_0xc = 0x0;
  iVar9->field13_0x12 = 0x0;
  iVar9->field14_0x14 = 0x0;
  &iVar9->field15_0x16 = 0x0;
  pcVar3 = *(char **)&iVar9->field_0xe;
  uVar6 = iVar9->field12_0x10;
  if ((uVar6 | pcVar3) != 0x0) {
    fn_ptr_1020_ba7e((pcVar3 & 0xffff | uVar6 << 0x10));
    fn_ptr_1000_17ce(pcVar3);
  }
  &iVar9->field_0xe = 0x0;
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1038_75ca(mut param_1: i16,mut param_2: u32,mut param_3: u32)

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  HFILE16 in_stack_0000ffca;
  u32 local_10 [0x2];
  let mut local_8: u32;

  uVar4 = (param_2 >> 0x10);
  iVar3 = param_2;
  pass1_1008_79f0(param_3,*(i32 *)(iVar3 + 0x4));
  if (param_1 != 0x0) {
    local_10[0] = (iVar3 + 0x8);
    BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_10),0x4,in_stack_0000ffca);
    if (BVar1 != 0x0) {
      write_to_file_1008_7a22(param_3,*(i32 *)(iVar3 + 0xe));
      if (BVar1 != 0x0) {
        local_8 = (iVar3 + 0xc);
        BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
        if (BVar1 != 0x0) {
          local_8 = (iVar3 + 0x12);
          BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
          if (BVar1 != 0x0) {
            local_8 = CONCAT22(local_8,(iVar3 + 0x14));
            BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
            if (BVar1 != 0x0) {
              local_8 = (iVar3 + 0x16);
              BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffca);
              if (BVar1 != 0x0) {
                iVar2 = write_to_file_1008_7b4c(param_3,(astruct_615 *)(param_2 & 0xffff0000 | (iVar3 + 0x1a)));
                if (iVar2 != 0x0) {
                  local_8 = (iVar3 + 0x20);
                  BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffca)
                  ;
                  if (BVar1 != 0x0) {
                    local_8 = local_8 & 0xffff0000 | (iVar3 + 0x24);
                    BVar1 = write_to_file_1008_7e1c
                                      (param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
                    if (BVar1 != 0x0) {
                      local_8 = local_8 & 0xffff0000 | (iVar3 + 0x26);
                      BVar1 = write_to_file_1008_7e1c
                                        (param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
                      if (BVar1 != 0x0) {
                        local_8 = local_8 & 0xffff0000 | (iVar3 + 0x28);
                        BVar1 = write_to_file_1008_7e1c
                                          (param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
                        if (BVar1 != 0x0) {
                          return;
                        }
                      }
                    }
                  }
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
pub fn file_1038_774e(u8 *param_1,param_2: *mut astruct_169,mut param_3: u32)

{
  astruct_169 *iVar2;
  let mut BVar1: bool;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar6: u16;
  let mut local_8: u16;
  let mut local_6: u16;
  let mut local_4: u16;
  astruct_169 *paVar5;

  if (u16_1050_0312 < 0x2) {
    return;
  }
  iVar2 = (astruct_169 *)param_2;
  iVar2 = (astruct_169 *)&iVar2->field4_0x4;
  paVar5 = (astruct_169 *)(param_2 & 0xffff0000 | ZEXT24(iVar2));
  pass1_1008_766e(param_1,param_3,paVar5);
  if (((((paVar5 != 0x0) &&
        (BVar1 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | ZEXT24(iVar2 + 0x1)),0x4),
        BVar1 != 0x0)) &&
       (iVar3 = file_1008_77cc(param_1,param_3,
                               (i32 *)(param_2 & 0xffff0000 |
                                       ZEXT24((&iVar2[0x1].field4_0x4 + 0x2)))), iVar3 != 0x0)) &&
      ((((BVar1 = read_file_1008_7dee((HFILE16 *)param_3,CONCAT22(0x1050,&local_4),0x2), BVar1 != 0x0 &&
         (BVar1 = read_file_1008_7dee((HFILE16 *)param_3,CONCAT22(0x1050,&local_6),0x2), BVar1 != 0x0)) &&
        ((BVar1 = read_file_1008_7dee((HFILE16 *)param_3,CONCAT22(0x1050,&local_8),0x2), BVar1 != 0x0 &&
         ((BVar1 = read_file_1008_7dee((HFILE16 *)param_3,
                                       (param_2 & 0xffff0000 |
                                             ZEXT24((&iVar2[0x2].field4_0x4 + 0x2))),0x4),
          BVar1 != 0x0 &&
          (BVar1 = read_file_1008_7bc8(param_3,(u16 *)(param_2 & 0xffff0000 | ZEXT24(&iVar2[0x3].field_0x2))),
          BVar1 != 0x0)))))) &&
       (BVar1 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | ZEXT24(iVar2 + 0x4)),0x4),
       BVar1 != 0x0)))) &&
     (((BVar1 = read_file_1008_7dee((HFILE16 *)param_3,
                                    (param_2 & 0xffff0000 | ZEXT24(&iVar2[0x4].field4_0x4)),0x2),
       BVar1 != 0x0 &&
       (BVar1 = read_file_1008_7dee((HFILE16 *)param_3,
                                    (param_2 & 0xffff0000 |
                                          ZEXT24((&iVar2[0x4].field4_0x4 + 0x2))),0x2), BVar1 != 0x0))
      && (BVar1 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | ZEXT24(iVar2 + 0x5)),0x2),
         BVar1 != 0x0)))) {
    uVar6 = (param_2 >> 0x10);
    &iVar2[0x1].field4_0x4 = local_4;
    uVar4 = switch_1008_72bc((HFILE16 *)param_3,local_6);
    &iVar2[0x2].field_0x2 = uVar4;
    &iVar2[0x2].field4_0x4 = local_8;
    return;
  }
  u16_1050_0310 = 0x6d2;
  return;
}



StructD * pass1_1038_78b8(StructD *param_1,param_2: u8)

{
  pass1_1038_6912(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_78e2(StructD *param_1,param_2: *mut astruct_431)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  astruct_431 *iVar4;
  let mut uVar5: u16;
  let mut paVar4: *mut Struct57;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar5 = (param_2 >> 0x10);
  iVar4 = (astruct_431 *)param_2;
  uVar1 = 0x0;
  param_2 = 0x0;
  &iVar4->field2_0x4 = 0x0;
  _PTR_LOOP_1050_5a64 = param_2;
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | uVar1;
  paVar4 = (astruct_57 *)(paVar3 & 0xffff0000 | uVar2);
  if (uVar2 == 0x0) {
    param_2 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar3,uVar1));
    param_2->field0_0x0 = uVar1;
    iVar4->field1_0x2 = paVar4;
  }
  mem_op_1000_179c(0xc,paVar4);
  uVar2 = paVar4 | uVar1;
  if (uVar2 == 0x0) {
    uVar1 = 0x0;
    uVar2 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar4,uVar1));
  }
  iVar4->field2_0x4 = uVar1;
  iVar4->field3_0x6 = uVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_7964(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u32;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  _PTR_LOOP_1050_5a64 = 0x0;
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar1 = (iVar4 + 0x2);
  if ((uVar1 | *param_1) != 0x0) {
    ppcVar3 = (code **)*param_1;
    (**ppcVar3)();
  }
  puVar2 = (iVar4 + 0x4);
  uVar1 = (iVar4 + 0x6);
  if ((uVar1 | puVar2) != 0x0) {
    ppcVar3 = (code **)*puVar2;
    (**ppcVar3)();
  }
  return;
}
pub fn pass1_1038_79b2(mut param_1: u16 ,u8 *param_2,mut param_3: u32,mut param_4: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut uVar4: u16;
  let mut uVar5: u16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar5 = 0x1000;
  mem_op_1000_179c(0x14,paVar3);
  uVar2 = paVar3 | param_1;
  if (uVar2 == 0x0) {
    param_1 = 0x0;
    uVar2 = 0x0;
  }
  else {
    uVar5 = 0x1030;
    pass1_1030_aefa((astruct_400 *)CONCAT22(paVar3,param_1),param_4);
  }
  uVar4 = (param_3 >> 0x10);
  ppcVar1 = (code **)((param_3 + 0x4) + 0x4);
  (**ppcVar1)(uVar5,(param_3 + 0x4),param_1,uVar2);
  return;
}
pub fn pass1_1038_79f2(mut param_1: u32,mut param_2: u32,mut param_3: u16 )

{
  code **ppcVar1;
  u8 *puVar2;
  let mut extraout_DX: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  u8 local_e [0x8];
  i32 lStack6;

  lStack6 = *(i32 *)(param_2 + 0x4);
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  pass1_1008_5784(CONCAT22(0x1050,local_e),(iVar3 + 0x4));
  do {
    puVar2 = local_e;
    pass1_1008_5b12(CONCAT22(0x1050,puVar2));
    if ((extraout_DX | puVar2) == 0x0) {
      return;
    }
  } while (*(i32 *)(puVar2 + 0x4) != lStack6);
  ppcVar1 = (code **)((iVar3 + 0x4) + 0xc);
  (**ppcVar1)(0x1008,(iVar3 + 0x4),puVar2,extraout_DX);
  return;
}
pub fn pass1_1038_7a5a(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)(*param_1 + 0x4);
  (**ppcVar1)();
  return;
}
pub fn pass1_1038_7a76(u32 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  astruct_615 *paVar3;
  let mut uVar4: u32;
  u8 local_a [0x4];
  let mut uStack6: u32;

  pass1_1008_5784(CONCAT22(0x1050,local_a),*param_1);
  while( true ) {
    paVar3 = (astruct_615 *)pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (paVar3 == NULL) break;
    pass1_1038_6a0e((astruct_419 *)CONCAT22(paVar3,(paVar3 >> 0x10) | paVar3),paVar3);
  }
  do {
    uStack6 = 0x0;
    do {
      uVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
      if (uVar4 == 0x0) {
        pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x4));
        while( true ) {
          uVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
          if (uVar4 == 0x0) break;
          pass1_1030_affc(uVar4);
        }
        return;
      }
      uVar2 = pass1_1038_6b3c(uVar4);
    } while (uVar2 == 0x0);
    ppcVar1 = (code **)(*param_1 + 0xc);
    (**ppcVar1)(0x1008);
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps

u16 pass1_1038_7b20(u32 *param_1,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u32;
  HFILE16 in_stack_0000ffce;
  let mut local_1c: u16;
  let mut uStack26: u16;
  let mut uStack24: u16;
  let mut uStack16: u32;
  u8 local_c [0x8];
  let mut local_4: u16;

  BVar2 = write_to_file_1008_7cac(param_2);
  if (BVar2 != 0x0) {
    local_1c = (*param_1 + 0x8);
    local_4 = local_1c;
    BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_1c),0x2,in_stack_0000ffce);
    if (BVar2 != 0x0) {
      pass1_1008_5784(CONCAT22(0x1050,local_c),*param_1);
      do {
        uStack16 = pass1_1008_5b12(CONCAT22(0x1050,local_c));
        if (uStack16 == 0x0) {
          uVar3 = (param_1 >> 0x10);
          uVar1 = (param_1 + 0x4);
          local_1c = (uVar1 + 0x8);
          local_4 = local_1c;
          BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_4),0x2,in_stack_0000ffce);
          if (BVar2 == 0x0) {
            return 0x0;
          }
          pass1_1008_5784(CONCAT22(0x1050,local_c),(param_1 + 0x4));
          do {
            uVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_c));
            uStack26 = uVar4;
            if (uVar4 == 0x0) {
              return 0x1;
            }
            pass1_1030_b768(uVar4,param_2);
            uStack24 = (uVar4 >> 0x10);
          } while (uVar4 != 0x0);
          return 0x0;
        }
        pass1_1038_75ca(uStack16,uStack16,param_2);
        uStack16 = (uStack16 >> 0x10);
      } while (uStack16 != 0x0);
    }
  }
  return 0x0;
}



u16 read_file_1038_7c02(u16_t param_1,u16_t param_2,u32 *param_3,HFILE16 *param_4)

{
  code **ppcVar1;
  let mut BVar2: bool;
  let mut uVar3: u16;
  u8 *puVar4;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  let mut paVar8: *mut Struct57;
  HFILE16 *pHVar9;
  u16 local_12 [0x2];
  let mut uStack14: u32;
  let mut local_4: u16;

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if (u16_1050_0312 < 0x2) {
    return 0x1;
  }
  read_file_1008_7cfe(param_4,(param_4 >> 0x10),0x17);
  if ((param_1 != 0x0) && (BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2), BVar2 != 0x0)) {
    while (local_4 != 0x0) {
      uVar3 = local_4;
      local_4 = local_4 - 0x1;
      pHVar9 = param_4;
      mem_op_1000_179c(0x2a,paVar7);
      uVar5 = paVar7;
      uVar6 = uVar5 | uVar3;
      paVar7 = (astruct_57 *)(paVar7 & 0xffff0000);
      paVar8 = (astruct_57 *)(paVar7 | uVar6);
      if (uVar6 == 0x0) {
        uVar3 = 0x0;
      }
      else {
        struct_1038_6520((astruct_308 *)CONCAT22(uVar5,uVar3));
        paVar7 = paVar8;
      }
      puVar4 = paVar7;
      uStack14 = CONCAT22(puVar4,uVar3);
      file_1038_774e(puVar4,(astruct_169 *)CONCAT22(puVar4,uVar3),pHVar9);
      if (uVar3 == 0x0) {
        return 0x0;
      }
      ppcVar1 = (code **)(*param_3 + 0x4);
      (**ppcVar1)();
    }
    local_4 = local_4 - 0x1;
    BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_12),0x2);
    if (BVar2 != 0x0) {
      while( true ) {
        if (local_12[0] == 0x0) {
          return 0x1;
        }
        uVar3 = local_12[0];
        local_12[0] = local_12[0] - 0x1;
        pHVar9 = param_4;
        mem_op_1000_179c(0x14,paVar7);
        uVar5 = paVar7;
        uVar6 = uVar5 | uVar3;
        paVar7 = (astruct_57 *)(paVar7 & 0xffff0000);
        paVar8 = (astruct_57 *)(paVar7 | uVar6);
        if (uVar6 == 0x0) {
          uVar3 = 0x0;
        }
        else {
          pass1_1030_ae6c((astruct_399 *)CONCAT22(uVar5,uVar3));
          paVar7 = paVar8;
        }
        file_1030_b836(paVar7,(astruct_401 *)CONCAT22(paVar7,uVar3),pHVar9);
        if (uVar3 == 0x0) break;
        ppcVar1 = (code **)((param_3 + 0x4) + 0x4);
        (**ppcVar1)();
      }
      return 0x0;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * pass1_1038_7d10(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar1: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar1: *mut Struct57;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082(param_2,CONCAT22(param_3,0x1853));
  uVar1 = (astruct_57 *)(param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  &iVar1[0x1].field3_0x6 = 0x0;
  param_2->field0_0x0 = 0x8876;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x40),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field3_0x6 = puVar2;
  iVar1[0x1].field4_0x8 = (puVar2 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_7d5c(StructD *param_1)

{
  let mut uVar1: u16;
  let mut in_stack_0000ffde: u16;

  uVar1 = (param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x8876;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}
pub fn destroy_window_1038_7d88(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  pass1_1008_b544(param_3,(param_1 + 0x94),param_2);
  DestroyWindow16(*(HWND16 *)(param_1 + 0x6));
  return;
}



LRESULT pass1_1038_7dac(param_1: *mut astruct_903,mut param_2: u16 )

{
  let mut LVar1: LRESULT;

  pass1_1040_78de();
  LVar1 = send_dlg_item_msg_1038_844a(param_1);
  return LVar1;
}
pub fn pass1_1038_7dc6(u8 *param_1,astruct_903 *pstruct903_param_2,mut param_3: u16 ,mut param_4: u32)

{
  let mut bVar1: bool;
  let mut LVar2: LRESULT;
  let mut uVar3: u32;

  bVar1 = false;
  if (param_4 == 0x1854) {
    if (param_4 != 0x1) goto LAB_1038_7e8c;
    send_dlg_item_msg_1038_8618s(param_1,pstruct903_param_2);
  }
  else {
    if (param_4 < 0x18550000) {
      if (param_4 == 0xeb) {
        LVar2 = send_dlg_item_msg_1038_844a(pstruct903_param_2);
        param_1 = (LVar2 >> 0x10);
      }
      else if (param_4 == 0xfb) {
        LVar2 = send_dlg_item_msg_1038_7eac(pstruct903_param_2);
        param_1 = (LVar2 >> 0x10);
      }
      else {
        if (param_4 != s_vrpal_bmp_1050_183a + 0x7) {//
LAB_1038_7e77:
          pass1_1040_b54a(param_1,pstruct903_param_2,param_3,param_4);
          return;
        }
        msg_box_op_1038_81be(0x0,param_1,pstruct903_param_2);
      }
      goto LAB_1038_7e8c;
    }
    if (param_4 == 0x1855) {
      if (param_4 != 0x1) goto LAB_1038_7e8c;
      send_dlg_item_msg_1038_87b2(param_1,pstruct903_param_2);
    }
    else if (param_4 == 0x1856) {
      if (param_4 != 0x1) goto LAB_1038_7e8c;
      pass1_1038_8810(pstruct903_param_2);
    }
    else if (param_4 == 0x1858) {
      send_dlg_item_msg_1038_7fae(0x0,param_1,pstruct903_param_2);
    }
    else {
      if (param_4 != 0x1859) goto LAB_1038_7e77;
      uVar3 = pass1_1038_801a(param_1,pstruct903_param_2);
      param_1 = (uVar3 >> 0x10);
    }
  }
  bVar1 = true;//
LAB_1038_7e8c:
  if (bVar1) {
    set_win_text_1038_8358(param_1,pstruct903_param_2);
    enable_win_1038_806a(param_1,(astruct_902 *)pstruct903_param_2);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT send_dlg_item_msg_1038_7eac(param_1: *mut astruct_903)

{
  let mut in_EDX: *mut Struct57;
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut l_param: *mut c_char;
  let mut LVar4: LRESULT;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000fff2: u16;

  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0x30),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  l_param = pass1_1010_375e(puVar3);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  pass1_1008_b1a6((iVar1 + 0x94),l_param);
  SendDlgItemMessage16(0x0,0x0,0xb,0x1854,*(HWND16 *)(iVar1 + 0x6));
  LVar4 = SendDlgItemMessage16(0x0,0x0,0x409,0x1854,*(HWND16 *)(iVar1 + 0x6));
  if (((WPARAM16)LVar4 != 0xffff) || ((LVar4 >> 0x10) != -0x1)) {
    SendDlgItemMessage16(0x0,(WPARAM16)LVar4,0x403,0x1854,*(HWND16 *)(iVar1 + 0x6));
    SendDlgItemMessage16((LPARAM)l_param,0x0,0x401,0x1854,*(HWND16 *)(iVar1 + 0x6));
    SendDlgItemMessage16(0x0,0xffff,0x407,0x1854,*(HWND16 *)(iVar1 + 0x6));
    SendDlgItemMessage16(0x0,0x0,0x405,0x1855,*(HWND16 *)(iVar1 + 0x6));
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x405,0x1856,*(HWND16 *)(iVar1 + 0x6));
    enable_win_1038_806a((LVar4 >> 0x10),(astruct_902 *)param_1);
  }
  LVar4 = SendDlgItemMessage16(0x0,0x1,0xb,0x1854,*(HWND16 *)(iVar1 + 0x6));
  return LVar4;
}
pub fn send_dlg_item_msg_1038_7fae(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut LVar3: LRESULT;

  uVar2 = (param_3 >> 0x10);
  iVar1 = param_3;
  pass1_1008_b146(param_1,param_2,(iVar1 + 0x94));
  SendDlgItemMessage16(0x0,0xffff,0x407,0x1855,*(HWND16 *)(iVar1 + 0x6));
  LVar3 = SendDlgItemMessage16(0x0,0xffff,0x407,0x1856,*(HWND16 *)(iVar1 + 0x6));
  pass1_1008_b61a(LVar3,(LVar3 >> 0x10),(iVar1 + 0x94),0x0);
  pass1_1008_b63a((iVar1 + 0x94),0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_801a(mut param_1: u16 ,param_2: *mut astruct_903) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  astruct_477 *paVar6;
  let mut pcVar7: *mut c_char;
  let mut uVar8: u32;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000fff0: u16;
  let mut uVar3: u32;

  paVar6 = (astruct_477 *)
           mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000fff0,0x30),in_stack_0000fe98,in_stack_0000ffbc,
                           in_stack_0000ffc2,in_stack_0000ffc6);
  uVar5 = (param_2 >> 0x10);
  uVar4 = param_2;
  pcVar7 = pass1_1008_b340((uVar4 + 0x94));
  uVar1 = pcVar7;
  uVar2 = (pcVar7 >> 0x10) | uVar1;
  uVar3 = uVar2;
  if (uVar2 != 0x0) {
    pass1_1010_3770(uVar2,paVar6,pcVar7);
    uVar8 = pass1_1038_af40(uVar4,uVar2,_PTR_LOOP_1050_5b7c,(uVar4 + 0x6),0x3);
    uVar3 = uVar8 >> 0x10;
    uVar1 = uVar8;
  }
  return CONCAT22(uVar3,uVar1);
}
