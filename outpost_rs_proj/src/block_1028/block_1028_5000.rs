
StructD * pass1_1028_504a(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_50d8(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x5280;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_50fa(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x5280;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_5124(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_5128(undefined1 param_1,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  let mut unaff_SI: u16;
  let mut in_stack_0000fd42: u16;
  let mut in_stack_0000fe66: u16;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000fe70: u16;
  let mut piVar4: *mut i16;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut uVar7: u16;
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
  u32 *puStack6;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  pass1_1028_bd38(param_2,param_3);
  puStack6 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fd42,
                             in_stack_0000fe66,in_stack_0000fe6c,in_stack_0000fe70);
  uVar2 = ((u32)puStack6 >> 0x10);
  uStack10 = (u32)((int)puStack6 + 0x20);
  puVar6 = &local_c;
  uVar7 = SUB42(&DAT_1050_1050,0x0);
  piVar1 = &local_e;
  uVar5 = SUB42(&DAT_1050_1050,0x0);
  piVar4 = piVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack10);
  piStack18 = piVar1;
  uStack16 = uVar2;
  pass1_1030_5b1c(CONCAT22(uVar2,piVar1),(u16 *)CONCAT22(uVar5,piVar4),(u16 *)CONCAT22(uVar7,puVar6));
  pass1_1028_b58e(param_3);
  uStack22 = CONCAT22(uVar2,piVar1);
  local_1c = (u32)(piVar1 + 0x6);
  iStack24 = piVar1[0x8];
  pass1_1028_c8ee(param_3,0x1,(u16 *)CONCAT22(0x1050,&local_1c));
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_1c),(u16 *)CONCAT22(0x1050,&local_22),
                  (u16 *)CONCAT22(0x1050,local_20),(u16 *)CONCAT22(0x1050,local_1e));
  if (local_e < local_22) {
    pass1_1030_5b3e(CONCAT22(uStack16,piStack18),local_22,local_c);
    pass1_1030_5b1c(CONCAT22(uStack16,piStack18),(u16 *)CONCAT22(0x1050,&local_e),(u16 *)CONCAT22(0x1050,&local_c)
                   );
  }
  uStack38 = (u32)((int)uStack22 + 0x2e);
  uStack42 = (u32)((int)uStack38 + 0x4);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_14e),0x0,0x0,0x6f,&local_1c,&DAT_1050_1050,uStack42,
                      uStack10);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_14e));
  pass1_1028_ccd0(param_3,(u16 *)CONCAT22(0x1050,&local_1c));
  return;
}



