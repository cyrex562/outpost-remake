


void __stdcall16far pass1_1030_de7c(ulong param_1,ulong param_2,ushort param_3)

{
  BOOL16 BVar1;
  undefined4 local_10 [0x3];
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    local_10[0] = *(undefined4 *)((int)param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c
                      ((ushort)param_2,(ushort)(param_2 >> 0x10),(ushort)local_10,param_3,(char *)0x4,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return;
    }
  }
  return;
}



void __stdcall16far pass1_1030_dec4(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  BOOL16 BVar1;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (((param_3 != 0x0) && (0x1 < (int)PTR_LOOP_1050_0312)) &&
     (BVar1 = read_file_1008_7dee((ushort)param_2,(ushort)(param_2 >> 0x10),(int)param_1 + 0x20,0x0,
                                  (ushort)(param_1 >> 0x10),0x4,0x1008), BVar1 == 0x0)) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
  return;
}



void __stdcall16far pass1_1030_df0c(ulong param_1,ushort param_2)

{
  undefined4 uVar1;
  ulong uVar2;
  long lVar3;
  ushort uVar4;
  int iVar5;
  ulong uVar6;
  undefined2 extraout_DX;
  uint uVar7;
  uint uVar8;
  undefined2 uVar9;
  ushort uVar10;
  undefined2 uStack24;
  undefined2 uStack22;
  uint uStack14;
  uint uStack10;
  
  pass1_1028_b58e(param_1);
  uVar1 = *(undefined4 *)(param_2 + 0x2e);
  uStack10 = (uint)uVar1;
  if ((*(uint *)(param_2 + 0x30) | uStack10) != 0x0) {
    uVar9 = (undefined2)((ulong)uVar1 >> 0x10);
    uVar1 = *(undefined4 *)(uStack10 + 0x210);
    uVar7 = *(uint *)(uStack10 + 0x212);
    uStack14 = (uint)uVar1;
    if ((uVar7 | uStack14) != 0x0) {
      uVar2 = *(ulong *)(uStack14 + 0xa);
      uVar4 = pass1_1030_dfcc(param_1);
      if (uVar4 != 0x0) {
        uStack24 = 0x1;
        uStack22 = 0x0;
        while (CONCAT22(uStack22,uStack24) < uVar2) {
          uVar6 = uVar2;
          uVar10 = uVar4;
          bad_1030_1312();
          uVar8 = uVar7;
          iVar5 = pass1_1030_cde8((int)uVar6,uVar7,uVar10);
          if (-0x1 < iVar5) {
            pass1_1030_cef8(uVar6 & 0xffff | (ulong)uVar7 << 0x10,CONCAT22(extraout_DX,param_2),0x1,iVar5);
            *(undefined4 *)((int)param_1 + 0x20) = *(undefined4 *)((int)uVar6 + 0x4);
            return;
          }
          lVar3 = CONCAT22(uStack22,uStack24) + 0x1;
          uStack24 = (undefined2)lVar3;
          uVar7 = uVar8;
          uStack22 = (undefined2)((ulong)lVar3 >> 0x10);
        }
      }
    }
  }
  return;
}



ushort __stdcall16far pass1_1030_dfcc(ulong param_1)

{
  int iVar1;
  ushort uStack4;
  
  iVar1 = *(int *)((int)param_1 + 0xc);
  if (iVar1 == 0x73) {
LAB_1030_dfde:
    uStack4 = 0x1;
  }
  else {
    if (iVar1 != 0x74) {
      if (iVar1 == 0x75) {
        return 0x3;
      }
      if (iVar1 == 0x77) goto LAB_1030_dfde;
      if (iVar1 != 0x78) {
        return 0x0;
      }
    }
    uStack4 = 0x2;
  }
  return uStack4;
}



astruct_18 * __stdcall16far pass1_1030_e010(astruct_18 *param_1,byte param_2)

{
  uint in_AX;
  
  pass1_1030_dcf4(&param_1->field_0x0,in_AX);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1030_e09e(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x2af7);
  param_1->field_0x0 = 0xe2ae;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCAiInput_1050_5972);
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_e0d4(uchar *param_1,ushort param_2,int param_3)

