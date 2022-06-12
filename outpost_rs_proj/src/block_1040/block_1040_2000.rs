
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_205e(StructD *param_1)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  StructD *iVar4;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x237e;
  iVar4->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  puVar1 = (u32*)&iVar4->field_0x8e;
  uVar2 = &iVar4->field_0x90;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*(char **)&iVar4->field_0xa2);
  fn_ptr_1000_17ce(*(char **)&iVar4->field_0xa6);
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar4->field_0x6);
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn create_win_1040_20d4(mut param_1: u32,StructB *struct_b_param_2,mut param_3: u16 )

{
  let mut cx: i16;
  StructB *struct_b_1;
  let mut uVar1: u16;
  u32 *puVar2;
  char *window_name;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa0: u16;
  RECT16 local_1e;
  let mut iStack26: i16;
  astruct_858 *iStack24;
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut iStack14: i16;
  let mut uStack12: u16;
  let mut iStack10: i16;
  let mut iStack8: i16;
  let mut uStack6: u16;
  let mut iStack4: i16;

  dialog_ui_fn_1040_78e2(struct_b_param_2);
  puVar2 = mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x48),in_stack_0000fe72,
                           in_stack_0000ff96,in_stack_0000ff9c,in_stack_0000ffa0);
  uStack12 = ((u32)puVar2 >> 0x10);
  iStack14 = (int)puVar2;
  iStack8 = (iStack14 + 0xa);
  iStack10 = (iStack14 + 0xc);
  uVar1 = ((u32)struct_b_param_2 >> 0x10);
  struct_b_1 = (StructB *)struct_b_param_2;
  uStack18 = pass1_1008_4772(*(astruct_76 **)&struct_b_1[0x7].field1_0x2);
  cx = ((int)uStack18 + 0x4);
  iStack4 = (iStack8 - cx) / 0x2;
  uStack6 = 0x5;
  SetWindowPos16(0x6,0x1d6,cx,0x5,iStack4,0x0,(HWND16)struct_b_1->lpvoid_field_0x8);
  GetClientRect16(&local_1e,(HWND16)&DAT_1050_1050);
  window_name = load_string_1010_847e(_u16_1050_14cc,0x592);
  uStack22 = 0x50010001;
  CreateWindow16(0x0,(void *)CONCAT22(0x1,HINSTANCE16_1050_038c),(HINSTANCE16)struct_b_1->lpvoid_field_0x8,0x19,0x58,
                 (INT16)(iStack24 + -0x28),(iStack26 + -0x58) / 0x2,0x1,(int)s_Rebel_1050_4ffc + 0x5,window_name,
                 s_OPButton_1050_5ce4);
  SetWindowPos16(0x45,iStack10 + -0xa,*(INT16 *)((int)uStack18 + 0x4),0x5,iStack4,0x0,
                 (HWND16)struct_b_1->lpvoid_field_0x8);
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn mix_draw_op_1040_21d6(param_1: *mut astruct_763)

