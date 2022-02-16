
void __stdcall16far struct_1028_d22e(ulong *param_1,ulong param_2,ushort param_3)

{
  uint uVar1;
  uchar *puVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x0;
  *(ulong *)((int)param_1 + 0x4) = param_2;
  mem_op_1000_179c(0xc,(uchar *)param_3,0x1000);
  uVar1 = (uint)param_2;
  puVar2 = (uchar *)(param_3 | uVar1);
  if (puVar2 == (uchar *)0x0) {
    *param_1 = 0x0;
  }
  else {
    struct_1028_d59c((ulong *)(param_2 & 0xffff | (ulong)param_3 << 0x10),puVar2);
    *(uint *)param_1 = uVar1;
    *(uchar **)((int)param_1 + 0x2) = puVar2;
  }
  return;
}



void __stdcall16far pass1_1028_d282(uint *param_1)

{
  uint uVar1;
  uint uVar2;
  astruct_18 *paStack6;
  
  uVar1 = *param_1;
  uVar2 = *(uint *)((int)param_1 + 0x2);
  paStack6 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1028_d658(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack6,0x1000);
  }
  return;
}



void __stdcall16far struct_1028_d2b0(ulong *param_1,ushort param_2,uchar param_3)

{
  undefined2 local_10c;
  undefined2 uStack266;
  
  struct_1028_9c62((int)&local_10c,param_2,0x3e80,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0x3a98,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0x36b0,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0x32c8,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0x2ee0,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0x2af8,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0x2710,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,(int)s_noth_bmp_1050_2321 + 0x7,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0x1f40,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0x1b58,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0x1770,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0x1388,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0xfa0,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0xbb8,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((int)&local_10c,param_2,0x3e8,param_2,param_3);
  fn_ptr_1028_d566(param_1,(ulong *)CONCAT22(param_2,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  pass1_1028_d6b2(*param_1);
  return;
}



BOOL16 __stdcall16far pass1_1028_d52c(ulong *param_1,ulong param_2,ulong *param_3)

{
  code **ppcVar1;
  int iVar2;
  BOOL16 BVar3;
  
  ppcVar1 = (code **)((int)*param_3 + 0x8);
  iVar2 = (**ppcVar1)();
  if (iVar2 != 0x0) {
    BVar3 = pass1_1028_d776(*param_1,param_2,param_3);
    if (BVar3 != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



BOOL16 __stdcall16far fn_ptr_1028_d566(ulong *param_1,ulong *param_2)

{
  code **ppcVar1;
  int iVar2;
  ushort uVar3;
  
  ppcVar1 = (code **)((int)*param_2 + 0x8);
  iVar2 = (**ppcVar1)();
  if (iVar2 != 0x0) {
    uVar3 = fn_ptr_1028_d742(*param_1,param_2);
    if (uVar3 != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1028_d59c(ulong *param_1,uchar *param_2)

{
  undefined2 *puVar1;
  uint uVar2;
  undefined2 *puVar3;
  uchar *puVar4;
  uchar *extraout_DX;
  astruct_158 *iVar5;
  undefined2 uVar5;
  undefined2 *puStack14;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_158 *)param_1;
  *param_1 = 0x0;
  iVar5->field_0x4 = (undefined2 *)0x0;
  iVar5->field_0x8 = (undefined2 *)0x0;
  puVar3 = (undefined2 *)*_PTR_LOOP_1050_5748;
  *param_1 = (ulong)puVar3;
  mem_op_1000_179c(0xc,param_2,0x1000);
  puVar1 = (undefined2 *)((ulong)puVar3 & 0xffff | ZEXT24(param_2) << 0x10);
  puVar4 = (uchar *)((uint)param_2 | (uint)puVar3);
  if (puVar4 == (uchar *)0x0) {
    iVar5->field_0x4 = (undefined2 *)0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)((ulong)puVar3 & 0xffff | ZEXT24(param_2) << 0x10));
    *puVar1 = 0xd804;
    *(undefined2 *)((uint)puVar3 + 0x2) = (int)&USHORT_1050_1028;
    iVar5->field_0x4 = puVar1;
    puVar3 = puVar1;
    puVar4 = extraout_DX;
  }
  uVar2 = (uint)puVar3;
  mem_op_1000_179c(0xc,puVar4,0x1000);
  puStack14 = (undefined2 *)CONCAT22(puVar4,uVar2);
  if (((uint)puVar4 | uVar2) == 0x0) {
    iVar5->field_0x8 = (undefined2 *)0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar4,uVar2));
    *puStack14 = 0xd804;
    *(undefined2 *)(uVar2 + 0x2) = (int)&USHORT_1050_1028;
    iVar5->field_0x8 = puStack14;
  }
  return;
}



void __stdcall16far pass1_1028_d658(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_446 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_446 *)param_1;
  puVar1 = iVar4->field_0x4;
  uVar2 = iVar4->field_0x6;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0x8;
  uVar2 = iVar4->field_0xa;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  return;
}



ushort __stdcall16far pass1_1028_d69e(ulong param_1)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x4);
  return *(ushort *)((int)uVar1 + 0x8);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_d6b2(ulong param_1)

{
  ulong *puVar1;
  ulong uVar2;
  code **ppcVar3;
  undefined4 *puVar4;
  uint uVar5;
  uint extraout_DX;
  undefined2 uVar6;
  ulong uVar7;
  
  uVar2 = *_PTR_LOOP_1050_65e2;
  while( true ) {
    uVar6 = (undefined2)(param_1 >> 0x10);
    uVar7 = pass1_1020_c860(*(ulong *)((int)param_1 + 0x8));
    uVar5 = (uint)(uVar7 >> 0x10);
    if (((uVar5 | (uint)uVar7) == 0x0) || (puVar1 = (ulong *)((uint)uVar7 + 0xc), uVar2 <= *puVar1 && *puVar1 != uVar2))
    break;
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x8) + 0x10);
    uVar7 = uVar2;
    (**ppcVar3)();
    puVar4 = (undefined4 *)(uVar7 & 0xffff | (ulong)extraout_DX << 0x10);
    fn_ptr_1028_d742(param_1,(ulong *)(uVar7 & 0xffff | (ulong)extraout_DX << 0x10));
    if (puVar4 != (undefined4 *)0x0) {
      ppcVar3 = (code **)*puVar4;
      (**ppcVar3)(0x1020,(int)uVar7,extraout_DX,0x1);
    }
  }
  return;
}



