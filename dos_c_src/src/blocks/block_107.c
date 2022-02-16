


LRESULT __stdcall16far send_msg_1040_4cb2(ulong param_1,HWND16 param_2)

{
  uchar uVar1;
  HWND16 HVar1;
  undefined2 in_DX;
  ulong uVar2;
  LRESULT LVar2;
  ushort uVar3;
  ushort uVar4;
  
  pass1_1040_b45e(param_1,param_2);
  HVar1 = GetDlgItem16(param_2,0x1770);
  uVar3 = 0xffff;
  uVar4 = 0x40d;
  pass1_1040_4d7e(param_1);
  uVar2 = (ulong)pass1_1040_4dcc(param_1,HVar1,in_DX);
  LVar2 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)uVar2,(WPARAM16)(uVar2 >> 0x10),CONCAT22(uVar4,uVar3));
  return LVar2;
}



void __stdcall16far pass1_1040_4cf4(ulong param_1,HWND16 param_2,ushort param_3)

{
  undefined4 uVar1;
  undefined4 uVar2;
  ushort uVar3;
  int iVar4;
  int iVar5;
  int unaff_DI;
  undefined2 uVar6;
  undefined2 uVar7;
  LRESULT LVar8;
  undefined local_52 [0x50];
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  GetDlgItem16(param_2,0x1770);
  LVar8 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
  uVar3 = (ushort)((ulong)LVar8 >> 0x10);
  if ((int)LVar8 != -0x1) {
    LVar8 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_52,param_3,CONCAT22(0x408,(int)LVar8));
    uVar3 = (ushort)((ulong)LVar8 >> 0x10);
  }
  uVar2 = *(undefined4 *)(iVar4 + 0x90);
  uVar1 = *(undefined4 *)(iVar4 + 0x94);
  uVar3 = pass1_1010_ae12((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),CONCAT22(param_3,local_52),
                          *(int *)((int)uVar2 + 0xa),uVar3);
  if (uVar3 != 0xffff) {
    uVar1 = *(undefined4 *)(iVar4 + 0x90);
    uVar7 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar5 = (int)uVar1;
    pass1_1010_ae92(*(ulong *)(iVar4 + 0x94),uVar3,*(uint *)(iVar5 + 0xa),*(ulong *)(iVar5 + 0x6),unaff_DI,param_3);
  }
  return;
}



void __stdcall16far pass1_1040_4d7e(ulong param_1)

{
  undefined4 uVar1;
  int *piVar2;
  undefined2 uVar3;
  int iStack8;
  ulong *puStack6;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x90);
  puStack6 = *(ulong **)((int)uVar1 + 0x2);
  iStack8 = 0x0;
  while ((piVar2 = *(int **)((int)param_1 + 0x90), *piVar2 != iStack8 && iStack8 <= *piVar2 &&
         (*(int *)((int)puStack6 + 0x4) != 0x1770))) {
    iStack8 = iStack8 + 0x1;
    puStack6 = (ulong *)((ulong)puStack6 & 0xffff0000 | (ulong)((int)puStack6 + 0xa));
  }
  pass1_1000_3e2c(*puStack6);
  return;
}



char * __stdcall16far pass1_1040_4dcc(ulong param_1,int param_2,ushort param_3)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined2 uVar3;
  char *pcVar4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  uVar2 = *(undefined4 *)((int)param_1 + 0x90);
  uVar1 = *(undefined4 *)((int)param_1 + 0x94);
  pcVar4 = string_op_1010_ada6(0x1010,param_3,(ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),param_2,
                               *(int *)((int)uVar2 + 0xa));
  return pcVar4;
}



astruct_18 * __stdcall16far pass1_1040_4df2(astruct_18 *param_1,byte param_2)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1040_4e94(astruct_57 *param_1,long param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  
  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_3,0xfab));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0x94) = 0x0;
  *(undefined4 *)(iVar1 + 0x98) = 0x0;
  *(undefined4 *)(iVar1 + 0xb0) = 0x0;
  *(undefined2 *)(iVar1 + 0xb4) = 0x0;
  *(undefined2 *)(iVar1 + 0xb6) = 0x0;
  *(undefined2 *)param_1 = 0x55a2;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  if (param_2 != 0x0) {
    uVar3 = (undefined2)((ulong)param_2 >> 0x10);
    *(undefined4 *)(iVar1 + 0xb0) = *(undefined4 *)((int)param_2 + 0x6);
    *(undefined2 *)(iVar1 + 0xb4) = *(undefined2 *)((int)param_2 + 0x14);
  }
  return;
}



void __stdcall16far pass1_1040_4f0a(astruct_18 *param_1)

{
  param_1->field_0x0 = 0x55a2;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(param_1);
  return;
}



ushort __stdcall16far
pass1_1040_4f28(ulong *param_1,int *param_2,ushort param_3,ushort param_4,int param_5,ushort param_6)

