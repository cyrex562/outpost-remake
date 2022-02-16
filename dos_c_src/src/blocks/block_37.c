
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_e964(uchar *param_1,ushort param_2,int param_3)

{
  ulong uVar1;
  uint uVar2;
  ushort *puVar3;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_2,param_1,param_3);
  uVar2 = (uint)((ulong)puVar3 >> 0x10);
  uVar1 = *(ulong *)((int)puVar3 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)(uVar1 >> 0x10));
  pass1_1038_4d28(uVar1 & 0xffff | (ulong)uVar2 << 0x10);
  return;
}



ulong __stdcall16far pass1_1010_e99a(ulong param_1,byte param_2)

{
  ushort unaff_SS;
  
  param_1 = param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xa);
  pass1_1010_a478((ushort *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1010_e9e4(astruct_261 *param_1,ushort param_2,ushort param_3)

{
  uint *puVar1;
  uint uVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  ulong uVar6;
  uchar *puVar7;
  int iVar8;
  astruct_79 *paVar9;
  ushort *puVar10;
  int iStack4;
  
  paVar9 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  puVar7 = (uchar *)((ulong)paVar9 >> 0x10);
  param_1->field_0xa = 0x389a;
  param_1->field_0xc = 0x1008;
  param_1->field_0xa = 0x3aa8;
  param_1->field_0xc = 0x1008;
  uVar5 = 0x0;
  *(undefined4 *)&param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  param_1->field_0x1a = 0x0;
  param_1->field_0x1e = 0x0;
  param_1->field_0x20 = 0x0;
  param_1->field_0x24 = 0x0;
  param_1->field_0x28 = 0x0;
  param_1->field_0x2c = 0x0;
  param_1->field_0x30 = 0x0;
  param_1->field_0x32 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x558;
  param_1->field_0x2 = 0x1018;
  param_1->field_0xa = 0x568;
  param_1->field_0xc = 0x1018;
  mem_op_1000_179c(0x4,puVar7,0x1000);
  if (((uint)puVar7 | uVar5) == 0x0) {
    *(undefined4 *)&param_1->field_0xe = 0x0;
  }
  else {
    puVar10 = pass1_1018_dcf6((ushort *)CONCAT22(puVar7,uVar5));
    param_1->field_0xe = (int)puVar10;
    param_1->field_0x10 = (int)((ulong)puVar10 >> 0x10);
  }
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0x34),(WNDCLASS16 *)0x0,0x24);
  param_1->field_0x38 = 0xfa;
  param_1->field_0x3c = 0x15e;
  uVar6 = 0x1c2;
  param_1->field_0x40 = 0x1c2;
  param_1->field_0x44 = 0x1c2;
  param_1->field_0x46 = 0x2260000;
  param_1->field_0x4a = 0x28a0000;
  param_1->field_0x4e = 0x730000;
  param_1->field_0x52 = 0x960000;
  param_1->field_0x56 = 0x0;
  for (iStack4 = 0x1; iStack4 < 0x9; iStack4 = iStack4 + 0x1) {
    pass1_1008_612e(0x0,0x1d,(uint)uVar6);
    uVar5 = (uint)uVar6;
    pass1_1008_612e(0x1,0x2,uVar5);
    if ((uVar6 & 0x1) != 0x0) {
      uVar5 = -uVar5;
    }
    iVar8 = iStack4 * 0x4;
    puVar1 = (uint *)(&param_1->field_0x34 + iVar8);
    uVar2 = *puVar1;
    uVar4 = uVar5 + *puVar1;
    uVar6 = (ulong)uVar4;
    iVar3 = *(int *)(&param_1->field_0x34 + iVar8 + 0x2);
    *(uint *)(&param_1->field_0x34 + iVar8) = uVar4;
    *(int *)(&param_1->field_0x36 + iVar8) = ((int)uVar5 >> 0xf) + iVar3 + (uint)CARRY2(uVar5,uVar2);
  }
  return;
}



