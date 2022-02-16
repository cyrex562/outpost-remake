


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_2306(ushort param_1,ushort param_2,ulong param_3)

{
  int *piVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  qword qVar4;
  undefined4 *puVar5;
  astruct_417 *uVar9;
  ulong uVar6;
  undefined4 *puVar7;
  uint uVar8;
  uint uVar10;
  astruct_419 *iVar11;
  astruct_418 *iVar12;
  undefined2 uVar11;
  undefined2 uVar12;
  undefined2 uVar13;
  ulong uVar14;
  ulong uStack42;
  ulong uStack34;
  ushort uStack30;
  ulong uStack24;
  undefined4 uStack12;
  int iStack8;
  
  uVar13 = 0x1030;
  uVar14 = struct_op_1030_73a8(param_3);
  uStack24 = uVar14 >> 0x10;
  uVar11 = (undefined2)(param_3 >> 0x10);
  iVar11 = (astruct_419 *)param_3;
  iStack8 = iVar11->field_0x34;
  uStack12 = 0x64;
  puVar2 = (undefined4 *)*(undefined4 *)((int)uVar14 + 0x22);
  puVar7 = puVar2;
  while( true ) {
    uVar8 = (uint)uStack24;
    uVar12 = (undefined2)((ulong)puVar2 >> 0x10);
    ppcVar3 = (code **)((int)*puVar2 + 0x10);
    (**ppcVar3)(uVar13,(int)puVar2,uVar12);
    uVar9 = (astruct_417 *)puVar7;
    uVar14 = (ulong)puVar7 & 0xffff;
    puVar5 = (undefined4 *)(uVar14 | (ulong)uVar8 << 0x10);
    if ((uVar8 | (uint)uVar9) == 0x0) break;
    if (uVar9->field_0xa == 0x0) {
      uStack24 = (ulong)(uVar8 | (uint)uVar9);
      if ((uVar8 | (uint)uVar9) != 0x0) {
        ppcVar3 = (code **)*puVar5;
        (**ppcVar3)((char)uVar13,uVar9,uVar8,0x1);
      }
    }
    else {
      uStack24 = 0x0;
      uStack30 = 0x0;
      if (uVar9->field_0x6 == 0x0) {
        if (uVar9->field_0x8 != 0x0) {
          uStack30 = pass1_1020_c42e(uVar9->field_0x8);
          goto LAB_1038_2385;
        }
      }
      else {
        uStack30 = switch_1020_c3b4(uVar9->field_0x6);
LAB_1038_2385:
        uVar13 = 0x1020;
        uStack24 = (ulong)(uVar9->field_0xa * uStack30);
      }
      uStack12._2_2_ = 0x0;
      if (uStack12 < uStack24) {
        uStack24 = uStack12 & 0xffff;
      }
      uStack34 = uStack24 | (ulong)uStack12._2_2_ << 0x10;
      uStack24 = uStack24 | (ulong)uStack12._2_2_ << 0x10;
      qVar4 = (qword)uStack24 / (qword)(ulong)uStack30;
      uVar6 = (ulong)qVar4;
      uStack24 = uStack24 % (ulong)uStack30;
      piVar1 = &uVar9->field_0xa;
      *piVar1 = *piVar1 - (int)qVar4;
      if (*piVar1 == 0x0) {
        uStack24 = (ulong)(uVar8 | (uint)uVar9);
        if ((uVar8 | (uint)uVar9) != 0x0) {
          ppcVar3 = (code **)*puVar5;
          (**ppcVar3)((char)uVar13,uVar9,uVar8,0x1);
        }
      }
      else {
        ppcVar3 = (code **)((int)*puVar2 + 0x8);
        (**ppcVar3)(uVar13,(char)puVar2,uVar12,uVar9,uVar8);
      }
      uStack12 = uStack12 - uStack34;
      puVar7 = (undefined4 *)0x0;
      uStack42 = 0x0;
      iVar12 = (astruct_418 *)uVar14;
      if (iVar12->field_0x6 == 0x0) {
        if (iVar12->field_0x8 != 0x0) {
          mem_op_1000_179c(0x2a,(uchar *)uStack24,0x1000);
          uVar10 = (uint)uStack24 | (uint)puVar7;
          uVar14 = (ulong)uVar10;
          if (uVar10 == 0x0) goto LAB_1038_244e;
          pass1_1038_6838((ushort *)((ulong)puVar7 & 0xffff | uStack24 << 0x10),uVar6,iVar12->field_0x8,0x1,
                          iVar11->field_0x4);
          goto LAB_1038_24b3;
        }
      }
      else {
        mem_op_1000_179c(0x2a,(uchar *)uStack24,0x1000);
        uVar10 = (uint)uStack24 | (uint)puVar7;
        uVar14 = (ulong)uVar10;
        if (uVar10 == 0x0) {
LAB_1038_244e:
          uVar13 = 0x1000;
          uStack42 = 0x0;
          uStack24 = uVar14;
        }
        else {
          pass1_1038_675c((ushort *)((ulong)puVar7 & 0xffff | uStack24 << 0x10),uVar6,iVar12->field_0x6,0x1,
                          iVar11->field_0x4);
LAB_1038_24b3:
          uVar13 = 0x1000;
          uStack42 = (ulong)puVar7 & 0xffff | uVar14 << 0x10;
          uStack24 = uVar14;
        }
      }
      if (uStack42 != 0x0) {
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        iStack8 = iStack8 + -0x1;
        if (iStack8 == 0x0) break;
        uStack12 = 0x64;
      }
    }
  }
  pass1_1030_6c4c(param_3,iStack8);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_24e8(ushort param_1,ushort param_2,ulong param_3,ushort param_4,ushort param_5,ushort param_6)

{
  uint uVar1;
  ulong uVar2;
  uchar *puVar3;
  uchar *puVar4;
  int iVar5;
  undefined2 uVar6;
  uint uStack30;
  astruct_18 *paStack28;
  undefined4 local_18;
  undefined2 local_14;
  uint uStack18;
  undefined4 uStack16;
  ulong *puStack12;
  int iStack8;
  ulong uStack6;
  
  uStack6 = struct_op_1030_73a8(param_3);
  puVar4 = (uchar *)(uStack6 >> 0x10);
  uVar6 = (undefined2)(param_3 >> 0x10);
  iVar5 = (int)param_3;
  iStack8 = *(int *)(iVar5 + 0x34);
  puStack12 = *(ulong **)((int)uStack6 + 0x28);
  uStack16 = 0x64;
  uStack18 = *(uint *)((int)puStack12 + 0x4);
  uVar2 = (ulong)uStack18;
  mem_op_1000_179c(0xa,puVar4,0x1000);
  uVar1 = (uint)uVar2;
  puVar3 = (uchar *)((uint)puVar4 | uVar1);
  if (puVar3 == (uchar *)0x0) {
    uVar1 = 0x0;
    puVar3 = (uchar *)0x0;
  }
  else {
    pass1_1020_ba3e((long *)(uVar2 & 0xffff | ZEXT24(puVar4) << 0x10),0x5,0x5,param_5,param_4);
  }
  paStack28 = (astruct_18 *)CONCAT22(puVar3,uVar1);
  for (uStack30 = 0x0; uVar2 = (ulong)uStack18, uStack30 < uStack18; uStack30 = uStack30 + 0x1) {
    pass1_1020_bb16(puStack12,(ulong *)CONCAT22(param_6,&local_18),(ushort *)CONCAT22(param_6,&local_14),uStack30);
    if (local_18 != 0x0) {
      uVar2 = local_18;
      uStack16._2_2_ = local_18._2_2_;
      if (uStack16 < local_18) {
        uVar2 = uStack16 & 0xffff;
      }
      uVar1 = (uint)uVar2;
      uVar2 = uVar2 & 0xffff | ZEXT24(uStack16._2_2_) << 0x10;
      local_18 = CONCAT22(local_18._2_2_ + (-(uint)((uint)local_18 < uVar1) - (int)uStack16._2_2_),
                          (uint)local_18 - uVar1);
      puVar3 = uStack16._2_2_;
      pass1_1020_bb8a((long *)puStack12,(uint)local_18 - uVar1,
                      CONCAT22(local_14,local_18._2_2_ + (-(uint)((uint)local_18 < uVar1) - (int)uStack16._2_2_)),
                      param_5,param_6);
      pass1_1020_bb70((long *)paStack28,uVar1,CONCAT22(local_14,uStack16._2_2_),param_5,param_4,param_6);
      uStack16 = uStack16 - uVar2;
      if (uStack16 == 0x0) {
        mem_op_1000_179c(0x2a,puVar3,0x1000);
        puVar4 = (uchar *)((uint)puVar3 | (uint)uVar2);
        if (puVar4 == (uchar *)0x0) {
          uVar2 = 0x0;
        }
        else {
          pass1_1038_666e((ushort *)(uVar2 & 0xffff | ZEXT24(puVar3) << 0x10),(long *)paStack28,0x1,
                          *(ulong *)(iVar5 + 0x4));
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        mem_op_1000_179c(0xa,puVar4,0x1000);
        puVar3 = (uchar *)((uint)puVar4 | (uint)uVar2);
        if (puVar3 == (uchar *)0x0) {
          uVar2 = 0x0;
          puVar3 = (uchar *)0x0;
        }
        else {
          pass1_1020_ba3e((long *)(uVar2 & 0xffff | ZEXT24(puVar4) << 0x10),0x5,0x5,param_5,param_4);
        }
        paStack28 = (astruct_18 *)(uVar2 & 0xffff | ZEXT24(puVar3) << 0x10);
        iStack8 = iStack8 + -0x1;
        if (iStack8 == 0x0) break;
        uStack16 = 0x64;
      }
    }
  }
  pass1_1020_ba94((long *)paStack28);
  puVar3 = (uchar *)((uint)puVar3 | (uint)uVar2);
  if (puVar3 == (uchar *)0x0) {
    if (paStack28 != (astruct_18 *)0x0) {
      fn_ptr_1020_ba7e((ulong *)paStack28);
      fn_ptr_1000_17ce(paStack28,0x1000);
    }
  }
  else {
    mem_op_1000_179c(0x2a,puVar3,0x1000);
    if (((uint)puVar3 | (uint)uVar2) != 0x0) {
      pass1_1038_666e((ushort *)(uVar2 & 0xffff | ZEXT24(puVar3) << 0x10),(long *)paStack28,0x1,*(ulong *)(iVar5 + 0x4))
      ;
    }
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
  }
  pass1_1030_6c4c(param_3,iStack8);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_26ee(ushort param_1,ushort param_2,ulong param_3,ushort param_4,ushort param_5,ushort param_6)

{
  uint uVar1;
  uint uVar2;
  ulong uVar3;
  uchar *puVar4;
  uchar *puVar5;
  uchar *puVar6;
  int iVar7;
  undefined2 uVar8;
  ulong uVar9;
  undefined4 uVar10;
  ulong uStack36;
  uint uStack20;
  uchar *puStack18;
  undefined4 uStack16;
  ushort uStack12;
  ushort uStack10;
  int iStack8;
  
  uVar9 = struct_op_1030_73a8(param_3);
  puVar6 = (uchar *)(uVar9 >> 0x10);
  uVar8 = (undefined2)(param_3 >> 0x10);
  iVar7 = (int)param_3;
  iStack8 = *(int *)(iVar7 + 0x34);
  uStack12 = pass1_1028_0d80(uVar9);
  uVar3 = (ulong)uStack12;
  uStack16 = 0x64;
  mem_op_1000_179c(0xa,puVar6,0x1000);
  puVar4 = (uchar *)((uint)puVar6 | (uint)uVar3);
  if (puVar4 == (uchar *)0x0) {
    uVar3 = 0x0;
    puVar4 = (uchar *)0x0;
  }
  else {
    pass1_1020_ba3e((long *)(uVar3 & 0xffff | ZEXT24(puVar6) << 0x10),0x5,0x5,param_5,param_4);
  }
  uStack20 = (uint)uVar3;
  uStack10 = uStack12;
  puStack18 = puVar4;
  do {
    uVar10 = pass1_1030_7c28(param_3,uStack10,(uint)uVar3,(uint)puVar4,param_6);
    puVar6 = (uchar *)((ulong)uVar10 >> 0x10);
    uVar1 = (uint)uVar10;
    puVar4 = (uchar *)((uint)puVar6 | uVar1);
    if (puVar4 != (uchar *)0x0) {
      puVar5 = puVar6;
      uVar2 = uVar1;
      if ((uStack16._2_2_ <= puVar6) && ((uStack16._2_2_ < puVar6 || ((uint)uStack16 < uVar1)))) {
        puVar5 = uStack16._2_2_;
        uVar2 = (uint)uStack16;
      }
      uStack36 = CONCAT22(puVar5,uVar2);
      puVar4 = puVar5;
      pass1_1030_7d1c(param_3,uVar1 - uVar2,CONCAT22(uStack10,puVar6 + (-(uint)(uVar1 < uVar2) - (int)puVar5)),uVar2,
                      puVar5,param_4,param_5,param_6);
      pass1_1020_bb70((long *)CONCAT22(puStack18,uStack20),uVar2,CONCAT22(uStack10,puVar5),param_5,param_4,param_6);
      uStack16 = uStack16 - uStack36;
      if (uStack16 == 0x0) {
        mem_op_1000_179c(0x2a,puVar4,0x1000);
        uStack10 = (ushort)uStack36;
        puVar6 = (uchar *)((uint)puVar4 | uStack10);
        if (puVar6 == (uchar *)0x0) {
          uStack10 = 0x0;
        }
        else {
          pass1_1038_666e((ushort *)(uStack36 & 0xffff | ZEXT24(puVar4) << 0x10),(long *)CONCAT22(puStack18,uStack20),
                          0x1,*(ulong *)(iVar7 + 0x4));
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        mem_op_1000_179c(0xa,puVar6,0x1000);
        puVar4 = (uchar *)((uint)puVar6 | uStack10);
        if (puVar4 == (uchar *)0x0) {
          uStack10 = 0x0;
          puVar4 = (uchar *)0x0;
        }
        else {
          pass1_1020_ba3e((long *)CONCAT22(puVar6,uStack10),0x5,0x5,param_5,param_4);
        }
        iStack8 = iStack8 + -0x1;
        uStack20 = uStack10;
        puStack18 = puVar4;
        if (iStack8 == 0x0) break;
        uStack16 = 0x64;
      }
    }
    uStack10 = pass1_1028_0d80(uVar9);
    uVar3 = (ulong)uStack10;
    if (uStack12 == 0x0) {
      uStack12 = uStack10;
    }
  } while (uStack12 != uStack10);
  pass1_1020_ba94((long *)CONCAT22(puStack18,uStack20));
  puVar4 = (uchar *)((uint)puVar4 | uStack10);
  if (puVar4 == (uchar *)0x0) {
    if (((uint)puStack18 | uStack20) != 0x0) {
      fn_ptr_1020_ba7e((ulong *)CONCAT22(puStack18,uStack20));
      fn_ptr_1000_17ce((astruct_18 *)CONCAT22(puStack18,uStack20),0x1000);
    }
  }
  else {
    mem_op_1000_179c(0x2a,puVar4,0x1000);
    if (((uint)puVar4 | uStack10) != 0x0) {
      pass1_1038_666e((ushort *)CONCAT22(puVar4,uStack10),(long *)CONCAT22(puStack18,uStack20),0x1,
                      *(ulong *)(iVar7 + 0x4));
    }
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
  }
  pass1_1030_6c4c(param_3,iStack8);
  return;
}



astruct_100 * __stdcall16far pass1_1038_28d8(astruct_100 *param_1,ushort param_2,uchar param_3)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x3a97);
  param_1->field_0x0 = 0x29fe;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),s_SCRoboMove_1050_59f8);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1038_290e(uint param_1,uint param_2)

{
  undefined2 unaff_SI;
  undefined2 unaff_DI;
  ushort unaff_SS;
  uchar in_AF;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
  if ((param_2 | param_1) != 0x0) {
    pass1_1038_4918(CONCAT22(param_2,param_1),param_1,param_2 | param_1,unaff_SS,in_AF);
  }
  pass1_1038_7a76(_PTR_LOOP_1050_5a64,unaff_SI,unaff_DI,unaff_SS);
  return 0x1;
}



void __stdcall16far pass1_1038_2944(ulong param_1,uint param_2,uchar *param_3)

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
    *puStack10 = 0x29fe;
    *(undefined2 *)(param_2 + 0x2) = (int)&PTR_LOOP_1050_1038;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_29d2(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
pass1_1038_2a0e(astruct_100 *param_1,ulong param_2,ulong param_3,ulong param_4,ulong param_5,ushort param_6,
               uchar param_7)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_op_1028_d1dc(param_6,param_7,param_1,0x2af7);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ulong *)(iVar1 + 0x108) = param_5;
  *(ulong *)(iVar1 + 0x10c) = param_4;
  *(ulong *)(iVar1 + 0x110) = param_3;
  *(ulong *)(iVar1 + 0x114) = param_2;
  param_1->field_0x0 = 0x309a;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return;
}



