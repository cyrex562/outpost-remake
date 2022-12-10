
pub fn pass1_1030_b13c()

{
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1030_b142(mut param_1: u32,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut bVar4: bool;
  let mut uVar5: u32;
  let mut uStack12: u32;

  uVar5 = struct_op_1030_73a8(param_2,in_AX,in_DX);
  uVar3 = (uVar5 >> 0x10);
  iVar1 = uVar5;
  iVar2 = (iVar1 + 0xc);
  uStack12 = 0;
  if (iVar2 == 0x18) {
    uStack12 = pass1_1028_1c1c();
    uVar3 = (iVar1 + 0x22);
  }
  else {
//    if (iVar2 != 0x3f) goto LAB_1030_b1a6;
    uStack12 = pass1_1028_20b0();
    uVar3 = (iVar1 + 0x24);
  }
  uStack12 = CONCAT22(uStack12,uVar3);//
// LAB_1030_b1a6:
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0xe) == 1) {
    bVar4 = (uStack12 & 0x10000) == 0;
  }
  else if ((iVar2 + 0xe) == 0x2) {
    bVar4 = (uStack12 & 0x20000) == 0;
  }
  else if ((iVar2 + 0xe) == 0x3) {
    bVar4 = (uStack12 & 0x40000) == 0;
  }
  else {
    bVar4 = (uStack12 & 0x80000) == 0;
  }
  if ((bVar4) || (uStack12 != 0)) {
    bVar4 = false;
    loop {
//      if (((uStack12 & 0x10000) != 0) && (uStack12 == 0)) goto LAB_1030_b239;
//      if (((uStack12 & 0x20000) != 0) && (uStack12 == 0)) goto LAB_1030_b247;
//      if (((uStack12 & 0x40000) != 0) && (uStack12 == 0)) goto LAB_1030_b255;
//      if (((uStack12 & 0x80000) != 0) && (uStack12 == 0)) goto LAB_1030_b263;
      if (bVar4) break;
      uStack12._1_3_ = (uStack12 >> 0x8) & 0xffff00;
      iVar1 = (iVar2 + 0xe);
      if (iVar1 == 1) {
        uStack12 = CONCAT31(uStack12._1_3_,0x4);
      }
      else if (iVar1 == 0x2) {
        uStack12 = CONCAT31(uStack12._1_3_,0x8);
      }
      else if (iVar1 == 0x3) {
        uStack12 = CONCAT31(uStack12._1_3_,1);
      }
      else {
        uStack12 = CONCAT31(uStack12._1_3_,0x2);
      }
      bVar4 = true;
    }
    if ((iVar2 + 0xe) == 1) {//
// LAB_1030_b255:
      (iVar2 + 0xe) = 0x3;
      return;
    }
    if ((iVar2 + 0xe) == 0x2) {//
// LAB_1030_b263:
      (iVar2 + 0xe) = 0x4;
      return;
    }
    if ((iVar2 + 0xe) == 0x3) {//
// LAB_1030_b239:
      (iVar2 + 0xe) = 0x1;
      return;
    }
    if ((iVar2 + 0xe) == 0x4) {//
// LAB_1030_b247:
      (iVar2 + 0xe) = 0x2;
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_b2aa(param_1: *mut u8,mut param_2: u32,param_3: *mut u16)

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  let mut uVar3: u32;
  let mut bStack23: u8;
  let mut local_6: u32;

  pass1_1030_b718(param_1,param_2,(param_2 >> 0x10),param_3,CONCAT22(0x1050,&local_6));
  bStack23 = (local_6 >> 0x18);
  uVar1 = bStack23;
  if (bStack23 != 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,local_6 & 0xffff | local_6 << 0x10);
    if ((local_6 | uVar1) != 0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(local_6,uVar1),uVar1,local_6 | uVar1);
      BVar2 = pass1_1008_c6ae(_u16_1050_06e0,(uVar3 + 0xc),0x42);
      if (BVar2 != 0) {
        pass1_1008_3f62((param_2 & 0xffff0000 | (param_2 + 0x8)),
                        CONCAT22(local_6,uVar1 + 0xc));
        return;
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1030_b344(mut param_1: u32) -> u32

{
  let mut puVar1: *mut u8;
  let mut puStack18: *mut u32;
  let mut puStack16: *mut u8;
  let mut local_e: [u8;0x2] = [0;0x2];
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  local_8 = (param_1 + 0x8);
  uStack4 = (param_1 + 0xc);
  puVar1 = param_1;
  pass1_1008_3eb4(CONCAT22(0x1050,&local_8),CONCAT22(0x1050,local_e),
                  CONCAT22(0x1050,&local_c),CONCAT22(0x1050,&local_a));
  local_8 = local_8 & 0xffff | (local_c - 1) << 0x10;
  puStack18 = &local_8;
  pass1_1030_b2aa(puVar1,param_1,CONCAT22(0x1050,puStack18));
  puStack16 = (puVar1 | puStack18);
  if (puStack16.is_null()) {
    local_8 = local_8 & 0xffff | (local_c + 1) << 0x10;
    puStack18 = &local_8;
    pass1_1030_b2aa(NULL,param_1,CONCAT22(0x1050,puStack18));
    puVar1 = (puStack16 | puStack18);
    if (puVar1.is_null()) {
      local_8 = local_a + -0x1;
      local_8 = local_c;
      puStack18 = &local_8;
      pass1_1030_b2aa(NULL,param_1,CONCAT22(0x1050,puStack18));
      puStack16 = (puVar1 | puStack18);
      if (puStack16.is_null()) {
        local_8 = CONCAT22(local_8,local_a + 1);
        puStack18 = &local_8;
        pass1_1030_b2aa(NULL,param_1,CONCAT22(0x1050,puStack18));
        if ((puStack16 | puStack18) == 0) {
          return 0x0;
        }
        (param_1 + 0xe) = 0x2;
      }
      else {
        (param_1 + 0xe) = 0x4;
        puStack16 = puVar1;
      }
    }
    else {
      (param_1 + 0xe) = 0x3;
    }
  }
  else {
    (param_1 + 0xe) = 0x1;
    puStack16 = puVar1;
  }
  return CONCAT22(puStack16,puStack18);
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1030_b454(mut param_1: u32,mut param_2: u32)

{
  let mut puVar1: *mut u32;
  let mut ppcVar2: *mut *mut code;
  let mut puVar3: *mut u8;
  let mut extraout_DX: u16;
  let mut iVar4: i16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut uVar9: u32;
  let mut lStack38: i32;
  let mut uStack30: u32;
  let mut local_12: [u8;0x4] = [0;0x4];
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut lStack6: i32;

  lStack6 = (param_2 + 0x4);
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  pass1_1008_5784(CONCAT22(0x1050,local_12),(iVar6 + 0x10));
  loop {
    puVar3 = local_12;
    pass1_1008_5b12(CONCAT22(0x1050,puVar3));
    uStack10 = CONCAT22(extraout_DX,puVar3);
    if ((extraout_DX | puVar3) == 0) break;
    if ((puVar3 + 0x20) == lStack6) {
      ppcVar2 = ((iVar6 + 0x10) + 0xc);
      (**ppcVar2)();
      uStack14 = 0;
      pass1_1038_69fe(uStack10);
    }
  }
  uVar8 = struct_op_1030_73a8(param_2,puVar3,0x0);
  iVar4 = (uVar8 >> 0x10);
  puVar1 = (uVar8 + 0x20);
  puVar3 = local_12;
  pass1_1008_5784(CONCAT22(0x1050,puVar3),puVar1);
  pass1_1030_b13c();
  uStack30 = CONCAT22(-((s_Unsupported_FileStructType_in_Op_1050_01ca + 0x2aU) < puVar3) - iVar4
                      ,0x1f4 - puVar3);
  loop {
    puVar3 = local_12;
    pass1_1008_5b12(CONCAT22(0x1050,puVar3));
    uStack10 = CONCAT22(extraout_DX_00,puVar3);
    uVar5 = extraout_DX_00 | puVar3;
    if (uVar5 == 0) {
      return;
    }
    pass1_1038_6984(CONCAT22(extraout_DX_00,puVar3));
    lStack38 = CONCAT22(uVar5,puVar3);
    if ((uVar5 <= uStack30) && ((uVar5 < uStack30 || (puVar3 <= uStack30)))) {
      uVar9 = (iVar6 + 0x10);
      ppcVar2 = ((iVar6 + 0x10) + 0x8);
      (**ppcVar2)();
      uStack30 -= lStack38;
      ppcVar2 = (*puVar1 + 0xc);
      (**ppcVar2)(&u16_1050_1038,puVar1,(puVar1 >> 0x10),uStack10,uVar9);
      uStack14 = 0;
    }
  } while (0x0 < uStack30);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_b578(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut puVar4: *mut u8;
  let mut bVar5: bool;
  let mut uVar6: u32;
  let mut uStack48: u32;
  let mut local_1c: [u8;0x2] = [0;0x2];
  let mut local_1a: i16;
  let mut local_18: i16;
  let mut local_16: u32;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut local_6: u32;

  pass1_1030_b718(param_1,param_1,param_1,
                  (param_1 & 0xffff0000 | (param_1 + 0x8)),CONCAT22(0x1050,&local_6));
  uStack48._3_1_ = (local_6 >> 0x18);
  uStack10 = uStack48._3_1_;
  if (uStack48._3_1_ == 0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,local_6 & 0xffff | local_6 << 0x10);
  uStack8 = local_6;
  uStack14 = struct_op_1030_73a8(CONCAT22(local_6,uStack10),uStack10,local_6);
  uStack16 = (uStack14 + 0xc);
  local_16 = (param_1 + 0x8);
  uStack18 = (param_1 + 0xc);
  puVar4 = param_1;
  pass1_1008_3eb4(CONCAT22(0x1050,&local_16),CONCAT22(0x1050,local_1c),
                  CONCAT22(0x1050,&local_1a),CONCAT22(0x1050,&local_18));
  iVar1 = (param_1 + 0xe);
  if (iVar1 == 0) {
    pass1_1030_b344(param_1 & 0xffff | ZEXT24(param_1) << 0x10);
    return;
  }
  if (iVar1 == 1) {
    uVar3 = local_1a - 0x1;//
// LAB_1030_b63e:
    local_16 = local_16 & 0xffff | uVar3 << 0x10;
    puVar2 = &local_16;
    pass1_1030_b2aa(puVar4,param_1 & 0xffff | ZEXT24(param_1) << 0x10,CONCAT22(0x1050,puVar2));
    uStack48 = CONCAT22(puVar4,puVar2);
    if ((puVar4 | puVar2) == 0) {
      return;
    }
    uVar6 = struct_op_1030_73a8(CONCAT22(puVar4,puVar2),puVar2,puVar4 | puVar2);
    uVar3 = (uVar6 + 0xc);
//    if (uVar3 == 0x3f) goto LAB_1030_b6e0;
    if (0x3f < uVar3) {
      return;
    }
//    if (uVar3 == '\x16') goto LAB_1030_b6e0;
    bVar5 = uVar3 == '\x18';
  }
  else {
    if (iVar1 == 0x2) {
      uVar3 = local_18 + 1;
    }
    else {
      if (iVar1 == 0x3) {
        uVar3 = local_1a + 1;
    // TODO: goto LAB_1030_b63e;
      }
      if (iVar1 != 0x4) {
        return;
      }
      uVar3 = local_18 - 0x1;
    }
    local_16 = local_16 & 0xffff0000 | uVar3;
    puVar2 = &local_16;
    pass1_1030_b2aa(puVar4,param_1 & 0xffff | ZEXT24(param_1) << 0x10,CONCAT22(0x1050,puVar2));
    uStack48 = CONCAT22(puVar4,puVar2);
    if ((puVar4 | puVar2) == 0) {
      return;
    }
    uVar6 = struct_op_1030_73a8(CONCAT22(puVar4,puVar2),puVar2,puVar4 | puVar2);
    iVar1 = (uVar6 + 0xc);
    if (iVar1 < 0x17) {
      return;
    }
    if (SBORROW2(iVar1,0x17)) {
      return;
    }
//    if (iVar1 == 0x18 || iVar1 + -0x17 < 1) goto LAB_1030_b6e0;
    bVar5 = iVar1 == 0x3f;
  }
  if (!bVar5) {
    return;
  }//
// LAB_1030_b6e0:
  pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x8)),
                  (uStack48 & 0xffff0000 | (uStack48 + 0xc)));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_b718(param_1: *mut u8,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,param_5: *mut u32)

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
pub fn pass1_1030_b768(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut iVar3: i16;
  let mut puVar4: *mut u8;
  let mut extraout_DX: u16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  in_stack_0000ffc8: HFILE16;
  let mut local_22: [u16;0x4] = [0;0x4];
  let mut local_1a: [u8;0xa] = [0;0xa];
  let mut local_10: u32;
  let mut puStack12: *mut u8;
  let mut uStack10: u16;
  let mut local_8: [u16;0x3] = [0;0x3];

  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  local_10 = (iVar5 + 0x4);
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_10),0x4,in_stack_0000ffc8);
  if ((BVar2 != 0) &&
     (iVar3 = write_to_file_1008_7b4c(param_2,(param_1 & 0xffff0000 | (iVar5 + 0x8))),
     iVar3 != 0)) {
    local_8[0] = (iVar5 + 0xe);
    BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffc8);
    if (BVar2 != 0) {
      uVar1 = (iVar5 + 0x10);
      local_22[0] = (uVar1 + 0x8);
      local_10 = local_10 & 0xffff0000 | local_22[0];
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_22),0x2,in_stack_0000ffc8);
      if (BVar2 == 0) {
        return;
      }
      pass1_1008_5784(CONCAT22(0x1050,local_1a),(iVar5 + 0x10));
      loop {
        puVar4 = local_1a;
        pass1_1008_5b12(CONCAT22(0x1050,puVar4));
        if ((extraout_DX | puVar4) == 0) {
          return;
        }
        puStack12 = puVar4;
        uStack10 = extraout_DX;
        pass1_1038_75ca(puVar4,CONCAT22(extraout_DX,puVar4),param_2);
      } while (puVar4.is_null() == false);
      return;
    }
  }
  u16_1050_0310 = 0x6d0;
  return;
}
pub fn file_1030_b836(param_1: *mut u8,param_2: *mut astruct_401,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  iVar4: *mut astruct_401;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut uVar8: u16;
  let mut uVar9: u32;
  let mut local_12: [u16;0x7] = [0;0x7];
  let mut local_4: u16;
  let mut paVar7: *mut Struct57;

  paVar6 = CONCAT22(in_register_0000000a,param_1);
  iVar4 = param_2;
  iVar4 = &iVar4.field4_0x4;
  BVar2 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | ZEXT24(iVar4)),0x4);
  if (((BVar2 == 0) ||
      (BVar2 = read_file_1008_7bc8(param_3,(param_2 & 0xffff0000 | ZEXT24(&iVar4.field_0x8))),
      BVar2 == 0)) ||
     (BVar2 = read_file_1008_7dee(param_3,CONCAT22(0x1050,&local_4),0x2), BVar2 == 0)) {
    u16_1050_0310 = 0x6d2;
  }
  else {
    uVar8 = (param_2 >> 0x10);
    iVar4.field13_0xe = local_4;
    BVar2 = read_file_1008_7dee(param_3,CONCAT22(0x1050,local_12),0x2);
    if (BVar2 != 0) {
      loop {
        if (local_12[0] == 0) {
          return;
        }
        uVar3 = local_12[0];
        uVar9 = param_3;
        local_12[0] = local_12[0] - 0x1;
        mem_op_1000_179c(0x2a,paVar6);
        uVar4 = paVar6;
        uVar5 = uVar4 | uVar3;
        paVar6 = (paVar6 & 0xffff0000);
        paVar7 = (paVar6 | uVar5);
        if (uVar5 == 0) {
          uVar3 = 0;
        }
        else {
          struct_1038_6520(CONCAT22(uVar4,uVar3));
          paVar6 = paVar7;
        }
        file_1038_774e(paVar6,CONCAT22(paVar6,uVar3),uVar9);
        if (uVar3 == 0) break;
        ppcVar1 = (*iVar4.field14_0x10 + 0x4);
        (**ppcVar1)();
      }
    }
  }
  return;
}



