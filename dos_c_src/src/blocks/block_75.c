
void __stdcall16far pass1_1028_e332(ulong param_1,ushort param_2,uint param_3,ushort param_4)

{
  if ((param_3._1_1_ != 0x0) && (param_3._1_1_ < 0xa)) {
    pass1_1030_13f6(*(ulong *)((int)param_1 + 0xa + (uint)param_3._1_1_ * 0x4),CONCAT22(param_3,param_2) & 0xffffff,
                    param_2,param_3 & 0xff,param_4);
  }
  return;
}



void __stdcall16far pass1_1028_e372(ulong param_1,ushort param_2,uint param_3,ushort param_4)

{
  ulong uVar1;
  ulong uVar2;
  undefined4 uVar3;
  code **ppcVar4;
  ulong uVar5;
  ulong uVar6;
  uint uVar7;
  uint uVar8;
  ulong uStack32;
  ulong uStack16;
  uint uStack10;
  
  if (param_3 >> 0x8 != 0xff) {
    uVar1 = *(ulong *)((int)param_1 + 0xa + (param_3 >> 0x8) * 0x4);
    uVar2 = *(ulong *)((int)uVar1 + 0xa);
    uVar7 = param_3 & 0xff;
    uStack16 = CONCAT22(param_3,param_2) & 0xffffff;
    pass1_1028_e1ec(param_1,param_2,param_3);
    uVar5 = *(ulong *)(param_2 + 0x8);
    pass1_1028_e1ec(param_1,(ushort)uVar5,(uint)(uVar5 >> 0x10));
    for (uStack32 = 0x1; uStack10 = (uint)(uVar2 >> 0x10), uStack32 < uVar2; uStack32 = uStack32 + 0x1) {
      if (uStack32 != uStack16) {
        uVar6 = uStack16;
        bad_1030_1312();
        uVar8 = uStack10 | (uint)uVar6;
        if (uVar8 != 0x0) {
          uVar3 = *(undefined4 *)((uint)uVar6 + 0x4);
          pass1_1030_13f6(uVar1,uStack32,(int)uVar3,uVar8,param_4);
          ppcVar4 = (code **)((int)*(undefined4 *)(uVar5 & 0xffff | (ulong)uVar7 << 0x10) + 0x18);
          (**ppcVar4)(0x1030,(int)(uVar5 & 0xffff),uVar7,uVar3);
        }
      }
    }
  }
  return;
}



void __stdcall16far pass1_1028_e44a(ulong param_1,long param_2,ushort param_3)

{
  ulong uVar1;
  ulong uVar2;
  ulong uVar3;
  ushort uVar4;
  uint uVar5;
  ulong uVar6;
  uint uVar7;
  undefined2 uVar8;
  ulong uStack18;
  uint uStack12;
  
  pass1_1028_e372(param_1,(ushort)param_2,(uint)((ulong)param_2 >> 0x10),param_3);
  uVar8 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(ulong *)((int)param_1 + 0x26);
  uVar2 = *(ulong *)((int)param_1 + 0x1e);
  uVar3 = *(ulong *)((int)uVar2 + 0xa);
  for (uStack18 = 0x1; uStack12 = (uint)(uVar3 >> 0x10), uStack18 < uVar3; uStack18 = uStack18 + 0x1) {
    uVar6 = uVar3;
    bad_1030_1312();
    uVar5 = (uint)uVar6;
    if (((uStack12 | uVar5) != 0x0) && (*(long *)(uVar5 + 0x8) != param_2)) {
      uVar8 = *(undefined2 *)(uVar5 + 0x16);
      uVar5 = *(uint *)(uVar5 + 0x18);
      uVar7 = uVar5 & 0xff;
      uVar4 = pass1_1030_13f6(uVar1,CONCAT22(uVar5,uVar8) & 0xffffff,uVar8,uVar7,param_3);
      pass1_1030_13f6(uVar2,uStack18,uVar4,uVar7,param_3);
    }
  }
  return;
}



void __stdcall16far pass1_1028_e4ec(ulong param_1)

{
  ulong *puVar1;
  long *plVar2;
  ulong uVar3;
  uint uVar4;
  undefined4 uVar5;
  uint in_DX;
  int iVar6;
  undefined2 uVar7;
  
  uVar5 = 0x0;
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*(int *)(iVar6 + 0x10) == 0x0) {
    do {
      if (*(long *)(iVar6 + 0x8) == 0x0) {
        return;
      }
      plVar2 = (long *)(iVar6 + 0x8);
      *plVar2 = *plVar2 + -0x1;
      bad_1030_1312();
      in_DX = in_DX | (uint)uVar5;
    } while (in_DX == 0x0);
  }
  else {
    do {
      uVar3 = *(ulong *)(iVar6 + 0xc);
      puVar1 = (ulong *)(iVar6 + 0x8);
      if (uVar3 <= *puVar1 && *puVar1 != uVar3) {
        return;
      }
      uVar4 = (uint)*(undefined4 *)(iVar6 + 0x8);
      plVar2 = (long *)(iVar6 + 0x8);
      *plVar2 = *plVar2 + 0x1;
      bad_1030_1312();
      in_DX = in_DX | uVar4;
    } while (in_DX == 0x0);
  }
  return;
}



uint16_t __stdcall16far
write_file_fn_1028_e56c(undefined2 param_1,undefined2 param_2,undefined4 param_3,uint16_t param_4)

