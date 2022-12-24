pub unsafe fn win_msg_1040_a308(param_1: *mut astruct_49, mut param_2: u16, mut param_3: u16, mut param_4: u16 ) -> u32

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut LVar6: LRESULT;
  let mut puVar7: *mut u32;
  let mut pcVar8: *mut c_char;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut WVar9: WPARAM16;
  let mut UVar10: u16;
  let mut uVar11: u16;
  let mut in_stack_0000fff2: u16;
  let mut iStack12: i16;

  uVar3 = (in_EDX >> 0x10);
  SendMessage16(0x0,0x0,0x405,param_4);
  LVar6 = SendMessage16(0x0,0x0,0xb,param_4);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar2 = (iVar4 + 0x90);
  if ((uVar2 + 0x10) == 0) {
    WVar9 = 0;
    UVar10 = 0x401;
    uVar11 = param_4;
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x531);
    SendMessage16(pcVar8,WVar9,UVar10,uVar11);
  }
  else {
    puVar7 = mixed_1010_20ba(CONCAT22(uVar3,(LVar6 >> 0x10)),_u16_1050_0ed0,
                             CONCAT22(in_stack_0000fff2,0x3),in_stack_0000fe9a,in_stack_0000ffbe,
                             in_stack_0000ffc4,in_stack_0000ffc8);
    // for (iStack12 = 0; uVar2 = (iVar4 + 0x90), piVar1 = (uVar2 + 0x10),
    //     *piVar1 != iStack12 && iStack12 <= *piVar1; iStack12 += 1)
    iStack12 = 0;
    uVar2 = iVar4 + 0x90;
    piVar1 = uVar2 + 0x10;
    while *piVar1 != iStack12 && iStack12 <= *piVar1
    {
      WVar9 = 0;
      UVar10 = 0x401;
      uVar2 = (iVar4 + 0x90);
      uVar2 = (uVar2 + 0xc);
      uVar11 = param_4;
      pcVar8 = load_string_1010_ac92
                         (puVar7,(puVar7 >> 0x10),(uVar2 + iStack12 * 0x2));
      SendMessage16(pcVar8,WVar9,UVar10,uVar11);
      iStack12 += 1;
    }
  }
  LVar6 = SendMessage16(0x0,0x1,0xb,param_4);
  return CONCAT22((LVar6 >> 0x10),1);
}





pub unsafe fn call_win_proc_1040_a40e(mut param_1: u16 ,param_2: HWND16,mut param_3: u32,param_4: LPARAM) -> u32

{
  let mut func: LPVOID = null_mut();
  let mut uVar1: u16;
  let mut ppcVar2: *mut *mut code;
  let mut wparam: WPARAM16;
  let mut uVar6: u16;
  let mut uVar3: u32;
  let mut uStack6: u32;
  let mut puVar3: *mut u32;
  let mut uVar5: u32;

  uStack6 = 0;
  wparam = (param_3 >> 0x10);
  if (param_4 == 0x19) {
    ppcVar2 = (_PTR_LOOP_1050_5ee0 + 0x34);
    uStack6 = (**ppcVar2)();
    param_1 = (uStack6 >> 0x10);
  }
  else {
    if (param_4 == 0x86) {
      ppcVar2 = (_PTR_LOOP_1050_5ee0 + 0x20);
      uVar3 = (**ppcVar2)();
      return uVar3;
    }
    if (param_4 == 0x110) {
      uVar3 = win_msg_1040_a308(_PTR_LOOP_1050_5ee0,param_2,param_3,wparam);
      return uVar3;
    }
  }
  if (uStack6 != 0) {
    return uStack6 & 0xffff | param_1 << 0x10;
  }
  uVar6 = (_u16_1050_5bc8 >> 0x10);
  func = (_u16_1050_5bc8 + 0x4);
  uVar1 = (_u16_1050_5bc8 + 0x6);
  if ((uVar1 | func) == 0) {
    return uVar1 << 0x10;
  }
  uVar3 = CallWindowProc16(CONCAT22(param_3,param_2),wparam,param_4,(param_4 >> 0x10),func);
  return uVar3;
}




pub unsafe fn struct_1040_a598(param_1: *mut astruct_259)

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
pub unsafe fn pass1_1040_a5d0(param_1: *mut StructD)

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
pub unsafe fn string_1040_a626(mut param_1: u16 ,param_2: *mut u16,param_3: *mut c_char)

{
  let mut uVar1: u16;

  uVar1 = str_op_1008_60e8(param_1,param_3);
  *param_2 = uVar1;
  (param_2 + 0x2) = param_1;
  return;
}
pub unsafe fn pass1_1040_a640(param_1: *mut Struct57,mut param_2: u32,mut param_3: u16 )

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









