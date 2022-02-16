
void __stdcall16far pt_in_rect_1018_1bda(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int *piVar1;
  undefined2 uVar2;
  int iVar3;
  BOOL16 BVar4;
  int iVar5;
  undefined2 uVar6;
  int iStack26;
  POINT16 PStack24;
  int local_14;
  int local_12;
  undefined2 uStack16;
  ulong uStack14;
  int local_a;
  int local_8;
  int iStack6;
  int iStack4;
  
  uStack14 = 0x0;
  iVar3 = (int)param_1;
  pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar3 + 0x3a)),(ushort *)CONCAT22(param_4,&local_14),
                  (ushort *)CONCAT22(param_4,&local_12));
  PStack24 = (POINT16)CONCAT22(param_2,param_3);
  uStack16 = 0x0;
  iStack26 = 0x0;
  while( true ) {
    uVar6 = (undefined2)(param_1 >> 0x10);
    piVar1 = (int *)(iVar3 + 0x44);
    if (*piVar1 == iStack26 || *piVar1 < iStack26) {
      return;
    }
    uVar2 = *(undefined2 *)(iVar3 + 0x42);
    iVar5 = *(int *)(iVar3 + 0x40) + iStack26 * 0x18;
    uStack14 = CONCAT22(uVar2,iVar5);
    pass1_1008_3e94((ushort *)CONCAT22(uVar2,iVar5),(ushort *)CONCAT22(param_4,&local_8),
                    (ushort *)CONCAT22(param_4,&local_a));
    local_a = local_a + local_12 + -0x6;
    iStack6 = local_a + 0xc;
    local_8 = local_8 + local_14 + -0x6;
    iStack4 = local_8 + 0xc;
    BVar4 = PtInRect16((RECT16 *)0x1008,PStack24);
    if (BVar4 != 0x0) break;
    iStack26 = iStack26 + 0x1;
  }
  pass1_1018_1eda(param_1,uStack14,param_4);
  return;
}



ushort __stdcall16far pass1_1018_1c9a(ulong param_1,int param_2)

