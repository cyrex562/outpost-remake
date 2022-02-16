


void __stdcall16far pass1_1038_349e(ulong param_1,ulong param_2)

{
  code **ppcVar1;
  uint uVar2;
  undefined2 uVar3;
  uint extraout_DX;
  uint uVar4;
  uint uVar5;
  uint extraout_DX_00;
  astruct_685 *iVar7;
  undefined2 uVar6;
  undefined4 *puVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  ulong uStack10;
  ulong uStack6;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar7 = (astruct_685 *)param_1;
  iVar7->field_0x200 = param_2;
  pass1_1038_4d0e(param_1,0x258);
  uVar3 = (undefined2)param_2;
  pass1_1038_4d0e(param_1,0x258);
  iVar7->field_0x204 = 0x0;
  iVar7->field_0x206 = 0x0;
  puVar7 = iVar7->field_0xc;
  uVar8 = SUB42(puVar7,0x0);
  uVar9 = (undefined2)((ulong)puVar7 >> 0x10);
  ppcVar1 = (code **)((int)*iVar7->field_0xc + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,uVar3);
  uVar5 = extraout_DX;
  for (uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1) {
    puVar7 = (undefined4 *)pass1_1030_1d7c(uVar3,uVar5,(ulong)iVar7->field_0xc);
    uVar4 = (uint)((ulong)puVar7 >> 0x10);
    uVar2 = (uint)puVar7;
    uVar5 = uVar4 | uVar2;
    if (uVar5 != 0x0) {
      ppcVar1 = (code **)((int)*puVar7 + 0x58);
      (**ppcVar1)(0x1030,uVar2,uVar4,(char)param_1,uVar6,uVar8,uVar9);
      *(undefined4 *)(uVar2 + 0x1c) = 0x0;
      uVar5 = extraout_DX_00;
    }
  }
  return;
}



void __stdcall16far pass1_1038_354a(ulong param_1,uint param_2,uchar *param_3)

{
  uint uVar1;
  astruct_424 *iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (astruct_424 *)param_1;
  if (*(long *)&iVar1->field_0x21a == 0x0) {
    mem_op_1000_179c(0xa,param_3,0x1000);
    uVar1 = (uint)param_3 | param_2;
    if (uVar1 == 0x0) {
      *(undefined4 *)&iVar1->field_0x21a = 0x0;
    }
    else {
      pass1_1030_9ecc((ulong *)CONCAT22(param_3,param_2),param_1);
      *(uint *)&iVar1->field_0x21a = param_2;
      iVar1->field_0x21c = uVar1;
    }
  }
  pass1_1030_9ef2(*(ulong **)&iVar1->field_0x21a);
  return;
}



void __stdcall16far pass1_1038_35a8(ulong param_1,ushort param_2,uint param_3,uchar *param_4)

{
  uint uVar1;
  astruct_425 *iVar3;
  undefined2 uVar2;
  ushort unaff_SS;
  uchar in_AF;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_425 *)param_1;
  if (*(long *)&iVar3->field_0x21a == 0x0) {
    mem_op_1000_179c(0xa,param_4,0x1000);
    uVar1 = (uint)param_4 | param_3;
    if (uVar1 == 0x0) {
      *(undefined4 *)&iVar3->field_0x21a = 0x0;
    }
    else {
      pass1_1030_9ecc((ulong *)CONCAT22(param_4,param_3),param_1);
      *(uint *)&iVar3->field_0x21a = param_3;
      iVar3->field_0x21c = uVar1;
    }
  }
  pass1_1030_9f40(*(ulong *)&iVar3->field_0x21a,param_2,unaff_SS,in_AF);
  return;
}



void __stdcall16far pass1_1038_3608(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0x21a),0x1000);
  *(undefined4 *)((int)param_1 + 0x21a) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_362e(ulong param_1)

