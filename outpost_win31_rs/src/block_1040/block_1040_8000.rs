






/*
Unable to decompile 'pass1_1040_805a'
Cause:
Low-level Error: Symbol $$undef00000006 extends beyond the end of the address space
*/










pub unsafe fn move_win_1040_826c(param_1: *mut StructB,param_2: INT16,param_3: BOOL16)

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
  GetWindowRect16(CONCAT22(0x1050,&local_e),(param_1 + 0x6));
  if ((param_3 == 0xffff) || (param_2 == -1)) {
    IVar1 = GetSystemMetrics16(SM_CXSCREEN);
    BStack4 = (IVar1 - (iStack10 - local_e)) / 0x2;
    IVar1 = GetSystemMetrics16(SM_CYSCREEN);
    param_2 = (IVar1 - (iStack8 - iStack12)) / 0x2;
  }
  else {
    BStack4 = param_3;
  }
  IVar1 = (param_1 + 0x6);
  IStack6 = param_2;
  MoveWindow16(0x0,IVar1,iStack10 - local_e,param_2,BStack4,IVar1);
  return;
}
pub unsafe fn draw_op_1040_82ee(astruct14_param_1: *mut astruct_14)

{
   let mut struct_1: *mut astruct_14;
  let mut struct_1_hi: HDC16;
  let mut local_1a: u32;
  let mut uStack22: u32;
  let mut rect_var_12: RECT16;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut brush_handle_1: HBRUSH16;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut iStack4: i16;

  struct_1_hi = (astruct14_param_1 >> 0x10);
  struct_1 = astruct14_param_1;
  iStack6 = (struct_1.field118_0x80 - struct_1.field116_0x7c) -0x2;
  iStack8 = (-(struct_1.field95_0x60 == 0) & 0x1e) + 0x25;
  iStack4 = iStack6;
  brush_handle_1 = CreateSolidBrush16(CONCAT22(0x100,iStack8));
  if (&struct_1[0x1].field_0x4 == 0) {
    local_1a = CONCAT22(struct_1.field98_0x66 + 0x2,struct_1.field97_0x64 + 2);
    uStack22 = CONCAT22(iStack4,iStack6);
    (struct_1 + 1) = local_1a;
    struct_1[0x1].field_0x4 = uStack22;
  }
  rect_var_12.x = (struct_1 + 1) + 2;
  rect_var_12.y =
       (struct_1[0x1].hwnd16_field6_0x6 - &struct_1[0x1].field_0x2) / 0x2 + &struct_1[0x1].field_0x2 +
       -0x2;
  iStack14 = &struct_1[0x1].field_0x4 -0x2;
  iStack12 = (struct_1[0x1].hwnd16_field6_0x6 - &struct_1[0x1].field_0x2) / 0x2 +
             &struct_1[0x1].field_0x2 + 2;
  FrameRect16(brush_handle_1,(struct_1 + 1),struct_1_hi);
  FrameRect16(brush_handle_1,&rect_var_12,&DAT_1050_1050);
  DeleteObject16(brush_handle_1);
  struct_1.field115_0x7a = &struct_1[0x1].field_0x4 + 2;
  return;
}