pub fn pass1_1030_b90c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1030_afa6(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1030_b936(mut param_1: u16 ,param_2: *mut astruct_365,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  pass1_1028_b22c(param_1,CONCAT22(param_3,param_2),param_4,param_5);
  param_2.field12_0xe = 0;
  param_2.field13_0x12 = 0;
  CONCAT22(param_3,param_2) = 0xbc0c;
  param_2.field1_0x2 = 0x1030;
  return;
}
pub fn pass1_1030_b96c(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut pcVar2: *mut c_char;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  *param_1 = 0xbc0c;
  (iVar3 + 0x2) = 0x1030;
  pcVar2 = *(iVar3 + 0xe);
  uVar1 = (iVar3 + 0x10);
  if ((uVar1 | pcVar2) != 0) {
    fn_ptr_1020_ba7e((pcVar2 & 0xffff | uVar1 << 0x10));
    fn_ptr_1000_17ce(pcVar2);
  }
  pass1_1028_b260(param_1);
  return;
}
pub fn pass1_1030_b9b2(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + 0xe) = 0;
  (param_1 + 0x12) = 0;
  return;
}
pub fn pass1_1030_b9da(param_1: *mut astruct_57,param_2: *mut astruct_172,param_3: *mut astruct_402,mut param_4: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut uVar3: *mut Struct57;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut paVar7: *mut Struct57;
  iVar7: *mut astruct_402;
  uVar6: *mut astruct_402;
  let mut uVar8: u32;
  let mut uStack12: u16;
  let mut uStack4: u16;
  let mut paVar6: *mut Struct57;

  uVar6 = (param_3 >> 0x10);
  iVar7 = param_3;
  paVar6 = param_1;
  if (&iVar7.field14_0xe == 0) {
    mem_op_1000_179c(0xa,param_1);
    uVar3 = (param_1 | param_2);
    paVar6 = ZEXT24(uVar3);
    if (uVar3.is_null()) {
      iVar7.field14_0xe = 0;
    }
    else {
      pass1_1020_ba3e((param_2 & 0xffff | param_1 << 0x10),0x5,0x5);
      iVar7.field14_0xe = param_2;
      iVar7.field15_0x10 = paVar6;
    }
    iVar7.field16_0x12 = 0;
  }
  for (uStack4 = 0x4; uStack4 < 0xe; uStack4 += 1) {
    uVar8 = pass1_1030_7c28(param_2,paVar6,param_4,uStack4);
    uVar2 = (uVar8 >> 0x10);
    param_2 = (uVar8 & 0xffff);
    uVar5 = uVar2 | param_2;
    paVar6 = uVar5;
    if (uVar5 != 0) {
      uVar4 = 0x64 - iVar7.field16_0x12;
      paVar7 = (uVar4 >> 0x10);
      uStack12 = uVar8;
      if (uVar8 < uVar4) {
        uVar4 = uVar8 & 0xffff;
        paVar7 = uVar2;
      }
      uVar5 = uVar4;
      param_2 = (uVar4 & 0xffff | paVar7 << 0x10);
      paVar6 = paVar7;
      pass1_1030_7d1c(uVar5,paVar7,param_4,uStack12 - uVar5,
                      CONCAT22(uStack4,(uVar2 - paVar7) - (uStack12 < uVar5)));
      pass1_1020_bb8a(*(i32 **)&iVar7.field14_0xe,uVar5,paVar7 | uStack4 << 0x10);
      puVar1 = &iVar7.field16_0x12;
      *puVar1 = &param_2.field0_0x0 + *puVar1;
      string_1020_c0ca(uStack4);
      vsprintf_op_1030_840a(paVar6,s_truck_0x_08lx_loaded__ld_of__s_f_1050_576c);
      if (0x63 < iVar7.field16_0x12) break;
    }
  }
  if (iVar7.field16_0x12 != 0) {
    return;
  }
  return;
}
pub fn pass1_1030_bb0e(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,mut param_4: u16 )

