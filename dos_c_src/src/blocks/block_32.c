

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1010_7818(ulong param_1,ulong param_2)

{
  undefined4 uVar1;
  ushort uVar2;
  BOOL16 BVar3;
  ushort uVar4;
  ushort uStack6;
  
  uVar4 = (ushort)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((ushort)param_1 + 0x14);
  uVar2 = pass1_1010_b028((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),param_2);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x1e);
  if (BVar3 == 0x0) {
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0xb);
    if (((BVar3 == 0x0) && (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x20), BVar3 == 0x0)) &&
       (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x1c), BVar3 == 0x0)) {
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x2);
      if ((BVar3 != 0x0) || (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x13), BVar3 != 0x0)) {
        return 0x5;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x11);
      if ((BVar3 != 0x0) || (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x12), BVar3 != 0x0)) {
        return 0x4;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x5);
      if (BVar3 != 0x0) {
        return 0x6;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x6);
      if (BVar3 != 0x0) {
        return 0x7;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x4);
      if (BVar3 != 0x0) {
        return 0x10;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x3);
      if (BVar3 != 0x0) {
        return 0x11;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x19);
      if (BVar3 != 0x0) {
        return 0x15;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x1d);
      if (BVar3 != 0x0) {
        return 0x16;
      }
      uVar2 = pass1_1010_7d7e((ushort)param_1,uVar4,0x1,uVar2);
      if (uVar2 == 0x0) {
        return 0x0;
      }
      return 0xc;
    }
    uStack6 = 0x1;
  }
  else {
    uStack6 = 0x18;
  }
  return uStack6;
}



void __stdcall16far ui_op_1010_79aa(undefined4 param_1,int param_2,long param_3,undefined2 param_4)

{
  undefined4 uVar1;
  undefined *puVar2;
  uint extraout_DX;
  undefined2 uVar3;
  long lStack18;
  long lStack14;
  undefined local_a [0x8];
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  if ((*(long *)((int)param_1 + 0x1c) != 0x0) && ((param_3 != 0x0 || (param_2 != 0x0)))) {
    pass1_1008_5784((ulong *)CONCAT22(param_4,local_a),*(ulong *)((int)param_1 + 0x1c));
    lStack18 = 0x0;
    do {
      puVar2 = local_a;
      pass1_1008_5b12(puVar2,param_4);
      lStack14 = CONCAT22(extraout_DX,puVar2);
      if ((extraout_DX | (uint)puVar2) == 0x0) goto LAB_1010_7a49;
      if (((param_2 == 0x0) && (*(long *)(puVar2 + 0x4) == param_3)) ||
         ((param_3 == 0x0 && (uVar1 = *(undefined4 *)(puVar2 + 0x8), *(int *)((int)uVar1 + 0xa) == param_2)))) break;
    } while ((*(long *)(puVar2 + 0x4) != param_3) ||
            (uVar1 = *(undefined4 *)(puVar2 + 0x8), *(int *)((int)uVar1 + 0xa) != param_2));
    lStack18 = lStack14;
LAB_1010_7a49:
    if (lStack18 != 0x0) {
      SetFocus16(0x1008);
      BringWindowToTop16((HWND16)s_tile2_bmp_1050_1538);
      return;
    }
  }
  return;
}



void __stdcall16far show_win_1010_7a76(ulong param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  undefined2 unaff_SS;
  long lVar3;
  undefined local_a [0x8];
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0x20) == 0x0) {
    *(undefined2 *)(iVar1 + 0x20) = 0x1;
    pass1_1008_5784((ulong *)CONCAT22(unaff_SS,local_a),*(ulong *)(iVar1 + 0x1c));
    while( true ) {
      lVar3 = pass1_1008_5b12(local_a,unaff_SS);
      if (lVar3 == 0x0) break;
      ShowWindow16(0x1008,0x0);
    }
  }
  return;
}



void __stdcall16far show_window_1010_7ace(ulong param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  long lVar3;
  undefined local_a [0x8];
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0x20) != 0x0) {
    *(undefined2 *)(iVar1 + 0x20) = 0x0;
    pass1_1008_5784((ulong *)CONCAT22(param_2,local_a),*(ulong *)(iVar1 + 0x1c));
    while( true ) {
      lVar3 = pass1_1008_5b12(local_a,param_2);
      if (lVar3 == 0x0) break;
      ShowWindow16(0x1008,0x1);
    }
  }
  return;
}



ulong __stdcall16far destroy_window_1010_7b26(ulong param_1,long param_2,UINT16 param_3,uint param_4)

