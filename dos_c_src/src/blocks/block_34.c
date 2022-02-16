
void __stdcall16far pass1_1010_988c(ulong param_1,int param_2)

{
  code **ppcVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  undefined2 uVar5;
  int iVar6;
  undefined2 uVar7;
  undefined2 unaff_SS;
  long lVar8;
  undefined local_a [0x8];
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  pass1_1008_5784((ulong *)CONCAT22(unaff_SS,local_a),*(ulong *)(iVar6 + 0xe));
  do {
    lVar8 = pass1_1008_5b12(local_a,unaff_SS);
    uVar5 = (undefined2)((ulong)lVar8 >> 0x10);
    iVar3 = (int)lVar8;
    if (lVar8 == 0x0) {
      return;
    }
  } while (*(int *)(iVar3 + 0x4) != param_2);
  iVar4 = *(int *)(iVar3 + 0x6) + -0x1;
  *(int *)(iVar3 + 0x6) = iVar4;
  if ((iVar4 < 0x1) &&
     (ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xe) + 0xc),
     (**ppcVar1)(0x1008,*(undefined4 *)(iVar6 + 0xe),lVar8), uVar2 = *(undefined4 *)(iVar6 + 0xe),
     *(int *)((int)uVar2 + 0x8) == 0x0)) {
    *(undefined2 *)(iVar6 + 0x16) = 0x1;
  }
  return;
}



void __stdcall16far pass1_1010_9f72(ulong param_1,int param_2,ushort param_3)

{
  ushort uVar1;
  
  uVar1 = (ushort)(param_1 >> 0x10);
  pass1_1010_9fa6((ushort)param_1,uVar1,*(ulong *)((ushort)param_1 + 0xe),param_2,param_3);
  return;
}



void __stdcall16far pass1_1010_9f8c(ulong param_1,int param_2,ushort param_3)

{
  ushort uVar1;
  
  uVar1 = (ushort)(param_1 >> 0x10);
  pass1_1010_9fa6((ushort)param_1,uVar1,*(ulong *)((ushort)param_1 + 0xa),param_2,param_3);
  return;
}



undefined2 __stdcall16far pass1_1010_9fa6(ushort param_1,ushort param_2,ulong param_3,int param_4,ushort param_5)

{
  undefined2 uVar1;
  long lVar2;
  undefined local_a [0x8];
  
  if (param_3 != 0x0) {
    pass1_1008_5784((ulong *)CONCAT22(param_5,local_a),param_3);
    while( true ) {
      lVar2 = pass1_1008_5b12(local_a,param_5);
      uVar1 = (undefined2)((ulong)lVar2 >> 0x10);
      if (lVar2 == 0x0) break;
      if (*(int *)((int)lVar2 + 0x4) == param_4) {
        return *(undefined2 *)((int)lVar2 + 0x6);
      }
    }
  }
  return 0x0;
}



void __stdcall16far pass1_1010_9fee(ulong param_1,ushort param_2,ushort param_3,uint param_4,uchar *param_5)

