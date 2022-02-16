
void __stdcall16far pass1_1008_b200(ulong param_1,ushort param_2)

{
  ulong uVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  undefined *puVar4;
  astruct_195 *uVar5;
  ushort uVar6;
  ulong uVar7;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  uchar *puVar8;
  uchar *puVar9;
  undefined2 extraout_DX_01;
  undefined2 uVar10;
  ushort uVar11;
  uchar *extraout_DX_02;
  astruct_194 *iVar12;
  undefined2 uVar12;
  char *pcVar13;
  undefined local_14 [0x12];
  
  uVar12 = (undefined2)(param_1 >> 0x10);
  iVar12 = (astruct_194 *)param_1;
  if (iVar12->field_0xe != (undefined4 *)0x0) {
    return;
  }
                    // WARNING: Load size is inaccurate
  puVar3 = iVar12->field_0xe;
  puVar9 = *(uchar **)((int)&iVar12->field_0xe + 0x2);
  if (((uint)puVar9 | (uint)puVar3) != 0x0) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)();
    puVar9 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar9,0x1000);
  if (((uint)puVar9 | (uint)puVar3) == 0x0) {
    puVar3 = (undefined4 *)0x0;
    puVar9 = (uchar *)0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar9,puVar3));
    puVar9 = extraout_DX_00;
  }
  *(undefined4 **)&iVar12->field_0xe = puVar3;
  *(uchar **)((int)&iVar12->field_0xe + 0x2) = puVar9;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x400);
  while( true ) {
    puVar8 = puVar9;
    puVar4 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar4));
    puVar9 = (uchar *)((uint)puVar8 | (uint)puVar4);
    if (puVar9 == (uchar *)0x0) break;
    uVar1 = *(ulong *)(puVar4 + 0x4);
    if (*(long *)(puVar4 + 0x200) == 0x8000001) {
      uVar7 = uVar1;
      mem_op_1000_179c(0xc,puVar9,0x1000);
      uVar5 = (astruct_195 *)uVar7;
      if (((uint)puVar9 | (uint)uVar5) == 0x0) {
        uVar5 = (astruct_195 *)0x0;
        uVar10 = 0x0;
      }
      else {
        pass1_1008_b0f2((ushort *)(uVar7 & 0xffff | ZEXT24(puVar9) << 0x10));
        uVar10 = extraout_DX_01;
      }
      pcVar13 = pass1_1038_4d28(CONCAT22(puVar8,puVar4));
      uVar11 = (ushort)((ulong)pcVar13 >> 0x10);
      uVar6 = str_op_1008_60e8(pcVar13,uVar11);
      uVar5->field_0x4 = uVar6;
      uVar5->field_0x6 = uVar11;
      uVar5->field_0x8 = uVar1;
      ppcVar2 = (code **)((int)*iVar12->field_0xe + 0x8);
      (**ppcVar2)((int)&PTR_LOOP_1050_1038,iVar12->field_0xe,uVar5,uVar10);
      puVar9 = extraout_DX_02;
    }
  }
  return;
}



ulong __stdcall16far pass1_1008_b340(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x16) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x16);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22(*(undefined2 *)(iVar2 + 0x6),*(undefined2 *)(iVar2 + 0x4));
  }
  return 0x0;
}