{
  astruct_13 *paVar1;
  code **ppcVar2;
  u8 uVar4;
  HPALETTE16 hpalette_7;
  let mut uVar7: u16;
  HANDLE16 handle;
  let mut extraout_var: u32;
  let mut in_DX: u16;
  let mut extraout_DX: u16;
  astruct_763 *iVar10;
  let mut count: i16;
  let mut uVar5: u32;
  COLORREF color;
  HGDIOBJ16 handle_00;
  HDC16 hdc_24;
  PAINTSTRUCT16 *paintstruct_22;
  u8 uVar1;
  u32 *uVar2;
  let mut uVar3: u16;
  astruct_764 *iVar5;
  let mut uVar11: u16;

  count = (i16)((u32)param_1 >> 0x10);
  iVar10 = (astruct_763 *)param_1;
  hdc_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar10->field6_0x6);
  paVar1 = *(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe);
  hpalette_7 = palette_op_1008_4e08((HPALETTE16)&hdc_24,in_DX,paVar1,(HDC16 *)CONCAT22(0x1050,&hdc_24));
  uVar2 = iVar10->field141_0x8e;
  ppcVar2 = (code **)((int)*iVar10->field141_0x8e + 0x4);
  (**ppcVar2)(0x1008,(int)uVar2,(int)((u32)uVar2 >> 0x10),0xffff,0xffff,&hdc_24,(int)&DAT_1050_1050);
  uVar5 = pass1_1008_4d72((u32)paVar1);
  uVar3 = (uVar5 >> 0x10);
  iVar5 = (astruct_764 *)uVar5;
  uVar7 = CONCAT11(iVar5->field_0x3e5,iVar5->field_0x3e6);
  uVar1 = iVar5->field996_0x3e4;
  color = SetBkColor16(0x0,hdc_24);
  extraout_DX = (color >> 0x10);
  uVar4 = SetTextColor16(CONCAT22(CONCAT11(0x2,uVar1),uVar7),hdc_24);
  handle_00 = 0x0;
  handle = GetProp16(s_hfont_1050_5ced,iVar10->field6_0x6);
  if (handle != 0x0) {
    handle_00 = SelectObject16(handle,hdc_24);
  }
  DrawText16(0x10,(RECT16 *)
                  ((u32)param_1 & 0xff000000 | (u32)CONCAT12((char)((u32)param_1 >> 0x10),&iVar10->rect_0x92)),
             -0x1,(LPCSTR)iVar10->field152_0xa2,hdc_24);
  SetTextColor16(CONCAT22(CONCAT11(0x2,iVar5->field_0x94),CONCAT11(iVar5->field_0x95,iVar5->field_0x96)),hdc_24);
  DrawText16(0x10,(RECT16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar10->field147_0x9a)),-0x1,
             (LPCSTR)iVar10->field153_0xa6,hdc_24);
  if (handle != 0x0) {
    SelectObject16(handle_00,hdc_24);
  }
  SetBkColor16(color,hdc_24);
  SetTextColor16(CONCAT31((u32)extraout_var,uVar4) & 0xffff | (u32)extraout_DX << 0x10,hdc_24);
  hpalette_7 = SelectPalette16(0x0,hpalette_7,hdc_24);
  DeleteObject16(hpalette_7);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar10->field6_0x6);
  return;
}



StructD * pass1_1040_2358(StructD *param_1,param_2: u8)

