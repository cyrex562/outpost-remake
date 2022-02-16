
void __stdcall16far pass1_1018_0ae8(ulong param_1,ushort param_2)

{
  *(ushort *)((int)param_1 + 0x5e) = param_2;
  return;
}



ushort __stdcall16far pass1_1018_0afa(ulong param_1)

{
  return *(ushort *)((int)param_1 + 0x5e);
}



ulong __stdcall16far pass1_1018_0b08(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x7c);
  uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
  iVar2 = (int)uVar1;
  return CONCAT22(*(undefined2 *)(iVar2 + 0x6),*(undefined2 *)(iVar2 + 0x4));
}



void __stdcall16far pass1_1018_0b1e(ushort *param_1,ushort *param_2,ushort param_3)

{
  int iVar1;
  undefined4 uVar2;
  astruct_74 *iVar3;
  undefined2 uVar3;
  ushort local_8;
  int local_6;
  int local_4;
  
  iVar3 = (astruct_74 *)param_1;
  iVar3 = (astruct_74 *)&iVar3->field_0x30;
  pass1_1008_3eb4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar3)),(ushort *)CONCAT22(param_3,&local_8),
                  (ushort *)CONCAT22(param_3,&local_6),(ushort *)CONCAT22(param_3,&local_4));
  if (local_4 + -0x3 < 0x1) {
    local_4 = 0x3;
  }
  if (local_6 + -0x3 < 0x1) {
    local_6 = 0x3;
  }
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  uVar2 = iVar3->field_0x5a;
  iVar1 = *(int *)((int)uVar2 + 0x4);
  if (iVar1 <= local_4 + 0x2) {
    local_4 = iVar1 + -0x3;
  }
  uVar2 = iVar3->field_0x5a;
  iVar1 = *(int *)((int)uVar2 + 0x8);
  if (iVar1 <= local_6 + 0x2) {
    local_6 = iVar1 + -0x3;
  }
  pass1_1008_6cec((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x40),local_8,
                  CONCAT22(local_4 + 0x2,local_6 + 0x2),local_8,CONCAT22(local_4 + -0x3,local_6 + -0x3));
  pass1_1008_3f62(param_2,(ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x40));
  pass1_1008_3f62((ushort *)((ulong)param_2 & 0xffff0000 | (ulong)((int)param_2 + 0x6)),
                  (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x46));
  return;
}



void __stdcall16far pass1_1018_0bf4(ushort param_1,int param_2,ulong param_3,int param_4)

{
  undefined4 uVar1;
  uint uVar2;
  ushort uVar3;
  long lVar4;
  uint uVar5;
  undefined local_14 [0xc];
  ushort local_8;
  undefined4 local_6;
  
  switch(param_4) {
  case 0x1:
    *(undefined4 *)((int)param_3 + 0xc) = 0x0;
    *(undefined4 *)((int)param_3 + 0x7e) = 0x0;
    return;
  case 0x4:
    pass1_1008_3eb4((ushort *)(param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x10)),(ushort *)CONCAT22(param_1,&local_8)
                    ,(ushort *)CONCAT22(param_1,&local_6),(ushort *)CONCAT22(param_1,(int)&local_6 + 0x2));
    uVar1 = *(undefined4 *)((int)param_3 + 0xc);
    local_8 = *(ushort *)((int)uVar1 + 0x1e);
    pass1_1008_3e76((ushort *)(param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x10)),local_8,(ushort)local_6,
                    (ushort)((ulong)local_6 >> 0x10));
    pass1_1008_6c90((ushort *)CONCAT22(param_1,local_14));
    pass1_1018_0b1e((ushort *)(param_3 & 0xffff0000 | (ulong)((int)param_3 - 0x20)),(ushort *)CONCAT22(param_1,local_14)
                    ,param_1);
    goto LAB_1018_0c71;
  case 0x5:
  case 0x6:
    uVar2 = (int)param_3 - 0x20;
    uVar5 = param_3._2_2_;
    pass1_1018_0dc6(param_3 & 0xffff0000 | (ulong)uVar2,param_1);
    pass1_1018_10c4(param_1,uVar5,param_3 & 0xffff0000 | (ulong)uVar2);
    pass1_1018_1346(param_1,uVar5,(astruct_93 *)(param_3 & 0xffff0000 | (ulong)uVar2));
