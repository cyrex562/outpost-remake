


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_0e86(astruct_18 *param_1,ushort param_2)

{
  uint uVar1;
  astruct_18 *paVar2;
  uchar *puVar3;
  int iVar4;
  int unaff_DI;
  undefined2 uVar5;
  undefined2 uVar6;
  ushort *puVar7;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  param_1->field_0x0 = (int)s_overflow_on_node__d_1050_11ca + 0x8;
  *(undefined2 *)(iVar4 + 0x2) = (int)&PTR_LOOP_1050_1040;
  paVar2 = *(astruct_18 **)(iVar4 + 0x92);
  uVar1 = *(uint *)(iVar4 + 0x94);
  puVar3 = (uchar *)(uVar1 | (uint)paVar2);
  if (puVar3 != (uchar *)0x0) {
    pass1_1040_a5d0((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10);
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  PTR_LOOP_1050_5b82 = (undefined *)*(undefined2 *)(iVar4 + 0x96);
  if (*(long *)(iVar4 + 0x92) == 0x0) {
    uVar6 = SUB42(&PTR_LOOP_1050_1038,0x0);
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)(iVar4 + 0x6));
  }
  else {
    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_2,puVar3,unaff_DI);
    uVar6 = 0x1010;
    pass1_1010_7b8c((ulong)puVar7,*(int *)(iVar4 + 0x6),param_2);
  }
  ui_cleanup_op_1040_782c(param_1,uVar6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far set_win_pos_1040_0f10(HWND16 param_1,ushort param_2,int param_3)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  int unaff_DI;
  undefined2 uVar4;
  ulong uVar5;
  ushort *puVar6;
  UINT16 check;
  
  dialog_ui_fn_1040_78e2(*(astruct_1 **)(param_3 + 0x6),param_1);
  uVar2 = *(undefined4 *)(param_3 + 0x6);
  uVar4 = (undefined2)((ulong)uVar2 >> 0x10);
  iVar3 = (int)uVar2;
  if (*(int *)(iVar3 + 0x98) == 0x0) {
    GetWindowRect16(param_1,(RECT16 *)(param_3 + -0x24));
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1830);
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)(param_3 + -0x2c));
    piVar1 = (int *)(param_3 + -0x20);
    *piVar1 = *piVar1 - *(int *)(param_3 + -0x24);
    iVar3 = (*(int *)(param_3 + -0x2a) - *(int *)(param_3 + -0x22)) + -0x2;
    *(int *)(param_3 + -0x1e) = iVar3;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x6,iVar3,*(INT16 *)(param_3 + -0x20),0x0,0x0,0x0);
    CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,0x1,0x1c1);
    uVar2 = *(undefined4 *)(param_3 + 0x6);
    uVar2 = *(undefined4 *)((int)uVar2 + 0x8e);
    *(undefined2 *)((int)uVar2 + 0xa) = 0x2;
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1830);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  }
  else {
    uVar2 = *(undefined4 *)(iVar3 + 0x92);
    uVar5 = struct_op_1030_73a8(*(ulong *)((int)uVar2 + 0x6));
    *(undefined2 *)(param_3 + -0x32) = (int)uVar5;
    *(undefined2 *)(param_3 + -0x30) = (int)(uVar5 >> 0x10);
    uVar2 = *(undefined4 *)(param_3 + -0x32);
    if (*(int *)((int)uVar2 + 0x20) == 0x2) {
      check = 0x1c1;
    }
    else {
      check = 0x1c2;
    }
    CheckDlgButton16(0x1030,0x1,check);
  }
  GetCursorPos16((POINT16 *)s_tile2_bmp_1050_1538);
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)(param_3 + -0xc));
  iVar3 = *(int *)(param_3 + -0x8) - *(int *)(param_3 + -0xc);
  *(int *)(param_3 + -0x12) = iVar3;
  *(int *)(param_3 + -0xe) = -(iVar3 / 0x2 - *(int *)(param_3 + -0x4));
  iVar3 = *(int *)(param_3 + -0x6) - *(int *)(param_3 + -0xa);
  *(int *)(param_3 + -0x14) = iVar3;
  *(int *)(param_3 + -0x10) = -(iVar3 / 0x2 - *(int *)(param_3 + -0x2));
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_2,(uchar *)(iVar3 >> 0xf),unaff_DI);
  uVar4 = (undefined2)((ulong)puVar6 >> 0x10);
  iVar3 = (int)puVar6;
  *(int *)(param_3 + -0x1c) = iVar3;
  *(undefined2 *)(param_3 + -0x1a) = uVar4;
  *(undefined2 *)(param_3 + -0x16) = *(undefined2 *)(iVar3 + 0xa);
  *(undefined2 *)(param_3 + -0x18) = *(undefined2 *)(iVar3 + 0xc);
  if (*(int *)(param_3 + -0x16) < *(int *)(param_3 + -0x12) + *(int *)(param_3 + -0xe)) {
    *(int *)(param_3 + -0xe) = *(int *)(param_3 + -0x16) - *(int *)(param_3 + -0x12);
  }
  if (*(int *)(param_3 + -0x18) < *(int *)(param_3 + -0x14) + *(int *)(param_3 + -0x10)) {
    *(int *)(param_3 + -0x10) = *(int *)(param_3 + -0x18) - *(int *)(param_3 + -0x14);
  }
  uVar2 = *(undefined4 *)(param_3 + -0x10);
  SetWindowPos16(0x1010,0x45,0x0,0x0,(INT16)uVar2,(INT16)((ulong)uVar2 >> 0x10),0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_109c(int param_1,ushort param_2,ushort param_3,ulong param_4,uchar *param_5,int param_6,ushort param_7,
               ushort param_8)

{
  undefined4 uVar1;
  bool bVar2;
  int iVar3;
  ushort *puVar4;
  
  bVar2 = false;
  if (param_4._2_2_ == 0x1c1) {
    *(undefined2 *)(param_1 + 0x96) = 0x2;
    bVar2 = true;
  }
  else {
    if (param_4._2_2_ == 0x1c2) {
      *(undefined2 *)(param_1 + 0x96) = 0x1;
      bVar2 = true;
    }
    else {
      if (param_4._2_2_ != 0x1830) {
        post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4._2_2_,param_7);
        return;
      }
      puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_8,param_5,param_6);
      iVar3 = (int)puVar4;
      uVar1 = *(undefined4 *)(param_1 + 0x92);
      ui_op_1010_79aa(puVar4,0xfb6,*(long *)((int)uVar1 + 0x6),param_8);
      if (iVar3 == 0x0) {
        uVar1 = *(undefined4 *)(param_1 + 0x92);
        unk_win_op_1010_7300((ulong)puVar4,0x0,0xc,*(ulong *)((int)uVar1 + 0x6));
      }
    }
  }
  if (bVar2) {
    uVar1 = *(undefined4 *)(param_1 + 0x8e);
    *(undefined2 *)((int)uVar1 + 0xa) = *(undefined2 *)(param_1 + 0x96);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_1152(int param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5,ushort param_6)

{
  ushort uVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  ushort *puVar5;
  
  if (*(long *)(param_1 + 0x92) != 0x0) {
    uVar2 = *(undefined4 *)(param_1 + 0x8e);
    uVar1 = *(ushort *)((int)uVar2 + 0xa);
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_3,param_4);
    uVar2 = *(undefined4 *)(param_1 + 0x92);
    uVar4 = (undefined2)((ulong)uVar2 >> 0x10);
    iVar3 = (int)uVar2;
    param_5 = 0x1010;
    pass1_1010_ae92((ulong)puVar5,uVar1,*(uint *)(iVar3 + 0xa),*(ulong *)(iVar3 + 0x6),param_4,param_6);
  }
  destroy_win_1040_7b98(CONCAT22(param_2,param_1),param_5);
  PTR_LOOP_1050_5b80 = (undefined *)0x0;
  return;
}



