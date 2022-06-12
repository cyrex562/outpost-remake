pub fn win_ui_op_1040_0000(param_1: *mut astruct_57,StructB *struct_b_param_1,mut param_3: u16 )

{
  astruct_57 *rect;
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut unaff_SI: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  u32 *puVar8;
  let mut uVar9: u32;
  let mut in_stack_0000fe16: u16;
  let mut in_stack_0000fe1a: u16;
  let mut in_stack_0000fe6a: u16;
  let mut in_stack_0000ff40: u16;
  let mut in_stack_0000ff44: u16;
  let mut in_stack_0000ff48: u16;
  let mut in_stack_0000ff8e: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff98: u16;
  let mut local_24: u16;
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut local_1a: u16;
  let mut uStack24: u16;
  let mut uStack22: u16;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut iStack12: i16;
  astruct_915 *paStack10;
  astruct_57 *paStack8;
  let mut uStack6: u16;
  let mut iStack4: i16;
  let mut iVar1: u16;
  let mut uVar4: u32;

    // Segment:    9
    // Offset:     0006f820
    // Length:     d974
    // Min Alloc:  d974
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  iStack4 = 0x8;
  for (paStack10 = NULL; uVar5 = struct_b_param_1, uVar6 = ((u32)struct_b_param_1 >> 0x10),
      (int)paStack10 < iStack4; paStack10 = paStack10 + 0x1) {
    iVar1 = (int)paStack10 * 0xe;
    local_24 = (iVar1 + 0x5c60);
    uStack34 = (iVar1 + 0x5c62);
    uStack32 = 0x1;
    uStack30 = 0x1;
    rect = (astruct_57 *)&local_24;
    MapDialogRect16((RECT16 *)rect,(HWND16)&DAT_1050_1050);
    mem_op_1000_179c(0x42,param_1);
    uVar2 = (astruct_57 *)param_1 | rect;
    uVar4 = (u32)param_1 & 0xffff0000 | (u32)uVar2;
    if (uVar2 == 0x0) {
      rect = NULL;
      uVar4 = (u32)param_1 & 0xffff0000;
    }
    else {
      pass1_1008_3bd6(uVar4,rect,(astruct_57 *)param_1,0x1,CONCAT22(local_24,uStack34),0x104,0x1020103,
                      CONCAT22((uVar5 + 0x6),(iVar1 + 0x5c64)),param_3,in_stack_0000fe16,
                      in_stack_0000fe1a,in_stack_0000ff40,in_stack_0000ff44,in_stack_0000ff48);
    }
    uStack6 = uVar4;
    paStack8 = rect;
    uVar7 = win_ui_op_1040_0558(struct_b_param_1,paStack10);
    param_1 = (astruct_57 *)(uVar4 & 0xffff0000 | uVar7 >> 0x10);
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  puVar8 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x48),in_stack_0000fe6a,
                           in_stack_0000ff8e,in_stack_0000ff94,in_stack_0000ff98);
  uStack16 = ((u32)puVar8 >> 0x10);
  uStack18 = puVar8;
  iStack12 = (uStack18 + 0xa);
  uStack14 = (uStack18 + 0xc);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_1a),*(HWND16 *)(uVar5 + 0x6));
  uVar3 = iStack12 >> 0xf;
  uStack28 = uStack22 - local_1a;
  local_1a = (iStack12 / 0x2 - uStack28) - 0x3;
  if ((int)local_1a < 0x0) {
    local_1a = 0x0;
  }
  SetWindowPos16(0x41,0x0,0x0,uStack24,local_1a,0x0,*(HWND16 *)(uVar5 + 0x6));
  uVar9 = pass1_1038_af40(uVar5,uVar3,_PTR_LOOP_1050_5b7c,(uVar5 + 0x6),0x17);
  uVar3 = ((u32)uVar9 >> 0x10);
  uVar1 = uVar9;
  (uVar5 + 0x96) = uVar1;
  (uVar5 + 0x98) = uVar3;
  win_1008_5c7c(uVar1,uVar3,_u16_1050_02a0,0x9e0001);
  (uVar5 + 0x8c) = uVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_0170(undefined1 param_1,mut param_2: u16 ,struct *param_3,mut param_4: u16 ,mut param_5: u16 ,mut param_6: i16)