{
  code **ppcVar1;
  undefined *puVar2;
  BOOL16 BVar3;
  uint16_t in_DX;
  uint16_t extraout_DX;
  undefined4 in_stack_0000000c;
  ulong local_2a [0x3];
  undefined4 *puStack28;
  ulong uStack24;
  undefined local_14 [0x8];
  undefined2 uStack12;
  uint16_t uStack10;
  undefined2 uStack8;
  uint16_t uStack6;
  int iStack4;
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_14),0x1,(ushort)in_stack_0000000c,
                  (uint)((ulong)in_stack_0000000c >> 0x10));
  uStack24 = 0x0;
  while( true ) {
    puVar2 = local_14;
    pass1_1028_e4ec(CONCAT22(param_4,puVar2));
    puStack28 = (undefined4 *)CONCAT22(in_DX,puVar2);
    in_DX = in_DX | (uint)puVar2;
    if (in_DX == 0x0) break;
    uStack24 = uStack24 + 0x1;
  }
  local_2a[0] = uStack24;
  BVar3 = write_to_file_1008_7e1c
                    ((ushort)param_3,(ushort)((ulong)param_3 >> 0x10),(ushort)local_2a,param_4,(char *)0x4,0x1008);
  if (BVar3 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  else {
    uStack12 = uStack8;
    uStack10 = uStack6;
    if (iStack4 != 0x0) {
      uStack12 = 0x1;
      uStack6 = 0x0;
      uStack10 = uStack6;
    }
    do {
      puVar2 = local_14;
      pass1_1028_e4ec(CONCAT22(param_4,puVar2));
      puStack28 = (undefined4 *)CONCAT22(uStack6,puVar2);
      if ((uStack6 | (uint)puVar2) == 0x0) {
        return 0x0;
      }
      ppcVar1 = (code **)((int)*puStack28 + 0xc);
      (**ppcVar1)(0x1008,puVar2,uStack6);
      local_2a[0] = local_2a[0] & 0xffff0000 | ZEXT24(puVar2);
      uStack6 = extraout_DX;
      in_DX = extraout_DX;
    } while (puVar2 != (undefined *)0x0);
  }
  return in_DX;
}



// WARNING: Instruction at (ram,0x10287af1) overlaps instruction at (ram,0x10287af0)
// 
// WARNING: Control flow encountered bad instruction data
// WARNING: Removing unreachable block (ram,0x1028e2f6)
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram

void __stdcall16far
pass1_1028_e628(ulong param_1,ushort param_2,ushort param_3,int param_4,int param_5,ushort param_6,ushort param_7,
               ushort param_8,ushort param_9,uchar param_10)

