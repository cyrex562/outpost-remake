

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_help_op_1020_0ec4(param_1: *mut u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let cVar2: u8;
  let uVar3: u16;
  let in_DX: *mut u8
  let uVar4: u16;
  let unaff_DI: i16;
  let puVar5: *mut u16;
  let uVar6: u32;
  astruct_43 *paVar7;
  let uVar8: u16;
  let uVar9: u16;
  let iVar10: i16;
  
  uVar8 = (param_1 >> 0x10);
  uVar3 = param_1;
  if (param_2 == 0xfb) {
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_3,in_DX,unaff_DI);
    pass1_1010_375e(puVar5);
    ppcVar1 = (code **)(*param_1 + 0x14);
    (**ppcVar1)();
    uVar6 = pass1_1010_375e(puVar5);
    uVar4 = (uVar6 >> 0x10);
    pass1_1010_4788((uVar3 + 0xf2),
                    (uVar6 & 0xffff | uVar4 << 0x10),uVar6,uVar4);
    return;
  }
  if (param_2 < 0xfc) {
    if (param_2 == 0xf3) {
      iVar10 = 0x3;
    }
    else {
      if (0xf3 < param_2) {
        return;
      }
      cVar2 = param_2;
      if (cVar2 == 'o') {
        paVar7 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,param_3);
        WinHelp16(0x1010,(LPCSTR)(s_New_failed_in_Op__Op_1050_0020 + 0x8),0x0,
                  CONCAT22(paVar7,0x1));
        return;
      }
      if (cVar2 == 'r') {
        iVar10 = uVar3 + 0xa;
        uVar9 = uVar8;
        puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_3,in_DX,unaff_DI);
        uVar4 = (puVar5 >> 0x10);
        pass1_1010_3770(puVar5,CONCAT22(uVar9,iVar10),uVar4);
        pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar3 + 0x8),0x3,uVar4,uVar3,
                        &PTR_LOOP_1050_1038,param_3);
        return;
      }
      if (cVar2 == -0xf) {
        iVar10 = 0x1;
      }
      else {
        if (cVar2 != -0xe) {
          return;
        }
        iVar10 = 0x2;
      }
    }
    pass1_1010_4674((uVar3 + 0xf2),iVar10,0x1010,param_3);
    return;
  }
  if (true) {
    switch(param_2) {
    default:
      goto switchD_1020_0feb_caseD_129;
    case 0x12a:
      uVar8 = 0xf012;
      break;
    case 0x12c:
      uVar8 = 0xf020;
    }
    PostMessage16(0x1020,0x0,0x0,CONCAT22(0x112,uVar8));
    return;
  }
switchD_1020_0feb_caseD_129:
  return;
}


fn enable_menu_1020_1000(HMENparam_1: u16,param_2: i16)
{
  if (param_2 != 0x0) {
    return;
  }
  EnableMenuItem16(param_1,0x400,0x3);
  return;
}


fn window_op_1020_10a0(astruct *param_1)
{
  let uVar1: u32;
  code **ppcVar2;
  astruct_160 *in_AX;
  let uVar3: u16;
  bool *pBVar4;
  let in_DX: *mut u8
  let puVar5: *mut u8
  let puVar6: *mut u8
  let extraout_DX: *mut u8
  let extraout_DX_00: u16;
  let unaff_DI: i16;
  let unaff_SS: u16;
  ulet in_AF: u8;
  let puVar7: *mut u16;
  let uVar8: u32;
  let uVar9: u16;
  let puVar10: *mut u8;
  let iVar11: i16;
  let uVar12: u16;
  
  iVar11 = param_1;
  uVar12 = (param_1 >> 0x10);
  create_window_ex_1008_9760(param_1,0x1008);
  mem_op_1000_179c(0x42,in_DX,0x1000);
  puVar5 = (in_DX | in_AX);
  if (puVar5 != 0x0) {
    pass1_1008_3bd6(in_AX,in_DX,0x0,0x1f009b,0x0,0x740075,
                    CONCAT22((iVar11 + 0x8),0xf1),puVar5,unaff_SS);
  }
  mem_op_1000_179c(0x42,puVar5,0x1000);
  puVar6 = (puVar5 | in_AX);
  if (puVar6 != 0x0) {
    pass1_1008_3bd6(in_AX,puVar5,0x0,0x31009b,0x0,0x760077,
                    CONCAT22((iVar11 + 0x8),0xf2),puVar6,unaff_SS);
  }
  mem_op_1000_179c(0x42,puVar6,0x1000);
  puVar5 = (puVar6 | in_AX);
  if (puVar5 != 0x0) {
    pass1_1008_3bd6(in_AX,puVar6,0x0,0x77009b,0x0,0x780079,
                    CONCAT22((iVar11 + 0x8),0xf3),puVar5,unaff_SS);
  }
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2d,unaff_SS,puVar5,unaff_DI);
  uVar9 = (puVar7 >> 0x10);
  (iVar11 + 0xf2) = puVar7;
  (iVar11 + 0xf4) = uVar9;
  (iVar11 + 0xe0) = (iVar11 + 0xf2);
  (iVar11 + 0xe2) = uVar9;
  puVar10 = PTR_LOOP_1050_038c;
  uVar3 = LoadIcon16(0x1010,(LPCSTR)s_PLNTICON_1050_4267);
  *(HICON16 *)(iVar11 + 0xc2) = uVar3;
  uVar1 = (iVar11 + 0xf2);
  ppcVar2 = (code **)((iVar11 + 0xf2) + 0x30);
  (**ppcVar2)(s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10),uVar3,
              puVar10);
  puVar5 = extraout_DX;
  mem_op_1000_179c(0x24,extraout_DX,0x1000);
  puVar6 = (puVar5 | uVar3);
  if (puVar6 == 0x0) {
    (iVar11 + 0xf6) = 0x0;
  }
  else {
    unk_win_ui_op_1020_1418((astruct_40 *)CONCAT22(puVar5,uVar3),(ULONG)param_1,unaff_SS);
    (iVar11 + 0xf6) = uVar3;
    *(uchar **)(iVar11 + 0xf8) = puVar6;
  }
  (iVar11 + 0xe8) = (iVar11 + 0xf6);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,unaff_SS,puVar6,unaff_DI);
  uVar8 = pass1_1018_04b8(puVar7);
  puVar5 = (uVar8 >> 0x10);
  pass1_1010_41d6((iVar11 + 0xf2),uVar8,puVar5,unaff_SS,in_AF);
  uVar8 = pass1_1010_451a((iVar11 + 0xf2),puVar5,unaff_DI,unaff_SS);
  pBVar4 = (bool *)uVar8;
  uVar1 = param_1;
  ppcVar2 = (code **)(uVar1 + 0x14);
  (**ppcVar2)(0x1010,iVar11,uVar12,0x0,pBVar4,(uVar8 >> 0x10));
  uVar9 = 0x1;
  ppcVar2 = (code **)(uVar1 + 0x10);
  (**ppcVar2)();
  pass1_1010_459e(*(long *)(iVar11 + 0xf2));
  ppcVar2 = (code **)((iVar11 + 0xf2) + 0x10);
  (**ppcVar2)(0x1010,(iVar11 + 0xf2),param_1,uVar9);
  MoveWindow16(0x1010,0x1,pBVar4[0x3],pBVar4[0x2],pBVar4[0x1],*pBVar4);
  UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
  return;
}


void 
win_ui_cursor_op_1020_1294
          (param_1: u32,param_2: i16,param_3: i16,param_4: u16,param_5: u16)

{
  code **ppcVar1;
  let in_AX: u16;
  HCURSOR16 HVar2;
  HCURSOR16 HVar3;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u32;
  let local_12: i16;
  let local_10: i16;
  let puStack14: *mut u16;
  let puStack10: u32;
  let local_6: i16;
  let iStack4: i16;
  
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,0x4000001);
  if ((param_4 | in_AX) == 0x0) {
    local_6 = param_3;
    iStack4 = param_2;
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puStack10 = pass1_1010_40cc((iVar4 + 0xf2),param_2,0x0);
    uVar6 = (iVar4 + 0xf2);
    puStack14 = (uVar6 & 0xffff0000 | (uVar6 + 0x76));
    pass1_1008_3e94(puStack14,CONCAT22(param_5,&local_12),
                    CONCAT22(param_5,&local_10));
    local_6 -= local_10;
    iStack4 -= local_12;
    iVar4 = pt_in_rect_1010_40f8
                      ((iVar4 + 0xf2),(POINT16 *)CONCAT22(param_5,&local_6),
                       0x1010);
    if (iVar4 != -0x1) {
      uVar6 = 0x0;
      HVar2 = LoadCursor16(0x1010,(LPCSTR)0x7f02);
      uVar6 = uVar6 & 0xffff0000 | HVar2;
      HVar3 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
      ppcVar1 = (code **)(*puStack10 + 0x4);
      (**ppcVar1)(s_tile2_bmp_1050_1538,puStack10,
                  (puStack10 >> 0x10),iVar4,iVar4 >> 0xf,iVar4,0x2,uVar6,HVar3
                  ,HVar2);
      pass1_1008_3e0e(param_1);
      SetCursor16(0x1008);
    }
  }
  return;
}


fn unk_win_ui_op_1020_1418(astruct_40 *param_1,Uparam_2: i32,param_3: u16)
{
  let uVar1: u32;
  astruct_13 *paVar2;
  code **ppcVar3;
  HDC16 *pHVar4;
  let puVar5: u32;
  let puVar6: *mut u8
  let extraout_DX: *mut u8
  astruct_40 *iVar5;
  let unaff_DI: i16;
  let uVar7: u16;
  let unaff_CS: u16;
  let puVar8: *mut u16;
  HDC16 local_8;
  let puStack6: *mut u16;
  
  get_sys_metrics_1020_7c1a((u16 *)param_1,param_2,unaff_CS);
  uVar7 = (param_1 >> 0x10);
  iVar5 = (astruct_40 *)param_1;
  &iVar5->field_0x14 = 0x0;
  iVar5->field_0x18 = 0x0;
  puVar8 = pass1_1008_3e38((u16 *)
                           (param_1 & 0xffff0000 | &iVar5->field_0x1e)
                          );
  param_1 = 0x1730;
  iVar5->field_0x2 = 0x1020;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2d,param_3,
                           (puVar8 >> 0x10),unaff_DI);
  puVar6 = (puVar8 >> 0x10);
  iVar5->field_0x14 = puVar8;
  *(uchar **)&iVar5->field_0x16 = puVar6;
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_3,puVar6,unaff_DI);
  uVar1 = &iVar5->field_0x14;
  ppcVar3 = (code **)(**(u32 **)&iVar5->field_0x14 + 0x4);
  (**ppcVar3)(0x1010,uVar1,(uVar1 >> 0x10),0x0,param_1);
  local_8 = GetDC16(0x1010);
  uVar1 = &iVar5->field_0x14;
  *(HDC16 *)(uVar1 + 0x7c) = local_8;
  uVar1 = &iVar5->field_0x14;
  puVar5 = (uVar1 + 0x66);
  iVar5->field_0x18 = puVar5;
  ppcVar3 = (code **)(*iVar5->field_0x18 + 0x14);
  (**ppcVar3)();
  paVar2 = *(astruct_13 **)(puStack6 + 0xe);
  puVar6 = extraout_DX;
  pass1_1008_4d84((astruct_90 *)(puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10),
                  paVar2,extraout_DX);
  pHVar4 = (HDC16 *)palette_op_1008_4e08(paVar2,&local_8,puVar6,0x1008);
  iVar5->field_0x1c = pHVar4;
  return;
}



fn win_ui_op_1020_150e(param_1: *mut u16,HDC16 param_2)
{
  HPALETTE16 HVar1;
  let iVar2: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  *param_1 = 0x1730;
  (iVar2 + 0x2) = 0x1020;
  if (*(long *)(iVar2 + 0x14) != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1ea6((iVar2 + 0x14),
                    param_1 & 0xffff | uVar3 << 0x10,unaff_SS);
  }
  HVar1 = SelectPalette16(param_2,0x0,*(bool *)(iVar2 + 0x1c));
  *(HPALETTE16 *)(iVar2 + 0x1c) = HVar1;
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  *param_1 = 0x3ab0;
  (iVar2 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar2 + 0x2) = 0x1008;
  return;
}


