


void __stdcall16far send_msg_1040_c85a(ulong param_1,HWND16 param_2)

{
  _PTR_LOOP_1050_5efe = param_1;
  SendMessage16(param_2,0x0,0x0,0x11100fa);
  return;
}





astruct_18 * __stdcall16far pass1_1040_a4c2(astruct_18 *param_1,byte param_2,ushort param_3)

{
  free_proc_inst_1040_a294(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar5

ulong __stdcall16far
call_win_proc_1040_a40e(HWND16 param_1,ulong param_2,LPARAM param_3,uint param_4,LPVOID param_5,ushort param_6)

{
  uint uVar1;
  code **ppcVar2;
  undefined4 *puVar4;
  WPARAM16 wparam;
  int iVar5;
  int unaff_DI;
  ushort uVar6;
  ulong uVar7;
  ulong uStack6;
  ulong *puVar3;
  undefined4 uVar5;
  
  uStack6 = 0x0;
  wparam = (WPARAM16)(param_2 >> 0x10);
  if ((int)param_3 == 0x19) {
    puVar4 = (undefined4 *)*(undefined4 *)&PTR_LOOP_1050_5ee0;
    ppcVar2 = (code **)((int)*puVar4 + 0x34);
    uStack6 = (**ppcVar2)(param_5,(char)puVar4,(int)((ulong)puVar4 >> 0x10),param_1,param_2,(int)&USHORT_1050_1050);
    param_4 = (uint)(uStack6 >> 0x10);
  }
  else {
    if ((int)param_3 == 0x86) {
      puVar4 = (undefined4 *)*(undefined4 *)&PTR_LOOP_1050_5ee0;
      ppcVar2 = (code **)((int)*puVar4 + 0x20);
      uVar7 = (**ppcVar2)(param_5,(int)puVar4,(int)((ulong)puVar4 >> 0x10),wparam);
      return uVar7;
    }
    if ((int)param_3 == 0x110) {
      uVar7 = win_msg_1040_a308(*(ulong *)&PTR_LOOP_1050_5ee0,unaff_DI,(HWND16)param_5,param_6);
      return uVar7;
    }
  }
  if (uStack6 != 0x0) {
    return uStack6 & 0xffff | (ulong)param_4 << 0x10;
  }
  uVar5 = *(undefined4 *)&PTR_LOOP_1050_5bc8;
  uVar6 = (ushort)((ulong)uVar5 >> 0x10);
  iVar5 = (int)uVar5;
  uVar1 = *(uint *)(iVar5 + 0x6);
  if ((uVar1 | *(uint *)(iVar5 + 0x4)) == 0x0) {
    return (ulong)uVar1 << 0x10;
  }
  uVar7 = CallWindowProc16(param_5,param_1,(UINT16)param_2,wparam,param_3);
  return uVar7;
}



void __stdcall16far reg_class_1040_98c0(ULONG param_1,HINSTANCE16 param_2,WNDCLASS16 *in_wnd_class_3)

{
  BOOL16 BVar1;
  ATOM AVar2;
  undefined2 l_name;
  undefined2 uStack26;
  undefined2 uStack24;
  undefined4 uStack22;
  undefined *puStack18;
  undefined2 uStack16;
  undefined2 uStack14;
  undefined2 uStack12;
  undefined4 uStack10;
  int iStack6;
  undefined2 uStack4;
  
  iStack6 = (int)param_1 + 0x4;
  BVar1 = GetClassInfo16(param_2,(SEGPTR)&l_name,in_wnd_class_3);
  if (BVar1 == 0x0) {
    l_name = *(undefined2 *)((int)param_1 + 0x54);
    uStack26 = 0x9cde;
    uStack24 = SUB42(&PTR_LOOP_1050_1040,0x0);
    uStack22 = 0x40000;
    puStack18 = PTR_LOOP_1050_038c;
    uStack16 = 0x0;
    uStack14 = *(undefined2 *)((int)param_1 + 0x58);
    uStack12 = *(undefined2 *)((int)param_1 + 0x56);
    uStack10 = 0x0;
    uStack4 = param_1._2_2_;
    AVar2 = RegisterClass16((WNDCLASS16 *)s_tile2_bmp_1050_1538);
    if (AVar2 == 0x0) {
      fn_ptr_op_1000_24cd(0x0,&stack0xfffe);
    }
  }
  return;
}




void __stdcall16far
win_op_1040_9cde(UINT16 param_1,WPARAM16 param_2,int param_3,ulong param_4,HWND16 param_5,ushort param_6)

{
  byte *pbVar1;
  int iVar2;
  undefined2 uVar3;
  ushort uVar4;
  int iVar5;
  INT16 IVar6;
  BOOL16 BVar7;
  uint offset;
  uchar *puVar8;
  uint uVar9;
  undefined2 uVar10;
  HWND16 HVar11;
  astruct_18 *paVar12;
  INT16 *pIVar13;
  LRESULT LVar14;
  ulong uVar15;
  byte bVar16;
  uint uStack30;
  RECT16 local_a [0x2];
  
  HVar11 = (HWND16)s_tile2_bmp_1050_1538;
  paVar12 = (astruct_18 *)GetWindowLong16(param_5,0x0);
  puVar8 = (uchar *)((ulong)paVar12 >> 0x10);
  iVar5 = (int)paVar12;
  uVar10 = (undefined2)(((ulong)paVar12 & 0xffff0000) >> 0x10);
  if ((uint)param_4 == 0x30) {
    *(int *)(iVar5 + 0x5a) = param_3;
  }
  else {
    bVar16 = (byte)param_4;
    if ((uint)param_4 < 0x31) {
      if ((uint)param_4 == 0x1f) {
        *(undefined2 *)(iVar5 + 0x4) = 0x0;
        ReleaseCapture16();
        return;
      }
      if (0x1f < (uint)param_4) goto LAB_1040_a1ae;
      if (bVar16 == 0x8) {
        pbVar1 = (byte *)(iVar5 + 0x4);
        *pbVar1 = *pbVar1 & 0xf7;
        uStack30 = 0x0;
        BVar7 = IsWindow16((HWND16)s_tile2_bmp_1050_1538);
        if (BVar7 != 0x0) {
          uVar15 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x870000);
          uStack30 = (uint)((uVar15 & 0x20) == 0x0);
        }
        *(undefined4 *)(iVar5 + 0x56) = 0x0;
        SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,CONCAT22(0x401,*(undefined2 *)(iVar5 + 0x5c)));
        if ((*(int *)(iVar5 + 0x5c) != 0x0) && (*(ushort *)(iVar5 + 0x5c) != paVar12->field_0x0)) {
          uVar3 = *(undefined2 *)(iVar5 + 0x5c);
          SendDlgItemMessage16
                    ((HWND16)s_tile2_bmp_1050_1538,uStack30,0x0,0x1,
                     CONCAT13((char)((uint)uVar3 >> 0x8),CONCAT12((char)uVar3,0x404)));
        }
        HVar11 = (HWND16)s_tile2_bmp_1050_1538;
        *(undefined2 *)(iVar5 + 0x5c) = 0x0;
      }
      else {
        if (bVar16 < 0x9) {
          if (bVar16 == 0x1) {
            pIVar13 = (INT16 *)GetWindowLong16((HWND16)s_tile2_bmp_1050_1538,0x0);
            iVar5 = (int)pIVar13;
            uVar10 = (undefined2)(((ulong)pIVar13 & 0xffff0000) >> 0x10);
            *(undefined2 *)(iVar5 + 0x2) = *(undefined2 *)(param_1 + 0x8);
            IVar6 = GetDlgCtrlID16((HWND16)s_tile2_bmp_1050_1538);
            *pIVar13 = IVar6;
            *(undefined4 *)(iVar5 + 0x56) = *(undefined4 *)(param_1 + 0x12);
            unk_str_op_1000_3d3e
                      ((char *)((ulong)pIVar13 & 0xffff0000 | (ulong)(iVar5 + 0x6)),*(char **)(param_1 + 0x16));
            if ((*(byte *)(param_1 + 0x12) & 0x1) != 0x0) {
              SendMessage16(0x1000,0x0,0x0,CONCAT22(0x401,*pIVar13));
            }
            if ((*(uint *)(param_1 + 0x14) & 0x800) == 0x0) {
              return;
            }
            pbVar1 = (byte *)(iVar5 + 0x4);
            *pbVar1 = *pbVar1 | 0x4;
            return;
          }
          if (bVar16 == 0x2) {
            fn_ptr_1000_17ce(paVar12,0x1000);
            SetWindowLong16(0x1000,0x0,0x0);
            return;
          }
          if (bVar16 != 0x7) goto LAB_1040_a1ae;
          pbVar1 = (byte *)(iVar5 + 0x4);
          *pbVar1 = *pbVar1 | 0x8;
          LVar14 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4000000);
          uVar4 = (ushort)LVar14;
          if (((int)((ulong)LVar14 >> 0x10) == 0x534b) &&
             (*(ushort *)(iVar5 + 0x5c) = uVar4, uVar4 != paVar12->field_0x0)) {
            SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x1,0x0,0x0,CONCAT22(uVar4,0x404));
          }
          HVar11 = (HWND16)s_tile2_bmp_1050_1538;
          SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,CONCAT22(0x401,paVar12->field_0x0));
          *(undefined4 *)(iVar5 + 0x56) = 0x1;
        }
        else {
          if (bVar16 == 0xa) {
            pbVar1 = (byte *)(iVar5 + 0x4);
            *pbVar1 = *pbVar1 & 0xfb;
            if (param_3 == 0x0) {
              pbVar1 = (byte *)(iVar5 + 0x4);
              *pbVar1 = *pbVar1 | 0x4;
            }
          }
          else {
            if (bVar16 != 0xc) {
              if (bVar16 == 0xf) {
                draw_op_1040_9948(param_4._2_2_,(ulong)paVar12,(int)s_tile2_bmp_1050_1538,param_6);
                return;
              }
              goto LAB_1040_a1ae;
            }
            if (CONCAT22(param_2,param_1) != 0x0) {
              HVar11 = 0x1000;
              unk_str_op_1000_3d3e
                        ((char *)((ulong)paVar12 & 0xffff0000 | (ulong)(iVar5 + 0x6)),(char *)CONCAT22(param_2,param_1))
              ;
            }
          }
        }
      }
      goto LAB_1040_9e20;
    }
    if ((uint)param_4 == 0x200) {
      if ((*(byte *)(iVar5 + 0x4) & 0x1) == 0x0) {
        return;
      }
      GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_a);
      iVar2 = *(int *)(iVar5 + 0x4);
      BVar7 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,(POINT16)CONCAT22(param_2,param_1));
      if (BVar7 == 0x0) {
        pbVar1 = (byte *)(iVar5 + 0x4);
        *pbVar1 = *pbVar1 & 0xfd;
      }
      else {
        pbVar1 = (byte *)(iVar5 + 0x4);
        *pbVar1 = *pbVar1 | 0x2;
      }
      param_1 = *(int *)(iVar5 + 0x4) - iVar2;
    }
    else {
      if ((uint)param_4 < 0x201) {
        offset = (uint)param_4 - 0x81;
        if (offset == 0x0) {
          uVar10 = 0x5e;
          mem_op_1000_179c(0x5e,puVar8,0x1000);
          uVar9 = (uint)puVar8 | offset;
          if (uVar9 == 0x0) {
            offset = 0x0;
            uVar9 = 0x0;
          }
          else {
            pass1_1040_9824((ulong *)CONCAT22(puVar8,offset));
          }
          SetWindowLong16(0x1000,offset,CONCAT22(uVar10,uVar9));
          return;
        }
        if ((uint)param_4 == 0x87) {
          return;
        }
        if ((uint)param_4 == 0x100) {
          if ((param_3 == 0x26) || (param_3 == 0x25)) {
            HVar11 = 0x1;
          }
          else {
            if ((param_3 != 0x28) && (param_3 != 0x27)) {
              if (((param_3 == 0x20) || (param_3 == 0xd)) && (*(int *)&PTR_LOOP_1050_5ed8 == 0x0)) {
                *(undefined2 *)&PTR_LOOP_1050_5ed8 = 0x1;
                pbVar1 = (byte *)(iVar5 + 0x4);
                *pbVar1 = *pbVar1 | 0x2;
                goto LAB_1040_9e20;
              }
              goto LAB_1040_a1ae;
            }
            HVar11 = 0x0;
          }
          GetNextDlgTabItem16((HWND16)s_tile2_bmp_1050_1538,HVar11,param_4._2_2_);
          SetFocus16((HWND16)s_tile2_bmp_1050_1538);
          return;
        }
        if (((uint)param_4 == 0x101) && (*(int *)&PTR_LOOP_1050_5ed8 != 0x0)) {
          *(undefined2 *)&PTR_LOOP_1050_5ed8 = 0x0;
          pbVar1 = (byte *)(iVar5 + 0x4);
          *pbVar1 = *pbVar1 & 0xfd;
          InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),0x0);
          UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
          SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,CONCAT22(0x111,paVar12->field_0x0));
          return;
        }
