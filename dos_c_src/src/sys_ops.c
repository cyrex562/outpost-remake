
void __stdcall16far struct_1040_b082(astruct_57 *param_1,ulong param_2)

{
  astruct_437 *iVar1;
  undefined2 uVar1;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,(ushort)param_2,(ushort)(param_2 >> 0x10));
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_437 *)param_1;
  iVar1->field_0x8e = 0x0;
  iVar1->field_0x90 = 0x0;
  *(undefined2 *)param_1 = 0xb772;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  return;
}


void __stdcall16far pass1_1040_b040(astruct_57 *param_1,ulong param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,*(ushort *)((int)param_2 + 0x12),param_3);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0x8e) = 0x0;
  *(ulong *)(iVar1 + 0x90) = param_2;
  *(undefined2 *)param_1 = 0xb772;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return;
}


void __stdcall16far pass1_1040_b0bc(astruct_57 *param_1,ulong param_2,ulong param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,(ushort)param_3,(ushort)(param_3 >> 0x10));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0x8e) = 0x0;
  *(ulong *)(iVar1 + 0x90) = param_2;
  *(undefined2 *)param_1 = 0xb772;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return;
}


void __stdcall16far
pass1_1040_4068(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  uchar *puVar1;
  astruct_723 *iVar2;
  undefined2 uVar2;
  ushort *puVar3;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfb7,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_723 *)param_1;
  *(undefined4 *)&iVar2->field_0x8e = 0x0;
  iVar2->field_0x92 = 0x0;
  iVar2->field_0x9a = 0x0;
  *(undefined2 *)param_1 = 0x4466;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  iVar2->field_0x76 = 0x1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_8,param_6,param_7);
  puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
  iVar2->field_0x8e = (int)puVar3;
  iVar2->field_0x90 = puVar1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_8,puVar1,param_7);
  iVar2->field_0x96 = (int)puVar3;
  iVar2->field_0x98 = (int)((ulong)puVar3 >> 0x10);
  return;
}




void __stdcall16far
get_sys_metrics_1040_7728(astruct_57 *param_1,ushort param_2,ulong param_3,ushort param_4,ushort param_5)

{
  INT16 IVar1;
  astruct_57 *iVar2;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_57 *)param_1;
  *(undefined2 *)param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  *(undefined2 *)param_1 = 0x3aa8;
  iVar2->field_0x2 = 0x1008;
  iVar2->field_0x4 = 0x0;
  iVar2->field_0x6 = 0x0;
  iVar2->field_0x8 = param_5;
  iVar2->field_0xa = param_4;
  iVar2->field_0xc = 0x0;
  iVar2->field_0x60 = 0x0;
  iVar2->field_0x62 = 0x0;
  iVar2->field_0x64 = 0x0;
  iVar2->field_0x66 = 0x0;
  iVar2->field_0x68 = 0x0;
  iVar2->field_0x6a = param_3;
  iVar2->field_0x6e = param_2;
  iVar2->field_0x70 = 0x0;
  iVar2->field_0x74 = 0x0;
  iVar2->field_0x76 = 0x0;
  iVar2->field_0x78 = 0x0;
  iVar2->field_0x8a = 0x0;
  iVar2->field_0x8c = 0x0;
  *(undefined2 *)param_1 = 0x840c;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x10),(char *)0x10505db0);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x7a),(WNDCLASS16 *)0x0,0x8);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x82),(WNDCLASS16 *)0x0,0x8);
  IVar1 = GetSystemMetrics16(0x1000);
  iVar2->field_0x62 = IVar1;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar2->field_0x64 = IVar1;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar2->field_0x66 = IVar1;
  return;
}




void __stdcall16far pass1_1040_44d2(astruct_57 *param_1,ulong param_2,ushort param_3,uint param_4,uchar *param_5)