void __stdcall16far pass1_1010_eb66(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined2 *puVar4;
  astruct_499 *iVar5;
  undefined2 uVar5;
  undefined2 *puStack14;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_499 *)param_1;
  *param_1 = 0x558;
  iVar5->field_0x2 = 0x1018;
  iVar5->field_0xa = 0x568;
  iVar5->field_0xc = 0x1018;
  puVar1 = iVar5->field_0xe;
  uVar2 = iVar5->field_0x10;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1018_04f2((ulong)param_1);
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x2c,0x1000);
  if (param_1 == (ushort *)0x0) {
    puVar4 = (undefined2 *)0x0;
    uVar5 = 0x0;
  }
  else {
    puVar4 = &iVar5->field_0xa;
  }
  puStack14 = (undefined2 *)CONCAT22(uVar5,puVar4);
  *puStack14 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



void __stdcall16far pass1_1010_ebf8(ulong param_1,ushort param_2,ushort param_3,int param_4)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(ushort *)((int)param_1 + param_4 * 0x4 + 0x34) = param_2;
  *(ushort *)((int)param_1 + param_4 * 0x4 + 0x36) = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_ec18(ushort param_1,ushort param_2,ulong param_3,int param_4,ushort param_5)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_3,(uint)(param_3 >> 0x10));
  return CONCAT22(*(undefined2 *)(param_4 + 0x12),*(undefined2 *)(param_4 + 0x10));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_ec40(ushort param_1,ushort param_2,ulong param_3,int param_4,ushort param_5)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_3,(uint)(param_3 >> 0x10));
  return CONCAT22(*(undefined2 *)(param_4 + 0x12),*(undefined2 *)(param_4 + 0x10));
}



void __stdcall16far pass1_1010_ec68(ulong param_1,ulong param_2,ushort param_3)

{
  uint uVar1;
  
  uVar1 = (uint)(param_1 >> 0x10);
  *(ulong *)((int)param_1 + 0x28) = param_2;
  pass1_1010_1f62(param_3,param_1 & 0xffff | (ulong)uVar1 << 0x10,0x13);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_ec84(ulong param_1,ushort param_2,uchar param_3)

{
  undefined local_10e [0x10c];
  
  pass1_1010_1f62(param_2,param_1,0x14);
  pass1_1030_532e((astruct_100 *)CONCAT22(param_2,local_10e),*(ulong *)((int)param_1 + 0x20),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_2,local_10e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1010_ecc6(ulong param_1,ushort *param_2,long param_3,ushort param_4,uint param_5,ushort param_6)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  pass1_1030_627e(param_6,param_4,param_5,_PTR_LOOP_1050_5740,param_2,param_3);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4,param_5);
  uVar1 = *(undefined4 *)(param_4 + 0x2e);
  uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
  iVar2 = (int)uVar1;
  if (*(long *)(iVar2 + 0x200) == 0x8000001) {
    pass1_1010_ed22(param_1,*(ulong *)(iVar2 + 0x4),param_6);
  }
  return;
}



void __stdcall16far pass1_1010_ed22(ulong param_1,ulong param_2,ushort param_3)

{
  uint uVar1;
  
  uVar1 = (uint)(param_1 >> 0x10);
  *(ulong *)((int)param_1 + 0x24) = param_2;
  pass1_1010_1f62(param_3,param_1 & 0xffff | (ulong)uVar1 << 0x10,0x12);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_ed3e(ulong param_1)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x16);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  return;
}



void __stdcall16far write_to_file_1010_ed58(ulong param_1,ulong param_2,uint16_t param_3)

