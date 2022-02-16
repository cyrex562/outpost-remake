
ushort __cdecl16far pass1_1020_a426(void)

{
  ushort *puVar1;
  
  pass1_1008_3e38((ushort *)&PTR_LOOP_1048_4230);
  puVar1 = pass1_1008_3e38((ushort *)0x10484236);
  return (ushort)puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort * __stdcall16far pass1_1020_a43e(ushort param_1,uchar *param_2,ushort *param_3)

{
  int unaff_DI;
  
  *param_3 = 0xba36;
  *(undefined2 *)((int)param_3 + 0x2) = 0x1020;
  if (_PTR_LOOP_1050_4e74 != 0x0) {
    return param_3;
  }
  mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_1,param_2,unaff_DI);
  if ((0x0 < (int)PTR_LOOP_1050_13ae) && (!SBORROW2((int)PTR_LOOP_1050_13ae,0x1))) {
    if (PTR_LOOP_1050_13ae == (undefined *)&PTR_LOOP_1050_0002 || (int)(PTR_LOOP_1050_13ae + -0x1) < 0x1) {
      PTR_LOOP_1050_4e74 = (undefined *)0x44b4;
      goto LAB_1020_a482;
    }
    if (PTR_LOOP_1050_13ae == (undefined *)&DAT_1050_0004) {
      PTR_LOOP_1050_4e74 = (undefined *)0x4b2c;
      goto LAB_1020_a482;
    }
  }
  PTR_LOOP_1050_4e74 = (undefined *)0x47f0;
LAB_1020_a482:
  _PTR_LOOP_1050_4e74 = CONCAT22(0x1050,PTR_LOOP_1050_4e74);
  return param_3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_a49a(ushort param_1,uchar param_2,uchar *param_3,ulong param_4,int *param_5,ushort param_6)

{
  ulong uVar1;
  int unaff_DI;
  ushort uVar2;
  undefined2 uVar3;
  undefined local_136 [0x128];
  undefined2 uStack14;
  uint uStack12;
  ulong uStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,param_3,unaff_DI);
  uStack12 = (uint)((ulong)puStack6 >> 0x10);
  uVar1 = *(ulong *)((int)puStack6 + 0x20);
  uStack10 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)(uVar1 >> 0x10));
  uStack14 = (undefined2)uVar1;
  if (param_5 != (int *)0x0) {
    uVar2 = (ushort)((ulong)param_5 >> 0x10);
    if (*(int *)((ulong *)param_5 + 0x1) == 0x0) {
      uVar3 = SUB42(&PTR_LOOP_1050_4230,0x0);
    }
    else {
      uVar3 = 0x4236;
    }
    pass1_1008_3f32(param_5,(int *)CONCAT22(0x1048,uVar3));
    struct_op_1028_87f0(param_1,param_2,(astruct_97 *)CONCAT22(param_1,local_136),0x0,0x0,param_6,(ulong *)param_5,uVar2
                        ,*(ulong *)((int)_PTR_LOOP_1050_4e70 + 0x4),uStack10);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_136));
    return;
  }
  pass1_1020_abc0(param_1,param_2,param_4,param_6,uVar1 & 0xffff | (ulong)uStack12 << 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_a54c(ushort param_1,uchar param_2,uchar *param_3,ushort param_4,ushort param_5,int param_6)

{
  undefined4 uVar1;
  int unaff_DI;
  ushort uVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  undefined local_140 [0x124];
  ulong *puStack28;
  int local_18;
  ushort local_16;
  ulong local_14;
  undefined *puStack16;
  undefined2 uStack14;
  undefined2 uStack12;
  undefined4 uStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,param_3,unaff_DI);
  uStack12 = (undefined2)((ulong)puStack6 >> 0x10);
  uVar1 = *(undefined4 *)((int)puStack6 + 0x20);
  uStack10 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  uStack14 = (undefined2)uVar1;
  local_14 = _PTR_LOOP_1048_4230;
  puStack16 = PTR_LOOP_1048_4234;
  puStack28 = &local_14;
  pass1_1008_3e94((ushort *)CONCAT22(param_1,&local_14),(ushort *)CONCAT22(param_1,&local_18),
                  (ushort *)CONCAT22(param_1,&local_16));
  if ((param_6 < 0x0) || (0x5 < param_6)) {
    pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_14),0x0,local_18 - 0x9,local_16);
    uVar5 = (undefined2)uStack10;
    uVar6 = (undefined2)((ulong)uStack10 >> 0x10);
    uVar1 = *(undefined4 *)((int)_PTR_LOOP_1050_4e70 + 0x4);
    uVar3 = (undefined2)uVar1;
    uVar4 = (undefined2)((ulong)uVar1 >> 0x10);
    uVar2 = 0x14;
  }
  else {
    pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_14),0x0,(local_18 - param_6) - 0x3,local_16);
    uVar5 = (undefined2)uStack10;
    uVar6 = (undefined2)((ulong)uStack10 >> 0x10);
    uVar1 = *(undefined4 *)((int)_PTR_LOOP_1050_4e70 + 0x4);
    uVar3 = (undefined2)uVar1;
    uVar4 = (undefined2)((ulong)uVar1 >> 0x10);
    uVar2 = 0x7b;
  }
  struct_op_1028_87f0(param_1,param_2,(astruct_97 *)CONCAT22(param_1,local_140),0x0,0x0,uVar2,&local_14,param_1,
                      CONCAT22(uVar4,uVar3),CONCAT22(uVar6,uVar5));
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_140));
  return;
}



