




// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar14
// WARNING: Unable to use type for symbol uVar15









pub unsafe fn create_window_1040_7620(mut param_1: u32,mut param_2: i16,pstruct_param_3: *mut astruct_860,mut param_4: u16 ,mut param_5: u16 )

{
  let mut iVar1: *mut astruct_860;
  let mut uVar1: u16;
  let mut window_name: *mut c_char;
  let mut h_instance: HISTANCE16;

  window_name = load_string_1010_847e(_u16_1050_14cc,param_4);
  _h_instance = 0x50000009;
  if (param_2 != 0) {
    _h_instance = 0x50020009;
  }
  uVar1 = (pstruct_param_3 >> 0x10);
  iVar1 = pstruct_param_3;
  CreateWindow16(0x0,CONCAT22(param_5,HINSTANCE16_1050_038c),(param_1 + 0x6),
                 iVar1.field4_0x6,iVar1.field3_0x4,iVar1.field2_0x2,pstruct_param_3,_h_instance,
                 (_h_instance >> 0x10),window_name,s_button_1050_5da8);
  return;
}



pub unsafe fn get_sys_metrics_1040_7728(param_1: *mut Struct57,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 )

{
  let mut IVar1: i16;
  let mut iVar2: *mut Struct57;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  param_1.field0_0x0 = 0x389a;
  iVar2.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x3aa8;
  iVar2.field1_0x2 = 0x1008;
  iVar2.field2_0x4 = 0;
  iVar2.field3_0x6 = 0;
  iVar2.field4_0x8 = param_5;
  iVar2.field5_0xa = param_4;
  iVar2.field6_0xc = 0;
  iVar2.field78_0x60 = 0;
  iVar2.field79_0x62 = 0;
  iVar2.field80_0x64 = 0;
  iVar2.field81_0x66 = 0;
  iVar2.field82_0x68 = 0;
  iVar2.field83_0x6a = param_3;
  iVar2.field84_0x6e = param_2;
  iVar2.field85_0x70 = 0;
  iVar2.field86_0x74 = 0;
  iVar2.field87_0x76 = 0;
  iVar2.field88_0x78 = 0;
  iVar2.field105_0x8a = 0;
  iVar2.field106_0x8c = 0;
  param_1.field0_0x0 = 0x840c;
  iVar2.field1_0x2 = &PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&iVar2.field8_0x10)),0x10505db0);
  pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x7a)),NULL,0x8);
  pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x82)),NULL,0x8);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  iVar2.field79_0x62 = IVar1;
  IVar1 = GetSystemMetrics16(SM_CXBORDER);
  iVar2.field80_0x64 = IVar1;
  IVar1 = GetSystemMetrics16(SM_CYBORDER);
  iVar2.field81_0x66 = IVar1;
  return;
}
pub unsafe fn ui_cleanup_op_1040_782c(param_1: *mut StructD)

{
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut struct_1: *mut StructD;
  let mut uVar2: u16;
  let mut uVar1: u16;
  let mut puVar1: *mut u32;
  let mut fn_ptr_1: *mut *mut code;

  uVar2 = (param_1 >> 0x10);
  struct_1 = param_1;
  param_1.address_offset_field_0x0 = 0x840c;
  (struct_1 + 0x2) = &PTR_LOOP_1050_1040;
  puVar2 = (struct_1 + 0x70);
  uVar3 = (struct_1 + 0x72);
  if ((uVar3 | puVar2) != 0) {
    fn_ptr_1 = *puVar2;
    (**fn_ptr_1)();
  }
  if ((struct_1 + 0x4) != 0) {
    DeleteObject16((struct_1 + 0x4));
    (struct_1 + 0x4) = 0;
  }
  if ((struct_1 + 0x68) != 0) {
    DestroyMenu16((struct_1 + 0x68));
  }
  RemoveProp16(s_thisLo_1050_5db1,(struct_1 + 0x6));
  RemoveProp16(s_thisHi_1050_5db8,(struct_1 + 0x6));
  RemoveProp16(s_procLo_1050_5dbf,(struct_1 + 0x6));
  RemoveProp16(s_procHi_1050_5dc6,(struct_1 + 0x6));
  param_1.address_offset_field_0x0 = 0x389a;
  (struct_1 + 0x2) = 0x1008;
  return;
}
pub unsafe fn pass1_1040_78de()

