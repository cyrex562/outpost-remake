


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_50e0(ulong param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  ushort uVar2;
  BOOL16 BVar3;
  uint extraout_DX;
  uint uVar4;
  uint uVar5;
  int iVar6;
  undefined2 uVar7;
  ulong uVar8;
  ulong uStack14;
  ulong uStack10;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*(long *)(iVar6 + 0xc) == 0x0) {
    param_3 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_3);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 = uStack14 + 0x1) {
    uVar8 = uStack10;
    pass1_1030_1d58(*(ulong *)(iVar6 + 0xc));
    uVar5 = uVar4 | (uint)uVar8;
    if (uVar5 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar8 & 0xffff | (ulong)uVar4 << 0x10);
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,param_2);
      if (BVar3 != 0x0) {
        uVar8 = struct_op_1030_73a8(uVar8 & 0xffff | (ulong)uVar4 << 0x10);
        uVar5 = (uint)(uVar8 >> 0x10);
      }
    }
    uVar4 = uVar5;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_518c(ulong param_1,ushort param_2,ushort param_3)

{
  uint *puVar1;
  undefined4 uVar2;
  code **ppcVar3;
  uint uVar4;
  ulong uVar5;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  bool bVar12;
  ulong uVar13;
  int iStack34;
  ulong uStack32;
  ulong *puStack28;
  ulong uStack10;
  ulong uStack6;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar7 = (int)param_1;
  if (*(int *)(iVar7 + 0x206) == 0x0) {
    if (*(long *)(iVar7 + 0xc) == 0x0) {
      param_2 = 0x0;
      uVar11 = 0x0;
    }
    else {
      uVar2 = *(undefined4 *)(iVar7 + 0xc);
      ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x10);
      (**ppcVar3)(param_3,(int)uVar2,(int)((ulong)uVar2 >> 0x10));
      uVar11 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar11,param_2);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1) {
      uVar2 = *(undefined4 *)(iVar7 + 0xc);
      ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x4);
      uVar5 = uStack6;
      (**ppcVar3)((char)param_3,(char)uVar2,(int)((ulong)uVar2 >> 0x10),(int)uStack10,(int)(uStack10 >> 0x10));
      uVar4 = (uint)uVar5;
      uVar6 = extraout_DX_00 | uVar4;
      if (uVar6 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,extraout_DX_00);
        param_3 = 0x1030;
        uVar13 = struct_op_1030_73a8(CONCAT22(uVar6,uVar4));
        uVar6 = (uint)(uVar13 >> 0x10);
        iVar8 = *(int *)((int)uVar13 + 0x12);
        uVar4 = (int)uVar13 + 0x14;
        uVar5 = (ulong)uVar4;
        puStack28 = (ulong *)(uVar13 & 0xffff0000 | (ulong)uVar4);
        uStack32 = 0x0;
        if ((iVar8 == 0x4) || (iVar8 == 0x5)) {
          uVar5 = *puStack28;
          uStack32 = uVar5;
        }
        if (uStack32 != 0x0) {
          for (iStack34 = 0x11; iStack34 < 0x25; iStack34 = iStack34 + 0x1) {
            if (((*(int *)(iVar7 + 0x204) == 0x0) || (iStack34 == 0x23)) || (iStack34 == 0x24)) {
              empty_1038_540a();
              iVar8 = iStack34 * 0x4;
              uVar11 = (undefined2)(uStack32 >> 0x10);
              iVar9 = (int)uStack32;
              puVar1 = (uint *)(iVar8 + iVar9 + 0x2);
              bVar12 = *puVar1 < uVar6;
              if ((bVar12 || *puVar1 == uVar6) &&
                 ((bVar12 || (puVar1 = (uint *)(iVar8 + iVar9), *puVar1 < (uint)uVar5 || *puVar1 == (uint)uVar5)))) {
                pass1_1038_5770(param_1,*(long *)(iVar8 + iVar9),iStack34);
              }
            }
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_52b8(ulong param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6,ushort param_7)

{
  undefined4 uVar1;
  code **ppcVar2;
  ulong uVar3;
  int iVar4;
  uint uVar5;
  undefined2 extraout_DX;
  undefined2 uVar6;
  uint extraout_DX_00;
  uint uVar7;
  undefined2 uVar8;
  ulong uVar9;
  int iVar11;
  ushort uVar12;
  uint uStack26;
  int iStack24;
  ulong uStack22;
  ulong uStack14;
  ulong uStack10;
  astruct_601 *iVar10;
  
  iVar4 = -(int)param_2;
  iVar11 = (int)param_1;
  pass1_1038_5694(param_1,CONCAT22(-(param_2._2_2_ + (uint)((int)param_2 != 0x0)),iVar4),param_3);
  if (param_3 != 0x24) {
    uVar8 = (undefined2)(param_1 >> 0x10);
    if (*(long *)(iVar11 + 0xc) == 0x0) {
      iVar4 = 0x0;
      uVar6 = 0x0;
    }
    else {
      uVar1 = *(undefined4 *)(iVar11 + 0xc);
      ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar11 + 0xc) + 0x10);
      (**ppcVar2)(param_6,(int)uVar1,(int)((ulong)uVar1 >> 0x10));
      uVar6 = extraout_DX;
    }
    uStack10 = CONCAT22(uVar6,iVar4);
    for (uStack14 = 0x0; uVar3 = param_2, uStack14 < uStack10; uStack14 = uStack14 + 0x1) {
      uVar1 = *(undefined4 *)(iVar11 + 0xc);
      ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar11 + 0xc) + 0x4);
      uVar9 = uStack10;
      (**ppcVar2)(param_6,(char)uVar1,(int)((ulong)uVar1 >> 0x10),(int)uStack14,(int)(uStack14 >> 0x10));
      uVar5 = (uint)uVar9;
      uVar7 = extraout_DX_00 | uVar5;
      if (uVar7 != 0x0) {
        uVar12 = param_3;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,extraout_DX_00);
        uStack22 = CONCAT22(uVar7,uVar5);
        param_6 = 0x1030;
        uVar9 = pass1_1030_7c28(CONCAT22(uVar7,uVar5),uVar12,uVar5,uVar7,param_7);
        uVar7 = (uint)(uVar9 >> 0x10);
        uVar5 = (uint)uVar9;
        if ((uVar7 | uVar5) != 0x0) {
          if (uVar9 < param_2) {
            param_2 = param_2 - uVar9;
            uStack26 = 0x0;
            iStack24 = 0x0;
          }
          else {
            uStack26 = uVar5 - (uint)param_2;
            iStack24 = (uVar7 - param_2._2_2_) - (uint)(uVar5 < (uint)param_2);
            param_2 = 0x0;
            uVar9 = uVar3;
          }
          param_6 = 0x1030;
          pass1_1030_7d1c(uStack22,uStack26,CONCAT22(param_3,iStack24),(int)uVar9,param_2._2_2_,param_4,param_5,param_7)
          ;
          if (param_2 == 0x0) {
            return;
          }
        }
      }
    }
  }
  return;
}