{
  undefined4 uVar1;
  uint uVar2;
  uchar *puVar3;
  int iVar4;
  undefined2 uVar5;
  int iVar6;
  undefined2 uVar7;
  int *piStack8;
  
  iVar6 = (int)param_1;
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  struct_1040_b082(param_1,CONCAT22(param_3,0xfa2));
  *(undefined2 *)param_1 = 0x4824;
  *(undefined2 *)(iVar6 + 0x2) = (int)&PTR_LOOP_1050_1040;
  mem_op_1000_179c(0x18,param_5,0x1000);
  puVar3 = (uchar *)((uint)param_5 | param_4);
  if (puVar3 == (uchar *)0x0) {
    *(undefined4 *)(iVar6 + 0x90) = 0x0;
  }
  else {
    struct_1040_a598((ushort *)CONCAT22(param_5,param_4));
    *(uint *)(iVar6 + 0x90) = param_4;
    *(uchar **)(iVar6 + 0x92) = puVar3;
  }
  *(undefined2 *)*(undefined4 *)(iVar6 + 0x90) = 0x14;
  iVar4 = **(int **)(iVar6 + 0x90);
  uVar2 = iVar4 * 0xa + 0x2;
  mem_op_1000_179c(uVar2,puVar3,0x1000);
  piStack8 = (int *)CONCAT22(puVar3,uVar2);
  if (((uint)puVar3 | uVar2) == 0x0) {
    uVar1 = *(undefined4 *)(iVar6 + 0x90);
    *(undefined4 *)((int)uVar1 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar4;
    pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iVar4,0xa,uVar2 + 0x2,(ushort)puVar3);
    uVar1 = *(undefined4 *)(iVar6 + 0x90);
    uVar5 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar4 = (int)uVar1;
    *(int *)(iVar4 + 0x2) = uVar2 + 0x2;
    *(uchar **)(iVar4 + 0x4) = puVar3;
  }
  uVar1 = *(undefined4 *)(iVar6 + 0x90);
  *(ulong *)((int)uVar1 + 0x6) = param_2;
  uVar1 = *(undefined4 *)(iVar6 + 0x90);
  *(undefined2 *)((int)uVar1 + 0xa) = 0x1;
  uVar1 = *(undefined4 *)(iVar6 + 0x90);
  *(undefined2 *)((int)uVar1 + 0x12) = *(undefined2 *)(iVar6 + 0xa);
  return;
}



void __stdcall16far
pass1_1040_45e8(int param_1,ushort param_2,ushort param_3,ulong param_4,uchar *param_5,ushort param_6,ushort param_7)

{
  astruct_18 *paVar1;
  code **ppcVar2;
  undefined4 uVar3;
  uint uVar4;
  astruct_18 *paVar5;
  uchar *puVar6;
  uchar *puVar7;
  int iVar8;
  int unaff_DI;
  undefined2 uVar9;
  astruct_20 *paVar10;
  int *piStack16;
  
  if (param_4._2_2_ != 0xeb) {
    pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,param_6,param_7);
    return;
  }
  paVar10 = (astruct_20 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_7,param_5,unaff_DI);
  puVar6 = (uchar *)((ulong)paVar10 >> 0x10);
  paVar1 = *(astruct_18 **)(param_1 + 0x90);
  if (paVar1 != (astruct_18 *)0x0) {
    paVar5 = paVar1;
    mem_op_1000_179c(0x18,puVar6,0x1000);
    uVar4 = (uint)paVar5;
    puVar7 = (uchar *)((uint)puVar6 | uVar4);
    if (puVar7 == (uchar *)0x0) {
      uVar4 = 0x0;
      puVar7 = (uchar *)0x0;
    }
    else {
      struct_1040_a598((ushort *)((ulong)paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
    }
    *(uint *)(param_1 + 0x90) = uVar4;
    *(uchar **)(param_1 + 0x92) = puVar7;
    *(undefined2 *)*(undefined4 *)(param_1 + 0x90) = 0x14;
    iVar8 = **(int **)(param_1 + 0x90);
    uVar4 = iVar8 * 0xa + 0x2;
    mem_op_1000_179c(uVar4,puVar7,0x1000);
    piStack16 = (int *)CONCAT22(puVar7,uVar4);
    if (((uint)puVar7 | uVar4) == 0x0) {
      uVar3 = *(undefined4 *)(param_1 + 0x90);
      *(undefined4 *)((int)uVar3 + 0x2) = 0x0;
    }
    else {
      *piStack16 = iVar8;
      pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iVar8,0xa,uVar4 + 0x2,(ushort)puVar7);
      uVar3 = *(undefined4 *)(param_1 + 0x90);
      uVar9 = (undefined2)((ulong)uVar3 >> 0x10);
      iVar8 = (int)uVar3;
      *(int *)(iVar8 + 0x2) = uVar4 + 0x2;
      *(uchar **)(iVar8 + 0x4) = puVar7;
    }
    uVar3 = *(undefined4 *)(param_1 + 0x90);
    *(undefined4 *)((int)uVar3 + 0x6) = *(undefined4 *)((int)paVar1 + 0x6);
    uVar3 = *(undefined4 *)(param_1 + 0x90);
    *(undefined2 *)((int)uVar3 + 0xa) = 0x1;
    uVar3 = *(undefined4 *)(param_1 + 0x90);
    *(undefined2 *)((int)uVar3 + 0x12) = *(undefined2 *)(param_1 + 0xa);
    pass1_1010_a50c(paVar10,0x10505d40,*(ulong *)(param_1 + 0x90));
    if (paVar1 != (astruct_18 *)0x0) {
      pass1_1040_a5d0((ulong)paVar1);
      fn_ptr_1000_17ce(paVar1,0x1000);
    }
    ppcVar2 = (code **)((int)*(undefined4 *)CONCAT22(param_2,param_1) + 0x70);
    (**ppcVar2)();
  }
  return;
}



