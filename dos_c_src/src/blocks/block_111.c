


ushort * __stdcall16far unk_win_ui_op_1040_9854(ushort *param_1,ushort param_2)

{
  HCURSOR16 HVar1;
  HGDIOBJ16 HVar2;
  int iVar3;
  uint uVar4;
  
  uVar4 = (uint)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar3 + 0x2) = 0x1008;
  *param_1 = 0xa230;
  *(undefined2 *)(iVar3 + 0x2) = (int)&PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar3 + 0x4)),s_OPButton_1050_5ece);
  *(undefined2 *)(iVar3 + 0x54) = 0x3;
  HVar1 = LoadCursor16(0x1000,(LPCSTR)0x7f00);
  *(HCURSOR16 *)(iVar3 + 0x58) = HVar1;
  HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  *(HGDIOBJ16 *)(iVar3 + 0x56) = HVar2;
  reg_class_1040_98c0((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10,(int)s_tile2_bmp_1050_1538,param_2);
  return param_1;
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



// WARNING: Could not reconcile some variable overlaps

void __cdecl16far draw_op_1040_9948(ushort param_1,ulong param_2,HWND16 param_3,RECT16 *param_4)

{
  int iVar1;
  int iVar2;
  int16_t mode;
  uint uVar3;
  HPEN16 handle;
  HPEN16 handle_00;
  HGDIOBJ16 handle_01;
  undefined *color;
  COLORREF color_00;
  COLORREF color_01;
  astruct_71 *iVar4;
  INT16 y;
  char *x;
  INT16 cx;
  INT16 cy;
  int iStack88;
  int iStack86;
  int iStack84;
  int iStack82;
  int iStack80;
  int iStack78;
  PAINTSTRUCT16 local_42;
  uint uStack34;
  uint uStack32;
  HGDIOBJ16 HStack30;
  int iStack28;
  int iStack26;
  int iStack24;
  int iStack22;
  int iStack20;
  RECT16 local_12;
  undefined4 uStack14;
  int local_a;
  int iStack8;
  int iStack6;
  int iStack4;
  
  iStack6 = 0x1;
  iStack4 = 0x1;
  local_a = 0x0;
  iStack8 = 0x0;
  iStack28 = 0x0;
  HStack30 = 0x0;
  y = (INT16)(param_2 >> 0x10);
  iVar4 = (astruct_71 *)param_2;
  uStack32 = iVar4->field_0x4 & 0x8;
  uStack34 = iVar4->field_0x56 & 0x1;
  BeginPaint16(param_3,&local_42);
  mode = SetMapMode16((HDC16)s_tile2_bmp_1050_1538,0x1);
  GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_12);
  iVar2 = (int)((ulong)uStack14 >> 0x10);
  iVar1 = iVar2 + -0x1;
  uStack14 = CONCAT22(iVar1,(int)uStack14 + -0x1);
  if (uStack34 != 0x0) {
    iStack26 = (int)local_12;
    iStack24 = (int)((ulong)local_12 >> 0x10);
    local_12 = CONCAT22(iStack24 + 0x2,iStack26 + 0x2);
    uStack14 = CONCAT22(iVar2 + -0x3,(int)uStack14 + -0x3);
    iStack22 = (int)uStack14 + -0x1;
    iStack20 = iVar1;
  }
  if (iVar4->field_0x6 != '\0') {
    iStack28 = 0x1;
    if (iVar4->field_0x5a != 0x0) {
      HStack30 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,iVar4->field_0x5a);
    }
    uVar3 = str_op_1000_3da4((char *)(param_2 & 0xffff0000 | ZEXT24(&iVar4->field_0x6)));
    DrawText16(0x1000,(LPCSTR)0x400,(INT16)&local_a,param_4,uVar3);
    iStack8 = ((uStack14._2_2_ - iStack4) + iStack8) / 0x2 + local_12.y;
    iStack4 = iStack4 + iStack8;
    local_a = (((int)uStack14 - iStack6) + local_a) / 0x2 + local_12.x;
    iStack6 = iStack6 + local_a;
  }
  handle = CreatePen16((INT16)s_tile2_bmp_1050_1538,(INT16)DAT_1050_5ec2,(COLORREF)((ulong)DAT_1050_5ec2 >> 0x10));
  handle_00 = CreatePen16((INT16)s_tile2_bmp_1050_1538,(INT16)DAT_1050_5ebe,(COLORREF)((ulong)DAT_1050_5ebe >> 0x10));
  handle_01 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
  if (uStack34 != 0x0) {
    iStack78 = 0x0;
    do {
      MoveTo16((HDC16)s_tile2_bmp_1050_1538,iStack20,iStack26);
      LineTo16((HDC16)s_tile2_bmp_1050_1538,iStack20,iStack22);
      LineTo16((HDC16)s_tile2_bmp_1050_1538,iStack24,iStack22);
      LineTo16((HDC16)s_tile2_bmp_1050_1538,iStack24,iStack26);
      LineTo16((HDC16)s_tile2_bmp_1050_1538,iStack20,iStack26);
      iStack24 = iStack24 + 0x1;
      iStack26 = iStack26 + 0x1;
      iStack22 = iStack22 + -0x1;
      iStack20 = iStack20 + -0x1;
      iStack78 = iStack78 + 0x1;
    } while (iStack78 < 0x1);
  }
  if ((iVar4->field_0x4 & 0x2) == 0x0) {
    iStack84 = (int)((ulong)local_12 >> 0x10);
    iStack82 = (int)uStack14;
    iStack78 = 0x0;
    iStack86 = local_12.x;
    iStack80 = uStack14._2_2_;
    do {
      SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
      MoveTo16((HDC16)s_tile2_bmp_1050_1538,iStack80,iStack86);
      LineTo16((HDC16)s_tile2_bmp_1050_1538,iStack80,iStack82);
      LineTo16((HDC16)s_tile2_bmp_1050_1538,iStack84,iStack82);
      iStack88 = 0x0;
      do {
        SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_00);
        LineTo16((HDC16)s_tile2_bmp_1050_1538,iStack84,iStack86);
        LineTo16((HDC16)s_tile2_bmp_1050_1538,iStack80,iStack86);
        iStack88 = iStack88 + 0x1;
      } while (iStack88 < 0x2);
      iStack84 = iStack84 + 0x1;
      iStack86 = iStack86 + 0x1;
      iStack82 = iStack82 + -0x1;
      iStack80 = iStack80 + -0x1;
      iStack78 = iStack78 + 0x1;
    } while (iStack78 < 0x2);
  }
  else {
    MoveTo16((HDC16)s_tile2_bmp_1050_1538,uStack14._2_2_,local_12.x);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,local_12.y,local_12.x);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,local_12.y,(int)uStack14 + 0x1);
    if (iStack28 != 0x0) {
      iStack8 = iStack8 + 0x2;
      local_a = local_a + 0x2;
      iStack6 = iStack6 + 0x2;
      iStack4 = iStack4 + 0x2;
    }
  }
  MoveTo16((HDC16)s_tile2_bmp_1050_1538,0x0,0x0);
  if (iStack28 != 0x0) {
    if ((iVar4->field_0x4 & 0x4) == 0x0) {
      color = PTR_LOOP_1050_5ec6;
      if (uStack32 != 0x0) {
        color = DAT_1050_5eca;
      }
      color_00 = SetTextColor16((HDC16)s_tile2_bmp_1050_1538,(COLORREF)color);
      color_01 = SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
      uVar3 = str_op_1000_3da4((char *)(param_2 & 0xffff0000 | ZEXT24(&iVar4->field_0x6)));
      DrawText16(0x1000,(LPCSTR)((int)&PTR_LOOP_1050_0000 + 0x1),(INT16)&local_a,param_4,uVar3);
      SetTextColor16((HDC16)s_tile2_bmp_1050_1538,color_00);
      SetBkColor16((HDC16)s_tile2_bmp_1050_1538,color_01);
    }
    else {
      GetStockObject16((INT16)s_tile2_bmp_1050_1538);
      cx = 0x0;
      cy = 0x0;
      x = &iVar4->field_0x6;
      uVar3 = str_op_1000_3da4((char *)(param_2 & 0xffff0000 | ZEXT24(x)));
      GrayString16(0x1000,iStack4 - iStack8,(LPVOID)(iStack6 - local_a),CONCAT22(local_a,iStack8),uVar3,(INT16)x,y,cx,cy
                  );
    }
    if (HStack30 != 0x0) {
      SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack30);
    }
  }
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_01);
  SetMapMode16((HDC16)s_tile2_bmp_1050_1538,mode);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_42);
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



