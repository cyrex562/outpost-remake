


StructD * pass1_1040_d056(StructD *param_1,param_2: u8)

{
  pass1_1040_ca74(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_d0f8(param_1: *mut astruct_57,mut param_2: u16 ,mut param_3: u16 ,StructD *param_4,mut param_5: u16 ,
                    mut param_6: u16 ,mut param_7: u16 ,mut param_8: u16 ,mut param_9: u16 ,mut param_10: u16 ,
                    mut param_11: u16 ,mut param_12: u16 ,mut param_13: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar5: *mut Struct57;
  let mut uVar7: u16;
  let mut puVar8: *mut u32;
  let mut uVar9: u32;
  let mut uVar5: u32;
  let mut paVar6: *mut Struct57;

  struct_1040_b082(param_1,CONCAT22(param_2,0x1845));
  uVar7 = (param_1 >> 0x10);
  iVar5 = (astruct_57 *)param_1;
  &iVar5[0x1].field3_0x6 = 0;
  &iVar5[0x1].field5_0xa = _PTR_LOOP_1050_5f16;
  &iVar5[0x1].field7_0xe = 0;
  iVar5[0x1].field9_0x12 = 0;
  param_1->field0_0x0 = 0xd8c4;
  iVar5->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar8 = mixed_1010_20ba((astruct_57 *)param_4,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x47),param_6,param_10,
                           param_11,param_12);
  uVar5 = param_4 & 0xffff0000;
  iVar5[0x1].field3_0x6 = puVar8;
  uVar2 = (puVar8 >> 0x10);
  iVar5[0x1].field4_0x8 = uVar2;
  uVar9 = pass1_1018_5732(puVar8,uVar2,iVar5[0x1].field3_0x6,uVar2,&iVar5[0x1].field5_0xa);
  paVar4 = (astruct_57 *)(uVar5 & 0xffff0000 | uVar9 >> 0x10);
  iVar5[0x1].field7_0xe = uVar9;
  uVar1 = (uVar9 >> 0x10);
  iVar5[0x1].field8_0x10 = uVar1;
  uVar1 |= iVar5[0x1].field7_0xe;
  if (uVar1 == 0) {
    mem_op_1000_179c(0xc,paVar4);
    uVar3 = paVar4 | uVar1;
    paVar6 = (astruct_57 *)(paVar4 & 0xffff0000 | uVar3);
    if (uVar3 == 0) {
      &iVar5[0x1].field7_0xe = 0;
    }
    else {
      pass1_1010_8ef2(paVar6,(astruct_170 *)CONCAT22(paVar4,uVar1),param_13,param_5,param_7,param_8,param_9);
      iVar5[0x1].field7_0xe = uVar1;
      iVar5[0x1].field8_0x10 = paVar6;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_d1bc(StructD *param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  StructD *iVar4;
  let mut uVar4: u16;
  let mut in_stack_0000ffd4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  param_1->address_offset_field_0x0 = 0xd8c4;
  iVar4->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar4->field_0x6);
  puVar1 = &iVar4->field_0x9c;
  uVar2 = &iVar4->field_0x9e;
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(&u16_1050_1038,puVar1,uVar2,1);
  }
  unk_draw_op_1040_b0f8(in_stack_0000ffd4,param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn send_dlg_item_msg_1040_d20c(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_929,i32 param_4)

{
  let mut puVar1: *mut u8;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut puVar6: *mut u32;
  let mut in_stack_0000fd96: u16;
  let mut in_stack_0000feba: u16;
  let mut in_stack_0000fec0: u16;
  let mut in_stack_0000fec4: u16;
  let mut puVar7: *mut u8;
  let mut uVar8: u16;
  u8 local_106 [0x100];
  let mut uStack6: u16;
  let mut uStack4: u16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if (param_4 == 0) {
    enable_win_1040_d60e(param_3);
    return;
  }
  uVar5 = (param_3 >> 0x10);
  iVar4 = param_3;
  if ((iVar4 + 0xa0) != 0) {
    pass1_1010_9210((iVar4 + 0x9c));
    enable_win_1040_d60e(param_3);
    pass1_1030_8344(_u16_1050_5748,param_4);
    uVar2 = SUB42(paVar3,0x0);
    puVar7 = local_106;
    uVar8 = SUB42(&DAT_1050_1050,0x0);
    uStack6 = param_1;
    uStack4 = uVar2;
    puVar6 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(puVar7,0x3),in_stack_0000fd96,
                             in_stack_0000feba,in_stack_0000fec0,in_stack_0000fec4);
    puVar1 = (puVar6 >> 0x10);
    pass1_1010_c3c2(puVar1,puVar6,puVar1,CONCAT22(uVar8,puVar7),CONCAT22(uVar2,param_1));
    SendDlgItemMessage16
              (CONCAT22(0x1050,local_106),0x0,0x401,s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(iVar4 + 0x6));
  }
  return;
}
pub fn pass1_1040_d29c(mut param_1: u32)

{
  let mut in_DX: u16;

  send_ldg_item_msg_1040_d79c(in_DX,(astruct_903 *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_d2ac(mut param_1: u16 ,astruct_903 *pstruct_param_2,mut param_3: u16 ,mut param_4: u32)

{
  let mut in_register_0000000a: u16;
  let mut LVar1: LRESULT;

  if (param_4 == s_dibtext_bmp_1050_1844 + 0x4U) {
    LVar1 = SendDlgItemMessage16
                      (0x0,0x0,0x405,s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(pstruct_param_2 + 0x6));
    struct_1010_9172(*(astruct_249 **)(pstruct_param_2 + 0x9c),
                     CONCAT22(in_register_0000000a,(LVar1 >> 0x10)));
  }
  else if (s_dibtext_bmp_1050_1844 + 0x4U < param_4) {
    if (param_4 == s_dibtext_bmp_1050_1844 + 0x5) {
      LVar1 = SendDlgItemMessage16
                        (0x0,0x0,0x40c,s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(pstruct_param_2 + 0x6));
      if ((LVar1 != -1) || ((LVar1 >> 0x10) != -1)) {
        SendDlgItemMessage16
                  (0x0,LVar1 - 0x1,0x403,s_dibtext_bmp_1050_1844 + 0x3,
                   *(HWND16 *)(pstruct_param_2 + 0x6));
        pass1_1010_91cc((pstruct_param_2 + 0x9c));
      }
    }
    else if (param_4 == s_dibtext_bmp_1050_1844 + 0x6U) {
      enable_win_1040_d6be(pstruct_param_2);
      pass1_1018_57d2((pstruct_param_2 + 0x94),pstruct_param_2);
      PostMessage16(0x0,0x203,0x111,HWND16_1050_0396);
    }
    else {
      if (param_4 != s_dibtext_bmp_1050_1844 + 0x7U) goto LAB_1040_d3b3;
      _u16_1050_5a68 = (pstruct_param_2 + 0x98);
      pass1_1038_af40(pstruct_param_2,param_1,_PTR_LOOP_1050_5b7c,(pstruct_param_2 + 0x6),
                      0x27);
    }
  }
  else if (param_4 == 0xeb) {
    send_ldg_item_msg_1040_d79c(param_1,pstruct_param_2);
  }
  else {
    if (param_4 != s_vrpal_bmp_1050_183a + 0x7U) {//
LAB_1040_d3b3:
      pass1_1040_b54a(param_1,pstruct_param_2,param_3,param_4);
      return;
    }
    msg_box_op_1040_d3d0(0x0,param_1,pstruct_param_2);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_op_1040_d3d0(param_1: *mut c_char,mut param_2: u16 ,mut param_3: u32)

{
  short in_buf_len_5;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u16;
  char local_206 [0x102];
  char local_104 [0x102];

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
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
               *(HWND16 *)(param_3 + 0x6));
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
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)(param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}
pub fn enable_win_1040_d60e(param_1: *mut astruct_929)

{
  let mut HVar1: HWND16;
  astruct_929 *iVar2;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_929 *)param_1;
  HVar1 = GetDlgItem16(0x1,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16(0x2,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x7,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x4,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x5,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x6,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x7,iVar2->field6_0x6);
  EnableWindow16(0x1,HVar1);
  iVar2->field159_0xa0 = 0;
  return;
}
pub fn enable_win_1040_d6be(param_1: *mut astruct_903)

{
  let mut HVar1: HWND16;
  astruct_903 *iVar2;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (astruct_903 *)param_1;
  HVar1 = GetDlgItem16(0x1,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(0x2,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x7,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x4,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x5,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x6,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(s_dibtext_bmp_1050_1844 + 0x7,iVar2->field6_0x6);
  EnableWindow16(0x0,HVar1);
  &iVar2[0x1].field_0x8 = 0x1;
  return;
}
pub fn pass1_1040_d76e(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar1 = (iVar2 + 0x94);
  pass1_1018_5742(uVar1,(uVar1 >> 0x10),(iVar2 + 0x9c),*(astruct_299 **)(iVar2 + 0x98)
                 );
  (iVar2 + 0x9c) = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn send_ldg_item_msg_1040_d79c(mut param_1: u16 ,param_2: *mut astruct_903)

{
  let mut lVar1: i32;
  let mut puVar2: *mut u8;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar7: u32;
  let mut in_stack_0000fd9a: u16;
  let mut in_stack_0000febe: u16;
  let mut in_stack_0000fec4: u16;
  let mut in_stack_0000fec8: u16;
  let mut in_stack_0000fef2: u16;
  let mut uStack268: u16;
  let mut uStack266: u32;
  char local_106 [0x100];
  let mut puStack6: *mut u32;

  puStack6 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000fef2,0x3),in_stack_0000fd9a,in_stack_0000febe,
                             in_stack_0000fec4,in_stack_0000fec8);
  puVar2 = (puStack6 >> 0x10);
  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  pass1_1010_c3c2(puVar2,puStack6,puVar2,CONCAT22(0x1050,local_106),(iVar4 + 0x98));
  SendDlgItemMessage16(CONCAT22(0x1050,local_106),0x0,0xc,s_dibtext_bmp_1050_1844 + 0x2,*(HWND16 *)(iVar4 + 0x6));
  SendDlgItemMessage16(0x0,0x0,0xb,s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(iVar4 + 0x6));
  uVar6 = SendDlgItemMessage16(0x0,0x0,0x405,s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(iVar4 + 0x6));
  uVar7 = uVar6 >> 0x10;
  uVar3 = uVar6;
  pass1_1010_9044((iVar4 + 0x9c));
  uStack266 = CONCAT22(uVar7,uVar3);
  uVar3 = 0;
  uStack268 = 0;
  while (CONCAT22(uStack268,uVar3) < uStack266) {
    pass1_1010_9130(local_106,uVar7,(iVar4 + 0x9c),CONCAT22(0x1050,local_106));
    if (local_106[0] != '\0') {
      uVar7 = SendDlgItemMessage16
                        (CONCAT22(0x1050,local_106),0x0,0x401,s_dibtext_bmp_1050_1844 + 0x3,
                         *(HWND16 *)(iVar4 + 0x6));
      uVar7 >>= 0x10;
    }
    lVar1 = CONCAT22(uStack268,uVar3) + 1;
    uVar3 = lVar1;
    uStack268 = (lVar1 >> 0x10);
  }
  SendDlgItemMessage16(0x0,0x1,0xb,s_dibtext_bmp_1050_1844 + 0x3,*(HWND16 *)(iVar4 + 0x6));
  return;
}



StructD * pass1_1040_d89e(StructD *param_1,param_2: u8)

{
  pass1_1040_d1bc(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