{
  pass1_1040_205e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_23ea(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 ,
                    u8 **param_7)

{
  let mut uVar1: u32;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  astruct_57 *iVar2;
  astruct_57 *uVar2;
  u32 *puVar3;
  let mut in_stack_0000fea2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd0: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x9a,param_3,0xfbd,param_6);
  uVar2 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar2 = (astruct_57 *)param_2;
  (u32)(iVar2 + 0x1) = 0x0;
  iVar2[0x1].field2_0x4 = 0x0;
  iVar2[0x1].field3_0x6 = 0x0;
  param_2->field0_0x0 = 0x2956;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  iVar2->field105_0x8a = 0x26;
  param_7 = (u8 **)CONCAT22((int)((u32)param_7 >> 0x10),0x6);
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,param_7,in_stack_0000fea2,in_stack_0000ffc6,in_stack_0000ffcc,
                           in_stack_0000ffd0);
  (iVar2 + 0x1)->field0_0x0 = puVar3;
  iVar2[0x1].field1_0x2 = ((u32)puVar3 >> 0x10);
  uVar1 = (u32)(iVar2 + 0x1);
  iVar2[0x1].field2_0x4 = ((int)uVar1 + 0x28);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_2464(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x2956;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn show_win_1040_2490(StructB *struct_b_param_1)

{
  code **ppcVar1;
  HWND16 hwnd;
  StructB *struct_b_4;
  let mut uVar3: u16;
  let mut piVar2: *mut i16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar3 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_4 = (StructB *)struct_b_param_1;
  hwnd = GetDlgItem16(0xfb1,(HWND16)struct_b_4->lpvoid_field_0x8);
  EnableWindow16(0x0,hwnd);
  ppcVar1 = (code **)((int)*(u32*)&struct_b_4[0x7].field1_0x2 + 0x10);
  piVar2 = (**ppcVar1)((int)s_tile2_bmp_1050_1538,(u32)&struct_b_4[0x7].field1_0x2);
  piVar2 = ((u32)piVar2 >> 0x10);
  move_win_1040_826c(struct_b_param_1,((int)piVar2 + 0x2) + -0x2,((int)piVar2 + 0x4) + *piVar2 + 0x3);
  ShowWindow16(0x5,(HWND16)struct_b_4->lpvoid_field_0x8);
  pass1_1018_1c9a(*(astruct_263 **)&struct_b_4[0x7].field1_0x2,0x1a0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1040_2512(param_1: *mut astruct_57,mut param_2: u16 ,StructC *param_3,mut param_4: u32,mut param_5: u16 ) -> u32

{
  u32 *puVar1;
  let mut uVar2: u16;
  astruct_263 *paVar3;
  let mut uVar4: u16;
  let mut UVar4: u16;
  HWND16 HVar5;
  let mut BVar6: bool;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut UVar6: u16;
  let mut uVar7: u16;
  let mut uVar11: u16;
  u8 *puVar8;
  let mut uVar12: u16;
  u8 *puVar9;
  StructC *iVar8;
  let mut iVar9: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut puVar15: *mut u16;
  let mut uVar10: u32;
  let mut in_stack_0000fe84: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb2: u16;
  u8 local_1e [0x4];
  let mut uStack26: u16;
  let mut uStack24: u16;
  u32 *local_16 [0x2];
  let mut uStack12: u16;
  u32 *puStack10;
  let mut BStack6: bool;
  let mut uStack4: u16;
  let mut piVar1: *mut i16;
  let mut in_stack_0000ffdc: u16;
  let mut uVar13: u32;
  let mut uVar14: u32;
  code **fn_ptr_21;

  BStack6 = 0x0;
  uStack4 = 0x0;
  if (param_5 == 0x2) {//
LAB_1040_266d:
    BStack6 = 0x1;
    uStack4 = 0x0;
  }
  else {
    iVar8 = (StructC *)param_3;
    if (param_5 - 0x2 < 0x19e) {//
LAB_1040_2539:
      param_2 = param_5;
    }
    else {
      uVar8 = ((u32)param_3 >> 0x10);
      if (param_5 - 0x1a0 < 0x14 || param_5 == 0x1b4) {
        UVar4 = IsDlgButtonChecked(param_5,iVar8->field6_0x6);
        if (UVar4 == 0x0) {
          puVar1 = &iVar8->field142_0x92;
          puVar1 = puVar1 + 0x1;
          if (0x0 < &iVar8->field142_0x92) {
            ((int)&iVar8->field142_0x92 + 0x2) = 0x0;
          }
          paVar3 = iVar8->field141_0x8e;
          if (((int)paVar3 + 0x28) == &iVar8->field142_0x92) {
            HVar5 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
            EnableWindow16(0x0,HVar5);
          }
        }
        else {
          puVar1 = &iVar8->field142_0x92;
          puVar1 = puVar1 + -0x1;
          HVar5 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
          BVar6 = IsWindowEnabled16(HVar5);
          if (BVar6 == 0x0) {
            HVar5 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
            EnableWindow16(0x1,HVar5);
          }
          if (&iVar8->field142_0x92 < 0x1) {
            ((int)&iVar8->field142_0x92 + 0x2) = 0x1;
          }
          pass1_1018_1c9a(iVar8->field141_0x8e,param_5);
          puStack10 = (u32 *)pass1_1018_1e78((u32)iVar8->field141_0x8e,-0x1);
          uVar2 = ((u32)puStack10 >> 0x10);
          uVar11 = uVar2 | puStack10;
          if (uVar11 == 0x0) {
            uStack12 = 0x0;
          }
          else {
            uStack12 = (puStack10 + 0x1c);
          }
          win_1008_5c7c(uStack12,uVar11,_u16_1050_02a0,CONCAT22(uStack12,0x1));
        }
        if ((-0x1 < &iVar8->field142_0x92) &&
           (paVar3 = iVar8->field141_0x8e, &iVar8->field142_0x92 <= ((int)paVar3 + 0x28))) {
          sys_1000_3f9c((char *)CONCAT13(0x10,CONCAT12(0x50,local_16)),s__d_1050_5cf4,&iVar8->field142_0x92);
          SetDlgItemText16(CONCAT22(0x1050,local_16),0xfb2,iVar8->field6_0x6);
        }
        goto LAB_1040_266d;
      }
      uVar4 = param_5 - 0xfb1;
      if (uVar4 != 0x0) goto LAB_1040_2539;
      if (&iVar8->field142_0x92 < 0x0) {
        mem_op_1000_179c(0xb4,param_1);
        uStack24 = param_1;
        puVar8 = (uStack24 | uVar4);
        uVar13 = (u32)param_1 & 0xffff0000 | ZEXT24(puVar8);
        uStack26 = uVar4;
        if (puVar8 == NULL) {
          iVar6 = 0x0;
          uVar12 = 0x0;
        }
        else {
          iVar6 = string_1040_8520(uVar13,(astruct_57 *)
                                          CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,uVar4)),
                                   HWND16_1050_0396,0x20030,0x57c057b);
          uVar12 = uVar13;
        }
        puStack10 = (u32 *)CONCAT22(uVar12,iVar6);
        fn_ptr_21 = (code **)((int)*puStack10 + 0x74);
        (**fn_ptr_21)(0x1000,iVar6,uVar12);
        goto LAB_1040_27c0;
      }
      if (0x0 < &iVar8->field142_0x92) {
        mem_op_1000_179c(0xb4,param_1);
        uStack24 = param_1;
        puVar9 = (uStack24 | uVar4);
        uVar13 = (u32)param_1 & 0xffff0000;
        uVar14 = uVar13 | ZEXT24(puVar9);
        uStack26 = uVar4;
        if (puVar9 == NULL) {
          iVar7 = 0x0;
        }
        else {
          iVar7 = string_1040_8520(uVar14,(astruct_57 *)
                                          CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,uVar4)),
                                   HWND16_1050_0396,0x20021,0x57d057b);
          uVar13 = uVar14;
        }
        puStack10 = (u32 *)CONCAT22((int)uVar13,iVar7);
        puVar15 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_1e),0x1,0xc2);
        param_1 = (astruct_57 *)(uVar13 & 0xffff0000 | (u32)puVar15 >> 0x10);
        param_2 = &DAT_1050_1050;
        fn_ptr_21 = (code **)((int)*puStack10 + 0x6c);
        uVar10._0_2_ = (**fn_ptr_21)(0x1008,(char)puStack10,(int)((u32)puStack10 >> 0x10),local_1e);
        if ((int)uVar10 == 0x2) goto LAB_1040_27c0;
      }
      local_16[0] = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x6),in_stack_0000fe84,
                                    in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
      param_1 = (astruct_57 *)((u32)local_16[0] >> 0x10);
      uStack12 = 0x1a0;
      do {
        UVar6 = IsDlgButtonChecked(uStack12,iVar8->field6_0x6);
        if (UVar6 == 0x1) {
          uVar9 = ((u32)local_16[0] >> 0x10);
          iVar9 = (i16)local_16[0];
          (iVar9 + (iVar9 + 0x56) * 0x2 + 0x4e) = uStack12;
          piVar1 = (iVar9 + 0x56);
          *piVar1 = *piVar1 + 0x1;
        }
        uStack12 += 0x1;
      } while ((int)uStack12 < 0x1b5);
      uVar2 = &iVar8->field142_0x92;
      puStack10 = (u32 *)((u32)puStack10 & 0xffff0000 | (u32)uVar2);
      paVar3 = iVar8->field141_0x8e;
      ((int)paVar3 + 0x28) = uVar2;
      PostMessage16(0x0,0xc8,0x111,HWND16_1050_0396);
      param_2 = 0x1;
    }
    uVar12 = SUB42(param_1,0x0);
    BStack6 = post_win_msg_1040_7b3c(param_3,param_4,(param_4 >> 0x10),param_2);
    uStack4 = uVar12;
  }//