LAB_1018_0c71:
    *(undefined4 *)((int)param_3 + 0x2c) = 0x0;
    lVar4 = *(long *)((int)param_3 + 0x1c);
    uVar3 = *(ushort *)((int)param_3 + 0x1e);
    uVar1 = *(undefined4 *)((int)param_3 + 0xc);
    if (*(long *)((int)uVar1 + 0x20) == lVar4) {
      pass1_1018_028c(*(ulong *)((int)param_3 + 0xc),*(ulong *)((int)param_3 + 0x1c),(ushort)lVar4,uVar3,param_1);
      *(undefined2 *)((int)param_3 + 0x2c) = (int)lVar4;
      *(ushort *)((int)param_3 + 0x2e) = uVar3;
      pass1_1010_1f62(param_1,param_3 & 0xffff0000 | (ulong)((int)param_3 - 0x20),param_4);
      return;
    }
    break;
  case 0x14:
    uVar1 = *(undefined4 *)((int)param_3 + 0xc);
    if (*(long *)((int)uVar1 + 0x20) != *(long *)((int)param_3 + 0x1c)) {
      post_win_msg_1020_291a(0x1020);
      return;
    }
    break;
  case 0x15:
    uVar3 = pass1_1010_65d0(param_1,*(ulong *)((int)param_3 + 0x7e),0x88);
    if (uVar3 != 0x0) {
      *(undefined2 *)((int)param_3 + 0x88) = 0x1;
      return;
    }
  }
  return;
}



void __stdcall16far pass1_1018_0d76(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x2c);
  if (*(long *)((int)uVar1 + 0x20) == *(long *)((int)param_1 + 0x3c)) {
    return;
  }
  return;
}



void __stdcall16far pass1_1018_0d9a(ulong param_1,ushort *param_2,ulong *param_3)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x7c);
  *param_3 = *(ulong *)((int)uVar1 + 0x16);
  uVar1 = *(undefined4 *)((int)param_1 + 0x7c);
  *param_2 = *(ushort *)((int)uVar1 + 0x1a);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_0dc6(ulong param_1,ushort param_2)

{
  uint *puVar1;
  undefined *puVar2;
  ushort uVar3;
  int iVar4;
  undefined4 *puVar5;
  astruct_18 *paVar6;
  uchar *in_DX;
  uchar *puVar7;
  uchar *puVar8;
  uint uVar9;
  astruct_91 *iVar13;
  undefined2 uVar10;
  undefined4 local_32;
  undefined2 uStack46;
  ulong uStack44;
  astruct_18 *paStack40;
  astruct_18 *paStack36;
  astruct_18 *paStack32;
  undefined4 uStack28;
  undefined4 uStack24;
  undefined local_14 [0x8];
  undefined2 uStack12;
  uint uStack10;
  undefined2 uStack8;
  uint uStack6;
  int iStack4;
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x400);
  uVar10 = (undefined2)(param_1 >> 0x10);
  iVar13 = (astruct_91 *)param_1;
  paStack36 = (astruct_18 *)iVar13->field_0x94;
  fn_ptr_1000_17ce(paStack36,0x1000);
  paStack40 = (astruct_18 *)iVar13->field_0x9a;
  paStack32 = paStack40;
  fn_ptr_1000_17ce(paStack40,0x1000);
  iVar13->field_0x94 = 0x0;
  iVar13->field_0x9a = 0x0;
  iVar13->field_0x92 = 0x0;
  iVar13->field_0x98 = 0x0;
  while( true ) {
    puVar2 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar2));
    uStack24 = CONCAT22(in_DX,puVar2);
    puVar7 = (uchar *)((uint)in_DX | (uint)puVar2);
    if (puVar7 == (uchar *)0x0) break;
    paVar6 = *(astruct_18 **)(puVar2 + 0x200);
    paStack40 = paVar6;
    if (paVar6 == (astruct_18 *)0x8000001) {
      puVar1 = &iVar13->field_0x92;
      *puVar1 = *puVar1 + 0x1;
      in_DX = puVar7;
    }
    else {
      if ((iVar13->field_0xa8 != 0x0) ||
         (pass1_1008_dfa6(iVar13->field_0xa2,*(long *)(puVar2 + 0x4),0x4000001,param_2), in_DX = puVar7,
         (int)paVar6 != 0x0)) {
        in_DX = puVar7;
        puVar1 = &iVar13->field_0x98;
        *puVar1 = *puVar1 + 0x1;
      }
    }
  }
  puVar8 = puVar7;
  if (iVar13->field_0x92 != 0x0) {
    uVar9 = iVar13->field_0x92;
    uStack44 = uStack44 & 0xffff0000 | (ulong)uVar9;
    uVar3 = uVar9 * 0x6;
    mem_op_1000_179c(uVar3,(uchar *)0x0,0x1000);
    paStack32 = (astruct_18 *)CONCAT22(puVar7,uVar3);
    puVar8 = (uchar *)((uint)puVar7 | uVar3);
    if (puVar8 == (uchar *)0x0) {
      iVar13->field_0x94 = 0x0;
    }
    else {
      pass1_1000_5586((uchar *)0x3e38,0x1008,(int)uStack44,0x6,uVar3,(ushort)puVar7);
      iVar13->field_0x94 = (ulong)paStack32;
    }
  }
  if (iVar13->field_0x98 != 0x0) {
    uVar9 = iVar13->field_0x98;
    uStack44 = uStack44 & 0xffff0000 | (ulong)uVar9;
    uVar3 = uVar9 * 0x6;
    mem_op_1000_179c(uVar3,puVar8,0x1000);
    paStack32 = (astruct_18 *)CONCAT22(puVar8,uVar3);
    if (((uint)puVar8 | uVar3) == 0x0) {
      iVar13->field_0x9a = 0x0;
    }
    else {
      pass1_1000_5586((uchar *)0x3e38,0x1008,(int)uStack44,0x6,uVar3,(ushort)puVar8);
      iVar13->field_0x9a = (ulong)paStack32;
    }
  }
  if (iStack4 != 0x0) {
    uStack8 = 0x1;
    uStack6 = 0x0;
  }
  uStack28 = 0x0;
  uStack12 = uStack8;
  uStack10 = uStack6;
