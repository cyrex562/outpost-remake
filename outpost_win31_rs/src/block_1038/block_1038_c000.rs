








pub unsafe fn send_msg_1038_c228(mut param_1: u32) -> LRESULT

{
  let mut wparam: WPARAM16;
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut LVar3: LRESULT;
  let mut wparam_00: WPARAM16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  LVar3 = SendMessage16(0x0,0x0,0x407,(iVar1 + 0x92));
  wparam = LVar3;
  SendMessage16(0x0,0x0,0x407,(iVar1 + 0x94));
  wparam_00 = 0x408;
  SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x9e),wparam,0x408,(iVar1 + 0x92));
  LVar3 = SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x19e),wparam_00,0x408,(iVar1 + 0x94));
  return LVar3;
}
pub unsafe fn enable_win_1038_c294(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut pcStack12: *mut c_char;

  uVar1 = param_1 + 0x9e;
  pcStack12 = (param_1 & 0xffff0000 | uVar1);
  uVar3 = param_1;
  pass1_1008_e320((param_1 + 0x8e),(param_1 & 0xffff0000 | (param_1 + 0x19e))
                  ,(param_1 & 0xffff0000 | uVar1));
  SetWindowText16(CONCAT22(uVar3,uVar1),(param_1 + 0x9a));
  uVar2 = pass1_1008_e2a4((param_1 + 0x8e),
                          (param_1 & 0xffff0000 | (param_1 + 0x19e)),pcStack12);
  EnableWindow16(uVar2 & 0x1,(param_1 + 0x96));
  EnableWindow16(uVar2 & 0x2,(param_1 + 0x98));
  return;
}



pub unsafe fn set_win_pos_1038_c31a(mut param_1: u32,mut param_2: u16 ,mut param_3: i16) -> BOOL16

{
  let mut iStack10: i16;

  if (param_3 == 1) {
    enable_win_1038_c294(param_1);
  }
  else {
    if (param_3 != 0x7) {
      return 0x0;
    }
    GetWindowRect16(CONCAT22(0x1050,&stack0xfff2),param_2);
    SetWindowPos16(0x2,0x50,iStack10 - param_2,0x0,0x0,0x0,param_2);
  }
  return 0x1;
}
pub unsafe fn send_msg_1038_c374(mut param_1: u32,param_2: *mut u32,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut LVar6: LRESULT;
  let mut lparam: *mut c_char;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar5 = SUB42(s_tile2_bmp_1050_1538,0x0);
  LVar6 = SendMessage16(0x0,0x0,0x40b,param_3);
  uVar3 = LVar6;
  ppcVar2 = (*param_2 + 0x10);
  (**ppcVar2)(s_tile2_bmp_1050_1538,param_2);
  uStack6 = CONCAT22(extraout_DX,uVar3);
  uStack10 = 0;
  loop {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar2 = (*param_2 + 0x4);
    uVar4 = uStack6;
    (**ppcVar2)(uVar5,param_2,uStack10,(uStack10 >> 0x10));
    uVar1 = (param_1 + 0x8e);
    lparam = string_1008_e586(uVar1,(uVar1 >> 0x10),
                                      CONCAT13((extraout_DX_00 >> 0x8),CONCAT12(extraout_DX_00,uVar4))
                                      ,uVar4,extraout_DX_00);
    LVar6 = SendMessage16(lparam,0x0,0x403,param_3);
    uVar5 = 0x1000;
    fn_ptr_1000_17ce;
    if (LVar6 == -1) { break; }
    if (LVar6 == -0x2) {
      return;
    }
    uStack10 += 0x1;
  }
  return;
}





pub unsafe fn pass1_1038_c4a2(param_1: *mut StructD,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x17c,param_6);
  uVar2 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  iVar1[0x1].field2_0x4 = 0;
  iVar1[0x1].field4_0x8 = 0;
  param_2.field0_0x0 = 0xc74c;
  iVar1.field1_0x2 = &u16_1050_1038;
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x3b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar3;
  iVar1[0x1].field1_0x2 = (puVar3 >> 0x10);
  return;
}




pub unsafe fn pass1_1038_c4fe(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xc74c;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
















pub unsafe fn
pass1_1038_c7b8(param_1: *mut u8,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 ) -> *mut Struct57

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfb8,param_6);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  param_2.field0_0x0 = 0xca6c;
  iVar1.field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x5),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return param_2;
}




pub unsafe fn pass1_1038_c80a(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xca6c;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar1





// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2











pub unsafe fn pass1_1038_cad8(param_1: *mut u8,param_2: *mut Struct57,mut param_3: u16 ) -> *mut Struct57

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
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x1cb,param_3);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  param_2.field0_0x0 = 0xcc9a;
  iVar1.field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x2c),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  iVar1.field86_0x74 = 0;
  return param_2;
}




pub unsafe fn pass1_1038_cb30(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xcc9a;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}











pub unsafe fn pass1_1038_cd06(param_1: *mut StructD,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfcc,param_6);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  iVar1[0x1].field2_0x4 = 0;
  param_2.field0_0x0 = 0xcf00;
  iVar1.field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x42),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return;
}




pub unsafe fn pass1_1038_cd5c(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xcf00;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}











pub unsafe fn make_proc_inst_1038_cf6c(param_1: *mut u8,param_2: *mut astruct_831)

{
  let mut iVar1: *mut astruct_831;
  let mut uVar1: u16;
// pub unsafe fn *pvVar1;

  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  param_2 = 0x389a;
  iVar1.field2_0x2 = 0x1008;
  iVar1.field3_0x4 = 0;
  iVar1.field5_0x8 = 0;
  param_2 = 0xd23e;
  iVar1.field2_0x2 = &u16_1050_1038;
  _u16_1050_5bc8 = param_2;
  pvVar1 = MakeProcInstance16(HINSTANCE16_1050_038c,&u16_1038_d116);
  iVar1.field3_0x4 = pvVar1;
  iVar1.field4_0x6 = (pvVar1 >> 0x10);
  pvVar1 = MakeProcInstance16(HINSTANCE16_1050_038c,&PTR_LAB_1038_d08b_1_1038_d01e);
  u16_1050_5bcc = pvVar1;
  u16_1050_5bce = (pvVar1 >> 0x10);
  return;
}




pub unsafe fn free_proc_inst_1038_cfda(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0xd23e;
  iVar1.address_offset_field_0x2 = &u16_1050_1038;
  FreeProcInstance16(&iVar1.hfile_0x4);
  FreeProcInstance16(_u16_1050_5bcc);
  iVar1.hfile_0x4 = 0;
  param_1.address_offset_field_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}
