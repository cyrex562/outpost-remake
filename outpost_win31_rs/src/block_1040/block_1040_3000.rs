


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_311a(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  let mut in_EDX: u32;
  let mut uVar4: u16;
  let mut uVar3: u32;
  let mut LVar5: LRESULT;
  paVar6: *mut astruct_27;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut iVar7: i16;

  uVar4 = (in_EDX >> 0x10);
  send_msg_1040_323c(CONCAT22(param_2,param_1));
  load_string_1010_847e(_u16_1050_14cc,0x531);
  if (param_4 == 0x181) {
    uVar3 = CONCAT22(uVar4,param_2);
    iVar1 = param_1 + 0x9a;
    iVar7 = iVar1;
    pass1_1018_3cda(*(astruct_506 **)(param_1 + 0x96),CONCAT22(param_2,param_1 + 0x19a),
                    CONCAT22(param_2,iVar1));
    pass1_1018_3424(iVar7,uVar3,(param_1 + 0x96));
    if (iVar7 == 0) {
      iVar7 = 0x21;
    }
    else {
      pass1_1018_3a42(uVar3,(param_1 + 0x96),CONCAT22(param_2,iVar1));
      pass1_1030_8344(_u16_1050_5748,CONCAT22(uVar3,iVar7));
      uVar2 = (iVar7 + 0x10);
      pass1_1030_8344(_u16_1050_5748,uVar2);
      PTR_LOOP_1050_5f0c = uVar2;
      PTR_LOOP_1050_5f0e = uVar3;
      PTR_LOOP_1050_5f10 = (&PTR_LOOP_1050_0000 + 1);
      iVar7 = 0x25;
    }
    pass1_1038_af40(param_1,uVar3,_PTR_LOOP_1050_5b7c,(param_1 + 0x8),iVar7);
    uVar4 = (uVar3 >> 0x10);
    LVar5 = SendMessage16(0x0,0x2,0x111,(param_1 + 0x6));
    iVar7 = 0x1;
    paVar6 =
             mixed_1010_20ba(CONCAT22(uVar4,(LVar5 >> 0x10)),_u16_1050_0ed0,
                             0x1002b,in_stack_0000fe8e,in_stack_0000ffb2,in_stack_0000ffb8,
                             in_stack_0000ffbc);
    pass1_1010_038e(paVar6,iVar7);
  }
  else {
    if ((param_4 == 0x181) || (0x1 < param_4 - 0x182U)) {
      post_win_msg_1040_7b3c((StructC *)CONCAT22(param_2,param_1),param_3,param_4,param_4);
      return;
    }
    set_win_pos_1040_331a(CONCAT22(param_2,param_1),param_3,param_4);
  }
  return;
}



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
pub fn enable_win_1040_32a8(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut uStack12: u32;

  uVar1 = param_1 + 0x19a;
  uStack12 = param_1 & 0xffff0000 | uVar1;
  uVar2 = param_1;
  pass1_1018_3a5c((param_1 + 0x96),(param_1 & 0xffff0000 | (param_1 + 0x9aU)),
                  (param_1 & 0xffff0000 | uVar1));
  SetWindowText16(CONCAT22(uVar2,uVar1),(param_1 + 0x90));
  BVar1 = string_1018_39d8((param_1 + 0x96),
                           (param_1 & 0xffff0000 | (param_1 + 0x9aU)),uStack12);
  EnableWindow16(BVar1 & 0x1,(param_1 + 0x8e));
  return;
}



pub fn set_win_pos_1040_331a(mut param_1: u32,mut param_2: u16 ,mut param_3: i16) -> BOOL16

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
pub fn send_msg_1040_3374(mut param_1: u32,param_2: *mut u32,mut param_3: u16 )

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
    if (LVar5 == -1) break;
    if (LVar5 == -0x2) {
      return;
    }
    uStack10 += 0x1;
  }
  return;
}



