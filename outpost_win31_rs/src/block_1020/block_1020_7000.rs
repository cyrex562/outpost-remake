
pub unsafe fn draw_op_1020_7070(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,hdc_param_5: HDC16) -> u16

{
  let mut hgdi_obj: u16;

  GetStockObject16(BLACK_BRUSH);
  if (COLORREF_1050_441e == 0) {
    COLORREF_1050_441e = 0x1000002;
  }
  if (0x6 < param_4) {
    return 0x0;
  }
  SetTextColor16(COLORREF_1050_441e,hdc_param_5);
  hgdi_obj = 0x100;
  SetBkColor16(0x1000000,hdc_param_5);
  return hgdi_obj;
}



pub fn pass1_1020_70c0(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  cleanup_menu_ui_op_1020_795c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_ui_op_1020_717e(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_40,param_4: *mut StructA)

{
  paVar1: *mut astruct_13;
  let mut ppcVar2: *mut *mut code;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut HVar6: HPALETTE16;
  let mut puVar6: *mut u32;
  let mut uVar6: u16;
  let mut uVar9: u16;
  let mut puVar7: *mut u8;
  let mut puVar10: *mut u8;
  let mut in_register_0000000a: u16;
  let mut paVar11: *mut Struct57;
  iVar7: *mut astruct_40;
  iVar8: *mut astruct_933;
  uVar7: *mut astruct_40;
  let mut uVar8: u16;
  let mut puVar12: *mut u32;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000fe84: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa6: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb2: u16;
  local_4: HDC16;
  let mut uVar3: u32;
  iVar9: *mut astruct_41;
  let mut in_stack_0000ffdc: u16;

  paVar11 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1020_7c1a(param_3,param_4);
  uVar7 = (param_3 >> 0x10);
  iVar7 = param_3;
  iVar7.field17_0x14 = 0;
  iVar7.field20_0x18 = param_4;
  iVar7.field21_0x1c = 0;
  (&iVar7[0x1].field0_0x0 + 1) = 0;
  param_3.field0_0x0 = 0x754c;
  iVar7.field1_0x2 = 0x1020;
  puVar12 = mixed_1010_20ba(paVar11,_u16_1050_0ed0,CONCAT22(param_2,0x4),in_stack_0000fe84,
                            in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
  uVar5 = (paVar11 >> 0x10);
  iVar7.field21_0x1c = puVar12;
  uVar9 = (puVar12 >> 0x10);
  iVar7.field_0x1e = uVar9;
  ppcVar2 = (*&iVar7.field21_0x1c + 0x4);
  (**ppcVar2)(0x1010,iVar7.field21_0x1c,uVar9,0x0,param_3);
  local_4 = GetDC16(iVar7.hwnd_0x4);
  uVar3 = &iVar7.field21_0x1c;
  *(uVar3 + 0x178) = local_4;
  uVar4 = &iVar7.field21_0x1c;
  uVar8 = (uVar4 >> 0x10);
  iVar8 = uVar4;
  puVar6 = iVar8.field36_0x24;
  uVar9 = (&iVar8.field36_0x24 + 2);
  paVar11 = CONCAT22(uVar5,uVar9);
  uVar5 = SUB42(puVar6,0x0);
  ppcVar2 = (*puVar6 + 0x14);
  (**ppcVar2)(0x38,uVar5,uVar9);
  puVar12 = mixed_1010_20ba(paVar11,_u16_1050_0ed0,CONCAT22(uVar5,0x29),in_stack_0000fe78,
                            in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
  puVar10 = (puVar12 >> 0x10);
  paVar1 = (puVar12 + 0xe);
  pass1_1008_4d84(puVar10,(puVar6 & 0xffff | paVar11 << 0x10),paVar1);
  HVar6 = palette_op_1008_4e08(&local_4,puVar10,paVar1,CONCAT13(0x10,CONCAT12(0x50,&local_4)));
  (&iVar7[0x1].field0_0x0 + 1) = HVar6;
  return;
}



// WARNING: Unable to use type for symbol uVar3
pub fn palette_op_1020_7270(pstruct_param_1: *mut StructD)

{
  let mut obj: HPALETTE16;
  let mut struct767_var1: *mut StructD;
  let mut uVar4: u16;
  let mut unaff_SS: u16;
  let mut pcStack8: *mut c_char;
  let mut uVar3: u32;
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar4 = (pstruct_param_1 >> 0x10);
  struct767_var1 = pstruct_param_1;
  pstruct_param_1.address_offset_field_0x0 = 0x754c;
  struct767_var1.address_offset_field_0x2 = 0x1020;
  if (&struct767_var1.field_0x1c != 0) {
    pass1_1010_1ea6(&struct767_var1.field_0x1c,
                    (pstruct_param_1 & 0xffff | uVar4 << 0x10));
  }
  uVar1 = &struct767_var1.field12_0x14;
  uVar2 = (&struct767_var1.field12_0x14 + 2);
  pcStack8 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0) {
    pass1_1008_5118(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack8);
  }
  uVar3 = &struct767_var1.field_0x1c;
  obj = SelectPalette16(0x0,struct767_var1.field19_0x20,*(uVar3 + 0x178));
  struct767_var1.field19_0x20 = obj;
  DeleteObject16(obj);
  pstruct_param_1.address_offset_field_0x0 = 0x3ab0;
  struct767_var1.address_offset_field_0x2 = 0x1008;
  pstruct_param_1.address_offset_field_0x0 = 0x389a;
  struct767_var1.address_offset_field_0x2 = 0x1008;
  return;
}
pub fn post_win_msg_1020_7308(mut param_1: u32,mut param_2: u16 )

{
  let mut cVar1: u8;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (param_2 != 0x12) {
    if (param_2 < 0x13) {
      cVar1 = param_2;
      if (cVar1 == '\x01') {
        (param_1 + 0x1c) = 0;
        return;
      }
      if (('\x03' < (cVar1 + -1)) && ((cVar1 + -0x5) < '\x02')) goto LAB_1020_7310;
    }
    return;
  }//
// LAB_1020_7310:
  PostMessage16(0x0,0xeb,0x111,(param_1 + 0x4));
  invalidate_rect_1020_735a(param_1);
  return;
}
pub fn invalidate_rect_1020_735a(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0x1c);
  InvalidateRect16(0x0,(uVar1 + 0x16c),(uVar1 >> 0x10));
  return;
}



// WARNING: Unable to use type for symbol uVar3
// WARNING: Unable to use type for symbol uVar4

pub fn win_ui_op_1020_737a(mut param_1: u16 ,param_2: *mut astruct_788) -> BOOL16

{
  let mut uVar8: u16;
  let mut is_iconic: bool;
  let mut puVar3: *mut c_char;
  let mut RVar9: RECT16;
  let mut in_DX: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut uVar10: u32;
  struct_1: *mut astruct_788;
  let mut uVar6: u16;
  let mut local_42: [u8;0x6] = [0;0x6];
  let mut local_brush_handle: RECT16;
  let mut iStack56: i16;
  let mut HStack54: HWND16;
  let mut HStack52: HWND16;
  let mut iStack50: i16;
  let mut rect_30: RECT16;
  let mut iStack44: i16;
  let mut iStack42: i16;
  let mut local_rect: HGDIOBJ16;
  let mut hicon_38: bool;
  hdc_24: HDC16;
  let mut local_paint_struct: [u8;0x20] = [0;0x20];
  uVar3: *mut astruct_126;
  let mut uVar1: u16;
  let mut uVar2: u16;
  uVar4: *mut astruct_126;
  let mut uVar7: u8;
  let mut fn_ptr_1: *mut *mut code;

  uVar6 = (param_2 >> 0x10);
  struct_1 = param_2;
  hdc_24 = BeginPaint16(CONCAT13(0x10,CONCAT12(0x50,local_paint_struct)),struct_1.hwnd_0x4);
  is_iconic = IsIconic16(struct_1.hwnd_0x4);
  if (is_iconic == 0) {
    uVar4 = struct_1.field22_0x1c;
    RVar9 = *(uVar4 + 0x24);
    local_brush_handle = RVar9;
    pass1_1018_2e5e(SUB42(RVar9,0x0),in_DX,struct_1.field22_0x1c);
    rect_30 = RVar9 & 0xffff | in_DX << 0x10;
    pass1_1008_3e54(CONCAT13(0x10,CONCAT12(0x50,local_42)),0x0,0x35,0xc);
    if (&struct_1.field19_0x14 != 0) {
      pass1_1008_5236(&struct_1.field19_0x14);
    }
    if (rect_30 != 0) {
      uVar1 = struct_1.field19_0x14;
      uVar8 = struct_1.field20_0x16;
      uVar10 = CONCAT22(in_register_0000000a,uVar8);
      if ((uVar8 | uVar1) != 0) {
        pass1_1008_5118(CONCAT22(uVar8,uVar1));
        fn_ptr_1000_17ce(CONCAT22(uVar8,uVar1));
      }
      puVar3 = local_42;
      pass1_1008_8ce4(rect_30,CONCAT22(0x1050,puVar3),local_brush_handle,uVar10);
      struct_1.field19_0x14 = puVar3;
      struct_1.field20_0x16 = uVar10;
    }
    fn_ptr_1 = (local_brush_handle + 0x4);
    (**fn_ptr_1)(0x1008,SUB42(local_brush_handle,0x0),(local_brush_handle >> 0x10),0x0,0x0,&hdc_24,
                 &DAT_1050_1050);
    fn_ptr_1 = (*struct_1.field21_0x18 + 0x94);
    (**fn_ptr_1)(0x1008,struct_1.field21_0x18,1);
    HStack52 = GetDlgItem16(0x1797,struct_1.hwnd_0x4);
    if (HStack52 != 0) {
      ShowWindow16(0x1,HStack52);
    }
  }
  else if (PTR_LOOP_1050_0010.is_null()) {
    uVar3 = struct_1.field22_0x1c;
    fn_ptr_1 = (struct_1.field22_0x1c + 0x2c);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,uVar3,(uVar3 >> 0x10));
    hicon_38 = is_iconic;
    if (is_iconic != 0) {
      local_rect = GetStockObject16(BLACK_BRUSH);
      GetClientRect16(&rect_30,&DAT_1050_1050);
      local_brush_handle = (RECT16)0x0;
      iStack56 = (iStack44 - rect_30.x) + -0x1;
      HStack54 = (iStack42 - rect_30.y) - 0x1;
      HStack52 = HStack54;
      iStack50 = iStack56;
      FillRect16(local_rect,&local_brush_handle,&DAT_1050_1050);
      DrawIcon16(hicon_38,0x2,0x2,hdc_24);
    }
  }
  is_iconic = EndPaint16(CONCAT22(0x1050,local_paint_struct),struct_1.hwnd_0x4);
  return is_iconic;
}



