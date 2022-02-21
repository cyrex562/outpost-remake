

ushort * __stdcall16far pass1_1008_941a(ushort *param_1,ushort param_2,ushort param_3)

{
  *param_1 = param_2;
  *(ushort *)((int)param_1 + 0x2) = param_3;
  return param_1;
}



ushort * __stdcall16far pass1_1008_9436(ushort *param_1)

{
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x2) = 0x0;
  return param_1;
}



void __stdcall16far pass1_1008_944e(ushort *param_1,ushort param_2,ushort param_3)

{
  *(ushort *)((int)param_1 + 0x2) = param_3;
  *param_1 = param_2;
  return;
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


ulong * __stdcall16far pass1_1008_80d2(ulong *param_1)

{
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x4) = 0x0;
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


void __stdcall16far pass1_1008_68ea(int param_1,ushort param_2,ulong *param_3,ushort param_4,ushort param_5,int param_6)

{
  code **ppcVar1;
  
  if (param_6 == 0x0) {
    if (*(long *)(param_1 + 0xce) != CONCAT22(param_4,param_3)) {
      if (*(long *)(param_1 + 0xce) != 0x0) {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0xce) + 0x10);
        (**ppcVar1)();
      }
      *(long *)(param_1 + 0xce) = CONCAT22(param_4,param_3);
      ppcVar1 = (code **)((int)*param_3 + 0x10);
      (**ppcVar1)();
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0xce) + 0xc);
      (**ppcVar1)();
      return;
    }
  }
  else {
    pass1_1008_3e0e(CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,param_1)));
  }
  return;
}


void __stdcall16far pass1_1008_6a04(ulong param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  undefined *puVar2;
  uint extraout_DX;
  undefined local_a [0x8];
  
  pass1_1008_57a4((ulong *)CONCAT22(param_3,local_a),param_1 & 0xffff0000 | (ulong)((int)param_1 + 0xd2));
  while( true ) {
    puVar2 = local_a;
    pass1_1008_5b12(puVar2,param_3);
    if ((extraout_DX | (uint)puVar2) == 0x0) break;
    ppcVar1 = (code **)((int)**(undefined4 **)(puVar2 + 0x4) + 0xc);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1008_6a4a(ulong param_1,int param_2,ushort param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  int iVar2;
  undefined *puVar3;
  uint extraout_DX;
  uint extraout_DX_00;
  undefined local_e [0x4];
  undefined4 uStack10;
  undefined4 uStack6;
  
  if (param_4 == 0x2) {
    iVar2 = (int)param_1;
    pass1_1008_57a4((ulong *)CONCAT22(param_5,local_e),param_1 & 0xffff0000 | (ulong)(iVar2 + 0xd2));
    do {
      puVar3 = local_e;
      pass1_1008_5b12(puVar3,param_5);
      uStack6 = CONCAT22(extraout_DX,puVar3);
      if ((extraout_DX | (uint)puVar3) == 0x0) break;
    } while (*(int *)(puVar3 + 0x8) != param_2);
    if (uStack6 != 0x0) {
      ppcVar1 = (code **)((int)*(undefined4 *)(iVar2 + 0xd2) + 0xc);
      (**ppcVar1)();
      uStack10 = 0x0;
      uStack6._0_2_ = local_e;
      pass1_1008_5b12();
      if ((extraout_DX_00 | (uint)(undefined *)uStack6) != 0x0) {
        ppcVar1 = (code **)((int)**(undefined4 **)((undefined *)uStack6 + 0x4) + 0x10);
        uStack6._2_2_ = extraout_DX_00;
        (**ppcVar1)();
        *(undefined4 *)(iVar2 + 0xce) = *(undefined4 *)((undefined *)uStack6 + 0x4);
        return;
      }
      *(undefined4 *)(iVar2 + 0xce) = 0x0;
    }
  }
  return;
}



void __stdcall16far pass1_1008_6b02(ulong param_1)

{
  code **ppcVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((*(uint *)(iVar2 + 0xd0) | *(uint *)(iVar2 + 0xce)) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar2 + 0xce) + 0x6c);
    (**ppcVar1)();
  }
  return;
}



void __stdcall16far pass1_1008_6b2e(ulong param_1)

{
  code **ppcVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((*(uint *)(iVar2 + 0xd0) | *(uint *)(iVar2 + 0xce)) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar2 + 0xce) + 0x68);
    (**ppcVar1)();
  }
  return;
}


void __stdcall16far pass1_1008_6c90(ushort *param_1)

{
  pass1_1008_3e38(param_1);
  pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x6)));
  return;
}



ulong * __stdcall16far pass1_1008_6cb4(ulong *param_1,ulong *param_2,ushort param_3,ulong *param_4,ushort param_5)

{
  astruct_362 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_362 *)param_1;
  *param_1 = *param_4;
  iVar1->field_0x4 = *(undefined2 *)(param_4 + 0x1);
  iVar1->field_0x6 = *param_2;
  iVar1->field_0xa = *(undefined2 *)(param_2 + 0x1);
  return param_1;
}



void __stdcall16far pass1_1008_6cec(ushort *param_1,ushort param_2,ulong param_3,ushort param_4,ulong param_5)

{
  pass1_1008_3e76(param_1,param_4,(ushort)param_5,(ushort)(param_5 >> 0x10));
  pass1_1008_3e76((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x6)),param_2,(ushort)param_3,
                  (ushort)(param_3 >> 0x10));
  return;
}



void __stdcall16far pass1_1008_6d18(ushort *param_1,ushort *param_2,ushort *param_3)

{
  pass1_1008_3f62(param_1,param_3);
  pass1_1008_3f62((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x6)),param_2);
  return;
}



void __stdcall16far pass1_1008_6d3e(ushort *param_1,ushort *param_2,ushort *param_3)

{
  pass1_1008_3f62(param_3,param_1);
  pass1_1008_3f62(param_2,(ushort *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x6)));
  return;
}



void __stdcall16far pass1_1008_6d64(ushort *param_1,ushort *param_2)

{
  pass1_1008_3f62(param_2,param_1);
  pass1_1008_3ee2((int *)param_2,(int *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x6)));
  return;
}


undefined2 * __stdcall16far pass1_1008_72a8(undefined2 *param_1,undefined2 param_2)

{
  *param_1 = param_2;
  return param_1;
}



ushort __stdcall16far switch_1008_72bc(ushort param_1,ushort param_2,ushort param_3)

