

void __stdcall16far win_sys_op_1008_84f2(UINT16 param_1,uint param_2,int param_3,ulong param_4,HWND16 param_5)

{
  byte *pbVar1;
  int iVar2;
  int iVar3;
  BOOL16 BVar4;
  uint uVar5;
  undefined2 uVar6;
  astruct_18 *paVar7;
  char cVar8;
  RECT16 local_a;
  int iStack4;
  
  paVar7 = (astruct_18 *)GetWindowLong16(param_5,0x0);
  uVar6 = (undefined2)((ulong)paVar7 >> 0x10);
  iVar3 = (int)paVar7;
  if ((uint)param_4 == 0x1f) {
    *(undefined2 *)(iVar3 + 0x4) = 0x0;
    KillTimer16((HWND16)s_tile2_bmp_1050_1538,0xfa6);
    KillTimer16((HWND16)s_tile2_bmp_1050_1538,0xfa7);
    ReleaseCapture16();
  }
  else {
    cVar8 = (char)param_4;
    if ((uint)param_4 < 0x20) {
      if ((uint)param_4 != 0x14) {
        if (0x14 < (uint)param_4) goto LAB_1008_8771;
        uVar5 = (uint)param_4 & 0xff00 | (uint)(byte)(cVar8 - 0x1U);
        if ((byte)(cVar8 - 0x1U) == 0x0) {
LAB_1008_8560:
          win_ui_op_1008_8214(param_4._2_2_,(uint)param_4,param_3,CONCAT22(param_2,param_1),uVar5,uVar6,
                              (int)s_tile2_bmp_1050_1538);
          return;
        }
        if (cVar8 == '\x02') {
          fn_ptr_1000_17ce(paVar7,0x1000);
        }
        else {
          if (cVar8 != '\f') {
            if (cVar8 != '\x0f') goto LAB_1008_8771;
            draw_op_1008_8288(param_4._2_2_,(ulong)paVar7,(int)s_tile2_bmp_1050_1538);
          }
        }
      }
    }
    else {
      if ((uint)param_4 == 0x200) {
        if ((*(byte *)(iVar3 + 0x4) & 0x1) != 0x0) {
          GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_a);
          iVar2 = *(int *)(iVar3 + 0x4);
          pbVar1 = (byte *)(iVar3 + 0x4);
          *pbVar1 = *pbVar1 & 0xf3;
          BVar4 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,(POINT16)CONCAT22(param_2,param_1));
          if (BVar4 == 0x0) {
            pbVar1 = (byte *)(iVar3 + 0x4);
            *pbVar1 = *pbVar1 | 0x2;
          }
          else {
            if ((int)param_2 < iStack4 >> 0x1) {
              pbVar1 = (byte *)(iVar3 + 0x4);
              *pbVar1 = *pbVar1 | 0x4;
            }
            else {
              pbVar1 = (byte *)(iVar3 + 0x4);
              *pbVar1 = *pbVar1 | 0x8;
            }
            pbVar1 = (byte *)(iVar3 + 0x4);
            *pbVar1 = *pbVar1 & 0xfd;
          }
          if (*(int *)(iVar3 + 0x4) != iVar2) {
            InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),0x0);
            UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
          }
        }
      }
      else {
        if ((uint)param_4 < 0x201) {
          uVar5 = (uint)param_4 - 0x81;
          if (uVar5 == 0x0) goto LAB_1008_8560;
          if ((uint)param_4 != 0x113) {
LAB_1008_8771:
            DefWindowProc16((HWND16)s_tile2_bmp_1050_1538,param_1,param_2,
                            CONCAT13((char)(param_4 >> 0x8),CONCAT12(cVar8,param_3)));
            return;
          }
          if (param_3 == 0xfa6) {
            KillTimer16((HWND16)s_tile2_bmp_1050_1538,0xfa6);
            SetTimer16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,(LPVOID)((int)&PTR_LOOP_1050_0000 + 0x1));
          }
          if ((*(byte *)(iVar3 + 0x4) & 0x2) == 0x0) {
            send_msg_1008_84ba(param_4._2_2_,(ulong)paVar7,(int)s_tile2_bmp_1050_1538);
          }
        }
        else {
          if ((uint)param_4 != 0x201) {
            if ((uint)param_4 == 0x202) {
              KillTimer16((HWND16)s_tile2_bmp_1050_1538,0xfa6);
              KillTimer16((HWND16)s_tile2_bmp_1050_1538,0xfa7);
              ReleaseCapture16();
              uVar5 = *(uint *)(iVar3 + 0x4);
              if (((uVar5 & 0x1) != 0x0) && ((uVar5 & 0xfffd) != 0x0)) {
                pbVar1 = (byte *)(iVar3 + 0x4);
                *pbVar1 = *pbVar1 & 0xf2;
                InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),0x0);
                UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
              }
              SendMessage16((HWND16)s_tile2_bmp_1050_1538,*(UINT16 *)(iVar3 + 0x2),0x0,0x11100f9);
              return;
            }
            if ((uint)param_4 != 0x203) goto LAB_1008_8771;
          }
          pbVar1 = (byte *)(iVar3 + 0x4);
          *pbVar1 = *pbVar1 | 0x1;
          GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_a);
          if (param_2 < (uint)(iStack4 >> 0x1)) {
            pbVar1 = (byte *)(iVar3 + 0x4);
            *pbVar1 = *pbVar1 | 0x4;
          }
          else {
            pbVar1 = (byte *)(iVar3 + 0x4);
            *pbVar1 = *pbVar1 | 0x8;
          }
          send_msg_1008_84ba(param_4._2_2_,(ulong)paVar7,(int)s_tile2_bmp_1050_1538);
          SetTimer16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,
                     (LPVOID)((int)s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x1c));
          InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),0x0);
          UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
          SetCapture16((HWND16)s_tile2_bmp_1050_1538);
        }
      }
    }
  }
  return;
}



