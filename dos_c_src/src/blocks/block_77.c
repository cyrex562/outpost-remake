
void __stdcall16far
pass1_1030_10b0(ushort param_1,ushort param_2,ushort param_3,undefined4 param_4,ulong param_5,uchar *param_6,
               astruct_179 *param_7,ushort param_8,ushort param_9,ushort param_10)

{
  ulong uVar1;
  ulong uVar2;
  ushort uVar3;
  ushort uVar4;
  uint uVar5;
  ushort *puVar6;
  uint uStack8;
  
  puVar6 = switch_1030_07ac(param_1,param_2,param_3,(ushort)param_4,(ushort)((ulong)param_4 >> 0x10),param_5,param_6,
                            param_7,param_8,param_9,param_10);
  uVar3 = (ushort)((ulong)puVar6 >> 0x10);
  uVar1 = *(ulong *)((ushort)puVar6 + 0x4);
  uVar2 = uVar1;
  uVar4 = uVar3;
  pass1_1028_e1ec(CONCAT22(param_2,param_1),(ushort)param_5,(uint)(param_5 >> 0x10));
  uVar5 = uVar4 | (uint)uVar2;
  if (uVar5 != 0x0) {
    pass1_1030_7e5a(uVar2 & 0xffff | (ulong)uVar4 << 0x10,uVar1,uVar5);
  }
  uStack8 = (uint)(uVar1 >> 0x10);
  pass1_1030_1358(*(ulong *)(param_1 + 0x26),(ushort)puVar6,uVar3,uVar1 & 0xffff | (ulong)(uStack8 & 0xff) << 0x10,
                  param_10);
  return;
}



void __stdcall16far pass1_1030_1120(ulong param_1,ushort param_2,uchar *param_3,ushort param_4)

{
  uchar *puVar1;
  
  mem_op_1000_179c(0x3b2,param_3,0x1000);
  puVar1 = (uchar *)((uint)param_3 | param_2);
  if (puVar1 == (uchar *)0x0) {
    param_2 = 0x0;
    puVar1 = (uchar *)0x0;
  }
  else {
    struct_1030_2112((ushort *)CONCAT22(param_3,param_2),0x0,param_2,puVar1);
  }
  pass1_1030_1358(*(ulong *)((int)param_1 + 0x2a),param_2,(ushort)puVar1,
                  *(ulong *)(param_2 + 0x4) & 0xffff | (ulong)(*(uint *)(param_2 + 0x6) & 0xff) << 0x10,param_4);
  return;
}



