


void __stdcall16far show_win_1038_b634(ulong param_1,HWND16 param_2)

{
  int iVar1;
  undefined2 uVar2;
  HWND16 HVar3;
  uint uStack4;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0xac) == 0x0) {
    *(undefined2 *)(iVar1 + 0xac) = 0x1;
    for (uStack4 = 0x1; uStack4 < 0x2b; uStack4 = uStack4 + 0x1) {
      HVar3 = param_2;
      if ((*(uint *)(uStack4 * 0x4 + iVar1 + 0x2) | *(uint *)(uStack4 * 0x4 + iVar1)) != 0x0) {
        HVar3 = (HWND16)s_tile2_bmp_1050_1538;
        ShowWindow16(param_2,0x0);
      }
      param_2 = HVar3;
    }
  }
  return;
}



void __stdcall16far show_win_1038_b68a(ulong param_1,HWND16 param_2)

{
  int iVar1;
  undefined2 uVar2;
  HWND16 HVar3;
  uint uStack4;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0xac) != 0x0) {
    *(undefined2 *)(iVar1 + 0xac) = 0x0;
    for (uStack4 = 0x1; uStack4 < 0x2b; uStack4 = uStack4 + 0x1) {
      HVar3 = param_2;
      if ((*(uint *)(uStack4 * 0x4 + iVar1 + 0x2) | *(uint *)(uStack4 * 0x4 + iVar1)) != 0x0) {
        HVar3 = (HWND16)s_tile2_bmp_1050_1538;
        ShowWindow16(param_2,0x1);
      }
      param_2 = HVar3;
    }
  }
  return;
}



void __stdcall16far pass1_1038_b6e0(ulong param_1,int param_2)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  uint uStack4;
  
  uStack4 = 0x1;
  while( true ) {
    if (0x2a < uStack4) {
      return;
    }
    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if (((*(uint *)(uStack4 * 0x4 + iVar2 + 0x2) | *(uint *)(uStack4 * 0x4 + iVar2)) != 0x0) &&
       (uVar1 = *(undefined4 *)(uStack4 * 0x4 + iVar2), *(int *)((int)uVar1 + 0x6) == param_2)) break;
    uStack4 = uStack4 + 0x1;
  }
  *(undefined4 *)(uStack4 * 0x4 + iVar2) = 0x0;
  return;
}



BOOL16 __stdcall16far bring_win_to_top_1038_b72e(u32 param_1,i16 param_2,HWND16 in_win_handle_3)

