
void __stdcall16far
pass1_1008_3bd6(astruct_160 *param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5,ulong param_6,
               ulong param_7,ushort param_8,ushort param_9)

{
  mixed_struct_op_1040_8fb8
            ((ushort *)CONCAT22(param_2,param_1),param_3,(char *)0x0,param_5,(ushort)param_6,(ushort)(param_6 >> 0x10),
             (ushort)param_7,(ushort)(param_7 >> 0x10),param_8,(int)&PTR_LOOP_1050_1040,param_9);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x3cfc;
  param_1->field_0x2 = 0x1008;
  param_1->field_0x36 = 0x0;
  param_1->field_0x26 = 0x0;
  pass1_1040_9252(CONCAT22(param_2,param_1),(ushort)&PTR_LOOP_1050_1040);
  create_window_1040_92dc(CONCAT22(param_2,param_1),(UINT16)&PTR_LOOP_1050_1040);
  mov_update_win_1040_93aa
            ((astruct_65 *)CONCAT22(param_2,param_1),(INT16)param_4,(ushort)(param_4 >> 0x10),(int)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
win_ui_op_1008_3c34(undefined4 param_1,byte param_2,undefined2 param_3,HDC16 param_4,undefined2 param_5)

{
  ushort uVar1;
  code **ppcVar2;
  HPALETTE16 b_force_background;
  int iVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  undefined4 *puStack6;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  if ((*(uint *)(iVar3 + 0xa) | *(uint *)(iVar3 + 0x8)) != 0x0) {
    puStack6 = (undefined4 *)*(undefined4 *)(iVar3 + 0x8);
    if ((*(long *)(iVar3 + 0xc) != 0x0) && ((param_2 & 0x1) != 0x0)) {
      puStack6 = (undefined4 *)*(undefined4 *)(iVar3 + 0xc);
    }
    if ((*(long *)(iVar3 + 0x10) != 0x0) && ((param_2 & 0x4) != 0x0)) {
      puStack6 = (undefined4 *)*(undefined4 *)(iVar3 + 0x10);
    }
    uVar5 = (undefined2)((ulong)_PTR_LOOP_1050_4230 >> 0x10);
    uVar1 = *(ushort *)((int)_PTR_LOOP_1050_4230 + 0x10);
    b_force_background =
         palette_op_1008_4e08
                   ((astruct_13 *)CONCAT22(uVar1,*(undefined2 *)((int)_PTR_LOOP_1050_4230 + 0xe)),&param_3,uVar1,param_4
                   );
    ppcVar2 = (code **)((int)*puStack6 + 0x4);
    (**ppcVar2)(param_4,(int)puStack6,(int)((ulong)puStack6 >> 0x10),*(undefined2 *)(iVar3 + 0x28),
                *(undefined2 *)(iVar3 + 0x26),&param_3,param_5);
    SelectPalette16(param_4,0x0,b_force_background);
    DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  }
  return;
}



ushort * __stdcall16far pass1_1008_3cd6(ushort *param_1,byte param_2)

{
  mix_win_ui_op_1040_911e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far post_msg_1008_3d20(ulong param_1,HWND16 param_2)

{
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,*(undefined2 *)((int)param_1 + 0xcc)));
  return;
}



ushort * __stdcall16far pass_1008_3d44(ushort *param_1,byte param_2)

{
  astruct_453 *uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  uVar1 = (astruct_453 *)param_1;
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1008_3e0e(ulong param_1)

{
  code **ppcVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x4) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x4);
    (**ppcVar1)();
  }
  return;
}



ushort * __stdcall16far pass1_1008_3e38(ushort *param_1)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x2) = 0x0;
  *(undefined2 *)((int)param_1 + 0x4) = 0x0;
  return param_1;
}



ushort * __stdcall16far pass1_1008_3e54(ushort *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = param_4;
  *(ushort *)((int)param_1 + 0x2) = param_3;
  *(ushort *)((int)param_1 + 0x4) = param_2;
  return param_1;
}



