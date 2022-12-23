
pub unsafe fn win_ui_op_1038_a788(mut param_1: u16 ,mut param_2: u32,mut param_3: i16)

{
  let mut hwnd: HWND16;
  let mut uVar1: u16;
  let mut hwnd_00: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut uVar3: u16;
  let mut pUVar2: *mut u16;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000ff70: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7a: u16;
  let mut puVar4: *mut u8;
  let mut uVar5: u16;
  let mut local_52: [u8;0x50] = [0;0x50];
  let mut puVar3: *mut u8;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  if (param_3 != 0) {
    uVar3 = (param_2 >> 0x10);
    hwnd = GetDlgItem16(0x115,(param_2 + 0x6));
    GetWindowText16(0x50,CONCAT22(0x1050,local_52),hwnd);
    uVar1 = str_op_1000_3da4(CONCAT22(0x1050,local_52));
    if (uVar1 != 0) {
      puVar4 = local_52;
      uVar5 = SUB42(&DAT_1050_1050,0x0);
      pUVar2 =
               mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(puVar4,0x2),in_stack_0000fe4c,
                               in_stack_0000ff70,in_stack_0000ff76,in_stack_0000ff7a);
      pass1_1010_5fd8((pUVar2 >> 0x10),pUVar2,CONCAT22(uVar5,puVar4));
      hwnd_00 = GetWindowWord16(-0x8,(param_2 + 0x6));
      PostMessage16(0x0,0x105,0x111,hwnd_00);
      destroy_win_1040_7b98(param_2);
    }
  }
  return;
}


pub unsafe fn pass1_1038_a80c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a6c8(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn enable_win_1038_a8f8(param_1: *mut StructC,mut param_2: u16 ,param_3: TwoWords)

{
  let mut hwnd: HWND16;
  let mut enable: bool;

  if (param_3.b_0x2 == 0x116) {
    SendDlgItemMessage16(0x0,0x1,0x401,0x11a,(param_1 + 0x6));
    hwnd = GetDlgItem16(0x11a,(param_1 + 0x6));
    enable = 0;
  }
  else {
    if ((param_3.b_0x2 == 0x116) || (0x2 < param_3.b_0x2 - 0x117)) {
      post_win_msg_1040_7b3c(param_1,param_2,param_3,param_3.b_0x2);
      return;
    }
    hwnd = GetDlgItem16(0x11a,(param_1 + 0x6));
    enable = 0x1;
  }
  EnableWindow16(enable,hwnd);
  return;
}


pub unsafe fn win_ui_op_1038_a972(struct_b_param_1: *mut StructB)

{
  let mut hwnd: HWND16;
  let mut BVar1: bool;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut uVar2: u32;
   let mut struct_b_3: *mut StructB;
  let mut uVar4: u16;
  let mut LVar5: LRESULT;
  let mut in_stack_0000ffaa: u16;

  uVar3 = (in_EDX >> 0x10);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar4 = (struct_b_param_1 >> 0x10);
  struct_b_3 = struct_b_param_1;
  SendDlgItemMessage16(0x0,0x1,0x401,0x116,struct_b_3.lpvoid_field_0x8);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0x401,0x11a,struct_b_3.lpvoid_field_0x8);
  uVar2 = CONCAT22(uVar3,(LVar5 >> 0x10));
  hwnd = GetDlgItem16(0x11a,struct_b_3.lpvoid_field_0x8);
  BVar1 = EnableWindow16(0x0,hwnd);
  win_1008_5c7c(BVar1,uVar2,_u16_1050_02a0,0x40001);
  (struct_b_3 + 0x7).field0_0x0 = BVar1;
  unk_win_ui_op_1038_a18c(uVar2,struct_b_param_1,in_stack_0000ffaa);
  ShowWindow16(0x5,struct_b_3.lpvoid_field_0x8);
  return;
}


pub unsafe fn win_sys_op_1038_a9fa(mut param_1: u32,mut param_2: i16)