{
  if (*(long *)(param_2 * 0x4 + (int)param_1) != 0x0) {
    SetFocus16(in_win_handle_3);
    BringWindowToTop16((HWND16)s_tile2_bmp_1050_1538);
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_b772(astruct_57 *param_1,uchar *param_2,int param_3,ushort param_4,ushort param_5)

{
  uchar *puVar1;
  astruct_705 *iVar2;
  undefined2 uVar2;
  ushort *puVar3;
  
  get_sys_metrics_1040_7728(param_1,0x9a,0x0,0xfbf,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_705 *)param_1;
  *(undefined4 *)&iVar2->field_0x8e = 0x0;
  *(undefined4 *)&iVar2->field_0x92 = 0x0;
  iVar2->field_0x96 = 0x1;
  iVar2->field_0x98 = 0x0;
  *(undefined2 *)param_1 = 0xbd70;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1038;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x36,param_4,param_2,param_3);
  puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
  iVar2->field_0x8e = (int)puVar3;
  iVar2->field_0x90 = puVar1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,param_4,puVar1,param_3);
  iVar2->field_0x92 = (int)puVar3;
  iVar2->field_0x94 = (int)((ulong)puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_b7f0(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xbd70;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1038_b81c(astruct_1 *param_1)

{
  ulong uVar1;
  undefined4 uVar2;
  code **ppcVar3;
  uint uVar4;
  BOOL16 win_enabled;
  uchar *in_DX;
  undefined2 extraout_DX;
  int iVar6;
  int unaff_DI;
  undefined2 uVar7;
  HWND16 HVar8;
  HWND16 hwnd_dlg;
  ushort unaff_SS;
  ushort *puVar9;
  int *piStack16;
  UINT16 UStack12;
  int iStack10;
  astruct_1 *iVar7;
  int *piVar5;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,unaff_SS,in_DX,unaff_DI);
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  *(undefined2 *)(iVar6 + 0x92) = (int)puVar9;
  *(undefined2 *)(iVar6 + 0x94) = (int)((ulong)puVar9 >> 0x10);
  uVar1 = *(ulong *)(iVar6 + 0x92);
  uVar4 = (int)uVar1 + 0x4e;
  uVar1 = uVar1 & 0xffff0000;
  piVar5 = (int *)(uVar1 | uVar4);
  iStack10 = 0x0;
  hwnd_dlg = 0x1010;
  for (UStack12 = 0x1a0; (int)UStack12 < 0x1b5; UStack12 = UStack12 + 0x1) {
    if (*(UINT16 *)(iStack10 * 0x2 + uVar4) == UStack12) {
      iStack10 = iStack10 + 0x1;
      HVar8 = hwnd_dlg;
    }
    else {
      HVar8 = (HWND16)s_tile2_bmp_1050_1538;
      CheckDlgButton16(hwnd_dlg,0x2,UStack12);
    }
    hwnd_dlg = HVar8;
  }
  GetDlgItem16(hwnd_dlg,0xfb1);
  win_enabled = EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  uVar2 = *(undefined4 *)(iVar6 + 0x92);
  ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x92) + 0x10);
  (**ppcVar3)((int)s_tile2_bmp_1050_1538,(int)uVar2,(int)((ulong)uVar2 >> 0x10));
  piStack16 = (int *)CONCAT22(extraout_DX,win_enabled);
  move_win_1040_826c(param_1,*(int *)(win_enabled + 0x2) + -0x2,*(int *)(win_enabled + 0x4) + *piStack16 + 0x3);
  ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  pass1_1018_1c9a(*(ulong *)(iVar6 + 0x92),*piVar5);
  GetDlgItem16(0x1018,*piVar5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far
win_ui_op_1038_b922(ulong *param_1,ulong param_2,uint param_3,ushort param_4,HWND16 param_5,WNDCLASS16 *param_6)

{
  int *piVar1;
  code **ppcVar2;
  UINT16 UVar3;
  BOOL16 BVar4;
  uint uVar5;
  uchar *puVar6;
  int iVar7;
  int unaff_DI;
  undefined2 uVar8;
  undefined2 uVar9;
  LRESULT LVar10;
  char *pcVar11;
  astruct_57 *paVar12;
  undefined4 uVar13;
  CHAR *pCVar14;
  WNDCLASS16 *pWVar15;
  undefined *puVar16;
  uint uStack1132;
  char local_464 [0x50];
  CHAR local_414 [0x400];
  ulong uStack20;
  undefined *puStack16;
  ushort *puStack14;
  int iStack10;
  HWND16 HStack8;
  BOOL16 BStack6;
  undefined2 uStack4;
  
  uVar13 = CONCAT22(param_4,HStack8);
  BStack6 = 0x0;
  uStack4 = 0x0;
  iVar7 = (int)param_1;
  uVar8 = (undefined2)((ulong)param_1 >> 0x10);
  if (param_3 < 0x1b5) {
    if (param_3 < 0x1a0) {
      uVar13 = CONCAT22(param_4,HStack8);
      if (param_3 != 0x2) goto LAB_1038_bbbf;
    }
    else {
      HStack8 = GetDlgItem16(param_5,param_3);
      LVar10 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4000000);
      iStack10 = (int)LVar10;
      if (iStack10 == 0x2) {
        BStack6 = 0x0;
        uStack4 = 0x0;
        goto LAB_1038_bc26;
      }
      SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,CONCAT13(0x4,CONCAT12(0x1,(uint)(iStack10 == 0x0))));
      UVar3 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,param_3);
      if (UVar3 == 0x0) {
        piVar1 = (int *)(iVar7 + 0x96);
        *piVar1 = *piVar1 + 0x1;
        if (*(int *)(iVar7 + 0x96) == 0x1) {
          GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
          EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
        }
      }
      else {
        piVar1 = (int *)(iVar7 + 0x96);
        *piVar1 = *piVar1 + -0x1;
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
        BVar4 = IsWindowEnabled16((HWND16)s_tile2_bmp_1050_1538);
        if (BVar4 == 0x0) {
          GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
          EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
        }
        if (*(int *)(iVar7 + 0x96) < 0x0) {
          CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,0x0,*(UINT16 *)(iVar7 + 0x98));
          *(undefined2 *)(iVar7 + 0x96) = 0x0;
        }
        *(uint *)(iVar7 + 0x98) = param_3;
        pass1_1018_1c9a(*(ulong *)(iVar7 + 0x92),param_3);
        puStack14 = (ushort *)pass1_1018_1e78(*(ulong *)(iVar7 + 0x92),-0x1);
        uVar5 = (uint)((ulong)puStack14 >> 0x10);
        if (puStack14 == (ushort *)0x0) {
          puStack16 = (undefined *)0x0;
        }
        else {
          puStack16 = (undefined *)*(ushort *)((uint)puStack14 + 0x1c);
        }
        win_1008_5c7c(_PTR_LOOP_1050_02a0,CONCAT22(puStack16,0x1),param_6,(ushort)puStack16,uVar5 | (uint)puStack14);
      }
    }
    BStack6 = 0x1;
    uStack4 = 0x0;
  }
  else {
    if (param_3 == 0xfb1) {
      for (uStack1132 = 0x1a0; uVar13 = CONCAT22(param_4,HStack8), uStack1132 < 0x1b5; uStack1132 = uStack1132 + 0x1) {
        UVar3 = IsDlgButtonChecked(param_5,uStack1132);
        if (UVar3 == 0x1) {
          pass1_1008_d818(*(ulong *)(iVar7 + 0x8e),uStack1132);
          uVar13 = CONCAT22(param_4,HStack8);
          goto LAB_1038_bba2;
        }
        param_5 = (HWND16)s_tile2_bmp_1050_1538;
      }
    }
    else {
      if (param_3 != 0xfbe) goto LAB_1038_bbbf;
      puStack14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_6,(uchar *)param_4,unaff_DI);
      puStack16 = PTR_LOOP_1050_13ae;
      if (PTR_LOOP_1050_13ae == (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1)) {
        puStack16 = (undefined *)&PTR_LOOP_1050_0002;
      }
      iStack10 = *(int *)((int)puStack16 * 0xc + 0x5b84) + -0x1;
      pass1_1008_612e(0x0,iStack10,iStack10);
      uStack20 = pass1_1018_1e78(*(ulong *)(iVar7 + 0x92),*(int *)(((int)puStack16 * 0x6 + iStack10) * 0x2 + 0x5b86));
      load_string_1010_84e0
                (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x50,local_464,
                 (short)param_6);
      pcVar11 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      puVar6 = (uchar *)((ulong)pcVar11 >> 0x10);
      uVar5 = wsprintf16((LPSTR)0x1010,local_414,&param_6->style);
      uVar9 = 0x1000;
      mem_op_1000_179c(0xb4,puVar6,0x1000);
      if (((uint)puVar6 | uVar5) == 0x0) {
        paVar12 = (astruct_57 *)0x0;
      }
      else {
        pCVar14 = local_414;
        pWVar15 = param_6;
        puVar16 = PTR_LOOP_1050_0396;
        pcVar11 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
        uVar9 = SUB42(&PTR_LOOP_1050_1040,0x0);
        paVar12 = pass1_1040_8478((astruct_57 *)CONCAT22(puVar6,puVar16),0x41,pcVar11,(char *)CONCAT22(pWVar15,pCVar14),
                                  (ushort)puVar16,(ushort)((ulong)pcVar11 >> 0x10));
      }
      ppcVar2 = (code **)((int)*(undefined4 *)paVar12 + 0x74);
      uVar13 = (**ppcVar2)(uVar9,(int)paVar12);
      if ((int)uVar13 != 0x1) goto LAB_1038_bc26;
      pass1_1008_d818(*(ulong *)(iVar7 + 0x8e),*(int *)((int)uStack20 + 0x1a));
      HStack8 = (HWND16)uVar13;
LAB_1038_bba2:
      param_5 = 0x1008;
      win_ui_cursor_op_1038_bc30((ulong)param_1,0x1008,(ushort)param_6);
      HStack8 = (HWND16)uVar13;
    }
    PostMessage16(param_5,0x0,0x0,0x11100ce);
    HStack8 = (HWND16)uVar13;
    param_3 = 0x1;