{
  uint uVar1;
  UCHAR *puVar2;
  uint extraout_DX;
  int iVar2;
  UINT16 uVar4;
  UCHAR local_a [0x8];
  
  uVar4 = (UINT16)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  uVar1 = *(uint *)(iVar2 + 0x1e) | *(uint *)(iVar2 + 0x1c);
  if (uVar1 != 0x0) {
    pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)(iVar2 + 0x1c));
    do {
      puVar2 = local_a;
      pass1_1008_5b12(puVar2,param_3);
      param_4 = extraout_DX | (uint)puVar2;
      if (param_4 == 0x0) break;
    } while (*(long *)(puVar2 + 0x4) != param_2);
    uVar1 = extraout_DX | (uint)puVar2;
    if (uVar1 != 0x0) {
      uVar1 = DestroyWindow16(0x1008);
    }
  }
  return CONCAT22(uVar1,param_4);
}



void __stdcall16far pass1_1010_7b8c(ulong param_1,int param_2,ushort param_3)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined4 uVar4;
  undefined *puVar5;
  uint extraout_DX;
  int iVar6;
  undefined2 uVar7;
  undefined4 uStack14;
  undefined local_a [0x8];
  
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  if ((*(uint *)(iVar6 + 0x1e) | *(uint *)(iVar6 + 0x1c)) != 0x0) {
    pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)(iVar6 + 0x1c));
    do {
      puVar5 = local_a;
      pass1_1008_5b12(puVar5,param_3);
      uStack14 = CONCAT22(extraout_DX,puVar5);
      if ((extraout_DX | (uint)puVar5) == 0x0) break;
      uVar4 = *(undefined4 *)(puVar5 + 0x8);
    } while (*(int *)((int)uVar4 + 0x6) != param_2);
    if ((extraout_DX | (uint)puVar5) != 0x0) {
      ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0x1c) + 0xc);
      (**ppcVar3)(0x1008,*(undefined4 *)(iVar6 + 0x1c),uStack14);
    }
    uVar4 = *(undefined4 *)(iVar6 + 0x1c);
    if (*(int *)((int)uVar4 + 0x8) == 0x0) {
      puVar1 = (undefined4 *)*(uint *)(iVar6 + 0x1c);
      uVar2 = *(uint *)(iVar6 + 0x1e);
      if ((uVar2 | (uint)puVar1) != 0x0) {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)(0x1008,puVar1,uVar2,0x1,puVar1,uVar2,puVar1,uVar2);
      }
      *(undefined4 *)(iVar6 + 0x1c) = 0x0;
    }
  }
  return;
}



void __stdcall16far send_msg_1010_7c42(ulong param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  long lVar3;
  undefined local_a [0x8];
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((*(uint *)(iVar1 + 0x1e) | *(uint *)(iVar1 + 0x1c)) != 0x0) {
    pass1_1008_5784((ulong *)CONCAT22(param_2,local_a),*(ulong *)(iVar1 + 0x1c));
    while( true ) {
      lVar3 = pass1_1008_5b12(local_a,param_2);
      if (lVar3 == 0x0) break;
      SendMessage16(0x1008,0x0,0x0,0x11100eb);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far send_msg_1010_7c9e(ulong param_1,int param_2,ushort param_3)

{
  BOOL16 BVar1;
  int iVar2;
  undefined2 uVar3;
  long lVar4;
  ulong uVar5;
  undefined local_a [0x8];
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (((*(uint *)(iVar2 + 0x1e) | *(uint *)(iVar2 + 0x1c)) != 0x0) && (param_2 != 0x0)) {
    pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)(iVar2 + 0x1c));
    while( true ) {
      lVar4 = pass1_1008_5b12(local_a,param_3);
      uVar3 = (undefined2)((ulong)lVar4 >> 0x10);
      if (lVar4 == 0x0) break;
      if (*(long *)((int)lVar4 + 0x4) != 0x0) {
        uVar5 = struct_op_1030_73a8(*(ulong *)((int)lVar4 + 0x4));
        BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((int)uVar5 + 0xc),param_2);
        if (BVar1 != 0x0) {
          SendMessage16(0x1008,0x0,0x0,0x11100eb);
        }
      }
    }
  }
  return;
}



undefined2 __stdcall16far pass1_1010_7d38(ushort param_1,ushort param_2,int param_3,ushort param_4,ushort param_5)

