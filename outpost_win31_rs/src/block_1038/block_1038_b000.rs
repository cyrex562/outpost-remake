
pub unsafe fn show_win_1038_b634(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uStack4: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0xac) == 0) {
    (iVar2 + 0xac) = 0x1;
    for (uStack4 = 0x1; uStack4 < 0x2b; uStack4 += 1) {
      if (((uStack4 * 0x4 + iVar2 + 0x2) | (uStack4 * 0x4 + iVar2)) != 0) {
        uVar1 = (uStack4 * 0x4 + iVar2);
        ShowWindow16(0x0,(uVar1 + 0x6));
      }
    }
  }
  return;
}
pub unsafe fn show_win_1038_b68a(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uStack4: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0xac) != 0) {
    (iVar2 + 0xac) = 0;
    for (uStack4 = 0x1; uStack4 < 0x2b; uStack4 += 1) {
      if (((uStack4 * 0x4 + iVar2 + 0x2) | (uStack4 * 0x4 + iVar2)) != 0) {
        uVar1 = (uStack4 * 0x4 + iVar2);
        ShowWindow16(0x1,(uVar1 + 0x6));
      }
    }
  }
  return;
}
pub unsafe fn pass1_1038_b6e0(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uStack4: u16;

  uStack4 = 0x1;
  loop {
    if (0x2a < uStack4) {
      return;
    }
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((((uStack4 * 0x4 + iVar2 + 0x2) | (uStack4 * 0x4 + iVar2)) != 0) &&
       (uVar1 = (uStack4 * 0x4 + iVar2), (uVar1 + 0x6) == param_2)) { break; }
    uStack4 += 0x1;
  }
  (uStack4 * 0x4 + iVar2) = 0;
  return;
}



pub unsafe fn bring_win_to_top_1038_b72e(mut param_1: u32,mut param_2: i16) -> BOOL16