fn mixed_ui_op_1020_179c(astruct_1 *param_1)
{
  let uVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let uVar4: u16;
  let IVar5: i16;
  let puVar6: *mut u8;
  let in_DX: *mut u8
  let extraout_DX: *mut u8
  let puVar7: *mut u8
  let uVar8: u16;
  let iVar9: i16;
  let iVar10: i16;
  let unaff_DI: i16;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  WNDCLASS16 *unaff_SS;
  let puVar14: *mut u16;
  WNDCLASS16 *in_resc_id_3;
  WNDCLASS16 *in_buffer_4;
  WNDCLASS16 local_178 [0xc];
  let uStack118: u32;
  let uStack114: u32;
  RECT16 local_6e;
  let uStack106: u32;
  HWND16 HStack102;
  let iStack98: i16;
  let iStack94: i16;
  let uStack78: u16;
  let puStack76: *mut u8
  let uStack74: u32;
  HWND16 HStack70;
  let uStack68: u32;
  let uStack64: u32;
  LPVOID pvStack60;
  let uStack58: u16;
  let uStack56: u16;
  ULONG *pUStack54;
  let uStack50: u32;
  let local_2e: [u8;12];
  RECT16 local_1c;
  let uStack24: u32;
  let iStack20: i16;
  let iStack18: i16;
  let puStack16: *mut u16;
  INT16 *pIStack12;
  let uStack8: u16;
  let puStack6: *mut u16;
  
  dialog_ui_fn_1040_78e2(param_1,&PTR_LOOP_1050_1040);
  uVar4 = 0x89;
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,unaff_SS,in_DX,unaff_DI);
  puVar7 = (puStack6 >> 0x10);
  uVar4 = pass1_1010_65d0(unaff_SS,puStack6,uVar4);
  uStack8 = (uVar4 == 0x0);
  uVar4 = pass1_1010_65d0(unaff_SS,puStack6,0x86);
  if (uVar4 != 0x0) {
    uStack8 = 0x0;
  }
  puVar14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,unaff_SS,puVar7,unaff_DI);
  uVar12 = (puVar14 >> 0x10);
  uVar8 = puVar14;
  uVar11 = (param_1 >> 0x10);
  iVar9 = param_1;
  (iVar9 + 0x8e) = uVar8;
  (iVar9 + 0x90) = uVar12;
  ppcVar2 = (code **)((iVar9 + 0x8e) + 0x10);
  (**ppcVar2)(0x1010,(iVar9 + 0x8e),uVar12,uStack8);
  puStack76 = extraout_DX;
  mem_op_1000_179c(0x12,extraout_DX,0x1000);
  puVar7 = (puStack76 | uVar8);
  uStack78 = uVar8;
  if (puVar7 == 0x0) {
    (iVar9 + 0x92) = 0x0;
  }
  else {
    pass1_1020_1eea((u16 *)CONCAT22(puStack76,uVar8),param_1,
                    (iVar9 + 0x6),puVar7,unaff_DI,unaff_SS);
    (iVar9 + 0x92) = uVar8;
    *(uchar **)(iVar9 + 0x94) = puVar7;
  }
  uVar1 = (iVar9 + 0x8e);
  pIStack12 = (INT16 *)(uVar1 & 0xffff0000 | (uVar1 + 0xa));
  puStack16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,puVar7,unaff_DI);
  GetClientRect16(0x1010,&local_1c);
  IVar5 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  uVar12 = (pIStack12 >> 0x10);
  iVar10 = pIStack12;
  (iVar10 + 0x6) = IVar5 + uStack24._2_2_;
  uVar13 = (puStack16 >> 0x10);
  iStack18 = (puStack16 + 0xa);
  iStack20 = (puStack16 + 0xc);
  (iVar10 + 0x2) = (iStack20 - (iVar10 + 0x6)) / 0x2;
  iVar10 = iStack18 - (iVar10 + 0x4);
  uVar8 = iVar10 >> 0xf;
  *pIStack12 = iVar10 / 0x2;
  pass1_1028_dc52((astruct_92 *)CONCAT22(unaff_SS,local_2e),0x1,0x0,0x100);
  uStack56 = 0x0;
  while( true ) {
    puVar6 = local_2e;
    pass1_1028_e4ec(CONCAT22(unaff_SS,puVar6));
    uStack50 = CONCAT22(uVar8,puVar6);
    uStack58 = uVar8 | puVar6;
    if (uStack58 == 0x0) break;
    pUStack54 = *(ULONG **)(puVar6 + 0x10);
    uVar8 = uStack58;
    if (pUStack54 != (ULONG *)0x0) {
      pass1_1000_3cea(param_1 & 0xffff0000 | (iVar9 + 0x10),*pUStack54);
      uVar8 = uStack58;
    }
  }
  uVar4 = pass1_1020_1da8(param_1,puVar6,0x0,unaff_SS);
  (iVar9 + 0x96) = uVar4;
  uVar4 = pass1_1010_65d0(unaff_SS,puStack6,0x86);
  if ((uVar4 == 0x0) || ((iVar9 + 0x96) != 0x0)) {
    uVar3 = (iVar9 + 0x8e);
    (uVar3 + 0x2c) = 0x0;
    HStack102 = GetDlgItem16(0x1010,0x175);
    if (uStack8 != 0x0) {
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (_PTR_LOOP_1050_14cc >> 0x10),0x100,local_178,
                 (short)unaff_SS);
      SetWindowText16(0x1010,(SEGPTR)local_178);
    }
    pvStack60 = MakeProcInstance16((LPVOID)s_tile2_bmp_1050_1538,
                                   (HANDLE16)PTR_LOOP_1050_038c);
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_6e);
    uStack114 = uStack106;
    iStack98 = uStack106 - local_6e.x;
    iStack94 = uStack106._2_2_ - local_6e.y;
    uStack118 = local_6e & 0xffff0000 |
                (-(iStack98 - (pIStack12 + 0x4)) / 0x2);
    IVar5 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    uVar1 = uStack118 & 0xffff;
    uStack118 = uVar1 | (uStack118._2_2_ - IVar5) << 0x10;
    uStack118._0_2_ = (bool)uVar1;
    MoveWindow16((HWND16)s_tile2_bmp_1050_1538,0x0,iStack94,iStack98,
                 uStack118._2_2_ - IVar5,(bool)uStack118);
  }
  else {
    win_1008_5c7c(_PTR_LOOP_1050_02a0,0x9d0001,unaff_SS,uVar4,uStack58);
    (iVar9 + 0x8c) = uVar4;
    pvStack60 = MakeProcInstance16((LPVOID)0x1008,(HANDLE16)PTR_LOOP_1050_038c);
  }
  EnumChildWindows1((HWND16)s_tile2_bmp_1050_1538,(LPVOID)0x0,ZEXT24(pvStack60) << 0x10);
  FreeProcInstance16((LPVOID)s_tile2_bmp_1050_1538);
  HStack70 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_1c);
  uStack64 = uStack24;
  local_1c.x = uStack24 - local_1c.x;
  uStack74 = CONCAT22(local_1c.x,uStack24._2_2_ - local_1c.y);
  uStack68 = local_1c & 0xffff0000 |
             (-(local_1c.x - (pIStack12 + 0x4)) / 0x2);
  IVar5 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  uStack68 = uStack68 & 0xffff | (uStack68._2_2_ - IVar5) << 0x10;
  if ((iVar9 + 0x96) == 0x0) {
    if (uStack8 == 0x0) goto LAB_1020_1b24;
    in_buffer_4 = local_178;
    in_resc_id_3 = (WNDCLASS16 *)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21);
  }
  else {
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x100,local_178,
               (short)unaff_SS);
    GetDlgItem16(0x1010,0x175);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_178);
    in_resc_id_3 = local_178;
    in_buffer_4 = unaff_SS;
    unaff_SS = (WNDCLASS16 *)0x3fe;
  }
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),in_resc_id_3,
             in_buffer_4,(short)unaff_SS);
  SetWindowText16(0x1010,(SEGPTR)local_178);
LAB_1020_1b24:
  MoveWindow16((HWND16)s_tile2_bmp_1050_1538,0x0,uStack74,
               (uStack74 >> 0x10),uStack68._2_2_,(bool)uStack68);
  uVar12 = (pIStack12 >> 0x10);
  iVar9 = pIStack12;
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x44,*(INT16 *)(iVar9 + 0x6),
                 *(INT16 *)(iVar9 + 0x4),*(INT16 *)(iVar9 + 0x2),*pIStack12,0x0);
  return;
}


void 
enable_window_1020_1bd4
          (param_1: i16,param_2: u16,param_3: u16,param_4: u32,HWND16 param_5)

{
  code **ppcVar1;
  let bVar2: bool;
  let in_AX: u16;
  let iVar3: i16;
  let in_DX: *mut u8
  let puVar4: *mut u8
  let uVar5: u16;
  let unaff_SS: u16;
  let puStack12: u32;
  
  bVar2 = false;
  pass1_1020_1d8e(CONCAT22(param_2,param_1),CONCAT22(param_4,param_3));
  if (in_AX != 0x0) {
    if (in_AX < 0x2) {
      bVar2 = true;
    }
    else {
      GetDlgItem16(param_5,0x1);
      pass1_1010_4e8c((param_1 + 0x8e),unaff_SS);
      in_AX = EnableWindow16(0x1010,0x1);
      pass1_1010_4df0((param_1 + 0x8e),in_DX,unaff_SS);
      if ((in_AX == 0x0) && (bVar2 = true, (param_1 + 0x96) == 0x0)) {
        in_AX = EnableWindow16(0x1010,0x0);
      }
    }
  }
  if (bVar2) {
    uVar5 = 0x1000;
    mem_op_1000_179c(0xb4,in_DX,0x1000);
    puVar4 = (in_DX | in_AX);
    if (puVar4 == 0x0) {
      iVar3 = 0x0;
      puVar4 = 0x0;
    }
    else {
      uVar5 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar3 = string_1040_8520((astruct_57 *)CONCAT22(in_DX,in_AX),
                               (param_1 + 0x6),0x30,0x2,0x57b,0x62a,puVar4,
                               unaff_SS);
    }
    puStack12 = CONCAT22(puVar4,iVar3);
    ppcVar1 = (code **)(*puStack12 + 0x74);
    (**ppcVar1)(uVar5,iVar3,puVar4);
  }
  return;
}


fn post_win_msg_1020_1ca4(param_1: u32) -> bool

{
  code **ppcVar1;
  let in_AX: u16;
  let iVar2: i16;
  let in_DX: *mut u8
  let puVar3: *mut u8
  let uVar4: u16;
  let unaff_SS: u16;
  let puStack10: u32;
  
  uVar4 = (param_1 >> 0x10);
  if ((param_1 + 0x96) == 0x0) {
    pass1_1010_4df0((param_1 + 0x8e),in_DX,unaff_SS);
    if (in_AX == 0x0) {
      mem_op_1000_179c(0xb4,in_DX,0x1000);
      puVar3 = (in_DX | in_AX);
      if (puVar3 == 0x0) {
        iVar2 = 0x0;
        puVar3 = 0x0;
      }
      else {
        iVar2 = string_1040_8520((astruct_57 *)CONCAT22(in_DX,in_AX),
                                 PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x62a,puVar3,
                                 unaff_SS);
      }
      puStack10 = CONCAT22(puVar3,iVar2);
      ppcVar1 = (code **)(*puStack10 + 0x74);
      (**ppcVar1)();
      return 0x0;
    }
    PostMessage16(0x1010,0x0,0x0,0x11100de);
  }
  return 0x1;
}



void 
set_win_tet_1020_1d2a
          (param_1: u16,param_2: u16,SEGPTR in_win_text_3,param_4: u16,
          INT16 in_dlg_id_5,HWND16 in_hwnd_6)

{
  GetDlgItem16(in_hwnd_6,in_dlg_id_5);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,in_win_text_3);
  return;
}


fn window_op_1020_2642(astruct *param_1)
{
  astruct_664 *in_AX;
  let in_DX: *mut u8
  let uVar1: u16;
  let iVar2: i16;
  let unaff_DI: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2),(iVar2 + 0x8),0x1018);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar1 = in_DX | in_AX;
  if (uVar1 != 0x0) {
    pass1_1020_27b0(in_AX,in_DX,(iVar2 + 0x8),unaff_DI,unaff_SS);
    *(astruct_664 **)(iVar2 + 0xee) = in_AX;
    (iVar2 + 0xf0) = uVar1;
    return;
  }
  (iVar2 + 0xee) = 0x0;
  return;
}


fn post_win_msg_1020_291a(HWND16 param_1)
{
  PostMessage16(param_1,0x0,0x0,0x100000);
  return;
}


u32 
send_msg_1020_29d8(param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16,
                  param_6: u16,param_7: u16)

{
  let puVar1: *mut u8
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar2: *mut u16;
  let iVar3: i16;
  
  iVar3 = (param_4 >> 0x10);
  post_win_msg_1020_79fc
            ((astruct_69 *)CONCAT22(param_2,param_1),param_3,param_4,iVar3,param_7
            );
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,unaff_SS,param_6,unaff_DI);
  puVar1 = (puVar2 >> 0x10);
  if (iVar3 == 0x0) {
    pass1_1018_270e(puVar2,0x1,(param_1 + 0xfc),puVar1,unaff_DI,unaff_SS);
  }
  else {
    pass1_1018_270e(puVar2,0x0,(param_1 + 0xfc),puVar1,unaff_DI,unaff_SS);
    SendMessage16(0x1018,0x0,0x0,0x1110069);
  }
  return CONCAT22(param_6,param_5);
}


fn bring_window_to_top_1020_2aae(param_1: u32,param_2: u32)
{
  let unaff_SS: u16;
  
  pass1_1008_3e0e(param_1);
  BringWindowToTop16(0x1008);
  pass1_1018_169e((param_1 + 0xf2),param_2,unaff_SS);
  return;
}


fn enable_menu_item_1020_2c2a(HMENparam_1: u16,param_2: i16) -> bool

{
  let BVar1: bool;
  let w_item_id: u16;
  
  if (param_2 != 0x0) {
    return param_2 - 0x1;
  }
  EnableMenuItem16(param_1,0x400,0x3);
  if (PTR_LOOP_1050_3960 < 0x2) {
    w_item_id = 0x401;
  }
  else {
    w_item_id = 0x400;
  }
  BVar1 = EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,w_item_id,0x5);
  return BVar1;
}


fn win_ui_op_1020_2cf0(astruct *param_1)
{
  let uVar1: u32;
  code **ppcVar2;
  let uVar3: u16;
  bool *pBVar4;
  let in_DX: *mut u8
  let uVar5: u16;
  let extraout_DX: *mut u8
  let puVar6: *mut u8
  let uVar7: u16;
  let extraout_DX_00: u16;
  let iVar8: i16;
  let unaff_DI: i16;
  let uVar9: u16;
  let unaff_SS: u16;
  let puVar10: *mut u16;
  let uVar11: u32;
  let puVar12: *mut u8;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,(iVar8 + 0xfc),unaff_SS,in_DX,
                            unaff_DI);
  uVar5 = (puVar10 >> 0x10);
  (iVar8 + 0xf2) = puVar10;
  (iVar8 + 0xf4) = uVar5;
  (iVar8 + 0xe0) = (iVar8 + 0xf2);
  (iVar8 + 0xe2) = uVar5;
  puVar12 = PTR_LOOP_1050_038c;
  uVar3 = LoadIcon16(0x1010,(LPCSTR)s_SITEICON_1050_428d);
  *(HICON16 *)(iVar8 + 0xc2) = uVar3;
  uVar1 = (iVar8 + 0xf2);
  ppcVar2 = (code **)((iVar8 + 0xf2) + 0x30);
  (**ppcVar2)(s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10),uVar3,
              puVar12);
  puVar6 = extraout_DX;
  mem_op_1000_179c(0x22,extraout_DX,0x1000);
  uVar7 = puVar6 | uVar3;
  if (uVar7 == 0x0) {
    (iVar8 + 0xf6) = 0x0;
  }
  else {
    load_draw_op_1020_2ede((u16 *)CONCAT22(puVar6,uVar3),param_1,0x1000);
    (iVar8 + 0xf6) = uVar3;
    (iVar8 + 0xf8) = uVar7;
  }
  (iVar8 + 0xe8) = (iVar8 + 0xf6);
  pass1_1018_0ac0((iVar8 + 0xf2),param_1 & 0xffff | uVar9 << 0x10)
  ;
  uVar11 = pass1_1018_0b08((iVar8 + 0xf2));
  pBVar4 = (bool *)uVar11;
  ppcVar2 = (code **)(param_1 + 0x14);
  (**ppcVar2)();
  ppcVar2 = (code **)((iVar8 + 0xf2) + 0x10);
  (**ppcVar2)();
  MoveWindow16(0x1018,0x1,pBVar4[0x3],pBVar4[0x2],pBVar4[0x1],*pBVar4);
  pass1_1008_3e0e(param_1);
  UpdateWindow16(0x1008);
  return;
}