pub unsafe fn msg_box_op_1040_a85a(param_1: *mut c_char,mut param_2: u16 ,mut param_3: u32)

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
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}




pub unsafe fn win_ui_dlg_op_1040_a94a(mut param_1: u16 ,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut puVar4: *mut u8;
  let mut uVar5: u16;
  let mut value: *mut u8;
  let mut pcVar6: *mut c_char;
  let mut uVar7: u16;
  let mut HVar8: HWND16;
  let mut value_00: u16;
  let mut puVar9: *mut u8;
  let mut in_register_0000000a: u16;
  let mut uVar10: u32;
  let mut iVar11: i16;
  let mut iVar12: i16;
  let mut unaff_SI: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut bVar15: bool;
  let mut puVar16: *mut u32;
  let mut uVar17: u32;
  let mut in_stack_0000fd7c: u16;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000feaa: u16;
  let mut uStack288: u16;
  let mut iStack278: i16;
  let mut iStack276: i16;
  let mut local_102: [u8;0x100] = [0;0x100];

  puVar16 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                            CONCAT22(unaff_SI,0x3),in_stack_0000fd7c,in_stack_0000fea0,in_stack_0000fea6,
                            in_stack_0000feaa);
  puVar4 = (puVar16 >> 0x10);
  uVar5 = puVar16;
  uVar13 = (param_2 >> 0x10);
  iVar11 = param_2;
  puVar9 = puVar4;
  pass1_1010_c3c2(puVar4,uVar5,puVar4,CONCAT22(0x1050,local_102),(iVar11 + 0x94));
  SetDlgItemText16(CONCAT22(0x1050,local_102),0x1f2,(iVar11 + 0x6));
  pass1_1010_c320(puVar9,uVar5,puVar4,CONCAT22(0x1050,local_102),(iVar11 + 0x94));
  SetDlgItemText16(CONCAT22(0x1050,local_102),0x1774,(iVar11 + 0x6));
  string_op_1010_c446(puVar9,puVar16,CONCAT22(0x1050,local_102),(iVar11 + 0x94));
  value = local_102;
  SetDlgItemText16(CONCAT22(0x1050,value),0x1773,(iVar11 + 0x6));
  pass1_1030_6ddc((iVar11 + 0x94));
  SetDlgItemInt16(0x0,value,0x1f5,(iVar11 + 0x6));
  pass1_1030_6e14((iVar11 + 0x94));
  SetDlgItemInt16(0x0,value,0x1f6,(iVar11 + 0x6));
  if ((iVar11 + 0x98) != 0) {
    struct_1010_dd5e(uVar5,puVar4,(iVar11 + 0x94));
    if ((puVar9 | value) != 0) {
      uVar3 = (iVar11 + 0x94);
      uVar14 = (uVar3 >> 0x10);
      iVar12 = uVar3;
      uVar2 = (iVar12 + 0x26);
      uVar10 = (iVar12 + 0x28);
      iStack276 = 0x1798;
      iStack278 = 0x17c3;
      (iVar11 + 0xea) = 0;
      uVar17 = uVar2;
    //   for (uStack288 = 0x1; uStack288 < 0x25; uStack288 += 1)
      for uStack288 in 1 .. 0x25
      {
        if (uVar2 == 0) {
          value_00 = 0;
          uVar10 = 0;
        }
        else {
          uVar17 = pass1_1020_bae6(uVar17,uVar10,uVar2,CONCAT22(uStack288,(uVar2 >> 0x10)));
          uVar10 = uVar17 >> 0x10;
          value_00 = uVar17;
        }
        uVar7 = uVar10;
        bVar15 = (value + uStack288 * 0x4) != 0;
        if (bVar15) {
          pcVar6 = string_1020_c0d8(uStack288);
          SetDlgItemText16(CONCAT22(uVar10,pcVar6),iStack276,(iVar11 + 0x6));
          SetDlgItemInt16(0x0,(value + uStack288 * 0x4),iStack278,(iVar11 + 0x6));
        }
        uVar7 |= value_00;
        if (uVar7 != 0) {
          if (!bVar15) {
            pcVar6 = string_1020_c0d8(uStack288);
            SetDlgItemText16(CONCAT22(uVar10,pcVar6),iStack276,(iVar11 + 0x6));
          }
          SetDlgItemInt16(0x0,value_00,iStack278,(iVar11 + 0x6));
          iVar12 = (iVar11 + 0xea);
          HVar8 = GetDlgItem16(iStack276,(iVar11 + 0x6));
          (iVar11 + iVar12 * 0x2 + 0x9a) = HVar8;
          piVar1 = (iVar11 + 0xea);
          *piVar1 = *piVar1 + 1;
          iVar12 = (iVar11 + 0xea);
          uVar7 = GetDlgItem16(iStack278,(iVar11 + 0x6));
          (iVar11 + iVar12 * 0x2 + 0x9a) = uVar7;
          piVar1 = (iVar11 + 0xea);
          *piVar1 = *piVar1 + 1;
          bVar15 = true;
        }
        uVar17 = uVar7;
        if (bVar15) {
          iStack276 += 0x1;
          iStack278 += 0x1;
        }
      }
    }
  }
  return;
}






