
pub fn pass1_1030_10b0(param_1: *mut astruct_12,param_2: *mut astruct_12,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,
                    mut param_6: u32,mut param_7: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut uStack8: u16;

  puVar6 = switch_1030_07ac(param_1,param_2,param_3,param_4,param_5,param_6,(param_6 >> 0x10),
                            param_7);
  uVar3 = (puVar6 >> 0x10);
  uVar1 = (puVar6 + 0x4);
  uVar2 = uVar1;
  uVar4 = uVar3;
  pass1_1028_e1ec(CONCAT22(param_4,param_3),param_7);
  uVar5 = uVar4 | uVar2;
  if (uVar5 != 0) {
    pass1_1030_7e5a(uVar5,(uVar2 & 0xffff | uVar4 << 0x10),uVar1);
  }
  uStack8 = (uVar1 >> 0x10);
  pass1_1030_1358(*(astruct_291 **)(param_3 + 0x26),puVar6,uVar3,
                  uVar1 & 0xffff | (uStack8 & 0xff) << 0x10);
  return;
}
pub fn pass1_1030_1120(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32)

{
  let mut puVar1: *mut u8;

  mem_op_1000_179c(0x3b2,param_2);
  puVar1 = (param_2 | param_1);
  if (puVar1.is_null()) {
    param_1 = 0;
    puVar1 = NULL;
  }
  else {
    struct_1030_2112(param_1,puVar1,CONCAT22(param_2,param_1),0x0);
  }
  pass1_1030_1358(*(astruct_291 **)(param_3 + 0x2a),param_1,puVar1,
                  (param_1 + 0x4) & 0xffff | ((param_1 + 0x6) & 0xff) << 0x10);
  return;
}



pub fn pass1_1030_117a(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1.address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1030_11aa(param_1: *mut astruct_156,param_2: i32,param_3: i32)

{
  iVar1: *mut astruct_156;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field0_0x0 = 0x389a;
  iVar1.field1_0x2 = 0x1008;
  iVar1.field2_0x4 = 0;
  iVar1.field3_0x6 = NULL;
  iVar1.field4_0xa = 0;
  iVar1.field5_0xe = param_3;
  iVar1.field6_0x12 = 0;
  iVar1.field7_0x16 = param_2;
  iVar1.field8_0x1a = 0x1;
    // just 0x1624
  param_1.field0_0x0 = s_462_bmp_1050_1620 + 0x4;
  iVar1.field1_0x2 = 0x1030;
  if (iVar1.field5_0xe == 0) {
    iVar1.field5_0xe = 0x5;
  }
  if (iVar1.field7_0x16 == 0) {
    iVar1.field7_0x16 = 0x5;
  }
  struct_1030_1550(param_1);
  *iVar1.field3_0x6 = 0;
  return;
}
pub fn pass1_1030_1244(param_1: *mut StructD)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut ppcVar4: *mut *mut code;
  let mut uVar5: u32;
  let mut iVar6: *mut StructD;
  let mut iVar7: i16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uStack6: u32;

  uVar9 = (param_1 >> 0x10);
  iVar6 = param_1;
  param_1.address_offset_field_0x0 = s_462_bmp_1050_1620 + 0x4;
  iVar6.address_offset_field_0x2 = 0x1030;
  if (iVar6.field14_0x1a != 0) {
    uStack6 = 0x1;
    while( true ) {
      puVar1 = &iVar6.field6_0xa;
      if (*puVar1 < uStack6 || *puVar1 == uStack6) break;
      iVar8 = uStack6 * 0x4;
      uVar5 = &iVar6.field_0x6;
      uVar10 = (uVar5 >> 0x10);
      iVar7 = uVar5;
      puVar2 = (iVar7 + iVar8);
      uVar3 = (iVar7 + iVar8 + 2);
      if ((uVar3 | puVar2) != 0) {
        ppcVar4 = *puVar2;
        (**ppcVar4)();
      }
      uStack6 += 0x1;
    }
  }
  fn_ptr_1000_17ce(*&iVar6.field_0x6);
  param_1.address_offset_field_0x0 = 0x389a;
  iVar6.address_offset_field_0x2 = 0x1008;
  return;
}
pub fn pass1_1030_12ca(param_1: *mut astruct_176)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u32;
  iVar3: *mut astruct_176;
  let mut uVar3: u16;
  let mut uStack6: u32;

  uStack6 = 0x1;
  while( true ) {
    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    puVar1 = &iVar3.field6_0xa;
    if (*puVar1 < uStack6 || *puVar1 == uStack6) {
      iVar3.field4_0x4 = 0;
      return;
    }
    uVar2 = iVar3.field5_0x6;
    if ((uVar2 + uStack6 * 0x4) == 0) break;
    uStack6 += 0x1;
  }
  return;
}
pub fn bad_1030_1312()

