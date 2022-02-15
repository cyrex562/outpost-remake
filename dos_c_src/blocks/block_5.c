
ushort __cdecl16near pass1_1000_2e74(uint *param_1,ushort param_2)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  uint *puVar4;
  uint *puVar5;
  
  if (PTR_LOOP_1050_61ec != (undefined *)0x0) {
    puVar5 = param_1 + 0x78;
    puVar4 = (uint *)0x5ff2;
    if ((param_1 == (uint *)0x621c) || (puVar4 = (uint *)&PTR_LOOP_1050_5ff6, param_1 == (uint *)0x6228)) {
      if (((*(byte *)(param_1 + 0x5) & 0xc) == 0x0) && ((*(byte *)puVar5 & 0x1) == 0x0)) {
        uVar2 = *puVar4;
        uVar3 = puVar4[0x1];
        if ((uVar2 | uVar3) == 0x0) {
          uVar2 = mem_1000_167a(0x200,param_2,uVar3);
          if (uVar3 == 0x0) {
            return 0x0;
          }
          *puVar4 = uVar2;
          puVar4[0x1] = uVar3;
        }
        param_1[0x3] = uVar2;
        param_1[0x4] = uVar3;
        *param_1 = uVar2;
        param_1[0x1] = uVar3;
        param_1[0x2] = 0x200;
        param_1[0x79] = 0x200;
        puVar1 = param_1 + 0x5;
        *(byte *)puVar1 = *(byte *)puVar1 | 0x2;
        *(byte *)puVar5 = 0x11;
        return 0x1;
      }
    }
    else {
      if ((byte)DAT_1050_5f8a <= *(byte *)((int)param_1 + 0xb)) {
        puVar1 = puVar5;
        *(byte *)puVar1 = *(byte *)puVar1 | 0x10;
      }
    }
  }
  return 0x0;
}



void __cdecl16near pass1_1000_2f00(int param_1,int *param_2,ushort param_3,ushort param_4,ushort param_5,uchar param_6)

{
  if (((*(byte *)(param_2 + 0x78) & 0x10) != 0x0) && ((*(byte *)(*(byte *)((int)param_2 + 0xb) + 0x5f90) & 0x40) != 0x0)
     ) {
    pass1_1000_2fa4(param_2,param_3,param_4,param_5,param_6);
    if (param_1 != 0x0) {
      *(byte *)(param_2 + 0x78) = 0x0;
      param_2[0x79] = 0x0;
      *param_2 = 0x0;
      param_2[0x1] = 0x0;
      param_2[0x3] = 0x0;
      param_2[0x4] = 0x0;
    }
  }
  return;
}



ushort __cdecl16far pass1_1000_2f48(long param_1,int param_2,ushort param_3,ushort param_4,ushort param_5,uchar param_6)

{
  ushort uVar1;
  uchar *puVar2;
  int iStack2;
  
  iStack2 = param_2 + 0x1;
  if (param_1 == 0x0) {
    uVar1 = pass1_1000_3038(0x0,param_3,param_4,param_5,param_6);
  }
  else {
    uVar1 = pass1_1000_2fa4((int *)param_1,param_3,param_4,param_5,param_6);
    if (uVar1 == 0x0) {
      if ((*(byte *)((int *)param_1 + 0x78) & 0x40) != 0x0) {
        puVar2 = pass1_1000_400a((uint)*(byte *)((int)(int *)param_1 + 0xb),(ushort)&iStack2);
        uVar1 = -(uint)(puVar2 != (uchar *)0x0);
      }
    }
    else {
      uVar1 = 0xffff;
    }
  }
  return uVar1;
}



ushort __cdecl16near pass1_1000_2fa4(int *param_1,ushort param_2,ushort param_3,ushort param_4,uchar param_5)

