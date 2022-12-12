
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1040_109c(param_1: *mut u8,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut uVar1: u32;
  let mut bVar2: bool;
  let mut iVar3: i16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut uVar5: u32;
  let mut paVar6: *mut Struct57;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000fff2: u16;

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  bVar2 = false;
  if (param_5 == 0x1c1) {
    (param_2 + 0x96) = 0x2;
    bVar2 = true;
  }
  else if (param_5 == 0x1c2) {
    (param_2 + 0x96) = 0x1;
    bVar2 = true;
  }
  else {
    if (param_5 != 0x1830) {
      post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
      return;
    }
    paVar6 =
             mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(in_stack_0000fff2,0x32),in_stack_0000fe9a,
                             in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
    uVar5 = paVar4 & 0xffff0000 | paVar6 >> 0x10;
    iVar3 = paVar6;
    uVar1 = (param_2 + 0x92);
    ui_op_1010_79aa(paVar6,0xfb6,(uVar1 + 0x6));
    if (iVar3 == 0) {
      uVar1 = (param_2 + 0x92);
      unk_win_op_1010_7300(uVar5,paVar6,0x0,0xc,(uVar1 + 0x6));
    }
  }
  if (bVar2) {
    uVar1 = (param_2 + 0x8e);
    (uVar1 + 0xa) = (param_2 + 0x96);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1040_1152(param_1: *mut u8,mut param_2: i16,mut param_3: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut puVar6: *mut u32;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000fff4: u16;

  paVar3 = CONCAT22(in_register_0000000a,param_1);
  if ((param_2 + 0x92) != 0) {
    uVar2 = (param_2 + 0x8e);
    uVar1 = (uVar2 + 0xa);
    puVar6 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,CONCAT22(in_stack_0000fff4,0x3),in_stack_0000fe9c,
                             in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
    uVar2 = (param_2 + 0x92);
    uVar5 = (uVar2 >> 0x10);
    iVar4 = uVar2;
    pass1_1010_ae92(puVar6,uVar1,(iVar4 + 0xa),(iVar4 + 0x6),
                    paVar3 & 0xffff0000 | puVar6 >> 0x10);
  }
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  PTR_LOOP_1050_5b80 = null_mut();
  return;
}



pub unsafe fn pass1_1040_11ac(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1040_0e86(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 *
pass1_1040_123e(param_1: *mut StructD,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfd1,param_6);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  param_2.field0_0x0 = 0x17b0;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x46),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1040_1290(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x17b0;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_ui_op_1040_12bc(param_1: u8,struct_b_param_1: *mut StructB)

{
  let mut uVar1: u32;
  let mut HVar2: HWND16;
  struct_b_3: *mut StructB;
  let mut uVar3: u16;
  let mut lparam: *mut c_char;
  let mut local_54: [u8;0x52] = [0;0x52];

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar3 = (struct_b_param_1 >> 0x10);
  struct_b_3 = struct_b_param_1;
  uVar1 = &struct_b_3[0x7].field1_0x2;
  sys_1000_3f9c(CONCAT22(0x1050,local_54),s__u_1050_5cd4,(uVar1 + 0xa));
  HVar2 = GetDlgItem16(0xfd2,struct_b_3.lpvoid_field_0x8);
  SendMessage16(CONCAT22(0x1050,local_54),0x0,0xc,HVar2);
  SetFocus16(HVar2);
  SendMessage16(-0x10000,0x0,0x401,HVar2);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  lparam = load_string_1010_847e(_u16_1050_14cc,0x531);
  HVar2 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x5,struct_b_3.lpvoid_field_0x8);
  send_msg_1040_1696(struct_b_param_1,HVar2);
  SendMessage16(lparam,0xffff,0x40d,HVar2);
  HVar2 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x4,struct_b_3.lpvoid_field_0x8);
  send_msg_1040_1696(struct_b_param_1,HVar2);
  SendMessage16(lparam,0xffff,0x40d,HVar2);
  ShowWindow16(0x5,struct_b_3.lpvoid_field_0x8);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_msg_op_1040_13b2(param_1: *mut astruct_892,mut param_2: i16)

{
  let mut HVar1: HWND16;
  let mut uVar4: u16;
  let mut iVar4: i16;
  let mut puVar5: *mut u8;
  let mut iVar5: i16;
  let mut puVar4: *mut c_char;
  let mut puVar6: *mut u8;
  let mut puVar7: *mut u8;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  let mut paVar2: *mut Struct57;
  struct_7: *mut astruct_892;
  let mut iVar6: i16;
  let mut struct_5_lo: u16;
  let mut uVar6: u16;
  let mut lresult_4: LRESULT;
  let mut pcVar6: *mut c_char;
  let mut puStack266: *mut u32;
  let mut local_100: [u8;0x52] = [0;0x52];
  let mut local_aa: [u8;0x52] = [0;0x52];
  let mut uStack88: u16;
  let mut handle_86: HWND16;
  let mut local_54: [u8;0x52] = [0;0x52];
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut fn_ptr_1: *mut *mut code;

  uVar5 = (in_EDX >> 0x10);
  struct_7 = param_1;
  struct_5_lo = (param_1 >> 0x10);
  if (param_2 != 0) {
    handle_86 = GetDlgItem16(0xfd2,struct_7.hwnd_0x6);
    SendMessage16(CONCAT22(0x1050,local_54),0x51,0xd,handle_86);
    uStack88 = pass1_1000_3e2c(CONCAT22(0x1050,local_54));
    HVar1 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x4,struct_7.hwnd_0x6);
    lresult_4 = SendMessage16(0x0,0x0,0x407,HVar1);
    if (lresult_4 != 0xffff) {
      SendMessage16(CONCAT22(0x1050,local_aa),lresult_4,0x408,HVar1);
    }
    HVar1 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x5,struct_7.hwnd_0x6);
    lresult_4 = SendMessage16(0x0,0x0,0x407,HVar1);
    if (lresult_4 != 0xffff) {
      SendMessage16(CONCAT22(0x1050,local_100),lresult_4,0x408,HVar1);
    }
    pcVar6 = load_string_1010_847e(_u16_1050_14cc,0x531);
    paVar2 = CONCAT22(uVar5,local_aa);
    uVar4 = pass1_1000_3d7a(CONCAT22(0x1050,local_aa),CONCAT22(0x1050,local_100));
    if (uVar4 != 0) {
      uVar4 = pass1_1000_3d7a(CONCAT22(0x1050,local_aa),pcVar6);
      if (uVar4 != 0) {
        uVar4 = pass1_1000_3d7a(CONCAT22(0x1050,local_100),pcVar6);
        if (uVar4 != 0) {
          pass1_1010_531c(local_aa,paVar2,struct_7.field141_0x8e,CONCAT22(0x1050,local_aa));
          puVar5 = local_100;
          pass1_1010_52fc(puVar5,paVar2,struct_7.field141_0x8e,CONCAT22(0x1050,puVar5));
          pass1_1010_5120(puVar5,paVar2,struct_7.field141_0x8e,uStack88);
          if (puVar5.is_null()) {
            mem_op_1000_179c(0xb4,paVar2);
            puVar7 = (paVar2 | puVar5);
            uVar3 = paVar2 & 0xffff0000 | ZEXT24(puVar7);
            if (puVar7.is_null()) {
              iVar5 = 0;
              uVar5 = 0;
            }
            else {
              iVar5 = string_1040_8520(uVar3,CONCAT22(paVar2,puVar5),HWND16_1050_0396,0x20030,
                                       0x7d2057b);
              uVar5 = uVar3;
            }
            fn_ptr_1 = (CONCAT13((uVar5 >> 0x8),CONCAT12(uVar5,iVar5)) +
                                0x74);
            (**fn_ptr_1)();
            return;
          }
          uVar1 = struct_7.field141_0x8e;
          uVar2 = struct_7.field141_0x8e;
          uVar6 = (uVar2 >> 0x10);
          iVar6 = uVar2;
          uVar3 = struct_7.field141_0x8e;
          pass1_1028_8d9e(CONCAT22(0x1050,&stack0xfdd2),(uVar3 + 0xa),
                          (uVar1 + 0x12),
                          (iVar6 + 0x16) & 0xffff | (iVar6 + 0x18) << 0x10);
          fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&stack0xfdd2));
          pass1_1028_8dec(CONCAT22(0x1050,&stack0xfdd2));
      // TODO: goto LAB_1040_1619;
        }
      }
    }
    mem_op_1000_179c(0xb4,paVar2);
    puVar6 = (paVar2 | uVar4);
    uVar3 = paVar2 & 0xffff0000 | ZEXT24(puVar6);
    if (puVar6.is_null()) {
      iVar4 = 0;
      uVar5 = 0;
    }
    else {
      iVar4 = string_1040_8520(uVar3,CONCAT22(paVar2,uVar4),HWND16_1050_0396,0x20030,0x755057b);
      uVar5 = uVar3;
    }
    puStack266 = CONCAT22(uVar5,iVar4);
    fn_ptr_1 = (*puStack266 + 0x74);
    (**fn_ptr_1)();
  }//
