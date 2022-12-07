
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_help_1040_800c(mut param_1: u32)

{
  let mut in_AX: u16;
  let mut uVar1: u16;
  let mut in_EDX: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut unaff_CS: u16;
  let mut iVar4: i16;
  let mut w_command: u16;
  let mut hwnd: HWND16;

  uVar1 = FUN_1010_830a(in_AX,in_EDX,unaff_CS,_u16_1050_14cc,0x1f8);
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0x8a) == 0) {
    hwnd = *(HWND16 *)(iVar2 + 0x6);
    iVar4 = 0;
    w_command = 0x3;
    iVar2 = 0;
  }
  else {
    hwnd = *(HWND16 *)(iVar2 + 0x6);
    w_command = 0x1;
    iVar2 = (iVar2 + 0x8a);
    iVar4 = iVar2 >> 0xf;
  }
  WinHelp16(CONCAT22(iVar4,iVar2),w_command,CONCAT22(in_EDX,uVar1),hwnd);
  return;
}



u16 pass1_1040_8054()

{
  return 0x0;
}


/*
Unable to decompile 'pass1_1040_805a'
Cause:
Low-level Error: Symbol $$undef00000006 extends beyond the end of the address space
*/


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_807e(param_1: *mut astruct_395,mut param_2: u16 )

{
  code **ppcVar1;
  let mut in_AX: u16;
  astruct_394 *paVar2;
  astruct_394 *paVar3;
  astruct_394 *paVar4;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut puVar7: *mut u8;
  let mut in_EDX: u32;
  let mut paVar8: *mut Struct57;
  astruct_395 *iVar9;
  astruct_395 *uVar9;
  let mut unaff_CS: u16;
  let mut puStack6: *mut u32;

  if (param_2 == 1) {
    pass1_1040_805a(in_EDX);
    return;
  }
  paVar2 = (astruct_394 *)FUN_1010_830a(in_AX,in_EDX,unaff_CS,_u16_1050_14cc,param_2);
  uVar5 = in_EDX;
  puStack6 = CONCAT22(uVar5,paVar2);
  paVar8 = (astruct_57 *)(in_EDX & 0xffff0000 | (uVar5 | paVar2));
  if ((uVar5 | paVar2) != 0) {
    ppcVar1 = (code **)(*puStack6 + 0x14);
    paVar3 = paVar2;
    (**ppcVar1)(0x1010,paVar2,uVar5);
    uVar6 = SUB42(paVar8,0x0);
    uVar9 = (astruct_395 *)(param_1 >> 0x10);
    iVar9 = (astruct_395 *)param_1;
    paVar4 = paVar3;
    if (iVar9->field112_0x70 != NULL) {
      paVar4 = *(astruct_394 **)&iVar9->field112_0x70;
      uVar5 = (&iVar9->field112_0x70 + 2);
      paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | (uVar5 | paVar4));
      if ((uVar5 | paVar4) != 0) {
        ppcVar1 = (code **)paVar4;
        (**ppcVar1)(0x1010,paVar4,uVar5,1);
      }
    }
    mem_op_1000_179c(0x14,paVar8);
    puVar7 = (paVar8 | paVar4);
    if (puVar7 == NULL) {
      paVar4 = NULL;
      puVar7 = NULL;
    }
    else {
      struct_1008_4c58(paVar4);
    }
    *(astruct_394 **)&iVar9->field112_0x70 = paVar4;
    (&iVar9->field112_0x70 + 0x2) = puVar7;
    pass1_1008_4d84(puVar7,iVar9->field112_0x70,CONCAT22(uVar6,paVar3));
    if (puStack6 != NULL) {
      ppcVar1 = (code **)*puStack6;
      (**ppcVar1)(0x1008,paVar2,in_EDX,1);
    }
    return;
  }
  return;
}
pub fn unk_win_ui_op_1040_8158(u32 *param_1,POINT16 param_2,mut param_3: i16)

{
  code **ppcVar1;
  let mut BVar2: bool;
  let mut iVar3: i16;
  let mut uVar4: u16;

  if (param_3 == 0x2) {
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x76) != 0) {
      ScreenToClient16((POINT16 *)CONCAT22(0x1050,&stack0xfffa),*(HWND16 *)(iVar3 + 0x6));
      GetSystemMetrics16(SM_CYCAPTION);
      BVar2 = PtInRect16((POINT16)(param_1 & 0xffff0000 | ZEXT24((RECT16 *)(iVar3 + 0x82))),
                         (RECT16 *)(iVar3 + 0x82));
      if (BVar2 != 0) {
        ppcVar1 = (code **)(*param_1 + 0x14);
        (**ppcVar1)(s_tile2_bmp_1050_1538,iVar3);
      }
    }
  }
  return;
}
pub fn check_dialog_msg_1040_81b6(mut param_1: u32)

