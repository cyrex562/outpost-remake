
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn mix_win_ui_op_1040_911e(StructD *param_1)

{
  u32 *puVar3;
  StructD *struct_1;
  let mut uVar5: u16;
  u32 *puVar1;
  u32 *puVar2;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  code **fn_ptr_1;

  uVar5 = ((u32)param_1 >> 0x10);
  struct_1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x9800;
  struct_1->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (&struct_1->field_0x38 != 0x0) {
    puVar1 = (u32 *)struct_1->field5_0x8;
    uVar2 = struct_1->field6_0xa;
    if ((uVar2 | puVar1) != 0x0) {
      fn_ptr_1 = (code **)*puVar1;
      (**fn_ptr_1)();
    }
    puVar3 = (u32 *)struct_1->field7_0xc;
    uVar3 = struct_1->field8_0xe;
    if ((uVar3 | puVar3) != 0x0) {
      fn_ptr_1 = (code **)*puVar3;
      (**fn_ptr_1)();
    }
    puVar3 = (u32*)&struct_1->field_0x10;
    uVar4 = struct_1->field11_0x12;
    if ((uVar4 | puVar3) != 0x0) {
      fn_ptr_1 = (code **)*puVar3;
      (**fn_ptr_1)();
    }
  }
  fn_ptr_1000_17ce(*(char **)&struct_1->hfile_0x4);
  SetWindowLong16(struct_1->field12_0x14,-0x4,struct_1->field13_0x18);
  RemoveProp16(s_thisLo_1050_5e1c,struct_1->field13_0x18);
  RemoveProp16(s_thisHi_1050_5e23,struct_1->field13_0x18);
  RemoveProp16(s_procLo_1050_5e2a,struct_1->field13_0x18);
  RemoveProp16(s_procHi_1050_5e31,struct_1->field13_0x18);
  RemoveProp16(s_IsDlg_1050_5e38,struct_1->field13_0x18);
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + -0x1;
  if (PTR_LOOP_1050_5e16 == NULL) {
    FreeProcInstance16(_u16_1050_5e18);
    _u16_1050_5e18 = NULL;
  }
  param_1->address_offset_field_0x0 = 0x389a;
  struct_1->address_offset_field_0x2 = 0x1008;
  return;
}
pub fn enable_win_1040_9234(mut param_1: u32,BOOL16 param_2)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if (((int)param_1 + 0x18) != 0x0) {
    EnableWindow16(param_2,*(HWND16 *)((int)param_1 + 0x18));
  }
  return;
}
pub fn pass1_1040_9252(param_1: *mut astruct_65)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut iVar4: i16;
  astruct_65 *iVar3;
  astruct_65 *uVar5;

  uVar5 = (astruct_65 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_65 *)param_1;
  if (*(i32 *)&iVar3->field2_0x4 != 0x0) {
    draw_text_1040_9650((astruct_65 *)((u32)param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
  }
  if (*(i32 *)&iVar3->field4_0x8 != 0x0) {
    pass1_1040_9618((astruct_65 *)((u32)param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
  }
  uVar2 = iVar3[0x1].field6_0xc;
  if (iVar3->field16_0x22 < (int)uVar2) {
    iVar3->field16_0x22 = uVar2;
  }
  uVar2 = iVar3[0x1].field7_0xe;
  if (iVar3->field17_0x24 < (int)uVar2) {
    iVar3->field17_0x24 = uVar2;
  }
  iVar4 = (iVar3 + 0x1)->field0_0x0 + iVar3[0x1].field2_0x4;
  if (iVar3->field16_0x22 < iVar4) {
    iVar3->field16_0x22 = iVar4;
  }
  iVar4 = iVar3[0x1].field1_0x2 + iVar3[0x1].field3_0x6;
  if (iVar3->field17_0x24 < iVar4) {
    iVar3->field17_0x24 = iVar4;
  }
  piVar1 = &iVar3->field16_0x22;
  *piVar1 = *piVar1 + iVar3[0x1].field8_0x10;
  piVar1 = &iVar3->field17_0x24;
  *piVar1 = *piVar1 + iVar3[0x1].field8_0x10;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn create_window_1040_92dc(param_1: *mut astruct_65)

{
  HWND16 hwnd;
  astruct_65 *pstruct65_2;
  astruct_65 *pstruct65_1;
  i32 lVar1;

  pstruct65_1 = (astruct_65 *)((u32)param_1 >> 0x10);
  pstruct65_2 = (astruct_65 *)param_1;
  if (pstruct65_2->field11_0x18 == 0x0) {
    hwnd = CreateWindow16(0x0,(void *)CONCAT22(pstruct65_2->field13_0x1c,HINSTANCE16_1050_038c),
                          pstruct65_2->field12_0x1a,0x0,0x0,pstruct65_2->field15_0x20,pstruct65_2->field14_0x1e,0xb,
                          0x4000,s__1050_5e3e,s_button_1050_5e3f);
    pstruct65_2->field11_0x18 = hwnd;
    lVar1 = SetWindowLong16(_u16_1050_5e18,-0x4,hwnd);
    lVar1 = (HANDLE16)((u32)lVar1 >> 0x10);
    &pstruct65_2->field10_0x14 = (int)lVar1;
    *(HANDLE16 *)((int)&pstruct65_2->field10_0x14 + 0x2) = lVar1;
    SetProp16(lVar1,s_procHi_1050_5e46,pstruct65_2->field11_0x18);
    SetProp16(*(HANDLE16 *)&pstruct65_2->field10_0x14,s_procLo_1050_5e4d,pstruct65_2->field11_0x18);
    SetProp16((HANDLE16)pstruct65_1,s_thisHi_1050_5e54,pstruct65_2->field11_0x18);
    SetProp16((HANDLE16)pstruct65_2,s_thisLo_1050_5e5b,pstruct65_2->field11_0x18);
    if (pstruct65_2[0x1].field12_0x1a != 0x0) {
      SetProp16(0x1,s_IsDlg_1050_5e62,pstruct65_2->field11_0x18);
    }
    ShowWindow16(0x5,pstruct65_2->field11_0x18);
  }
  return;
}
pub fn mov_update_win_1040_93aa(param_1: *mut astruct_65,INT16 param_2,mut param_3: u16 )

{
  astruct_65 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_65 *)param_1;
  iVar1->field14_0x1e = param_3;
  iVar1->field15_0x20 = param_2;
  MoveWindow16(0x1,iVar1->field17_0x24,iVar1->field16_0x22,param_2,iVar1->field14_0x1e,iVar1->field11_0x18);
  UpdateWindow16(iVar1->field11_0x18);
  return;
}



LRESULT pass1_1040_93e6(mut param_1: u32)

{
  let mut uVar1: u16;
  LRESULT LVar2;

  uVar1 = (param_1 >> 0x10);
  LVar2 = SendMessage16(0x0,*(WPARAM16 *)((int)param_1 + 0x1c),0x111,*(HWND16 *)((int)param_1 + 0x1a));
  return LVar2;
}



LRESULT send_msg_1040_9404(mut param_1: u32)

{
  let mut uVar1: u16;
  LRESULT LVar2;

  uVar1 = (param_1 >> 0x10);
  LVar2 = SendMessage16(0x0,*(WPARAM16 *)((int)param_1 + 0x1c),0x111,*(HWND16 *)((int)param_1 + 0x1a));
  return LVar2;
}
pub fn pass1_1040_9422(u32 *param_1)

{
  code **ppcVar1;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x8) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x10);
    (**ppcVar1)();
  }
  if (*(i32 *)((int)param_1 + 0x4) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x14);
    (**ppcVar1)();
  }
  return;
}
pub fn unk_draw_op_1040_9458(param_1: *mut astruct_17,param_2: u8,mut param_3: u16 )