LAB_1040_a1ae:
        DefWindowProc16((HWND16)s_tile2_bmp_1050_1538,param_1,param_2,
                        CONCAT13((char)(param_4 >> 0x8),CONCAT12(bVar16,param_3)));
        return;
      }
      if ((uint)param_4 == 0x201) {
LAB_1040_9e74:
        SetFocus16((HWND16)s_tile2_bmp_1050_1538);
        pbVar1 = (byte *)(iVar5 + 0x4);
        *pbVar1 = *pbVar1 | 0x3;
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),0x0);
        UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
        SetCapture16((HWND16)s_tile2_bmp_1050_1538);
        return;
      }
      if ((uint)param_4 == 0x202) {
        ReleaseCapture16();
        GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_a);
        if ((*(byte *)(iVar5 + 0x4) & 0x1) == 0x0) {
          return;
        }
        pbVar1 = (byte *)(iVar5 + 0x4);
        *pbVar1 = *pbVar1 & 0xfc;
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),0x0);
        UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
        BVar7 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,(POINT16)CONCAT22(param_2,param_1));
        if (BVar7 == 0x0) {
          return;
        }
        PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,CONCAT22(0x111,paVar12->field_0x0));
        return;
      }
      if ((uint)param_4 == 0x203) goto LAB_1040_9e74;
      if ((uint)param_4 != 0x404) goto LAB_1040_a1ae;
      if (param_3 == 0x1) {
        *(undefined4 *)(iVar5 + 0x56) = 0x1;
      }
      else {
        *(undefined4 *)(iVar5 + 0x56) = 0x0;
      }
    }
  }
  HVar11 = (HWND16)s_tile2_bmp_1050_1538;
  if (param_1 == 0x0) {
    return;
  }