{
  let mut BVar1: bool;
  let mut uVar2: u16;
  MSG16 local_14;

  uVar2 = (param_1 >> 0x10);
  (param_1 + 0x78) = 0x1;
  while( true ) {
    BVar1 = IsWindow16(*(HWND16 *)(param_1 + 0x6));
    if (BVar1 == 0) {
      return;
    }
    local_14.hwnd = (HWND16)&DAT_1050_1050;
    BVar1 = GetMessage16(0x0,0x0,0x0,&local_14);
    if (BVar1 == 0) break;
    IsDialogMessage16(&local_14,(HWND16)&DAT_1050_1050);
  }
  return;
}
pub fn win_ui_op_1040_81fe(mut param_1: u32)

{
  SetSysModalWindow(*(HWND16 *)(param_1 + 0x6));
  return;
}
pub fn destroy_win_1040_8212(param_1: *mut astruct_899)

{
  let mut is_window: bool;
  astruct_899 *struct_1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  struct_1 = (astruct_899 *)param_1;
  if (struct_1->field140_0x8c != 0) {
    is_window = IsWindow16(struct_1->field140_0x8c);
    if (is_window != 0) {
      DestroyWindow16(struct_1->field140_0x8c);
      struct_1->field140_0x8c = 0;
    }
  }
  return;
}



u16 pass1_1040_824a(mut param_1: u32,mut param_2: i16)

{
  if ((param_1 + 0x6) != param_2) {
    return 0x1;
  }
  return 0x0;
}



u16 FUN_1040_8260()

{
  return 0x0;
}



u16 FUN_1040_8266()

{
  return 0x0;
}
pub fn move_win_1040_826c(StructB *param_1,INT16 param_2,BOOL16 param_3)

{
  let mut IVar1: i16;
  let mut struct_b_1_hi: u16;
  let mut local_e: i16;
  let mut iStack12: i16;
  let mut iStack10: i16;
  let mut iStack8: i16;
  let mut IStack6: i16;
  let mut BStack4: bool;

  struct_b_1_hi = (param_1 >> 0x10);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_e),*(HWND16 *)(param_1 + 0x6));
  if ((param_3 == 0xffff) || (param_2 == -1)) {
    IVar1 = GetSystemMetrics16(SM_CXSCREEN);
    BStack4 = (IVar1 - (iStack10 - local_e)) / 0x2;
    IVar1 = GetSystemMetrics16(SM_CYSCREEN);
    param_2 = (IVar1 - (iStack8 - iStack12)) / 0x2;
  }
  else {
    BStack4 = param_3;
  }
  IVar1 = *(INT16 *)(param_1 + 0x6);
  IStack6 = param_2;
  MoveWindow16(0x0,IVar1,iStack10 - local_e,param_2,BStack4,IVar1);
  return;
}
pub fn draw_op_1040_82ee(astruct_14 *astruct14_param_1)

{
  astruct_14 *struct_1;
  HDC16 struct_1_hi;
  let mut local_1a: u32;
  let mut uStack22: u32;
  let mut rect_var_12: RECT16;
  let mut iStack14: i16;
  let mut iStack12: i16;
  HBRUSH16 brush_handle_1;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut iStack4: i16;

  struct_1_hi = (HDC16)(astruct14_param_1 >> 0x10);
  struct_1 = (astruct_14 *)astruct14_param_1;
  iStack6 = (struct_1->field118_0x80 - struct_1->field116_0x7c) + -0x2;
  iStack8 = (-(struct_1->field95_0x60 == 0) & 0x1e) + 0x25;
  iStack4 = iStack6;
  brush_handle_1 = CreateSolidBrush16(CONCAT22(0x100,iStack8));
  if (&struct_1[0x1].field_0x4 == 0) {
    local_1a = CONCAT22(struct_1->field98_0x66 + 0x2,struct_1->field97_0x64 + 2);
    uStack22 = CONCAT22(iStack4,iStack6);
    (struct_1 + 1) = local_1a;
    &struct_1[0x1].field_0x4 = uStack22;
  }
  rect_var_12.x = (struct_1 + 1) + 2;
  rect_var_12.y =
       (struct_1[0x1].hwnd16_field6_0x6 - &struct_1[0x1].field_0x2) / 0x2 + &struct_1[0x1].field_0x2 +
       -0x2;
  iStack14 = &struct_1[0x1].field_0x4 + -0x2;
  iStack12 = (struct_1[0x1].hwnd16_field6_0x6 - &struct_1[0x1].field_0x2) / 0x2 +
             &struct_1[0x1].field_0x2 + 2;
  FrameRect16(brush_handle_1,(RECT16 *)(struct_1 + 1),struct_1_hi);
  FrameRect16(brush_handle_1,&rect_var_12,(HDC16)&DAT_1050_1050);
  DeleteObject16(brush_handle_1);
  struct_1->field115_0x7a = &struct_1[0x1].field_0x4 + 2;
  return;
}