{
  let mut iVar1: i16;
  HWND16 hwnd_1;
  let mut BVar2: bool;
  astruct_915 *paVar3;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  let mut unaff_SI: u16;
  let mut uVar6: u32;
  LRESULT LVar7;
  u32 *puVar8;
  char *l_param;
  let mut uVar9: u32;
  let mut in_stack_0000fd7c: u16;
  let mut in_stack_0000fd86: u16;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000feaa: u16;
  let mut in_stack_0000feb0: u16;
  let mut in_stack_0000feb4: u16;
  HCURSOR16 *pHVar10;
  let mut uVar11: u16;
  u8 uVar12;
  u8 uVar13;
  let mut uVar14: u16;
  WPARAM16 w_param;
  let mut msg: u16;
  INT16 id;
  let mut in_stack_0000fedc: u32;
  let mut uVar15: u32;
  HCURSOR16 local_18;
  let mut local_16: u16;
  astruct_598 *paStack20;
  astruct_915 *paStack16;
  let mut uStack14: u16;
  u32 *puStack12;
  astruct_915 *paStack8;
  WPARAM16 WStack6;
  let mut iStack4: i16;
  astruct_890 *iVar2;
  astruct_891 *iVar3;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  iStack4 = 0x8;
  WStack6 = 0x0;
  switch(param_6) {
  case 0x167:
    enable_win_1040_060e((u32)param_3,0x3);
    hwnd_1 = GetDlgItem16(0x16b,*(HWND16 *)((int)param_3 + 0x6));
    EnableWindow16(0x1,hwnd_1);
    iStack4 = 0x0;
    break;
  case 0x168:
    enable_win_1040_060e((u32)param_3,0x3);
    hwnd_1 = GetDlgItem16(0x16b,*(HWND16 *)((int)param_3 + 0x6));
    EnableWindow16(0x1,hwnd_1);
    iStack4 = 0x1;
    break;
  case 0x169:
    enable_win_1040_060e((u32)param_3,0x3);
    hwnd_1 = GetDlgItem16(0x16b,*(HWND16 *)((int)param_3 + 0x6));
    EnableWindow16(0x1,hwnd_1);
    iStack4 = 0x2;
    break;
  case 0x16a:
    enable_win_1040_060e((u32)param_3,0x3);
    hwnd_1 = GetDlgItem16(0x16b,*(HWND16 *)((int)param_3 + 0x6));
    EnableWindow16(0x1,hwnd_1);
    iStack4 = 0x3;
    break;
  case 0x16b:
    hwnd_1 = GetDlgItem16(0x16b,*(HWND16 *)((int)param_3 + 0x6));
    BVar2 = EnableWindow16(0x0,hwnd_1);
    if (((int)param_3 + 0x92) != 0x3) {
      win_1008_5c5c(BVar2,paVar5,_u16_1050_02a0,0x1de);
    }
    if (((int)param_3 + 0x92) != 0x8) {
      iVar2 = (astruct_890 *)(((int)param_3 + 0x92) * 0xe);
      WStack6 = *(WPARAM16 *)(iVar2 + 0x5c6c);
      pass1_1010_6604((u32)((int)param_3 + 0x8e),(iVar2 + 0x5c66));
      ((int)param_3 + 0x92) = 0x8;
    }
    for (paStack8 = NULL; (int)paStack8 < 0x4; paStack8 = paStack8 + 0x1) {
      uVar6 = win_ui_op_1040_0558((StructB *)param_3,paStack8);
      paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | uVar6 >> 0x10);
    }
    goto LAB_1040_04da;
  case 0x16c:
    hwnd_1 = GetDlgItem16(0x16d,*(HWND16 *)((int)param_3 + 0x6));
    EnableWindow16(0x1,hwnd_1);
    iStack4 = 0x5;
    ((int)param_3 + 0x94) = 0x5;
    goto LAB_1040_04da;
  case 0x16d:
    hwnd_1 = GetDlgItem16(0x16d,*(HWND16 *)((int)param_3 + 0x6));
    BVar2 = EnableWindow16(0x0,hwnd_1);
    win_1008_5c5c(BVar2,paVar5,_u16_1050_02a0,0x1de);
    uVar11 = ((u32)paVar5 >> 0x10);
    if (((int)param_3 + 0x94) != 0x8) {
      iVar3 = (astruct_891 *)(((int)param_3 + 0x94) * 0xe);
      WStack6 = *(WPARAM16 *)(iVar3 + 0x5c6c);
      pass1_1010_6604((u32)((int)param_3 + 0x8e),(iVar3 + 0x5c66));
      ((int)param_3 + 0x94) = 0x8;
    }
    LVar7 = win_ui_op_1040_0558((StructB *)param_3,(astruct_915 *)((int)&u32_1050_0004 + 0x1));
    paVar5 = (astruct_57 *)CONCAT22(uVar11,(int)((u32)LVar7 >> 0x10));
    puStack12 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x39),in_stack_0000fd7c,
                                in_stack_0000fea0,in_stack_0000fea6,in_stack_0000feaa);
    paVar3 = *(astruct_915 **)((int)puStack12 + 0x20);
    uVar14 = SUB42(&DAT_1050_1050,0x0);
    uVar12 = SUB21(&local_16,0x0);
    uVar13 = (u8)(&local_16 >> 0x8);
    pHVar10 = &local_18;
    uVar11 = SUB42(&DAT_1050_1050,0x0);
    uStack14 = ((int)paVar3 >> 0xf) + 0x200;
    uVar6 = (u32)paVar5 & 0xffff0000 | (u32)uStack14;
    paStack16 = paVar3;
    paStack8 = paVar3;
    pass1_1030_8344((u32)_u16_1050_5748,CONCAT22(uStack14,paVar3));
    paStack20 = (astruct_598 *)CONCAT22((int)uVar6,paVar3);
    pass1_1030_2f1a(CONCAT22((int)uVar6,paVar3),(u16 *)CONCAT22(uVar11,pHVar10),
                    (u16 *)CONCAT22(uVar14,CONCAT11(uVar13,uVar12)));
    paVar5 = (astruct_57 *)(uVar6 & 0xffff0000 | (u32)((int)(local_18 - local_16) >> 0xf));
    local_16 += (int)(local_18 - local_16) / 0x2;
    uVar4 = pass1_1030_2fac(paStack20);
    set_window_text_1018_6086((u32)((int)param_3 + 0x96),uVar4,local_16);
    goto LAB_1040_04da;
  case 0x16e:
    puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x39),in_stack_0000fd7c,
                             in_stack_0000fea0,in_stack_0000fea6,in_stack_0000feaa);
    paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)puVar8 >> 0x10);
    iVar1 = ((int)puVar8 + 0x20);
    local_18 = LoadCursor16((char *)0x7f02,0x0);
    local_16 = SetCursor16(local_18);
    pass1_1030_532e((astruct_97 *)CONCAT22(0x1050,&stack0xfed6),(long)iVar1 + 0x2000000);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&stack0xfed6));
    pass1_1030_838e((u32 *)_u16_1050_5748);
    pass1_1030_8334();
    SetCursor16(local_16);
    PostMessage16(0x0,0x7e,0x111,HWND16_1050_0396);
    DestroyWindow16(*(HWND16 *)((int)param_3 + 0x6));
    goto LAB_1040_04da;
  default:
    post_win_msg_1040_7b3c((StructC *)param_3,param_4,param_5,param_6);
    return;
  }
  ((int)param_3 + 0x92) = iStack4;//