void __stdcall16far pass1_1038_2a5c(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x309a;
  *(undefined2 *)(iVar4 + 0x2) = (int)&PTR_LOOP_1050_1038;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x114);
  uVar2 = *(uint *)(iVar4 + 0x116);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x110);
  uVar2 = *(uint *)(iVar4 + 0x112);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *param_1 = 0x389a;
  *(undefined2 *)(iVar4 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far
pass1_1038_2ac2(ulong param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6,uchar param_7)

{
  undefined4 uVar1;
  ushort uVar2;
  ushort uVar3;
  ulong uStack10;
  ulong uStack6;
  
  uVar3 = (ushort)(param_1 >> 0x10);
  uVar2 = (ushort)param_1;
  uVar1 = *(undefined4 *)(uVar2 + 0x108);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  uStack6 = CONCAT22(param_3,param_2);
  uVar1 = *(undefined4 *)(uVar2 + 0x10c);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  uStack10 = CONCAT22(param_3,param_2);
  pass1_1038_2c82(uVar2,uVar3,*(ulong *)(uVar2 + 0x110),CONCAT22(param_3,param_2),uStack6,param_4,param_5,
                  (int)&USHORT_1050_1028,param_6,param_7);
  pass1_1038_2c82(uVar2,uVar3,*(ulong *)(uVar2 + 0x114),uStack6,uStack10,param_4,param_5,(int)&USHORT_1050_1028,param_6,
                  param_7);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1038_2b2e(ulong param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  ushort uVar2;
  undefined2 unaff_SI;
  undefined2 unaff_DI;
  ushort uVar3;
  undefined2 unaff_SS;
  ulong uStack6;
  
  uVar3 = (ushort)(param_1 >> 0x10);
  uVar2 = (ushort)param_1;
  uVar1 = *(undefined4 *)(uVar2 + 0x108);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  uStack6 = CONCAT22(param_3,param_2);
  uVar1 = *(undefined4 *)(uVar2 + 0x10c);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  pass1_1038_2f92(uVar2,uVar3,*(ulong *)(uVar2 + 0x110),CONCAT22(param_3,param_2),unaff_SI,unaff_DI,unaff_SS);
  pass1_1038_2f92(uVar2,uVar3,*(ulong *)(uVar2 + 0x114),uStack6,unaff_SI,unaff_DI,unaff_SS);
  return 0x1;
}



void __stdcall16far pass1_1038_2b9a(ulong param_1,astruct_422 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_421 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x118,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  iVar5 = (astruct_421 *)param_1;
  uVar6 = (undefined2)(param_1 >> 0x10);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    param_2->field_0x4 = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = &param_2->field_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = (int)&USHORT_1050_1028;
    param_2->field_0x108 = iVar5->field_0x108;
    param_2->field_0x10c = iVar5->field_0x10c;
    param_2->field_0x110 = iVar5->field_0x110;
    param_2->field_0x114 = iVar5->field_0x114;
    *puStack10 = 0x309a;
    param_2->field_0x2 = (int)&PTR_LOOP_1050_1038;
  }
  iVar5->field_0x114 = 0x0;
  iVar5->field_0x110 = 0x0;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_2c82(ushort param_1,ushort param_2,ulong param_3,ulong param_4,ulong param_5,ushort param_6,ushort param_7,
               ushort param_8,ushort param_9,uchar param_10)

{
  uint *puVar1;
  int *piVar2;
  uint uVar3;
  uint uVar4;
  undefined4 uVar5;
  long lVar6;
  code **ppcVar7;
  uint uVar8;
  undefined4 *puVar9;
  int iVar10;
  ulong uVar11;
  uchar *puVar12;
  uchar *puVar13;
  ushort uVar14;
  int iVar16;
  astruct_702 *iVar15;
  ushort uVar17;
  undefined2 uVar18;
  uchar *puVar19;
  ushort *puVar20;
  undefined uVar21;
  undefined4 uStack22;
  ulong local_12;
  ushort *puStack14;
  undefined4 uStack10;
  undefined4 uStack6;
  
  uVar17 = (ushort)(param_5 >> 0x10);
  uVar14 = (ushort)param_5;
  uStack6 = *(ulong *)(uVar14 + 0x200);
  uVar18 = (undefined2)(param_4 >> 0x10);
  iVar16 = (int)param_4;
  uStack10 = *(undefined4 *)(iVar16 + 0x200);
  puVar13 = *(uchar **)(iVar16 + 0x202);
  puVar19 = (uchar *)(param_3 >> 0x10);
  iVar15 = (astruct_702 *)param_3;
  iVar10 = iVar15->field_0xc;
  if (iVar10 == 0x1) {
    puStack14 = (ushort *)param_3;
    pass1_1038_52b8(param_5,*(ulong *)&iVar15->field_0x8,*(ushort *)&iVar15->field_0xe,param_6,param_7,param_8,param_9);
    return;
  }
  if (iVar10 == 0x2) {
    puStack14 = (ushort *)param_3;
    if (iVar15->field_0xe != 0x0) {
      pass1_1038_3efc(uVar14,uVar17,param_4,iVar15->field_0xe,(int)iVar15,(ushort)puVar19);
      return;
    }
    pass1_1020_a43e(param_9,puVar19,(ushort *)CONCAT22(param_9,&local_12));
    uStack22 = *(long *)((int)puStack14 + 0x8);
    while( true ) {
      uStack22 = uStack22 + -0x1;
      if ((uStack22._2_2_ | (uint)uStack22) == 0x0) break;
      pass1_1020_a6ee(CONCAT13((char)(param_9 >> 0x8),CONCAT12((char)param_9,&local_12)),
                      *(ushort *)((int)puStack14 + 0x12),(uint)&local_12,uStack22._2_2_ | (uint)uStack22,param_7,param_9
                      ,param_10);
    }
  }
  else {
    if (iVar10 == 0x3) {
      pass1_1038_3f38((ulong *)param_5,(ulong *)param_4,iVar15->field_0xe,0x0,(ushort)puVar13);
      return;
    }
    uStack6._2_2_ = (uint)(uStack6 >> 0x10);
    if (iVar10 == 0x4) {
      PTR_LOOP_1050_5f2e = (undefined *)(uStack6._2_2_ & 0xff);
      if (((int)uStack6 == 0x1) && ((uStack6 & 0xff0000) == 0x0)) {
        local_12 = *(ulong *)(uVar14 + 0x1f6);
        pass1_1030_3694(local_12,*(int *)&iVar15->field_0xe,*(long *)&iVar15->field_0x8,(uchar *)0x0,0x1030,param_9);
        *(undefined2 *)((int)&iVar15->field_0xe + 0x2) = (int)local_12;
        iVar15->field_0x12 = PTR_LOOP_1050_5f2e;
      }
      else {
        if (_PTR_LOOP_1050_5f2c == 0x0) {
          PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
        }
        else {
        }
        uVar14 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
        *(ushort *)((int)&iVar15->field_0xe + 0x2) = uVar14;
        iVar15->field_0x12 = PTR_LOOP_1050_5f2e;
        iVar10 = *(int *)&iVar15->field_0xe;
        if (iVar10 != 0x3) {
          if (iVar10 != 0x4) {
            uVar5 = *(undefined4 *)((int)&iVar15->field_0xe + 0x2);
            *(undefined4 *)((int)uVar5 + 0x28) = *(undefined4 *)&iVar15->field_0x8;
            return;
          }
          uVar5 = *(undefined4 *)((int)&iVar15->field_0xe + 0x2);
          *(undefined4 *)((int)uVar5 + 0xdc) = *(undefined4 *)&iVar15->field_0x8;
          return;
        }
        uVar5 = *(undefined4 *)((int)&iVar15->field_0xe + 0x2);
        *(undefined4 *)((int)uVar5 + 0x64) = *(undefined4 *)&iVar15->field_0x8;
      }
    }
    else {
      if (iVar10 == 0x5) {
        if (*(int *)&iVar15->field_0xe == 0xc) {
          if (((int)uStack6 == 0x1) && ((uStack6 & 0xff0000) == 0x0)) {
            uVar5 = *(undefined4 *)(uVar14 + 0x1f6);
            uVar3 = iVar15->field_0x8;
            iVar10 = iVar15->field_0xa;
            uVar8 = -uVar3;
            uVar18 = (undefined2)((ulong)uVar5 >> 0x10);
            iVar16 = (int)uVar5;
            puVar1 = (uint *)(iVar16 + 0x170);
            uVar4 = *puVar1;
            *puVar1 = *puVar1 + uVar8;
            piVar2 = (int *)(iVar16 + 0x172);
            *piVar2 = (*piVar2 - (iVar10 + (uint)(uVar3 != 0x0))) + (uint)CARRY2(uVar4,uVar8);
          }
        }
        else {
          pass1_1038_43cc(uVar14,uVar17,iVar15->field_0x8,*(uint *)&iVar15->field_0xe,(int)iVar15,(int)puVar19);
        }
      }
      else {
        iVar10 = iVar10 + -0x7;
        if (iVar10 == 0x0) {
          lVar6 = iVar15->field_0xe;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)lVar6,(uint)((ulong)lVar6 >> 0x10));
          puVar12 = puVar13;
          pass1_1038_349e(CONCAT22(puVar13,iVar10),*(ulong *)(iVar16 + 0x200));
          uVar21 = (undefined)((uint)puVar13 >> 0x8);
          pass1_1038_4d0e(CONCAT13(uVar21,CONCAT12((char)puVar13,iVar10)),0x258);
          pass1_1038_4d0e(CONCAT13(uVar21,CONCAT12((char)puVar13,iVar10)),0x258);
          puVar20 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_9,puVar12,param_7);
          puVar13 = (uchar *)((ulong)puVar20 >> 0x10);
          pass1_1008_de58((ulong)puVar20,iVar15->field_0xe,0x4000001,param_9);
          puVar20 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_9,puVar13,param_7);
          puVar13 = (uchar *)((ulong)puVar20 >> 0x10);
          uVar11 = *(ulong *)((int)puVar20 + 0x20);
          pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),uVar11);
          local_12 = uVar11 & 0xffff | ZEXT24(puVar13) << 0x10;
          uVar14 = pass1_1030_5b00(uVar11 & 0xffff | ZEXT24(puVar13) << 0x10);
          puStack14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uVar14,param_9,puVar13,param_7);
          puVar9 = (undefined4 *)((int)puStack14 + 0x20);
          ppcVar7 = (code **)((int)*puVar9 + 0x4);
          (**ppcVar7)(0x1010,puVar9,(char)((ulong)puStack14 >> 0x10),0x6);
        }
      }
    }
  }
  return;
}



