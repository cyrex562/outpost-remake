
pub unsafe fn struct_1020_d06c(param_1: *mut u16) -> *mut u16

{
  struct_1028_b354(param_1);
  *param_1 = 0xd314;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}



pub unsafe fn pass1_1020_d08e(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> *mut u16

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0xd314;
  (param_2 + 0x2) = 0x1020;
  return &param_2.field0_0x0;
}
pub unsafe fn pass1_1020_d0b8(param_1: *mut astruct_15)

{
  let mut uVar1: u32;
  let mut iVar2: i16;

  if ((param_1 + 0x12) != 0x6) {
    return;
  }
  uVar1 = pass1_1028_b4f2(param_1);
  iVar2 = uVar1;
  if ((iVar2 + 0x200) != 0x8000002) {
    pass1_1028_cb04(param_1);
    if ((iVar2 == 0) || (pass1_1020_d194(param_1), iVar2 == 0)) {
      iVar2 = 0x6;
  // TODO: goto LAB_1020_d10b;
    }
    pass1_1028_c952(param_1);
  }
  iVar2 = 0x5;//
// LAB_1020_d10b:
  pass1_1028_bdac(param_1,iVar2);
  return;
}



pub unsafe fn pass1_1020_d118(mut param_1: i16,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,mut param_5: u32,mut param_6: u32) -> u16