{
  int *piVar1;
  undefined2 uVar2;
  BOOL16 BVar3;
  int iVar4;
  undefined4 *puVar5;
  int iVar6;
  undefined2 uVar7;
  ushort uVar8;
  ushort uVar9;
  undefined4 local_22;
  undefined2 uStack30;
  ulong local_12 [0x2];
  ulong local_a;
  int iStack4;
  
  BVar3 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar3 != 0x0) {
    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    local_12[0] = *(ulong *)(iVar6 + 0x16);
    uVar8 = (ushort)param_2;
    uVar9 = (ushort)(param_2 >> 0x10);
    BVar3 = write_to_file_1008_7e1c(uVar8,uVar9,(ushort)local_12,param_3,(char *)0x4,0x1008);
    if (BVar3 != 0x0) {
      local_a = *(ulong *)(iVar6 + 0x1a);
      BVar3 = write_to_file_1008_7e1c(uVar8,uVar9,(ushort)&local_a,param_3,(char *)0x4,0x1008);
      if (BVar3 != 0x0) {
        local_a = *(ulong *)(iVar6 + 0x20);
        BVar3 = write_to_file_1008_7e1c(uVar8,uVar9,(ushort)&local_a,param_3,(char *)0x4,0x1008);
        if (BVar3 != 0x0) {
          local_a = *(ulong *)(iVar6 + 0x24);
          BVar3 = write_to_file_1008_7e1c(uVar8,uVar9,(ushort)&local_a,param_3,(char *)0x4,0x1008);
          if (BVar3 != 0x0) {
            local_a = local_a & 0xffff0000 | (ulong)*(uint *)(iVar6 + 0x30);
            BVar3 = write_to_file_1008_7e1c(uVar8,uVar9,(ushort)&local_a,param_3,(char *)0x2,0x1008);
            if (BVar3 != 0x0) {
              local_a = local_a & 0xffff0000 | (ulong)*(uint *)(iVar6 + 0x32);
              BVar3 = write_to_file_1008_7e1c(uVar8,uVar9,(ushort)&local_a,param_3,(char *)0x2,0x1008);
              if (BVar3 != 0x0) {
                iStack4 = 0x0;
                while( true ) {
                  piVar1 = (int *)(iVar6 + 0x30);
                  if (*piVar1 == iStack4 || *piVar1 < iStack4) {
                    return;
                  }
                  uVar2 = *(undefined2 *)(iVar6 + 0x2e);
                  puVar5 = (undefined4 *)(*(int *)(iVar6 + 0x2c) + iStack4 * 0x6);
                  local_22 = *puVar5;
                  uStack30 = *(undefined2 *)(puVar5 + 0x1);
                  local_12[0] = local_12[0] & 0xffff0000 | ZEXT24(&local_22);
                  iVar4 = write_to_file_1008_7b4c(param_2,CONCAT22(param_3,&local_22),0x1008,param_3);
                  if (iVar4 == 0x0) break;
                  iStack4 = iStack4 + 0x1;
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_0000(ulong param_1,ulong param_2,int param_3,uchar *param_4,uint16_t param_5)

{
  int *piVar1;
  int iVar2;
  ulong uVar3;
  ushort uVar4;
  int iVar5;
  BOOL16 BVar6;
  ushort uVar7;
  ushort uVar8;
  undefined local_20 [0x10];
  int iStack16;
  
                    // Segment:    4
                    // Offset:     00024460
                    // Length:     ee6a
                    // Min Alloc:  ee6a
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  uVar8 = (ushort)param_2;
  uVar7 = (ushort)(param_2 >> 0x10);
  read_file_1008_7cfe(uVar8,uVar7,0x2,0x1008,param_5);
  if (param_3 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d4;
  }
  else {
    iVar5 = (int)param_1;
    uVar4 = (ushort)(param_1 >> 0x10);
    BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x16,0x0,uVar4,0x4,0x1008);
    if ((((BVar6 != 0x0) && (BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x1a,0x0,uVar4,0x4,0x1008), BVar6 != 0x0))
        && (BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x20,0x0,uVar4,0x4,0x1008), BVar6 != 0x0)) &&
       (((BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x24,0x0,uVar4,0x4,0x1008), BVar6 != 0x0 &&
         (BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x30,0x0,uVar4,0x2,0x1008), BVar6 != 0x0)) &&
        (BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x32,0x0,uVar4,0x2,0x1008), BVar6 != 0x0)))) {
      if (*(int *)(iVar5 + 0x30) != 0x0) {
        iVar2 = *(int *)(iVar5 + 0x32);
        if (_PTR_LOOP_1050_5f2c == 0x0) {
          PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_4,0x1000);
          PTR_LOOP_1050_5f2e = param_4;
        }
        else {
        }
        uVar7 = fn_ptr_op_1000_1708(iVar2 * 0x6,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
        *(ushort *)(iVar5 + 0x2c) = uVar7;
        *(uchar **)(iVar5 + 0x2e) = PTR_LOOP_1050_5f2e;
        pass1_1008_3e38((ushort *)CONCAT22(param_5,local_20));
        for (iStack16 = 0x0; piVar1 = (int *)(iVar5 + 0x30), *piVar1 != iStack16 && iStack16 <= *piVar1;
            iStack16 = iStack16 + 0x1) {
          BVar6 = read_file_1008_7bc8(param_2,(ushort *)CONCAT22(param_5,local_20),0x1008,param_5);
          if (BVar6 == 0x0) {
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return;
          }
          uVar3 = *(ulong *)(iVar5 + 0x2c);
          pass1_1008_3f62((ushort *)(uVar3 & 0xffff0000 | (ulong)(uint)((int)uVar3 + iStack16 * 0x6)),
                          (ushort *)CONCAT22(param_5,local_20));
        }
      }
      return;
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



void __stdcall16far pass1_1018_017c(ulong param_1,ushort param_2,ushort param_3)

{
  uint uVar1;
  
  uVar1 = (uint)(param_1 >> 0x10);
  *(ushort *)((int)param_1 + 0x1e) = param_2;
  pass1_1010_1f62(param_3,param_1 & 0xffff | (ulong)uVar1 << 0x10,0x4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1018_0196(ulong param_1,ulong param_2,ulong param_3,ushort param_4,uchar *param_5,ushort param_6)

{
  int *piVar1;
  int iVar2;
  undefined4 uVar3;
  ulong uVar4;
  ushort uVar5;
  ulong uVar6;
  int iVar7;
  undefined2 uVar8;
  long lVar9;
  
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),param_3);
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar7 = (int)param_1;
  if (*(long *)(iVar7 + 0x2c) == 0x0) {
    *(undefined2 *)(iVar7 + 0x32) = 0x5;
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_5,0x1000);
      PTR_LOOP_1050_5f2e = param_5;
    }
    else {
    }
    uVar5 = fn_ptr_op_1000_1708(0x1e,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
    uVar5 = *(int *)(iVar7 + 0x30) + 0x1;
    PTR_LOOP_1050_5f2e = param_5;
    if ((int)uVar5 < *(int *)(iVar7 + 0x32)) goto LAB_1018_022a;
    piVar1 = (int *)(iVar7 + 0x32);
    *piVar1 = *piVar1 + 0x5;
    uVar3 = *(undefined4 *)(iVar7 + 0x2c);
    lVar9 = pass1_1000_0ed4(0x1000,param_6,0x1,*(int *)(iVar7 + 0x32) * 0x6,0x0,(ushort *)uVar3,
                            (ushort)((ulong)uVar3 >> 0x10));
    PTR_LOOP_1050_5f2e = (undefined *)((ulong)lVar9 >> 0x10);
    uVar5 = (ushort)lVar9;
  }
  *(ushort *)(iVar7 + 0x2c) = uVar5;
  *(uchar **)(iVar7 + 0x2e) = PTR_LOOP_1050_5f2e;
LAB_1018_022a:
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),param_2);
  uVar6 = *(ulong *)(uVar5 + 0x10);
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),uVar6);
  iVar2 = *(int *)(iVar7 + 0x30);
  piVar1 = (int *)(iVar7 + 0x30);
  *piVar1 = *piVar1 + 0x1;
  uVar4 = *(ulong *)(iVar7 + 0x2c);
  pass1_1008_3f62((ushort *)(uVar4 & 0xffff0000 | (ulong)(uint)((int)uVar4 + iVar2 * 0x6)),
                  (ushort *)CONCAT22(PTR_LOOP_1050_5f2e,(int)uVar6 + 0xc));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_028c(ulong param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5)

{
  undefined4 uVar1;
  code **ppcVar2;
  undefined *puVar3;
  uint uVar4;
  int iVar5;
  ulong uVar6;
  uchar *puVar7;
  uchar *puVar8;
  uint uVar9;
  uint extraout_DX;
  ushort uVar10;
  int iVar11;
  int unaff_DI;
  undefined2 uVar12;
  undefined2 uVar13;
  undefined2 uVar14;
  uint uVar15;
  int iStack36;
  undefined4 *puStack28;
  undefined local_18 [0x4];
  uint uStack20;
  ushort *puStack12;
  ushort uStack8;
  ushort uStack6;
  uchar *puStack4;
  
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),param_2);
  uStack6 = param_3;
  puStack4 = (uchar *)param_4;
  uStack8 = pass1_1030_5b00(CONCAT22(param_4,param_3));
  puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uStack8,param_5,(uchar *)param_4,unaff_DI);
  pass1_1008_6c90((ushort *)CONCAT22(param_5,local_18));
  pass1_1018_0b1e(puStack12,(ushort *)CONCAT22(param_5,local_18),param_5);
  puVar7 = (uchar *)((int)uStack20 >> 0xf);
  if (((uint)puVar7 | uStack20) == 0x0) {
    puVar3 = local_18;
    pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(param_5,puVar3),param_2,param_5);
  }
  else {
    puVar3 = local_18;
    pass1_1030_62e4(_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_5,puVar3),param_2,param_5);
  }
  puStack28 = (undefined4 *)CONCAT22(puVar7,puVar3);
  uVar4 = (uint)puVar7 | (uint)puVar3;
  if (uVar4 == 0x0) {
    return;
  }
  puVar8 = puVar7;
  pass1_1018_04f2(param_1);
  uVar14 = 0x1c;
  uVar13 = 0x1000;
  mem_op_1000_179c(0x1c,puVar8,0x1000);
  uVar9 = (uint)puVar8 | uVar4;
  iVar11 = (int)param_1;
  uVar12 = (undefined2)(param_1 >> 0x10);
  uVar15 = uVar4;
  if (uVar9 == 0x0) {
    *(undefined4 *)(iVar11 + 0x12) = 0x0;
  }
  else {
    uVar13 = 0x1008;
    struct_op_1008_8e9e((astruct_78 *)CONCAT22(puVar8,uVar4),0x6,0x24);
    *(uint *)(iVar11 + 0x12) = uVar4;
    *(uint *)(iVar11 + 0x14) = uVar9;
  }
  ppcVar2 = (code **)((int)*puStack28 + 0x10);
  (**ppcVar2)(uVar13,puVar3,puVar7,uVar14,uVar15);
  for (iStack36 = 0x0; iStack36 < (int)uVar4; iStack36 = iStack36 + 0x1) {
    uVar6 = SEXT24(iStack36);
    ppcVar2 = (code **)((int)*puStack28 + 0x4);
    (**ppcVar2)();
    if ((extraout_DX | (uint)uVar6) != 0x0) {
      iVar5 = iStack36 / 0x6;
      uVar10 = iStack36 % 0x6;
      uVar1 = *(undefined4 *)(iVar11 + 0xe);
      pass1_1018_dd7c((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),CONCAT22(iStack36 % 0x6,iVar5),
                      uVar6 & 0xffff | (ulong)extraout_DX << 0x10,uVar10,param_5);
      pass1_1008_8faa(*(undefined4 *)(iVar11 + 0x12),CONCAT22(uVar10,iVar5));
    }
  }
  return;
}