void __stdcall16far
pass1_1038_2f92(ushort param_1,ushort param_2,ulong param_3,ulong param_4,ushort param_5,ushort param_6,ushort param_7)

{
  uint *puVar1;
  int *piVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  ulong uVar6;
  undefined4 uVar7;
  int iVar8;
  int iVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  int iStack10;
  
  uVar10 = (undefined2)(param_4 >> 0x10);
  iVar8 = (int)param_4;
  uVar6 = *(ulong *)(iVar8 + 0x200);
  uVar11 = (undefined2)(param_3 >> 0x10);
  iVar9 = (int)param_3;
  iVar3 = *(int *)(iVar9 + 0xc);
  if (iVar3 == 0x1) {
    uVar7 = *(undefined4 *)(iVar9 + 0x8);
    pass1_1038_3cc0(param_4,(uint)uVar7,(uchar *)((ulong)uVar7 >> 0x10),*(ushort *)(iVar9 + 0xe),param_5,param_6,param_7
                   );
    return;
  }
  if (iVar3 == 0x4) {
    pass1_1030_355c(*(ulong *)(iVar8 + 0x1f6),*(ulong *)(iVar9 + 0x10));
    return;
  }
  if (iVar3 == 0x5) {
    if (*(int *)(iVar9 + 0xe) != 0xc) {
      pass1_1038_5798(param_4,*(long *)(iVar9 + 0x8),*(int *)(iVar9 + 0xe));
      return;
    }
    iStack10 = (int)uVar6;
    if ((iStack10 == 0x1) && ((uVar6 & 0xff0000) == 0x0)) {
      uVar7 = *(undefined4 *)(iVar8 + 0x1f6);
      uVar4 = *(uint *)(iVar9 + 0x8);
      iVar3 = *(int *)(iVar9 + 0xa);
      uVar10 = (undefined2)((ulong)uVar7 >> 0x10);
      iVar8 = (int)uVar7;
      puVar1 = (uint *)(iVar8 + 0x170);
      uVar5 = *puVar1;
      *puVar1 = *puVar1 + uVar4;
      piVar2 = (int *)(iVar8 + 0x172);
      *piVar2 = *piVar2 + iVar3 + (uint)CARRY2(uVar5,uVar4);
      return;
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_3074(astruct_18 *param_1,byte param_2)

{
  pass1_1038_2a5c(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1038_30aa(ushort *param_1,ushort param_2)

{
  uint *puVar1;
  uchar *puVar2;
  uchar *puVar3;
  uint uVar4;
  astruct_423 *iVar5;
  undefined2 uVar5;
  ushort *puVar6;
  
  puVar6 = struct_1030_17ce(param_1,0x0,0x0);
  puVar2 = (uchar *)((ulong)puVar6 >> 0x10);
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_423 *)param_1;
  iVar5->field_0x10 = 0x0;
  iVar5->field_0x14 = 0x0;
  iVar5->field_0x18 = 0x258;
  iVar5->field_0x1a = 0x258;
  iVar5->field_0x1c = 0x0;
  iVar5->field_0x1e = 0x0;
  iVar5->field_0x22 = 0x0;
  iVar5->field_0x24 = 0x32;
  *(undefined4 *)&iVar5->field_0x1f6 = 0x0;
  iVar5->field_0x1fa = 0x0;
  iVar5->field_0x1fe = 0x0;
  iVar5->field_0x200 = 0x8000001;
  iVar5->field_0x204 = 0x0;
  iVar5->field_0x206 = 0x0;
  iVar5->field_0x208 = 0x1;
  iVar5->field_0x20a = 0x0;
  iVar5->field_0x20c = 0x0;
  iVar5->field_0x20e = 0x0;
  iVar5->field_0x210 = 0x0;
  iVar5->field_0x214 = 0x0;
  iVar5->field_0x216 = 0x0;
  iVar5->field_0x21a = 0x0;
  *param_1 = 0x6504;
  iVar5->field_0x2 = (int)&PTR_LOOP_1050_1038;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x26),(WNDCLASS16 *)0x0,0x94);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0xba),(WNDCLASS16 *)0x0,0x94);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x14e),(WNDCLASS16 *)0x0,0x54)
  ;
  puVar1 = pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x1a2),
                           (WNDCLASS16 *)0x0,0x54);
  mem_op_1000_179c(0x1b0,puVar2,0x1000);
  puVar3 = (uchar *)((uint)puVar2 | (uint)puVar1);
  if (puVar3 == (uchar *)0x0) {
    *(undefined4 *)&iVar5->field_0x1f6 = 0x0;
  }
  else {
    pass1_1030_314c((ushort *)CONCAT22(puVar2,puVar1),iVar5->field_0x4,puVar3,param_2);
    iVar5->field_0x1f6 = puVar1;
    iVar5->field_0x1f8 = puVar3;
  }
  mem_op_1000_179c(0x1e,puVar3,0x1000);
  uVar4 = (uint)puVar3 | (uint)puVar1;
  if (uVar4 == 0x0) {
    puVar1 = (uint *)0x0;
    uVar4 = 0x0;
  }
  else {
    struct_1020_c444((astruct_75 *)CONCAT22(puVar3,puVar1),0x64,0xc8);
  }
  iVar5->field_0xc = puVar1;
  iVar5->field_0xe = uVar4;
  return;
}



