


pub unsafe fn pass1_1038_709c(param_1: *mut u8,param_2: *mut astruct_618,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  let mut lVar2: i32;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut pSVar9: *mut StructD;
  let mut in_register_0000000a: u16;
  let mut iVar8: *mut astruct_618;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut paStack40: *mut astruct_99;
  let mut paStack16: *mut astruct_99;
  let mut uStack12: u16;
  let mut local_a: i32;
  let mut local_6: u16;
  let mut uStack4: u16;
  let mut uVar3: *mut astruct_617;
  let mut uVar4: *mut astruct_619;
  let mut uVar5: *mut astruct_620;
  let mut uVar6: *mut astruct_621;

  uVar11 = (param_2 >> 0x10);
  iVar8 = param_2;
  if (((&iVar8.field13_0xe + 0x2) | &iVar8.field13_0xe) == 0) {
    if (iVar8.field12_0xc == 0) {
      if (iVar8.field14_0x12 == 0) {
        if (iVar8.field15_0x14 == 0) {
          return;
        }
        paStack40 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar8 = (paStack40 >> 0x10);
        uVar3 = paStack40;
        if ((uVar8 | uVar3) == 0) {
          paStack40 = null_mut();
        }
        else {
          paStack40.field0_0x0 = 0x389a;
          uVar3.field2_0x2 = 0x1008;
          uVar3.field3_0x4 = 0;
          uVar3.field4_0x6 = 0;
          uVar3.field5_0x8 = 0;
          uVar3.field6_0xa = 0;
          uVar3.field7_0xc = 0;
          paStack40.field0_0x0 = 0x56ce;
          uVar3.field2_0x2 = 0x1018;
        }
        uVar12 = (paStack40 >> 0x10);
        (paStack40 + 0x8) = iVar8.field15_0x14;
        (paStack40 + 0xa) = &iVar8.field16_0x16;
        uVar8 = pass1_1020_c42e(iVar8.field15_0x14);
      }
      else {
        pass1_1030_7c50(0x0,CONCAT22(in_register_0000000a,param_1),param_3,
                        iVar8.field16_0x16,iVar8.field14_0x12);
        paStack40 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar8 = (paStack40 >> 0x10);
        uVar4 = paStack40;
        if ((uVar8 | uVar4) == 0) {
          paStack40 = null_mut();
        }
        else {
          paStack40.field0_0x0 = 0x389a;
          uVar4.field2_0x2 = 0x1008;
          uVar4.field3_0x4 = 0;
          uVar4.field4_0x6 = 0;
          uVar4.field5_0x8 = 0;
          uVar4.field6_0xa = 0;
          uVar4.field7_0xc = 0;
          paStack40.field0_0x0 = 0x56ce;
          uVar4.field2_0x2 = 0x1018;
        }
        uVar12 = (paStack40 >> 0x10);
        (paStack40 + 0x6) = iVar8.field14_0x12;
        (paStack40 + 0xa) = &iVar8.field16_0x16;
        uVar8 = switch_1020_c3b4(iVar8.field14_0x12);
      }
      uVar12 = (paStack40 >> 0x10);
      iVar10 = paStack40;
      lVar2 = uVar8 * (iVar10 + 0xa);
      pSVar9 = (lVar2 >> 0x10);
      uVar8 = lVar2;
    }
    else {
      paStack40 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
      uVar8 = (paStack40 >> 0x10);
      uVar5 = paStack40;
      pSVar9 = (uVar8 | uVar5);
      if (pSVar9.is_null()) {
        paStack40 = null_mut();
      }
      else {
        paStack40.field0_0x0 = 0x389a;
        uVar5.field2_0x2 = 0x1008;
        uVar5.field3_0x4 = 0;
        uVar5.field4_0x6 = 0;
        uVar5.field5_0x8 = 0;
        uVar5.field6_0xa = 0;
        uVar5.field7_0xc = 0;
        paStack40.field0_0x0 = 0x56ce;
        uVar5.field2_0x2 = 0x1018;
      }
      uVar12 = (paStack40 >> 0x10);
      iVar10 = paStack40;
      (iVar10 + 0x4) = iVar8.field12_0xc;
      uVar8 = &iVar8.field16_0x16;
      (iVar10 + 0xa) = uVar8;
    }
    (iVar10 + 0xc) = uVar8;
    pass1_1030_6a2c(uVar8,pSVar9,param_3,CONCAT22(uVar12,iVar10));
  }
  else {
    puVar1 = iVar8.field13_0xe;
    uStack4 = (puVar1 + 0x4);
    for uStack12 in 0 .. uStack4 {
      pass1_1020_bb16(iVar8.field13_0xe,CONCAT22(0x1050,&local_a),CONCAT22(0x1050,&local_6),uStack12
                     );
      if (local_a != 0) {
        paStack16 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar8 = (paStack16 >> 0x10);
        uVar6 = paStack16;
        if ((uVar8 | uVar6) == 0) {
          paStack16 = null_mut();
        }
        else {
          paStack16.field0_0x0 = 0x389a;
          uVar6.field2_0x2 = 0x1008;
          uVar6.field3_0x4 = 0;
          uVar6.field4_0x6 = 0;
          uVar6.field5_0x8 = 0;
          uVar6.field6_0xa = 0;
          uVar6.field7_0xc = 0;
          paStack16.field0_0x0 = 0x56ce;
          uVar6.field2_0x2 = 0x1018;
        }
        uVar12 = (paStack16 >> 0x10);
        iVar10 = paStack16;
        (iVar10 + 0x4) = local_6;
        (iVar10 + 0xa) = local_a;
        uVar7 = pass1_1020_c3ae();
        lVar2 = uVar7 * (iVar10 + 0xa);
        uVar8 = lVar2;
        (iVar10 + 0xc) = uVar8;
        pass1_1030_6a2c(uVar8,(lVar2 >> 0x10),param_3,paStack16);
      }
    }
  }
  return;
}