ulong __stdcall16far pass1_1008_87a2(ulong param_1,byte param_2)

{
  pass1_1008_8168((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1008_87cc(astruct_86 *param_1,int param_2,int param_3,ushort param_4,ulong param_5,ulong param_6,ushort param_7)

{
  long lVar1;
  uint uVar2;
  BOOL16 BVar3;
  int *piVar4;
  uchar *puVar5;
  astruct_86 *iVar5;
  int iVar6;
  int unaff_DI;
  undefined2 uVar7;
  undefined2 uVar8;
  ushort *puVar9;
  int *piStack48;
  undefined4 local_24;
  ushort uStack32;
  ulong uStack30;
  astruct_18 *paStack26;
  undefined4 uStack18;
  int iStack14;
  int iStack12;
  int iStack10;
  int iStack8;
  ulong uStack6;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_86 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar5->field_0x2 = 0x1008;
  iVar5->field_0x4 = (astruct_76 *)param_5;
  *(undefined4 *)&iVar5->field_0x8 = 0x0;
  iVar5->field_0xc = param_3;
  iVar5->field_0xe = param_2;
  iVar5->field_0x10 = 0x0;
  iVar5->field_0x12 = 0x0;
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x1c)));
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x22));
  puVar9 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x28));
  iVar5->field_0x2e = param_4;
  iVar5->field_0x30 = 0xffff;
  iVar5->field_0x3a = 0x0;
  iVar5->field_0x3e = 0x1;
  iVar5->field_0x40 = 0x1;
  iVar5->field_0x42 = param_6;
  param_1->field_0x0 = 0x8e9a;
  iVar5->field_0x2 = 0x1008;
  if (_PTR_LOOP_1050_0382 == (ushort *)0x0) {
    _PTR_LOOP_1050_0382 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2e,param_7,(uchar *)((ulong)puVar9 >> 0x10),unaff_DI);
  }
  uStack6 = pass1_1008_4772(iVar5->field_0x4);
  iVar5->field_0x12 = 0x2f - *(int *)((int)uStack6 + 0x8);
  uVar8 = (undefined2)((ulong)_PTR_LOOP_1050_0382 >> 0x10);
  iVar6 = (int)_PTR_LOOP_1050_0382;
  iStack8 = *(int *)(iVar6 + 0xa);
  iStack10 = *(int *)(iVar6 + 0xc);
  iStack12 = *(int *)(iVar6 + 0xe);
  iStack14 = *(int *)(iVar6 + 0x10);
  iVar6 = iVar5->field_0xc;
  lVar1 = (long)(iVar6 + iVar5->field_0xe) * (long)iStack14;
  puVar5 = (uchar *)((ulong)lVar1 >> 0x10);
  pass1_1008_3e76((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x1c)),0x0,
                  (int)lVar1 + iVar5->field_0x12 + iStack10,
                  (iVar6 - iVar5->field_0xe) * iStack12 + iVar5->field_0x10 + iStack8);
  iVar5->field_0x14 = iVar5->field_0x1c + 0x20;
  iVar5->field_0x16 = *(int *)((int)uStack6 + 0x8) + iVar5->field_0x1e + -0x25;
  iVar5->field_0x18 = iVar5->field_0x14 + 0x32;
  uVar2 = iVar5->field_0x16 + 0x19;
  iVar5->field_0x1a = uVar2;
  mem_op_1000_179c(0x6,puVar5,0x1000);
  paStack26 = (astruct_18 *)CONCAT22(puVar5,uVar2);
  uStack18._2_2_ = (uint)puVar5 | uVar2;
  if (uStack18._2_2_ == 0x0) {
    *(undefined4 *)&iVar5->field_0x8 = 0x0;
  }
  else {
    puVar9 = pass1_1008_ada2((ushort *)CONCAT22(puVar5,uVar2),iVar5->field_0x2e);
    uStack18._2_2_ = (uint)((ulong)puVar9 >> 0x10);
    iVar5->field_0x8 = (uint)puVar9;
    iVar5->field_0xa = uStack18._2_2_;
  }
  BVar3 = pass1_1008_aed8(*(ulong *)&iVar5->field_0x8);
  if (BVar3 == 0x0) {
    paStack26 = *(astruct_18 **)&iVar5->field_0x8;
    uStack18 = paStack26;
    fn_ptr_1000_17ce(paStack26,0x1000);
    *(undefined4 *)&iVar5->field_0x8 = 0x0;
  }
  else {
    piVar4 = *(int **)&iVar5->field_0x8;
    pass1_1018_20ee((ulong)_PTR_LOOP_1050_0382,piVar4);
    uStack18._0_2_ = SUB42(piVar4,0x0);
    pass1_1008_add2(*(ushort **)&iVar5->field_0x8);
    uStack30 = pass1_1008_4772((astruct_76 *)CONCAT22(uStack18._2_2_,(undefined2)uStack18));
    pass1_1018_214e((ushort)_PTR_LOOP_1050_0382,(ushort)((ulong)_PTR_LOOP_1050_0382 >> 0x10),
                    (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x28),iVar5->field_0x2e);
    local_24 = *(undefined4 *)&iVar5->field_0x1c;
    uStack32 = iVar5->field_0x20;
    pass1_1008_3f32((int *)CONCAT22(param_7,&local_24),
                    (int *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x28));
    piStack48 = (int *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x32);
    pass1_1008_3e94((ushort *)CONCAT22(param_7,&local_24),
                    (ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x34)),
                    (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x32));
    uVar8 = (undefined2)(uStack30 >> 0x10);
    iVar5->field_0x36 = *(int *)((int)uStack30 + 0x4) + *piStack48;
    uVar2 = *(int *)((int)uStack30 + 0x8) + iVar5->field_0x34;
    iVar5->field_0x38 = uVar2;
    pass1_1008_612e(0x2,0x5,uVar2);
    iVar5->field_0x3e = uVar2;
  }
  return;
}