{
  let mut hwnd: u16;
  let mut in_EDX: *mut Struct57;
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut LVar4: LRESULT;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000fff0: u16;

  if (param_2 != 0) {
    puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22(in_stack_0000fff0,0x2),in_stack_0000fe98,
                             in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x116,(iVar1 + 0x6));
    if (((LVar4 >> 0x10) | LVar4) == 0) {
      LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x117,(iVar1 + 0x6));
      if (((LVar4 >> 0x10) | LVar4) == 0) {
        LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x118,(iVar1 + 0x6));
        if (((LVar4 >> 0x10) | LVar4) == 0) {
          LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x119,(iVar1 + 0x6));
          if (((LVar4 >> 0x10) | LVar4) != 0) {
            PTR_LOOP_1050_13ae = &u32_1050_0004;
          }
        }
        else {
          PTR_LOOP_1050_13ae = (&u16_1050_0002 + 1);
        }
      }
      else {
        PTR_LOOP_1050_13ae = &u16_1050_0002;
      }
    }
    else {
      PTR_LOOP_1050_13ae = (&PTR_LOOP_1050_0000 + 1);
    }
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x11a,(iVar1 + 0x6));
    (puVar3 + 0x82) = LVar4;
    hwnd = GetWindowWord16(-0x8,(iVar1 + 0x6));
    PostMessage16(0x0,0x105,0x111,hwnd);
    destroy_win_1040_7b98(param_1);
  }
  return;
}