{
  int *piVar1;
  ulong uVar2;
  uint uVar3;
  undefined2 uVar4;
  undefined2 unaff_SS;
  int iStack10;
  
  iStack10 = 0x0;
  while( true ) {
    uVar4 = (undefined2)(param_1 >> 0x10);
    piVar1 = (int *)((int)param_1 + 0x44);
    if (*piVar1 == iStack10 || *piVar1 < iStack10) {
      return 0x0;
    }
    uVar2 = *(ulong *)((int)param_1 + 0x40);
    uVar3 = (int)uVar2 + iStack10 * 0x18;
    if (*(int *)(*(int *)(uVar3 + 0xc) * 0x1e + 0x3c32) == param_2) break;
    iStack10 = iStack10 + 0x1;
  }
  pass1_1018_1eda(param_1,uVar2 & 0xffff0000 | (ulong)uVar3,unaff_SS);
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1018_1ce8(ushort param_1,ulong param_2)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  undefined2 uVar5;
  int iStack26;
  undefined local_18 [0x2];
  undefined local_16 [0x2];
  int iStack20;
  int iStack18;
  int iStack16;
  uint local_e;
  int local_c;
  int local_a;
  int iStack8;
  undefined4 uStack6;
  
  uVar5 = (undefined2)(param_2 >> 0x10);
  iVar3 = (int)param_2;
  uStack6 = *(ulong *)(iVar3 + 0x40);
  iStack8 = 0x0;
  do {
    piVar1 = (int *)(iVar3 + 0x44);
    if (*piVar1 == iStack8 || *piVar1 < iStack8) {
      return;
    }
    pass1_1008_3eb4((ushort *)(uStack6 & 0xffff0000 | (ulong)(uint)(iStack8 * 0x18 + (int)uStack6)),
                    (ushort *)CONCAT22(param_1,&local_e),(ushort *)CONCAT22(param_1,&local_c),
                    (ushort *)CONCAT22(param_1,&local_a));
    local_a = local_a / 0xa;
    iStack16 = local_c % 0xa;
    if (iStack16 != 0x0) {
      if (iStack16 < 0x6) {
        local_c = local_c - iStack16;
      }
      else {
        local_c = local_c + (0xa - iStack16);
      }
    }
    iStack18 = pass1_1000_49b2(local_e);
    iStack18 = iStack18 / 0x5;
    if (0x14 < iStack18) {
      iStack18 = 0x14;
      iVar2 = pass1_1000_49b2(local_e);
      local_e = ((int)local_e / iVar2) * 0x64;
    }
    iStack16 = pass1_1000_49b2(local_e);
    iStack16 = iStack16 % 0x5;
    if (iStack16 != 0x0) {
      if ((int)local_e < 0x0) {
        iVar2 = iStack16;
        if (0x2 < iStack16) {
          iVar2 = iStack16 + -0x5;
        }
        local_e = local_e + iVar2;
      }
      else {
        if (iStack16 < 0x3) {
          local_e = local_e - iStack16;
        }
        else {
          local_e = local_e + (0x5 - iStack16);
        }
      }
    }
    iStack20 = *(int *)(iStack18 * 0x48 + 0x3c20);
    if (local_c < iStack20) {
      for (iStack26 = iStack18; iStack26 < 0x15; iStack26 = iStack26 + 0x1) {
        piVar1 = (int *)(iStack26 * 0x48 + 0x3c20);
        if (*piVar1 == local_c || *piVar1 < local_c) {
          iStack18 = iStack26;
          break;
        }
      }
    }
    pass1_1008_3e94((ushort *)(param_2 & 0xffff0000 | (ulong)(iVar3 + 0x3a)),(ushort *)CONCAT22(param_1,local_18),
                    (ushort *)CONCAT22(param_1,local_16));
    uVar4 = iStack8 * 0x18 + (int)uStack6;
    *(int *)(uVar4 + 0x6) = local_a;
    *(int *)(uVar4 + 0x8) = iStack18;
    pass1_1008_3e76((ushort *)(uStack6 & 0xffff0000 | (ulong)uVar4),0x0,local_e,
                    *(ushort *)((iStack18 * 0x24 + local_a) * 0x2 + 0x3c20));
    *(undefined2 *)(uVar4 + 0xa) = *(undefined2 *)(local_a * 0x2 + 0x3966);
    iStack8 = iStack8 + 0x1;
  } while( true );
}



ulong __stdcall16far pass1_1018_1e78(ulong param_1,int param_2)

{
  undefined4 uVar1;
  
  if (param_2 == -0x1) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x46);
    param_2 = *(int *)((int)uVar1 + 0xc);
  }
  return CONCAT22(0x1050,param_2 * 0x1e + 0x3c18);
}



void __stdcall16far get_sys_metrics_1018_1ea0(astruct_55 *param_1,ushort param_2)

{
  INT16 IVar1;
  INT16 IVar2;
  astruct_55 *iVar3;
  undefined2 uVar3;
  
  IVar1 = GetSystemMetrics16(param_2);
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_55 *)param_1;
  iVar3->field_0x2e = IVar1 * 0x2 + iVar3->field_0x36;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar3->field_0x30 = IVar1 + iVar3->field_0x38 + IVar2;
  return;
}



void __stdcall16far pass1_1018_1eda(ulong param_1,ulong param_2,ushort param_3)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x46) != 0x0) {
    uVar1 = *(undefined4 *)(iVar2 + 0x46);
    *(undefined2 *)((int)uVar1 + 0xe) = 0x2;
  }
  *(ulong *)(iVar2 + 0x46) = param_2;
  *(undefined2 *)((int)param_2 + 0xe) = 0x1;
  pass1_1010_1f62(param_3,param_1,0xd);
  return;
}



ushort __stdcall16far pass1_1018_1f1a(ulong param_1,int param_2)

{
  int *piVar1;
  int iVar2;
  undefined2 uVar3;
  int iStack6;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(int *)(iVar2 + 0x56) == 0x0) {
    return 0x0;
  }
  iStack6 = 0x0;
  while( true ) {
    piVar1 = (int *)(iVar2 + 0x56);
    if (*piVar1 == iStack6 || *piVar1 < iStack6) {
      return 0x0;
    }
    if (*(int *)(iVar2 + 0x4e + iStack6 * 0x2) == param_2) break;
    iStack6 = iStack6 + 0x1;
  }
  return 0x1;
}



