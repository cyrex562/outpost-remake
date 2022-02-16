void __stdcall16far pass1_1040_d76e(ulong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar2 + 0x94);
  pass1_1018_5742((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),*(ulong **)(iVar2 + 0x9c),*(ulong *)(iVar2 + 0x98));
  *(undefined4 *)(iVar2 + 0x9c) = 0x0;
  return;
}


void __stdcall16far pass1_1040_d0f8(astruct_57 *param_1,ushort param_2)

{
  uint uVar1;
  uchar *in_DX;
  ushort uVar2;
  uchar *puVar3;
  uchar *puVar4;
  astruct_438 *iVar5;
  int unaff_DI;
  undefined2 uVar5;
  ushort unaff_SS;
  ushort *puVar6;
  ulong uVar7;
  astruct_392 *iVar8;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x1845));
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_438 *)param_1;
  *(undefined4 *)&iVar5->field_0x94 = 0x0;
  iVar5->field_0x98 = _PTR_LOOP_1050_5f16;
  *(undefined4 *)&iVar5->field_0x9c = 0x0;
  iVar5->field_0xa0 = 0x0;
  *(undefined2 *)param_1 = 0xd8c4;
  iVar5->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x47,unaff_SS,in_DX,unaff_DI);
  uVar2 = (ushort)((ulong)puVar6 >> 0x10);
  iVar5->field_0x94 = (ushort)puVar6;
  iVar5->field_0x96 = uVar2;
  uVar7 = pass1_1018_5732(iVar5->field_0x94,uVar2,iVar5->field_0x98,(ushort)puVar6,uVar2,unaff_SS);
  puVar3 = (uchar *)(uVar7 >> 0x10);
  iVar5->field_0x9c = (uint)uVar7;
  iVar5->field_0x9e = puVar3;
  uVar1 = (uint)puVar3 | iVar5->field_0x9c;
  if (uVar1 == 0x0) {
    mem_op_1000_179c(0xc,puVar3,0x1000);
    puVar4 = (uchar *)((uint)puVar3 | uVar1);
    if (puVar4 == (uchar *)0x0) {
      *(undefined4 *)&iVar5->field_0x9c = 0x0;
    }
    else {
      pass1_1010_8ef2((ushort *)CONCAT22(puVar3,uVar1),puVar4,unaff_SS);
      iVar5->field_0x9c = uVar1;
      iVar5->field_0x9e = puVar4;
    }
  }
  return;
}


void __stdcall16far pass1_1040_ca16(astruct_57 *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  astruct_727 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x1840));
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_727 *)param_1;
  iVar1->field_0x94 = _PTR_LOOP_1050_5f0c;
  *(undefined4 *)&iVar1->field_0x98 = 0x0;
  iVar1->field_0x9c = 0x0;
  iVar1->field_0x9e = 0x0;
  *(undefined2 *)param_1 = 0xd07c;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3e,param_5,param_3,param_4);
  iVar1->field_0x98 = (int)puVar2;
  iVar1->field_0x9a = (int)((ulong)puVar2 >> 0x10);
  return;
}



ushort * __stdcall16far pass1_1040_c9cc(ushort *param_1,byte param_2)