LAB_1040_04da:
  uVar11 = (in_stack_0000fedc >> 0x10);
  if (iStack4 != 0x8) {
    uVar15 = in_stack_0000fedc & 0xffff0000 | (u32)((int)param_3 + 0x6);
    id = *(INT16 *)(iStack4 * 0xe + 0x5c68);
    w_param = 0x0;
    msg = 0xc;
    l_param = load_string_1010_847e(_u16_1050_14cc,(iStack4 * 0xe + 0x5c6a));
    uVar6 = (u32)paVar5 & 0xffff0000;
    uVar9 = SendDlgItemMessage16((LPARAM)l_param,w_param,msg,id,(HWND16)uVar15);
    uVar11 = (uVar15 >> 0x10);
    paVar5 = (astruct_57 *)(uVar6 & 0xffff0000 | uVar9 >> 0x10);
  }
  if ((WStack6 != 0x0) &&
     (puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(uVar11,0x2),in_stack_0000fd86,
                               in_stack_0000feaa,in_stack_0000feb0,in_stack_0000feb4),
     ((int)puVar8 + 0x20) != 0x0)) {
    PostMessage16(0x0,WStack6,0x111,HWND16_1050_0396);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT win_ui_op_1040_0558(StructB *param_1,param_2: *mut astruct_915)

{
  HWND16 hwnd;
  let mut iVar2: i16;
  StructB *iVar3;
  let mut uVar3: u16;
  char *l_param;
  LRESULT LVar4;
  let mut uVar5: u16;
  WPARAM16 w_param;
  let mut msg: u16;
  INT16 id;
  LPVOID hwnd_00;
  astruct_913 *iVar1;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (StructB *)param_1;
  iVar1 = (astruct_913 *)((int)param_2 * 0xe);
  hwnd = GetDlgItem16(*(INT16 *)(iVar1 + 0x5c64),(HWND16)iVar3.lpvoid_field_0x8);
  iVar2 = pass1_1010_659a((u32)&iVar3[0x7].field1_0x2,(iVar1 + 0x5c66));
  if ((iVar2 == 0x0) && ((iVar1 + 0x5c66) != 0xa)) {
    EnableWindow16(0x0,hwnd);
    hwnd_00 = iVar3.lpvoid_field_0x8;
    id = *(INT16 *)((int)param_2 * 0xe + 0x5c68);
    uVar5 = 0x531;
  }
  else {
    EnableWindow16(0x1,hwnd);
    hwnd_00 = iVar3.lpvoid_field_0x8;
    id = *(INT16 *)((int)param_2 * 0xe + 0x5c68);
    uVar5 = 0x649;
  }
  msg = 0xc;
  w_param = 0x0;
  l_param = load_string_1010_847e(_u16_1050_14cc,uVar5);
  LVar4 = SendDlgItemMessage16((LPARAM)l_param,w_param,msg,id,(HWND16)hwnd_00);
  return LVar4;
}



// WARNING: Could not reconcile some variable overlaps
pub fn enable_win_1040_060e(mut param_1: u32,mut param_2: i16)

{
  INT16 *pIVar1;
  HWND16 hwnd;
  let mut iStack10: i16;
  INT16 *pIStack8;

  pIStack8 = (INT16 *)CONCAT22(0x1050,&stack0x000a);
  iStack10 = param_2;
  while( true ) {
    pIVar1 = pIStack8;
    if (iStack10 == 0x0) break;
    pIStack8 = (INT16 *)((u32)pIStack8 & 0xffff0000 | (u32)((int)pIStack8 + 0x2));
    hwnd = GetDlgItem16(*pIVar1,*(HWND16 *)((int)param_1 + 0x6));
    EnableWindow16(0x0,hwnd);
    iStack10 = iStack10 + -0x1;
  }
  return;
}



StructD * pass1_1040_0656(StructD *param_1,param_2: u8)

{
  destroy_win_1038_ef3a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 *
pass1_1040_06e8(StructD *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfbc,param_6);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  param_2.field0_0x0 = 0xb90;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x7),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_073a(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xb90;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn show_win_1040_0766(StructB *struct_b_param_1,mut param_2: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  u32 *puVar2;
  let mut in_stack_0000fe7e: u16;
  let mut in_stack_0000fe86: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb4: u16;
  let mut piVar3: *mut i16;
  let mut uVar4: u16;
  let mut piVar5: *mut i16;
  let mut uVar6: u16;
  let mut in_stack_0000ffde: u16;
  let mut local_a: i16;
  let mut local_8: i16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffde,0x2),in_stack_0000fe86,
                           in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
  paVar1 = (astruct_57 *)((u32)paVar1 & 0xffff0000 | (u32)puVar2 >> 0x10);
  uStack6 = SUB42(puVar2,0x0);
  uStack4 = ((u32)puVar2 >> 0x10);
  pass1_1010_6118((u32)puVar2);
  piVar5 = &local_8;
  uVar6 = SUB42(&DAT_1050_1050,0x0);
  piVar3 = &local_a;
  uVar4 = SUB42(&DAT_1050_1050,0x0);
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(piVar3,0x48),in_stack_0000fe7e,in_stack_0000ffa2
                           ,in_stack_0000ffa8,in_stack_0000ffac);
  pass1_1008_3e94((u16 *)((u32)puVar2 & 0xffff0000 | (u32)((int)puVar2 + 0xe)),(u16 *)CONCAT22(uVar4,piVar3),
                  (char *)CONCAT22(uVar6,piVar5));
  move_win_1040_826c(struct_b_param_1,local_a + 0x8c,local_8 + 0xb9);
  ShowWindow16(0x5,*(HWND16 *)((int)struct_b_param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_07dc(mut param_1: u16 ,StructC *pstruct_c_param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  code **ppcVar1;
  INT16 IVar2;
  u8 *puVar3;
  u8 *puVar4;
  u8 *puVar5;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  u32 *puVar7;
  u32 *puVar8;
  let mut in_stack_0000f69a: u16;
  let mut in_stack_0000f7be: u16;
  let mut in_stack_0000f7c4: u16;
  let mut in_stack_0000f7c8: u16;
  let mut BVar9: bool;
  let mut in_stack_0000f7f2: u16;
  let mut uStack2060: u32;
  char local_806 [0x400];
  u32 local_406 [0x100];
  let mut uStack6: u32;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uStack6 = 0x0;
  if (param_5 == 0x73) {
    enable_window_1040_0acc(pstruct_c_param_2,0x0);
    puVar4 = paVar6;
    puVar3 = pass1_1008_5fd8(puVar4);
    uStack2060 = CONCAT22(puVar4,puVar3);
    puVar5 = puVar4;
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_806,(short)&DAT_1050_1050);
    IVar2 = MessageBox16(0x34,(char *)CONCAT13(0x10,CONCAT12(0x50,local_806)),(char *)CONCAT22(puVar4,puVar3),
                         HWND16_1050_0396);
    local_406[0] = uStack2060;
    fn_ptr_1000_17ce((char *)CONCAT22(puVar4,puVar3));
    if (IVar2 == 0x6) {
      PostMessage16(0x0,0xcb,0x111,HWND16_1050_0396);
      BVar9 = post_win_msg_1040_7b3c(pstruct_c_param_2,param_3,param_4,0x1);
      uStack6 = CONCAT22(puVar5,BVar9);
    }
  }
  else {
    if (param_5 < 0x74) {
      if (param_5 == 0x6e) {
        ((int)_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
        puVar8 = (u32 *)
                 pass1_1038_af40(pstruct_c_param_2,param_1,_PTR_LOOP_1050_5b7c,
                                 (pstruct_c_param_2 + 0x6),0x2);
        ppcVar1 = (code **)((int)*puVar8 + 0x3c);
        (**ppcVar1)((int)&u16_1050_1038,(int)puVar8,(int)((u32)puVar8 >> 0x10));
        SetFocus16(*(HWND16 *)(pstruct_c_param_2 + 0x6));
        return;
      }
      if (0x6e < param_5) {//
LAB_1040_09f9:
        post_win_msg_1040_7b3c(pstruct_c_param_2,param_3,param_4,param_5);
        return;
      }
      if ((char)param_5 == '\x02') {//
LAB_1040_09b4:
        post_win_msg_1040_7b3c(pstruct_c_param_2,0x0,0x0,0x2);
        PostMessage16(0x0,0xee,0x111,HWND16_1050_0396);
        return;
      }
      if ((char)param_5 != 'd') goto LAB_1040_09f9;
      PostMessage16(0x0,0x64,0x111,HWND16_1050_0396);
      BVar9 = 0x0;
      goto LAB_1040_0821;
    }
    if (param_5 != 0x74) {
      if (param_5 == 0xee) goto LAB_1040_09b4;
      if (param_5 == 0x13d) {
        enable_window_1040_0acc(pstruct_c_param_2,0x1);
        return;
      }
      goto LAB_1040_09f9;
    }
    enable_window_1040_0acc(pstruct_c_param_2,0x0);
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)local_406,
               (short)&DAT_1050_1050);
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_806,(short)&DAT_1050_1050);
    IVar2 = MessageBox16(0x34,(char *)CONCAT13(0x10,CONCAT12(0x50,local_406)),(char *)CONCAT22(0x1050,local_806),
                         HWND16_1050_0396);
    if (IVar2 == 0x6) {
      PostMessage16(0x0,0x7a,0x111,HWND16_1050_0396);
      BVar9 = post_win_msg_1040_7b3c(pstruct_c_param_2,param_3,param_4,0x1);
      uStack6 = CONCAT22((int)paVar6,BVar9);
      puVar7 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000f7f2,0x2),in_stack_0000f69a,
                               in_stack_0000f7be,in_stack_0000f7c4,in_stack_0000f7c8);
      pass1_1010_60fa((u32)puVar7);
    }
  }
  BVar9 = 0x1;//