LAB_1040_9e20:
  InvalidateRect16(HVar11,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),0x0);
  UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
  return;
}


void __stdcall16far make_proc_inst_1040_a234(uchar *param_1,uchar *param_2,ushort param_3,ulong param_4,LPVOID param_5)

{
  LPVOID pvVar1;
  undefined2 in_DX;
  
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22((int)param_4,param_3),(ushort)(param_4 >> 0x10));
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xa4e8;
  *(int *)(param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  if (_PTR_LOOP_1050_5edc == 0x0) {
    pvVar1 = MakeProcInstance16(param_5,(HANDLE16)PTR_LOOP_1050_038c);
    _PTR_LOOP_1050_5edc = CONCAT22(in_DX,pvVar1);
  }
  *(long *)(param_1 + 0xc) = _PTR_LOOP_1050_5edc;
  PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + 0x1;
  PTR_LOOP_1050_5ee0 = param_1;
  PTR_LOOP_1050_5ee2 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far free_proc_inst_1040_a294(astruct_18 *param_1,ushort param_2)

{
  param_1->field_0x0 = 0xa4e8;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + -0x1;
  if (PTR_LOOP_1050_5eda == (undefined *)0x0) {
    FreeProcInstance16((LPVOID)param_2);
    _PTR_LOOP_1050_5edc = 0x0;
  }
  unk_draw_op_1040_b0f8(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined4 __stdcall16far win_msg_1040_a308(ulong param_1,int param_2,HWND16 param_3,ushort param_4)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  HWND16 hwnd;
  LRESULT LVar5;
  ushort *puVar6;
  char *pcVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  int iStack12;
  
  SendMessage16(param_3,0x0,0x0,0x4050000);
  LVar5 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0000);
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  uVar2 = *(undefined4 *)(iVar3 + 0x90);
  if (*(int *)((int)uVar2 + 0x10) == 0x0) {
    uVar4 = 0x0;
    uVar8 = 0x401;
    pcVar7 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    SendMessage16(0x1010,(UINT16)pcVar7,(WPARAM16)((ulong)pcVar7 >> 0x10),CONCAT22(uVar8,uVar4));
  }
  else {
    hwnd = 0x1010;
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,(uchar *)((ulong)LVar5 >> 0x10),param_2);
    for (iStack12 = 0x0; uVar2 = *(undefined4 *)(iVar3 + 0x90), piVar1 = (int *)((int)uVar2 + 0x10),
        *piVar1 != iStack12 && iStack12 <= *piVar1; iStack12 = iStack12 + 0x1) {
      uVar8 = 0x0;
      uVar9 = 0x401;
      uVar2 = *(undefined4 *)(iVar3 + 0x90);
      uVar2 = *(undefined4 *)((int)uVar2 + 0xc);
      pcVar7 = load_string_1010_ac92
                         (0x1010,(ushort)puVar6,(ushort)((ulong)puVar6 >> 0x10),*(int *)((int)uVar2 + iStack12 * 0x2));
      hwnd = (HWND16)s_tile2_bmp_1050_1538;
      SendMessage16(0x1010,(UINT16)pcVar7,(WPARAM16)((ulong)pcVar7 >> 0x10),CONCAT22(uVar9,uVar8));
    }
  }
  LVar5 = SendMessage16(hwnd,0x0,0x0,0xb0001);
  return CONCAT22((int)((ulong)LVar5 >> 0x10),0x1);
}