void __stdcall16far pass1_1008_3e76(ushort *param_1,ushort param_2,ushort param_3,ushort param_4)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = param_4;
  *(ushort *)((int)param_1 + 0x2) = param_3;
  *(ushort *)((int)param_1 + 0x4) = param_2;
  return;
}



void __stdcall16far pass1_1008_3e94(ushort *param_1,ushort *param_2,ushort *param_3)

{
  *param_3 = *param_1;
  *param_2 = *(ushort *)((int)param_1 + 0x2);
  return;
}



void __stdcall16far pass1_1008_3eb4(ushort *param_1,ushort *param_2,ushort *param_3,ushort *param_4)

{
  ushort uVar1;
  
  *param_4 = *param_1;
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_3 = *(ushort *)((int)param_1 + 0x2);
  *param_2 = *(ushort *)((int)param_1 + 0x4);
  return;
}



void __stdcall16far pass1_1008_3ee2(int *param_1,int *param_2)

{
  int iVar1;
  int iVar2;
  ushort uVar3;
  ushort uVar4;
  
  iVar1 = *param_2 - *param_1;
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  *param_1 = iVar1 + 0x1;
  uVar3 = (ushort)((ulong)param_2 >> 0x10);
  uVar4 = (ushort)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  iVar1 = *(int *)((int)param_2 + 0x2) - *(int *)(iVar2 + 0x2);
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  *(int *)(iVar2 + 0x2) = iVar1 + 0x1;
  iVar1 = *(int *)((int)param_2 + 0x4) - *(int *)(iVar2 + 0x4);
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  *(int *)(iVar2 + 0x4) = iVar1 + 0x1;
  return;
}



void __stdcall16far pass1_1008_3f32(int *param_1,int *param_2)

{
  int *piVar1;
  ushort uVar2;
  ushort uVar3;
  
  *param_1 = *param_1 + *param_2;
  uVar2 = (ushort)((ulong)param_2 >> 0x10);
  uVar3 = (ushort)((ulong)param_1 >> 0x10);
  piVar1 = (int *)((int)param_1 + 0x2);
  *piVar1 = *piVar1 + *(int *)((int)param_2 + 0x2);
  piVar1 = (int *)((int)param_1 + 0x4);
  *piVar1 = *piVar1 + *(int *)((int)param_2 + 0x4);
  return;
}



void __stdcall16far pass1_1008_3f62(ushort *param_1,ushort *param_2)

{
  undefined2 uVar1;
  undefined2 uVar2;
  
  *param_1 = *param_2;
  uVar1 = (undefined2)((ulong)param_2 >> 0x10);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x2) = *(undefined2 *)((int)param_2 + 0x2);
  *(undefined2 *)((int)param_1 + 0x4) = *(undefined2 *)((int)param_2 + 0x4);
  return;
}



void __stdcall16far struct_op_1008_3f92(astruct_76 *param_1,astruct_83 *param_2)

{
  code **ppcVar1;
  astruct_76 *iVar2;
  undefined2 uVar2;
  
  struct_op_1008_56b4(param_1);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  *(undefined4 *)&iVar2->field_0x6 = 0x0;
  *(undefined4 *)((int)&iVar2->field_0x8 + 0x2) = 0x0;
  *(undefined2 *)&iVar2->field_0xe = 0x0;
  *(undefined4 *)((int)&iVar2->field_0xe + 0x2) = 0x0;
  iVar2->field_0x14 = 0x0;
  *(undefined4 *)&iVar2->field_0x18 = 0x0;
  iVar2->field_0x1c = 0x0;
  param_1->field_0x0 = (ushort)&PTR_LOOP_1050_48de;
  iVar2->field_0x2 = 0x1008;
  if (param_2 == (astruct_83 *)0x0) {
    return;
  }
  ppcVar1 = (code **)((int)*(undefined4 *)param_2 + 0x8);
  (**ppcVar1)();
  struct_op_1008_4214(param_1,param_2);
  pass1_1008_47cc(param_1);
  pass1_1008_4834(param_1);
  return;
}



