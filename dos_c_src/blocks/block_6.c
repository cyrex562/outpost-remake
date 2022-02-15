
void __cdecl16near pass1_1000_3552(int param_1,int param_2,ushort param_3)

{
  int *piVar1;
  uint uVar2;
  undefined2 in_DX;
  uint uVar3;
  undefined2 unaff_CS;
  undefined in_AF;
  
  if (param_1 != 0x0) {
    piVar1 = (int *)(param_2 + -0xa);
    *piVar1 = *piVar1 + param_1;
    uVar3 = 0x0;
    do {
      uVar2 = pass1_1000_3503((char)in_DX,in_DX,param_2,unaff_CS,param_3,in_AF);
      uVar3 = uVar3 | uVar2;
      param_1 = param_1 + -0x1;
    } while (param_1 != 0x0);
    if (uVar3 != 0x0) {
      *(undefined2 *)(param_2 + -0xa) = 0xffff;
    }
  }
  return;
}



void __cdecl16near
pass1_1000_356e(uint param_1,uint param_2,uint param_3,int param_4,int param_5,byte *param_6,undefined2 param_7,
               undefined2 param_8)

{
  byte *pbVar1;
  ulong uVar2;
  byte bVar3;
  
  while (((0x0 < param_5 || (param_1 != 0x0)) || (param_3 != 0x0))) {
    uVar2 = (ulong)param_3;
    param_3 = param_3 / param_2;
    uVar2 = uVar2 % (ulong)param_2 << 0x10 | (ulong)param_1;
    param_1 = (uint)(uVar2 / param_2);
    bVar3 = (char)(uVar2 % (ulong)param_2) + 0x30;
    if (0x39 < bVar3) {
      bVar3 = bVar3 + *(char *)(param_4 + -0x3);
    }
    pbVar1 = param_6;
    param_6 = param_6 + -0x1;
    *pbVar1 = bVar3;
    param_5 = param_5 + -0x1;
  }
  return;
}



ushort * __cdecl16far pass1_1000_35aa(void)

{
  ushort *puVar1;
  
  puVar1 = (ushort *)&PTR_LOOP_1050_6210;
  while( true ) {
    if (PTR_LOOP_1050_5ff0 < puVar1) {
      return (ushort *)0x0;
    }
    if ((*(byte *)(puVar1 + 0x5) & 0x83) == 0x0) break;
    puVar1 = puVar1 + 0x6;
  }
  *(undefined *)(puVar1 + 0x5) = 0x0;
  puVar1[0x2] = 0x0;
  puVar1[0x4] = 0x0;
  puVar1[0x3] = 0x0;
  puVar1[0x1] = 0x0;
  *puVar1 = 0x0;
  *(undefined *)((int)puVar1 + 0xb) = 0xff;
  return puVar1;
}



// WARNING: Removing unreachable block (ram,0x10003622)

undefined2 __cdecl16far dos3_call_op_1000_35fe(uint param_1,int param_2)

{
  code *pcVar1;
  undefined2 uVar2;
  bool bVar3;
  
  if (param_1 < DAT_1050_5f8a) {
    bVar3 = false;
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)(param_2 + 0x1);
    if (!bVar3) {
      *(undefined *)(param_1 + 0x5f90) = 0x0;
    }
  }
  else {
    uVar2 = 0x900;
    bVar3 = true;
  }
  if (bVar3) {
    pass1_1000_29b5(uVar2);
    return 0xffff;
  }
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x100036b5)
// WARNING: Removing unreachable block (ram,0x10003681)
// WARNING: Removing unreachable block (ram,0x100036f7)
// WARNING: Removing unreachable block (ram,0x100036d8)

void __cdecl16far mixed_dos3_call_1000_3636(uint param_1,uint param_2,uint param_3,uint param_4,undefined2 param_5)

