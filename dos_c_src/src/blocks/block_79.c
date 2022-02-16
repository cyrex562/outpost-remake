


void __stdcall16far pass1_1030_3006(ulong param_1,ulong param_2)

{
  *(ulong *)((int)param_1 + 0x10) = param_2;
  return;
}



void __stdcall16far pass1_1030_301a(ulong param_1,char *param_2,ushort param_3)

{
  undefined4 uVar1;
  ushort uVar2;
  int iVar4;
  astruct_608 *iVar3;
  undefined2 uVar5;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x10) != 0x0) {
    uVar1 = *(undefined4 *)(iVar4 + 0x10);
    fn_ptr_1000_17ce(*(astruct_18 **)((int)uVar1 + 0x2),0x1000);
    uVar2 = str_op_1008_60e8(param_2,param_3);
    uVar1 = *(undefined4 *)(iVar4 + 0x10);
    uVar5 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar3 = (astruct_608 *)uVar1;
    iVar3->field_0x2 = uVar2;
    iVar3->field_0x4 = param_3;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1030_3058(ulong param_1,uint param_2,uchar *param_3)

{
  uint *puVar1;
  code **ppcVar2;
  uchar *puVar3;
  uchar *extraout_DX;
  astruct_375 *iVar4;
  undefined2 uVar4;
  undefined4 uVar5;
  ulong uVar6;
  uint uStack4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_375 *)param_1;
  puVar3 = param_3;
  if (iVar4->field_0xc == (undefined4 *)0x0) {
    mem_op_1000_179c(0x18,param_3,0x1000);
    puVar3 = (uchar *)((uint)param_3 | param_2);
    if (puVar3 == (uchar *)0x0) {
      iVar4->field_0xc = (undefined4 *)0x0;
    }
    else {
      uVar5 = struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_3,param_2),0x5,0x5);
      puVar3 = (uchar *)((ulong)uVar5 >> 0x10);
      *(int *)&iVar4->field_0xc = (int)uVar5;
      *(uchar **)((int)&iVar4->field_0xc + 0x2) = puVar3;
    }
  }
  for (uStack4 = 0x0; uVar5 = iVar4->field_0x10, puVar1 = (uint *)((int)uVar5 + 0x22),
      uStack4 <= *puVar1 && *puVar1 != uStack4; uStack4 = uStack4 + 0x1) {
    uVar6 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(ushort)puVar3,0x3);
    ppcVar2 = (code **)((int)*iVar4->field_0xc + 0x8);
    (**ppcVar2)((int)&USHORT_1050_1028,iVar4->field_0xc,(int)uVar6,(int)(uVar6 >> 0x10),uStack4,0x0);
    puVar3 = extraout_DX;
  }
  return 0x1;
}



astruct_18 * __stdcall16far pass1_1030_310a(astruct_18 *param_1,byte param_2)

{
  pass1_1030_29e6(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_314c(ushort *param_1,ulong param_2,uchar *param_3,ushort param_4)

{
  astruct_364 *iVar1;
  int unaff_DI;
  undefined2 uVar1;
  int iStack12;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_364 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x170 = 0x0;
  iVar1->field_0x1a4 = param_2;
  iVar1->field_0x1a8 = 0x5;
  iVar1->field_0x1aa = 0x0;
  iVar1->field_0x1ae = 0x10;
  *param_1 = 0x3af2;
  iVar1->field_0x2 = 0x1030;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x4),(WNDCLASS16 *)0x0,0x16c);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x18c),(WNDCLASS16 *)0x0,0x18)
  ;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x174),(WNDCLASS16 *)0x0,0xc);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x180),(WNDCLASS16 *)0x0,0xc);
  mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_3,unaff_DI);
  if ((int)PTR_LOOP_1050_13ae < 0x2) {
    pass1_1030_34da((ulong)param_1);
  }
  else {
    iVar1->field_0x176 = 0x1;
    iVar1->field_0x178 = 0x2;
    iVar1->field_0x17a = 0x2;
    iVar1->field_0x17c = 0x60001;
    for (iStack12 = 0x1; iStack12 < 0x6; iStack12 = iStack12 + 0x1) {
      *(undefined2 *)(&iVar1->field_0x180 + iStack12 * 0x2) = 0x64;
    }
  }
  return;
}



void __stdcall16far pass1_1030_3258(ulong param_1,ushort param_2)

{
  *(ushort *)((int)param_1 + 0x1ae) = param_2;
  return;
}



void __stdcall16far pass1_1030_326a(ulong param_1,ulong param_2,uint param_3,ushort param_4)

{
  uint uVar1;
  ulong uVar2;
  uint uVar3;
  astruct_692 *iVar4;
  uint uVar4;
  long lStack6;
  
  uVar4 = (uint)(param_1 >> 0x10);
  iVar4 = (astruct_692 *)param_1;
  if (iVar4->field_0x1aa == 0x0) {
    iVar4->field_0x1aa = 0x1;
  }
  else {
    param_2 = iVar4->field_0x1aa * 0x2;
    iVar4->field_0x1aa = param_2;
  }
  uVar1 = (uint)param_2;
  pass1_1030_38b8();
  lStack6 = CONCAT22(param_3,uVar1);
  uVar2 = iVar4->field_0x1aa;
  uVar3 = *(uint *)((int)&iVar4->field_0x1aa + 0x2);
  if (lStack6 < (long)uVar2) {
    uVar2 = (ulong)uVar1;
    uVar3 = param_3;
  }
  *(int *)&iVar4->field_0x1aa = (int)uVar2;
  *(uint *)((int)&iVar4->field_0x1aa + 0x2) = uVar3;
  pass1_1030_375a(param_1 & 0xffff | (ulong)uVar4 << 0x10,0x0,uVar2 & 0xffff | (ulong)uVar3 << 0x10,param_4);
  return;
}



void __stdcall16far write_to_file_1030_32e4(ulong param_1,ulong param_2,ushort param_3)