{
  let mut hwnd: HWND16;
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if ((param_2 * 0x4 + param_1) != 0) {
    uVar1 = (param_2 * 0x4 + param_1);
    hwnd = (uVar1 + 0x6);
    SetFocus16(hwnd);
    BringWindowToTop16(hwnd);
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_b772(param_1: *mut u8,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar2: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar2: *mut Struct57;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;
  ppuVar3: *mut *mut u8;

  paVar1 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x9a,0x0,0xfbf,param_3);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  (iVar2 + 1) = 0;
  iVar2[0x1].field2_0x4 = 0;
  iVar2[0x1].field4_0x8 = 0x1;
  iVar2[0x1].field5_0xa = 0;
  param_2.field0_0x0 = 0xbd70;
  iVar2.field1_0x2 = &u16_1050_1038;
  ppuVar3 = CONCAT22(unaff_BP,0x36);
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,ppuVar3,in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  (iVar2 + 1).field0_0x0 = puVar2;
  iVar2[0x1].field1_0x2 = (puVar2 >> 0x10);
  puVar2 = mixed_1010_20ba((paVar1 & 0xffff0000 | puVar2 >> 0x10),_u16_1050_0ed0,
                           CONCAT22((ppuVar3 >> 0x10),0x6),in_stack_0000fea6,in_stack_0000ffca
                           ,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar2;
  iVar2[0x1].field3_0x6 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_b7f0(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xbd70;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_ui_op_1038_b81c(mut param_1: u16 ,StructB *struct_b_param_1)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut ppcVar3: *mut *mut code;
  let mut uVar4: u16;
  let mut HVar5: HWND16;
  win_enabled: *mut astruct_909;
  let mut extraout_DX: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  let mut uVar9: u32;
  StructB *struct_b_8;
  let mut unaff_SI: u16;
  let mut uVar7: u16;
  let mut puVar10: *mut u32;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut piStack16: *mut i16;
  let mut iStack12: i16;
  let mut iStack10: i16;
  StructB *iVar7;
  let mut uVar8: u16;
  let mut piVar6: *mut i16;

  paVar7 = CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  puVar10 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,CONCAT22(unaff_SI,0x6),in_stack_0000fe8e,
                            in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar9 = puVar10 >> 0x10;
  uVar7 = (struct_b_param_1 >> 0x10);
  struct_b_8 = (StructB *)struct_b_param_1;
  struct_b_8[0x7].lpvoid_field_0x8 = (LPVOID)puVar10;
  struct_b_8[0x7].max_count_field_0x10 = (puVar10 >> 0x10);
  uVar1 = &struct_b_8[0x7].lpvoid_field_0x8;
  uVar4 = uVar1 + 0x4e;
  uVar1 &= 0xffff0000;
  piVar6 = (uVar1 | uVar4);
  iStack10 = 0;
  for (iStack12 = 0x1a0; extraout_DX = uVar9, iStack12 < 0x1b5; iStack12 += 1) {
    if ((iStack10 * 0x2 + uVar4) == iStack12) {
      iStack10 += 0x1;
    }
    else {
      CheckDlgButton16(0x2,iStack12,struct_b_8.lpvoid_field_0x8);
    }
  }
  HVar5 = GetDlgItem16(0xfb1,struct_b_8.lpvoid_field_0x8);
  win_enabled = EnableWindow16(0x0,HVar5);
  uVar2 = &struct_b_8[0x7].lpvoid_field_0x8;
  ppcVar3 = (*&struct_b_8[0x7].lpvoid_field_0x8 + 0x10);
  (**ppcVar3)(s_tile2_bmp_1050_1538,uVar2,(uVar2 >> 0x10));
  piStack16 = CONCAT22(extraout_DX,win_enabled);
  move_win_1040_826c(struct_b_param_1,win_enabled.field1_0x2 + -0x2,win_enabled.field2_0x4 + *piStack16 + 0x3);
  ShowWindow16(0x5,struct_b_8.lpvoid_field_0x8);
  pass1_1018_1c9a(*(astruct_263 **)&struct_b_8[0x7].lpvoid_field_0x8,*piVar6);
  HVar5 = GetDlgItem16(*piVar6,struct_b_8.lpvoid_field_0x8);
  SetFocus16(HVar5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn win_ui_op_1038_b922(param_1: *mut StructD,StructC *param_2,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ) -> u32

{
  let mut piVar1: *mut i16;
  let mut ppcVar2: *mut *mut code;
  let mut UVar3: u16;
  let mut HVar4: HWND16;
  let mut BVar5: bool;
  let mut uVar6: u16;
  let mut uVar7: u16;
  StructC *iVar8;
  StructC *uVar8;
  let mut uVar9: u16;
  let mut LVar10: LRESULT;
  let mut pcVar11: *mut c_char;
  let mut paVar12: *mut Struct57;
  let mut in_stack_0000fa38: u16;
  let mut in_stack_0000fb5c: u16;
  let mut in_stack_0000fb62: u16;
  let mut in_stack_0000fb66: u16;
  let mut uVar13: u8;
  WORD *pWVar14;
  let mut uVar15: u16;
  let mut puStack1128: *mut u32;
  let mut local_464: [u8;0x50] = [0;0x50];
  let mut local_414: [u16;0x200] = [0;0x200];
  let mut puStack20: *mut u16;
  let mut puStack16: *mut u8;
  let mut puStack14: *mut u32;
  let mut uStack10: u16;
  let mut HStack8: HWND16;
  let mut BStack6: bool;
  let mut uStack4: u16;

  BStack6 = 0;
  uStack4 = 0;
  iVar8 = (StructC *)param_2;
  uVar8 = (StructC *)(param_2 >> 0x10);
  if (param_4 < 0x1b5) {
    if (param_4 < 0x1a0) {
//      if (param_4 != 0x2) goto LAB_1038_bbbf;
    }
    else {
      HStack8 = GetDlgItem16(param_4,iVar8.field6_0x6);
      LVar10 = SendMessage16(0x0,0x0,0x400,HStack8);
      uStack10 = LVar10;
      if (uStack10 == 0x2) {
        BStack6 = 0;
        uStack4 = 0;
    // TODO: goto LAB_1038_bc26;
      }
      SendMessage16(0x0,(uStack10 == 0),0x401,HStack8);
      UVar3 = IsDlgButtonChecked(param_4,iVar8.field6_0x6);
      if (UVar3 == 0) {
        piVar1 = &iVar8.field_0x96;
        *piVar1 = *piVar1 + 1;
        if (&iVar8.field_0x96 == 1) {
          HVar4 = GetDlgItem16(0xfb1,iVar8.field6_0x6);
          EnableWindow16(0x0,HVar4);
        }
      }
      else {
        piVar1 = &iVar8.field_0x96;
        *piVar1 = *piVar1 + -0x1;
        HVar4 = GetDlgItem16(0xfb1,iVar8.field6_0x6);
        BVar5 = IsWindowEnabled16(HVar4);
        if (BVar5 == 0) {
          HVar4 = GetDlgItem16(0xfb1,iVar8.field6_0x6);
          EnableWindow16(0x1,HVar4);
        }
        if (&iVar8.field_0x96 < 0x0) {
          CheckDlgButton16(0x0,iVar8.field145_0x98,iVar8.field6_0x6);
          iVar8.field_0x96 = 0;
        }
        iVar8.field145_0x98 = param_4;
        pass1_1018_1c9a(iVar8.field142_0x92,param_4);
        puStack14 = pass1_1018_1e78(iVar8.field142_0x92,-1);
        uVar6 = (puStack14 >> 0x10);
        uVar7 = uVar6 | puStack14;
        if (uVar7 == 0) {
          puStack16 = NULL;
        }
        else {
          puStack16 = (puStack14 + 0x1c);
        }
        win_1008_5c7c(puStack16,uVar7,_u16_1050_02a0,CONCAT22(puStack16,1));
      }
    }
    BStack6 = 0x1;
    uStack4 = 0;
  }
  else {
    if (param_4 == 0xfb1) {
      for (uVar6 = 0x1a0; uVar6 < 0x1b5; uVar6 += 1) {
        UVar3 = IsDlgButtonChecked(uVar6,iVar8.field6_0x6);
        if (UVar3 == 1) {
          pass1_1008_d818(iVar8.field141_0x8e,uVar6);
      // TODO: goto LAB_1038_bba2;
        }
      }
    }
    else {
//      if (param_4 != 0xfbe) goto LAB_1038_bbbf;
      puStack14 = mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(param_5,0x2),
                                  in_stack_0000fa38,in_stack_0000fb5c,in_stack_0000fb62,in_stack_0000fb66);
      uVar9 = (param_1 >> 0x10);
      puStack16 = PTR_LOOP_1050_13ae;
      if (PTR_LOOP_1050_13ae == (&PTR_LOOP_1050_0000 + 1)) {
        puStack16 = &u16_1050_0002;
      }
      uStack10 = (puStack16 * 0xc + 0x5b84) - 0x1;
      pass1_1008_612e(uStack10,0x0,uStack10);
      puStack20 = pass1_1018_1e78(iVar8.field142_0x92,((puStack16 * 0x6 + uStack10) * 0x2 + 0x5b86)
                                        );
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x50,local_464,&DAT_1050_1050);
      pcVar11 = load_string_1010_847e(_u16_1050_14cc,*puStack20);
      uVar15 = (pcVar11 >> 0x10);
      paVar12 = CONCAT22(uVar9,uVar15);
      uVar13 = SUB21(local_464,0x0);
      uVar6 = wsprintf16(local_414,0x5bc01050,
                         CONCAT13((local_464 >> 0x8),CONCAT12(uVar13,0x1050)),uVar13,
                         &DAT_1050_1050,pcVar11,uVar15);
      uVar9 = 0x1000;
      mem_op_1000_179c(0xb4,paVar12);
      uVar7 = paVar12;
      if ((uVar7 | uVar6) == 0) {
        uVar6 = 0;
        paVar12 = NULL;
      }
      else {
        pWVar14 = local_414;
        uVar15 = SUB42(&DAT_1050_1050,0x0);
        HVar4 = HWND16_1050_0396;
        pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x57b);
        uVar9 = SUB42(&PTR_LOOP_1050_1040,0x0);
        paVar12 = pass1_1040_8478((pcVar11 >> 0x10),CONCAT22(uVar7,uVar6),0x41,pcVar11,
                                  CONCAT22(uVar15,pWVar14),HVar4);
        uVar6 = paVar12;
      }
      param_1 = (paVar12 >> 0x10);
      puStack1128 = (paVar12 & 0xffff0000 | uVar6);
      ppcVar2 = (*puStack1128 + 0x74);
      HStack8 = (**ppcVar2)(uVar9,uVar6,(paVar12 >> 0x10));
//      if (HStack8 != 1) goto LAB_1038_bc26;
      pass1_1008_d818(iVar8.field141_0x8e,(puStack20 + 0x1a));//
// LAB_1038_bba2:
      win_ui_cursor_op_1038_bc30(param_2);
    }
    PostMessage16(0x0,0xce,0x111,HWND16_1050_0396);
    param_4 = 0x1;//
// LAB_1038_bbbf:
    uVar9 = SUB42(param_1,0x0);
    BStack6 = post_win_msg_1040_7b3c(param_2,param_3,(param_3 >> 0x10),param_4);
    uStack4 = uVar9;
  }//
// LAB_1038_bc26:
  return CONCAT22(uStack4,BStack6);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_ui_cursor_op_1038_bc30(StructC *param_1)

{
  let mut uVar1: u32;
  let mut local_112: u16;
  let mut uStack272: u16;
  let mut HStack6: HCURSOR16;
  let mut HStack4: HCURSOR16;

  HStack4 = LoadCursor16(0x7f02,0x0);
  HStack6 = SetCursor16(HStack4);
  uVar1 = (param_1 + 0x8e);
  pass1_1030_532e(CONCAT22(0x1050,&local_112),(uVar1 + 0xe) + 0x1000000);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_112));
  pass1_1030_838e(_u16_1050_5748);
  local_112 = 0x389a;
  uStack272 = 0x1008;
  pass1_1030_8334();
  SetCursor16(HStack6);
  return;
}
pub unsafe fn pass1_1038_bca8(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut uVar4: u32;
  let mut paVar5: *mut astruct_394;
  let mut puVar6: *mut u32;
  let mut uVar7: u16;
  let mut puVar8: *mut u8;
  let mut in_EDX: u32;
  let mut paVar9: *mut Struct57;
  let mut iVar10: i16;
  let mut iVar11: i16;
  let mut uVar12: u16;
  let mut uVar13: u16;

  uVar12 = (param_1 >> 0x10);
  iVar10 = param_1;
  uVar3 = (iVar10 + 0x8e);
  uVar13 = (uVar3 >> 0x10);
  iVar11 = uVar3;
  puVar6 = (iVar11 + 0xa);
  paVar9 = (in_EDX & 0xffff0000 | (iVar11 + 0xc));
  ppcVar2 = (*puVar6 + 0x14);
  (**ppcVar2)();
  paVar5 = puVar6;
  uVar4 = paVar9 << 0x10;
  if ((iVar10 + 0x70) != 0) {
    paVar5 = *(astruct_394 **)(iVar10 + 0x70);
    uVar1 = (iVar10 + 0x72);
    uVar7 = uVar1 | paVar5;
    paVar9 = (paVar9 & 0xffff0000 | uVar7);
    if (uVar7 != 0) {
      ppcVar2 = paVar5;
      (**ppcVar2)();
    }
  }
  mem_op_1000_179c(0x14,paVar9);
  puVar8 = (paVar9 | paVar5);
  if (puVar8.is_null()) {
    paVar5 = NULL;
    puVar8 = NULL;
  }
  else {
    struct_1008_4c58(paVar5);
  }
  *(astruct_394 **)(iVar10 + 0x70) = paVar5;
  (iVar10 + 0x72) = puVar8;
  pass1_1008_4d84(puVar8,(iVar10 + 0x70),puVar6 & 0xffff | uVar4);
  return;
}



pub unsafe fn pass1_1038_bd4a(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_b7f0(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_bddc(param_1: *mut StructD,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar1: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar1: *mut Struct57;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x176,param_6);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  iVar1[0x1].field2_0x4 = 0;
  iVar1[0x1].field3_0x6 = 0;
  iVar1[0x1].field4_0x8 = 0;
  iVar1[0x1].field5_0xa = 0;
  iVar1[0x1].field6_0xc = 0;
  iVar1[0x1].field7_0xe = 0;
  param_2.field0_0x0 = 0xc436;
  iVar1.field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x3b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_be4a(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xc436;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_be76(param_1: *mut u8,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut astruct_27;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  if (param_3 == 0) {
    iVar2 = 0;
    paVar1 =
             mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,0x2b,
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1010_038e(paVar1,iVar2);
  }
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_dlg_op_1038_bea4(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut HVar3: HWND16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;
  let mut uVar8: u32;
  let mut lparam: *mut c_char;
  let mut LVar9: LRESULT;
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
  let mut local_116: *mut u32;
  let mut local_112: *mut u32;
  let mut local_10e: [u16;0x41] = [0;0x41];
  let mut local_8c: [u8;0x82] = [0;0x82];
  let mut uStack10: u32;
  let mut puStack6: *mut u32;

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  puStack6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(in_stack_0000fed2,0x2),in_stack_0000fd7a,
                             in_stack_0000fe9e,in_stack_0000fea4,in_stack_0000fea8);
  paVar4 = (paVar4 & 0xffff0000 | puStack6 >> 0x10);
  uStack10 = (puStack6 + 0x68);
  uVar6 = (param_2 >> 0x10);
  iVar5 = param_2;
  GetWindowText16(0x80,CONCAT22(0x1050,local_8c),(iVar5 + 0x6));
  wsprintf16(local_10e,CONCAT22(local_8c,0x1050),CONCAT22(uStack10,0x1050),
             (uStack10 >> 0x10));
  SetWindowText16(CONCAT22(0x1050,local_10e),(iVar5 + 0x6));
  HVar3 = GetDlgItem16(0x179,(iVar5 + 0x6));
  (iVar5 + 0x92) = HVar3;
  pass1_1008_e3ec(*(astruct_218 **)(iVar5 + 0x8e),CONCAT22(0x1050,&local_116),
                  CONCAT22(0x1050,&local_112));
  send_msg_1038_c374(param_2,local_112,(iVar5 + 0x92));
  puVar7 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(in_stack_0000fed4,0x2f),in_stack_0000fd7c,
                           in_stack_0000fea0,in_stack_0000fea6,in_stack_0000feaa);
  uVar2 = (puVar7 >> 0x10);
  uVar8 = (puVar7 + 0x24);
  uVar1 = (iVar5 + 0x8e);
  uVar8 = string_1008_e586(uVar1,(uVar1 >> 0x10),uVar8,uVar8,uVar2);
  SendMessage16(uVar8,0xffff,0x40d,(iVar5 + 0x92));
  HVar3 = GetDlgItem16(0x17a,(iVar5 + 0x6));
  (iVar5 + 0x94) = HVar3;
  send_msg_1038_c374(param_2,local_116,HVar3);
  lparam = load_string_1010_847e(_u16_1050_14cc,0x531);
  LVar9 = SendMessage16(lparam,0x0,0x403,(iVar5 + 0x94));
  (iVar5 + 0x9c) = LVar9;
  SendMessage16(lparam,0xffff,0x40d,(iVar5 + 0x94));
  HVar3 = GetDlgItem16(0x178,(iVar5 + 0x6));
  (iVar5 + 0x96) = HVar3;
  HVar3 = GetDlgItem16(0x177,(iVar5 + 0x6));
  (iVar5 + 0x98) = HVar3;
  HVar3 = GetDlgItem16(0x184,(iVar5 + 0x6));
  (iVar5 + 0x9a) = HVar3;
  return;
}
