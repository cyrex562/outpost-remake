
void 
win_op_1018_294a(param_1: i16,param_2: u16,param_3: u16,Uparam_4: i32,param_5: u16,
                LPCSTR in_string_6)

{
  if (((param_1 + 0x18) != 0x0) && (param_4._2_2_ == 0x280)) {
    (param_1 + 0x18) = 0x0;
  }
  create_dc_1018_4e04((astruct_8 **)CONCAT22(param_2,param_1),param_3,param_4,
                      param_4._2_2_,in_string_6,param_5);
  return;
}


fn unk_win_ui_op_1018_4f18(astruct_39 *param_1,param_2: u16,param_3: u32)
{
  code **ppcVar1;
  let puVar2: u32;
  RECT16 *rect;
  let iVar3: i16;
  let uVar4: u32;
  let extraout_DX: *mut u8
  let puVar5: *mut u8
  let extraout_DX_00: *mut u8
  let puVar6: *mut u8
  let uVar7: u16;
  astruct_39 *iVar6;
  let uVar8: u16;
  let uVar9: u16;
  let unaff_SS: u16;
  astruct_76 *paStack22;
  RECT16 local_12;
  let iStack14: i16;
  let iStack12: i16;
  let uStack10: u32;
  astruct_43 *paStack6;
  
  paStack6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_2,unaff_SS);
  uVar4 = paStack6 & 0xffff;
  ppcVar1 = (code **)(paStack6 + 0x14);
  (**ppcVar1)(0x1010,uVar4,(paStack6 >> 0x10));
  puVar2 = uVar4;
  uStack10 = uVar4 & 0xffff | ZEXT24(extraout_DX) << 0x10;
  uVar8 = (param_1 >> 0x10);
  iVar6 = (astruct_39 *)param_1;
  puVar5 = extraout_DX;
  if (*(long *)&iVar6->field_0xe != 0x0) {
    uVar7 = iVar6->field_0x10;
    puVar2 = &iVar6->field_0xe;
    puVar5 = (uchar *)(uVar7 | puVar2);
    if (puVar5 != (uchar *)0x0) {
      ppcVar1 = (code **)*puVar2;
      (**ppcVar1)();
      puVar5 = extraout_DX_00;
    }
  }
  mem_op_1000_179c(0x14,puVar5,0x1000);
  puVar6 = (uchar *)(puVar5 | puVar2);
  if (puVar6 == (uchar *)0x0) {
    puVar2 = 0x0;
    puVar6 = (uchar *)0x0;
  }
  else {
    struct_1008_4c58((u16 *)CONCAT22(puVar5,puVar2));
  }
  iVar6->field_0xe = puVar2;
  iVar6->field_0x10 = puVar6;
  pass1_1008_4d84(*(astruct_90 **)&iVar6->field_0xe,uStack10,puVar6);
  rect = &local_12;
  GetClientRect16(0x1008,rect);
  uVar9 = 0x1000;
  mem_op_1000_179c(0x1e,puVar6,0x1000);
  paStack22 = (astruct_76 *)CONCAT22(puVar6,rect);
  uVar7 = puVar6 | rect;
  if (uVar7 == 0x0) {
    &iVar6->field_0xa = 0x0;
  }
  else {
    iVar3 = (iStack12 - local_12.y) + 0x1;
    uVar9 = 0x1008;
    pass1_1008_405c(paStack22,&iVar6->field_0xe,iVar3,
                    (iStack14 - local_12.x) + 0x1);
    iVar6->field_0xa = iVar3;
    iVar6->field_0xc = uVar7;
  }
  if (paStack6 != (astruct_43 *)0x0) {
    ppcVar1 = (code **)paStack6;
    (**ppcVar1)(uVar9,paStack6,(paStack6 >> 0x10),0x1);
  }
  return;
}


fn win_1018_598c(astruct *param_1,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  astruct_131 *iVar1;
  let uVar2: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar2 = (param_1 >> 0x10);
  iVar1 = (astruct_131 *)param_1;
  get_dc_1018_4db0(iVar1->field_0xf2,iVar1->field_0x8,0x1008);
  mem_op_1000_179c(0x2a,(uchar *)param_3,0x1000);
  uVar1 = param_3 | param_2;
  if (uVar1 != 0x0) {
    pass1_1018_5b06((astruct_132 *)param_2,param_3,iVar1->field_0x8,unaff_SS);
    iVar1->field_0xee = param_2;
    iVar1->field_0xf0 = uVar1;
    return;
  }
  &iVar1->field_0xee = 0x0;
  return;
}