pub unsafe fn pass1_1038_7356(param_1: *mut astruct_615,param_2: *mut astruct_419)

{
  let mut ppuVar1: *mut *mut u8;
  let mut puVar2: *mut u16;
  let mut pcVar3: *mut c_char;
  let mut lVar4: i32;
  let mut in_AX: u16;
  let mut BVar5: bool;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_DX: u16;
  let mut puVar8: *mut u8;
  let mut puVar9: *mut u8;
  let mut iVar9: *mut astruct_615;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut bVar13: bool;
  let mut uVar14: u32;
  let mut uVar15: u32;
  let mut paStack50: *mut astruct_99;
  let mut paStack26: *mut astruct_99;
  let mut uVar8: *mut astruct_616;
  let mut uVar10: *mut astruct_622;

  uVar14 = struct_op_1030_73a8(param_2,in_AX,in_DX);
  puVar8 = (uVar14 >> 0x10);
  uVar6 = uVar14;
  puVar9 = puVar8;
  BVar5 = pass1_1008_c6ae(_u16_1050_06e0,(uVar6 + 0xc),0x4);
  iVar9 = param_1;
  uVar11 = (param_1 >> 0x10);
  if (BVar5 == 0) {
    uVar7 = pass1_1008_c6ae(_u16_1050_06e0,(uVar6 + 0xc),0x3);
    if (uVar7 == 0) {
// code_r0x10387545:
      pass1_1038_6f5a(uVar7,puVar9,param_1,param_2);
  // TODO: goto LAB_1038_7549;
    }
    if ((iVar9.field9_0xc != 0) || (&iVar9.field_0xe != 0)) {
      uVar14 = pass1_1028_45e2(uVar6,puVar9,uVar14);
      puVar9 = (uVar14 >> 0x10);
      uVar7 = uVar14;
      ppuVar1 = &iVar9.field16_0x18;
      bVar13 = *ppuVar1 < puVar9;
      if ((bVar13 || *ppuVar1 == puVar9) &&
         ((bVar13 || (puVar2 = &iVar9.field15_0x16, *puVar2 < uVar7 || *puVar2 == uVar7)))){
            // goto code_r0x10387545;
        }
    }
  }
  else {
    uVar15 = pass1_1028_62c8(uVar14);
    puVar9 = (uVar15 >> 0x10);
    uVar7 = uVar15;
    ppuVar1 = &iVar9.field16_0x18;
    bVar13 = *ppuVar1 < puVar9;
    if ((bVar13 || *ppuVar1 == puVar9) &&
       ((bVar13 || (puVar2 = &iVar9.field15_0x16, *puVar2 < uVar7 || *puVar2 == uVar7)))) {
      if (iVar9.field13_0x12 == 0) {
//        if (iVar9.field14_0x14 == 0) goto LAB_1038_74e0;
        paStack50 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar6 = (paStack50 >> 0x10);
        uVar10 = paStack50;
        if ((uVar6 | uVar10) == 0) {
          paStack50 = null_mut();
        }
        else {
          paStack50.field0_0x0 = 0x389a;
          uVar10.field2_0x2 = 0x1008;
          uVar10.field3_0x4 = 0;
          uVar10.field4_0x6 = 0;
          uVar10.field5_0x8 = 0;
          uVar10.field6_0xa = 0;
          uVar10.field7_0xc = 0;
          paStack50.field0_0x0 = 0x56ce;
          uVar10.field2_0x2 = 0x1018;
        }
        uVar12 = (paStack50 >> 0x10);
        iVar10 = paStack50;
        (iVar10 + 0x8) = iVar9.field14_0x14;
        (iVar10 + 0xa) = iVar9.field15_0x16;
        uVar6 = pass1_1020_c42e(iVar9.field14_0x14);
      }
      else {
        paStack26 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar6 = (paStack26 >> 0x10);
        uVar8 = paStack26;
        if ((uVar6 | uVar8) == 0) {
          paStack26 = null_mut();
        }
        else {
          paStack26.field0_0x0 = 0x389a;
          uVar8.field2_0x2 = 0x1008;
          uVar8.field3_0x4 = 0;
          uVar8.field4_0x6 = 0;
          uVar8.field5_0x8 = 0;
          uVar8.field6_0xa = 0;
          uVar8.field7_0xc = 0;
          paStack26.field0_0x0 = 0x56ce;
          uVar8.field2_0x2 = 0x1018;
        }
        uVar12 = (paStack26 >> 0x10);
        iVar10 = paStack26;
        (iVar10 + 0x6) = iVar9.field13_0x12;
        (iVar10 + 0xa) = iVar9.field15_0x16;
        uVar6 = switch_1020_c3b4(iVar9.field13_0x12);
      }
      lVar4 = uVar6 * (iVar10 + 0xa);
      puVar9 = (lVar4 >> 0x10);
      uVar7 = lVar4;
      (iVar10 + 0xc) = uVar7;
      pass1_1028_6408(uVar14,CONCAT22(uVar12,iVar10));
  // TODO: goto LAB_1038_7549;
    }
  }//
// LAB_1038_74e0:
  pass1_1038_709c(puVar9,param_1,param_2);//
// LAB_1038_7549:
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar9.field8_0x8);
  pass1_1030_6c4c(CONCAT22(puVar9,uVar7),(uVar7 + 0x34) + iVar9.field29_0x26);
  iVar9.field9_0xc = 0;
  iVar9.field13_0x12 = 0;
  iVar9.field14_0x14 = 0;
  iVar9.field15_0x16 = 0;
  pcVar3 = *&iVar9.field_0xe;
  uVar6 = iVar9.field12_0x10;
  if ((uVar6 | pcVar3) != 0) {
    fn_ptr_1020_ba7e((pcVar3 & 0xffff | uVar6 << 0x10));
    fn_ptr_1000_17ce(pcVar3);
  }
  iVar9.field_0xe = 0;
  return;
}




