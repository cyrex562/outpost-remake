


void __stdcall16far pass1_1030_8a2c(ushort *param_1)

{
  uint uVar1;
  astruct_18 *paVar2;
  astruct_613 *iVar3;
  undefined2 uVar3;
  int iStack4;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_613 *)param_1;
  *param_1 = 0x8e38;
  iVar3->field_0x2 = 0x1030;
  iStack4 = 0x0;
  do {
    paVar2 = *(astruct_18 **)(&iVar3[0x1].field_0x0 + iStack4 * 0x4);
    uVar1 = (&iVar3[0x1].field_0x2)[iStack4 * 0x2];
    if ((uVar1 | (uint)paVar2) != 0x0) {
      pass1_1030_8604((astruct_18 **)((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10));
      fn_ptr_1000_17ce(paVar2,0x1000);
    }
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x5);
  fn_ptr_1030_84d0((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x4);
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1030_8aa0(ulong param_1,ulong param_2,ushort *param_3,uint param_4,ushort param_5)

{
  uint uVar1;
  int unaff_DI;
  ulong local_12;
  undefined *puStack14;
  ulong uStack12;
  undefined local_8 [0x2];
  undefined local_6 [0x2];
  undefined local_4 [0x2];
  
  puStack14 = local_8;
  pass1_1008_3eb4(param_3,(ushort *)CONCAT13((char)(param_5 >> 0x8),CONCAT12((char)param_5,puStack14)),
                  (ushort *)CONCAT22(param_5,local_6),(ushort *)CONCAT22(param_5,local_4));
  bad_1030_8cd2();
  uStack12 = CONCAT22(param_4,puStack14);
  uVar1 = param_4 | (uint)puStack14;
  if (uVar1 != 0x0) {
    pass1_1030_8d9e(param_1,param_5);
    local_12 = param_2;
    pass1_1030_8660(uStack12,(ulong *)CONCAT22(param_5,&local_12),(ushort)puStack14,(uint)&local_12,uVar1,param_5,
                    unaff_DI);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1030_8b00(ulong param_1,ushort *param_2,ushort *param_3,ushort param_4)

{
  undefined4 *puVar1;
  int *piVar2;
  ushort uVar3;
  undefined4 local_2a;
  undefined4 uStack38;
  undefined4 uStack28;
  undefined4 *puStack18;
  undefined4 *puStack16;
  int *piStack14;
  int local_c;
  undefined local_a [0x4];
  undefined4 uStack6;
  
  uStack6 = 0x0;
  puVar1 = (undefined4 *)(local_a + 0x2);
  piVar2 = &local_c;
  pass1_1008_3eb4(param_2,(ushort *)CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,piVar2)),
                  (ushort *)CONCAT22(param_4,local_a),(ushort *)CONCAT22(param_4,puVar1));
  bad_1030_8cd2();
  puStack16 = puVar1;
  piStack14 = piVar2;
  pass1_1030_8d9e(param_1,param_4);
  puStack18 = puVar1;
  pass1_1030_861a((ushort)puStack16,(ushort)piStack14,(ushort)puVar1,(uint)puVar1,(uint)piVar2,param_4);
  uStack38 = *puVar1;
  uVar3 = *(ushort *)((int)puVar1 + 0x2);
  uStack38._3_1_ = (char)((ulong)uStack38 >> 0x18);
  uStack6 = uStack38;
  if (uStack38._3_1_ == '\0') {
    puVar1 = &local_2a;
    uStack28 = uStack38;
    pass1_1030_8c66(param_1,local_c,(byte *)local_a,(ushort)((ulong)local_a >> 0x10),(ulong *)CONCAT22(param_4,puVar1),
                    uVar3);
    uStack6 = *puVar1;
    uVar3 = *(ushort *)((int)puVar1 + 0x2);
  }
  *param_3 = (ushort)uStack6;
  *(ushort *)((int)param_3 + 0x2) = uVar3;
  return;
}



void __stdcall16far pass1_1030_8bac(ulong param_1,ushort param_2)

{
  int iStack4;
  
  iStack4 = 0x0;
  do {
    pass1_1030_86ec(*(undefined4 *)((int)param_1 + 0x38 + iStack4 * 0x4),param_2);
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x5);
  return;
}



void __stdcall16far pass1_1030_8bdc(ulong param_1,ulong param_2,ushort *param_3,int param_4,ushort param_5)

{
  undefined *puVar1;
  ulong local_12;
  undefined *puStack14;
  long *plStack12;
  undefined local_8 [0x2];
  undefined local_6 [0x2];
  undefined local_4 [0x2];
  
  puStack14 = local_4;
  puVar1 = local_8;
  pass1_1008_3eb4(param_3,(ushort *)CONCAT13((char)(param_5 >> 0x8),CONCAT12((char)param_5,puVar1)),
                  (ushort *)CONCAT22(param_5,local_6),(ushort *)CONCAT22(param_5,puStack14));
  bad_1030_8cd2();
  plStack12 = (long *)CONCAT22(puVar1,puStack14);
  pass1_1030_8d9e(param_1,param_5);
  local_12 = param_2;
  pass1_1030_871e(plStack12,(ulong *)CONCAT22(param_5,&local_12),(ushort)puStack14,param_4,param_5);
  return;
}



void __stdcall16far pass1_1030_8c38(ulong param_1,int param_2,ushort param_3)

{
  int iStack4;
  
  iStack4 = 0x0;
  do {
    pass1_1030_877c(*(uint **)((int)param_1 + 0x38 + iStack4 * 0x4),param_2,param_3);
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x5);
  return;
}



void __stdcall16far
pass1_1030_8c66(ulong param_1,int param_2,byte *param_3,ushort param_4,ulong *param_5,ushort param_6)

{
  byte bVar1;
  uint uVar2;
  ulong uStack6;
  
  pass1_1008_4544(*(ulong *)((int)param_1 + 0x12));
  bVar1 = *param_3;
  uVar2 = (uint)bVar1;
  uStack6 = (ulong)(uVar2 + 0x1);
  if (0x0 < param_2) {
    if (uVar2 == 0x0) {
      uStack6 = 0x7;
    }
    else {
      if (((bVar1 == 0x0) || (SBORROW2(uVar2,0x1))) || (0x1 < (int)(uVar2 - 0x1))) {
        uStack6 = 0x9;
      }
      else {
        uStack6 = 0x8;
      }
    }
  }
  *param_5 = uStack6;
  return;
}



void __stdcall16far bad_1030_8cd2(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_8d08(ulong param_1,ushort param_2)

{
  int *piVar1;
  undefined4 uVar2;
  ushort uVar3;
  undefined2 uVar4;
  ulong uVar5;
  ulong uStack16;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    uVar4 = (undefined2)(param_1 >> 0x10);
    piVar1 = (int *)((int)param_1 + 0x1e);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    uVar3 = iStack4 * 0x6;
    uVar2 = *(undefined4 *)((int)param_1 + 0x1a);
    *(undefined2 *)((int)uVar2 + uVar3 + 0x4) = 0x0;
    pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x500);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,param_2);
    uStack16 = CONCAT22(param_2,uVar3);
    uVar5 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,param_2,0x7);
    param_2 = (ushort)(uVar5 >> 0x10);
    pass1_1030_7e5a(uStack16,uVar5 & 0xffff | (ulong)param_2 << 0x10,param_2);
    iStack4 = iStack4 + 0x1;
  }
  return;
}