uchar * __stdcall16far win_ui_op_1040_8718(astruct_37 *param_1,ushort param_2)

{
  int *piVar1;
  int iVar2;
  ushort uVar3;
  uchar *extraout_DX;
  uchar *puVar4;
  int unaff_DI;
  undefined2 uVar5;
  ushort *puVar6;
  undefined2 uVar7;
  undefined2 uVar9;
  UINT32 UVar10;
  int local_104 [0x80];
  uint uStack4;
  ushort uVar8;
  
  uVar5 = 0x1008;
  unk_win_msg_op_1008_9510((int *)&PTR_LOOP_1050_5df4,0x1008,param_2);
  UVar10 = (UINT32)param_1;
  uVar8 = (ushort)((ulong)param_1 >> 0x10);
  dialog_ui_fn_1040_78e2((astruct_1 *)param_1,0x1008);
  PTR_LOOP_1050_5df6 = (undefined *)*(undefined2 *)(UVar10 + 0x6);
  if (*(long *)(UVar10 + 0x94) != 0x0) {
    uVar5 = 0x1000;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(UVar10 + 0x10)),*(char **)(UVar10 + 0x94));
  }
  get_sys_metrics_1040_8c66(param_1,uVar5);
  uStack4 = *(byte *)(UVar10 + 0x98) & 0xf;
  if (uStack4 == 0x1) {
    *(int *)(UVar10 + 0xae) = (*(int *)(UVar10 + 0xaa) + -0xc4) / 0x2;
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0xff,(char *)local_104,
               param_2);
    create_window_1040_8bea(UVar10,uVar8,0x1,0x1,(int)*(undefined4 *)(UVar10 + 0xae));
    piVar1 = (int *)(UVar10 + 0xae);
    *piVar1 = *piVar1 + 0x6c;
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0xff,(char *)local_104,
               param_2);
    uVar9 = (undefined2)*(undefined4 *)(UVar10 + 0xae);
    uVar7 = 0x2;
  }
  else {
    if (uStack4 != 0x4) {
      *(int *)(UVar10 + 0xae) = (*(int *)(UVar10 + 0xaa) + -0x58) / 0x2;
      load_string_1010_84e0
                (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0xff,(char *)local_104,
                 param_2);
      uVar9 = (undefined2)*(undefined4 *)(UVar10 + 0xae);
      uVar5 = 0x1;
      uVar7 = 0x1;
      goto LAB_1040_88a5;
    }
    *(int *)(UVar10 + 0xae) = (*(int *)(UVar10 + 0xaa) + -0xc4) / 0x2;
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0xff,(char *)local_104,
               param_2);
    create_window_1040_8bea(UVar10,uVar8,0x1,0x6,(int)*(undefined4 *)(UVar10 + 0xae));
    piVar1 = (int *)(UVar10 + 0xae);
    *piVar1 = *piVar1 + 0x6c;
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0xff,(char *)local_104,
               param_2);
    uVar9 = (undefined2)*(undefined4 *)(UVar10 + 0xae);
    uVar7 = 0x7;
  }
  uVar5 = 0x0;