{
  char *pcVar1;
  int *piVar2;
  char cVar3;
  ulong uVar4;
  ulong uVar5;
  long lVar6;
  code **ppcVar7;
  undefined2 *puVar8;
  undefined2 *puVar9;
  undefined2 uVar10;
  BOOL16 BVar11;
  uint uVar12;
  undefined4 uVar13;
  int iVar14;
  undefined2 *extraout_DX;
  uint extraout_DX_00;
  uchar *extraout_DX_01;
  undefined2 uVar15;
  uint uVar16;
  undefined2 *puVar17;
  astruct_348 *uVar18;
  astruct_349 *paVar18;
  ushort uVar19;
  astruct_349 *uVar20;
  undefined2 uVar21;
  ushort uVar22;
  bool bVar23;
  bool bVar24;
  ushort *puVar25;
  astruct_99 *paVar26;
  undefined4 *puVar27;
  undefined2 local_154;
  undefined2 uStackY338;
  undefined2 local_14c;
  undefined2 uStackY330;
  undefined2 uStackY80;
  undefined2 uStackY78;
  undefined uVar28;
  undefined uVar29;
  undefined uVar30;
  undefined uVar31;
  undefined uVar32;
  undefined uVar33;
  undefined uVar34;
  undefined uVar35;
  undefined uVar36;
  ushort uVar37;
  undefined uVar38;
  undefined uVar39;
  int iVar40;
  undefined2 in_stack_0000ffca;
  undefined2 in_stack_0000ffcc;
  undefined4 local_30;
  uint uStack44;
  ushort uStack42;
  ushort uStack40;
  ushort uStack38;
  undefined4 *puStack36;
  undefined2 *puStack32;
  undefined2 *puStack30;
  uint uStack28;
  uint uStack26;
  undefined2 **ppuStack24;
  ushort local_16;
  undefined2 *local_14;
  int local_12;
  undefined2 *local_10;
  undefined2 *puStack14;
  code *pcStack12;
  ushort *puStack10;
  undefined4 *local_6;
  
  uVar21 = SUB42(&USHORT_1050_1050,0x0);
  uVar19 = param_6;
  uVar22 = param_7;
  BVar11 = read_file_1008_7dee(param_2,param_3,(ushort)&local_6,0x0,param_9,0x4,0x1008);
  if (BVar11 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
  puStack10 = (ushort *)0x0;
  if (((param_4 == 0x0) && ((char)(param_5 - 0x100U) == '\0')) &&
     (puVar17 = (undefined2 *)(param_5 - 0x100U >> 0x7), puVar17 < (undefined2 *)((int)&PTR_LOOP_1050_000e + 0x1))) {
    uVar37 = (ushort)(param_1 >> 0x10);
    uVar20 = (astruct_349 *)param_1;
    uVar34 = (undefined)(param_9 >> 0x8);
    switch(puVar17) {
    case (undefined2 *)0x0:
      pass1_1030_145a(uVar20->field_0xe,(long)local_6);
      uStack28 = 0x0;
      uStack26 = 0x0;
      while ((undefined4 *)CONCAT22(uStack26,uStack28) < local_6) {
        puVar27 = local_6;
        mem_op_1000_179c(0x14,(uchar *)puVar17,0x1000);
        puStack32 = (undefined2 *)(uint)puVar27;
        puStack30 = puVar17;
        if (((uint)puVar17 | (uint)puStack32) == 0x0) {
          puVar17 = (undefined2 *)0x0;
          local_16 = 0x0;
        }
        else {
          puVar25 = pass1_1030_5d0a((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10));
          local_16 = (ushort)((ulong)puVar25 >> 0x10);
          puVar17 = (undefined2 *)puVar25;
        }
        ppcVar7 = (code **)((int)*(undefined4 *)CONCAT22(local_16,puVar17) + 0x10);
        ppuStack24 = (undefined2 **)puVar17;
        (**ppcVar7)();
        if (puVar17 == (undefined2 *)0x0) {
          return;
        }
        uVar5 = *(ulong *)(ppuStack24 + 0x2);
        uVar16 = (uint)ppuStack24[0x3];
        puStack14 = (undefined2 *)uVar5;
        pcStack12 = (code *)(uVar5 >> 0x10);
        puVar17 = (undefined2 *)(uVar16 & 0xff);
        pass1_1030_14b4(uVar20->field_0xe,(ushort)ppuStack24,local_16,uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10,
                        param_9);
        lVar6 = CONCAT22(uStack26,uStack28) + 0x1;
        uStack28 = (uint)lVar6;
        uStack26 = (uint)((ulong)lVar6 >> 0x10);
      }
      break;
    case (undefined2 *)0x1:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case (undefined2 *)0x2:
      pass1_1030_145a(uVar20->field_0x12,(long)local_6);
      uStack40 = 0x0;
      uStack38 = 0x0;
      while ((undefined4 *)CONCAT22(uStack38,uStack40) < local_6) {
        puVar27 = local_6;
        mem_op_1000_179c(0x1c,(uchar *)puVar17,0x1000);
        puStack32 = (undefined2 *)(uint)puVar27;
        uVar16 = (uint)puVar17 | (uint)puStack32;
        puStack30 = puVar17;
        if (uVar16 == 0x0) {
          uVar12 = 0x0;
          uVar16 = 0x0;
        }
        else {
          uVar12 = (uint)puStack32;
          pass1_1030_2958((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10));
        }
        puStack36 = (undefined4 *)CONCAT22(uVar16,uVar12);
        ppcVar7 = (code **)((int)*puStack36 + 0x10);
        (**ppcVar7)();
        if (uVar12 == 0x0) {
          return;
        }
        uVar19 = (ushort)((ulong)puStack36 >> 0x10);
        uVar18 = (astruct_348 *)puStack36;
        uVar5 = *(ulong *)&uVar18->field_0x4;
        uVar16 = uVar18->field_0x6;
        puStack14 = (undefined2 *)uVar5;
        pcStack12 = (code *)(uVar5 >> 0x10);
        puVar17 = (undefined2 *)(uVar16 & 0xff);
        pass1_1030_14b4(uVar20->field_0x12,(ushort)uVar18,uVar19,uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10,param_9
                       );
        lVar6 = CONCAT22(uStack38,uStack40) + 0x1;
        uStack40 = (ushort)lVar6;
        uStack38 = (ushort)((ulong)lVar6 >> 0x10);
      }
      break;
    case (undefined2 *)0x3:
      uStackY78 = SUB42(&USHORT_1050_1028,0x0);
      uStackY80 = 0x970b;
      uVar19 = &uVar20->field_0x114;
      pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x500);
      uStackY78 = 0x9728;
      local_16 = uVar19;
      local_14 = puVar17;
      pass1_1030_61fe(_PTR_LOOP_1050_5740,CONCAT22(puVar17,uVar19),
                      param_1 & 0xffff0000 | (ulong)(uint)&uVar20->field_0x114,*(long *)&uVar20->field_0x108,uVar19,
                      (ushort)puVar17,param_9);
      if ((uVar20->field_0x11a == 0xa) || (uVar20->field_0x11a == 0x37)) {
        if (uVar20->field_0x11a == 0x37) {
          puVar17 = *(undefined2 **)((int)&uVar20->field_0x11e + 0x2);
          uVar5 = uVar20->field_0x10c;
          uStack42 = (ushort)uVar5;
          uStack40 = (ushort)(uVar5 >> 0x10);
          *(ulong *)((int)uVar20->field_0x11e + 0x20) = uVar5;
        }
        uVar19 = &uVar20->field_0x114;
        uStackY78 = 0x1030;
        uStackY80 = 0x9788;
        pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x400);
        *(ushort *)&uVar20->field_0x10c = uVar19;
        *(undefined2 **)((int)&uVar20->field_0x10c + 0x2) = puVar17;
        pass1_1018_0196((ulong)local_6,CONCAT22(puVar17,*(undefined2 *)&uVar20->field_0x10c),
                        *(ulong *)&uVar20->field_0x108,uVar19,(uchar *)puVar17,param_9);
        if (uVar20->field_0x11a == 0xa) {
          pass1_1010_ed22((ulong)local_6,uVar20->field_0x10c,param_9);
        }
      }
      uVar5 = uVar20->field_0x10c;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar5,(uint)(uVar5 >> 0x10));
      *(ushort *)&uVar20->field_0x110 = uVar19;
      *(undefined2 **)((int)&uVar20->field_0x110 + 0x2) = puVar17;
      uStack26 = (uint)puVar17 | *(uint *)&uVar20->field_0x110;
      if (uStack26 != 0x0) {
        ppcVar7 = (code **)((int)*uVar20->field_0x110 + 0x8);
        (**ppcVar7)();
        puVar17 = extraout_DX;
      }
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,local_16,(uint)local_14);
      ppuStack24 = (undefined2 **)puVar17;
      pass1_1030_73ee(CONCAT22(puVar17,uStack26),uVar20->field_0x10c,(ushort)puVar17);
      BVar11 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar20->field_0x11a,0x31);
      puStack32 = puVar17;
      if ((BVar11 == 0x0) && (uVar20->field_0x122 == 0x0)) {
        uVar21 = (undefined2)((ulong)*(undefined4 *)(uStack26 + 0xc) >> 0x10);
        if (*(int *)(uStack26 + 0x10) < 0x1) {
          uVar10 = 0x5;
        }
        else {
          uVar10 = 0x6;
        }
        *(undefined2 *)(uStack26 + 0x14) = uVar10;
        puStack32 = ppuStack24;
      }
      uVar13 = *(undefined4 *)(uStack26 + 0x16);
      puStack30 = (undefined2 *)uVar13;
      uStack28 = (uint)((ulong)uVar13 >> 0x10);
      pass1_1028_e1ec(*(ulong *)&PTR_LOOP_1050_65e2,(ushort)puStack30,uStack28);
      puStack36 = (undefined4 *)CONCAT22((int)uVar13,puStack36._0_2_);
      if (CONCAT22(uStack28,puStack30) != 0x0) {
        struct_1030_e4fa((astruct_100 *)CONCAT22(param_9,&local_14c),CONCAT22(uStack28,puStack30),param_9,param_10);
        fn_ptr_1030_835a((ulong **)*(ulong **)&PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_9,&local_14c));
        local_14c = 0x389a;
        uStackY330 = 0x1008;
      }
      ppcVar7 = (code **)((int)*uVar20->field_0x11e + 0x4);
      (**ppcVar7)();
      puVar27 = uVar20->field_0x11e;
      uStackY78 = 0x9902;
      pass1_1030_7e5a(CONCAT13((char)((uint)ppuStack24 >> 0x8),CONCAT12((char)ppuStack24,uStack26)),
                      *(ulong *)((int)puVar27 + 0x4),extraout_DX_00);
      return;
    case (undefined2 *)0x4:
      pass1_1030_145a(uVar20->field_0x16,(long)local_6);
      uStack40 = 0x0;
      uStack38 = 0x0;
      while ((undefined4 *)CONCAT22(uStack38,uStack40) < local_6) {
        puVar27 = local_6;
        mem_op_1000_179c(0x1e,(uchar *)puVar17,0x1000);
        puStack32 = (undefined2 *)(uint)puVar27;
        puStack30 = puVar17;
        if (((uint)puVar17 | (uint)puStack32) == 0x0) {
          iVar14 = 0x0;
          uVar21 = 0x0;
        }
        else {
          puVar25 = pass1_1030_560e((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10));
          uVar21 = (undefined2)((ulong)puVar25 >> 0x10);
          iVar14 = (int)puVar25;
        }
        puStack36 = (undefined4 *)CONCAT22(uVar21,iVar14);
        ppcVar7 = (code **)((int)*puStack36 + 0x10);
        (**ppcVar7)();
        if (iVar14 == 0x0) {
          return;
        }
        uVar21 = (undefined2)((ulong)puStack36 >> 0x10);
        uVar5 = *(ulong *)((int)puStack36 + 0x4);
        puStack14 = (undefined2 *)uVar5;
        pcStack12 = (code *)(uVar5 >> 0x10);
        uVar4 = *(ulong *)((int)puStack36 + 0x10);
        uStack28 = (uint)uVar4;
        uStack26 = (uint)(uVar4 >> 0x10);
        pass1_1030_6222(_PTR_LOOP_1050_5740,0x0,uVar4,uVar5,uStack28,extraout_DX_01,param_9);
        puVar17 = (undefined2 *)((uint)pcStack12 & 0xff);
        pass1_1030_14b4(uVar20->field_0x16,(ushort)puStack36,(ushort)((ulong)puStack36 >> 0x10),
                        CONCAT22(pcStack12,puStack14) & 0xffffff,param_9);
        lVar6 = CONCAT22(uStack38,uStack40) + 0x1;
        uStack40 = (ushort)lVar6;
        uStack38 = (ushort)((ulong)lVar6 >> 0x10);
      }
      break;
    case (undefined2 *)0x5:
      *puVar17 = 0x5280;
      puVar17[0x1] = (int)&USHORT_1050_1028;
      return;
    case (undefined2 *)0x6:
      pass1_1030_145a(uVar20->field_0x1a,(long)local_6);
      for (local_30 = (undefined4 *)0x0; local_30 < local_6; local_30 = (undefined4 *)((long)local_30 + 0x1)) {
        puVar27 = local_6;
        mem_op_1000_179c(0x21e,(uchar *)puVar17,0x1000);
        puStack32 = (undefined2 *)(uint)puVar27;
        uVar16 = (uint)puVar17 | (uint)puStack32;
        puStack30 = puVar17;
        if (uVar16 == 0x0) {
          uVar12 = 0x0;
          uVar16 = 0x0;
        }
        else {
          uVar12 = (uint)puStack32;
          pass1_1038_30aa((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10),param_9);
        }
        ppcVar7 = (code **)((int)*(undefined4 *)CONCAT22(uVar16,uVar12) + 0x10);
        uStack44 = uVar12;
        uStack42 = uVar16;
        (**ppcVar7)();
        if (uVar12 == 0x0) {
          return;
        }
        uVar5 = *(ulong *)(uStack44 + 0x4);
        uVar16 = *(uint *)(uStack44 + 0x6);
        puStack14 = (undefined2 *)uVar5;
        pcStack12 = (code *)(uVar5 >> 0x10);
        puVar17 = (undefined2 *)(uVar16 & 0xff);
        pass1_1030_14b4(uVar20->field_0x1a,uStack44,uStack42,uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10,param_9);
      }
      break;
    default:
      pass1_1030_145a(uVar20->field_0x1e,(long)local_6);
      pass1_1030_66de(_PTR_LOOP_1050_5740,(ulong)local_6,param_9);
      local_30 = (undefined4 *)0x0;
      while( true ) {
        if (local_6 <= local_30) {
          pass1_1030_154c();
          pass1_1030_6740(_PTR_LOOP_1050_5740,param_9,param_7);
          return;
        }
        local_14 = (undefined2 *)_PTR_LOOP_1050_5744;
        local_12 = (int)(_PTR_LOOP_1050_5744 >> 0x10);
        paVar26 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5744);
        puStack30 = (undefined2 *)((ulong)paVar26 >> 0x10);
        puStack32 = (undefined2 *)(uint)paVar26;
        uVar16 = (uint)puStack30 | (uint)puStack32;
        if (uVar16 == 0x0) {
          uVar12 = 0x0;
          uVar16 = 0x0;
        }
        else {
          uVar12 = (uint)puStack32;
          pass1_1030_67cc(&paVar26->field_0x0);
        }
        ppcVar7 = (code **)((int)*(undefined4 *)CONCAT22(uVar16,uVar12) + 0x10);
        uStack44 = uVar12;
        uStack42 = uVar16;
        (**ppcVar7)();
        if (uVar12 == 0x0) break;
        uVar5 = *(ulong *)(uStack44 + 0x4);
        puStack14 = (undefined2 *)uVar5;
        pcStack12 = (code *)(uVar5 >> 0x10);
        lVar6 = *(long *)(uStack44 + 0x8);
        uStack40 = (ushort)lVar6;
        uStack38 = (ushort)((ulong)lVar6 >> 0x10);
        param_7 = (ushort)&local_30;
        puStack36 = (undefined4 *)((ulong)puStack36 & 0xffff0000 | ZEXT24(&stack0xffca));
        uStackY78 = 0xe977;
        pass1_1030_671c(_PTR_LOOP_1050_5740,uVar5,(ushort *)CONCAT22(param_9,&stack0xffca),lVar6,(ushort)&stack0xffca,
                        uStack42,param_7,param_9);
        pass1_1030_14b4(uVar20->field_0x1e,uStack44,uStack42,CONCAT22(pcStack12,puStack14) & 0xffffff,param_9);
        local_30 = (undefined4 *)((long)local_30 + 0x1);
      }
      return;
    case (undefined2 *)0x9:
      local_6 = (undefined4 *)((ulong)local_6 & 0xffff);
      pcStack12 = (code *)uVar20->field_0x2e;
      puStack10 = (ushort *)(ulong)uVar20->field_0x30;
      (*pcStack12)();
      return;
    case (undefined2 *)0xa:
      pass1_1030_145a(uVar20->field_0x22,(long)local_6);
      uVar21 = 0x0;
      uVar10 = 0x0;
      while ((undefined4 *)CONCAT22(uVar10,uVar21) < local_6) {
        puVar27 = local_6;
        mem_op_1000_179c(0xe,(uchar *)puVar17,0x1000);
        puStack32 = (undefined2 *)(uint)puVar27;
        puStack30 = puVar17;
        if (((uint)puVar17 | (uint)puStack32) == 0x0) {
          iVar14 = 0x0;
          uVar15 = 0x0;
        }
        else {
          puVar25 = pass1_1028_b204((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10));
          uVar15 = (undefined2)((ulong)puVar25 >> 0x10);
          iVar14 = (int)puVar25;
        }
        local_30 = (undefined4 *)CONCAT22(uVar15,iVar14);
        ppcVar7 = (code **)((int)*local_30 + 0x10);
        (**ppcVar7)();
        if (iVar14 == 0x0) {
          return;
        }
        uVar22 = (ushort)((ulong)local_30 >> 0x10);
        uVar19 = (ushort)local_30;
        uVar5 = *(ulong *)(uVar19 + 0x4);
        uVar16 = *(uint *)(uVar19 + 0x6);
        puStack14 = (undefined2 *)uVar5;
        pcStack12 = (code *)(uVar5 >> 0x10);
        puVar17 = (undefined2 *)(uVar16 & 0xff);
        pass1_1030_14b4(uVar20->field_0x22,uVar19,uVar22,uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10,param_9);
        lVar6 = CONCAT22(uVar10,uVar21) + 0x1;
        uVar21 = (undefined2)lVar6;
        uVar10 = (undefined2)((ulong)lVar6 >> 0x10);
      }
      break;
    case (undefined2 *)0xb:
      if (puVar17 < (undefined2 *)((int)&PTR_LOOP_1050_000e + 0x1)) {
        pcVar1 = (char *)(param_6 + 0x23);
        cVar3 = *pcVar1;
        *pcVar1 = *pcVar1 << 0x6;
        piVar2 = (int *)((int)puVar17 + param_6);
        *piVar2 = *piVar2 + (-0x6600 - (uint)((char)(cVar3 << 0x5) < '\0'));
      }
      else {
        pass1_1028_780c(uVar19,uVar22,CONCAT22(in_stack_0000ffcc,in_stack_0000ffca));
        if (param_4 == 0x0) goto code_r0x10287b17;
      }
      uVar29 = 0x0;
      uVar31 = 0x4;
      iVar14 = 0x1d;
      uStackY78 = 0x7b0a;
      puVar25 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_9,(uchar *)puVar17,param_7);
      puVar17 = (undefined2 *)((ulong)puVar25 >> 0x10);
      param_4 = (int)puVar25;
      pass1_1010_043a((ulong)puVar25,CONCAT13(uVar31,CONCAT12(uVar29,puVar17)),iVar14,param_9);
