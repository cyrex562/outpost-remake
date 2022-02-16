
void __stdcall16far file_1008_7548(undefined4 param_1,long *param_2,HFILE16 param_3,ushort param_4)

{
  code **ppcVar1;
  ushort uVar2;
  BOOL16 BVar3;
  uint uVar4;
  ulong uVar5;
  uint uVar6;
  ushort uVar7;
  undefined4 local_1c;
  undefined2 local_18 [0x5];
  ulong uStack14;
  ulong uStack10;
  undefined4 local_6;
  
  local_6 = 0x0;
  uVar7 = (ushort)param_1;
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  BVar3 = read_file_1008_7dee(uVar7,uVar2,(ushort)&local_6,0x0,param_4,0x4,param_3);
  if (BVar3 == 0x0) {
    return;
  }
  if (local_6 != 0x0) {
    uVar5 = local_6;
    if (local_6 < 0xc8) {
      local_6._2_2_ = (uchar *)0x0;
      uVar5 = 0xc8;
    }
    uVar4 = (uint)uVar5;
    uStack10 = uVar5 & 0xffff | ZEXT24(local_6._2_2_) << 0x10;
    if (*param_2 == 0x0) {
      param_3 = 0x1000;
      mem_op_1000_179c(0x1e,local_6._2_2_,0x1000);
      uVar6 = (uint)local_6._2_2_ | uVar4;
      if (uVar6 == 0x0) {
        *param_2 = 0x0;
      }
      else {
        param_3 = 0x1020;
        struct_1020_c444((astruct_75 *)CONCAT22(local_6._2_2_,uVar4),0x64,uStack10);
        *(uint *)param_2 = uVar4;
        *(uint *)((int)param_2 + 0x2) = uVar6;
      }
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x24);
    (**ppcVar1)(param_3,*param_2);
    for (uStack14 = 0x0; uStack14 < local_6; uStack14 = uStack14 + 0x1) {
      BVar3 = read_file_1008_7dee(uVar7,uVar2,(ushort)&local_1c,0x0,param_4,0x4,param_3);
      if ((BVar3 == 0x0) ||
         (BVar3 = read_file_1008_7dee(uVar7,uVar2,(ushort)local_18,0x0,param_4,0x2,param_3), BVar3 == 0x0)) {
        ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x1c);
        (**ppcVar1)(param_3,(char)*param_2,(int)((ulong)*param_2 >> 0x10));
        return;
      }
      ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x28);
      (**ppcVar1)(param_3,(int)*param_2,(char)((ulong)*param_2 >> 0x10),local_18[0],(char)local_1c,
                  (int)((ulong)local_1c >> 0x10));
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x1c);
    (**ppcVar1)(param_3,(char)*param_2,(int)((ulong)*param_2 >> 0x10));
  }
  return;
}



void __stdcall16far pass1_1008_766e(ulong param_1,ulong *param_2,ushort param_3,ushort param_4,uchar *param_5)

{
  ulong *puVar1;
  uchar *puVar2;
  ulong local_6;
  
  *param_2 = 0x0;
  local_6 = 0x0;
  puVar1 = &local_6;
  file_1008_76e4(param_1,(long *)CONCAT22(param_3,puVar1),param_4,param_3,(ushort)param_5);
  if (puVar1 != (ulong *)0x0) {
    if (local_6 != 0x0) {
      mem_op_1000_179c(0xc,param_5,0x1000);
      puVar2 = (uchar *)((uint)param_5 | (uint)puVar1);
      if (puVar2 == (uchar *)0x0) {
        puVar1 = (ulong *)0x0;
        puVar2 = (uchar *)0x0;
      }
      else {
        pass1_1010_8ef2((ushort *)CONCAT22(param_5,puVar1),puVar2,param_3);
      }
      *(ulong **)param_2 = puVar1;
      *(uchar **)((int)param_2 + 0x2) = puVar2;
      fn_ptr_1010_905e(*param_2,local_6);
    }
    return;
  }
  return;
}



void __stdcall16far file_1008_76e4(ulong param_1,long *param_2,ushort param_3,ushort param_4,ushort param_5)

