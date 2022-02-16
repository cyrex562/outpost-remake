


void __stdcall16far pass1_1030_4bbe(ushort param_1,ushort param_2,ulong param_3,int param_4)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined2 uVar3;
  int iVar4;
  astruct_117 *iVar5;
  undefined4 *puVar5;
  undefined4 *puVar6;
  uint uVar7;
  
  uVar7 = (uint)(param_3 >> 0x10);
  iVar5 = (astruct_117 *)param_3;
  if (iVar5->field_0x12 == 0x0) {
    pass1_1030_4f5a(param_1,param_2,param_3 & 0xffff | (ulong)uVar7 << 0x10);
  }
  puVar6 = &iVar5->field_0x16;
  uVar3 = *(undefined2 *)((int)&iVar5->field_0x12 + 0x2);
  puVar5 = (undefined4 *)(*(int *)&iVar5->field_0x12 + param_4 * 0x98);
  for (iVar4 = 0x26; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
    puVar2 = puVar6;
    puVar6 = puVar6 + 0x1;
    puVar1 = puVar5;
    puVar5 = puVar5 + 0x1;
    *puVar2 = *puVar1;
  }
  return;
}



void __stdcall16far pass1_1030_4c06(ulong param_1,int param_2,uint16_t param_3,ushort param_4)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined2 uVar3;
  undefined4 *puVar4;
  int iVar5;
  undefined4 *puVar6;
  uint uVar7;
  
  uVar7 = (uint)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(long *)(iVar5 + 0x15c) == 0x0) {
    pass1_1030_5044(param_1 & 0xffff | (ulong)uVar7 << 0x10,param_4,param_3);
  }
  puVar4 = (undefined4 *)(iVar5 + 0xae);
  uVar3 = *(undefined2 *)(iVar5 + 0x15e);
  puVar6 = (undefined4 *)(*(int *)(iVar5 + 0x15c) + param_2 * 0xae);
  for (iVar5 = 0x2b; iVar5 != 0x0; iVar5 = iVar5 + -0x1) {
    puVar2 = puVar4;
    puVar4 = puVar4 + 0x1;
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    *puVar2 = *puVar1;
  }
  *(undefined2 *)puVar4 = *(undefined2 *)puVar6;
  return;
}



void __stdcall16far
pass1_1030_4c52(ushort param_1,ushort param_2,ulong param_3,ulong param_4,uint param_5,ushort param_6)