{
  ushort in_AX;
  uchar *in_DX;
  int iVar1;
  int unaff_DI;
  uint uVar2;
  ushort unaff_SS;
  astruct_67 *paVar3;
  
  uVar2 = (uint)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0x214) == 0x0) {
    pass1_1038_4f54(param_1 & 0xffff | (ulong)uVar2 << 0x10,0x1f,in_AX);
    if (in_AX == 0x0) {
      *(undefined2 *)(iVar1 + 0x214) = 0x14;
    }
    else {
      *(undefined2 *)(iVar1 + 0x214) = 0x28;
    }
    paVar3 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,unaff_SS,in_DX,unaff_DI);
    post_win_msg_1008_a0e4(paVar3,0x0,0x0,0x1,*(ulong *)(iVar1 + 0x4),0x38,0x1008,unaff_SS);
    *(undefined4 *)(iVar1 + 0x216) = 0x0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_3698(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int *piVar1;
  uint *puVar2;
  undefined4 uVar3;
  code **ppcVar4;
  undefined2 uVar5;
  BOOL16 BVar6;
  ushort uVar7;
  uint uVar8;
  long lVar9;
  ulong uVar10;
  uint uVar11;
  undefined2 uVar12;
  uint uVar13;
  ulong uVar14;
  int iVar15;
  undefined2 uVar16;
  ulong uVar17;
  ulong uStack32;
  ulong uStack18;
  ulong uStack14;
  undefined4 uStack10;
  undefined4 uStack6;
  
  uVar16 = (undefined2)(param_1 >> 0x10);
  iVar15 = (int)param_1;
  if (*(int *)(iVar15 + 0x214) == 0x0) {
    return;
  }
  pass1_1030_38b8();
  uStack6 = CONCAT22(param_3,param_2);
  uStack6 = uStack6 - *(long *)(iVar15 + 0x216);
  if (0x0 < uStack6) {
    uStack6 = uStack6 + 0x3;
    uStack10 = uStack6 / 0x5;
    uVar14 = uStack6 % 0x5;
    if (*(long *)(iVar15 + 0xc) == 0x0) {
      uVar5 = 0x0;
      uVar14 = 0x0;
    }
    else {
      uVar3 = *(undefined4 *)(iVar15 + 0xc);
      ppcVar4 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar15 + 0xc) + 0x10);
      lVar9 = uStack10;
      (**ppcVar4)(0x1030,(int)uVar3,(int)((ulong)uVar3 >> 0x10));
      uVar5 = (undefined2)lVar9;
    }
    uStack14 = CONCAT22((int)uVar14,uVar5);
    for (uStack18 = 0x0; uVar12 = (undefined2)uVar14, uVar10 = uStack14, uStack18 < uStack14; uStack18 = uStack18 + 0x1)
    {
      uVar17 = pass1_1030_1d7c(uVar5,uVar12,*(ulong *)(iVar15 + 0xc));
      uVar8 = (uint)(uVar17 >> 0x10);
      uVar13 = uVar8 | (uint)uVar17;
      uVar14 = (ulong)uVar13;
      if (uVar13 != 0x0) {
        BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((uint)uVar17 + 0xc),0x4);
        uVar8 = (uint)uVar14;
        uVar10 = (ulong)BVar6;
        if (BVar6 != 0x0) {
          uVar7 = pass1_1028_678c(uVar17,0xf,param_4);
          uStack32 = CONCAT22(uVar8,uVar7);
          uVar14 = (ulong)(uVar8 | uVar7);
          uVar10 = (ulong)uVar7;
          if ((uVar8 | uVar7) != 0x0) {
            if (uStack10 < (long)uStack32) {
              uVar8 = (uint)uStack10;
              pass1_1028_6356(uVar17,0xf,uVar8,uStack10._2_2_,param_4);
              uVar13 = uVar8 * 0x5;
              uVar11 = uStack10._2_2_ * 0x5 + (uint)CARRY2(uVar8,uVar8) * 0x2 + (uint)CARRY2(uVar8 * 0x2,uVar8 * 0x2) +
                       (uint)CARRY2(uVar8 * 0x4,uVar8);
              uVar14 = (ulong)uVar11;
              puVar2 = (uint *)(iVar15 + 0x216);
              uVar8 = *puVar2;
              *puVar2 = *puVar2 + uVar13;
              piVar1 = (int *)(iVar15 + 0x218);
              *piVar1 = *piVar1 + uVar11 + (uint)CARRY2(uVar8,uVar13);
              uStack10 = 0x0;
              uVar10 = (ulong)uVar13;
            }
            else {
              pass1_1028_6356(uVar17,0xf,uVar7,uVar8,param_4);
              uVar13 = uVar8 * 0x5 + (uint)CARRY2(uVar7,uVar7) * 0x2 + (uint)CARRY2(uVar7 * 0x2,uVar7 * 0x2) +
                       (uint)CARRY2(uVar7 * 0x4,uVar7);
              uVar14 = (ulong)uVar13;
              puVar2 = (uint *)(iVar15 + 0x216);
              uVar8 = *puVar2;
              *puVar2 = *puVar2 + uVar7 * 0x5;
              piVar1 = (int *)(iVar15 + 0x218);
              *piVar1 = *piVar1 + uVar13 + (uint)CARRY2(uVar8,uVar7 * 0x5);
              uStack10 = uStack10 - uStack32;
              uVar10 = uStack32;
            }
          }
        }
        uVar12 = (undefined2)uVar14;
        if (uStack10 == 0x0) break;
      }
    }
    uVar5 = (undefined2)uVar10;
    pass1_1030_38b8();
    uStack6 = CONCAT22(uVar12,uVar5);
    uStack6 = uStack6 - *(long *)(iVar15 + 0x216);
    uStack6._2_2_ = (uint)((ulong)uStack6 >> 0x10);
    if ((uStack6._2_2_ | (uint)uStack6) != 0x0) {
      uStack32 = uStack6 / (long)*(int *)(iVar15 + 0x214);
      if ((long)uStack32 < 0x1) {
        uStack32 = 0x1;
      }
      pass1_1030_375a(*(ulong *)(iVar15 + 0x1f6),0x0,uStack32,param_4);
    }
  }
  piVar1 = (int *)(iVar15 + 0x214);
  *piVar1 = *piVar1 + -0x1;
  return;
}