{
  return;
}
pub fn pass1_1030_1358(param_1: *mut astruct_291,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u16;
  let mut lVar3: i32;
  pstruct_291_2: *mut astruct_291;
  let mut iVar4: i16;
  pstruct_291_1: *mut astruct_291;
  let mut uVar5: u16;
  let mut bVar6: bool;

  if (param_4 == 0) {
    return;
  }
  pstruct_291_1 = (param_1 >> 0x10);
  pstruct_291_2 = param_1;
  puVar1 = &pstruct_291_2.field7_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (pstruct_291_2.field6_0x6 == 0)) {
    puVar2 = (&pstruct_291_2.field13_0x12 + 2);
    bVar6 = *puVar2 < param_4;
    if ((bVar6 || *puVar2 == param_4) &&
       ((bVar6 || (puVar1 = &pstruct_291_2.field13_0x12,
                  puVar1 < param_4 || puVar1 == param_4)))) {
      struct_1030_1550((param_1 & 0xffff | ZEXT24(pstruct_291_1) << 0x10));
    }
    puVar1 = &pstruct_291_2.field13_0x12;
    if (*puVar1 < param_4 || *puVar1 == param_4) {
      return;
    }
    if (pstruct_291_2.field6_0x6 == 0) {
      return;
    }
    puVar2 = &pstruct_291_2.field8_0xc;
    bVar6 = *puVar2 < param_4;
    if ((bVar6 || *puVar2 == param_4) &&
       ((bVar6 || (puVar2 = &pstruct_291_2.field7_0xa, *puVar2 < param_4 || *puVar2 == param_4)))) {
      pstruct_291_2.field7_0xa = (param_4 + 1);
      pstruct_291_2.field8_0xc = (param_4 + 0x1 >> 0x10);
    }
  }
  lVar3 = pstruct_291_2.field6_0x6;
  uVar5 = (lVar3 >> 0x10);
  iVar4 = lVar3;
  (iVar4 + param_4 * 0x4) = param_2;
  (iVar4 + param_4 * 0x4 + 0x2) = param_3;
  return;
}