{
  pass1_1040_c5ac(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


void __stdcall16far pass1_1040_c71e(ulong param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  pass1_1040_9252(param_1,param_2);
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(int *)(iVar1 + 0x28) = *(int *)(iVar1 + 0x24) / 0x2 - *(int *)(iVar1 + 0x2c) / 0x2;
  return;
}


void __stdcall16far pass1_1040_c630(ulong *param_1,ushort param_2,ushort param_3)

{
  int iVar1;
  code **ppcVar2;
  undefined4 uVar3;
  ulong uVar4;
  astruct_165 *iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_165 *)param_1;
  uVar3 = iVar4->field_0x42;
  if (*(int *)((int)uVar3 + 0x12) != 0x71) {
    iVar4->field_0x36 = 0x5;
    iVar4->field_0x26 = 0x5;
    iVar4->field_0x28 = 0x5;
    iVar1 = iVar4->field_0x36;
    iVar4->field_0x30 = iVar1;
    iVar4->field_0x2e = iVar1;
    if (PTR_LOOP_1050_5f02 == (undefined *)0x0) {
      _PTR_LOOP_1050_5f04 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0xff,param_3);
      param_2 = 0x1010;
      _PTR_LOOP_1050_5f08 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x100,param_3);
    }
    PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + 0x1;
    iVar4->field_0x8 = _PTR_LOOP_1050_5f04;
    iVar4->field_0xc = _PTR_LOOP_1050_5f08;
    pass1_1040_9618((ulong)param_1);
    iVar4->field_0x20 = 0x0;
    iVar4->field_0x1e = 0xc8;
    iVar4->field_0x22 = 0xa0;
    iVar4->field_0x24 = iVar4->field_0x2c + iVar4->field_0x36;
    iVar4->field_0x2e = iVar4->field_0x36 * 0x3 + iVar4->field_0x2a;
    iVar4->field_0x30 = iVar4->field_0x36;
    iVar4->field_0x32 = iVar4->field_0x22 - iVar4->field_0x36;
    iVar4->field_0x3c = 0x25;
    uVar4 = *param_1;
    ppcVar2 = (code **)((int)uVar4 + 0x4);
    (**ppcVar2)(param_2,param_1);
    ppcVar2 = (code **)((int)uVar4 + 0x8);
    (**ppcVar2)(param_2,(char)param_1,uVar5);
  }
  return;
}


ushort __stdcall16far pass1_1040_c60e(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x42) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x42);
    return *(ushort *)((int)uVar1 + 0x12);
  }
  return 0x0;
}


ulong __stdcall16far pass1_1040_c518(ulong param_1,byte param_2,ushort param_3)

{
  pass1_1040_bf92((ushort *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


void __stdcall16far pass1_1040_bf92(ushort *param_1,ushort param_2)

{
  astruct_514 *iVar1;
  uint uVar1;
  
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_514 *)param_1;
  *param_1 = 0xc53e;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  pass1_1010_1ea6(iVar1->field_0x6,(ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10,param_2);
  unk_destroy_win_op_1010_2fa0(iVar1->field_0x6,0x1010);
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}



void __stdcall16far pass1_1040_bfde(ulong param_1,ulong *param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  *(ulong **)(iVar3 + 0x6) = param_2;
  ppcVar1 = (code **)((int)*param_2 + 0x4);
  (**ppcVar1)();
  uVar2 = *(undefined4 *)(iVar3 + 0x6);
  *(undefined2 *)((int)uVar2 + 0x22) = *(undefined2 *)(iVar3 + 0x4);
  pass1_1010_2ee2(*(ulong **)(iVar3 + 0x6),param_3,0x1010);
  return;
}


ushort __stdcall16far pass1_1040_bb5a(ulong param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x94) + 0x8);
  (**ppcVar1)();
  return 0x0;
}



void __stdcall16far pass1_1040_b8be(ulong *param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*param_1 + 0x80);
  (**ppcVar1)();
  return;
}


ushort __stdcall16far pass1_1040_b316(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,int param_5)