{
  int *piVar1;
  byte bVar2;
  int iVar3;
  uchar *puVar4;
  uchar *puVar5;
  ushort uVar6;
  
  uVar6 = 0x0;
  bVar2 = *(byte *)(param_1 + 0x5);
  if (((bVar2 & 0x3) == 0x2) && (((bVar2 & 0x8) != 0x0 || ((*(byte *)(param_1 + 0x78) & 0x1) != 0x0)))) {
    puVar4 = (uchar *)(*param_1 - param_1[0x3]);
    if (0x0 < (int)puVar4) {
      puVar5 = mixed_dos3_call_1000_39f2
                         ((uchar *)(uint)*(byte *)((int)param_1 + 0xb),(char *)CONCAT22(param_1[0x4],param_1[0x3]),
                          puVar4,param_2,param_3,param_4,param_5);
      if (puVar5 == puVar4) {
        if ((*(byte *)(param_1 + 0x5) & 0x80) != 0x0) {
          piVar1 = param_1 + 0x5;
          *(byte *)piVar1 = *(byte *)piVar1 & 0xfd;
        }
      }
      else {
        piVar1 = param_1 + 0x5;
        *(byte *)piVar1 = *(byte *)piVar1 | 0x20;
        uVar6 = 0xffff;
      }
    }
  }
  iVar3 = param_1[0x4];
  *param_1 = param_1[0x3];
  param_1[0x1] = iVar3;
  param_1[0x2] = 0x0;
  return uVar6;
}



void __cdecl16far pass1_1000_3024(ushort param_1,ushort param_2,ushort param_3,uchar param_4)

{
  pass1_1000_3038(0x1,param_1,param_2,param_3,param_4);
  return;
}



int pass1_1000_3038(int param_1,ushort param_2,ushort param_3,ushort param_4,uchar param_5)

{
  ushort uVar1;
  undefined *puVar2;
  int iVar3;
  int iStack4;
  
  iVar3 = 0x0;
  iStack4 = 0x0;
  for (puVar2 = (undefined *)&PTR_LOOP_1050_6210; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0xc) {
    if ((param_1 == 0x1) && ((puVar2[0xa] & 0x83) != 0x0)) {
      uVar1 = pass1_1000_2f48(CONCAT22(0x1050,puVar2),(int)&stack0xfffe,param_2,param_3,param_4,param_5);
      if (uVar1 != 0xffff) {
        iVar3 = iVar3 + 0x1;
      }
    }
    else {
      if ((param_1 == 0x0) &&
         (((puVar2[0xa] & 0x2) != 0x0 &&
          (uVar1 = pass1_1000_2f48(CONCAT22(0x1050,puVar2),(int)&stack0xfffe,param_2,param_3,param_4,param_5),
          uVar1 == 0xffff)))) {
        iStack4 = -0x1;
      }
    }
  }
  if (param_1 == 0x1) {
    iStack4 = iVar3;
  }
  return iStack4;
}



// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING: Unable to track spacebase fully for stack

ushort pass1_1000_30a4(int param_1,uint param_2,uint param_3,uint param_4,int param_5,uint param_6,ushort param_7,
                      ushort param_8,ushort param_9,byte param_10)