code_r0x10287b17:
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x2,0x400);
      pass1_1028_780c((ushort)uVar20,uVar37,CONCAT22(puVar17,param_4));
      puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_9,(uchar *)puVar17,param_7);
      pcStack12 = (code *)PTR_LOOP_1050_13ae;
      if (0x2 < (int)PTR_LOOP_1050_13ae) {
        puVar25 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_9,(uchar *)((ulong)puStack10 >> 0x10),param_7);
        uVar32 = (undefined)((ulong)puVar25 >> 0x10);
        uVar33 = (undefined)((ulong)puVar25 >> 0x18);
        uVar35 = 0x1;
        uVar36 = 0x0;
        uVar29 = (char)puVar25;
        uVar31 = (char)((ulong)puVar25 >> 0x8);
        while (CONCAT11(uVar36,uVar35) < 0x9) {
          uVar28 = uVar29;
          uVar30 = uVar31;
          if ((undefined4 *)*(long *)(CONCAT11(uVar31,uVar29) + 0x34 + CONCAT11(uVar36,uVar35) * 0x4) == local_6) {
            puVar9 = (undefined2 *)((int)&PTR_LOOP_1050_0000 + 0x1);
            local_30 = (undefined4 *)CONCAT22(local_30._2_2_,0x1);
            uVar35 = 0xd7;
            uVar36 = 0x7b;
            pass1_1008_612e(0x1,0x64,0x1);
            puVar17 = (undefined2 *)(CONCAT11(uVar36,uVar35) - 0x7);
            if (puVar17 == (undefined2 *)0x0) {
              bVar24 = SBORROW2((int)puVar9,0x32);
              puVar8 = puVar9 + -0x19;
              bVar23 = puVar9 == (undefined2 *)((int)s_New_failed_in_Op__Op_1050_0020 + 0x12);
LAB_1028_7b74:
              if (!bVar23 && bVar24 == (int)puVar8 < 0x0) {
                local_30 = (undefined4 *)((ulong)local_30 & 0xffff0000);
              }
            }
            else {
              puVar17 = (undefined2 *)(CONCAT11(uVar36,uVar35) - 0x8);
              if (puVar17 == (undefined2 *)0x0) {
                bVar24 = SBORROW2((int)puVar9,0x19);
                puVar8 = (undefined2 *)((int)puVar9 + -0x19);
                bVar23 = puVar8 == (undefined2 *)0x0;
                goto LAB_1028_7b74;
              }
            }
            puStack30 = puVar9;
            if ((int)local_30 != 0x0) {
              pass1_1028_90e6((astruct_100 *)CONCAT13(uVar34,CONCAT12((char)param_9,&local_154)),CONCAT11(uVar36,uVar35)
                              ,param_9,param_10);
              puVar17 = &local_154;
              uVar32 = 0x8;
              uVar33 = 0x10;
              uVar29 = 0xc;
              uVar31 = 0x7c;
              fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_9,puVar17));
              local_154 = 0x389a;
              uStackY338 = 0x1008;
            }
            uVar38 = 0x0;
            uVar39 = 0x0;
            uVar35 = 0x23;
            uVar36 = 0x7c;
            pass1_1008_612e(0x0,0xa,(uint)puVar17);
            ppuStack24 = (undefined2 **)puVar17;
            if (CONCAT11(uVar36,uVar35) == 0x7) {
              iVar40 = 0x7;
              puVar17 = puVar17 + 0x37;
              iVar14 = (int)puVar17 >> 0xf;
            }
            else {
              uVar28 = uVar29;
              uVar30 = uVar31;
              if (CONCAT11(uVar36,uVar35) != 0x8) goto LAB_1028_7ba0;
              iVar40 = 0x8;
              puVar17 = puVar17 + 0x32;
              iVar14 = ((int)puVar17 >> 0xf) + (uint)((undefined2 *)0xff9b < puVar17);
            }
            uVar19 = iVar40 + iVar14 + (uint)CARRY2(CONCAT11(uVar39,uVar38),(uint)puVar17);
            uVar28 = 0x8;
            uVar30 = 0x10;
            uVar35 = uVar32;
            uVar36 = uVar33;
            pass1_1010_ebf8(CONCAT13(uVar33,CONCAT12(uVar32,CONCAT11(uVar31,uVar29))),
                            CONCAT11(uVar39,uVar38) + (int)puVar17,uVar19,uVar19);
            uVar32 = uVar29;
            uVar33 = uVar31;
          }
