
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_b872(ushort param_1,uchar param_2,ulong param_3,ulong param_4)

{
  ushort uVar1;
  ushort uVar2;
  ushort uVar3;
  ulong *puVar4;
  undefined *puVar5;
  undefined4 *puVar6;
  ushort *puVar7;
  ushort uVar8;
  undefined local_136 [0x124];
  ulong local_12;
  int local_c;
  int local_a;
  undefined4 local_8;
  undefined2 uStack4;
  
  uVar8 = (ushort)(param_4 >> 0x10);
  puVar6 = (undefined4 *)pass1_1030_5b5c((int)param_4,uVar8);
  local_8 = *puVar6;
  uStack4 = *(undefined2 *)((int)puVar6 + 0x4);
  pass1_1008_3e94((ushort *)CONCAT22(param_1,&local_8),(ushort *)CONCAT22(param_1,&local_c),
                  (ushort *)CONCAT22(param_1,&local_a));
  uVar1 = local_a - 0xa;
  pass1_1008_612e(0xa,uVar1,uVar1);
  uVar2 = local_c - 0xa;
  pass1_1008_612e(0xa,uVar2,uVar2);
  puVar7 = pass1_1008_3e54((ushort *)CONCAT22(param_1,&local_12),0x0,uVar2,uVar1);
  uVar1 = (ushort)((ulong)puVar7 >> 0x10);
  while( true ) {
    puVar4 = &local_12;
    pass1_1020_b482(param_1,param_3,(undefined4 *)CONCAT22(param_1,puVar4),param_4);
    if (puVar4 != (ulong *)0x0) break;
    uVar2 = local_a - 0xa;
    pass1_1008_612e(0xa,uVar2,uVar2);
    uVar3 = local_c - 0xa;
    pass1_1008_612e(0xa,uVar3,uVar3);
    pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_12),0x0,uVar3,uVar2);
  }
  struct_op_1028_8888(param_1,param_2,(astruct_100 *)CONCAT22(param_1,local_136),0x0,0xa,&local_12,param_1,0x8000002,0x0
                      ,*(ulong *)((int)param_4 + 0x4));
  puVar5 = local_136;
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,puVar5));
  pass1_1020_b97e(param_1,puVar5,uVar1,(ushort)param_3,(ushort)(param_3 >> 0x10),0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_b97e(ushort param_1,int param_2,ushort param_3,ushort param_4,ushort param_5,int param_6)

{
  undefined4 uVar1;
  int local_e;
  ushort local_c;
  int iStack10;
  ushort uStack8;
  undefined4 uStack6;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x2,0x400);
  _PTR_LOOP_1050_4e70 = CONCAT22(param_3,param_2);
  uVar1 = *(undefined4 *)(param_2 + 0x10);
  uStack6 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  iStack10 = (int)uVar1;
  uStack8 = param_3;
  pass1_1008_3f62((ushort *)&PTR_LOOP_1048_4230,(ushort *)CONCAT22(param_3,iStack10 + 0xc));
  pass1_1008_3e94((ushort *)&PTR_LOOP_1048_4230,(ushort *)CONCAT22(param_1,&local_e),
                  (ushort *)CONCAT22(param_1,&local_c));
  if (param_6 == 0x0) {
    pass1_1008_3e76((ushort *)&PTR_LOOP_1048_4230,0x0,local_e + 0x1,local_c - 0x1);
    pass1_1008_3e94((ushort *)&PTR_LOOP_1048_4230,(ushort *)CONCAT22(param_1,&local_e),
                    (ushort *)CONCAT22(param_1,&local_c));
  }
  pass1_1008_3e76((ushort *)0x10484236,0x1,local_e - 0x2,local_c);
  return;
}



void __cdecl16far pass1_1020_ba2b(void)

{
  init_globals_1020_96d4();
  pass1_1020_a426();
  return;
}



void __stdcall16far pass1_1020_ba3e(long *param_1,ushort param_2,int param_3,ushort param_4,ushort param_5)