fn win_ui_op_1020_36f6(param_1: u32,param_2: i16,short param_3)
{
  let iVar1: i16;
  code **ppcVar2;
  let uVar3: u32;
  char *pcVar4;
  let uVar5: u16;
  let uVar6: u16;
  SEGPTR lp_string;
  let iVar7: i16;
  let uVar8: u16;
  HWND16 hwnd;
  char *pcVar9;
  let id: i16;
  let puStack1034: *mut u8;
  char local_406 [0x400];
  let uStack6: u32;
  
  iVar7 = param_1;
  uVar8 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (iVar7 + 0x8) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  uStack6 = pass1_1018_1e78((iVar7 + 0x8),-0x1);
  uVar6 = (uStack6 >> 0x10);
  GetWindowText16(0x1018,0x3ff,local_406);
  pcVar4 = pass1_1000_472c(CONCAT22(param_3,local_406),':');
  puStack1034 = CONCAT22(uVar6,pcVar4 + 0x2);
  *puStack1034 = 0x0;
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_406,param_3);
  ppcVar2 = (code **)((iVar7 + 0x18) + 0x18);
  (**ppcVar2)();
  uVar3 = (iVar7 + 0x8);
  iVar1 = (uVar3 + 0x4a);
  uVar3 = (uStack6 + 0x2);
  SetDlgItemText16(0x1010,uVar3,(SEGPTR)(uVar3 >> 0x10));
  uVar3 = (uStack6 + 0xa);
  SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,uVar3,
                   (SEGPTR)(uVar3 >> 0x10));
  uVar3 = (uStack6 + 0x12);
  SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,uVar3,
                   (SEGPTR)(uVar3 >> 0x10));
  uVar3 = (uStack6 + 0xe);
  SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,uVar3,
                   (SEGPTR)(uVar3 >> 0x10));
  if (iVar1 != 0x0) {
    hwnd = 0x1018;
    uVar5 = pass1_1018_1f1a((iVar7 + 0x8),(uStack6 + 0x1a));
    if (uVar5 != 0x0) {
      uVar3 = (uStack6 + 0x16);
      id = uVar3;
      lp_string = (SEGPTR)(uVar3 >> 0x10);
      goto LAB_1020_3846;
    }
  }
  hwnd = 0x1010;
  pcVar9 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  lp_string = (SEGPTR)(pcVar9 >> 0x10);
  id = pcVar9;
LAB_1020_3846:
  SetDlgItemText16(hwnd,id,lp_string);
  if (*(long *)(iVar7 + 0x1c) != 0x0) {
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,*(INT16 *)(uStack6 + 0x1a));
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    return;
  }
  InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)0x0,0x0);
  return;
}


fn window_op_1020_38aa(astruct *param_1)
{
  code **ppcVar1;
  let uVar2: u16;
  astruct_160 *paVar3;
  let uVar4: u32;
  let in_DX: *mut u8
  let uVar5: u16;
  let extraout_DX: *mut u8
  let puVar6: *mut u8
  let extraout_DX_00: *mut u8
  let puVar7: *mut u8
  let uVar8: u16;
  let extraout_DX_01: u16;
  let unaff_DI: i16;
  HWND16 HVar9;
  let unaff_SS: u16;
  let puVar10: *mut u16;
  let uVar11: u16;
  let uVar12: u16;
  RECT16 local_12;
  let iStack14: i16;
  let iStack12: i16;
  RECT16 local_a;
  let iStack6: i16;
  let iStack4: i16;
  
  uVar11 = param_1;
  uVar12 = (param_1 >> 0x10);
  create_window_ex_1008_9760(param_1,0x1008);
  puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,unaff_SS,in_DX,unaff_DI);
  uVar5 = (puVar10 >> 0x10);
  (uVar11 + 0xfa) = puVar10;
  (uVar11 + 0xfc) = uVar5;
  (uVar11 + 0xe0) = (uVar11 + 0xfa);
  (uVar11 + 0xe2) = uVar5;
  if ((uVar12 | uVar11) == 0x0) {
    uVar2 = 0x0;
    uVar8 = 0x0;
  }
  else {
    uVar2 = uVar11 + 0xf2;
    uVar8 = uVar12;
  }
  ppcVar1 = (code **)((uVar11 + 0xfa) + 0x4);
  (**ppcVar1)(0x1010,(uVar11 + 0xfa),0x0,uVar2,uVar8);
  puVar7 = extraout_DX;
  pass1_1018_1a8e((uVar11 + 0xfa),extraout_DX,unaff_DI,unaff_SS);
  mem_op_1000_179c(0x20,puVar7,0x1000);
  puVar6 = (puVar7 | uVar2);
  if (puVar6 == 0x0) {
    (uVar11 + 0xf6) = 0x0;
  }
  else {
    unk_draw_op_1020_3da4((astruct_24 *)CONCAT22(puVar7,uVar2),(ULONG)param_1);
    (uVar11 + 0xf6) = uVar2;
    *(uchar **)(uVar11 + 0xf8) = extraout_DX_00;
    puVar6 = extraout_DX_00;
  }
  uVar4 = (uVar11 + 0xf6);
  (uVar11 + 0xe8) = uVar4;
  mem_op_1000_179c(0x42,puVar6,0x1000);
  paVar3 = (astruct_160 *)uVar4;
  puVar7 = (puVar6 | paVar3);
  if (puVar7 == 0x0) {
    (uVar11 + 0x102) = 0x0;
  }
  else {
    pass1_1008_3bd6(paVar3,puVar6,0x0,0xf1,0x0,0x1ac01ad,
                    CONCAT22((uVar11 + 0x8),0xf4),puVar7,unaff_SS);
    *(astruct_160 **)(uVar11 + 0x102) = paVar3;
    *(uchar **)(uVar11 + 0x104) = puVar7;
  }
  HVar9 = 0x1000;
  mem_op_1000_179c(0x42,puVar7,0x1000);
  uVar8 = puVar7 | paVar3;
  if (uVar8 == 0x0) {
    (uVar11 + 0x106) = 0x0;
  }
  else {
    HVar9 = 0x1008;
    pass1_1008_3bd6(paVar3,puVar7,0x0,0xf500f1,0x0,0x1ae01af,
                    CONCAT22((uVar11 + 0x8),0xf5),uVar8,unaff_SS);
    *(astruct_160 **)(uVar11 + 0x106) = paVar3;
    (uVar11 + 0x108) = uVar8;
  }
  uVar4 = (uVar11 + 0xfa);
  ppcVar1 = (code **)((uVar11 + 0xfa) + 0x10);
  (**ppcVar1)(HVar9,uVar4,(uVar4 >> 0x10));
  puVar7 = paVar3->field_0x2;
  uVar8 = MoveWindow16(HVar9,0x1,*(INT16 *)&paVar3->field_0x6,*(INT16 *)&paVar3->field_0x4
                       ,puVar7,*(bool *)paVar3);
  HVar9 = 0x1000;
  mem_op_1000_179c(0x8e,puVar7,0x1000);
  uVar2 = puVar7 | uVar8;
  if (uVar2 == 0x0) {
    (uVar11 + 0x10a) = 0x0;
  }
  else {
    HVar9 = &PTR_LOOP_1050_1040;
    get_sys_metrics_1040_7728
              ((astruct_57 *)CONCAT22(puVar7,uVar8),0x1,0x0,0xfc0,
               (uVar11 + 0x8));
    (uVar11 + 0x10a) = uVar8;
    (uVar11 + 0x10c) = uVar2;
  }
  uVar4 = (uVar11 + 0x10a);
  (uVar4 + 0x74) = 0x1;
  uVar4 = (uVar11 + 0x10a);
  ppcVar1 = (code **)((uVar11 + 0x10a) + 0x8);
  (**ppcVar1)(HVar9,uVar4,(uVar4 >> 0x10));
  if (((uVar11 + 0x10c) | (uVar11 + 0x10a)) != 0x0) {
    GetWindowRect16(HVar9,&local_a);
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_12);
    iStack12 -= local_12.y;
    iStack14 = iStack6 - local_a.x;
    local_12.x = local_a.x;
    local_12.y = iStack4 + 0x3;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x44,iStack12,iStack14,local_12.y,
                   local_a.x,0x0);
  }
  return;
}


fn post_msg_1020_4394(param_1: u32,param_2: u16,HWND16 param_3)
{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  iVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  if (param_2 == 0x10) {
    if ((iVar2 + 0x34) != 0x0) {
      PostMessage16(param_3,0x0,0x0,0x11100f6);
      return;
    }
  }
  else {
    if (param_2 < 0x11) {
      if ((char)param_2 == '\x01') {
        (iVar2 + 0x18) = 0x0;
        return;
      }
      if ((char)param_2 == '\v') {
        uVar1 = (iVar2 + 0x2c);
        (uVar1 + 0xe) = (iVar2 + -0xda);
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_1020_43f6(astruct *param_1,uchar *param_2,param_3: u16)
{
  code **ppcVar1;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let unaff_DI: i16;
  let puVar7: *mut u16;
  let lVar8: i32;
  let uVar9: u16;
  astruct_282 *iVar9;
  
  iVar9 = (astruct_282 *)param_1;
  uVar9 = (param_1 >> 0x10);
  create_window_ex_1008_9760(param_1,0x1008);
  get_dc_1018_4db0(iVar9->field_0xfa,iVar9->field_0x8,0x1018);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_3,param_2,unaff_DI);
  &iVar9->field_0x10e = puVar7;
  (&iVar9->field_0x10e + 0x2) = (puVar7 >> 0x10);
  if (param_1 == (astruct *)0x0) {
    iVar2 = 0x0;
    uVar4 = 0x0;
  }
  else {
    iVar2 = &iVar9->field_0xe2;
    uVar4 = uVar9;
  }
  ppcVar1 = (code **)(*iVar9->field_0x10e + 0x4);
  lVar8 = (**ppcVar1)(0x1010,iVar9->field_0x10e,0xb,iVar2,uVar4);
  mem_op_1000_179c(0x30,(lVar8 >> 0x10),0x1000);
  uVar5 = (lVar8 >> 0x10);
  uVar3 = lVar8;
  uVar6 = uVar5 | uVar3;
  if (lVar8 == 0x0) {
    &iVar9->field_0xf6 = 0x0;
  }
  else {
    pass1_1020_62e0(uVar3,uVar5,iVar9->field_0x8,param_3);
    iVar9->field_0xf6 = uVar3;
    iVar9->field_0xf8 = uVar6;
  }
  ui_op_1020_536e(param_1,0x0,-0x1,0x3,uVar6);
  return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
mixed_menu_op_1020_44ec
          (param_1: u32,param_2: u16,param_3: i16,HMENparam_4: u16,HMENparam_5: u16,
          param_6: u16)

{
  let uVar1: u32;
  let uVar2: u16;
  let UVar3: u16;
  let Bvar4: bool;
  HMENU16 HVar5;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u32;
  let in_DX: *mut u8
  let puVar9: *mut u8
  let iVar10: i16;
  let unaff_DI: i16;
  let uVar11: u16;
  HMENU16 HVar12;
  ulet in_AF: u8;
  let local_138: [u16;0x2];
  let local_134: [u16;0x2];
  let puStack304: *mut u16;
  let uStack300: u32;
  let uStack296: u32;
  let uStack292: u32;
  char *pcStack286;
  let uStack278: u32;
  let BStack270: bool;
  let uStack268: u32;
  let local_108: [u32;0x40];
  let uStack8: u16;
  let puStack6: *mut u16;
  
  uVar11 = (param_1 >> 0x10);
  iVar10 = param_1;
  HVar12 = param_5;
  if ((iVar10 + 0x106) != 0x0) {
    if (*(HMENU16 *)(iVar10 + 0x106) == param_4) {
      puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
      uVar1 = (iVar10 + 0x108);
      uStack8 = (uVar1 + 0x2e);
      uVar1 = (iVar10 + 0x108);
      uVar11 = (uVar1 >> 0x10);
      iVar10 = uVar1;
      uStack296 = *(char **)(iVar10 + 0x42);
      puVar9 = *(uchar **)(iVar10 + 0x44);
      uStack296._3_1_ = (byte)(uStack296 >> 0x18);
      uVar6 = uStack296._3_1_;
      pcStack286 = uStack296;
      uStack268 = uStack296;
      if (uStack296._3_1_ == 0x0) {
        uVar2 = pass1_1020_bd80(uStack8);
        HVar12 = 0x1000;
        unk_str_op_1000_3d3e
                  (CONCAT22(param_6,local_108),CONCAT22(puVar9,uVar2));
      }
      else {
        pass1_1030_8344(_PTR_LOOP_1050_5748,
                        (_PTR_LOOP_1050_5748 >> 0x10),
                        uStack296 & 0xffff | ZEXT24(puVar9) << 0x10);
        uStack296 = CONCAT22(puVar9,uVar6);
        HVar12 = 0x1010;
        pass1_1010_c3c2(puStack6,(puStack6 >> 0x10),
                        CONCAT22(param_6,local_108),CONCAT22(puVar9,uVar6),puVar9,in_AF,
                        param_6);
      }
      BStack270 = ModifyMenu16(HVar12,local_108,param_6,0x76,0x0);
      UVar3 = GetMenuState16((HMENU16)s_tile2_bmp_1050_1538,0x0,0x13c);
      if (UVar3 != 0xffff) {
        DeleteMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0,0x13c);
      }
      HVar12 = 0x1008;
      BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x20);
      if (BVar4 != 0x0) {
        uStack296 = load_string_1010_847e
                              (_PTR_LOOP_1050_14cc,
                               (_PTR_LOOP_1050_14cc >> 0x10),0x1010);
        HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
        InsertMenu16(0x1010,uStack296,(uStack296 >> 0x10),0x13c,
                     0x400);
      }
      if (((s_VrMode_1050_42ca + 0x8) + uStack8 * 0x2) == 0x0) {
        UVar3 = 0x1;
        param_4 = 0x77;
        goto LAB_1020_464c;
      }
      param_4 = 0x77;
    }
    else {
      HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
      HVar5 = GetSubMenu16(param_5,0x1);
      uStack296 = (uStack296 & 0xffff0000 | HVar5);
      if (HVar5 != param_4) goto LAB_1020_479e;
      EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x1,0x200);
      EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x1,0x201);
      EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x1,0x202);
      uVar1 = (iVar10 + 0x108);
      uVar8 = (uVar1 + 0x42);
      uStack292 = uVar8;
      pass1_1030_8344(_PTR_LOOP_1050_5748,
                      (_PTR_LOOP_1050_5748 >> 0x10),uVar8);
      uVar6 = uVar8;
      pcStack286 = (uVar8 & 0xffff | ZEXT24(in_DX) << 0x10);
      if ((in_DX | uVar6) == 0x0) {
        return;
      }
      uStack278 = (uVar6 + 0x2e);
      if (((uVar6 + 0x30) | uStack278) == 0x0) {
        return;
      }
      uStack268 = *(char **)(uStack278 + 0x200);
      local_108[0] = struct_op_1030_73a8(CONCAT13((char)(in_DX >> 0x8),
                                                  CONCAT12((char)in_DX,uVar6)));
      uVar11 = (local_108[0] >> 0x10);
      puStack6 = *(u16 **)(local_108[0] + 0x1c);
      uVar6 = (local_108[0] + 0x1e);
      if ((uVar6 | puStack6) != 0x0) {
        uStack268 = (puStack6 & 0xffff | uVar6 << 0x10);
      }
      uStack268._2_2_ &= 0xff;
      if (uStack268 != 0x1) {
        return;
      }
      if ((uStack268 & 0xff0000) != 0x0) {
        return;
      }
      local_134[0] = pass1_1030_6fa0(pcStack286);
      HVar12 = 0x1008;
      BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,local_134[0],0x3f);
      if (BVar4 != 0x0) {
        HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
        BVar4 = EnableMenuItem16(0x1008,0x0,0x201);
      }
      if (*(long *)(pcStack286 + 0x36) != 0x0) {
        BVar4 = EnableMenuItem16(HVar12,0x0,0x202);
      }
      HVar12 = 0x1030;
      pass1_1030_69cc(pcStack286,BVar4,uStack268._2_2_,param_6);
      if (BVar4 == 0x0) {
        return;
      }
      param_4 = 0x200;
    }
    UVar3 = 0x0;
    goto LAB_1020_464c;
  }