{
  byte *pbVar1;
  code *pcVar2;
  uint uVar3;
  int iVar4;
  bool bVar5;
  undefined4 uVar6;
  
  if (((param_1 < DAT_1050_5f8a) || (PTR_LOOP_1050_61ec == (undefined *)0x0)) || (0x2 < param_1)) {
    if ((PTR_LOOP_1050_6064 == (undefined *)0x0) || ((param_3 & 0x8000) == 0x0)) goto LAB_1000_36e3;
    if (param_4 == 0x0) goto LAB_1000_369b;
    bVar5 = false;
    pcVar2 = (code *)swi(0x21);
    uVar6 = (*pcVar2)();
    iVar4 = (int)((ulong)uVar6 >> 0x10);
    uVar3 = (uint)uVar6;
    if (bVar5) goto LAB_1000_299d;
    if ((param_4 & 0x2) == 0x0) {
      if (-0x1 < (int)(iVar4 + param_3 + (uint)CARRY2(uVar3,param_2))) {
LAB_1000_36e3:
        bVar5 = false;
        pcVar2 = (code *)swi(0x21);
        uVar3 = (*pcVar2)();
        if (!bVar5) {
          pbVar1 = (byte *)(param_1 + 0x5f90);
          bVar5 = false;
          *pbVar1 = *pbVar1 & 0xfd;
        }
        goto LAB_1000_299d;
      }
    }
    else {
      pcVar2 = (code *)swi(0x21);
      uVar6 = (*pcVar2)(iVar4);
      if (-0x1 < (int)((int)((ulong)uVar6 >> 0x10) + param_3 + (uint)CARRY2((uint)uVar6,param_2))) goto LAB_1000_36e3;
      pcVar2 = (code *)swi(0x21);
      (*pcVar2)();
    }
LAB_1000_369b:
    uVar3 = (uint)s_471_bmp_1050_1600;
  }
  else {
    uVar3 = 0x900;
  }
  bVar5 = true;
LAB_1000_299d:
  if (bVar5) {
    pass1_1000_29b5(uVar3);
  }
  return;
}



// WARNING: Removing unreachable block (ram,0x10003989)
// WARNING: Removing unreachable block (ram,0x100038a1)
// WARNING: Removing unreachable block (ram,0x10003867)
// WARNING: Removing unreachable block (ram,0x10003799)
// WARNING: Removing unreachable block (ram,0x100037ec)
// WARNING: Removing unreachable block (ram,0x10003967)
// WARNING: Removing unreachable block (ram,0x1000391a)
// WARNING: Removing unreachable block (ram,0x100038f2)
// WARNING: Removing unreachable block (ram,0x10003765)
// WARNING: Removing unreachable block (ram,0x100037b7)
// WARNING: Removing unreachable block (ram,0x10003803)
// WARNING: Removing unreachable block (ram,0x1000381c)
// WARNING: Removing unreachable block (ram,0x1000393a)
// WARNING: Removing unreachable block (ram,0x1000384b)
// WARNING: Removing unreachable block (ram,0x1000388b)
// WARNING: Removing unreachable block (ram,0x100038ba)
// WARNING: Removing unreachable block (ram,0x100039b9)

uint __cdecl16far
mixed_dos3_call_1000_370a(ushort param_1,ushort param_2,uint param_3,byte param_4,uint param_5,int param_6)