pub unsafe fn pass1_1038_aaf0(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a8cc(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn unk_win_ui_op_1038_ac38(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,hdc_param_5: HDC16)

{
  let mut IVar1: i16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar6: u32;
  let mut uVar4: u8;
  let mut iVar1: *mut astruct_46;
  let mut iVar2: *mut astruct_786;
  let mut uVar2: u16;
  let mut uVar5: u16;
  let mut uVar1: u16;

  GetStockObject16(BLACK_BRUSH);
  if (_u16_1050_5b78 == 0) {
    uVar6 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
    uVar1 = (uVar6 >> 0x10);
    iVar2 = uVar6;
    _u16_1050_5b6c = CONCAT12(iVar2.field_0x3ec,CONCAT11(iVar2.field_0x3ed,iVar2.field_0x3ee));
    _u16_1050_5b70 = CONCAT12(iVar2.field_0x3e4,CONCAT11(iVar2.field_0x3e5,iVar2.field_0x3e6));
    _u16_1050_5b74 = CONCAT12(iVar2.field_0x3f8,CONCAT11(iVar2.field_0x3f9,iVar2.field_0x3fa));
    _u16_1050_5b78 = CONCAT12(iVar2.field_0x94,CONCAT11(iVar2.field_0x95,iVar2.field_0x96));
  }
  if (param_4 < 0x4) {//
// LAB_1038_acf0:
    IVar1 = GetDlgCtrlID16(param_3);
    if (IVar1 == 0xfd4) {
      uVar2 = _u16_1050_5b70;
      uVar5 = (_u16_1050_5b70 >> 0x10);
  // TODO: goto LAB_1038_ad0e;
    }
    if (IVar1 != 0xfd5) {
      if (IVar1 == 0xfd6) {
        uVar2 = _u16_1050_5b6c;
        uVar5 = (_u16_1050_5b6c >> 0x10);
    // TODO: goto LAB_1038_ad0e;
      }
      if (IVar1 == 0xfd7) {
        uVar2 = _u16_1050_5b74;
        uVar5 = (_u16_1050_5b74 >> 0x10);
    // TODO: goto LAB_1038_ad0e;
      }
    }
  }
  else if (param_4 != 0x4) {
    if ((param_4 == 0x4) || (0x1 < param_4 - 0x5)) {
      return;
    }
// TODO: goto LAB_1038_acf0;
  }
  uVar2 = _u16_1050_5b78;
  uVar5 = (_u16_1050_5b78 >> 0x10);//
// LAB_1038_ad0e:
  SetTextColor16(CONCAT22(uVar5,uVar2),hdc_param_5);
  SetBkColor16(0x1000000,hdc_param_5);
  return;
}


pub unsafe fn pass1_1038_ad4c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_abb0(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1038_ae28(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_ae08(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn show_win_1038_c044(struct_b_param_1: *mut StructB)

{
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  uVar1 = (struct_b_param_1 >> 0x10);
  ShowWindow16(0x5,(struct_b_param_1 + 0x6));
  SetFocus16((struct_b_param_1 + 0x6));
  return;
}


pub unsafe fn pass1_1038_be76(param_1: *mut u8,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct27;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  if (param_3 == 0) {
    iVar2 = 0;
    paVar1 =
             mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,0x2b,
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1010_038e(paVar1,iVar2);
  }
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  return;
}


pub unsafe fn pass1_1038_c410(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_be4a(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1038_c52a(param_1: *mut u8,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct27;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  if (param_3 == 0) {
    iVar2 = 0;
    paVar1 =
             mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,0x2b,
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1010_038e(paVar1,iVar2);
  }
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  return;
}


pub unsafe fn show_win_1038_c558(struct_b_param_1: *mut StructB)

{
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  uVar1 = (struct_b_param_1 >> 0x10);
  ShowWindow16(0x5,(struct_b_param_1 + 0x6));
  SetFocus16((struct_b_param_1 + 0x6));
  return;
}


pub unsafe fn win_dlg_op_1038_c58e(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut in_register_0000000a: u16;
  let mut iVar2: i16;
  let mut unaff_SI: u16;
  let mut uVar3: u16;
  let mut in_stack_0000f68e: u16;
  let mut in_stack_0000f7b2: u16;
  let mut in_stack_0000f7b8: u16;
  let mut in_stack_0000f7bc: u16;
  let mut puStack2070: *mut u32;
  let mut local_80e: [u16;0x201] = [0;0x201];
  let mut local_40c: [u16;0x201] = [0;0x201];
  let mut uStack10: u32;
  let mut puStack6: *mut u32;

  puStack6 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             CONCAT22(unaff_SI,0x2),in_stack_0000f68e,in_stack_0000f7b2,in_stack_0000f7b8,
                             in_stack_0000f7bc);
  uStack10 = (puStack6 + 0x68);
  uVar3 = (param_2 >> 0x10);
  iVar2 = param_2;
  GetWindowText16(0x80,CONCAT22(0x1050,local_40c),(iVar2 + 0x6));
  wsprintf16(local_80e,CONCAT22(local_40c,0x1050),CONCAT22(uStack10,0x1050),
             (uStack10 >> 0x10));
  SetWindowText16(CONCAT22(0x1050,local_80e),(iVar2 + 0x6));
  puStack2070 = (param_2 & 0xffff0000 | (iVar2 + 0x96));
  pass1_1008_e038((iVar2 + 0x8e),(param_2 & 0xffff0000 | ZEXT24((iVar2 + 0x92))),
                  (param_2 & 0xffff0000 | (iVar2 + 0x96)));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x400,local_80e,
             &DAT_1050_1050);
  uVar1 = (iVar2 + 0x92);
  wsprintf16(local_40c,CONCAT22(local_80e,0x1050),CONCAT22(*puStack2070,0x1050),
             (*puStack2070 >> 0x10),uVar1,(uVar1 >> 0x10));
  SetDlgItemText16(CONCAT22(0x1050,local_40c),0x17f,(iVar2 + 0x6));
  return;
}


pub unsafe fn message_box_op_1038_c672(param_1: u8,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut uVar1: u16;
  let mut local_404: [u8;0x402] = [0;0x402];

  uVar1 = (_u16_1050_14cc >> 0x10);
  if (param_5 == 0x17d) {
    load_string_1010_84e0(_u16_1050_14cc,uVar1,0x3ff,local_404,&DAT_1050_1050);
    MessageBox16(0x30,*(param_2 + 0x92),CONCAT22(0x1050,local_404),(param_2 + 0x6));
  }
  else {
    if (param_5 != 0x17e) {
      post_win_msg_1040_7b3c(CONCAT22(param_3,param_2),param_4,param_5,param_5);
      return;
    }
    load_string_1010_84e0(_u16_1050_14cc,uVar1,0x3ff,local_404,&DAT_1050_1050);
    MessageBox16(0x30,*(param_2 + 0x92),CONCAT22(0x1050,local_404),(param_2 + 0x6));
    pass1_1008_e164((param_2 + 0x8e));
  }
  PostMessage16(0x0,0x2,0x111,(param_2 + 0x6));
  return;
}


pub unsafe fn pass1_1038_c726(StructD_32: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_c4fe(StructD_32);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(StructD_32);
  }
  return StructD_32;
}

pub unsafe fn win_ui_op_1038_c89c(struct_b_param_1: *mut StructB)

{
  let mut HVar1: HWND16;
   let mut struct_b_4: *mut StructB;
  let mut uVar3: u16;
  let mut enable: bool;
  let mut iVar1: *mut astruct_910;
  let mut uVar1: u32;
  let mut uVar2: u32;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar3 = (struct_b_param_1 >> 0x10);
  struct_b_4 = struct_b_param_1;
  CheckRadioButton16(0xfac,0xfad,0xfac,struct_b_4.lpvoid_field_0x8);
  uVar1 = &struct_b_4[0x7].field1_0x2;
  (uVar1 + 0xa) = 0x1;
  uVar2 = &struct_b_4[0x7].field1_0x2;
  iVar1 = (uVar2 + 0x12);
  if (iVar1 == &u32_1050_0004) {//
// LAB_1038_c8da:
    HVar1 = GetDlgItem16(0xfce,struct_b_4.lpvoid_field_0x8);
    if (HVar1 != 0) {
      EnableWindow16(0x1,HVar1);
    }
    HVar1 = GetDlgItem16(0x1,struct_b_4.lpvoid_field_0x8);
//    if (HVar1 == 0) goto LAB_1038_c93c;
    enable = 0;
  }
  else {
//    if (((iVar1 -0x5) < 1) || (SBORROW2((iVar1 -0x5),1))) goto LAB_1038_c93c;
    if (iVar1 != &u16_1050_0008 && 0x0 < (iVar1 -0x7)) {
//      if (iVar1 != (&u16_1050_0008 + 1)) goto LAB_1038_c93c;
  // TODO: goto LAB_1038_c8da;
    }
    HVar1 = GetDlgItem16(0xfce,struct_b_4.lpvoid_field_0x8);
//    if (HVar1 == 0) goto LAB_1038_c93c;
    enable = 0x1;
  }
  EnableWindow16(enable,HVar1);//
// LAB_1038_c93c:
  move_win_1040_826c(struct_b_param_1,0xc8,0x0);
  ShowWindow16(0x5,struct_b_4.lpvoid_field_0x8);
  return;
}

pub unsafe fn win_dlg_op_1038_c95e(struct_param_1: *mut astruct_882,mut param_2: i16)

{
  let mut uVar3: u32;
  let mut UVar4: u16;
  let mut UVar5: u16;
  let mut UVar6: u16;
  let mut iVar3: *mut astruct_882;
  let mut uVar7: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;

  iVar3 = struct_param_1;
  uVar7 = (struct_param_1 >> 0x10);
  if (param_2 == 0) {
    uVar3 = iVar3.field141_0x8e;
    (uVar3 + 0xa) = 0;
  }
  else {
    UVar4 = IsDlgButtonChecked(0xfac,iVar3.field6_0x6);
    if (UVar4 == 0) {
      UVar5 = IsDlgButtonChecked(0xfad,iVar3.field6_0x6);
      if (UVar5 == 0) {
        UVar6 = IsDlgButtonChecked(0xfae,iVar3.field6_0x6);
        if (UVar6 == 0) {
          UVar6 = IsDlgButtonChecked(0xfaf,iVar3.field6_0x6);
          if (UVar6 == 0) {
            UVar6 = IsDlgButtonChecked(0xfb0,iVar3.field6_0x6);
            if (UVar6 != 0) {
              uVar3 = iVar3.field141_0x8e;
              (uVar3 + 0xa) = 0x5;
            }
          }
          else {
            uVar3 = iVar3.field141_0x8e;
            (uVar3 + 0xa) = 0x4;
          }
        }
        else {
          uVar3 = iVar3.field141_0x8e;
          (uVar3 + 0xa) = 0x3;
        }
      }
      else {
        uVar2 = iVar3.field141_0x8e;
        (uVar2 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = iVar3.field141_0x8e;
      (uVar1 + 0xa) = 0x1;
    }
  }
  DestroyWindow16(iVar3.field6_0x6);
  PTR_LOOP_1050_5b80 = null_mut();
  return;
}

pub unsafe fn FUN_1038_ca42()

{
  return;
}

pub unsafe fn pass1_1038_ca46(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_c80a(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn show_win_1038_cb5c(mut param_1: u32,struct_b_param_1: *mut StructB,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;
  let mut paVar5: *mut Struct57;
  let mut uVar6: u16;
  let mut paVar7: *mut Struct57;
   let mut struct_b_5: *mut StructB;
  let mut uVar8: u16;
  let mut puVar9: *mut u16;
  let mut in_stack_0000fe48: u16;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000ff72: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7a: u16;
  let mut iStack10: i16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar8 = (struct_b_param_1 >> 0x10);
  struct_b_5 = struct_b_param_1;
  uVar3 = pass1_1008_eb6e();
  for iStack10 in 0 .. uVar3 {
    uVar1 = &struct_b_5[0x7].field1_0x2;
    puVar9 = pass1_1008_eb5c(uVar1,(uVar1 >> 0x10),iStack10);
    paVar7 = (param_1 & 0xffff0000 | puVar9 >> 0x10);
    paVar4 = puVar9;
    uVar2 = (puVar9 >> 0x10);
    paVar5 = paVar4;
    mem_op_1000_179c(0x42,paVar7);
    uVar6 = paVar7 | paVar5;
    param_1 = paVar7 & 0xffff0000 | uVar6;
    if (uVar6 != 0) {
      pass1_1008_3bd6(param_1,paVar5,paVar7,0x0,CONCAT22(*puVar9,paVar4.field1_0x2),0x101,0xff0100,
                      CONCAT22(struct_b_5.lpvoid_field_0x8,paVar4.field2_0x4),param_3,in_stack_0000fe48,
                      in_stack_0000fe4c,in_stack_0000ff72,in_stack_0000ff76,in_stack_0000ff7a);
    }
  }
  win_1008_5c7c(uVar3,param_1,_u16_1050_02a0,0x90001);
  ShowWindow16(0x5,struct_b_5.lpvoid_field_0x8);
  return;
}


pub unsafe fn destroy_window_1038_cc00(param_1: *mut StructC,mut param_2: u16 ,mut param_3: u32,param_4: *mut u8)

{
  let mut uVar1: u16;
  let mut iVar2: i16;

  uVar1 = param_3 - 0x1cd;
  if (uVar1 == 0) {
    iVar2 = 0x1;
  }
  else {
    uVar1 = param_3 - 0x1ce;
    if (uVar1 == 0) {
      iVar2 = 0x2;
    }
    else {
      uVar1 = param_3 - 0x1cf;
      if (uVar1 == 0) {
        iVar2 = 0x3;
      }
      else {
        uVar1 = param_3 - 0x1d0;
        if (uVar1 == 0) {
          iVar2 = 0x4;
        }
        else {
          uVar1 = param_3 - 0x1d1;
          if (uVar1 != 0) {
            post_win_msg_1040_7b3c(param_1,param_2,param_3,param_3);
            return;
          }
          iVar2 = 0x5;
        }
      }
    }
  }
  pass1_1008_eb74(param_4,(param_1 + 0x8e),iVar2);
  if (uVar1 != 0) {
    win_1008_5c7c(uVar1,param_4,_u16_1050_02a0,CONCAT22(uVar1,1));
    DestroyWindow16((param_1 + 0x6));
  }
  return;
}


pub unsafe fn pass1_1038_cc74(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_cb30(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn destroy_window_1038_cd88(struct_b_param_1: *mut StructB)

{
   let mut struct_1: *mut StructB;
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  uVar1 = (struct_b_param_1 >> 0x10);
  struct_1 = struct_b_param_1;
  ShowWindow16(0x5,struct_1.lpvoid_field_0x8);
  struct_1[0x7].lpvoid_field_0x8 = (&PTR_LOOP_1050_0000 + 1);
  unk_win_msg_op_1008_9510((struct_b_param_1 & 0xffff0000 | ZEXT24(&struct_1[0x7].lpvoid_field_0x8)));
  DestroyWindow16(struct_1.lpvoid_field_0x8);
  return;
}

pub unsafe fn check_dlg_btn_checked_1038_cdd6(param_1: *mut astruct_61,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut UVar2: u16;
  let mut iVar3: *mut astruct_61;
  let mut uVar3: u16;

  iVar3 = param_1;
  uVar3 = (param_1 >> 0x10);
  if (param_2 == 0) {
    uVar1 = iVar3.field142_0x8e;
    (uVar1 + 0xa) = 0;
  }
  else {
    UVar2 = IsDlgButtonChecked(0x182e,&iVar3.field_0x6);
    if (UVar2 == 0) {
      UVar2 = IsDlgButtonChecked(0x182f,&iVar3.field_0x6);
      if (UVar2 == 0) {
        UVar2 = IsDlgButtonChecked(0x1829,&iVar3.field_0x6);
        if (UVar2 == 0) {
          UVar2 = IsDlgButtonChecked(0x182a,&iVar3.field_0x6);
          if (UVar2 == 0) {
            UVar2 = IsDlgButtonChecked(0x182c,&iVar3.field_0x6);
            if (UVar2 == 0) {
              UVar2 = IsDlgButtonChecked(0x182d,&iVar3.field_0x6);
              if (UVar2 != 0) {
                uVar1 = iVar3.field142_0x8e;
                (uVar1 + 0xa) = 0x7;
              }
            }
            else {
              uVar1 = iVar3.field142_0x8e;
              (uVar1 + 0xa) = 0x6;
            }
          }
          else {
            uVar1 = iVar3.field142_0x8e;
            (uVar1 + 0xa) = 0x4;
          }
        }
        else {
          uVar1 = iVar3.field142_0x8e;
          (uVar1 + 0xa) = 0x3;
        }
      }
      else {
        uVar1 = iVar3.field142_0x8e;
        (uVar1 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = iVar3.field142_0x8e;
      (uVar1 + 0xa) = 0x1;
    }
  }
  iVar3.field143_0x92 = 0;
  return;
}

pub unsafe fn FUN_1038_ced6()

{
  return;
}

pub unsafe fn pass1_1038_ceda(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_cd5c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn call_win_proc_1038_d020
               (base_param_6: u16,win_handle_1: HWND16,mut param_3: u32,l_param: HWND16,hwnd_param_4: HWND16,
               win_handle_2: HWND16) -> i32

{
  let mut handle_1: HANDLE16;
  let mut handle_2: HANDLE16;
  let mut handle_3: HANDLE16;
  let mut var1: u16;
  let mut lresult: LRESULT;
  let mut var5: i32;
  let mut var6: *mut u32;
  let mut var7: i32;
  let mut var8: u16;
  let mut fn_ptr_1: *mut *mut code;

  handle_1 = GetProp16(CONCAT22(base_param_6,0x5bd7),hwnd_param_4);
  handle_2 = GetProp16(CONCAT13((base_param_6 >> 0x8),CONCAT12(base_param_6,0x5bd0)),
                       hwnd_param_4);
  var7 = CONCAT22(handle_1,handle_2);
  handle_1 = GetProp16(CONCAT22(base_param_6,0x5be5),hwnd_param_4);
  handle_3 = GetProp16(CONCAT22(base_param_6,0x5bde),hwnd_param_4);
  var6 = CONCAT22(handle_1,handle_3);
  if ((handle_1 | handle_3) != 0) {
    var5 = 0;
    if (l_param == 0x19) {
      fn_ptr_1 = (*var6 + 0x34);
      var5 = (**fn_ptr_1)(s_tile2_bmp_1050_1538,handle_3,handle_1,win_handle_1,param_3);
    }
    else {
      if (l_param == 0x86) {
        fn_ptr_1 = (*var6 + 0x20);
        var1 = (**fn_ptr_1)(s_tile2_bmp_1050_1538,handle_3,handle_1,param_3);
    // TODO: goto LAB_1038_d10e;
      }
      if ((l_param == 0x112) && ((param_3 & 0xfff0) == 0xf140)) {
        lresult = SendMessage16(0x0,0xf140,0x112,&HWND16_1050_0396);
        var1 = (lresult == 0);
    // TODO: goto LAB_1038_d10e;
      }
    }
    if (var5 != 0) {
      return var5;
    }
  }
  if (var7 != 0) {
    lresult = CallWindowProc16(CONCAT22(param_3,win_handle_1),param_3,l_param,hwnd_param_4,handle_2);
    return lresult;
  }
  var1 = 0;//
// LAB_1038_d10e:
  return var1;
}

pub unsafe fn win_prop_op_1038_d118(base_addr_param_4: u16,mut param_2: u32,mut param_3: u32,hwnd_param_3: HWND16)

{
  let mut uVar1: u32;
  let mut cVar2: u8;
  let mut HVar3: HANDLE16;
  let mut HVar4: HANDLE16;
  let mut puStack6: *mut u32;
  let mut uVar2: u32;
  let mut fn_ptr_1: *mut *mut code;

  HVar3 = GetProp16(CONCAT22(base_addr_param_4,0x5bf3),hwnd_param_3);
  HVar4 = GetProp16(CONCAT22(base_addr_param_4,0x5bec),hwnd_param_3);
  puStack6 = CONCAT22(HVar3,HVar4);
  if (param_3 == 0x30) {
    if (param_3 == 0) {
      return;
    }
    SetProp16(param_3,CONCAT22(base_addr_param_4,0x5c06),hwnd_param_3);
    return;
  }
  if (param_3 < 0x310000) {
    cVar2 = (param_3 >> 0x10);
    if (cVar2 == '\x02') {
      if ((HVar3 | HVar4) != 0) {
        uVar1 = *puStack6;
        fn_ptr_1 = uVar1 + 0x6;
        (**fn_ptr_1)(s_tile2_bmp_1050_1538,HVar4,HVar3,param_2,param_3);
        if (puStack6.is_null() == false) {
          fn_ptr_1 = uVar1;
          (**fn_ptr_1)(s_tile2_bmp_1050_1538,HVar4,HVar3,1);
        }
      }
      HVar3 = GetProp16(CONCAT22(base_addr_param_4,0x5bfa),hwnd_param_3);
      if (HVar3 == 0) {
        return;
      }
      DeleteObject16(HVar3);
      RemoveProp16(CONCAT22(base_addr_param_4,0x5c00),hwnd_param_3);
      return;
    }
    if (cVar2 == '\x06') {
      if ((param_3 != 1) && (param_3 != 0x2)) {
        uVar1 = &u16_1050_5bc8;
        (uVar1 + 0x8) = 0;
        return;
      }
      uVar2 = &u16_1050_5bc8;
      (uVar2 + 0x8) = hwnd_param_3;
      return;
    }
  }
  if ((HVar3 | HVar4) != 0) {
    fn_ptr_1 = (*puStack6 + 0xc);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,HVar4,HVar3,param_2,param_3);
  }
  return;
}

pub unsafe fn pass1_1038_d218(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  free_proc_inst_1038_cfda(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn win_ui_op_1038_d2a2(param_1: *mut Struct57,struct_b_param_1: *mut StructB,mut param_3: u16 )

{
  let mut rect: *mut Struct57;
  let mut iVar1: i16;
  let mut hwnd_2: HWND16;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut iVar5: *mut astruct_912;
   let mut struct_b_6: *mut StructB;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;
  let mut l_param: *mut c_char;
  let mut LVar8: LRESULT;
  let mut in_stack_0000fe2e: u16;
  let mut in_stack_0000fe32: u16;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ff58: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ff60: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut w_param: WPARAM16;
  let mut msg: u16;
  let mut id: i16;
  let mut uVar9: u16;
  let mut hwnd: LPVOID = null_mut();
  let mut local_16: u16;
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut uStack10: u16;
  let mut uStack8: u32;
  let mut uStack4: u16;
  let mut paVar5: *mut Struct57;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uStack4 = 0x7;
//   for (uStack10 = 0; struct_b_6 = struct_b_param_1, uVar6 = (struct_b_param_1 >> 0x10),
//       uStack10 < uStack4; uStack10 += 1)
      uStack10 = 0;
      struct_b_6 = struct_b_param_1;
      uVar6 = struct_b_param_1 >> 0x10;
      while uStack10 < uStack4
      {
    iVar5 = (uStack10 * 0xc);
    local_16 = (iVar5 + 0x5c0c);
    uStack20 = (iVar5 + 0x5c0e);
    uStack18 = 0x1;
    uStack16 = 0x1;
    rect = &local_16;
    MapDialogRect16(rect,&DAT_1050_1050);
    mem_op_1000_179c(0x42,param_1);
    uVar3 = param_1 | rect;
    paVar5 = (param_1 & 0xffff0000 | uVar3);
    if (uVar3 == 0) {
      rect = null_mut();
      param_1 = (param_1 & 0xffff0000);
    }
    else {
      pass1_1008_3bd6(paVar5,rect,param_1,0x1,CONCAT22(local_16,uStack20),0x104,0x1020103,
                      CONCAT22(struct_b_6.lpvoid_field_0x8,(iVar5 + 0x5c10)),param_3,in_stack_0000fe2e,
                      in_stack_0000fe32,in_stack_0000ff58,in_stack_0000ff5c,in_stack_0000ff60);
      param_1 = paVar5;
    }
    uStack8 = CONCAT22(param_1,rect);
    if ((uStack10 * 0xc + 0x5c12) == 0) {
      EnableWindow16(0x0,&rect.field11_0x18);
    }
    uStack10 += 1;
  }
  uVar9 = 0x86;
  puVar7 = mixed_1010_20ba(param_1,_u16_1050_0ed0,0x860009,in_stack_0000fe80,in_stack_0000ffa4,
                           in_stack_0000ffaa,in_stack_0000ffae);
  uVar4 = (puVar7 >> 0x10);
  uStack14 = puVar7;
  uStack12 = uVar4;
  iVar1 = pass1_1010_659a(puVar7,uVar9);
  if (iVar1 == 0) {
    hwnd_2 = GetDlgItem16(0x14a,struct_b_6.lpvoid_field_0x8);
    EnableWindow16(0x0,hwnd_2);
    hwnd = struct_b_6.lpvoid_field_0x8;
    msg = 0xc;
    id = 0x144;
    w_param = 0;
    l_param = load_string_1010_847e(_u16_1050_14cc,0x531);
    LVar8 = SendDlgItemMessage16(l_param,w_param,msg,id,hwnd);
    uVar4 = (LVar8 >> 0x10);
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  BVar2 = ShowWindow16(0x5,struct_b_6.lpvoid_field_0x8);
  win_1008_5c7c(BVar2,uVar4,_u16_1050_02a0,0x9a0001);
  (struct_b_6 + 0x7).field0_0x0 = BVar2;
  return;
}