fn win_ui_op_1018_5e9a(astruct_1 *param_1,param_2: u16)
{
  let uVar1: u32;
  ULONG *pUVar2;
  let IVar3: i16;
  let puVar4: *mut u8;
  let in_DX: *mut u8
  let puVar5: *mut u8
  let puVar6: *mut u8
  let uVar7: u16;
  let uVar8: u16;
  let iVar9: i16;
  let unaff_DI: i16;
  let uVar10: u16;
  let puVar11: *mut u16;
  let local_28: [u8;12];
  let iStack22: i16;
  let uStack20: u16;
  let iStack18: i16;
  let iStack16: i16;
  RECT16 local_e;
  let iStack8: i16;
  INT16 *pIStack6;
  
  dialog_ui_fn_1040_78e2(param_1,&PTR_LOOP_1050_1040);
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,param_2,in_DX,unaff_DI);
  puVar5 = (uchar *)(puVar11 >> 0x10);
  uVar7 = puVar11;
  uVar10 = (param_1 >> 0x10);
  iVar9 = param_1;
  (iVar9 + 0x8e) = uVar7;
  *(uchar **)(iVar9 + 0x90) = puVar5;
  mem_op_1000_179c(0x12,puVar5,0x1000);
  puVar6 = (uchar *)(puVar5 | uVar7);
  if (puVar6 == (uchar *)0x0) {
    (iVar9 + 0x92) = 0x0;
  }
  else {
    pass1_1018_6198((u16 *)CONCAT22(puVar5,uVar7),param_1,
                    (iVar9 + 0x6),puVar6,unaff_DI,param_2);
    (iVar9 + 0x92) = uVar7;
    *(uchar **)(iVar9 + 0x94) = puVar6;
  }
  uVar1 = (iVar9 + 0x8e);
  pIStack6 = (INT16 *)(uVar1 & 0xffff0000 | (uVar1 + 0xa));
  GetClientRect16(0x1000,&local_e);
  IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  (pIStack6 + 0x6) = IVar3 + iStack8;
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_2,puVar6,unaff_DI);
  uStack20 = (puVar11 >> 0x10);
  iStack22 = puVar11;
  iStack16 = (iStack22 + 0xa);
  iStack18 = (iStack22 + 0xc);
  uVar10 = (pIStack6 >> 0x10);
  (pIStack6 + 0x2) = (iStack18 - (pIStack6 + 0x6)) / 0x2;
  uVar7 = iStack16 >> 0xf;
  *pIStack6 = iStack16 / 0x2 + 0x3;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_28),0x1,0x0,0x100);
  while( true ) {
    puVar4 = local_28;
    pass1_1028_e4ec(CONCAT22(param_2,puVar4));
    uVar8 = uVar7 | puVar4;
    if (uVar8 == 0x0) break;
    pUVar2 = *(ULONG **)(puVar4 + 0x10);
    uVar7 = uVar8;
    if (pUVar2 != (ULONG *)0x0) {
      pass1_1000_3cea(param_1 & 0xffff0000 | (iVar9 + 0x10),*pUVar2);
      uVar7 = uVar8;
    }
  }
  uVar10 = (pIStack6 >> 0x10);
  iVar9 = pIStack6;
  SetWindowPos16((HWND16)&USHORT_1050_1028,0x44,*(INT16 *)(iVar9 + 0x6),
                 *(INT16 *)(iVar9 + 0x4),*(INT16 *)(iVar9 + 0x2),*pIStack6,0x0);
  return;
}


void 
set_window_text_1018_6066
          (param_1: u16,param_2: u16,SEGPTR in_win_text_3,param_4: u16,
          INT16 dialog_id_5,HWND16 in_hwnd_6)

{
  GetDlgItem16(in_hwnd_6,dialog_id_5);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,in_win_text_3);
  return;
}



fn set_window_text_1018_6086(Uparam_1: i32,LPSTR param_2,param_3: *mut u16)
{
  HWND16 HStack8;
  
  wsprintf16(param_2,&stack0xfff4,param_3);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1be);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)&stack0xfff4);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,&stack0xfff4,param_3);
  HStack8 = (param_1 + 0x6);
  HStack8 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1bf);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)&stack0xfff4);
  return;
}


fn window_op_1018_67b6(astruct *param_1)
{
  astruct_658 *in_AX;
  let in_DX: *mut u8
  let uVar1: u16;
  let iVar2: i16;
  let unaff_DI: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2),(iVar2 + 0x8),0x1008);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar1 = in_DX | in_AX;
  if (uVar1 != 0x0) {
    pass1_1018_6924(in_AX,in_DX,(iVar2 + 0x8),unaff_DI,unaff_SS);
    *(astruct_658 **)(iVar2 + 0xee) = in_AX;
    (iVar2 + 0xf0) = uVar1;
    return;
  }
  (iVar2 + 0xee) = 0x0;
  return;
}