pub unsafe fn pass1_1038_75ca(mut param_1: i16,mut param_2: u32,mut param_3: u32)

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
   let mut in_stack_0000ffca: HFILE16;
  let mut local_10: [u32;0x2] = [0;0x2];
  let mut local_8: u32;

  uVar4 = (param_2 >> 0x10);
  iVar3 = param_2;
  pass1_1008_79f0(param_3,(iVar3 + 0x4));
  if (param_1 != 0) {
    local_10[0] = (iVar3 + 0x8);
    BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_10),0x4,in_stack_0000ffca);
    if (BVar1 != 0) {
      write_to_file_1008_7a22(param_3,(iVar3 + 0xe));
      if (BVar1 != 0) {
        local_8 = (iVar3 + 0xc);
        BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
        if (BVar1 != 0) {
          local_8 = (iVar3 + 0x12);
          BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
          if (BVar1 != 0) {
            local_8 = CONCAT22(local_8,(iVar3 + 0x14));
            BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
            if (BVar1 != 0) {
              local_8 = (iVar3 + 0x16);
              BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffca);
              if (BVar1 != 0) {
                iVar2 = write_to_file_1008_7b4c(param_3,(param_2 & 0xffff0000 | (iVar3 + 0x1a)));
                if (iVar2 != 0) {
                  local_8 = (iVar3 + 0x20);
                  BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffca)
                  ;
                  if (BVar1 != 0) {
                    local_8 = local_8 & 0xffff0000 | (iVar3 + 0x24);
                    BVar1 = write_to_file_1008_7e1c
                                      (param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
                    if (BVar1 != 0) {
                      local_8 = local_8 & 0xffff0000 | (iVar3 + 0x26);
                      BVar1 = write_to_file_1008_7e1c
                                        (param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
                      if (BVar1 != 0) {
                        local_8 = local_8 & 0xffff0000 | (iVar3 + 0x28);
                        BVar1 = write_to_file_1008_7e1c
                                          (param_3,CONCAT22(0x1050,&local_8),0x2,in_stack_0000ffca);
                        if (BVar1 != 0) {
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
pub unsafe fn file_1038_774e(param_1: *mut u8,param_2: *mut astruct_169,mut param_3: u32)

{
  let mut iVar2: *mut astruct_169;
  let mut BVar1: bool;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar6: u16;
  let mut local_8: u16;
  let mut local_6: u16;
  let mut local_4: u16;
  let mut paVar5: *mut astruct_169;

  if (u16_1050_0312 < 0x2) {
    return;
  }
  iVar2 = param_2;
  iVar2 = &iVar2.field4_0x4;
  paVar5 = (param_2 & 0xffff0000 | ZEXT24(iVar2));
  pass1_1008_766e(param_1,param_3,paVar5);
  if (((((paVar5 != 0) &&
        (BVar1 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | ZEXT24(iVar2 + 1)),0x4),
        BVar1 != 0)) &&
       (iVar3 = file_1008_77cc(param_1,param_3,
                               (param_2 & 0xffff0000 |
                                       ZEXT24((&iVar2[0x1].field4_0x4 + 0x2)))), iVar3 != 0)) &&
      ((((BVar1 = read_file_1008_7dee(param_3,CONCAT22(0x1050,&local_4),0x2), BVar1 != 0x0 &&
         (BVar1 = read_file_1008_7dee(param_3,CONCAT22(0x1050,&local_6),0x2), BVar1 != 0)) &&
        ((BVar1 = read_file_1008_7dee(param_3,CONCAT22(0x1050,&local_8),0x2), BVar1 != 0x0 &&
         ((BVar1 = read_file_1008_7dee(param_3,
                                       (param_2 & 0xffff0000 |
                                             ZEXT24((&iVar2[0x2].field4_0x4 + 0x2))),0x4),
          BVar1 != 0x0 &&
          (BVar1 = read_file_1008_7bc8(param_3,(param_2 & 0xffff0000 | ZEXT24(&iVar2[0x3].field_0x2))),
          BVar1 != 0)))))) &&
       (BVar1 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | ZEXT24(iVar2 + 0x4)),0x4),
       BVar1 != 0)))) &&
     (((BVar1 = read_file_1008_7dee(param_3,
                                    (param_2 & 0xffff0000 | ZEXT24(&iVar2[0x4].field4_0x4)),0x2),
       BVar1 != 0x0 &&
       (BVar1 = read_file_1008_7dee(param_3,
                                    (param_2 & 0xffff0000 |
                                          ZEXT24((&iVar2[0x4].field4_0x4 + 0x2))),0x2), BVar1 != 0))
      && (BVar1 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | ZEXT24(iVar2 + 0x5)),0x2),
         BVar1 != 0)))) {
    uVar6 = (param_2 >> 0x10);
    iVar2[0x1].field4_0x4 = local_4;
    uVar4 = switch_1008_72bc(param_3,local_6);
    iVar2[0x2].field_0x2 = uVar4;
    iVar2[0x2].field4_0x4 = local_8;
    return;
  }
  u16_1050_0310 = 0x6d2;
  return;
}



pub unsafe fn pass1_1038_78b8(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_6912(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}




pub unsafe fn pass1_1038_78e2(param_1: *mut StructD,param_2: *mut astruct_431)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut iVar4: *mut astruct_431;
  let mut uVar5: u16;
  let mut paVar4: *mut Struct57;

  paVar3 = CONCAT22(in_register_0000000a,param_1);
  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  uVar1 = 0;
  param_2 = 0;
  iVar4.field2_0x4 = 0;
  _PTR_LOOP_1050_5a64 = param_2;
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | uVar1;
  paVar4 = (paVar3 & 0xffff0000 | uVar2);
  if (uVar2 == 0) {
    param_2 = 0;
  }
  else {
    set_struct_1008_574a(CONCAT22(paVar3,uVar1));
    param_2.field0_0x0 = uVar1;
    iVar4.field1_0x2 = paVar4;
  }
  mem_op_1000_179c(0xc,paVar4);
  uVar2 = paVar4 | uVar1;
  if (uVar2 == 0) {
    uVar1 = 0;
    uVar2 = 0;
  }
  else {
    set_struct_1008_574a(CONCAT22(paVar4,uVar1));
  }
  iVar4.field2_0x4 = uVar1;
  iVar4.field3_0x6 = uVar2;
  return;
}




pub unsafe fn pass1_1038_7964(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u32;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  _PTR_LOOP_1050_5a64 = 0;
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar1 = (iVar4 + 2);
  if ((uVar1 | *param_1) != 0) {
    ppcVar3 = *param_1;
    (**ppcVar3)();
  }
  puVar2 = (iVar4 + 0x4);
  uVar1 = (iVar4 + 0x6);
  if ((uVar1 | puVar2) != 0) {
    ppcVar3 = *puVar2;
    (**ppcVar3)();
  }
  return;
}
pub unsafe fn pass1_1038_79b2(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32,mut param_4: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut uVar4: u16;
  let mut uVar5: u16;

  paVar3 = CONCAT22(in_register_0000000a,param_2);
  uVar5 = 0x1000;
  mem_op_1000_179c(0x14,paVar3);
  uVar2 = paVar3 | param_1;
  if (uVar2 == 0) {
    param_1 = 0;
    uVar2 = 0;
  }
  else {
    uVar5 = 0x1030;
    pass1_1030_aefa(CONCAT22(paVar3,param_1),param_4);
  }
  uVar4 = (param_3 >> 0x10);
  ppcVar1 = ((param_3 + 0x4) + 0x4);
  (**ppcVar1)(uVar5,(param_3 + 0x4),param_1,uVar2);
  return;
}
pub unsafe fn pass1_1038_79f2(mut param_1: u32,mut param_2: u32,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u8;
  let mut extraout_DX: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut local_e: [u8;0x8] = [0;0x8];
  let mut lStack6: i32;

  lStack6 = (param_2 + 0x4);
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  pass1_1008_5784(CONCAT22(0x1050,local_e),(iVar3 + 0x4));
  loop {
    puVar2 = local_e;
    pass1_1008_5b12(CONCAT22(0x1050,puVar2));
    if ((extraout_DX | puVar2) == 0) {
      return;
    }
    if !((puVar2 + 0x4) != lStack6) { break;}
  }
  ppcVar1 = ((iVar3 + 0x4) + 0xc);
  (**ppcVar1)(0x1008,(iVar3 + 0x4),puVar2,extraout_DX);
  return;
}
pub unsafe fn pass1_1038_7a5a(param_1: u32)

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = (*param_1 + 0x4);
  (**ppcVar1)();
  return;
}
pub unsafe fn pass1_1038_7a76(param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut paVar3: *mut astruct_615;
  let mut uVar4: u32;
  let mut local_a: [u8;0x4] = [0;0x4];
  let mut uStack6: u32;

  pass1_1008_5784(CONCAT22(0x1050,local_a),*param_1);
  loop {
    paVar3 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (paVar3.is_null()) { break; }
    pass1_1038_6a0e(CONCAT22(paVar3,(paVar3 >> 0x10) | paVar3),paVar3);
  }
  loop {
    uStack6 = 0;
    loop {
      uVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
      if (uVar4 == 0) {
        pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x4));
        loop {
          uVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
          if (uVar4 == 0) { break; }
          pass1_1030_affc(uVar4);
        }
        return;
      }
      uVar2 = pass1_1038_6b3c(uVar4);
      if uVar2 != 0 {break;}
    }
    ppcVar1 = (*param_1 + 0xc);
    (**ppcVar1)(0x1008);
  }
}





pub unsafe fn pass1_1038_7b20(param_1: u32,mut param_2: u32) -> u16

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u32;
   let mut in_stack_0000ffce: HFILE16;
  let mut local_1c: u16;
  let mut uStack26: u16;
  let mut uStack24: u16;
  let mut uStack16: u32;
  let mut local_c: [u8;0x8] = [0;0x8];
  let mut local_4: u16;

  BVar2 = write_to_file_1008_7cac(param_2);
  if (BVar2 != 0) {
    local_1c = (*param_1 + 0x8);
    local_4 = local_1c;
    BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_1c),0x2,in_stack_0000ffce);
    if (BVar2 != 0) {
      pass1_1008_5784(CONCAT22(0x1050,local_c),*param_1);
      loop {
        uStack16 = pass1_1008_5b12(CONCAT22(0x1050,local_c));
        if (uStack16 == 0) {
          uVar3 = (param_1 >> 0x10);
          uVar1 = (param_1 + 0x4);
          local_1c = (uVar1 + 0x8);
          local_4 = local_1c;
          BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_4),0x2,in_stack_0000ffce);
          if (BVar2 == 0) {
            return 0x0;
          }
          pass1_1008_5784(CONCAT22(0x1050,local_c),(param_1 + 0x4));
          loop {
            uVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_c));
            uStack26 = uVar4;
            if (uVar4 == 0) {
              return 0x1;
            }
            pass1_1030_b768(uVar4,param_2);
            uStack24 = (uVar4 >> 0x10);
            if uVar4 == 0 {break;}
          }
          return 0x0;
        }
        pass1_1038_75ca(uStack16,uStack16,param_2);
        uStack16 = (uStack16 >> 0x10);
        if uStack16 == 0 {break;}
      }
    }
  }
  return 0x0;
}