void __stdcall16far pass1_1038_53ba(ulong param_1,int param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  if (*(ulong *)((int)param_1 + 0x1a2 + param_2 * 0x4) < *(ulong *)((int)param_1 + 0x14e + param_2 * 0x4)) {
    return;
  }
  return;
}



void __stdcall16far empty_1038_540a(void)

{
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_5464(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  undefined4 uVar1;
  code **ppcVar2;
  uint uVar3;
  ulong uVar4;
  uint extraout_DX;
  uint extraout_DX_00;
  uint extraout_DX_01;
  uint extraout_DX_02;
  uint uVar5;
  int iVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  undefined2 local_2e;
  undefined2 uStack44;
  uint local_2a;
  uint uStack40;
  undefined4 *puStack34;
  uint uStack30;
  uint uStack28;
  ulong *puStack26;
  undefined4 uStack22;
  uint uStack18;
  uint uStack16;
  ulong uStack14;
  ulong uStack10;
  undefined4 uStack6;
  
  pass1_1038_56ba(param_1);
  pass1_1038_57c0(param_1);
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*(long *)(iVar6 + 0xc) == 0x0) {
    param_2 = 0x0;
    uVar5 = 0x0;
  }
  else {
    uVar1 = *(undefined4 *)(iVar6 + 0xc);
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x10);
    (**ppcVar2)(param_3,(int)uVar1,(int)((ulong)uVar1 >> 0x10));
    uVar5 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar5,param_2);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 = uStack14 + 0x1) {
    uVar1 = *(undefined4 *)(iVar6 + 0xc);
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x4);
    uVar4 = uStack10;
    (**ppcVar2)(param_3,(char)uVar1,(int)((ulong)uVar1 >> 0x10),(int)uStack14,(int)(uStack14 >> 0x10));
    uVar3 = (uint)uVar4;
    uVar5 = extraout_DX_02 | uVar3;
    uStack18 = uVar3;
    uStack16 = extraout_DX_02;
    if (uVar5 != 0x0) {
      param_3 = (ushort)&USHORT_1050_1028;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_02);
      uStack22 = CONCAT22(uVar5,uVar3);
      puStack26 = *(ulong **)(uVar3 + 0x22);
      if ((*(uint *)(uVar3 + 0x24) | (uint)puStack26) == 0x0) {
        uStack28 = 0x0;
      }
      else {
        uStack28 = *(uint *)((uint)puStack26 + 0x4);
      }
      for (uStack30 = 0x0; uStack30 < uStack28; uStack30 = uStack30 + 0x1) {
        param_3 = 0x1020;
        pass1_1020_bb16(puStack26,(ulong *)CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,&local_2e)),
                        (ushort *)CONCAT22(param_4,&local_2a),uStack30);
        if (CONCAT22(uStack44,local_2e) != 0x0) {
          pass1_1038_5694(param_1,CONCAT22(uStack44,local_2e),local_2a);
        }
      }
      uVar9 = (undefined2)((ulong)uStack22 >> 0x10);
      puStack34 = (undefined4 *)*(undefined4 *)((int)uStack22 + 0x1e);
      uVar5 = *(uint *)((int)uStack22 + 0x20);
      uVar3 = uVar5 | (uint)puStack34;
      if (uVar3 == 0x0) {
        uVar3 = 0x0;
      }
      else {
        ppcVar2 = (code **)((int)*puStack34 + 0x10);
        (**ppcVar2)(param_3,(uint)puStack34,uVar5);
        uVar5 = extraout_DX_00;
      }
      uStack28 = uVar3;
      for (uStack30 = 0x0; uStack30 < uStack28; uStack30 = uStack30 + 0x1) {
        ppcVar2 = (code **)((int)*puStack34 + 0x4);
        uVar3 = uStack28;
        (**ppcVar2)(param_3,(char)puStack34,(int)((ulong)puStack34 >> 0x10),uStack30,0x0);
        uVar5 = extraout_DX_01 | uVar3;
        local_2a = uVar3;
        uStack40 = extraout_DX_01;
        if (uVar5 != 0x0) {
          param_3 = (ushort)&USHORT_1050_1028;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_01);
          iVar7 = *(int *)(uVar3 + 0xc) * 0x4;
          *(long *)(iVar6 + iVar7 + 0x14e) = *(long *)(iVar6 + 0x14e + iVar7) + 0x1;
        }
      }
    }
  }
  uVar4 = uStack10;
  pass1_1030_38f2(*(ulong *)(iVar6 + 0x1f6),0x3,param_4);
  uVar3 = (uint)uVar4;
  uStack6._0_2_ = uVar3;
  uStack6._2_2_ = uVar5;
  pass1_1030_38f2(*(ulong *)(iVar6 + 0x1f6),0x4,param_4);
  uStack6 = CONCAT22(uStack6._2_2_ + uVar5 + (uint)CARRY2((uint)uStack6,uVar3),(uint)uStack6 + uVar3);
  if (uStack6 == 0x0) {
    pass1_1030_38f2(*(ulong *)(iVar6 + 0x1f6),0x2,param_4);
    uStack6 = CONCAT22(uVar5,uVar3);
  }
  uVar1 = *(undefined4 *)(iVar6 + 0x1f6);
  uStack6 = uStack6 + *(long *)((int)uVar1 + 0x170);
  pass1_1038_5694(param_1,uStack6,0x24);
  return;
}



