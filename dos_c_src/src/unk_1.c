void __stdcall16far pass1_1040_d76e(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar2 + 0x94);
  pass1_1018_5742((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),*(ulong **)(iVar2 + 0x9c),*(ulong *)(iVar2 + 0x98));
  *(undefined4 *)(iVar2 + 0x9c) = 0x0;
  return;
}


void __stdcall16far pass1_1040_d0f8(astruct_57 *param_1,ushort param_2)

{
  uint uVar1;
  uchar *in_DX;
  ushort uVar2;
  uchar *puVar3;
  uchar *puVar4;
  astruct_438 *iVar5;
  int unaff_DI;
  undefined2 uVar5;
  ushort unaff_SS;
  ushort *puVar6;
  ulong uVar7;
  astruct_392 *iVar8;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x1845));
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_438 *)param_1;
  *(undefined4 *)&iVar5->field_0x94 = 0x0;
  iVar5->field_0x98 = _PTR_LOOP_1050_5f16;
  *(undefined4 *)&iVar5->field_0x9c = 0x0;
  iVar5->field_0xa0 = 0x0;
  *(undefined2 *)param_1 = 0xd8c4;
  iVar5->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x47,unaff_SS,in_DX,unaff_DI);
  uVar2 = (ushort)((ulong)puVar6 >> 0x10);
  iVar5->field_0x94 = (ushort)puVar6;
  iVar5->field_0x96 = uVar2;
  uVar7 = pass1_1018_5732(iVar5->field_0x94,uVar2,iVar5->field_0x98,(ushort)puVar6,uVar2,unaff_SS);
  puVar3 = (uchar *)(uVar7 >> 0x10);
  iVar5->field_0x9c = (uint)uVar7;
  iVar5->field_0x9e = puVar3;
  uVar1 = (uint)puVar3 | iVar5->field_0x9c;
  if (uVar1 == 0x0) {
    mem_op_1000_179c(0xc,puVar3,0x1000);
    puVar4 = (uchar *)((uint)puVar3 | uVar1);
    if (puVar4 == (uchar *)0x0) {
      *(undefined4 *)&iVar5->field_0x9c = 0x0;
    }
    else {
      pass1_1010_8ef2((ushort *)CONCAT22(puVar3,uVar1),puVar4,unaff_SS);
      iVar5->field_0x9c = uVar1;
      iVar5->field_0x9e = puVar4;
    }
  }
  return;
}