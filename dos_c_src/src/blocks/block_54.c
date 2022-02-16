
astruct_18 * __stdcall16far pass1_1020_6208(astruct_18 *param_1,byte param_2,ushort param_3)

{
  param_1 = (astruct_18 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xe2));
  destroy_cursor_1020_42f4((ushort *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_62e0(int param_1,ushort param_2,ushort param_3,ushort param_4)

{
  undefined4 *puVar1;
  code **ppcVar2;
  uint *puVar3;
  undefined4 uVar4;
  uchar *extraout_DX;
  uchar *puVar5;
  uchar *puVar6;
  undefined2 uVar7;
  uchar *extraout_DX_00;
  int unaff_DI;
  ushort *puVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  int iVar11;
  ushort uVar12;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)(param_1 + 0x14) = 0x0;
  *(undefined4 *)(param_1 + 0x2c) = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x67c2;
  *(undefined2 *)(param_1 + 0x2) = 0x1020;
  puVar6 = extraout_DX;
  puVar3 = pass1_1000_4906((astruct_20 *)CONCAT22(param_2,param_1 + 0x18),(WNDCLASS16 *)0x0,0x14);
  mem_op_1000_179c(0x3c,puVar6,0x1000);
  puVar5 = (uchar *)((uint)puVar6 | (uint)puVar3);
  if (puVar5 == (uchar *)0x0) {
    *(undefined4 *)(param_1 + 0x1c) = 0x0;
  }
  else {
    pass1_1020_87c2((ushort *)CONCAT22(puVar6,puVar3),param_4,unaff_DI);
    *(uint **)(param_1 + 0x1c) = puVar3;
    *(uchar **)(param_1 + 0x1e) = puVar5;
  }
  mem_op_1000_179c(0x26,puVar5,0x1000);
  puVar6 = (uchar *)((uint)puVar5 | (uint)puVar3);
  if (puVar6 == (uchar *)0x0) {
    puVar3 = (uint *)0x0;
    puVar6 = (uchar *)0x0;
  }
  else {
    pass1_1020_8a9c((ushort *)CONCAT22(puVar5,puVar3));
  }
  *(uint **)(param_1 + 0x20) = puVar3;
  *(uchar **)(param_1 + 0x22) = puVar6;
  mem_op_1000_179c(0xbe,puVar6,0x1000);
  puVar5 = (uchar *)((uint)puVar6 | (uint)puVar3);
  if (puVar5 == (uchar *)0x0) {
    puVar3 = (uint *)0x0;
    puVar5 = (uchar *)0x0;
  }
  else {
    pass1_1020_8eaa((ushort *)CONCAT22(puVar6,puVar3),param_4);
  }
  *(uint **)(param_1 + 0x24) = puVar3;
  *(uchar **)(param_1 + 0x26) = puVar5;
  mem_op_1000_179c(0x20,puVar5,0x1000);
  puVar6 = (uchar *)((uint)puVar5 | (uint)puVar3);
  if (puVar6 == (uchar *)0x0) {
    puVar3 = (uint *)0x0;
    puVar6 = (uchar *)0x0;
  }
  else {
    pass1_1020_8360((ushort *)CONCAT22(puVar5,puVar3),param_4);
  }
  *(uint **)(param_1 + 0x28) = puVar3;
  *(uchar **)(param_1 + 0x2a) = puVar6;
  pass1_1020_6746(CONCAT22(param_2,param_1),0x1,0x4);
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_4,puVar6,unaff_DI);
  uVar7 = (undefined2)((ulong)puVar8 >> 0x10);
  *(undefined2 *)(param_1 + 0x14) = (int)puVar8;
  *(undefined2 *)(param_1 + 0x16) = uVar7;
  uVar10 = 0x0;
  uVar9 = *(undefined2 *)(param_1 + 0x14);
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0x14) + 0x4);
  iVar11 = param_1;
  uVar12 = param_2;
  (**ppcVar2)();
  *(undefined4 *)(param_1 + 0x6) = *(undefined4 *)(param_1 + 0x14);
  uVar4 = *(undefined4 *)(param_1 + 0x14);
  puVar1 = (undefined4 *)*(undefined4 *)((int)uVar4 + 0xa);
  uVar4 = CONCAT22(param_2,param_1 + 0xa);
  ppcVar2 = (code **)((int)*puVar1 + 0x8);
  (**ppcVar2)(0x1010,(int)puVar1,(int)((ulong)puVar1 >> 0x10),uVar4,uVar9,uVar7,uVar10,iVar11,uVar12);
  *(undefined2 *)(param_1 + 0x12) = (int)uVar4;
  *(undefined2 *)(param_1 + 0x10) = 0x1;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,extraout_DX_00,unaff_DI);
  *(undefined2 *)(param_1 + 0x2c) = (int)puVar8;
  *(undefined2 *)(param_1 + 0x2e) = (int)((ulong)puVar8 >> 0x10);
  return;
}



void __stdcall16far pass1_1020_6466(ushort *param_1,ushort param_2,ushort param_3)

