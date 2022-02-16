
void __stdcall16far string_1040_a626(ushort *param_1,char *param_2,ushort param_3)

{
  ushort uVar1;
  
  uVar1 = str_op_1008_60e8(param_2,param_3);
  *param_1 = uVar1;
  *(ushort *)((int)param_1 + 0x2) = param_3;
  return;
}


char * __stdcall16far pass1_1040_4dcc(ulong param_1,int param_2,ushort param_3)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined2 uVar3;
  char *pcVar4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  uVar2 = *(undefined4 *)((int)param_1 + 0x90);
  uVar1 = *(undefined4 *)((int)param_1 + 0x94);
  pcVar4 = string_op_1010_ada6(0x1010,param_3,(ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),param_2,
                               *(int *)((int)uVar2 + 0xa));
  return pcVar4;
}


void __stdcall16far pass1_1040_5d42(ulong param_1)

{
  uint uVar1;
  char cVar2;
  int iVar3;
  undefined2 uVar4;
  ulong uVar5;
  
  uVar5 = pass1_1040_5d12(param_1);
  if (uVar5 != 0x0) {
    uVar1 = *(uint *)((int)uVar5 + 0xc);
    iVar3 = (int)param_1;
    uVar4 = (undefined2)(param_1 >> 0x10);
    if (uVar1 == 0x5f) {
      *(undefined2 *)(iVar3 + 0x96) = 0x53;
      return;
    }
    if (uVar1 < 0x60) {
      cVar2 = (char)uVar1;
      if (cVar2 == '(') {
        *(undefined2 *)(iVar3 + 0x96) = 0x54;
        return;
      }
      if (cVar2 == ')') {
        *(undefined2 *)(iVar3 + 0x96) = 0x55;
        return;
      }
      if (cVar2 == ']') {
        *(undefined2 *)(iVar3 + 0x96) = 0x51;
        return;
      }
      if (cVar2 == '^') {
        *(undefined2 *)(iVar3 + 0x96) = 0x52;
        return;
      }
    }
  }
  return;
}