{
  code **ppcVar1;
  uchar *puVar2;
  uchar *extraout_DX;
  astruct_252 *iVar3;
  astruct_253 *iVar4;
  undefined2 uVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  undefined2 *puStack10;
  undefined2 *puStack6;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_252 *)param_1;
  puVar2 = param_5;
  if (iVar3->field_0x12 == (undefined4 *)0x0) {
    mem_op_1000_179c(0xc,param_5,0x1000);
    puVar2 = (uchar *)((uint)param_5 | param_4);
    if (puVar2 == (uchar *)0x0) {
      iVar3->field_0x12 = (undefined4 *)0x0;
    }
    else {
      set_struct_1008_574a((astruct_21 *)CONCAT22(param_5,param_4));
      *(uint *)&iVar3->field_0x12 = param_4;
      *(uchar **)((int)&iVar3->field_0x12 + 0x2) = extraout_DX;
      puVar2 = extraout_DX;
    }
  }
  uVar5 = 0x8;
  mem_op_1000_179c(0x8,puVar2,0x1000);
  puStack10 = (undefined2 *)CONCAT22(puVar2,param_4);
  if (((uint)puVar2 | param_4) == 0x0) {
    puStack6 = (undefined2 *)0x0;
  }
  else {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_4 + 0x2) = 0x1008;
    *puStack10 = 0xa1c4;
    *(undefined2 *)(param_4 + 0x2) = 0x1010;
    puStack6 = puStack10;
  }
  uVar4 = (undefined2)((ulong)puStack6 >> 0x10);
  iVar4 = (astruct_253 *)puStack6;
  iVar4->field_0x4 = param_3;
  iVar4->field_0x6 = param_2;
  ppcVar1 = (code **)((int)*iVar3->field_0x12 + 0x4);
  (**ppcVar1)(0x1000,iVar3->field_0x12,iVar4,uVar4,uVar5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_a0a0(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  ushort uVar5;
  undefined2 uVar6;
  bool bVar7;
  bool bVar8;
  long lVar9;
  int iStack12;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_4,local_a),*(ulong *)((int)param_1 + 0xa));
  iStack12 = 0x4;
  mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_2,param_3);
  if ((PTR_LOOP_1050_13ae != (undefined *)&PTR_LOOP_1050_0002) &&
     (PTR_LOOP_1050_13ae != (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1))) {
    iStack12 = 0x2;
  }
  do {
    while( true ) {
      lVar9 = pass1_1008_5b12(local_a,param_4);
      uVar6 = (undefined2)((ulong)lVar9 >> 0x10);
      iVar4 = (int)lVar9;
      if (lVar9 == 0x0) {
        return;
      }
      iVar2 = *(int *)(iVar4 + 0x4);
      if (iVar2 != 0x12) break;
      piVar1 = (int *)(iVar4 + 0x6);
      bVar8 = SBORROW2(*piVar1,0x2);
      iVar3 = *piVar1 + -0x2;
      bVar7 = iVar3 == 0x0;
LAB_1010_a151:
      if (!bVar7 && bVar8 == iVar3 < 0x0) {
LAB_1010_a153:
        piVar1 = (int *)(iVar4 + 0x6);
        *piVar1 = *piVar1 - *(int *)(iVar4 + 0x6) / iStack12;
      }
    }
    if (((iVar2 != 0x3e) && (iVar2 != 0x41)) && (iVar2 != 0x80)) {
      if (iVar2 == 0x83) {
        piVar1 = (int *)(iVar4 + 0x6);
        bVar8 = SBORROW2(*piVar1,0x1);
        iVar2 = *piVar1;
        iVar3 = iVar2 + -0x1;
        bVar7 = iVar2 == 0x1;
        goto LAB_1010_a151;
      }
      goto LAB_1010_a153;
    }
    iVar2 = *(int *)(iVar4 + 0x6);
    uVar5 = iVar2 / 0x2;
    piVar1 = (int *)(iVar4 + 0x6);
    *piVar1 = *piVar1 - uVar5;
    if (uVar5 == 0x0) {
      uVar5 = 0x1;
    }
    pass1_1010_9fee(param_1,uVar5,*(ushort *)(iVar4 + 0x4),uVar5,iVar2 >> 0xf);
  } while( true );
}