{
  code *pcVar1;
  uint uVar2;
  int iVar3;
  byte bVar4;
  uint uVar5;
  uint extraout_DX;
  uint uVar6;
  bool bVar7;
  bool bVar8;
  undefined2 uVar9;
  byte bVar10;
  char local_5;
  
  _param_4 = param_5;
  bVar10 = 0x0;
  if (((param_3 & 0x8000) == 0x0) && (((param_3 & 0x4000) != 0x0 || ((DAT_1050_6061 & 0x80) == 0x0)))) {
    bVar10 = 0x80;
  }
  uVar9 = SUB42(&USHORT_1050_1050,0x0);
  bVar7 = false;
  pcVar1 = (code *)swi(0x21);
  uVar5 = param_3;
  uVar2 = (*pcVar1)(bVar10,param_4,(int)&USHORT_1050_1050,param_6 + 0x1);
  if (bVar7) {
    if ((uVar2 == 0x2) && ((uVar5 & 0x100) != 0x0)) {
      bVar7 = false;
      pass1_1000_39e1();
      _param_4 = param_5;
      if ((param_4 != 0x0) || (uVar5 = param_5, (param_3 & 0x2) == 0x0)) {
        uVar5 = 0x0;
      }
LAB_1000_38e3:
      bVar8 = false;
      pcVar1 = (code *)swi(0x21);
      uVar2 = (*pcVar1)();
      if (bVar8) goto LAB_1000_299d;
      if ((param_4 != 0x0) || (uVar6 = uVar2, (param_3 & 0x2) == 0x0)) {
        pcVar1 = (code *)swi(0x21);
        (*pcVar1)();
        bVar8 = false;
        pcVar1 = (code *)swi(0x21);
        uVar2 = (*pcVar1)();
        if (bVar8) goto LAB_1000_299d;
        uVar6 = uVar2;
        if ((!bVar7) && ((_param_4 & 0x1) != 0x0)) {
          uVar5 = (uint)(byte)((byte)uVar5 | 0x1);
          bVar8 = false;
          pcVar1 = (code *)swi(0x21);
          uVar2 = (*pcVar1)();
          if (bVar8) goto LAB_1000_299d;
        }
      }
LAB_1000_3973:
      if ((bVar10 & 0x40) == 0x0) {
        pcVar1 = (code *)swi(0x21);
        (*pcVar1)();
        bVar4 = 0x0;
        if ((uVar5 & 0x1) != 0x0) {
          bVar4 = 0x10;
        }
        if ((param_3 & 0x8) != 0x0) {
          bVar4 = bVar4 | 0x20;
        }
      }
      else {
        bVar4 = 0x0;
      }
      if (uVar6 < *(uint *)&DAT_1050_5f8a) {
        *(byte *)(uVar6 + 0x5f90) = bVar4 | bVar10 | 0x1;
        return uVar6;
      }
      pcVar1 = (code *)swi(0x21);
      (*pcVar1)();
      uVar2 = 0x1800;
    }
  }
  else {
    if ((uVar5 & 0x500) != 0x500) {
      bVar7 = true;
      pcVar1 = (code *)swi(0x21);
      (*pcVar1)();
      if ((extraout_DX & 0x80) != 0x0) {
        bVar10 = bVar10 | 0x40;
      }
      uVar6 = uVar2;
      if ((bVar10 & 0x40) == 0x0) {
        if ((param_3 & 0x200) == 0x0) {
          if (((bVar10 & 0x80) != 0x0) && ((param_3 & 0x2) != 0x0)) {
            pcVar1 = (code *)swi(0x21);
            (*pcVar1)();
            pcVar1 = (code *)swi(0x21);
            iVar3 = (*pcVar1)();
            if ((iVar3 != 0x0) && (local_5 == '\x1a')) {
              pcVar1 = (code *)swi(0x21);
              (*pcVar1)();
              pcVar1 = (code *)swi(0x21);
              (*pcVar1)();
            }
            uVar5 = 0x0;
            pcVar1 = (code *)swi(0x21);
            (*pcVar1)();
            uVar6 = uVar2;
          }
        }
        else {
          if ((param_3 & 0x3) == 0x0) {
            pcVar1 = (code *)swi(0x21);
            (*pcVar1)();
            pcVar1 = (code *)swi(0x21);
            (*pcVar1)();
            goto LAB_1000_38e3;
          }
          uVar5 = 0x0;
          pcVar1 = (code *)swi(0x21);
          (*pcVar1)();
          uVar6 = uVar2;
        }
      }
      goto LAB_1000_3973;
    }
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
    uVar2 = 0x1100;
  }
  bVar8 = true;
LAB_1000_299d:
  if (bVar8) {
    pass1_1000_29b5(uVar2);
    uVar2 = 0xffff;
  }
  return uVar2;
}



void __cdecl16near pass1_1000_39e1(void)

{
  return;
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)
// WARNING: Removing unreachable block (ram,0x10003a40)
// WARNING: Removing unreachable block (ram,0x10003b7e)

uchar * __cdecl16far
mixed_dos3_call_1000_39f2
          (uchar *param_1,char *param_2,uchar *param_3,ushort param_4,ushort param_5,ushort param_6,char param_7)

