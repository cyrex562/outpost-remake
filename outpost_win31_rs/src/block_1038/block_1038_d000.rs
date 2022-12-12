


call_win_proc_1038_d020: i32
               (base_param_6: u16,win_handle_1: HWND16,mut param_3: u32,l_param: HWND16,hwnd_param_4: HWND16,
               win_handle_2: HWND16)

{
  HANDLE16 handle_1;
  HANDLE16 handle_2;
  HANDLE16 handle_3;
  let mut var1: u16;
  let mut lresult: LRESULT;
  let mut var5: i32;
  let mut var6: *mut u32;
  let mut var7: i32;
  let mut var8: u16;
  let mut fn_ptr_1: *mut *mut code;

  handle_1 = GetProp16((LPCSTR)CONCAT22(base_param_6,0x5bd7),hwnd_param_4);
  handle_2 = GetProp16((LPCSTR)CONCAT13((base_param_6 >> 0x8),CONCAT12(base_param_6,0x5bd0)),
                       hwnd_param_4);
  var7 = CONCAT22(handle_1,handle_2);
  handle_1 = GetProp16((LPCSTR)CONCAT22(base_param_6,0x5be5),hwnd_param_4);
  handle_3 = GetProp16((LPCSTR)CONCAT22(base_param_6,0x5bde),hwnd_param_4);
  var6 = CONCAT22(handle_1,handle_3);
  if ((handle_1 | handle_3) != 0) {
    var5 = 0;
    if (l_param == 0x19) {
      fn_ptr_1 = (*var6 + 0x34);
      var5 = (**fn_ptr_1)(s_tile2_bmp_1050_1538,handle_3,handle_1,win_handle_1,param_3);
    }
    else {
      if (l_param == 0x86) {
        fn_ptr_1 = (*var6 + 0x20);
        var1 = (**fn_ptr_1)(s_tile2_bmp_1050_1538,handle_3,handle_1,param_3);
    // TODO: goto LAB_1038_d10e;
      }
      if ((l_param == 0x112) && ((param_3 & 0xfff0) == 0xf140)) {
        lresult = SendMessage16(0x0,0xf140,0x112,&HWND16_1050_0396);
        var1 = (lresult == 0);
    // TODO: goto LAB_1038_d10e;
      }
    }
    if (var5 != 0) {
      return var5;
    }
  }
  if (var7 != 0) {
    lresult = CallWindowProc16(CONCAT22(param_3,win_handle_1),param_3,l_param,hwnd_param_4,(LPVOID)handle_2);
    return lresult;
  }
  var1 = 0;//
// LAB_1038_d10e:
  return var1;
}



// WARNING: Unable to use type for symbol uVar2
pub unsafe fn win_prop_op_1038_d118(base_addr_param_4: u16,mut param_2: u32,mut param_3: u32,hwnd_param_3: HWND16)