{
  if ((int)PTR_LOOP_1050_0312 < 0x2) {
    switch(param_3) {
    case 0x1:
      param_3 = 0x1;
      break;
    case 0x2:
      param_3 = 0x2;
      break;
    case 0x3:
      param_3 = 0x3;
      break;
    case 0x4:
      param_3 = 0x5;
      break;
    case 0x5:
      param_3 = 0x4;
      break;
    case 0x6:
      param_3 = 0x6;
      break;
    case 0x7:
      param_3 = 0x7;
      break;
    case 0x8:
      param_3 = 0x8;
      break;
    case 0x9:
      param_3 = 0x9;
      break;
    case 0xa:
      param_3 = 0xa;
      break;
    case 0xb:
      param_3 = 0xb;
      break;
    case 0xc:
      param_3 = 0xc;
      break;
    case 0xd:
      param_3 = 0xd;
      break;
    case 0xe:
      param_3 = 0xe;
      break;
    case 0xf:
      param_3 = 0xf;
      break;
    case 0x10:
      return 0x10;
    case 0x11:
      return 0x11;
    case 0x12:
      return 0x12;
    case 0x13:
      return 0x13;
    default:
      return 0x0;
    }
  }
  return param_3;
}



ushort __stdcall16far pass1_1008_738c(ushort param_1,ushort param_2,ushort param_3)

{
  ushort uVar1;
  
  switch(param_3) {
  case 0x1:
    uVar1 = 0x3;
    break;
  case 0x2:
    uVar1 = 0x4;
    break;
  case 0x3:
    return 0x5;
  case 0x4:
    return 0x6;
  case 0x5:
    return 0x8;
  case 0x6:
    return 0x9;
  case 0x7:
    return 0xa;
  default:
    uVar1 = 0x0;
  }
  return uVar1;
}



int __stdcall16far switch_1008_73ea(ushort param_1,ushort param_2,int param_3)

{
  int iStack4;
  
  iStack4 = param_3;
  if ((int)PTR_LOOP_1050_0312 < 0x2) {
    switch(param_3) {
    case 0x18:
    case 0x19:
    case 0x1a:
    case 0x1b:
    case 0x1c:
    case 0x1d:
    case 0x1e:
    case 0x1f:
    case 0x20:
    case 0x21:
    case 0x22:
    case 0x23:
    case 0x24:
    case 0x25:
    case 0x26:
    case 0x27:
    case 0x28:
    case 0x29:
    case 0x2a:
    case 0x2b:
    case 0x2c:
    case 0x2d:
    case 0x2e:
    case 0x2f:
    case 0x30:
    case 0x31:
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
    case 0x38:
    case 0x39:
    case 0x3a:
    case 0x3b:
    case 0x3c:
      iStack4 = param_3 + 0x3;
      break;
    case 0x3d:
    case 0x3e:
      iStack4 = param_3 + 0x4;
      break;
    case 0x3f:
    case 0x40:
    case 0x41:
    case 0x42:
    case 0x43:
    case 0x44:
    case 0x45:
    case 0x46:
    case 0x47:
    case 0x48:
    case 0x49:
    case 0x4a:
    case 0x4b:
    case 0x4c:
    case 0x4d:
    case 0x4e:
    case 0x4f:
    case 0x50:
    case 0x51:
    case 0x52:
    case 0x53:
    case 0x54:
    case 0x55:
    case 0x56:
    case 0x57:
    case 0x58:
    case 0x59:
    case 0x5a:
    case 0x5b:
    case 0x5c:
    case 0x5d:
    case 0x5e:
    case 0x5f:
    case 0x60:
    case 0x61:
    case 0x62:
    case 0x63:
    case 0x64:
    case 0x65:
    case 0x66:
      iStack4 = param_3 + 0x8;
      break;
    case 0x67:
    case 0x68:
    case 0x69:
    case 0x6a:
    case 0x6b:
    case 0x6c:
    case 0x6d:
    case 0x6e:
    case 0x6f:
    case 0x70:
    case 0x71:
    case 0x72:
    case 0x73:
    case 0x74:
    case 0x75:
    case 0x76:
    case 0x77:
    case 0x78:
    case 0x79:
    case 0x7a:
    case 0x7b:
    case 0x7c:
    case 0x7d:
    case 0x7e:
    case 0x7f:
    case 0x80:
      iStack4 = param_3 + 0x9;
    }
  }
  return iStack4;
}

long __stdcall16far pass1_1008_57f0(ulong param_1,int param_2,ushort param_3)

{
  bool bVar1;
  long lVar2;
  int iStack12;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),param_1);
  iStack12 = 0x0;
  do {
    lVar2 = pass1_1008_5b12(local_a,param_3);
    if (lVar2 == 0x0) {
      return 0x0;
    }
    bVar1 = iStack12 != param_2;
    iStack12 = iStack12 + 0x1;
  } while (bVar1);
  return lVar2;
}



void __stdcall16far pass1_1008_5830(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined4 uVar4;
  undefined4 *puVar5;
  int iVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  
  while( true ) {
    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    if (*(long *)(iVar6 + 0x4) == 0x0) break;
    if (*(int *)(iVar6 + 0xa) != 0x0) {
      uVar4 = *(undefined4 *)(iVar6 + 0x4);
      uVar9 = (undefined2)((ulong)uVar4 >> 0x10);
      iVar7 = (int)uVar4;
      puVar1 = (undefined4 *)*(uint *)(iVar7 + 0x8);
      uVar2 = *(uint *)(iVar7 + 0xa);
      if ((uVar2 | (uint)puVar1) != 0x0) {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
      }
    }
    puVar5 = (undefined4 *)*(long *)(iVar6 + 0x4);
    *(undefined4 *)(iVar6 + 0x4) = *(undefined4 *)((int)puVar5 + 0x4);
    if (puVar5 != (undefined4 *)0x0) {
      ppcVar3 = (code **)*puVar5;
      (**ppcVar3)();
    }
  }
  *(undefined2 *)(iVar6 + 0x8) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_58a6(ulong param_1,ulong param_2)

{
  int *piVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  astruct_99 *paStack6;
  
  paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_029c);
  uVar3 = (uint)((ulong)paStack6 >> 0x10);
  uVar2 = (uint)paStack6;
  if ((uVar3 | uVar2) == 0x0) {
    paStack6 = (astruct_99 *)0x0;
  }
  else {
    paStack6->field_0x0 = 0x389a;
    *(undefined2 *)(uVar2 + 0x2) = 0x1008;
    *(undefined4 *)(uVar2 + 0x4) = 0x0;
    *(undefined4 *)(uVar2 + 0x8) = 0x0;
    paStack6->field_0x0 = 0x5bc0;
    *(undefined2 *)(uVar2 + 0x2) = 0x1008;
  }
  if (paStack6 == (astruct_99 *)0x0) {
    return;
  }
  uVar5 = (undefined2)((ulong)paStack6 >> 0x10);
  *(ulong *)((int)paStack6 + 0x8) = param_2;
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  *(undefined4 *)((int)paStack6 + 0x4) = *(undefined4 *)(iVar4 + 0x4);
  *(astruct_99 **)(iVar4 + 0x4) = paStack6;
  piVar1 = (int *)(iVar4 + 0x8);
  *piVar1 = *piVar1 + 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_593c(ulong *param_1,ulong param_2)

{
  int *piVar1;
  code **ppcVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  astruct_99 *paStack6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(int *)(iVar5 + 0x8) == 0x0) {
    ppcVar2 = (code **)((int)*param_1 + 0x4);
    (**ppcVar2)();
    return;
  }
  paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_029c);
  uVar4 = (uint)((ulong)paStack6 >> 0x10);
  uVar3 = (uint)paStack6;
  if ((uVar4 | uVar3) == 0x0) {
    paStack6 = (astruct_99 *)0x0;
  }
  else {
    paStack6->field_0x0 = 0x389a;
    *(undefined2 *)(uVar3 + 0x2) = 0x1008;
    *(undefined4 *)(uVar3 + 0x4) = 0x0;
    *(undefined4 *)(uVar3 + 0x8) = 0x0;
    paStack6->field_0x0 = 0x5bc0;
    *(undefined2 *)(uVar3 + 0x2) = 0x1008;
  }
  if (paStack6 == (astruct_99 *)0x0) {
    return;
  }
  *(ulong *)((int)paStack6 + 0x8) = param_2;
  do {
    param_1 = *(ulong **)((int)param_1 + 0x4);
    uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  } while (*(long *)((int)param_1 + 0x4) != 0x0);
  *(astruct_99 **)((int)param_1 + 0x4) = paStack6;
  piVar1 = (int *)(iVar5 + 0x8);
  *piVar1 = *piVar1 + 0x1;
  return;
}



