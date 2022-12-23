
pub unsafe fn unk_win_ui_op_1038_d400
               (mut param_1: u16 ,param_2: *mut astruct_885,param_3: u8,param_4: u8,mut param_5: u16 ,mut param_6: u16 ,
               mut param_7: u32)

{
  let mut HVar1: HWND16;
  let mut iVar2: i16;
  let mut uVar2: u16;
  let mut BVar2: bool;
  let mut in_DX: u16;
  let mut in_register_0000000a: u16;
  let mut uVar4: u16;
  let mut puVar4: *mut u16;
  let mut puVar5: *mut u32;
  let mut LVar6: LRESULT;
  let mut pcVar7: *mut c_char;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbc: u16;
  let mut WVar8: WPARAM16;
  let mut UVar9: u16;
  let mut IVar10: i16;
  let mut uVar11: u16;
  let mut in_stack_0000ffe6: u16;
  let mut local_c: [u8;0x4] = [0;0x4];
  let mut WStack8: WPARAM16;
  let mut uStack6: u32;
  let mut paVar3: *mut Struct57;

  uStack6 = 0;
  WStack8 = 0;
  match param_7 {
  0x145 =>{
    HVar1 = GetDlgItem16(0x146,param_2.field6_0x6);
    uVar2 = EnableWindow16(0x1,HVar1);
    uStack6 = 0x13f0647;
    uVar11 = 0x1f1;}
// TODO: goto LAB_1038_d490;
  0x146 =>{
    uStack6 = 0x1400648;
    puVar4 = pass1_1008_941a(CONCAT22(0x1050,local_c),0x1,0xc4);
    puVar4 = (puVar4 >> 0x10);
    paVar3 = CONCAT22(in_register_0000000a,puVar4);
    win_1008_5c9e(local_c,puVar4,_u16_1050_02a0,CONCAT22(0x1050,local_c));
    uVar11 = 0x86;
    puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,0x860009,in_stack_0000fe8c,in_stack_0000ffb0,
                             in_stack_0000ffb6,in_stack_0000ffba);
    uVar4 = (paVar3 >> 0x10);
    pass1_1010_6604(puVar5,uVar11);
    HVar1 = GetDlgItem16(0x145,param_2.field6_0x6);
    EnableWindow16(0x0,HVar1);
    HVar1 = param_2.field6_0x6;
    UVar9 = 0xc;
    IVar10 = 0x13f;
    WVar8 = 0;
    pcVar7 = load_string_1010_847e(_u16_1050_14cc,0x649);
    LVar6 = SendDlgItemMessage16(pcVar7,WVar8,UVar9,IVar10,HVar1);
    paVar3 = CONCAT22(uVar4,(LVar6 >> 0x10));
    HVar1 = GetDlgItem16(0x146,param_2.field6_0x6);
    EnableWindow16(0x0,HVar1);
    iVar2 = pass1_1010_659a(puVar5,0x86);
    if (iVar2 == 0) {
      HVar1 = GetDlgItem16(0x14a,param_2.field6_0x6);
      uVar4 = (paVar3 >> 0x10);
      EnableWindow16(0x0,HVar1);
      HVar1 = param_2.field6_0x6;
      UVar9 = 0xc;
      IVar10 = 0x144;
      WVar8 = 0;
      pcVar7 = load_string_1010_847e(_u16_1050_14cc,0x531);
      LVar6 = SendDlgItemMessage16(pcVar7,WVar8,UVar9,IVar10,HVar1);
      paVar3 = CONCAT22(uVar4,(LVar6 >> 0x10));
    }
    puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x2),in_stack_0000fe8e,
                             in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
    if ((puVar5 + 0x20) != 0) {
      PostMessage16(0x0,0xaf,0x111,HWND16_1050_0396);
    }}
    // break;
  0x147 =>{
    HVar1 = GetDlgItem16(0x148,param_2.field6_0x6);
    uVar2 = EnableWindow16(0x1,HVar1);
    uStack6 = 0x1410647;
    uVar11 = 0x1f5;}