BOOL16 __stdcall16far pass1_1020_a644(ushort param_1,ushort param_2,ulong param_3,uint16_t param_4)

{
  BOOL16 BVar1;
  
  BVar1 = write_to_file_1008_7cac(param_3,param_4);
  if (BVar1 != 0x0) {
    BVar1 = 0x1;
  }
  return BVar1;
}



BOOL16 __stdcall16far read_file_1020_a65e(undefined4 param_1,undefined4 param_2,uint16_t param_3,uint16_t param_4)

{
  BOOL16 BVar1;
  ushort in_DX;
  undefined local_a [0x2];
  undefined local_8 [0x2];
  undefined local_6 [0x2];
  undefined local_4 [0x2];
  uint16_t uVar3;
  uint16_t uVar2;
  
  uVar2 = (uint16_t)param_2;
  uVar3 = (uint16_t)((ulong)param_2 >> 0x10);
  read_file_1008_7cfe(uVar2,uVar3,0xb,0x1008,param_3);
  if (param_4 != 0x0) {
    if (0x1 < (int)PTR_LOOP_1050_0312) {
LAB_1020_a6dc:
      pass1_1020_b97e(param_3,param_4,in_DX,(ushort)param_1,(ushort)((ulong)param_1 >> 0x10),0x0);
      return 0x1;
    }
    BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)local_4,0x0,param_3,0x2,0x1008);
    if (BVar1 != 0x0) {
      BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)local_8,0x0,param_3,0x2,0x1008);
      if (BVar1 != 0x0) {
        BVar1 = read_file_1008_7dee(uVar2,uVar3,(ushort)local_6,0x0,param_3,0x2,0x1008);
        if (BVar1 != 0x0) {
          param_4 = read_file_1008_7dee(uVar2,uVar3,(ushort)local_a,0x0,param_3,0x2,0x1008);
          if (param_4 != 0x0) goto LAB_1020_a6dc;
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_a6ee(ulong param_1,ushort param_2,uint param_3,uint param_4,int param_5,ushort param_6,uchar param_7)

{
  undefined4 uVar1;
  ushort uVar2;
  ushort *puVar3;
  ushort uVar4;
  undefined local_13e [0x120];
  undefined4 uStack30;
  BOOL16 BStack26;
  ulong local_18;
  undefined2 uStack20;
  int iStack18;
  undefined2 uStack16;
  undefined4 uStack14;
  ushort *puStack10;
  undefined4 uStack6;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x2,0x400);
  uStack6 = CONCAT22(param_4,param_3);
  if (((uchar *)(param_4 | param_3) == (uchar *)0x0) || (*(long *)(param_3 + 0x200) == 0x8000002)) {
    puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,(uchar *)(param_4 | param_3),param_5);
    uStack16 = (undefined2)((ulong)puStack10 >> 0x10);
    uVar1 = *(undefined4 *)((int)puStack10 + 0x20);
    uStack14 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
    iStack18 = (int)uVar1;
    puVar3 = pass1_1008_3e38((ushort *)CONCAT22(param_6,&local_18));
    uVar2 = (ushort)((ulong)puVar3 >> 0x10);
    BStack26 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,param_2,0x28);
    if (BStack26 != 0x0) {
      uStack20 = 0x1;
    }
    uVar4 = (ushort)(param_1 >> 0x10);
    pass1_1020_b2da(param_6,(ushort)param_1,uVar4,(uint)(BStack26 != 0x0),(ushort *)CONCAT22(param_6,&local_18),
                    CONCAT22(uStack16,iStack18));
    struct_op_1028_87f0(param_6,param_7,(astruct_97 *)CONCAT22(param_6,local_13e),0x0,0x0,param_2,&local_18,param_6,
                        *(ulong *)((int)_PTR_LOOP_1050_4e70 + 0x4),*(ulong *)(iStack18 + 0x4));
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_6,local_13e));
    if (BStack26 != 0x0) {
      pass1_1020_ad90(param_6,uVar2,(ushort)param_1,uVar4,(ushort *)CONCAT22(param_6,&local_18),
                      *(ulong *)(iStack18 + 0x4));
    }
    *(undefined4 *)((int)uStack30 + 0x1c) = 0x8000001;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_a80e(ushort param_1,ushort param_2,int param_3,uint param_4,uint param_5,ushort param_6,uchar param_7,
               int param_8)