{
  return;
}




pub unsafe fn dialog_ui_fn_1040_78e2(in_struct_1: *mut StructB)

{
  let mut puVar1: *mut u8;
  let mut dialog_handle: LPVOID = null_mut();
  let mut uVar2: u16;
   let mut struct_b_1: *mut StructB;
   let mut local_string_1: *mut StructB;
  let mut uVar3: u16;
  let mut lVar4: i32;
  let mut local_string_2: HANDLE16;
  let mut HStack8: HANDLE16;
// pub unsafe fn *pvStack6;
  let mut fn_ptr_1: *mut *mut code;

  local_string_1 = (in_struct_1 >> 0x10);
  struct_b_1 = in_struct_1;
  if (&struct_b_1.field6_0xc == 0) {
    uVar3 = (_u16_1050_5bc8 >> 0x10);
    puVar1 = (_u16_1050_5bc8 + 0x4);
    uVar2 = (_u16_1050_5bc8 + 0x6);
  }
  else {
    puVar1 = struct_b_1.field6_0xc;
    uVar2 = struct_b_1.field7_0xe;
  }
  pvStack6 = CONCAT22(uVar2,puVar1);
  dialog_handle =
       CreateDialog16(pvStack6,struct_b_1.max_count_field_0x10,ZEXT24(struct_b_1.field5_0xa),
                              HINSTANCE16_1050_038c);
  struct_b_1.lpvoid_field_0x8 = dialog_handle;
  GetWindowText16(0x50,in_struct_1 & 0xffff0000 | ZEXT24(&struct_b_1.field8_0x10),dialog_handle);
  lVar4 = GetWindowLong16(-0x4,struct_b_1.lpvoid_field_0x8);
  SetWindowLong16(_u16_1050_5bcc,-0x4,struct_b_1.lpvoid_field_0x8);
  SetProp16(struct_b_1,s_thisLo_1050_5dcd,struct_b_1.lpvoid_field_0x8);
  SetProp16(local_string_1,s_thisHi_1050_5dd4,struct_b_1.lpvoid_field_0x8);
  local_string_2 = lVar4;
  SetProp16(local_string_2,s_procLo_1050_5ddb,struct_b_1.lpvoid_field_0x8);
  HStack8 = (lVar4 >> 0x10);
  SetProp16(HStack8,s_procHi_1050_5de2,struct_b_1.lpvoid_field_0x8);
  fn_ptr_1 = (in_struct_1 + 0x50);
  (**fn_ptr_1)(s_tile2_bmp_1050_1538,in_struct_1);
  return;
}



pub unsafe fn pass1_1040_79c0(param_1: u32,param_2: *mut i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut cVar2: u8;
  let mut uVar3: u16;

  if (param_5 == 0xa1) {
    ppcVar1 = (*param_1 + 0x38);
    uVar3 = (**ppcVar1)();
    return uVar3;
  }
  if (param_5 < 0xa2) {
    if (param_5 == 0x85) {
      ppcVar1 = (*param_1 + 0x1c);
      (**ppcVar1)();
      return 0x1;
    }
    if (param_5 < 0x86) {
      cVar2 = param_5;
      if (cVar2 == '\x02') {
        ppcVar1 = (*param_1 + 0x24);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\x0f') {
        ppcVar1 = (*param_1 + 0x18);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\x0f') {
        ppcVar1 = (*param_1 + 0x60);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (cVar2 == '+') {
        if (*param_2 != 0x4) {
          return 0x1;
        }
        win_ui_get_prop_op_1040_9566(CONCAT22(param_3,param_2));
        return 0x1;
      }
    }
  }
  else {
    if (param_5 == 0x114) {
      ppcVar1 = (*param_1 + 0x58);
      uVar3 = (**ppcVar1)();
      return uVar3;
    }
    if (param_5 < 0x115) {
      if (param_5 == 0x104) {
        ppcVar1 = (*param_1 + 0x30);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x111) {
        ppcVar1 = (*param_1 + 0x10);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
    }
    else {
      if (param_5 == 0x115) {
        ppcVar1 = (*param_1 + 0x54);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x201) {
        ppcVar1 = (*param_1 + 0x44);
        (**ppcVar1)();
        return 0x1;
      }
      if (param_5 == 0x204) {
        ppcVar1 = (*param_1 + 0x28);
        (**ppcVar1)();
        return 0x1;
      }
    }
  }
  return 0x0;
}



pub unsafe fn post_win_msg_1040_7b3c(param_1: *mut StructC,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16) -> BOOL16

{
  let mut ppcVar1: *mut *mut code;

  if ((param_4 == 1) || (param_4 == 0x2)) {
    ppcVar1 = (param_1 + 0x14);
    (**ppcVar1)();
  }
  else if (param_4 == 0x6f) {
    ppcVar1 = (param_1 + 0x2c);
    (**ppcVar1)();
  }
  else {
    if (param_4 != 0x12e) {
      return 0x0;
    }
    PostMessage16(0x0,0xf060,0x112,(param_1 + 0x6));
  }
  return 0x1;
}
pub unsafe fn destroy_win_1040_7b98(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x74) == 0) {
    DestroyWindow16((param_1 + 0x6));
  }
  return;
}