astruct_18 * __stdcall16far pass1_1040_11ac(astruct_18 *param_1,byte param_2,ushort param_3)

{
  pass1_1040_0e86(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * __stdcall16far
pass1_1040_123e(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_717 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfd1,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_717 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  *(undefined2 *)param_1 = 0x17b0;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x46,param_8,param_6,param_7);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_1290(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x17b0;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1040_12bc(astruct_1 *param_1,ushort param_2,uchar *param_3)

{
  undefined4 uVar1;
  WPARAM16 wparam;
  HWND16 HVar2;
  undefined2 uVar3;
  undefined in_AF;
  char *pcVar4;
  uchar local_54 [0x52];
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  uVar1 = *(undefined4 *)((int)param_1 + 0x8e);
  uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
  sys_1000_3f9c(local_54,param_3,0x5cd4,(ushort)&USHORT_1050_1050,*(ushort *)((int)uVar1 + 0xa),&stack0xfffe,uVar3,
                0x1000,param_3,in_AF);
  GetDlgItem16(0x1000,0xfd2);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_54,(WPARAM16)param_3,0xc0000);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0xffff,0x4010000);
  move_win_1040_826c(param_1,-0x1,0xffff);
  pcVar4 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  wparam = (WPARAM16)((ulong)pcVar4 >> 0x10);
  HVar2 = GetDlgItem16(0x1010,(int)s_vrpal_bmp_1050_183a + 0x5);
  send_msg_1040_1696((ulong)param_1,HVar2,(ushort)param_3,(int)s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)pcVar4,wparam,0x40dffff);
  HVar2 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_vrpal_bmp_1050_183a + 0x4);
  send_msg_1040_1696((ulong)param_1,HVar2,(ushort)param_3,(int)s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)pcVar4,wparam,0x40dffff);
  ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x5);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_msg_op_1040_13b2(ulong param_1,int param_2,HWND16 param_3,ushort param_4)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint uVar3;
  int iVar4;
  undefined *puVar5;
  uchar *puVar6;
  uchar *puVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined in_AF;
  LRESULT LVar11;
  undefined4 *puStack562;
  undefined local_22e [0x118];
  ulong uStack278;
  ulong uStack274;
  undefined *puStack270;
  uchar *puStack268;
  undefined4 uStack266;
  uint uStack262;
  char *pcStack260;
  undefined local_100 [0x52];
  int iStack174;
  HWND16 HStack172;
  uchar local_aa [0x52];
  uint uStack88;
  HWND16 HStack86;
  undefined local_54 [0x52];
  
  iVar4 = (int)param_1;
  uVar9 = (undefined2)(param_1 >> 0x10);
  if (param_2 != 0x0) {
    HStack86 = GetDlgItem16(param_3,0xfd2);
    SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_54,param_4,0xd0051);
    uStack88 = pass1_1000_3e2c(CONCAT22(param_4,local_54));
    HStack172 = GetDlgItem16(0x1000,(int)s_vrpal_bmp_1050_183a + 0x4);
    LVar11 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
    iStack174 = (int)LVar11;
    if (iStack174 != -0x1) {
      SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_aa,param_4,CONCAT22(0x408,iStack174));
    }
    HStack172 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_vrpal_bmp_1050_183a + 0x5);
    LVar11 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
    iStack174 = (int)LVar11;
    if (iStack174 != -0x1) {
      SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_100,param_4,CONCAT22(0x408,iStack174));
    }
    pcStack260 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    puVar6 = local_aa;
    uVar3 = pass1_1000_3d7a(CONCAT22(param_4,puVar6),CONCAT22(param_4,local_100));
    if (uVar3 != 0x0) {
      uVar3 = pass1_1000_3d7a(CONCAT22(param_4,local_aa),(ulong)pcStack260);
      if (uVar3 != 0x0) {
        uVar3 = pass1_1000_3d7a(CONCAT22(param_4,local_100),(ulong)pcStack260);
        if (uVar3 != 0x0) {
          pass1_1010_531c(*(ulong *)(iVar4 + 0x8e),CONCAT22(param_4,local_aa),(ushort)local_aa,puVar6,param_4);
          puVar5 = local_100;
          pass1_1010_52fc(*(ulong *)(iVar4 + 0x8e),CONCAT22(param_4,puVar5),(ushort)puVar5,puVar6,param_4);
          pass1_1010_5120(*(ulong *)(iVar4 + 0x8e),uStack88,(uint)puVar5,(uint)puVar6,param_4);
          uStack266 = (undefined4 *)CONCAT22(uStack266._2_2_,puVar5);
          if (puVar5 == (undefined *)0x0) {
            mem_op_1000_179c(0xb4,puVar6,0x1000);
            puVar7 = (uchar *)((uint)puVar6 | (uint)puVar5);
            puStack270 = puVar5;
            puStack268 = puVar6;
            if (puVar7 == (uchar *)0x0) {
              iVar4 = 0x0;
              puVar7 = (uchar *)0x0;
            }
            else {
              iVar4 = string_1040_8520((astruct_57 *)CONCAT22(puVar6,puVar5),(ushort)PTR_LOOP_1050_0396,0x30,0x2,0x57b,
                                       0x7d2,puVar7,param_4);
            }
            puStack562 = (undefined4 *)CONCAT22(puVar7,iVar4);
            ppcVar1 = (code **)((int)*puStack562 + 0x74);
            (**ppcVar1)(0x1000,iVar4,puVar7);
            return;
          }
          uVar2 = *(undefined4 *)(iVar4 + 0x8e);
          uStack274 = *(ulong *)((int)uVar2 + 0x12);
          uVar2 = *(undefined4 *)(iVar4 + 0x8e);
          uVar10 = (undefined2)((ulong)uVar2 >> 0x10);
          iVar8 = (int)uVar2;
          uStack278 = *(ulong *)(iVar8 + 0x16);
          uVar2 = *(undefined4 *)(iVar4 + 0x8e);
          uStack262 = *(uint *)((int)uVar2 + 0xa);
          pass1_1028_8d9e((astruct_100 *)CONCAT22(param_4,local_22e),(ulong)uStack262,uStack274,
                          uStack278 & 0xffff | (ulong)*(uint *)(iVar8 + 0x18) << 0x10,param_4,in_AF);
          fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,local_22e));
          param_3 = (HWND16)&USHORT_1050_1028;
          pass1_1028_8dec((ushort *)CONCAT22(param_4,local_22e));
          goto LAB_1040_1619;
        }
      }
    }
    param_3 = 0x1000;
    mem_op_1000_179c(0xb4,puVar6,0x1000);
    puVar7 = (uchar *)((uint)puVar6 | uVar3);
    puStack270 = (undefined *)uVar3;
    puStack268 = puVar6;
    if (puVar7 == (uchar *)0x0) {
      iVar4 = 0x0;
      puVar7 = (uchar *)0x0;
    }
    else {
      iVar4 = string_1040_8520((astruct_57 *)CONCAT22(puVar6,uVar3),(ushort)PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x755,
                               puVar7,param_4);
    }
    uStack266 = (undefined4 *)CONCAT22(puVar7,iVar4);
    ppcVar1 = (code **)((int)*uStack266 + 0x74);
    (**ppcVar1)(0x1000,iVar4,puVar7);
  }