{
  code **ppcVar1;
  uint uVar2;
  BOOL16 BVar3;
  undefined2 extraout_DX;
  ushort uVar4;
  undefined local_18 [0xe];
  ulong uStack10;
  ulong local_6;
  
  local_6 = 0x0;
  uVar4 = (ushort)(param_1 >> 0x10);
  uVar2 = read_file_1008_7dee((ushort)param_1,uVar4,(ushort)&local_6,0x0,param_4,0x4,param_3);
  if (uVar2 == 0x0) {
    return;
  }
  if (local_6 != 0x0) {
    if (*param_2 == 0x0) {
      param_3 = 0x1000;
      mem_op_1000_179c(0x18,(uchar *)param_5,0x1000);
      if ((param_5 | uVar2) == 0x0) {
        *param_2 = 0x0;
      }
      else {
        param_3 = 0x1030;
        struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_5,uVar2),0x5,local_6);
        *(uint *)param_2 = uVar2;
        *(undefined2 *)((int)param_2 + 0x2) = extraout_DX;
      }
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x14);
    (**ppcVar1)(param_3,(int)*param_2,(int)((ulong)*param_2 >> 0x10),local_6);
    for (uStack10 = 0x0; uStack10 < local_6; uStack10 = uStack10 + 0x1) {
      BVar3 = read_file_1008_7dee((ushort)param_1,uVar4,(ushort)local_18,0x0,param_4,0x4,param_3);
      if (BVar3 == 0x0) {
        return;
      }
      ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x18);
      (**ppcVar1)();
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*param_2 + 0x1c);
    (**ppcVar1)();
  }
  return;
}



undefined2 __stdcall16far file_1008_77cc(ulong param_1,long *param_2,uchar *param_3,HFILE16 param_4,ushort param_5)

{
  uint uVar1;
  BOOL16 BVar2;
  uint uVar3;
  ushort unaff_SI;
  ushort unaff_DI;
  ushort uVar4;
  ushort uVar5;
  undefined2 local_14 [0x2];
  undefined4 local_10 [0x2];
  uint uStack6;
  uint local_4;
  
  local_4 = 0x0;
  uVar4 = (ushort)param_1;
  uVar5 = (ushort)(param_1 >> 0x10);
  uVar1 = read_file_1008_7dee(uVar4,uVar5,(ushort)&local_4,0x0,param_5,0x2,param_4);
  if (uVar1 == 0x0) {
    return 0x0;
  }
  if (local_4 != 0x0) {
    if (*param_2 == 0x0) {
      param_4 = 0x1000;
      mem_op_1000_179c(0xa,param_3,0x1000);
      uVar3 = (uint)param_3 | uVar1;
      if (uVar3 == 0x0) {
        *param_2 = 0x0;
      }
      else {
        param_4 = 0x1020;
        pass1_1020_ba3e((long *)CONCAT22(param_3,uVar1),0x5,0x5,unaff_DI,unaff_SI);
        *(uint *)param_2 = uVar1;
        *(uint *)((int)param_2 + 0x2) = uVar3;
      }
    }
    for (uStack6 = 0x0; uStack6 < local_4; uStack6 = uStack6 + 0x1) {
      BVar2 = read_file_1008_7dee(uVar4,uVar5,(ushort)local_14,0x0,param_5,0x2,param_4);
      if (BVar2 == 0x0) {
        return 0x0;
      }
      BVar2 = read_file_1008_7dee(uVar4,uVar5,(ushort)local_10,0x0,param_5,0x4,param_4);
      if (BVar2 == 0x0) {
        return 0x0;
      }
      param_4 = 0x1020;
      pass1_1020_bb8a((long *)*param_2,(uint)local_10[0],CONCAT22(local_14[0],(int)((ulong)local_10[0] >> 0x10)),
                      unaff_DI,param_5);
    }
  }
  return 0x1;
}



void __stdcall16far
pass1_1008_7898(ulong param_1,ulong *param_2,ushort param_3,ushort param_4,HFILE16 param_5,ushort param_6)