fn mix_ui_op_1018_6adc(astruct_28 *param_1)
{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u16;
  let in_DX: *mut u8
  let uVar4: u16;
  let iVar5: i16;
  let unaff_DI: i16;
  let uVar6: u16;
  WNDCLASS16 *unaff_SS;
  let puVar7: *mut u16;
  astruct_43 *paVar8;
  
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  uVar4 = (puVar7 >> 0x10);
  iVar1 = (puVar7 + 0xa);
  iVar2 = (puVar7 + 0xc);
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  if (0x1 < iVar1 - (iVar5 + 0xe6)) {
    uVar4 = iVar1 >> 0xf;
    (iVar5 + 0xe2) = iVar1 / 0x2 - ((iVar5 + 0xe6) + 0x1) / 0x2;
  }
  if (0x1 < iVar2 - (iVar5 + 0xe8)) {
    uVar4 = iVar2 >> 0xf;
    (iVar5 + 0xe4) = iVar2 / 0x2 - ((iVar5 + 0xe8) + 0x1) / 0x2;
  }
  uVar3 = ShowCursor16(0x1010);
  if ((iVar5 + 0xee) != 0x0) {
    win_1008_5c5c(unaff_SS,uVar3,uVar4,_PTR_LOOP_1050_02a0,(iVar5 + 0xee));
    (iVar5 + 0xf0) = uVar3;
  }
  paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,(iVar5 + 0xec),
                               unaff_SS);
  mci_send_command_1008_53ae
            (paVar8,(iVar5 + 0x8),0x1008,unaff_SS);
  ShowCursor16(0x1008);
  unk_destroy_window_op_1018_6bb6(param_1,s_tile2_bmp_1050_1538);
  return;
}


fn win_1018_df40(astruct *param_1,param_2: u16,uchar *param_3,param_4: u16)
{
  astruct_267 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  mem_op_1000_179c(0xa,param_3,0x1000);
  iVar1 = (astruct_267 *)param_1;
  uVar1 = (param_1 >> 0x10);
  if ((uchar *)(param_3 | param_2) != (uchar *)0x0) {
    puVar2 = struct_1018_e100((u16 *)CONCAT22(param_3,param_2),iVar1->field_0x8,
                              (uchar *)(param_3 | param_2),param_4);
    iVar1->field_0xe2 = puVar2;
    iVar1->field_0xe4 = (puVar2 >> 0x10);
    return;
  }
  &iVar1->field_0xe2 = 0x0;
  return;
}


fn window_op_1018_e384(astruct *param_1)
{
  astruct_659 *in_AX;
  let in_DX: *mut u8
  let uVar1: u16;
  let iVar2: i16;
  let unaff_DI: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2),(iVar2 + 0x8),0x1008);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar1 = in_DX | in_AX;
  if (uVar1 != 0x0) {
    pass1_1018_e4f2(in_AX,in_DX,(iVar2 + 0x8),unaff_DI,unaff_SS);
    *(astruct_659 **)(iVar2 + 0xee) = in_AX;
    (iVar2 + 0xf0) = uVar1;
    return;
  }
  (iVar2 + 0xee) = 0x0;
  return;
}


fn window_op_1018_e6c6(astruct *param_1)
{
  astruct_660 *in_AX;
  let in_DX: *mut u8
  let uVar1: u16;
  let iVar2: i16;
  let unaff_DI: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2),(iVar2 + 0x8),0x1008);
  mem_op_1000_179c(0x18,in_DX,0x1000);
  uVar1 = in_DX | in_AX;
  if (uVar1 != 0x0) {
    pass1_1018_e834(in_AX,in_DX,(iVar2 + 0x8),unaff_DI,unaff_SS);
    *(astruct_660 **)(iVar2 + 0xee) = in_AX;
    (iVar2 + 0xf0) = uVar1;
    return;
  }
  (iVar2 + 0xee) = 0x0;
  return;
}


fn post_win_msg_1018_ea0a(param_1: u16,param_2: u16,param_3: i16,HWND16 param_4)
{
  if (param_3 == 0xed) {
    PostMessage16(param_4,0x0,0x0,0x11100c6);
  }
  return;
}


fn window_op_1018_eada(astruct *param_1)
{
  astruct_661 *in_AX;
  let in_DX: *mut u8
  let uVar1: u16;
  let iVar2: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2),(iVar2 + 0x8),0x1008);
  mem_op_1000_179c(0x28,in_DX,0x1000);
  uVar1 = in_DX | in_AX;
  if (uVar1 != 0x0) {
    pass1_1018_ec74(in_AX,in_DX,(iVar2 + 0x8),unaff_SS);
    *(astruct_661 **)(iVar2 + 0xee) = in_AX;
    (iVar2 + 0xf0) = uVar1;
    return;
  }
  (iVar2 + 0xee) = 0x0;
  return;
}