LAB_1038_bbbf:
    BStack6 = post_win_msg_1040_7b3c(param_1,(ushort)param_2,(ushort)(param_2 >> 0x10),param_3,(int)&PTR_LOOP_1050_1040)
    ;
    uStack4 = (undefined2)((ulong)uVar13 >> 0x10);
    HStack8 = (HWND16)uVar13;
  }
LAB_1038_bc26:
  return CONCAT22(uStack4,BStack6);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_cursor_op_1038_bc30(ulong param_1,HINSTANCE16 param_2,ushort param_3)

{
  undefined4 uVar1;
  uchar in_AF;
  undefined2 local_112;
  undefined2 uStack272;
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;
  
  HStack4 = LoadCursor16(param_2,(LPCSTR)0x7f02);
  HStack6 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  uVar1 = *(undefined4 *)((int)param_1 + 0x8e);
  pass1_1030_532e((astruct_100 *)CONCAT22(param_3,&local_112),(long)*(int *)((int)uVar1 + 0xe) + 0x1000000,param_3,in_AF
                 );
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_3,&local_112));
  pass1_1030_838e((ulong *)_PTR_LOOP_1050_5748,param_3,in_AF);
  local_112 = 0x389a;
  uStack272 = 0x1008;
  pass1_1030_8334(_PTR_LOOP_1050_5748);
  SetCursor16(0x1030);
  return;
}