astruct_18 * __stdcall16far pass1_1030_117a(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1030_11aa(ushort *param_1,long param_2,long param_3,ushort param_4)

{
  astruct_156 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_156 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x6 = (undefined4 *)0x0;
  iVar1->field_0xa = 0x0;
  iVar1->field_0xe = param_3;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x16 = param_2;
  iVar1->field_0x1a = 0x1;
  *param_1 = (int)s_462_bmp_1050_1620 + 0x4;
  iVar1->field_0x2 = 0x1030;
  if (iVar1->field_0xe == 0x0) {
    iVar1->field_0xe = 0x5;
  }
  if (iVar1->field_0x16 == 0x0) {
    iVar1->field_0x16 = 0x5;
  }
  struct_1030_1550((ulong)param_1,param_4);
  *iVar1->field_0x6 = 0x0;
  return;
}



void __stdcall16far pass1_1030_1244(ushort *param_1)

{
  ulong *puVar1;
  undefined4 *puVar2;
  uint uVar3;
  code **ppcVar4;
  astruct_18 *paVar5;
  astruct_606 *iVar6;
  int iVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  ulong uStack6;
  
  uVar9 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (astruct_606 *)param_1;
  *param_1 = (int)s_462_bmp_1050_1620 + 0x4;
  iVar6->field_0x2 = 0x1030;
  if (iVar6->field_0x1a != 0x0) {
    uStack6 = 0x1;
    while( true ) {
      puVar1 = &iVar6->field_0xa;
      if (*puVar1 < uStack6 || *puVar1 == uStack6) break;
      iVar8 = (int)uStack6 * 0x4;
      paVar5 = iVar6->field_0x6;
      uVar10 = (undefined2)((ulong)paVar5 >> 0x10);
      iVar7 = (int)paVar5;
      puVar2 = (undefined4 *)*(uint *)(iVar7 + iVar8);
      uVar3 = *(uint *)(iVar7 + iVar8 + 0x2);
      if ((uVar3 | (uint)puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      uStack6 = uStack6 + 0x1;
    }
  }
  fn_ptr_1000_17ce(iVar6->field_0x6,0x1000);
  *param_1 = 0x389a;
  iVar6->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1030_12ca(ulong param_1)

{
  ulong *puVar1;
  undefined4 uVar2;
  astruct_176 *iVar3;
  undefined2 uVar3;
  ulong uStack6;
  
  uStack6 = 0x1;
  while( true ) {
    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar3 = (astruct_176 *)param_1;
    puVar1 = &iVar3->field_0xa;
    if (*puVar1 < uStack6 || *puVar1 == uStack6) {
      iVar3->field_0x4 = 0x0;
      return;
    }
    uVar2 = iVar3->field_0x6;
    if (*(long *)((int)uVar2 + (int)uStack6 * 0x4) == 0x0) break;
    uStack6 = uStack6 + 0x1;
  }
  return;
}



void __stdcall16far bad_1030_1312(void)

{
  return;
}



void __stdcall16far pass1_1030_1358(ulong param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5)

{
  ulong *puVar1;
  uint *puVar2;
  long lVar3;
  astruct_291 *iVar4;
  int iVar5;
  uint uVar6;
  undefined2 uVar7;
  bool bVar8;
  
  if (param_4 == 0x0) {
    return;
  }
  uVar6 = (uint)(param_1 >> 0x10);
  iVar4 = (astruct_291 *)param_1;
  puVar1 = (ulong *)&iVar4->field_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4->field_0x6 == 0x0)) {
    puVar2 = (uint *)((int)&iVar4->field_0x12 + 0x2);
    bVar8 = *puVar2 < param_4._2_2_;
    if ((bVar8 || *puVar2 == param_4._2_2_) &&
       ((bVar8 || (puVar1 = &iVar4->field_0x12, *(uint *)puVar1 < (uint)param_4 || *(uint *)puVar1 == (uint)param_4))))
    {
      struct_1030_1550(param_1 & 0xffff | (ulong)uVar6 << 0x10,param_5);
    }
    puVar1 = &iVar4->field_0x12;
    if (*puVar1 < param_4 || *puVar1 == param_4) {
      return;
    }
    if (iVar4->field_0x6 == 0x0) {
      return;
    }
    puVar2 = &iVar4->field_0xc;
    bVar8 = *puVar2 < param_4._2_2_;
    if ((bVar8 || *puVar2 == param_4._2_2_) &&
       ((bVar8 || (puVar2 = &iVar4->field_0xa, *puVar2 < (uint)param_4 || *puVar2 == (uint)param_4)))) {
      iVar4->field_0xa = (uint)(param_4 + 0x1);
      iVar4->field_0xc = (uint)(param_4 + 0x1 >> 0x10);
    }
  }
  lVar3 = iVar4->field_0x6;
  uVar7 = (undefined2)((ulong)lVar3 >> 0x10);
  iVar5 = (int)lVar3;
  *(ushort *)(iVar5 + (uint)param_4 * 0x4) = param_2;
  *(ushort *)(iVar5 + (uint)param_4 * 0x4 + 0x2) = param_3;
  return;
}



ushort __stdcall16far pass1_1030_13f6(ulong param_1,ulong param_2,uint param_3,uint param_4,ushort param_5)

{
  code **ppcVar1;
  undefined2 uVar2;
  undefined4 *puStack8;
  ushort uStack4;
  
  uStack4 = 0x0;
  bad_1030_1312();
  puStack8 = (undefined4 *)CONCAT22(param_4,param_3);
  if ((param_4 | param_3) != 0x0) {
    uStack4 = 0x1;
    uVar2 = (undefined2)(param_1 >> 0x10);
    if ((*(int *)((int)param_1 + 0x1a) != 0x0) && ((param_4 | param_3) != 0x0)) {
      ppcVar1 = (code **)*puStack8;
      (**ppcVar1)();
    }
    pass1_1030_1358(param_1,0x0,0x0,param_2,param_5);
    *(undefined2 *)((int)param_1 + 0x4) = 0x1;
  }
  return uStack4;
}



void __stdcall16far pass1_1030_145a(ulong param_1,long param_2)

{
  ulong uVar1;
  undefined2 uVar2;
  astruct_346 *iVar4;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_346 *)param_1;
  fn_ptr_1000_17ce((astruct_18 *)iVar4->field_0x6,0x1000);
  iVar4->field_0x6 = 0x0;
  iVar4->field_0xa = 0x0;
  uVar1 = iVar4->field_0x16 + param_2;
  uVar2 = (undefined2)(uVar1 >> 0x10);
  if (uVar1 < iVar4->field_0xe) {
    uVar1 = (ulong)*(uint *)&iVar4->field_0xe;
    uVar2 = *(undefined2 *)((int)&iVar4->field_0xe + 0x2);
  }
  *(int *)&iVar4->field_0xe = (int)uVar1;
  *(undefined2 *)((int)&iVar4->field_0xe + 0x2) = uVar2;
  iVar4->field_0x12 = 0x0;
  return;
}



void __stdcall16far pass1_1030_14b4(ulong param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5)

{
  ulong *puVar1;
  uint *puVar2;
  long lVar3;
  astruct_345 *iVar5;
  astruct_344 *iVar4;
  uint uVar4;
  undefined2 uVar5;
  bool bVar6;
  
  uVar4 = (uint)(param_1 >> 0x10);
  iVar5 = (astruct_345 *)param_1;
  puVar1 = (ulong *)&iVar5->field_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar5->field_0x6 == 0x0)) {
    puVar2 = (uint *)((int)&iVar5->field_0x12 + 0x2);
    bVar6 = *puVar2 < param_4._2_2_;
    if ((bVar6 || *puVar2 == param_4._2_2_) &&
       ((bVar6 || (puVar1 = &iVar5->field_0x12, *(uint *)puVar1 < (uint)param_4 || *(uint *)puVar1 == (uint)param_4))))
    {
      struct_1030_1550(param_1 & 0xffff | (ulong)uVar4 << 0x10,param_5);
    }
    puVar1 = &iVar5->field_0x12;
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar5->field_0x6 == 0x0)) {
      return;
    }
    puVar2 = &iVar5->field_0xc;
    bVar6 = *puVar2 < param_4._2_2_;
    if ((bVar6 || *puVar2 == param_4._2_2_) &&
       ((bVar6 || (puVar2 = &iVar5->field_0xa, *puVar2 < (uint)param_4 || *puVar2 == (uint)param_4)))) {
      iVar5->field_0xa = (uint)(param_4 + 0x1);
      iVar5->field_0xc = (uint)(param_4 + 0x1 >> 0x10);
    }
  }
  lVar3 = iVar5->field_0x6;
  uVar5 = (undefined2)((ulong)lVar3 >> 0x10);
  iVar4 = (astruct_344 *)lVar3;
  *(ushort *)(iVar4 + (uint)param_4 * 0x4) = param_2;
  *(ushort *)(iVar4 + (uint)param_4 * 0x4 + 0x2) = param_3;
  return;
}