void __stdcall16far pass1_1038_387e(ulong param_1,int param_2,int param_3,ulong param_4,uint param_5)

{
  code **ppcVar1;
  long lVar2;
  ushort uVar3;
  int iVar4;
  ulong uVar5;
  ulong uVar6;
  ulong uVar7;
  uchar *extraout_DX;
  uchar *puVar8;
  uchar *puVar9;
  uint uVar10;
  uint extraout_DX_00;
  uint uVar11;
  astruct_302 *iVar10;
  undefined2 uVar12;
  int iStack22;
  uint uStack12;
  ulong uStack10;
  ulong uStack6;
  
  if (param_2 != param_3) {
    iVar10 = (astruct_302 *)param_1;
    uVar12 = (undefined2)(param_1 >> 0x10);
    if (param_2 < param_3) {
      uStack12 = param_3 - param_2;
      if ((iVar10->field_0x210 == 0x0) || (lVar2 = iVar10->field_0x210, *(long *)((int)lVar2 + 0xa) == 0x0)) {
        if (iVar10->field_0xc == (undefined4 *)0x0) {
          uVar11 = 0x0;
          puVar8 = (uchar *)0x0;
        }
        else {
          ppcVar1 = (code **)((int)*iVar10->field_0xc + 0x10);
          uVar11 = uStack12;
          (**ppcVar1)();
          puVar8 = extraout_DX;
        }
        uStack6 = CONCAT22(puVar8,uVar11);
        for (uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1) {
          uVar6 = uStack6;
          pass1_1030_1d58((ulong)iVar10->field_0xc);
          puVar9 = (uchar *)((uint)puVar8 | (uint)uVar6);
          if ((puVar9 != (uchar *)0x0) &&
             (uVar3 = pass1_1030_6fa0(uVar6 & 0xffff | ZEXT24(puVar8) << 0x10), uVar3 == 0xb)) {
            pass1_1030_7c50(CONCAT13((char)((uint)puVar8 >> 0x8),CONCAT12((char)puVar8,(uint)uVar6)),(long)(int)uStack12
                            ,0x4,uStack12,puVar9);
            return;
          }
          puVar8 = puVar9;
        }
      }
      else {
        lVar2 = iVar10->field_0x210;
        uVar6 = *(ulong *)((int)lVar2 + 0xa);
        for (uStack10 = 0x0; uStack10 < uVar6; uStack10 = uStack10 + 0x1) {
          uVar5 = uVar6;
          bad_1030_1312();
          uVar11 = (uint)uVar5;
          uVar10 = param_5 | uVar11;
          if (((uVar10 != 0x0) && (pass1_1030_cc44(uVar11,param_5,uStack12,param_4,0x4), uVar11 != 0x0)) &&
             (uStack12 = uStack12 - uVar11, uStack12 == 0x0)) {
            return;
          }
          param_5 = uVar10;
        }
      }
    }
    else {
      iStack22 = param_2 - param_3;
      if ((iVar10->field_0x210 == 0x0) || (lVar2 = iVar10->field_0x210, *(long *)((int)lVar2 + 0xa) == 0x0)) {
        if (iVar10->field_0xc == (undefined4 *)0x0) {
          iVar4 = 0x0;
          uVar11 = 0x0;
        }
        else {
          ppcVar1 = (code **)((int)*iVar10->field_0xc + 0x10);
          iVar4 = iStack22;
          (**ppcVar1)();
          uVar11 = extraout_DX_00;
        }
        uStack6 = CONCAT22(uVar11,iVar4);
        for (uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1) {
          uVar6 = uStack6;
          pass1_1030_1d58((ulong)iVar10->field_0xc);
          uVar10 = uVar11 | (uint)uVar6;
          if ((uVar10 != 0x0) && (uVar3 = pass1_1030_6fa0(uVar6 & 0xffff | (ulong)uVar11 << 0x10), uVar3 == 0xb)) {
            pass1_1030_6e9c(CONCAT13((char)(uVar11 >> 0x8),CONCAT12((char)uVar11,(uint)uVar6)),(long)iStack22,0x4);
            return;
          }
          uVar11 = uVar10;
        }
      }
      else {
        lVar2 = iVar10->field_0x210;
        uVar6 = *(ulong *)((int)lVar2 + 0xa);
        for (uStack10 = 0x0; uStack10 < uVar6; uStack10 = uStack10 + 0x1) {
          uVar7 = uVar6;
          bad_1030_1312();
          uVar5 = (ulong)param_5;
          uVar11 = (uint)uVar7;
          param_5 = param_5 | uVar11;
          if (param_5 != 0x0) {
            pass1_1030_ce72(uVar5 << 0x10 | uVar7 & 0xffff,iStack22,param_4,0x4);
            iStack22 = iStack22 - uVar11;
            if (iStack22 == 0x0) {
              return;
            }
          }
        }
      }
    }
  }
  return;
}