astruct_18 * pass1_1018_1f6a(ushort param_1,astruct_18 *param_2,byte param_3,ushort param_4)

{
  param_2 = (astruct_18 *)((ulong)param_2 & 0xffff0000 | (ulong)((int)param_2 - 0x20));
  pass1_1018_1a04((ushort *)param_2,param_4);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2,0x1000);
  }
  return param_2;
}



ulong __stdcall16far pass1_1018_1f7a(int param_1,ushort param_2)

{
  return CONCAT22(param_2,param_1 + 0x2a);
}



ushort * __stdcall16far pass1_1018_1f8a(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1018_1a04(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1018_1ff4(astruct_634 *param_1,ushort param_2,ushort param_3)

{
  int *piVar1;
  int unaff_DI;
  ushort unaff_SS;
  astruct_79 *paVar2;
  int *piVar3;
  int *piVar4;
  ushort uVar5;
  int local_a;
  int local_8;
  undefined4 uStack6;
  
  paVar2 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)&param_1->field_0xa = 0xb9010b;
  param_1->field_0xe = 0x170035;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x21e8;
  param_1->field_0x2 = 0x1018;
  piVar4 = &local_8;
  piVar3 = &local_a;
  uVar5 = unaff_SS;
  uStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,(uchar *)((ulong)paVar2 >> 0x10),unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)uStack6 & 0xffff0000 | (ulong)((int)uStack6 + 0xe)),
                  (ushort *)CONCAT22(unaff_SS,piVar3),(ushort *)CONCAT22(uVar5,piVar4));
  piVar1 = &param_1->field_0xa;
  *piVar1 = *piVar1 + local_8;
  piVar1 = &param_1->field_0xc;
  *piVar1 = *piVar1 + local_a;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,param_1 + 0x1),(WNDCLASS16 *)0x0,0x7f4);
  return CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1018_2076(ushort *param_1,ushort param_2)

{
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  *param_1 = 0x21e8;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
  pass1_1018_209c((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
  pass1_1010_1d80(param_1,param_2);
  return;
}



void __stdcall16far pass1_1018_209c(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  int iStack4;
  
  iStack4 = 0x0;
  do {
    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1 + 0x12;
    puVar1 = (undefined4 *)*(uint *)(iVar4 + iStack4 * 0x4);
    uVar2 = *(uint *)(iVar4 + iStack4 * 0x4 + 0x2);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    *(undefined4 *)((int)param_1 + iStack4 * 0x4 + 0x12) = 0x0;
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x1fd);
  return;
}



void __stdcall16far pass1_1018_20ee(ulong param_1,int *param_2)

{
  BOOL16 BVar1;
  ushort in_DX;
  uint uVar2;
  
  BVar1 = pass1_1008_aed8((ulong)param_2);
  if (BVar1 == 0x0) {
    return;
  }
  uVar2 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + *param_2 * 0x4 + 0x12) == 0x0) {
    pass1_1018_216e(param_1 & 0xffff | (ulong)uVar2 << 0x10,(ulong)param_2,in_DX);
  }
  pass1_1008_ae26(param_2);
  return;
}



void __stdcall16far pass1_1018_214e(ushort param_1,ushort param_2,ushort *param_3,int param_4)