{
  code **ppcVar1;
  ushort uVar2;
  
  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566((int *)CONCAT22(param_3,param_2),param_6);
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,(ushort)param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (code **)((int)*param_1 + 0x7c);
    (**ppcVar1)(param_6,param_1,param_2,CONCAT22(param_4,param_3));
  }
  return 0x1;
}



void __stdcall16far pass1_1040_4f82(ulong *param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*param_1 + 0x80);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far set_win_pos_1040_4f96(astruct_1 *param_1,ushort param_2,ushort param_3,uchar *param_4)

{
  ulong *puVar1;
  code **ppcVar2;
  ushort uVar3;
  ulong uVar4;
  astruct_160 *paVar5;
  ushort uVar6;
  ushort uVar7;
  int iVar8;
  uchar *extraout_DX;
  uchar *puVar9;
  uchar *puVar10;
  ushort uVar11;
  ushort uVar12;
  astruct_443 *iVar11;
  int unaff_DI;
  undefined2 uVar13;
  undefined2 uVar14;
  ushort *puVar15;
  BOOL16 BVar16;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  puVar15 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x41,param_3,param_4,unaff_DI);
  uVar14 = (undefined2)((ulong)puVar15 >> 0x10);
  paVar5 = (astruct_160 *)puVar15;
  uVar13 = (undefined2)((ulong)param_1 >> 0x10);
  iVar11 = (astruct_443 *)param_1;
  *(astruct_160 **)&iVar11->field_0x98 = paVar5;
  *(undefined2 *)((int)&iVar11->field_0x98 + 0x2) = uVar14;
  ppcVar2 = (code **)((int)*iVar11->field_0x98 + 0x10);
  (**ppcVar2)(0x1010,*(undefined2 *)&iVar11->field_0x98,uVar14);
  puVar10 = extraout_DX;
  mem_op_1000_179c(0xa,extraout_DX,0x1000);
  puVar9 = (uchar *)((uint)puVar10 | (uint)paVar5);
  if (puVar9 == (uchar *)0x0) {
    iVar11->field_0x94 = 0x0;
  }
  else {
    puVar15 = struct_1040_bf3e((ushort *)CONCAT22(puVar10,paVar5),iVar11->field_0x6);
    puVar9 = (uchar *)((ulong)puVar15 >> 0x10);
    paVar5 = (astruct_160 *)puVar15;
    *(astruct_160 **)&iVar11->field_0x94 = paVar5;
    *(uchar **)((int)&iVar11->field_0x94 + 0x2) = puVar9;
  }
  pass1_1040_bfde(iVar11->field_0x94,iVar11->field_0x98,param_3);
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar10 = (uchar *)((uint)puVar9 | (uint)paVar5);
  if (puVar10 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar5,(ushort)puVar9,0x1,0xa000a,0x0,0x800081,CONCAT22(iVar11->field_0x6,0x10a),(ushort)puVar10,
                    param_3);
  }
  mem_op_1000_179c(0x42,puVar10,0x1000);
  puVar9 = (uchar *)((uint)puVar10 | (uint)paVar5);
  if (puVar9 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar5,(ushort)puVar10,0x1,0xa0028,0x0,0x840085,CONCAT22(iVar11->field_0x6,0x10b),(ushort)puVar9,
                    param_3);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar10 = (uchar *)((uint)puVar9 | (uint)paVar5);
  if (puVar10 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar5,(ushort)puVar9,0x1,0xa0046,0x0,0x860087,CONCAT22(iVar11->field_0x6,0x10d),(ushort)puVar10,
                    param_3);
  }
  mem_op_1000_179c(0x42,puVar10,0x1000);
  puVar9 = (uchar *)((uint)puVar10 | (uint)paVar5);
  if (puVar9 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar5,(ushort)puVar10,0x1,0xa0064,0x0,0x880089,CONCAT22(iVar11->field_0x6,0x10e),(ushort)puVar9,
                    param_3);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar10 = (uchar *)((uint)puVar9 | (uint)paVar5);
  if (puVar10 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar5,(ushort)puVar9,0x1,0xa0082,0x0,0x820083,CONCAT22(iVar11->field_0x6,0x10c),(ushort)puVar10,
                    param_3);
  }
  mem_op_1000_179c(0x42,puVar10,0x1000);
  puVar9 = (uchar *)((uint)puVar10 | (uint)paVar5);
  if (puVar9 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar5,(ushort)puVar10,0x1,0xa00d2,0x0,0x8a008b,CONCAT22(iVar11->field_0x6,0xbbb),(ushort)puVar9,
                    param_3);
  }
  BVar16 = 0x42;
  uVar14 = 0x1000;
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar10 = (uchar *)((uint)puVar9 | (uint)paVar5);
  if (puVar10 == (uchar *)0x0) {
    paVar5 = (astruct_160 *)0x0;
    puVar10 = (uchar *)0x0;
  }
  else {
    uVar14 = 0x1008;
    pass1_1008_3bd6(paVar5,(ushort)puVar9,0x1,0xa00a0,0x8e,0x8c008d,CONCAT22(iVar11->field_0x6,0xbbc),(ushort)puVar10,
                    param_3);
  }
  puVar9 = puVar10;
  enable_win_1040_9234(CONCAT22(puVar10,paVar5),BVar16,uVar14);
  puVar15 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_3,puVar9,unaff_DI);
  uVar11 = (ushort)((ulong)puVar15 >> 0x10);
  uVar3 = (ushort)puVar15;
  uVar12 = uVar11;
  uVar6 = pass1_1010_a5ac(uVar3,uVar11,iVar11->field_0xb0);
  uVar7 = pass1_1010_ac62(uVar3,uVar11,0x1e,uVar6,uVar12);
  if (uVar7 != 0x0) {
    pass1_1010_a5ca(uVar3,uVar11,uVar6,uVar7,uVar12);
    if (0x0 < (int)uVar7) {
      pass1_1010_a58a(uVar3,uVar11,uVar6,uVar7,uVar12);
      if (uVar7 == 0x0) {
        enable_win_1040_9234(CONCAT22(puVar10,paVar5),0x1,0x1010);
      }
    }
  }
  puVar1 = iVar11->field_0x98;
  iVar8 = (int)puVar1;
  uVar4 = (ulong)puVar1 & 0xffff0000;
  uVar14 = (undefined2)(uVar4 >> 0x10);
  SetWindowPos16(0x1010,0x40,*(INT16 *)(iVar8 + 0x10),*(INT16 *)(iVar8 + 0xe),*(INT16 *)(iVar8 + 0xc),
                 *(INT16 *)(uVar4 | iVar8 + 0xa),0x0);
  return;
}