{
  code **ppcVar1;
  BOOL16 BVar2;
  undefined2 extraout_DX;
  undefined2 uVar3;
  ushort uVar4;
  ushort uVar5;
  undefined2 local_26;
  undefined4 local_24 [0x3];
  undefined4 local_18;
  undefined2 local_14 [0x5];
  ulong uStack10;
  ulong uStack6;
  
  if (param_2 == (ulong *)0x0) {
    param_3 = 0x0;
    uVar3 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*param_2 + 0x10);
    (**ppcVar1)();
    uVar3 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar3,param_3);
  local_18 = CONCAT22(uVar3,param_3);
  uVar4 = (ushort)param_1;
  uVar5 = (ushort)(param_1 >> 0x10);
  BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)&local_18,param_6,(char *)0x4,param_5);
  if (BVar2 != 0x0) {
    uStack10 = 0x0;
    while( true ) {
      if (uStack6 <= uStack10) {
        return;
      }
      pass1_1020_c4a8((ulong)param_2,(ushort *)CONCAT22(param_6,local_14),(ulong *)CONCAT22(param_6,&local_18),
                      (int)uStack10,param_4,param_6);
      local_24[0] = local_18;
      BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)local_24,param_6,(char *)0x4,0x1020);
      if (BVar2 == 0x0) break;
      local_26 = local_14[0];
      BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,(ushort)&local_26,param_6,(char *)0x2,0x1020);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
      }
      uStack10 = uStack10 + 0x1;
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  return;
}



uint16_t __stdcall16far
write_to_file_1008_7954(undefined4 param_1,undefined4 *param_2,uint16_t param_3,HFILE16 param_4,uint16_t param_5)

{
  code **ppcVar1;
  BOOL16 BVar2;
  ulong uVar3;
  uint16_t extraout_DX;
  uint16_t uVar4;
  uint16_t extraout_DX_00;
  ushort uVar5;
  uint16_t local_20;
  uint16_t uStack30;
  uint16_t local_18;
  uint16_t uStack22;
  ulong uStack10;
  ulong uStack6;
  
  if (param_2 == (undefined4 *)0x0) {
    param_3 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*param_2 + 0x10);
    (**ppcVar1)(param_4,param_2);
    uVar4 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar4,param_3);
  uVar5 = (ushort)((ulong)param_1 >> 0x10);
  local_18 = param_3;
  uStack22 = uVar4;
  BVar2 = write_to_file_1008_7e1c((ushort)param_1,uVar5,(ushort)&local_18,param_5,(char *)0x4,param_4);
  if (BVar2 != 0x0) {
    uStack10 = 0x0;
    while( true ) {
      if (uStack6 <= uStack10) {
        return uVar4;
      }
      ppcVar1 = (code **)((int)*param_2 + 0x4);
      uVar3 = uStack6;
      (**ppcVar1)();
      local_20 = (uint16_t)uVar3;
      uVar4 = extraout_DX_00;
      uStack30 = extraout_DX_00;
      local_18 = local_20;
      uStack22 = extraout_DX_00;
      BVar2 = write_to_file_1008_7e1c((ushort)param_1,uVar5,(ushort)&local_20,param_5,(char *)0x4,param_4);
      if (BVar2 == 0x0) break;
      uStack10 = uStack10 + 0x1;
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  return uVar4;
}



void __stdcall16far pass1_1008_79f0(ulong param_1,long param_2,HFILE16 param_3,uint16_t param_4)

{
  uint16_t uVar1;
  undefined2 uVar2;
  undefined2 uStack4;
  
  if (param_2 == 0x0) {
    uVar1 = 0x0;
    uStack4 = 0x0;
  }
  else {
    uVar2 = (undefined2)((ulong)param_2 >> 0x10);
    uVar1 = *(uint16_t *)((int)param_2 + 0x4);
    uStack4 = *(undefined2 *)((int)param_2 + 0x6);
  }
  write_to_file_1008_7954(param_1,(undefined4 *)CONCAT22(uStack4,uVar1),uVar1,param_3,param_4);
  return;
}



void __stdcall16far write_to_file_1008_7a22(undefined4 param_1,long param_2,HFILE16 param_3,uint16_t param_4)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  undefined4 local_24 [0x2];
  uint local_1c [0x5];
  uint local_12;
  undefined4 local_10;
  uint uStack10;
  uint uStack6;
  uint uStack4;
  
  if (param_2 == 0x0) {
    uStack4 = 0x0;
  }
  else {
    uStack4 = *(uint *)((int)param_2 + 0x4);
  }
  local_12 = uStack4;
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)((ulong)param_1 >> 0x10);
  BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)&local_12,param_4,(char *)0x2,param_3);
  if (BVar1 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  else {
    uStack6 = 0x0;
    while( true ) {
      if (uStack4 <= uStack6) {
        return;
      }
      pass1_1020_bb16((ulong *)param_2,(ulong *)CONCAT22(param_4,&local_10),(ushort *)CONCAT22(param_4,&local_12),
                      uStack6);
      uStack10 = local_12;
      local_1c[0] = local_12;
      BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)local_1c,param_4,(char *)0x2,0x1020);
      if (BVar1 == 0x0) break;
      local_24[0] = local_10;
      BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)local_24,param_4,(char *)0x4,0x1020);
      if (BVar1 == 0x0) {
        return;
      }
      uStack6 = uStack6 + 0x1;
    }
  }
  return;
}