{
  code **ppcVar1;
  ushort uStack4;
  
  if (param_5 == 0xf) {
    ppcVar1 = (code **)((int)*param_1 + 0x60);
    uStack4 = (**ppcVar1)();
  }
  else {
    if (param_5 == 0x111) {
      ppcVar1 = (code **)((int)*param_1 + 0x10);
      (**ppcVar1)();
      uStack4 = 0x1;
    }
    else {
      uStack4 = pass1_1040_79c0(param_1,(int *)param_2,param_3,param_4,param_5);
    }
  }
  return uStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1040_b17c(ulong param_1,undefined4 param_2,uchar *param_3,int param_4,int param_5,ushort param_6)

{
  int *piVar1;
  undefined4 uVar2;
  char *pcVar3;
  ushort uVar4;
  int iVar5;
  undefined2 uVar6;
  uchar *puVar7;
  ushort *puVar8;
  ushort *puStack12;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    uVar6 = (undefined2)(param_1 >> 0x10);
    iVar5 = (int)param_1;
    piVar1 = *(int **)(iVar5 + 0x90);
    puVar7 = (uchar *)((ulong)piVar1 >> 0x10);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    param_5 = *(int *)(iStack4 * 0x2 + (int)param_2);
    uVar2 = *(undefined4 *)((int)piVar1 + 0x2);
    *(int *)(iStack4 * 0xa + (int)uVar2 + 0x4) = param_5;
    iStack4 = iStack4 + 0x1;
    param_3 = puVar7;
  }
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_3,param_5);
  uVar4 = (ushort)((ulong)puVar8 >> 0x10);
  uVar2 = *(undefined4 *)(iVar5 + 0x90);
  puStack12 = *(ushort **)((int)uVar2 + 0x2);
  for (iStack4 = 0x0; piVar1 = *(int **)(iVar5 + 0x90), *piVar1 != iStack4 && iStack4 <= *piVar1;
      iStack4 = iStack4 + 0x1) {
    uVar2 = *(undefined4 *)(iVar5 + 0x90);
    uVar2 = *(undefined4 *)((int)uVar2 + 0x6);
    pcVar3 = pass1_1010_b038((uchar *)puVar8,(ushort)uVar2,(ushort)((ulong)uVar2 >> 0x10),
                             *(uchar **)((int)puStack12 + 0x4),param_4);
    string_1040_a626(puStack12,(char *)CONCAT22(uVar4,pcVar3),uVar4);
    puStack12 = (ushort *)((ulong)puStack12 & 0xffff0000 | (ulong)((int)puStack12 + 0xa));
  }
  return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_ac84(astruct_57 *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  astruct_726 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x1f3));
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_726 *)param_1;
  iVar1->field_0x94 = 0x0;
  *(undefined4 *)&iVar1->field_0x98 = 0x0;
  *(undefined2 *)param_1 = 0xafc4;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  iVar1->field_0x94 = _PTR_LOOP_1050_5ef0;
  _PTR_LOOP_1050_5ef0 = 0x0;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3d,param_5,param_3,param_4);
  iVar1->field_0x98 = (int)puVar2;
  iVar1->field_0x9a = (int)((ulong)puVar2 >> 0x10);
  return;
}


ushort * __stdcall16far pass1_1040_a204(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


ulong __stdcall16far
pass1_1040_a2cc(int param_1,ulong param_2,ulong param_3,ushort param_4,uchar *param_5,ushort param_6,ushort param_7)

{
  uint uVar1;
  
  if (param_3._2_2_ == 0x1826) {
    if (((int)param_3 == 0x1) || ((0x1 < (int)param_3 - 0x1U && ((int)param_3 - 0x3U < 0x2)))) {
      uVar1 = 0x1;
    }
    else {
      uVar1 = 0x0;
    }
    return (ulong)uVar1;
  }
  pass1_1040_b54a(param_1,(ushort)param_2,(ushort)(param_2 >> 0x10),param_3,param_5,param_6,param_7);
  return CONCAT22(param_5,param_4);
}


void __stdcall16far pass1_1040_8b3c(ushort param_1,ulong param_2,ulong param_3,ushort param_4)

{
  if ((param_3._2_2_ != (undefined *)0x0) &&
     ((param_3._2_2_ == (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1) ||
       param_3._2_2_ == (undefined *)&PTR_LOOP_1050_0002 ||
      (((undefined *)((int)&PTR_LOOP_1050_0002 + 0x1U) < param_3._2_2_ + -0x2 &&
       (param_3._2_2_ + -0x6 < (undefined *)&PTR_LOOP_1050_0002)))))) {
    PTR_LOOP_1050_5df4 = (undefined *)0x0;
    PTR_LOOP_1050_5df8 = param_3._2_2_;
    return;
  }
  post_win_msg_1040_7b3c
            ((ulong *)CONCAT22((int)param_2,param_1),(ushort)(param_2 >> 0x10),(ushort)param_3,(int)param_3._2_2_,
             param_4);
  return;
}



ushort * __stdcall16far pass1_1040_8e58(int param_1,ushort param_2,ushort param_3,ulong param_4)

{
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22((int)param_4,param_3),(ushort)(param_4 >> 0x10));
  *(ushort *)CONCAT22(param_2,param_1) = 0x8f3c;
  *(undefined2 *)(param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return (ushort *)CONCAT22(param_2,param_1);
}



void __stdcall16far pass1_1040_9422(ulong *param_1)

{
  code **ppcVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x8) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x10);
    (**ppcVar1)();
  }
  if (*(long *)((int)param_1 + 0x4) != 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x14);
    (**ppcVar1)();
  }
  return;
}



