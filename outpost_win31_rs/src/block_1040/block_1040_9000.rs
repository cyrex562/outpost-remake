use crate::windef::RECT16;



pub unsafe fn mix_win_ui_op_1040_911e(param_1: *mut StructD)

{
  let mut puVar3: *mut u32;
  let mut struct_1: *mut StructD;
  let mut uVar5: u16;
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut fn_ptr_1: *mut *mut code;

  uVar5 = (param_1 >> 0x10);
  struct_1 = param_1;
  param_1.address_offset_field_0x0 = 0x9800;
  struct_1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (&struct_1.field_0x38 != 0) {
    puVar1 = struct_1.field5_0x8;
    uVar2 = struct_1.field6_0xa;
    if ((uVar2 | puVar1) != 0) {
      fn_ptr_1 = *puVar1;
      (**fn_ptr_1)();
    }
    puVar3 = struct_1.field7_0xc;
    uVar3 = struct_1.field8_0xe;
    if ((uVar3 | puVar3) != 0) {
      fn_ptr_1 = *puVar3;
      (**fn_ptr_1)();
    }
    puVar3 = &struct_1.field_0x10;
    uVar4 = struct_1.field11_0x12;
    if ((uVar4 | puVar3) != 0) {
      fn_ptr_1 = *puVar3;
      (**fn_ptr_1)();
    }
  }
  fn_ptr_1000_17ce(*&struct_1.hfile_0x4);
  SetWindowLong16(struct_1.field12_0x14,-0x4,struct_1.field13_0x18);
  RemoveProp16(s_thisLo_1050_5e1c,struct_1.field13_0x18);
  RemoveProp16(s_thisHi_1050_5e23,struct_1.field13_0x18);
  RemoveProp16(s_procLo_1050_5e2a,struct_1.field13_0x18);
  RemoveProp16(s_procHi_1050_5e31,struct_1.field13_0x18);
  RemoveProp16(s_IsDlg_1050_5e38,struct_1.field13_0x18);
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 -0x1;
  if (PTR_LOOP_1050_5e16.is_null()) {
    FreeProcInstance16(_u16_1050_5e18);
    _u16_1050_5e18 = null_mut();
  }
  param_1.address_offset_field_0x0 = 0x389a;
  struct_1.address_offset_field_0x2 = 0x1008;
  return;
}
pub unsafe fn enable_win_1040_9234(mut param_1: u32,param_2: BOOL16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x18) != 0) {
    EnableWindow16(param_2,(param_1 + 0x18));
  }
  return;
}
pub unsafe fn pass1_1040_9252(param_1: *mut astruct_65)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut iVar4: i16;
  let mut iVar3: *mut astruct_65;
  let mut uVar5: *mut astruct_65;

  uVar5 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (&iVar3.field2_0x4 != 0) {
    draw_text_1040_9650((param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
  }
  if (&iVar3.field4_0x8 != 0) {
    pass1_1040_9618((param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
  }
  uVar2 = iVar3[0x1].field6_0xc;
  if (iVar3.field16_0x22 < uVar2) {
    iVar3.field16_0x22 = uVar2;
  }
  uVar2 = iVar3[0x1].field7_0xe;
  if (iVar3.field17_0x24 < uVar2) {
    iVar3.field17_0x24 = uVar2;
  }
  iVar4 = (iVar3 + 1).field0_0x0 + iVar3[0x1].field2_0x4;
  if (iVar3.field16_0x22 < iVar4) {
    iVar3.field16_0x22 = iVar4;
  }
  iVar4 = iVar3[0x1].field1_0x2 + iVar3[0x1].field3_0x6;
  if (iVar3.field17_0x24 < iVar4) {
    iVar3.field17_0x24 = iVar4;
  }
  piVar1 = &iVar3.field16_0x22;
  *piVar1 = *piVar1 + iVar3[0x1].field8_0x10;
  piVar1 = &iVar3.field17_0x24;
  *piVar1 = *piVar1 + iVar3[0x1].field8_0x10;
  return;
}




pub unsafe fn create_window_1040_92dc(param_1: *mut astruct_65)

{
  let mut hwnd: HWND16;
   let mut pstruct65_2: *mut astruct_65;
   let mut pstruct65_1: *mut astruct_65;
  let mut lVar1: i32;

  pstruct65_1 = (param_1 >> 0x10);
  pstruct65_2 = param_1;
  if (pstruct65_2.field11_0x18 == 0) {
    hwnd = CreateWindow16(0x0,CONCAT22(pstruct65_2.field13_0x1c,HINSTANCE16_1050_038c),
                          pstruct65_2.field12_0x1a,0x0,0x0,pstruct65_2.field15_0x20,pstruct65_2.field14_0x1e,0xb,
                          0x4000,s__1050_5e3e,s_button_1050_5e3f);
    pstruct65_2.field11_0x18 = hwnd;
    lVar1 = SetWindowLong16(_u16_1050_5e18,-0x4,hwnd);
    lVar1 = (lVar1 >> 0x10);
    pstruct65_2.field10_0x14 = lVar1;
   (&pstruct65_2.field10_0x14 + 0x2) = lVar1;
    SetProp16(lVar1,s_procHi_1050_5e46,pstruct65_2.field11_0x18);
    SetProp16(&pstruct65_2.field10_0x14,s_procLo_1050_5e4d,pstruct65_2.field11_0x18);
    SetProp16(pstruct65_1,s_thisHi_1050_5e54,pstruct65_2.field11_0x18);
    SetProp16(pstruct65_2,s_thisLo_1050_5e5b,pstruct65_2.field11_0x18);
    if (pstruct65_2[0x1].field12_0x1a != 0) {
      SetProp16(0x1,s_IsDlg_1050_5e62,pstruct65_2.field11_0x18);
    }
    ShowWindow16(0x5,pstruct65_2.field11_0x18);
  }
  return;
}
pub unsafe fn mov_update_win_1040_93aa(param_1: *mut astruct_65,param_2: INT16,mut param_3: u16 )

{
  let mut iVar1: *mut astruct_65;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field14_0x1e = param_3;
  iVar1.field15_0x20 = param_2;
  MoveWindow16(0x1,iVar1.field17_0x24,iVar1.field16_0x22,param_2,iVar1.field14_0x1e,iVar1.field11_0x18);
  UpdateWindow16(iVar1.field11_0x18);
  return;
}







pub unsafe fn draw_text_1040_94fc(struct_param_1: *mut Struct37,hdc_param_2: HDC16)

{
   let mut struct_1: *mut Struct37;
  let mut struct_1_lo: u16;
  let mut colorref_2: COLORREF;
  let mut u16_var_1: u16;
  let mut u16_var3: u16;

  struct_1_lo = (struct_param_1 >> 0x10);
  struct_1 = struct_param_1;
  colorref_2 = SetBkColor16(CONCAT22(0x100,struct_1.field49_0x3a),hdc_param_2);
  SetTextColor16(CONCAT22(0x100,struct_1.field50_0x3c),hdc_param_2);
  DrawText16(0x10,(struct_param_1 & 0xffff0000 | ZEXT24(&struct_1.field40_0x2e)),-0x1,
             struct_1.field1_0x4,hdc_param_2);
  u16_var_1 = ((colorref_2 & 0xffff0000) >> 0x10);
  u16_var3 = hdc_param_2;
  SetBkColor16(colorref_2 & 0xffff0000 | hdc_param_2,hdc_param_2);
  SetTextColor16(CONCAT22(u16_var_1,u16_var3),hdc_param_2);
  return;
}
pub unsafe fn win_ui_get_prop_op_1040_9566(param_1: *mut i16)

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  let mut ppcVar3: *mut *mut code;
  let mut HVar4: HANDLE16;
  let mut HVar5: HANDLE16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut puStack12: *mut u32;

  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  if (*param_1 == 0x4) {
    uVar1 = (iVar6 + 0xc);
    HVar4 = GetProp16(s_thisHi_1050_5e6f,(iVar6 + 0xa));
    HVar5 = GetProp16(s_thisLo_1050_5e68,(iVar6 + 0xa));
    if ((HVar4 | HVar5) != 0) {
      iVar2 = (iVar6 + 0x6);
      if (iVar2 == 1) {
        puStack12 = CONCAT22(HVar4,uVar1);
        ppcVar3 = (*puStack12 + 0xc);
        (**ppcVar3)(s_tile2_bmp_1050_1538,uVar1,HVar4,(iVar6 + 0x8));
        return;
      }
      if (iVar2 == 0x2) {
        puStack12 = CONCAT22(HVar4,uVar1);
        ppcVar3 = (*puStack12 + 0x10);
        (**ppcVar3)(s_tile2_bmp_1050_1538,uVar1,HVar4,(iVar6 + 0x8));
        return;
      }
      if (iVar2 == 0x4) {
        puStack12 = CONCAT22(HVar4,uVar1);
        ppcVar3 = (*puStack12 + 0x18);
        (**ppcVar3)(s_tile2_bmp_1050_1538,uVar1,HVar4,*(iVar6 + 0x8) & 0x10);
        return;
      }
    }
  }
  return;
}
pub unsafe fn pass1_1040_9618(param_1: *mut astruct_65)

{
  let mut uVar1: u16;
  let mut iVar2: *mut astruct_65;
  let mut uVar2: u16;
  let mut uVar3: u32;

  uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar3 = pass1_1008_4772(&iVar2.field4_0x8);
  uVar1 = (uVar3 >> 0x10);
  iVar2[0x1].field2_0x4 = (uVar3 + 0x4);
  iVar2[0x1].field3_0x6 = (uVar3 + 0x8);
  return;
}
pub unsafe fn draw_text_1040_9650(param_1: *mut astruct_65)

{
  let mut hdc: HDC16;

  hdc = GetDC16(0x0);
  DrawText16(0x410,(param_1 & 0xffff0000 | (param_1 + 0x2e)),-0x1,
             (param_1 + 0x4),hdc);
  ReleaseDC16(hdc,0x0);
  return;
}
pub unsafe fn call_win_proc_1040_9684(win_handle_1: HWND16,mut param_2: u16 ,w_param_1: WPARAM16,l_param_1: LPARAM)

{
  let mut handle_1: HANDLE16;
  let mut handle_2: HANDLE16;
  let mut bool_1: bool;
  let mut uVar2: u16;
  let mut local_1a: [RECT16;0x2] = [RECT16::default();2];
  let mut var18: *mut u32;
  let mut var14: *mut u32;
  let mut var10: *mut u32;
  let mut var6: i32;
  let mut var2: u32;
  let mut var4: u16;
  let mut var11: u16;
  let mut var7: u16;
  let mut var8: u16;
  let mut var9: u16;
  let mut uVar1: u16;
  let mut fn_ptr_1: *mut *mut code;

  handle_1 = GetProp16(s_procHi_1050_5e7d,l_param_1);
  handle_2 = GetProp16(s_procLo_1050_5e76,l_param_1);
  var6 = CONCAT22(handle_1,handle_2);
  handle_1 = GetProp16(s_thisHi_1050_5e8b,l_param_1);
  handle_2 = GetProp16(s_thisLo_1050_5e84,l_param_1);
  var10 = CONCAT22(handle_1,handle_2);
  if ((handle_1 | handle_2) != 0) {
    if (l_param_1 == 0x2) {
      var18 = var10;
      var14 = var10;
      if (var10.is_null() == false) {
        fn_ptr_1 = *var10;
        (**fn_ptr_1)(s_tile2_bmp_1050_1538,handle_2,handle_1,1);
      }
    }
    else if (l_param_1 == 0x201) {
      handle_1 = GetProp16(s_IsDlg_1050_5e92,l_param_1);
      if (handle_1 == 0) {
        uVar2 = (var10 + 0x18);
        GetClientRect16(local_1a,&DAT_1050_1050);
        bool_1 = PtInRect16(CONCAT22(param_2,win_handle_1),local_1a);
        if (bool_1 == 0) {
          return;
        }
        debug_print_1008_6048(uVar1,s_button__08lx_ldown_1050_5e98);
        fn_ptr_1 = (*var10 + 0x1c);
        (**fn_ptr_1)(0x1008,var10,(var10 >> 0x10),param_2,win_handle_1,w_param_1,uVar2);
        return;
      }
    }
    else if (l_param_1 == 0x204) {
      uVar2 = (handle_2 + 0x18);
      var4 = uVar1;
      GetClientRect16(local_1a,&DAT_1050_1050);
      bool_1 = PtInRect16(CONCAT22(param_2,win_handle_1),local_1a);
      if (bool_1 == 0) {
        return;
      }
      debug_print_1008_6048(var4,s_button__08lx_rdown_1050_5eab);
      fn_ptr_1 = (*var10 + 0x20);
      (**fn_ptr_1)(0x8,var10,(var10 >> 0x10),param_2,win_handle_1,w_param_1,uVar2);
      return;
    }
  }
  if (var6 != 0) {
    CallWindowProc16(CONCAT13((param_2 >> 0x8),CONCAT12(param_2,win_handle_1)),w_param_1,l_param_1,
                     l_param_1,var6);
  }
  return;
}



pub unsafe fn pass1_1040_97da(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  mix_win_ui_op_1040_911e(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub unsafe fn pass1_1040_9824(param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0;
  (iVar1 + 0x4) = 0;
  (iVar1 + 0x56) = 0;
  (iVar1 + 0x5a) = 0;
  (iVar1 + 0x5c) = 0;
  *(iVar1 + 0x6) = 0;
  return;
}



pub unsafe fn unk_win_ui_op_1040_9854(param_1: *mut astruct_787) -> *mut u16

{
  let mut HVar1: HCURSOR16;
  let mut HVar2: HGDIOBJ16;
   let mut struct_1: *mut astruct_787;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  struct_1 = param_1;
  param_1.offset = 0x389a;
  struct_1.base = 0x1008;
  param_1.offset = 0xa230;
  struct_1.base = &PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&struct_1.field_0x4)),s_OPButton_1050_5ece);
  struct_1.field82_0x54 = 0x3;
  HVar1 = LoadCursor16(0x7f00,0x0);
  struct_1.field84_0x58 = HVar1;
  HVar2 = GetStockObject16(BLACK_BRUSH);
  struct_1.field83_0x56 = HVar2;
  reg_class_1040_98c0(param_1 & 0xffff | uVar3 << 0x10);
  return &param_1.offset;
}
pub unsafe fn reg_class_1040_98c0(mut param_1: u32)

{
  let mut BVar1: bool;
  let mut AVar2: ATOM;
  let mut wndclass: WNDCLASS16;

  wndclass.lpsz_class_name = param_1 + 0x4;
  BVar1 = GetClassInfo16(&wndclass,CONCAT22(wndclass.lpsz_class_name,0x1050),param_1);
  if (BVar1 == 0) {
    wndclass.style = (param_1 + 0x54);
    wndclass.lpfn_wnd_proc = 0x9cde;
    wndclass.lpfn_wnd_proc = SUB42(&PTR_LOOP_1050_1040,0x0);
    wndclass._6_4_ = 0x40000;
    wndclass.h_instance = HINSTANCE16_1050_038c;
    wndclass.h_icon = 0;
    wndclass.h_cursor = (param_1 + 0x58);
    wndclass.hbr_background = (param_1 + 0x56);
    wndclass.lpsz_menu_name = 0;
    wndclass.lpsz_class_name = param_1;
    AVar2 = RegisterClass16(&wndclass);
    if (AVar2 == 0) {
      fn_ptr_op_1000_24cd(0x0);
    }
  }
  return;
}



// WARNING: Variable defined which should be unmapped: iStack88
// WARNING: Variable defined which should be unmapped: x1
// WARNING: Variable defined which should be unmapped: y2
// WARNING: Variable defined which should be unmapped: x2
// WARNING: Variable defined which should be unmapped: y1

pub unsafe fn draw_op_1040_9948(mut param_1: u16 ,param_2: *mut astruct_71)

{
  let mut hdc16_dev_ctx_1: HDC16;
  let mut mode: i16;
  let mut uVar3: u16;
  let mut handle: HPEN16;
  let mut hgdiobj16_00: HPEN16;
  let mut hgdiobj_2: HGDIOBJ16;
  let mut hdc_lt_gray_brush_1: HGDIOBJ16;
  let mut cch_1: u16;
  let mut puVar1: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
   let mut struct71_var4: *mut astruct_71;
  let mut uVar7: u16;
  let mut pcVar1: *mut c_char;
  let mut uVar2: u16;
  let mut HVar3: HDC16;
  let mut iStack88: i16;
  let mut x1: i16;
  let mut y2: i16;
  let mut x2: i16;
  let mut y1: i16;
  let mut iStack78: i16;
  let mut paintstruct_42: [u8;0x20] = [0;0x20];
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut hgdiobj_1: HGDIOBJ16;
  let mut iStack28: i16;
  let mut x4: i16;
  let mut y6: i16;
  let mut x5: i16;
  let mut y4: i16;
  let mut rect_12: RECT16;
  let mut x3: i16;
  let mut y3: i16;
  let mut rect_a: i16;
  let mut iStack8: i16;
  let mut x6: i16;
  let mut y7: i16;
  let mut iVar1: i16;
  let mut iVar2: *mut astruct_782;
  let mut uVar8: u8;
  let mut uVar9: u8;
  let mut uVar14: u16;
  let mut uVar10: u8;
  let mut uVar11: u8;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar4_00: u16;

  x6 = 0x1;
  y7 = 0x1;
  rect_a = 0;
  iStack8 = 0;
  iStack28 = 0;
  hgdiobj_1 = 0;
  uVar7 = (param_2 >> 0x10);
  struct71_var4 = param_2;
  uStack32 = struct71_var4.field4_0x4 & 0x8;
  uStack34 = struct71_var4.field86_0x56 & 0x1;
  hdc16_dev_ctx_1 = BeginPaint16(CONCAT22(0x1050,paintstruct_42),param_1);
  mode = SetMapMode16(0x1,hdc16_dev_ctx_1);
  GetClientRect16(&rect_12,&DAT_1050_1050);
  iVar2 = (_x3 >> 0x10);
  _x3 = CONCAT22(iVar2 -0x1,x3 -1);
  if (uStack34 != 0) {
    x4 = rect_12;
    y6 = (rect_12 >> 0x10);
    rect_12 = CONCAT22(y6 + 0x2,x4 + 2);
    _x3 = CONCAT22(iVar2 -0x3,x3 -0x3);
    x5 = x3 -0x1;
    y4 = (iVar2 -1);
  }
  if (struct71_var4.field6_0x6 != '\0') {
    iStack28 = 0x1;
    if (struct71_var4.hgdiobj_field90_0x5a != 0) {
      hgdiobj_1 = SelectObject16(struct71_var4.hgdiobj_field90_0x5a,hdc16_dev_ctx_1);
    }
    pcVar1 = &struct71_var4.field6_0x6;
    uVar2 = uVar7;
    HVar3 = hdc16_dev_ctx_1;
    uVar3 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(pcVar1)));
    DrawText16(0x400,CONCAT22(0x1050,&rect_a),uVar3,CONCAT22(uVar2,pcVar1),HVar3);
    iStack8 = ((y3 - y7) + iStack8) / 0x2 + rect_12.y;
    y7 += iStack8;
    rect_a = ((x3 - x6) + rect_a) / 0x2 + rect_12.x;
    x6 += rect_a;
  }
    // 1050:5ec2
  handle = CreatePen16(COLORREF_1050_5ec2,0x1,0x0);
    // 1050:5ebe
  hgdiobj16_00 = CreatePen16(COLORREF_1050_5ebe,0x1,0x0);
  hgdiobj_2 = SelectObject16(handle,hdc16_dev_ctx_1);
  if (uStack34 != 0) {
    iStack78 = 0;
    loop {
      MoveTo16(y4,x4,hdc16_dev_ctx_1);
      LineTo16(y4,x5,hdc16_dev_ctx_1);
      LineTo16(y6,x5,hdc16_dev_ctx_1);
      LineTo16(y6,x4,hdc16_dev_ctx_1);
      LineTo16(y4,x4,hdc16_dev_ctx_1);
      y6 += 0x1;
      x4 += 0x1;
      x5 += -0x1;
      y4 += -0x1;
      iStack78 += 0x1;
      if iStack78 >= 1 {break;}
    }
  }
  if ((struct71_var4.field4_0x4 & 0x2) == 0) {
    y2 = (rect_12 >> 0x10);
    x2 = _x3;
    iStack78 = 0;
    x1 = rect_12.x;
    y1 = y3;
    loop {
      SelectObject16(handle,hdc16_dev_ctx_1);
      MoveTo16(y1,x1,hdc16_dev_ctx_1);
      LineTo16(y1,x2,hdc16_dev_ctx_1);
      LineTo16(y2,x2,hdc16_dev_ctx_1);
      loop {
        SelectObject16(hgdiobj16_00,hdc16_dev_ctx_1);
        LineTo16(y2,x1,hdc16_dev_ctx_1);
        LineTo16(y1,x1,hdc16_dev_ctx_1);
        if !((hdc16_dev_ctx_1 + 1) < 0x2) {break;}
      }
      y2 += 0x1;
      x1 += 0x1;
      x2 += -0x1;
      y1 += -0x1;
      iStack78 += 0x1;
      if iStack78 >= 0x2 {break;}
    }
  }
  else {
    MoveTo16(y3,rect_12.x,hdc16_dev_ctx_1);
    LineTo16(rect_12.y,rect_12.x,hdc16_dev_ctx_1);
    LineTo16(rect_12.y,x3 + 0x1,hdc16_dev_ctx_1);
    if (iStack28 != 0) {
      iStack8 += 0x2;
      rect_a += 0x2;
      x6 += 0x2;
      y7 += 0x2;
    }
  }
  MoveTo16(0x0,0x0,hdc16_dev_ctx_1);
  if (iStack28 != 0) {
    if ((struct71_var4.field4_0x4 & 0x4) == 0) {
      uVar4 = u16_1050_5ec8;
      puVar1 = u16_1050_5ec6;
      if (uStack32 != 0) {
        uVar4 = u16_1050_5ecc;
        puVar1 = u16_1050_5eca;
      }
      SetTextColor16(CONCAT22(uVar4,puVar1),hdc16_dev_ctx_1);
      SetBkColor16(0x1000000,hdc16_dev_ctx_1);
      pcVar1 = &struct71_var4.field6_0x6;
      HVar3 = hdc16_dev_ctx_1;
      uVar6 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(pcVar1)));
      DrawText16(0x1,CONCAT22(0x1050,&rect_a),uVar6,CONCAT22(uVar7,pcVar1),HVar3);
      uVar13 = s_tile2_bmp_1050_1538;
      uVar14 = 0x9c8d;
      SetTextColor16(CONCAT22(HVar3,uVar7),hdc16_dev_ctx_1);
      SetBkColor16(CONCAT22(uVar13,uVar14),hdc16_dev_ctx_1);
    }
    else {
      hdc_lt_gray_brush_1 = GetStockObject16(LTGRAY_BRUSH);
      uVar4_00 = 0;
      uVar12 = 0;
      pcVar1 = &struct71_var4.field6_0x6;
      HVar3 = hdc16_dev_ctx_1;
      cch_1 = str_op_1000_3da4((param_2 & 0xffff0000 | ZEXT24(pcVar1)));
      GrayString16(y7 - iStack8,x6 - rect_a,iStack8,rect_a,cch_1,CONCAT22(uVar7,pcVar1),
                   CONCAT22(uVar12,uVar4_00),hdc_lt_gray_brush_1,HVar3);
    }
    if (hgdiobj_1 != 0) {
      SelectObject16(hgdiobj_1,hdc16_dev_ctx_1);
    }
  }
  SelectObject16(hgdiobj_2,hdc16_dev_ctx_1);
  SetMapMode16(mode,hdc16_dev_ctx_1);
  DeleteObject16(handle);
  DeleteObject16(hgdiobj16_00);
  EndPaint16(CONCAT22(0x1050,paintstruct_42),param_1);
  return;
}