ushort __stdcall16far pass1_1008_7ad4(ulong param_1,long *param_2,ushort param_3,HFILE16 param_4,ushort param_5)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  undefined2 local_14 [0x2];
  undefined4 local_10 [0x2];
  uint uStack6;
  uint local_4;
  
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)(param_1 >> 0x10);
  BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)&local_4,0x0,param_5,0x2,param_4);
  if (BVar1 != 0x0) {
    uStack6 = 0x0;
    while( true ) {
      if (local_4 <= uStack6) {
        return 0x1;
      }
      BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)local_14,0x0,param_5,0x2,param_4);
      if ((BVar1 == 0x0) ||
         (BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)local_10,0x0,param_5,0x4,param_4), BVar1 == 0x0)) break;
      param_4 = 0x1020;
      pass1_1020_bb8a(param_2,(uint)local_10[0],CONCAT22(local_14[0],(int)((ulong)local_10[0] >> 0x10)),param_3,param_5)
      ;
      uStack6 = uStack6 + 0x1;
    }
  }
  return 0x0;
}



undefined2 __stdcall16far write_to_file_1008_7b4c(undefined4 param_1,ushort *param_2,HFILE16 param_3,uint16_t param_4)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  undefined2 local_12 [0x3];
  undefined2 local_c [0x2];
  undefined2 local_8;
  undefined2 local_6;
  undefined2 local_4;
  
  pass1_1008_3eb4(param_2,(ushort *)CONCAT22(param_4,&local_8),(ushort *)CONCAT22(param_4,&local_6),
                  (ushort *)CONCAT22(param_4,&local_4));
  local_12[0] = local_4;
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)((ulong)param_1 >> 0x10);
  BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)local_12,param_4,(char *)0x2,param_3);
  if (BVar1 != 0x0) {
    local_c[0] = local_6;
    BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)local_c,param_4,(char *)0x2,param_3);
    if (BVar1 != 0x0) {
      local_c[0] = local_8;
      BVar1 = write_to_file_1008_7e1c(uVar2,uVar3,(ushort)local_c,param_4,(char *)0x2,param_3);
      if (BVar1 != 0x0) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



BOOL16 __stdcall16far read_file_1008_7bc8(ulong param_1,ushort *param_2,HFILE16 param_3,ushort param_4)

{
  BOOL16 BVar1;
  ushort uVar2;
  ushort uVar3;
  ushort local_8;
  undefined4 local_6;
  
  uVar2 = (ushort)param_1;
  uVar3 = (ushort)(param_1 >> 0x10);
  BVar1 = read_file_1008_7dee(uVar2,uVar3,(int)&local_6 + 0x2,0x0,param_4,0x2,param_3);
  if (BVar1 != 0x0) {
    BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)&local_6,0x0,param_4,0x2,param_3);
    if (BVar1 != 0x0) {
      BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)&local_8,0x0,param_4,0x2,param_3);
      if (BVar1 != 0x0) {
        pass1_1008_3e76(param_2,local_8,(ushort)local_6,(ushort)((ulong)local_6 >> 0x10));
        return 0x1;
      }
    }
  }
  return 0x0;
}



BOOL16 __stdcall16far pass1_1008_7c2a(ulong param_1,char *param_2,HFILE16 param_3)

