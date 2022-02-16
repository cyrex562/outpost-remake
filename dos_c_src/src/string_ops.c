
void __stdcall16far string_1040_a626(ushort *param_1,char *param_2,ushort param_3)

{
  ushort uVar1;
  
  uVar1 = str_op_1008_60e8(param_2,param_3);
  *param_1 = uVar1;
  *(ushort *)((int)param_1 + 0x2) = param_3;
  return;
}