{
  let mut puVar1: *mut u32;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut uVar7: u32;
  let mut uStack8: u16;
  let mut paVar6: *mut Struct57;

  uVar3 = pass1_1030_7bee(param_3);
  uVar4 = uVar3;
  if (uVar3 != 0) {
    return;
  }
  pass1_1030_b9b2(param_2);
  uVar2 = uVar4 & 0xffff;
  puVar1 = (uVar2 | param_4 << 0x10);
  uVar5 = param_4 | uVar4;
  paVar6 = CONCAT22(in_register_0000000a,uVar5);
  if (uVar5 != 0) {
    for (uStack8 = 0x4; uStack8 < 0x25; uStack8 += 1) {
      uVar7 = pass1_1020_bae6(uVar4,paVar6,uVar2,CONCAT22(uStack8,param_4));
      uVar4 = uVar7 & 0xffff;
      uVar5 = (uVar7 >> 0x10) | uVar4;
      paVar6 = (paVar6 & 0xffff0000 | uVar5);
      if (uVar5 != 0) {
        pass1_1030_7ddc(uVar4,paVar6,param_3,uVar7,uStack8);
        uVar3 = pass1_1030_7bee(param_3);
        uVar4 = uVar3;
        if (uVar3 != 0) {
          return;
        }
        string_1020_c0ca(uStack8);
        vsprintf_op_1030_840a(paVar6,s_truck_0x_08lx_unloaded__ld_of__s_1050_5798);
        pass1_1020_bb8a(puVar1,0x0,uStack8 << 0x10);
      }
    }
    if (puVar1.is_null() == false) {
      fn_ptr_1020_ba7e(puVar1);
      fn_ptr_1000_17ce(puVar1);
    }
  }
  return;
}