{
  astruct_585 *iVar1;
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_585 *)param_1;
  *param_1 = 0x67c2;
  iVar1->field_0x2 = 0x1020;
  if (iVar1->field_0x14 != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1ea6(iVar1->field_0x14,(ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10,param_3);
  }
  palette_op_1020_92c4(param_1,param_2);
  return;
}



ulong __stdcall16far pass1_1020_6498(ulong param_1,int param_2)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x18 + param_2 * 0x4) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x18 + param_2 * 0x4);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22(*(undefined2 *)(iVar2 + 0xa),*(undefined2 *)(iVar2 + 0x8));
  }
  return 0x0;
}



ushort __stdcall16far pass1_1020_64d4(ulong param_1,int param_2)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x18 + param_2 * 0x4) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x18 + param_2 * 0x4);
    return *(ushort *)((int)uVar1 + 0x4);
  }
  return 0x0;
}



void __stdcall16far mix_draw_op_1020_650c(astruct_7 *param_1,HWND16 param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  PAINTSTRUCT16 local_28;
  int iStack8;
  undefined4 *puStack6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  uVar2 = *(undefined4 *)(iVar3 + 0x14);
  puStack6 = (undefined4 *)*(undefined4 *)((int)uVar2 + 0xa);
  if ((*(int *)(iVar3 + 0x10) != 0x0) || (uVar2 = *(undefined4 *)(iVar3 + 0x14), *(int *)((int)uVar2 + 0x24) != 0x0)) {
    draw_op_1020_9364(param_1,param_2,param_3);
    if (*(long *)(iVar3 + 0x24) != 0x0) {
      uVar2 = *(undefined4 *)(iVar3 + 0x24);
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x24) + 0x14);
      (**ppcVar1)(param_2,(int)uVar2,(int)((ulong)uVar2 >> 0x10));
    }
  }
  iStack8 = 0x0;
  do {
    iVar4 = iVar3 + 0x18;
    iVar5 = iStack8 * 0x4;
    if (*(long *)(iVar4 + iVar5) != 0x0) {
      uVar2 = *(undefined4 *)(iVar4 + iVar5);
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + iVar5) + 0x8);
      (**ppcVar1)(param_2,(char)uVar2,(int)((ulong)uVar2 >> 0x10),(int)puStack6,(int)((ulong)puStack6 >> 0x10));
    }
    iStack8 = iStack8 + 0x1;
  } while (iStack8 < 0x5);
  uVar7 = *(undefined2 *)(iVar3 + 0x4);
  BeginPaint16(param_2,&local_28);
  ppcVar1 = (code **)((int)*puStack6 + 0x4);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puStack6,(int)((ulong)puStack6 >> 0x10),0x0,0x0,iVar3 + 0xa,uVar6,uVar7);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_28);
  return;
}



void __stdcall16far unk_win_op_1020_65cc(astruct_60 *param_1,int param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  BOOL16 BVar3;
  ushort uVar4;
  astruct_59 *iVar4;
  astruct_60 *iVar5;
  int iVar6;
  astruct_60 *uVar7;
  HWND16 hwnd;
  int iStack4;
  
  iVar5 = (astruct_60 *)param_1;
  uVar7 = (astruct_60 *)((ulong)param_1 >> 0x10);
  if (param_2 == 0x1) {
    iVar5->field_0x14 = 0x0;
    return;
  }
  if (param_2 == 0x2) {
    for (iStack4 = 0x0; iStack4 < 0x5; iStack4 = iStack4 + 0x1) {
      iVar4 = (astruct_59 *)&iVar5->field_0x18;
      iVar6 = iStack4 * 0x4;
      if ((*(uint *)(iVar4 + iVar6 + 0x2) | *(uint *)(iVar4 + iVar6)) != 0x0) {
        ppcVar1 = (code **)((int)**(undefined4 **)(iVar4 + iVar6) + 0x4);
        (**ppcVar1)(param_3,*(undefined4 *)(iVar4 + iVar6));
      }
    }
  }
  else {
    if (((0x0 < param_2 + -0x3) && (!SBORROW2(param_2 + -0x3,0x1))) && (param_2 + -0x4 < 0x4)) {
      BVar3 = IsIconic16(param_3);
      if (BVar3 == 0x0) {
        BVar3 = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
        if ((BVar3 == 0x0) && (uVar2 = iVar5->field_0x14, *(int *)((int)uVar2 + 0x24) != 0x0)) {
          InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)0x0,0x0);
          uVar4 = pass1_1020_64d4((ulong)param_1,0x2);
          if (uVar4 == 0x0) {
            pass1_1020_6746((ulong)param_1,0x1,0x2);
          }
          uVar4 = pass1_1020_64d4((ulong)param_1,0x3);
          if (uVar4 == 0x0) {
            pass1_1020_6746((ulong)param_1,0x1,0x3);
          }
          hwnd = 0x1018;
          uVar4 = pass1_1018_255e(iVar5->field_0x14);
          if (uVar4 == 0x0) {
            hwnd = (HWND16)s_tile2_bmp_1050_1538;
            SendMessage16(0x1018,0x0,0x0,0x1110069);
          }
          else {
            uVar4 = pass1_1020_64d4((ulong)param_1,0x1);
            if (uVar4 == 0x0) {
              pass1_1020_6746((ulong)param_1,0x1,0x1);
            }
          }
          SendMessage16(hwnd,0x0,0x0,0x11100f0);
          uVar2 = iVar5->field_0x2c;
          if (*(int *)((int)uVar2 + 0x7a) != 0x0) {
            uVar2 = iVar5->field_0x2c;
            *(undefined2 *)((int)uVar2 + 0x7a) = 0x0;
            SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1110131);
            return;
          }
        }
      }
    }
  }
  return;
}