ulong __stdcall16far pass1_1038_565e(ushort param_1,uchar *param_2,ulong param_3)

{
  int iVar1;
  undefined2 uVar2;
  ulong uVar3;
  undefined local_4 [0x2];
  
  uVar2 = (undefined2)(param_3 >> 0x10);
  iVar1 = (int)param_3;
  uVar3 = pass1_1030_8e3c(param_1,(uint)local_4,param_2,CONCAT22(param_1,local_4),*(ulong *)(iVar1 + 0x4));
  pass1_1038_582c(param_3,uVar3);
  return CONCAT22(*(undefined2 *)(iVar1 + 0x16),*(undefined2 *)(iVar1 + 0x14));
}



void __stdcall16far pass1_1038_5694(ulong param_1,long param_2,int param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(long *)((int)param_1 + param_3 * 0x4 + 0x26) = *(long *)((int)param_1 + 0x26 + param_3 * 0x4) + param_2;
  return;
}



void __stdcall16far pass1_1038_56ba(ulong param_1)

{
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x26)),(WNDCLASS16 *)0x0,0x94);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_56d6(ulong param_1,int param_2)

{
  code **ppcVar1;
  int iVar2;
  uint *puVar3;
  uint uVar4;
  ulong uVar5;
  undefined2 extraout_DX;
  undefined2 uVar6;
  uint extraout_DX_00;
  uint uVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  ulong uStack10;
  ulong uStack6;
  
  iVar2 = (int)param_1;
  uVar9 = 0x1000;
  puVar3 = pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)(iVar2 + 0xba)),(WNDCLASS16 *)0x0,0x94);
  if (param_2 != 0x0) {
    uVar8 = (undefined2)(param_1 >> 0x10);
    if (*(long *)(iVar2 + 0xc) == 0x0) {
      puVar3 = (uint *)0x0;
      uVar6 = 0x0;
    }
    else {
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar2 + 0xc) + 0x10);
      (**ppcVar1)();
      uVar6 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar6,puVar3);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1) {
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar2 + 0xc) + 0x4);
      uVar5 = uStack6;
      (**ppcVar1)(uVar9,*(undefined4 *)(iVar2 + 0xc));
      uVar4 = (uint)uVar5;
      uVar7 = extraout_DX_00 | uVar4;
      if (uVar7 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,extraout_DX_00);
        uVar9 = 0x1030;
        pass1_1030_72d0(CONCAT22(uVar7,uVar4));
      }
    }
  }
  return;
}