void __stdcall16far pass1_1008_4016(astruct_76 *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  struct_op_1008_56b4(param_1);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0x6) = 0x0;
  *(undefined4 *)(iVar1 + 0xa) = 0x0;
  *(undefined2 *)(iVar1 + 0xe) = 0x0;
  *(undefined4 *)(iVar1 + 0x10) = 0x0;
  *(undefined4 *)(iVar1 + 0x14) = 0x0;
  *(undefined4 *)(iVar1 + 0x18) = 0x0;
  *(undefined2 *)(iVar1 + 0x1c) = 0x0;
  param_1->field_0x0 = (ushort)&PTR_LOOP_1050_48de;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_405c(astruct_76 *param_1,ulong param_2,int param_3,int param_4)

{
  undefined4 uVar1;
  sqword sVar2;
  int iVar3;
  long lVar4;
  uchar *puVar5;
  astruct_76 *iVar4;
  uint uVar6;
  ulong uStack10;
  
  struct_op_1008_56b4(param_1);
  uVar6 = (uint)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_76 *)param_1;
  *(undefined4 *)&iVar4->field_0x6 = 0x0;
  *(undefined4 *)((int)&iVar4->field_0x8 + 0x2) = 0x0;
  *(undefined2 *)&iVar4->field_0xe = 0x0;
  *(undefined4 *)((int)&iVar4->field_0xe + 0x2) = 0x0;
  iVar4->field_0x14 = 0x0;
  *(undefined4 *)&iVar4->field_0x18 = 0x0;
  iVar4->field_0x1c = 0x0;
  param_1->field_0x0 = (ushort)&PTR_LOOP_1050_48de;
  iVar4->field_0x2 = 0x1008;
  iVar3 = param_4 * 0x8 + 0x1f;
  iVar3 = ((int)(iVar3 + (iVar3 >> 0xf & 0x1fU)) >> 0x5) << 0x2;
  uStack10 = SEXT24(param_3);
  lVar4 = (long)iVar3 * (long)param_3 + 0x436;
  lVar4 = mem_op_1000_0a48(0x1,(uint)lVar4,(int)((ulong)lVar4 >> 0x10),_PTR_LOOP_1050_5f2c,0x1000);
  iVar4->field_0x6 = (int)lVar4;
  *(int *)&iVar4->field_0x8 = (int)((ulong)lVar4 >> 0x10);
  pass1_1008_47cc((astruct_76 *)((ulong)param_1 & 0xffff | (ulong)uVar6 << 0x10));
  *(int *)&iVar4->field_0x18 = iVar3;
  iVar4->field_0x1a = iVar3 >> 0xf;
  *(undefined4 *)*(undefined4 *)((int)&iVar4->field_0xe + 0x2) = 0x28;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(long *)((int)uVar1 + 0x4) = (long)param_4;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(ulong *)((int)uVar1 + 0x8) = uStack10;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined2 *)((int)uVar1 + 0xc) = 0x1;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined2 *)((int)uVar1 + 0xe) = 0x8;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined4 *)((int)uVar1 + 0x10) = 0x0;
  sVar2 = (qword)*(ulong *)&iVar4->field_0x18 * (qword)uStack10;
  puVar5 = (uchar *)((qword)sVar2 >> 0x20);
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined4 *)((int)uVar1 + 0x14) = (long)sVar2;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined4 *)((int)uVar1 + 0x20) = 0x100;
  uVar1 = *(undefined4 *)((int)&iVar4->field_0xe + 0x2);
  *(undefined4 *)((int)uVar1 + 0x24) = 0x100;
  pass1_1008_4834(param_1);
  pass1_1008_4d84(*(astruct_90 **)((int)&iVar4->field_0x8 + 0x2),param_2,puVar5);
  return;
}