void __stdcall16far pass1_1038_bca8(ulong param_1)

{
  uint uVar1;
  code **ppcVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  uchar *extraout_DX_00;
  uchar *puVar7;
  int iVar8;
  undefined2 uVar9;
  
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar8 = (int)param_1;
  uVar3 = *(undefined4 *)(iVar8 + 0x8e);
  puVar5 = (undefined4 *)*(undefined4 *)((int)uVar3 + 0xa);
  ppcVar2 = (code **)((int)*puVar5 + 0x14);
  (**ppcVar2)();
  puVar4 = (undefined4 *)puVar5;
  puVar6 = extraout_DX;
  if (*(long *)(iVar8 + 0x70) != 0x0) {
    puVar4 = (undefined4 *)*(uint *)(iVar8 + 0x70);
    uVar1 = *(uint *)(iVar8 + 0x72);
    puVar6 = (uchar *)(uVar1 | (uint)puVar4);
    if (puVar6 != (uchar *)0x0) {
      ppcVar2 = (code **)*puVar4;
      (**ppcVar2)();
      puVar6 = extraout_DX_00;
    }
  }
  mem_op_1000_179c(0x14,puVar6,0x1000);
  puVar7 = (uchar *)((uint)puVar6 | (uint)puVar4);
  if (puVar7 == (uchar *)0x0) {
    puVar4 = (undefined4 *)0x0;
    puVar7 = (uchar *)0x0;
  }
  else {
    struct_1008_4c58((ushort *)CONCAT22(puVar6,puVar4));
  }
  *(undefined2 *)(iVar8 + 0x70) = puVar4;
  *(uchar **)(iVar8 + 0x72) = puVar7;
  pass1_1008_4d84(*(astruct_90 **)(iVar8 + 0x70),(ulong)puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10,puVar7);
  return;
}



astruct_18 * __stdcall16far pass1_1038_bd4a(astruct_18 *param_1,byte param_2)