LAB_1028_7ba0:
          iVar14 = CONCAT11(uVar36,uVar35) + 0x1;
          uVar35 = (undefined)iVar14;
          uVar29 = uVar28;
          uVar31 = uVar30;
          uVar36 = (undefined)((uint)iVar14 >> 0x8);
        }
      }
      return;
    case (undefined2 *)0xc:
      paVar18 = uVar20;
      pass1_1030_145a(uVar20->field_0x26,(long)local_6);
      uVar21 = 0x0;
      uVar10 = 0x0;
      while ((undefined4 *)CONCAT22(uVar10,uVar21) < local_6) {
        BVar11 = read_file_1008_7dee(param_2,param_3,(ushort)&local_30,0x0,param_9,0x2,0x1008);
        if (BVar11 == 0x0) {
          PTR_LOOP_1050_0310 = (undefined *)0x6d2;
          return;
        }
        uStack44 = switch_1008_73ea(param_2,param_3,(int)local_30);
        puVar27 = (undefined4 *)
                  switch_1030_0000((ushort)uVar20,uVar37,uStack44,(uchar *)puVar17,(uint)paVar18,param_6,param_7);
        uStack38 = (ushort)((ulong)puVar27 >> 0x10);
        uVar19 = (ushort)puVar27;
        ppcVar7 = (code **)((int)*puVar27 + 0x10);
        uStack40 = uVar19;
        (**ppcVar7)();
        if (uVar19 == 0x0) {
          return;
        }
        uVar5 = *(ulong *)(uStack40 + 0x4);
        uVar16 = *(uint *)(uStack40 + 0x6);
        puStack14 = (undefined2 *)uVar5;
        pcStack12 = (code *)(uVar5 >> 0x10);
        puVar17 = (undefined2 *)(uVar16 & 0xff);
        paVar18 = uVar20;
        pass1_1030_14b4(uVar20->field_0x26,uStack40,uStack38,uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10,param_9);
        lVar6 = CONCAT22(uVar10,uVar21) + 0x1;
        uVar21 = (undefined2)lVar6;
        uVar10 = (undefined2)((ulong)lVar6 >> 0x10);
      }
      break;
    case (undefined2 *)0xd:
      puStack10 = (ushort *)(ZEXT24(puVar17) << 0x10);
      uVar13 = *(undefined4 *)&PTR_LOOP_1050_000c;
      local_10 = (undefined2 *)uVar13;
      puStack14 = (undefined2 *)((ulong)uVar13 >> 0x10);
      pcStack12 = *(code **)&PTR_LOOP_1050_0010;
      ppuStack24 = &local_10;
      uStackY78 = 0x211d;
      pass1_1008_3eb4((ushort *)CONCAT13(uVar34,CONCAT12((char)param_9,&local_10)),(ushort *)CONCAT22(param_9,&local_16)
                      ,(ushort *)CONCAT22(param_9,&local_14),(ushort *)CONCAT22(param_9,&local_12));
      ppuStack24 = (undefined2 **)((int)local_14 + -0x1);
      puStack14 = ppuStack24;
      uVar16 = pass1_1028_21ba((ushort)uVar20,uVar37,(ushort *)CONCAT22(param_9,&local_10),(long)local_6,(uint)&local_10
                               ,(uint)puVar17,param_9);
      if (uVar16 == 0x0) {
        ppuStack24 = (undefined2 **)((int)local_14 + 0x1);
        puStack14 = ppuStack24;
        uVar16 = pass1_1028_21ba((ushort)uVar20,uVar37,(ushort *)CONCAT22(param_9,&local_10),(long)local_6,
                                 (uint)&local_10,(uint)puVar17,param_9);
        if (uVar16 == 0x0) {
          puStack14 = local_14;
          ppuStack24 = (undefined2 **)(local_12 + -0x1);
          local_10 = ppuStack24;
          uVar16 = pass1_1028_21ba((ushort)uVar20,uVar37,(ushort *)CONCAT22(param_9,&local_10),(long)local_6,
                                   (uint)&local_10,(uint)puVar17,param_9);
          if (uVar16 == 0x0) {
            ppuStack24 = (undefined2 **)(local_12 + 0x1);
            local_10 = ppuStack24;
            uVar16 = pass1_1028_21ba((ushort)uVar20,uVar37,(ushort *)CONCAT22(param_9,&local_10),(long)local_6,
                                     (uint)&local_10,(uint)puVar17,param_9);
            if (uVar16 == 0x0) {
              return;
            }
          }
        }
      }
      pass1_1038_79b2(_PTR_LOOP_1050_5a64,(ulong)puStack10,uVar16,(uchar *)puVar17);
      return;
    case (undefined2 *)0xe:
      pass1_1030_145a(uVar20->field_0x2a,(long)local_6);
      uVar21 = 0x0;
      uVar10 = 0x0;
      while ((undefined4 *)CONCAT22(uVar10,uVar21) < local_6) {
        puVar27 = local_6;
        mem_op_1000_179c(0x3b2,(uchar *)puVar17,0x1000);
        puStack32 = (undefined2 *)(uint)puVar27;
        uVar16 = (uint)puVar17 | (uint)puStack32;
        puStack30 = puVar17;
        if (uVar16 == 0x0) {
          uVar12 = 0x0;
          uVar16 = 0x0;
        }
        else {
          uVar12 = (uint)puStack32;
          pass1_1030_2068((ushort *)((ulong)puVar27 & 0xffff | ZEXT24(puVar17) << 0x10));
        }
        local_30 = (undefined4 *)CONCAT22(uVar16,uVar12);
        ppcVar7 = (code **)((int)*local_30 + 0x10);
        (**ppcVar7)();
        if (uVar12 == 0x0) {
          return;
        }
        uVar22 = (ushort)((ulong)local_30 >> 0x10);
        uVar19 = (ushort)local_30;
        uVar5 = *(ulong *)(uVar19 + 0x4);
        uVar16 = *(uint *)(uVar19 + 0x6);
        puStack14 = (undefined2 *)uVar5;
        pcStack12 = (code *)(uVar5 >> 0x10);
        puVar17 = (undefined2 *)(uVar16 & 0xff);
        pass1_1030_14b4(uVar20->field_0x2a,uVar19,uVar22,uVar5 & 0xffff | ((ulong)uVar16 & 0xff) << 0x10,param_9);
        lVar6 = CONCAT22(uVar10,uVar21) + 0x1;
        uVar21 = (undefined2)lVar6;
        uVar10 = (undefined2)((ulong)lVar6 >> 0x10);
      }
    }
    pass1_1030_154c();
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1028_ebee(ulong param_1,ushort param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  ulong uVar3;
  
  mem_op_1000_179c(0x14,(uchar *)param_3,0x1000);
  if ((uchar *)(param_3 | param_2) != (uchar *)0x0) {
    pass1_1030_1a32((ushort *)CONCAT22(param_3,param_2),param_2,(uchar *)(param_3 | param_2));
  }
  uVar3 = struct_1030_4574(*(ulong *)((int)param_1 + 0x52));
  uVar2 = (undefined2)((ulong)_PTR_LOOP_1050_5166 >> 0x10);
  iVar1 = (int)_PTR_LOOP_1050_5166;
  *(undefined2 *)(iVar1 + 0x10) = (int)uVar3;
  *(undefined2 *)(iVar1 + 0x12) = (int)(uVar3 >> 0x10);
  uVar2 = (undefined2)((ulong)_PTR_LOOP_1050_5166 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)_PTR_LOOP_1050_5166 + 0x6),*(undefined2 *)((int)_PTR_LOOP_1050_5166 + 0x4));
}