// LAB_1040_1619:
  DestroyWindow16(struct_7.hwnd_0x6);
  return;
}



pub unsafe fn set_win_pos_1040_162a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32) -> u32

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  let mut iStack6: i16;

  if ((param_4 != s_vrpal_bmp_1050_183a + 0x5) && (param_4 != s_vrpal_bmp_1050_183a + 0x4)) {
    BVar2 = post_win_msg_1040_7b3c
                      ((StructC *)CONCAT22(param_3,param_2),param_3,param_4,param_4);
    return CONCAT22(param_1,BVar2);
  }
  if (param_4 == 0x7) {
    GetWindowRect16(CONCAT22(0x1050,&stack0xfff6),param_3);
    SetWindowPos16(0x2,0x50,iStack6 - param_3,0x0,0x0,0x0,param_3);
  }
  else if ((param_4 != 0x9) && (param_4 != 0xa)) {
    uVar1 = 0;
// TODO: goto LAB_1040_164d;
  }
  uVar1 = 0x1;//
// LAB_1040_164d:
  return uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn send_msg_1040_1696(param_1: *mut StructB,mut param_2: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut puVar3: *mut u16;
  let mut puVar4: *mut u8;
  let mut puVar5: *mut u8;
  let mut uVar6: u16;
  let mut LVar7: LRESULT;
  let mut pcVar8: *mut c_char;
  let mut WVar9: WPARAM16;
  let mut UVar10: u16;
  let mut uVar11: u16;
  let mut uStack18: u16;
  let mut local_4: u16;

  SendMessage16(0x0,0x0,0x40b,param_2);
  LVar7 = SendMessage16(0x0,0x0,0xb,param_2);
  puVar4 = (LVar7 >> 0x10);
  local_4 = 0;
  puVar3 = &local_4;
  uVar6 = (param_1 >> 0x10);
  pass1_1010_519a(puVar4,(param_1 + 0x8e),CONCAT22(0x1050,puVar3));
  puVar5 = puVar4;
  for uStack18 in 0 .. local_4 {
    uVar1 = (puVar3 + uStack18 * 0x2);
    WVar9 = 0;
    UVar10 = 0x403;
    uVar2 = (param_1 + 0x8e);
    uVar11 = param_2;
    pcVar8 = string_1010_5286(uVar1,puVar5,uVar2,(uVar2 >> 0x10),uVar1);
    LVar7 = SendMessage16(pcVar8,WVar9,UVar10,uVar11);
    puVar5 = (LVar7 >> 0x10);
    fn_ptr_1000_17ce(pcVar8);
  }
  WVar9 = 0;
  UVar10 = 0x40a;
  uVar11 = param_2;
  pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x531);
  SendMessage16(pcVar8,WVar9,UVar10,uVar11);
  SendMessage16(0x0,0x1,0xb,param_2);
  return;
}
pub unsafe fn FUN_1040_1786()

