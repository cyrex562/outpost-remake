
ulong __stdcall16far mem_op_1000_1b68(ushort param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ulong uVar1;
  
  if (*(int *)(param_3 + 0x14) != -0x4153) {
    pass1_1000_1e61(param_2,0xa,0x0,0x0);
    return (ulong)param_1 << 0x10;
  }
  uVar1 = mem_op_1000_1b9a(0x0,param_3,param_4,param_2);
  return uVar1;
}



ulong __stdcall16far mem_op_1000_1b9a(UINT16 param_1,UINT32 param_2,undefined2 param_3,UINT16 param_4)

{
  uint uVar1;
  undefined4 uVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  long lVar6;
  uint *puStack8;
  uint uStack4;
  
  *(undefined2 *)(param_2 + 0x14) = 0x0;
  uStack4 = 0x0;
  do {
    iVar5 = *(int *)(uStack4 * 0x2);
    if (iVar5 != 0x0) {
      do {
        uVar2 = *(undefined4 *)(iVar5 + 0x8);
        *(undefined2 *)((int)uVar2 + 0xc) = 0x0;
        mem_op_1000_13ce(param_4);
        iVar5 = *(int *)(iVar5 + 0x4);
      } while (*(int *)(uStack4 * 0x2) != iVar5);
    }
    uStack4 = uStack4 + 0x1;
  } while (uStack4 < 0x5);
  uVar4 = *(uint *)(param_2 + 0x12);
  uVar3 = *(uint *)(param_2 + 0x10);
  while( true ) {
    puStack8 = (uint *)CONCAT22(uVar4,uVar3);
    if ((uVar4 | uVar3) == 0x0) break;
    uVar1 = *puStack8;
    uVar4 = *(uint *)(uVar3 + 0x2);
    mem_op_1000_13ce(param_4);
    uVar3 = uVar1;
  }
  pass1_1000_20a2(param_2,param_3);
  lVar6 = mem_op_1000_13ce(param_4);
  return CONCAT22((int)((ulong)lVar6 >> 0x10),0x1);
}



BOOL16 mem_op_1000_1dfa(int param_1,byte param_2,uint param_3,uint param_4)

