

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn win_ui_op_1008_2b54(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32) -> i16

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut uVar7: u16;
  let mut pcVar8: *mut c_char;
//   local_a6: *mut u32 [0x14];
let mut local_a6: [*mut u32;0x14] = [null_mut();0x14];
let mut local_56: [u8;0x50] = [0;0x50];
  let mut iStack6: i16;
  let mut iStack4: i16;
  let mut uVar6: u32;

  paVar5 = CONCAT22(in_register_0000000a,param_2);
  iStack4 = 0;
  if (_PTR_LOOP_1050_4230 == 0) {
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x5f2);
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_56),pcVar8);
    pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x57b);
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_a6),pcVar8);
    iStack4 = MessageBox16(0x21,CONCAT13(0x10,CONCAT12(0x50,local_a6)),CONCAT22(0x1050,local_56),
                           HWND16_1050_0396);
  }
  else {
    uVar7 = 0x1000;
    mem_op_1000_179c(0xb4,paVar5);
    uVar3 = paVar5 | param_1;
    uVar6 = paVar5 & 0xffff0000 | uVar3;
    if (uVar3 == 0) {
      iVar2 = 0;
      uVar4 = 0;
    }
    else {
      uVar7 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar2 = string_1040_8520(uVar6,CONCAT22(paVar5,param_1),HWND16_1050_0396,0x20021,0x5f2057b);
      uVar4 = uVar6;
    }
    local_a6[0] = CONCAT22(uVar4,iVar2);
    ppcVar1 = (*local_a6[0] + 0x74);
    iStack4 = (**ppcVar1)(uVar7,iVar2,uVar4,param_1);
  }
  iStack6 = iStack4;
  if (iStack4 != 1) {
    iStack6 = 0;
  }
  if (((iStack6 != 0) && (_u16_1050_5748 != 0)) &&
     (uVar3 = (_u16_1050_5748 + 0x8),
     local_a6[0] = (local_a6[0] & 0xffff0000 | uVar3), uVar3 != 0)) {
    PostMessage16(0x0,0xb4,0x111,(param_3 + 0x8));
    iStack6 = 0;
  }
  return iStack6;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn ui_op_1008_2c4e(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_72,mut param_4: u16 )

{
  let mut uVar1: u32;
  let mut hcursor_5: HCURSOR16;
  let mut uVar3: u16;
  let mut paVar2: *mut astruct_20;
  let mut uVar5: *mut astruct_20;
  let mut in_stack_0000fff8: u16;
  let mut uVar2: u32;
  let mut ppaVar1: *mut astruct_20;
  let mut fn_ptr_1: *mut *mut code;

  hcursor_5 = LoadCursor16(0x7f02,0x0);
  hcursor_5 = SetCursor16(hcursor_5);
  uVar5 = CONCAT22(param_1,hcursor_5);
  ppaVar1 = &(param_3)[0x1].field7_0x10;
  ppaVar1.offset_0x0 = ppaVar1.offset_0x0 + 1;
  paVar2 = param_3;
  if (&(param_3)[0x1].field5_0xc != 0) {
    uVar2 = &(param_3)[0x1].field5_0xc;
    paVar2 = *&(param_3)[0x1].field5_0xc;
    fn_ptr_1 = &paVar2.field_0x90;
    uVar5 = (**fn_ptr_1)(s_tile2_bmp_1050_1538,uVar2,(uVar2 >> 0x10));
  }
  big_switch_1008_15d4(paVar2,param_3,CONCAT22(param_2,param_4));
  uVar3 = (uVar5 >> 0x10);
  (param_3)[0x1].field5_0xc = uVar5;
  (param_3)[0x1].field6_0xe = uVar3;
  fn_ptr_1 = (*&(param_3)[0x1].field5_0xc + 0x8);
  (**fn_ptr_1)(s_tile2_bmp_1050_1538,(param_3)[0x1].field5_0xc,uVar3);
  if ((&(param_3)[0x1].field2_0x4 + 0x2) != 0) {
    uVar1 = (&(param_3)[0x1].field2_0x4 + 2);
    fn_ptr_1 = ((&(param_3)[0x1].field2_0x4 + 0x2) + 0xc)
    ;
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10),0x0);
  }
  show_win_1038_b634(_PTR_LOOP_1050_5b7c);
  show_win_1010_7a76((&(param_3)[0x1].field7_0x10 + 0x2));
  uVar1 = &(param_3)[0x1].field5_0xc;
  fn_ptr_1 = (*&(param_3)[0x1].field5_0xc + 0xc);
  (**fn_ptr_1)(0x1010,uVar1,(uVar1 >> 0x10),0x5);
  uVar1 = &(param_3)[0x1].field5_0xc;
  BringWindowToTop16((uVar1 + 0x8));
  SetCursor16(hcursor_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn post_msg_1008_2d22(param_1: *mut astruct_72)

{
  let mut piVar1: *mut i16;
  let mut puVar2: *mut u32;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: *mut astruct_72;
  let mut uVar4: u16;
  let mut puVar5: *mut u32;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;

  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  if ((iVar4.field230_0xee.is_null() == false) &&
     (piVar1 = &iVar4.field231_0xf2, *piVar1 = *piVar1 + -0x1, iVar4.field231_0xf2 < 1)) {
    puVar7 = iVar4.field230_0xee;
    ppcVar3 = (*iVar4.field230_0xee + 0x90);
    (**ppcVar3)();
    iVar4.field230_0xee = NULL;
    iVar4.field231_0xf2 = 0;
    if (iVar4.field227_0xe8.is_null() == false) {
      uVar6 = 0x3;
      puVar5 = iVar4.field227_0xe8;
      ppcVar3 = (*iVar4.field227_0xe8 + 0xc);
      (**ppcVar3)();
      show_win_1038_b68a(_PTR_LOOP_1050_5b7c);
      show_window_1010_7ace(iVar4.field232_0xf4);
      puVar2 = iVar4.field227_0xe8;
      ppcVar3 = (*iVar4.field227_0xe8 + 0x98);
      (**ppcVar3)(0x1010,puVar2,(puVar2 >> 0x10),0x1,puVar5,uVar6,puVar7);
      PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
    }
  }
  return;
}
pub unsafe fn cursor_op_1008_2dcc(mut param_1: u16 ,param_2: *mut astruct_20,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  let mut uVar1: u32;
  let mut ppcVar2: *mut *mut code;
  let mut cursor_handle: HCURSOR16;
  let mut hcursor: HCURSOR16;
  let mut extraout_DX: u16;
  let mut paVar3: *mut astruct_20;

  cursor_handle = LoadCursor16(0x7f02,0x0);
  hcursor = SetCursor16(cursor_handle);
  paVar3 = param_2;
  cursor_handle = hcursor;
  if ((&(param_2)[0x1].field2_0x4 + 0x2) != 0) {
    uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
    paVar3 = (&(param_2)[0x1].field2_0x4 + 2);
    ppcVar2 = &paVar3.field_0x90;
    (**ppcVar2)(s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10));
    param_1 = extraout_DX;
  }
  big_switch_1008_15d4(paVar3,param_2,CONCAT22(param_5,param_3));
  (&(param_2)[0x1].field2_0x4 + 0x2) = cursor_handle;
  (param_2)[0x1].field3_0x8 = param_1;
  uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
  if ((uVar1 + 0xe0) == 0) {
    uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
    ppcVar2 = ((&(param_2)[0x1].field2_0x4 + 0x2) + 0x8);
    (**ppcVar2)(s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10));
    uVar1 = (&(param_2)[0x1].field2_0x4 + 2);
    ppcVar2 = ((&(param_2)[0x1].field2_0x4 + 0x2) + 0xc);
    (**ppcVar2)(s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10),0x3);
    (param_2).field153_0xce = (&(param_2)[0x1].field2_0x4 + 2);
  }
  else {
    (&(param_2)[0x1].field2_0x4 + 0x2) = 0;
    ui_op_1008_2c4e(param_1,param_4,param_2,param_3);
    (param_2).field153_0xce = 0;
  }
  SetCursor16(hcursor);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_ui_cursor_op_1008_2e9a
               (param_1: *mut astruct_57,param_2: *mut astruct_72,param_3: *mut WNDCLASS16 ,mut param_4: u16 ,mut param_5: u16 ,
               mut param_6: u16 )