pub fn pass1_1040_3410(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1040_2f06(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_34a2(param_1: *mut StructD,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_3506(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = s_Null_Ptr_1050_38f3 + 0x7;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_3532(param_1: *mut u8,mut param_2: u16 ,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  paVar1: *mut astruct_27;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  iVar2 = 0;
  paVar1 =
           mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,0x2b,
                           in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
  pass1_1010_038e(paVar1,iVar2);
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  return;
}
pub fn show_win_1040_355a(StructB *param_1)

{
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  uVar1 = (param_1 >> 0x10);
  ShowWindow16(0x5,(param_1 + 0x6));
  SetFocus16((param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn set_win_text_1040_3590(mut param_1: u16 ,param_2: *mut astruct_923)

{
  let mut HVar1: HWND16;
  let mut HVar2: HWND16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  iVar5: *mut astruct_923;
  let mut uVar5: u16;
  let mut in_stack_0000f8f8: u16;
  let mut in_stack_0000fa1c: u16;
  let mut in_stack_0000fa22: u16;
  let mut in_stack_0000fa26: u16;
  let mut uVar6: u8;
  let mut in_stack_0000fa50: u16;
  let mut local_59a: u32;
  let mut local_596: u32;
  let mut BStack1426: bool;
  let mut uStack1424: u16;
  let mut local_58e: [u16;0x41] = [0;0x41];
  let mut local_50c: [u16;0x80] = [0;0x80];
  let mut uStack1036: u32;
  let mut puStack1032: *mut u32;
  let mut local_404: [u8;0x402] = [0;0x402];

  puStack1032 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                                CONCAT22(in_stack_0000fa50,0x2),in_stack_0000f8f8,in_stack_0000fa1c,
                                in_stack_0000fa22,in_stack_0000fa26);
  uVar4 = (puStack1032 >> 0x10);
  uStack1036 = (puStack1032 + 0x68);
  uVar5 = (param_2 >> 0x10);
  iVar5 = param_2;
  GetWindowText16(0x80,CONCAT22(0x1050,local_50c),iVar5.field6_0x6);
  uVar6 = SUB21(local_50c,0x0);
  wsprintf16(local_58e,CONCAT13((local_50c >> 0x8),CONCAT12(uVar6,0x1050)),uVar6,
             CONCAT22(uStack1036,0x1050),(uStack1036 >> 0x10));
  BStack1426 = SetWindowText16(CONCAT22(0x1050,local_58e),iVar5.field6_0x6);
  sprintf_op_1018_34b6(BStack1426,uVar4,iVar5.field141_0x8e);
  uStack1424 = uVar4;
  pass1_1018_3d44(iVar5.field141_0x8e,CONCAT22(0x1050,&local_59a),CONCAT22(0x1050,&local_596));
  HVar1 = GetDlgItem16(0x193,iVar5.field6_0x6);
  iVar5.field148_0x98 = HVar1;
  EnableWindow16(0x1,HVar1);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_404,&DAT_1050_1050);
  wsprintf16(local_50c,0x50,CONCAT22(local_404,0x1050),CONCAT22(local_596,0x1050),
             (local_596 >> 0x10));
  HVar1 = GetDlgItem16(0x195,iVar5.field6_0x6);
  SetWindowText16(CONCAT22(0x1050,local_50c),HVar1);
  HVar2 = GetDlgItem16(0x196,iVar5.field6_0x6);
  HVar1 = HVar2;
  sprintf_op_1018_34b6(HVar2,uVar4,iVar5.field141_0x8e);
  SetWindowText16(CONCAT22(uVar4,HVar2),HVar1);
  HVar1 = GetDlgItem16(0x197,iVar5.field6_0x6);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_404,&DAT_1050_1050);
  SetWindowText16(CONCAT22(0x1050,local_404),HVar1);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_404,&DAT_1050_1050);
  wsprintf16(local_50c,CONCAT22(local_404,0x1050),CONCAT22(local_59a,0x1050),
             (local_59a >> 0x10));
  HVar1 = GetDlgItem16(0x198,iVar5.field6_0x6);
  SetWindowText16(CONCAT22(0x1050,local_50c),HVar1);
  uVar3 = GetDlgItem16(0x199,iVar5.field6_0x6);
  HVar1 = uVar3;
  unk_str_op_1018_35b0(uVar3,iVar5.field141_0x8e);
  if ((uVar4 | uVar3) == 0) {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_404,&DAT_1050_1050);
    SetWindowText16(CONCAT22(0x1050,local_404),HVar1);
    GetDlgItem16(0x19a,iVar5.field6_0x6);
    HVar1 = _u16_1050_14cc;
    load_string_1010_84e0(HVar1,(_u16_1050_14cc >> 0x10),0x3ff,local_404,&DAT_1050_1050);
    SetWindowText16(CONCAT22(0x1050,local_404),HVar1);
    EnableWindow16(0x0,iVar5.field148_0x98);
    return;
  }
  SetWindowText16(CONCAT22(uVar4,uVar3),HVar1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn message_box_op_1040_37f0(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut uVar4: u16;
  let mut LVar5: LRESULT;
  let mut in_stack_0000fa94: u16;
  let mut in_stack_0000fbb8: u16;
  let mut in_stack_0000fbbe: u16;
  let mut in_stack_0000fbc2: u16;
  let mut in_stack_0000fbec: u16;
  let mut iVar6: i16;
  let mut local_40c: [u8;0x402] = [0;0x402];
  let mut pcStack10: *mut c_char;
  paStack6: *mut astruct_27;

  paVar3 = CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0x193) {
    paStack6 =
               mixed_1010_20ba(paVar3,_u16_1050_0ed0,CONCAT22(in_stack_0000fbec,0x2),in_stack_0000fa94,
                               in_stack_0000fbb8,in_stack_0000fbbe,in_stack_0000fbc2);
    uVar2 = (paStack6 >> 0x10);
    pcStack10 = *(paStack6 + 0x68);
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_40c,&DAT_1050_1050);
    uVar1 = MessageBox16(0x30,pcStack10,CONCAT22(0x1050,local_40c),(param_2 + 0x6));
    pass1_1018_3710(uVar1,uVar2,*(astruct_263 **)(param_2 + 0x8e));
    PostMessage16(0x0,0x2,0x111,(param_2 + 0x6));
  }
  else {
    if (param_5 != 0x194) {
      post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
      return;
    }
    pass1_1038_af40(param_2,param_1,_PTR_LOOP_1050_5b7c,(param_2 + 0x8),0x21);
    uVar4 = (paVar3 >> 0x10);
    LVar5 = SendMessage16(0x0,0x2,0x111,(param_2 + 0x6));
    iVar6 = 0x1;
    paStack6 =
               mixed_1010_20ba(CONCAT22(uVar4,(LVar5 >> 0x10)),_u16_1050_0ed0,
                               0x1002b,in_stack_0000fa94,in_stack_0000fbb8,in_stack_0000fbbe,
                               in_stack_0000fbc2);
    pass1_1010_038e(paStack6,iVar6);
  }
  return;
}



pub fn pass1_1040_38d4(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1040_3506(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_3966(param_1: *mut u8,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_39e2(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x3ffc;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_3a0e(param_1: *mut u8,mut param_2: u16 ,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  paVar1: *mut astruct_27;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  iVar2 = 0;
  paVar1 =
           mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,0x2b,
                           in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
  pass1_1010_038e(paVar1,iVar2);
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  return;
}



pub unsafe fn enable_win_1040_3a36(param_1: *mut astruct_924,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16) -> u16

{
  let mut puVar1: *mut u16;
  let mut bVar2: bool;
  iVar3: *mut astruct_924;
  let mut uVar3: u16;

  bVar2 = false;
  iVar3 = param_1;
  uVar3 = (param_1 >> 0x10);
  if (param_4 == 0) {
    if (iVar3.field155_0x9e <= iVar3.field154_0x9c) goto LAB_1040_3a79;
    puVar1 = &iVar3.field154_0x9c;
    *puVar1 = *puVar1 + 1;
  }
  else {
    if (param_4 != 1) goto LAB_1040_3a79;
    if (iVar3.field154_0x9c == 0) goto LAB_1040_3a79;
    puVar1 = &iVar3.field154_0x9c;
    *puVar1 = *puVar1 - 0x1;
  }
  bVar2 = true;//
// LAB_1040_3a79:
  if (bVar2) {
    SetDlgItemInt16(0x0,iVar3.field154_0x9c,0x18e,iVar3.field6_0x6);
  }
  if ((iVar3.field154_0x9c != 0) && (iVar3.field158_0xa2 == 0)) {
    iVar3.field158_0xa2 = 0x1;
    EnableWindow16(0x1,iVar3.field153_0x9a);
  }
  if ((iVar3.field154_0x9c == 0) && (iVar3.field158_0xa2 != 0)) {
    iVar3.field158_0xa2 = 0;
    EnableWindow16(0x0,iVar3.field153_0x9a);
  }
  return 0x0;
}
pub fn show_win_1040_3ae8(StructB *param_1)

{
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  uVar1 = (param_1 >> 0x10);
  ShowWindow16(0x5,(param_1 + 0x6));
  SetFocus16((param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_3b1e(mut param_1: u16 ,StructC *struct_c_param_1)

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut HVar3: HWND16;
  StructC *pSVar4;
  let mut in_register_0000000a: u16;
  StructC *struct_c_4;
  let mut unaff_SI: u16;
  StructC *struct_c_param_2;
  let mut uVar5: u32;
  let mut in_stack_0000fd8a: u16;
  let mut in_stack_0000feae: u16;
  let mut in_stack_0000feb4: u16;
  let mut in_stack_0000feb8: u16;
  let mut puStack282: *mut u32;
  let mut local_10e: [u16;0x41] = [0;0x41];
  let mut local_8c: [u16;0x41] = [0;0x41];
  let mut uStack10: u32;
  let mut puStack6: *mut u32;

  puStack6 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             CONCAT22(unaff_SI,0x2),in_stack_0000fd8a,in_stack_0000feae,in_stack_0000feb4,
                             in_stack_0000feb8);
  uStack10 = (puStack6 + 0x68);
  struct_c_param_2 = (StructC *)(struct_c_param_1 >> 0x10);
  struct_c_4 = (StructC *)struct_c_param_1;
  GetWindowText16(0x80,CONCAT22(0x1050,local_8c),struct_c_4.field6_0x6);
  wsprintf16(local_10e,CONCAT22(local_8c,0x1050),CONCAT22(uStack10,0x1050),
             (uStack10 >> 0x10));
  SetWindowText16(CONCAT22(0x1050,local_10e),struct_c_4.field6_0x6);
  puStack282 = (struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4.field_0x96));
  pSVar4 = struct_c_param_2;
  pass1_1018_3d44(struct_c_4.field141_0x8e,
                  (struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4.field142_0x92)),
                  (struct_c_param_1 & 0xffff0000 | ZEXT24(&struct_c_4.field_0x96)));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x80,local_10e,&DAT_1050_1050
            );
  uVar1 = struct_c_4.field142_0x92;
  wsprintf16(local_8c,CONCAT22(local_10e,0x1050),CONCAT22(*puStack282,0x1050),
             (*puStack282 >> 0x10),uVar1,(uVar1 >> 0x10));
  SetDlgItemText16(CONCAT22(0x1050,local_8c),0x187,struct_c_4.field6_0x6);
  BVar2 = CheckRadioButton16(0x188,0x18d,0x188,struct_c_4.field6_0x6);
  struct_c_4.field149_0xa0 = 0x188;
  uVar5 = switch_1018_3b9e(BVar2,pSVar4,struct_c_4.field141_0x8e,struct_c_4.field149_0xa0);
  send_dlg_item_msg_1040_3f12(struct_c_4,struct_c_param_2,uVar5);
  dialog_item_ui_op_1040_3e08(struct_c_param_1);
  HVar3 = GetDlgItem16(0x186,struct_c_4.field6_0x6);
  struct_c_4.field146_0x9a = HVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_ui_op_1040_3c64
               (mut param_1: u16 ,StructC *struct_c_param_1,StructC *struct_c_param_2,mut param_4: u16 ,mut param_5: u32)

{
  let mut UVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut uVar3: u32;
  let mut LVar4: LRESULT;
  paVar5: *mut astruct_27;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut iVar6: i16;

  if (param_5 == 0x186) {
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x409,0x190,struct_c_param_1.field6_0x6);
    uVar2 = (LVar4 >> 0x10);
    UVar1 = GetDlgItemInt16(0x0,NULL,0x0,0x18e);
    pass1_1018_36e6(struct_c_param_1.field141_0x8e,UVar1,LVar4,struct_c_param_1.field149_0xa0);
    pass1_1038_af40(struct_c_param_1,uVar2,_PTR_LOOP_1050_5b7c,&struct_c_param_1.field_0x8,0x22);
    LVar4 = SendMessage16(0x0,0x2,0x111,struct_c_param_1.field6_0x6);
    iVar6 = 0x1;
    paVar5 =
             mixed_1010_20ba(CONCAT22(in_register_0000000a,(LVar4 >> 0x10)),_u16_1050_0ed0,
                             0x1002b,in_stack_0000fe9a,in_stack_0000ffbe,in_stack_0000ffc4,
                             in_stack_0000ffc8);
    pass1_1010_038e(paVar5,iVar6);
  }
  else {
    if (param_5 - 0x186 < 0x2) {//
// LAB_1040_3c7f:
      post_win_msg_1040_7b3c
                ((StructC *)CONCAT22(struct_c_param_2,struct_c_param_1),param_4,param_5,param_5);
      return;
    }
    if (param_5 - 0x188 < 0x5 || param_5 == 0x18d) {
      struct_c_param_1.field149_0xa0 = param_5;
      uVar3 = switch_1018_3b9e(param_5,param_1,struct_c_param_1.field141_0x8e,param_5);
      send_dlg_item_msg_1040_3f12(struct_c_param_1,struct_c_param_2,uVar3);
    }
    else {
      if (param_5 - 0x188 != 0x8) goto LAB_1040_3c7f;
      if (param_5 != 1) {
        return;
      }
    }
    dialog_item_ui_op_1040_3e08((StructC *)CONCAT22(struct_c_param_2,struct_c_param_1));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn get_dc_op_1040_3d5e(param_1: *mut astruct_1) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  iVar4: *mut astruct_1;
  iVar3: *mut astruct_934;
  let mut uVar3: u16;
  let mut puStack8: *mut u32;
  local_4: HDC16;

  uVar3 = (param_1 >> 0x10);
  iVar4 = param_1;
  local_4 = GetDC16(iVar4.field6_0x6);
  uVar2 = FUN_1010_830a(local_4,in_EDX,s_tile2_bmp_1050_1538,_u16_1050_14cc,iVar4.field163_0xa4);
  puStack8 = CONCAT22(in_EDX,uVar2);
  iVar3 = *puStack8;
  ppcVar1 = &iVar3.field6_0x8;
  (**ppcVar1)(0x1010,uVar2,in_EDX,&local_4,&DAT_1050_1050);
  ppcVar1 = &iVar3.field4_0x4;
  (**ppcVar1)(0x1010,puStack8,0x50078,&local_4,&DAT_1050_1050);
  ppcVar1 = &iVar3.field8_0xc;
  (**ppcVar1)(0x1010,puStack8,&local_4,&DAT_1050_1050);
  ReleaseDC16(local_4,iVar4.field6_0x6);
  return 0x0;
}
pub fn invalidate_rect_1040_3ddc(StructC *in_struct_1)

{
  let mut rect: RECT16;
  let mut uStack6: u32;

  rect = (RECT16)0x780005;
  uStack6 = 0xdc0069;
  InvalidateRect16(0x0,&rect,&DAT_1050_1050);
  return;
}
pub fn dialog_item_ui_op_1040_3e08(StructC *struct_c_param_1)

{
  let mut uVar1: u16;
  StructC *struct_c_1;
  let mut var3: u16;
  let mut LVar2: LRESULT;

  var3 = (struct_c_param_1 >> 0x10);
  struct_c_1 = (StructC *)struct_c_param_1;
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
  switch(struct_c_1.field149_0xa0) {
  case 0x188:
    struct_c_1.field152_0xa4 = 0x5;
    break;
  case 0x189:
    struct_c_1.field152_0xa4 = 0x6;
    break;
  case 0x18a:
    struct_c_1.field152_0xa4 = 0x7;
    break;
  case 0x18b:
    struct_c_1.field152_0xa4 = 0x8;
    break;
  case 0x18c:
    struct_c_1.field152_0xa4 = 0x9;
    break;
  case 0x18d:
    struct_c_1.field152_0xa4 = 0xa;
  }
  invalidate_rect_1040_3ddc(struct_c_param_1);
  return;
}
pub fn send_dlg_item_msg_1040_3f12(StructC *struct_c_param_1,StructC *struct_c_param_2,mut param_3: u32)

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
    if ((extraout_DX | puVar1) == 0) break;
    LVar3 = SendDlgItemMessage16(*(LPARAM *)(puVar1 + 0x4),0x0,0x401,0x190,struct_c_param_1.field6_0x6);
    iVar2 = (LVar3 >> 0x10);
    if (((LVar3 == -1) && (iVar2 == -1)) || ((LVar3 == -0x2 && (iVar2 == -1)))) break;
  }
  SendDlgItemMessage16(0x0,0x0,0x407,0x190,struct_c_param_1.field6_0x6);
  SendDlgItemMessage16(0x0,0x1,0xb,0x190,struct_c_param_1.field6_0x6);
  return;
}



pub fn pass1_1040_3fd6(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1040_39e2(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