{
  let mut uVar1: u32;
  let mut cVar2: u8;
  HANDLE16 HVar3;
  HANDLE16 HVar4;
  let mut puStack6: *mut u32;
  let mut uVar2: u32;
  let mut fn_ptr_1: *mut *mut code;

  HVar3 = GetProp16((LPCSTR)CONCAT22(base_addr_param_4,0x5bf3),hwnd_param_3);
  HVar4 = GetProp16((LPCSTR)CONCAT22(base_addr_param_4,0x5bec),hwnd_param_3);
  puStack6 = CONCAT22(HVar3,HVar4);
  if (param_3 == 0x30) {
    if ((HANDLE16)param_3 == 0) {
      return;
    }
    SetProp16((HANDLE16)param_3,CONCAT22(base_addr_param_4,0x5c06),hwnd_param_3);
    return;
  }
  if (param_3 < 0x310000) {
    cVar2 = (param_3 >> 0x10);
    if (cVar2 == '\x02') {
      if ((HVar3 | HVar4) != 0) {
        uVar1 = *puStack6;
        fn_ptr_1 = uVar1 + 0x6;
        (**fn_ptr_1)(s_tile2_bmp_1050_1538,HVar4,HVar3,param_2,param_3);
        if (puStack6.is_null() == false) {
          fn_ptr_1 = uVar1;
          (**fn_ptr_1)(s_tile2_bmp_1050_1538,HVar4,HVar3,1);
        }
      }
      HVar3 = GetProp16((LPCSTR)CONCAT22(base_addr_param_4,0x5bfa),hwnd_param_3);
      if (HVar3 == 0) {
        return;
      }
      DeleteObject16(HVar3);
      RemoveProp16((LPCSTR)CONCAT22(base_addr_param_4,0x5c00),hwnd_param_3);
      return;
    }
    if (cVar2 == '\x06') {
      if (((HANDLE16)param_3 != 1) && ((HANDLE16)param_3 != 0x2)) {
        uVar1 = &u16_1050_5bc8;
        (uVar1 + 0x8) = 0;
        return;
      }
      uVar2 = &u16_1050_5bc8;
      (uVar2 + 0x8) = hwnd_param_3;
      return;
    }
  }
  if ((HVar3 | HVar4) != 0) {
    fn_ptr_1 = (*puStack6 + 0xc);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,HVar4,HVar3,param_2,param_3);
  }
  return;
}