{
  ushort uVar1;
  int iVar2;
  BOOL16 BVar3;
  ushort uVar4;
  ushort uVar5;
  undefined4 local_16 [0x2];
  undefined2 local_c;
  undefined4 local_a [0x2];
  
  iVar2 = (int)param_1;
  uVar1 = (ushort)(param_1 >> 0x10);
  uVar4 = (ushort)param_2;
  uVar5 = (ushort)(param_2 >> 0x10);
  BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x4,uVar1,(char *)0x16c,0x1008);
  if (BVar3 != 0x0) {
    BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x174,uVar1,&DAT_0000_000c,0x1008);
    if (BVar3 != 0x0) {
      BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x180,uVar1,&DAT_0000_000c,0x1008);
      if (BVar3 != 0x0) {
        BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x18c,uVar1,(char *)0x18,0x1008);
        if (BVar3 != 0x0) {
          local_c = *(undefined2 *)(iVar2 + 0x1a8);
          BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)&local_c,param_3,(char *)0x2,0x1008);
          if (BVar3 != 0x0) {
            local_16[0] = *(undefined4 *)(iVar2 + 0x1aa);
            BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)local_16,param_3,(char *)0x4,0x1008);
            if (BVar3 != 0x0) {
              local_a[0] = *(undefined4 *)(iVar2 + 0x170);
              BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)local_a,param_3,(char *)0x4,0x1008);
              if (BVar3 != 0x0) {
                local_c = *(undefined2 *)(iVar2 + 0x1ae);
                BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)&local_c,param_3,(char *)0x2,0x1008);
                if (BVar3 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  return;
}



void __stdcall16far read_file_1030_33f0(ulong param_1,ulong param_2)

{
  ushort uVar1;
  astruct_430 *iVar2;
  BOOL16 BVar2;
  ushort uVar3;
  ushort uVar4;
  
  iVar2 = (astruct_430 *)param_1;
  iVar2 = (astruct_430 *)&iVar2->field_0x4;
  uVar1 = (ushort)(param_1 >> 0x10);
  uVar3 = (ushort)param_2;
  uVar4 = (ushort)(param_2 >> 0x10);
  BVar2 = read_file_1008_7dee(uVar3,uVar4,(ushort)iVar2,0x0,uVar1,0x16c,0x1008);
  if (((((BVar2 != 0x0) &&
        (BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x174,0x0,uVar1,0xc,0x1008), BVar2 != 0x0)) &&
       (BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x180,0x0,uVar1,0xc,0x1008), BVar2 != 0x0)) &&
      ((BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x18c,0x0,uVar1,0x18,0x1008), BVar2 != 0x0 &&
       (BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x1a8,0x0,uVar1,0x2,0x1008), BVar2 != 0x0)))) &&
     (BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x1aa,0x0,uVar1,0x4,0x1008), BVar2 != 0x0)) {
    if ((int)PTR_LOOP_1050_0312 < 0x2) {
      return;
    }
    BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x170,0x0,uVar1,0x4,0x1008);
    if ((BVar2 != 0x0) &&
       (BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x1ae,0x0,uVar1,0x2,0x1008), BVar2 != 0x0)) {
      return;
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  return;
}



void __stdcall16far pass1_1030_34da(ulong param_1)

{
  astruct_682 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  iVar1 = (astruct_682 *)param_1;
  iVar1->field_0x176 = 0x1;
  iVar1->field_0x178 = 0x1;
  iVar1->field_0x17a = 0x1;
  iVar1->field_0x17c = 0x1;
  iVar1->field_0x17e = 0x4;
  iVar1->field_0x182 = 0x32;
  iVar1->field_0x184 = 0xa;
  iVar1->field_0x186 = 0xa;
  iVar1->field_0x188 = 0xa;
  iVar1->field_0x18a = 0x4b;
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | ZEXT24(iVar1 + 0x1)),(WNDCLASS16 *)0x0,0x18);
  return;
}



void __stdcall16far pass1_1030_3534(ulong param_1,ulong param_2)

{
  *(ulong *)((int)param_1 + 0x4) = param_2;
  return;
}



void __stdcall16far pass1_1030_3548(ulong param_1,long param_2)

{
  long *plVar1;
  
  plVar1 = (long *)((int)param_1 + 0x4);
  *plVar1 = *plVar1 + param_2;
  return;
}



void __stdcall16far pass1_1030_355c(ulong param_1,ulong param_2)

{
  int iVar1;
  undefined2 uVar2;
  int iStack4;
  
  iStack4 = 0x0;
  do {
    iVar1 = iStack4 * 0x4;
    uVar2 = (undefined2)(param_1 >> 0x10);
    *(long *)((int)param_1 + iVar1 + 0x4) = *(long *)(iVar1 + (int)param_2) + *(long *)((int)param_1 + 0x4 + iVar1);
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x5b);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_35a4(ulong param_1,long param_2,uchar *param_3,ushort param_4,ushort param_5)

{
  uint *puVar1;
  uchar **ppuVar2;
  uint uVar3;
  uchar *puVar4;
  ushort uVar5;
  uint uVar6;
  ulong uVar7;
  uchar *puVar8;
  undefined2 uVar9;
  undefined uVar10;
  undefined uVar11;
  undefined local_c [0x2];
  undefined4 local_a;
  undefined4 uStack6;
  
  vsprintf_op_1030_840a((ulong)s_Pop_Leaving__ld_1050_516a,param_4,param_5,(ushort)param_3);
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_3,0x1000);
    PTR_LOOP_1050_5f2e = param_3;
  }
  else {
  }
  uVar5 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  uStack6 = CONCAT22(PTR_LOOP_1050_5f2e,uVar5);
  uVar10 = (undefined)param_5;
  uVar11 = (undefined)(param_5 >> 0x8);
  pass1_1030_3948(param_1,(ushort *)CONCAT22(param_5,local_c),(int *)CONCAT13(uVar11,CONCAT12(uVar10,&local_a)),0x3);
  uVar7 = (ulong)((int)&local_a + 0x2U);
  pass1_1030_3948(param_1,(ushort *)CONCAT22(param_5,(int)&local_a + 0x2U),
                  (int *)CONCAT13(uVar11,CONCAT12(uVar10,local_c)),0x4);
  do {
    uVar6 = (uint)uVar7;
    if (param_2 < 0x1) break;
    pass1_1008_612e((int)local_a,(int)((ulong)local_a >> 0x10),uVar6);
    uVar7 = ZEXT24(&param_2);
    pass1_1030_3a3a(param_1,(long *)CONCAT13(uVar11,CONCAT12(uVar10,&param_2)),uVar6);
    uVar9 = (undefined2)((ulong)uStack6 >> 0x10);
    puVar1 = (uint *)(uVar6 * 0x4 + (int)uStack6);
    uVar3 = *puVar1;
    *puVar1 = *puVar1 + (uint)uVar7;
    ppuVar2 = (uchar **)(uVar6 * 0x4 + (int)uStack6 + 0x2);
    *ppuVar2 = PTR_LOOP_1050_5f2e + (int)(*ppuVar2 + CARRY2(uVar3,(uint)uVar7));
    pass1_1030_38f2(param_1,0x3,param_5);
    uVar6 = (uint)uVar7;
    puVar8 = PTR_LOOP_1050_5f2e;
    pass1_1030_38f2(param_1,0x4,param_5);
    puVar4 = PTR_LOOP_1050_5f2e + (int)puVar8;
    PTR_LOOP_1050_5f2e = puVar8;
  } while (((uint)(puVar4 + CARRY2(uVar6,(uint)uVar7)) | uVar6 + (uint)uVar7) != 0x0);
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x18c)),(WNDCLASS16 *)0x0,0x18);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_3694(ulong param_1,int param_2,long param_3,uchar *param_4,ushort param_5,ushort param_6)