{
  undefined4 local_e;
  undefined2 uStack10;
  undefined2 local_8;
  undefined local_6 [0x2];
  undefined local_4 [0x2];
  
  local_e = *(undefined4 *)(param_3 + 0xc);
  uStack10 = *(undefined2 *)(param_3 + 0x10);
  pass1_1008_3eb4((ushort *)CONCAT22(param_5,&local_e),(ushort *)CONCAT22(param_5,&local_8),
                  (ushort *)CONCAT22(param_5,local_6),(ushort *)CONCAT22(param_5,local_4));
  return local_8;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1010_7d7e(ushort param_1,ushort param_2,int param_3,int param_4)

{
  BOOL16 BVar1;
  
  if (param_3 != 0x3) {
    if ((param_4 < 0xa) || (0x7f < param_4)) {
      return 0x0;
    }
    BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,param_4,0x3c);
    if (BVar1 != 0x0) {
      return 0x0;
    }
    if (((param_4 == 0x6a) && (param_3 != 0x4)) && (param_3 != 0x5)) {
      return 0x0;
    }
  }
  return 0x1;
}



ulong __stdcall16far pass1_1010_7dc6(ulong param_1,byte param_2)

{
  undefined2 unaff_SS;
  
  param_1 = param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xa);
  pass1_1010_6bb2((ushort *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1010_7dd2(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_7e40(ulong *param_1,uchar *param_2,int param_3,ushort param_4)

{
  undefined4 uVar1;
  astruct_652 *puVar2;
  undefined2 uVar2;
  ushort *puVar3;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  puVar2 = (astruct_652 *)param_1;
  *param_1 = 0x0;
  puVar2->field_0x67c = 0x0;
  puVar2->field_0x680 = 0x0;
  puVar2->field_0xe82 = 0x0;
  puVar2->field_0xe84 = 0x0;
  *(undefined4 *)&puVar2->field_0xe88 = 0x0;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&puVar2->field_0x4),(WNDCLASS16 *)0x0,0x228)
  ;
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&puVar2->field_0x22c),(WNDCLASS16 *)0x0,
                  0x228);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&puVar2->field_0x454),(WNDCLASS16 *)0x0,
                  0x228);
  *(undefined *)&puVar2->field_0x682 = 0x0;
  *(undefined *)&puVar2->field_0xa82 = 0x0;
  _PTR_LOOP_1050_14cc = param_1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_2,param_3);
  puVar2->field_0xe88 = (int)puVar3;
  puVar2->field_0xe8a = (int)((ulong)puVar3 >> 0x10);
  uVar1 = *(undefined4 *)&puVar2->field_0xe88;
  puVar2->field_0xe84 = *(undefined4 *)((int)uVar1 + 0x64);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_7efc(ulong *param_1,ushort param_2)

{
  uint uVar1;
  uint uVar2;
  undefined4 *puVar3;
  code **ppcVar4;
  astruct_448 *iVar5;
  undefined2 uVar5;
  astruct_18 *paStack8;
  int iStack4;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_448 *)param_1;
  uVar1 = iVar5->field_0x67c;
  uVar2 = iVar5->field_0x67e;
  paStack8 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_64a2((uint *)CONCAT22(uVar2,uVar1));
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paStack8,0x1000);
  }
  for (iStack4 = 0x0; iStack4 < 0x8a; iStack4 = iStack4 + 0x1) {
    puVar3 = (undefined4 *)*(uint *)(&iVar5->field_0x4 + iStack4 * 0x4);
    uVar1 = *(uint *)(&iVar5->field_0x4 + iStack4 * 0x4 + 0x2);
    if ((uVar1 | (uint)puVar3) != 0x0) {
      ppcVar4 = (code **)*puVar3;
      (**ppcVar4)(param_2,puVar3,uVar1,0x1);
    }
    puVar3 = (undefined4 *)*(uint *)(&iVar5->field_0x22c + iStack4 * 0x4);
    uVar1 = *(uint *)(&iVar5->field_0x22c + iStack4 * 0x4 + 0x2);
    if ((uVar1 | (uint)puVar3) != 0x0) {
      ppcVar4 = (code **)*puVar3;
      (**ppcVar4)(param_2,puVar3);
    }
    puVar3 = (undefined4 *)*(uint *)(&iVar5->field_0x454 + iStack4 * 0x4);
    uVar1 = *(uint *)(&iVar5->field_0x454 + iStack4 * 0x4 + 0x2);
    if ((uVar1 | (uint)puVar3) != 0x0) {
      ppcVar4 = (code **)*puVar3;
      (**ppcVar4)(param_2,puVar3);
    }
  }
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  _PTR_LOOP_1050_14cc = 0x0;
  return;
}



void __stdcall16far pass1_1010_7fd6(ulong param_1)

{
  uint uVar1;
  uint uVar2;
  astruct_489 *iVar3;
  undefined2 uVar3;
  astruct_18 *paStack6;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_489 *)param_1;
  uVar1 = iVar3->field_0x67c;
  uVar2 = iVar3->field_0x67e;
  paStack6 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_64a2((uint *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack6,0x1000);
  }
  *(undefined4 *)&iVar3->field_0x67c = 0x0;
  iVar3->field_0x680 = 0x0;
  return;
}



