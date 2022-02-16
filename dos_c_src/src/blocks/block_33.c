
void __stdcall16far
pass1_1010_89f0(ushort param_1,ushort param_2,ushort param_3,ulong param_4,HINSTANCE16 param_5,ushort param_6)

{
  uint uVar1;
  undefined2 uVar2;
  uint uVar3;
  uchar *puVar4;
  uchar *puVar5;
  int iVar6;
  ulong uVar7;
  undefined4 uStack22;
  uint uStack8;
  
  uVar3 = *(uint *)(param_1 + 0x67c);
  uVar1 = *(uint *)(param_1 + 0x67e);
  if ((uVar1 | uVar3) != 0x0) {
    pass1_1008_64a2((uint *)CONCAT22(uVar1,uVar3));
    param_5 = 0x1000;
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(uVar1,uVar3),0x1000);
  }
  uVar7 = set_err_mode_1010_8b14(CONCAT22(param_2,param_1),*(ULONG *)(*(int *)(param_1 + 0xe82) * 0x4 + 0x24be),param_6)
  ;
  puVar4 = (uchar *)(uVar7 >> 0x10);
  uVar3 = (uint)uVar7;
  iVar6 = *(int *)(param_1 + 0xe82) * 0x4;
  if ((*(uint *)(iVar6 + 0x24be) == uVar3) && (*(uchar **)(iVar6 + 0x24c0) == puVar4)) {
    msg_box_op_1010_8bb4(param_1,param_2,uVar7,param_5,param_6);
  }
  mem_op_1000_179c(0x8,puVar4,0x1000);
  puVar5 = (uchar *)((uint)puVar4 | uVar3);
  if (puVar5 == (uchar *)0x0) {
    uVar3 = 0x0;
    puVar5 = (uchar *)0x0;
  }
  else {
    file_1008_6414((ulong *)CONCAT13((char)((uint)puVar4 >> 0x8),CONCAT12((char)puVar4,uVar3)),uVar7,param_6,puVar5);
  }
  *(uint *)(param_1 + 0x67c) = uVar3;
  *(uchar **)(param_1 + 0x67e) = puVar5;
  *(undefined2 *)(param_1 + 0x680) = 0x0;
  if ((*(uint *)(param_1 + 0x67e) | *(uint *)(param_1 + 0x67c)) != 0x0) {
    for (uStack8 = 0x1; (int)uStack8 < 0xa; uStack8 = uStack8 + 0x1) {
      iVar6 = uStack8 * 0xa;
      uVar2 = *(undefined2 *)(iVar6 + 0x2558);
      uVar3 = uStack8;
      pass1_1008_64c8(*(ulong **)(param_1 + 0x67c),
                      CONCAT13((char)((uint)uVar2 >> 0x8),CONCAT12((char)uVar2,*(undefined2 *)(iVar6 + 0x255a))),
                      *(int *)(iVar6 + 0x2556),uStack8,puVar5);
      uStack22 = CONCAT22(puVar5,uVar3);
      pass1_1010_86de(param_1,param_2,(uchar)param_3,CONCAT22(puVar5,uVar3));
      *(undefined4 *)(uStack8 * 0x4 + (int)param_4) = uStack22;
    }
  }
  return;
}



ulong __stdcall16far set_err_mode_1010_8b14(ulong param_1,ULONG param_2,ushort param_3)

{
  uint uVar1;
  uint uVar2;
  long lVar3;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0xe84));
  SetErrorMode16(0x1008);
  do {
    lVar3 = pass1_1008_5b12(local_a,param_3);
    if (lVar3 == 0x0) {
      SetErrorMode16(0x1008);
      return (ulong)param_2;
    }
    uVar1 = (int)param_1 + 0xa82;
    unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (ulong)uVar1),*(char **)((int)lVar3 + 0x4));
    pass1_1000_3cea(param_1 & 0xffff0000 | (ulong)uVar1,param_2);
    uVar2 = dos3_call_1000_51aa((ushort)&stack0xfffe);
  } while (uVar2 != 0x0);
  SetErrorMode16(0x1000);
  return param_1 & 0xffff0000 | (ulong)uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far msg_box_op_1010_8bb4(ushort param_1,ushort param_2,ulong param_3,HINSTANCE16 param_4,ushort param_5)