void __stdcall16far pass1_1008_59f4(ulong param_1,long param_2)

{
  int *piVar1;
  undefined4 *puVar2;
  uint uVar3;
  undefined4 *puVar4;
  code **ppcVar5;
  undefined4 *puVar6;
  undefined2 uVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  undefined2 uVar12;
  uint uStack10;
  undefined4 *puStack6;
  
  puStack6 = (undefined4 *)0x0;
  uVar9 = (undefined2)(param_1 >> 0x10);
  puVar6 = puStack6;
  puVar4 = (undefined4 *)param_1;
  do {
    puStack6 = puVar6;
    uVar10 = (undefined2)((ulong)puVar4 >> 0x10);
    iVar8 = (int)puVar4;
    puVar4 = (undefined4 *)*(long *)(iVar8 + 0x4);
    uStack10 = (uint)puVar4;
    uVar11 = (undefined2)((ulong)puVar4 >> 0x10);
    if ((*(uint *)(iVar8 + 0x6) | uStack10) == 0x0) break;
    puVar6 = puVar4;
  } while (*(long *)(uStack10 + 0x8) != param_2);
  if (puVar4 != (undefined4 *)0x0) {
    if (puStack6 == (undefined4 *)0x0) {
      uVar10 = *(undefined2 *)(uStack10 + 0x4);
      uVar7 = *(undefined2 *)(uStack10 + 0x6);
      puStack6 = (undefined4 *)param_1;
    }
    else {
      uVar10 = *(undefined2 *)(uStack10 + 0x4);
      uVar7 = *(undefined2 *)(uStack10 + 0x6);
    }
    uVar12 = (undefined2)((ulong)puStack6 >> 0x10);
    *(undefined2 *)((int)puStack6 + 0x4) = uVar10;
    *(undefined2 *)((int)puStack6 + 0x6) = uVar7;
    if (*(int *)((int)param_1 + 0xa) != 0x0) {
      puVar2 = (undefined4 *)*(uint *)(uStack10 + 0x8);
      uVar3 = *(uint *)(uStack10 + 0xa);
      if ((uVar3 | (uint)puVar2) != 0x0) {
        ppcVar5 = (code **)*puVar2;
        (**ppcVar5)();
      }
    }
    if (puVar4 != (undefined4 *)0x0) {
      ppcVar5 = (code **)*puVar4;
      (**ppcVar5)();
    }
    piVar1 = (int *)((int)param_1 + 0x8);
    *piVar1 = *piVar1 + -0x1;
    return;
  }
  return;
}



void __stdcall16far pass1_1008_5ab8(ulong param_1)

{
  int *piVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined2 uVar5;
  uint uVar6;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x4) == 0x0) {
    return;
  }
  puVar3 = (undefined4 *)*(undefined4 *)(iVar4 + 0x4);
  uVar6 = (uint)((ulong)puVar3 >> 0x10);
  *(undefined4 *)(iVar4 + 0x4) = *(undefined4 *)((uint)puVar3 + 0x4);
  if ((uVar6 | (uint)puVar3) != 0x0) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)();
  }
  piVar1 = (int *)(iVar4 + 0x8);
  *piVar1 = *piVar1 + -0x1;
  return;
}



void __stdcall16far pass1_1008_5b12(long *param_1)

{
  undefined4 uVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  
  if ((*param_1 != 0x0) && (*(int *)((int)*param_1 + 0x8) != 0x0)) {
    uVar4 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (int)param_1;
    if (*(long *)(iVar2 + 0x4) == 0x0) {
      uVar5 = (undefined2)((ulong)*param_1 >> 0x10);
      iVar3 = (int)*param_1;
    }
    else {
      uVar1 = *(undefined4 *)(iVar2 + 0x4);
      uVar5 = (undefined2)((ulong)uVar1 >> 0x10);
      iVar3 = (int)uVar1;
    }
    *(undefined4 *)(iVar2 + 0x4) = *(undefined4 *)(iVar3 + 0x4);
    if (*(long *)(iVar2 + 0x4) != 0x0) {
      return;
    }
  }
  return;
}



ushort * __stdcall16far pass1_1008_5b6e(ushort *param_1,byte param_2)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = 0x389a;
  ((int *)param_1)[0x1] = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((int *)param_1,uVar1,0x1000);
  }
  return param_1;
}

void __stdcall16far pass1_1008_5c34(ushort *param_1)

{
  ushort unaff_SS;
  
  *param_1 = 0x5fc8;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  _PTR_LOOP_1050_02a0 = 0x0;
  pass1_1010_1d80(param_1,unaff_SS);
  return;
}



void __cdecl16far pass1_1008_612e(int param_1,int param_2,uint param_3)

{
  uint uVar1;
  uint uVar2;
  long lVar3;
  int iVar4;
  int iStack18;
  int iStack16;
  
  uVar1 = pass1_1000_4d24();
  uVar2 = (param_2 - param_1) + 0x1;
  if (((int)uVar2 >> 0xf | uVar2) == 0x0) {
    return;
  }
  iStack16 = 0x1;
  iStack18 = param_1;
  do {
    if (param_2 < iStack18) {
      return;
    }
    lVar3 = (long)iStack16 * (long)(0x7fff / (sqword)(long)(int)uVar2);
    iVar4 = (int)((ulong)lVar3 >> 0x10);
    if ((int)uVar1 >> 0xf <= iVar4) {
      if ((int)uVar1 >> 0xf < iVar4) {
        return;
      }
      if (uVar1 <= (uint)lVar3) {
        return;
      }
    }
    iStack18 = iStack18 + 0x1;
    iStack16 = iStack16 + 0x1;
  } while( true );
}



