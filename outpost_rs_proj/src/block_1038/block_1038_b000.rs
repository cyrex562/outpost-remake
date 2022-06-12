
pub fn show_win_1038_b634(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uStack4: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0xac) == 0x0) {
    (iVar2 + 0xac) = 0x1;
    for (uStack4 = 0x1; uStack4 < 0x2b; uStack4 += 0x1) {
      if (((uStack4 * 0x4 + iVar2 + 0x2) | (uStack4 * 0x4 + iVar2)) != 0x0) {
        uVar1 = (u32)(uStack4 * 0x4 + iVar2);
        ShowWindow16(0x0,*(HWND16 *)((int)uVar1 + 0x6));
      }
    }
  }
  return;
}
pub fn show_win_1038_b68a(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uStack4: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0xac) != 0x0) {
    (iVar2 + 0xac) = 0x0;
    for (uStack4 = 0x1; uStack4 < 0x2b; uStack4 += 0x1) {
      if (((uStack4 * 0x4 + iVar2 + 0x2) | (uStack4 * 0x4 + iVar2)) != 0x0) {
        uVar1 = (u32)(uStack4 * 0x4 + iVar2);
        ShowWindow16(0x1,*(HWND16 *)((int)uVar1 + 0x6));
      }
    }
  }
  return;
}
pub fn pass1_1038_b6e0(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uStack4: u16;

  uStack4 = 0x1;
  while( true ) {
    if (0x2a < uStack4) {
      return;
    }
    uVar3 = (param_1 >> 0x10);
    iVar2 = (int)param_1;
    if ((((uStack4 * 0x4 + iVar2 + 0x2) | (uStack4 * 0x4 + iVar2)) != 0x0) &&
       (uVar1 = (u32)(uStack4 * 0x4 + iVar2), ((int)uVar1 + 0x6) == param_2)) break;
    uStack4 += 0x1;
  }
  (u32)(uStack4 * 0x4 + iVar2) = 0x0;
  return;
}



BOOL16 bring_win_to_top_1038_b72e(mut param_1: u32,mut param_2: i16)

