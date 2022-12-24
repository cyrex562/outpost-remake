




// WARNING: Unable to use type for symbol uVar2







pub unsafe fn pass1_1038_d242(param_1: *mut Struct57,mut param_2: u16 ) -> *mut Struct57

{
  let mut uVar1: *mut Struct57;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x13e,param_2);
  uVar1 = (param_1 >> 0x10);
  param_1.field0_0x0 = 0xd6ea;
  (param_1 + 0x2) = &u16_1050_1038;
  (param_1 + 0x74) = 0x1;
  return param_1;
}




pub unsafe fn pass1_1038_d276(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xd6ea;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
















pub unsafe fn pass1_1038_d756(param_1: *mut StructD,param_2: *mut Struct57,mut param_3: u16 ) -> *mut Struct57

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
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x11b,param_3);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  (iVar2 + 1).field0_0x0 = 0;
  iVar2[0x1].field1_0x2 = 0;
  iVar2[0x1].field2_0x4 = 0;
  iVar2[0x1].field4_0x8 = 0;
  param_2.field0_0x0 = 0xe0d4;
  iVar2.field1_0x2 = &u16_1050_1038;
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(unaff_BP,0x2b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar3;
  iVar2[0x1].field3_0x6 = (puVar3 >> 0x10);
  ppcVar1 = (*&iVar2[0x1].field2_0x4 + 0x4);
  (**ppcVar1)();
  return param_2;
}




pub unsafe fn pass1_1038_d7d0(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0xe0d4;
  iVar1.address_offset_field_0x2 = &u16_1050_1038;
  if (&iVar1.field_0x90 != 0) {
    pass1_1010_1ea6(_u16_1050_02a0,param_1);
  }
  if (&iVar1.field_0x92 != 0) {
    pass1_1010_1ea6(&iVar1.field_0x92,param_1);
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1.field_0x6);
  fn_ptr_1000_17ce(*&iVar1.field_0x96);
  ui_cleanup_op_1040_782c(param_1);
  return;
}






// WARNING: Removing unreachable block (ram,0x1038dad3)
// WARNING: Removing unreachable block (ram,0x1038daea)
pub unsafe fn pass1_1038_de20(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: i16)

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut local_12: [u8;0x4] = [0;0x4];
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut puStack10: *mut u32;
  let mut uStack6: u16;
  let mut iStack4: i16;

  paVar4 = CONCAT22(in_register_0000000a,param_2);
  iStack4 = 0x644;
  uStack6 = 0;
  match param_6 -0x11c {
  0x0 =>{
    iStack4 = 0x635;
    uStack6 = 0x3a;}
    // break;
  0x1 =>{
    iStack4 = 0x636;
    uStack6 = 0x3b;}
    // break;
  0x2 =>{
    iStack4 = 0x637;
    uStack6 = 0x3c;}
    // break;
  0x4 =>{
    iStack4 = 0x639;
    uStack6 = 0x3e;}
    // break;
  0x5 =>{
    iStack4 = 0x63a;
    uStack6 = 0x3f;}
    // break;
  0x6 =>{
    iStack4 = 0x63b;
    uStack6 = 0x40;}
    // break;
  0x7 =>{
    iStack4 = 0x640;
    uStack6 = 0x45;}
    // break;
  0x9 =>{
    iStack4 = 0x642;
    uStack6 = 0x47;}
    // break;
  0xa =>{
    iStack4 = 0x641;
    uStack6 = 0x46;}
    // break;
  0xb =>{
    iStack4 = 0x63f;
    uStack6 = 0x44;}
  };
  if (iStack4 != 0) {
    uVar6 = 0x1000;
    mem_op_1000_179c(0xb4,paVar4);
    uStack12 = paVar4;
    uVar5 = paVar4 & 0xffff0000 | (uStack12 | param_1);
    uStack14 = param_1;
    if ((uStack12 | param_1) == 0) {
      iVar2 = 0;
      uVar3 = 0;
    }
    else {
      uVar6 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar2 = string_1040_8520(uVar5,CONCAT22(uStack12,param_1),(param_3 + 0x6),0x20000,
                               CONCAT22(iStack4,0x634));
      uVar3 = uVar5;
    }
    puStack10 = CONCAT22(uVar3,iVar2);
    if (uStack6 == 0) {
      ppcVar1 = (*puStack10 + 0x74);
      (**ppcVar1)(uVar6,iVar2,uVar3);
    }
    else {
      pass1_1008_941a(CONCAT22(0x1050,local_12),0x1,uStack6);
      ppcVar1 = (*puStack10 + 0x6c);
      (**ppcVar1)(0x1008,puStack10,(puStack10 >> 0x10),local_12,&DAT_1050_1050);
    }
  }
  return;
}





pub unsafe fn pass1_1038_df5c(mut param_1: u16 ,mut param_2: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  uVar2 = (param_2 >> 0x10);
  uVar1 = param_2;
  pass1_1010_038e(uVar1 + 0x92,1);
  uVar3 = pass1_1038_af40(uVar1,param_1,_PTR_LOOP_1050_5b7c,(uVar1 + 0x8),0x20);
  return uVar3;
}




pub unsafe fn pass1_1038_df86(param_1: *mut StructD,mut param_2: u32)

{
  let mut pcVar1: *mut c_char;
  let mut ppcVar2: *mut *mut code;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut uVar7: u32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut puVar11: *mut u32;
  let mut pcVar12: *mut c_char;
  let mut paVar13: *mut Struct57;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffe6: u16;
  let mut puStack22: *mut u32;

  paVar13 = CONCAT22(in_register_0000000a,param_1);
  puVar11 = mixed_1010_20ba(paVar13,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x2),in_stack_0000fe8e,
                            in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar7 = paVar13 & 0xffff0000 | puVar11 >> 0x10;
  pcVar1 = *(puVar11 + 0x68);
  uVar9 = (param_2 >> 0x10);
  uVar8 = param_2;
  BVar3 = pass1_1010_041a();
  if (BVar3 != 0) {
    pass1_1010_038e(uVar8 + 0x92,1);
    pass1_1038_af40(uVar8,uVar7,_PTR_LOOP_1050_5b7c,(uVar8 + 0x8),0x1e);
    return;
  }
  pcVar12 = load_string_1010_847e(_u16_1050_14cc,0x7d5);
  paVar13 = (uVar7 & 0xffff0000 | pcVar12 >> 0x10);
  uVar4 = pcVar12;
  uVar10 = 0x1000;
  mem_op_1000_179c(0xb4,paVar13);
  uVar5 = paVar13 | uVar4;
  if (uVar5 == 0) {
    uVar9 = 0;
    uVar6 = 0;
  }
  else {
    uVar10 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar13 = pass1_1040_8478(uVar5,CONCAT22(paVar13,uVar4),0x20,pcVar1,pcVar12,
                              (uVar8 + 0x6));
    uVar6 = (paVar13 >> 0x10);
    uVar9 = SUB42(paVar13,0x0);
  }
  puStack22 = CONCAT22(uVar6,uVar9);
  ppcVar2 = (*puStack22 + 0x74);
  (**ppcVar2)(uVar10,uVar9,uVar6);
  return;
}