void __stdcall16far pass1_1008_64a2(uint *param_1)

{
  uint uVar1;
  code **ppcVar2;
  
  uVar1 = *(uint *)((int)param_1 + 0x2);
  if ((uVar1 | (uint)(undefined4 *)*param_1) != 0x0) {
    ppcVar2 = (code **)*(undefined4 *)*param_1;
    (**ppcVar2)();
  }
  return;
}


ulong __stdcall16far pass1_1008_4b5e(ulong *param_1)

{
  code **ppcVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(int *)(iVar3 + 0x1e) == 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x8);
    iVar2 = (**ppcVar1)();
    if (iVar2 == 0x0) {
      return 0x0;
    }
  }
  return CONCAT22(*(undefined2 *)(iVar3 + 0x6),*(undefined2 *)(iVar3 + 0x4));
}


ushort __stdcall16far pass1_1008_4d26(ulong param_1,ushort *param_2,int param_3)

{
  int *piVar1;
  undefined2 uVar2;
  long lVar3;
  astruct_650 *iVar5;
  astruct_649 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar5 = (astruct_650 *)param_1;
  if (((iVar5->field_0x4 != 0x0) && (-0x1 < param_3)) &&
     (piVar1 = &iVar5->field_0xc, *piVar1 != param_3 && param_3 <= *piVar1)) {
    uVar2 = *(undefined2 *)((int)param_2 + 0x2);
    lVar3 = iVar5->field_0x4;
    uVar4 = (undefined2)((ulong)lVar3 >> 0x10);
    iVar4 = (astruct_649 *)lVar3;
    *(ushort *)(iVar4 + param_3 * 0x4) = *param_2;
    *(undefined2 *)(iVar4 + param_3 * 0x4 + 0x2) = uVar2;
    return 0x1;
  }
  return 0x0;
}



ulong __stdcall16far pass1_1008_4d72(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0x6),*(undefined2 *)((int)param_1 + 0x4));
}



void __stdcall16far pass1_1008_50c2(astruct_110 *param_1,ulong param_2,ulong param_3,uint *param_4,ulong param_5)

{
  astruct_110 *iVar1;
  uint uVar1;
  
  param_1->field_0x0 = *param_4;
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_110 *)param_1;
  iVar1->field_0x2 = *(undefined2 *)((int)param_4 + 0x2);
  iVar1->field_0x4 = param_3;
  iVar1->field_0x8 = param_2;
  iVar1->field_0xc = param_5;
  iVar1->field_0x10 = 0x0;
  pass1_1008_52fc((uint *)((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10));
  return;
}


void __stdcall16far pass1_1008_5134(ulong param_1)

{
  uint *puVar1;
  int iVar2;
  uint uVar3;
  long lVar4;
  int iVar5;
  int iVar6;
  undefined2 uVar7;
  int iStack16;
  long lStack14;
  ulong uStack10;
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  lVar4 = *(long *)(iVar6 + 0x4) * *(long *)(iVar6 + 0x8);
  lVar4 = mem_op_1000_0a48(0x1,(uint)lVar4,(int)((ulong)lVar4 >> 0x10),_PTR_LOOP_1050_5f2c,0x1000);
  uVar3 = (uint)((ulong)lVar4 >> 0x10);
  *(undefined2 *)(iVar6 + 0x10) = (int)lVar4;
  *(uint *)(iVar6 + 0x12) = uVar3;
  if ((uVar3 | *(uint *)(iVar6 + 0x10)) == 0x0) {
    return;
  }
  iVar5 = *(int *)(iVar6 + 0x8);
  iVar2 = *(int *)(iVar6 + 0xa);
  lVar4 = CONCAT22(iVar2 - (uint)(iVar5 == 0x0),iVar5 + -0x1) * *(long *)(iVar6 + 0x4);
  puVar1 = (uint *)(iVar6 + 0x10);
  uVar3 = (uint)lVar4;
  uStack10 = CONCAT22(((int)((ulong)lVar4 >> 0x10) + (uint)CARRY2(uVar3,*puVar1)) * 0x100 + *(int *)(iVar6 + 0x12),
                      uVar3 + *puVar1);
  lStack14 = CONCAT22(iVar2,iVar5);
  iStack16 = *(int *)(iVar6 + 0x2);
  while (lStack14 != 0x0) {
    iVar2 = iStack16 + 0x1;
    iVar5 = iStack16 >> 0xf;
    pass1_1008_4544(*(ulong *)(iVar6 + 0xc));
    pass1_1000_48a8(uStack10,CONCAT22(iVar5,iStack16),*(int *)(iVar6 + 0x4));
    iVar5 = *(int *)(iVar6 + 0x4);
    uVar3 = -iVar5;
    uStack10 = CONCAT22((int)(uStack10 >> 0x10) +
                        ((uint)CARRY2((uint)uStack10,uVar3) - (*(int *)(iVar6 + 0x6) + (uint)(iVar5 != 0x0))) * 0x100,
                        (uint)uStack10 + uVar3);
    iStack16 = iVar2;
    lStack14 = lStack14 + -0x1;
  }
  return;
}


void __stdcall16far pass1_1008_5236(ulong param_1)

{
  uint *puVar1;
  int iVar2;
  uint uVar3;
  long lVar4;
  int iVar5;
  astruct_109 *iVar6;
  undefined2 uVar6;
  bool bVar7;
  int iStack12;
  long lStack10;
  uint uStack6;
  int iStack4;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar6 = (astruct_109 *)param_1;
  iVar5 = iVar6->field_0x8;
  iVar2 = iVar6->field_0xa;
  lVar4 = CONCAT22(iVar2 - (uint)(iVar5 == 0x0),iVar5 + -0x1) * *(long *)&iVar6->field_0x4;
  puVar1 = &iVar6->field_0x10;
  uVar3 = (uint)lVar4;
  uStack6 = uVar3 + *puVar1;
  iStack4 = ((int)((ulong)lVar4 >> 0x10) + (uint)CARRY2(uVar3,*puVar1)) * 0x100 + iVar6->field_0x12;
  lStack10 = CONCAT22(iVar2,iVar5);
  iStack12 = iVar6->field_0x2;
  while (lStack10 != 0x0) {
    iVar2 = iStack12 + 0x1;
    iVar5 = iStack12 >> 0xf;
    pass1_1008_4544(iVar6->field_0xc);
    pass1_1000_48a8(CONCAT22(iVar5,iStack12),CONCAT22(iStack4,uStack6),*(int *)&iVar6->field_0x4);
    iVar5 = *(int *)&iVar6->field_0x4;
    uVar3 = -iVar5;
    bVar7 = CARRY2(uStack6,uVar3);
    uStack6 = uStack6 + uVar3;
    iStack4 = iStack4 + ((uint)bVar7 - (iVar6->field_0x6 + (uint)(iVar5 != 0x0))) * 0x100;
    iStack12 = iVar2;
    lStack10 = lStack10 + -0x1;
  }
  return;
}