void __stdcall16far pass1_1010_8018(ulong param_1,uint param_2,uchar *param_3,ushort param_4)

{
  int iVar1;
  ushort *uVar2;
  
  if (*(int *)(param_2 * 0xa + 0x1fa0) != 0x0) {
    pass1_1010_878c((astruct_87 **)param_1,*(int *)(param_2 * 0xa + 0x1fa0),param_4);
    uVar2 = (ushort *)(param_1 >> 0x10);
    if (*(long *)((int)param_1 + 0x67c) != 0x0) {
      iVar1 = param_2 * 0xa;
      pass1_1008_64c8(*(ulong **)((int)param_1 + 0x67c),
                      CONCAT22(*(undefined2 *)(iVar1 + 0x1fa6),*(undefined2 *)(iVar1 + 0x1fa8)),*(int *)(iVar1 + 0x1fa4)
                      ,param_2,param_3);
      return;
    }
  }
  return;
}



void __stdcall16far pass1_1010_8096(ulong *param_1,int param_2)

{
  ushort uVar1;
  ushort uVar2;
  ushort uVar3;
  ushort uVar4;
  ushort unaff_SS;
  char *pcVar5;
  ushort *puVar6;
  undefined local_306 [0x100];
  undefined local_206 [0x100];
  undefined local_106 [0x104];
  
  uVar4 = (ushort)((ulong)param_1 >> 0x10);
  uVar3 = (ushort)param_1;
  str_1000_4d58(*(char **)(*(int *)(uVar3 + 0xe82) * 0x4 + 0x2526),(char *)0x0,0x0,CONCAT22(unaff_SS,local_206),
                (WNDCLASS16 *)CONCAT22(unaff_SS,local_306));
  unk_str_op_1000_3d3e((char *)CONCAT22(unaff_SS,local_106),(char *)CONCAT22(unaff_SS,local_206));
  if (param_2 == 0x2) {
    puVar6 = &USHORT_1050_3194;
  }
  else {
    puVar6 = &USHORT_1050_3196;
  }
  pass1_1000_3cea(CONCAT22(unaff_SS,local_106),(ULONG)puVar6);
  pass1_1000_3cea(CONCAT22(unaff_SS,local_106),CONCAT22(unaff_SS,local_306));
  pcVar5 = (char *)set_err_mode_1010_8b14((ulong)param_1,CONCAT22(unaff_SS,local_106),unaff_SS);
  uVar2 = (ushort)((ulong)pcVar5 >> 0x10);
  if (((undefined *)pcVar5 == local_106) && (uVar2 == unaff_SS)) {
    msg_box_op_1010_8bb4(uVar3,uVar4,(ulong)pcVar5 & 0xffff | (ulong)uVar2 << 0x10,0x1000,unaff_SS);
  }
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  uVar1 = str_op_1008_60e8(pcVar5,uVar2);
  *(ushort *)param_1 = uVar1;
  *(ushort *)(uVar3 + 0x2) = uVar2;
  return;
}



void __stdcall16far pass1_1010_8170(astruct_87 **param_1,int param_2,uchar *param_3,ushort param_4)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  uVar1 = *(uint *)(iVar2 + 0x680);
  iVar3 = param_2 * 0x10;
  if (*(uint *)(iVar3 + 0x16) != uVar1) {
    pass1_1010_8096((ulong *)param_1,*(int *)(iVar3 + 0x16));
    pass1_1010_878c(param_1,*(int *)(iVar3 + 0x16),param_4);
    if (*(long *)(iVar2 + 0x67c) == 0x0) {
      return;
    }
  }
  iVar3 = param_2 * 0x10;
  pass1_1008_6562(*(ulong **)(iVar2 + 0x67c),CONCAT22(*(undefined2 *)(iVar3 + 0x1c),*(undefined2 *)(iVar3 + 0x1e)),
                  *(int *)(iVar3 + 0x1a),uVar1,param_3);
  return;
}



void __stdcall16far pass1_1010_81f6(HINSTANCE16 param_1,ushort param_2,astruct_87 **param_3,long param_4,int param_5)