void __stdcall16far pass1_1008_8aa2(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  uint uVar3;
  code **ppcVar4;
  undefined4 uVar5;
  astruct_462 *iVar6;
  undefined2 uVar6;
  astruct_18 *paStack16;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (astruct_462 *)param_1;
  *param_1 = 0x8e9a;
  iVar6->field_0x2 = 0x1008;
  uVar5 = *(undefined4 *)&iVar6->field_0x4;
  if (*(int *)((int)uVar5 + 0x1c) != 0x0) {
    puVar1 = iVar6->field_0x4;
    uVar2 = iVar6->field_0x6;
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
  }
  uVar2 = iVar6->field_0x3a;
  uVar3 = iVar6->field_0x3c;
  paStack16 = (astruct_18 *)CONCAT22(uVar3,uVar2);
  if ((uVar3 | uVar2) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar3,uVar2));
    fn_ptr_1000_17ce(paStack16,0x1000);
  }
  *param_1 = 0x389a;
  iVar6->field_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_8b20(ulong param_1,ushort param_2)

{
  int iVar1;
  int *piVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  undefined local_a [0x2];
  undefined local_8 [0x2];
  astruct_76 *paStack6;
  
  uVar5 = (uint)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x8) != 0x0) {
    iVar1 = *(int *)(iVar4 + 0x40);
    piVar2 = (int *)(iVar4 + 0x40);
    *piVar2 = *piVar2 + 0x1;
    uVar3 = iVar1 % *(int *)(iVar4 + 0x3e);
    if (uVar3 == 0x0) {
      *(undefined2 *)(iVar4 + 0x40) = 0x1;
      piVar2 = *(int **)(iVar4 + 0x8);
      pass1_1018_20ee(_PTR_LOOP_1050_0382,piVar2);
      paStack6 = (astruct_76 *)((ulong)piVar2 & 0xffff | (ulong)uVar3 << 0x10);
      pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar4 + 0x28U)),(ushort *)CONCAT22(param_2,local_a),
                      (ushort *)CONCAT22(param_2,local_8));
      pass1_1008_8d8a(param_1 & 0xffff | (ulong)uVar5 << 0x10,paStack6,*(ulong *)(iVar4 + 0x4));
      pass1_1008_4480(*(ulong *)(iVar4 + 0x4),(ushort *)(param_1 & 0xffff0000 | (ulong)(iVar4 + 0x28U)),paStack6,param_2
                     );
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_8bc6(ushort param_1,uint param_2,ulong param_3)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  undefined local_a [0x2];
  undefined local_8 [0x2];
  astruct_76 *paStack6;
  
  uVar3 = (uint)(param_3 >> 0x10);
  iVar2 = (int)param_3;
  if (*(long *)(iVar2 + 0x8) == 0x0) {
    return;
  }
  piVar1 = *(int **)(iVar2 + 0x8);
  pass1_1018_20ee(_PTR_LOOP_1050_0382,piVar1);
  paStack6 = (astruct_76 *)((ulong)piVar1 & 0xffff | (ulong)param_2 << 0x10);
  pass1_1008_3e94((ushort *)(param_3 & 0xffff0000 | (ulong)(iVar2 + 0x28U)),(ushort *)CONCAT22(param_1,local_a),
                  (ushort *)CONCAT22(param_1,local_8));
  pass1_1008_8d8a(param_3 & 0xffff | (ulong)uVar3 << 0x10,paStack6,*(ulong *)(iVar2 + 0x4));
  pass1_1008_4480(*(ulong *)(iVar2 + 0x4),(ushort *)(param_3 & 0xffff0000 | (ulong)(iVar2 + 0x28U)),paStack6,param_1);
  return;
}