LAB_1040_27c0:
  return CONCAT22(uStack4,BStack6);
}



// WARNING: Unable to use type for symbol uVar6
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn draw_ui_op_1040_27cc(param_1: *mut astruct_752,HWND16 hwnd16_param_2,mut param_3: u16 ,HDC16 hdc_param_4) -> u32

{
  let mut uVar1: u32;
  HBRUSH16 brush_handle_var8;
  let mut IVar3: i16;
  astruct_752 *iVar3;
  let mut uVar3: u16;
  let mut unaff_CS: u16;
  let mut uVar7: u16;
  let mut uVar4: u32;
  HDC16 hdc;
  astruct_753 *iVar2;
  let mut uVar2: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  code **fn_ptr_1;

  uVar7 = SUB42(&PTR_LOOP_1050_1040,0x0);
  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_752 *)param_1;
  if (iVar3->brush_handle_field4_0x4 == 0x0) {
    uVar7 = SUB42(s_tile2_bmp_1050_1538,0x0);
    brush_handle_var8 = CreateSolidBrush16(WHITE_BRUSH);
    iVar3->brush_handle_field4_0x4 = brush_handle_var8;
  }
  if (_u16_1050_5cf8 == 0x0) {
    fn_ptr_1 = (code **)((int)(u32)param_1 + 0x68);
    uVar1 = (**fn_ptr_1)(uVar7,param_1,iVar3->field109_0x6e);
    uVar4 = pass1_1008_4d72(uVar1);
    uVar2 = (uVar4 >> 0x10);
    iVar2 = (astruct_753 *)uVar4;
    _u16_1050_5cf8 = CONCAT22(CONCAT11(0x2,iVar2->field_0x94),CONCAT11(iVar2->field_0x95,iVar2->field_0x96));
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return 0x0;
    }
    uVar5 = 0x284a;
    IVar3 = GetDlgCtrlID16(hwnd16_param_2);
    if ((iVar3->field146_0x94 != 0x0) && (IVar3 == 0xfb2)) {
      uVar6 = 0xff;
      hdc = 0x0;
      goto LAB_1040_286e;
    }
  }
  uVar5 = _u16_1050_5cf8;
  uVar6 = ((u32)_u16_1050_5cf8 >> 0x10);
  hdc = hdc_param_4;//