void __stdcall16far pass1_1008_41bc(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  long lVar3;
  code **ppcVar4;
  astruct_288 *iVar5;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_288 *)param_1;
  *param_1 = (ushort)&PTR_LOOP_1050_48de;
  iVar5->field_0x2 = 0x1008;
  puVar1 = iVar5->field_0xa;
  uVar2 = iVar5->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  if (iVar5->field_0x6 != 0x0) {
    lVar3 = iVar5->field_0x6;
    call_fn_ptr_1000_0dc6((u16)lVar3,(u16)((ulong)lVar3 >> 0x10),0x1000);
  }
  *param_1 = 0x389a;
  iVar5->field_0x2 = 0x1008;
  return;
}



void __stdcall16far struct_op_1008_4214(astruct_76 *param_1,astruct_83 *param_2)

{
  ulong *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_83 *iVar4;
  astruct_83 *uVar4;
  
  uVar4 = (astruct_83 *)((ulong)param_2 >> 0x10);
  iVar4 = (astruct_83 *)param_2;
  *(undefined4 *)((int)param_1 + 0x6) = iVar4->field_0x1a;
  iVar4->field_0x1a = 0x0;
  puVar1 = iVar4->field_0x4;
  uVar2 = iVar4->field_0x6;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *(undefined4 *)&iVar4->field_0x4 = 0x0;
  iVar4->field_0xe = 0x0;
  iVar4->field_0x12 = 0x0;
  iVar4->field_0x16 = 0x0;
  iVar4->field_0x1e = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far memcpy_op_1008_4274(ulong param_1,undefined2 param_2)

{
  int iVar1;
  uchar *puVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  undefined2 uVar6;
  ulong uVar7;
  long lVar8;
  
  uVar6 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x6) != 0x0) {
    uVar7 = pass1_1000_1284(*(ulong *)(iVar4 + 0x6),0x1000);
    iVar1 = (int)(uVar7 >> 0x10);
    lVar8 = mem_op_1000_0a48(0x1,(uint)(LPCVOID)uVar7,iVar1,_PTR_LOOP_1050_5f2c,0x1000);
    uVar5 = (uint)lVar8;
    puVar2 = (uchar *)((uint)((ulong)lVar8 >> 0x10) | uVar5);
    if (puVar2 != (uchar *)0x0) {
      hmemcpy16((LPVOID)&PTR_LOOP_1050_1000,(LPCVOID)uVar7,CONCAT22((int)*(undefined4 *)(iVar4 + 0x6),iVar1));
      mem_op_1000_179c(0x1e,puVar2,0x1000);
      uVar3 = (uint)puVar2 | uVar5;
      if (uVar3 == 0x0) {
        uVar5 = 0x0;
        uVar3 = 0x0;
      }
      else {
        pass1_1008_4016((astruct_76 *)CONCAT22(puVar2,uVar5));
      }
      *(long *)(uVar5 + 0x6) = lVar8;
      pass1_1008_47cc((astruct_76 *)CONCAT22(uVar3,uVar5));
      pass1_1008_4834((astruct_76 *)CONCAT22(uVar3,uVar5));
      *(undefined2 *)(uVar5 + 0x1c) = 0x1;
      return;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1008_431c(ulong param_1,byte param_2)

{
  ulong *puVar1;
  undefined4 uVar2;
  bool bVar3;
  ulong uVar4;
  int iVar5;
  uint uVar6;
  undefined4 uStack6;
  
  uVar6 = (uint)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(long *)(iVar5 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | (ulong)uVar6 << 0x10));
  }
  if ((*(uint *)(iVar5 + 0x8) | *(uint *)(iVar5 + 0x6)) == 0x0) {
    bVar3 = false;
  }
  else {
    if ((*(uint *)(iVar5 + 0xc) | *(uint *)(iVar5 + 0xa)) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | (ulong)uVar6 << 0x10));
    }
    bVar3 = true;
  }
  if (bVar3) {
    if ((*(uint *)(iVar5 + 0x16) | *(uint *)(iVar5 + 0x14)) == 0x0) {
      return;
    }
    uStack6 = 0x0;
    while( true ) {
      uVar2 = *(undefined4 *)(iVar5 + 0x10);
      puVar1 = (ulong *)((int)uVar2 + 0x8);
      if (*puVar1 == uStack6 || (long)*puVar1 < (long)uStack6) break;
      uVar4 = uStack6;
      pass1_1008_4544(param_1);
      uVar2 = *(undefined4 *)(iVar5 + 0x10);
      pass1_1000_4906((astruct_20 *)(uVar4 & 0xffff | (ulong)uStack6._2_2_ << 0x10),(WNDCLASS16 *)(uint)param_2,
                      *(uint *)((int)uVar2 + 0x4));
      uStack6 = uStack6 + 0x1;
    }
  }
  return;
}