{
  uint uVar1;
  BOOL16 BVar2;
  ushort uVar3;
  
  uVar3 = (ushort)(param_1 >> 0x10);
  if (param_2 != (char *)0x0) {
    uVar1 = str_op_1000_3da4(param_2);
    BVar2 = write_to_file_1008_7e1c
                      ((ushort)param_1,uVar3,(ushort)param_2,(ushort)((ulong)param_2 >> 0x10),
                       (char *)(long)(int)(uVar1 + 0x1),0x1000);
    return BVar2;
  }
  write_to_file_1008_7e1c
            ((ushort)param_1,uVar3,(int)s_playerName_1050_148e + 0xc,(ushort)&USHORT_1050_1050,(char *)0x1,param_3);
  return 0x1;
}



void __stdcall16far read_file_1008_7c6e(ushort param_1,ushort param_2,char *param_3,HFILE16 param_4)

{
  char *pcVar1;
  char local_c [0xa];
  
  while( true ) {
    pcVar1 = param_3;
    WIN16_hread(param_4,0x1,ZEXT24(local_c) << 0x10);
    if (local_c[0] == '\0') break;
    param_3 = (char *)((ulong)param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x1));
    *pcVar1 = local_c[0];
    param_4 = (HFILE16)s_tile2_bmp_1050_1538;
  }
  *param_3 = '\0';
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far write_to_file_1008_7cac(undefined4 param_1,uint16_t param_2)

