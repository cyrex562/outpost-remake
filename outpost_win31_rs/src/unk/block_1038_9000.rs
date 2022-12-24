





pub unsafe fn pass1_1038_9144(mut param_1: u16 ,param_2: *mut u16,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar6: *mut Struct57;
  let mut iVar7: i16;
  let mut unaff_SI: u16;
  let mut uVar8: *mut Struct57;
  let mut uVar9: u16;
  let mut puVar10: *mut u32;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut piStack8: *mut i16;
  let mut paVar5: *mut Struct57;

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082(param_2,CONCAT22(param_3,0xfaa));
  uVar8 = (param_2 >> 0x10);
  iVar6 = param_2;
  iVar6[0x1].field3_0x6 = 0;
  iVar6[0x1].field4_0x8 = 0;
  iVar6[0x1].field5_0xa = 0;
  *param_2 = 0x99a2;
  iVar6.field1_0x2 = &u16_1050_1038;
  iVar6.field105_0x8a = 0x27;
  puVar10 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(unaff_SI,0x28),in_stack_0000fe9c,
                            in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
  paVar4 = (paVar4 & 0xffff0000 | puVar10 >> 0x10);
  uVar2 = puVar10;
  iVar6[0x1].field5_0xa = uVar2;
  iVar6[0x1].field6_0xc = (puVar10 >> 0x10);
  mem_op_1000_179c(0x18,paVar4);
  uVar3 = paVar4 | uVar2;
  paVar5 = (paVar4 & 0xffff0000 | uVar3);
  if (uVar3 == 0) {
    iVar6[0x1].field1_0x2 = 0;
  }
  else {
    struct_1040_a598(CONCAT22(paVar4,uVar2));
    iVar6[0x1].field1_0x2 = uVar2;
    iVar6[0x1].field2_0x4 = paVar5;
  }
  *&iVar6[0x1].field1_0x2 = 0x11;
  iVar7 = *&iVar6[0x1].field1_0x2;
  uVar2 = iVar7 * 0xa + 2;
  mem_op_1000_179c(uVar2,paVar5);
  uVar3 = paVar5;
  piStack8 = CONCAT22(uVar3,uVar2);
  if ((uVar3 | uVar2) == 0) {
    uVar1 = &iVar6[0x1].field1_0x2;
    (uVar1 + 0x2) = 0;
  }
  else {
    *piStack8 = iVar7;
    pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iVar7,0xa,uVar2 + 0x2,uVar3);
    uVar1 = &iVar6[0x1].field1_0x2;
    uVar9 = (uVar1 >> 0x10);
    iVar7 = uVar1;
    (iVar7 + 0x2) = uVar2 + 2;
    (iVar7 + 0x4) = uVar3;
  }
  uVar1 = &iVar6[0x1].field1_0x2;
  (uVar1 + 0xa) = 0x18;
  uVar1 = &iVar6[0x1].field1_0x2;
  (uVar1 + 0x12) = iVar6.field5_0xa;
  return;
}


pub unsafe fn pass1_1038_993a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16) -> i16

{
  let mut iStack6: i16;

  iStack6 = 0;
  loop {
    if (0xe < iStack6) {
      return -0x1;
    }
    if ((iStack6 * 0xe + 0x5a70) == param_3) { break; }
    iStack6 += 0x1;
  }
  return iStack6;
}







pub unsafe fn pass1_1038_9a1e(param_1: *mut Struct57,param_2: *mut Struct57,mut param_3: u16 ,mut param_4: u32) -> *mut u16

{
  pass1_1040_b040(CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),(param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0x9af6;
  param_1.field1_0x2 = &u16_1050_1038;
  return CONCAT22(param_2,param_1);
}
pub unsafe fn pass1_1038_9a48(param_1: *mut StructD)

{
  let mut in_stack_0000ffde: u16;

  param_1.address_offset_field_0x0 = 0x9af6;
  (param_1 + 0x2) = &u16_1050_1038;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}





pub unsafe fn pass1_1038_9b72(param_1: *mut Struct57,param_2: *mut Struct57,mut param_3: u16 ,mut param_4: u32) -> u32

{
  let mut iStack4: i16;

  pass1_1040_b040(CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),(param_4 >> 0x10));
  param_1[0x2].field6_0xc = 0;
  CONCAT22(param_2,param_1) = 0x9efa;
  param_1.field1_0x2 = &u16_1050_1038;
  iStack4 = 0;
  loop {
    (&param_1[0x1].field3_0x6)[iStack4] = 0;
    iStack4 += 0x1;
    if iStack4 >= 0x4a {break;}
  }
  return CONCAT22(param_2,param_1);
}















pub unsafe fn pass1_1038_9f76(param_1: *mut Struct57,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ) -> *mut Struct57

{
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfba,param_5);
  param_1.field0_0x0 = 0xa0b6;
  (param_1 + 0x2) = &u16_1050_1038;
  return param_1;
}




pub unsafe fn pass1_1038_9fa4(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xa0b6;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
