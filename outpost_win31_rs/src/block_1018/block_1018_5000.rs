
pub fn pass1_1018_5032(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  clenaup_win_ui_1018_4d22(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1018_5070(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0;
  (param_1 + 0xe) = 0;
  (param_1 + 0x12) = 0;
  (param_1 + 0x16) = 0;
  param_1.offset_0x0 = 0x56d2;
  (param_1 + 0x2) = 0x1018;
  return;
}
pub fn pass1_1018_50ac(param_1: *mut u16)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x56d2;
  (iVar4 + 0x2) = 0x1018;
  puVar1 = (iVar4 + 0xe);
  uVar2 = (iVar4 + 0x10);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_50ea(mut param_1: u32,mut param_2: u16 ,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  paStack6: *mut astruct_99;

  paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
  uVar6 = (paStack6 >> 0x10);
  uVar4 = paStack6;
  if ((uVar6 | uVar4).is_null()) {
    paStack6 = NULL;
  }
  else {
    paStack6.field0_0x0 = 0x389a;
    (uVar4 + 0x2) = 0x1008;
    (uVar4 + 0x4) = 0;
    (uVar4 + 0x6) = 0;
    (uVar4 + 0x8) = 0;
    (uVar4 + 0xa) = 0;
    (uVar4 + 0xc) = 0;
    paStack6.field0_0x0 = 0x56ce;
    (uVar4 + 0x2) = 0x1018;
  }
  uVar9 = (paStack6 >> 0x10);
  uVar7 = paStack6;
  (uVar7 + 0xa) = param_2;
  uVar10 = (param_1 >> 0x10);
  iVar8 = param_1;
  uVar3 = (iVar8 + 0xa);
  iVar1 = (uVar3 + 0xc);
  if (iVar1 == 1) {
    uVar3 = (iVar8 + 0xa);
    uVar5 = (uVar3 + 0xe);
    (uVar7 + 0x4) = uVar5;
  }
  else if (iVar1 == 0x5) {
    uVar3 = (iVar8 + 0xa);
    uVar5 = (uVar3 + 0xe);
    (uVar7 + 0x6) = uVar5;
  }
  else {
    if (iVar1 != 0x6) {
      if ((uVar9 | uVar7) == 0) {
        return;
      }
      ppcVar2 = paStack6;
      (**ppcVar2)();
      return;
    }
    uVar3 = (iVar8 + 0xa);
    uVar5 = (uVar3 + 0xe);
    (uVar7 + 0x8) = uVar5;
  }
  pass1_1030_6c66(uVar5,(uVar6 | uVar4),param_3,0x1,paStack6);
  return;
}
pub fn pass1_1018_51d2(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0xe);
  uVar2 = (iVar4 + 0x10);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0xe) = 0;
  return;
}



pub fn pass1_1018_5206(mut param_1: u32,param_2: *mut c_char) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut local_a: [u8;0x8] = [0;0x8];

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  (iVar3 + 0xa) = 0;
  pass1_1008_5784(CONCAT22(0x1050,local_a),(iVar3 + 0xe));
  loop {
    uVar5 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    uVar2 = (uVar5 >> 0x10);
    (iVar3 + 0xa) = uVar5;
    (iVar3 + 0xc) = uVar2;
    if ((uVar2 | (iVar3 + 0xa)) == 0) break;
    uVar5 = (iVar3 + 0xa);
    iVar1 = pass1_1000_3d7a(*(uVar5 + 0x4),param_2);
  } while (iVar1 != 0);
  return CONCAT22((iVar3 + 0xc),(iVar3 + 0xa));
}