pub unsafe fn pass1_1040_ac84(param_1: *mut u8,param_2: *mut Struct57,mut param_3: u16 )

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




pub unsafe fn pass1_1040_ace8(param_1: *mut StructD)

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





pub unsafe fn msg_box_ui_op_1040_ad66(param_1: *mut c_char,mut param_2: u16 ,mut param_3: u32)

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
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}




pub unsafe fn win_ui_op_1040_ae04(mut param_1: u16 ,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut bVar2: bool;
  let mut pWVar3: *mut WORD;
  let mut iVar4: i16;
  let mut pcVar5: *mut c_char;
  let mut lVar6: i32;
  let mut puVar7: *mut u8;
  let mut uVar8: u16;
  let mut in_register_0000000a: u16;
  let mut iVar9: i16;
   let mut plVar10: *mut i32;
  let mut unaff_SI: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut puVar13: *mut u32;
  let mut uVar14: u32;
  let mut uVar15: u32;
  let mut lp_string: *mut c_char;
  let mut uVar16: u32;
  let mut in_stack_0000fd84: u16;
  let mut in_stack_0000fea8: u16;
  let mut in_stack_0000feae: u16;
  let mut in_stack_0000feb2: u16;
  let mut iStack280: i16;
  let mut local_102: [u16;0x80] = [0;0x80];

  puVar13 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                            CONCAT22(unaff_SI,0x3),in_stack_0000fd84,in_stack_0000fea8,in_stack_0000feae,
                            in_stack_0000feb2);
  puVar7 = (puVar13 >> 0x10);
  uVar11 = (param_2 >> 0x10);
  iVar9 = param_2;
  pass1_1010_c3c2(puVar7,puVar13,puVar7,CONCAT22(0x1050,local_102),(iVar9 + 0x94));
  pWVar3 = local_102;
  SetDlgItemText16(CONCAT22(0x1050,pWVar3),0x1778,(iVar9 + 0x6));
  uVar14 = struct_op_1030_73a8((iVar9 + 0x94),pWVar3,puVar7);
  iVar4 = uVar14 + 0x20;
  uVar15 = pass1_1030_8326();
  uVar16 = uVar15 >> 0x10;
  iVar1 = 0;
  bVar2 = false;
//   for (iStack280 = 0; uVar8 = uVar16, iStack280 < 0xa; iStack280 += 1)
  iStack280 = 0;
  uVar8 = uVar16;
  while iStack280 < 0xa
  {
    uVar12 = ((uVar14 & 0xffff0000) >> 0x10);
    if (((iStack280 * 0xc + iVar4 + 0x2) | (iStack280 * 0xc + iVar4)) != 0) {
      plVar10 = (iStack280 * 0xc + iVar4);
      pcVar5 = string_op_1020_c222((plVar10 + 1));
      SetDlgItemText16(CONCAT22(uVar8,pcVar5),iVar1 + 0x1d2,(iVar9 + 0x6));
      lVar6 = *plVar10 - uVar15;
      wsprintf16(local_102,0x5ef41050,0xf4,CONCAT22(lVar6,0x1050),(lVar6 >> 0x10));
      SetDlgItemText16(CONCAT22(0x1050,local_102),iVar1 + 0x1dc,(iVar9 + 0x6));
      uVar16 = unk_load_str_op_1010_8c96
                         (uVar8,(iVar9 + 0x98),CONCAT22(0x1050,local_102),
                          uVar14 & 0xffff0000 | ZEXT24(plVar10));
      uVar16 &= 0xffff;
      SetDlgItemText16(CONCAT22(0x1050,local_102),iVar1 + 0x1e6,(iVar9 + 0x6));
      iVar1 += 0x1;
      bVar2 = true;
    }
    iStack280 += 1;
  }
  if (!bVar2) {
    lp_string = load_string_1010_847e(_u16_1050_14cc,0x531);
    SetDlgItemText16(lp_string,0x1d2,(iVar9 + 0x6));
  }
  return;
}



pub unsafe fn pass1_1040_b040(param_1: *mut Struct57,mut param_2: u32,mut param_3: u16 )

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