ulong __stdcall16far pass1_1008_b366(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1a) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x1a);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22(*(undefined2 *)(iVar2 + 0x6),*(undefined2 *)(iVar2 + 0x4));
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1008_b38c(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined2 uVar3;
  astruct_197 *iVar3;
  astruct_196 *iVar4;
  undefined2 uVar4;
  ushort *puVar5;
  int iStack4;
  astruct_198 *iVar5;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_196 *)param_1;
  if (iVar4->field_0x12 == (undefined4 *)0x0) {
    mem_op_1000_179c(0xc,param_3,0x1000);
    puVar5 = (ushort *)CONCAT22((uint)param_3 | param_2,param_2);
    if (((uint)param_3 | param_2) == 0x0) {
      iVar4->field_0x12 = (undefined4 *)0x0;
    }
    else {
      puVar5 = (ushort *)set_struct_1008_574a((astruct_21 *)CONCAT22(param_3,param_2));
      *(int *)&iVar4->field_0x12 = (int)puVar5;
      *(undefined2 *)((int)&iVar4->field_0x12 + 0x2) = (int)((ulong)puVar5 >> 0x10);
    }
    for (iStack4 = 0x6d9; iStack4 < 0x6e7; iStack4 = iStack4 + 0x1) {
      if (iStack4 == 0x6e3) {
        pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
        if (*(int *)((int)puVar5 + 0x136) != 0x0) goto LAB_1008_b44a;
      }
      else {
LAB_1008_b44a:
        mem_op_1000_179c(0xa,(uchar *)((ulong)puVar5 >> 0x10),0x1000);
        if (puVar5 == (ushort *)0x0) {
          puVar5 = (ushort *)0x0;
        }
        else {
          puVar5 = pass1_1008_b11e(puVar5);
        }
        uVar3 = (undefined2)((ulong)puVar5 >> 0x10);
        iVar5 = (astruct_198 *)puVar5;
        load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
        iVar5->field_0x4 = (int)puVar5;
        iVar5->field_0x6 = (int)((ulong)puVar5 >> 0x10);
        iVar5->field_0x8 = iStack4 + -0x6d8;
        puVar1 = iVar4->field_0x12;
        ppcVar2 = (code **)((int)*iVar4->field_0x12 + 0x8);
        puVar5 = (ushort *)(**ppcVar2)(0x1010,(int)puVar1,(int)((ulong)puVar1 >> 0x10),iVar5,uVar3);
      }
    }
  }
  return CONCAT22(*(undefined2 *)((int)&iVar4->field_0x12 + 0x2),*(undefined2 *)&iVar4->field_0x12);
}



ulong __stdcall16far pass1_1008_b47a(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x1e) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x1e);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22(*(undefined2 *)(iVar2 + 0x6),*(undefined2 *)(iVar2 + 0x4));
  }
  return 0x0;
}



void __stdcall16far pass1_1008_b4a0(ulong param_1,long param_2,uint param_3,uchar *param_4,ushort param_5)

{
  undefined4 uVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  int iVar4;
  undefined2 uVar5;
  ulong uVar6;
  long lVar7;
  
  iVar4 = (int)param_1;
  uVar5 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x0) {
    *(undefined4 *)(iVar4 + 0x16) = 0x0;
  }
  else {
    pass1_1008_b9ce(param_1,param_2,param_5);
    *(uint *)(iVar4 + 0x16) = param_3;
    *(uchar **)(iVar4 + 0x18) = param_4;
  }
  uVar1 = *(undefined4 *)(iVar4 + 0x16);
  if (*(int *)((int)uVar1 + 0x8) != 0x0) {
    pass1_1008_b200(param_1,param_5);
    uVar6 = pass1_1008_b38c(param_1,param_3,param_4);
    uVar3 = (undefined2)(uVar6 >> 0x10);
    uVar2 = (undefined2)uVar6;
    uVar1 = *(undefined4 *)(iVar4 + 0x16);
    pass1_1008_b85c(param_1,*(long *)((int)uVar1 + 0xa));
    *(undefined2 *)(iVar4 + 0x1a) = uVar2;
    *(undefined2 *)(iVar4 + 0x1c) = uVar3;
    uVar1 = *(undefined4 *)(iVar4 + 0x16);
    lVar7 = pass1_1008_b8ac(param_1,*(int *)((int)uVar1 + 0xe),param_5);
    *(undefined2 *)(iVar4 + 0x1e) = (int)lVar7;
    *(undefined2 *)(iVar4 + 0x20) = (int)((ulong)lVar7 >> 0x10);
    return;
  }
  *(undefined4 *)(iVar4 + 0x1a) = 0x0;
  *(undefined4 *)(iVar4 + 0x1e) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_b544(ulong param_1,int param_2,ushort param_3,ushort param_4)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined2 uVar6;
  int iVar7;
  undefined2 uVar8;
  
  iVar7 = (int)param_1;
  uVar8 = (undefined2)(param_1 >> 0x10);
  if (param_2 != 0x0) {
    if (*(long *)(iVar7 + 0x1a) != 0x0) {
      uVar4 = *(undefined4 *)(iVar7 + 0x16);
      *(undefined2 *)((int)uVar4 + 0x8) = 0x1;
      uVar4 = *(undefined4 *)(iVar7 + 0x1a);
      uVar5 = *(undefined4 *)(iVar7 + 0x16);
      *(undefined4 *)((int)uVar5 + 0xa) = *(undefined4 *)((int)uVar4 + 0x8);
      uVar4 = *(undefined4 *)(iVar7 + 0x1e);
      uVar6 = *(undefined2 *)((int)uVar4 + 0x8);
      uVar4 = *(undefined4 *)(iVar7 + 0x16);
      *(undefined2 *)((int)uVar4 + 0xe) = uVar6;
      uVar4 = *(undefined4 *)(iVar7 + 0x16);
      pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                      *(ulong *)((int)uVar4 + 0xa));
      param_4 = (ushort)&PTR_LOOP_1050_1038;
      pass1_1038_3608(CONCAT22(param_3,uVar6));
    }
  }
  *(undefined4 *)(iVar7 + 0x1e) = 0x0;
  *(undefined4 *)(iVar7 + 0x1a) = 0x0;
  *(undefined4 *)(iVar7 + 0x16) = 0x0;
  puVar1 = (undefined4 *)*(uint *)(iVar7 + 0xe);
  uVar2 = *(uint *)(iVar7 + 0x10);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_4,puVar1,uVar2,0x1);
  }
  *(undefined4 *)(iVar7 + 0xe) = 0x0;
  puVar1 = (undefined4 *)*(uint *)(iVar7 + 0x12);
  uVar2 = *(uint *)(iVar7 + 0x14);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_4,puVar1,uVar2,0x1);
  }
  *(undefined4 *)(iVar7 + 0x12) = 0x0;
  return;
}