{
  ushort uVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  char *pcStack8;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    uVar1 = pass1_1000_47a4(param_4,0x1050518a,param_6);
    pcStack8 = (char *)CONCAT22(param_5,uVar1);
    if ((param_5 | uVar1) == 0x0) break;
    if (*pcStack8 != '\"') {
      iVar2 = pass1_1000_3e2c(CONCAT22(param_5,uVar1));
      iVar3 = (int)param_3;
      uVar4 = (undefined2)(param_3 >> 0x10);
      if (iStack4 < 0x25) {
        *(int *)(iStack4 * 0x4 + iVar3) = iVar2;
        *(uint *)(iStack4 * 0x4 + iVar3 + 0x2) = param_5;
      }
      else {
        if (iStack4 == 0x25) {
          *(int *)(iVar3 + 0x94) = iVar2;
        }
        else {
          if (iStack4 == 0x26) {
            *(int *)(iVar3 + 0x96) = iVar2;
          }
          else {
            if (iStack4 == 0x27) {
              *(int *)(iVar3 + 0x98) = iVar2;
            }
            else {
              if (iStack4 == 0x28) {
                *(int *)(iVar3 + 0x9a) = iVar2;
              }
              else {
                if (iStack4 == 0x29) {
                  *(int *)(iVar3 + 0x9c) = iVar2;
                }
                else {
                  if (iStack4 == 0x2a) {
                    *(int *)(iVar3 + 0x9e) = iVar2;
                  }
                  else {
                    if (iStack4 == 0x2b) {
                      *(int *)(iVar3 + 0xa0) = iVar2;
                    }
                    else {
                      if (iStack4 == 0x2c) {
                        *(int *)(iVar3 + 0xa2) = iVar2;
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      iStack4 = iStack4 + 0x1;
    }
    param_4 = 0x0;
  }
  return;
}



void __stdcall16far pass1_1030_4d3a(uint param_1,ushort param_2,ushort param_3,ulong param_4,ulong param_5)

{
  ushort uVar1;
  int iVar2;
  astruct_118 *iVar3;
  undefined2 uVar3;
  ushort unaff_SS;
  char *pcStack8;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    uVar1 = pass1_1000_47a4(param_5,0x1050518a,unaff_SS);
    pcStack8 = (char *)CONCAT22(param_1,uVar1);
    if ((param_1 | uVar1) == 0x0) break;
    if (*pcStack8 != '\"') {
      iVar2 = pass1_1000_3e2c(CONCAT22(param_1,uVar1));
      iVar3 = (astruct_118 *)param_4;
      uVar3 = (undefined2)(param_4 >> 0x10);
      if (iStack4 < 0x25) {
        *(int *)(&iVar3->field_0x0 + iStack4 * 0x4) = iVar2;
        *(uint *)(&iVar3->field_0x2 + iStack4 * 0x4) = param_1;
      }
      else {
        if (iStack4 == 0x25) {
          iVar3->field_0x94 = iVar2;
        }
        else {
          if (iStack4 == 0x26) {
            iVar3->field_0x96 = iVar2;
          }
        }
      }
      iStack4 = iStack4 + 0x1;
    }
    param_5 = 0x0;
  }
  return;
}



void __stdcall16far pass1_1030_4dbc(ulong param_1,ulong param_2,long param_3)

{
  long *plVar1;
  int *piVar2;
  long lVar3;
  uint uVar4;
  int iVar5;
  undefined2 uVar6;
  
  iVar5 = (int)param_1;
  uVar6 = (undefined2)(param_1 >> 0x10);
  if (0x0 < param_3) {
    *(ulong *)(iVar5 + 0x160) = param_2;
    *(long *)(iVar5 + 0x164) = param_3;
  }
  if ((*(long *)(iVar5 + 0x160) == 0x0) ||
     (lVar3 = *(long *)(iVar5 + 0x164), plVar1 = (long *)(iVar5 + 0x164), *plVar1 = *plVar1 + -0x1, lVar3 == 0x0)) {
    *(undefined4 *)(iVar5 + 0x160) = 0x0;
  }
  else {
    uVar4 = str_op_1000_3da4(*(char **)(iVar5 + 0x160));
    piVar2 = (int *)(iVar5 + 0x160);
    *piVar2 = *piVar2 + uVar4 + 0x2;
  }
  return;
}



void __stdcall16far pass1_1030_4e34(ushort param_1,ushort param_2,long param_3,char *param_4)

{
  while (param_3 != 0x0) {
    if ((*param_4 == '\r') || (*param_4 == '\n')) {
      *param_4 = '\0';
    }
    param_4 = (char *)((ulong)param_4 & 0xffff0000 | (ulong)((int)param_4 + 0x1));
    param_3 = param_3 + -0x1;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uint16_t __stdcall16far
read_file_1030_4e70(ulong param_1,undefined4 *param_2,byte **param_3,long param_4,uint16_t param_5)

{
  uint uVar1;
  HFILE16 HVar2;
  uint16_t uVar3;
  ushort unaff_SS;
  ulong uVar4;
  long lVar5;
  byte *pbStack60;
  long lStack56;
  undefined4 uStack20;
  
  *param_3 = (byte *)0x0;
  *param_2 = 0x0;
  if (param_4 != 0x0) {
    uVar4 = pass1_1030_5164(param_1,param_4,unaff_SS);
    param_5 = (uint16_t)(uVar4 >> 0x10);
    uVar1 = dos3_call_1000_51aa((ushort)&stack0xfffe);
    if (uVar1 == 0x0) {
      *param_2 = uStack20;
      HVar2 = _lopen16((LPCSTR)&PTR_LOOP_1050_1000,0x0);
      if (HVar2 != 0xffff) {
        lVar5 = mem_op_1000_0a48(0x1,(uint)*param_2,(int)((ulong)*param_2 >> 0x10),_PTR_LOOP_1050_5f2c,0x1000);
        param_5 = (uint16_t)((ulong)lVar5 >> 0x10);
        *(int *)param_3 = (int)lVar5;
        *(uint *)((int)param_3 + 0x2) = param_5;
        if ((param_5 | *(uint *)param_3) != 0x0) {
          lStack56 = WIN16_hread(0x1000,(SEGPTR)*param_2,CONCAT22((int)*param_3,(int)((ulong)*param_2 >> 0x10)));
          uVar3 = (uint16_t)((ulong)lStack56 >> 0x10);
          _lclose16((HFILE16)s_tile2_bmp_1050_1538);
          pbStack60 = *param_3;
          while (lStack56 != 0x0) {
            if ((*(byte *)(*pbStack60 + 0x608b) & 0x20) == 0x0) {
              *pbStack60 = *pbStack60 + 0x80;
            }
            pbStack60 = (byte *)((ulong)pbStack60 & 0xffff0000 | (ulong)((int)pbStack60 + 0x1));
            lStack56 = lStack56 + -0x1;
          }
          return uVar3;
        }
      }
    }
  }
  return param_5;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_4f5a(ushort param_1,ushort param_2,ulong param_3)

{
  uint uVar1;
  char *pcVar2;
  long *plVar3;
  ushort uVar4;
  int iVar5;
  char *pcVar6;
  undefined2 extraout_DX;
  undefined2 extraout_DX_00;
  ushort uVar7;
  ushort uVar8;
  uint uStack22;
  ulong uStack20;
  u16 uStack14;
  uint uStack12;
  long local_a;
  char *local_6;
  
  plVar3 = &local_a;
  PTR_LOOP_1050_5f2e =
       (undefined *)
       read_file_1030_4e70(param_3,(undefined4 *)CONCAT22(param_1,plVar3),(byte **)CONCAT22(param_1,&local_6),
                           (long)s_bldgbld_dat_1050_56fc,param_2);
  pcVar2 = local_6;
  if (plVar3 != (long *)0x0) {
    uVar7 = (ushort)param_3;
    uVar8 = (ushort)(param_3 >> 0x10);
    pcVar6 = local_6;
    pass1_1030_4e34(uVar7,uVar8,local_a,local_6);
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708((uint)pcVar6 * 0x98,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
    *(ushort *)(uVar7 + 0x12) = uVar4;
    *(undefined2 *)(uVar7 + 0x14) = PTR_LOOP_1050_5f2e;
    pass1_1030_4dbc(param_3,(ulong)local_6,(ulong)pcVar6 & 0xffff);
    uStack20 = CONCAT22(extraout_DX,uVar4);
    for (uStack22 = 0x0; uStack22 < (uint)pcVar6; uStack22 = uStack22 + 0x1) {
      uVar1 = *(uint *)(uVar7 + 0x14);
      iVar5 = *(int *)(uVar7 + 0x12) + uStack22 * 0x98;
      pass1_1030_4d3a(uVar1,uVar7,uVar8,CONCAT22(uVar1,iVar5),uStack20);
      pass1_1030_4dbc(param_3,0x0,0x0);
      uStack20 = CONCAT22(extraout_DX_00,iVar5);
    }
    uStack12 = (uint)((ulong)pcVar2 >> 0x10);
    uStack14 = (u16)pcVar2;
    if ((uStack12 | uStack14) != 0x0) {
      call_fn_ptr_1000_0dc6(uStack14,uStack12,0x1000);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_5044(ulong param_1,ushort param_2,uint16_t param_3)

{
  uint uVar1;
  char *pcVar2;
  long *plVar3;
  uint uVar4;
  ushort uVar5;
  undefined2 uVar6;
  int iVar7;
  char *pcVar8;
  undefined2 extraout_DX;
  undefined2 extraout_DX_00;
  ushort uVar10;
  ushort uVar11;
  ulong uStack28;
  uint uStack24;
  ulong uStack22;
  u16 uStack14;
  uint uStack12;
  long local_a;
  char *local_6;
  ulong uVar9;
  
  plVar3 = &local_a;
  PTR_LOOP_1050_5f2e =
       (undefined *)
       read_file_1030_4e70(param_1,(undefined4 *)CONCAT22(param_2,plVar3),(byte **)CONCAT22(param_2,&local_6),
                           (long)s_bldgops_dat_1050_5708,param_3);
  pcVar2 = local_6;
  if (plVar3 != (long *)0x0) {
    uVar10 = (ushort)param_1;
    uVar11 = (ushort)(param_1 >> 0x10);
    pcVar8 = local_6;
    pass1_1030_4e34(uVar10,uVar11,local_a,local_6);
    uVar4 = (uint)pcVar8;
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar5 = fn_ptr_op_1000_1708(uVar4 * 0xae,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
    uVar9 = (ulong)uVar5;
    uStack28 = CONCAT22(PTR_LOOP_1050_5f2e,uVar5);
    if (((uint)PTR_LOOP_1050_5f2e | uVar5) == 0x0) {
      *(undefined4 *)(uVar10 + 0x15c) = 0x0;
    }
    else {
      pass1_1000_5586((uchar *)0x51f0,0x1030,uVar4,0xae,uVar5,(ushort)PTR_LOOP_1050_5f2e);
      *(ulong *)(uVar10 + 0x15c) = uStack28;
      uVar9 = uStack28;
    }
    uVar6 = (undefined2)uVar9;
    pass1_1030_4dbc(param_1,(ulong)local_6,(ulong)pcVar8 & 0xffff);
    uStack22 = CONCAT22(extraout_DX,uVar6);
    for (uStack24 = 0x0; uStack24 < uVar4; uStack24 = uStack24 + 0x1) {
      uVar1 = *(uint *)(uVar10 + 0x15e);
      iVar7 = *(int *)(uVar10 + 0x15c) + uStack24 * 0xae;
      pass1_1030_4c52(uVar10,uVar11,CONCAT22(uVar1,iVar7),uStack22,uVar1,param_2);
      pass1_1030_4dbc(param_1,0x0,0x0);
      uStack22 = CONCAT22(extraout_DX_00,iVar7);
    }
    uStack12 = (uint)((ulong)pcVar2 >> 0x10);
    uStack14 = (u16)pcVar2;
    if ((uStack12 | uStack14) != 0x0) {
      call_fn_ptr_1000_0dc6(uStack14,uStack12,0x1000);
    }
  }
  return;
}



ulong __stdcall16far pass1_1030_5164(ulong param_1,ULONG param_2,ushort param_3)

{
  uint uVar1;
  uint uVar2;
  long lVar3;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0x568));
  do {
    lVar3 = pass1_1008_5b12(local_a,param_3);
    if (lVar3 == 0x0) {
      return (ulong)param_2;
    }
    uVar1 = (int)param_1 + 0x168;
    unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (ulong)uVar1),*(char **)((int)lVar3 + 0x4));
    pass1_1000_3cea(param_1 & 0xffff0000 | (ulong)uVar1,param_2);
    uVar2 = dos3_call_1000_51aa((ushort)&stack0xfffe);
  } while (uVar2 != 0x0);
  return param_1 & 0xffff0000 | (ulong)uVar1;
}



void __cdecl16far pass1_1030_51eb(void)

{
  undefined2 unaff_SS;
  
  pass1_1030_3b28(unaff_SS);
  return;
}



ulong __stdcall16far pass1_1030_51f0(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0xa4) = 0x0;
  *(undefined2 *)(iVar1 + 0xa6) = 0x0;
  *(undefined2 *)(iVar1 + 0xa8) = 0x0;
  *(undefined2 *)(iVar1 + 0xaa) = 0x0;
  *(undefined2 *)(iVar1 + 0xac) = 0x0;
  return param_1;
}



void __stdcall16far pass1_1030_521c(astruct_100 *param_1,ulong param_2,ushort param_3,uchar param_4)

{
  int iVar1;
  uchar *puVar2;
  
  struct_op_1028_d1dc(param_3,param_4,param_1,0x32c7);
  puVar2 = (uchar *)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ulong *)(iVar1 + 0x108) = param_2;
  param_1->field_0x0 = 0x55fe;
  *(undefined2 *)(iVar1 + 0x2) = 0x1030;
  sys_1000_3f9c((uchar *)(iVar1 + 0x8),puVar2,(ushort)s_SCGenKids_0x_08lx_1050_5714,(ushort)&USHORT_1050_1050,
                (ushort)param_2,&stack0xfffe,puVar2,0x1000,param_3,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1030_5260(ulong param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  code **ppcVar2;
  undefined4 *puStack6;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x108);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  puStack6 = (undefined4 *)CONCAT22(param_3,param_2);
  ppcVar2 = (code **)((int)*puStack6 + 0x14);
  (**ppcVar2)();
  return 0x1;
}



void __stdcall16far pass1_1030_5290(ulong param_1,astruct_376 *param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_377 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x10c,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | (uint)param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (astruct_377 *)param_1;
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
    *puStack10 = 0x55fe;
    param_2->field_0x2 = 0x1030;
  }
  return;
}



void __stdcall16far pass1_1030_532e(astruct_100 *param_1,ulong param_2,ushort param_3,uchar param_4)

{
  int iVar1;
  uchar *puVar2;
  
  struct_op_1028_d1dc(param_3,param_4,param_1,0x32c7);
  puVar2 = (uchar *)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ulong *)(iVar1 + 0x108) = param_2;
  param_1->field_0x0 = 0x55ee;
  *(undefined2 *)(iVar1 + 0x2) = 0x1030;
  sys_1000_3f9c((uchar *)(iVar1 + 0x8),puVar2,(ushort)s_SCSelect__u___d_1050_5726,(ushort)&USHORT_1050_1050,
                (ushort)*(undefined4 *)(iVar1 + 0x4),&stack0xfffe,puVar2,0x1000,param_3,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1030_538a(ulong param_1,int param_2,ushort param_3)

{
  uchar *puVar1;
  uint uVar2;
  astruct_694 *iVar4;
  undefined2 uVar3;
  ushort *puVar4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_694 *)param_1;
  puVar1 = *(uchar **)((int)&iVar4->field_0x108 + 0x2);
  uVar2 = (uint)puVar1 >> 0x8;
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,puVar1,param_2);
  if (uVar2 == 0x1) {
    pass1_1018_04ca((ulong)puVar4,iVar4->field_0x108);
  }
  else {
    if (uVar2 == 0x2) {
      pass1_1018_04a4((ulong)puVar4,iVar4->field_0x108);
    }
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_53f4(ulong param_1,ushort param_2,ushort param_3,uchar param_4)

{
  undefined4 uVar1;
  uint uVar2;
  int iVar3;
  undefined2 uVar4;
  ulong uVar5;
  byte bStack291;
  undefined local_11e [0x10e];
  undefined4 uStack16;
  undefined4 uStack12;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  uStack12 = *(undefined4 *)(iVar3 + 0x108);
  uStack12._3_1_ = (char)((ulong)uStack12 >> 0x18);
  if (uStack12._3_1_ == -0x1) {
    uVar5 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,param_2,(byte)((ulong)*(undefined4 *)(iVar3 + 0x108) >> 0x18));
    param_2 = (ushort)(uVar5 >> 0x10);
  }
  else {
    uStack16 = (ulong *)*(undefined4 *)(iVar3 + 0x108);
    uStack16._3_1_ = (char)((ulong)uStack16 >> 0x18);
    if (uStack16._3_1_ == '\x03') {
      pass1_1028_e44a(_PTR_LOOP_1050_65e2,*(long *)(iVar3 + 0x108),param_3);
    }
    else {
      uVar1 = *(undefined4 *)(iVar3 + 0x108);
      pass1_1028_e372(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10),param_3);
    }
  }
  uStack12 = *(undefined4 *)(iVar3 + 0x108);
  uStack12._3_1_ = (char)((ulong)uStack12 >> 0x18);
  if (uStack12._3_1_ != '\x03') {
    pass1_1030_521c((astruct_100 *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,local_11e)),
                    *(ulong *)(iVar3 + 0x108),param_3,param_4);
    uStack16 = *_PTR_LOOP_1050_5748;
    fn_ptr_1028_d566(uStack16,(ulong *)CONCAT22(param_3,local_11e));
    bStack291 = (byte)((ulong)*(undefined4 *)(iVar3 + 0x108) >> 0x18);
    uVar2 = (uint)bStack291;
    if (bStack291 == 0x2) {
      uVar1 = *(undefined4 *)(iVar3 + 0x108);
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
      pass1_1010_82f8(_PTR_LOOP_1050_14cc,**(ushort **)(uVar2 + 0x10));
    }
  }
  return;
}



void pass1_1030_54f8(astruct_378 *param_1,uchar *param_2,ulong param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  astruct_379 *iVar5;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x10c,param_2,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_2,param_1);
  if (((uint)param_2 | (uint)param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field_0x2 = 0x1008;
    uVar6 = (undefined2)(param_3 >> 0x10);
    iVar5 = (astruct_379 *)param_3;
    param_1->field_0x4 = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = &param_1->field_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field_0x2 = (int)&USHORT_1050_1028;
    param_1->field_0x108 = iVar5->field_0x108;
    *puStack10 = 0x55ee;
    param_1->field_0x2 = 0x1030;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1030_5596(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_18 * __stdcall16far pass1_1030_55c2(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1030_560e(ushort *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_1030_17ce(param_1,0x64,0x1f4);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0x10) = 0x0;
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x14)));
  *(undefined2 *)(iVar1 + 0x1a) = 0x0;
  *(undefined2 *)(iVar1 + 0x1c) = 0x0;
  *param_1 = (ushort)s_procLo_1050_5bd0;
  *(undefined2 *)(iVar1 + 0x2) = 0x1030;
  return param_1;
}



ushort * __stdcall16far struct_1030_565a(ushort *param_1,ulong param_2,uint param_3,uchar *param_4)

{
  astruct_353 *iVar1;
  undefined2 uVar1;
  
  pass1_1030_183c(param_1,0x64,0x1f4,0x3000000,param_2,param_3,param_4);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_353 *)param_1;
  iVar1->field_0x10 = 0x0;
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x14));
  iVar1->field_0x1a = 0x0;
  iVar1->field_0x1c = 0x0;
  *param_1 = (ushort)s_procLo_1050_5bd0;
  iVar1->field_0x2 = 0x1030;
  return param_1;
}



void __stdcall16far pass1_1030_56b0(ushort *param_1)

{
  uint uVar1;
  astruct_18 *paVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  *param_1 = (ushort)s_procLo_1050_5bd0;
  *(undefined2 *)(iVar3 + 0x2) = 0x1030;
  paVar2 = *(astruct_18 **)(iVar3 + 0x10);
  uVar1 = *(uint *)(iVar3 + 0x12);
  if ((uVar1 | (uint)paVar2) != 0x0) {
    fn_ptr_1030_84d0((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10);
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  pass1_1030_18b2(param_1);
  return;
}



void __stdcall16far pass1_1030_56f6(ulong param_1,ulong param_2,uint16_t param_3,ushort param_4)

{
  int *piVar1;
  ulong uVar2;
  undefined4 uVar3;
  ushort uVar4;
  BOOL16 BVar5;
  int iVar6;
  int iVar7;
  undefined2 uVar8;
  ushort uVar9;
  undefined2 local_e [0x3];
  undefined2 local_8 [0x2];
  int iStack4;
  
  uVar4 = pass1_1030_1978(param_1,param_2,param_3,param_4);
  if (uVar4 != 0x0) {
    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar7 = (int)param_1;
    local_e[0] = *(undefined2 *)*(undefined4 *)(iVar7 + 0x10);
    uVar4 = (ushort)param_2;
    uVar9 = (ushort)(param_2 >> 0x10);
    BVar5 = write_to_file_1008_7e1c(uVar4,uVar9,(ushort)local_e,param_4,(char *)0x2,0x1008);
    if (BVar5 != 0x0) {
      uVar3 = *(undefined4 *)(iVar7 + 0x10);
      local_8[0] = *(undefined2 *)((int)uVar3 + 0x2);
      BVar5 = write_to_file_1008_7e1c(uVar4,uVar9,(ushort)local_8,param_4,(char *)0x2,0x1008);
      if ((BVar5 != 0x0) &&
         (uVar3 = *(undefined4 *)(iVar7 + 0x10), BVar5 = pass1_1008_7c2a(param_2,*(char **)((int)uVar3 + 0x4),0x1008),
         BVar5 != 0x0)) {
        uVar3 = *(undefined4 *)(iVar7 + 0x10);
        local_8[0] = *(undefined2 *)((int)uVar3 + 0x1a);
        BVar5 = write_to_file_1008_7e1c(uVar4,uVar9,(ushort)local_8,param_4,(char *)0x2,0x1008);
        if (BVar5 != 0x0) {
          for (iStack4 = 0x0; uVar3 = *(undefined4 *)(iVar7 + 0x10), piVar1 = (int *)((int)uVar3 + 0x1a),
              *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 = iStack4 + 0x1) {
            uVar3 = *(undefined4 *)(iVar7 + 0x10);
            uVar2 = *(ulong *)((int)uVar3 + 0x16);
            iVar6 = write_to_file_1008_7b4c
                              (param_2,uVar2 & 0xffff0000 | (ulong)(uint)((int)uVar2 + iStack4 * 0x6),0x1008,param_4);
            if (iVar6 == 0x0) goto LAB_1030_5734;
          }
          iVar6 = write_to_file_1008_7b4c(param_2,param_1 & 0xffff0000 | (ulong)(iVar7 + 0x14),0x1008,param_4);
          if (iVar6 != 0x0) {
            local_8[0] = *(undefined2 *)(iVar7 + 0x1c);
            BVar5 = write_to_file_1008_7e1c(uVar4,uVar9,(ushort)local_8,param_4,(char *)0x2,0x1008);
            if (BVar5 != 0x0) {
              return;
            }
          }
        }
      }
    }
LAB_1030_5734:
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far file_1030_581e(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  int *piVar1;
  int iVar2;
  ulong uVar3;
  ushort uVar4;
  BOOL16 BVar5;
  undefined *puVar6;
  ushort uVar7;
  undefined4 uVar8;
  uchar *puVar9;
  astruct_380 *iVar9;
  undefined2 uVar10;
  uchar in_AF;
  ushort uVar11;
  ushort uVar12;
  undefined4 uStack1040;
  int iStack1036;
  undefined local_408 [0x400];
  undefined4 uStack8;
  int local_4;
  astruct_381 *iVar12;
  
  iVar12 = (astruct_381 *)param_1;
  uVar12 = (ushort)(param_1 >> 0x10);
  file_1030_19b4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 != 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)param_4,0x1000);
      PTR_LOOP_1050_5f2e = param_4;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(0x20,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
    puVar9 = (uchar *)((uint)PTR_LOOP_1050_5f2e | uVar4);
    if (puVar9 == (uchar *)0x0) {
      uVar4 = 0x0;
      puVar9 = (uchar *)0x0;
    }
    else {
      pass1_1030_84ae(CONCAT22(PTR_LOOP_1050_5f2e,uVar4));
    }
    iVar12->field_0x10 = uVar4;
    iVar12->field_0x12 = puVar9;
    uVar4 = (ushort)param_2;
    uVar11 = (ushort)(param_2 >> 0x10);
    BVar5 = read_file_1008_7dee(uVar4,uVar11,(ushort)&local_4,0x0,param_5,0x2,0x1008);
    if (BVar5 != 0x0) {
      uVar8 = *(undefined4 *)((int)_PTR_LOOP_1050_65e2 + 0x52);
      uStack8 = uVar8;
      pass1_1030_4782(param_5,in_AF,puVar9,(ushort)uVar8,(ushort)((ulong)uVar8 >> 0x10),0x0,0x1,local_4);
      iVar12->field_0x10 = (ushort)uVar8;
      iVar12->field_0x12 = puVar9;
      BVar5 = read_file_1008_7dee(uVar4,uVar11,iVar12->field_0x10 + 0x2,0x0,(ushort)puVar9,0x2,0x1008);
      if (BVar5 != 0x0) {
        puVar6 = local_408;
        read_file_1008_7c6e(uVar4,uVar11,(char *)CONCAT22(param_5,puVar6),0x1008);
        if (puVar6 != (undefined *)0x0) {
          uVar8 = *(undefined4 *)&iVar12->field_0x10;
          fn_ptr_1000_17ce(*(astruct_18 **)((int)uVar8 + 0x4),0x1000);
          uVar7 = str_op_1008_60e8((char *)CONCAT22(param_5,local_408),(ushort)puVar9);
          uVar8 = *(undefined4 *)&iVar12->field_0x10;
          uVar10 = (undefined2)((ulong)uVar8 >> 0x10);
          iVar9 = (astruct_380 *)uVar8;
          iVar9->field_0x4 = uVar7;
          iVar9->field_0x6 = puVar9;
          uVar8 = *(undefined4 *)&iVar12->field_0x10;
          BVar5 = read_file_1008_7dee(uVar4,uVar11,(int)uVar8 + 0x1a,0x0,(ushort)((ulong)uVar8 >> 0x10),0x2,0x1008);
          if (BVar5 != 0x0) {
            uVar8 = *(undefined4 *)&iVar12->field_0x10;
            iVar2 = *(int *)((int)uVar8 + 0x1a);
            uVar7 = iVar2 * 0x6;
            mem_op_1000_179c(uVar7,puVar9,0x1000);
            uStack1040 = CONCAT22(puVar9,uVar7);
            if (((uint)puVar9 | uVar7) == 0x0) {
              uVar8 = *(undefined4 *)&iVar12->field_0x10;
              *(undefined4 *)((int)uVar8 + 0x16) = 0x0;
            }
            else {
              pass1_1000_5586((uchar *)0x3e38,0x1008,iVar2,0x6,uVar7,(ushort)puVar9);
              uVar8 = *(undefined4 *)&iVar12->field_0x10;
              *(undefined4 *)((int)uVar8 + 0x16) = uStack1040;
            }
            for (iStack1036 = 0x0; uVar8 = *(undefined4 *)&iVar12->field_0x10, piVar1 = (int *)((int)uVar8 + 0x1a),
                *piVar1 != iStack1036 && iStack1036 <= *piVar1; iStack1036 = iStack1036 + 0x1) {
              uVar8 = *(undefined4 *)&iVar12->field_0x10;
              uVar3 = *(ulong *)((int)uVar8 + 0x16);
              BVar5 = read_file_1008_7bc8(param_2,(ushort *)
                                                  (uVar3 & 0xffff0000 | (ulong)(uint)((int)uVar3 + iStack1036 * 0x6)),
                                          0x1008,param_5);
              if (BVar5 == 0x0) goto LAB_1030_58a7;
            }
            BVar5 = read_file_1008_7bc8(param_2,(ushort *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar12->field_0x14),
                                        0x1008,param_5);
            if ((BVar5 != 0x0) &&
               (BVar5 = read_file_1008_7dee(uVar4,uVar11,&iVar12->field_0x1c,0x0,uVar12,0x2,0x1008), BVar5 != 0x0)) {
              return;
            }
          }
        }
      }
    }
LAB_1030_58a7:
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



void __stdcall16far pass1_1030_5a52(ulong param_1,ulong *param_2,ulong *param_3)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x10);
  *param_3 = *(ulong *)((int)uVar1 + 0xe);
  uVar1 = *(undefined4 *)((int)param_1 + 0x10);
  *param_2 = *(ulong *)((int)uVar1 + 0x12);
  return;
}



void __stdcall16far pass1_1030_5a80(ulong param_1,ulong param_2,ushort param_3)

{
  ulong *puVar1;
  ushort uVar2;
  ulong uVar3;
  undefined local_20 [0xc];
  ulong local_14;
  undefined4 uStack14;
  undefined4 uStack10;
  int iStack6;
  undefined2 uStack4;
  
  uVar2 = (ushort)(param_1 >> 0x10);
  *(ulong *)((int)param_1 + 0x10) = param_2;
  uVar3 = pass1_1008_4772(*(astruct_76 **)((int)param_2 + 0xe));
  uStack4 = (undefined2)(uVar3 >> 0x10);
  iStack6 = (int)uVar3;
  uStack10 = *(undefined4 *)(iStack6 + 0x4);
  uStack14 = *(undefined4 *)(iStack6 + 0x8);
  pass1_1008_3e54((ushort *)CONCAT22(param_3,&local_14),0x0,(int)uStack14 - 0x1,(int)uStack10 - 0x1);
  puVar1 = (ulong *)((int)param_1 + 0x14);
  pass1_1008_6cb4((ulong *)CONCAT22(param_3,local_20),&local_14,param_3,puVar1,uVar2);
  pass1_1008_6d64((ushort *)CONCAT22(param_3,local_20),(ushort *)(param_1 & 0xffff0000 | ZEXT24(puVar1)));
  return;
}



int __stdcall16far pass1_1030_5b00(ulong param_1)

{
  return *(int *)((int)param_1 + 0x4) + 0xb;
}



void __stdcall16far pass1_1030_5b1c(ulong param_1,ushort *param_2,ushort *param_3)

{
  ushort uVar1;
  
  uVar1 = (ushort)(param_1 >> 0x10);
  *param_3 = *(ushort *)((int)param_1 + 0x1a);
  *param_2 = *(ushort *)((int)param_1 + 0x1c);
  return;
}



void __stdcall16far pass1_1030_5b3e(ulong param_1,int param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(ushort *)(iVar1 + 0x1a) = param_3;
  if (*(int *)(iVar1 + 0x1c) < param_2) {
    *(int *)(iVar1 + 0x1c) = param_2;
  }
  return;
}