LAB_1040_1619:
  DestroyWindow16(param_3);
  return;
}



ulong __stdcall16far set_win_pos_1040_162a(ushort param_1,ulong param_2,ulong param_3,ushort param_4,HWND16 param_5)

{
  uint uVar1;
  BOOL16 BVar2;
  RECT16 local_a;
  int iStack6;
  
  if ((param_3._2_2_ != (int)s_vrpal_bmp_1050_183a + 0x5) && (param_3._2_2_ != (int)s_vrpal_bmp_1050_183a + 0x4)) {
    BVar2 = post_win_msg_1040_7b3c
                      ((ulong *)CONCAT22((int)param_2,param_1),param_2._2_2_,(ushort)param_3,param_3._2_2_,param_5);
    return CONCAT22(param_4,BVar2);
  }
  if ((ushort)param_3 == 0x7) {
    GetWindowRect16(param_5,&local_a);
    iStack6 = iStack6 - local_a.x;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x2,0x50,iStack6,0x0,0x0,0x0);
  }
  else {
    if (((ushort)param_3 != 0x9) && ((ushort)param_3 != 0xa)) {
      uVar1 = 0x0;
      goto LAB_1040_164d;
    }
  }
  uVar1 = 0x1;
LAB_1040_164d:
  return (ulong)uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far send_msg_1040_1696(ulong param_1,ushort param_2,ushort param_3,HWND16 param_4)