{
  code **ppcVar1;
  let mut hpal: *mut u16;
  HPALETTE16 obj;
  let mut uVar3: u16;
  astruct_17 *iVar4;
  let mut uVar4: u16;
  let mut puStack8: *mut u16;
  u32 *puStack6;
  let mut UVar2: u32;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_17 *)param_1;
  if (iVar4->field8_0x8 != NULL) {
    puStack6 = iVar4->field8_0x8;
    if (((((int)&iVar4->field9_0xc + 0x2) | &iVar4->field9_0xc) != 0x0) && ((param_2 & 0x1) != 0x0)) {
      puStack6 = iVar4->field9_0xc;
    }
    if ((iVar4->field10_0x10 != NULL) && ((param_2 & 0x4) != 0x0)) {
      puStack6 = iVar4->field10_0x10;
    }
    hpal = &param_3;
    UVar2 = *puStack6;
    ppcVar1 = (code **)((int)UVar2 + 0x8);
    (**ppcVar1)();
    ppcVar1 = (code **)((int)UVar2 + 0x4);
    (**ppcVar1)();
    obj = SelectPalette16(0x0,(HPALETTE16)hpal,param_3);
    DeleteObject16(obj);
  }
  return;
}
pub fn draw_text_1040_94fc(astruct_37 *struct_param_1,HDC16 hdc_param_2)

