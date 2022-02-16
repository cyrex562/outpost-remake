
void __stdcall16far pass1_1020_86d8(ulong param_1)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    uVar4 = (undefined2)(param_1 >> 0x10);
    piVar1 = (int *)((int)param_1 + 0x6);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    uVar2 = *(undefined4 *)((int)param_1 + 0xc);
    uVar4 = (undefined2)((ulong)uVar2 >> 0x10);
    iVar3 = (int)uVar2;
    if (*(long *)(iVar3 + iStack4 * 0x4) != 0x0) {
      pass1_1008_5236(*(ulong *)(iVar3 + iStack4 * 0x4));
    }
    iStack4 = iStack4 + 0x1;
  }
  return;
}



void __stdcall16far pass1_1020_8712(ulong param_1,int *param_2,astruct_76 *param_3,ushort *param_4)

{
  undefined2 uVar1;
  ulong uVar2;
  
  pass1_1008_3f32((int *)param_4,(int *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x10)));
  uVar2 = pass1_1008_4772(param_3);
  uVar1 = (undefined2)(uVar2 >> 0x10);
  pass1_1008_3e94(param_4,(ushort *)((ulong)param_2 & 0xffff0000 | ZEXT24((int *)((int)param_2 + 0x2))),
                  (ushort *)((ulong)param_2 & 0xffff | (ulong)param_2._2_2_ << 0x10));
  *(int *)((int)param_2 + 0x4) = *(int *)((int)uVar2 + 0x4) + *param_2;
  *(int *)((int)param_2 + 0x6) = *(int *)((int)uVar2 + 0x8) + *(int *)((int)param_2 + 0x2);
  return;
}



astruct_18 * __stdcall16far pass1_1020_8784(astruct_18 *param_1,byte param_2)

{
  pass1_1020_8556(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_87c2(ushort *param_1,ushort param_2,int param_3)

{
  undefined4 uVar1;
  astruct_281 *iVar2;
  uint uVar2;
  ushort *puVar3;
  undefined local_12 [0x8];
  int iStack10;
  ushort *puStack8;
  int iStack4;
  
  struct_1020_847a(param_1,0x4,param_2);
  iStack4 = 0x4;
  iVar2 = (astruct_281 *)param_1;
  iVar2 = (astruct_281 *)&iVar2->field_0x16;
  puStack8 = (ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar2));
  do {
    pass1_1008_3e38(puStack8);
    puStack8 = (ushort *)((ulong)puStack8 & 0xffff0000 | (ulong)((int)puStack8 + 0x6));
    iStack4 = iStack4 + -0x1;
  } while (iStack4 != 0x0);
  uVar2 = (uint)((ulong)param_1 >> 0x10);
  *(undefined4 *)&iVar2->field_0x2e = 0x0;
  puVar3 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x32));
  iVar2->field_0x38 = 0x0;
  *param_1 = 0x8a84;
  iVar2->field_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_2,(uchar *)((ulong)puVar3 >> 0x10),param_3);
  iVar2->field_0x2e = (int)puVar3;
  iVar2->field_0x30 = (int)((ulong)puVar3 >> 0x10);
  iStack10 = 0x0;
  do {
    uVar1 = *(undefined4 *)&iVar2->field_0x2e;
    pass1_1018_26d8((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),iStack10,
                    (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)(&iVar2->field_0x16 + iStack10 * 0x6)));
    uVar1 = *(undefined4 *)&iVar2->field_0x2e;
    pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10,
                    (int *)CONCAT22(iVar2->field_0xa,iVar2->field_0x8 + iStack10 * 0x8),
                    *(astruct_76 **)((int)uVar1 + 0x2e + iStack10 * 0x4),
                    (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)(&iVar2->field_0x16 + iStack10 * 0x6)));
    iStack10 = iStack10 + 0x1;
  } while (iStack10 < 0x4);
  uVar1 = *(undefined4 *)&iVar2->field_0x2e;
  pass1_1018_2548((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),
                  (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x32));
  uVar1 = *(undefined4 *)&iVar2->field_0x2e;
  iVar2->field_0x38 = *(undefined4 *)((int)uVar1 + 0x6e);
  pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10,(int *)CONCAT22(param_2,local_12),
                  (astruct_76 *)iVar2->field_0x38,
                  (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x32));
  return;
}



void __stdcall16far pass1_1020_8908(ulong param_1,ulong param_2,ushort param_3)

{
  astruct_76 *paVar1;
  undefined4 uVar2;
  uint uVar3;
  uint uVar4;
  uchar *puVar5;
  uchar *puVar6;
  undefined2 uVar7;
  astruct_284 *iVar8;
  int iVar9;
  int iVar10;
  undefined2 uVar11;
  undefined2 uVar12;
  ulong uVar13;
  astruct_110 *paStack28;
  int iStack4;
  
  for (iStack4 = 0x0; iVar8 = (astruct_284 *)param_1, uVar11 = (undefined2)(param_1 >> 0x10), iStack4 < 0x4;
      iStack4 = iStack4 + 0x1) {
    if (iVar8->field_0x4 == 0x0) {
      uVar2 = iVar8->field_0xc;
      uVar11 = (undefined2)((ulong)uVar2 >> 0x10);
      iVar10 = (int)uVar2;
      iVar9 = iStack4 * 0x4;
      if ((*(uint *)(iVar10 + iVar9 + 0x2) | *(uint *)(iVar10 + iVar9)) != 0x0) {
        pass1_1008_5236(*(ulong *)(iVar10 + iVar9));
      }
    }
    else {
      uVar2 = iVar8->field_0x2e;
      paVar1 = *(astruct_76 **)((int)uVar2 + 0x2e + iStack4 * 0x4);
      uVar13 = pass1_1008_4772(paVar1);
      puVar5 = (uchar *)(uVar13 >> 0x10);
      uVar3 = (uint)uVar13;
      uVar2 = iVar8->field_0xc;
      iVar10 = iStack4 * 0x4;
      if (*(long *)((int)uVar2 + iVar10) == 0x0) {
        puVar6 = puVar5;
        uVar4 = uVar3;
        mem_op_1000_179c(0x14,puVar5,0x1000);
        paStack28 = (astruct_110 *)CONCAT22(puVar6,uVar4);
        if (((uint)puVar6 | uVar4) == 0x0) {
          uVar2 = iVar8->field_0xc;
          *(undefined4 *)((int)uVar2 + iStack4 * 0x4) = 0x0;
        }
        else {
          uVar4 = &iVar8->field_0x16 + iStack4 * 0x6;
          uVar7 = uVar11;
          pass1_1008_50c2(paStack28,*(ulong *)(uVar3 + 0x8),*(ulong *)(uVar3 + 0x4),
                          (uint *)(param_1 & 0xffff0000 | (ulong)uVar4),param_2);
          uVar2 = iVar8->field_0xc;
          uVar12 = (undefined2)((ulong)uVar2 >> 0x10);
          iVar9 = (int)uVar2;
          *(uint *)(iVar9 + iVar10) = uVar4;
          *(undefined2 *)(iVar9 + iVar10 + 0x2) = uVar7;
        }
        uVar2 = iVar8->field_0xc;
        pass1_1008_5134(*(ulong *)((int)uVar2 + iStack4 * 0x4));
      }
      uVar2 = iVar8->field_0xc;
      pass1_1008_5236(*(ulong *)((int)uVar2 + iStack4 * 0x4));
      pass1_1008_4480(param_2,(ushort *)(param_1 & 0xffff0000 | (ulong)(uint)(&iVar8->field_0x16 + iStack4 * 0x6)),
                      paVar1,param_3);
    }
  }
  if (iVar8->field_0x4 != 0x0) {
    pass1_1008_4480(param_2,(ushort *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar8->field_0x32),iVar8->field_0x38,param_3
                   );
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_8a5e(astruct_18 *param_1,byte param_2)

{
  pass1_1020_8556(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_8a9c(ushort *param_1)

{
  undefined4 uVar1;
  ulong uVar2;
  uint uVar3;
  ushort uVar4;
  ushort unaff_SS;
  ushort *puVar5;
  astruct_76 *paVar6;
  astruct_43 *paVar7;
  int iVar8;
  uint uVar9;
  ushort *puStack76;
  undefined local_48 [0x1e];
  undefined local_2a [0x24];
  undefined2 uStack6;
  undefined2 uStack4;
  
  iVar8 = (int)param_1;
  uVar9 = (uint)((ulong)param_1 >> 0x10);
  struct_1020_847a(param_1,0x2,unaff_SS);
  uVar3 = iVar8 + 0x16;
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar3));
  puStack76 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar8 + 0x1cU));
  puVar5 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar8 + 0x1cU)));
  *(undefined4 *)(iVar8 + 0x22) = 0x0;
  *param_1 = 0x8e92;
  *(undefined2 *)(iVar8 + 0x2) = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,unaff_SS,(uchar *)((ulong)puVar5 >> 0x10),uVar9);
  uVar4 = (ushort)((ulong)puVar5 >> 0x10);
  *(undefined2 *)(iVar8 + 0x22) = (int)puVar5;
  *(ushort *)(iVar8 + 0x24) = uVar4;
  pass1_1018_2678(*(ushort *)(iVar8 + 0x22),uVar4,(ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar3));
  paVar6 = (astruct_76 *)pass1_1018_268e(*(ulong *)(iVar8 + 0x22));
  uStack4 = (undefined2)((ulong)paVar6 >> 0x10);
  uStack6 = SUB42(paVar6,0x0);
  pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar9 << 0x10,*(int **)(iVar8 + 0x8),paVar6,
                  (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar3));
  uVar1 = *(undefined4 *)(iVar8 + 0x22);
  pass1_1018_26c2((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),puStack76);
  paVar7 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x2,unaff_SS);
  struct_op_1008_48fe((astruct_81 *)CONCAT13((char)(unaff_SS >> 0x8),CONCAT12((char)unaff_SS,local_2a)),0x1,
                      (char *)paVar7,(ushort)((ulong)paVar7 >> 0x10));
  struct_op_1008_3f92((astruct_76 *)CONCAT22(unaff_SS,local_48),(astruct_83 *)CONCAT22(unaff_SS,local_2a));
  uVar2 = *(ulong *)(iVar8 + 0x8);
  pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar9 << 0x10,(int *)(uVar2 & 0xffff0000 | (ulong)((int)uVar2 + 0x8))
                  ,(astruct_76 *)CONCAT22(unaff_SS,local_48),puStack76);
  pass1_1008_41bc((ushort *)CONCAT22(unaff_SS,local_48));
  close_file_1008_496c(local_2a,unaff_SS);
  return;
}