{
  undefined3 uVar1;
  uint uVar2;
  
  if ((param_2 & 0x4) == 0x0) {
    uVar2 = (uint)(byte)(((byte)(-(uint)((param_2 & 0x2) == 0x0) >> 0x8) & 0xfe) + 0x92) << 0x8;
  }
  else {
    uVar2 = 0x1800;
  }
  if ((param_4 == 0x0) ||
     ((param_4 & 0xff00 & (uint)(byte)(((byte)(-(uint)((param_2 & 0x4) == 0x0) >> 0x8) & 0x82) + 0x18) << 0x8) != uVar2)
     ) {
    return 0x1;
  }
  if (param_1 != 0x0) {
    uVar1 = SegmentLimit(param_4);
    if (CARRY2(param_3,param_1 - 0x1U)) {
      return 0x1;
    }
    if ((uint)uVar1 < param_3 + (param_1 - 0x1U)) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

UINT16 pass1_1000_1e61(UINT16 param_1,UINT16 param_2,UINT16 param_3,UINT16 param_4)

{
  int iVar1;
  BOOL16 BVar2;
  UINT16 UVar3;
  UINT16 UStack64;
  UINT16 UStack62;
  UINT16 UStack60;
  code *pcStack6;
  undefined *puStack4;
  UINT16 uVar3;
  
  uVar3 = (UINT16)&USHORT_1050_1050;
  UStack62 = param_3;
  UStack60 = param_4;
  UStack64 = param_2;
  puStack4 = (undefined *)&USHORT_1050_1050;
  pcStack6 = (code *)&PTR_PTR_1050_5f1a;
  if (((uint)PTR_LOOP_1050_5f1c | (uint)PTR_PTR_1050_5f1a) == 0x0) {
    pcStack6 = (code *)0x0;
    puStack4 = (undefined *)0x0;
  }
  else {
    iVar1 = mem_op_1000_21b6((UINT16)PTR_PTR_1050_5f1a,(UINT16)PTR_LOOP_1050_5f1c);
    pcStack6 = (code *)PTR_PTR_1050_5f1a;
    puStack4 = PTR_LOOP_1050_5f1c;
    if (iVar1 == 0x0) {
      PTR_PTR_1050_5f1a = (undefined *)&PTR_PTR_1050_1f7e;
      PTR_LOOP_1050_5f1c = (undefined *)&PTR_LOOP_1050_1000;
      pcStack6 = (code *)&PTR_PTR_1050_1f7e;
      puStack4 = (undefined *)&PTR_LOOP_1050_1000;
    }
  }
  if (((uint)puStack4 | (uint)pcStack6) == 0x0) {
    return 0x0;
  }
  BVar2 = msg_box_op_1000_1f24((int)&PTR_PTR_1050_5f1a,(UINT16)&USHORT_1050_1050,0x0,0x1000);
  if (BVar2 == 0x0) {
    UVar3 = (*pcStack6)(0x1000,&UStack64);
  }
  else {
    puStack4 = (undefined *)0x0;
    pcStack6 = (code *)0x0;
    UVar3 = 0x0;
  }
  if (((uint)puStack4 | (uint)pcStack6) != 0x0) {
    pass1_1000_1f68(uVar3);
  }
  return UVar3;
}



UINT16 __stdcall16far _SHI_INVOKEERRORHANDLER1(void)

{
  int iVar1;
  BOOL16 BVar2;
  UINT16 uVar2;
  UINT16 unaff_CS;
  code *pcStack6;
  UINT8 *puStack4;
  UINT16 uVar3;
  
  uVar3 = (UINT16)&USHORT_1050_1050;
  puStack4 = (UINT8 *)&USHORT_1050_1050;
  if (((uint)PTR_LOOP_1050_5f1c | (uint)PTR_PTR_1050_5f1a) == 0x0) {
    pcStack6 = (code *)0x0;
    puStack4 = (UINT8 *)0x0;
  }
  else {
    iVar1 = mem_op_1000_21b6((UINT16)PTR_PTR_1050_5f1a,(UINT16)PTR_LOOP_1050_5f1c);
    pcStack6 = (code *)PTR_PTR_1050_5f1a;
    puStack4 = PTR_LOOP_1050_5f1c;
    if (iVar1 == 0x0) {
      PTR_PTR_1050_5f1a = (undefined *)&PTR_PTR_1050_1f7e;
      PTR_LOOP_1050_5f1c = (undefined *)&PTR_LOOP_1050_1000;
      pcStack6 = (code *)&PTR_PTR_1050_1f7e;
      puStack4 = (UINT8 *)&PTR_LOOP_1050_1000;
    }
  }
  if (((uint)puStack4 | (uint)pcStack6) != 0x0) {
    BVar2 = msg_box_op_1000_1f24((int)&PTR_PTR_1050_5f1a,(UINT16)&USHORT_1050_1050,0x0,unaff_CS);
    if (BVar2 == 0x0) {
      uVar2 = (*pcStack6)();
    }
    else {
      puStack4 = (UINT8 *)0x0;
      pcStack6 = (code *)0x0;
      uVar2 = 0x0;
    }
    if (((uint)puStack4 | (uint)pcStack6) != 0x0) {
      pass1_1000_1f68(uVar3);
    }
    return uVar2;
  }
  return 0x0;
}



BOOL16 msg_box_op_1000_1f24(int param_1,UINT16 param_2,uint param_3,UINT16 param_4)

{
  int *piVar1;
  UINT16 unaff_CS;
  
  if (param_3 < *(uint *)(param_1 + 0xc)) {
    msg_box_op_1000_214c(0x0,0x0,0xd940,(UINT16)&PTR_LOOP_1050_1040,param_4);
    return 0x1;
  }
  piVar1 = (int *)(param_1 + 0xc);
  *piVar1 = *piVar1 + 0x1;
  return 0x0;
}



void __cdecl16far pass1_1000_1f68(void)

{
  PTR_LOOP_1050_5f26 = PTR_LOOP_1050_5f26 + -0x1;
  if ((int)PTR_LOOP_1050_5f26 < 0x0) {
    PTR_LOOP_1050_5f26 = (undefined *)0x0;
  }
  return;
}



// WARNING: Removing unreachable block (ram,0x10001f92)

BOOL16 __stdcall16far pass1_1000_1f7e(uint *param_1,ushort param_2)

{
  char cVar1;
  BOOL16 BVar2;
  uint uVar3;
  int iVar4;
  char *pcVar5;
  
  uVar3 = *param_1;
  if (uVar3 == 0xf) {
LAB_1000_1fb6:
    iVar4 = 0x1;
  }
  else {
    if (uVar3 < 0x10) {
      cVar1 = (char)uVar3;
      if (cVar1 == '\x02') goto LAB_1000_1fb6;
      if (('\0' < (char)(cVar1 + -0x2)) && ((char)(cVar1 + -0x3) < '\f')) {
        iVar4 = 0x0;
        goto LAB_1000_1fbe;
      }
    }
    iVar4 = 0x0;
    uVar3 = 0x1;
  }
LAB_1000_1fbe:
  pcVar5 = pass1_1000_1fd2(uVar3);
  BVar2 = msg_box_op_1000_214c(0x0,iVar4,(UINT16)pcVar5,(UINT16)((ulong)pcVar5 >> 0x10),param_2);
  return BVar2;
}



char * __cdecl16near pass1_1000_1fd2(int param_1)

{
  if (param_1 == 0x2) {
    return "Out of memory.  Please free some memory, then choose retry.";
  }
  return (char *)CONCAT22(0x1000,param_1 * 0x17 + 0x1c7a);
}



// WARNING: Removing unreachable block (ram,0x10002018)

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



void __cdecl16near pass1_1000_201c(int param_1,int param_2,UINT16 param_3)

{
  undefined2 uVar1;
  undefined4 uVar2;
  uint uVar3;
  BOOL16 BVar4;
  int iVar5;
  undefined2 uVar6;
  
  if (param_1 == 0x0) {
    *(undefined2 *)(param_2 + 0x6) = 0x0;
    *(undefined2 *)(param_2 + 0x4) = 0x0;
  }
  uVar3 = *(uint *)(param_2 + 0x6) | *(uint *)(param_2 + 0x4);
  while (uVar3 != 0x0) {
    BVar4 = pass1_1000_206c(*(uint *)(param_2 + 0x4),*(uint *)(param_2 + 0x6));
    if (BVar4 == 0x0) {
      uVar2 = *(undefined4 *)(param_2 + 0x4);
      uVar6 = (undefined2)((ulong)uVar2 >> 0x10);
      iVar5 = (int)uVar2;
      uVar1 = *(undefined2 *)(iVar5 + 0x2c);
      *(undefined2 *)(param_2 + 0x4) = *(undefined2 *)(iVar5 + 0x2a);
      *(undefined2 *)(param_2 + 0x6) = uVar1;
    }
    else {
      mem_op_1000_1b9a(0x1,*(UINT32 *)(param_2 + 0x4),*(undefined2 *)(param_2 + 0x6),param_3);
    }
    uVar3 = *(uint *)(param_2 + 0x6) | *(uint *)(param_2 + 0x4);
  }
  return;
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



BOOL16 __stdcall16far msg_box_op_1000_214c(UINT16 param_1,int param_2,UINT16 param_3,UINT16 param_4,UINT16 param_5)

{
  INT16 IVar1;
  int iVar2;
  LPCSTR text;
  
  text = (LPCSTR)(0x2 - (param_2 == 0x0) | 0x2110);
  MessageBeep16(param_5);
  do {
    IVar1 = MessageBox16((HWND16)s_tile2_bmp_1050_1538,text,(LPCSTR)0x1de8,0x1000);
    iVar2 = IVar1 + -0x1;
    if (iVar2 == 0x0) {
      return 0x0;
    }
    if ((0x0 < iVar2) && (!SBORROW2(iVar2,0x1))) {
      if (IVar1 == 0x3 || IVar1 + -0x2 < 0x1) {
        fatal_app_exit_1000_3e9e((int)s_tile2_bmp_1050_1538);
        return 0x0;
      }
      if (IVar1 == 0x4) {
        return 0x1;
      }
      if (IVar1 == 0x5) {
        return 0x0;
      }
    }
    if (((uint)text & 0x2000) == 0x0) {
      return 0x0;
    }
    text = (LPCSTR)((uint)text & 0xdfef | 0x1010);
  } while( true );
}



bool __stdcall16far mem_op_1000_21b6(UINT16 param_1,UINT16 param_2)

{
  BOOL16 BVar1;
  
  BVar1 = mem_op_1000_1dfa(0x0,0x4,param_1,param_2);
  return BVar1 == 0x0;
}



// WARNING: Removing unreachable block (ram,0x100021de)

ushort __stdcall16far pass1_1000_21d2(byte param_1,long param_2,uint param_3,uint param_4,undefined param_5)

{
  undefined3 uVar1;
  BOOL16 BVar2;
  
  BVar2 = mem_op_1000_1dfa(0x0,param_1,param_3,param_4);
  if (BVar2 == 0x0) {
    if ((param_1 & 0x4) == 0x0) {
      uVar1 = SegmentLimit((ulong)param_4);
      if ((bool)((byte)((uint3)uVar1 >> 0x10) & 0x1)) {
        if (param_2 == 0x0) {
          return 0x1;
        }
        if ((!CARRY4((ulong)param_3,param_2 - 0x1U)) && ((ulong)param_3 + (param_2 - 0x1U) <= (ulong)(uint)uVar1)) {
          return 0x1;
        }
      }
    }
    else {
      BVar2 = pass1_1000_22c0(param_3,param_4,(ushort)param_2,param_2._2_2_,_param_1);
      if (BVar2 != 0x0) {
        return 0x1;
      }
    }
  }
  return 0x0;
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



// WARNING: Removing unreachable block (ram,0x1000234c)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int * entry(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,CONTEXT *in_task_context,
           undefined2 param_7,int param_8)

{
  ushort *puVar1;
  ushort uVar2;
  char *pcVar3;
  code *pcVar4;
  ushort uVar5;
  LPCSTR str;
  ushort *puVar6;
  ushort *puVar7;
  char *pcVar8;
  CHAR *unaff_SS;
  bool bVar9;
  DWORD DVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  int iVar13;
  int iVar14;
  undefined *puVar15;
  undefined2 uVar16;
  
  uVar11 = CONCAT22(param_7,PTR_LOOP_1050_5f84);
  do {
    uVar16 = 0x0;
    InitTask16(in_task_context);
    PTR_LOOP_1050_5f84 = (undefined *)uVar11;
    if ((param_8 != 0x0) &&
       (bVar9 = param_1 < (undefined *)0xff00, param_1 = param_1 + 0x100, PTR_LOOP_1050_5f7e = (undefined *)param_5,
       bVar9)) {
      PTR_LOOP_1050_5f48 = (undefined *)param_1;
      PTR_LOOP_1050_5f4a = (undefined *)param_3;
      PTR_LOOP_1050_5f4c = (undefined *)param_4;
      PTR_LOOP_1050_5f4e = (undefined *)param_2;
      PTR_LOOP_1050_5f50 = (undefined *)param_5;
      LockSegment16((HGLOBAL16)s_tile2_bmp_1050_1538);
      PTR_LOOP_1050_5f52 = (undefined *)((ulong)uVar11 >> 0x10);
      PTR_LOOP_1050_5f84 = (undefined *)uVar11;
      DVar10 = GetVersion16();
      PTR_LOOP_1050_5f52 = (undefined *)((ulong)uVar11 >> 0x10);
      PTR_LOOP_1050_5f84 = (undefined *)uVar11;
      PTR_LOOP_1050_5f80 = (undefined *)CONCAT11((char)DVar10,(char)(DVar10 >> 0x8));
      pcVar4 = (code *)swi(0x21);
      uVar12 = uVar11;
      uVar11 = (*pcVar4)(uVar16);
      PTR_LOOP_1050_5f52 = (undefined *)((ulong)uVar12 >> 0x10);
      PTR_LOOP_1050_5f84 = (undefined *)uVar12;
      _DAT_1050_5f82 = CONCAT11((char)uVar11,(char)((ulong)uVar11 >> 0x8));
      DAT_1050_5f87 = 0x0;
      WaitEvent16(0x1000);
      PTR_LOOP_1050_5f84 = (undefined *)uVar11;
      puVar15 = PTR_LOOP_1050_5f4c;
      param_8 = InitApp16((HINSTANCE16)s_tile2_bmp_1050_1538);
      PTR_LOOP_1050_5f84 = (undefined *)uVar11;
      if (param_8 != 0x0) break;
    }
    in_task_context = (CONTEXT *)s_tile2_bmp_1050_1538;
    param_8 = CONCAT11((char)((uint)param_8 >> 0x8),0xff);
    pass1_1000_24db(param_8,0x0);
    PTR_LOOP_1050_5f84 = (undefined *)uVar11;
  } while( true );
  dos3_call_1000_23ea(param_2,param_5,0x0,(ushort)unaff_SS);
  PTR_LOOP_1050_5f84 = (undefined *)uVar11;
  pass1_1000_262c((undefined *)0x238d,(undefined *)s_tile2_bmp_1050_1538,unaff_SS,(int)s_tile2_bmp_1050_1538);
  PTR_LOOP_1050_5f84 = (undefined *)uVar11;
  pass1_1000_27d6((int)((ulong)uVar11 >> 0x10));
  uVar11 = ret_op_1000_55ac(puVar15);
  uVar5 = (ushort)uVar11;
  init_1000_23be(param_1,(ushort)((ulong)uVar11 >> 0x10),(ushort)unaff_SS);
  fn_ptr_op_1000_24cd(uVar5,0x0);
  iVar14 = 0x15;
  iVar13 = 0x15;
  pass1_1000_25a8(param_5,(int)s_tile2_bmp_1050_1538);
  pass1_1000_2913(iVar13,param_5,(uint16_t)s_tile2_bmp_1050_1538);
  str = poss_str_op_1000_28dc(iVar14);
  if (str != (PCHAR)0x0) {
    iVar13 = 0x9;
    if (*str == 'M') {
      iVar13 = 0xf;
    }
    str = str + iVar13;
    iVar13 = 0x22;
    pcVar8 = str;
    do {
      if (iVar13 == 0x0) break;
      iVar13 = iVar13 + -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar8[-0x1] = '\0';
  }
  FatalAppExit16((UINT16)s_tile2_bmp_1050_1538,str);
  FatalExit();
  puVar7 = (ushort *)&PTR_LOOP_1050_63fe;
  do {
    puVar1 = puVar7;
    puVar7 = puVar7 + 0x1;
    uVar2 = *puVar1;
    puVar6 = puVar7;
    if ((uVar2 == uVar5) || (puVar6 = (ushort *)(uVar2 + 0x1), puVar6 == (ushort *)0x0)) {
      return (int *)puVar6;
    }
    iVar13 = -0x1;
    do {
      if (iVar13 == 0x0) break;
      iVar13 = iVar13 + -0x1;
      puVar1 = puVar7;
      puVar7 = (ushort *)((int)puVar7 + 0x1);
    } while (*(char *)puVar1 != '\0');
  } while( true );
}



void __cdecl16far init_1000_23be(ushort param_1,ushort param_2,ushort param_3)

{
  init_op_1008_54aa(PTR_LOOP_1050_5f52,CONCAT22(PTR_LOOP_1050_5f50,PTR_LOOP_1050_5f4e),PTR_LOOP_1050_5f4a,
                    PTR_LOOP_1050_5f4c,(ushort)&USHORT_1050_1050,param_1,param_2,param_3);
  return;
}



// WARNING: Removing unreachable block (ram,0x10002400)
// WARNING: Removing unreachable block (ram,0x10002422)

int * __cdecl16far dos3_call_1000_23ea(ushort param_1,ushort param_2,int param_3,ushort param_4)

{
  byte *pbVar1;
  byte *pbVar2;
  byte bVar3;
  int *piVar4;
  byte *pbVar5;
  char *pcVar6;
  uint16_t uVar7;
  code **ppcVar8;
  code *pcVar9;
  uint uVar10;
  byte bVar11;
  byte bVar12;
  uint16_t uVar13;
  LPCSTR str;
  int *piVar14;
  undefined2 extraout_DX;
  undefined2 uVar15;
  uint uVar16;
  byte *pbVar17;
  int *piVar18;
  byte *pbVar19;
  char *pcVar20;
  bool bVar21;
  undefined4 uVar22;
  int iVar23;
  int iVar24;
  
  pcVar9 = (code *)swi(0x21);
  (*pcVar9)(param_3 + 0x1);
  pcVar9 = (code *)swi(0x21);
  PTR_LOOP_1050_5f6a = (undefined *)param_1;
  PTR_LOOP_1050_5f6c = (undefined *)param_2;
  (*pcVar9)();
  uVar15 = extraout_DX;
  uVar13 = pass1_1000_29dc(param_4);
  uVar22 = CONCAT22(uVar15,uVar13);
  if (*(int *)&PTR_LOOP_1050_6202 != 0x0) {
    uVar7 = *(uint16_t *)&PTR_LOOP_1050_5f7e;
    bVar21 = false;
    ppcVar8 = (code **)&PTR_LOOP_1050_6200;
    (**ppcVar8)(0x1000);
    if (bVar21) {
      iVar24 = 0x2;
      iVar23 = 0x2;
      pass1_1000_25a8(uVar7,0x1000);
      pass1_1000_2913(iVar23,uVar7,0x1000);
      str = poss_str_op_1000_28dc(iVar24);
      if (str != (PCHAR)0x0) {
        iVar23 = 0x9;
        if (*str == 'M') {
          iVar23 = 0xf;
        }
        str = str + iVar23;
        iVar23 = 0x22;
        pcVar20 = str;
        do {
          if (iVar23 == 0x0) break;
          iVar23 = iVar23 + -0x1;
          pcVar6 = pcVar20;
          pcVar20 = pcVar20 + 0x1;
        } while (*pcVar6 != '\r');
        pcVar20[-0x1] = '\0';
      }
      FatalAppExit16(0x1000,str);
      FatalExit();
      piVar18 = (int *)&PTR_LOOP_1050_63fe;
      do {
        piVar4 = piVar18;
        piVar18 = piVar18 + 0x1;
        iVar23 = *piVar4;
        piVar14 = piVar18;
        if ((iVar23 == (int)&USHORT_1050_1050) || (piVar14 = (int *)(iVar23 + 0x1), piVar14 == (int *)0x0)) {
          return piVar14;
        }
        iVar23 = -0x1;
        do {
          if (iVar23 == 0x0) break;
          iVar23 = iVar23 + -0x1;
          piVar4 = piVar18;
          piVar18 = (int *)((int)piVar18 + 0x1);
        } while (*(char *)piVar4 != '\0');
      } while( true );
    }
    ppcVar8 = (code **)&PTR_LOOP_1050_6200;
    uVar22 = (**ppcVar8)(0x1000);
  }
  iVar23 = *(int *)((int)s_New_failed_in_Op__Op_1050_0020 + 0xc);
  piVar18 = (int *)uVar22;
  if (iVar23 != 0x0) {
    pbVar19 = (byte *)0x0;
    piVar14 = (int *)uVar22;
    do {
      bVar21 = *pbVar19 == 0x0;
      piVar18 = piVar14;
      if (bVar21) break;
      iVar24 = 0xd;
      pbVar17 = (byte *)s__C_FILE_INFO__1050_5f5c;
      do {
        if (iVar24 == 0x0) break;
        iVar24 = iVar24 + -0x1;
        pbVar5 = pbVar19;
        pbVar19 = pbVar19 + 0x1;
        pbVar1 = pbVar17;
        pbVar17 = pbVar17 + 0x1;
        bVar21 = *pbVar1 == *pbVar5;
      } while (bVar21);
      if (bVar21) {
        pbVar17 = (byte *)0x5f90;
        uVar16 = (uint)((ulong)uVar22 >> 0x10);
        goto LAB_1000_2495;
      }
      iVar24 = 0x7fff;
      piVar18 = (int *)0x0;
      bVar21 = true;
      do {
        if (iVar24 == 0x0) break;
        iVar24 = iVar24 + -0x1;
        pbVar1 = pbVar19;
        pbVar19 = pbVar19 + 0x1;
        bVar21 = *pbVar1 == 0x0;
      } while (!bVar21);
      piVar14 = piVar18;
    } while (bVar21);
  }
LAB_1000_24a9:
  fn_ptr_op_1000_2594(0x620c,0x620c);
  fn_ptr_op_1000_2594(0x620c,0x620c);
  fn_ptr_op_1000_2594(0x61fe,0x61ee);
  return piVar18;
LAB_1000_2495:
  pbVar2 = pbVar19 + 0x1;
  bVar3 = *pbVar19;
  uVar10 = (uint)piVar14 & 0xff00;
  bVar11 = bVar3 + 0xbf;
  piVar18 = (int *)(uVar10 | bVar11);
  if (bVar3 < 0x41) goto LAB_1000_24a9;
  pbVar19 = pbVar19 + 0x2;
  bVar3 = *pbVar2;
  piVar14 = (int *)(uVar16 & 0xff00);
  bVar12 = bVar3 + 0xbf;
  piVar18 = (int *)((uint)piVar14 | (uint)bVar12);
  if (bVar3 < 0x41) goto LAB_1000_24a9;
  pbVar1 = pbVar17;
  pbVar17 = pbVar17 + 0x1;
  *pbVar1 = bVar12 | bVar11 * '\x10';
  uVar16 = uVar10;
  goto LAB_1000_2495;
}



// WARNING: Removing unreachable block (ram,0x10002557)

void __cdecl16far fn_ptr_op_1000_24cd(ushort param_1,int param_2)

{
  code *pcVar1;
  int iVar2;
  undefined2 uVar6;
  char cVar7;
  ushort uVar5;
  ushort uVar3;
  uint16_t uVar4;
  
  iVar2 = param_2 + 0x1;
  uVar5 = (ushort)&USHORT_1050_1050;
  PTR_LOOP_1050_5fc9._0_1_ = 0x0;
  uVar6 = 0x0;
  fn_ptr_op_1000_2594(0x68b6,0x68b6);
  fn_ptr_op_1000_2594((int)&PTR_LOOP_1050_6210,0x620c);
  ret_op_1000_55ac(param_1,uVar6,uVar5,iVar2);
  cVar7 = (char)((uint)uVar6 >> 0x8);
  fn_ptr_op_1000_2594((int)&PTR_LOOP_1050_6210,(int)&PTR_LOOP_1050_6210);
  fn_ptr_op_1000_2594((int)&PTR_LOOP_1050_6210,(int)&PTR_LOOP_1050_6210);
  dos3_op_1000_256b();
  if (cVar7 == '\0') {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
  }
  return;
}



// WARNING: Removing unreachable block (ram,0x10002513)
// WARNING: Removing unreachable block (ram,0x10002557)

void __cdecl16far pass1_1000_24db(undefined2 param_1,ushort param_2)

{
  code *pcVar1;
  int iVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  
  iVar2 = param_2 + 0x1;
  uVar4 = SUB42(&USHORT_1050_1050,0x0);
  PTR_LOOP_1050_5fc9._0_1_ = 0x0;
  uVar3 = 0x1;
  fn_ptr_op_1000_2594((int)&PTR_LOOP_1050_6210,(int)&PTR_LOOP_1050_6210);
  fn_ptr_op_1000_2594((int)&PTR_LOOP_1050_6210,(int)&PTR_LOOP_1050_6210);
  dos3_op_1000_256b(uVar3,uVar4,iVar2);
  if ((char)((uint)uVar3 >> 0x8) == '\0') {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
  }
  return;
}



// WARNING: Removing unreachable block (ram,0x10002589)

void __cdecl16near dos3_op_1000_256b(void)

{
  code *pcVar1;
  
  if (PTR_LOOP_1050_6202 != (undefined *)0x0) {
    (*(code *)PTR_LOOP_1050_6200)();
  }
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  return;
}



void __cdecl16near fn_ptr_op_1000_2594(code **param_1,code **param_2)

{
  code **ppcVar1;
  code **ppcVar2;
  code **fn_ptr_1;
  
  while (param_2 < param_1) {
    ppcVar2 = param_1 + -0x2;
    ppcVar1 = param_1 + -0x1;
    param_1 = ppcVar2;
    if (((uint)*ppcVar2 | (uint)*ppcVar1) != 0x0) {
      fn_ptr_1 = ppcVar2;
      (**fn_ptr_1)();
    }
  }
  return;
}



void __cdecl16far pass1_1000_25a8(uint16_t param_1,uint16_t param_2)

{
  pass1_1000_2913(0xfc,param_1,param_2);
  pass1_1000_2913(0xff,param_1,param_2);
  return;
}



int * exit_1000_25cc(int param_1,ushort param_2,ushort param_3)

{
  int *piVar1;
  char *pcVar2;
  LPCSTR str;
  int *piVar3;
  int *piVar4;
  char *pcVar5;
  int iVar6;
  int iVar7;
  
  iVar7 = 0x2;
  iVar6 = 0x2;
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