{
  uint uVar1;
  BOOL16 BVar2;
  undefined2 unaff_ES;
  undefined1 in_AF;
  uchar local_c [0xa];
  
  sys_1000_3f9c(local_c,(uchar *)param_2,0x340,(ushort)&USHORT_1050_1050,(ushort)_PTR_s_dcbSC_1050_0336_1050_033c,
                &stack0xfffe,unaff_ES,0x1000,param_2,in_AF);
  uVar1 = str_op_1000_3da4((char *)CONCAT22(param_2,local_c));
  BVar2 = write_to_file_1008_7e1c
                    ((ushort)param_1,(ushort)((ulong)param_1 >> 0x10),(ushort)local_c,param_2,(char *)(ulong)uVar1,
                     0x1000);
  if (BVar2 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return BVar2;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
read_file_1008_7cfe(undefined2 param_1,undefined2 param_2,undefined2 param_3,uint16_t param_4,uint16_t param_5)

{
  bool bVar1;
  ushort uVar2;
  uchar in_AF;
  long lVar3;
  ushort in_stack_0000fbd2;
  uint16_t in_stack_0000fbd4;
  undefined4 uStack1040;
  char local_406 [0x400];
  undefined4 uStack6;
  
  uStack6 = 0x0;
  bVar1 = false;
  do {
    _llseek16(param_4,uStack6 << 0x10,(INT16)((ulong)uStack6 >> 0x10));
    param_4 = (uint16_t)s_tile2_bmp_1050_1538;
    lVar3 = WIN16_hread((HFILE16)s_tile2_bmp_1050_1538,0x400,ZEXT24(local_406) << 0x10);
    for (uStack1040 = 0x0; uStack1040 < lVar3; uStack1040 = uStack1040 + 0x1) {
      if (local_406[(uint)uStack1040] == *_PTR_s_dcbSC_1050_0336_1050_033c) {
        if (!bVar1) {
          bVar1 = true;
          uStack6 = CONCAT22((int)((ulong)uStack6 >> 0x10) + uStack1040._2_2_ +
                             (uint)CARRY2((uint)uStack6,(uint)uStack1040),(uint)uStack6 + (uint)uStack1040);
          break;
        }
        bVar1 = false;
        uVar2 = pass1_1008_7e4a((ushort)((ulong)_PTR_s_dcbSC_1050_0336_1050_033c >> 0x10),(uchar *)param_5,in_AF,
                                (char *)CONCAT22(param_5,local_406 + (uint)uStack1040),in_stack_0000fbd2,
                                in_stack_0000fbd4);
        if (uVar2 != 0x0) {
          lVar3 = uStack1040 + uStack6 + 0x7;
          _llseek16((HFILE16)s_tile2_bmp_1050_1538,lVar3 * 0x10000,(INT16)((ulong)lVar3 >> 0x10));
          return;
        }
      }
    }
    if (!bVar1) {
      if (lVar3 < 0x400) {
        return;
      }
      uStack6._0_2_ = CONCAT11(uStack6._1_1_ + 0x4,(undefined)uStack6);
      uStack6 = CONCAT22((int)((ulong)uStack6 >> 0x10) + (uint)(0xfb < uStack6._1_1_),(uint)uStack6);
    }
  } while( true );
}



BOOL16 __stdcall16far
read_file_1008_7dee(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,SEGPTR param_6,
                   HFILE16 param_7)

{
  long lVar1;
  
  lVar1 = WIN16_hread(param_7,param_6,CONCAT22(param_3,param_4));
  if (lVar1 != CONCAT22(param_4,param_6)) {
    return 0x0;
  }
  return 0x1;
}



BOOL16 __stdcall16far
write_to_file_1008_7e1c
          (ushort param_1,ushort param_2,ushort param_3,ushort param_4,char *buf_to_write,HFILE16 file_handle)

{
  char *pcVar1;
  
  pcVar1 = (char *)_hwrite16(file_handle,(LPCSTR)buf_to_write,CONCAT22(param_3,(int)((ulong)buf_to_write >> 0x10)));
  if (pcVar1 != buf_to_write) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far
pass1_1008_7e4a(ushort param_1,uchar *param_2,uchar param_3,char *param_4,ushort param_5,uint16_t param_6)

{
  uint uVar1;
  
  sys_1000_3f9c((uchar *)&param_5,param_2,0x347,(ushort)&USHORT_1050_1050,(ushort)_PTR_s_dcbSC_1050_0336_1050_033c,
                &stack0xfffe,param_1,0x1000,param_2,param_3);
  uVar1 = str_op_1000_3da4((char *)CONCAT22(param_2,&param_5));
  uVar1 = pass1_1000_3de8(param_4,(char *)CONCAT22(param_2,&param_5),uVar1,param_5,param_6);
  if (uVar1 == 0x0) {
    return 0x1;
  }
  return 0x0;
}



ushort * __stdcall16far pass1_1008_7e98(ushort *param_1,byte param_2)

{
  astruct_460 *uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  uVar1 = (astruct_460 *)param_1;
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_20 * __stdcall16far unk_draw_op_1008_7f62(astruct_20 *param_1,UINT16 param_2,ULONG param_3,UINT16 param_4)

{
  HGDIOBJ16 HVar1;
  HCURSOR16 HVar2;
  astruct_20 *uVar4;
  astruct_20 *iVar3;
  
  iVar3 = (astruct_20 *)param_1;
  uVar4 = (astruct_20 *)((ulong)param_1 >> 0x10);
  set_struct_1008_687a(param_1,param_3);
  iVar3->field_0xde = param_2;
  param_1->field_0x0 = 0x8042;
  iVar3->field_0x2 = 0x1008;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x5b)),s_SOLChildPar_1050_0358);
  HVar1 = GetStockObject16(0x1000);
  iVar3->hgdiobj_field_0xc6 = HVar1;
  HVar2 = LoadCursor16((HINSTANCE16)s_tile2_bmp_1050_1538,(LPCSTR)0x7f00);
  iVar3->hcursor_field_0xc4 = HVar2;
  iVar3->field_0xc8 = 0x2008;
  iVar3->field_0xac = 0x44000000;
  iVar3->field_0xbc = *(UINT16 *)((int)param_3 + 0x8);
  iVar3->field_0xca = iVar3->field_0xde;
  win_ui_reg_class_1008_96d2(param_1,(int)s_tile2_bmp_1050_1538,param_4);
  return param_1;
}



void __stdcall16far pass1_1008_7ffa(ushort *param_1,byte param_2)

{
  astruct_461 *uVar1;
  undefined2 uVar2;
  
  uVar1 = (astruct_461 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



ulong * __stdcall16far pass1_1008_80d2(ulong *param_1)

{
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x4) = 0x0;
  return param_1;
}



astruct_23 * __stdcall16far unk_draw_op_1008_80ee(astruct_23 *param_1,UINT16 param_2)

{
  HCURSOR16 HVar1;
  HGDIOBJ16 HVar2;
  astruct_23 *iVar3;
  astruct_23 *uVar3;
  
  uVar3 = (astruct_23 *)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_23 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x54 = 0x0;
  iVar3->field_0x56 = 0x0;
  iVar3->field_0x58 = 0x0;
  param_1->field_0x0 = 0x87c8;
  iVar3->field_0x2 = 0x1008;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x4)),s_MicroSpinControl_1050_0370);
  iVar3->field_0x54 = 0x3;
  HVar1 = LoadCursor16(0x1000,(LPCSTR)0x7f00);
  iVar3->field_0x58 = HVar1;
  HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  iVar3->field_0x56 = HVar2;
  pass1_1008_818c((astruct_23 *)((ulong)param_1 & 0xffff | ZEXT24(uVar3) << 0x10),(int)s_tile2_bmp_1050_1538,param_2);
  return param_1;
}



