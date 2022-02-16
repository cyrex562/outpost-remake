
void __stdcall16far pass1_1010_2816(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(int *)(iVar4 + 0x124) != 0x0) {
    iVar5 = *(int *)(iVar4 + 0x124) * 0x4;
    puVar1 = (undefined4 *)*(uint *)(iVar5 + iVar4);
    uVar2 = *(uint *)(iVar5 + iVar4 + 0x2);
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    *(undefined4 *)(*(int *)(iVar4 + 0x124) * 0x4 + iVar4) = 0x0;
    *(undefined2 *)(iVar4 + 0x124) = 0x0;
  }
  return;
}



ushort __cdecl16far pass1_1010_286c(void)

{
  ushort *puVar1;
  
  pass1_1008_3e54((ushort *)&PTR_LOOP_1048_0000,0x0,0x5,0x12c);
  pass1_1008_3e54((ushort *)0x105065a6,0x0,0x9b,0x20);
  pass1_1008_3e54((ushort *)0x105065ac,0x0,0xf5,0x3f);
  pass1_1008_3e54((ushort *)0x105065b2,0x0,0x114,0x4a);
  pass1_1008_3e54((ushort *)0x105065b8,0x0,0x135,0x45);
  pass1_1008_3e54((ushort *)0x105065be,0x0,0xf5,0x7b);
  puVar1 = pass1_1008_3e54((ushort *)0x105065c4,0x0,0x117,0x91);
  return (ushort)puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_28e6(astruct_631 *param_1,uchar *param_2,ushort param_3,uchar *param_4,ushort param_5)

{
  undefined4 uVar1;
  ushort uVar2;
  int iVar3;
  undefined2 uVar4;
  astruct_43 *paVar5;
  int iStack6;
  
  struct_op_1018_4cda((int)param_1,(ushort)param_2,param_3);
  *(undefined4 *)&param_1->field_0x1c = 0x0;
  param_1->field_0x20 = 0x0;
  param_1->field_0x22 = 0x0;
  param_1->field_0x24 = 0x0;
  param_1->field_0x26 = 0x0;
  *(int *)CONCAT22(param_2,param_1) = (int)s_add16_wav_1050_2bdc + 0x8;
  param_1->field_0x2 = 0x1010;
  PTR_LOOP_1050_4230 = (undefined *)param_1;
  PTR_LOOP_1050_4232 = param_2;
  pass1_1018_4dce((ulong *)CONCAT13((char)((uint)param_2 >> 0x8),CONCAT12((char)param_2,param_1)),0x56,param_4,param_5);
  paVar5 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x4,param_5);
  PTR_LOOP_1050_5f2e = (undefined *)((ulong)paVar5 >> 0x10);
  param_1->field_0x1c = (int)paVar5;
  param_1->field_0x1e = PTR_LOOP_1050_5f2e;
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
  }
  uVar2 = fn_ptr_op_1000_1708(0x40,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  param_1->field_0x28 = uVar2;
  param_1->field_0x2a = PTR_LOOP_1050_5f2e;
  for (iStack6 = 0x0; iStack6 < 0x10; iStack6 = iStack6 + 0x1) {
    paVar5 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,iStack6 + 0x56,param_5);
    uVar1 = *(undefined4 *)&param_1->field_0x28;
    uVar4 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar3 = (int)uVar1;
    *(undefined2 *)(iVar3 + iStack6 * 0x4) = (int)paVar5;
    *(undefined2 *)(iVar3 + iStack6 * 0x4 + 0x2) = (int)((ulong)paVar5 >> 0x10);
  }
  return;
}



void __stdcall16far pass1_1010_29c6(astruct_11 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_476 *iVar5;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_476 *)param_1;
  *(int *)param_1 = (int)s_add16_wav_1050_2bdc + 0x8;
  iVar5->field_0x2 = 0x1010;
  if (*(long *)&iVar5->field_0x1c != 0x0) {
    puVar1 = (undefined4 *)*(uint *)&iVar5->field_0x1c;
    uVar2 = iVar5->field_0x1e;
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    *(undefined4 *)&iVar5->field_0x1c = 0x0;
    fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x28,0x1000);
    iVar5->field_0x28 = 0x0;
  }
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  return;
}



// WARNING: Instruction at (ram,0x10104b2b) overlaps instruction at (ram,0x10104b2a)
// 
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack

uint16_t __stdcall16far
draw_fn_1010_2a32(uint16_t param_1,uint16_t param_2,uint16 *__return_storage_ptr__,int param_4,ushort param_5,
                 ulong param_6,uint16_t param_7,uint16_t param_8,uint16_t param_9,uint16_t param_10)

