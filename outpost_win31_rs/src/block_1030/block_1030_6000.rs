

pub unsafe fn pass1_1030_6118(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1030_5d78(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_615a(param_1: *mut StructD,param_2: *mut astruct_137)

{
  let mut extraout_DX: u16;
  let mut uVar1: u16;
  let mut iVar2: *mut astruct_137;
  let mut uVar2: u16;

  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  uVar1 = 0;
  param_2 = 0;
  iVar2.field4_0x4 = 0;
  mem_op_1000_179c(0xc,param_1);
  extraout_DX = param_1 | uVar1;
  if (extraout_DX == 0) {
    iVar2.field4_0x4 = 0;
  }
  else {
    set_struct_1008_574a(CONCAT22(param_1,uVar1));
    iVar2.field4_0x4 = uVar1;
    iVar2.field5_0x6 = extraout_DX;
  }
  _PTR_LOOP_1050_5740 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_61b0(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u32;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

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
  _PTR_LOOP_1050_5740 = 0;
  return;
}
pub unsafe fn pass1_1030_61fe(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,mut param_5: u32,param_6: i32)

{
  pass1_1030_677a(param_3,param_6);
  pass1_1030_8aa0(CONCAT22(param_2,param_1),param_4,param_5,param_2);
  return;
}



pub unsafe fn pass1_1030_6222(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32,mut param_4: i16,mut param_5: u32,mut param_6: u32) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut uStack6: u32;

  paVar3 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x4c,paVar3);
  uVar2 = paVar3 | param_1;
  if (uVar2 == 0) {
    param_1 = 0;
    uVar2 = 0;
  }
  else {
    pass1_1030_88ce(CONCAT22(paVar3,param_1),param_5,param_6);
  }
  uStack6 = CONCAT22(uVar2,param_1);
  ppcVar1 = ((param_3 + 0x4) + 0x4);
  (**ppcVar1)();
  if (param_4 != 0) {
    pass1_1030_8d08(uStack6,uVar2);
  }
  return 0x0;
}
pub unsafe fn pass1_1030_627e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,param_5: i32)