LAB_1020_479e:
  iVar7 = param_3 + -0x1;
  if (iVar7 == 0x0) {
    pass1_1018_2504(0x0,in_DX);
    if (iVar7 == 0x0) {
      EnableMenuItem16(0x1018,0x401,0x0);
LAB_1020_47e3:
      HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
      UVar3 = 0x401;
      goto LAB_1020_464c;
    }
    HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
    EnableMenuItem16(0x1018,0x400,0x0);
  }
  else {
    if (param_3 == 0x2) {
      uVar2 = pass1_1020_64d4((iVar10 + 0xf6),0x2);
      if (uVar2 == 0x0) {
        EnableMenuItem16(HVar12,0x401,0x0);
      }
      else {
        EnableMenuItem16(HVar12,0x400,0x0);
      }
      HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
      EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,CONCAT11(0x4,uVar2 == 0x0),0x1);
      if ((PTR_LOOP_1050_0010 != 0x0) || (*(long *)(iVar10 + 0x102) == 0x0))
      goto LAB_1020_47e3;
    }
    else {
      if (param_3 == 0x3) {
        local_138[0] = 0x0;
        local_134[0] = 0x0;
        HVar12 = 0x1010;
        puStack304 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,in_DX,unaff_DI);
        uVar11 = (puStack304 >> 0x10);
        uStack300 = (puStack304 + 0x20);
        uVar6 = (puStack304 + 0x22);
        if ((uVar6 | uStack300) != 0x0) {
          HVar12 = 0x1030;
          pass1_1030_8308(_PTR_LOOP_1050_5748,
                          (_PTR_LOOP_1050_5748 >> 0x10),
                          CONCAT22(param_6,local_134),
                          CONCAT22(param_6,local_138),
                          uStack300 & 0xffff | uVar6 << 0x10,local_134,
                          uVar6);
          local_138[0] = (puStack304 + 0x1e);
        }
        uStack296 = (uStack296 & 0xffff0000);
        do {
          CheckMenuItem16(HVar12,0x400,uStack296);
          HVar12 = (HMENU16)s_tile2_bmp_1050_1538;
          EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x401,uStack296);
          uStack296 = (uStack296 & 0xffff0000 |
                              (uStack296 + 0x1));
        } while ((uStack296 + 0x1) < 0x5);
        CheckMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x408,local_138[0]);
        for (uStack296 = (uStack296 & 0xffff0000);
            uStack296 <= local_134[0];
            uStack296 = (uStack296 & 0xffff0000 |
                                (uStack296 + 0x1))) {
          EnableMenuItem16((HMENU16)s_tile2_bmp_1050_1538,0x400,uStack296);
        }
        return;
      }
      if (param_3 != 0x4) {
        return;
      }
      param_4 = 0x2;
    }
  }
  UVar3 = 0x400;
LAB_1020_464c:
  EnableMenuItem16(HVar12,UVar3,param_4);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
win_sys_op_1020_493c
          (param_1: *mut u32,param_2: u16,uchar *param_3,param_4: u16,HCURSOR16 param_5
          ,WNDCLASS16 *param_6)

{
  code **ppcVar1;
  HCURSOR16 HVar2;
  let puVar3: *mut u8;
  let iVar4: i16;
  let puVar5: u32;
  let uVar6: u16;
  let puVar7: *mut u8
  let puVar8: *mut u8
  let uVar9: u16;
  let uVar10: u16;
  let unaff_DI: i16;
  let uVar11: u16;
  ulet in_AF: u8;
  let uVar12: u32;
  let puVar13: *mut u16;
  astruct_100 *paVar14;
  char *pcVar15;
  let uVar16: u8;
  let uVar17: u16;
  let uVar18: u16;
  let uVar19: u16;
  let local_356: [u32;0x42];
  let local_24e: u16;
  let puStack588: *mut u8
  let local_144: u32;
  let uStack320: u32;
  let local_13c: u32;
  let uStack42: u16;
  let uStack38: u32;
  let uStack34: u16;
  let puStack32: *mut u8
  let uStack30: u32;
  let uStack26: u32;
  let uStack22: u32;
  astruct_43 *paStack18;
  let puStack14: *mut u8;
  let puStack12: *mut u8
  let uStack10: u16;
  let uStack6: u32;
  
  if (param_2 == 0xe9) {
    return;
  }
  uVar9 = param_1;
  puVar8 = (param_1 >> 0x10);
  if (param_2 < 0xea) {
    if (false) {
      return;
    }
    switch(param_2) {
    case 0x69:
      iVar4 = 0x0;
      break;
    case 0x6a:
      iVar4 = 0x1;
      break;
    case 0x6b:
      iVar4 = 0x2;
      break;
    case 0x6c:
      iVar4 = 0x3;
      break;
    case 0x6d:
      iVar4 = 0x4;
      break;
    default:
      return;
    case 0x77:
      if (((uVar9 + 0x10a) | (uVar9 + 0x108)) == 0x0) {
        return;
      }
      uVar12 = (uVar9 + 0x108);
      uVar19 = ((s_VrMode_1050_42ca + 0x8) +
                        (uVar12 + 0x2e) * 0x2);
      uStack26 = (uStack26 & 0xffff0000 | uVar19);
      if (uVar19 == 0x0) {
        return;
      }
      paStack18 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,param_6);
      WinHelp16(0x1010,(LPCSTR)uStack26,
                CONCAT11((u8)((LPCSTR)uStack26 >> 0xf),
                         (u8)((LPCSTR)uStack26 >> 0xf)),
                CONCAT22(paStack18,0x1));
      return;
    case 0x78:
      puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x45,param_6,param_3,unaff_DI)
      ;
      puStack588 = (puVar13 >> 0x10);
      local_24e = puVar13;
      enum_child_windows_1010_01be(0x1010);
      return;
    }
    set_cursor_1020_5764(param_1,iVar4,param_6);
    return;
  }
  if (param_2 == 0x132) {
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0xffff;
    goto LAB_1020_4ef8;
  }
  if (param_2 < 0x133) {
    if (param_2 == 0x102) {
      uVar16 = 0x0;
      mem_op_1000_179c(0xb4,param_3,0x1000);
      puVar8 = (param_3 | param_2);
      uStack34 = param_2;
      puStack32 = param_3;
      if (puVar8 == 0x0) {
        iVar4 = 0x0;
        puVar8 = 0x0;
      }
      else {
        uVar16 = 0x40;
        iVar4 = string_1040_8520((astruct_57 *)CONCAT22(param_3,param_2),
                                 PTR_LOOP_1050_0396,0x31,0x2,0x57b,0x62b,puVar8,
                                 param_6);
      }
      local_144 = CONCAT22(puVar8,iVar4);
      ppcVar1 = (code **)(*local_144 + 0x74);
      (**ppcVar1)(uVar16,iVar4,puVar8);
      uStack320 = CONCAT22(uStack320._2_2_,iVar4);
      if (iVar4 != 0x1) {
        return;
      }
      pass1_1028_837e((astruct_100 *)CONCAT22(param_6,&local_13c),param_6,in_AF);
LAB_1020_4b6c:
      fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_6,&local_13c));
      return;
    }
    if (param_2 < 0x103) {
      if (false) {
        return;
      }
      switch(param_2) {
      case 0xf0:
        ui_op_1020_536e(param_1,0x0,-0x1,0x1,param_3);
        return;
      default:
        return;
      case 0xf6:
        if ((uVar9 + 0x116) != 0x0) {
          if (param_1 == 0x0) {
            iVar4 = 0x0;
            param_3 = 0x0;
          }
          else {
            iVar4 = uVar9 + 0xe2;
            param_3 = puVar8;
          }
          local_356[0] = CONCAT22(param_3,iVar4);
          pass1_1010_1ea6(_PTR_LOOP_1050_02a0,CONCAT22(param_3,iVar4),param_6);
          (uVar9 + 0x116) = 0x0;
        }
        iVar4 = 0x12;
        break;
      case 0xf7:
        unk_win_op_1010_7300((uVar9 + 0x10e),0x0,0x9,0x0);
        return;
      case 0xfb:
        local_144 = 
                    mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_3,
                                    unaff_DI);
        uStack320 = 
                    mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_6,
                                    (local_144 >> 0x10),unaff_DI);
        pcVar15 = pass1_1010_375e(uStack320);
        pass1_1010_c25e(local_144,(local_144 >> 0x10),pcVar15,
                        pcVar15,(pcVar15 >> 0x10),unaff_DI,
                        param_6);
        return;
      case 0xfc:
        post_msg_1020_55b0(param_1,param_6);
        return;
      case 0x101:
        uStack26 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,param_3,
                                   unaff_DI);
        uVar11 = (uStack26 >> 0x10);
        uStack22 = (uStack26 + 0x24);
        puVar7 = *(uchar **)(uStack26 + 0x26);
        uStack22._0_2_ = puVar7 | uStack22;
        if (uStack22 == 0x0) {
          uVar16 = 0x0;
          mem_op_1000_179c(0xb4,puVar7,0x1000);
          puVar8 = (puVar7 | uStack22);
          uStack34 = uStack22;
          puStack32 = puVar7;
          if (puVar8 == 0x0) {
            puVar5 = 0x0;
            puVar8 = 0x0;
          }
          else {
            uVar16 = 0x40;
            puVar5 = 
                     string_1040_8520((astruct_57 *)CONCAT22(puVar7,uStack22),
                                      PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x730,
                                      puVar8,param_6);
          }
          uStack30 = CONCAT22(puVar8,puVar5);
LAB_1020_4c5f:
          ppcVar1 = (code **)(*puVar5 + 0x74);
          (**ppcVar1)(uVar16,puVar5,puVar8);
          return;
        }
        uVar12 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar9 + 0x8),0xe,
                                 puVar7,uVar9,&PTR_LOOP_1050_1038,
                                 param_6);
        paStack18 = (astruct_43 *)
                    mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x43,param_6,
                                    (uVar12 >> 0x10),unaff_DI);
        uVar11 = (paStack18 >> 0x10);
        iVar4 = paStack18;
        puStack14 = (iVar4 + 0xa);
        uStack10 = (iVar4 + 0xc);
        uVar9 = (iVar4 + 0xe);
        uStack6 = CONCAT22(uStack6._2_2_,uVar9);
        if ((iVar4 + 0x10) != 0x0) {
          return;
        }
        pass1_1028_84ca((astruct_100 *)CONCAT22(param_6,&local_13c),uStack22,uVar9,
                        uStack10,puStack14,param_6,in_AF);
        goto LAB_1020_4b6c;
      }
    }
    else {
      if (param_2 != 0x106) {
        if (param_2 < 0x107) {
          if (param_2 == 0x103) {
            local_144 = 
                        mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,param_3,
                                        unaff_DI);
            uVar11 = (local_144 >> 0x10);
            uStack320 = *(char **)(local_144 + 0x24);
            puStack32 = *(uchar **)(local_144 + 0x26);
            uStack34 = puStack32 | uStack320;
            if (uStack34 != 0x0) {
              uVar12 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar9 + 0x8),0xf,
                                       puStack32,uVar9,&PTR_LOOP_1050_1038
                                       ,param_6);
              local_13c = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x42,param_6,
                                          (uVar12 >> 0x10),unaff_DI);
              uStack42 = (local_13c + 0xa);
              if (uStack42 == 0x0) {
                return;
              }
              pass1_1030_e63e((astruct_100 *)CONCAT22(param_6,&local_24e),uStack42,param_6
                              ,in_AF);
              fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_6,&local_24e));
              return;
            }
            uVar16 = 0x0;
            mem_op_1000_179c(0xb4,puStack32,0x1000);
            puVar8 = (puStack32 | uStack34);
            if (puVar8 == 0x0) {
              puVar5 = 0x0;
              puVar8 = 0x0;
            }
            else {
              uVar16 = 0x40;
              puVar5 = 
                       string_1040_8520((astruct_57 *)CONCAT22(puStack32,uStack34),
                                        PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x730,
                                        puVar8,param_6);
            }
            uStack38 = CONCAT22(puVar8,puVar5);
          }
          else {
            if (param_2 != 0x104) {
              return;
            }
            puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_3,
                                      unaff_DI);
            puStack32 = (puVar13 >> 0x10);
            uStack34 = puVar13;
            local_24e = uStack34;
            puStack588 = puStack32;
            pass1_1010_af66(puVar13,puStack32);
            local_144 = CONCAT22(local_144._2_2_,uStack34);
            if (uStack34 != 0x0) {
              uVar16 = 0x0;
              mem_op_1000_179c(0xb4,puStack32,0x1000);
              puVar8 = (puStack32 | uStack34);
              if (puVar8 == 0x0) {
                iVar4 = 0x0;
                puVar8 = 0x0;
              }
              else {
                uVar16 = 0x40;
                iVar4 = string_1040_8520((astruct_57 *)CONCAT22(puStack32,uStack34),
                                         PTR_LOOP_1050_0396,0x31,0x2,0x57b,0x62c,
                                         puVar8,param_6);
              }
              uStack320 = CONCAT22(puVar8,iVar4);
              ppcVar1 = (code **)(*uStack320 + 0x74);
              (**ppcVar1)(uVar16,iVar4,puVar8);
              local_13c = CONCAT22(local_13c._2_2_,iVar4);
              if (iVar4 != 0x1) {
                return;
              }
              uVar16 = (u8)(param_6 >> 0x8);
              paVar14 = pass1_1030_e79a((astruct_100 *)
                                        CONCAT13(uVar16,CONCAT12((char)param_6,local_356))
                                        ,param_6,in_AF);
              uVar9 = (paVar14 >> 0x10);
              puVar5 = local_356;
              fn_ptr_1030_835a(_PTR_LOOP_1050_5748,
                               CONCAT13(uVar16,CONCAT12((char)param_6,puVar5)));
              win_1008_5c5c(param_6,puVar5,uVar9,_PTR_LOOP_1050_02a0,0x1e6);
              return;
            }
            uVar16 = 0x0;
            mem_op_1000_179c(0xb4,puStack32,0x1000);
            puVar8 = (puStack32 | uStack34);
            if (puVar8 == 0x0) {
              puVar5 = 0x0;
              puVar8 = 0x0;
            }
            else {
              uVar16 = 0x40;
              puVar5 = 
                       string_1040_8520((astruct_57 *)CONCAT22(puStack32,uStack34),
                                        PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x731,
                                        puVar8,param_6);
            }
            local_356[0] = CONCAT22(puVar8,puVar5);
          }
          goto LAB_1020_4c5f;
        }
        if (param_2 == 0x12f) {
          pass1_1020_61c4(uVar9,puVar8,CONCAT22(param_6,&local_144),
                          CONCAT22(param_6,&local_24e),param_3,unaff_DI,
                          param_6);
          iVar4 = local_24e + 0x6a;
        }
        else {
          if (param_2 != 0x130) {
            if (param_2 != 0x131) {
              return;
            }
            uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x2);
            if (uVar6 == 0x0) {
              return;
            }
            iVar4 = 0x7;
            goto LAB_1020_49b7;
          }
          pass1_1020_61c4(uVar9,puVar8,CONCAT22(param_6,&local_144),
                          CONCAT22(param_6,&local_24e),param_3,unaff_DI,
                          param_6);
          iVar4 = local_24e + 0x68;
        }
        uStack320 = CONCAT22(uStack320._2_2_,iVar4);
        if (0x6d < iVar4) {
          return;
        }
        if (iVar4 < 0x69) {
          return;
        }
        ppcVar1 = (code **)(*param_1 + 0x40);
        (**ppcVar1)();
        return;
      }
      iVar4 = 0x13;
    }