{
  uint *puVar1;
  char cVar2;
  char *pcVar3;
  byte bVar4;
  ushort uVar5;
  uint *puVar6;
  
  puVar6 = (uint *)(param_5 + *(int *)(param_3 + param_6) + (uint)param_10);
  puVar1 = puVar6;
  *puVar1 = *puVar1 ^ (uint)puVar6;
  puVar1 = (uint *)((int)puVar6 + param_3 + 0x31);
  *puVar1 = *puVar1 ^ param_4;
  puVar1 = (uint *)((int)puVar6 + -0x3acf);
  *puVar1 = *puVar1 ^ param_3;
  puVar1 = puVar6 + -0x3794;
  *puVar1 = *puVar1 ^ param_2;
  *(int *)(param_1 + -0x2) = param_4 + 0x1;
  *(undefined2 *)(param_1 + -0x4) = (int)&USHORT_1050_1050;
  *(ushort *)(param_1 + -0x6) = param_8;
  *(undefined2 *)(param_1 + -0x8) = 0x30c5;
  exit_1000_25f2(*(ushort *)(param_1 + -0x8),*(ushort *)(param_1 + -0x6),*(int *)(param_1 + -0x4),0x214,param_7,param_8,
                 param_9);
  *(uint **)(param_1 + -0x6) = puVar6;
  *(uint *)(param_1 + -0x8) = param_6 ^ (uint)puVar6;
  *(undefined2 *)(param_1 + -0xc) = 0x0;
  *(undefined *)(param_1 + -0x9) = 0x0;
  pcVar3 = *(char **)(param_1 + 0x8);
  cVar2 = *pcVar3;
  *(int *)(param_1 + 0x8) = (int)pcVar3 + 0x1;
  *(char *)(param_1 + -0x6) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < *(int *)(param_1 + -0xc))) {
    if ((byte)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(byte *)(ulong)((byte)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(byte *)(ulong)((byte)(bVar4 * '\b' + *(char *)(param_1 + -0x9)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x9) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return *(ushort *)(param_1 + -0xc);
}



ushort __cdecl16far
sys_1000_30b4(ushort param_1,ushort param_2,byte *param_3,int param_4,uint param_5,ushort param_6,ushort param_7,
             ushort param_8)

{
  byte bVar1;
  byte bVar2;
  ushort uVar3;
  int iVar3;
  undefined2 uVar4;
  
  iVar3 = param_4 + 0x1;
  uVar4 = SUB42(&USHORT_1050_1050,0x0);
  exit_1000_25f2(0x30c5,param_7,(int)&USHORT_1050_1050,0x214,param_6,param_7,param_8);
  bVar1 = *param_3;
  if (bVar1 == 0x0) {
    return 0x0;
  }
  if ((byte)(bVar1 - 0x20) < 0x59) {
    bVar2 = *(byte *)(ulong)((byte)(bVar1 - 0x20) + 0x5ffe) & 0xf;
  }
  else {
    bVar2 = 0x0;
  }
                    // WARNING: Could not emulate address calculation at 0x10003101
                    // WARNING: Treating indirect jump as call
  uVar3 = (**(code **)((char)(*(byte *)(ulong)((byte)(bVar2 * '\b') + 0x5ffe) >> 0x4) * 0x2 + 0x30a4))
                    (param_5 & 0xff00 | (uint)bVar1,uVar4,iVar3);
  return uVar3;
}



// WARNING (jumptable): Unable to track spacebase fully for stack

ushort __cdecl16far pass1_1000_3113(ushort param_1,ushort param_2)

{
  char cVar1;
  char *pcVar2;
  byte bVar3;
  ushort uVar4;
  
  pass1_1000_3552(0x1,param_1,param_2);
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  *(int *)(param_1 + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(param_1 - 0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < *(int *)(param_1 - 0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)(ulong)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)(ulong)((byte)(bVar3 * '\b' + *(char *)(param_1 - 0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 - 0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return *(ushort *)(param_1 - 0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

ushort __cdecl16far pass1_1000_311e(int param_1,undefined2 param_2)

{
  char cVar1;
  char *pcVar2;
  byte bVar3;
  ushort uVar4;
  
  *(undefined2 *)(param_1 + -0x12) = 0x0;
  *(undefined2 *)(param_1 + -0xc) = 0x0;
  *(undefined2 *)(param_1 + -0x14) = 0x0;
  *(undefined2 *)(param_1 + -0x6) = 0x20;
  *(undefined2 *)(param_1 + -0xe) = 0xffff;
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  *(int *)(param_1 + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)(ulong)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)(ulong)((byte)(bVar3 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return *(ushort *)(param_1 + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

ushort __cdecl16far pass1_1000_3134(int param_1,ushort param_2)

{
  byte *pbVar1;
  char cVar2;
  char *pcVar3;
  byte bVar4;
  ushort uVar5;
  
  cVar2 = *(char *)(param_1 + -0x4);
  if (cVar2 == '-') {
    pbVar1 = (byte *)(param_1 + -0x6);
    *pbVar1 = *pbVar1 | 0x4;
  }
  else {
    if (cVar2 == '+') {
      pbVar1 = (byte *)(param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x1;
    }
    else {
      if (cVar2 == ' ') {
        pbVar1 = (byte *)(param_1 + -0x6);
        *pbVar1 = *pbVar1 | 0x2;
      }
      else {
        if (cVar2 == '#') {
          pbVar1 = (byte *)(param_1 + -0x6);
          *pbVar1 = *pbVar1 | 0x80;
        }
        else {
          pbVar1 = (byte *)(param_1 + -0x6);
          *pbVar1 = *pbVar1 | 0x8;
        }
      }
    }
  }
  pcVar3 = *(char **)(param_1 + 0xa);
  cVar2 = *pcVar3;
  *(int *)(param_1 + 0xa) = (int)pcVar3 + 0x1;
  *(char *)(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(byte *)(ulong)((byte)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(byte *)(ulong)((byte)(bVar4 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return *(ushort *)(param_1 + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

ushort __cdecl16far pass1_1000_3168(int param_1,ushort param_2)

{
  byte *pbVar1;
  char cVar2;
  char *pcVar3;
  byte bVar4;
  ushort uVar5;
  
  cVar2 = *(char *)(param_1 + -0x4);
  if (cVar2 == '*') {
    uVar5 = pass1_1000_34cf(param_1,param_2);
    if ((int)uVar5 < 0x0) {
      uVar5 = -uVar5;
      pbVar1 = (byte *)(param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x4;
    }
  }
  else {
    uVar5 = *(int *)(param_1 + -0xc) * 0xa + (uint)(byte)(cVar2 - 0x30);
  }
  *(ushort *)(param_1 + -0xc) = uVar5;
  pcVar3 = *(char **)(param_1 + 0xa);
  cVar2 = *pcVar3;
  *(int *)(param_1 + 0xa) = (int)pcVar3 + 0x1;
  *(char *)(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(byte *)(ulong)((byte)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(byte *)(ulong)((byte)(bVar4 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return *(ushort *)(param_1 + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

ushort __cdecl16far pass1_1000_3194(int param_1,undefined2 param_2)

{
  char cVar1;
  char *pcVar2;
  byte bVar3;
  ushort uVar4;
  
  *(undefined2 *)(param_1 + -0xe) = 0x0;
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  *(int *)(param_1 + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)(ulong)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)(ulong)((byte)(bVar3 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return *(ushort *)(param_1 + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

ushort __cdecl16far pass1_1000_319c(int param_1,ushort param_2)

{
  char cVar1;
  char *pcVar2;
  byte bVar3;
  ushort uVar4;
  
  cVar1 = *(char *)(param_1 + -0x4);
  if (cVar1 == '*') {
    uVar4 = pass1_1000_34cf(param_1,param_2);
    if ((int)uVar4 < 0x0) {
      uVar4 = 0xffff;
    }
  }
  else {
    uVar4 = *(int *)(param_1 + -0xe) * 0xa + (uint)(byte)(cVar1 - 0x30);
  }
  *(ushort *)(param_1 + -0xe) = uVar4;
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  *(int *)(param_1 + 0xa) = (int)pcVar2 + 0x1;
  *(char *)(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)(ulong)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)(ulong)((byte)(bVar3 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return *(ushort *)(param_1 + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

ushort __cdecl16far pass1_1000_31c5(int param_1,ushort param_2)

{
  byte *pbVar1;
  char cVar2;
  char *pcVar3;
  byte bVar4;
  ushort uVar5;
  
  cVar2 = *(char *)(param_1 + -0x4);
  if (cVar2 == 'l') {
    pbVar1 = (byte *)(param_1 + -0x6);
    *pbVar1 = *pbVar1 | 0x10;
  }
  else {
    if (cVar2 == 'F') {
      pbVar1 = (byte *)(param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x20;
    }
    else {
      if (cVar2 == 'N') {
        pbVar1 = (byte *)(param_1 + -0x5);
        *pbVar1 = *pbVar1 | 0x10;
      }
      else {
        if (cVar2 == 'L') {
          pbVar1 = (byte *)(param_1 + -0x5);
          *pbVar1 = *pbVar1 | 0x4;
        }
        else {
          pbVar1 = (byte *)(param_1 + -0x5);
          *pbVar1 = *pbVar1 | 0x8;
        }
      }
    }
  }
  pcVar3 = *(char **)(param_1 + 0xa);
  cVar2 = *pcVar3;
  *(int *)(param_1 + 0xa) = (int)pcVar3 + 0x1;
  *(char *)(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < *(int *)(param_1 + -0xa))) {
    if ((byte)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(byte *)(ulong)((byte)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(byte *)(ulong)((byte)(bVar4 * '\b' + *(char *)(param_1 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return *(ushort *)(param_1 + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

ushort __cdecl16far pass1_1000_31f7(ushort param_1,int param_2,ushort *param_3,int param_4,ushort param_5)

{
  int *piVar1;
  byte *pbVar2;
  ushort *puVar3;
  char cVar4;
  char *pcVar5;
  byte bVar6;
  ushort uVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  char *pcVar11;
  uint uVar12;
  ushort *puVar13;
  char *pcVar14;
  bool bVar15;
  ulong uVar16;
  
  cVar4 = *(char *)(param_2 + -0x4);
  if ((cVar4 == 'd') || (cVar4 == 'i')) {
    pbVar2 = (byte *)(param_2 + -0x6);
    *pbVar2 = *pbVar2 | 0x40;
LAB_1000_3399:
    *(undefined *)(param_2 + -0x8) = 0xa;
LAB_1000_33d4:
    if ((*(byte *)(param_2 + -0x6) & 0x10) == 0x0) {
      uVar7 = pass1_1000_34cf(param_2,param_5);
      if ((*(byte *)(param_2 + -0x6) & 0x40) == 0x0) {
        uVar16 = (ulong)uVar7;
      }
      else {
        uVar16 = SEXT24((int)uVar7);
      }
    }
    else {
      uVar16 = pass1_1000_34d8(param_2,param_5);
    }
    if (((*(byte *)(param_2 + -0x6) & 0x40) != 0x0) && ((long)uVar16 < 0x0)) {
      pbVar2 = (byte *)(param_2 + -0x5);
      *pbVar2 = *pbVar2 | 0x1;
      uVar16 = CONCAT22(-((int)(uVar16 >> 0x10) + (uint)((int)uVar16 != 0x0)),-(int)uVar16);
    }
    if (*(int *)(param_2 + -0xe) < 0x0) {
      *(undefined2 *)(param_2 + -0xe) = 0x1;
    }
    else {
      pbVar2 = (byte *)(param_2 + -0x6);
      *pbVar2 = *pbVar2 & 0xf7;
    }
    if (uVar16 == 0x0) {
      *(undefined2 *)(param_2 + -0x12) = 0x0;
    }
    pcVar11 = (char *)(uint)*(byte *)(param_2 + -0x8);
    pass1_1000_356e((uint)uVar16,(uint)pcVar11,(uint)(uVar16 >> 0x10),param_2,*(int *)(param_2 + -0xe),
                    (byte *)(param_2 + -0x17),param_5,param_5);
    if (((*(byte *)(param_2 + -0x5) & 0x2) != 0x0) && ((pcVar11 == (char *)0x0 || (*(byte *)(param_2 + -0x17) != 0x30)))
       ) {
      *(undefined *)(param_2 + -0x18) = 0x30;
      pcVar11 = pcVar11 + 0x1;
    }
  }
  else {
    if (cVar4 == 'u') goto LAB_1000_3399;
    if (cVar4 == 'X') {
      *(undefined *)(param_2 + -0x3) = 0x7;
LAB_1000_33a9:
      if ((*(byte *)(param_2 + -0x6) & 0x80) != 0x0) {
        *(undefined2 *)(param_2 + -0x12) = 0x2;
        *(undefined *)(param_2 + -0x10) = 0x30;
        *(char *)(param_2 + -0xf) = *(char *)(param_2 + -0x3) + 'Q';
      }
      *(undefined *)(param_2 + -0x8) = 0x10;
      goto LAB_1000_33d4;
    }
    if (cVar4 == 'x') {
      *(undefined *)(param_2 + -0x3) = 0x27;
      goto LAB_1000_33a9;
    }
    if (cVar4 == 'o') {
      if ((*(byte *)(param_2 + -0x6) & 0x80) != 0x0) {
        pbVar2 = (byte *)(param_2 + -0x5);
        *pbVar2 = *pbVar2 | 0x2;
      }
      *(undefined *)(param_2 + -0x8) = 0x8;
      goto LAB_1000_33d4;
    }
    if (cVar4 == 'c') {
      uVar7 = pass1_1000_34cf(param_2,param_5);
      *(undefined *)(param_2 + -0x216) = (char)uVar7;
      pcVar11 = (char *)0x1;
    }
    else {
      if (cVar4 == 's') {
        pass1_1000_34e6(param_1,param_2,param_5);
        if ((param_3 != (ushort *)0x0) || (pcVar11 = DAT_1050_605d, param_4 != 0x0)) {
          iVar10 = *(int *)(param_2 + -0xe);
          puVar13 = param_3;
          if (iVar10 != 0x0) {
            bVar15 = true;
            do {
              if (iVar10 == 0x0) break;
              iVar10 = iVar10 + -0x1;
              puVar3 = puVar13;
              puVar13 = (ushort *)((int)puVar13 + 0x1);
              bVar15 = *(char *)puVar3 == '\0';
            } while (!bVar15);
            if (bVar15) {
              puVar13 = (ushort *)((int)puVar13 + -0x1);
            }
          }
          pcVar11 = (char *)((int)puVar13 - (int)param_3);
        }
      }
      else {
        if (cVar4 == 'n') {
          pass1_1000_34e6(param_1,param_2,param_5);
          *param_3 = *(ushort *)(param_2 + -0xa);
          if ((*(byte *)(param_2 + -0x6) & 0x10) != 0x0) {
            param_3[0x1] = 0x0;
          }
          goto LAB_1000_30cf;
        }
        if (cVar4 == 'p') {
          if ((*(byte *)(param_2 + -0x6) & 0x30) == 0x0) {
            uVar7 = pass1_1000_34cf(param_2,param_5);
            uVar16 = (ulong)uVar7;
          }
          else {
            uVar16 = pass1_1000_34d8(param_2,param_5);
            uVar12 = (uint)(uVar16 >> 0x10);
            if ((*(byte *)(param_2 + -0x5) & 0x18) == 0x0) {
              *(undefined *)(param_2 + -0x3) = 0x7;
              pass1_1000_356e((uint)uVar16,0x10,0x0,param_2,0x4,(byte *)(param_2 + -0x20e),param_5,param_5);
              pass1_1000_356e(uVar12,0x10,0x0,param_2,0x4,(byte *)(param_2 + -0x213),param_5,param_5);
              *(undefined *)(param_2 + -0x212) = 0x3a;
              pcVar11 = (char *)0x9;
              goto LAB_1000_3444;
            }
          }
          *(undefined *)(param_2 + -0x3) = 0x7;
          pass1_1000_356e((uint)uVar16,0x10,0x0,param_2,0x4,(byte *)(param_2 + -0x213),param_5,param_5);
          pcVar11 = (char *)0x4;
        }
        else {
          if ((cVar4 == 'E') || (cVar4 == 'G')) {
            piVar1 = (int *)(param_2 + -0x14);
            *piVar1 = *piVar1 + 0x1;
          }
          pbVar2 = (byte *)(param_2 + -0x6);
          *pbVar2 = *pbVar2 | 0x40;
          bVar6 = *(byte *)(param_2 + -0x4) | 0x20;
          iVar10 = *(int *)(param_2 + -0xe);
          if (iVar10 < 0x1) {
            if (iVar10 == 0x0) {
              if (bVar6 == 0x67) {
                *(undefined2 *)(param_2 + -0xe) = 0x1;
              }
            }
            else {
              *(undefined2 *)(param_2 + -0xe) = 0x6;
            }
          }
          pcVar11 = (char *)(param_2 + -0x216);
          if ((*(byte *)(param_2 + -0x5) & 0x4) == 0x0) {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_6068)();
            piVar1 = (int *)(param_2 + 0xe);
            *piVar1 = *piVar1 + 0x8;
          }
          else {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_607c)();
            piVar1 = (int *)(param_2 + 0xe);
            *piVar1 = *piVar1 + 0xa;
          }
          if (((*(byte *)(param_2 + -0x6) & 0x80) != 0x0) && (*(int *)(param_2 + -0xe) == 0x0)) {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_6074)();
          }
          if ((bVar6 == 0x67) && ((*(uint *)(param_2 + -0x6) & 0x80) == 0x0)) {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_6070)();
          }
          if (*pcVar11 == '-') {
            pcVar11 = (char *)(param_2 + -0x215);
            pbVar2 = (byte *)(param_2 + -0x5);
            *pbVar2 = *pbVar2 | 0x1;
          }
          iVar10 = -0x1;
          pcVar14 = pcVar11;
          do {
            if (iVar10 == 0x0) break;
            iVar10 = iVar10 + -0x1;
            pcVar5 = pcVar14;
            pcVar14 = pcVar14 + 0x1;
          } while (*pcVar5 != '\0');
          pcVar11 = pcVar14 + (-0x1 - (int)pcVar11);
        }
      }
    }
  }
LAB_1000_3444:
  if ((*(byte *)(param_2 + -0x6) & 0x40) != 0x0) {
    if ((*(byte *)(param_2 + -0x5) & 0x1) == 0x0) {
      if ((*(byte *)(param_2 + -0x6) & 0x1) == 0x0) {
        if ((*(byte *)(param_2 + -0x6) & 0x2) != 0x0) {
          *(undefined *)(param_2 + -0x10) = 0x20;
          *(undefined2 *)(param_2 + -0x12) = 0x1;
        }
      }
      else {
        *(undefined *)(param_2 + -0x10) = 0x2b;
        *(undefined2 *)(param_2 + -0x12) = 0x1;
      }
    }
    else {
      *(undefined *)(param_2 + -0x10) = 0x2d;
      *(undefined2 *)(param_2 + -0x12) = 0x1;
    }
  }
  iVar8 = *(int *)(param_2 + -0xc) - (int)pcVar11;
  iVar10 = *(int *)(param_2 + -0x12);
  iVar9 = iVar8 - iVar10;
  if (iVar8 < iVar10) {
    iVar9 = 0x0;
  }
  if ((*(byte *)(param_2 + -0x6) & 0xc) == 0x0) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
  pass1_1000_3534(*(undefined2 *)(param_2 + -0x12),param_2,param_5);
  if (((*(byte *)(param_2 + -0x6) & 0x8) != 0x0) && ((*(byte *)(param_2 + -0x6) & 0x4) == 0x0)) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
  pass1_1000_3534(pcVar11,param_2,param_5);
  if ((*(byte *)(param_2 + -0x6) & 0x4) != 0x0) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
LAB_1000_30cf:
  pcVar5 = *(char **)(param_2 + 0xa);
  cVar4 = *pcVar5;
  *(int *)(param_2 + 0xa) = (int)pcVar5 + 0x1;
  *(char *)(param_2 + -0x4) = cVar4;
  if ((cVar4 != '\0') && (-0x1 < *(int *)(param_2 + -0xa))) {
    if ((byte)(cVar4 - 0x20U) < 0x59) {
      bVar6 = *(byte *)(ulong)((byte)(cVar4 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar6 = 0x0;
    }
    bVar6 = *(byte *)(ulong)((byte)(bVar6 * '\b' + *(char *)(param_2 + -0x7)) + 0x5ffe) >> 0x4;
    *(byte *)(param_2 + -0x7) = bVar6;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
                    // WARNING: Treating indirect jump as call
    uVar7 = (**(code **)((char)bVar6 * 0x2 + 0x30a4))();
    return uVar7;
  }
  return *(ushort *)(param_2 + -0xa);
}



ushort __cdecl16near pass1_1000_34cf(int param_1,ushort param_2)

{
  ushort uVar1;
  ushort *puVar2;
  
  puVar2 = *(ushort **)(param_1 + 0xe);
  uVar1 = *puVar2;
  *(int *)(param_1 + 0xe) = (int)puVar2 + 0x2;
  return uVar1;
}



ulong __cdecl16near pass1_1000_34d8(int param_1,ushort param_2)

{
  undefined2 uVar1;
  undefined2 uVar2;
  undefined2 *puVar3;
  
  puVar3 = (undefined2 *)*(undefined4 *)(param_1 + 0xe);
  uVar1 = *puVar3;
  uVar2 = *(undefined2 *)((int)puVar3 + 0x2);
  *(int *)(param_1 + 0xe) = (int)puVar3 + 0x4;
  return CONCAT22(uVar2,uVar1);
}



ulong __cdecl16near pass1_1000_34e6(uint param_1,int param_2,ushort param_3)

{
  ushort uVar1;
  ulong uVar2;
  
  if ((*(byte *)(param_2 + -0x6) & 0x20) != 0x0) {
    uVar2 = pass1_1000_34d8(param_2,param_3);
    return uVar2;
  }
  uVar1 = pass1_1000_34cf(param_2,param_3);
  if (uVar1 == 0x0) {
    return (ulong)param_1 << 0x10;
  }
  return CONCAT22(param_1,uVar1);
}



int __cdecl16near pass1_1000_3503(char param_1,uint param_2,int param_3,ushort param_4,ushort param_5,uchar param_6)

{
  int *piVar1;
  char *pcVar2;
  char **ppcVar3;
  uint uVar4;
  int *piVar5;
  ushort uVar6;
  
  ppcVar3 = (char **)*(int **)(param_3 + 0x6);
  uVar6 = (ushort)((ulong)ppcVar3 >> 0x10);
  piVar5 = (int *)ppcVar3;
  piVar1 = piVar5 + 0x2;
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 < 0x0) {
    uVar4 = mem_1000_2bb6((int)param_1,piVar5,param_3,uVar6,param_4,param_5,param_6,param_2);
    if (uVar4 == 0xffff) {
      return -0x1;
    }
  }
  else {
    pcVar2 = *ppcVar3;
    *ppcVar3 = *ppcVar3 + 0x1;
    *pcVar2 = param_1;
  }
  return 0x0;
}



void __cdecl16near pass1_1000_3534(int param_1,int param_2,ushort param_3)

{
  int *piVar1;
  undefined *puVar2;
  uint uVar3;
  undefined2 in_DX;
  undefined *unaff_DI;
  uint uVar4;
  undefined2 unaff_ES;
  undefined2 unaff_CS;
  undefined in_AF;
  
  if (param_1 != 0x0) {
    piVar1 = (int *)(param_2 + -0xa);
    *piVar1 = *piVar1 + param_1;
    uVar4 = 0x0;
    do {
      puVar2 = unaff_DI;
      unaff_DI = unaff_DI + 0x1;
      uVar3 = pass1_1000_3503(*puVar2,in_DX,param_2,unaff_CS,param_3,in_AF);
      uVar4 = uVar4 | uVar3;
      param_1 = param_1 + -0x1;
    } while (param_1 != 0x0);
    if (uVar4 != 0x0) {
      *(undefined2 *)(param_2 + -0xa) = 0xffff;
    }
  }
  return;
}

