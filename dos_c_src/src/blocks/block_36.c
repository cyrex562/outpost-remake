
// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1010_d448(uchar *param_1,ulong param_2,uint *param_3,uchar *param_4,uchar param_5,int param_6)

{
  uint uVar1;
  ushort *puVar2;
  undefined4 uVar3;
  ulong uVar4;
  ushort *puVar5;
  char *pcVar6;
  int iVar7;
  uint uVar8;
  ushort uVar9;
  int iVar10;
  undefined2 uVar11;
  ulong uVar12;
  ushort uVar13;
  ushort local_40c;
  ulong uStack1034;
  ulong uStack1030;
  uchar local_402 [0x400];
  
  uVar11 = (undefined2)((ulong)param_3 >> 0x10);
  iVar10 = (int)param_3;
  uStack1030 = struct_op_1030_73a8(*(ulong *)(iVar10 + 0x6));
  uVar8 = (uint)(uStack1030 >> 0x10);
  uVar1 = (uint)uStack1030;
  if ((uVar8 | uVar1) != 0x0) {
    uStack1034 = *(ulong *)(uVar1 + 0x20);
    uVar1 = *(uint *)(uVar1 + 0x22);
    if ((uVar1 | (uint)uStack1034) != 0x0) {
      local_40c = 0x0;
      puVar5 = &local_40c;
      uVar13 = (ushort)((ulong)param_1 >> 0x10);
      pass1_1010_d984((ushort)param_1,uVar13,(int *)CONCAT22(param_4,puVar5),0x3,
                      uStack1034 & 0xffff | (ulong)uVar1 << 0x10,(ulong)&PTR_DAT_1050_1805_1050_368e,(ulong)param_3,
                      param_4,param_5);
      puVar2 = *(ushort **)(iVar10 + 0x2);
      uVar9 = *(ushort *)(iVar10 + 0x4);
      *(undefined2 *)((int)puVar2 + 0x4) = PTR_DAT_1050_1805_1050_368e;
      uVar3 = *(undefined4 *)(iVar10 + 0x6);
      pcVar6 = pass1_1010_b038(param_1,(ushort)uVar3,(ushort)((ulong)uVar3 >> 0x10),*(uchar **)((int)puVar2 + 0x4),
                               param_6);
      unk_str_op_1000_3d3e((char *)CONCAT22(param_4,local_402),(char *)CONCAT22(uVar9,pcVar6));
      string_1040_a626(puVar2,(char *)CONCAT22(param_4,local_402),uVar9);
      uVar4 = *(ulong *)(iVar10 + 0x2);
      uVar9 = *(ushort *)(iVar10 + 0x4);
      iVar7 = (int)uVar4;
      *(undefined2 *)(iVar7 + 0xe) = PTR_DAT_1050_1822_1050_3690;
      sys_1000_3f9c(local_402,param_4,0x3920,(ushort)&USHORT_1050_1050,local_40c,&stack0xfffe,uVar9,0x1000,param_4,
                    param_5);
      string_1040_a626((ushort *)(uVar4 & 0xffff0000 | (ulong)(iVar7 + 0xa)),(char *)CONCAT22(param_4,local_402),uVar9);
      uVar4 = *(ulong *)(iVar10 + 0x2);
      uVar11 = *(undefined2 *)(iVar10 + 0x4);
      iVar10 = (int)uVar4;
      *(undefined2 *)(iVar10 + 0x18) = PTR_DAT_1050_1823_1050_3692;
      uVar12 = pass1_1028_62c8(uStack1030,(ushort)param_4);
      uVar9 = (ushort)(uVar12 >> 0x10);
      sys_1000_3f9c(local_402,param_4,0x3923,(ushort)&USHORT_1050_1050,(ushort)uVar12,&stack0xfffe,uVar11,0x1000,param_4
                    ,param_5);
      string_1040_a626((ushort *)(uVar4 & 0xffff0000 | (ulong)(iVar10 + 0x14)),(char *)CONCAT22(param_4,local_402),uVar9
                      );
      pass1_1010_dc36((ushort)param_1,uVar13,(uint)puVar5,param_2,param_3,(ushort)param_4);
    }
  }
  return;
}



void __stdcall16far pass1_1010_d5ae(uchar *param_1,ulong param_2,uint *param_3,uchar *param_4,uchar param_5,int param_6)