LAB_1020_49b7:
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar9 + 0x8),iVar4,param_3,
                    uVar9,&PTR_LOOP_1050_1038,param_6);
    return;
  }
  if (param_2 == 0x1c8) {
    SendMessage16(param_5,0x0,0x0,0x1110072);
    return;
  }
  if (0x1c8 < param_2) {
    if (param_2 == 0x1ca) {
      local_144 = 
                  mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_3,unaff_DI
                                 );
      uStack320 = 
                  pass1_1010_c234(local_144,(local_144 >> 0x10),
                                  unaff_DI,param_6);
      uVar10 = (uStack320 >> 0x10);
      uVar19 = uStack320;
      if ((uchar *)(uVar10 | uVar19) == 0x0) {
        return;
      }
      local_13c = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_6,
                                  (uVar10 | uVar19),unaff_DI);
      param_3 = (local_13c >> 0x10);
      pass1_1010_3770(local_13c,CONCAT22(uVar10,uVar19),param_3);
      iVar4 = 0x3;
    }
    else {
      uVar17 = (_PTR_LOOP_1050_5748 >> 0x10);
      uVar6 = _PTR_LOOP_1050_5748;
      if (param_2 == 0x200) {
        uVar12 = (uVar9 + 0x108);
        uVar11 = (uVar12 >> 0x10);
        iVar4 = uVar12;
        uStack26 = *(u16 **)(iVar4 + 0x42);
        param_3 = *(uchar **)(iVar4 + 0x44);
        uStack26._3_1_ = (byte)(uStack26 >> 0x18);
        puStack14 = uStack26._3_1_;
        if (uStack26._3_1_ != 0x5) {
          return;
        }
        pass1_1030_8344(uVar6,uVar17,uStack26 & 0xffff | ZEXT24(param_3) << 0x10);
        iVar4 = 0x25;
        PTR_LOOP_1050_5f0c = puStack14;
        PTR_LOOP_1050_5f0e = param_3;
        puStack12 = param_3;
      }
      else {
        if (param_2 == 0x201) {
          uVar12 = (uVar9 + 0x108);
          uVar11 = (uVar12 >> 0x10);
          iVar4 = uVar12;
          uStack26 = *(u16 **)(iVar4 + 0x42);
          param_3 = *(uchar **)(iVar4 + 0x44);
          uStack26._3_1_ = (byte)(uStack26 >> 0x18);
          puStack14 = uStack26._3_1_;
          if (uStack26._3_1_ != 0x5) {
            return;
          }
          pass1_1030_8344(uVar6,uVar17,uStack26 & 0xffff | ZEXT24(param_3) << 0x10)
          ;
          iVar4 = 0x26;
          PTR_LOOP_1050_5f16 = puStack14;
          PTR_LOOP_1050_5f18 = param_3;
          puStack12 = param_3;
        }
        else {
          if (param_2 != 0x202) {
            if (param_2 != 0x203) {
              return;
            }
            if ((uVar9 + 0xf4) != 0x1) {
              return;
            }
            HVar2 = SetCursor16(param_5);
            *(HCURSOR16 *)(uVar9 + 0xee) = HVar2;
            (uVar9 + 0xf4) = 0x3;
            SetCapture16((HWND16)s_tile2_bmp_1050_1538);
            return;
          }
          uVar12 = (uVar9 + 0x108);
          uVar11 = (uVar12 >> 0x10);
          iVar4 = uVar12;
          uStack6 = (iVar4 + 0x42);
          param_3 = *(uchar **)(iVar4 + 0x44);
          uStack6._3_1_ = (byte)(uStack6 >> 0x18);
          puVar3 = uStack6._3_1_;
          if (uStack6._3_1_ != 0x5) {
            return;
          }
          pass1_1030_8344(uVar6,uVar17,uStack6 & 0xffff | ZEXT24(param_3) << 0x10);
          uStack22 = CONCAT22(param_3,puVar3);
          iVar4 = 0x27;
          PTR_LOOP_1050_5a68 = puVar3;
          PTR_LOOP_1050_5a6a = param_3;
        }
      }
    }
    goto LAB_1020_49b7;
  }
  if (false) {
    return;
  }
  switch(param_2) {
  case 0x133:
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0xffff;
    uVar11 = 0x0;
    break;
  case 0x134:
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0x1;
    goto LAB_1020_4ef8;
  case 0x135:
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0x1;
    uVar11 = 0x0;
    break;
  case 0x136:
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0xfffb;
    goto LAB_1020_4ef8;
  case 0x137:
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0xfffb;
    uVar11 = 0x0;
    break;
  case 0x138:
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar11 = 0x5;
LAB_1020_4ef8:
    uVar18 = 0x0;
    break;
  case 0x139:
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x3);
    if (uVar6 == 0x0) {
      return;
    }
    uVar18 = 0x5;
    uVar11 = 0x0;
    break;
  default:
    goto switchD_1020_518a_caseD_13a;
  case 0x13c:
    uVar6 = pass1_1020_64d4((uVar9 + 0xf6),0x2);
    if (uVar6 != 0x0) {
      iVar4 = 0x1a;
      goto LAB_1020_49b7;
    }
    goto switchD_1020_518a_caseD_13a;
  }
  pass1_1020_2a94((uVar9 + 0xce),CONCAT22(uVar11,uVar18),param_6);
switchD_1020_518a_caseD_13a:
  return;
}


fn win_ui_cursor_op_1020_522e(astruct_52 *param_1,param_2: u16,param_3: u16)
{
  let iVar1: i16;
  code **ppcVar2;
  let BVar3: bool;
  let in_DX: *mut u8
  let iVar4: i16;
  let unaff_DI: i16;
  let uVar5: u16;
  HCURSOR16 unaff_CS;
  let unaff_SS: u16;
  let puVar6: *mut u16;
  let uVar7: u8;
  let uVar8: u8;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  iVar1 = (iVar4 + 0xf4);
  if (iVar1 == 0x2) {
    SetCursor16(unaff_CS);
    (iVar4 + 0xee) = 0x0;
    (iVar4 + 0xf4) = 0x1;
    (iVar4 + 0x10c) = 0x0;
    ReleaseCapture16();
    return;
  }
  if (iVar1 == 0x3) {
    SetCursor16(unaff_CS);
    (iVar4 + 0xee) = 0x0;
    (iVar4 + 0xf4) = 0x1;
    (iVar4 + 0x10c) = 0x0;
    ReleaseCapture16();
    uVar7 = 0x0;
    uVar8 = 0x0;
    uVar5 = 0x0;
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x47,unaff_SS,in_DX,unaff_DI);
    pass1_1018_57e6(puVar6,CONCAT22(uVar5,CONCAT11(uVar8,uVar7)),unaff_SS);
    return;
  }
  BVar3 = menu_ui_op_1020_5bf2(param_1,param_2,param_3);
  if (BVar3 == 0x0) {
    ppcVar2 = (code **)((iVar4 + 0x4) + 0x60);
    (**ppcVar2)();
  }
  return;
}


fn ui_op_1020_536e(param_1: u32,param_2: u32,param_3: i16,param_4: i16,uchar *param_5)
{
  let piVar1: *mut i16;
  let UVar2: u16;
  code **ppcVar3;
  let uVar4: u16;
  let uVar5: u16;
  let UVar6: u16;
  let uVar7: u16;
  let puVar8: *mut u8
  let extraout_DX: *mut u8
  let puVar9: *mut u8
  let iVar10: i16;
  let puVar11: u32;
  let unaff_DI: i16;
  let uVar12: u16;
  let unaff_SS: u16;
  let puVar13: *mut u16;
  let uVar14: u32;
  let uVar15: u32;
  let uVar16: u8;
  let uVar17: u8;
  let uVar18: u16;
  let uVar19: u16;
  let puStack16: u32;
  
  uVar7 = param_4 - 0x1;
  uVar12 = (param_1 >> 0x10);
  iVar10 = param_1;
  if (uVar7 == 0x0) {
    if (*(long *)(iVar10 + 0xfe) == 0x0) {
      mem_op_1000_179c(0xfc,param_5,0x1000);
      uVar14 = CONCAT22(param_5 | uVar7,uVar7);
      if ((param_5 | uVar7) == 0x0) {
        (iVar10 + 0xfe) = 0x0;
      }
      else {
        piVar1 = (i16 *)(iVar10 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        uVar14 = unk_win_ui_op_1020_67ce
                           (CONCAT13((char)(param_5 >> 0x8),
                                     CONCAT12((char)param_5,uVar7)),
                            (iVar10 + 0xcc),param_1);
        (iVar10 + 0xfe) = uVar14;
        (iVar10 + 0x100) = (uVar14 >> 0x10);
      }
      pass1_1008_6978(param_1,0x0,(iVar10 + 0xfe),uVar14,
                      (uVar14 >> 0x10));
      uVar14 = (iVar10 + 0xfe);
      uVar18 = uVar14;
      uVar19 = (uVar14 >> 0x10);
      uVar14 = (iVar10 + 0xfe);
      uVar12 = (uVar14 >> 0x10);
      puVar11 = uVar14;
      goto LAB_1020_53f3;
    }
  }
  else {
    if (param_4 == 0x2) {
      uVar4 = param_3 + 0xc;
      puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uVar4,unaff_SS,param_5,unaff_DI);
      puVar9 = (puVar13 >> 0x10);
      uVar5 = pass1_1018_0afa(puVar13);
      if (uVar5 == 0x0) {
        piVar1 = (i16 *)(iVar10 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        UVar2 = (iVar10 + 0xcc);
        uVar12 = 0xfe;
        UVar6 = UVar2;
        mem_op_1000_179c(0xfe,puVar9,0x1000);
        puVar8 = (puVar9 | UVar6);
        if (puVar8 == 0x0) {
          UVar6 = 0x0;
          puVar8 = 0x0;
        }
        else {
          pass1_1020_289a((u16 *)
                          CONCAT13((char)(puVar9 >> 0x8),
                                   CONCAT12((char)puVar9,UVar6)),UVar2,param_1,unaff_SS);
        }
        puStack16 = CONCAT22(puVar8,UVar6);
        uVar16 = SUB21(puVar8,0x0);
        uVar17 = (u8)(puVar8 >> 0x8);
        pass1_1020_294a(CONCAT13(uVar17,CONCAT12(uVar16,UVar6)),
                        CONCAT22(param_2,uVar12),(param_2 >> 0x10),
                        puVar8,unaff_DI,unaff_SS);
        unaff_DI = (*puStack16 >> 0x10);
        iVar10 = *puStack16;
        ppcVar3 = (code **)(iVar10 + 0x8);
        uVar14 = (**ppcVar3)(0x1000,UVar6,puVar8,uVar4);
        pass1_1008_3e0e(CONCAT13(uVar17,CONCAT12(uVar16,UVar6)));
        pass1_1008_6978(param_1,UVar2,CONCAT22(puVar8,UVar6),uVar14,
                        (uVar14 >> 0x10));
        ppcVar3 = (code **)(iVar10 + 0xc);
        (**ppcVar3)(0x1008,UVar6,uVar16,0x1);
        puVar9 = extraout_DX;
      }
      else {
        uVar15 = pass1_1018_0ad4(puVar13);
        puVar9 = (uVar15 >> 0x10);
        pass1_1008_3e0e(uVar15);
      }
      pass1_1018_1662(puVar13,0x0,0x0,unaff_SS);
      BringWindowToTop16(0x1018);
      uVar4 = 0x1;
      iVar10 = 0x4;
      puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,unaff_SS,puVar9,unaff_DI);
      pass1_1010_089e(unaff_SS,puVar13,uVar4,iVar10);
      pass1_1010_089e(unaff_SS,puVar13,0x1,0x3);
      return;
    }
    uVar7 = param_4 - 0x3;
    if ((uVar7 == 0x0) && (*(long *)(iVar10 + 0x102) == 0x0)) {
      mem_op_1000_179c(0xfc,param_5,0x1000);
      puVar9 = (param_5 | uVar7);
      if (puVar9 == 0x0) {
        (iVar10 + 0x102) = 0x0;
      }
      else {
        piVar1 = (i16 *)(iVar10 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        pass1_1020_0dc4((u16 *)
                        CONCAT13((char)(param_5 >> 0x8),
                                 CONCAT12((char)param_5,uVar7)),(iVar10 + 0xcc)
                        ,param_1,unaff_SS);
        (iVar10 + 0x102) = uVar7;
        *(uchar **)(iVar10 + 0x104) = puVar9;
      }
      pass1_1008_6978(param_1,0x0,(iVar10 + 0x102),uVar7,puVar9);
      uVar14 = (iVar10 + 0x102);
      uVar18 = uVar14;
      uVar19 = (uVar14 >> 0x10);
      uVar14 = (iVar10 + 0x102);
      uVar12 = (uVar14 >> 0x10);
      puVar11 = uVar14;
LAB_1020_53f3:
      ppcVar3 = (code **)(*puVar11 + 0xc);
      (**ppcVar3)(0x8,uVar18,uVar19,0x5);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn post_msg_1020_55b0(param_1: u32,param_2: u16) -> u16

{
  code **ppcVar1;
  let uVar2: u16;
  let in_DX: *mut u8
  let puVar3: *mut u8
  let uVar4: u16;
  let extraout_DX: *mut u8
  let extraout_DX_00: *mut u8
  let unaff_DI: i16;
  let uVar5: u16;
  HWND16 hwnd;
  HWND16 hwnd_00;
  ulet in_AF: u8;
  let puVar5: *mut u16;
  char *pcVar6;
  let puVar6: u32;
  LRESULT LVar7;
  let uVar8: u8;
  let local_114: i16;
  let local_112: [u8;2];
  let iStack272: i16;
  let local_10e: i16;
  char local_10c [0x100];
  let puStack12: *mut u16;
  let iStack8: i16;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  puVar3 = (puStack6 >> 0x10);
  iStack8 = (puStack6 + 0x20);
  puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_2,puVar3,unaff_DI);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x100,local_10c,param_2);
  puVar5 = pass1_1008_9436((u16 *)CONCAT22(param_2,local_112));
  uVar8 = (u8)(param_2 >> 0x8);
  pcVar6 = pass1_1008_a8f4(puStack12,

                                   CONCAT13(uVar8,CONCAT12((char)param_2,&local_114)),
                                   CONCAT22(param_2,local_112),
                                   CONCAT22(param_2,&local_10e),
                                   (puVar5 >> 0x10),0x1008,param_2,in_AF);
  uVar2 = pcVar6;
  puVar3 = ((pcVar6 >> 0x10) | uVar2);
  uVar5 = (param_1 >> 0x10);
  hwnd_00 = 0x1008;
  if ((pcVar6 != 0x0) && (*pcVar6 != '\0')) {
    hwnd = 0x1000;
    mem_op_1000_179c(0xb4,puVar3,0x1000);
    if ((puVar3 | uVar2) == 0x0) {
      puVar6 = 0x0;
    }
    else {
      hwnd = &PTR_LOOP_1050_1040;
      puVar6 = pass1_1040_8478((astruct_57 *)CONCAT22(puVar3,uVar2),0x0,
                                        CONCAT13(uVar8,CONCAT12((char)param_2,
                                                                        local_10c)),pcVar6
                                        ,(param_1 + 0x8),
                                        puVar3 | uVar2);
    }
    uVar4 = (puVar6 >> 0x10);
    if (iStack272 == 0x0) {
      ppcVar1 = (code **)(*puVar6 + 0x74);
      (**ppcVar1)(hwnd,(puVar6 & 0xffff),uVar4);
      puVar3 = extraout_DX_00;
    }
    else {
      ppcVar1 = (code **)(*puVar6 + 0x6c);
      (**ppcVar1)(hwnd,(puVar6 & 0xffff),uVar4,local_112,param_2);
      puVar3 = extraout_DX;
    }
    if ((iStack8 == 0x0) || (hwnd_00 = hwnd, local_114 == 0x0)) {
      hwnd_00 = s_tile2_bmp_1050_1538;
      PostMessage16(hwnd,0x0,0x0,0x11100fc);
    }
  }
  if ((iStack8 != 0x0) && (local_114 != 0x0)) {
    LVar7 = SendMessage16(hwnd_00,0x0,0x0,CONCAT22(0x111,local_114));
    (param_1 + 0x112) = 0x1;
    return (LVar7 >> 0x10);
  }
  if (local_10e == 0x6) {
    PostMessage16(hwnd_00,0x0,0x0,0x11100b0);
    hwnd_00 = 0x1010;
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,puVar3,unaff_DI);
    puVar3 = (puVar5 >> 0x10);
    (puVar5 + 0x20) = 0x1;
  }
  if (local_10e == 0x15) {
    PostMessage16(hwnd_00,0x0,0x0,0x1110097);
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,puVar3,unaff_DI);
    puVar3 = (puVar5 >> 0x10);
    (puVar5 + 0x20) = 0x1;
  }
  return puVar3;
}