pub unsafe fn read_file_1038_7c02(param_1: u16,param_2: u16,param_3: *mut u32,param_4: *mut HFILE16) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut puVar4: *mut u8;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  let mut paVar8: *mut Struct57;
   let mut pHVar9: *mut HFILE16;
  let mut local_12: [u16;0x2] = [0;0x2];
  let mut uStack14: u32;
  let mut local_4: u16;

  paVar7 = CONCAT22(in_register_0000000a,param_2);
  if (u16_1050_0312 < 0x2) {
    return 0x1;
  }
  read_file_1008_7cfe(param_4,(param_4 >> 0x10),0x17);
  if ((param_1 != 0) && (BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2), BVar2 != 0)) {
    while (local_4 != 0) {
      uVar3 = local_4;
      local_4 = local_4 - 0x1;
      pHVar9 = param_4;
      mem_op_1000_179c(0x2a,paVar7);
      uVar5 = paVar7;
      uVar6 = uVar5 | uVar3;
      paVar7 = (paVar7 & 0xffff0000);
      paVar8 = (paVar7 | uVar6);
      if (uVar6 == 0) {
        uVar3 = 0;
      }
      else {
        struct_1038_6520(CONCAT22(uVar5,uVar3));
        paVar7 = paVar8;
      }
      puVar4 = paVar7;
      uStack14 = CONCAT22(puVar4,uVar3);
      file_1038_774e(puVar4,CONCAT22(puVar4,uVar3),pHVar9);
      if (uVar3 == 0) {
        return 0x0;
      }
      ppcVar1 = (*param_3 + 0x4);
      (**ppcVar1)();
    }
    local_4 = local_4 - 0x1;
    BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_12),0x2);
    if (BVar2 != 0) {
      loop {
        if (local_12[0] == 0) {
          return 0x1;
        }
        uVar3 = local_12[0];
        local_12[0] = local_12[0] - 0x1;
        pHVar9 = param_4;
        mem_op_1000_179c(0x14,paVar7);
        uVar5 = paVar7;
        uVar6 = uVar5 | uVar3;
        paVar7 = (paVar7 & 0xffff0000);
        paVar8 = (paVar7 | uVar6);
        if (uVar6 == 0) {
          uVar3 = 0;
        }
        else {
          pass1_1030_ae6c(CONCAT22(uVar5,uVar3));
          paVar7 = paVar8;
        }
        file_1030_b836(paVar7,CONCAT22(paVar7,uVar3),pHVar9);
        if (uVar3 == 0) { break; }
        ppcVar1 = ((param_3 + 0x4) + 0x4);
        (**ppcVar1)();
      }
      return 0x0;
    }
  }
  return 0x0;
}





