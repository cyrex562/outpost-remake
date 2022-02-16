
void __stdcall16far pass1_1008_6732(ushort *param_1)

{
  long lVar1;
  astruct_457 *iVar2;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_457 *)param_1;
  *param_1 = 0x685a;
  iVar2->field_0x2 = 0x1008;
  if (iVar2->field_0x10 != 0x0) {
    lVar1 = iVar2->field_0x10;
    call_fn_ptr_1000_0dc6((u16)lVar1,(u16)((ulong)lVar1 >> 0x10),0x1000);
  }
  iVar2->field_0x10 = 0x0;
  pass1_1008_41bc(param_1);
  return;
}



void __stdcall16far memcpy_op_1008_676e(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 uVar1;
  long lVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x6) == 0x0) {
    return;
  }
  mem_op_1000_179c(0x1e,param_3,0x1000);
  uVar3 = (uint)param_3 | param_2;
  if (uVar3 == 0x0) {
    param_2 = 0x0;
    uVar3 = 0x0;
  }
  else {
    uVar1 = *(undefined4 *)(iVar4 + 0x10);
    uVar7 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar5 = (int)uVar1;
    struct_op_1008_6604((astruct_85 *)CONCAT22(param_3,param_2),*(undefined2 *)(iVar5 + 0x8),
                        *(undefined2 *)(iVar5 + 0x4));
  }
  pass1_1000_48a8(*(ulong *)(param_2 + 0x10),*(ulong *)(iVar4 + 0x10),0x28);
  uVar1 = *(undefined4 *)(param_2 + 0x10);
  lVar2 = *(long *)((int)uVar1 + 0x8) * *(long *)(iVar4 + 0x18);
  hmemcpy16((LPVOID)&PTR_LOOP_1050_1000,(LPCVOID)lVar2,
            CONCAT22((int)*(undefined4 *)(iVar4 + 0x6),(int)((ulong)lVar2 >> 0x10)));
  *(undefined2 *)(param_2 + 0x1c) = 0x1;
  return;
}



ulong __stdcall16far pass1_1008_6834(ulong param_1,byte param_2)