{
  char *pcVar1;
  undefined local_402 [0x400];
  
  pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),param_4);
  unk_str_op_1000_3d3e((char *)CONCAT22(param_5,local_402),pcVar1);
  pass1_1000_3cea(CONCAT22(param_5,local_402),param_3);
  pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1000);
  MessageBox16(0x1000,(LPCSTR)0x1010,(LPCSTR)pcVar1,(UINT16)((ulong)pcVar1 >> 0x10));
  PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x11100ee);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_8c32(astruct_640 *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int unaff_DI;
  astruct_79 *paVar1;
  ushort *puVar2;
  
  paVar1 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  *(undefined4 *)&param_1->field_0xa = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x8ee2;
  param_1->field_0x2 = 0x1010;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,(uchar *)((ulong)paVar1 >> 0x10),unaff_DI);
  param_1->field_0xa = (int)puVar2;
  param_1->field_0xc = (int)((ulong)puVar2 >> 0x10);
  return CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1010_8c78(ushort *param_1,ushort param_2)

{
  *param_1 = 0x8ee2;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far unk_load_str_op_1010_8c96(ulong param_1,ulong param_2,ulong param_3,ushort param_4,ushort param_5)

{
  undefined4 uVar1;
  INT16 IVar2;
  undefined4 *puVar3;
  int iVar4;
  uint uVar5;
  uchar *in_DX;
  int iVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  uchar in_AF;
  ulong uVar9;
  char *pcVar10;
  LPCSTR spec;
  WORD *valist;
  ulong uStack46;
  undefined4 local_10;
  int iStack12;
  int iStack10;
  uchar *puStack8;
  undefined2 uStack6;
  undefined2 uStack4;
  
  uVar7 = (undefined2)(param_3 >> 0x10);
  iVar6 = (int)param_3;
  uVar5 = *(uint *)(iVar6 + 0x6);
  uVar9 = (ulong)uVar5;
  spec = (LPCSTR)param_2;
  valist = (WORD *)(param_2 >> 0x10);
  if (uVar5 != 0x0) {
    uVar8 = (undefined2)(param_1 >> 0x10);
    if (uVar5 == 0x1) {
      uVar9 = param_3 & 0xffff;
      iVar4 = (int)uVar9;
      param_4 = 0x1010;
      switch(*(int *)(iVar6 + 0x4) + -0x1) {
      case 0x0:
      case 0x1:
        uVar1 = *(undefined4 *)(iVar6 + 0x8);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
        local_10 = *(undefined4 *)(iVar4 + 0xc);
        iStack12 = *(int *)(iVar4 + 0x10);
        iStack10 = iVar4;
        if (0x0 < iStack12) {
          pcVar10 = load_string_1010_847e
                              ((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),
                               (HINSTANCE16)&USHORT_1050_1028);
          uStack4 = (undefined2)((ulong)pcVar10 >> 0x10);
          uStack6 = SUB42(pcVar10,0x0);
          IVar2 = wsprintf16((LPSTR)&USHORT_1050_1028,spec,valist);
          return CONCAT22(IVar2,uStack4);
        }
        break;
      case 0x2:
        uVar1 = *(undefined4 *)(iVar6 + 0x8);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
        local_10 = *(undefined4 *)(iVar4 + 0xc);
        iStack12 = *(int *)(iVar4 + 0x10);
        if (0x0 < iStack12) {
          iStack12 = 0x0;
          uVar9 = struct_op_1030_73a8(CONCAT22(in_DX,iVar4));
          uVar9 = pass1_1028_bb24(uVar9);
          in_DX = (uchar *)(uVar9 >> 0x10);
          iStack10 = (int)uVar9;
          puVar3 = &local_10;
          puStack8 = in_DX;
          pass1_1030_627e(param_5,(uint)puVar3,(uint)in_DX,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_5,puVar3),uVar9)
          ;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)puVar3,(uint)in_DX);
          uVar1 = *(undefined4 *)((int)param_1 + 0xa);
          pass1_1010_c3c2((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),0x0,CONCAT22(in_DX,puVar3),in_DX,in_AF,param_5);
          uStack46 = CONCAT22(in_DX,puVar3);
          pcVar10 = load_string_1010_847e
                              ((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),
                               (HINSTANCE16)&USHORT_1050_1028);
          uStack4 = (undefined2)((ulong)pcVar10 >> 0x10);
          uStack6 = SUB42(pcVar10,0x0);
          wsprintf16((LPSTR)&USHORT_1050_1028,spec,valist);
          goto LAB_1010_8def;
        }
        break;
      default:
        goto switchD_1010_8e11_caseD_4;
      case 0x4:
      case 0x7:
      case 0x8:
      case 0xa:
        goto LAB_1010_8ea5;
      }
      uVar9 = ZEXT24(&local_10);
      param_4 = (ushort)&USHORT_1050_1028;
    }
    else {
      uVar9 = (ulong)(uVar5 - 0x2);
      if (uVar5 - 0x2 == 0x0) {
        iVar4 = *(int *)(iVar6 + 0x4);
        uVar5 = iVar4 - 0x4;
        if (uVar5 != 0x0) {
          uVar5 = iVar4 - 0xc;
          uVar9 = (ulong)uVar5;
          if (uVar5 != 0x0) goto LAB_1010_8ea5;
        }
        uVar1 = *(undefined4 *)(iVar6 + 0x8);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
        uVar1 = *(undefined4 *)((int)param_1 + 0xa);
        pass1_1010_c3c2((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),0x0,CONCAT22(in_DX,uVar5),in_DX,in_AF,param_5);
        uStack46 = CONCAT22(in_DX,uVar5);
        pcVar10 = load_string_1010_847e
                            ((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),
                             (HINSTANCE16)&USHORT_1050_1028);
        uStack4 = (undefined2)((ulong)pcVar10 >> 0x10);
        uStack6 = SUB42(pcVar10,0x0);
        wsprintf16((LPSTR)&USHORT_1050_1028,spec,valist);
LAB_1010_8def:
        fn_ptr_1000_17ce((astruct_18 *)(uStack46 & 0xffff | ZEXT24(in_DX) << 0x10),0x1000);
        return CONCAT22((int)uStack46,in_DX);
      }
    }
  }
