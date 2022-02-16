


astruct_18 * __stdcall16far pass1_1040_2358(astruct_18 *param_1,byte param_2)

{
  pass1_1040_205e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_23ea(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6,
               ushort param_7)

{
  undefined4 uVar1;
  astruct_436 *iVar2;
  int unaff_DI;
  undefined2 uVar2;
  ushort *puVar3;
  
  get_sys_metrics_1040_7728(param_1,0x9a,param_2,0xfbd,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_436 *)param_1;
  *(undefined4 *)&iVar2->field_0x8e = 0x0;
  iVar2->field_0x92 = 0x0;
  iVar2->field_0x94 = 0x0;
  *(undefined2 *)param_1 = 0x2956;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  iVar2->field_0x8a = 0x26;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,param_6,(uchar *)param_7,unaff_DI);
  iVar2->field_0x8e = (int)puVar3;
  iVar2->field_0x90 = (int)((ulong)puVar3 >> 0x10);
  uVar1 = *(undefined4 *)&iVar2->field_0x8e;
  iVar2->field_0x92 = *(undefined2 *)((int)uVar1 + 0x28);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_2464(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x2956;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



void __stdcall16far show_win_1040_2490(astruct_1 *param_1,HWND16 param_2)

{
  code **ppcVar1;
  undefined2 uVar2;
  astruct_1 *iVar4;
  undefined2 uVar3;
  int *piVar4;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_1 *)param_1;
  GetDlgItem16(param_2,0xfb1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
                    // WARNING: Load size is inaccurate
  ppcVar1 = (code **)((int)*iVar4->field_0x8e + 0x10);
  piVar4 = (int *)(**ppcVar1)((int)s_tile2_bmp_1050_1538,*(undefined4 *)&iVar4->field_0x8e);
  uVar2 = (undefined2)((ulong)piVar4 >> 0x10);
  move_win_1040_826c(param_1,*(int *)((int)piVar4 + 0x2) + -0x2,*(int *)((int)piVar4 + 0x4) + *piVar4 + 0x3);
  ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x5);
  pass1_1018_1c9a(*(ulong *)&iVar4->field_0x8e,0x1a0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far
win_ui_op_1040_2512(ulong *param_1,ulong param_2,UINT16 param_3,HWND16 param_4,WNDCLASS16 *param_5,uchar *param_6)

{
  int *piVar1;
  code **ppcVar2;
  uint uVar3;
  BOOL16 BVar4;
  int iVar5;
  int iVar6;
  UINT16 UVar7;
  uchar *puVar8;
  int unaff_DI;
  undefined2 uVar9;
  undefined2 uVar10;
  HWND16 hwnd;
  undefined in_AF;
  undefined4 uVar11;
  undefined local_1e [0x4];
  uint uStack26;
  uchar *puStack24;
  ushort *local_16 [0x2];
  ushort uStack12;
  undefined4 *puStack10;
  BOOL16 BStack6;
  uchar *puStack4;
  
  BStack6 = 0x0;
  puStack4 = (uchar *)0x0;
  if (param_3 == 0x2) {
LAB_1040_266d:
    BStack6 = 0x1;
    puStack4 = (uchar *)0x0;
  }
  else {
    uVar9 = (undefined2)((ulong)param_1 >> 0x10);
    if (0x19d < param_3 - 0x2) {
      iVar5 = (int)param_1;
      if (param_3 - 0x1a0 < 0x14 || param_3 == 0x1b4) {
        UVar7 = IsDlgButtonChecked(param_4,param_3);
        if (UVar7 == 0x0) {
          piVar1 = (int *)(iVar5 + 0x92);
          *piVar1 = *piVar1 + 0x1;
          if (0x0 < *(int *)(iVar5 + 0x92)) {
            *(undefined2 *)(iVar5 + 0x94) = 0x0;
          }
          uVar11 = *(undefined4 *)(iVar5 + 0x8e);
          if (*(int *)((int)uVar11 + 0x28) == *(int *)(iVar5 + 0x92)) {
            GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
          }
        }
        else {
          piVar1 = (int *)(iVar5 + 0x92);
          *piVar1 = *piVar1 + -0x1;
          GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
          BVar4 = IsWindowEnabled16((HWND16)s_tile2_bmp_1050_1538);
          if (BVar4 == 0x0) {
            GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
          }
          if (*(int *)(iVar5 + 0x92) < 0x1) {
            *(undefined2 *)(iVar5 + 0x94) = 0x1;
          }
          pass1_1018_1c9a(*(ulong *)(iVar5 + 0x8e),param_3);
          puStack10 = (undefined4 *)pass1_1018_1e78(*(ulong *)(iVar5 + 0x8e),-0x1);
          uVar3 = (uint)((ulong)puStack10 >> 0x10);
          if (puStack10 == (undefined4 *)0x0) {
            uStack12 = 0x0;
          }
          else {
            uStack12 = *(ushort *)((uint)puStack10 + 0x1c);
          }
          win_1008_5c7c(_PTR_LOOP_1050_02a0,CONCAT22(uStack12,0x1),param_5,uStack12,uVar3 | (uint)puStack10);
        }
        if ((-0x1 < *(int *)(iVar5 + 0x92)) &&
           (uVar11 = *(undefined4 *)(iVar5 + 0x8e), *(int *)(iVar5 + 0x92) <= *(int *)((int)uVar11 + 0x28))) {
          sys_1000_3f9c((uchar *)local_16,(uchar *)param_5,0x5cf4,(ushort)&USHORT_1050_1050,*(ushort *)(iVar5 + 0x92),
                        &stack0xfffe,uVar9,0x1000,param_5,in_AF);
          SetDlgItemText16(0x1000,(INT16)local_16,(SEGPTR)param_5);
        }
        goto LAB_1040_266d;
      }
      uVar3 = param_3 - 0xfb1;
      if (uVar3 == 0x0) {
        if (*(int *)(iVar5 + 0x92) < 0x0) {
          mem_op_1000_179c(0xb4,param_6,0x1000);
          puVar8 = (uchar *)((uint)param_6 | uVar3);
          uStack26 = uVar3;
          puStack24 = param_6;
          if (puVar8 == (uchar *)0x0) {
            iVar5 = 0x0;
            puVar8 = (uchar *)0x0;
          }
          else {
            iVar5 = string_1040_8520((astruct_57 *)CONCAT22(param_6,uVar3),(ushort)PTR_LOOP_1050_0396,0x30,0x2,0x57b,
                                     0x57c,puVar8,(ushort)param_5);
          }
          puStack10 = (undefined4 *)CONCAT22(puVar8,iVar5);
          ppcVar2 = (code **)((int)*puStack10 + 0x74);
          (**ppcVar2)(0x1000,iVar5,puVar8);
          goto LAB_1040_27c0;
        }
        if (0x0 < *(int *)(iVar5 + 0x92)) {
          mem_op_1000_179c(0xb4,param_6,0x1000);
          puVar8 = (uchar *)((uint)param_6 | uVar3);
          uStack26 = uVar3;
          puStack24 = param_6;
          if (puVar8 == (uchar *)0x0) {
            iVar6 = 0x0;
            puVar8 = (uchar *)0x0;
          }
          else {
            iVar6 = string_1040_8520((astruct_57 *)CONCAT22(param_6,uVar3),(ushort)PTR_LOOP_1050_0396,0x21,0x2,0x57b,
                                     0x57d,puVar8,(ushort)param_5);
          }
          puStack10 = (undefined4 *)CONCAT22(puVar8,iVar6);
          pass1_1008_941a((ushort *)CONCAT22(param_5,local_1e),0x1,0xc2);
          ppcVar2 = (code **)((int)*puStack10 + 0x6c);
          uVar11 = (**ppcVar2)(0x1008,(char)puStack10,(int)((ulong)puStack10 >> 0x10),local_1e,param_5);
          param_6 = (uchar *)((ulong)uVar11 >> 0x10);
          if ((int)uVar11 == 0x2) goto LAB_1040_27c0;
        }
        local_16[0] = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,(ushort)param_5,param_6,unaff_DI);
        param_6 = (uchar *)((ulong)local_16[0] >> 0x10);
        uStack12 = 0x1a0;
        hwnd = 0x1010;
        do {
          UVar7 = IsDlgButtonChecked(hwnd,uStack12);
          if (UVar7 == 0x1) {
            uVar10 = (undefined2)((ulong)local_16[0] >> 0x10);
            iVar6 = (int)local_16[0];
            *(UINT16 *)(iVar6 + *(int *)(iVar6 + 0x56) * 0x2 + 0x4e) = uStack12;
            piVar1 = (int *)(iVar6 + 0x56);
            *piVar1 = *piVar1 + 0x1;
          }
          uStack12 = uStack12 + 0x1;
          hwnd = (HWND16)s_tile2_bmp_1050_1538;
        } while ((int)uStack12 < 0x1b5);
        uVar3 = *(uint *)(iVar5 + 0x92);
        puStack10 = (undefined4 *)((ulong)puStack10 & 0xffff0000 | (ulong)uVar3);
        uVar11 = *(undefined4 *)(iVar5 + 0x8e);
        *(uint *)((int)uVar11 + 0x28) = uVar3;
        param_4 = (HWND16)s_tile2_bmp_1050_1538;
        PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x11100c8);
        param_3 = 0x1;
      }
    }
    BStack6 = post_win_msg_1040_7b3c(param_1,(ushort)param_2,(ushort)(param_2 >> 0x10),param_3,param_4);
    puStack4 = param_6;
  }
LAB_1040_27c0:
  return CONCAT22(puStack4,BStack6);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined4 __stdcall16far draw_ui_op_1040_27cc(ulong *param_1,ushort param_2,uint param_3,COLORREF param_4)

{
  code **ppcVar1;
  undefined2 uVar2;
  int iVar3;
  HBRUSH16 HVar4;
  INT16 IVar5;
  int iVar6;
  undefined2 uVar7;
  COLORREF CVar8;
  HWND16 hdc;
  ulong uVar9;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  CVar8 = param_4;
  if (*(int *)(iVar6 + 0x4) == 0x0) {
    CVar8 = (COLORREF)s_tile2_bmp_1050_1538;
    HVar4 = CreateSolidBrush16(param_4);
    *(HBRUSH16 *)(iVar6 + 0x4) = HVar4;
  }
  if (_PTR_LOOP_1050_5cf8 == 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x68);
    uVar9 = (**ppcVar1)(CVar8,param_1,*(undefined2 *)(iVar6 + 0x6e));
    CVar8 = 0x1008;
    uVar9 = pass1_1008_4d72(uVar9);
    uVar2 = (undefined2)(uVar9 >> 0x10);
    iVar3 = (int)uVar9;
    _PTR_LOOP_1050_5cf8 =
         CONCAT22(CONCAT11(0x2,*(undefined *)(iVar3 + 0x94)),
                  CONCAT11(*(undefined *)(iVar3 + 0x95),*(undefined *)(iVar3 + 0x96)));
  }
  hdc = CVar8;
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return 0x0;
    }
    hdc = (HWND16)s_tile2_bmp_1050_1538;
    IVar5 = GetDlgCtrlID16(CVar8);
    if ((*(int *)(iVar6 + 0x94) != 0x0) && (IVar5 == 0xfb2)) {
      CVar8 = 0x0;
      goto LAB_1040_286e;
    }
  }
  CVar8 = (COLORREF)_PTR_LOOP_1050_5cf8;