void __stdcall16far pass1_1008_52fc(uint *param_1)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  long lVar4;
  undefined2 uVar5;
  int iVar6;
  int iVar7;
  astruct_111 *iVar8;
  undefined2 uVar8;
  ulong uVar9;
  uint uStack14;
  int iStack12;
  
  uVar8 = (undefined2)((ulong)param_1 >> 0x10);
  iVar8 = (astruct_111 *)param_1;
  uVar9 = pass1_1008_4772(iVar8->field_0xc);
  uVar5 = (undefined2)(uVar9 >> 0x10);
  iVar7 = (int)uVar9;
  iVar6 = *(int *)(iVar7 + 0x4);
  uVar3 = iVar6 - 0x1;
  iVar6 = *(int *)(iVar7 + 0x6) - (uint)(iVar6 == 0x0);
  lVar4 = *(long *)(iVar7 + 0x8) + -0x1;
  uVar2 = *param_1;
  puVar1 = &iVar8->field_0x4;
  iVar7 = ((int)uVar2 >> 0xf) + iVar8->field_0x6 + (uint)CARRY2(uVar2,*puVar1);
  if ((iVar6 <= iVar7) && ((iVar6 < iVar7 || (uVar3 < uVar2 + *puVar1)))) {
    iVar8->field_0x4 = uVar3 - uVar2;
    iVar8->field_0x6 = (iVar6 - ((int)uVar2 >> 0xf)) - (uint)(uVar3 < uVar2);
  }
  uVar2 = iVar8->field_0x2;
  puVar1 = &iVar8->field_0x8;
  iVar6 = ((int)uVar2 >> 0xf) + iVar8->field_0xa + (uint)CARRY2(uVar2,*puVar1);
  iStack12 = (int)((ulong)lVar4 >> 0x10);
  if ((iStack12 <= iVar6) && ((uStack14 = (uint)lVar4, iStack12 < iVar6 || (uStack14 < uVar2 + *puVar1)))) {
    iVar8->field_0x8 = uStack14 - uVar2;
    iVar8->field_0xa = (iStack12 - ((int)uVar2 >> 0xf)) - (uint)(uStack14 < uVar2);
  }
  return;
}



ulong * __stdcall16far pass1_1008_5394(ulong *param_1)

{
  *param_1 = 0x0;
  return param_1;
}



void __stdcall16far pass1_1008_53aa(void)

{
  return;
}



void __stdcall16far pass1_1008_5784(ulong *param_1,ulong param_2)

{
  *param_1 = param_2;
  *(undefined4 *)((int)param_1 + 0x4) = 0x0;
  return;
}



void __stdcall16far pass1_1008_57a4(ulong *param_1,ulong param_2)

{
  *param_1 = param_2;
  *(undefined4 *)((int)param_1 + 0x4) = 0x0;
  return;
}



void __stdcall16far pass1_1008_57c4(ushort *param_1)

{
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  *param_1 = 0x5bc4;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  pass1_1008_5830((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  return;
}



void __stdcall16far pass1_1008_3e0e(ulong param_1)

{
  code **ppcVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x4) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x4);
    (**ppcVar1)();
  }
  return;
}



ushort * __stdcall16far pass1_1008_3e38(ushort *param_1)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x2) = 0x0;
  *(undefined2 *)((int)param_1 + 0x4) = 0x0;
  return param_1;
}



ushort * __stdcall16far pass1_1008_3e54(ushort *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = param_4;
  *(ushort *)((int)param_1 + 0x2) = param_3;
  *(ushort *)((int)param_1 + 0x4) = param_2;
  return param_1;
}



void __stdcall16far pass1_1008_3e76(ushort *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = param_4;
  *(ushort *)((int)param_1 + 0x2) = param_3;
  *(ushort *)((int)param_1 + 0x4) = param_2;
  return;
}



void __stdcall16far pass1_1008_3e94(ushort *param_1,ushort *param_2,ushort *param_3)

{
  *param_3 = *param_1;
  *param_2 = *(ushort *)((int)param_1 + 0x2);
  return;
}



void __stdcall16far pass1_1008_3eb4(ushort *param_1,ushort *param_2,ushort *param_3,ushort *param_4)

{
  ushort uVar1;
  
  *param_4 = *param_1;
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_3 = *(ushort *)((int)param_1 + 0x2);
  *param_2 = *(ushort *)((int)param_1 + 0x4);
  return;
}



void __stdcall16far pass1_1008_3ee2(int *param_1,int *param_2)

{
  int iVar1;
  int iVar2;
  ushort uVar3;
  ushort uVar4;
  
  iVar1 = *param_2 - *param_1;
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  *param_1 = iVar1 + 0x1;
  uVar3 = (ushort)((ulong)param_2 >> 0x10);
  uVar4 = (ushort)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  iVar1 = *(int *)((int)param_2 + 0x2) - *(int *)(iVar2 + 0x2);
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  *(int *)(iVar2 + 0x2) = iVar1 + 0x1;
  iVar1 = *(int *)((int)param_2 + 0x4) - *(int *)(iVar2 + 0x4);
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  *(int *)(iVar2 + 0x4) = iVar1 + 0x1;
  return;
}



void __stdcall16far pass1_1008_3f32(int *param_1,int *param_2)

{
  int *piVar1;
  ushort uVar2;
  ushort uVar3;
  
  *param_1 = *param_1 + *param_2;
  uVar2 = (ushort)((ulong)param_2 >> 0x10);
  uVar3 = (ushort)((ulong)param_1 >> 0x10);
  piVar1 = (int *)((int)param_1 + 0x2);
  *piVar1 = *piVar1 + *(int *)((int)param_2 + 0x2);
  piVar1 = (int *)((int)param_1 + 0x4);
  *piVar1 = *piVar1 + *(int *)((int)param_2 + 0x4);
  return;
}



void __stdcall16far pass1_1008_3f62(ushort *param_1,ushort *param_2)

{
  undefined2 uVar1;
  undefined2 uVar2;
  
  *param_1 = *param_2;
  uVar1 = (undefined2)((ulong)param_2 >> 0x10);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x2) = *(undefined2 *)((int)param_2 + 0x2);
  *(undefined2 *)((int)param_1 + 0x4) = *(undefined2 *)((int)param_2 + 0x4);
  return;
}


void __stdcall16far pass1_1008_431c(ulong param_1,byte param_2)