{
  pass1_1038_b7f0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_bddc(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_706 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x176,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_706 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  iVar1->field_0x96 = 0x0;
  iVar1->field_0x98 = 0x0;
  iVar1->field_0x9a = 0x0;
  iVar1->field_0x9c = 0x0;
  *(undefined2 *)param_1 = 0xc436;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_8,param_6,param_7);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_be4a(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xc436;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_be76(ushort param_1,ulong param_2,uchar *param_3,int param_4,ushort param_5)

{
  ushort *puVar1;
  int iVar2;
  
  if (param_2._2_2_ == 0x0) {
    iVar2 = 0x0;
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
    pass1_1010_038e((ulong)puVar1,iVar2,param_5);
  }
  destroy_win_1040_7b98(CONCAT22((int)param_2,param_1),(int)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_dlg_op_1038_bea4(ulong param_1,WORD *param_2)

{
  undefined4 uVar1;
  HWND16 HVar2;
  uchar *in_DX;
  uchar *puVar3;
  uint uVar4;
  WPARAM16 wparam;
  int iVar5;
  int unaff_DI;
  undefined2 uVar6;
  ushort *puVar7;
  ulong uVar8;
  char *pcVar9;
  LRESULT LVar10;
  ulong *local_116;
  ulong *local_112;
  CHAR local_10e [0x82];
  undefined local_8c [0x82];
  undefined4 uStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_2,in_DX,unaff_DI);
  puVar3 = (uchar *)((ulong)puStack6 >> 0x10);
  uStack10 = *(undefined4 *)((int)puStack6 + 0x68);
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  GetWindowText16(0x1010,0x80,(INT16)local_8c);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_10e,param_2);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_10e);
  HVar2 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x179);
  *(HWND16 *)(iVar5 + 0x92) = HVar2;
  pass1_1008_e3ec(*(ulong *)(iVar5 + 0x8e),(ulong *)CONCAT22(param_2,&local_116),(ulong *)CONCAT22(param_2,&local_112),
                  (ushort)param_2);
  send_msg_1038_c374(param_1,local_112,*(uint *)(iVar5 + 0x92),0x1008);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,(ushort)param_2,puVar3,unaff_DI);
  uVar4 = (uint)((ulong)puVar7 >> 0x10);
  uVar8 = *(ulong *)((int)puVar7 + 0x24);
  uVar1 = *(undefined4 *)(iVar5 + 0x8e);
  uVar8 = string_1008_e586((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),uVar8,(uint)uVar8,uVar4);
  SendMessage16(0x1008,(UINT16)uVar8,(WPARAM16)(uVar8 >> 0x10),0x40dffff);
  HVar2 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x17a);
  *(HWND16 *)(iVar5 + 0x94) = HVar2;
  send_msg_1038_c374(param_1,local_116,HVar2,(int)s_tile2_bmp_1050_1538);
  pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  wparam = (WPARAM16)((ulong)pcVar9 >> 0x10);
  LVar10 = SendMessage16(0x1010,(UINT16)pcVar9,wparam,0x4030000);
  *(undefined2 *)(iVar5 + 0x9c) = (int)LVar10;
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)pcVar9,wparam,0x40dffff);
  HVar2 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x178);
  *(HWND16 *)(iVar5 + 0x96) = HVar2;
  HVar2 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x177);
  *(HWND16 *)(iVar5 + 0x98) = HVar2;
  HVar2 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x184);
  *(HWND16 *)(iVar5 + 0x9a) = HVar2;
  return;
}



void __stdcall16far show_win_1038_c044(astruct_1 *param_1)