{
  uint *puVar1;
  uchar **ppuVar2;
  uint uVar3;
  ushort uVar4;
  uint uVar5;
  ulong uVar6;
  uchar *puVar7;
  
  vsprintf_op_1030_840a((ulong)s_Pop_Leaving__ld_1050_517a,param_5,param_6,(ushort)param_4);
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_4,0x1000);
    PTR_LOOP_1050_5f2e = param_4;
  }
  else {
  }
  uVar4 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  uVar6 = (ulong)(param_2 - 0x1U);
  puVar7 = PTR_LOOP_1050_5f2e;
  if (((param_2 < 0x1) || (SBORROW2(param_2,0x1))) ||
     (uVar6 = (ulong)(param_2 - 0x5U), param_2 - 0x5U != 0x0 && 0x3 < (int)(param_2 - 0x1U))) {
    while (uVar5 = (uint)uVar6, 0x0 < param_3) {
      pass1_1008_612e(0x0,0x5a,uVar5);
      uVar6 = ZEXT24(&param_3);
      pass1_1030_3a3a(param_1,(long *)CONCAT13((char)(param_6 >> 0x8),CONCAT12((char)param_6,&param_3)),uVar5);
      puVar1 = (uint *)(uVar5 * 0x4 + uVar4);
      uVar3 = *puVar1;
      *puVar1 = *puVar1 + (uint)uVar6;
      ppuVar2 = (uchar **)(uVar5 * 0x4 + uVar4 + 0x2);
      *ppuVar2 = puVar7 + (int)(*ppuVar2 + CARRY2(uVar3,(uint)uVar6));
    }
  }
  else {
    pass1_1030_39dc(param_1,(long *)CONCAT22(param_6,&param_3),
                    CONCAT13((char)((uint)PTR_LOOP_1050_5f2e >> 0x8),CONCAT12((char)PTR_LOOP_1050_5f2e,uVar4)),param_2);
  }
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x18c)),(WNDCLASS16 *)0x0,0x18);
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1030_375a(ulong param_1,int param_2,long param_3,ushort param_4)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  long lVar4;
  long lVar5;
  int iVar6;
  int iVar7;
  undefined2 uVar8;
  int iStack20;
  undefined4 uStack18;
  int local_6;
  int local_4;
  
  iVar6 = (int)param_1;
  if (param_2 == 0x0) {
    local_4 = 0x5a;
    while ((-0x1 < local_4 && (pass1_1030_3a3a(param_1,(long *)CONCAT22(param_4,&param_3),local_4), param_3 != 0x0))) {
      local_4 = local_4 + -0x1;
    }
  }
  else {
    pass1_1030_3948(param_1,(ushort *)CONCAT22(param_4,&local_4),(int *)CONCAT22(param_4,&local_6),param_2);
    iVar2 = (local_4 - local_6) + 0x1;
    lVar4 = param_3 / (long)iVar2;
    lVar5 = lVar4 * iVar2;
    uVar3 = (uint)lVar5;
    uStack18 = CONCAT22(((int)((ulong)param_3 >> 0x10) - (int)((ulong)lVar5 >> 0x10)) - (uint)((uint)param_3 < uVar3),
                        (uint)param_3 - uVar3);
    for (iStack20 = local_6; iStack20 <= local_4; iStack20 = iStack20 + 0x1) {
      iVar7 = iStack20 * 0x4;
      uVar8 = (undefined2)(param_1 >> 0x10);
      *(long *)(iVar6 + iVar7 + 0x4) = *(long *)(iVar6 + iVar7 + 0x4) - lVar4;
      iVar2 = *(int *)(iVar6 + iVar7 + 0x6);
      if ((uStack18._2_2_ | (uint)uStack18) != 0x0) {
        iVar1 = *(int *)(iVar6 + iVar7 + 0x4);
        *(int *)(iVar6 + iVar7 + 0x4) = iVar1 + -0x1;
        *(int *)(iVar6 + iVar7 + 0x6) = iVar2 - (uint)(iVar1 == 0x0);
        uStack18 = uStack18 + -0x1;
      }
      if (*(int *)(iVar6 + iStack20 * 0x4 + 0x6) < 0x0) {
        *(undefined4 *)(iVar6 + iStack20 * 0x4 + 0x4) = 0x0;
      }
    }
  }
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x18c)),(WNDCLASS16 *)0x0,0x18);
  return;
}



void __stdcall16far pass1_1030_387c(ulong param_1)