LAB_1040_88a5:
  create_window_1040_8bea(UVar10,uVar8,uVar5,uVar7,uVar9);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_2,extraout_DX,unaff_DI);
  uVar5 = (undefined2)((ulong)puVar6 >> 0x10);
  local_104[0] = *(int *)((int)puVar6 + 0xa);
  uStack4 = *(int *)((int)puVar6 + 0xc);
  iVar2 = uStack4 - *(int *)(UVar10 + 0xac);
  puVar4 = (uchar *)(iVar2 >> 0xf);
  SetWindowPos16(0x1010,0x40,*(INT16 *)(UVar10 + 0xac),*(INT16 *)(UVar10 + 0xaa),iVar2 / 0x2,
                 (local_104[0] - *(int *)(UVar10 + 0xaa)) / 0x2,0x0);
  PTR_LOOP_1050_5df4 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
  unk_win_msg_op_1008_9510((int *)&PTR_LOOP_1050_5df4,0x1008,param_2);
  destroy_win_1040_8b7e(0x1008);
  PTR_LOOP_1050_5df6 = (undefined *)0x0;
  if (*(int *)(UVar10 + 0xb2) != 0x0) {
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_2,puVar4,unaff_DI);
    uVar3 = pass1_1008_ab54((ulong)puVar6);
    if (uVar3 != 0x0) {
      PostMessage16(0x1008,0x0,0x0,0x11100fc);
    }
  }
  return PTR_LOOP_1050_5df8;
}


void __stdcall16far pass1_1040_8978(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,WNDCLASS16 *param_5)

{
  code **ppcVar1;
  
  unk_win_msg_op_1008_9510((int *)&PTR_LOOP_1050_5df4,0x1008,param_5);
  win_1008_5c5c(param_5,param_3,param_4,_PTR_LOOP_1050_02a0,param_2);
  ppcVar1 = (code **)((int)*param_1 + 0x74);
  (**ppcVar1)(0x1008,param_1);
  return;
}


void __stdcall16far pass1_1040_89a4(ulong *param_1,ushort *param_2,uchar *param_3,int param_4,WNDCLASS16 *param_5)