void __stdcall16far pass1_1038_5770(ulong param_1,long param_2,int param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(long *)((int)param_1 + param_3 * 0x4 + 0xba) = *(long *)((int)param_1 + 0xba + param_3 * 0x4) + param_2;
  return;
}



void __stdcall16far pass1_1038_5798(ulong param_1,long param_2,int param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(long *)((int)param_1 + param_3 * 0x4 + 0x14e) = *(long *)((int)param_1 + 0x14e + param_3 * 0x4) + param_2;
  return;
}



void __stdcall16far pass1_1038_57c0(ulong param_1)

{
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x14e)),(WNDCLASS16 *)0x0,0x54);
  return;
}



void __stdcall16far pass1_1038_57dc(ulong param_1,long param_2,int param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(long *)((int)param_1 + param_3 * 0x4 + 0x1a2) = *(long *)((int)param_1 + 0x1a2 + param_3 * 0x4) + param_2;
  return;
}



void __stdcall16far pass1_1038_5804(ulong param_1,long param_2,int param_3)

{
  ushort uVar1;
  
  uVar1 = (ushort)(param_1 >> 0x10);
  *(long *)((int)param_1 + param_3 * 0x4 + 0x1a2) = *(long *)((int)param_1 + 0x1a2 + param_3 * 0x4) - param_2;
  return;
}



void __stdcall16far pass1_1038_582c(ulong param_1,ulong param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x14);
  uVar2 = *(uint *)(iVar4 + 0x16);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(ulong *)(iVar4 + 0x14) = param_2;
  return;
}



void __stdcall16far pass1_1038_5860(ulong param_1,ushort param_2,ulong param_3,int param_4)

{
  code **ppcVar1;
  ulong uVar2;
  ulong uVar3;
  uint extraout_DX;
  int extraout_DX_00;
  int iVar4;
  undefined2 uVar5;
  ulong uStack14;
  int iStack6;
  int iStack4;
  
  if (param_4 == 0x0) {
    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0xc) + 0x10);
    uVar2 = param_3;
    (**ppcVar1)();
    uVar2 = uVar2 & 0xffff | (ulong)extraout_DX << 0x10;
    for (uStack14 = 0x0; uStack14 < uVar2; uStack14 = uStack14 + 0x1) {
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0xc) + 0x4);
      uVar3 = uVar2;
      (**ppcVar1)();
      iStack6 = (int)param_3;
      if (((int)uVar3 == iStack6) && (iStack4 = (int)(param_3 >> 0x10), extraout_DX_00 == iStack4)) {
        return;
      }
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0xc) + 0xc);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_58e6(ushort param_1,ushort param_2,ulong param_3,ulong param_4,ulong param_5,int param_6,ushort param_7)

{
  int iVar1;
  code **ppcVar2;
  undefined4 uVar3;
  BOOL16 BVar4;
  undefined4 *puVar5;
  uint uVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  ulong uVar10;
  undefined4 local_12;
  int iStack14;
  int iStack12;
  ulong uStack6;
  
  for (uStack6 = 0x0; uStack6 < param_3; uStack6 = uStack6 + 0x1) {
    uVar9 = (undefined2)(param_4 >> 0x10);
    iVar7 = (int)param_4;
    if ((*(long *)((int)uStack6 * 0x4 + iVar7) != 0x0) &&
       (uVar3 = *(undefined4 *)((int)uStack6 * 0x4 + iVar7),
       BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((int)uVar3 + 0xc),0x2e), BVar4 != 0x0)) {
      uVar8 = (undefined2)(param_5 >> 0x10);
      iVar1 = *(int *)((int)uStack6 * 0x4 + (int)param_5);
      uVar8 = *(undefined2 *)((int)uStack6 * 0x4 + (int)param_5 + 0x2);
      local_12 = *(undefined4 *)(iVar1 + 0xc);
      iStack12 = *(int *)(iVar1 + 0x10);
      iStack14 = iStack12;
      if (iStack12 == param_6) {
        iStack14 = iStack12 + -0x1;
        uVar10 = pass1_1028_bb24(*(ulong *)((int)uStack6 * 0x4 + iVar7));
        uVar6 = (uint)(uVar10 >> 0x10);
        puVar5 = &local_12;
        pass1_1030_627e(param_7,(uint)puVar5,uVar6,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_7,puVar5),
                        uVar10 & 0xffff | (ulong)uVar6 << 0x10);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)puVar5,uVar6);
        if ((uVar6 | (uint)puVar5) != 0x0) {
          uVar10 = struct_op_1030_73a8(CONCAT22(uVar6,puVar5));
          uVar6 = *(uint *)((int)uVar10 + 0x1a);
          if (((uVar6 & 0x2) != 0x0) && ((uVar6 & 0x1) != 0x0)) {
            uVar3 = *(undefined4 *)((int)uStack6 * 0x4 + iVar7);
            *(undefined2 *)((int)uVar3 + 0x1a) = 0x3;
            ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uStack6 * 0x4 + iVar7) + 0x28);
            (**ppcVar2)();
          }
        }
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_5a16(ushort param_1,ushort param_2,ulong param_3,ulong param_4)