{
  pass1_1008_3e76(param_3,0x0,*(ushort *)(param_4 * 0x4 + 0x3e90),*(ushort *)(param_4 * 0x4 + 0x3e8e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_216e(ulong param_1,ulong param_2,ushort param_3)

{
  ushort uVar1;
  ushort uVar2;
  ushort uVar3;
  uint uStack8;
  
  uStack8 = pass1_1008_adf2(param_2);
  uVar1 = pass1_1008_ae0c(param_2);
  for (; (int)uStack8 <= (int)uVar1; uStack8 = uStack8 + 0x1) {
    uVar2 = uVar1;
    pass1_1010_8018(_PTR_LOOP_1050_14cc,uStack8,(uchar *)param_3,0x1010);
    uVar3 = (ushort)(param_1 >> 0x10);
    *(ushort *)((int)param_1 + uStack8 * 0x4 + 0x12) = uVar2;
    *(uchar **)((int)param_1 + uStack8 * 0x4 + 0x14) = (uchar *)param_3;
  }
  return;
}



ulong __stdcall16far pass1_1018_21c2(ulong param_1,byte param_2,ushort param_3)

{
  pass1_1018_2076((ushort *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort __cdecl16far pass1_1018_21f8(void)

{
  ushort *puVar1;
  
  pass1_1008_3e54(&USHORT_1048_4210,0x0,0x195,0x1);
  pass1_1008_3e54(&USHORT_1050_65ca,0x0,0xe0,0x1b1);
  pass1_1008_3e54(&USHORT_1050_65d0,0x0,0x17a,0x72);
  pass1_1008_3e54(&USHORT_1050_65d6,0x0,0xde,0x93);
  pass1_1008_3e54(&USHORT_1050_65dc,0x0,0x177,0x1da);
  pass1_1008_3e54(&USHORT_1048_4216,0x0,0x195,0x21c);
  pass1_1008_3e54(&USHORT_1048_421c,0x0,0x1b6,0x22c);
  pass1_1008_3e54(&USHORT_1048_4222,0x0,0x109,0x5);
  puVar1 = pass1_1008_3e54(&USHORT_1048_4228,0x0,0xfd,0x1fd);
  return (ushort)puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1018_229c(astruct_632 *param_1,uchar *param_2,ushort param_3,uchar *param_4,ushort param_5)

{
  int *piVar1;
  astruct_43 *paVar2;
  int iStack4;
  
  struct_op_1018_4cda((int)param_1,(ushort)param_2,param_3);
  param_1->field_0x1c = 0x389a;
  param_1->field_0x1e = 0x1008;
  param_1->field_0x1c = 0x3aa8;
  param_1->field_0x1e = 0x1008;
  param_1->field_0x20 = 0x0;
  param_1->field_0x24 = 0x0;
  param_1->field_0x26 = 0x0;
  *(undefined4 *)&param_1->field_0x2a = 0x0;
  param_1->field_0x3e = 0x0;
  param_1->field_0x40 = 0x0;
  param_1->field_0x42 = 0x0;
  param_1->field_0x44 = 0x0;
  *(undefined4 *)&param_1->field_0x6e = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x2ada;
  param_1->field_0x2 = 0x1018;
  param_1->field_0x1c = (int)s_fem132_wav_1050_2aec + 0x6;
  param_1->field_0x1e = 0x1018;
  PTR_LOOP_1050_4230 = (undefined *)param_1;
  PTR_LOOP_1050_4232 = param_2;
  pass1_1018_4dce((ulong *)CONCAT22(param_2,param_1),0x105,param_4,param_5);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1a8,param_5);
  param_1->field_0x2a = (int)paVar2;
  param_1->field_0x2c = (int)((ulong)paVar2 >> 0x10);
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0x2e),(WNDCLASS16 *)0x0,0x10);
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0x46),(WNDCLASS16 *)0x0,0x28);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x6c,param_5);
  param_1->field_0x2e = (int)paVar2;
  param_1->field_0x30 = (int)((ulong)paVar2 >> 0x10);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x68,param_5);
  param_1->field_0x32 = (int)paVar2;
  param_1->field_0x34 = (int)((ulong)paVar2 >> 0x10);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x66,param_5);
  param_1->field_0x36 = (int)paVar2;
  param_1->field_0x38 = (int)((ulong)paVar2 >> 0x10);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x6a,param_5);
  param_1->field_0x3a = (int)paVar2;
  param_1->field_0x3c = (int)((ulong)paVar2 >> 0x10);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1cd,param_5);
  param_1->field_0x6e = (int)paVar2;
  param_1->field_0x70 = (int)((ulong)paVar2 >> 0x10);
  iStack4 = 0x0;
  do {
    paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,iStack4 + 0x8f,param_5);
    *(undefined2 *)(&param_1->field_0x46 + iStack4 * 0x4) = (int)paVar2;
    *(undefined2 *)(&param_1->field_0x48 + iStack4 * 0x4) = (int)((ulong)paVar2 >> 0x10);
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0xa);
  if (CONCAT22(param_2,param_1) == 0x0) {
    piVar1 = (int *)0x0;
    param_2 = (uchar *)0x0;
  }
  else {
    piVar1 = &param_1->field_0x1c;
  }
  pass1_1008_9262((int)_PTR_LOOP_1050_0388,(ushort)((ulong)_PTR_LOOP_1050_0388 >> 0x10),0x73,CONCAT22(param_2,piVar1),
                  (uint)piVar1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_2440(astruct_11 *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int *piVar4;
  undefined2 uVar6;
  astruct_502 *uVar5;
  undefined2 uVar7;
  undefined2 *puStack6;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  uVar5 = (astruct_502 *)param_1;
  *(undefined2 *)param_1 = 0x2ada;
  uVar5->field_0x2 = 0x1018;
  uVar5->field_0x1c = (int)s_fem132_wav_1050_2aec + 0x6;
  uVar5->field_0x1e = 0x1018;
  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (param_1 == (astruct_11 *)0x0) {
      piVar4 = (int *)0x0;
      uVar6 = 0x0;
    }
    else {
      piVar4 = &uVar5->field_0x1c;
      uVar6 = uVar7;
    }
    param_2 = 0x1008;
    pass1_1008_92b2(_PTR_LOOP_1050_0388,0x73,CONCAT22(uVar6,piVar4));
  }
  puVar1 = uVar5->field_0x2a;
  uVar2 = uVar5->field_0x2c;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_2,puVar1,uVar2,0x1);
  }
  puVar1 = uVar5->field_0x6e;
  uVar2 = uVar5->field_0x70;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_2,puVar1,uVar2,0x1);
  }
  if (param_1 == (astruct_11 *)0x0) {
    piVar4 = (int *)0x0;
    uVar7 = 0x0;
  }
  else {
    piVar4 = &uVar5->field_0x1c;
  }
  puStack6 = (undefined2 *)CONCAT22(uVar7,piVar4);
  *puStack6 = 0x389a;
  piVar4[0x1] = 0x1008;
  clenaup_win_ui_1018_4d22(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_2504(uint param_1,uint param_2)

{
  ushort uVar1;
  
  pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x4000001);
  if ((param_2 | param_1) != 0x0) {
    uVar1 = pass1_1028_d69e(**_PTR_LOOP_1050_5748);
    if (uVar1 == 0x0) {
      return;
    }
  }
  return;
}