void __stdcall16far pass1_1008_8c4e(ulong param_1,ulong param_2,ushort param_3)

{
  undefined2 uVar1;
  uint *puVar2;
  uchar *puVar3;
  uchar *puVar4;
  uint uVar5;
  int iVar6;
  undefined2 uVar7;
  ulong uVar8;
  astruct_110 *paStack14;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar8 = pass1_1008_4772(*(astruct_76 **)(iVar6 + 0x4));
  puVar3 = (uchar *)(uVar8 >> 0x10);
  uVar5 = 0x0;
  if ((*(int *)(iVar6 + 0xc) == 0x0) || (*(int *)(iVar6 + 0xe) == 0x0)) {
    puVar4 = puVar3;
    mem_op_1000_179c(0x14,puVar3,0x1000);
    paStack14 = (astruct_110 *)CONCAT22(puVar4,uVar5);
    uVar5 = (uint)puVar4 | uVar5;
    if (uVar5 == 0x0) {
      uVar1 = 0x0;
      uVar5 = 0x0;
    }
    else {
      puVar2 = (uint *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x1c));
      pass1_1008_50c2(paStack14,*(ulong *)((int)uVar8 + 0x8),*(ulong *)((int)uVar8 + 0x4),puVar2,param_2);
      uVar1 = SUB42(puVar2,0x0);
    }
    pass1_1008_5134(CONCAT22(uVar5,uVar1));
  }
  pass1_1008_4480(param_2,(ushort *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x1c)),*(astruct_76 **)(iVar6 + 0x4),param_3
                 );
  return;
}