void __stdcall16far
pass1_1038_3222(ushort *param_1,ulong param_2,ulong param_3,uint param_4,uchar *param_5,uchar param_6,uchar *param_7)

{
  uint *puVar1;
  uchar *puVar2;
  uint uVar3;
  uint uVar4;
  astruct_363 *iVar5;
  undefined2 uVar5;
  ushort *puVar6;
  uchar local_16 [0x14];
  
  puVar6 = pass1_1030_183c(param_1,0x0,0x0,0x4000000,param_3,param_4,param_5);
  puVar2 = (uchar *)((ulong)puVar6 >> 0x10);
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_363 *)param_1;
  iVar5->field_0x10 = param_2;
  iVar5->field_0x14 = 0x0;
  iVar5->field_0x18 = 0x258;
  iVar5->field_0x1a = 0x258;
  iVar5->field_0x1c = 0x0;
  iVar5->field_0x1e = 0x0;
  iVar5->field_0x22 = 0x0;
  iVar5->field_0x24 = 0x32;
  *(undefined4 *)&iVar5->field_0x1f6 = 0x0;
  *(undefined4 *)&iVar5->field_0x1fa = 0x0;
  iVar5->field_0x1fe = 0x0;
  iVar5->field_0x200 = 0x8000001;
  iVar5->field_0x204 = 0x0;
  iVar5->field_0x206 = 0x0;
  iVar5->field_0x208 = 0x1;
  iVar5->field_0x20a = 0x0;
  iVar5->field_0x20c = 0x0;
  iVar5->field_0x20e = 0x0;
  iVar5->field_0x210 = 0x0;
  iVar5->field_0x214 = 0x0;
  iVar5->field_0x216 = 0x0;
  iVar5->field_0x21a = 0x0;
  *param_1 = 0x6504;
  iVar5->field_0x2 = (int)&PTR_LOOP_1050_1038;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x26),(WNDCLASS16 *)0x0,0x94);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0xba),(WNDCLASS16 *)0x0,0x94);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x14e),(WNDCLASS16 *)0x0,0x54)
  ;
  puVar1 = pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar5->field_0x1a2),
                           (WNDCLASS16 *)0x0,0x54);
  mem_op_1000_179c(0x1b0,puVar2,0x1000);
  uVar3 = (uint)puVar2 | (uint)puVar1;
  if (uVar3 == 0x0) {
    *(undefined4 *)&iVar5->field_0x1f6 = 0x0;
  }
  else {
    pass1_1030_314c((ushort *)CONCAT22(puVar2,puVar1),*(ulong *)&iVar5->field_0x4,uVar3,param_7);
    iVar5->field_0x1f6 = puVar1;
    iVar5->field_0x1f8 = uVar3;
  }
  puVar2 = (uchar *)(iVar5->field_0x6 & 0xff);
  sys_1000_3f9c(local_16,param_7,0x5a1a,(ushort)&USHORT_1050_1050,(ushort)*(undefined4 *)&iVar5->field_0x4,&stack0xfffe,
                uVar5,0x1000,param_7,param_6);
  uVar3 = str_op_1008_60e8((char *)CONCAT22(param_7,local_16),(ushort)puVar2);
  iVar5->field_0x1fa = uVar3;
  iVar5->field_0x1fc = puVar2;
  mem_op_1000_179c(0x1e,puVar2,0x1000);
  uVar4 = (uint)puVar2 | uVar3;
  if (uVar4 == 0x0) {
    *(undefined4 *)&iVar5->field_0xc = 0x0;
  }
  else {
    struct_1020_c444((astruct_75 *)CONCAT22(puVar2,uVar3),0x64,0xc8);
    iVar5->field_0xc = uVar3;
    iVar5->field_0xe = uVar4;
  }
  return;
}



void __stdcall16far pass1_1038_33f8(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x6504;
  *(undefined2 *)(iVar4 + 0x2) = (int)&PTR_LOOP_1050_1038;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x14);
  uVar2 = *(uint *)(iVar4 + 0x16);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x1f6);
  uVar2 = *(uint *)(iVar4 + 0x1f8);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar4 + 0x1fa),0x1000);
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x210);
  uVar2 = *(uint *)(iVar4 + 0x212);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1000,puVar1,uVar2,0x1);
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar4 + 0x21a),0x1000);
  pass1_1030_18b2(param_1);
  return;
}