LAB_1018_0f74:
  puVar2 = local_14;
  pass1_1028_e4ec(CONCAT22(param_2,puVar2));
  uStack24 = CONCAT22(uStack6,puVar2);
  uVar9 = uStack6 | (uint)puVar2;
  if (uVar9 == 0x0) {
    return;
  }
  uStack44 = *(ulong *)(puVar2 + 0x200);
  paVar6 = *(astruct_18 **)(puVar2 + 0x10);
  paStack40 = paVar6;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)paVar6,(uint)((ulong)paVar6 >> 0x10));
  paStack36 = (astruct_18 *)((ulong)paVar6 & 0xffff | (ulong)uVar9 << 0x10);
  local_32 = *(undefined4 *)((int)paVar6 + 0xc);
  uStack46 = *(undefined2 *)((int)paVar6 + 0x10);
  puVar5 = &local_32;
  if (uStack44 != 0x8000001) goto LAB_1018_0ffc;
  iVar4 = *(int *)&iVar13->field_0x94;
  uStack6 = *(uint *)((int)&iVar13->field_0x94 + 0x2);
  uStack28 = uStack28 & 0xffff | (ulong)(uStack28._2_2_ + 0x1) << 0x10;
  goto LAB_1018_0fe8;
LAB_1018_0ffc:
  if ((iVar13->field_0xa8 != 0x0) ||
     (pass1_1008_dfa6(iVar13->field_0xa2,*(long *)((int)uStack24 + 0x4),0x4000001,param_2), uStack6 = uVar9,
     puVar5 != (undefined4 *)0x0)) {
    iVar4 = *(int *)&iVar13->field_0x9a;
    uStack6 = *(uint *)((int)&iVar13->field_0x9a + 0x2);
    uStack28 = uStack28 & 0xffff0000 | (ulong)((int)uStack28 + 0x1);
    uStack28._2_2_ = (int)uStack28;
LAB_1018_0fe8:
    pass1_1008_3f62((ushort *)CONCAT22(uStack6,iVar4 + uStack28._2_2_ * 0x6),(ushort *)CONCAT22(param_2,&local_32));
  }
  goto LAB_1018_0f74;
}



void __stdcall16far pass1_1018_1054(ulong param_1,ushort *param_2,ulong *param_3,ushort param_4)

{
  int iVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(long *)(iVar1 + 0x94) == 0x0) {
    pass1_1018_0dc6(param_1 & 0xffff | (ulong)uVar2 << 0x10,param_4);
  }
  *param_3 = *(ulong *)(iVar1 + 0x94);
  *param_2 = *(ushort *)(iVar1 + 0x92);
  return;
}



void __stdcall16far pass1_1018_108c(ulong param_1,ushort *param_2,ulong *param_3,ushort param_4)

{
  int iVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(long *)(iVar1 + 0x9a) == 0x0) {
    pass1_1018_0dc6(param_1 & 0xffff | (ulong)uVar2 << 0x10,param_4);
  }
  *param_3 = *(ulong *)(iVar1 + 0x9a);
  *param_2 = *(ushort *)(iVar1 + 0x98);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_10c4(ushort param_1,uint param_2,ulong param_3)

