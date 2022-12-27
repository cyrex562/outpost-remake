use crate::gui;
use crate::gui::dialog;
use crate::gui::dialog::dlg_a;

pub fn pass1_1038_8810(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut local_102: [u8;0x100] = [0;0x100];

  uVar2 = (param_1 >> 0x10);
  uVar1 = dlg_a::send_dlg_item_msg_1038_8164(param_1, uVar2, CONCAT22(0x1050, local_102), 0x1856);
  if (uVar1 != 0) {
    pass1_1008_b63a((param_1 + 0x94),CONCAT22(0x1050,local_102));
  }
  return;
}












pub fn pass1_1038_88f2(param_1: *mut Struct57,mut param_2: u16 )

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;

  struct_1040_b082(param_1,CONCAT22(param_2,0x184c));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field3_0x6 = _u16_1050_5a68;
  iVar1[0x1].field5_0xa = 0;
  iVar1[0x1].field6_0xc = 0;
  iVar1[0x1].field7_0xe = 0;
  iVar1[0x1].field8_0x10 = 0;
  param_1.field0_0x0 = 0x8c2e;
  iVar1.field1_0x2 = &u16_1050_1038;
  return;
}




pub fn pass1_1038_893a(param_1: *mut StructD)

{
  let mut uVar1: u16;
  let mut in_stack_0000ffde: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x8c2e;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}


pub fn pass1_1038_8caa(param_1: *mut u8,param_2: *mut Struct57,mut param_3: u16 ) -> *mut Struct57

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar1: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082(param_2,CONCAT22(param_3,0x185a));
  uVar2 = (param_2 >> 0x10);
  iVar1 = param_2;
  iVar1[0x1].field3_0x6 = 0;
  param_2.field0_0x0 = 0x90c8;
  iVar1.field1_0x2 = &u16_1050_1038;
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x3f),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field3_0x6 = puVar3;
  iVar1[0x1].field4_0x8 = (puVar3 >> 0x10);
  return param_2;
}




pub fn pass1_1038_8cf6(param_1: *mut StructD)

{
  let mut uVar1: u16;
  let mut in_stack_0000ffde: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x90c8;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}
