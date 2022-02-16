
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_43cc(int param_1,undefined2 param_2,uint param_3,uint param_4,int param_5,int param_6)

{
  code **ppcVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  ulong uVar5;
  uchar *puVar6;
  undefined2 extraout_DX;
  undefined2 uVar7;
  int iVar8;
  int iVar9;
  undefined2 uVar10;
  ulong *puVar11;
  ulong uVar12;
  ulong uStack22;
  ulong uStack18;
  undefined4 *puStack14;
  
  if (param_4 == 0x5) {
    pass1_1038_4900(CONCAT22(param_2,param_1));
    return;
  }
  pass1_1038_53ba(CONCAT22(param_2,param_1),param_4);
  if ((param_6 != 0x0) || (param_5 != 0x0)) {
    iVar8 = param_4 * 0x4;
    uVar2 = *(uint *)(param_1 + iVar8 + 0x14e);
    iVar9 = (*(int *)(param_1 + iVar8 + 0x150) - ((int)param_3 >> 0xf)) - (uint)(uVar2 < param_3);
    *(int *)(param_1 + iVar8 + 0x14e) = uVar2 - param_3;
    *(int *)(param_1 + iVar8 + 0x150) = iVar9;
    if (iVar9 < 0x0) {
      *(undefined4 *)(param_1 + iVar8 + 0x14e) = 0x0;
    }
    uVar10 = 0x1008;
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
    puVar6 = (uchar *)((ulong)puVar11 >> 0x10);
    uVar2 = (uint)puVar11;
    pass1_1038_4e78(uVar2,puVar6,CONCAT22(param_2,param_1),puVar11);
    puStack14 = (undefined4 *)CONCAT22(puVar6,uVar2);
    ppcVar1 = (code **)((int)*puStack14 + 0x10);
    uVar3 = uVar2;
    (**ppcVar1)(0x1008,uVar2,puVar6);
    uStack18 = CONCAT22(extraout_DX,uVar3);
    uVar7 = extraout_DX;
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 = uStack22 + 0x1) {
      uVar12 = pass1_1030_1d7c(uVar3,uVar7,(ulong)puStack14);
      uVar7 = (undefined2)(uVar12 >> 0x10);
      uVar5 = uVar12 & 0xffff;
      for (; uVar4 = (uint)uVar5, param_3 != 0x0; param_3 = param_3 - 0x1) {
        pass1_1030_cf78(uVar12,param_4);
        uVar5 = (ulong)uVar4;
        if (uVar4 == 0x0) break;
      }
      uVar10 = 0x1030;
      if (param_3 == 0x0) break;
    }
    if (puStack14 != (undefined4 *)0x0) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar10,uVar2,(char)puVar6,0x1);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_44d8(int param_1,ushort param_2,uint param_3,uint param_4,int param_5,int param_6)

{
  code **ppcVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  ulong uVar5;
  uchar *puVar6;
  ushort extraout_DX;
  ushort uVar7;
  ushort uVar8;
  astruct_697 *iVar9;
  int iVar10;
  undefined2 uVar11;
  ulong *puVar12;
  ulong uVar13;
  ulong uStack22;
  ulong uStack18;
  undefined4 *puStack14;
  
  if (param_4 == 0x5) {
    pass1_1038_4900(CONCAT22(param_2,param_1));
    return;
  }
  pass1_1038_53ba(CONCAT22(param_2,param_1),param_4);
  if ((param_6 != 0x0) || (param_5 != 0x0)) {
    iVar9 = (astruct_697 *)(param_4 * 0x4);
    uVar2 = *(uint *)(iVar9 + param_1 + 0x14e);
    iVar10 = (*(int *)(iVar9 + param_1 + 0x150) - ((int)param_3 >> 0xf)) - (uint)(uVar2 < param_3);
    *(uint *)(iVar9 + param_1 + 0x14e) = uVar2 - param_3;
    *(int *)(iVar9 + param_1 + 0x150) = iVar10;
    if (iVar10 < 0x0) {
      *(undefined4 *)(iVar9 + param_1 + 0x14e) = 0x0;
    }
    uVar11 = 0x1008;
    puVar12 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
    puVar6 = (uchar *)((ulong)puVar12 >> 0x10);
    uVar2 = (uint)puVar12;
    pass1_1038_4e78(uVar2,puVar6,CONCAT22(param_2,param_1),puVar12);
    puStack14 = (undefined4 *)CONCAT22(puVar6,uVar2);
    ppcVar1 = (code **)((int)*puStack14 + 0x10);
    uVar3 = uVar2;
    (**ppcVar1)(0x1008,uVar2,puVar6);
    uStack18 = CONCAT22(extraout_DX,uVar3);
    uVar7 = extraout_DX;
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 = uStack22 + 0x1) {
      uVar13 = pass1_1030_1d7c(uVar3,uVar7,(ulong)puStack14);
      uVar8 = (ushort)(uVar13 >> 0x10);
      uVar5 = uVar13 & 0xffff;
      uVar7 = uVar8;
      for (; uVar4 = (uint)uVar5, param_3 != 0x0; param_3 = param_3 - 0x1) {
        pass1_1030_d00c((int)uVar13,uVar8,param_4);
        uVar5 = (ulong)uVar4;
        if (uVar4 == 0x0) break;
      }
      uVar11 = 0x1030;
      if (param_3 == 0x0) break;
    }
    if (puStack14 != (undefined4 *)0x0) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar11,uVar2,(char)puVar6,0x1);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_45e4(ulong param_1,uint param_2,int param_3,ushort param_4)