{
  HWND16 hwnd;
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)(param_2 * 0x4 + (int)param_1) != 0x0) {
    uVar1 = (u32)(param_2 * 0x4 + (int)param_1);
    hwnd = *(HWND16 *)((int)uVar1 + 0x6);
    SetFocus16(hwnd);
    BringWindowToTop16(hwnd);
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_b772(u8 *param_1,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar2;
  let mut unaff_BP: u16;
  astruct_57 *uVar2;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;
  u8 **ppuVar3;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x9a,0x0,0xfbf,param_3);
  uVar2 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar2 = (astruct_57 *)param_2;
  (u32)(iVar2 + 0x1) = 0x0;
  (u32)&iVar2[0x1].field2_0x4 = 0x0;
  iVar2[0x1].field4_0x8 = 0x1;
  iVar2[0x1].field5_0xa = 0x0;
  param_2->field0_0x0 = 0xbd70;
  iVar2->field1_0x2 = &u16_1050_1038;
  ppuVar3 = (u8 **)CONCAT22(unaff_BP,0x36);
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,ppuVar3,in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  (iVar2 + 0x1)->field0_0x0 = puVar2;
  iVar2[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  puVar2 = mixed_1010_20ba((astruct_57 *)((u32)paVar1 & 0xffff0000 | (u32)puVar2 >> 0x10),_u16_1050_0ed0,
                           (u8 **)CONCAT22((int)((u32)ppuVar3 >> 0x10),0x6),in_stack_0000fea6,in_stack_0000ffca
                           ,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar2;
  iVar2[0x1].field3_0x6 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_b7f0(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xbd70;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_b81c(mut param_1: u16 ,StructB *struct_b_param_1)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  code **ppcVar3;
  let mut uVar4: u16;
  HWND16 HVar5;
  astruct_909 *win_enabled;
  let mut extraout_DX: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar7;
  let mut uVar9: u32;
  StructB *struct_b_8;
  let mut unaff_SI: u16;
  let mut uVar7: u16;
  u32 *puVar10;
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

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  puVar10 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x6),in_stack_0000fe8e,
                            in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar9 = (u32)puVar10 >> 0x10;
  uVar7 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_8 = (StructB *)struct_b_param_1;
  struct_b_8[0x7].lpvoid_field_0x8 = (LPVOID)puVar10;
  struct_b_8[0x7].max_count_field_0x10 = ((u32)puVar10 >> 0x10);
  uVar1 = (u32)&struct_b_8[0x7].lpvoid_field_0x8;
  uVar4 = (int)uVar1 + 0x4e;
  uVar1 &= 0xffff0000;
  piVar6 = (uVar1 | uVar4);
  iStack10 = 0x0;
  for (iStack12 = 0x1a0; extraout_DX = uVar9, iStack12 < 0x1b5; iStack12 += 0x1) {
    if ((iStack10 * 0x2 + uVar4) == iStack12) {
      iStack10 += 0x1;
    }
    else {
      CheckDlgButton16(0x2,iStack12,(HWND16)struct_b_8->lpvoid_field_0x8);
    }
  }
  HVar5 = GetDlgItem16(0xfb1,(HWND16)struct_b_8->lpvoid_field_0x8);
  win_enabled = (astruct_909 *)EnableWindow16(0x0,HVar5);
  uVar2 = (u32)&struct_b_8[0x7].lpvoid_field_0x8;
  ppcVar3 = (code **)((int)*(u32*)&struct_b_8[0x7].lpvoid_field_0x8 + 0x10);
  (**ppcVar3)((int)s_tile2_bmp_1050_1538,(int)uVar2,(int)((u32)uVar2 >> 0x10));
  piStack16 = CONCAT22(extraout_DX,win_enabled);
  move_win_1040_826c(struct_b_param_1,win_enabled->field1_0x2 + -0x2,win_enabled->field2_0x4 + *piStack16 + 0x3);
  ShowWindow16(0x5,(HWND16)struct_b_8->lpvoid_field_0x8);
  pass1_1018_1c9a(*(astruct_263 **)&struct_b_8[0x7].lpvoid_field_0x8,*piVar6);
  HVar5 = GetDlgItem16(*piVar6,(HWND16)struct_b_8->lpvoid_field_0x8);
  SetFocus16(HVar5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1038_b922(StructD *param_1,StructC *param_2,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ) -> u32

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut UVar3: u16;
  HWND16 HVar4;
  let mut BVar5: bool;
  let mut uVar6: u16;
  let mut uVar7: u16;
  StructC *iVar8;
  StructC *uVar8;
  let mut uVar9: u16;
  LRESULT LVar10;
  char *pcVar11;
  astruct_57 *paVar12;
  let mut in_stack_0000fa38: u16;
  let mut in_stack_0000fb5c: u16;
  let mut in_stack_0000fb62: u16;
  let mut in_stack_0000fb66: u16;
  u8 uVar13;
  WORD *pWVar14;
  let mut uVar15: u16;
  u32 *puStack1128;
  char local_464 [0x50];
  WORD local_414 [0x200];
  let mut puStack20: *mut u16;
  u8 *puStack16;
  u32 *puStack14;
  let mut uStack10: u16;
  HWND16 HStack8;
  let mut BStack6: bool;
  let mut uStack4: u16;

  BStack6 = 0x0;
  uStack4 = 0x0;
  iVar8 = (StructC *)param_2;
  uVar8 = (StructC *)((u32)param_2 >> 0x10);
  if (param_4 < 0x1b5) {
    if (param_4 < 0x1a0) {
      if (param_4 != 0x2) goto LAB_1038_bbbf;
    }
    else {
      HStack8 = GetDlgItem16(param_4,iVar8->field6_0x6);
      LVar10 = SendMessage16(0x0,0x0,0x400,HStack8);
      uStack10 = LVar10;
      if (uStack10 == 0x2) {
        BStack6 = 0x0;
        uStack4 = 0x0;
        goto LAB_1038_bc26;
      }
      SendMessage16(0x0,(uStack10 == 0x0),0x401,HStack8);
      UVar3 = IsDlgButtonChecked(param_4,iVar8->field6_0x6);
      if (UVar3 == 0x0) {
        piVar1 = &iVar8->field_0x96;
        *piVar1 = *piVar1 + 0x1;
        if (&iVar8->field_0x96 == 0x1) {
          HVar4 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
          EnableWindow16(0x0,HVar4);
        }
      }
      else {
        piVar1 = &iVar8->field_0x96;
        *piVar1 = *piVar1 + -0x1;
        HVar4 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
        BVar5 = IsWindowEnabled16(HVar4);
        if (BVar5 == 0x0) {
          HVar4 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
          EnableWindow16(0x1,HVar4);
        }
        if (&iVar8->field_0x96 < 0x0) {
          CheckDlgButton16(0x0,iVar8->field145_0x98,iVar8->field6_0x6);
          &iVar8->field_0x96 = 0x0;
        }
        iVar8->field145_0x98 = param_4;
        pass1_1018_1c9a((astruct_263 *)iVar8->field142_0x92,param_4);
        puStack14 = (u32 *)pass1_1018_1e78(iVar8->field142_0x92,-0x1);
        uVar6 = ((u32)puStack14 >> 0x10);
        uVar7 = uVar6 | puStack14;
        if (uVar7 == 0x0) {
          puStack16 = NULL;
        }
        else {
          puStack16 = (puStack14 + 0x1c);
        }
        win_1008_5c7c(puStack16,uVar7,_u16_1050_02a0,CONCAT22(puStack16,0x1));
      }
    }
    BStack6 = 0x1;
    uStack4 = 0x0;
  }
  else {
    if (param_4 == 0xfb1) {
      for (uVar6 = 0x1a0; uVar6 < 0x1b5; uVar6 += 0x1) {
        UVar3 = IsDlgButtonChecked(uVar6,iVar8->field6_0x6);
        if (UVar3 == 0x1) {
          pass1_1008_d818(iVar8->field141_0x8e,uVar6);
          goto LAB_1038_bba2;
        }
      }
    }
    else {
      if (param_4 != 0xfbe) goto LAB_1038_bbbf;
      puStack14 = mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_5,0x2),
                                  in_stack_0000fa38,in_stack_0000fb5c,in_stack_0000fb62,in_stack_0000fb66);
      uVar9 = ((u32)param_1 >> 0x10);
      puStack16 = PTR_LOOP_1050_13ae;
      if (PTR_LOOP_1050_13ae == ((int)&PTR_LOOP_1050_0000 + 0x1)) {
        puStack16 = &u16_1050_0002;
      }
      uStack10 = ((int)puStack16 * 0xc + 0x5b84) - 0x1;
      pass1_1008_612e(uStack10,0x0,uStack10);
      puStack20 = (u16 *)pass1_1018_1e78(iVar8->field142_0x92,(((int)puStack16 * 0x6 + uStack10) * 0x2 + 0x5b86)
                                        );
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x50,local_464,(short)&DAT_1050_1050);
      pcVar11 = load_string_1010_847e(_u16_1050_14cc,*puStack20);
      uVar15 = ((u32)pcVar11 >> 0x10);
      paVar12 = (astruct_57 *)CONCAT22(uVar9,uVar15);
      uVar13 = SUB21(local_464,0x0);
      uVar6 = wsprintf16(local_414,(char *)0x5bc01050,
                         (char *)CONCAT13((char)(local_464 >> 0x8),CONCAT12(uVar13,0x1050)),uVar13,
                         (int)&DAT_1050_1050,(int)pcVar11,uVar15);
      uVar9 = 0x1000;
      mem_op_1000_179c(0xb4,paVar12);
      uVar7 = paVar12;
      if ((uVar7 | uVar6) == 0x0) {
        uVar6 = 0x0;
        paVar12 = NULL;
      }
      else {
        pWVar14 = local_414;
        uVar15 = SUB42(&DAT_1050_1050,0x0);
        HVar4 = HWND16_1050_0396;
        pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x57b);
        uVar9 = SUB42(&PTR_LOOP_1050_1040,0x0);
        paVar12 = pass1_1040_8478(((u32)pcVar11 >> 0x10),(astruct_57 *)CONCAT22(uVar7,uVar6),0x41,pcVar11,
                                  (char *)CONCAT22(uVar15,pWVar14),HVar4);
        uVar6 = paVar12;
      }
      param_1 = (StructD *)((u32)paVar12 >> 0x10);
      puStack1128 = (u32 *)((u32)paVar12 & 0xffff0000 | (u32)uVar6);
      ppcVar2 = (code **)((int)*puStack1128 + 0x74);
      HStack8 = (**ppcVar2)(uVar9,uVar6,(int)((u32)paVar12 >> 0x10));
      if (HStack8 != 0x1) goto LAB_1038_bc26;
      pass1_1008_d818(iVar8->field141_0x8e,((int)puStack20 + 0x1a));//
LAB_1038_bba2:
      win_ui_cursor_op_1038_bc30(param_2);
    }
    PostMessage16(0x0,0xce,0x111,HWND16_1050_0396);
    param_4 = 0x1;//
LAB_1038_bbbf:
    uVar9 = SUB42(param_1,0x0);
    BStack6 = post_win_msg_1040_7b3c(param_2,param_3,(param_3 >> 0x10),param_4);
    uStack4 = uVar9;
  }//