{
  let mut local_12: [u32;0x2] = [0;0x2];
  let mut uStack10: u32;
  let mut uStack6: u32;

  uStack6 = 0;
  pass1_1030_677a(param_3,param_5);
  uStack10 = CONCAT22(param_2,param_1);
  if ((param_2 | param_1) != 0) {
    pass1_1030_8b00(uStack10,param_4,CONCAT22(0x1050,local_12));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub unsafe fn pass1_1030_62e4(param_1: u32,param_2: *mut u16,param_3: i32)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut paVar5: *mut Struct57;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut puVar8: *mut u16;
  let mut local_64: [i16;0x3] = [0;0x3];
  let mut uStack94: u32;
  let mut uStack88: u16;
  let mut uStack78: u16;
  let mut uStack76: u16;
  let mut local_40: u32;
  let mut uStack60: u32;
  let mut uStack56: u16;
  let mut puStack54: *mut u32;
  let mut uStack52: u16;
  let mut puStack50: *mut u32;
  let mut uStack48: u16;
  let mut uStack46: u16;
  let mut iStack44: i16;
  let mut local_2a: [u8;0x2] = [0;0x2];
  let mut local_28: i16;
  let mut local_26: i16;
  let mut local_24: u16;
  let mut local_22: [u8;0x2] = [0;0x2];
  let mut local_20: [u8;0x2] = [0;0x2];
  let mut local_1e: u16;
  let mut local_1c: u16;
  let mut local_1a: u16;
  let mut local_18: [u8;0x6] = [0;0x6];
  let mut local_12: [u8;0x6] = [0;0x6];
  let mut local_c: [u8;0x6] = [0;0x6];
  let mut uStack6: u32;

  uVar7 = (param_1 >> 0x10);
  puVar2 = param_1;
  uStack52 = (param_1 + 2);
  paVar5 = (in_EDX & 0xffff0000 | uStack52);
  puStack54 = puVar2;
  puStack50 = puVar2;
  uStack48 = uStack52;
  if ((uStack52 | puVar2) != 0) {
    ppcVar1 = *puVar2;
    (**ppcVar1)();
  }
  mem_op_1000_179c(0x18,paVar5);
  uStack52 = paVar5;
  uVar3 = uStack52 | puVar2;
  puStack54 = puVar2;
  if (uVar3 == 0) {
    puVar2 = null_mut();
    uVar3 = 0;
  }
  else {
    struct_op_1030_1cd8(CONCAT13((paVar5 >> 0x8),CONCAT12(paVar5,puVar2)),0x5,0x5);
  }
  param_1 = puVar2;
  (param_1 + 0x2) = uVar3;
  pass1_1030_677a(param_1,param_3);
  uStack6 = CONCAT22(uVar3,puVar2);
  if ((uVar3 | puVar2) != 0) {
    pass1_1008_3e38(CONCAT22(0x1050,local_c));
    pass1_1008_3e38(CONCAT22(0x1050,local_12));
    puVar8 = pass1_1008_3e38(CONCAT22(0x1050,local_18));
    uVar6 = puVar8 >> 0x10;
    pass1_1008_6d3e(param_2,CONCAT22(0x1050,local_12),CONCAT22(0x1050,local_c));
    pass1_1008_3eb4(CONCAT22(0x1050,local_c),CONCAT13(0x10,CONCAT12(0x50,&local_1e)),
                    CONCAT22(0x1050,&local_1c),CONCAT22(0x1050,&local_1a));
    pass1_1008_3eb4(CONCAT22(0x1050,local_12),CONCAT13(0x10,CONCAT12(0x50,&local_24)),
                    CONCAT22(0x1050,local_22),CONCAT22(0x1050,local_20));
    pass1_1008_6d64(param_2,CONCAT22(0x1050,local_18));
    pass1_1008_3eb4(CONCAT22(0x1050,local_18),CONCAT13(0x10,CONCAT12(0x50,local_2a)),
                    CONCAT22(0x1050,&local_28),CONCAT22(0x1050,&local_26));
    if (local_24 == local_1e) {
      iStack44 = 0;
      uStack46 = local_1c;
      loop {
        uVar4 = uVar6;
        uVar3 = local_28 + local_1c;
        if (uVar3 <= uStack46) { break; }
        for (uStack56 = local_1a; uStack56 < (local_26 + local_1a); uStack56 += 1) {
          uStack88 = local_1e;
          pass1_1008_3e54(CONCAT13(0x10,CONCAT12(0x50,local_64)),local_1e,uStack46,uStack56);
          pass1_1030_8b00(uStack6,CONCAT22(0x1050,local_64),CONCAT22(0x1050,&local_40));
          uStack60 = local_40;
          local_64[0] = iStack44;
          uStack60 = local_40;
          uStack78 = uStack60;
          uStack76 = local_40;
          uStack76._1_1_ = (local_40 >> 0x18);
          if (uStack76._1_1_ == '\0') {
            uStack60 = 0;
            local_40 = 0;
          }
          uVar6 = local_40;
          uStack94 = CONCAT22(local_40,uStack60);
          ppcVar1 = (*param_1 + 0x8);
          iStack44 = iStack44 + 1;
          (**ppcVar1)();
        }
        uStack46 += 0x1;
      }
      ppcVar1 = (*param_1 + 0x10);
      (**ppcVar1)(0x1008,*param_1);
      if ((uVar4 | uVar3) != 0) {
        return;
      }
    }
  }
  return;
}
pub unsafe fn pass1_1030_64ce(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,param_5: i32,param_6: *mut u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut local_e: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uStack6 = 0;
  pass1_1030_677a(param_3,param_5);
  uStack10 = CONCAT22(param_2,param_1);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0) {
    puVar1 = &local_e;
    pass1_1030_8b00(uStack10,param_4,CONCAT22(0x1050,puVar1));
    uStack6 = *puVar1;
  }
  *param_6 = uStack6;
  return;
}
pub unsafe fn pass1_1030_6522(param_1: u32,mut param_2: u32,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut paVar5: *mut Struct57;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut puVar8: *mut u16;
  let mut local_64: [u8;0xc] = [0;0xc];
  let mut uStack88: u16;
  let mut local_40: u32;
  let mut uStack60: u32;
  let mut uStack56: u16;
  let mut puStack54: *mut u32;
  let mut uStack52: u16;
  let mut puStack50: *mut u32;
  let mut uStack48: u16;
  let mut uStack46: u16;
  let mut iStack44: i16;
  let mut local_2a: [u8;0x2] = [0;0x2];
  let mut local_28: i16;
  let mut local_26: i16;
  let mut local_24: u16;
  let mut local_22: [u8;0x2] = [0;0x2];
  let mut local_20: [u8;0x2] = [0;0x2];
  let mut local_1e: u16;
  let mut local_1c: u16;
  let mut local_1a: u16;
  let mut local_18: [u8;0x6] = [0;0x6];
  let mut local_12: [u8;0x6] = [0;0x6];
  let mut local_c: [u8;0x6] = [0;0x6];
  let mut uStack6: u32;

  uVar7 = (param_1 >> 0x10);
  puVar2 = param_1;
  uStack52 = (param_1 + 2);
  paVar5 = (in_EDX & 0xffff0000 | uStack52);
  puStack54 = puVar2;
  puStack50 = puVar2;
  uStack48 = uStack52;
  if ((uStack52 | puVar2) != 0) {
    ppcVar1 = *puVar2;
    (**ppcVar1)();
  }
  mem_op_1000_179c(0x18,paVar5);
  uStack52 = paVar5;
  uVar3 = uStack52 | puVar2;
  puStack54 = puVar2;
  if (uVar3 == 0) {
    puVar2 = null_mut();
    uVar3 = 0;
  }
  else {
    struct_op_1030_1cd8(CONCAT13((paVar5 >> 0x8),CONCAT12(paVar5,puVar2)),0x5,0x5);
  }
  param_1 = puVar2;
  (param_1 + 0x2) = uVar3;
  pass1_1030_677a(param_1,param_3);
  uStack6 = CONCAT22(uVar3,puVar2);
  if ((uVar3 | puVar2) != 0) {
    pass1_1008_3e38(CONCAT22(0x1050,local_c));
    pass1_1008_3e38(CONCAT22(0x1050,local_12));
    puVar8 = pass1_1008_3e38(CONCAT22(0x1050,local_18));
    uVar6 = puVar8 >> 0x10;
    pass1_1008_6d3e(param_2,CONCAT22(0x1050,local_12),CONCAT22(0x1050,local_c));
    pass1_1008_3eb4(CONCAT22(0x1050,local_c),CONCAT13(0x10,CONCAT12(0x50,&local_1e)),
                    CONCAT22(0x1050,&local_1c),CONCAT22(0x1050,&local_1a));
    pass1_1008_3eb4(CONCAT22(0x1050,local_12),CONCAT13(0x10,CONCAT12(0x50,&local_24)),
                    CONCAT22(0x1050,local_22),CONCAT22(0x1050,local_20));
    pass1_1008_6d64(param_2,CONCAT22(0x1050,local_18));
    pass1_1008_3eb4(CONCAT22(0x1050,local_18),CONCAT13(0x10,CONCAT12(0x50,local_2a)),
                    CONCAT22(0x1050,&local_28),CONCAT22(0x1050,&local_26));
    if (local_24 == local_1e) {
      iStack44 = 0;
      uStack46 = local_1c;
      loop {
        uVar4 = uVar6;
        uVar3 = local_28 + local_1c;
        if (uVar3 <= uStack46) { break; }
        for (uStack56 = local_1a; uStack56 < (local_26 + local_1a); uStack56 += 1) {
          uStack88 = local_1e;
          pass1_1008_3e54(CONCAT13(0x10,CONCAT12(0x50,local_64)),local_1e,uStack46,uStack56);
          pass1_1030_8b00(uStack6,CONCAT22(0x1050,local_64),CONCAT22(0x1050,&local_40));
          uStack60 = local_40;
          uVar6 = (iStack44 >> 0xf);
          ppcVar1 = (*param_1 + 0x8);
          iStack44 = iStack44 + 1;
          (**ppcVar1)();
        }
        uStack46 += 0x1;
      }
      ppcVar1 = (*param_1 + 0x10);
      (**ppcVar1)(0x1008,*param_1);
      if ((uVar4 | uVar3) != 0) {
        return;
      }
    }
  }
  return;
}
pub unsafe fn pass1_1030_66de(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut local_a: [u8;0x8] = [0;0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x4));
  loop {
    uVar1 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (uVar1 == 0) { break; }
    pass1_1030_8bac(uVar1,param_2);
  }
  return;
}
pub unsafe fn pass1_1030_671c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,param_5: *mut u16,param_6: i32)