void __stdcall16far pass1_1008_8ce4(ulong param_1,ushort *param_2,ulong param_3,ushort param_4)

{
  undefined *puVar1;
  uchar *puVar2;
  uint uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  ushort *puVar7;
  undefined local_10 [0x6];
  undefined4 uStack10;
  ulong uStack6;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uStack6 = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x4));
  uStack10 = 0x0;
  puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_4,local_10),0x0,*(ushort *)(iVar4 + 0x12),*(ushort *)(iVar4 + 0x10))
  ;
  puVar2 = (uchar *)((ulong)puVar7 >> 0x10);
  puVar1 = local_10;
  pass1_1008_3f32((int *)param_2,(int *)CONCAT22(param_4,puVar1));
  mem_op_1000_179c(0x14,puVar2,0x1000);
  uVar3 = (uint)puVar2 | (uint)puVar1;
  if (uVar3 == 0x0) {
    puVar1 = (undefined *)0x0;
    uVar3 = 0x0;
  }
  else {
    uVar6 = (undefined2)(uStack6 >> 0x10);
    pass1_1008_50c2((astruct_110 *)CONCAT22(puVar2,puVar1),*(ulong *)((int)uStack6 + 0x8),*(ulong *)((int)uStack6 + 0x4)
                    ,param_2,param_3);
  }
  uStack10 = CONCAT22(uVar3,puVar1);
  pass1_1008_5134(CONCAT22(uVar3,puVar1));
  pass1_1008_4480(param_3,param_2,*(astruct_76 **)(iVar4 + 0x4),param_4);
  return;
}



void __stdcall16far pass1_1008_8d8a(ulong param_1,astruct_76 *param_2,ulong param_3)

{
  uint uVar1;
  char cVar2;
  uint *puVar3;
  uchar *puVar4;
  uchar *puVar5;
  uint uVar6;
  astruct_112 *iVar7;
  undefined2 uVar7;
  ulong uVar8;
  astruct_110 *paStack10;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar7 = (astruct_112 *)param_1;
  uVar1 = iVar7->field_0x2e;
  if ((int)uVar1 < 0x28) {
    if (((int)uVar1 < 0x25) && (uVar1 != 0x23)) {
      if (0x23 < uVar1) {
        return;
      }
      cVar2 = (char)uVar1;
      if (((cVar2 != '\v') && (cVar2 != '\x0f')) && (cVar2 != '!')) {
        return;
      }
    }
  }
  else {
    if ((int)uVar1 < 0x46) {
      if ((int)uVar1 < 0x43) {
        if ((int)uVar1 < 0x33) {
          return;
        }
        if ((uVar1 != 0x34 && 0x0 < (int)(uVar1 - 0x33)) && (uVar1 != 0x37)) {
          return;
        }
      }
    }
    else {
      if (uVar1 != 0x49) {
        if ((int)(uVar1 - 0x49) < 0x2a) {
          return;
        }
        if (0x5 < (int)(uVar1 - 0x73)) {
          return;
        }
      }
    }
  }
  if (iVar7->field_0x3a == 0x0) {
    uVar8 = pass1_1008_4772(param_2);
    puVar4 = (uchar *)(uVar8 >> 0x10);
    uVar1 = (uint)uVar8;
    puVar5 = puVar4;
    uVar6 = uVar1;
    mem_op_1000_179c(0x14,puVar4,0x1000);
    paStack10 = (astruct_110 *)CONCAT22(puVar5,uVar6);
    uVar6 = (uint)puVar5 | uVar6;
    if (uVar6 == 0x0) {
      iVar7->field_0x3a = 0x0;
    }
    else {
      puVar3 = (uint *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar7->field_0x28);
      pass1_1008_50c2(paStack10,*(ulong *)(uVar1 + 0x8),*(ulong *)(uVar1 + 0x4),puVar3,param_3);
      *(int *)&iVar7->field_0x3a = (int)puVar3;
      *(uint *)((int)&iVar7->field_0x3a + 0x2) = uVar6;
    }
    pass1_1008_5134(iVar7->field_0x3a);
    return;
  }
  pass1_1008_5236(iVar7->field_0x3a);
  return;
}