{
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far msg_box_op_1038_c07a(int param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5)

{
  int iVar1;
  ushort uVar2;
  undefined2 unaff_CS;
  HWND16 hwnd;
  uchar in_AF;
  uchar local_70c [0x200];
  char local_50c [0x100];
  char local_40c [0x402];
  ulong uStack10;
  undefined4 uStack6;
  
  send_msg_1038_c228(CONCAT22(param_2,param_1),unaff_CS);
  uStack6 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  if (param_4._2_2_ == 0x177) {
    pass1_1008_e05e(*(ulong *)(param_1 + 0x8e),0x2,CONCAT22(param_2,param_1 + 0x19eU),CONCAT22(param_2,param_1 + 0x9e),
                    param_5,in_AF);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x200,local_40c,param_5);
    sys_1000_3f9c(local_70c,(uchar *)param_5,(ushort)local_40c,param_5,param_1 + 0x19eU,&stack0xfffe,param_2,0x1000,
                  param_5,in_AF);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_50c,param_5);
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    MessageBox16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x10),local_50c,param_5);
  }
  else {
    if (param_4._2_2_ != 0x178) {
      if ((param_4._2_2_ != 0x178) && (param_4._2_2_ - 0x179U < 0x2)) {
        set_win_pos_1038_c31a(CONCAT22(param_2,param_1),param_3,(ushort)param_4,0x1010);
        return;
      }
      post_win_msg_1040_7b3c
                ((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4._2_2_,(int)&PTR_LOOP_1050_1040);
      return;
    }
    uStack10 = CONCAT22(param_2,param_1 + 0x9e);
    uVar2 = param_2;
    iVar1 = pass1_1008_e10c(*(ulong *)(param_1 + 0x8e),CONCAT22(param_2,param_1 + 0x19e),
                            CONCAT22(param_2,param_1 + 0x9e),param_2,param_5);
    if (iVar1 == 0x0) {
      load_string_1010_84e0
                (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_40c,param_5
                );
      load_string_1010_84e0
                (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_50c,param_5
                );
      MessageBox16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x10),local_50c,param_5);
      return;
    }
    hwnd = 0x1008;
    pass1_1008_e01c(*(ulong *)(param_1 + 0x8e),CONCAT22(param_2,param_1 + 0x19e),uStack10);
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(param_1 + 0x8),0x1f,uVar2,param_1,0x1008,param_5);
  }
  PostMessage16(hwnd,0x0,0x0,0x1110002);
  return;
}



LRESULT __stdcall16far send_msg_1038_c228(ulong param_1,HWND16 param_2)

{
  WPARAM16 wparam;
  LRESULT LVar1;
  LRESULT LVar2;
  
  wparam = (WPARAM16)(param_1 >> 0x10);
  LVar1 = SendMessage16(param_2,0x0,0x0,0x4070000);
  LVar2 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,(int)param_1 + 0x9e,wparam,CONCAT22(0x408,(int)LVar1));
  LVar1 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,(int)param_1 + 0x19e,wparam,CONCAT22(0x408,(int)LVar2));
  return LVar1;
}



void __stdcall16far enable_win_1038_c294(ulong param_1)

{
  SEGPTR lp_string;
  ushort uVar1;
  undefined2 unaff_SS;
  ulong uStack12;
  
  lp_string = (int)param_1 + 0x9e;
  uStack12 = param_1 & 0xffff0000 | (ulong)lp_string;
  pass1_1008_e320(*(astruct_102 **)((int)param_1 + 0x8e),param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x19eU),
                  param_1 & 0xffff0000 | (ulong)lp_string,unaff_SS);
  SetWindowText16(0x1008,lp_string);
  uVar1 = pass1_1008_e2a4(*(ulong *)((int)param_1 + 0x8e),param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x19eU),uStack12
                         );
  EnableWindow16(0x1008,uVar1 & 0x1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,uVar1 & 0x2);
  return;
}



BOOL16 __stdcall16far set_win_pos_1038_c31a(ulong param_1,ushort param_2,int param_3,HWND16 param_4)

{
  RECT16 local_e;
  int iStack10;
  ushort uStack6;
  int iStack4;
  
  iStack4 = param_3;
  uStack6 = param_2;
  if (param_3 == 0x1) {
    enable_win_1038_c294(param_1);
  }
  else {
    if (param_3 != 0x7) {
      return 0x0;
    }
    GetWindowRect16(param_4,&local_e);
    iStack10 = iStack10 - local_e.x;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x2,0x50,iStack10,0x0,0x0,0x0);
  }
  return 0x1;
}



void __stdcall16far send_msg_1038_c374(ulong param_1,ulong *param_2,uint param_3,HWND16 param_4)