void __stdcall16far pass1_1018_03ea(ulong param_1,int param_2,ushort param_3)

{
  if (param_2 != 0x5) {
    return;
  }
  pass1_1010_1f62(param_3,param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xa),0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1018_0412(ulong param_1,ushort param_2,ulong param_3,ushort param_4,ulong param_5,ushort param_6,uchar param_7)

{
  undefined *puVar1;
  undefined local_128 [0x124];
  ushort uStack4;
  
  uStack4 = 0x0;
  if (((0x72 < (int)param_4) && (!SBORROW2(param_4,0x73))) &&
     ((param_4 == 0x75 || (int)(param_4 - 0x74) < 0x1 ||
      ((0x0 < (int)(param_4 - 0x76) && ((int)(param_4 - 0x77) < 0x2)))))) {
    uStack4 = 0x1;
  }
  struct_op_1028_933c((astruct_100 *)CONCAT22(param_6,local_128),param_2,uStack4,param_4,(ulong *)param_3,
                      (ushort)(param_3 >> 0x10),*(ulong *)((int)param_1 + 0x24),param_5,param_6,param_7);
  puVar1 = local_128;
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_6,puVar1));
  if (puVar1 != (undefined *)0x0) {
    pass1_1010_1f62(param_6,param_1,0x6);
  }
  return;
}