StructD * pass1_1040_83e6(StructD *param_1,param_2: u8)

{
  ui_cleanup_op_1040_782c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_57 *
pass1_1040_8478(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u16 ,char *param_4,char *param_5,mut param_6: u16 )

{
  let mut uVar1: u16;
  let mut iVar2: *mut Struct57;
  let mut uVar2: *mut Struct57;

  get_sys_metrics_1040_7728(param_2,0x1,0x0,0xfc3,param_6);
  uVar2 = (astruct_57 *)(param_2 >> 0x10);
  iVar2 = (astruct_57 *)param_2;
  (iVar2 + 1)->field0_0x0 = 0;
  iVar2[0x1].field5_0xa = param_3;
  iVar2[0x1].field6_0xc = 0;
  &iVar2[0x1].field_0x24 = 0;
  param_2->field0_0x0 = 0x8ddc;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  &iVar2[0x1].field8_0x10 = 0;
  iVar2[0x1].field10_0x14 = 0x12c;
  uVar1 = str_op_1008_60e8(param_1,param_5);
  iVar2[0x1].field1_0x2 = uVar1;
  iVar2[0x1].field2_0x4 = param_1;
  uVar1 = str_op_1008_60e8(param_1,param_4);
  iVar2[0x1].field3_0x6 = uVar1;
  iVar2[0x1].field4_0x8 = param_1;
  load_icon_1040_8b92(param_2);
  PTR_LOOP_1050_5df8 = NULL;
  return param_2;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 string_1040_8520(mut param_1: u32,param_2: *mut astruct_57,mut param_3: u16 ,mut param_4: u32,mut param_5: u32)

{
  let mut uVar1: u32;
  let mut puVar2: *mut u32;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut pcVar6: *mut c_char;
  let mut uVar7: u16;
  let mut iStack22: i16;
  let mut iStack16: i16;
  let mut puStack14: *mut u16;
  let mut iVar7: *mut Struct57;

  iVar7 = (astruct_57 *)param_2;
  uVar7 = (param_2 >> 0x10);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0xfc3,param_3);
  (iVar7 + 1)->field0_0x0 = 0;
  iVar7[0x1].field5_0xa = param_4;
  iVar7[0x1].field6_0xc = 0;
  &iVar7[0x1].field_0x24 = 0;
  param_2->field0_0x0 = 0x8ddc;
  iVar7->field1_0x2 = &PTR_LOOP_1050_1040;
  &iVar7[0x1].field8_0x10 = 0;
  iVar7[0x1].field10_0x14 = 0x12c;
  puVar2 = &param_5;
  iStack16 = param_4;
  if (param_4 != 0) {
    puVar2 = (&param_5 + 2);
    load_string_1010_84ac(_u16_1050_14cc,(INT16)(_u16_1050_14cc >> 0x10),param_5);
    iVar7[0x1].field3_0x6 = param_5;
    iVar7[0x1].field4_0x8 = param_1;
    iStack16 = param_4 + -0x1;
  }
  puStack14 = CONCAT22(0x1050,puVar2);
  iStack22 = 0;
  while (puVar3 = puStack14, iStack16 != 0) {
    puStack14 = (puStack14 & 0xffff0000 | (puStack14 + 0x2));
    pcVar6 = load_string_1010_847e(_u16_1050_14cc,*puVar3);
    param_1 = param_1 & 0xffff0000 | pcVar6 >> 0x10;
    uVar4 = str_op_1000_3da4(pcVar6);
    iStack22 += uVar4;
    iStack16 = iStack16 + -0x1;
  }
  uVar5 = iStack22 + 1;
  mem_op_1000_179c(uVar5,(astruct_57 *)param_1);
  iVar7[0x1].field1_0x2 = uVar5;
  iVar7[0x1].field2_0x4 = param_1;
  puStack14 = CONCAT22(0x1050,&param_5 + 2);
  iStack16 = param_4 + -0x1;
  if (param_4 + -0x1 != 0) {
    puStack14 = CONCAT22(0x1050,&stack0x0012);
    uVar1 = &iVar7[0x1].field1_0x2;
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,uVar1,(short)(uVar1 >> 0x10)
              );
    iStack16 = param_4 + -0x2;
  }
  while (puVar3 = puStack14, iStack16 != 0) {
    puStack14 = (puStack14 & 0xffff0000 | (puStack14 + 0x2));
    pcVar6 = load_string_1010_847e(_u16_1050_14cc,*puVar3);
    pass1_1000_3cea(&iVar7[0x1].field1_0x2,pcVar6);
    iStack16 = iStack16 + -0x1;
  }
  load_icon_1040_8b92(param_2);
  PTR_LOOP_1050_5df8 = NULL;
  return iVar7;
}
pub fn pass1_1040_869a(StructD *param_1)