{
  byte *pbVar1;
  undefined *puVar2;
  code *pcVar3;
  uint uVar4;
  uchar *puVar5;
  int *piVar6;
  uchar *puVar7;
  uint uVar8;
  int *piVar9;
  uchar *puVar10;
  int *piVar11;
  int iVar12;
  undefined *puVar13;
  byte *pbVar14;
  undefined *puVar15;
  int unaff_BP;
  byte *pbVar16;
  undefined *puVar17;
  ushort uVar18;
  undefined uVar19;
  byte bVar20;
  char cVar21;
  bool bVar22;
  char cVar23;
  char cVar24;
  undefined4 uVar25;
  char *pcVar26;
  int *piStack8;
  int *piStack6;
  int iStack2;
  
  puVar5 = DAT_1050_5f8a;
  iStack2 = unaff_BP + 0x1;
  puVar7 = DAT_1050_5f8a;
  if ((PTR_LOOP_1050_61ec != (undefined *)0x0) &&
     (puVar7 = PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e, param_1 < (uchar *)((int)&PTR_LOOP_1050_0002 + 0x1U))) {
    param_1 = DAT_1050_5f8a;
  }
  if (puVar7 <= param_1) {
    uVar19 = true;
    piVar6 = (int *)0x900;
    goto LAB_1000_299d;
  }
  puVar7 = param_1;
  if ((param_1[0x5f90] & 0x20) != 0x0) {
    uVar19 = false;
    pcVar3 = (code *)swi(0x21);
    piVar6 = (int *)(*pcVar3)();
    param_5 = 0x1000;
    if ((bool)uVar19) goto LAB_1000_299d;
  }
  pbVar14 = (byte *)param_2;
  if ((puVar7[0x5f90] & 0x80) == 0x0) {
LAB_1000_3acf:
    uVar19 = false;
    piVar6 = (int *)param_3;
    if (param_3 != (uchar *)0x0) {
      uVar19 = puVar7 < puVar5;
      if ((bool)uVar19) {
        uVar19 = 0x0;
        pcVar3 = (code *)swi(0x21);
        uVar25 = (*pcVar3)();
      }
      else {
        piVar6 = pass1_1000_55b1(0x3b71,param_4,param_5);
        uVar25 = CONCAT22(pbVar14,piVar6);
      }
      piVar6 = (int *)uVar25;
      if ((bool)uVar19) {
        piVar6 = (int *)CONCAT11(0x9,(char)uVar25);
      }
      else {
        uVar19 = false;
        if (piVar6 == (int *)0x0) {
          if (((puVar7[0x5f90] & 0x40) == 0x0) || (*(char *)((ulong)uVar25 >> 0x10) != '\x1a')) {
            uVar19 = true;
            piVar6 = (int *)0x1c00;
          }
          else {
            uVar19 = false;
          }
        }
      }
    }
  }
  else {
    bVar22 = true;
    piStack6 = (int *)0x0;
    piStack8 = (int *)0x0;
    puVar10 = param_3;
    pbVar16 = pbVar14;
    if (param_3 != (uchar *)0x0) {
      do {
        if (puVar10 == (uchar *)0x0) break;
        puVar10 = puVar10 + -0x1;
        pbVar1 = pbVar16;
        pbVar16 = pbVar16 + 0x1;
        bVar22 = *pbVar1 == 0xa;
      } while (!bVar22);
      param_4 = param_2._2_2_;
      if (!bVar22) goto LAB_1000_3acf;
      pcVar26 = param_2;
      uVar8 = pass1_1000_3bac();
      pcVar26._2_2_ = (int)((ulong)pcVar26 >> 0x10);
      iVar12 = (int)pcVar26;
      if (uVar8 < 0xa9) {
        piVar6 = exit_1000_25f2(0x3ad9,param_5,pcVar26._2_2_,-0x4,param_2._2_2_,param_5,param_6);
        piVar11 = (int *)(pbVar16 + -iVar12);
        if (piVar11 == (int *)0x0) {
          return (uchar *)piVar6;
        }
        bVar20 = param_1 < puVar5;
        uVar8 = (int)param_1 - (int)puVar5;
        cVar24 = (int)uVar8 < 0x0;
        cVar23 = uVar8 == 0x0;
        cVar21 = (POPCOUNT(uVar8 & 0xff) & 0x1U) == 0x0;
        if ((bool)bVar20) {
          bVar20 = 0x0;
          cVar24 = '\0';
          cVar23 = '\x01';
          cVar21 = '\x01';
          pcVar3 = (code *)swi(0x21);
          piVar9 = (int *)(*pcVar3)((int)&USHORT_1050_1050,puVar10,puVar7);
        }
        else {
          piVar9 = pass1_1000_55b1(0x3af1,param_2._2_2_,param_5);
        }
        if (!(bool)bVar20) {
          bVar20 = piVar11 < piVar9;
          uVar8 = (int)piVar11 - (int)piVar9;
          cVar24 = (int)uVar8 < 0x0;
          cVar23 = uVar8 == 0x0;
          cVar21 = (POPCOUNT(uVar8 & 0xff) & 0x1U) == 0x0;
          piStack6 = piVar9;
          if ((bool)bVar20 || (bool)cVar23) {
            return (uchar *)piVar6;
          }
        }
        uVar8 = (uint)(byte)(cVar24 << 0x7 | cVar23 << 0x6 | param_7 << 0x4 | cVar21 << 0x2 | 0x2U | bVar20) << 0x8;
        piVar6 = (int *)((uint)piVar9 & 0xff | uVar8);
        if (piStack6 == (int *)0x0) {
          uVar19 = (uVar8 & 0x100) != 0x0;
          if ((bool)uVar19) {
            piVar6 = (int *)CONCAT11(0x9,(char)((uint)piVar9 & 0xff));
          }
          else {
            if (((param_1[0x5f90] & 0x40) == 0x0) || (*param_2 != '\x1a')) {
              uVar19 = true;
              piVar6 = (int *)0x1c00;
            }
            else {
              uVar19 = false;
            }
          }
          goto LAB_1000_299d;
        }
      }
      else {
        puVar15 = &stack0xffec;
        iVar12 = 0x200;
        if (uVar8 < 0x228) {
          iVar12 = 0x80;
        }
        iVar12 = -iVar12;
        puVar13 = &stack0xffec + iVar12;
        puVar17 = &stack0xffec + iVar12;
        *(ushort *)(&stack0xffea + iVar12) = param_6;
        uVar18 = *(ushort *)(&stack0xffea + iVar12);
        do {
          pbVar1 = pbVar14;
          pbVar14 = pbVar14 + 0x1;
          bVar20 = *pbVar1;
          uVar4 = uVar8 & 0xff00;
          uVar8 = uVar4 | bVar20;
          if (bVar20 == 0xa) {
            uVar8 = CONCAT11((char)(uVar4 >> 0x8),0xd);
            if (puVar17 == puVar15) {
              *(undefined2 *)(&stack0xffea + iVar12) = 0x3abd;
              uVar8 = mixed_dos3_call_1000_3ad9(uVar8,puVar13,&iStack2,puVar17,uVar18,param_5,param_6,param_7);
            }
            puVar2 = puVar17;
            puVar17 = puVar17 + 0x1;
            *puVar2 = (char)uVar8;
            uVar8 = CONCAT11((char)(uVar8 >> 0x8),0xa);
            piStack8 = (int *)((int)piStack8 + 0x1);
          }
          if (puVar17 == puVar15) {
            *(undefined2 *)(&stack0xffea + iVar12) = 0x3ac8;
            uVar8 = mixed_dos3_call_1000_3ad9(uVar8,puVar13,&iStack2,puVar17,uVar18,param_5,param_6,param_7);
          }
          puVar2 = puVar17;
          puVar17 = puVar17 + 0x1;
          *puVar2 = (char)uVar8;
          param_3 = param_3 + -0x1;
        } while (param_3 != (uchar *)0x0);
        *(undefined2 *)(&stack0xffea + iVar12) = 0x3ab1;
        mixed_dos3_call_1000_3ad9(uVar8,puVar13,&iStack2,puVar17,uVar18,param_5,param_6,param_7);
      }
    }
    uVar19 = piStack6 < piStack8;
    piVar6 = (int *)((int)piStack6 - (int)piStack8);
  }
LAB_1000_299d:
  if ((bool)uVar19) {
    pass1_1000_29b5(piVar6);
    piVar6 = (int *)0xffff;
  }
  return (uchar *)piVar6;
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)