pub unsafe fn pass1_1030_13f6(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_291,mut param_4: u32) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut puStack8: *mut u32;
  let mut uStack4: u16;

  uStack4 = 0;
  bad_1030_1312();
  puStack8 = CONCAT22(param_2,param_1);
  if ((param_2 | param_1) != 0) {
    uStack4 = 0x1;
    uVar2 = (param_3 >> 0x10);
    if (((param_3 + 0x1a) != 0) && ((param_2 | param_1) != 0)) {
      ppcVar1 = *puStack8;
      (**ppcVar1)();
    }
    pass1_1030_1358(param_3,0x0,0x0,param_4);
    (param_3 + 0x4) = 0x1;
  }
  return uStack4;
}
pub fn pass1_1030_145a(param_1: *mut astruct_346,param_2: i32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  pstruct_1: *mut astruct_346;
  pstruct_1_hi: *mut astruct_346;

  pstruct_1_hi = (param_1 >> 0x10);
  pstruct_1 = param_1;
  fn_ptr_1000_17ce(pstruct_1.field6_0x6);
  pstruct_1.field6_0x6 = 0;
  pstruct_1.field7_0xa = 0;
  uVar1 = pstruct_1.field10_0x16 + param_2;
  uVar2 = (uVar1 >> 0x10);
  if (uVar1 < pstruct_1.field8_0xe) {
    uVar1 = &pstruct_1.field8_0xe;
    uVar2 = (&pstruct_1.field8_0xe + 2);
  }
  &pstruct_1.field8_0xe = uVar1;
  (&pstruct_1.field8_0xe + 0x2) = uVar2;
  pstruct_1.field9_0x12 = 0;
  return;
}
pub fn pass1_1030_14b4(param_1: *mut astruct_156,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u16;
  let mut puVar3: *mut u32;
  pstruct_1: *mut astruct_156;
  iVar4: *mut astruct_344;
  pstruct_1_hi: *mut astruct_156;
  let mut uVar4: u16;
  let mut bVar5: bool;

  pstruct_1_hi = (param_1 >> 0x10);
  pstruct_1 = param_1;
  puVar1 = &pstruct_1.field4_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (pstruct_1.field3_0x6.is_null())) {
    puVar2 = (&pstruct_1.field6_0x12 + 2);
    bVar5 = *puVar2 < param_4;
    if ((bVar5 || *puVar2 == param_4) &&
       ((bVar5 || (puVar3 = &pstruct_1.field6_0x12, puVar3 < param_4 || puVar3 == param_4
                  )))) {
      struct_1030_1550((param_1 & 0xffff | ZEXT24(pstruct_1_hi) << 0x10));
    }
    puVar1 = &pstruct_1.field6_0x12;
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (pstruct_1.field3_0x6.is_null())) {
      return;
    }
    puVar2 = (&pstruct_1.field4_0xa + 2);
    bVar5 = *puVar2 < param_4;
    if ((bVar5 || *puVar2 == param_4) &&
       ((bVar5 || (puVar3 = &pstruct_1.field4_0xa, puVar3 < param_4 || puVar3 == param_4)
        ))) {
      &pstruct_1.field4_0xa = (param_4 + 1);
      (&pstruct_1.field4_0xa + 0x2) = (param_4 + 0x1 >> 0x10);
    }
  }
  puVar3 = pstruct_1.field3_0x6;
  uVar4 = (puVar3 >> 0x10);
  iVar4 = puVar3;
  (iVar4 + param_4 * 0x4) = param_2;
  (iVar4 + param_4 * 0x4 + 0x2) = param_3;
  return;
}
pub fn pass1_1030_154c()

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1030_1550(param_1: *mut astruct_156)