void __stdcall16far pass1_1008_b61a(ulong param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5)

{
  undefined2 uVar1;
  
  pass1_1008_b8fa(param_1,param_2,param_4,param_5);
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(ushort *)((int)param_1 + 0x1a) = param_3;
  *(ushort *)((int)param_1 + 0x1c) = param_4;
  return;
}



void __stdcall16far pass1_1008_b63a(ulong param_1,ulong param_2)

{
  undefined2 in_AX;
  undefined2 in_DX;
  undefined2 uVar1;
  ushort unaff_SS;
  
  pass1_1008_b964(param_1,param_2,unaff_SS);
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x1e) = in_AX;
  *(undefined2 *)((int)param_1 + 0x20) = in_DX;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far load_string_1008_b65a(ULONG param_1,LPSTR in_string_2,ULONG param_3,UINT16 param_4)

{
  ushort unaff_SS;
  
  pass1_1008_b9ce(param_1,CONCAT22(param_4,param_3._2_2_),unaff_SS);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,in_string_2,
             (short)param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far load_str_and_sprintf_1008_b69c(astruct_25 *param_1,WORD *param_2,uchar *param_3)

{
  code **ppcVar1;
  char *in_buffer_4;
  ushort uVar2;
  ushort uVar3;
  ushort uVar4;
  astruct_25 *iVar5;
  undefined2 uVar5;
  astruct_26 *paVar6;
  undefined4 uVar7;
  int iStack516;
  char local_202 [0x100];
  CHAR local_102 [0x100];
  
  in_buffer_4 = local_202;
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,in_buffer_4,
             (short)param_2);
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_25 *)param_1;
  if (iVar5->field_0xa == (undefined4 *)0x0) {
    mem_op_1000_179c(0xc,param_3,0x1000);
    if (((uint)param_3 | (uint)in_buffer_4) == 0x0) {
      paVar6 = (astruct_26 *)0x0;
    }
    else {
      paVar6 = (astruct_26 *)set_struct_1008_574a((astruct_21 *)CONCAT22(param_3,in_buffer_4));
    }
    *(int *)&iVar5->field_0xa = (int)paVar6;
    *(undefined2 *)((int)&iVar5->field_0xa + 0x2) = (int)((ulong)paVar6 >> 0x10);
    for (iStack516 = 0x1; iStack516 < 0x6; iStack516 = iStack516 + 0x1) {
      mem_op_1000_179c(0x12,(uchar *)((ulong)paVar6 >> 0x10),0x1000);
      if (paVar6 == (astruct_26 *)0x0) {
        uVar7 = 0x0;
      }
      else {
        uVar7 = set_stuct_1008_b0bc(paVar6);
      }
      uVar3 = (ushort)((ulong)uVar7 >> 0x10);
      uVar4 = uVar3;
      wsprintf16((LPSTR)&PTR_LOOP_1050_1000,local_102,param_2);
      uVar2 = str_op_1008_60e8((char *)CONCAT22(param_2,local_102),uVar4);
      *(ushort *)((int)uVar7 + 0x4) = uVar2;
      *(ushort *)((int)uVar7 + 0x6) = uVar4;
      ppcVar1 = (code **)((int)*iVar5->field_0xa + 0x8);
      paVar6 = (astruct_26 *)(**ppcVar1)();
    }
    iVar5->field_0x22 = 0x5;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far load_str_and_sprintf_1008_b78a(ULONG param_1,WORD *param_2,uchar *param_3,uint param_4)

{
  int *piVar1;
  code **ppcVar2;
  ushort uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined4 uVar6;
  char local_206 [0x100];
  CHAR local_106 [0x100];
  int iStack6;
  undefined2 uStack4;
  
  mem_op_1000_179c(0x12,param_3,0x1000);
  if (((uint)param_3 | param_4) == 0x0) {
    uVar6 = 0x0;
  }
  else {
    uVar6 = set_stuct_1008_b0bc((astruct_26 *)CONCAT22(param_3,param_4));
  }
  uStack4 = (undefined2)((ulong)uVar6 >> 0x10);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,
             (short)param_2);
  iStack6 = (int)uVar6;
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  piVar1 = (int *)(iVar4 + 0x22);
  *piVar1 = *piVar1 + 0x1;
  wsprintf16((LPSTR)0x1010,local_106,param_2);
  iStack6 = (int)uVar6;
  uVar3 = str_op_1008_60e8((char *)CONCAT22(param_2,local_106),(ushort)((ulong)uVar6 >> 0x10));
  iStack6 = (int)uVar6;
  *(ushort *)(iStack6 + 0x4) = uVar3;
  *(undefined2 *)(iStack6 + 0x6) = (int)((ulong)uVar6 >> 0x10);
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0xa) + 0x8);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,*(undefined4 *)(iVar4 + 0xa),iStack6,uStack4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1008_b820(ulong param_1,int param_2,ushort param_3)

