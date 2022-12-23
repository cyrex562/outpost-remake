
pub unsafe fn struct_1040_b082(param_1: *mut Struct57,mut param_2: u32)

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
pub unsafe fn pass1_1040_b0bc(param_1: *mut Struct57,mut param_2: u32,mut param_3: u32)

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





pub unsafe fn unk_draw_op_1040_b0f8(mut param_1: u16 ,param_2: *mut StructD)

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







pub unsafe fn unk_win_ui_op_1040_b230(mut param_1: u16 ,param_2: *mut StructB)

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
    PTR_LOOP_1050_5ef8 = null_mut();
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




pub unsafe fn pass1_1040_b45e(mut param_1: u32)

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
    // for (iStack8 = 0; piVar2 = (iVar3 + 0x90), *piVar2 != iStack8 && iStack8 <= *piVar2; iStack8 += 1)
    iStack8 = 0;
    piVar2 = iVar3 + 0x90;
    while *piVar2 != iStack8 && iStack8 <= *piVar2
    {
      uVar1 = (puStack6 + 2);
      SetDlgItemText16(CONCAT22(uVar1,*puStack6),(uVar1 >> 0x10),(iVar3 + 0x6));
      puStack6 = (puStack6 & 0xffff0000 | (puStack6 + 0xa));
      iStack8 += 1;
    }
  }
  return;
}







pub unsafe fn pass1_1040_b54a(param_1: *mut u8,param_2: *mut Struct903,mut param_3: u16 ,mut param_4: u32)

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
  let mut iVar6: *mut astruct_515;
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
        post_win_msg_1040_7b3c(param_2,param_3,param_4,param_4);
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
pub unsafe fn destroy_window_1040_b726(mut param_1: u32,mut param_2: i16)

{
  let mut fn_ptr_1: *mut *mut code;

  if (param_2 != 0) {
    fn_ptr_1 = (param_1 + 0x78);
    (**fn_ptr_1)();
  }
  DestroyWindow16((param_1 + 0x6));
  return;
}



pub unsafe fn pass1_1040_b7ee(param_1: *mut Struct57,param_2: i32,mut param_3: u16 )

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





// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol puVar4
// WARNING: Unable to use type for symbol uVar18
// WARNING: Unable to use type for symbol uVar19




pub unsafe fn struct_1040_bf3e(param_1: *mut astruct_442,mut param_2: u16 ) -> *mut u16

{
  let mut iVar1: *mut astruct_442;
  let mut uVar1: *mut astruct_442;

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
pub unsafe fn pass1_1040_bf92(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0xc53e;
  iVar1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  pass1_1010_1ea6(&iVar1.field_0x6,(param_1 & 0xffff | uVar1 << 0x10));
  unk_destroy_win_op_1010_2fa0(&iVar1.field_0x6);
  param_1.address_offset_field_0x0 = 0x3ab0;
  iVar1.address_offset_field_0x2 = 0x1008;
  param_1.address_offset_field_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}
pub unsafe fn pass1_1040_bfde(param_1: *mut c_void,param_2: *mut u32)

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
