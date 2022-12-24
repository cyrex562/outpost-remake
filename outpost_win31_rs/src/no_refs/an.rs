pub unsafe fn pass1_1038_8d98(param_1: *mut u8,param_2: *mut Struct903,mut param_3: u16 ,mut param_4: u32)

{
  if (param_4 == 0xeb) {
    send_dlg_item_msg_1038_8f74(param_2);
  }
  else {
    if (param_4 != s_vrpal_bmp_1050_183a + 0x7) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4);
      return;
    }
    msg_box_op_1038_8dda(0x0,param_1,param_2);
  }
  return;
}

pub unsafe fn pass1_1038_90a2(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_8cf6(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn pass1_1038_927c(param_1: u32)

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = (*param_1 + 0x74);
  (**ppcVar1)();
  return;
}


pub unsafe fn draw_op_1038_92f6(param_1: *mut u8,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut pSVar5: *mut StructD;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  let mut paVar9: *mut Struct57;
  let mut unaff_SI: u16;
  let mut uVar10: u16;
  let mut in_stack_0000fe88: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb6: u16;
   let mut hfile_param: *mut HFILE16;
  let mut local_1a: [BOOL16;0x2] = [false;2];
  let mut UStack22: u16;
  let mut pSStack20: *mut StructD;
  let mut pSStack16: *mut StructD;
  let mut iStack12: i16;
  let mut pSStack10: *mut StructD;
  let mut paStack6: *mut astruct_20;
  let mut paVar8: *mut Struct57;

  paVar7 = CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0xeb) {
    paStack6 =
               mixed_1010_20ba(paVar7,_u16_1050_0ed0,CONCAT22(unaff_SI,0x3),in_stack_0000fe88,
                               in_stack_0000ffac,in_stack_0000ffb2,in_stack_0000ffb6);
    paVar7 = (paVar7 & 0xffff0000 | paStack6 >> 0x10);
    pSVar5 = (param_2 + 0x90);
    if (pSVar5.is_null() == false) {
      pSStack10 = pSVar5;
      mem_op_1000_179c(0x18,paVar7);
      uVar3 = pSVar5;
      pSStack16 = (pSVar5 & 0xffff | paVar7 << 0x10);
      uVar6 = paVar7 | uVar3;
      paVar9 = (paVar7 & 0xffff0000);
      paVar8 = (paVar9 | uVar6);
      if (uVar6 == 0) {
        uVar3 = 0;
      }
      else {
        struct_1040_a598((pSVar5 & 0xffff | paVar7 << 0x10));
        paVar9 = paVar8;
      }
      (param_2 + 0x90) = uVar3;
      (param_2 + 0x92) = paVar9;
      (param_2 + 0x90) = 0x11;
      iStack12 = *(param_2 + 0x90);
      uVar3 = iStack12 * 0xa + 2;
      mem_op_1000_179c(uVar3,paVar9);
      uVar6 = paVar9;
      pSStack16 = CONCAT22(uVar6,uVar3);
      if ((uVar6 | uVar3) == 0) {
        uVar2 = (param_2 + 0x90);
        (uVar2 + 0x2) = 0;
      }
      else {
        pSStack16 = iStack12;
        pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iStack12,0xa,uVar3 + 0x2,uVar6);
        uVar2 = (param_2 + 0x90);
        uVar10 = (uVar2 >> 0x10);
        iVar4 = uVar2;
        (iVar4 + 0x2) = uVar3 + 2;
        (iVar4 + 0x4) = uVar6;
      }
      uVar10 = (pSStack10 >> 0x10);
      uVar2 = (param_2 + 0x90);
      (uVar2 + 0x6) = (pSStack10 + 0x6);
      uVar2 = (param_2 + 0x90);
      (uVar2 + 0xa) = (pSStack10 + 0xa);
      uVar2 = (param_2 + 0x90);
      (uVar2 + 0x12) = (param_2 + 0xa);
      uVar10 = 0x1010;
      pass1_1010_a50c(paStack6,&u32_1050_5b42,(param_2 + 0x90));
      pSStack20 = pSStack10;
      pSStack16 = pSStack10;
      if (pSStack10.is_null() == false) {
        pass1_1040_a5d0(pSStack10);
        uVar10 = 0x1000;
        fn_ptr_1000_17ce(pSStack10);
      }
      ppcVar1 = (CONCAT22(param_3,param_2) + 0x70);
      (**ppcVar1)(uVar10,param_2,param_3);
    }
  }
  else {
    if (param_5 != 0xf9) {
      pass1_1040_b54a(param_1,CONCAT13((param_3 >> 0x8),CONCAT12(param_3,param_2)),param_4,
                      param_5);
      return;
    }
    iVar4 = pass1_1038_993a(param_2,param_3,param_4);
    if (-0x1 < iVar4) {
      hfile_param = (param_2 + 0x6);
      UStack22 = GetDlgItemInt16(0x1,local_1a,&DAT_1050_1050,(iVar4 * 0xe + 0x5a72));
      if (local_1a[0] != 0) {
        FUN_1010_2a32((param_2 + 0x98),CONCAT22((iVar4 * 0xe + 0x5a72),UStack22),hfile_param,
                      unaff_SI);
      }
    }
  }
  return;
}


