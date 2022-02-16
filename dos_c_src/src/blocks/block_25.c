
void __stdcall16far pass1_1010_038e(ulong param_1,int param_2,ushort param_3)

{
  bool bVar1;
  astruct_707 *iVar2;
  undefined2 uVar2;
  
  bVar1 = false;
  iVar2 = (astruct_707 *)param_1;
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (param_2 != 0x0) {
    if (iVar2->field_0x18 == 0x0) {
      iVar2->field_0x12 = DAT_1050_0e28;
      iVar2->field_0x14 = PTR_LOOP_1050_0e30;
      iVar2->field_0x16 = PTR_LOOP_1050_0ea8;
      DAT_1050_0e28 = 0x0;
      PTR_LOOP_1050_0e30 = (undefined *)0x0;
      PTR_LOOP_1050_0ea8 = (undefined *)0x0;
      iVar2->field_0x18 = 0x1;
      bVar1 = true;
      goto LAB_1010_0404;
    }
  }
  if (param_2 == 0x0) {
    if (iVar2->field_0x18 != 0x0) {
      DAT_1050_0e28 = iVar2->field_0x12;
      PTR_LOOP_1050_0e30 = iVar2->field_0x14;
      PTR_LOOP_1050_0ea8 = iVar2->field_0x16;
      iVar2->field_0x18 = 0x0;
      bVar1 = true;
    }
  }
LAB_1010_0404:
  if (bVar1) {
    pass1_1010_1f62(param_3,param_1,0x3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far pass1_1010_041a(void)

{
  ulong uVar1;
  
  uVar1 = pass1_1030_8326();
  if (((int)(uVar1 >> 0x10) == 0x0) && ((uint)uVar1 < 0x64)) {
    return 0x0;
  }
  return 0x1;
}



void __stdcall16far pass1_1010_043a(ulong param_1,long param_2,int param_3,ushort param_4)

{
  undefined4 *puVar1;
  code **ppcVar2;
  astruct_225 *puVar3;
  uint extraout_DX;
  uint uVar3;
  astruct_226 *iVar4;
  astruct_227 *iVar5;
  undefined2 uVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  undefined2 *puStack18;
  undefined2 *puStack14;
  undefined local_a [0x8];
  
  iVar4 = (astruct_226 *)param_1;
  uVar4 = (undefined2)(param_1 >> 0x10);
  if (param_3 == 0xc) {
    if (iVar4->field_0xe != 0x0) {
      return;
    }
    iVar4->field_0xe = 0x1;
  }
  else {
    if (param_3 == 0xd) {
      if (iVar4->field_0x10 != 0x0) {
        return;
      }
      iVar4->field_0x10 = 0x1;
    }
    else {
      if (param_3 == 0x12) {
        return;
      }
    }
  }
  pass1_1010_089e(param_4,param_1,0x1,0x6);
  pass1_1008_5784((ulong *)CONCAT22(param_4,local_a),(ulong)iVar4->field_0xa);
  do {
    puVar3 = (astruct_225 *)local_a;
    pass1_1008_5b12(puVar3,param_4);
    uVar3 = extraout_DX | (uint)puVar3;
    if (uVar3 == 0x0) {
      uVar6 = 0xa;
      mem_op_1000_179c(0xa,(uchar *)0x0,0x1000);
      puStack18 = (undefined2 *)CONCAT22(uVar3,puVar3);
      if ((uVar3 | (uint)puVar3) == 0x0) {
        puStack14 = (undefined2 *)0x0;
      }
      else {
        *puStack18 = 0x389a;
        *(undefined2 *)((int)&puVar3->field_0x0 + 0x2) = 0x1008;
        *puStack18 = 0xea8;
        *(undefined2 *)((int)&puVar3->field_0x0 + 0x2) = 0x1010;
        puStack14 = puStack18;
      }
      uVar5 = (undefined2)((ulong)puStack14 >> 0x10);
      iVar5 = (astruct_227 *)puStack14;
      iVar5->field_0x4 = param_3;
      iVar5->field_0x6 = param_2;
      puVar1 = iVar4->field_0xa;
      ppcVar2 = (code **)((int)*iVar4->field_0xa + 0x8);
      (**ppcVar2)(0x1000,(int)puVar1,(char)((ulong)puVar1 >> 0x10),iVar5,uVar5,uVar6);
      return;
    }
  } while ((puVar3->field_0x4 != param_3) || (puVar3->field_0x6 != param_2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_0538(ulong param_1,char **param_2,char **param_3,ushort param_4,ushort param_5)

{
  int iVar1;
  ulong uVar2;
  code **ppcVar3;
  uint uVar4;
  int iVar5;
  char *pcVar6;
  uchar *puVar7;
  uint extraout_DX;
  uchar *puVar8;
  uchar *extraout_DX_00;
  ushort uVar9;
  ushort uVar10;
  undefined2 uVar11;
  undefined4 uVar12;
  undefined4 *puStack6;
  
  uVar4 = 0x0;
  *param_3 = (char *)0x0;
  *param_2 = (char *)0x0;
  uVar10 = (ushort)(param_1 >> 0x10);
  uVar9 = (ushort)param_1;
  uVar12 = *(undefined4 *)(uVar9 + 0xa);
  ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar9 + 0xa) + 0x10);
  (**ppcVar3)();
  puStack6 = (undefined4 *)CONCAT22(extraout_DX,uVar4);
  puVar8 = (uchar *)(extraout_DX | uVar4);
  if (puVar8 == (uchar *)0x0) {
    return;
  }
  iVar1 = *(int *)(uVar4 + 0x4);
  uVar2 = *(ulong *)(uVar4 + 0x6);
  if ((extraout_DX | uVar4) != 0x0) {
    ppcVar3 = (code **)*puStack6;
    (**ppcVar3)(param_4,uVar4,(char)extraout_DX,0x1,uVar12);
    puVar8 = extraout_DX_00;
  }
  uVar12 = *(undefined4 *)(uVar9 + 0xa);
  if (*(int *)((int)uVar12 + 0x8) == 0x0) {
    pass1_1010_089e(param_5,param_1,0x0,0x6);
    pass1_1010_1f62(param_5,param_1,0x3);
  }
  iVar5 = iVar1 + 0x757;
  load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),param_4);
  *(int *)param_3 = iVar5;
  *(uchar **)((int)param_3 + 0x2) = puVar8;
  while (pcVar6 = pass1_1000_472c((ulong)*param_3,'%'), ((uint)puVar8 | (uint)pcVar6) != 0x0) {
    pass1_1010_09b4(uVar9,uVar10,(uchar *)CONCAT22(puVar8,pcVar6),param_3,uVar2,puVar8,param_5);
  }
  if (0x1e < iVar1 - 0x1U) goto LAB_1010_0850;
  uVar11 = (undefined2)((ulong)param_2 >> 0x10);
  switch(iVar1) {
  case 0x1:
    goto LAB_1010_0619;
  case 0x2:
    goto LAB_1010_0619;
  case 0x3:
    break;
  case 0x4:
    goto LAB_1010_0619;
  case 0x5:
    goto LAB_1010_0619;
  case 0x6:
    break;
  case 0x7:
    goto LAB_1010_0619;
  case 0x8:
    goto LAB_1010_0619;
  case 0x9:
    break;
  case 0xa:
    goto LAB_1010_0619;
  case 0xb:
    goto LAB_1010_0619;
  case 0xc:
    break;
  case 0xd:
    goto LAB_1010_0619;
  case 0xe:
    break;
  case 0xf:
    goto LAB_1010_0619;
  case 0x10:
    break;
  case 0x11:
    break;
  case 0x12:
    break;
  case 0x13:
    break;
  case 0x14:
    break;
  case 0x15:
    break;
  case 0x16:
LAB_1010_0619:
    puVar7 = pass1_1008_5fd8(param_5,puVar8);
    goto LAB_1010_0621;
  case 0x17:
    break;
  case 0x18:
    break;
  case 0x19:
  case 0x1f:
LAB_1010_0785:
    puVar7 = pass1_1008_5fd8(param_5,puVar8);
    goto LAB_1010_0621;
  case 0x1a:
    goto LAB_1010_0785;
  case 0x1b:
    goto LAB_1010_0785;
  case 0x1c:
    break;
  case 0x1d:
    break;
  case 0x1e:
    puVar7 = pass1_1008_5fd8(param_5,puVar8);
    *(uchar **)param_2 = puVar7;
    *(uchar **)((int)param_2 + 0x2) = puVar8;
    goto LAB_1010_0785;
  }
  puVar7 = pass1_1008_5fd8(param_5,puVar8);
LAB_1010_0621:
  *(uchar **)param_2 = puVar7;
  *(uchar **)((int)param_2 + 0x2) = puVar8;
LAB_1010_0850:
  while (pcVar6 = pass1_1000_472c((ulong)*param_2,'%'), ((uint)puVar8 | (uint)pcVar6) != 0x0) {
    pass1_1010_09b4(uVar9,uVar10,(uchar *)CONCAT22(puVar8,pcVar6),param_2,uVar2,puVar8,param_5);
  }
  return;
}



ushort __stdcall16far pass1_1010_0886(void)

{
  return 0xa;
}



ushort __stdcall16far pass1_1010_088c(void)

{
  return 0x3;
}



ushort __stdcall16far pass1_1010_0892(void)

{
  return 0x3;
}



ushort __stdcall16far pass1_1010_0898(void)

{
  return 0x3;
}



void __stdcall16far pass1_1010_089e(ushort param_1,ulong param_2,ushort param_3,int param_4)

{
  *(ushort *)((param_4 + -0x1) * 0x8 + 0xe28) = param_3;
  pass1_1010_1f62(param_1,param_2,0x3);
  return;
}



void __stdcall16far pass1_1010_08c0(ulong param_1,ushort param_2,int param_3,ushort param_4)

{
  *(ushort *)((param_3 + -0x1) * 0x8 + 0xea8) = param_2;
  pass1_1010_1f62(param_4,param_1,0x3);
  return;
}



ulong __stdcall16far pass1_1010_08e2(ushort param_1,ushort param_2,int param_3)

{
  if (PTR_LOOP_1050_4fe8 != (undefined *)0x0) {
    DAT_1050_0e28 = 0x0;
    PTR_LOOP_1050_0e30 = (undefined *)0x0;
    PTR_LOOP_1050_0e38 = (undefined *)0x0;
    PTR_LOOP_1050_0e40 = (undefined *)0x0;
    PTR_LOOP_1050_0e48 = (undefined *)0x0;
    DAT_1050_0e58 = 0x0;
    DAT_1050_0e60 = 0x0;
    PTR_LOOP_1050_0e70 = (undefined *)0x0;
  }
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe22);
}



ulong __stdcall16far pass1_1010_091e(ushort param_1,ushort param_2,int param_3)

{
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe72);
}