{
  return;
}



pub unsafe fn pass1_1040_178a(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1040_1290(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1040_181c(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfbb,param_6);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  iVar1[0x1].field2_0x4 = 0;
  iVar1[0x1].field3_0x6 = 0;
  param_2.field0_0x0 = 0x1c48;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x2),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1040_1876(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x1c48;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn show_win_1040_18a2(struct_b_param_1: *mut StructB)

{
  let mut uVar1: u32;
  struct_b_2: *mut StructB;
  let mut uVar2: u16;
  let mut local_304: [u16;0x80] = [0;0x80];
  let mut local_204: [u8;0x100] = [0;0x100];
  let mut local_104: [u8;0x100] = [0;0x100];
  let mut uStack4: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  check_dialog_btn_1040_1afe(struct_b_param_1);
  struct_b_2 = struct_b_param_1;
  uVar2 = (struct_b_param_1 >> 0x10);
  if (PTR_LOOP_1050_13ae.is_null() == false) {
    if (PTR_LOOP_1050_13ae == &u16_1050_0002) {
      uStack4 = 0x621;
    }
    else if (PTR_LOOP_1050_13ae == (&u16_1050_0002 + 1)) {
      uStack4 = 0x622;
    }
    else if (PTR_LOOP_1050_13ae == &u32_1050_0004) {
      uStack4 = 0x623;
    }
    else {
      uStack4 = 0x620;
    }
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_204,&DAT_1050_1050);
    wsprintf16(local_304,0x5cda1050,CONCAT22(local_204,0x1050),&DAT_1050_1050);
    SetDlgItemText16(CONCAT22(0x1050,local_304),0xfe0,struct_b_2.lpvoid_field_0x8);
    uVar1 = &struct_b_2[0x7].field1_0x2;
    if ((uVar1 + 0x82) == 0) {
      uStack4 = 0x627;
    }
    else {
      uStack4 = 0x626;
    }
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_204,&DAT_1050_1050);
    wsprintf16(local_304,0x5cdf1050,CONCAT22(local_204,0x1050),&DAT_1050_1050);
    SetDlgItemText16(CONCAT22(0x1050,local_304),0xfdf,struct_b_2.lpvoid_field_0x8);
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  ShowWindow16(0x5,struct_b_2.lpvoid_field_0x8);
  return;
}
pub unsafe fn unk_win_ui_op_1040_19ea(param_1: *mut astruct_32,mut param_2: i16,param_3: *mut u8)

{
  let mut pSVar1: *mut StructD;
  let mut UVar2: u16;
  pstruct32_1: *mut astruct_32;
  pstruct_32_hi: *mut astruct_32;

  pstruct32_1 = param_1;
  pstruct_32_hi = (param_1 >> 0x10);
  if (param_2 != 0) {
    UVar2 = IsDlgButtonChecked(0xfdb,pstruct32_1.field6_0x6);
    pass1_1010_5d9c(param_3,pstruct32_1.pstructd_0x8e,UVar2);
    UVar2 = IsDlgButtonChecked(0xfdc,pstruct32_1.field6_0x6);
    pSVar1 = pstruct32_1.pstructd_0x8e;
    (pSVar1 + 0x20) = UVar2;
    UVar2 = IsDlgButtonChecked(0xfdd,pstruct32_1.field6_0x6);
    pSVar1 = pstruct32_1.pstructd_0x8e;
    (pSVar1 + 0x74) = UVar2;
    UVar2 = IsDlgButtonChecked(0xfde,pstruct32_1.field6_0x6);
    pSVar1 = pstruct32_1.pstructd_0x8e;
    (pSVar1 + 0x72) = UVar2;
    if (pstruct32_1.field142_0x92 != 0) {
      pSVar1 = pstruct32_1.pstructd_0x8e;
      pass1_1000_4906((pSVar1 & 0xffff0000 | (pSVar1 + 0x22)),NULL,0x40);
    }
    if (pstruct32_1.field143_0x94 != 0) {
      pass1_1010_60a0(pstruct32_1.pstructd_0x8e);
    }
  }
  DestroyWindow16(pstruct32_1.field6_0x6);
  return;
}



pub unsafe fn pass1_1040_1ab0(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32) -> u32

{
  let mut BStack6: bool;
  let mut uStack4: u16;

  BStack6 = 0;
  uStack4 = 0;
  if (param_5 == 0x1831) {
    (param_2 + 0x92) = 0x1;
    (param_2 + 0x94) = 0x1;
    check_dialog_btn_1040_1b8a((StructC *)CONCAT22(param_3,param_2));
  }
  else {
    BStack6 = post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
    uStack4 = param_1;
  }
  return CONCAT22(uStack4,BStack6);
}
pub unsafe fn check_dialog_btn_1040_1afe(param_1: *mut StructB)

{
  let mut check: u16;
  let mut check_00: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;
  iVar3: *mut StructB;
  let mut uVar3: u16;
  let mut check_01: u16;

  uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar1 = &iVar3[0x7].field1_0x2;
  uVar2 = &iVar3[0x7].field1_0x2;
  check = (uVar2 + 0x20);
  uVar2 = &iVar3[0x7].field1_0x2;
  check_00 = (uVar2 + 0x74);
  uVar2 = &iVar3[0x7].field1_0x2;
  check_01 = (uVar2 + 0x72);
  CheckDlgButton16((uVar1 + 0x1e),0xfdb,iVar3.lpvoid_field_0x8);
  CheckDlgButton16(check_00,0xfdd,iVar3.lpvoid_field_0x8);
  CheckDlgButton16(check_01,0xfde,iVar3.lpvoid_field_0x8);
  CheckDlgButton16(check,0xfdc,iVar3.lpvoid_field_0x8);
  return;
}
pub unsafe fn check_dialog_btn_1040_1b8a(StructC *param_1)

{
  let mut check: u16;
  let mut check_00: u16;
  let mut check_01: u16;
  StructC *iVar1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = (StructC *)param_1;
  check_00 = pass1_1010_60b4();
  pass1_1010_60c6();
  check_01 = pass1_1010_60c0();
  pass1_1010_60ba();
  CheckDlgButton16(check_00,0xfdb,iVar1.field6_0x6);
  CheckDlgButton16(check_01,0xfdd,iVar1.field6_0x6);
  CheckDlgButton16(0xfde,0xfde,iVar1.field6_0x6);
  check = iVar1.field6_0x6;
  CheckDlgButton16(check,0xfdc,check);
  return;
}
pub unsafe fn FUN_1040_1c1e()

{
  return;
}



pub unsafe fn pass1_1040_1c22(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1040_1876(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1040_1cb4(param_1: *mut StructD,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar2: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar2: *mut Struct57;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;
  ppuVar3: *mut *mut u8;

  paVar1 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xe8,param_6);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  (iVar2 + 1) = 0;
  iVar2[0x1].field2_0x4 = 0;
  param_2.field0_0x0 = 0x1eee;
  iVar2.field1_0x2 = &PTR_LOOP_1050_1040;
  ppuVar3 = CONCAT22(unaff_BP,0x2);
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,ppuVar3,in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  (iVar2 + 1).field0_0x0 = puVar2;
  iVar2[0x1].field1_0x2 = (puVar2 >> 0x10);
  puVar2 = mixed_1010_20ba((paVar1 & 0xffff0000 | puVar2 >> 0x10),_u16_1050_0ed0,
                           CONCAT22((ppuVar3 >> 0x10),0x37),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar2;
  iVar2[0x1].field3_0x6 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1040_1d24(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x1eee;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub unsafe fn show_win_1040_1d50(param_1: *mut StructB)

{
  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(0x5,(param_1 + 0x6));
  return;
}
pub unsafe fn unk_win_ui_op_1040_1d7a(param_1: *mut astruct_33,mut param_2: i16)

{
  let mut UVar2: u16;
  let mut UVar1: u16;
  let mut iVar3: *mut astruct_33;
  let mut uVar3: *mut astruct_33;
  let mut uVar1: u32;

  iVar3 = param_1;
  uVar3 = (param_1 >> 0x10);
  if ((param_2 != 0) && (uVar1 = iVar3.field141_0x8e, (uVar1 + 0x72) != 0)) {
    UVar2 = IsDlgButtonChecked(0xe1,iVar3.hwnd_0x6);
    if (UVar2 != 0) {
      pass1_1008_a930(iVar3.field142_0x92,0x1d5);
    }
    UVar1 = IsDlgButtonChecked(0xe2,iVar3.hwnd_0x6);
    if (UVar1 != 0) {
      pass1_1008_a930(iVar3.field142_0x92,0x1d6);
    }
    UVar1 = IsDlgButtonChecked(0xe3,iVar3.hwnd_0x6);
    if (UVar1 != 0) {
      pass1_1008_a930(iVar3.field142_0x92,0x1d7);
    }
    UVar1 = IsDlgButtonChecked(0xe5,iVar3.hwnd_0x6);
    if (UVar1 != 0) {
      pass1_1008_a930(iVar3.field142_0x92,0x1d8);
    }
    UVar1 = IsDlgButtonChecked(0xe6,iVar3.hwnd_0x6);
    if (UVar1 != 0) {
      pass1_1008_a930(iVar3.field142_0x92,0x1e2);
    }
    UVar1 = IsDlgButtonChecked(0xe7,iVar3.hwnd_0x6);
    if (UVar1 != 0) {
      pass1_1008_a930(iVar3.field142_0x92,0x1dc);
    }
    return;
  }
  DestroyWindow16(iVar3.hwnd_0x6);
  return;
}



pub unsafe fn pass1_1040_1e80(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32) -> u32

{
  let mut BStack6: bool;
  let mut uStack4: u16;

  BStack6 = 0;
  uStack4 = 0;
  if (param_5 == 0xe4) {
    pass1_1008_a9ec((param_2 + 0x92));
  }
  else {
    BStack6 = post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
    uStack4 = param_1;
  }
  return CONCAT22(uStack4,BStack6);
}
pub unsafe fn FUN_1040_1ec4()

{
  return;
}



pub unsafe fn pass1_1040_1ec8(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1040_1d24(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar6
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1040_1f5a(param_1: *mut astruct_57,mut param_2: u16 ,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut paVar3: *mut Struct57;
  let mut unaff_DI: u16;
  let mut unaff_CS: u16;
  let mut uVar4: u32;
  let mut puVar5: *mut u32;
  let mut paVar6: *mut astruct_27;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa6: u16;
  let mut puVar7: *mut u32;
  let mut puVar8: *mut u32;
  let mut uVar9: u16;
  let mut local_16: u32;
  let mut uStack18: u32;
  let mut iVar6: *mut Struct57;
  let mut uVar6: u16;

  iVar6 = param_1;
  uVar6 = (param_1 >> 0x10);
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfcf,param_2);
  (iVar6 + 1) = 0;
  iVar6[0x1].field10_0x14 = 0;
  iVar6[0x1].field11_0x18 = 0;
  param_1.field0_0x0 = 0x237e;
  iVar6.field1_0x2 = &PTR_LOOP_1050_1040;
  uVar2 = FUN_1010_830a(0x0,param_3,unaff_CS,_u16_1050_14cc,0x1cc);
  (iVar6 + 1).field0_0x0 = uVar2;
  iVar6[0x1].field1_0x2 = param_3;
  uVar4 = pass1_1008_4772(CONCAT22(param_3,(iVar6 + 1).field0_0x0));
  paVar3 = (param_3 & 0xffff0000 | uVar4 >> 0x10);
  uVar9 = (uVar4 >> 0x10);
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,CONCAT22(unaff_DI,0x48),in_stack_0000fe78,
                           in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
  local_16 = CONCAT22((uVar4 + 0x8) + 0xa,0xa);
  uStack18 = CONCAT22(0x1d6,(uVar4 + 0x4) + -0xa);
  iVar6[0x1].field2_0x4 = local_16;
  iVar6[0x1].field4_0x8 = uStack18;
  iVar6[0x1].field6_0xc = local_16;
  iVar6[0x1].field8_0x10 = uStack18;
  puVar1 = &iVar6[0x1].field7_0xe;
  *puVar1 = *puVar1 + 0x14;
  puVar8 = &iVar6[0x1].field10_0x14;
  puVar7 = &iVar6[0x1].field11_0x18;
  uVar9 = uVar6;
  paVar6 =
           mixed_1010_20ba((paVar3 & 0xffff0000 | puVar5 >> 0x10),_u16_1050_0ed0,
                           CONCAT22(puVar7,0x2b),in_stack_0000fe70,in_stack_0000ff94,in_stack_0000ff9a,
                           in_stack_0000ff9e);
  pass1_1010_0538(paVar6,CONCAT22(uVar6,puVar7),CONCAT22(uVar9,puVar8));
  return;
}