{
  int *piVar1;
  code **ppcVar2;
  undefined4 uVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  uchar *puVar11;
  int iVar12;
  uint uVar13;
  undefined2 uVar14;
  bool bVar15;
  ulong *puVar16;
  ushort uStack28;
  undefined4 *puStack22;
  
  uVar14 = (undefined2)(param_1 >> 0x10);
  iVar12 = (int)param_1;
  pass1_1030_38f2(*(ulong *)(iVar12 + 0x1f6),0x2,param_4);
  iVar8 = param_3;
  uVar4 = param_2;
  pass1_1030_38f2(*(ulong *)(iVar12 + 0x1f6),0x1,param_4);
  bVar15 = param_2 < uVar4;
  uVar13 = param_2 - uVar4;
  iVar10 = param_3 - iVar8;
  pass1_1030_38f2(*(ulong *)(iVar12 + 0x1f6),0x4,param_4);
  iVar9 = iVar8;
  uVar5 = uVar4;
  pass1_1030_38f2(*(ulong *)(iVar12 + 0x1f6),0x3,param_4);
  uVar7 = *(uint *)(iVar12 + 0x24);
  uVar6 = uVar7 + (uVar4 - uVar5);
  iVar10 = ((int)uVar7 >> 0xf) + ((iVar8 - iVar9) - (uint)(uVar4 < uVar5)) + (uint)CARRY2(uVar7,uVar4 - uVar5) +
           (iVar10 - (uint)bVar15) + (uint)CARRY2(uVar6,uVar13);
  if ((iVar10 < 0x0) || ((iVar10 < 0x1 && (uVar6 + uVar13 == 0x0)))) {
    iVar10 = -0x1;
  }
  else {
    iVar10 = 0x1;
  }
  piVar1 = (int *)(iVar12 + 0x24);
  *piVar1 = *piVar1 + iVar10;
  puVar16 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x16);
  puVar11 = (uchar *)((ulong)puVar16 >> 0x10);
  uVar7 = (uint)puVar16;
  pass1_1038_4d6e(param_1,puVar16,uVar7,puVar11);
  puStack22 = (undefined4 *)CONCAT22(puVar11,uVar7);
  uVar3 = *puStack22;
  ppcVar2 = (code **)uVar3 + 0x8;
  uVar5 = uVar7;
  (**ppcVar2)(0x1008,uVar7,puVar11);
  if (puStack22 != (undefined4 *)0x0) {
    ppcVar2 = (code **)uVar3;
    (**ppcVar2)(0x1008,uVar7,(char)puVar11,0x1);
  }
  piVar1 = (int *)(iVar12 + 0x24);
  *piVar1 = *piVar1 + uVar5 * 0x2;
  iVar10 = *(int *)(iVar12 + 0x24);
  if (0x64 < iVar10) {
    iVar10 = 0x64;
  }
  *(int *)(iVar12 + 0x24) = iVar10;
  if (iVar10 < 0x0) {
    iVar10 = 0x0;
  }
  *(int *)(iVar12 + 0x24) = iVar10;
  iVar10 = iVar10 / 0xa;
  uStack28 = 0x10;
  if (iVar10 < 0xb) {
    uStack28 = 0x14;
  }
  else {
    if (iVar10 < 0x15) {
      uStack28 = 0x13;
    }
    else {
      if (iVar10 < 0x1f) {
        uStack28 = 0x12;
      }
      else {
        if (iVar10 < 0x29) {
          uStack28 = 0x11;
        }
        else {
          if (iVar10 < 0x33) {
            uStack28 = 0x10;
          }
          else {
            if (iVar10 < 0x3d) {
              uStack28 = 0xf;
            }
            else {
              if (iVar10 < 0x47) {
                uStack28 = 0xe;
              }
              else {
                if (iVar10 < 0x51) {
                  uStack28 = 0xd;
                }
                else {
                  if (iVar10 < 0x5b) {
                    uStack28 = 0xc;
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  pass1_1030_3258(*(ulong *)(iVar12 + 0x1f6),uStack28);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_4760(ulong param_1)

{
  uint *puVar1;
  code **ppcVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  uint uVar6;
  uchar *puVar7;
  uchar *puVar8;
  uint extraout_DX;
  undefined2 extraout_DX_00;
  undefined2 extraout_DX_01;
  undefined2 extraout_DX_02;
  undefined2 uVar9;
  undefined2 extraout_DX_03;
  undefined2 extraout_DX_04;
  astruct_700 *iVar10;
  undefined2 uVar10;
  undefined2 uVar11;
  undefined2 unaff_SS;
  ulong *puVar12;
  ulong uVar13;
  undefined uVar14;
  uchar *puVar15;
  ulong uStack26;
  ulong uStack22;
  undefined4 *puStack14;
  undefined4 *puStack10;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar10 = (astruct_700 *)param_1;
  puVar1 = &iVar10->field_0x22;
  *puVar1 = *puVar1 + iVar10->field_0x20c;
  puVar12 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x26);
  puVar7 = (uchar *)((ulong)puVar12 >> 0x10);
  uVar6 = (uint)puVar12;
  pass1_1038_4d6e(param_1,puVar12,uVar6,puVar7);
  puStack10 = (undefined4 *)CONCAT22(puVar7,uVar6);
  uVar11 = 0x1008;
  puVar12 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1a);
  puVar8 = (uchar *)((ulong)puVar12 >> 0x10);
  uVar3 = (uint)puVar12;
  pass1_1038_4d6e(param_1,puVar12,uVar3,puVar8);
  puStack14 = (undefined4 *)CONCAT22(puVar8,uVar3);
  ppcVar2 = (code **)((int)*puStack14 + 0x10);
  uVar4 = uVar3;
  (**ppcVar2)(0x1008,uVar3,puVar8);
  uVar14 = (undefined)uVar6;
  puVar15 = puVar7;
  if ((extraout_DX | uVar4) == 0x0) {
    ppcVar2 = (code **)((int)*puStack10 + 0x10);
    (**ppcVar2)();
    puVar1 = &iVar10->field_0x22;
    *puVar1 = *puVar1 + uVar4;
    uVar9 = extraout_DX_00;
  }
  else {
    ppcVar2 = (code **)((int)*puStack10 + 0x10);
    (**ppcVar2)();
    uStack22 = CONCAT22(extraout_DX_03,uVar4);
    uVar9 = extraout_DX_03;
    for (uStack26 = 0x0; uStack26 < uStack22; uStack26 = uStack26 + 0x1) {
      uVar13 = pass1_1030_1d7c(uVar4,uVar9,(ulong)puStack10);
      iVar5 = (int)uVar13;
      uVar11 = SUB42(&USHORT_1050_1028,0x0);
      func_0x10285a94();
      if (iVar5 == 0x2) {
        if ((*_PTR_LOOP_1050_65e2 & 0x1) == 0x0) goto LAB_1038_485e;
      }
      else {
        if (iVar5 != 0x3) {
LAB_1038_485e:
          puVar1 = &iVar10->field_0x22;
          *puVar1 = *puVar1 + 0x1;
        }
      }
      uVar9 = extraout_DX_04;
    }
  }
  if (puStack10 != (undefined4 *)0x0) {
    ppcVar2 = (code **)*puStack10;
    (**ppcVar2)(uVar11,uVar6,puVar7,0x1,uVar14,puVar15);
    uVar9 = extraout_DX_01;
  }
  if (puStack14 != (undefined4 *)0x0) {
    ppcVar2 = (code **)*puStack14;
    (**ppcVar2)(uVar11,uVar3,puVar8,0x1);
    uVar9 = extraout_DX_02;
  }
  pass1_1038_45e4(param_1,(int)puStack14,uVar9,unaff_SS);
  if (0x32 < iVar10->field_0x24) {
    puVar1 = &iVar10->field_0x22;
    *puVar1 = *puVar1 - 0x1;
  }
  if (iVar10->field_0x24 < 0x32) {
    puVar1 = &iVar10->field_0x22;
    *puVar1 = *puVar1 + 0x1;
  }
  if (iVar10->field_0x18 < 0xfa) {
    puVar1 = &iVar10->field_0x22;
    *puVar1 = *puVar1 + 0x2;
  }
  else {
    if (iVar10->field_0x18 < 0x1c2) {
      puVar1 = &iVar10->field_0x22;
      *puVar1 = *puVar1 + 0x1;
    }
    else {
      if (0x225 < iVar10->field_0x18) {
        if (iVar10->field_0x18 < 0x2ee) {
          puVar1 = &iVar10->field_0x22;
          *puVar1 = *puVar1 - 0x1;
        }
        else {
          puVar1 = &iVar10->field_0x22;
          *puVar1 = *puVar1 - 0x2;
        }
      }
    }
  }
  uVar6 = iVar10->field_0x22;
  if (0x64 < (int)uVar6) {
    uVar6 = 0x64;
  }
  iVar10->field_0x22 = uVar6;
  if ((int)uVar6 < 0x0) {
    uVar6 = 0x0;
  }
  iVar10->field_0x22 = uVar6;
  return;
}



void __stdcall16far pass1_1038_48e0(ulong param_1,int param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = *(int *)((int)param_1 + 0x20e) + param_2;
  if (0xa < iVar1) {
    iVar1 = 0xa;
  }
  *(int *)((int)param_1 + 0x20e) = iVar1;
  return;
}



void __stdcall16far pass1_1038_4900(ulong param_1)

{
  int *piVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  piVar1 = (int *)((int)param_1 + 0x20e);
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 < 0x0) {
    *(undefined2 *)((int)param_1 + 0x20e) = 0x0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_4918(ulong param_1,int param_2,ushort param_3,ushort param_4,uchar param_5)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  ulong *puVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  ulong uVar11;
  byte bStack347;
  undefined local_14a [0x4];
  ulong *puStack326;
  undefined local_144 [0x124];
  ulong local_20;
  undefined2 uStack28;
  ulong uStack26;
  ulong uStack18;
  ulong uStack14;
  undefined4 uStack10;
  undefined4 uStack6;
  
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar7 = (int)param_1;
  if (*(long *)(iVar7 + 0x4) != 0x4000001) {
    return;
  }
  uVar2 = *(undefined4 *)(iVar7 + 0x8);
  pass1_1028_e1ec((ulong)_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)((ulong)uVar2 >> 0x10));
  uStack6 = CONCAT22(param_3,param_2);
  uStack10 = *(undefined4 *)(param_2 + 0x10);
  uVar10 = (undefined2)((ulong)uStack10 >> 0x10);
  iVar8 = (int)uStack10;
  if (*(int *)(iVar8 + 0x1c) == 0x0) {
    return;
  }
  uStack14 = 0x0;
  switch(*(undefined2 *)(iVar7 + 0x20e)) {
  case 0x1:
    uStack14._0_2_ = 0x1e;
    break;
  case 0x2:
    uStack14._0_2_ = 0x1c;
    break;
  case 0x3:
    uStack14._0_2_ = 0x1a;
    break;
  case 0x4:
    uStack14._0_2_ = 0x18;
    break;
  case 0x5:
    uStack14._0_2_ = 0x16;
    break;
  case 0x6:
    uStack14._0_2_ = 0x14;
    break;
  case 0x7:
    uStack14._0_2_ = 0x12;
    break;
  case 0x8:
    uStack14._0_2_ = 0x10;
    break;
  case 0x9:
    uStack14._0_2_ = 0xe;
    break;
  case 0xa:
    uStack14._0_2_ = 0xc;
    break;
  default:
    goto switchD_1038_49cf_caseD_a;
  }
  uStack14 = (ulong)(uint)uStack14;
switchD_1038_49cf_caseD_a:
  uStack18 = *_PTR_LOOP_1050_65e2;
  if (((uint)uStack14 != 0x0) &&
     ((int)((uStack18 & 0xffff | (ulong)*(uint *)((int)_PTR_LOOP_1050_65e2 + 0x2) << 0x10) % uStack14) == 0x0)) {
    piVar1 = (int *)(iVar8 + 0x1c);
    *piVar1 = *piVar1 + -0x1;
    piVar1 = (int *)(iVar8 + 0x1a);
    *piVar1 = *piVar1 + 0x1;
    iVar3 = *(int *)(iVar8 + 0x1a) * 0x6 + *(int *)(iVar8 + 0x16);
    uVar10 = *(undefined2 *)(iVar8 + 0x18);
    local_20 = *(ulong *)(iVar3 + -0x6);
    uStack28 = *(undefined2 *)(iVar3 + -0x2);
    puStack326 = &local_20;
    puVar4 = &local_20;
    pass1_1030_64ce(param_4,puVar4,uVar10,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_4,puVar4),*(long *)(iVar7 + 0x8),
                    (ulong *)CONCAT22(param_4,local_14a));
    uStack26 = *puVar4;
    uVar6 = *(uint *)((int)puVar4 + 0x2);
    bStack347 = (byte)(uStack26 >> 0x18);
    uVar5 = (uint)bStack347;
    if (bStack347 != 0x0) {
      pass1_1028_e1ec((ulong)_PTR_LOOP_1050_65e2,(ushort)uStack26,uVar6);
      uVar11 = struct_op_1030_73a8(CONCAT22(uVar6,uVar5));
      uVar6 = (uint)(uVar11 >> 0x10);
      if ((uVar6 | (uint)uVar11) != 0x0) {
        iVar8 = *(int *)((uint)uVar11 + 0xc);
        if (iVar8 < 0x1) {
          return;
        }
        if (SBORROW2(iVar8,0x1)) {
          return;
        }
        if (0x8 < iVar8 + -0x1) {
          return;
        }
      }
    }
    struct_op_1028_87f0(param_4,param_5,(astruct_97 *)CONCAT22(param_4,local_144),0x0,0x0,0x10,&local_20,param_4,
                        *(ulong *)(iVar7 + 0x4),*(ulong *)(iVar7 + 0x8));
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,local_144));
  }
  return;
}



void __stdcall16far pass1_1038_4b20(ulong param_1,ulong param_2,ulong param_3,uint param_4)

{
  ulong uVar1;
  
  uVar1 = *(ulong *)((int)param_1 + 0xc);
  pass1_1020_c4f4(uVar1,(ushort)param_2,(ushort)(param_2 >> 0x10),param_3,(astruct_361 *)uVar1,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_4b40(ulong param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  uint uVar2;
  ulong uVar3;
  undefined2 extraout_DX;
  undefined2 uVar4;
  uint extraout_DX_00;
  uint uVar5;
  int iVar6;
  undefined2 uVar7;
  ulong uStack14;
  ulong uStack10;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*(long *)(iVar6 + 0xc) == 0x0) {
    param_2 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_2);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 = uStack14 + 0x1) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x4);
    uVar3 = uStack10;
    (**ppcVar1)(param_3,*(undefined4 *)(iVar6 + 0xc));
    uVar2 = (uint)uVar3;
    uVar5 = extraout_DX_00 | uVar2;
    if (uVar5 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00);
      param_3 = 0x1030;
      struct_op_1030_73a8(CONCAT22(uVar5,uVar2));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_4c1a(ulong param_1,ushort param_2,ulong param_3,ushort param_4)

{
  code **ppcVar1;
  uint uVar2;
  ushort uVar3;
  ulong uVar4;
  uint uVar5;
  int iVar6;
  undefined2 uVar7;
  undefined4 uVar8;
  ulong uStack14;
  ulong uStack10;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar8 = *(undefined4 *)(iVar6 + 0xc);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x10);
  (**ppcVar1)();
  uStack10 = CONCAT22((int)param_3,param_2);
  for (uStack14 = 0x0; uVar5 = (uint)param_3, uStack14 < uStack10; uStack14 = uStack14 + 0x1) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x4);
    uVar4 = uStack10;
    (**ppcVar1)(param_4,*(undefined4 *)(iVar6 + 0xc),uStack14,uVar8);
    uVar2 = (uint)uVar4;
    param_3 = (ulong)(uVar5 | uVar2);
    if ((uVar5 | uVar2) != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,uVar5);
      uVar3 = pass1_1030_6fa0(CONCAT22((int)param_3,uVar2));
      param_4 = 0x1008;
      pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar3,0xe);
    }
  }
  return;
}