{
  ushort uVar1;
  undefined2 uVar2;
  ushort uStack12;
  ulong uStack10;
  
  if (param_4 == 0x8000001) {
    uStack10 = (ulong)param_3 & 0xffff0000 | (ulong)((ushort)param_3 + 0x22c);
    uStack12 = 0xfa;
  }
  else {
    if (param_4 == 0x8000002) {
      uStack10 = (ulong)param_3 & 0xffff0000 | (ulong)((ushort)param_3 + 0x454);
      uStack12 = 0xfc;
    }
    else {
      uStack10 = (ulong)param_3 & 0xffff0000 | (ulong)((ushort)param_3 + 0x4);
      uStack12 = 0x2;
    }
  }
  uVar2 = (undefined2)(uStack10 >> 0x10);
  uVar1 = param_3._2_2_;
  if (*(long *)(param_5 * 0x4 + (int)uStack10) == 0x0) {
    if ((0x0 < param_5) && (param_5 < 0xa)) {
      pass1_1010_89f0((ushort)param_3,param_3._2_2_,uStack12,uStack10,param_1,param_2);
      return;
    }
    if (param_5 < 0xa) {
      return;
    }
    if (0x7f < param_5) {
      return;
    }
    if (*(long *)((int)uStack10 + 0x14) == 0x0) {
      pass1_1010_89f0((ushort)param_3,param_3._2_2_,uStack12,uStack10,param_1,param_2);
    }
    pass1_1010_887a(param_3,uStack10,param_5,param_1,param_2);
  }
  pass1_1010_866c(uVar1,(ushort)param_3,param_3._2_2_,uStack10,param_5);
  return;
}



void __stdcall16far pass1_1010_82f8(ulong param_1,ushort param_2)

{
  *(ushort *)((int)param_1 + 0xe82) = param_2;
  return;
}



// WARNING: Could not reconcile some variable overlaps

astruct_43 * __stdcall16far unk_io_op_1010_830a(ulong param_1,ushort param_2,ushort param_3)

{
  uint in_AX;
  undefined4 *puVar1;
  undefined4 *puVar2;
  uchar *in_DX;
  uint uVar3;
  astruct_45 *iVar3;
  astruct_44 *iVar2;
  int iVar4;
  HINSTANCE16 unaff_CS;
  ushort uVar5;
  ushort uVar6;
  undefined4 local_2e;
  undefined4 uStack10;
  astruct_43 *paStack6;
  
  paStack6 = (astruct_43 *)0x0;
  iVar3 = (astruct_45 *)(param_2 * 0x10);
  uVar5 = (ushort)param_1;
  uVar6 = (ushort)(param_1 >> 0x10);
  if (iVar3->field_0x10 == 0x1) {
    uStack10 = (char *)set_err_mode_1010_8b14(param_1,*(ULONG *)&iVar3->field_0x12,param_3);
    uStack10._2_2_ = (ushort)((ulong)uStack10 >> 0x10);
    if ((iVar3->field_0x12 == (int)uStack10) && (iVar3->field_0x14 == uStack10._2_2_)) {
      msg_box_op_1010_8bb4(uVar5,uVar6,(ulong)uStack10,unaff_CS,param_3);
      return (astruct_43 *)0x0;
    }
    puVar1 = &local_2e;
    struct_op_1008_48fe((astruct_81 *)CONCAT22(param_3,puVar1),0x1,uStack10,uStack10._2_2_);
    mem_op_1000_179c(0x1e,(uchar *)((ulong)uStack10 >> 0x10),0x1000);
    uVar3 = (uint)((ulong)uStack10 >> 0x10) | (uint)puVar1;
    if (uVar3 == 0x0) {
      puVar2 = (undefined4 *)0x0;
      uVar3 = 0x0;
    }
    else {
      puVar2 = &local_2e;
      struct_op_1008_3f92((astruct_76 *)((ulong)uStack10 & 0xffff0000 | ZEXT24(puVar1)),
                          (astruct_83 *)CONCAT22(param_3,puVar2));
    }
    paStack6 = (astruct_43 *)CONCAT22(uVar3,puVar2);
    close_file_1008_496c(&local_2e,param_3);
    local_2e = paStack6;
  }
  else {
    if (*(int *)(param_2 * 0x10 + 0x10) == 0x2) {
      pass1_1010_878c((astruct_87 **)param_1,*(int *)(param_2 * 0x10 + 0x16),unaff_CS);
      if (*(long *)(uVar5 + 0x67c) == 0x0) {
        return (astruct_43 *)0x0;
      }
      iVar2 = (astruct_44 *)(param_2 * 0x10);
      pass1_1008_6562(*(ulong **)(uVar5 + 0x67c),CONCAT22(iVar2->field_0x1c,iVar2->field_0x1e),iVar2->field_0x1a,in_AX,
                      in_DX);
      local_2e = (astruct_43 *)CONCAT22(in_DX,in_AX);
    }
    else {
      iVar4 = param_2 * 0x10;
      if (*(int *)(iVar4 + 0x10) == 0x3) {
        local_2e = (astruct_43 *)set_err_mode_1010_8b14(param_1,*(ULONG *)(iVar4 + 0x12),param_3);
        if ((*(int *)(iVar4 + 0x12) == (int)local_2e) && (*(int *)(iVar4 + 0x14) == (int)((ulong)local_2e >> 0x10))) {
          msg_box_op_1010_8bb4(uVar5,uVar6,(ulong)local_2e,unaff_CS,param_3);
          local_2e = local_2e;
        }
      }
      else {
        local_2e = paStack6;
        if (*(int *)(param_2 * 0x10 + 0x10) == 0x4) {
          local_2e = (astruct_43 *)set_err_mode_1010_8b14(param_1,*(ULONG *)(param_2 * 0x10 + 0x12),param_3);
        }
      }
    }
  }
  paStack6 = local_2e;
  return paStack6;
}