LAB_1040_286e:
  SetTextColor16(CONCAT22(uVar6,uVar5),hdc);
  SetBkColor16(0x1000000,hdc_param_4);
  return CONCAT22(0x1050,iVar3->brush_handle_field4_0x4);
}
pub fn pass1_1040_288e(mut param_1: u32)

{
  let mut uVar1: u16;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u32;
  astruct_394 *paVar5;
  u32 *puVar6;
  let mut uVar7: u16;
  u8 *puVar8;
  let mut in_EDX: u32;
  astruct_57 *paVar9;
  let mut iVar10: i16;
  let mut iVar11: i16;
  let mut uVar12: u16;
  let mut uVar13: u16;

  uVar12 = (param_1 >> 0x10);
  iVar10 = (int)param_1;
  uVar3 = (u32)(iVar10 + 0x8e);
  uVar13 = ((u32)uVar3 >> 0x10);
  iVar11 = (int)uVar3;
  puVar6 = (u32 *)(u32)(iVar11 + 0x24);
  paVar9 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)(iVar11 + 0x26));
  ppcVar2 = (code **)((int)*puVar6 + 0x14);
  (**ppcVar2)();
  paVar5 = (astruct_394 *)puVar6;
  uVar4 = (long)paVar9 << 0x10;
  if (*(i32 *)(iVar10 + 0x70) != 0x0) {
    paVar5 = *(astruct_394 **)(iVar10 + 0x70);
    uVar1 = (iVar10 + 0x72);
    uVar7 = uVar1 | paVar5;
    paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)uVar7);
    if (uVar7 != 0x0) {
      ppcVar2 = (code **)(u32)paVar5;
      (**ppcVar2)();
    }
  }
  mem_op_1000_179c(0x14,paVar9);
  puVar8 = (paVar9 | paVar5);
  if (puVar8 == NULL) {
    paVar5 = NULL;
    puVar8 = NULL;
  }
  else {
    struct_1008_4c58(paVar5);
  }
  *(astruct_394 **)(iVar10 + 0x70) = paVar5;
  *(u8 **)(iVar10 + 0x72) = puVar8;
  pass1_1008_4d84(puVar8,*(astruct_90 **)(iVar10 + 0x70),(u32)puVar6 & 0xffff | uVar4);
  return;
}



StructD * pass1_1040_2930(StructD *param_1,param_2: u8)

{
  pass1_1040_2464(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * pas1_1040_29c2(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_57,mut param_4: u32,mut param_5: u16 )

{
  astruct_57 *iVar1;
  astruct_57 *uVar1;

  pass1_1040_b0bc(param_3,param_4,CONCAT22(param_5,0x157));
  uVar1 = (astruct_57 *)((u32)param_3 >> 0x10);
  iVar1 = (astruct_57 *)param_3;
  param_3->field0_0x0 = 0x2e26;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x64b);
  iVar1[0x1].field3_0x6 = param_1;
  iVar1[0x1].field4_0x8 = param_2;
  load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),0x64a);
  iVar1[0x1].field5_0xa = param_1;
  iVar1[0x1].field6_0xc = param_2;
  return param_3;
}
pub fn pass1_1040_2a22(StructD *param_1)

{
  StructD *iVar1;
  let mut uVar1: u16;
  let mut in_stack_0000ffd2: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x2e26;
  iVar1->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(*(char **)&iVar1->field_0x94);
  fn_ptr_1000_17ce(*(char **)&iVar1->field_0x98);
  unk_draw_op_1040_b0f8(in_stack_0000ffd2,param_1);
  return;
}
pub fn dlg_ui_op_1040_2a64(mut param_1: u16 ,StructB *struct_b_param_1)