void __stdcall16far pass1_1020_6746(ulong param_1,int param_2,int param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  
  if (param_3 != 0x0) {
    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    if (*(long *)(iVar3 + 0x18 + param_3 * 0x4) != 0x0) {
      uVar2 = *(undefined4 *)(iVar3 + 0x18 + param_3 * 0x4);
      *(int *)((int)uVar2 + 0x4) = param_2;
      *(undefined2 *)(iVar3 + 0x10) = 0x1;
      if (param_2 == 0x0) {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x18 + param_3 * 0x4) + 0x14);
        (**ppcVar1)();
      }
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_679c(astruct_18 *param_1,byte param_2,ushort param_3,ushort param_4)

{
  pass1_1020_6466(&param_1->field_0x0,param_3,param_4);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far unk_win_ui_op_1020_67ce(astruct_20 *in_struct_1,UINT16 param_2,ULONG param_3)

{
  HGDIOBJ16 HVar1;
  HCURSOR16 HVar2;
  astruct_20 *iVar3;
  astruct_20 *uVar4;
  UINT16 unaff_SS;
  
  struct_1020_790e(&in_struct_1->field_0x0,(ulong)s_TPPOPMENU_1050_43fa,param_2,param_3,unaff_SS);
  uVar4 = (astruct_20 *)((ulong)in_struct_1 >> 0x10);
  iVar3 = (astruct_20 *)in_struct_1;
  iVar3[0x1].field_0x10 = 0x0;
  *(undefined4 *)&iVar3[0x1].field_0x14 = 0x0;
  in_struct_1->field_0x0 = 0x70e6;
  iVar3->field_0x2 = 0x1020;
  unk_str_op_1000_3d3e((char *)((ulong)in_struct_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x5b)),s_VrMode2_1050_4404);
  HVar1 = GetStockObject16(0x1000);
  iVar3->hgdiobj_field_0xc6 = HVar1;
  HVar2 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)0x7f00);
  iVar3->hcursor_field_0xc4 = HVar2;
  iVar3->field_0xac = 0x44c00000;
  iVar3->field_0xc8 = 0x2020;
  iVar3->field_0xbc = *(UINT16 *)((int)param_3 + 0x8);
  iVar3->field_0xca = param_2;
  win_ui_reg_class_1008_96d2(in_struct_1,0x1008,unaff_SS);
  window_op_1020_6c3a(in_struct_1);
  return;
}



void __stdcall16far pass1_1020_687c(ulong param_1,ushort param_2,ushort param_3)

{
  uchar uVar1;
  
  uVar1 = (uchar)param_2;
  get_win_ui_info_op_1020_7a50(param_1,param_3);
  destroy_icon_1020_6bd2(param_1,uVar1,param_3);
  return;
}



void __stdcall16far realize_palette_1020_6896(ulong param_1,int param_2,HGDIOBJ16 param_3)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined4 uVar3;
  int iVar4;
  undefined2 uVar5;
  
  if (param_2 != 0x0) {
    uVar3 = *(undefined4 *)((int)param_1 + 0xf2);
    uVar5 = (undefined2)((ulong)uVar3 >> 0x10);
    iVar4 = (int)uVar3;
    puVar1 = (undefined4 *)*(undefined4 *)(iVar4 + 0x24);
    ppcVar2 = (code **)((int)*puVar1 + 0x18);
    (**ppcVar2)(param_3,(int)puVar1,*(undefined2 *)(iVar4 + 0x26));
    UnrealizeObject16(param_3);
    RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
  }
  return;
}



void __stdcall16far pass1_1020_68de(ulong param_1,ushort param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0xf6) != 0x0) {
    invalidate_rect_1020_735a(*(ulong *)((int)param_1 + 0xf6),param_2);
  }
  return;
}



void __stdcall16far pt_in_rect_1020_68fc(ulong *param_1,undefined2 param_2,ushort param_3)