void __stdcall16far pass1_1038_4cba(void)

{
  pass1_1030_38b8();
  return;
}



void __stdcall16far pass1_1038_4cd0(ulong param_1,ulong param_2,ushort param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(ushort *)((int)param_1 + 0x1c) = param_3;
  *(ulong *)((int)param_1 + 0x1e) = param_2;
  return;
}



void __stdcall16far pass1_1038_4cea(ulong param_1,ulong *param_2,ushort *param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *param_3 = *(ushort *)((int)param_1 + 0x1c);
  *param_2 = *(ulong *)((int)param_1 + 0x1e);
  return;
}



void __stdcall16far pass1_1038_4d0e(ulong param_1,ushort param_2)

{
  astruct_686 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  iVar1 = (astruct_686 *)param_1;
  iVar1->field_0x1a = iVar1->field_0x18;
  iVar1->field_0x18 = param_2;
  return;
}



char * __stdcall16far pass1_1038_4d28(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return (char *)CONCAT22(*(undefined2 *)((int)param_1 + 0x1fc),*(undefined2 *)((int)param_1 + 0x1fa));
}



void __stdcall16far pass1_1038_4d3c(ulong param_1,char *param_2,ushort param_3)

{
  ushort uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar2 + 0x1fa),0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  *(ushort *)(iVar2 + 0x1fa) = uVar1;
  *(ushort *)(iVar2 + 0x1fc) = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_4d6e(ulong param_1,ulong *param_2,uint param_3,uchar *param_4)

