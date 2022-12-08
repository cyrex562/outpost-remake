
pub fn struct_1040_b082(param_1: *mut astruct_57,mut param_2: u32)

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,param_2,(param_2 >> 0x10));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 1).field0_0x0 = 0;
  iVar1[0x1].field1_0x2 = 0;
  param_1.field0_0x0 = 0xb772;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}
pub fn pass1_1040_b0bc(param_1: *mut astruct_57,mut param_2: u32,mut param_3: u32)

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: u16;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,param_3,(param_3 >> 0x10));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 1).field0_0x0 = 0;
  iVar1[0x1].field1_0x2 = param_2;
  param_1.field0_0x0 = 0xb772;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_draw_op_1040_b0f8(mut param_1: u16 ,param_2: *mut StructD)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_EDX: *mut Struct57;
  let mut iVar3: *mut StructD;
  let mut uVar3: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe92: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc0: u16;
  let mut uStack22: u16;
  let mut pcStack10: *mut c_char;
  let mut in_stack_0000ffe8: u32;

  uVar3 = (param_2 >> 0x10);
  iVar3 = param_2;
  param_2.address_offset_field_0x0 = 0xb772;
  iVar3.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  _param_1 = CONCAT22(uStack22,0x32);
  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,_param_1,in_stack_0000fe92,in_stack_0000ffb6,
                           in_stack_0000ffbc,in_stack_0000ffc0);
  pass1_1010_7b8c(puVar3,&iVar3.field_0x6);
  if (&iVar3.field_0x8e != 0) {
    DeleteObject16(&iVar3.field_0x8e);
    iVar3.field_0x8e = 0;
  }
  uVar1 = &iVar3.field_0x90;
  uVar2 = &iVar3.field_0x92;
  pcStack10 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0) {
    _param_1 = CONCAT22(uVar2,uVar1);
    pass1_1040_a5d0(_param_1);
    fn_ptr_1000_17ce(pcStack10);
  }
  ui_cleanup_op_1040_782c(param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_b17c(param_1: *mut u8,mut param_2: u32,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut pcVar3: *mut c_char;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut uVar5: u32;
  let mut iVar6: i16;
  let mut unaff_SI: u16;
  let mut uVar7: u16;
  let mut puVar8: *mut u32;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  u8 **ppuVar9;
  let mut puStack12: *mut u16;
  let mut iStack4: i16;

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  iStack4 = 0;
  loop {
    uVar7 = (param_2 >> 0x10);
    iVar6 = param_2;
    piVar1 = (iVar6 + 0x90);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    paVar4 = (paVar4 & 0xffff0000 | piVar1 >> 0x10);
    uVar2 = (piVar1 + 2);
    (iStack4 * 0xa + uVar2 + 0x4) = (iStack4 * 0x2 + param_3);
    iStack4 += 0x1;
  }
  ppuVar9 = CONCAT22(unaff_SI,0x3);
  puVar8 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,ppuVar9,in_stack_0000fe94,in_stack_0000ffb8,in_stack_0000ffbe,
                           in_stack_0000ffc2);
  uVar5 = puVar8 >> 0x10;
  uVar2 = (iVar6 + 0x90);
  puStack12 = (uVar2 + 2);
  for (iStack4 = 0; piVar1 = (iVar6 + 0x90), *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 += 1) {
    ppuVar9 = (ppuVar9 & 0xffff0000);
    uVar2 = (iVar6 + 0x90);
    uVar2 = (uVar2 + 0x6);
    pcVar3 = pass1_1010_b038(puVar8,uVar2,(uVar2 >> 0x10),
                             (puStack12 + 0x4),(ppuVar9 >> 0x10));
    string_1040_a626(uVar5,puStack12,CONCAT22(uVar5,pcVar3));
    puStack12 = (puStack12 & 0xffff0000 | (puStack12 + 0xa));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_ui_op_1040_b230(mut param_1: u16 ,StructB *param_2)

{
  let mut ppcVar1: *mut *mut code;
  let mut cy_caption_1: i16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut uVar3: u16;
  let mut in_stack_0000fe6e: u16;
  let mut in_stack_0000ff92: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ff9c: u16;
  let mut piVar3: *mut i16;
  let mut uVar4: u16;
  let mut pcVar5: *mut c_char;
  let mut local_1a: i16;
  let mut iStack24: i16;
  let mut iStack22: i16;
  let mut iStack20: i16;
  let mut iStack18: i16;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut puStack10: *mut u32;
  let mut local_6: i16;
  let mut local_4: i16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(param_2);
  if (PTR_LOOP_1050_5ef8 == (&u32_1050_0004 + 1)) {
    PTR_LOOP_1050_5ef8 = NULL;
  }
  pcVar5 = CONCAT22(0x1050,&local_4);
  piVar3 = &local_6;
  uVar4 = SUB42(&DAT_1050_1050,0x0);
  puStack10 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(piVar3,0x48),in_stack_0000fe6e,
                              in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
  pass1_1008_3e94((puStack10 & 0xffff0000 | (puStack10 + 0xe)),CONCAT22(uVar4,piVar3),
                  pcVar5);
  uVar3 = (puStack10 >> 0x10);
  iStack12 = (puStack10 + 0xa);
  iStack14 = (puStack10 + 0xc);
  cy_caption_1 = GetSystemMetrics16(SM_CYCAPTION);
  iStack16 = cy_caption_1 * PTR_LOOP_1050_5ef8 + 0xa;
  PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 1;
  iStack18 = iStack16 + local_6;
  iStack16 += local_4;
  uVar4 = (param_2 >> 0x10);
  GetWindowRect16(CONCAT22(0x1050,&local_1a),(param_2 + 0x6));
  if (iStack14 < (iStack20 - iStack24) + iStack18) {
    iStack18 = -0x2 - ((iStack20 - iStack24) - iStack14);
  }
  if (iStack12 < (iStack22 - local_1a) + iStack16) {
    iStack16 = -0x2 - ((iStack22 - local_1a) - iStack12);
  }
  SetWindowPos16(0x1,0x0,0x0,iStack18,iStack16,0x0,(param_2 + 0x6));
  ppcVar1 = (param_2 + 0x6c);
  (**ppcVar1)(s_tile2_bmp_1050_1538,param_2);
  return;
}



pub unsafe fn pass1_1040_b316(param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut uStack4: u16;

  if (param_5 == 0xf) {
    ppcVar1 = (*param_1 + 0x60);
    uStack4 = (**ppcVar1)();
  }
  else if (param_5 == 0x111) {
    ppcVar1 = (*param_1 + 0x10);
    (**ppcVar1)();
    uStack4 = 0x1;
  }
  else {
    uStack4 = pass1_1040_79c0(param_1,param_2,param_3,param_4,param_5);
  }
  return uStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_b372(mut param_1: u32,hwnd_param_2: HWND16,mut param_3: u16 ,hdc_param_4: HDC16)

{
  let mut uVar1: u16;
  let mut dlg_ctrl_id: i16;
  let mut local_brush_handle: HBRUSH16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar2: u32;
  iVar1: *mut astruct_798;
  let mut uVar3: u16;
  let mut uVar6: u16;

  uVar5 = (param_1 >> 0x10);
  if ((param_1 + 0x8e) == 0) {
    local_brush_handle = CreateSolidBrush16(WHITE_BRUSH);
    (param_1 + 0x8e) = local_brush_handle;
  }
  if (_PTR_LOOP_1050_5efa == 0) {
    uVar2 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
    uVar1 = (uVar2 >> 0x10);
    iVar1 = uVar2;
    _PTR_LOOP_1050_5efa = CONCAT12(iVar1.field_0x94,CONCAT11(iVar1.field_0x95,iVar1.field_0x96));
  }
  if (param_3 < 0x4) {//
// LAB_1040_b3ea:
    dlg_ctrl_id = GetDlgCtrlID16(hwnd_param_2);
    if (dlg_ctrl_id == 0x14c) {
      uVar3 = 0xffff;
      uVar6 = 0;
  // TODO: goto LAB_1040_b41a;
    }
    if (dlg_ctrl_id == 0x175) {
      uVar3 = 0xff;
      uVar6 = 0;
  // TODO: goto LAB_1040_b41a;
    }
  }
  else if (param_3 != 0x4) {
    if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
      return;
    }
// TODO: goto LAB_1040_b3ea;
  }
  uVar3 = _PTR_LOOP_1050_5efa;
  uVar6 = (_PTR_LOOP_1050_5efa >> 0x10);//
// LAB_1040_b41a:
  SetTextColor16(CONCAT22(uVar6,uVar3),hdc_param_4);
  SetBkColor16(0x1000000,hdc_param_4);
  return;
}
pub fn show_win_1040_b43c(param_1: u32)

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = (*param_1 + 0x70);
  (**ppcVar1)();
  ShowWindow16(0x5,(param_1 + 0x6));
  return;
}
pub fn pass1_1040_b45e(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut piVar2: *mut i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut iStack8: i16;
  let mut puStack6: *mut u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if ((iVar3 + 0x90) != 0) {
    uVar1 = (iVar3 + 0x90);
    (uVar1 + 0x14) = (iVar3 + 0x6);
    uVar1 = (iVar3 + 0x90);
    puStack6 = (uVar1 + 2);
    for (iStack8 = 0; piVar2 = (iVar3 + 0x90), *piVar2 != iStack8 && iStack8 <= *piVar2; iStack8 += 1) {
      uVar1 = (puStack6 + 2);
      SetDlgItemText16(CONCAT22(uVar1,*puStack6),(uVar1 >> 0x10),(iVar3 + 0x6));
      puStack6 = (puStack6 & 0xffff0000 | (puStack6 + 0xa));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_b4c8(param_1: *mut u8,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut puVar8: *mut u32;
  let mut in_stack_0000fea2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000fffa: u16;

  paVar5 = CONCAT22(in_register_0000000a,param_1);
  uVar7 = (param_2 >> 0x10);
  if ((param_2 + 0x90) != 0) {
    puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,CONCAT22(in_stack_0000fffa,0x32),in_stack_0000fea2,
                             in_stack_0000ffc6,in_stack_0000ffcc,in_stack_0000ffd0);
    uVar6 = paVar5 & 0xffff0000 | puVar8 >> 0x10;
    uVar3 = puVar8;
    uVar2 = (param_2 + 0x90);
    iVar1 = (uVar2 + 0xa);
    iVar4 = iVar1 + -0x4;
    if (iVar4 == 0) {
      ui_op_1010_79aa(puVar8,0xfd9,0x0);
      if (iVar4 == 0) {
        uVar7 = 0xe;//
// LAB_1040_b50f:
        unk_win_op_1010_7300
                  (uVar6,(puVar8 & 0xffff0000 | uVar3),CONCAT22(iVar4,iVar4),uVar7,
                   CONCAT22(iVar4,iVar4));
        return;
      }
    }
    else if (((0x0 < iVar1 + -0x5) && (!SBORROW2(iVar1 + -0x5,1))) &&
            (iVar4 = iVar1 + -0x7, iVar4 == 0x0 || iVar1 + -0x6 < 1)) {
      ui_op_1010_79aa(puVar8,0xfda,0x0);
      if (iVar4 == 0) {
        uVar7 = 0xd;
    // TODO: goto LAB_1040_b50f;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_b54a(param_1: *mut u8,param_2: *mut astruct_903,mut param_3: u16 ,mut param_4: u32)

{
  let mut pSVar1: *mut StructD;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut iVar5: i16;
  let mut pSVar6: *mut StructD;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u32;
  iVar6: *mut astruct_515;
  let mut uVar10: u16;
  let mut puVar11: *mut u32;
  let mut paVar12: *mut Struct57;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut uVar13: u8;
  let mut uVar14: u8;
  let mut uVar15: u16;
  let mut uVar16: u16;
  let mut uVar17: u16;
  let mut in_stack_0000ffe6: u16;

  paVar8 = CONCAT22(in_register_0000000a,param_1);
  if (param_4 == 0xea) {
    ppcVar2 = (param_2 + 0x5c);
    (**ppcVar2)();
  }
  else if (param_4 == 0xeb) {
    puVar11 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x3),in_stack_0000fe8e,
                              in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
    uVar7 = (puVar11 >> 0x10);
    pSVar1 = (param_2 + 0x90);
    if (pSVar1.is_null() == false) {
      uVar10 = (pSVar1 >> 0x10);
      uVar15 = 0x1010;
      pSVar6 = pSVar1;
      pass1_1010_ad64(pSVar1,uVar7,puVar11,CONCAT22((pSVar1 + 0xa),uVar7),
                      (pSVar1 + 0x6));
      (param_2 + 0x90) = pSVar6;
      (param_2 + 0x92) = uVar7;
      if ((uVar7 | (param_2 + 0x90)) == 0) {
        (param_2 + 0x90) = pSVar1;
      }
      else {
        if (pSVar1.is_null() == false) {
          pass1_1040_a5d0(pSVar1);
          uVar15 = 0x1000;
          fn_ptr_1000_17ce(pSVar1);
        }
        ppcVar2 = (param_2 + 0x70);
        (**ppcVar2)(uVar15,param_2);
      }
    }
  }
  else {
    if (param_4 == 0x1790) {
      paVar12 =
                mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x32),in_stack_0000fe8e,
                                in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
      uVar9 = paVar8 & 0xffff0000 | paVar12 >> 0x10;
      uVar3 = (param_2 + 0x90);
      uVar3 = (uVar3 + 0x6);
      iVar4 = pass1_1010_7d38(paVar12,(paVar12 >> 0x10),uVar3,(uVar3 >> 0x10)
                             );
      iVar5 = iVar4;
      ui_op_1010_79aa(paVar12,0xfab,0x0);
      if (iVar5 != 0) {
        return;
      }
      if (iVar4 == 0) {
        uVar3 = (param_2 + 0x90);
        uVar10 = (uVar3 >> 0x10);
        iVar6 = uVar3;
        uVar3 = iVar6.field6_0x6;
        uVar16 = uVar3;
        uVar17 = (uVar3 >> 0x10);
        uVar15 = 0x14;
      }
      else {
        uVar3 = (param_2 + 0x90);
        uVar10 = (uVar3 >> 0x10);
        iVar6 = uVar3;
        uVar3 = iVar6.field6_0x6;
        uVar16 = uVar3;
        uVar17 = (uVar3 >> 0x10);
        uVar15 = 0x9;
      }
      uVar13 = uVar10;
      uVar14 = (uVar10 >> 0x8);
    }
    else if (param_4 == 0x1824) {
      paVar12 =
                mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x32),in_stack_0000fe8e,
                                in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
      uVar9 = paVar8 & 0xffff0000 | paVar12 >> 0x10;
      iVar6 = paVar12;
      uVar3 = (param_2 + 0x90);
      ui_op_1010_79aa(paVar12,0xfc5,(uVar3 + 0x6));
      if (iVar6.is_null() == false) {
        return;
      }
      uVar3 = (param_2 + 0x90);
      uVar3 = (uVar3 + 0x6);
      uVar16 = uVar3;
      uVar17 = (uVar3 >> 0x10);
      uVar15 = 0x12;
      uVar13 = 0;
      uVar14 = 0;
    }
    else {
      if (param_4 != 0x1830) {
        post_win_msg_1040_7b3c((StructC *)param_2,param_3,param_4,param_4);
        return;
      }
      paVar12 =
                mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x32),in_stack_0000fe8e,
                                in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
      uVar9 = paVar8 & 0xffff0000 | paVar12 >> 0x10;
      iVar6 = paVar12;
      uVar3 = (param_2 + 0x90);
      ui_op_1010_79aa(paVar12,0xfb6,(uVar3 + 0x6));
      if (iVar6.is_null() == false) {
        return;
      }
      uVar3 = (param_2 + 0x90);
      uVar3 = (uVar3 + 0x6);
      uVar16 = uVar3;
      uVar17 = (uVar3 >> 0x10);
      uVar15 = 0xc;
      uVar13 = 0;
      uVar14 = 0;
    }
    unk_win_op_1010_7300(uVar9,paVar12,CONCAT13(uVar14,CONCAT12(uVar13,iVar6)),uVar15,CONCAT22(uVar17,uVar16));
  }
  return;
}
pub fn destroy_window_1040_b726(mut param_1: u32,mut param_2: i16)

{
  let mut fn_ptr_1: *mut *mut code;

  if (param_2 != 0) {
    fn_ptr_1 = (param_1 + 0x78);
    (**fn_ptr_1)();
  }
  DestroyWindow16((param_1 + 0x6));
  return;
}



pub fn pass1_1040_b74c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  let mut in_stack_0000ffde: u16;

  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1040_b7ee(param_1: *mut astruct_57,param_2: i32,mut param_3: u16 )

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;
  let mut uVar2: u16;

  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_3,0xfab));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field3_0x6 = 0;
  iVar1[0x1].field5_0xa = 0;
  iVar1[0x1].field18_0x22 = 0;
  iVar1[0x1].field21_0x26 = 0;
  iVar1[0x1].field_0x28 = 0;
  param_1.field0_0x0 = 0xbeba;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  if (param_2 != 0) {
    uVar2 = (param_2 >> 0x10);
    iVar1[0x1].field18_0x22 = (param_2 + 0x6);
    iVar1[0x1].field21_0x26 = (param_2 + 0x14);
  }
  return;
}