{
  ulong uVar1;
  undefined4 uVar2;
  uint *puVar3;
  uchar *puVar4;
  uchar *puVar5;
  undefined2 uVar6;
  LRESULT LVar7;
  astruct_18 *paVar8;
  char *pcVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  uint uStack18;
  uint local_4;
  
  SendMessage16(param_4,0x0,0x0,0x40b0000);
  LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0000);
  puVar4 = (uchar *)((ulong)LVar7 >> 0x10);
  local_4 = 0x0;
  puVar3 = &local_4;
  uVar6 = (undefined2)(param_1 >> 0x10);
  pass1_1010_519a(*(ulong *)((int)param_1 + 0x8e),(int *)CONCAT22(param_3,puVar3),puVar4,param_3);
  puVar5 = puVar4;
  for (uStack18 = 0x0; uStack18 < local_4; uStack18 = uStack18 + 0x1) {
    uVar1 = *(ulong *)(puVar3 + uStack18 * 0x2);
    uVar10 = 0x0;
    uVar11 = 0x403;
    uVar2 = *(undefined4 *)((int)param_1 + 0x8e);
    paVar8 = (astruct_18 *)
             string_1010_5286((ushort)uVar2,(ushort)((ulong)uVar2 >> 0x10),uVar1,(char *)uVar1,(uint)puVar5);
    LVar7 = SendMessage16(0x1010,(UINT16)paVar8,(WPARAM16)((ulong)paVar8 >> 0x10),CONCAT22(uVar11,uVar10));
    puVar5 = (uchar *)((ulong)LVar7 >> 0x10);
    fn_ptr_1000_17ce(paVar8,0x1000);
  }
  uVar6 = 0x0;
  uVar10 = 0x40a;
  pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  SendMessage16(0x1010,(UINT16)pcVar9,(WPARAM16)((ulong)pcVar9 >> 0x10),CONCAT22(uVar10,uVar6));
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0001);
  return;
}