{
  int *piVar1;
  undefined4 uVar2;
  uint uVar3;
  undefined *puVar4;
  undefined *puVar5;
  uint uVar6;
  uint extraout_DX;
  uchar *puVar7;
  uchar *puVar8;
  int iVar9;
  undefined2 uVar10;
  ushort *puVar11;
  undefined4 uStack42;
  undefined local_1c [0x8];
  undefined4 uStack20;
  uint uStack16;
  undefined4 uStack14;
  ulong uStack10;
  int iStack6;
  ushort uStack4;
  
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x40,param_2,param_1,param_3);
  uStack4 = (ushort)((ulong)puVar11 >> 0x10);
  iStack6 = (int)puVar11;
  uStack10 = pass1_1008_b820((ulong)puVar11,iStack6,uStack4);
  uVar3 = (uint)uStack10;
  uVar6 = (uint)(uStack10 >> 0x10) | uVar3;
  if (uVar6 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x800);
    uStack14 = CONCAT22(uVar6,uVar3);
    uStack16 = (uint)(*(int *)(uVar3 + 0x154) != 0x0);
    pass1_1008_5784((ulong *)CONCAT22(param_2,local_1c),uStack10);
    while( true ) {
      puVar4 = local_1c;
      pass1_1008_5b12(puVar4,param_2);
      uStack20 = CONCAT22(extraout_DX,puVar4);
      puVar7 = (uchar *)(extraout_DX | (uint)puVar4);
      if (puVar7 == (uchar *)0x0) break;
      if (*(int *)(puVar4 + 0x8) != 0x0) {
        uVar2 = *(undefined4 *)(puVar4 + 0xa);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)((ulong)uVar2 >> 0x10));
        puVar8 = puVar7;
        puVar5 = puVar4;
        pass1_1038_354a(CONCAT22(puVar7,puVar4),(uint)puVar4,puVar7);
        if (puVar5 != (undefined *)0x0) {
          uVar10 = (undefined2)((ulong)uStack20 >> 0x10);
          if (uStack16 == 0x0) {
            iVar9 = *(int *)((int)uStack20 + 0xe) * 0xc;
            uStack42 = *(undefined4 *)(iVar9 + 0x58c4);
            uVar3 = *(uint *)(iVar9 + 0x58c8);
          }
          else {
            iVar9 = *(int *)((int)uStack20 + 0xe) * 0xc;
            uStack42 = *(undefined4 *)(iVar9 + 0x58be);
            uVar3 = *(uint *)(iVar9 + 0x58c2);
          }
          uVar6 = uVar3;
          pass1_1038_35a8(CONCAT22(puVar7,puVar4),*(ushort *)(*(int *)((int)uStack20 + 0x10) * 0x2 + (int)uStack42),
                          uVar3,puVar8);
          if (uVar6 != 0x0) {
            uVar10 = (undefined2)((ulong)uStack20 >> 0x10);
            iVar9 = (int)uStack20;
            piVar1 = (int *)(iVar9 + 0x10);
            *piVar1 = *piVar1 + 0x1;
            if ((int)uVar3 <= *(int *)(iVar9 + 0x10)) {
              *(undefined2 *)(iVar9 + 0x10) = 0x0;
            }
          }
        }
      }
    }
  }
  return;
}



void __stdcall16far pass1_1030_e1f4(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x108,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    *(undefined4 *)(param_2 + 0x4) = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = (undefined4 *)(param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *puStack10 = 0xe2ae;
    *(undefined2 *)(param_2 + 0x2) = 0x1030;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_e282(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
struct_1030_e2be(astruct_100 *param_1,ushort param_2,ulong param_3,ulong param_4,ushort param_5,uchar param_6)

{
  astruct_217 *iVar1;
  undefined2 uVar1;
  
  struct_op_1028_d1dc(param_5,param_6,param_1,0x2af7);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_217 *)param_1;
  iVar1->field_0x108 = param_4;
  iVar1->field_0x10c = param_3;
  iVar1->field_0x110 = param_2;
  param_1->field_0x0 = 0xe4ea;
  iVar1->field_0x2 = 0x1030;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined2 __stdcall16far pass1_1030_e300(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  ushort *puVar1;
  
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,param_2,param_3);
  pass1_1010_089e(param_4,(ulong)puVar1,*(ushort *)((int)param_1 + 0x110),0x2);
  return 0x1;
}



ushort __stdcall16far pass1_1030_e328(ulong param_1,ushort param_2,ushort param_3,ushort param_4,uchar param_5)

{
  uint uVar1;
  
  uVar1 = (uint)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x110) == 0x0) {
    pass1_1030_e4ba(param_1);
  }
  else {
    pass1_1030_e410(param_4,param_2,param_5,param_3,param_1 & 0xffff | (ulong)uVar1 << 0x10);
  }
  return 0x1;
}