{
  code **ppcVar1;
  ushort uVar2;
  BOOL16 BVar3;
  ulong uVar4;
  undefined2 uVar5;
  POINT16 PStack6;
  
  PStack6 = (POINT16)CONCAT22(param_2,param_3);
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  uVar2 = pass1_1018_31d0(*(ulong *)((int)param_1 + 0xf2));
  if (uVar2 != 0x0) {
    uVar4 = *(ulong *)((int)param_1 + 0xf2);
    uVar4 = uVar4 & 0xffff0000 | (ulong)((int)uVar4 + 0x16c);
    BVar3 = PtInRect16((RECT16 *)0x1018,PStack6);
    if (BVar3 != 0x0) {
      ppcVar1 = (code **)((int)*param_1 + 0x40);
      (**ppcVar1)((int)s_tile2_bmp_1050_1538,param_1,0xef,uVar4);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uint __stdcall16far unk_destroy_win_op_1020_694c(ULONG param_1,uint param_2,HWND16 param_3,ushort param_4)

{
  undefined4 uVar1;
  uint uVar2;
  HWND16 HVar3;
  int iVar4;
  astruct_43 *paVar5;
  undefined2 uVar6;
  
  uVar2 = param_2;
  if (param_2 != 0x12b) {
    iVar4 = (int)param_1;
    uVar6 = (undefined2)(param_1 >> 0x10);
    if (param_2 < 0x12c) {
      if (param_2 == 0x6f) {
        paVar5 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,param_4);
        uVar2 = WinHelp16(0x1010,(LPCSTR)((int)s_New_failed_in_Op__Op_1050_0020 + 0x9),0x0,CONCAT22((int)paVar5,0x1));
        return uVar2;
      }
      if (param_2 == 0xeb) {
        uVar2 = GetDlgItem16(param_3,0x1797);
        if (uVar2 != 0x0) {
LAB_1020_6a6f:
          win_ui_fn_1020_6e98(param_1,(HWND16)s_tile2_bmp_1050_1538,param_4);
          return uVar2;
        }
      }
      else {
        uVar2 = param_2 - 0xef;
        if (uVar2 == 0x0) {
          pass1_1018_2e28(*(ulong *)(iVar4 + 0xf2));
          pass1_1008_3e0e(param_1);
        }
        else {
          uVar2 = param_2 - 0x129;
          if ((uVar2 != 0x0) && (uVar2 = param_2 - 0x12a, uVar2 == 0x0)) {
            uVar6 = 0xf012;
LAB_1020_69c3:
            uVar2 = PostMessage16(param_3,0x0,0x0,CONCAT22(0x112,uVar6));
            return uVar2;
          }
        }
      }
    }
    else {
      if (param_2 == 0xbb8) {
        HVar3 = GetDlgItem16(param_3,0x1797);
        if (HVar3 != 0x0) {
          DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
        }
        uVar2 = pass1_1018_31d0(*(ulong *)(iVar4 + 0xf2));
        if (uVar2 != 0x0) {
          uVar1 = *(undefined4 *)(iVar4 + 0xf2);
          uVar2 = pass1_1018_2d9a((int)uVar1,(int)((ulong)uVar1 >> 0x10));
LAB_1020_6a0b:
          invalidate_rect_1020_735a(*(ulong *)(iVar4 + 0xf6),0x1018);
          return uVar2;
        }
      }
      else {
        if (param_2 < 0xbb9) {
          if (param_2 == 0x12c) {
            uVar6 = 0xf020;
            goto LAB_1020_69c3;
          }
          uVar2 = param_2 - 0x12d;
          if (param_2 != 0x12c) {
            uVar2 = param_2 - 0x12e;
          }
        }
        else {
          if (param_2 == 0xbb9) {
            HVar3 = GetDlgItem16(param_3,0x1797);
            if (HVar3 != 0x0) {
              DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
            }
            uVar2 = pass1_1018_31d0(*(ulong *)(iVar4 + 0xf2));
            if (uVar2 != 0x0) {
              uVar1 = *(undefined4 *)(iVar4 + 0xf2);
              uVar2 = pass1_1018_2dde((int)uVar1,(int)((ulong)uVar1 >> 0x10));
              goto LAB_1020_6a0b;
            }
          }
          else {
            uVar2 = param_2 - 0xbba;
            if (uVar2 == 0x0) {
              uVar2 = GetDlgItem16(param_3,0x1797);
              if (uVar2 != 0x0) {
                uVar2 = DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
                return uVar2;
              }
              goto LAB_1020_6a6f;
            }
          }
        }
      }
    }
  }
  return uVar2;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far
win_ui_op_1020_6ae6(ulong *param_1,ushort param_2,int param_3,int param_4,HWND16 param_5,WPARAM16 param_6)

{
  code **ppcVar1;
  undefined *puVar2;
  int iVar3;
  undefined2 uVar4;
  HWND16 hwnd;
  LRESULT LVar5;
  uint16_t in_stack_0000ff86;
  uint16_t in_stack_0000ff88;
  HWND16 HVar6;
  undefined local_58 [0x50];
  undefined4 uStack8;
  HWND16 HStack4;
  
  if (param_4 == 0x1797) {
    uVar4 = (undefined2)((ulong)param_1 >> 0x10);
    iVar3 = (int)param_1;
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    HStack4 = GetDlgItem16(param_5,0x1797);
    if (HStack4 != 0x0) {
      if (param_3 == 0x2) {
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
        uStack8 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4090000);
        if (uStack8 != -0x1) {
          HVar6 = HStack4;
          LVar5 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_58,param_6,CONCAT22(0x40a,(int)uStack8));
          puVar2 = local_58;
          pass1_1018_30ca(*(ulong *)(iVar3 + 0xf2),(char *)CONCAT22(param_6,puVar2),(ushort)((ulong)LVar5 >> 0x10));
          hwnd = 0x1018;
          pass1_1018_2fe8(*(ulong *)(iVar3 + 0xf2),in_stack_0000ff86,in_stack_0000ff88);
          if (puVar2 != (undefined *)0x0) {
            invalidate_rect_1020_735a(*(ulong *)(iVar3 + 0xf6),0x1018);
            ppcVar1 = (code **)((int)*param_1 + 0x40);
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



void __stdcall16far enable_menu_item_1020_6b9a(HMENU16 param_1,int param_2)

{
  if (param_2 != 0x0) {
    return;
  }
  EnableMenuItem16(param_1,0x400,0x0);
  return;
}



void __stdcall16far pass1_1020_6bbc(ulong param_1,ushort param_2,ushort param_3)

{
  win_ui_op_1020_737a(*(ULONG *)((int)param_1 + 0xf6),param_2,param_3);
  return;
}



void __stdcall16far destroy_icon_1020_6bd2(ulong param_1,uchar param_2,HICON16 param_3)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar6 = *(undefined2 *)(iVar4 + 0xc2);
  DestroyIcon16(param_3);
  *(undefined2 *)(iVar4 + 0xc2) = 0x0;
  *(undefined2 *)(iVar4 + 0x8) = 0x0;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xf6);
  uVar2 = *(uint *)(iVar4 + 0xf8);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)((int)s_tile2_bmp_1050_1538,puVar1,uVar2,0x1,uVar6);
  }
  *(undefined4 *)(iVar4 + 0xf6) = 0x0;
  pass1_1010_1dda(*(ulong *)(iVar4 + 0xf2));
  *(undefined4 *)(iVar4 + 0xf2) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far window_op_1020_6c3a(astruct *param_1)

{
  undefined4 uVar1;
  code **ppcVar2;
  HICON16 HVar3;
  astruct_160 *paVar4;
  BOOL16 *pBVar5;
  undefined4 uVar6;
  uchar *in_DX;
  undefined2 uVar7;
  uchar *extraout_DX;
  uchar *puVar8;
  uchar *puVar9;
  uint uVar10;
  undefined2 extraout_DX_00;
  int iVar11;
  int unaff_DI;
  undefined2 uVar12;
  ushort unaff_SS;
  ushort *puVar13;
  undefined *puVar14;
  ulong local_6;
  
  create_window_ex_1008_9760(param_1,0x1008);
  puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x4,unaff_SS,in_DX,unaff_DI);
  uVar7 = (undefined2)((ulong)puVar13 >> 0x10);
  uVar12 = (undefined2)((ulong)param_1 >> 0x10);
  iVar11 = (int)param_1;
  *(undefined2 *)(iVar11 + 0xf2) = (int)puVar13;
  *(undefined2 *)(iVar11 + 0xf4) = uVar7;
  *(undefined2 *)(iVar11 + 0xe0) = *(undefined2 *)(iVar11 + 0xf2);
  *(undefined2 *)(iVar11 + 0xe2) = uVar7;
  puVar14 = PTR_LOOP_1050_038c;
  HVar3 = LoadIcon16(0x1010,(LPCSTR)s_TILEICON_1050_440c);
  *(HICON16 *)(iVar11 + 0xc2) = HVar3;
  uVar6 = *(undefined4 *)(iVar11 + 0xf2);
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar11 + 0xf2) + 0x30);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)uVar6,(int)((ulong)uVar6 >> 0x10),HVar3,puVar14);
  paVar4 = (astruct_160 *)((int)&local_6 + 0x2);
  puVar9 = extraout_DX;
  pass1_1018_2d22(*(ulong *)(iVar11 + 0xf2),(int *)CONCAT22(unaff_SS,&local_6),
                  (ushort *)CONCAT13((char)(unaff_SS >> 0x8),CONCAT12((char)unaff_SS,paVar4)),0xbb8);
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (uchar *)((uint)puVar9 | (uint)paVar4);
  if (puVar8 != (uchar *)0x0) {
    uVar7 = *(undefined2 *)(iVar11 + 0x8);
    pass1_1008_3bd6(paVar4,(ushort)puVar9,0x0,local_6,0x0,0x7c007d,
                    CONCAT13((char)((uint)uVar7 >> 0x8),CONCAT12((char)uVar7,0xbb8)),(ushort)puVar8,unaff_SS);
  }
  paVar4 = (astruct_160 *)((int)&local_6 + 0x2);
  pass1_1018_2d22(*(ulong *)(iVar11 + 0xf2),(int *)CONCAT22(unaff_SS,&local_6),(ushort *)CONCAT22(unaff_SS,paVar4),0xbb9
                 );
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (uchar *)((uint)puVar8 | (uint)paVar4);
  if (puVar9 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar4,(ushort)puVar8,0x0,local_6,0x0,0x7e007f,CONCAT22(*(undefined2 *)(iVar11 + 0x8),0xbb9),
                    (ushort)puVar9,unaff_SS);
  }
  paVar4 = (astruct_160 *)((int)&local_6 + 0x2);
  pass1_1018_2d22(*(ulong *)(iVar11 + 0xf2),(int *)CONCAT22(unaff_SS,&local_6),(ushort *)CONCAT22(unaff_SS,paVar4),0xbba
                 );
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (uchar *)((uint)puVar9 | (uint)paVar4);
  if (puVar8 != (uchar *)0x0) {
    pass1_1008_3bd6(paVar4,(ushort)puVar9,0x0,local_6,0x1b2,0x1b001b1,CONCAT22(*(undefined2 *)(iVar11 + 0x8),0xbba),
                    (ushort)puVar8,unaff_SS);
  }
  mem_op_1000_179c(0x22,puVar8,0x1000);
  uVar10 = (uint)puVar8 | (uint)paVar4;
  if (uVar10 == 0x0) {
    *(undefined4 *)(iVar11 + 0xf6) = 0x0;
  }
  else {
    unk_win_ui_op_1020_717e((UINT16 *)CONCAT22(puVar8,paVar4),(ULONG)param_1,unaff_SS);
    *(astruct_160 **)(iVar11 + 0xf6) = paVar4;
    *(uint *)(iVar11 + 0xf8) = uVar10;
  }
  uVar6 = *(undefined4 *)(iVar11 + 0xf6);
  *(undefined4 *)(iVar11 + 0xe8) = uVar6;
  uVar1 = *(undefined4 *)(iVar11 + 0xf2);
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar11 + 0xf2) + 0x10);
  (**ppcVar2)(0x1000,(int)uVar1,(int)((ulong)uVar1 >> 0x10));
  pBVar5 = (BOOL16 *)uVar6;
  MoveWindow16(0x1000,0x1,pBVar5[0x3],pBVar5[0x2],pBVar5[0x1],*pBVar5);
  uVar6 = *(undefined4 *)param_1;
  ppcVar2 = (code **)((int)uVar6 + 0x94);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,iVar11,(char)((ulong)param_1 >> 0x10),0x0);
  ppcVar2 = (code **)((int)uVar6 + 0x10);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,param_1,0x1);
  UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_6e52(ushort param_1,uint param_2,uint param_3,int param_4,ushort param_5,int param_6)

