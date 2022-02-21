
void __cdecl16far pass1_1000_29af(uint param_1)

{
  pass1_1000_29b5(param_1 & 0xff);
  return;
}



void __cdecl16near pass1_1000_29b5(uint param_1)

{
  char cVar1;
  
  PTR_LOOP_1050_5f88._0_1_ = (byte)param_1;
  cVar1 = (char)(param_1 >> 0x8);
  if (cVar1 != '\0') goto LAB_1000_29d2;
  if ((byte)PTR_LOOP_1050_5f88 < 0x22) {
    if ((byte)PTR_LOOP_1050_5f88 < 0x20) {
      if (0x13 < (byte)PTR_LOOP_1050_5f88) goto LAB_1000_29cc;
    }
    else {
      param_1 = 0x5;
    }
  }
  else {
LAB_1000_29cc:
    param_1 = 0x13;
  }
  cVar1 = *(char *)(ulong)((param_1 & 0xff) + 0x5fd6);
LAB_1000_29d2:
  PTR_LOOP_1050_5f78 = (undefined *)(int)cVar1;
  return;
}



uint16_t __cdecl16far pass1_1000_29dc(uint16_t param_1)

{
  if (___EXPORTEDSTUB != (code)0xb8) {
    return param_1;
  }
  return uRam100029ed;
}



uint16_t __cdecl16far ___EXPORTEDSTUB(void)

{
  return 0x0;
}



ushort * __cdecl16far
pass1_1000_2b02(ushort param_1,ushort param_2,ushort param_3,ushort param_4,byte param_5,uint param_6,int param_7)

{
  ushort *puVar1;
  int iStack2;
  
  iStack2 = param_7 + 0x1;
  puVar1 = pass1_1000_35aa();
  if ((param_6 | (uint)puVar1) == 0x0) {
    puVar1 = (ushort *)0x0;
  }
  else {
    puVar1 = pass1_1000_2d34(param_1,param_2,(byte *)CONCAT22(param_4,param_3),param_5,puVar1,&iStack2);
  }
  return puVar1;
}



void __cdecl16far
pass1_1000_2b3c(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,int param_6)

{
  int iStack2;
  
  iStack2 = param_6 + 0x1;
  pass1_1000_2b02(param_1,param_2,param_3,param_4,0x0,param_5,&iStack2);
  return;
}


void __cdecl16far pass1_1000_2ba0(ushort param_1,ushort param_2,ushort param_3,uchar param_4)

{
  pass1_1000_3024(param_1,param_2,param_3,param_4);
  if ((char)PTR_LOOP_1050_5fc9 != '\0') {
    pass1_1000_3f5c(&stack0xfffe,param_1,param_2,param_3,param_4);
  }
  return;
}


void __cdecl16near pass1_1000_2cb0(ushort *param_1,ushort param_2)

{
  ushort *puVar1;
  byte bVar2;
  
  bVar2 = *(byte *)(param_1 + 0x5);
  if (((bVar2 & 0x83) != 0x0) && ((bVar2 & 0x8) != 0x0)) {
    pass1_1000_16ee(param_1[0x3],param_1[0x4],param_2);
    puVar1 = param_1 + 0x5;
    *(byte *)puVar1 = *(byte *)puVar1 & 0xf7;
    param_1[0x3] = 0x0;
    param_1[0x4] = 0x0;
    *param_1 = 0x0;
    param_1[0x1] = 0x0;
    param_1[0x2] = 0x0;
  }
  return;
}



void __cdecl16far pass1_1000_1f68(void)

{
  PTR_LOOP_1050_5f26 = PTR_LOOP_1050_5f26 + -0x1;
  if ((int)PTR_LOOP_1050_5f26 < 0x0) {
    PTR_LOOP_1050_5f26 = (undefined *)0x0;
  }
  return;
}


char * __cdecl16near pass1_1000_1fd2(int param_1)

{
  if (param_1 == 0x2) {
    return "Out of memory.  Please free some memory, then choose retry.";
  }
  return (char *)CONCAT22(0x1000,param_1 * 0x17 + 0x1c7a);
}


BOOL16 __cdecl16far pass1_1000_1fea(void)