{
  undefined2 uVar1;
  
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
  if (*(int *)(param_2 + 0x152) == 0x0) {
    return 0x0;
  }
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0xc),*(undefined2 *)((int)param_1 + 0xa));
}



void __stdcall16far pass1_1008_b85c(ulong param_1,long param_2)

{
  undefined *puVar1;
  uint extraout_DX;
  undefined2 unaff_SS;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(unaff_SS,local_a),*(ulong *)((int)param_1 + 0xe));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,unaff_SS);
    if ((extraout_DX | (uint)puVar1) == 0x0) {
      return;
    }
  } while (*(long *)(puVar1 + 0x8) != param_2);
  return;
}



long __stdcall16far pass1_1008_b8ac(ulong param_1,int param_2,ushort param_3)

{
  long lVar1;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0x12));
  do {
    lVar1 = pass1_1008_5b12(local_a,param_3);
    if (lVar1 == 0x0) {
      return 0x0;
    }
  } while (*(int *)((int)lVar1 + 0x8) != param_2);
  return lVar1;
}



void __stdcall16far pass1_1008_b8fa(ulong param_1,ulong param_2,ushort param_3,ushort param_4)

{
  undefined *puVar1;
  int iVar2;
  uint extraout_DX;
  undefined local_a [0x8];
  
  if (param_2 == 0x0) {
    return;
  }
  pass1_1008_5784((ulong *)CONCAT22(param_4,local_a),*(ulong *)((int)param_1 + 0xe));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_4);
    if ((extraout_DX | (uint)puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a(*(ulong *)(puVar1 + 0x4),param_2);
  } while (iVar2 != 0x0);
  return;
}



void __stdcall16far pass1_1008_b964(ulong param_1,ulong param_2,ushort param_3)

{
  undefined *puVar1;
  int iVar2;
  uint extraout_DX;
  undefined local_a [0x8];
  
  if (param_2 == 0x0) {
    return;
  }
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0x12));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_3);
    if ((extraout_DX | (uint)puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a(*(ulong *)(puVar1 + 0x4),param_2);
  } while (iVar2 != 0x0);
  return;
}



void __stdcall16far pass1_1008_b9ce(ulong param_1,ulong param_2,ushort param_3)

{
  undefined *puVar1;
  int iVar2;
  uint extraout_DX;
  undefined local_a [0x8];
  
  if (param_2 == 0x0) {
    return;
  }
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0xa));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_3);
    if ((extraout_DX | (uint)puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a(*(ulong *)(puVar1 + 0x4),param_2);
  } while (iVar2 != 0x0);
  return;
}



