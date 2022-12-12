
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



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn msg_box_op_1038_c07a(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut local_70c: [u8;0x200] = [0;0x200];
  let mut local_50c: [u8;0x100] = [0;0x100];
  let mut local_40c: [u8;0x402] = [0;0x402];
  let mut uStack10: u32;
  let mut uStack6: u32;

  send_msg_1038_c228(CONCAT22(param_2,param_1));
  uStack6 = load_string_1010_847e(_u16_1050_14cc,0x531);
  if (param_4 == 0x177) {
    pass1_1008_e05e((param_1 + 0x8e),0x2,CONCAT22(param_2,param_1 + 0x19eU),
                    CONCAT22(param_2,param_1 + 0x9e));
    load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x200,local_40c,&DAT_1050_1050)
    ;
    sys_1000_3f9c(CONCAT22(0x1050,local_70c),CONCAT22(0x1050,local_40c),param_1 + 0x19eU);
    load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_50c,&DAT_1050_1050)
    ;
    MessageBox16(0x30,CONCAT22(0x1050,local_50c),CONCAT22(0x1050,local_70c),(param_1 + 0x6));
  }
  else {
    if (param_4 != 0x178) {
      if ((param_4 != 0x178) && (param_4 - 0x179U < 0x2)) {
        set_win_pos_1038_c31a(CONCAT22(param_2,param_1),param_3,param_4);
        return;
      }
      post_win_msg_1040_7b3c((StructC *)CONCAT22(param_2,param_1),param_3,param_4,param_4);
      return;
    }
    uStack10 = CONCAT22(param_2,param_1 + 0x9e);
    uVar2 = param_2;
    iVar1 = pass1_1008_e10c((param_1 + 0x8e),CONCAT22(param_2,param_1 + 0x19e),
                            CONCAT22(param_2,param_1 + 0x9e),param_2,&DAT_1050_1050);
    if (iVar1 == 0) {
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_40c,&DAT_1050_1050);
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_50c,&DAT_1050_1050);
      MessageBox16(0x30,CONCAT22(0x1050,local_50c),CONCAT22(0x1050,local_40c),(param_1 + 0x6)
                  );
      return;
    }
    pass1_1008_e01c((param_1 + 0x8e),CONCAT22(param_2,param_1 + 0x19e),uStack10);
    pass1_1038_af40(param_1,uVar2,_PTR_LOOP_1050_5b7c,(param_1 + 0x8),0x1f);
  }
  PostMessage16(0x0,0x2,0x111,(param_1 + 0x6));
  return;
}



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
  pass1_1008_e320((param_1 + 0x8e),(param_1 & 0xffff0000 | (param_1 + 0x19eU))
                  ,(param_1 & 0xffff0000 | uVar1));
  SetWindowText16(CONCAT22(uVar3,uVar1),(param_1 + 0x9a));
  uVar2 = pass1_1008_e2a4((param_1 + 0x8e),
                          (param_1 & 0xffff0000 | (param_1 + 0x19eU)),pcStack12);
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



pub unsafe fn pass1_1038_c410(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_be4a(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_c4a2(param_1: *mut StructD,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_c52a(param_1: *mut u8,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut astruct_27;
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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
  puStack2070 = (param_2 & 0xffff0000 | (iVar2 + 0x96U));
  pass1_1008_e038((iVar2 + 0x8e),(param_2 & 0xffff0000 | ZEXT24((iVar2 + 0x92))),
                  (param_2 & 0xffff0000 | (iVar2 + 0x96U)));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x400,local_80e,
             &DAT_1050_1050);
  uVar1 = (iVar2 + 0x92);
  wsprintf16(local_40c,CONCAT22(local_80e,0x1050),CONCAT22(*puStack2070,0x1050),
             (*puStack2070 >> 0x10),uVar1,(uVar1 >> 0x10));
  SetDlgItemText16(CONCAT22(0x1050,local_40c),0x17f,(iVar2 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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
      post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 *
pass1_1038_c7b8(param_1: *mut u8,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn destroy_window_1038_c836(param_1: *mut astruct_881,mut param_2: u32,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut local_6: [u8;0x4] = [0;0x4];
  let mut uVar1: u32;

  if (param_3 == 0xfce) {
    puVar1 = pass1_1008_941a(CONCAT22(0x1050,local_6),0x1,0xac);
    win_1008_5c9e(local_6,(puVar1 >> 0x10),_u16_1050_02a0,CONCAT22(0x1050,local_6));
    uVar1 = param_1.field141_0x8e;
    (uVar1 + 0xa) = 0x6;
    DestroyWindow16(param_1.field6_0x6);
    PTR_LOOP_1050_5b80 = null_mut();
    return;
  }
  post_win_msg_1040_7b3c
            ((StructC *)CONCAT22(param_2,param_1),(param_2 >> 0x10),param_3,param_3);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
pub unsafe fn win_ui_op_1038_c89c(struct_b_param_1: *mut StructB)

{
  let mut HVar1: HWND16;
  struct_b_4: *mut StructB;
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
//    if (((iVar1 + -0x5) < 1) || (SBORROW2((iVar1 + -0x5),1))) goto LAB_1038_c93c;
    if (iVar1 != &u16_1050_0008 && 0x0 < (iVar1 + -0x7)) {
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



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1038_cad8(param_1: *mut u8,param_2: *mut astruct_57,mut param_3: u16 ) -> *mut astruct_57

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn show_win_1038_cb5c(mut param_1: u32,struct_b_param_1: *mut StructB,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;
  let mut paVar5: *mut Struct57;
  let mut uVar6: u16;
  let mut paVar7: *mut Struct57;
  struct_b_5: *mut StructB;
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn destroy_window_1038_cc00(StructC *param_1,mut param_2: u16 ,mut param_3: u32,param_4: *mut u8)

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_cd06(param_1: *mut StructD,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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
pub unsafe fn destroy_window_1038_cd88(struct_b_param_1: *mut StructB)

{
  struct_1: *mut StructB;
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn make_proc_inst_1038_cf6c(param_1: *mut u8,param_2: *mut astruct_831)

{
  let mut iVar1: *mut astruct_831;
  let mut uVar1: u16;
pub unsafe fn *pvVar1;

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn free_proc_inst_1038_cfda(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0xd23e;
  iVar1.address_offset_field_0x2 = &u16_1050_1038;
  FreeProcInstance16(*(void **)&iVar1.hfile_0x4);
  FreeProcInstance16(_u16_1050_5bcc);
  iVar1.hfile_0x4 = 0;
  param_1.address_offset_field_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}