{
  ulong *puVar1;
  undefined4 uVar2;
  bool bVar3;
  ulong uVar4;
  int iVar5;
  uint uVar6;
  undefined4 uStack6;
  
  uVar6 = (uint)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(long *)(iVar5 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | (ulong)uVar6 << 0x10));
  }
  if ((*(uint *)(iVar5 + 0x8) | *(uint *)(iVar5 + 0x6)) == 0x0) {
    bVar3 = false;
  }
  else {
    if ((*(uint *)(iVar5 + 0xc) | *(uint *)(iVar5 + 0xa)) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | (ulong)uVar6 << 0x10));
    }
    bVar3 = true;
  }
  if (bVar3) {
    if ((*(uint *)(iVar5 + 0x16) | *(uint *)(iVar5 + 0x14)) == 0x0) {
      return;
    }
    uStack6 = 0x0;
    while( true ) {
      uVar2 = *(undefined4 *)(iVar5 + 0x10);
      puVar1 = (ulong *)((int)uVar2 + 0x8);
      if (*puVar1 == uStack6 || (long)*puVar1 < (long)uStack6) break;
      uVar4 = uStack6;
      pass1_1008_4544(param_1);
      uVar2 = *(undefined4 *)(iVar5 + 0x10);
      pass1_1000_4906((astruct_20 *)(uVar4 & 0xffff | (ulong)uStack6._2_2_ << 0x10),(WNDCLASS16 *)(uint)param_2,
                      *(uint *)((int)uVar2 + 0x4));
      uStack6 = uStack6 + 0x1;
    }
  }
  return;
}



undefined4 __stdcall16far pass1_1008_43cc(ulong param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
  }
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(iVar2 + 0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(*(undefined2 *)(iVar2 + 0x16),*(undefined2 *)(iVar2 + 0x14));
}



ulong __stdcall16far pass1_1008_4426(ulong param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
  }
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(iVar2 + 0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(*(undefined2 *)(iVar2 + 0xc),*(undefined2 *)(iVar2 + 0xa));
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1008_4480(ulong param_1,ushort *param_2,astruct_76 *param_3,ushort param_4)

{
  int iVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  uint uVar5;
  ulong uVar6;
  int iStack26;
  char *pcStack24;
  char *pcStack20;
  int iStack16;
  int local_6;
  undefined local_4 [0x2];
  
  pass1_1008_3e94(param_2,(ushort *)CONCAT22(param_4,&local_6),(ushort *)CONCAT22(param_4,local_4));
  uVar6 = pass1_1008_4772(param_3);
  uVar4 = (undefined2)(uVar6 >> 0x10);
  iVar1 = *(int *)((int)uVar6 + 0x4);
  iVar2 = *(int *)((int)uVar6 + 0x8);
  for (iStack16 = 0x0; iStack16 < iVar2; iStack16 = iStack16 + 0x1) {
    uVar5 = local_6 >> 0xf;
    iVar3 = local_6;
    local_6 = local_6 + 0x1;
    pass1_1008_4544(param_1);
    pcStack20 = (char *)CONCAT22(uVar5,iVar3);
    uVar6 = SEXT24(iStack16);
    pass1_1008_4544((ulong)param_3);
    pcStack24 = (char *)(uVar6 & 0xffff | (ulong)uVar5 << 0x10);
    iStack26 = iVar1;
    while (iStack26 != 0x0) {
      if (*pcStack24 != -0x1) {
        *pcStack20 = *pcStack24;
      }
      pcStack24 = (char *)CONCAT22((int)((ulong)pcStack24 >> 0x10) + (-(uint)(0xfffe < (uint)pcStack24) & 0x6c),
                                   (uint)pcStack24 + 0x1);
      pcStack20 = (char *)CONCAT22((int)((ulong)pcStack20 >> 0x10) + (-(uint)(0xfffe < (uint)pcStack20) & 0x6c),
                                   (uint)pcStack20 + 0x1);
      iStack26 = iStack26 + -0x1;
    }
  }
  return;
}



void __stdcall16far pass1_1008_4544(ulong param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
  }
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(iVar2 + 0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return;
  }
  return;
}


ulong __stdcall16far pass1_1008_4772(astruct_76 *param_1)

{
  bool bVar1;
  astruct_76 *iVar2;
  uint uVar2;
  
  uVar2 = (uint)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  if (*(long *)&iVar2->field_0x6 == 0x0) {
    pass1_1008_47cc((astruct_76 *)((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10));
  }
  if (*(long *)&iVar2->field_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)((int)&iVar2->field_0x8 + 0x2) == 0x0) {
      pass1_1008_4834((astruct_76 *)((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(iVar2->field_0x12,*(undefined2 *)((int)&iVar2->field_0xe + 0x2));
}



void __stdcall16far pass1_1008_47cc(astruct_76 *param_1)

{
  ulong uVar1;
  undefined4 uVar2;
  uint uVar3;
  int iVar5;
  int iVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  ulong uStack14;
  int iVar4;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(long *)(iVar5 + 0x6) != 0x0) {
    uVar1 = *(ulong *)(iVar5 + 0x6);
    iVar6 = *(int *)(iVar5 + 0x8);
    iVar4 = (int)uVar1;
    uVar3 = iVar4 + 0xe;
    *(ulong *)(iVar5 + 0x10) = uVar1 & 0xffff0000 | (ulong)uVar3;
    *(int *)(iVar5 + 0x14) = iVar4 + 0x436;
    *(int *)(iVar5 + 0x16) = iVar6 + (-(uint)(0xfbd7 < uVar3) & 0x6c);
    uVar2 = *(undefined4 *)(iVar5 + 0x10);
    uVar8 = (undefined2)((ulong)uVar2 >> 0x10);
    iVar6 = (int)uVar2;
    uStack14 = (ulong)*(uint *)(iVar6 + 0xe);
    *(long *)(iVar5 + 0x18) = (long)(uStack14 * *(long *)(iVar6 + 0x4) + 0x1f) / 0x20 << 0x2;
  }
  return;
}


ushort __stdcall16far pass1_1008_48aa(ulong param_1)

{
  return *(ushort *)((int)param_1 + 0xe);
}


void __stdcall16far pass1_1008_3714(ulong param_1)

{
  pass1_1008_3e0e(param_1);
  return;
}



ulong __stdcall16far pass1_1008_372c(int param_1,ushort param_2)

{
  return CONCAT22(param_2,param_1 + 0xa);
}



void __stdcall16far pass1_1008_373c(void)

{
  return;
}



void __stdcall16far pass1_1008_3740(void)

{
  return;
}



void __stdcall16far pass1_1008_3744(void)

{
  return;
}



void __stdcall16far pass1_1008_3748(void)

{
  return;
}



void __stdcall16far pass1_1008_374c(void)

{
  return;
}



void __stdcall16far pass1_1008_3750(void)

{
  return;
}



void __stdcall16far pass1_1008_3754(void)

{
  return;
}



ushort __stdcall16far pass1_1008_3758(void)

{
  return 0x1;
}



void __stdcall16far pass1_1008_375e(void)

{
  return;
}



void __stdcall16far pass1_1008_3762(void)

{
  return;
}



void __stdcall16far pass1_1008_3766(void)

{
  return;
}



void __stdcall16far pass1_1008_377a(void)

{
  return;
}


ushort * __stdcall16far pass1_1008_392e(ushort *param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x3aa8;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *(ushort *)(iVar1 + 0x4) = param_2;
  *param_1 = 0x3ab0;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x3aa0;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return param_1;
}



void __stdcall16far pass1_1008_397a(ushort *param_1)

{
  astruct_452 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_452 *)param_1;
  *param_1 = 0x3aa0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}

void __stdcall16far pass1_1008_3a10(void)

{
  return;
}

void __stdcall16far pass1_1008_1246(ulong param_1)

{
  code **ppcVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0xe8) + 0x4c);
    (**ppcVar1)();
  }
  return;
}