void __stdcall16far pass1_1040_9618(ulong param_1)

{
  undefined2 uVar1;
  astruct_162 *iVar2;
  undefined2 uVar2;
  ulong uVar3;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar2 = (astruct_162 *)param_1;
  uVar3 = pass1_1008_4772(iVar2->field_0x8);
  uVar1 = (undefined2)(uVar3 >> 0x10);
  iVar2->field_0x2a = *(undefined2 *)((int)uVar3 + 0x4);
  iVar2->field_0x2c = *(undefined2 *)((int)uVar3 + 0x8);
  return;
}



ushort __stdcall16far pass1_1040_824a(ulong param_1,int param_2)

{
  if (*(int *)((int)param_1 + 0x6) != param_2) {
    return 0x1;
  }
  return 0x0;
}



void __stdcall16far pass1_1040_807e(ulong param_1,ushort param_2,ushort param_3)

{
  uint uVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  uchar *in_DX;
  uint uVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  uchar *extraout_DX_00;
  uchar *puVar7;
  astruct_395 *iVar9;
  undefined2 uVar8;
  astruct_43 *paVar9;
  ulong uStack10;
  astruct_393 *iVar8;
  
  if (param_2 == 0x1) {
    pass1_1040_805a(in_DX);
    return;
  }
  paVar9 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_2,param_3);
  uVar5 = (uint)((ulong)paVar9 >> 0x10);
  puVar3 = (undefined4 *)paVar9;
  if ((uVar5 | (uint)puVar3) != 0x0) {
    ppcVar2 = (code **)((int)*(undefined4 *)paVar9 + 0x14);
    puVar4 = puVar3;
    (**ppcVar2)(0x1010,puVar3,uVar5);
    uStack10 = CONCAT22(extraout_DX,puVar4);
    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar9 = (astruct_395 *)param_1;
    puVar6 = extraout_DX;
    if (iVar9->field_0x70 != (astruct_90 *)0x0) {
      puVar4 = *(undefined4 **)&iVar9->field_0x70;
      uVar1 = *(uint *)((int)&iVar9->field_0x70 + 0x2);
      puVar6 = (uchar *)(uVar1 | (uint)puVar4);
      if (puVar6 != (uchar *)0x0) {
        ppcVar2 = (code **)*puVar4;
        (**ppcVar2)();
        puVar6 = extraout_DX_00;
      }
    }
    mem_op_1000_179c(0x14,puVar6,0x1000);
    puVar7 = (uchar *)((uint)puVar6 | (uint)puVar4);
    if (puVar7 == (uchar *)0x0) {
      puVar4 = (undefined4 *)0x0;
      puVar7 = (uchar *)0x0;
    }
    else {
      struct_1008_4c58((ushort *)CONCAT22(puVar6,puVar4));
    }
    *(undefined4 **)&iVar9->field_0x70 = puVar4;
    *(uchar **)((int)&iVar9->field_0x70 + 0x2) = puVar7;
    pass1_1008_4d84(iVar9->field_0x70,uStack10,puVar7);
    if (paVar9 != (astruct_43 *)0x0) {
      ppcVar2 = (code **)*(undefined4 *)paVar9;
      (**ppcVar2)(0x1008,puVar3,uVar5,0x1);
    }
    return;
  }
  return;
}


ulong __stdcall16far pass1_1040_805a(uchar *param_1)

{
  int unaff_DI;
  undefined2 uVar1;
  ushort unaff_SS;
  
  if (_PTR_LOOP_1050_4230 == 0x0) {
    mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x28,unaff_SS,param_1,unaff_DI);
  }
  uVar1 = (undefined2)((ulong)_PTR_LOOP_1050_4230 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)_PTR_LOOP_1050_4230 + 0x10),*(undefined2 *)((int)_PTR_LOOP_1050_4230 + 0xe));
}


ushort __stdcall16far pass1_1040_8054(void)

{
  return 0x0;
}


void __stdcall16far pass1_1040_78de(void)

{
  return;
}