// TODO: goto LAB_1038_d490;
  0x148 =>{
    HVar1 = GetDlgItem16(0x149,param_2.field6_0x6);
    uVar2 = EnableWindow16(0x1,HVar1);
    uStack6 = 0x1420647;
    uVar11 = 0x1f2;//
// LAB_1038_d490:
    win_1008_5c5c(uVar2,param_1,_u16_1050_02a0,uVar11);}
    // break;
  0x149 =>{
    uStack6 = 0x1430648;
    PostMessage16(0x0,0xb8,0x111,HWND16_1050_0396);
    DestroyWindow16(param_2.field6_0x6);}
    // break;
  0x14a =>{
    HVar1 = GetDlgItem16(0x145,param_2.field6_0x6);
    EnableWindow16(0x1,HVar1);
    HVar1 = param_2.field6_0x6;
    UVar9 = 0xc;
    IVar10 = 0x140;
    WVar8 = 0;
    pcVar7 = load_string_1010_847e(_u16_1050_14cc,0x649);
    SendDlgItemMessage16(pcVar7,WVar8,UVar9,IVar10,HVar1);}
    // break;
  0x14b =>{
    HVar1 = GetDlgItem16(0x147,param_2.field6_0x6);
    EnableWindow16(0x1,HVar1);}
    // break;
  _ =>{
    post_win_msg_1040_7b3c(CONCAT22(CONCAT11(param_4,param_3),param_2),param_5,param_6,param_7);
    return;}
  };
  if (((uStack6 != 0) && (uStack6 != 0)) && (BVar2 = IsWindow16(param_2.field6_0x6), BVar2 != 0)) {
    HVar1 = param_2.field6_0x6;
    WVar8 = 0;
    UVar9 = 0xc;
    pcVar7 = load_string_1010_847e(_u16_1050_14cc,uStack6);
    SendDlgItemMessage16(pcVar7,WVar8,UVar9,uStack6,HVar1);
  }
  if (WStack8 != 0) {
    PostMessage16(0x0,WStack8,0x111,HWND16_1050_0396);
  }
  return;
}