{
  undefined *puVar1;
  bool bVar2;
  
  puVar1 = PTR_LOOP_1050_5f22 + 0x1;
  bVar2 = PTR_LOOP_1050_5f22 == (undefined *)0x0;
  PTR_LOOP_1050_5f22 = puVar1;
  if ((bVar2) && (((uint)PTR_LOOP_1050_5f20 | (uint)PTR_LOOP_1050_5f1e) != 0x0)) {
    PTR_LOOP_1050_5f22 = (undefined *)&PTR_LOOP_1050_0002;
  }
  return 0x1;
}


BOOL16 __stdcall16far pass1_1000_206c(uint param_1,uint param_2)

{
  ushort uVar1;
  
  uVar1 = pass1_1000_21d2(0x2,0x42,param_1,param_2,0x1);
  if ((uVar1 != 0x0) && (*(int *)(param_1 + 0x14) == -0x4153)) {
    return 0x1;
  }
  return 0x0;
}



void __stdcall16far pass1_1000_20a2(uint param_1,uint param_2)

{
  int iVar1;
  undefined2 uVar2;
  uint uVar3;
  uint uVar4;
  undefined2 uVar5;
  uint uVar6;
  uint uVar7;
  uint uStack8;
  uint uStack4;
  
  iVar1 = *(int *)(param_1 + 0x2e);
  uVar2 = *(undefined2 *)(param_1 + 0x30);
  uStack8 = 0x0;
  uVar3 = *(uint *)(iVar1 + 0x4);
  uStack4 = *(uint *)(iVar1 + 0x6);
  uVar7 = 0x0;
  if ((uStack4 | uVar3) != 0x0) {
    while ((uVar6 = uVar3, uVar4 = uStack4, uVar6 != param_1 || (uStack4 != param_2))) {
      uVar3 = *(uint *)(uVar6 + 0x2a);
      uStack4 = *(uint *)(uVar6 + 0x2c);
      uVar7 = uVar6;
      uStack8 = uVar4;
      if ((uStack4 | uVar3) == 0x0) {
        return;
      }
    }
    if ((uStack8 | uVar7) != 0x0) {
      uVar2 = *(undefined2 *)(uVar6 + 0x2c);
      *(undefined2 *)(uVar7 + 0x2a) = *(undefined2 *)(uVar6 + 0x2a);
      *(undefined2 *)(uVar7 + 0x2c) = uVar2;
      return;
    }
    uVar5 = *(undefined2 *)(uVar6 + 0x2c);
    *(undefined2 *)(iVar1 + 0x4) = *(undefined2 *)(uVar6 + 0x2a);
    *(undefined2 *)(iVar1 + 0x6) = uVar5;
  }
  return;
}



ushort __cdecl16far ret_true_1000_2146(void)

{
  return 0x1;
}



void __cdecl16far empty_fn_1000_214a(void)

{
  return;
}

ulong pass1_1000_2242(uint param_1,uint param_2,uint param_3,int param_4,ushort param_5,undefined *param_6)

{
  uint uVar1;
  uint uVar2;
  bool bVar3;
  
  uVar1 = param_2 | param_1;
  while( true ) {
    if (uVar1 == 0x0) {
      return 0x0;
    }
    uVar1 = param_1;
    if (param_2 != 0x0) {
      uVar1 = 0xffff;
    }
    if (CARRY2(param_3,uVar1) != false) {
      uVar1 = -param_3;
    }
    bVar3 = param_1 < uVar1;
    param_1 = param_1 - uVar1;
    param_2 = param_2 - bVar3;
    uVar2 = (*(code *)param_6)(uVar1,param_5,param_3,param_4);
    if (uVar2 != 0x0) break;
    bVar3 = CARRY2(param_3,uVar1);
    param_3 = param_3 + uVar1;
    param_4 = param_4 + (uint)bVar3 * 0x100;
    uVar1 = param_2 | param_1;
  }
  return CONCAT22(param_2 + CARRY2(uVar2,param_1),uVar2 + param_1);
}



BOOL16 pass1_1000_22c0(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  ulong uVar1;
  
  uVar1 = pass1_1000_2242(param_3,param_4,param_1,param_2,param_5,(undefined *)0x1dfa);
  if (uVar1 == 0x0) {
    return 0x1;
  }
  return 0x0;
}