ushort __stdcall16far pass1_1040_5238(ulong param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x94) + 0x8);
  (**ppcVar1)();
  return 0x0;
}



void __stdcall16far destroy_win_1040_5256(astruct_34 *param_1,HWND16 param_2)

{
  ULONG *pUVar1;
  uint uVar2;
  code **ppcVar3;
  BOOL16 BVar4;
  astruct_34 *iVar5;
  undefined2 uVar5;
  HWND16 HVar6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_34 *)param_1;
  HVar6 = param_2;
  if (iVar5->field_0xb6 != 0x0) {
    HVar6 = (HWND16)s_tile2_bmp_1050_1538;
    BVar4 = IsWindow16(param_2);
    if (BVar4 != 0x0) {
      HVar6 = (HWND16)s_tile2_bmp_1050_1538;
      DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
    }
  }
  iVar5->field_0xb6 = 0x0;
  pUVar1 = iVar5->field_0x94;
  uVar2 = iVar5->field_0x96;
  if ((uVar2 | (uint)pUVar1) != 0x0) {
    ppcVar3 = (code **)*pUVar1;
    (**ppcVar3)(HVar6,pUVar1,uVar2,0x1);
  }
  *(undefined4 *)&iVar5->field_0x94 = 0x0;
  iVar5->field_0x98 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
win_ui_op_1040_52c0(int param_1,ushort param_2,ushort param_3,ulong param_4,HWND16 param_5,ushort param_6)