void __stdcall16far
pass1_1040_48a0(astruct_57 *param_1,ushort param_2,ulong param_3,ushort param_4,uchar *param_5,ushort param_6)

{
  int iVar1;
  int *piVar2;
  uint uVar3;
  uchar *puVar4;
  uchar *puVar5;
  astruct_444 *iVar5;
  astruct_445 *iVar6;
  int unaff_DI;
  undefined2 uVar6;
  undefined2 uVar7;
  ushort *puVar8;
  int *piStack8;
  
  struct_1040_b082(param_1,CONCAT22(param_4,0xfa1));
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_444 *)param_1;
  iVar5->field_0x94 = 0x0;
  *(int *)param_1 = (int)&PTR_LOOP_1050_4e18;
  iVar5->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_5,unaff_DI);
  puVar4 = (uchar *)((ulong)puVar8 >> 0x10);
  uVar3 = (uint)puVar8;
  *(uint *)&iVar5->field_0x94 = uVar3;
  *(uchar **)((int)&iVar5->field_0x94 + 0x2) = puVar4;
  mem_op_1000_179c(0x18,puVar4,0x1000);
  puVar5 = (uchar *)((uint)puVar4 | uVar3);
  if (puVar5 == (uchar *)0x0) {
    iVar5->field_0x90 = (int *)0x0;
  }
  else {
    struct_1040_a598((ushort *)CONCAT22(puVar4,uVar3));
    *(uint *)&iVar5->field_0x90 = uVar3;
    *(uchar **)((int)&iVar5->field_0x90 + 0x2) = puVar5;
  }
  *iVar5->field_0x90 = 0x7;
  iVar1 = *iVar5->field_0x90;
  uVar3 = iVar1 * 0xa + 0x2;
  mem_op_1000_179c(uVar3,puVar5,0x1000);
  piStack8 = (int *)CONCAT22(puVar5,uVar3);
  if (((uint)puVar5 | uVar3) == 0x0) {
    piVar2 = iVar5->field_0x90;
    *(undefined4 *)((int)piVar2 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar1;
    pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iVar1,0xa,uVar3 + 0x2,(ushort)puVar5);
    piVar2 = iVar5->field_0x90;
    uVar7 = (undefined2)((ulong)piVar2 >> 0x10);
    iVar6 = (astruct_445 *)piVar2;
    iVar6->field_0x2 = uVar3 + 0x2;
    iVar6->field_0x4 = puVar5;
  }
  piVar2 = iVar5->field_0x90;
  *(ulong *)((int)piVar2 + 0x6) = param_3;
  piVar2 = iVar5->field_0x90;
  *(ushort *)((int)piVar2 + 0xa) = param_2;
  piVar2 = iVar5->field_0x90;
  *(undefined2 *)((int)piVar2 + 0x12) = iVar5->field_0xa;
  iVar1 = *(int *)&iVar5->field_0x90;
  uVar7 = *(undefined2 *)((int)&iVar5->field_0x90 + 0x2);
  pass1_1010_debe(iVar5->field_0x94,*(ushort *)(iVar1 + 0xa),(ushort *)CONCAT22(uVar7,iVar1 + 0x10),
                  (ulong *)CONCAT22(uVar7,iVar1 + 0xc),param_3,param_6);
  return;
}



void __stdcall16far
pass1_1040_23ea(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6,
               ushort param_7)