LAB_1040_286e:
  SetTextColor16(hdc,CVar8);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
  return CONCAT22(0x1050,*(undefined2 *)(iVar6 + 0x4));
}



void __stdcall16far pass1_1040_288e(ulong param_1)

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
  puVar5 = (undefined4 *)*(undefined4 *)((int)uVar3 + 0x24);
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



astruct_18 * __stdcall16far pass1_1040_2930(astruct_18 *param_1,byte param_2)

{
  pass1_1040_2464(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * __stdcall16far
pas1_1040_29c2(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5)

{
  int iVar1;
  undefined2 uVar2;
  
  pass1_1040_b0bc(param_1,param_2,CONCAT22(param_3,0x157));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)param_1 = 0x2e26;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  *(ushort *)(iVar1 + 0x94) = param_4;
  *(ushort *)(iVar1 + 0x96) = param_5;
  load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  *(ushort *)(iVar1 + 0x98) = param_4;
  *(ushort *)(iVar1 + 0x9a) = param_5;
  return param_1;
}



void __stdcall16far pass1_1040_2a22(astruct_18 *param_1)

{
  astruct_625 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_625 *)param_1;
  param_1->field_0x0 = 0x2e26;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(iVar1->field_0x94,0x1000);
  fn_ptr_1000_17ce(iVar1->field_0x98,0x1000);
  unk_draw_op_1040_b0f8(param_1);
  return;
}