ushort * __stdcall16far pass1_1040_a204(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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



ulong __stdcall16far
pass1_1040_a2cc(int param_1,ulong param_2,ulong param_3,ushort param_4,uchar *param_5,ushort param_6,ushort param_7)

{
  uint uVar1;
  
  if (param_3._2_2_ == 0x1826) {
    if (((int)param_3 == 0x1) || ((0x1 < (int)param_3 - 0x1U && ((int)param_3 - 0x3U < 0x2)))) {
      uVar1 = 0x1;
    }
    else {
      uVar1 = 0x0;
    }
    return (ulong)uVar1;
  }
  pass1_1040_b54a(param_1,(ushort)param_2,(ushort)(param_2 >> 0x10),param_3,param_5,param_6,param_7);
  return CONCAT22(param_5,param_4);
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



void __stdcall16far get_dlg_item_1040_a3d0(ulong param_1,HWND16 param_2)

{
  long lVar1;
  astruct_49 *iVar3;
  int unaff_DI;
  undefined2 uVar2;
  ushort unaff_SS;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_49 *)param_1;
  if (iVar3->field_0x90 != 0x0) {
    lVar1 = iVar3->field_0x90;
    *(undefined2 *)((int)lVar1 + 0x14) = iVar3->field_0x6;
    GetDlgItem16(param_2,0x1826);
    win_msg_1040_a308(param_1,unaff_DI,(HWND16)s_tile2_bmp_1050_1538,unaff_SS);
  }
  return;
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



astruct_18 * __stdcall16far pass1_1040_a4c2(astruct_18 *param_1,byte param_2,ushort param_3)

{
  free_proc_inst_1040_a294(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1040_a564(ulong *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x4) = 0x0;
  *(undefined4 *)((int)param_1 + 0x6) = 0x0;
  return;
}



void __stdcall16far pass1_1040_a582(ulong *param_1)

{
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  return;
}



void __stdcall16far struct_1040_a598(ushort *param_1)

{
  astruct_259 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_259 *)param_1;
  *param_1 = 0x0;
  iVar1->field_0x2 = 0x0;
  iVar1->field_0x6 = 0x0;
  iVar1->field_0xa = 0x0;
  iVar1->field_0xc = 0x0;
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  return;
}



void __stdcall16far pass1_1040_a5d0(ulong param_1)

{
  uint uVar1;
  uint uVar2;
  astruct_258 *iVar4;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_258 *)param_1;
  uVar1 = iVar4->field_0x2;
  uVar2 = iVar4->field_0x4;
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1000_54e8((uchar *)0xa582,(ushort)&PTR_LOOP_1050_1040,*(int *)(uVar1 - 0x2),0xa,uVar1,uVar2);
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(uVar2,uVar1 - 0x2),0x1000);
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar4->field_0xc,0x1000);
  return;
}