void __stdcall16far pass1_1030_8d9e(ulong param_1,ushort param_2)

{
  undefined local_c [0x2];
  undefined local_a [0x2];
  undefined local_8 [0x6];
  
  pass1_1008_3e38((ushort *)CONCAT22(param_2,local_8));
  pass1_1008_6d64((ushort *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x28)),(ushort *)CONCAT22(param_2,local_8));
  pass1_1008_3e94((ushort *)CONCAT22(param_2,local_8),(ushort *)CONCAT22(param_2,local_c),
                  (ushort *)CONCAT22(param_2,local_a));
  return;
}



astruct_18 * __stdcall16far pass1_1030_8e12(astruct_18 *param_1,byte param_2)

{
  pass1_1030_8a2c(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1030_8e3c(ushort param_1,uint param_2,uchar *param_3,ulong param_4,ulong param_5)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  int unaff_DI;
  ulong *puVar5;
  ushort *puVar6;
  ushort uVar7;
  
  mem_op_1000_179c(0xc,param_3,0x1000);
  if (((uint)param_3 | param_2) == 0x0) {
    puVar5 = (ulong *)0x0;
  }
  else {
    puVar5 = (ulong *)set_struct_1008_574a((astruct_21 *)CONCAT22(param_3,param_2));
  }
  if (param_5._3_1_ == '\x04') {
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,(uchar *)((ulong)puVar5 >> 0x10),unaff_DI);
    uVar3 = (uint)((ulong)puVar6 >> 0x10);
    uVar1 = *(uint *)((int)puVar6 + 0x1e);
    uVar2 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_5,(uint)(param_5 >> 0x10));
    uVar7 = (ushort)(param_4 >> 0x10);
    uVar4 = uVar3;
    if ((int)uVar1 < 0x1) {
      pass1_1030_9296(param_4,puVar5,CONCAT22(uVar3,uVar2),param_1,uVar2,uVar3);
      pass1_1030_951a(param_1,uVar4,param_4,puVar5,CONCAT22(uVar3,uVar2));
    }
    else {
      pass1_1030_9adc((ushort)param_4,uVar7,puVar5,CONCAT22(uVar3,uVar2),uVar2,uVar3);
      pass1_1030_9c1c(param_4,puVar5,CONCAT22(uVar3,uVar2));
    }
    pass1_1030_9d42(param_1,uVar4,(ushort)param_4,uVar7,puVar5,CONCAT22(uVar3,uVar2));
  }
  return (ulong)puVar5;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_8f04(ushort param_1,ushort param_2,ulong param_3,ulong param_4,uint param_5)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  ulong uVar5;
  undefined2 uVar6;
  int iStack8;
  ulong uStack6;
  
  pass1_1038_53ba(param_3,0x1);
  if ((((param_5 != 0x0) || (0x1 < (uint)param_4)) &&
      ((pass1_1038_53ba(param_3,0x2), param_5 != 0x0 || (0x1 < (uint)param_4)))) &&
     ((pass1_1038_53ba(param_3,0x3), param_5 != 0x0 || (0x1 < (uint)param_4)))) {
    pass1_1038_53ba(param_3,0x4);
    uVar5 = (ulong)param_5;
    if ((param_5 != 0x0) || (0x1 < (uint)param_4)) {
      empty_1038_540a();
      uStack6 = param_4 & 0xffff | uVar5 << 0x10;
      iStack8 = 0x0;
      do {
        uVar3 = (uint)uVar5;
        uVar2 = (uint)param_4;
        if (0x0 < *(int *)(iStack8 * 0x2 + (int)_PTR_LOOP_1050_580e)) {
          empty_1038_540a();
          uVar6 = (undefined2)((ulong)_PTR_LOOP_1050_580e >> 0x10);
          uVar1 = *(uint *)(iStack8 * 0x2 + (int)_PTR_LOOP_1050_580e);
          param_4 = (ulong)uVar1;
          uVar4 = (int)uVar1 >> 0xf;
          uVar5 = (ulong)uVar4;
          if ((uVar3 <= uVar4) && ((uVar3 < uVar4 || (uVar2 < uVar1)))) {
            if (0x1c < iStack8) {
              return;
            }
            uVar2 = *(uint *)(iStack8 * 0x2 + (int)_PTR_LOOP_1050_580e);
            param_4 = SEXT24((int)uVar2);
            uVar5 = param_4 >> 0x10;
            if ((long)uStack6 < (long)param_4) {
              return;
            }
            uStack6 = CONCAT22(((int)(uStack6 >> 0x10) - ((int)uVar2 >> 0xf)) - (uint)((uint)uStack6 < uVar2),
                               (uint)uStack6 - uVar2);
          }
        }
        iStack8 = iStack8 + 0x1;
        if (0x24 < iStack8) {
          return;
        }
      } while( true );
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far
pass1_1030_8fe4(ushort param_1,uint param_2,uint param_3,ushort param_4,ushort param_5,ushort *param_6,long param_7)

{
  int iVar1;
  uint uVar2;
  ulong uVar3;
  
  pass1_1030_627e(param_1,param_2,param_3,_PTR_LOOP_1050_5740,param_6,param_7);
  uVar2 = param_3 | param_2;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_2,param_3);
    if ((uVar2 | param_2) != 0x0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(uVar2,param_2));
      if ((uVar3 != 0x0) && ((iVar1 = *(int *)((int)uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_9048(ushort param_1,ulong param_2,int param_3,ulong param_4)

{
  int iVar1;
  ulong uVar2;
  code **ppcVar3;
  BOOL16 BVar4;
  ushort uVar5;
  ulong uVar6;
  uchar *puVar7;
  uint extraout_DX;
  uint extraout_DX_00;
  uint uVar8;
  undefined2 uVar9;
  undefined4 *puVar10;
  undefined2 uVar11;
  undefined2 uVar12;
  ulong *puVar13;
  ulong uVar14;
  ushort uVar15;
  undefined uVar16;
  ulong uStack36;
  undefined local_18 [0x2];
  int local_16;
  int local_14;
  int local_12;
  int iStack16;
  undefined4 uStack12;
  uint uStack8;
  uchar *puStack6;
  int iStack4;
  
  iStack4 = 0x8 - (uint)(param_3 == 0x0);
  puVar13 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,iStack4);
  puVar7 = (uchar *)((ulong)puVar13 >> 0x10);
  uVar8 = (uint)puVar13;
  uStack8 = uVar8;
  puStack6 = puVar7;
  pass1_1038_4e78(uVar8,puVar7,param_4,puVar13);
  uStack12 = (undefined4 *)CONCAT22(puVar7,uVar8);
  uVar12 = 0x1008;
  pass1_1008_3e38((ushort *)CONCAT22(param_1,&local_12));
  uVar2 = *(ulong *)((int)param_4 + 0x8);
  uVar11 = (undefined2)((ulong)uStack12 >> 0x10);
  uVar9 = SUB42(uStack12,0x0);
  ppcVar3 = (code **)((int)*uStack12 + 0x10);
  uVar6 = uVar2;
  (**ppcVar3)(0x1008,uVar9,uVar11);
  uVar6 = uVar6 & 0xffff | (ulong)extraout_DX << 0x10;
  uStack36 = 0x0;
  while( true ) {
    if (uVar6 <= uStack36) {
      if (uStack12 != (undefined4 *)0x0) {
        ppcVar3 = (code **)*uStack12;
        (**ppcVar3)(uVar12,(int)uStack12,(char)((ulong)uStack12 >> 0x10),0x1,uVar9,uVar11,uStack12,uStack12);
      }
      return;
    }
    ppcVar3 = (code **)((int)*uStack12 + 0x4);
    uVar14 = uVar6;
    (**ppcVar3)();
    uVar5 = (ushort)uVar14;
    uVar8 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,extraout_DX_00);
    pass1_1008_3f62((ushort *)CONCAT22(param_1,&local_12),(ushort *)CONCAT22(uVar8,uVar5 + 0xc));
    uVar12 = 0x1008;
    pass1_1008_3eb4((ushort *)CONCAT22(param_1,&local_12),(ushort *)CONCAT22(param_1,local_18),
                    (ushort *)CONCAT22(param_1,&local_16),(ushort *)CONCAT22(param_1,&local_14));
    uVar14 = struct_op_1030_73a8(CONCAT22(uVar8,uVar5));
    uVar8 = (uint)(uVar14 >> 0x10);
    iVar1 = *(int *)((int)uVar14 + 0xc);
    if (iVar1 - 0x7aU < 0x6) break;
LAB_1030_91fa:
    uStack36 = uStack36 + 0x1;
  }
  uVar12 = 0x1030;
  uVar5 = (ushort)param_2;
  uVar15 = (ushort)(param_2 >> 0x10);
  switch(iVar1) {
  default:
    iStack16 = local_16 + -0x1;
    BVar4 = pass1_1030_8fe4(param_1,(uint)&local_12,uVar8,uVar5,uVar15,(ushort *)CONCAT22(param_1,&local_12),uVar2);
    if (BVar4 != 0x0) goto LAB_1030_91cb;
    iStack16 = local_16 + 0x1;
    BVar4 = pass1_1030_8fe4(param_1,(uint)&local_12,uVar8,uVar5,uVar15,(ushort *)CONCAT22(param_1,&local_12),uVar2);
    if (BVar4 == 0x0) {
      iStack16 = local_16;
      local_12 = local_14 + -0x1;
      BVar4 = pass1_1030_8fe4(param_1,(uint)&local_12,uVar8,uVar5,uVar15,(ushort *)CONCAT22(param_1,&local_12),uVar2);
      goto joined_r0x1030911e;
    }
LAB_1030_9144:
    break;
  case 0x7b:
  case 0x7e:
    iStack16 = local_16 + -0x1;
    BVar4 = pass1_1030_8fe4(param_1,(uint)&local_12,uVar8,uVar5,uVar15,(ushort *)CONCAT22(param_1,&local_12),uVar2);
    if (BVar4 == 0x0) {
      iStack16 = local_16 + 0x1;
      goto LAB_1030_912c;
    }
    if (uStack12 == (undefined4 *)0x0) {
      return;
    }
    uVar12 = (undefined2)((ulong)uStack12 >> 0x10);
    puVar10 = (undefined4 *)uStack12;
    uVar16 = (undefined)((ulong)uStack12 >> 0x10);
    goto LAB_1030_90e6;
  case 0x7c:
  case 0x7d:
    local_12 = local_14 + -0x1;
    BVar4 = pass1_1030_8fe4(param_1,(uint)&local_12,uVar8,uVar5,uVar15,(ushort *)CONCAT22(param_1,&local_12),uVar2);
joined_r0x1030911e:
    if (BVar4 == 0x0) {
      local_12 = local_14 + 0x1;
LAB_1030_912c:
      BVar4 = pass1_1030_8fe4(param_1,(uint)&local_12,uVar8,uVar5,uVar15,(ushort *)CONCAT22(param_1,&local_12),uVar2);
      if (BVar4 != 0x0) goto LAB_1030_9144;
      goto LAB_1030_91fa;
    }
LAB_1030_91cb:
  }
  puVar10 = (undefined4 *)uStack12;
  if ((uStack12._2_2_ | (uint)puVar10) != 0x0) {
    uVar12 = (undefined2)((ulong)uStack12 >> 0x10);
    uVar16 = (undefined)((ulong)uStack12 >> 0x10);
LAB_1030_90e6:
    ppcVar3 = (code **)*puVar10;
    (**ppcVar3)(0x1030,puVar10,uVar16,0x1,uVar9,uVar11,uStack12,uStack12);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_9296(ulong param_1,ulong *param_2,ulong param_3,ushort param_4,uint param_5,uint param_6)

{
  code **ppcVar1;
  undefined2 uVar2;
  undefined *puVar3;
  undefined2 in_register_00000002;
  astruct_99 *paVar4;
  ulong uVar6;
  uint uVar7;
  uint extraout_DX;
  uint extraout_DX_00;
  uchar *puVar8;
  uchar *extraout_DX_01;
  uchar *extraout_DX_02;
  uint extraout_DX_03;
  uint uVar9;
  astruct_116 *iVar11;
  astruct_115 *iVar10;
  astruct_114 *iVar9;
  int unaff_DI;
  undefined2 uVar10;
  undefined uVar11;
  undefined local_3a [0xc];
  undefined4 uStack46;
  ulong uStack36;
  ulong uStack30;
  undefined2 uStack26;
  astruct_99 *paStack18;
  ulong uStack14;
  ushort *puStack10;
  astruct_99 *paStack6;
  astruct_113 *uVar5;
  
  paVar4 = (astruct_99 *)CONCAT22(in_register_00000002,param_5);
  pass1_1038_53ba(param_3,0x1);
  uVar7 = param_6 | (uint)paVar4;
  uVar10 = (undefined2)((ulong)param_2 >> 0x10);
  uVar11 = SUB41(param_2,0x0);
  if (uVar7 != 0x0) {
    uStack30 = _PTR_LOOP_1050_5768;
    uVar6 = _PTR_LOOP_1050_5768;
    paStack18 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar7 = (uint)((ulong)paStack18 >> 0x10);
    paVar4 = (astruct_99 *)(uVar6 & 0xffff0000 | (ulong)paStack18 & 0xffff);
    if ((uVar7 | (uint)((ulong)paStack18 & 0xffff)) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      iVar11 = (astruct_116 *)paStack18;
      paStack18->field_0x0 = 0x389a;
      iVar11->field_0x2 = 0x1008;
      iVar11->field_0x4 = 0x73;
      paStack18->field_0x0 = 0x9ec8;
      iVar11->field_0x2 = 0x1030;
      paVar4 = paStack18;
      paStack6 = paStack18;
    }
    ppcVar1 = (code **)((int)*param_2 + 0x4);
    (**ppcVar1)(0x1000,uVar11,uVar10,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
    uVar7 = extraout_DX;
  }
  pass1_1038_53ba(param_3,0x2);
  uVar7 = uVar7 | (uint)paVar4;
  if (uVar7 != 0x0) {
    uStack30 = _PTR_LOOP_1050_5768;
    uVar6 = _PTR_LOOP_1050_5768;
    paStack18 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar7 = (uint)((ulong)paStack18 >> 0x10);
    paVar4 = (astruct_99 *)(uVar6 & 0xffff0000 | (ulong)paStack18 & 0xffff);
    if ((uVar7 | (uint)((ulong)paStack18 & 0xffff)) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      iVar10 = (astruct_115 *)paStack18;
      paStack18->field_0x0 = 0x389a;
      iVar10->field_0x2 = 0x1008;
      iVar10->field_0x4 = 0x74;
      paStack18->field_0x0 = 0x9ec8;
      iVar10->field_0x2 = 0x1030;
      paVar4 = paStack18;
      paStack6 = paStack18;
    }
    ppcVar1 = (code **)((int)*param_2 + 0x8);
    (**ppcVar1)(0x1000,uVar11,uVar10,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
    uVar7 = extraout_DX_00;
  }
  pass1_1038_53ba(param_3,0x3);
  puVar8 = (uchar *)(uVar7 | (uint)paVar4);
  if (puVar8 != (uchar *)0x0) {
    uStack36 = _PTR_LOOP_1050_5768;
    uVar6 = _PTR_LOOP_1050_5768;
    paStack18 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar7 = (uint)((ulong)paStack18 >> 0x10);
    paVar4 = (astruct_99 *)(uVar6 & 0xffff0000 | (ulong)paStack18 & 0xffff);
    if ((uVar7 | (uint)((ulong)paStack18 & 0xffff)) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      iVar9 = (astruct_114 *)paStack18;
      paStack18->field_0x0 = 0x389a;
      iVar9->field_0x2 = 0x1008;
      iVar9->field_0x4 = 0x75;
      paStack18->field_0x0 = 0x9ec8;
      iVar9->field_0x2 = 0x1030;
      paVar4 = paStack18;
      paStack6 = paStack18;
    }
    ppcVar1 = (code **)((int)*param_2 + 0x8);
    (**ppcVar1)(0x1000,uVar11,uVar10,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
    puVar8 = extraout_DX_01;
  }
  pass1_1030_8f04((ushort)param_1,(ushort)(param_1 >> 0x10),param_3,(ulong)paVar4,(uint)puVar8);
  if ((int)paVar4 != 0x0) {
    uStack36 = _PTR_LOOP_1050_5768;
    paStack18 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar7 = (uint)((ulong)paStack18 >> 0x10);
    uVar5 = (astruct_113 *)paStack18;
    if ((uVar7 | (uint)uVar5) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      paStack18->field_0x0 = 0x389a;
      uVar5->field_0x2 = 0x1008;
      uVar5->field_0x4 = 0x37;
      paStack18->field_0x0 = 0x9ec8;
      uVar5->field_0x2 = 0x1030;
      paStack6 = paStack18;
    }
    ppcVar1 = (code **)((int)*param_2 + 0x8);
    (**ppcVar1)(0x1000,uVar11,uVar10,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
    puVar8 = extraout_DX_02;
  }
  puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_4,puVar8,unaff_DI);
  uVar2 = (undefined2)((ulong)puStack10 >> 0x10);
  uStack14 = *(ulong *)((int)puStack10 + 0xe);
  uVar7 = *(uint *)((int)puStack10 + 0x10);
  if ((uVar7 | (uint)uStack14) != 0x0) {
    pass1_1008_5784((ulong *)CONCAT22(param_4,local_3a),uStack14 & 0xffff | (ulong)uVar7 << 0x10);
    while( true ) {
      puVar3 = local_3a;
      pass1_1008_5b12(puVar3,param_4);
      uStack46 = CONCAT22(extraout_DX_03,puVar3);
      if ((extraout_DX_03 | (uint)puVar3) == 0x0) break;
      if ((*(int *)(puVar3 + 0x4) == 0x3e) || (*(int *)(puVar3 + 0x4) == 0x41)) {
        uStack30 = _PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        uVar9 = (uint)((ulong)paStack18 >> 0x10);
        uVar7 = (uint)paStack18;
        if ((uVar9 | uVar7) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          uStack26 = *(undefined2 *)((int)uStack46 + 0x4);
          paStack18->field_0x0 = 0x389a;
          *(undefined2 *)(uVar7 + 0x2) = 0x1008;
          *(undefined2 *)(uVar7 + 0x4) = uStack26;
          paStack18->field_0x0 = 0x9ec8;
          *(undefined2 *)(uVar7 + 0x2) = 0x1030;
          paStack6 = paStack18;
        }
        ppcVar1 = (code **)((int)*param_2 + 0x8);
        (**ppcVar1)(0x1000,uVar11,uVar10,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_951a(ushort param_1,ushort param_2,ulong param_3,ulong *param_4,ulong param_5)

{
  code **ppcVar1;
  uint uVar6;
  ushort *puVar7;
  ushort uVar8;
  uchar *puVar9;
  uint extraout_DX;
  uint uVar10;
  undefined2 extraout_DX_00;
  uint extraout_DX_01;
  int iVar11;
  undefined2 *puVar12;
  int unaff_DI;
  undefined2 uVar13;
  undefined2 uVar14;
  undefined uVar15;
  ulong *puVar16;
  ulong uVar17;
  undefined uVar18;
  undefined uVar19;
  undefined uVar20;
  undefined4 *puStack76;
  ulong uStack70;
  ulong uStack62;
  astruct_99 *paStack58;
  ushort local_36;
  uint uStack52;
  undefined4 uStack46;
  undefined2 uStack42;
  uint uStack40;
  int iStack38;
  ushort *puStack36;
  ushort *puStack32;
  int iStack28;
  int iStack20;
  ulong uStack18;
  ulong uStack14;
  ushort *puStack10;
  astruct_99 *paStack6;
  astruct_122 *uVar2;
  astruct_123 *uVar3;
  astruct_124 *uVar4;
  astruct_125 *uVar5;
  
  puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,param_1,(uchar *)param_2,unaff_DI);
  puVar9 = (uchar *)((ulong)puStack10 >> 0x10);
  uVar6 = (int)puStack10 + 0xa;
  uStack14 = (ulong)puStack10 & 0xffff0000 | (ulong)uVar6;
  pass1_1030_9048(param_1,param_3,0x0,param_5);
  uVar13 = (undefined2)((ulong)param_4 >> 0x10);
  uVar20 = SUB41(param_4,0x0);
  if (uVar6 != 0x0) {
    iStack28 = 0x0;
    puStack32 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_1,puVar9,unaff_DI);
    uVar14 = (undefined2)((ulong)puStack32 >> 0x10);
    puStack36 = *(ushort **)((int)puStack32 + 0xe);
    uVar6 = *(uint *)((int)puStack32 + 0x10);
    if ((uVar6 | (uint)puStack36) != 0x0) {
      pass1_1008_5784((ulong *)CONCAT22(param_1,&local_36),(ulong)puStack36 & 0xffff | (ulong)uVar6 << 0x10);
      while( true ) {
        puVar7 = &local_36;
        pass1_1008_5b12(puVar7,param_1);
        uStack46 = CONCAT22(extraout_DX,puVar7);
        if ((extraout_DX | (uint)puVar7) == 0x0) break;
        if ((puVar7[0x2] != 0x3e) && (puVar7[0x2] != 0x41)) {
          paStack6 = pass1_1000_07fc(0x1000,(ulong)_PTR_LOOP_1050_5768);
          uVar10 = (uint)((ulong)paStack6 >> 0x10);
          uVar6 = (uint)paStack6;
          if ((uVar10 | uVar6) == 0x0) {
            paStack6 = (astruct_99 *)0x0;
          }
          else {
            uVar14 = *(undefined2 *)((int)uStack46 + 0x4);
            paStack6->field_0x0 = 0x389a;
            *(undefined2 *)(uVar6 + 0x2) = 0x1008;
            *(undefined2 *)(uVar6 + 0x4) = uVar14;
            paStack6->field_0x0 = 0x9ec8;
            *(undefined2 *)(uVar6 + 0x2) = 0x1030;
          }
          ppcVar1 = (code **)((int)*param_4 + 0x8);
          (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
          if (*(int *)((int)uStack46 + 0x4) == 0x13) {
            iStack28 = 0x1;
          }
        }
      }
    }
    for (iStack38 = 0xa; iStack38 < 0x41; iStack38 = iStack38 + 0x1) {
      if ((((((iStack38 != 0x37) && (iStack38 != 0x35)) && (iStack38 != 0x36)) &&
           ((iStack38 != 0x25 && (iStack38 != 0x26)))) &&
          ((iStack38 != 0x27 && ((*(int *)(iStack38 * 0x2 + (int)uStack14) != 0x0 && (iStack38 != 0x13)))))) &&
         ((iStack38 != 0x14 || (iStack28 == 0x0)))) {
        paStack6 = pass1_1000_07fc(0x1000,(ulong)_PTR_LOOP_1050_5768);
        uVar10 = (uint)((ulong)paStack6 >> 0x10);
        uVar6 = (uint)paStack6;
        if ((uVar10 | uVar6) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          paStack6->field_0x0 = 0x389a;
          *(undefined2 *)(uVar6 + 0x2) = 0x1008;
          *(int *)(uVar6 + 0x4) = iStack38;
          paStack6->field_0x0 = 0x9ec8;
          *(undefined2 *)(uVar6 + 0x2) = 0x1030;
        }
        ppcVar1 = (code **)((int)*param_4 + 0x8);
        (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
      }
    }
  }
  uVar14 = (undefined2)(uStack14 >> 0x10);
  if (*(int *)((int)uStack14 + 0x6a) == 0x0) {
    if (*(int *)((int)uStack14 + 0x6c) != 0x0) {
      paStack58 = pass1_1000_07fc(0x1000,(ulong)_PTR_LOOP_1050_5768);
      uVar6 = (uint)((ulong)paStack58 >> 0x10);
      puVar12 = (undefined2 *)paStack58;
      if ((uVar6 | (uint)puVar12) == 0x0) goto LAB_1030_973e;
      paStack58->field_0x0 = 0x389a;
      puVar12[0x1] = 0x1008;
      puVar12[0x2] = 0x36;
      goto LAB_1030_9728;
    }
  }
  else {
    paStack58 = pass1_1000_07fc(0x1000,(ulong)_PTR_LOOP_1050_5768);
    uVar6 = (uint)((ulong)paStack58 >> 0x10);
    puVar12 = (undefined2 *)paStack58;
    if ((uVar6 | (uint)puVar12) == 0x0) {
LAB_1030_973e:
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      paStack58->field_0x0 = 0x389a;
      puVar12[0x1] = 0x1008;
      puVar12[0x2] = 0x35;
LAB_1030_9728:
      *puVar12 = 0x9ec8;
      puVar12[0x1] = 0x1030;
      paStack6 = paStack58;
    }
    ppcVar1 = (code **)((int)*param_4 + 0x8);
    (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
  }
  uVar14 = (undefined2)(uStack14 >> 0x10);
  iVar11 = (int)uStack14;
  if (*(int *)(iVar11 + 0x4a) == 0x0) {
    if (*(int *)(iVar11 + 0x4c) == 0x0) {
      if (*(int *)(iVar11 + 0x4e) == 0x0) goto LAB_1030_97e8;
      paStack58 = pass1_1000_07fc(0x1000,(ulong)_PTR_LOOP_1050_5768);
      uVar6 = (uint)((ulong)paStack58 >> 0x10);
      puVar12 = (undefined2 *)paStack58;
      if ((uVar6 | (uint)puVar12) != 0x0) {
        paStack58->field_0x0 = 0x389a;
        puVar12[0x1] = 0x1008;
        puVar12[0x2] = 0x27;
        goto LAB_1030_9879;
      }
    }
    else {
      paStack58 = pass1_1000_07fc(0x1000,(ulong)_PTR_LOOP_1050_5768);
      uVar6 = (uint)((ulong)paStack58 >> 0x10);
      puVar12 = (undefined2 *)paStack58;
      if ((uVar6 | (uint)puVar12) != 0x0) {
        paStack58->field_0x0 = 0x389a;
        puVar12[0x1] = 0x1008;
        puVar12[0x2] = 0x26;
        goto LAB_1030_9879;
      }
    }
LAB_1030_97d0:
    paStack6 = (astruct_99 *)0x0;
  }
  else {
    paStack58 = pass1_1000_07fc(0x1000,(ulong)_PTR_LOOP_1050_5768);
    uVar6 = (uint)((ulong)paStack58 >> 0x10);
    puVar12 = (undefined2 *)paStack58;
    if ((uVar6 | (uint)puVar12) == 0x0) goto LAB_1030_97d0;
    paStack58->field_0x0 = 0x389a;
    puVar12[0x1] = 0x1008;
    puVar12[0x2] = 0x25;
LAB_1030_9879:
    *puVar12 = 0x9ec8;
    puVar12[0x1] = 0x1030;
    paStack6 = paStack58;
  }
  ppcVar1 = (code **)((int)*param_4 + 0x8);
  (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
LAB_1030_97e8:
  uStack18 = (ulong)puStack10 & 0xffff0000 | (ulong)((int)puStack10 + 0x11e);
  if (*(int *)((int)puStack10 + 0x138) != 0x0) {
    puVar16 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
    puVar9 = (uchar *)((ulong)puVar16 >> 0x10);
    uVar6 = (uint)puVar16;
    uVar15 = 0x38;
    pass1_1038_4d6e(param_5,puVar16,uVar6,puVar9);
    puStack76 = (undefined4 *)CONCAT22(puVar9,uVar6);
    ppcVar1 = (code **)((int)*puStack76 + 0x10);
    uVar10 = uVar6;
    (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar6,puVar9);
    uStack70 = CONCAT22(extraout_DX_00,uVar10);
    for (uStack62 = 0x0; uStack62 < uStack70; uStack62 = uStack62 + 0x1) {
      ppcVar1 = (code **)((int)*puStack76 + 0x4);
      uVar17 = uStack70;
      (**ppcVar1)(uVar15,(char)uVar6,puVar9,(int)uStack62,(int)(uStack62 >> 0x10));
      uVar8 = (ushort)uVar17;
      iVar11 = 0xd;
      uVar10 = extraout_DX_01;
      local_36 = uVar8;
      uStack52 = extraout_DX_01;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar8,extraout_DX_01);
      uStack46 = CONCAT22(uVar10,uVar8);
      uVar17 = struct_op_1030_73a8(CONCAT22(uVar10,uVar8));
      uVar10 = (uint)(uVar17 >> 0x10);
      uStack42 = (undefined2)uVar17;
      uVar15 = 0x28;
      uStack40 = uVar10;
      uVar8 = pass1_1028_6744(param_1,uVar17,iVar11);
      if ((uVar10 | uVar8) != 0x0) {
        puStack32 = _PTR_LOOP_1050_5768;
        paStack6 = pass1_1000_07fc(0x1000,(ulong)_PTR_LOOP_1050_5768);
        uVar10 = (uint)((ulong)paStack6 >> 0x10);
        uVar5 = (astruct_125 *)paStack6;
        if ((uVar10 | (uint)uVar5) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          paStack6->field_0x0 = 0x389a;
          uVar5->field_0x2 = 0x1008;
          uVar5->field_0x4 = 0x4c;
          paStack6->field_0x0 = 0x9ec8;
          uVar5->field_0x2 = 0x1030;
        }
        ppcVar1 = (code **)((int)*param_4 + 0x8);
        (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
        puStack36 = _PTR_LOOP_1050_5768;
        paStack6 = pass1_1000_07fc(0x1000,(ulong)_PTR_LOOP_1050_5768);
        uVar10 = (uint)((ulong)paStack6 >> 0x10);
        uVar4 = (astruct_124 *)paStack6;
        if ((uVar10 | (uint)uVar4) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          paStack6->field_0x0 = 0x389a;
          uVar4->field_0x2 = 0x1008;
          uVar4->field_0x4 = 0x4d;
          paStack6->field_0x0 = 0x9ec8;
          uVar4->field_0x2 = 0x1030;
        }
        uVar18 = SUB41(paStack6,0x0);
        uVar19 = (undefined)((ulong)paStack6 >> 0x10);
        ppcVar1 = (code **)((int)*param_4 + 0x8);
        puVar16 = param_4;
        (**ppcVar1)();
        puStack36 = _PTR_LOOP_1050_5768;
        uVar15 = 0x0;
        paStack6 = pass1_1000_07fc(0x1000,(ulong)_PTR_LOOP_1050_5768);
        uVar10 = (uint)((ulong)paStack6 >> 0x10);
        uVar3 = (astruct_123 *)paStack6;
        if ((uVar10 | (uint)uVar3) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          paStack6->field_0x0 = 0x389a;
          uVar3->field_0x2 = 0x1008;
          uVar3->field_0x4 = 0x4e;
          paStack6->field_0x0 = 0x9ec8;
          uVar3->field_0x2 = 0x1030;
        }
        ppcVar1 = (code **)((int)*param_4 + 0x8);
        (**ppcVar1)(0x1000,param_4,paStack6,puVar16,uVar18,uVar19);
        break;
      }
    }
    if (puStack76 != (undefined4 *)0x0) {
      ppcVar1 = (code **)*puStack76;
      (**ppcVar1)(uVar15,uVar6,puVar9,0x1);
    }
  }
  for (iStack20 = 0x7a; iStack20 < 0x7d; iStack20 = iStack20 + 0x1) {
    if (*(int *)(iStack20 * 0x2 + (int)uStack14) != 0x0) {
      paStack6 = pass1_1000_07fc(0x1000,(ulong)_PTR_LOOP_1050_5768);
      uVar6 = (uint)((ulong)paStack6 >> 0x10);
      uVar2 = (astruct_122 *)paStack6;
      if ((uVar6 | (uint)uVar2) == 0x0) {
        paStack6 = (astruct_99 *)0x0;
      }
      else {
        paStack6->field_0x0 = 0x389a;
        uVar2->field_0x2 = 0x1008;
        uVar2->field_0x4 = iStack20;
        paStack6->field_0x0 = 0x9ec8;
        uVar2->field_0x2 = 0x1030;
      }
      ppcVar1 = (code **)((int)*param_4 + 0x8);
      (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
    }
  }
  return;
}