{
  ushort uVar1;
  ulong uVar2;
  ushort uVar3;
  ushort *puVar4;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x2,0x400);
  if (((uchar *)(param_5 | param_4) == (uchar *)0x0) || (*(long *)(param_4 + 0x200) == 0x8000002)) {
    puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,(uchar *)(param_5 | param_4),param_8);
    uVar3 = (ushort)((ulong)puVar4 >> 0x10);
    uVar2 = *(ulong *)((int)puVar4 + 0x20);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)(uVar2 >> 0x10));
    uVar1 = (ushort)uVar2;
    if (param_3 == 0xa) {
      pass1_1020_b872(param_6,param_7,CONCAT22(param_2,param_1),uVar2 & 0xffff | (ulong)uVar3 << 0x10);
      return;
    }
    pass1_1020_b0aa(param_1,param_2,param_3,uVar3);
    if (uVar1 != 0x0) {
      pass1_1020_abc0(param_6,param_7,CONCAT22(param_2,param_1),uVar1,uVar2 & 0xffff | (ulong)uVar3 << 0x10);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_a89e(ushort param_1,ulong param_2,ulong *param_3,ushort param_4)

{
  int *piVar1;
  undefined *puVar2;
  ushort uVar3;
  ushort uVar4;
  ulong uVar5;
  uchar *in_DX;
  uint uVar6;
  ulong *puVar7;
  uint extraout_DX;
  int unaff_DI;
  uchar in_AF;
  ushort uVar8;
  ushort uVar9;
  undefined uVar10;
  undefined uVar11;
  undefined2 local_5ee;
  undefined2 uStack1516;
  ulong *puStack1218;
  int iStack1214;
  undefined4 uStack1212;
  undefined local_4b8 [0x8];
  ulong uStack1200;
  ushort *puStack1196;
  undefined local_4a8 [0x124];
  undefined local_384 [0x124];
  undefined local_260 [0x124];
  undefined local_13c [0x124];
  ushort local_18;
  ushort local_16;
  ulong local_14;
  undefined2 uStack16;
  ulong uStack14;
  ulong uStack10;
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,in_DX,unaff_DI);
  uVar6 = (uint)((ulong)puStack6 >> 0x10);
  uVar5 = *(ulong *)((int)puStack6 + 0x20);
  uStack10 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar5,(uint)(uVar5 >> 0x10));
  uStack14 = uVar5 & 0xffff | (ulong)uVar6 << 0x10;
  local_14 = *param_3;
  uStack16 = *(undefined2 *)(param_3 + 0x1);
  puStack1218 = &local_14;
  puVar7 = &local_14;
  pass1_1008_3e94((ushort *)CONCAT22(param_1,puVar7),(ushort *)CONCAT22(param_1,&local_18),
                  (ushort *)CONCAT22(param_1,&local_16));
  uVar10 = (undefined)param_1;
  uVar11 = (undefined)(param_1 >> 0x8);
  pass1_1008_3e76((ushort *)CONCAT13(uVar11,CONCAT12(uVar10,&local_14)),0x0,local_18,local_16 + 0x2);
  struct_op_1028_8888(param_1,in_AF,(astruct_100 *)CONCAT22(param_1,local_13c),0x0,0x7a,&local_14,param_1,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_13c));
  pass1_1008_3e76((ushort *)CONCAT13(uVar11,CONCAT12(uVar10,&local_14)),0x0,local_18 - 0x2,local_16);
  struct_op_1028_8888(param_1,in_AF,(astruct_100 *)CONCAT22(param_1,local_260),0x0,0x47,&local_14,param_1,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_260));
  pass1_1008_3e76((ushort *)CONCAT13(uVar11,CONCAT12(uVar10,&local_14)),0x1,local_18 - 0x2,local_16);
  struct_op_1028_8888(param_1,in_AF,(astruct_100 *)CONCAT22(param_1,local_384),0x0,0x6a,&local_14,param_1,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_384));
  uVar8 = (ushort)param_2;
  uVar9 = (ushort)(param_2 >> 0x10);
  pass1_1020_ad90(param_1,(ushort)puVar7,uVar8,uVar9,(ushort *)CONCAT22(param_1,&local_14),uStack10);
  pass1_1008_3e76((ushort *)CONCAT13(uVar11,CONCAT12(uVar10,&local_14)),0x1,local_18 - 0x2,local_16 + 0x1);
  struct_op_1028_8888(param_1,in_AF,(astruct_100 *)CONCAT22(param_1,local_4a8),0x0,0x7f,&local_14,param_1,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_4a8));
  pass1_1020_ad90(param_1,(ushort)puVar7,uVar8,uVar9,(ushort *)CONCAT22(param_1,&local_14),uStack10);
  puStack1196 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_1,(uchar *)puVar7,(int)&uStack14);
  uStack1200 = *(ulong *)((int)puStack1196 + 0x12);
  pass1_1008_5784((ulong *)CONCAT22(param_1,local_4b8),uStack1200);
  iStack1214 = 0x0;
  do {
    do {
      puVar2 = local_4b8;
      pass1_1008_5b12(puVar2,param_1);
      uStack1212 = CONCAT22(extraout_DX,puVar2);
      if ((extraout_DX | (uint)puVar2) == 0x0) {
        pass1_1010_9674((ulong)puStack1196);
        return;
      }
    } while ((*(int *)(puVar2 + 0x4) != 0x3e) && (*(int *)(puVar2 + 0x4) != 0x41));
    while (0x0 < *(int *)((int)uStack1212 + 0x6)) {
      if (iStack1214 == 0x0) {
        uVar4 = local_16 - 0x2;
LAB_1020_ab4a:
        uVar3 = local_18 - 0x2;
LAB_1020_ab51:
        iStack1214 = iStack1214 + 0x1;
        pass1_1008_3e76((ushort *)CONCAT13(uVar11,CONCAT12(uVar10,&local_14)),0x0,uVar3,uVar4);
      }
      else {
        if (iStack1214 == 0x1) {
          uVar4 = local_16 + 0x2;
          goto LAB_1020_ab4a;
        }
        if (iStack1214 == 0x2) {
          uVar4 = local_16 + 0x2;
LAB_1020_ab6e:
          uVar3 = local_18 + 0x2;
          goto LAB_1020_ab51;
        }
        if (iStack1214 == 0x3) {
          uVar4 = local_16 - 0x2;
          goto LAB_1020_ab6e;
        }
        iStack1214 = iStack1214 + 0x1;
        pass1_1020_b2da(param_1,uVar8,uVar9,0x0,(ushort *)CONCAT22(param_1,&local_14),uStack14);
      }
      struct_op_1028_8888(param_1,in_AF,(astruct_100 *)CONCAT22(param_1,&local_5ee),0x0,
                          *(ushort *)((int)uStack1212 + 0x4),&local_14,param_1,0x8000002,0x4000002,uStack10);
      fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,&local_5ee));
      piVar1 = (int *)((int)uStack1212 + 0x6);
      *piVar1 = *piVar1 + -0x1;
      local_5ee = 0x389a;
      uStack1516 = 0x1008;
    }
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_abc0(ushort param_1,uchar param_2,ulong param_3,ushort param_4,ulong param_5)