void __stdcall16far pass1_1018_04a4(ulong param_1,ulong param_2)

{
  *(ulong *)((int)param_1 + 0x16) = param_2;
  return;
}



ulong __stdcall16far pass1_1018_04b8(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0x18),*(undefined2 *)((int)param_1 + 0x16));
}



void __stdcall16far pass1_1018_04ca(ulong param_1,ulong param_2)

{
  *(ulong *)((int)param_1 + 0x1a) = param_2;
  return;
}



void __stdcall16far pass1_1018_04de(ulong param_1,ulong param_2)

{
  *(ulong *)((int)param_1 + 0x20) = param_2;
  return;
}



void __stdcall16far pass1_1018_04f2(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  ushort uVar5;
  
  uVar5 = (ushort)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x12);
  uVar2 = *(uint *)(iVar4 + 0x14);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)(iVar4 + 0x12) = 0x0;
  return;
}



ushort * __stdcall16far pass1_1018_0526(ushort *param_1,byte param_2,ushort param_3)

{
  param_1 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xa));
  pass1_1010_eb66(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1018_0570(astruct_55 *param_1,ushort param_2,ushort param_3)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined2 *puVar3;
  ushort uVar4;
  uchar *puVar5;
  undefined2 uVar6;
  uchar *extraout_DX;
  int unaff_DI;
  ushort *puVar7;
  undefined2 uVar8;
  astruct_262 *uVar9;
  
  uVar9 = (astruct_262 *)param_1;
  uVar8 = (undefined2)((ulong)param_1 >> 0x10);
  get_sys_metrics_1018_4b1e(param_1,0x0,param_2);
  uVar9->field_0x20 = 0x389a;
  uVar9->field_0x22 = 0x1008;
  uVar9->field_0x20 = 0x3aa8;
  uVar9->field_0x22 = 0x1008;
  uVar9->field_0x24 = 0x0;
  uVar9->field_0x2c = (undefined4 *)0x0;
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&uVar9->field_0x30));
  puVar7 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&uVar9->field_0x36));
  puVar5 = (uchar *)((ulong)puVar7 >> 0x10);
  uVar9->field_0x3c = 0x0;
  pass1_1008_6c90((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&uVar9->field_0x40));
  uVar6 = 0x0;
  uVar9->field_0x4c = 0x0;
  uVar9->field_0x5a = 0x0;
  uVar9->field_0x5e = 0x0;
  uVar9->field_0x60 = 0x0;
  uVar9->field_0x64 = 0xff00;
  uVar9->field_0x66 = 0x0;
  uVar9->field_0x68 = 0x10000fb;
  uVar9->field_0x6c = 0x10000f9;
  uVar9->field_0x70 = 0x10000ff;
  uVar9->field_0x74 = 0x10000fe;
  uVar9->field_0x78 = 0x10000fc;
  uVar9->field_0x7c = 0x0;
  uVar9->field_0x80 = 0x0;
  uVar9->field_0x84 = 0x1;
  uVar9->field_0x86 = 0x0;
  uVar9->field_0x88 = 0x0;
  uVar9->field_0x8c = 0x0;
  uVar9->field_0x8e = 0x0;
  uVar9->field_0x92 = 0x0;
  uVar9->field_0x94 = 0x0;
  uVar9->field_0x98 = 0x0;
  uVar9->field_0x9a = 0x0;
  *(undefined4 *)&uVar9->field_0xa2 = 0x0;
  uVar9->field_0xa6 = 0xffff;
  uVar9->field_0xa8 = 0x0;
  param_1->field_0x0 = 0x1874;
  uVar9->field_0x2 = 0x1018;
  uVar9->field_0x20 = 0x18b0;
  uVar9->field_0x22 = 0x1018;
  if ((PTR_LOOP_1050_3960 == (undefined *)0x0) && (_PTR_LOOP_1050_3962 == 0x0)) {
    mem_op_1000_179c(0x8,puVar5,0x1000);
    _PTR_LOOP_1050_3962 = CONCAT22(puVar5,uVar6);
    pass1_1000_4906((astruct_20 *)CONCAT22(puVar5,uVar6),(WNDCLASS16 *)0x0,0x8);
  }
  PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + 0x1;
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,puVar5,unaff_DI);
  *(int *)&uVar9->field_0x2c = (int)puVar7;
  *(undefined2 *)((int)&uVar9->field_0x2c + 0x2) = (int)((ulong)puVar7 >> 0x10);
  if (param_1 == (astruct_55 *)0x0) {
    puVar3 = (undefined2 *)0x0;
    uVar6 = 0x0;
  }
  else {
    puVar3 = &uVar9->field_0x20;
    uVar6 = uVar8;
  }
  puVar1 = uVar9->field_0x2c;
  ppcVar2 = (code **)((int)*uVar9->field_0x2c + 0x4);
  (**ppcVar2)(0x1010,(int)puVar1,(int)((ulong)puVar1 >> 0x10),0x0,puVar3,uVar6);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,extraout_DX,unaff_DI);
  puVar5 = (uchar *)((ulong)puVar7 >> 0x10);
  if (*(int *)((int)puVar7 + 0x80) != 0x0) {
    uVar9->field_0x84 = 0x2;
  }
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,param_3,puVar5,unaff_DI);
  puVar5 = (uchar *)((ulong)puVar7 >> 0x10);
  uVar9->field_0x9e = (uint)puVar7;
  uVar9->field_0xa0 = puVar5;
  uVar4 = pass1_1010_65d0(param_3,(ulong)puVar7 & 0xffff0000 | (ulong)uVar9->field_0x9e,0x88);
  if (uVar4 != 0x0) {
    uVar9->field_0xa8 = 0x1;
  }
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_3,puVar5,unaff_DI);
  uVar9->field_0xa2 = (int)puVar7;
  uVar9->field_0xa4 = (int)((ulong)puVar7 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_078e(ushort *param_1,ushort param_2)

{
  uint uVar1;
  uint uVar2;
  undefined2 *puVar3;
  undefined2 uVar4;
  astruct_500 *uVar5;
  undefined2 uVar6;
  undefined2 *puStack26;
  astruct_18 *paStack6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  uVar5 = (astruct_500 *)param_1;
  *param_1 = 0x1874;
  uVar5->field_0x2 = 0x1018;
  uVar5->field_0x20 = 0x18b0;
  uVar5->field_0x22 = 0x1018;
  PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + -0x1;
  *(undefined2 *)((int)_PTR_LOOP_1050_3962 + uVar5->field_0x12 * 0x2 + -0x4) = 0x0;
  if (PTR_LOOP_1050_3960 == (undefined *)0x0) {
    fn_ptr_1000_17ce(_PTR_LOOP_1050_3962,0x1000);
    _PTR_LOOP_1050_3962 = (astruct_18 *)0x0;
  }
  fn_ptr_1000_17ce((astruct_18 *)uVar5->field_0x94,0x1000);
  fn_ptr_1000_17ce((astruct_18 *)uVar5->field_0x9a,0x1000);
  fn_ptr_1000_17ce((astruct_18 *)uVar5->field_0x88,0x1000);
  fn_ptr_1000_17ce((astruct_18 *)uVar5->field_0x8e,0x1000);
  if (uVar5->field_0x2c != 0x0) {
    if (param_1 == (ushort *)0x0) {
      puVar3 = (undefined2 *)0x0;
      uVar4 = 0x0;
    }
    else {
      puVar3 = &uVar5->field_0x20;
      uVar4 = uVar6;
    }
    pass1_1010_1ea6(uVar5->field_0x2c,CONCAT22(uVar4,puVar3),param_2);
  }
  if (uVar5->field_0x9e != 0x0) {
    if (param_1 == (ushort *)0x0) {
      puVar3 = (undefined2 *)0x0;
      uVar4 = 0x0;
    }
    else {
      puVar3 = &uVar5->field_0x20;
      uVar4 = uVar6;
    }
    pass1_1010_1ea6(uVar5->field_0x9e,CONCAT22(uVar4,puVar3),param_2);
  }
  uVar1 = uVar5->field_0x60;
  uVar2 = uVar5->field_0x62;
  paStack6 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack6,0x1000);
  }
  uVar5->field_0x4c = 0x0;
  if (param_1 == (ushort *)0x0) {
    puVar3 = (undefined2 *)0x0;
    uVar6 = 0x0;
  }
  else {
    puVar3 = &uVar5->field_0x20;
  }
  puStack26 = (undefined2 *)CONCAT22(uVar6,puVar3);
  *puStack26 = 0x389a;
  puVar3[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_0902(ulong *param_1,ulong param_2)

{
  undefined4 uVar1;
  code **ppcVar2;
  astruct_76 **ppaVar3;
  astruct_76 **ppaVar4;
  int iVar5;
  undefined2 uVar6;
  ulong uVar7;
  ulong *puVar8;
  ulong *puVar9;
  
  puVar9 = (ulong *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x28));
  ppaVar3 = (astruct_76 **)((int)param_1 + 0x24);
  puVar8 = (ulong *)((ulong)param_1 & 0xffff0000 | ZEXT24(ppaVar3));
  uVar6 = param_1._2_2_;
  ppaVar4 = ppaVar3;
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),param_2);
  pass1_1030_5a52(CONCAT22(uVar6,ppaVar4),puVar8,puVar9);
  uVar7 = pass1_1008_4772(*ppaVar3);
  *(undefined2 *)((int)param_1 + 0x5a) = (int)uVar7;
  *(undefined2 *)((int)param_1 + 0x5c) = (int)(uVar7 >> 0x10);
  iVar5 = pass1_1018_17f0();
  *(int *)((int)param_1 + 0x12) = iVar5 + 0x2;
  *(undefined2 *)(iVar5 * 0x2 + (int)_PTR_LOOP_1050_3962) = 0x1;
  ppcVar2 = (code **)((int)*param_1 + 0x18);
  (**ppcVar2)();
  *(ulong *)((int)param_1 + 0x3c) = param_2;
  uVar1 = *(undefined4 *)((int)param_1 + 0x2c);
  uVar7 = pass1_1010_ec18((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),
                          param_2 & 0xffff0000 | (ulong)*(uint *)((int)param_1 + 0x3c),(int)param_2,param_2._2_2_);
  *(undefined2 *)((int)param_1 + 0x7c) = (int)uVar7;
  *(undefined2 *)((int)param_1 + 0x7e) = (int)(uVar7 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far get_sys_metrics_1018_09a8(ulong param_1,INT16 param_2)

{
  undefined4 uVar1;
  INT16 IVar2;
  INT16 IVar3;
  uchar *in_DX;
  int iVar4;
  int unaff_DI;
  undefined2 uVar5;
  ushort unaff_SS;
  ushort *puVar6;
  ushort *puVar7;
  ushort *puVar8;
  int local_a;
  int local_8;
  int iStack6;
  INT16 IStack4;
  
  IStack4 = GetSystemMetrics16(param_2);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  iStack6 = *(int *)(iVar4 + 0x12) + -0x2;
  puVar8 = (ushort *)CONCAT22(unaff_SS,&local_8);
  puVar7 = (ushort *)CONCAT22(unaff_SS,&local_a);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)puVar6 & 0xffff0000 | (ulong)((int)puVar6 + 0xe)),puVar7,puVar8);
  *(int *)(iVar4 + 0x18) = iStack6 * IStack4 + local_8 + 0x146;
  *(int *)(iVar4 + 0x1a) = iStack6 * IStack4 + local_a + 0x9;
  IVar2 = GetSystemMetrics16(0x1008);
  uVar1 = *(undefined4 *)(iVar4 + 0x5a);
  *(int *)(iVar4 + 0x1c) = IVar2 * 0x2 + *(int *)((int)uVar1 + 0x4);
  IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  IVar3 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  uVar1 = *(undefined4 *)(iVar4 + 0x5a);
  *(int *)(iVar4 + 0x1e) = IVar3 + IVar2 + *(int *)((int)uVar1 + 0x8);
  return;
}