pub unsafe fn draw_op_1040_7bb2(in_struct_1: *mut astruct_14)

{
  let mut is_iconic: bool;
  let mut x: i16;
  let mut y1: i16;
  let mut iVar5: i16;
  let mut pen_handle: HPEN16;
  let mut obj_handle: HGDIOBJ16;
  let mut y: i16;
  let mut brush_handle: HGDIOBJ16;
  let mut handle: HANDLE16;
  let mut count: u16;
  let mut count_00: i16;
   let mut struct_1: *mut astruct_14;
   let mut pRVar1: *mut RECT16;
  let mut win_long: i32;
  let mut DVar2: u32;
   let mut rect: *mut RECT16;
  let mut count_01: i16;
  let mut local_rect_12: i16;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut HStack10: HPALETTE16;
  let mut uStack8: u32;
  let mut win_hdc16_4: HDC16;
  let mut uVar5: u8;
  let mut uVar6: u8;
  let mut uVar7: u32;
  let mut pcVar3: *mut u8;
  let mut uVar3: u16;
  let mut func_ptr_1: *mut *mut code;
  let mut hdc16_dev_ctx_1: HDC16;

  pRVar1 = (in_struct_1 >> 0x10);
  struct_1 = in_struct_1;
  is_iconic = IsIconic16(struct_1.hwnd16_field6_0x6);
  if (is_iconic == 0) {
    win_hdc16_4 = GetWindowDC16(struct_1.hwnd16_field6_0x6);
    func_ptr_1 = (in_struct_1 + 0x68);
    uStack8 = (**func_ptr_1)(s_tile2_bmp_1050_1538,in_struct_1);
    if (uStack8.is_null() == false) {
      HStack10 = palette_op_1008_4e08
                           (&win_hdc16_4,(uStack8 >> 0x10) | uStack8,uStack8,
                            CONCAT22(0x1050,&win_hdc16_4));
      GetWindowRect16(CONCAT13(0x10,CONCAT12(0x50,&local_rect_12)),struct_1.hwnd16_field6_0x6);
      x = (iStack14 - local_rect_12) -0x1;
      y1 = (iStack12 - iStack16) -0x1;
      iVar5 = (-(struct_1.field95_0x60 == 0) & 0x1e) + 0x25;
      pen_handle = CreatePen16(CONCAT13(0x1,iVar5),0x0,0x0);
      obj_handle = SelectObject16(pen_handle,win_hdc16_4);
      MoveTo16(0x0,0x0,win_hdc16_4);
      LineTo16(0x0,x,win_hdc16_4);
      LineTo16(y1,x,win_hdc16_4);
      LineTo16(y1,0x0,win_hdc16_4);
      LineTo16(0x0,0x0,win_hdc16_4);
      win_i32 = GetWindowLong16(-0x10,struct_1.hwnd16_field6_0x6);
      if (((win_i32 & 0x800000) != 0) && ((win_i32 & 0x400000) != 0)) {
        y = struct_1.field96_0x62 - struct_1.field98_0x66;
        MoveTo16(y,0x0,win_hdc16_4);
        LineTo16(y,x,win_hdc16_4);
        struct_1.field115_0x7a = struct_1.field97_0x64;
        struct_1.field116_0x7c = struct_1.field98_0x66;
        struct_1.field117_0x7e = x;
        struct_1.field118_0x80 = struct_1.field96_0x62 - struct_1.field98_0x66;
        rect = pRVar1;
        hdc16_dev_ctx_1 = win_hdc16_4;
        brush_handle = GetStockObject16(BLACK_BRUSH);
        FillRect16(brush_handle,rect,hdc16_dev_ctx_1);
        if (struct_1.field112_0x76 != 0) {
          draw_op_1040_82ee(in_struct_1);
        }
        if (struct_1.field15_0x10 != '\0') {
          handle = GetProp16(s_hfont_1050_5de9,struct_1.hwnd16_field6_0x6);
          if (handle != 0) {
            SelectObject16(handle,win_hdc16_4);
          }
          SetBkColor16(0x0,win_hdc16_4);
          count_01 = 0x100;
          hdc16_dev_ctx_1 = win_hdc16_4;
          SetTextColor16(CONCAT22(0x100,iVar5),win_hdc16_4);
          count = lstrlen16(CONCAT22(hdc16_dev_ctx_1,count_01));
          DVar2 = GetTextExtent16(count,CONCAT22(count_01,win_hdc16_4),win_hdc16_4);
          count_00 = ((struct_1.field117_0x7e - struct_1.field115_0x7a) / 0x2 - DVar2 / 0x2) +
                     struct_1.field115_0x7a;
          brush_handle = (struct_1.field118_0x80 - struct_1.field116_0x7c) / 0x2 - (DVar2 >> 0x10) / 0x2;
          TextOut16(count_01,CONCAT22(count_01,win_hdc16_4),brush_handle,count_00,win_hdc16_4);
          if (count_00 != 0) {
            SelectObject16(brush_handle,win_hdc16_4);
          }
        }
      }
      func_ptr_1 = (in_struct_1 + 0x64);
      (**func_ptr_1)(s_tile2_bmp_1050_1538,in_struct_1,uStack8,win_hdc16_4);
      HStack10 = SelectPalette16(0x0,HStack10,win_hdc16_4);
      DeleteObject16(HStack10);
      SelectObject16(obj_handle,win_hdc16_4);
      DeleteObject16(pen_handle);
    }
    ReleaseDC16(win_hdc16_4,struct_1.hwnd16_field6_0x6);
    return;
  }
  return;
}



