
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + -0x1;
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn create_window_1040_92dc(param_1: *mut astruct_65)

{
  let mut hwnd: HWND16;
  pstruct65_2: *mut astruct_65;
  pstruct65_1: *mut astruct_65;
  let mut lVar1: i32;

  pstruct65_1 = (param_1 >> 0x10);
  pstruct65_2 = param_1;
  if (pstruct65_2.field11_0x18 == 0) {
    hwnd = CreateWindow16(0x0,CONCAT22(pstruct65_2.field13_0x1c,HINSTANCE16_1050_038c),
                          pstruct65_2.field12_0x1a,0x0,0x0,pstruct65_2.field15_0x20,pstruct65_2.field14_0x1e,0xb,
                          0x4000,s__1050_5e3e,s_button_1050_5e3f);
    pstruct65_2.field11_0x18 = hwnd;
    lVar1 = SetWindowLong16(_u16_1050_5e18,-0x4,hwnd);
    lVar1 = (HANDLE16)(lVar1 >> 0x10);
    pstruct65_2.field10_0x14 = lVar1;
    *(HANDLE16 *)(&pstruct65_2.field10_0x14 + 0x2) = lVar1;
    SetProp16(lVar1,s_procHi_1050_5e46,pstruct65_2.field11_0x18);
    SetProp16(*(HANDLE16 *)&pstruct65_2.field10_0x14,s_procLo_1050_5e4d,pstruct65_2.field11_0x18);
    SetProp16((HANDLE16)pstruct65_1,s_thisHi_1050_5e54,pstruct65_2.field11_0x18);
    SetProp16((HANDLE16)pstruct65_2,s_thisLo_1050_5e5b,pstruct65_2.field11_0x18);
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



pub unsafe fn pass1_1040_93e6(mut param_1: u32) -> LRESULT

{
  let mut uVar1: u16;
  let mut LVar2: LRESULT;

  uVar1 = (param_1 >> 0x10);
  LVar2 = SendMessage16(0x0,(param_1 + 0x1c),0x111,(param_1 + 0x1a));
  return LVar2;
}



pub unsafe fn send_msg_1040_9404(mut param_1: u32) -> LRESULT

{
  let mut uVar1: u16;
  let mut LVar2: LRESULT;

  uVar1 = (param_1 >> 0x10);
  LVar2 = SendMessage16(0x0,(param_1 + 0x1c),0x111,(param_1 + 0x1a));
  return LVar2;
}
pub unsafe fn pass1_1040_9422(param_1: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0x8) != 0) {
    ppcVar1 = (*param_1 + 0x10);
    (**ppcVar1)();
  }
  if ((param_1 + 0x4) != 0) {
    ppcVar1 = (*param_1 + 0x14);
    (**ppcVar1)();
  }
  return;
}
pub unsafe fn unk_draw_op_1040_9458(param_1: *mut astruct_17,param_2: u8,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut hpal: *mut u16;
  let mut obj: HPALETTE16;
  let mut uVar3: u16;
  let mut iVar4: *mut astruct_17;
  let mut uVar4: u16;
  let mut puStack8: *mut u16;
  let mut puStack6: *mut u32;
  let mut UVar2: u32;

  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (iVar4.field8_0x8.is_null() == false) {
    puStack6 = iVar4.field8_0x8;
    if ((((&iVar4.field9_0xc + 0x2) | &iVar4.field9_0xc) != 0) && ((param_2 & 1) != 0)) {
      puStack6 = iVar4.field9_0xc;
    }
    if ((iVar4.field10_0x10.is_null() == false) && ((param_2 & 0x4) != 0)) {
      puStack6 = iVar4.field10_0x10;
    }
    hpal = &param_3;
    UVar2 = *puStack6;
    ppcVar1 = (UVar2 + 0x8);
    (**ppcVar1)();
    ppcVar1 = (UVar2 + 0x4);
    (**ppcVar1)();
    obj = SelectPalette16(0x0,hpal,param_3);
    DeleteObject16(obj);
  }
  return;
}
pub unsafe fn draw_text_1040_94fc(struct_param_1: *mut astruct_37,hdc_param_2: HDC16)