fn win_1020_0316(astruct *param_1,uchar *param_2,param_3: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u8
  let puVar4: *mut u8
  astruct_273 *iVar1;
  let unaff_DI: i16;
  let uVar5: u16;
  let puVar6: *mut u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x1,param_3,param_2,unaff_DI);
  puVar3 = (uchar *)(puVar6 >> 0x10);
  uVar5 = (param_1 >> 0x10);
  iVar1 = (astruct_273 *)param_1;
  iVar1->field_0xe2 = puVar6;
  iVar1->field_0xe4 = puVar3;
  uVar1 = &iVar1->field_0xe2;
  (uVar1 + 0x16) = iVar1->field_0xea;
  uVar2 = iVar1->field_0xee;
  uVar1 = &iVar1->field_0xe2;
  (uVar1 + 0x12) = uVar2;
  struct_1010_3c52(&iVar1->field_0xe2,iVar1->field_0xec,param_3);
  mem_op_1000_179c(0x12,puVar3,0x1000);
  puVar4 = (uchar *)(puVar3 | uVar2);
  if (puVar4 != (uchar *)0x0) {
    pass1_1020_04f6((u16 *)CONCAT22(puVar3,uVar2),iVar1->field_0x8,puVar4,unaff_DI,
                    param_3);
    iVar1->field_0xe6 = uVar2;
    iVar1->field_0xe8 = puVar4;
    return;
  }
  &iVar1->field_0xe6 = 0x0;
  return;
}



fn post_msg_1020_03b2(param_1: u32,HWND16 param_2)
{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0xe2);
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,(uVar1 + 0x16)));
  return;
}



fn post_msg_1020_03d6(param_1: u32,HWND16 param_2)
{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0xe2);
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,(uVar1 + 0x16)));
  return;
}



fn post_msg_1020_03fa(param_1: u32,HWND16 param_2)
{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0xe2);
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,(uVar1 + 0x16)));
  return;
}


fn post_win_msg_1020_061c(param_1: u32,param_2: i16,HWND16 param_3)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (param_1 + 0x6) = 0x0;
    return;
  }
  if (param_2 != 0x2) {
    return;
  }
  uVar1 = (param_1 + 0x6);
  PostMessage16(param_3,0x0,0x0,CONCAT22(0x111,(uVar1 + 0x16)));
  return;
}


fn send_win_msg_1020_08fe(astruct_63 *param_1,HWND16 param_2)
{
  let BVar1: bool;
  astruct_63 *iVar2;
  astruct_63 *uVar2;
  
  uVar2 = (astruct_63 *)(param_1 >> 0x10);
  iVar2 = (astruct_63 *)param_1;
  param_1 = 0xb0e;
  iVar2->field_0x2 = 0x1020;
  if (iVar2->field_0xe8 != 0x0) {
    BVar1 = IsWindow16(param_2);
    if (BVar1 != 0x0) {
      SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1110001);
    }
    iVar2->field_0xe8 = 0x0;
  }
  pass1_1008_57c4((u16 *)
                  (param_1 & 0xffff0000 | &iVar2->field_0xd2));
  param_1 = 0x380a;
  iVar2->field_0x2 = 0x1008;
  param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  return;
}



fn send_msg_1020_097e(param_1: u32,HWND16 param_2)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (((iVar1 + 0xea) | (iVar1 + 0xe8)) != 0x0) {
    SendMessage16(param_2,0x0,0x0,0x1110001);
    (iVar1 + 0xe8) = 0x0;
  }
  return;
}



fn win_1020_09ba(astruct *param_1,param_2: u16,uchar *param_3,param_4: u16)
{
  let puVar1: *mut u8
  astruct_275 *iVar1;
  let uVar2: u16;
  
  create_window_ex_1008_9760(param_1,0x1008);
  mem_op_1000_179c(0xe,param_3,0x1000);
  puVar1 = (uchar *)(param_3 | param_2);
  iVar1 = (astruct_275 *)param_1;
  uVar2 = (param_1 >> 0x10);
  if (puVar1 != (uchar *)0x0) {
    struct_1020_0baa((u16 *)CONCAT22(param_3,param_2),iVar1->field_0x8,puVar1,param_4);
    iVar1->field_0xe2 = param_2;
    iVar1->field_0xe4 = puVar1;
    return;
  }
  &iVar1->field_0xe2 = 0x0;
  return;
}