{
  ushort *puVar1;
  undefined4 uVar2;
  ulong uVar3;
  undefined *puVar4;
  ushort *puVar5;
  char *pcVar6;
  int iVar7;
  ushort uVar8;
  int iVar9;
  undefined2 uVar10;
  ulong uVar11;
  ushort uVar12;
  ushort local_40c;
  uint uStack1034;
  uint uStack1032;
  ulong uStack1030;
  uchar local_402 [0x400];
  
  uVar10 = (undefined2)((ulong)param_3 >> 0x10);
  iVar9 = (int)param_3;
  uStack1030 = struct_op_1030_73a8(*(ulong *)(iVar9 + 0x6));
  uStack1034 = (uint)uStack1030;
  uStack1032 = (uint)(uStack1030 >> 0x10) | uStack1034;
  if (uStack1032 != 0x0) {
    pass1_1028_45fe(uStack1030,uStack1034,(ushort)param_4);
    if ((uStack1032 | uStack1034) != 0x0) {
      local_40c = 0x0;
      puVar5 = &local_40c;
      uVar12 = (ushort)((ulong)param_1 >> 0x10);
      pass1_1010_d984((ushort)param_1,uVar12,(int *)CONCAT22(param_4,puVar5),0x3,CONCAT22(uStack1032,uStack1034),
                      (ulong)&PTR_DAT_1050_1805_1050_3706,(ulong)param_3,param_4,param_5);
      puVar1 = *(ushort **)(iVar9 + 0x2);
      uVar8 = *(ushort *)(iVar9 + 0x4);
      *(undefined2 *)((int)puVar1 + 0x4) = PTR_DAT_1050_1805_1050_3706;
      uVar2 = *(undefined4 *)(iVar9 + 0x6);
      pcVar6 = pass1_1010_b038(param_1,(ushort)uVar2,(ushort)((ulong)uVar2 >> 0x10),*(uchar **)((int)puVar1 + 0x4),
                               param_6);
      unk_str_op_1000_3d3e((char *)CONCAT22(param_4,local_402),(char *)CONCAT22(uVar8,pcVar6));
      string_1040_a626(puVar1,(char *)CONCAT22(param_4,local_402),uVar8);
      uVar3 = *(ulong *)(iVar9 + 0x2);
      uVar8 = *(ushort *)(iVar9 + 0x4);
      iVar7 = (int)uVar3;
      *(undefined2 *)(iVar7 + 0xe) = PTR_DAT_1050_1822_1050_3708;
      sys_1000_3f9c(local_402,param_4,0x3926,(ushort)&USHORT_1050_1050,local_40c,&stack0xfffe,uVar8,0x1000,param_4,
                    param_5);
      string_1040_a626((ushort *)(uVar3 & 0xffff0000 | (ulong)(iVar7 + 0xa)),(char *)CONCAT22(param_4,local_402),uVar8);
      puVar4 = PTR_DAT_1050_1823_1050_370a;
      uVar3 = *(ulong *)(iVar9 + 0x2);
      iVar9 = *(int *)(iVar9 + 0x4);
      iVar7 = (int)uVar3;
      *(undefined2 *)(iVar7 + 0x18) = PTR_DAT_1050_1823_1050_370a;
      uVar11 = pass1_1028_45e2(uStack1030,(uint)puVar4,iVar9,(ushort)param_4);
      uVar8 = (ushort)(uVar11 >> 0x10);
      sys_1000_3f9c(local_402,param_4,0x3929,(ushort)&USHORT_1050_1050,(ushort)uVar11,&stack0xfffe,iVar9,0x1000,param_4,
                    param_5);
      string_1040_a626((ushort *)(uVar3 & 0xffff0000 | (ulong)(iVar7 + 0x14)),(char *)CONCAT22(param_4,local_402),uVar8)
      ;
      pass1_1010_dc36((ushort)param_1,uVar12,(uint)puVar5,param_2,param_3,(ushort)param_4);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1010_d710(ulong param_1,ulong param_2,uint *param_3,uchar *param_4,uchar param_5)

{
  ulong uVar1;
  long lVar2;
  ushort *puVar3;
  char *pcVar4;
  int iVar5;
  ushort uVar6;
  uint in_DX;
  uint uVar7;
  int unaff_SI;
  int iVar8;
  astruct_496 *iVar9;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  undefined4 uVar12;
  ushort uVar13;
  ushort uVar14;
  ushort uStack322;
  int iStack316;
  int iStack314;
  int iStack312;
  uint local_136 [0x4a];
  undefined4 local_a2;
  int iStack14;
  undefined4 uStack12;
  ushort *puStack8;
  int iStack4;
  
  iStack4 = 0x0;
  do {
    uVar9 = (undefined2)(param_2 >> 0x10);
    iVar8 = (int)param_2;
    uVar10 = (undefined2)((ulong)param_3 >> 0x10);
    iVar9 = (astruct_496 *)param_3;
    puVar3 = iVar9->field_0x2;
    *(undefined2 *)(iStack4 * 0xa + (int)puVar3 + 0x4) = *(undefined2 *)(iStack4 * 0x2 + iVar8);
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x4);
  puStack8 = iVar9->field_0x2;
  iStack4 = 0x0;
  do {
    uVar1 = iVar9->field_0x6;
    pcVar4 = pass1_1010_b038((uchar *)param_1,(ushort)uVar1,(ushort)(uVar1 >> 0x10),*(uchar **)((int)puStack8 + 0x4),
                             unaff_SI);
    string_1040_a626(puStack8,(char *)CONCAT22(in_DX,pcVar4),in_DX);
    iStack4 = iStack4 + 0x1;
    puStack8 = (ushort *)((ulong)puStack8 & 0xffff0000 | (ulong)((int)puStack8 + 0xa));
  } while (iStack4 < 0x4);
  uVar13 = (ushort)param_1;
  uVar14 = (ushort)(param_1 >> 0x10);
  struct_1010_dd5e(uVar13,uVar14,iVar9->field_0x6);
  uStack12 = CONCAT22(in_DX,pcVar4);
  in_DX = in_DX | (uint)pcVar4;
  if (in_DX != 0x0) {
    iStack14 = 0x0;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_4,&local_a2),(WNDCLASS16 *)0x0,0x94);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_4,local_136),(WNDCLASS16 *)0x0,0x94);
    iStack314 = 0x0;
    iStack312 = 0x0;
    iStack316 = 0x0;
    uVar1 = iVar9->field_0x6;
    lVar2 = *(long *)((int)uVar1 + 0x26);
    for (uStack322 = 0x1; (int)uStack322 < 0x25; uStack322 = uStack322 + 0x1) {
      if (*(long *)(uStack322 * 0x4 + (int)uStack12) != 0x0) {
        if (iStack14 == 0x0) {
          iStack14 = 0x1;
        }
        pcVar4 = string_1020_c0d8(uStack322);
        uVar7 = in_DX | (uint)pcVar4;
        if (uVar7 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_a2)[iStack312],s_Null_Ptr_1050_392c);
        }
        else {
          uVar6 = str_op_1008_60e8((char *)CONCAT22(in_DX,pcVar4),uVar7);
          *(ushort *)(&local_a2 + iStack312) = uVar6;
          *(uint *)((int)&local_a2 + iStack312 * 0x4 + 0x2) = uVar7;
        }
        uVar11 = (undefined2)((ulong)uStack12 >> 0x10);
        uVar7 = *(uint *)(uStack322 * 0x4 + (int)uStack12);
        in_DX = *(uint *)(uStack322 * 0x4 + (int)uStack12 + 0x2);
        local_136[iStack312 * 0x2] = uVar7;
        local_136[iStack312 * 0x2 + 0x1] = in_DX;
        iStack312 = iStack312 + 0x1;
        if (lVar2 == 0x0) {
          iVar5 = 0x0;
        }
        else {
          uVar12 = pass1_1020_bae6((ushort)lVar2,CONCAT22(uStack322,(int)((ulong)lVar2 >> 0x10)),uVar7,in_DX,
                                   (ushort)param_4);
          in_DX = (uint)((ulong)uVar12 >> 0x10);
          iVar5 = (int)uVar12;
        }
        if (iVar5 == 0x0) {
          iStack316 = iStack316 + 0x2;
        }
        else {
          *(undefined2 *)(uVar13 + iStack314 * 0x2 + 0xa4) = *(undefined2 *)(iVar8 + iStack316 * 0x2 + 0x8);
          *(undefined2 *)(uVar13 + (iStack314 + 0x1) * 0x2 + 0xa4) =
               *(undefined2 *)(iVar8 + (iStack316 + 0x1) * 0x2 + 0x8);
          iStack316 = iStack316 + 0x2;
          iStack314 = iStack314 + 0x2;
        }
      }
    }
    uVar7 = pass1_1010_db2e(uVar13,uVar14,0x4,CONCAT22(param_4,&local_a2),CONCAT22(param_4,local_136),param_2,
                            (int *)param_3,param_4,param_5);
    if (iStack14 != 0x0) {
      iVar9->field_0x16 = 0x1;
    }
    while (iStack312 != 0x0) {
      fn_ptr_1000_17ce((astruct_18 *)(&local_a2)[iStack312 + -0x1],0x1000);
      iStack312 = iStack312 + -0x1;
    }
    pass1_1010_dc36(uVar13,uVar14,uVar7,param_2,param_3,(ushort)param_4);
  }
  return;
}



