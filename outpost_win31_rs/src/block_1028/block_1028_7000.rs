
pub unsafe fn pass1_1028_737e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut puVar3: *mut u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar6 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = paVar6;
  puStack10 = CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    (param_1 + 0x4) = (param_3 + 0x4);
    puVar3 = (param_3 + 0x8);
    puVar7 = (param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0; iVar4 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0x749e;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_740c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut puVar5: *mut u8;
  let mut extraout_DX: u16;
  let mut puVar6: *mut u32;
  let mut lStack14: i32;
  let mut puStack10: *mut u32;

  puVar6 = pass1_1008_c6fa(_u16_1050_06e0,param_3);
  puVar5 = (puVar6 >> 0x10);
  uVar3 = puVar6;
  pass1_1038_4d6e(uVar3,puVar5,param_4,puVar6);
  puStack10 = CONCAT22(puVar5,uVar3);
  uVar2 = *puStack10;
  ppcVar1 = uVar2 + 0x8;
  uVar4 = uVar3;
  (**ppcVar1)(&u16_1050_1038,uVar3,puVar5);
  lStack14 = CONCAT22(extraout_DX,uVar4);
  if (puStack10.is_null() == false) {
    ppcVar1 = uVar2;
    (**ppcVar1)(&u16_1050_1038,uVar3,puVar5,1);
  }
  if (lStack14 != 0) {
    return;
  }
  return;
}



pub unsafe fn pass1_1028_7472(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1.address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn pass1_1028_74ae(param_1: *mut astruct_97) -> *mut astruct_97

{
  struct_op_1028_d1dc(param_1,0x1387);
  param_1.offset_0x0 = 0x819a;
    // just 0x1028
  (param_1 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)),s_SCEvent_1050_4ff4);
  return param_1;
}



pub unsafe fn pass1_1028_74e4(param_1: u8,param_2: *mut astruct_57,mut param_3: u32) -> u16