ushort * __stdcall16far pass1_1008_8e74(ushort *param_1,byte param_2)

{
  pass1_1008_8aa2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_op_1008_8e9e(astruct_78 *param_1,ulong param_2,ulong param_3)

{
  astruct_78 *iVar1;
  astruct_78 *uVar1;
  ushort unaff_SS;
  
  uVar1 = (astruct_78 *)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_78 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x6 = (ulong *)0x0;
  iVar1->field_0xa = 0x0;
  iVar1->field_0xe = param_3;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x16 = param_2;
  iVar1->field_0x1a = 0x1;
  param_1->field_0x0 = 0x9170;
  iVar1->field_0x2 = 0x1008;
  if (iVar1->field_0xe < 0x7) {
    iVar1->field_0xe = 0x6;
  }
  pass1_1008_909c((ulong)param_1,unaff_SS);
  *iVar1->field_0x6 = 0x0;
  return;
}



void __stdcall16far pass1_1008_8f24(ushort *param_1)

{
  ulong *puVar1;
  undefined4 *puVar2;
  uint uVar3;
  code **ppcVar4;
  undefined4 uVar5;
  astruct_463 *iVar6;
  int iVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  ulong uStack6;
  
  uVar9 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (astruct_463 *)param_1;
  *param_1 = 0x9170;
  iVar6->field_0x2 = 0x1008;
  if (iVar6->field_0x1a != 0x0) {
    uStack6 = 0x0;
    while( true ) {
      puVar1 = &iVar6->field_0xa;
      if (*puVar1 < uStack6 || *puVar1 == uStack6) break;
      iVar8 = (int)uStack6 * 0x4;
      uVar5 = iVar6->field_0x6;
      uVar10 = (undefined2)((ulong)uVar5 >> 0x10);
      iVar7 = (int)uVar5;
      puVar2 = (undefined4 *)*(uint *)(iVar7 + iVar8);
      uVar3 = *(uint *)(iVar7 + iVar8 + 0x2);
      if ((uVar3 | (uint)puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      uStack6 = uStack6 + 0x1;
    }
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar6->field_0x6,0x1000);
  *param_1 = 0x389a;
  iVar6->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1008_8faa(ulong param_1,ulong param_2)

{
  uint uVar1;
  
  uVar1 = (uint)(param_1 >> 0x10);
  pass1_1008_9004(param_1 & 0xffff | (ulong)uVar1 << 0x10,(ushort)param_2,(ushort)(param_2 >> 0x10),
                  *(ulong *)((int)param_1 + 0xa));
  return;
}



void __stdcall16far empty_1008_8fc4(void)

{
  return;
}



void __stdcall16far pass1_1008_9004(ulong param_1,ushort param_2,ushort param_3,ulong param_4)

{
  ulong *puVar1;
  uint *puVar2;
  long lVar3;
  astruct_107 *iVar4;
  astruct_108 *iVar5;
  uint uVar4;
  undefined2 uVar5;
  ushort unaff_SS;
  bool bVar6;
  
  uVar4 = (uint)(param_1 >> 0x10);
  iVar4 = (astruct_107 *)param_1;
  puVar1 = (ulong *)&iVar4->field_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4->field_0x6 == 0x0)) {
    puVar2 = (uint *)((int)&iVar4->field_0x12 + 0x2);
    bVar6 = *puVar2 < param_4._2_2_;
    if ((bVar6 || *puVar2 == param_4._2_2_) &&
       ((bVar6 || (puVar1 = &iVar4->field_0x12, *(uint *)puVar1 < (uint)param_4 || *(uint *)puVar1 == (uint)param_4))))
    {
      pass1_1008_909c(param_1 & 0xffff | (ulong)uVar4 << 0x10,unaff_SS);
    }
    puVar1 = &iVar4->field_0x12;
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4->field_0x6 == 0x0)) {
      return;
    }
    puVar2 = &iVar4->field_0xc;
    bVar6 = *puVar2 < param_4._2_2_;
    if ((bVar6 || *puVar2 == param_4._2_2_) &&
       ((bVar6 || (puVar2 = &iVar4->field_0xa, *puVar2 < (uint)param_4 || *puVar2 == (uint)param_4)))) {
      iVar4->field_0xa = (uint)(param_4 + 0x1);
      iVar4->field_0xc = (uint)(param_4 + 0x1 >> 0x10);
    }
  }
  lVar3 = iVar4->field_0x6;
  uVar5 = (undefined2)((ulong)lVar3 >> 0x10);
  iVar5 = (astruct_108 *)lVar3;
  *(ushort *)(iVar5 + (uint)param_4 * 0x4) = param_2;
  *(ushort *)(iVar5 + (uint)param_4 * 0x4 + 0x2) = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_909c(ulong param_1,ushort param_2)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  astruct_106 *iVar5;
  undefined2 uVar4;
  long lVar5;
  long lStack10;
  undefined4 uStack6;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar5 = (astruct_106 *)param_1;
  if (*(long *)&iVar5->field_0x12 == 0x0) {
    uVar3 = iVar5->field_0xe;
    PTR_LOOP_1050_5f2e = iVar5->field_0x10;
  }
  else {
    uVar2 = *(uint *)&iVar5->field_0x12;
    puVar1 = &iVar5->field_0x16;
    uVar3 = uVar2 + *puVar1;
    PTR_LOOP_1050_5f2e = (undefined *)(iVar5->field_0x14 + iVar5->field_0x18 + (uint)CARRY2(uVar2,*puVar1));
  }
  uStack6 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
  if (iVar5->field_0x6 == 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar3 = fn_ptr_op_1000_1708(uVar3 << 0x2,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
    lVar5 = iVar5->field_0x6;
    lVar5 = pass1_1000_0ed4(0x1000,param_2,0x1,uVar3 * 0x4,
                            ((int)PTR_LOOP_1050_5f2e * 0x2 + (uint)CARRY2(uVar3,uVar3)) * 0x2 +
                            (uint)CARRY2(uVar3 * 0x2,uVar3 * 0x2),(ushort *)lVar5,(ushort)((ulong)lVar5 >> 0x10));
    PTR_LOOP_1050_5f2e = (undefined *)((ulong)lVar5 >> 0x10);
    uVar3 = (uint)lVar5;
  }
  lStack10 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
  if (((uint)PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
    *(undefined4 *)&iVar5->field_0x12 = uStack6;
    iVar5->field_0x6 = lStack10;
  }
  return;
}



ushort * __stdcall16far pass1_1008_914a(ushort *param_1,byte param_2)

{
  pass1_1008_8f24(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_op_1008_9174(astruct_88 *param_1,ulong param_2,ulong param_3)

{
  astruct_88 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_88 *)param_1;
  *(undefined2 *)param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_3;
  iVar1->field_0x8 = param_2;
  iVar1->field_0xc = param_2;
  iVar1->field_0x10 = 0x0;
  *(undefined2 *)param_1 = 0x9412;
  iVar1->field_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort * __stdcall16far pass1_1008_91ba(ushort *param_1,HWND16 param_2)

{
  UINT16 UVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar2 + 0x2) = 0x1008;
  *(undefined2 *)(iVar2 + 0x4) = 0x0;
  set_struct_1008_574a((astruct_21 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar2 + 0x6)));
  *param_1 = 0x9416;
  *(undefined2 *)(iVar2 + 0x2) = 0x1008;
  _PTR_LOOP_1050_0388 = param_1;
  UVar1 = SetTimer16(param_2,0x0,0x0,(LPVOID)((int)&PTR_LOOP_1050_0000 + 0x1));
  if (UVar1 == 0x0) {
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  PTR_LOOP_1050_038a = (undefined *)((ulong)_PTR_LOOP_1050_0388 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far kill_timer_1008_921c(ushort *param_1,HWND16 param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x9416;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  KillTimer16(param_2,0x1);
  _PTR_LOOP_1050_0388 = 0x0;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x6)));
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}



void __stdcall16far pass1_1008_9262(int param_1,ushort param_2,ulong param_3,ulong param_4,uint param_5,uchar *param_6)

{
  code **ppcVar1;
  uint uVar2;
  
  mem_op_1000_179c(0x12,param_6,0x1000);
  uVar2 = (uint)param_6 | param_5;
  if (uVar2 == 0x0) {
    param_5 = 0x0;
    uVar2 = 0x0;
  }
  else {
    struct_op_1008_9174((astruct_88 *)CONCAT22(param_6,param_5),param_3,param_4);
  }
  if ((uVar2 | param_5) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)(param_1 + 0x6) + 0x4);
    (**ppcVar1)();
  }
  return;
}