void __stdcall16far string_1040_a626(ushort *param_1,char *param_2,ushort param_3)

{
  ushort uVar1;
  
  uVar1 = str_op_1008_60e8(param_2,param_3);
  *param_1 = uVar1;
  *(ushort *)((int)param_1 + 0x2) = param_3;
  return;
}



void __stdcall16far pass1_1040_a640(astruct_57 *param_1,ulong param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_1040_b082(param_1,CONCAT22(param_3,0x1f1));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ulong *)(iVar1 + 0x94) = param_2;
  *(undefined2 *)(iVar1 + 0x98) = 0x0;
  *(undefined2 *)(iVar1 + 0xea) = 0x0;
  *(undefined2 *)param_1 = 0xac08;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far draw_op_1040_a67e(ulong param_1,int param_2,uint param_3,COLORREF param_4)

{
  int *piVar1;
  bool bVar2;
  undefined2 uVar3;
  int iVar4;
  HBRUSH16 HVar5;
  int iVar6;
  undefined2 uVar7;
  COLORREF hdc;
  ulong uVar8;
  int iStack14;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  hdc = param_4;
  if (*(int *)(iVar6 + 0x8e) == 0x0) {
    hdc = (COLORREF)s_tile2_bmp_1050_1538;
    HVar5 = CreateSolidBrush16(param_4);
    *(HBRUSH16 *)(iVar6 + 0x8e) = HVar5;
  }
  if (_PTR_LOOP_1050_5ee8 == 0x0) {
    hdc = 0x1008;
    uVar8 = pass1_1008_4d72(*(ulong *)((int)_PTR_LOOP_1050_4230 + 0xe));
    uVar3 = (undefined2)(uVar8 >> 0x10);
    iVar4 = (int)uVar8;
    _PTR_LOOP_1050_5ee8 =
         (ulong)CONCAT12(*(undefined *)(iVar4 + 0x94),
                         CONCAT11(*(undefined *)(iVar4 + 0x95),*(undefined *)(iVar4 + 0x96)));
    PTR_LOOP_1050_5eec = (undefined *)CONCAT11(*(undefined *)(iVar4 + 0x3e5),*(undefined *)(iVar4 + 0x3e6));
    PTR_LOOP_1050_5eee = (undefined *)(uint)*(byte *)(iVar4 + 0x3e4);
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return;
    }
    bVar2 = false;
    for (iStack14 = 0x0; piVar1 = (int *)(iVar6 + 0xea), *piVar1 != iStack14 && iStack14 <= *piVar1;
        iStack14 = iStack14 + 0x1) {
      if (*(int *)(iVar6 + 0x9a + iStack14 * 0x2) == param_2) {
        bVar2 = true;
        break;
      }
    }
    if (bVar2) {
      PTR_LOOP_1050_5ee8 = PTR_LOOP_1050_5eec;
    }
  }
  SetTextColor16(hdc,(COLORREF)PTR_LOOP_1050_5ee8);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
