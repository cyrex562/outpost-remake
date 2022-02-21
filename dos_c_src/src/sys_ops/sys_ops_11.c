
void __stdcall16far mixed_win_sys_op_1008_016e(ulong param_1,ushort param_2)

{
  code **ppcVar1;
  undefined2 *puVar2;
  int iVar3;
  uint uVar4;
  undefined4 uVar5;
  uchar *puVar6;
  uchar *extraout_DX;
  uchar *puVar7;
  uint uVar8;
  int unaff_DI;
  undefined2 uVar9;
  HINSTANCE16 instance;
  undefined2 uVar10;
  DWORD DVar11;
  ulong *puVar12;
  ulong uVar13;
  astruct_20 *paVar14;
  CHAR local_1be [0x80];
  CHAR local_13e [0xac];
  CHAR local_92 [0x80];
  uint uStack18;
  uchar *puStack16;
  undefined4 *puStack14;
  uint uStack10;
  uint uStack8;
  undefined2 uStack6;
  uchar *puStack4;
  
  instance = (HINSTANCE16)s_tile2_bmp_1050_1538;
  DVar11 = GetVersion16();
  puVar7 = (uchar *)(DVar11 >> 0x10);
  uStack6 = (undefined2)(DVar11 & 0xffff);
  uVar4 = (uint)DVar11 & 0xff;
  uStack10 = (uint)(byte)((DVar11 & 0xffff) >> 0x8);
  uStack8 = uVar4;
  puStack4 = puVar7;
  if ((uVar4 < 0x3) || ((uVar4 == 0x3 && (uStack10 < 0xa)))) {
    uVar10 = 0x1000;
    mem_op_1000_179c(0xb4,puVar7,0x1000);
    puVar6 = (uchar *)((uint)puVar7 | uVar4);
    uStack18 = uVar4;
    puStack16 = puVar7;
    if (puVar6 == (uchar *)0x0) {
      iVar3 = 0x0;
      puVar6 = (uchar *)0x0;
    }
    else {
      uVar10 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar3 = string_1040_8520((astruct_57 *)CONCAT22(puVar7,uVar4),0x0,0x10,0x2,0x5de,0x5dd,puVar6,param_2);
    }
    puStack14 = (undefined4 *)CONCAT22(puVar6,iVar3);
    ppcVar1 = (code **)((int)*puStack14 + 0x74);
    (**ppcVar1)(uVar10,iVar3,(char)puVar6);
    instance = 0x1000;
    puVar7 = extraout_DX;
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  debug_print_1008_6048((ulong)s_version__d__d_1050_0012,instance,param_2);
  if ((uStack8 == 0x3) && (0xb < (int)uStack10)) {
    PTR_LOOP_1050_0010 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
  }
  LoadString16(instance,0x80,local_92,param_2);
  uVar4 = dos3_call_1000_51aa((ushort)&stack0xfffe);
  if (uVar4 != 0x0) {
    LoadString16(0x1000,0x80,local_13e,param_2);
    LoadString16((HINSTANCE16)s_tile2_bmp_1050_1538,0x80,local_1be,param_2);
    uVar4 = MessageBox16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)&PTR_LOOP_1050_0010,local_13e,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  mem_op_1000_179c(0x4,puVar7,0x1000);
  if (((uint)puVar7 | uVar4) == 0x0) {
    uVar10 = 0x0;
    puVar6 = (uchar *)0x0;
    uStack18 = uVar4;
    puStack16 = puVar7;
  }
  else {
    uStack18 = uVar4;
    puStack16 = puVar7;
    puVar12 = pass1_1008_5394((ulong *)CONCAT22(puVar7,uVar4));
    puVar6 = (uchar *)((ulong)puVar12 >> 0x10);
    uVar10 = SUB42(puVar12,0x0);
  }
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  *(undefined2 *)(iVar3 + 0x8) = uVar10;
  *(uchar **)(iVar3 + 0xa) = puVar6;
  uVar5 = *(undefined4 *)(iVar3 + 0x8);
  puVar2 = (undefined2 *)*(undefined4 *)(iVar3 + 0x8);
  _PTR_LOOP_1050_0298 = uVar5;
  *puVar2 = 0x70;
  *(undefined2 *)((int)puVar2 + 0x2) = (int)s_tile2_bmp_1050_1538;
  uVar10 = 0x1000;
  mem_op_1000_179c(0x126,puVar6,0x1000);
  uVar4 = (uint)uVar5;
  puVar7 = (uchar *)((uint)puVar6 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar6;
  if (puVar7 != (uchar *)0x0) {
    uVar10 = 0x1010;
    uVar13 = pass1_1010_2024(CONCAT13((char)((uint)puVar6 >> 0x8),CONCAT12((char)puVar6,uVar4)));
    puVar7 = (uchar *)(uVar13 >> 0x10);
    uVar4 = (uint)uVar13;
  }
  if (_PTR_LOOP_1050_0ed0 == 0x0) {
    debug_print_1008_6048((ulong)s_New_failed_in_Op__Op_1050_0020,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  uVar10 = 0x1000;
  mem_op_1000_179c(0xe8c,puVar7,0x1000);
  puVar6 = (uchar *)((uint)puVar7 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar7;
  if (puVar6 != (uchar *)0x0) {
    uVar10 = 0x1010;
    pass1_1010_7e40((ulong *)CONCAT22(puVar7,uVar4),puVar6,unaff_DI,param_2);
  }
  if (_PTR_LOOP_1050_14cc == 0x0) {
    debug_print_1008_6048(0x10500035,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  uVar10 = 0x1000;
  mem_op_1000_179c(0xb0,puVar6,0x1000);
  puVar7 = (uchar *)((uint)puVar6 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar6;
  if (puVar7 != (uchar *)0x0) {
    uVar10 = SUB42(&PTR_LOOP_1050_1038,0x0);
    paVar14 = pass1_1038_aeca((astruct_20 *)CONCAT22(puVar6,uVar4),param_2);
    puVar7 = (uchar *)((ulong)paVar14 >> 0x10);
    uVar4 = (uint)paVar14;
  }
  if (_PTR_LOOP_1050_5b7c == 0x0) {
    debug_print_1008_6048((ulong)s_New_failed_in_Op__Op__DialogCtr_1050_0053,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  uVar10 = 0x1000;
  mem_op_1000_179c(0xa,puVar7,0x1000);
  puVar6 = (uchar *)((uint)puVar7 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar7;
  if (puVar6 != (uchar *)0x0) {
    uVar10 = SUB42(&PTR_LOOP_1050_1038,0x0);
    make_proc_inst_1038_cf6c((ushort *)CONCAT22(puVar7,uVar4),puVar6,(int)&PTR_LOOP_1050_1038);
  }
  if (_PTR_LOOP_1050_5bc8 == 0x0) {
    debug_print_1008_6048((ulong)s_New_failed_in_Op__Op__DialogHand_1050_0073,uVar10,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  mem_op_1000_179c(0x14,puVar6,0x1000);
  puVar7 = (uchar *)((uint)puVar6 | uVar4);
  uStack18 = uVar4;
  puStack16 = puVar6;
  if (puVar7 != (uchar *)0x0) {
    pass1_1008_5bdc((astruct_79 *)CONCAT22(puVar6,uVar4),unaff_DI,param_2);
  }
  if (_PTR_LOOP_1050_02a0 == 0x0) {
    debug_print_1008_6048((ulong)s_New_failed_in_Op__Op__Simulator_1050_0097,0x1000,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  mem_op_1000_179c(0xfc,puVar7,0x1000);
  uVar8 = (uint)puVar7 | uVar4;
  uStack18 = uVar4;
  puStack16 = puVar7;
  if (uVar8 == 0x0) {
    uVar4 = 0x0;
    uVar8 = 0x0;
  }
  else {
    set_struct_op_1008_0536((ushort *)CONCAT22(puVar7,uVar4),0x1000,param_2);
  }
  *(uint *)(iVar3 + 0x4) = uVar4;
  *(uint *)(iVar3 + 0x6) = uVar8;
  if (*(long *)(iVar3 + 0x4) == 0x0) {
    debug_print_1008_6048((ulong)s_New_failed_in_Op__Op_1050_00b7,0x1000,param_2);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  win_ui_reg_class_1008_96d2(*(astruct_20 **)(iVar3 + 0x4),0x1000,param_2);
  uVar5 = *(undefined4 *)(iVar3 + 0x4);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x4) + 0x8);
  (**ppcVar1)(0x1000,(int)uVar5,(int)((ulong)uVar5 >> 0x10));
  uVar5 = *(undefined4 *)(iVar3 + 0x4);
  PTR_LOOP_1050_0396 = (undefined *)*(undefined2 *)((int)uVar5 + 0x8);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x4) + 0xc);
  (**ppcVar1)(0x1000,*(undefined4 *)(iVar3 + 0x4),0x3);
  UpdateWindow16(0x1000);
  return;
}


BOOL16 __stdcall16far pass1_1008_07d8(ushort param_1,BOOL16 param_2,uchar *param_3,ushort param_4)

{
  ushort uVar2;
  ushort uVar1;
  uchar in_AF;
  ulong uVar3;
  
  if (_PTR_LOOP_1050_5748 == (ulong *)0x0) {
    uVar1 = 0x1000;
    mem_op_1000_179c(0xa,param_3,0x1000);
    uVar2 = (uint)param_3 | param_2;
    if (uVar2 != 0x0) {
      uVar1 = 0x1030;
      struct_1030_8128((ulong *)CONCAT22(param_3,param_2),uVar2,param_4);
    }
    if (_PTR_LOOP_1050_5748 == (ulong *)0x0) {
      debug_print_1008_6048((ulong)s_New_failed_in_Op__Op__Simulator_1050_0110,uVar1,param_4);
      fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
    }
    uVar3 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,uVar2,0x8);
    uVar3 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(ushort)(uVar3 >> 0x10),0x8);
    pass1_1028_e2e0(_PTR_LOOP_1050_65e2,(ushort)(uVar3 >> 0x10),0xff);
    pass1_1030_838e(_PTR_LOOP_1050_5748,param_4,in_AF);
    param_2 = pass1_1030_8334(_PTR_LOOP_1050_5748);
  }
  return param_2;
}


void __cdecl16far
pass1_1000_4aea(uint param_1,uint param_2,int param_3,uint param_4,uchar *param_5,int param_6,int param_7,ushort param_8
               ,ushort param_9,ushort param_10)

{
  uint *puVar1;
  code **ppcVar2;
  long lVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  astruct_171 *puVar11;
  uint uVar9;
  uint uVar10;
  uint uVar11;
  bool bVar12;
  uint uStack26;
  uint uStack24;
  uint uStack22;
  uint uVar13;
  uint uVar14;
  undefined4 uStack18;
  uint uStack14;
  
  if ((param_4 != 0x0) && (param_3 != 0x0)) {
    uStack14 = param_1;
    uVar11 = param_2;
    for (iVar6 = param_3 + -0x1; iVar6 != 0x0; iVar6 = iVar6 + -0x1) {
      uVar9 = uStack14 + param_4;
      uVar11 = uVar11 + (-(uint)CARRY2(uStack14,param_4) & 0x6c);
      uStack18 = CONCAT22(uVar11,uVar9);
      iVar5 = (*(code *)param_5)(param_9);
      if (iVar5 < 0x0) {
        uVar11 = param_3 - 0x1;
        iVar6 = 0x0;
        do {
          uVar11 = uVar11 >> 0x1;
          iVar6 = iVar6 + -0x1;
        } while (iVar6 != 0x0 && uVar11 != 0x0);
        if (((int)((ulong)(uint)-iVar6 * 0x8 >> 0x10) != 0x0) ||
           (uVar11 = pass1_1000_3bac(), uVar11 < (uint)((ulong)(uint)-iVar6 * 0x8))) {
          exit_1000_25f2(0x4b7b,param_9,param_7,-0x4,param_8,param_9,param_10);
          return;
        }
        puVar11 = (astruct_171 *)&stack0xfff6;
        lVar3 = (ulong)(param_3 - 0x1) * (ulong)param_4;
        uVar11 = (uint)lVar3;
        uStack14 = uVar11 + param_1;
        uVar11 = ((int)((ulong)lVar3 >> 0x10) + (uint)CARRY2(uVar11,param_1)) * 0x100 + param_2;
        uStack18._0_2_ = param_1;
        uStack18._2_2_ = param_2;
LAB_1000_4b7d:
        if (puVar11 <= (astruct_171 *)&uStack18) {
          return;
        }
LAB_1000_4b81:
        if ((uStack18._2_2_ < uVar11) || ((uStack18._2_2_ <= uVar11 && ((uint)uStack18 < uStack14)))) {
          uStack22 = uStack14;
          puVar1 = &puVar11->field_0x14;
          uVar8 = uStack14 + *puVar1;
          uVar7 = uVar11 + (-(uint)CARRY2(uStack14,*puVar1) & 0x6c);
          uVar9 = uStack18._2_2_;
          uVar10 = (uint)uStack18;
          uStack26 = (uint)uStack18;
          uStack24 = uStack18._2_2_;
          uVar13 = uVar11;
LAB_1000_4bbc:
          do {
            puVar1 = &puVar11->field_0x14;
            bVar12 = CARRY2(uVar10,*puVar1);
            uVar10 = uVar10 + *puVar1;
            uVar9 = uVar9 + (-(uint)bVar12 & 0x6c);
            uVar4 = uStack22;
            if ((uVar10 != uStack14) || (uVar9 != uVar11)) {
              ppcVar2 = (code **)&puVar11->field_0x16;
              iVar6 = (**ppcVar2)(param_9,uVar10,uVar9,(uint)uStack18,uStack18._2_2_);
              if (iVar6 < 0x1) {
                if (iVar6 != 0x0) {
                  uStack26 = uVar10;
                  uStack24 = uVar9;
                }
                goto LAB_1000_4bbc;
              }
            }
            do {
              uVar14 = uVar13;
              uStack22 = uVar4;
              puVar1 = &puVar11->field_0x14;
              bVar12 = uVar8 < *puVar1;
              uVar8 = uVar8 - *puVar1;
              uVar7 = uVar7 - (-(uint)bVar12 & 0x6c);
              ppcVar2 = (code **)&puVar11->field_0x16;
              iVar6 = (**ppcVar2)(param_9,(uint)uStack18,uStack18._2_2_,uVar8,uVar7);
              if (0x0 < iVar6) break;
              uVar4 = uVar8;
              uVar13 = uVar7;
            } while (((iVar6 != 0x0) || (uVar4 = uStack22, uVar13 = uVar14, uVar8 != (uint)uStack18)) ||
                    (uVar7 != uStack18._2_2_));
            if ((uVar7 < uVar9) || ((uVar7 <= uVar9 && (uVar8 <= uVar10)))) goto LAB_1000_4c58;
            pass1_1000_4ceb(puVar11->field_0x14,uVar10,uVar8,uVar7);
            uStack26 = uVar10;
            uStack24 = uVar9;
            uVar13 = uVar7;
            uStack22 = uVar8;
          } while( true );
        }
        goto LAB_1000_4b7d;
      }
      uStack14 = uVar9;
    }
  }
  return;
LAB_1000_4c58:
  pass1_1000_4ceb(puVar11->field_0x14,(uint)uStack18,uVar8,uVar7);
  uVar10 = ((uVar11 - (-(uint)(uStack14 < uStack22) & 0x6c)) - uVar14) +
           (-(uint)CARRY2(uStack14 - uStack22,(uint)uStack18) & 0x6c) + uStack18._2_2_;
  uVar9 = -(uint)((uStack14 - uStack22) + (uint)uStack18 < uStack26) & 0x6c;
  if ((uVar10 < uVar9) || (uVar10 - uVar9 < uStack24)) {
    uStack14 = uStack26;
    uVar11 = uStack24;
  }
  else {
    uStack18._0_2_ = uStack22;
    uStack18._2_2_ = uVar14;
  }
  goto LAB_1000_4b81;
}

int * pass1_1000_4f1a(int param_1,uint16_t param_2,uint16_t param_3)

{
  int *piVar1;
  char *pcVar2;
  LPCSTR str;
  int *piVar3;
  int *piVar4;
  char *pcVar5;
  int iVar6;
  int iVar7;
  
  iVar7 = 0x19;
  iVar6 = 0x19;
  pass1_1000_25a8(param_2,param_3);
  pass1_1000_2913(iVar6,param_2,param_3);
  str = poss_str_op_1000_28dc(iVar7);
  if (str != (PCHAR)0x0) {
    iVar6 = 0x9;
    if (*str == 'M') {
      iVar6 = 0xf;
    }
    str = str + iVar6;
    iVar6 = 0x22;
    pcVar5 = str;
    do {
      if (iVar6 == 0x0) break;
      iVar6 = iVar6 + -0x1;
      pcVar2 = pcVar5;
      pcVar5 = pcVar5 + 0x1;
    } while (*pcVar2 != '\r');
    pcVar5[-0x1] = '\0';
  }
  FatalAppExit16(param_3,str);
  FatalExit();
  piVar4 = (int *)&PTR_LOOP_1050_63fe;
  do {
    piVar1 = piVar4;
    piVar4 = piVar4 + 0x1;
    iVar6 = *piVar1;
    piVar3 = piVar4;
    if ((iVar6 == param_1) || (piVar3 = (int *)(iVar6 + 0x1), piVar3 == (int *)0x0)) {
      return piVar3;
    }
    iVar6 = -0x1;
    do {
      if (iVar6 == 0x0) break;
      iVar6 = iVar6 + -0x1;
      piVar1 = piVar4;
      piVar4 = (int *)((int)piVar4 + 0x1);
    } while (*(char *)piVar1 != '\0');
  } while( true );
}



// WARNING: Removing unreachable block (ram,0x10004f47)

ushort __cdecl16far dos3_call_1000_4f20(ushort param_1)

{
  code *pcVar1;
  undefined2 uVar2;
  bool bVar3;
  
  bVar3 = false;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)((int)&USHORT_1050_1050,param_1 + 0x1);
  if (bVar3) {
    pass1_1000_29b5(uVar2);
    return 0xffff;
  }
  return 0x0;
}

ushort __cdecl16far dos3call_1000_4f54(ulong param_1,int param_2)

{
  char cVar1;
  code *pcVar2;
  uint uVar3;
  char *pcVar4;
  bool bVar5;
  undefined4 uVar6;
  
  bVar5 = false;
  pcVar2 = (code *)swi(0x21);
  uVar6 = (*pcVar2)((int)&USHORT_1050_1050,param_2 + 0x1);
  pcVar4 = (char *)((ulong)uVar6 >> 0x10);
  uVar3 = (uint)uVar6;
  if ((bVar5) && (bVar5 = uVar3 < 0x10, uVar3 == 0x10)) {
    do {
      cVar1 = *pcVar4;
      pcVar4 = pcVar4 + 0x1;
      if (cVar1 == '\0') goto LAB_1000_4f90;
    } while ((cVar1 != '?') && (cVar1 != '*'));
    uVar3 = 0x3;
LAB_1000_4f90:
    bVar5 = true;
  }
  if (!bVar5) {
    return 0x0;
  }
  pass1_1000_29b5(uVar3);
  return 0xffff;
}



// WARNING: Removing unreachable block (ram,0x10004fa9)

int __cdecl16far dos3_call_1000_4f94(ushort param_1)

{
  code *pcVar1;
  byte bVar2;
  
  pcVar1 = (code *)swi(0x21);
  bVar2 = (*pcVar1)(param_1 + 0x1);
  return bVar2 + 0x1;
}



// WARNING: Removing unreachable block (ram,0x10004fd7)
// WARNING: Removing unreachable block (ram,0x10004feb)

ushort __cdecl16far dos3_call_1000_4fbe(char param_1,int param_2)

{
  code *pcVar1;
  char cVar2;
  ushort uVar3;
  
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)(param_2 + 0x1);
  pcVar1 = (code *)swi(0x21);
  cVar2 = (*pcVar1)();
  uVar3 = 0xffff;
  if ((char)(cVar2 + '\x01') == param_1) {
    uVar3 = 0x0;
  }
  return uVar3;
}

void __cdecl16far
pass1_1000_5026(int param_1,uint param_2,uint param_3,uint param_4,int param_5,ushort param_6,ushort param_7)

{
  uint uVar1;
  uint uVar2;
  ushort uStack304;
  ushort local_12c [0x3];
  ushort uStack294;
  undefined *local_124 [0x6];
  int iStack280;
  undefined local_116;
  undefined uStack277;
  char cStack272;
  undefined *puStack270;
  undefined local_108;
  undefined uStack263;
  undefined uStack262;
  undefined auStack261 [0x101];
  undefined2 local_4;
  int iStack2;
  
  iStack2 = param_5 + 0x1;
  local_4 = SUB42(&USHORT_1050_1050,0x0);
  _uStack304 = (char *)CONCAT22(param_7,&local_108);
  if (param_1 == 0x0) {
    param_1 = dos3_call_1000_4f94((ushort)&iStack2);
  }
  *_uStack304 = (char)param_1 + '@';
  uStack263 = 0x3a;
  puStack270 = auStack261;
  uStack262 = 0x5c;
  uStack277 = 0x47;
  cStack272 = (char)param_1;
  local_12c[0] = param_7;
  uStack294 = param_7;
  dos3_call_set_struct_1000_42de
            ((ushort *)CONCAT22(param_7,&local_116),(ushort *)CONCAT22(param_7,local_124),
             (ushort *)CONCAT22(param_7,local_12c));
  if (iStack280 == 0x0) {
    uVar1 = str_op_1000_3da4((char *)CONCAT22(param_7,&local_108));
    uVar1 = uVar1 + 0x1;
    uStack304 = param_2;
    uVar2 = param_3 | param_2;
    if (uVar2 == 0x0) {
      if ((int)param_4 < (int)uVar1) {
        param_4 = uVar1;
      }
      uStack304 = mem_1000_167a(param_4,param_6,0x0);
      param_3 = uVar2;
      if ((uVar2 | uStack304) == 0x0) {
        PTR_LOOP_1050_5f78 = (undefined *)&PTR_LOOP_1050_000c;
        return;
      }
    }
    if ((int)param_4 < (int)uVar1) {
      PTR_LOOP_1050_5f78 = (undefined *)((int)s_New_failed_in_Op__Op_1050_0020 + 0x2);
    }
    else {
      unk_str_op_1000_3d3e((char *)CONCAT22(param_3,uStack304),(char *)CONCAT22(param_7,&local_108));
    }
  }
  else {
    PTR_LOOP_1050_5f78 = (undefined *)((int)&PTR_LOOP_1050_000c + 0x1);
    PTR_LOOP_1050_5f88 = local_124[0];
  }
  return;
}



// WARNING: Removing unreachable block (ram,0x10005167)

ushort __cdecl16far dos3_call_1000_514e(int param_1)

{
  code *pcVar1;
  undefined2 uVar2;
  bool bVar3;
  
  bVar3 = false;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)((int)&USHORT_1050_1050,param_1 + 0x1);
  if (bVar3) {
    pass1_1000_29b5(uVar2);
    return 0xffff;
  }
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x1000518c)

uint __cdecl16far dos3_call_1000_5174(ushort param_1)

{
  code *pcVar1;
  uint uVar2;
  bool bVar3;
  
  bVar3 = false;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)(param_1 + 0x1);
  if (!bVar3) {
    return 0x0;
  }
  pass1_1000_29b5(uVar2);
  return uVar2 & 0xff;
}



// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)

uint __cdecl16far dos3_calls_1000_5198(int param_1)

{
  code *pcVar1;
  uint uVar2;
  byte bVar3;
  
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)((int)&USHORT_1050_1050,param_1 + 0x1);
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  bVar3 = 0x0;
  pcVar1 = (code *)swi(0x21);
  uVar2 = (*pcVar1)();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  if ((bVar3 & 0x1) == 0x0) {
    return 0x0;
  }
  pass1_1000_29b5(uVar2);
  return uVar2 & 0xff;
}


void __cdecl16far fatal_app_exit_1000_3e9e(UINT16 app_exit_action)

{
  FatalAppExit16(app_exit_action,(LPCSTR)s_ABNORMAL_PROGRAM_TERMINATION_1050_6544);
  return;
}

ushort __cdecl16far
sys_1000_3f9c(uchar *param_1,uchar *param_2,ushort param_3,ushort param_4,ushort param_5,int param_6,ushort param_7,
             ushort param_8,ushort param_9,uchar param_10)

{
  undefined *puVar1;
  ushort uVar2;
  ushort local_4;
  int iStack2;
  
  iStack2 = param_6 + 0x1;
  PTR_LOOP_1050_68b2._0_1_ = 0x42;
  PTR_LOOP_1050_68ae = param_1;
  PTR_LOOP_1050_68b0 = param_2;
  _USHORT_1050_68a8 = (undefined *)CONCAT22(param_2,param_1);
  PTR_LOOP_1050_68ac = (undefined *)0x7fff;
  uVar2 = sys_1000_30b4((ushort)&USHORT_1050_68a8,(ushort)&USHORT_1050_1050,(byte *)CONCAT22(param_4,param_3),
                        (int)&iStack2,(uint)&USHORT_1050_68a8,param_7,param_8,param_9);
  puVar1 = _USHORT_1050_68a8;
  PTR_LOOP_1050_68ac = PTR_LOOP_1050_68ac + -0x1;
  if ((int)PTR_LOOP_1050_68ac < 0x0) {
    mem_1000_2bb6(0x0,(int *)&USHORT_1050_68a8,(int)&iStack2,param_7,param_8,param_9,param_10,(uint)param_2);
  }
  else {
    _USHORT_1050_68a8 = (undefined *)((ulong)_USHORT_1050_68a8 & 0xffff0000 | (ulong)(USHORT_1050_68a8 + 0x1));
    *puVar1 = 0x0;
  }
  return uVar2;
}



uchar * __cdecl16far pass1_1000_400a(int param_1,ushort param_2)

{
  uchar *puVar1;
  int iStack2;
  
  iStack2 = param_2 + 0x1;
  if ((param_1 < 0x0) || ((int)PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1)) {
    PTR_LOOP_1050_5f78 = (undefined *)&DAT_1050_0009;
    puVar1 = (uchar *)0xffff;
  }
  else {
    if (((PTR_LOOP_1050_61ec == (undefined *)0x0) || ((param_1 < DAT_1050_5f8a && (0x2 < param_1)))) &&
       (0x31d < CONCAT11(DAT_1050_5f83,DAT_1050_5f82))) {
      puVar1 = PTR_LOOP_1050_5f88;
      if (((*(byte *)(param_1 + 0x5f90) & 0x1) == 0x0) ||
         (puVar1 = (uchar *)dos3_call_1000_5174((ushort)&iStack2), puVar1 != (uchar *)0x0)) {
        PTR_LOOP_1050_5f88 = puVar1;
        PTR_LOOP_1050_5f78 = (undefined *)&DAT_1050_0009;
        puVar1 = (uchar *)0xffff;
      }
    }
    else {
      puVar1 = (uchar *)0x0;
    }
  }
  return puVar1;
}

void __cdecl16far free_mem_1000_407a(ushort param_1,ushort param_2,undefined2 param_3)

{
  GlobalFree16(0x1000);
  return;
}


int * __cdecl16far mixed_sys_op_1000_40af(uint param_1,int param_2,uint param_3,ushort param_4,uint16_t param_5)

{
  ushort *puVar1;
  ushort uVar2;
  char *pcVar3;
  undefined2 *puVar4;
  LPCSTR str;
  ushort *puVar5;
  uint uVar6;
  uint uVar7;
  HGLOBAL16 HVar8;
  SEGPTR SVar9;
  int iVar10;
  uint uVar11;
  ushort *puVar12;
  char *pcVar13;
  undefined2 *puVar14;
  undefined2 unaff_SS;
  bool bVar15;
  int iVar16;
  uint uVar17;
  
  do {
    uVar6 = (uint)((ulong)param_1 * (ulong)param_3);
    uVar7 = param_2 * param_3 + (int)((ulong)param_1 * (ulong)param_3 >> 0x10);
    if ((uVar7 | uVar6) != 0x0) {
      puVar12 = (ushort *)0x0;
      if ((uVar7 < 0x3) && ((uVar7 < 0x2 || (uVar6 == 0x0)))) {
        if (uVar7 == 0x0) {
          uVar6 = uVar6 + 0xfff & 0xf000;
          if (uVar6 == 0x0) {
            uVar7 = 0x1;
          }
        }
        else {
          if ((param_3 - 0x1 & param_3) != 0x0) {
            puVar12 = (ushort *)(((ulong)uVar7 << 0x10) % (ulong)param_3);
            bVar15 = CARRY2(uVar6,(uint)puVar12);
            uVar6 = uVar6 + (int)puVar12;
            if (bVar15) goto LAB_1000_41aa;
            uVar7 = 0x1;
          }
        }
      }
      else {
        if ((param_3 - 0x1 & param_3) != 0x0) goto LAB_1000_41aa;
      }
      uVar17 = 0x0;
      uVar11 = uVar7;
      HVar8 = GLobalAlloc16(0x1000,CONCAT22(uVar7,uVar6));
      if ((HVar8 != 0x0) && ((uVar17 & 0x1) != 0x0)) {
        SVar9 = WIN16_GlobalLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
        if ((SVar9 != 0x0) || (uVar7 == 0x0)) {
          iVar16 = 0x12;
          iVar10 = 0x12;
          pass1_1000_25a8(param_5,(int)s_tile2_bmp_1050_1538);
          pass1_1000_2913(iVar10,param_5,(uint16_t)s_tile2_bmp_1050_1538);
          str = poss_str_op_1000_28dc(iVar16);
          if (str == (PCHAR)0x0) goto LAB_1000_28cb;
          iVar10 = 0x9;
          if (*str == 'M') {
            iVar10 = 0xf;
          }
          str = str + iVar10;
          iVar10 = 0x22;
          pcVar13 = str;
          break;
        }
        HVar8 = pass1_1000_422a(uVar7,HVar8,(int)s_tile2_bmp_1050_1538,unaff_SS);
        if (HVar8 == 0x0) {
          GlobalUnlock16((HGLOBAL16)s_tile2_bmp_1050_1538);
          GlobalFree16((HGLOBAL16)s_tile2_bmp_1050_1538);
          HVar8 = 0x0;
        }
      }
      param_4 = (ushort)s_tile2_bmp_1050_1538;
      if (HVar8 != 0x0) {
        puVar14 = (undefined2 *)0x0;
        for (; uVar11 != 0x0; uVar11 = uVar11 - 0x1) {
          for (iVar10 = -0x8000; iVar10 != 0x0; iVar10 = iVar10 + -0x1) {
            puVar4 = puVar14;
            puVar14 = puVar14 + 0x1;
            *puVar4 = 0x0;
          }
          HVar8 = HVar8 + 0x100;
        }
        if (uVar6 != 0x0) {
          for (; uVar6 != 0x0; uVar6 = uVar6 - 0x1) {
            puVar4 = puVar14;
            puVar14 = (undefined2 *)((int)puVar14 + 0x1);
            *(undefined *)puVar4 = 0x0;
          }
        }
        return (int *)puVar12;
      }
    }
LAB_1000_41aa:
    if (((uint)PTR_LOOP_1050_618e | (uint)PTR_LOOP_1050_618c) == 0x0) {
      return (int *)(ushort *)0x0;
    }
    iVar10 = (*(code *)PTR_LOOP_1050_618c)(param_4,param_3,param_1,param_2);
    if (iVar10 == 0x0) {
      return (int *)(ushort *)0x0;
    }
  } while( true );
  while( true ) {
    iVar10 = iVar10 + -0x1;
    pcVar3 = pcVar13;
    pcVar13 = pcVar13 + 0x1;
    if (*pcVar3 == '\r') break;
    if (iVar10 == 0x0) break;
  }
  pcVar13[-0x1] = '\0';
LAB_1000_28cb:
  FatalAppExit16((UINT16)s_tile2_bmp_1050_1538,str);
  FatalExit();
  puVar12 = (ushort *)&PTR_LOOP_1050_63fe;
  do {
    puVar1 = puVar12;
    puVar12 = puVar12 + 0x1;
    uVar2 = *puVar1;
    puVar5 = puVar12;
    if ((uVar2 == HVar8) || (puVar5 = (ushort *)(uVar2 + 0x1), puVar5 == (ushort *)0x0)) {
      return (int *)puVar5;
    }
    iVar10 = -0x1;
    do {
      if (iVar10 == 0x0) break;
      iVar10 = iVar10 + -0x1;
      puVar1 = puVar12;
      puVar12 = (ushort *)((int)puVar12 + 0x1);
    } while (*(char *)puVar1 != '\0');
  } while( true );
}

void __cdecl16far dos3_call_set_struct_1000_42de(ushort *param_1,ushort *param_2,ushort *param_3)

{
  undefined2 uVar1;
  ushort uVar2;
  code *pcVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  int iVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  bool bVar10;
  undefined4 uVar11;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar5 = *(undefined2 *)(iVar6 + 0x2);
  uVar4 = *(undefined2 *)(iVar6 + 0x4);
  uVar1 = *(undefined2 *)(iVar6 + 0x8);
  uVar7 = *(undefined2 *)(iVar6 + 0xa);
  uVar8 = (undefined2)((ulong)param_3 >> 0x10);
  uVar2 = *param_3;
  uVar9 = *(undefined2 *)((int)param_3 + 0x6);
  bVar10 = false;
  pcVar3 = (code *)swi(0x21);
  uVar11 = (*pcVar3)();
  *param_3 = uVar2;
  *(undefined2 *)((int)param_3 + 0x6) = uVar9;
  uVar9 = (undefined2)((ulong)param_2 >> 0x10);
  iVar6 = (int)param_2;
  *param_2 = (uint)uVar11;
  *(undefined2 *)(iVar6 + 0x2) = uVar5;
  *(undefined2 *)(iVar6 + 0x4) = uVar4;
  *(undefined2 *)(iVar6 + 0x6) = (int)((ulong)uVar11 >> 0x10);
  *(undefined2 *)(iVar6 + 0x8) = uVar1;
  *(undefined2 *)(iVar6 + 0xa) = uVar7;
  if (bVar10) {
    pass1_1000_29af((uint)uVar11);
  }
  *(uint *)(iVar6 + 0xc) = (uint)bVar10;
  return;
}


void __cdecl16far dos3_call_op_1000_435c(UINT16 *param_1,uint param_2,undefined2 param_3,int param_4,UINT16 param_5)

{
  code *pcVar1;
  UINT16 UVar2;
  uint uVar3;
  uint extraout_DX;
  uint extraout_DX_00;
  uint extraout_DX_01;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  int iStack2;
  
  iStack2 = param_4 + 0x1;
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)((int)&USHORT_1050_1050);
  pcVar1 = (code *)swi(0x21);
  uVar3 = param_2;
  uVar5 = extraout_DX;
  (*pcVar1)();
  uVar9 = extraout_DX_00 >> 0x8;
  uVar8 = uVar3 & 0xff;
  uVar6 = uVar3 >> 0x8;
  pcVar1 = (code *)swi(0x21);
  uVar7 = uVar6;
  (*pcVar1)();
  uVar4 = extraout_DX_01;
  if ((uVar5 != extraout_DX_01) && (uVar4 = extraout_DX_01, (char)uVar6 == '\x17')) {
    uVar3 = param_2;
    uVar4 = uVar5;
  }
  UVar2 = pass1_1000_462e(uVar3 - 0x7bc,uVar4 >> 0x8,uVar4 & 0xff,uVar7,uVar8,uVar9,&iStack2,param_5,uVar4);
  if (param_1._2_2_ != 0x0) {
    *(uint *)((int)param_1 + 0x2) = uVar4;
    *param_1 = UVar2;
  }
  return;
}

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

void __cdecl16far pass1_1000_3cd8(ushort param_1,ushort param_2)

{
  free_mem_1000_407a(param_1,param_2,&stack0xfffe);
  return;
}

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