{
  undefined2 uVar1;
  ushort uVar2;
  code **ppcVar3;
  ushort uVar4;
  ushort uVar5;
  undefined2 uVar6;
  ushort *puVar7;
  
  unk_win_msg_op_1008_9510((int *)&PTR_LOOP_1050_5df4,0x1008,param_5);
  uVar1 = *(undefined2 *)((int)param_2 + 0x2);
  uVar2 = *param_2;
  uVar6 = 0x1010;
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_5,param_3,param_4);
  uVar5 = (ushort)((ulong)puVar7 >> 0x10);
  uVar4 = (ushort)puVar7;
  if (*(int *)(uVar4 + 0x72) != 0x0) {
    uVar6 = 0x1008;
    win_1008_5c7c(_PTR_LOOP_1050_02a0,CONCAT22(uVar1,uVar2),param_5,uVar4,uVar5);
    *(ushort *)((int)param_1 + 0x8c) = uVar4;
  }
  ppcVar3 = (code **)((int)*param_1 + 0x74);
  (**ppcVar3)(uVar6,param_1);
  return;
}



HANDLE16 __stdcall16far
create_window_1040_8bea(UINT32 param_1,undefined2 param_2,int param_3,INT16 param_4,HMENU16 param_5)

{
  HANDLE16 HVar1;
  LPCSTR unaff_CS;
  LRESULT LVar2;
  HWND16 in_stack_0000000e;
  ulong uStack6;
  
  uStack6 = 0x50010000;
  if (param_3 != 0x0) {
    uStack6 = 0x50010001;
  }
  if (*(int *)(param_1 + 0x74) != 0x0) {
    uStack6 = uStack6 | 0x8000000;
  }
  CreateWindow16(unaff_CS,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,param_4,*(INT16 *)(param_1 + 0x6),0x17,0x58,
                 in_stack_0000000e,param_5,(HINSTANCE16)uStack6,(LPVOID)(uStack6 >> 0x10));
  HVar1 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5e09);
  if (HVar1 != 0x0) {
    LVar2 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x1,0x0,CONCAT22(0x30,HVar1));
    HVar1 = (HANDLE16)LVar2;
  }
  return HVar1;
}


void __stdcall16far
mixed_struct_op_1040_8fb8
          (ushort *param_1,ushort param_2,char *param_3,ushort param_4,ushort param_5,ushort param_6,ushort param_7,
          ushort param_8,ushort param_9,LPVOID param_10,ushort param_11)

{
  undefined2 uVar1;
  ushort uVar2;
  LPVOID pvVar3;
  int iVar4;
  undefined2 uVar5;
  astruct_43 *paVar6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar4 + 0x2) = 0x1008;
  *(undefined4 *)(iVar4 + 0x4) = 0x0;
  *(undefined4 *)(iVar4 + 0x8) = 0x0;
  *(undefined4 *)(iVar4 + 0xc) = 0x0;
  *(undefined4 *)(iVar4 + 0x10) = 0x0;
  *(undefined4 *)(iVar4 + 0x14) = 0x0;
  *(undefined2 *)(iVar4 + 0x18) = 0x0;
  *(ushort *)(iVar4 + 0x1a) = param_8;
  *(ushort *)(iVar4 + 0x1c) = param_7;
  *(undefined2 *)(iVar4 + 0x36) = 0x5;
  *(undefined2 *)(iVar4 + 0x38) = 0x0;
  *(undefined2 *)(iVar4 + 0x3a) = 0x0;
  *(undefined2 *)(iVar4 + 0x3c) = 0x2;
  *(undefined2 *)(iVar4 + 0x3e) = 0x0;
  *(ushort *)(iVar4 + 0x40) = param_2;
  *param_1 = 0x9800;
  *(undefined2 *)(iVar4 + 0x2) = (int)&PTR_LOOP_1050_1040;
  uVar1 = *(undefined2 *)(iVar4 + 0x36);
  *(undefined2 *)(iVar4 + 0x28) = uVar1;
  *(undefined2 *)(iVar4 + 0x26) = uVar1;
  *(undefined2 *)(iVar4 + 0x2c) = 0x0;
  *(undefined2 *)(iVar4 + 0x2a) = 0x0;
  if ((param_6 != 0x0) && (param_5 != 0x0)) {
    *(undefined2 *)(iVar4 + 0x38) = 0x1;
    paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_6,param_11);
    *(undefined2 *)(iVar4 + 0x8) = (int)paVar6;
    *(undefined2 *)(iVar4 + 0xa) = (int)((ulong)paVar6 >> 0x10);
    param_10 = (LPVOID)0x1010;
    paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_5,param_11);
    param_9 = (ushort)((ulong)paVar6 >> 0x10);
    *(undefined2 *)(iVar4 + 0xc) = (int)paVar6;
    *(ushort *)(iVar4 + 0xe) = param_9;
    if (param_4 == 0x0) {
      *(undefined4 *)(iVar4 + 0x10) = 0x0;
    }
    else {
      param_10 = (LPVOID)0x1010;
      paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_4,param_11);
      param_9 = (ushort)((ulong)paVar6 >> 0x10);
      *(undefined2 *)(iVar4 + 0x10) = (int)paVar6;
      *(ushort *)(iVar4 + 0x12) = param_9;
    }
  }
  uVar1 = *(undefined2 *)(iVar4 + 0x36);
  *(undefined2 *)(iVar4 + 0x30) = uVar1;
  *(undefined2 *)(iVar4 + 0x2e) = uVar1;
  *(undefined4 *)(iVar4 + 0x32) = 0x0;
  if (param_3 != (char *)0x0) {
    param_10 = (LPVOID)0x1008;
    uVar2 = str_op_1008_60e8(param_3,param_9);
    *(ushort *)(iVar4 + 0x4) = uVar2;
    *(ushort *)(iVar4 + 0x6) = param_9;
  }
  *(undefined4 *)(iVar4 + 0x22) = 0x0;
  *(undefined2 *)(iVar4 + 0x1e) = 0x0;
  *(undefined2 *)(iVar4 + 0x20) = 0x0;
  if (_PTR_LOOP_1050_5e18 == 0x0) {
    pvVar3 = MakeProcInstance16(param_10,(HANDLE16)PTR_LOOP_1050_038c);
    _PTR_LOOP_1050_5e18 = CONCAT22(param_9,pvVar3);
  }
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + 0x1;
  return;
}