pub unsafe fn pass1_1040_83e6(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  ui_cleanup_op_1040_782c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn pass1_1040_8478(mut param_1: u16 ,param_2: *mut Struct57,mut param_3: u16 ,param_4: *mut c_char,param_5: *mut c_char,mut param_6: u16 ) -> *mut Struct57

{
  let mut uVar1: u16;
  let mut iVar2: *mut Struct57;
  let mut uVar2: *mut Struct57;

  get_sys_metrics_1040_7728(param_2,0x1,0x0,0xfc3,param_6);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  (iVar2 + 1).field0_0x0 = 0;
  iVar2[0x1].field5_0xa = param_3;
  iVar2[0x1].field6_0xc = 0;
  iVar2[0x1].field_0x24 = 0;
  param_2.field0_0x0 = 0x8ddc;
  iVar2.field1_0x2 = &PTR_LOOP_1050_1040;
  iVar2[0x1].field8_0x10 = 0;
  iVar2[0x1].field10_0x14 = 0x12c;
  uVar1 = str_op_1008_60e8(param_1,param_5);
  iVar2[0x1].field1_0x2 = uVar1;
  iVar2[0x1].field2_0x4 = param_1;
  uVar1 = str_op_1008_60e8(param_1,param_4);
  iVar2[0x1].field3_0x6 = uVar1;
  iVar2[0x1].field4_0x8 = param_1;
  load_icon_1040_8b92(param_2);
  PTR_LOOP_1050_5df8 = null_mut();
  return param_2;
}






pub unsafe fn string_1040_8520(mut param_1: u32,param_2: *mut Struct57,mut param_3: u16 ,mut param_4: u32,mut param_5: u32) -> i16

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

  iVar7 = param_2;
  uVar7 = (param_2 >> 0x10);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0xfc3,param_3);
  (iVar7 + 1).field0_0x0 = 0;
  iVar7[0x1].field5_0xa = param_4;
  iVar7[0x1].field6_0xc = 0;
  iVar7[0x1].field_0x24 = 0;
  param_2.field0_0x0 = 0x8ddc;
  iVar7.field1_0x2 = &PTR_LOOP_1050_1040;
  iVar7[0x1].field8_0x10 = 0;
  iVar7[0x1].field10_0x14 = 0x12c;
  puVar2 = &param_5;
  iStack16 = param_4;
  if (param_4 != 0) {
    puVar2 = (&param_5 + 2);
    load_string_1010_84ac(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),param_5);
    iVar7[0x1].field3_0x6 = param_5;
    iVar7[0x1].field4_0x8 = param_1;
    iStack16 = param_4 -0x1;
  }
  puStack14 = CONCAT22(0x1050,puVar2);
  iStack22 = 0;
  while (puVar3 = puStack14, iStack16 != 0) {
    puStack14 = (puStack14 & 0xffff0000 | (puStack14 + 0x2));
    pcVar6 = load_string_1010_847e(_u16_1050_14cc,*puVar3);
    param_1 = param_1 & 0xffff0000 | pcVar6 >> 0x10;
    uVar4 = str_op_1000_3da4(pcVar6);
    iStack22 += uVar4;
    iStack16 = iStack16 -0x1;
  }
  uVar5 = iStack22 + 1;
  mem_op_1000_179c(uVar5,param_1);
  iVar7[0x1].field1_0x2 = uVar5;
  iVar7[0x1].field2_0x4 = param_1;
  puStack14 = CONCAT22(0x1050,&param_5 + 2);
  iStack16 = param_4 -0x1;
  if (param_4 -0x1 != 0) {
    puStack14 = CONCAT22(0x1050,&stack0x0012);
    uVar1 = &iVar7[0x1].field1_0x2;
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,uVar1,(uVar1 >> 0x10)
              );
    iStack16 = param_4 -0x2;
  }
  while (puVar3 = puStack14, iStack16 != 0) {
    puStack14 = (puStack14 & 0xffff0000 | (puStack14 + 0x2));
    pcVar6 = load_string_1010_847e(_u16_1050_14cc,*puVar3);
    pass1_1000_3cea(&iVar7[0x1].field1_0x2,pcVar6);
    iStack16 = iStack16 -0x1;
  }
  load_icon_1040_8b92(param_2);
  PTR_LOOP_1050_5df8 = null_mut();
  return iVar7;
}
pub unsafe fn pass1_1040_869a(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x8ddc;
  iVar1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(*&iVar1.field_0x90);
  fn_ptr_1000_17ce(*&iVar1.field_0x94);
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub unsafe fn enable_win_1040_86dc(mut param_1: u32)

{
  let mut HVar1: HWND16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  HVar1 = GetDlgItem16(0x1,(param_1 + 0x6));
  if (HVar1 != 0) {
    EnableWindow16(0x1,HVar1);
    HVar1 = GetDlgItem16(0x2,(param_1 + 0x6));
    if (HVar1 != 0) {
      EnableWindow16(0x1,HVar1);
    }
  }
  return;
}














pub unsafe fn destroy_win_1040_8b7e(mut param_1: u32)

{
  DestroyWindow16((param_1 + 0x6));
  return;
}
pub unsafe fn load_icon_1040_8b92(param_1: *mut Struct57)

{
  let mut bVar1: u8;
  let mut HVar2: HICON16;
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
  (param_1 + 0x8e) = HVar2;
  return;
}



pub unsafe fn create_window_1040_8bea
                   (param_1: *mut Struct37,mut param_2: u16 ,mut param_3: i16,mut param_4: u16 ,param_5: INT16,param_6: INT16,param_7: *mut c_char) -> HANDLE16

{
  let mut hwnd: HWND16;
  let mut wparam: HANDLE16;
  let mut LVar1: LRESULT;
  let mut uStack6: u32;

  uStack6 = 0x50010000;
  if (param_3 != 0) {
    uStack6 = 0x50010001;
  }
  if (&param_1.field_0x74 != 0) {
    uStack6 |= 0x8000000;
  }
  hwnd = CreateWindow16(0x0,CONCAT22(param_4,HINSTANCE16_1050_038c),
                        (&param_1.field1_0x4 + 0x2),0x17,0x58,param_6,param_5,uStack6,
                        (uStack6 >> 0x10),param_7,s_OPButton_1050_5e00);
  wparam = GetProp16(s_hfont_1050_5e09,(&param_1.field1_0x4 + 0x2));
  if (wparam != 0) {
    LVar1 = SendMessage16(0x1,wparam,0x30,hwnd);
    wparam = LVar1;
  }
  return wparam;
}
pub unsafe fn get_sys_metrics_1040_8c66(param_1: *mut Struct37)

{
  let mut piVar1: *mut i16;
  let mut bVar2: u8;
  let mut HVar3: HDC16;
  let mut IVar4: i16;
   let mut struct_1: *mut Struct37;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  struct_1 = param_1;
  HVar3 = GetDC16((&struct_1.field1_0x4 + 0x2));
  draw_text_1040_8d14(param_1,HVar3);
  struct_1[0x1].field1_0x4 = *&struct_1.field144_0x9e;
  &struct_1[0x1].field_0x8 = (struct_1 + 1).field0_0x0;
  IVar4 = GetSystemMetrics16(SM_CYCAPTION);
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + IVar4;
  bVar2 = struct_1.field138_0x98 & 0xf0;
  if ((((bVar2 == 0x30) || (bVar2 == 0x10)) || (bVar2 == 0x40)) || (bVar2 == 0x20)) {
    IVar4 = GetSystemMetrics16(SM_CYICON);
    if (&struct_1[0x1].field_0xa < IVar4) {
      IVar4 = GetSystemMetrics16(SM_CYICON);
      struct_1[0x1].field_0xa = IVar4;
    }
  }
  piVar1 = &struct_1[0x1].field_0x8;
  *piVar1 = *piVar1 + 0x14;
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + 0xa;
  struct_1[0x1].field_0xe = &struct_1[0x1].field_0xa;
  piVar1 = &struct_1[0x1].field_0xa;
  *piVar1 = *piVar1 + 0x30;
  HVar3 = *(&struct_1.field1_0x4 + 2);
  ReleaseDC16(HVar3,HVar3);
  return;
}
pub unsafe fn draw_text_1040_8d14(param_1: *mut Struct37,hdc_param_2: HDC16)

{
  let mut in_string: LPCSTR;
  let mut bVar1: u8;
  let mut IVar2: i16;
  let mut handle: HANDLE16;
   let mut struct_1: *mut Struct37;
  let mut uVar3: u16;
  let mut obj_handle_1: HGDIOBJ16;

  uVar3 = (param_1 >> 0x10);
  struct_1 = param_1;
  bVar1 = struct_1.field138_0x98 & 0xf0;
  if ((((bVar1 == 0x30) || (bVar1 == 0x10)) || (bVar1 == 0x40)) || (bVar1 == 0x20)) {
    struct_1.field145_0xa0 = 0xa;
    IVar2 = GetSystemMetrics16(SM_CXICON);
    struct_1.field144_0x9e = IVar2 + 0x28;
  }
  else {
    struct_1.field145_0xa0 = 0xa;
    struct_1.field144_0x9e = 0x14;
  }
  handle = GetProp16(s_hfont_1050_5e0f,(&struct_1.field1_0x4 + 0x2));
  if (handle != 0) {
    SelectObject16(handle,hdc_param_2);
  }
  in_string = struct_1.field133_0x90;
  obj_handle_1 = (in_string >> 0x10);
  DrawText16(0x410,(param_1 & 0xffff0000 | ZEXT24(&struct_1.field144_0x9e)),-0x1,in_string,hdc_param_2
            );
  if (obj_handle_1 != 0) {
    SelectObject16(obj_handle_1,hdc_param_2);
  }
  return;
}





pub unsafe fn pass1_1040_8e58(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32) -> *mut u16

{
  pass1_1040_b040(CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),(param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0x8f3c;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  return CONCAT22(param_2,param_1);
}
pub unsafe fn pass1_1040_8e82(param_1: *mut StructD)

{
  let mut in_stack_0000ffde: u16;

  param_1.address_offset_field_0x0 = 0x8f3c;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}