pub unsafe fn set_text_bk_color_1040_7e5e(param_1: u32,mut param_2: u16 ,mut param_3: u16 ,param_4: HDC16) -> u32

{
  let mut HVar1: HGDIOBJ16;
  let mut iVar2: *mut astruct_936;
  let mut IVar2: i16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut hdc: HDC16;
  let mut fn_ptr_1: *mut *mut code;

  HVar1 = GetStockObject16(BLACK_BRUSH);
  if (COLORREF_1050_5df0 == 0) {
    fn_ptr_1 = (*param_1 + 0x68);
    uVar3 = (**fn_ptr_1)(s_tile2_bmp_1050_1538,param_1,(param_1 + 0x6e));
    if (uVar3 == 0) {
      return 0x0;
    }
    uVar3 = pass1_1008_4d72(uVar3);
    uVar4 = (uVar3 >> 0x10);
    iVar2 = uVar3;
    COLORREF_1050_5df0 = CONCAT22(CONCAT11(0x2,iVar2.field_0x94),CONCAT11(iVar2.field_0x95,iVar2.field_0x96));
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return 0x0;
    }
    uVar4 = 0x7ed5;
    IVar2 = GetDlgCtrlID16(param_2);
    if (IVar2 == 0x14c) {
      uVar5 = 0xffff;
      hdc = 0;
  // TODO: goto LAB_1040_7f00;
    }
    if (IVar2 == 0x175) {
      uVar5 = 0xff;
      hdc = 0;
  // TODO: goto LAB_1040_7f00;
    }
  }
  uVar4 = COLORREF_1050_5df0;
  uVar5 = (COLORREF_1050_5df0 >> 0x10);
  hdc = param_4;//
// LAB_1040_7f00:
  SetTextColor16(CONCAT22(uVar5,uVar4),hdc);
  SetBkColor16(0x1000000,param_4);
  return CONCAT22(0x1050,HVar1);
}