undefined4 __stdcall16far pass1_1008_43cc(ulong param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
  }
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(iVar2 + 0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(*(undefined2 *)(iVar2 + 0x16),*(undefined2 *)(iVar2 + 0x14));
}



ulong __stdcall16far pass1_1008_4426(ulong param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
  }
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(iVar2 + 0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(*(undefined2 *)(iVar2 + 0xc),*(undefined2 *)(iVar2 + 0xa));
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1008_4480(ulong param_1,ushort *param_2,astruct_76 *param_3,ushort param_4)

{
  int iVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  uint uVar5;
  ulong uVar6;
  int iStack26;
  char *pcStack24;
  char *pcStack20;
  int iStack16;
  int local_6;
  undefined local_4 [0x2];
  
  pass1_1008_3e94(param_2,(ushort *)CONCAT22(param_4,&local_6),(ushort *)CONCAT22(param_4,local_4));
  uVar6 = pass1_1008_4772(param_3);
  uVar4 = (undefined2)(uVar6 >> 0x10);
  iVar1 = *(int *)((int)uVar6 + 0x4);
  iVar2 = *(int *)((int)uVar6 + 0x8);
  for (iStack16 = 0x0; iStack16 < iVar2; iStack16 = iStack16 + 0x1) {
    uVar5 = local_6 >> 0xf;
    iVar3 = local_6;
    local_6 = local_6 + 0x1;
    pass1_1008_4544(param_1);
    pcStack20 = (char *)CONCAT22(uVar5,iVar3);
    uVar6 = SEXT24(iStack16);
    pass1_1008_4544((ulong)param_3);
    pcStack24 = (char *)(uVar6 & 0xffff | (ulong)uVar5 << 0x10);
    iStack26 = iVar1;
    while (iStack26 != 0x0) {
      if (*pcStack24 != -0x1) {
        *pcStack20 = *pcStack24;
      }
      pcStack24 = (char *)CONCAT22((int)((ulong)pcStack24 >> 0x10) + (-(uint)(0xfffe < (uint)pcStack24) & 0x6c),
                                   (uint)pcStack24 + 0x1);
      pcStack20 = (char *)CONCAT22((int)((ulong)pcStack20 >> 0x10) + (-(uint)(0xfffe < (uint)pcStack20) & 0x6c),
                                   (uint)pcStack20 + 0x1);
      iStack26 = iStack26 + -0x1;
    }
  }
  return;
}



void __stdcall16far pass1_1008_4544(ulong param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
  }
  if (*(long *)(iVar2 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(iVar2 + 0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | (ulong)uVar3 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return;
  }
  return;
}



void __stdcall16far set_di_bits_to_device_1008_45d6(ulong param_1,HDC16 param_2)

{
  INT16 info;
  undefined4 uVar1;
  bool bVar2;
  int iVar3;
  int y_dest;
  undefined2 uVar4;
  INT16 cx;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)param_1);
  }
  if ((*(uint *)(iVar3 + 0x8) | *(uint *)(iVar3 + 0x6)) == 0x0) {
    bVar2 = false;
  }
  else {
    if ((*(uint *)(iVar3 + 0xc) | *(uint *)(iVar3 + 0xa)) == 0x0) {
      pass1_1008_4834((astruct_76 *)param_1);
    }
    bVar2 = true;
  }
  if (!bVar2) {
    return;
  }
  uVar1 = *(undefined4 *)(iVar3 + 0x10);
  cx = (INT16)((ulong)uVar1 >> 0x10);
  y_dest = (int)uVar1;
  info = *(INT16 *)(y_dest + 0x8);
  uVar1 = *(undefined4 *)(iVar3 + 0x14);
  SetDIBitsToDevice(param_2,0x0,y_dest,cx,(INT16)uVar1,(INT16)((ulong)uVar1 >> 0x10),info,0x0,0x0,(LPCVOID)0x0,
                    (BITMAPINFO *)info,*(UINT16 *)(y_dest + 0x4));
  return;
}