char * __stdcall16far load_string_1010_847e(int param_1,INT16 in_buf_len_2,HINSTANCE16 in_hinstsance_3)

{
  LoadString16(in_hinstsance_3,0x3ff,(LPSTR)(param_1 + 0x682),in_buf_len_2);
  return (char *)CONCAT22(in_buf_len_2,(LPSTR)(param_1 + 0x682));
}



void __stdcall16far load_string_1010_84ac(int param_1,INT16 param_2,HINSTANCE16 param_3)

{
  ushort uVar1;
  
  uVar1 = param_2;
  LoadString16(param_3,0x3ff,(LPSTR)(param_1 + 0x682),param_2);
  str_op_1008_60e8((char *)CONCAT22(param_2,(LPSTR)(param_1 + 0x682)),uVar1);
  return;
}



void __stdcall16far
load_string_1010_84e0
          (HINSTANCE16 in_hinstance_5,ushort param_2,ushort param_3,ushort in_resc_id_3,char *in_buffer_4,
          short in_buf_len_5)

{
  LoadString16(in_hinstance_5,in_resc_id_3,in_buffer_4,in_buf_len_5);
  return;
}



void __stdcall16far pass1_1010_84f8(ulong param_1,int param_2,ushort param_3)

{
  undefined4 uVar1;
  undefined2 uStack780;
  char local_308 [0x100];
  undefined local_208 [0x100];
  undefined local_108 [0x104];
  int iStack4;
  
  if (*(int *)(param_2 * 0x10 + 0x10) != 0x3) {
    return;
  }
  uVar1 = *(undefined4 *)((int)param_1 + 0xe88);
  iStack4 = *(int *)((int)uVar1 + 0x70);
  str_1000_4d58(*(char **)(param_2 * 0x10 + 0x12),(char *)0x0,0x0,CONCAT22(param_3,local_208),
                (WNDCLASS16 *)CONCAT22(param_3,local_308));
  unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_108),(char *)CONCAT22(param_3,local_208));
  if (local_308[0] == '\0') {
    if (iStack4 == 0x0) {
      uStack780 = 0x14c0;
    }
    else {
      uStack780 = 0x14ba;
    }
    _uStack780 = CONCAT22(0x1050,uStack780);
  }
  else {
    _uStack780 = CONCAT22(param_3,local_308);
  }
  pass1_1000_3cea(CONCAT22(param_3,local_108),_uStack780);
  set_err_mode_1010_8b14(param_1,CONCAT22(param_3,local_108),param_3);
  return;
}



void __stdcall16far pass1_1010_85be(ulong param_1,int param_2,int param_3,ushort param_4)

{
  ulong uVar1;
  undefined local_30a [0x100];
  undefined local_20a [0x100];
  undefined local_10a [0x108];
  
  if (param_2 == 0x2) {
    uVar1 = *(ulong *)(param_3 * 0x4 + 0x2e34);
    str_1000_4d58((char *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x3)),(char *)0x0,0x0,CONCAT22(param_4,local_20a),
                  (WNDCLASS16 *)CONCAT22(param_4,local_30a));
    unk_str_op_1000_3d3e((char *)CONCAT22(param_4,local_10a),s_male_1050_14c6);
    pass1_1000_3cea(CONCAT22(param_4,local_10a),CONCAT22(param_4,local_20a));
    pass1_1000_3cea(CONCAT22(param_4,local_10a),CONCAT22(param_4,local_30a));
    set_err_mode_1010_8b14(param_1,CONCAT22(param_4,local_10a),param_4);
    return;
  }
  set_err_mode_1010_8b14(param_1,*(ULONG *)(param_3 * 0x4 + 0x2e34),param_4);
  return;
}



void __stdcall16far pass1_1010_866c(ushort param_1,ushort param_2,ushort param_3,ulong param_4,uint param_5)