{
  let mut iVar1: i16;

  pass1_1028_7fb6(param_1,param_3);
  pass1_1028_7c4e(param_2,param_3);
  pass1_1028_7dfc(param_1,param_2,param_3);
  iVar1 = post_msg_1028_76da();
  pass1_1028_767e(iVar1,param_2);
  pass1_1028_75bc();
  pass1_1028_78b8(param_1,param_2,param_3);
  return 0x1;
}
pub unsafe fn pass1_1028_752e(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut puVar3: *mut u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar6 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = paVar6;
  puStack10 = CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    (param_1 + 0x4) = (param_3 + 0x4);
    puVar3 = (param_3 + 0x8);
    puVar7 = (param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0; iVar4 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0x819a;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_75bc()

{
  let mut paVar1: *mut astruct_92;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut in_stack_0000ffcc: u16;
  let mut uVar5: u32;
  let mut uStack28: u32;
  let mut local_18: *mut astruct_92;

  uVar2 = (*_PTR_LOOP_1050_65e2 % 0x7b);
  uVar4 = uVar2;
  if ((uVar2 == 0) && (0x95 < *_PTR_LOOP_1050_65e2)) {
    uVar5 = CONCAT22(0x7603,in_stack_0000ffcc);
    pass1_1028_dc52(CONCAT22(0x1050,&local_18),0x1,0x0,0x400);
    loop {
      paVar1 = &local_18;
      pass1_1028_e4ec(CONCAT22(0x1050,paVar1));
      uStack28 = CONCAT22(uVar4,paVar1);
      uVar2 = uVar4 | paVar1;
      uVar4 = uVar4 & 0xffff0000 | uVar2;
      if (uVar2 == 0) { break; }
      pass1_1008_612e(paVar1,0x1,0x64);
      if (paVar1 < 0x6) {
        pass1_1038_362e(uStack28,paVar1,uVar5,uVar4);
      }
    }
    if (local_18.field6_0x10 != 0) {
      local_18.field5_0xc = 0x1;
      local_18.field5_0xc = 0;
    }
    uVar4 = local_18.field5_0xc;
    local_18.field4_0x8 = local_18.field5_0xc;
    local_18.field4_0x8 = local_18.field5_0xc;
    loop {
      uVar2 = uVar4;
      paVar1 = &local_18;
      pass1_1028_e4ec(CONCAT22(0x1050,paVar1));
      uVar3 = uVar2 | paVar1;
      uVar4 = uVar3;
      if (uVar3 == 0) { break; }
      pass1_1038_3698(paVar1,uVar3,CONCAT22(uVar2,paVar1));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_767e(mut param_1: i16,mut param_2: u16 )

{
  let mut uVar1: u16;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffec: u16;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x8000001);
  if (((param_1 + 0x152) != 0) && (uVar1 = (*_PTR_LOOP_1050_65e2 % 0x64), uVar1 == 0)) {
    puVar2 = mixed_1010_20ba(uVar1,_u16_1050_0ed0,CONCAT22(in_stack_0000ffec,0x40),
                             in_stack_0000fe94,in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
    load_str_and_sprintf_1008_b78a(CONCAT22(puVar2,(puVar2 >> 0x10)),puVar2);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn post_msg_1028_76da()

{
  let mut lVar1: i32;
  let mut uVar2: u16;
  let mut in_EDX: *mut Struct57;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffe4: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;

  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22((in_stack_0000ffe4 >> 0x10),0x2c),
                           in_stack_0000fe8e,in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar2 = (puVar3 >> 0x10);
  lVar1 = (puVar3 + 0xc);
  uStack8 = (lVar1 >> 0x10);
  uStack10 = lVar1;
  if (((uStack8 | uStack10) != 0) && (*_PTR_LOOP_1050_65e2 == lVar1)) {
    PostMessage16(0x0,0x106,0x111,HWND16_1050_0396);
    (puVar3 + 0xc) = 0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_7742(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,param_4: *mut astruct_15)

{
  let mut ppcVar1: *mut *mut code;
  let mut paVar2: *mut astruct_670;
  let mut uVar3: u16;
  let mut puVar4: *mut u8;
  let mut uVar5: u16;
  let mut puVar6: *mut u8;
  let mut extraout_DX: u16;
  let mut uVar7: u16;
  let mut extraout_DX_00: u16;
  let mut puVar8: *mut u32;
  let mut paVar9: *mut astruct_691;
  let mut uVar10: u32;
  let mut uVar11: u8;
  let mut uVar12: u8;
  let mut uVar13: u16;
  let mut uStack26: u32;
  let mut local_16: [u8;0x2] = [0;0x2];
  let mut uStack20: u32;
  let mut uStack16: u16;
  let mut puStack14: *mut u32;
  let mut uStack10: u16;
  let mut puStack8: *mut u8;
  let mut uStack6: u16;
  let mut uStack4: u16;

  puVar8 = pass1_1008_c6fa(_u16_1050_06e0,0x18);
  uVar5 = (puVar8 >> 0x10);
  uVar7 = SUB42(puVar8,0x0);
  uStack6 = uVar7;
  uStack4 = uVar5;
  paVar9 = pass1_1028_b4f2(param_4);
  puVar6 = (paVar9 >> 0x10);
  uVar3 = paVar9;
  uStack10 = uVar3;
  puStack8 = puVar6;
  pass1_1038_4d6e(uVar3,puVar6,paVar9,CONCAT22(uVar5,uVar7));
  puStack14 = CONCAT22(puVar6,uVar3);
  uStack16 = 0;
  ppcVar1 = (*puStack14 + 0x10);
  uVar13 = uVar3;
  (**ppcVar1)(&u16_1050_1038,uVar3,puVar6);
  uStack20 = CONCAT22(extraout_DX,uVar3);
  uVar10 = pass1_1030_bcae(local_16,&DAT_1050_1050);
  uVar7 = (uVar10 >> 0x10);
  uStack26 = 0;
  loop {
    if (uStack20 <= uStack26) {//
// LAB_1028_77e7:
      if (puStack14.is_null() == false) {
        ppcVar1 = *puStack14;
        (**ppcVar1)(0x1030,puStack14,(puStack14 >> 0x10),0x1,uVar13,puVar6,puStack14,puStack14);
      }
      return;
    }
    uVar10 = uStack20;
    pass1_1030_1d58(puStack14);
    uVar5 = uVar10;
    uVar11 = uVar10;
    uVar12 = (uVar10 >> 0x8);
    pass1_1028_b58e(param_4);
    puVar4 = local_16;
    paVar2 = CONCAT22(uVar7,CONCAT11(uVar12,uVar11));
    uVar7 = extraout_DX_00;
    pass1_1030_bd74(puVar4,&DAT_1050_1050,CONCAT22(extraout_DX_00,uVar5),paVar2);
    if (puVar4 <= param_3) {
      uStack16 = 0x1;
  // TODO: goto LAB_1028_77e7;
    }
    uStack26 += 0x1;
  }
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_780c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut in_EDX: u32;
  let mut paVar7: *mut Struct57;
  let mut uVar8: u16;
  let mut puVar9: *mut u32;
  let mut puVar10: *mut u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut puStack10: *mut u32;

  uVar8 = (in_EDX >> 0x10);
  puVar9 = pass1_1008_c6fa(_u16_1050_06e0,0x25);
  paVar7 = CONCAT22(uVar8,(puVar9 >> 0x10));
  uVar2 = puVar9;
  uVar8 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4e78(uVar2,paVar7,param_3,puVar9);
  uVar6 = paVar7;
  puStack10 = CONCAT22(uVar6,uVar2);
  ppcVar1 = (*puStack10 + 0x10);
  uVar3 = uVar2;
  (**ppcVar1)(&u16_1050_1038,uVar2,uVar6);
  uStack14 = CONCAT22(uVar6,uVar3);
  uVar5 = (uVar6 | uVar3);
  if ((uVar6 | uVar3) == 0) {
    return;
  }
  uStack18 = 0;
  loop {
    uVar3 = uVar5;
    if (uStack14 <= uStack18) { break; }
    ppcVar1 = (*puStack10 + 0x4);
    uVar5 = uStack14;
    (**ppcVar1)();
    uVar4 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5 & 0xffff | uVar3 << 0x10);
    uVar8 = 0x1030;
    puVar10 = struct_op_1030_73a8(CONCAT22(uVar3,uVar4),uVar4,uVar3);
    uVar5 = puVar10 >> 0x10;
    ppcVar1 = (*puVar10 + 0x24);
    (**ppcVar1)();
    uStack18 += 0x1;
  }
  if (puStack10.is_null() == false) {
    ppcVar1 = *puStack10;
    (**ppcVar1)(uVar8,uVar2,paVar7,1);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_78b8(param_1: u8,param_2: i32,mut param_3: u32)

{
  let mut puVar1: *mut u8;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut paVar4: *mut astruct_92;
  let mut puVar5: *mut u8;
  let mut puVar6: *mut u16;
  let mut puVar7: *mut u16;
  let mut uVar8: u32;
  let mut puVar9: *mut u8;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut iVar12: i16;
  let mut paVar13: *mut Struct57;
  let mut unaff_SI: u16;
  let mut bVar15: bool;
  let mut bVar16: bool;
  let mut puVar17: *mut u16;
  let mut puVar18: *mut u16;
  let mut paVar19: *mut astruct_27;
  let mut puVar20: *mut u32;
  let mut in_stack_0000fd48: u16;
  let mut in_stack_0000fd4e: u16;
  let mut in_stack_0000fd52: u16;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000fe76: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000fe7c: u16;
  let mut uVar21: u16;
  let mut uVar22: u16;
  let mut uVar23: u16;
  let mut iVar24: i16;
  let mut uStack340: u16;
  let mut uStack338: u16;
  let mut puStack74: *mut u32;
  let mut puStack70: *mut u8;
  let mut uStack68: u16;
  let mut local_42: *mut astruct_92;
  let mut local_30: *mut astruct_92;
  local_1e: *mut u8 [0x3];
  let mut local_18: u32;
  let mut puStack20: *mut u8;
  let mut uStack18: u16;
  let mut paStack16: *mut astruct_364;
  let mut puStack12: *mut u8;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u32;
  let mut paVar14: *mut Struct57;

  puVar9 = param_2;
  uVar8 = *_PTR_LOOP_1050_65e2;
  uStack6 = uVar8;
  if (uVar8 == 0x98) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
    puVar9 = param_2;
    paStack16 = (uVar8 & 0xffff | param_2 << 0x10);
    if ((uVar8 + 0x200) == 0x8000002) {
      pass1_1020_a43e(puVar9,CONCAT22(0x1050,&local_18));
      puVar17 = pass1_1008_3e38(CONCAT22(0x1050,local_1e));
      puVar9 = (puVar17 >> 0x10);
      puVar2 = &local_18;
      pass1_1020_a49a(puVar9,in_stack_0000fd52,CONCAT22(0x1050,puVar2),CONCAT22(0x1050,local_1e),0x7a);
      pass1_1038_4f54(puVar2,paStack16,1);
      if (puVar2.is_null()) {
        pass1_1020_a49a(puVar9,in_stack_0000fd52,CONCAT22(0x1050,&local_18),NULL,0x35);
      }
    }
  }
  if ((0xe < uStack6) && (uStack6 < 0x16)) {
    puVar18 = pass1_1020_a43e(puVar9,CONCAT22(0x1050,local_1e));
    local_18 = uStack6 - 0xf;
    pass1_1020_a54c((puVar18 >> 0x10),local_1e,&DAT_1050_1050,local_18);
  }
  paVar13 = (uStack6 % 0x7d);
  paVar14 = paVar13;
  if (paVar13.is_null()) {
    pass1_1008_612e((uStack6 % 0x7d),0x1,0x64);
    local_1e[0] = paVar13;
    if (local_1e[0] < 0x1a) {
      pass1_1028_dc52(CONCAT22(0x1050,&local_30),0x1,0x0,0x400);
      loop {
        paVar13 = ZEXT24(&local_30);
        pass1_1028_e4ec(CONCAT22(0x1050,&local_30));
        uVar3 = paVar13;
        uVar10 = paVar14;
        local_18 = paVar13 & 0xffff | paVar14 << 0x10;
        uVar11 = uVar10 | uVar3;
        paVar14 = (paVar14 & 0xffff0000 | uVar11);
//        if (uVar11 == 0) goto LAB_1028_79d6;
      } while ((uVar3 + 0x200) == 0x8000002);
      pass1_1038_43cc(uVar3,uVar11,uVar3,uVar10,0x1,0x4);//
// LAB_1028_79d6:
      local_30 = 0x389a;
      local_30.field2_0x2 = 0x1008;
    }
  }
  puVar5 = paVar13;
  if (uStack6 == 0x5) {
    uVar23 = SUB42(&DAT_1050_1050,0x0);
    uVar22 = SUB42(s_Rebel_1050_4ffc,0x0);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
    local_30.field2_0x2 = paVar14;
    local_30 = puVar5;
    pass1_1038_4d3c(CONCAT22(local_30.field2_0x2,puVar5),CONCAT22(uVar23,uVar22),local_30.field2_0x2);
  }
  if (uStack6 == 0x12c) {
    uVar23 = 0x400;
    iVar12 = 0xf;
    uVar22 = 0x1;
    paVar19 =
              mixed_1010_20ba(paVar14,_u16_1050_0ed0,0x1002b,in_stack_0000fd48,in_stack_0000fe6c,
                              in_stack_0000fe72,in_stack_0000fe76);
    paVar14 = (paVar14 & 0xffff0000 | paVar19 >> 0x10);
    puVar5 = paVar19;
    local_30.field2_0x2 = (paVar19 >> 0x10);
    local_30 = puVar5;
    pass1_1010_043a(paVar19,CONCAT22(uVar23,uVar22),iVar12);
  }
  if (uStack6 == 0x3d) {
    puVar20 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2),in_stack_0000fd4e,
                              in_stack_0000fe72,in_stack_0000fe78,in_stack_0000fe7c);
    paVar14 = (paVar14 & 0xffff0000 | puVar20 >> 0x10);
    local_30 = puVar20;
    local_30.field2_0x2 = (puVar20 >> 0x10);
    local_1e[0] = PTR_LOOP_1050_13ae;
    puVar5 = PTR_LOOP_1050_13ae;
    if (PTR_LOOP_1050_13ae != (&PTR_LOOP_1050_0000 + 1)) {
      pass1_1028_dc52(CONCAT22(0x1050,&local_42),0x1,0x0,0x400);
      loop {
        paVar4 = &local_42;
        pass1_1028_e4ec(CONCAT22(0x1050,paVar4));
        uVar3 = paVar14;
        local_18 = CONCAT22(uVar3,paVar4);
        paVar14 = (paVar14 & 0xffff0000 | (uVar3 | paVar4));
        if ((uVar3 | paVar4) == 0) { break; }
        paStack16 = &paVar4[0x1b].field6_0x10;
        pass1_1030_34da(paStack16);
      }
      uVar23 = 0x400;
      iVar12 = 0x10;
      uVar22 = 0x1;
      paVar19 =
                mixed_1010_20ba(paVar14,_u16_1050_0ed0,0x1002b,in_stack_0000fd48,in_stack_0000fe6c,
                                in_stack_0000fe72,in_stack_0000fe76);
      paVar14 = (paVar14 & 0xffff0000 | paVar19 >> 0x10);
      puVar5 = paVar19;
      uStack18 = (paVar19 >> 0x10);
      puStack20 = puVar5;
      pass1_1010_043a(paVar19,CONCAT22(uVar23,uVar22),iVar12);
      local_42._0_4_ = &PTR_pass1_1008_377e_1008_389a;
    }
  }
  if (uStack6 == 0x96) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
    puStack74 = CONCAT22(paVar14,puVar5);
    uVar21 = (param_3 >> 0x10);
    pass1_1028_780c(param_3,uVar21,CONCAT22(paVar14,puVar5));
    if (puVar5.is_null() == false) {
      uVar23 = 0x400;
      iVar12 = 0x1d;
      uVar22 = 0x1;
      paVar19 =
                mixed_1010_20ba(paVar14,_u16_1050_0ed0,0x1002b,in_stack_0000fd48,in_stack_0000fe6c,
                                in_stack_0000fe72,in_stack_0000fe76);
      paVar14 = (paVar14 & 0xffff0000 | paVar19 >> 0x10);
      puVar5 = paVar19;
      uStack68 = (paVar19 >> 0x10);
      puStack70 = puVar5;
      pass1_1010_043a(paVar19,CONCAT22(uVar23,uVar22),iVar12);
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
    puStack74 = CONCAT22(paVar14,puVar5);
    pass1_1028_780c(param_3,uVar21,CONCAT22(paVar14,puVar5));
  }
  puVar20 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2),in_stack_0000fd4e,
                            in_stack_0000fe72,in_stack_0000fe78,in_stack_0000fe7c);
  uStack10 = SUB42(puVar20,0x0);
  uStack8 = (puVar20 >> 0x10);
  puStack12 = PTR_LOOP_1050_13ae;
  if (0x2 < PTR_LOOP_1050_13ae) {
    puStack74 = mixed_1010_20ba((paVar14 & 0xffff0000 | puVar20 >> 0x10),_u16_1050_0ed0,
                                CONCAT22(unaff_SI,0x2f),in_stack_0000fd4e,in_stack_0000fe72,
                                in_stack_0000fe78,in_stack_0000fe7c);
    for (puStack70 = 0x1; puStack70 < 0x9; puStack70 = (puStack70 + 1)) {
      local_42._0_4_ = (puStack74 + 0x34 + puStack70 * 0x4);
      if (local_42._0_4_ == uStack6) {
        puVar5 = (&PTR_LOOP_1050_0000 + 1);
        local_30 = 0x1;
        pass1_1008_612e(0x1,0x1,0x64);
        puVar7 = (puStack70 - 0x7);
        if (puVar7.is_null()) {
          bVar16 = SBORROW2(puVar5,0x32);
          puVar1 = puVar5 + -0x32;
          bVar15 = puVar5 == (s_New_failed_in_Op__Op_1050_0020 + 0x12);//
// LAB_1028_7b74:
          if (!bVar15 && bVar16 == puVar1 < 0x0) {
            local_30 = null_mut();
          }
        }
        else {
          puVar7 = (puStack70 - 0x8);
          if (puVar7.is_null()) {
            bVar16 = SBORROW2(puVar5,0x19);
            puVar1 = puVar5 + -0x19;
            bVar15 = puVar1.is_null();
        // TODO: goto LAB_1028_7b74;
          }
        }
        local_1e[0] = puVar5;
        if (local_30.is_null() == false) {
          pass1_1028_90e6(CONCAT22(0x1050,&uStack340),puStack70);
          puVar7 = &uStack340;
          fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,puVar7));
          uStack340 = 0x389a;
          uStack338 = 0x1008;
        }
        pass1_1008_612e(puVar7,0x0,0xa);
        local_18 = local_18 & 0xffff0000 | ZEXT24(puVar7);
        if (puStack70 == 0x7) {
          iVar24 = 0x7;
          puVar6 = puVar7 + 0x37;
          iVar12 = puVar6 >> 0xf;
        }
        else {
//          if (puStack70 != 0x8) goto LAB_1028_7ba0;
          iVar24 = 0x8;
          puVar6 = puVar7 + 0x32;
          iVar12 = (puVar7 >> 0xf) + (0xff9b < puVar7);
        }
        uVar21 = (local_42._0_4_ >> 0x10) + iVar12 + CARRY2(local_42,puVar6);
        local_42._0_4_ = CONCAT22(uVar21,local_42 + puVar6);
        pass1_1010_ebf8(puStack74,local_42 + puVar6,uVar21,iVar24);
      }//
// LAB_1028_7ba0:
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_7c4e(param_1: *mut astruct_57,mut param_2: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u8;
  let mut paVar3: *mut astruct_92;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar7: u8;
  let mut puVar8: *mut u32;
  let mut paVar9: *mut astruct_97;
  let mut in_stack_0000fd32: u16;
  let mut in_stack_0000fe56: u16;
  let mut in_stack_0000fe5c: u16;
  let mut in_stack_0000fe60: u16;
  let mut in_stack_0000fe8a: u16;
  let mut uVar10: u16;
  let mut local_156: u16;
  let mut uStack340: u16;
  let mut uStack70: u16;
  let mut uStack68: u16;
  let mut iStack66: i16;
  let mut paStack64: *mut astruct_15;
  let mut uStack56: u32;
  let mut uStack52: u16;
  let mut uStack50: u32;
  let mut puStack46: *mut u32;
  let mut uStack42: u16;
  let mut puStack40: *mut u8;
  let mut paStack38: *mut astruct_691;
  let mut local_22: *mut astruct_92;
  let mut iStack10: i16;

  mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(in_stack_0000fe8a,0x2),in_stack_0000fd32,
                  in_stack_0000fe56,in_stack_0000fe5c,in_stack_0000fe60);
  puVar2 = PTR_LOOP_1050_13ae;
  if (0x2 < PTR_LOOP_1050_13ae) {
    uVar6 = *_PTR_LOOP_1050_65e2;
    iStack10 = (uVar6 >> 0x10);
    if ((0x2 < uVar6) && (uVar6 = CONCAT22(iStack10 - (uVar6 < 0x2),uVar6 - 0x2) % 0x14, uVar6 == 0)
       ) {
      pass1_1028_dc52(CONCAT13(0x10,CONCAT12(0x50,&local_22)),0x1,0x0,0x400);
      loop {
        uVar5 = uVar6;
        paVar3 = &local_22;
        pass1_1028_e4ec(CONCAT22(0x1050,paVar3));
        paStack38 = CONCAT22(uVar5,paVar3);
        uVar6 = (uVar5 | paVar3);
        if ((uVar5 | paVar3) == 0) { break; }
        if (paVar3[0x1c].field4_0x8 != 0x8000002) {
          puVar8 = pass1_1008_c6fa(_u16_1050_06e0,0x2a);
          uVar6 = puVar8 >> 0x10;
          uVar5 = puVar8;
          puStack40 = (puVar8 >> 0x10);
          uVar7 = 0x38;
          uStack42 = uVar5;
          pass1_1038_4d6e(uVar5,puStack40,paStack38,puVar8);
          puStack46 = CONCAT22(uVar6,uVar5);
          ppcVar1 = (*puStack46 + 0x10);
          (**ppcVar1)(&u16_1050_1038,uVar5,uVar6);
          uStack50 = CONCAT22(uVar6,uVar5);
          if (puVar2 == (&u16_1050_0002 + 1)) {
            uStack52 = 0x6;
          }
          else {
            uStack52 = 0xc;
          }
          for uStack56 in 0 .. uStack50 {
            paStack64 = pass1_1030_1d7c(uStack50,uVar6,puStack46);
            uVar6 = paStack64 >> 0x10;
            iVar4 = paStack64;
            pass1_1028_7742(param_2,(param_2 >> 0x10),0x4,paStack64);
            uVar5 = uStack52;
            if (iVar4 == 0) {
              uVar5 = 0x19;
            }
            uVar7 = 0x8;
            uStack68 = uVar5;
            iStack66 = iVar4;
            pass1_1008_612e(uVar5,0x1,0x64);
            uStack70 = uVar5;
            if (uVar5 <= uStack68) {
              paVar9 = pass1_1028_8fc0(CONCAT13(0x10,CONCAT12(0x50,&local_156)),
                                       (paStack64 + 0x4),(paStack38 + 0x4));
              uVar6 = paVar9 >> 0x10;
              uVar7 = 0x30;
              fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_156));
              local_156 = 0x389a;
              uStack340 = 0x1008;
            }
          }
          uVar10 = (puStack46 >> 0x10);
          if (puStack46.is_null() == false) {
            ppcVar1 = *puStack46;
            (**ppcVar1)(uVar7,puStack46,uVar10,0x1,puStack46,uVar10,puStack46);
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_7dfc(param_1: u8,param_2: *mut u8,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u8;
  let mut paVar3: *mut astruct_92;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut uVar5: u32;
  let mut paVar6: *mut Struct57;
  let mut uVar7: u8;
  let mut puVar8: *mut u32;
  let mut puVar9: *mut u32;
  let mut paVar10: *mut astruct_97;
  let mut in_stack_0000fd30: u16;
  let mut in_stack_0000fe54: u16;
  let mut in_stack_0000fe5a: u16;
  let mut in_stack_0000fe5e: u16;
  let mut in_stack_0000fe88: u16;
  let mut uVar11: u16;
  let mut local_158: u16;
  let mut uStack342: u16;
  let mut uStack72: u16;
  let mut uStack70: u16;
  let mut paStack68: *mut astruct_15;
  let mut uStack60: u32;
  let mut uStack56: u16;
  let mut uStack54: u16;
  let mut iStack52: i16;
  let mut uStack50: u32;
  let mut puStack46: *mut u32;
  let mut uStack42: u16;
  let mut puStack40: *mut u8;
  let mut paStack38: *mut astruct_691;
  let mut local_22: *mut astruct_92;

  mixed_1010_20ba(CONCAT22(in_register_0000000a,param_2),_u16_1050_0ed0,
                  CONCAT22(in_stack_0000fe88,0x2),in_stack_0000fd30,in_stack_0000fe54,in_stack_0000fe5a,
                  in_stack_0000fe5e);
  puVar2 = PTR_LOOP_1050_13ae;
  if (((0x2 < PTR_LOOP_1050_13ae) && (0x3 < *_PTR_LOOP_1050_65e2)) &&
     (uVar5 = *_PTR_LOOP_1050_65e2 % 0x14, uVar5 == 0)) {
    pass1_1028_dc52(CONCAT13(0x10,CONCAT12(0x50,&local_22)),0x1,0x0,0x400);
    loop {
      paVar3 = &local_22;
      pass1_1028_e4ec(CONCAT22(0x1050,paVar3));
      uVar4 = uVar5;
      paStack38 = CONCAT22(uVar4,paVar3);
      uVar5 = uVar5 & 0xffff0000 | (uVar4 | paVar3);
      if ((uVar4 | paVar3) == 0) { break; }
      if (paVar3[0x1c].field4_0x8 != 0x8000002) {
        puVar8 = pass1_1008_c6fa(_u16_1050_06e0,0x29);
        paVar6 = (uVar5 & 0xffff0000 | puVar8 >> 0x10);
        uVar4 = puVar8;
        puStack40 = (puVar8 >> 0x10);
        uStack42 = uVar4;
        pass1_1038_4d6e(uVar4,puStack40,paStack38,puVar8);
        puStack46 = CONCAT22(paVar6,uVar4);
        ppcVar1 = (*puStack46 + 0x10);
        (**ppcVar1)(&u16_1050_1038,uVar4,paVar6);
        uStack50 = CONCAT22(paVar6,uVar4);
        uVar7 = 0x10;
        puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,CONCAT22(in_stack_0000fe88,0x2),in_stack_0000fd30,
                                 in_stack_0000fe54,in_stack_0000fe5a,in_stack_0000fe5e);
        uVar5 = paVar6 & 0xffff0000 | puVar9 >> 0x10;
        uStack56 = SUB42(puVar9,0x0);
        uStack54 = (puVar9 >> 0x10);
        if (puVar2 == (&u16_1050_0002 + 1)) {
          iStack52 = 0x5;
        }
        else {
          iStack52 = 0x1e;
        }
        for uStack60 in 0 .. uStack50 {
          paStack68 = pass1_1030_1d7c(uStack50,uVar5,puStack46);
          uVar5 = uVar5 & 0xffff0000 | paStack68 >> 0x10;
          uVar4 = paStack68;
          uVar7 = 0x8;
          pass1_1008_612e(uVar4,0x1,0x64);
          uStack70 = uVar4;
          if ((uVar4 <= iStack52) &&
             (pass1_1028_7742(param_3,(param_3 >> 0x10),0x4,paStack68), uStack72 = uVar4, uVar4 == 0)) {
            paVar10 = pass1_1028_b0de(CONCAT13(0x10,CONCAT12(0x50,&local_158)),
                                      (paStack68 + 0x4),(paStack38 + 0x4));
            uVar5 = uVar5 & 0xffff0000 | paVar10 >> 0x10;
            uVar7 = 0x30;
            fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_158));
            local_158 = 0x389a;
            uStack342 = 0x1008;
          }
        }
        in_stack_0000fe88 = SUB42(puStack46,0x0);
        uVar11 = (puStack46 >> 0x10);
        if (puStack46.is_null() == false) {
          ppcVar1 = *puStack46;
          (**ppcVar1)(uVar7,in_stack_0000fe88,uVar11,0x1,in_stack_0000fe88,uVar11,puStack46);
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_7fb6(param_1: u8,mut param_2: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut paVar2: *mut astruct_92;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut paVar5: *mut Struct57;
  let mut uVar6: u8;
  let mut puVar7: *mut u32;
  let mut puVar8: *mut u32;
  let mut paVar9: *mut astruct_97;
  let mut in_stack_0000fd30: u16;
  let mut in_stack_0000fe54: u16;
  let mut in_stack_0000fe5a: u16;
  let mut in_stack_0000fe5e: u16;
  let mut in_stack_0000fe88: u16;
  let mut uVar10: u16;
  let mut local_158: u16;
  let mut uStack342: u16;
  let mut uStack72: u16;
  let mut uStack68: u16;
  let mut uStack66: u16;
  let mut paStack64: *mut astruct_15;
  let mut uStack56: u32;
  let mut iStack52: i16;
  let mut puStack50: *mut u8;
  let mut uStack48: u16;
  let mut uStack46: u16;
  let mut uStack44: u32;
  let mut puStack40: *mut u32;
  let mut uStack36: u16;
  let mut puStack34: *mut u8;
  let mut paStack32: *mut astruct_691;
  let mut local_1c: *mut astruct_92;

  if ((0xb < *_PTR_LOOP_1050_65e2) && (uVar4 = *_PTR_LOOP_1050_65e2 % 0x32, uVar4 == 0)) {
    pass1_1028_dc52(CONCAT13(0x10,CONCAT12(0x50,&local_1c)),0x1,0x0,0x400);
    loop {
      paVar2 = &local_1c;
      pass1_1028_e4ec(CONCAT22(0x1050,paVar2));
      uVar3 = uVar4;
      paStack32 = CONCAT22(uVar3,paVar2);
      uVar4 = uVar4 & 0xffff0000 | (uVar3 | paVar2);
      if ((uVar3 | paVar2) == 0) { break; }
      if (paVar2[0x1c].field4_0x8 != 0x8000002) {
        puVar7 = pass1_1008_c6fa(_u16_1050_06e0,0x11);
        paVar5 = (uVar4 & 0xffff0000 | puVar7 >> 0x10);
        uVar3 = puVar7;
        puStack34 = (puVar7 >> 0x10);
        uStack36 = uVar3;
        pass1_1038_4d6e(uVar3,puStack34,paStack32,puVar7);
        puStack40 = CONCAT22(paVar5,uVar3);
        ppcVar1 = (*puStack40 + 0x10);
        (**ppcVar1)(&u16_1050_1038,uVar3,paVar5);
        uStack44 = CONCAT22(paVar5,uVar3);
        uVar6 = 0x10;
        puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,CONCAT22(in_stack_0000fe88,0x2),in_stack_0000fd30,
                                 in_stack_0000fe54,in_stack_0000fe5a,in_stack_0000fe5e);
        uVar4 = paVar5 & 0xffff0000 | puVar8 >> 0x10;
        uStack48 = SUB42(puVar8,0x0);
        uStack46 = (puVar8 >> 0x10);
        puStack50 = PTR_LOOP_1050_13ae;
        if (PTR_LOOP_1050_13ae < 0x3) {
          iStack52 = 0x5;
        }
        else {
          iStack52 = 0x14;
        }
        for uStack56 in 0 .. uStack44 {
          uVar6 = 0x30;
          paStack64 = pass1_1030_1d7c(uStack44,uVar4,puStack40);
          uVar4 = uVar4 & 0xffff0000 | paStack64 >> 0x10;
          uVar3 = (paStack64 + 0x20);
          uStack66 = uVar3;
          if (((uVar3 != 0) && (uVar3 != 0x70)) && (uVar3 != 0x71)) {
            uVar6 = 0x8;
            pass1_1008_612e(uVar3,0x1,0x64);
            uStack68 = uVar3;
            if ((uVar3 <= iStack52) &&
               (pass1_1028_7742(param_2,(param_2 >> 0x10),0x4,paStack64), uStack72 = uVar3, uVar3 == 0)) {
              paVar9 = pass1_1028_8698(CONCAT13(0x10,CONCAT12(0x50,&local_158)),
                                       (paStack64 + 0x4),(paStack32 + 0x4));
              uVar4 = uVar4 & 0xffff0000 | paVar9 >> 0x10;
              uVar6 = 0x30;
              fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_158));
              local_158 = 0x389a;
              uStack342 = 0x1008;
            }
          }
        }
        in_stack_0000fe88 = SUB42(puStack40,0x0);
        uVar10 = (puStack40 >> 0x10);
        if (puStack40.is_null() == false) {
          ppcVar1 = *puStack40;
          (**ppcVar1)(uVar6,in_stack_0000fe88,uVar10,0x1,in_stack_0000fe88,uVar10,puStack40);
        }
      }
    }
  }
  return;
}