void __cdecl16far pass1_1000_25a8(uint16_t param_1,uint16_t param_2)

{
  pass1_1000_2913(0xfc,param_1,param_2);
  pass1_1000_2913(0xff,param_1,param_2);
  return;
}



BOOL16 __cdecl16near pass1_1000_115c(int param_1,uint *param_2)

{
  byte *pbVar1;
  uint *puVar2;
  uint uVar3;
  uint uVar4;
  uint *puVar5;
  int iVar6;
  uint uStack4;
  
  uVar3 = *param_2 & 0x7ffc;
  uVar4 = param_1 + 0x5U & 0xfffc;
  uVar4 = (uVar4 - 0x8 & ~-(uint)(uVar4 < 0x8)) + 0x8;
  if (uVar3 < uVar4) {
    puVar5 = (uint *)(uVar3 + (int)param_2);
    if (((*(byte *)puVar5 & 0x1) != 0x0) || ((*puVar5 & 0xfffc) + uVar3 < uVar4)) {
      return 0x0;
    }
    if (puVar5 == (uint *)PTR_LOOP_1050_000e) {
      PTR_LOOP_1050_000e = (undefined *)puVar5[0x1];
    }
    *(uint *)(puVar5[0x2] + 0x2) = puVar5[0x1];
    *(uint *)(puVar5[0x1] + 0x4) = puVar5[0x2];
    uStack4 = ((*puVar5 & 0xfffc) + uVar3) - uVar4;
    if (uStack4 < s_version__d__d_1050_0012._0_2_) {
      puVar2 = param_2;
      *puVar2 = *puVar2 + (*puVar5 & 0xfffc);
      pbVar1 = (byte *)((int)puVar5 + (*puVar5 & 0xfffc));
      *pbVar1 = *pbVar1 | 0x2;
      return 0x1;
    }
  }
  else {
    uStack4 = uVar3 - uVar4;
    if (uStack4 < s_version__d__d_1050_0012._0_2_) {
      return 0x1;
    }
    puVar5 = (uint *)(uVar3 + (int)param_2);
    if ((*(byte *)puVar5 & 0x1) == 0x0) {
      uStack4 = uStack4 + (*puVar5 & 0xfffc);
      if (puVar5 == (uint *)PTR_LOOP_1050_000e) {
        PTR_LOOP_1050_000e = (undefined *)puVar5[0x1];
      }
      *(uint *)(puVar5[0x2] + 0x2) = puVar5[0x1];
      *(uint *)(puVar5[0x1] + 0x4) = puVar5[0x2];
    }
    if (*DAT_1050_0004 < uStack4) {
      *DAT_1050_0004 = uStack4;
    }
  }
  *param_2 = *param_2 & 0x8003 | uVar4;
  *(uint *)(uVar4 + (int)param_2) = uStack4 | 0x2;
  iVar6 = uVar4 + (int)param_2;
  *(undefined2 *)(iVar6 + 0x4) = PTR_LOOP_1050_0010;
  *(undefined2 *)(iVar6 + 0x2) = *(undefined2 *)(PTR_LOOP_1050_0010 + 0x2);
  *(int *)(*(int *)(PTR_LOOP_1050_0010 + 0x2) + 0x4) = iVar6;
  *(int *)(PTR_LOOP_1050_0010 + 0x2) = iVar6;
  *(uint *)((byte *)(iVar6 + uStack4) + -0x2) = uStack4;
  pbVar1 = (byte *)(iVar6 + uStack4);
  *pbVar1 = *pbVar1 & 0xfd;
  return 0x1;
}


uchar * __cdecl16far pass1_1000_17e8(uchar *param_1,uchar *param_2)

{
  uchar *puVar1;
  
  puVar1 = PTR_LOOP_1050_5f34;
  PTR_LOOP_1050_5f34 = param_1;
  PTR_LOOP_1050_5f36 = param_2;
  return puVar1;
}


ushort __cdecl16far
pass1_1000_188e(ushort *param_1,uint param_2,uint param_3,uint param_4,ushort param_5,ushort param_6)