{
  astruct_172 *iVar1;
  undefined2 uVar1;
  ushort unaff_SS;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_172 *)param_1;
  *param_1 = 0x0;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x6 = param_3;
  iVar1->field_0x8 = param_2;
  if (iVar1->field_0x6 == 0x0) {
    iVar1->field_0x6 = 0x5;
  }
  pass1_1020_bcc4(param_1,param_4,unaff_SS);
  return;
}



void __stdcall16far fn_ptr_1020_ba7e(ulong *param_1)

{
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  return;
}



void __stdcall16far pass1_1020_ba94(long *param_1)

{
  uint *puVar1;
  uint uStack8;
  
  if (*param_1 == 0x0) {
    return;
  }
  uStack8 = 0x0;
  while( true ) {
    puVar1 = (uint *)((int)param_1 + 0x4);
    if (*puVar1 < uStack8 || *puVar1 == uStack8) break;
    uStack8 = uStack8 + 0x1;
  }
  return;
}



undefined4 __stdcall16far pass1_1020_bae6(ushort param_1,ulong param_2,uint param_3,uint param_4,ushort param_5)

{
  undefined2 *puStack6;
  
  pass1_1020_bc92((uint *)CONCAT22((int)param_2,param_1),(ushort)(param_2 >> 0x10),param_5);
  puStack6 = (undefined2 *)CONCAT22(param_4,param_3);
  if ((param_4 | param_3) != 0x0) {
    return CONCAT22(*(undefined2 *)(param_3 + 0x2),*puStack6);
  }
  return 0x0;
}



void __stdcall16far pass1_1020_bb16(ulong *param_1,ulong *param_2,ushort *param_3,uint param_4)

{
  if (*(uint *)((int)param_1 + 0x4) < param_4) {
    *param_3 = 0x0;
    *param_2 = 0x0;
    return;
  }
  *param_3 = *(ushort *)(param_4 * 0x6 + (int)*param_1 + 0x4);
  *param_2 = *(ulong *)((int)*param_1 + param_4 * 0x6);
  return;
}



void __stdcall16far
pass1_1020_bb70(long *param_1,uint param_2,undefined4 param_3,ushort param_4,undefined2 param_5,ushort param_6)

{
  pass1_1020_bba4(param_1,0x1,param_2,(int)param_3,(ushort)((ulong)param_3 >> 0x10),param_4,param_6);
  return;
}



void __stdcall16far pass1_1020_bb8a(long *param_1,uint param_2,undefined4 param_3,ushort param_4,ushort param_5)

{
  pass1_1020_bba4(param_1,0x0,param_2,(int)param_3,(ushort)((ulong)param_3 >> 0x10),param_4,param_5);
  return;
}



BOOL16 __stdcall16far
pass1_1020_bba4(long *param_1,int param_2,uint param_3,int param_4,ushort param_5,ushort param_6,ushort param_7)

{
  uint *in_AX;
  uint in_DX;
  uint uVar1;
  uint uVar2;
  bool bVar3;
  uint *puStack6;
  
  pass1_1020_bc92((uint *)param_1,param_5,param_7);
  puStack6 = (uint *)CONCAT22(in_DX,in_AX);
  uVar1 = in_DX | (uint)in_AX;
  if (uVar1 == 0x0) {
    pass1_1020_bc92((uint *)param_1,0x0,param_7);
    uVar2 = uVar1 | (uint)in_AX;
    if (uVar2 == 0x0) {
      pass1_1020_bcc4(param_1,param_6,param_7);
      pass1_1020_bc92((uint *)param_1,0x0,param_7);
      if ((uVar2 | (uint)in_AX) == 0x0) {
        return 0x0;
      }
      in_AX[0x2] = param_5;
      uVar1 = uVar2;
    }
    else {
      in_AX[0x2] = param_5;
    }
    if (param_2 != 0x0) {
      uVar2 = *in_AX;
      bVar3 = CARRY2(uVar2,param_3);
      param_3 = uVar2 + param_3;
      param_4 = in_AX[0x1] + param_4 + (uint)bVar3;
    }
    *in_AX = param_3;
    in_AX[0x1] = param_4;
    pass1_1020_bc72((uint *)param_1,param_6,param_7);
  }
  else {
    if (param_2 != 0x0) {
      bVar3 = CARRY2(*puStack6,param_3);
      param_3 = *puStack6 + param_3;
      param_4 = in_AX[0x1] + param_4 + (uint)bVar3;
    }
    *in_AX = param_3;
    in_AX[0x1] = param_4;
  }
  return 0x1;
}