void __stdcall16far
pass1_1010_d984(ushort param_1,ushort param_2,int *param_3,int param_4,ulong param_5,ulong param_6,ulong param_7,
               uchar *param_8,uchar param_9)

{
  undefined *puVar1;
  char *pcVar2;
  int iVar3;
  uint extraout_DX;
  uint uVar4;
  ushort uVar5;
  int iVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  ushort *puStack1052;
  uchar local_418 [0x400];
  ushort uStack24;
  char *pcStack22;
  ushort uStack18;
  undefined4 uStack16;
  undefined local_c [0x8];
  int iStack4;
  
  iStack4 = param_4;
  pass1_1008_5784((ulong *)CONCAT22(param_8,local_c),param_5);
  do {
    puVar1 = local_c;
    pass1_1008_5b12(puVar1,param_8);
    uStack16 = CONCAT22(extraout_DX,puVar1);
    uVar4 = extraout_DX | (uint)puVar1;
    if (uVar4 == 0x0) {
      return;
    }
    uStack18 = *(ushort *)(puVar1 + 0xa);
    pcStack22 = (char *)0x0;
    if (*(int *)(puVar1 + 0x4) == 0x0) {
      if (*(int *)(puVar1 + 0x6) == 0x0) {
        if (*(int *)(puVar1 + 0x8) == 0x0) {
          return;
        }
        pcVar2 = string_op_1020_c2f8(*(ushort *)(puVar1 + 0x8));
      }
      else {
        pcVar2 = string_op_1020_c222(*(ushort *)(puVar1 + 0x6));
      }
    }
    else {
      pcVar2 = string_1020_c0d8(*(ushort *)(puVar1 + 0x4));
    }
    pcStack22 = (char *)CONCAT22(uVar4,pcVar2);
    uStack24 = *(ushort *)((int)uStack16 + 0xc);
    *param_3 = *param_3 + uStack24;
    uVar8 = (undefined2)(param_7 >> 0x10);
    iVar6 = (int)param_7;
    uVar5 = *(ushort *)(iVar6 + 0x4);
    iVar3 = *(int *)(iVar6 + 0x2) + iStack4 * 0xa;
    puStack1052 = (ushort *)CONCAT22(uVar5,iVar3);
    uVar9 = (undefined2)(param_6 >> 0x10);
    iVar7 = (int)param_6;
    *(undefined2 *)(iVar3 + 0x4) = *(undefined2 *)(iStack4 * 0x2 + iVar7);
    sys_1000_3f9c(local_418,param_8,0x3935,(ushort)&USHORT_1050_1050,uStack18,&stack0xfffe,uVar5,0x1000,param_8,param_9)
    ;
    string_1040_a626(puStack1052,(char *)CONCAT22(param_8,local_418),uVar5);
    uVar5 = *(ushort *)(iVar6 + 0x4);
    iStack4 = iStack4 + 0x1;
    iVar3 = *(int *)(iVar6 + 0x2) + iStack4 * 0xa;
    puStack1052 = (ushort *)CONCAT22(uVar5,iVar3);
    *(undefined2 *)(iVar3 + 0x4) = *(undefined2 *)(iStack4 * 0x2 + iVar7);
    unk_str_op_1000_3d3e((char *)CONCAT22(param_8,local_418),pcStack22);
    string_1040_a626(puStack1052,(char *)CONCAT22(param_8,local_418),uVar5);
    iVar3 = (iStack4 + 0x1) * 0xa + *(int *)(iVar6 + 0x2);
    uVar5 = *(ushort *)(iVar6 + 0x4);
    puStack1052 = (ushort *)CONCAT22(uVar5,iVar3);
    *(undefined2 *)(iVar3 + 0x4) = *(undefined2 *)((iStack4 + 0x1) * 0x2 + iVar7);
    iStack4 = iStack4 + 0x2;
    sys_1000_3f9c(local_418,param_8,0x3938,(ushort)&USHORT_1050_1050,uStack24,&stack0xfffe,uVar5,0x1000,param_8,param_9)
    ;
    string_1040_a626(puStack1052,(char *)CONCAT22(param_8,local_418),uVar5);
  } while( true );
}



