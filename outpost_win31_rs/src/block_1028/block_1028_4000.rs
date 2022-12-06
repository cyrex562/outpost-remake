
u16 * struct_1028_406c(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x42ec;
  (param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_408e(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x42ec;
  (param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_40b8(mut param_1: u16 ,param_2: *mut astruct_15)

{
  code **ppcVar1;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut puVar7: *mut u32;
  let mut uStack54: u32;
  u8 local_2c [0x6];
  let mut puStack38: *mut u32;
  let mut uStack34: u32;
  let mut puStack26: *mut u32;
  let mut uStack24: u32;
  let mut local_14: u32;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  pass1_1028_b58e(param_2);
  local_14 = (param_1 + 0xc);
  iStack8 = (param_1 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar7 = CONCAT22(0x1050,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  uStack6 = param_1;
  uStack4 = extraout_DX;
  uVar6 = pass1_1028_bb24(param_2);
  uVar5 = (uVar6 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(puVar2,uVar5,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar2),
                  uVar6 & 0xffff | uVar5 << 0x10,puVar7);
  uStack34 = *puVar2;
  uVar5 = (puVar2 + 0x2);
  uStack54._3_1_ = (uStack34 >> 0x18);
  uVar3 = uStack54._3_1_;
  uStack24 = uStack34;
  if (uStack54._3_1_ != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack34 & 0xffff | uVar5 << 0x10);
    uStack54 = (astruct_419 *)CONCAT22(uVar5,uVar3);
    uVar4 = pass1_1030_6fa0(CONCAT22(uVar5,uVar3));
    if (uVar4 == 0x64) {
      puStack38 = struct_op_1030_73a8(uStack54,0x64,uVar5);
      ppcVar1 = (code **)(*puStack38 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b514(param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_4194(param_1: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut in_EDX: u32;
  let mut uVar2: u16;
  let mut uVar3: u32;
  astruct_27 *paVar4;
  let mut in_stack_0000fe92: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffea: u16;

  uVar2 = (in_EDX >> 0x10);
  pass1_1028_be9e(param_1);
  uVar3 = pass1_1028_b4f2(param_1);
  uVar1 = (uVar3 >> 0x10);
  if ((*(i32 *)(uVar3 + 0x200) != 0x8000002) && ((param_1 + 0x12) == 0x5)) {
    paVar4 = (astruct_27 *)
             mixed_1010_20ba((astruct_57 *)CONCAT22(uVar2,uVar1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000ffea,0x2b),in_stack_0000fe92,in_stack_0000ffb6,
                             in_stack_0000ffbc,in_stack_0000ffc0);
    pass1_1010_043a(paVar4,*(i32 *)(uVar3 + 0x4),0xe);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_41ea(mut param_1: i16,param_2: *mut astruct_15)

{
  code **ppcVar1;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut puVar7: *mut u32;
  let mut uStack54: u32;
  u8 local_2c [0x6];
  let mut puStack38: *mut u32;
  let mut uStack34: u32;
  let mut puStack26: *mut u32;
  let mut uStack24: u32;
  let mut local_14: u32;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b514(param_2);
  pass1_1028_b58e(param_2);
  local_14 = (param_1 + 0xc);
  iStack8 = (param_1 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar7 = CONCAT22(0x1050,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  iStack6 = param_1;
  uStack4 = extraout_DX;
  uVar6 = pass1_1028_bb24(param_2);
  uVar5 = (uVar6 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(puVar2,uVar5,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar2),
                  uVar6 & 0xffff | uVar5 << 0x10,puVar7);
  uStack34 = *puVar2;
  uVar5 = (puVar2 + 0x2);
  uStack54._3_1_ = (uStack34 >> 0x18);
  uVar3 = uStack54._3_1_;
  if (uStack54._3_1_ != 0x0) {
    uStack24 = uStack34;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack34 & 0xffff | uVar5 << 0x10);
    uStack54 = (astruct_419 *)CONCAT22(uVar5,uVar3);
    uVar4 = pass1_1030_6fa0(CONCAT22(uVar5,uVar3));
    if (uVar4 == 0x64) {
      puStack38 = struct_op_1030_73a8(uStack54,0x64,uVar5);
      ppcVar1 = (code **)(*puStack38 + 0x24);
      (**ppcVar1)();
    }
  }
  return;
}
pub fn FUN_1028_42c2()

{
  return;
}



StructD * FUN_1028_42c6(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



u16 * struct_1028_4354(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x446a;
  (param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_4376(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x446a;
  (param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



u16 pass1_1028_43a0(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if (((param_1 + 0x12) != 0x6) && ((param_1 + 0x12) != 0x5)) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_43c2(u8 *param_1,mut param_2: i16,mut param_3: u16 ,mut param_4: i16)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fea2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000fffa: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1028_bd38(param_1,(astruct_15 *)CONCAT22(param_3,param_2));
  if (param_4 == 0x0) {
    puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fffa,0x8),in_stack_0000fea2,
                             in_stack_0000ffc6,in_stack_0000ffcc,in_stack_0000ffd0);
    pass1_1010_988c(puVar2,(param_2 + 0xc));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_43f6(u8 *param_1,param_2: *mut astruct_15,mut param_3: i16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut uVar5: u16;

  uVar1 = 0x83;
  puVar4 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,(u8 **)0x830009,
                           in_stack_0000fe9a,in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  uVar3 = (puVar4 >> 0x10);
  uVar1 = pass1_1010_65d0(puVar4,uVar1);
  if (0x0 < uVar1) {
    uVar2 = pass1_1028_b58e(param_2);
    if (param_3 == 0x0) {
      uVar5 = 0x0;
    }
    else {
      uVar5 = 0x3e8;
    }
    pass1_1030_7d1c(uVar2,uVar3,(astruct_397 *)CONCAT22(uVar3,uVar2),uVar5,0x230000);
  }
  return;
}



StructD * pass1_1028_4444(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_44d2(param_1: *mut astruct_180)

{
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = 0x0;
  param_1.field0_0x0 = 0x4836;
  (param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_44fe(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  (param_2 + 0x20) = 0x0;
  param_2.field0_0x0 = 0x4836;
  (param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn pass1_1028_4530(param_1: *mut u16)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x4836;
  (iVar4 + 0x2) = 0x1028;
  puVar1 = (iVar4 + 0x20);
  uVar2 = (iVar4 + 0x22);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1028_b418(param_1);
  return;
}
pub fn pass1_1028_456e(mut param_1: u16 ,param_2: *mut astruct_15,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  pass1_1028_b46e(param_1,param_2,param_3);
  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  puVar1 = (iVar4 + 0x20);
  uVar2 = (iVar4 + 0x22);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0x20) = 0x0;
  return;
}
pub fn pass1_1028_45b0(param_1: *mut astruct_15)

{
  let mut in_EDX: *mut Struct57;
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;

  pass1_1028_be9e(param_1);
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x12) == 0x5) {
    uVar3 = 0x0;
    iVar4 = 0x4;
    uVar2 = 0x2;
    uVar1 = pass1_1028_b58e((astruct_15 *)(param_1 & 0xffff | uVar1 << 0x10));
    pass1_1030_7c50(uVar1,in_EDX,(astruct_305 *)CONCAT22(in_EDX,uVar1),CONCAT22(uVar3,uVar2),iVar4);
  }
  return;
}



pub fn pass1_1028_45e2(mut param_1: u16 ,mut param_2: i16,mut param_3: u32) -> u32

{
  pass1_1028_478a(param_1,(astruct_15 *)param_3);
  return CONCAT22(-(0x3e8 < param_1) - param_2,0x3e8 - param_1);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_45fe(mut param_1: i16,param_2: *mut astruct_15,mut param_3: u32)

{
  let mut uVar1: u32;
  code **ppcVar2;
  astruct_21 *paVar3;
  let mut uVar4: u16;
  let mut paVar5: *mut Struct57;
  astruct_15 *pstruct15_1;
  astruct_314 *iVar5;
  astruct_15 *pstruct15_2;
  let mut uVar6: u16;
  astruct_99 *paStack44;
  let mut local_28: i32;
  astruct_21 *paStack34;
  let mut uStack32: u16;
  astruct_99 *paStack30;
  i16 local_1a [0x4];
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut puStack10: *mut u32;
  let mut uStack6: u32;
  astruct_313 *uVar2;

  pass1_1028_b58e(param_2);
  uStack6 = CONCAT22(param_3,param_1);
  puStack10 = (param_1 + 0x22);
  pstruct15_2 = (astruct_15 *)(param_2 >> 0x10);
  pstruct15_1 = (astruct_15 *)param_2;
  paVar3 = pstruct15_1.field24_0x20;
  uStack32 = pstruct15_1.field25_0x22;
  paVar5 = (astruct_57 *)(param_3 & 0xffff0000 | uStack32);
  paStack30 = CONCAT22(uStack32,paVar3);
  paStack34 = paVar3;
  if ((uStack32 | paVar3) != 0x0) {
    ppcVar2 = (code **)paVar3;
    (**ppcVar2)();
  }
  mem_op_1000_179c(0xc,paVar5);
  uStack32 = paVar5;
  uVar4 = uStack32 | paVar3;
  paStack34 = paVar3;
  if (uVar4 == 0x0) {
    paVar3 = NULL;
    uVar4 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(uStack32,paVar3));
  }
  pstruct15_1.field24_0x20 = paVar3;
  pstruct15_1.field25_0x22 = uVar4;
  if (puStack10 != NULL) {
    uStack14 = (puStack10 + 0x4);
    for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
      pass1_1020_bb16(puStack10,CONCAT22(0x1050,&local_28),CONCAT22(0x1050,local_1a),uStack18);
      if ((local_28 != 0x0) && (local_1a[0] != 0x0)) {
        paStack30 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar4 = (paStack30 >> 0x10);
        uVar2 = (astruct_313 *)paStack30;
        if ((uVar4 | uVar2) == 0x0) {
          paStack44 = NULL;
        }
        else {
          paStack30.field0_0x0 = 0x389a;
          uVar2.field2_0x2 = 0x1008;
          uVar2.field3_0x4 = 0x0;
          uVar2.field4_0x6 = 0x0;
          uVar2.field5_0x8 = 0x0;
          uVar2.field6_0xa = 0x0;
          uVar2.field7_0xc = 0x0;
          paStack30.field0_0x0 = 0x56ce;
          uVar2.field2_0x2 = 0x1018;
          paStack44 = paStack30;
        }
        uVar6 = (paStack44 >> 0x10);
        iVar5 = (astruct_314 *)paStack44;
        iVar5.field4_0x4 = local_1a[0];
        iVar5.field9_0xa = local_28;
        iVar5.field10_0xc = local_28;
        uVar1 = &pstruct15_1.field24_0x20;
        ppcVar2 = (code **)(*&pstruct15_1.field24_0x20 + 0x8);
        (**ppcVar2)(0x0,uVar1,(uVar1 >> 0x10),iVar5,uVar6);
      }
    }
  }
  return;
}



u16 pass1_1028_4768(mut param_1: u16 ,mut param_2: i16,mut param_3: u32)

{
  pass1_1028_478a(param_1,(astruct_15 *)param_3);
  if ((param_2 == 0x0) && (param_1 < 0x3e8)) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1028_478a(mut param_1: i16,param_2: *mut astruct_15)

{
  let mut extraout_DX: u16;
  let mut local_1e: i32;
  i16 local_1a [0x4];
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut lStack14: i32;
  let mut puStack10: *mut u32;
  let mut uStack6: u32;

  pass1_1028_b58e(param_2);
  uStack6 = CONCAT22(extraout_DX,param_1);
  puStack10 = (param_1 + 0x22);
  lStack14 = 0x0;
  if (((param_1 + 0x24) | puStack10) == 0x0) {
    return;
  }
  uStack16 = (puStack10 + 0x4);
  for (uStack18 = 0x0; uStack18 < uStack16; uStack18 += 0x1) {
    pass1_1020_bb16(puStack10,CONCAT22(0x1050,&local_1e),CONCAT22(0x1050,local_1a),uStack18);
    if (0x0 < local_1a[0]) {
      lStack14 += local_1e;
    }
  }
  return;
}



StructD * pass1_1028_4810(StructD *param_1,param_2: u8)

{
  pass1_1028_4530(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_489e(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = &PTR_LOOP_1050_4942;
  (param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_48c0(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = &PTR_LOOP_1050_4942;
  (param_2 + 0x2) = 0x1028;
  (param_2 + 0xe) = (param_2 + 0xc);
  (param_2 + 0x10) = (param_2 + 0xc);
  return &param_2.field0_0x0;
}
pub fn pass1_1028_48fa(u32 *param_1)

{
  pass1_1028_bdac((astruct_15 *)param_1,0x0);
  return;
}
pub fn FUN_1028_490c()

{
  return;
}
pub fn FUN_1028_4910()

{
  return;
}
pub fn FUN_1028_4914()

{
  return;
}
pub fn FUN_1028_4918()

{
  return;
}



StructD * FUN_1028_491c(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



pub fn pass1_1028_4920() -> u32

{
  let mut unaff_BP: i16;

  pass1_1028_b418((u16*)(unaff_BP + 0x6));
  if ((*(unaff_BP + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(unaff_BP + 0x6));
  }
  return CONCAT22((unaff_BP + 0x8),(unaff_BP + 0x6));
}



u16 * struct_1028_49aa(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x4b1c;
  (param_1 + 0x2) = 0x1028;
  pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x20)),NULL,0xa);
  return &param_1.field0_0x0;
}



pub fn pass1_1028_49de(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> u32

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x4b1c;
  (param_2 + 0x2) = 0x1028;
  pass1_1000_4906((param_2 & 0xffff0000 | (param_2 + 0x20)),NULL,0xa);
  return param_2;
}
pub fn pass1_1028_4a1a(mut param_1: u32,mut param_2: u32)

{
  let mut BVar1: bool;
  HFILE16 in_stack_0000ffe8;

  BVar1 = write_to_file_1028_b5ec((astruct_731 *)param_1,param_2);
  if ((BVar1 != 0x0) &&
     (BVar1 = write_to_file_1008_7e1c
                        (param_2,param_1 & 0xffff0000 | (param_1 + 0x20),0xa,in_stack_0000ffe8
                        ), BVar1 == 0x0)) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  return;
}
pub fn pass1_1028_4a5a(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  let mut BVar1: bool;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if ((param_1 != 0x0) &&
     (BVar1 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | (param_3 + 0x20)),0xa),
     BVar1 == 0x0)) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  return;
}



u16 pass1_1028_4a9a(mut param_1: u32,mut param_2: i16)

{
  return (param_1 + 0x20 + param_2 * 0x2);
}
pub fn pass1_1028_4ab2(mut param_1: u32,mut param_2: u16 ,mut param_3: i16)

{
  (param_1 + param_3 * 0x2 + 0x20) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_4aca(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  StructD *pSVar3;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000fff6: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if ((param_2 + 0x20) != 0x0) {
    puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff6,0x2f),in_stack_0000fe9e,
                             in_stack_0000ffc2,in_stack_0000ffc8,in_stack_0000ffcc);
    pSVar3 = (paVar2 & 0xffff0000 | puVar4 >> 0x10);
    uVar1 = SUB42(puVar4,0x0);
    pass1_1010_ed3e(puVar4);
    pass1_1030_2a2c(pSVar3,(astruct_678 *)CONCAT22(pSVar3,uVar1));
  }
  return;
}



StructD * pass1_1028_4af6(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_4b84(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x5070
  param_1.field0_0x0 = s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 0x1;
  (param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_4ba6(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 0x1;
  (param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



u16 pass1_1028_4bd0(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if (((param_1 + 0x12) != 0x6) && ((param_1 + 0x12) != 0x5)) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_4bf2(mut param_1: i16,param_2: *mut astruct_15,mut param_3: u32)

{
  code **ppcVar1;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut puVar6: *mut u32;
  let mut uStack54: u32;
  u8 local_2c [0x6];
  let mut puStack38: *mut u32;
  let mut uStack34: u32;
  let mut puStack26: *mut u32;
  let mut uStack24: u32;
  let mut local_14: u32;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b58e(param_2);
  local_14 = (param_1 + 0xc);
  iStack8 = (param_1 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar6 = CONCAT22(0x1050,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  iStack6 = param_1;
  uStack4 = extraout_DX;
  uVar5 = pass1_1028_bb24(param_2);
  uVar4 = (uVar5 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(puVar2,uVar4,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar2),
                  uVar5 & 0xffff | uVar4 << 0x10,puVar6);
  uStack34 = *puVar2;
  uVar4 = (puVar2 + 0x2);
  uStack54._3_1_ = (uStack34 >> 0x18);
  uVar3 = uStack54._3_1_;
  uStack24 = uStack34;
  if (uStack54._3_1_ != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack34 & 0xffff | uVar4 << 0x10);
    uStack54 = (astruct_419 *)CONCAT22(uVar4,uVar3);
    uVar3 = pass1_1030_6fa0(CONCAT22(uVar4,uVar3));
    if ((uVar3 == 0x62) || (uVar3 == 0x63)) {
      puStack38 = struct_op_1030_73a8(uStack54,uVar3,uVar4);
      uVar3 = puStack38;
      ppcVar1 = (code **)(*puStack38 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b46e(uVar3,param_2,param_3);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_4cd6(mut param_1: i16,param_2: *mut astruct_15)

{
  code **ppcVar1;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut puVar7: *mut u32;
  let mut uStack54: u32;
  u8 local_2c [0x6];
  let mut puStack38: *mut u32;
  let mut uStack34: u32;
  let mut puStack26: *mut u32;
  let mut uStack24: u32;
  let mut local_14: u32;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b514(param_2);
  pass1_1028_b58e(param_2);
  local_14 = (param_1 + 0xc);
  iStack8 = (param_1 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar7 = CONCAT22(0x1050,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  iStack6 = param_1;
  uStack4 = extraout_DX;
  uVar6 = pass1_1028_bb24(param_2);
  uVar5 = (uVar6 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(puVar2,uVar5,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar2),
                  uVar6 & 0xffff | uVar5 << 0x10,puVar7);
  uStack34 = *puVar2;
  uVar5 = (puVar2 + 0x2);
  uStack54._3_1_ = (uStack34 >> 0x18);
  uVar3 = uStack54._3_1_;
  if (uStack54._3_1_ != 0x0) {
    uStack24 = uStack34;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack34 & 0xffff | uVar5 << 0x10);
    uStack54 = (astruct_419 *)CONCAT22(uVar5,uVar3);
    uVar4 = pass1_1030_6fa0(CONCAT22(uVar5,uVar3));
    if ((uVar4 == 0x62) || (uVar4 == 0x63)) {
      puStack38 = struct_op_1030_73a8(uStack54,uVar4,uVar5);
      ppcVar1 = (code **)(*puStack38 + 0x24);
      (**ppcVar1)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_4db2(param_1: u8,u8 *param_2,param_3: *mut astruct_15,mut param_4: i16)

{
  let mut BVar1: bool;
  let mut piVar2: *mut i16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut unaff_SI: u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fd42: u16;
  let mut in_stack_0000fe66: u16;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000fe70: u16;
  let mut piVar6: *mut i16;
  let mut uVar7: u16;
  let mut puVar8: *mut u16;
  let mut uVar9: u16;
  u8 local_14e [0x124];
  let mut uStack42: u32;
  let mut uStack38: u32;
  let mut local_22: i16;
  u8 local_20 [0x2];
  u8 local_1e [0x2];
  let mut local_1c: u32;
  let mut iStack24: i16;
  let mut uStack22: u32;
  let mut piStack18: *mut i16;
  let mut uStack16: u16;
  let mut local_e: i16;
  let mut local_c: u16;
  let mut uStack10: u32;
  let mut puStack6: *mut u32;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  BVar1 = pass1_1008_c6ae(_u16_1050_06e0,(param_3 + 0xc),0x29);
  if (BVar1 != 0x0) {
    pass1_1028_bd38(paVar4,param_3);
    if ((param_4 == 0x0) && ((param_3 + 0xc) == 0x13)) {
      puVar5 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x8),in_stack_0000fd42,
                               in_stack_0000fe66,in_stack_0000fe6c,in_stack_0000fe70);
      paVar4 = (astruct_57 *)(paVar4 & 0xffff0000 | puVar5 >> 0x10);
      pass1_1010_988c(puVar5,(param_3 + 0xc));
    }
    puStack6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fd42,
                               in_stack_0000fe66,in_stack_0000fe6c,in_stack_0000fe70);
    uVar3 = (puStack6 >> 0x10);
    uStack10 = (puStack6 + 0x20);
    puVar8 = &local_c;
    uVar9 = SUB42(&DAT_1050_1050,0x0);
    piVar2 = &local_e;
    uVar7 = SUB42(&DAT_1050_1050,0x0);
    piVar6 = piVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack10);
    piStack18 = piVar2;
    uStack16 = uVar3;
    pass1_1030_5b1c(CONCAT22(uVar3,piVar2),CONCAT22(uVar7,piVar6),CONCAT22(uVar9,puVar8));
    pass1_1028_b58e(param_3);
    uStack22 = CONCAT22(uVar3,piVar2);
    local_1c = (piVar2 + 0x6);
    iStack24 = piVar2[0x8];
    pass1_1028_c8ee(param_3,0x1,CONCAT22(0x1050,&local_1c));
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_1c),CONCAT22(0x1050,&local_22),
                    CONCAT22(0x1050,local_20),CONCAT22(0x1050,local_1e));
    if (local_e < local_22) {
      pass1_1030_5b3e(CONCAT22(uStack16,piStack18),local_22,local_c);
      pass1_1030_5b1c(CONCAT22(uStack16,piStack18),CONCAT22(0x1050,&local_e),
                      CONCAT22(0x1050,&local_c));
    }
    uStack38 = (uStack22 + 0x2e);
    uStack42 = (uStack38 + 0x4);
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_14e),0x0,0x0,0x62,&local_1c,&DAT_1050_1050,uStack42,
                        uStack10);
    fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_14e));
    pass1_1028_ccd0(param_3,CONCAT22(0x1050,&local_1c));
  }
  return;
}
pub fn pass1_1028_4f30(param_1: *mut astruct_15,mut param_2: i16)

{
  astruct_397 *paVar1;
  let mut uVar2: u16;

  paVar1 = (astruct_397 *)pass1_1028_b58e(param_1);
  if (param_2 == 0x0) {
    uVar2 = 0x0;
  }
  else {
    uVar2 = 0x3e8;
  }
  pass1_1030_7d1c(paVar1,(paVar1 >> 0x10),paVar1,uVar2,0x230000);
  return;
}



u16 pass1_1028_4f62(param_1: *mut astruct_15)

{
  let mut uVar1: u32;

  uVar1 = pass1_1028_b58e(param_1);
  if ((uVar1 + 0x10) == 0x0) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_4faa(param_1: *mut astruct_15,mut param_2: u16 ) -> u32

{
  let mut uVar1: u16;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut uStack6: u32;

  uVar1 = pass1_1028_4f62(param_1);
  if (uVar1 != 0x0) {
    return param_1;
  }
  uStack6 = pass1_1028_b58e(param_1);
  local_c = (uStack6 + 0xc);
  uStack8 = 0x0;
  uVar4 = pass1_1028_bb24(param_1);
  uVar3 = (uVar4 >> 0x10);
  puVar2 = &local_c;
  pass1_1030_627e(puVar2,uVar3,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar2),
                  uVar4 & 0xffff | uVar3 << 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar3,puVar2));
  if ((uVar3 | puVar2) == 0x0) {
    return 0x0;
  }
  uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,puVar2),puVar2,uVar3 | puVar2);
  return uVar4;
}