void __stdcall16far pass1_1018_2548(ushort param_1,ushort param_2,ushort *param_3)

{
  pass1_1008_3f62(param_3,&USHORT_1048_4228);
  return;
}



ushort __stdcall16far pass1_1018_255e(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x26) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x26);
    return *(ushort *)((int)uVar1 + 0xa);
  }
  return 0x0;
}



uchar * __stdcall16far
pass1_1018_2580(ulong param_1,ushort param_2,ulong param_3,ushort param_4,ushort param_5,uchar param_6)

{
  uchar *puVar1;
  int iVar2;
  undefined2 uVar3;
  uchar local_8 [0x6];
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x20) == 0x0) {
    return (uchar *)0x6ad;
  }
  pass1_1008_3e38((ushort *)CONCAT22(param_5,local_8));
  pass1_1018_161c(param_5,*(ulong *)(iVar2 + 0x20),(ushort *)CONCAT22(param_5,local_8),(int)param_3,
                  (int)(param_3 >> 0x10));
  puVar1 = local_8;
  pass1_1018_17ce(*(ulong *)(iVar2 + 0x20),CONCAT22(puVar1,param_2),CONCAT22(param_4,param_5),param_5,param_6);
  return puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1018_25d2(ulong param_1,ushort param_2,ulong param_3,int param_4,ushort param_5)

