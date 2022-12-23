





pub unsafe fn send_msg_1040_323c(mut param_1: u32) -> LRESULT

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
  SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x9a),wparam,0x408,(iVar1 + 0x92));
  LVar3 = SendMessage16(param_1 & 0xffff0000 | (iVar1 + 0x19a),wparam_00,0x408,(iVar1 + 0x94));
  return LVar3;
}
pub unsafe fn enable_win_1040_32a8(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut uStack12: u32;

  uVar1 = param_1 + 0x19a;
  uStack12 = param_1 & 0xffff0000 | uVar1;
  uVar2 = param_1;
  pass1_1018_3a5c((param_1 + 0x96),(param_1 & 0xffff0000 | (param_1 + 0x9a)),
                  (param_1 & 0xffff0000 | uVar1));
  SetWindowText16(CONCAT22(uVar2,uVar1),(param_1 + 0x90));
  BVar1 = string_1018_39d8((param_1 + 0x96),
                           (param_1 & 0xffff0000 | (param_1 + 0x9a)),uStack12);
  EnableWindow16(BVar1 & 0x1,(param_1 + 0x8e));
  return;
}



pub unsafe fn set_win_pos_1040_331a(mut param_1: u32,mut param_2: u16 ,mut param_3: i16) -> BOOL16

{
  let mut iStack10: i16;

  if (param_3 == 1) {
    enable_win_1040_32a8(param_1);
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
pub unsafe fn send_msg_1040_3374(mut param_1: u32,param_2: *mut u32,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar4: u16;
  let mut LVar5: LRESULT;
  let mut lparam: *mut c_char;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar4 = SUB42(s_tile2_bmp_1050_1538,0x0);
  LVar5 = SendMessage16(0x0,0x0,0x40b,param_3);
  uVar2 = LVar5;
  ppcVar1 = (*param_2 + 0x10);
  (**ppcVar1)(s_tile2_bmp_1050_1538,param_2);
  uStack6 = CONCAT22(extraout_DX,uVar2);
  uStack10 = 0;
  loop {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (*param_2 + 0x4);
    uVar3 = uStack6;
    (**ppcVar1)(uVar4,param_2,uStack10,(uStack10 >> 0x10));
    lparam = pass1_1018_3a7a(uVar3,extraout_DX_00,(param_1 + 0x96),
                                     CONCAT13((extraout_DX_00 >> 0x8),CONCAT12(extraout_DX_00,uVar3)))
    ;
    LVar5 = SendMessage16(lparam,0x0,0x403,param_3);
    uVar4 = 0x1000;
    fn_ptr_1000_17ce;
    if (LVar5 == -1) { break; }
    if (LVar5 == -0x2) {
      return;
    }
    uStack10 += 0x1;
  }
  return;
}






pub unsafe fn pass1_1040_34a2(param_1: *mut StructD,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x192,param_6);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  iVar1[0x1].field2_0x4 = 0;
  iVar1[0x1].field3_0x6 = 0;
  iVar1[0x1].field4_0x8 = 0;
  iVar1[0x1].field5_0xa = 0;
  param_2.field0_0x0 = s_Null_Ptr_1050_38f3 + 0x7;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x3c),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return;
}




pub unsafe fn pass1_1040_3506(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = s_Null_Ptr_1050_38f3 + 0x7;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}


pub unsafe fn pass1_1040_3966(param_1: *mut u8,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x185,param_6);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  iVar1[0x1].field2_0x4 = 0;
  iVar1[0x1].field4_0x8 = 0;
  iVar1[0x1].field6_0xc = 0;
  iVar1[0x1].field7_0xe = 0;
  iVar1[0x1].field8_0x10 = 0;
  iVar1[0x1].field9_0x12 = 0;
  iVar1[0x1].field10_0x14 = 0;
  (&iVar1[0x1].field10_0x14 + 0x2) = 0x5;
  param_2.field0_0x0 = 0x3ffc;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x3c),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return;
}




pub unsafe fn pass1_1040_39e2(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x3ffc;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}


















pub unsafe fn invalidate_rect_1040_3ddc(in_struct_1: *mut StructC)

{
  let mut rect: RECT16;
  let mut uStack6: u32;

  rect = 0x780005;
  uStack6 = 0xdc0069;
  InvalidateRect16(0x0,&rect,&DAT_1050_1050);
  return;
}
pub unsafe fn dialog_item_ui_op_1040_3e08(struct_c_param_1: *mut StructC)

{
  let mut uVar1: u16;
  let mut struct_c_1: *mut StructC;
  let mut var3: u16;
  let mut LVar2: LRESULT;

  var3 = (struct_c_param_1 >> 0x10);
  struct_c_1 = struct_c_param_1;
  CheckRadioButton16(struct_c_1.field149_0xa0,0x18d,0x188,struct_c_1.field6_0x6);
  struct_c_1.field147_0x9c = 0;
  struct_c_1.field148_0x9e = 0;
  LVar2 = SendDlgItemMessage16(0x0,0x0,0x409,0x190,struct_c_1.field6_0x6);
  if (LVar2 != -1) {
    uVar1 = pass1_1018_3ab2(struct_c_1.field141_0x8e,LVar2,struct_c_1.field149_0xa0);
    struct_c_1.field148_0x9e = uVar1;
  }
  SetDlgItemInt16(0x0,struct_c_1.field147_0x9c,0x18e,struct_c_1.field6_0x6);
  SetDlgItemInt16(0x0,struct_c_1.field148_0x9e,0x191,struct_c_1.field6_0x6);
  match struct_c_1.field149_0xa0 {
  0x188 =>{
    struct_c_1.field152_0xa4 = 0x5;}
    // break;
  0x189 =>{
    struct_c_1.field152_0xa4 = 0x6;}
    // break;
  0x18a =>{
    struct_c_1.field152_0xa4 = 0x7;}
    // break;
  0x18b =>{
    struct_c_1.field152_0xa4 = 0x8;}
    // break;
  0x18c =>{
    struct_c_1.field152_0xa4 = 0x9;}
    // break;
  0x18d =>{
    struct_c_1.field152_0xa4 = 0xa;}
  }
  invalidate_rect_1040_3ddc(struct_c_param_1);
  return;
}
pub unsafe fn send_dlg_item_msg_1040_3f12(struct_c_param_1: *mut StructC,struct_c_param_2: *mut StructC,mut param_3: u32)

{
  let mut puVar1: *mut u8;
  let mut extraout_DX: u16;
  let mut iVar2: i16;
  let mut LVar3: LRESULT;
  let mut local_a: [u8;0x8] = [0;0x8];

  SendDlgItemMessage16(0x0,0x0,0xb,0x190,struct_c_param_1.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x190,struct_c_param_1.field6_0x6);
  pass1_1008_5784(CONCAT22(0x1050,local_a),param_3);
  loop {
    puVar1 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,puVar1));
    if ((extraout_DX | puVar1) == 0) { break; }
    LVar3 = SendDlgItemMessage16((puVar1 + 0x4),0x0,0x401,0x190,struct_c_param_1.field6_0x6);
    iVar2 = (LVar3 >> 0x10);
    if (((LVar3 == -1) && (iVar2 == -1)) || ((LVar3 == -0x2 && (iVar2 == -1)))) { break; }
  }
  SendDlgItemMessage16(0x0,0x0,0x407,0x190,struct_c_param_1.field6_0x6);
  SendDlgItemMessage16(0x0,0x1,0xb,0x190,struct_c_param_1.field6_0x6);
  return;
}