void __stdcall16far pass1_1030_e34e(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  astruct_404 *in_AX;
  int iVar3;
  astruct_403 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x112,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_403 *)param_1;
    *(undefined4 *)(param_2 + 0x4) = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = (undefined4 *)(param_2 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *(undefined4 *)(param_2 + 0x108) = iVar5->field_0x108;
    *(undefined4 *)(param_2 + 0x10c) = iVar5->field_0x10c;
    *(undefined2 *)(param_2 + 0x110) = iVar5->field_0x110;
    *puStack10 = 0xe4ea;
    *(undefined2 *)(param_2 + 0x2) = 0x1030;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_e410(ushort param_1,ushort param_2,uchar param_3,ushort param_4,ulong param_5)

{
  undefined4 uVar1;
  uchar *puVar2;
  undefined2 uVar3;
  ushort *puVar4;
  undefined local_10 [0x6];
  undefined local_a [0x4];
  uint uStack6;
  uint uStack4;
  
  uVar1 = *(undefined4 *)((int)param_5 + 0x10c);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  puVar2 = (uchar *)(param_4 | param_2);
  if (puVar2 != (uchar *)0x0) {
    uStack6 = param_2;
    uStack4 = param_4;
    pass1_1038_4fd8(param_2,CONCAT22(param_4,param_2),0x21);
    if (param_2 == 0x0) {
      pass1_1020_a43e(param_1,puVar2,(ushort *)CONCAT22(param_1,local_a));
      puVar4 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_10),0x0,0x2,0xfffd);
      uVar3 = (undefined2)((ulong)puVar4 >> 0x10);
      pass1_1020_a49a(param_1,param_3,uVar3,CONCAT22(param_1,local_a),(int *)CONCAT22(param_1,local_10),0x7a);
      pass1_1008_3e76((ushort *)CONCAT22(param_1,local_10),0x0,0x3,0xfffe);
      pass1_1020_a49a(param_1,param_3,uVar3,CONCAT22(param_1,local_a),(int *)CONCAT22(param_1,local_10),0x7a);
      pass1_1008_3e76((ushort *)CONCAT22(param_1,local_10),0x0,0x3,0xfffd);
      pass1_1020_a49a(param_1,param_3,uVar3,CONCAT22(param_1,local_a),(int *)CONCAT22(param_1,local_10),0x21);
    }
  }
  return;
}



void __stdcall16far pass1_1030_e4ba(void)

{
  return;
}



astruct_18 * __stdcall16far pass1_1030_e4be(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1030_e4fa(astruct_100 *param_1,ulong param_2,ushort param_3,uchar param_4)

{
  astruct_289 *iVar1;
  uchar *puVar1;
  
  struct_op_1028_d1dc(param_3,param_4,param_1,0x3e80);
  puVar1 = (uchar *)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_289 *)param_1;
  iVar1->field_0x108 = param_2;
  param_1->field_0x0 = 0xe62e;
  iVar1->field_0x2 = 0x1030;
  sys_1000_3f9c(&iVar1->field_0x8,puVar1,(ushort)s_SCKillBldg__0x_08lx_1050_597c,(ushort)&USHORT_1050_1050,
                (ushort)iVar1->field_0x108,&stack0xfffe,puVar1,0x1000,param_3,param_4);
  return;
}



ushort __stdcall16far pass1_1030_e540(void)