LAB_1010_8ea5:
  load_string_1010_84e0
            (param_4,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,spec,(short)valist);
switchD_1010_8e11_caseD_4:
  return CONCAT22((int)uVar9,in_DX);
}



ulong __stdcall16far pass1_1010_8ebc(ulong param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1010_8c78((ushort *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_8ef2(ushort *param_1,uchar *param_2,ushort param_3)

{
  uint uVar1;
  uchar *puVar2;
  uchar *extraout_DX;
  astruct_170 *iVar3;
  int unaff_DI;
  undefined2 uVar3;
  ushort *puVar4;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_170 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  uVar1 = 0x0;
  *(undefined4 *)&iVar3->field_0x4 = 0x0;
  *(undefined4 *)&iVar3->field_0x8 = 0x0;
  *param_1 = 0x9254;
  iVar3->field_0x2 = 0x1010;
  mem_op_1000_179c(0x18,param_2,0x1000);
  puVar2 = (uchar *)((uint)param_2 | uVar1);
  if (puVar2 == (uchar *)0x0) {
    *(undefined4 *)&iVar3->field_0x4 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_2,uVar1),0x5,0x5);
    iVar3->field_0x4 = uVar1;
    iVar3->field_0x6 = extraout_DX;
    puVar2 = extraout_DX;
  }
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_3,puVar2,unaff_DI);
  iVar3->field_0x8 = (int)puVar4;
  iVar3->field_0xa = (int)((ulong)puVar4 >> 0x10);
  return;
}



void __stdcall16far pass1_1010_8f78(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_490 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_490 *)param_1;
  *param_1 = 0x9254;
  iVar4->field_0x2 = 0x1010;
  puVar1 = iVar4->field_0x4;
  uVar2 = iVar4->field_0x6;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *param_1 = 0x389a;
  iVar4->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1010_8fba(ulong param_1,ushort param_2)

{
  code **ppcVar1;
  ulong uVar2;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  astruct_411 *iVar3;
  undefined2 uVar3;
  ulong uStack14;
  ulong uStack10;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_411 *)param_1;
  ppcVar1 = (code **)((int)*iVar3->field_0x4 + 0x10);
  (**ppcVar1)();
  uStack10 = CONCAT22(extraout_DX,param_2);
  uStack14 = 0x0;
  while( true ) {
    if (uStack10 <= uStack14) {
      return;
    }
    ppcVar1 = (code **)((int)*iVar3->field_0x4 + 0x4);
    uVar2 = uStack10;
    (**ppcVar1)();
    if ((extraout_DX_00 | (uint)uVar2) != 0x0) break;
    uStack14 = uStack14 + 0x1;
  }
  ppcVar1 = (code **)((int)*iVar3->field_0x4 + 0x8);
  (**ppcVar1)();
  return;
}