{
  undefined4 uVar1;
  code **ppcVar2;
  undefined2 uVar3;
  ulong uVar4;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  uint uVar5;
  undefined2 uVar6;
  LRESULT LVar7;
  astruct_18 *paVar8;
  uint uVar9;
  ulong uStack10;
  ulong uStack6;
  
  uVar6 = SUB42(s_tile2_bmp_1050_1538,0x0);
  uVar9 = param_3;
  LVar7 = SendMessage16(param_4,0x0,0x0,0x40b0000);
  uVar3 = (undefined2)LVar7;
  uVar5 = (uint)param_2;
  ppcVar2 = (code **)((int)*param_2 + 0x10);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,param_2,uVar9);
  uStack6 = CONCAT22(extraout_DX,uVar3);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar2 = (code **)((int)*param_2 + 0x4);
    uVar4 = uStack6;
    (**ppcVar2)(uVar6,param_2,(char)uStack10,(int)(uStack10 >> 0x10),uVar5);
    uVar1 = *(undefined4 *)((int)param_1 + 0x8e);
    paVar8 = (astruct_18 *)
             string_1008_e586((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),
                              uVar4 & 0xffff | (ulong)extraout_DX_00 << 0x10,(uint)uVar4,extraout_DX_00);
    uVar5 = param_3;
    LVar7 = SendMessage16(0x1008,(UINT16)paVar8,(WPARAM16)((ulong)paVar8 >> 0x10),0x4030000);
    uVar6 = 0x1000;
    fn_ptr_1000_17ce(paVar8,0x1000);
    if (LVar7 == -0x1) break;
    if (LVar7 == -0x2) {
      return;
    }
    uStack10 = uStack10 + 0x1;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_c410(astruct_18 *param_1,byte param_2)

{
  pass1_1038_be4a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_c4a2(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_708 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x17c,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_708 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x96 = 0x0;
  *(undefined2 *)param_1 = 0xc74c;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_8,param_6,param_7);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_c4fe(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xc74c;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_c52a(ushort param_1,ulong param_2,uchar *param_3,int param_4,ushort param_5)

{
  ushort *puVar1;
  int iVar2;
  
  if (param_2._2_2_ == 0x0) {
    iVar2 = 0x0;
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
    pass1_1010_038e((ulong)puVar1,iVar2,param_5);
  }
  destroy_win_1040_7b98(CONCAT22((int)param_2,param_1),(int)&PTR_LOOP_1050_1040);
  return;
}



void __stdcall16far show_win_1038_c558(astruct_1 *param_1)

{
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_dlg_op_1038_c58e(ulong param_1,WORD *param_2)

{
  uchar *in_DX;
  int iVar1;
  int unaff_DI;
  CHAR local_80e [0x402];
  CHAR local_40c [0x402];
  undefined4 uStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_2,in_DX,unaff_DI);
  uStack10 = *(undefined4 *)((int)puStack6 + 0x68);
  iVar1 = (int)param_1;
  GetWindowText16(0x1010,0x80,(INT16)local_40c);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_80e,param_2);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_80e);
  pass1_1008_e038(*(ulong *)(iVar1 + 0x8e),(ulong *)(param_1 & 0xffff0000 | (ulong)(iVar1 + 0x92)),
                  (ulong *)(param_1 & 0xffff0000 | (ulong)(iVar1 + 0x96)));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x400,local_80e,
             (short)param_2);
  wsprintf16((LPSTR)0x1010,local_40c,param_2);
  SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)local_40c,(SEGPTR)param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far message_box_op_1038_c672(int param_1,ushort param_2,ushort param_3,ulong param_4,short param_5)