pub unsafe fn pass1_1040_b864(param_1: u32,param_2: *mut i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;

  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566(CONCAT22(param_3,param_2));
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (*param_1 + 0x7c);
    (**ppcVar1)();
  }
  return 0x1;
}
pub fn pass1_1040_b8be(param_1: u32)

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = (*param_1 + 0x80);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_1040_b8d2(mut param_1: u16 ,StructB *param_2)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut in_register_0000000a: u16;
  let mut paVar10: *mut Struct57;
  let mut paVar12: *mut Struct57;
  StructB *struct_b_10;
  StructB *struct_b_10_hi;
  let mut uVar13: u16;
  let mut puVar14: *mut u32;
  let mut puVar15: *mut u16;
  let mut in_stack_0000fe3a: u16;
  let mut in_stack_0000fe3e: u16;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000ff64: u16;
  let mut in_stack_0000ff68: u16;
  let mut in_stack_0000ff6c: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffe4: u16;
  let mut paVar11: *mut Struct57;

  paVar10 = CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(param_2);
  puVar14 = mixed_1010_20ba(paVar10,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe4,0x31),in_stack_0000fe8c,
                            in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  paVar10 = (paVar10 & 0xffff0000 | puVar14 >> 0x10);
  paVar4 = puVar14;
  struct_b_10_hi = (StructB *)(param_2 >> 0x10);
  struct_b_10 = (StructB *)param_2;
  struct_b_10[0x7].field6_0xc = paVar4;
  struct_b_10[0x7].field7_0xe = (puVar14 >> 0x10);
  mem_op_1000_179c(0xa,paVar10);
  uVar8 = paVar10 | paVar4;
  paVar11 = (paVar10 & 0xffff0000 | uVar8);
  if (uVar8 == 0) {
    &struct_b_10[0x7].max_count_field_0x10 = 0;
  }
  else {
    puVar15 = struct_1040_bf3e(CONCAT22(paVar10,paVar4),struct_b_10.lpvoid_field_0x8);
    paVar11 = (paVar11 & 0xffff0000 | puVar15 >> 0x10);
    paVar4 = puVar15;
    struct_b_10[0x7].max_count_field_0x10 = paVar4;
    struct_b_10[0x7].field5_0xa = (puVar15 >> 0x10);
  }
  pass1_1040_bfde(*(void **)&struct_b_10[0x7].max_count_field_0x10,&struct_b_10[0x7].field6_0xc);
  mem_op_1000_179c(0x42,paVar11);
  uVar8 = paVar11 | paVar4;
  paVar10 = (paVar11 & 0xffff0000 | uVar8);
  if (uVar8 != 0) {
    pass1_1008_3bd6(paVar10,paVar4,paVar11,0x1,0xa000a,0x0,0x800081,
                    CONCAT22(struct_b_10.lpvoid_field_0x8,0x10a),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar10);
  uVar8 = paVar10 | paVar4;
  paVar11 = (paVar10 & 0xffff0000 | uVar8);
  if (uVar8 != 0) {
    pass1_1008_3bd6(paVar11,paVar4,paVar10,0x1,0xa0028,0x0,0x840085,
                    CONCAT22(struct_b_10.lpvoid_field_0x8,0x10b),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar11);
  uVar8 = paVar11 | paVar4;
  paVar10 = (paVar11 & 0xffff0000 | uVar8);
  if (uVar8 != 0) {
    pass1_1008_3bd6(paVar10,paVar4,paVar11,0x1,0xa0046,0x0,0x860087,
                    CONCAT22(struct_b_10.lpvoid_field_0x8,0x10d),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar10);
  uVar8 = paVar10 | paVar4;
  paVar11 = (paVar10 & 0xffff0000 | uVar8);
  if (uVar8 != 0) {
    pass1_1008_3bd6(paVar11,paVar4,paVar10,0x1,0xa0064,0x0,0x880089,
                    CONCAT22(struct_b_10.lpvoid_field_0x8,0x10e),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar11);
  uVar8 = paVar11 | paVar4;
  paVar10 = (paVar11 & 0xffff0000 | uVar8);
  if (uVar8 != 0) {
    pass1_1008_3bd6(paVar10,paVar4,paVar11,0x1,0xa0082,0x0,0x820083,
                    CONCAT22(struct_b_10.lpvoid_field_0x8,0x10c),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar10);
  uVar8 = paVar10 | paVar4;
  paVar11 = (paVar10 & 0xffff0000 | uVar8);
  if (uVar8 != 0) {
    pass1_1008_3bd6(paVar11,paVar4,paVar10,0x1,0xa00d2,0x0,0x8a008b,
                    CONCAT22(struct_b_10.lpvoid_field_0x8,0xbbb),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
  }
  mem_op_1000_179c(0x42,paVar11);
  uVar8 = paVar11 | paVar4;
  paVar10 = (paVar11 & 0xffff0000);
  paVar12 = (paVar10 | uVar8);
  if (uVar8 == 0) {
    paVar4 = NULL;
  }
  else {
    pass1_1008_3bd6(paVar12,paVar4,paVar11,0x1,0xa00a0,0x8e,0x8c008d,
                    CONCAT22(struct_b_10.lpvoid_field_0x8,0xbbc),in_stack_0000ffac,in_stack_0000fe3a,in_stack_0000fe3e,
                    in_stack_0000ff64,in_stack_0000ff68,in_stack_0000ff6c);
    paVar10 = paVar12;
  }
  puVar14 = mixed_1010_20ba(paVar10,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe4,0x3),in_stack_0000fe8c,
                            in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  uVar3 = (puVar14 >> 0x10);
  uVar2 = puVar14;
  uVar9 = uVar3;
  uVar5 = pass1_1010_a5ac(uVar2,uVar3,struct_b_10[0x8].field8_0x10);
  uVar6 = pass1_1010_ac62(uVar5,uVar9,uVar2,uVar3,0x1e);
  if (uVar6 != 0) {
    pass1_1010_a5ca(uVar6,uVar9,uVar2,uVar3,uVar5);
    if (0x0 < uVar6) {
      pass1_1010_a58a(uVar6,uVar9,uVar2,uVar3,uVar5);
      if (uVar6 == 0) goto LAB_1040_bb26;
    }
  }
  enable_win_1040_9234(CONCAT22(paVar10,paVar4),0x0);//
// LAB_1040_bb26:
  uVar1 = &struct_b_10[0x7].field6_0xc;
  iVar7 = uVar1;
  uVar1 &= 0xffff0000;
  uVar13 = (uVar1 >> 0x10);
  SetWindowPos16(0x40,(iVar7 + 0x10),(iVar7 + 0xe),(iVar7 + 0xc),
                 (uVar1 | iVar7 + 0xa),0x0,struct_b_10.lpvoid_field_0x8);
  return;
}



pub unsafe fn pass1_1040_bb5a(mut param_1: u32) -> u16

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = ((param_1 + 0x94) + 0x8);
  (**ppcVar1)();
  return 0x0;
}
pub fn destroy_win_1040_bb78(param_1: *mut astruct_35)

{
  let mut uVar1: u16;
  let mut is_window: bool;
  pstruct35_5: *mut astruct_35;
  pstruct35_hi: *mut astruct_35;
  let mut unaff_CS: u16;
  let mut puVar1: *mut u32;
  let mut fn_ptr_1: *mut *mut code;

  pstruct35_hi = (param_1 >> 0x10);
  pstruct35_5 = param_1;
  if (pstruct35_5.hwnd_0xb6 != 0) {
    // 0x1538
    unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
    is_window = IsWindow16(pstruct35_5.hwnd_0xb6);
    if (is_window != 0) {
    // 0x1538
      unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
      DestroyWindow16(pstruct35_5.hwnd_0xb6);
    }
  }
  pstruct35_5.hwnd_0xb6 = 0;
  puVar1 = pstruct35_5.field148_0x94;
  uVar1 = pstruct35_5.field149_0x96;
  if ((uVar1 | puVar1) != 0) {
    fn_ptr_1 = *puVar1;
    (**fn_ptr_1)(unaff_CS,puVar1,uVar1,1);
  }
  &pstruct35_5.field148_0x94 = 0;
  pstruct35_5.field150_0x98 = 0;
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol puVar4
// WARNING: Unable to use type for symbol uVar18
// WARNING: Unable to use type for symbol uVar19
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_bbe2(param_1: *mut u8,param_2: HWND16,param_3: *mut astruct_900,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u32)

{
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut puVar5: *mut u32;
  let mut uVar6: u16;
  let mut BVar7: bool;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar7: u16;
  let mut uVar9: u16;
  let mut hwnd: HWND16;
  let mut uVar10: u16;
  let mut uVar13: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut in_register_0000000a: u16;
  let mut paVar14: *mut Struct57;
  let mut uVar15: u32;
  let mut puVar16: *mut u32;
  let mut paVar17: *mut Struct57;
  let mut uVar16: u32;
  let mut in_stack_0000fe84: u16;
  let mut in_stack_0000fe86: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb4: u16;
  let mut uVar21: u16;
  let mut uStack30: u16;
  let mut local_a: RECT16;
  let mut iStack6: i16;
  let mut iStack4: i16;
  let mut uVar1: u32;
  let mut puVar4: *mut u32;
  let mut uVar17: u16;
  let mut uVar18: u16;
  let mut uVar19: u16;
  let mut in_stack_0000ffde: u16;
  let mut uVar20: u16;

  paVar14 = CONCAT22(in_register_0000000a,param_1);
  if (param_6 != 0x10c) {
    if (param_6 < 0x10d) {
      if (param_6 == 0xfa) {
        ppcVar3 = (*param_3.field148_0x98 + 0x18);
        (**ppcVar3)();
        return;
      }
      if (param_6 == 0x10a) {
        GetClientRect16(&local_a,&DAT_1050_1050);
        puVar5 = param_3.field148_0x98;
        local_a.y += 0x3;
        local_a.x = (puVar5 + 0x1a) + -0x9;
        iStack6 += -0x3;
        iStack4 += -0x3;
        InvalidateRect16(0x1,&local_a,&DAT_1050_1050);
        unk_destroy_win_op_1010_2fa0(param_3.field148_0x98);
        pass1_1010_32c0(param_3.field148_0x98,0x0);
        pass1_1010_2ee2(param_3.field148_0x98);
        return;
      }
      if (param_6 != 0x10b) {//
// LAB_1040_be78:
        pass1_1040_b54a(param_1,CONCAT22(param_4,param_3),param_5,param_6);
        return;
      }
      puVar4 = param_3.field148_0x98;
      uVar2 = (puVar4 + 0x12);
      uVar21 = uVar2;
      puVar16 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,CONCAT22(uVar2,0x3),in_stack_0000fe84,
                                in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
      uVar8 = (puVar16 >> 0x10);
      uStack30 = puVar16;
      uVar6 = uStack30;
      uVar13 = uVar8;
      pass1_1010_a5ca(uStack30,uVar8,uStack30,uVar8,uVar21);
      if ((uVar2 != 0x70) && (uVar6 == 0)) {
        return;
      }
      uVar1 = param_3.field169_0xb0;
      uVar18 = uVar1;
      uVar19 = (uVar1 >> 0x10);
      puVar5 = param_3.field148_0x98;
      uVar17 = (puVar5 + 0x12);
    }
    else {
      if (param_6 != 0x10d) {
        if (param_6 == 0x10e) {
          paVar17 =
                    mixed_1010_20ba(paVar14,_u16_1050_0ed0,CONCAT22(param_2,0x32),in_stack_0000fe86,
                                    in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
          uVar15 = paVar14 & 0xffff0000 | paVar17 >> 0x10;
          iVar7 = paVar17;
          ui_op_1010_79aa(paVar17,0xfc6,param_3.field169_0xb0);
          if (iVar7 != 0) {
            return;
          }
          unk_win_op_1010_7300(uVar15,paVar17,0x0,0x13,param_3.field169_0xb0);
          return;
        }
        if (param_6 != 0xbbb) {
          if (param_6 == 0xbbc) {
            puVar16 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,CONCAT22(param_2,0x3),in_stack_0000fe86,
                                      in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
            uVar2 = (puVar16 >> 0x10);
            uVar8 = puVar16;
            uVar13 = uVar2;
            uVar7 = pass1_1010_a5ac(uVar8,uVar2,param_3.field169_0xb0);
            uVar9 = uVar7;
            pass1_1010_a58a(uVar7,uVar13,uVar8,uVar2,uVar7);
            if (uVar9 == 0) {
              pass1_1010_a568(0x0,uVar13,uVar8,uVar2,uVar7);
            }
            hwnd = GetDlgItem16(0xbbc,param_3.field6_0x6);
            EnableWindow16(0x0,hwnd);
            return;
          }
      // TODO: goto LAB_1040_be78;
        }
        if ((param_3.field171_0xb6 == 0) || (BVar7 = IsWindow16(param_3.field171_0xb6), BVar7 == 0)) {
          uVar16 = pass1_1038_af40(param_3,paVar14,_PTR_LOOP_1050_5b7c,param_3.field6_0x6,0x1b);
          param_3.field171_0xb6 = (uVar16 + 0x6);
          ShowWindow16(0x1,param_3.field171_0xb6);
          return;
        }
        param_2 = param_3.field171_0xb6;
    // TODO: goto LAB_1040_bd39;
      }
      puVar16 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,CONCAT22(param_2,0x3),in_stack_0000fe86,
                                in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
      uVar13 = (puVar16 >> 0x10);
      uStack30 = puVar16;
      uVar15 = param_3.field169_0xb0;
      uVar18 = uVar15;
      uVar19 = (uVar15 >> 0x10);
      uVar17 = 0x71;
      uVar8 = uVar13;
    }
    pass1_1010_a5ec(uVar13,uStack30,uVar8,uVar17,CONCAT22(uVar19,uVar18));
    if ((param_3.field170_0xb4 != 0) && (BVar7 = IsWindow16(param_3.field170_0xb4), BVar7 != 0)) {
      SendMessage16(0x0,0xeb,0x111,param_3.field170_0xb4);
    }
  }
  param_2 = param_3.field6_0x6;//
// LAB_1040_bd39:
  DestroyWindow16(param_2);
  return;
}



pub fn pass1_1040_be94(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  let mut in_stack_0000ffda: u16;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub fn struct_1040_bf3e(param_1: *mut astruct_442,mut param_2: u16 ) -> *mut u16

{
  iVar1: *mut astruct_442;
  uVar1: *mut astruct_442;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field0_0x0 = 0x389a;
  iVar1.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x3aa8;
  iVar1.field1_0x2 = 0x1008;
  iVar1.field2_0x4 = param_2;
  param_1.field0_0x0 = 0x3ab0;
  iVar1.field1_0x2 = 0x1008;
  iVar1.field3_0x6 = 0;
  param_1.field0_0x0 = 0xc53e;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return &param_1.field0_0x0;
}
pub fn pass1_1040_bf92(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0xc53e;
  iVar1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  pass1_1010_1ea6(&iVar1.field_0x6,(param_1 & 0xffff | uVar1 << 0x10));
  unk_destroy_win_op_1010_2fa0(*(astruct_873 **)&iVar1.field_0x6);
  param_1.address_offset_field_0x0 = 0x3ab0;
  iVar1.address_offset_field_0x2 = 0x1008;
  param_1.address_offset_field_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}
pub fn pass1_1040_bfde(void *param_1,param_2: *mut u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  (iVar3 + 0x6) = param_2;
  ppcVar1 = (*param_2 + 0x4);
  (**ppcVar1)();
  uVar2 = (iVar3 + 0x6);
  (uVar2 + 0x22) = (iVar3 + 0x4);
  pass1_1010_2ee2((iVar3 + 0x6));
  return;
}
pub fn invalidate_rect_1040_c028(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut uVar10: u16;
  erase: *mut RECT16;
  rect: *mut RECT16;
  let mut hwnd: HWND16;
  let mut local_a: RECT16;
  let mut iStack6: i16;
  let mut iStack4: i16;
  let mut piVar9: *mut i16;

  iVar8 = param_1;
  uVar10 = (param_1 >> 0x10);
  if (param_2 == 0x8) {
    GetClientRect16(&local_a,&DAT_1050_1050);
    uVar1 = (iVar8 + 0x6);
    uVar3 = (iVar8 + 0x6);
    iVar6 = (uVar3 + 0x16);
    uVar3 = (iVar8 + 0x6);
    local_a.x = (uVar3 + 0x1a);
    uVar3 = (iVar8 + 0x6);
    local_a.y = (uVar3 + 0x1c);
    if (iVar6 != 0) {
      if (iVar6 < 0x2) {
        iVar5 = 0x1;
      }
      else {
        iVar5 = 0x2;
      }
      uVar2 = ((iVar6 - iVar5) * 0x4 + uVar1 + 0x2a);
      iVar6 = uVar2;
      uVar2 &= 0xffff0000;
      local_a.x = (iVar6 + 0x22) + (uVar2 | iVar6 + 0x1e);
    }
    uVar1 = (iVar8 + 0x6);
    iStack6 = (uVar1 + 0x1e);
    iStack4 += -0x5;
  }
  else {
    if (param_2 != 0x9) {
      if (param_2 != 0xa) {
        return;
      }
      uVar1 = (iVar8 + 0x6);
      uVar7 = uVar1 + 0x2a;
      if (((iVar8 + 0x8) | uVar7) == 0) {
        return;
      }
      uVar3 = (iVar8 + 0x6);
      uVar2 = (((uVar3 + 0x16) + -1) * 0x4 + uVar7);
      iVar8 = uVar2;
      uVar2 &= 0xffff0000;
      piVar9 = (uVar2 | iVar8 + 0x1e);
      uVar10 = (uVar2 >> 0x10);
      local_a.y = (iVar8 + 0x20) + -0x8;
      local_a.x = *piVar9;
      iStack6 = (iVar8 + 0x22) + *piVar9;
      iStack4 = (iVar8 + 0x20);
      rect = &local_a;
      hwnd = &DAT_1050_1050;
      erase = NULL;
  // TODO: goto LAB_1040_c19d;
    }
    local_a.x = 0;
    local_a.y = 0;
    iStack6 = 0;
    iStack4 = 0;
    GetClientRect16(&local_a,&DAT_1050_1050);
    uVar1 = (iVar8 + 0x6);
    local_a.x = (uVar1 + 0x1a);
    uVar1 = (iVar8 + 0x6);
    iStack6 = (uVar1 + 0x1e);
    iStack4 += -0x5;
    uVar1 = (iVar8 + 0x6);
    uVar3 = (iVar8 + 0x6);
    iVar6 = (uVar3 + 0x16);
    if (0x0 < iVar6) {
      uVar1 = (uVar1 + iVar6 * 0x4 + 0x26);
      iVar6 = uVar1;
      uVar4 = (uVar1 >> 0x10);
      local_a.y = (iVar6 + 0x20) + (iVar6 + 0x24);
    }
  }
  hwnd = (iVar8 + 0x4);
  erase = &local_a;
  rect = &DAT_1050_1050;//
// LAB_1040_c19d:
  InvalidateRect16(erase,rect,hwnd);
  return;
}