{
  let mut uVar1: u32;
  let mut in_AX: u16;
  astruct_57 *paVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  StructB *struct_b_6;
  astruct_918 *iVar8;
  let mut uVar7: u16;
  let mut in_stack_0000fe30: u16;
  let mut in_stack_0000fe34: u16;
  let mut in_stack_0000ff5a: u16;
  let mut in_stack_0000ff5e: u16;
  let mut in_stack_0000ff62: u16;
  let mut in_stack_0000ffa2: u16;
  let mut iVar9: i16;
  RECT16 local_16;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut iStack14: i16;
  let mut uStack12: u32;
  let mut uStack8: u32;
  let mut iStack4: i16;
  astruct_57 *paVar6;

  unk_win_ui_op_1040_b230(param_1,struct_b_param_1);
  iStack4 = 0x5;
  iVar9 = 0x0;
  uVar7 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_6 = (StructB *)struct_b_param_1;
  uVar1 = (u32)&struct_b_6[0x7].hwnd_0x6;
  uStack12 = struct_op_1030_73a8(*(astruct_419 **)((int)uVar1 + 0x6),in_AX,param_1);
  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,(int)(uStack12 >> 0x10));
  PTR_LOOP_1050_5d04 = pass1_1028_4a9a(uStack12,iVar9);
  for (iStack14 = 0x0; iStack14 < iStack4; iStack14 += 0x1) {
    if (iStack14 != 0x0) {
      ((int)&PTR_LOOP_1050_5d04 + iStack14 * 0xc) = 0x0;
    }
    iVar9 = iStack14 * 0xc;
    local_16.x = *(INT16 *)(iVar9 + 0x5cfc);
    local_16.y = *(INT16 *)(iVar9 + 0x5cfe);
    paVar2 = (astruct_57 *)((int)&PTR_LOOP_1050_0000 + 0x1);
    uStack18 = 0x1;
    uStack16 = 0x1;
    MapDialogRect16(&local_16,(HWND16)&DAT_1050_1050);
    mem_op_1000_179c(0x42,paVar5);
    uVar4 = (astruct_57 *)paVar5 | paVar2;
    paVar6 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)uVar4);
    if (uVar4 == 0x0) {
      paVar2 = NULL;
      paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000);
    }
    else {
      pass1_1008_3bd6((u32)paVar6,paVar2,(astruct_57 *)paVar5,0x1,CONCAT22(local_16.x,local_16.y),0x101,0xff0100,
                      CONCAT22(struct_b_6->lpvoid_field_0x8,(iVar9 + 0x5d00)),in_stack_0000ffa2,
                      in_stack_0000fe30,in_stack_0000fe34,in_stack_0000ff5a,in_stack_0000ff5e,in_stack_0000ff62);
      paVar5 = paVar6;
    }
    uVar4 = paVar5;
    uStack8 = CONCAT22(uVar4,paVar2);
    if (PTR_LOOP_1050_5d04 == NULL) {
      if ((iStack14 != 0x0) && ((uVar4 | paVar2) != 0x0)) {
        EnableWindow16(0x0,*(HWND16 *)&paVar2->field11_0x18);
      }
    }
    else {
      iVar8 = (astruct_918 *)(iStack14 * 0xc);
      uVar3 = pass1_1028_4a9a(uStack12,(iVar8 + 0x5d02));
      if (uVar3 != 0x0) {
        (iVar8 + 0x5d04) = 0x1;
        SetDlgItemText16((u32)&struct_b_6[0x7].field6_0xc,*(INT16 *)(iVar8 + 0x5d06),
                         (HWND16)struct_b_6->lpvoid_field_0x8);
      }
    }
  }
  return;
}
pub fn win_ui_op_1040_2bb2(u8 *param_1,astruct_903 *pstruct_903_param_2,mut param_3: u16 ,mut param_4: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  HWND16 HVar3;
  astruct_922 *iVar4;
  let mut iVar5: i16;
  astruct_920 *iVar3;
  let mut uVar6: u16;
  let mut uVar7: u16;
  u8 *id;
  let mut iStack8: i16;
  let mut iStack4: i16;

  if (param_4 == 0x158) {
    PTR_LOOP_1050_5d04 = (PTR_LOOP_1050_5d04 == NULL);
    if (PTR_LOOP_1050_5d04 == NULL) {
      for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 0x1) {
        iVar5 = iStack8 * 0xc;
        HVar3 = GetDlgItem16(*(INT16 *)(iVar5 + 0x5d00),*(HWND16 *)((int)pstruct_903_param_2 + 0x6));
        EnableWindow16(0x0,HVar3);
        ((int)&PTR_LOOP_1050_5d04 + iVar5) = 0x0;
        SetDlgItemText16((u32)((int)pstruct_903_param_2 + 0x94),
                         *(INT16 *)((int)&PTR_s_post_1050_015d_1050_5d06 + iVar5),
                         *(HWND16 *)((int)pstruct_903_param_2 + 0x6));
      }
      HVar3 = *(HWND16 *)((int)pstruct_903_param_2 + 0x6);
      uVar1 = (u32)((int)pstruct_903_param_2 + 0x94);
      uVar6 = uVar1;
      uVar7 = ((u32)uVar1 >> 0x10);
      id = PTR_s_post_1050_015d_1050_5d06;
      goto LAB_1040_2ccc;
    }
    for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 0x1) {
      iVar3 = (astruct_920 *)(iStack8 * 0xc);
      HVar3 = GetDlgItem16(*(INT16 *)(iVar3 + 0x5d00),*(HWND16 *)((int)pstruct_903_param_2 + 0x6));
      EnableWindow16(0x1,HVar3);
      (iVar3 + 0x5d04) = 0x0;
      SetDlgItemText16((u32)((int)pstruct_903_param_2 + 0x94),*(INT16 *)(iVar3 + 0x5d06),
                       *(HWND16 *)((int)pstruct_903_param_2 + 0x6));
    }
    HVar3 = *(HWND16 *)((int)pstruct_903_param_2 + 0x6);
    id = PTR_s_post_1050_015d_1050_5d06;
  }
  else {
    if (param_4 == 0x159) {
      iStack4 = 0x1;
    }
    else if (param_4 == 0x15a) {
      iStack4 = 0x2;
    }
    else if (param_4 == 0x15b) {
      iStack4 = 0x3;
    }
    else {
      if (param_4 != 0x15c) {
        pass1_1040_b54a(param_1,pstruct_903_param_2,param_3,param_4);
        return;
      }
      iStack4 = 0x4;
    }
    if (iStack4 == 0x0) {
      return;
    }
    iVar4 = (astruct_922 *)(iStack4 * 0xc);
    uVar2 = ((iVar4 + 0x5d04) == 0x0);
    (iVar4 + 0x5d04) = uVar2;
    if (uVar2 == 0x0) {
      HVar3 = *(HWND16 *)((int)pstruct_903_param_2 + 0x6);
      uVar1 = (u32)((int)pstruct_903_param_2 + 0x94);
      uVar6 = uVar1;
      uVar7 = ((u32)uVar1 >> 0x10);
      id = *(u8 **)(iVar4 + 0x5d06);
      goto LAB_1040_2ccc;
    }
    HVar3 = *(HWND16 *)((int)pstruct_903_param_2 + 0x6);
    id = ((int)&PTR_s_post_1050_015d_1050_5d06 + iStack4 * 0xc);
  }
  uVar1 = (u32)((int)pstruct_903_param_2 + 0x98);
  uVar6 = uVar1;
  uVar7 = ((u32)uVar1 >> 0x10);//