uint __cdecl16near
mixed_dos3_call_1000_3ad9
          (uint param_1,int param_2,int param_3,int param_4,ushort param_5,ushort param_6,ushort param_7,char param_8)

{
  uint *puVar1;
  int *piVar2;
  code *pcVar3;
  uint uVar4;
  uint uVar5;
  int *piVar6;
  int *piVar7;
  uint uVar8;
  byte bVar9;
  bool bVar10;
  char cVar11;
  char cVar12;
  char cVar13;
  
  piVar7 = (int *)(param_4 - param_2);
  if (piVar7 == (int *)0x0) {
    return param_1;
  }
  uVar8 = *(uint *)(param_3 + 0x6);
  puVar1 = (uint *)(param_3 + -0xc);
  bVar9 = uVar8 < *puVar1;
  uVar5 = uVar8 - *puVar1;
  cVar13 = (int)uVar5 < 0x0;
  cVar12 = uVar5 == 0x0;
  cVar11 = (POPCOUNT(uVar5 & 0xff) & 0x1U) == 0x0;
  if ((bool)bVar9) {
    bVar9 = 0x0;
    cVar13 = '\0';
    cVar12 = '\x01';
    cVar11 = '\x01';
    pcVar3 = (code *)swi(0x21);
    piVar6 = (int *)(*pcVar3)((int)&USHORT_1050_1050);
  }
  else {
    piVar6 = pass1_1000_55b1(0x3af1,param_5,param_6);
  }
  if (!(bool)bVar9) {
    piVar2 = (int *)(param_3 + -0x4);
    *piVar2 = *piVar2 + (int)piVar6;
    bVar9 = piVar7 < piVar6;
    uVar5 = (int)piVar7 - (int)piVar6;
    cVar13 = (int)uVar5 < 0x0;
    cVar12 = uVar5 == 0x0;
    cVar11 = (POPCOUNT(uVar5 & 0xff) & 0x1U) == 0x0;
    if ((bool)bVar9 || (bool)cVar12) {
      return param_1;
    }
  }
  uVar4 = (uint)(byte)(cVar13 << 0x7 | cVar12 << 0x6 | param_8 << 0x4 | cVar11 << 0x2 | 0x2U | bVar9) << 0x8;
  uVar5 = (uint)piVar6 & 0xff | uVar4;
  if (*(int *)(param_3 + -0x4) == 0x0) {
    bVar10 = (uVar4 & 0x100) != 0x0;
    if (bVar10) {
      uVar5 = CONCAT11(0x9,(char)((uint)piVar6 & 0xff));
    }
    else {
      if (((*(byte *)(uVar8 + 0x5f90) & 0x40) == 0x0) || (**(char **)(param_3 + 0x8) != '\x1a')) {
        bVar10 = true;
        uVar5 = 0x1c00;
      }
      else {
        bVar10 = false;
      }
    }
  }
  else {
    uVar5 = *(uint *)(param_3 + -0x4);
    puVar1 = (uint *)(param_3 + -0x6);
    bVar10 = uVar5 < *puVar1;
    uVar5 = uVar5 - *puVar1;
  }
  if (bVar10) {
    *(undefined2 *)(*(int *)(param_3 + -0xa) + 0x2) = 0x29a2;
    pass1_1000_29b5(uVar5);
    uVar5 = 0xffff;
  }
  return uVar5;
}