void __stdcall16far fn_ptr_1028_d728(ulong param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x10);
  (**ppcVar1)();
  return;
}



ushort __stdcall16far fn_ptr_1028_d742(ulong param_1,ulong *param_2)

{
  code **ppcVar1;
  ulong uVar2;
  
  ppcVar1 = (code **)((int)*param_2 + 0xc);
  uVar2 = (**ppcVar1)();
  pass1_1020_c872(*(ulong *)((int)param_1 + 0x4),*(undefined4 *)((int)uVar2 + 0x4),uVar2);
  return 0x1;
}



BOOL16 __stdcall16far pass1_1028_d776(ulong param_1,ulong param_2,ulong *param_3)

{
  code **ppcVar1;
  ulong uVar2;
  
  ppcVar1 = (code **)((int)*param_3 + 0xc);
  uVar2 = (**ppcVar1)();
  pass1_1020_c872(*(ulong *)((int)param_1 + 0x8),param_2,uVar2);
  return 0x1;
}



BOOL16 __stdcall16far pass1_1028_d7a0(ushort param_1,ushort param_2,ulong param_3,uint16_t param_4)

{
  BOOL16 BVar1;
  
  BVar1 = write_to_file_1008_7cac(param_3,param_4);
  if (BVar1 != 0x0) {
    BVar1 = 0x1;
  }
  return BVar1;
}



int __stdcall16far
read_file_1028_d7ba(undefined2 param_1,undefined2 param_2,undefined4 param_3,uint16_t param_4,uint16_t param_5)