{
  pass1_1030_677a(param_3,param_6);
  pass1_1030_8bdc(CONCAT22(param_2,param_1),param_4,param_5);
  return;
}
pub unsafe fn pass1_1030_6740(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut local_a: [u8;0x8] = [0;0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x4));
  loop {
    uVar1 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (uVar1 == 0) { break; }
    pass1_1030_8c38(uVar1);
  }
  return;
}
pub unsafe fn pass1_1030_677a(mut param_1: u32,param_2: i32)

{
  let mut puVar1: *mut u8;
  let mut extraout_DX: u16;
  let mut uVar2: u16;
  let mut local_a: [u8;0x8] = [0;0x8];

  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0x4) == 0) {
    return;
  }
  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x4));
  loop {
    puVar1 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,puVar1));
    if ((extraout_DX | puVar1) == 0) {
      return;
    }
  } while ((puVar1 + 0x24) != param_2);
  return;
}
pub unsafe fn pass1_1030_67cc(param_1: *mut astruct_180)

{
  let mut iVar1: *mut astruct_180;
  let mut uVar1: u16;

  struct_1030_1628(param_1);
  iVar1 = param_1;
  iVar1 = &iVar1.field10_0xc;
  pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  iVar1.field13_0x12 = 0;
  iVar1.field14_0x14 = 0;
  iVar1.field15_0x16 = 0;
  iVar1.field17_0x1a = 0;
  (&iVar1.field18_0x1c + 0x2) = 0;
  iVar1[0x1].field1_0x2 = 0;
  iVar1[0x1].field_0x6 = 0;
  iVar1[0x1].field_0xa = 0;
  iVar1[0x1].field11_0xe = 0;
  iVar1[0x1].field13_0x12 = 0;
  iVar1[0x1].field14_0x14 = 0;
  iVar1[0x1].field16_0x18 = 0;
  iVar1[0x1].field15_0x16 = 0;
  iVar1[0x1].field18_0x1c = 0;
  iVar1[0x1].field17_0x1a = 0;
  (iVar1 + 0x2).field0_0x0 = 0;
  (&iVar1[0x1].field18_0x1c + 0x2) = 0;
  param_1.field0_0x0 = 0x8114;
  iVar1.field1_0x2 = 0x1030;
  return;
}
pub unsafe fn pass1_1030_684c(param_1: *mut u16,param_2: *mut u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u32)

