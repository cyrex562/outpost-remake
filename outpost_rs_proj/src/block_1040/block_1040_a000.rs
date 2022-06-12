

u16 * pass1_1040_a204(param_1: *mut u16,param_2: u8)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn make_proc_inst_1040_a234(u8 *param_1,u8 *param_2,mut param_3: u16 ,mut param_4: u32)

{
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22((int)param_4,param_3),(param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0xa4e8;
  (param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  if (_PTR_LOOP_1050_5edc == NULL) {
    _PTR_LOOP_1050_5edc = MakeProcInstance16(HINSTANCE16_1050_038c,call_win_proc_1040_a40e);
  }
  *(void **)(param_1 + 0xc) = _PTR_LOOP_1050_5edc;
  PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + 0x1;
  PTR_LOOP_1050_5ee0 = param_1;
  PTR_LOOP_1050_5ee2 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn free_proc_inst_1040_a294(StructD *param_1)

{
  let mut in_stack_0000ffde: u16;

  param_1->address_offset_field_0x0 = 0xa4e8;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + -0x1;
  if (PTR_LOOP_1050_5eda == NULL) {
    FreeProcInstance16(_PTR_LOOP_1050_5edc);
    _PTR_LOOP_1050_5edc = NULL;
  }
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}



pub fn pass1_1040_a2cc(mut param_1: u16 ,u8 *param_2,mut param_3: i16,mut param_4: u32,mut param_5: u32) -> u32

{
  let mut uVar1: u16;

  if (param_5 == 0x1826) {
    if (((int)param_5 == 0x1) || ((0x1 < (int)param_5 - 0x1U && ((int)param_5 - 0x3U < 0x2)))) {
      uVar1 = 0x1;
    }
    else {
      uVar1 = 0x0;
    }
    return (u32)uVar1;
  }
  pass1_1040_b54a(param_2,(astruct_903 *)CONCAT22((int)param_4,param_3),(param_4 >> 0x10),param_5);
  return CONCAT22(param_2,param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_msg_1040_a308(param_1: *mut astruct_49,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ) -> u32

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  LRESULT LVar6;
  u32 *puVar7;
  char *pcVar8;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  WPARAM16 WVar9;
  let mut UVar10: u16;
  let mut uVar11: u16;
  let mut in_stack_0000fff2: u16;
  let mut iStack12: i16;

  uVar3 = ((u32)in_EDX >> 0x10);
  SendMessage16(0x0,0x0,0x405,param_4);
  LVar6 = SendMessage16(0x0,0x0,0xb,param_4);
  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar2 = (u32)(iVar4 + 0x90);
  if (((int)uVar2 + 0x10) == 0x0) {
    WVar9 = 0x0;
    UVar10 = 0x401;
    uVar11 = param_4;
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x531);
    SendMessage16((LPARAM)pcVar8,WVar9,UVar10,uVar11);
  }
  else {
    puVar7 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar3,(int)((u32)LVar6 >> 0x10)),_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000fff2,0x3),in_stack_0000fe9a,in_stack_0000ffbe,
                             in_stack_0000ffc4,in_stack_0000ffc8);
    for (iStack12 = 0x0; uVar2 = (u32)(iVar4 + 0x90), piVar1 = ((int)uVar2 + 0x10),
        *piVar1 != iStack12 && iStack12 <= *piVar1; iStack12 += 0x1) {
      WVar9 = 0x0;
      UVar10 = 0x401;
      uVar2 = (u32)(iVar4 + 0x90);
      uVar2 = (u32)((int)uVar2 + 0xc);
      uVar11 = param_4;
      pcVar8 = load_string_1010_ac92
                         (puVar7,((u32)puVar7 >> 0x10),((int)uVar2 + iStack12 * 0x2));
      SendMessage16((LPARAM)pcVar8,WVar9,UVar10,uVar11);
    }
  }
  LVar6 = SendMessage16(0x0,0x1,0xb,param_4);
  return CONCAT22((int)((u32)LVar6 >> 0x10),0x1);
}
pub fn get_dlg_item_1040_a3d0(param_1: *mut astruct_49)