{
  astruct_37 *struct_1;
  let mut struct_1_lo: u16;
  COLORREF colorref_2;
  let mut u16_var_1: u16;
  let mut u16_var3: u16;

  struct_1_lo = ((u32)struct_param_1 >> 0x10);
  struct_1 = (astruct_37 *)struct_param_1;
  colorref_2 = SetBkColor16(CONCAT22(0x100,struct_1->field49_0x3a),hdc_param_2);
  SetTextColor16(CONCAT22(0x100,struct_1->field50_0x3c),hdc_param_2);
  DrawText16(0x10,(RECT16 *)((u32)struct_param_1 & 0xffff0000 | ZEXT24(&struct_1->field40_0x2e)),-0x1,
             struct_1->field1_0x4,hdc_param_2);
  u16_var_1 = ((colorref_2 & 0xffff0000) >> 0x10);
  u16_var3 = hdc_param_2;
  SetBkColor16(colorref_2 & 0xffff0000 | (u32)hdc_param_2,hdc_param_2);
  SetTextColor16(CONCAT22(u16_var_1,u16_var3),hdc_param_2);
  return;
}
pub fn win_ui_get_prop_op_1040_9566(i16 *param_1)

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  code **ppcVar3;
  HANDLE16 HVar4;
  HANDLE16 HVar5;
  let mut iVar6: i16;
  let mut uVar7: u16;
  u32 *puStack12;

  uVar7 = ((u32)param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*param_1 == 0x4) {
    uVar1 = (iVar6 + 0xc);
    HVar4 = GetProp16(s_thisHi_1050_5e6f,*(HWND16 *)(iVar6 + 0xa));
    HVar5 = GetProp16(s_thisLo_1050_5e68,*(HWND16 *)(iVar6 + 0xa));
    if ((HVar4 | HVar5) != 0x0) {
      iVar2 = (iVar6 + 0x6);
      if (iVar2 == 0x1) {
        puStack12 = (u32 *)CONCAT22(HVar4,uVar1);
        ppcVar3 = (code **)((int)*puStack12 + 0xc);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,uVar1,HVar4,(iVar6 + 0x8));
        return;
      }
      if (iVar2 == 0x2) {
        puStack12 = (u32 *)CONCAT22(HVar4,uVar1);
        ppcVar3 = (code **)((int)*puStack12 + 0x10);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,uVar1,HVar4,(iVar6 + 0x8));
        return;
      }
      if (iVar2 == 0x4) {
        puStack12 = (u32 *)CONCAT22(HVar4,uVar1);
        ppcVar3 = (code **)((int)*puStack12 + 0x18);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,uVar1,HVar4,*(iVar6 + 0x8) & 0x10);
        return;
      }
    }
  }
  return;
}
pub fn pass1_1040_9618(param_1: *mut astruct_65)

{
  let mut uVar1: u16;
  astruct_65 *iVar2;
  let mut uVar2: u16;
  let mut uVar3: u32;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_65 *)param_1;
  uVar3 = pass1_1008_4772(*(astruct_76 **)&iVar2->field4_0x8);
  uVar1 = (uVar3 >> 0x10);
  iVar2[0x1].field2_0x4 = ((int)uVar3 + 0x4);
  iVar2[0x1].field3_0x6 = ((int)uVar3 + 0x8);
  return;
}
pub fn draw_text_1040_9650(param_1: *mut astruct_65)

{
  HDC16 hdc;

  hdc = GetDC16(0x0);
  DrawText16(0x410,(RECT16 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x2e)),-0x1,
             *(LPCSTR *)((int)param_1 + 0x4),hdc);
  ReleaseDC16(hdc,0x0);
  return;
}
pub fn call_win_proc_1040_9684(HWND16 win_handle_1,mut param_2: u16 ,WPARAM16 w_param_1,LPARAM l_param_1)