{
  StructD *iVar1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1->address_offset_field_0x0 = 0x8ddc;
  iVar1->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(*&iVar1->field_0x90);
  fn_ptr_1000_17ce(*&iVar1->field_0x94);
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn enable_win_1040_86dc(mut param_1: u32)

{
  let mut HVar1: HWND16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  HVar1 = GetDlgItem16(0x1,*(HWND16 *)(param_1 + 0x6));
  if (HVar1 != 0) {
    EnableWindow16(0x1,HVar1);
    HVar1 = GetDlgItem16(0x2,*(HWND16 *)(param_1 + 0x6));
    if (HVar1 != 0) {
      EnableWindow16(0x1,HVar1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u8 * win_ui_op_1040_8718(param_1: *mut astruct_37)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_EDX: u32;
  let mut paVar6: *mut Struct57;
  let mut puVar7: *mut u32;
  let mut in_stack_0000fd88: u16;
  let mut in_stack_0000fd8a: u16;
  let mut in_stack_0000feac: u16;
  let mut in_stack_0000feae: u16;
  let mut in_stack_0000feb2: u16;
  let mut in_stack_0000feb4: u16;
  let mut in_stack_0000feb6: u16;
  let mut in_stack_0000feb8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut in_stack_0000fee0: u16;
  let mut in_stack_0000fee2: u16;
  i16 local_104 [0x80];
  let mut uStack4: u16;
  let mut uVar8: u16;
  astruct_37 *paVar12;
  astruct_37 *uVar12;

  uVar5 = (in_EDX >> 0x10);
  unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
  paVar12 = (astruct_37 *)param_1;
  uVar12 = (astruct_37 *)(param_1 >> 0x10);
  dialog_ui_fn_1040_78e2((StructB *)param_1);
  PTR_LOOP_1050_5df6 = (&paVar12->field1_0x4 + 2);
  if (*(i32 *)&paVar12->field_0x94 != 0) {
    unk_str_op_1000_3d3e
              ((param_1 & 0xffff0000 | ZEXT24(&paVar12->field_0x10)),*&paVar12->field_0x94);
  }
  get_sys_metrics_1040_8c66(param_1);
  uStack4 = paVar12->field138_0x98 & 0xf;
  if (uStack4 == 1) {
    iVar3 = &paVar12[0x1].field_0x8 + -0xc4;
    paVar6 = (astruct_57 *)CONCAT22(uVar5,iVar3 >> 0xf);
    &paVar12[0x1].field_0xc = iVar3 / 0x2;
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0xff,local_104,
               (short)&DAT_1050_1050);
    uVar2 = &paVar12[0x1].field_0xc;
    create_window_1040_8bea
              (paVar12,uVar12,0x1,0x1,uVar2,(uVar2 >> 0x10),CONCAT22(0x1050,local_104));
    piVar1 = &paVar12[0x1].field_0xc;
    *piVar1 = *piVar1 + 0x6c;
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0xff,local_104,
               (short)&DAT_1050_1050);
    uVar2 = &paVar12[0x1].field_0xc;
    uVar10 = uVar2;
    uVar11 = (uVar2 >> 0x10);
    uVar9 = 0x2;
  }
  else {
    if (uStack4 != 0x4) {
      iVar3 = &paVar12[0x1].field_0x8 + -0x58;
      paVar6 = (astruct_57 *)CONCAT22(uVar5,iVar3 >> 0xf);
      &paVar12[0x1].field_0xc = iVar3 / 0x2;
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0xff,local_104,
                 (short)&DAT_1050_1050);
      uVar2 = &paVar12[0x1].field_0xc;
      uVar10 = uVar2;
      uVar11 = (uVar2 >> 0x10);
      uVar5 = 0x1;
      uVar9 = 0x1;
  // TODO: goto LAB_1040_88a5;
    }
    iVar3 = &paVar12[0x1].field_0x8 + -0xc4;
    paVar6 = (astruct_57 *)CONCAT22(uVar5,iVar3 >> 0xf);
    &paVar12[0x1].field_0xc = iVar3 / 0x2;
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0xff,local_104,
               (short)&DAT_1050_1050);
    uVar2 = &paVar12[0x1].field_0xc;
    create_window_1040_8bea
              (paVar12,uVar12,0x1,0x6,uVar2,(uVar2 >> 0x10),CONCAT22(0x1050,local_104));
    piVar1 = &paVar12[0x1].field_0xc;
    *piVar1 = *piVar1 + 0x6c;
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0xff,local_104,
               (short)&DAT_1050_1050);
    uVar2 = &paVar12[0x1].field_0xc;
    uVar10 = uVar2;
    uVar11 = (uVar2 >> 0x10);
    uVar9 = 0x7;
  }
  uVar5 = 0;//
