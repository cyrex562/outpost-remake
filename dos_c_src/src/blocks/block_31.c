
void __stdcall16far struct_1010_6326(astruct_630 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  param_1->field_0x1a = 0x0;
  param_1->field_0x1e = 0x0;
  param_1->field_0x22 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x66f0;
  param_1->field_0x2 = 0x1010;
  return;
}



void __stdcall16far write_to_file_1010_6372(ulong param_1,ulong param_2,uint16_t param_3)

{
  BOOL16 BVar1;
  astruct_729 *iVar2;
  undefined2 uVar2;
  ushort uVar3;
  ushort uVar4;
  undefined4 local_10 [0x2];
  undefined4 local_8;
  
  BVar1 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar1 != 0x0) {
    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar2 = (astruct_729 *)param_1;
    local_10[0] = iVar2->field_0xa;
    uVar3 = (ushort)param_2;
    uVar4 = (ushort)(param_2 >> 0x10);
    BVar1 = write_to_file_1008_7e1c(uVar3,uVar4,(ushort)local_10,param_3,(char *)0x4,0x1008);
    if (BVar1 != 0x0) {
      local_8 = iVar2->field_0xe;
      BVar1 = write_to_file_1008_7e1c(uVar3,uVar4,(ushort)&local_8,param_3,(char *)0x4,0x1008);
      if (BVar1 != 0x0) {
        local_8 = iVar2->field_0x12;
        BVar1 = write_to_file_1008_7e1c(uVar3,uVar4,(ushort)&local_8,param_3,(char *)0x4,0x1008);
        if (BVar1 != 0x0) {
          local_8 = iVar2->field_0x16;
          BVar1 = write_to_file_1008_7e1c(uVar3,uVar4,(ushort)&local_8,param_3,(char *)0x4,0x1008);
          if (BVar1 != 0x0) {
            local_8 = iVar2->field_0x1a;
            BVar1 = write_to_file_1008_7e1c(uVar3,uVar4,(ushort)&local_8,param_3,(char *)0x4,0x1008);
            if (BVar1 != 0x0) {
              local_8 = iVar2->field_0x1e;
              BVar1 = write_to_file_1008_7e1c(uVar3,uVar4,(ushort)&local_8,param_3,(char *)0x4,0x1008);
              if (BVar1 != 0x0) {
                local_8 = iVar2->field_0x22;
                BVar1 = write_to_file_1008_7e1c(uVar3,uVar4,(ushort)&local_8,param_3,(char *)0x4,0x1008);
                if (BVar1 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



void __stdcall16far pass1_1010_648a(ulong param_1,ulong param_2,int param_3,uint16_t param_4)

{
  ushort uVar1;
  int iVar2;
  BOOL16 BVar3;
  ushort uVar4;
  ushort uVar5;
  
  uVar4 = (ushort)param_2;
  uVar5 = (ushort)(param_2 >> 0x10);
  read_file_1008_7cfe(uVar4,uVar5,0x7,0x1008,param_4);
  if (param_3 != 0x0) {
    iVar2 = (int)param_1;
    uVar1 = (ushort)(param_1 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0xa,0x0,uVar1,0x4,0x1008);
    if (BVar3 != 0x0) {
      BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0xe,0x0,uVar1,0x4,0x1008);
      if (BVar3 != 0x0) {
        BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x12,0x0,uVar1,0x4,0x1008);
        if (BVar3 != 0x0) {
          BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x16,0x0,uVar1,0x4,0x1008);
          if (BVar3 != 0x0) {
            BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x1a,0x0,uVar1,0x4,0x1008);
            if (BVar3 != 0x0) {
              BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x1e,0x0,uVar1,0x4,0x1008);
              if (BVar3 != 0x0) {
                BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x22,0x0,uVar1,0x4,0x1008);
                if (BVar3 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



void __stdcall16far pass1_1010_6566(ulong param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  ushort uVar1;
  ushort uVar2;
  int local_4;
  
  uVar1 = (ushort)param_1;
  uVar2 = (ushort)(param_1 >> 0x10);
  switch_1010_6646(uVar1,uVar2,(ushort *)CONCAT22(param_5,&local_4),param_4);
  if (local_4 != 0x0) {
    *(ushort *)(uVar1 + local_4) = param_3;
    *(ushort *)(uVar1 + local_4 + 0x2) = param_2;
  }
  return;
}



int __stdcall16far pass1_1010_659a(ulong param_1,ushort param_2,ushort param_3)

{
  ushort uVar1;
  ushort uVar2;
  int local_4;
  
  uVar1 = (ushort)param_1;
  uVar2 = (ushort)(param_1 >> 0x10);
  switch_1010_6646(uVar1,uVar2,(ushort *)CONCAT22(param_3,&local_4),param_2);
  if (local_4 == 0x0) {
    return 0x0;
  }
  return *(int *)(uVar1 + local_4) - *(int *)(uVar1 + local_4 + 0x2);
}



ushort __stdcall16far pass1_1010_65d0(ushort param_1,ulong param_2,ushort param_3)

{
  ushort uVar1;
  int local_4;
  
  uVar1 = (ushort)(param_2 >> 0x10);
  switch_1010_6646((ushort)param_2,uVar1,(ushort *)CONCAT22(param_1,&local_4),param_3);
  if (local_4 == 0x0) {
    return 0x0;
  }
  return *(ushort *)((ushort)param_2 + local_4 + 0x2);
}



void __stdcall16far pass1_1010_6604(ulong param_1,ushort param_2,ushort param_3)

{
  int iVar1;
  ushort uVar2;
  ushort uVar3;
  int local_4;
  
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)(param_1 >> 0x10);
  switch_1010_6646(uVar2,uVar3,(ushort *)CONCAT22(param_3,&local_4),param_2);
  if (local_4 != 0x0) {
    iVar1 = *(int *)(uVar2 + local_4 + 0x2);
    *(undefined2 *)(uVar2 + local_4) = *(undefined2 *)(uVar2 + local_4);
    *(int *)(uVar2 + local_4 + 0x2) = iVar1 + 0x1;
    pass1_1010_1f62(param_3,param_1 & 0xffff | (ulong)uVar3 << 0x10,0x15);
  }
  return;
}



void __stdcall16far switch_1010_6646(ushort param_1,ushort param_2,ushort *param_3,ushort param_4)

{
  switch(param_4) {
  case 0x83:
    *param_3 = 0xa;
    break;
  case 0x84:
    *param_3 = 0xe;
    break;
  case 0x85:
    *param_3 = 0x12;
    break;
  case 0x86:
    *param_3 = 0x16;
    return;
  case 0x87:
    *param_3 = 0x1a;
    return;
  case 0x88:
    *param_3 = 0x1e;
    return;
  case 0x89:
    *param_3 = 0x22;
    return;
  default:
    *param_3 = 0x0;
    return;
  }
  return;
}



ushort * __stdcall16far pass1_1010_66ca(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1010_6700(astruct_636 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0x148 = 0x33;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x6aac;
  param_1->field_0x2 = 0x1010;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0xa),(WNDCLASS16 *)0x0,0x114);
  param_1->field_0x32 = 0x1;
  param_1->field_0x40 = 0x1;
  param_1->field_0x46 = 0x1;
  param_1->field_0x4e = 0x1;
  param_1->field_0x54 = 0x1;
  param_1->field_0x5e = 0x1;
  param_1->field_0x68 = 0x1;
  param_1->field_0x6c = 0x1;
  param_1->field_0x74 = 0x1;
  param_1->field_0x78 = 0x1;
  param_1->field_0x7a = 0x1;
  param_1->field_0x7e = 0x1;
  param_1->field_0x82 = 0x1;
  param_1->field_0xa2 = 0x1;
  param_1->field_0xa4 = 0x1;
  param_1->field_0xa6 = 0x1;
  param_1->field_0xa8 = 0x1;
  param_1->field_0xae = 0x1;
  param_1->field_0xb2 = 0x1;
  param_1->field_0xb8 = 0x1;
  param_1->field_0xbe = 0x1;
  param_1->field_0xc0 = 0x1;
  param_1->field_0xc4 = 0x1;
  param_1->field_0xd4 = 0x1;
  param_1->field_0xda = 0x1;
  param_1->field_0xe2 = 0x1;
  param_1->field_0xfe = 0x1;
  param_1->field_0x100 = 0x1;
  param_1->field_0x102 = 0x1;
  param_1->field_0x104 = 0x1;
  param_1->field_0x106 = 0x1;
  param_1->field_0x108 = 0x1;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0x11e),(WNDCLASS16 *)0x0,0x2a);
  param_1->field_0x120 = 0x1;
  param_1->field_0x122 = 0x1;
  param_1->field_0x124 = 0x1;
  param_1->field_0x126 = 0x1;
  param_1->field_0x128 = 0x1;
  param_1->field_0x12c = 0x1;
  param_1->field_0x138 = 0x1;
  return CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1010_6814(ulong param_1,ushort param_2,int param_3)

{
  *(ushort *)((int)param_1 + param_3 * 0x2 + 0x11e) = param_2;
  return;
}



void __stdcall16far pass1_1010_682e(ulong param_1,ushort param_2,int param_3)

{
  *(ushort *)((int)param_1 + param_3 * 0x2 + 0xa) = param_2;
  return;
}



void __stdcall16far write_to_file_1010_6846(undefined4 param_1,undefined4 param_2,uint16_t param_3)

{
  ushort uVar1;
  BOOL16 BVar2;
  int iVar3;
  ushort uVar4;
  ushort uVar5;
  undefined2 local_c [0x5];
  
  BVar2 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar2 != 0x0) {
    iVar3 = (int)param_1;
    uVar1 = (ushort)((ulong)param_1 >> 0x10);
    uVar4 = (ushort)param_2;
    uVar5 = (ushort)((ulong)param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,iVar3 + 0xa,uVar1,(char *)0x114,0x1008);
    if (BVar2 != 0x0) {
      BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,iVar3 + 0x11e,uVar1,(char *)0x2a,0x1008);
      if (BVar2 != 0x0) {
        local_c[0] = *(undefined2 *)(iVar3 + 0x148);
        BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)local_c,param_3,(char *)0x2,0x1008);
        if (BVar2 != 0x0) {
          return;
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



void __stdcall16far pass1_1010_68c6(ulong param_1,ulong param_2,uint param_3,uchar *param_4,uint16_t param_5)

{
  astruct_248 *iVar2;
  BOOL16 BVar1;
  int iVar3;
  ushort uVar4;
  ushort uVar5;
  uchar *puVar6;
  ushort uVar7;
  ushort uVar8;
  SEGPTR SVar9;
  ushort uVar10;
  astruct_18 *paStack18;
  astruct_18 *paStack10;
  int local_6;
  uint uStack4;
  
  uVar8 = (ushort)param_2;
  uVar10 = (ushort)(param_2 >> 0x10);
  read_file_1008_7cfe(uVar8,uVar10,0x3,0x1008,param_5);
  if (param_3 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d4;
    return;
  }
  iVar2 = (astruct_248 *)param_1;
  uVar7 = (ushort)(param_1 >> 0x10);
  if ((int)PTR_LOOP_1050_0312 < 0x2) {
    uVar4 = 0x102;
    SVar9 = 0x102;
    mem_op_1000_179c(0x102,param_4,0x1000);
    paStack10 = (astruct_18 *)CONCAT22(param_4,param_3);
    puVar6 = param_4;
    BVar1 = read_file_1008_7dee(uVar8,uVar10,param_3,uVar4,(ushort)param_4,SVar9,0x1008);
    paStack18 = paStack10;
    if (BVar1 == 0x0) goto LAB_1010_692c;
    uStack4 = 0x1;
    do {
      iVar3 = switch_1008_73ea(uVar8,uVar10,uStack4);
      *(undefined2 *)(&iVar2->field_0xa + iVar3 * 0x2) = *(undefined2 *)(uStack4 * 0x2 + param_3);
      uStack4 = uStack4 + 0x1;
    } while (uStack4 < 0x81);
    fn_ptr_1000_17ce(paStack10,0x1000);
    uVar4 = (ushort)paStack10;
    param_4 = puVar6;
  }
  else {
    uVar4 = read_file_1008_7dee(uVar8,uVar10,&iVar2->field_0xa,0x0,uVar7,0x114,0x1008);
    if (uVar4 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return;
    }
  }
  if ((int)PTR_LOOP_1050_0312 < 0x2) {
    uVar5 = 0x2a;
    SVar9 = 0x2a;
    mem_op_1000_179c(0x2a,param_4,0x1000);
    paStack18 = (astruct_18 *)CONCAT22(param_4,uVar4);
    BVar1 = read_file_1008_7dee(uVar8,uVar10,uVar4,uVar5,(ushort)param_4,SVar9,0x1008);
    if (BVar1 == 0x0) {
LAB_1010_692c:
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      fn_ptr_1000_17ce((astruct_18 *)((ulong)paStack18 & 0xffff | ZEXT24(param_4) << 0x10),0x1000);
      return;
    }
    uStack4 = 0x0;
    do {
      uVar5 = switch_1008_72bc(uVar8,uVar10,uStack4);
      *(undefined2 *)(&iVar2->field_0x11e + uVar5 * 0x2) = *(undefined2 *)(uStack4 * 0x2 + uVar4);
      uStack4 = uStack4 + 0x1;
    } while (uStack4 < 0x15);
    fn_ptr_1000_17ce(paStack18,0x1000);
  }
  else {
    BVar1 = read_file_1008_7dee(uVar8,uVar10,&iVar2->field_0x11e,0x0,uVar7,0x2a,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return;
    }
  }
  BVar1 = read_file_1008_7dee(uVar8,uVar10,(ushort)&local_6,0x0,param_5,0x2,0x1008);
  if (BVar1 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
  BVar1 = switch_1008_73ea(uVar8,uVar10,local_6);
  iVar2->field_0x148 = BVar1;
  return;
}



ushort * __stdcall16far pass1_1010_6a86(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_6abc(astruct_635 *param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  uchar *extraout_DX;
  int unaff_DI;
  ushort unaff_SS;
  astruct_79 *paVar2;
  ushort *puVar3;
  
  paVar2 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x389a;
  param_1->field_0xc = 0x1008;
  param_1->field_0xa = 0x3aa8;
  param_1->field_0xc = 0x1008;
  param_1->field_0xe = 0x0;
  param_1->field_0x10 = 0x0;
  param_1->field_0x14 = (undefined4 *)0x0;
  param_1->field_0x1c = 0x0;
  param_1->field_0x20 = 0x0;
  param_1->field_0x22 = (undefined4 *)0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x7e28;
  param_1->field_0x2 = 0x1010;
  param_1->field_0xa = 0x7e38;
  param_1->field_0xc = 0x1010;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,(uchar *)((ulong)paVar2 >> 0x10),unaff_DI);
  *(int *)&param_1->field_0x14 = (int)puVar3;
  *(undefined2 *)((int)&param_1->field_0x14 + 0x2) = (int)((ulong)puVar3 >> 0x10);
  ppcVar1 = (code **)((int)*param_1->field_0x14 + 0x4);
  (**ppcVar1)();
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,unaff_SS,extraout_DX,unaff_DI);
  *(int *)&param_1->field_0x22 = (int)puVar3;
  *(undefined2 *)((int)&param_1->field_0x22 + 0x2) = (int)((ulong)puVar3 >> 0x10);
  ppcVar1 = (code **)((int)*param_1->field_0x22 + 0x4);
  (**ppcVar1)();
  return;
}



void __stdcall16far pass1_1010_6bb2(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  code **ppcVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  undefined2 *puStack14;
  
  uVar7 = (uint)((ulong)param_1 >> 0x10);
  uVar6 = (uint)param_1;
  *param_1 = 0x7e28;
  *(undefined2 *)(uVar6 + 0x2) = 0x1010;
  *(undefined2 *)(uVar6 + 0xa) = 0x7e38;
  *(undefined2 *)(uVar6 + 0xc) = 0x1010;
  puVar1 = (undefined4 *)*(uint *)(uVar6 + 0x1c);
  uVar3 = *(uint *)(uVar6 + 0x1e);
  if ((uVar3 | (uint)puVar1) != 0x0) {
    ppcVar2 = (code **)*puVar1;
    (**ppcVar2)();
  }
  *(undefined4 *)(uVar6 + 0x1c) = 0x0;
  if (*(long *)(uVar6 + 0x14) != 0x0) {
    uVar3 = uVar7 | uVar6;
    if (param_1 == (ushort *)0x0) {
      uVar5 = 0x0;
    }
    else {
      uVar3 = uVar6 + 0xa;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6(*(ulong *)(uVar6 + 0x14),CONCAT22(uVar5,uVar3),param_2);
  }
  if (*(long *)(uVar6 + 0x22) != 0x0) {
    uVar3 = uVar7 | uVar6;
    if (param_1 == (ushort *)0x0) {
      uVar5 = 0x0;
    }
    else {
      uVar3 = uVar6 + 0xa;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6(*(ulong *)(uVar6 + 0x22),CONCAT22(uVar5,uVar3),param_2);
  }
  *(undefined4 *)(uVar6 + 0x14) = 0x0;
  *(undefined4 *)(uVar6 + 0x22) = 0x0;
  if (param_1 == (ushort *)0x0) {
    iVar4 = 0x0;
    uVar7 = 0x0;
  }
  else {
    iVar4 = uVar6 + 0xa;
  }
  puStack14 = (undefined2 *)CONCAT22(uVar7,iVar4);
  *puStack14 = 0x389a;
  *(undefined2 *)(iVar4 + 0x2) = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



ushort __cdecl16far pass1_1010_6ca2(ulong param_1,int param_2,ushort param_3,ushort param_4)

{
  undefined4 uVar1;
  ushort *puVar2;
  ushort uVar3;
  int iStack10;
  int iStack8;
  
  _iStack8 = (ushort *)CONCAT22(param_4,&stack0x000a);
  iStack10 = param_2;
  do {
    puVar2 = _iStack8;
    if (iStack10 == 0x0) {
      return 0x1;
    }
    _iStack8 = (ushort *)((ulong)_iStack8 & 0xffff0000 | (ulong)(iStack8 + 0x2));
    uVar3 = *puVar2;
    uVar1 = *(undefined4 *)((int)param_1 + 0x14);
    pass1_1010_a5ca((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),uVar3,uVar3,param_3);
    iStack10 = iStack10 + -0x1;
  } while (uVar3 == 0x0);
  return 0x0;
}



ushort pass1_1010_6cf8(ushort param_1,ulong param_2,int param_3,ushort param_4,ushort param_5,ushort param_6,
                      ushort param_7)

{
  ushort uVar1;
  
  switch(param_3) {
  case 0x1:
    pass1_1010_715c(param_2,0x1,param_6,param_5,param_7,param_4);
    send_msg_1010_7c9e(param_2,0x12,param_4);
    return 0x1;
  default:
    return 0x0;
  case 0x4:
    uVar1 = 0x2;
    break;
  case 0x5:
    uVar1 = 0x3;
    break;
  case 0x6:
    uVar1 = 0x4;
    break;
  case 0x7:
    uVar1 = 0x5;
    break;
  case 0x9:
    pass1_1010_715c(param_2,0x6,param_6,param_5,param_7,param_4);
  case 0x2e:
    uVar1 = 0x38;
    break;
  case 0xa:
  case 0x80:
    uVar1 = 0x2d;
    break;
  case 0xb:
    uVar1 = 0x7;
    break;
  case 0xc:
  case 0x17:
  case 0x18:
  case 0x19:
  case 0x21:
  case 0x75:
  case 0x81:
    if (param_3 == 0x75) {
      pass1_1010_715c(param_2,0x8,param_6,param_5,param_7,param_4);
      pass1_1010_715c(param_2,0x9,param_6,param_5,param_7,param_4);
    }
    uVar1 = pass1_1010_6ca2(param_2,0x7,param_5,param_4);
    if (uVar1 != 0x0) {
      pass1_1010_715c(param_2,0x10,uVar1,param_5,param_7,param_4);
    }
    param_6 = pass1_1010_6ca2(param_2,0x3,param_5,param_4);
    if (param_6 != 0x0) {
      pass1_1010_715c(param_2,0x11,param_6,param_5,param_7,param_4);
    }
    if (param_3 == 0x21) {
      pass1_1010_715c(param_2,0x14,param_6,param_5,param_7,param_4);
    }
    if (param_3 != 0xc) {
      return 0x1;
    }
    uVar1 = 0x9;
    goto code_r0x10106d4c;
  case 0xe:
    uVar1 = 0xc;
    goto code_r0x10106d4c;
  case 0x10:
  case 0x11:
  case 0x13:
    uVar1 = 0xd;
    break;
  case 0x12:
    uVar1 = 0xe;
    break;
  case 0x1b:
  case 0x1f:
  case 0x5b:
  case 0x78:
  case 0x7e:
  case 0x7f:
    if (param_3 == 0x7e) {
      pass1_1010_715c(param_2,0x2c,param_6,param_5,param_7,param_4);
    }
    if (param_3 == 0x5b) {
      pass1_1010_715c(param_2,0x38,param_6,param_5,param_7,param_4);
    }
    if (param_3 == 0x1f) {
      pass1_1010_715c(param_2,0x3f,param_6,param_5,param_7,param_4);
    }
    if (param_3 == 0x7f) {
      pass1_1010_715c(param_2,0x42,param_6,param_5,param_7,param_4);
    }
    param_6 = pass1_1010_6ca2(param_2,0x5,param_5,param_4);
    if ((param_6 == 0x0) && (param_6 = pass1_1010_6ca2(param_2,0x5,param_5,param_4), param_6 == 0x0)) {
      return 0x1;
    }
    uVar1 = 0x37;
    break;
  case 0x1d:
  case 0x2a:
  case 0x2c:
  case 0x3c:
  case 0x3d:
  case 0x4b:
  case 0x53:
  case 0x54:
  case 0x55:
  case 0x5a:
    uVar1 = pass1_1010_6ca2(param_2,0x2,param_5,param_4);
    if (uVar1 != 0x0) {
      pass1_1010_715c(param_2,0x12,uVar1,param_5,param_7,param_4);
    }
    uVar1 = pass1_1010_6ca2(param_2,0x8,param_5,param_4);
    if (uVar1 != 0x0) {
      pass1_1010_715c(param_2,0x1a,uVar1,param_5,param_7,param_4);
    }
    if (param_3 == 0x2c) {
      pass1_1010_715c(param_2,0x1d,uVar1,param_5,param_7,param_4);
    }
    param_6 = pass1_1010_6ca2(param_2,0x2,param_5,param_4);
    if (param_6 == 0x0) {
      return 0x1;
    }
    uVar1 = 0x1c;
    break;
  case 0x22:
    uVar1 = 0x15;
    break;
  case 0x25:
    uVar1 = 0x16;
    break;
  case 0x26:
    pass1_1010_715c(param_2,0x17,param_6,param_5,param_7,param_4);
  case 0x1e:
    uVar1 = 0x13;
    break;
  case 0x27:
    uVar1 = 0x18;
    break;
  case 0x29:
    uVar1 = 0x19;
    break;
  case 0x2b:
    uVar1 = 0x1b;
    break;
  case 0x2f:
  case 0x36:
    param_6 = pass1_1010_6ca2(param_2,0x2,param_5,param_4);
    if (param_6 == 0x0) {
      return 0x0;
    }
    uVar1 = 0x1e;
    break;
  case 0x30:
    uVar1 = 0x1f;
    break;
  case 0x31:
    uVar1 = 0x35;
    break;
  case 0x33:
    uVar1 = 0x21;
    break;
  case 0x34:
    uVar1 = 0x22;
    break;
  case 0x35:
    pass1_1010_715c(param_2,0x23,param_6,param_5,param_7,param_4);
  case 0x65:
  case 0x66:
  case 0x6b:
  case 0x6c:
  case 0x6d:
  case 0x6e:
  case 0x6f:
    uVar1 = 0x34;
    break;
  case 0x38:
    pass1_1010_715c(param_2,0x24,param_6,param_5,param_7,param_4);
    uVar1 = 0x3d;
    break;
  case 0x39:
    uVar1 = 0x25;
    break;
  case 0x3e:
    pass1_1010_715c(param_2,0x26,param_6,param_5,param_7,param_4);
    pass1_1010_715c(param_2,0x28,param_6,param_5,param_7,param_4);
    uVar1 = 0x27;
    break;
  case 0x40:
    uVar1 = 0x2a;
    break;
  case 0x41:
    uVar1 = 0x39;
    break;
  case 0x42:
    uVar1 = 0x3a;
    break;
  case 0x44:
    uVar1 = 0x36;
    break;
  case 0x45:
    uVar1 = 0x3b;
    break;
  case 0x49:
    uVar1 = 0x29;
    break;
  case 0x50:
    uVar1 = 0x2b;
    break;
  case 0x56:
    pass1_1010_715c(param_2,0x3c,param_6,param_5,param_7,param_4);
    uVar1 = 0x3e;
    break;
  case 0x5d:
    pass1_1010_715c(param_2,0x2f,param_6,param_5,param_7,param_4);
    uVar1 = 0x40;
    break;
  case 0x5e:
  case 0x60:
    uVar1 = 0x2f;
    break;
  case 0x5f:
    pass1_1010_715c(param_2,0x34,param_6,param_5,param_7,param_4);
    uVar1 = 0x41;
    break;
  case 0x61:
    uVar1 = 0x30;
    break;
  case 0x63:
    uVar1 = 0x31;
    break;
  case 0x64:
    uVar1 = 0x24;
    break;
  case 0x68:
    uVar1 = 0x32;
    break;
  case 0x69:
    uVar1 = 0x33;
    break;
  case 0x76:
    uVar1 = 0xa;
code_r0x10106d4c:
    pass1_1010_715c(param_2,uVar1,param_6,param_5,param_7,param_4);
    uVar1 = 0xb;
  }
  pass1_1010_715c(param_2,uVar1,param_6,param_5,param_7,param_4);
  return 0x1;
}



void __stdcall16far
pass1_1010_715c(ulong param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6)

{
  pass1_1010_a69c(*(ulong *)((int)param_1 + 0x14),param_2,param_3,param_4,param_5,param_6);
  return;
}



void __stdcall16far pass1_1010_71b0(int param_1,ushort param_2)

{
  ulong uVar1;
  
  uVar1 = *(ulong *)(param_1 + 0x6);
  send_msg_1010_7c42(uVar1 & 0xffff0000 | (ulong)((int)uVar1 - 0xa),param_2);
  return;
}



void __stdcall16far pass1_1010_71c2(uint param_1,ushort param_2,int param_3,UINT16 param_4)

{
  ulong uVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  
  if (param_1 == 0x13) {
    uVar2 = *(undefined4 *)(param_3 + 0x6);
    uVar2 = *(undefined4 *)((int)uVar2 + 0x18);
    uVar1 = *(ulong *)(param_3 + 0x6);
    destroy_window_1010_7b26
              (uVar1 & 0xffff0000 | (ulong)((int)uVar1 - 0xa),*(long *)((int)uVar2 + 0x28),param_4,param_2);
    return;
  }
  if (param_1 < 0x14) {
    if ((char)param_1 == '\x01') {
      uVar2 = *(undefined4 *)(param_3 + 0x6);
      uVar4 = (undefined2)((ulong)uVar2 >> 0x10);
      iVar3 = (int)uVar2;
      *(undefined4 *)(iVar3 + 0xa) = 0x0;
      *(undefined4 *)(iVar3 + 0x18) = 0x0;
      return;
    }
    if ((char)param_1 == '\x05') {
      uVar1 = *(ulong *)(param_3 + 0x6);
      send_msg_1010_7c42(uVar1 & 0xffff0000 | (ulong)((int)uVar1 - 0xa),param_4);
      return;
    }
  }
  return;
}



void __stdcall16far pass1_1010_71d6(ulong param_1,int param_2,ushort *param_3,uint param_4,uint param_5,ushort param_6)

{
  int iVar1;
  undefined4 uVar2;
  uint uVar3;
  int iVar4;
  ushort uVar5;
  ushort uVar6;
  uint uVar7;
  undefined2 uVar8;
  ulong uVar9;
  ushort uStack20;
  uint uStack14;
  ulong uStack6;
  
  uVar8 = (undefined2)(param_1 >> 0x10);
  uVar2 = *(undefined4 *)((int)param_1 + 0x14);
  pass1_1010_ad22((ushort)uVar2,param_5,param_6,(ushort)((ulong)uVar2 >> 0x10),*param_3);
  uStack6 = CONCAT22(param_5,param_4);
  if ((param_5 | param_4) == 0x0) {
    return;
  }
  uVar9 = struct_op_1030_73a8(CONCAT22(param_5,param_4));
  uVar7 = (uint)(uVar9 >> 0x10);
  uVar3 = (uint)uVar9;
  if (((uVar7 | uVar3) != 0x0) && (*(long *)(uVar3 + 0x1c) == 0x8000002)) {
    return;
  }
  uVar2 = *(undefined4 *)(param_4 + 0x2e);
  uStack14 = (uint)uVar2;
  if (((*(uint *)(param_4 + 0x30) | uStack14) != 0x0) && (*(long *)(uStack14 + 0x200) == 0x8000002)) {
    return;
  }
  uVar2 = *(undefined4 *)((int)param_1 + 0x14);
  uVar5 = pass1_1010_b028((ushort)uVar2,(ushort)((ulong)uVar2 >> 0x10),uVar9);
  iVar1 = *(int *)(uVar3 + 0x12);
  iVar4 = iVar1;
  if ((iVar1 != 0x4) && (iVar4 = param_2, iVar1 == 0x7)) {
    param_2 = 0x5;
    iVar4 = param_2;
  }
  param_2 = iVar4;
  uVar6 = param_2 - 0x2;
  if (uVar6 != 0x0) {
    if (param_2 == 0x3) {
      uVar6 = uVar5 - 0xb;
      if ((uVar6 == 0x0) || (uVar6 = uVar5 - 0x37, uVar6 == 0x0)) {
        uStack20 = 0xb;
      }
      else {
        uStack20 = 0xa;
      }
      goto LAB_1010_72a7;
    }
    uVar6 = param_2 - 0x4;
    if (uVar6 == 0x0) {
      uStack20 = 0x17;
      goto LAB_1010_72a7;
    }
    uVar6 = param_2 - 0x5;
    if (uVar6 != 0x0) {
      uVar6 = pass1_1010_7818(param_1,uVar9);
      uStack20 = uVar6;
      goto LAB_1010_72a7;
    }
  }
  uStack20 = 0xc;
LAB_1010_72a7:
  if (uStack20 == 0x0) {
    return;
  }
  ui_op_1010_79aa(param_1,0x0,uStack6,param_6);
  if (uVar6 == 0x0) {
    unk_win_op_1010_7300(param_1,0x0,uStack20,uStack6);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far unk_win_op_1010_7300(ulong param_1,ulong param_2,uint param_3,ulong param_4)

{
  undefined4 uVar1;
  code **ppcVar2;
  char cVar3;
  uint uVar4;
  uchar *in_DX;
  uchar *puVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  uchar *extraout_DX_00;
  uchar *puVar7;
  uchar *puVar8;
  int unaff_DI;
  undefined2 uVar9;
  HWND16 HVar10;
  HWND16 HVar11;
  ushort unaff_SS;
  ulong uVar12;
  astruct_57 *paVar13;
  ushort *puVar14;
  LRESULT LVar15;
  undefined2 uVar16;
  undefined uVar17;
  undefined2 uVar18;
  undefined2 *puStack20;
  undefined2 *puStack14;
  undefined4 *puStack10;
  undefined4 uStack6;
  
  if (param_3 == 0x0) {
    return;
  }
  uStack6 = param_2;
  puVar8 = (uchar *)param_1;
  uVar9 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x0) {
    uVar1 = *(undefined4 *)(puVar8 + 0x14);
    pass1_1010_ad64((ushort)uVar1,CONCAT22(param_3,(int)((ulong)uVar1 >> 0x10)),param_4,0x0,(ushort)in_DX);
    uStack6 = param_2 & 0xffff | ZEXT24(in_DX) << 0x10;
  }
  switch(param_3) {
  case 0x1:
  case 0x4:
  case 0x6:
  case 0x7:
  case 0x8:
  case 0x9:
  case 0xd:
  case 0xe:
  case 0x14:
  case 0x18:
    break;
  default:
    if ((uStack6._2_2_ | (uint)uStack6) == 0x0) {
      return;
    }
  }
  pass1_1010_1f62(unaff_SS,param_1,0xb);
  if (*(int *)(puVar8 + 0xe) == 0x0) {
    return;
  }
  puVar6 = puVar8;
  switch(param_3 - 0x1) {
  case 0x0:
    mem_op_1000_179c(0x94,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) {
LAB_1010_73fe:
      HVar10 = 0x1000;
      puVar6 = (uchar *)0x0;
      puVar5 = (uchar *)0x0;
    }
    else {
      HVar10 = (HWND16)&PTR_LOOP_1050_1040;
      pass1_1040_44d2((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8),CONCAT12((char)in_DX,puVar6)),param_4,
                      *(ushort *)(puVar8 + 0xe),(uint)puVar6,puVar5);
    }
    break;
  default:
    mem_op_1000_179c(0x94,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    pass1_1040_b040((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8),CONCAT12((char)in_DX,puVar6)),uStack6,
                    *(ushort *)(puVar8 + 0xe));
    break;
  case 0x3:
    mem_op_1000_179c(0x9e,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    pass1_1040_5626((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8),CONCAT12((char)in_DX,puVar6)),param_4,
                    *(ushort *)(puVar8 + 0xe),puVar5);
    break;
  case 0x4:
    mem_op_1000_179c(0x94,in_DX,0x1000);
    if (((uint)in_DX | (uint)puVar6) == 0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    puVar14 = pass1_1040_8e58((int)puVar6,(ushort)in_DX,(uint)uStack6,
                              CONCAT22(*(undefined2 *)(puVar8 + 0xe),uStack6._2_2_));
    puVar5 = (uchar *)((ulong)puVar14 >> 0x10);
    puVar6 = (uchar *)puVar14;
    break;
  case 0x5:
  case 0x6:
    mem_op_1000_179c(0x98,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    pass1_1040_48a0(CONCAT22(in_DX,puVar6),param_3,param_4,*(ushort *)(puVar8 + 0xe),puVar5,unaff_SS);
    break;
  case 0x7:
    mem_op_1000_179c(0x9c,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1038;
    pass1_1038_9144((ushort *)CONCAT22(in_DX,puVar6),*(ushort *)(puVar8 + 0xe),unaff_SS);
    break;
  case 0x8:
    mem_op_1000_179c(0xb8,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    pass1_1040_b7ee((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8),CONCAT12((char)in_DX,puVar6)),uStack6,
                    *(ushort *)(puVar8 + 0xe));
    break;
  case 0x9:
  case 0xa:
    mem_op_1000_179c(0x94,in_DX,0x1000);
    if (((uint)in_DX | (uint)puVar6) == 0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1038;
    puVar14 = pass1_1038_9a1e((int)puVar6,(ushort)in_DX,(uint)uStack6,
                              CONCAT22(*(undefined2 *)(puVar8 + 0xe),uStack6._2_2_));
    puVar5 = (uchar *)((ulong)puVar14 >> 0x10);
    puVar6 = (uchar *)puVar14;
    break;
  case 0xb:
    mem_op_1000_179c(0x12a,in_DX,0x1000);
    if (((uint)in_DX | (uint)puVar6) == 0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1038;
    uVar12 = pass1_1038_9b72((int)puVar6,(ushort)in_DX,(uint)uStack6,
                             CONCAT22(*(undefined2 *)(puVar8 + 0xe),uStack6._2_2_));
    puVar5 = (uchar *)(uVar12 >> 0x10);
    puVar6 = (uchar *)uVar12;
    break;
  case 0xc:
    mem_op_1000_179c(0x9c,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    pass1_1040_6826((astruct_57 *)CONCAT22(in_DX,puVar6),*(ushort *)(puVar8 + 0xe));
    break;
  case 0xd:
    mem_op_1000_179c(0x9c,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    pass1_1040_6fb6((astruct_57 *)CONCAT22(in_DX,puVar6),*(ushort *)(puVar8 + 0xe));
    break;
  case 0x12:
    mem_op_1000_179c(0x94,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    make_proc_inst_1040_a234
              (puVar6,in_DX,(uint)uStack6,CONCAT22(*(undefined2 *)(puVar8 + 0xe),uStack6._2_2_),(int)&PTR_LOOP_1050_1040
              );
    break;
  case 0x13:
    mem_op_1000_179c(0xb8,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    pass1_1040_4e94((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8),CONCAT12((char)in_DX,puVar6)),uStack6,
                    *(ushort *)(puVar8 + 0xe));
    break;
  case 0x14:
    mem_op_1000_179c(0x9a,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    pass1_1040_0e1c((astruct_57 *)CONCAT22(in_DX,puVar6),0x1,uStack6,*(ushort *)(puVar8 + 0xe),puVar5,unaff_DI,unaff_SS)
    ;
    break;
  case 0x15:
    mem_op_1000_179c(0x9c,in_DX,0x1000);
    if (((uint)in_DX | (uint)puVar6) == 0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    paVar13 = pas1_1040_29c2((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8),CONCAT12((char)in_DX,puVar6)),uStack6,
                             *(ushort *)(puVar8 + 0xe),(ushort)puVar6,(uint)in_DX | (uint)puVar6);
    puVar5 = (uchar *)((ulong)paVar13 >> 0x10);
    puVar6 = (uchar *)paVar13;
    break;
  case 0x16:
    mem_op_1000_179c(0x12a,in_DX,0x1000);
    if (((uint)in_DX | (uint)puVar6) == 0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1038;
    puVar14 = pass1_1038_adde((int)puVar6,(ushort)in_DX,(uint)uStack6,
                              CONCAT22(*(undefined2 *)(puVar8 + 0xe),uStack6._2_2_));
    puVar5 = (uchar *)((ulong)puVar14 >> 0x10);
    puVar6 = (uchar *)puVar14;
    break;
  case 0x17:
    mem_op_1000_179c(0xec,in_DX,0x1000);
    puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
    if (puVar5 == (uchar *)0x0) goto LAB_1010_73fe;
    HVar10 = (HWND16)&PTR_LOOP_1050_1040;
    pass1_1040_a640((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8),CONCAT12((char)in_DX,puVar6)),param_4,
                    *(ushort *)(puVar8 + 0xe));
  }
  puStack10 = (undefined4 *)CONCAT22(puVar5,puVar6);
  ppcVar2 = (code **)((int)*puStack10 + 0x8);
  (**ppcVar2)(HVar10,puVar6,puVar5);
  puVar7 = extraout_DX;
  HVar11 = HVar10;
  if (param_3 != 0x17) {
    if (0x17 < param_3) goto LAB_1010_7710;
    cVar3 = (char)param_3;
    if ((cVar3 != '\x05') && (((char)(cVar3 + -0x5) < '\x05' || ('\x02' < (char)(cVar3 + -0xa))))) goto LAB_1010_7710;
  }
  if (*(int *)((uint)uStack6 + 0x16) != 0x0) {
    HVar11 = (HWND16)s_tile2_bmp_1050_1538;
    LVar15 = SendMessage16(HVar10,0x0,0x0,0x11100f8);
    puVar7 = (uchar *)((ulong)LVar15 >> 0x10);
  }
LAB_1010_7710:
  HVar10 = HVar11;
  if (puStack10 != (undefined4 *)0x0) {
    uVar18 = *(undefined2 *)(puVar6 + 0x6);
    uVar17 = (undefined)HVar11;
    HVar10 = (HWND16)s_tile2_bmp_1050_1538;
    uVar4 = IsWindow16(HVar11);
    if (uVar4 != 0x0) {
      puVar6 = puVar7;
      if (*(long *)(puVar8 + 0x1c) == 0x0) {
        uVar17 = 0xc;
        mem_op_1000_179c(0xc,puVar7,0x1000);
        puVar6 = (uchar *)((uint)puVar7 | uVar4);
        if (puVar6 == (uchar *)0x0) {
          *(undefined4 *)(puVar8 + 0x1c) = 0x0;
        }
        else {
          set_struct_1008_574a((astruct_21 *)CONCAT22(puVar7,uVar4));
          *(uint *)(puVar8 + 0x1c) = uVar4;
          *(uchar **)(puVar8 + 0x1e) = extraout_DX_00;
          puVar6 = extraout_DX_00;
        }
      }
      uVar16 = 0xc;
      mem_op_1000_179c(0xc,puVar6,0x1000);
      puStack14 = (undefined2 *)CONCAT22(puVar6,uVar4);
      if (((uint)puVar6 | uVar4) == 0x0) {
        puStack20 = (undefined2 *)0x0;
      }
      else {
        *puStack14 = 0x389a;
        *(undefined2 *)(uVar4 + 0x2) = 0x1008;
        *(ulong *)(uVar4 + 0x4) = param_4;
        *(undefined4 *)(uVar4 + 0x8) = puStack10;
        *puStack14 = 0x7e24;
        *(undefined2 *)(uVar4 + 0x2) = 0x1010;
        puStack20 = puStack14;
      }
      ppcVar2 = (code **)((int)**(undefined4 **)(puVar8 + 0x1c) + 0x4);
      (**ppcVar2)(0x1000,*(undefined4 *)(puVar8 + 0x1c),(char)puStack20,(int)((ulong)puStack20 >> 0x10),uVar16,uVar17,
                  uVar18);
      return;
    }
  }
  if (((uint)puVar5 | (uint)puVar6) != 0x0) {
    ppcVar2 = (code **)*puStack10;
    (**ppcVar2)(HVar10,puVar6,(char)puVar5,0x1);
  }
  return;
}