void __stdcall16far
pass1_1028_ec36(ulong param_1,ushort param_2,int param_3,ushort param_4,ulong param_5,ushort param_6,uchar *param_7,
               ushort param_8)

{
  undefined4 uVar1;
  ushort uVar2;
  ushort uVar3;
  uchar *puVar4;
  uchar *puVar5;
  undefined2 uVar6;
  ushort *puVar7;
  
  mem_op_1000_179c(0x14,param_7,0x1000);
  if ((uchar *)((uint)param_7 | param_6) == (uchar *)0x0) {
    uVar2 = 0x0;
    puVar4 = (uchar *)0x0;
  }
  else {
    puVar7 = pass1_1030_5d3c((ushort *)CONCAT22(param_7,param_6),param_5,param_6,(uchar *)((uint)param_7 | param_6));
    puVar4 = (uchar *)((ulong)puVar7 >> 0x10);
    uVar2 = (ushort)puVar7;
  }
  uVar6 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x52);
  puVar5 = puVar4;
  uVar3 = uVar2;
  pass1_1030_4594(puVar4,(ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),param_3);
  pass1_1030_5fe2(CONCAT22(puVar4,uVar2),CONCAT22(puVar5,uVar3));
  pass1_1030_1358(*(ulong *)((int)param_1 + 0xe),uVar2,(ushort)puVar4,
                  *(ulong *)(uVar2 + 0x4) & 0xffff | ((ulong)*(uint *)(uVar2 + 0x6) & 0xff) << 0x10,param_8);
  return;
}



