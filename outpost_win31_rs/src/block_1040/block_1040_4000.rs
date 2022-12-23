

pub unsafe fn pass1_1040_4068(param_1: *mut u8,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar2: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;
  let mut ppuVar4: *mut *mut u8;

  paVar1 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfb7,param_6);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  (iVar2 + 1) = 0;
  iVar2[0x1].field2_0x4 = 0;
  iVar2[0x1].field6_0xc = 0;
  param_2.field0_0x0 = 0x4466;
  iVar2.field1_0x2 = &PTR_LOOP_1050_1040;
  iVar2.field87_0x76 = 0x1;
  ppuVar4 = CONCAT22(unaff_BP,0x2);
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,ppuVar4,in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  (iVar2 + 1).field0_0x0 = puVar3;
  iVar2[0x1].field1_0x2 = (puVar3 >> 0x10);
  puVar3 = mixed_1010_20ba((paVar1 & 0xffff0000 | puVar3 >> 0x10),_u16_1050_0ed0,
                           CONCAT22((ppuVar4 >> 0x10),0x29),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field4_0x8 = puVar3;
  iVar2[0x1].field5_0xa = (puVar3 >> 0x10);
  return;
}




pub unsafe fn pass1_1040_40e2(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x4466;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}









// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
pub unsafe fn pass1_1040_44d2(mut param_1: u16 ,param_2: *mut u8,param_3: *mut Struct57,mut param_4: u32,mut param_5: u16 )

{
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: *mut Struct57;
  let mut iVar7: i16;
  let mut uVar8: *mut Struct57;
  let mut uVar9: u16;
  let mut piStack8: *mut i16;
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut paVar6: *mut Struct57;

  struct_1040_b082(param_3,CONCAT22(param_5,0xfa2));
  uVar8 = (param_3 >> 0x10);
  iVar6 = param_3;
  param_3.field0_0x0 = 0x4824;
  iVar6.field1_0x2 = &PTR_LOOP_1050_1040;
  mem_op_1000_179c(0x18,param_2);
  uVar4 = param_2 | param_1;
  paVar6 = (param_2 & 0xffff0000 | uVar4);
  if (uVar4 == 0) {
    iVar6[0x1].field1_0x2 = 0;
  }
  else {
    struct_1040_a598(CONCAT22(param_2,param_1));
    iVar6[0x1].field1_0x2 = param_1;
    iVar6[0x1].field2_0x4 = paVar6;
  }
  *&iVar6[0x1].field1_0x2 = 0x14;
  iVar7 = *&iVar6[0x1].field1_0x2;
  uVar4 = iVar7 * 0xa + 2;
  mem_op_1000_179c(uVar4,paVar6);
  uVar5 = paVar6;
  piStack8 = CONCAT22(uVar5,uVar4);
  if ((uVar5 | uVar4) == 0) {
    uVar3 = &iVar6[0x1].field1_0x2;
    (uVar3 + 0x2) = 0;
  }
  else {
    *piStack8 = iVar7;
    pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iVar7,0xa,uVar4 + 0x2,uVar5);
    uVar3 = &iVar6[0x1].field1_0x2;
    uVar9 = (uVar3 >> 0x10);
    iVar7 = uVar3;
    (iVar7 + 0x2) = uVar4 + 2;
    (iVar7 + 0x4) = uVar5;
  }
  uVar1 = &iVar6[0x1].field1_0x2;
  (uVar1 + 0x6) = param_4;
  uVar2 = &iVar6[0x1].field1_0x2;
  (uVar2 + 0xa) = 0x1;
  uVar3 = &iVar6[0x1].field1_0x2;
  (uVar3 + 0x12) = iVar6.field5_0xa;
  return;
}




pub unsafe fn pass1_1040_4766(param_1: u32)

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = (*param_1 + 0x74);
  (**ppcVar1)();
  return;
}