{
  code **ppcVar1;
  undefined4 uVar2;
  BOOL16 BVar3;
  int iVar4;
  undefined2 uVar5;
  ulong uStack6;
  
  for (uStack6 = 0x0; uStack6 < param_3; uStack6 = uStack6 + 0x1) {
    uVar5 = (undefined2)(param_4 >> 0x10);
    iVar4 = (int)param_4;
    if ((*(long *)((int)uStack6 * 0x4 + iVar4) != 0x0) &&
       (uVar2 = *(undefined4 *)((int)uStack6 * 0x4 + iVar4),
       BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((int)uVar2 + 0xc),0x2f), BVar3 != 0x0)) {
      uVar2 = *(undefined4 *)((int)uStack6 * 0x4 + iVar4);
      *(undefined2 *)((int)uVar2 + 0x1a) = 0x3;
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uStack6 * 0x4 + iVar4) + 0x28);
      (**ppcVar1)();
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_5a96(ushort param_1,ushort param_2,ulong param_3,ulong param_4)

{
  code **ppcVar1;
  undefined4 uVar2;
  BOOL16 BVar3;
  int iVar4;
  undefined2 uVar5;
  ulong uStack6;
  
  for (uStack6 = 0x0; uStack6 < param_3; uStack6 = uStack6 + 0x1) {
    uVar5 = (undefined2)(param_4 >> 0x10);
    iVar4 = (int)param_4;
    if ((*(long *)((int)uStack6 * 0x4 + iVar4) != 0x0) &&
       (uVar2 = *(undefined4 *)((int)uStack6 * 0x4 + iVar4),
       BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((int)uVar2 + 0xc),0x2c), BVar3 != 0x0)) {
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uStack6 * 0x4 + iVar4) + 0x54);
      (**ppcVar1)();
      if (BVar3 != 0x0) {
        uVar2 = *(undefined4 *)(iVar4 + (int)uStack6 * 0x4);
        *(undefined2 *)((int)uVar2 + 0x1a) = 0x3;
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uStack6 * 0x4 + iVar4) + 0x28);
        (**ppcVar1)();
        uVar2 = *(undefined4 *)(iVar4 + (int)uStack6 * 0x4);
        *(undefined2 *)((int)uVar2 + 0x1a) = 0x2;
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_5b3c(ushort param_1,ushort param_2,ulong param_3,ulong param_4)

{
  code **ppcVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  BOOL16 BVar4;
  int iVar5;
  undefined2 uVar6;
  ulong uStack6;
  
  for (uStack6 = 0x0; uStack6 < param_3; uStack6 = uStack6 + 0x1) {
    uVar6 = (undefined2)(param_4 >> 0x10);
    iVar5 = (int)param_4;
    if (((*(long *)((int)uStack6 * 0x4 + iVar5) != 0x0) &&
        (uVar2 = *(undefined4 *)((int)uStack6 * 0x4 + iVar5),
        BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((int)uVar2 + 0xc),0x2d), BVar4 != 0x0)) &&
       (ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uStack6 * 0x4 + iVar5) + 0x50), (**ppcVar1)(),
       BVar4 != 0x0)) {
      uVar2 = *(undefined4 *)((int)uStack6 * 0x4 + iVar5);
      uVar3 = *(undefined4 *)((int)uStack6 * 0x4 + iVar5);
      *(uint *)((int)uVar3 + 0x1a) = *(uint *)((int)uVar2 + 0x1a) | 0x1;
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uStack6 * 0x4 + iVar5) + 0x28);
      (**ppcVar1)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far
pass1_1038_5be8(undefined4 param_1,uint param_2,int param_3,ushort *param_4,uint param_5,uint param_6,ushort param_7)