{
  let mut in_EDX: *mut Struct57;
  let mut iVar1: i16;
  let mut uVar2: u16;

  pass1_1030_165e(in_EDX,param_1,0x5000000,param_6);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0xc) = *param_2;
  (iVar1 + 0x10) = (param_2 + 1);
  (iVar1 + 0x12) = param_4;
  (iVar1 + 0x14) = param_4;
  (iVar1 + 0x16) = 0;
  (iVar1 + 0x1a) = 0;
  (iVar1 + 0x1e) = 0;
  (iVar1 + 0x22) = 0;
  (iVar1 + 0x26) = 0;
  (iVar1 + 0x2a) = 0;
  (iVar1 + 0x2e) = 0;
  (iVar1 + 0x32) = 0;
  (iVar1 + 0x34) = 0;
  (iVar1 + 0x36) = 0;
  (iVar1 + 0x3a) = 0;
  (iVar1 + 0x3e) = 0;
  *param_1 = 0x8114;
  (iVar1 + 0x2) = 0x1030;
  return;
}
pub unsafe fn pass1_1030_68dc(param_1: *mut astruct_611)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut pcVar4: *mut c_char;
  let mut ppcVar5: *mut *mut code;
  let mut iVar6: *mut astruct_611;
  let mut uVar6: u16;
  let mut unaff_CS: u16;
  let mut pcStack10: *mut c_char;

  uVar6 = (param_1 >> 0x10);
  iVar6 = param_1;
  param_1 = 0x8114;
  iVar6.field2_0x2 = 0x1030;
  pcVar4 = *&iVar6.field_0x22;
  uVar1 = iVar6.field33_0x24;
  if ((uVar1 | pcVar4) != 0) {
    fn_ptr_1020_ba7e((pcVar4 & 0xffff | uVar1 << 0x10));
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcVar4);
  }
  uVar1 = iVar6.field34_0x26;
  uVar2 = iVar6.field35_0x28;
  pcStack10 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0) {
    fn_ptr_1020_ba7e(CONCAT22(uVar2,uVar1));
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcStack10);
  }
  puVar3 = iVar6.field29_0x1e;
  uVar1 = iVar6.field30_0x20;
  if ((uVar1 | puVar3) != 0) {
    ppcVar5 = *puVar3;
    (**ppcVar5)(unaff_CS,puVar3,uVar1,1);
  }
  puVar3 = iVar6.field48_0x36;
  uVar1 = iVar6.field49_0x38;
  if ((uVar1 | puVar3) != 0) {
    ppcVar5 = *puVar3;
    (**ppcVar5)(unaff_CS,puVar3,uVar1,1);
  }
  puVar3 = iVar6.field50_0x3a;
  uVar1 = iVar6.field51_0x3c;
  if ((uVar1 | puVar3) != 0) {
    ppcVar5 = *puVar3;
    (**ppcVar5)(unaff_CS,puVar3,uVar1,1);
  }
  puVar3 = iVar6.field52_0x3e;
  uVar1 = iVar6.field53_0x40;
  if ((uVar1 | puVar3) != 0) {
    ppcVar5 = *puVar3;
    (**ppcVar5)(unaff_CS,puVar3,uVar1,1);
  }
  pass1_1030_16b2(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_69cc(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u32;

  uVar4 = (param_3 >> 0x10);
  iVar3 = param_3;
  if ((iVar3 + 0x3e) != 0) {
    return;
  }
  if (((iVar3 + 0x22) != 0) && (pass1_1020_ba94(*(i32 **)(iVar3 + 0x22)), (param_2 | param_1) != 0)) {
    return;
  }
  uVar1 = pass1_1030_6fa0(param_3);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x4);
  if ((BVar2 != 0) &&
     (uVar5 = pass1_1028_67d4((iVar3 + 0x1a)), ((uVar5 >> 0x10) | uVar5) != 0)) {
    return;
  }
  return;
}
pub unsafe fn pass1_1030_6a2c(mut param_1: u16 ,param_2: *mut StructD,param_3: *mut astruct_382,param_4: *mut astruct_383)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar3: u16;
  let mut iVar2: *mut astruct_384;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut iVar4: *mut astruct_382;
  let mut iVar5: *mut astruct_383;
  let mut uVar6: *mut astruct_382;
  let mut uVar2: *mut astruct_383;
  let mut local_a: [u8;0x8] = [0;0x8];

  paVar6 = CONCAT22(in_register_0000000a,param_2);
  uVar6 = (param_3 >> 0x10);
  iVar4 = param_3;
  if (iVar4.field62_0x3e.is_null()) {
    mem_op_1000_179c(0xc,paVar6);
    uVar4 = paVar6;
    uVar5 = uVar4 | param_1;
    paVar6 = uVar5;
    if (uVar5 == 0) {
      iVar4.field62_0x3e = null_mut();
    }
    else {
      uVar3 = set_struct_1008_574a(CONCAT22(uVar4,param_1));
      iVar4.field62_0x3e = uVar3;
      (&iVar4.field62_0x3e + 0x2) = paVar6;
    }
  }
  pass1_1008_5784(CONCAT22(0x1050,local_a),iVar4.field62_0x3e);
  loop {
    loop {
      uVar4 = paVar6;
      iVar2 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
      paVar6 = (uVar4 | iVar2);
//      if ((uVar4 | iVar2) == 0) goto LAB_1030_6af4;
      uVar2 = (param_4 >> 0x10);
      iVar5 = param_4;
    } while ((iVar2.field5_0x6 != iVar5.field5_0x6) || (iVar2.field4_0x4 != iVar5.field4_0x4));
  } while (iVar2.field6_0x8 != iVar5.field6_0x8);
  iVar2.field7_0xa = iVar2.field7_0xa + iVar5.field7_0xa;
  iVar2.field8_0xc = iVar2.field8_0xc + iVar5.field8_0xc;
  param_4 = null_mut();//