{
  ulong uVar1;
  code **ppcVar2;
  undefined4 uVar3;
  int iVar4;
  undefined *puVar5;
  uint uVar6;
  ushort uVar7;
  ulong uVar8;
  uint uVar9;
  uchar *puVar10;
  uint extraout_DX;
  uint extraout_DX_00;
  uint extraout_DX_01;
  uint extraout_DX_02;
  int iVar11;
  undefined2 uVar12;
  undefined uVar13;
  bool bVar14;
  ulong *puVar15;
  ulong uStack60;
  ulong uStack56;
  undefined4 uStack52;
  undefined4 *puStack48;
  undefined4 *puStack40;
  uint uStack30;
  uint uStack28;
  undefined local_16 [0x8];
  undefined2 uStack14;
  uint uStack12;
  undefined2 uStack10;
  uint uStack8;
  int iStack6;
  int iStack4;
  
  uVar12 = (undefined2)(param_3 >> 0x10);
  iVar11 = (int)param_3;
  iStack4 = *(int *)(iVar11 + 0x86);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar11 + 0x88),0x1000);
  *(undefined2 *)(iVar11 + 0x86) = 0x0;
  *(undefined4 *)(iVar11 + 0x88) = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_1 >> 0x8),CONCAT12((char)param_1,local_16)),0x1,0x0,0x400);
  uStack30 = 0x0;
  uStack28 = 0x0;
  while( true ) {
    uVar9 = param_2;
    puVar5 = local_16;
    pass1_1028_e4ec(CONCAT22(param_1,puVar5));
    param_2 = uVar9 | (uint)puVar5;
    if (param_2 == 0x0) break;
    if (*(long *)(iVar11 + 0x3c) == *(long *)(puVar5 + 0x8)) {
      puVar15 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
      puVar10 = (uchar *)((ulong)puVar15 >> 0x10);
      uVar6 = (uint)puVar15;
      pass1_1038_4e78(uVar6,puVar10,CONCAT22(uVar9,puVar5),puVar15);
      puStack48 = (undefined4 *)CONCAT22(puVar10,uVar6);
      uVar3 = *puStack48;
      ppcVar2 = (code **)uVar3 + 0x8;
      uVar9 = uVar6;
      (**ppcVar2)((int)&PTR_LOOP_1050_1038,uVar6,puVar10);
      bVar14 = CARRY2(uStack30,uVar9);
      uStack30 = uStack30 + uVar9;
      uStack28 = uStack28 + extraout_DX + (uint)bVar14;
      param_2 = extraout_DX;
      if (puStack48 != (undefined4 *)0x0) {
        ppcVar2 = (code **)uVar3;
        (**ppcVar2)(0x38,uVar6,puVar10,0x1);
        param_2 = extraout_DX_00;
      }
    }
  }
  if ((uStack28 | uStack30) != 0x0) {
    *(uint *)(iVar11 + 0x86) = uStack30;
    uVar7 = uStack30 * 0x6;
    mem_op_1000_179c(uVar7,(uchar *)0x0,0x1000);
    uStack52 = CONCAT22(param_2,uVar7);
    if ((param_2 | uVar7) == 0x0) {
      *(undefined4 *)(iVar11 + 0x88) = 0x0;
    }
    else {
      pass1_1000_5586((uchar *)0x3e38,0x1008,uStack30,0x6,uVar7,param_2);
      *(undefined4 *)(iVar11 + 0x88) = uStack52;
    }
    if (iStack6 != 0x0) {
      uStack10 = 0x1;
      uStack8 = 0x0;
    }
    iVar4 = 0x0;
    uStack14 = uStack10;
    uStack12 = uStack8;
    while( true ) {
      uVar9 = uStack8;
      puVar5 = local_16;
      pass1_1028_e4ec(CONCAT22(param_1,puVar5));
      if ((uVar9 | (uint)puVar5) == 0x0) break;
      uStack8 = uVar9 | (uint)puVar5;
      if (*(long *)(iVar11 + 0x3c) == *(long *)(puVar5 + 0x8)) {
        puVar15 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
        puVar10 = (uchar *)((ulong)puVar15 >> 0x10);
        uVar6 = (uint)puVar15;
        uVar13 = 0x38;
        pass1_1038_4e78(uVar6,puVar10,CONCAT13((char)(uVar9 >> 0x8),CONCAT12((char)uVar9,puVar5)),puVar15);
        puStack40 = (undefined4 *)CONCAT22(puVar10,uVar6);
        ppcVar2 = (code **)((int)*puStack40 + 0x10);
        uVar9 = uVar6;
        (**ppcVar2)(0x38,uVar6,puVar10);
        uStack56 = CONCAT22(extraout_DX_01,uVar9);
        uStack8 = extraout_DX_01;
        for (uStack60 = 0x0; uStack60 < uStack56; uStack60 = uStack60 + 0x1) {
          uVar8 = uStack56;
          pass1_1030_1d58((ulong)puStack40);
          uVar1 = *(ulong *)(iVar11 + 0x88);
          uVar13 = 0x8;
          pass1_1008_3f62((ushort *)
                          (uVar1 & 0xff000000 | (ulong)CONCAT12((char)(uVar1 >> 0x10),(int)uVar1 + iVar4 * 0x6)),
                          (ushort *)CONCAT22(uStack8,(int)uVar8 + 0xc));
          iVar4 = iVar4 + 0x1;
        }
        if (puStack40 != (undefined4 *)0x0) {
          ppcVar2 = (code **)*puStack40;
          (**ppcVar2)(uVar13,(char)uVar6,puVar10,0x1);
          uStack8 = extraout_DX_02;
        }
      }
    }
    if (*(int *)(iVar11 + 0x86) != iStack4) {
      pass1_1010_1f62(param_1,param_3,0x6);
    }
  }
  return;
}