void __stdcall16far pass1_1020_8bae(ushort *param_1)

{
  *param_1 = 0x8e92;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  pass1_1020_8556(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_8bcc(ulong param_1,ushort param_2)

{
  undefined4 uVar1;
  ulong *puVar2;
  uint uVar3;
  uint uVar4;
  uint *puVar5;
  uchar *puVar6;
  uint uVar7;
  undefined2 extraout_DX;
  astruct_285 *iVar9;
  astruct_286 *iVar10;
  undefined2 uVar8;
  undefined2 uVar9;
  astruct_43 *paVar10;
  undefined local_58 [0x1e];
  undefined local_3a [0x26];
  ulong uStack20;
  ushort uStack12;
  astruct_76 *paStack10;
  ulong uStack6;
  
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar9 = (astruct_285 *)param_1;
  if (iVar9->field_0x4 != 0x0) {
    uVar1 = iVar9->field_0x22;
    uStack6 = *(ulong *)((int)uVar1 + 0xa);
    paStack10 = (astruct_76 *)pass1_1018_268e(iVar9->field_0x22);
    uVar9 = (undefined2)((ulong)paStack10 >> 0x10);
    uVar1 = iVar9->field_0x22;
    uStack12 = *(ushort *)((int)uVar1 + 0x16);
    if (*iVar9->field_0xc == 0x0) {
      uStack20 = pass1_1008_4772(paStack10);
      puVar6 = (uchar *)(uStack20 >> 0x10);
      uVar3 = (uint)uStack20;
      mem_op_1000_179c(0x14,puVar6,0x1000);
      uVar7 = (uint)puVar6 | uVar3;
      if (uVar7 == 0x0) {
        *iVar9->field_0xc = 0x0;
      }
      else {
        puVar5 = (uint *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar9->field_0x16);
        uVar9 = (undefined2)(uStack20 >> 0x10);
        pass1_1008_50c2((astruct_110 *)CONCAT22(puVar6,uVar3),*(ulong *)((int)uStack20 + 0x8),
                        *(ulong *)((int)uStack20 + 0x4),puVar5,uStack6);
        puVar2 = iVar9->field_0xc;
        *(int *)puVar2 = (int)puVar5;
        *(uint *)((int)puVar2 + 0x2) = uVar7;
      }
      pass1_1008_5134(*iVar9->field_0xc);
      paVar10 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x2,param_2);
      struct_op_1008_48fe((astruct_81 *)CONCAT22(param_2,local_3a),0x1,(char *)paVar10,(ushort)((ulong)paVar10 >> 0x10))
      ;
      struct_op_1008_3f92((astruct_76 *)CONCAT22(param_2,local_58),(astruct_83 *)CONCAT22(param_2,local_3a));
      uStack20 = pass1_1008_4772((astruct_76 *)CONCAT22(param_2,local_58));
      puVar6 = (uchar *)(uStack20 >> 0x10);
      uVar3 = (uint)uStack20;
      mem_op_1000_179c(0x14,puVar6,0x1000);
      uVar7 = (uint)puVar6 | uVar3;
      if (uVar7 == 0x0) {
        puVar2 = iVar9->field_0xc;
        *(undefined4 *)((int)puVar2 + 0x4) = 0x0;
      }
      else {
        uVar4 = &iVar9->field_0x16;
        uVar9 = (undefined2)(uStack20 >> 0x10);
        pass1_1008_50c2((astruct_110 *)CONCAT22(puVar6,uVar3),*(ulong *)((int)uStack20 + 0x8),
                        *(ulong *)((int)uStack20 + 0x4),(uint *)(param_1 & 0xffff0000 | (ulong)uVar4),uStack6);
        puVar2 = iVar9->field_0xc;
        uVar9 = (undefined2)((ulong)puVar2 >> 0x10);
        iVar10 = (astruct_286 *)puVar2;
        iVar10->field_0x4 = uVar4;
        iVar10->field_0x6 = uVar7;
      }
      puVar2 = iVar9->field_0xc;
      pass1_1008_5134(*(ulong *)((int)puVar2 + 0x4));
      pass1_1008_41bc((ushort *)CONCAT22(param_2,local_58));
      close_file_1008_496c(local_3a,param_2);
      uVar9 = extraout_DX;
    }
    puVar2 = iVar9->field_0xc;
    pass1_1008_5236(*(ulong *)((int)puVar2 + 0x4));
    pass1_1008_5236(*iVar9->field_0xc);
    uVar3 = &iVar9->field_0x16;
    pass1_1008_4480(uStack6,(ushort *)(param_1 & 0xffff0000 | (ulong)uVar3),paStack10,param_2);
    invalidate_rect_1020_8d90(param_1,uStack12,uStack6,uVar3,uVar9,param_2);
  }
  return;
}



void __stdcall16far
invalidate_rect_1020_8d90(ulong param_1,ushort param_2,ulong param_3,undefined2 param_4,uint param_5,ushort param_6)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  undefined in_AF;
  int local_48;
  int iStack70;
  int iStack68;
  int iStack66;
  int local_40;
  int local_3e;
  ulong uStack60;
  undefined local_38 [0x28];
  uchar local_10 [0xa];
  uint uStack6;
  uint uStack4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  uStack6 = pass1_1018_266a(*(ulong *)(iVar2 + 0x22));
  if (uStack6 != 0x0) {
    pass1_1018_265c(*(undefined4 *)(iVar2 + 0x22));
    if ((param_5 | uStack6) != 0x0) {
      uStack4 = param_5;
      sys_1000_3f9c(local_10,(uchar *)param_6,(ushort)s__03ld_1050_442a,(ushort)&USHORT_1050_1050,uStack6,&stack0xfffe,
                    uVar3,0x1000,param_6,in_AF);
      uVar1 = *(undefined4 *)(iVar2 + 0x22);
      file_and_draw_op_1008_4f20
                ((ushort *)CONCAT22(param_6,local_38),*(ulong *)((int)uVar1 + 0xe),0x25,CONCAT22(param_6,local_10),
                 param_6);
      pass1_1008_4480(param_3,(ushort *)(param_1 & 0xffff0000 | (ulong)(iVar2 + 0x1c)),
                      (astruct_76 *)CONCAT22(param_6,local_38),param_6);
      uStack60 = pass1_1008_4772((astruct_76 *)CONCAT22(param_6,local_38));
      pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar2 + 0x1c)),(ushort *)CONCAT22(param_6,&local_40),
                      (ushort *)CONCAT22(param_6,&local_3e));
      local_48 = local_3e;
      iStack70 = local_40;
      uVar3 = (undefined2)(uStack60 >> 0x10);
      iStack68 = local_3e + *(int *)((int)uStack60 + 0x4);
      iStack66 = local_40 + *(int *)((int)uStack60 + 0x8);
      InvalidateRect16(0x1008,(RECT16 *)0x0,(BOOL16)&local_48);
      pass1_1008_41bc((ushort *)CONCAT22(param_6,local_38));
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_8e6c(astruct_18 *param_1,byte param_2)

{
  pass1_1020_8bae(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_8eaa(ushort *param_1,ushort param_2)

{
  uint uVar1;
  ushort uVar2;
  uchar *puVar3;
  astruct_668 *iVar4;
  uint uVar4;
  ushort *puVar5;
  astruct_43 *paVar6;
  undefined local_a [0x8];
  
  struct_1020_847a(param_1,0x25,param_2);
  uVar4 = (uint)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_668 *)param_1;
  *(undefined4 *)&iVar4->field_0x16 = 0x0;
  iVar4->field_0xaa = 0x0;
  uVar1 = &iVar4->field_0xae;
  puVar5 = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar1));
  *(undefined4 *)&iVar4->field_0xb4 = 0x0;
  iVar4->field_0xb8 = 0xffff;
  *(undefined4 *)&iVar4->field_0xba = 0x0;
  *param_1 = 0x9204;
  iVar4->field_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_2,(uchar *)((ulong)puVar5 >> 0x10),uVar4);
  uVar2 = (ushort)((ulong)puVar5 >> 0x10);
  iVar4->field_0x16 = (int)puVar5;
  iVar4->field_0x18 = uVar2;
  pass1_1018_2646(iVar4->field_0x16,uVar2,(ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar1));
  paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1ce,param_2);
  puVar3 = (uchar *)((ulong)paVar6 >> 0x10);
  iVar4->field_0xb4 = (uint)paVar6;
  iVar4->field_0xb6 = puVar3;
  pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10,(int *)CONCAT22(param_2,local_a),
                  (astruct_76 *)((ulong)paVar6 & 0xffff0000 | (ulong)iVar4->field_0xb4),
                  (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar1));
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,puVar3,uVar4);
  iVar4->field_0xba = (int)puVar5;
  iVar4->field_0xbc = (int)((ulong)puVar5 >> 0x10);
  return;
}



void __stdcall16far pass1_1020_8f74(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_593 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_593 *)param_1;
  *param_1 = 0x9204;
  iVar4->field_0x2 = 0x1020;
  puVar1 = iVar4->field_0xb4;
  uVar2 = iVar4->field_0xb6;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1020_8556(param_1);
  return;
}



void __stdcall16far invalidate_rect_1020_8fb4(ulong param_1,ushort param_2)

{
  int iVar1;
  undefined4 uVar2;
  uint erase;
  ulong uVar3;
  uint in_DX;
  uint extraout_DX;
  uint uVar4;
  int iVar5;
  undefined2 uVar6;
  ushort unaff_SS;
  int iStack8;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  uVar2 = *(undefined4 *)(iVar5 + 0xba);
  if (*(int *)((int)uVar2 + 0x1e) != 0x0) {
    pass1_1018_2862(*(ulong *)(iVar5 + 0x16));
    *(ushort *)(iVar5 + 0xaa) = param_2;
    *(uint *)(iVar5 + 0xac) = in_DX;
    if ((in_DX | *(uint *)(iVar5 + 0xaa)) != 0x0) {
      uVar2 = *(undefined4 *)(iVar5 + 0xaa);
      iVar1 = *(int *)((int)uVar2 + 0xa);
      for (iStack8 = 0x0; iStack8 < iVar1; iStack8 = iStack8 + 0x1) {
        uVar3 = SEXT24(iStack8);
        empty_1008_8fc4(*(undefined4 *)(iVar5 + 0xaa),uVar3);
        erase = (uint)uVar3;
        uVar4 = extraout_DX | erase;
        if (((uVar4 != 0x0) && (0x9 < *(int *)(erase + 0x2e))) &&
           (pass1_1008_8b20(uVar3 & 0xffff | (ulong)extraout_DX << 0x10,unaff_SS), (uVar4 | erase) != 0x0)) {
          InvalidateRect16(0x1008,(RECT16 *)0x0,erase);
        }
      }
    }
  }
  return;
}