{
  int *piVar1;
  code **ppcVar2;
  uint uVar3;
  ushort uVar4;
  undefined2 extraout_DX;
  undefined2 extraout_DX_00;
  undefined2 uVar5;
  uint extraout_DX_01;
  uint uVar6;
  int iVar7;
  undefined2 uVar8;
  ulong uVar9;
  int iStack30;
  ulong uStack26;
  ulong uStack14;
  ulong uStack10;
  undefined4 *puStack6;
  
  mem_op_1000_179c(0x18,param_4,0x1000);
  if (((uint)param_4 | param_3) == 0x0) {
    param_3 = 0x0;
    uVar8 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_4,param_3),0x5,0x5);
    uVar8 = extraout_DX;
  }
  puStack6 = (undefined4 *)CONCAT22(uVar8,param_3);
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar7 = (int)param_1;
  if (*(long *)(iVar7 + 0xc) == 0x0) {
    param_3 = 0x0;
    uVar5 = 0x0;
  }
  else {
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x10);
    (**ppcVar2)();
    uVar5 = extraout_DX_00;
  }
  uStack10 = CONCAT22(uVar5,param_3);
  uStack14 = 0x0;
  do {
    if (uStack10 <= uStack14) {
      return;
    }
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x4);
    uVar9 = uStack10;
    (**ppcVar2)();
    uVar3 = (uint)uVar9;
    uVar6 = extraout_DX_01 | uVar3;
    if (uVar6 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_01);
      uStack26 = CONCAT22(uVar6,uVar3);
      uVar4 = pass1_1030_6fa0(CONCAT22(uVar6,uVar3));
      iStack30 = 0x0;
      while( true ) {
        piVar1 = (int *)((int)param_2 + 0x4);
        if (*piVar1 == iStack30 || *piVar1 < iStack30) break;
        if (*(ushort *)((int)*param_2 + iStack30 * 0x2) == uVar4) {
          uVar9 = struct_op_1030_73a8(uStack26);
          if (*(int *)((int)uVar9 + 0x12) == 0x5) {
            ppcVar2 = (code **)((int)*puStack6 + 0xc);
            (**ppcVar2)();
          }
          break;
        }
        iStack30 = iStack30 + 0x1;
      }
    }
    uStack14 = uStack14 + 0x1;
  } while( true );
}