{
  ulong uVar1;
  char cVar2;
  int iVar3;
  bool bVar4;
  
  if ((int)param_5 < 0x28) {
    if (((int)param_5 < 0x25) && (param_5 != 0x23)) {
      if (0x23 < param_5) {
        return;
      }
      cVar2 = (char)param_5;
      if (((cVar2 != '\v') && (cVar2 != '\x0f')) && (cVar2 != '!')) {
        return;
      }
    }
  }
  else {
    if (param_5 != 0x37) {
      if ((int)param_5 < 0x38) {
        if ((int)param_5 < 0x33) {
          return;
        }
        bVar4 = SBORROW2(param_5 - 0x33,0x1);
        iVar3 = param_5 - 0x34;
      }
      else {
        if (param_5 == 0x49) goto LAB_1010_8691;
        if ((int)(param_5 - 0x49) < 0x2a) {
          return;
        }
        bVar4 = SBORROW2(param_5 - 0x73,0x5);
        iVar3 = param_5 - 0x78;
      }
      if (iVar3 != 0x0 && bVar4 == iVar3 < 0x0) {
        return;
      }
    }
  }
LAB_1010_8691:
  uVar1 = *(ulong *)(param_5 * 0x4 + (int)param_4);
  memcpy_op_1008_676e(uVar1,(int)uVar1,param_1);
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1010_86de(ushort param_1,ushort param_2,uchar param_3,ulong param_4)

{
  long *plVar1;
  int iVar2;
  bool bVar3;
  undefined2 uVar4;
  long lVar5;
  ulong uVar6;
  long lStack20;
  undefined4 uStack10;
  
  uVar6 = pass1_1008_4772((astruct_76 *)param_4);
  uVar4 = (undefined2)(uVar6 >> 0x10);
  uStack10 = 0x0;
  do {
    plVar1 = (long *)((int)uVar6 + 0x8);
    if (*plVar1 == uStack10 || *plVar1 < uStack10) {
      return;
    }
    lVar5 = uStack10;
    pass1_1008_4544(param_4);
    iVar2 = (int)lVar5;
    bVar3 = false;
    for (lStack20 = 0x0; plVar1 = (long *)((int)uVar6 + 0x4), *plVar1 != lStack20 && lStack20 <= *plVar1;
        lStack20 = lStack20 + 0x1) {
      if (bVar3) {
LAB_1010_86fc:
        if (bVar3) {
          if (*(char *)((int)lStack20 + iVar2) == -0x1) {
            *(uchar *)((int)lStack20 + iVar2) = param_3;
            break;
          }
        }
      }
      else {
        if (*(char *)((int)lStack20 + iVar2) == -0x1) goto LAB_1010_86fc;
        *(uchar *)((int)lStack20 + iVar2 + -0x1) = param_3;
        bVar3 = true;
      }
    }
    uStack10 = uStack10 + 0x1;
  } while( true );
}



void __stdcall16far pass1_1010_878c(astruct_87 **param_1,int param_2,HINSTANCE16 param_3)

{
  uint uVar1;
  uint uVar2;
  ushort uVar4;
  uchar *puVar3;
  uchar *puVar4;
  astruct_87 *uVar6;
  int iVar5;
  ushort uVar7;
  ushort unaff_SS;
  astruct_87 *paVar8;
  astruct_87 *paVar9;
  
  uVar7 = (ushort)((ulong)param_1 >> 0x10);
  uVar6 = (astruct_87 *)param_1;
  if (uVar6->field_0x680 == param_2) {
    return;
  }
  uVar1 = uVar6->field_0x67c;
  puVar4 = uVar6->field_0x67e;
  puVar3 = (uchar *)((uint)puVar4 | uVar1);
  uVar2 = uVar1;
  if (puVar3 != (uchar *)0x0) {
    pass1_1008_64a2((uint *)CONCAT22(puVar4,uVar1));
    param_3 = 0x1000;
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(puVar4,uVar1),0x1000);
  }
  if ((param_2 == 0x1) || (param_2 == 0x2)) {
    mem_op_1000_179c(0x8,puVar3,0x1000);
    puVar4 = (uchar *)((uint)puVar3 | uVar2);
    if (puVar4 == (uchar *)0x0) {
      *(undefined4 *)&uVar6->field_0x67c = 0x0;
      goto LAB_1010_8869;
    }
    paVar8 = *param_1;
    paVar9 = (astruct_87 *)CONCAT22(puVar3,uVar2);
LAB_1010_8853:
    uVar4 = (ushort)paVar9;
    file_1008_6414((ulong *)paVar9,(ulong)paVar8,unaff_SS,puVar4);
  }
  else {
    iVar5 = param_2 * 0x4;
    paVar8 = (astruct_87 *)set_err_mode_1010_8b14((ulong)param_1,*(ULONG *)(iVar5 + 0x172a),unaff_SS);
    paVar9 = paVar8;
    if ((*(int *)(iVar5 + 0x172a) == (int)paVar8) && (*(int *)(iVar5 + 0x172c) == (int)((ulong)paVar8 >> 0x10))) {
      msg_box_op_1010_8bb4((ushort)uVar6,uVar7,(ulong)paVar8,param_3,unaff_SS);
    }
    mem_op_1000_179c(0x8,(uchar *)((ulong)paVar9 >> 0x10),0x1000);
    puVar4 = (uchar *)((uint)((ulong)paVar9 >> 0x10) | (uint)paVar9);
    if (paVar9 != (astruct_87 *)0x0) goto LAB_1010_8853;
    uVar4 = 0x0;
    puVar4 = (uchar *)0x0;
  }
  uVar6->field_0x67c = uVar4;
  uVar6->field_0x67e = puVar4;
LAB_1010_8869:
  uVar6->field_0x680 = param_2;
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far
pass1_1010_887a(astruct_87 **param_1,undefined4 param_2,int param_3,undefined2 param_4,ushort param_5)

{
  uint uVar1;
  ulong uVar2;
  ulong uVar3;
  uchar *in_DX;
  uchar *puVar4;
  undefined2 extraout_DX;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  byte bVar8;
  undefined local_26 [0x6];
  ushort uStack32;
  ushort uStack30;
  ulong uStack28;
  undefined4 uStack24;
  ulong uStack20;
  ulong uStack16;
  astruct_76 *paStack12;
  astruct_76 *paStack8;
  uint uStack4;
  
  uStack4 = param_3 - 0xa;
  pass1_1010_878c(param_1,*(int *)(uStack4 * 0xa + 0x3382),param_4);
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x67c) != 0x0) {
    iVar5 = uStack4 * 0xa;
    uVar1 = uStack4;
    pass1_1008_6562(*(ulong **)((int)param_1 + 0x67c),
                    CONCAT22(*(undefined2 *)(iVar5 + 0x3388),*(undefined2 *)(iVar5 + 0x338a)),*(int *)(iVar5 + 0x3386),
                    uStack4,in_DX);
    paStack8 = (astruct_76 *)CONCAT22(in_DX,uVar1);
    uVar6 = (undefined2)((ulong)param_2 >> 0x10);
    paStack12 = *(astruct_76 **)((int)param_2 + 0x14);
    uStack16 = pass1_1008_4772(paStack12);
    uStack20 = pass1_1008_4772(paStack8);
    puVar4 = (uchar *)(uStack20 >> 0x10);
    uVar2 = *(ulong *)((int)uStack20 + 0x4);
    uVar7 = (undefined2)(uStack16 >> 0x10);
    iVar5 = (int)uStack16;
    if ((long)uVar2 < *(long *)(iVar5 + 0x4)) {
      uVar2 = (ulong)*(uint *)(iVar5 + 0x4);
    }
    uVar3 = *(ulong *)((int)uStack20 + 0x8);
    if ((long)uVar3 < *(long *)(iVar5 + 0x8)) {
      uVar3 = (ulong)*(uint *)(iVar5 + 0x8);
    }
    uVar1 = (uint)uVar3;
    uStack24 = uVar3 & 0xffff | uVar2 << 0x10;
    bVar8 = 0x1e;
    mem_op_1000_179c(0x1e,puVar4,0x1000);
    if (((uint)puVar4 | uVar1) == 0x0) {
      uVar1 = 0x0;
      uVar7 = 0x0;
    }
    else {
      struct_op_1008_6604((astruct_85 *)CONCAT22(puVar4,uVar1),(int)uStack24,(int)(uStack24 >> 0x10));
      uVar7 = extraout_DX;
    }
    uStack28 = CONCAT22(uVar7,uVar1);
    pass1_1008_431c(CONCAT22(uVar7,uVar1),bVar8);
    uVar7 = (undefined2)(uStack16 >> 0x10);
    uStack30 = (uStack24._2_2_ - *(int *)((int)uStack16 + 0x4)) / 0x2;
    uStack32 = (int)uStack24 - *(int *)((int)uStack16 + 0x8);
    pass1_1008_3e54((ushort *)CONCAT22(param_5,local_26),0x0,uStack32,uStack30);
    pass1_1008_4480(uStack28,(ushort *)CONCAT22(param_5,local_26),paStack12,param_5);
    pass1_1008_3e76((ushort *)CONCAT22(param_5,local_26),0x0,0x0,0x7);
    pass1_1008_4480(uStack28,(ushort *)CONCAT22(param_5,local_26),paStack8,param_5);
    *(ulong *)(param_3 * 0x4 + (int)param_2) = uStack28;
  }
  return;
}