pub unsafe fn pass1_1038_d6c4(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_d276(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn post_win_msg_1038_d840(param_1: *mut astruct_70,mut param_2: u16 )

{
  let mut iVar1: *mut astruct_70;
  let mut uVar1: *mut astruct_70;

  iVar1 = param_1;
  uVar1 = (param_1 >> 0x10);
  if (param_2 == 0x10) {
    if (iVar1.field142_0x8e != 0) {
      PostMessage16(0x0,iVar1.field142_0x8e,0x111,&iVar1.field_0x6);
      iVar1.field142_0x8e = 0;
      return;
    }
  }
  else if (param_2 < 0x11) {
    if (param_2 == '\x01') {
      iVar1.field143_0x90 = 0;
      iVar1.field144_0x92 = 0;
      return;
    }
    if (param_2 == '\x03') {
      pass1_1038_e03e(param_1);
      return;
    }
  }
  return;
}


pub unsafe fn FUN_1038_d8ae(param_1: *mut StructD,mut param_2: u16 ,struct_b_param_2: *mut StructB,mut param_4: u16 ,mut param_5: u16 )

{
  let mut pvVar1: LPVOID = null_mut();
  let mut uVar2: u32;
  let mut puVar3: *mut u32;
  let mut rect: *mut Struct57;
  let mut uVar4: u16;
  let mut paVar5: *mut Struct57;
   let mut struct_b_1: *mut StructB;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar9: u16;
  let mut uVar8: u16;
  let mut in_stack_0000fe24: u16;
  let mut in_stack_0000fe28: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000ff4e: u16;
  let mut in_stack_0000ff52: u16;
  let mut in_stack_0000ff56: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa6: u16;
  let mut local_26: u16;
  let mut uStack36: u16;
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut puStack30: *mut u16;
  let mut puStack14: *mut u32;
  let mut iStack10: i16;
  let mut uStack8: u16;
  let mut HStack6: HCURSOR16;
  let mut HStack4: HCURSOR16;

  HStack4 = LoadCursor16(0x7f02,0x0);
  HStack6 = SetCursor16(HStack4);
  dialog_ui_fn_1040_78e2(struct_b_param_2);
  uVar7 = (struct_b_param_2 >> 0x10);
  struct_b_1 = struct_b_param_2;
  uStack8 = pass1_1010_0886();
  if (_PTR_LOOP_1050_5f2c == 0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_1);
  }
  else {
    param_1 = (param_1 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
  }
  puStack30 = CONCAT22(param_1,PTR_LOOP_1050_5f2c);
  puVar3 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,param_1);
  struct_b_1[0x7].field5_0xa = puVar3;
  struct_b_1[0x7].field6_0xc = param_1;
//   for (iStack10 = 0x1; iStack10 <= uStack8; iStack10 += 1)
  for iStack10 in 1 .. uStack8
  {

    uVar2 = &struct_b_1[0x7].lpvoid_field_0x8;
    puStack30 = pass1_1010_08e2(uVar2,(uVar2 >> 0x10),iStack10);
    paVar5 = (param_1 & 0xffff0000 | puStack30 >> 0x10);
    local_26 = *puStack30;
    uStack36 = (puStack30 + 2);
    uStack34 = 0x1;
    uStack32 = 0x1;
    rect = &local_26;
    MapDialogRect16(rect,&DAT_1050_1050);
    mem_op_1000_179c(0x42,paVar5);
    uVar4 = paVar5 | rect;
    param_1 = (paVar5 & 0xffff0000 | uVar4);
    if (uVar4 == 0) {
      uVar2 = &struct_b_1[0x7].field5_0xa;
      (uVar2 + iStack10 * 0x4) = 0;
    }
    else {
      pvVar1 = struct_b_1.lpvoid_field_0x8;
      pass1_1008_3bd6(param_1,rect,paVar5,0x0,CONCAT22(local_26,uStack36),0x101,0xff0100,
                      CONCAT13((pvVar1 >> 0x8),CONCAT12(pvVar1,(puStack30 + 0x4)))
                      ,param_4,in_stack_0000fe24,in_stack_0000fe28,in_stack_0000ff4e,in_stack_0000ff52,in_stack_0000ff56
                     );
      uVar2 = &struct_b_1[0x7].field5_0xa;
      uVar8 = (uVar2 >> 0x10);
      iVar6 = uVar2;
      (iVar6 + iStack10 * 0x4) = rect;
      (iVar6 + iStack10 * 0x4 + 0x2) = param_1;
    }
    uVar2 = &struct_b_1[0x7].field5_0xa;
    uVar9 = (uVar2 >> 0x10);
    iVar6 = uVar2;
    if ((iVar6 + iStack10 * 0x4) != 0) {
      uVar2 = (iVar6 + iStack10 * 0x4);
      (uVar2 + 0x3e) = 0x1;
      uVar2 = &struct_b_1[0x7].field5_0xa;
      enable_win_1040_9234((uVar2 + iStack10 * 0x4),(puStack30 + 0x6));
    }
  }
  puStack14 = mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(param_5,0x2),in_stack_0000fe78
                              ,in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
  SetWindowText16((puStack14 + 0x68),struct_b_1.lpvoid_field_0x8);
  ShowWindow16(0x5,struct_b_1.lpvoid_field_0x8);
  SetCursor16(HStack6);
  return;
}


pub unsafe fn unk_win_sys_op_1038_da68(param_1: *mut StructD,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut in_BX: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut uVar8: u8;
  let mut iVar9: i16;
  let mut puStack14: *mut u32;
  let mut uStack8: u16;
  let mut iStack4: i16;

  paVar5 = CONCAT22(in_register_0000000a,param_1);
  uVar8 = (param_3 >> 0x8);
  if (param_4 == 0x204) {
    pass1_1038_de20(in_BX,param_1,CONCAT13(uVar8,CONCAT12(param_3,param_2)),0x204,param_5,
                    param_5);
    return;
  }
  iStack4 = 0;
  uStack8 = 0;
  if (param_5 == 0x121) {
    iStack4 = 0x6ec;
    uStack8 = 0x15;
// TODO: goto LAB_1038_dac3;
  }
  if (param_5 < 0x1220000) {
    uVar2 = param_5 - 0x100;
    if (uVar2 == 0) {
      param_5 = uVar2;
      if ((param_2 + 0x8e) == 0) {
        pass1_1010_1ea6(_u16_1050_02a0,CONCAT22(param_3,param_2));
        (param_2 + 0x90) = 0;
      }
      iStack4 = 0x72c;
      uStack8 = 0x48;
  // TODO: goto LAB_1038_dac3;
    }
    if (param_5 - 0x11c == 0) {
      param_5 = param_5 - 0x11c;
      pass1_1038_df86(param_1,CONCAT22(param_3,param_2));
  // TODO: goto LAB_1038_dac3;
    }
    if (param_5 == 0x11d) {
      uVar7 = pass1_1038_df5c(param_1,CONCAT22(param_3,param_2));
      paVar5 = (paVar5 & 0xffff0000 | uVar7 >> 0x10);
      param_5 = uVar7;
  // TODO: goto LAB_1038_dac3;
    }
    if (param_5 == 0x11e) {
      iVar9 = 0x1d;
    }
    else {
      if (param_5 != 0x120) {//
// LAB_1038_dc20:
        post_win_msg_1040_7b3c
                  (CONCAT13(uVar8,CONCAT12(param_3,param_2)),param_4,param_5,param_5);
        return;
      }
      iVar9 = 0x1c;
    }
  }
  else if (param_5 == 0x122) {
    iVar9 = 0x14;
  }
  else {
    if (param_5 != 0x123) {
      if (param_5 - 0x125 == 0) {
        ppcVar1 = (*_u16_1050_02a0 + 0x4);
        param_5 = param_5 - 0x125;
        (**ppcVar1)();
        (param_2 + 0x90) = 0x1;
        win_1008_5c5c(param_5,paVar5,_u16_1050_02a0,0x1db);
        (param_2 + 0x8e) = 0x100;
      }
      else {
        iVar9 = param_5 - 0x126;
        if (iVar9 == 0) {
          (param_2 + 0x8e) = 0;
          win_1008_5c7c(0x0,param_1,_u16_1050_02a0,0xcb0001);
          uVar3 = FUN_1010_830a(iVar9,paVar5,0x1008,_u16_1050_14cc,0x1f8);
          param_5 = WinHelp16(0x0,0x3,CONCAT22(paVar5,uVar3),(param_2 + 0x6));
        }
        else {
//          if (param_5 - 0x127 != 0) goto LAB_1038_dc20;
          param_5 = param_5 - 0x127;
          post_win_msg_1038_dcb0(0x0,paVar5,CONCAT22(param_3,param_2));
        }
      }
  // TODO: goto LAB_1038_dac3;
    }
    iVar9 = 0x28;
  }
  uVar7 = pass1_1038_af40(param_2,param_1,_PTR_LOOP_1050_5b7c,(param_2 + 0x8),iVar9);
  paVar5 = (paVar5 & 0xffff0000 | uVar7 >> 0x10);
  param_5 = uVar7;//
// LAB_1038_dac3:
  if (iStack4 != 0) {
    mem_op_1000_179c(0xb4,paVar5);
    uVar4 = paVar5 | param_5;
    uVar7 = paVar5 & 0xffff0000 | uVar4;
    if (uVar4 == 0) {
      uVar6 = 0x1000;
      iVar9 = 0;
      uVar3 = 0;
    }
    else {
      uVar6 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar9 = string_1040_8520(uVar7,
                                     CONCAT13((paVar5 >> 0x8),CONCAT12(paVar5,param_5)),
                               (param_2 + 0x6),0x20000,CONCAT22(iStack4,0x634));
      uVar3 = uVar7;
    }
    puStack14 = CONCAT22(uVar3,iVar9);
    if (uStack8 == 0) {
      ppcVar1 = (*puStack14 + 0x74);
      (**ppcVar1)(uVar6,iVar9,uVar3);
    }
    else {
      pass1_1008_941a(CONCAT22(0x1050,&stack0xffea),0x1,uStack8);
      ppcVar1 = (*puStack14 + 0x6c);
      (**ppcVar1)(0x1008,iVar9,uVar3,&stack0xffea,&DAT_1050_1050);
    }
  }
  return;
}