{
  HANDLE16 handle_1;
  HANDLE16 handle_2;
  let mut bool_1: bool;
  let mut uVar2: u16;
  RECT16 local_1a [0x2];
  u32 *var18;
  u32 *var14;
  u32 *var10;
  i32 var6;
  let mut var2: u32;
  let mut var4: u16;
  let mut var11: u16;
  let mut var7: u16;
  let mut var8: u16;
  let mut var9: u16;
  let mut uVar1: u16;
  code **fn_ptr_1;

  handle_1 = GetProp16(s_procHi_1050_5e7d,l_param_1);
  handle_2 = GetProp16(s_procLo_1050_5e76,l_param_1);
  var6 = CONCAT22(handle_1,handle_2);
  handle_1 = GetProp16(s_thisHi_1050_5e8b,l_param_1);
  handle_2 = GetProp16(s_thisLo_1050_5e84,l_param_1);
  var10 = (u32 *)CONCAT22(handle_1,handle_2);
  if ((handle_1 | handle_2) != 0x0) {
    if (l_param_1 == 0x2) {
      var18 = var10;
      var14 = var10;
      if (var10 != NULL) {
        fn_ptr_1 = (code **)*var10;
        (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,handle_2,handle_1,0x1);
      }
    }
    else if (l_param_1 == 0x201) {
      handle_1 = GetProp16(s_IsDlg_1050_5e92,l_param_1);
      if (handle_1 == 0x0) {
        uVar2 = ((int)var10 + 0x18);
        GetClientRect16(local_1a,(HWND16)&DAT_1050_1050);
        bool_1 = PtInRect16((POINT16)CONCAT22(param_2,win_handle_1),local_1a);
        if (bool_1 == 0x0) {
          return;
        }
        debug_print_1008_6048(uVar1,s_button__08lx_ldown_1050_5e98);
        fn_ptr_1 = (code **)((int)*var10 + 0x1c);
        (**fn_ptr_1)(0x1008,(char)var10,(int)((u32)var10 >> 0x10),param_2,win_handle_1,w_param_1,uVar2);
        return;
      }
    }
    else if (l_param_1 == 0x204) {
      uVar2 = (handle_2 + 0x18);
      var4 = uVar1;
      GetClientRect16(local_1a,(HWND16)&DAT_1050_1050);
      bool_1 = PtInRect16((POINT16)CONCAT22(param_2,win_handle_1),local_1a);
      if (bool_1 == 0x0) {
        return;
      }
      debug_print_1008_6048(var4,s_button__08lx_rdown_1050_5eab);
      fn_ptr_1 = (code **)((int)*var10 + 0x20);
      (**fn_ptr_1)(0x8,(int)var10,(char)((u32)var10 >> 0x10),param_2,win_handle_1,w_param_1,uVar2);
      return;
    }
  }
  if (var6 != 0x0) {
    CallWindowProc16(CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,win_handle_1)),w_param_1,l_param_1,
                     l_param_1,(LPVOID)var6);
  }
  return;
}



StructD * pass1_1040_97da(StructD *param_1,param_2: u8)

{
  mix_win_ui_op_1040_911e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1040_9824(u32 *param_1)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x0;
  (iVar1 + 0x4) = 0x0;
  (u32)(iVar1 + 0x56) = 0x0;
  (iVar1 + 0x5a) = 0x0;
  (iVar1 + 0x5c) = 0x0;
  *(iVar1 + 0x6) = 0x0;
  return;
}



u16 * unk_win_ui_op_1040_9854(param_1: *mut astruct_787)

{
  HCURSOR16 HVar1;
  HGDIOBJ16 HVar2;
  astruct_787 *struct_1;
  let mut uVar3: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  struct_1 = (astruct_787 *)param_1;
  param_1->offset = 0x389a;
  struct_1->base = 0x1008;
  param_1->offset = 0xa230;
  struct_1->base = &PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&struct_1->field_0x4)),s_OPButton_1050_5ece);
  struct_1->field82_0x54 = 0x3;
  HVar1 = LoadCursor16((char *)0x7f00,0x0);
  struct_1->field84_0x58 = HVar1;
  HVar2 = GetStockObject16(BLACK_BRUSH);
  struct_1->field83_0x56 = HVar2;
  reg_class_1040_98c0((u32)param_1 & 0xffff | (u32)uVar3 << 0x10);
  return &param_1->offset;
}
pub fn reg_class_1040_98c0(mut param_1: u32)