LAB_1040_88a5:
  create_window_1040_8bea(paVar12,uVar12,uVar5,uVar9,uVar10,uVar11,CONCAT22(0x1050,local_104));
  puVar7 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fee0,0x48),in_stack_0000fd88,
                           in_stack_0000feac,in_stack_0000feb2,in_stack_0000feb6);
  uVar5 = (puVar7 >> 0x10);
  local_104[0] = (puVar7 + 0xa);
  uStack4 = (puVar7 + 0xc);
  iVar3 = uStack4 - &paVar12[0x1].field_0xa;
  paVar6 = (astruct_57 *)(paVar6 & 0xffff0000 | (iVar3 >> 0xf));
  SetWindowPos16(0x40,*(INT16 *)&paVar12[0x1].field_0xa,*(INT16 *)&paVar12[0x1].field_0x8,iVar3 / 0x2,
                 (local_104[0] - &paVar12[0x1].field_0x8) / 0x2,0x0,*(HWND16 *)(&paVar12->field1_0x4 + 0x2)
                );
  PTR_LOOP_1050_5df4 = (&PTR_LOOP_1050_0000 + 1);
  unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
  destroy_win_1040_8b7e(param_1);
  PTR_LOOP_1050_5df6 = NULL;
  if (&paVar12[0x1].field_0x10 != 0) {
    puVar7 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fee2,0x37),in_stack_0000fd8a,
                             in_stack_0000feae,in_stack_0000feb4,in_stack_0000feb8);
    uVar4 = pass1_1008_ab54(puVar7);
    if (uVar4 != 0) {
      PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
    }
  }
  return PTR_LOOP_1050_5df8;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_8978(mut param_1: u16 ,mut param_2: u16 ,u32 *param_3,mut param_4: u16 )