void __stdcall16far dlg_ui_op_1040_2a64(astruct_1 *param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  astruct_160 *paVar2;
  ushort uVar3;
  uchar *puVar4;
  uchar *puVar5;
  int iVar6;
  undefined2 uVar7;
  HWND16 hwnd;
  HWND16 hwnd_00;
  int iVar8;
  RECT16 local_16;
  undefined2 uStack18;
  undefined2 uStack16;
  int iStack14;
  ulong uStack12;
  undefined4 uStack8;
  int iStack4;
  
  unk_win_ui_op_1040_b230(param_1,param_2,param_3);
  iStack4 = 0x5;
  iVar8 = 0x0;
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar6 + 0x90);
  uStack12 = struct_op_1030_73a8(*(ulong *)((int)uVar1 + 0x6));
  puVar4 = (uchar *)(uStack12 >> 0x10);
  hwnd = (HWND16)&USHORT_1050_1028;
  PTR_LOOP_1050_5d04 = (undefined *)pass1_1028_4a9a(uStack12,iVar8);
  for (iStack14 = 0x0; iStack14 < iStack4; iStack14 = iStack14 + 0x1) {
    if (iStack14 != 0x0) {
      *(undefined2 *)((int)&PTR_LOOP_1050_5d04 + iStack14 * 0xc) = 0x0;
    }
    iVar8 = iStack14 * 0xc;
    local_16.x = *(INT16 *)(iVar8 + 0x5cfc);
    local_16.y = *(INT16 *)(iVar8 + 0x5cfe);
    paVar2 = (astruct_160 *)((int)&PTR_LOOP_1050_0000 + 0x1);
    uStack18 = 0x1;
    uStack16 = 0x1;
    MapDialogRect16(hwnd,&local_16);
    hwnd_00 = 0x1000;
    mem_op_1000_179c(0x42,puVar4,0x1000);
    puVar5 = (uchar *)((uint)puVar4 | (uint)paVar2);
    if (puVar5 == (uchar *)0x0) {
      paVar2 = (astruct_160 *)0x0;
      puVar4 = (uchar *)0x0;
    }
    else {
      hwnd_00 = 0x1008;
      pass1_1008_3bd6(paVar2,(ushort)puVar4,0x1,CONCAT22(local_16.x,local_16.y),0x101,0xff0100,
                      CONCAT22(*(undefined2 *)(iVar6 + 0x6),*(undefined2 *)(iVar8 + 0x5d00)),(ushort)puVar5,param_3);
      puVar4 = puVar5;
    }
    uStack8 = CONCAT22(puVar4,paVar2);
    if (PTR_LOOP_1050_5d04 == (undefined *)0x0) {
      hwnd = hwnd_00;
      if ((iStack14 != 0x0) && (((uint)puVar4 | (uint)paVar2) != 0x0)) {
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
        EnableWindow16(hwnd_00,0x0);
      }
    }
    else {
      iVar8 = iStack14 * 0xc;
      uVar3 = pass1_1028_4a9a(uStack12,*(int *)(iVar8 + 0x5d02));
      hwnd = (HWND16)&USHORT_1050_1028;
      if (uVar3 != 0x0) {
        *(undefined2 *)((int)&PTR_LOOP_1050_5d04 + iVar8) = 0x1;
        uVar1 = *(undefined4 *)(iVar6 + 0x98);
        SetDlgItemText16((HWND16)&USHORT_1050_1028,(INT16)uVar1,(SEGPTR)((ulong)uVar1 >> 0x10));
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
      }
    }
  }
  return;
}