{
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1030_e546(ulong param_1,ushort param_2)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x108);
  pass1_1028_e332(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10),param_2);
  return 0x1;
}



void __stdcall16far pass1_1030_e564(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_405 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x10c,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_405 *)param_1;
    *(undefined4 *)(param_2 + 0x4) = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = (undefined4 *)(param_2 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *(undefined4 *)(param_2 + 0x108) = iVar5->field_0x108;
    *puStack10 = 0xe62e;
    *(undefined2 *)(param_2 + 0x2) = 0x1030;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_e602(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1030_e63e(astruct_100 *param_1,ushort param_2,ushort param_3,uchar param_4)

{
  int iVar1;
  undefined2 uVar2;
  
  iVar1 = (int)param_1;
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  struct_op_1028_d1dc(param_3,param_4,param_1,0xf9f);
  *(ushort *)(iVar1 + 0x108) = param_2;
  param_1->field_0x0 = 0xe78a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x8)),s_SCKillColony_1050_5990);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1030_e67c(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  ushort uVar1;
  astruct_67 *paVar2;
  
  paVar2 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,param_2,param_3);
  uVar1 = pass1_1008_aaa8((ushort)paVar2,(ushort)((ulong)paVar2 >> 0x10),*(ushort *)((int)param_1 + 0x108));
  if (uVar1 != 0x0) {
    post_win_msg_1008_a0e4(paVar2,0x0,0x0,0x1,0x0,uVar1,0x1008,param_4);
  }
  return 0x1;
}