uint __stdcall16far
pass1_1010_db2e(ushort param_1,ushort param_2,uint param_3,ulong param_4,ulong param_5,ulong param_6,int *param_7,
               uchar *param_8,uchar param_9)

{
  ushort uVar1;
  astruct_493 *iVar2;
  int iVar3;
  ushort uVar4;
  astruct_492 *iVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  uint uStack94;
  int iStack92;
  uint uStack90;
  ushort *puStack88;
  uchar local_54 [0x52];
  
  uStack94 = param_3;
  uStack90 = param_3;
  iStack92 = 0x0;
  while( true ) {
    uVar7 = (undefined2)((ulong)param_7 >> 0x10);
    iVar4 = (astruct_492 *)param_7;
    if (*param_7 - 0x1U < uStack94) {
      return uStack94;
    }
    uVar1 = iVar4->field_0x4;
    iVar2 = (astruct_493 *)(iVar4->field_0x2 + uStack94 * 0xa);
    uVar5 = (undefined2)(param_5 >> 0x10);
    uVar6 = (undefined2)(param_4 >> 0x10);
    if ((*(long *)(iStack92 * 0x4 + (int)param_5) == 0x0) && (*(long *)(iStack92 * 0x4 + (int)param_4) == 0x0)) break;
    uVar4 = uVar1;
    unk_str_op_1000_3d3e((char *)CONCAT22(param_8,local_54),*(char **)(iStack92 * 0x4 + (int)param_4));
    uVar6 = (undefined2)(param_6 >> 0x10);
    iVar2->field_0x4 = *(undefined2 *)(uStack90 * 0x2 + (int)param_6);
    string_1040_a626((ushort *)CONCAT22(uVar1,iVar2),(char *)CONCAT22(param_8,local_54),uVar4);
    sys_1000_3f9c(local_54,param_8,0x393b,(ushort)&USHORT_1050_1050,
                  (ushort)*(undefined4 *)((int)param_5 + iStack92 * 0x4),&stack0xfffe,uVar5,0x1000,param_8,param_9);
    uVar1 = iVar4->field_0x4;
    iVar3 = iVar4->field_0x2 + (uStack94 + 0x1) * 0xa;
    puStack88 = (ushort *)CONCAT22(uVar1,iVar3);
    *(undefined2 *)(iVar3 + 0x4) = *(undefined2 *)((uStack90 + 0x1) * 0x2 + (int)param_6);
    string_1040_a626(puStack88,(char *)CONCAT22(param_8,local_54),uVar1);
    uStack94 = uStack94 + 0x2;
    uStack90 = uStack90 + 0x2;
    iStack92 = iStack92 + 0x1;
  }
  return uStack94;
}



void __stdcall16far
pass1_1010_dc36(ushort param_1,ushort param_2,uint param_3,ulong param_4,uint *param_5,ushort param_6)