void __stdcall16far win_ui_op_1040_2bb2(int param_1,ushort param_2,ushort param_3,ulong param_4,HWND16 param_5)

{
  uint uVar1;
  uchar *in_DX;
  ushort unaff_SS;
  undefined4 uVar2;
  int iStack8;
  int iStack4;
  
  if (param_4._2_2_ == 0x158) {
    PTR_LOOP_1050_5d04 = (undefined *)(uint)(PTR_LOOP_1050_5d04 == (undefined *)0x0);
    if (PTR_LOOP_1050_5d04 == (undefined *)0x0) {
      for (iStack8 = 0x1; iStack8 < 0x5; iStack8 = iStack8 + 0x1) {
        GetDlgItem16(param_5,*(INT16 *)(iStack8 * 0xc + 0x5d00));
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
        *(undefined2 *)((int)&PTR_LOOP_1050_5d04 + iStack8 * 0xc) = 0x0;
        uVar2 = *(undefined4 *)(param_1 + 0x94);
        param_5 = (HWND16)s_tile2_bmp_1050_1538;
        SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)uVar2,(SEGPTR)((ulong)uVar2 >> 0x10));
      }
      uVar2 = *(undefined4 *)(param_1 + 0x94);
      goto LAB_1040_2ccc;
    }
    for (iStack8 = 0x1; iStack8 < 0x5; iStack8 = iStack8 + 0x1) {
      GetDlgItem16(param_5,*(INT16 *)(iStack8 * 0xc + 0x5d00));
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
      *(undefined2 *)((int)&PTR_LOOP_1050_5d04 + iStack8 * 0xc) = 0x0;
      uVar2 = *(undefined4 *)(param_1 + 0x94);
      param_5 = (HWND16)s_tile2_bmp_1050_1538;
      SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)uVar2,(SEGPTR)((ulong)uVar2 >> 0x10));
    }
  }
  else {
    if (param_4._2_2_ == 0x159) {
      iStack4 = 0x1;
    }
    else {
      if (param_4._2_2_ == 0x15a) {
        iStack4 = 0x2;
      }
      else {
        if (param_4._2_2_ == 0x15b) {
          iStack4 = 0x3;
        }
        else {
          if (param_4._2_2_ != 0x15c) {
            pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,unaff_SS);
            return;
          }
          iStack4 = 0x4;
        }
      }
    }
    if (iStack4 == 0x0) {
      return;
    }
    uVar1 = (uint)(*(int *)((int)&PTR_LOOP_1050_5d04 + iStack4 * 0xc) == 0x0);
    *(uint *)((int)&PTR_LOOP_1050_5d04 + iStack4 * 0xc) = uVar1;
    if (uVar1 == 0x0) {
      uVar2 = *(undefined4 *)(param_1 + 0x94);
      goto LAB_1040_2ccc;
    }
  }
  uVar2 = *(undefined4 *)(param_1 + 0x98);