{
  struct_1: *mut astruct_37;
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
  HANDLE16 HVar4;
  HANDLE16 HVar5;
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
             *(LPCSTR *)(param_1 + 0x4),hdc);
  ReleaseDC16(hdc,0x0);
  return;
}
pub unsafe fn call_win_proc_1040_9684(win_handle_1: HWND16,mut param_2: u16 ,w_param_1: WPARAM16,l_param_1: LPARAM)

{
  HANDLE16 handle_1;
  HANDLE16 handle_2;
  let mut bool_1: bool;
  let mut uVar2: u16;
  RECT16 local_1a [0x2];
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
  struct_1: *mut astruct_787;
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
  AVar2: ATOM;
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
// WARNING: Could not reconcile some variable overlaps
pub unsafe fn draw_op_1040_9948(mut param_1: u16 ,param_2: *mut astruct_71)

{
  let mut hdc16_dev_ctx_1: HDC16;
  let mut mode: i16;
  let mut uVar3: u16;
  handle: HPEN16;
  hgdiobj16_00: HPEN16;
  let mut hgdiobj_2: HGDIOBJ16;
  let mut hdc_lt_gray_brush_1: HGDIOBJ16;
  let mut cch_1: u16;
  let mut puVar1: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  struct71_var4: *mut astruct_71;
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
  _x3 = CONCAT22(iVar2 + -0x1,x3 + -1);
  if (uStack34 != 0) {
    x4 = rect_12;
    y6 = (rect_12 >> 0x10);
    rect_12 = CONCAT22(y6 + 0x2,x4 + 2);
    _x3 = CONCAT22(iVar2 + -0x3,x3 + -0x3);
    x5 = x3 + -0x1;
    y4 = (iVar2 + -1);
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
    } while (iStack78 < 1);
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
      } while ((hdc16_dev_ctx_1 + 1) < 0x2);
      y2 += 0x1;
      x1 += 0x1;
      x2 += -0x1;
      y1 += -0x1;
      iStack78 += 0x1;
    } while (iStack78 < 0x2);
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
pub unsafe fn win_op_1040_9cde(lparam_param_1: LPARAM,wparam_param_2: WPARAM16,msg_param_3: u16,hwnd_param_4: HWND16,
                     mut param_5: u16 ,mut param_6: u16 ,mut param_7: u32)