astruct_18 * __stdcall16far pass1_1040_178a(astruct_18 *param_1,byte param_2)

{
  pass1_1040_1290(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_181c(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               ushort param_7)

{
  astruct_433 *iVar1;
  int unaff_DI;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfbb,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_433 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  *(undefined2 *)param_1 = 0x1c48;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_7,param_6,unaff_DI);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_1876(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x1c48;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far show_win_1040_18a2(astruct_1 *param_1,HWND16 param_2,WORD *param_3)

{
  undefined4 uVar1;
  CHAR local_304 [0x100];
  char local_204 [0x100];
  char local_104 [0x100];
  undefined2 uStack4;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  check_dialog_btn_1040_1afe(param_1);
  if (PTR_LOOP_1050_13ae != (undefined *)0x0) {
    if (PTR_LOOP_1050_13ae == (undefined *)&PTR_LOOP_1050_0002) {
      uStack4 = 0x621;
    }
    else {
      if (PTR_LOOP_1050_13ae == (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1)) {
        uStack4 = 0x622;
      }
      else {
        if (PTR_LOOP_1050_13ae == (undefined *)&DAT_1050_0004) {
          uStack4 = 0x623;
        }
        else {
          uStack4 = 0x620;
        }
      }
    }
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,
               (short)param_3);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_204,
               (short)param_3);
    wsprintf16((LPSTR)0x1010,local_304,param_3);
    SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)local_304,(SEGPTR)param_3);
    uVar1 = *(undefined4 *)((int)param_1 + 0x8e);
    if (*(int *)((int)uVar1 + 0x82) == 0x0) {
      uStack4 = 0x627;
    }
    else {
      uStack4 = 0x626;
    }
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,
               (short)param_3);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_204,
               (short)param_3);
    wsprintf16((LPSTR)0x1010,local_304,param_3);
    param_2 = (HWND16)s_tile2_bmp_1050_1538;
    SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,(INT16)local_304,(SEGPTR)param_3);
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  return;
}



void __stdcall16far unk_win_ui_op_1040_19ea(astruct_32 *param_1,int param_2,HWND16 param_3)

{
  ulong uVar1;
  UINT16 UVar2;
  uchar *in_DX;
  astruct_32 *iVar4;
  int unaff_DI;
  astruct_32 *uVar3;
  ushort unaff_SS;
  
  iVar4 = (astruct_32 *)param_1;
  uVar3 = (astruct_32 *)((ulong)param_1 >> 0x10);
  if (param_2 != 0x0) {
    UVar2 = IsDlgButtonChecked(param_3,0xfdb);
    pass1_1010_5d9c(iVar4->field_0x8e,UVar2,in_DX,unaff_DI,unaff_SS);
    UVar2 = IsDlgButtonChecked(0x1010,0xfdc);
    uVar1 = iVar4->field_0x8e;
    *(UINT16 *)((int)uVar1 + 0x20) = UVar2;
    UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0xfdd);
    uVar1 = iVar4->field_0x8e;
    *(UINT16 *)((int)uVar1 + 0x74) = UVar2;
    param_3 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0xfde);
    uVar1 = iVar4->field_0x8e;
    *(UINT16 *)((int)uVar1 + 0x72) = UVar2;
    if (iVar4->field_0x92 != 0x0) {
      uVar1 = iVar4->field_0x8e;
      param_3 = 0x1000;
      pass1_1000_4906((astruct_20 *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x22)),(WNDCLASS16 *)0x0,0x40);
    }
    if (iVar4->field_0x94 != 0x0) {
      param_3 = 0x1010;
      pass1_1010_60a0(iVar4->field_0x8e);
    }
  }
  DestroyWindow16(param_3);
  return;
}