{
  int iStack4;
  
  iStack4 = 0x5a;
  do {
    *(undefined4 *)(iStack4 * 0x4 + (int)param_1 + 0x4) = *(undefined4 *)(iStack4 * 0x4 + (int)param_1);
    iStack4 = iStack4 + -0x1;
  } while (0x0 < iStack4);
  *(undefined4 *)((int)param_1 + 0x4) = 0x0;
  return;
}



void __stdcall16far pass1_1030_38b8(void)

{
  int iStack8;
  
  iStack8 = 0x0;
  do {
    iStack8 = iStack8 + 0x1;
  } while (iStack8 < 0x5b);
  return;
}



void __stdcall16far pass1_1030_38f2(ulong param_1,int param_2,ushort param_3)

{
  int iStack12;
  int local_a;
  int local_8;
  undefined4 uStack6;
  
  uStack6 = 0x0;
  pass1_1030_3948(param_1,(ushort *)CONCAT22(param_3,&local_a),(int *)CONCAT22(param_3,&local_8),param_2);
  for (iStack12 = local_8; iStack12 <= local_a; iStack12 = iStack12 + 0x1) {
  }
  return;
}



void __stdcall16far pass1_1030_3948(ulong param_1,ushort *param_2,int *param_3,int param_4)

{
  undefined2 uVar1;
  
  if (param_4 == 0x1) {
    *param_3 = 0x0;
    *param_2 = 0x3;
    return;
  }
  uVar1 = (undefined2)(param_1 >> 0x10);
  if (param_4 == 0x2) {
    *param_3 = 0x4;
    *param_2 = *(ushort *)((int)param_1 + 0x1ae);
    return;
  }
  if (param_4 == 0x3) {
    *param_3 = *(int *)((int)param_1 + 0x1ae) + 0x1;
    *param_2 = 0x27;
    return;
  }
  if (param_4 != 0x4) {
    if (param_4 == 0x5) {
      *param_3 = 0x4c;
    }
    else {
      *param_3 = 0x0;
    }
    *param_2 = 0x5a;
    return;
  }
  *param_3 = 0x28;
  *param_2 = 0x4b;
  return;
}



void __stdcall16far pass1_1030_39dc(ulong param_1,long *param_2,ulong param_3,int param_4)

{
  int iVar1;
  undefined2 in_DX;
  undefined2 uVar2;
  undefined2 unaff_SS;
  int iStack8;
  int local_6;
  int local_4;
  
  pass1_1030_3948(param_1,(ushort *)CONCAT22(unaff_SS,&local_6),(int *)CONCAT22(unaff_SS,&local_4),param_4);
  iStack8 = local_6;
  while( true ) {
    if (iStack8 < local_4) {
      return;
    }
    iVar1 = local_4;
    pass1_1030_3a3a(param_1,param_2,iStack8);
    uVar2 = (undefined2)(param_3 >> 0x10);
    *(int *)(iStack8 * 0x4 + (int)param_3) = iVar1;
    *(undefined2 *)(iStack8 * 0x4 + (int)param_3 + 0x2) = in_DX;
    if (*param_2 == 0x0) break;
    iStack8 = iStack8 + -0x1;
  }
  return;
}