{
  undefined4 uVar1;
  astruct_436 *iVar2;
  int unaff_DI;
  undefined2 uVar2;
  ushort *puVar3;
  
  get_sys_metrics_1040_7728(param_1,0x9a,param_2,0xfbd,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_436 *)param_1;
  *(undefined4 *)&iVar2->field_0x8e = 0x0;
  iVar2->field_0x92 = 0x0;
  iVar2->field_0x94 = 0x0;
  *(undefined2 *)param_1 = 0x2956;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  iVar2->field_0x8a = 0x26;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,param_6,(uchar *)param_7,unaff_DI);
  iVar2->field_0x8e = (int)puVar3;
  iVar2->field_0x90 = (int)((ulong)puVar3 >> 0x10);
  uVar1 = *(undefined4 *)&iVar2->field_0x8e;
  iVar2->field_0x92 = *(undefined2 *)((int)uVar1 + 0x28);
  return;
}



void __stdcall16far
pass1_1040_2ea2(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_720 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x180,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_720 *)param_1;
  iVar1->field_0x8e = 0x0;
  iVar1->field_0x90 = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  *(undefined4 *)&iVar1->field_0x96 = 0x0;
  *(undefined2 *)param_1 = 0x3436;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3c,param_8,param_6,param_7);
  iVar1->field_0x96 = (int)puVar2;
  iVar1->field_0x98 = (int)((ulong)puVar2 >> 0x10);
  return;
}



void __stdcall16far
pass1_1040_34a2(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_721 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x192,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_721 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  iVar1->field_0x96 = 0x0;
  iVar1->field_0x98 = 0x0;
  *(int *)param_1 = (int)s_Null_Ptr_1050_38f3 + 0x7;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3c,param_8,param_6,param_7);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return;
}



astruct_57 * __stdcall16far
pass1_1040_123e(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_717 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfd1,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_717 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  *(undefined2 *)param_1 = 0x17b0;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x46,param_8,param_6,param_7);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return param_1;
}



void __stdcall16far
pass1_1040_181c(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               ushort param_7)

{
  astruct_433 *iVar1;
  int unaff_DI;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfbb,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_433 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  *(undefined2 *)param_1 = 0x1c48;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_7,param_6,unaff_DI);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return;
}



void __stdcall16far
pass1_1040_1cb4(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  uchar *puVar1;
  astruct_718 *iVar2;
  undefined2 uVar2;
  ushort *puVar3;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xe8,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_718 *)param_1;
  *(undefined4 *)&iVar2->field_0x8e = 0x0;
  *(undefined4 *)&iVar2->field_0x92 = 0x0;
  *(undefined2 *)param_1 = 0x1eee;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_8,param_6,param_7);
  puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
  iVar2->field_0x8e = (int)puVar3;
  iVar2->field_0x90 = puVar1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_8,puVar1,param_7);
  iVar2->field_0x92 = (int)puVar3;
  iVar2->field_0x94 = (int)((ulong)puVar3 >> 0x10);
  return;
}



void __stdcall16far pass1_1040_1f5a(astruct_57 *param_1,ushort param_2,int param_3,ushort param_4)

{
  int *piVar1;
  uchar *puVar2;
  astruct_719 *iVar3;
  astruct_43 *paVar3;
  ulong uVar4;
  ushort *puVar5;
  int iVar6;
  undefined2 uVar7;
  int iVar8;
  int iVar9;
  undefined2 uVar10;
  undefined4 local_16;
  undefined4 uStack18;
  
  iVar6 = (int)param_1;
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfcf,param_2);
  *(undefined4 *)(iVar6 + 0x8e) = 0x0;
  *(undefined4 *)(iVar6 + 0xa2) = 0x0;
  *(undefined4 *)(iVar6 + 0xa6) = 0x0;
  *(undefined2 *)param_1 = 0x237e;
  *(undefined2 *)(iVar6 + 0x2) = (int)&PTR_LOOP_1050_1040;
  paVar3 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1cc,param_4);
  *(undefined2 *)(iVar6 + 0x8e) = (int)paVar3;
  *(undefined2 *)(iVar6 + 0x90) = (int)((ulong)paVar3 >> 0x10);
  uVar4 = pass1_1008_4772((astruct_76 *)((ulong)paVar3 & 0xffff0000 | (ulong)*(uint *)(iVar6 + 0x8e)));
  puVar2 = (uchar *)(uVar4 >> 0x10);
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,puVar2,param_3);
  local_16 = CONCAT22(*(int *)((int)uVar4 + 0x8) + 0xa,0xa);
  uStack18 = CONCAT22(0x1d6,*(int *)((int)uVar4 + 0x4) + -0xa);
  *(undefined4 *)(iVar6 + 0x92) = local_16;
  *(undefined4 *)(iVar6 + 0x96) = uStack18;
  *(undefined4 *)(iVar6 + 0x9a) = local_16;
  *(undefined4 *)(iVar6 + 0x9e) = uStack18;
  piVar1 = (int *)(iVar6 + 0x9c);
  *piVar1 = *piVar1 + 0x14;
  iVar9 = iVar6 + 0xa2;
  iVar8 = iVar6 + 0xa6;
  uVar10 = uVar7;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,(uchar *)((ulong)puVar5 >> 0x10),iVar6 + 0xa2);
  pass1_1010_0538((ulong)puVar5,(char **)CONCAT22(uVar7,iVar8),(char **)CONCAT22(uVar10,iVar9),0x1010,param_4);
  return;
}



