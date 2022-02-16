

void __stdcall16far pass1_1040_869a(astruct_18 *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  param_1->field_0x0 = 0x8ddc;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x90),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x94),0x1000);
  ui_cleanup_op_1040_782c(param_1,0x1000);
  return;
}