{
  int *piVar1;
  char *pcVar2;
  byte *pbVar3;
  undefined4 uVar4;
  byte bVar5;
  uint uVar6;
  uint uVar7;
  code **ppcVar8;
  code *pcVar9;
  uint16 *puVar10;
  uint uVar11;
  HPALETTE16 b_force_background;
  HGDIOBJ16 handle;
  uint uVar12;
  ushort uVar13;
  BOOL16 BVar14;
  int iVar15;
  byte bVar16;
  uchar *extraout_DX;
  uchar *extraout_DX_00;
  uchar *puVar17;
  uchar *extraout_DX_01;
  uchar *extraout_DX_02;
  uchar *puVar18;
  int unaff_SI;
  int iVar19;
  int iVar20;
  ushort unaff_DI;
  undefined2 uVar21;
  HDC16 hdc;
  ushort unaff_SS;
  byte bVar22;
  bool bVar23;
  undefined1 in_AF;
  ulong uVar24;
  uint uStack54;
  undefined4 *puStack46;
  uint uStack42;
  undefined4 *puStack38;
  int local_22;
  int iStack32;
  HGDIOBJ16 HStack30;
  undefined uVar25;
  HGDIOBJ16 handle_00;
  undefined uVar26;
  undefined uVar27;
  undefined uVar28;
  
  puVar10 = __return_storage_ptr__;
  uVar27 = (undefined)param_9;
  uVar28 = (undefined)(param_9 >> 0x8);
  bVar22 = 0x0;
  uVar26 = 0x0;
  uVar25 = (undefined)((uint)param_4 >> 0x8);
  if (((ushort)param_6 + 0xec76 & 0x3) != 0x0) goto LAB_1010_2ad8;
  uVar11 = (ushort)param_6 + 0xec76 >> 0x1;
  if (0x1c < uVar11) goto LAB_1010_2ad8;
  switch(uVar11) {
  default:
    goto switchD_1010_2ab5_caseD_0;
  case 0x1:
  case 0x3:
  case 0xb:
    *(uint16_t *)(uVar11 + 0xa) = param_8;
  case 0x9:
  case 0xf:
  case 0x15:
  case 0x1b:
    *(uint16_t *)(uVar11 + 0xa) = param_8;
    *(uint16_t *)(uVar11 + 0x10) = param_8;
    *(uint16_t *)(uVar11 + 0xc) = param_8;
    return (uint16_t)(uchar *)param_10;
  case 0x5:
    BVar14 = write_to_file_1008_7e1c
                       (param_5,(ushort)param_6,param_8,0x0,
                        (char *)CONCAT13(param_1._1_1_,CONCAT12((undefined)param_1,param_9)),0x1008);
    if (BVar14 != 0x0) {
      return param_7;
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
    return param_7;
  case 0x6:
    bVar22 = 0x0;
    goto LAB_1010_2ad8;
  case 0x7:
    ppcVar8 = (code **)*(undefined4 *)param_8;
    (**ppcVar8)();
    iVar15 = param_5 + 0x105;
    puVar17 = extraout_DX;
    pass1_1010_8170(_PTR_LOOP_1050_14cc,iVar15,extraout_DX,0x1010);
    iVar19 = param_5 * 0x4;
    *(int *)((int)__return_storage_ptr__ + iVar19 + 0x26) = iVar15;
    *(uchar **)((int)__return_storage_ptr__ + iVar19 + 0x28) = puVar17;
    handle_00 = (HGDIOBJ16)&USHORT_1050_1050;
    uVar24 = pass1_1008_4772(*(astruct_76 **)((int)__return_storage_ptr__ + iVar19 + 0x26));
    puVar17 = (uchar *)(uVar24 >> 0x10);
    CreateDC16((LPCSTR)0x1008,(LPCSTR)uVar24,(LPCSTR)puVar17,(DEVMODEA *)puVar17);
    b_force_background =
         palette_op_1008_4e08(*(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe),&stack0xffec,(ushort)puVar17,0x1008);
    handle = SelectObject16(0x1008,CONCAT11(uVar26,bVar22));
    hdc = (HDC16)s_tile2_bmp_1050_1538;
    HStack30 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_00);
    for (iStack32 = 0x0; piVar1 = (int *)((int)__return_storage_ptr__ + 0x74),
        *piVar1 != iStack32 && iStack32 <= *piVar1; iStack32 = iStack32 + 0x1) {
      iVar15 = (iStack32 * 0x10 + param_5) * 0x8;
      puVar17 = *(uchar **)((int)__return_storage_ptr__ + 0x72);
      hdc = 0x1000;
      b_force_background = 0x48e1;
      uVar11 = pass1_1000_484c(CONCAT13((char)(unaff_SS >> 0x8),CONCAT12((char)unaff_SS,&stack0xfff2)),
                               CONCAT13((char)((uint)puVar17 >> 0x8),
                                        CONCAT12((char)puVar17,iVar15 + *(int *)(__return_storage_ptr__ + 0x7))),0x8);
      if (uVar11 != 0x0) {
        uVar4 = *(undefined4 *)(__return_storage_ptr__ + 0x7);
        uVar21 = (undefined2)((ulong)uVar4 >> 0x10);
        iVar19 = (int)uVar4;
        iVar20 = iVar15 + iVar19;
        hdc = (HDC16)s_tile2_bmp_1050_1538;
        b_force_background = (HPALETTE16)&PTR_LOOP_1050_4908;
        Rectangle16(0x1000,*(INT16 *)(iVar20 + 0x6),*(INT16 *)(iVar20 + 0x4),*(INT16 *)(iVar20 + 0x2),
                    *(INT16 *)(iVar19 + iVar15));
      }
    }
    SelectPalette16(hdc,0x0,b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
    SelectObject16((HDC16)s_tile2_bmp_1050_1538,HStack30);
    DeleteDC16((HDC16)s_tile2_bmp_1050_1538);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    return (uint16_t)puVar17;
  case 0x8:
    bVar22 = 0x3;
    goto LAB_1010_2ad8;
  case 0xd:
    pbVar3 = (byte *)(uVar11 + unaff_SI);
    bVar22 = *pbVar3;
    bVar5 = *pbVar3 + (byte)param_7;
    *pbVar3 = bVar5 + (uVar11 < 0x1c);
    uVar6 = (uint)(CARRY1(bVar22,(byte)param_7) || CARRY1(bVar5,uVar11 < 0x1c));
    uVar7 = param_8 + 0xeff0;
    bVar22 = param_8 < 0x1010 || uVar7 < uVar6;
    uVar12 = uVar7 - uVar6;
    pcVar9 = (code *)swi(0x4);
    if (SBORROW2(param_8,0x1010) != SBORROW2(uVar7,uVar6)) {
      (*pcVar9)();
      param_7 = (uint16_t)extraout_DX_00;
    }
    bVar23 = uVar12 < 0x1010 || uVar12 + 0xeff0 < (uint)bVar22;
    pbVar3 = (byte *)(uVar11 + unaff_SI);
    bVar22 = *pbVar3;
    bVar16 = (byte)param_7;
    bVar5 = *pbVar3;
    *pbVar3 = bVar5 + bVar16 + bVar23;
    pcVar2 = (char *)(uVar11 + unaff_SI);
    *pcVar2 = *pcVar2 + bVar16 + (CARRY1(bVar22,bVar16) || CARRY1(bVar5 + bVar16,bVar23));
    struct_op_1018_4cda(CONCAT11(uVar27,uVar26),CONCAT11((undefined)param_1,uVar28),
                        CONCAT11((undefined)param_2,param_1._1_1_));
    iVar15 = CONCAT11(uVar27,uVar26);
    piVar1 = (int *)CONCAT13((undefined)param_1,CONCAT12(uVar28,iVar15));
    *piVar1 = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1;
    *(undefined2 *)(iVar15 + 0x2) = 0x1010;
    pass1_1018_4dce((ulong *)CONCAT13((undefined)param_1,CONCAT12(uVar28,iVar15)),0x1b3,(uchar *)param_7,unaff_SS);
    _PTR_LOOP_1050_4230 = CONCAT13((undefined)param_1,CONCAT12(uVar28,CONCAT11(uVar27,uVar26)));
    return (uint16_t)(uchar *)CONCAT11((undefined)param_1,uVar28);
  case 0xe:
    *(ushort *)(__return_storage_ptr__ + 0x2) = param_5;
    break;
  case 0x11:
    do {
                    // WARNING: Do nothing block with infinite loop
    } while( true );
  case 0x12:
    PTR_LOOP_1050_10c6 = (undefined *)(uint)(0x0 < (int)param_5);
    PTR_LOOP_1050_1142 = (undefined *)(uint)(0x2 < (int)param_5);
    break;
  case 0x13:
    iVar15 = (int)__return_storage_ptr__ * 0x8 + param_1;
    if (((*(int *)(iVar15 + 0x22) != 0x0) || (*(int *)(iVar15 + 0x24) != 0x0)) ||
       ((*(int *)(iVar15 + 0x26) != 0x0 || (*(int *)(iVar15 + 0x28) != 0x0)))) {
      uVar4 = *(undefined4 *)(param_1 + 0xe);
      HStack30 = 0x627c;
      sys_1000_3f9c((uchar *)uVar4,(uchar *)((ulong)uVar4 >> 0x10),(ushort)s__d__d__d__d_1050_14ae,
                    (ushort)&USHORT_1050_1050,
                    (ushort)*(undefined4 *)((int)__return_storage_ptr__ * 0x8 + param_1 + 0x22),&stack0xfffa,param_2,
                    0x1000,unaff_SS,in_AF);
      uVar4 = *(undefined4 *)(param_1 + 0xa);
      HStack30 = 0x62a0;
      WritePrivateProfileString16
                ((LPCSTR)&PTR_LOOP_1050_1000,(LPCSTR)uVar4,(LPCSTR)((ulong)uVar4 >> 0x10),
                 (LPCSTR)*(undefined4 *)(param_1 + 0xe));
    }
    return param_7;
  case 0x14:
    *(ushort *)((int)__return_storage_ptr__ + 0x24) = param_5;
    break;
  case 0x17:
    puVar17 = (uchar *)(param_7 - 0x1);
    pbVar3 = (byte *)(uVar11 + unaff_SI);
    *pbVar3 = *pbVar3 | (byte)puVar17;
    *(uint16_t *)((int)__return_storage_ptr__ + 0x12) = param_8;
    *(uchar **)((int)__return_storage_ptr__ + 0x14) = puVar17;
    uStack42 = 0x0;
    while( true ) {
      if (uStack54 <= uStack42) {
        BVar14 = read_file_1008_7dee(param_5,(ushort)param_6,(int)__return_storage_ptr__ + 0x1a,0x0,param_4,0x2,0x1008);
        if (((BVar14 != 0x0) &&
            (BVar14 = read_file_1008_7dee(param_5,(ushort)param_6,(int)__return_storage_ptr__ + 0x1c,0x0,param_4,0x2,
                                          0x1008), BVar14 != 0x0)) &&
           (BVar14 = read_file_1008_7dee(param_5,(ushort)param_6,(int)__return_storage_ptr__ + 0x1e,0x0,param_4,0x2,
                                         0x1008), BVar14 != 0x0)) {
          return (uint16_t)puVar17;
        }
        PTR_LOOP_1050_0310 = (undefined *)0x6d2;
        return (uint16_t)puVar17;
      }
      uVar11 = uStack54;
      mem_op_1000_179c(0x8,puVar17,0x1000);
      puStack46 = (undefined4 *)CONCAT22(puVar17,uVar11);
      puVar18 = (uchar *)((uint)puVar17 | uVar11);
      if (puVar18 == (uchar *)0x0) {
        puStack38 = (undefined4 *)0x0;
      }
      else {
        *(undefined2 *)puStack46 = 0x389a;
        *(undefined2 *)(uVar11 + 0x2) = 0x1008;
        *(undefined2 *)puStack46 = 0xa1c4;
        *(undefined2 *)(uVar11 + 0x2) = 0x1010;
        puStack38 = puStack46;
      }
      BVar14 = read_file_1008_7dee(param_5,(ushort)param_6,(ushort)&local_22,0x0,unaff_SS,0x2,0x1008);
      uVar13 = (ushort)((ulong)puStack38 >> 0x10);
      if ((BVar14 == 0x0) ||
         (BVar14 = read_file_1008_7dee(param_5,(ushort)param_6,(int)puStack38 + 0x6,0x0,uVar13,0x2,0x1008),
         BVar14 == 0x0)) break;
      iVar15 = switch_1008_73ea(param_5,(ushort)param_6,local_22);
      *(int *)((int)puStack38 + 0x4) = iVar15;
      ppcVar8 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)__return_storage_ptr__ + 0x12) + 0x4);
      (**ppcVar8)();
      uStack42 = uStack42 + 0x1;
      puVar17 = extraout_DX_02;
    }
    if (puStack38 == (undefined4 *)0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d2;
      return (uint16_t)puVar18;
    }
    ppcVar8 = (code **)*puStack38;
    (**ppcVar8)();
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return (uint16_t)extraout_DX_01;
  case 0x18:
    bVar22 = 0x2;
    goto LAB_1010_2ad8;
  case 0x19:
    uVar13 = pass1_1010_6ca2(CONCAT13(uVar25,CONCAT12((char)param_4,__return_storage_ptr__)),0x8,param_7,unaff_SS);
    if (uVar13 != 0x0) {
      __return_storage_ptr__ = (uint16 *)((int)s_version__d__d_1050_0012 + 0x8);
      pass1_1010_715c(CONCAT22(0x1a,puVar10),0x1a,uVar13,param_7,unaff_DI,unaff_SS);
    }
    if (param_5 == 0x2c) {
      pass1_1010_715c(CONCAT22(0x1d,__return_storage_ptr__),0x1d,uVar13,param_7,unaff_DI,unaff_SS);
    }
    uVar13 = pass1_1010_6ca2(0x5a,0x2,param_7,unaff_SS);
    if (uVar13 != 0x0) {
      pass1_1010_715c(0x1c005a,0x1c,uVar13,param_7,unaff_DI,unaff_SS);
    }
    return param_7;
  case 0x1a:
    *(ushort *)((int)__return_storage_ptr__ + 0x26) = param_5;
  }
  bVar22 = 0x1;
