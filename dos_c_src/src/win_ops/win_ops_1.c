


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


astruct_57 * __stdcall16far
pass1_1040_8478(astruct_57 *param_1,ushort param_2,char *param_3,char *param_4,ushort param_5,ushort param_6)

{
  ushort uVar1;
  astruct_712 *iVar2;
  undefined2 uVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfc3,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_712 *)param_1;
  iVar2->field_0x8e = 0x0;
  iVar2->field_0x98 = param_2;
  iVar2->field_0x9a = 0x0;
  iVar2->field_0xb2 = 0x0;
  *(undefined2 *)param_1 = 0x8ddc;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  iVar2->field_0x9e = 0x0;
  iVar2->field_0xa2 = 0x12c;
  uVar1 = str_op_1008_60e8(param_4,param_6);
  iVar2->field_0x90 = uVar1;
  iVar2->field_0x92 = param_6;
  uVar1 = str_op_1008_60e8(param_3,param_6);
  iVar2->field_0x94 = uVar1;
  iVar2->field_0x96 = param_6;
  load_icon_1040_8b92((ulong)param_1,0x1008);
  PTR_LOOP_1050_5df8 = (undefined *)0x0;
  return param_1;
}


void __stdcall16far check_dialog_msg_1040_81b6(ulong param_1,HWND16 param_2)

{
  BOOL16 BVar1;
  MSG16 local_14;
  
  *(undefined2 *)((int)param_1 + 0x78) = 0x1;
  while( true ) {
    BVar1 = IsWindow16(param_2);
    if (BVar1 == 0x0) {
      return;
    }
    BVar1 = GetMessage16((MSG16 *)s_tile2_bmp_1050_1538,0x0,0x0,0x0);
    if (BVar1 == 0x0) break;
    param_2 = (HWND16)s_tile2_bmp_1050_1538;
    IsDialogMessage16((HWND16)s_tile2_bmp_1050_1538,&local_14);
  }
  return;
}


void __stdcall16far unk_win_ui_op_1040_8158(ulong *param_1,POINT16 param_2,int param_3,HWND16 param_4)

{
  code **ppcVar1;
  INT16 IVar2;
  BOOL16 BVar3;
  ulong uVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  POINT16 local_6;
  
  if (param_3 == 0x2) {
    uVar6 = (undefined2)((ulong)param_1 >> 0x10);
    iVar5 = (int)param_1;
    if (*(int *)(iVar5 + 0x76) != 0x0) {
      local_6 = param_2;
      uVar6 = *(undefined2 *)(iVar5 + 0x6);
      ScreenToClient16(param_4,&local_6);
      uVar7 = 0x4;
      IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
      local_6 = (POINT16)((ulong)local_6 & 0xffff | (ulong)(uint)(local_6.y + IVar2) << 0x10);
      uVar4 = (ulong)param_1 & 0xffff0000 | (ulong)(iVar5 + 0x82);
      BVar3 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,local_6);
      if (BVar3 != 0x0) {
        ppcVar1 = (code **)((int)*param_1 + 0x14);
        (**ppcVar1)((int)s_tile2_bmp_1050_1538,param_1,0x0,uVar4,uVar7,uVar6);
      }
    }
  }
  return;
}


void __stdcall16far win_help_1040_800c(ulong param_1,ushort param_2)

{
  undefined2 uVar1;
  astruct_43 *paVar2;
  LPCSTR lp_help_file;
  UINT16 w_command;
  undefined2 uVar3;
  
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,param_2);
  uVar1 = (undefined2)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x8a) == 0x0) {
    w_command = 0x0;
    uVar3 = 0x3;
    lp_help_file = (LPCSTR)0x0;
  }
  else {
    uVar3 = 0x1;
    lp_help_file = *(LPCSTR *)((int)param_1 + 0x8a);
    w_command = (int)lp_help_file >> 0xf;
  }
  WinHelp16(0x1010,lp_help_file,w_command,CONCAT22((int)paVar2,uVar3));
  return;
}


void __stdcall16far destroy_win_1040_7b98(ULONG param_1,HWND16 param_2)

{
  if (*(int *)((int)param_1 + 0x74) == 0x0) {
    DestroyWindow16(param_2);
  }
  return;
}


BOOL16 __stdcall16far post_win_msg_1040_7b3c(ulong *param_1,ushort param_2,ushort param_3,int param_4,HWND16 param_5)

