
pub fn enable_win_1038_806a(mut param_1: u16 ,param_2: *mut astruct_902)

{
  let mut HVar1: HWND16;
  let mut BVar2: bool;
  iVar3: *mut astruct_902;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut uVar6: u32;

  uVar3 = (param_2 >> 0x10);
  iVar3 = param_2;
  HVar1 = GetDlgItem16(0x1,iVar3.field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(0x1858,iVar3.field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(0x1859,iVar3.field6_0x6);
  BVar2 = EnableWindow16(0x0,HVar1);
  uVar4 = pass1_1008_b820(BVar2,param_1,iVar3.field147_0x94);
  if (uVar4 != 0) {
    uVar4 = pass1_1008_b340(iVar3.field147_0x94);
    uVar5 = pass1_1008_b366(iVar3.field147_0x94);
    uVar6 = pass1_1008_b47a(iVar3.field147_0x94);
    if (((uVar4 != 0) && (uVar5 != 0)) && (uVar6 != 0)) {
      HVar1 = GetDlgItem16(0x1,iVar3.field6_0x6);
      EnableWindow16(0x1,HVar1);
      HVar1 = GetDlgItem16(0x1858,iVar3.field6_0x6);
      EnableWindow16(0x1,HVar1);
    }
    if (uVar4 != 0) {
      HVar1 = GetDlgItem16(0x1859,iVar3.field6_0x6);
      EnableWindow16(0x1,HVar1);
    }
  }
  return;
}



pub unsafe fn send_dlg_item_msg_1038_8164(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u8,mut param_4: u16 ) -> u16

{
  let mut LVar1: LRESULT;

  *param_3 = '\0';
  LVar1 = SendDlgItemMessage16(0x0,0x0,0x409,param_4,(param_1 + 0x6));
  if ((LVar1 != -1) &&
     (LVar1 = SendDlgItemMessage16((LPARAM)param_3,LVar1,0x40a,param_4,(param_1 + 0x6)),
     LVar1 != -1)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_op_1038_81be(param_1: *mut c_char,mut param_2: u16 ,param_3: *mut astruct_903)

{
  short in_buf_len_5;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u16;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_206,(short)&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  uVar2 = (param_3 >> 0x10);
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn set_win_text_1038_8358(mut param_1: u16 ,param_2: *mut astruct_903)

{
  let mut pcVar1: *mut c_char;
  let mut uVar2: u16;
  let mut uVar4: u16;
  let mut uVar3: u16;
  let mut local_30a: [u8;0x102] = [0;0x102];
  let mut local_208: [u8;0x100] = [0;0x100];
  let mut local_108: [u8;0x100] = [0;0x100];
  let mut uStack8: u32;
  let mut HStack4: HWND16;
  let mut uVar1: u32;

  uVar3 = (param_2 >> 0x10);
  uVar4 = param_2;
  HStack4 = GetDlgItem16(0x1857,(uVar4 + 0x6));
  uStack8 = pass1_1008_b820(HStack4,param_1,(uVar4 + 0x94));
  if (uStack8 == 0) {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_30a,(short)&DAT_1050_1050);
    pcVar1 = local_30a;
  }
  else {
    uVar2 = send_dlg_item_msg_1038_8164(uVar4,uVar3,CONCAT22(0x1050,local_108),0x1854);
    if (uVar2 == 0) {
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_208,(short)&DAT_1050_1050);
    }
    else {
      load_string_1008_b65a((uVar4 + 0x94),local_208,CONCAT22(local_108,0x1050),&DAT_1050_1050);
    }
    pcVar1 = local_208;
  }
  SetWindowText16(CONCAT22(0x1050,pcVar1),HStack4);
  return;
}
pub fn send_dlg_item_msg_1038_8400(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  let mut lVar1: i32;
  let mut local_a: [u8;0x8] = [0;0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),param_3);
  while( true ) {
    lVar1 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (lVar1 == 0) break;
    SendDlgItemMessage16(*(LPARAM *)(lVar1 + 0x4),0x0,0x401,param_4,(param_1 + 0x6));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT send_dlg_item_msg_1038_844a(param_1: *mut astruct_903)

{
  let mut hwnd: HWND16;
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut uVar3: u16;
  uVar4: *mut astruct_903;
  let mut uVar5: u16;
  let mut LVar6: LRESULT;
  let mut local_108: [u8;0x102] = [0;0x102];
  let mut uStack6: u32;

  uVar5 = (param_1 >> 0x10);
  uVar4 = param_1;
  SendDlgItemMessage16(0x0,0x0,0xb,0x1854,uVar4.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0xb,0x1855,uVar4.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0xb,0x1856,uVar4.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x1854,uVar4.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x1855,uVar4.field6_0x6);
  LVar6 = SendDlgItemMessage16(0x0,0x0,0x405,0x1856,uVar4.field6_0x6);
  uStack6 = pass1_1008_b820(LVar6,(LVar6 >> 0x10),uVar4.field147_0x94);
  uVar2 = (uStack6 >> 0x10) | uStack6;
  if (uStack6 == 0) {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_108,(short)&DAT_1050_1050);
    SendDlgItemMessage16(CONCAT22(0x1050,local_108),0x0,0x401,0x1854,uVar4.field6_0x6);
    SendDlgItemMessage16(0x0,0x1,0xb,0x1854,uVar4.field6_0x6);
    SendDlgItemMessage16(0x0,0x1,0xb,0x1855,uVar4.field6_0x6);
    LVar6 = SendDlgItemMessage16(0x0,0x1,0xb,0x1856,uVar4.field6_0x6);
    uVar3 = (LVar6 >> 0x10);
    hwnd = GetDlgItem16(0x1857,uVar4.field6_0x6);
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_108,(short)&DAT_1050_1050);
    BVar1 = SetWindowText16(CONCAT22(0x1050,local_108),hwnd);
    return CONCAT22(uVar3,BVar1);
  }
  send_dlg_item_msg_1038_8400(uVar4,uVar5,uStack6,0x1854);
  set_win_text_1038_8358(uVar2,param_1);
  SendDlgItemMessage16(0x0,0x1,0xb,0x1854,uVar4.field6_0x6);
  SendDlgItemMessage16(0x0,0x1,0xb,0x1855,uVar4.field6_0x6);
  LVar6 = SendDlgItemMessage16(0x0,0x1,0xb,0x1856,uVar4.field6_0x6);
  return LVar6;
}



// WARNING: Could not reconcile some variable overlaps

pub unsafe fn send_dlg_item_msg_1038_8618s(mut param_1: u16 ,param_2: *mut astruct_903) -> u16

{
  let mut in_AX: i16;
  let mut uVar1: u16;
  let mut puVar2: *mut u8;
  let mut puVar3: *mut u8;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut LVar6: LRESULT;
  let mut l_param: u32;
  let mut uVar7: u32;
  let mut local_106: [u8;0x100] = [0;0x100];
  let mut uStack6: u32;

  uVar5 = (param_2 >> 0x10);
  uVar4 = param_2;
  uStack6 = pass1_1008_b820(in_AX,param_1,(uVar4 + 0x94));
  uVar1 = uStack6;
  if (uStack6 != 0) {
    uVar1 = send_dlg_item_msg_1038_8164(uVar4,uVar5,CONCAT22(0x1050,local_106),0x1854);
    if (uVar1 != 0) {
      SendDlgItemMessage16(0x0,0x0,0xb,0x1855,(uVar4 + 0x6));
      SendDlgItemMessage16(0x0,0x0,0xb,0x1856,(uVar4 + 0x6));
      SendDlgItemMessage16(0x0,0x0,0x405,0x1855,(uVar4 + 0x6));
      LVar6 = SendDlgItemMessage16(0x0,0x0,0x405,0x1856,(uVar4 + 0x6));
      puVar3 = (LVar6 >> 0x10);
      puVar2 = local_106;
      pass1_1008_b4a0(puVar2,puVar3,(uVar4 + 0x94),CONCAT22(0x1050,puVar2));
      pass1_1008_b200(*(astruct_194 **)(uVar4 + 0x94));
      uVar7 = CONCAT22(puVar3 | puVar2,puVar2);
      if ((puVar3 | puVar2) != 0) {
        send_dlg_item_msg_1038_8400(uVar4,uVar5,CONCAT22(puVar3,puVar2),0x1855);
        l_param = pass1_1008_b366((uVar4 + 0x94));
        uVar7 = l_param & 0xffff | ((l_param >> 0x10) | l_param) << 0x10;
        if (l_param != 0) {
          uVar7 = SendDlgItemMessage16(l_param,0xffff,0x40d,0x1855,(uVar4 + 0x6));
        }
      }
      uVar7 = pass1_1008_b38c(CONCAT22(uVar7,(uVar7 >> 0x10)),*(astruct_196 **)(uVar4 + 0x94));
      if (uVar7 != 0) {
        send_dlg_item_msg_1038_8400(uVar4,uVar5,uVar7,0x1856);
        uVar7 = pass1_1008_b47a((uVar4 + 0x94));
        if (uVar7 != 0) {
          SendDlgItemMessage16(uVar7,0xffff,0x40d,0x1856,(uVar4 + 0x6));
        }
      }
      SendDlgItemMessage16(0x0,0x1,0xb,0x1855,(uVar4 + 0x6));
      LVar6 = SendDlgItemMessage16(0x0,0x1,0xb,0x1856,(uVar4 + 0x6));
      uVar1 = LVar6;
    }
  }
  return uVar1;
}



pub unsafe fn send_dlg_item_msg_1038_87b2(mut param_1: u16 ,param_2: *mut astruct_903) -> u16

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut l_param: *mut c_char;
  let mut LVar3: LRESULT;
  let mut uVar4: u16;
  let mut local_102: [u8;0x100] = [0;0x100];

  uVar4 = param_2;
  uVar1 = (param_2 >> 0x10);
  uVar2 = send_dlg_item_msg_1038_8164(uVar4,uVar1,CONCAT22(0x1050,local_102),0x1855);
  if (uVar2 != 0) {
    pass1_1008_b61a(local_102,param_1,(uVar4 + 0x94),CONCAT22(0x1050,local_102));
    l_param = load_string_1008_b1f0();
    LVar3 = SendDlgItemMessage16((LPARAM)l_param,0xffff,0x40d,0x1856,(uVar4 + 0x6));
    uVar2 = LVar3;
  }
  return uVar2;
}
pub fn pass1_1038_8810(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut local_102: [u8;0x100] = [0;0x100];

  uVar2 = (param_1 >> 0x10);
  uVar1 = send_dlg_item_msg_1038_8164(param_1,uVar2,CONCAT22(0x1050,local_102),0x1856);
  if (uVar1 != 0) {
    pass1_1008_b63a((param_1 + 0x94),CONCAT22(0x1050,local_102));
  }
  return;
}



pub unsafe fn FUN_1038_8842() -> u16

{
  return 0x0;
}
pub fn pass1_1038_8848()

{
  return;
}
pub fn pass1_1038_884c()

{
  return;
}



pub fn pass1_1038_8850(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_7d5c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_88f2(param_1: *mut astruct_57,mut param_2: u16 )

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;

  struct_1040_b082(param_1,CONCAT22(param_2,0x184c));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  &iVar1[0x1].field3_0x6 = _u16_1050_5a68;
  iVar1[0x1].field5_0xa = 0;
  iVar1[0x1].field6_0xc = 0;
  iVar1[0x1].field7_0xe = 0;
  iVar1[0x1].field8_0x10 = 0;
  param_1.field0_0x0 = 0x8c2e;
  iVar1.field1_0x2 = &u16_1050_1038;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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



pub unsafe fn pass1_1038_8966(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,mut param_5: u16 ) -> u16

{
  let mut piVar1: *mut i16;
  let mut bVar2: bool;
  let mut iVar3: i16;
  let mut uVar4: u16;

  bVar2 = false;
  iVar3 = param_1;
  uVar4 = (param_1 >> 0x10);
  if (param_4 == 0) {
    if ((iVar3 + 0x98) < 1) goto LAB_1038_89af;
    piVar1 = (iVar3 + 0x9a);
    *piVar1 = *piVar1 + 1;
    piVar1 = (iVar3 + 0x98);
    *piVar1 = *piVar1 + -0x1;
  }
  else {
    if (param_4 != 1) goto LAB_1038_89af;
    if ((iVar3 + 0x9a) < 1) goto LAB_1038_89af;
    piVar1 = (iVar3 + 0x9a);
    *piVar1 = *piVar1 + -0x1;
    piVar1 = (iVar3 + 0x98);
    *piVar1 = *piVar1 + 1;
  }
  bVar2 = true;//
// LAB_1038_89af:
  if (bVar2) {
    SetDlgItemInt16(0x0,(iVar3 + 0x9a),s_dibtext_bmp_1050_1844 + 0x9,(iVar3 + 0x6));
    SetDlgItemInt16(0x0,(iVar3 + 0x98),s_dibtext_bmp_1050_1844 + 0xb,(iVar3 + 0x6));
  }
  return 0x0;
}
pub fn pass1_1038_89e8(mut param_1: u32)

{
  send_dlg_item_msg_1038_8b58(param_1);
  return;
}
pub fn pass1_1038_89f8(param_1: *mut astruct_903,mut param_2: u16 ,mut param_3: u32,param_4: *mut u8,mut param_5: u16 )

{
  if (param_3 == 0xeb) {
    send_dlg_item_msg_1038_8b58(param_1);
  }
  else {
    if (param_3 != s_vrpal_bmp_1050_183a + 0x7) {
      pass1_1040_b54a(param_4,param_1,param_2,param_3);
      return;
    }
    msg_box_ui_op_1038_8a3a(0x0,param_4,param_1,&DAT_1050_1050);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_ui_op_1038_8a3a(param_1: *mut c_char,mut param_2: u16 ,param_3: *mut astruct_903,mut param_4: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut local_20a: [u8;0x102] = [0;0x102];
  let mut pcStack264: *mut c_char;
  short sStack262;
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  sStack262 = (short)paVar1;
  pcStack264 = param_1;
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,sStack262);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(sStack262,pcStack264),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(sStack262,pcStack264),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x101,local_20a,(short)&DAT_1050_1050);
  MessageBox16(0x0,CONCAT22(0x1050,local_20a),CONCAT22(sStack262,pcStack264),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(sStack262,pcStack264));
  return;
}
pub fn unk_win_ui_op_1038_8afe(mut param_1: u16 ,param_2: *mut astruct_50)

{
  let mut uVar1: u32;
  let mut dlg_item: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  iVar4: *mut astruct_50;
  uVar4: *mut astruct_50;
  let mut local_4: bool;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  uVar4 = (param_2 >> 0x10);
  iVar4 = param_2;
  dlg_item = GetDlgItemInt16(0x0,&local_4,&DAT_1050_1050,s_dibtext_bmp_1050_1844 + 0x9);
  pass1_1030_6c1a(iVar4.field148_0x94,dlg_item);
  uVar1 = iVar4.field148_0x94;
  pass1_1038_387e(paVar2,*(astruct_302 **)(uVar1 + 0x2e),dlg_item,iVar4.field153_0x9c,iVar4.field148_0x94);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn send_dlg_item_msg_1038_8b58(param_1: *mut astruct_903)

{
  let mut uVar1: u32;
  let mut puVar2: *mut u8;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut in_EDX: *mut Struct57;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut LVar7: LRESULT;
  let mut in_stack_0000fd96: u16;
  let mut in_stack_0000feba: u16;
  let mut in_stack_0000fec0: u16;
  let mut in_stack_0000fec4: u16;
  let mut in_stack_0000feee: u16;
  let mut local_106: [u8;0x100] = [0;0x100];
  let mut puStack6: *mut u32;

  puStack6 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22(in_stack_0000feee,0x3),in_stack_0000fd96,
                             in_stack_0000feba,in_stack_0000fec0,in_stack_0000fec4);
  puVar2 = (puStack6 >> 0x10);
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  pass1_1010_c3c2(puVar2,puStack6,puVar2,CONCAT22(0x1050,local_106),(iVar5 + 0x94));
  LVar7 = SendDlgItemMessage16
                    (CONCAT22(0x1050,local_106),0x0,0xc,s_dibtext_bmp_1050_1844 + 0x2,(iVar5 + 0x6));
  uVar4 = (LVar7 >> 0x10);
  uVar1 = (iVar5 + 0x94);
  (iVar5 + 0x9c) = (uVar1 + 0x32);
  (iVar5 + 0x9a) = (iVar5 + 0x9c);
  SetDlgItemInt16(0x0,(iVar5 + 0x9c),s_dibtext_bmp_1050_1844 + 0x9,(iVar5 + 0x6));
  uVar1 = (iVar5 + 0x94);
  uVar3 = (uVar1 + 0x2e);
  pass1_1038_3aa6(uVar3,uVar4,uVar3);
  (iVar5 + 0x98) = uVar3;
  SetDlgItemInt16(0x0,uVar3,s_dibtext_bmp_1050_1844 + 0xb,(iVar5 + 0x6));
  return;
}



pub fn pass1_1038_8c08(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_893a(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * pass1_1038_8caa(param_1: *mut u8,param_2: *mut astruct_57,mut param_3: u16 )

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
  &iVar1[0x1].field3_0x6 = 0;
  param_2.field0_0x0 = 0x90c8;
  iVar1.field1_0x2 = &u16_1050_1038;
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x3f),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field3_0x6 = puVar3;
  iVar1[0x1].field4_0x8 = (puVar3 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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
pub fn send_dlg_item_msg_1038_8d22(mut param_1: u32,undefined1 param_2)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut LVar3: LRESULT;
  let mut local_106: [u8;0x100] = [0;0x100];
  let mut WStack6: WPARAM16;
  let mut iStack4: i16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  LVar3 = SendDlgItemMessage16(0x0,0x0,0x409,0x185b,(iVar1 + 0x6));
  WStack6 = LVar3;
  iStack4 = WStack6 >> 0xf;
  if (WStack6 != 0xffff) {
    SendDlgItemMessage16(CONCAT22(0x1050,local_106),WStack6,0x40a,0x185b,(iVar1 + 0x6));
    pass1_1008_c79a((iVar1 + 0x94),CONCAT22(0x1050,local_106));
  }
  return;
}



LRESULT pass1_1038_8d7e(param_1: *mut astruct_903)

{
  let mut LVar1: LRESULT;

  pass1_1040_78de();
  LVar1 = send_dlg_item_msg_1038_8f74(param_1);
  return LVar1;
}
pub fn pass1_1038_8d98(param_1: *mut u8,param_2: *mut astruct_903,mut param_3: u16 ,mut param_4: u32)

{
  if (param_4 == 0xeb) {
    send_dlg_item_msg_1038_8f74(param_2);
  }
  else {
    if (param_4 != s_vrpal_bmp_1050_183a + 0x7) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4);
      return;
    }
    msg_box_op_1038_8dda(0x0,param_1,param_2);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_op_1038_8dda(param_1: *mut c_char,mut param_2: u16 ,param_3: *mut astruct_903)

{
  short in_buf_len_5;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u16;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,(short)&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  uVar2 = (param_3 >> 0x10);
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT send_dlg_item_msg_1038_8f74(param_1: *mut astruct_903)

{
  let mut uVar1: u32;
  let mut iVar3: i16;
  let mut hwnd: HWND16;
  iVar2: *mut astruct_903;
  let mut uVar4: u16;
  let mut lVar4: i32;
  let mut LVar5: LRESULT;
  let mut enable: bool;
  local_50c: u16 [0x80];
  let mut local_40c: [u8;0x8] = [0;0x8];
  let mut local_404: u32;

  uVar4 = (param_1 >> 0x10);
  iVar2 = param_1;
  SendDlgItemMessage16(0x0,0x0,0xb,0x185b,iVar2.field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x185b,iVar2.field6_0x6);
  iVar3 = pass1_1008_c83a(iVar2.field147_0x94);
  if (iVar3 == 0) {
    local_404 = pass1_1008_c85e(iVar2.field147_0x94);
    pass1_1008_5784(CONCAT22(0x1050,local_40c),local_404);
    while( true ) {
      lVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_40c));
      if (lVar4 == 0) break;
      uVar1 = (lVar4 + 0x4);
      wsprintf16(local_50c,0x5a6c1050,CONCAT22(uVar1,0x1050),(uVar1 >> 0x10));
      SendDlgItemMessage16(CONCAT22(0x1050,local_50c),0x0,0x401,0x185b,iVar2.field6_0x6);
    }
    hwnd = GetDlgItem16(0x1,iVar2.field6_0x6);
    enable = 0x1;
  }
  else {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,&local_404,
               (short)&DAT_1050_1050);
    SendDlgItemMessage16(CONCAT22(0x1050,&local_404),0x0,0x401,0x185b,iVar2.field6_0x6);
    hwnd = GetDlgItem16(0x1,iVar2.field6_0x6);
    enable = 0;
  }
  EnableWindow16(enable,hwnd);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0xb,0x185b,iVar2.field6_0x6);
  return LVar5;
}