{
  i32 lVar1;
  HWND16 dlg_item;
  astruct_49 *iVar3;
  astruct_49 *uVar2;

  uVar2 = (astruct_49 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_49 *)param_1;
  if (iVar3->field143_0x90 != 0x0) {
    lVar1 = iVar3->field143_0x90;
    *(HWND16 *)((int)lVar1 + 0x14) = iVar3->field6_0x6;
    dlg_item = GetDlgItem16(0x1826,iVar3->field6_0x6);
    win_msg_1040_a308(param_1,0x0,0x0,dlg_item);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn call_win_proc_1040_a40e(mut param_1: u16 ,HWND16 param_2,mut param_3: u32,LPARAM param_4) -> u32

{
  LPVOID func;
  let mut uVar1: u16;
  code **ppcVar2;
  WPARAM16 wparam;
  let mut uVar6: u16;
  let mut uVar3: u32;
  let mut uStack6: u32;
  u32 *puVar3;
  let mut uVar5: u32;

  uStack6 = 0x0;
  wparam = (WPARAM16)(param_3 >> 0x10);
  if (param_4 == 0x19) {
    ppcVar2 = (code **)((int)(u32)_PTR_LOOP_1050_5ee0 + 0x34);
    uStack6 = (**ppcVar2)();
    param_1 = (uStack6 >> 0x10);
  }
  else {
    if (param_4 == 0x86) {
      ppcVar2 = (code **)((int)(u32)_PTR_LOOP_1050_5ee0 + 0x20);
      uVar3 = (**ppcVar2)();
      return uVar3;
    }
    if (param_4 == 0x110) {
      uVar3 = win_msg_1040_a308(_PTR_LOOP_1050_5ee0,param_2,param_3,wparam);
      return uVar3;
    }
  }
  if (uStack6 != 0x0) {
    return uStack6 & 0xffff | (u32)param_1 << 0x10;
  }
  uVar6 = ((u32)_u16_1050_5bc8 >> 0x10);
  func = *(LPVOID *)((int)_u16_1050_5bc8 + 0x4);
  uVar1 = ((int)_u16_1050_5bc8 + 0x6);
  if ((uVar1 | func) == 0x0) {
    return (u32)uVar1 << 0x10;
  }
  uVar3 = CallWindowProc16(CONCAT22(param_3,param_2),wparam,param_4,(HWND16)((u32)param_4 >> 0x10),func);
  return uVar3;
}



StructD * pass1_1040_a4c2(StructD *param_1,param_2: u8)

{
  free_proc_inst_1040_a294(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1040_a564(u32 *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = 0x0;
  ((int)param_1 + 0x4) = 0x0;
  (u32)((int)param_1 + 0x6) = 0x0;
  return;
}
pub fn pass1_1040_a582(u32 *param_1)

{
  fn_ptr_1000_17ce((char *)*param_1);
  return;
}
pub fn struct_1040_a598(param_1: *mut astruct_259)

{
  astruct_259 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_259 *)param_1;
  param_1->field0_0x0 = 0x0;
  iVar1->field1_0x2 = 0x0;
  iVar1->field2_0x6 = 0x0;
  iVar1->field3_0xa = 0x0;
  iVar1->field4_0xc = 0x0;
  iVar1->field5_0x10 = 0x0;
  iVar1->field6_0x12 = 0x0;
  iVar1->field7_0x14 = 0x0;
  iVar1->field8_0x16 = 0x0;
  return;
}
pub fn pass1_1040_a5d0(StructD *param_1)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  StructD *iVar4;
  let mut uVar3: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  uVar1 = iVar4->address_offset_field_0x2;
  uVar2 = iVar4->hfile_0x4;
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1000_54e8(0xa582,&PTR_LOOP_1050_1040,(uVar1 - 0x2),0xa,uVar1,uVar2);
    fn_ptr_1000_17ce((char *)CONCAT22(uVar2,uVar1 - 0x2));
  }
  fn_ptr_1000_17ce(*(char **)&iVar4->field7_0xc);
  return;
}
pub fn string_1040_a626(mut param_1: u16 ,param_2: *mut u16,char *param_3)

{
  let mut uVar1: u16;

  uVar1 = str_op_1008_60e8(param_1,param_3);
  *param_2 = uVar1;
  ((int)param_2 + 0x2) = param_1;
  return;
}
pub fn pass1_1040_a640(param_1: *mut astruct_57,mut param_2: u32,mut param_3: u16 )

{
  astruct_57 *iVar1;
  astruct_57 *uVar1;

  struct_1040_b082(param_1,CONCAT22(param_3,0x1f1));
  uVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_57 *)param_1;
  (u32)&iVar1[0x1].field3_0x6 = param_2;
  iVar1[0x1].field5_0xa = 0x0;
  &iVar1[0x1].field_0x5c = 0x0;
  param_1->field0_0x0 = 0xac08;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn draw_op_1040_a67e(astruct_750 *struct750_param_1,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 )

{
  let mut bVar1: bool;
  HBRUSH16 brush_handle_var2;
  astruct_750 *struct750_var4;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut iStack14: i16;
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  astruct_751 *iVar2;

  uVar3 = ((u32)struct750_param_1 >> 0x10);
  struct750_var4 = (astruct_750 *)struct750_param_1;
  if (struct750_var4->hbrush16_field142_0x8e == 0x0) {
    brush_handle_var2 = CreateSolidBrush16(WHITE_BRUSH);
    struct750_var4->hbrush16_field142_0x8e = brush_handle_var2;
  }
  if (_u16_1050_5ee8 == 0x0) {
    uVar4 = pass1_1008_4d72((u32)((int)_PTR_LOOP_1050_4230 + 0xe));
    uVar2 = (uVar4 >> 0x10);
    iVar2 = (astruct_751 *)uVar4;
    _u16_1050_5ee8 = (u32)CONCAT12(iVar2->field_0x94,CONCAT11(iVar2->field_0x95,iVar2->field_0x96));
    u16_1050_5eec = CONCAT11(iVar2->field_0x3e5,iVar2->field_0x3e6);
    u16_1050_5eee = iVar2->field996_0x3e4;
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return;
    }
    bVar1 = false;
    for (iStack14 = 0x0; piVar1 = &struct750_var4->field233_0xea, *piVar1 != iStack14 && iStack14 <= *piVar1;
        iStack14 += 0x1) {
      if ((&struct750_var4->field_0x9a + iStack14 * 0x2) == param_2) {
        bVar1 = true;
        break;
      }
    }
    if (bVar1) {
      u16_1050_5ee8 = u16_1050_5eec;
      u16_1050_5eea = u16_1050_5eee;
    }
  }
  SetTextColor16(CONCAT22(u16_1050_5eea,u16_1050_5ee8),param_4);
  SetBkColor16(0x1000000,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_a784(undefined1 param_1,mut param_2: u16 ,mut param_3: i16,mut param_4: i16,mut param_5: u16 ,mut param_6: u32)

{
  HWND16 hwnd;
  let mut iVar1: i16;

  iVar1 = param_3;
  if (param_6 != 0xeb) {
    if (param_6 == 0x1f4) {
      msg_box_op_1040_a85a(0x0,param_2,CONCAT22(param_4,param_3));
      return;
    }
    if (param_6 == 0x1f7) {
      _PTR_LOOP_1050_5ef0 = (u32)(param_3 + 0x94);
      pass1_1038_af40(param_3,param_2,_PTR_LOOP_1050_5b7c,(param_3 + 0x8),0x23);
      PostMessage16(0x0,0x2,0x111,*(HWND16 *)(param_3 + 0x6));
      return;
    }
    if (param_6 != 0x17d8) {
      pass1_1040_b54a(param_2,(astruct_903 *)CONCAT22(param_4,param_3),param_5,param_6);
      return;
    }
    SetWindowPos16(0x6,0xed,0x237,0x0,0x0,0x0,*(HWND16 *)(param_3 + 0x6));
    hwnd = GetDlgItem16(0x17d8,*(HWND16 *)(param_3 + 0x6));
    iVar1 = (int)s_tile2_bmp_1050_1538;
    EnableWindow16(0x0,hwnd);
    (param_3 + 0x98) = 0x1;
    param_4 = param_3;
  }
  win_ui_dlg_op_1040_a94a(param_2,CONCAT22(param_4,iVar1));
  return;
}
pub fn pass1_1040_a84a(undefined1 param_1,mut param_2: u16 ,mut param_3: u32)

{
  win_ui_dlg_op_1040_a94a(param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_op_1040_a85a(char *param_1,mut param_2: u16 ,mut param_3: u32)

{
  short in_buf_len_5;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  char local_206 [0x102];
  char local_104 [0x102];

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_206,(short)&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  fn_ptr_1000_17ce((char *)CONCAT22(in_buf_len_5,param_1));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_dlg_op_1040_a94a(mut param_1: u16 ,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut uVar3: u32;
  u8 *puVar4;
  let mut uVar5: u16;
  u8 *value;
  char *pcVar6;
  let mut uVar7: u16;
  HWND16 HVar8;
  let mut value_00: u16;
  u8 *puVar9;
  let mut in_register_0000000a: u16;
  let mut uVar10: u32;
  let mut iVar11: i16;
  let mut iVar12: i16;
  let mut unaff_SI: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut bVar15: bool;
  u32 *puVar16;
  let mut uVar17: u32;
  let mut in_stack_0000fd7c: u16;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000feaa: u16;
  let mut uStack288: u16;
  let mut iStack278: i16;
  let mut iStack276: i16;
  u8 local_102 [0x100];

  puVar16 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                            (u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fd7c,in_stack_0000fea0,in_stack_0000fea6,
                            in_stack_0000feaa);
  puVar4 = ((u32)puVar16 >> 0x10);
  uVar5 = puVar16;
  uVar13 = (param_2 >> 0x10);
  iVar11 = (int)param_2;
  puVar9 = puVar4;
  pass1_1010_c3c2(puVar4,uVar5,puVar4,CONCAT22(0x1050,local_102),(u32)(iVar11 + 0x94));
  SetDlgItemText16(CONCAT22(0x1050,local_102),0x1f2,*(HWND16 *)(iVar11 + 0x6));
  pass1_1010_c320((char *)puVar9,uVar5,puVar4,CONCAT22(0x1050,local_102),(u32)(iVar11 + 0x94));
  SetDlgItemText16(CONCAT22(0x1050,local_102),0x1774,*(HWND16 *)(iVar11 + 0x6));
  string_op_1010_c446(puVar9,(u32)puVar16,CONCAT22(0x1050,local_102),(u32)(iVar11 + 0x94));
  value = local_102;
  SetDlgItemText16(CONCAT22(0x1050,value),0x1773,*(HWND16 *)(iVar11 + 0x6));
  pass1_1030_6ddc((u32)(iVar11 + 0x94));
  SetDlgItemInt16(0x0,value,0x1f5,*(HWND16 *)(iVar11 + 0x6));
  pass1_1030_6e14((u32)(iVar11 + 0x94));
  SetDlgItemInt16(0x0,value,0x1f6,*(HWND16 *)(iVar11 + 0x6));
  if ((iVar11 + 0x98) != 0x0) {
    struct_1010_dd5e(uVar5,puVar4,(u32)(iVar11 + 0x94));
    if ((puVar9 | value) != 0x0) {
      uVar3 = (u32)(iVar11 + 0x94);
      uVar14 = ((u32)uVar3 >> 0x10);
      iVar12 = (int)uVar3;
      uVar2 = (u32)(iVar12 + 0x26);
      uVar10 = (u32)(iVar12 + 0x28);
      iStack276 = 0x1798;
      iStack278 = 0x17c3;
      (iVar11 + 0xea) = 0x0;
      uVar17 = uVar2;
      for (uStack288 = 0x1; (int)uStack288 < 0x25; uStack288 += 0x1) {
        if (uVar2 == 0x0) {
          value_00 = 0x0;
          uVar10 = 0x0;
        }
        else {
          uVar17 = pass1_1020_bae6(uVar17,uVar10,uVar2,CONCAT22(uStack288,(int)(uVar2 >> 0x10)));
          uVar10 = uVar17 >> 0x10;
          value_00 = uVar17;
        }
        uVar7 = uVar10;
        bVar15 = *(i32 *)(value + uStack288 * 0x4) != 0x0;
        if (bVar15) {
          pcVar6 = string_1020_c0d8(uStack288);
          SetDlgItemText16(CONCAT22((int)uVar10,pcVar6),iStack276,*(HWND16 *)(iVar11 + 0x6));
          SetDlgItemInt16(0x0,(value + uStack288 * 0x4),iStack278,*(HWND16 *)(iVar11 + 0x6));
        }
        uVar7 |= value_00;
        if (uVar7 != 0x0) {
          if (!bVar15) {
            pcVar6 = string_1020_c0d8(uStack288);
            SetDlgItemText16(CONCAT22((int)uVar10,pcVar6),iStack276,*(HWND16 *)(iVar11 + 0x6));
          }
          SetDlgItemInt16(0x0,value_00,iStack278,*(HWND16 *)(iVar11 + 0x6));
          iVar12 = (iVar11 + 0xea);
          HVar8 = GetDlgItem16(iStack276,*(HWND16 *)(iVar11 + 0x6));
          *(HWND16 *)(iVar11 + iVar12 * 0x2 + 0x9a) = HVar8;
          piVar1 = (iVar11 + 0xea);
          *piVar1 = *piVar1 + 0x1;
          iVar12 = (iVar11 + 0xea);
          uVar7 = GetDlgItem16(iStack278,*(HWND16 *)(iVar11 + 0x6));
          *(HWND16 *)(iVar11 + iVar12 * 0x2 + 0x9a) = uVar7;
          piVar1 = (iVar11 + 0xea);
          *piVar1 = *piVar1 + 0x1;
          bVar15 = true;
        }
        uVar17 = (u32)uVar7;
        if (bVar15) {
          iStack276 += 0x1;
          iStack278 += 0x1;
        }
      }
    }
  }
  return;
}



StructD * pass1_1040_abe2(StructD *param_1,param_2: u8)

{
  let mut in_stack_0000ffda: u16;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_ac84(u8 *param_1,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar1;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082(param_2,CONCAT22(param_3,0x1f3));
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)&iVar1[0x1].field3_0x6 = 0x0;
  (u32)&iVar1[0x1].field5_0xa = 0x0;
  param_2->field0_0x0 = 0xafc4;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  (u32)&iVar1[0x1].field3_0x6 = _PTR_LOOP_1050_5ef0;
  _PTR_LOOP_1050_5ef0 = 0x0;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3d),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field5_0xa = puVar2;
  iVar1[0x1].field6_0xc = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_ace8(StructD *param_1)

{
  let mut uVar1: u16;
  let mut in_stack_0000ffde: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xafc4;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}
pub fn pass1_1040_ad14(mut param_1: u32)

{
  let mut in_DX: u16;

  win_ui_op_1040_ae04(in_DX,param_1);
  return;
}
pub fn pass1_1040_ad24(u8 *param_1,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  if (param_5 == 0xeb) {
    win_ui_op_1040_ae04(param_1,CONCAT22(param_3,param_2));
  }
  else {
    if (param_5 != 0x1f0) {
      pass1_1040_b54a(param_1,(astruct_903 *)CONCAT22(param_3,param_2),param_4,param_5);
      return;
    }
    msg_box_ui_op_1040_ad66(0x0,param_1,CONCAT22(param_3,param_2));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_ui_op_1040_ad66(char *param_1,mut param_2: u16 ,mut param_3: u32)

{
  short in_buf_len_5;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  char local_206 [0x102];
  char local_104 [0x102];

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_206,(short)&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  fn_ptr_1000_17ce((char *)CONCAT22(in_buf_len_5,param_1));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_ae04(mut param_1: u16 ,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut bVar2: bool;
  WORD *pWVar3;
  let mut iVar4: i16;
  char *pcVar5;
  i32 lVar6;
  u8 *puVar7;
  let mut uVar8: u16;
  let mut in_register_0000000a: u16;
  let mut iVar9: i16;
  i32 *plVar10;
  let mut unaff_SI: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  u32 *puVar13;
  let mut uVar14: u32;
  let mut uVar15: u32;
  char *lp_string;
  let mut uVar16: u32;
  let mut in_stack_0000fd84: u16;
  let mut in_stack_0000fea8: u16;
  let mut in_stack_0000feae: u16;
  let mut in_stack_0000feb2: u16;
  let mut iStack280: i16;
  WORD local_102 [0x80];

  puVar13 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                            (u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fd84,in_stack_0000fea8,in_stack_0000feae,
                            in_stack_0000feb2);
  puVar7 = ((u32)puVar13 >> 0x10);
  uVar11 = (param_2 >> 0x10);
  iVar9 = (int)param_2;
  pass1_1010_c3c2(puVar7,puVar13,puVar7,CONCAT22(0x1050,local_102),(u32)(iVar9 + 0x94));
  pWVar3 = local_102;
  SetDlgItemText16(CONCAT22(0x1050,pWVar3),0x1778,*(HWND16 *)(iVar9 + 0x6));
  uVar14 = struct_op_1030_73a8(*(astruct_419 **)(iVar9 + 0x94),pWVar3,puVar7);
  iVar4 = (int)uVar14 + 0x20;
  uVar15 = pass1_1030_8326();
  uVar16 = uVar15 >> 0x10;
  iVar1 = 0x0;
  bVar2 = false;
  for (iStack280 = 0x0; uVar8 = uVar16, iStack280 < 0xa; iStack280 += 0x1) {
    uVar12 = ((uVar14 & 0xffff0000) >> 0x10);
    if (((iStack280 * 0xc + iVar4 + 0x2) | (iStack280 * 0xc + iVar4)) != 0x0) {
      plVar10 = (i32 *)(iStack280 * 0xc + iVar4);
      pcVar5 = string_op_1020_c222((plVar10 + 0x1));
      SetDlgItemText16(CONCAT22(uVar8,pcVar5),iVar1 + 0x1d2,*(HWND16 *)(iVar9 + 0x6));
      lVar6 = *plVar10 - uVar15;
      wsprintf16(local_102,(char *)0x5ef41050,0xf4,CONCAT22((int)lVar6,0x1050),(int)((u32)lVar6 >> 0x10));
      SetDlgItemText16(CONCAT22(0x1050,local_102),iVar1 + 0x1dc,*(HWND16 *)(iVar9 + 0x6));
      uVar16 = unk_load_str_op_1010_8c96
                         (uVar8,(u32)(iVar9 + 0x98),CONCAT22(0x1050,local_102),
                          uVar14 & 0xffff0000 | ZEXT24(plVar10));
      uVar16 &= 0xffff;
      SetDlgItemText16(CONCAT22(0x1050,local_102),iVar1 + 0x1e6,*(HWND16 *)(iVar9 + 0x6));
      iVar1 += 0x1;
      bVar2 = true;
    }
  }
  if (!bVar2) {
    lp_string = load_string_1010_847e(_u16_1050_14cc,0x531);
    SetDlgItemText16((u32)lp_string,0x1d2,*(HWND16 *)(iVar9 + 0x6));
  }
  return;
}



StructD * pass1_1040_af9e(StructD *param_1,param_2: u8)

{
  pass1_1040_ace8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1040_b040(param_1: *mut astruct_57,mut param_2: u32,mut param_3: u16 )

{
  astruct_57 *iVar1;
  astruct_57 *uVar1;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,((int)param_2 + 0x12),param_3);
  uVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_57 *)param_1;
  (iVar1 + 0x1)->field0_0x0 = 0x0;
  (u32)&iVar1[0x1].field1_0x2 = param_2;
  param_1->field0_0x0 = 0xb772;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}