{
  code **ppcVar1;
  
  if ((param_4 == 0x1) || (param_4 == 0x2)) {
    ppcVar1 = (code **)((int)*param_1 + 0x14);
    (**ppcVar1)();
  }
  else {
    if (param_4 == 0x6f) {
      ppcVar1 = (code **)((int)*param_1 + 0x2c);
      (**ppcVar1)(param_5,param_1);
    }
    else {
      if (param_4 != 0x12e) {
        return 0x0;
      }
      PostMessage16(param_5,0x0,0x0,0x112f060);
    }
  }
  return 0x1;
}



void __stdcall16far ui_cleanup_op_1040_782c(astruct_18 *param_1,HGDIOBJ16 param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  HGDIOBJ16 menu;
  HMENU16 hwnd;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  param_1->field_0x0 = 0x840c;
  *(undefined2 *)(iVar4 + 0x2) = (int)&PTR_LOOP_1050_1040;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x70);
  uVar2 = *(uint *)(iVar4 + 0x72);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_2,puVar1,uVar2,0x1);
  }
  menu = param_2;
  if (*(int *)(iVar4 + 0x4) != 0x0) {
    menu = (HGDIOBJ16)s_tile2_bmp_1050_1538;
    DeleteObject16(param_2);
    *(undefined2 *)(iVar4 + 0x4) = 0x0;
  }
  hwnd = menu;
  if (*(int *)(iVar4 + 0x68) != 0x0) {
    hwnd = (HMENU16)s_tile2_bmp_1050_1538;
    DestroyMenu16(menu);
  }
  RemoveProp16(hwnd,(LPCSTR)s_thisLo_1050_5db1);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisHi_1050_5db8);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procLo_1050_5dbf);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procHi_1050_5dc6);
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)(iVar4 + 0x2) = 0x1008;
  return;
}


void __stdcall16far
create_window_1040_7620(undefined4 param_1,int param_2,HMENU16 *in_menu_handle,undefined2 param_4,INT16 param_5)

{
  int iVar1;
  undefined2 uVar2;
  HINSTANCE16 h_instance;
  
  load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  _h_instance = 0x50000009;
  if (param_2 != 0x0) {
    _h_instance = 0x50020009;
  }
  uVar2 = (undefined2)((ulong)in_menu_handle >> 0x10);
  iVar1 = (int)in_menu_handle;
  CreateWindow16((LPCSTR)0x1010,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,param_5,*(INT16 *)((int)param_1 + 0x6),
                 *(INT16 *)(iVar1 + 0x6),*(INT16 *)(iVar1 + 0x4),*(HWND16 *)(iVar1 + 0x2),*in_menu_handle,
                 (HINSTANCE16)_h_instance,(LPVOID)((ulong)_h_instance >> 0x10));
  return;
}


void __stdcall16far post_win_msg_1040_7f56(ulong param_1,char *param_2)

{
  unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x10)),param_2);
  PostMessage16(0x1000,0x0,0x0,0x850000);
  return;
}


BOOL16 __stdcall16far post_win_msg_1040_7f1c(ulong param_1,int param_2,HWND16 param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0x78) != 0x0) {
    return 0x0;
  }
  if (*(int *)(iVar1 + 0x60) != param_2) {
    *(int *)(iVar1 + 0x60) = param_2;
    PostMessage16(param_3,0x0,0x0,0x850000);
  }
  return 0x1;
}



void __stdcall16far win_ui_op_1040_6d1a(int param_1,ushort param_2,ushort param_3,ulong param_4)

{
  code **ppcVar1;
  undefined4 uVar2;
  uchar *in_DX;
  ushort unaff_SS;
  int iVar3;
  RECT16 local_a;
  int iStack6;
  int iStack4;
  
  switch(param_4._2_2_) {
  case 0xfa:
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0x94) + 0x18);
    (**ppcVar1)();
    break;
  default:
    pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,(ushort)&PTR_LOOP_1050_1040,unaff_SS);
    return;
  case 0xfd:
    if (DAT_1050_0ecc == 0x0) {
      return;
    }
    DAT_1050_0ecc = 0x0;
    goto LAB_1040_6deb;
  case 0xfe:
    if (DAT_1050_0ecc == 0x1) {
      return;
    }
    DAT_1050_0ecc = 0x1;
    goto LAB_1040_6deb;
  case 0xff:
    if (DAT_1050_0ecc == 0x2) {
      return;
    }
    DAT_1050_0ecc = 0x2;
