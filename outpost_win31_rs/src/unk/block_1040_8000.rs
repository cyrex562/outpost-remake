






/*
Unable to decompile 'pass1_1040_805a'
Cause:
Low-level Error: Symbol $$undef00000006 extends beyond the end of the address space
*/










use crate::{draw_ops, resources};


pub fn pass1_1040_83e6(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  ui_cleanup_op_1040_782c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub fn pass1_1040_8478(mut param_1: u16 ,param_2: *mut Struct57,mut param_3: u16 ,param_4: *mut c_char,param_5: *mut c_char,mut param_6: u16 ) -> *mut Struct57

{
  let mut uVar1: u16;
  let mut iVar2: *mut Struct57;
  let mut uVar2: *mut Struct57;

  get_sys_metrics_1040_7728(param_2,0x1,0x0,0xfc3,param_6);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  (iVar2 + 1).field0_0x0 = 0;
  iVar2[0x1].field5_0xa = param_3;
  iVar2[0x1].field6_0xc = 0;
  iVar2[0x1].field_0x24 = 0;
  param_2.field0_0x0 = 0x8ddc;
  iVar2.field1_0x2 = &PTR_LOOP_1050_1040;
  iVar2[0x1].field8_0x10 = 0;
  iVar2[0x1].field10_0x14 = 0x12c;
  uVar1 = str_op_1008_60e8(param_1,param_5);
  iVar2[0x1].field1_0x2 = uVar1;
  iVar2[0x1].field2_0x4 = param_1;
  uVar1 = str_op_1008_60e8(param_1,param_4);
  iVar2[0x1].field3_0x6 = uVar1;
  iVar2[0x1].field4_0x8 = param_1;
  resources::load_icon_1040_8b92(param_2);
  PTR_LOOP_1050_5df8 = null_mut();
  return param_2;
}






pub fn string_1040_8520(mut param_1: u32,param_2: *mut Struct57,mut param_3: u16 ,mut param_4: u32,mut param_5: u32) -> i16

{
  let mut uVar1: u32;
  let mut puVar2: *mut u32;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut pcVar6: *mut c_char;
  let mut uVar7: u16;
  let mut iStack22: i16;
  let mut iStack16: i16;
  let mut puStack14: *mut u16;
  let mut iVar7: *mut Struct57;

  iVar7 = param_2;
  uVar7 = (param_2 >> 0x10);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0xfc3,param_3);
  (iVar7 + 1).field0_0x0 = 0;
  iVar7[0x1].field5_0xa = param_4;
  iVar7[0x1].field6_0xc = 0;
  iVar7[0x1].field_0x24 = 0;
  param_2.field0_0x0 = 0x8ddc;
  iVar7.field1_0x2 = &PTR_LOOP_1050_1040;
  iVar7[0x1].field8_0x10 = 0;
  iVar7[0x1].field10_0x14 = 0x12c;
  puVar2 = &param_5;
  iStack16 = param_4;
  if (param_4 != 0) {
    puVar2 = (&param_5 + 2);
    load_string_1010_84ac(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),param_5);
    iVar7[0x1].field3_0x6 = param_5;
    iVar7[0x1].field4_0x8 = param_1;
    iStack16 = param_4 -0x1;
  }
  puStack14 = CONCAT22(0x1050,puVar2);
  iStack22 = 0;
  while (puVar3 = puStack14, iStack16 != 0) {
    puStack14 = (puStack14 & 0xffff0000 | (puStack14 + 0x2));
    pcVar6 = load_string_1010_847e(_u16_1050_14cc,*puVar3);
    param_1 = param_1 & 0xffff0000 | pcVar6 >> 0x10;
    uVar4 = str_op_1000_3da4(pcVar6);
    iStack22 += uVar4;
    iStack16 = iStack16 -0x1;
  }
  uVar5 = iStack22 + 1;
  mem_op_1000_179c(uVar5,param_1);
  iVar7[0x1].field1_0x2 = uVar5;
  iVar7[0x1].field2_0x4 = param_1;
  puStack14 = CONCAT22(0x1050,&param_5 + 2);
  iStack16 = param_4 -0x1;
  if (param_4 -0x1 != 0) {
    puStack14 = CONCAT22(0x1050,&stack0x0012);
    uVar1 = &iVar7[0x1].field1_0x2;
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,uVar1,(uVar1 >> 0x10)
              );
    iStack16 = param_4 -0x2;
  }
  while (puVar3 = puStack14, iStack16 != 0) {
    puStack14 = (puStack14 & 0xffff0000 | (puStack14 + 0x2));
    pcVar6 = load_string_1010_847e(_u16_1050_14cc,*puVar3);
    pass1_1000_3cea(&iVar7[0x1].field1_0x2,pcVar6);
    iStack16 = iStack16 -0x1;
  }
  resources::load_icon_1040_8b92(param_2);
  PTR_LOOP_1050_5df8 = null_mut();
  return iVar7;
}
pub fn pass1_1040_869a(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x8ddc;
  iVar1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(*&iVar1.field_0x90);
  fn_ptr_1000_17ce(*&iVar1.field_0x94);
  ui_cleanup_op_1040_782c(param_1);
  return;
}


pub fn pass1_1040_8e58(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32) -> *mut u16

{
  pass1_1040_b040(CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),(param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0x8f3c;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  return CONCAT22(param_2,param_1);
}
pub fn pass1_1040_8e82(param_1: *mut StructD)

{
  let mut in_stack_0000ffde: u16;

  param_1.address_offset_field_0x0 = 0x8f3c;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}