ulong __stdcall16far
pass1_1040_1ab0(int param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5,ushort param_6)

{
  BOOL16 BStack6;
  ushort uStack4;
  
  BStack6 = 0x0;
  uStack4 = 0x0;
  if (param_4._2_2_ == 0x1831) {
    *(undefined2 *)(param_1 + 0x92) = 0x1;
    *(undefined2 *)(param_1 + 0x94) = 0x1;
    check_dialog_btn_1040_1b8a(param_1,param_2);
  }
  else {
    BStack6 = post_win_msg_1040_7b3c
                        ((ulong *)CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,param_1)),param_3,
                         (ushort)param_4,param_4._2_2_,param_6);
    uStack4 = param_5;
  }
  return CONCAT22(uStack4,BStack6);
}



void __stdcall16far check_dialog_btn_1040_1afe(undefined4 param_1)

{
  INT16 id;
  INT16 id_00;
  INT16 id_01;
  undefined4 uVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  HWND16 unaff_CS;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar3 + 0x8e);
  uVar2 = *(undefined4 *)(iVar3 + 0x8e);
  id = *(INT16 *)((int)uVar2 + 0x20);
  uVar2 = *(undefined4 *)(iVar3 + 0x8e);
  id_00 = *(INT16 *)((int)uVar2 + 0x74);
  uVar2 = *(undefined4 *)(iVar3 + 0x8e);
  id_01 = *(INT16 *)((int)uVar2 + 0x72);
  CheckDlgButton16(unaff_CS,*(INT16 *)((int)uVar1 + 0x1e),0xfdb);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_00,0xfdd);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_01,0xfde);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id,0xfdc);
  return;
}



void __stdcall16far check_dialog_btn_1040_1b8a(void)

{
  ushort id;
  ushort id_00;
  ushort id_01;
  ushort id_02;
  
  id = pass1_1010_60b4();
  id_00 = pass1_1010_60c6();
  id_01 = pass1_1010_60c0();
  id_02 = pass1_1010_60ba();
  CheckDlgButton16(0x1010,id,0xfdb);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_01,0xfdd);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_02,0xfde);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_00,0xfdc);
  return;
}



astruct_18 * __stdcall16far pass1_1040_1c22(astruct_18 *param_1,byte param_2)

{
  pass1_1040_1876(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_1cb4(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  uchar *puVar1;
  astruct_718 *iVar2;
  undefined2 uVar2;
  ushort *puVar3;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xe8,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_718 *)param_1;
  *(undefined4 *)&iVar2->field_0x8e = 0x0;
  *(undefined4 *)&iVar2->field_0x92 = 0x0;
  *(undefined2 *)param_1 = 0x1eee;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_8,param_6,param_7);
  puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
  iVar2->field_0x8e = (int)puVar3;
  iVar2->field_0x90 = puVar1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_8,puVar1,param_7);
  iVar2->field_0x92 = (int)puVar3;
  iVar2->field_0x94 = (int)((ulong)puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_1d24(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x1eee;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



void __stdcall16far show_win_1040_1d50(astruct_1 *param_1,HWND16 param_2)

{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  return;
}



void __stdcall16far unk_win_ui_op_1040_1d7a(astruct_33 *param_1,int param_2,HWND16 param_3)

{
  undefined4 uVar1;
  UINT16 UVar2;
  astruct_33 *iVar3;
  astruct_33 *uVar3;
  HWND16 HVar3;
  HWND16 HVar4;
  ushort unaff_SS;
  
  iVar3 = (astruct_33 *)param_1;
  uVar3 = (astruct_33 *)((ulong)param_1 >> 0x10);
  if ((param_2 != 0x0) && (uVar1 = iVar3->field_0x8e, *(int *)((int)uVar1 + 0x72) != 0x0)) {
    HVar3 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(param_3,0xe1);
    if (UVar2 != 0x0) {
      HVar3 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d5,unaff_SS);
    }
    HVar4 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar3,0xe2);
    if (UVar2 != 0x0) {
      HVar4 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d6,unaff_SS);
    }
    HVar3 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar4,0xe3);
    if (UVar2 != 0x0) {
      HVar3 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d7,unaff_SS);
    }
    HVar4 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar3,0xe5);
    if (UVar2 != 0x0) {
      HVar4 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d8,unaff_SS);
    }
    HVar3 = (HWND16)s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar4,0xe6);
    if (UVar2 != 0x0) {
      HVar3 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1e2,unaff_SS);
    }
    UVar2 = IsDlgButtonChecked(HVar3,0xe7);
    if (UVar2 != 0x0) {
      pass1_1008_a930(iVar3->field_0x92,0x1dc,unaff_SS);
    }
    return;
  }
  DestroyWindow16(param_3);
  return;
}