void __stdcall16far pass1_1038_eeda(astruct_57 *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  astruct_714 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x166,param_2);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_714 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  *(undefined2 *)param_1 = 0x67c;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,param_5,param_3,param_4);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  iVar1->field_0x74 = 0x1;
  return;
}



astruct_57 * __stdcall16far
pass1_1040_06e8(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               ushort param_7)

{
  int iVar1;
  int unaff_DI;
  undefined2 uVar2;
  ushort *puVar3;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfbc,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0x8e) = 0x0;
  *(undefined2 *)param_1 = 0xb90;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x7,param_7,param_6,unaff_DI);
  *(undefined2 *)(iVar1 + 0x8e) = (int)puVar3;
  *(undefined2 *)(iVar1 + 0x90) = (int)((ulong)puVar3 >> 0x10);
  return param_1;
}



void __stdcall16far pass1_1040_0a1a(ulong param_1)

{
  uint uVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  undefined4 uVar4;
  undefined4 *puVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  uchar *extraout_DX_00;
  uchar *puVar7;
  int iVar8;
  int iVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  ulong uStack10;
  uint uStack6;
  
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar8 = (int)param_1;
  uVar4 = *(undefined4 *)(iVar8 + 0x8e);
  uVar11 = (undefined2)((ulong)uVar4 >> 0x10);
  iVar9 = (int)uVar4;
  puVar2 = (undefined4 *)*(undefined4 *)(iVar9 + 0xa);
  uStack6 = (uint)puVar2;
  puVar5 = (undefined4 *)(*(uint *)(iVar9 + 0xc) | uStack6);
  if (puVar5 == (undefined4 *)0x0) {
    return;
  }
  ppcVar3 = (code **)((int)*puVar2 + 0x14);
  (**ppcVar3)();
  uStack10 = CONCAT22(extraout_DX,puVar5);
  puVar6 = extraout_DX;
  if (*(long *)(iVar8 + 0x70) != 0x0) {
    puVar5 = (undefined4 *)*(uint *)(iVar8 + 0x70);
    uVar1 = *(uint *)(iVar8 + 0x72);
    puVar6 = (uchar *)(uVar1 | (uint)puVar5);
    if (puVar6 != (uchar *)0x0) {
      ppcVar3 = (code **)*puVar5;
      (**ppcVar3)();
      puVar6 = extraout_DX_00;
    }
  }
  mem_op_1000_179c(0x14,puVar6,0x1000);
  puVar7 = (uchar *)((uint)puVar6 | (uint)puVar5);
  if (puVar7 == (uchar *)0x0) {
    puVar5 = (undefined4 *)0x0;
    puVar7 = (uchar *)0x0;
  }
  else {
    struct_1008_4c58((ushort *)CONCAT22(puVar6,puVar5));
  }
  *(undefined2 *)(iVar8 + 0x70) = puVar5;
  *(uchar **)(iVar8 + 0x72) = puVar7;
  pass1_1008_4d84(*(astruct_90 **)(iVar8 + 0x70),uStack10,puVar7);
  return;
}



astruct_57 * __stdcall16far
pass1_1040_0bfc(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_715 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfcd,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_715 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  *(undefined2 *)param_1 = 0xdb0;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,param_8,param_6,param_7);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  iVar1->field_0x74 = 0x1;
  return param_1;
}





// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_0e1c(astruct_57 *param_1,ushort param_2,ulong param_3,ushort param_4,uchar *param_5,int param_6,
               ushort param_7)

{
  astruct_716 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c0,param_4);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_716 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = param_3;
  iVar1->field_0x96 = 0x0;
  iVar1->field_0x98 = param_2;
  *(int *)param_1 = (int)s_overflow_on_node__d_1050_11ca + 0x8;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3a,param_7,param_5,param_6);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return;
}