LAB_1040_2ccc:
  SetDlgItemText16(param_5,(INT16)uVar2,(SEGPTR)((ulong)uVar2 >> 0x10));
  return;
}



void __stdcall16far win_dlg_item_1040_2d48(ulong param_1,HWND16 param_2,BOOL16 param_3)

{
  UINT16 UVar1;
  UINT16 value;
  BOOL16 local_4;
  
  pass1_1040_b45e(param_1,param_2);
  UVar1 = GetDlgItemInt16(param_2,0x1,&local_4,param_3);
  value = GetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,&local_4,param_3);
  if (UVar1 != 0x0) {
    value = value / UVar1;
  }
  SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,value,0x165);
  return;
}



void __stdcall16far pass1_1040_2dac(ulong param_1)

{
  undefined4 uVar1;
  ulong uVar2;
  int iStack10;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x90);
  uVar2 = struct_op_1030_73a8(*(ulong *)((int)uVar1 + 0x6));
  for (iStack10 = 0x0; iStack10 < 0x5; iStack10 = iStack10 + 0x1) {
    pass1_1028_4ab2(uVar2,*(ushort *)((int)&PTR_LOOP_1050_5d04 + iStack10 * 0xc),*(int *)(iStack10 * 0xc + 0x5d02));
  }
  return;
}



astruct_18 * __stdcall16far pass1_1040_2e00(astruct_18 *param_1,byte param_2)