LAB_1040_2ccc:
  SetDlgItemText16(CONCAT22(uVar7,uVar6),(INT16)id,HVar3);
  return;
}
pub fn win_dlg_item_1040_2d48(mut param_1: u32)

{
  let mut UVar1: u16;
  let mut value: u16;
  let mut local_4: bool;

  pass1_1040_b45e(param_1);
  UVar1 = GetDlgItemInt16(0x1,&local_4,(INT16)&DAT_1050_1050,0x163);
  value = GetDlgItemInt16(0x1,&local_4,(INT16)&DAT_1050_1050,0x165);
  if (UVar1 != 0x0) {
    value /= UVar1;
  }
  SetDlgItemInt16(0x1,value,0x165,*(HWND16 *)((int)param_1 + 0x6));
  return;
}
pub fn pass1_1040_2dac(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar2: u32;
  let mut iStack10: i16;

  uVar1 = (u32)((int)param_1 + 0x90);
  uVar2 = struct_op_1030_73a8(*(astruct_419 **)((int)uVar1 + 0x6),in_AX,in_DX);
  for (iStack10 = 0x0; iStack10 < 0x5; iStack10 += 0x1) {
    pass1_1028_4ab2(uVar2,((int)&PTR_LOOP_1050_5d04 + iStack10 * 0xc),(iStack10 * 0xc + 0x5d02));
  }
  return;
}