void __stdcall16far stretch_di_bits_1008_465a(ulong param_1,HDC16 param_2)

{
  PVOID bits;
  INT16 height_src;
  undefined4 uVar1;
  bool bVar2;
  int iVar3;
  int height_dst;
  undefined2 uVar4;
  INT16 x_src;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)param_1);
  }
  if ((*(uint *)(iVar3 + 0x8) | *(uint *)(iVar3 + 0x6)) == 0x0) {
    bVar2 = false;
  }
  else {
    if ((*(uint *)(iVar3 + 0xc) | *(uint *)(iVar3 + 0xa)) == 0x0) {
      pass1_1008_4834((astruct_76 *)param_1);
    }
    bVar2 = true;
  }
  if (!bVar2) {
    return;
  }
  uVar1 = *(undefined4 *)(iVar3 + 0x10);
  x_src = (INT16)((ulong)uVar1 >> 0x10);
  height_dst = (int)uVar1;
  bits = *(PVOID *)(height_dst + 0x4);
  height_src = *(INT16 *)(height_dst + 0x8);
  uVar1 = *(undefined4 *)(iVar3 + 0x14);
  StretchDIBits16(param_2,0x20,0xcc,0x0,height_dst,x_src,(INT16)uVar1,(INT16)((ulong)uVar1 >> 0x10),height_src,bits,
                  (BITMAPINFO *)0x0,0x0,CONCAT22(bits,height_src));
  return;
}



ushort __stdcall16far palette_op_1008_46e4(ulong param_1,ushort param_2,ushort param_3,HDC16 param_4)

{
  bool bVar1;
  ushort uVar2;
  HPALETTE16 HVar2;
  int iVar3;
  ushort uVar4;
  ulong uVar5;
  undefined4 uVar6;
  
  uVar4 = (ushort)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(long *)(iVar3 + 0x6) == 0x0) {
    uVar5._0_2_ = param_2;
    uVar5._2_2_ = param_3;
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | (ulong)uVar4 << 0x10));
    param_2 = (ushort)uVar5;
    param_3 = uVar5._2_2_;
  }
  uVar6 = CONCAT22(param_3,param_2);
  if (*(long *)(iVar3 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)(iVar3 + 0xa) == 0x0) {
      uVar6 = pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | (ulong)uVar4 << 0x10));
    }
    bVar1 = true;
  }
  uVar2 = (ushort)uVar6;
  if (!bVar1) {
    return 0x0;
  }
  create_palette_1008_4e38(*(astruct_13 **)(iVar3 + 0xa),param_4,(int)((ulong)uVar6 >> 0x10));
  *(ushort *)(iVar3 + 0xe) = uVar2;
  HVar2 = SelectPalette16(param_4,0x0,*(BOOL16 *)(iVar3 + 0xe));
  *(HPALETTE16 *)(iVar3 + 0x4) = HVar2;
  RealizePalette16((HDC16)s_tile2_bmp_1050_1538);
  return *(ushort *)(iVar3 + 0x4);
}