void __stdcall16far pass1_1030_e6c2(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_406 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x10a,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_406 *)param_1;
    *(undefined4 *)(param_2 + 0x4) = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = (undefined4 *)(param_2 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *(undefined2 *)(param_2 + 0x108) = iVar5->field_0x108;
    *puStack10 = 0xe78a;
    *(undefined2 *)(param_2 + 0x2) = 0x1030;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_e75e(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1030_e79a(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0xf9f);
  param_1->field_0x0 = 0xe890;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e
            ((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCKillRebelColony_1050_599e);
  return param_1;
}



ushort __stdcall16far pass1_1030_e7d0(void)

{
  return 0x1;
}



void __stdcall16far pass1_1030_e7d6(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x108,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    *(undefined4 *)(param_2 + 0x4) = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = (undefined4 *)(param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *puStack10 = 0xe890;
    *(undefined2 *)(param_2 + 0x2) = 0x1030;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_e864(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
pass1_1030_e8a0(astruct_100 *param_1,ulong param_2,ushort param_3,ulong param_4,ushort param_5,uchar param_6)

{
  astruct_408 *iVar1;
  uchar *puVar1;
  
  struct_op_1028_d1dc(param_5,param_6,param_1,0x2710);
  puVar1 = (uchar *)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_408 *)param_1;
  iVar1->field_0x108 = param_2;
  iVar1->field_0x10c = param_4;
  iVar1->field_0x110 = param_3;
  param_1->field_0x0 = 0xeb40;
  iVar1->field_0x2 = 0x1030;
  sys_1000_3f9c(&iVar1->field_0x8,puVar1,(ushort)s_SCMoveBas_to_0x_08lx_1050_59b0,(ushort)&USHORT_1050_1050,
                (ushort)iVar1->field_0x10c,&stack0xfffe,puVar1,0x1000,param_5,param_6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far
pass1_1030_e8f8(ulong param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6)

{
  uint uVar1;
  uint uVar2;
  undefined4 uVar3;
  int iVar4;
  undefined2 uVar5;
  ulong uVar6;
  astruct_18 *paStack20;
  ulong uStack6;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x108) != 0x0) {
    uVar3 = *(undefined4 *)(iVar4 + 0x10c);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar3,(uint)((ulong)uVar3 >> 0x10));
    uStack6 = CONCAT22(param_3,param_2);
    uVar6 = struct_op_1030_73a8(CONCAT22(param_3,param_2));
    if (*(int *)((int)uVar6 + 0xc) == *(int *)(iVar4 + 0x110)) {
      pass1_1030_ea50(param_1,uStack6,param_4,param_5,param_6);
    }
    uVar1 = *(uint *)(iVar4 + 0x108);
    uVar2 = *(uint *)(iVar4 + 0x10a);
    paStack20 = (astruct_18 *)CONCAT22(uVar2,uVar1);
    if ((uVar2 | uVar1) != 0x0) {
      fn_ptr_1020_ba7e((ulong *)CONCAT22(uVar2,uVar1));
      fn_ptr_1000_17ce(paStack20,0x1000);
    }
    *(undefined4 *)(iVar4 + 0x108) = 0x0;
  }
  return 0x1;
}



void __stdcall16far pass1_1030_e98e(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_407 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x112,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_407 *)param_1;
    *(undefined4 *)(param_2 + 0x4) = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = (undefined4 *)(param_2 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *(undefined4 *)(param_2 + 0x108) = iVar5->field_0x108;
    *(undefined4 *)(param_2 + 0x10c) = iVar5->field_0x10c;
    *(undefined2 *)(param_2 + 0x110) = iVar5->field_0x110;
    *puStack10 = 0xeb40;
    *(undefined2 *)(param_2 + 0x2) = 0x1030;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_ea50(ulong param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  int iVar3;
  undefined2 uVar4;
  ulong uVar5;
  ulong local_12;
  uint local_e;
  int iStack12;
  uint uStack10;
  uint uStack8;
  undefined4 uStack6;
  
  uStack6 = 0x1869f;
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)(iVar3 + 0x110),0x3);
  if (BVar2 != 0x0) {
    uVar5 = struct_op_1030_73a8(param_2);
    iStack12 = (int)(uVar5 >> 0x10);
    local_e = (uint)uVar5;
    uStack6 = pass1_1028_45e2(uVar5,local_e,iStack12,param_5);
  }
  uVar1 = *(undefined4 *)(iVar3 + 0x108);
  uStack8 = *(uint *)((int)uVar1 + 0x4);
  uStack10 = 0x0;
  while( true ) {
    if (uStack8 <= uStack10) {
      return;
    }
    pass1_1020_bb16(*(ulong **)(iVar3 + 0x108),(ulong *)CONCAT22(param_5,&local_12),(ushort *)CONCAT22(param_5,&local_e)
                    ,uStack10);
    if (uStack6 < local_12) {
      pass1_1030_7ddc(param_2,uStack6,local_e,(uint)uStack6,uStack6._2_2_,param_3,param_4,param_5);
      uStack6 = 0x0;
    }
    else {
      uStack6 = uStack6 - local_12;
      pass1_1030_7ddc(param_2,local_12,local_e,(uint)local_12,uStack6._2_2_,param_3,param_4,param_5);
    }
    if (((uint)uStack6._2_2_ | (uint)uStack6) == 0x0) break;
    uStack10 = uStack10 + 0x1;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_eb14(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1030_eb50(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x1f3f);
  param_1->field_0x0 = 0xecb2;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCMines_1050_59c6);
  return param_1;
}



ushort __stdcall16far pass1_1030_eb86(uint param_1,ushort param_2)

{
  int iVar1;
  code **ppcVar2;
  undefined *puVar3;
  uint uVar4;
  uint extraout_DX;
  undefined4 *puStack24;
  undefined local_14 [0x12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x700);
  while( true ) {
    uVar4 = param_1;
    puVar3 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar3));
    puStack24 = (undefined4 *)CONCAT22(uVar4,puVar3);
    param_1 = uVar4 | (uint)puVar3;
    if (param_1 == 0x0) break;
    if (*(int *)(puVar3 + 0x12) == 0x5) {
      iVar1 = *(int *)(puVar3 + 0xc);
      if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
         ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
        ppcVar2 = (code **)((int)*puStack24 + 0x2c);
        (**ppcVar2)((int)&USHORT_1050_1028);
        param_1 = extraout_DX;
      }
    }
  }
  return 0x1;
}