{
  pass1_1040_2a22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_2ea2(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_720 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x180,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_720 *)param_1;
  iVar1->field_0x8e = 0x0;
  iVar1->field_0x90 = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  *(undefined4 *)&iVar1->field_0x96 = 0x0;
  *(undefined2 *)param_1 = 0x3436;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3c,param_8,param_6,param_7);
  iVar1->field_0x96 = (int)puVar2;
  iVar1->field_0x98 = (int)((ulong)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_2f06(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x3436;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_2f32(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  ushort *puVar1;
  int iVar2;
  
  iVar2 = 0x0;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,(uchar *)param_3,param_4);
  pass1_1010_038e((ulong)puVar1,iVar2,param_5);
  destroy_win_1040_7b98(CONCAT22(param_2,param_1),0x1010);
  return;
}



void __stdcall16far show_win_1040_2f5a(astruct_1 *param_1,HWND16 param_2)

{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_dlg_op_1040_2f90(ulong param_1,WORD *param_2)

{
  HWND16 HVar1;
  uchar *in_DX;
  uchar *puVar2;
  uint uVar3;
  UINT16 msg;
  int iVar4;
  int unaff_DI;
  undefined2 uVar5;
  ushort *puVar6;
  ulong uVar7;
  char *pcVar8;
  ulong *local_116;
  ulong *local_112;
  CHAR local_10e [0x82];
  undefined local_8c [0x82];
  undefined4 uStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_2,in_DX,unaff_DI);
  puVar2 = (uchar *)((ulong)puStack6 >> 0x10);
  uStack10 = *(undefined4 *)((int)puStack6 + 0x68);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  GetWindowText16(0x1010,0x80,(INT16)local_8c);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_10e,param_2);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_10e);
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x182);
  *(HWND16 *)(iVar4 + 0x92) = HVar1;
  pass1_1018_3a94(*(ulong *)(iVar4 + 0x96),(ulong *)CONCAT22(param_2,&local_116),(ulong *)CONCAT22(param_2,&local_112),
                  param_2);
  send_msg_1040_3374(param_1,local_112,*(uint *)(iVar4 + 0x92),0x1018);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,(ushort)param_2,puVar2,unaff_DI);
  uVar3 = (uint)((ulong)puVar6 >> 0x10);
  uVar7 = *(ulong *)((int)puVar6 + 0x24);
  uVar7 = pass1_1018_3a7a(*(ulong *)(iVar4 + 0x96),uVar7,(uint)uVar7,uVar3);
  SendMessage16(0x1018,(UINT16)uVar7,(WPARAM16)(uVar7 >> 0x10),0x40dffff);
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x183);
  *(HWND16 *)(iVar4 + 0x94) = HVar1;
  send_msg_1040_3374(param_1,local_116,HVar1,(int)s_tile2_bmp_1050_1538);
  pcVar8 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  msg = (UINT16)((ulong)pcVar8 >> 0x10);
  SendDlgItemMessage16(0x1010,(INT16)pcVar8,msg,0x0,0x1830403);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,(INT16)pcVar8,msg,0xffff,0x183040d);
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x181);
  *(HWND16 *)(iVar4 + 0x8e) = HVar1;
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x184);
  *(HWND16 *)(iVar4 + 0x90) = HVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1040_311a(int param_1,ushort param_2,ushort param_3,ulong param_4)