LAB_1040_0821:
  enable_window_1040_0acc(pstruct_c_param_2,BVar9);
  return;
}
pub fn pass1_1040_0a1a(mut param_1: u32)

{
  let mut uVar1: u16;
  u32 *puVar2;
  code **ppcVar3;
  let mut uVar4: u32;
  astruct_394 *paVar5;
  let mut uVar6: u16;
  u8 *puVar7;
  let mut in_EDX: u32;
  astruct_57 *paVar8;
  let mut iVar9: i16;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uStack10: u32;
  let mut uStack6: u16;

  uVar11 = (param_1 >> 0x10);
  iVar9 = (int)param_1;
  uVar4 = (u32)(iVar9 + 0x8e);
  uVar12 = ((u32)uVar4 >> 0x10);
  iVar10 = (int)uVar4;
  puVar2 = (u32 *)(u32)(iVar10 + 0xa);
  uVar1 = (iVar10 + 0xc);
  paVar8 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar1);
  uStack6 = puVar2;
  paVar5 = (astruct_394 *)(uVar1 | uStack6);
  if (paVar5 == NULL) {
    return;
  }
  ppcVar3 = (code **)((int)*puVar2 + 0x14);
  (**ppcVar3)();
  uStack10 = CONCAT22((int)paVar8,paVar5);
  if (*(i32 *)(iVar9 + 0x70) != 0x0) {
    paVar5 = *(astruct_394 **)(iVar9 + 0x70);
    uVar1 = (iVar9 + 0x72);
    uVar6 = uVar1 | paVar5;
    paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)uVar6);
    if (uVar6 != 0x0) {
      ppcVar3 = (code **)(u32)paVar5;
      (**ppcVar3)();
    }
  }
  mem_op_1000_179c(0x14,paVar8);
  puVar7 = (paVar8 | paVar5);
  if (puVar7 == NULL) {
    paVar5 = NULL;
    puVar7 = NULL;
  }
  else {
    struct_1008_4c58(paVar5);
  }
  *(astruct_394 **)(iVar9 + 0x70) = paVar5;
  *(u8 **)(iVar9 + 0x72) = puVar7;
  pass1_1008_4d84(puVar7,*(astruct_90 **)(iVar9 + 0x70),uStack10);
  return;
}
pub fn enable_window_1040_0acc(StructC *param_1,BOOL16 enable_3)