{
  let mut pcVar1: *mut c_char;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut HVar6: HCURSOR16;
  let mut uVar8: u16;
  let mut uVar9: u32;
  let mut unaff_SI: u16;
  let mut uVar7: *mut astruct_72;
  let mut in_stack_0000fc78: u16;
  let mut in_stack_0000fd9c: u16;
  let mut in_stack_0000fda2: u16;
  let mut in_stack_0000fda6: u16;
  let mut local_224: [u8;0x108] = [0;0x108];
  let mut uStack266: u16;
  let mut uStack262: u32;
  let mut local_102: [u8;0x100] = [0;0x100];

  local_102[0] = '\0';
  uStack262 =
              mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2),in_stack_0000fc78,
                              in_stack_0000fd9c,in_stack_0000fda2,in_stack_0000fda6);
  uVar3 = (uStack262 >> 0x10);
  iVar4 = uStack262;
  pcVar1 = *(iVar4 + 0x16);
  uVar5 = (iVar4 + 0x18);
  uVar9 = param_1 & 0xffff0000 | uVar5;
  uStack266 = pcVar1;
  uStack266 = uVar5 | uStack266;
  if (uStack266 == 0) {
    save_file_1008_3178(uVar5,param_2,1);
    uVar5 = uVar9;
    uVar8 = uVar5 | uStack266;
    uVar9 = uVar9 & 0xffff0000 | uVar8;
    if (uVar8 == 0) {
      PostMessage16(0x0,0x13d,0x111,HWND16_1050_0396);
      return;
    }
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_102),CONCAT22(uVar5,uStack266));
    str_1000_4d58(CONCAT22(0x1050,local_102),NULL,0x0,CONCAT22(0x1050,local_224),
                  CONCAT22(0x1050,&param_3));
    if (param_3 != '\0') {
      pass1_1000_3cea(CONCAT22(0x1050,local_224),CONCAT22(0x1050,&param_3));
    }
    struct_1010_5f1e(uVar9,uStack262,CONCAT22(0x1050,local_224));
  }
  else {
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_102),*(iVar4 + 0x1a));
    uVar5 = str_op_1000_3da4(CONCAT22(0x1050,local_102));
    if (local_102[uVar5 - 0x1] != '\\') {
      local_102[uVar5] = '\\';
      local_102[uVar5 + 0x1] = '\0';
    }
    pass1_1000_3cea(CONCAT22(0x1050,local_102),pcVar1);
  }
  if (local_102[0] != '\0') {
    uVar7 = (param_2 >> 0x10);
    send_msg_1020_097e((param_2 + 0xe8));
    uVar2 = (param_2 + 0xe8);
    UpdateWindow16((uVar2 + 0x8));
    HVar6 = LoadCursor16(0x7f02,0x0);
    param_3 = (param_3 & 0xffff0000 | HVar6);
    HVar6 = SetCursor16(HVar6);
    param_3 = CONCAT22(0x1050,local_102);
    win_ui_op_1008_1414(uVar9,param_2,param_3,param_6,param_5,param_4);
    param_3 = CONCAT22(HVar6,0x1538);
    SetCursor16(HVar6);
  }
  return;
}