void __stdcall16far pass1_1038_4e78(uint param_1,uchar *param_2,ulong param_3,ulong *param_4)

{
  int *piVar1;
  code **ppcVar2;
  ushort uVar3;
  ulong uVar4;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  uint uVar5;
  uint extraout_DX_01;
  uint uVar6;
  int iVar7;
  undefined2 uVar8;
  int iStack26;
  ulong uStack14;
  ulong uStack10;
  undefined4 *puStack6;
  
  mem_op_1000_179c(0x18,param_2,0x1000);
  if (((uint)param_2 | param_1) == 0x0) {
    param_1 = 0x0;
    uVar8 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_2,param_1),0x5,0x5);
    uVar8 = extraout_DX;
  }
  puStack6 = (undefined4 *)CONCAT22(uVar8,param_1);
  uVar8 = (undefined2)(param_3 >> 0x10);
  iVar7 = (int)param_3;
  if (*(long *)(iVar7 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar5 = 0x0;
  }
  else {
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x10);
    (**ppcVar2)();
    uVar5 = extraout_DX_00;
  }
  uStack10 = CONCAT22(uVar5,param_1);
  uStack14 = 0x0;
  do {
    if (uStack10 <= uStack14) {
      return;
    }
    uVar4 = uStack10;
    pass1_1030_1d58(*(ulong *)(iVar7 + 0xc));
    uVar6 = uVar5 | (uint)uVar4;
    if (uVar6 != 0x0) {
      uVar3 = pass1_1030_6fa0(uVar4 & 0xffff | (ulong)uVar5 << 0x10);
      iStack26 = 0x0;
      while( true ) {
        piVar1 = (int *)((int)param_4 + 0x4);
        if (*piVar1 == iStack26 || *piVar1 < iStack26) break;
        if (*(ushort *)((int)*param_4 + iStack26 * 0x2) == uVar3) {
          ppcVar2 = (code **)((int)*puStack6 + 0xc);
          (**ppcVar2)();
          uVar6 = extraout_DX_01;
          break;
        }
        iStack26 = iStack26 + 0x1;
      }
    }
    uStack14 = uStack14 + 0x1;
    uVar5 = uVar6;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_4f54(ulong param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  ushort uVar2;
  BOOL16 BVar3;
  ulong uVar4;
  uint extraout_DX;
  uint uVar5;
  uint uVar6;
  int iVar7;
  undefined2 uVar8;
  ulong uStack10;
  ulong uStack6;
  
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar7 = (int)param_1;
  if (*(long *)(iVar7 + 0xc) == 0x0) {
    param_3 = 0x0;
    uVar5 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar7 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar5 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar5,param_3);
  uStack10 = 0x0;
  do {
    if (uStack6 <= uStack10) {
      return;
    }
    uVar4 = uStack6;
    pass1_1030_1d58(*(ulong *)(iVar7 + 0xc));
    uVar6 = uVar5 | (uint)uVar4;
    if (uVar6 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar4 & 0xffff | (ulong)uVar5 << 0x10);
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,param_2);
      if (BVar3 != 0x0) {
        return;
      }
    }
    uStack10 = uStack10 + 0x1;
    uVar5 = uVar6;
  } while( true );
}