void __stdcall16far pass1_1020_bc72(uint *param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  ushort uVar2;
  
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x2);
  pass1_1000_4aea(*param_1,(uint)uVar1,(int)((ulong)uVar1 >> 0x10),0x6,(uchar *)0xbd6c,(int)&stack0xfffe,param_2,uVar2,
                  0x1000,param_3);
  return;
}



void __stdcall16far pass1_1020_bc92(uint *param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  undefined local_c [0x4];
  ushort uStack8;
  
  uStack8 = param_2;
  uVar1 = *(undefined4 *)((int)param_1 + 0x2);
  pass1_1000_49c6((ushort)local_c,param_3,*param_1,(uint)uVar1,(uint)((ulong)uVar1 >> 0x10),0x6,(uchar *)0xbd6c,
                  (int)&stack0xfffe);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_bcc4(long *param_1,ushort param_2,ushort param_3)

{
  uint *puVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  long lVar6;
  long lStack12;
  
  uVar5 = (uint)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(int *)(iVar4 + 0x4) == 0x0) {
    PTR_LOOP_1050_5f2e = (undefined *)0x0;
    iVar2 = *(int *)(iVar4 + 0x6);
  }
  else {
    uVar3 = *(uint *)(iVar4 + 0x4);
    puVar1 = (uint *)(iVar4 + 0x8);
    iVar2 = uVar3 + *puVar1;
    PTR_LOOP_1050_5f2e = (undefined *)(uint)CARRY2(uVar3,*puVar1);
  }
  if (PTR_LOOP_1050_5f2e == (undefined *)0x0) {
    if (*param_1 == 0x0) {
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(0x0,0x1000);
      }
      else {
      }
      uVar3 = fn_ptr_op_1000_1708(iVar2 * 0x6,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
      lVar6 = pass1_1000_0ed4(0x1000,param_3,0x1,iVar2 * 0x6,0x0,(ushort *)*param_1,(ushort)((ulong)*param_1 >> 0x10));
      PTR_LOOP_1050_5f2e = (undefined *)((ulong)lVar6 >> 0x10);
      uVar3 = (uint)lVar6;
    }
    lStack12 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    if (((uint)PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
      *(int *)(iVar4 + 0x4) = iVar2;
      *param_1 = lStack12;
      pass1_1020_bc72((uint *)((ulong)param_1 & 0xffff | (ulong)uVar5 << 0x10),param_2,param_3);
    }
  }
  return;
}



int __cdecl16far pass1_1020_bd6c(ulong param_1,ulong param_2)

{
  return *(int *)((int)param_1 + 0x4) - *(int *)((int)param_2 + 0x4);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __cdecl16far pass1_1020_bd80(ushort param_1)

{
  char *pcVar1;
  ushort uStack6;
  
  switch(param_1) {
  case 0x1:
  case 0x6:
    break;
  case 0x2:
    break;
  case 0x3:
  case 0x7:
    break;
  case 0x4:
  case 0x8:
    break;
  case 0x5:
  case 0x9:
    break;
  case 0xa:
    break;
  case 0xb:
  case 0x37:
    break;
  case 0xc:
  case 0x35:
  case 0x36:
    break;
  case 0xd:
    break;
  case 0xe:
    break;
  case 0xf:
    break;
  case 0x10:
    break;
  case 0x11:
    break;
  case 0x12:
    break;
  case 0x13:
  case 0x14:
  case 0x15:
    break;
  case 0x16:
  case 0x19:
    break;
  case 0x17:
  case 0x1a:
    break;
  case 0x18:
    break;
  case 0x1b:
  case 0x1c:
  case 0x1d:
    break;
  case 0x1e:
  case 0x1f:
  case 0x20:
    break;
  case 0x21:
    break;
  case 0x22:
  case 0x23:
  case 0x24:
    break;
  case 0x25:
  case 0x26:
  case 0x27:
    break;
  case 0x28:
  case 0x29:
    break;
  case 0x2a:
  case 0x2b:
    break;
  case 0x2c:
    break;
  case 0x2d:
  case 0x2e:
    break;
  case 0x2f:
  case 0x30:
    break;
  case 0x31:
  case 0x32:
    break;
  case 0x33:
  case 0x34:
    break;
  case 0x38:
  case 0x39:
    break;
  case 0x3a:
  case 0x3b:
    break;
  case 0x3c:
  case 0x3d:
    break;
  case 0x3e:
    break;
  case 0x3f:
    break;
  case 0x40:
    break;
  case 0x41:
    break;
  case 0x42:
  case 0x46:
  case 0x6b:
    break;
  case 0x43:
    uStack6 = (ushort)s_bidLRoadConst_1050_4e7a;
    return uStack6;
  case 0x44:
    uStack6 = (ushort)s_bidRRoadConst_1050_4e88;
    return uStack6;
  case 0x45:
    uStack6 = (ushort)s_bidXRoadConst_1050_4e96;
    return uStack6;
  case 0x47:
    break;
  case 0x48:
  case 0x49:
  case 0x4a:
  case 0x70:
  case 0x71:
  case 0x72:
    break;
  case 0x4b:
    break;
  case 0x4c:
    break;
  case 0x4d:
    break;
  case 0x4e:
    break;
  case 0x4f:
  case 0x50:
  case 0x51:
    break;
  case 0x52:
  case 0x53:
    break;
  case 0x54:
  case 0x55:
  case 0x56:
    break;
  case 0x57:
  case 0x58:
  case 0x59:
    break;
  case 0x5a:
    break;
  case 0x5b:
  case 0x5c:
    break;
  case 0x5d:
  case 0x5e:
  case 0x5f:
    break;
  case 0x60:
  case 0x61:
    break;
  case 0x62:
  case 0x63:
    break;
  case 0x64:
    break;
  case 0x65:
    break;
  case 0x66:
  case 0x67:
    break;
  case 0x68:
  case 0x69:
    break;
  case 0x6a:
    break;
  case 0x6c:
  case 0x6d:
    break;
  case 0x6e:
    break;
  case 0x6f:
    break;
  case 0x73:
  case 0x77:
    break;
  case 0x74:
  case 0x78:
  case 0x79:
    break;
  case 0x75:
    break;
  case 0x76:
    break;
  case 0x7a:
    break;
  case 0x7b:
    break;
  case 0x7c:
    break;
  case 0x7d:
    break;
  case 0x7e:
    break;
  case 0x7f:
    break;
  case 0x80:
    break;
  case 0x81:
    break;
  case 0x82:
    break;
  case 0x83:
    break;
  case 0x84:
    break;
  case 0x85:
    break;
  case 0x86:
    break;
  case 0x87:
    break;
  case 0x88:
    break;
  case 0x89:
  }
  pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  return (ushort)pcVar1;
}



void __cdecl16far string_1020_c0ca(ushort param_1)

{
  string_1020_c0d8(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * __cdecl16far string_1020_c0d8(ushort param_1)

{
  char *pcVar1;
  
  switch(param_1) {
  case 0x1:
    break;
  case 0x2:
    break;
  case 0x3:
    break;
  case 0x4:
    break;
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x7:
    break;
  case 0x8:
    break;
  case 0x9:
    break;
  case 0xa:
    break;
  case 0xb:
    break;
  case 0xc:
    break;
  case 0xd:
    break;
  case 0xe:
    break;
  case 0xf:
    break;
  case 0x10:
    break;
  case 0x11:
    break;
  case 0x12:
    break;
  case 0x13:
    break;
  case 0x14:
    break;
  case 0x15:
    break;
  case 0x16:
    break;
  case 0x17:
    break;
  case 0x18:
    break;
  case 0x19:
    break;
  case 0x1a:
    break;
  case 0x1b:
    break;
  case 0x1c:
    break;
  case 0x1d:
    break;
  case 0x1e:
    break;
  case 0x1f:
    break;
  case 0x21:
    break;
  case 0x23:
    break;
  case 0x24:
  }
  pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  return (char *)pcVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * __cdecl16far string_op_1020_c222(ushort param_1)

{
  char *pcVar1;
  
  switch(param_1) {
  case 0x1:
    break;
  case 0x2:
    break;
  case 0x3:
    break;
  case 0x4:
    break;
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x7:
    break;
  case 0x8:
    break;
  case 0x9:
    break;
  case 0xa:
    break;
  case 0xb:
    break;
  case 0xc:
    break;
  case 0xd:
    break;
  case 0xe:
    break;
  case 0xf:
    break;
  case 0x10:
    break;
  case 0x11:
    break;
  case 0x12:
    break;
  case 0x13:
    break;
  case 0x14:
  }
  pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  return (char *)pcVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * __cdecl16far string_op_1020_c2f8(ushort param_1)

{
  char *pcVar1;
  
  switch(param_1) {
  case 0x1:
    break;
  case 0x2:
    break;
  case 0x3:
    break;
  case 0x4:
    break;
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x7:
    break;
  case 0x8:
    break;
  case 0x9:
    break;
  case 0xa:
    break;
  case 0xb:
    break;
  case 0xc:
    break;
  case 0xd:
    break;
  case 0xe:
    break;
  case 0xf:
    break;
  case 0x10:
  }
  pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  return (char *)pcVar1;
}



ushort __cdecl16far pass1_1020_c3ae(void)

{
  return 0x1;
}



ushort __cdecl16far switch_1020_c3b4(ushort param_1)

{
  ushort uStack6;
  
  uStack6 = 0x1;
  switch(param_1) {
  case 0x1:
  case 0x2:
  case 0x3:
  case 0x5:
  case 0x8:
  case 0x9:
  case 0xa:
  case 0xb:
  case 0xc:
    uStack6 = 0x3;
    break;
  case 0x4:
    uStack6 = 0x6;
    break;
  case 0x6:
  case 0xf:
  case 0x10:
  case 0x11:
  case 0x12:
  case 0x13:
    uStack6 = 0xa;
    break;
  case 0x7:
    uStack6 = 0x2;
    break;
  case 0xd:
  case 0xe:
    uStack6 = 0x1;
  }
  return uStack6;
}



ushort __cdecl16far pass1_1020_c42e(int param_1)

{
  ushort uVar1;
  
  if (param_1 == 0xf) {
    uVar1 = 0x1;
  }
  else {
    uVar1 = 0x3;
  }
  return uVar1;
}



void __stdcall16far struct_1020_c444(astruct_75 *param_1,ulong param_2,ulong param_3)

{
  astruct_75 *iVar1;
  astruct_75 *uVar1;
  
  struct_op_1030_1cd8(param_1,param_2,param_3);
  uVar1 = (astruct_75 *)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_75 *)param_1;
  *(undefined4 *)(iVar1 + 0x1) = 0x0;
  *(undefined2 *)&iVar1[0x1].field_0x4 = 0x0;
  param_1->field_0x0 = 0xc834;
  iVar1->field_0x2 = 0x1020;
  return;
}



void __stdcall16far pass1_1020_c47a(ushort *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0xc834;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0x18),0x1000);
  pass1_1030_1d28(param_1);
  return;
}



void __stdcall16far
pass1_1020_c4a8(ulong param_1,ushort *param_2,ulong *param_3,int param_4,ushort param_5,ushort param_6)

{
  undefined4 uVar1;
  ulong *puVar2;
  uint uVar3;
  undefined2 uVar4;
  
  uVar3 = (uint)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x1c) != 0x0) {
    pass1_1020_c6a4(param_1 & 0xffff | (ulong)uVar3 << 0x10,param_5,param_6);
  }
  uVar1 = *(undefined4 *)((int)param_1 + 0x18);
  uVar4 = (undefined2)((ulong)uVar1 >> 0x10);
  puVar2 = (ulong *)((int)uVar1 + param_4 * 0x6);
  *param_3 = *puVar2;
  *param_2 = *(ushort *)(puVar2 + 0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_c4f4(ulong param_1,ushort param_2,ushort param_3,ulong param_4,astruct_361 *param_5,uint param_6)

{
  astruct_361 *paVar1;
  ushort uVar2;
  uint uVar3;
  
  pass1_1020_c6de(param_1,param_4);
  uVar3 = param_6 | (uint)param_5;
  if (uVar3 != 0x0) {
    paVar1 = param_5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_4,(uint)(param_4 >> 0x10));
    uVar2 = pass1_1030_6fa0(CONCAT22(uVar3,paVar1));
    param_5->field_0x4 = *(undefined2 *)(uVar2 * 0x2 + 0x4ea4);
  }
  return;
}



ulong __stdcall16far pass1_1020_c538(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0x12),*(undefined2 *)((int)param_1 + 0x10));
}