fn menu_ui_op_1020_5bf2(astruct_52 *param_1,HWND16 param_2,RECT16 *param_3) -> bool

{
  let uVar1: u32;
  let uVar2: u16;
  let BVar3: bool;
  RECT16 **ppRVar4;
  HMENU16 HVar5;
  let in_DX: u16;
  let uVar6: u16;
  astruct_52 *iVar5;
  let uVar7: u16;
  RECT16 *unaff_CS;
  RECT16 *instance;
  WNDCLASS16 *unaff_SS;
  let uVar8: u32;
  POlet local_10: i16;
  let iStack12: i16;
  let uStack10: u32;
  RECT16 *local_6;
  HWND16 HStack4;
  
  local_6 = param_3;
  HStack4 = param_2;
  uVar7 = (param_1 >> 0x10);
  iVar5 = (astruct_52 *)param_1;
  uVar2 = pass1_1020_64d4(iVar5->field_0xf6,0x2);
  uVar8 = in_DX << 0x10;
  instance = unaff_CS;
  if (uVar2 != 0x0) {
    uStack10 = pass1_1020_6498(iVar5->field_0xf6,0x2);
    instance = (RECT16 *)s_tile2_bmp_1050_1538;
    uVar8 = uStack10;
    BVar3 = PtInRect16(unaff_CS,(POINT16)CONCAT22(HStack4,local_6));
    if (BVar3 != 0x0) {
      PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1110131);
      return 0x1;
    }
  }
  uVar2 = pass1_1020_64d4(iVar5->field_0xf6,0x3);
  if (uVar2 == 0x0) {
    return 0x0;
  }
  ppRVar4 = &local_6;
  pt_in_rect_1020_5856(param_1,(POINT16 *)CONCAT22(unaff_SS,ppRVar4),ppRVar4);
  uVar6 = (uVar8 >> 0x10);
  iVar5->field_0x108 = ppRVar4;
  iVar5->field_0x10a = uVar6;
  if ((uVar6 | iVar5->field_0x108) == 0x0) {
    return 0x0;
  }
  if (iVar5->field_0x106 == 0x0) {
    HVar5 = LoadMenu16((HINSTANCE16)instance,(LPCSTR)s_TILEMENU_1050_43f0);
    iVar5->field_0x106 = HVar5;
    if (HVar5 == 0x0) {
      return 0x1;
    }
    instance = (RECT16 *)s_tile2_bmp_1050_1538;
    HVar5 = GetSubMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0);
    iVar5->field_0x106 = HVar5;
    if (HVar5 == 0x0) {
      return 0x1;
    }
  }
  uVar1 = &iVar5->field_0x108;
  uStack10._0_2_ = (uVar1 + 0x2e);
  iStack12 = 0x0;
  if (uStack10 == 0x42) {
    iStack12 = 0xc9;
  }
  else {
    if (((s_VrMode_1050_42ca + 0x8) + uStack10 * 0x2) == 0x0) {
      iStack12 = 0xc8;
    }
  }
  if (iStack12 != 0x0) {
    instance = (RECT16 *)0x1008;
    win_1008_5c7c(_PTR_LOOP_1050_02a0,CONCAT22(iStack12,0x1),unaff_SS,uStack10,
                  (uVar8 >> 0x10));
  }
  local_10.x = param_3;
  local_10.y = param_2;
  ClientToScreen16((HWND16)instance,&local_10);
  TrackPopupMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0,0x0,iVar5->field_0x8,0x0,local_10.y,
                   (RECT16 *)local_10.x);
  return 0x1;
}


fn win_ui_op_1020_5de8(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let uVar1: u16;
  let uVar2: u32;
  let puVar3: *mut u16;
  let in_DX: *mut u8
  let uVar4: u16;
  let unaff_DI: i16;
  let uVar5: u16;
  let puVar6: *mut u16;
  let uVar7: u8;
  let uVar8: u8;
  let uStack18: u16;
  let cStack15: u8;
  let local_6: u16;
  let uStack4: u16;
  
  ReleaseCapture16();
  uVar5 = (param_1 >> 0x10);
  SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  (param_1 + 0xee) = 0x0;
  (param_1 + 0xf4) = 0x1;
  local_6 = param_3;
  uStack4 = param_2;
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x47,param_4,in_DX,unaff_DI);
  uVar4 = (puVar6 >> 0x10);
  puVar3 = &local_6;
  pt_in_rect_1020_5856
            (param_1,(POINT16 *)
                     CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,puVar3)),
             puVar3);
  if ((uVar4 | puVar3) != 0x0) {
    uVar2 = (puVar3 + 0x21);
    uVar1 = puVar3[0x22];
    cStack15 = (uVar2 >> 0x18);
    if (cStack15 == '\x05') {
      uVar7 = (u8)uVar1;
      uVar8 = (u8)(uVar1 >> 0x8);
      uStack18 = uVar2;
      goto LAB_1020_5e62;
    }
  }
  uStack18 = 0x0;
  uVar7 = 0x0;
  uVar8 = 0x0;
LAB_1020_5e62:
  pass1_1018_57e6(puVar6,CONCAT13(uVar8,CONCAT12(uVar7,uStack18)),param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_ui_op_1020_5e76(param_1: u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  astruct_57 *paVar2;
  let puVar3: *mut u16;
  let puVar4: *mut u8
  let iVar5: i16;
  let uVar6: u16;
  let in_DX: u16;
  let puVar7: *mut u8
  let puVar8: *mut u8
  let iVar9: i16;
  let puVar10: u32;
  let unaff_DI: i16;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  let unaff_SS: *mut u8
  let in_AF: u8;
  char *pcVar14;
  let uVar15: u8;
  u16 *local_2aa [0x40];
  uchar *local_1aa [0x80];
  char local_aa [0x80];
  let uStack42: u32;
  astruct_57 *paStack38;
  char local_22 [0x10];
  let puStack18: *mut u8
  let uStack16: u16;
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u32;
  let local_6: u16;
  let uStack4: u16;
  
  ReleaseCapture16();
  uVar11 = (param_1 >> 0x10);
  iVar9 = param_1;
  SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  (iVar9 + 0xee) = 0x0;
  (iVar9 + 0xf4) = 0x1;
  local_6 = param_3;
  uStack4 = param_2;
  puVar3 = &local_6;
  uVar15 = (u8)(unaff_SS >> 0x8);
  pt_in_rect_1020_5856
            (param_1,(POINT16 *)CONCAT13(uVar15,CONCAT12((char)unaff_SS,puVar3)),puVar3);
  uStack10 = CONCAT22(in_DX,puVar3);
  puVar7 = (in_DX | puVar3);
  if (puVar7 == 0x0) goto LAB_1020_6176;
  uStack12 = puVar3[0x6];
  uStack14 = puVar3[0x7];
  uStack16 = 0x0;
  uVar13 = 0x1018;
  puVar4 = pass1_1018_2580((iVar9 + 0xfa),0x0,CONCAT22(uStack12,uStack14),
                           (iVar9 + 0x10c),unaff_SS,in_AF);
  if (puVar4 == 0x6b2) goto LAB_1020_6176;
  puStack18 = puVar4;
  if (puVar4 == 0x6b8) {
    mem_op_1000_179c(0xb4,puVar7,0x1000);
    uStack42 = CONCAT22(puVar7,puVar4);
    puVar8 = (puVar7 | puVar4);
    if (puVar8 == 0x0) {
      iVar5 = 0x0;
      puVar8 = 0x0;
    }
    else {
      iVar5 = string_1040_8520((astruct_57 *)
                               CONCAT13((char)(puVar7 >> 0x8),
                                        CONCAT12((char)puVar7,puVar4)),
                               PTR_LOOP_1050_0396,0x40,0x2,0x6b8,0x6ad,puVar8,
                               unaff_SS);
    }
    paStack38 = (astruct_57 *)CONCAT22(puVar8,iVar5);
    uVar13 = 0xa5;
LAB_1020_5f84:
    pass1_1008_941a((u16 *)CONCAT22(unaff_SS,local_22),0x1,uVar13);
    pcVar14 = local_22;
    uVar12 = (paStack38 >> 0x10);
    puVar10 = paStack38;
  }
  else {
    if (puVar4 == 0x6b4) {
      mem_op_1000_179c(0xb4,puVar7,0x1000);
      uStack42 = CONCAT22(puVar7,puVar4);
      puVar8 = (puVar7 | puVar4);
      if (puVar8 == 0x0) {
        iVar5 = 0x0;
        puVar8 = 0x0;
      }
      else {
        iVar5 = string_1040_8520((astruct_57 *)
                                 CONCAT13((char)(puVar7 >> 0x8),
                                          CONCAT12((char)puVar7,puVar4)),
                                 PTR_LOOP_1050_0396,0x40,0x2,0x57b,puStack18,
                                 puVar8,unaff_SS);
      }
      paStack38 = (astruct_57 *)CONCAT22(puVar8,iVar5);
      uVar13 = 0xab;
      goto LAB_1020_5f84;
    }
    if (puVar4 == 0x6b6) {
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_aa,
                 (short)unaff_SS);
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_1aa,
                 (short)unaff_SS);
      uVar6 = sys_1000_3f9c((uchar *)local_2aa,unaff_SS,local_1aa,unaff_SS
                            ,PTR_LOOP_1050_50cc,&stack0xfffe,uVar11,0x1000,
                            unaff_SS,in_AF);
      uVar12 = 0x1000;
      mem_op_1000_179c(0xb4,puVar7,0x1000);
      uStack42 = CONCAT22(puVar7,uVar6);
      if ((puVar7 | uVar6) == 0x0) {
        paStack38 = (astruct_57 *)0x0;
      }
      else {
        uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
        paStack38 = pass1_1040_8478((astruct_57 *)CONCAT22(puVar7,uVar6),0x40,
                                    CONCAT13(uVar15,CONCAT12((char)unaff_SS,
                                                                     local_aa)),
                                    CONCAT22(unaff_SS,local_2aa),
                                    PTR_LOOP_1050_0396,puVar7 | uVar6);
      }
      puVar8 = (paStack38 >> 0x10);
      puVar10 = paStack38;
      paVar2 = paStack38;
LAB_1020_6027:
      ppcVar1 = (code **)(*puVar10 + 0x74);
      (**ppcVar1)(uVar12,paVar2);
      goto LAB_1020_6176;
    }
    if (puVar4 < 0x6a7) {
      if (((iVar9 + 0x10c) == 0x78) || ((iVar9 + 0x10c) == 0x74)) {
        uVar13 = 0x1010;
        local_2aa[0] = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x5,unaff_SS,puVar7,
                                       unaff_DI);
        puVar7 = (local_2aa[0] >> 0x10);
        if ((local_2aa[0] + 0xa) == 0x0) {
          return;
        }
      }
      if (((((iVar9 + 0x10c) == 0x6c) || ((iVar9 + 0x10c) == 0x6d)) ||
          ((iVar9 + 0x10c) == 0x31)) || ((iVar9 + 0x10c) == 0x32)) {
        uVar13 = 0x1010;
        local_2aa[0] = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3a,unaff_SS,puVar7,
                                       unaff_DI);
        if ((local_2aa[0] + 0xa) == 0x0) {
          return;
        }
      }
      pass1_1020_68de((iVar9 + 0xfe),uVar13);
      goto LAB_1020_6176;
    }
    if ((uchar *)0x6b1 < puVar4) {
      uVar12 = 0x1000;
      mem_op_1000_179c(0xb4,puVar7,0x1000);
      uStack42 = CONCAT22(puVar7,puVar4);
      puVar8 = (puVar7 | puVar4);
      if (puVar8 == 0x0) {
        puVar10 = 0x0;
        puVar8 = 0x0;
      }
      else {
        uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
        puVar10 = 
                  string_1040_8520((astruct_57 *)
                                   CONCAT13((char)(puVar7 >> 0x8),
                                            CONCAT12((char)puVar7,puVar4)),
                                   PTR_LOOP_1050_0396,0x40,0x2,0x57b,puStack18,
                                   puVar8,unaff_SS);
      }
      local_2aa[0] = CONCAT22(puVar8,puVar10);
      paVar2 = (astruct_57 *)CONCAT22(puVar8,puVar10);
      goto LAB_1020_6027;
    }
    mem_op_1000_179c(0xb4,puVar7,0x1000);
    uStack42 = CONCAT22(puVar7,puVar4);
    puVar8 = (puVar7 | puVar4);
    if (puVar8 == 0x0) {
      iVar5 = 0x0;
      puVar8 = 0x0;
    }
    else {
      iVar5 = string_1040_8520((astruct_57 *)
                               CONCAT13((char)(puVar7 >> 0x8),
                                        CONCAT12((char)puVar7,puVar4)),
                               PTR_LOOP_1050_0396,0x40,0x2,0x57b,puStack18,puVar8,
                               unaff_SS);
    }
    local_2aa[0] = CONCAT22(puVar8,iVar5);
    local_1aa[0] = puStack18 + -0x608;
    pass1_1008_941a((u16 *)CONCAT22(unaff_SS,local_aa),0x1,local_1aa[0]);
    pcVar14 = local_aa;
    uVar12 = (local_2aa[0] >> 0x10);
    puVar10 = local_2aa[0];
  }
  ppcVar1 = (code **)(*puVar10 + 0x6c);
  (**ppcVar1)(0x1008,puVar10,uVar12,pcVar14);