void __stdcall16far pass1_1018_1320(ulong param_1,ushort *param_2,ulong *param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *param_3 = *(ulong *)((int)param_1 + 0x88);
  *param_2 = *(ushort *)((int)param_1 + 0x86);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_1346(ushort param_1,uint param_2,astruct_93 *param_3)

{
  code **ppcVar1;
  int iVar2;
  undefined *puVar3;
  uint uVar4;
  ushort uVar5;
  uchar *puVar6;
  uint extraout_DX;
  uint extraout_DX_00;
  uint extraout_DX_01;
  uint uVar7;
  uint extraout_DX_02;
  astruct_93 *iVar9;
  undefined2 uVar8;
  undefined uVar9;
  ulong *puVar10;
  ulong uVar11;
  ulong uVar12;
  ulong uStack70;
  undefined4 *puStack56;
  ulong uStack52;
  undefined4 *puStack48;
  undefined4 uStack30;
  undefined local_16 [0x8];
  undefined2 uStack14;
  uint uStack12;
  undefined2 uStack10;
  uint uStack8;
  int iStack6;
  uint uStack4;
  
  uVar8 = (undefined2)((ulong)param_3 >> 0x10);
  iVar9 = (astruct_93 *)param_3;
  uStack4 = iVar9->field_0x8c;
  fn_ptr_1000_17ce((astruct_18 *)iVar9->field_0x8e,0x1000);
  iVar9->field_0x8c = 0x0;
  iVar9->field_0x8e = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_1 >> 0x8),CONCAT12((char)param_1,local_16)),0x1,0x0,0x400);
  uStack30 = 0x0;
  while( true ) {
    uVar7 = param_2;
    puVar3 = local_16;
    pass1_1028_e4ec(CONCAT22(param_1,puVar3));
    param_2 = uVar7 | (uint)puVar3;
    if (param_2 == 0x0) break;
    if (iVar9->field_0x3c == *(long *)(puVar3 + 0x8)) {
      puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
      puVar6 = (uchar *)((ulong)puVar10 >> 0x10);
      uVar4 = (uint)puVar10;
      uVar9 = 0x38;
      pass1_1038_4e78(uVar4,puVar6,CONCAT22(uVar7,puVar3),puVar10);
      puStack48 = (undefined4 *)CONCAT22(puVar6,uVar4);
      ppcVar1 = (code **)((int)*puStack48 + 0x10);
      uVar7 = uVar4;
      (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar4,puVar6);
      uStack52 = CONCAT22(extraout_DX,uVar7);
      param_2 = extraout_DX;
      for (puStack56 = (undefined4 *)0x0; puStack56 < uStack52; puStack56 = (undefined4 *)((long)puStack56 + 0x1)) {
        uVar9 = 0x30;
        uVar11 = pass1_1030_1d7c(uVar7,param_2,(ulong)puStack48);
        param_2 = (uint)(uVar11 >> 0x10);
        if (*(int *)((int)uVar11 + 0x12) == 0x9) {
          uStack30 = uStack30 + 0x1;
        }
      }
      if (puStack48 != (undefined4 *)0x0) {
        ppcVar1 = (code **)*puStack48;
        (**ppcVar1)(uVar9,uVar4,puVar6,0x1);
        param_2 = extraout_DX_00;
      }
    }
  }
  if ((uStack30._2_2_ | (uint)uStack30) != 0x0) {
    iVar9->field_0x8c = (uint)uStack30;
    uVar5 = (uint)uStack30 * 0x6;
    mem_op_1000_179c(uVar5,(uchar *)0x0,0x1000);
    uStack70 = CONCAT22(param_2,uVar5);
    if ((param_2 | uVar5) == 0x0) {
      iVar9->field_0x8e = 0x0;
    }
    else {
      pass1_1000_5586((uchar *)0x3e38,0x1008,(uint)uStack30,0x6,uVar5,param_2);
      iVar9->field_0x8e = uStack70;
    }
    if (iStack6 != 0x0) {
      uStack10 = 0x1;
      uStack8 = 0x0;
    }
    iVar2 = 0x0;
    uStack14 = uStack10;
    uStack12 = uStack8;
    while( true ) {
      uVar7 = uStack8;
      puVar3 = local_16;
      pass1_1028_e4ec(CONCAT22(param_1,puVar3));
      if ((uVar7 | (uint)puVar3) == 0x0) break;
      uStack8 = uVar7 | (uint)puVar3;
      if (iVar9->field_0x3c == *(long *)(puVar3 + 0x8)) {
        puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
        puVar6 = (uchar *)((ulong)puVar10 >> 0x10);
        uVar4 = (uint)puVar10;
        uVar9 = 0x38;
        pass1_1038_4e78(uVar4,puVar6,CONCAT13((char)(uVar7 >> 0x8),CONCAT12((char)uVar7,puVar3)),puVar10);
        puStack56 = (undefined4 *)CONCAT22(puVar6,uVar4);
        ppcVar1 = (code **)((int)*puStack56 + 0x10);
        uVar7 = uVar4;
        (**ppcVar1)(0x38,uVar4,puVar6);
        uStack52 = CONCAT22(extraout_DX_01,uVar7);
        uStack8 = extraout_DX_01;
        for (puStack48 = (undefined4 *)0x0; puStack48 < uStack52; puStack48 = (undefined4 *)((long)puStack48 + 0x1)) {
          uVar11 = uStack52;
          pass1_1030_1d58((ulong)puStack56);
          uVar9 = 0x30;
          uVar12 = struct_op_1030_73a8(uVar11 & 0xffff | (ulong)uStack8 << 0x10);
          uVar7 = (uint)(uVar12 >> 0x10);
          if (*(int *)((int)uVar12 + 0x12) == 0x9) {
            uVar12 = iVar9->field_0x8e;
            uVar9 = 0x8;
            pass1_1008_3f62((ushort *)
                            (uVar12 & 0xff000000 | (ulong)CONCAT12((char)(uVar12 >> 0x10),(int)uVar12 + iVar2 * 0x6)),
                            (ushort *)CONCAT22(uStack8,(int)uVar11 + 0xc));
            iVar2 = iVar2 + 0x1;
          }
          uStack8 = uVar7;
        }
        if (puStack56 != (undefined4 *)0x0) {
          ppcVar1 = (code **)*puStack56;
          (**ppcVar1)(uVar9,(char)uVar4,puVar6,0x1);
          uStack8 = extraout_DX_02;
        }
      }
    }
    if (iVar9->field_0x8c != uStack4) {
      pass1_1010_1f62(param_1,(ulong)param_3,0x6);
    }
  }
  return;
}