pub unsafe fn pass1_1038_d218(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  free_proc_inst_1038_cfda(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_d242(param_1: *mut astruct_57,mut param_2: u16 )

{
  let mut uVar1: *mut Struct57;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x13e,param_2);
  uVar1 = (param_1 >> 0x10);
  param_1.field0_0x0 = 0xd6ea;
  (param_1 + 0x2) = &u16_1050_1038;
  (param_1 + 0x74) = 0x1;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_d276(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xd6ea;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_ui_op_1038_d2a2(param_1: *mut astruct_57,StructB *struct_b_param_1,mut param_3: u16 )

{
  let mut rect: *mut Struct57;
  let mut iVar1: i16;
  let mut hwnd_2: HWND16;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut iVar5: *mut astruct_912;
  StructB *struct_b_6;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;
  let mut l_param: *mut c_char;
  let mut LVar8: LRESULT;
  let mut in_stack_0000fe2e: u16;
  let mut in_stack_0000fe32: u16;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ff58: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ff60: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut w_param: WPARAM16;
  let mut msg: u16;
  let mut id: i16;
  let mut uVar9: u16;
  LPVOID hwnd;
  let mut local_16: u16;
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut uStack10: u16;
  let mut uStack8: u32;
  let mut uStack4: u16;
  let mut paVar5: *mut Struct57;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uStack4 = 0x7;
  for (uStack10 = 0; struct_b_6 = (StructB *)struct_b_param_1, uVar6 = (struct_b_param_1 >> 0x10),
      uStack10 < uStack4; uStack10 += 1) {
    iVar5 = (uStack10 * 0xc);
    local_16 = (iVar5 + 0x5c0c);
    uStack20 = (iVar5 + 0x5c0e);
    uStack18 = 0x1;
    uStack16 = 0x1;
    rect = &local_16;
    MapDialogRect16(rect,&DAT_1050_1050);
    mem_op_1000_179c(0x42,param_1);
    uVar3 = param_1 | rect;
    paVar5 = (param_1 & 0xffff0000 | uVar3);
    if (uVar3 == 0) {
      rect = NULL;
      param_1 = (param_1 & 0xffff0000);
    }
    else {
      pass1_1008_3bd6(paVar5,rect,param_1,0x1,CONCAT22(local_16,uStack20),0x104,0x1020103,
                      CONCAT22(struct_b_6.lpvoid_field_0x8,(iVar5 + 0x5c10)),param_3,in_stack_0000fe2e,
                      in_stack_0000fe32,in_stack_0000ff58,in_stack_0000ff5c,in_stack_0000ff60);
      param_1 = paVar5;
    }
    uStack8 = CONCAT22(param_1,rect);
    if ((uStack10 * 0xc + 0x5c12) == 0) {
      EnableWindow16(0x0,&rect.field11_0x18);
    }
  }
  uVar9 = 0x86;
  puVar7 = mixed_1010_20ba(param_1,_u16_1050_0ed0,0x860009,in_stack_0000fe80,in_stack_0000ffa4,
                           in_stack_0000ffaa,in_stack_0000ffae);
  uVar4 = (puVar7 >> 0x10);
  uStack14 = puVar7;
  uStack12 = uVar4;
  iVar1 = pass1_1010_659a(puVar7,uVar9);
  if (iVar1 == 0) {
    hwnd_2 = GetDlgItem16(0x14a,struct_b_6.lpvoid_field_0x8);
    EnableWindow16(0x0,hwnd_2);
    hwnd = struct_b_6.lpvoid_field_0x8;
    msg = 0xc;
    id = 0x144;
    w_param = 0;
    l_param = load_string_1010_847e(_u16_1050_14cc,0x531);
    LVar8 = SendDlgItemMessage16(l_param,w_param,msg,id,hwnd);
    uVar4 = (LVar8 >> 0x10);
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  BVar2 = ShowWindow16(0x5,struct_b_6.lpvoid_field_0x8);
  win_1008_5c7c(BVar2,uVar4,_u16_1050_02a0,0x9a0001);
  (struct_b_6 + 0x7).field0_0x0 = BVar2;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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
  0x145 =>
    HVar1 = GetDlgItem16(0x146,param_2.field6_0x6);
    uVar2 = EnableWindow16(0x1,HVar1);
    uStack6 = 0x13f0647;
    uVar11 = 0x1f1;
// TODO: goto LAB_1038_d490;
  0x146 =>
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
    }
    break;
  0x147 =>
    HVar1 = GetDlgItem16(0x148,param_2.field6_0x6);
    uVar2 = EnableWindow16(0x1,HVar1);
    uStack6 = 0x1410647;
    uVar11 = 0x1f5;
// TODO: goto LAB_1038_d490;
  0x148 =>
    HVar1 = GetDlgItem16(0x149,param_2.field6_0x6);
    uVar2 = EnableWindow16(0x1,HVar1);
    uStack6 = 0x1420647;
    uVar11 = 0x1f2;//
// LAB_1038_d490:
    win_1008_5c5c(uVar2,param_1,_u16_1050_02a0,uVar11);
    break;
  0x149 =>
    uStack6 = 0x1430648;
    PostMessage16(0x0,0xb8,0x111,HWND16_1050_0396);
    DestroyWindow16(param_2.field6_0x6);
    break;
  0x14a =>
    HVar1 = GetDlgItem16(0x145,param_2.field6_0x6);
    EnableWindow16(0x1,HVar1);
    HVar1 = param_2.field6_0x6;
    UVar9 = 0xc;
    IVar10 = 0x140;
    WVar8 = 0;
    pcVar7 = load_string_1010_847e(_u16_1050_14cc,0x649);
    SendDlgItemMessage16(pcVar7,WVar8,UVar9,IVar10,HVar1);
    break;
  0x14b =>
    HVar1 = GetDlgItem16(0x147,param_2.field6_0x6);
    EnableWindow16(0x1,HVar1);
    break;
  _ =>
    post_win_msg_1040_7b3c((StructC *)CONCAT22(CONCAT11(param_4,param_3),param_2),param_5,param_6,param_7);
    return;
  }
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * pass1_1038_d756(param_1: *mut StructD,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut iVar2: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar2: *mut Struct57;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x11b,param_3);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  (iVar2 + 1).field0_0x0 = 0;
  iVar2[0x1].field1_0x2 = 0;
  iVar2[0x1].field2_0x4 = 0;
  iVar2[0x1].field4_0x8 = 0;
  param_2.field0_0x0 = 0xe0d4;
  iVar2.field1_0x2 = &u16_1050_1038;
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(unaff_BP,0x2b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar3;
  iVar2[0x1].field3_0x6 = (puVar3 >> 0x10);
  ppcVar1 = (*&iVar2[0x1].field2_0x4 + 0x4);
  (**ppcVar1)();
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_d7d0(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0xe0d4;
  iVar1.address_offset_field_0x2 = &u16_1050_1038;
  if (&iVar1.field_0x90 != 0) {
    pass1_1010_1ea6(_u16_1050_02a0,param_1);
  }
  if (&iVar1.field_0x92 != 0) {
    pass1_1010_1ea6(&iVar1.field_0x92,param_1);
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1.field_0x6);
  fn_ptr_1000_17ce(*&iVar1.field_0x96);
  ui_cleanup_op_1040_782c(param_1);
  return;
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn FUN_1038_d8ae(param_1: *mut StructD,mut param_2: u16 ,StructB *struct_b_param_2,mut param_4: u16 ,mut param_5: u16 )

{
  LPVOID pvVar1;
  let mut uVar2: u32;
  let mut puVar3: *mut u32;
  let mut rect: *mut Struct57;
  let mut uVar4: u16;
  let mut paVar5: *mut Struct57;
  StructB *struct_b_1;
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
  struct_b_1 = (StructB *)struct_b_param_2;
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
  for (iStack10 = 0x1; iStack10 <= uStack8; iStack10 += 1) {
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
      *(astruct_57 **)(iVar6 + iStack10 * 0x4) = rect;
      (iVar6 + iStack10 * 0x4 + 0x2) = param_1;
    }
    uVar2 = &struct_b_1[0x7].field5_0xa;
    uVar9 = (uVar2 >> 0x10);
    iVar6 = uVar2;
    if ((iVar6 + iStack10 * 0x4) != 0) {
      uVar2 = (iVar6 + iStack10 * 0x4);
      (uVar2 + 0x3e) = 0x1;
      uVar2 = &struct_b_1[0x7].field5_0xa;
      enable_win_1040_9234((uVar2 + iStack10 * 0x4),*(BOOL16 *)(puStack30 + 0x6));
    }
  }
  puStack14 = mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(param_5,0x2),in_stack_0000fe78
                              ,in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
  SetWindowText16((puStack14 + 0x68),struct_b_1.lpvoid_field_0x8);
  ShowWindow16(0x5,struct_b_1.lpvoid_field_0x8);
  SetCursor16(HStack6);
  return;
}



// WARNING: Removing unreachable block (ram,0x1038dad3)
// WARNING: Removing unreachable block (ram,0x1038daea)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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
                  ((StructC *)CONCAT13(uVar8,CONCAT12(param_3,param_2)),param_4,param_5,param_5);
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn post_win_msg_1038_dcb0(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut paVar7: *mut Struct57;
  let mut uVar8: u32;
  let mut puVar9: *mut u16;
  let mut paVar10: *mut astruct_67;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u8;
  let mut uVar15: u8;
  let mut local_18: u16;
  let mut uStack22: u16;
  let mut local_14: [u8;0x4] = [0;0x4];
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut local_a: [u8;0x4] = [0;0x4];
  let mut puStack6: *mut u32;

  mem_op_1000_179c(0xb4,param_2);
  uStack14 = param_2;
  uVar8 = param_2 & 0xffff0000;
  uVar6 = uVar8 | (uStack14 | param_1);
  iVar4 = param_3;
  uVar5 = (param_3 >> 0x10);
  uStack16 = param_1;
  if ((uStack14 | param_1) == 0) {
    iVar2 = 0;
  }
  else {
    iVar2 = string_1040_8520(uVar6,CONCAT22(uStack14,param_1),(iVar4 + 0x6),0x30004,0x7260634);
    uVar8 = uVar6;
  }
  puStack6 = CONCAT22(uVar8,iVar2);
  puVar9 = pass1_1008_941a(CONCAT22(0x1050,local_a),0x1,0x49);
  paVar7 = (uVar8 & 0xffff0000 | puVar9 >> 0x10);
  ppcVar1 = (*puStack6 + 0x6c);
  uVar3 = (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),local_a,&DAT_1050_1050);
  uStack12 = uVar3;
  if (uVar3 == 0x6) {
    mem_op_1000_179c(0xb4,paVar7);
    uStack14 = paVar7;
    uVar8 = paVar7 & 0xffff0000;
    uVar6 = uVar8 | (uStack14 | uVar3);
    uStack16 = uVar3;
    if ((uStack14 | uVar3) == 0) {
      iVar4 = 0;
    }
    else {
      iVar4 = string_1040_8520(uVar6,CONCAT13((paVar7 >> 0x8),CONCAT12(paVar7,uVar3)),
                               (iVar4 + 0x6),0x20000,0x7280634);
      uVar8 = uVar6;
    }
    puStack6 = CONCAT22(uVar8,iVar4);
    puVar9 = pass1_1008_941a(CONCAT22(0x1050,local_14),0x1,0x4a);
    paVar7 = (uVar8 & 0xffff0000 | puVar9 >> 0x10);
    ppcVar1 = (*puStack6 + 0x6c);
    (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),local_14,&DAT_1050_1050);
    uVar14 = 0;
    uVar15 = 0;
    iVar2 = 0x15;
    uVar12 = 0x1;
    uVar13 = 0;
    uVar11 = 0;
    iVar4 = 0;
    uVar5 = 0;
    paVar10 =
              mixed_1010_20ba(paVar7,_u16_1050_0ed0,0x37,in_stack_0000fe72,in_stack_0000ff96,
                              in_stack_0000ff9c,in_stack_0000ffa0);
    uStack22 = (paVar10 >> 0x10);
    local_18 = SUB42(paVar10,0x0);
    post_win_msg_1008_a0e4(paVar10,CONCAT22(uVar11,uVar5),iVar4,uVar12,CONCAT13(uVar15,CONCAT12(uVar14,uVar13)),iVar2);
    PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
    return;
  }
  mem_op_1000_179c(0xb4,paVar7);
  uStack14 = paVar7;
  uVar8 = paVar7 & 0xffff0000 | (uStack14 | uVar3);
  uStack16 = uVar3;
  if ((uStack14 | uVar3) == 0) {
    iVar4 = 0;
    uVar5 = 0;
  }
  else {
    iVar4 = string_1040_8520(uVar8,CONCAT13((paVar7 >> 0x8),CONCAT12(paVar7,uVar3)),
                             (iVar4 + 0x6),0x20000,0x7290634);
    uVar5 = uVar8;
  }
  puStack6 = CONCAT22(uVar5,iVar4);
  pass1_1008_941a(CONCAT22(0x1050,&local_18),0x1,0x4b);
  ppcVar1 = (*puStack6 + 0x6c);
  (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),&local_18,&DAT_1050_1050);
  return;
}
pub unsafe fn pass1_1038_de20(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: i16)

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut local_12: [u8;0x4] = [0;0x4];
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut puStack10: *mut u32;
  let mut uStack6: u16;
  let mut iStack4: i16;

  paVar4 = CONCAT22(in_register_0000000a,param_2);
  iStack4 = 0x644;
  uStack6 = 0;
  match param_6 + -0x11c {
  0x0 =>
    iStack4 = 0x635;
    uStack6 = 0x3a;
    break;
  0x1 =>
    iStack4 = 0x636;
    uStack6 = 0x3b;
    break;
  0x2 =>
    iStack4 = 0x637;
    uStack6 = 0x3c;
    break;
  0x4 =>
    iStack4 = 0x639;
    uStack6 = 0x3e;
    break;
  0x5 =>
    iStack4 = 0x63a;
    uStack6 = 0x3f;
    break;
  0x6 =>
    iStack4 = 0x63b;
    uStack6 = 0x40;
    break;
  0x7 =>
    iStack4 = 0x640;
    uStack6 = 0x45;
    break;
  0x9 =>
    iStack4 = 0x642;
    uStack6 = 0x47;
    break;
  0xa =>
    iStack4 = 0x641;
    uStack6 = 0x46;
    break;
  0xb =>
    iStack4 = 0x63f;
    uStack6 = 0x44;
  }
  if (iStack4 != 0) {
    uVar6 = 0x1000;
    mem_op_1000_179c(0xb4,paVar4);
    uStack12 = paVar4;
    uVar5 = paVar4 & 0xffff0000 | (uStack12 | param_1);
    uStack14 = param_1;
    if ((uStack12 | param_1) == 0) {
      iVar2 = 0;
      uVar3 = 0;
    }
    else {
      uVar6 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar2 = string_1040_8520(uVar5,CONCAT22(uStack12,param_1),(param_3 + 0x6),0x20000,
                               CONCAT22(iStack4,0x634));
      uVar3 = uVar5;
    }
    puStack10 = CONCAT22(uVar3,iVar2);
    if (uStack6 == 0) {
      ppcVar1 = (*puStack10 + 0x74);
      (**ppcVar1)(uVar6,iVar2,uVar3);
    }
    else {
      pass1_1008_941a(CONCAT22(0x1050,local_12),0x1,uStack6);
      ppcVar1 = (*puStack10 + 0x6c);
      (**ppcVar1)(0x1008,puStack10,(puStack10 >> 0x10),local_12,&DAT_1050_1050);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1038_df5c(mut param_1: u16 ,mut param_2: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  uVar2 = (param_2 >> 0x10);
  uVar1 = param_2;
  pass1_1010_038e(*(astruct_27 **)(uVar1 + 0x92),1);
  uVar3 = pass1_1038_af40(uVar1,param_1,_PTR_LOOP_1050_5b7c,(uVar1 + 0x8),0x20);
  return uVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_df86(param_1: *mut StructD,mut param_2: u32)

{
  let mut pcVar1: *mut c_char;
  let mut ppcVar2: *mut *mut code;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut uVar7: u32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut puVar11: *mut u32;
  let mut pcVar12: *mut c_char;
  let mut paVar13: *mut Struct57;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffe6: u16;
  let mut puStack22: *mut u32;

  paVar13 = CONCAT22(in_register_0000000a,param_1);
  puVar11 = mixed_1010_20ba(paVar13,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x2),in_stack_0000fe8e,
                            in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar7 = paVar13 & 0xffff0000 | puVar11 >> 0x10;
  pcVar1 = *(puVar11 + 0x68);
  uVar9 = (param_2 >> 0x10);
  uVar8 = param_2;
  BVar3 = pass1_1010_041a();
  if (BVar3 != 0) {
    pass1_1010_038e(*(astruct_27 **)(uVar8 + 0x92),1);
    pass1_1038_af40(uVar8,uVar7,_PTR_LOOP_1050_5b7c,(uVar8 + 0x8),0x1e);
    return;
  }
  pcVar12 = load_string_1010_847e(_u16_1050_14cc,0x7d5);
  paVar13 = (uVar7 & 0xffff0000 | pcVar12 >> 0x10);
  uVar4 = pcVar12;
  uVar10 = 0x1000;
  mem_op_1000_179c(0xb4,paVar13);
  uVar5 = paVar13 | uVar4;
  if (uVar5 == 0) {
    uVar9 = 0;
    uVar6 = 0;
  }
  else {
    uVar10 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar13 = pass1_1040_8478(uVar5,CONCAT22(paVar13,uVar4),0x20,pcVar1,pcVar12,
                              (uVar8 + 0x6));
    uVar6 = (paVar13 >> 0x10);
    uVar9 = SUB42(paVar13,0x0);
  }
  puStack22 = CONCAT22(uVar6,uVar9);
  ppcVar2 = (*puStack22 + 0x74);
  (**ppcVar2)(uVar10,uVar9,uVar6);
  return;
}