{
  let mut BVar1: bool;
  HWND16 HVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  BVar1 = IsWindow16(*(HWND16 *)(iVar3 + 0x6));
  if (BVar1 != 0x0) {
    HVar2 = GetDlgItem16(0x64,*(HWND16 *)(iVar3 + 0x6));
    BVar1 = IsWindow16(HVar2);
    if (BVar1 != 0x0) {
      EnableWindow16(enable_3,HVar2);
      HVar2 = GetDlgItem16(0x74,*(HWND16 *)(iVar3 + 0x6));
      EnableWindow16(enable_3,HVar2);
      HVar2 = GetDlgItem16(0x73,*(HWND16 *)(iVar3 + 0x6));
      EnableWindow16(enable_3,HVar2);
      HVar2 = GetDlgItem16(0x6e,*(HWND16 *)(iVar3 + 0x6));
      EnableWindow16(enable_3,HVar2);
      HVar2 = GetDlgItem16(0xee,*(HWND16 *)(iVar3 + 0x6));
      EnableWindow16(enable_3,HVar2);
    }
  }
  return;
}



StructD * pass1_1040_0b6a(StructD *param_1,param_2: u8)

{
  pass1_1040_073a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 *
pass1_1040_0bfc(StructD *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfcd,param_6);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  param_2.field0_0x0 = 0xdb0;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x39),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  iVar1.field86_0x74 = 0x1;
  return param_2;
}
pub fn pass1_1040_0c54(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xdb0;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  (u32)((int)param_1 + 0x8e) = 0x0;
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn show_win_1040_0c7c(StructB *param_1)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut local_6: u32;

  dialog_ui_fn_1040_78e2(param_1);
  uVar2 = ((u32)param_1 >> 0x10);
  uVar1 = (u32)((int)param_1 + 0x8e);
  pass1_1010_4f30(uVar1,((u32)uVar1 >> 0x10),(u16 *)CONCAT22(0x1050,&local_6),
                  (u16 *)CONCAT22(0x1050,(int)&local_6 + 0x2));
  move_win_1040_826c(param_1,(INT16)local_6,(BOOL16)((u32)local_6 >> 0x10));
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn set_text_bk_color_1040_0cc0(u32 *param_1,mut param_2: u16 ,mut param_3: u16 ,HWND16 hwnd_param_4) -> u32

{
  astruct_783 *iVar1;
  let mut uVar3: u16;
  let mut uVar1: u32;
  HGDIOBJ16 hobject;
  code **fn_ptr_1;

  hobject = GetStockObject16(BLACK_BRUSH);
  if (_u16_1050_5cd0 == 0x0) {
    fn_ptr_1 = (code **)((int)*param_1 + 0x68);
    uVar1 = (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,param_1,((int)param_1 + 0x6e));
    uVar1 = pass1_1008_4d72(uVar1);
    uVar3 = (uVar1 >> 0x10);
    iVar1 = (astruct_783 *)uVar1;
    _u16_1050_5cd0 = CONCAT22(CONCAT11(0x2,iVar1.field_0x94),CONCAT11(iVar1.field_0x95,iVar1.field_0x96));
  }
  if (0x3 < param_3) {
    if (param_3 == 0x4) {
      hobject = GetStockObject16(HOLLOW_BRUSH);
    }
    else if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
      return 0x0;
    }
  }
  SetTextColor16(_u16_1050_5cd0,hwnd_param_4);
  SetBkColor16(0x1000000,hwnd_param_4);
  return CONCAT22(0x1050,hobject);
}
pub fn post_win_msg_1040_0d5e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16)