LAB_1010_2ad8:
  if ((bVar22 == 0x1) || (bVar22 == 0x2)) {
    if (bVar22 == 0x1) {
      param_5 = *(int *)(__return_storage_ptr__ + 0x2) + *(int *)((int)__return_storage_ptr__ + 0x22) +
                *(int *)((int)__return_storage_ptr__ + 0x24) + *(int *)((int)__return_storage_ptr__ + 0x26);
    }
    if (param_5 != 0x0) {
      param_7 = (int)param_5 >> 0xf;
      param_5 = (int)param_5 / 0x2 + 0x1;
      if (0x5 < (int)param_5) {
        param_5 = 0x5;
      }
      if (((bVar22 == 0x1) && (PTR_LOOP_1050_10c6 != (undefined *)0x0)) && (0x4 < (int)param_5)) {
        param_5 = 0x4;
      }
    }
  }
  *(ushort *)((uint)bVar22 * 0x7c + 0xed6) = param_5;
  pass1_1010_1f62(unaff_SS,CONCAT13(uVar25,CONCAT12((char)param_4,__return_storage_ptr__)),0xc);
switchD_1010_2ab5_caseD_0:
  return param_7;
}



void __stdcall16far pass1_1010_2b50(ushort param_1,ushort param_2,ushort *param_3)