int __cdecl16far pass1_1000_3bac(void)

{
  int iVar1;
  
  if (PTR_LOOP_1050_5f48 < &stack0x0004) {
    iVar1 = -((int)PTR_LOOP_1050_5f48 - (int)&stack0x0004);
  }
  else {
    iVar1 = 0x0;
  }
  return iVar1;
}



void __cdecl16near pass1_1000_3bc0(int param_1,int param_2,uint *param_3,ushort param_4,ushort param_5,ushort param_6)

{
  int *piVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  undefined2 *puVar6;
  bool bVar7;
  ulong uVar8;
  
  if ((*(byte *)(param_2 + 0x2) & 0x1) != 0x0) {
    pass1_1000_3cb7(param_2);
    uVar4 = *param_3;
    if ((uVar4 & 0x1) != 0x0) {
      param_1 = (param_1 - uVar4) + -0x1;
    }
    uVar4 = *(uint *)(param_2 + 0x4);
    if (uVar4 != 0x0) {
      uVar3 = param_1 + 0x2U + uVar4;
      if (!CARRY2(param_1 + 0x2U,uVar4)) {
        param_4 = pass1_1000_29dc(param_6);
        uVar4 = *(uint *)&PTR_LOOP_1050_6066;
        if (uVar4 == 0x1000) goto LAB_1000_3c12;
        uVar2 = 0x8000;
        while (uVar4 <= uVar2) {
          uVar2 = uVar2 >> 0x1;
          if (uVar2 == 0x0) goto LAB_1000_3c2b;
        }
        if (uVar2 < 0x8) goto LAB_1000_3c2b;
        uVar4 = uVar2 << 0x1;
        goto LAB_1000_3c12;
      }
      uVar2 = 0x0;
      uVar4 = 0xfff0;
      if (uVar3 == 0x0) {
        while( true ) {
          bVar7 = false;
          uVar8 = mixed_mem_op_1000_3c51(uVar2,uVar3,param_2,param_4,param_5,0x3c23);
          if (!bVar7) break;
          if (uVar4 == 0xfff0) {
            return;
          }
LAB_1000_3c2b:
          uVar4 = 0x10;
LAB_1000_3c12:
          uVar4 = uVar4 - 0x1;
          uVar2 = uVar4 + uVar3;
          if (CARRY2(uVar4,uVar3)) {
            uVar2 = 0x0;
          }
          uVar4 = ~uVar4;
          uVar2 = uVar2 & uVar4;
        }
        iVar5 = (int)uVar8 - *(int *)(param_2 + 0x4);
        *(int *)(param_2 + 0x4) = (int)uVar8;
        *(uint **)(param_2 + 0xa) = param_3;
        piVar1 = *(int **)(param_2 + 0xc);
        *piVar1 = iVar5 + -0x1;
        puVar6 = (undefined2 *)((int)piVar1 + iVar5);
        *puVar6 = 0xfffe;
        *(undefined2 **)(param_2 + 0xc) = puVar6;
      }
    }
  }
  return;
}



ulong __cdecl16near
mixed_mem_op_1000_3c51(HGLOBAL16 param_1,HGLOBAL16 param_2,int param_3,uint16_t param_4,uint16_t param_5,int param_6)