void __stdcall16far pass1_1030_154c(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1030_1550(ulong param_1,ushort param_2)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  astruct_157 *iVar5;
  undefined2 uVar4;
  long lVar5;
  long lStack10;
  undefined4 uStack6;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar5 = (astruct_157 *)param_1;
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



astruct_18 * __stdcall16far pass1_1030_15fe(astruct_18 *param_1,byte param_2)

{
  pass1_1030_1244(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1030_1628(ushort *param_1)

{
  astruct_181 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_181 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = 0x0;
  *param_1 = 0x17ba;
  iVar1->field_0x2 = 0x1030;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_165e(ushort *param_1,ulong param_2,ulong param_3,ushort param_4)

{
  astruct_175 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_175 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  *(undefined4 *)&iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = param_3;
  *param_1 = 0x17ba;
  iVar1->field_0x2 = 0x1030;
  pass1_1030_5c8a(_PTR_LOOP_1050_5736,param_2);
  iVar1->field_0x4 = (int)param_3;
  iVar1->field_0x6 = param_4;
  return;
}



void __stdcall16far pass1_1030_16b2(ushort *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x17ba;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  return;
}



void __stdcall16far pass1_1030_16d6(ulong param_1,ulong param_2,ushort param_3)

{
  BOOL16 BVar1;
  undefined2 uVar2;
  ushort uVar3;
  undefined4 local_10 [0x2];
  undefined4 local_8;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  local_10[0] = *(undefined4 *)((int)param_1 + 0x4);
  uVar3 = (ushort)(param_2 >> 0x10);
  BVar1 = write_to_file_1008_7e1c((ushort)param_2,uVar3,(ushort)local_10,param_3,(char *)0x4,0x1008);
  if (BVar1 != 0x0) {
    local_8 = *(undefined4 *)((int)param_1 + 0x8);
    BVar1 = write_to_file_1008_7e1c((ushort)param_2,uVar3,(ushort)&local_8,param_3,(char *)0x4,0x1008);
    if (BVar1 != 0x0) {
      return;
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  return;
}



void __stdcall16far file_1030_1730(ulong param_1,ulong param_2)

{
  ushort uVar1;
  BOOL16 BVar2;
  ushort uVar3;
  
  uVar1 = (ushort)(param_1 >> 0x10);
  uVar3 = (ushort)(param_2 >> 0x10);
  BVar2 = read_file_1008_7dee((ushort)param_2,uVar3,(int)param_1 + 0x4,0x0,uVar1,0x4,0x1008);
  if (BVar2 != 0x0) {
    BVar2 = read_file_1008_7dee((ushort)param_2,uVar3,(int)param_1 + 0x8,0x0,uVar1,0x4,0x1008);
    if (BVar2 != 0x0) {
      return;
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  return;
}



void __stdcall16far pass1_1030_177a(ulong param_1,ulong param_2)

{
  *(ulong *)((int)param_1 + 0x8) = param_2;
  return;
}



astruct_18 * __stdcall16far pass1_1030_1794(astruct_18 *param_1,byte param_2)

{
  pass1_1030_16b2(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1030_17ce(ushort *param_1,ulong param_2,ulong param_3)

{
  astruct_75 *paVar1;
  undefined4 uVar2;
  undefined2 uVar3;
  astruct_343 *iVar3;
  
  iVar3 = (astruct_343 *)param_1;
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  paVar1 = (astruct_75 *)struct_1030_1628(param_1);
  *(undefined4 *)&iVar3->field_0xc = 0x0;
  *param_1 = 0x1a16;
  iVar3->field_0x2 = 0x1030;
  if ((param_3 != 0x0) || (param_2 != 0x0)) {
    mem_op_1000_179c(0x18,(uchar *)((ulong)paVar1 >> 0x10),0x1000);
    if (paVar1 == (astruct_75 *)0x0) {
      uVar2 = 0x0;
    }
    else {
      uVar2 = struct_op_1030_1cd8(paVar1,param_2,param_3);
    }
    iVar3->field_0xc = (int)uVar2;
    iVar3->field_0xe = (int)((ulong)uVar2 >> 0x10);
  }
  return param_1;
}



ushort * __stdcall16far
pass1_1030_183c(ushort *param_1,ulong param_2,ulong param_3,ulong param_4,ulong param_5,uint param_6,uchar *param_7)

{
  undefined4 uVar1;
  undefined2 uVar2;
  astruct_351 *iVar2;
  
  iVar2 = (astruct_351 *)param_1;
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  pass1_1030_165e(param_1,param_4,param_5,(ushort)param_7);
  *(undefined4 *)&iVar2->field_0xc = 0x0;
  *param_1 = 0x1a16;
  iVar2->field_0x2 = 0x1030;
  if ((param_3 != 0x0) || (param_2 != 0x0)) {
    mem_op_1000_179c(0x18,param_7,0x1000);
    if (((uint)param_7 | param_6) == 0x0) {
      uVar1 = 0x0;
    }
    else {
      uVar1 = struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_7,param_6),param_2,param_3);
    }
    iVar2->field_0xc = (int)uVar1;
    iVar2->field_0xe = (int)((ulong)uVar1 >> 0x10);
  }
  return param_1;
}



void __stdcall16far pass1_1030_18b2(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x1a16;
  *(undefined2 *)(iVar4 + 0x2) = 0x1030;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xc);
  uVar2 = *(uint *)(iVar4 + 0xe);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1030_16b2(param_1);
  return;
}



void __stdcall16far pass1_1030_18f0(ulong param_1,int param_2,int param_3,ushort param_4)

{
  code **ppcVar1;
  ulong uVar2;
  undefined2 extraout_DX;
  int extraout_DX_00;
  int iVar3;
  undefined2 uVar4;
  ulong uStack10;
  ulong uStack6;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0xc) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0xc) + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX,param_4);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1) {
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0xc) + 0x4);
      uVar2 = uStack6;
      (**ppcVar1)();
      if (((int)uVar2 == param_2) && (extraout_DX_00 == param_3)) {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0xc) + 0x8);
        (**ppcVar1)();
      }
    }
  }
  return;
}