{
  undefined4 *puVar1;
  ushort uVar2;
  ulong uVar3;
  int iVar4;
  uint uVar5;
  undefined4 *puVar6;
  undefined2 uVar7;
  uint uStack90;
  undefined *local_54;
  undefined4 local_52 [0x14];
  
  local_54 = PTR_s_New_failed_in_Op_Op_1050_0020_1050_393f;
  puVar6 = local_52;
  for (iVar4 = 0x13; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    *puVar1 = 0x0;
  }
  *(undefined2 *)puVar6 = 0x0;
  *(undefined *)(undefined2 *)((int)puVar6 + 0x2) = 0x0;
  uStack90 = param_3;
  while( true ) {
    uVar7 = (undefined2)((ulong)param_5 >> 0x10);
    if (*param_5 < uStack90 || *param_5 == uStack90) break;
    uVar3 = *(ulong *)((int)param_5 + 0x2);
    uVar2 = *(ushort *)((int)param_5 + 0x4);
    uVar5 = (int)uVar3 + uStack90 * 0xa;
    *(undefined2 *)(uVar5 + 0x4) = *(undefined2 *)(uStack90 * 0x2 + (int)param_4);
    uStack90 = uStack90 + 0x1;
    string_1040_a626((ushort *)(uVar3 & 0xffff0000 | (ulong)uVar5),(char *)CONCAT22(param_6,&local_54),uVar2);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far
string_1010_dcac(ushort param_1,ushort param_2,ushort param_3,int param_4,ulong param_5,astruct_104 *param_6)

{
  ulong uVar1;
  astruct_105 *iVar2;
  ushort uVar2;
  uint uVar3;
  astruct_104 *iVar5;
  ushort uVar6;
  ushort uVar7;
  char *pcVar4;
  
  pcVar4 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),param_1);
  uVar6 = (ushort)((ulong)param_6 >> 0x10);
  iVar5 = (astruct_104 *)param_6;
  uVar2 = *(ushort *)((int)&iVar5->field_0x2 + 0x2);
  iVar2 = (astruct_105 *)(*(int *)&iVar5->field_0x2 + param_4 * 0xa);
  uVar7 = (ushort)(param_5 >> 0x10);
  iVar2->field_0x4 = *(undefined2 *)(param_4 * 0x2 + (int)param_5);
  string_1040_a626((ushort *)CONCAT22(uVar2,iVar2),pcVar4,uVar2);
  unk_str_op_1000_3d3e(pcVar4,(char *)0x10503941);
  uVar2 = param_4 + 0x1;
  uVar1 = iVar5->field_0x2;
  uVar3 = (int)uVar1 + uVar2 * 0xa;
  *(undefined2 *)(uVar3 + 0x4) = *(undefined2 *)(uVar2 * 0x2 + (int)param_5);
  string_1040_a626((ushort *)(uVar1 & 0xffff0000 | (ulong)uVar3),pcVar4,uVar2);
  return uVar2;
}