pub unsafe fn pass1_1038_7d10(mut param_1: u16 ,param_2: *mut Struct57,mut param_3: u16 ) -> *mut Struct57

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

  paVar1 = CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082(param_2,CONCAT22(param_3,0x1853));
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  iVar1[0x1].field3_0x6 = 0;
  param_2.field0_0x0 = 0x8876;
  iVar1.field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x40),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field3_0x6 = puVar2;
  iVar1[0x1].field4_0x8 = (puVar2 >> 0x10);
  return param_2;
}




pub unsafe fn pass1_1038_7d5c(param_1: *mut StructD)

{
  let mut uVar1: u16;
  let mut in_stack_0000ffde: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x8876;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}


pub unsafe fn pass1_1038_801a(mut param_1: u16 ,param_2: *mut Struct903) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut paVar6: *mut astruct_477;
  let mut pcVar7: *mut c_char;
  let mut uVar8: u32;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000fff0: u16;
  let mut uVar3: u32;

  paVar6 =
           mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           CONCAT22(in_stack_0000fff0,0x30),in_stack_0000fe98,in_stack_0000ffbc,
                           in_stack_0000ffc2,in_stack_0000ffc6);
  uVar5 = (param_2 >> 0x10);
  uVar4 = param_2;
  pcVar7 = pass1_1008_b340((uVar4 + 0x94));
  uVar1 = pcVar7;
  uVar2 = (pcVar7 >> 0x10) | uVar1;
  uVar3 = uVar2;
  if (uVar2 != 0) {
    pass1_1010_3770(uVar2,paVar6,pcVar7);
    uVar8 = pass1_1038_af40(uVar4,uVar2,_PTR_LOOP_1050_5b7c,(uVar4 + 0x6),0x3);
    uVar3 = uVar8 >> 0x10;
    uVar1 = uVar8;
  }
  return CONCAT22(uVar3,uVar1);
}