{
  if (param_3 != 0x0) {
    PostMessage16(0x0,0x1,0x111,*(HWND16 *)(param_1 + 0x8));
  }
  return;
}



u16 pass1_1040_0d80(void)

{
  return 0x1;
}
pub fn FUN_1040_0d86(void)

{
  return;
}



StructD * pass1_1040_0d8a(StructD *param_1,param_2: u8)

{
  pass1_1040_0c54(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_0e1c(StructD *param_1,param_2: *mut astruct_57,mut param_3: u16 ,mut param_4: u32,mut param_5: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x1c0,param_5);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  (u32)&iVar1[0x1].field2_0x4 = param_4;
  iVar1[0x1].field4_0x8 = 0x0;
  iVar1[0x1].field5_0xa = param_3;
    // just 0x11d2
  param_2.field0_0x0 = (int)s_overflow_on_node__d_1050_11ca + 0x8;
    // just 0x1040
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3a),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_0e86(StructD *param_1)

{
  let mut uVar1: u16;
  char *pcVar2;
  let mut uVar3: u16;
  let mut in_EDX: u32;
  let mut iVar5: i16;
  let mut uVar6: u16;
  u32 *puVar7;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffee: u16;
  astruct_57 *paVar4;

  uVar6 = ((u32)param_1 >> 0x10);
  iVar5 = (int)param_1;
  param_1.address_offset_field_0x0 = (int)s_overflow_on_node__d_1050_11ca + 0x8;
  (iVar5 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pcVar2 = *(char **)(iVar5 + 0x92);
  uVar1 = (iVar5 + 0x94);
  uVar3 = uVar1 | pcVar2;
  paVar4 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar3);
  if (uVar3 != 0x0) {
    pass1_1040_a5d0((StructD *)((u32)pcVar2 & 0xffff | (u32)uVar1 << 0x10));
    fn_ptr_1000_17ce(pcVar2);
  }
  PTR_LOOP_1050_5b82 = (iVar5 + 0x96);
  if (*(i32 *)(iVar5 + 0x92) == 0x0) {
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(iVar5 + 0x6));
  }
  else {
    puVar7 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffee,0x32),in_stack_0000fe96,
                             in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
    pass1_1010_7b8c((u32)puVar7,(iVar5 + 0x6));
  }
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn FUN_1040_0f0c(mut param_1: u16 ,StructB *param_2)

