

pub unsafe fn pass1_1040_205e(param_1: *mut StructD)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: *mut StructD;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  param_1.address_offset_field_0x0 = 0x237e;
  iVar4.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  puVar1 = &iVar4.field_0x8e;
  uVar2 = &iVar4.field_0x90;
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*&iVar4.field_0xa2);
  fn_ptr_1000_17ce(*&iVar4.field_0xa6);
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar4.field_0x6);
  ui_cleanup_op_1040_782c(param_1);
  return;
}













pub unsafe fn pass1_1040_23ea(mut param_1: u16 ,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 ,
                    param_7: *mut *mut u8)

{
  let mut uVar1: u32;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut iVar2: *mut Struct57;
  let mut uVar2: *mut Struct57;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fea2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd0: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x9a,param_3,0xfbd,param_6);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  (iVar2 + 1) = 0;
  iVar2[0x1].field2_0x4 = 0;
  iVar2[0x1].field3_0x6 = 0;
  param_2.field0_0x0 = 0x2956;
  iVar2.field1_0x2 = &PTR_LOOP_1050_1040;
  iVar2.field105_0x8a = 0x26;
  param_7 = CONCAT22((param_7 >> 0x10),0x6);
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,param_7,in_stack_0000fea2,in_stack_0000ffc6,in_stack_0000ffcc,
                           in_stack_0000ffd0);
  (iVar2 + 1).field0_0x0 = puVar3;
  iVar2[0x1].field1_0x2 = (puVar3 >> 0x10);
  uVar1 = (iVar2 + 1);
  iVar2[0x1].field2_0x4 = (uVar1 + 0x28);
  return;
}



















pub unsafe fn pass1_1040_2a22(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;
  let mut in_stack_0000ffd2: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x2e26;
  iVar1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(*&iVar1.field_0x94);
  fn_ptr_1000_17ce(*&iVar1.field_0x98);
  unk_draw_op_1040_b0f8(in_stack_0000ffd2,param_1);
  return;
}









pub unsafe fn pass1_1040_2f06(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x3436;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