{
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut uVar3: u16;

  uVar2 = param_3;
  uVar3 = (param_3 >> 0x10);
  pass1_1028_c7b6(param_2,uVar2,uVar3,param_4,param_6);
  if (param_1 == 0x5) {
    pass1_1028_c23e(0x5,param_2,uVar2,uVar3,param_4,param_5,param_6);
    if (param_1 != 0) {
      pass1_1028_c3aa(uVar2,uVar3,param_4,param_5,param_6);
      if (param_1 != 0) {
        BVar1 = pass1_1028_c314(param_1,param_2,uVar2,uVar3,param_4,param_5,(param_5 >> 0x10),param_6);
        if (BVar1 != 0) {
          return 0x1;
        }
      }
    }
  }
  else {
    PTR_LOOP_1050_50ca = 0x6a8;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_d194(param_1: *mut astruct_15)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u8;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut in_EDX: u32;
  let mut paVar13: *mut Struct57;
  let mut uVar14: u16;
  let mut uVar15: u32;
  let mut puVar16: *mut u32;
  let mut puVar17: *mut u32;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  let mut uVar18: u8;
  let mut uVar19: u8;
  let mut uVar20: u16;
  let mut uVar21: u16;
  let mut uStack42: u32;
  let mut uStack38: u32;
  let mut puStack34: *mut u32;
  let mut local_4: [u8;0x2] = [0;0x2];
  let mut paVar12: *mut Struct57;

  uVar9 = (in_EDX >> 0x10);
  pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar15 = pass1_1028_b4f2(param_1);
  uVar8 = (uVar15 >> 0x10);
  paVar12 = CONCAT22(uVar9,uVar8);
  uVar7 = (uVar15 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7);
  uVar8 = uVar7;
  paVar13 = paVar12;
  pass1_1028_b58e(param_1);
  uVar9 = SUB42(paVar13,0x0);
  puVar2 = local_4;
  pass1_1030_bd74(puVar2,&DAT_1050_1050,uVar7 & 0xffff | paVar12 << 0x10,
                  CONCAT22(uVar9,uVar8));
  if (puVar2 < 0x0) {
    return;
  }
  if (0x1e < puVar2) {
    uVar3 = 0x87;
    puVar16 = mixed_1010_20ba(paVar13,_u16_1050_0ed0,0x870009,in_stack_0000fe70,in_stack_0000ff94,
                              in_stack_0000ff9a,in_stack_0000ff9e);
    uVar10 = (paVar13 >> 0x10);
    uVar3 = pass1_1010_65d0(puVar16,uVar3);
    if (uVar3 == 0) {
      puVar17 = pass1_1008_c6fa(_u16_1050_06e0,0x15);
      paVar12 = CONCAT22(uVar10,(puVar17 >> 0x10));
      uVar4 = puVar17;
      uVar14 = SUB42(&u16_1050_1038,0x0);
      pass1_1038_4e78(uVar4,paVar12,uVar15,puVar17);
      uVar10 = SUB42(paVar12,0x0);
      puStack34 = CONCAT22(uVar10,uVar4);
      ppcVar1 = (*puStack34 + 0x10);
      paVar13 = paVar12;
      uVar5 = uVar4;
      uVar21 = uVar4;
      (**ppcVar1)(&u16_1050_1038,uVar4,uVar10);
      uStack38 = CONCAT22(paVar13,uVar5);
      uStack42 = 0;
      loop {
        if (uStack38 <= uStack42) {
          if (puStack34.is_null()) {
            return;
          }
          ppcVar1 = *puStack34;
          (**ppcVar1)(uVar14,uVar4,paVar12,0x1,uVar21,uVar10,puStack34,puStack34);
          return;
        }
        uVar18 = uVar8;
        uVar19 = (uVar8 >> 0x8);
        uVar15 = uStack38;
        uVar20 = uVar9;
        pass1_1030_1d58(puStack34);
        uVar6 = uVar15;
        uVar11 = SUB42(paVar13,0x0);
        puVar2 = local_4;
        uVar14 = 0x1030;
        pass1_1030_bd74(puVar2,&DAT_1050_1050,uVar15 & 0xffff | paVar13 << 0x10,
                        CONCAT22(uVar20,CONCAT11(uVar19,uVar18)));
        if ((0x0 < puVar2) && (puVar2 < 0x1f)) { break; }
        uStack42 += 0x1;
      }
      if (puStack34.is_null()) {
        return;
      }
      ppcVar1 = *puStack34;
      (**ppcVar1)(0x1030,uVar4,paVar12,0x1,uVar21,uVar10,puStack34,puStack34,uVar6,uVar11);
      return;
    }
  }
  return;
}



pub unsafe fn pass1_1020_d2ee(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn struct_1020_d37c(param_1: *mut astruct_180) -> *mut u16

{
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = 0;
  param_1.field0_0x0 = 0xd53e;
  (param_1 + 0x2) = 0x1020;
  return &param_1.field0_0x0;
}



pub unsafe fn pass1_1020_d3a4(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: u16 ,mut param_4: i16,mut param_5: u32) -> *mut u16

{
  let mut uVar1: u16;

  pass1_1028_b39e(param_1,param_2,param_4,param_5);
  uVar1 = (param_2 >> 0x10);
  (param_2 + 0x20) = param_3;
  param_2.field0_0x0 = 0xd53e;
  (param_2 + 0x2) = 0x1020;
  return &param_2.field0_0x0;
}



pub unsafe fn write_to_file_1020_d3d4(param_1: *mut astruct_731,param_2: *mut u8) -> BOOL16

{
  let mut BVar1: bool;
  in_stack_0000ffde: HFILE16;
  let mut local_c: [u16;0x5] = [0;0x5];

  BVar1 = write_to_file_1028_b5ec(param_1,param_2);
  if (BVar1 != 0) {
    local_c[0] = (param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffde);
    if (BVar1 == 0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}



pub unsafe fn pass1_1020_d41a(BOOL16 param_1,param_2: *mut u8,param_3: *mut astruct_373,param_4: *mut HFILE16) -> BOOL16

{
  let mut BVar1: bool;
  let mut local_4: u16;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0) {
    BVar1 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2);
    if (BVar1 == 0) {
      u16_1050_0310 = 0x6d2;
      return BVar1;
    }
    (param_3 + 0x20) = local_4;
    param_1 = 0x1;
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1020_d460(mut param_1: i16,mut param_2: u16 ,param_3: *mut u32,param_4: *mut u16,mut param_5: u32,mut param_6: u32) -> u16

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut uVar3: u32;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fe90: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffe8: u16;

  uVar1 = pass1_1028_bc90(param_1,param_2,param_3,param_4,param_5,param_6);
  if (uVar1 != 0) {
    uVar3 = pass1_1038_af40(_PTR_LOOP_1050_4230,param_2,_PTR_LOOP_1050_5b7c,
                            (_PTR_LOOP_1050_4230 + 0x16),0x11);
    paVar2 = CONCAT22(in_register_0000000a,(uVar3 >> 0x10));
    PTR_LOOP_1050_5b80 = (&PTR_LOOP_1050_0000 + 1);
    unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5b80);
    puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe8,0x3a),in_stack_0000fe90,
                             in_stack_0000ffb4,in_stack_0000ffba,in_stack_0000ffbe);
    (param_3 + 0x20) = (puVar4 + 0xa);
    uVar1 = 0x1;
  }
  return uVar1;
}
pub unsafe fn pass1_1020_d4ca(mut param_1: i16,param_2: *mut astruct_15)