{
  code **ppcVar1;
  BOOL16 BVar2;
  int iVar3;
  ushort uVar4;
  uchar *in_DX;
  ushort uVar5;
  ushort uVar6;
  ushort uVar7;
  int unaff_DI;
  ushort *puVar8;
  ushort *puVar9;
  undefined4 uVar10;
  ushort uVar11;
  undefined2 uVar12;
  undefined2 uVar13;
  ushort uStack30;
  RECT16 local_a;
  int iStack6;
  int iStack4;
  
  if (param_4._2_2_ != 0x10c) {
    if (param_4._2_2_ < 0x10d) {
      if (param_4._2_2_ == 0xfa) {
        uVar10 = *(undefined4 *)(param_1 + 0x98);
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0x98) + 0x18);
        (**ppcVar1)(param_5,(char)uVar10,(int)((ulong)uVar10 >> 0x10),0x0,(int)_PTR_LOOP_1050_5efe,
                    (int)((ulong)_PTR_LOOP_1050_5efe >> 0x10));
        return;
      }
      if (param_4._2_2_ == 0x10a) {
        GetClientRect16(param_5,&local_a);
        uVar10 = *(undefined4 *)(param_1 + 0x98);
        local_a.y = local_a.y + 0x3;
        local_a.x = *(int *)((int)uVar10 + 0x1a) + -0x9;
        iStack6 = iStack6 + -0x3;
        iStack4 = iStack4 + -0x3;
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),(BOOL16)&local_a);
        unk_destroy_win_op_1010_2fa0(*(ULONG *)(param_1 + 0x98),0x1010);
        pass1_1010_32c0(*(ulong *)(param_1 + 0x98),0x0);
        pass1_1010_2ee2(*(ulong **)(param_1 + 0x98),param_6,0x1010);
        return;
      }
      if (param_4._2_2_ != 0x10b) goto LAB_1040_5560;
      uVar10 = *(undefined4 *)(param_1 + 0x98);
      uVar11 = *(ushort *)((int)uVar10 + 0x12);
      uVar6 = uVar11;
      puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
      uVar5 = (ushort)((ulong)puVar8 >> 0x10);
      puVar9 = puVar8;
      pass1_1010_a5ca((ushort)puVar8,uVar5,uVar6,(ushort)puVar8,uVar5);
      uVar6 = (ushort)((ulong)puVar9 >> 0x10);
      if ((uVar11 != 0x70) && ((int)puVar9 == 0x0)) {
        return;
      }
      uVar10 = *(undefined4 *)(param_1 + 0xb0);
      uVar12 = (undefined2)uVar10;
      uVar13 = (undefined2)((ulong)uVar10 >> 0x10);
      uVar10 = *(undefined4 *)(param_1 + 0x98);
      uVar11 = *(ushort *)((int)uVar10 + 0x12);
    }
    else {
      if (param_4._2_2_ != 0x10d) {
        if (param_4._2_2_ == 0x10e) {
          puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_6,in_DX,unaff_DI);
          iVar3 = (int)puVar8;
          ui_op_1010_79aa(puVar8,0xfc6,*(long *)(param_1 + 0xb0),param_6);
          if (iVar3 != 0x0) {
            return;
          }
          unk_win_op_1010_7300((ulong)puVar8,0x0,0x13,*(ulong *)(param_1 + 0xb0));
          return;
        }
        if (param_4._2_2_ == 0xbbb) {
          if (*(int *)(param_1 + 0xb6) != 0x0) {
            BVar2 = IsWindow16(param_5);
            param_5 = (HWND16)s_tile2_bmp_1050_1538;
            if (BVar2 != 0x0) goto LAB_1040_5417;
          }
          uVar10 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(param_1 + 0x6),0x1b,(ushort)in_DX,param_1,
                                   (ushort)&PTR_LOOP_1050_1038,param_6);
          *(undefined2 *)(param_1 + 0xb6) = *(undefined2 *)((int)uVar10 + 0x6);
          set_win_pos_1038_abdc((int)&PTR_LOOP_1050_1038);
          ShowWindow16((HWND16)&PTR_LOOP_1050_1038,0x1);
          return;
        }
        if (param_4._2_2_ == 0xbbc) {
          puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
          uVar7 = (ushort)((ulong)puVar8 >> 0x10);
          uVar6 = (ushort)puVar8;
          uVar5 = uVar7;
          uVar4 = pass1_1010_a5ac(uVar6,uVar7,*(ulong *)(param_1 + 0xb0));
          uVar11 = uVar4;
          pass1_1010_a58a(uVar6,uVar7,uVar4,uVar4,uVar5);
          if (uVar11 == 0x0) {
            pass1_1010_a568(uVar6,uVar7,uVar4,0x0,uVar5);
          }
          GetDlgItem16(0x1010,0xbbc);
          EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
          return;
        }
LAB_1040_5560:
        pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,param_6);
        return;
      }
      puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
      uVar6 = (ushort)((ulong)puVar8 >> 0x10);
      uVar10 = *(undefined4 *)(param_1 + 0xb0);
      uVar12 = (undefined2)uVar10;
      uVar13 = (undefined2)((ulong)uVar10 >> 0x10);
      uVar11 = 0x71;
      uVar5 = uVar6;
    }
    uStack30 = (ushort)puVar8;
    param_5 = 0x1010;
    pass1_1010_a5ec(uStack30,uVar5,uVar11,CONCAT22(uVar13,uVar12),uVar6);
    if (*(int *)(param_1 + 0xb4) != 0x0) {
      param_5 = (HWND16)s_tile2_bmp_1050_1538;
      BVar2 = IsWindow16(0x1010);
      if (BVar2 != 0x0) {
        param_5 = (HWND16)s_tile2_bmp_1050_1538;
        SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x11100eb);
      }
    }
  }
LAB_1040_5417:
  DestroyWindow16(param_5);
  return;
}



astruct_18 * __stdcall16far pass1_1040_557c(astruct_18 *param_1,byte param_2)