{
  let mut uVar1: u32;
  let mut in_AX: u16;
  HWND16 HVar2;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  u32 *puVar7;
  let mut in_stack_0000fe6e: u16;
  let mut in_stack_0000ff92: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ff9c: u16;
  INT16 id;
  let mut in_stack_0000ffc6: u16;
  u8 local_2e [0x2];
  let mut iStack44: i16;
  let mut local_26: i16;
  let mut iStack36: i16;
  let mut iStack34: i16;
  let mut iStack32: i16;
  let mut iStack30: i16;
  let mut uStack28: u16;
  let mut iStack26: i16;
  let mut iStack24: i16;
  let mut iStack22: i16;
  let mut iStack20: i16;
  let mut iStack18: i16;
  let mut iStack16: i16;
  let mut local_e: i16;
  let mut iStack12: i16;
  let mut iStack10: i16;
  let mut iStack8: i16;
  POINT16 local_6;

  dialog_ui_fn_1040_78e2(param_2);
  uVar5 = ((u32)param_2 >> 0x10);
  iVar4 = (int)param_2;
  if ((iVar4 + 0x98) == 0x0) {
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_26),*(HWND16 *)(iVar4 + 0x6));
    uVar3 = ((u32)in_EDX >> 0x10);
    HVar2 = GetDlgItem16(0x1830,*(HWND16 *)(iVar4 + 0x6));
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,local_2e),HVar2);
    iStack34 -= local_26;
    iStack32 = (iStack44 - iStack36) + -0x2;
    SetWindowPos16(0x6,iStack32,iStack34,0x0,0x0,0x0,*(HWND16 *)(iVar4 + 0x6));
    CheckDlgButton16(0x1,0x1c1,*(HWND16 *)(iVar4 + 0x6));
    uVar1 = (u32)(iVar4 + 0x8e);
    ((int)uVar1 + 0xa) = 0x2;
    HVar2 = GetDlgItem16(0x1830,*(HWND16 *)(iVar4 + 0x6));
    in_stack_0000ffc6 = 0x0;
    EnableWindow16(0x0,HVar2);
  }
  else {
    uVar1 = (u32)(iVar4 + 0x92);
    uVar6 = struct_op_1030_73a8(*(astruct_419 **)((int)uVar1 + 0x6),in_AX,(int)in_EDX);
    uVar3 = ((u32)in_EDX >> 0x10);
    if (((int)uVar6 + 0x20) == 0x2) {
      HVar2 = *(HWND16 *)(iVar4 + 0x6);
      id = 0x1c1;
    }
    else {
      HVar2 = *(HWND16 *)(iVar4 + 0x6);
      id = 0x1c2;
    }
    CheckDlgButton16(0x1,id,HVar2);
  }
  GetCursorPos16(&local_6);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_e),*(HWND16 *)(iVar4 + 0x6));
  iStack20 = iStack10 - local_e;
  iStack16 = -(iStack20 / 0x2 - local_6.x);
  iStack22 = iStack8 - iStack12;
  iStack18 = -(iStack22 / 0x2 - local_6.y);
  puVar7 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar3,iStack22 >> 0xf),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000ffc6,0x48),in_stack_0000fe6e,in_stack_0000ff92,
                           in_stack_0000ff98,in_stack_0000ff9c);
  uStack28 = ((u32)puVar7 >> 0x10);
  iStack30 = (int)puVar7;
  iStack24 = (iStack30 + 0xa);
  iStack26 = (iStack30 + 0xc);
  if (iStack24 < iStack20 + iStack16) {
    iStack16 = iStack24 - iStack20;
  }
  if (iStack26 < iStack22 + iStack18) {
    iStack18 = iStack26 - iStack22;
  }
  SetWindowPos16(0x45,0x0,0x0,iStack18,iStack16,0x0,*(HWND16 *)(iVar4 + 0x6));
  return;
}