{
  pass1_1008_6732((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far set_struct_1008_687a(astruct_20 *param_1,ULONG param_2)

{
  astruct_20 *iVar1;
  astruct_20 *uVar1;
  
  set_struct_op_1008_9584(param_1,param_2);
  uVar1 = (astruct_20 *)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  iVar1->field_0xcc = 0x0;
  iVar1->field_0xce = 0x0;
  set_struct_1008_574a((astruct_21 *)((ulong)param_1 & 0xffff0000 | ZEXT24(&iVar1->field_0xd2)));
  param_1->field_0x0 = 0x6bfc;
  iVar1->field_0x2 = 0x1008;
  (iVar1->field_0xd2).field_0xa = 0x0;
  return;
}



BOOL16 __stdcall16far pass1_1008_68c6(ushort param_1,ushort param_2,int param_3,ushort param_4)

{
  BOOL16 BVar1;
  ushort in_DX;
  ushort unaff_SS;
  
  BVar1 = show_win_1008_96ae(CONCAT22(param_2,param_1),param_3,param_4);
  pass1_1008_6a04(CONCAT22(param_2,param_1),in_DX,unaff_SS);
  return BVar1;
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



void __stdcall16far pass1_1008_6978(ulong param_1,int param_2,ulong param_3,uint param_4,uchar *param_5)

{
  code **ppcVar1;
  undefined2 *puStack10;
  undefined2 *puStack6;
  
  mem_op_1000_179c(0xa,param_5,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_5,param_4);
  if (((uint)param_5 | param_4) == 0x0) {
    puStack6 = (undefined2 *)0x0;
  }
  else {
    if (param_2 == 0x0) {
      param_2 = *(int *)((int)param_1 + 0xcc);
    }
    *puStack10 = 0x389a;
    *(undefined2 *)(param_4 + 0x2) = 0x1008;
    *(ulong *)(param_4 + 0x4) = param_3;
    *(int *)(param_4 + 0x8) = param_2;
    *puStack10 = 0x6c8c;
    *(undefined2 *)(param_4 + 0x2) = 0x1008;
    puStack6 = puStack10;
  }
  ppcVar1 = (code **)((int)*(undefined4 *)((int)param_1 + 0xd2) + 0x4);
  (**ppcVar1)(0x1000,(undefined4 *)((int)param_1 + 0xd2),param_1._2_2_,puStack6);
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



ushort * __stdcall16far pass1_1008_6b5a(ushort *param_1,byte param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_458 *uVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  uVar4 = (astruct_458 *)param_1;
  *param_1 = 0x6c8c;
  uVar4->field_0x2 = 0x1008;
  puVar1 = uVar4->field_0x4;
  uVar2 = uVar4->field_0x6;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *param_1 = 0x389a;
  uVar4->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1008_6bb4(ushort *param_1,byte param_2)

{
  astruct_459 *uVar1;
  undefined2 uVar2;
  
  uVar1 = (astruct_459 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong * __stdcall16far str_1008_6d8a(ulong *param_1,char *param_2,ushort param_3,ushort param_4,uchar param_5)

{
  ushort uVar1;
  ushort uVar2;
  
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x4) = 0xffff;
  PTR_LOOP_1050_0312 = (undefined *)&DAT_1050_0004;
  sys_1000_3f9c((uchar *)0x65a0,(uchar *)&USHORT_1050_1050,(ushort)_PTR_s_SC_03d_1050_0314_1050_031c,
                (ushort)((ulong)_PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),0x4,&stack0xfffe,uVar2,0x1000,param_4,param_5
               );
  uVar1 = str_op_1008_60e8(param_2,param_3);
  *(ushort *)param_1 = uVar1;
  *(ushort *)((int)param_1 + 0x2) = param_3;
  return param_1;
}



void __stdcall16far close_file_1008_6dd0(undefined4 *param_1,HFILE16 param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x4) != -0x1) {
    _lclose16(param_2);
    *(undefined2 *)((int)param_1 + 0x4) = 0xffff;
  }
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  return;
}



BOOL16 __stdcall16far file_fn_1008_6e02(uint32_t *param_1,LPCSTR in_string,uint16_t param_3)

{
  int var1;
  BOOL16 var2;
  uchar *extraout_DX;
  int unaff_DI;
  ushort uVar1;
  undefined local_4 [0x2];
  
  PTR_LOOP_1050_0310 = (undefined *)0x0;
  var1 = write_to_file_1008_70a6((undefined4 *)param_1,in_string);
  if (var1 != 0x0) {
    uVar1 = (ushort)((ulong)param_1 >> 0x10);
    pass1_1008_72a8();
    var1 = pass1_1008_7006((ushort)param_1,uVar1,CONCAT22(param_3,local_4),extraout_DX,unaff_DI,param_3);
    if ((var1 != 0x0) && (var1 = file_fn_1008_6eee(param_1,local_4,param_3), var1 != 0x0)) {
      var2 = file_fn_1008_726c((ushort)param_1,uVar1,(HFILE16)in_string);
      if (var2 == 0x0) {
        return 0x0;
      }
      return 0x1;
    }
    _lclose16((HFILE16)in_string);
  }
  return 0x0;
}



BOOL16 __stdcall16far read_file_1008_6e78(uint32_t param_1,uint16_t param_2,LPCSTR in_string,uint16_t param_4)

{
  BOOL16 b_var1;
  int i_var2;
  undefined *var3;
  uchar *extraout_DX;
  int unaff_DI;
  undefined local_4 [0x2];
  
  PTR_LOOP_1050_0310 = (undefined *)0x0;
  b_var1 = read_file_1008_7146(param_1,param_2,in_string,param_4);
  if (b_var1 != 0x0) {
    pass1_1008_72a8();
    i_var2 = pass1_1008_7056(param_1,param_2,CONCAT22(param_4,local_4),extraout_DX,unaff_DI,param_4);
    if (i_var2 != 0x0) {
      var3 = local_4;
      read_file_1008_6f7a(param_1,param_2,CONCAT22(param_4,var3),param_4);
      if (var3 != (undefined *)0x0) {
        b_var1 = file_fn_1008_726c(param_1,param_2,(HFILE16)in_string);
        if (b_var1 == 0x0) {
          return 0x0;
        }
        return 0x1;
      }
    }
    _lclose16((HFILE16)in_string);
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far file_fn_1008_6eee(undefined2 param_1,undefined2 param_2,ulong param_3)

{
  BOOL16 BVar1;
  ushort uVar2;
  uchar *in_DX;
  ushort unaff_SS;
  undefined4 uVar3;
  undefined local_e [0x4];
  undefined4 uStack10;
  undefined4 *puStack6;
  
  puStack6 = (undefined4 *)*_PTR_LOOP_1050_5748;
  uStack10 = *puStack6;
  pass1_1020_a43e(unaff_SS,in_DX,(ushort *)CONCAT22(unaff_SS,local_e));
  BVar1 = pass1_1028_d7a0((ushort)uStack10,(ushort)((ulong)uStack10 >> 0x10),param_3,unaff_SS);
  if (BVar1 != 0x0) {
    BVar1 = pass1_1030_5c1a(_PTR_LOOP_1050_5736,param_3,unaff_SS);
    if (BVar1 != 0x0) {
      uVar3 = write_to_file_1028_dce2(_PTR_LOOP_1050_65e2,param_3,unaff_SS);
      if ((int)((ulong)uVar3 >> 0x10) != 0x0) {
        uVar2 = pass1_1038_7b20(_PTR_LOOP_1050_5a64,param_3,unaff_SS);
        if (uVar2 != 0x0) {
          BVar1 = pass1_1020_a644((ushort)local_e,unaff_SS,param_3,unaff_SS);
          if (BVar1 != 0x0) {
            return;
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far read_file_1008_6f7a(undefined2 param_1,undefined2 param_2,undefined4 param_3,uint16_t param_4)

{
  uint16_t var5;
  int i_var3;
  BOOL16 b_var4;
  uchar *in_DX;
  uint16_t uVar1;
  ushort *puVar2;
  undefined local_e [0x4];
  undefined4 uStack10;
  undefined4 *puStack6;
  
  puStack6 = (undefined4 *)*_PTR_LOOP_1050_5748;
  uStack10 = *puStack6;
  puVar2 = pass1_1020_a43e(param_4,in_DX,(ushort *)CONCAT22(param_4,local_e));
  uVar1 = (uint16_t)((ulong)puVar2 >> 0x10);
  var5 = read_file_1028_d7ba((int)uStack10,(int)((ulong)uStack10 >> 0x10),param_3,param_4,(uint16_t)puVar2);
  if (var5 != 0x0) {
    var5 = read_file_1030_5c52(_PTR_LOOP_1050_5736,param_3,var5,param_4);
    if (var5 != 0x0) {
      read_file_1028_def2(_PTR_LOOP_1050_65e2,param_3,param_4,var5);
      if (var5 != 0x0) {
        i_var3 = read_file_1038_7c02(_PTR_LOOP_1050_5a64,param_3,var5,uVar1);
        if (i_var3 != 0x0) {
          b_var4 = read_file_1020_a65e(CONCAT22(param_4,local_e),param_3,param_4,(uint16_t)local_e);
          if (b_var4 != 0x0) {
            return;
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far
pass1_1008_7006(ushort param_1,ushort param_2,ulong param_3,uchar *param_4,int param_5,ushort param_6)

{
  code **ppcVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if ((int)PTR_LOOP_1050_0334 <= iStack4) {
      return 0x1;
    }
    puVar2 = (undefined4 *)
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,*(ushort *)(iStack4 * 0x2 + 0x320),param_6,param_4,param_5);
    ppcVar1 = (code **)((int)*puVar2 + 0x8);
    uVar3 = (**ppcVar1)(0x1010,puVar2,param_3);
    param_4 = (uchar *)((ulong)uVar3 >> 0x10);
    if ((int)uVar3 == 0x0) break;
    iStack4 = iStack4 + 0x1;
  }
  return (int)uVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far
pass1_1008_7056(ushort param_1,ushort param_2,ulong param_3,uchar *param_4,int param_5,ushort param_6)

{
  code **ppcVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    if ((int)PTR_LOOP_1050_0334 <= iStack4) {
      return 0x1;
    }
    puVar2 = (undefined4 *)
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,*(ushort *)(iStack4 * 0x2 + 0x320),param_6,param_4,param_5);
    ppcVar1 = (code **)((int)*puVar2 + 0xc);
    uVar3 = (**ppcVar1)(0x1010,puVar2,param_3);
    param_4 = (uchar *)((ulong)uVar3 >> 0x10);
    if ((int)uVar3 == 0x0) break;
    iStack4 = iStack4 + 0x1;
  }
  return (int)uVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined2 __stdcall16far write_to_file_1008_70a6(undefined4 *param_1,LPCSTR param_2)

{
  HFILE16 HVar1;
  int iVar2;
  undefined2 uVar3;
  LPCSTR pCVar4;
  undefined2 unaff_SS;
  undefined in_AF;
  long lVar5;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  pCVar4 = param_2;
  if (*(int *)(iVar2 + 0x4) != -0x1) {
    pCVar4 = (LPCSTR)s_tile2_bmp_1050_1538;
    _lclose16((HFILE16)param_2);
    *(undefined2 *)(iVar2 + 0x4) = 0xffff;
  }
  HVar1 = _lcreat16(pCVar4,0x0);
  *(HFILE16 *)(iVar2 + 0x4) = HVar1;
  if (HVar1 == 0xffff) {
    PTR_LOOP_1050_0310 = (undefined *)0x6cf;
  }
  else {
    PTR_LOOP_1050_0312 = (undefined *)&DAT_1050_0004;
    sys_1000_3f9c((uchar *)0x65a0,(uchar *)&USHORT_1050_1050,(ushort)_PTR_s_SC_03d_1050_0314_1050_031c,
                  (ushort)((ulong)_PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),0x4,&stack0xfffe,uVar3,0x1000,unaff_SS,
                  in_AF);
    pCVar4 = (LPCSTR)str_op_1000_3da4((char *)0x105065a0);
    lVar5 = _hwrite16(0x1000,pCVar4,CONCAT22(0x65a0,(int)pCVar4 >> 0xf));
    if (lVar5 == (int)pCVar4) {
      return 0x1;
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return 0x0;
}



BOOL16 __stdcall16far read_file_1008_7146(int32_t param_1,uint16_t param_2,LPCSTR param_3,uint16_t param_4)

{
  HFILE16 HVar1;
  int iVar2;
  LPCSTR path;
  
  path = param_3;
  if (*(int *)(param_1 + 0x4) != -0x1) {
    path = (LPCSTR)s_tile2_bmp_1050_1538;
    _lclose16((HFILE16)param_3);
    *(undefined2 *)(param_1 + 0x4) = 0xffff;
  }
  HVar1 = _lopen16(path,0x0);
  *(HFILE16 *)(param_1 + 0x4) = HVar1;
  if (HVar1 == 0xffff) {
    PTR_LOOP_1050_0310 = (undefined *)0x6cf;
  }
  else {
    iVar2 = read_file_1008_71a0(CONCAT22(param_2,param_1),param_4);
    if (iVar2 != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x100871e6)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined2 __stdcall16far read_file_1008_71a0(undefined4 param_1,uint16_t param_2)

{
  uint buffer;
  uint uVar1;
  undefined in_AF;
  long lVar2;
  int iStack26;
  int iStack24;
  int iStack22;
  char local_e [0x9];
  undefined uStack5;
  undefined2 uStack4;
  
  uStack4 = 0x1;
  buffer = str_op_1000_3da4((char *)0x105065a0);
  iStack22 = 0x0;
  lVar2 = WIN16_hread(0x1000,buffer,CONCAT22(local_e,(int)buffer >> 0xf));
  uVar1 = (uint)lVar2;
  if ((int)buffer < lVar2) {
    uVar1 = buffer;
  }
  iStack24 = uVar1 - 0x2;
  if (iStack24 < 0x0) {
    iStack24 = 0x0;
  }
  iStack26 = 0x2;
  while (iStack24 != 0x0) {
    iStack22 = iStack22 * 0xa + local_e[iStack26] + -0x30;
    iStack26 = iStack26 + 0x1;
    iStack24 = iStack24 + -0x1;
  }
  if (iStack22 == 0x1) {
    PTR_LOOP_1050_0312 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
  }
  else {
    if (iStack22 == 0x4) {
      PTR_LOOP_1050_0312 = (undefined *)&DAT_1050_0004;
    }
    else {
      uStack5 = 0x0;
      PTR_LOOP_1050_0312 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
      uStack4 = 0x0;
    }
  }
  sys_1000_3f9c((uchar *)0x65a0,(uchar *)&USHORT_1050_1050,(ushort)_PTR_s_SC_03d_1050_0314_1050_031c,
                (ushort)((ulong)_PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),(ushort)PTR_LOOP_1050_0312,&stack0xfffe,
                (int)((ulong)param_1 >> 0x10),0x1000,param_2,in_AF);
  return uStack4;
}



BOOL16 __stdcall16far file_fn_1008_726c(uint32_t param_1,uint16_t param_2,HFILE16 file_handle)

{
  HFILE16 HVar1;
  
  if (*(int *)(param_1 + 0x4) != -0x1) {
    HVar1 = _lclose16(file_handle);
    if (HVar1 == 0xffff) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d1;
      return 0x0;
    }
    *(undefined2 *)(param_1 + 0x4) = 0xffff;
    PTR_LOOP_1050_0310 = (undefined *)0x0;
  }
  return 0x1;
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