{
  uint uVar1;
  char *pcVar2;
  
  pass1_1018_2e5e(param_1,param_2,param_3,*(ulong *)(param_4 + 0xf2));
  uVar1 = param_3 | param_2;
  if (uVar1 == 0x0) {
    pcVar2 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  }
  else {
    pass1_1018_2d84(param_2,*(ulong *)(param_4 + 0xf2));
    pcVar2 = (char *)CONCAT22(uVar1,param_2);
  }
  string_1020_79b4(param_1,CONCAT22(param_5,param_4),param_6,pcVar2);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_fn_1020_6e98(undefined4 param_1,HWND16 in_win_handle,UINT16 param_3)

{
  int *piVar1;
  astruct_18 *paVar2;
  HWND16 window_handle;
  ushort uVar3;
  undefined2 in_DX;
  WPARAM16 WVar4;
  int iVar5;
  undefined2 uVar6;
  LRESULT LVar7;
  char *pcVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  int iStack36;
  undefined4 window_name;
  RECT16 rectangle;
  INT16 IStack6;
  int iStack4;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  GetClientRect16(in_win_handle,&rectangle);
  window_name = (astruct_18 *)0x0;
  window_handle = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1797);
  if (window_handle != 0x0) {
    DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
  }
  pass1_1018_30fc(*(ulong *)(iVar5 + 0xf2),(ushort **)CONCAT22(param_3,&window_name),in_DX);
  if ((window_name._2_2_ | (uint)(LPCSTR)window_name) != 0x0) {
    window_handle =
         CreateWindow16((LPCSTR)0x1018,(LPCSTR)window_name,CONCAT22(PTR_LOOP_1050_038c,window_name._2_2_),0x1797,
                        *(INT16 *)(iVar5 + 0x8),iStack4 + -0x19,IStack6,0x0,0x0,0x103,(LPVOID)0x40a0);
    paVar2 = window_name;
    if (window_handle == 0x0) {
      if ((window_name._2_2_ | (uint)window_name) != 0x0) {
        pass1_1018_2afa((ulong *)window_name);
        fn_ptr_1000_17ce(paVar2,0x1000);
        return;
      }
    }
    else {
      LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0000);
      WVar4 = (WPARAM16)((ulong)LVar7 >> 0x10);
      if (*(int *)((int)window_name + 0x4) == 0x0) {
        uVar9 = 0x0;
        uVar10 = 0x401;
        pcVar8 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
        SendMessage16(0x1010,(UINT16)pcVar8,(WPARAM16)((ulong)pcVar8 >> 0x10),CONCAT22(uVar10,uVar9));
      }
      else {
        iStack36 = 0x0;
        while( true ) {
          piVar1 = (int *)((int)window_name + 0x4);
          if (*piVar1 == iStack36 || *piVar1 < iStack36) break;
          uVar9 = 0x0;
          uVar10 = 0x401;
          uVar3 = pass1_1020_bd80(*(ushort *)((int)*(undefined4 *)window_name + iStack36 * 0x2));
          LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,uVar3,WVar4,CONCAT22(uVar10,uVar9));
          WVar4 = (WPARAM16)((ulong)LVar7 >> 0x10);
          iStack36 = iStack36 + 0x1;
        }
      }
      LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0001);
      WVar4 = (WPARAM16)((ulong)LVar7 >> 0x10);
      uVar3 = (ushort)LVar7;
      uVar9 = 0xffff;
      uVar10 = 0x40d;
      pass1_1018_2d84(uVar3,*(ulong *)(iVar5 + 0xf2));
      LVar7 = SendMessage16(0x1018,uVar3,WVar4,CONCAT22(uVar10,uVar9));
      iVar5 = (int)LVar7;
      if ((iVar5 != -0x1) || ((int)((ulong)LVar7 >> 0x10) != -0x1)) {
        SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,CONCAT22(0x407,iVar5));
        SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,CONCAT22(0x418,iVar5));
      }
      paVar2 = window_name;
      window_handle = (HWND16)s_tile2_bmp_1050_1538;
      if (window_name != (astruct_18 *)0x0) {
        pass1_1018_2afa((ulong *)window_name);
        window_handle = 0x1000;
        fn_ptr_1000_17ce(paVar2,0x1000);
      }
      ShowWindow16(window_handle,0x1);
      SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