ulong __stdcall16far pass1_1008_4772(astruct_76 *param_1)

{
  bool bVar1;
  astruct_76 *iVar2;
  uint uVar2;
  
  uVar2 = (uint)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  if (*(long *)&iVar2->field_0x6 == 0x0) {
    pass1_1008_47cc((astruct_76 *)((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10));
  }
  if (*(long *)&iVar2->field_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(long *)((int)&iVar2->field_0x8 + 0x2) == 0x0) {
      pass1_1008_4834((astruct_76 *)((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(iVar2->field_0x12,*(undefined2 *)((int)&iVar2->field_0xe + 0x2));
}



void __stdcall16far pass1_1008_47cc(astruct_76 *param_1)

{
  ulong uVar1;
  undefined4 uVar2;
  uint uVar3;
  int iVar5;
  int iVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  ulong uStack14;
  int iVar4;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  if (*(long *)(iVar5 + 0x6) != 0x0) {
    uVar1 = *(ulong *)(iVar5 + 0x6);
    iVar6 = *(int *)(iVar5 + 0x8);
    iVar4 = (int)uVar1;
    uVar3 = iVar4 + 0xe;
    *(ulong *)(iVar5 + 0x10) = uVar1 & 0xffff0000 | (ulong)uVar3;
    *(int *)(iVar5 + 0x14) = iVar4 + 0x436;
    *(int *)(iVar5 + 0x16) = iVar6 + (-(uint)(0xfbd7 < uVar3) & 0x6c);
    uVar2 = *(undefined4 *)(iVar5 + 0x10);
    uVar8 = (undefined2)((ulong)uVar2 >> 0x10);
    iVar6 = (int)uVar2;
    uStack14 = (ulong)*(uint *)(iVar6 + 0xe);
    *(long *)(iVar5 + 0x18) = (long)(uStack14 * *(long *)(iVar6 + 0x4) + 0x1f) / 0x20 << 0x2;
  }
  return;
}



void __stdcall16far pass1_1008_4834(astruct_76 *param_1)

{
  code **ppcVar1;
  undefined4 *puVar2;
  ulong uVar3;
  uchar *extraout_DX;
  uchar *puVar4;
  ushort extraout_DX_00;
  astruct_76 *struct_var5_1;
  astruct_76 *struct_var5;
  astruct_76 *paStack10;
  
  struct_var5 = (astruct_76 *)((ulong)param_1 >> 0x10);
  struct_var5_1 = (astruct_76 *)param_1;
  puVar2 = (undefined4 *)*(uint *)((int)&struct_var5_1->field_0x8 + 0x2);
  puVar4 = (uchar *)struct_var5_1->field_0xc;
  if (((uint)puVar4 | (uint)puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    (**ppcVar1)();
    puVar4 = extraout_DX;
  }
  mem_op_1000_179c(0x14,puVar4,0x1000);
  paStack10 = (astruct_76 *)CONCAT22(puVar4,puVar2);
  if (((uint)puVar4 | (uint)puVar2) != 0x0) {
    uVar3 = *(ulong *)((int)&struct_var5_1->field_0xe + 0x2);
    uVar3 = uVar3 & 0xffff0000 | (ulong)((int)uVar3 + 0x28);
    struct_op_1008_4c98(paStack10,uVar3,0x100);
    *(undefined2 *)((int)&struct_var5_1->field_0x8 + 0x2) = (int)uVar3;
    struct_var5_1->field_0xc = extraout_DX_00;
    return;
  }
  *(undefined4 *)((int)&struct_var5_1->field_0x8 + 0x2) = 0x0;
  return;
}



ushort __stdcall16far pass1_1008_48aa(ulong param_1)

{
  return *(ushort *)((int)param_1 + 0xe);
}



ushort * __stdcall16far pass1_1008_48b8(ushort *param_1,byte param_2)

{
  pass1_1008_41bc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Instruction at (ram,0x10084942) overlaps instruction at (ram,0x10084941)
// 

void __stdcall16far
pass1_1008_48de(ushort param_1,ulong param_2,int param_3,uint param_4,ushort *param_5,int param_6,int param_7,
               byte *param_8,ushort param_9,ushort param_10,char param_11,ushort param_12,uchar param_13)

{
  byte *pbVar1;
  undefined4 uVar2;
  byte bVar3;
  ushort uVar4;
  byte bVar5;
  ushort uVar6;
  undefined *puVar7;
  int iVar8;
  undefined2 uVar9;
  
  uVar6 = param_4 & 0xff | (uint)(byte)((char)(param_4 >> 0x8) + (char)param_4 + param_11) << 0x8;
  puVar7 = (undefined *)(param_6 + 0x1);
  pbVar1 = (byte *)((int)param_5 + param_7);
  bVar5 = (byte)(param_4 & 0xff);
  *pbVar1 = *pbVar1 | bVar5;
  bVar3 = in(0x46);
  pbVar1 = (byte *)((int)param_5 + param_7);
  *pbVar1 = *pbVar1 | bVar5;
  if (param_3 == 0x1) {
    pbVar1 = (byte *)((int)param_5 + param_7);
    *pbVar1 = *pbVar1 | bVar5;
    iVar8 = param_7 + 0x1;
    pbVar1 = (byte *)((int)param_5 + iVar8);
    bVar5 = (byte)param_12;
    *pbVar1 = *pbVar1 | bVar5;
    pbVar1 = (byte *)((int)param_5 + iVar8);
    *pbVar1 = *pbVar1 | bVar5;
    *param_8 = bVar3;
    pbVar1 = (byte *)((int)param_5 + iVar8);
    *pbVar1 = *pbVar1 | bVar5;
    uVar6 = param_12;
    if (*pbVar1 != 0x0) {
      pbVar1 = (byte *)((int)param_5 + iVar8);
      *pbVar1 = *pbVar1 | bVar5;
      puVar7 = (undefined *)((int)&param_12 + 0x1);
      param_5 = (ushort *)(param_2 >> 0x8);
      *(undefined2 *)CONCAT13(param_13,param_2._1_3_) = 0x389a;
      param_5[0x1] = 0x1008;
      param_9 = (ushort)(CONCAT13(param_13,param_2._1_3_) >> 0x10);
      *(undefined4 *)(param_5 + 0x2) = 0x0;
      *(undefined4 *)(param_5 + 0x4) = 0x0;
      param_5[0x6] = 0xffff;
      *(undefined4 *)(param_5 + 0x7) = 0x0;
      *(undefined4 *)(param_5 + 0x9) = 0x0;
      *(undefined4 *)(param_5 + 0xb) = 0x0;
      *(undefined4 *)(param_5 + 0xd) = 0x0;
      param_5[0xf] = 0x0;
    }
  }
  else {
    param_5[0x11] = bVar3 | 0x800;
  }
  param_5[0x11] = *(ushort *)(puVar7 + 0xa);
  *param_5 = (ushort)&PTR_LOOP_1050_4c4c;
  param_5[0x1] = 0x1008;
  uVar4 = str_op_1008_60e8(*(char **)(puVar7 + 0xc),uVar6);
  uVar2 = *(undefined4 *)(puVar7 + 0x6);
  uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
  iVar8 = (int)uVar2;
  *(ushort *)(iVar8 + 0x8) = uVar4;
  *(ushort *)(iVar8 + 0xa) = uVar6;
  return;
}