StructD * pass1_1040_2e00(StructD *param_1,param_2: u8)

{
  pass1_1040_2a22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_2ea2(StructD *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x180,param_6);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (iVar1 + 0x1)->field0_0x0 = 0x0;
  iVar1[0x1].field1_0x2 = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  iVar1[0x1].field3_0x6 = 0x0;
  (u32)&iVar1[0x1].field4_0x8 = 0x0;
  param_2->field0_0x0 = 0x3436;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3c),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field4_0x8 = puVar2;
  iVar1[0x1].field5_0xa = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_2f06(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x3436;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_2f32(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_27 *paVar1;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  iVar2 = 0x0;
  paVar1 = (astruct_27 *)
           mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,(u8 **)0x2b,
                           in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
  pass1_1010_038e(paVar1,iVar2);
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  return;
}
pub fn show_win_1040_2f5a(StructB *param_1)

{
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  uVar1 = ((u32)param_1 >> 0x10);
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  SetFocus16(*(HWND16 *)((int)param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_dlg_op_1040_2f90(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u16;
  HWND16 HVar2;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  astruct_943 *iVar4;
  let mut uVar4: u16;
  u32 *puVar5;
  let mut uVar6: u32;
  char *l_param;
  let mut in_stack_0000fd7a: u16;
  let mut in_stack_0000fd7c: u16;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000fea4: u16;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000fea8: u16;
  let mut in_stack_0000feaa: u16;
  let mut in_stack_0000fed2: u16;
  let mut in_stack_0000fed4: u16;
  u32 *local_116;
  u32 *local_112;
  WORD local_10e [0x41];
  u8 local_8c [0x82];
  let mut uStack10: u32;
  u32 *puStack6;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puStack6 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fed2,0x2),in_stack_0000fd7a,
                             in_stack_0000fe9e,in_stack_0000fea4,in_stack_0000fea8);
  paVar3 = (astruct_57 *)((u32)paVar3 & 0xffff0000 | (u32)puStack6 >> 0x10);
  uStack10 = (u32)((int)puStack6 + 0x68);
  uVar4 = (param_2 >> 0x10);
  iVar4 = (astruct_943 *)param_2;
  GetWindowText16(0x80,CONCAT22(0x1050,local_8c),iVar4->field6_0x6);
  wsprintf16(local_10e,(char *)CONCAT22(local_8c,0x1050),(char *)CONCAT22((int)uStack10,0x1050),
             (int)((u32)uStack10 >> 0x10));
  SetWindowText16(CONCAT22(0x1050,local_10e),iVar4->field6_0x6);
  HVar2 = GetDlgItem16(0x182,iVar4->field6_0x6);
  iVar4->field143_0x92 = HVar2;
  pass1_1018_3a94(iVar4->field145_0x96,(u32 *)CONCAT22(0x1050,&local_116),(u32 *)CONCAT22(0x1050,&local_112));
  send_msg_1040_3374(param_2,local_112,iVar4->field143_0x92);
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fed4,0x2f),in_stack_0000fd7c,
                           in_stack_0000fea0,in_stack_0000fea6,in_stack_0000feaa);
  uVar1 = ((u32)puVar5 >> 0x10);
  uVar6 = (u32)((int)puVar5 + 0x24);
  uVar6 = pass1_1018_3a7a(uVar6,uVar1,iVar4->field145_0x96,uVar6);
  SendMessage16(uVar6,0xffff,0x40d,iVar4->field143_0x92);
  HVar2 = GetDlgItem16(0x183,iVar4->field6_0x6);
  iVar4->field144_0x94 = HVar2;
  send_msg_1040_3374(param_2,local_116,HVar2);
  l_param = load_string_1010_847e(_u16_1050_14cc,0x531);
  SendDlgItemMessage16((LPARAM)l_param,0x0,0x403,0x183,iVar4->field6_0x6);
  SendDlgItemMessage16((LPARAM)l_param,0xffff,0x40d,0x183,iVar4->field6_0x6);
  HVar2 = GetDlgItem16(0x181,iVar4->field6_0x6);
  iVar4->field141_0x8e = HVar2;
  HVar2 = GetDlgItem16(0x184,iVar4->field6_0x6);
  iVar4->field142_0x90 = HVar2;
  return;
}