{
  pass1_1008_3f62(param_3,(ushort *)&PTR_LOOP_1048_0000);
  return;
}



ulong __stdcall16far pass1_1010_2b66(ulong param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)param_1 + 0x1e),*(undefined2 *)((int)param_1 + 0x1c));
}



void __stdcall16far pass1_1010_2b78(ushort param_1,ushort param_2,int param_3,ulong param_4)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined4 *puVar5;
  
  puVar3 = (undefined4 *)(param_3 * 0x7c + 0xed4);
  puVar5 = (undefined4 *)param_4;
  for (iVar4 = 0x1f; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
    puVar2 = puVar5;
    puVar5 = puVar5 + 0x1;
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar2 = *puVar1;
  }
  return;
}



ulong __stdcall16far pass1_1010_2b98(ulong param_1,int param_2)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x28);
  uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
  iVar2 = (int)uVar1;
  return CONCAT22(*(undefined2 *)(param_2 * 0x4 + iVar2 + -0x156),*(undefined2 *)(param_2 * 0x4 + iVar2 + -0x158));
}



void __cdecl16far pass1_1010_2bb9(void)

{
  pass1_1010_286c();
  return;
}



astruct_11 * __stdcall16far pass1_1010_2bbe(astruct_11 *param_1,byte param_2)

