
void __stdcall16far pass1_1020_51c6(ulong param_1,ushort param_2,ulong param_3,ushort param_4,ushort param_5)

{
  code **ppcVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  ushort uVar5;
  
  uVar4 = (uint)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  iVar2 = *(int *)(iVar3 + 0xf4);
  uVar5 = (ushort)param_3;
  if (iVar2 == 0x2) {
    win_ui_op_1020_5e76(param_1 & 0xffff | (ulong)uVar4 << 0x10,param_2,uVar5);
    return;
  }
  iVar2 = iVar2 + -0x3;
  if (iVar2 != 0x0) {
    pt_in_rect_op_1020_58ce
              (param_1 & 0xffff | (ulong)uVar4 << 0x10,param_2,uVar5,(byte)(param_3 >> 0x10),param_4,param_5);
    if (iVar2 == 0x0) {
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x4) + 0x5c);
      (**ppcVar1)(param_4,*(undefined4 *)(iVar3 + 0x4),param_2,param_3);
    }
    return;
  }
  win_ui_op_1020_5de8(param_1 & 0xffff | (ulong)uVar4 << 0x10,param_2,uVar5,param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_cursor_op_1020_522e(astruct_52 *param_1,ushort param_2,ushort param_3)

{
  int iVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  uchar *in_DX;
  int iVar4;
  int unaff_DI;
  undefined2 uVar5;
  HCURSOR16 unaff_CS;
  ushort unaff_SS;
  ushort *puVar6;
  undefined uVar7;
  undefined uVar8;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  iVar1 = *(int *)(iVar4 + 0xf4);
  if (iVar1 == 0x2) {
    SetCursor16(unaff_CS);
    *(undefined2 *)(iVar4 + 0xee) = 0x0;
    *(undefined2 *)(iVar4 + 0xf4) = 0x1;
    *(undefined2 *)(iVar4 + 0x10c) = 0x0;
    ReleaseCapture16();
    return;
  }
  if (iVar1 == 0x3) {
    SetCursor16(unaff_CS);
    *(undefined2 *)(iVar4 + 0xee) = 0x0;
    *(undefined2 *)(iVar4 + 0xf4) = 0x1;
    *(undefined2 *)(iVar4 + 0x10c) = 0x0;
    ReleaseCapture16();
    uVar7 = 0x0;
    uVar8 = 0x0;
    uVar5 = 0x0;
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x47,unaff_SS,in_DX,unaff_DI);
    pass1_1018_57e6((ulong)puVar6,CONCAT22(uVar5,CONCAT11(uVar8,uVar7)),unaff_SS);
    return;
  }
  BVar3 = menu_ui_op_1020_5bf2(param_1,param_2,param_3);
  if (BVar3 == 0x0) {
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0x4) + 0x60);
    (**ppcVar2)();
  }
  return;
}