{
  ushort uVar1;
  long lVar2;
  
  if ((param_2 | (uint)param_1) == 0x0) {
    uVar1 = pass1_1000_180c(param_3,param_4,param_5);
    return uVar1;
  }
  if (param_3 == 0x0) {
    pass1_1000_18d2((uint)param_1,param_2,param_5);
    return 0x0;
  }
  lVar2 = pass1_1000_0ed4(param_5,param_6,0x0,param_3,0x0,param_1,param_2);
  return (ushort)lVar2;
}


ushort __stdcall16far pass1_1000_1a54(ushort param_1,int param_2,ushort param_3,ushort param_4)

{
  uint uVar1;
  ushort uVar2;
  
  if (*(int *)(param_2 + 0x14) != -0x4153) {
    pass1_1000_1e61(param_4,0xa,0x0,0x0);
    return 0x0;
  }
  uVar1 = pass1_1000_1ab0(param_1);
  if (uVar1 < *(int *)(param_2 + 0x18) + 0x14U) {
    uVar2 = 0x0;
  }
  else {
    uVar2 = *(ushort *)(param_2 + 0x1a);
    *(uint *)(param_2 + 0x1a) = uVar1;
    *(uint *)(param_2 + 0x1c) = uVar1 >> 0x2;
  }
  return uVar2;
}



uint __cdecl16near pass1_1000_1ab0(uint param_1)

{
  uint uVar1;
  uint uVar2;
  
  if (param_1 == 0x2000) {
    return 0x2000;
  }
  if (param_1 < 0xfff0) {
    if (param_1 < 0x1001) {
      return 0x1000;
    }
    uVar1 = 0x2000;
    if (param_1 < 0x2001) {
      do {
        uVar2 = uVar1;
        uVar1 = uVar2 >> 0x1;
      } while (param_1 <= uVar1);
      return uVar2 & 0xfffe;
    }
    while (uVar1 = uVar1 * 0x2, uVar1 != 0x0) {
      if (param_1 <= uVar1) {
        return (uVar1 + 0x10 & -(uint)(uVar1 < 0xfff0)) - 0x10;
      }
    }
  }
  return 0xfff0;
}



BOOL16 __stdcall16far pass1_1000_1afe(UINT16 param_1,UINT32 param_2,UINT16 param_3)

{
  uint uVar1;
  UINT16 unaff_CS;
  
  if (param_1 == 0x0) {
    uVar1 = 0x0;
  }
  else {
    uVar1 = param_1 + 0x1 & 0xfffe;
  }
  if (*(int *)(param_2 + 0x14) == -0x4153) {
    if ((uVar1 < param_1) || (*(int *)(param_2 + 0x1a) - 0x14U < uVar1)) {
      pass1_1000_1e61(unaff_CS,0x3,param_2,param_3);
    }
    else {
      if (*(int *)(param_2 + 0x2) == 0x0) {
        *(uint *)(param_2 + 0x18) = uVar1;
        return 0x1;
      }
    }
    return 0x0;
  }
  pass1_1000_1e61(unaff_CS,0xa,0x0,0x0);
  return 0x0;
}
void __cdecl16near pass1_1000_0000(ushort *param_1,ushort *param_2,uint param_3)

{
  ushort *puVar1;
  ushort *puVar2;
  uint uVar3;
  
                    // Segment:    1
                    // Offset:     00000a20
                    // Length:     55b7
                    // Min Alloc:  55b7
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  for (uVar3 = param_3 >> 0x1; uVar3 != 0x0; uVar3 = uVar3 - 0x1) {
    puVar2 = param_2;
    param_2 = param_2 + 0x1;
    puVar1 = param_1;
    param_1 = param_1 + 0x1;
    *puVar2 = *puVar1;
  }
  return;
}


void __cdecl16near pass1_1000_0368(ushort param_1,ushort param_2,ushort param_3)

{
  ushort *puVar1;
  
  if (*(ushort *)(param_1 + 0x4) == param_1) {
    *(undefined2 *)(param_3 + param_2 * 0x2) = 0x0;
  }
  else {
    *(undefined2 *)(*(int *)(param_1 + 0x6) + 0x4) = *(undefined2 *)(param_1 + 0x4);
    *(undefined2 *)(*(int *)(param_1 + 0x4) + 0x6) = *(undefined2 *)(param_1 + 0x6);
    puVar1 = (ushort *)(param_2 * 0x2 + param_3);
    if (*puVar1 == param_1) {
      *puVar1 = *(ushort *)(param_1 + 0x4);
    }
  }
  *(undefined2 *)(param_1 + 0x4) = *(undefined2 *)(param_3 + 0xa);
  *(ushort *)(param_3 + 0xa) = param_1;
  return;
}