{
  pass1_1010_29c6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1010_2bfc(astruct_644 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xc = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x10 = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0x2cc2;
  param_1->field_0x2 = 0x1010;
  return (ushort *)CONCAT22(param_2,param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort * __stdcall16far unk_load_str_op_1010_2c34(void)

{
  UINT16 *pUVar1;
  uchar *in_DX;
  short in_buf_len_5;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar2;
  
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,in_DX,unaff_DI);
  mem_op_1000_179c(0x80,(uchar *)((ulong)puVar2 >> 0x10),0x1000);
  in_buf_len_5 = (short)((ulong)puVar2 >> 0x10);
  load_string_1010_84e0
            (0x1000,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x80,(char *)puVar2,
             in_buf_len_5);
  pUVar1 = pass1_1000_3cea((ULONG)puVar2,(ULONG)s__1050_11c8);
  pass1_1010_e964(in_buf_len_5,unaff_SS,unaff_DI);
  pass1_1000_3cea((ULONG)puVar2,CONCAT22(in_buf_len_5,pUVar1));
  return puVar2;
}



ushort * __stdcall16far pass1_1010_2c9c(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far struct_1010_2cd2(astruct_79 *param_1,astruct_79 *param_2,ushort param_3,ushort param_4)

{
  int unaff_DI;
  astruct_79 *paVar1;
  ushort *puVar2;
  int *piVar3;
  int *piVar4;
  ushort uVar5;
  int local_6;
  int local_4;
  
  paVar1 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)&param_1[0x1].field_0x8 = 0x0;
  param_1[0x2].field_0x2 = 0x0;
  *(undefined2 *)&param_1[0x2].field_0x4 = 0x0;
  *(undefined2 *)&param_1[0x3].field_0x4 = 0x0;
  *(undefined2 *)((int)&param_1[0x3].field_0x4 + 0x2) = 0x0;
  param_1[0x3].field_0x8 = 0x0;
  ((astruct_79 *)(param_1 + 0x4))->field_0x0 = 0x0;
  *(undefined4 *)&param_1[0x8].field_0x2 = 0x0;
  *(undefined4 *)((int)&param_1[0x8].field_0x4 + 0x2) = 0x0;
  ((astruct_79 *)(param_1 + 0x9))->field_0x0 = 0x0;
  *(undefined2 *)&param_1[0x9].field_0x4 = 0x0;
  param_1[0x9].field_0x2 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x36da;
  param_1->field_0x2 = 0x1010;
  piVar4 = &local_4;
  piVar3 = &local_6;
  uVar5 = param_4;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,(uchar *)((ulong)paVar1 >> 0x10),unaff_DI);
  pass1_1008_3e94((ushort *)((ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0xe)),(ushort *)CONCAT22(param_4,piVar3)
                  ,(ushort *)CONCAT22(uVar5,piVar4));
  param_1[0x1].field_0x4 = 0x19001db;
  ((astruct_79 *)(param_1 + 0x1))->field_0x0 = 0x140 - (*(int *)&param_1[0x1].field_0x4 / 0x2 - local_4);
  param_1[0x1].field_0x2 = 0xf0 - (*(int *)((int)&param_1[0x1].field_0x4 + 0x2) / 0x2 - local_6);
  *(undefined4 *)((int)&param_1[0x2].field_0x4 + 0x2) = 0xa006e;
  *(undefined4 *)(param_1 + 0x3) = 0xa012c;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1[0x4].field_0x2),(WNDCLASS16 *)0x0,0x28);
  return;
}



void __stdcall16far pass1_1010_2db2(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_473 *iVar5;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_473 *)param_1;
  *param_1 = 0x36da;
  iVar5->field_0x2 = 0x1010;
  puVar1 = iVar5->field_0x56;
  uVar2 = iVar5->field_0x58;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x5c,0x1000);
  pass1_1010_1d80(param_1,param_2);
  return;
}



ulong __stdcall16far pass1_1010_2e02(ulong param_1,int param_2)

{
  undefined4 uVar1;
  astruct_163 *iVar2;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x5c) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x5c);
    uVar2 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (astruct_163 *)uVar1;
    return CONCAT22(*(undefined2 *)(iVar2 + param_2 * 0x4 + 0x2),*(undefined2 *)(iVar2 + param_2 * 0x4));
  }
  return 0x0;
}



void __stdcall16far pass1_1010_2e30(ulong param_1,ushort param_2,ushort param_3,int param_4)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x5c) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x5c);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    *(ushort *)(iVar2 + param_4 * 0x4) = param_2;
    *(ushort *)(iVar2 + param_4 * 0x4 + 0x2) = param_3;
  }
  return;
}



void __stdcall16far pass1_1010_2e5c(ushort param_1,ushort param_2,ulong param_3)

{
  ulong uStack12;
  
  uStack12 = param_3;
  if (param_3 == 0x0) {
    return;
  }
  for (; (uStack12 & 0xf) != 0x0; uStack12 = uStack12 >> 0x4) {
  }
  return;
}



void __stdcall16far pass1_1010_2ee2(ulong *param_1,ushort param_2,ushort param_3)