void __stdcall16far pass1_1020_52de(ULONG param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  int iVar6;
  undefined2 uVar7;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  puVar1 = (undefined4 *)*(uint *)(iVar6 + 0xf6);
  uVar2 = *(uint *)(iVar6 + 0xf8);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)(iVar6 + 0xf6) = 0x0;
  if (*(long *)(iVar6 + 0xfa) != 0x0) {
    if (param_1 == 0x0) {
      iVar4 = 0x0;
      uVar5 = 0x0;
    }
    else {
      iVar4 = iVar6 + 0xe2;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6(*(ulong *)(iVar6 + 0xfa),CONCAT22(uVar5,iVar4),param_2);
  }
  destroy_win_1008_628e(param_1,0x1008);
  if (*(long *)(iVar6 + 0xfa) != 0x0) {
    pass1_1010_1dda(*(ulong *)(iVar6 + 0xfa));
  }
  *(undefined4 *)(iVar6 + 0xfa) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far ui_op_1020_536e(ulong param_1,undefined4 param_2,int param_3,int param_4,uchar *param_5)

{
  int *piVar1;
  UINT16 UVar2;
  code **ppcVar3;
  ushort uVar4;
  ushort uVar5;
  UINT16 UVar6;
  uint uVar7;
  uchar *puVar8;
  uchar *extraout_DX;
  uchar *puVar9;
  int iVar10;
  undefined4 *puVar11;
  int unaff_DI;
  undefined2 uVar12;
  ushort unaff_SS;
  ushort *puVar13;
  undefined4 uVar14;
  ulong uVar15;
  undefined uVar16;
  undefined uVar17;
  undefined2 uVar18;
  undefined2 uVar19;
  undefined4 *puStack16;
  
  uVar7 = param_4 - 0x1;
  uVar12 = (undefined2)(param_1 >> 0x10);
  iVar10 = (int)param_1;
  if (uVar7 == 0x0) {
    if (*(long *)(iVar10 + 0xfe) == 0x0) {
      mem_op_1000_179c(0xfc,param_5,0x1000);
      uVar14 = CONCAT22((uint)param_5 | uVar7,uVar7);
      if (((uint)param_5 | uVar7) == 0x0) {
        *(undefined4 *)(iVar10 + 0xfe) = 0x0;
      }
      else {
        piVar1 = (int *)(iVar10 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        uVar14 = unk_win_ui_op_1020_67ce
                           (CONCAT13((char)((uint)param_5 >> 0x8),CONCAT12((char)param_5,uVar7)),
                            *(UINT16 *)(iVar10 + 0xcc),param_1);
        *(undefined2 *)(iVar10 + 0xfe) = (int)uVar14;
        *(undefined2 *)(iVar10 + 0x100) = (int)((ulong)uVar14 >> 0x10);
      }
      pass1_1008_6978(param_1,0x0,*(ulong *)(iVar10 + 0xfe),(uint)uVar14,(uchar *)((ulong)uVar14 >> 0x10));
      uVar14 = *(undefined4 *)(iVar10 + 0xfe);
      uVar18 = (undefined2)uVar14;
      uVar19 = (undefined2)((ulong)uVar14 >> 0x10);
      uVar14 = *(undefined4 *)(iVar10 + 0xfe);
      uVar12 = (undefined2)((ulong)uVar14 >> 0x10);
      puVar11 = (undefined4 *)uVar14;
      goto LAB_1020_53f3;
    }
  }
  else {
    if (param_4 == 0x2) {
      uVar4 = param_3 + 0xc;
      puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uVar4,unaff_SS,param_5,unaff_DI);
      puVar9 = (uchar *)((ulong)puVar13 >> 0x10);
      uVar5 = pass1_1018_0afa((ulong)puVar13);
      if (uVar5 == 0x0) {
        piVar1 = (int *)(iVar10 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        UVar2 = *(UINT16 *)(iVar10 + 0xcc);
        uVar12 = 0xfe;
        UVar6 = UVar2;
        mem_op_1000_179c(0xfe,puVar9,0x1000);
        puVar8 = (uchar *)((uint)puVar9 | UVar6);
        if (puVar8 == (uchar *)0x0) {
          UVar6 = 0x0;
          puVar8 = (uchar *)0x0;
        }
        else {
          pass1_1020_289a((ushort *)CONCAT13((char)((uint)puVar9 >> 0x8),CONCAT12((char)puVar9,UVar6)),UVar2,param_1,
                          unaff_SS);
        }
        puStack16 = (undefined4 *)CONCAT22(puVar8,UVar6);
        uVar16 = SUB21(puVar8,0x0);
        uVar17 = (undefined)((uint)puVar8 >> 0x8);
        pass1_1020_294a(CONCAT13(uVar17,CONCAT12(uVar16,UVar6)),CONCAT22((int)param_2,uVar12),
                        (ushort)((ulong)param_2 >> 0x10),puVar8,unaff_DI,unaff_SS);
        unaff_DI = (int)((ulong)*puStack16 >> 0x10);
        iVar10 = (int)*puStack16;
        ppcVar3 = (code **)(iVar10 + 0x8);
        uVar14 = (**ppcVar3)(0x1000,UVar6,puVar8,uVar4);
        pass1_1008_3e0e(CONCAT13(uVar17,CONCAT12(uVar16,UVar6)));
        pass1_1008_6978(param_1,UVar2,CONCAT22(puVar8,UVar6),(uint)uVar14,(uchar *)((ulong)uVar14 >> 0x10));
        ppcVar3 = (code **)(iVar10 + 0xc);
        (**ppcVar3)(0x1008,(char)UVar6,uVar16,0x1);
        puVar9 = extraout_DX;
      }
      else {
        uVar15 = pass1_1018_0ad4((ulong)puVar13);
        puVar9 = (uchar *)(uVar15 >> 0x10);
        pass1_1008_3e0e(uVar15);
      }
      pass1_1018_1662((ulong)puVar13,0x0,0x0,unaff_SS);
      BringWindowToTop16(0x1018);
      uVar4 = 0x1;
      iVar10 = 0x4;
      puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,unaff_SS,puVar9,unaff_DI);
      pass1_1010_089e(unaff_SS,(ulong)puVar13,uVar4,iVar10);
      pass1_1010_089e(unaff_SS,(ulong)puVar13,0x1,0x3);
      return;
    }
    uVar7 = param_4 - 0x3;
    if ((uVar7 == 0x0) && (*(long *)(iVar10 + 0x102) == 0x0)) {
      mem_op_1000_179c(0xfc,param_5,0x1000);
      puVar9 = (uchar *)((uint)param_5 | uVar7);
      if (puVar9 == (uchar *)0x0) {
        *(undefined4 *)(iVar10 + 0x102) = 0x0;
      }
      else {
        piVar1 = (int *)(iVar10 + 0xcc);
        *piVar1 = *piVar1 + 0x1;
        pass1_1020_0dc4((ushort *)CONCAT13((char)((uint)param_5 >> 0x8),CONCAT12((char)param_5,uVar7)),
                        *(UINT16 *)(iVar10 + 0xcc),param_1,unaff_SS);
        *(uint *)(iVar10 + 0x102) = uVar7;
        *(uchar **)(iVar10 + 0x104) = puVar9;
      }
      pass1_1008_6978(param_1,0x0,*(ulong *)(iVar10 + 0x102),uVar7,puVar9);
      uVar14 = *(undefined4 *)(iVar10 + 0x102);
      uVar18 = (undefined2)uVar14;
      uVar19 = (undefined2)((ulong)uVar14 >> 0x10);
      uVar14 = *(undefined4 *)(iVar10 + 0x102);
      uVar12 = (undefined2)((ulong)uVar14 >> 0x10);
      puVar11 = (undefined4 *)uVar14;
LAB_1020_53f3:
      ppcVar3 = (code **)((int)*puVar11 + 0xc);
      (**ppcVar3)(0x8,uVar18,uVar19,0x5);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far post_msg_1020_55b0(ulong param_1,ushort param_2)

{
  code **ppcVar1;
  uint uVar2;
  uchar *in_DX;
  uchar *puVar3;
  undefined2 uVar4;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  int unaff_DI;
  ushort uVar5;
  HWND16 hwnd;
  HWND16 hwnd_00;
  uchar in_AF;
  ushort *puVar5;
  char *pcVar6;
  ulong *puVar6;
  LRESULT LVar7;
  undefined uVar8;
  int local_114;
  undefined local_112 [0x2];
  int iStack272;
  int local_10e;
  char local_10c [0x100];
  ushort *puStack12;
  int iStack8;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  puVar3 = (uchar *)((ulong)puStack6 >> 0x10);
  iStack8 = *(int *)((int)puStack6 + 0x20);
  puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_2,puVar3,unaff_DI);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_10c,param_2);
  puVar5 = pass1_1008_9436((ushort *)CONCAT22(param_2,local_112));
  uVar8 = (undefined)(param_2 >> 0x8);
  pcVar6 = (char *)pass1_1008_a8f4((ulong)puStack12,(ushort *)CONCAT13(uVar8,CONCAT12((char)param_2,&local_114)),
                                   (ushort *)CONCAT22(param_2,local_112),(ushort *)CONCAT22(param_2,&local_10e),
                                   (ushort)((ulong)puVar5 >> 0x10),0x1008,param_2,in_AF);
  uVar2 = (uint)pcVar6;
  puVar3 = (uchar *)((uint)((ulong)pcVar6 >> 0x10) | uVar2);
  uVar5 = (ushort)(param_1 >> 0x10);
  hwnd_00 = 0x1008;
  if ((pcVar6 != (char *)0x0) && (*pcVar6 != '\0')) {
    hwnd = 0x1000;
    mem_op_1000_179c(0xb4,puVar3,0x1000);
    if (((uint)puVar3 | uVar2) == 0x0) {
      puVar6 = (ulong *)0x0;
    }
    else {
      hwnd = (HWND16)&PTR_LOOP_1050_1040;
      puVar6 = (ulong *)pass1_1040_8478((astruct_57 *)CONCAT22(puVar3,uVar2),0x0,
                                        (char *)CONCAT13(uVar8,CONCAT12((char)param_2,local_10c)),pcVar6,
                                        *(ushort *)((int)param_1 + 0x8),(uint)puVar3 | uVar2);
    }
    uVar4 = (undefined2)((ulong)puVar6 >> 0x10);
    if (iStack272 == 0x0) {
      ppcVar1 = (code **)((int)*puVar6 + 0x74);
      (**ppcVar1)(hwnd,(int)((ulong)puVar6 & 0xffff),uVar4);
      puVar3 = extraout_DX_00;
    }
    else {
      ppcVar1 = (code **)((int)*puVar6 + 0x6c);
      (**ppcVar1)(hwnd,(char)((ulong)puVar6 & 0xffff),uVar4,local_112,param_2);
      puVar3 = extraout_DX;
    }
    if ((iStack8 == 0x0) || (hwnd_00 = hwnd, local_114 == 0x0)) {
      hwnd_00 = (HWND16)s_tile2_bmp_1050_1538;
      PostMessage16(hwnd,0x0,0x0,0x11100fc);
    }
  }
  if ((iStack8 != 0x0) && (local_114 != 0x0)) {
    LVar7 = SendMessage16(hwnd_00,0x0,0x0,CONCAT22(0x111,local_114));
    *(undefined2 *)((int)param_1 + 0x112) = 0x1;
    return (ushort)(uchar *)((ulong)LVar7 >> 0x10);
  }
  if (local_10e == 0x6) {
    PostMessage16(hwnd_00,0x0,0x0,0x11100b0);
    hwnd_00 = 0x1010;
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,puVar3,unaff_DI);
    puVar3 = (uchar *)((ulong)puVar5 >> 0x10);
    *(undefined2 *)((int)puVar5 + 0x20) = 0x1;
  }
  if (local_10e == 0x15) {
    PostMessage16(hwnd_00,0x0,0x0,0x1110097);
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,puVar3,unaff_DI);
    puVar3 = (uchar *)((ulong)puVar5 >> 0x10);
    *(undefined2 *)((int)puVar5 + 0x20) = 0x1;
  }
  return (ushort)puVar3;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far set_cursor_1020_5764(ulong param_1,int param_2,ushort param_3)

{
  uint uVar1;
  undefined4 uVar2;
  uchar *in_DX;
  int iVar3;
  int unaff_DI;
  undefined2 uVar4;
  HINSTANCE16 h_instance;
  HCURSOR16 hcursor;
  int local_e;
  undefined local_c [0x2];
  ulong uStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,in_DX,unaff_DI);
  uVar4 = (undefined2)((ulong)puStack6 >> 0x10);
  uStack10 = *(ulong *)((int)puStack6 + 0x20);
  uVar1 = *(uint *)((int)puStack6 + 0x22);
  if ((uVar1 | (uint)uStack10) != 0x0) {
    h_instance = 0x1030;
    pass1_1030_8308((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                    (ushort *)CONCAT22(param_3,&local_e),
                    (ushort *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,local_c)),
                    uStack10 & 0xffff | (ulong)uVar1 << 0x10,(ushort)&local_e,uVar1);
    if (param_2 <= local_e) {
      uVar4 = (undefined2)(param_1 >> 0x10);
      iVar3 = (int)param_1;
      if (*(int *)(iVar3 + 0xf4) != 0x1) {
        SetCursor16(0x1030);
        *(undefined2 *)(iVar3 + 0xee) = 0x0;
        *(undefined2 *)(iVar3 + 0xf4) = 0x1;
        *(undefined2 *)(iVar3 + 0x10c) = 0x0;
        h_instance = (HINSTANCE16)s_tile2_bmp_1050_1538;
        ReleaseCapture16();
      }
      LoadCursor16(h_instance,(LPCSTR)0x7f02);
      SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
      hcursor = 0x1018;
      pass1_1018_017c((ulong)puStack6,param_2,param_3);
      uVar2 = *(undefined4 *)(iVar3 + 0xf6);
      *(undefined2 *)((int)uVar2 + 0x10) = 0x1;
      if (*(long *)(iVar3 + 0xfe) != 0x0) {
        pass1_1020_68de(*(ulong *)(iVar3 + 0xfe),0x1018);
        hcursor = (HCURSOR16)s_tile2_bmp_1050_1538;
        PostMessage16(0x1018,0x0,0x0,0x11100eb);
      }
      SetCursor16(hcursor);
    }
  }
  return;
}