HGDIOBJ16 __stdcall16far draw_op_1020_7070(INT16 param_1,uint param_2)

{
  HGDIOBJ16 HVar1;
  
  HVar1 = GetStockObject16(param_1);
  if (_PTR_LOOP_1050_441e == 0x0) {
    _PTR_LOOP_1050_441e = 0x1000002;
  }
  if (0x6 < param_2) {
    return 0x0;
  }
  SetTextColor16((HDC16)s_tile2_bmp_1050_1538,(COLORREF)_PTR_LOOP_1050_441e);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
  return HVar1;
}



astruct_3 * __stdcall16far pass1_1020_70c0(astruct_3 *param_1,byte param_2,ushort param_3)

{
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_win_ui_op_1020_717e(UINT16 *param_1,ULONG param_2,UINT16 param_3)

{
  astruct_13 *paVar1;
  code **ppcVar2;
  undefined4 uVar3;
  HPALETTE16 HVar4;
  undefined4 *puVar5;
  uchar *in_DX;
  undefined2 uVar6;
  uchar *extraout_DX;
  uchar *puVar7;
  int iVar8;
  int iVar10;
  int unaff_DI;
  undefined2 uVar11;
  undefined2 uVar12;
  undefined2 unaff_CS;
  ushort *puVar13;
  HDC16 local_4;
  astruct_41 *iVar9;
  
  get_sys_metrics_1020_7c1a(param_1,param_2,unaff_CS);
  uVar11 = (undefined2)((ulong)param_1 >> 0x10);
  iVar8 = (int)param_1;
  *(undefined4 *)(iVar8 + 0x14) = 0x0;
  *(ULONG *)(iVar8 + 0x18) = param_2;
  *(undefined4 *)(iVar8 + 0x1c) = 0x0;
  *(undefined2 *)(iVar8 + 0x20) = 0x0;
  *param_1 = 0x754c;
  *(undefined2 *)(iVar8 + 0x2) = 0x1020;
  puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x4,param_3,in_DX,unaff_DI);
  uVar6 = (undefined2)((ulong)puVar13 >> 0x10);
  *(undefined2 *)(iVar8 + 0x1c) = (int)puVar13;
  *(undefined2 *)(iVar8 + 0x1e) = uVar6;
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0x1c) + 0x4);
  (**ppcVar2)(0x1010,*(undefined2 *)(iVar8 + 0x1c),uVar6,0x0,param_1);
  uVar6 = *(undefined2 *)(iVar8 + 0x4);
  local_4 = GetDC16(0x1010);
  uVar3 = *(undefined4 *)(iVar8 + 0x1c);
  *(HDC16 *)((int)uVar3 + 0x178) = local_4;
  uVar3 = *(undefined4 *)(iVar8 + 0x1c);
  uVar12 = (undefined2)((ulong)uVar3 >> 0x10);
  iVar10 = (int)uVar3;
  puVar5 = (undefined4 *)*(undefined4 *)(iVar10 + 0x24);
  ppcVar2 = (code **)((int)*puVar5 + 0x14);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)puVar5,*(undefined2 *)(iVar10 + 0x26),uVar6);
  puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_3,extraout_DX,unaff_DI);
  puVar7 = (uchar *)((ulong)puVar13 >> 0x10);
  paVar1 = *(astruct_13 **)((int)puVar13 + 0xe);
  pass1_1008_4d84((astruct_90 *)((ulong)puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10),(ulong)paVar1,puVar7);
  HVar4 = palette_op_1008_4e08(paVar1,&local_4,(ushort)puVar7,0x1008);
  *(HPALETTE16 *)(iVar8 + 0x20) = HVar4;
  return;
}