void __stdcall16far pass1_1030_3a3a(ulong param_1,long *param_2,int param_3)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  undefined2 uVar9;
  
  iVar2 = *(int *)((int)param_2 + 0x2);
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  iVar7 = iVar6 + 0x4;
  iVar8 = param_3 * 0x4;
  piVar1 = (int *)(iVar7 + iVar8 + 0x2);
  iVar3 = *piVar1;
  if ((iVar3 < iVar2) ||
     ((uVar5 = (uint)*param_2, *piVar1 == iVar2 || iVar3 < iVar2 && (*(uint *)(iVar7 + iVar8) < uVar5)))) {
    *param_2 = *param_2 - *(long *)(iVar6 + 0x4 + param_3 * 0x4);
    *(undefined4 *)(iVar6 + param_3 * 0x4 + 0x4) = 0x0;
  }
  else {
    uVar4 = *(uint *)(iVar7 + iVar8);
    iVar3 = *(int *)(iVar7 + iVar8 + 0x2);
    *(int *)(iVar6 + iVar8 + 0x4) = uVar4 - uVar5;
    *(int *)(iVar6 + iVar8 + 0x6) = (iVar3 - iVar2) - (uint)(uVar4 < uVar5);
    *param_2 = 0x0;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_3ac6(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ulong * __stdcall16far pass1_1030_3af6(ulong *param_1,ushort param_2,ushort param_3,ulong *param_4,ushort param_5)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = *param_4;
  *(undefined2 *)(iVar1 + 0x4) = *(undefined2 *)(param_4 + 0x1);
  *(ushort *)(iVar1 + 0x6) = param_3;
  *(ushort *)(iVar1 + 0x8) = param_2;
  return param_1;
}



ushort __cdecl16far pass1_1030_3b28(undefined2 param_1)

{
  ushort *puVar1;
  ulong *puVar2;
  undefined local_8 [0x6];
  
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffc4,0x0);
  pass1_1030_3af6((ulong *)&USHORT_1050_65e6,0x115,0x15b,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0x19);
  pass1_1030_3af6((ulong *)&USHORT_1050_65f0,0x116,0x15c,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffdd,0x32);
  pass1_1030_3af6((ulong *)&USHORT_1050_65fa,0x117,0x15d,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0x4b);
  pass1_1030_3af6((ulong *)&USHORT_1050_6604,0x118,0x15e,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xf,0x64);
  pass1_1030_3af6((ulong *)&USHORT_1050_660e,0x119,0x15f,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x28,0x7d);
  pass1_1030_3af6((ulong *)&USHORT_1050_6618,0x11a,0x160,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffec,0x96);
  pass1_1030_3af6((ulong *)&USHORT_1050_6622,0x11b,0x161,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x14,0xaf);
  pass1_1030_3af6((ulong *)&USHORT_1050_662c,0x11c,0x162,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x1e,0xc8);
  pass1_1030_3af6((ulong *)&USHORT_1050_6636,0x11d,0x163,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xfffb,0xe1);
  pass1_1030_3af6((ulong *)&USHORT_1050_6640,0x11e,0x164,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x32,0xfa);
  pass1_1030_3af6((ulong *)&USHORT_1050_664a,0x11f,0x165,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x1e,0xe1);
  pass1_1030_3af6((ulong *)&USHORT_1050_6654,0x120,0x166,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffe7,0xfa);
  pass1_1030_3af6((ulong *)&USHORT_1050_665e,0x121,0x167,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0x113);
  pass1_1030_3af6((ulong *)&USHORT_1050_6668,0x122,0x168,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x28,0x12c);
  pass1_1030_3af6((ulong *)&USHORT_1050_6672,0x123,0x169,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xf,0x145);
  pass1_1030_3af6((ulong *)&USHORT_1050_667c,0x124,0x16a,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffec,0x15e);
  pass1_1030_3af6((ulong *)&USHORT_1050_6686,0x125,0x16b,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0x0);
  pass1_1030_3af6((ulong *)&USHORT_1050_6690,0x126,0x16c,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x2d,0x19);
  pass1_1030_3af6((ulong *)&USHORT_1050_669a,0x127,0x16d,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xa,0x32);
  pass1_1030_3af6((ulong *)&USHORT_1050_66a4,0x128,0x16e,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffe2,0x4b);
  pass1_1030_3af6((ulong *)&USHORT_1050_66ae,0x129,0x16f,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x5,0x64);
  pass1_1030_3af6((ulong *)&USHORT_1050_66b8,0x12a,0x170,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x32,0x7d);
  pass1_1030_3af6((ulong *)&USHORT_1050_66c2,0x12b,0x171,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffc9,0x96);
  pass1_1030_3af6((ulong *)&USHORT_1050_66cc,0x12c,0x172,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xfffb,0xaf);
  pass1_1030_3af6((ulong *)&USHORT_1050_66d6,0x12d,0x173,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffe7,0xc8);
  pass1_1030_3af6((ulong *)&USHORT_1050_66e0,0x12e,0x174,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x32,0x32);
  pass1_1030_3af6((ulong *)&USHORT_1050_66ea,0x12f,0x175,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x3c,0x64);
  pass1_1030_3af6((ulong *)&USHORT_1050_66f4,0x130,0x176,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffc4,0xe1);
  pass1_1030_3af6((ulong *)&USHORT_1050_66fe,0x131,0x177,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0x19);
  pass1_1030_3af6((ulong *)&USHORT_1050_6708,0x132,0x178,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x5,0xaf);
  pass1_1030_3af6((ulong *)&USHORT_1050_6712,0x133,0x179,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0x19);
  pass1_1030_3af6((ulong *)&USHORT_1050_671c,0x134,0x17a,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x23,0x19);
  pass1_1030_3af6((ulong *)&USHORT_1050_6726,0x135,0x17b,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xfffb,0x32);
  pass1_1030_3af6((ulong *)&USHORT_1050_6730,0x136,0x17c,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xf,0x32);
  pass1_1030_3af6((ulong *)&USHORT_1050_673a,0x137,0x17d,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x2d,0x4b);
  pass1_1030_3af6((ulong *)&USHORT_1050_6744,0x138,0x17e,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x1e,0x4b);
  pass1_1030_3af6((ulong *)&USHORT_1050_674e,0x139,0x17f,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x2d,0x64);
  pass1_1030_3af6((ulong *)&USHORT_1050_6758,0x13a,0x180,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffe7,0x7d);
  pass1_1030_3af6((ulong *)&USHORT_1050_6762,0x13b,0x181,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x5,0xaf);
  pass1_1030_3af6((ulong *)&USHORT_1050_676c,0x13c,0x182,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0xc8);
  pass1_1030_3af6((ulong *)&USHORT_1050_6776,0x13d,0x183,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffce,0xc8);
  pass1_1030_3af6((ulong *)&USHORT_1050_6780,0x13e,0x184,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xf,0xfa);
  pass1_1030_3af6((ulong *)&USHORT_1050_678a,0x13f,0x185,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x1e,0x113);
  pass1_1030_3af6((ulong *)&USHORT_1050_6794,0x140,0x186,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffe2,0x12c);
  pass1_1030_3af6((ulong *)&USHORT_1050_679e,0x141,0x187,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x64,0x12c);
  pass1_1030_3af6((ulong *)&USHORT_1050_67a8,0x142,0x188,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x32,0x145);
  pass1_1030_3af6((ulong *)&USHORT_1050_67b2,0x143,0x189,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x64,0x145);
  pass1_1030_3af6((ulong *)&USHORT_1050_67bc,0x144,0x18a,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x1e,0x15e);
  pass1_1030_3af6((ulong *)&USHORT_1050_67c6,0x145,0x18b,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffd3,0x15e);
  pass1_1030_3af6((ulong *)&USHORT_1050_67d0,0x146,0x18c,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x32,0xfa);
  pass1_1030_3af6((ulong *)&USHORT_1050_67da,0x147,0x18d,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xf,0x19);
  pass1_1030_3af6((ulong *)&USHORT_1050_67e4,0x148,0x18e,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0x32);
  pass1_1030_3af6((ulong *)&USHORT_1050_67ee,0x149,0x18f,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0xaf);
  pass1_1030_3af6((ulong *)&USHORT_1050_67f8,0x14a,0x190,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xfffb,0xe1);
  pass1_1030_3af6((ulong *)&USHORT_1050_6802,0x14b,0x191,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xa,0x15e);
  pass1_1030_3af6((ulong *)&USHORT_1050_680c,0x14c,0x192,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0x19);
  pass1_1030_3af6((ulong *)&USHORT_1050_6816,0x14d,0x193,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x1e,0x32);
  pass1_1030_3af6((ulong *)&USHORT_1050_6820,0x14e,0x194,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xfffb,0x64);
  pass1_1030_3af6((ulong *)&USHORT_1050_682a,0x14f,0x195,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xf,0x64);
  pass1_1030_3af6((ulong *)&USHORT_1050_6834,0x150,0x196,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x1e,0x7d);
  pass1_1030_3af6((ulong *)&USHORT_1050_683e,0x151,0x197,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffdd,0xe1);
  pass1_1030_3af6((ulong *)&USHORT_1050_6848,0x152,0x198,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0x113);
  pass1_1030_3af6((ulong *)&USHORT_1050_6852,0x153,0x199,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x2d,0x12c);
  pass1_1030_3af6((ulong *)&USHORT_1050_685c,0x154,0x19a,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffe7,0x145);
  pass1_1030_3af6((ulong *)&USHORT_1050_6866,0x155,0x19b,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xa,0x15e);
  pass1_1030_3af6((ulong *)&USHORT_1050_6870,0x156,0x19c,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0x4b);
  pass1_1030_3af6((ulong *)&USHORT_1050_687a,0x157,0x19d,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x5,0x64);
  pass1_1030_3af6((ulong *)&USHORT_1050_6884,0x158,0x19e,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0xffec,0x96);
  pass1_1030_3af6((ulong *)&USHORT_1050_688e,0x159,0x19f,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_8),0x0,0x0,0x113);
  puVar2 = pass1_1030_3af6((ulong *)&USHORT_1050_6898,0x15a,0x1a0,(ulong *)puVar1,(ushort)((ulong)puVar1 >> 0x10));
  return (ushort)puVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1030_44be(ulong *param_1,ushort param_2)