{
  read_file_1008_7cfe((int)param_3,(int)((ulong)param_3 >> 0x10),0x8,0x1008,param_4);
  if (param_5 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d4;
    return param_5;
  }
  return 0x1;
}



astruct_18 * __stdcall16far pass1_1028_d7de(astruct_18 *param_1,byte param_2)

{
  pass1_1008_57c4(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_d81c(ulong *param_1,ulong param_2,uchar *param_3,ushort param_4)

{
  uint *puVar1;
  uchar *puVar2;
  uchar *puVar3;
  uint uVar4;
  astruct_136 *iVar6;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (astruct_136 *)param_1;
  *param_1 = 0x0;
  iVar6->field_0x4 = param_2;
  *(undefined4 *)&iVar6->field_0x52 = 0x0;
  _PTR_LOOP_1050_65e2 = param_1;
  iVar6->field_0x32 = 0xec36;
  iVar6->field_0x34 = (int)&USHORT_1050_1028;
  iVar6->field_0x36 = 0xecac;
  iVar6->field_0x38 = (int)&USHORT_1050_1028;
  iVar6->field_0x3a = 0xed2c;
  iVar6->field_0x3c = (int)&USHORT_1050_1028;
  iVar6->field_0x3e = 0xedc4;
  iVar6->field_0x40 = (int)&USHORT_1050_1028;
  iVar6->field_0x42 = 0xee54;
  iVar6->field_0x44 = (int)&USHORT_1050_1028;
  iVar6->field_0x46 = 0xef00;
  iVar6->field_0x48 = (int)&USHORT_1050_1028;
  iVar6->field_0x4a = 0x10b0;
  iVar6->field_0x4c = 0x1030;
  iVar6->field_0x4e = 0x1120;
  iVar6->field_0x50 = 0x1030;
  mem_op_1000_179c(0x8,param_3,0x1000);
  uVar4 = (uint)param_2;
  puVar2 = (uchar *)((uint)param_3 | uVar4);
  if (puVar2 != (uchar *)0x0) {
    pass1_1030_615a((astruct_137 *)(param_2 & 0xffff | ZEXT24(param_3) << 0x10),(ushort)puVar2);
  }
  mem_op_1000_179c(0x56c,puVar2,0x1000);
  puVar3 = (uchar *)((uint)puVar2 | uVar4);
  if (puVar3 == (uchar *)0x0) {
    uVar4 = 0x0;
    puVar3 = (uchar *)0x0;
  }
  else {
    struct_1030_44be((ulong *)CONCAT22(puVar2,uVar4),(ushort)puVar3);
  }
  iVar6->field_0x52 = uVar4;
  iVar6->field_0x54 = puVar3;
  mem_op_1000_179c(0x4,puVar3,0x1000);
  puVar2 = (uchar *)((uint)puVar3 | uVar4);
  if (puVar2 != (uchar *)0x0) {
    struct_1008_bde0((ulong *)CONCAT22(puVar3,uVar4),puVar2);
  }
  puVar1 = pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar6->field_0xa),
                           (WNDCLASS16 *)0x0,0x24);
  mem_op_1000_179c(0x1c,puVar2,0x1000);
  puVar3 = (uchar *)((uint)puVar2 | (uint)puVar1);
  if (puVar3 == (uchar *)0x0) {
    *(undefined4 *)&iVar6->field_0xe = 0x0;
  }
  else {
    struct_1030_11aa((ushort *)CONCAT22(puVar2,puVar1),0x5,0x15,param_4);
    iVar6->field_0xe = puVar1;
    iVar6->field_0x10 = puVar3;
  }
  mem_op_1000_179c(0x1c,puVar3,0x1000);
  puVar2 = (uchar *)((uint)puVar3 | (uint)puVar1);
  if (puVar2 == (uchar *)0x0) {
    puVar1 = (uint *)0x0;
    puVar2 = (uchar *)0x0;
  }
  else {
    struct_1030_11aa((ushort *)CONCAT22(puVar3,puVar1),0x5,0xa,param_4);
  }
  iVar6->field_0x12 = puVar1;
  iVar6->field_0x14 = puVar2;
  mem_op_1000_179c(0x1c,puVar2,0x1000);
  puVar3 = (uchar *)((uint)puVar2 | (uint)puVar1);
  if (puVar3 == (uchar *)0x0) {
    puVar1 = (uint *)0x0;
    puVar3 = (uchar *)0x0;
  }
  else {
    struct_1030_11aa((ushort *)CONCAT22(puVar2,puVar1),0x5,0x19,param_4);
  }
  iVar6->field_0x16 = puVar1;
  iVar6->field_0x18 = puVar3;
  mem_op_1000_179c(0x1c,puVar3,0x1000);
  puVar2 = (uchar *)((uint)puVar3 | (uint)puVar1);
  if (puVar2 == (uchar *)0x0) {
    puVar1 = (uint *)0x0;
    puVar2 = (uchar *)0x0;
  }
  else {
    struct_1030_11aa((ushort *)CONCAT22(puVar3,puVar1),0x5,0xa,param_4);
  }
  iVar6->field_0x1a = puVar1;
  iVar6->field_0x1c = puVar2;
  mem_op_1000_179c(0x1c,puVar2,0x1000);
  puVar3 = (uchar *)((uint)puVar2 | (uint)puVar1);
  if (puVar3 == (uchar *)0x0) {
    puVar1 = (uint *)0x0;
    puVar3 = (uchar *)0x0;
  }
  else {
    struct_1030_11aa((ushort *)CONCAT22(puVar2,puVar1),0x64,0x1f4,param_4);
  }
  iVar6->field_0x1e = puVar1;
  iVar6->field_0x20 = puVar3;
  mem_op_1000_179c(0x1c,puVar3,0x1000);
  puVar2 = (uchar *)((uint)puVar3 | (uint)puVar1);
  if (puVar2 == (uchar *)0x0) {
    puVar1 = (uint *)0x0;
    puVar2 = (uchar *)0x0;
  }
  else {
    struct_1030_11aa((ushort *)CONCAT22(puVar3,puVar1),0x19,0x64,param_4);
  }
  iVar6->field_0x22 = puVar1;
  iVar6->field_0x24 = puVar2;
  mem_op_1000_179c(0x1c,puVar2,0x1000);
  puVar3 = (uchar *)((uint)puVar2 | (uint)puVar1);
  if (puVar3 == (uchar *)0x0) {
    puVar1 = (uint *)0x0;
    puVar3 = (uchar *)0x0;
  }
  else {
    struct_1030_11aa((ushort *)CONCAT22(puVar2,puVar1),0x64,0x1f4,param_4);
  }
  iVar6->field_0x26 = puVar1;
  iVar6->field_0x28 = puVar3;
  mem_op_1000_179c(0x1c,puVar3,0x1000);
  uVar4 = (uint)puVar3 | (uint)puVar1;
  if (uVar4 == 0x0) {
    puVar1 = (uint *)0x0;
    uVar4 = 0x0;
  }
  else {
    struct_1030_11aa((ushort *)CONCAT22(puVar3,puVar1),0x5,0x2,param_4);
  }
  iVar6->field_0x2a = puVar1;
  iVar6->field_0x2c = uVar4;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_daba(ulong param_1,ushort param_2)

{
  uint uVar1;
  uint uVar2;
  undefined4 *puVar3;
  code **ppcVar4;
  astruct_18 *paVar5;
  astruct_447 *iVar5;
  undefined2 uVar6;
  astruct_18 *paStack14;
  
  paVar5 = _PTR_LOOP_1050_5740;
  if (_PTR_LOOP_1050_5740 != (astruct_18 *)0x0) {
    pass1_1030_61b0(&_PTR_LOOP_1050_5740->field_0x0);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar5,0x1000);
  }
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (astruct_447 *)param_1;
  uVar1 = iVar5->field_0x52;
  uVar2 = iVar5->field_0x54;
  paStack14 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1030_4538((ulong *)CONCAT22(uVar2,uVar1));
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paStack14,0x1000);
  }
  if (_PTR_LOOP_1050_5166 != (undefined4 *)0x0) {
    ppcVar4 = (code **)*_PTR_LOOP_1050_5166;
    (**ppcVar4)(param_2,(int)_PTR_LOOP_1050_5166);
  }
  paVar5 = _PTR_LOOP_1050_06e0;
  _PTR_LOOP_1050_65e2 = 0x0;
  if (_PTR_LOOP_1050_06e0 != (astruct_18 *)0x0) {
    pass1_1008_c626((ulong *)_PTR_LOOP_1050_06e0);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar5,0x1000);
  }
  puVar3 = iVar5->field_0xe;
  uVar1 = iVar5->field_0x10;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field_0x12;
  uVar1 = iVar5->field_0x14;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field_0x16;
  uVar1 = iVar5->field_0x18;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field_0x1a;
  uVar1 = iVar5->field_0x1c;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field_0x1e;
  uVar1 = iVar5->field_0x20;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field_0x22;
  uVar1 = iVar5->field_0x24;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field_0x26;
  uVar1 = iVar5->field_0x28;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field_0x2a;
  uVar1 = iVar5->field_0x2c;
  if ((uVar1 | (uint)puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(param_2,puVar3,uVar1,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_dc52(astruct_92 *param_1,int param_2,ushort param_3,uint param_4)

{
  undefined4 uVar1;
  astruct_92 *iVar2;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_92 *)param_1;
  *(undefined2 *)param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  iVar2->field_0x4 = *(undefined4 *)((int)_PTR_LOOP_1050_65e2 + (param_4 >> 0x8) * 0x4 + 0xa);
  iVar2->field_0x8 = 0x1;
  iVar2->field_0x10 = param_2;
  *(undefined2 *)param_1 = 0x11a6;
  iVar2->field_0x2 = 0x1030;
  uVar1 = iVar2->field_0x4;
  iVar2->field_0xc = *(undefined4 *)((int)uVar1 + 0xa);
  if (param_2 == 0x0) {
    iVar2->field_0x8 = iVar2->field_0xc;
  }
  else {
    iVar2->field_0x8 = 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far write_to_file_1028_dce2(undefined4 *param_1,undefined4 param_2,uint16_t param_3)

{
  code **ppcVar1;
  BOOL16 BVar2;
  undefined *puVar3;
  uint16_t in_DX;
  uint extraout_DX;
  uint uVar4;
  int iVar5;
  undefined2 uVar6;
  ushort uVar7;
  undefined4 local_26 [0x2];
  undefined2 local_1e [0x3];
  undefined4 uStack24;
  undefined local_14 [0x12];
  
  uVar7 = (ushort)((ulong)param_2 >> 0x10);
  BVar2 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar2 != 0x0) {
    local_26[0] = *param_1;
    BVar2 = write_to_file_1008_7e1c((ushort)param_2,uVar7,(ushort)local_26,param_3,(char *)0x4,0x1008);
    if (BVar2 != 0x0) {
      uVar6 = (undefined2)((ulong)param_1 >> 0x10);
      iVar5 = (int)param_1;
      local_1e[0] = *(undefined2 *)(iVar5 + 0x8);
      BVar2 = write_to_file_1008_7e1c((ushort)param_2,uVar7,(ushort)local_1e,param_3,(char *)0x2,0x1008);
      if (BVar2 != 0x0) {
        ppcVar1 = (code **)((int)*_PTR_LOOP_1050_5166 + 0xc);
        (**ppcVar1)(0x1008,(int)_PTR_LOOP_1050_5166,(int)((ulong)_PTR_LOOP_1050_5166 >> 0x10),param_2);
        in_DX = extraout_DX;
        if (BVar2 != 0x0) {
          BVar2 = write_to_file_1008_7cac(param_2,param_3);
          if (BVar2 != 0x0) {
            in_DX = write_file_fn_1028_e56c(iVar5,uVar6,param_2,param_3);
            if (BVar2 != 0x0) {
              BVar2 = write_to_file_1008_7cac(param_2,param_3);
              if (BVar2 != 0x0) {
                in_DX = write_file_fn_1028_e56c(iVar5,uVar6,param_2,param_3);
                if (BVar2 != 0x0) {
                  BVar2 = write_to_file_1008_7cac(param_2,param_3);
                  if (BVar2 != 0x0) {
                    in_DX = write_file_fn_1028_e56c(iVar5,uVar6,param_2,param_3);
                    if (BVar2 != 0x0) {
                      BVar2 = write_to_file_1008_7cac(param_2,param_3);
                      if (BVar2 != 0x0) {
                        in_DX = write_file_fn_1028_e56c(iVar5,uVar6,param_2,param_3);
                        if (BVar2 != 0x0) {
                          BVar2 = write_to_file_1008_7cac(param_2,param_3);
                          if (BVar2 != 0x0) {
                            in_DX = write_file_fn_1028_e56c(iVar5,uVar6,param_2,param_3);
                            if (BVar2 != 0x0) {
                              BVar2 = write_to_file_1008_7cac(param_2,param_3);
                              if (BVar2 != 0x0) {
                                in_DX = write_file_fn_1028_e56c(iVar5,uVar6,param_2,param_3);
                                if (BVar2 != 0x0) {
                                  BVar2 = write_to_file_1008_7cac(param_2,param_3);
                                  if (BVar2 != 0x0) {
                                    in_DX = write_file_fn_1028_e56c(iVar5,uVar6,param_2,param_3);
                                    if (BVar2 != 0x0) {
                                      BVar2 = write_to_file_1008_7cac(param_2,param_3);
                                      if (BVar2 != 0x0) {
                                        in_DX = write_file_fn_1028_e56c(iVar5,uVar6,param_2,param_3);
                                        if (BVar2 != 0x0) {
                                          pass1_1028_dc52((astruct_92 *)CONCAT22(param_3,local_14),0x1,0x0,0x400);
                                          while( true ) {
                                            uVar4 = in_DX;
                                            puVar3 = local_14;
                                            pass1_1028_e4ec(CONCAT22(param_3,puVar3));
                                            uStack24 = CONCAT22(uVar4,puVar3);
                                            in_DX = uVar4 | (uint)puVar3;
                                            if (in_DX == 0x0) break;
                                            if (*(long *)(puVar3 + 0x200) != 0x8000002) {
                                              pass1_1038_3ba0(CONCAT22(uVar4,puVar3));
                                            }
                                          }
                                          return 0x10000;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  return (ulong)in_DX;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far read_file_1028_def2(ulong param_1,undefined4 param_2,uint16_t param_3,uint16_t param_4)

{
  code **ppcVar1;
  BOOL16 BVar2;
  ushort unaff_SI;
  ushort unaff_DI;
  ushort uVar3;
  uchar in_AF;
  ushort uVar4;
  ushort uVar5;
  
  uVar4 = (ushort)param_2;
  uVar5 = (ushort)((ulong)param_2 >> 0x10);
  read_file_1008_7cfe(uVar4,uVar5,0xa,0x1008,param_3);
  if (param_4 != 0x0) {
    uVar3 = (ushort)(param_1 >> 0x10);
    BVar2 = read_file_1008_7dee(uVar4,uVar5,(ushort)param_1,0x0,uVar3,0x4,0x1008);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee(uVar4,uVar5,(ushort)param_1 + 0x8,0x0,uVar3,0x2,0x1008);
      if (BVar2 != 0x0) {
        uVar3 = (ushort)((ulong)*_PTR_LOOP_1050_5166 >> 0x10);
        ppcVar1 = (code **)((int)*_PTR_LOOP_1050_5166 + 0x10);
        (**ppcVar1)(0x1008,(int)_PTR_LOOP_1050_5166,(int)((ulong)_PTR_LOOP_1050_5166 >> 0x10),param_2);
        if (BVar2 != 0x0) {
          read_file_1008_7cfe(uVar4,uVar5,0xc,0x1008,param_3);
          if (BVar2 != 0x0) {
            pass1_1028_e628(param_1,uVar4,uVar5,0x0,0x100,unaff_SI,unaff_DI,uVar3,param_3,in_AF);
            if (BVar2 != 0x0) {
              read_file_1008_7cfe(uVar4,uVar5,0xd,0x1008,param_3);
              if (BVar2 != 0x0) {
                pass1_1028_e628(param_1,uVar4,uVar5,0x0,0x200,unaff_SI,unaff_DI,uVar3,param_3,in_AF);
                if (BVar2 != 0x0) {
                  read_file_1008_7cfe(uVar4,uVar5,0xe,0x1008,param_3);
                  if (BVar2 != 0x0) {
                    pass1_1028_e628(param_1,uVar4,uVar5,0x0,0x300,unaff_SI,unaff_DI,uVar3,param_3,in_AF);
                    if (BVar2 != 0x0) {
                      read_file_1008_7cfe(uVar4,uVar5,0xf,0x1008,param_3);
                      if (BVar2 != 0x0) {
                        pass1_1028_e628(param_1,uVar4,uVar5,0x0,0x400,unaff_SI,unaff_DI,uVar3,param_3,in_AF);
                        if (BVar2 != 0x0) {
                          read_file_1008_7cfe(uVar4,uVar5,0x10,0x1008,param_3);
                          if (BVar2 != 0x0) {
                            pass1_1028_e628(param_1,uVar4,uVar5,0x0,0x500,unaff_SI,unaff_DI,uVar3,param_3,in_AF);
                            if (BVar2 != 0x0) {
                              read_file_1008_7cfe(uVar4,uVar5,0x11,0x1008,param_3);
                              if (BVar2 != 0x0) {
                                pass1_1028_e628(param_1,uVar4,uVar5,0x0,0x600,unaff_SI,unaff_DI,uVar3,param_3,in_AF);
                                if (BVar2 != 0x0) {
                                  read_file_1008_7cfe(uVar4,uVar5,0x12,0x1008,param_3);
                                  if (BVar2 != 0x0) {
                                    pass1_1028_e628(param_1,uVar4,uVar5,0x0,0x700,unaff_SI,unaff_DI,uVar3,param_3,in_AF)
                                    ;
                                    if (BVar2 != 0x0) {
                                      read_file_1008_7cfe(uVar4,uVar5,0x13,0x1008,param_3);
                                      if (BVar2 != 0x0) {
                                        pass1_1028_e628(param_1,uVar4,uVar5,0x0,0x800,unaff_SI,unaff_DI,uVar3,param_3,
                                                        in_AF);
                                        if (BVar2 != 0x0) {
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  return;
}



void __stdcall16far pass1_1028_e0a0(ulong param_1,ulong param_2,uchar *param_3,ushort param_4,uchar param_5)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x52);
  pass1_1030_4782(param_4,param_5,param_3,(ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),0x1,(int)param_2,
                  (int)(param_2 >> 0x10));
  return;
}



ulong __stdcall16far pass1_1028_e0bc(ulong param_1,int param_2,ulong *param_3,uchar *param_4,ushort param_5)

{
  ulong *puVar1;
  ulong *puVar2;
  ulong *puVar3;
  int iVar4;
  uchar *puVar5;
  ulong *puVar6;
  
  mem_op_1000_179c(0x98,param_4,0x1000);
  puVar3 = param_3;
  puVar5 = param_4;
  pass1_1030_4bbe(param_5,(ushort)param_4,*(ulong *)((int)param_1 + 0x52),param_2);
  puVar6 = param_3;
  for (iVar4 = 0x26; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
    puVar2 = puVar6;
    puVar6 = puVar6 + 0x1;
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar2 = *puVar1;
  }
  return CONCAT22(param_4,param_3);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_e100(ulong param_1,ushort param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  astruct_311 *uVar4;
  int iVar4;
  uint uVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  undefined2 uVar8;
  ushort unaff_SS;
  ulong uStack10;
  ulong uStack6;
  ulong uVar3;
  
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_3,0x1000);
    PTR_LOOP_1050_5f2e = param_3;
  }
  else {
  }
  uVar4 = (astruct_311 *)fn_ptr_op_1000_1708(0xae,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  uVar3 = ZEXT24(uVar4);
  uStack10 = CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
  uVar5 = (uint)PTR_LOOP_1050_5f2e | (uint)uVar4;
  if (uVar5 == 0x0) {
    uStack6 = 0x0;
  }
  else {
    uVar4->field_0xa4 = 0x0;
    uVar4->field_0xa8 = 0x0;
    uVar4->field_0xac = 0x0;
    uStack6 = uStack10;
    uVar3 = uStack10;
  }
  puVar6 = (undefined4 *)uVar3;
  pass1_1030_4c06(*(ulong *)((int)param_1 + 0x52),param_2,uVar5,unaff_SS);
  uVar8 = (undefined2)(uStack6 >> 0x10);
  puVar7 = (undefined4 *)uStack6;
  for (iVar4 = 0x2b; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
    puVar2 = puVar7;
    puVar7 = puVar7 + 0x1;
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    *puVar2 = *puVar1;
  }
  *(undefined2 *)puVar7 = *(undefined2 *)puVar6;
  return;
}



void __stdcall16far
pass1_1028_e198(ulong param_1,ushort *param_2,ushort *param_3,ulong param_4,ushort param_5,ushort param_6)

{
  pass1_1028_e1ec(param_1,(ushort)param_4,(uint)(param_4 >> 0x10));
  pass1_1030_5b1c(CONCAT22(param_6,param_5),param_2,param_3);
  return;
}



void __stdcall16far bad_1028_e1bc(ulong param_1,ulong param_2)

{
  return;
}



void __stdcall16far pass1_1028_e1ec(ulong param_1,ushort param_2,uint param_3)

{
  if (param_3._1_1_ == '\0') {
    return;
  }
  if (param_3._1_1_ == -0x1) {
    return;
  }
  bad_1030_1312();
  return;
}



void __stdcall16far send_msg_1028_e242(ulong *param_1,int param_2,HWND16 param_3)

{
  uchar *puVar1;
  int unaff_DI;
  ushort unaff_SS;
  LRESULT LVar2;
  
  puVar1 = (uchar *)(*param_1 % 0x64);
  if (*param_1 % 0x64 == 0x0) {
    LVar2 = SendMessage16(param_3,0x0,0x0,0x410000);
    puVar1 = (uchar *)((ulong)LVar2 >> 0x10);
  }
  *param_1 = *param_1 + 0x1;
  if (param_2 != 0x0) {
    pass1_1028_e28a(puVar1,unaff_DI,unaff_SS);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_e28a(uchar *param_1,int param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 *puVar2;
  undefined2 uVar3;
  ushort *puVar4;
  
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,param_1,param_2);
  uVar3 = (undefined2)((ulong)puVar4 >> 0x10);
  puVar2 = (undefined4 *)((int)puVar4 + 0xa);
  ppcVar1 = (code **)((int)*puVar2 + 0x4);
  (**ppcVar1)(0x1010,puVar2,uVar3,0x5);
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1028_e2ac(ulong param_1,uint param_2)

{
  ulong uStack6;
  
  uStack6 = *(ulong *)((int)param_1 + (param_2 >> 0x8) * 0x4 + 0x2e);
  (*(code *)uStack6)();
  return;
}



ulong __stdcall16far pass1_1028_e2e0(ulong param_1,ushort param_2,byte param_3)

{
  int iVar1;
  undefined2 uVar2;
  ulong uVar3;
  undefined2 auStack10 [0x3];
  uint uStack4;
  
  uStack4 = (uint)param_3;
  if (uStack4 == 0xff) {
    uVar3 = pass1_1028_ebee(param_1,0x0,param_2);
    return uVar3;
  }
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1 + 0x2e;
  auStack10[0] = *(undefined2 *)(iVar1 + uStack4 * 0x4 + 0x2);
  uVar3 = (**(code **)(iVar1 + uStack4 * 0x4))();
  return uVar3;
}