void __stdcall16far palette_op_1020_7270(ushort *param_1,HDC16 param_2)

{
  uint uVar1;
  uint uVar2;
  HPALETTE16 HVar3;
  int iVar4;
  uint uVar5;
  ushort unaff_SS;
  astruct_18 *paStack8;
  
  uVar5 = (uint)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x754c;
  *(undefined2 *)(iVar4 + 0x2) = 0x1020;
  if (*(long *)(iVar4 + 0x1c) != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1ea6(*(ulong *)(iVar4 + 0x1c),(ulong)param_1 & 0xffff | (ulong)uVar5 << 0x10,unaff_SS);
  }
  uVar1 = *(uint *)(iVar4 + 0x14);
  uVar2 = *(uint *)(iVar4 + 0x16);
  paStack8 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar2,uVar1));
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paStack8,0x1000);
  }
  HVar3 = SelectPalette16(param_2,0x0,*(BOOL16 *)(iVar4 + 0x20));
  *(HPALETTE16 *)(iVar4 + 0x20) = HVar3;
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  *param_1 = 0x3ab0;
  *(undefined2 *)(iVar4 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar4 + 0x2) = 0x1008;
  return;
}



void __stdcall16far post_win_msg_1020_7308(ulong param_1,uint param_2,HWND16 param_3)