{
  ushort uVar1;
  uchar *puVar2;
  undefined2 uVar3;
  ushort *puVar4;
  ushort *puVar5;
  undefined local_8 [0x6];
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x20) == 0x0) {
    return 0x0;
  }
  puVar4 = pass1_1008_3e38((ushort *)CONCAT22(param_5,local_8));
  puVar2 = (uchar *)((ulong)puVar4 >> 0x10);
  pass1_1018_161c(param_5,*(ulong *)((int)param_1 + 0x20),(ushort *)CONCAT22(param_5,local_8),(int)param_3,
                  (int)(param_3 >> 0x10));
  puVar5 = (ushort *)CONCAT22(param_5,local_8);
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_5,puVar2,param_4);
  uVar1 = (ushort)puVar4;
  pass1_1010_71d6((ulong)puVar4,param_2,puVar5,uVar1,(uint)((ulong)puVar4 >> 0x10),param_5);
  return uVar1;
}



void __stdcall16far pass1_1018_262e(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x44) = 0x1;
  *(undefined4 *)((int)param_1 + 0x3e) = 0x0;
  return;
}



void __stdcall16far pass1_1018_2646(ushort param_1,ushort param_2,ushort *param_3)

{
  pass1_1008_3f62(param_3,&USHORT_1048_4222);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1018_265c(void)

{
  ulong uVar1;
  
  uVar1 = pass1_1030_8326();
  return uVar1;
}



ushort __stdcall16far pass1_1018_266a(ulong param_1)

{
  return *(ushort *)((int)param_1 + 0x44);
}



void __stdcall16far pass1_1018_2678(ushort param_1,ushort param_2,ushort *param_3)

{
  pass1_1008_3f62(param_3,&USHORT_1048_4216);
  return;
}



ulong __stdcall16far pass1_1018_268e(ulong param_1)

{
  astruct_287 *iVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar1 = (astruct_287 *)param_1;
  if (iVar1->field_0x42 != 0x0) {
    *(undefined4 *)&iVar1->field_0x40 = 0x0;
    iVar1->field_0x44 = 0x1;
  }
  iVar2 = iVar1->field_0x3e * 0x4;
  return CONCAT22(*(undefined2 *)(&iVar1[0x1].field_0x2 + iVar2),*(undefined2 *)(&iVar1[0x1].field_0x0 + iVar2));
}



void __stdcall16far pass1_1018_26c2(ushort param_1,ushort param_2,ushort *param_3)

{
  pass1_1008_3f62(param_3,&USHORT_1048_421c);
  return;
}



void __stdcall16far pass1_1018_26d8(ushort param_1,ushort param_2,int param_3,ushort *param_4)

{
  pass1_1008_3f62(param_4,(ushort *)CONCAT22(0x1050,(int)&USHORT_1050_65ca + param_3 * 0x6));
  return;
}



void __stdcall16far pass1_1018_26f8(ushort param_1,ushort param_2,ushort *param_3)

{
  pass1_1008_3f62(param_3,&USHORT_1048_4210);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_270e(ulong param_1,int param_2,uint param_3,uchar *param_4,int param_5,ushort param_6)

{
  code **ppcVar1;
  ulong uVar2;
  int iVar3;
  undefined2 uVar4;
  uchar *extraout_DX;
  astruct_655 *iVar5;
  undefined2 uVar5;
  ushort *puVar6;
  
  iVar5 = (astruct_655 *)param_1;
  uVar5 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x0) {
    if ((iVar5->field_0x20 == 0x0) || (uVar2 = iVar5->field_0x20, *(uint *)((int)uVar2 + 0x8) != param_3)) {
      puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,param_3,param_6,param_4,param_5);
      if (iVar5->field_0x20 != 0x0) {
        if (param_1 == 0x0) {
          iVar3 = 0x0;
          uVar4 = 0x0;
        }
        else {
          iVar3 = &iVar5->field_0x1c;
          uVar4 = uVar5;
        }
        pass1_1010_1ea6(iVar5->field_0x20,CONCAT22(uVar4,iVar3),param_6);
      }
      iVar5->field_0x20 = (ulong)puVar6;
      if (param_1 == 0x0) {
        param_3 = 0x0;
        uVar4 = 0x0;
      }
      else {
        param_3 = &iVar5->field_0x1c;
        uVar4 = uVar5;
      }
      uVar2 = iVar5->field_0x20;
      ppcVar1 = (code **)((int)*(undefined4 *)iVar5->field_0x20 + 0x4);
      (**ppcVar1)(0x1010,(int)uVar2,(int)(uVar2 >> 0x10),0x0,param_3,uVar4);
      param_4 = extraout_DX;
    }
    pass1_1018_2862(param_1);
    if (((uint)param_4 | param_3) != 0x0) {
      iVar5->field_0x24 = 0x1;
    }
    pass1_1010_1f62(param_6,param_1,0x7);
  }
  else {
    if ((*(uint *)((int)&iVar5->field_0x20 + 0x2) | *(uint *)&iVar5->field_0x20) != 0x0) {
      if (param_1 == 0x0) {
        iVar3 = 0x0;
        uVar4 = 0x0;
      }
      else {
        iVar3 = &iVar5->field_0x1c;
        uVar4 = uVar5;
      }
      pass1_1010_1ea6(iVar5->field_0x20,CONCAT22(uVar4,iVar3),param_6);
      iVar5->field_0x20 = 0x0;
      return;
    }
  }
  return;
}