ushort * __stdcall16far pass1_1010_a172(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_95f8(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1010_a198(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1010_a1d8(astruct_627 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int iVar1;
  code **ppcVar2;
  int unaff_DI;
  astruct_79 *paVar3;
  ushort *puVar4;
  uint uStack4;
  
  paVar3 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x389a;
  param_1->field_0xc = 0x1008;
  param_1->field_0xa = 0x3aa8;
  param_1->field_0xc = 0x1008;
  param_1->field_0x138 = (undefined4 *)0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xe9cc;
  param_1->field_0x2 = 0x1010;
  param_1->field_0xa = 0xe9dc;
  param_1->field_0xc = 0x1010;
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,(uchar *)((ulong)paVar3 >> 0x10),unaff_DI);
  *(int *)&param_1->field_0x138 = (int)puVar4;
  *(undefined2 *)((int)&param_1->field_0x138 + 0x2) = (int)((ulong)puVar4 >> 0x10);
  ppcVar2 = (code **)((int)*param_1->field_0x138 + 0x4);
  (**ppcVar2)();
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0xa4),(WNDCLASS16 *)0x0,0x94);
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0xe),(WNDCLASS16 *)0x0,0x96);
  uStack4 = 0x0;
  do {
    iVar1 = &param_1->field_0x0 + uStack4 * 0x6;
    *(code **)(iVar1 + 0xe) = pass1_1010_c7e2;
    *(undefined2 *)(iVar1 + 0x12) = 0x0;
    uStack4 = uStack4 + 0x1;
  } while (uStack4 < 0x19);
  param_1->field_0x4a = pass1_1010_c864;
  param_1->field_0x4e = 0x0;
  param_1->field_0x50 = pass1_1010_cc56;
  param_1->field_0x54 = 0x0;
  param_1->field_0x56 = pass1_1010_cf36;
  param_1->field_0x5a = 0x0;
  param_1->field_0x2c = pass1_1010_d24a;
  param_1->field_0x30 = 0x0;
  param_1->field_0x6e = pass1_1010_d448;
  param_1->field_0x72 = 0x0;
  param_1->field_0x74 = pass1_1010_d5ae;
  param_1->field_0x78 = 0x0;
  param_1->field_0x98 = pass1_1010_d710;
  param_1->field_0x9c = 0x0;
  return;
}



void __stdcall16far pass1_1010_a478(ushort *param_1,ushort param_2)