{
  int iVar1;
  ulong uVar2;
  undefined *puVar3;
  undefined2 unaff_CS;
  ushort unaff_SS;
  LRESULT LVar4;
  ushort *puVar5;
  int iVar6;
  
  send_msg_1040_323c(CONCAT22(param_2,param_1),unaff_CS);
  load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  if (param_4._2_2_ == 0x181) {
    iVar1 = param_1 + 0x9a;
    puVar3 = (undefined *)param_2;
    iVar6 = iVar1;
    pass1_1018_3cda(*(ulong **)(param_1 + 0x96),(char *)CONCAT22(param_2,param_1 + 0x19a),
                    (char *)CONCAT22(param_2,iVar1));
    pass1_1018_3424(*(ulong *)(param_1 + 0x96),iVar6,(uint)puVar3,unaff_SS);
    if (iVar6 == 0x0) {
      iVar6 = 0x21;
    }
    else {
      pass1_1018_3a42(*(ulong *)(param_1 + 0x96),CONCAT22(param_2,iVar1),puVar3,unaff_SS);
      pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),CONCAT22(puVar3,iVar6));
      uVar2 = *(ulong *)(iVar6 + 0x10);
      pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),uVar2);
      PTR_LOOP_1050_5f0c = (undefined *)uVar2;
      PTR_LOOP_1050_5f10 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
      iVar6 = 0x25;
      PTR_LOOP_1050_5f0e = puVar3;
    }
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(param_1 + 0x8),iVar6,(ushort)puVar3,param_1,
                    (ushort)&PTR_LOOP_1050_1038,unaff_SS);
    LVar4 = SendMessage16((HWND16)&PTR_LOOP_1050_1038,0x0,0x0,0x1110002);
    iVar6 = 0x1;
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,unaff_SS,(uchar *)((ulong)LVar4 >> 0x10),param_2);
    pass1_1010_038e((ulong)puVar5,iVar6,unaff_SS);
  }
  else {
    if ((param_4._2_2_ == 0x181) || (0x1 < param_4._2_2_ - 0x182U)) {
      post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4._2_2_,0x1010);
      return;
    }
    set_win_pos_1040_331a(CONCAT22(param_2,param_1),param_3,(ushort)param_4,0x1010);
  }
  return;
}



LRESULT __stdcall16far send_msg_1040_323c(ulong param_1,HWND16 param_2)

{
  WPARAM16 wparam;
  LRESULT LVar1;
  LRESULT LVar2;
  
  wparam = (WPARAM16)(param_1 >> 0x10);
  LVar1 = SendMessage16(param_2,0x0,0x0,0x4070000);
  LVar2 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,(int)param_1 + 0x9a,wparam,CONCAT22(0x408,(int)LVar1));
  LVar1 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,(int)param_1 + 0x19a,wparam,CONCAT22(0x408,(int)LVar2));
  return LVar1;
}



void __stdcall16far enable_win_1040_32a8(ulong param_1)

{
  SEGPTR lp_string;
  BOOL16 BVar1;
  ushort unaff_SS;
  ulong uStack12;
  
  lp_string = (int)param_1 + 0x19a;
  uStack12 = param_1 & 0xffff0000 | (ulong)lp_string;
  pass1_1018_3a5c(*(ulong *)((int)param_1 + 0x96),param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x9aU),
                  param_1 & 0xffff0000 | (ulong)lp_string,unaff_SS);
  SetWindowText16(0x1018,lp_string);
  BVar1 = string_1018_39d8(unaff_SS,*(ulong *)((int)param_1 + 0x96),param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x9aU)
                           ,uStack12);
  EnableWindow16(0x1018,BVar1 & 0x1);
  return;
}



BOOL16 __stdcall16far set_win_pos_1040_331a(ulong param_1,ushort param_2,int param_3,HWND16 param_4)

