










pub fn
pass1_1040_123e(param_1: *mut StructD,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 ) -> *mut Struct57

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfd1,param_6);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  param_2.field0_0x0 = 0x17b0;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x46),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return param_2;
}




pub fn pass1_1040_1290(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x17b0;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}










pub fn pass1_1040_181c(mut param_1: u16 ,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfbb,param_6);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  iVar1[0x1].field2_0x4 = 0;
  iVar1[0x1].field3_0x6 = 0;
  param_2.field0_0x0 = 0x1c48;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x2),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return;
}




pub fn pass1_1040_1876(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x1c48;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}


pub fn pass1_1040_1cb4(param_1: *mut StructD,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar2: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar2: *mut Struct57;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;
  let mut ppuVar3: *mut *mut u8;

  paVar1 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xe8,param_6);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  (iVar2 + 1) = 0;
  iVar2[0x1].field2_0x4 = 0;
  param_2.field0_0x0 = 0x1eee;
  iVar2.field1_0x2 = &PTR_LOOP_1050_1040;
  ppuVar3 = CONCAT22(unaff_BP,0x2);
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,ppuVar3,in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  (iVar2 + 1).field0_0x0 = puVar2;
  iVar2[0x1].field1_0x2 = (puVar2 >> 0x10);
  puVar2 = mixed_1010_20ba((paVar1 & 0xffff0000 | puVar2 >> 0x10),_u16_1050_0ed0,
                           CONCAT22((ppuVar3 >> 0x10),0x37),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar2;
  iVar2[0x1].field3_0x6 = (puVar2 >> 0x10);
  return;
}




pub fn pass1_1040_1d24(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x1eee;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}















pub fn pass1_1040_1f5a(param_1: *mut Struct57,mut param_2: u16 ,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut paVar3: *mut Struct57;
  let mut unaff_DI: u16;
  let mut unaff_CS: u16;
  let mut uVar4: u32;
  let mut puVar5: *mut u32;
  let mut paVar6: *mut Struct27;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa6: u16;
  let mut puVar7: *mut u32;
  let mut puVar8: *mut u32;
  let mut uVar9: u16;
  let mut local_16: u32;
  let mut uStack18: u32;
  let mut iVar6: *mut Struct57;
  let mut uVar6: u16;

  iVar6 = param_1;
  uVar6 = (param_1 >> 0x10);
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfcf,param_2);
  (iVar6 + 1) = 0;
  iVar6[0x1].field10_0x14 = 0;
  iVar6[0x1].field11_0x18 = 0;
  param_1.field0_0x0 = 0x237e;
  iVar6.field1_0x2 = &PTR_LOOP_1050_1040;
  uVar2 = FUN_1010_830a(0x0,param_3,unaff_CS,_u16_1050_14cc,0x1cc);
  (iVar6 + 1).field0_0x0 = uVar2;
  iVar6[0x1].field1_0x2 = param_3;
  uVar4 = pass1_1008_4772(CONCAT22(param_3,(iVar6 + 1).field0_0x0));
  paVar3 = (param_3 & 0xffff0000 | uVar4 >> 0x10);
  uVar9 = (uVar4 >> 0x10);
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,CONCAT22(unaff_DI,0x48),in_stack_0000fe78,
                           in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
  local_16 = CONCAT22((uVar4 + 0x8) + 0xa,0xa);
  uStack18 = CONCAT22(0x1d6,(uVar4 + 0x4) -0xa);
  iVar6[0x1].field2_0x4 = local_16;
  iVar6[0x1].field4_0x8 = uStack18;
  iVar6[0x1].field6_0xc = local_16;
  iVar6[0x1].field8_0x10 = uStack18;
  puVar1 = &iVar6[0x1].field7_0xe;
  *puVar1 = *puVar1 + 0x14;
  puVar8 = &iVar6[0x1].field10_0x14;
  puVar7 = &iVar6[0x1].field11_0x18;
  uVar9 = uVar6;
  paVar6 =
           mixed_1010_20ba((paVar3 & 0xffff0000 | puVar5 >> 0x10),_u16_1050_0ed0,
                           CONCAT22(puVar7,0x2b),in_stack_0000fe70,in_stack_0000ff94,in_stack_0000ff9a,
                           in_stack_0000ff9e);
  pass1_1010_0538(paVar6,CONCAT22(uVar6,puVar7),CONCAT22(uVar9,puVar8));
  return;
}