void __stdcall16far pass1_1020_c54a(ulong param_1,int param_2,ushort param_3)

{
  uint uVar1;
  
  uVar1 = (uint)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x1c) != 0x0) {
    pass1_1020_c6a4(param_1 & 0xffff | (ulong)uVar1 << 0x10,param_2,param_3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_c5b8(ushort param_1,uint param_2,int param_3,ushort param_4)

{
  long *plVar1;
  undefined4 uVar2;
  code **ppcVar3;
  undefined4 *puVar4;
  uint uVar5;
  uint extraout_DX;
  uint uVar6;
  int iVar7;
  undefined2 uVar8;
  
  uVar2 = *(undefined4 *)(param_3 + 0xa);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)((ulong)uVar2 >> 0x10));
  uVar5 = pass1_1030_6fa0(CONCAT22(param_2,param_1));
  *(ushort *)(param_3 + -0x6) = uVar5;
  pass1_1020_c6de(*(ulong *)(param_3 + 0x6),0x0);
  *(uint *)(param_3 + -0xa) = uVar5;
  *(uint *)(param_3 + -0x8) = param_2;
  if ((param_2 | uVar5) == 0x0) {
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_3 + 0x6) + 0x20);
    (**ppcVar3)();
    uVar6 = extraout_DX;
    pass1_1020_c6de(*(ulong *)(param_3 + 0x6),0x0);
    *(uint *)(param_3 + -0xa) = uVar5;
    *(uint *)(param_3 + -0x8) = uVar6;
    if ((uVar6 | uVar5) == 0x0) {
      return;
    }
  }
  uVar2 = *(undefined4 *)(param_3 + 0x6);
  uVar8 = (undefined2)((ulong)uVar2 >> 0x10);
  iVar7 = (int)uVar2;
  *(undefined2 *)(iVar7 + 0x1c) = 0x1;
  plVar1 = (long *)(iVar7 + 0x8);
  *plVar1 = *plVar1 + 0x1;
  puVar4 = (undefined4 *)*(undefined4 *)(param_3 + -0xa);
  *puVar4 = *(undefined4 *)(param_3 + 0xa);
  *(undefined2 *)((int)puVar4 + 0x4) = *(undefined2 *)(*(int *)(param_3 + -0x6) * 0x2 + 0x4ea4);
  return;
}