pub fn pass1_1030_bbe6(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1030_b96c(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub fn pass1_1030_bc24(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32) -> *mut u16

{
  pass1_1028_b22c(param_1,CONCAT22(param_3,param_2),param_4,param_5);
  CONCAT22(param_3,param_2) = 0xbc96;
  (param_2 + 0x2) = 0x1030;
  return CONCAT22(param_3,param_2);
}
pub fn pass1_1030_bc4e(param_1: *mut u16)

{
  *param_1 = 0xbc96;
  (param_1 + 0x2) = 0x1030;
  pass1_1028_b260(param_1);
  return;
}
pub fn FUN_1030_bc6c()

{
  return;
}



pub fn pass1_1030_bc70(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1030_bc4e(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub fn pass1_1030_bcae(mut param_1: u16 ,mut param_2: u16 ) -> u32

{
  return CONCAT22(param_2,param_1);
}
pub fn pass1_1030_bcbc(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,mut param_4: u16 ,mut param_5: u32)

{
  pass1_1030_bcde(param_1,param_2,CONCAT22(param_3,param_2),
                  CONCAT22(param_4,param_3),(param_5 + 0x4));
  return;
}
pub fn pass1_1030_bcde(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,param_5: i32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut local_14: i16;
  let mut local_12: i16;
  let mut local_10: i16;
  let mut local_e: i16;
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut lStack6: i32;

  uVar2 = (param_3 >> 0x10);
  iVar1 = param_3;
  lStack6 = (iVar1 + 0x8);
  if (lStack6 != param_5) {
    return;
  }
  local_c = (iVar1 + 0xc);
  uStack8 = (iVar1 + 0x10);
  pass1_1008_3e94(param_4,CONCAT22(0x1050,&local_10),CONCAT22(0x1050,&local_e));
  pass1_1008_3e94(CONCAT22(0x1050,&local_c),CONCAT22(0x1050,&local_14),CONCAT22(0x1050,&local_12))
  ;
  pass1_1000_49b2(local_e - local_12);
  pass1_1000_49b2(local_10 - local_14);
  return;
}
pub fn pass1_1030_bd74(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut astruct_670)

{
  iVar1: *mut astruct_670;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut local_1e: i16;
  let mut local_1c: i16;
  let mut local_1a: i16;
  let mut local_18: i16;
  let mut local_16: u32;
  let mut uStack18: u16;
  let mut local_10: u32;
  let mut uStack12: u16;
  let mut lStack10: i32;
  let mut lStack6: i32;

  uVar3 = (param_4 >> 0x10);
  iVar1 = param_4;
  lStack6 = iVar1.field8_0x8;
  uVar4 = (param_3 >> 0x10);
  iVar2 = param_3;
  lStack10 = (iVar2 + 0x8);
  if (lStack10 != lStack6) {
    return;
  }
  local_10 = iVar1.field9_0xc;
  uStack12 = iVar1.field10_0x10;
  local_16 = (iVar2 + 0xc);
  uStack18 = (iVar2 + 0x10);
  pass1_1008_3e94(CONCAT22(0x1050,&local_10),CONCAT22(0x1050,&local_1a),CONCAT22(0x1050,&local_18)
                 );
  pass1_1008_3e94(CONCAT22(0x1050,&local_16),CONCAT22(0x1050,&local_1e),CONCAT22(0x1050,&local_1c)
                 );
  pass1_1000_49b2(local_18 - local_1c);
  pass1_1000_49b2(local_1a - local_1e);
  return;
}



astruct_180 * set_fn_ptr_1030_be34(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0xc006;
  (param_1 + 0x2) = 0x1030;
  return param_1;
}



pub fn pass1_1030_be56(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> *mut u16

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0xc006;
  (param_2 + 0x2) = 0x1030;
  return &param_2.field0_0x0;
}



// WARNING: Unable to use type for symbol uVar2
pub fn pass1_1030_be80(param_1: *mut u8,param_2: *mut astruct_15)

{
  let mut piVar1: *mut i16;
  let mut pSVar2: *mut StructD;
  let mut iVar3: i16;
  let mut BVar4: bool;
  let mut uVar5: u32;
  let mut extraout_DX: u16;
  let mut uVar6: u16;
  pstruct15_7: *mut astruct_15;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut uVar2: *mut StructD;

  pass1_1028_bf22(param_1,param_2);
  uVar7 = (param_2 >> 0x10);
  pstruct15_7 = param_2;
  if (pstruct15_7.field15_0x12 == 0x5) {
    pSVar2 = pstruct15_7.field16_0x14;
    (pSVar2 + 0xa4) = 0x1e;
    uVar2 = pstruct15_7.field16_0x14;
    (uVar2 + 0xac) = 0x1;
    iVar8 = pstruct15_7.field10_0xc;
    iVar3 = iVar8 + -0x1b;
    if (iVar3 == 0) {
      pSVar2 = pstruct15_7.field16_0x14;
      (pSVar2 + 0xaa) = 0xa;
    }
    else {
      iVar3 = iVar8 + -0x1c;
      if (iVar3 == 0) {
        pSVar2 = pstruct15_7.field16_0x14;
        (pSVar2 + 0xaa) = 0xb;
      }
      else {
        iVar3 = iVar8 + -0x1d;
        if (iVar3 == 0) {
          pSVar2 = pstruct15_7.field16_0x14;
          (pSVar2 + 0xaa) = 0xc;
        }
      }
    }
    pass1_1028_b58e(param_2);
    uVar5 = (iVar3 + 0x2e);
    iVar8 = 0xc;
    uVar6 = extraout_DX;
    pass1_1038_3fb0(uVar5);
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | uVar6 << 0x10,iVar8);
    if (BVar4 != 0) {
      pSVar2 = pstruct15_7.field16_0x14;
      piVar1 = (pSVar2 + 0xaa);
      *piVar1 = *piVar1 + 1;
    }
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | uVar6 << 0x10,0xe);
    if (BVar4 != 0) {
      pSVar2 = pstruct15_7.field16_0x14;
      piVar1 = (pSVar2 + 0xaa);
      *piVar1 = *piVar1 + 1;
    }
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | uVar6 << 0x10,0x76);
    if (BVar4 != 0) {
      pSVar2 = pstruct15_7.field16_0x14;
      piVar1 = (pSVar2 + 0xaa);
      *piVar1 = *piVar1 + 1;
    }
  }
  return;
}
pub fn pass1_1030_bf6e(mut param_1: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut uVar8: u16;
  let mut uStack6: u32;

  uVar8 = 0x1e;
  uVar3 = pass1_1028_b58e(param_1);
  uVar4 = in_EDX;
  uStack6 = CONCAT22(uVar4,uVar3);
  uVar7 = pass1_1030_7c28(uVar3,uVar4,CONCAT22(uVar4,uVar3),uVar8);
  uVar4 = 0x3e8 - uVar7;
  uVar2 = (param_1 + 0x14);
  uVar6 = (uVar2 >> 0x10);
  iVar5 = uVar2;
  puVar1 = (iVar5 + 0xaa);
  uVar3 = -(uVar4 < *puVar1);
  pass1_1030_7ddc(uVar3,(in_EDX & 0xffff0000 | uVar7 >> 0x10),uStack6,
                  ((uVar4 - *puVar1 & uVar3) + (iVar5 + 0xaa)),0x1e);
  return;
}



pub unsafe fn pass1_1030_bfb8(mut param_1: u32) -> u16

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u16;

  uVar3 = 0x1e;
  uVar1 = pass1_1028_b58e(param_1);
  uVar2 = pass1_1030_7c28(uVar1,(uVar1 >> 0x10),uVar1,uVar3);
  return 0x3e8 - uVar2;
}



pub fn pass1_1030_bfe0(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1030_c06e(param_1: *mut astruct_180)

{
  iVar1: *mut astruct_180;
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 1) = 0;
  iVar1[0x1].field_0x4 = 0;
  param_1.field0_0x0 = 0xc68e;
  iVar1.field1_0x2 = 0x1030;
  return;
}