{
  int *piVar1;
  char *pcVar2;
  LPCSTR str;
  int *piVar3;
  HGLOBAL16 HVar4;
  int *piVar5;
  char *pcVar6;
  DWORD DVar7;
  HGLOBAL16 HVar8;
  int iVar9;
  int iVar10;
  
  if ((*(byte *)(param_3 + 0x2) & 0x4) == 0x0) {
    HVar8 = *(HGLOBAL16 *)(param_3 + 0x6);
    param_5 = (uint16_t)s_tile2_bmp_1050_1538;
    HVar4 = GlobalReAlloc16(0x1000,CONCAT22(param_1,0x20),(uint)(param_1 == 0x0));
    if (HVar4 == 0x0) {
LAB_1000_3cb6:
      return CONCAT22(param_1,HVar4);
    }
    if (HVar4 == HVar8) {
      param_5 = (uint16_t)s_tile2_bmp_1050_1538;
      HVar4 = param_2;
      DVar7 = GlobalSize16((HGLOBAL16)s_tile2_bmp_1050_1538);
      if (DVar7 != 0x0) {
        param_1 = HVar4;
        if ((*(byte *)(HVar8 + 0x2) & 0x4) != 0x0) {
          param_1 = HVar4 - 0x1;
          *(HGLOBAL16 *)(HVar8 - 0x2) = param_1;
        }
        goto LAB_1000_3cb6;
      }
    }
  }
  iVar10 = 0x12;
  iVar9 = 0x12;
  pass1_1000_25a8(param_4,param_5);
  pass1_1000_2913(iVar9,param_4,param_5);
  str = poss_str_op_1000_28dc(iVar10);
  if (str != (PCHAR)0x0) {
    iVar9 = 0x9;
    if (*str == 'M') {
      iVar9 = 0xf;
    }
    str = str + iVar9;
    iVar9 = 0x22;
    pcVar6 = str;
    do {
      if (iVar9 == 0x0) break;
      iVar9 = iVar9 + -0x1;
      pcVar2 = pcVar6;
      pcVar6 = pcVar6 + 0x1;
    } while (*pcVar2 != '\r');
    pcVar6[-0x1] = '\0';
  }
  FatalAppExit16(param_5,str);
  FatalExit();
  piVar5 = (int *)&PTR_LOOP_1050_63fe;
  do {
    piVar1 = piVar5;
    piVar5 = piVar5 + 0x1;
    iVar9 = *piVar1;
    piVar3 = piVar5;
    if ((iVar9 == param_6) || (piVar3 = (int *)(iVar9 + 0x1), piVar3 == (int *)0x0)) {
      return CONCAT22(param_6,piVar3);
    }
    iVar9 = -0x1;
    do {
      if (iVar9 == 0x0) break;
      iVar9 = iVar9 + -0x1;
      piVar1 = piVar5;
      piVar5 = (int *)((int)piVar5 + 0x1);
    } while (*(char *)piVar1 != '\0');
  } while( true );
}



void __cdecl16near pass1_1000_3cb7(int param_1)

{
  uint uVar1;
  uint *puVar2;
  
  puVar2 = *(uint **)(param_1 + 0xa);
  if (puVar2 == *(uint **)(param_1 + 0xc)) {
    puVar2 = *(uint **)(param_1 + 0x8);
  }
  while( true ) {
    uVar1 = *puVar2;
    if (uVar1 == 0xfffe) break;
    puVar2 = (uint *)((int)puVar2 + (uVar1 & 0xfffe) + 0x2);
  }
  return;
}



void __cdecl16far pass1_1000_3cd8(ushort param_1,ushort param_2)

{
  free_mem_1000_407a(param_1,param_2,&stack0xfffe);
  return;
}



UINT16 * __cdecl16far pass1_1000_3cea(ULONG param_1,ULONG param_2)