{
  let mut pbVar1: *mut u8;
  let mut iVar2: i16;
  let mut bVar3: u8;
  let mut WVar4: WPARAM16;
  let mut BVar5: bool;
  let mut uVar9: u32;
  let mut uVar6: u16;
  let mut uVar8: i16;
  let mut uVar10: u16;
  let mut win_long_11: i32;
  WPARAM16 *pWVar11;
  let mut LVar12: LRESULT;
  let mut uVar13: u32;
  let mut uVar14: u8;
  let mut uVar15: u8;
  RECT16 rect_a [0x2];
  let mut iVar3: i16;
  let mut paVar7: *mut Struct57;
  let mut hwnd_dlg_8: HWND16;

  uVar10 = (param_7 >> 0x10);
  win_long_11 = GetWindowLong16(0x0,hwnd_param_4);
  paVar7 = CONCAT22(uVar10,(win_long_11 >> 0x10));
  iVar3 = win_long_11;
  uVar8 = ((win_long_11 & 0xffff0000U) >> 0x10);
  if (msg_param_3 == 0x30) {
    (iVar3 + 0x5a) = wparam_param_2;
  }
  else {
    if (msg_param_3 < 0x31) {
      if (msg_param_3 == 0x1f) {
        (iVar3 + 0x4) = 0;
        ReleaseCapture16();
        return;
      }
//      if (0x1f < msg_param_3) goto LAB_1040_a1ae;
      bVar3 = msg_param_3;
      if (bVar3 == 0x8) {
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xf7;
        uVar6 = 0;
        BVar5 = IsWindow16(wparam_param_2);
        if (BVar5 != 0) {
          uVar13 = SendMessage16(0x0,0x0,0x87,wparam_param_2);
          uVar6 = ((uVar13 & 0x20) == 0);
        }
        (iVar3 + 0x56) = 0;
        SendMessage16(0x0,(iVar3 + 0x5c),0x401,(iVar3 + 0x2));
        if (((iVar3 + 0x5c) != 0) && ((iVar3 + 0x5c) != win_long_11)) {
          SendDlgItemMessage16(uVar6,0x1,0x404,(iVar3 + 0x5c),(iVar3 + 0x2));
        }
        (iVar3 + 0x5c) = 0;
      }
      else if (bVar3 < 0x9) {
        if (bVar3 == 1) {
          pWVar11 = (WPARAM16 *)GetWindowLong16(0x0,hwnd_param_4);
          iVar2 = pWVar11;
          uVar10 = ((pWVar11 & 0xffff0000) >> 0x10);
          (iVar2 + 0x2) = (lparam_param_1 + 0x8);
          WVar4 = GetDlgCtrlID16(hwnd_param_4);
          *pWVar11 = WVar4;
          (iVar2 + 0x56) = (lparam_param_1 + 0x12);
          unk_str_op_1000_3d3e
                    ((pWVar11 & 0xffff0000 | (iVar2 + 0x6)),*(lparam_param_1 + 0x16)
                    );
          if ((*(lparam_param_1 + 0x12) & 1) != 0) {
            SendMessage16(0x0,*pWVar11,0x401,(lparam_param_1 + 0x8));
          }
          if (((lparam_param_1 + 0x14) & 0x800) == 0) {
            return;
          }
          pbVar1 = (iVar2 + 0x4);
          *pbVar1 = *pbVar1 | 0x4;
          return;
        }
        if (bVar3 == 0x2) {
          fn_ptr_1000_17ce(win_long_11);
          SetWindowLong16(0x0,0x0,hwnd_param_4);
          return;
        }
//        if (bVar3 != 0x7) goto LAB_1040_a1ae;
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 | 0x8;
        LVar12 = SendMessage16(0x0,0x0,0x400,(iVar3 + 0x2));
        iVar2 = LVar12;
        if (((LVar12 >> 0x10) == 0x534b) && ((iVar3 + 0x5c) = iVar2, iVar2 != win_long_11))
        {
          SendDlgItemMessage16(0x1,0x0,0x404,iVar2,(iVar3 + 0x2));
        }
        SendMessage16(0x0,win_long_11,0x401,(iVar3 + 0x2));
        (iVar3 + 0x56) = 0x1;
      }
      else if (bVar3 == 0xa) {
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xfb;
        if (wparam_param_2 == 0) {
          pbVar1 = (iVar3 + 0x4);
          *pbVar1 = *pbVar1 | 0x4;
        }
      }
      else {
        if (bVar3 != 0xc) {
          if (bVar3 == 0xf) {
            draw_op_1040_9948(hwnd_param_4,win_long_11);
            return;
          }
      // TODO: goto LAB_1040_a1ae;
        }
        if (lparam_param_1 != 0) {
          unk_str_op_1000_3d3e((win_long_11 & 0xffff0000U | (iVar3 + 0x6)),lparam_param_1);
        }
      }
  // TODO: goto LAB_1040_9e20;
    }
    if (msg_param_3 == 0x200) {
      if ((*(iVar3 + 0x4) & 1) == 0) {
        return;
      }
      GetClientRect16(rect_a,&DAT_1050_1050);
      iVar2 = (iVar3 + 0x4);
      BVar5 = PtInRect16(lparam_param_1,rect_a);
      if (BVar5 == 0) {
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xfd;
      }
      else {
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 | 0x2;
      }
      lparam_param_1 = (iVar3 + 0x4) - iVar2;
    }
    else {
      if (msg_param_3 < 0x201) {
        uVar9 = msg_param_3 - 0x81;
        if (uVar9 == 0) {
          uVar14 = 0;
          uVar15 = 0;
          mem_op_1000_179c(0x5e,paVar7);
          uVar6 = paVar7 | uVar9;
          if (uVar6 == 0) {
            uVar9 = 0;
            uVar6 = 0;
          }
          else {
            pass1_1040_9824(CONCAT22(paVar7,uVar9));
          }
          SetWindowLong16(CONCAT22(uVar6,uVar9),CONCAT11(uVar15,uVar14),hwnd_param_4);
          return;
        }
        if (msg_param_3 == 0x87) {
          return;
        }
        if (msg_param_3 == 0x100) {
          if ((wparam_param_2 == 0x26) || (wparam_param_2 == 0x25)) {
            hwnd_dlg_8 = (iVar3 + 2);
            BVar5 = 0x1;
          }
          else {
            if ((wparam_param_2 != 0x28) && (wparam_param_2 != 0x27)) {
              if (((wparam_param_2 == 0x20) || (wparam_param_2 == 0xd)) && (PTR_LOOP_1050_5ed8.is_null())) {
                PTR_LOOP_1050_5ed8 = (&PTR_LOOP_1050_0000 + 1);
                pbVar1 = (iVar3 + 0x4);
                *pbVar1 = *pbVar1 | 0x2;
            // TODO: goto LAB_1040_9e20;
              }
          // TODO: goto LAB_1040_a1ae;
            }
            hwnd_dlg_8 = (iVar3 + 2);
            BVar5 = 0;
          }
          hwnd_dlg_8 = GetNextDlgTabItem16(BVar5,hwnd_param_4,hwnd_dlg_8);
          SetFocus16(hwnd_dlg_8);
          return;
        }
        if ((msg_param_3 == 0x101) && (PTR_LOOP_1050_5ed8.is_null() == false)) {
          PTR_LOOP_1050_5ed8 = null_mut();
          pbVar1 = (iVar3 + 0x4);
          *pbVar1 = *pbVar1 & 0xfd;
          InvalidateRect16(0x1,NULL,0x0);
          UpdateWindow16(hwnd_param_4);
          SendMessage16(0x0,win_long_11,0x111,(iVar3 + 0x2));
          return;
        }//
// LAB_1040_a1ae:
        DefWindowProc16(lparam_param_1,wparam_param_2,msg_param_3,hwnd_param_4);
        return;
      }
      if (msg_param_3 == 0x201) {//
// LAB_1040_9e74:
        SetFocus16(hwnd_param_4);
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 | 0x3;
        InvalidateRect16(0x1,NULL,0x0);
        UpdateWindow16(hwnd_param_4);
        SetCapture16(hwnd_param_4);
        return;
      }
      if (msg_param_3 == 0x202) {
        ReleaseCapture16();
        GetClientRect16(rect_a,&DAT_1050_1050);
        if ((*(iVar3 + 0x4) & 1) == 0) {
          return;
        }
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xfc;
        InvalidateRect16(0x1,NULL,0x0);
        UpdateWindow16(hwnd_param_4);
        BVar5 = PtInRect16(lparam_param_1,rect_a);
        if (BVar5 == 0) {
          return;
        }
        PostMessage16(0x0,win_long_11,0x111,(iVar3 + 0x2));
        return;
      }
//      if (msg_param_3 == 0x203) goto LAB_1040_9e74;
//      if (msg_param_3 != 0x404) goto LAB_1040_a1ae;
      if (wparam_param_2 == 1) {
        (iVar3 + 0x56) = 0x1;
      }
      else {
        (iVar3 + 0x56) = 0;
      }
    }
  }
  if (lparam_param_1 == 0) {
    return;
  }//
// LAB_1040_9e20:
  InvalidateRect16(0x1,NULL,0x0);
  UpdateWindow16(hwnd_param_4);
  return;
}