void __stdcall16far pass1_1010_9044(ulong param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x10);
  (**ppcVar1)();
  return;
}



void __stdcall16far fn_ptr_1010_905e(ulong param_1,ulong param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_169 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_169 *)param_1;
  puVar1 = *(undefined4 **)&iVar4->field_0x4;
  uVar2 = *(uint *)((int)&iVar4->field_0x4 + 0x2);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  iVar4->field_0x4 = param_2;
  return;
}



void __stdcall16far pass1_1010_9092(ulong param_1,ushort param_2,ushort param_3)

{
  code **ppcVar1;
  ulong uVar2;
  uchar *extraout_DX;
  uchar *puVar3;
  uchar *puVar4;
  uint extraout_DX_00;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  undefined4 uVar8;
  ulong uStack14;
  ulong uStack6;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  uVar8 = *(undefined4 *)(iVar5 + 0x4);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar5 + 0x4) + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_2);
  uVar7 = 0xc;
  puVar3 = extraout_DX;
  mem_op_1000_179c(0xc,extraout_DX,0x1000);
  puVar4 = (uchar *)((uint)puVar3 | param_2);
  if (puVar4 == (uchar *)0x0) {
    param_2 = 0x0;
    puVar4 = (uchar *)0x0;
  }
  else {
    pass1_1010_8ef2((ushort *)CONCAT22(puVar3,param_2),puVar4,param_3);
  }
  for (uStack14 = 0x0; uStack14 < uStack6; uStack14 = uStack14 + 0x1) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar5 + 0x4) + 0x4);
    uVar2 = uStack6;
    (**ppcVar1)(0x1000,*(undefined4 *)(iVar5 + 0x4),uStack14,uVar7,uVar8);
    if ((extraout_DX_00 | (uint)uVar2) != 0x0) {
      ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_2 + 0x4) + 0xc);
      (**ppcVar1)(0x1000,*(undefined4 *)(param_2 + 0x4),(uint)uVar2,extraout_DX_00);
    }
  }
  return;
}



void __stdcall16far pass1_1010_9130(ulong param_1,uchar *param_2,uint param_3,uint param_4,ushort param_5,uchar param_6)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  pass1_1030_1d58(*(ulong *)((int)param_1 + 0x4));
  if ((uchar *)(param_4 | param_3) != (uchar *)0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x8);
    pass1_1010_c3c2((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),(ulong)param_2,CONCAT22(param_4,param_3),
                    (uchar *)(param_4 | param_3),param_6,param_5);
    return;
  }
  *param_2 = '\0';
  return;
}



void __stdcall16far struct_1010_9172(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_249 *iVar4;
  undefined2 uVar4;
  astruct_75 *paVar5;
  undefined4 uVar6;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar4 = (astruct_249 *)param_1;
  puVar1 = iVar4->field_0x4;
  uVar2 = iVar4->field_0x6;
  paVar5 = (astruct_75 *)CONCAT22(uVar2,puVar1);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    paVar5 = (astruct_75 *)(**ppcVar3)();
  }
  mem_op_1000_179c(0x18,(uchar *)((ulong)paVar5 >> 0x10),0x1000);
  if (paVar5 == (astruct_75 *)0x0) {
    uVar6 = 0x0;
  }
  else {
    uVar6 = struct_op_1030_1cd8(paVar5,0x5,0x5);
  }
  iVar4->field_0x4 = (undefined4 *)uVar6;
  iVar4->field_0x6 = (uint)((ulong)uVar6 >> 0x10);
  return;
}



void __stdcall16far pass1_1010_91cc(ulong param_1)

{
  code **ppcVar1;
  undefined2 uVar2;
  long lVar3;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x10);
  lVar3 = (**ppcVar1)();
  if (lVar3 != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x8);
    (**ppcVar1)();
  }
  return;
}



void __stdcall16far pass1_1010_9210(ulong param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0xc);
  (**ppcVar1)();
  return;
}



ushort * __stdcall16far pass1_1010_922e(ushort *param_1,byte param_2)

