//
// Created by cyrex on 2022-05-22.
//










pub unsafe fn pass1_1038_e03e(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut iStack6: i16;

  uVar4 = (param_1 >> 0x10);
  uVar2 = pass1_1010_0886();
//   for (iStack6 = 0x1; iStack6 <= uVar2; iStack6 += 1)
  for iStack6 in 1 .. uVar2
  {
    uVar1 = (param_1 + 0x92);
    uVar6 = pass1_1010_08e2(uVar1,(uVar1 >> 0x10),iStack6);
    uVar1 = (param_1 + 0x96);
    uVar5 = (uVar1 >> 0x10);
    iVar3 = uVar1;
    if ((iVar3 + iStack6 * 0x4) != 0) {
      enable_win_1040_9234((iVar3 + iStack6 * 0x4),(uVar6 + 0x6));
    }
  }
  return;
}







pub unsafe fn pass1_1038_e140(param_1: *mut Struct57,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ) -> *mut Struct57

{
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfc2,param_5);
  param_1.field0_0x0 = 0xe264;
  (param_1 + 0x2) = &u16_1050_1038;
  return param_1;
}




pub unsafe fn pass1_1038_e16e(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xe264;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}










pub unsafe fn pass1_1038_e2d0(param_1: *mut Struct57,mut param_2: u16 ) -> *mut Struct57

{
  let mut uVar1: *mut Struct57;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c3,param_2);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x8e) = 0;
  param_1.field0_0x0 = 0xe62e;
  (param_1 + 0x2) = &u16_1050_1038;
  return param_1;
}




pub unsafe fn pass1_1038_e308(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0xe62e;
  iVar1.address_offset_field_0x2 = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1.field_0x6);
  fn_ptr_1000_17ce(*&iVar1.field_0x8e);
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar2










pub unsafe fn pass1_1038_e69a(param_1: *mut StructD,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfcb,param_6);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  iVar1[0x1].field2_0x4 = 0;
  param_2.field0_0x0 = 0xe92e;
  iVar1.field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x43),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return;
}




pub unsafe fn pass1_1038_e6f0(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xe92e;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}









pub unsafe fn
pass1_1038_e99a(param_1: *mut u8,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 ) -> *mut Struct57

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar1: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar2: *mut Struct57;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfb9,param_6);
  uVar2 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  param_2.field0_0x0 = 0xeb32;
  iVar1.field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x30),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return param_2;
}




pub unsafe fn pass1_1038_e9ec(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xeb32;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}






pub unsafe fn pass1_1038_eb9e(param_1: *mut Struct57,mut param_2: u16 ) -> *mut Struct57

{
  let mut uVar1: *mut Struct57;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c7,param_2);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x8e) = 0;
  param_1.field0_0x0 = 0xee6e;
  (param_1 + 0x2) = &u16_1050_1038;
  return param_1;
}




pub unsafe fn pass1_1038_ebd6(param_1: *mut StructD)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0xee6e;
  (iVar1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(iVar1 + 0x6));
  fn_ptr_1000_17ce(*(iVar1 + 0x8e));
  ui_cleanup_op_1040_782c(param_1);
  return;
}












pub unsafe fn pass1_1038_eeda(param_1: *mut StructD,param_2: *mut Struct57,mut param_3: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x166,param_3);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  iVar1[0x1].field2_0x4 = 0;
  iVar1[0x1].field3_0x6 = 0;
  param_2.field0_0x0 = 0x67c;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x9),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  iVar1.field86_0x74 = 0x1;
  return;
}




pub unsafe fn destroy_win_1038_ef3a(param_1: *mut StructD)

{
  let mut uVar2: u32;
  let mut iVar1: *mut StructD;
  let mut uVar1: *mut StructD;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x67c;
  iVar1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (&iVar1.field_0x96 != 0) {
    uVar2 = &iVar1.field_0x96;
    DestroyWindow16((uVar2 + 0x6));
    iVar1.field_0x96 = 0;
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1.field_0x6);
  ui_cleanup_op_1040_782c(param_1);
  return;
}
