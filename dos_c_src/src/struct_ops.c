ushort * __stdcall16far struct_1040_bf3e(ushort *param_1,ushort param_2)

{
  astruct_442 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_442 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_2;
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x6 = 0x0;
  *param_1 = 0xc53e;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  return param_1;
}


void __stdcall16far pass1_1040_b7ee(astruct_57 *param_1,long param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  
  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_3,0xfab));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0x94) = 0x0;
  *(undefined4 *)(iVar1 + 0x98) = 0x0;
  *(undefined4 *)(iVar1 + 0xb0) = 0x0;
  *(undefined2 *)(iVar1 + 0xb4) = 0x0;
  *(undefined2 *)(iVar1 + 0xb6) = 0x0;
  *(undefined2 *)param_1 = 0xbeba;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  if (param_2 != 0x0) {
    uVar3 = (undefined2)((ulong)param_2 >> 0x10);
    *(undefined4 *)(iVar1 + 0xb0) = *(undefined4 *)((int)param_2 + 0x6);
    *(undefined2 *)(iVar1 + 0xb4) = *(undefined2 *)((int)param_2 + 0x14);
  }
  return;
}


void __stdcall16far pass1_1040_a640(astruct_57 *param_1,ulong param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_1040_b082(param_1,CONCAT22(param_3,0x1f1));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ulong *)(iVar1 + 0x94) = param_2;
  *(undefined2 *)(iVar1 + 0x98) = 0x0;
  *(undefined2 *)(iVar1 + 0xea) = 0x0;
  *(undefined2 *)param_1 = 0xac08;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return;
}



void __stdcall16far struct_1040_a598(ushort *param_1)

{
  astruct_259 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_259 *)param_1;
  *param_1 = 0x0;
  iVar1->field_0x2 = 0x0;
  iVar1->field_0x6 = 0x0;
  iVar1->field_0xa = 0x0;
  iVar1->field_0xc = 0x0;
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  return;
}


void __stdcall16far pass1_1040_a564(ulong *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x4) = 0x0;
  *(undefined4 *)((int)param_1 + 0x6) = 0x0;
  return;
}



void __stdcall16far pass1_1040_9824(ulong *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x0;
  *(undefined2 *)(iVar1 + 0x4) = 0x0;
  *(undefined4 *)(iVar1 + 0x56) = 0x0;
  *(undefined2 *)(iVar1 + 0x5a) = 0x0;
  *(undefined2 *)(iVar1 + 0x5c) = 0x0;
  *(undefined *)(iVar1 + 0x6) = 0x0;
  return;
}