void __stdcall16far pass1_1008_92b2(ulong param_1,long param_2,long param_3)

{
  code **ppcVar1;
  undefined *puVar2;
  uint extraout_DX;
  undefined2 unaff_SS;
  undefined local_c [0x4];
  undefined4 uStack8;
  undefined2 uStack4;
  
  uStack4 = 0x0;
  pass1_1008_57a4((ulong *)CONCAT22(unaff_SS,local_c),param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x6));
  while( true ) {
    puVar2 = local_c;
    pass1_1008_5b12(puVar2,unaff_SS);
    if ((extraout_DX | (uint)puVar2) == 0x0) break;
    if ((*(long *)(puVar2 + 0x4) == param_3) && (*(long *)(puVar2 + 0x8) == param_2)) {
      uStack4 = 0x1;
      ppcVar1 = (code **)((int)*(undefined4 *)((int)param_1 + 0x6) + 0xc);
      (**ppcVar1)();
      uStack8 = 0x0;
    }
  }
  return;
}



void __stdcall16far pass1_1008_932a(ulong param_1,ushort param_2)

{
  uint uVar1;
  code **ppcVar2;
  undefined *puVar3;
  uint extraout_DX;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  undefined local_a [0x8];
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(int *)(iVar5 + 0x4) == 0x0) {
    *(undefined2 *)(iVar5 + 0x4) = 0x1;
    pass1_1008_57a4((ulong *)CONCAT22(param_2,local_a),param_1 & 0xffff0000 | (ulong)(iVar5 + 0x6));
    while( true ) {
      puVar3 = local_a;
      pass1_1008_5b12(puVar3,param_2);
      if ((extraout_DX | (uint)puVar3) == 0x0) break;
      uVar1 = *(uint *)(puVar3 + 0xc);
      iVar4 = *(int *)(puVar3 + 0xe) - (uint)(uVar1 < 0x37);
      *(uint *)(puVar3 + 0xc) = uVar1 - 0x37;
      *(int *)(puVar3 + 0xe) = iVar4;
      if ((iVar4 < 0x1) && (((iVar4 < 0x0 || (*(int *)(puVar3 + 0xc) == 0x0)) && (*(int *)(puVar3 + 0x10) == 0x0)))) {
        ppcVar2 = (code **)((int)**(undefined4 **)(puVar3 + 0x4) + 0x4);
        (**ppcVar2)();
        *(undefined4 *)(puVar3 + 0xc) = *(undefined4 *)(puVar3 + 0x8);
      }
    }
    *(undefined2 *)(iVar5 + 0x4) = 0x0;
  }
  return;
}



ushort * __stdcall16far pass1_1008_93c0(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_93ec(ushort *param_1,byte param_2)

{
  undefined2 unaff_CS;
  
  kill_timer_1008_921c(param_1,unaff_CS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_941a(ushort *param_1,ushort param_2,ushort param_3)

{
  *param_1 = param_2;
  *(ushort *)((int)param_1 + 0x2) = param_3;
  return param_1;
}



ushort * __stdcall16far pass1_1008_9436(ushort *param_1)

{
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x2) = 0x0;
  return param_1;
}



void __stdcall16far pass1_1008_944e(ushort *param_1,ushort param_2,ushort param_3)

{
  *(ushort *)((int)param_1 + 0x2) = param_3;
  *param_1 = param_2;
  return;
}