void __stdcall16far pass1_1030_ebf8(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x108,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    *(undefined4 *)(param_2 + 0x4) = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = (undefined4 *)(param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *puStack10 = 0xecb2;
    *(undefined2 *)(param_2 + 0x2) = 0x1030;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_ec86(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * __stdcall16far pass1_1030_ecc2(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0xf9f);
  param_1->field_0x0 = 0xb96;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCMorale_1050_59ce);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_ecf8(ulong param_1,ulong param_2,int param_3,ushort param_4,uchar param_5)

{
  int iVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  ushort uVar4;
  ulong uVar5;
  undefined *puVar6;
  int iVar7;
  ulong uVar8;
  uint uVar9;
  uint uVar10;
  uint uVar11;
  uint uVar12;
  ulong uVar13;
  undefined2 uVar14;
  bool bVar15;
  ushort *puVar16;
  undefined4 *puVar17;
  ushort uVar18;
  ulong uStack64;
  int iStack56;
  ushort uStack54;
  ulong uStack38;
  undefined local_22 [0x12];
  undefined2 uStack16;
  undefined2 uStack14;
  ushort uStack12;
  uint uStack10;
  uint uStack8;
  uint uStack6;
  undefined2 uStack4;
  
  uStack12 = 0x0;
  puVar16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,(uchar *)param_2,param_3);
  uVar13 = param_2 & 0xffff0000 | (ulong)puVar16 >> 0x10;
  uStack10 = (uint)puVar16;
  uStack4 = (undefined2)((ulong)puVar16 >> 0x10);
  uStack6 = uStack10;
  pass1_1010_ed3e((ulong)puVar16);
  uStack8 = (uint)uVar13;
  uVar13 = uVar13 & 0xffff0000 | (ulong)(uStack8 | uStack10);
  if ((uStack8 | uStack10) != 0x0) {
    uStack12 = pass1_1030_2aaa(CONCAT22(uStack8,uStack10));
  }
  if ((int)uStack12 < 0x2) {
    uStack12 = 0x0;
  }
  puVar16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,(uchar *)uVar13,param_3);
  uVar13 = uVar13 & 0xffff0000 | (ulong)puVar16 >> 0x10;
  uStack16 = SUB42(puVar16,0x0);
  uStack14 = (undefined2)((ulong)puVar16 >> 0x10);
  if ((0x0 < (int)PTR_LOOP_1050_13ae) && (!SBORROW2((int)PTR_LOOP_1050_13ae,0x1))) {
    if (PTR_LOOP_1050_13ae == (undefined *)&PTR_LOOP_1050_0002 || (int)(PTR_LOOP_1050_13ae + -0x1) < 0x1) {
      if (0x6 < (int)uStack12) {
        uStack12 = uStack12 - 0x2;
        goto LAB_1030_ed5b;
      }
      bVar15 = SBORROW2(uStack12,0x4);
      iVar1 = uStack12 - 0x4;
    }
    else {
      if (PTR_LOOP_1050_13ae != (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1)) goto LAB_1030_ed5b;
      bVar15 = SBORROW2(uStack12,0x7);
      iVar1 = uStack12 - 0x7;
    }
    if (bVar15 == iVar1 < 0x0) {
      uStack12 = uStack12 - 0x1;
    }
  }
LAB_1030_ed5b:
  pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,local_22)),0x1,0x0,0x400);
  while( true ) {
    puVar6 = local_22;
    pass1_1028_e4ec(CONCAT22(param_4,puVar6));
    uVar9 = (uint)uVar13;
    uStack38 = CONCAT22(uVar9,puVar6);
    if ((uVar9 | (uint)puVar6) == 0x0) break;
    uVar10 = (uint)*(undefined4 *)(puVar6 + 0x1f6);
    uVar13 = uVar13 & 0xffff0000 | (ulong)*(uint *)(puVar6 + 0x1f8);
    if ((*(int *)(puVar6 + 0x1fe) != 0x0) && (*(long *)(puVar6 + 0x200) != 0x8000002)) {
      pass1_1030_38b8();
      uVar10 = (uint)uVar13 | uVar10;
      uVar8 = uVar13 & 0xffff0000;
      uVar13 = uVar8 | uVar10;
      if (uVar10 != 0x0) {
        puVar2 = *(undefined4 **)(puVar6 + 0xc);
        uVar10 = *(uint *)(puVar6 + 0xe);
        uVar8 = uVar8 | uVar10;
        ppcVar3 = (code **)((int)*puVar2 + 0x10);
        puVar17 = puVar2;
        (**ppcVar3)((int)&USHORT_1050_1028,(int)puVar2,uVar10);
        uVar5 = (ulong)puVar17 & 0xffff | uVar8 << 0x10;
        uStack54 = *(ushort *)(puVar6 + 0x18);
        uVar14 = SUB42(&PTR_LOOP_1050_1038,0x0);
        pass1_1038_4760(CONCAT22(uVar9,puVar6));
        iVar1 = *(int *)(puVar6 + 0x22);
        iStack56 = iVar1 / 0xa;
        uVar13 = uVar8 & 0xffff0000 | (long)iVar1 % 0xa & 0xffffU;
        iVar1 = *(int *)(puVar6 + 0x24);
        if (iVar1 < 0x33) {
          if (iVar1 < 0x32) {
            iStack56 = iStack56 + -0x1;
          }
        }
        else {
          uStack54 = uStack54 + 0x1;
        }
        for (uStack64 = 0x0; uStack64 < uVar5; uStack64 = uStack64 + 0x1) {
          ppcVar3 = (code **)((int)*puVar2 + 0x4);
          uVar8 = uVar5;
          (**ppcVar3)(uVar14,(char)puVar2,(int)((ulong)puVar2 >> 0x10),(int)uStack64,(int)(uStack64 >> 0x10));
          uVar10 = (uint)uVar8;
          uVar11 = (uint)uVar13;
          uVar12 = uVar11 | uVar10;
          uVar13 = uVar13 & 0xffff0000 | (ulong)uVar12;
          if (uVar12 != 0x0) {
            uVar14 = SUB42(&USHORT_1050_1028,0x0);
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar10,uVar11);
            puVar17 = (undefined4 *)struct_op_1030_73a8(CONCAT22((int)uVar13,uVar10));
            uVar10 = (uint)puVar17;
            uVar11 = (uint)((ulong)puVar17 >> 0x10);
            uVar13 = uVar13 & 0xffff0000 | (ulong)(uVar11 | uVar10);
            if (((uVar11 | uVar10) != 0x0) && (*(int *)(uVar10 + 0x12) == 0x5)) {
              ppcVar3 = (code **)((int)*puVar17 + 0x48);
              (**ppcVar3)((int)&USHORT_1050_1028,uVar10,uVar11);
              if ((int)uVar10 < 0x0) {
                iStack56 = iStack56 + uVar10;
              }
              else {
                uStack54 = uStack54 + uVar10;
              }
            }
          }
        }
        iStack56 = iStack56 - uStack12;
        iVar1 = *(int *)(puVar6 + 0x20a);
        uVar18 = (ushort)(param_1 >> 0x10);
        uVar4 = (ushort)param_1;
        iVar7 = iVar1;
        pass1_1038_01c0(uVar4,uVar18,uStack38,param_4);
        iVar7 = iVar7 - iVar1;
        iStack56 = iStack56 - iVar7;
        pass1_1038_008e(uVar4,uVar18,uStack38,(uchar *)uVar13,param_3,param_4);
        if (iVar7 < 0x0) {
          iStack56 = iStack56 + iVar7;
        }
        else {
          uStack54 = uStack54 + iVar7;
        }
        if (0x3e8 < (int)uStack54) {
          uStack54 = 0x3e8;
        }
        if ((int)uStack54 < 0x0) {
          uStack54 = 0x0;
        }
        uStack54 = uStack54 + iStack56;
        if (0x3e8 < (int)uStack54) {
          uStack54 = 0x3e8;
        }
        if ((int)uStack54 < 0x0) {
          uStack54 = 0x0;
        }
        pass1_1038_4d0e(uStack38,uStack54);
        if (*(long *)(puVar6 + 0x4) == 0x4000001) {
          pass1_1038_08d4(uVar4,CONCAT22(uStack54,uVar18),uStack38,uVar13,param_4,param_5);
        }
        pass1_1038_095e(uVar4,uVar18,uStack54,uStack38,(uchar *)uVar13,param_3,param_4);
      }
    }
  }
  return;
}