{
  let mut BVar1: bool;
  ATOM AVar2;
  WNDCLASS16 wndclass;

  wndclass.lpsz_class_name._0_2_ = (int)param_1 + 0x4;
  BVar1 = GetClassInfo16(&wndclass,(char *)CONCAT22((int)wndclass.lpsz_class_name,0x1050),param_1);
  if (BVar1 == 0x0) {
    wndclass.style = ((int)param_1 + 0x54);
    wndclass.lpfn_wnd_proc._0_2_ = 0x9cde;
    wndclass.lpfn_wnd_proc = SUB42(&PTR_LOOP_1050_1040,0x0);
    wndclass._6_4_ = 0x40000;
    wndclass.h_instance = HINSTANCE16_1050_038c;
    wndclass.h_icon = 0x0;
    wndclass.h_cursor = *(HCURSOR16 *)((int)param_1 + 0x58);
    wndclass.hbr_background = *(HBRUSH16 *)((int)param_1 + 0x56);
    wndclass.lpsz_menu_name = 0x0;
    wndclass.lpsz_class_name = param_1;
    AVar2 = RegisterClass16(&wndclass);
    if (AVar2 == 0x0) {
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
pub fn draw_op_1040_9948(mut param_1: u16 ,param_2: *mut astruct_71)

{
  HDC16 hdc16_dev_ctx_1;
  let mut mode: i16;
  let mut uVar3: u16;
  HPEN16 handle;
  HPEN16 hgdiobj16_00;
  HGDIOBJ16 hgdiobj_2;
  HGDIOBJ16 hdc_lt_gray_brush_1;
  let mut cch_1: u16;
  let mut puVar1: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  astruct_71 *struct71_var4;
  let mut uVar7: u16;
  char *pcVar1;
  let mut uVar2: u16;
  HDC16 HVar3;
  let mut iStack88: i16;
  let mut x1: i16;
  let mut y2: i16;
  let mut x2: i16;
  let mut y1: i16;
  let mut iStack78: i16;
  u8 paintstruct_42 [0x20];
  let mut uStack34: u16;
  let mut uStack32: u16;
  HGDIOBJ16 hgdiobj_1;
  let mut iStack28: i16;
  let mut x4: i16;
  let mut y6: i16;
  let mut x5: i16;
  let mut y4: i16;
  RECT16 rect_12;
  let mut x3: i16;
  let mut y3: i16;
  let mut rect_a: i16;
  let mut iStack8: i16;
  let mut x6: i16;
  let mut y7: i16;
  let mut iVar1: i16;
  astruct_782 *iVar2;
  u8 uVar8;
  u8 uVar9;
  let mut uVar14: u16;
  u8 uVar10;
  u8 uVar11;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar4_00: u16;

  x6 = 0x1;
  y7 = 0x1;
  rect_a = 0x0;
  iStack8 = 0x0;
  iStack28 = 0x0;
  hgdiobj_1 = 0x0;
  uVar7 = ((u32)param_2 >> 0x10);
  struct71_var4 = (astruct_71 *)param_2;
  uStack32 = struct71_var4->field4_0x4 & 0x8;
  uStack34 = struct71_var4->field86_0x56 & 0x1;
  hdc16_dev_ctx_1 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_42),param_1);
  mode = SetMapMode16(0x1,hdc16_dev_ctx_1);
  GetClientRect16(&rect_12,(HWND16)&DAT_1050_1050);
  iVar2 = (astruct_782 *)((u32)_x3 >> 0x10);
  _x3 = CONCAT22(iVar2 + -0x1,x3 + -0x1);
  if (uStack34 != 0x0) {
    x4 = (i16)rect_12;
    y6 = (i16)((u32)rect_12 >> 0x10);
    rect_12 = CONCAT22(y6 + 0x2,x4 + 0x2);
    _x3 = CONCAT22(iVar2 + -0x3,x3 + -0x3);
    x5 = x3 + -0x1;
    y4 = (i16)(iVar2 + -0x1);
  }
  if (struct71_var4->field6_0x6 != '\0') {
    iStack28 = 0x1;
    if (struct71_var4->hgdiobj_field90_0x5a != 0x0) {
      hgdiobj_1 = SelectObject16(struct71_var4->hgdiobj_field90_0x5a,hdc16_dev_ctx_1);
    }
    pcVar1 = &struct71_var4->field6_0x6;
    uVar2 = uVar7;
    HVar3 = hdc16_dev_ctx_1;
    uVar3 = str_op_1000_3da4((char *)((u32)param_2 & 0xffff0000 | ZEXT24(pcVar1)));
    DrawText16(0x400,(RECT16 *)CONCAT22(0x1050,&rect_a),uVar3,(LPCSTR)CONCAT22(uVar2,pcVar1),HVar3);
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
  if (uStack34 != 0x0) {
    iStack78 = 0x0;
    do {
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
    } while (iStack78 < 0x1);
  }
  if ((struct71_var4->field4_0x4 & 0x2) == 0x0) {
    y2 = (i16)((u32)rect_12 >> 0x10);
    x2 = (i16)_x3;
    iStack78 = 0x0;
    x1 = rect_12.x;
    y1 = y3;
    do {
      SelectObject16(handle,hdc16_dev_ctx_1);
      MoveTo16(y1,x1,hdc16_dev_ctx_1);
      LineTo16(y1,x2,hdc16_dev_ctx_1);
      LineTo16(y2,x2,hdc16_dev_ctx_1);
      do {
        SelectObject16(hgdiobj16_00,hdc16_dev_ctx_1);
        LineTo16(y2,x1,hdc16_dev_ctx_1);
        LineTo16(y1,x1,hdc16_dev_ctx_1);
      } while ((int)(hdc16_dev_ctx_1 + 0x1) < 0x2);
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
    if (iStack28 != 0x0) {
      iStack8 += 0x2;
      rect_a += 0x2;
      x6 += 0x2;
      y7 += 0x2;
    }
  }
  MoveTo16(0x0,0x0,hdc16_dev_ctx_1);
  if (iStack28 != 0x0) {
    if ((struct71_var4->field4_0x4 & 0x4) == 0x0) {
      uVar4 = u16_1050_5ec8;
      puVar1 = u16_1050_5ec6;
      if (uStack32 != 0x0) {
        uVar4 = u16_1050_5ecc;
        puVar1 = u16_1050_5eca;
      }
      SetTextColor16(CONCAT22(uVar4,puVar1),hdc16_dev_ctx_1);
      SetBkColor16(0x1000000,hdc16_dev_ctx_1);
      pcVar1 = &struct71_var4->field6_0x6;
      HVar3 = hdc16_dev_ctx_1;
      uVar6 = str_op_1000_3da4((char *)((u32)param_2 & 0xffff0000 | ZEXT24(pcVar1)));
      DrawText16(0x1,(RECT16 *)CONCAT22(0x1050,&rect_a),uVar6,(LPCSTR)CONCAT22(uVar7,pcVar1),HVar3);
      uVar13 = s_tile2_bmp_1050_1538;
      uVar14 = 0x9c8d;
      SetTextColor16(CONCAT22(HVar3,uVar7),hdc16_dev_ctx_1);
      SetBkColor16(CONCAT22(uVar13,uVar14),hdc16_dev_ctx_1);
    }
    else {
      hdc_lt_gray_brush_1 = GetStockObject16(LTGRAY_BRUSH);
      uVar4_00 = 0x0;
      uVar12 = 0x0;
      pcVar1 = &struct71_var4->field6_0x6;
      HVar3 = hdc16_dev_ctx_1;
      cch_1 = str_op_1000_3da4((char *)((u32)param_2 & 0xffff0000 | ZEXT24(pcVar1)));
      GrayString16(y7 - iStack8,x6 - rect_a,iStack8,rect_a,cch_1,CONCAT22(uVar7,pcVar1),
                   (void *)CONCAT22(uVar12,uVar4_00),hdc_lt_gray_brush_1,HVar3);
    }
    if (hgdiobj_1 != 0x0) {
      SelectObject16(hgdiobj_1,hdc16_dev_ctx_1);
    }
  }
  SelectObject16(hgdiobj_2,hdc16_dev_ctx_1);
  SetMapMode16(mode,hdc16_dev_ctx_1);
  DeleteObject16(handle);
  DeleteObject16(hgdiobj16_00);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_42),param_1);
  return;
}
pub fn win_op_1040_9cde(LPARAM lparam_param_1,WPARAM16 wparam_param_2,u16 msg_param_3,HWND16 hwnd_param_4,
                     mut param_5: u16 ,mut param_6: u16 ,mut param_7: u32)

{
  u8 *pbVar1;
  let mut iVar2: i16;
  u8 bVar3;
  WPARAM16 WVar4;
  let mut BVar5: bool;
  let mut uVar9: u32;
  let mut uVar6: u16;
  let mut uVar8: i16;
  let mut uVar10: u16;
  i32 win_long_11;
  WPARAM16 *pWVar11;
  LRESULT LVar12;
  let mut uVar13: u32;
  u8 uVar14;
  u8 uVar15;
  RECT16 rect_a [0x2];
  let mut iVar3: i16;
  astruct_57 *paVar7;
  HWND16 hwnd_dlg_8;

  uVar10 = ((u32)param_7 >> 0x10);
  win_long_11 = GetWindowLong16(0x0,hwnd_param_4);
  paVar7 = (astruct_57 *)CONCAT22(uVar10,(int)((u32)win_long_11 >> 0x10));
  iVar3 = (i16)win_long_11;
  uVar8 = (i16)((win_long_11 & 0xffff0000U) >> 0x10);
  if (msg_param_3 == 0x30) {
    *(WPARAM16 *)(iVar3 + 0x5a) = wparam_param_2;
  }
  else {
    if (msg_param_3 < 0x31) {
      if (msg_param_3 == 0x1f) {
        (iVar3 + 0x4) = 0x0;
        ReleaseCapture16();
        return;
      }
      if (0x1f < msg_param_3) goto LAB_1040_a1ae;
      bVar3 = (u8)msg_param_3;
      if (bVar3 == 0x8) {
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xf7;
        uVar6 = 0x0;
        BVar5 = IsWindow16(wparam_param_2);
        if (BVar5 != 0x0) {
          uVar13 = SendMessage16(0x0,0x0,0x87,wparam_param_2);
          uVar6 = ((uVar13 & 0x20) == 0x0);
        }
        (u32)(iVar3 + 0x56) = 0x0;
        SendMessage16(0x0,*(WPARAM16 *)(iVar3 + 0x5c),0x401,*(HWND16 *)(iVar3 + 0x2));
        if (((iVar3 + 0x5c) != 0x0) && ((iVar3 + 0x5c) != win_long_11)) {
          SendDlgItemMessage16((u32)uVar6,0x1,0x404,*(INT16 *)(iVar3 + 0x5c),*(HWND16 *)(iVar3 + 0x2));
        }
        (iVar3 + 0x5c) = 0x0;
      }
      else if (bVar3 < 0x9) {
        if (bVar3 == 0x1) {
          pWVar11 = (WPARAM16 *)GetWindowLong16(0x0,hwnd_param_4);
          iVar2 = (int)pWVar11;
          uVar10 = (((u32)pWVar11 & 0xffff0000) >> 0x10);
          (iVar2 + 0x2) = ((int)lparam_param_1 + 0x8);
          WVar4 = GetDlgCtrlID16(hwnd_param_4);
          *pWVar11 = WVar4;
          (u32)(iVar2 + 0x56) = (u32)((int)lparam_param_1 + 0x12);
          unk_str_op_1000_3d3e
                    ((char *)((u32)pWVar11 & 0xffff0000 | (u32)(iVar2 + 0x6)),*(char **)((int)lparam_param_1 + 0x16)
                    );
          if ((*((int)lparam_param_1 + 0x12) & 0x1) != 0x0) {
            SendMessage16(0x0,*pWVar11,0x401,*(HWND16 *)((int)lparam_param_1 + 0x8));
          }
          if ((((int)lparam_param_1 + 0x14) & 0x800) == 0x0) {
            return;
          }
          pbVar1 = (iVar2 + 0x4);
          *pbVar1 = *pbVar1 | 0x4;
          return;
        }
        if (bVar3 == 0x2) {
          fn_ptr_1000_17ce((char *)win_long_11);
          SetWindowLong16(0x0,0x0,hwnd_param_4);
          return;
        }
        if (bVar3 != 0x7) goto LAB_1040_a1ae;
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 | 0x8;
        LVar12 = SendMessage16(0x0,0x0,0x400,*(HWND16 *)(iVar3 + 0x2));
        iVar2 = (int)LVar12;
        if (((int)((u32)LVar12 >> 0x10) == 0x534b) && ((iVar3 + 0x5c) = iVar2, iVar2 != win_long_11))
        {
          SendDlgItemMessage16(0x1,0x0,0x404,iVar2,*(HWND16 *)(iVar3 + 0x2));
        }
        SendMessage16(0x0,*(WPARAM16 *)win_long_11,0x401,*(HWND16 *)(iVar3 + 0x2));
        (u32)(iVar3 + 0x56) = 0x1;
      }
      else if (bVar3 == 0xa) {
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xfb;
        if (wparam_param_2 == 0x0) {
          pbVar1 = (iVar3 + 0x4);
          *pbVar1 = *pbVar1 | 0x4;
        }
      }
      else {
        if (bVar3 != 0xc) {
          if (bVar3 == 0xf) {
            draw_op_1040_9948(hwnd_param_4,(astruct_71 *)win_long_11);
            return;
          }
          goto LAB_1040_a1ae;
        }
        if (lparam_param_1 != 0x0) {
          unk_str_op_1000_3d3e((char *)(win_long_11 & 0xffff0000U | (u32)(iVar3 + 0x6)),(char *)lparam_param_1);
        }
      }
      goto LAB_1040_9e20;
    }
    if (msg_param_3 == 0x200) {
      if ((*(iVar3 + 0x4) & 0x1) == 0x0) {
        return;
      }
      GetClientRect16(rect_a,(HWND16)&DAT_1050_1050);
      iVar2 = (iVar3 + 0x4);
      BVar5 = PtInRect16((POINT16)lparam_param_1,rect_a);
      if (BVar5 == 0x0) {
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xfd;
      }
      else {
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 | 0x2;
      }
      lparam_param_1._0_2_ = (iVar3 + 0x4) - iVar2;
    }
    else {
      if (msg_param_3 < 0x201) {
        uVar9._0_2_ = msg_param_3 - 0x81;
        if (uVar9 == 0x0) {
          uVar14 = 0x0;
          uVar15 = 0x0;
          mem_op_1000_179c(0x5e,paVar7);
          uVar6 = paVar7 | uVar9;
          if (uVar6 == 0x0) {
            uVar9._0_2_ = 0x0;
            uVar6 = 0x0;
          }
          else {
            pass1_1040_9824((u32 *)CONCAT22(paVar7,uVar9));
          }
          SetWindowLong16(CONCAT22(uVar6,uVar9),CONCAT11(uVar15,uVar14),hwnd_param_4);
          return;
        }
        if (msg_param_3 == 0x87) {
          return;
        }
        if (msg_param_3 == 0x100) {
          if ((wparam_param_2 == 0x26) || (wparam_param_2 == 0x25)) {
            hwnd_dlg_8 = *(HWND16 *)(iVar3 + 0x2);
            BVar5 = 0x1;
          }
          else {
            if ((wparam_param_2 != 0x28) && (wparam_param_2 != 0x27)) {
              if (((wparam_param_2 == 0x20) || (wparam_param_2 == 0xd)) && (PTR_LOOP_1050_5ed8 == NULL)) {
                PTR_LOOP_1050_5ed8 = ((int)&PTR_LOOP_1050_0000 + 0x1);
                pbVar1 = (iVar3 + 0x4);
                *pbVar1 = *pbVar1 | 0x2;
                goto LAB_1040_9e20;
              }
              goto LAB_1040_a1ae;
            }
            hwnd_dlg_8 = *(HWND16 *)(iVar3 + 0x2);
            BVar5 = 0x0;
          }
          hwnd_dlg_8 = GetNextDlgTabItem16(BVar5,hwnd_param_4,hwnd_dlg_8);
          SetFocus16(hwnd_dlg_8);
          return;
        }
        if ((msg_param_3 == 0x101) && (PTR_LOOP_1050_5ed8 != NULL)) {
          PTR_LOOP_1050_5ed8 = NULL;
          pbVar1 = (iVar3 + 0x4);
          *pbVar1 = *pbVar1 & 0xfd;
          InvalidateRect16(0x1,NULL,0x0);
          UpdateWindow16(hwnd_param_4);
          SendMessage16(0x0,*(WPARAM16 *)win_long_11,0x111,*(HWND16 *)(iVar3 + 0x2));
          return;
        }//
LAB_1040_a1ae:
        DefWindowProc16(lparam_param_1,wparam_param_2,msg_param_3,hwnd_param_4);
        return;
      }
      if (msg_param_3 == 0x201) {//
LAB_1040_9e74:
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
        GetClientRect16(rect_a,(HWND16)&DAT_1050_1050);
        if ((*(iVar3 + 0x4) & 0x1) == 0x0) {
          return;
        }
        pbVar1 = (iVar3 + 0x4);
        *pbVar1 = *pbVar1 & 0xfc;
        InvalidateRect16(0x1,NULL,0x0);
        UpdateWindow16(hwnd_param_4);
        BVar5 = PtInRect16((POINT16)lparam_param_1,rect_a);
        if (BVar5 == 0x0) {
          return;
        }
        PostMessage16(0x0,*(WPARAM16 *)win_long_11,0x111,*(HWND16 *)(iVar3 + 0x2));
        return;
      }
      if (msg_param_3 == 0x203) goto LAB_1040_9e74;
      if (msg_param_3 != 0x404) goto LAB_1040_a1ae;
      if (wparam_param_2 == 0x1) {
        (u32)(iVar3 + 0x56) = 0x1;
      }
      else {
        (u32)(iVar3 + 0x56) = 0x0;
      }
    }
  }
  if ((int)lparam_param_1 == 0x0) {
    return;
  }//
LAB_1040_9e20:
  InvalidateRect16(0x1,NULL,0x0);
  UpdateWindow16(hwnd_param_4);
  return;
}