void __stdcall16far pass1_1040_741e(ulong param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  uint uVar5;
  
  uVar5 = (uint)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1010_1ea6(*(ulong *)(iVar4 + 0x94),param_1 & 0xffff | (ulong)uVar5 << 0x10,param_2);
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x98);
  uVar2 = *(uint *)(iVar4 + 0x9a);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1010,puVar1,uVar2,0x1);
  }
  *(undefined4 *)(iVar4 + 0x98) = 0x0;
  *(undefined4 *)(iVar4 + 0x94) = 0x0;
  return;
}




int __stdcall16far pass1_1040_5eaa(ulong param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  switch(*(undefined2 *)(iVar1 + 0x9a)) {
  case 0x0:
  case 0x70:
  case 0x71:
    *(undefined2 *)(iVar1 + 0x98) = 0x0;
    return iVar1;
  case 0x1:
  case 0x2:
    *(undefined2 *)(iVar1 + 0x98) = 0xd;
    return iVar1;
  case 0x3:
    *(undefined2 *)(iVar1 + 0x98) = 0xe;
    return iVar1;
  case 0x4:
  case 0x4b:
    *(undefined2 *)(iVar1 + 0x98) = 0xf;
    break;
  case 0x5:
    *(undefined2 *)(iVar1 + 0x98) = 0x10;
    return iVar1;
  case 0x6:
    *(undefined2 *)(iVar1 + 0x98) = 0x11;
    return iVar1;
  case 0x7:
    *(undefined2 *)(iVar1 + 0x98) = 0x12;
    break;
  case 0x8:
    *(undefined2 *)(iVar1 + 0x98) = 0x13;
    break;
  case 0x9:
  case 0xa:
  case 0xb:
    *(undefined2 *)(iVar1 + 0x98) = 0x14;
    break;
  case 0xc:
    *(undefined2 *)(iVar1 + 0x98) = 0x18;
    break;
  case 0xd:
    *(undefined2 *)(iVar1 + 0x98) = 0x19;
    break;
  case 0xe:
  case 0x76:
    *(undefined2 *)(iVar1 + 0x98) = 0x17;
    break;
  case 0xf:
  case 0x10:
  case 0x11:
    *(undefined2 *)(iVar1 + 0x98) = 0x1a;
    break;
  case 0x12:
    *(undefined2 *)(iVar1 + 0x98) = 0x1b;
    break;
  case 0x13:
    *(undefined2 *)(iVar1 + 0x98) = 0x1c;
    break;
  case 0x14:
    *(undefined2 *)(iVar1 + 0x98) = 0x1d;
    break;
  case 0x15:
  case 0x16:
  case 0x17:
  case 0x18:
  case 0x19:
    *(undefined2 *)(iVar1 + 0x98) = 0x1e;
    break;
  case 0x1a:
    *(undefined2 *)(iVar1 + 0x98) = 0x1f;
    break;
  case 0x1b:
    *(undefined2 *)(iVar1 + 0x98) = 0x20;
    break;
  case 0x1c:
  case 0x1d:
  case 0x1e:
    *(undefined2 *)(iVar1 + 0x98) = 0x21;
    break;
  case 0x1f:
    *(undefined2 *)(iVar1 + 0x98) = 0x22;
    break;
  case 0x20:
    *(undefined2 *)(iVar1 + 0x98) = 0x23;
    break;
  case 0x21:
    *(undefined2 *)(iVar1 + 0x98) = 0x24;
    break;
  case 0x22:
    *(undefined2 *)(iVar1 + 0x98) = 0x25;
    break;
  case 0x23:
  case 0x24:
  case 0x25:
  case 0x26:
  case 0x27:
  case 0x28:
  case 0x29:
  case 0x2a:
  case 0x2b:
    *(undefined2 *)(iVar1 + 0x98) = 0x26;
    break;
  case 0x2c:
    *(undefined2 *)(iVar1 + 0x98) = 0x27;
    break;
  case 0x2d:
    *(undefined2 *)(iVar1 + 0x98) = 0x28;
    break;
  case 0x2e:
  case 0x2f:
  case 0x30:
  case 0x31:
    *(undefined2 *)(iVar1 + 0x98) = 0x29;
    break;
  case 0x32:
  case 0x33:
  case 0x34:
  case 0x35:
  case 0x4d:
    *(undefined2 *)(iVar1 + 0x98) = 0x2a;
    break;
  case 0x36:
    *(undefined2 *)(iVar1 + 0x98) = 0x2b;
    break;
  case 0x37:
  case 0x38:
  case 0x39:
    *(undefined2 *)(iVar1 + 0x98) = 0x2c;
    break;
  case 0x3a:
    *(undefined2 *)(iVar1 + 0x98) = 0x2d;
    break;
  case 0x3b:
  case 0x3c:
    *(undefined2 *)(iVar1 + 0x98) = 0x2e;
    break;
  case 0x3d:
    *(undefined2 *)(iVar1 + 0x98) = 0x2f;
    break;
  case 0x3e:
    *(undefined2 *)(iVar1 + 0x98) = 0x30;
    break;
  case 0x3f:
    *(undefined2 *)(iVar1 + 0x98) = 0x31;
    break;
  case 0x40:
    *(undefined2 *)(iVar1 + 0x98) = 0x32;
    break;
  case 0x41:
    *(undefined2 *)(iVar1 + 0x98) = 0x33;
    break;
  case 0x42:
    *(undefined2 *)(iVar1 + 0x98) = 0x34;
    break;
  case 0x43:
    *(undefined2 *)(iVar1 + 0x98) = 0x35;
    break;
  case 0x44:
    *(undefined2 *)(iVar1 + 0x98) = 0x36;
    break;
  case 0x45:
    *(undefined2 *)(iVar1 + 0x98) = 0x37;
    break;
  case 0x46:
    *(undefined2 *)(iVar1 + 0x98) = 0x38;
    break;
  case 0x47:
    *(undefined2 *)(iVar1 + 0x98) = 0x39;
    break;
  case 0x48:
  case 0x49:
  case 0x4a:
    *(undefined2 *)(iVar1 + 0x98) = 0x3a;
    break;
  case 0x4c:
    *(undefined2 *)(iVar1 + 0x98) = 0x3b;
    break;
  case 0x4e:
    *(undefined2 *)(iVar1 + 0x98) = 0x3c;
    break;
  case 0x4f:
  case 0x50:
    *(undefined2 *)(iVar1 + 0x98) = 0x3d;
    break;
  case 0x51:
  case 0x52:
  case 0x53:
  case 0x54:
  case 0x55:
    *(undefined2 *)(iVar1 + 0x98) = 0x3e;
    break;
  case 0x56:
  case 0x57:
  case 0x58:
  case 0x59:
  case 0x5a:
    *(undefined2 *)(iVar1 + 0x98) = 0x3f;
    break;
  case 0x5b:
    *(undefined2 *)(iVar1 + 0x98) = 0x40;
    break;
  case 0x5c:
  case 0x5d:
  case 0x5e:
    *(undefined2 *)(iVar1 + 0x98) = 0x41;
    break;
  case 0x5f:
  case 0x60:
  case 0x61:
    *(undefined2 *)(iVar1 + 0x98) = 0x42;
    break;
  case 0x62:
  case 0x63:
  case 0x64:
  case 0x65:
  case 0x66:
    *(undefined2 *)(iVar1 + 0x98) = 0x43;
    break;
  case 0x67:
  case 0x68:
    *(undefined2 *)(iVar1 + 0x98) = 0x44;
    break;
  case 0x69:
    *(undefined2 *)(iVar1 + 0x98) = 0x45;
    break;
  case 0x6a:
    *(undefined2 *)(iVar1 + 0x98) = 0x46;
    break;
  case 0x6b:
    *(undefined2 *)(iVar1 + 0x98) = 0x47;
    break;
  case 0x6c:
    *(undefined2 *)(iVar1 + 0x98) = 0x48;
    break;
  case 0x6d:
    *(undefined2 *)(iVar1 + 0x98) = 0x49;
    break;
  case 0x6e:
    *(undefined2 *)(iVar1 + 0x98) = 0x4a;
    break;
  case 0x6f:
    *(undefined2 *)(iVar1 + 0x98) = 0x4b;
    break;
  case 0x74:
    *(undefined2 *)(iVar1 + 0x98) = 0x15;
    break;
  case 0x75:
    *(undefined2 *)(iVar1 + 0x98) = 0x16;
    break;
  case 0x78:
  case 0x7a:
  case 0x7b:
  case 0x7c:
  case 0x7d:
  case 0x7e:
  case 0x7f:
  case 0x80:
  case 0x81:
  case 0x82:
    *(undefined2 *)(iVar1 + 0x98) = 0x4c;
  }
  return iVar1;
}