void __stdcall16far pass1_1018_15f6(ulong param_1,ushort *param_2,ulong *param_3)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *param_3 = *(ulong *)((int)param_1 + 0x8e);
  *param_2 = *(ushort *)((int)param_1 + 0x8c);
  return;
}



void __stdcall16far pass1_1018_161c(ushort param_1,ulong param_2,ushort *param_3,int param_4,int param_5)

{
  ushort uVar1;
  ushort uVar2;
  ulong local_6;
  
  pass1_1008_3e94((ushort *)(param_2 & 0xffff0000 | (ulong)((int)param_2 + 0x36)),(ushort *)CONCAT22(param_1,&local_6),
                  (ushort *)CONCAT22(param_1,(int)&local_6 + 0x2));
  uVar1 = local_6._2_2_ + param_5 + -0x3;
  uVar2 = (int)local_6 + param_4 + -0x3;
  local_6 = CONCAT22(uVar1,uVar2);
  pass1_1008_3e76(param_3,*(ushort *)((int)param_2 + 0x44),uVar2,uVar1);
  return;
}



void __stdcall16far pass1_1018_1662(ulong param_1,int param_2,int param_3,ushort param_4)

{
  int local_6;
  int local_4;
  
  pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x36)),(ushort *)CONCAT22(param_4,&local_6),
                  (ushort *)CONCAT22(param_4,&local_4));
  pass1_1018_16b8(param_1,*(ushort *)((int)param_1 + 0x44),CONCAT22(local_4 + param_3,local_6 + param_2),param_4);
  return;
}



void __stdcall16far pass1_1018_169e(ulong param_1,ulong param_2,ushort param_3)

{
  uint uVar1;
  
  uVar1 = (uint)(param_1 >> 0x10);
  pass1_1018_16b8(param_1 & 0xffff | (ulong)uVar1 << 0x10,*(ushort *)((int)param_1 + 0x44),param_2,param_3);
  return;
}



// WARNING: Unable to use type for symbol uVar2

void __stdcall16far pass1_1018_16b8(ulong param_1,ushort param_2,ulong param_3,ushort param_4)

{
  int iVar1;
  undefined4 uVar3;
  long lVar4;
  ushort uVar5;
  int iVar6;
  ushort uVar7;
  ushort uVar8;
  undefined local_6 [0x2];
  undefined local_4 [0x2];
  undefined4 uVar2;
  
  if (param_3._2_2_ + -0x3 < 0x1) {
    param_3 = CONCAT22(0x3,(int)param_3);
  }
  if ((int)param_3 + -0x3 < 0x1) {
    param_3 = CONCAT22(param_3._2_2_,0x3);
  }
  uVar7 = (ushort)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar2 = *(undefined4 *)(iVar6 + 0x5a);
  iVar1 = *(int *)((int)uVar2 + 0x4);
  if (iVar1 <= param_3._2_2_ + 0x2) {
    param_3 = param_3 & 0xffff | (ulong)(iVar1 - 0x3) << 0x10;
  }
  uVar3 = *(undefined4 *)(iVar6 + 0x5a);
  iVar1 = *(int *)((int)uVar3 + 0x8);
  if (iVar1 <= (int)param_3 + 0x2) {
    param_3 = param_3 & 0xffff0000 | (ulong)(iVar1 - 0x3);
  }
  uVar8 = (ushort)(param_3 >> 0x10);
  pass1_1008_3e76((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x30)),param_2,(ushort)param_3,uVar8);
  uVar5 = uVar7;
  pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x36U)),(ushort *)CONCAT22(param_4,local_6),
                  (ushort *)CONCAT22(param_4,local_4));
  pass1_1008_3e76((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x36U)),0x0,(ushort)param_3,uVar8);
  *(undefined4 *)(iVar6 + 0x4c) = 0x0;
  lVar4 = *(long *)(iVar6 + 0x3c);
  uVar3 = *(undefined4 *)(iVar6 + 0x2c);
  if (*(long *)((int)uVar3 + 0x20) == lVar4) {
    pass1_1018_028c(*(ulong *)(iVar6 + 0x2c),*(ulong *)(iVar6 + 0x3c),(ushort)lVar4,uVar5,param_4);
    *(undefined2 *)(iVar6 + 0x4c) = (int)lVar4;
    *(ushort *)(iVar6 + 0x4e) = uVar5;
    pass1_1010_1f62(param_4,param_1,0x4);
  }
  return;
}