// LAB_1030_6af4:
  if (param_4.is_null() == false) {
    ppcVar1 = (*iVar4.field62_0x3e + 0x8);
    (**ppcVar1)(0x1008,iVar4.field62_0x3e,param_4);
  }
  return;
}



pub unsafe fn pass1_1030_6b16(param_1: *mut astruct_412) -> u32

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut uVar4: u32;
  let mut iVar5: *mut astruct_412;
  let mut uVar5: u16;
  let mut uVar6: u32;

  uVar5 = (param_1 >> 0x10);
  iVar5 = param_1;
  if (&iVar5.field_0x3a == 0) {
    return 0x0;
  }
  ppcVar3 = (*&iVar5.field_0x3a + 0x10);
  uVar6 = (**ppcVar3)();
  uVar4 = &iVar5.field_0x3a;
  if ((uVar4 + 0x8) == 0) {
    puVar1 = &iVar5.field_0x3a;
    uVar2 = iVar5.field60_0x3c;
    if ((uVar2 | puVar1) != 0) {
      ppcVar3 = *puVar1;
      (**ppcVar3)();
    }
    iVar5.field_0x3a = 0;
  }
  return uVar6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_6b86(mut param_1: u16 ,mut param_2: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut extraout_DX: u16;
  let mut uVar3: u16;
  let mut extraout_DX_00: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut unaff_CS: u16;
  let mut uStack12: u32;
  let mut uStack8: u32;

  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  if ((iVar4 + 0x1e) == 0) {
    param_1 = 0;
    uVar3 = 0;
  }
  else {
    ppcVar1 = ((iVar4 + 0x1e) + 0x10);
    (**ppcVar1)();
    uVar3 = extraout_DX;
  }
  uStack8 = CONCAT22(uVar3,param_1);
  for uStack12 in 0 .. uStack8 {
    ppcVar1 = ((iVar4 + 0x1e) + 0x4);
    uVar2 = uStack8;
    (**ppcVar1)(unaff_CS,(iVar4 + 0x1e));
    if ((extraout_DX_00 | uVar2) != 0) {
      unaff_CS = 0x1028;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2 & 0xffff | extraout_DX_00 << 0x10);
    }
  }
  return;
}
pub unsafe fn pass1_1030_6c1a(mut param_1: u32,mut param_2: i16)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  iVar2 = (iVar3 + 0x32);
  (iVar3 + 0x32) = param_2;
  piVar1 = (iVar3 + 0x34);
  *piVar1 = *piVar1 + (param_2 - iVar2);
  iVar2 = (iVar3 + 0x32);
  if (iVar2 < 0x0) {
    iVar2 = 0;
  }
  (iVar3 + 0x32) = iVar2;
  return;
}
pub unsafe fn pass1_1030_6c4c(mut param_1: u32,mut param_2: i16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (param_1 + 0x32);
  if (param_2 < iVar1) {
    iVar1 = param_2;
  }
  (param_1 + 0x34) = iVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_6c66(mut param_1: u16 ,param_2: *mut u8,param_3: *mut astruct_386,mut param_4: i16,param_5: *mut astruct_385)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut iVar7: *mut astruct_386;
  let mut iVar6: *mut astruct_385;
  let mut uVar7: *mut astruct_386;
  let mut uVar8: *mut astruct_385;
  let mut uVar9: u16;

  paVar6 = CONCAT22(in_register_0000000a,param_2);
  uVar9 = 0x1030;
  uVar7 = (param_3 >> 0x10);
  iVar7 = param_3;
  if (iVar7.field55_0x3a.is_null()) {
    uVar9 = 0x1000;
    mem_op_1000_179c(0xc,paVar6);
    uVar5 = paVar6;
    uVar4 = uVar5 | param_1;
    paVar6 = (paVar6 & 0xffff0000 | uVar4);
    if (uVar4 == 0) {
      iVar7.field55_0x3a = null_mut();
    }
    else {
      uVar9 = 0x1008;
      set_struct_1008_574a(CONCAT22(uVar5,param_1));
      iVar7.field55_0x3a = param_1;
      (&iVar7.field55_0x3a + 0x2) = paVar6;
    }
  }
  ppcVar1 = (*iVar7.field55_0x3a + 0x8);
  (**ppcVar1)(uVar9,iVar7.field55_0x3a,param_5);
  if (param_4 != 0) {
    uVar8 = (param_5 >> 0x10);
    iVar6 = param_5;
    if (iVar6.field5_0x6 != 0) {
      pass1_1030_6e9c(param_3,iVar6.field7_0xa,iVar6.field5_0x6);
      return;
    }
    if (iVar6.field4_0x4 != 0) {
      uVar5 = iVar6.field7_0xa;
      uVar4 = -uVar5;
      uVar5 = -(uVar5 != 0);
      pass1_1030_7ddc(uVar4,(paVar6 & 0xffff0000 | uVar5),param_3,
                      CONCAT13((uVar5 >> 0x8),CONCAT12(uVar5,uVar4)),iVar6.field4_0x4);
      return;
    }
    if (iVar6.field6_0x8 != 0) {
      uVar2 = pass1_1030_6fa0(param_3);
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x4);
      if (BVar3 != 0) {
        pass1_1028_6356(iVar7.field26_0x1a,0x0,iVar6.field7_0xa,0x0);
      }
    }
  }
  return;
}