LAB_1040_6deb:
    uVar2 = *(undefined4 *)(param_1 + 0x94);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0x94) + 0x10);
    (**ppcVar1)((int)&PTR_LOOP_1050_1040,(int)uVar2,(int)((ulong)uVar2 >> 0x10));
    pass1_1010_2ee2(*(ulong **)(param_1 + 0x94),unaff_SS,0x1010);
    PostMessage16(0x1010,0x0,0x0,0x111010a);
    break;
  case 0x107:
    iVar3 = 0x0;
    goto LAB_1040_6e48;
  case 0x108:
    iVar3 = 0x1;
LAB_1040_6e48:
    win_ui_op_1010_3202(*(ulong *)(param_1 + 0x94),iVar3,0x1010);
    break;
  case 0x10a:
    GetClientRect16((HWND16)&PTR_LOOP_1050_1040,&local_a);
    uVar2 = *(undefined4 *)(param_1 + 0x94);
    local_a.y = local_a.y + 0x3;
    local_a.x = *(int *)((int)uVar2 + 0x1a) + -0x9;
    iStack6 = iStack6 + -0x3;
    iStack4 = iStack4 + -0x3;
    InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),(BOOL16)&local_a);
    unk_destroy_win_op_1010_2fa0(*(ULONG *)(param_1 + 0x94),0x1010);
    pass1_1010_32c0(*(ulong *)(param_1 + 0x94),0x0);
    pass1_1010_2ee2(*(ulong **)(param_1 + 0x94),unaff_SS,0x1010);
    break;
  case 0x10c:
    DestroyWindow16((HWND16)&PTR_LOOP_1050_1040);
  }
  return;
}



void __stdcall16far
create_window_1040_6eae(undefined4 param_1,int param_2,HMENU16 *in_menu_handle,undefined2 param_4,INT16 param_5)

{
  int iVar1;
  undefined2 uVar2;
  HINSTANCE16 h_instance;
  
  load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  _h_instance = 0x50000009;
  if (param_2 != 0x0) {
    _h_instance = 0x50020009;
  }
  uVar2 = (undefined2)((ulong)in_menu_handle >> 0x10);
  iVar1 = (int)in_menu_handle;
  CreateWindow16((LPCSTR)0x1010,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,param_5,*(INT16 *)((int)param_1 + 0x6),
                 *(INT16 *)(iVar1 + 0x6),*(INT16 *)(iVar1 + 0x4),*(HWND16 *)(iVar1 + 0x2),*in_menu_handle,
                 (HINSTANCE16)_h_instance,(LPVOID)((ulong)_h_instance >> 0x10));
  return;
}



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


void __stdcall16far pass1_1040_3a0e(ushort param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  ushort *puVar1;
  int iVar2;
  
  iVar2 = 0x0;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  pass1_1010_038e((ulong)puVar1,iVar2,param_5);
  destroy_win_1040_7b98(CONCAT22(param_2,param_1),0x1010);
  return;
}


ushort __stdcall16far enable_win_1040_3a36(ulong param_1,ushort param_2,ushort param_3,int param_4,HWND16 param_5)

{
  int *piVar1;
  bool bVar2;
  int iVar3;
  undefined2 uVar4;
  HWND16 hwnd;
  HWND16 hwnd_00;
  
  bVar2 = false;
  iVar3 = (int)param_1;
  uVar4 = (undefined2)(param_1 >> 0x10);
  if (param_4 == 0x0) {
    if (*(uint *)(iVar3 + 0x9e) <= *(uint *)(iVar3 + 0x9c)) goto LAB_1040_3a79;
    piVar1 = (int *)(iVar3 + 0x9c);
    *piVar1 = *piVar1 + 0x1;
  }
  else {
    if (param_4 != 0x1) goto LAB_1040_3a79;
    if (*(int *)(iVar3 + 0x9c) == 0x0) goto LAB_1040_3a79;
    piVar1 = (int *)(iVar3 + 0x9c);
    *piVar1 = *piVar1 + -0x1;
  }
  bVar2 = true;
LAB_1040_3a79:
  hwnd = param_5;
  if (bVar2) {
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    SetDlgItemInt16(param_5,0x0,*(UINT16 *)(iVar3 + 0x9c),0x18e);
  }
  hwnd_00 = hwnd;
  if ((*(int *)(iVar3 + 0x9c) != 0x0) && (*(int *)(iVar3 + 0xa2) == 0x0)) {
    *(undefined2 *)(iVar3 + 0xa2) = 0x1;
    hwnd_00 = (HWND16)s_tile2_bmp_1050_1538;
    EnableWindow16(hwnd,0x1);
  }
  if ((*(int *)(iVar3 + 0x9c) == 0x0) && (*(int *)(iVar3 + 0xa2) != 0x0)) {
    *(undefined2 *)(iVar3 + 0xa2) = 0x0;
    EnableWindow16(hwnd_00,0x0);
  }
  return 0x0;
}