void __stdcall16far pass1_1008_1272(ulong param_1,int param_2)

{
  code **ppcVar1;
  uint uVar2;
  
  uVar2 = (uint)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0xe8) + 0x88);
    (**ppcVar1)();
    return;
  }
  pass1_1008_9cc4(param_1 & 0xffff | (ulong)uVar2 << 0x10,param_2);
  return;
}



void __stdcall16far pass1_1008_12aa(ulong param_1)

{
  code **ppcVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0xe8) + 0x8c);
    (**ppcVar1)();
    return;
  }
  pass1_1008_9ce0();
  return;
}

ulong __stdcall16far pass1_1000_5224(uint param_1,uint param_2,uint param_3,uint param_4)

{
  ulong uVar1;
  long lVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  bool bVar10;
  char cVar11;
  uint uVar9;
  
  cVar11 = (int)param_2 < 0x0;
  if ((bool)cVar11) {
    bVar10 = param_1 != 0x0;
    param_1 = -param_1;
    param_2 = -(uint)bVar10 - param_2;
  }
  if ((int)param_4 < 0x0) {
    cVar11 = cVar11 + '\x01';
    bVar10 = param_3 != 0x0;
    param_3 = -param_3;
    param_4 = -(uint)bVar10 - param_4;
  }
  uVar3 = param_1;
  uVar5 = param_3;
  uVar6 = param_2;
  uVar9 = param_4;
  if (param_4 == 0x0) {
    uVar3 = param_2 / param_3;
    iVar4 = (int)(((ulong)param_2 % (ulong)param_3 << 0x10 | (ulong)param_1) / (ulong)param_3);
  }
  else {
    do {
      uVar8 = uVar9 >> 0x1;
      uVar5 = uVar5 >> 0x1 | (uint)((uVar9 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar6 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (uint)((uVar6 & 0x1) != 0x0) << 0xf;
      uVar6 = uVar7;
      uVar9 = uVar8;
    } while (uVar8 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / (ulong)uVar5;
    iVar4 = (int)uVar1;
    lVar2 = (ulong)param_3 * (uVar1 & 0xffff);
    uVar3 = (uint)((ulong)lVar2 >> 0x10);
    uVar5 = uVar3 + iVar4 * param_4;
    if (((CARRY2(uVar3,iVar4 * param_4)) || (param_2 < uVar5)) || ((param_2 <= uVar5 && (param_1 < (uint)lVar2)))) {
      iVar4 = iVar4 + -0x1;
    }
    uVar3 = 0x0;
  }
  if (cVar11 == '\x01') {
    bVar10 = iVar4 != 0x0;
    iVar4 = -iVar4;
    uVar3 = -(uint)bVar10 - uVar3;
  }
  return CONCAT22(uVar3,iVar4);
}



ulong __stdcall16far pass1_1000_52be(uint param_1,uint param_2,uint param_3,uint param_4)

{
  if ((param_4 | param_2) == 0x0) {
    return (ulong)param_1 * (ulong)param_3;
  }
  return (ulong)param_1 * (ulong)param_3 & 0xffff |
         (ulong)((int)((ulong)param_1 * (ulong)param_3 >> 0x10) + param_2 * param_3 + param_1 * param_4) << 0x10;
}



ulong __stdcall16far pass1_1000_52f0(uint param_1,uint param_2,uint param_3,uint param_4)

{
  ulong uVar1;
  long lVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  uint uVar11;
  bool bVar12;
  bool bVar13;
  
  bVar13 = (int)param_2 < 0x0;
  if (bVar13) {
    bVar12 = param_1 != 0x0;
    param_1 = -param_1;
    param_2 = -(uint)bVar12 - param_2;
  }
  uVar11 = (uint)bVar13;
  if ((int)param_4 < 0x0) {
    bVar13 = param_3 != 0x0;
    param_3 = -param_3;
    param_4 = -(uint)bVar13 - param_4;
  }
  uVar3 = param_1;
  uVar4 = param_3;
  uVar8 = param_2;
  uVar9 = param_4;
  if (param_4 == 0x0) {
    iVar5 = (int)(((ulong)param_2 % (ulong)param_3 << 0x10 | (ulong)param_1) % (ulong)param_3);
    iVar6 = 0x0;
    if ((int)(uVar11 - 0x1) < 0x0) goto LAB_1000_538a;
  }
  else {
    do {
      uVar10 = uVar9 >> 0x1;
      uVar4 = uVar4 >> 0x1 | (uint)((uVar9 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar8 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (uint)((uVar8 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar7;
      uVar9 = uVar10;
    } while (uVar10 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / (ulong)uVar4;
    uVar3 = (int)uVar1 * param_4;
    lVar2 = (uVar1 & 0xffff) * (ulong)param_3;
    uVar8 = (uint)((ulong)lVar2 >> 0x10);
    uVar4 = (uint)lVar2;
    uVar9 = uVar8 + uVar3;
    if (((CARRY2(uVar8,uVar3)) || (param_2 < uVar9)) || ((param_2 <= uVar9 && (param_1 < uVar4)))) {
      bVar13 = uVar4 < param_3;
      uVar4 = uVar4 - param_3;
      uVar9 = (uVar9 - param_4) - (uint)bVar13;
    }
    iVar5 = uVar4 - param_1;
    iVar6 = (uVar9 - param_2) - (uint)(uVar4 < param_1);
    if (-0x1 < (int)(uVar11 - 0x1)) goto LAB_1000_538a;
  }
  bVar13 = iVar5 != 0x0;
  iVar5 = -iVar5;
  iVar6 = -(uint)bVar13 - iVar6;
LAB_1000_538a:
  return CONCAT22(iVar6,iVar5);
}



ulong __stdcall16far pass1_1000_5390(uint param_1,uint param_2,uint param_3,uint param_4)

{
  ulong uVar1;
  long lVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  
  uVar3 = param_1;
  uVar8 = param_4;
  uVar6 = param_2;
  uVar9 = param_3;
  if (param_4 == 0x0) {
    uVar3 = param_2 / param_3;
    iVar4 = (int)(((ulong)param_2 % (ulong)param_3 << 0x10 | (ulong)param_1) / (ulong)param_3);
  }
  else {
    do {
      uVar5 = uVar8 >> 0x1;
      uVar9 = uVar9 >> 0x1 | (uint)((uVar8 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar6 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (uint)((uVar6 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar5;
      uVar6 = uVar7;
    } while (uVar5 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / (ulong)uVar9;
    iVar4 = (int)uVar1;
    lVar2 = (ulong)param_3 * (uVar1 & 0xffff);
    uVar3 = (uint)((ulong)lVar2 >> 0x10);
    uVar8 = uVar3 + iVar4 * param_4;
    if (((CARRY2(uVar3,iVar4 * param_4)) || (param_2 < uVar8)) || ((param_2 <= uVar8 && (param_1 < (uint)lVar2)))) {
      iVar4 = iVar4 + -0x1;
    }
    uVar3 = 0x0;
  }
  return CONCAT22(uVar3,iVar4);
}



ulong __stdcall16far pass1_1000_53f0(uint param_1,uint param_2,uint param_3,uint param_4)

{
  ulong uVar1;
  long lVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  bool bVar11;
  
  uVar3 = param_1;
  uVar4 = param_4;
  uVar9 = param_2;
  uVar10 = param_3;
  if (param_4 == 0x0) {
    iVar6 = (int)(((ulong)param_2 % (ulong)param_3 << 0x10 | (ulong)param_1) % (ulong)param_3);
    iVar7 = 0x0;
  }
  else {
    do {
      uVar5 = uVar4 >> 0x1;
      uVar10 = uVar10 >> 0x1 | (uint)((uVar4 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar9 >> 0x1;
      uVar3 = uVar3 >> 0x1 | (uint)((uVar9 & 0x1) != 0x0) << 0xf;
      uVar4 = uVar5;
      uVar9 = uVar8;
    } while (uVar5 != 0x0);
    uVar1 = CONCAT22(uVar8,uVar3) / (ulong)uVar10;
    uVar3 = (int)uVar1 * param_4;
    lVar2 = (uVar1 & 0xffff) * (ulong)param_3;
    uVar9 = (uint)((ulong)lVar2 >> 0x10);
    uVar4 = (uint)lVar2;
    uVar10 = uVar9 + uVar3;
    if (((CARRY2(uVar9,uVar3)) || (param_2 < uVar10)) || ((param_2 <= uVar10 && (param_1 < uVar4)))) {
      bVar11 = uVar4 < param_3;
      uVar4 = uVar4 - param_3;
      uVar10 = (uVar10 - param_4) - (uint)bVar11;
    }
    iVar6 = -(uVar4 - param_1);
    iVar7 = -(uint)(uVar4 - param_1 != 0x0) - ((uVar10 - param_2) - (uint)(uVar4 < param_1));
  }
  return CONCAT22(iVar7,iVar6);
}



int __cdecl16far pass1_1000_545a(ulong param_1,ulong param_2)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  byte *pbVar5;
  byte *pbVar6;
  
  pbVar6 = (byte *)param_2;
  pbVar5 = (byte *)param_1;
  bVar4 = 0xff;
  do {
    do {
      if (bVar4 == 0x0) goto LAB_1000_5499;
      pbVar1 = pbVar6;
      pbVar6 = pbVar6 + 0x1;
      bVar4 = *pbVar1;
      bVar3 = *pbVar5;
      pbVar5 = pbVar5 + 0x1;
    } while (bVar3 == bVar4);
    bVar2 = bVar4 + 0xbf + (-((byte)(bVar4 + 0xbf) < 0x1a) & 0x20U) + 0x41;
    bVar3 = bVar3 + 0xbf;
    bVar4 = bVar3 + (-(bVar3 < 0x1a) & 0x20U) + 0x41;
  } while (bVar4 == bVar2);
  bVar4 = (bVar4 < bVar2) * -0x2 + 0x1;
LAB_1000_5499:
  return (int)(char)bVar4;
}



uint * __cdecl16far pass1_1000_54a0(ulong param_1,uint param_2,uint param_3)

{
  uint *puVar1;
  undefined uVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint *puVar7;
  int iVar8;
  
  if (param_3 != 0x0) {
    iVar8 = (int)(param_1 >> 0x10);
    uVar5 = -(int)(uint *)param_1;
    uVar6 = param_3;
    if (uVar5 != 0x0) {
      uVar6 = (uVar5 - param_3 & -(uint)(uVar5 < param_3)) + param_3;
      uVar5 = param_3 - uVar6;
    }
    uVar3 = param_2 & 0xff | param_2 << 0x8;
    puVar7 = (uint *)param_1;
    for (uVar4 = uVar6 >> 0x1; uVar4 != 0x0; uVar4 = uVar4 - 0x1) {
      puVar1 = puVar7;
      puVar7 = puVar7 + 0x1;
      *puVar1 = uVar3;
    }
    for (uVar6 = (uint)((uVar6 & 0x1) != 0x0); uVar2 = (undefined)(param_2 & 0xff), uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
      puVar1 = puVar7;
      puVar7 = (uint *)((int)puVar7 + 0x1);
      *(undefined *)puVar1 = uVar2;
    }
    if (uVar5 != 0x0) {
      for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
        puVar1 = puVar7;
        puVar7 = puVar7 + 0x1;
        *puVar1 = uVar3;
      }
      for (uVar6 = (uint)((uVar5 & 0x1) != 0x0); uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
        puVar1 = puVar7;
        puVar7 = (uint *)((int)puVar7 + 0x1);
        *(undefined *)puVar1 = uVar2;
      }
    }
  }
  return (uint *)param_1;
}



void __stdcall16far pass1_1000_54e8(uchar *param_1,ushort param_2,int param_3,int param_4,int param_5,ushort param_6)

{
  int iVar1;
  
  iVar1 = param_3;
  while (iVar1 = iVar1 + -0x1, -0x1 < iVar1) {
    (*(code *)param_1)();
  }
  return;
}



void __stdcall16far pass1_1000_5512(uchar *param_1,ushort param_2,int param_3,int param_4,ushort param_5)

{
  bool bVar1;
  ushort uStack4;
  
  pass1_1000_52be(param_3,param_4,param_5,0x0);
  while( true ) {
    bVar1 = param_3 == 0x0;
    param_3 = param_3 + -0x1;
    param_4 = param_4 - (uint)bVar1;
    if (param_4 < 0x0) break;
    uStack4 = param_2;
    (*(code *)param_1)();
  }
  return;
}



void __stdcall16far pass1_1000_5586(uchar *param_1,ushort param_2,int param_3,int param_4,int param_5,ushort param_6)

{
  int iVar1;
  
  iVar1 = param_3;
  while (iVar1 = iVar1 + -0x1, -0x1 < iVar1) {
    (*(code *)param_1)();
  }
  return;
}



void __cdecl16near ret_op_1000_55ac(void)

{
  return;
}