pub fn pass1_1018_526a(mut param_1: u32,mut param_2: u32) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0xe) == 0) {
    pass1_1018_5292((param_1 & 0xffff | uVar2 << 0x10),param_2);
  }
  return CONCAT22((iVar1 + 0x10),(iVar1 + 0xe));
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_5292(param_1: *mut astruct_9,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut uVar3: u32;
  let mut uVar11: u16;
  let mut uVar4: u16;
  let mut BVar4: bool;
  let mut puVar5: *mut c_char;
  paVar5: *mut astruct_11;
  let mut pcVar6: *mut c_char;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut pcVar9: *mut c_char;
  let mut uVar10: u16;
  let mut uVar9: u32;
  let mut uVar12: u16;
  let mut in_EDX: u32;
  let mut paVar13: *mut Struct57;
  let mut paVar14: *mut Struct57;
  pstruct9_v18: *mut astruct_9;
  pstruct9_v16: *mut astruct_9;
  let mut uVar16: u16;
  let mut uVar17: u16;
  paVar18: *mut astruct_203;
  let mut puVar19: *mut u16;
  let mut uStack50: u16;
  let mut local_26: [u8;0x8] = [0;0x8];
  let mut uStack30: u32;
  let mut uStack26: u32;
  let mut uStack22: u32;
  let mut puStack18: u16;
  let mut pSStack16: *mut StructD;
  let mut puStack14: u16;
  let mut pSStack12: *mut StructD;
  let mut uStack10: u16;
  let mut uStack8: u32;
  let mut uStack4: u16;
  let mut uVar2: u32;
  iVar1: *mut astruct_11;
  let mut paVar15: *mut Struct57;
  let mut fn_ptr_2: *mut *mut code;

  pstruct9_v16 = (param_1 >> 0x10);
  pstruct9_v18 = param_1;
  puStack18 = pstruct9_v18.field14_0xe;
  uVar9 = puStack18;
  pSStack16 = pstruct9_v18.field15_0x10;
  paVar13 = (in_EDX & 0xffff0000 | ZEXT24(pSStack16));
  puStack14 = puStack18;
  pSStack12 = pSStack16;
  if ((pSStack16 | puStack18) != 0) {
    fn_ptr_2 = puStack18;
    (**fn_ptr_2)();
  }
  mem_op_1000_179c(0xc,paVar13);
  puStack18 = uVar9;
  pSStack16 = paVar13;
  paVar15 = (paVar13 & 0xffff0000);
  paVar14 = (paVar15 | (pSStack16 | puStack18));
  if ((pSStack16 | puStack18) == 0) {
    uVar9 = 0;
  }
  else {
    set_struct_1008_574a((uVar9 & 0xffff | paVar13 << 0x10));
    paVar15 = paVar14;
  }
  pstruct9_v18.field14_0xe = uVar9;
  pstruct9_v18.field15_0x10 = paVar15;
  uStack4 = 0x21;
  while( true ) {
    if (uStack4 < 0x0) break;
    uStack22 = pass1_1030_7c28(uVar9,paVar15,param_2,uStack4);
    uVar9 = uStack22 & 0xffff;
    uVar11 = uVar9;
    uVar12 = (uStack22 >> 0x10) | uVar11;
    paVar15 = (paVar15 & 0xffff0000 | uVar12);
    if (uVar12 != 0) {
      string_1020_c0ca(uStack4);
      uVar4 = str_op_1008_60e8(paVar15,CONCAT22(paVar15,uVar11));
      uVar9 = uVar4;
      uStack26 = CONCAT22(paVar15,uVar4);
      mem_op_1000_179c(0x10,paVar15);
      puStack14 = uVar9;
      pSStack12 = paVar15;
      paVar13 = (paVar15 & 0xffff0000 | (pSStack12 | puStack14));
      if ((pSStack12 | puStack14) == 0) {
        uVar9 = 0;
        paVar15 = (paVar15 & 0xffff0000);
      }
      else {
        struct_1018_4790(uVar9 & 0xffff | paVar15 << 0x10,uStack22,uStack26,uStack4);
        paVar15 = paVar13;
      }
      uStack30 = uVar9 & 0xffff | paVar15 << 0x10;
      uVar2 = &pstruct9_v18.field14_0xe;
      fn_ptr_2 = (*&pstruct9_v18.field14_0xe + 0x4);
      (**fn_ptr_2)(0x0,uVar2,(uVar2 >> 0x10),uVar9,paVar15);
    }
    uStack4 -= 1;
  }
  uStack8 = struct_op_1030_73a8(param_2,uVar9,paVar15);
  paVar13 = (paVar15 & 0xffff0000 | uStack8 >> 0x10);
  uStack10 = (uStack8 + 0xc);
  BVar4 = pass1_1008_c6ae(_u16_1050_06e0,uStack10,0x4);
  if (BVar4 != 0) {
    uStack30 = uStack8;
    uStack26 = (uStack8 + 0x20);
    pass1_1008_5784(CONCAT22(0x1050,local_26),uStack26);
    while( true ) {
      puVar5 = local_26;
      pass1_1008_5b12(CONCAT22(0x1050,puVar5));
      uVar12 = paVar13;
      uStack22 = CONCAT22(uVar12,puVar5);
      paVar13 = (paVar13 & 0xffff0000 | (uVar12 | puVar5));
      if ((uVar12 | puVar5) == 0) break;
      iVar1 = *(astruct_11 **)(puVar5 + 0x6);
      paVar5 = iVar1 + -0x7;
      if (paVar5.is_null()) {//
// LAB_1018_53f0:
        pcVar6 = string_op_1020_c222((puVar5 + 0x6));
        uVar7 = str_op_1008_60e8(paVar13,CONCAT22(paVar13,pcVar6));
        uVar17 = SUB42(paVar13,0x0);
        uVar8 = uVar7;
        mem_op_1000_179c(0x10,paVar13);
        pSStack16 = paVar13;
        paVar13 = (paVar13 & 0xffff0000);
        puStack18 = uVar8;
        if ((pSStack16 | uVar8) == 0) {
          uVar17 = 0;
        }
        else {
          uVar16 = (uStack22 >> 0x10);
          paVar18 = struct_1018_48b0(CONCAT22(pSStack16,uVar8),(uStack22 + 0xa),
                                     CONCAT22(uVar17,uVar7),(uStack22 + 0x6));
          paVar13 = (paVar13 & 0xffff0000 | paVar18 >> 0x10);
          uVar17 = SUB42(paVar18,0x0);
        }
        uVar1 = &pstruct9_v18.field14_0xe;
        fn_ptr_2 = (*&pstruct9_v18.field14_0xe + 0x4);
        (**fn_ptr_2)(0x0,uVar1,(uVar1 >> 0x10),uVar17,paVar13);
      }
      else if (((0x5 < paVar5) && (!SBORROW2(paVar5,0x6))) && ((iVar1 + -0xd) < 0x2)) goto LAB_1018_53f0;
      uVar17 = (uStack22 >> 0x10);
      if ((uStack22 + 0x8) != 0) {
        pcVar6 = string_op_1020_c2f8((uStack22 + 0x8));
        uVar7 = str_op_1008_60e8(paVar13,CONCAT22(paVar13,pcVar6));
        uVar17 = SUB42(paVar13,0x0);
        uVar8 = uVar7;
        mem_op_1000_179c(0x10,paVar13);
        pSStack12 = paVar13;
        paVar13 = (paVar13 & 0xffff0000);
        puStack14 = uVar8;
        if ((pSStack12 | uVar8) == 0) {
          uVar17 = 0;
        }
        else {
          uVar16 = (uStack22 >> 0x10);
          puVar19 = struct_1018_48e8(CONCAT22(pSStack12,uVar8),(uStack22 + 0xa),
                                     CONCAT22(uVar17,uVar7),(uStack22 + 0x8));
          paVar13 = (paVar13 & 0xffff0000 | puVar19 >> 0x10);
          uVar17 = SUB42(puVar19,0x0);
        }
        uVar1 = &pstruct9_v18.field14_0xe;
        fn_ptr_2 = (*&pstruct9_v18.field14_0xe + 0x4);
        (**fn_ptr_2)(0x0,uVar1,(uVar1 >> 0x10),uVar17,paVar13);
      }
    }
  }
  uVar17 = (param_2 >> 0x10);
  uVar3 = (param_2 + 0x3e);
  uVar12 = (param_2 + 0x40);
  paVar13 = (paVar13 & 0xffff0000 | uVar12);
  uStack50 = uVar3;
  if ((uVar12 | uStack50) != 0) {
    pass1_1008_5784(CONCAT22(0x1050,local_26),uVar3 & 0xffff | uVar12 << 0x10);
    while( true ) {
      pcVar6 = local_26;
      pass1_1008_5b12(CONCAT22(0x1050,pcVar6));
      uVar12 = paVar13;
      paVar13 = (paVar13 & 0xffff0000 | (uVar12 | pcVar6));
      if ((uVar12 | pcVar6) == 0) break;
      if ((pcVar6 + 0x4) != 0) {
        pcVar9 = string_1020_c0d8((pcVar6 + 0x4));
        uVar10 = str_op_1008_60e8(paVar13,CONCAT22(paVar13,pcVar9));
        uStack30 = CONCAT22(paVar13,uVar10);
        mem_op_1000_179c(0x10,paVar13);
        pSStack16 = paVar13;
        paVar13 = (paVar13 & 0xffff0000);
        paVar15 = (paVar13 | (pSStack16 | uVar10));
        puStack18 = uVar10;
        if ((pSStack16 | uVar10) == 0) {
          uVar10 = 0;
        }
        else {
          struct_1018_4790(CONCAT22(pSStack16,uVar10),(pcVar6 + 0xa),uStack30,(pcVar6 + 0x4))
          ;
          paVar13 = paVar15;
        }
        uStack26 = CONCAT22(paVar13,uVar10);
        uVar1 = &pstruct9_v18.field14_0xe;
        fn_ptr_2 = (*&pstruct9_v18.field14_0xe + 0x4);
        (**fn_ptr_2)(0x0,uVar1,(uVar1 >> 0x10),uVar10,paVar13);
      }
      if ((pcVar6 + 0x6) != 0) {
        pcVar9 = string_op_1020_c222((pcVar6 + 0x6));
        uVar10 = str_op_1008_60e8(paVar13,CONCAT22(paVar13,pcVar9));
        uStack30 = CONCAT22(paVar13,uVar10);
        mem_op_1000_179c(0x10,paVar13);
        pSStack12 = paVar13;
        paVar13 = (paVar13 & 0xffff0000);
        puStack14 = uVar10;
        if ((pSStack12 | uVar10) == 0) {
          uVar17 = 0;
        }
        else {
          paVar18 = struct_1018_48b0(CONCAT22(pSStack12,uVar10),(pcVar6 + 0xa),uStack30,
                                     (pcVar6 + 0x6));
          paVar13 = (paVar13 & 0xffff0000 | paVar18 >> 0x10);
          uVar17 = SUB42(paVar18,0x0);
        }
        uStack26 = CONCAT22(paVar13,uVar17);
        uVar1 = &pstruct9_v18.field14_0xe;
        fn_ptr_2 = (*&pstruct9_v18.field14_0xe + 0x4);
        (**fn_ptr_2)(0x0,uVar1,(uVar1 >> 0x10),uVar17,paVar13);
      }
      if ((pcVar6 + 0x8) != 0) {
        pcVar9 = string_op_1020_c2f8((pcVar6 + 0x8));
        uVar10 = str_op_1008_60e8(paVar13,CONCAT22(paVar13,pcVar9));
        uStack30 = CONCAT22(paVar13,uVar10);
        mem_op_1000_179c(0x10,paVar13);
        pSStack16 = paVar13;
        paVar13 = (paVar13 & 0xffff0000);
        puStack18 = uVar10;
        if ((pSStack16 | uVar10) == 0) {
          uVar17 = 0;
        }
        else {
          puVar19 = struct_1018_48e8(CONCAT22(pSStack16,uVar10),(pcVar6 + 0xa),uStack30,
                                     (pcVar6 + 0x8));
          paVar13 = (paVar13 & 0xffff0000 | puVar19 >> 0x10);
          uVar17 = SUB42(puVar19,0x0);
        }
        uStack26 = CONCAT22(paVar13,uVar17);
        uVar1 = &pstruct9_v18.field14_0xe;
        fn_ptr_2 = (*&pstruct9_v18.field14_0xe + 0x4);
        (**fn_ptr_2)(0x0,uVar1,(uVar1 >> 0x10),uVar17,paVar13);
      }
    }
  }
  return;
}