void __stdcall16far pass1_1008_8168(ushort *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x87c8;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  return;
}



void __stdcall16far pass1_1008_818c(astruct_23 *param_1,HINSTANCE16 param_2,WNDCLASS16 *param_3)

{
  BOOL16 BVar1;
  ATOM AVar2;
  undefined2 local_1c;
  undefined2 uStack26;
  undefined2 uStack24;
  undefined4 uStack22;
  undefined *puStack18;
  undefined2 uStack16;
  undefined2 uStack14;
  undefined2 uStack12;
  undefined4 uStack10;
  int iStack6;
  undefined2 uStack4;
  
  iStack6 = (int)param_1 + 0x4;
  BVar1 = GetClassInfo16(param_2,(SEGPTR)&local_1c,param_3);
  if (BVar1 == 0x0) {
    local_1c = *(undefined2 *)((int)param_1 + 0x54);
    uStack26 = 0x84f2;
    uStack24 = 0x1008;
    uStack22 = 0x40000;
    puStack18 = PTR_LOOP_1050_038c;
    uStack16 = 0x0;
    uStack14 = *(undefined2 *)((int)param_1 + 0x58);
    uStack12 = *(undefined2 *)((int)param_1 + 0x56);
    uStack10 = 0x0;
    uStack4 = param_1._2_2_;
    AVar2 = RegisterClass16((WNDCLASS16 *)s_tile2_bmp_1050_1538);
    if (AVar2 == 0x0) {
      fn_ptr_op_1000_24cd(0x0,&stack0xfffe);
    }
  }
  return;
}



undefined2 __cdecl16far
win_ui_op_1008_8214(ushort param_1,int param_2,ushort param_3,ulong param_4,uint param_5,uchar *param_6,HWND16 param_7)