{
  UINT16 *pUVar1;
  char *pcVar2;
  UINT16 *pUVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  UINT16 *pUVar7;
  char *pcVar8;
  UINT16 *pUVar9;
  UINT16 *pUVar10;
  undefined2 uVar11;
  undefined2 uVar12;
  bool bVar13;
  
  uVar11 = (undefined2)(param_1 >> 0x10);
  bVar13 = true;
  iVar4 = -0x1;
  pUVar7 = (UINT16 *)param_1;
  do {
    if (iVar4 == 0x0) break;
    iVar4 = iVar4 + -0x1;
    pUVar1 = pUVar7;
    pUVar7 = (UINT16 *)((int)pUVar7 + 0x1);
    bVar13 = *(char *)pUVar1 == '\0';
  } while (!bVar13);
  pUVar10 = (UINT16 *)((int)pUVar7 + -0x1);
  uVar12 = (undefined2)(param_2 >> 0x10);
  pcVar8 = (char *)param_2;
  uVar5 = 0xffff;
  do {
    if (uVar5 == 0x0) break;
    uVar5 = uVar5 - 0x1;
    pcVar2 = pcVar8;
    pcVar8 = pcVar8 + 0x1;
    bVar13 = *pcVar2 == '\0';
  } while (!bVar13);
  uVar5 = ~uVar5;
  if (!bVar13) {
    pcVar8 = pcVar8 + -uVar5;
    uVar5 = uVar5 + 0x1;
  }
  pUVar9 = (UINT16 *)(pcVar8 + -uVar5);
  if (uVar5 == 0x0) {
    pUVar1 = pUVar9;
    pUVar9 = pUVar9 + 0x1;
    *pUVar10 = *pUVar1;
    uVar5 = 0xfffe;
    pUVar10 = (UINT16 *)((int)pUVar7 + 0x1);
  }
  else {
    if (((uint)pUVar9 & 0x1) != 0x0) {
      pUVar1 = pUVar9;
      pUVar9 = (UINT16 *)((int)pUVar9 + 0x1);
      *(undefined *)pUVar10 = *(undefined *)pUVar1;
      uVar5 = uVar5 - 0x1;
      pUVar10 = pUVar7;
    }
  }
  for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
    pUVar3 = pUVar10;
    pUVar10 = pUVar10 + 0x1;
    pUVar1 = pUVar9;
    pUVar9 = pUVar9 + 0x1;
    *pUVar3 = *pUVar1;
  }
  for (uVar5 = (uint)((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 = uVar5 - 0x1) {
    pUVar3 = pUVar10;
    pUVar10 = (UINT16 *)((int)pUVar10 + 0x1);
    pUVar1 = pUVar9;
    pUVar9 = (UINT16 *)((int)pUVar9 + 0x1);
    *(undefined *)pUVar3 = *(undefined *)pUVar1;
  }
  return (UINT16 *)param_1;
}



void __cdecl16far unk_str_op_1000_3d3e(char *param_1,char *in_string_2)

{
  undefined2 *puVar4;
  undefined2 *puVar5;
  uint uVar6;
  uint uVar7;
  char *l_string_2;
  char *puVar6;
  char *puVar7;
  ushort uVar8;
  char *l_string_1;
  bool l_b_var8;
  char *puVar3;
  char *puVar2;
  char *puVar1;
  
  l_string_1 = (char *)((ulong)in_string_2 >> 0x10);
  l_string_2 = (char *)in_string_2;
  l_b_var8 = true;
  uVar6 = 0xffff;
  puVar6 = l_string_2;
  do {
    if (uVar6 == 0x0) break;
    uVar6 = uVar6 - 0x1;
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    l_b_var8 = *puVar1 == '\0';
  } while (!l_b_var8);
  uVar6 = ~uVar6;
  uVar8 = (ushort)((ulong)param_1 >> 0x10);
  puVar7 = (char *)param_1;
  if (l_b_var8) {
    if (((ulong)param_1 & 0x1) != 0x0) {
      puVar7 = puVar7 + 0x1;
      l_string_2 = l_string_2 + 0x1;
      *param_1 = *in_string_2;
      uVar6 = uVar6 - 0x1;
    }
  }
  else {
    puVar7 = puVar7 + 0x2;
    l_string_2 = l_string_2 + 0x2;
    *(undefined2 *)param_1 = *(undefined2 *)in_string_2;
    uVar6 = uVar6 - 0x1;
  }
  for (uVar7 = uVar6 >> 0x1; uVar7 != 0x0; uVar7 = uVar7 - 0x1) {
    puVar5 = (undefined2 *)puVar7;
    puVar7 = (char *)((int)puVar7 + 0x2);
    puVar4 = (undefined2 *)l_string_2;
    l_string_2 = (char *)((int)l_string_2 + 0x2);
    *puVar5 = *puVar4;
  }
  for (uVar6 = (uint)((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
    puVar5 = (undefined2 *)puVar7;
    puVar7 = (char *)((int)puVar7 + 0x1);
    puVar4 = (undefined2 *)l_string_2;
    l_string_2 = (char *)((int)l_string_2 + 0x1);
    *(undefined *)puVar5 = *(undefined *)puVar4;
  }
  return;
}