LAB_1020_6176:
  (iVar9 + 0x10c) = 0x0;
  return;
}


fn unk_win_op_1020_65cc(astruct_60 *param_1,param_2: i16,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let BVar3: bool;
  let uVar4: u16;
  astruct_59 *iVar4;
  astruct_60 *iVar5;
  let iVar6: i16;
  astruct_60 *uVar7;
  HWND16 hwnd;
  let iStack4: i16;
  
  iVar5 = (astruct_60 *)param_1;
  uVar7 = (astruct_60 *)(param_1 >> 0x10);
  if (param_2 == 0x1) {
    iVar5->field_0x14 = 0x0;
    return;
  }
  if (param_2 == 0x2) {
    for (iStack4 = 0x0; iStack4 < 0x5; iStack4 += 0x1) {
      iVar4 = (astruct_59 *)&iVar5->field_0x18;
      iVar6 = iStack4 * 0x4;
      if (((iVar4 + iVar6 + 0x2) | (iVar4 + iVar6)) != 0x0) {
        ppcVar1 = (code **)(**(u32 **)(iVar4 + iVar6) + 0x4);
        (**ppcVar1)(param_3,(iVar4 + iVar6));
      }
    }
  }
  else {
    if (((0x0 < param_2 + -0x3) && (!SBORROW2(param_2 + -0x3,0x1))) &&
       (param_2 + -0x4 < 0x4)) {
      BVar3 = IsIconic16(param_3);
      if (BVar3 == 0x0) {
        BVar3 = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
        if ((BVar3 == 0x0) &&
           (uVar2 = iVar5->field_0x14, (uVar2 + 0x24) != 0x0)) {
          InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)0x0,0x0);
          uVar4 = pass1_1020_64d4(param_1,0x2);
          if (uVar4 == 0x0) {
            pass1_1020_6746(param_1,0x1,0x2);
          }
          uVar4 = pass1_1020_64d4(param_1,0x3);
          if (uVar4 == 0x0) {
            pass1_1020_6746(param_1,0x1,0x3);
          }
          hwnd = 0x1018;
          uVar4 = pass1_1018_255e(iVar5->field_0x14);
          if (uVar4 == 0x0) {
            hwnd = s_tile2_bmp_1050_1538;
            SendMessage16(0x1018,0x0,0x0,0x1110069);
          }
          else {
            uVar4 = pass1_1020_64d4(param_1,0x1);
            if (uVar4 == 0x0) {
              pass1_1020_6746(param_1,0x1,0x1);
            }
          }
          SendMessage16(hwnd,0x0,0x0,0x11100f0);
          uVar2 = iVar5->field_0x2c;
          if ((uVar2 + 0x7a) != 0x0) {
            uVar2 = iVar5->field_0x2c;
            (uVar2 + 0x7a) = 0x0;
            SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1110131);
            return;
          }
        }
      }
    }
  }
  return;
}


fn unk_win_ui_op_1020_67ce(astruct_20 *in_struct_1,param_2: u16,Uparam_3: i32)
{
  HGDIOBJ16 HVar1;
  HCURSOR16 HVar2;
  astruct_20 *iVar3;
  astruct_20 *uVar4;
  let unaff_SS: u16;
  
  struct_1020_790e((u16 *)in_struct_1,s_TPPOPMENU_1050_43fa,param_2,param_3,
                   unaff_SS);
  uVar4 = (astruct_20 *)(in_struct_1 >> 0x10);
  iVar3 = (astruct_20 *)in_struct_1;
  iVar3[0x1].field_0x10 = 0x0;
  &iVar3[0x1].field_0x14 = 0x0;
  in_struct_1->field_0x0 = 0x70e6;
  iVar3->field_0x2 = 0x1020;
  unk_str_op_1000_3d3e
            ((in_struct_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x5b)),
             s_VrMode2_1050_4404);
  HVar1 = GetStockObject16(0x1000);
  iVar3->hgdiobj_field_0xc6 = HVar1;
  HVar2 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)0x7f00);
  iVar3->hcursor_field_0xc4 = HVar2;
  iVar3->field_0xac = 0x44c00000;
  iVar3->field_0xc8 = 0x2020;
  iVar3->field_0xbc = (param_3 + 0x8);
  iVar3->field_0xca = param_2;
  win_ui_reg_class_1008_96d2(in_struct_1,0x1008,unaff_SS);
  window_op_1020_6c3a(in_struct_1);
  return;
}


fn realize_palette_1020_6896(param_1: u32,param_2: i16,HGDIOBJ16 param_3)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  
  if (param_2 != 0x0) {
    uVar3 = (param_1 + 0xf2);
    uVar5 = (uVar3 >> 0x10);
    iVar4 = uVar3;
    puVar1 = (iVar4 + 0x24);
    ppcVar2 = (code **)(*puVar1 + 0x18);
    (**ppcVar2)(param_3,puVar1,(iVar4 + 0x26));
    UnrealizeObject16(param_3);
    RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
  }
  return;
}


void 
win_ui_op_1020_6ae6(param_1: *mut u32,param_2: u16,param_3: i16,param_4: i16,HWND16 param_5,
                   WPARAM16 param_6)

{
  code **ppcVar1;
  let puVar2: *mut u8;
  let iVar3: i16;
  let uVar4: u16;
  HWND16 hwnd;
  LRESULT LVar5;
  u16_t in_stack_0000ff86;
  u16_t in_stack_0000ff88;
  HWND16 HVar6;
  let local_58: [u8;50];
  let uStack8: u32;
  HWND16 HStack4;
  
  if (param_4 == 0x1797) {
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    hwnd = s_tile2_bmp_1050_1538;
    HStack4 = GetDlgItem16(param_5,0x1797);
    if (HStack4 != 0x0) {
      if (param_3 == 0x2) {
        hwnd = s_tile2_bmp_1050_1538;
        uStack8 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4090000);
        if ((uStack8 != -0x1) || (false)) {
          HVar6 = HStack4;
          LVar5 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,local_58,param_6,
                                CONCAT22(0x40a,uStack8));
          puVar2 = local_58;
          pass1_1018_30ca((iVar3 + 0xf2),CONCAT22(param_6,puVar2),
                          (LVar5 >> 0x10));
          hwnd = 0x1018;
          pass1_1018_2fe8((iVar3 + 0xf2),in_stack_0000ff86,in_stack_0000ff88);
          if (puVar2 != 0x0) {
            invalidate_rect_1020_735a((iVar3 + 0xf6),0x1018);
            ppcVar1 = (code **)(*param_1 + 0x40);
            (**ppcVar1)(0x1018,param_1,0xef,HVar6);
          }
        }
      }
      else {
        if (param_3 != 0x3) {
          return;
        }
      }
      DestroyWindow16(hwnd);
    }
  }
  return;
}


fn enable_menu_item_1020_6b9a(HMENparam_1: u16,param_2: i16)
{
  if (param_2 != 0x0) {
    return;
  }
  EnableMenuItem16(param_1,0x400,0x0);
  return;
}


fn window_op_1020_6c3a(astruct *param_1)
{
  let uVar1: u32;
  code **ppcVar2;
  HICON16 HVar3;
  astruct_160 *paVar4;
  bool *pBVar5;
  let uVar6: u32;
  let in_DX: *mut u8
  let uVar7: u16;
  let extraout_DX: *mut u8
  let puVar8: *mut u8
  let puVar9: *mut u8
  let uVar10: u16;
  let extraout_DX_00: u16;
  let iVar11: i16;
  let unaff_DI: i16;
  let uVar12: u16;
  let unaff_SS: u16;
  let puVar13: *mut u16;
  let puVar14: *mut u8;
  let local_6: u32;
  
  create_window_ex_1008_9760(param_1,0x1008);
  puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x4,unaff_SS,in_DX,unaff_DI);
  uVar7 = (puVar13 >> 0x10);
  uVar12 = (param_1 >> 0x10);
  iVar11 = param_1;
  (iVar11 + 0xf2) = puVar13;
  (iVar11 + 0xf4) = uVar7;
  (iVar11 + 0xe0) = (iVar11 + 0xf2);
  (iVar11 + 0xe2) = uVar7;
  puVar14 = PTR_LOOP_1050_038c;
  HVar3 = LoadIcon16(0x1010,(LPCSTR)s_TILEICON_1050_440c);
  *(HICON16 *)(iVar11 + 0xc2) = HVar3;
  uVar6 = (iVar11 + 0xf2);
  ppcVar2 = (code **)((iVar11 + 0xf2) + 0x30);
  (**ppcVar2)(s_tile2_bmp_1050_1538,uVar6,(uVar6 >> 0x10),HVar3,
              puVar14);
  paVar4 = (astruct_160 *)(&local_6 + 0x2);
  puVar9 = extraout_DX;
  pass1_1018_2d22((iVar11 + 0xf2),(i16 *)CONCAT22(unaff_SS,&local_6),

                  CONCAT13((char)(unaff_SS >> 0x8),CONCAT12((char)unaff_SS,paVar4)),0xbb8)
  ;
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 != 0x0) {
    uVar7 = (iVar11 + 0x8);
    pass1_1008_3bd6(paVar4,puVar9,0x0,local_6,0x0,0x7c007d,
                    CONCAT13((char)(uVar7 >> 0x8),CONCAT12((char)uVar7,0xbb8)),
                    puVar8,unaff_SS);
  }
  paVar4 = (astruct_160 *)(&local_6 + 0x2);
  pass1_1018_2d22((iVar11 + 0xf2),(i16 *)CONCAT22(unaff_SS,&local_6),
                  CONCAT22(unaff_SS,paVar4),0xbb9);
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (puVar8 | paVar4);
  if (puVar9 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar8,0x0,local_6,0x0,0x7e007f,
                    CONCAT22((iVar11 + 0x8),0xbb9),puVar9,unaff_SS)
    ;
  }
  paVar4 = (astruct_160 *)(&local_6 + 0x2);
  pass1_1018_2d22((iVar11 + 0xf2),(i16 *)CONCAT22(unaff_SS,&local_6),
                  CONCAT22(unaff_SS,paVar4),0xbba);
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar9,0x0,local_6,0x1b2,0x1b001b1,
                    CONCAT22((iVar11 + 0x8),0xbba),puVar8,unaff_SS)
    ;
  }
  mem_op_1000_179c(0x22,puVar8,0x1000);
  uVar10 = puVar8 | paVar4;
  if (uVar10 == 0x0) {
    (iVar11 + 0xf6) = 0x0;
  }
  else {
    unk_win_ui_op_1020_717e((u16 *)CONCAT22(puVar8,paVar4),(ULONG)param_1,unaff_SS);
    *(astruct_160 **)(iVar11 + 0xf6) = paVar4;
    (iVar11 + 0xf8) = uVar10;
  }
  uVar6 = (iVar11 + 0xf6);
  (iVar11 + 0xe8) = uVar6;
  uVar1 = (iVar11 + 0xf2);
  ppcVar2 = (code **)((iVar11 + 0xf2) + 0x10);
  (**ppcVar2)(0x1000,uVar1,(uVar1 >> 0x10));
  pBVar5 = (bool *)uVar6;
  MoveWindow16(0x1000,0x1,pBVar5[0x3],pBVar5[0x2],pBVar5[0x1],*pBVar5);
  uVar6 = param_1;
  ppcVar2 = (code **)(uVar6 + 0x94);
  (**ppcVar2)(s_tile2_bmp_1050_1538,iVar11,(param_1 >> 0x10),0x0);
  ppcVar2 = (code **)(uVar6 + 0x10);
  (**ppcVar2)(s_tile2_bmp_1050_1538,param_1,0x1);
  UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
  return;
}