void __stdcall16far struct_1010_dd5e(ushort param_1,ushort param_2,ulong param_3)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  ulong uVar4;
  long *plStack16;
  
  if (param_3 != 0x0) {
    uVar4 = struct_op_1030_73a8(param_3);
    uVar3 = (uint)(uVar4 >> 0x10);
    iVar2 = (int)uVar4;
    plStack16 = (long *)(uVar4 & 0xffff0000 | (ulong)(iVar2 + 0x14U));
    if ((uVar3 | iVar2 + 0x14U) != 0x0) {
      iVar1 = *(int *)(iVar2 + 0x12);
      iVar2 = *(int *)(iVar2 + 0x18);
      if (((((iVar1 == 0x4) ||
            ((((iVar1 == 0x6 && (iVar2 == 0x4)) || (iVar1 == 0x5)) || ((iVar1 == 0x6 && (iVar2 == 0x5)))))) ||
           (iVar1 == 0x8)) || ((iVar1 == 0x6 && (iVar2 == 0x8)))) && (*plStack16 != 0x0)) {
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far load_str_1010_ddf6(ulong param_1,ulong param_2)

{
  short in_buf_len_5;
  ulong uVar1;
  
  in_buf_len_5 = (short)(param_1 >> 0x10);
  *(undefined *)((int)param_1 + 0x13c) = 0x0;
  uVar1 = struct_op_1030_73a8(param_2);
  switch(*(undefined2 *)((int)uVar1 + 0x12)) {
  case 0x1:
  case 0x2:
  case 0x4:
  case 0x7:
  case 0x9:
    break;
  case 0x3:
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x8:
    break;
  default:
    goto switchD_1010_de53_caseD_9;
  }
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,
             (char *)((int)param_1 + 0x13c),in_buf_len_5);
switchD_1010_de53_caseD_9:
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_de78(ulong param_1,ulong param_2)

{
  short in_buf_len_5;
  
  in_buf_len_5 = (short)(param_1 >> 0x10);
  *(undefined *)((int)param_1 + 0x13c) = 0x0;
  pass1_1030_809c(param_2);
  load_string_1010_84e0
            (0x1030,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,
             (char *)((int)param_1 + 0x13c),in_buf_len_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1010_debe(ulong param_1,ushort param_2,ushort *param_3,ulong *param_4,ulong param_5,ushort param_6)

{
  byte bVar1;
  ushort uVar2;
  uint uVar3;
  int iVar4;
  ushort uVar5;
  uchar *puVar6;
  int iVar7;
  int unaff_DI;
  undefined2 uVar8;
  ulong uVar9;
  ushort *puVar10;
  ushort uVar11;
  int iStack34;
  ushort uStack30;
  int iStack26;
  int iStack24;
  int iStack22;
  int iStack20;
  
  *param_4 = 0x0;
  *param_3 = 0x0;
  uVar9 = struct_op_1030_73a8(param_5);
  puVar6 = (uchar *)(uVar9 >> 0x10);
  iVar4 = *(int *)((int)uVar9 + 0x12);
  uVar5 = (ushort)param_1;
  uVar11 = (ushort)(param_1 >> 0x10);
  uVar2 = pass1_1010_b028(uVar5,uVar11,uVar9);
  puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,param_6,puVar6,unaff_DI);
  puVar6 = (uchar *)((ulong)puVar10 >> 0x10);
  iVar7 = (int)param_4;
  uVar8 = (undefined2)((ulong)param_4 >> 0x10);
  if (param_2 == 0x13) {
    iStack34 = 0x0;
    while (iStack34 = iStack34 + 0x1, iStack34 < 0x43) {
      param_2 = pass1_1010_ac62(uVar5,uVar11,iStack34,param_2,(ushort)puVar6);
      if (param_2 != 0x0) {
        *param_3 = *param_3 + 0x1;
      }
    }
    uVar2 = *param_3 * 0x2;
    mem_op_1000_179c(uVar2,puVar6,0x1000);
    *(ushort *)param_4 = uVar2;
    *(uchar **)(iVar7 + 0x2) = puVar6;
    if (((uint)puVar6 | *(uint *)param_4) != 0x0) {
      iStack34 = 0x0;
      for (uStack30 = 0x0; uVar2 = uStack30, *param_3 != uStack30 && (int)uStack30 <= (int)*param_3;
          uStack30 = uStack30 + 0x1) {
        do {
          iStack34 = iStack34 + 0x1;
          if (0x42 < iStack34) goto LAB_1010_e0d4;
          uVar2 = pass1_1010_ac62(uVar5,uVar11,iStack34,uVar2,(ushort)puVar6);
        } while (uVar2 == 0x0);
        *(int *)(uStack30 * 0x2 + (int)*param_4) = iStack34;
LAB_1010_e0d4:
      }
    }
  }
  else {
    if (param_2 < 0x14) {
      if ((char)param_2 == '\x06') {
        if (((iVar4 == 0x5) || (iVar4 == 0x6)) || (iVar4 == 0x8)) {
          uVar3 = (int)puVar10 + 0x11e;
          if (uVar2 == 0xf) {
            iStack20 = 0xf;
            iStack22 = 0x13;
          }
          else {
            if (uVar2 == 0xe) {
              iStack22 = 0x4;
              iStack20 = 0x1;
            }
            else {
              iStack22 = 0xe;
              iStack20 = 0x1;
            }
          }
          iVar4 = pass1_1010_e128(uVar5,uVar11,iStack22,iStack20,(ulong)puVar10 & 0xffff0000 | (ulong)uVar3);
          *param_3 = iVar4 + 0x1U;
          if (iVar4 + 0x1U != 0x0) {
            uVar2 = *param_3 * 0x2;
            mem_op_1000_179c(uVar2,puVar6,0x1000);
            *(ushort *)param_4 = uVar2;
            *(uchar **)(iVar7 + 0x2) = puVar6;
            iStack24 = 0x0;
            for (iStack26 = iStack20; iStack26 <= iStack22; iStack26 = iStack26 + 0x1) {
              if (*(int *)(iStack26 * 0x2 + uVar3) != 0x0) {
                *(int *)((int)*param_4 + iStack24 * 0x2) = iStack26;
                iStack24 = iStack24 + 0x1;
              }
            }
            *(undefined2 *)((int)*param_4 + iStack24 * 0x2) = 0x14;
            return;
          }
        }
      }
      else {
        bVar1 = (char)param_2 - 0x7;
        if ((bVar1 == 0x0) && (((iVar4 == 0x5 || (iVar4 == 0x6)) || (iVar4 == 0x8)))) {
          uVar5 = pass1_1010_ac62(uVar5,uVar11,0x7,param_2 & 0xff00 | (uint)bVar1,(ushort)puVar6);
          uVar2 = -(uint)(uVar5 == 0x0) + 0x10;
          *param_3 = uVar2;
          uVar2 = uVar2 * 0x2;
          mem_op_1000_179c(uVar2,puVar6,0x1000);
          *(ushort *)param_4 = uVar2;
          *(uchar **)(iVar7 + 0x2) = puVar6;
          if (((uint)puVar6 | *(uint *)param_4) == 0x0) {
            *param_3 = 0x0;
            return;
          }
          for (iStack26 = 0x0; iStack26 < (int)(-(uint)(uVar5 == 0x0) + 0xf); iStack26 = iStack26 + 0x1) {
            *(int *)(iStack26 * 0x2 + (int)*param_4) = iStack26 + 0x1;
          }
          *(undefined2 *)(iStack26 * 0x2 + (int)*param_4) = 0x10;
          return;
        }
      }
    }
  }
  return;
}



int __stdcall16far pass1_1010_e128(ushort param_1,ushort param_2,int param_3,int param_4,ulong param_5)

{
  int iStack6;
  int iStack4;
  
  iStack4 = 0x0;
  for (iStack6 = param_4; iStack6 <= param_3; iStack6 = iStack6 + 0x1) {
    if (*(int *)(iStack6 * 0x2 + (int)param_5) != 0x0) {
      iStack4 = iStack4 + 0x1;
    }
  }
  return iStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_e15e(ulong param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  code **ppcVar1;
  ushort uVar2;
  uint uVar3;
  ulong uVar4;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  uint uVar5;
  ulong uStack18;
  ulong uStack14;
  undefined4 *puStack10;
  
  pass1_1010_afde(param_1,0xc);
  puStack10 = (undefined4 *)CONCAT22(param_3,param_2);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  uVar2 = param_2;
  (**ppcVar1)(param_4,param_2,param_3);
  uStack14 = CONCAT22(extraout_DX,uVar2);
  for (uStack18 = 0x0; uStack18 < uStack14; uStack18 = uStack18 + 0x1) {
    ppcVar1 = (code **)((int)*puStack10 + 0x4);
    uVar4 = uStack14;
    (**ppcVar1)(param_4,param_2,param_3,(char)uStack18,(int)(uStack18 >> 0x10));
    uVar3 = (uint)uVar4;
    uVar5 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_00);
    param_4 = 0x1030;
    pass1_1030_7c28(CONCAT13((char)(uVar5 >> 0x8),CONCAT12((char)uVar5,uVar3)),0x23,uVar3,uVar5,param_5);
  }
  if (puStack10 != (undefined4 *)0x0) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(param_4,param_2,(char)param_3,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_e1f4(ulong param_1,ulong param_2,ushort param_3,ushort param_4)

{
  undefined2 uVar1;
  BOOL16 BVar2;
  char *pcVar3;
  ushort uVar4;
  undefined2 uVar5;
  int iVar6;
  short in_buf_len_5;
  ulong uVar7;
  
  in_buf_len_5 = (short)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  *(undefined *)(iVar6 + 0x13c) = 0x0;
  uVar7 = struct_op_1030_73a8(param_2);
  uVar5 = (undefined2)(uVar7 >> 0x10);
  uVar1 = *(undefined2 *)((int)uVar7 + 0xc);
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0xc);
  if ((((((((BVar2 == 0x0) && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x14), BVar2 == 0x0)) &&
          (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0xa), BVar2 == 0x0)) &&
         ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x15), BVar2 == 0x0 &&
          (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0xb), BVar2 == 0x0)))) &&
        (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x16), BVar2 == 0x0)) &&
       (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x17), BVar2 == 0x0 &&
         (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x21), BVar2 == 0x0)) &&
        ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1c), BVar2 == 0x0 &&
         (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1d), BVar2 == 0x0 &&
           (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x18), BVar2 == 0x0)) &&
          (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x19), BVar2 == 0x0)))))))) &&
      ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x4), BVar2 == 0x0 &&
       (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x3), BVar2 == 0x0)))) &&
     (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1e), BVar2 == 0x0 &&
       (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x23), BVar2 == 0x0 &&
         (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1b), BVar2 == 0x0)) &&
        ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1f), BVar2 == 0x0 &&
         (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1), BVar2 == 0x0 &&
           (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x2), BVar2 == 0x0)) &&
          (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x13), BVar2 == 0x0)))))))) &&
      (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x20), BVar2 == 0x0 &&
        (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0xe), BVar2 == 0x0)) &&
       (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x10), BVar2 == 0x0)))))) {
    pcVar3 = (char *)pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x12);
    if ((pcVar3 == (char *)0x0) &&
       (pcVar3 = (char *)pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x11), pcVar3 == (char *)0x0)) {
      BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x6);
      if (BVar2 == 0x0) {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x5);
        if (BVar2 == 0x0) {
          pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x40);
          goto LAB_1010_e241;
        }
        uVar4 = pass1_1030_7f98(param_2);
        pcVar3 = string_op_1020_c222(uVar4);
      }
      else {
        uVar4 = pass1_1030_7f5a(param_2);
        pcVar3 = string_op_1020_c2f8(uVar4);
      }
    }
    else {
      pass1_1010_e58a(param_1,uVar7,uVar5,param_3,param_4);
    }
    unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x13c)),(char *)CONCAT22(uVar5,pcVar3));
  }
  else {
LAB_1010_e241:
    load_string_1010_84e0
              (0x1008,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,
               (char *)(iVar6 + 0x13c),in_buf_len_5);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_e58a(ulong param_1,ulong param_2,uchar *param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  uint uVar2;
  BOOL16 BVar3;
  undefined4 *puVar4;
  uint extraout_DX;
  uint uVar5;
  undefined2 extraout_DX_00;
  uint extraout_DX_01;
  int iVar6;
  short in_buf_len_5;
  undefined2 uVar7;
  undefined4 *puVar8;
  
  in_buf_len_5 = (short)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  *(undefined *)(iVar6 + 0x13c) = 0x0;
  uVar7 = (undefined2)(param_2 >> 0x10);
  puVar4 = (undefined4 *)*(int *)((int)param_2 + 0x20);
  uVar7 = *(undefined2 *)((int)param_2 + 0xc);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar7,0x11);
  if (BVar3 == 0x0) {
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar7,0x12);
    if (BVar3 == 0x0) {
      return;
    }
    puVar8 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x31,param_5,param_3,param_4);
    ppcVar1 = (code **)((int)*puVar8 + 0x14);
    (**ppcVar1)(0x1008,(int)puVar8,(int)((ulong)puVar8 >> 0x10),puVar4,(int)puVar4 >> 0xf);
    uVar5 = extraout_DX_01 | (uint)puVar4;
    uVar2 = extraout_DX_01;
  }
  else {
    puVar8 = (undefined4 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x41,param_5,param_3,param_4);
    ppcVar1 = (code **)((int)*puVar8 + 0x14);
    (**ppcVar1)(0x1008,(int)puVar8,(int)((ulong)puVar8 >> 0x10),puVar4,(int)puVar4 >> 0xf);
    uVar5 = extraout_DX | (uint)puVar4;
    uVar2 = extraout_DX;
  }
  if (uVar5 == 0x0) {
    load_string_1010_84e0
              (0x1008,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,
               (char *)(iVar6 + 0x13c),in_buf_len_5);
  }
  else {
    ppcVar1 = (code **)((int)*puVar4 + 0x14);
    (**ppcVar1)(0x1008,(char)puVar4,uVar2);
    unk_str_op_1000_3d3e
              ((char *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x13c)),(char *)CONCAT22(extraout_DX_00,puVar4));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_e682(ulong param_1,ulong param_2,ushort param_3,uchar param_4)