StructD * pass1_1028_525a(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_52e8(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x535e;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_530a(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x535e;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_5334(void)

{
  return;
}



StructD * FUN_1028_5338(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



pub fn pass1_1028_533c(void) -> u32

{
  let mut unaff_BP: i16;

  pass1_1028_b418((u16*)(unaff_BP + 0x6));
  if ((*(unaff_BP + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(char **)(unaff_BP + 0x6));
  }
  return CONCAT22((unaff_BP + 0x8),(unaff_BP + 0x6));
}



u16 * struct_1028_53c6(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x54bc;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_53e8(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x54bc;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn pass1_1028_5412(param_1: *mut astruct_15)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut iVar4: i16;

  uVar2 = ((u32)param_1 >> 0x10);
  if (((int)param_1 + 0x12) != 0x6) {
    return;
  }
  uVar3 = pass1_1028_b4f2(param_1);
  if (*(i32 *)((int)uVar3 + 0x200) != 0x8000002) {
    if (*(i32 *)((int)param_1 + 0x1c) == 0x8000002) {
      iVar4 = 0x6;
      goto code_r0x1028548e;
    }
    ppcVar1 = (code **)((int)(u32)param_1 + 0x64);
    iVar4 = (**ppcVar1)();
    if (iVar4 == 0x0) {
      return;
    }
    pass1_1028_c0f0(iVar4,param_1,0x1);
    if (iVar4 == 0x0) {
      iVar4 = 0x6;
      goto code_r0x1028548e;
    }
    pass1_1028_c952(param_1);
    pass1_1028_c00a(iVar4,param_1,0x1);
  }
  iVar4 = 0x5;
code_r0x1028548e:
  pass1_1028_bdac(param_1,iVar4);
  return;
}



StructD * pass1_1028_5496(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1028_5524(u8 *param_1,param_2: *mut u16)

{
  let mut in_register_0000000a: u16;

  struct_1028_0068((astruct_57 *)CONCAT22(in_register_0000000a,param_1),(astruct_180 *)param_2);
  *param_2 = 0x55c8;
  ((int)param_2 + 0x2) = 0x1028;
  return param_2;
}



u16 * pass1_1028_5546(StructD *param_1,param_2: *mut astruct_12,mut param_3: u16 ,mut param_4: u32)

{
  pass1_1028_00cc(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x55c8;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn pass1_1028_5570(param_1: *mut astruct_15)

{
  astruct_57 *in_EDX;
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;

  pass1_1028_0550(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  if (((int)param_1 + 0x12) == 0x5) {
    uVar3 = 0x0;
    iVar4 = 0x4;
    uVar2 = 0x1;
    uVar1 = pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
    pass1_1030_7c50(uVar1,in_EDX,(astruct_305 *)CONCAT22((int)in_EDX,uVar1),CONCAT22(uVar3,uVar2),iVar4);
  }
  return;
}



StructD * pass1_1028_55a2(StructD *param_1,param_2: u8)

{
  pass1_1028_0138(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_180 * struct_1028_5630(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x56ac;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_5652(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x56ac;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



u16 FUN_1028_567c(void)

{
  return 0x0;
}
pub fn FUN_1028_5682(void)

{
  return;
}



StructD * FUN_1028_5686(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}
pub fn FUN_1028_5714(void)

{
  return;
}



StructD * FUN_1028_5718(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



u16 * struct_1028_57a6(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x581c;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_57c8(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x581c;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_57f2(void)

{
  return;
}



StructD * FUN_1028_57f6(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



pub fn pass1_1028_57fa(void) -> u32

{
  let mut unaff_BP: i16;

  pass1_1028_b418((u16*)(unaff_BP + 0x6));
  if ((*(unaff_BP + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(char **)(unaff_BP + 0x6));
  }
  return CONCAT22((unaff_BP + 0x8),(unaff_BP + 0x6));
}



u16 * pass1_1028_5884(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x58fe;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_58a6(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x58fe;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_58d0(void)

{
  return;
}
pub fn FUN_1028_58d4(void)

{
  return;
}



StructD * FUN_1028_58d8(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



pub fn pass1_1028_58dc(void) -> u32

{
  let mut unaff_BP: i16;

  pass1_1028_b418((u16*)(unaff_BP + 0x6));
  if ((*(unaff_BP + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(char **)(unaff_BP + 0x6));
  }
  return CONCAT22((unaff_BP + 0x8),(unaff_BP + 0x6));
}



u16 * struct_1028_5966(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x59e0
  param_1.field0_0x0 = (int)s_mineToSmelter__no_mines_1050_59df + 0x1;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_5988(StructD *param_1,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = (int)s_mineToSmelter__no_mines_1050_59df + 0x1;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_59b2(void)

{
  return;
}
pub fn FUN_1028_59b6(void)

{
  return;
}



StructD * FUN_1028_59ba(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



pub fn pass1_1028_59be(void) -> u32

{
  let mut unaff_BP: i16;

  pass1_1028_b418((u16*)(unaff_BP + 0x6));
  if ((*(unaff_BP + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(char **)(unaff_BP + 0x6));
  }
  return CONCAT22((unaff_BP + 0x8),(unaff_BP + 0x6));
}



u16 * struct_1028_5a48(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x5bec
  param_1.field0_0x0 = s_thisLo_1050_5bec;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_5a6a(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = s_thisLo_1050_5bec;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_5a94(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_15,u32 *param_4)

{
  code **ppcVar1;
  let mut uVar2: u16;
  u8 *puVar3;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut extraout_DX_00: u16;
  let mut uVar6: u32;
  astruct_670 *paVar7;
  let mut uStack14: u32;
  u8 local_a [0x2];
  let mut uStack8: u16;
  let mut uStack6: u32;

  ppcVar1 = (code **)((int)*param_4 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_1);
  if ((extraout_DX | param_1) == 0x0) {
    return;
  }
  uStack8 = 0x1;
  uVar6 = pass1_1030_bcae(local_a,&DAT_1050_1050);
  uVar5 = (uVar6 >> 0x10);
  uStack14 = 0x0;
  while( true ) {
    if (uStack6 <= uStack14) {
      return;
    }
    uVar4 = uStack6;
    pass1_1030_1d58((u32)param_4);
    uVar2 = uVar4;
    uVar6 = (u32)uVar5;
    paVar7 = (astruct_670 *)(uVar4 & 0xffff | (u32)uVar5 << 0x10);
    pass1_1028_b58e(param_3);
    puVar3 = local_a;
    uVar5 = extraout_DX_00;
    pass1_1030_bd74(puVar3,&DAT_1050_1050,CONCAT22(extraout_DX_00,uVar2),paVar7);
    if ((int)puVar3 < 0x5) break;
    uStack14 += 0x1;
  }
  struct_op_1030_73a8((astruct_419 *)(uVar4 & 0xffff | uVar6 << 0x10),puVar3,uVar5);
  return;
}
pub fn pass1_1028_5b42(param_1: *mut astruct_15)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut iVar4: i16;

  uVar2 = ((u32)param_1 >> 0x10);
  if (((int)param_1 + 0x12) != 0x6) {
    return;
  }
  uVar3 = pass1_1028_b4f2(param_1);
  if (*(i32 *)((int)uVar3 + 0x200) != 0x8000002) {
    if (*(i32 *)((int)param_1 + 0x1c) == 0x8000002) {
      iVar4 = 0x6;
      goto code_r0x10285bbe;
    }
    ppcVar1 = (code **)((int)(u32)param_1 + 0x64);
    iVar4 = (**ppcVar1)();
    if (iVar4 == 0x0) {
      return;
    }
    pass1_1028_c0f0(iVar4,param_1,0x2);
    if (iVar4 == 0x0) {
      iVar4 = 0x6;
      goto code_r0x10285bbe;
    }
    pass1_1028_c952(param_1);
    pass1_1028_c00a(iVar4,param_1,0x2);
  }
  iVar4 = 0x5;
code_r0x10285bbe:
  pass1_1028_bdac(param_1,iVar4);
  return;
}



StructD * pass1_1028_5bc6(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_5c54(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x5d8e
  param_1.field0_0x0 = (int)s_static_1050_5d8b + 0x3;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_5c76(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = (int)s_static_1050_5d8b + 0x3;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn FUN_1028_5ca0(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut extraout_DX: u16;
  let mut uVar1: u32;
  u8 local_12e [0x124];
  let mut uStack10: u32;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b58e(param_3);
  uStack10 = (u32)(param_1 + 0x2e);
  iStack6 = param_1;
  uStack4 = extraout_DX;
  uVar1 = pass1_1028_bb24(param_3);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_12e),0x0,0x0,0x65,(u32 *)(iStack6 + 0xc),uStack4,
                      (u32)((int)uStack10 + 0x4),uVar1);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_12e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn FUN_1028_5d0e(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut uVar1: u32;
  let mut extraout_DX: u16;
  char local_11c [0x10e];

  pass1_1028_b58e(param_3);
  uVar1 = (u32)(param_1 + 0x2e);
  pass1_1028_68de((astruct_97 *)CONCAT22(0x1050,local_11c),0x1,(u32)((int)uVar1 + 0x4));
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_11c));
  return;
}



StructD * pass1_1028_5d68(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_180 * set_fn_ptr_1028_5df6(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x5e70
  param_1.field0_0x0 = (int)s_thisHi_1050_5e6f + 0x1;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_5e18(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = (int)s_thisHi_1050_5e6f + 0x1;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_5e42(void)

{
  return;
}
pub fn FUN_1028_5e46(void)

{
  return;
}



StructD * FUN_1028_5e4a(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



astruct_180 * set_fn_ptr_1028_5ed8(param_1: *mut astruct_180)

{
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  ((int)param_1 + 0x20) = 0x0;
  param_1.field0_0x0 = 0x6054;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_5f00(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  ((int)param_2 + 0x20) = 0x0;
  param_2.field0_0x0 = 0x6054;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_5f30(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut BVar1: bool;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar2: u16;
  astruct_15 *pstruct15_5;
  let mut uVar5: u16;
  u32 **ppuVar1;
  let mut iVar3: i16;

  pass1_1028_be9e(param_3);
  uVar5 = ((u32)param_3 >> 0x10);
  pstruct15_5 = (astruct_15 *)param_3;
  if (pstruct15_5.field15_0x12 == 0x5) {
    pstruct15_5.field24_0x20 = (astruct_21 *)((int)s_New_failed_in_Op__Op__DialogCtr_1050_0053 + 0x11);
    pass1_1028_b58e(param_3);
    uVar3 = (u32)(param_1 + 0x2e);
    iVar3 = 0x61;
    uVar2 = extraout_DX;
    pass1_1038_3fb0(uVar3);
    BVar1 = pass1_1030_25b2(uVar3 & 0xffff | (u32)uVar2 << 0x10,iVar3);
    if (BVar1 != 0x0) {
      ppuVar1 = (u32 **)&pstruct15_5.field24_0x20;
      *ppuVar1 = *ppuVar1 + 0x19;
    }
  }
  return;
}



BOOL16 write_to_file_1028_5f82(param_1: *mut astruct_731,u8 *param_2)

{
  let mut BVar1: bool;
  HFILE16 in_stack_0000ffde;
  u16 local_c [0x5];

  BVar1 = write_to_file_1028_b5ec(param_1,(u32)param_2);
  if (BVar1 != 0x0) {
    local_c[0] = ((int)param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffde);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}
pub fn FUN_1028_5fc8(mut param_1: u16 ,param_2: *mut astruct_373,HFILE16 *param_3)

{
  let mut in_AX: i16;
  let mut BVar1: bool;
  u8 *in_DX;

  file_1028_b81a(in_AX,in_DX,param_2,param_3);
  if ((in_AX != 0x0) &&
     (BVar1 = read_file_1008_7dee(param_3,((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x20)),0x2),
     BVar1 == 0x0)) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  return;
}