void __stdcall16far pt_in_rect_1020_5856(ulong param_1,POINT16 *param_2,uint param_3)

{
  ulong *puVar1;
  BOOL16 BVar2;
  ulong uVar3;
  uint in_DX;
  uint extraout_DX;
  ulong uStack10;
  
  pass1_1018_2862(*(ulong *)((int)param_1 + 0xfa));
  if ((in_DX | param_3) != 0x0) {
    uStack10 = 0x0;
    while( true ) {
      puVar1 = (ulong *)(param_3 + 0xa);
      if (*puVar1 < uStack10 || *puVar1 == uStack10) break;
      uVar3 = uStack10;
      empty_1008_8fc4(param_3,in_DX,(int)uStack10,(int)(uStack10 >> 0x10));
      if ((extraout_DX | (uint)uVar3) != 0x0) {
        BVar2 = PtInRect16((RECT16 *)0x1008,*param_2);
        if (BVar2 != 0x0) {
          return;
        }
      }
      uStack10 = uStack10 + 0x1;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pt_in_rect_op_1020_58ce(ulong param_1,ushort param_2,ushort param_3,byte param_4,RECT16 *param_5,ushort param_6)

{
  code **ppcVar1;
  undefined4 uVar2;
  ushort uVar3;
  BOOL16 BVar4;
  ushort *msg;
  uchar *in_DX;
  uint uVar5;
  uchar *puVar6;
  int iVar7;
  int iVar8;
  int unaff_DI;
  undefined2 uVar9;
  undefined2 uVar10;
  RECT16 *rect;
  RECT16 *rect_00;
  ulong uVar11;
  ushort *puVar12;
  undefined local_34 [0x6];
  ulong uStack46;
  ushort *puStack38;
  undefined4 uStack30;
  ushort *puStack26;
  ushort local_18 [0x2];
  ushort uStack20;
  undefined4 uStack18;
  undefined2 uStack14;
  uchar *puStack12;
  uint uStack10;
  uint uStack8;
  ushort local_6;
  ushort uStack4;
  
  local_6 = param_3;
  uStack4 = param_2;
  uStack8 = param_4 & 0x8;
  uStack10 = param_4 & 0x4;
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar7 = (int)param_1;
  uVar3 = pass1_1020_64d4(*(ulong *)(iVar7 + 0xf6),0x2);
  uStack30._2_2_ = in_DX;
  rect = param_5;
  if (uVar3 == 0x0) {
LAB_1020_5942:
    uVar3 = pass1_1020_64d4(*(ulong *)(iVar7 + 0xf6),0x4);
    rect_00 = rect;
    if (uVar3 == 0x0) {
LAB_1020_5a16:
      uVar3 = pass1_1020_64d4(*(ulong *)(iVar7 + 0xf6),0x1);
      if (uVar3 != 0x0) {
        uStack30 = pass1_1020_6498(*(ulong *)(iVar7 + 0xf6),0x1);
        uStack30._2_2_ = (uchar *)(uStack30 >> 0x10);
        for (puStack26 = (ushort *)0x0; (int)puStack26 < 0x4; puStack26 = (ushort *)((int)puStack26 + 0x1)) {
          BVar4 = PtInRect16(rect_00,(POINT16)CONCAT22(uStack4,local_6));
          if (BVar4 != 0x0) {
            local_18[0] = 0x0;
            uStack20 = 0x0;
            if (puStack26 == (ushort *)0x0) {
              uStack20 = (-(uint)(uStack10 == 0x0) & 0x4) - 0x5;
            }
            else {
              if (puStack26 == (ushort *)((int)&PTR_LOOP_1050_0000 + 0x1)) {
                uStack20 = (-(uint)(uStack10 == 0x0) & 0xfffc) + 0x5;
              }
              else {
                if (puStack26 == (ushort *)&PTR_LOOP_1050_0002) {
                  local_18[0] = (-(uint)(uStack10 == 0x0) & 0x4) - 0x5;
                }
                else {
                  if (puStack26 == (ushort *)((int)&PTR_LOOP_1050_0002 + 0x1)) {
                    local_18[0] = (-(uint)(uStack10 == 0x0) & 0xfffc) + 0x5;
                  }
                }
              }
            }
            pass1_1020_2a94(*(ulong *)(iVar7 + 0xce),CONCAT22(local_18[0],uStack20),param_6);
            return;
          }
          rect_00 = (RECT16 *)s_tile2_bmp_1050_1538;
        }
      }
      uVar3 = pass1_1020_64d4(*(ulong *)(iVar7 + 0xf6),0x3);
      if (uVar3 != 0x0) {
        uStack30._0_2_ = &local_6;
        pt_in_rect_1020_5856(param_1,(POINT16 *)CONCAT22(param_6,(ushort *)uStack30),(ushort *)uStack30);
        uVar5 = (uint)uStack30._2_2_ | (uint)(ushort *)uStack30;
        if (uVar5 != 0x0) {
          puStack26 = (ushort *)((ushort *)uStack30)[0x17];
          if (((uStack8 == 0x0) || (uStack10 == 0x0)) && (uStack10 == 0x0)) {
            local_18[0] = 0x1;
          }
          else {
            local_18[0] = 0x2;
          }
          uStack20 = ((ushort *)uStack30)[0x6];
          uStack18 = (ushort *)CONCAT22(uStack18._2_2_,((ushort *)uStack30)[0x7]);
          if ((puStack26 == (ushort *)0xb) || (puStack26 == (ushort *)0x37)) {
            uVar2 = *(undefined4 *)(iVar7 + 0xfa);
            uVar10 = (undefined2)((ulong)uVar2 >> 0x10);
            iVar8 = (int)uVar2;
            uStack46 = *(ulong *)(iVar8 + 0x20);
            uVar5 = *(uint *)(iVar8 + 0x22);
            if ((uVar5 | (uint)uStack46) != 0x0) {
              puVar12 = pass1_1008_3e38((ushort *)CONCAT22(param_6,local_34));
              puVar6 = (uchar *)((ulong)puVar12 >> 0x10);
              pass1_1018_161c(param_6,uStack46,(ushort *)CONCAT22(param_6,local_34),(int)uStack18,uStack20);
              puStack38 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,puVar6,unaff_DI);
              uVar5 = (uint)((ulong)puStack38 >> 0x10);
              pass1_1010_ecc6((ulong)puStack38,(ushort *)CONCAT22(param_6,local_34),*(long *)((int)uStack46 + 0x3c),
                              (ushort)local_34,uVar5,param_6);
            }
          }
          uVar3 = pass1_1018_25d2(*(ulong *)(iVar7 + 0xfa),local_18[0],
                                  (ulong)uStack18 & 0xffff | (ulong)uStack20 << 0x10,unaff_DI,param_6);
          if (uVar3 != 0x0) {
            return;
          }
          uVar3 = pass1_1020_5d56((ulong *)param_1,CONCAT22(uStack30._2_2_,(ushort *)uStack30),uVar5,unaff_DI,param_6);
          if (uVar3 != 0x0) {
            return;
          }
        }
      }
      return;
    }
    uVar11 = pass1_1020_6498(*(ulong *)(iVar7 + 0xf6),0x4);
    uStack30._2_2_ = (uchar *)(uVar11 >> 0x10);
    uVar10 = (undefined2)uVar11;
    rect_00 = (RECT16 *)s_tile2_bmp_1050_1538;
    puVar6 = uStack30._2_2_;
    uStack14 = uVar10;
    puStack12 = uStack30._2_2_;
    BVar4 = PtInRect16(rect,(POINT16)CONCAT22(uStack4,local_6));
    if (BVar4 == 0x0) goto LAB_1020_5a16;
    rect = (RECT16 *)0x1010;
    uStack18 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_6,uStack30._2_2_,unaff_DI);
    if (*(int *)((int)uStack18 + 0x72) != 0x0) {
      *(undefined2 *)(iVar7 + 0x116) = 0x1;
      if (param_1 == 0x0) {
        iVar7 = 0x0;
        uVar9 = 0x0;
      }
      else {
        iVar7 = iVar7 + 0xe2;
      }
      uStack30 = CONCAT22(uVar9,iVar7);
      ppcVar1 = (code **)((int)*_PTR_LOOP_1050_02a0 + 0x4);
      (**ppcVar1)(0x1010,(int)_PTR_LOOP_1050_02a0,(int)((ulong)_PTR_LOOP_1050_02a0 >> 0x10),0x10,iVar7,uVar9,uVar10,
                  puVar6);
      puVar12 = pass1_1008_941a((ushort *)CONCAT22(param_6,local_18),0x1,0x86);
      msg = local_18;
      rect = (RECT16 *)0x1008;
      win_1008_5c9e((ulong)_PTR_LOOP_1050_02a0,(ulong *)CONCAT22(param_6,msg),msg,(int)((ulong)puVar12 >> 0x10),param_6)
      ;
      if (msg != (ushort *)0x0) {
        return;
      }
      uVar9 = 0xf6;
      puStack26 = msg;
      goto LAB_1020_5936;
    }
    uVar9 = 0xf6;
  }
  else {
    uVar11 = pass1_1020_6498(*(ulong *)(iVar7 + 0xf6),0x2);
    uStack30._2_2_ = (uchar *)(uVar11 >> 0x10);
    uStack14 = (undefined2)uVar11;
    rect = (RECT16 *)s_tile2_bmp_1050_1538;
    puStack12 = uStack30._2_2_;
    BVar4 = PtInRect16(param_5,(POINT16)CONCAT22(uStack4,local_6));
    if (BVar4 == 0x0) goto LAB_1020_5942;
    uVar9 = 0x68;
  }
  msg = (ushort *)0x0;
LAB_1020_5936:
  PostMessage16((HWND16)rect,(UINT16)msg,(WPARAM16)msg,CONCAT22(0x111,uVar9));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far menu_ui_op_1020_5bf2(astruct_52 *param_1,HWND16 param_2,RECT16 *param_3)

{
  undefined4 uVar1;
  ushort uVar2;
  BOOL16 BVar3;
  RECT16 **ppRVar4;
  HMENU16 HVar5;
  uint in_DX;
  uint uVar6;
  astruct_52 *iVar5;
  undefined2 uVar7;
  RECT16 *unaff_CS;
  RECT16 *instance;
  WNDCLASS16 *unaff_SS;
  ulong uVar8;
  POINT16 local_10;
  int iStack12;
  undefined4 uStack10;
  RECT16 *local_6;
  HWND16 HStack4;
  
  local_6 = param_3;
  HStack4 = param_2;
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_52 *)param_1;
  uVar2 = pass1_1020_64d4(iVar5->field_0xf6,0x2);
  uVar8 = (ulong)in_DX << 0x10;
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
  pt_in_rect_1020_5856((ulong)param_1,(POINT16 *)CONCAT22(unaff_SS,ppRVar4),ppRVar4);
  uVar6 = (uint)(uVar8 >> 0x10);
  iVar5->field_0x108 = (uint)ppRVar4;
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
  uVar1 = *(undefined4 *)&iVar5->field_0x108;
  uStack10._0_2_ = *(ushort *)((int)uVar1 + 0x2e);
  iStack12 = 0x0;
  if ((ushort)uStack10 == 0x42) {
    iStack12 = 0xc9;
  }
  else {
    if (*(int *)((int)s_VrMode_1050_42ca + 0x8 + (ushort)uStack10 * 0x2) == 0x0) {
      iStack12 = 0xc8;
    }
  }
  if (iStack12 != 0x0) {
    instance = (RECT16 *)0x1008;
    win_1008_5c7c(_PTR_LOOP_1050_02a0,CONCAT22(iStack12,0x1),unaff_SS,(ushort)uStack10,(ushort)(uVar8 >> 0x10));
  }
  local_10.x = (INT16)param_3;
  local_10.y = param_2;
  ClientToScreen16((HWND16)instance,&local_10);
  TrackPopupMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0,0x0,iVar5->field_0x8,0x0,local_10.y,(RECT16 *)local_10.x);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1020_5d56(ulong *param_1,ulong param_2,uchar *param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  ushort uVar2;
  ushort uVar3;
  int local_12 [0x2];
  int local_e;
  int local_c;
  int local_a [0x2];
  int iStack6;
  
  iStack6 = *(int *)((int)param_2 + 0x2e);
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)((ulong)param_1 >> 0x10);
  if (iStack6 == 0x47) {
    pass1_1020_61c4(uVar2,uVar3,CONCAT22(param_5,&local_c),(ushort *)CONCAT22(param_5,local_a),param_3,param_4,param_5);
    if (local_a[0] == 0x0) goto LAB_1020_5d8b;
    if (local_c <= local_a[0]) {
      return 0x1;
    }
  }
  else {
    if (iStack6 != 0x6a) {
      return 0x0;
    }
    pass1_1020_61c4(uVar2,uVar3,CONCAT22(param_5,&local_e),(ushort *)CONCAT22(param_5,local_12),param_3,param_4,param_5)
    ;
    if (local_e <= local_12[0]) {
LAB_1020_5d8b:
      ppcVar1 = (code **)((int)*param_1 + 0x40);
      (**ppcVar1)();
      return 0x1;
    }
  }
  pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar2 + 0x8),0x9,(ushort)param_3,uVar2,(ushort)&PTR_LOOP_1050_1038,
                  param_5);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1020_5de8(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ushort uVar1;
  undefined4 uVar2;
  ushort *puVar3;
  uchar *in_DX;
  uint uVar4;
  int unaff_DI;
  undefined2 uVar5;
  ushort *puVar6;
  undefined uVar7;
  undefined uVar8;
  undefined2 uStack18;
  char cStack15;
  ushort local_6;
  ushort uStack4;
  
  ReleaseCapture16();
  uVar5 = (undefined2)(param_1 >> 0x10);
  SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  *(undefined2 *)((int)param_1 + 0xee) = 0x0;
  *(undefined2 *)((int)param_1 + 0xf4) = 0x1;
  local_6 = param_3;
  uStack4 = param_2;
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x47,param_4,in_DX,unaff_DI);
  uVar4 = (uint)((ulong)puVar6 >> 0x10);
  puVar3 = &local_6;
  pt_in_rect_1020_5856(param_1,(POINT16 *)CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,puVar3)),puVar3);
  if ((uVar4 | (uint)puVar3) != 0x0) {
    uVar2 = *(undefined4 *)(puVar3 + 0x21);
    uVar1 = puVar3[0x22];
    cStack15 = (char)((ulong)uVar2 >> 0x18);
    if (cStack15 == '\x05') {
      uVar7 = (undefined)uVar1;
      uVar8 = (undefined)(uVar1 >> 0x8);
      uStack18 = (undefined2)uVar2;
      goto LAB_1020_5e62;
    }
  }
  uStack18 = 0x0;
  uVar7 = 0x0;
  uVar8 = 0x0;
