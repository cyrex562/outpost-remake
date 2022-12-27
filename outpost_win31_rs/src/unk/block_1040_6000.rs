

pub fn pass1_1040_6402(param_1: *mut StructD,param_2: *mut Struct57,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut iVar2: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar2: *mut Struct57;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x1850,param_3);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  (iVar2 + 1) = 0;
  iVar2[0x1].field2_0x4 = 0;
  param_2.field0_0x0 = 0x67ba;
  iVar2.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(unaff_BP,0x2b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar3;
  iVar2[0x1].field3_0x6 = (puVar3 >> 0x10);
  ppcVar1 = (*&iVar2[0x1].field2_0x4 + 0x4);
  (**ppcVar1)();
  return;
}




pub fn pass1_1040_6470(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x67ba;
  iVar1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (&iVar1.field_0x92 != 0) {
    pass1_1010_1ea6(&iVar1.field_0x92,param_1);
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1.field_0x6);
  fn_ptr_1000_17ce(*&iVar1.field_0x8e);
  ui_cleanup_op_1040_782c(param_1);
  return;
}


pub fn pass1_1040_6826(param_1: *mut Struct57,mut param_2: u16 )

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;

  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_2,0xfda));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field3_0x6 = 0;
  iVar1[0x1].field5_0xa = 0;
  param_1.field0_0x0 = 0x6f32;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}
pub fn pass1_1040_6862(param_1: *mut StructD)

{
  let mut in_stack_0000ffde: u16;

  param_1.address_offset_field_0x0 = 0x6f32;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}














pub fn pass1_1040_6fb6(param_1: *mut Struct57,mut param_2: u16 )

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;

  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_2,0xfd9));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field3_0x6 = 0;
  iVar1[0x1].field5_0xa = 0;
  param_1.field0_0x0 = 0x76a4;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}