{
  pass1_1010_8f78(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1010_9258(ushort *param_1)

{
  struct_1010_383a(param_1);
  *param_1 = 0x958e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
  return param_1;
}



void __stdcall16far pass1_1010_927a(ushort *param_1)

{
  *param_1 = 0x958e;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
  pass1_1010_3880(param_1);
  return;
}



ulong __stdcall16far
pass1_1010_9298(astruct_79 *param_1,astruct_79 *param_2,ushort param_3,ushort param_4,uchar *param_5,ushort param_6)

{
  struct_1010_2cd2(param_1,param_2,param_3,param_6);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x9566;
  param_1->field_0x2 = 0x1010;
  mem_op_1000_179c(0x20c,param_5,0x1000);
  param_1[0x9].field_0x2 = param_4;
  *(uchar **)&param_1[0x9].field_0x4 = param_5;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_5,param_1[0x9].field_0x2),(WNDCLASS16 *)0x0,0x20c);
  return CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1010_92e6(ushort *param_1,ushort param_2)

{
  *param_1 = 0x9566;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
  pass1_1010_2db2(param_1,param_2);
  return;
}



void __stdcall16far pass1_1010_9304(ushort param_1,ushort param_2,int param_3,uint param_4,uchar *param_5)

{
  if (param_3 != 0x0) {
    mem_op_1000_179c(param_3 << 0x2,param_5,0x1000);
    return;
  }
  mem_op_1000_179c(0x1a,param_5,0x1000);
  if (((uint)param_5 | param_4) != 0x0) {
    pass1_1010_9258((ushort *)CONCAT22(param_5,param_4));
    return;
  }
  return;
}



void __stdcall16far pass1_1010_9348(ulong param_1,int param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  *(int *)(param_2 * 0x8 + 0x319e) = param_2;
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(int *)(iVar1 + 0x16) = param_2 * 0x8 + 0x3198;
  *(undefined2 *)(iVar1 + 0x18) = (int)&USHORT_1050_1050;
  *(int *)(iVar1 + 0x12) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_9372(ulong *param_1,uint param_2,int param_3,int param_4,int param_5)

{
  code **ppcVar1;
  char cVar2;
  uint uVar3;
  uint uVar4;
  int unaff_DI;
  ushort unaff_SS;
  bool bVar5;
  undefined4 uVar6;
  ulong uVar7;
  
  if (0x0 < param_4) {
    if (_PTR_LOOP_1050_3528 == (ushort *)0x0) {
      ppcVar1 = (code **)((int)*param_1 + 0x18);
      uVar6 = (**ppcVar1)();
      _PTR_LOOP_1050_3528 =
           mixed_1010_20ba(_PTR_LOOP_1050_0ed0,(ushort)uVar6,unaff_SS,(uchar *)((ulong)uVar6 >> 0x10),unaff_DI);
    }
    uVar6 = *(undefined4 *)((int)param_1 + 0xc);
    uVar7 = pass1_1010_2e02((ulong)_PTR_LOOP_1050_3528,*(int *)((int)uVar6 + 0x12));
    uVar3 = param_2 + 0x1;
    uVar4 = param_3 + (uint)(0xfffe < param_2);
    for (cVar2 = ((char)param_4 + -0x1) * '\x04'; cVar2 != '\0'; cVar2 = cVar2 + -0x1) {
      bVar5 = CARRY2(uVar3,uVar3);
      uVar3 = uVar3 * 0x2;
      uVar4 = uVar4 * 0x2 + (uint)bVar5;
    }
    pass1_1010_2e30((ulong)_PTR_LOOP_1050_3528,uVar3 | (uint)uVar7,uVar4 | (uint)(uVar7 >> 0x10),param_5);
  }
  return;
}



void __stdcall16far pass1_1010_93f0(ulong param_1,ushort param_2)

{
  undefined *puVar1;
  undefined2 uVar2;
  int iVar3;
  undefined2 uVar4;
  ushort *puVar5;
  undefined local_1c [0x1a];
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x56) == 0x0) {
    puVar5 = pass1_1010_9258((ushort *)CONCAT22(param_2,local_1c));
    uVar2 = (undefined2)((ulong)puVar5 >> 0x10);
    puVar1 = local_1c;
    pass1_1010_398e((ulong *)CONCAT22(param_2,puVar1),0x0,0x0,0x0,(ushort)puVar1);
    *(ushort *)(iVar3 + 0x56) = (ushort)puVar1;
    *(undefined2 *)(iVar3 + 0x58) = uVar2;
    pass1_1010_927a((ushort *)CONCAT22(param_2,local_1c));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * __stdcall16far load_string_1010_9432(HINSTANCE16 param_1)

{
  char *pcVar1;
  
  pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),param_1);
  return pcVar1;
}