void __stdcall16far pass1_1018_280c(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(int *)(iVar2 + 0x24) == 0x0) {
    return;
  }
  *(undefined2 *)(iVar2 + 0x24) = 0x0;
  if (*(long *)(iVar2 + 0x20) == 0x0) {
    *(undefined4 *)(iVar2 + 0x26) = 0x0;
  }
  else {
    uVar1 = *(undefined4 *)(iVar2 + 0x20);
    *(undefined4 *)(iVar2 + 0x26) = *(undefined4 *)((int)uVar1 + 0x4c);
  }
  return;
}



void __stdcall16far pass1_1018_2862(ulong param_1)

{
  long lVar1;
  astruct_654 *iVar2;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar2 = (astruct_654 *)param_1;
  if (iVar2->field_0x20 == 0x0) {
    iVar2->field_0x26 = 0x0;
  }
  else {
    lVar1 = iVar2->field_0x20;
    iVar2->field_0x26 = *(undefined4 *)((int)lVar1 + 0x4c);
  }
  return;
}



void __stdcall16far pass1_1018_289c(ulong param_1,int param_2,uint param_3,ushort param_4)

{
  uint uVar1;
  
  if (param_2 == 0x1) {
    *(undefined4 *)((int)param_1 + 0x4) = 0x0;
    return;
  }
  if (param_2 == 0x2) {
    pass1_1018_2922(param_1 & 0xffff0000 | (ulong)((int)param_1 - 0x1c));
  }
  else {
    if ((((param_2 + -0x3 < 0x1) || (SBORROW2(param_2 + -0x3,0x1))) || (0x1 < param_2 + -0x5)) ||
       (*(long *)((int)param_1 + 0x4) == 0x0)) {
      return;
    }
    uVar1 = (int)param_1 - 0x1c;
    pass1_1018_2862(param_1 & 0xffff0000 | (ulong)uVar1);
    if ((param_3 | uVar1) != 0x0) {
      *(undefined2 *)((int)param_1 + 0x8) = 0x1;
    }
  }
  pass1_1010_1f62(param_4,param_1 & 0xffff0000 | (ulong)((int)param_1 - 0x1c),param_2);
  return;
}



void __stdcall16far pass1_1018_2922(ulong param_1)

{
  int *piVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((*(int *)(iVar2 + 0x40) != 0x0) &&
     (piVar1 = (int *)(iVar2 + 0x3e), *piVar1 = *piVar1 + 0x1, 0x9 < *(int *)(iVar2 + 0x3e))) {
    *(undefined2 *)(iVar2 + 0x3e) = 0x0;
    *(undefined2 *)(iVar2 + 0x42) = 0x1;
  }
  return;
}



void __stdcall16far
win_op_1018_294a(int param_1,UINT16 param_2,UINT16 param_3,ULONG param_4,UINT16 param_5,LPCSTR in_string_6)