{
  RECT16 local_e;
  int iStack10;
  ushort uStack6;
  int iStack4;
  
  iStack4 = param_3;
  uStack6 = param_2;
  if (param_3 == 0x1) {
    enable_win_1040_32a8(param_1);
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



void __stdcall16far send_msg_1040_3374(ulong param_1,ulong *param_2,uint param_3,HWND16 param_4)

{
  code **ppcVar1;
  undefined2 uVar2;
  ulong uVar3;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  uint uVar4;
  undefined2 uVar5;
  LRESULT LVar6;
  astruct_18 *paVar7;
  uint uVar8;
  ulong uStack10;
  ulong uStack6;
  
  uVar5 = SUB42(s_tile2_bmp_1050_1538,0x0);
  uVar8 = param_3;
  LVar6 = SendMessage16(param_4,0x0,0x0,0x40b0000);
  uVar2 = (undefined2)LVar6;
  uVar4 = (uint)param_2;
  ppcVar1 = (code **)((int)*param_2 + 0x10);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,param_2,uVar8);
  uStack6 = CONCAT22(extraout_DX,uVar2);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (code **)((int)*param_2 + 0x4);
    uVar3 = uStack6;
    (**ppcVar1)(uVar5,param_2,(char)uStack10,(int)(uStack10 >> 0x10),uVar4);
    paVar7 = (astruct_18 *)
             pass1_1018_3a7a(*(ulong *)((int)param_1 + 0x96),uVar3 & 0xffff | (ulong)extraout_DX_00 << 0x10,(uint)uVar3,
                             extraout_DX_00);
    uVar4 = param_3;
    LVar6 = SendMessage16(0x1018,(UINT16)paVar7,(WPARAM16)((ulong)paVar7 >> 0x10),0x4030000);
    uVar5 = 0x1000;
    fn_ptr_1000_17ce(paVar7,0x1000);
    if (LVar6 == -0x1) break;
    if (LVar6 == -0x2) {
      return;
    }
    uStack10 = uStack10 + 0x1;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1040_3410(astruct_18 *param_1,byte param_2)

{
  pass1_1040_2f06(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_34a2(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_721 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x192,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_721 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  iVar1->field_0x96 = 0x0;
  iVar1->field_0x98 = 0x0;
  *(int *)param_1 = (int)s_Null_Ptr_1050_38f3 + 0x7;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3c,param_8,param_6,param_7);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_3506(astruct_18 *param_1)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = (int)s_Null_Ptr_1050_38f3 + 0x7;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_3532(ushort param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  ushort *puVar1;
  int iVar2;
  
  iVar2 = 0x0;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  pass1_1010_038e((ulong)puVar1,iVar2,param_5);
  destroy_win_1040_7b98(CONCAT22(param_2,param_1),0x1010);
  return;
}



void __stdcall16far show_win_1040_355a(astruct_1 *param_1,HWND16 param_2)

{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far set_win_text_1040_3590(ulong param_1,WORD *param_2)

{
  HWND16 HVar1;
  SEGPTR lp_string;
  uint lp_string_00;
  uchar *in_DX;
  uint uVar2;
  int iVar3;
  int unaff_DI;
  undefined2 uVar4;
  undefined local_59a [0x4];
  undefined local_596 [0x4];
  BOOL16 BStack1426;
  uint uStack1424;
  CHAR local_58e [0x82];
  CHAR local_50c [0x100];
  undefined4 uStack1036;
  ushort *puStack1032;
  char local_404 [0x402];
  
  puStack1032 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_2,in_DX,unaff_DI);
  uVar2 = (uint)((ulong)puStack1032 >> 0x10);
  uStack1036 = *(undefined4 *)((int)puStack1032 + 0x68);
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  GetWindowText16(0x1010,0x80,(INT16)local_50c);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_58e,param_2);
  BStack1426 = SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_58e);
  sprintf_op_1018_34b6(*(ulong *)(iVar3 + 0x8e),(uchar)BStack1426);
  uStack1424 = uVar2;
  pass1_1018_3d44(*(ulong *)(iVar3 + 0x8e),(ulong *)CONCAT22(param_2,local_59a),(ulong *)CONCAT22(param_2,local_596));
  HVar1 = GetDlgItem16(0x1018,0x193);
  *(HWND16 *)(iVar3 + 0x98) = HVar1;
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,
             (short)param_2);
  wsprintf16((LPSTR)0x1010,local_50c,param_2);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x195);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_50c);
  lp_string = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x196);
  sprintf_op_1018_34b6(*(ulong *)(iVar3 + 0x8e),(uchar)lp_string);
  SetWindowText16(0x1018,lp_string);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x197);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,
             (short)param_2);
  SetWindowText16(0x1010,(SEGPTR)local_404);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,
             (short)param_2);
  wsprintf16((LPSTR)0x1010,local_50c,param_2);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x198);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_50c);
  lp_string_00 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x199);
  unk_str_op_1018_35b0(*(ulong *)(iVar3 + 0x8e),(ushort)param_2,lp_string_00);
  if ((uVar2 | lp_string_00) == 0x0) {
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,
               (short)param_2);
    SetWindowText16(0x1010,(SEGPTR)local_404);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x19a);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,
               (short)param_2);
    SetWindowText16(0x1010,(SEGPTR)local_404);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
    return;
  }
  SetWindowText16(0x1018,lp_string_00);
  return;
}