{
  pass1_1040_4f0a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1040_5626(astruct_57 *param_1,ulong param_2,ushort param_3,uchar *param_4)

{
  int *piVar1;
  uint uVar2;
  uchar *puVar3;
  int iVar4;
  undefined2 uVar5;
  ulong uVar6;
  undefined2 uVar7;
  int *piStack12;
  astruct_441 *iVar8;
  astruct_396 *iVar7;
  astruct_439 *iVar6;
  
  iVar8 = (astruct_441 *)param_1;
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  struct_1040_b082(param_1,CONCAT22(param_3,0xfa3));
  uVar2 = 0x0;
  iVar8->field_0x94 = 0x0;
  iVar8->field_0x96 = 0x0;
  iVar8->field_0x98 = 0x0;
  iVar8->field_0x9c = 0x0;
  *(undefined2 *)param_1 = 0x6386;
  iVar8->field_0x2 = (int)&PTR_LOOP_1050_1040;
  mem_op_1000_179c(0x18,param_4,0x1000);
  puVar3 = (uchar *)((uint)param_4 | uVar2);
  if (puVar3 == (uchar *)0x0) {
    iVar8->field_0x90 = (int *)0x0;
  }
  else {
    struct_1040_a598((ushort *)CONCAT22(param_4,uVar2));
    *(uint *)&iVar8->field_0x90 = uVar2;
    *(uchar **)((int)&iVar8->field_0x90 + 0x2) = puVar3;
  }
  *iVar8->field_0x90 = 0x6;
  iVar4 = *iVar8->field_0x90;
  uVar2 = iVar4 * 0xa + 0x2;
  mem_op_1000_179c(uVar2,puVar3,0x1000);
  piStack12 = (int *)CONCAT22(puVar3,uVar2);
  if (((uint)puVar3 | uVar2) == 0x0) {
    piVar1 = iVar8->field_0x90;
    *(undefined4 *)((int)piVar1 + 0x2) = 0x0;
  }
  else {
    *piStack12 = iVar4;
    pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iVar4,0xa,uVar2 + 0x2,(ushort)puVar3);
    piVar1 = iVar8->field_0x90;
    uVar5 = (undefined2)((ulong)piVar1 >> 0x10);
    iVar4 = (int)piVar1;
    *(int *)(iVar4 + 0x2) = uVar2 + 0x2;
    *(uchar **)(iVar4 + 0x4) = puVar3;
  }
  piVar1 = iVar8->field_0x90;
  *(ulong *)((int)piVar1 + 0x6) = param_2;
  piVar1 = iVar8->field_0x90;
  *(undefined2 *)((int)piVar1 + 0xa) = 0x4;
  piVar1 = iVar8->field_0x90;
  *(undefined2 *)((int)piVar1 + 0x12) = iVar8->field_0xa;
  uVar6 = pass1_1040_5d12((ulong)param_1);
  uVar2 = (uint)(uVar6 >> 0x10);
  if ((uVar2 | (uint)uVar6) == 0x0) {
    iVar8->field_0x9a = 0x0;
  }
  else {
    iVar8->field_0x9a = *(undefined2 *)((uint)uVar6 + 0x20);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far enable_win_1040_5780(ulong *param_1)

{
  code **ppcVar1;
  undefined4 uVar2;
  ushort uVar3;
  uchar *extraout_DX;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar4;
  
  ppcVar1 = (code **)((int)*param_1 + 0x74);
  (**ppcVar1)();
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,extraout_DX,unaff_DI);
  uVar2 = *(undefined4 *)((int)param_1 + 0x90);
  uVar3 = pass1_1010_acc0((ushort)puVar4,(ushort)((ulong)puVar4 >> 0x10),*(ulong *)((int)uVar2 + 0x6));
  if (uVar3 != 0x0) {
    GetDlgItem16(0x1010,0x1790);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  }
  return;
}



void __stdcall16far pass1_1040_57d4(astruct_1 *param_1,uchar *param_2,int param_3,ushort param_4,ushort param_5)

{
  pass1_1040_5d42((ulong)param_1);
  pass1_1040_5eaa((ulong)param_1);
  pass1_1040_5dc4((ulong)param_1,param_2,param_3,param_5);
  unk_win_ui_op_1040_b230(param_1,param_4,param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1040_5800(int param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint uVar3;
  ushort uVar4;
  astruct_18 *paVar5;
  uchar *in_DX;
  uchar *puVar6;
  uchar *puVar7;
  uchar *extraout_DX;
  int iVar8;
  uchar *unaff_DI;
  undefined2 uVar9;
  HWND16 hwnd;
  ushort unaff_SS;
  int *piStack24;
  RECT16 local_14 [0x2];
  int iStack12;
  astruct_18 *paStack10;
  astruct_20 *paStack6;
  
  if (param_4._2_2_ == 0xeb) {
    paStack6 = (astruct_20 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,in_DX,(int)unaff_DI);
    puVar6 = (uchar *)((ulong)paStack6 >> 0x10);
    paVar5 = *(astruct_18 **)(param_1 + 0x90);
    if (paVar5 != (astruct_18 *)0x0) {
      paStack10 = paVar5;
      mem_op_1000_179c(0x18,puVar6,0x1000);
      uVar3 = (uint)paVar5;
      puVar7 = (uchar *)((uint)puVar6 | uVar3);
      if (puVar7 == (uchar *)0x0) {
        uVar3 = 0x0;
        puVar7 = (uchar *)0x0;
      }
      else {
        struct_1040_a598((ushort *)((ulong)paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
      }
      *(uint *)(param_1 + 0x90) = uVar3;
      *(uchar **)(param_1 + 0x92) = puVar7;
      *(undefined2 *)*(undefined4 *)(param_1 + 0x90) = 0x6;
      iStack12 = **(int **)(param_1 + 0x90);
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,puVar7,0x1000);
      piStack24 = (int *)CONCAT22(puVar7,uVar3);
      if (((uint)puVar7 | uVar3) == 0x0) {
        uVar2 = *(undefined4 *)(param_1 + 0x90);
        *(undefined4 *)((int)uVar2 + 0x2) = 0x0;
      }
      else {
        *piStack24 = iStack12;
        pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iStack12,0xa,uVar3 + 0x2,(ushort)puVar7);
        uVar2 = *(undefined4 *)(param_1 + 0x90);
        uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
        iVar8 = (int)uVar2;
        *(int *)(iVar8 + 0x2) = uVar3 + 0x2;
        *(uchar **)(iVar8 + 0x4) = puVar7;
        unaff_DI = puVar7;
      }
      uVar2 = *(undefined4 *)(param_1 + 0x90);
      *(undefined4 *)((int)uVar2 + 0x6) = *(undefined4 *)((int)paStack10 + 0x6);
      uVar2 = *(undefined4 *)(param_1 + 0x90);
      *(undefined2 *)((int)uVar2 + 0xa) = 0x4;
      uVar2 = *(undefined4 *)(param_1 + 0x90);
      *(undefined2 *)((int)uVar2 + 0x12) = *(undefined2 *)(param_1 + 0xa);
      hwnd = 0x1010;
      pass1_1010_a50c(paStack6,0x10505d78,*(ulong *)(param_1 + 0x90));
      if (paStack10 != (astruct_18 *)0x0) {
        pass1_1040_a5d0((ulong)paStack10);
        hwnd = 0x1000;
        fn_ptr_1000_17ce(paStack10,0x1000);
      }
      ppcVar1 = (code **)((int)*(undefined4 *)CONCAT22(param_2,param_1) + 0x70);
      (**ppcVar1)();
      puVar6 = extraout_DX;
      uVar4 = pass1_1040_5cd6(CONCAT22(param_2,param_1));
      if (uVar4 != 0x0) {
        pass1_1040_5eaa(CONCAT22(param_2,param_1));
        *(undefined2 *)(param_1 + 0x94) = 0x0;
      }
      pass1_1040_5dc4(CONCAT22(param_2,param_1),puVar6,(int)unaff_DI,unaff_SS);
      GetWindowRect16(hwnd,local_14);
      InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,*(RECT16 **)(param_1 + 0x9c),0x0);
      if (*(int *)(param_1 + 0x9c) != 0x0) {
        *(undefined2 *)(param_1 + 0x9c) = 0x0;
      }
    }
  }
  else {
    if (param_4._2_2_ != 0x13b) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,unaff_SS);
      return;
    }
    GetDlgItem16(param_5,0x1790);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far draw_op_1040_5a06(ulong param_1,HWND16 param_2,ushort param_3)

{
  uint *puVar1;
  undefined4 uVar2;
  code **ppcVar3;
  undefined4 uVar4;
  HPALETTE16 b_force_background;
  int iVar5;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  int x;
  int y;
  ushort in_DX;
  int iVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  INT16 IVar9;
  ulong uVar10;
  astruct_43 *paVar11;
  astruct_76 *paVar12;
  undefined2 uVar13;
  HDC16 *pHVar14;
  ushort uVar15;
  HDC16 HVar16;
  HDC16 HVar17;
  HDC16 HVar18;
  undefined2 uVar19;
  undefined2 uVar20;
  undefined2 uVar21;
  uint uStack82;
  int iStack72;
  int iStack68;
  astruct_76 *paStack54;
  HDC16 local_2c;
  PAINTSTRUCT16 local_2a;
  RECT16 local_a [0x2];
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar21 = *(undefined2 *)(iVar6 + 0x6);
  GetWindowRect16(param_2,local_a);
  uVar13 = *(undefined2 *)(iVar6 + 0x6);
  local_2c = BeginPaint16((HWND16)s_tile2_bmp_1050_1538,&local_2a);
  uVar8 = 0x1008;
  b_force_background = palette_op_1008_4e08(*(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe),&local_2c,in_DX,0x1008);
  paStack54 = (astruct_76 *)0x0;
  if (*(int *)(iVar6 + 0x98) != 0x0) {
    paStack54 = (astruct_76 *)unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,*(ushort *)(iVar6 + 0x98),param_3);
    IVar9 = 0x1008;
    uVar10 = pass1_1008_4772(paStack54);
    if (((uint)(uVar10 >> 0x10) | (uint)uVar10) == 0x0) {
      if (paStack54 != (astruct_76 *)0x0) {
        if (paStack54 != (astruct_76 *)0x0) {
          ppcVar3 = (code **)*(undefined4 *)paStack54;
          (**ppcVar3)(0x1008,(int)paStack54,(int)((ulong)paStack54 >> 0x10),0x1,uVar13);
        }
      }
      IVar9 = 0x1010;
      paStack54 = (astruct_76 *)unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x4d,param_3);
    }
    uVar8 = SUB42(s_tile2_bmp_1050_1538,0x0);
    GetSystemMetrics16(IVar9);
    ppcVar3 = (code **)((int)*(undefined4 *)paStack54 + 0x4);
    (**ppcVar3)();
  }
  if (paStack54 != (astruct_76 *)0x0) {
    if (paStack54 != (astruct_76 *)0x0) {
      ppcVar3 = (code **)*(undefined4 *)paStack54;
      (**ppcVar3)(uVar8,(int)paStack54,(int)((ulong)paStack54 >> 0x10),0x1,uVar13,uVar21);
    }
  }
  paVar11 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,*(ushort *)(iVar6 + 0x96),param_3);
  uVar21 = (undefined2)((ulong)paVar11 >> 0x10);
  pHVar14 = &local_2c;
  uVar19 = 0x4;
  uVar20 = 0xd;
  uVar15 = param_3;
  IVar9 = GetSystemMetrics16(0x1010);
  iVar5 = -(IVar9 + -0x23);
  uVar4 = *(undefined4 *)paVar11;
  ppcVar3 = (code **)uVar4 + 0x2;
  uVar13 = (int)paVar11;
  uVar8 = uVar21;
  (**ppcVar3)();
  if (paVar11 != (astruct_43 *)0x0) {
    if (paVar11 != (astruct_43 *)0x0) {
      ppcVar3 = (code **)uVar4;
      (**ppcVar3)((int)s_tile2_bmp_1050_1538,(int)paVar11,uVar21,0x1,uVar13,uVar8,iVar5,uVar19,uVar20,pHVar14,uVar15);
    }
  }
  handle = CreatePen16((INT16)s_tile2_bmp_1050_1538,0x25,0x100);
  handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
  paVar12 = (astruct_76 *)unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x4f,param_3);
  uVar21 = (undefined2)((ulong)paVar12 >> 0x10);
  uVar10 = pass1_1008_4772(paVar12);
  uVar13 = (undefined2)(uVar10 >> 0x10);
  uVar4 = *(undefined4 *)((int)uVar10 + 0x4);
  uVar2 = *(undefined4 *)((int)uVar10 + 0x8);
  IVar9 = GetSystemMetrics16(0x1008);
  iVar5 = -(IVar9 + -0xc1);
  IVar9 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iStack72 = (int)uVar2;
  x = 0xc5 - (IVar9 - iStack72);
  MoveTo16((HDC16)s_tile2_bmp_1050_1538,iVar5,0x82);
  iStack68 = (int)uVar4;
  y = iStack68 * 0xa + 0x85;
  LineTo16((HDC16)s_tile2_bmp_1050_1538,iVar5,y);
  HVar18 = local_2c;
  LineTo16((HDC16)s_tile2_bmp_1050_1538,x,y);
  HVar17 = local_2c;
  LineTo16((HDC16)s_tile2_bmp_1050_1538,x,0x82);
  HVar16 = local_2c;
  LineTo16((HDC16)s_tile2_bmp_1050_1538,iVar5,0x82);
  for (uStack82 = 0x0; puVar1 = (uint *)(iVar6 + 0x94), uStack82 <= *puVar1 && *puVar1 != uStack82;
      uStack82 = uStack82 + 0x1) {
    pHVar14 = &local_2c;
    iVar5 = iStack68 * uStack82 + 0x84;
    uVar13 = 0x4;
    uVar15 = param_3;
    IVar9 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    ppcVar3 = (code **)((int)*(undefined4 *)paVar12 + 0x4);
    (**ppcVar3)((int)s_tile2_bmp_1050_1538,(int)paVar12,uVar21,-(IVar9 + -0xc4),uVar13,iVar5,pHVar14,uVar15,HVar16,
                HVar17);
  }
  if (paVar12 != (astruct_76 *)0x0) {
    if (paVar12 != (astruct_76 *)0x0) {
      ppcVar3 = (code **)*(undefined4 *)paVar12;
      (**ppcVar3)((int)s_tile2_bmp_1050_1538,(int)paVar12,uVar21,0x1,HVar16,HVar17,HVar18);
    }
  }
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_00);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,b_force_background);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_2a);
  return;
}