{
  if ((*(int *)(param_1 + 0x18) != 0x0) && (param_4._2_2_ == 0x280)) {
    *(undefined2 *)(param_1 + 0x18) = 0x0;
  }
  create_dc_1018_4e04((astruct_8 **)CONCAT22(param_2,param_1),param_3,(int)param_4,param_4._2_2_,in_string_6,param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far mixed_sys_op_1018_2978(ulong param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  undefined *puVar2;
  undefined *puVar3;
  RECT16 *rect;
  int iVar4;
  uchar *in_DX;
  uint uVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  uchar *puVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  undefined uVar12;
  astruct_76 *paStack62;
  RECT16 local_3a;
  int iStack54;
  int iStack52;
  ulong uStack50;
  undefined4 *puStack46;
  undefined local_2a [0x24];
  ushort uStack6;
  
  pass1_1010_8096(_PTR_LOOP_1050_14cc,0x1);
  puVar2 = local_2a;
  uStack6 = param_2;
  struct_op_1008_48fe((astruct_81 *)CONCAT22(param_3,puVar2),0x1,(char *)CONCAT22(in_DX,param_2),(ushort)in_DX);
  uVar9 = 0x1000;
  mem_op_1000_179c(0x1e,in_DX,0x1000);
  uVar5 = (uint)in_DX | (uint)puVar2;
  if (uVar5 == 0x0) {
    puVar3 = (undefined *)0x0;
    uVar5 = 0x0;
  }
  else {
    puVar3 = local_2a;
    uVar9 = 0x1008;
    struct_op_1008_3f92((astruct_76 *)CONCAT22(in_DX,puVar2),(astruct_83 *)CONCAT22(param_3,puVar3));
  }
  puStack46 = (undefined4 *)CONCAT22(uVar5,puVar3);
  ppcVar1 = (code **)((int)*puStack46 + 0x14);
  (**ppcVar1)(uVar9,puVar3,uVar5);
  uStack50 = CONCAT22(extraout_DX,puVar3);
  puVar6 = extraout_DX;
  mem_op_1000_179c(0x14,extraout_DX,0x1000);
  puVar7 = (uchar *)((uint)puVar6 | (uint)puVar3);
  if (puVar7 == (uchar *)0x0) {
    puVar3 = (undefined *)0x0;
    puVar7 = (uchar *)0x0;
  }
  else {
    struct_1008_4c58((ushort *)CONCAT13((char)((uint)puVar6 >> 0x8),CONCAT12((char)puVar6,puVar3)));
  }
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar8 = (int)param_1;
  *(uint *)(iVar8 + 0xe) = (uint)puVar3;
  *(uchar **)(iVar8 + 0x10) = puVar7;
  pass1_1008_4d84(*(astruct_90 **)(iVar8 + 0xe),uStack50,puVar7);
  uVar12 = SUB21(PTR_LOOP_1050_0396,0x0);
  rect = &local_3a;
  GetClientRect16(0x1008,rect);
  uVar11 = 0x1e;
  uVar10 = 0x1000;
  mem_op_1000_179c(0x1e,puVar7,0x1000);
  paStack62 = (astruct_76 *)CONCAT22(puVar7,rect);
  uVar5 = (uint)puVar7 | (uint)rect;
  if (uVar5 == 0x0) {
    *(undefined4 *)(iVar8 + 0xa) = 0x0;
  }
  else {
    iVar4 = (iStack52 - local_3a.y) + 0x1;
    uVar10 = 0x1008;
    pass1_1008_405c(paStack62,*(ulong *)(iVar8 + 0xe),iVar4,(iStack54 - local_3a.x) + 0x1);
    *(int *)(iVar8 + 0xa) = iVar4;
    *(uint *)(iVar8 + 0xc) = uVar5;
  }
  if (puStack46 != (undefined4 *)0x0) {
    ppcVar1 = (code **)*puStack46;
    (**ppcVar1)(uVar10,(int)puStack46,(int)((ulong)puStack46 >> 0x10),0x1,uVar11,uVar12);
  }
  close_file_1008_496c(local_2a,param_3);
  return;
}



void __cdecl16far pass1_1018_2aa3(void)

{
  pass1_1018_21f8();
  return;
}