void __stdcall16far pass1_1008_ba38(ulong param_1,ulong param_2,HFILE16 param_3,uint16_t param_4)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  undefined *puVar3;
  uint extraout_DX;
  int iVar4;
  undefined2 uVar5;
  ushort uVar6;
  ushort uVar7;
  undefined4 local_2a [0x3];
  undefined2 local_1e [0x5];
  undefined local_14 [0x8];
  undefined2 local_c;
  undefined4 uStack10;
  undefined2 local_6 [0x2];
  
  BVar2 = write_to_file_1008_7cac(param_2,param_4);
  if (BVar2 != 0x0) {
    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    local_c = *(undefined2 *)(iVar4 + 0x22);
    uVar6 = (ushort)param_2;
    uVar7 = (ushort)(param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)&local_c,param_4,(char *)0x2,param_3);
    if (BVar2 != 0x0) {
      if (*(long *)(iVar4 + 0xa) == 0x0) {
        local_c = 0x0;
      }
      else {
        uVar1 = *(undefined4 *)(iVar4 + 0xa);
        local_c = *(undefined2 *)((int)uVar1 + 0x8);
      }
      local_1e[0] = local_c;
      BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_1e,param_4,(char *)0x2,param_3);
      if (BVar2 != 0x0) {
        pass1_1008_5784((ulong *)CONCAT22(param_4,local_14),*(ulong *)(iVar4 + 0xa));
        do {
          puVar3 = local_14;
          pass1_1008_5b12(puVar3,param_4);
          uStack10 = CONCAT22(extraout_DX,puVar3);
          if ((extraout_DX | (uint)puVar3) == 0x0) {
            return;
          }
          BVar2 = pass1_1008_7c2a(param_2,*(char **)(puVar3 + 0x4),param_3);
          if (BVar2 == 0x0) break;
          local_6[0] = *(undefined2 *)((int)uStack10 + 0x8);
          BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_6,param_4,(char *)0x2,param_3);
          if (BVar2 == 0x0) break;
          local_2a[0] = *(undefined4 *)((int)uStack10 + 0xa);
          BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_2a,param_4,(char *)0x4,param_3);
          if (BVar2 == 0x0) break;
          local_6[0] = *(undefined2 *)((int)uStack10 + 0xe);
          BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_6,param_4,(char *)0x2,param_3);
        } while (BVar2 != 0x0);
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



void __stdcall16far
file_1008_bb5e(ulong param_1,ulong param_2,int param_3,uchar *param_4,uint16_t param_5,uint16_t param_6)