{
  undefined4 uVar1;
  HWND16 hwnd;
  uchar in_AF;
  ushort uVar2;
  char local_404 [0x402];
  
  uVar2 = (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10);
  if (param_4._2_2_ == 0x17d) {
    load_string_1010_84e0(0x1010,(ushort)_PTR_LOOP_1050_14cc,uVar2,0x3ff,local_404,param_5);
    uVar1 = *(undefined4 *)(param_1 + 0x92);
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    MessageBox16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x10),(LPCSTR)uVar1,
                 (UINT16)((ulong)uVar1 >> 0x10));
  }
  else {
    if (param_4._2_2_ != 0x17e) {
      post_win_msg_1040_7b3c
                ((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4._2_2_,(int)&PTR_LOOP_1050_1040);
      return;
    }
    load_string_1010_84e0(0x1010,(ushort)_PTR_LOOP_1050_14cc,uVar2,0x3ff,local_404,param_5);
    uVar1 = *(undefined4 *)(param_1 + 0x92);
    MessageBox16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x10),(LPCSTR)uVar1,
                 (UINT16)((ulong)uVar1 >> 0x10));
    hwnd = 0x1008;
    pass1_1008_e164(*(ulong *)(param_1 + 0x8e),param_5,in_AF);
  }
  PostMessage16(hwnd,0x0,0x0,0x1110002);
  return;
}



astruct_18 * __stdcall16far pass1_1038_c726(astruct_18 *param_1,byte param_2)

{
  pass1_1038_c4fe(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * __stdcall16far
pass1_1038_c7b8(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               ushort param_7)

{
  astruct_435 *iVar1;
  int unaff_DI;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfb8,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_435 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  *(undefined2 *)param_1 = 0xca6c;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x5,param_7,param_6,unaff_DI);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_c80a(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xca6c;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far destroy_window_1038_c836(int param_1,ULONG param_2,ULONG param_3,UINT16 param_4)

{
  undefined4 uVar1;
  ushort *puVar2;
  undefined local_6 [0x4];
  
  if (param_3._2_2_ == 0xfce) {
    puVar2 = pass1_1008_941a((ushort *)CONCAT22(param_4,local_6),0x1,0xac);
    win_1008_5c9e(_PTR_LOOP_1050_02a0,(ulong *)CONCAT22(param_4,local_6),local_6,(int)((ulong)puVar2 >> 0x10),param_4);
    uVar1 = *(undefined4 *)(param_1 + 0x8e);
    *(undefined2 *)((int)uVar1 + 0xa) = 0x6;
    DestroyWindow16(0x1008);
    PTR_LOOP_1050_5b80 = (undefined *)0x0;
    return;
  }
  post_win_msg_1040_7b3c
            ((ulong *)CONCAT22((undefined2)param_2,param_1),(ushort)(param_2 >> 0x10),(ushort)param_3,param_3._2_2_,
             (int)&PTR_LOOP_1050_1040);
  return;
}



void __stdcall16far win_ui_op_1038_c89c(astruct_1 *param_1)

{
  int iVar1;
  undefined4 uVar2;
  HWND16 HVar3;
  undefined2 uVar4;
  BOOL16 enable;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  CheckRadioButton16((HWND16)&PTR_LOOP_1050_1040,0xfac,0xfad,0xfac);
  uVar2 = *(undefined4 *)((int)param_1 + 0x8e);
  *(undefined2 *)((int)uVar2 + 0xa) = 0x1;
  uVar2 = *(undefined4 *)((int)param_1 + 0x8e);
  iVar1 = *(int *)((int)uVar2 + 0x12);
  if (iVar1 == 0x4) {
LAB_1038_c8da:
    HVar3 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfce);
    if (HVar3 != 0x0) {
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    }
    HVar3 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1);
    if (HVar3 == 0x0) goto LAB_1038_c93c;
    enable = 0x0;
  }
  else {
    if ((iVar1 + -0x5 < 0x1) || (SBORROW2(iVar1 + -0x5,0x1))) goto LAB_1038_c93c;
    if (iVar1 != 0x8 && 0x0 < iVar1 + -0x7) {
      if (iVar1 != 0x9) goto LAB_1038_c93c;
      goto LAB_1038_c8da;
    }
    HVar3 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfce);
    if (HVar3 == 0x0) goto LAB_1038_c93c;
    enable = 0x1;
  }
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,enable);
LAB_1038_c93c:
  move_win_1040_826c(param_1,0xc8,0x0);
  ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  return;
}
