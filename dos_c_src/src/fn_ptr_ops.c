astruct_18 * __stdcall16far pass1_1040_d89e(astruct_18 *param_1,byte param_2)

{
  pass1_1040_d1bc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}

astruct_18 * __stdcall16far pass1_1040_d056(astruct_18 *param_1,byte param_2)

{
  pass1_1040_ca74(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_18 * __stdcall16far pass1_1040_af9e(astruct_18 *param_1,byte param_2)

{
  pass1_1040_ace8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1040_a5d0(ulong param_1)

{
  uint uVar1;
  uint uVar2;
  astruct_258 *iVar4;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_258 *)param_1;
  uVar1 = iVar4->field_0x2;
  uVar2 = iVar4->field_0x4;
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1000_54e8((uchar *)0xa582,(ushort)&PTR_LOOP_1050_1040,*(int *)(uVar1 - 0x2),0xa,uVar1,uVar2);
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(uVar2,uVar1 - 0x2),0x1000);
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar4->field_0xc,0x1000);
  return;
}



void __stdcall16far pass1_1040_a582(ulong *param_1)

{
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  return;
}


astruct_18 * __stdcall16far pass1_1040_8db6(astruct_18 *param_1,byte param_2)

{
  pass1_1040_869a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_18 * __stdcall16far pass1_1040_8f16(astruct_18 *param_1,byte param_2)

{
  pass1_1040_8e82(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}