{
  code **ppcVar1;

  unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
  win_1008_5c5c(param_1,param_2,_u16_1050_02a0,param_4);
  ppcVar1 = (code **)(*param_3 + 0x74);
  (**ppcVar1)(0x1008,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_89a4(param_1: *mut u8,u32 *param_2,param_3: *mut u16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut uVar7: u16;
  let mut puVar8: *mut u32;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000fff0: u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
  uVar1 = (param_3 + 2);
  uVar2 = *param_3;
  uVar7 = 0x1010;
  puVar8 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff0,0x2),in_stack_0000fe98,
                           in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
  uVar4 = (puVar8 >> 0x10);
  uVar5 = puVar8;
  if ((uVar5 + 0x72) != 0) {
    uVar7 = 0x1008;
    win_1008_5c7c(uVar5,uVar4,_u16_1050_02a0,CONCAT22(uVar1,uVar2));
    (param_2 + 0x8c) = uVar5;
  }
  ppcVar3 = (code **)(*param_2 + 0x74);
  (**ppcVar3)(uVar7,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn mixed_draw_op_1040_8a06(mut param_1: u16 ,param_2: *mut astruct_765)

{
  astruct_13 *paVar1;
  let mut uVar6: u8;
  HPALETTE16 HVar7;
  HANDLE16 handle;
  let mut extraout_var: u32;
  let mut extraout_DX: u16;
  astruct_765 *iVar10;
  let mut count: i16;
  let mut uVar8: u32;
  COLORREF color;
  let mut color_00: u32;
  HDC16 hdc_local_24;
  PAINTSTRUCT16 paintstruct_22;
  let mut uVar1: u8;
  let mut uVar2: u8;
  let mut uVar3: u8;
  LPCSTR uVar4;
  let mut uVar5: u16;
  astruct_766 *iVar2;

  count = (param_2 >> 0x10);
  iVar10 = (astruct_765 *)param_2;
  hdc_local_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar10->hwnd_field6_0x6);
  paVar1 = *(astruct_13 **)(_PTR_LOOP_1050_4230 + 0xe);
  HVar7 = palette_op_1008_4e08((HPALETTE16)&hdc_local_24,param_1,paVar1,(HDC16 *)CONCAT22(0x1050,&hdc_local_24));
  uVar8 = pass1_1008_4d72(paVar1);
  uVar5 = (uVar8 >> 0x10);
  iVar2 = (astruct_766 *)uVar8;
  uVar1 = iVar2->field149_0x95;
  uVar2 = iVar2->field150_0x96;
  uVar3 = iVar2->field148_0x94;
  DrawIcon16(iVar10->field141_0x8e,0xa,0x14,hdc_local_24);
  color = SetBkColor16(0x0,hdc_local_24);
  extraout_DX = (color >> 0x10);
  uVar6 = SetTextColor16(CONCAT22(CONCAT11(0x2,uVar3),CONCAT11(uVar1,uVar2)),hdc_local_24);
  color_00 = CONCAT31(extraout_var,uVar6) & 0xffff | extraout_DX << 0x10;
  handle = GetProp16(s_hfont_1050_5dfa,iVar10->hwnd_field6_0x6);
  if (handle != 0) {
    SelectObject16(handle,hdc_local_24);
  }
  DrawText16(0x10,(RECT16 *)(param_2 & 0xffff0000 | ZEXT24(&iVar10->rect_0x9e)),-0x1,
             (LPCSTR)iVar10->field142_0x90,hdc_local_24);
  if (handle != 0) {
    SelectObject16(hdc_local_24,hdc_local_24);
  }
  SetBkColor16(color,hdc_local_24);
  SetTextColor16(color_00,hdc_local_24);
  HVar7 = SelectPalette16(0x0,HVar7,hdc_local_24);
  DeleteObject16(HVar7);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar10->hwnd_field6_0x6);
  return;
}
pub fn pass1_1040_8b3c(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  if ((param_3 != NULL) &&
     ((param_3 == (&PTR_LOOP_1050_0000 + 1) || param_3 == &u16_1050_0002 ||
      (((&u16_1050_0002 + 1) < param_3 + -0x2 &&
       (param_3 + -0x6 < &u16_1050_0002)))))) {
    PTR_LOOP_1050_5df4 = NULL;
    PTR_LOOP_1050_5df8 = param_3;
    return;
  }
  post_win_msg_1040_7b3c
            ((StructC *)CONCAT22(param_2,param_1),(param_2 >> 0x10),param_3,param_3);
  return;
}
pub fn destroy_win_1040_8b7e(mut param_1: u32)

{
  DestroyWindow16(*(HWND16 *)(param_1 + 0x6));
  return;
}
pub fn load_icon_1040_8b92(param_1: *mut astruct_57)

{
  let mut bVar1: u8;
  HICON16 HVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;

  uVar3 = (param_1 >> 0x10);
  bVar1 = *(param_1 + 0x98) & 0xf0;
  if (bVar1 == 0x30) {
    uVar4 = 0x7f03;
  }
  else if ((bVar1 == 0x10) || (bVar1 == 0x10)) {
    uVar4 = 0x7f01;
  }
  else if ((bVar1 == 0x40) || (bVar1 == 0x40)) {
    uVar4 = 0x7f04;
  }
  else {
    if (bVar1 != 0x20) {
      return;
    }
    uVar4 = 0x7f02;
  }
  HVar2 = LoadIcon16(uVar4,0x0);
  *(HICON16 *)(param_1 + 0x8e) = HVar2;
  return;
}



HANDLE16 create_window_1040_8bea
                   (param_1: *mut astruct_37,mut param_2: u16 ,mut param_3: i16,mut param_4: u16 ,INT16 param_5,INT16 param_6,char *param_7)

{
  let mut hwnd: HWND16;
  HANDLE16 wparam;
  let mut LVar1: LRESULT;
  let mut uStack6: u32;

  uStack6 = 0x50010000;
  if (param_3 != 0) {
    uStack6 = 0x50010001;
  }
  if (&param_1->field_0x74 != 0) {
    uStack6 |= 0x8000000;
  }
  hwnd = CreateWindow16(0x0,(void *)CONCAT22(param_4,HINSTANCE16_1050_038c),
                        *(HINSTANCE16 *)(&param_1->field1_0x4 + 0x2),0x17,0x58,param_6,param_5,(INT16)uStack6,
                        (INT16)(uStack6 >> 0x10),param_7,s_OPButton_1050_5e00);
  wparam = GetProp16(s_hfont_1050_5e09,*(HWND16 *)(&param_1->field1_0x4 + 0x2));
  if (wparam != 0) {
    LVar1 = SendMessage16(0x1,wparam,0x30,hwnd);
    wparam = (HANDLE16)LVar1;
  }
  return wparam;
}
pub fn get_sys_metrics_1040_8c66(param_1: *mut astruct_37)

{
  let mut piVar1: *mut i16;
  let mut bVar2: u8;
  HDC16 HVar3;
  let mut IVar4: i16;
  astruct_37 *struct_1;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  struct_1 = (astruct_37 *)param_1;
  HVar3 = GetDC16(*(HWND16 *)(&struct_1->field1_0x4 + 0x2));
  draw_text_1040_8d14(param_1,HVar3);
  struct_1[0x1].field1_0x4 = *&struct_1->field144_0x9e;
  *(RECT16 **)&struct_1[0x1].field_0x8 = (struct_1 + 1)->field0_0x0;
  IVar4 = GetSystemMetrics16(SM_CYCAPTION);
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + IVar4;
  bVar2 = struct_1->field138_0x98 & 0xf0;
  if ((((bVar2 == 0x30) || (bVar2 == 0x10)) || (bVar2 == 0x40)) || (bVar2 == 0x20)) {
    IVar4 = GetSystemMetrics16(SM_CYICON);
    if (&struct_1[0x1].field_0xa < IVar4) {
      IVar4 = GetSystemMetrics16(SM_CYICON);
      *(INT16 *)&struct_1[0x1].field_0xa = IVar4;
    }
  }
  piVar1 = &struct_1[0x1].field_0x8;
  *piVar1 = *piVar1 + 0x14;
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + 0xa;
  &struct_1[0x1].field_0xe = &struct_1[0x1].field_0xa;
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + 0x30;
  HVar3 = *(HDC16 *)(&struct_1->field1_0x4 + 2);
  ReleaseDC16(HVar3,HVar3);
  return;
}
pub fn draw_text_1040_8d14(param_1: *mut astruct_37,HDC16 hdc_param_2)

{
  LPCSTR in_string;
  let mut bVar1: u8;
  let mut IVar2: i16;
  HANDLE16 handle;
  astruct_37 *struct_1;
  let mut uVar3: u16;
  HGDIOBJ16 obj_handle_1;

  uVar3 = (param_1 >> 0x10);
  struct_1 = (astruct_37 *)param_1;
  bVar1 = struct_1->field138_0x98 & 0xf0;
  if ((((bVar1 == 0x30) || (bVar1 == 0x10)) || (bVar1 == 0x40)) || (bVar1 == 0x20)) {
    struct_1->field145_0xa0 = 0xa;
    IVar2 = GetSystemMetrics16(SM_CXICON);
    struct_1->field144_0x9e = IVar2 + 0x28;
  }
  else {
    struct_1->field145_0xa0 = 0xa;
    struct_1->field144_0x9e = 0x14;
  }
  handle = GetProp16(s_hfont_1050_5e0f,*(HWND16 *)(&struct_1->field1_0x4 + 0x2));
  if (handle != 0) {
    SelectObject16(handle,hdc_param_2);
  }
  in_string = struct_1->field133_0x90;
  obj_handle_1 = (HGDIOBJ16)(in_string >> 0x10);
  DrawText16(0x410,(RECT16 *)(param_1 & 0xffff0000 | ZEXT24(&struct_1->field144_0x9e)),-0x1,in_string,hdc_param_2
            );
  if (obj_handle_1 != 0) {
    SelectObject16(obj_handle_1,hdc_param_2);
  }
  return;
}



StructD * pass1_1040_8db6(StructD *param_1,param_2: u8)

{
  pass1_1040_869a(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1040_8e58(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),(param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0x8f3c;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  return CONCAT22(param_2,param_1);
}
pub fn pass1_1040_8e82(StructD *param_1)

{
  let mut in_stack_0000ffde: u16;

  param_1->address_offset_field_0x0 = 0x8f3c;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}
pub fn enable_window_1040_8ea0(param_1: *mut u8,param_2: *mut astruct_903,mut param_3: u16 ,mut param_4: u32)

{
  let mut enable: bool;
  let mut hwnd: HWND16;

  if (param_4 == 0xf8) {
    hwnd = GetDlgItem16(0x17d8,*(HWND16 *)(param_2 + 0x6));
    enable = 0x1;
  }
  else {
    if (param_4 != 0x17d8) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4);
      return;
    }
    SetWindowPos16(0x6,0xf6,0x269,0x0,0x0,0x0,*(HWND16 *)(param_2 + 0x6));
    enable = (BOOL16)s_tile2_bmp_1050_1538;
    GetDlgItem16(0x17d8,*(HWND16 *)(param_2 + 0x6));
    hwnd = 0;
  }
  EnableWindow16(enable,hwnd);
  return;
}



StructD * pass1_1040_8f16(StructD *param_1,param_2: u8)

{
  pass1_1040_8e82(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn mixed_struct_op_1040_8fb8
               (mut param_1: u32,param_2: *mut astruct_65,mut param_3: u16 ,char *param_4,mut param_5: u16 ,mut param_6: u16 ,mut param_7: u16 ,
               mut param_8: u16 ,mut param_9: u16 ,mut param_10: u16 )

{
  let mut uVar5: u16;
  let mut uVar3: u16;
  let mut uVar6: u16;
  astruct_65 *iVar3;
  astruct_65 *uVar4;
  let mut unaff_CS: u16;
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar4 = (astruct_65 *)(param_2 >> 0x10);
  iVar3 = (astruct_65 *)param_2;
  param_2->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  &iVar3->field2_0x4 = 0;
  &iVar3->field4_0x8 = 0;
  &iVar3->field6_0xc = 0;
  &iVar3->field8_0x10 = 0;
  iVar3->field10_0x14 = 0;
  iVar3->field11_0x18 = 0;
  iVar3->field12_0x1a = param_9;
  iVar3->field13_0x1c = param_8;
  iVar3[0x1].field8_0x10 = 0x5;
  iVar3[0x1].field9_0x12 = 0;
  &iVar3[0x1].field10_0x14 = 0;
  (&iVar3[0x1].field10_0x14 + 0x2) = 0x2;
  iVar3[0x1].field11_0x18 = 0;
  iVar3[0x1].field12_0x1a = param_3;
  param_2->field0_0x0 = 0x9800;
    // this is probably just the value 0x1040
  iVar3->field1_0x2 = &PTR_LOOP_1050_1040;
  uVar1 = iVar3[0x1].field8_0x10;
  iVar3[0x1].field1_0x2 = uVar1;
  (iVar3 + 1)->field0_0x0 = uVar1;
  iVar3[0x1].field3_0x6 = 0;
  iVar3[0x1].field2_0x4 = 0;
  if ((param_7 != 0) && (param_6 != 0)) {
    iVar3[0x1].field9_0x12 = 0x1;
    uVar5 = FUN_1010_830a(0x0,param_1,unaff_CS,_u16_1050_14cc,param_7);
    iVar3->field4_0x8 = uVar5;
    iVar3->field5_0xa = param_1;
    uVar5 = FUN_1010_830a(uVar5,param_1,0x1010,_u16_1050_14cc,param_6);
    iVar3->field6_0xc = uVar5;
    iVar3->field7_0xe = param_1;
    if (param_5 == 0) {
      &iVar3->field8_0x10 = 0;
    }
    else {
      uVar5 = FUN_1010_830a(uVar5,param_1,0x1010,_u16_1050_14cc,param_5);
      iVar3->field8_0x10 = uVar5;
      iVar3->field9_0x12 = param_1;
    }
  }
  uVar6 = param_1;
  uVar2 = iVar3[0x1].field8_0x10;
  iVar3[0x1].field5_0xa = uVar2;
  iVar3[0x1].field4_0x8 = uVar2;
  &iVar3[0x1].field6_0xc = 0;
  if (param_4 != NULL) {
    uVar3 = str_op_1008_60e8(uVar6,param_4);
    iVar3->field2_0x4 = uVar3;
    iVar3->field3_0x6 = uVar6;
  }
  &iVar3->field16_0x22 = 0;
  iVar3->field14_0x1e = 0;
  iVar3->field15_0x20 = 0;
  if (_u16_1050_5e18 == NULL) {
    _u16_1050_5e18 = MakeProcInstance16(HINSTANCE16_1050_038c,call_win_proc_1040_9684);
  }
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + 1;
  return;
}