ulong __stdcall16far
pass1_1040_1e80(int param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5,ushort param_6)

{
  BOOL16 BStack6;
  ushort uStack4;
  
  BStack6 = 0x0;
  uStack4 = 0x0;
  if (param_4._2_2_ == 0xe4) {
    pass1_1008_a9ec(*(ulong *)(param_1 + 0x92));
  }
  else {
    BStack6 = post_win_msg_1040_7b3c((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4._2_2_,param_6);
    uStack4 = param_5;
  }
  return CONCAT22(uStack4,BStack6);
}



astruct_18 * __stdcall16far pass1_1040_1ec8(astruct_18 *param_1,byte param_2)

{
  pass1_1040_1d24(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_1f5a(astruct_57 *param_1,ushort param_2,int param_3,ushort param_4)

{
  int *piVar1;
  uchar *puVar2;
  astruct_719 *iVar3;
  astruct_43 *paVar3;
  ulong uVar4;
  ushort *puVar5;
  int iVar6;
  undefined2 uVar7;
  int iVar8;
  int iVar9;
  undefined2 uVar10;
  undefined4 local_16;
  undefined4 uStack18;
  
  iVar6 = (int)param_1;
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfcf,param_2);
  *(undefined4 *)(iVar6 + 0x8e) = 0x0;
  *(undefined4 *)(iVar6 + 0xa2) = 0x0;
  *(undefined4 *)(iVar6 + 0xa6) = 0x0;
  *(undefined2 *)param_1 = 0x237e;
  *(undefined2 *)(iVar6 + 0x2) = (int)&PTR_LOOP_1050_1040;
  paVar3 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1cc,param_4);
  *(undefined2 *)(iVar6 + 0x8e) = (int)paVar3;
  *(undefined2 *)(iVar6 + 0x90) = (int)((ulong)paVar3 >> 0x10);
  uVar4 = pass1_1008_4772((astruct_76 *)((ulong)paVar3 & 0xffff0000 | (ulong)*(uint *)(iVar6 + 0x8e)));
  puVar2 = (uchar *)(uVar4 >> 0x10);
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,puVar2,param_3);
  local_16 = CONCAT22(*(int *)((int)uVar4 + 0x8) + 0xa,0xa);
  uStack18 = CONCAT22(0x1d6,*(int *)((int)uVar4 + 0x4) + -0xa);
  *(undefined4 *)(iVar6 + 0x92) = local_16;
  *(undefined4 *)(iVar6 + 0x96) = uStack18;
  *(undefined4 *)(iVar6 + 0x9a) = local_16;
  *(undefined4 *)(iVar6 + 0x9e) = uStack18;
  piVar1 = (int *)(iVar6 + 0x9c);
  *piVar1 = *piVar1 + 0x14;
  iVar9 = iVar6 + 0xa2;
  iVar8 = iVar6 + 0xa6;
  uVar10 = uVar7;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,(uchar *)((ulong)puVar5 >> 0x10),iVar6 + 0xa2);
  pass1_1010_0538((ulong)puVar5,(char **)CONCAT22(uVar7,iVar8),(char **)CONCAT22(uVar10,iVar9),0x1010,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_205e(astruct_18 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_624 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_624 *)param_1;
  param_1->field_0x0 = 0x237e;
  iVar4->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar1 = iVar4->field_0x8e;
  uVar2 = iVar4->field_0x90;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(iVar4->field_0xa2,0x1000);
  fn_ptr_1000_17ce(iVar4->field_0xa6,0x1000);
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,iVar4->field_0x6);
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1038);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far create_win_1040_20d4(ushort param_1,ushort param_2,ushort param_3,astruct_1 *param_4)