void __stdcall16far
pass1_1028_ecac(ulong param_1,ushort param_2,int *param_3,ushort param_4,ulong param_5,ushort param_6,uchar *param_7,
               ushort param_8)

{
  undefined4 uVar1;
  int **ppiVar2;
  uchar *puVar3;
  uchar *puVar4;
  undefined2 uVar5;
  
  mem_op_1000_179c(0x1c,param_7,0x1000);
  puVar3 = (uchar *)((uint)param_7 | param_6);
  if (puVar3 == (uchar *)0x0) {
    param_6 = 0x0;
    puVar3 = (uchar *)0x0;
  }
  else {
    struct_1030_299a((ushort *)CONCAT22(param_7,param_6),param_5,param_6,puVar3);
  }
  uVar5 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x52);
  puVar4 = puVar3;
  ppiVar2 = (int **)param_3;
  pass1_1030_4628(puVar3,(ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),(int)param_3);
  *ppiVar2 = param_3;
  pass1_1030_3006(CONCAT22(puVar3,param_6),CONCAT22(puVar4,ppiVar2));
  pass1_1030_1358(*(ulong *)((int)param_1 + 0x12),param_6,(ushort)puVar3,
                  *(ulong *)(param_6 + 0x4) & 0xffff | ((ulong)*(uint *)(param_6 + 0x6) & 0xff) << 0x10,param_8);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_ed2c(ulong param_1,ushort param_2,int param_3,ushort param_4,ulong param_5,ushort param_6,uchar *param_7,
               ushort param_8,uchar param_9)