{
  INT16 IVar1;
  ulong *puVar2;
  undefined2 *puVar3;
  undefined2 uVar4;
  
  if (param_2 == 0x81) {
    uVar4 = 0x6;
    mem_op_1000_179c(0x6,param_6,0x1000);
    if (((uint)param_6 | param_5) == 0x0) {
      puVar2 = (ulong *)0x0;
    }
    else {
      puVar2 = pass1_1008_80d2((ulong *)CONCAT22(param_6,param_5));
    }
    param_7 = (HWND16)s_tile2_bmp_1050_1538;
    SetWindowLong16(0x1000,(INT16)puVar2,CONCAT22(uVar4,(int)((ulong)puVar2 >> 0x10)));
  }
  if (param_2 == 0x1) {
    puVar3 = (undefined2 *)GetWindowLong16(param_7,0x0);
    *puVar3 = *(undefined2 *)((int)param_4 + 0x8);
    IVar1 = GetDlgCtrlID16((HWND16)s_tile2_bmp_1050_1538);
    *(INT16 *)((int)puVar3 + 0x2) = IVar1;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl16far draw_op_1008_8288(ushort param_1,ulong param_2,HWND16 param_3)

{
  HGDIOBJ16 HVar1;
  HGDIOBJ16 HVar2;
  int x;
  undefined2 uVar3;
  RECT16 local_58;
  uint uStack84;
  uint uStack82;
  HBRUSH16 HStack80;
  HPEN16 HStack78;
  HPEN16 HStack76;
  HDC16 HStack74;
  uint uStack72;
  uint uStack70;
  uint uStack68;
  uint uStack66;
  uint uStack64;
  uint uStack62;
  PAINTSTRUCT16 local_3c;
  int local_1c;
  int iStack26;
  int iStack24;
  int iStack22;
  int iStack20;
  int iStack18;
  int local_10;
  int iStack14;
  int iStack12;
  int iStack10;
  int iStack8;
  int iStack6;
  uint uStack4;
  
  HStack74 = BeginPaint16(param_3,&local_3c);
  uStack4 = 0x0;
  HStack76 = CreatePen16((INT16)s_tile2_bmp_1050_1538,(INT16)_PTR_LOOP_1050_0368,
                         (COLORREF)((ulong)_PTR_LOOP_1050_0368 >> 0x10));
  HStack78 = CreatePen16((INT16)s_tile2_bmp_1050_1538,(INT16)DAT_1050_0364,(COLORREF)((ulong)DAT_1050_0364 >> 0x10));
  HStack80 = CreateSolidBrush16((COLORREF)s_tile2_bmp_1050_1538);
  GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_58);
  uStack62 = uStack84;
  uStack64 = uStack82;
  uStack66 = uStack84 >> 0x1;
  uStack68 = uStack82 >> 0x1;
  uStack70 = uStack84 >> 0x2;
  uStack72 = uStack82 >> 0x2;
  HVar1 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar1 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar1);
  HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  HVar2 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar2);
  Rectangle16((HDC16)s_tile2_bmp_1050_1538,uStack82,uStack84,local_58.y,local_58.x);
  MoveTo16((HDC16)s_tile2_bmp_1050_1538,uStack68,0x0);
  LineTo16((HDC16)s_tile2_bmp_1050_1538,uStack68,uStack62);
  uVar3 = (undefined2)(param_2 >> 0x10);
  if ((*(byte *)((int)param_2 + 0x4) & 0x4) != 0x0) {
    uStack4 = 0x1;
  }
  local_10 = uStack66 + uStack4;
  iStack14 = uStack72 + uStack4 + -0x2;
  iStack12 = local_10 + -0x3;
  iStack10 = uStack72 + uStack4 + 0x1;
  iStack8 = local_10 + 0x3;
  iStack6 = iStack10;
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack76);
  if (uStack4 == 0x0) {
    MoveTo16((HDC16)s_tile2_bmp_1050_1538,uStack68 - 0x2,0x1);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,0x1,0x1);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,0x1,uStack62 - 0x1);
  }
  uStack4 = (uint)((*(byte *)((int)param_2 + 0x4) & 0x8) != 0x0);
  local_1c = uStack66 + uStack4;
  iStack22 = (uStack64 - uStack72) + uStack4;
  iStack26 = iStack22 + 0x1;
  iStack24 = local_1c + -0x3;
  iStack22 = iStack22 + -0x2;
  iStack20 = local_1c + 0x3;
  iStack18 = iStack22;
  if (uStack4 == 0x0) {
    MoveTo16((HDC16)s_tile2_bmp_1050_1538,uStack82 - 0x2,0x1);
    x = uStack68 + 0x1;
    LineTo16((HDC16)s_tile2_bmp_1050_1538,x,0x1);
    LineTo16((HDC16)s_tile2_bmp_1050_1538,x,uStack62 - 0x1);
  }
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack78);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack80);
  Polygon16((HDC16)s_tile2_bmp_1050_1538,(POINT16 *)((int)&PTR_LOOP_1050_0002 + 0x1),(INT16)&local_10);
  Polygon16((HDC16)s_tile2_bmp_1050_1538,(POINT16 *)((int)&PTR_LOOP_1050_0002 + 0x1),(INT16)&local_1c);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar2);
  SelectObject16((HDC16)s_tile2_bmp_1050_1538,HVar1);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_3c);
  return;
}



void __cdecl16far send_msg_1008_84ba(ushort param_1,ulong param_2,HWND16 param_3)

{
  int iVar1;
  undefined2 uVar2;
  undefined2 uStack4;
  
  uVar2 = (undefined2)(param_2 >> 0x10);
  iVar1 = (int)param_2;
  if ((*(byte *)(iVar1 + 0x4) & 0x4) == 0x0) {
    if ((*(byte *)(iVar1 + 0x4) & 0x8) == 0x0) {
      return;
    }
    uStack4 = 0x1;
  }
  else {
    uStack4 = 0x0;
  }
  SendMessage16(param_3,*(UINT16 *)(iVar1 + 0x2),0x0,CONCAT22(0x115,uStack4));
  return;
}