ushort __stdcall16far pass1_1040_5cd6(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  ulong uVar3;
  
  uVar3 = pass1_1040_5d12(param_1);
  if (uVar3 != 0x0) {
    iVar1 = *(int *)((int)uVar3 + 0x20);
    uVar2 = (undefined2)(param_1 >> 0x10);
    if (*(int *)((int)param_1 + 0x9a) != iVar1) {
      *(int *)((int)param_1 + 0x9a) = iVar1;
      return 0x1;
    }
  }
  return 0x0;
}



ulong __stdcall16far pass1_1040_5d12(ulong param_1)

{
  uint uVar1;
  uint uVar2;
  undefined4 uVar3;
  astruct_440 *iVar4;
  undefined2 uVar4;
  ulong uVar5;
  
  uVar3 = *(undefined4 *)((int)param_1 + 0x90);
  uVar4 = (undefined2)((ulong)uVar3 >> 0x10);
  iVar4 = (astruct_440 *)uVar3;
  uVar1 = iVar4->field_0x6;
  uVar2 = iVar4->field_0x8;
  if ((uVar2 | uVar1) != 0x0) {
    uVar5 = struct_op_1030_73a8(CONCAT22(uVar2,uVar1));
    return uVar5;
  }
  return 0x0;
}



void __stdcall16far pass1_1040_5d42(ulong param_1)