pub unsafe fn pass1_1040_48a0(param_1: *mut Struct57,param_2: *mut Struct57,mut param_3: u16 ,mut param_4: u32,mut param_5: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut puVar7: *mut u8;
  let mut paVar8: *mut Struct57;
  let mut iVar5: *mut Struct57;
  let mut iVar6: *mut astruct_445;
  let mut unaff_SI: u16;
  let mut uVar10: *mut Struct57;
  let mut uVar11: u16;
  let mut puVar12: *mut u32;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut piStack8: *mut i16;
  let mut paVar9: *mut Struct57;

  struct_1040_b082(param_2,CONCAT22(param_5,0xfa1));
  uVar10 = (param_2 >> 0x10);
  iVar5 = param_2;
  iVar5[0x1].field3_0x6 = 0;
  param_2.field0_0x0 = &PTR_LOOP_1050_4e18;
  iVar5.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar12 = mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(unaff_SI,0x3),in_stack_0000fe9c,
                            in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
  paVar8 = (param_1 & 0xffff0000 | puVar12 >> 0x10);
  uVar5 = puVar12;
  iVar5[0x1].field3_0x6 = uVar5;
  iVar5[0x1].field4_0x8 = (puVar12 >> 0x10);
  mem_op_1000_179c(0x18,paVar8);
  uVar6 = paVar8 | uVar5;
  paVar9 = (paVar8 & 0xffff0000 | uVar6);
  if (uVar6 == 0) {
    iVar5[0x1].field1_0x2 = 0;
  }
  else {
    struct_1040_a598(CONCAT22(paVar8,uVar5));
    iVar5[0x1].field1_0x2 = uVar5;
    iVar5[0x1].field2_0x4 = paVar9;
  }
  *&iVar5[0x1].field1_0x2 = 0x7;
  iVar1 = *&iVar5[0x1].field1_0x2;
  uVar5 = iVar1 * 0xa + 2;
  mem_op_1000_179c(uVar5,paVar9);
  puVar7 = paVar9;
  piStack8 = CONCAT22(puVar7,uVar5);
  if ((puVar7 | uVar5) == 0) {
    uVar4 = &iVar5[0x1].field1_0x2;
    (uVar4 + 0x2) = 0;
  }
  else {
    *piStack8 = iVar1;
    pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iVar1,0xa,uVar5 + 0x2,puVar7);
    uVar4 = &iVar5[0x1].field1_0x2;
    uVar11 = (uVar4 >> 0x10);
    iVar6 = uVar4;
    iVar6.field2_0x2 = uVar5 + 2;
    iVar6.field3_0x4 = puVar7;
  }
  uVar4 = &iVar5[0x1].field1_0x2;
  (uVar4 + 0x6) = param_4;
  uVar4 = &iVar5[0x1].field1_0x2;
  (uVar4 + 0xa) = param_3;
  uVar4 = &iVar5[0x1].field1_0x2;
  (uVar4 + 0x12) = iVar5.field5_0xa;
  uVar2 = iVar5[0x1].field1_0x2;
  uVar3 = iVar5[0x1].field2_0x4;
  pass1_1010_debe(&iVar5[0x1].field3_0x6,(uVar2 + 0xa),CONCAT22(uVar3,uVar2 + 0x10),
                  CONCAT22(uVar3,uVar2 + 0xc),param_4);
  return;
}






pub unsafe fn pass1_1040_4d7e(param_1: *mut astruct_48)

{
  let mut uVar1: u32;
  let mut piVar2: *mut i16;
  let mut uVar3: u16;
  let mut iStack8: i16;
  let mut puStack6: *mut u32;

  uVar3 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x90);
  puStack6 = (uVar1 + 2);
  iStack8 = 0;
  while ((piVar2 = (param_1 + 0x90), *piVar2 != iStack8 && iStack8 <= *piVar2 &&
         ((puStack6 + 0x4) != 0x1770))) {
    iStack8 += 0x1;
    puStack6 = (puStack6 & 0xffff0000 | (puStack6 + 0xa));
  }
  pass1_1000_3e2c(*puStack6);
  return;
}



pub unsafe fn pass1_1040_4dcc(mut param_1: u16 ,param_2: *mut astruct_48,mut param_3: i16) -> *mut c_char

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut pcVar4: *mut c_char;

  uVar3 = (param_2 >> 0x10);
  uVar2 = (param_2 + 0x90);
  uVar1 = (param_2 + 0x94);
  pcVar4 = string_op_1010_ada6(param_1,uVar1,(uVar1 >> 0x10),param_3,(uVar2 + 0xa));
  return pcVar4;
}



pub unsafe fn pass1_1040_4e94(param_1: *mut Struct57,param_2: i32,mut param_3: u16 )

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;
  let mut uVar2: u16;

  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_3,0xfab));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field3_0x6 = 0;
  iVar1[0x1].field5_0xa = 0;
  iVar1[0x1].field18_0x22 = 0;
  iVar1[0x1].field21_0x26 = 0;
  iVar1[0x1].field_0x28 = 0;
  param_1.field0_0x0 = 0x55a2;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  if (param_2 != 0) {
    uVar2 = (param_2 >> 0x10);
    iVar1[0x1].field18_0x22 = (param_2 + 0x6);
    iVar1[0x1].field21_0x26 = (param_2 + 0x14);
  }
  return;
}
pub unsafe fn pass1_1040_4f0a(param_1: *mut StructD)

{
  let mut in_stack_0000ffde: u16;

  param_1.address_offset_field_0x0 = 0x55a2;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}