pub fn pass1_1020_7526(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  palette_op_1020_7270(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1020_7554(mut param_1: u16 ,param_2: *mut astruct_20,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut iVar2: *mut astruct_20;
  let mut unaff_BP: u16;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  unk_draw_op_1020_7f7a(param_2,0x5,CONCAT22(param_4,param_3),param_5);
  uVar3 = (param_2 >> 0x10);
  iVar2 = param_2;
  iVar2[0x1].field5_0xc = 0;
  iVar2[0x1].field7_0x10 = NULL;
  param_2.offset_0x0 = 0x7780;
  iVar2.base_0x2 = 0x1020;
  (iVar2 + 1)->offset_0x0 = 0x781c;
  iVar2[0x1].base_0x2 = 0x1020;
  puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(unaff_BP,0x25),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  uVar1 = (puVar4 >> 0x10);
  iVar2[0x1].field7_0x10 = puVar4;
  (&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
  iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
  (&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
  return;
}
pub fn pass1_1020_75c4(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0x7780;
  iVar1.address_offset_field_0x2 = 0x1020;
  iVar1.field_0xe2 = 0x781c;
  iVar1.field_0xe4 = 0x1020;
  pass1_1020_808e(param_1);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_1020_75f0(param_1: *mut astruct_283,mut param_2: u32)

{
  let mut pUVar1: *mut u16;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar8: u16;
  iVar7: *mut astruct_283;
  let mut uVar9: u16;
  let mut puVar10: *mut u16;
  let mut in_stack_0000ff4c: u16;
  let mut in_stack_0000ff62: u16;
  let mut uStack10: u32;
  let mut local_6: [u8;0x4] = [0;0x4];
  let mut paVar6: *mut Struct57;
  let mut uVar7: u32;

  uVar8 = (param_2 >> 0x10);
  uVar9 = (param_1 >> 0x10);
  iVar7 = param_1;
  if (iVar7.field235_0xee.is_null() == false) {
    ppcVar2 = (*iVar7.field235_0xee + 0x8);
    (**ppcVar2)();
  }
  if (iVar7.field233_0xea == 0) {
    iVar7.field233_0xea = 0x1;
    puVar10 = pass1_1008_941a(CONCAT22(0x1050,local_6),0x1,0x91);
    uVar4 = (puVar10 >> 0x10);
    paVar6 = CONCAT22(uVar8,uVar4);
    uVar3 = ZEXT24(local_6);
    win_1008_5c9e(local_6,uVar4,_u16_1050_02a0,CONCAT22(0x1050,local_6));
    iVar7.field234_0xec = uVar3;
    mem_op_1000_179c(0x112,paVar6);
    uVar5 = paVar6 | uVar3;
    uVar7 = paVar6 & 0xffff0000 | uVar5;
    if (uVar5 == 0) {
      uVar9 = 0;
      uStack10 = NULL;
    }
    else {
      pUVar1 = &iVar7.field204_0xcc;
      *pUVar1 = *pUVar1 + 1;
      struct_1020_3644(uVar7,CONCAT13((paVar6 >> 0x8),CONCAT12(paVar6,uVar3)),
                       iVar7.field204_0xcc,param_1 & 0xffff | uVar9 << 0x10,in_stack_0000ff4c,
                       in_stack_0000ff62);
      uVar9 = uVar3;
      uStack10 = (uVar3 & 0xffff | uVar7 << 0x10);
    }
    pass1_1008_6978(uVar9,uVar7,param_1,0x0,uStack10 & 0xffff0000 | uVar9);
    ppcVar2 = (*uStack10 + 0xc);
    (**ppcVar2)(0x8,uStack10,uStack10,0x5);
  }
  return;
}
pub fn window_op_1020_76aa(param_1: *mut StructA,param_2: *mut astruct_666)

{
  let mut uVar3: u16;
  let mut in_EDX: *mut Struct57;
  let iVar1: *mut StructA;
  let mut uVar2: u16;

  create_window_ex_1008_9760(param_1);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  get_dc_1018_4db0(*(astruct_126 **)&iVar1[0x1].field20_0x26,iVar1.field4_0x8);
  mem_op_1000_179c(0x18,in_EDX);
  uVar3 = in_EDX | param_2;
  if (uVar3 != 0) {
    pass1_1020_7824(uVar3,param_2,in_EDX,iVar1.field4_0x8);
    iVar1[0x1].field18_0x22 = param_2;
    iVar1[0x1].field19_0x24 = uVar3;
    return;
  }
  iVar1[0x1].field18_0x22 = 0;
  return;
}
pub fn pass1_1020_770e(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0xee);
  uVar2 = (iVar4 + 0xf0);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0xee) = 0;
  destroy_win_1008_628e(param_1 & 0xffff | uVar5 << 0x10);
  return;
}



pub fn pass1_1020_774c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1020_75c4(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_7824(mut param_1: u16 ,param_2: *mut astruct_666,mut param_3: u16 ,mut param_4: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut puVar4: *mut u8;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut puVar6: *mut u32;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffca: u32;
  let mut in_stack_0000fff2: u16;

  paVar5 = CONCAT22(in_register_0000000a,param_1);
  set_struct_op_1020_921c(param_1,CONCAT22(param_3,param_2),param_4,in_stack_0000ffca);
  &param_2.field16_0x14 = 0;
  CONCAT22(param_3,param_2) = 0x7902;
  param_2.field2_0x2 = 0x1020;
  puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,CONCAT22(in_stack_0000fff2,0x25),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  uVar3 = (puVar6 >> 0x10);
  param_2.field16_0x14 = puVar6;
  param_2.field17_0x16 = uVar3;
  param_2.field5_0x6 = param_2.field16_0x14;
  param_2.field6_0x8 = uVar3;
  uVar2 = &param_2.field16_0x14;
  puVar4 = &param_2.field_0xa;
  ppcVar1 = ((uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_2.field15_0x12 = puVar4;
  draw_op_1020_9364(CONCAT22(param_3,param_2));
  return;
}
pub fn pass1_1020_78ac(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x7902;
  (iVar1 + 0x2) = 0x1020;
  if ((iVar1 + 0x14) != 0) {
    pass1_1010_1dda((iVar1 + 0x14));
  }
  palette_op_1020_92c4(param_1);
  return;
}



pub fn pass1_1020_78dc(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1020_78ac(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1020_790e(param_1: *mut u16,param_2: *mut c_char,mut param_3: u16 ,mut param_4: u32)

{
  iVar1: *mut astruct_271;
  let mut uVar1: u16;

  unk_draw_op_1008_7f62(param_1,param_3,param_4);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field223_0xe0 = 0;
  iVar1.field224_0xe4 = 0;
  iVar1.field225_0xe8 = 0;
  iVar1.field226_0xec = 0;
  iVar1.field227_0xee = param_2;
  *param_1 = 0x7b86;
  iVar1.field2_0x2 = 0x1020;
  return;
}
pub fn cleanup_menu_ui_op_1020_795c(in_struct_1: *mut StructD)

{
  let mut local_struct_1: *mut StructD;
  let mut uVar1: *mut StructD;

  uVar1 = (in_struct_1 >> 0x10);
  local_struct_1 = in_struct_1;
  in_struct_1.address_offset_field_0x0 = 0x7b86;
  local_struct_1.address_offset_field_0x2 = 0x1020;
  if (local_struct_1.hmenu_0xec != 0) {
    DestroyMenu16(local_struct_1.hmenu_0xec);
  }
  pass1_1008_57c4((in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.field192_0xd2)));
  in_struct_1.address_offset_field_0x0 = 0x380a;
  local_struct_1.address_offset_field_0x2 = 0x1008;
  in_struct_1.address_offset_field_0x0 = 0x389a;
  local_struct_1.address_offset_field_0x2 = 0x1008;
  return;
}



pub unsafe fn pass1_1020_79ae() -> u16

{
  return 0x1;
}
pub fn string_1020_79b4(mut param_1: u32,mut param_2: i16,param_3: *mut c_char)

{
  let mut in_DX: u16;

  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0xa)),param_3);
  if (param_2 != 0) {
    draw_op_1020_7cc8(in_DX,*(StructE **)(param_1 + 0xe8));
  }
  return;
}
pub fn pass1_1020_79e4(mut param_1: u32)

{
  let mut in_DX: u16;

  draw_op_1020_7cc8(in_DX,*(StructE **)(param_1 + 0xe8));
  return;
}
pub fn post_win_msg_1020_79fc(param_1: *mut astruct_69,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  let mut puVar1: *mut u32;
  let mut ppcVar2: *mut *mut code;
  let mut iVar3: i16;
  iVar4: *mut astruct_69;
  let mut uVar4: u16;
  let mut uVar5: u16;

  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar5 = (iVar4.field223_0xe0 >> 0x10);
  ppcVar2 = (*iVar4.field223_0xe0 + 0x24);
  iVar3 = (**ppcVar2)();
  if (iVar3 != param_4) {
    PostMessage16(0x0,0x0,0x85,iVar4.field8_0x8);
    puVar1 = iVar4.field223_0xe0;
    ppcVar2 = (*iVar4.field223_0xe0 + 0x28);
    (**ppcVar2)(s_tile2_bmp_1050_1538,puVar1,(puVar1 >> 0x10),param_4,uVar5);
  }
  return;
}
pub fn get_win_ui_info_op_1020_7a50(param_1: *mut astruct_868)

{
  let mut ppcVar1: *mut *mut code;
  let mut b_var2: bool;
  iVar2: *mut astruct_868;
  let mut var5: u16;

  var5 = (param_1 >> 0x10);
  iVar2 = param_1;
  b_var2 = IsIconic16(iVar2.field8_0x8);
  if (b_var2 == 0) {
    GetWindowRect16(CONCAT22(0x1050,&stack0xfff6),iVar2.field8_0x8);
    GetSystemMetrics16(SM_CXBORDER);
    GetSystemMetrics16(SM_CYBORDER);
  }
  ppcVar1 = (*&iVar2.field_0xe0 + 0x14);
  (**ppcVar1)();
  return;
}
pub fn win_ui_menu_op_1020_7ad2(param_1: *mut astruct_854,param_2: HWND16,param_3: *mut RECT16)

{
//   HMENlet mut HVar1: u16;
let mut HVar1: HMENU16;
iVar2: *mut astruct_854;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2.field236_0xee.is_null() == false) && (iVar2.field235_0xec == 0)) {
    HVar1 = LoadMenu16(iVar2.field236_0xee,HINSTANCE16_1050_038c);
    iVar2.field235_0xec = HVar1;
    if (HVar1 == 0) {
      return;
    }
    HVar1 = GetSubMenu16(0x0,iVar2.field235_0xec);
    iVar2.field235_0xec = HVar1;
    if (HVar1 == 0) {
      return;
    }
  }
  ClientToScreen16(CONCAT22(0x1050,&stack0xfffa),iVar2.field8_0x8);
  HVar1 = iVar2.field235_0xec;
  TrackPopupMenu16(NULL,iVar2.field8_0x8,0x0,HVar1,0x0,0x0,HVar1);
  return;
}



pub fn pass1_1020_7b60(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  cleanup_menu_ui_op_1020_795c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn get_sys_metrics_1020_7c1a(param_1: *mut astruct_40,param_2: *mut StructA)

{
  let mut IVar1: i16;
  iVar3: *mut astruct_40;
  let mut uVar3: u16;
  uVar4: *mut astruct_40;
  let mut uVar1: u16;

  uVar3 = (param_2 >> 0x10);
  uVar1 = (param_2 + 0x8);
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  param_1.field0_0x0 = 0x389a;
  iVar3.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x3aa8;
  iVar3.field1_0x2 = 0x1008;
  iVar3.hwnd_0x4 = uVar1;
  param_1.field0_0x0 = 0x3ab0;
  iVar3.field1_0x2 = 0x1008;
  *(StructA **)&iVar3.field_0x6 = param_2;
  iVar3.field_0xa = 0;
  iVar3.field_0xe = 0;
  iVar3.field_0x10 = 0;
  iVar3.field_0x12 = 0;
  param_1.field0_0x0 = 0x7f72;
  iVar3.field1_0x2 = 0x1020;
  iVar3.field_0xa = (param_2 + 0xe4);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  iVar3.field_0xe = IVar1;
  IVar1 = GetSystemMetrics16(SM_CXBORDER);
  iVar3.field_0x10 = IVar1;
  IVar1 = GetSystemMetrics16(SM_CYBORDER);
  iVar3.field_0x12 = IVar1;
  return;
}



// WARNING: Unable to use type for symbol puVar2
// WARNING: Unable to use type for symbol iVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn draw_op_1020_7cc8(mut param_1: u16 ,StructE *struct_e_param_1)

{
  let mut y_00: i16;
  let mut str46_len: i16;
  let mut x: i16;
  let mut iVar3: i16;
  let mut brush_handle_2: HGDIOBJ16;
  let mut width: i16;
  let mut iVar9: i16;
  let mut puVar6: *mut u32;
  let mut iVar5: i16;
  handle: HPEN16;
  let mut string_1: *mut c_char;
  let mut y: i16;
  let mut extraout_DX: u16;
  let mut iVar6: i16;
  StructE *struct_e_1;
  iVar7: *mut astruct_781;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut DVar1: u32;
  rect: *mut RECT16;
  hdc: HDC16;
  let mut iVar2: i16;
  let mut string_46: *mut c_char;
  let mut local_rect_1: i16;
  let mut iStack18: i16;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut hpalette_12: HPALETTE16;
  paStack10: *mut astruct_13;
  win_hdc_1: HDC16;
  let mut is_iconic: bool;
  let mut puVar2: *mut u32;
  let mut style: i16;
  let mut iVar8: i16;
  let mut iVar1: i16;
  let mut fn_ptr_1: *mut *mut code;

  uVar7 = (struct_e_param_1 >> 0x10);
  struct_e_1 = (StructE *)struct_e_param_1;
  is_iconic = IsIconic16(struct_e_1.hwnd16_field4_0x4);
  if ((is_iconic == 0) || (PTR_LOOP_1050_0010.is_null() == false)) {
    win_hdc_1 = GetWindowDC16(struct_e_1.hwnd16_field4_0x4);
    paStack10 = (_PTR_LOOP_1050_4230 + 0xe);
    hpalette_12 = palette_op_1008_4e08(&win_hdc_1,param_1,paStack10,CONCAT22(0x1050,&win_hdc_1));
    GetWindowRect16(CONCAT22(0x1050,&local_rect_1),struct_e_1.hwnd16_field4_0x4);
    x = (iStack16 - local_rect_1) + -0x1;
    y = (iStack14 - iStack18) + -0x1;
    iVar1 = struct_e_1.field12_0x12;
    iVar3 = y;
    if (is_iconic == 0) {
      iVar3 = struct_e_1.field10_0xe - struct_e_1.field12_0x12;
    }
    rect = &DAT_1050_1050;
    hdc = win_hdc_1;
    iVar2 = y;
    brush_handle_2 = GetStockObject16(BLACK_BRUSH);
    FillRect16(brush_handle_2,rect,hdc);
    puVar2 = struct_e_1.field5_0x6;
    uVar8 = (puVar2 >> 0x10);
    iVar7 = puVar2;
    puVar6 = &iVar7.field_0xe0;
    style = iVar7.field226_0xe2;
    width = puVar6;
    fn_ptr_1 = (*puVar6 + 0x24);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,width,style,0x0,0x0,iVar2);
    iVar5 = (-(puVar6 == 0) & 0x1e) + 0x25;
    handle = CreatePen16(CONCAT22(0x100,iVar5),width,style);
    brush_handle_2 = SelectObject16(handle,win_hdc_1);
    MoveTo16(0x0,0x0,win_hdc_1);
    LineTo16(0x0,x,win_hdc_1);
    LineTo16(y,x,win_hdc_1);
    LineTo16(y,0x0,win_hdc_1);
    string_1 = LineTo16(0x0,0x0,win_hdc_1);
    if (is_iconic == 0) {
      y_00 = struct_e_1.field10_0xe - struct_e_1.field12_0x12;
      MoveTo16(y_00,0x0,win_hdc_1);
      string_1 = LineTo16(y_00,x,win_hdc_1);
    }
    fn_ptr_1 = (*struct_e_1.field5_0x6 + 0x18);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538);
    string_46 = CONCAT22(extraout_DX,string_1);
    if (*string_1 != '\0') {
      SetBkColor16(0x0,win_hdc_1);
      SetTextColor16(CONCAT22(0x100,iVar5),win_hdc_1);
      str46_len = lstrlen16(string_46);
      DVar1 = GetTextExtent16(str46_len,
                              (LPCSTR)CONCAT13((extraout_DX >> 0x8),CONCAT12(extraout_DX,string_1)),
                              win_hdc_1);
      iVar6 = (DVar1 >> 0x10);
      if (is_iconic == 0) {
        iVar9 = (iVar3 - iVar1) / 0x2 - iVar6 / 0x2;
        iVar8 = x / 0x2 - DVar1 / 0x2;
      }
      else {
        iVar9 = y / 0x2 - iVar6 / 0x2;
        iVar8 = 0x2;
      }
      TextOut16(iVar9,CONCAT22(extraout_DX,string_1),iVar9,iVar8,win_hdc_1);
    }
    hpalette_12 = SelectPalette16(0x0,hpalette_12,win_hdc_1);
    DeleteObject16(hpalette_12);
    SelectObject16(brush_handle_2,win_hdc_1);
    DeleteObject16(handle);
    ReleaseDC16(win_hdc_1,struct_e_1.hwnd16_field4_0x4);
  }
  return;
}



pub fn pass1_1020_7f38(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x3ab0;
  (param_1 + 0x2) = 0x1008;
  param_1.address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_draw_op_1020_7f7a(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  let mut HVar1: HGDIOBJ16;
  let mut hcursor2: HCURSOR16;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut paVar2: *mut Struct57;
  let mut struct_1: *mut astruct_20;
  let mut uVar4: u16;
  let mut unaff_SS: u16;
  let mut paVar4: *mut astruct_20;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe90: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffe8: u16;
  let mut uVar1: u16;

  uVar3 = (in_EDX >> 0x10);
  paVar4 = unk_draw_op_1008_61b2
                     (in_stack_0000ffbc,param_1,param_2,param_3,CONCAT22(param_4,param_3));
  paVar2 = CONCAT22(uVar3,(paVar4 >> 0x10));
  uVar4 = (param_1 >> 0x10);
  struct_1 = param_1;
  (struct_1 + 1)->offset_0x0 = 0x389a;
  struct_1[0x1].base_0x2 = 0x1008;
  (struct_1 + 1)->offset_0x0 = 0x3aa8;
  struct_1[0x1].base_0x2 = 0x1008;
  struct_1[0x1].field2_0x4 = 0;
  struct_1[0x1].field3_0x8 = 0;
  struct_1[0x1].field4_0xa = 0;
  param_1.offset_0x0 = 0x82bc;
  struct_1.base_0x2 = 0x1020;
  (struct_1 + 1)->offset_0x0 = 0x8358;
  struct_1[0x1].base_0x2 = 0x1020;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&struct_1.field60_0x5b)),s_VrMode_1050_4422);
  HVar1 = GetStockObject16(HOLLOW_BRUSH);
  struct_1.hgdiobj_field_0xc6 = HVar1;
  hcursor2 = LoadCursor16(0x7f00,0x0);
  struct_1.hcursor_field_0xc4 = hcursor2;
  struct_1.field150_0xc8 = 0x2028;
  struct_1.field139_0xac = 0x47000000;
  struct_1.field145_0xbc = (param_3 + 0x8);
  puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe8,0x48),in_stack_0000fe90,
                           in_stack_0000ffb4,in_stack_0000ffba,in_stack_0000ffbe);
  uVar1 = (puVar5 >> 0x10);
  struct_1.field141_0xb4 = 0;
  struct_1.field142_0xb6 = 0;
  struct_1.field143_0xb8 = (puVar5 + 0xa);
  struct_1.field144_0xba = (puVar5 + 0xc);
  struct_1.field151_0xca = param_3;
  win_ui_reg_class_1008_96d2(param_1);
  return;
}