void __stdcall16far pass1_1010_944e(ushort param_1,ushort param_2,int param_3)

{
  code **ppcVar1;
  ulong uVar2;
  
  if (*(long *)(param_1 + 0x56) == 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)CONCAT22(param_2,param_1) + 0x10);
    (**ppcVar1)();
  }
  uVar2 = pass1_1010_2e02(CONCAT22(param_2,param_1),param_3);
  pass1_1010_2e5c(param_1,param_2,uVar2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool __stdcall16far
pass1_1010_9488(ushort param_1,ushort param_2,ulong param_3,uchar *param_4,int param_5,ushort param_6)

{
  ushort uVar1;
  ushort uVar2;
  ushort uVar3;
  ushort *puVar4;
  ushort uVar5;
  ushort uStack10;
  
  uVar5 = *(ushort *)((int)param_3 + 0x12);
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_4,param_5);
  uVar2 = (ushort)((ulong)puVar4 >> 0x10);
  uVar1 = uVar5 - 0x32;
  uStack10 = (ushort)puVar4;
  uVar3 = uVar2;
  if (uVar1 == 0x0) {
    pass1_1010_a5ca(uStack10,uVar2,0x32,0x0,uVar2);
    if (uVar1 != 0x0) {
      return false;
    }
    uVar5 = 0x4d;
  }
  else {
    uVar1 = uVar5 - 0x3f;
    if (uVar1 == 0x0) {
      pass1_1010_a5ca(uStack10,uVar2,0x3f,0x0,uVar2);
      if (uVar1 != 0x0) {
        return false;
      }
      uVar5 = 0x4e;
    }
  }
  pass1_1010_a5ca(uStack10,uVar2,uVar5,uVar1,uVar3);
  return uVar1 == 0x0;
}



ushort __stdcall16far pass1_1010_9502(ulong param_1)

{
  undefined4 uVar1;
  
  uVar1 = *(undefined4 *)((int)param_1 + 0x16);
  return *(ushort *)((int)uVar1 + 0x2);
}



ushort __stdcall16far pass1_1010_9514(void)

{
  return 0x31;
}



ushort * __stdcall16far pass1_1010_951a(ushort *param_1,byte param_2)

{
  pass1_1010_927a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1010_9540(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_92e6(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1010_95aa(astruct_629 *param_1,ushort param_2,ushort param_3)

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  param_1->field_0x18 = 0x0;
  param_1->field_0x1a = 0x0;
  param_1->field_0x1c = 0xa;
  param_1->field_0x1e = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xa1c8;
  param_1->field_0x2 = 0x1010;
  return;
}



void __stdcall16far pass1_1010_95f8(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_491 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_491 *)param_1;
  *param_1 = 0xa1c8;
  iVar4->field_0x2 = 0x1010;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0xe;
  uVar2 = iVar4->field_0x10;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0x12;
  uVar2 = iVar4->field_0x14;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



void __stdcall16far pass1_1010_9674(ulong param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x12);
  uVar2 = *(uint *)(iVar4 + 0x14);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)(iVar4 + 0x12) = 0x0;
  return;
}



void __stdcall16far pass1_1010_96a8(ulong param_1,int param_2)

{
  int *piVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  piVar1 = (int *)((int)param_1 + 0x1e);
  *piVar1 = *piVar1 - param_2;
  if (*piVar1 < 0x0) {
    *(undefined2 *)((int)param_1 + 0x1e) = 0x0;
  }
  return;
}



ushort __stdcall16far pass1_1010_96c2(ulong param_1)