{
  i32 *plVar1;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut pSVar5: *mut StructD;
  iVar5: *mut astruct_156;
  let mut uVar6: u16;
  let mut lVar7: i32;
  let mut puStack10: *mut u32;
  let mut uStack6: u32;

  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  if (iVar5.field6_0x12 == 0) {
    uVar4 = &iVar5.field5_0xe;
    pSVar5 = (in_EDX & 0xffff0000 | (&iVar5.field5_0xe + 0x2));
  }
  else {
    uVar2 = &iVar5.field6_0x12;
    plVar1 = &iVar5.field7_0x16;
    uVar4 = uVar2 + plVar1;
    pSVar5 =
             (in_EDX & 0xffff0000 |
             ((&iVar5.field6_0x12 + 0x2) + (&iVar5.field7_0x16 + 0x2) +
                    CARRY2(uVar2,plVar1)));
  }
  uStack6 = CONCAT22(pSVar5,uVar4);
  if (iVar5.field3_0x6.is_null()) {
    if (_PTR_LOOP_1050_5f2c == 0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
      PTR_LOOP_1050_5f2e = pSVar5;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(uVar4 << 0x2,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
  }
  else {
    puVar3 = iVar5.field3_0x6;
    lVar7 = pass1_1000_0ed4(0x1,uVar4 * 0x4,
                            (pSVar5 * 0x2 + CARRY2(uVar4,uVar4)) * 0x2 +
                            CARRY2(uVar4 * 0x2,uVar4 * 0x2),puVar3,
                            (puVar3 >> 0x10));
    PTR_LOOP_1050_5f2e = (lVar7 >> 0x10);
    uVar4 = lVar7;
  }
  puStack10 = CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
  if ((PTR_LOOP_1050_5f2e | uVar4) != 0) {
    iVar5.field6_0x12 = uStack6;
    iVar5.field3_0x6 = puStack10;
  }
  return;
}



pub fn pass1_1030_15fe(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1030_1244(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1030_1628(param_1: *mut astruct_180)

{
  iVar1: *mut astruct_180;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field0_0x0 = 0x389a;
  iVar1.field1_0x2 = 0x1008;
  &iVar1.field_0x4 = 0;
  &iVar1.field_0x8 = 0;
  param_1.field0_0x0 = 0x17ba;
  iVar1.field1_0x2 = 0x1030;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_165e(param_1: *mut astruct_57,param_2: *mut astruct_175,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u16;
  iVar1: *mut astruct_175;
  let mut uVar2: u16;

  uVar1 = SUB42(param_1,0x0);
  uVar2 = (param_2 >> 0x10);
  iVar1 = param_2;
  param_2.field0_0x0 = 0x389a;
  iVar1.field1_0x2 = 0x1008;
  &iVar1.field2_0x4 = 0;
  iVar1.field4_0x8 = param_4;
  param_2.field0_0x0 = 0x17ba;
  iVar1.field1_0x2 = 0x1030;
  pass1_1030_5c8a(_PTR_LOOP_1050_5736,param_3);
  iVar1.field2_0x4 = param_4;
  iVar1.field3_0x6 = uVar1;
  return;
}
pub fn pass1_1030_16b2(param_1: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x17ba;
  (param_1 + 0x2) = 0x1030;
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  return;
}
pub fn pass1_1030_16d6(param_1: *mut astruct_731,mut param_2: u32)

{
  let mut BVar1: bool;
  let mut uVar2: u16;
  in_stack_0000ffd8: mut HFILE16;
  u32 local_10 [0x2];
  let mut local_8: u32;

  uVar2 = (param_1 >> 0x10);
  local_10[0] = (param_1 + 0x4);
  BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_10),0x4,in_stack_0000ffd8);
  if (BVar1 != 0) {
    local_8 = (param_1 + 0x8);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffd8);
    if (BVar1 != 0) {
      return;
    }
  }
  u16_1050_0310 = 0x6d0;
  return;
}
pub fn file_1030_1730(param_1: *mut astruct_373,HFILE16 *param_2)

{
  let mut BVar1: bool;

  BVar1 = read_file_1008_7dee(param_2,(param_1 & 0xffff0000 | (param_1 + 0x4)),0x4);
  if (BVar1 != 0) {
    BVar1 = read_file_1008_7dee(param_2,(param_1 & 0xffff0000 | (param_1 + 0x8)),0x4);
    if (BVar1 != 0) {
      return;
    }
  }
  u16_1050_0310 = 0x6d2;
  return;
}
pub fn pass1_1030_177a(mut param_1: u32,mut param_2: u32)

{
  (param_1 + 0x8) = param_2;
  return;
}
pub fn FUN_1030_178e()

{
  return;
}



pub fn pass1_1030_1794(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1030_16b2(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1030_17ce(param_1: *mut astruct_180,mut param_2: u32,mut param_3: u32,param_4: *mut astruct_57)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  iVar4: *mut astruct_180;
  uVar4: *mut astruct_180;

  uVar1 = struct_1030_1628(param_1);
  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  &iVar4.field10_0xc = 0;
  param_1.field0_0x0 = 0x1a16;
  iVar4.field1_0x2 = 0x1030;
  if ((param_3 != 0) || (param_2 != 0)) {
    mem_op_1000_179c(0x18,param_4);
    uVar3 = param_4 | uVar1;
    if (uVar3 == 0) {
      uVar2 = 0;
      uVar3 = 0;
    }
    else {
      uVar2 = struct_op_1030_1cd8(CONCAT22(param_4,uVar1),param_2,param_3);
    }
    iVar4.field10_0xc = uVar2;
    iVar4.field11_0xe = uVar3;
  }
  return &param_1.field0_0x0;
}



u16 * pass1_1030_183c(mut param_1: u16 ,param_2: *mut astruct_57,param_3: *mut u16,mut param_4: u32,mut param_5: u32,mut param_6: u32,
                        mut param_7: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;

  pass1_1030_165e(param_2,param_3,param_6,param_7);
  uVar4 = (param_3 >> 0x10);
  iVar3 = param_3;
  (iVar3 + 0xc) = 0;
  *param_3 = 0x1a16;
  (iVar3 + 0x2) = 0x1030;
  if ((param_5 != 0) || (param_4 != 0)) {
    mem_op_1000_179c(0x18,param_2);
    uVar2 = param_2 | param_1;
    if (uVar2 == 0) {
      uVar1 = 0;
      uVar2 = 0;
    }
    else {
      uVar1 = struct_op_1030_1cd8(CONCAT22(param_2,param_1),param_4,param_5);
    }
    (iVar3 + 0xc) = uVar1;
    (iVar3 + 0xe) = uVar2;
  }
  return param_3;
}
pub fn pass1_1030_18b2(param_1: *mut u16)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x1a16;
  (iVar4 + 0x2) = 0x1030;
  puVar1 = (iVar4 + 0xc);
  uVar2 = (iVar4 + 0xe);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  pass1_1030_16b2(param_1);
  return;
}
pub fn pass1_1030_18f0(mut param_1: u16 ,mut param_2: u32,mut param_3: i16,mut param_4: i16)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar4 = (param_2 >> 0x10);
  iVar3 = param_2;
  if ((iVar3 + 0xc) != 0) {
    ppcVar1 = ((iVar3 + 0xc) + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX,param_1);
    for (uStack10 = 0; uStack10 < uStack6; uStack10 += 1) {
      ppcVar1 = ((iVar3 + 0xc) + 0x4);
      uVar2 = uStack6;
      (**ppcVar1)();
      if ((uVar2 == param_3) && (extraout_DX_00 == param_4)) {
        ppcVar1 = ((iVar3 + 0xc) + 0x8);
        (**ppcVar1)();
      }
    }
  }
  return;
}



pub unsafe fn pass1_1030_1972() -> u16

{
  return 0x1;
}



pub unsafe fn pass1_1030_1978(param_1: u16,param_2: *mut astruct_731,mut param_3: u32) -> u16

{
  pass1_1030_16d6(param_2,param_3);
  if (param_1 != 0) {
    write_to_file_1008_7954(param_1,param_3,(param_2 + 0xc));
    if (param_1 == 0) {
      u16_1050_0310 = 0x6d0;
      return param_1;
    }
    param_1 = 0x1;
  }
  return param_1;
}
pub fn file_1030_19b4(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_373,HFILE16 *param_4)

{
  i32 *plVar1;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;

  paVar2 = CONCAT22(in_register_0000000a,param_2);
  file_1030_1730(param_3,param_4);
  if (param_1 != 0) {
    plVar1 = (param_3 & 0xffff0000 | (param_3 + 0xc));
    file_1008_76e4(paVar2,param_4,plVar1);
    if (plVar1 == 0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
  }
  return;
}



pub fn pass1_1030_19f0(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1030_18b2(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1030_1a32(param_1: *mut u16,mut param_2: u16 ,param_3: *mut u8)

{
  let mut in_register_0000000a: u16;

  pass1_1030_183c(param_2,CONCAT22(in_register_0000000a,param_3),param_1,0x1,0x16,0xff000000,0x0);
  PTR_LOOP_1050_5168 = (param_1 >> 0x10);
  PTR_LOOP_1050_5166 = param_1;
  (PTR_LOOP_1050_5166 + 0x10) = 0;
  *param_1 = 0x1cbc;
  (PTR_LOOP_1050_5166 + 0x2) = 0x1030;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_1a74(param_1: *mut u16)

{
  *param_1 = 0x1cbc;
  (param_1 + 0x2) = 0x1030;
  _PTR_LOOP_1050_5166 = 0;
  pass1_1030_18b2(param_1);
  return;
}



pub unsafe fn pass1_1030_1a9c(mut param_1: u32,mut param_2: u32) -> u16

{
  let mut piVar1: *mut i16;
  let mut in_AX: u16;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut iVar4: i16;
  let mut uVar5: u16;
  in_stack_0000ffde: mut HFILE16;
  u16 local_c [0x5];

  uVar2 = pass1_1030_1978(in_AX,param_1,param_2);
  if (uVar2 != 0) {
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    local_c[0] = (iVar4 + 0x10);
    BVar3 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffde);
    if (BVar3 != 0) {
      if (*(iVar4 + 0x10) == 0) {
        return 0x1;
      }
      piVar1 = (iVar4 + 0x10);
      BVar3 = write_to_file_1008_7e1c
                        (param_2,(piVar1 + 0x2),(*piVar1 * 0x2),
                         in_stack_0000ffde);
      if (BVar3 != 0) {
        return 0x1;
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn file_1030_1b18(mut param_1: i16,param_2: *mut u8,mut param_3: u32,mut param_4: u32) -> u16

{
  let mut uVar1: u16;
  let mut piVar2: *mut i16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut BVar5: bool;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut pSVar7: *mut StructD;
  let mut paVar8: *mut Struct57;
  let mut iVar9: i16;
  iVar7: *mut astruct_368;
  let mut uVar10: u16;
  let mut uVar11: u16;

  pSVar7 = CONCAT22(in_register_0000000a,param_2);
  file_1030_19b4(param_1,param_2,param_3,(HFILE16 *)param_4);
  if (param_1 != 0) {
    if (_PTR_LOOP_1050_5f2c == 0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
    }
    else {
      pSVar7 = (pSVar7 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    uVar4 = fn_ptr_op_1000_1708(0x6,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar7);
    uVar10 = (param_3 >> 0x10);
    iVar9 = param_3;
    (iVar9 + 0x10) = uVar4;
    (iVar9 + 0x12) = pSVar7;
    uVar1 = (iVar9 + 0x12);
    paVar8 = (pSVar7 & 0xffff0000 | uVar1);
    BVar5 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(uVar1,(iVar9 + 0x10)),0x2);
    if (BVar5 != 0) {
      piVar2 = (iVar9 + 0x10);
      if (*piVar2 == 0) {
        return 0x1;
      }
      uVar1 = *piVar2 * 0x2;
      uVar6 = uVar1;
      mem_op_1000_179c(uVar1,paVar8);
      uVar3 = (iVar9 + 0x10);
      uVar11 = (uVar3 >> 0x10);
      iVar7 = uVar3;
      iVar7.field2_0x2 = uVar6;
      iVar7.field3_0x4 = paVar8;
      uVar3 = (iVar9 + 0x10);
      BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(uVar3 + 0x2),uVar1);
      if (BVar5 != 0) {
        return 0x1;
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_1be2(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u16;
  let mut uVar3: *mut Struct57;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uStack4: u16;

  uVar4 = (param_3 >> 0x10);
  iVar3 = param_3;
  if ((iVar3 + 0xc) == 0) {
    mem_op_1000_179c(0x18,param_2);
    uVar3 = param_2;
    param_2 = (param_2 & 0xffff0000 | (uVar3 | param_1));
    if ((uVar3 | param_1) == 0) {
      (iVar3 + 0xc) = 0;
    }
    else {
      struct_op_1030_1cd8(CONCAT22(uVar3,param_1),0x5,0x5);
      (iVar3 + 0xc) = param_1;
      (iVar3 + 0xe) = param_2;
    }
  }
  for (uStack4 = 0; puVar2 = (u16*)(iVar3 + 0x10), uStack4 <= *puVar2 && *puVar2 != uStack4; uStack4 += 1) {
    uVar5 = pass1_1028_e2e0(param_2,_PTR_LOOP_1050_65e2,1);
    param_2 = (param_2 & 0xffff0000 | uVar5 >> 0x10);
    ppcVar1 = ((iVar3 + 0xc) + 0x8);
    (**ppcVar1)(0x1028,(iVar3 + 0xc),uVar5,(uVar5 >> 0x10),uStack4,0x0);
  }
  return;
}



pub fn pass1_1030_1c96(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1030_1a74(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_op_1030_1cd8(param_1: *mut astruct_75,mut param_2: u32,mut param_3: u32)

{
  struct_var1: *mut astruct_75;
  struct_var2: *mut astruct_75;

  struct_var2 = (param_1 >> 0x10);
  struct_var1 = param_1;
  param_1.field0_0x0 = 0x389a;
  struct_var1.field1_0x2 = 0x1008;
  struct_var1.field2_0x4 = 0;
  struct_var1.field3_0x8 = 0;
  struct_var1.field4_0xc = param_3;
  struct_var1.field5_0x10 = 0;
  struct_var1.field6_0x14 = param_2;
  param_1.field0_0x0 = 0x2044;
  struct_var1.field1_0x2 = 0x1030;
  return;
}
pub fn pass1_1030_1d28(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x2044;
  iVar1.address_offset_field_0x2 = 0x1030;
  fn_ptr_1000_17ce(*&iVar1.hfile_0x4);
  param_1.address_offset_field_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_1d58(mut param_1: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;

  ppcVar1 = (param_1 + 0x4);
  uVar2 = (**ppcVar1)();
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  return;
}



pub fn pass1_1030_1d7c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32) -> u32

{
  let mut uVar1: u32;

  pass1_1030_1d58(param_3);
  if ((param_2 | param_1) != 0) {
    uVar1 = struct_op_1030_73a8(CONCAT22(param_2,param_1),param_1,param_2 | param_1);
    return uVar1;
  }
  return 0x0;
}



pub fn pass1_1030_1daa(mut param_1: u32) -> u32

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0xa),(param_1 + 0x8));
}
pub fn pass1_1030_1dbc()

{
  return;
}
pub fn pass1_1030_1dfc(param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u16;
  let mut ppcVar3: *mut *mut code;
  let mut uVar4: u32;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut bVar7: bool;

  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  puVar1 = (iVar5 + 0x8);
  if ((*puVar1 < param_4 || *puVar1 == param_4) || ((iVar5 + 0x4) == 0)) {
    puVar2 = (iVar5 + 0x12);
    bVar7 = *puVar2 < param_4;
    if ((bVar7 || *puVar2 == param_4) &&
       ((bVar7 || (puVar2 = (iVar5 + 0x10), *puVar2 < param_4 || *puVar2 == param_4)))) {
      ppcVar3 = (*param_1 + 0x20);
      (**ppcVar3)();
    }
    puVar1 = (iVar5 + 0x10);
    if ((*puVar1 < param_4 || *puVar1 == param_4) || ((iVar5 + 0x4) == 0)) {
      return;
    }
    puVar2 = (iVar5 + 0xa);
    bVar7 = *puVar2 < param_4;
    if ((bVar7 || *puVar2 == param_4) &&
       ((bVar7 || (puVar2 = (iVar5 + 0x8), *puVar2 < param_4 || *puVar2 == param_4)))) {
      (iVar5 + 0x8) = (param_4 + 1);
      (iVar5 + 0xa) = (param_4 + 0x1 >> 0x10);
    }
  }
  uVar4 = (iVar5 + 0x4);
  uVar6 = (uVar4 >> 0x10);
  iVar5 = uVar4;
  (iVar5 + param_4 * 0x4) = param_2;
  (iVar5 + param_4 * 0x4 + 0x2) = param_3;
  return;
}
pub fn pass1_1030_1e96(param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uStack6: u32;

  uStack6 = 0;
  while( true ) {
    uVar4 = (param_1 >> 0x10);
    puVar1 = (param_1 + 0x8);
    if ((*puVar1 < uStack6 || *puVar1 == uStack6) ||
       (uVar3 = (param_1 + 0x4), (uVar3 + uStack6 * 0x4) == 0)) break;
    uStack6 += 0x1;
  }
  ppcVar2 = (*param_1 + 0x8);
  (**ppcVar2)();
  return;
}
pub fn pass1_1030_1eee(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar1 = (iVar2 + 0xc);
  param_2 = (iVar2 + 0xe);
  if (uVar1 < param_2) {
    uVar1 = param_2 & 0xffff;
  }
  (iVar2 + 0xc) = uVar1;
  (iVar2 + 0xe) = param_2;
  return;
}
pub fn pass1_1030_1f16(param_1: u32,mut param_2: u32)

{
  i32 *plVar1;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (((iVar4 + 0x4) == 0) || ((iVar4 + 0x10) <= (iVar4 + 0x8))) {
    ppcVar2 = (*param_1 + 0x20);
    (**ppcVar2)();
  }
  uVar3 = (iVar4 + 0x4);
  ((iVar4 + 0x8) * 0x4 + uVar3) = param_2;
  plVar1 = (iVar4 + 0x8);
  *plVar1 = *plVar1 + 1;
  return;
}
pub fn FUN_1030_1f6c()

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn FUN_1030_1f70(mut param_1: u16 ,mut param_2: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut pSVar5: *mut StructD;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut lVar8: i32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar7 = (param_2 >> 0x10);
  iVar6 = param_2;
  if ((iVar6 + 0x10) == 0) {
    uVar4 = (iVar6 + 0xc);
    pSVar5 = (in_EDX & 0xffff0000 | (iVar6 + 0xe));
  }
  else {
    uVar2 = (iVar6 + 0x10);
    puVar1 = (iVar6 + 0x14);
    uVar4 = uVar2 + *puVar1;
    pSVar5 =
             (in_EDX & 0xffff0000 |
             ((iVar6 + 0x12) + (iVar6 + 0x16) + CARRY2(uVar2,*puVar1)));
  }
  uStack6 = CONCAT22(pSVar5,uVar4);
  if ((iVar6 + 0x4) == 0) {
    if (_PTR_LOOP_1050_5f2c == 0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
      PTR_LOOP_1050_5f2e = pSVar5;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(uVar4 << 0x2,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
  }
  else {
    uVar3 = (iVar6 + 0x4);
    lVar8 = pass1_1000_0ed4(0x1,uVar4 * 0x4,
                            (pSVar5 * 0x2 + CARRY2(uVar4,uVar4)) * 0x2 +
                            CARRY2(uVar4 * 0x2,uVar4 * 0x2),uVar3,
                            (uVar3 >> 0x10));
    PTR_LOOP_1050_5f2e = (lVar8 >> 0x10);
    uVar4 = lVar8;
  }
  uStack10 = CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
  if ((PTR_LOOP_1050_5f2e | uVar4) != 0) {
    (iVar6 + 0x10) = uStack6;
    (iVar6 + 0x4) = uStack10;
  }
  return;
}