LAB_1020_5e62:
  pass1_1018_57e6((ulong)puVar6,CONCAT13(uVar8,CONCAT12(uVar7,uStack18)),param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1020_5e76(ulong param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  astruct_57 *paVar2;
  ushort *puVar3;
  uchar *puVar4;
  int iVar5;
  uint uVar6;
  uint in_DX;
  uchar *puVar7;
  uchar *puVar8;
  int iVar9;
  undefined4 *puVar10;
  int unaff_DI;
  undefined2 uVar11;
  undefined2 uVar12;
  ushort uVar13;
  uchar *unaff_SS;
  undefined in_AF;
  char *pcVar14;
  undefined uVar15;
  ushort *local_2aa [0x40];
  uchar *local_1aa [0x80];
  char local_aa [0x80];
  undefined4 uStack42;
  astruct_57 *paStack38;
  char local_22 [0x10];
  uchar *puStack18;
  undefined2 uStack16;
  ushort uStack14;
  ushort uStack12;
  undefined4 uStack10;
  ushort local_6;
  ushort uStack4;
  
  ReleaseCapture16();
  uVar11 = (undefined2)(param_1 >> 0x10);
  iVar9 = (int)param_1;
  SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  *(undefined2 *)(iVar9 + 0xee) = 0x0;
  *(undefined2 *)(iVar9 + 0xf4) = 0x1;
  local_6 = param_3;
  uStack4 = param_2;
  puVar3 = &local_6;
  uVar15 = (undefined)((uint)unaff_SS >> 0x8);
  pt_in_rect_1020_5856(param_1,(POINT16 *)CONCAT13(uVar15,CONCAT12((char)unaff_SS,puVar3)),puVar3);
  uStack10 = CONCAT22(in_DX,puVar3);
  puVar7 = (uchar *)(in_DX | (uint)puVar3);
  if (puVar7 == (uchar *)0x0) goto LAB_1020_6176;
  uStack12 = puVar3[0x6];
  uStack14 = puVar3[0x7];
  uStack16 = 0x0;
  uVar13 = 0x1018;
  puVar4 = pass1_1018_2580(*(ulong *)(iVar9 + 0xfa),0x0,CONCAT22(uStack12,uStack14),*(ushort *)(iVar9 + 0x10c),unaff_SS,
                           in_AF);
  if (puVar4 == (uchar *)0x6b2) goto LAB_1020_6176;
  puStack18 = puVar4;
  if (puVar4 == (uchar *)0x6b8) {
    mem_op_1000_179c(0xb4,puVar7,0x1000);
    uStack42 = CONCAT22(puVar7,puVar4);
    puVar8 = (uchar *)((uint)puVar7 | (uint)puVar4);
    if (puVar8 == (uchar *)0x0) {
      iVar5 = 0x0;
      puVar8 = (uchar *)0x0;
    }
    else {
      iVar5 = string_1040_8520((astruct_57 *)CONCAT13((char)((uint)puVar7 >> 0x8),CONCAT12((char)puVar7,puVar4)),
                               (ushort)PTR_LOOP_1050_0396,0x40,0x2,0x6b8,0x6ad,puVar8,(ushort)unaff_SS);
    }
    paStack38 = (astruct_57 *)CONCAT22(puVar8,iVar5);
    uVar13 = 0xa5;
LAB_1020_5f84:
    pass1_1008_941a((ushort *)CONCAT22(unaff_SS,local_22),0x1,uVar13);
    pcVar14 = local_22;
    uVar12 = (undefined2)((ulong)paStack38 >> 0x10);
    puVar10 = (undefined4 *)paStack38;
  }
  else {
    if (puVar4 == (uchar *)0x6b4) {
      mem_op_1000_179c(0xb4,puVar7,0x1000);
      uStack42 = CONCAT22(puVar7,puVar4);
      puVar8 = (uchar *)((uint)puVar7 | (uint)puVar4);
      if (puVar8 == (uchar *)0x0) {
        iVar5 = 0x0;
        puVar8 = (uchar *)0x0;
      }
      else {
        iVar5 = string_1040_8520((astruct_57 *)CONCAT13((char)((uint)puVar7 >> 0x8),CONCAT12((char)puVar7,puVar4)),
                                 (ushort)PTR_LOOP_1050_0396,0x40,0x2,0x57b,puStack18,puVar8,(ushort)unaff_SS);
      }
      paStack38 = (astruct_57 *)CONCAT22(puVar8,iVar5);
      uVar13 = 0xab;
      goto LAB_1020_5f84;
    }
    if (puVar4 == (uchar *)0x6b6) {
      load_string_1010_84e0
                (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_aa,
                 (short)unaff_SS);
      load_string_1010_84e0
                (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,(char *)local_1aa
                 ,(short)unaff_SS);
      uVar6 = sys_1000_3f9c((uchar *)local_2aa,unaff_SS,(ushort)local_1aa,(ushort)unaff_SS,(ushort)PTR_LOOP_1050_50cc,
                            &stack0xfffe,uVar11,0x1000,unaff_SS,in_AF);
      uVar12 = 0x1000;
      mem_op_1000_179c(0xb4,puVar7,0x1000);
      uStack42 = CONCAT22(puVar7,uVar6);
      if (((uint)puVar7 | uVar6) == 0x0) {
        paStack38 = (astruct_57 *)0x0;
      }
      else {
        uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
        paStack38 = pass1_1040_8478((astruct_57 *)CONCAT22(puVar7,uVar6),0x40,
                                    (char *)CONCAT13(uVar15,CONCAT12((char)unaff_SS,local_aa)),
                                    (char *)CONCAT22(unaff_SS,local_2aa),(ushort)PTR_LOOP_1050_0396,(uint)puVar7 | uVar6
                                   );
      }
      puVar8 = (uchar *)((ulong)paStack38 >> 0x10);
      puVar10 = (undefined4 *)paStack38;
      paVar2 = paStack38;
LAB_1020_6027:
      ppcVar1 = (code **)((int)*puVar10 + 0x74);
      (**ppcVar1)(uVar12,paVar2);
      goto LAB_1020_6176;
    }
    if (puVar4 < (uchar *)0x6a7) {
      if ((*(int *)(iVar9 + 0x10c) == 0x78) || (*(int *)(iVar9 + 0x10c) == 0x74)) {
        uVar13 = 0x1010;
        local_2aa[0] = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x5,(ushort)unaff_SS,puVar7,unaff_DI);
        puVar7 = (uchar *)((ulong)local_2aa[0] >> 0x10);
        if (*(int *)((int)local_2aa[0] + 0xa) == 0x0) {
          return;
        }
      }
      if ((((*(int *)(iVar9 + 0x10c) == 0x6c) || (*(int *)(iVar9 + 0x10c) == 0x6d)) || (*(int *)(iVar9 + 0x10c) == 0x31)
          ) || (*(int *)(iVar9 + 0x10c) == 0x32)) {
        uVar13 = 0x1010;
        local_2aa[0] = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3a,(ushort)unaff_SS,puVar7,unaff_DI);
        if (*(int *)((int)local_2aa[0] + 0xa) == 0x0) {
          return;
        }
      }
      pass1_1020_68de(*(ulong *)(iVar9 + 0xfe),uVar13);
      goto LAB_1020_6176;
    }
    if ((uchar *)0x6b1 < puVar4) {
      uVar12 = 0x1000;
      mem_op_1000_179c(0xb4,puVar7,0x1000);
      uStack42 = CONCAT22(puVar7,puVar4);
      puVar8 = (uchar *)((uint)puVar7 | (uint)puVar4);
      if (puVar8 == (uchar *)0x0) {
        puVar10 = (undefined4 *)0x0;
        puVar8 = (uchar *)0x0;
      }
      else {
        uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
        puVar10 = (undefined4 *)
                  string_1040_8520((astruct_57 *)CONCAT13((char)((uint)puVar7 >> 0x8),CONCAT12((char)puVar7,puVar4)),
                                   (ushort)PTR_LOOP_1050_0396,0x40,0x2,0x57b,puStack18,puVar8,(ushort)unaff_SS);
      }
      local_2aa[0] = (ushort *)CONCAT22(puVar8,puVar10);
      paVar2 = (astruct_57 *)CONCAT22(puVar8,puVar10);
      goto LAB_1020_6027;
    }
    mem_op_1000_179c(0xb4,puVar7,0x1000);
    uStack42 = CONCAT22(puVar7,puVar4);
    puVar8 = (uchar *)((uint)puVar7 | (uint)puVar4);
    if (puVar8 == (uchar *)0x0) {
      iVar5 = 0x0;
      puVar8 = (uchar *)0x0;
    }
    else {
      iVar5 = string_1040_8520((astruct_57 *)CONCAT13((char)((uint)puVar7 >> 0x8),CONCAT12((char)puVar7,puVar4)),
                               (ushort)PTR_LOOP_1050_0396,0x40,0x2,0x57b,puStack18,puVar8,(ushort)unaff_SS);
    }
    local_2aa[0] = (ushort *)CONCAT22(puVar8,iVar5);
    local_1aa[0] = puStack18 + -0x608;
    pass1_1008_941a((ushort *)CONCAT22(unaff_SS,local_aa),0x1,(ushort)local_1aa[0]);
    pcVar14 = local_aa;
    uVar12 = (undefined2)((ulong)local_2aa[0] >> 0x10);
    puVar10 = (undefined4 *)local_2aa[0];
  }
  ppcVar1 = (code **)((int)*puVar10 + 0x6c);
  (**ppcVar1)(0x1008,(char)puVar10,uVar12,pcVar14);
LAB_1020_6176:
  *(undefined2 *)(iVar9 + 0x10c) = 0x0;
  return;
}



void __stdcall16far pass1_1020_6184(ulong param_1,ushort param_2,ushort param_3)

{
  HCURSOR16 HVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(int *)(iVar2 + 0xf4) == 0x1) {
    HVar1 = SetCursor16(param_3);
    *(HCURSOR16 *)(iVar2 + 0xee) = HVar1;
    *(ushort *)(iVar2 + 0x10c) = param_2;
    SetCapture16((HWND16)s_tile2_bmp_1050_1538);
    *(undefined2 *)(iVar2 + 0xf4) = 0x2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_61c4(ushort param_1,ushort param_2,ulong param_3,ushort *param_4,uchar *param_5,int param_6,ushort param_7)

{
  ulong uVar1;
  ushort uVar2;
  ushort *puVar3;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_7,param_5,param_6);
  uVar2 = (ushort)((ulong)puVar3 >> 0x10);
  uVar1 = *(ulong *)((int)puVar3 + 0x20);
  pass1_1030_8308((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),(ushort *)param_3,param_4,
                  uVar1,(ushort)uVar1,uVar2);
  *param_4 = *(ushort *)((int)puVar3 + 0x1e);
  return;
}