void __stdcall16far mix_win_ui_op_1040_911e(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  undefined4 uVar3;
  code **ppcVar4;
  int iVar5;
  undefined2 uVar6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *param_1 = 0x9800;
  *(undefined2 *)(iVar5 + 0x2) = (int)&PTR_LOOP_1050_1040;
  if (*(int *)(iVar5 + 0x38) != 0x0) {
    puVar1 = (undefined4 *)*(uint *)(iVar5 + 0x8);
    uVar2 = *(uint *)(iVar5 + 0xa);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
    puVar1 = (undefined4 *)*(uint *)(iVar5 + 0xc);
    uVar2 = *(uint *)(iVar5 + 0xe);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
    puVar1 = (undefined4 *)*(uint *)(iVar5 + 0x10);
    uVar2 = *(uint *)(iVar5 + 0x12);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar5 + 0x4),0x1000);
  uVar3 = *(undefined4 *)(iVar5 + 0x14);
  SetWindowLong16(0x1000,(INT16)uVar3,CONCAT22(0xfffc,(int)((ulong)uVar3 >> 0x10)));
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5e1c);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisHi_1050_5e23);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procLo_1050_5e2a);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procHi_1050_5e31);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5e38);
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + -0x1;
  if (PTR_LOOP_1050_5e16 == (undefined *)0x0) {
    FreeProcInstance16((LPVOID)s_tile2_bmp_1050_1538);
    _PTR_LOOP_1050_5e18 = 0x0;
  }
  *param_1 = 0x389a;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  return;
}


void __stdcall16far enable_win_1040_9234(ulong param_1,BOOL16 param_2,HWND16 param_3)

{
  if (*(int *)((int)param_1 + 0x18) != 0x0) {
    EnableWindow16(param_3,param_2);
  }
  return;
}


LRESULT __stdcall16far pass1_1040_93e6(ulong param_1,HWND16 param_2)

{
  LRESULT LVar1;
  
  LVar1 = SendMessage16(param_2,0x0,0x0,CONCAT22(0x111,*(undefined2 *)((int)param_1 + 0x1c)));
  return LVar1;
}



LRESULT __stdcall16far send_msg_1040_9404(ulong param_1,HWND16 param_2)

{
  LRESULT LVar1;
  
  LVar1 = SendMessage16(param_2,0x0,0x0,CONCAT22(0x111,*(undefined2 *)((int)param_1 + 0x1c)));
  return LVar1;
}


void __cdecl16far win_ui_get_prop_op_1040_9566(int *param_1,HWND16 param_2)