pub fn pass1_1018_567c(param_1: *mut u16,param_2: u8) -> *mut u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    pass1_1000_093a(param_1);
  }
  return param_1;
}



pub fn pass1_1018_56a8(mut param_1: u32,param_2: u8) -> u32

{
  pass1_1018_50ac(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub fn pass1_1018_56e6(param_1: *mut astruct_19,mut param_2: u16 ) -> *mut u16

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0;
  param_1.offset_0x0 = 0x5830;
  (param_1 + 0x2) = 0x1018;
  return &param_1.offset_0x0;
}
pub fn pass1_1018_5714(param_1: *mut u16)

{
  *param_1 = 0x5830;
  (param_1 + 0x2) = 0x1018;
  pass1_1010_1d80(param_1);
  return;
}



pub fn pass1_1018_5732(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32) -> u32

{
  let mut uVar1: u32;

  uVar1 = pass1_1030_6d4e(param_1,param_2,param_5);
  return uVar1;
}
pub fn pass1_1018_5742(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u32,param_4: *mut astruct_299)

{
  let mut puVar1: *mut u32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut bVar4: bool;
  let mut puVar5: *mut u32;
  let mut uVar6: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uStack16: u32;

  bVar4 = false;
  puVar1 = (param_3 + 0x4);
  ppcVar2 = (*puVar1 + 0x10);
  puVar5 = puVar1;
  (**ppcVar2)();
  uVar3 = puVar5 & 0xffff | extraout_DX << 0x10;
  uStack16 = 0;
  loop {
    if (uVar3 <= uStack16) {//
// LAB_1018_579f:
      if (!bVar4) {
        if (param_3.is_null() == false) {
          ppcVar2 = *param_3;
          (**ppcVar2)();
        }
        param_3 = NULL;
      }
      pass1_1030_6d80(param_4,param_3);
      return;
    }
    ppcVar2 = (*puVar1 + 0x4);
    uVar6 = uVar3;
    (**ppcVar2)();
    if ((extraout_DX_00 | uVar6) != 0) {
      bVar4 = true;
  // TODO: goto LAB_1018_579f;
    }
    uStack16 += 0x1;
  } while( true );
}
pub fn pass1_1018_57d2(mut param_1: u32,mut param_2: u32)

{
  (param_1 + 0xa) = param_2;
  return;
}
pub fn pass1_1018_57e6(mut param_1: u32,param_2: i32,mut param_3: u16 ,mut param_4: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  send_dlg_item_msg_1040_d20c(param_3,param_4,*(astruct_929 **)(param_1 + 0xa),param_2);
  (param_1 + 0xa) = 0;
  return;
}



pub fn pass1_1018_580a(param_1: *mut u16,param_2: u8) -> *mut u16

{
  pass1_1018_5714(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1018_5840(param_1: *mut astruct_57,param_2: *mut astruct_20,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,
                     mut param_6: u16 ,mut param_7: u16 ,mut param_8: u16 ,mut param_9: u16 )

{
  let mut uVar1: u16;
  let mut iVar2: *mut astruct_20;
  let mut unaff_BP: u16;
  let mut uVar2: *mut astruct_20;
  let mut puVar2: *mut u32;

  unk_draw_op_1020_7f7a(param_2,0x6,CONCAT22(param_4,param_3),param_5);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  &iVar2[0x1].field5_0xc = 0;
  iVar2[0x1].field7_0x10 = NULL;
  iVar2[0x1].field8_0x14 = 0;
  param_2.offset_0x0 = s_Alloc__s_1050_5a5b + 0x7;
  iVar2.base_0x2 = 0x1018;
  (iVar2 + 1)->offset_0x0 = 0x5afe;
  iVar2[0x1].base_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x27),param_6,param_7,param_8,param_9)
  ;
  uVar1 = (puVar2 >> 0x10);
  &iVar2[0x1].field7_0x10 = puVar2;
  (&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
  &iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
  (&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
  return;
}
pub fn pass1_1018_58b6(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = s_Alloc__s_1050_5a5b + 0x7;
  (iVar1 + 0x2) = 0x1018;
  (iVar1 + 0xe2) = 0x5afe;
  (iVar1 + 0xe4) = 0x1018;
  pass1_1020_808e(param_1);
  return;
}
pub fn invalidate_rect_1018_58e2(param_1: *mut astruct_58,mut param_2: i16)

{
  let mut piVar1: *mut i16;
  iVar2: *mut astruct_58;
  let mut uVar2: u16;

  if (param_2 == 0x105) {
    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    piVar1 = &iVar2.field245_0xf6;
    *piVar1 = *piVar1 + 1;
    if (u16_1050_4240 <= iVar2.field245_0xf6) {
      PostMessage16(0x0,0xca,0x111,HWND16_1050_0396);
      return;
    }
    iVar2.field234_0xea = 0;
    InvalidateRect16(0x0,NULL,0x0);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1018_5932(mut param_1: u16 ,mut param_2: u32) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;

  uVar4 = (param_2 >> 0x10);
  uVar3 = param_2;
  uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
  if (uVar2 != 0) {
    ppcVar1 = ((uVar3 + 0xee) + 0x8);
    uVar5 = (**ppcVar1)();
    param_1 = (uVar5 >> 0x10);
    uVar2 = uVar5;
  }
  if ((uVar3 + 0xea) == 0) {
    (uVar3 + 0xea) = 0x1;
    uVar5 = pass1_1038_af40(uVar3,param_1,_PTR_LOOP_1050_5b7c,(uVar3 + 0x8),
                            ((uVar3 + 0xf6) * 0x2 + 0x4238));
    uVar2 = uVar5;
  }
  return uVar2;
}
pub fn win_1018_598c(mut param_1: u16 ,param_2: *mut astruct_57,struct_param_1: *mut StructA,mut param_4: u16 ,mut param_5: u16 )

{
  let mut uVar1: u16;
  let struct_1: *mut StructA;
  let mut uVar3: u16;
  let mut in_stack_0000fe68: u16;
  let mut in_stack_0000ff8c: u16;
  let mut in_stack_0000ff92: u16;
  let mut in_stack_0000ff96: u16;
  let mut uVar2: u32;

  create_window_ex_1008_9760(struct_param_1);
  uVar3 = (struct_param_1 >> 0x10);
  struct_1 = struct_param_1;
  get_dc_1018_4db0(*(astruct_126 **)&struct_1[0x1].field20_0x26,struct_1.field4_0x8);
  mem_op_1000_179c(0x2a,param_2);
  uVar1 = param_2 | param_1;
  uVar2 = param_2 & 0xffff0000 | uVar1;
  if (uVar1 != 0) {
    pass1_1018_5b06(uVar2,CONCAT22(param_2,param_1),struct_1.field4_0x8,param_4,param_5,
                    in_stack_0000fe68,in_stack_0000ff8c,in_stack_0000ff92,in_stack_0000ff96);
    struct_1[0x1].field18_0x22 = param_1;
    struct_1[0x1].field19_0x24 = uVar2;
    return;
  }
  &struct_1[0x1].field18_0x22 = 0;
  return;
}
pub fn FUN_1018_59f0(mut param_1: u16 ,mut param_2: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  puVar1 = (iVar4 + 0xee);
  uVar2 = (iVar4 + 0xf0);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0xee) = 0;
  destroy_win_1008_628e(param_2 & 0xffff | uVar5 << 0x10);
  return;
}



pub fn pass1_1018_5a2e(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1018_58b6(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub fn FUN_1018_5a3c(mut param_1: u16 ,param_2: *mut StructD,param_3: u8) -> *mut StructD

{
  pass1_1018_58b6(&param_2.address_offset_field_0x0);
  if ((param_3 & 1) != 0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



// WARNING: Unable to use type for symbol uVar4
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_5b06(mut param_1: u32,param_2: *mut StructA,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,
                    mut param_6: u16 ,mut param_7: u16 ,mut param_8: u16 ,mut param_9: u16 )

{
  paVar1: *mut astruct_76;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar10: u16;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u32;
  let mut unaff_SI: u16;
  let mut puVar11: *mut u16;
  let mut puVar12: *mut u32;
  paVar13: *mut astruct_76;
  let mut uVar14: u32;
  let mut local_c: [u8;0x6] = [0;0x6];
  let mut puStack6: *mut u32;
  let mut uVar4: u16;

  set_struct_op_1020_921c(param_1,param_2,param_3,CONCAT22(param_5,param_4));
  uVar10 = (param_1 >> 0x10);
  (param_2 + 0x14) = 0;
  (param_2 + 0x18) = 0;
  puVar11 = pass1_1008_3e38((param_2 & 0xffff0000 | (param_2 + 0x1c)));
  paVar8 = CONCAT22(uVar10,(puVar11 >> 0x10));
  param_2.field0_0x0 = &u16_1050_5e1a;
  (param_2 + 0x2) = 0x1018;
  puStack6 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(unaff_SI,0x48),param_6,param_7,param_8,param_9
                            );
  uVar9 = paVar8 & 0xffff0000;
  puVar11 = pass1_1008_3e38(CONCAT22(0x1050,local_c));
  paVar8 = (uVar9 & 0xffff0000 | puVar11 >> 0x10);
  pass1_1008_3f62(CONCAT22(0x1050,local_c),
                  (puStack6 & 0xffff0000 | (puStack6 + 0xe)));
  puVar12 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(unaff_SI,0x27),param_6,param_7,param_8,param_9)
  ;
  uVar10 = (paVar8 >> 0x10);
  (param_2 + 0x14) = puVar12;
  (param_2 + 0x16) = (puVar12 >> 0x10);
  ppcVar2 = ((param_2 + 0x14) + 0x4);
  (**ppcVar2)();
  (param_2 + 0x6) = (param_2 + 0x14);
  uVar3 = (param_2 + 0x14);
  paVar1 = (uVar3 + 0xa);
  iVar4 = param_2 + 0xa;
  ppcVar2 = (paVar1 + 0x8);
  (**ppcVar2)();
  (param_2 + 0x12) = iVar4;
  draw_op_1020_9364((param_2 & 0xffff | param_2 << 0x10));
  uVar9 = (param_2 + 0x14);
  pass1_1008_3f62((param_2 & 0xffff0000 | (param_2 + 0x1c)),
                  (uVar9 & 0xffff0000 | (uVar9 + 0x52)));
  pass1_1008_3f32((param_2 & 0xffff0000 | (param_2 + 0x1cU)),CONCAT22(0x1050,local_c));
  paVar13 = pass1_1008_9f48(*(astruct_134 **)(param_2 + 0x14));
  uVar14 = pass1_1008_4772(paVar13);
  uVar6 = (uVar14 >> 0x10);
  paVar8 = CONCAT22(uVar10,uVar6);
  uVar4 = uVar14;
  uVar5 = uVar4;
  mem_op_1000_179c(0x14,paVar8);
  uVar7 = paVar8 | uVar5;
  if (uVar7 == 0) {
    (param_2 + 0x18) = 0;
  }
  else {
    pass1_1008_50c2(CONCAT22(paVar8,uVar5),(uVar4 + 0x8),(uVar4 + 0x4),
                    (param_2 & 0xffff0000 | (param_2 + 0x1cU)),paVar1);
    (param_2 + 0x18) = uVar5;
    (param_2 + 0x1a) = uVar7;
  }
  pass1_1008_5134((param_2 + 0x18));
  (param_2 + 0x22) = (param_2 + 0x1c);
  (param_2 + 0x24) = (param_2 + 0x1e);
  (param_2 + 0x26) = (uVar4 + 0x4) + (param_2 + 0x22) + 1;
  (param_2 + 0x28) = (uVar4 + 0x8) + (param_2 + 0x24) + 1;
  return;
}
pub fn pass1_1018_5cc8(param_1: *mut StructD)

{
  let mut uVar1: u16;
  let mut pcVar2: *mut c_char;
  let mut iVar3: *mut StructD;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  param_1.address_offset_field_0x0 = &u16_1050_5e1a;
  iVar3.address_offset_field_0x2 = 0x1018;
  pcVar2 = *&iVar3.field13_0x18;
  uVar1 = iVar3.field14_0x1a;
  if ((uVar1 | pcVar2) != 0) {
    pass1_1008_5118(pcVar2 & 0xffff | uVar1 << 0x10);
    fn_ptr_1000_17ce(pcVar2);
  }
  if (iVar3.field12_0x14 != 0) {
    pass1_1010_1ea6(iVar3.field12_0x14,(param_1 & 0xffff | uVar3 << 0x10));
    pass1_1010_1dda(iVar3.field12_0x14);
  }
  palette_op_1020_92c4(param_1);
  return;
}
pub fn invalidate_rect_1018_5d32(mut param_1: u32,mut param_2: i16)

{
  let mut hwnd: HWND16;

  hwnd = (param_1 >> 0x10);
  if (param_2 == 1) {
    (param_1 + 0x14) = 0;
    return;
  }
  if (param_2 != 0x2) {
    return;
  }
  InvalidateRect16(0x0,(param_1 + 0x22),hwnd);
  return;
}



// WARNING: Unable to use type for symbol uVar4
pub fn misc_draw_op_1018_5d6c(param_1: *mut astruct_839)

{
  paVar1: *mut astruct_76;
  struct_4: *mut astruct_839;
  let mut uVar5: u16;
  paVar2: *mut astruct_76;
  PAINTSTRUCT16 local_22;
  let mut puVar1: *mut u32;
  uVar4: *mut astruct_134;
  let mut fn_ptr_1: *mut *mut code;

  uVar5 = (param_1 >> 0x10);
  struct_4 = param_1;
  BeginPaint16(CONCAT22(0x1050,&local_22),struct_4.field4_0x4);
  uVar4 = struct_4.pstruct134_0x14;
  paVar1 = (uVar4 + 0xa);
  paVar2 = pass1_1008_9f48(struct_4.pstruct134_0x14);
  pass1_1008_5236(struct_4.field20_0x18);
  pass1_1008_4480(paVar1,(param_1 & 0xffff0000 | ZEXT24(struct_4 + 1)),paVar2);
  fn_ptr_1 = (paVar1 + 0x4);
  (**fn_ptr_1)(0x1008,paVar1,(paVar1 >> 0x10),0x0,
               param_1 & 0xffff0000 | ZEXT24(&struct_4.field_0xa));
  EndPaint16(CONCAT22(0x1050,&local_22),struct_4.field4_0x4);
  return;
}



pub fn pass1_1018_5df4(param_1: *mut u16,param_2: u8) -> *mut u16

{
  pass1_1018_5cc8(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_57 * pass1_1018_5e26(param_1: *mut astruct_57,mut param_2: u16 )

{
  let mut uVar1: *mut Struct57;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfd0,param_2);
  uVar1 = (param_1 >> 0x10);
  param_1.field0_0x0 = 0x6128;
  (param_1 + 0x2) = 0x1018;
  (param_1 + 0x74) = 0x1;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_5e5a(param_1: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x6128;
  (param_1 + 0x2) = 0x1018;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn pass1_1018_5e86(param_1: u32)

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = (*param_1 + 0x6c);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1018_5e9a(mut param_1: u16 ,StructB *structb_param_1)

{
  char **ppcVar1;
  LPVOID pvVar2;
  let mut IVar3: i16;
  astruct_92 **ppaVar4;
  let mut puVar5: *mut u8;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  let mut uVar9: u32;
  StructB *structb_9;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut puVar13: *mut u32;
  let mut in_stack_0000fe5a: u16;
  let mut in_stack_0000ff7e: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff88: u16;
  let mut in_stack_0000ffb2: u16;
  local_28: *mut astruct_92;
  let mut iStack22: i16;
  let mut uStack20: u16;
  let mut iStack18: i16;
  let mut iStack16: i16;
  let mut local_e: RECT16;
  let mut iStack8: i16;
  INT16 *pIStack6;
  let mut paVar8: *mut Struct57;

  paVar7 = CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(structb_param_1);
  puVar13 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,CONCAT22(in_stack_0000ffb2,0x39),in_stack_0000fe5a,
                            in_stack_0000ff7e,in_stack_0000ff84,in_stack_0000ff88);
  paVar7 = (paVar7 & 0xffff0000 | puVar13 >> 0x10);
  pvVar2 = (LPVOID)puVar13;
  uVar11 = (structb_param_1 >> 0x10);
  structb_9 = (StructB *)structb_param_1;
  structb_9[0x7].field1_0x2 = pvVar2;
  structb_9[0x7].hwnd_0x6 = (puVar13 >> 0x10);
  mem_op_1000_179c(0x12,paVar7);
  puVar5 = (paVar7 | pvVar2);
  paVar8 = (paVar7 & 0xffff0000 | ZEXT24(puVar5));
  if (puVar5.is_null()) {
    &structb_9[0x7].lpvoid_field_0x8 = 0;
  }
  else {
    pass1_1018_6198(puVar5,CONCAT22(paVar7,pvVar2),structb_param_1,
                    structb_9.lpvoid_field_0x8);
    structb_9[0x7].lpvoid_field_0x8 = pvVar2;
    structb_9[0x7].max_count_field_0x10 = paVar8;
  }
  uVar9 = &structb_9[0x7].field1_0x2;
  pIStack6 = (uVar9 & 0xffff0000 | (uVar9 + 0xa));
  GetClientRect16(&local_e,&DAT_1050_1050);
  IVar3 = GetSystemMetrics16(SM_CYCAPTION);
  (pIStack6 + 0x6) = IVar3 + iStack8;
  puVar13 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(in_stack_0000ffb2,0x48),in_stack_0000fe5a,
                            in_stack_0000ff7e,in_stack_0000ff84,in_stack_0000ff88);
  uStack20 = (puVar13 >> 0x10);
  iStack22 = puVar13;
  iStack16 = (iStack22 + 0xa);
  iStack18 = (iStack22 + 0xc);
  uVar12 = (pIStack6 >> 0x10);
  (pIStack6 + 0x2) = (iStack18 - (pIStack6 + 0x6)) / 0x2;
  uVar9 = (iStack16 >> 0xf);
  *pIStack6 = iStack16 / 0x2 + 0x3;
  pass1_1028_dc52(CONCAT22(0x1050,&local_28),0x1,0x0,0x100);
  while( true ) {
    uVar6 = uVar9;
    ppaVar4 = &local_28;
    pass1_1028_e4ec(CONCAT22(0x1050,ppaVar4));
    uVar9 = (uVar6 | ppaVar4);
    if ((uVar6 | ppaVar4) == 0) break;
    ppcVar1 = *(char ***)(ppaVar4 + 0x8);
    if (ppcVar1.is_null() == false) {
      pass1_1000_3cea(structb_param_1 & 0xffff0000 | ZEXT24(&structb_9.field8_0x10),*ppcVar1);
    }
  }
  uVar12 = (pIStack6 >> 0x10);
  iVar10 = pIStack6;
  SetWindowPos16(0x44,(iVar10 + 0x6),(iVar10 + 0x4),(iVar10 + 0x2),*pIStack6,0x0,
                 structb_9.lpvoid_field_0x8);
  return;
}
pub fn pass1_1018_5ffa(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0x92);
  uVar2 = (iVar4 + 0x94);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0x92) = 0;
  pass1_1010_1dda((iVar4 + 0x8e));
  (iVar4 + 0x8e) = 0;
  return;
}