{
  undefined4 uVar1;
  code **ppcVar2;
  int iVar3;
  undefined2 extraout_DX;
  int iVar4;
  undefined2 uVar5;
  ulong uVar6;
  ulong *puStack6;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x52) != 0x0) {
    return;
  }
  iVar3 = 0x0;
  *(undefined2 *)(iVar4 + 0x28) = 0x0;
  uVar6 = *param_1;
  ppcVar2 = (code **)((int)uVar6 + 0x20);
  (**ppcVar2)(param_3,param_1,*(undefined4 *)(iVar4 + 0x12));
  if (iVar3 == 0x0) {
    puStack6 = *(ulong **)(iVar4 + 0x56);
  }
  else {
    uVar1 = *(undefined4 *)(iVar4 + 0x12);
    ppcVar2 = (code **)((int)uVar6 + 0x14);
    (**ppcVar2)(param_3,param_1,(char)uVar1,(int)((ulong)uVar1 >> 0x10));
    puStack6 = (ulong *)CONCAT22(extraout_DX,iVar3);
    uVar6 = pass1_1010_2e02((ulong)param_1,*(int *)(iVar3 + 0x12));
    pass1_1010_35a4(param_1,uVar6,(int)(uVar6 >> 0x10));
  }
  pass1_1010_32f4(param_1,puStack6,param_2,param_3);
  pass1_1010_1f62(param_2,(ulong)param_1,0x8);
  if (*(long *)(iVar4 + 0x52) != 0x0) {
    return;
  }
  return;
}



void __stdcall16far unk_destroy_win_op_1010_2fa0(ULONG param_1,HWND16 param_2)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  HWND16 HVar5;
  int iStack4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  *(undefined2 *)(iVar3 + 0x28) = 0x0;
  iStack4 = 0x0;
  while( true ) {
    piVar1 = (int *)(iVar3 + 0x16);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    DestroyWindow16(param_2);
    iStack4 = iStack4 + 0x1;
    param_2 = (HWND16)s_tile2_bmp_1050_1538;
  }
  *(undefined2 *)(iVar3 + 0x16) = 0x0;
  if ((*(uint *)(iVar3 + 0x54) | *(uint *)(iVar3 + 0x52)) != 0x0) {
    iStack4 = 0x0;
    do {
      uVar2 = *(undefined4 *)(iVar3 + 0x52);
      HVar5 = param_2;
      if (*(long *)((int)uVar2 + iStack4 * 0x4) != 0x0) {
        HVar5 = (HWND16)s_tile2_bmp_1050_1538;
        DestroyWindow16(param_2);
        uVar2 = *(undefined4 *)(iVar3 + 0x52);
        *(undefined4 *)((int)uVar2 + iStack4 * 0x4) = 0x0;
      }
      iStack4 = iStack4 + 0x1;
      param_2 = HVar5;
    } while (iStack4 < 0xa);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar3 + 0x52),0x1000);
    *(undefined4 *)(iVar3 + 0x52) = 0x0;
  }
  return;
}



void __stdcall16far unk_destroy_win_op_1010_305a(astruct_27 *param_1,int param_2,astruct_65 *param_3,UINT16 param_4)

{
  int *piVar1;
  ULONG UVar2;
  long lVar3;
  bool bVar4;
  ushort uVar5;
  astruct_27 *iVar4;
  int iVar6;
  astruct_27 *uVar7;
  undefined2 uVar8;
  HWND16 hwnd;
  HWND16 hwnd_00;
  ushort unaff_SS;
  int iStack10;
  int iStack8;
  int iStack6;
  
  hwnd = (HWND16)&PTR_LOOP_1050_1040;
  uVar5 = pass1_1040_c60e((ulong)param_3);
  uVar7 = (astruct_27 *)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_27 *)param_1;
  iVar4->field_0x12 = uVar5;
  iVar4->field_0x14 = 0x0;
  iStack6 = 0x0;
  bVar4 = false;
  iVar4->field_0x28 = 0x0;
  iStack8 = 0x0;
  do {
    piVar1 = &iVar4->field_0x16;
    if (*piVar1 == iStack8 || *piVar1 < iStack8) {
LAB_1010_30ad:
      iVar6 = iStack6;
      if (bVar4) {
        while (iStack8 = iVar6 + 0x1, piVar1 = &iVar4->field_0x16, *piVar1 != iStack8 && iStack8 <= *piVar1) {
          DestroyWindow16(hwnd);
          (&iVar4->field_0x2e)[iVar6] = 0x0;
          hwnd = (HWND16)s_tile2_bmp_1050_1538;
          iVar6 = iStack8;
        }
        iVar4->field_0x16 = iStack6 + 0x1;
        pass1_1010_1f62(unaff_SS,(ulong)param_1,0x9);
      }
      else {
        iVar6 = iVar4->field_0x16;
        (&iVar4->field_0x2a)[iVar6 * 0x2] = (UINT16)param_3;
        (&iVar4->field_0x2c)[iVar6 * 0x2] = (UINT16)((ulong)param_3 >> 0x10);
        iStack10 = 0xa;
        piVar1 = &iVar4->field_0x16;
        *piVar1 = *piVar1 + 0x1;
        if (0x1 < iVar4->field_0x16) {
          UVar2 = (&iVar4->field_0x22)[iVar4->field_0x16];
          iVar6 = (int)UVar2;
          uVar8 = (undefined2)(UVar2 >> 0x10);
          iStack10 = *(int *)(iVar6 + 0x20) + *(int *)(iVar6 + 0x24) + 0x8;
        }
        hwnd = (HWND16)&PTR_LOOP_1050_1040;
        mov_update_win_1040_93aa(param_3,iStack10,iVar4->field_0x1a,(int)&PTR_LOOP_1050_1040);
      }
      if (!bVar4) {
        pass1_1010_1f62(unaff_SS,(ulong)param_1,0xa);
      }
      if (param_2 == 0x0) {
        if (iVar4->field_0x52 != 0x0) {
          iStack8 = 0x0;
          hwnd_00 = hwnd;
          do {
            lVar3 = iVar4->field_0x52;
            uVar8 = (undefined2)((ulong)lVar3 >> 0x10);
            iVar6 = (int)lVar3;
            hwnd = hwnd_00;
            if ((*(long *)(iVar6 + iStack8 * 0x4) != 0x0) && (*(astruct_65 **)(iVar6 + iStack8 * 0x4) != param_3)) {
              hwnd = (HWND16)s_tile2_bmp_1050_1538;
              DestroyWindow16(hwnd_00);
            }
            lVar3 = iVar4->field_0x52;
            *(undefined4 *)((int)lVar3 + iStack8 * 0x4) = 0x0;
            iStack8 = iStack8 + 0x1;
            hwnd_00 = hwnd;
          } while (iStack8 < 0xa);
        }
        pass1_1010_32da((ulong *)param_1,(ulong)param_3,hwnd,unaff_SS);
        pass1_1010_1f62(unaff_SS,(ulong)param_1,0x8);
      }
      return;
    }
    if (*(astruct_65 **)(&iVar4->field_0x2a + iStack8 * 0x2) == param_3) {
      bVar4 = true;
      iStack6 = iStack8;
      goto LAB_1010_30ad;
    }
    iStack8 = iStack8 + 0x1;
  } while( true );
}