{
  undefined2 uVar1;
  int iVar2;
  code **ppcVar3;
  HANDLE16 HVar4;
  HANDLE16 HVar5;
  int iVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  undefined4 *puStack12;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*param_1 == 0x4) {
    uVar1 = *(undefined2 *)(iVar6 + 0xc);
    uVar9 = *(undefined2 *)(iVar6 + 0xa);
    HVar4 = GetProp16(param_2,(LPCSTR)s_thisHi_1050_5e6f);
    uVar8 = *(undefined2 *)(iVar6 + 0xa);
    HVar5 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5e68);
    puStack12 = (undefined4 *)CONCAT22(HVar4,HVar5);
    if ((HVar4 | HVar5) != 0x0) {
      iVar2 = *(int *)(iVar6 + 0x6);
      if (iVar2 == 0x1) {
        ppcVar3 = (code **)((int)*puStack12 + 0xc);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,HVar5,HVar4,*(undefined2 *)(iVar6 + 0x8),uVar1,uVar8,uVar9);
        return;
      }
      if (iVar2 == 0x2) {
        ppcVar3 = (code **)((int)*puStack12 + 0x10);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,HVar5,HVar4,*(undefined2 *)(iVar6 + 0x8),uVar1);
        return;
      }
      if (iVar2 == 0x4) {
        ppcVar3 = (code **)((int)*puStack12 + 0x18);
        (**ppcVar3)((int)s_tile2_bmp_1050_1538,HVar5,HVar4,*(byte *)(iVar6 + 0x8) & 0x10,uVar1);
        return;
      }
    }
  }
  return;
}




// WARNING: Unable to use type for symbol var11
// WARNING: Unable to use type for symbol var7
// WARNING: Unable to use type for symbol var8

void __stdcall16far
call_win_proc_1040_9684
          (HWND16 win_handle_1,u16 param_2,WPARAM16 w_param_1,LPARAM l_param_1,HWND16 win_handle_2,u16 param_6)

{
  HANDLE16 handle_1;
  HANDLE16 handle_2;
  BOOL16 bool_1;
  RECT16 local_1a [0x2];
  u32 *var18;
  u32 *var14;
  u32 *var10;
  i32 var6;
  u32 var2;
  u16 var4;
  undefined2 var11;
  undefined2 var7;
  undefined2 var8;
  u16 var9;
  code **fn_ptr_1;
  RECT16 *rect_1;
  undefined4 var3;
  undefined2 var5;
  
  var9 = (u16)&USHORT_1050_1050;
  var8 = l_param_1._2_2_;
  handle_1 = GetProp16(win_handle_2,(LPCSTR)s_procHi_1050_5e7d);
  var7 = l_param_1._2_2_;
  handle_2 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procLo_1050_5e76);
  var6 = CONCAT22(handle_1,handle_2);
  var11 = l_param_1._2_2_;
  handle_1 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisHi_1050_5e8b);
  var5 = l_param_1._2_2_;
  handle_2 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5e84);
  var10 = (u32 *)CONCAT22(handle_1,handle_2);
  if ((handle_1 | handle_2) != 0x0) {
    if ((int)l_param_1 == 0x2) {
      var18 = var10;
      var14 = var10;
      if (var10 != (u32 *)0x0) {
        fn_ptr_1 = (code **)*var10;
        (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,handle_2,handle_1,0x1,var5,var11,var7,var8,var9);
      }
    }
    else {
      if ((int)l_param_1 == 0x201) {
        handle_1 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5e92);
        if (handle_1 == 0x0) {
          var5 = *(undefined2 *)((int)var10 + 0x18);
          GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_1a);
          rect_1 = local_1a;
          var2 = CONCAT22(var5,param_6);
          bool_1 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,
                              (POINT16)CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,win_handle_1)));
          if (bool_1 == 0x0) {
            return;
          }
          debug_print_1008_6048(CONCAT22(param_6,0x5e98),0x1008,param_6);
          fn_ptr_1 = (code **)((int)*var10 + 0x1c);
          (**fn_ptr_1)(0x1008,(int)var10,(int)((ulong)var10 >> 0x10),param_2,win_handle_1,(char)w_param_1,rect_1,var2,
                       l_param_1._2_2_);
          return;
        }
      }
      else {
        if ((int)l_param_1 == 0x204) {
          var4 = *(u16 *)(handle_2 + 0x18);
          GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_1a);
          var3 = CONCAT22(param_6,local_1a);
          bool_1 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,(POINT16)CONCAT22(param_2,win_handle_1));
          if (bool_1 == 0x0) {
            return;
          }
          debug_print_1008_6048(CONCAT22(param_6,0x5eab),0x1008,param_6);
          fn_ptr_1 = (code **)((int)*var10 + 0x20);
          (**fn_ptr_1)(0x1008,(int)var10,(int)((ulong)var10 >> 0x10),param_2,(char)win_handle_1,w_param_1,var3,var4);
          return;
        }
      }
    }
  }
  if (var6 != 0x0) {
    CallWindowProc16((LPVOID)s_tile2_bmp_1050_1538,win_handle_1,param_2,w_param_1,l_param_1);
  }
  return;
}



ushort * __stdcall16far pass1_1040_97da(ushort *param_1,byte param_2)

{
  mix_win_ui_op_1040_911e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