win_ui_op_1040_a784(int param_1,int param_2,ushort param_3,ulong param_4,ushort param_5,ushort param_6,ushort param_7)

{
  int iVar1;
  
  if (param_4._2_2_ != 0xeb) {
    if (param_4._2_2_ == 0x1f4) {
      msg_box_op_1040_a85a(CONCAT22(param_2,param_1),0x0,param_5,param_7);
      return;
    }
    if (param_4._2_2_ == 0x1f7) {
      _PTR_LOOP_1050_5ef0 = *(undefined4 *)(param_1 + 0x94);
      pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(param_1 + 0x8),0x23,param_5,param_1,(ushort)&PTR_LOOP_1050_1038,
                      param_7);
      PostMessage16((HWND16)&PTR_LOOP_1050_1038,0x0,0x0,0x1110002);
      return;
    }
    if (param_4._2_2_ != 0x17d8) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,(uchar *)param_5,param_6,param_7);
      return;
    }
    iVar1 = *(int *)(param_1 + 0x6);
    SetWindowPos16(param_6,0x6,0xed,0x237,0x0,0x0,0x0);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x17d8);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
    *(undefined2 *)(param_1 + 0x98) = 0x1;
    param_1 = param_2;
    param_2 = iVar1;
  }
  win_ui_dlg_op_1040_a94a(CONCAT22(param_2,param_1),param_7);
  return;
}



void __stdcall16far pass1_1040_a84a(ulong param_1,undefined2 param_2)

{
  win_ui_dlg_op_1040_a94a(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far msg_box_op_1040_a85a(ulong param_1,char *param_2,uchar *param_3,UINT16 param_4)

{
  char local_206 [0x102];
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,param_4);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,
             (short)param_3);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_3,param_2),0x1000);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_dlg_op_1040_a94a(ulong param_1,ushort param_2)