{
  let mut BVar1: bool;
  let mut uVar2: u32;
  let mut extraout_DX: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;

  if ((param_2 + 0x20) == 0x2) {
    return;
  }
  pass1_1028_b58e(param_2);
  uVar2 = (param_1 + 0x2e);
  iVar4 = 0x63;
  uVar3 = extraout_DX;
  pass1_1038_3fb0(uVar2);
  BVar1 = pass1_1030_25b2(uVar2 & 0xffff | uVar3 << 0x10,iVar4);
  if (BVar1 != 0) {
    return;
  }
  return;
}



pub unsafe fn pass1_1020_d518(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn struct_1020_d5a6(param_1: *mut astruct_180) -> *mut u16

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0xd7fe;
  (param_1 + 0x2) = 0x1020;
  return &param_1.field0_0x0;
}



pub unsafe fn pass1_1020_d5c8(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> *mut u16

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0xd7fe;
  (param_2 + 0x2) = 0x1020;
  return &param_2.field0_0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_d5f2(mut param_1: i16,param_2: *mut astruct_15,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  let mut bStack55: u8;
  let mut local_32: [u8;0xa] = [0;0xa];
  let mut uStack40: u32;
  let mut uStack36: u32;
  let mut puStack28: *mut u32;
  let mut local_1a: u32;
  let mut iStack22: i16;
  let mut uStack20: u16;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b58e(param_2);
  local_c = (param_1 + 0xc);
  iStack18 = (param_1 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_DX;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_1;
  uStack4 = extraout_DX;
  pass1_1028_bab6(iStack18,extraout_DX,param_2);
  uVar2 = pass1_1030_2fac(CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  iStack22 = iStack8;
  uStack36 = CONCAT22(uStack36,&local_1a);
  iStack14 += 0x1;
  uStack20 = uVar2;
  if (iStack14 < uVar2) {
    puVar7 = CONCAT22(0x1050,local_32);
    iStack22 = iStack14;
    uVar5 = pass1_1028_bb24(param_2);
    uVar4 = (uVar5 >> 0x10);
    puVar3 = &local_1a;
    pass1_1030_64ce(puVar3,uVar4,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),
                    uVar5 & 0xffff | uVar4 << 0x10,puVar7);
    uStack40 = *puVar3;
    uVar4 = (puVar3 + 2);
    bStack55 = (uStack40 >> 0x18);
    uVar2 = bStack55;
    uStack36 = uStack40;
    if (bStack55 != 0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack40 & 0xffff | uVar4 << 0x10);
      puVar6 = struct_op_1030_73a8(CONCAT22(uVar4,uVar2),uVar2,uVar4);
      uVar2 = puVar6;
      ppcVar1 = (*puVar6 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b46e(uVar2,param_2,param_3);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_d6e6(mut param_1: i16,param_2: *mut astruct_15)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  let mut bStack55: u8;
  let mut local_32: [u8;0xa] = [0;0xa];
  let mut uStack40: u32;
  let mut uStack36: u32;
  let mut puStack28: *mut u32;
  let mut local_1a: u32;
  let mut iStack22: i16;
  let mut uStack20: u16;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b514(param_2);
  pass1_1028_b58e(param_2);
  local_c = (param_1 + 0xc);
  iStack18 = (param_1 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_DX;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_1;
  uStack4 = extraout_DX;
  pass1_1028_bab6(iStack18,extraout_DX,param_2);
  uStack20 = pass1_1030_2fac(CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  uStack36 = CONCAT22(uStack36,&local_1a);
  iStack22 = iStack14 + 1;
  if (iStack22 < uStack20) {
    puVar7 = CONCAT22(0x1050,local_32);
    iStack14 = iStack22;
    uVar5 = pass1_1028_bb24(param_2);
    uVar4 = (uVar5 >> 0x10);
    puVar2 = &local_1a;
    pass1_1030_64ce(puVar2,uVar4,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar2),
                    uVar5 & 0xffff | uVar4 << 0x10,puVar7);
    uStack40 = *puVar2;
    uVar4 = (puVar2 + 2);
    bStack55 = (uStack40 >> 0x18);
    uVar3 = bStack55;
    if (bStack55 != 0) {
      uStack36 = uStack40;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack40 & 0xffff | uVar4 << 0x10);
      puVar6 = struct_op_1030_73a8(CONCAT22(uVar4,uVar3),uVar3,uVar4);
      if ((puVar6 + 0xc) == 0x6a) {
        ppcVar1 = (*puVar6 + 0x24);
        (**ppcVar1)();
      }
    }
  }
  return;
}



pub unsafe fn pass1_1020_d7d8(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn struct_1020_d866(param_1: *mut astruct_180) -> *mut u16

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0xd8ec;
  (param_1 + 0x2) = 0x1020;
  return &param_1.field0_0x0;
}



pub unsafe fn pass1_1020_d888(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> *mut u16

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0xd8ec;
  (param_2 + 0x2) = 0x1020;
  return &param_2.field0_0x0;
}
pub unsafe fn FUN_1020_d8b2()

{
  return;
}
pub unsafe fn FUN_1020_d8b6()

{
  return;
}
pub unsafe fn FUN_1020_d8ba()

{
  return;
}
pub unsafe fn FUN_1020_d8be()

{
  return;
}
pub unsafe fn FUN_1020_d8c2()

{
  return;
}



pub unsafe fn FUN_1020_d8c6(mut param_1: u16 ,param_2: *mut StructD,param_3: u8) -> *mut StructD

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 1) != 0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn struct_1020_d954(param_1: *mut astruct_180)

{
  let mut in_EDX: *mut Struct57;
  let mut iVar1: *mut astruct_180;
  let mut unaff_BP: u16;
  let mut uVar1: u16;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  struct_1030_dc96(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field_0x4 = 0;
  iVar1[0x1].field_0x6 = 0;
  iVar1[0x1].field_0x8 = 0;
  param_1.field0_0x0 = 0xe792;
  iVar1.field1_0x2 = 0x1020;
  puVar2 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22(unaff_BP,0x2f),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field_0x8 = puVar2;
  iVar1[0x1].field_0xa = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn struct_1020_d99e(param_1: *mut astruct_57,param_2: *mut astruct_12,mut param_3: u16 ,mut param_4: i16,mut param_5: u32) -> *mut u16

{
  let mut iVar1: *mut astruct_12;
  let mut unaff_BP: u16;
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  puVar2 = pass1_1030_dcc2(param_1,param_2,param_4,param_5);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  iVar1.field23_0x24 = param_3;
  (iVar1 + 1).field0_0x0 = 0;
  iVar1[0x1].field1_0x2 = 0;
  param_2.field0_0x0 = 0xe792;
  iVar1.field1_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba((param_1 & 0xffff0000 | puVar2 >> 0x10),_u16_1050_0ed0,
                           CONCAT22(unaff_BP,0x2f),in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  iVar1[0x1].field1_0x2 = puVar3;
  iVar1[0x1].field_0x4 = (puVar3 >> 0x10);
  iVar1.field12_0x10 = 0x49;
  return &param_2.field0_0x0;
}
pub unsafe fn pass1_1020_d9fa(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut extraout_DX: u16;

  if ((param_2 + 0xc) != 0x79) {
    pass1_1030_df0c(param_1,param_2);
    pass1_1028_b58e(param_2);
    pass1_1038_57dc((param_1 + 0x2e),0x1,0x2);
  }
  return;
}
pub unsafe fn pass1_1020_da3c(param_1: u32)

{
  pass1_1028_bdac(param_1,0x2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_da4e(mut param_1: u16 ,param_2: *mut u32,param_3: *mut u16,mut param_4: u32,mut param_5: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut BVar4: bool;
  let mut extraout_DX: *mut u8;
  let mut puVar5: *mut u8;
  let mut extraout_DX_00: *mut u8;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut uVar8: u32;
  let mut uVar9: u16;
  let mut uVar11: u16;
  let mut puVar10: *mut u16;
  let mut uVar12: u32;
  let mut bStack31: u8;
  let mut local_e: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u32;

  puVar2 = &local_e;
  pass1_1030_64ce(puVar2,param_1,_PTR_LOOP_1050_5740,param_3,param_5,CONCAT22(0x1050,puVar2));
  uStack6 = *puVar2;
  uVar6 = (puVar2 + 2);
  bStack31 = (uStack6 >> 0x18);
  uVar3 = bStack31;
  if (bStack31 != 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | uVar6 << 0x10);
    uVar7 = struct_op_1030_73a8(CONCAT22(uVar6,uVar3),uVar3,uVar6);
    uVar6 = (uVar7 >> 0x10);
    uVar3 = uVar7;
    if ((uVar3 + 0xc) == 0x10) {
      PTR_LOOP_1050_50ca = 0x6a9;
      return;
    }
  }
  uVar9 = param_2;
  uVar11 = (param_2 >> 0x10);
  pass1_1028_c7b6(uVar6,uVar9,uVar11,param_3,param_5);
  uVar8 = param_2 & 0xffff | uVar11 << 0x10;
  ppcVar1 = (*param_2 + 0x60);
  puVar10 = param_3;
  uVar7 = param_4;
  uVar12 = param_5;
  uStack8 = uVar3;
  (**ppcVar1)();
  if (((uVar3 != 0) &&
      (puVar5 = extraout_DX, pass1_1028_c23e(uVar3,extraout_DX,uVar9,uVar11,param_3,param_4,param_5), uVar3 != 0x0
      )) && (BVar4 = pass1_1028_c314(uVar3,puVar5,uVar9,uVar11,param_3,param_4,(param_4 >> 0x10)
                                     ,param_5), BVar4 != 0)) {
    uVar6 = (param_3 >> 0x10);
    if ((((param_3 + 0x4) == 0) && (uStack8 != 0x4)) &&
       (ppcVar1 = (*param_2 + 0x5c),
       (**ppcVar1)(0x1028,param_2,param_3,param_4,param_5,uVar8,puVar10,uVar7,uVar12), puVar5 = extraout_DX_00,
       BVar4 == 0)) {
      return;
    }
    uStack10 = (param_3 + 0x4);
    if (uStack10 != 0) {
      pass1_1020_df10(uStack10,puVar5,param_2,(param_3 & 0xffff | uVar6 << 0x10),param_5)
      ;
      return;
    }
    pass1_1020_deac(0x0,puVar5,param_2,(param_3 & 0xffff | uVar6 << 0x10),param_5);
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_db86(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: u32,param_5: i32)

{
  let mut iVar1: i16;
  let mut puVar2: *mut u8;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut puVar6: *mut u16;
  let mut local_4: [u8;0x2] = [0;0x2];

  uVar5 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar4 = (uVar5 >> 0x10);
  iVar1 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar3 = (iVar1 + 0x10);
  puVar6 = param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
  puVar2 = local_4;
  pass1_1030_bcde(puVar2,&DAT_1050_1050,uVar3 & 0xffff | uVar4 << 0x10,puVar6,param_5);
  if (puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = 0x6af;
  }
  else {
    if ((puVar2 < 0x1f) || ((param_3 + 0x4) < 1)) {
      return;
    }
    PTR_LOOP_1050_50ca = 0x6b6;
    PTR_LOOP_1050_50cc = puVar2 + -0x1e;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_dc1c(param_1: *mut astruct_15,param_2: *mut u16)

{
  let mut iVar1: i16;
  let mut ppcVar2: *mut *mut code;
  let mut puVar3: *mut u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut puVar7: *mut u32;
  let mut puVar8: *mut u32;
  let mut bStack27: u8;
  let mut local_a: [u8;0x4] = [0;0x4];
  let mut uStack6: u32;

  puVar8 = CONCAT22(0x1050,local_a);
  uVar6 = pass1_1028_bb24(param_1);
  uVar5 = (uVar6 >> 0x10);
  puVar3 = uVar6;
  pass1_1030_64ce(puVar3,uVar5,_PTR_LOOP_1050_5740,param_2,uVar6 & 0xffff | uVar5 << 0x10,puVar8);
  uStack6 = *puVar3;
  uVar5 = (puVar3 + 2);
  bStack27 = (uStack6 >> 0x18);
  uVar4 = bStack27;
  if (bStack27 != 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | uVar5 << 0x10);
    puVar7 = struct_op_1030_73a8(CONCAT22(uVar5,uVar4),uVar4,uVar5);
    iVar1 = (puVar7 + 0xc);
    if (((iVar1 < 1) || (SBORROW2(iVar1,1))) ||
       ((iVar1 != 0x9 && 0x7 < iVar1 + -0x1 && ((iVar1 + -0x9 < 0x6a || (0x6 < iVar1 + -0x73)))))) {
      ppcVar2 = (*puVar7 + 0x24);
      (**ppcVar2)();
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub unsafe fn pass1_1020_dca8(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut local_2e: [u8;0xe] = [0;0xe];
  let mut puStack32: *mut u32;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut uStack26: u16;
  let mut uStack24: u16;
  let mut uStack22: u16;
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut local_10: u32;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut local_6: [u8;0x2] = [0;0x2];
  let mut local_4: i16;

  pass1_1028_c1f8(local_6,param_1,param_2,CONCAT22(0x1050,local_6),CONCAT22(0x1050,&local_4));
  uStack10 = pass1_1028_b58e(param_2);
  uVar1 = (uStack10 >> 0x10);
  local_10 = (uStack10 + 0xc);
  uStack12 = (uStack10 + 0x10);
  puStack32 = &local_10;
  uStack18 = local_10;
  uStack20 = (local_10 >> 0x10);
  uStack24 = local_10 - 0x1;
  if (uStack24 < 0x0) {
    uStack24 = 0;
  }
  uVar2 = local_4 - 0x1;
  uStack26 = local_10 + 1;
  if (uVar2 < (local_10 + 1)) {
    uStack26 = uVar2;
  }
  uStack28 = uStack20 - 0x1;
  if (uStack28 < 0x0) {
    uStack28 = 0;
  }
  uStack30 = uStack20 + 1;
  if (uVar2 < (uStack20 + 1)) {
    uStack30 = uVar2;
  }
  uStack22 = uStack12;
  pass1_1008_3e54(CONCAT22(0x1050,local_2e),uStack12,uStack28,uStack24);
  pass1_1020_dc1c(param_2,CONCAT22(0x1050,local_2e));
  pass1_1008_3e54(CONCAT22(0x1050,local_2e),uStack22,uStack28,uStack18);
  pass1_1020_dc1c(param_2,CONCAT22(0x1050,local_2e));
  pass1_1008_3e54(CONCAT22(0x1050,local_2e),uStack22,uStack28,uStack26);
  pass1_1020_dc1c(param_2,CONCAT22(0x1050,local_2e));
  pass1_1008_3e54(CONCAT22(0x1050,local_2e),uStack22,uStack20,uStack24);
  pass1_1020_dc1c(param_2,CONCAT22(0x1050,local_2e));
  pass1_1008_3e54(CONCAT22(0x1050,local_2e),uStack22,uStack20,uStack26);
  pass1_1020_dc1c(param_2,CONCAT22(0x1050,local_2e));
  pass1_1008_3e54(CONCAT22(0x1050,local_2e),uStack22,uStack30,uStack24);
  pass1_1020_dc1c(param_2,CONCAT22(0x1050,local_2e));
  pass1_1008_3e54(CONCAT22(0x1050,local_2e),uStack22,uStack30,uStack18);
  pass1_1020_dc1c(param_2,CONCAT22(0x1050,local_2e));
  pass1_1008_3e54(CONCAT22(0x1050,local_2e),uStack22,uStack30,uStack26);
  pass1_1020_dc1c(param_2,CONCAT22(0x1050,local_2e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_de32(param_1: *mut u8,mut param_2: u32,mut param_3: u16 )

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut uVar4: u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000fff0: u16;

  puVar5 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           CONCAT22(in_stack_0000fff0,0x5),in_stack_0000fe98,in_stack_0000ffbc,
                           in_stack_0000ffc2,in_stack_0000ffc6);
  uVar1 = (puVar5 >> 0x10);
  (puVar5 + 0x12) = param_3;
  uVar3 = uVar1;
  BVar2 = bring_win_to_top_1038_b72e(_PTR_LOOP_1050_5b7c,0x4);
  if (BVar2 == 0) {
    pass1_1038_af40(_PTR_LOOP_1050_4230,uVar3,_PTR_LOOP_1050_5b7c,
                    (_PTR_LOOP_1050_4230 + 0x16),0x4);
  }
  PTR_LOOP_1050_5b80 = (&PTR_LOOP_1050_0000 + 1);
  unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5b80);
  uVar4 = (param_2 >> 0x10);
  (param_2 + 0x24) = (puVar5 + 0xa);
  if ((param_2 + 0x24) == 0) {
    PTR_LOOP_1050_50ca = 0x6b2;
  }
  return;
}



pub unsafe fn pass1_1020_deac(mut param_1: i16,param_2: *mut u8,mut param_3: u32,param_4: *mut u16,param_5: i32) -> BOOL16

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar1 = param_3;
  uVar2 = (param_3 >> 0x10);
  pass1_1028_c7b6(param_2,uVar1,uVar2,param_4,param_5);
  if (param_1 < 1) {
    return 0x0;
  }
  if (SBORROW2(param_1,1)) {
    return 0x0;
  }
  if (param_1 != 0x3 && 0x0 < param_1 + -0x2) {
    if (param_1 == 0x4) {
      pass1_1020_de32(param_2,param_3,0x4);
      if ((uVar1 + 0x24) == 0x6) {
        return 0x1;
      }
      return 0x0;
    }
    if (param_1 != 0x5) {
      return 0x0;
    }
  }
  (uVar1 + 0x24) = 0x1;
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_df10(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32,param_4: *mut u16,param_5: i32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut bStack31: u8;
  let mut local_e: u32;
  let mut uStack10: u32;
  let mut uStack6: u16;
  let mut uStack4: u16;

  uStack4 = 0;
  uVar6 = param_3;
  uVar7 = (param_3 >> 0x10);
  pass1_1028_c7b6(param_2,uVar6,uVar7,param_4,param_5);
  uStack6 = param_1;
  if (param_1 == 0) {
    puVar1 = &local_e;
    pass1_1030_64ce(puVar1,param_2,_PTR_LOOP_1050_5740,param_4,param_5,CONCAT22(0x1050,puVar1));
    uStack10 = *puVar1;
    uVar4 = (puVar1 + 2);
    bStack31 = (uStack10 >> 0x18);
    uVar2 = bStack31;
    if (bStack31 != 0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack10 & 0xffff | uVar4 << 0x10);
      uVar5 = struct_op_1030_73a8(CONCAT22(uVar4,uVar2),uVar2,uVar4);
      if ((uVar5 + 0xc) == 0x6a) {
        BVar3 = pass1_1020_e044(param_3);
        if (BVar3 == 0) {
          (uVar6 + 0x24) = 0x1;
        }
        else {
          PTR_LOOP_1050_50ca = 0x6ac;
        }
      }
    }
  }
  else if (((0x5 < param_1) && (!SBORROW2(param_1,0x6))) && ((param_1 - 0x6) < 0x4)) {
    pass1_1020_de32(param_2,param_3,param_1);
    match (uVar6 + 0x24) {
    0x1 =>
      BVar3 = pass1_1020_e044(param_3);
      if (BVar3 != 0) {
        PTR_LOOP_1050_50ca = 0x6ac;
      }
      break;
    0x2 =>
    0x3 =>
    0x4 =>
    0x5 =>
      pass1_1020_e652(param_3,param_4,(param_4 >> 0x10),param_5);
    }
  }
  return;
}