void __stdcall16far pass1_1018_179e(ulong param_1,ulong param_2,ushort param_3,ushort param_4)

{
  ushort local_8;
  undefined4 local_6;
  
  pass1_1008_3eb4((ushort *)param_2,(ushort *)CONCAT22(param_4,&local_8),(ushort *)CONCAT22(param_4,&local_6),
                  (ushort *)CONCAT22(param_4,(int)&local_6 + 0x2));
  pass1_1018_16b8(param_1,local_8,local_6,param_4);
  return;
}



void __stdcall16far pass1_1018_17ce(ulong param_1,ulong param_2,ulong param_3,ushort param_4,uchar param_5)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  pass1_1018_0412(*(ulong *)((int)param_1 + 0x2c),(ushort)param_2,CONCAT22((int)param_3,(int)(param_2 >> 0x10)),
                  (ushort)(param_3 >> 0x10),*(ulong *)((int)param_1 + 0x3c),param_4,param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far pass1_1018_17f0(void)

{
  int iStack4;
  
  iStack4 = 0x0;
  while ((iStack4 < 0x4 && (*(int *)(iStack4 * 0x2 + (int)_PTR_LOOP_1050_3962) != 0x0))) {
    iStack4 = iStack4 + 0x1;
  }
  return iStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_181c(ulong param_1,char *param_2,uchar param_3,ushort param_4)

{
  undefined in_AH;
  undefined2 uVar1;
  
  uVar1 = CONCAT11(in_AH,param_3);
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),
                  *(ulong *)((int)param_1 + 0x3c));
  pass1_1030_5b6c(CONCAT22(param_4,uVar1),param_2,param_4);
  return;
}



ushort * __stdcall16far pass1_1018_1842(ushort *param_1,byte param_2,ushort param_3)

{
  param_1 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0x20));
  pass1_1018_078e(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_18b8(ushort param_1,astruct_55 *param_2,ushort param_3)

{
  uchar *puVar1;
  astruct_55 *iVar3;
  int unaff_DI;
  astruct_55 *uVar3;
  ushort *puVar2;
  astruct_43 *paVar3;
  ulong uVar4;
  int *piVar5;
  ushort uVar6;
  int *piVar7;
  ushort uVar8;
  int local_6;
  int local_4;
  ushort uVar1;
  
  get_sys_metrics_1018_4b1e(param_2,0x0,param_3);
  uVar3 = (astruct_55 *)((ulong)param_2 >> 0x10);
  iVar3 = (astruct_55 *)param_2;
  iVar3->field_0x20 = 0x389a;
  iVar3->field_0x22 = 0x1008;
  iVar3->field_0x20 = 0x3aa8;
  iVar3->field_0x22 = 0x1008;
  *(undefined4 *)&iVar3->field_0x24 = 0x0;
  iVar3->field_0x28 = 0x4;
  puVar2 = pass1_1008_3e38((ushort *)((ulong)param_2 & 0xffff0000 | ZEXT24(iVar3 + 0x1)));
  puVar1 = (uchar *)((ulong)puVar2 >> 0x10);
  *(undefined4 *)&iVar3[0x1].field_0x6 = 0x0;
  iVar3[0x1].field_0xa = 0x0;
  *(undefined4 *)&iVar3[0x1].field_0xc = 0x0;
  iVar3[0x1].field_0x10 = 0x0;
  iVar3[0x1].field_0x1c = 0x0;
  param_2->field_0x0 = 0x1fb0;
  iVar3->field_0x2 = 0x1018;
  iVar3->field_0x20 = 0x1fec;
  iVar3->field_0x22 = 0x1018;
  pass1_1000_4906((astruct_20 *)((ulong)param_2 & 0xffff0000 | (ulong)(uint)&iVar3[0x1].field_0x14),(WNDCLASS16 *)0x0,
                  0x8);
  piVar7 = &local_4;
  piVar5 = &local_6;
  uVar6 = param_1;
  uVar8 = param_1;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_1,puVar1,unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0xe)),(ushort *)CONCAT22(uVar6,piVar5),
                  (ushort *)CONCAT22(uVar8,piVar7));
  paVar3 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x9a,param_1);
  iVar3->field_0x24 = (ushort)paVar3;
  *(undefined2 *)&iVar3->field_0x26 = (int)((ulong)paVar3 >> 0x10);
  uVar4 = pass1_1008_4772((astruct_76 *)((ulong)paVar3 & 0xffff0000 | (ulong)iVar3->field_0x24));
  uVar1 = (ushort)(uVar4 >> 0x10);
  pass1_1000_4906((astruct_20 *)((ulong)param_2 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x32),(WNDCLASS16 *)0x0,0x8);
  iVar3->field_0x36 = *(int *)((int)uVar4 + 0x4);
  iVar3->field_0x38 = *(int *)((int)uVar4 + 0x8);
  iVar3->field_0x2a = local_4 + 0x14;
  iVar3->field_0x2c = local_6 + 0x14;
  get_sys_metrics_1018_1ea0(param_2,0x1000);
  pass1_1008_3e76((ushort *)((ulong)param_2 & 0xffff0000 | ZEXT24(iVar3 + 0x1)),0x0,0x88,0x99);
  return;
}