void __stdcall16far win_ui_op_1010_3202(ulong param_1,int param_2,HWND16 param_3)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  HWND16 hwnd;
  ushort unaff_SS;
  int iStack4;
  
  iVar3 = (int)param_1;
  uVar4 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x0) {
    piVar1 = (int *)(iVar3 + 0x28);
    *piVar1 = *piVar1 + -0xa;
    if (*piVar1 < 0x0) {
      *(undefined2 *)(iVar3 + 0x28) = 0x0;
    }
  }
  else {
    piVar1 = (int *)(iVar3 + 0x28);
    *piVar1 = *piVar1 + *(int *)(iVar3 + 0x18);
  }
  if (*(long *)(iVar3 + 0x52) != 0x0) {
    iStack4 = 0x0;
    hwnd = param_3;
    do {
      uVar2 = *(undefined4 *)(iVar3 + 0x52);
      param_3 = hwnd;
      if (*(long *)((int)uVar2 + iStack4 * 0x4) != 0x0) {
        param_3 = (HWND16)s_tile2_bmp_1050_1538;
        DestroyWindow16(hwnd);
        uVar2 = *(undefined4 *)(iVar3 + 0x52);
        *(undefined4 *)((int)uVar2 + iStack4 * 0x4) = 0x0;
      }
      iStack4 = iStack4 + 0x1;
      hwnd = param_3;
    } while (iStack4 < 0xa);
  }
  if (*(int *)(iVar3 + 0x16) == 0x0) {
    pass1_1010_32f4((ulong *)param_1,*(ulong **)(iVar3 + 0x56),unaff_SS,param_3);
  }
  else {
    pass1_1010_32da((ulong *)param_1,*(ulong *)(iVar3 + *(int *)(iVar3 + 0x16) * 0x4 + 0x26),param_3,unaff_SS);
  }
  pass1_1010_1f62(unaff_SS,param_1,0x8);
  return;
}



void __stdcall16far pass1_1010_32c0(ulong param_1,ulong param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x28) = 0x0;
  *(ulong *)((int)param_1 + 0x12) = param_2;
  return;
}



void __stdcall16far pass1_1010_32da(ulong *param_1,ulong param_2,ushort param_3,ushort param_4)

