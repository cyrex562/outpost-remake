
int * pass1_1000_25d2(int param_1,int param_2,ushort param_3,ushort param_4,ushort param_5,undefined *param_6)

{
  uint *puVar1;
  int *piVar2;
  char *pcVar3;
  undefined *puVar4;
  uint uVar5;
  int *piVar6;
  LPCSTR str;
  int *piVar7;
  char *pcVar8;
  int iVar9;
  
  puVar4 = (undefined *)(param_2 + 0x1U & 0xfffe);
  if ((puVar4 < &param_1) &&
     (uVar5 = -((int)puVar4 - (int)&param_1), puVar1 = (uint *)&PTR_LOOP_1050_000a, *puVar1 < uVar5 || *puVar1 == uVar5)
     ) {
    puVar1 = (uint *)&PTR_LOOP_1050_000c;
    if (uVar5 <= *puVar1 && *puVar1 != uVar5) {
      *(uint *)&PTR_LOOP_1050_000c = uVar5;
    }
                    // WARNING: Could not recover jumptable at 0x100025f0. Too many branches
                    // WARNING: Treating indirect jump as call
    piVar6 = (int *)(*(code *)param_6)();
    return piVar6;
  }
  iVar9 = 0x0;
  pass1_1000_25a8(param_3,param_4);
  pass1_1000_2913(iVar9,param_3,param_4);
  str = poss_str_op_1000_28dc(0x0);
  if (str != (PCHAR)0x0) {
    iVar9 = 0x9;
    if (*str == 'M') {
      iVar9 = 0xf;
    }
    str = str + iVar9;
    iVar9 = 0x22;
    pcVar8 = str;
    do {
      if (iVar9 == 0x0) break;
      iVar9 = iVar9 + -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar8[-0x1] = '\0';
  }
  FatalAppExit16(param_4,str);
  FatalExit();
  piVar6 = (int *)&PTR_LOOP_1050_63fe;
  do {
    piVar2 = piVar6;
    piVar6 = piVar6 + 0x1;
    iVar9 = *piVar2;
    piVar7 = piVar6;
    if ((iVar9 == param_1) || (piVar7 = (int *)(iVar9 + 0x1), piVar7 == (int *)0x0)) {
      return piVar7;
    }
    iVar9 = -0x1;
    do {
      if (iVar9 == 0x0) break;
      iVar9 = iVar9 + -0x1;
      piVar2 = piVar6;
      piVar6 = (int *)((int)piVar6 + 0x1);
    } while (*(char *)piVar2 != '\0');
  } while( true );
}



// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING: Unable to track spacebase fully for stack
// WARNING: Variable defined which should be unmapped: param_2
// WARNING: Variable defined which should be unmapped: param_1

int * __cdecl16far
exit_1000_25f2(ushort param_1,ushort param_2,int param_3,int param_4,ushort param_5,ushort param_6,ushort param_7)

{
  int **ppiVar1;
  int *piVar2;
  char *pcVar3;
  undefined *puVar4;
  int *piVar5;
  uint16_t uVar6;
  LPCSTR str;
  int iVar7;
  int *piVar8;
  char *pcVar9;
  
  puVar4 = (undefined *)(param_4 + 0x1U & 0xfffe);
  if ((puVar4 < &param_3) &&
     (piVar5 = (int *)-((int)puVar4 - (int)&param_3), ppiVar1 = (int **)&PTR_LOOP_1050_000a,
     *ppiVar1 < piVar5 || *ppiVar1 == piVar5)) {
    ppiVar1 = (int **)&PTR_LOOP_1050_000c;
    if (piVar5 <= *ppiVar1 && *ppiVar1 != piVar5) {
      *(int **)&PTR_LOOP_1050_000c = piVar5;
    }
    piVar5[-0x1] = param_2;
    piVar5[-0x2] = param_1;
    return piVar5;
  }
  uVar6 = pass1_1000_29dc(param_7);
  if (*(int *)0x5fce != -0x1) {
                    // WARNING: Could not recover jumptable at 0x10002622. Too many branches
                    // WARNING: Treating indirect jump as call
    piVar5 = (int *)(*(code *)(ulong)*(uint *)0x5fce)();
    return piVar5;
  }
  pass1_1000_25a8(param_5,param_6);
  pass1_1000_2913(0x0,param_5,param_6);
  str = poss_str_op_1000_28dc(0x0);
  if (str != (PCHAR)0x0) {
    iVar7 = 0x9;
    if (*str == 'M') {
      iVar7 = 0xf;
    }
    str = str + iVar7;
    iVar7 = 0x22;
    pcVar9 = str;
    do {
      if (iVar7 == 0x0) break;
      iVar7 = iVar7 + -0x1;
      pcVar3 = pcVar9;
      pcVar9 = pcVar9 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar9[-0x1] = '\0';
  }
  FatalAppExit16(param_6,str);
  FatalExit();
  piVar5 = (int *)&PTR_LOOP_1050_63fe;
  do {
    piVar2 = piVar5;
    piVar5 = piVar5 + 0x1;
    iVar7 = *piVar2;
    piVar8 = piVar5;
    if ((iVar7 == param_3) || (piVar8 = (int *)(iVar7 + 0x1), piVar8 == (int *)0x0)) {
      return piVar8;
    }
    iVar7 = -0x1;
    do {
      if (iVar7 == 0x0) break;
      iVar7 = iVar7 + -0x1;
      piVar2 = piVar5;
      piVar5 = (int *)((int)piVar5 + 0x1);
    } while (*(char *)piVar2 != '\0');
  } while( true );
}



// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 : 0x1000270c
// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram

void pass1_1000_262c(undefined *param_1,undefined *param_2,CHAR *param_3,HINSTANCE16 param_4)

{
  char *pcVar1;
  char cVar2;
  undefined2 uVar3;
  undefined *puVar4;
  INT16 IVar5;
  uint16_t uVar6;
  uint uVar7;
  uint uVar8;
  undefined *in_DX;
  int iVar9;
  char **ppcVar10;
  char *pcVar11;
  char *pcVar12;
  char *pcVar13;
  undefined2 unaff_ES;
  undefined2 uVar14;
  undefined *puStack4;
  CHAR *pCStack2;
  
  PTR_LOOP_1050_5fd2 = param_1;
  PTR_LOOP_1050_5fd4 = param_2;
  param_2 = (undefined *)0x263d;
  param_1 = (undefined *)pass1_1000_2950(0x8,in_DX,unaff_ES,param_4);
  pCStack2 = PTR_LOOP_1050_5f4c;
  puStack4 = in_DX;
  PTR_LOOP_1050_5fc2 = param_1;
  PTR_LOOP_1050_5fc4 = in_DX;
  IVar5 = GetModuleFileName16(param_4,(LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x25),(INT16)param_1);
  puStack4[IVar5] = 0x0;
  iVar9 = 0x1;
  PTR_LOOP_1050_5fb8 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
  pcVar11 = (char *)((int)s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
LAB_1000_266c:
  do {
    do {
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
    } while (cVar2 == ' ');
  } while (cVar2 == '\t');
  if ((cVar2 != '\r') && (cVar2 != '\0')) {
    PTR_LOOP_1050_5fb8 = PTR_LOOP_1050_5fb8 + 0x1;
    do {
      pcVar11 = pcVar11 + -0x1;
LAB_1000_267f:
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
      if ((cVar2 == ' ') || (cVar2 == '\t')) goto LAB_1000_266c;
      if ((cVar2 == '\r') || (cVar2 == '\0')) break;
      if (cVar2 == '\"') {
LAB_1000_26b8:
        do {
          while( true ) {
            while( true ) {
              pcVar1 = pcVar11;
              pcVar11 = pcVar11 + 0x1;
              cVar2 = *pcVar1;
              if ((cVar2 == '\r') || (cVar2 == '\0')) goto LAB_1000_26e8;
              if (cVar2 == '\"') goto LAB_1000_267f;
              if (cVar2 == '\\') break;
              iVar9 = iVar9 + 0x1;
            }
            uVar7 = 0x0;
            do {
              pcVar13 = pcVar11;
              uVar7 = uVar7 + 0x1;
              pcVar11 = pcVar13 + 0x1;
              cVar2 = *pcVar13;
            } while (cVar2 == '\\');
            if (cVar2 == '\"') break;
            iVar9 = iVar9 + uVar7;
            pcVar11 = pcVar13;
          }
          iVar9 = iVar9 + (uVar7 >> 0x1) + (uint)((uVar7 & 0x1) != 0x0);
        } while ((uVar7 & 0x1) != 0x0);
        goto LAB_1000_267f;
      }
      if (cVar2 != '\\') {
        iVar9 = iVar9 + 0x1;
        goto LAB_1000_267f;
      }
      uVar7 = 0x0;
      do {
        uVar7 = uVar7 + 0x1;
        pcVar1 = pcVar11;
        pcVar11 = pcVar11 + 0x1;
        cVar2 = *pcVar1;
      } while (cVar2 == '\\');
      if (cVar2 == '\"') {
        iVar9 = iVar9 + (uVar7 >> 0x1) + (uint)((uVar7 & 0x1) != 0x0);
        if ((uVar7 & 0x1) == 0x0) goto LAB_1000_26b8;
        goto LAB_1000_267f;
      }
      iVar9 = iVar9 + uVar7;
    } while( true );
  }
LAB_1000_26e8:
  pCStack2 = (CHAR *)&USHORT_1050_1050;
  iVar9 = -((uint)(PTR_LOOP_1050_5fb8 + (int)(PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + iVar9 + 0x1) & 0xfffe);
  PTR_LOOP_1050_5fba = (undefined *)((int)&param_1 + iVar9);
  pcVar13 = (char *)((int)&param_1 + (int)(PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + iVar9);
  PTR_LOOP_1050_5fbc = param_3;
  *(CHAR **)((int)&pCStack2 + iVar9) = param_3;
  puVar4 = PTR_LOOP_1050_5fc4;
  uVar14 = *(undefined2 *)((int)&pCStack2 + iVar9);
  *(undefined2 *)((int)&param_1 + iVar9) = PTR_LOOP_1050_5fc2;
  *(undefined2 *)((int)&param_2 + iVar9) = puVar4;
  ppcVar10 = (char **)(&stack0x0004 + iVar9);
  *(int *)((int)&pCStack2 + iVar9) = (int)&param_1 + iVar9;
  *(undefined2 *)((int)&puStack4 + iVar9) = (int)s_tile2_bmp_1050_1538;
  *(undefined2 *)(&stack0xfffa + iVar9) = 0x271f;
  uVar6 = pass1_1000_29dc(param_3);
  uVar3 = *(undefined2 *)&PTR_LOOP_1050_5f7e;
  pcVar11 = (char *)((int)s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
LAB_1000_272e:
  do {
    do {
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
    } while (cVar2 == ' ');
  } while (cVar2 == '\t');
  if ((cVar2 == '\r') || (cVar2 == '\0')) {
LAB_1000_27c1:
    *(undefined2 *)((int)&pCStack2 + iVar9) = (int)s_tile2_bmp_1050_1538;
    *(undefined2 *)((int)&puStack4 + iVar9) = 0x27c5;
    uVar6 = pass1_1000_29dc(param_3);
    *ppcVar10 = (char *)0x0;
    ppcVar10[0x1] = (char *)0x0;
                    // WARNING: Could not recover jumptable at 0x100027d2. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(ulong)*(uint *)&PTR_LOOP_1050_5fd2)();
    _PTR_LOOP_1050_5fc2 = CONCAT22(PTR_LOOP_1050_5fc4,PTR_LOOP_1050_5fc2);
    return;
  }
  *ppcVar10 = pcVar13;
  ppcVar10[0x1] = param_3;
  ppcVar10 = ppcVar10 + 0x2;
  do {
    pcVar11 = pcVar11 + -0x1;
LAB_1000_274f:
    pcVar1 = pcVar11;
    pcVar11 = pcVar11 + 0x1;
    cVar2 = *pcVar1;
    if ((cVar2 == ' ') || (cVar2 == '\t')) {
      pcVar1 = pcVar13;
      pcVar13 = pcVar13 + 0x1;
      *pcVar1 = '\0';
      goto LAB_1000_272e;
    }
    if ((cVar2 == '\r') || (cVar2 == '\0')) {
LAB_1000_27be:
      *pcVar13 = '\0';
      goto LAB_1000_27c1;
    }
    pcVar12 = pcVar11;
    if (cVar2 == '\"') {
LAB_1000_278b:
      while( true ) {
        pcVar11 = pcVar12 + 0x1;
        cVar2 = *pcVar12;
        if ((cVar2 == '\r') || (cVar2 == '\0')) goto LAB_1000_27be;
        if (cVar2 == '\"') break;
        if (cVar2 == '\\') {
          uVar7 = 0x0;
          do {
            pcVar12 = pcVar11;
            uVar7 = uVar7 + 0x1;
            pcVar11 = pcVar12 + 0x1;
            cVar2 = *pcVar12;
          } while (cVar2 == '\\');
          if (cVar2 == '\"') {
            for (uVar8 = uVar7 >> 0x1; uVar8 != 0x0; uVar8 = uVar8 - 0x1) {
              pcVar1 = pcVar13;
              pcVar13 = pcVar13 + 0x1;
              *pcVar1 = '\\';
            }
            if ((uVar7 & 0x1) == 0x0) break;
            pcVar1 = pcVar13;
            pcVar13 = pcVar13 + 0x1;
            *pcVar1 = '\"';
            pcVar12 = pcVar11;
          }
          else {
            for (; uVar7 != 0x0; uVar7 = uVar7 - 0x1) {
              pcVar1 = pcVar13;
              pcVar13 = pcVar13 + 0x1;
              *pcVar1 = '\\';
            }
          }
        }
        else {
          pcVar1 = pcVar13;
          pcVar13 = pcVar13 + 0x1;
          *pcVar1 = cVar2;
          pcVar12 = pcVar11;
        }
      }
      goto LAB_1000_274f;
    }
    if (cVar2 != '\\') {
      pcVar1 = pcVar13;
      pcVar13 = pcVar13 + 0x1;
      *pcVar1 = cVar2;
      goto LAB_1000_274f;
    }
    uVar7 = 0x0;
    do {
      uVar7 = uVar7 + 0x1;
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
    } while (cVar2 == '\\');
    if (cVar2 == '\"') {
      for (uVar8 = uVar7 >> 0x1; uVar8 != 0x0; uVar8 = uVar8 - 0x1) {
        pcVar1 = pcVar13;
        pcVar13 = pcVar13 + 0x1;
        *pcVar1 = '\\';
      }
      pcVar12 = pcVar11;
      if ((uVar7 & 0x1) == 0x0) goto LAB_1000_278b;
      pcVar1 = pcVar13;
      pcVar13 = pcVar13 + 0x1;
      *pcVar1 = '\"';
      goto LAB_1000_274f;
    }
    for (; uVar7 != 0x0; uVar7 = uVar7 - 0x1) {
      pcVar1 = pcVar13;
      pcVar13 = pcVar13 + 0x1;
      *pcVar1 = '\\';
    }
  } while( true );
}



void __cdecl16far pass1_1000_27d6(uint16_t *param_1)

{
  int *piVar1;
  char *pcVar2;
  uint16_t *puVar3;
  int *piVar4;
  char cVar5;
  SEGPTR SVar6;
  uint16_t *puVar7;
  uint16_t **ppuVar8;
  int iVar9;
  undefined2 uVar10;
  uint16_t *puVar11;
  int iVar12;
  int *piVar13;
  int *piVar14;
  char *pcVar15;
  int *piVar16;
  bool bVar17;
  uint16_t *puVar18;
  
  SVar6 = GetDOSEnvironment16();
  if (SVar6 != 0x0) {
    param_1 = (uint16_t *)0x0;
  }
  iVar12 = 0x0;
  pcVar15 = (char *)0x0;
  iVar9 = -0x1;
  if (param_1 != (uint16_t *)0x0) {
    cVar5 = *(char *)0x0;
    while (cVar5 != '\0') {
      do {
        if (iVar9 == 0x0) break;
        iVar9 = iVar9 + -0x1;
        pcVar2 = pcVar15;
        pcVar15 = pcVar15 + 0x1;
      } while (*pcVar2 != '\0');
      iVar12 = iVar12 + 0x1;
      pcVar2 = pcVar15;
      pcVar15 = pcVar15 + 0x1;
      cVar5 = *pcVar2;
    }
  }
  uVar10 = 0x9;
  puVar11 = param_1;
  puVar7 = pass1_1000_2950(0x9,param_1,param_1,(int)s_tile2_bmp_1050_1538);
  puVar18 = puVar11;
  ppuVar8 = (uint16_t **)pass1_1000_2950(uVar10,puVar11,param_1,(int)s_tile2_bmp_1050_1538);
  piVar13 = (int *)0x0;
  PTR_LOOP_1050_5fbe = (undefined *)ppuVar8;
  PTR_LOOP_1050_5fc0 = (undefined *)puVar11;
  do {
    if (iVar12 == 0x0) {
      *ppuVar8 = (uint16_t *)0x0;
      ppuVar8[0x1] = (uint16_t *)0x0;
      return;
    }
    bVar17 = *piVar13 == s__C_FILE_INFO__1050_5f5c._0_2_;
    if (bVar17) {
      piVar16 = (int *)s__C_FILE_INFO__1050_5f5c;
      iVar9 = 0x6;
      piVar14 = piVar13;
      do {
        if (iVar9 == 0x0) break;
        iVar9 = iVar9 + -0x1;
        piVar4 = piVar16;
        piVar16 = piVar16 + 0x1;
        piVar1 = piVar14;
        piVar14 = piVar14 + 0x1;
        bVar17 = *piVar1 == *piVar4;
      } while (bVar17);
      if (!bVar17) goto LAB_1000_2867;
    }
    else {
LAB_1000_2867:
      *ppuVar8 = puVar7;
      ppuVar8[0x1] = puVar18;
      ppuVar8 = ppuVar8 + 0x2;
    }
    do {
      piVar1 = piVar13;
      piVar13 = (int *)((int)piVar13 + 0x1);
      cVar5 = *(char *)piVar1;
      puVar3 = puVar7;
      puVar7 = (uint16_t *)((int)puVar7 + 0x1);
      *(char *)puVar3 = cVar5;
    } while (cVar5 != '\0');
    iVar12 = iVar12 + -0x1;
  } while( true );
}



PCHAR __stdcall16far poss_str_op_1000_28dc(int param_1)

{
  int *piVar1;
  PCHAR piVar2;
  int iVar2;
  PCHAR piVar3;
  
  piVar3 = (PCHAR)&PTR_LOOP_1050_63fe;
  do {
    piVar1 = (int *)piVar3;
    piVar3 = (PCHAR)((int)piVar3 + 0x2);
    iVar2 = *piVar1;
    piVar2 = piVar3;
    if ((iVar2 == param_1) || (piVar2 = (PCHAR)(iVar2 + 0x1), piVar2 == (PCHAR)0x0)) {
      return (PCHAR)(int *)piVar2;
    }
    iVar2 = -0x1;
    do {
      if (iVar2 == 0x0) break;
      iVar2 = iVar2 + -0x1;
      piVar1 = (int *)piVar3;
      piVar3 = (PCHAR)((int)piVar3 + 0x1);
    } while (*(char *)piVar1 != '\0');
  } while( true );
}



void __stdcall16far pass1_1000_2913(int param_1,uint16_t param_2,uint16_t param_3)

{
  char *pcVar1;
  char *pcVar2;
  int iVar3;
  
  if (PTR_LOOP_1050_61ec != (undefined *)0x0) {
    pcVar2 = poss_str_op_1000_28dc(param_1);
    if (pcVar2 != (PCHAR)0x0) {
      iVar3 = -0x1;
      do {
        if (iVar3 == 0x0) break;
        iVar3 = iVar3 + -0x1;
        pcVar1 = pcVar2;
        pcVar2 = pcVar2 + 0x1;
      } while (*pcVar1 != '\0');
      pass1_1000_55b1(0x2944,param_2,param_3);
    }
  }
  return;
}



uint16_t * __cdecl16near pass1_1000_2950(int param_1,uint param_2,uint16_t param_3,ushort param_4)

{
  uint16_t *puVar1;
  uint16_t uVar2;
  char *pcVar3;
  undefined *puVar4;
  LPCSTR str;
  int iVar5;
  uint16_t *puVar6;
  uint in_AX;
  uint16_t *puVar7;
  char *pcVar8;
  uint16_t uVar9;
  
  puVar4 = PTR_LOOP_1050_6066;
  PTR_LOOP_1050_6066 = (undefined *)&PTR_LOOP_1050_1000;
  uVar9 = param_3;
  puVar7 = (uint16_t *)mem_1000_167a(in_AX,param_4,param_2);
  PTR_LOOP_1050_6066 = puVar4;
  if ((param_2 | (uint)puVar7) != 0x0) {
    return puVar7;
  }
  iVar5 = param_1;
  pass1_1000_25a8(param_3,param_4);
  pass1_1000_2913(param_1,param_3,param_4);
  str = poss_str_op_1000_28dc(iVar5);
  if (str != (PCHAR)0x0) {
    iVar5 = 0x9;
    if (*str == 'M') {
      iVar5 = 0xf;
    }
    str = str + iVar5;
    iVar5 = 0x22;
    pcVar8 = str;
    do {
      if (iVar5 == 0x0) break;
      iVar5 = iVar5 + -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar8[-0x1] = '\0';
  }
  FatalAppExit16(param_4,str);
  FatalExit();
  puVar7 = (uint16_t *)&PTR_LOOP_1050_63fe;
  do {
    puVar1 = puVar7;
    puVar7 = puVar7 + 0x1;
    uVar2 = *puVar1;
    puVar6 = puVar7;
    if ((uVar2 == uVar9) || (puVar6 = (uint16_t *)(uVar2 + 0x1), puVar6 == (uint16_t *)0x0)) {
      return puVar6;
    }
    iVar5 = -0x1;
    do {
      if (iVar5 == 0x0) break;
      iVar5 = iVar5 + -0x1;
      puVar1 = puVar7;
      puVar7 = (uint16_t *)((int)puVar7 + 0x1);
    } while (*(char *)puVar1 != '\0');
  } while( true );
}



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



ushort __cdecl16far
pass1_1000_2a00(ushort *param_1,int param_2,ushort param_3,ushort param_4,ushort param_5,uchar param_6)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  ushort uVar4;
  ushort uVar5;
  undefined *puStack20;
  char local_10;
  undefined uStack15;
  undefined local_e [0x8];
  ushort uStack6;
  undefined2 local_4;
  int iStack2;
  
  iStack2 = param_2 + 0x1;
  local_4 = SUB42(&USHORT_1050_1050,0x0);
  uVar5 = 0xffff;
  if ((*(byte *)(param_1 + 0x5) & 0x40) != 0x0) {
    *(undefined *)(param_1 + 0x5) = 0x0;
    return 0xffff;
  }
  if ((*(byte *)(param_1 + 0x5) & 0x83) == 0x0) goto LAB_1000_2af2;
  uVar5 = pass1_1000_2fa4((int *)param_1,param_3,param_4,param_5,param_6);
  uStack6 = param_1[0x7a];
  pass1_1000_2cb0(param_1,param_4);
  if (DAT_1050_5f8a < (int)(uint)*(byte *)((int)param_1 + 0xb)) {
    piVar2 = pass1_1000_55b1(0x2a63,param_3,param_4);
    if ((int)piVar2 < 0x0) goto LAB_1000_2a6a;
LAB_1000_2a82:
    bVar1 = false;
  }
  else {
    iVar3 = dos3_call_op_1000_35fe((uint)*(byte *)((int)param_1 + 0xb),&iStack2);
    if (-0x1 < iVar3) goto LAB_1000_2a82;
LAB_1000_2a6a:
    bVar1 = true;
  }
  if (!bVar1) {
    if (uStack6 == 0x0) goto LAB_1000_2af2;
    unk_str_op_1000_3d3e((char *)CONCAT22(param_5,&local_10),(char *)0x10505fea);
    puStack20 = local_e;
    if (local_10 == '\\') {
      puStack20 = &uStack15;
    }
    else {
      pass1_1000_3cea(CONCAT22(param_5,&local_10),0x10505fec);
    }
    pass1_1000_3e82(uStack6,(uchar *)CONCAT22(param_5,puStack20),0xa);
    uVar4 = dos3_call_1000_514e(&iStack2);
    if (uVar4 == 0x0) goto LAB_1000_2af2;
  }
  uVar5 = 0xffff;
LAB_1000_2af2:
  *(undefined *)(param_1 + 0x5) = 0x0;
  return uVar5;
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



ushort __cdecl16far
pass1_1000_2b5c(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,int param_6,ushort param_7,
               ushort param_8)

{
  ushort uVar1;
  ushort uVar2;
  undefined in_AF;
  int iStack2;
  
  iStack2 = param_6 + 0x1;
  uVar1 = pass1_1000_2e74((uint *)param_1,param_7);
  uVar2 = sys_1000_30b4(param_1,(ushort)&USHORT_1050_1050,(byte *)CONCAT22(param_4,param_3),(int)&iStack2,param_1,
                        param_5,param_7,param_8);
  pass1_1000_2f00(uVar1,(int *)param_1,param_5,param_7,param_8,in_AF);
  return uVar2;
}



void __cdecl16far pass1_1000_2ba0(ushort param_1,ushort param_2,ushort param_3,uchar param_4)

{
  pass1_1000_3024(param_1,param_2,param_3,param_4);
  if ((char)PTR_LOOP_1050_5fc9 != '\0') {
    pass1_1000_3f5c(&stack0xfffe,param_1,param_2,param_3,param_4);
  }
  return;
}



uint __cdecl16far
mem_1000_2bb6(uint param_1,int *param_2,int param_3,ushort param_4,ushort param_5,ushort param_6,uchar param_7,
             uint param_8)

{
  int *piVar1;
  int iVar2;
  int *piVar3;
  byte bVar4;
  uchar *puVar5;
  uchar *puVar6;
  uchar *puVar7;
  int iStack2;
  
  piVar3 = param_2;
  iStack2 = param_3 + 0x1;
  bVar4 = *(byte *)(param_2 + 0x5);
  if (((bVar4 & 0x82) != 0x0) && ((bVar4 & 0x40) == 0x0)) {
    param_2[0x2] = 0x0;
    if ((bVar4 & 0x1) != 0x0) {
      if ((bVar4 & 0x10) == 0x0) goto LAB_1000_2c37;
      *param_2 = param_2[0x3];
      bVar4 = bVar4 & 0xfe;
    }
    *(byte *)(param_2 + 0x5) = bVar4 & 0xef | 0x2;
    puVar7 = (uchar *)(uint)*(byte *)((int)param_2 + 0xb);
    if (((bVar4 & 0x8) == 0x0) &&
       (((bVar4 & 0x4) != 0x0 ||
        (((*(byte *)(param_2 + 0x78) & 0x1) == 0x0 &&
         (((PTR_LOOP_1050_61ec != (undefined *)0x0 &&
           (((param_2 == (int *)0x621c || (param_2 == (int *)0x6228)) && ((puVar7[0x5f90] & 0x40) != 0x0)))) ||
          (mem_1000_2ce8(param_2,param_8,param_5), (*(byte *)(piVar3 + 0x5) & 0x8) == 0x0)))))))) {
      puVar5 = mixed_dos3_call_1000_39f2
                         (puVar7,(char *)CONCAT22(param_6,&param_1),(uchar *)((int)&PTR_LOOP_1050_0000 + 0x1),param_4,
                          param_5,param_6,param_7);
      puVar6 = (uchar *)((int)&PTR_LOOP_1050_0000 + 0x1);
    }
    else {
      iVar2 = piVar3[0x3];
      puVar6 = (uchar *)(*piVar3 - iVar2);
      *piVar3 = iVar2 + 0x1;
      piVar3[0x2] = piVar3[0x79] + -0x1;
      if (puVar6 == (uchar *)0x0) {
        puVar5 = (uchar *)0x0;
        if ((puVar7[0x5f90] & 0x20) != 0x0) {
          mixed_dos3_call_1000_3636((uint)puVar7,0x0,0x0,0x2,&iStack2);
          puVar5 = (uchar *)0x0;
          puVar6 = puVar5;
        }
      }
      else {
        puVar5 = mixed_dos3_call_1000_39f2
                           (puVar7,(char *)CONCAT22(piVar3[0x4],piVar3[0x3]),puVar6,param_4,param_5,param_6,param_7);
      }
      **(undefined **)(piVar3 + 0x3) = (char)param_1;
    }
    if (puVar5 == puVar6) {
      return param_1 & 0xff;
    }
  }
LAB_1000_2c37:
  piVar1 = piVar3 + 0x5;
  *(byte *)piVar1 = *(byte *)piVar1 | 0x20;
  return 0xffff;
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



void __cdecl16near mem_1000_2ce8(int *param_1,uint param_2,ushort param_3)

{
  int *piVar1;
  ushort uVar2;
  
  uVar2 = mem_1000_167a(0x200,param_3,param_2);
  if (param_2 == 0x0) {
    piVar1 = param_1 + 0x5;
    *(byte *)piVar1 = *(byte *)piVar1 | 0x4;
    param_1[0x79] = 0x1;
    param_2 = (uint)&USHORT_1050_1050;
    uVar2 = (int)param_1 + 0xf1;
  }
  else {
    piVar1 = param_1 + 0x5;
    *(byte *)piVar1 = *(byte *)piVar1 | 0x8;
    param_1[0x79] = 0x200;
  }
  param_1[0x1] = param_2;
  *param_1 = uVar2;
  param_1[0x4] = param_2;
  param_1[0x3] = uVar2;
  param_1[0x2] = 0x0;
  return;
}



ushort * __cdecl16far
pass1_1000_2d34(ushort param_1,ushort param_2,byte *param_3,byte param_4,ushort *param_5,int param_6)

{
  byte bVar1;
  bool bVar2;
  bool bVar3;
  uint uVar4;
  undefined uStack14;
  byte bStack8;
  undefined uStack6;
  int iStack2;
  
  iStack2 = param_6 + 0x1;
  bStack8 = (byte)PTR_LOOP_1050_6062;
  bVar3 = false;
  bVar1 = *param_3;
  if (bVar1 == 0x77) {
    uVar4 = 0x301;
  }
  else {
    if (0x77 < bVar1) {
      return (ushort *)0x0;
    }
    if (bVar1 != 0x61) {
      if (bVar1 != 0x72) {
        return (ushort *)0x0;
      }
      uVar4 = 0x0;
      uStack6 = 0x1;
      goto LAB_1000_2d6c;
    }
    uVar4 = 0x109;
  }
  uStack6 = 0x2;
LAB_1000_2d6c:
  bVar2 = true;
LAB_1000_2d71:
  param_3 = (byte *)((ulong)param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x1));
  if ((*param_3 == 0x0) || (!bVar2)) {
    uVar4 = mixed_dos3_call_1000_370a(param_1,param_2,uVar4,param_4,0x1a4,&iStack2);
    if ((int)uVar4 < 0x0) {
      return (ushort *)0x0;
    }
    PTR_LOOP_1050_5fee = PTR_LOOP_1050_5fee + 0x1;
    *(undefined *)(param_5 + 0x5) = uStack6;
    param_5[0x1] = 0x0;
    *param_5 = 0x0;
    param_5[0x4] = 0x0;
    param_5[0x3] = 0x0;
    uStack14 = (undefined)uVar4;
    *(undefined *)((int)param_5 + 0xb) = uStack14;
    *(byte *)(param_5 + 0x78) = bStack8;
    param_5[0x2] = 0x0;
    param_5[0x7a] = 0x0;
    return param_5;
  }
  bVar1 = *param_3;
  if (bVar1 == 0x74) {
    if ((uVar4 & 0xc000) == 0x0) {
      uVar4 = uVar4 | 0x4000;
      goto LAB_1000_2d71;
    }
  }
  else {
    if (0x74 < bVar1) goto LAB_1000_2da4;
    if (bVar1 == 0x2b) {
      if ((uVar4 & 0x2) != 0x0) goto LAB_1000_2da4;
      uVar4 = uVar4 & 0xfffe | 0x2;
      uStack6 = 0x80;
      goto LAB_1000_2d71;
    }
    if (bVar1 == 0x62) {
      if ((uVar4 & 0xc000) == 0x0) {
        uVar4 = uVar4 | 0x8000;
        goto LAB_1000_2d71;
      }
    }
    else {
      if (bVar1 != 0x63) {
        if ((bVar1 != 0x6e) || (bVar3)) goto LAB_1000_2da4;
        bVar3 = true;
        bStack8 = bStack8 & 0xbf;
        goto LAB_1000_2d71;
      }
      if (!bVar3) {
        bVar3 = true;
        bStack8 = bStack8 | 0x40;
        goto LAB_1000_2d71;
      }
    }
  }
LAB_1000_2da4:
  bVar2 = false;
  goto LAB_1000_2d71;
}