{
  int y;
  int unaff_DI;
  undefined2 uVar1;
  ushort *puVar2;
  RECT16 local_1e;
  int iStack26;
  int iStack24;
  undefined4 uStack22;
  ulong uStack18;
  int iStack14;
  undefined2 uStack12;
  int iStack10;
  int iStack8;
  undefined2 uStack6;
  int iStack4;
  
  dialog_ui_fn_1040_78e2(param_4,param_2);
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_1,(uchar *)param_3,unaff_DI);
  uStack12 = (undefined2)((ulong)puVar2 >> 0x10);
  iStack14 = (int)puVar2;
  iStack8 = *(int *)(iStack14 + 0xa);
  iStack10 = *(int *)(iStack14 + 0xc);
  uVar1 = (undefined2)((ulong)param_4 >> 0x10);
  uStack18 = pass1_1008_4772(*(astruct_76 **)((int)param_4 + 0x8e));
  y = *(int *)((int)uStack18 + 0x4);
  iStack4 = (iStack8 - y) / 0x2;
  uStack6 = 0x5;
  SetWindowPos16(0x1008,0x6,0x1d6,y,0x5,iStack4,0x0);
  GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_1e);
  load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  uStack22 = 0x50010001;
  CreateWindow16((LPCSTR)0x1010,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,0x1,*(INT16 *)((int)param_4 + 0x6),0x19,
                 0x58,iStack24 - 0x28,(iStack26 + -0x58) / 0x2,0x1,(LPVOID)((int)s_Rebel_1050_4ffc + 0x5));
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x45,iStack10 + -0xa,*(INT16 *)((int)uStack18 + 0x4),0x5,iStack4,0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far mix_draw_op_1040_21d6(ulong param_1,HWND16 param_2,ushort param_3)

{
  undefined uVar1;
  undefined uVar2;
  astruct_13 *paVar3;
  code **ppcVar4;
  int iVar5;
  HPALETTE16 b_force_background;
  COLORREF color;
  COLORREF color_00;
  HANDLE16 handle;
  ushort in_DX;
  int iVar6;
  RECT16 *rect;
  ulong uVar7;
  undefined2 uVar8;
  HGDIOBJ16 HStack62;
  HDC16 local_24;
  PAINTSTRUCT16 local_22;
  
  rect = (RECT16 *)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar8 = *(undefined2 *)(iVar6 + 0x6);
  local_24 = BeginPaint16(param_2,&local_22);
  paVar3 = *(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe);
  b_force_background = palette_op_1008_4e08(paVar3,&local_24,in_DX,0x1008);
  ppcVar4 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x8e) + 0x4);
  (**ppcVar4)(0x1008,*(undefined4 *)(iVar6 + 0x8e),0xffffffff,&local_24,param_3,uVar8);
  uVar7 = pass1_1008_4d72((ulong)paVar3);
  uVar8 = (undefined2)(uVar7 >> 0x10);
  iVar5 = (int)uVar7;
  uVar1 = *(undefined *)(iVar5 + 0x3e5);
  uVar2 = *(undefined *)(iVar5 + 0x3e6);
  color = SetBkColor16(0x1008,0x0);
  color_00 = SetTextColor16((HDC16)s_tile2_bmp_1050_1538,CONCAT11(uVar1,uVar2));
  HStack62 = 0x0;
  handle = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5ced);
  if (handle != 0x0) {
    HStack62 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
  }
  DrawText16((HDC16)s_tile2_bmp_1050_1538,(LPCSTR)&PTR_LOOP_1050_0010,iVar6 + 0x92,rect,0xffff);
  SetTextColor16((HDC16)s_tile2_bmp_1050_1538,CONCAT11(*(undefined *)(iVar5 + 0x95),*(undefined *)(iVar5 + 0x96)));
  DrawText16((HDC16)s_tile2_bmp_1050_1538,(LPCSTR)&PTR_LOOP_1050_0010,iVar6 + 0x9a,rect,0xffff);
  if (handle != 0x0) {
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack62);
  }
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,color);
  SetTextColor16((HDC16)s_tile2_bmp_1050_1538,color_00);
  SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,b_force_background);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}