ulong __stdcall16far pass1_1010_0932(ushort param_1,ushort param_2,int param_3)

{
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe8a);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far
pass1_1010_0946(undefined2 param_1,undefined2 param_2,int param_3,uchar *param_4,int param_5,ushort param_6)

{
  ushort *puVar1;
  long lVar2;
  long lVar3;
  
  PTR_LOOP_1050_0ea8 = (undefined *)0x0;
  lVar3 = 0x4000001;
  lVar2 = 0x4000002;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_6,param_4,param_5);
  pass1_1008_dfa6((ulong)puVar1,lVar2,lVar3,param_6);
  if ((int)puVar1 != 0x0) {
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x4000002);
    if (*(long *)((int)puVar1 + 0x200) == 0x8000002) {
      PTR_LOOP_1050_0ea8 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
    }
  }
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xea2);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1010_09b4(ushort param_1,ushort param_2,uchar *param_3,char **param_4,ulong param_5,uchar *param_6,ushort param_7)

{
  byte bVar1;
  bool bVar2;
  bool bVar3;
  undefined2 uVar4;
  uint uVar5;
  uint uVar6;
  ushort uVar7;
  int unaff_DI;
  ushort *puVar8;
  char *pcStack18;
  undefined4 uStack10;
  
  bVar3 = false;
  bVar2 = false;
  bVar1 = *(byte *)((int)param_3 + 0x1);
  if (bVar1 == 0x70) {
LAB_1010_0a06:
    bVar3 = false;
    bVar2 = true;
  }
  else {
    if (bVar1 < 0x71) {
      if (bVar1 != 0x43) {
        if (bVar1 == 0x50) goto LAB_1010_0a06;
        if (bVar1 != 0x63) goto LAB_1010_09db;
      }
      bVar3 = true;
      bVar2 = false;
    }
  }
LAB_1010_09db:
  uVar4 = 0x0;
  uStack10 = (char *)0x0;
  if (bVar2) {
    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_7,param_6,unaff_DI);
    uVar4 = (undefined2)((ulong)puVar8 >> 0x10);
    uStack10 = (char *)CONCAT22(*(undefined2 *)((int)puVar8 + 0x6e),*(undefined2 *)((int)puVar8 + 0x6c));
  }
  else {
    if (!bVar3) goto LAB_1010_0a36;
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),param_5);
    uStack10 = pass1_1038_4d28(CONCAT22(param_6,uVar4));
  }
  param_6 = (uchar *)((ulong)uStack10 >> 0x10);