{
  uint uVar1;
  char cVar2;
  int iVar3;
  undefined2 uVar4;
  ulong uVar5;
  
  uVar5 = pass1_1040_5d12(param_1);
  if (uVar5 != 0x0) {
    uVar1 = *(uint *)((int)uVar5 + 0xc);
    iVar3 = (int)param_1;
    uVar4 = (undefined2)(param_1 >> 0x10);
    if (uVar1 == 0x5f) {
      *(undefined2 *)(iVar3 + 0x96) = 0x53;
      return;
    }
    if (uVar1 < 0x60) {
      cVar2 = (char)uVar1;
      if (cVar2 == '(') {
        *(undefined2 *)(iVar3 + 0x96) = 0x54;
        return;
      }
      if (cVar2 == ')') {
        *(undefined2 *)(iVar3 + 0x96) = 0x55;
        return;
      }
      if (cVar2 == ']') {
        *(undefined2 *)(iVar3 + 0x96) = 0x51;
        return;
      }
      if (cVar2 == '^') {
        *(undefined2 *)(iVar3 + 0x96) = 0x52;
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_5dc4(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  code **ppcVar1;
  undefined4 uVar2;
  ushort uVar3;
  uint uVar4;
  uint uVar5;
  uchar *puVar6;
  uint extraout_DX;
  astruct_724 *iVar7;
  undefined2 uVar7;
  ushort *puVar8;
  undefined4 *puVar9;
  ushort uVar10;
  int iStack18;
  
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,param_2,param_3);
  puVar6 = (uchar *)((ulong)puVar8 >> 0x10);
  uVar3 = (ushort)puVar8;
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar7 = (astruct_724 *)param_1;
  pass1_1010_a5ca(uVar3,(ushort)puVar6,iVar7->field_0x9a,uVar3,(ushort)puVar6);
  if (uVar3 == 0x0) {
    iVar7->field_0x94 = 0x0;
    iVar7->field_0x9c = 0x1;
  }
  if (-0x1 < (int)uVar3) {
    if ((int)iVar7->field_0x9a < 0x72) {
      uVar10 = 0x31;
    }
    else {
      uVar10 = 0x41;
    }
    puVar9 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uVar10,param_4,puVar6,param_3);
    uVar4 = iVar7->field_0x9a;
    ppcVar1 = (code **)((int)*puVar9 + 0x14);
    (**ppcVar1)(0x1010,(int)puVar9,(int)((ulong)puVar9 >> 0x10),uVar4,(int)uVar4 >> 0xf);
    if ((extraout_DX | uVar4) == 0x0) {
      iStack18 = 0x0;
    }
    else {
      uVar2 = *(undefined4 *)(uVar4 + 0x16);
      iStack18 = *(int *)((int)uVar2 + 0x4);
    }
    if ((iStack18 != 0x0) && (uVar3 != 0x0)) {
      uVar5 = (int)((iStack18 - uVar3) * 0x64) / iStack18;
      uVar4 = uVar5 / 0xa;
      iVar7->field_0x94 = uVar4;
      if (0x4 < uVar5 % 0xa) {
        iVar7->field_0x94 = uVar4 + 0x1;
      }
    }
  }
  return;
}