{
  return *(ushort *)((int)param_1 + 0x1e);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far pass1_1010_96d0(ulong param_1)

{
  int *piVar1;
  int iVar2;
  astruct_690 *iVar3;
  undefined2 uVar3;
  ulong uVar4;
  int iStack8;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_690 *)param_1;
  if (iVar3->field_0x1a != 0x0) {
    if (0x0 < iVar3->field_0x1c) {
      piVar1 = &iVar3->field_0x1c;
      *piVar1 = *piVar1 + -0x1;
    }
    if ((iVar3->field_0x1c == 0x0) && (iVar3->field_0x1e != 0x0)) {
      iStack8 = 0x1;
      uVar4 = pass1_1030_8326();
      iVar2 = (int)(uVar4 >> 0x10);
      if ((iVar2 != 0x0) || (0x32 < (uint)uVar4)) {
        iStack8 = 0x5;
      }
      if ((iVar2 != 0x0) || (0x3c < (uint)uVar4)) {
        iStack8 = 0xa;
      }
      if (iVar3->field_0x1e < iStack8) {
        iStack8 = iVar3->field_0x1e;
      }
      piVar1 = &iVar3->field_0x1e;
      *piVar1 = *piVar1 - iStack8;
      if (*piVar1 < 0x0) {
        iVar3->field_0x1e = 0x0;
      }
      if (0x0 < iVar3->field_0x1e) {
        return iStack8;
      }
      return -0x1;
    }
  }
  return 0x0;
}



void __stdcall16far pass1_1010_9766(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  int in_AX;
  undefined2 uVar1;
  
  uVar1 = (undefined2)(param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x1a) = 0x1;
  pass1_1010_a0a0(param_1,param_2,param_3,param_4);
  pass1_1010_9f8c(param_1,0x80,param_4);
  *(int *)((int)param_1 + 0x1e) = in_AX * 0x32;
  return;
}



void __stdcall16far pass1_1010_9794(ulong param_1,ushort param_2)

{
  int iVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  uint uVar4;
  astruct_251 *puVar5;
  undefined4 *puVar6;
  uchar *puVar7;
  uchar *extraout_DX;
  undefined2 extraout_DX_00;
  undefined2 uVar8;
  uint extraout_DX_01;
  astruct_250 *iVar9;
  undefined2 uVar9;
  undefined8 local_a;
  
  uVar9 = (undefined2)(param_1 >> 0x10);
  iVar9 = (astruct_250 *)param_1;
  if (iVar9->field_0x18 == 0x0) {
    iVar9->field_0x18 = 0x1;
    puVar6 = iVar9->field_0xe;
    uVar4 = *(uint *)((int)&iVar9->field_0xe + 0x2);
    puVar7 = (uchar *)(uVar4 | (uint)(undefined4 *)puVar6);
    if (puVar7 != (uchar *)0x0) {
      ppcVar2 = (code **)*(undefined4 *)puVar6;
      (**ppcVar2)();
      puVar7 = extraout_DX;
    }
    mem_op_1000_179c(0xc,puVar7,0x1000);
    uVar4 = (uint)puVar6;
    if (((uint)puVar7 | uVar4) == 0x0) {
      uVar4 = 0x0;
      uVar8 = 0x0;
    }
    else {
      set_struct_1008_574a((astruct_21 *)((ulong)puVar6 & 0xffff | ZEXT24(puVar7) << 0x10));
      uVar8 = extraout_DX_00;
    }
    *(uint *)&iVar9->field_0xe = uVar4;
    *(undefined2 *)((int)&iVar9->field_0xe + 0x2) = uVar8;
    pass1_1008_5784((ulong *)CONCAT22(param_2,&local_a),(ulong)iVar9->field_0xa);
    while( true ) {
      puVar5 = (astruct_251 *)&local_a;
      pass1_1008_5b12(puVar5,param_2);
      if ((extraout_DX_01 | (uint)puVar5) == 0x0) break;
      iVar1 = puVar5->field_0x4;
      if ((iVar1 == 0x3e) || (iVar1 == 0x41)) {
        puVar6 = iVar9->field_0xa;
        *(undefined2 *)((int)puVar6 + 0xa) = 0x0;
        puVar6 = iVar9->field_0xa;
        ppcVar2 = (code **)((int)*iVar9->field_0xa + 0xc);
        (**ppcVar2)();
        puVar3 = iVar9->field_0xa;
        *(undefined2 *)((int)puVar3 + 0xa) = 0x1;
        local_a._4_4_ = 0x0;
        ppcVar2 = (code **)((int)*iVar9->field_0xe + 0x4);
        (**ppcVar2)(0x1008,iVar9->field_0xe,CONCAT22(extraout_DX_01,puVar5),puVar6);
      }
    }
  }
  return;
}