pub unsafe fn pass1_1030_6d4e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32) -> u32

{
  let mut uVar1: u16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  uStack6 = 0;
  uStack4 = 0;
  uVar1 = (param_3 >> 0x10);
  if ((param_3 + 0x36) != 0) {
    pass1_1010_9092(param_1,(param_3 + 0x36));
    uStack6 = param_1;
    uStack4 = param_2;
  }
  return CONCAT22(uStack4,uStack6);
}
pub unsafe fn pass1_1030_6d80(param_1: *mut astruct_299,mut param_2: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: *mut astruct_299;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = &iVar4.field54_0x36;
  uVar2 = (&iVar4.field54_0x36 + 2);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  iVar4.field54_0x36 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1030_6db4(param_1: *mut u8) -> u16

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000fff0: u16;

  puVar3 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           CONCAT22(in_stack_0000fff0,0x2f),in_stack_0000fe98,in_stack_0000ffbc,
                           in_stack_0000ffc2,in_stack_0000ffc6);
  uVar2 = (puVar3 >> 0x10);
  iVar1 = puVar3;
  pass1_1010_ed3e(puVar3);
  return (iVar1 + 0x18);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_6ddc(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut BVar2: bool;

  uVar1 = pass1_1030_6fa0(param_1);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1e);
  if (BVar2 != 0) {
    pass1_1030_d0c6((param_1 + 0x1a));
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_6e14(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut BVar3: bool;

  uVar2 = pass1_1030_6fa0(param_1);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x1e);
  if (BVar3 != 0) {
    uVar1 = (param_1 + 0x1a);
    pass1_1030_d102(uVar1,(uVar1 >> 0x10));
    return;
  }
  return;
}
pub unsafe fn pass1_1030_6e4c(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0x1a) == 0) {
    struct_op_1030_73a8((param_1 & 0xffff | uVar3 << 0x10),in_AX,in_DX);
  }
  if (((iVar2 + 0x1a) != 0) && (uVar1 = (iVar2 + 0x1a), (uVar1 + 0x12) == 0x4)) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_6e9c(param_1: *mut astruct_301,param_2: i32,mut param_3: i16)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut iVar6: *mut astruct_301;
  let mut uVar6: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar6 = (param_1 >> 0x10);
  iVar6 = param_1;
  uVar2 = (&iVar6.field30_0x1e + 0x2) | &iVar6.field30_0x1e;
  if (uVar2 != 0) {
    ppcVar1 = (*iVar6.field30_0x1e + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX,uVar2);
    for uStack10 in 0 .. uStack6 {
      ppcVar1 = (*iVar6.field30_0x1e + 0x4);
      uVar4 = uStack6;
      (**ppcVar1)();
      uVar2 = uVar4;
      uVar5 = extraout_DX_00 | uVar2;
      if (uVar5 != 0) {
        uVar3 = uVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | extraout_DX_00 << 0x10);
        if ((uVar3 + 0xc) == param_3) {
          param_2 += -0x1;
          pass1_1028_e332(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00);
          ppcVar1 = (*iVar6.field30_0x1e + 0x8);
          (**ppcVar1)(0x1028,iVar6.field30_0x1e,0x0,uStack10);
        }
        if ((param_2 | param_2) == 0) {
          return;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_6f5a(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut BVar2: bool;

  uVar1 = pass1_1030_6fa0(param_1);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x4);
  if (BVar2 != 0) {
    pass1_1028_6302((param_1 + 0x1a));
  }
  return;
}



pub unsafe fn pass1_1030_6fa0(mut param_1: u32) -> u16

{
  let mut uVar1: u32;
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0x1a) == 0) {
    struct_op_1030_73a8((param_1 & 0xffff | uVar3 << 0x10),in_AX,in_DX);
  }
  if ((iVar2 + 0x1a) != 0) {
    uVar1 = (iVar2 + 0x1a);
    return (uVar1 + 0xc);
  }
  return 0x0;
}
pub unsafe fn pass1_1030_6fd4(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0x1a) == 0) {
    struct_op_1030_73a8((param_1 & 0xffff | uVar2 << 0x10),in_AX,in_DX);
  }
  uVar1 = (param_1 + 0x1a);
  if ((uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}