{
  pass1_1010_32f4(param_1,*(ulong **)((int)param_2 + 0x42),param_4,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_32f4(ulong *param_1,ulong *param_2,ushort param_3,ushort param_4)

{
  uint *puVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  code **ppcVar5;
  astruct_65 *paVar6;
  ulong uVar7;
  uint uVar8;
  ushort uVar9;
  uint uVar10;
  int iVar11;
  undefined *extraout_DX;
  undefined2 extraout_DX_00;
  astruct_166 *iVar10;
  int iVar12;
  int iVar13;
  undefined2 uVar14;
  undefined2 uVar15;
  undefined2 uVar16;
  uint *puStack48;
  uint uStack16;
  int iStack12;
  
  uVar14 = (undefined2)((ulong)param_1 >> 0x10);
  iVar10 = (astruct_166 *)param_1;
  if (iVar10->field_0x52 != (astruct_65 *)0x0) {
    param_4 = 0x1000;
    fn_ptr_1000_17ce((astruct_18 *)iVar10->field_0x52,0x1000);
    iVar10->field_0x52 = (astruct_65 *)0x0;
    iVar10->field_0x18 = 0x0;
  }
  uVar8 = param_2._2_2_ | (uint)param_2;
  if ((param_2 != (ulong *)0x0) &&
     (ppcVar5 = (code **)((int)*param_1 + 0x24), (**ppcVar5)(param_4,param_1,param_2), uVar8 != 0x0)) {
    ppcVar5 = (code **)((int)*param_2 + 0x4);
    (**ppcVar5)(param_4,param_2);
    iVar10->field_0x18 = uVar8;
    if (uVar8 != 0x0) {
      iVar10->field_0x24 = 0x0;
      iVar10->field_0x26 = 0x0;
      puVar1 = &iVar10->field_0x18;
      *puVar1 = *puVar1 - iVar10->field_0x28;
      if (0xa < (int)iVar10->field_0x18) {
        iVar10->field_0x26 = 0x1;
        iVar10->field_0x18 = 0xa;
      }
      if (0x1 < iVar10->field_0x28) {
        iVar10->field_0x24 = 0x1;
      }
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2e = extraout_DX;
        PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)extraout_DX,0x1000);
      }
      else {
      }
      uVar16 = 0x1000;
      uVar9 = fn_ptr_op_1000_1708(0x28,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
      *(ushort *)&iVar10->field_0x52 = uVar9;
      *(undefined2 *)((int)&iVar10->field_0x52 + 0x2) = PTR_LOOP_1050_5f2e;
      if ((*(uint *)((int)&iVar10->field_0x52 + 0x2) | *(uint *)&iVar10->field_0x52) != 0x0) {
        uVar3 = *(undefined4 *)((uint)param_2 + 0x8);
        iVar11 = iVar10->field_0x10;
        iStack12 = 0x0;
        for (uStack16 = 0x0; puVar1 = &iVar10->field_0x18, *puVar1 != uStack16 && (int)uStack16 <= (int)*puVar1;
            uStack16 = uStack16 + 0x1) {
          paVar6 = iVar10->field_0x52;
          uVar8 = (int)paVar6 + uStack16 * 0x4;
          uVar7 = (ulong)paVar6 & 0xffff0000;
          puStack48 = (uint *)(uVar7 | uVar8);
          uVar4 = *(undefined4 *)((iVar10->field_0x28 + uStack16) * 0x4 + (int)uVar3);
          ppcVar5 = (code **)((int)*param_1 + 0x1c);
          uVar10 = uStack16;
          (**ppcVar5)(uVar16,param_1,(int)uVar4,(char)((ulong)uVar4 >> 0x10),iVar10->field_0x22);
          *puStack48 = uVar10;
          *(undefined2 *)(uVar8 + 0x2) = extraout_DX_00;
          paVar6 = iVar10->field_0x52;
          uVar4 = *(undefined4 *)((int)paVar6 + uStack16 * 0x4);
          iStack12 = iStack12 + *(int *)((int)uVar4 + 0x24) + 0x8;
          if (iVar11 + -0xa < iStack12) {
            uVar16 = 0x1008;
            debug_print_1008_6048((ulong)s_overflow_on_node__d_1050_11ca,0x1008,param_3);
            iVar10->field_0x18 = uStack16 - 0x1;
            iVar10->field_0x26 = 0x1;
            paVar6 = iVar10->field_0x52;
            uVar15 = (undefined2)((ulong)paVar6 >> 0x10);
            iVar13 = (int)paVar6;
            puVar2 = (undefined4 *)*(uint *)(iVar13 + uStack16 * 0x4);
            uVar8 = *(uint *)(iVar13 + uStack16 * 0x4 + 0x2);
            if ((uVar8 | (uint)puVar2) != 0x0) {
              ppcVar5 = (code **)*puVar2;
              (**ppcVar5)(0x1008,puVar2,(char)uVar8,0x1);
            }
            paVar6 = iVar10->field_0x52;
            iVar13 = uStack16 * 0x4;
            *(undefined4 *)((int)paVar6 + iVar13) = 0x0;
            if (0x0 < (int)uStack16) {
              paVar6 = iVar10->field_0x52;
              uVar15 = (undefined2)((ulong)paVar6 >> 0x10);
              iVar12 = (int)paVar6;
              puVar2 = (undefined4 *)*(uint *)(iVar12 + iVar13 + -0x4);
              uVar8 = *(uint *)(iVar12 + iVar13 + -0x2);
              if ((uVar8 | (uint)puVar2) != 0x0) {
                ppcVar5 = (code **)*puVar2;
                (**ppcVar5)(0x1008,puVar2,(char)uVar8,0x1);
              }
              paVar6 = iVar10->field_0x52;
              *(undefined4 *)(uStack16 * 0x4 + (int)paVar6 + -0x4) = 0x0;
            }
          }
        }
        iVar10->field_0x20 = 0xa;
        uVar9 = iVar10->field_0x1e;
        mov_update_win_1040_93aa(*(astruct_65 **)iVar10->field_0x52,0xa,uVar9,(int)&PTR_LOOP_1050_1040);
        for (uStack16 = 0x1; puVar1 = &iVar10->field_0x18, *puVar1 != uStack16 && (int)uStack16 <= (int)*puVar1;
            uStack16 = uStack16 + 0x1) {
          paVar6 = iVar10->field_0x52;
          uVar3 = *(undefined4 *)(uStack16 * 0x4 + (int)paVar6 + -0x4);
          iVar11 = (int)uVar3;
          uVar16 = (undefined2)((ulong)uVar3 >> 0x10);
          paVar6 = iVar10->field_0x52;
          mov_update_win_1040_93aa
                    (*(astruct_65 **)((int)paVar6 + uStack16 * 0x4),
                     *(int *)(iVar11 + 0x20) + *(int *)(iVar11 + 0x24) + 0x8,uVar9,(int)&PTR_LOOP_1050_1040);
        }
      }
    }
  }
  return;
}