ushort __stdcall16far pass1_1030_1972(void)

{
  return 0x1;
}



ushort __stdcall16far pass1_1030_1978(ulong param_1,ulong param_2,uint16_t param_3,ushort param_4)

{
  pass1_1030_16d6(param_1,param_2,param_4);
  if (param_3 != 0x0) {
    write_to_file_1008_7954(param_2,(undefined4 *)*(undefined4 *)((int)param_1 + 0xc),param_3,0x1008,param_4);
    if (param_3 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return param_3;
    }
    param_3 = 0x1;
  }
  return param_3;
}



void __stdcall16far file_1030_19b4(ulong param_1,ulong param_2,int param_3,ushort param_4,ushort param_5)

{
  long *plVar1;
  
  file_1030_1730(param_1,param_2);
  if (param_3 != 0x0) {
    plVar1 = (long *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0xc));
    file_1008_76e4(param_2,plVar1,0x1008,param_5,param_4);
    if ((int)plVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return;
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_19f0(astruct_18 *param_1,byte param_2)

{
  pass1_1030_18b2(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1030_1a32(ushort *param_1,uint param_2,uchar *param_3)

{
  pass1_1030_183c(param_1,0x1,0x16,0xff000000,0x0,param_2,param_3);
  PTR_LOOP_1050_5168 = (undefined *)((ulong)param_1 >> 0x10);
  PTR_LOOP_1050_5166 = (undefined *)param_1;
  *(undefined4 *)(PTR_LOOP_1050_5166 + 0x10) = 0x0;
  *param_1 = 0x1cbc;
  *(undefined2 *)(PTR_LOOP_1050_5166 + 0x2) = 0x1030;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_1a74(ushort *param_1)

{
  *param_1 = 0x1cbc;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
  _PTR_LOOP_1050_5166 = 0x0;
  pass1_1030_18b2(param_1);
  return;
}



undefined2 __stdcall16far pass1_1030_1a9c(ulong param_1,ulong param_2,ushort param_3)

{
  undefined4 uVar1;
  int *piVar2;
  uint16_t in_AX;
  ushort uVar3;
  BOOL16 BVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 local_c [0x5];
  
  uVar3 = pass1_1030_1978(param_1,param_2,in_AX,param_3);
  if (uVar3 != 0x0) {
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (int)param_1;
    local_c[0] = *(undefined2 *)*(undefined4 *)(iVar5 + 0x10);
    uVar3 = (ushort)(param_2 >> 0x10);
    BVar4 = write_to_file_1008_7e1c((ushort)param_2,uVar3,(ushort)local_c,param_3,(char *)0x2,0x1008);
    if (BVar4 != 0x0) {
      if (**(int **)(iVar5 + 0x10) == 0x0) {
        return 0x1;
      }
      piVar2 = *(int **)(iVar5 + 0x10);
      uVar1 = *(undefined4 *)((int)piVar2 + 0x2);
      BVar4 = write_to_file_1008_7e1c
                        ((ushort)param_2,uVar3,(ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),
                         (char *)(ulong)(uint)(*piVar2 * 0x2),0x1008);
      if (BVar4 != 0x0) {
        return 0x1;
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined2 __stdcall16far file_1030_1b18(ulong param_1,ulong param_2,int param_3,undefined *param_4,undefined2 param_5)

{
  undefined4 uVar1;
  int *piVar2;
  ushort uVar3;
  ushort uVar4;
  BOOL16 BVar5;
  ushort uVar6;
  uchar *puVar7;
  astruct_368 *iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  astruct_370 *iVar10;
  astruct_369 *iVar9;
  
  iVar10 = (astruct_370 *)param_1;
  uVar9 = (undefined2)(param_1 >> 0x10);
  file_1030_19b4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 != 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_4,0x1000);
      PTR_LOOP_1050_5f2e = param_4;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(0x6,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
    *(ushort *)&iVar10->field_0x10 = uVar4;
    *(undefined2 *)((int)&iVar10->field_0x10 + 0x2) = PTR_LOOP_1050_5f2e;
    puVar7 = *(uchar **)((int)&iVar10->field_0x10 + 0x2);
    uVar4 = (ushort)(param_2 >> 0x10);
    BVar5 = read_file_1008_7dee((ushort)param_2,uVar4,*(ushort *)&iVar10->field_0x10,0x0,(ushort)puVar7,0x2,0x1008);
    if (BVar5 != 0x0) {
      piVar2 = iVar10->field_0x10;
      if (*piVar2 == 0x0) {
        return 0x1;
      }
      uVar3 = *piVar2 * 0x2;
      uVar6 = uVar3;
      mem_op_1000_179c(uVar3,puVar7,0x1000);
      piVar2 = iVar10->field_0x10;
      uVar8 = (undefined2)((ulong)piVar2 >> 0x10);
      iVar7 = (astruct_368 *)piVar2;
      iVar7->field_0x2 = uVar6;
      iVar7->field_0x4 = puVar7;
      piVar2 = iVar10->field_0x10;
      uVar1 = *(undefined4 *)((int)piVar2 + 0x2);
      BVar5 = read_file_1008_7dee((ushort)param_2,uVar4,(ushort)uVar1,0x0,(ushort)((ulong)uVar1 >> 0x10),uVar3,0x1008);
      if (BVar5 != 0x0) {
        return 0x1;
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_1be2(ulong param_1,uint param_2,uchar *param_3)

{
  code **ppcVar1;
  uint *puVar2;
  uchar *puVar3;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  int iVar4;
  undefined2 uVar5;
  ulong uVar6;
  uint uStack4;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar3 = param_3;
  if (*(long *)(iVar4 + 0xc) == 0x0) {
    mem_op_1000_179c(0x18,param_3,0x1000);
    puVar3 = (uchar *)((uint)param_3 | param_2);
    if (puVar3 == (uchar *)0x0) {
      *(undefined4 *)(iVar4 + 0xc) = 0x0;
    }
    else {
      struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_3,param_2),0x5,0x5);
      *(uint *)(iVar4 + 0xc) = param_2;
      *(uchar **)(iVar4 + 0xe) = extraout_DX;
      puVar3 = extraout_DX;
    }
  }
  for (uStack4 = 0x0; puVar2 = *(uint **)(iVar4 + 0x10), uStack4 <= *puVar2 && *puVar2 != uStack4;
      uStack4 = uStack4 + 0x1) {
    uVar6 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(ushort)puVar3,0x1);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0xc) + 0x8);
    (**ppcVar1)((int)&USHORT_1050_1028,*(undefined4 *)(iVar4 + 0xc),(int)uVar6,(int)(uVar6 >> 0x10),uStack4,0x0);
    puVar3 = extraout_DX_00;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_1c96(astruct_18 *param_1,byte param_2)

{
  pass1_1030_1a74(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_op_1030_1cd8(astruct_75 *param_1,ulong param_2,ulong param_3)

{
  astruct_75 *struct_var1;
  astruct_75 *struct_var2;
  
  struct_var2 = (astruct_75 *)((ulong)param_1 >> 0x10);
  struct_var1 = (astruct_75 *)param_1;
  param_1->field_0x0 = 0x389a;
  struct_var1->field_0x2 = 0x1008;
  struct_var1->field_0x4 = 0x0;
  struct_var1->field_0x8 = 0x0;
  struct_var1->field_0xc = param_3;
  struct_var1->field_0x10 = 0x0;
  struct_var1->field_0x14 = param_2;
  param_1->field_0x0 = 0x2044;
  struct_var1->field_0x2 = 0x1030;
  return;
}



void __stdcall16far pass1_1030_1d28(ushort *param_1)

{
  astruct_594 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_594 *)param_1;
  *param_1 = 0x2044;
  iVar1->field_0x2 = 0x1030;
  fn_ptr_1000_17ce(iVar1->field_0x4,0x1000);
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_1d58(ulong param_1)

{
  code **ppcVar1;
  undefined4 uVar2;
  
  ppcVar1 = (code **)((int)*(undefined4 *)param_1 + 0x4);
  uVar2 = (**ppcVar1)();
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)((ulong)uVar2 >> 0x10));
  return;
}



ulong __stdcall16far pass1_1030_1d7c(uint param_1,uint param_2,ulong param_3)

{
  ulong uVar1;
  
  pass1_1030_1d58(param_3);
  if ((param_2 | param_1) != 0x0) {
    uVar1 = struct_op_1030_73a8(CONCAT22(param_2,param_1));
    return uVar1;
  }
  return 0x0;
}



ulong __stdcall16far pass1_1030_1daa(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0xa),*(undefined2 *)((int)param_1 + 0x8));
}



void __stdcall16far pass1_1030_1dbc(void)

{
  return;
}



void __stdcall16far pass1_1030_1dfc(ulong *param_1,ushort param_2,ushort param_3,ulong param_4)

{
  ulong *puVar1;
  uint *puVar2;
  code **ppcVar3;
  undefined4 uVar4;
  int iVar5;
  undefined2 uVar6;
  bool bVar7;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  puVar1 = (ulong *)(iVar5 + 0x8);
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (*(long *)(iVar5 + 0x4) == 0x0)) {
    puVar2 = (uint *)(iVar5 + 0x12);
    bVar7 = *puVar2 < param_4._2_2_;
    if ((bVar7 || *puVar2 == param_4._2_2_) &&
       ((bVar7 || (puVar2 = (uint *)(iVar5 + 0x10), *puVar2 < (uint)param_4 || *puVar2 == (uint)param_4)))) {
      ppcVar3 = (code **)((int)*param_1 + 0x20);
      (**ppcVar3)();
    }
    puVar1 = (ulong *)(iVar5 + 0x10);
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (*(long *)(iVar5 + 0x4) == 0x0)) {
      return;
    }
    puVar2 = (uint *)(iVar5 + 0xa);
    bVar7 = *puVar2 < param_4._2_2_;
    if ((bVar7 || *puVar2 == param_4._2_2_) &&
       ((bVar7 || (puVar2 = (uint *)(iVar5 + 0x8), *puVar2 < (uint)param_4 || *puVar2 == (uint)param_4)))) {
      *(undefined2 *)(iVar5 + 0x8) = (int)(param_4 + 0x1);
      *(undefined2 *)(iVar5 + 0xa) = (int)(param_4 + 0x1 >> 0x10);
    }
  }
  uVar4 = *(undefined4 *)(iVar5 + 0x4);
  uVar6 = (undefined2)((ulong)uVar4 >> 0x10);
  iVar5 = (int)uVar4;
  *(ushort *)(iVar5 + (uint)param_4 * 0x4) = param_2;
  *(ushort *)(iVar5 + (uint)param_4 * 0x4 + 0x2) = param_3;
  return;
}



void __stdcall16far pass1_1030_1e96(ulong *param_1)

{
  ulong *puVar1;
  code **ppcVar2;
  undefined4 uVar3;
  undefined2 uVar4;
  ulong uStack6;
  
  uStack6 = 0x0;
  while( true ) {
    uVar4 = (undefined2)((ulong)param_1 >> 0x10);
    puVar1 = (ulong *)((int)param_1 + 0x8);
    if ((*puVar1 < uStack6 || *puVar1 == uStack6) ||
       (uVar3 = *(undefined4 *)((int)param_1 + 0x4), *(long *)((int)uVar3 + (int)uStack6 * 0x4) == 0x0)) break;
    uStack6 = uStack6 + 0x1;
  }
  ppcVar2 = (code **)((int)*param_1 + 0x8);
  (**ppcVar2)();
  return;
}