{
  ushort uVar1;
  undefined2 uVar2;
  ushort *puVar3;
  ushort uVar4;
  undefined local_12e [0x124];
  BOOL16 BStack10;
  ulong local_8;
  undefined2 uStack4;
  
  puVar3 = pass1_1008_3e38((ushort *)CONCAT22(param_1,&local_8));
  uVar1 = (ushort)((ulong)puVar3 >> 0x10);
  BStack10 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,param_4,0x28);
  if (BStack10 != 0x0) {
    uStack4 = 0x1;
  }
  uVar4 = (ushort)(param_3 >> 0x10);
  pass1_1020_b2da(param_1,(ushort)param_3,uVar4,(uint)(BStack10 != 0x0),(ushort *)CONCAT22(param_1,&local_8),param_5);
  uVar2 = (undefined2)(param_5 >> 0x10);
  struct_op_1028_87f0(param_1,param_2,(astruct_97 *)CONCAT22(param_1,local_12e),0x0,0x0,param_4,&local_8,param_1,
                      *(ulong *)((int)_PTR_LOOP_1050_4e70 + 0x4),*(ulong *)((int)param_5 + 0x4));
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_12e));
  if (BStack10 != 0x0) {
    pass1_1020_ad90(param_1,uVar1,(ushort)param_3,uVar4,(ushort *)CONCAT22(param_1,&local_8),
                    *(ulong *)((int)param_5 + 0x4));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_ac6e(ushort param_1,uchar param_2,ulong param_3,int param_4,int param_5,int param_6)

{
  ushort uVar1;
  ulong *puVar2;
  ulong uVar3;
  ushort uVar4;
  int unaff_DI;
  ushort *puVar5;
  undefined2 uVar6;
  undefined local_146 [0x12c];
  int iStack26;
  ushort uStack24;
  ulong uStack22;
  ushort *puStack18;
  ulong local_e;
  ushort local_8;
  undefined4 local_6;
  
  if (param_4 == 0x0) {
    uVar6 = SUB42(&PTR_LOOP_1050_4230,0x0);
  }
  else {
    uVar6 = 0x4236;
  }
  pass1_1008_3eb4((ushort *)CONCAT22(0x1048,uVar6),(ushort *)CONCAT22(param_1,&local_8),
                  (ushort *)CONCAT22(param_1,&local_6),(ushort *)CONCAT22(param_1,(int)&local_6 + 0x2));
  if (param_6 == 0x0) {
    local_6 = local_6 & 0xffff | (ulong)(uint)(local_6._2_2_ + param_5) << 0x10;
  }
  else {
    if (param_6 == 0x1) {
      local_6 = local_6 & 0xffff0000 | (ulong)(uint)((int)local_6 + param_5);
    }
    else {
      if (param_6 == 0x2) {
        local_6 = local_6 & 0xffff | (ulong)(uint)(local_6._2_2_ - param_5) << 0x10;
      }
    }
  }
  puVar5 = pass1_1008_3e54((ushort *)CONCAT22(param_1,&local_e),local_8,(ushort)local_6,(ushort)(local_6 >> 0x10));
  puStack18 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,(uchar *)((ulong)puVar5 >> 0x10),unaff_DI);
  uVar4 = (ushort)((ulong)puStack18 >> 0x10);
  uVar3 = *(ulong *)((int)puStack18 + 0x20);
  uStack22 = uVar3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar3,(uint)(uVar3 >> 0x10));
  iStack26 = (int)uVar3;
  uStack24 = uVar4;
  uVar1 = pass1_1020_b1ae((int)&local_e,uVar4,param_1,(ushort)param_3,(ushort)(param_3 >> 0x10),
                          (ushort *)CONCAT22(param_1,&local_e),*(ulong *)(iStack26 + 0x4));
  if (uVar1 != 0x0) {
    puVar2 = &local_e;
    pass1_1020_b240(param_1,uVar4,param_3,CONCAT22(param_1,puVar2),CONCAT22(uStack24,iStack26));
    if (puVar2 != (ulong *)0x0) {
      struct_op_1028_87f0(param_1,param_2,(astruct_97 *)CONCAT22(param_1,local_146),0x0,0x0,
                          (-(uint)(param_4 == 0x0) & 0xfffb) + 0x7f,&local_e,param_1,
                          *(ulong *)((int)_PTR_LOOP_1050_4e70 + 0x4),uStack22);
      fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_146));
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_ad90(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort *param_5,ulong param_6)