{
  ulong uVar2;
  ushort uVar3;
  ushort uVar4;
  uchar *puVar5;
  uchar *puVar6;
  uchar *puVar7;
  undefined2 uVar8;
  ushort *puVar9;
  undefined4 uVar1;
  
  mem_op_1000_179c(0x1e,param_7,0x1000);
  if ((uchar *)((uint)param_7 | param_6) == (uchar *)0x0) {
    uVar3 = 0x0;
    puVar5 = (uchar *)0x0;
  }
  else {
    puVar9 = struct_1030_565a((ushort *)CONCAT22(param_7,param_6),param_5,param_6,(uchar *)((uint)param_7 | param_6));
    puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
    uVar3 = (ushort)puVar9;
  }
  uVar8 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x52);
  puVar6 = puVar5;
  uVar4 = uVar3;
  pass1_1030_4782(param_8,param_9,puVar5,(ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),0x1,0x1,param_3);
  puVar7 = puVar6;
  pass1_1030_5a80(CONCAT22(puVar5,uVar3),CONCAT22(puVar6,uVar4),param_8);
  uVar2 = *(ulong *)(uVar3 + 0x4);
  pass1_1030_6222(_PTR_LOOP_1050_5740,0x1,CONCAT22(puVar6,uVar4),uVar2,(uint)uVar2,puVar7,param_8);
  pass1_1030_1358(*(ulong *)((int)param_1 + 0x16),uVar3,(ushort)puVar5,uVar2 & 0xffffff,param_8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_edc4(ulong param_1,ushort param_2,ushort *param_3,long param_4,uchar *param_5,ushort param_6)

{
  ushort uVar1;
  ulong uVar2;
  uchar *puVar3;
  uchar in_AF;
  undefined local_1a [0x4];
  ulong uStack22;
  undefined4 uStack18;
  ulong uStack14;
  ulong uStack10;
  ushort *puStack6;
  
  puStack6 = param_3;
  pass1_1030_64ce(param_6,(int)param_3,param_5,_PTR_LOOP_1050_5740,param_3,param_4,(ulong *)CONCAT22(param_6,local_1a));
  uVar2 = *(ulong *)param_3;
  uStack14 = uVar2;
  uStack10 = uVar2;
  mem_op_1000_179c(0x21e,param_5,0x1000);
  uVar1 = (ushort)uVar2;
  puVar3 = (uchar *)((uint)param_5 | uVar1);
  if (puVar3 == (uchar *)0x0) {
    uVar1 = 0x0;
    puVar3 = (uchar *)0x0;
  }
  else {
    pass1_1038_3222((ushort *)(uVar2 & 0xffff | ZEXT24(param_5) << 0x10),uStack14,param_4,uVar1,puVar3,in_AF,
                    (uchar *)param_6);
  }
  uStack18 = CONCAT22(puVar3,uVar1);
  uStack22 = *(ulong *)(uVar1 + 0x4);
  pass1_1030_1358(*(ulong *)((int)param_1 + 0x1a),uVar1,(ushort)puVar3,
                  uStack22 & 0xffff | ((ulong)*(uint *)(uVar1 + 0x6) & 0xff) << 0x10,param_6);
  return;
}