{
  char cVar1;
  
  if (param_2 != 0x12) {
    if (param_2 < 0x13) {
      cVar1 = (char)param_2;
      if (cVar1 == '\x01') {
        *(undefined4 *)((int)param_1 + 0x1c) = 0x0;
        return;
      }
      if (('\x03' < (char)(cVar1 + -0x1)) && ((char)(cVar1 + -0x5) < '\x02')) goto LAB_1020_7310;
    }
    return;
  }
LAB_1020_7310:
  PostMessage16(param_3,0x0,0x0,0x11100eb);
  invalidate_rect_1020_735a(param_1,(int)s_tile2_bmp_1050_1538);
  return;
}



void __stdcall16far invalidate_rect_1020_735a(ulong param_1,HWND16 param_2)

{
  InvalidateRect16(param_2,(RECT16 *)0x0,(int)*(undefined4 *)((int)param_1 + 0x1c) + 0x16c);
  return;
}



BOOL16 __stdcall16far win_ui_op_1020_737a(ULONG param_1,HWND16 param_2,UINT16 param_3)

{
  uint uVar1;
  code **ppcVar2;
  undefined4 uVar3;
  BOOL16 BVar4;
  undefined *puVar5;
  undefined4 *puVar6;
  uint in_DX;
  uint uVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  astruct_18 *paStack78;
  undefined local_42 [0x6];
  undefined4 *local_brush_handle;
  int iStack56;
  HWND16 HStack54;
  HWND16 HStack52;
  int iStack50;
  RECT16 local_30;
  int iStack44;
  int iStack42;
  RECT16 *local_rect;
  BOOL16 BStack38;
  HDC16 local_24;
  PAINTSTRUCT16 local_paint_struct;
  
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar8 = (int)param_1;
  uVar11 = *(undefined2 *)(iVar8 + 0x4);
  local_24 = BeginPaint16(param_2,&local_paint_struct);
  uVar10 = *(undefined2 *)(iVar8 + 0x4);
  BVar4 = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
  if (BVar4 == 0x0) {
    uVar3 = *(undefined4 *)(iVar8 + 0x1c);
    puVar6 = (undefined4 *)*(undefined4 *)((int)uVar3 + 0x24);
    local_brush_handle = puVar6;
    pass1_1018_2e5e(param_3,(ushort)puVar6,in_DX,*(ulong *)(iVar8 + 0x1c));
    local_30 = (ulong)puVar6 & 0xffff | (ulong)in_DX << 0x10;
    pass1_1008_3e54((ushort *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,local_42)),0x0,0x35,0xc);
    if (*(long *)(iVar8 + 0x14) != 0x0) {
      pass1_1008_5236(*(ulong *)(iVar8 + 0x14));
    }
    if (local_30 != 0x0) {
      uVar1 = *(uint *)(iVar8 + 0x14);
      uVar7 = *(uint *)(iVar8 + 0x16);
      paStack78 = (astruct_18 *)CONCAT22(uVar7,uVar1);
      if ((uVar7 | uVar1) != 0x0) {
        pass1_1008_5118(CONCAT22(uVar7,uVar1));
        fn_ptr_1000_17ce(paStack78,0x1000);
      }
      puVar5 = local_42;
      pass1_1008_8ce4(local_30,(ushort *)CONCAT22(param_3,puVar5),(ulong)local_brush_handle,param_3);
      *(int *)(iVar8 + 0x14) = (int)puVar5;
      *(uint *)(iVar8 + 0x16) = uVar7;
    }
    ppcVar2 = (code **)((int)*local_brush_handle + 0x4);
    (**ppcVar2)(0x1008,(int)local_brush_handle,(int)((ulong)local_brush_handle >> 0x10),0x0,0x0);
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0x18) + 0x94);
    (**ppcVar2)(0x1008,*(undefined4 *)(iVar8 + 0x18),0x1);
    HStack52 = GetDlgItem16(0x1008,0x1797);
    if (HStack52 != 0x0) {
      ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    }
  }
  else {
    if (PTR_LOOP_1050_0010 == (undefined *)0x0) {
      ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0x1c) + 0x2c);
      (**ppcVar2)((int)s_tile2_bmp_1050_1538,*(undefined4 *)(iVar8 + 0x1c),uVar10,uVar11);
      BStack38 = BVar4;
      if (BVar4 != 0x0) {
        local_rect = (RECT16 *)GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_30);
        local_brush_handle = (undefined4 *)0x0;
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

