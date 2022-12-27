pub fn struct_1040_a598(param_1: *mut astruct_259)

{
  let mut iVar1: *mut astruct_259;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field0_0x0 = 0;
  iVar1.field1_0x2 = 0;
  iVar1.field2_0x6 = 0;
  iVar1.field3_0xa = 0;
  iVar1.field4_0xc = 0;
  iVar1.field5_0x10 = 0;
  iVar1.field6_0x12 = 0;
  iVar1.field7_0x14 = 0;
  iVar1.field8_0x16 = 0;
  return;
}
pub fn pass1_1040_a5d0(param_1: *mut StructD)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut iVar4: *mut StructD;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar1 = iVar4.address_offset_field_0x2;
  uVar2 = iVar4.hfile_0x4;
  if ((uVar2 | uVar1) != 0) {
    pass1_1000_54e8(0xa582,&PTR_LOOP_1050_1040,(uVar1 - 0x2),0xa,uVar1,uVar2);
    fn_ptr_1000_17ce(CONCAT22(uVar2,uVar1 - 0x2));
  }
  fn_ptr_1000_17ce(*&iVar4.field7_0xc);
  return;
}
pub fn string_1040_a626(mut param_1: u16 ,param_2: *mut u16,param_3: *mut c_char)

{
  let mut uVar1: u16;

  uVar1 = str_op_1008_60e8(param_1,param_3);
  *param_2 = uVar1;
  (param_2 + 0x2) = param_1;
  return;
}
pub fn pass1_1040_a640(param_1: *mut Struct57,mut param_2: u32,mut param_3: u16 )

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;

  struct_1040_b082(param_1,CONCAT22(param_3,0x1f1));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field3_0x6 = param_2;
  iVar1[0x1].field5_0xa = 0;
  iVar1[0x1].field_0x5c = 0;
  param_1.field0_0x0 = 0xac08;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}


pub fn pass1_1040_ac84(param_1: *mut u8,param_2: *mut Struct57,mut param_3: u16 )

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
  struct_1040_b082(param_2,CONCAT22(param_3,0x1f3));
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  iVar1[0x1].field3_0x6 = 0;
  iVar1[0x1].field5_0xa = 0;
  param_2.field0_0x0 = 0xafc4;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  iVar1[0x1].field3_0x6 = _PTR_LOOP_1050_5ef0;
  _PTR_LOOP_1050_5ef0 = 0;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x3d),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field5_0xa = puVar2;
  iVar1[0x1].field6_0xc = (puVar2 >> 0x10);
  return;
}




pub fn pass1_1040_ace8(param_1: *mut StructD)

{
  let mut uVar1: u16;
  let mut in_stack_0000ffde: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xafc4;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}





pub fn msg_box_ui_op_1040_ad66(param_1: *mut c_char,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,0x1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}


pub fn pass1_1040_b040(param_1: *mut Struct57,mut param_2: u32,mut param_3: u16 )

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,(param_2 + 0x12),param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 1).field0_0x0 = 0;
  iVar1[0x1].field1_0x2 = param_2;
  param_1.field0_0x0 = 0xb772;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}