LAB_1010_0a36:
  if ((uStack10._2_2_ | (uint)uStack10) != 0x0) {
    uVar5 = str_op_1000_3da4(*param_4);
    uVar6 = str_op_1000_3da4(uStack10);
    uVar7 = uVar6 + 0xa + uVar5;
    mem_op_1000_179c(uVar7,param_6,0x1000);
    pcStack18 = (char *)CONCAT22(param_6,uVar7);
    *param_3 = '\0';
    unk_str_op_1000_3d3e((char *)CONCAT22(param_6,uVar7),*param_4);
    pass1_1000_3cea(CONCAT22(param_6,uVar7),(ULONG)uStack10);
    pass1_1000_3cea(CONCAT22(param_6,uVar7),(ulong)param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x2));
    fn_ptr_1000_17ce((astruct_18 *)*param_4,0x1000);
    *param_4 = pcStack18;
  }
  return;
}



void __stdcall16far pass1_1010_0ad2(ulong param_1,ulong param_2,uint16_t param_3)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  undefined *puVar3;
  uint extraout_DX;
  int iVar4;
  undefined2 uVar5;
  ushort uVar6;
  ushort uVar7;
  undefined4 local_2a [0x2];
  undefined2 local_22 [0x2];
  undefined2 local_1e [0x3];
  undefined2 local_18 [0x3];
  undefined4 uStack18;
  undefined local_e [0x8];
  undefined2 uStack6;
  int iStack4;
  
  BVar2 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar2 == 0x0) {
    return;
  }
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0xa) == 0x0) {
    uStack6 = 0x0;
  }
  else {
    uVar1 = *(undefined4 *)(iVar4 + 0xa);
    uStack6 = *(undefined2 *)((int)uVar1 + 0x8);
  }
  local_1e[0] = uStack6;
  uVar6 = (ushort)param_2;
  uVar7 = (ushort)(param_2 >> 0x10);
  BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_1e,param_3,(char *)0x2,0x1008);
  if (BVar2 != 0x0) {
    pass1_1008_5784((ulong *)CONCAT22(param_3,local_e),*(ulong *)(iVar4 + 0xa));
    do {
      puVar3 = local_e;
      pass1_1008_5b12(puVar3,param_3);
      uStack18 = CONCAT22(extraout_DX,puVar3);
      if ((extraout_DX | (uint)puVar3) == 0x0) {
        local_22[0] = *(undefined2 *)(iVar4 + 0xe);
        BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_22,param_3,(char *)0x2,0x1008);
        if (BVar2 == 0x0) {
          PTR_LOOP_1050_0310 = (undefined *)0x6d0;
          return;
        }
        local_22[0] = *(undefined2 *)(iVar4 + 0x10);
        BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_22,param_3,(char *)0x2,0x1008);
        if (BVar2 == 0x0) {
          PTR_LOOP_1050_0310 = (undefined *)0x6d0;
          return;
        }
        if (*(int *)(iVar4 + 0x18) != 0x0) {
          DAT_1050_0e28 = *(undefined2 *)(iVar4 + 0x12);
          PTR_LOOP_1050_0e30 = (undefined *)*(undefined2 *)(iVar4 + 0x14);
          PTR_LOOP_1050_0ea8 = (undefined *)*(undefined2 *)(iVar4 + 0x16);
        }
        iStack4 = 0x0;
        while( true ) {
          if (0x9 < iStack4) {
            iStack4 = 0x0;
            while( true ) {
              if (0x2 < iStack4) {
                if (*(int *)(iVar4 + 0x18) != 0x0) {
                  DAT_1050_0e28 = 0x0;
                  PTR_LOOP_1050_0e30 = (undefined *)0x0;
                  PTR_LOOP_1050_0ea8 = (undefined *)0x0;
                }
                return;
              }
              local_1e[0] = *(undefined2 *)(iStack4 * 0x8 + 0xea8);
              BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_1e,param_3,(char *)0x2,0x1008);
              if (BVar2 == 0x0) break;
              iStack4 = iStack4 + 0x1;
            }
            PTR_LOOP_1050_0310 = (undefined *)0x6d0;
            return;
          }
          local_1e[0] = *(undefined2 *)(iStack4 * 0x8 + 0xe28);
          BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_1e,param_3,(char *)0x2,0x1008);
          if (BVar2 == 0x0) break;
          iStack4 = iStack4 + 0x1;
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
      }
      local_18[0] = *(undefined2 *)(puVar3 + 0x4);
      BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_18,param_3,(char *)0x2,0x1008);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = (undefined *)0x6d0;
        return;
      }
      local_2a[0] = *(undefined4 *)((int)uStack18 + 0x6);
      BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,(ushort)local_2a,param_3,(char *)0x4,0x1008);
    } while (BVar2 != 0x0);
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  return;
}