pub unsafe fn win_ui_op_1038_977a(param_1: *mut Struct57,mut param_2: i16,mut param_3: u16 ,mut param_4: i16)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar6: u16;
  let mut local_10: [u8;0x4] = [0;0x4];
  let mut puStack12: *mut u32;
  let mut iStack8: i16;
  let mut uStack6: u16;
  let mut local_4: bool;
  let mut uVar5: u32;

  iStack8 = 0;
  uVar6 = (param_2 + 0x6);
  uVar2 = GetDlgItemInt16(0x1,&local_4,&DAT_1050_1050,0xfa8);
  uStack6 = uVar2;
  if (uVar2 != 0) {
    mem_op_1000_179c(0xb4,param_1);
    uVar3 = param_1 | uVar2;
    uVar5 = param_1 & 0xffff0000 | uVar3;
    if (uVar3 == 0) {
      iVar2 = 0;
      uVar4 = 0;
    }
    else {
      iVar2 = string_1040_8520(uVar5,CONCAT22(param_1,uVar2),(param_2 + 0x6),0x20041,
                               0x5da05db);
      uVar4 = uVar5;
    }
    puStack12 = CONCAT22(uVar4,iVar2);
    pass1_1008_941a(CONCAT22(0x1050,local_10),0x1,0xc3);
    ppcVar1 = (*puStack12 + 0x6c);
    iStack8 = (**ppcVar1)(0x1008,puStack12,(puStack12 >> 0x10),local_10,&DAT_1050_1050,uVar6,uVar2
                         );
  }
  if ((iStack8 == 1) || (uStack6 == 0)) {
    destroy_window_1040_b726(CONCAT22(param_3,param_2),param_4);
  }
  return;
}

pub unsafe fn pass1_1038_997c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  let mut in_stack_0000ffda: u16;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1038_9ad0(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_9a48(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1038_9ed4(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  let mut in_stack_0000ffda: u16;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn FUN_1038_a08c()

{
  return;
}

pub unsafe fn pass1_1038_a090(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_9fa4(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn pass1_1038_a174(mut param_1: u32,mut param_2: i16)

{
  if (param_2 == 1) {
    (param_1 + 0x8e) = 0;
  }
  return;
}

pub unsafe fn FUN_1038_a2a4() -> u16

{
  return 0x0;
}

pub unsafe fn pass1_1038_a2aa(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a156(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn show_win_1038_a396(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut StructB)

{
  let mut in_register_0000000a: u16;
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut in_stack_0000ffaa: u16;

  uVar1 = CONCAT22(in_register_0000000a,param_2);
  dialog_ui_fn_1040_78e2(param_3);
  unk_win_ui_op_1038_a18c(uVar1,param_3,in_stack_0000ffaa);
  win_1008_5c7c(param_1,uVar1,_u16_1050_02a0,0x10001);
  uVar2 = (param_3 >> 0x10);
  (param_3 + 0x8c) = param_1;
  ShowWindow16(0x5,(param_3 + 0x6));
  return;
}

pub unsafe fn pass1_1038_a402(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a36a(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1038_a608(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_a4c2(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