void __stdcall16far
send_dlg_item_msg_1040_3f12(ushort param_1,ushort param_2,ulong param_3,HWND16 param_4,ushort param_5)

{
  undefined4 uVar1;
  undefined *puVar2;
  uint extraout_DX;
  int iVar3;
  HWND16 hwnd;
  LRESULT LVar4;
  undefined local_a [0x8];
  
  SendDlgItemMessage16(param_4,0x0,0x0,0x0,0x190000b);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1900405);
  pass1_1008_5784((ulong *)CONCAT22(param_5,local_a),param_3);
  while( true ) {
    puVar2 = local_a;
    hwnd = 0x1008;
    pass1_1008_5b12(puVar2,param_5);
    if ((extraout_DX | (uint)puVar2) == 0x0) break;
    uVar1 = *(undefined4 *)(puVar2 + 0x4);
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    LVar4 = SendDlgItemMessage16(0x1008,(INT16)uVar1,(UINT16)((ulong)uVar1 >> 0x10),0x0,0x1900401);
    iVar3 = (int)((ulong)LVar4 >> 0x10);
    if ((((int)LVar4 == -0x1) && (iVar3 == -0x1)) || (((int)LVar4 == -0x2 && (iVar3 == -0x1)))) break;
  }
  SendDlgItemMessage16(hwnd,0x0,0x0,0x0,0x1900407);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x190000b);
  return;
}



void __stdcall16far get_win_rect_1040_43ea(int param_1,HWND16 param_2,ushort param_3,ushort param_4)

{
  undefined4 uVar1;
  RECT16 local_a;
  int iStack6;
  int iStack4;
  
  GetWindowRect16(param_2,&local_a);
  iStack6 = iStack6 - local_a.x;
  iStack4 = iStack4 - local_a.y;
  pass1_1010_5fb0(*(ulong *)(param_1 + 0x8e),0x0,(ulong *)&local_a,param_3,0x7);
  uVar1 = *(undefined4 *)(param_1 + 0x8e);
  *(uint *)((int)uVar1 + 0x7a) = (uint)(*(int *)(param_1 + 0x9a) == 0x0);
  return;
}


LRESULT __stdcall16far send_win_msg_1040_4a0a(astruct_48 **param_1,HWND16 param_2)

{
  int *piVar1;
  code **ppcVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  ushort uVar5;
  astruct_48 *iVar5;
  undefined2 uVar6;
  LRESULT LVar7;
  char *pcVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  int iStack10;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_48 *)param_1;
  ppcVar2 = (code **)((int)*param_1 + 0x74);
  (**ppcVar2)(param_2,param_1,0x5d6a,(int)&USHORT_1050_1050);
  GetDlgItem16(param_2,0x1770);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x40b0000);
  LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0000);
  uVar5 = (ushort)((ulong)LVar7 >> 0x10);
  for (iStack10 = 0x0; uVar3 = iVar5->field_0x90, piVar1 = (int *)((int)uVar3 + 0x10),
      *piVar1 != iStack10 && iStack10 <= *piVar1; iStack10 = iStack10 + 0x1) {
    uVar10 = 0x0;
    uVar9 = 0x403;
    uVar3 = iVar5->field_0x90;
    uVar3 = *(undefined4 *)((int)uVar3 + 0xc);
    pcVar8 = pass1_1040_4dcc((ulong)param_1,*(int *)((int)uVar3 + iStack10 * 0x2),uVar5);
    LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)pcVar8,(WPARAM16)((ulong)pcVar8 >> 0x10),
                          CONCAT22(uVar9,uVar10));
    uVar5 = (ushort)((ulong)LVar7 >> 0x10);
  }
  pass1_1040_4d7e((ulong)param_1);
  if (iStack10 == 0x0) {
    uVar10 = 0x40a;
    uVar4 = iVar5->field_0x90;
    uVar3 = iVar5->field_0x94;
    pcVar8 = string_op_1010_ada6(0x1010,uVar5,(ushort)uVar3,(ushort)((ulong)uVar3 >> 0x10),0x0,
                                 *(int *)((int)uVar4 + 0xa));
    SendMessage16(0x1010,(UINT16)pcVar8,(WPARAM16)((ulong)pcVar8 >> 0x10),CONCAT22(uVar10,iStack10));
  }
  LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0001);
  return LVar7;
}



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