LAB_1038_bc26:
  return CONCAT22(uStack4,BStack6);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_cursor_op_1038_bc30(StructC *param_1)

{
  let mut uVar1: u32;
  let mut local_112: u16;
  let mut uStack272: u16;
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;

  HStack4 = LoadCursor16((char *)0x7f02,0x0);
  HStack6 = SetCursor16(HStack4);
  uVar1 = (u32)((int)param_1 + 0x8e);
  pass1_1030_532e((astruct_97 *)CONCAT22(0x1050,&local_112),(long)((int)uVar1 + 0xe) + 0x1000000);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_112));
  pass1_1030_838e((u32 *)_u16_1050_5748);
  local_112 = 0x389a;
  uStack272 = 0x1008;
  pass1_1030_8334();
  SetCursor16(HStack6);
  return;
}
pub fn pass1_1038_bca8(mut param_1: u32)

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
  puVar6 = (u32 *)(u32)(iVar11 + 0xa);
  paVar9 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)(iVar11 + 0xc));
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



StructD * pass1_1038_bd4a(StructD *param_1,param_2: u8)

{
  pass1_1038_b7f0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_bddc(StructD *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x176,param_6);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  iVar1[0x1].field3_0x6 = 0x0;
  iVar1[0x1].field4_0x8 = 0x0;
  iVar1[0x1].field5_0xa = 0x0;
  iVar1[0x1].field6_0xc = 0x0;
  iVar1[0x1].field7_0xe = 0x0;
  param_2->field0_0x0 = 0xc436;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_be4a(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xc436;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_be76(u8 *param_1,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_register_0000000a: u16;
  astruct_27 *paVar1;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  if (param_3 == 0x0) {
    iVar2 = 0x0;
    paVar1 = (astruct_27 *)
             mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,(u8 **)0x2b,
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1010_038e(paVar1,iVar2);
  }
  destroy_win_1040_7b98(CONCAT22((int)param_3,param_2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_dlg_op_1038_bea4(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  HWND16 HVar3;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  let mut iVar5: i16;
  let mut uVar6: u16;
  u32 *puVar7;
  let mut uVar8: u32;
  char *lparam;
  LRESULT LVar9;
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

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puStack6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fed2,0x2),in_stack_0000fd7a,
                             in_stack_0000fe9e,in_stack_0000fea4,in_stack_0000fea8);
  paVar4 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)puStack6 >> 0x10);
  uStack10 = (u32)((int)puStack6 + 0x68);
  uVar6 = (param_2 >> 0x10);
  iVar5 = (int)param_2;
  GetWindowText16(0x80,CONCAT22(0x1050,local_8c),*(HWND16 *)(iVar5 + 0x6));
  wsprintf16(local_10e,(char *)CONCAT22(local_8c,0x1050),(char *)CONCAT22((int)uStack10,0x1050),
             (int)((u32)uStack10 >> 0x10));
  SetWindowText16(CONCAT22(0x1050,local_10e),*(HWND16 *)(iVar5 + 0x6));
  HVar3 = GetDlgItem16(0x179,*(HWND16 *)(iVar5 + 0x6));
  *(HWND16 *)(iVar5 + 0x92) = HVar3;
  pass1_1008_e3ec(*(astruct_218 **)(iVar5 + 0x8e),(u32 *)CONCAT22(0x1050,&local_116),
                  (u32 *)CONCAT22(0x1050,&local_112));
  send_msg_1038_c374(param_2,local_112,(iVar5 + 0x92));
  puVar7 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fed4,0x2f),in_stack_0000fd7c,
                           in_stack_0000fea0,in_stack_0000fea6,in_stack_0000feaa);
  uVar2 = ((u32)puVar7 >> 0x10);
  uVar8 = (u32)((int)puVar7 + 0x24);
  uVar1 = (u32)(iVar5 + 0x8e);
  uVar8 = string_1008_e586(uVar1,((u32)uVar1 >> 0x10),uVar8,uVar8,uVar2);
  SendMessage16(uVar8,0xffff,0x40d,*(HWND16 *)(iVar5 + 0x92));
  HVar3 = GetDlgItem16(0x17a,*(HWND16 *)(iVar5 + 0x6));
  *(HWND16 *)(iVar5 + 0x94) = HVar3;
  send_msg_1038_c374(param_2,local_116,HVar3);
  lparam = load_string_1010_847e(_u16_1050_14cc,0x531);
  LVar9 = SendMessage16((LPARAM)lparam,0x0,0x403,*(HWND16 *)(iVar5 + 0x94));
  (iVar5 + 0x9c) = (int)LVar9;
  SendMessage16((LPARAM)lparam,0xffff,0x40d,*(HWND16 *)(iVar5 + 0x94));
  HVar3 = GetDlgItem16(0x178,*(HWND16 *)(iVar5 + 0x6));
  *(HWND16 *)(iVar5 + 0x96) = HVar3;
  HVar3 = GetDlgItem16(0x177,*(HWND16 *)(iVar5 + 0x6));
  *(HWND16 *)(iVar5 + 0x98) = HVar3;
  HVar3 = GetDlgItem16(0x184,*(HWND16 *)(iVar5 + 0x6));
  *(HWND16 *)(iVar5 + 0x9a) = HVar3;
  return;
}