{
  int iVar1;
  undefined2 uVar2;
  int iVar3;
  BOOL16 BVar4;
  uint uVar5;
  ulong uVar6;
  int iStack14;
  ulong uStack10;
  
  pass1_1030_627e(param_7,param_5,param_6,_PTR_LOOP_1050_5740,param_4,*(long *)((int)param_1 + 0x8));
  uVar5 = param_6 | param_5;
  if (uVar5 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5,param_6);
    uStack10 = CONCAT22(uVar5,param_5);
    iStack14 = 0x7a;
    if (0x0 < *(int *)((int)param_4 + 0x4)) {
      if (param_3 == 0x7b) {
        param_3 = 0x7e;
      }
      else {
        if (param_3 == 0x7c) {
          param_3 = 0x7d;
        }
      }
      iStack14 = 0x7f;
    }
    uVar6 = struct_op_1030_73a8(uStack10);
    uVar2 = (undefined2)(uVar6 >> 0x10);
    iVar3 = (int)uVar6;
    if ((((*(uint *)(iVar3 + 0x1a) & param_2) == 0x0) &&
        (((iVar1 = *(int *)(iVar3 + 0xc), iVar1 == iStack14 || (iVar1 == param_3)) ||
         (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,iVar1,0x2b), BVar4 != 0x0)))) && (*(int *)(iVar3 + 0x12) != 0x7))
    {
      *(uint *)(iVar3 + 0x1a) = *(uint *)(iVar3 + 0x1a) | param_2;
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1038_5cc6(ulong param_1,ulong param_2,ulong param_3,ulong param_4,int param_5,uint param_6)

{
  ulong uVar1;
  undefined4 uVar2;
  ushort uVar3;
  uint uVar4;
  undefined2 uVar5;
  ushort unaff_SS;
  ushort *puVar6;
  int local_14;
  int local_12;
  int local_10;
  ulong uStack14;
  int local_a;
  int iStack8;
  int iStack4;
  
  puVar6 = pass1_1008_3e38((ushort *)CONCAT22(unaff_SS,&local_a));
  uVar4 = (uint)((ulong)puVar6 >> 0x10);
  do {
    iStack4 = 0x0;
    for (uStack14 = 0x0; uStack14 < param_2; uStack14 = uStack14 + 0x1) {
      uVar5 = (undefined2)(param_4 >> 0x10);
      if (*(long *)((int)uStack14 * 0x4 + (int)param_4) != 0x0) {
        uVar1 = *(ulong *)((int)uStack14 * 0x4 + (int)param_4);
        pass1_1008_3f62((ushort *)CONCAT22(unaff_SS,&local_a),(ushort *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0xc))
                       );
        pass1_1008_3eb4((ushort *)CONCAT22(unaff_SS,&local_a),(ushort *)CONCAT22(unaff_SS,&local_14),
                        (ushort *)CONCAT22(unaff_SS,&local_12),(ushort *)CONCAT22(unaff_SS,&local_10));
        if (local_14 == param_5) {
          uVar5 = (undefined2)(param_3 >> 0x10);
          if ((*(long *)((int)uStack14 * 0x4 + (int)param_3) != 0x0) &&
             (uVar2 = *(undefined4 *)((int)uStack14 * 0x4 + (int)param_3),
             (*(uint *)((int)uVar2 + 0x1a) & param_6) != 0x0)) {
            iStack8 = local_12 + -0x1;
            uVar3 = pass1_1038_5be8(param_1,param_6,0x7b,(ushort *)CONCAT22(unaff_SS,&local_a),(uint)&local_a,uVar4,
                                    unaff_SS);
            if (uVar3 != 0x0) {
              iStack4 = 0x1;
            }
            iStack8 = local_12 + 0x1;
            uVar3 = pass1_1038_5be8(param_1,param_6,0x7b,(ushort *)CONCAT22(unaff_SS,&local_a),(uint)&local_a,uVar4,
                                    unaff_SS);
            if (uVar3 != 0x0) {
              iStack4 = 0x1;
            }
            iStack8 = local_12;
            local_a = local_10 + -0x1;
            uVar3 = pass1_1038_5be8(param_1,param_6,0x7c,(ushort *)CONCAT22(unaff_SS,&local_a),(uint)&local_a,uVar4,
                                    unaff_SS);
            if (uVar3 != 0x0) {
              iStack4 = 0x1;
            }
            local_a = local_10 + 0x1;
            uVar3 = pass1_1038_5be8(param_1,param_6,0x7c,(ushort *)CONCAT22(unaff_SS,&local_a),(uint)&local_a,uVar4,
                                    unaff_SS);
            if (uVar3 != 0x0) {
              iStack4 = 0x1;
            }
          }
        }
      }
    }
  } while (iStack4 != 0x0);
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1038_5e16(ulong param_1,ulong param_2,int param_3,ushort param_4,ushort param_5)

{
  BOOL16 BVar1;
  ulong *puVar2;
  int iVar3;
  ushort uVar4;
  ushort uVar5;
  ushort uVar6;
  undefined4 local_14 [0x2];
  undefined4 local_c;
  ulong *puStack6;
  
  pass1_1030_16d6(param_1,param_2,param_5);
  if (param_3 != 0x0) {
    uVar4 = (ushort)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    puVar2 = *(ulong **)(iVar3 + 0xc);
    puStack6 = puVar2;
    pass1_1008_7898(param_2,puVar2,(ushort)puVar2,param_4,0x1008,param_5);
    if ((int)puVar2 != 0x0) {
      local_14[0] = *(undefined4 *)(iVar3 + 0x10);
      uVar5 = (ushort)param_2;
      uVar6 = (ushort)(param_2 >> 0x10);
      BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_14,param_5,(char *)0x4,0x1008);
      if (BVar1 != 0x0) {
        local_c._0_2_ = *(undefined2 *)(iVar3 + 0x18);
        BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
        if (BVar1 != 0x0) {
          local_c._0_2_ = *(undefined2 *)(iVar3 + 0x1a);
          BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
          if (BVar1 != 0x0) {
            local_c = CONCAT22(local_c._2_2_,*(undefined2 *)(iVar3 + 0x1c));
            BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
            if (BVar1 != 0x0) {
              local_c = *(ulong *)(iVar3 + 0x1e);
              BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x4,0x1008);
              if (BVar1 != 0x0) {
                local_c = local_c & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x22);
                BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
                if (BVar1 != 0x0) {
                  local_c = local_c & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x24);
                  BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
                  if (BVar1 != 0x0) {
                    BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,iVar3 + 0x26,uVar4,(char *)0x94,0x1008);
                    if (BVar1 != 0x0) {
                      BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,iVar3 + 0x14e,uVar4,(char *)0x54,0x1008);
                      if (BVar1 != 0x0) {
                        BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,iVar3 + 0x1a2,uVar4,(char *)0x54,0x1008);
                        if (BVar1 != 0x0) {
                          write_to_file_1030_32e4(*(ulong *)(iVar3 + 0x1f6),param_2,param_5);
                          BVar1 = pass1_1008_7c2a(param_2,*(char **)(iVar3 + 0x1fa),0x1008);
                          if (BVar1 != 0x0) {
                            local_c = local_c & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x1fe);
                            BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
                            if (BVar1 != 0x0) {
                              local_c = *(ulong *)(iVar3 + 0x200);
                              BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x4,0x1008);
                              if (BVar1 != 0x0) {
                                local_c = local_c & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x204);
                                BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008)
                                ;
                                if (BVar1 != 0x0) {
                                  local_c = local_c & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x206);
                                  BVar1 = write_to_file_1008_7e1c
                                                    (uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
                                  if (BVar1 != 0x0) {
                                    local_c = local_c & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x208);
                                    BVar1 = write_to_file_1008_7e1c
                                                      (uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
                                    if (BVar1 != 0x0) {
                                      local_c = local_c & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x20a);
                                      BVar1 = write_to_file_1008_7e1c
                                                        (uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
                                      if (BVar1 != 0x0) {
                                        local_c = local_c & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x20c);
                                        BVar1 = write_to_file_1008_7e1c
                                                          (uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
                                        if (BVar1 != 0x0) {
                                          local_c = local_c & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x20e);
                                          BVar1 = write_to_file_1008_7e1c
                                                            (uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
                                          if (BVar1 != 0x0) {
                                            local_c = local_c & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x214);
                                            BVar1 = write_to_file_1008_7e1c
                                                              (uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x2,0x1008);
                                            if (BVar1 != 0x0) {
                                              local_c = *(undefined4 *)(iVar3 + 0x216);
                                              BVar1 = write_to_file_1008_7e1c
                                                                (uVar5,uVar6,(ushort)&local_c,param_5,(char *)0x4,0x1008
                                                                );
                                              if (BVar1 != 0x0) {
                                                return;
                                              }
                                            }
                                          }
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



void __stdcall16far file_1038_6118(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  undefined2 uVar1;
  undefined4 *puVar2;
  BOOL16 BVar3;
  ushort uVar4;
  ushort uVar5;
  undefined *puVar6;
  uchar *puVar7;
  astruct_429 *iVar9;
  ushort uVar8;
  ushort uVar9;
  ushort uVar10;
  SEGPTR SVar11;
  astruct_18 *paStack1046;
  ushort uStack1042;
  undefined local_408 [0x400];
  undefined2 local_8;
  undefined4 local_6;
  
  file_1030_1730(param_1,param_2);
  if (param_3 == 0x0) {
    return;
  }
  local_6 = 0x0;
  puVar2 = &local_6;
  file_1008_7548(param_2,(long *)CONCAT22(param_5,puVar2),0x1008,param_5);
  if (puVar2 != (undefined4 *)0x0) {
    uVar8 = (ushort)(param_1 >> 0x10);
    iVar9 = (astruct_429 *)param_1;
    iVar9->field_0xc = local_6;
    uVar9 = (ushort)param_2;
    uVar10 = (ushort)(param_2 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x10,0x0,uVar8,0x4,0x1008);
    if (((((BVar3 != 0x0) &&
          (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x18,0x0,uVar8,0x2,0x1008), BVar3 != 0x0)) &&
         (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x1a,0x0,uVar8,0x2,0x1008), BVar3 != 0x0)) &&
        ((BVar3 = read_file_1008_7dee(uVar9,uVar10,(ushort)&local_8,0x0,param_5,0x2,0x1008), BVar3 != 0x0 &&
         (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x1e,0x0,uVar8,0x4,0x1008), BVar3 != 0x0)))) &&
       (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x22,0x0,uVar8,0x2,0x1008), BVar3 != 0x0)) {
      iVar9->field_0x1c = local_8;
      BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x24,0x0,uVar8,0x2,0x1008);
      if ((BVar3 != 0x0) &&
         (uVar4 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x26,0x0,uVar8,0x94,0x1008), uVar4 != 0x0)) {
        if ((int)PTR_LOOP_1050_0312 < 0x2) {
          uVar5 = 0x54;
          SVar11 = 0x54;
          mem_op_1000_179c(0x54,param_4,0x1000);
          paStack1046 = (astruct_18 *)CONCAT22(param_4,uVar4);
          BVar3 = read_file_1008_7dee(uVar9,uVar10,uVar4,uVar5,(ushort)param_4,SVar11,0x1008);
          if (BVar3 == 0x0) {
LAB_1038_626a:
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            fn_ptr_1000_17ce(paStack1046,0x1000);
            return;
          }
          uStack1042 = 0x0;
          do {
            uVar5 = switch_1008_72bc(uVar9,uVar10,uStack1042);
            uVar1 = *(undefined2 *)(uStack1042 * 0x4 + uVar4 + 0x2);
            *(undefined2 *)(&iVar9->field_0x14e + uVar5 * 0x4) = *(undefined2 *)(uStack1042 * 0x4 + uVar4);
            *(undefined2 *)(&iVar9->field_0x150 + uVar5 * 0x4) = uVar1;
            uStack1042 = uStack1042 + 0x1;
          } while ((int)uStack1042 < 0x15);
          BVar3 = read_file_1008_7dee(uVar9,uVar10,uVar4,0x0,(ushort)param_4,0x54,0x1008);
          if (BVar3 == 0x0) goto LAB_1038_626a;
          uStack1042 = 0x0;
          do {
            uVar5 = switch_1008_72bc(uVar9,uVar10,uStack1042);
            puVar7 = *(uchar **)(uStack1042 * 0x4 + uVar4 + 0x2);
            *(undefined2 *)(&iVar9->field_0x1a2 + uVar5 * 0x4) = *(undefined2 *)(uStack1042 * 0x4 + uVar4);
            *(uchar **)(&iVar9->field_0x1a4 + uVar5 * 0x4) = puVar7;
            uStack1042 = uStack1042 + 0x1;
          } while ((int)uStack1042 < 0x15);
          fn_ptr_1000_17ce(paStack1046,0x1000);
          param_4 = puVar7;
        }
        else {
          BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x14e,0x0,uVar8,0x54,0x1008);
          if (BVar3 == 0x0) {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
          }
          BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x1a2,0x0,uVar8,0x54,0x1008);
          if (BVar3 == 0x0) {
            PTR_LOOP_1050_0310 = (undefined *)0x6d2;
            return;
          }
        }
        read_file_1030_33f0(iVar9->field_0x1f6,param_2);
        puVar6 = local_408;
        read_file_1008_7c6e(uVar9,uVar10,(char *)CONCAT22(param_5,puVar6),0x1008);
        if (puVar6 != (undefined *)0x0) {
          uVar4 = str_op_1008_60e8((char *)CONCAT22(param_5,local_408),(ushort)param_4);
          iVar9->field_0x1fa = uVar4;
          iVar9->field_0x1fc = (ushort)param_4;
          BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x1fe,0x0,uVar8,0x2,0x1008);
          if (((((BVar3 != 0x0) &&
                (BVar3 = read_file_1008_7dee(uVar9,uVar10,CONCAT11((char)(param_1 >> 0x8) + '\x02',(char)param_1),0x0,
                                             uVar8,0x4,0x1008), BVar3 != 0x0)) &&
               (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x204,0x0,uVar8,0x2,0x1008), BVar3 != 0x0)) &&
              (((BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x206,0x0,uVar8,0x2,0x1008), BVar3 != 0x0 &&
                (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x208,0x0,uVar8,0x2,0x1008), BVar3 != 0x0)) &&
               ((BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x20a,0x0,uVar8,0x2,0x1008), BVar3 != 0x0 &&
                ((BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x20c,0x0,uVar8,0x2,0x1008), BVar3 != 0x0 &&
                 (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x20e,0x0,uVar8,0x2,0x1008), BVar3 != 0x0))))))
              )) && (((int)PTR_LOOP_1050_0312 < 0x2 ||
                     ((BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x214,0x0,uVar8,0x2,0x1008), BVar3 != 0x0
                      && (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x216,0x0,uVar8,0x4,0x1008),
                         BVar3 != 0x0)))))) {
            return;
          }
          PTR_LOOP_1050_0310 = (undefined *)0x6d0;
          return;
        }
      }
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  return;
}