{
  code **ppcVar1;
  ushort *puVar2;
  undefined *puVar3;
  int iVar4;
  undefined4 uVar5;
  uint uVar6;
  undefined2 extraout_DX;
  uchar in_AF;
  undefined4 *puVar7;
  ushort uVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined2 local_17e;
  undefined2 uStack380;
  int iStack90;
  undefined4 *puStack78;
  ushort uStack70;
  int iStack68;
  undefined4 uStack66;
  undefined4 *puStack62;
  undefined local_3a [0xc];
  undefined4 local_2e;
  undefined2 uStack42;
  int iStack40;
  ushort uStack38;
  int local_24;
  int local_22;
  undefined4 uStack32;
  undefined4 uStack28;
  undefined4 uStack24;
  ushort *puStack20;
  uint uStack18;
  int iStack16;
  int iStack14;
  undefined4 uStack12;
  ushort local_8;
  int local_6;
  int local_4;
  
  puVar2 = &local_8;
  pass1_1008_3eb4(param_5,(ushort *)CONCAT22(param_1,puVar2),(ushort *)CONCAT22(param_1,&local_6),
                  (ushort *)CONCAT22(param_1,&local_4));
  pass1_1030_627e(param_1,(uint)puVar2,param_2,(ulong)_PTR_LOOP_1050_5740,param_5,param_6);
  puStack20 = puVar2;
  uStack18 = param_2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)puVar2,param_2);
  uStack24 = CONCAT22(param_2,puVar2);
  uStack28 = *(undefined4 *)(puVar2 + 0x17);
  uVar5 = *(undefined4 *)((int)uStack28 + 0x4);
  uStack32 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_6,(uint)(param_6 >> 0x10));
  iStack40 = (int)uVar5;
  uStack38 = param_2;
  puVar7 = (undefined4 *)pass1_1030_5b5c(iStack40,param_2);
  uVar6 = (uint)((ulong)puVar7 >> 0x10);
  local_2e = *puVar7;
  uStack42 = *(undefined2 *)((int)puVar7 + 0x4);
  puStack78 = &local_2e;
  pass1_1008_3e94((ushort *)CONCAT22(param_1,&local_2e),(ushort *)CONCAT22(param_1,&local_24),
                  (ushort *)CONCAT22(param_1,&local_22));
  iStack14 = local_4 + 0x1;
  uStack12 = CONCAT22(local_4 + -0x1,local_6 - 0x1U);
  iStack16 = local_6 + 0x1;
  if (local_4 + -0x1 < 0x0) {
    uStack12 = (ulong)(local_6 - 0x1U);
  }
  if (local_22 <= iStack14) {
    iStack14 = local_22 + -0x1;
  }
  if ((int)(ushort)uStack12 < 0x0) {
    uStack12 = uStack12 & 0xffff0000;
  }
  if (local_24 <= iStack16) {
    iStack16 = local_24 + -0x1;
  }
  pass1_1008_6c90((ushort *)CONCAT22(param_1,local_3a));
  pass1_1008_6cec((ushort *)CONCAT22(param_1,local_3a),local_8,CONCAT22(iStack14,iStack16),local_8,uStack12);
  puVar3 = local_3a;
  pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(param_1,puVar3),param_6,param_1);
  puStack62 = (undefined4 *)CONCAT22(uVar6,puVar3);
  if ((uVar6 | (uint)puVar3) != 0x0) {
    uStack66 = 0x0;
    iStack68 = 0x0;
    for (uStack70 = (ushort)uStack12; (int)uStack70 <= iStack16; uStack70 = uStack70 + 0x1) {
      for (puStack78 = (undefined4 *)uStack12._2_2_; (int)puStack78 <= iStack14;
          puStack78 = (undefined4 *)((int)puStack78 + 0x1)) {
        ppcVar1 = (code **)((int)*puStack62 + 0x4);
        iVar4 = iStack68;
        iStack68 = iStack68 + 0x1;
        (**ppcVar1)(0x1030,(int)puStack62,(int)((ulong)puStack62 >> 0x10));
        uStack66 = CONCAT22(extraout_DX,iVar4);
        uStack66._3_1_ = (char)((uint)extraout_DX >> 0x8);
        if (uStack66._3_1_ == '\0') {
          iStack90 = iVar4;
          if (iVar4 == 0x7) {
            pass1_1008_3e76(param_5,local_8,uStack70,(ushort)puStack78);
            uVar9 = (undefined2)uStack32;
            uVar10 = (undefined2)((ulong)uStack32 >> 0x10);
            uVar8 = 0x6;
          }
          else {
            if (iVar4 == 0x8) {
              pass1_1008_3e76(param_5,local_8,uStack70,(ushort)puStack78);
              uVar9 = (undefined2)uStack32;
              uVar10 = (undefined2)((ulong)uStack32 >> 0x10);
              uVar8 = 0x7;
            }
            else {
              if (iVar4 != 0x9) goto LAB_1020_af1c;
              pass1_1008_3e76(param_5,local_8,uStack70,(ushort)puStack78);
              uVar9 = (undefined2)uStack32;
              uVar10 = (undefined2)((ulong)uStack32 >> 0x10);
              uVar8 = 0x8;
            }
          }
          struct_op_1028_87f0(param_1,in_AF,(astruct_97 *)CONCAT22(param_1,&local_17e),0x0,0x0,uVar8,(ulong *)param_5,
                              (ushort)((ulong)param_5 >> 0x10),CONCAT22(uVar10,uVar9),param_6);
          fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,&local_17e));
          local_17e = 0x389a;
          uStack380 = 0x1008;
        }
LAB_1020_af1c:
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_afc4(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort *param_5,long param_6)

{
  undefined4 *puVar1;
  uint uVar2;
  uint uVar3;
  ulong uVar4;
  byte bStack27;
  undefined4 local_a;
  undefined4 uStack6;
  
  puVar1 = &local_a;
  pass1_1030_64ce(param_1,puVar1,param_2,_PTR_LOOP_1050_5740,param_5,param_6,(ulong *)CONCAT22(param_1,puVar1));
  uStack6 = *puVar1;
  uVar3 = *(uint *)((int)puVar1 + 0x2);
  bStack27 = (byte)((ulong)uStack6 >> 0x18);
  uVar2 = (uint)bStack27;
  if (bStack27 == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack6,uVar3);
  uVar4 = struct_op_1030_73a8(CONCAT22(uVar3,uVar2));
  uVar3 = (uint)(uVar4 >> 0x10);
  if ((uVar3 | (uint)uVar4) != 0x0) {
    switch(*(undefined2 *)((uint)uVar4 + 0xc)) {
    case 0x1:
      break;
    case 0x2:
      break;
    case 0x3:
      break;
    case 0x4:
      break;
    case 0x5:
      break;
    case 0x6:
      break;
    case 0x7:
      return;
    case 0x8:
      return;
    case 0x9:
      return;
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_b0aa(ushort param_1,ushort param_2,int param_3,ushort param_4)

{
  undefined4 *puVar1;
  code **ppcVar2;
  int iVar3;
  undefined4 *puVar4;
  uint extraout_DX;
  uint uVar5;
  uint uVar6;
  undefined2 uVar7;
  ulong uVar8;
  ulong uStack20;
  
  uVar7 = (undefined2)((ulong)_PTR_LOOP_1050_4e74 >> 0x10);
  if (*(int *)((int)_PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) == 0x0) {
    return;
  }
  if (*(int *)((int)_PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) != -0x1) {
    if (PTR_LOOP_1050_4e78 == (undefined *)0x0) {
      iVar3 = param_3;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x2,0x400);
      puVar1 = (undefined4 *)*(ulong *)(iVar3 + 0xc);
      ppcVar2 = (code **)((int)*puVar1 + 0x10);
      puVar4 = puVar1;
      (**ppcVar2)();
      uVar6 = extraout_DX;
      for (uStack20 = 0x0; uStack20 < ((ulong)puVar4 & 0xffff | (ulong)extraout_DX << 0x10); uStack20 = uStack20 + 0x1)
      {
        uVar8 = pass1_1030_1d7c((int)((ulong)puVar4 & 0xffff),uVar6,(ulong)puVar1);
        uVar5 = (uint)(uVar8 >> 0x10);
        uVar6 = uVar5 | (uint)uVar8;
        if ((uVar6 != 0x0) && ((iVar3 = *(int *)((uint)uVar8 + 0xc), iVar3 == 0x2a || (iVar3 == 0x2b)))) {
          PTR_LOOP_1050_4e78 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
          break;
        }
      }
      if (PTR_LOOP_1050_4e78 == (undefined *)0x0) {
        PTR_LOOP_1050_4e78 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
        return;
      }
    }
    iVar3 = *(int *)((int)_PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) + -0x1;
    pass1_1008_612e(0x0,iVar3,iVar3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far
pass1_1020_b1ae(int param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,ushort *param_6,ulong param_7)

{
  undefined4 *puVar1;
  int local_14;
  int local_12;
  int local_10;
  int local_e;
  undefined4 local_c;
  undefined2 uStack8;
  int iStack6;
  ushort uStack4;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_7,(uint)(param_7 >> 0x10));
  iStack6 = param_1;
  uStack4 = param_2;
  puVar1 = (undefined4 *)pass1_1030_5b5c(param_1,param_2);
  local_c = *puVar1;
  uStack8 = *(undefined2 *)((int)puVar1 + 0x4);
  pass1_1008_3e94(param_6,(ushort *)CONCAT22(param_3,&local_10),(ushort *)CONCAT22(param_3,&local_e));
  pass1_1008_3e94((ushort *)CONCAT22(param_3,&local_c),(ushort *)CONCAT22(param_3,&local_14),
                  (ushort *)CONCAT22(param_3,&local_12));
  if ((((0xb < local_e) && (0xb < local_10)) && (local_e < local_12 + -0xb)) && (local_10 < local_14 + -0xb)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_b240(ushort param_1,ushort param_2,ulong param_3,ulong param_4,ulong param_5)

{
  undefined4 *puVar1;
  uint uVar2;
  ushort uVar3;
  uint uVar4;
  uint uVar5;
  undefined2 uVar6;
  ulong uVar7;
  byte bStack31;
  undefined4 local_a;
  undefined4 uStack6;
  
  puVar1 = &local_a;
  uVar6 = (undefined2)(param_5 >> 0x10);
  pass1_1030_64ce(param_1,puVar1,param_2,_PTR_LOOP_1050_5740,(ushort *)param_4,*(long *)((int)param_5 + 0x4),
                  (ulong *)CONCAT22(param_1,puVar1));
  uStack6 = *puVar1;
  uVar5 = *(uint *)((int)puVar1 + 0x2);
  bStack31 = (byte)((ulong)uStack6 >> 0x18);
  uVar2 = (uint)bStack31;
  if (bStack31 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack6,uVar5);
    uVar7 = struct_op_1030_73a8(CONCAT22(uVar5,uVar2));
    uVar4 = (uint)(uVar7 >> 0x10);
    uVar2 = (uint)uVar7;
    uVar5 = uVar4 | uVar2;
    if ((uVar5 != 0x0) && (uVar2 = *(uint *)(uVar2 + 0xc), 0x9 < (int)uVar2)) {
      return;
    }
  }
  uVar3 = pass1_1020_b1ae(uVar2,uVar5,param_1,(ushort)param_3,(ushort)(param_3 >> 0x10),(ushort *)param_4,
                          *(ulong *)((int)param_5 + 0x4));
  if (uVar3 == 0x0) {
    return;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far
pass1_1020_b2da(ushort param_1,ushort param_2,ushort param_3,int param_4,ushort *param_5,ulong param_6)

{
  int iVar1;
  undefined2 uVar2;
  ushort uVar3;
  undefined *puVar4;
  ushort uVar5;
  uchar in_AF;
  ushort *puVar6;
  undefined **ppuVar7;
  int iStack28;
  undefined local_1a [0x6];
  undefined2 uStack20;
  undefined2 uStack18;
  int *piStack16;
  int *piStack12;
  ushort local_8;
  undefined4 local_6;
  
  if (param_4 == 0x0) {
    uVar2 = 0x4e6a;
  }
  else {
    uVar2 = 0x4e6e;
  }
  piStack12 = (int *)CONCAT22(0x1050,uVar2);
  if (param_4 == 0x0) {
    uStack20 = 0x4e68;
  }
  else {
    uStack20 = 0x4e6c;
  }
  uStack18 = SUB42(&USHORT_1050_1050,0x0);
  piStack16 = (int *)CONCAT22(0x1050,uStack20);
  do {
    if (param_4 == 0x0) {
      ppuVar7 = &PTR_LOOP_1048_4230;
    }
    else {
      ppuVar7 = (undefined **)0x10484236;
    }
    pass1_1008_3eb4((ushort *)ppuVar7,(ushort *)CONCAT22(param_1,&local_8),(ushort *)CONCAT22(param_1,&local_6),
                    (ushort *)CONCAT22(param_1,(int)&local_6 + 0x2));
    iVar1 = *piStack12;
    if (iVar1 == 0x0) {
      local_6 = CONCAT22(local_6._2_2_ + *piStack16,(int)local_6 + -0x1);
    }
    else {
      if (iVar1 == 0x1) {
        local_6 = CONCAT22(local_6._2_2_ + -0x1,(int)local_6 + *piStack16);
      }
      else {
        if (iVar1 == 0x2) {
          local_6 = CONCAT22(local_6._2_2_ - *piStack16,(int)local_6 + -0x1);
        }
      }
    }
    puVar6 = pass1_1008_3e54((ushort *)CONCAT22(param_1,local_1a),local_8,(ushort)local_6,(ushort)(local_6 >> 0x10));
    uVar5 = (ushort)((ulong)puVar6 >> 0x10);
    uVar2 = (undefined2)(param_6 >> 0x10);
    uVar3 = pass1_1020_b1ae((int)local_1a,uVar5,param_1,param_2,param_3,(ushort *)CONCAT22(param_1,local_1a),
                            *(ulong *)((int)param_6 + 0x4));
    if (uVar3 != 0x0) {
      puVar4 = local_1a;
      pass1_1020_b240(param_1,uVar5,CONCAT22(param_3,param_2),CONCAT22(param_1,puVar4),param_6);
      if (puVar4 != (undefined *)0x0) {
LAB_1020_b46e:
        pass1_1008_3e76(param_5,local_8,(ushort)local_6,(ushort)(local_6 >> 0x10));
        return;
      }
    }
    iVar1 = *piStack12;
    if (iVar1 == 0x0) {
LAB_1020_b45e:
      local_6 = local_6 & 0xffff0000 | (ulong)((int)local_6 + 0x2);
    }
    else {
      if (iVar1 == 0x1) {
        local_6 = local_6 & 0xffff | (ulong)(local_6._2_2_ + 0x2) << 0x10;
      }
      else {
        if (iVar1 == 0x2) goto LAB_1020_b45e;
      }
    }
    pass1_1008_3e76((ushort *)CONCAT22(param_1,local_1a),local_8,(ushort)local_6,(ushort)(local_6 >> 0x10));
    uVar3 = pass1_1020_b1ae((int)local_1a,uVar5,param_1,param_2,param_3,(ushort *)CONCAT22(param_1,local_1a),
                            *(ulong *)((int)param_6 + 0x4));
    if (uVar3 != 0x0) {
      puVar4 = local_1a;
      pass1_1020_b240(param_1,uVar5,CONCAT22(param_3,param_2),CONCAT22(param_1,puVar4),param_6);
      if (puVar4 != (undefined *)0x0) goto LAB_1020_b46e;
    }
    iStack28 = *piStack12 + 0x1;
    if (0x2 < iStack28) {
      iStack28 = 0x0;
      *piStack16 = *piStack16 + 0x1;
    }
    *piStack12 = iStack28;
    pass1_1020_ac6e(param_1,in_AF,CONCAT22(param_3,param_2),param_4,*piStack16,iStack28);
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_b482(ushort param_1,ulong param_2,undefined4 *param_3,ulong param_4)

{
  undefined *puVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  uint uVar4;
  uint uVar5;
  undefined4 *puVar6;
  ulong uVar7;
  ushort uVar8;
  ushort uVar9;
  undefined4 *puVar10;
  int iStack46;
  undefined4 local_2a;
  ushort local_26;
  undefined4 local_24;
  undefined2 uStack32;
  long lStack30;
  undefined4 uStack26;
  undefined local_16 [0x12];
  undefined local_4 [0x2];
  
  uVar7 = pass1_1030_bcae((ushort)local_4,param_1);
  uVar4 = (uint)(uVar7 >> 0x10);
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_1,local_16),0x1,0x0,0x400);
  while( true ) {
    puVar1 = local_16;
    pass1_1028_e4ec(CONCAT22(param_1,puVar1));
    uStack26 = CONCAT22(uVar4,puVar1);
    uVar5 = uVar4 | (uint)puVar1;
    if (uVar5 == 0x0) {
      pass1_1020_b240(param_1,0x0,param_2,(ulong)param_3,param_4);
      if (puVar1 != (undefined *)0x0) {
        lStack30 = *(long *)((int)param_4 + 0x4);
        local_24 = *param_3;
        uStack32 = *(undefined2 *)((int)param_3 + 0x4);
        puVar6 = &local_2a;
        pass1_1008_3eb4((ushort *)CONCAT22(param_1,&local_24),(ushort *)CONCAT22(param_1,puVar6),
                        (ushort *)CONCAT22(param_1,(int)&local_2a + 0x2),(ushort *)CONCAT22(param_1,&local_26));
        pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,local_2a._2_2_ - 0x1,local_26 - 0x1);
        puVar2 = &local_24;
        uVar8 = (ushort)param_2;
        uVar9 = (ushort)(param_2 >> 0x10);
        pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),lStack30);
        if (puVar2 != (undefined4 *)0x0) {
          pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,(ushort)((ulong)local_2a >> 0x10),
                          local_26 - 0x1);
          puVar2 = &local_24;
          pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),lStack30);
          if (puVar2 != (undefined4 *)0x0) {
            pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,local_2a._2_2_ + 0x1,local_26 - 0x1);
            puVar2 = &local_24;
            pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),lStack30);
            if (puVar2 != (undefined4 *)0x0) {
              pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,local_2a._2_2_ - 0x1,local_26);
              puVar2 = &local_24;
              pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),lStack30);
              if (puVar2 != (undefined4 *)0x0) {
                pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,local_2a._2_2_ + 0x1,local_26);
                puVar2 = &local_24;
                pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),lStack30);
                if (puVar2 != (undefined4 *)0x0) {
                  pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,local_2a._2_2_ + 0x1,
                                  local_26 + 0x1);
                  puVar2 = &local_24;
                  pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),lStack30);
                  if (puVar2 != (undefined4 *)0x0) {
                    pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,
                                    (ushort)((ulong)local_2a >> 0x10),local_26 + 0x1);
                    puVar2 = &local_24;
                    pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),lStack30);
                    if (puVar2 != (undefined4 *)0x0) {
                      pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,local_2a._2_2_ - 0x1,
                                      local_26 + 0x1);
                      puVar2 = &local_24;
                      pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),lStack30);
                      if (puVar2 != (undefined4 *)0x0) {
                        pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,local_2a._2_2_ - 0x2,
                                        local_26 - 0x2);
                        puVar2 = &local_24;
                        pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),lStack30);
                        if (puVar2 != (undefined4 *)0x0) {
                          pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,local_2a._2_2_ + 0x2,
                                          local_26 - 0x2);
                          puVar2 = &local_24;
                          pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),lStack30
                                         );
                          if (puVar2 != (undefined4 *)0x0) {
                            pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,local_2a._2_2_ - 0x2,
                                            local_26 + 0x2);
                            puVar2 = &local_24;
                            pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),
                                            lStack30);
                            if (puVar2 != (undefined4 *)0x0) {
                              pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,
                                              local_2a._2_2_ + 0x2,local_26 + 0x2);
                              puVar2 = &local_24;
                              pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),
                                              lStack30);
                              if (puVar2 != (undefined4 *)0x0) {
                                pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,
                                                local_2a._2_2_ - 0x1,local_26 + 0x2);
                                puVar2 = &local_24;
                                pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),
                                                lStack30);
                                if (puVar2 != (undefined4 *)0x0) {
                                  pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),(ushort)local_2a,
                                                  local_2a._2_2_ - 0x1,local_26 + 0x3);
                                  puVar2 = &local_24;
                                  pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,(ushort *)CONCAT22(param_1,puVar2),
                                                  lStack30);
                                  if (puVar2 != (undefined4 *)0x0) {
                                    iStack46 = 0x3;
                                    while( true ) {
                                      if (0x9 < iStack46) {
                                        return;
                                      }
                                      pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_24),0x0,
                                                      local_2a._2_2_ - iStack46,local_26);
                                      puVar2 = &local_24;
                                      pass1_1020_afc4(param_1,(ushort)puVar6,uVar8,uVar9,
                                                      (ushort *)CONCAT22(param_1,puVar2),lStack30);
                                      if (puVar2 == (undefined4 *)0x0) break;
                                      iStack46 = iStack46 + 0x1;
                                    }
                                    return;
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      return;
    }
    uVar3 = *(undefined4 *)(puVar1 + 0x10);
    puVar10 = param_3;
    uVar7 = param_4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar3,(uint)((ulong)uVar3 >> 0x10));
    puVar1 = local_4;
    pass1_1030_bcbc(param_1,(ushort)puVar1,CONCAT22((int)uVar3,param_1),CONCAT22((int)puVar10,uVar5),
                    (ushort)((ulong)puVar10 >> 0x10),uVar7);
    if ((int)puVar1 < 0x0) break;
    uVar4 = uVar5;
    if ((int)puVar1 < 0x65) {
      return;
    }
  }
  return;
}