void __stdcall16far pass1_1040_6402(astruct_57 *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  astruct_725 *iVar2;
  undefined2 uVar2;
  ushort *puVar3;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1850,param_2);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_725 *)param_1;
  iVar2->field_0x8e = 0x0;
  iVar2->field_0x92 = (undefined4 *)0x0;
  *(undefined2 *)param_1 = 0x67ba;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  *(int *)&iVar2->field_0x92 = (int)puVar3;
  *(undefined2 *)((int)&iVar2->field_0x92 + 0x2) = (int)((ulong)puVar3 >> 0x10);
  ppcVar1 = (code **)((int)*iVar2->field_0x92 + 0x4);
  (**ppcVar1)();
  return;
}



void __stdcall16far pass1_1040_6470(astruct_18 *param_1,ushort param_2)

{
  astruct_18 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_18 *)param_1;
  param_1->field_0x0 = 0x67ba;
  iVar1->field_0x2 = (ushort)&PTR_LOOP_1050_1040;
  if (*(long *)&iVar1->field_0x92 != 0x0) {
    pass1_1010_1ea6(*(ulong *)&iVar1->field_0x92,(long)param_1,param_2);
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,iVar1->field_0x6);
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar1->field_0x8e,0x1000);
  ui_cleanup_op_1040_782c(param_1,0x1000);
  return;
}