void __stdcall16far file_1010_0c7c(ulong param_1,ulong param_2,int param_3,uchar *param_4,uint16_t param_5)

{
  undefined4 *puVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  astruct_229 *uVar4;
  ushort uVar5;
  uchar *extraout_DX;
  astruct_228 *iVar6;
  undefined2 uVar6;
  ushort uVar7;
  ushort uVar8;
  undefined2 local_2a [0x2];
  ushort uStack38;
  undefined4 *puStack26;
  undefined4 *puStack22;
  undefined2 local_12 [0x5];
  astruct_229 *paStack8;
  astruct_229 *local_6;
  ushort uStack4;
  
  uVar7 = (ushort)param_2;
  uVar8 = (ushort)(param_2 >> 0x10);
  read_file_1008_7cfe(uVar7,uVar8,0x6,0x1008,param_5);
  if (param_3 == 0x0) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d4;
  }
  else {
    BVar3 = read_file_1008_7dee(uVar7,uVar8,(ushort)&local_6,0x0,param_5,0x2,0x1008);
    if (BVar3 != 0x0) {
      paStack8 = (astruct_229 *)0x0;
      while( true ) {
        iVar6 = (astruct_228 *)param_1;
        uVar5 = (ushort)(param_1 >> 0x10);
        if (local_6 <= paStack8) break;
        uVar4 = local_6;
        mem_op_1000_179c(0xa,param_4,0x1000);
        puStack26 = (undefined4 *)CONCAT22(param_4,uVar4);
        if (((uint)param_4 | (uint)uVar4) == 0x0) {
          puStack22 = (undefined4 *)0x0;
        }
        else {
          *(undefined2 *)puStack26 = 0x389a;
          uVar4->field_0x2 = 0x1008;
          *(undefined2 *)puStack26 = 0xea8;
          uVar4->field_0x2 = 0x1010;
          puStack22 = puStack26;
        }
        BVar3 = read_file_1008_7dee(uVar7,uVar8,(ushort)local_12,0x0,param_5,0x2,0x1008);
        if ((BVar3 == 0x0) ||
           (BVar3 = read_file_1008_7dee(uVar7,uVar8,(int)puStack22 + 0x6,0x0,(ushort)((ulong)puStack22 >> 0x10),0x4,
                                        0x1008), BVar3 == 0x0)) {
          puStack26 = puStack22;
          if (puStack22 != (undefined4 *)0x0) {
            ppcVar2 = (code **)*puStack22;
            (**ppcVar2)(0x1008,(int)puStack22,(int)((ulong)puStack22 >> 0x10),0x1);
          }
          goto LAB_1010_0cb1;
        }
        uVar6 = (undefined2)((ulong)puStack22 >> 0x10);
        *(undefined2 *)((int)puStack22 + 0x4) = local_12[0];
        puVar1 = iVar6->field_0xa;
        ppcVar2 = (code **)((int)*iVar6->field_0xa + 0x8);
        (**ppcVar2)(0x8,(int)puVar1,(int)((ulong)puVar1 >> 0x10),(int)puStack22,uVar6);
        paStack8 = (astruct_229 *)&paStack8->field_0x1;
        param_4 = extraout_DX;
      }
      BVar3 = read_file_1008_7dee(uVar7,uVar8,&iVar6->field_0xe,0x0,uVar5,0x2,0x1008);
      if ((BVar3 != 0x0) &&
         (BVar3 = read_file_1008_7dee(uVar7,uVar8,&iVar6->field_0x10,0x0,uVar5,0x2,0x1008), BVar3 != 0x0)) {
        for (uStack4 = 0x0; (int)uStack4 < 0xa; uStack4 = uStack4 + 0x1) {
          BVar3 = read_file_1008_7dee(uVar7,uVar8,(ushort)local_2a,0x0,param_5,0x2,0x1008);
          if (BVar3 == 0x0) goto LAB_1010_0cb1;
          uVar5 = uStack4;
          if ((int)PTR_LOOP_1050_0312 < 0x2) {
            uVar5 = pass1_1008_738c(uVar7,uVar8,uStack4);
          }
          *(undefined2 *)(uVar5 * 0x8 + 0xe28) = local_2a[0];
          uStack38 = uVar5;
        }
        if (0x2 < (int)PTR_LOOP_1050_0312) {
          uStack4 = 0x0;
          do {
            BVar3 = read_file_1008_7dee(uVar7,uVar8,(ushort)local_2a,0x0,param_5,0x2,0x1008);
            if (BVar3 == 0x0) goto LAB_1010_0cb1;
            *(undefined2 *)(uStack4 * 0x8 + 0xea8) = local_2a[0];
            uStack4 = uStack4 + 0x1;
          } while ((int)uStack4 < 0x3);
        }
        return;
      }
    }
LAB_1010_0cb1:
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



ushort * __stdcall16far pass1_1010_0e46(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_0350(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1010_0e6c(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1010_0eac(uchar *param_1,uchar *param_2,ushort param_3,uchar *param_4,ushort param_5)

{
  struct_op_1018_4cda((int)param_1,(ushort)param_2,param_3);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xf0c;
  *(undefined2 *)(param_1 + 0x2) = 0x1010;
  PTR_LOOP_1050_4230 = param_1;
  PTR_LOOP_1050_4232 = param_2;
  pass1_1018_4dce((ulong *)CONCAT22(param_2,param_1),0xff,param_4,param_5);
  return CONCAT22(param_2,param_1);
}



astruct_11 * __stdcall16far pass1_1010_0ee6(astruct_11 *param_1,byte param_2)

{
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1010_0f24(astruct_79 *param_1,astruct_79 *param_2,ushort param_3,uchar *param_4,ushort param_5)

{
  int unaff_DI;
  ushort *puVar1;
  
  struct_1010_2cd2(param_1,param_2,param_3,param_5);
  *(undefined4 *)((int)&param_1[0x9].field_0x4 + 0x2) = 0x0;
  *(undefined4 *)(param_1 + 0xa) = 0x0;
  *(undefined2 *)&param_1[0xa].field_0x4 = 0x0;
  *(undefined4 *)((int)&param_1[0xa].field_0x4 + 0x2) = 0x0;
  *(int *)CONCAT22(param_2,param_1) = (int)s_648_bmp_1050_1919 + 0x1;
  param_1->field_0x2 = 0x1010;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_5,param_4,unaff_DI);
  *(undefined2 *)((int)&param_1[0xa].field_0x4 + 0x2) = (int)puVar1;
  param_1[0xa].field_0x8 = (ushort)((ulong)puVar1 >> 0x10);
  return;
}



void __stdcall16far pass1_1010_0f76(ushort *param_1,ushort param_2)

{
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  *param_1 = (int)s_648_bmp_1050_1919 + 0x1;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
  pass1_1010_17c0((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
  pass1_1010_2db2(param_1,param_2);
  return;
}



void __stdcall16far struct_1010_0f9c(ulong *param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  ushort uVar2;
  undefined *puVar3;
  undefined *puVar4;
  undefined4 uVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  uchar *puVar7;
  uchar *extraout_DX_00;
  undefined2 extraout_DX_01;
  undefined2 extraout_DX_02;
  undefined2 extraout_DX_03;
  uchar *extraout_DX_04;
  astruct_232 *iVar8;
  astruct_231 *iVar9;
  astruct_230 *iVar10;
  astruct_233 *iVar11;
  undefined2 uVar8;
  undefined2 uVar9;
  ulong *puVar10;
  undefined4 uVar11;
  ulong *puVar12;
  undefined uVar13;
  undefined4 uStack36;
  int iStack32;
  ushort uStack30;
  uint *puStack28;
  undefined4 uStack24;
  undefined local_14 [0x12];
  
  uVar8 = (undefined2)((ulong)param_1 >> 0x10);
  iVar8 = (astruct_232 *)param_1;
  ppcVar1 = (code **)((int)*param_1 + 0x38);
  (**ppcVar1)();
  iVar8->field_0x68 = param_2;
  if ((*(long *)&iVar8->field_0x60 != 0x0) && (iVar8->field_0x68 == 0x1)) {
    return;
  }
  if (iVar8->field_0x68 == 0x0) {
    return;
  }
  puVar7 = extraout_DX;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_3,local_14),0x1,0x0,0x700);
  uVar2 = iVar8->field_0x68 * 0x18;
  mem_op_1000_179c(uVar2,puVar7,0x1000);
  iVar8->field_0x60 = uVar2;
  iVar8->field_0x62 = puVar7;
  puStack28 = (uint *)CONCAT22(puVar7,iVar8->field_0x60);
  uStack30 = iVar8->field_0x68;
  do {
    do {
      puVar6 = puVar7;
      puVar3 = local_14;
      pass1_1028_e4ec(CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,puVar3)));
      uStack24 = CONCAT22(puVar6,puVar3);
      puVar7 = (uchar *)((uint)puVar6 | (uint)puVar3);
      if (puVar7 == (uchar *)0x0) goto LAB_1010_10ca;
      iVar9 = (astruct_231 *)*param_1;
      ppcVar1 = (code **)&iVar9->field_0x40;
      puVar4 = puVar3;
      (**ppcVar1)();
      puVar7 = extraout_DX_00;
    } while (puVar4 == (undefined *)0x0);
    uVar13 = SUB21(puVar6,0x0);
    pass1_1028_b58e(CONCAT13((char)((uint)puVar6 >> 0x8),CONCAT12(uVar13,puVar3)));
    uStack36 = CONCAT22(extraout_DX_01,puVar4);
    ppcVar1 = (code **)&iVar9->field_0x2c;
    puVar12 = param_1;
    (**ppcVar1)();
    uVar9 = (undefined2)((ulong)puStack28 >> 0x10);
    iVar10 = (astruct_230 *)puStack28;
    *puStack28 = (uint)puVar4;
    iVar10->field_0x2 = extraout_DX_02;
    ppcVar1 = (code **)&iVar9->field_0x30;
    puVar10 = param_1;
    uVar11 = uStack24;
    (**ppcVar1)();
    iVar10->field_0x8 = puVar4;
    iVar10->field_0xa = extraout_DX_03;
    iVar10->field_0xc = uStack36;
    ppcVar1 = (code **)&iVar9->field_0x3c;
    uVar5 = uStack36;
    (**ppcVar1)((int)&USHORT_1050_1028,param_1,uStack24,puVar10,uVar11,puVar12,puVar3,uVar13);
    iVar10->field_0x10 = (int)uVar5;
    iVar10->field_0x12 = extraout_DX_04;
    iVar10->field_0x14 = uStack36;
    puStack28 = (uint *)((ulong)puStack28 & 0xffff0000 | ZEXT24(iVar10 + 0x1));
    uStack30 = uStack30 - 0x1;
    puVar7 = extraout_DX_04;
  } while (uStack30 != 0x0);
LAB_1010_10ca:
  uVar2 = iVar8->field_0x68 << 0x2;
  mem_op_1000_179c(uVar2,puVar7,0x1000);
  iVar8->field_0x64 = uVar2;
  iVar8->field_0x66 = puVar7;
  iStack32 = 0x0;
  uStack30 = 0x0;
  while( true ) {
    if ((int)(iVar8->field_0x68 * 0x3) <= (int)uStack30) break;
    puVar7 = iVar8->field_0x62;
    uVar5 = *(undefined4 *)&iVar8->field_0x64;
    uVar9 = (undefined2)((ulong)uVar5 >> 0x10);
    iVar11 = (astruct_233 *)uVar5;
    *(ushort *)(iVar11 + iStack32 * 0x4) = iVar8->field_0x60 + uStack30 * 0x8;
    *(uchar **)(iVar11 + iStack32 * 0x4 + 0x2) = puVar7;
    uStack30 = uStack30 + 0x3;
    iStack32 = iStack32 + 0x1;
  }
  return;
}



void __stdcall16far pass1_1010_1146(ulong param_1,ushort param_2,int param_3,ushort param_4)

{
  undefined4 uVar1;
  ushort uVar2;
  
  DAT_1050_0ecc = param_2;
  uVar2 = (ushort)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x66);
  pass1_1000_4aea(*(uint *)((int)param_1 + 0x64),(uint)uVar1,(int)((ulong)uVar1 >> 0x10),0x4,
                  (uchar *)((int)s_dibtext_bmp_1050_1844 + 0x6),(int)&stack0xfffe,param_3,uVar2,0x1000,param_4);
  return;
}



void __stdcall16far pass1_1010_116c(ulong *param_1,int param_2,ushort param_3)

{
  code **ppcVar1;
  int iVar2;
  undefined2 uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined4 uVar6;
  ushort uStack4;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x56) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x34);
    (**ppcVar1)();
  }
  ppcVar1 = (code **)((int)*param_1 + 0x28);
  uVar6 = (**ppcVar1)();
  uVar3 = (undefined2)((ulong)uVar6 >> 0x10);
  if ((int)uVar6 != 0x0) {
    uStack4 = DAT_1050_0ecc;
    iVar2 = DAT_1050_0ecc + 0x1;
    if (iVar2 == 0x0) {
      uStack4 = 0x0;
    }
    pass1_1010_1146((ulong)param_1,uStack4,param_2,param_3);
    pass1_1010_11c6((ulong)param_1,iVar2,uVar3);
    *(int *)(iVar4 + 0x56) = iVar2;
    *(undefined2 *)(iVar4 + 0x58) = uVar3;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1010_11c6(ulong param_1,uint param_2,uchar *param_3)

{
  int *piVar1;
  uint *puVar2;
  code **ppcVar3;
  ulong uVar4;
  undefined4 uVar5;
  astruct_239 *iVar6;
  int iVar7;
  int iVar8;
  uint uVar9;
  uint uVar10;
  uchar *puVar11;
  uchar *puVar12;
  uchar *puVar13;
  uchar *puVar14;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  undefined2 uVar15;
  uchar *extraout_DX_01;
  uchar *puVar16;
  ulong *puVar17;
  astruct_234 *iVar18;
  int iVar19;
  int iVar21;
  astruct_238 *iVar20;
  undefined2 uVar22;
  undefined2 uVar23;
  ushort *puVar24;
  ulong *puStack50;
  int iStack42;
  int iStack40;
  astruct_20 *paStack38;
  int iStack28;
  ulong *puStack26;
  ulong *puStack22;
  undefined4 uStack14;
  ulong uStack10;
  
  if (DAT_1050_0ecc == -0x1) {
    return;
  }
  mem_op_1000_179c(0x1a,param_3,0x1000);
  if (((uint)param_3 | param_2) == 0x0) {
    iVar6 = (astruct_239 *)0x0;
    puVar11 = (uchar *)0x0;
  }
  else {
    puVar24 = pass1_1010_37d4((ushort *)CONCAT22(param_3,param_2));
    puVar11 = (uchar *)((ulong)puVar24 >> 0x10);
    iVar6 = (astruct_239 *)puVar24;
  }
  uStack10 = 0x10500ece;
  uStack14 = 0x0;
  puVar12 = puVar11;
  while( true ) {
    uVar22 = (undefined2)(param_1 >> 0x10);
    iVar18 = (astruct_234 *)param_1;
    piVar1 = &iVar18->field_0x68;
    if (*piVar1 == (int)uStack14 || *piVar1 < (int)uStack14) break;
    uVar5 = iVar18->field_0x64;
    uVar4 = *(ulong *)((int)uVar5 + (int)uStack14 * 0x4);
    puVar17 = (ulong *)((int)uVar4 + DAT_1050_0ecc * 0x8);
    puStack50 = (ulong *)(uVar4 & 0xffff0000 | ZEXT24(puVar17));
    iVar7 = pass1_1000_475e(uStack10,*puVar17);
    if (iVar7 != 0x0) {
      uStack10 = *puStack50;
      uStack14 = uStack14 & 0xffff | (ulong)(uStack14._2_2_ + 0x1) << 0x10;
    }
    uStack14 = uStack14 & 0xffff0000 | (ulong)((int)uStack14 + 0x1);
  }
  iVar6->field_0x10 = uStack14._2_2_;
  puVar24 = struct_1010_38f8(CONCAT22(puVar11,iVar6),uStack14._2_2_,uStack14._2_2_,puVar12);
  puVar13 = (uchar *)((ulong)puVar24 >> 0x10);
  iVar8 = 0x0;
  mem_op_1000_179c(0x400,puVar13,0x1000);
  puVar12 = puVar13;
  iVar7 = iVar8;
  mem_op_1000_179c(0x400,puVar13,0x1000);
  paStack38 = (astruct_20 *)CONCAT22(puVar12,iVar7);
  iStack28 = 0x0;
  pass1_1000_4906((astruct_20 *)CONCAT22(puVar13,iVar8),(WNDCLASS16 *)0x0,0x400);
  pass1_1000_4906((astruct_20 *)CONCAT22(puVar12,iVar7),(WNDCLASS16 *)0x0,0x400);
  iStack42 = 0x0;
  uVar10 = 0x0;
  do {
    puVar2 = &iVar6->field_0x10;
    if (*puVar2 == uVar10 || (int)*puVar2 < (int)uVar10) {
      return;
    }
    uVar5 = iVar18->field_0x64;
    uVar23 = (undefined2)((ulong)uVar5 >> 0x10);
    iVar19 = (int)uVar5;
    iVar21 = *(int *)(iVar19 + iStack28 * 0x4);
    puVar16 = *(uchar **)(iVar19 + iStack28 * 0x4 + 0x2);
    iVar19 = iVar21 + *(int *)(DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
    puStack22 = (ulong *)CONCAT22(puVar16,iVar19);
    uVar9 = iVar21 + *(int *)(DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
    puVar14 = puVar16;
    mem_op_1000_179c(0x1a,puVar16,0x1000);
    if (((uint)puVar14 | uVar9) == 0x0) {
      uVar5 = iVar6->field_0x8;
      *(undefined4 *)((int)uVar5 + uVar10 * 0x4) = 0x0;
    }
    else {
      puVar24 = pass1_1010_37d4((ushort *)CONCAT22(puVar14,uVar9));
      uVar5 = iVar6->field_0x8;
      uVar23 = (undefined2)((ulong)uVar5 >> 0x10);
      iVar21 = (int)uVar5;
      *(undefined2 *)(iVar21 + uVar10 * 0x4) = (int)puVar24;
      *(undefined2 *)(iVar21 + uVar10 * 0x4 + 0x2) = (int)((ulong)puVar24 >> 0x10);
    }
    iStack42 = iStack42 + 0x1;
    uVar5 = iVar6->field_0x8;
    uVar23 = (undefined2)((ulong)uVar5 >> 0x10);
    iVar21 = (int)uVar5;
    uVar5 = *(undefined4 *)(iVar21 + uVar10 * 0x4);
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar21 + uVar10 * 0x4) + 0x1c);
    (**ppcVar3)(0x1000,(int)uVar5,(int)((ulong)uVar5 >> 0x10),iStack42,iVar19,puVar16);
    uStack14 = (ulong)uVar10;
    puVar16 = extraout_DX;
    while( true ) {
      piVar1 = &iVar18->field_0x68;
      if (*piVar1 == iStack28 || *piVar1 < iStack28) break;
      iVar19 = iStack28 * 0x4;
      uVar5 = iVar18->field_0x64;
      uVar5 = *(undefined4 *)((int)uVar5 + iVar19);
      iVar21 = pass1_1000_475e(*puStack22,*(ulong *)((int)uVar5 + *(int *)(DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
      if (iVar21 != 0x0) break;
      uVar5 = iVar18->field_0x64;
      uVar23 = (undefined2)((ulong)uVar5 >> 0x10);
      iVar21 = (int)uVar5;
      puVar16 = *(uchar **)(iVar21 + iVar19 + 0x2);
      uVar10 = *(int *)(iVar21 + iVar19) + *(int *)(DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
      puStack26 = (ulong *)CONCAT22(puVar16,uVar10);
      mem_op_1000_179c(0x1a,puVar16,0x1000);
      if (((uint)puVar16 | uVar10) == 0x0) {
        uVar23 = 0x0;
        uVar15 = 0x0;
      }
      else {
        puVar24 = pass1_1010_37d4((ushort *)CONCAT22(puVar16,uVar10));
        uVar15 = (undefined2)((ulong)puVar24 >> 0x10);
        uVar23 = SUB42(puVar24,0x0);
      }
      *(undefined2 *)(uStack14._2_2_ * 0x4 + iVar8) = uVar23;
      *(undefined2 *)(uStack14._2_2_ * 0x4 + iVar8 + 0x2) = uVar15;
      uVar5 = iVar18->field_0x64;
      uVar23 = (undefined2)((ulong)uVar5 >> 0x10);
      iVar21 = (int)uVar5;
      iStack42 = iStack42 + 0x1;
      uVar5 = *(undefined4 *)(uStack14._2_2_ * 0x4 + iVar8);
      ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(uStack14._2_2_ * 0x4 + iVar8) + 0x1c);
      (**ppcVar3)(0x1000,(int)uVar5,(int)((ulong)uVar5 >> 0x10),iStack42,
                  *(int *)(iVar21 + iStack28 * 0x4) + *(int *)(DAT_1050_0ecc * 0x6 + 0xebc) * 0x8,
                  *(undefined2 *)(iVar21 + iStack28 * 0x4 + 0x2));
      iStack40 = 0x0;
      puVar16 = extraout_DX_00;
      while( true ) {
        piVar1 = &iVar18->field_0x68;
        if (*piVar1 == iStack28 || *piVar1 < iStack28) break;
        uVar5 = iVar18->field_0x64;
        uVar5 = *(undefined4 *)((int)uVar5 + iStack28 * 0x4);
        iVar21 = pass1_1000_475e(*puStack26,*(ulong *)((int)uVar5 + *(int *)(DAT_1050_0ecc * 0x6 + 0xebc) * 0x8));
        if (iVar21 != 0x0) break;
        uVar5 = iVar18->field_0x64;
        uVar5 = *(undefined4 *)((int)uVar5 + iStack28 * 0x4);
        uVar10 = pass1_1000_475e(*puStack22,*(ulong *)((int)uVar5 + *(int *)(DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
        if (uVar10 != 0x0) break;
        mem_op_1000_179c(0x1a,puVar16,0x1000);
        if (((uint)puVar16 | uVar10) == 0x0) {
          uVar23 = 0x0;
          uVar15 = 0x0;
        }
        else {
          puVar24 = pass1_1010_37d4((ushort *)CONCAT22(puVar16,uVar10));
          uVar15 = (undefined2)((ulong)puVar24 >> 0x10);
          uVar23 = SUB42(puVar24,0x0);
        }
        *(undefined2 *)(iStack40 * 0x4 + iVar7) = uVar23;
        *(undefined2 *)(iStack40 * 0x4 + iVar7 + 0x2) = uVar15;
        uVar5 = iVar18->field_0x64;
        uVar23 = (undefined2)((ulong)uVar5 >> 0x10);
        iVar20 = (astruct_238 *)uVar5;
        iStack42 = iStack42 + 0x1;
        uVar5 = *(undefined4 *)(iStack40 * 0x4 + iVar7);
        ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iStack40 * 0x4 + iVar7) + 0x1c);
        (**ppcVar3)(0x1000,(int)uVar5,(int)((ulong)uVar5 >> 0x10),iStack42,
                    *(int *)(iVar20 + iStack28 * 0x4) + *(int *)(DAT_1050_0ecc * 0x6 + 0xebe) * 0x8,
                    *(undefined2 *)(iVar20 + iStack28 * 0x4 + 0x2));
        iStack28 = iStack28 + 0x1;
        iStack40 = iStack40 + 0x1;
        puVar16 = extraout_DX_01;
      }
      uVar5 = *(undefined4 *)(uStack14._2_2_ * 0x4 + iVar8);
      *(int *)((int)uVar5 + 0x10) = iStack40;
      uVar10 = iStack40 << 0x2;
      iVar21 = iVar7;
      puVar14 = puVar12;
      puVar24 = struct_1010_38f8(*(ulong *)(uStack14._2_2_ * 0x4 + iVar8),iStack40,uVar10,puVar16);
      puVar16 = (uchar *)((ulong)puVar24 >> 0x10);
      pass1_1000_48a8((ulong)puVar24,CONCAT22(puVar14,iVar21),uVar10);
      pass1_1000_4906(paStack38,(WNDCLASS16 *)0x0,0x400);
      uStack14 = uStack14 & 0xffff | (ulong)(uStack14._2_2_ + 0x1) << 0x10;
    }
    uVar5 = iVar6->field_0x8;
    uVar5 = *(undefined4 *)((int)uVar5 + (int)uStack14 * 0x4);
    *(int *)((int)uVar5 + 0x10) = uStack14._2_2_;
    uVar10 = uStack14._2_2_ << 0x2;
    uVar5 = iVar6->field_0x8;
    iVar21 = iVar8;
    puVar14 = puVar13;
    puVar24 = struct_1010_38f8(*(ulong *)((int)uVar5 + (int)uStack14 * 0x4),uStack14._2_2_,uVar10,puVar16);
    pass1_1000_48a8((ulong)puVar24,CONCAT22(puVar14,iVar21),uVar10);
    pass1_1000_4906((astruct_20 *)CONCAT22(puVar13,iVar8),(WNDCLASS16 *)0x0,0x400);
    uVar10 = (int)uStack14 + 0x1;
  } while( true );
}