{
  uint uVar1;
  BOOL16 BVar2;
  ulong uVar3;
  uint uVar4;
  ushort uVar5;
  uchar *in_buf_len_5;
  uint uVar6;
  undefined4 uVar7;
  ushort uVar8;
  ushort uVar9;
  uint local_1e;
  uint uStack28;
  ushort local_1a;
  undefined2 uStack24;
  ushort uStack22;
  undefined4 uStack20;
  ulong uStack16;
  ulong uStack12;
  undefined2 uStack8;
  ulong uStack6;
  
  in_buf_len_5 = (uchar *)(param_1 >> 0x10);
  uVar5 = (ushort)param_1;
  *(undefined *)(uVar5 + 0x13c) = 0x0;
  uStack6 = struct_op_1030_73a8(param_2);
  uVar6 = (uint)(uStack6 >> 0x10);
  uStack8 = *(undefined2 *)((int)uStack6 + 0xc);
  uVar4 = uVar6;
  uVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x1);
  if (((uVar1 == 0x0) && (uVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x13), uVar1 == 0x0)) &&
     (uVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x2), uVar1 == 0x0)) {
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0xe);
    if (BVar2 != 0x0) {
      uVar7 = *(undefined4 *)(uVar5 + 0x138);
      uVar3 = *(ulong *)((int)uVar7 + 0x24);
      uStack16 = uVar3;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar3,(uint)(uVar3 >> 0x10));
      uStack12 = uVar3 & 0xffff | (ulong)uVar4 << 0x10;
      uStack20 = *(undefined4 *)((int)uVar3 + 0x1f6);
      uVar6 = (uint)((ulong)uStack20 >> 0x10);
      uVar8 = *(ushort *)((int)uStack20 + 0x1a8);
      uVar9 = 0x3947;
      uStack22 = uVar8;