astruct_18 * __stdcall16far pass1_1040_6794(astruct_18 *param_1,byte param_2,ushort param_3)

{
  pass1_1040_6470(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


void __stdcall16far pass1_1040_6826(astruct_57 *param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_2,0xfda));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0x94) = 0x0;
  *(undefined4 *)(iVar1 + 0x98) = 0x0;
  *(undefined2 *)param_1 = 0x6f32;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return;
}


ushort __stdcall16far
pass1_1040_68d2(ulong *param_1,int *param_2,ushort param_3,ushort param_4,int param_5,ushort param_6)

{
  code **ppcVar1;
  ushort uVar2;
  
  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566((int *)CONCAT22(param_3,param_2),param_6);
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,(ushort)param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (code **)((int)*param_1 + 0x80);
    (**ppcVar1)(param_6,param_1,param_2,CONCAT22(param_4,param_3));
  }
  return 0x1;
}



void __stdcall16far pass1_1040_6cac(ulong param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  uint uVar5;
  
  uVar5 = (uint)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1010_1ea6(*(ulong *)(iVar4 + 0x94),param_1 & 0xffff | (ulong)uVar5 << 0x10,param_2);
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x98);
  uVar2 = *(uint *)(iVar4 + 0x9a);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1010,puVar1,uVar2,0x1);
  }
  *(undefined4 *)(iVar4 + 0x98) = 0x0;
  *(undefined4 *)(iVar4 + 0x94) = 0x0;
  return;
}



void __stdcall16far pass1_1040_6fb6(astruct_57 *param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_2,0xfd9));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined4 *)(iVar1 + 0x94) = 0x0;
  *(undefined4 *)(iVar1 + 0x98) = 0x0;
  *(undefined2 *)param_1 = 0x76a4;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return;
}




void __stdcall16far pass1_1040_4d7e(ulong param_1)

{
  undefined4 uVar1;
  int *piVar2;
  undefined2 uVar3;
  int iStack8;
  ulong *puStack6;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  uVar1 = *(undefined4 *)((int)param_1 + 0x90);
  puStack6 = *(ulong **)((int)uVar1 + 0x2);
  iStack8 = 0x0;
  while ((piVar2 = *(int **)((int)param_1 + 0x90), *piVar2 != iStack8 && iStack8 <= *piVar2 &&
         (*(int *)((int)puStack6 + 0x4) != 0x1770))) {
    iStack8 = iStack8 + 0x1;
    puStack6 = (ulong *)((ulong)puStack6 & 0xffff0000 | (ulong)((int)puStack6 + 0xa));
  }
  pass1_1000_3e2c(*puStack6);
  return;
}