void __stdcall16far pass1_1018_1a04(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined2 *puVar4;
  astruct_501 *iVar5;
  undefined2 uVar5;
  undefined2 *puStack14;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_501 *)param_1;
  *param_1 = 0x1fb0;
  iVar5->field_0x2 = 0x1018;
  iVar5->field_0x20 = 0x1fec;
  iVar5->field_0x22 = 0x1018;
  puVar1 = iVar5->field_0x24;
  uVar2 = iVar5->field_0x26;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x40,0x1000);
  if (param_1 == (ushort *)0x0) {
    puVar4 = (undefined2 *)0x0;
    uVar5 = 0x0;
  }
  else {
    puVar4 = &iVar5->field_0x20;
  }
  puStack14 = (undefined2 *)CONCAT22(uVar5,puVar4);
  *puStack14 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_1a8e(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  long lVar1;
  astruct_653 *iVar2;
  uint uVar2;
  ushort *puVar3;
  int *piVar4;
  int local_8;
  undefined4 uStack6;
  
  uVar2 = (uint)(param_1 >> 0x10);
  iVar2 = (astruct_653 *)param_1;
  if (iVar2->field_0x44 != 0x0) {
    if (iVar2->field_0x46 != 0x0) {
      lVar1 = iVar2->field_0x46;
      *(undefined2 *)((int)lVar1 + 0xe) = 0x0;
      iVar2->field_0x46 = 0x0;
    }
    piVar4 = &iVar2->field_0x4a;
    *piVar4 = *piVar4 + 0x1;
    return;
  }
  piVar4 = (int *)CONCAT22(param_4,&local_8);
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,param_2,param_3);
  pass1_1010_bf1e((ulong)puVar3,piVar4,(int)puVar3,(uchar *)((ulong)puVar3 >> 0x10),param_4);
  iVar2->field_0x44 = local_8;
  iVar2->field_0x40 = uStack6;
  pass1_1018_1ce8(param_4,param_1 & 0xffff | (ulong)uVar2 << 0x10);
  return;
}



void __stdcall16far pass1_1018_1b02(ushort param_1,ulong param_2,int param_3)

{
  int *piVar1;
  ulong uVar2;
  ulong uVar3;
  astruct_96 *uVar4;
  astruct_95 *iVar5;
  undefined2 uVar5;
  int iStack12;
  ushort local_6;
  undefined local_4 [0x2];
  
  iStack12 = 0x0;
  while( true ) {
    uVar5 = (undefined2)(param_2 >> 0x10);
    iVar5 = (astruct_95 *)param_2;
    piVar1 = &iVar5->field_0x44;
    if (*piVar1 == iStack12 || *piVar1 < iStack12) break;
    uVar2 = iVar5->field_0x40;
    uVar4 = (astruct_96 *)uVar2;
    uVar4 = uVar4 + iStack12;
    uVar2 = uVar2 & 0xffff0000;
    uVar3 = ZEXT24(uVar4);
    piVar1 = &uVar4->field_0x6;
    *piVar1 = *piVar1 + param_3 * 0x2 + -0x1;
    uVar5 = (undefined2)(uVar2 >> 0x10);
    if (0x23 < uVar4->field_0x6) {
      uVar4->field_0x6 = 0x0;
    }
    if (uVar4->field_0x6 < 0x0) {
      uVar4->field_0x6 = 0x23;
    }
    pass1_1008_3f62((ushort *)(uVar2 | (uint)&uVar4->field_0x10),(ushort *)(uVar2 | uVar3));
    uVar4->field_0x16 = uVar4->field_0xa;
    pass1_1008_3e94((ushort *)(uVar2 | uVar3),(ushort *)CONCAT22(param_1,&local_6),(ushort *)CONCAT22(param_1,local_4));
    pass1_1008_3e76((ushort *)(uVar2 | uVar3),0x0,local_6,
                    *(ushort *)((uVar4->field_0x8 * 0x24 + uVar4->field_0x6) * 0x2 + 0x3c20));
    uVar4->field_0xa = *(undefined2 *)(uVar4->field_0x6 * 0x2 + 0x3966);
    iStack12 = iStack12 + 0x1;
  }
  pass1_1010_1f62(param_1,param_2,0xd);
  return;
}
