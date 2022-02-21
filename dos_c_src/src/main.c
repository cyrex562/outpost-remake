
int main(int argc, char **argv)
{
    return 0;
}



void __stdcall16far
init_op_1008_54aa(uchar *param_1,char *param_2,uchar *param_3,uchar *param_4,ushort param_5,ushort param_6,
                 ushort param_7,ushort param_8)

{
  code **ppcVar1;
  uint uVar3;
  ushort in_CX;
  ushort in_DX;
  uchar *puVar4;
  ushort extraout_DX;
  ushort uVar5;
  ushort extraout_DX_00;
  ushort uVar6;
  ushort extraout_DX_01;
  ulong uVar7;
  ulong *puStack12;
  ulong uVar2;
  
  if (param_3 != (uchar *)0x0) {
    return;
  }
  dos3_call_op_1000_435c((UINT16 *)0x0,in_CX,in_DX,&stack0xfffe,param_8);
  pass1_1000_4d0c(param_5);
  pass1_1000_1fea();
  _PTR_LOOP_1050_03a0 = mem_op_1000_1902(0x0,0x32,0x0,0x12,0x1000,in_DX);
  _PTR_LOOP_1050_029c = mem_op_1000_1902(0x0,0x64,0x0,0xc,0x1000,(int)(_PTR_LOOP_1050_03a0 >> 0x10));
  _PTR_LOOP_1050_4fb8 = mem_op_1000_1902(0x0,0x64,0x0,0x10,0x1000,(int)(_PTR_LOOP_1050_029c >> 0x10));
  _PTR_LOOP_1050_68a2 = mem_op_1000_1902(0x0,0x64,0x0,0xe,0x1000,(int)(_PTR_LOOP_1050_4fb8 >> 0x10));
  _PTR_LOOP_1050_5744 = mem_op_1000_1902(0x0,0x1f4,0x0,0x42,0x1000,(int)(_PTR_LOOP_1050_68a2 >> 0x10));
  uVar7 = mem_op_1000_1902(0x0,0x32,0x0,0x6,0x1000,(int)(_PTR_LOOP_1050_5744 >> 0x10));
  puVar4 = (uchar *)(uVar7 >> 0x10);
  PTR_LOOP_1050_5768 = (undefined *)uVar7;
  PTR_LOOP_1050_038c = param_4;
  PTR_LOOP_1050_038e = param_3;
  PTR_LOOP_1050_0390 = param_1;
  PTR_LOOP_1050_576a = puVar4;
  uVar3 = str_op_1008_60e8(param_2,(ushort)puVar4);
  _PTR_LOOP_1050_0392 = CONCAT22(puVar4,uVar3);
  mem_op_1000_179c(0xc,puVar4,0x1000);
  if (((uint)puVar4 | uVar3) == 0x0) {
    uVar3 = 0x0;
    uVar5 = 0x0;
  }
  else {
    struct_op_1008_0000((ushort *)CONCAT13((char)((uint)puVar4 >> 0x8),CONCAT12((char)puVar4,uVar3)));
    uVar5 = extraout_DX;
  }
  puStack12 = (ulong *)CONCAT22(uVar5,uVar3);
  if (_PTR_LOOP_1050_0392 != 0x0) {
    ppcVar1 = (code **)((int)*puStack12 + 0x4);
    (**ppcVar1)(0x1000,(char)uVar3,uVar5,(int)_PTR_LOOP_1050_0392,(char)((ulong)_PTR_LOOP_1050_0392 >> 0x10));
  }
  uVar2 = *puStack12;
  ppcVar1 = (code **)uVar2 + 0x4;
  (**ppcVar1)(0x1000,uVar3,(char)uVar5);
  uVar6 = extraout_DX_00;
  win_msg_op_1008_9498((MSG *)&PTR_LOOP_1050_1000,(MSG16 *)param_8);
  if (puStack12 != (ulong *)0x0) {
    ppcVar1 = (code **)uVar2;
    (**ppcVar1)(0x1000,uVar3,(char)uVar5,0x1);
    uVar6 = extraout_DX_01;
  }
  uVar7 = mem_op_1000_1b68(uVar6,0x1000,(ushort)_PTR_LOOP_1050_03a0,(ushort)(_PTR_LOOP_1050_03a0 >> 0x10));
  uVar7 = mem_op_1000_1b68((ushort)(uVar7 >> 0x10),0x1000,(ushort)_PTR_LOOP_1050_029c,
                           (ushort)(_PTR_LOOP_1050_029c >> 0x10));
  uVar7 = mem_op_1000_1b68((ushort)(uVar7 >> 0x10),0x1000,(ushort)_PTR_LOOP_1050_4fb8,
                           (ushort)(_PTR_LOOP_1050_4fb8 >> 0x10));
  uVar7 = mem_op_1000_1b68((ushort)(uVar7 >> 0x10),0x1000,(ushort)_PTR_LOOP_1050_68a2,
                           (ushort)(_PTR_LOOP_1050_68a2 >> 0x10));
  mem_op_1000_1b68((ushort)(uVar7 >> 0x10),0x1000,(ushort)_PTR_LOOP_1050_5744,(ushort)(_PTR_LOOP_1050_5744 >> 0x10));
  return;
}


void __cdecl16far init_1000_23be(ushort param_1,ushort param_2,ushort param_3)

{
  init_op_1008_54aa(PTR_LOOP_1050_5f52,CONCAT22(PTR_LOOP_1050_5f50,PTR_LOOP_1050_5f4e),PTR_LOOP_1050_5f4a,
                    PTR_LOOP_1050_5f4c,(ushort)&USHORT_1050_1050,param_1,param_2,param_3);
  return;
}
