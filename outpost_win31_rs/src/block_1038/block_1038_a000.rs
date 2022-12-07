
pub fn destroy_window_1038_a072(param_1: *mut astruct_880,mut param_2: u16 ,mut param_3: i16)

{
  if (param_3 != 0) {
    DestroyWindow16(param_1.field6_0x6);
  }
  return;
}
pub fn FUN_1038_a08c()

{
  return;
}



pub fn pass1_1038_a090(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_9fa4(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_a122(param_1: *mut astruct_57,param_2: *mut astruct_57,mut param_3: u16 ,mut param_4: u32,mut param_5: u32)

{
  get_sys_metrics_1040_7728
            (CONCAT22(param_2,param_1),param_3,param_4,param_5,(param_5 >> 0x10));
  (param_1 + 1).field0_0x0 = 0;
  (CONCAT22(param_2,param_1)).field0_0x0 = 0xa2d0;
  param_1.field1_0x2 = &u16_1050_1038;
  return CONCAT22(param_2,param_1);
}
pub fn pass1_1038_a156(param_1: *mut StructD)

{
  param_1->address_offset_field_0x0 = 0xa2d0;
  (param_1 + 0x2) = &u16_1050_1038;
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn pass1_1038_a174(mut param_1: u32,mut param_2: i16)

{
  if (param_2 == 1) {
    (param_1 + 0x8e) = 0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_ui_op_1038_a18c(param_1: *mut astruct_57,StructB *param_2,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u8;
  let mut uVar3: u16;
  let mut IVar4: i16;
  let mut uVar5: u32;
  let mut puVar6: *mut u16;
  let mut in_stack_0000fe56: u16;
  let mut in_stack_0000fe5c: u16;
  let mut in_stack_0000ff7a: u16;
  let mut in_stack_0000ff80: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff86: u16;
  let mut in_stack_0000ff8a: u16;
  let mut piVar7: *mut i16;
  let mut uVar8: u8;
  let mut uVar9: u8;
  local_2c: i16[0x2];
  let mut iStack40: i16;
  let mut puStack36: *mut u32;
  let mut iStack32: i16;
  let mut uStack30: u16;
  let mut local_1c: i16;
  let mut local_1a: [u8;0x2] = [0;0x2];
  let mut uStack24: u32;
  let mut puStack20: *mut u32;
  let mut local_10: i16;
  let mut local_e: bool;
  let mut local_c: [u8;0x6] = [0;0x6];
  let mut puStack6: *mut u32;

  puStack6 = mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(param_3,0x27),in_stack_0000fe5c,
                             in_stack_0000ff80,in_stack_0000ff86,in_stack_0000ff8a);
  uVar5 = param_1 & 0xffff0000;
  puVar6 = pass1_1008_3e38(CONCAT22(0x1050,local_c));
  uVar5 = uVar5 & 0xffff0000 | puVar6 >> 0x10;
  pass1_1008_3f62(CONCAT22(0x1050,local_c),
                  (puStack6 & 0xffff0000 | (puStack6 + 0x52)));
  puVar2 = local_c;
  pass1_1008_3e94(CONCAT22(0x1050,puVar2),CONCAT22(0x1050,&local_10),CONCAT22(0x1050,&local_e));
  uVar3 = FUN_1010_830a(puVar2,uVar5,0x1008,_u16_1050_14cc,0x1c0);
  puStack20 = CONCAT22(uVar5,uVar3);
  uStack24 = pass1_1008_4772(CONCAT22(uVar5,uVar3));
  puVar2 = local_1a;
  piVar7 = &local_1c;
  uVar8 = 0x50;
  uVar9 = 0x10;
  puStack36 = mixed_1010_20ba((uVar5 & 0xffff0000 | uStack24 >> 0x10),_u16_1050_0ed0,
                              CONCAT22(piVar7,0x48),in_stack_0000fe56,in_stack_0000ff7a,in_stack_0000ff80,
                              in_stack_0000ff84);
  pass1_1008_3e94((puStack36 & 0xffff0000 | (puStack36 + 0xe)),
                  CONCAT13(uVar9,CONCAT12(uVar8,piVar7)),CONCAT22(0x1050,puVar2));
  uVar3 = (puStack36 >> 0x10);
  uStack30 = (puStack36 + 0xa);
  iStack32 = (puStack36 + 0xc);
  local_10 += (iStack32 * 0xa) / 0x258 + (uStack24 + 0x8) + local_1c;
  GetWindowRect16(CONCAT22(0x1050,local_2c),(param_2 + 0x6));
  IVar4 = GetSystemMetrics16(SM_CXSCREEN);
  local_e = (IVar4 - (iStack40 - local_2c[0])) / 0x2;
  move_win_1040_826c(param_2,local_10,local_e);
  if (puStack20.is_null() == false) {
    uVar3 = (puStack20 >> 0x10);
    ppcVar1 = *puStack20;
    (**ppcVar1)(&PTR_LOOP_1050_1040,puStack20,uVar3,0x1,uVar3);
  }
  return;
}



pub unsafe fn FUN_1038_a2a4() -> u16

{
  return 0x0;
}



pub fn pass1_1038_a2aa(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a156(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1038_a33c(param_1: *mut u16,mut param_2: u16 )

{
  let mut paVar1: *mut Struct57;

  paVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfc7));
  *param_1 = 0xa428;
  (param_1).field1_0x2 = &u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_a36a(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xa428;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn show_win_1038_a396(mut param_1: u16 ,mut param_2: u16 ,StructB *param_3)

{
  let mut in_register_0000000a: u16;
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut in_stack_0000ffaa: u16;

  uVar1 = CONCAT22(in_register_0000000a,param_2);
  dialog_ui_fn_1040_78e2(param_3);
  unk_win_ui_op_1038_a18c(uVar1,param_3,in_stack_0000ffaa);
  win_1008_5c7c(param_1,uVar1,_u16_1050_02a0,0x10001);
  uVar2 = (param_3 >> 0x10);
  (param_3 + 0x8c) = param_1;
  ShowWindow16(0x5,(param_3 + 0x6));
  return;
}
pub fn destroy_win_1038_a3d2(mut param_1: u32)

{
  hwnd: u16;

  hwnd = GetWindowWord16(-0x8,(param_1 + 0x6));
  PostMessage16(0x0,0x105,0x111,hwnd);
  destroy_win_1040_7b98(param_1);
  return;
}



pub fn pass1_1038_a402(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a36a(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1038_a494(param_1: *mut u16,mut param_2: u16 )

{
  let mut paVar1: *mut Struct57;

  paVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfc8));
  *param_1 = 0xa62e;
  (param_1).field1_0x2 = &u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_a4c2(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xa62e;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_a4ee(mut param_1: u16 ,mut param_2: u16 ,StructB *struct_b_param_1)

{
  let mut lp_string: u32;
  let mut hwnd: HWND16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u16;
  StructB *struct_b_1;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  let mut LVar5: LRESULT;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffec: u16;

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  win_1008_5c7c(param_1,paVar1,_u16_1050_02a0,0x20001);
  uVar3 = (struct_b_param_1 >> 0x10);
  struct_b_1 = (StructB *)struct_b_param_1;
  (struct_b_1 + 0x7).field0_0x0 = param_1;
  puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(in_stack_0000ffec,0x2),in_stack_0000fe94,
                           in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
  uVar2 = (paVar1 >> 0x10);
  lp_string = (puVar4 + 0x6c);
  hwnd = GetDlgItem16(0x114,struct_b_1->lpvoid_field_0x8);
  SetWindowText16(lp_string,hwnd);
  SetFocus16(hwnd);
  LVar5 = SendMessage16(-0x10000,0x0,0x401,hwnd);
  unk_win_ui_op_1038_a18c(CONCAT22(uVar2,(LVar5 >> 0x10)),struct_b_param_1,in_stack_0000ff9e);
  ShowWindow16(0x5,struct_b_1->lpvoid_field_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_a584(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: i16)

{
  let mut hwnd: HWND16;
  let mut uVar1: u16;
  hwnd_00: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  paVar3: *mut astruct_486;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000ff70: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7a: u16;
  let mut puVar4: *mut u8;
  let mut uVar5: u16;
  let mut local_52: [u8;0x50] = [0;0x50];

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  if (param_4 != 0) {
    hwnd = GetDlgItem16(0x114,(param_2 + 0x6));
    GetWindowText16(0x50,CONCAT22(0x1050,local_52),hwnd);
    uVar1 = str_op_1000_3da4(CONCAT22(0x1050,local_52));
    if (uVar1 != 0) {
      puVar4 = local_52;
      uVar5 = SUB42(&DAT_1050_1050,0x0);
      paVar3 =
               mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(puVar4,0x2),in_stack_0000fe4c,
                               in_stack_0000ff70,in_stack_0000ff76,in_stack_0000ff7a);
      pass1_1010_6006((paVar3 >> 0x10),paVar3,CONCAT22(uVar5,puVar4));
      hwnd_00 = GetWindowWord16(-0x8,(param_2 + 0x6));
      PostMessage16(0x0,0x105,0x111,hwnd_00);
      destroy_win_1040_7b98(CONCAT22(param_3,param_2));
    }
  }
  return;
}



pub fn pass1_1038_a608(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a4c2(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1038_a69a(param_1: *mut u16,mut param_2: u16 )

{
  let mut paVar1: *mut Struct57;

  paVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfc9));
  *param_1 = 0xa832;
  (param_1).field1_0x2 = &u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_a6c8(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xa832;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_a6f4(mut param_1: u16 ,StructB *param_2)

{
  let mut lp_string: u32;
  let mut hwnd: HWND16;
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut uVar3: u32;
  StructB *struct_b_3;
  let mut uVar5: u16;
  let mut puVar6: *mut u32;
  let mut LVar7: LRESULT;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffec: u16;
  let mut uVar4: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(param_2);
  puVar6 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(in_stack_0000ffec,0x2),in_stack_0000fe94,
                           in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
  uVar4 = (paVar2 >> 0x10);
  lp_string = (puVar6 + 0x68);
  uVar5 = (param_2 >> 0x10);
  struct_b_3 = (StructB *)param_2;
  hwnd = GetDlgItem16(0x115,struct_b_3->lpvoid_field_0x8);
  SetWindowText16(lp_string,hwnd);
  SetFocus16(hwnd);
  LVar7 = SendMessage16(-0x10000,0x0,0x401,hwnd);
  uVar3 = CONCAT22(uVar4,(LVar7 >> 0x10));
  uVar1 = LVar7;
  unk_win_ui_op_1038_a18c(uVar3,param_2,in_stack_0000ff9e);
  win_1008_5c7c(uVar1,uVar3,_u16_1050_02a0,0x30001);
  (struct_b_3 + 0x7).field0_0x0 = uVar1;
  ShowWindow16(0x5,struct_b_3->lpvoid_field_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_a788(mut param_1: u16 ,mut param_2: u32,mut param_3: i16)

{
  let mut hwnd: HWND16;
  let mut uVar1: u16;
  hwnd_00: u16;
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



pub fn pass1_1038_a80c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a6c8(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1038_a89e(param_1: *mut u16,mut param_2: u16 )

{
  let mut paVar1: *mut Struct57;

  paVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfca));
  *param_1 = 0xab16;
  (param_1).field1_0x2 = &u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_a8cc(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xab16;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}
pub fn enable_win_1038_a8f8(StructC *param_1,mut param_2: u16 ,TwoWords param_3)

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_a972(StructB *struct_b_param_1)

{
  let mut hwnd: HWND16;
  let mut BVar1: bool;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut uVar2: u32;
  StructB *struct_b_3;
  let mut uVar4: u16;
  let mut LVar5: LRESULT;
  let mut in_stack_0000ffaa: u16;

  uVar3 = (in_EDX >> 0x10);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar4 = (struct_b_param_1 >> 0x10);
  struct_b_3 = (StructB *)struct_b_param_1;
  SendDlgItemMessage16(0x0,0x1,0x401,0x116,struct_b_3->lpvoid_field_0x8);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0x401,0x11a,struct_b_3->lpvoid_field_0x8);
  uVar2 = CONCAT22(uVar3,(LVar5 >> 0x10));
  hwnd = GetDlgItem16(0x11a,struct_b_3->lpvoid_field_0x8);
  BVar1 = EnableWindow16(0x0,hwnd);
  win_1008_5c7c(BVar1,uVar2,_u16_1050_02a0,0x40001);
  (struct_b_3 + 0x7).field0_0x0 = BVar1;
  unk_win_ui_op_1038_a18c(uVar2,struct_b_param_1,in_stack_0000ffaa);
  ShowWindow16(0x5,struct_b_3->lpvoid_field_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_sys_op_1038_a9fa(mut param_1: u32,mut param_2: i16)

{
  hwnd: u16;
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



pub fn pass1_1038_aaf0(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a8cc(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_ab82(param_1: *mut astruct_57,mut param_2: u16 )

{
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfd3,param_2);
  param_1.field0_0x0 = 0xad72;
  (param_1 + 0x2) = &u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_abb0(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xad72;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn set_win_pos_1038_abdc(param_1: *mut astruct_940)

{
  let mut hwnd: HWND16;
  iVar1: *mut astruct_940;
  let mut uVar1: u16;
  let mut in_stack_0000fff0: i16;
  let mut local_a: i16;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut iStack4: i16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  GetWindowRect16(CONCAT22(0x1050,&local_a),iVar1.field6_0x6);
  hwnd = GetDlgItem16(0xfd7,iVar1.field6_0x6);
  GetWindowRect16(CONCAT22(0x1050,&stack0xffee),hwnd);
  iStack6 -= local_a;
  iStack4 = (in_stack_0000fff0 - iStack8) + -0x2;
  SetWindowPos16(0x6,iStack4,iStack6,0x0,0x0,0x0,iVar1.field6_0x6);
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar5
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_ui_op_1038_ac38(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,hdc_param_5: HDC16)

{
  let mut IVar1: i16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar6: u32;
  let mut uVar4: u8
  iVar1: *mut astruct_46;
  iVar2: *mut astruct_786;
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



pub fn pass1_1038_ad4c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_abb0(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1038_adde(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  pass1_1038_9b72(param_1,param_2,param_3,param_4);
  CONCAT22(param_2,param_1) = 0xae4e;
  (param_1 + 0x2) = &u16_1050_1038;
  return CONCAT22(param_2,param_1);
}
pub fn pass1_1038_ae08(param_1: *mut StructD)

{
  let mut in_stack_0000ffda: u16;

  param_1->address_offset_field_0x0 = 0xae4e;
  (param_1 + 0x2) = &u16_1050_1038;
  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  return;
}



pub fn pass1_1038_ae28(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_ae08(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_aeca(param_1: *mut StructD) -> *mut StructD

{
  let mut uVar1: u16;
  let mut addr_offset_b6: u16;
  let mut seg_addr_180: u16;
  let mut u8_array_5c: [u8;0x5a] = [0;0x5a];

  uVar1 = (param_1 >> 0x10);
  (param_1 + 0xac) = 0;
  (param_1 + 0xae) = 0;
  if (_PTR_LOOP_1050_5b7c.is_null()) {
    _PTR_LOOP_1050_5b7c = param_1;
  }
  pass1_1000_4906(param_1,NULL,0xac);
  unk_draw_op_1008_80ee(CONCAT22(0x1050,u8_array_5c));
  unk_win_ui_op_1040_9854(CONCAT22(0x1050,&addr_offset_b6));
  addr_offset_b6 = 0x389a;
  seg_addr_180 = 0x1008;
  pass1_1008_8168(CONCAT22(0x1050,u8_array_5c));
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_af34()

{
  _PTR_LOOP_1050_5b7c = 0;
  return;
}



pub fn pass1_1038_af40(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: i16) -> u32

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut puVar3: *mut u8;
  let mut uVar4: u16;
  let mut puVar5: *mut u8;
  let mut pSVar6: *mut StructD;
  let mut in_register_0000000a: u16;
  let mut pSVar7: *mut StructD;
  let mut uVar8: u32;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut paVar12: *mut Struct57;
  let mut puVar13: *mut u16;
  let mut in_stack_0000fe74: u16;
  let mut in_stack_0000fe86: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd4: u32;
  let mut in_stack_0000ffde: u16;

  paVar12 = CONCAT22(in_register_0000000a,param_2);
  puVar3 = bring_win_to_top_1038_b72e(param_3,param_5);
  iVar9 = param_3;
  uVar10 = (param_3 >> 0x10);
  if (puVar3.is_null() == false) goto LAB_1038_b61f;
  uVar11 = SUB42(&u16_1050_1038,0x0);
  PTR_LOOP_1050_5b82 = puVar3;
  switch(param_5) {
  case 0x1:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
    if ((paVar12 | param_1) == 0) {//
// LAB_1038_afa0:
      uVar11 = 0x1000;
      param_1 = 0;
      pSVar6 = NULL;
    }
    else {
      paVar12 = pass1_1038_9f76(CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
      pSVar6 = (paVar12 >> 0x10);
      param_1 = paVar12;
    }
    break;
  case 0x2:
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_181c(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x3:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    puVar5 = (paVar12 | param_1);
    if (puVar5.is_null()) goto LAB_1038_afa0;
    paVar12 = pass1_1038_e99a(puVar5,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x4:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    puVar5 = (paVar12 | param_1);
    if (puVar5.is_null()) goto LAB_1038_afa0;
    paVar12 = pass1_1038_c7b8(puVar5,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x5:
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_23ea(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4,in_stack_0000ffd4);
    break;
  case 0x6:
    mem_op_1000_179c(0x92,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar12 = pass1_1040_06e8(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x7:
    mem_op_1000_179c(0x9c,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_4068(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x8:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_b772(pSVar6,CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x9:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_e140(CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0xa:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a33c(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (puVar13 >> 0x10);
    param_1 = puVar13;
    break;
  case 0xb:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a494(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (puVar13 >> 0x10);
    param_1 = puVar13;
    break;
  case 0xc:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a69a(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (puVar13 >> 0x10);
    param_1 = puVar13;
    break;
  case 0xd:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a89e(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (puVar13 >> 0x10);
    param_1 = puVar13;
    break;
  case 0xe:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x94,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_e69a(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0xf:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x94,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_cd06(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x10:
    mem_op_1000_179c(0x92,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar12 = pass1_1040_0bfc(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x11:
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_0e1c(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,param_4);
    break;
  case 0x12:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    paVar12 = pass1_1038_d756(pSVar6,CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x13:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    puVar5 = (paVar12 | param_1);
    if (puVar5.is_null()) goto LAB_1038_afa0;
    paVar12 = pass1_1038_cad8(puVar5,CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x14:
    mem_op_1000_179c(0xaa,paVar12);
    uVar4 = paVar12 | param_1;
    uVar8 = paVar12 & 0xffff0000 | uVar4;
    if (uVar4 == 0) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_1f5a(CONCAT22(paVar12,param_1),param_4,uVar8);
    pSVar6 = uVar8;
    break;
  case 0x15:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_d242(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x16:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_eeda(pSVar6,CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x17:
    mem_op_1000_179c(0x96,paVar12);
    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    uVar11 = 0x1018;
    paVar12 = pass1_1018_5e26(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  default:
// TODO: goto switchD_1038_b581_caseD_18;
  case 0x19:
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_1cb4(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x1a:
    mem_op_1000_179c(0x92,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar12 = pass1_1040_123e(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x1b:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_ab82(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x1c:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_e2d0(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x1d:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_eb9e(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x1e:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x29e,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_bddc(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x1f:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_c4a2(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x20:
    mem_op_1000_179c(0x29a,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_2ea2(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x21:
    mem_op_1000_179c(0xa6,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_3966(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x22:
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_34a2(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x23:
    mem_op_1000_179c(0x9c,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_ac84(pSVar6,CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x25:
    mem_op_1000_179c(0xa0,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_ca16(pSVar6,CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x26:
    mem_op_1000_179c(0xa2,paVar12);
    uVar4 = paVar12 | param_1;
    pSVar7 = (paVar12 & 0xffff0000 | uVar4);
    if (uVar4 == 0) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_d0f8(CONCAT22(paVar12,param_1),param_4,in_stack_0000ffde,pSVar7,in_stack_0000fe74,
                    in_stack_0000fe86,in_stack_0000ff98,in_stack_0000ff9e,in_stack_0000ffa2,in_stack_0000ffaa,
                    in_stack_0000ffb0,in_stack_0000ffb4,in_stack_0000ffcc);
    pSVar6 = pSVar7;
    break;
  case 0x27:
    uVar11 = 0x1000;
    mem_op_1000_179c(0xa0,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_88f2(CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x28:
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (paVar12 | param_1);
    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_6402(pSVar6,CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x29:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x98,paVar12);
    uVar4 = paVar12 | param_1;
    if (uVar4 == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_7d10(uVar4,CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x2a:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x98,paVar12);
    puVar5 = (paVar12 | param_1);
    if (puVar5.is_null()) goto LAB_1038_afa0;
    paVar12 = pass1_1038_8caa(puVar5,CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;
  }
  (param_5 * 0x4 + iVar9) = param_1;
  *(StructD **)(param_5 * 0x4 + iVar9 + 0x2) = pSVar6;
switchD_1038_b581_caseD_18:
  if ((param_5 * 0x4 + iVar9) != 0) {
    if ((iVar9 + 0xae) != 0) {
      uVar2 = (param_5 * 0x4 + iVar9);
      (uVar2 + 0x6e) = (iVar9 + 0xae);
    }
    (iVar9 + 0xae) = 0;
    uVar2 = (param_5 * 0x4 + iVar9);
    ppcVar1 = ((param_5 * 0x4 + iVar9) + 0x8);
    (**ppcVar1)(uVar11,uVar2,(uVar2 >> 0x10));
  }//
// LAB_1038_b61f:
  return CONCAT22((param_5 * 0x4 + iVar9 + 0x2),(param_5 * 0x4 + iVar9));
}