void __stdcall16far pass1_1038_4fd8(ushort param_1,ulong param_2,ushort param_3)

{
  code **ppcVar1;
  ushort uVar2;
  ulong uVar3;
  uint extraout_DX;
  uint uVar4;
  uint uVar5;
  int iVar6;
  undefined2 uVar7;
  ulong uStack10;
  ulong uStack6;
  
  uVar7 = (undefined2)(param_2 >> 0x10);
  iVar6 = (int)param_2;
  if (*(long *)(iVar6 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar4,param_1);
  uStack10 = 0x0;
  do {
    if (uStack6 <= uStack10) {
      return;
    }
    uVar3 = uStack6;
    pass1_1030_1d58(*(ulong *)(iVar6 + 0xc));
    uVar5 = uVar4 | (uint)uVar3;
    if (uVar5 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar3 & 0xffff | (ulong)uVar4 << 0x10);
      if (uVar2 == param_3) {
        return;
      }
    }
    uStack10 = uStack10 + 0x1;
    uVar4 = uVar5;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_5050(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  code **ppcVar1;
  ushort uVar2;
  ulong uVar3;
  uint extraout_DX;
  uint uVar4;
  uint uVar5;
  int iVar6;
  undefined2 uVar7;
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
    uVar3 = uStack10;
    pass1_1030_1d58(*(ulong *)(iVar6 + 0xc));
    uVar5 = uVar4 | (uint)uVar3;
    if (uVar5 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar3 & 0xffff | (ulong)uVar4 << 0x10);
      pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,param_2);
    }
    uVar4 = uVar5;
  }
  return;
}