LAB_1010_e76d:
      sys_1000_3f9c((uchar *)(uVar5 + 0x13c),in_buf_len_5,uVar9,(ushort)&USHORT_1050_1050,uVar8,&stack0xfffe,uVar6,
                    0x1000,param_3,param_4);
      return;
    }
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x5);
    if ((BVar2 == 0x0) && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x6), BVar2 == 0x0)) {
      BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x10);
      if (BVar2 == 0x0) {
        local_1e = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0xc);
        if ((local_1e == 0x0) && (local_1e = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x14), local_1e == 0x0)) {
          BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0xa);
          if (BVar2 == 0x0) {
            uVar8 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x1e);
            if (uVar8 == 0x0) {
              load_string_1010_84e0
                        (0x1008,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,
                         (char *)(uVar5 + 0x13c),(short)in_buf_len_5);
              return;
            }
            pass1_1030_6ddc(param_2);
            uVar9 = 0x395c;
            local_1e = uVar8;
            goto LAB_1010_e76d;
          }
          uVar7 = pass1_1030_7c28(param_2,0x21,BVar2,uVar4,param_3);
          uStack28 = (uint)((ulong)uVar7 >> 0x10);
          uVar1 = (uint)uVar7;
          uVar8 = 0x3958;
          local_1e = uVar1;
        }
        else {
          pass1_1010_e8f6(uVar5,(ushort)in_buf_len_5,param_2,param_3);
          uStack28 = uVar4;
          uVar7 = pass1_1030_7c28(CONCAT22(uVar4,local_1e),0x23,local_1e,uVar4,param_3);
          uStack24 = (undefined2)((ulong)uVar7 >> 0x10);
          uVar1 = (uint)uVar7;
          uVar8 = 0x3954;
          local_1a = uVar1;
        }
      }
      else {
        uVar7 = pass1_1030_7c28(param_2,0x1e,BVar2,uVar4,param_3);
        uStack28 = (uint)((ulong)uVar7 >> 0x10);
        uVar1 = (uint)uVar7;
        uVar8 = 0x3950;
        local_1e = uVar1;
      }
    }
    else {
      local_1e = 0x0;
      local_1a = 0x0;
      pass1_1010_e8d0(uVar5,(ushort)in_buf_len_5,(ushort *)CONCAT22(param_3,&local_1a),
                      (ushort *)CONCAT22(param_3,&local_1e),param_2,(ushort)&local_1e);
      uVar8 = 0x394a;
      uVar1 = local_1e;
    }
  }
  else {
    pass1_1010_e8f6(uVar5,(ushort)in_buf_len_5,param_2,param_3);
    uStack12 = CONCAT22(uVar4,uVar1);
    pass1_1030_70f4(CONCAT22(uVar4,uVar1));
    uStack16 = CONCAT22(uVar4,uVar1);
    uVar8 = 0x3943;
  }
  sys_1000_3f9c((uchar *)(uVar5 + 0x13c),in_buf_len_5,uVar8,(ushort)&USHORT_1050_1050,uVar1,&stack0xfffe,uVar6,0x1000,
                param_3,param_4);
  return;
}



void __stdcall16far
pass1_1010_e8d0(ushort param_1,ushort param_2,ushort *param_3,ushort *param_4,ulong param_5,ushort param_6)

{
  pass1_1030_7064(param_5);
  *param_4 = param_6;
  pass1_1030_70ac(param_5);
  *param_3 = param_6;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_e8f6(ushort param_1,ushort param_2,ulong param_3,ushort param_4)

{
  undefined2 uVar1;
  BOOL16 BVar2;
  undefined2 uVar3;
  ulong uVar4;
  
  uVar4 = struct_op_1030_73a8(param_3);
  uVar1 = *(undefined2 *)((int)uVar4 + 0xc);
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x13);
  if (BVar2 == 0x0) {
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x14);
    if (BVar2 == 0x0) {
      return;
    }
    uVar4 = pass1_1028_4faa(uVar4,param_4);
    uVar3 = (undefined2)(uVar4 >> 0x10);
    uVar1 = (undefined2)uVar4;
  }
  else {
    uVar4 = pass1_1028_121e(uVar4,param_4);
    uVar3 = (undefined2)(uVar4 >> 0x10);
    uVar1 = (undefined2)uVar4;
  }
  pass1_1028_b58e(CONCAT22(uVar3,uVar1));
  return;
}