void __stdcall16far pass1_1020_c644(ulong *param_1,ushort param_2,ulong param_3)

{
  long *plVar1;
  undefined2 uVar2;
  code **ppcVar3;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  ulong *puStack6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(long *)(iVar5 + 0x18) == 0x0) {
    ppcVar3 = (code **)((int)*param_1 + 0x20);
    (**ppcVar3)();
  }
  iVar4 = *(int *)(iVar5 + 0x8) * 0x6 + *(int *)(iVar5 + 0x18);
  uVar2 = *(undefined2 *)(iVar5 + 0x1a);
  puStack6 = (ulong *)CONCAT22(uVar2,iVar4);
  plVar1 = (long *)(iVar5 + 0x8);
  *plVar1 = *plVar1 + 0x1;
  *puStack6 = param_3;
  *(ushort *)(iVar4 + 0x4) = param_2;
  return;
}



void __stdcall16far pass1_1020_c694(ulong param_1,int param_2,ushort param_3)

{
  pass1_1020_c6a4(param_1,param_2,param_3);
  return;
}



void __stdcall16far pass1_1020_c6a4(ulong param_1,int param_2,ushort param_3)

{
  long lVar1;
  astruct_359 *iVar2;
  ushort uVar2;
  
  uVar2 = (ushort)(param_1 >> 0x10);
  iVar2 = (astruct_359 *)param_1;
  if ((iVar2->field_0x18 != 0x0) && (iVar2->field_0x8 != 0x0)) {
    lVar1 = iVar2->field_0x18;
    pass1_1000_4aea((uint)lVar1,(uint)((ulong)lVar1 >> 0x10),iVar2->field_0x10,0x6,(uchar *)0xc7fa,(int)&stack0xfffe,
                    param_2,uVar2,0x1000,param_3);
    iVar2->field_0x1c = 0x0;
  }
  return;
}