{
  undefined2 *puVar1;
  undefined2 uVar2;
  astruct_497 *uVar3;
  undefined2 uVar4;
  undefined2 *puStack6;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  uVar3 = (astruct_497 *)param_1;
  *param_1 = 0xe9cc;
  uVar3->field_0x2 = 0x1010;
  uVar3->field_0xa = 0xe9dc;
  uVar3->field_0xc = 0x1010;
  if (uVar3->field_0x138 != 0x0) {
    if (param_1 == (ushort *)0x0) {
      puVar1 = (undefined2 *)0x0;
      uVar2 = 0x0;
    }
    else {
      puVar1 = &uVar3->field_0xa;
      uVar2 = uVar4;
    }
    pass1_1010_1ea6(uVar3->field_0x138,CONCAT22(uVar2,puVar1),param_2);
  }
  uVar3->field_0x138 = 0x0;
  if (param_1 == (ushort *)0x0) {
    puVar1 = (undefined2 *)0x0;
    uVar4 = 0x0;
  }
  else {
    puVar1 = &uVar3->field_0xa;
  }
  puStack6 = (undefined2 *)CONCAT22(uVar4,puVar1);
  *puStack6 = 0x389a;
  puVar1[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1010_a50c(astruct_20 *param_1,ulong param_2,ulong param_3)

{
  int iVar1;
  astruct_260 *iVar2;
  undefined4 local_8;
  int iStack4;
  
  iVar1 = (int)param_1;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0xa4)),(WNDCLASS16 *)0x0,0x94);
  iVar2 = (astruct_260 *)(*(int *)((int)param_3 + 0xa) * 0x6 + iVar1);
  local_8 = iVar2->field_0xe;
  iStack4 = iVar2->field_0x12;
  (*(code *)local_8)(0x1000,iVar1 + iStack4,param_1._2_2_,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_a568(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
  pass1_1030_2622(CONCAT22(param_5,param_4),param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_a58a(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
  pass1_1030_266c(param_4,CONCAT22(param_3,param_5));
  return;
}



ushort __stdcall16far pass1_1010_a5ac(ushort param_1,ushort param_2,ulong param_3)

{
  ulong uVar1;
  
  uVar1 = struct_op_1030_73a8(param_3);
  return *(ushort *)((int)uVar1 + 0x20);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_a5ca(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
  pass1_1030_2242(CONCAT22(param_5,param_4),param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_a5ec(ushort param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5)

{
  code **ppcVar1;
  uint uVar2;
  ushort uVar3;
  ushort uVar4;
  ushort uVar5;
  ushort extraout_DX;
  undefined4 *puVar6;
  ulong uStack6;
  
  uVar2 = param_4._2_2_ | (uint)param_4;
  if (param_4 != 0x0) {
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
    uStack6 = CONCAT22(param_5,uVar2);
    puVar6 = (undefined4 *)struct_op_1030_73a8(param_4);
    uVar5 = (ushort)((ulong)puVar6 >> 0x10);
    uVar4 = *(ushort *)((int)puVar6 + 0x20);
    if (uVar4 != param_3) {
      uVar3 = param_3;
      pass1_1010_a5ca(param_1,param_2,uVar4,param_3,uVar5);
      if ((uVar4 != 0x70) && ((int)uVar3 < 0x0)) {
        pass1_1030_25d8(CONCAT22(param_5,uVar2),uVar3 + 0x1,uVar4);
      }
      ppcVar1 = (code **)((int)*puVar6 + 0x8);
      uVar4 = param_3;
      (**ppcVar1)();
      if (param_3 != 0x70) {
        pass1_1010_a5ca(param_1,param_2,param_3,uVar4,extraout_DX);
        if ((int)uVar4 < 0x0) {
          pass1_1030_25d8(uStack6,uVar4 - 0x1,param_3);
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_a69c(ulong param_1,uint param_2,int param_3,uchar *param_4,int param_5,ushort param_6)

{
  int iVar1;
  ushort uVar2;
  uchar *puVar3;
  uchar *puVar4;
  astruct_25 *paVar5;
  astruct_67 *paVar6;
  ushort *puVar7;
  ushort uVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  ushort uStack22;
  int iStack20;
  
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
  if (param_2 == 0x1) {
    puVar3 = param_4;
    for (iStack20 = 0x0; iStack20 < 0x83; iStack20 = iStack20 + 0x1) {
      iVar1 = pass1_1030_2242(CONCAT22(param_4,param_3),iStack20);
      if (0x19 < iVar1) {
        uStack22 = iVar1 - 0x5;
        if ((int)uStack22 < 0x19) {
          uStack22 = 0x19;
        }
        pass1_1030_25d8(CONCAT22(param_4,param_3),uStack22,iStack20);
      }
    }
    goto switchD_1010_aaef_caseD_b;
  }
  puVar3 = param_4;
  pass1_1030_25f0(CONCAT22(param_4,param_3),0x0,param_2);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,param_6,puVar3,param_5);
  puVar3 = (uchar *)((ulong)puVar7 >> 0x10);
  uVar2 = (ushort)param_1;
  uVar8 = (ushort)(param_1 >> 0x10);
  puVar4 = puVar3;
  switch(param_2) {
  case 0xa:
  case 0xc:
    iVar1 = 0x1b;
    break;
  default:
    goto switchD_1010_aaef_caseD_b;
  case 0x10:
    pass1_1010_682e((ulong)puVar7,0x1,0x2d);
    if (*(int *)(param_3 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x2d;
    goto LAB_1010_a91f;
  case 0x12:
    pass1_1010_682e((ulong)puVar7,0x1,0x16);
    pass1_1010_682e((ulong)puVar7,0x1,0x17);
    pass1_1010_682e((ulong)puVar7,0x1,0x18);
    pass1_1010_682e((ulong)puVar7,0x1,0x40);
    iVar1 = 0x3f;
    goto LAB_1010_a96c;
  case 0x13:
    iVar1 = 0x35;
    goto LAB_1010_a91f;
  case 0x19:
    goto switchD_1010_aaef_caseD_19;
  case 0x1a:
    iVar1 = 0xf;
    goto LAB_1010_a96c;
  case 0x1c:
    iVar1 = 0x11;
    goto LAB_1010_a96c;
  case 0x1d:
  case 0x24:
    pass1_1010_abd2(uVar2,uVar8,0x1e,puVar3,param_5,param_6);
    iVar1 = 0x5b;
    goto LAB_1010_a91f;
  case 0x1e:
    uVar2 = 0x1;
    iVar1 = 0x2;
    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_6,puVar3,param_5);
    puVar3 = (uchar *)((ulong)puVar7 >> 0x10);
    pass1_1010_08c0((ulong)puVar7,uVar2,iVar1,param_6);
    paVar5 = (astruct_25 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x40,param_6,puVar3,param_5);
    puVar3 = (uchar *)((ulong)paVar5 >> 0x10);
    load_str_and_sprintf_1008_b69c(paVar5,param_6,puVar3);
    goto switchD_1010_aaef_caseD_b;
  case 0x22:
    iVar1 = 0x8;
    goto LAB_1010_aabe;
  case 0x23:
    iVar1 = 0xc;
    goto LAB_1010_aabe;
  case 0x25:
    pass1_1010_abd2(uVar2,uVar8,0x14,puVar3,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x1b,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x1e,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x22,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x25,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x28,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x2a,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x2d,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x2f,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x31,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x35,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x38,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x3a,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x3c,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x48,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x4f,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x52,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x54,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x57,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x5b,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x5d,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x62,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x66,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x68,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x6c,puVar4,param_5,param_6);
    goto switchD_1010_aaef_caseD_19;
  case 0x29:
    iVar1 = 0x25;
    break;
  case 0x2a:
    iVar1 = 0xf;
    goto LAB_1010_aabe;
  case 0x2b:
    iVar1 = 0x6e;
    goto LAB_1010_a96c;
  case 0x30:
    iVar1 = 0x54;
    break;
  case 0x33:
    pass1_1010_abd2(uVar2,uVar8,0x31,puVar3,param_5,param_6);
    iVar1 = 0x6c;
    goto LAB_1010_a91f;
  case 0x36:
    iVar1 = 0x13;
    goto LAB_1010_aabe;
  case 0x37:
    iVar1 = 0x2c;
LAB_1010_a96c:
    pass1_1010_682e((ulong)puVar7,0x1,iVar1);
    goto switchD_1010_aaef_caseD_b;
  case 0x38:
    pass1_1010_682e((ulong)puVar7,0x1,0x28);
    if (*(int *)(param_3 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x28;
    goto LAB_1010_a91f;
  case 0x39:
    iVar1 = 0x10;
    goto LAB_1010_aabe;
  case 0x3a:
    iVar1 = 0x11;
    goto LAB_1010_aabe;
  case 0x3b:
    iVar1 = 0x12;
LAB_1010_aabe:
    pass1_1010_6814((ulong)puVar7,0x1,iVar1);
    goto switchD_1010_aaef_caseD_b;
  case 0x3c:
    pass1_1010_abd2(uVar2,uVar8,0x14,puVar3,param_5,param_6);
    iVar1 = 0x62;
    goto LAB_1010_a91f;
  case 0x3d:
    pass1_1010_682e((ulong)puVar7,0x1,0x66);
    if (*(int *)(param_3 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x66;
LAB_1010_a91f:
    pass1_1010_abd2(uVar2,uVar8,iVar1,puVar3,param_5,param_6);
    goto switchD_1010_aaef_caseD_b;
  case 0x3e:
    iVar1 = 0x5d;
    break;
  case 0x3f:
    iVar1 = 0x22;
    break;
  case 0x40:
    iVar1 = 0x57;
    break;
  case 0x41:
    iVar1 = 0x4f;
  }
  pass1_1010_abd2(uVar2,uVar8,iVar1,puVar3,param_5,param_6);
switchD_1010_aaef_caseD_b:
  paVar6 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_6,puVar3,param_5);
  puVar3 = (uchar *)((ulong)paVar6 >> 0x10);
  uVar2 = pass1_1008_ab12((ushort)paVar6,(ushort)puVar3,param_2);
  if (uVar2 != 0x0) {
    post_win_msg_1008_a0e4(paVar6,0x0,0x0,0x1,0x0,uVar2,0x1008,param_6);
  }
  post_win_msg_1008_a0e4(paVar6,0x0,0x0,0x1,0x0,0x3d,0x1008,param_6);
  uVar10 = 0x400;
  iVar1 = 0x6;
  uVar9 = 0x1;
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_6,puVar3,param_5);
  pass1_1010_043a((ulong)puVar7,CONCAT22(uVar10,uVar9),iVar1,param_6);
  return;
switchD_1010_aaef_caseD_19:
  *(undefined2 *)((int)puVar7 + 0x148) = 0x34;
  puVar3 = puVar4;
  goto switchD_1010_aaef_caseD_b;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl16far pass1_1010_abd2(ushort param_1,ushort param_2,int param_3,uchar *param_4,int param_5,ushort param_6)

{
  bool bVar1;
  int *piVar2;
  ushort *puVar3;
  int iStack20;
  int iStack16;
  int iStack14;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,param_6,param_4,param_5);
  bVar1 = false;
  iStack16 = param_3;
  _iStack20 = (int *)CONCAT22(param_6,&stack0x000a);
  while( true ) {
    piVar2 = _iStack20;
    if (iStack16 == 0x0) {
      return;
    }
    if (bVar1) break;
    if (*(int *)(iStack16 * 0x2 + (int)puVar3 + 0xa) != 0x0) {
      bVar1 = true;
      iStack14 = iStack16;
    }
    _iStack20 = (int *)((ulong)_iStack20 & 0xffff0000 | (ulong)(iStack20 + 0x2));
    iStack16 = *piVar2;
  }
  pass1_1010_682e((ulong)puVar3,0x0,iStack14);
  pass1_1010_682e((ulong)puVar3,0x1,iStack16);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1010_ac62(ushort param_1,ushort param_2,int param_3,ushort param_4,ushort param_5)

{
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
  return *(ushort *)(param_4 + 0x116 + param_3 * 0x2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * __stdcall16far load_string_1010_ac92(HINSTANCE16 param_1,ushort param_2,ushort param_3,int param_4)

{
  char *pcVar1;
  
  if ((0x0 < param_4) && (param_4 < 0x43)) {
    pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),param_1);
    return pcVar1;
  }
  return (char *)0x0;
}



ushort __stdcall16far pass1_1010_acc0(ushort param_1,ushort param_2,ulong param_3)

{
  ulong uVar1;
  
  uVar1 = struct_op_1030_73a8(param_3);
  if (*(int *)((int)uVar1 + 0x12) != 0x4) {
    return 0x1;
  }
  return 0x0;
}



void __stdcall16far pass1_1010_acec(ulong param_1,int param_2,ushort param_3)

{
  if (param_2 == 0x1) {
    *(undefined4 *)((int)param_1 + 0x12e) = 0x0;
  }
  else {
    if (param_2 != 0x5) {
      return;
    }
  }
  pass1_1010_1f62(param_3,param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xa),param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_ad22(ushort param_1,uint param_2,ushort param_3,ushort param_4,ushort param_5)

{
  undefined4 uVar1;
  ushort *puVar2;
  
  uVar1 = *(undefined4 *)(param_1 + 0x138);
  puVar2 = &param_5;
  pass1_1030_627e(param_3,(uint)puVar2,param_2,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_3,puVar2),
                  *(long *)((int)uVar1 + 0x20));
  if ((param_2 | (uint)puVar2) == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)puVar2,param_2);
  return;
}



void __stdcall16far pass1_1010_ad64(ushort param_1,ulong param_2,ulong param_3,ulong param_4,ushort param_5)

{
  if (param_3 != 0x0) {
    param_4 = *(ulong *)((int)param_3 + 0x2e);
    if (*(long *)((int)param_4 + 0x200) == 0x8000002) {
      return;
    }
  }
  pass1_1010_c58as(param_1,(ushort)param_2,(ushort)(param_2 >> 0x10),param_3,(ushort)param_4,param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * __stdcall16far
string_op_1010_ada6(HINSTANCE16 param_1,ushort param_2,ushort param_3,ushort param_4,int param_5,int param_6)

{
  char *pcVar1;
  char *pcStack6;
  
  pcStack6 = (char *)0x0;
  if (param_6 == 0x6) {
    if (param_5 == 0x0) goto LAB_1010_adee;
    pcVar1 = string_op_1020_c222(param_5);
  }
  else {
    if (param_6 != 0x7) {
      return (char *)0x0;
    }
    if (param_5 == 0x0) goto LAB_1010_adee;
    pcVar1 = string_op_1020_c2f8(param_5);
  }
  param_1 = 0x1020;
  pcStack6 = (char *)CONCAT22(param_2,pcVar1);
LAB_1010_adee:
  if (pcStack6 == (char *)0x0) {
    pcStack6 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),param_1);
  }
  return pcStack6;
}



ushort __stdcall16far pass1_1010_ae12(ushort param_1,ushort param_2,ulong param_3,int param_4,ushort param_5)

{
  char *pcVar1;
  int iVar2;
  ushort uStack4;
  
  if (param_4 == 0x6) {
    for (uStack4 = 0x0; (int)uStack4 < 0x15; uStack4 = uStack4 + 0x1) {
      pcVar1 = string_op_1020_c222(uStack4);
      iVar2 = pass1_1000_3d7a(param_3,CONCAT22(param_5,pcVar1));
      if (iVar2 == 0x0) {
        return uStack4;
      }
    }
  }
  else {
    if (param_4 == 0x7) {
      for (uStack4 = 0x0; (int)uStack4 < 0x11; uStack4 = uStack4 + 0x1) {
        pcVar1 = string_op_1020_c2f8(uStack4);
        iVar2 = pass1_1000_3d7a(param_3,CONCAT22(param_5,pcVar1));
        if (iVar2 == 0x0) {
          return uStack4;
        }
      }
    }
  }
  return 0xffff;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_ae92(ulong param_1,ushort param_2,uint param_3,ulong param_4,int param_5,ushort param_6)

{
  ushort uVar1;
  uchar *puVar2;
  ulong uVar3;
  astruct_67 *paVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  int iVar7;
  undefined uVar8;
  undefined uVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  int iVar12;
  
  if (param_3 == 0x15) {
    uVar3 = struct_op_1030_73a8(param_4);
    if (uVar3 != 0x0) {
      *(ushort *)((int)uVar3 + 0x20) = param_2;
      return;
    }
  }
  else {
    if (param_3 < 0x16) {
      if ((char)param_3 == '\x06') {
        pass1_1030_7f1a(param_4,param_2,param_6);
        uVar3 = struct_op_1030_73a8(param_4);
        uVar1 = pass1_1010_b028((ushort)param_1,(ushort)(param_1 >> 0x10),uVar3);
        uVar3 = pass1_1030_8326();
        puVar2 = (uchar *)(uVar3 >> 0x10);
        if (((uVar1 == 0xe) && ((puVar2 != (uchar *)0x0 || (0x32 < (uint)uVar3)))) &&
           ((param_2 == 0x1 || (((param_2 == 0x2 || (param_2 == 0x4)) || (param_2 == 0x3)))))) {
          uVar11 = 0x0;
          iVar12 = 0xb;
          uVar8 = 0x1;
          uVar9 = 0x0;
          uVar10 = 0x0;
          uVar6 = 0x0;
          iVar7 = 0x0;
          uVar5 = 0x0;
          paVar4 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_6,puVar2,param_5);
          post_win_msg_1008_a0e4
                    (paVar4,CONCAT22(uVar6,uVar5),iVar7,CONCAT11(uVar9,uVar8),CONCAT22(uVar11,uVar10),iVar12,0x1008,
                     param_6);
          return;
        }
      }
      else {
        if ((char)param_3 == '\a') {
          pass1_1030_7eda(param_4,param_2,param_6);
          return;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_af66(ulong param_1,uint param_2)

{
  undefined4 uVar1;
  ulong uVar2;
  uint uVar3;
  ushort in_stack_00000008;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x138);
  uVar2 = *(ulong *)((int)uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)(uVar2 >> 0x10));
  uVar3 = param_2 | (uint)uVar2;
  if (uVar3 == 0x0) {
    return;
  }
  pass1_1038_5050(uVar2 & 0xffff | (ulong)param_2 << 0x10,in_stack_00000008,(uint)uVar2,uVar3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_afa2(ulong param_1,uint param_2)

{
  undefined4 uVar1;
  ulong uVar2;
  ushort in_stack_00000008;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x138);
  uVar2 = *(ulong *)((int)uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)(uVar2 >> 0x10));
  if ((param_2 | (uint)uVar2) == 0x0) {
    return;
  }
  pass1_1038_50e0(uVar2 & 0xffff | (ulong)param_2 << 0x10,in_stack_00000008,(uint)uVar2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_afde(ulong param_1,int param_2)

{
  undefined4 uVar1;
  ulong uVar2;
  uint in_DX;
  ulong *puVar3;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x138);
  uVar2 = *(ulong *)((int)uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)(uVar2 >> 0x10));
  if ((in_DX | (uint)uVar2) == 0x0) {
    return;
  }
  puVar3 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,param_2);
  pass1_1038_4e78((uint)puVar3,(uchar *)((ulong)puVar3 >> 0x10),uVar2 & 0xffff | (ulong)in_DX << 0x10,puVar3);
  return;
}



ushort __stdcall16far pass1_1010_b028(ushort param_1,ushort param_2,ulong param_3)

{
  return *(ushort *)((int)param_3 + 0xc);
}


/*
Unable to decompile 'pass1_1010_b038'
Cause: 
Low-level Error: Overlapping input varnodes
*/


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far bad_1010_bf08(ushort param_1,ushort param_2,ulong param_3)

{
  bad_1028_e1bc(_PTR_LOOP_1050_65e2,param_3);
  return;
}



void __stdcall16far pass1_1010_bf1e(ulong param_1,int *param_2,int param_3,uchar *param_4,ushort param_5)

{
  ulong uVar1;
  uint uVar2;
  undefined *puVar3;
  int iVar4;
  undefined2 uVar5;
  undefined4 uStack40;
  int iStack36;
  uint uStack32;
  ushort *puStack26;
  undefined local_16 [0x12];
  int iStack4;
  
  bad_1010_bf08((ushort)param_1,(ushort)(param_1 >> 0x10),0x1000000);
  iStack4 = param_3 + -0x1;
  *param_2 = iStack4;
  uVar2 = iStack4 * 0x18;
  mem_op_1000_179c(uVar2,param_4,0x1000);
  uStack40 = CONCAT22(param_4,uVar2);
  uStack32 = (uint)param_4 | uVar2;
  iVar4 = (int)param_2;
  uVar5 = (undefined2)((ulong)param_2 >> 0x10);
  if (uStack32 == 0x0) {
    *(undefined4 *)(iVar4 + 0x2) = 0x0;
  }
  else {
    pass1_1000_5586((uchar *)0x4092,0x1020,iStack4,0x18,uVar2,(ushort)param_4);
    *(undefined4 *)(iVar4 + 0x2) = uStack40;
  }
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_5,local_16),0x1,0x0,0x100);
  puStack26 = *(ushort **)(iVar4 + 0x2);
  iStack36 = 0x0;
  while( true ) {
    puVar3 = local_16;
    pass1_1028_e4ec(CONCAT22(param_5,puVar3));
    if ((uStack32 | (uint)puVar3) == 0x0) break;
    uVar1 = *(ulong *)(puVar3 + 0x10);
    uStack32 = uStack32 | (uint)puVar3;
    if (uVar1 != 0x0) {
      uStack32 = (uint)(uVar1 >> 0x10);
      pass1_1008_3f62(puStack26,(ushort *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x4)));
      *(int *)((int)puStack26 + 0xc) = iStack36;
      iStack36 = iStack36 + 0x1;
      puStack26 = (ushort *)((ulong)puStack26 & 0xffff0000 | (ulong)((int)puStack26 + 0x18));
    }
  }
  return;
}