fn win_ui_fn_1020_6e98(param_1: u32,HWND16 in_win_handle,param_3: u16)
{
  let piVar1: *mut i16;
  astruct_18 *paVar2;
  HWND16 window_handle;
  let uVar3: u16;
  let in_DX: u16;
  WPARAM16 WVar4;
  let iVar5: i16;
  let uVar6: u16;
  LRESULT LVar7;
  char *pcVar8;
  let uVar9: u16;
  let uVar10: u16;
  let iStack36: i16;
  let window_name: u32;
  RECT16 rectangle;
  let IStack6: i16;
  let iStack4: i16;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  GetClientRect16(in_win_handle,&rectangle);
  window_name = (astruct_18 *)0x0;
  window_handle = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1797);
  if (window_handle != 0x0) {
    DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
  }
  pass1_1018_30fc((iVar5 + 0xf2),(u16 **)CONCAT22(param_3,&window_name),in_DX
                 );
  if ((window_name._2_2_ | (LPCSTR)window_name) != 0x0) {
    window_handle =
         CreateWindow16((LPCSTR)0x1018,(LPCSTR)window_name,
                        CONCAT22(PTR_LOOP_1050_038c,window_name._2_2_),0x1797,
                        *(INT16 *)(iVar5 + 0x8),iStack4 + -0x19,IStack6,0x0,0x0,0x103,
                        (LPVOID)0x40a0);
    paVar2 = window_name;
    if (window_handle == 0x0) {
      if ((window_name._2_2_ | window_name) != 0x0) {
        pass1_1018_2afa(window_name);
        fn_ptr_1000_17ce(paVar2,0x1000);
        return;
      }
    }
    else {
      LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0000);
      WVar4 = (WPARAM16)(LVar7 >> 0x10);
      if ((window_name + 0x4) == 0x0) {
        uVar9 = 0x0;
        uVar10 = 0x401;
        pcVar8 = load_string_1010_847e
                           (_PTR_LOOP_1050_14cc,
                            (_PTR_LOOP_1050_14cc >> 0x10),0x1010);
        SendMessage16(0x1010,pcVar8,(WPARAM16)(pcVar8 >> 0x10),
                      CONCAT22(uVar10,uVar9));
      }
      else {
        iStack36 = 0x0;
        while( true ) {
          piVar1 = (i16 *)(window_name + 0x4);
          if (*piVar1 == iStack36 || *piVar1 < iStack36) break;
          uVar9 = 0x0;
          uVar10 = 0x401;
          uVar3 = pass1_1020_bd80(
                                   (window_name + iStack36 * 0x2));
          LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,uVar3,WVar4,
                                CONCAT22(uVar10,uVar9));
          WVar4 = (WPARAM16)(LVar7 >> 0x10);
          iStack36 += 0x1;
        }
      }
      LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0001);
      WVar4 = (WPARAM16)(LVar7 >> 0x10);
      uVar3 = LVar7;
      uVar9 = 0xffff;
      uVar10 = 0x40d;
      pass1_1018_2d84(uVar3,(iVar5 + 0xf2));
      LVar7 = SendMessage16(0x1018,uVar3,WVar4,CONCAT22(uVar10,uVar9));
      iVar5 = LVar7;
      if ((iVar5 != -0x1) || ((LVar7 >> 0x10) != -0x1)) {
        SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,CONCAT22(0x407,iVar5));
        SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,CONCAT22(0x418,iVar5));
      }
      paVar2 = window_name;
      window_handle = s_tile2_bmp_1050_1538;
      if (window_name != (astruct_18 *)0x0) {
        pass1_1018_2afa(window_name);
        window_handle = 0x1000;
        fn_ptr_1000_17ce(paVar2,0x1000);
      }
      ShowWindow16(window_handle,0x1);
      SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    }
  }
  return;
}


fn unk_win_ui_op_1020_717e(param_1: *mut u16,Uparam_2: i32,param_3: u16)
{
  astruct_13 *paVar1;
  code **ppcVar2;
  let uVar3: u32;
  HPALETTE16 HVar4;
  let puVar5: u32;
  let in_DX: *mut u8
  let uVar6: u16;
  let extraout_DX: *mut u8
  let puVar7: *mut u8
  let iVar8: i16;
  let iVar10: i16;
  let unaff_DI: i16;
  let uVar11: u16;
  let uVar12: u16;
  let unaff_CS: u16;
  let puVar13: *mut u16;
  HDC16 local_4;
  astruct_41 *iVar9;
  
  get_sys_metrics_1020_7c1a(param_1,param_2,unaff_CS);
  uVar11 = (param_1 >> 0x10);
  iVar8 = param_1;
  (iVar8 + 0x14) = 0x0;
  *(ULONG *)(iVar8 + 0x18) = param_2;
  (iVar8 + 0x1c) = 0x0;
  (iVar8 + 0x20) = 0x0;
  *param_1 = 0x754c;
  (iVar8 + 0x2) = 0x1020;
  puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x4,param_3,in_DX,unaff_DI);
  uVar6 = (puVar13 >> 0x10);
  (iVar8 + 0x1c) = puVar13;
  (iVar8 + 0x1e) = uVar6;
  ppcVar2 = (code **)((iVar8 + 0x1c) + 0x4);
  (**ppcVar2)(0x1010,(iVar8 + 0x1c),uVar6,0x0,param_1);
  uVar6 = (iVar8 + 0x4);
  local_4 = GetDC16(0x1010);
  uVar3 = (iVar8 + 0x1c);
  *(HDC16 *)(uVar3 + 0x178) = local_4;
  uVar3 = (iVar8 + 0x1c);
  uVar12 = (uVar3 >> 0x10);
  iVar10 = uVar3;
  puVar5 = (iVar10 + 0x24);
  ppcVar2 = (code **)(*puVar5 + 0x14);
  (**ppcVar2)(s_tile2_bmp_1050_1538,puVar5,(iVar10 + 0x26),uVar6)
  ;
  puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_3,extraout_DX,unaff_DI);
  puVar7 = (puVar13 >> 0x10);
  paVar1 = *(astruct_13 **)(puVar13 + 0xe);
  pass1_1008_4d84((astruct_90 *)(puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10),
                  paVar1,puVar7);
  HVar4 = palette_op_1008_4e08(paVar1,&local_4,puVar7,0x1008);
  *(HPALETTE16 *)(iVar8 + 0x20) = HVar4;
  return;
}


fn post_win_msg_1020_7308(param_1: u32,param_2: u16,HWND16 param_3)
{
  let cVar1: u8;
  
  if (param_2 != 0x12) {
    if (param_2 < 0x13) {
      cVar1 = param_2;
      if (cVar1 == '\x01') {
        (param_1 + 0x1c) = 0x0;
        return;
      }
      if (('\x03' < (cVar1 + -0x1)) && ((char)(cVar1 + -0x5) < '\x02'))
      goto LAB_1020_7310;
    }
    return;
  }
LAB_1020_7310:
  PostMessage16(param_3,0x0,0x0,0x11100eb);
  invalidate_rect_1020_735a(param_1,s_tile2_bmp_1050_1538);
  return;
}



fn win_ui_op_1020_737a(Uparam_1: i32,HWND16 param_2,param_3: u16) -> bool

{
  let uVar1: u16;
  code **ppcVar2;
  let uVar3: u32;
  let Bvar4: bool;
  let puVar5: *mut u8;
  let puVar6: u32;
  let in_DX: u16;
  let uVar7: u16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  astruct_18 *paStack78;
  let local_42: [u8;6];
  let local_brush_handle: u32;
  let iStack56: i16;
  HWND16 HStack54;
  HWND16 HStack52;
  let iStack50: i16;
  RECT16 local_30;
  let iStack44: i16;
  let iStack42: i16;
  RECT16 *local_rect;
  let BStack38: bool;
  HDC16 local_24;
  PAINTSTRUCT16 local_paint_struct;
  
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  uVar11 = (iVar8 + 0x4);
  local_24 = BeginPaint16(param_2,&local_paint_struct);
  uVar10 = (iVar8 + 0x4);
  BVar4 = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
  if (BVar4 == 0x0) {
    uVar3 = (iVar8 + 0x1c);
    puVar6 = (uVar3 + 0x24);
    local_brush_handle = puVar6;
    pass1_1018_2e5e(param_3,puVar6,in_DX,(iVar8 + 0x1c));
    local_30 = puVar6 & 0xffff | in_DX << 0x10;
    pass1_1008_3e54((u16 *)
                    CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,local_42)),0x0,
                    0x35,0xc);
    if (*(long *)(iVar8 + 0x14) != 0x0) {
      pass1_1008_5236((iVar8 + 0x14));
    }
    if (local_30 != 0x0) {
      uVar1 = (iVar8 + 0x14);
      uVar7 = (iVar8 + 0x16);
      paStack78 = (astruct_18 *)CONCAT22(uVar7,uVar1);
      if ((uVar7 | uVar1) != 0x0) {
        pass1_1008_5118(CONCAT22(uVar7,uVar1));
        fn_ptr_1000_17ce(paStack78,0x1000);
      }
      puVar5 = local_42;
      pass1_1008_8ce4(local_30,CONCAT22(param_3,puVar5),
                      local_brush_handle,param_3);
      (iVar8 + 0x14) = puVar5;
      (iVar8 + 0x16) = uVar7;
    }
    ppcVar2 = (code **)(*local_brush_handle + 0x4);
    (**ppcVar2)(0x1008,local_brush_handle,(local_brush_handle >> 0x10),
                0x0,0x0);
    ppcVar2 = (code **)((iVar8 + 0x18) + 0x94);
    (**ppcVar2)(0x1008,(iVar8 + 0x18),0x1);
    HStack52 = GetDlgItem16(0x1008,0x1797);
    if (HStack52 != 0x0) {
      ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    }
  }
  else {
    if (PTR_LOOP_1050_0010 == 0x0) {
      ppcVar2 = (code **)((iVar8 + 0x1c) + 0x2c);
      (**ppcVar2)(s_tile2_bmp_1050_1538,(iVar8 + 0x1c),uVar10,uVar11);
      BStack38 = BVar4;
      if (BVar4 != 0x0) {
        local_rect = (RECT16 *)GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_30);
        local_brush_handle = 0x0;
        iStack56 = (iStack44 - local_30.x) + -0x1;
        HStack54 = (iStack42 - local_30.y) - 0x1;
        HStack52 = HStack54;
        iStack50 = iStack56;
        FillRect16((HDC16)s_tile2_bmp_1050_1538,local_rect,(HBRUSH16)&local_brush_handle);
        DrawIcon16((HDC16)s_tile2_bmp_1050_1538,BStack38,0x2,0x2);
      }
    }
  }
  BVar4 = EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_paint_struct);
  return BVar4;
}


fn win_1020_75f0(param_1: u32,param_2: u16)
{
  let pUVar1: *mut u16;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u32;
  let puVar5: *mut u8
  let puVar6: *mut u8
  astruct_283 *iVar7;
  let uVar7: u16;
  let puVar8: *mut u16;
  let puStack10: u32;
  let local_6: [u8;4];
  
  uVar7 = (param_1 >> 0x10);
  iVar7 = (astruct_283 *)param_1;
  if (iVar7->field_0xee != 0x0) {
    ppcVar2 = (code **)(*iVar7->field_0xee + 0x8);
    (**ppcVar2)();
  }
  if (iVar7->field_0xea == 0x0) {
    iVar7->field_0xea = 0x1;
    puVar8 = pass1_1008_941a((u16 *)CONCAT22(param_2,local_6),0x1,0x91);
    puVar5 = (puVar8 >> 0x10);
    uVar4 = ZEXT24(local_6);
    win_1008_5c9e(_PTR_LOOP_1050_02a0,CONCAT22(param_2,local_6),local_6,puVar5,
                  param_2);
    iVar7->field_0xec = uVar4;
    mem_op_1000_179c(0x112,puVar5,0x1000);
    puVar6 = (puVar5 | uVar4);
    if (puVar6 == 0x0) {
      uVar3 = 0x0;
      puStack10 = 0x0;
    }
    else {
      pUVar1 = &iVar7->field_0xcc;
      *pUVar1 = *pUVar1 + 0x1;
      struct_1020_3644((u16 *)(uVar4 & 0xffff | ZEXT24(puVar5) << 0x10),
                       iVar7->field_0xcc,param_1,param_2);
      uVar3 = uVar4;
      puStack10 = (uVar4 & 0xffff | ZEXT24(puVar6) << 0x10);
    }
    pass1_1008_6978(param_1,0x0,puStack10 & 0xffff0000 | uVar3,uVar3,puVar6)
    ;
    ppcVar2 = (code **)(*puStack10 + 0xc);
    (**ppcVar2)();
  }
  return;
}



fn window_op_1020_76aa(astruct *param_1)
{
  astruct_666 *in_AX;
  let in_DX: *mut u8
  let uVar3: u32;
  let iVar1: i16;
  let unaff_DI: i16;
  let uVar2: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  get_dc_1018_4db0(*(ULONG *)(iVar1 + 0xf2),(iVar1 + 0x8),0x1018);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar3 = in_DX | in_AX;
  if (uVar3 != 0x0) {
    pass1_1020_7824(in_AX,in_DX,(iVar1 + 0x8),unaff_DI,unaff_SS);
    *(astruct_666 **)(iVar1 + 0xee) = in_AX;
    (iVar1 + 0xf0) = uVar3;
    return;
  }
  (iVar1 + 0xee) = 0x0;
  return;
}


void 
post_win_msg_1020_79fc
          (astruct_69 *param_1,param_2: u16,param_3: u16,param_4: i16,HWND16 param_5)

{
  let puVar1: u32;
  code **ppcVar2;
  let iVar3: i16;
  astruct_69 *iVar4;
  let uVar4: u16;
  let uVar5: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_69 *)param_1;
  ppcVar2 = (code **)(*iVar4->field_0xe0 + 0x24);
  iVar3 = (**ppcVar2)(param_5,iVar4->field_0xe0);
  if (iVar3 != param_4) {
    uVar5 = iVar4->field_0x8;
    PostMessage16(param_5,0x0,0x0,0x850000);
    puVar1 = iVar4->field_0xe0;
    ppcVar2 = (code **)(*iVar4->field_0xe0 + 0x28);
    (**ppcVar2)(s_tile2_bmp_1050_1538,puVar1,(puVar1 >> 0x10),
                param_4,uVar5);
  }
  return;
}



fn get_win_ui_info_op_1020_7a50(param_1: u32,HWND16 param_2)
{
  code **ppcVar1;
  let b_var2: bool;
  let IVar2: i16;
  let IVar3: i16;
  let var5: u16;
  RECT16 local_a;
  let iStack6: i16;
  let iStack4: i16;
  
  local_a.x = 0x0;
  local_a.y = 0x0;
  iStack6 = 0x0;
  iStack4 = 0x0;
  var5 = (param_1 >> 0x10);
  b_var2 = IsIconic16(param_2);
  if (b_var2 == 0x0) {
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_a);
    iStack6 -= local_a.x;
    iStack4 -= local_a.y;
    IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    local_a.x += IVar2 * 0x2;
    local_a.y += IVar3 * 0x2;
  }
  ppcVar1 = (code **)((param_1 + 0xe0) + 0x14);
  (**ppcVar1)(s_tile2_bmp_1050_1538,(param_1 + 0xe0),&local_a);
  return;
}



fn win_ui_menu_op_1020_7ad2(param_1: u32,HWND16 param_2,RECT16 *param_3,HWND16 param_4)
{
  HMENU16 HVar1;
  let iVar2: i16;
  let uVar3: u16;
  POlet local_6: i16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((*(long *)(iVar2 + 0xee) != 0x0) && ((iVar2 + 0xec) == 0x0)) {
    HVar1 = LoadMenu16(param_4,(LPCSTR)(iVar2 + 0xee));
    *(HMENU16 *)(iVar2 + 0xec) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
    param_4 = s_tile2_bmp_1050_1538;
    HVar1 = GetSubMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0);
    *(HMENU16 *)(iVar2 + 0xec) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
  }
  local_6.x = param_3;
  local_6.y = param_2;
  ClientToScreen16(param_4,&local_6);
  TrackPopupMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0,0x0,*(INT16 *)(iVar2 + 0x8),0x0,
                   local_6.y,(RECT16 *)local_6.x);
  return;
}