{
  int *piVar1;
  ulong uVar2;
  undefined4 uVar3;
  ushort uVar4;
  undefined *value;
  char *pcVar5;
  uint uVar6;
  uchar *in_DX;
  uchar *puVar8;
  uchar *puVar9;
  uint lp_string;
  int iVar10;
  int iVar11;
  int unaff_DI;
  undefined2 uVar12;
  undefined2 uVar13;
  HWND16 HVar14;
  uchar in_AF;
  bool bVar15;
  ushort *puVar16;
  long lVar17;
  ushort uStack288;
  uint uStack286;
  BOOL16 BStack278;
  int iStack276;
  undefined local_102 [0x100];
  ulong uVar7;
  
  puVar16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_2,in_DX,unaff_DI);
  puVar8 = (uchar *)((ulong)puVar16 >> 0x10);
  uVar4 = (ushort)puVar16;
  uVar12 = (undefined2)(param_1 >> 0x10);
  iVar10 = (int)param_1;
  puVar9 = puVar8;
  pass1_1010_c3c2(uVar4,(ushort)puVar8,CONCAT22(param_2,local_102),*(ulong *)(iVar10 + 0x94),puVar8,in_AF,param_2);
  SetDlgItemText16(0x1010,(INT16)local_102,param_2);
  pass1_1010_c320(uVar4,(ushort)puVar8,(uchar *)CONCAT22(param_2,local_102),*(ulong *)(iVar10 + 0x94),puVar9);
  SetDlgItemText16(0x1010,(INT16)local_102,param_2);
  string_op_1010_c446(param_2,in_AF,puVar9,(ulong)puVar16,CONCAT22(param_2,local_102),*(undefined4 *)(iVar10 + 0x94));
  value = local_102;
  SetDlgItemText16(0x1010,(INT16)value,param_2);
  pass1_1030_6ddc(*(ulong *)(iVar10 + 0x94));
  SetDlgItemInt16(0x1030,0x0,(UINT16)value,0x1f5);
  pass1_1030_6e14(*(ulong *)(iVar10 + 0x94));
  SetDlgItemInt16(0x1030,0x0,(UINT16)value,0x1f6);
  if (*(int *)(iVar10 + 0x98) != 0x0) {
    HVar14 = 0x1010;
    struct_1010_dd5e(uVar4,(ushort)puVar8,*(ulong *)(iVar10 + 0x94));
    if (((uint)puVar9 | (uint)value) != 0x0) {
      uVar3 = *(undefined4 *)(iVar10 + 0x94);
      uVar13 = (undefined2)((ulong)uVar3 >> 0x10);
      iVar11 = (int)uVar3;
      uVar2 = *(ulong *)(iVar11 + 0x26);
      lp_string = *(uint *)(iVar11 + 0x28);
      iStack276 = 0x1798;
      BStack278 = 0x17c3;
      *(undefined2 *)(iVar10 + 0xea) = 0x0;
      uVar7 = uVar2;
      for (uStack288 = 0x1; (int)uStack288 < 0x25; uStack288 = uStack288 + 0x1) {
        if (uVar2 == 0x0) {
          lVar17 = 0x0;
        }
        else {
          HVar14 = 0x1020;
          lVar17 = pass1_1020_bae6((ushort)uVar2,CONCAT22(uStack288,(int)(uVar2 >> 0x10)),(uint)uVar7,lp_string,param_2)
          ;
        }
        uVar6 = (uint)((ulong)lVar17 >> 0x10);
        bVar15 = *(long *)(value + uStack288 * 0x4) != 0x0;
        lp_string = uVar6;
        if (bVar15) {
          pcVar5 = string_1020_c0d8(uStack288);
          SetDlgItemText16(0x1020,(INT16)pcVar5,lp_string);
          HVar14 = (HWND16)s_tile2_bmp_1050_1538;
          SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,*(UINT16 *)(value + uStack288 * 0x4),BStack278);
        }
        uStack286 = (uint)lVar17;
        uVar6 = uVar6 | uStack286;
        if (lVar17 != 0x0) {
          if (!bVar15) {
            pcVar5 = string_1020_c0d8(uStack288);
            HVar14 = (HWND16)s_tile2_bmp_1050_1538;
            SetDlgItemText16(0x1020,(INT16)pcVar5,lp_string);
          }
          SetDlgItemInt16(HVar14,0x0,uStack286,BStack278);
          iVar11 = *(int *)(iVar10 + 0xea);
          HVar14 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,iStack276);
          *(HWND16 *)(iVar10 + iVar11 * 0x2 + 0x9a) = HVar14;
          piVar1 = (int *)(iVar10 + 0xea);
          *piVar1 = *piVar1 + 0x1;
          iVar11 = *(int *)(iVar10 + 0xea);
          HVar14 = (HWND16)s_tile2_bmp_1050_1538;
          uVar6 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,BStack278);
          *(HWND16 *)(iVar10 + iVar11 * 0x2 + 0x9a) = uVar6;
          piVar1 = (int *)(iVar10 + 0xea);
          *piVar1 = *piVar1 + 0x1;
          bVar15 = true;
        }
        uVar7 = (ulong)uVar6;
        if (bVar15) {
          iStack276 = iStack276 + 0x1;
          BStack278 = BStack278 + 0x1;
        }
      }
    }
  }
  return;
}