void __stdcall16far pass1_1020_9068(ulong *param_1,uchar *param_2,int param_3,ushort param_4)

{
  int iVar1;
  ulong uVar2;
  code **ppcVar3;
  undefined4 uVar4;
  uint uVar5;
  ulong uVar6;
  uint extraout_DX;
  uint uVar7;
  int iVar8;
  int iVar9;
  uint uVar10;
  undefined2 uVar11;
  int iStack10;
  
  uVar10 = (uint)((ulong)param_1 >> 0x10);
  iVar8 = (int)param_1;
  uVar4 = *(undefined4 *)(iVar8 + 0x16);
  uVar2 = *(ulong *)((int)uVar4 + 0xa);
  uVar6 = uVar2;
  pass1_1018_280c(*(ulong *)(iVar8 + 0x16));
  *(undefined2 *)(iVar8 + 0xaa) = (int)uVar6;
  *(uchar **)(iVar8 + 0xac) = param_2;
  uVar5 = (uint)param_2 | *(uint *)(iVar8 + 0xaa);
  if (uVar5 == 0x0) {
    pass1_1018_2862(*(ulong *)(iVar8 + 0x16));
    *(uint *)(iVar8 + 0xaa) = uVar5;
    *(uchar **)(iVar8 + 0xac) = param_2;
  }
  if ((*(uint *)(iVar8 + 0xac) | *(uint *)(iVar8 + 0xaa)) != 0x0) {
    pass1_1020_915a((ulong)param_1 & 0xffff | (ulong)uVar10 << 0x10,param_2,param_3,param_4);
    pass1_1008_4480(uVar2,(ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar8 + 0xae)),*(astruct_76 **)(iVar8 + 0xb4)
                    ,param_4);
    ppcVar3 = (code **)((int)*param_1 + 0x10);
    (**ppcVar3)();
    uVar4 = *(undefined4 *)(iVar8 + 0xaa);
    iVar1 = *(int *)((int)uVar4 + 0xa);
    for (iStack10 = 0x0; iStack10 < iVar1; iStack10 = iStack10 + 0x1) {
      uVar6 = SEXT24(iStack10);
      empty_1008_8fc4(*(undefined4 *)(iVar8 + 0xaa),uVar6);
      uVar5 = (uint)uVar6;
      uVar7 = extraout_DX | uVar5;
      if (uVar7 != 0x0) {
        pass1_1008_8c4e(uVar6 & 0xffff | (ulong)extraout_DX << 0x10,uVar2,param_4);
        uVar4 = *(undefined4 *)(iVar8 + 0xc);
        uVar11 = (undefined2)((ulong)uVar4 >> 0x10);
        iVar9 = (int)uVar4;
        *(uint *)(iVar9 + iStack10 * 0x4) = uVar5;
        *(uint *)(iVar9 + iStack10 * 0x4 + 0x2) = uVar7;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_915a(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  int iVar1;
  astruct_669 *iVar2;
  undefined2 uVar2;
  ushort *puVar3;
  astruct_43 *paVar4;
  ushort uStack12;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,param_2,param_3);
  iVar1 = *(int *)((int)puVar3 + 0x1e);
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar2 = (astruct_669 *)param_1;
  if (iVar2->field_0xb8 != iVar1) {
    uStack12 = 0x1ce;
    if (iVar1 == 0x1) {
      uStack12 = 0x1cf;
    }
    else {
      if (iVar1 == 0x2) {
        uStack12 = 0x1d0;
      }
      else {
        if (iVar1 == 0x3) {
          uStack12 = 0x1d1;
        }
        else {
          if (iVar1 == 0x4) {
            uStack12 = 0x1d2;
          }
        }
      }
    }
    paVar4 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,uStack12,param_4);
    iVar2->field_0xb4 = (int)paVar4;
    iVar2->field_0xb6 = (int)((ulong)paVar4 >> 0x10);
    iVar2->field_0xb8 = iVar1;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1020_91de(astruct_18 *param_1,byte param_2)

{
  pass1_1020_8f74(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far set_struct_op_1020_921c(astruct_42 *param_1,ushort param_2)

{
  HDC16 HVar1;
  uchar *in_DX;
  astruct_42 *iVar3;
  int unaff_DI;
  astruct_42 *uVar3;
  HWND16 unaff_CS;
  ushort unaff_SS;
  ushort *pUVar3;
  
  uVar3 = (astruct_42 *)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_42 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x3aa8;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x4 = param_2;
  param_1->field_0x0 = 0x3ab0;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x6 = 0x0;
  iVar3->field_0xa = 0x0;
  iVar3->field_0xc = 0x0;
  iVar3->field_0xe = 0x0;
  iVar3->field_0x10 = 0x0;
  iVar3->field_0x12 = 0x0;
  param_1->field_0x0 = 0x96c8;
  iVar3->field_0x2 = 0x1020;
  HVar1 = GetDC16(unaff_CS);
  iVar3->field_0xa = HVar1;
  pUVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  pUVar3._2_2_ = (undefined2)((ulong)pUVar3 >> 0x10);
  iVar3->field_0xc = *(ushort *)((int)pUVar3 + 0xa);
  iVar3->field_0xe = *(ushort *)((int)pUVar3 + 0xc);
  return;
}



void __stdcall16far palette_op_1020_92c4(undefined2 *param_1,HDC16 param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x96c8;
  *(undefined2 *)(iVar1 + 0x2) = 0x1020;
  if (*(int *)(iVar1 + 0x12) != 0x0) {
    SelectPalette16(param_2,0x0,*(BOOL16 *)(iVar1 + 0x12));
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  }
  *param_1 = 0x3ab0;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}



void __stdcall16far mix_draw_op_1020_9312(ulong param_1,HWND16 param_2)

{
  undefined4 *puVar1;
  code **ppcVar2;
  undefined4 uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  PAINTSTRUCT16 local_22;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar6 = *(undefined2 *)(iVar4 + 0x4);
  BeginPaint16(param_2,&local_22);
  uVar3 = *(undefined4 *)(iVar4 + 0x6);
  puVar1 = (undefined4 *)*(undefined4 *)((int)uVar3 + 0xa);
  ppcVar2 = (code **)((int)*puVar1 + 0x4);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,(int)puVar1,(int)((ulong)puVar1 >> 0x10),0x0,
              param_1 & 0xffff0000 | (ulong)(iVar4 + 0xa),uVar6);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far draw_op_1020_9364(astruct_7 *param_1,HWND16 in_win_handle_2,INT16 param_3)

{
  int *piVar1;
  uint uVar2;
  int iVar3;
  undefined4 uVar4;
  int iVar5;
  RECT16 *pRVar6;
  astruct_7 *local_struct_1;
  undefined2 var7;
  undefined2 uVar7;
  int iStack62;
  uint uStack58;
  undefined local_38 [0x4];
  HGDIOBJ16 HStack52;
  HPEN16 HStack50;
  undefined2 uStack48;
  undefined4 uStack46;
  undefined4 uStack42;
  undefined4 uStack38;
  undefined4 uStack34;
  undefined4 uStack30;
  uint *puStack26;
  int iStack22;
  int iStack20;
  ulong local_12;
  undefined4 uStack14;
  RECT16 local_a;
  ulong uStack6;
  
  var7 = (undefined2)((ulong)param_1 >> 0x10);
  local_struct_1 = (astruct_7 *)param_1;
  GetClientRect16(in_win_handle_2,&local_a);
  local_12 = local_a;
  uStack14 = uStack6;
  iStack20 = DAT_1050_4216;
  iStack22 = DAT_1050_422c;
  puStack26 = _PTR_PTR_DAT_1050_0009_1050_4172_1050_4212;
  uStack30 = _PTR_PTR_1050_4218;
  uStack34 = _PTR_PTR_s_ew_failed_in_Op_Op_1050_0021_1050_41da_1050_421c;
  uStack38 = _PTR_PTR_DAT_1050_0041_1050_4202_1050_4220;
  uStack42 = _PTR_DAT_1050_419a_1050_4224;
  uStack46 = _PTR_PTR_1050_4228;
  uVar4 = local_struct_1->field_0x6;
  uStack48 = *(undefined2 *)((int)uVar4 + 0x12);
  uStack58 = 0x9;
  do {
    uVar4 = *(undefined4 *)(uStack58 * 0x4 + (int)uStack34);
    HStack50 = CreatePen16((INT16)s_tile2_bmp_1050_1538,(INT16)uVar4,(COLORREF)((ulong)uVar4 >> 0x10));
    HStack52 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack50);
    MoveToEx16((HDC16)s_tile2_bmp_1050_1538,(INT16)local_38,param_3,*(POINT16 **)(uStack58 * 0x2 + (int)puStack26));
    LineTo16((HDC16)s_tile2_bmp_1050_1538,*(INT16 *)((int)puStack26 + uStack58 * 0x2),(INT16)uStack6);
    iVar3 = (iStack20 - uStack58) * 0x2;
    MoveToEx16((HDC16)s_tile2_bmp_1050_1538,(INT16)local_38,param_3,*(POINT16 **)(iVar3 + (int)puStack26));
    LineTo16((HDC16)s_tile2_bmp_1050_1538,*(INT16 *)((int)puStack26 + iVar3),(INT16)uStack6);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack52);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    uStack58 = uStack58 - 0x1;
  } while (uStack58 < 0x8000);
  pRVar6 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  uVar7 = (undefined2)((ulong)puStack26 >> 0x10);
  local_a = CONCAT22(*(int *)((int)puStack26 + 0x12) + 0x1,local_a.x);
  uVar2 = *(uint *)((int)puStack26 + 0x14);
  uStack14 = uStack14 & 0xffff | (ulong)uVar2 << 0x10;
  uStack6 = CONCAT22(uVar2,(INT16)uStack6);
  FillRect16((HDC16)s_tile2_bmp_1050_1538,pRVar6,(HBRUSH16)&local_a);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  iStack62 = 0x8;
  for (uStack58 = 0x1; (int)uStack58 < 0xa; uStack58 = uStack58 + 0x1) {
    pRVar6 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
    uStack6 = uStack6 & 0xffff | (ulong)(local_a.y - 0x1) << 0x10;
    local_12 = local_12 & 0xffff | (ulong)(uStack14._2_2_ + 0x1) << 0x10;
    uVar7 = (undefined2)((ulong)puStack26 >> 0x10);
    local_a = local_a & 0xffff | (ulong)(*(int *)(iStack62 * 0x2 + (int)puStack26) + 0x1) << 0x10;
    uStack14 = uStack14 & 0xffff | (ulong)*(uint *)(uStack58 * 0x2 + (int)puStack26 + 0x14) << 0x10;
    FillRect16((HDC16)s_tile2_bmp_1050_1538,pRVar6,(HBRUSH16)&local_a);
    FillRect16((HDC16)s_tile2_bmp_1050_1538,pRVar6,(HBRUSH16)&local_12);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    iStack62 = iStack62 + -0x1;
  }
  pRVar6 = (RECT16 *)CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  local_a = local_a & 0xffff;
  uStack6 = uStack6 & 0xffff | (ulong)*puStack26 << 0x10;
  local_12 = local_12 & 0xffff | (ulong)(*(int *)(iStack20 * 0x2 + (int)puStack26) + 0x1) << 0x10;
  uStack14 = uStack14 & 0xffff | (ulong)local_struct_1->field_0xe << 0x10;
  FillRect16((HDC16)s_tile2_bmp_1050_1538,pRVar6,(HBRUSH16)&local_a);
  FillRect16((HDC16)s_tile2_bmp_1050_1538,pRVar6,(HBRUSH16)&local_12);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  uStack58 = 0x3;
  do {
    uVar4 = *(undefined4 *)(uStack58 * 0x4 + (int)uStack38);
    HStack50 = CreatePen16((INT16)s_tile2_bmp_1050_1538,(INT16)uVar4,(COLORREF)((ulong)uVar4 >> 0x10));
    HStack52 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack50);
    iVar5 = uStack58 * 0x2;
    iVar3 = *(int *)(iVar5 + (int)uStack42);
    uVar7 = (undefined2)((ulong)uStack46 >> 0x10);
    piVar1 = (int *)(iVar5 + (int)uStack46);
    MoveToEx16((HDC16)s_tile2_bmp_1050_1538,(INT16)local_38,param_3,
               *(POINT16 **)(*(int *)(iVar5 + (int)uStack46) * 0x2 + (int)puStack26));
    LineTo16((HDC16)s_tile2_bmp_1050_1538,*(INT16 *)((iStack20 - *piVar1) * 0x2 + (int)puStack26),iVar3 + local_a.x);
    iVar3 = *(int *)((iStack22 - uStack58) * 0x2 + (int)uStack42);
    MoveToEx16((HDC16)s_tile2_bmp_1050_1538,(INT16)local_38,param_3,*(POINT16 **)(*piVar1 * 0x2 + (int)puStack26));
    LineTo16((HDC16)s_tile2_bmp_1050_1538,*(INT16 *)((iStack20 - *piVar1) * 0x2 + (int)puStack26),iVar3 + local_a.x);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack52);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    uStack58 = uStack58 - 0x1;
  } while (uStack58 < 0x8000);
  local_struct_1->field_0x10 = 0x0;
  return;
}



astruct_18 * __stdcall16far pass1_1020_96a2(astruct_18 *param_1,byte param_2,ushort param_3)

{
  palette_op_1020_92c4(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl16far init_globals_1020_96d4(void)

{
  undefined2 *puVar1;
  int iVar2;
  undefined2 *puVar3;
  
  _PTR_LOOP_1050_4514 = 0x0;
  _PTR_LOOP_1050_451a = 0x0;
  PTR_LOOP_1050_4520 = (undefined *)0x4430;
  PTR_LOOP_1050_4522 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4526 = (undefined *)0x4430;
  PTR_LOOP_1050_4528 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4524 = PTR_LOOP_1050_4434;
  PTR_LOOP_1050_452a = PTR_LOOP_1050_4434;
  PTR_LOOP_1050_452c = (undefined *)0x4430;
  PTR_LOOP_1050_452e = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4530 = PTR_LOOP_1050_4434;
  PTR_LOOP_1050_4532 = (undefined *)0x4430;
  PTR_LOOP_1050_4534 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4536 = PTR_LOOP_1050_4434;
  _PTR_LOOP_1050_4538 = 0x0;
  _PTR_LOOP_1050_453e = 0x0;
  PTR_LOOP_1050_4544 = (undefined *)0x4436;
  PTR_LOOP_1050_4546 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_454a = (undefined *)0x4436;
  PTR_LOOP_1050_454c = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4548 = PTR_LOOP_1050_443a;
  PTR_LOOP_1050_454e = PTR_LOOP_1050_443a;
  PTR_LOOP_1050_4550 = (undefined *)0x4436;
  PTR_LOOP_1050_4552 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4554 = PTR_LOOP_1050_443a;
  PTR_LOOP_1050_4512 = DAT_1050_4462;
  PTR_LOOP_1050_455a = DAT_1050_4462;
  PTR_LOOP_1050_4556 = (undefined *)0x4454;
  PTR_LOOP_1050_4558 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_455c = (undefined *)0x4454;
  PTR_LOOP_1050_455e = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4560 = DAT_1050_4462;
  PTR_LOOP_1050_4562 = (undefined *)0x4454;
  PTR_LOOP_1050_4564 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4566 = DAT_1050_4462;
  PTR_LOOP_1050_456a = (undefined *)0x0;
  PTR_LOOP_1050_4568 = (undefined *)0x0;
  PTR_LOOP_1050_456e = (undefined *)0x443c;
  PTR_LOOP_1050_4570 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4574 = (undefined *)0x443c;
  PTR_LOOP_1050_4576 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4572 = DAT_1050_4446;
  PTR_LOOP_1050_4578 = DAT_1050_4446;
  PTR_LOOP_1050_457a = (undefined *)0x443c;
  PTR_LOOP_1050_457c = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_457e = DAT_1050_4446;
  PTR_LOOP_1050_4580 = (undefined *)0x443c;
  PTR_LOOP_1050_4582 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4584 = DAT_1050_4446;
  PTR_LOOP_1050_4586 = (undefined *)0x443c;
  PTR_LOOP_1050_4588 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_458a = DAT_1050_4446;
  PTR_LOOP_1050_458c = (undefined *)0x443c;
  PTR_LOOP_1050_458e = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4590 = DAT_1050_4446;
  PTR_LOOP_1050_4592 = (undefined *)0x4454;
  PTR_LOOP_1050_4594 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4596 = DAT_1050_4462;
  PTR_LOOP_1050_4598 = (undefined *)0x4454;
  PTR_LOOP_1050_459a = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_459c = DAT_1050_4462;
  PTR_LOOP_1050_459e = (undefined *)0x4436;
  PTR_LOOP_1050_45a0 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_45a2 = PTR_LOOP_1050_443a;
  PTR_LOOP_1050_45a4 = (undefined *)0x4436;
  PTR_LOOP_1050_45a6 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_45a8 = PTR_LOOP_1050_443a;
  _PTR_LOOP_1050_45aa = 0x0;
  _PTR_LOOP_1050_45b0 = 0x0;
  _PTR_LOOP_1050_45b6 = 0x0;
  PTR_LOOP_1050_45bc = (undefined *)0x443c;
  PTR_LOOP_1050_45be = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_45c0 = DAT_1050_4446;
  PTR_LOOP_1050_45c2 = (undefined *)0x443c;
  PTR_LOOP_1050_45c4 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_45c6 = DAT_1050_4446;
  _PTR_LOOP_1050_45c8 = 0x0;
  _PTR_LOOP_1050_45ce = 0x0;
  _PTR_LOOP_1050_45d4 = 0x0;
  _PTR_LOOP_1050_45da = 0x0;
  PTR_LOOP_1050_45e0 = (undefined *)0x443c;
  PTR_LOOP_1050_45e2 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_45e4 = DAT_1050_4446;
  PTR_LOOP_1050_45e6 = (undefined *)0x443c;
  PTR_LOOP_1050_45e8 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_45ea = DAT_1050_4446;
  _PTR_LOOP_1050_45ec = 0x0;
  _PTR_LOOP_1050_45f2 = 0x0;
  _PTR_LOOP_1050_45f8 = 0x0;
  PTR_LOOP_1050_45fe = (undefined *)0x443c;
  PTR_LOOP_1050_4600 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4602 = DAT_1050_4446;
  PTR_LOOP_1050_4604 = (undefined *)0x443c;
  PTR_LOOP_1050_4606 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4608 = DAT_1050_4446;
  _PTR_LOOP_1050_460a = 0x0;
  _PTR_LOOP_1050_4610 = 0x0;
  PTR_LOOP_1050_451e = (undefined *)0xffff;
  PTR_LOOP_1050_45ae = (undefined *)0xffff;
  PTR_LOOP_1050_45b4 = (undefined *)0xffff;
  PTR_LOOP_1050_45ba = (undefined *)0xffff;
  PTR_LOOP_1050_45cc = (undefined *)0xffff;
  PTR_LOOP_1050_45d2 = (undefined *)0xffff;
  PTR_LOOP_1050_45f6 = (undefined *)0xffff;
  PTR_LOOP_1050_45fc = (undefined *)0xffff;
  PTR_LOOP_1050_460e = (undefined *)0xffff;
  PTR_LOOP_1050_4614 = (undefined *)0xffff;
  _PTR_LOOP_1050_4616 = 0x0;
  _PTR_LOOP_1050_461c = 0x0;
  _PTR_LOOP_1050_4622 = 0x0;
  _PTR_LOOP_1050_4628 = 0x0;
  _PTR_LOOP_1050_462e = 0x0;
  _PTR_LOOP_1050_4634 = 0x0;
  PTR_LOOP_1050_4518 = (undefined *)0x0;
  PTR_LOOP_1050_453c = (undefined *)0x0;
  PTR_LOOP_1050_4542 = (undefined *)0x0;
  PTR_LOOP_1050_456c = (undefined *)0x0;
  PTR_LOOP_1050_45d8 = (undefined *)0x0;
  PTR_LOOP_1050_45de = (undefined *)0x0;
  PTR_LOOP_1050_45f0 = (undefined *)0x0;
  PTR_LOOP_1050_461a = (undefined *)0x0;
  PTR_LOOP_1050_4620 = (undefined *)0x0;
  PTR_LOOP_1050_4626 = (undefined *)0x0;
  PTR_LOOP_1050_462c = (undefined *)0x0;
  PTR_LOOP_1050_4632 = (undefined *)0x0;
  PTR_LOOP_1050_4638 = (undefined *)0x0;
  _PTR_LOOP_1050_463a = 0x0;
  _PTR_LOOP_1050_4640 = 0x0;
  _PTR_LOOP_1050_4646 = 0x0;
  _PTR_LOOP_1050_464c = 0x0;
  _PTR_LOOP_1050_4652 = 0x0;
  _PTR_LOOP_1050_4658 = 0x0;
  PTR_LOOP_1050_465e = (undefined *)0x4448;
  PTR_LOOP_1050_4660 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4664 = (undefined *)0x4448;
  PTR_LOOP_1050_4666 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4662 = DAT_1050_4452;
  PTR_LOOP_1050_4668 = DAT_1050_4452;
  PTR_LOOP_1050_466a = (undefined *)0x4448;
  PTR_LOOP_1050_466c = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_466e = DAT_1050_4452;
  PTR_LOOP_1050_4670 = (undefined *)0x4454;
  PTR_LOOP_1050_4672 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4676 = (undefined *)0x4454;
  PTR_LOOP_1050_4678 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4674 = DAT_1050_4462;
  PTR_LOOP_1050_467a = DAT_1050_4462;
  PTR_LOOP_1050_467c = (undefined *)0x4454;
  PTR_LOOP_1050_467e = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4680 = DAT_1050_4462;
  PTR_LOOP_1050_4682 = (undefined *)0x4454;
  PTR_LOOP_1050_4684 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4686 = DAT_1050_4462;
  PTR_LOOP_1050_4688 = (undefined *)0x4454;
  PTR_LOOP_1050_468a = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_468c = DAT_1050_4462;
  PTR_LOOP_1050_468e = (undefined *)0x4448;
  PTR_LOOP_1050_4690 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4692 = DAT_1050_4452;
  PTR_LOOP_1050_4694 = (undefined *)0x4448;
  PTR_LOOP_1050_4696 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4698 = DAT_1050_4452;
  PTR_LOOP_1050_469a = (undefined *)0x4448;
  PTR_LOOP_1050_469c = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_469e = DAT_1050_4452;
  PTR_LOOP_1050_46a0 = (undefined *)0x4448;
  PTR_LOOP_1050_46a2 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_46a4 = DAT_1050_4452;
  PTR_LOOP_1050_46a6 = (undefined *)0x4454;
  PTR_LOOP_1050_46a8 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_46aa = DAT_1050_4462;
  PTR_LOOP_1050_46ac = (undefined *)0x4454;
  PTR_LOOP_1050_46ae = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_46b0 = DAT_1050_4462;
  PTR_LOOP_1050_46b2 = (undefined *)0x4454;
  PTR_LOOP_1050_46b4 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_46b6 = DAT_1050_4462;
  PTR_LOOP_1050_46b8 = (undefined *)0x4454;
  PTR_LOOP_1050_46ba = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_46bc = DAT_1050_4462;
  PTR_LOOP_1050_46be = (undefined *)0x4454;
  PTR_LOOP_1050_46c0 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_46c2 = DAT_1050_4462;
  PTR_LOOP_1050_46c6 = (undefined *)0x0;
  PTR_LOOP_1050_46c4 = (undefined *)0x0;
  PTR_LOOP_1050_46cc = (undefined *)0x0;
  PTR_LOOP_1050_46ca = (undefined *)0x0;
  PTR_LOOP_1050_46d2 = (undefined *)0x0;
  PTR_LOOP_1050_46d0 = (undefined *)0x0;
  PTR_LOOP_1050_46d8 = (undefined *)0x0;
  PTR_LOOP_1050_46d6 = (undefined *)0x0;
  PTR_LOOP_1050_46de = (undefined *)0x0;
  PTR_LOOP_1050_46dc = (undefined *)0x0;
  PTR_LOOP_1050_46e2 = (undefined *)0x4454;
  PTR_LOOP_1050_46e4 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_46e6 = DAT_1050_4462;
  PTR_LOOP_1050_46e8 = (undefined *)0x4448;
  PTR_LOOP_1050_46ea = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_46ec = DAT_1050_4452;
  PTR_LOOP_1050_46ee = (undefined *)0x4448;
  PTR_LOOP_1050_46f0 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_46f2 = DAT_1050_4452;
  _PTR_LOOP_1050_46f4 = 0x0;
  _PTR_LOOP_1050_46fa = 0x0;
  PTR_LOOP_1050_46f8 = (undefined *)0xffff;
  PTR_LOOP_1050_46fe = (undefined *)0xffff;
  _PTR_LOOP_1050_4700 = 0x0;
  _PTR_LOOP_1050_4706 = 0x0;
  PTR_LOOP_1050_470c = (undefined *)0x4448;
  PTR_LOOP_1050_470e = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4710 = DAT_1050_4452;
  PTR_LOOP_1050_4712 = (undefined *)0x4448;
  PTR_LOOP_1050_4714 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4716 = DAT_1050_4452;
  _PTR_LOOP_1050_4718 = 0x0;
  _PTR_LOOP_1050_471e = 0x0;
  _PTR_LOOP_1050_4724 = 0x0;
  _PTR_LOOP_1050_472a = 0x0;
  _PTR_LOOP_1050_4730 = 0x0;
  _PTR_LOOP_1050_4736 = 0x0;
  _PTR_LOOP_1050_473c = 0x0;
  _PTR_LOOP_1050_4742 = 0x0;
  _PTR_LOOP_1050_4748 = 0x0;
  _PTR_LOOP_1050_474e = 0x0;
  _PTR_LOOP_1050_4754 = 0x0;
  _PTR_LOOP_1050_475a = 0x0;
  _PTR_LOOP_1050_4760 = 0x0;
  PTR_LOOP_1050_463e = (undefined *)0x0;
  PTR_LOOP_1050_4644 = (undefined *)0x0;
  PTR_LOOP_1050_464a = (undefined *)0x0;
  PTR_LOOP_1050_4650 = (undefined *)0x0;
  PTR_LOOP_1050_4656 = (undefined *)0x0;
  PTR_LOOP_1050_465c = (undefined *)0x0;
  PTR_LOOP_1050_46c8 = (undefined *)0x0;
  PTR_LOOP_1050_46ce = (undefined *)0x0;
  PTR_LOOP_1050_46d4 = (undefined *)0x0;
  PTR_LOOP_1050_46da = (undefined *)0x0;
  PTR_LOOP_1050_46e0 = (undefined *)0x0;
  PTR_LOOP_1050_4704 = (undefined *)0x0;
  PTR_LOOP_1050_470a = (undefined *)0x0;
  PTR_LOOP_1050_471c = (undefined *)0x0;
  PTR_LOOP_1050_4722 = (undefined *)0x0;
  PTR_LOOP_1050_4728 = (undefined *)0x0;
  PTR_LOOP_1050_472e = (undefined *)0x0;
  PTR_LOOP_1050_4734 = (undefined *)0x0;
  PTR_LOOP_1050_473a = (undefined *)0x0;
  PTR_LOOP_1050_4740 = (undefined *)0x0;
  PTR_LOOP_1050_4746 = (undefined *)0x0;
  PTR_LOOP_1050_474c = (undefined *)0x0;
  PTR_LOOP_1050_4752 = (undefined *)0x0;
  PTR_LOOP_1050_4758 = (undefined *)0x0;
  PTR_LOOP_1050_475e = (undefined *)0x0;
  PTR_LOOP_1050_4764 = (undefined *)0x0;
  _PTR_LOOP_1050_4766 = 0x0;
  _PTR_LOOP_1050_476c = 0x0;
  _PTR_LOOP_1050_4772 = 0x0;
  _PTR_LOOP_1050_4778 = 0x0;
  _PTR_LOOP_1050_477e = 0x0;
  _PTR_LOOP_1050_4784 = 0x0;
  _PTR_LOOP_1050_478a = 0x0;
  _PTR_LOOP_1050_4790 = 0x0;
  _PTR_LOOP_1050_4796 = 0x0;
  _PTR_LOOP_1050_479c = 0x0;
  _PTR_LOOP_1050_47a2 = 0x0;
  _PTR_LOOP_1050_47a8 = 0x0;
  _PTR_LOOP_1050_47ae = 0x0;
  _PTR_LOOP_1050_47b4 = 0x0;
  PTR_LOOP_1050_476a = (undefined *)0x0;
  PTR_LOOP_1050_4770 = (undefined *)0x0;
  PTR_LOOP_1050_4776 = (undefined *)0x0;
  PTR_LOOP_1050_477c = (undefined *)0x0;
  PTR_LOOP_1050_4782 = (undefined *)0x0;
  PTR_LOOP_1050_4788 = (undefined *)0x0;
  PTR_LOOP_1050_478e = (undefined *)0x0;
  PTR_LOOP_1050_4794 = (undefined *)0x0;
  PTR_LOOP_1050_479a = (undefined *)0x0;
  PTR_LOOP_1050_47a0 = (undefined *)0x0;
  PTR_LOOP_1050_47a6 = (undefined *)0x0;
  PTR_LOOP_1050_47ac = (undefined *)0x0;
  PTR_LOOP_1050_47b2 = (undefined *)0x0;
  PTR_LOOP_1050_47b8 = (undefined *)0x0;
  puVar3 = (undefined2 *)0x47ba;
  for (iVar2 = 0x1b; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar1 = 0x0;
  }
  _PTR_LOOP_1050_4850 = 0x0;
  _PTR_LOOP_1050_4856 = 0x0;
  PTR_LOOP_1050_484e = PTR_LOOP_1050_4468;
  PTR_LOOP_1050_4860 = PTR_LOOP_1050_4468;
  PTR_LOOP_1050_485c = (undefined *)0x4464;
  PTR_LOOP_1050_485e = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4862 = (undefined *)0x4464;
  PTR_LOOP_1050_4864 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4866 = PTR_LOOP_1050_4468;
  PTR_LOOP_1050_4868 = (undefined *)0x4464;
  PTR_LOOP_1050_486a = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_486c = PTR_LOOP_1050_4468;
  PTR_LOOP_1050_486e = (undefined *)0x4464;
  PTR_LOOP_1050_4870 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4872 = PTR_LOOP_1050_4468;
  _PTR_LOOP_1050_4874 = 0x0;
  _PTR_LOOP_1050_487a = 0x0;
  PTR_LOOP_1050_4880 = (undefined *)0x4436;
  PTR_LOOP_1050_4882 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4886 = (undefined *)0x4436;
  PTR_LOOP_1050_4888 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4884 = PTR_LOOP_1050_443a;
  PTR_LOOP_1050_488a = PTR_LOOP_1050_443a;
  PTR_LOOP_1050_488c = (undefined *)0x4436;
  PTR_LOOP_1050_488e = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4890 = PTR_LOOP_1050_443a;
  PTR_LOOP_1050_4892 = (undefined *)0x4482;
  PTR_LOOP_1050_4894 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4898 = (undefined *)0x4482;
  PTR_LOOP_1050_489a = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4896 = PTR_LOOP_1050_4486;
  PTR_LOOP_1050_489c = PTR_LOOP_1050_4486;
  PTR_LOOP_1050_489e = (undefined *)0x4482;
  PTR_LOOP_1050_48a0 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48a2 = PTR_LOOP_1050_4486;
  PTR_LOOP_1050_48a6 = (undefined *)0x0;
  PTR_LOOP_1050_48a4 = (undefined *)0x0;
  PTR_LOOP_1050_48aa = (undefined *)0x4488;
  PTR_LOOP_1050_48ac = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48b0 = (undefined *)0x4488;
  PTR_LOOP_1050_48b2 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48ae = PTR_LOOP_1050_448c;
  PTR_LOOP_1050_48b4 = PTR_LOOP_1050_448c;
  PTR_LOOP_1050_48b6 = (undefined *)0x4488;
  PTR_LOOP_1050_48b8 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48ba = PTR_LOOP_1050_448c;
  PTR_LOOP_1050_48bc = (undefined *)0x446a;
  PTR_LOOP_1050_48be = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48c2 = (undefined *)0x446a;
  PTR_LOOP_1050_48c4 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48c0 = PTR_LOOP_1050_446e;
  PTR_LOOP_1050_48c6 = PTR_LOOP_1050_446e;
  PTR_LOOP_1050_48c8 = (undefined *)0x446a;
  PTR_LOOP_1050_48ca = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48cc = PTR_LOOP_1050_446e;
  PTR_LOOP_1050_48ce = (undefined *)0x447a;
  PTR_LOOP_1050_48d0 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48d4 = (undefined *)0x447a;
  PTR_LOOP_1050_48d6 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48d2 = DAT_1050_4480;
  PTR_LOOP_1050_48d8 = DAT_1050_4480;
  PTR_LOOP_1050_48da = (undefined *)0x4436;
  PTR_LOOP_1050_48dc = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48de = PTR_LOOP_1050_443a;
  PTR_LOOP_1050_48e0 = (undefined *)0x4436;
  PTR_LOOP_1050_48e2 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48e4 = PTR_LOOP_1050_443a;
  PTR_LOOP_1050_48e6 = (undefined *)0x447a;
  PTR_LOOP_1050_48e8 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48ea = DAT_1050_4480;
  _PTR_LOOP_1050_48ec = 0x0;
  _PTR_LOOP_1050_48f2 = 0x0;
  PTR_LOOP_1050_48f8 = (undefined *)0x447a;
  PTR_LOOP_1050_48fa = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_48fc = DAT_1050_4480;
  PTR_LOOP_1050_48fe = (undefined *)0x447a;
  PTR_LOOP_1050_4900 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4902 = DAT_1050_4480;
  _PTR_LOOP_1050_4904 = 0x0;
  _PTR_LOOP_1050_490a = 0x0;
  PTR_LOOP_1050_485a = (undefined *)0xffff;
  PTR_LOOP_1050_48f0 = (undefined *)0xffff;
  PTR_LOOP_1050_48f6 = (undefined *)0xffff;
  PTR_LOOP_1050_4908 = (undefined *)0xffff;
  PTR_LOOP_1050_490e = (undefined *)0xffff;
  _PTR_LOOP_1050_4910 = 0x0;
  _PTR_LOOP_1050_4916 = 0x0;
  PTR_LOOP_1050_4854 = (undefined *)0x0;
  PTR_LOOP_1050_4878 = (undefined *)0x0;
  PTR_LOOP_1050_487e = (undefined *)0x0;
  PTR_LOOP_1050_48a8 = (undefined *)0x0;
  PTR_LOOP_1050_4914 = (undefined *)0x0;
  PTR_LOOP_1050_491a = (undefined *)0x0;
  PTR_LOOP_1050_491c = (undefined *)0x4488;
  PTR_LOOP_1050_491e = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4920 = PTR_LOOP_1050_448c;
  PTR_LOOP_1050_4922 = (undefined *)0x4488;
  PTR_LOOP_1050_4924 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4926 = PTR_LOOP_1050_448c;
  _PTR_LOOP_1050_4928 = 0x0;
  _PTR_LOOP_1050_492e = 0x0;
  _PTR_LOOP_1050_4934 = 0x0;
  PTR_LOOP_1050_493a = (undefined *)0x446a;
  PTR_LOOP_1050_493c = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4940 = (undefined *)0x446a;
  PTR_LOOP_1050_4942 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_493e = PTR_LOOP_1050_446e;
  PTR_LOOP_1050_4944 = PTR_LOOP_1050_446e;
  _PTR_LOOP_1050_4946 = 0x0;
  _PTR_LOOP_1050_494c = 0x0;
  _PTR_LOOP_1050_4952 = 0x0;
  _PTR_LOOP_1050_4958 = 0x0;
  _PTR_LOOP_1050_495e = 0x0;
  _PTR_LOOP_1050_4964 = 0x0;
  _PTR_LOOP_1050_496a = 0x0;
  _PTR_LOOP_1050_4970 = 0x0;
  _PTR_LOOP_1050_4976 = 0x0;
  _PTR_LOOP_1050_497c = 0x0;
  _PTR_LOOP_1050_4982 = 0x0;
  _PTR_LOOP_1050_4988 = 0x0;
  _PTR_LOOP_1050_498e = 0x0;
  _PTR_LOOP_1050_4994 = 0x0;
  PTR_LOOP_1050_499a = (undefined *)0x4448;
  PTR_LOOP_1050_499c = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49a0 = (undefined *)0x4448;
  PTR_LOOP_1050_49a2 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_499e = DAT_1050_4452;
  PTR_LOOP_1050_49a4 = DAT_1050_4452;
  PTR_LOOP_1050_49a6 = (undefined *)0x4448;
  PTR_LOOP_1050_49a8 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49aa = DAT_1050_4452;
  PTR_LOOP_1050_49ac = (undefined *)0x4470;
  PTR_LOOP_1050_49ae = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49b2 = (undefined *)0x4470;
  PTR_LOOP_1050_49b4 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49b0 = PTR_DAT_1050_0004_1050_4478;
  PTR_LOOP_1050_49b6 = PTR_DAT_1050_0004_1050_4478;
  PTR_LOOP_1050_49b8 = (undefined *)0x4470;
  PTR_LOOP_1050_49ba = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49bc = PTR_DAT_1050_0004_1050_4478;
  PTR_LOOP_1050_49be = (undefined *)0x4470;
  PTR_LOOP_1050_49c0 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49c2 = PTR_DAT_1050_0004_1050_4478;
  PTR_LOOP_1050_49c4 = (undefined *)0x4470;
  PTR_LOOP_1050_49c6 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49c8 = PTR_DAT_1050_0004_1050_4478;
  PTR_LOOP_1050_49ca = (undefined *)0x4448;
  PTR_LOOP_1050_49cc = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49ce = DAT_1050_4452;
  PTR_LOOP_1050_49d0 = (undefined *)0x4448;
  PTR_LOOP_1050_49d2 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49d4 = DAT_1050_4452;
  PTR_LOOP_1050_49d6 = (undefined *)0x4448;
  PTR_LOOP_1050_49d8 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49da = DAT_1050_4452;
  PTR_LOOP_1050_49dc = (undefined *)0x4448;
  PTR_LOOP_1050_49de = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49e0 = DAT_1050_4452;
  PTR_LOOP_1050_49e2 = (undefined *)0x4482;
  PTR_LOOP_1050_49e4 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49e8 = (undefined *)0x4482;
  PTR_LOOP_1050_49ea = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49e6 = PTR_LOOP_1050_4486;
  PTR_LOOP_1050_49ec = PTR_LOOP_1050_4486;
  PTR_LOOP_1050_49ee = (undefined *)0x4470;
  PTR_LOOP_1050_49f0 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49f2 = PTR_DAT_1050_0004_1050_4478;
  PTR_LOOP_1050_49f4 = (undefined *)0x4470;
  PTR_LOOP_1050_49f6 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49f8 = PTR_DAT_1050_0004_1050_4478;
  PTR_LOOP_1050_49fa = (undefined *)0x4470;
  PTR_LOOP_1050_49fc = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_49fe = PTR_DAT_1050_0004_1050_4478;
  PTR_LOOP_1050_4a02 = (undefined *)0x0;
  PTR_LOOP_1050_4a00 = (undefined *)0x0;
  PTR_LOOP_1050_4a08 = (undefined *)0x0;
  PTR_LOOP_1050_4a06 = (undefined *)0x0;
  PTR_LOOP_1050_4a0e = (undefined *)0x0;
  PTR_LOOP_1050_4a0c = (undefined *)0x0;
  PTR_LOOP_1050_4a14 = (undefined *)0x0;
  PTR_LOOP_1050_4a12 = (undefined *)0x0;
  PTR_LOOP_1050_4a1a = (undefined *)0x0;
  PTR_LOOP_1050_4a18 = (undefined *)0x0;
  PTR_LOOP_1050_4a1e = (undefined *)0x4470;
  PTR_LOOP_1050_4a20 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4a22 = PTR_DAT_1050_0004_1050_4478;
  PTR_LOOP_1050_4a24 = (undefined *)0x4448;
  PTR_LOOP_1050_4a26 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4a28 = DAT_1050_4452;
  PTR_LOOP_1050_4a2a = (undefined *)0x4448;
  PTR_LOOP_1050_4a2c = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4a2e = DAT_1050_4452;
  _PTR_LOOP_1050_4a30 = 0x0;
  _PTR_LOOP_1050_4a36 = 0x0;
  PTR_LOOP_1050_492c = (undefined *)0xffff;
  PTR_LOOP_1050_4932 = (undefined *)0xffff;
  PTR_LOOP_1050_4938 = (undefined *)0xffff;
  PTR_LOOP_1050_494a = (undefined *)0xffff;
  PTR_LOOP_1050_4950 = (undefined *)0xffff;
  PTR_LOOP_1050_4a34 = (undefined *)0xffff;
  PTR_LOOP_1050_4a3a = (undefined *)0xffff;
  _PTR_LOOP_1050_4a3c = 0x0;
  _PTR_LOOP_1050_4a42 = 0x0;
  PTR_LOOP_1050_4956 = (undefined *)0x0;
  PTR_LOOP_1050_495c = (undefined *)0x0;
  PTR_LOOP_1050_4962 = (undefined *)0x0;
  PTR_LOOP_1050_4968 = (undefined *)0x0;
  PTR_LOOP_1050_496e = (undefined *)0x0;
  PTR_LOOP_1050_4974 = (undefined *)0x0;
  PTR_LOOP_1050_497a = (undefined *)0x0;
  PTR_LOOP_1050_4980 = (undefined *)0x0;
  PTR_LOOP_1050_4986 = (undefined *)0x0;
  PTR_LOOP_1050_498c = (undefined *)0x0;
  PTR_LOOP_1050_4992 = (undefined *)0x0;
  PTR_LOOP_1050_4998 = (undefined *)0x0;
  PTR_LOOP_1050_4a04 = (undefined *)0x0;
  PTR_LOOP_1050_4a0a = (undefined *)0x0;
  PTR_LOOP_1050_4a10 = (undefined *)0x0;
  PTR_LOOP_1050_4a16 = (undefined *)0x0;
  PTR_LOOP_1050_4a1c = (undefined *)0x0;
  PTR_LOOP_1050_4a40 = (undefined *)0x0;
  PTR_LOOP_1050_4a46 = (undefined *)0x0;
  PTR_LOOP_1050_4a48 = (undefined *)0x4448;
  PTR_LOOP_1050_4a4a = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4a4c = DAT_1050_4452;
  PTR_LOOP_1050_4a4e = (undefined *)0x4448;
  PTR_LOOP_1050_4a50 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4a52 = DAT_1050_4452;
  _PTR_LOOP_1050_4a54 = 0x0;
  _PTR_LOOP_1050_4a5a = 0x0;
  _PTR_LOOP_1050_4a60 = 0x0;
  _PTR_LOOP_1050_4a66 = 0x0;
  _PTR_LOOP_1050_4a6c = 0x0;
  _PTR_LOOP_1050_4a72 = 0x0;
  _PTR_LOOP_1050_4a78 = 0x0;
  _PTR_LOOP_1050_4a7e = 0x0;
  _PTR_LOOP_1050_4a84 = 0x0;
  _PTR_LOOP_1050_4a8a = 0x0;
  _PTR_LOOP_1050_4a90 = 0x0;
  _PTR_LOOP_1050_4a96 = 0x0;
  _PTR_LOOP_1050_4a9c = 0x0;
  _PTR_LOOP_1050_4aa2 = 0x0;
  _PTR_LOOP_1050_4aa8 = 0x0;
  _PTR_LOOP_1050_4aae = 0x0;
  _PTR_LOOP_1050_4ab4 = 0x0;
  _PTR_LOOP_1050_4aba = 0x0;
  _PTR_LOOP_1050_4ac0 = 0x0;
  _PTR_LOOP_1050_4ac6 = 0x0;
  _PTR_LOOP_1050_4acc = 0x0;
  _PTR_LOOP_1050_4ad2 = 0x0;
  _PTR_LOOP_1050_4ad8 = 0x0;
  _PTR_LOOP_1050_4ade = 0x0;
  _PTR_LOOP_1050_4ae4 = 0x0;
  _PTR_LOOP_1050_4aea = 0x0;
  _PTR_LOOP_1050_4af0 = 0x0;
  PTR_LOOP_1050_4a58 = (undefined *)0x0;
  PTR_LOOP_1050_4a5e = (undefined *)0x0;
  PTR_LOOP_1050_4a64 = (undefined *)0x0;
  PTR_LOOP_1050_4a6a = (undefined *)0x0;
  PTR_LOOP_1050_4a70 = (undefined *)0x0;
  PTR_LOOP_1050_4a76 = (undefined *)0x0;
  PTR_LOOP_1050_4a7c = (undefined *)0x0;
  PTR_LOOP_1050_4a82 = (undefined *)0x0;
  PTR_LOOP_1050_4a88 = (undefined *)0x0;
  PTR_LOOP_1050_4a8e = (undefined *)0x0;
  PTR_LOOP_1050_4a94 = (undefined *)0x0;
  PTR_LOOP_1050_4a9a = (undefined *)0x0;
  PTR_LOOP_1050_4aa0 = (undefined *)0x0;
  PTR_LOOP_1050_4aa6 = (undefined *)0x0;
  PTR_LOOP_1050_4aac = (undefined *)0x0;
  PTR_LOOP_1050_4ab2 = (undefined *)0x0;
  PTR_LOOP_1050_4ab8 = (undefined *)0x0;
  PTR_LOOP_1050_4abe = (undefined *)0x0;
  PTR_LOOP_1050_4ac4 = (undefined *)0x0;
  PTR_LOOP_1050_4aca = (undefined *)0x0;
  PTR_LOOP_1050_4ad0 = (undefined *)0x0;
  PTR_LOOP_1050_4ad6 = (undefined *)0x0;
  PTR_LOOP_1050_4adc = (undefined *)0x0;
  PTR_LOOP_1050_4ae2 = (undefined *)0x0;
  PTR_LOOP_1050_4ae8 = (undefined *)0x0;
  PTR_LOOP_1050_4aee = (undefined *)0x0;
  PTR_LOOP_1050_4af4 = (undefined *)0x0;
  puVar3 = (undefined2 *)0x4af6;
  for (iVar2 = 0x1b; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar1 = 0x0;
  }
  PTR_LOOP_1050_4b9c = PTR_LOOP_1050_4434;
  _PTR_LOOP_1050_4b9e = 0x0;
  _PTR_LOOP_1050_4ba4 = 0x0;
  _PTR_LOOP_1050_4baa = 0x0;
  PTR_LOOP_1050_4ba2 = (undefined *)0xffff;
  PTR_LOOP_1050_4ba8 = (undefined *)0xffff;
  PTR_LOOP_1050_4bae = (undefined *)0xffff;
  _PTR_LOOP_1050_4bb0 = 0x0;
  _PTR_LOOP_1050_4bb6 = 0x0;
  PTR_LOOP_1050_4bbc = (undefined *)0x448e;
  PTR_LOOP_1050_4bbe = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4bc2 = (undefined *)0x448e;
  PTR_LOOP_1050_4bc4 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4bc0 = DAT_1050_4494;
  PTR_LOOP_1050_4bc6 = DAT_1050_4494;
  PTR_LOOP_1050_4bc8 = (undefined *)0x448e;
  PTR_LOOP_1050_4bca = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4bcc = DAT_1050_4494;
  PTR_LOOP_1050_4bce = (undefined *)0x4482;
  PTR_LOOP_1050_4bd0 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4bd4 = (undefined *)0x4482;
  PTR_LOOP_1050_4bd6 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4bd2 = PTR_LOOP_1050_4486;
  PTR_LOOP_1050_4bd8 = PTR_LOOP_1050_4486;
  PTR_LOOP_1050_4bda = (undefined *)0x4482;
  PTR_LOOP_1050_4bdc = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4bde = PTR_LOOP_1050_4486;
  PTR_LOOP_1050_4be2 = (undefined *)0x0;
  PTR_LOOP_1050_4be0 = (undefined *)0x0;
  PTR_LOOP_1050_4bb4 = (undefined *)0x0;
  PTR_LOOP_1050_4bba = (undefined *)0x0;
  PTR_LOOP_1050_4be4 = (undefined *)0x0;
  PTR_LOOP_1050_4be6 = (undefined *)0x44ac;
  PTR_LOOP_1050_4be8 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4bec = (undefined *)0x44ac;
  PTR_LOOP_1050_4bee = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4bea = DAT_1050_44b2;
  PTR_LOOP_1050_4bf0 = DAT_1050_44b2;
  PTR_LOOP_1050_4bf2 = (undefined *)0x44ac;
  PTR_LOOP_1050_4bf4 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4bf6 = DAT_1050_44b2;
  PTR_LOOP_1050_4bf8 = (undefined *)0x446a;
  PTR_LOOP_1050_4bfa = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4bfe = (undefined *)0x446a;
  PTR_LOOP_1050_4c00 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4bfc = PTR_LOOP_1050_446e;
  PTR_LOOP_1050_4c02 = PTR_LOOP_1050_446e;
  PTR_LOOP_1050_4c04 = (undefined *)0x446a;
  PTR_LOOP_1050_4c06 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4c08 = PTR_LOOP_1050_446e;
  PTR_LOOP_1050_4c0a = (undefined *)0x448e;
  PTR_LOOP_1050_4c0c = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4c0e = DAT_1050_4494;
  PTR_LOOP_1050_4c10 = (undefined *)0x448e;
  PTR_LOOP_1050_4c12 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4c14 = DAT_1050_4494;
  PTR_LOOP_1050_4c16 = (undefined *)0x44ac;
  PTR_LOOP_1050_4c18 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4c1a = DAT_1050_44b2;
  PTR_LOOP_1050_4c22 = (undefined *)0x448e;
  PTR_LOOP_1050_4c24 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4c26 = DAT_1050_4494;
  _PTR_LOOP_1050_4c28 = 0x0;
  _PTR_LOOP_1050_4c2e = 0x0;
  _PTR_LOOP_1050_4c34 = 0x0;
  _PTR_LOOP_1050_4c3a = 0x0;
  _PTR_LOOP_1050_4c40 = 0x0;
  _PTR_LOOP_1050_4c46 = 0x0;
  _PTR_LOOP_1050_4c4c = 0x0;
  _PTR_LOOP_1050_4c52 = 0x0;
  PTR_LOOP_1050_4c1c = (undefined *)0x44ac;
  PTR_LOOP_1050_4c1e = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4c58 = (undefined *)0x44ac;
  PTR_LOOP_1050_4c5a = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4c20 = DAT_1050_44b2;
  PTR_LOOP_1050_4c5c = DAT_1050_44b2;
  PTR_LOOP_1050_4c5e = (undefined *)0x44ac;
  PTR_LOOP_1050_4c60 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4c62 = DAT_1050_44b2;
  _PTR_LOOP_1050_4c64 = 0x0;
  _PTR_LOOP_1050_4c6a = 0x0;
  _PTR_LOOP_1050_4c70 = 0x0;
  PTR_LOOP_1050_4c76 = (undefined *)0x446a;
  PTR_LOOP_1050_4c78 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4c7c = (undefined *)0x446a;
  PTR_LOOP_1050_4c7e = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4c7a = PTR_LOOP_1050_446e;
  PTR_LOOP_1050_4c80 = PTR_LOOP_1050_446e;
  _PTR_LOOP_1050_4c82 = 0x0;
  _PTR_LOOP_1050_4c88 = 0x0;
  PTR_LOOP_1050_4c2c = (undefined *)0xffff;
  PTR_LOOP_1050_4c32 = (undefined *)0xffff;
  PTR_LOOP_1050_4c38 = (undefined *)0xffff;
  PTR_LOOP_1050_4c3e = (undefined *)0xffff;
  PTR_LOOP_1050_4c44 = (undefined *)0xffff;
  PTR_LOOP_1050_4c4a = (undefined *)0xffff;
  PTR_LOOP_1050_4c68 = (undefined *)0xffff;
  PTR_LOOP_1050_4c6e = (undefined *)0xffff;
  PTR_LOOP_1050_4c74 = (undefined *)0xffff;
  PTR_LOOP_1050_4c86 = (undefined *)0xffff;
  PTR_LOOP_1050_4c8c = (undefined *)0xffff;
  _PTR_LOOP_1050_4c8e = 0x0;
  _PTR_LOOP_1050_4c94 = 0x0;
  _PTR_LOOP_1050_4c9a = 0x0;
  _PTR_LOOP_1050_4ca0 = 0x0;
  _PTR_LOOP_1050_4ca6 = 0x0;
  _PTR_LOOP_1050_4cac = 0x0;
  _PTR_LOOP_1050_4cb2 = 0x0;
  _PTR_LOOP_1050_4cb8 = 0x0;
  _PTR_LOOP_1050_4cbe = 0x0;
  _PTR_LOOP_1050_4cc4 = 0x0;
  _PTR_LOOP_1050_4cca = 0x0;
  _PTR_LOOP_1050_4cd0 = 0x0;
  PTR_LOOP_1050_4cd6 = (undefined *)0x4496;
  PTR_LOOP_1050_4cd8 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4cdc = (undefined *)0x4496;
  PTR_LOOP_1050_4cde = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4cda = DAT_1050_44a2;
  PTR_LOOP_1050_4ce0 = DAT_1050_44a2;
  PTR_LOOP_1050_4ce2 = (undefined *)0x4496;
  PTR_LOOP_1050_4ce4 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4ce6 = DAT_1050_44a2;
  PTR_LOOP_1050_4ce8 = (undefined *)0x4496;
  PTR_LOOP_1050_4cea = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4cec = DAT_1050_44a2;
  PTR_LOOP_1050_4cee = (undefined *)0x4496;
  PTR_LOOP_1050_4cf0 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4cf2 = DAT_1050_44a2;
  PTR_LOOP_1050_4cf4 = (undefined *)0x44a4;
  PTR_LOOP_1050_4cf6 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4cfa = (undefined *)0x44a4;
  PTR_LOOP_1050_4cfc = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4cf8 = DAT_1050_44aa;
  PTR_LOOP_1050_4cfe = DAT_1050_44aa;
  PTR_LOOP_1050_4d00 = (undefined *)0x44a4;
  PTR_LOOP_1050_4d02 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d04 = DAT_1050_44aa;
  PTR_LOOP_1050_4d06 = (undefined *)0x4496;
  PTR_LOOP_1050_4d08 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d0a = DAT_1050_44a2;
  PTR_LOOP_1050_4d0c = (undefined *)0x4496;
  PTR_LOOP_1050_4d0e = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d10 = DAT_1050_44a2;
  PTR_LOOP_1050_4d12 = (undefined *)0x4496;
  PTR_LOOP_1050_4d14 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d16 = DAT_1050_44a2;
  PTR_LOOP_1050_4d18 = (undefined *)0x4496;
  PTR_LOOP_1050_4d1a = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d1c = DAT_1050_44a2;
  PTR_LOOP_1050_4d1e = (undefined *)0x4482;
  PTR_LOOP_1050_4d20 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d24 = (undefined *)0x4482;
  PTR_LOOP_1050_4d26 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d22 = PTR_LOOP_1050_4486;
  PTR_LOOP_1050_4d28 = PTR_LOOP_1050_4486;
  PTR_LOOP_1050_4d2a = (undefined *)0x44a4;
  PTR_LOOP_1050_4d2c = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d2e = DAT_1050_44aa;
  PTR_LOOP_1050_4d30 = (undefined *)0x44a4;
  PTR_LOOP_1050_4d32 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d34 = DAT_1050_44aa;
  PTR_LOOP_1050_4d36 = (undefined *)0x44a4;
  PTR_LOOP_1050_4d38 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d3a = DAT_1050_44aa;
  _PTR_LOOP_1050_4d3c = 0x0;
  _PTR_LOOP_1050_4d42 = 0x0;
  PTR_LOOP_1050_4c50 = (undefined *)0x0;
  PTR_LOOP_1050_4c56 = (undefined *)0x0;
  PTR_LOOP_1050_4c92 = (undefined *)0x0;
  PTR_LOOP_1050_4c98 = (undefined *)0x0;
  PTR_LOOP_1050_4c9e = (undefined *)0x0;
  PTR_LOOP_1050_4ca4 = (undefined *)0x0;
  PTR_LOOP_1050_4caa = (undefined *)0x0;
  PTR_LOOP_1050_4cb0 = (undefined *)0x0;
  PTR_LOOP_1050_4cb6 = (undefined *)0x0;
  PTR_LOOP_1050_4cbc = (undefined *)0x0;
  PTR_LOOP_1050_4cc2 = (undefined *)0x0;
  PTR_LOOP_1050_4cc8 = (undefined *)0x0;
  PTR_LOOP_1050_4cce = (undefined *)0x0;
  PTR_LOOP_1050_4cd4 = (undefined *)0x0;
  PTR_LOOP_1050_4d40 = (undefined *)0x0;
  PTR_LOOP_1050_4d46 = (undefined *)0x0;
  _PTR_LOOP_1050_4d48 = 0x0;
  _PTR_LOOP_1050_4d4e = 0x0;
  _PTR_LOOP_1050_4d54 = 0x0;
  PTR_LOOP_1050_4d5a = (undefined *)0x44a4;
  PTR_LOOP_1050_4d5c = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d5e = DAT_1050_44aa;
  PTR_LOOP_1050_4d60 = (undefined *)0x4496;
  PTR_LOOP_1050_4d62 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d66 = (undefined *)0x4496;
  PTR_LOOP_1050_4d68 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d64 = DAT_1050_44a2;
  PTR_LOOP_1050_4d6a = DAT_1050_44a2;
  _PTR_LOOP_1050_4d6c = 0x0;
  _PTR_LOOP_1050_4d72 = 0x0;
  PTR_LOOP_1050_4d70 = (undefined *)0xffff;
  PTR_LOOP_1050_4d76 = (undefined *)0xffff;
  _PTR_LOOP_1050_4d78 = 0x0;
  _PTR_LOOP_1050_4d7e = 0x0;
  PTR_LOOP_1050_4d84 = (undefined *)0x4496;
  PTR_LOOP_1050_4d86 = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d88 = DAT_1050_44a2;
  PTR_LOOP_1050_4d8a = (undefined *)0x4496;
  PTR_LOOP_1050_4d8c = (undefined *)&USHORT_1050_1050;
  PTR_LOOP_1050_4d8e = DAT_1050_44a2;
  _PTR_LOOP_1050_4d90 = 0x0;
  _PTR_LOOP_1050_4d96 = 0x0;
  _PTR_LOOP_1050_4d9c = 0x0;
  _PTR_LOOP_1050_4da2 = 0x0;
  _PTR_LOOP_1050_4da8 = 0x0;
  _PTR_LOOP_1050_4dae = 0x0;
  _PTR_LOOP_1050_4db4 = 0x0;
  _PTR_LOOP_1050_4dba = 0x0;
  _PTR_LOOP_1050_4dc0 = 0x0;
  _PTR_LOOP_1050_4dc6 = 0x0;
  _PTR_LOOP_1050_4dcc = 0x0;
  _PTR_LOOP_1050_4dd2 = 0x0;
  _PTR_LOOP_1050_4dd8 = 0x0;
  _PTR_LOOP_1050_4dde = 0x0;
  _PTR_LOOP_1050_4de4 = 0x0;
  _PTR_LOOP_1050_4dea = 0x0;
  _PTR_LOOP_1050_4df0 = 0x0;
  _PTR_LOOP_1050_4df6 = 0x0;
  _PTR_LOOP_1050_4dfc = 0x0;
  _PTR_LOOP_1050_4e02 = 0x0;
  _PTR_LOOP_1050_4e08 = 0x0;
  _PTR_LOOP_1050_4e0e = 0x0;
  _PTR_LOOP_1050_4e14 = 0x0;
  _PTR_LOOP_1050_4e1a = 0x0;
  _PTR_LOOP_1050_4e20 = 0x0;
  _PTR_LOOP_1050_4e26 = 0x0;
  _PTR_LOOP_1050_4e2c = 0x0;
  PTR_LOOP_1050_4d4c = (undefined *)0x0;
  PTR_LOOP_1050_4d52 = (undefined *)0x0;
  PTR_LOOP_1050_4d58 = (undefined *)0x0;
  PTR_LOOP_1050_4d7c = (undefined *)0x0;
  PTR_LOOP_1050_4d82 = (undefined *)0x0;
  PTR_LOOP_1050_4d94 = (undefined *)0x0;
  PTR_LOOP_1050_4d9a = (undefined *)0x0;
  PTR_LOOP_1050_4da0 = (undefined *)0x0;
  PTR_LOOP_1050_4da6 = (undefined *)0x0;
  PTR_LOOP_1050_4dac = (undefined *)0x0;
  PTR_LOOP_1050_4db2 = (undefined *)0x0;
  PTR_LOOP_1050_4db8 = (undefined *)0x0;
  PTR_LOOP_1050_4dbe = (undefined *)0x0;
  PTR_LOOP_1050_4dc4 = (undefined *)0x0;
  PTR_LOOP_1050_4dca = (undefined *)0x0;
  PTR_LOOP_1050_4dd0 = (undefined *)0x0;
  PTR_LOOP_1050_4dd6 = (undefined *)0x0;
  PTR_LOOP_1050_4ddc = (undefined *)0x0;
  PTR_LOOP_1050_4de2 = (undefined *)0x0;
  PTR_LOOP_1050_4de8 = (undefined *)0x0;
  PTR_LOOP_1050_4dee = (undefined *)0x0;
  PTR_LOOP_1050_4df4 = (undefined *)0x0;
  PTR_LOOP_1050_4dfa = (undefined *)0x0;
  PTR_LOOP_1050_4e00 = (undefined *)0x0;
  PTR_LOOP_1050_4e06 = (undefined *)0x0;
  PTR_LOOP_1050_4e0c = (undefined *)0x0;
  PTR_LOOP_1050_4e12 = (undefined *)0x0;
  PTR_LOOP_1050_4e18 = (undefined *)0x0;
  PTR_LOOP_1050_4e1e = (undefined *)0x0;
  PTR_LOOP_1050_4e24 = (undefined *)0x0;
  PTR_LOOP_1050_4e2a = (undefined *)0x0;
  PTR_LOOP_1050_4e30 = (undefined *)0x0;
  puVar3 = (undefined2 *)0x4e32;
  for (iVar2 = 0x1b; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar1 = 0x0;
  }
  return;
}