ulong __stdcall16far pass1_1018_0a50(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0x84) == 0x2) {
    return CONCAT22(*(undefined2 *)(iVar1 + 0x2a),*(undefined2 *)(iVar1 + 0x28));
  }
  return CONCAT22(*(undefined2 *)(iVar1 + 0x26),*(undefined2 *)(iVar1 + 0x24));
}



void __stdcall16far pass1_1018_0a76(ulong param_1,ushort param_2)

{
  undefined2 uVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x84) == 0x1) {
    uVar1 = 0x2;
  }
  else {
    uVar1 = 0x1;
  }
  *(undefined2 *)((int)param_1 + 0x84) = uVar1;
  pass1_1010_1f62(param_2,param_1 & 0xffff | (ulong)uVar2 << 0x10,0x4);
  return;
}



void __stdcall16far pass1_1018_0aa0(ulong param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ushort *)(iVar1 + 0x14) = param_2;
  pass1_1018_04de(*(ulong *)(iVar1 + 0x2c),*(ulong *)(iVar1 + 0x3c));
  return;
}



void __stdcall16far pass1_1018_0ac0(ulong param_1,ulong param_2)

{
  *(ulong *)((int)param_1 + 0x80) = param_2;
  return;
}



ulong __stdcall16far pass1_1018_0ad4(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0x82),*(undefined2 *)((int)param_1 + 0x80));
}