{
  code **ppcVar1;
  ushort uVar2;
  astruct_199 *iVar3;
  BOOL16 BVar3;
  uint uVar5;
  astruct_200 *uVar4;
  undefined *puVar6;
  ushort uVar7;
  uchar *extraout_DX;
  uchar *puVar8;
  ushort uVar9;
  ushort uVar10;
  uchar *extraout_DX_00;
  ushort extraout_DX_01;
  ushort uVar11;
  ushort uVar12;
  undefined2 uVar13;
  undefined2 uVar14;
  astruct_200 *paStack286;
  undefined4 *puStack284;
  undefined local_118 [0x100];
  undefined2 local_18 [0x2];
  undefined2 local_14 [0x2];
  astruct_200 *local_10 [0x4];
  undefined4 local_8;
  
  if ((int)PTR_LOOP_1050_0312 < 0x2) {
    return;
  }
  uVar11 = (ushort)param_2;
  uVar12 = (ushort)(param_2 >> 0x10);
  read_file_1008_7cfe(uVar11,uVar12,0x16,param_5,param_6);
  if (param_3 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d4;
  }
  else {
    iVar3 = (astruct_199 *)param_1;
    iVar3 = (astruct_199 *)&iVar3->field_0x22;
    uVar2 = (ushort)(param_1 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar11,uVar12,(ushort)iVar3,0x0,uVar2,0x2,param_5);
    if ((BVar3 != 0x0) &&
       (uVar5 = read_file_1008_7dee(uVar11,uVar12,(ushort)local_10,0x0,param_6,0x2,param_5), uVar5 != 0x0)) {
      if (local_10[0] == (astruct_200 *)0x0) {
        return;
      }
      uVar14 = 0xc;
      mem_op_1000_179c(0xc,param_4,0x1000);
      if (((uint)param_4 | uVar5) == 0x0) {
        uVar5 = 0x0;
        puVar8 = (uchar *)0x0;
      }
      else {
        set_struct_1008_574a((astruct_21 *)CONCAT22(param_4,uVar5));
        puVar8 = extraout_DX;
      }
      *(uint *)&iVar3->field_0xa = uVar5;
      *(uchar **)((int)&iVar3->field_0xa + 0x2) = puVar8;
      paStack286 = (astruct_200 *)0x0;
      while( true ) {
        if (local_10[0] <= paStack286) {
          return;
        }
        uVar13 = 0x12;
        uVar4 = local_10[0];
        mem_op_1000_179c(0x12,puVar8,0x1000);
        if (((uint)puVar8 | (uint)uVar4) == 0x0) {
          uVar4 = (astruct_200 *)0x0;
          uVar9 = 0x0;
        }
        else {
          set_stuct_1008_b0bc((astruct_26 *)CONCAT22(puVar8,uVar4));
          uVar9 = extraout_DX_01;
        }
        puStack284 = (undefined4 *)CONCAT22(uVar9,uVar4);
        puVar6 = local_118;
        uVar10 = uVar9;
        read_file_1008_7c6e(uVar11,uVar12,(char *)CONCAT22(param_6,puVar6),0x1000);
        if ((((puVar6 == (undefined *)0x0) ||
             (BVar3 = read_file_1008_7dee(uVar11,uVar12,(ushort)local_14,0x0,param_6,0x2,0x1000), BVar3 == 0x0)) ||
            (BVar3 = read_file_1008_7dee(uVar11,uVar12,(ushort)&local_8,0x0,param_6,0x4,0x1000), BVar3 == 0x0)) ||
           (BVar3 = read_file_1008_7dee(uVar11,uVar12,(ushort)local_18,0x0,param_6,0x2,0x1000), BVar3 == 0x0)) break;
        uVar7 = str_op_1008_60e8((char *)CONCAT22(param_6,local_118),uVar10);
        uVar4->field_0x4 = uVar7;
        uVar4->field_0x6 = uVar10;
        uVar4->field_0x8 = local_14[0];
        uVar4->field_0xa = local_8;
        uVar4->field_0xe = local_18[0];
        ppcVar1 = (code **)((int)*iVar3->field_0xa + 0x8);
        (**ppcVar1)();
        paStack286 = (astruct_200 *)&paStack286->field_0x1;
        puVar8 = extraout_DX_00;
      }
      if (puStack284 != (undefined4 *)0x0) {
        ppcVar1 = (code **)*puStack284;
        (**ppcVar1)(0x1000,uVar4,uVar9,0x1,uVar13,uVar14,puStack284);
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



ulong __stdcall16far pass1_1008_bd02(ulong param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1008_afde((ushort *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_bd28(ushort *param_1,byte param_2)

{
  pass1_1008_b08c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_18 * __stdcall16far pass1_1008_bd4e(astruct_18 *param_1,byte param_2)

{
  pass1_1008_b08c(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_18 * __stdcall16far pass1_1008_bd74(astruct_18 *param_1,byte param_2)

{
  pass1_1008_b08c(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_18 * __stdcall16far pass1_1008_bd9a(astruct_18 *param_1,byte param_2)

{
  pass1_1008_b08c(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1008_bde0(ulong *param_1,uchar *param_2)

{
  ushort uVar1;
  astruct_139 *iVar2;
  astruct_140 *iVar3;
  astruct_141 *iVar4;
  astruct_142 *iVar5;
  astruct_143 *iVar6;
  astruct_144 *iVar7;
  astruct_145 *iVar8;
  astruct_146 *iVar9;
  astruct_147 *iVar10;
  astruct_148 *iVar11;
  astruct_149 *iVar12;
  astruct_150 *iVar2_00;
  astruct_151 *iVar2_01;
  astruct_152 *iVar2_02;
  astruct_153 *iVar2_03;
  astruct_154 *iVar2_04;
  astruct_155 *iVar2_05;
  int iVar2_06;
  ushort uVar3;
  undefined2 uVar13;
  
  _PTR_LOOP_1050_06e0 = param_1;
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_2,0x1000);
    PTR_LOOP_1050_5f2e = param_2;
  }
  else {
  }
  uVar1 = fn_ptr_op_1000_1708(0x1aa,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  *(ushort *)param_1 = uVar1;
  *(uchar **)((int)param_1 + 0x2) = PTR_LOOP_1050_5f2e;
  uVar3 = (ushort)(*param_1 >> 0x10);
  iVar2 = (astruct_139 *)*param_1;
  iVar2->field_0x6 = 0x6e4;
  iVar2->field_0x8 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xa) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar3 = (astruct_140 *)*param_1;
  iVar3->field_0xc = 0x6ea;
  iVar3->field_0xe = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x10) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar4 = (astruct_141 *)*param_1;
  iVar4->field_0x12 = 0x6ee;
  iVar4->field_0x14 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x16) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar5 = (astruct_142 *)*param_1;
  iVar5->field_0x18 = 0x6f2;
  iVar5->field_0x1a = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x1c) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar6 = (astruct_143 *)*param_1;
  iVar6->field_0x1e = 0x6f6;
  iVar6->field_0x20 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x22) = 0x4;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar7 = (astruct_144 *)*param_1;
  iVar7->field_0x24 = 0x6fe;
  iVar7->field_0x26 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x28) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar8 = (astruct_145 *)*param_1;
  iVar8->field_0x2a = 0x702;
  iVar8->field_0x2c = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x2e) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar9 = (astruct_146 *)*param_1;
  iVar9->field_0x30 = 0x708;
  iVar9->field_0x32 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x34) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar10 = (astruct_147 *)*param_1;
  iVar10->field_0x36 = 0x70e;
  iVar10->field_0x38 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x3a) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar11 = (astruct_148 *)*param_1;
  iVar11->field_0x3c = 0x714;
  iVar11->field_0x3e = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x40) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar12 = (astruct_149 *)*param_1;
  iVar12->field_0x42 = 0x71a;
  iVar12->field_0x44 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x46) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_00 = (astruct_150 *)*param_1;
  iVar2_00->field_0x48 = 0x71e;
  iVar2_00->field_0x4a = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x4c) = 0x7;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_01 = (astruct_151 *)*param_1;
  iVar2_01->field_0x4e = 0x72c;
  iVar2_01->field_0x50 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x52) = 0x6;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_02 = (astruct_152 *)*param_1;
  iVar2_02->field_0x54 = 0x738;
  iVar2_02->field_0x56 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x58) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_03 = (astruct_153 *)*param_1;
  iVar2_03->field_0x5a = 0x73e;
  iVar2_03->field_0x5c = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x5e) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_04 = (astruct_154 *)*param_1;
  iVar2_04->field_0x60 = 0x744;
  iVar2_04->field_0x62 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x64) = 0x4;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_05 = (astruct_155 *)*param_1;
  iVar2_05->field_0x66 = 0x74c;
  iVar2_05->field_0x68 = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x6a) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x6c) = 0x750;
  *(undefined2 *)(iVar2_06 + 0x6e) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x70) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x72) = 0x756;
  *(undefined2 *)(iVar2_06 + 0x74) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x76) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x78) = 0x75a;
  *(undefined2 *)(iVar2_06 + 0x7a) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x7c) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x7e) = 0x75e;
  *(undefined2 *)(iVar2_06 + 0x80) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x82) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x84) = 0x764;
  *(undefined2 *)(iVar2_06 + 0x86) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x88) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x8a) = 0x76a;
  *(undefined2 *)(iVar2_06 + 0x8c) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x8e) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x90) = 0x770;
  *(undefined2 *)(iVar2_06 + 0x92) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x94) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x96) = 0x774;
  *(undefined2 *)(iVar2_06 + 0x98) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x9a) = 0x4;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x9c) = 0x77c;
  *(undefined2 *)(iVar2_06 + 0x9e) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xa0) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xa2) = 0x780;
  *(undefined2 *)(iVar2_06 + 0xa4) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xa6) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xa8) = 0x782;
  *(undefined2 *)(iVar2_06 + 0xaa) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xac) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xae) = 0x786;
  *(undefined2 *)(iVar2_06 + 0xb0) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xb2) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xb4) = 0x78a;
  *(undefined2 *)(iVar2_06 + 0xb6) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xb8) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xba) = 0x78e;
  *(undefined2 *)(iVar2_06 + 0xbc) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xbe) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xc0) = 0x792;
  *(undefined2 *)(iVar2_06 + 0xc2) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xc4) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xc6) = 0x796;
  *(undefined2 *)(iVar2_06 + 0xc8) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xca) = 0x4;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xcc) = 0x79e;
  *(undefined2 *)(iVar2_06 + 0xce) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xd0) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xd2) = 0x7a0;
  *(undefined2 *)(iVar2_06 + 0xd4) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xd6) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xd8) = 0x7a4;
  *(undefined2 *)(iVar2_06 + 0xda) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xdc) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xde) = 0x7a6;
  *(undefined2 *)(iVar2_06 + 0xe0) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xe2) = 0x6;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xe4) = 0x7b2;
  *(undefined2 *)(iVar2_06 + 0xe6) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xe8) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xea) = 0x7b4;
  *(undefined2 *)(iVar2_06 + 0xec) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xee) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xf0) = 0x7ba;
  *(undefined2 *)(iVar2_06 + 0xf2) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xf4) = 0x2d;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xf6) = 0x814;
  *(undefined2 *)(iVar2_06 + 0xf8) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0xfa) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0xfc) = 0x81a;
  *(undefined2 *)(iVar2_06 + 0xfe) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x100) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x102) = 0x81c;
  *(undefined2 *)(iVar2_06 + 0x104) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x106) = 0x4b;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x108) = 0x8b2;
  *(undefined2 *)(iVar2_06 + 0x10a) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x10c) = 0x6;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x10e) = 0x8be;
  *(undefined2 *)(iVar2_06 + 0x110) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x112) = 0x4;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x11a) = 0x8c6;
  *(undefined2 *)(iVar2_06 + 0x11c) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x11e) = 0x35;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x120) = 0x930;
  *(undefined2 *)(iVar2_06 + 0x122) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x124) = 0x2e;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x114) = 0x98c;
  *(undefined2 *)(iVar2_06 + 0x116) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x118) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x126) = 0x98e;
  *(undefined2 *)(iVar2_06 + 0x128) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x12a) = 0x9;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x12c) = 0x9a0;
  *(undefined2 *)(iVar2_06 + 0x12e) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x130) = 0x1a;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x132) = 0x9d4;
  *(undefined2 *)(iVar2_06 + 0x134) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x136) = 0x8;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x138) = 0x9e4;
  *(undefined2 *)(iVar2_06 + 0x13a) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x13c) = 0x4a;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x144) = 0xa78;
  *(undefined2 *)(iVar2_06 + 0x146) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x148) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x14a) = 0xa7c;
  *(undefined2 *)(iVar2_06 + 0x14c) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x14e) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x150) = 0xa7e;
  *(undefined2 *)(iVar2_06 + 0x152) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x154) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x156) = 0xa80;
  *(undefined2 *)(iVar2_06 + 0x158) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x15a) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x15c) = 0xa86;
  *(undefined2 *)(iVar2_06 + 0x15e) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x160) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x168) = 0xa8a;
  *(undefined2 *)(iVar2_06 + 0x16a) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x16c) = 0x1b;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x16e) = 0xac0;
  *(undefined2 *)(iVar2_06 + 0x170) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x172) = 0x16;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x174) = 0xaec;
  *(undefined2 *)(iVar2_06 + 0x176) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x178) = 0x3e;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x17a) = 0xb68;
  *(undefined2 *)(iVar2_06 + 0x17c) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x17e) = 0x46;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x180) = 0xbf4;
  *(undefined2 *)(iVar2_06 + 0x182) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x184) = 0x1;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x186) = 0xbf6;
  *(undefined2 *)(iVar2_06 + 0x188) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x18a) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x18c) = 0xbfc;
  *(undefined2 *)(iVar2_06 + 0x18e) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x190) = 0x3;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x192) = 0xc02;
  *(undefined2 *)(iVar2_06 + 0x194) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x196) = 0xa;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x198) = 0xc16;
  *(undefined2 *)(iVar2_06 + 0x19a) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x19c) = 0x24;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x19e) = 0xc5e;
  *(undefined2 *)(iVar2_06 + 0x1a0) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x1a2) = 0x2;
  uVar13 = (undefined2)(*param_1 >> 0x10);
  iVar2_06 = (int)*param_1;
  *(undefined2 *)(iVar2_06 + 0x1a4) = 0xc62;
  *(undefined2 *)(iVar2_06 + 0x1a6) = (int)&USHORT_1050_1050;
  *(undefined2 *)((int)*param_1 + 0x1a8) = 0x44;
  return;
}