void __cdecl16near pass1_1000_05b4(byte param_1,int param_2)

{
  *(undefined2 *)(param_2 + 0xa) = 0x1;
  *(undefined2 *)(param_2 + 0x8) = 0x668;
  *(byte *)(param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
  *(undefined2 *)(param_2 + 0x10) = 0x0;
  *(undefined2 *)(param_2 + 0xe) = 0x0;
  return;
}


UINT16 pass1_1000_0782(UINT32 param_1,UINT16 param_2,int param_3)

{
  undefined2 in_stack_00000004;
  
  *(undefined2 *)(param_3 + 0xe) = 0x0;
  *(int *)(param_3 + 0x10) = param_3 + 0x14;
  *(undefined2 *)(param_3 + 0x8) = 0x9a0;
  pass1_1000_07ac(*(undefined2 *)(param_1 + 0x18),param_2,param_3);
  return 0x1;
}



void __cdecl16near pass1_1000_07ac(uint param_1,int param_2,int param_3)

{
  undefined2 *puVar1;
  int iVar2;
  uint uVar3;
  
  puVar1 = *(undefined2 **)(param_3 + 0x10);
  *(undefined2 **)(param_3 + 0xe) = puVar1;
  uVar3 = param_2 + (param_3 - (int)puVar1);
  iVar2 = (int)puVar1 + (uVar3 - uVar3 % param_1);
  *(int *)(param_3 + 0x10) = iVar2;
  while (puVar1 < (undefined2 *)(iVar2 - param_1)) {
    *puVar1 = (undefined2 *)((int)puVar1 + param_1);
    puVar1 = (undefined2 *)((int)puVar1 + param_1);
  }
  *puVar1 = 0x0;
  return;
}

uint __cdecl16near pass1_1000_09ca(int param_1,UINT32 *param_2)

{
  uint *puVar1;
  int iVar2;
  undefined4 uVar3;
  UINT16 *puVar4;
  
  puVar1 = param_2 + 0xa;
  puVar4 = (UINT16 *)(((int)param_2 + (param_1 - (int)puVar1) + -0x6 & 0xfffcU) + (int)puVar1);
  *puVar4 = 0x1;
  param_2[0x7] = (UINT32)puVar1;
  puVar4[0x2] = (UINT16)puVar4;
  puVar4[0x1] = (UINT16)puVar4;
  param_2[0x8] = (UINT32)puVar4;
  if ((*(byte *)(param_2 + 0x6) & 0x7) == 0x2) {
    param_2[0x9] = 0x8;
  }
  else {
    uVar3 = *(undefined4 *)param_2;
    iVar2 = *(int *)((int)uVar3 + 0x18);
    param_2[0x9] = (iVar2 - 0x5U & ~-(uint)(iVar2 + 0x3U < 0x8)) + 0x8;
  }
  puVar4[-0x1] = (int)puVar4 - (int)puVar1;
  *puVar1 = (int)puVar4 - (int)puVar1 | 0x2;
  param_2[0xc] = (UINT32)puVar4;
  param_2[0xb] = puVar4[0x1];
  *(uint **)(puVar4[0x1] + 0x4) = puVar1;
  puVar4[0x1] = (UINT16)puVar1;
  param_2[0x4] = 0xe08;
  return *puVar1 & 0xfffc;
}



ulong __cdecl16near pass1_1000_0c32(ushort param_1,ushort param_2,ushort param_3)

{
  uint *puVar1;
  byte *pbVar2;
  int *piVar3;
  undefined4 uVar4;
  uint uVar5;
  uint *puVar6;
  int iVar7;
  uint *puVar8;
  undefined2 uVar9;
  ushort uStack14;
  uint *puStack8;
  uint uStack6;
  
  puVar8 = *(uint **)(param_3 + 0xe);
  uStack6 = 0x0;
  puVar6 = puVar8;
  while( true ) {
    do {
      uVar5 = *puVar6;
      if (param_1 <= uVar5) {
        uVar5 = (uVar5 & 0xfffc) - param_1;
        puVar1 = (uint *)(param_3 + 0x12);
        puStack8 = puVar6;
        if (*puVar1 < uVar5 || *puVar1 == uVar5) {
          uStack14 = param_1;
          if ((param_2 & 0x6) == 0x0) {
            puStack8 = (uint *)(uVar5 + (int)puVar6);
            puStack8[-0x1] = uVar5;
            *puVar6 = uVar5 | 0x2;
            puVar8 = (uint *)puVar6[0x1];
            pbVar2 = (byte *)((int)puStack8 + param_1);
            *pbVar2 = *pbVar2 | 0x2;
            *puStack8 = param_1 | 0x1;
          }
          else {
            *puVar6 = param_1 & 0xff00 | *(byte *)puVar6 & 0x2 | param_1 & 0xff | 0x1;
            *(uint *)(puVar6[0x2] + 0x2) = puVar6[0x1];
            *(uint *)(puVar6[0x1] + 0x4) = puVar6[0x2];
            puVar8 = (uint *)((int)puVar6 + param_1);
            *(uint *)((int)puVar8 + (uVar5 - 0x2)) = uVar5;
            *puVar8 = uVar5 | 0x2;
            uVar5 = *(uint *)(param_3 + 0x10);
            puVar8[0x2] = uVar5;
            puVar8[0x1] = *(uint *)(uVar5 + 0x2);
            *(uint **)(*(int *)(uVar5 + 0x2) + 0x4) = puVar8;
            *(uint **)(uVar5 + 0x2) = puVar8;
          }
        }
        else {
          puVar8 = (uint *)puVar6[0x1];
          *(uint **)(puVar6[0x2] + 0x2) = puVar8;
          *(uint *)(puVar6[0x1] + 0x4) = puVar6[0x2];
          puVar1 = puVar6;
          *(byte *)puVar1 = *(byte *)puVar1 | 0x1;
          uStack14 = *puVar6 & 0xfffc;
          *(uint *)((int)puVar6 + uStack14) = *(uint *)((int)puVar6 + uStack14) | 0x2;
        }
        *(uint **)(param_3 + 0xe) = puVar8;
        if ((param_2 & 0x1) != 0x0) {
          puVar6 = puStack8;
          for (uVar5 = uStack14 - 0x2 >> 0x1; puVar6 = puVar6 + 0x1, uVar5 != 0x0; uVar5 = uVar5 - 0x1) {
            *puVar6 = 0x0;
          }
          if ((uStack14 - 0x2 & 0x1) != 0x0) {
            *(undefined *)puVar6 = 0x0;
          }
        }
        if (((param_2 & 0x2) != 0x0) && (puVar8[0x1] == puVar8[0x2])) {
          **(uint **)(param_3 + 0x4) = **(uint **)(*(int *)(param_3 + 0x10) + 0x2) & 0xfffc;
          uVar4 = *(undefined4 *)(param_3 + 0x4);
          pbVar2 = (byte *)((int)uVar4 + 0x3);
          *pbVar2 = *pbVar2 | 0x80;
        }
        piVar3 = (int *)(param_3 + 0xa);
        *piVar3 = *piVar3 + 0x1;
        return CONCAT22(0x1050,puStack8 + 0x1);
      }
      if (uStack6 < uVar5) {
        uStack6 = uVar5;
      }
      puVar6 = (uint *)puVar6[0x1];
    } while (puVar6 != puVar8);
    if (((param_2 & 0x2) == 0x0) || ((param_2 & 0x40) != 0x0)) break;
    uVar4 = *(undefined4 *)param_3;
    uVar9 = (undefined2)((ulong)uVar4 >> 0x10);
    iVar7 = (int)uVar4;
    if (*(int *)(iVar7 + 0x34) == 0x0) break;
    uStack6 = (**(code **)(iVar7 + 0x34))();
    if ((uStack6 < param_1) || (puVar6 = *(uint **)(param_3 + 0xe), puVar6 == (uint *)0x0)) break;
  }
  **(uint **)(param_3 + 0x4) = uStack6 & 0xfffc;
  return 0x0;
}