{
  astruct_138 *iVar1;
  int unaff_DI;
  undefined2 uVar1;
  ushort unaff_SS;
  ushort *puVar2;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_138 *)param_1;
  *param_1 = 0x0;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x152 = 0x0;
  iVar1->field_0x154 = 0x0;
  iVar1->field_0x156 = 0x0;
  iVar1->field_0x158 = 0x0;
  iVar1->field_0x15a = 0x0;
  iVar1->field_0x15c = 0x0;
  iVar1->field_0x160 = 0x0;
  iVar1->field_0x164 = 0x0;
  iVar1->field_0x568 = 0x0;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,unaff_SS,(uchar *)param_2,unaff_DI);
  iVar1->field_0x568 = *(undefined4 *)((int)puVar2 + 0x64);
  return;
}



void __stdcall16far pass1_1030_4538(ulong *param_1)

{
  undefined2 uVar1;
  
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0x12),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0x15c),0x1000);
  return;
}



ulong __stdcall16far struct_1030_4574(ulong param_1)

{
  astruct_159 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  iVar1 = (astruct_159 *)param_1;
  iVar1->field_0xc = DAT_1050_518c;
  iVar1->field_0xe = 0x518e;
  iVar1->field_0x10 = (int)&USHORT_1050_1050;
  return param_1 & 0xffff0000 | ZEXT24(&iVar1->field_0xc);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_4594(uchar *param_1,ushort param_2,ushort param_3,int param_4)

{
  undefined2 uVar1;
  undefined2 *puVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  ushort *puVar7;
  undefined2 *puStack8;
  
  puVar2 = (undefined2 *)(ulong)(param_4 - 0x1U);
  mem_op_1000_179c(0x10,param_1,0x1000);
  puStack8 = (undefined2 *)((ulong)puVar2 & 0xffff | ZEXT24(param_1) << 0x10);
  uVar3 = (uint)param_1 | (uint)puVar2;
  if (uVar3 == 0x0) {
    puStack8 = (undefined2 *)0x0;
  }
  else {
    puVar7 = pass1_1008_3e38((ushort *)CONCAT22(param_1,(uint)puVar2 + 0x4));
    uVar3 = (uint)((ulong)puVar7 >> 0x10);
    puVar2 = puStack8;
  }
  uVar1 = SUB42(puVar2,0x0);
  iVar4 = (param_4 - 0x1U) * 0x12;
  load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  uVar6 = (undefined2)((ulong)puStack8 >> 0x10);
  iVar5 = (int)puStack8;
  *puStack8 = uVar1;
  *(uint *)(iVar5 + 0x2) = uVar3;
  *(undefined2 *)(iVar5 + 0xa) = *(undefined2 *)(iVar4 + 0x51ba);
  pass1_1008_3e76((ushort *)((ulong)puStack8 & 0xffff0000 | (ulong)(iVar5 + 0x4)),*(ushort *)(iVar4 + 0x51c0),
                  *(ushort *)(iVar4 + 0x51be),*(ushort *)(iVar4 + 0x51bc));
  *(int *)(iVar5 + 0xc) = iVar4 + 0x51c2;
  *(undefined2 *)(iVar5 + 0xe) = (int)&USHORT_1050_1050;
  return;
}



void __stdcall16far pass1_1030_4628(uchar *param_1,ushort param_2,ushort param_3,int param_4)

{
  undefined4 uVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int *piVar7;
  undefined2 uVar8;
  int iStack24;
  int *piStack20;
  int iStack10;
  int *piStack8;
  
  uVar2 = param_4 - 0x1;
  uVar3 = uVar2;
  mem_op_1000_179c(0x28,param_1,0x1000);
  piStack20 = (int *)CONCAT22(param_1,uVar3);
  if (((uint)param_1 | uVar3) == 0x0) {
    piStack8 = (int *)0x0;
  }
  else {
    pass1_1008_3e38((ushort *)CONCAT22(param_1,uVar3 + 0x6));
    piStack8 = piStack20;
  }
  uVar8 = (undefined2)((ulong)piStack8 >> 0x10);
  iVar5 = (int)piStack8;
  *(undefined4 *)(iVar5 + 0x2) = 0x0;
  iVar6 = uVar2 * 0x5e;
  pass1_1008_3e76((ushort *)((ulong)piStack8 & 0xffff0000 | (ulong)(iVar5 + 0x6)),*(ushort *)(iVar6 + 0x5336),
                  *(ushort *)(iVar6 + 0x5334),*(ushort *)(iVar6 + 0x5332));
  *(undefined2 *)(iVar5 + 0xc) = *(undefined2 *)(iVar6 + 0x5348);
  *piStack8 = param_4;
  *(undefined4 *)(iVar5 + 0xe) = *(undefined4 *)(iVar6 + 0x534a);
  iStack10 = 0x0;
  do {
    uVar3 = *(uint *)((uVar2 * 0x2f + iStack10) * 0x2 + 0x5338);
    *(uint *)(iVar5 + iStack10 * 0x2 + 0x12) = uVar3;
    iStack10 = iStack10 + 0x1;
  } while (iStack10 < 0x8);
  uVar1 = *(undefined4 *)((int)&DAT_1050_5350 + uVar2 * 0x5e);
  pass1_1008_612e((int)uVar1,(int)((ulong)uVar1 >> 0x10),uVar3);
  *(uint *)(iVar5 + 0x22) = uVar3;
  piVar7 = (int *)(uVar2 * 0x5e + 0x5354);
  *(int **)(iVar5 + 0x24) = piVar7;
  *(undefined2 *)(iVar5 + 0x26) = (int)&USHORT_1050_1050;
  iVar6 = *piVar7;
  pass1_1000_4906((astruct_20 *)CONCAT22(0x1050,piVar7),(WNDCLASS16 *)0x0,0x1e);
  iStack10 = 0x0;
LAB_1030_474c:
  if ((int)uVar3 <= iStack10) {
    return;
  }
  do {
    iVar4 = *(int *)(uVar2 * 0x5e + 0x534e) + iVar6 + -0x1;
    pass1_1008_612e(iVar6,iVar4,iVar4);
    iStack24 = 0x0;
    while( true ) {
      if (iStack10 < iStack24) {
        uVar1 = *(undefined4 *)(iVar5 + 0x24);
        *(int *)((int)uVar1 + iStack10 * 0x2) = iVar4;
        iStack10 = iStack10 + 0x1;
        goto LAB_1030_474c;
      }
      uVar1 = *(undefined4 *)(iVar5 + 0x24);
      if (*(int *)((int)uVar1 + iStack24 * 0x2) == iVar4) break;
      iStack24 = iStack24 + 0x1;
    }
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1030_4782(uchar *param_1,uchar param_2,uchar *param_3,ushort param_4,ushort param_5,int param_6,int param_7,
               int param_8)

{
  int iVar1;
  ushort uVar2;
  uchar **ppuVar3;
  uchar *puVar4;
  uint uVar5;
  ushort uVar6;
  int iVar7;
  int unaff_DI;
  undefined2 uVar8;
  undefined2 uVar9;
  ushort *puVar10;
  astruct_43 *paVar11;
  ulong uVar12;
  int iStack220;
  uchar *local_c4;
  uchar *puStack194;
  undefined *local_c0;
  ushort uStack190;
  int iStack188;
  astruct_18 *paStack184;
  int iStack180;
  astruct_18 *paStack178;
  astruct_18 *paStack174;
  uint uStack170;
  ushort uStack168;
  uint uStack166;
  uint uStack164;
  uint uStack162;
  uchar **ppuStack160;
  int iStack158;
  int iStack156;
  int iStack154;
  undefined2 uStack152;
  char *pcStack150;
  uchar local_92 [0x80];
  ulong uStack18;
  ulong uStack14;
  undefined2 uStack10;
  undefined2 uStack8;
  int *piStack6;
  
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_3,0x1000);
    PTR_LOOP_1050_5f2e = param_3;
  }
  else {
  }
  local_c4 = PTR_LOOP_1050_5f2c;
  puStack194 = PTR_LOOP_1050_5f2e;
  uVar2 = fn_ptr_op_1000_1708(0x20,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  paStack184 = (astruct_18 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar2);
  puVar4 = (uchar *)((uint)PTR_LOOP_1050_5f2e | uVar2);
  if (puVar4 == (uchar *)0x0) {
    uVar2 = 0x0;
    puVar4 = (uchar *)0x0;
  }
  else {
    pass1_1030_84ae(CONCAT22(PTR_LOOP_1050_5f2e,uVar2));
  }
  piStack6 = (int *)CONCAT22(puVar4,uVar2);
  *piStack6 = param_8;
  pass1_1008_3f62((ushort *)CONCAT13((char)((uint)puVar4 >> 0x8),CONCAT12((char)puVar4,uVar2 + 0x8)),
                  (ushort *)CONCAT22(0x1050,(int)&USHORT_1050_65e6 + param_8 * 0xa));
  if (param_7 != 0x0) {
    puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,(ushort)param_1,puVar4,unaff_DI);
    uStack8 = (undefined2)((ulong)puVar10 >> 0x10);
    uStack10 = SUB42(puVar10,0x0);
    uStack14 = pass1_1018_04b8((ulong)puVar10);
    uVar5 = (uint)(uStack14 >> 0x10);
    uVar2 = (ushort)uStack14;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,uVar5);
    uStack18 = CONCAT22(uVar5,uVar2);
    pcStack150 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)(_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar6 = (ushort)((ulong)pcStack150 >> 0x10);
    uVar2 = pass1_1030_2a98(uStack18);
    uVar8 = (undefined2)((ulong)piStack6 >> 0x10);
    *(ushort *)((int)piStack6 + 0x2) = uVar2;
    sys_1000_3f9c(local_92,param_1,(ushort)pcStack150,(ushort)((ulong)pcStack150 >> 0x10),uVar2,&stack0xfffe,uVar8,
                  0x1000,param_1,param_2);
    uVar2 = str_op_1008_60e8((char *)CONCAT22(param_1,local_92),uVar6);
    uVar8 = (undefined2)((ulong)piStack6 >> 0x10);
    *(ushort *)((int)piStack6 + 0x4) = uVar2;
    *(ushort *)((int)piStack6 + 0x6) = uVar6;
    paVar11 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,*(ushort *)(param_8 * 0xa + 0x65ec),(ushort)param_1);
    uVar8 = (undefined2)((ulong)piStack6 >> 0x10);
    *(undefined2 *)((int)piStack6 + 0xe) = (int)paVar11;
    *(undefined2 *)((int)piStack6 + 0x10) = (int)((ulong)paVar11 >> 0x10);
    paVar11 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,*(ushort *)(param_8 * 0xa + 0x65ee),(ushort)param_1);
    uVar8 = (undefined2)((ulong)piStack6 >> 0x10);
    iVar7 = (int)piStack6;
    *(undefined2 *)(iVar7 + 0x12) = (int)paVar11;
    *(undefined2 *)(iVar7 + 0x14) = (int)((ulong)paVar11 >> 0x10);
    uVar12 = pass1_1008_4772(*(astruct_76 **)(iVar7 + 0xe));
    uStack152 = (undefined2)(uVar12 >> 0x10);
    iStack154 = (int)uVar12;
    iStack156 = *(int *)(iStack154 + 0x4) + -0x1;
    iStack158 = *(int *)(iStack154 + 0x8) + -0x1;
    if (param_6 != 0x0) {
      ppuStack160 = (uchar **)((int)&PTR_LOOP_1050_000e + 0x1);
      if (uStack14 == 0x0) {
        debug_print_1008_6048((ulong)s_get_site_data_without_planet__1050_56de,0x1008,param_1);
      }
      else {
        ppuVar3 = &local_c4;
        pass1_1030_2f1a(uStack18,(ushort *)CONCAT13((char)((uint)param_1 >> 0x8),CONCAT12((char)param_1,&local_c0)),
                        (ushort *)CONCAT22(param_1,ppuVar3));
        pass1_1008_612e((int)local_c4,(int)local_c0,(uint)ppuVar3);
        ppuStack160 = ppuVar3;
      }
      iVar7 = (int)ppuStack160 * 0xa;
      uVar8 = (undefined2)((ulong)piStack6 >> 0x10);
      *(int *)((int)piStack6 + 0x1c) = iVar7;
      *(int *)((int)piStack6 + 0x1c) = iVar7 / 0x64;
      puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)param_1,(uchar *)(iVar7 % 0x64),unaff_DI);
      puStack194 = (uchar *)((ulong)puVar10 >> 0x10);
      local_c4 = (uchar *)puVar10;
      local_c0 = PTR_LOOP_1050_13ae;
      uVar2 = 0x84;
      puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,(ushort)param_1,puStack194,unaff_DI);
      uStack190 = pass1_1010_65d0((ushort)param_1,(ulong)puVar10,uVar2);
      iStack188 = 0x3c;
      if ((int)local_c0 < 0x3) {
        if (0x0 < (int)uStack190) {
          iStack188 = 0x5a;
        }
      }
      else {
        if (uStack190 == 0x1) {
          iStack188 = 0x44;
        }
        else {
          if (uStack190 == 0x2) {
            iStack188 = 0x4b;
          }
          else {
            if (uStack190 == 0x3) {
              iStack188 = 0x53;
            }
            else {
              if (uStack190 == 0x4) {
                iStack188 = 0x5a;
              }
            }
          }
        }
      }
      iVar7 = iStack188 * (int)ppuStack160;
      ppuStack160 = (uchar **)(iVar7 / 0x64);
      puVar4 = (uchar *)(iVar7 % 0x64);
      uVar8 = (undefined2)((ulong)piStack6 >> 0x10);
      *(int *)((int)piStack6 + 0x1a) = (int)ppuStack160;
      uStack164 = (int)ppuStack160 + *(int *)((int)piStack6 + 0x1c);
      uVar2 = uStack164 * 0x6;
      uStack162 = uStack164;
      mem_op_1000_179c(uVar2,puVar4,0x1000);
      paStack184 = (astruct_18 *)CONCAT22(puVar4,uVar2);
      PTR_LOOP_1050_5f2e = (undefined *)((uint)puVar4 | uVar2);
      if (PTR_LOOP_1050_5f2e == (undefined *)0x0) {
        *(undefined4 *)((int)piStack6 + 0x16) = 0x0;
      }
      else {
        pass1_1000_5586((uchar *)0x3e38,0x1008,uStack164,0x6,uVar2,(ushort)puVar4);
        *(undefined4 *)((int)piStack6 + 0x16) = paStack184;
      }
      uStack170 = uStack162 * 0x2;
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
      }
      else {
      }
      uVar2 = fn_ptr_op_1000_1708(uStack170,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
      paStack174 = (astruct_18 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar2);
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
      }
      else {
      }
      uVar2 = fn_ptr_op_1000_1708(uStack170,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
      paStack178 = (astruct_18 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar2);
      iStack180 = 0x0;
LAB_1030_4b57:
      uVar5 = uStack162;
      if (iStack180 < (int)uStack162) {
        do {
          pass1_1008_612e(0x0,iStack156,uVar5);
          uStack166 = uVar5;
          pass1_1008_612e(0x0,iStack158,uVar5);
          iStack220 = 0x0;
          while( true ) {
            iVar7 = (int)paStack174;
            uVar8 = (undefined2)((ulong)paStack174 >> 0x10);
            uVar9 = (undefined2)((ulong)paStack178 >> 0x10);
            uStack168 = uVar5;
            if (iStack180 <= iStack220) {
              iVar1 = iStack180 * 0x2;
              *(uint *)(iVar1 + iVar7) = uStack166;
              *(ushort *)(iVar1 + (int)paStack178) = uVar5;
              uVar12 = *(ulong *)((int)piStack6 + 0x16);
              pass1_1008_3e76((ushort *)(uVar12 & 0xffff0000 | (ulong)(uint)((int)uVar12 + iStack180 * 0x6)),0x0,uVar5,
                              *(ushort *)(iVar1 + iVar7));
              iStack180 = iStack180 + 0x1;
              goto LAB_1030_4b57;
            }
            if ((*(uint *)(iStack220 * 0x2 + iVar7) == uStack166) &&
               (*(ushort *)(iStack220 * 0x2 + (int)paStack178) == uVar5)) break;
            iStack220 = iStack220 + 0x1;
          }
        } while( true );
      }
      fn_ptr_1000_17ce(paStack174,0x1000);
      paStack184 = paStack178;
      fn_ptr_1000_17ce(paStack178,0x1000);
    }
  }
  return;
}