void __stdcall16far pass1_1038_3aa6(ulong param_1,ushort param_2,uint param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  ushort uVar3;
  ulong uVar4;
  ulong uVar5;
  uint extraout_DX;
  uint uVar6;
  uint uVar7;
  int iVar8;
  undefined2 uVar9;
  ulong uStack12;
  ulong uStack8;
  
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar8 = (int)param_1;
  if ((*(long *)(iVar8 + 0x210) == 0x0) || (uVar2 = *(undefined4 *)(iVar8 + 0x210), *(long *)((int)uVar2 + 0xa) == 0x0))
  {
    if (*(long *)(iVar8 + 0xc) == 0x0) {
      param_2 = 0x0;
      uVar6 = 0x0;
    }
    else {
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0xc) + 0x10);
      (**ppcVar1)();
      uVar6 = extraout_DX;
    }
    uStack8 = CONCAT22(uVar6,param_2);
    for (uStack12 = 0x0; uStack12 < uStack8; uStack12 = uStack12 + 0x1) {
      uVar4 = uStack8;
      pass1_1030_1d58(*(ulong *)(iVar8 + 0xc));
      uVar7 = uVar6 | (uint)uVar4;
      if ((uVar7 != 0x0) && (uVar3 = pass1_1030_6fa0(uVar4 & 0xffff | (ulong)uVar6 << 0x10), uVar3 == 0xb)) {
        pass1_1030_6b86(uVar4 & 0xffff | (ulong)uVar6 << 0x10,0xb,0x1030);
        return;
      }
      uVar6 = uVar7;
    }
  }
  else {
    uVar2 = *(undefined4 *)(iVar8 + 0x210);
    uVar4 = *(ulong *)((int)uVar2 + 0xa);
    for (uStack12 = 0x0; uStack12 < uVar4; uStack12 = uStack12 + 0x1) {
      uVar5 = uVar4;
      bad_1030_1312();
      uVar6 = param_3 | (uint)uVar5;
      if (uVar6 != 0x0) {
        pass1_1030_ce2e((uint)uVar5,param_3,0x4);
      }
      param_3 = uVar6;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_3ba0(ulong param_1)

{
  undefined4 *puVar1;
  code **ppcVar2;
  char cVar3;
  undefined4 *puVar4;
  ulong uVar5;
  uint uVar6;
  uint uVar7;
  ulong uVar8;
  uchar *puVar9;
  uchar *extraout_DX;
  uchar *puVar10;
  uint uVar11;
  astruct_428 *iVar13;
  undefined2 uVar12;
  undefined2 uVar13;
  ushort unaff_SS;
  ulong *puVar14;
  ulong uVar15;
  ulong uStack20;
  
  uVar12 = (undefined2)(param_1 >> 0x10);
  iVar13 = (astruct_428 *)param_1;
  puVar1 = *(undefined4 **)&iVar13->field_0x210;
  uVar6 = *(uint *)((int)&iVar13->field_0x210 + 0x2);
  if ((uVar6 | (uint)puVar1) != 0x0) {
    ppcVar2 = (code **)*puVar1;
    (**ppcVar2)();
  }
  puVar14 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
  puVar9 = (uchar *)((ulong)puVar14 >> 0x10);
  uVar8 = (ulong)puVar14 & 0xffff;
  pass1_1038_4d6e(param_1,puVar14,(uint)uVar8,puVar9);
  uVar5 = uVar8 & 0xffff;
  puVar4 = (undefined4 *)(uVar5 | ZEXT24(puVar9) << 0x10);
  ppcVar2 = (code **)((int)*puVar4 + 0x10);
  (**ppcVar2)(0x1008,(int)uVar8,puVar9);
  uVar6 = (uint)uVar8;
  if ((extraout_DX == (uchar *)0x0) && (uVar6 < 0x5)) {
    uVar6 = 0x5;
  }
  uVar6 = uVar6 + 0x1;
  uVar13 = 0x1000;
  puVar10 = extraout_DX;
  uVar7 = uVar6;
  mem_op_1000_179c(0x1c,extraout_DX,0x1000);
  uVar11 = (uint)puVar10 | uVar7;
  if (uVar11 == 0x0) {
    iVar13->field_0x210 = 0x0;
  }
  else {
    uVar11 = (int)uVar6 >> 0xf;
    cVar3 = (char)((ulong)uVar6 >> 0x8);
    uVar13 = 0x1030;
    struct_1030_11aa((ushort *)CONCAT22(puVar10,uVar7),0x5,CONCAT13(cVar3 >> 0xf,CONCAT12(cVar3 >> 0x7,uVar6)),unaff_SS)
    ;
    *(uint *)&iVar13->field_0x210 = uVar6;
    *(uint *)((int)&iVar13->field_0x210 + 0x2) = uVar11;
  }
  uVar15 = iVar13->field_0x210;
  *(undefined2 *)((int)uVar15 + 0x1a) = 0x0;
  for (uStack20 = 0x0; uStack20 < (uVar8 & 0xffff | ZEXT24(extraout_DX) << 0x10); uStack20 = uStack20 + 0x1) {
    uVar15 = pass1_1030_1d7c((int)(uVar8 & 0xffff),uVar11,(ulong)puVar4);
    uVar6 = (uint)(uVar15 >> 0x10);
    uVar11 = uVar6 | (uint)uVar15;
    if (uVar11 != 0x0) {
      pass1_1030_1358(iVar13->field_0x210,(uint)uVar15,uVar6,uStack20 + 0x1,unaff_SS);
    }
    uVar13 = 0x1030;
  }
  if (puVar4 != (undefined4 *)0x0) {
    ppcVar2 = (code **)*puVar4;
    (**ppcVar2)(uVar13,(int)uVar5,(char)puVar9,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_3cc0(ulong param_1,uint param_2,uchar *param_3,ushort param_4,ushort param_5,ushort param_6,ushort param_7)

{
  long lVar1;
  code **ppcVar2;
  uint uVar3;
  undefined4 *puVar4;
  ushort uVar5;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  undefined2 extraout_DX_01;
  uint extraout_DX_02;
  uint uVar6;
  uchar *extraout_DX_03;
  uchar *puVar7;
  uchar *extraout_DX_04;
  undefined4 *puVar8;
  uchar *puVar9;
  undefined2 uVar10;
  ulong *puVar11;
  ulong uVar12;
  ulong uVar13;
  undefined uVar14;
  undefined uVar15;
  undefined uVar16;
  undefined uVar17;
  undefined4 *puStack26;
  ulong uStack22;
  ulong uStack18;
  ulong uStack14;
  undefined4 *puStack10;
  
  if (param_4 == 0x1e) {
    uVar10 = 0x1008;
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x27);
    puVar9 = (uchar *)((ulong)puVar11 >> 0x10);
    puVar8 = (undefined4 *)puVar11;
    pass1_1038_4e78((uint)puVar8,puVar9,param_1,puVar11);
    puStack10 = (undefined4 *)CONCAT22(puVar9,puVar8);
    ppcVar2 = (code **)((int)*puStack10 + 0x10);
    puVar4 = puVar8;
    (**ppcVar2)(0x1008,puVar8,puVar9);
    uStack14 = CONCAT22(extraout_DX_00,puVar4);
    puVar7 = extraout_DX_00;
    for (uStack18 = 0x0; uStack18 < uStack14; uStack18 = uStack18 + 0x1) {
      uVar12 = pass1_1030_1d7c(puVar4,puVar7,(ulong)puStack10);
      puVar7 = (uchar *)((uint)(uVar12 >> 0x10) | (uint)uVar12);
      if (puVar7 != (uchar *)0x0) {
        uVar5 = pass1_1030_bfb8(uVar12,param_7);
        puStack26 = (undefined4 *)CONCAT22(puVar7,uVar5);
        puVar7 = (uchar *)((uint)puVar7 | uVar5);
        if (puVar7 != (uchar *)0x0) {
          pass1_1028_b58e(uVar12);
          if (CONCAT22(param_3,param_2) <= puStack26) {
            uVar10 = 0x1030;
            pass1_1030_7ddc(CONCAT22(extraout_DX_01,uVar5),
                            CONCAT13((char)((uint)param_3 >> 0x8),CONCAT12((char)param_3,param_2)),0x1e,param_2,param_3,
                            param_5,param_6,param_7);
            break;
          }
          puVar7 = param_3;
          pass1_1030_7ddc(CONCAT22(extraout_DX_01,uVar5),(long)puStack26,0x1e,param_2,param_3,param_5,param_6,param_7);
          lVar1 = CONCAT22(param_3,param_2) - (long)puStack26;
          param_2 = (uint)lVar1;
          param_3 = (uchar *)((ulong)lVar1 >> 0x10);
        }
      }
      uVar10 = 0x1030;
    }
    puStack26 = puStack10;
    if (puStack10 == (undefined4 *)0x0) {
      return;
    }
  }
  else {
    if (param_4 != 0x21) {
      uVar10 = 0x1008;
      puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x3);
      puVar7 = (uchar *)((ulong)puVar11 >> 0x10);
      uVar3 = (uint)puVar11;
      pass1_1038_4e78(uVar3,puVar7,param_1,puVar11);
      puStack26 = (undefined4 *)CONCAT22(puVar7,uVar3);
      ppcVar2 = (code **)((int)*puStack26 + 0x10);
      (**ppcVar2)(0x1008,uVar3,puVar7);
      uStack22 = CONCAT22(extraout_DX,uVar3);
      uStack18 = 0x0;
      puVar7 = extraout_DX;
LAB_1038_3e9c:
      if (uStack18 < uStack22) {
        uVar10 = 0x1030;
        uVar12 = pass1_1030_1d7c(uVar3,puVar7,(ulong)puStack26);
        puVar7 = (uchar *)((uint)(uVar12 >> 0x10) | (uint)uVar12);
        if (puVar7 == (uchar *)0x0) goto LAB_1038_3e98;
        uVar10 = SUB42(&USHORT_1050_1028,0x0);
        uVar13 = pass1_1028_45e2(uVar12,(uint)uVar12,(int)puVar7,param_7);
        uVar6 = (uint)uVar13;
        puVar7 = (uchar *)((uint)(uVar13 >> 0x10) | uVar6);
        if (puVar7 == (uchar *)0x0) goto LAB_1038_3e98;
        pass1_1028_b58e(uVar12);
        uVar12 = CONCAT22(param_3,param_2);
        if (uVar13 < uVar12) {
          uVar10 = 0x1030;
          puVar7 = param_3;
          pass1_1030_7ddc(CONCAT22(extraout_DX_04,uVar6),uVar13,param_4,param_2,param_3,param_5,param_6,param_7);
          lVar1 = CONCAT22(param_3,param_2) - uVar13;
          param_2 = (uint)lVar1;
          param_3 = (uchar *)((ulong)lVar1 >> 0x10);
          goto LAB_1038_3e98;
        }
        uVar16 = SUB21(param_3,0x0);
        uVar17 = (undefined)((uint)param_3 >> 0x8);
        uVar14 = (undefined)uVar6;
        uVar15 = (undefined)(uVar6 >> 0x8);
        puVar7 = extraout_DX_04;
LAB_1038_3e67:
        uVar10 = 0x1030;
        pass1_1030_7ddc(CONCAT22(puVar7,CONCAT11(uVar15,uVar14)),CONCAT13(uVar17,CONCAT12(uVar16,param_2)),param_4,
                        (uint)uVar12,param_3,param_5,param_6,param_7);
      }
      goto LAB_1038_3e6c;
    }
    uVar10 = 0x1008;
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0xa);
    puVar7 = (uchar *)((ulong)puVar11 >> 0x10);
    uVar3 = (uint)puVar11;
    pass1_1038_4e78(uVar3,puVar7,param_1,puVar11);
    puStack26 = (undefined4 *)CONCAT22(puVar7,uVar3);
    ppcVar2 = (code **)((int)*puStack26 + 0x10);
    (**ppcVar2)(0x1008,uVar3,puVar7);
    uStack22 = CONCAT22(extraout_DX_02,uVar3);
    uVar6 = extraout_DX_02;
    for (uStack18 = 0x0; uStack18 < uStack22; uStack18 = uStack18 + 0x1) {
      uVar10 = 0x1030;
      uVar13 = pass1_1030_1d7c(uVar3,uVar6,(ulong)puStack26);
      uVar12 = uVar13 & 0xffff;
      uVar6 = (uint)(uVar13 >> 0x10) | (uint)uVar12;
      if (uVar6 != 0x0) {
        uVar16 = SUB21(param_3,0x0);
        uVar17 = (undefined)((uint)param_3 >> 0x8);
        pass1_1028_b58e(uVar13);
        uVar14 = (undefined)uVar12;
        uVar15 = (undefined)(uVar12 >> 0x8);
        param_3 = extraout_DX_03;
        puVar7 = extraout_DX_03;
        goto LAB_1038_3e67;
      }
    }
LAB_1038_3e6c:
    if (puStack26 == (undefined4 *)0x0) {
      return;
    }
    puVar9 = (uchar *)((ulong)puStack26 >> 0x10);
    puVar8 = (undefined4 *)puStack26;
  }
  ppcVar2 = (code **)*puVar8;
  (**ppcVar2)(uVar10,(int)puStack26,(char)puVar9,0x1);
  return;
LAB_1038_3e98:
  uStack18 = uStack18 + 0x1;
  goto LAB_1038_3e9c;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_3efc(ushort param_1,ushort param_2,ulong param_3,ulong param_4,int param_5,ushort param_6)

{
  code **ppcVar1;
  undefined4 *puStack6;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_4,(uint)(param_4 >> 0x10));
  puStack6 = (undefined4 *)CONCAT22(param_6,param_5);
  *(undefined4 *)(param_5 + 0x1c) = *(undefined4 *)((int)param_3 + 0x4);
  ppcVar1 = (code **)((int)*puStack6 + 0x58);
  (**ppcVar1)((int)&USHORT_1050_1028,param_5,param_6,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_3f38(ulong *param_1,ulong *param_2,ulong param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  int iVar2;
  undefined2 extraout_DX;
  ushort extraout_DX_00;
  ulong *puVar3;
  undefined2 uVar4;
  undefined4 uVar5;
  undefined2 uVar6;
  ulong uStack10;
  undefined4 *puStack6;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_3,(uint)(param_3 >> 0x10));
  puStack6 = (undefined4 *)CONCAT22(param_5,param_4);
  iVar2 = param_4;
  pass1_1028_b58e(CONCAT22(param_5,param_4));
  uStack10 = CONCAT22(extraout_DX,iVar2);
  uVar5 = *(undefined4 *)(iVar2 + 0x4);
  ppcVar1 = (code **)((int)*param_1 + 0x18);
  (**ppcVar1)((int)&USHORT_1050_1028,param_1,uVar5);
  uVar6 = 0x0;
  uVar4 = 0x0;
  ppcVar1 = (code **)((int)*param_2 + 0x8);
  puVar3 = param_2;
  (**ppcVar1)();
  pass1_1030_73ee(uStack10,*(ulong *)((int)param_2 + 0x4),extraout_DX_00);
  ppcVar1 = (code **)((int)*puStack6 + 0x58);
  (**ppcVar1)(0x1030,param_4,param_5,param_2,puVar3,uVar4,uVar5,uVar6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_3fb0(ulong param_1)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x200);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_3fca(ulong param_1,uint param_2,ushort param_3)

{
  undefined4 uVar1;
  code **ppcVar2;
  ushort uVar3;
  uint extraout_DX;
  uint uVar4;
  uint extraout_DX_00;
  ushort uVar5;
  int iVar6;
  int unaff_DI;
  ushort uVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  ulong uVar10;
  ushort *puVar11;
  undefined uVar12;
  undefined uVar13;
  undefined uVar14;
  undefined uVar15;
  ushort uVar16;
  int iStack38;
  int local_24;
  undefined local_22 [0x2];
  int *piStack32;
  undefined2 uStack30;
  undefined *puStack28;
  undefined2 uStack26;
  undefined2 uStack24;
  ulong uStack22;
  uint uStack18;
  uint uStack16;
  astruct_18 *paStack14;
  astruct_18 *paStack10;
  ulong uStack6;
  
  uVar7 = (ushort)(param_1 >> 0x10);
  uVar5 = (ushort)param_1;
  if (*(long *)(uVar5 + 0xc) == 0x0) {
    param_2 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar5 + 0xc) + 0x10);
    (**ppcVar2)();
    uVar4 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar4,param_2);
  PTR_LOOP_1050_5f2e = (undefined *)(uVar4 | param_2);
  if (PTR_LOOP_1050_5f2e != (uchar *)0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar3 = fn_ptr_op_1000_1708((int)uStack6 << 0x2,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
    paStack10 = (astruct_18 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar9 = 0x1000;
    uVar3 = fn_ptr_op_1000_1708((int)uStack6 << 0x2,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
    paStack14 = (astruct_18 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    for (uStack22 = 0x0; uStack22 < uStack6; uStack22 = uStack22 + 0x1) {
      uVar1 = *(undefined4 *)(uVar5 + 0xc);
      ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar5 + 0xc) + 0x4);
      uVar10 = uStack6;
      (**ppcVar2)(uVar9,(char)uVar1,(int)((ulong)uVar1 >> 0x10),(char)uStack22,(int)(uStack22 >> 0x10));
      uVar4 = (uint)uVar10;
      PTR_LOOP_1050_5f2e = (undefined *)(extraout_DX_00 | uVar4);
      uStack18 = uVar4;
      uStack16 = extraout_DX_00;
      if (PTR_LOOP_1050_5f2e != (uchar *)0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,extraout_DX_00);
        uStack22._0_2_ = (int)uStack22 * 0x4;
        uVar8 = (undefined2)((ulong)paStack10 >> 0x10);
        iVar6 = (int)paStack10;
        *(uint *)((int)uStack22 + iVar6) = uVar4;
        *(uchar **)((int)uStack22 + iVar6 + 0x2) = PTR_LOOP_1050_5f2e;
        uVar9 = 0x1030;
        uVar10 = struct_op_1030_73a8(CONCAT22(PTR_LOOP_1050_5f2e,*(undefined2 *)((int)uStack22 + iVar6)));
        PTR_LOOP_1050_5f2e = (undefined *)(uVar10 >> 0x10);
        uVar8 = (undefined2)((ulong)paStack14 >> 0x10);
        *(undefined2 *)((int)paStack14 + (int)uStack22) = (int)uVar10;
        *(uchar **)((int)paStack14 + (int)uStack22 + 0x2) = PTR_LOOP_1050_5f2e;
      }
    }
    for (uStack22 = 0x0; uStack22 < uStack6; uStack22 = uStack22 + 0x1) {
      uVar9 = (undefined2)((ulong)paStack14 >> 0x10);
      iVar6 = (int)paStack14;
      if ((*(long *)((int)uStack22 * 0x4 + iVar6) != 0x0) &&
         (uVar1 = *(undefined4 *)((int)uStack22 * 0x4 + iVar6), *(undefined2 *)((int)uVar1 + 0x1a) = 0x0,
         uVar1 = *(undefined4 *)((int)uStack22 * 0x4 + iVar6), *(int *)((int)uVar1 + 0x12) == 0x5)) {
        pass1_1028_bdac(*(ulong **)((int)uStack22 * 0x4 + iVar6),0x6,(ushort)&USHORT_1050_1028);
      }
    }
    *(undefined2 *)(uVar5 + 0x204) = 0x0;
    puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,PTR_LOOP_1050_5f2e,unaff_DI);
    uStack30 = (undefined2)((ulong)puVar11 >> 0x10);
    uStack26 = SUB42(puVar11,0x0);
    puStack28 = PTR_LOOP_1050_13ae;
    if (PTR_LOOP_1050_13ae == (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1)) {
      *(undefined2 *)(uVar5 + 0x204) = 0x1;
    }
    uStack24 = uStack30;
    pass1_1038_5a96(uVar5,uVar7,uStack6,(ulong)paStack14);
    pass1_1038_5cc6(param_1,uStack6,(ulong)paStack14,(ulong)paStack10,0x0,0x2);
    pass1_1038_5b3c(uVar5,uVar7,uStack6,(ulong)paStack14);
    pass1_1038_5cc6(param_1,uStack6,(ulong)paStack14,(ulong)paStack10,0x0,0x1);
    uVar14 = SUB21(local_22,0x0);
    uVar15 = (undefined)((uint)local_22 >> 0x8);
    piStack32 = &local_24;
    uVar12 = SUB21(piStack32,0x0);
    uVar13 = (undefined)((uint)piStack32 >> 0x8);
    uVar1 = *(undefined4 *)(uVar5 + 0x8);
    uVar3 = param_3;
    uVar16 = param_3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
    pass1_1030_5b1c(CONCAT22(uStack30,piStack32),(ushort *)CONCAT22(uVar3,CONCAT11(uVar13,uVar12)),
                    (ushort *)CONCAT22(uVar16,CONCAT11(uVar15,uVar14)));
    for (iStack38 = 0x1; iStack38 <= local_24; iStack38 = iStack38 + 0x1) {
      pass1_1038_58e6(uVar5,uVar7,uStack6,(ulong)paStack14,(ulong)paStack10,iStack38,param_3);
      pass1_1038_5cc6(param_1,uStack6,(ulong)paStack14,(ulong)paStack10,iStack38,0x3);
    }
    pass1_1038_5a16(uVar5,uVar7,uStack6,(ulong)paStack14);
    for (uStack22 = 0x0; uStack22 < uStack6; uStack22 = uStack22 + 0x1) {
      uVar9 = (undefined2)((ulong)paStack14 >> 0x10);
      iVar6 = (int)paStack14;
      if ((*(long *)((int)uStack22 * 0x4 + iVar6) != 0x0) &&
         (uVar1 = *(undefined4 *)((int)uStack22 * 0x4 + iVar6), *(int *)((int)uVar1 + 0x12) != 0x5)) {
        uVar1 = *(undefined4 *)((int)uStack22 * 0x4 + iVar6);
        ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)uStack22 * 0x4 + iVar6) + 0x28);
        (**ppcVar2)(0x1030,(char)uVar1,(int)((ulong)uVar1 >> 0x10));
      }
    }
    fn_ptr_1000_17ce(paStack10,0x1000);
    fn_ptr_1000_17ce(paStack14,0x1000);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_42cc(ulong param_1,ushort param_2)

{
  code **ppcVar1;
  undefined4 uVar2;
  bool bVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uchar *puVar7;
  uint extraout_DX;
  uint uVar8;
  uint extraout_DX_00;
  int iVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  ulong *puVar12;
  undefined4 *puVar13;
  ulong uStack24;
  ulong uStack18;
  undefined4 *puStack10;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar9 = (int)param_1;
  if (*(long *)(iVar9 + 0x1f6) == 0x0) {
    return;
  }
  uVar11 = 0x1008;
  puVar12 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2d);
  puVar7 = (uchar *)((ulong)puVar12 >> 0x10);
  uVar4 = (uint)puVar12;
  pass1_1038_4d6e(param_1,puVar12,uVar4,puVar7);
  puStack10 = (undefined4 *)CONCAT22(puVar7,uVar4);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  uVar5 = uVar4;
  (**ppcVar1)(0x1008,uVar4,puVar7);
  uStack18 = CONCAT22(extraout_DX,uVar5);
  bVar3 = false;
  uVar8 = extraout_DX;
  for (uStack24 = 0x0; uStack24 < uStack18; uStack24 = uStack24 + 0x1) {
    uVar11 = 0x1030;
    puVar13 = (undefined4 *)pass1_1030_1d7c(uVar5,uVar8,(ulong)puStack10);
    uVar6 = (uint)puVar13;
    uVar8 = (uint)((ulong)puVar13 >> 0x10) | uVar6;
    if (uVar8 != 0x0) {
      ppcVar1 = (code **)((int)*puVar13 + 0x50);
      (**ppcVar1)();
      uVar8 = extraout_DX_00;
      if (uVar6 != 0x0) {
        bVar3 = true;
      }
    }
  }
  if (bVar3) {
    uVar2 = *(undefined4 *)(iVar9 + 0x1f6);
    *(undefined4 *)((int)uVar2 + 0x1aa) = 0x0;
  }
  else {
    uVar11 = 0x1030;
    pass1_1030_38b8();
    uVar8 = uVar8 | (uint)uStack18;
    if (uVar8 != 0x0) {
      uVar11 = 0x1030;
      pass1_1030_326a(*(ulong *)(iVar9 + 0x1f6),uStack18,uVar8,param_2);
    }
  }
  if (puStack10 != (undefined4 *)0x0) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(uVar11,uVar4,(char)puVar7,0x1);
  }
  return;
}


