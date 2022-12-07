

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_9048(mut param_1: u32,mut param_2: i16,mut param_3: u32)

{
  let mut uVar1: u32;
  let mut ppcVar2: *mut *mut code;
  let mut BVar3: bool;
  let mut iVar4: i16;
  let mut piVar5: *mut i16;
  let mut uVar6: u32;
  let mut uVar7: u32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut in_EDX: u32;
  let mut paVar10: *mut Struct57;
  let mut uVar11: u16;
  let mut puVar12: *mut u32;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut puVar15: *mut u32;
  let mut puVar16: *mut u16;
  let mut uVar17: u32;
  let mut uVar18: u16;
  let mut uVar19: u16;
  let mut uVar20: u8;
  let mut uStack36: u32;
  let mut local_18: [u8;0x2] = [0;0x2];
  let mut local_16: i16;
  let mut local_14: i16;
  let mut local_12: i16;
  let mut iStack16: i16;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut uStack6: u16;
  let mut iStack4: i16;

  uVar11 = (in_EDX >> 0x10);
  iStack4 = 0x8 - (param_2 == 0);
  puVar15 = pass1_1008_c6fa(_u16_1050_06e0,iStack4);
  uStack6 = (puVar15 >> 0x10);
  paVar10 = CONCAT22(uVar11,uStack6);
  uVar9 = puVar15;
  uStack8 = uVar9;
  pass1_1038_4e78(uVar9,paVar10,param_3,puVar15);
  uStack12 = CONCAT22(paVar10,uVar9);
  uVar14 = 0x1008;
  puVar16 = pass1_1008_3e38(CONCAT22(0x1050,&local_12));
  uVar7 = puVar16 >> 0x10;
  uVar1 = (param_3 + 0x8);
  uVar13 = (uStack12 >> 0x10);
  uVar11 = SUB42(uStack12,0x0);
  ppcVar2 = (*uStack12 + 0x10);
  uVar6 = uVar1;
  (**ppcVar2)(0x1008,uVar11,uVar13);
  uVar6 = uVar6 & 0xffff | uVar7 << 0x10;
  uStack36 = 0;
  while( true ) {
    uVar9 = uVar7;
    if (uVar6 <= uStack36) {
      if (uStack12.is_null() == false) {
        ppcVar2 = *uStack12;
        (**ppcVar2)(uVar14,uStack12,(uStack12 >> 0x10),0x1,uVar11,uVar13,uStack12,uStack12);
      }
      return;
    }
    ppcVar2 = (*uStack12 + 0x4);
    uVar7 = uVar6;
    (**ppcVar2)();
    iVar4 = uVar7;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7 & 0xffff | uVar9 << 0x10);
    uVar8 = uVar9;
    pass1_1008_3f62(CONCAT22(0x1050,&local_12),CONCAT22(uVar9,iVar4 + 0xc));
    piVar5 = &local_12;
    uVar14 = 0x1008;
    pass1_1008_3eb4(CONCAT22(0x1050,piVar5),CONCAT22(0x1050,local_18),
                    CONCAT22(0x1050,&local_16),CONCAT22(0x1050,&local_14));
    uVar17 = struct_op_1030_73a8(CONCAT22(uVar9,iVar4),piVar5,uVar8);
    uVar7 = uVar17 >> 0x10;
    uVar9 = (uVar17 >> 0x10);
    iVar4 = (uVar17 + 0xc);
    if (iVar4 - 0x7aU < 0x6) break;//
// LAB_1030_91fa:
    uStack36 += 0x1;
  }
  uVar14 = 0x1030;
  uVar18 = param_1;
  uVar19 = (param_1 >> 0x10);
  switch(iVar4) {
  default:
    iStack16 = local_16 + -0x1;
    BVar3 = pass1_1030_8fe4(&local_12,uVar9,uVar18,uVar19,CONCAT22(0x1050,&local_12),uVar1);
    if (BVar3 != 0) goto LAB_1030_91cb;
    iStack16 = local_16 + 1;
    BVar3 = pass1_1030_8fe4(&local_12,uVar7,uVar18,uVar19,CONCAT22(0x1050,&local_12),uVar1);
    if (BVar3 == 0) {
      iStack16 = local_16;
      local_12 = local_14 + -0x1;
      BVar3 = pass1_1030_8fe4(&local_12,uVar7,uVar18,uVar19,CONCAT22(0x1050,&local_12),uVar1);
  // TODO: goto joined_r0x1030911e;
    }//
// LAB_1030_9144:
    break;
  case 0x7b:
  case 0x7e:
    iStack16 = local_16 + -0x1;
    BVar3 = pass1_1030_8fe4(&local_12,uVar9,uVar18,uVar19,CONCAT22(0x1050,&local_12),uVar1);
    if (BVar3 == 0) {
      iStack16 = local_16 + 1;
  // TODO: goto LAB_1030_912c;
    }
    if (uStack12.is_null()) {
      return;
    }
    uVar14 = (uStack12 >> 0x10);
    puVar12 = uStack12;
    uVar20 = (uStack12 >> 0x10);
// TODO: goto LAB_1030_90e6;
  case 0x7c:
  case 0x7d:
    local_12 = local_14 + -0x1;
    BVar3 = pass1_1030_8fe4(&local_12,uVar9,uVar18,uVar19,CONCAT22(0x1050,&local_12),uVar1);
joined_r0x1030911e:
    if (BVar3 == 0) {
      local_12 = local_14 + 1;//
// LAB_1030_912c:
      BVar3 = pass1_1030_8fe4(&local_12,uVar7,uVar18,uVar19,CONCAT22(0x1050,&local_12),uVar1);
      if (BVar3 != 0) goto LAB_1030_9144;
  // TODO: goto LAB_1030_91fa;
    }//
// LAB_1030_91cb:
  }
  puVar12 = uStack12;
  if ((uStack12 | puVar12) != 0) {
    uVar14 = (uStack12 >> 0x10);
    uVar20 = (uStack12 >> 0x10);//
// LAB_1030_90e6:
    ppcVar2 = *puVar12;
    (**ppcVar2)(0x1030,puVar12,uVar20,0x1,uVar11,uVar13,uStack12,uStack12);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_9296(mut param_1: u16 ,mut param_2: u32,param_3: *mut u32,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u8;
  in_EAX: *mut astruct_99;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  iVar11: *mut astruct_99;
  pstruct99_10: *mut astruct_99;
  iVar9: *mut astruct_99;
  let mut in_stack_0000fe6a: u16;
  let mut in_stack_0000ff8e: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ffc2: u16;
  let mut local_3a: [u8;0xc] = [0;0xc];
  let mut uStack46: u32;
  let mut uStack36: u32;
  let mut uStack30: u32;
  let mut uStack26: u16;
  pstruct99_18: *mut astruct_99;
  let mut uStack14: u32;
  let mut puStack10: *mut u32;
  paStack6: *mut astruct_99;
  uVar5: *mut astruct_99;
  let mut fn_ptr_1: *mut *mut code;

  pass1_1038_53ba(param_4,1);
  uVar4 = param_1 | in_EAX;
  if (uVar4 != 0) {
    uStack30 = _PTR_LOOP_1050_5768;
    uVar3 = _PTR_LOOP_1050_5768;
    pstruct99_18 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar6 = (pstruct99_18 >> 0x10);
    in_EAX = (uVar3 & 0xffff0000 | pstruct99_18 & 0xffff);
    uVar4 = uVar6 | (pstruct99_18 & 0xffff);
    if (uVar4 == 0) {
      paStack6 = NULL;
    }
    else {
      iVar11 = pstruct99_18;
      pstruct99_18.field0_0x0 = 0x389a;
      iVar11.field1_0x2 = 0x1008;
      &iVar11.field2_0x4 = 0x73;
      pstruct99_18.field0_0x0 = 0x9ec8;
      iVar11.field1_0x2 = 0x1030;
      in_EAX = pstruct99_18;
      paStack6 = pstruct99_18;
    }
    fn_ptr_1 = (*param_3 + 0x4);
    (**fn_ptr_1)(0x1000,param_3,paStack6,(paStack6 >> 0x10));
  }
  pass1_1038_53ba(param_4,0x2);
  uVar4 |= in_EAX;
  if (uVar4 != 0) {
    uStack30 = _PTR_LOOP_1050_5768;
    uVar3 = _PTR_LOOP_1050_5768;
    pstruct99_18 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar6 = (pstruct99_18 >> 0x10);
    in_EAX = (uVar3 & 0xffff0000 | pstruct99_18 & 0xffff);
    uVar4 = uVar6 | (pstruct99_18 & 0xffff);
    if (uVar4 == 0) {
      paStack6 = NULL;
    }
    else {
      pstruct99_10 = pstruct99_18;
      pstruct99_18.field0_0x0 = 0x389a;
      pstruct99_10.field1_0x2 = 0x1008;
      &pstruct99_10.field2_0x4 = 0x74;
      pstruct99_18.field0_0x0 = 0x9ec8;
      pstruct99_10.field1_0x2 = 0x1030;
      in_EAX = pstruct99_18;
      paStack6 = pstruct99_18;
    }
    fn_ptr_1 = (*param_3 + 0x8);
    (**fn_ptr_1)(0x1000,param_3,paStack6,(paStack6 >> 0x10));
  }
  pass1_1038_53ba(param_4,0x3);
  uVar4 |= in_EAX;
  paVar7 = CONCAT22(in_register_0000000a,uVar4);
  if (uVar4 != 0) {
    uStack36 = _PTR_LOOP_1050_5768;
    uVar3 = _PTR_LOOP_1050_5768;
    pstruct99_18 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    in_EAX = (uVar3 & 0xffff0000 | pstruct99_18 & 0xffff);
    uVar4 = (pstruct99_18 >> 0x10);
    uVar6 = uVar4 | (pstruct99_18 & 0xffff);
    paVar7 = (paVar7 & 0xffff0000 | uVar6);
    if (uVar6 == 0) {
      paStack6 = NULL;
    }
    else {
      iVar9 = pstruct99_18;
      pstruct99_18.field0_0x0 = 0x389a;
      iVar9.field1_0x2 = 0x1008;
      &iVar9.field2_0x4 = 0x75;
      pstruct99_18.field0_0x0 = 0x9ec8;
      iVar9.field1_0x2 = 0x1030;
      in_EAX = pstruct99_18;
      paStack6 = pstruct99_18;
    }
    fn_ptr_1 = (*param_3 + 0x8);
    (**fn_ptr_1)(0x1000,param_3,paStack6,(paStack6 >> 0x10));
  }
  pass1_1030_8f04(in_EAX,paVar7,param_2,(param_2 >> 0x10),param_4);
  if (in_EAX != 0) {
    uStack36 = _PTR_LOOP_1050_5768;
    pstruct99_18 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar5 = pstruct99_18;
    uVar4 = (pstruct99_18 >> 0x10);
    paVar7 = (paVar7 & 0xffff0000 | (uVar4 | uVar5));
    if ((uVar4 | uVar5) == 0) {
      paStack6 = NULL;
    }
    else {
      pstruct99_18.field0_0x0 = 0x389a;
      uVar5.field1_0x2 = 0x1008;
      &uVar5.field2_0x4 = 0x37;
      pstruct99_18.field0_0x0 = 0x9ec8;
      uVar5.field1_0x2 = 0x1030;
      paStack6 = pstruct99_18;
    }
    fn_ptr_1 = (*param_3 + 0x8);
    (**fn_ptr_1)(0x1000,param_3,paStack6,(paStack6 >> 0x10));
  }
  puStack10 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,CONCAT22(in_stack_0000ffc2,0x8),in_stack_0000fe6a,
                              in_stack_0000ff8e,in_stack_0000ff94,in_stack_0000ff98);
  uVar1 = (puStack10 >> 0x10);
  uStack14 = (puStack10 + 0xe);
  uVar4 = (puStack10 + 0x10);
  uVar3 = uVar4;
  if ((uVar4 | uStack14) != 0) {
    pass1_1008_5784(CONCAT22(0x1050,local_3a),uStack14 & 0xffff | uVar4 << 0x10);
    while( true ) {
      uVar4 = uVar3;
      puVar2 = local_3a;
      pass1_1008_5b12(CONCAT22(0x1050,puVar2));
      uStack46 = CONCAT22(uVar4,puVar2);
      uVar3 = (uVar4 | puVar2);
      if ((uVar4 | puVar2) == 0) break;
      if (((puVar2 + 0x4) == 0x3e) || ((puVar2 + 0x4) == 0x41)) {
        uStack30 = _PTR_LOOP_1050_5768;
        pstruct99_18 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
        uVar6 = (pstruct99_18 >> 0x10);
        uVar4 = pstruct99_18;
        uVar3 = (uVar6 | uVar4);
        if ((uVar6 | uVar4) == 0) {
          paStack6 = NULL;
        }
        else {
          uStack26 = (uStack46 + 0x4);
          pstruct99_18.field0_0x0 = 0x389a;
          (uVar4 + 0x2) = 0x1008;
          (uVar4 + 0x4) = uStack26;
          pstruct99_18.field0_0x0 = 0x9ec8;
          (uVar4 + 0x2) = 0x1030;
          paStack6 = pstruct99_18;
        }
        fn_ptr_1 = (*param_3 + 0x8);
        (**fn_ptr_1)(0x1000,param_3,paStack6,(paStack6 >> 0x10));
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_951a(mut param_1: u16 ,mut param_2: u32,param_3: *mut u32,mut param_4: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut in_register_0000000a: u16;
  let mut paVar10: *mut Struct57;
  let mut iVar11: i16;
  let mut puVar12: *mut u16;
  let mut unaff_SI: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut uVar15: u8;
  let mut puVar16: *mut u32;
  let mut uVar17: u32;
  let mut in_stack_0000fe48: u16;
  let mut in_stack_0000ff6c: u16;
  let mut in_stack_0000ff72: u16;
  let mut in_stack_0000ff76: u16;
  let mut uVar18: u8;
  let mut uVar19: u8;
  let mut uVar20: u8;
  let mut puStack76: *mut u32;
  let mut uStack70: u32;
  let mut uStack62: u32;
  paStack58: *mut astruct_99;
  let mut local_36: u16;
  let mut uStack52: u16;
  let mut uStack46: u32;
  let mut uStack42: u16;
  let mut uStack40: u16;
  let mut iStack38: i16;
  let mut puStack36: *mut u32;
  let mut puStack32: *mut u32;
  let mut iStack28: i16;
  let mut iStack20: i16;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut puStack10: *mut u32;
  paStack6: *mut astruct_99;
  uVar2: *mut astruct_99;
  uVar3: *mut astruct_99;
  uVar4: *mut astruct_99;
  uVar5: *mut astruct_99;

  paVar10 = CONCAT22(in_register_0000000a,param_1);
  puStack10 = mixed_1010_20ba(paVar10,_u16_1050_0ed0,CONCAT22(unaff_SI,0x35),in_stack_0000fe48,
                              in_stack_0000ff6c,in_stack_0000ff72,in_stack_0000ff76);
  paVar10 = (paVar10 & 0xffff0000 | puStack10 >> 0x10);
  uVar6 = puStack10 + 0xa;
  uStack14 = puStack10 & 0xffff0000 | uVar6;
  pass1_1030_9048(param_2,0x0,param_4);
  uVar13 = (param_3 >> 0x10);
  uVar20 = SUB41(param_3,0x0);
  if (uVar6 != 0) {
    iStack28 = 0;
    puStack32 = mixed_1010_20ba(paVar10,_u16_1050_0ed0,CONCAT22(unaff_SI,0x8),in_stack_0000fe48,
                                in_stack_0000ff6c,in_stack_0000ff72,in_stack_0000ff76);
    uVar14 = (puStack32 >> 0x10);
    puStack36 = (puStack32 + 0xe);
    uVar6 = (puStack32 + 0x10);
    uVar17 = uVar6;
    if ((uVar6 | puStack36) != 0) {
      pass1_1008_5784(CONCAT22(0x1050,&local_36),puStack36 & 0xffff | uVar6 << 0x10);
      while( true ) {
        uVar6 = uVar17;
        puVar12 = &local_36;
        pass1_1008_5b12(CONCAT22(0x1050,puVar12));
        uStack46 = CONCAT22(uVar6,puVar12);
        uVar17 = (uVar6 | puVar12);
        if ((uVar6 | puVar12) == 0) break;
        if ((puVar12[0x2] != 0x3e) && (puVar12[0x2] != 0x41)) {
          paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
          uVar7 = (paStack6 >> 0x10);
          uVar6 = paStack6;
          uVar17 = (uVar7 | uVar6);
          if ((uVar7 | uVar6) == 0) {
            paStack6 = NULL;
          }
          else {
            uVar14 = (uStack46 + 0x4);
            paStack6.field0_0x0 = 0x389a;
            (uVar6 + 0x2) = 0x1008;
            (uVar6 + 0x4) = uVar14;
            paStack6.field0_0x0 = 0x9ec8;
            (uVar6 + 0x2) = 0x1030;
          }
          ppcVar1 = (*param_3 + 0x8);
          (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
          if ((uStack46 + 0x4) == 0x13) {
            iStack28 = 0x1;
          }
        }
      }
    }
    for (iStack38 = 0xa; iStack38 < 0x41; iStack38 += 1) {
      if ((((((iStack38 != 0x37) && (iStack38 != 0x35)) && (iStack38 != 0x36)) &&
           ((iStack38 != 0x25 && (iStack38 != 0x26)))) &&
          ((iStack38 != 0x27 && (((iStack38 * 0x2 + uStack14) != 0x0 && (iStack38 != 0x13)))))) &&
         ((iStack38 != 0x14 || (iStack28 == 0)))) {
        paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
        uVar7 = (paStack6 >> 0x10);
        uVar6 = paStack6;
        if ((uVar7 | uVar6) == 0) {
          paStack6 = NULL;
        }
        else {
          paStack6.field0_0x0 = 0x389a;
          (uVar6 + 0x2) = 0x1008;
          (uVar6 + 0x4) = iStack38;
          paStack6.field0_0x0 = 0x9ec8;
          (uVar6 + 0x2) = 0x1030;
        }
        ppcVar1 = (*param_3 + 0x8);
        (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
      }
    }
  }
  uVar14 = (uStack14 >> 0x10);
  if ((uStack14 + 0x6a) == 0) {
    if ((uStack14 + 0x6c) != 0) {
      paStack58 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
      uVar6 = (paStack58 >> 0x10);
      puVar12 = paStack58;
      if ((uVar6 | puVar12) == 0) goto LAB_1030_973e;
      paStack58.field0_0x0 = 0x389a;
      puVar12[0x1] = 0x1008;
      puVar12[0x2] = 0x36;
  // TODO: goto LAB_1030_9728;
    }
  }
  else {
    paStack58 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar6 = (paStack58 >> 0x10);
    puVar12 = paStack58;
    if ((uVar6 | puVar12) == 0) {//
// LAB_1030_973e:
      paStack6 = NULL;
    }
    else {
      paStack58.field0_0x0 = 0x389a;
      puVar12[0x1] = 0x1008;
      puVar12[0x2] = 0x35;//
// LAB_1030_9728:
      *puVar12 = 0x9ec8;
      puVar12[0x1] = 0x1030;
      paStack6 = paStack58;
    }
    ppcVar1 = (*param_3 + 0x8);
    (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
  }
  uVar14 = (uStack14 >> 0x10);
  iVar11 = uStack14;
  if ((iVar11 + 0x4a) == 0) {
    if ((iVar11 + 0x4c) == 0) {
      if ((iVar11 + 0x4e) == 0) goto LAB_1030_97e8;
      paStack58 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
      uVar6 = (paStack58 >> 0x10);
      puVar12 = paStack58;
      if ((uVar6 | puVar12) != 0) {
        paStack58.field0_0x0 = 0x389a;
        puVar12[0x1] = 0x1008;
        puVar12[0x2] = 0x27;
    // TODO: goto LAB_1030_9879;
      }
    }
    else {
      paStack58 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
      uVar6 = (paStack58 >> 0x10);
      puVar12 = paStack58;
      if ((uVar6 | puVar12) != 0) {
        paStack58.field0_0x0 = 0x389a;
        puVar12[0x1] = 0x1008;
        puVar12[0x2] = 0x26;
    // TODO: goto LAB_1030_9879;
      }
    }//
// LAB_1030_97d0:
    paStack6 = NULL;
  }
  else {
    paStack58 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar6 = (paStack58 >> 0x10);
    puVar12 = paStack58;
    if ((uVar6 | puVar12) == 0) goto LAB_1030_97d0;
    paStack58.field0_0x0 = 0x389a;
    puVar12[0x1] = 0x1008;
    puVar12[0x2] = 0x25;//
// LAB_1030_9879:
    *puVar12 = 0x9ec8;
    puVar12[0x1] = 0x1030;
    paStack6 = paStack58;
  }
  ppcVar1 = (*param_3 + 0x8);
  (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));//
// LAB_1030_97e8:
  uStack18 = puStack10 & 0xffff0000 | (puStack10 + 0x11e);
  if ((puStack10 + 0x138) != 0) {
    puVar16 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
    uVar17 = puVar16 >> 0x10;
    uVar6 = puVar16;
    uVar15 = 0x38;
    pass1_1038_4d6e(uVar6,(puVar16 >> 0x10),param_4,puVar16);
    uVar14 = uVar17;
    puStack76 = CONCAT22(uVar14,uVar6);
    ppcVar1 = (*puStack76 + 0x10);
    uVar7 = uVar6;
    (**ppcVar1)(&u16_1050_1038,uVar6,uVar14);
    uStack70 = CONCAT22(uVar17,uVar7);
    for (uStack62 = 0; uVar7 = uVar17, uStack62 < uStack70; uStack62 += 1) {
      ppcVar1 = (*puStack76 + 0x4);
      uVar17 = uStack70;
      (**ppcVar1)(uVar15,uVar6,uVar14,uStack62,(uStack62 >> 0x10));
      uVar8 = uVar17;
      iVar11 = 0xd;
      local_36 = uVar8;
      uStack52 = uVar7;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar17 & 0xffff | uVar7 << 0x10);
      uStack46 = CONCAT22(uVar7,uVar8);
      uVar17 = struct_op_1030_73a8(CONCAT22(uVar7,uVar8),uVar8,uVar7);
      uVar7 = (uVar17 >> 0x10);
      uStack42 = uVar17;
      uVar15 = 0x28;
      uStack40 = uVar7;
      uVar9 = pass1_1028_6744(uVar17,iVar11);
      uVar17 = (uVar7 | uVar9);
      if ((uVar7 | uVar9) != 0) {
        puStack32 = _PTR_LOOP_1050_5768;
        paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
        uVar7 = (paStack6 >> 0x10);
        uVar5 = paStack6;
        if ((uVar7 | uVar5) == 0) {
          paStack6 = NULL;
        }
        else {
          paStack6.field0_0x0 = 0x389a;
          uVar5.field1_0x2 = 0x1008;
          &uVar5.field2_0x4 = 0x4c;
          paStack6.field0_0x0 = 0x9ec8;
          uVar5.field1_0x2 = 0x1030;
        }
        ppcVar1 = (*param_3 + 0x8);
        (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
        puStack36 = _PTR_LOOP_1050_5768;
        paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
        uVar7 = (paStack6 >> 0x10);
        uVar4 = paStack6;
        if ((uVar7 | uVar4) == 0) {
          paStack6 = NULL;
        }
        else {
          paStack6.field0_0x0 = 0x389a;
          uVar4.field1_0x2 = 0x1008;
          &uVar4.field2_0x4 = 0x4d;
          paStack6.field0_0x0 = 0x9ec8;
          uVar4.field1_0x2 = 0x1030;
        }
        uVar18 = SUB41(paStack6,0x0);
        uVar19 = (paStack6 >> 0x10);
        ppcVar1 = (*param_3 + 0x8);
        puVar16 = param_3;
        (**ppcVar1)();
        puStack36 = _PTR_LOOP_1050_5768;
        uVar15 = 0;
        paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
        uVar7 = (paStack6 >> 0x10);
        uVar3 = paStack6;
        if ((uVar7 | uVar3) == 0) {
          paStack6 = NULL;
        }
        else {
          paStack6.field0_0x0 = 0x389a;
          uVar3.field1_0x2 = 0x1008;
          &uVar3.field2_0x4 = 0x4e;
          paStack6.field0_0x0 = 0x9ec8;
          uVar3.field1_0x2 = 0x1030;
        }
        ppcVar1 = (*param_3 + 0x8);
        (**ppcVar1)(0x1000,param_3,paStack6,puVar16,uVar18,uVar19);
        break;
      }
    }
    if (puStack76.is_null() == false) {
      ppcVar1 = *puStack76;
      (**ppcVar1)(uVar15,uVar6,uVar14,1);
    }
  }
  for (iStack20 = 0x7a; iStack20 < 0x7d; iStack20 += 1) {
    if ((iStack20 * 0x2 + uStack14) != 0) {
      paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
      uVar6 = (paStack6 >> 0x10);
      uVar2 = paStack6;
      if ((uVar6 | uVar2) == 0) {
        paStack6 = NULL;
      }
      else {
        paStack6.field0_0x0 = 0x389a;
        uVar2.field1_0x2 = 0x1008;
        &uVar2.field2_0x4 = iStack20;
        paStack6.field0_0x0 = 0x9ec8;
        uVar2.field1_0x2 = 0x1030;
      }
      ppcVar1 = (*param_3 + 0x8);
      (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_9adc(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u32,mut param_6: u32)

{
  let mut ppcVar1: *mut *mut code;
  pstruct99_2: *mut astruct_99;
  let mut uVar2: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  pstruct99_7: *mut astruct_99;
  iVar6: *mut astruct_99;
  pstruct99_6: *mut astruct_99;
  pstruct99_3: *mut astruct_99;

  pass1_1038_53ba(param_6,1);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0) {
    pstruct99_6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar2 = (pstruct99_6 >> 0x10);
    pstruct99_2 = (pstruct99_6 & 0xffff);
    if ((uVar2 | pstruct99_2) == 0) {
      pstruct99_6 = NULL;
    }
    else {
      pstruct99_7 = pstruct99_6;
      pstruct99_6.field0_0x0 = 0x389a;
      pstruct99_7.field1_0x2 = 0x1008;
      &pstruct99_7.field2_0x4 = 0x77;
      pstruct99_6.field0_0x0 = 0x9ec8;
      pstruct99_7.field1_0x2 = 0x1030;
      pstruct99_2 = pstruct99_6;
    }
    param_1 = pstruct99_2;
    ppcVar1 = (*param_5 + 0x4);
    (**ppcVar1)(0x1000,param_5,pstruct99_6,(pstruct99_6 >> 0x10));
    uVar2 = extraout_DX;
  }
  pass1_1038_53ba(param_6,0x2);
  uVar2 |= param_1;
  if (uVar2 != 0) {
    pstruct99_6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar2 = (pstruct99_6 >> 0x10);
    pstruct99_2 = (pstruct99_6 & 0xffff);
    if ((uVar2 | pstruct99_2) == 0) {
      pstruct99_6 = NULL;
    }
    else {
      iVar6 = pstruct99_6;
      pstruct99_6.field0_0x0 = 0x389a;
      iVar6.field1_0x2 = 0x1008;
      &iVar6.field2_0x4 = 0x78;
      pstruct99_6.field0_0x0 = 0x9ec8;
      iVar6.field1_0x2 = 0x1030;
      pstruct99_2 = pstruct99_6;
    }
    param_1 = pstruct99_2;
    ppcVar1 = (*param_5 + 0x8);
    (**ppcVar1)(0x1000,param_5,pstruct99_6,(pstruct99_6 >> 0x10));
    uVar2 = extraout_DX_00;
  }
  pass1_1038_53ba(param_6,0x3);
  if ((uVar2 | param_1) != 0) {
    pstruct99_6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar2 = (pstruct99_6 >> 0x10);
    pstruct99_3 = pstruct99_6;
    if ((uVar2 | pstruct99_3) == 0) {
      pstruct99_6 = NULL;
    }
    else {
      pstruct99_6.field0_0x0 = 0x389a;
      pstruct99_3.field1_0x2 = 0x1008;
      &pstruct99_3.field2_0x4 = 0x75;
      pstruct99_6.field0_0x0 = 0x9ec8;
      pstruct99_3.field1_0x2 = 0x1030;
    }
    ppcVar1 = (*param_5 + 0x8);
    (**ppcVar1)(0x1000,param_5,pstruct99_6,(pstruct99_6 >> 0x10));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_9c1c(mut param_1: u32,param_2: *mut u32,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut in_EDX: *mut Struct57;
  let mut unaff_SI: u16;
  let mut puVar7: *mut u32;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut iStack24: i16;
  let mut iStack16: i16;
  paStack6: *mut astruct_99;

  puVar7 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22(unaff_SI,0x35),in_stack_0000fe72,
                           in_stack_0000ff96,in_stack_0000ff9c,in_stack_0000ffa0);
  iVar5 = puVar7 + 0xa;
  uVar4 = (puVar7 >> 0x10);
  iVar6 = iVar5;
  pass1_1030_9048(param_1,0x1,param_3);
  if (iVar6 != 0) {
    for (iStack24 = 0x4f; iStack24 < 0x70; iStack24 += 1) {
      if ((iStack24 * 0x2 + iVar5) != 0) {
        paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
        uVar3 = (paStack6 >> 0x10);
        uVar2 = paStack6;
        if ((uVar3 | uVar2) == 0) {
          paStack6 = NULL;
        }
        else {
          paStack6.field0_0x0 = 0x389a;
          (uVar2 + 0x2) = 0x1008;
          (uVar2 + 0x4) = iStack24;
          paStack6.field0_0x0 = 0x9ec8;
          (uVar2 + 0x2) = 0x1030;
        }
        ppcVar1 = (*param_2 + 0x8);
        (**ppcVar1)(0x1000,param_2,paStack6,(paStack6 >> 0x10));
      }
    }
  }
  for (iStack16 = 0x7d; iStack16 < 0x80; iStack16 += 1) {
    if ((iStack16 * 0x2 + iVar5) != 0) {
      paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
      uVar3 = (paStack6 >> 0x10);
      uVar2 = paStack6;
      if ((uVar3 | uVar2) == 0) {
        paStack6 = NULL;
      }
      else {
        paStack6.field0_0x0 = 0x389a;
        (uVar2 + 0x2) = 0x1008;
        (uVar2 + 0x4) = iStack16;
        paStack6.field0_0x0 = 0x9ec8;
        (uVar2 + 0x2) = 0x1030;
      }
      ppcVar1 = (*param_2 + 0x8);
      (**ppcVar1)(0x1000,param_2,paStack6,(paStack6 >> 0x10));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_9d42(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u32,mut param_5: u32)

{
  let mut puVar1: *mut u32;
  StructD **ppSVar2;
  let mut pSVar3: *mut StructD;
  let mut ppcVar4: *mut *mut code;
  let mut puVar5: *mut u16;
  let mut pcVar6: *mut c_char;
  let mut pcVar7: *mut c_char;
  let mut extraout_DX: u16;
  let mut uVar9: u16;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut string_a6: [u8;0x4] = [0;0x4];
  let mut uStack158: u32;
  let mut iStack154: i16;
  let mut local_98: *mut StructD;
  let mut uStack12: u32;
  let mut uStack8: u32;
  let mut iStack4: i16;
  let mut uVar8: u32;
  pstruct117_2: *mut astruct_117;

  uVar11 = (param_5 >> 0x10);
  if ((param_5 + 0x206) == 0) {
    iStack4 = (param_5 + 0x204);
    puVar5 = pass1_1000_4906(CONCAT22(0x1050,&local_98),NULL,0x94);
    uVar8 = ZEXT24(puVar5);
    iStack154 = 0x11;
    loop {
      empty_1038_540a();
      uVar11 = uVar8;
      (&local_98 + iStack154) = uVar11;
      (&local_98 + iStack154 * 0x4 + 0x2) = param_1;
      iStack154 += 0x1;
    } while (iStack154 < 0x25);
    empty_1038_540a();
    uStack158 = CONCAT22(param_1,uVar11);
    pass1_1008_5784(CONCAT22(0x1050,string_a6),param_4);
    pstruct117_2 = *(astruct_117 **)(_PTR_LOOP_1050_65e2 + 0x52);
    while( true ) {
      pcVar6 = string_a6;
      pass1_1008_5b12(CONCAT22(0x1050,pcVar6));
      uVar9 = extraout_DX | pcVar6;
      if (uVar9 == 0) break;
      pcVar7 = pcVar6;
      pass1_1030_4bbe(uVar9,pstruct117_2,(pcVar6 + 0x4));
      if (iStack4 == 0) {
        for (iStack154 = 0x11; iStack154 < 0x25; iStack154 += 1) {
          iVar10 = iStack154 * 0x4;
          if (((pcVar7 + iVar10) != 0) &&
             (pSVar3 = (&local_98)[iStack154], ppSVar2 = (StructD **)(pcVar7 + iVar10),
             pSVar3 <= *ppSVar2 && *ppSVar2 != pSVar3)) {
            puVar1 = (pcVar7 + iVar10);
            if (uStack158 <= *puVar1 && *puVar1 != uStack158) goto LAB_1030_9e17;
            uStack158 -= (pcVar7 + iVar10);
          }
        }
      }
      else {
        puVar1 = (pcVar7 + 0x8c);
        if ((uStack12 <= *puVar1 && *puVar1 != uStack12) ||
           (puVar1 = (pcVar7 + 0x90), uStack8 <= *puVar1 && *puVar1 != uStack8)) {//
// LAB_1030_9e17:
          ppcVar4 = (*param_4 + 0xc);
          (**ppcVar4)(0x1008,param_4,pcVar6,extraout_DX);
        }
      }
    }
  }
  return;
}



u16 * pass1_1030_9e9c(param_1: *mut u16,param_2: u8)

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    pass1_1000_093a(param_1);
  }
  return param_1;
}
pub fn pass1_1030_9ecc(param_1: u32,param_2: *mut astruct_424)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_1 = 0;
  *(astruct_424 **)(param_1 + 0x4) = param_2;
  (param_1 + 0x8) = 0;
  return;
}



pub unsafe fn pass1_1030_9ef2(param_1: u32) -> u16

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar3: u32;

  if (*param_1 != 0) {
    uVar3 = struct_op_1030_73a8(*param_1,in_AX,in_DX);
    uVar2 = (uVar3 >> 0x10);
    iVar1 = (uVar3 + 0xc);
    if (((iVar1 != 0x5) && (iVar1 != 0x9)) && ((uVar3 + 0x12) < 0x5)) {
      return 0x0;
    }
    pass1_1030_9f64(param_1);
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_9f40(param_1: u8,mut param_2: u32,mut param_3: u16 )

{
  let mut uVar1: u16;

  uVar1 = pass1_1008_c646(_u16_1050_06e0,CONCAT22(param_3,(_u16_1050_06e0 >> 0x10)));
  (param_2 + 0x8) = uVar1;
  pass1_1030_9f7a(param_2,uVar1);
  return;
}
pub fn pass1_1030_9f64(param_1: u32)

{
  (param_1 + 0x8) = 0;
  *param_1 = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_9f7a(param_1: *mut u16,mut param_2: u16 )

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut puVar3: *mut u32;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut puVar7: *mut u16;
  let mut local_130: [u8;0x120] = [0;0x120];
  paStack16: *mut astruct_15;
  let mut uStack12: u32;
  let mut local_8: u32;
  let mut iStack4: i16;

  puVar7 = pass1_1008_3e38(CONCAT22(0x1050,&local_8));
  uVar4 = (puVar7 >> 0x10);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,param_2,0x28);
  if (BVar2 != 0) {
    iStack4 = 0x1;
  }
  puVar3 = &local_8;
  pass1_1030_a278(puVar3,uVar4,param_1,CONCAT22(0x1050,puVar3));
  if (puVar3.is_null() == false) {
    uVar6 = (param_1 >> 0x10);
    uVar5 = param_1;
    uVar1 = (uVar5 + 0x4);
    uStack12 = (uVar1 + 0x8);
    uVar1 = (uVar5 + 0x4);
    struct_op_1028_87f0(CONCAT22(0x1050,local_130),0x0,0x0,param_2,&local_8,&DAT_1050_1050,
                        (uVar1 + 0x4),uStack12);
    fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_130));
    pass1_1028_b58e(paStack16);
    *param_1 = paStack16;
    (uVar5 + 0x2) = extraout_DX;
    if (0x0 < iStack4) {
      pass1_1030_a044(extraout_DX,uVar5,uVar6,CONCAT22(0x1050,&local_8),uStack12);
    }
  }
  return;
}