void __stdcall16far pass1_1020_c6de(ulong param_1,long param_2)

{
  ulong *puVar1;
  undefined4 uVar2;
  astruct_360 *iVar3;
  int unaff_DI;
  uint uVar3;
  ushort unaff_SS;
  ulong uStack6;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar3 = (astruct_360 *)param_1;
  if (iVar3->field_0x1c != 0x0) {
    pass1_1020_c6a4(param_1 & 0xffff | (ulong)uVar3 << 0x10,unaff_DI,unaff_SS);
  }
  uStack6 = 0x0;
  while( true ) {
    puVar1 = &iVar3->field_0x10;
    if (*puVar1 < uStack6 || *puVar1 == uStack6) {
      return;
    }
    uVar2 = iVar3->field_0x18;
    if (*(long *)((int)uVar2 + (int)uStack6 * 0x6) == param_2) break;
    uStack6 = uStack6 + 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_c73a(ulong param_1,ushort param_2)

{
  uint *puVar1;
  uint uVar2;
  undefined4 uVar3;
  uint uVar4;
  int iVar5;
  undefined2 uVar6;
  long lVar7;
  undefined4 uStack10;
  undefined4 uStack6;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(long *)(iVar5 + 0x10) == 0x0) {
    uVar4 = *(uint *)(iVar5 + 0xc);
    PTR_LOOP_1050_5f2e = (undefined *)*(undefined2 *)(iVar5 + 0xe);
  }
  else {
    uVar2 = *(uint *)(iVar5 + 0x10);
    puVar1 = (uint *)(iVar5 + 0x14);
    uVar4 = uVar2 + *puVar1;
    PTR_LOOP_1050_5f2e = (undefined *)(*(int *)(iVar5 + 0x12) + *(int *)(iVar5 + 0x16) + (uint)CARRY2(uVar2,*puVar1));
  }
  uStack6 = CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
  if (*(long *)(iVar5 + 0x18) == 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(uVar4 * 0x6,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
    uVar3 = *(undefined4 *)(iVar5 + 0x18);
    lVar7 = pass1_1000_0ed4(0x1000,param_2,0x1,uVar4 * 0x6,
                            ((int)PTR_LOOP_1050_5f2e * 0x3 + (uint)CARRY2(uVar4,uVar4) + (uint)CARRY2(uVar4 * 0x2,uVar4)
                            ) * 0x2 + (uint)CARRY2(uVar4 * 0x3,uVar4 * 0x3),(ushort *)uVar3,
                            (ushort)((ulong)uVar3 >> 0x10));
    PTR_LOOP_1050_5f2e = (undefined *)((ulong)lVar7 >> 0x10);
    uVar4 = (uint)lVar7;
  }
  uStack10 = CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
  if (((uint)PTR_LOOP_1050_5f2e | uVar4) != 0x0) {
    *(undefined4 *)(iVar5 + 0x10) = uStack6;
    *(undefined4 *)(iVar5 + 0x18) = uStack10;
  }
  *(undefined2 *)(iVar5 + 0x1c) = 0x1;
  return;
}
