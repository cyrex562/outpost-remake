
void __stdcall16far pass1_1040_d1bc(astruct_18 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_513 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_513 *)param_1;
  param_1->field_0x0 = 0xd8c4;
  iVar4->field_0x2 = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,iVar4->field_0x6);
  puVar1 = iVar4->field_0x9c;
  uVar2 = iVar4->field_0x9e;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)((int)&PTR_LOOP_1050_1038,puVar1,uVar2,0x1);
  }
  unk_draw_op_1040_b0f8(param_1);
  return;
}
