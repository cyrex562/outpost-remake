
void __stdcall16far file_1008_6414(ulong *param_1,ulong param_2,ushort param_3,uchar *param_4)

{
  code **ppcVar1;
  undefined *puVar2;
  uint uVar3;
  undefined2 extraout_DX;
  int iVar4;
  undefined2 uVar5;
  astruct_76 *paStack42;
  undefined local_26 [0x24];
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x0;
  *(undefined4 *)(iVar4 + 0x4) = 0x0;
  puVar2 = local_26;
  struct_op_1008_48fe((astruct_81 *)CONCAT22(param_3,puVar2),0x1,(char *)param_2,(ushort)param_4);
  mem_op_1000_179c(0x1e,param_4,0x1000);
  paStack42 = (astruct_76 *)CONCAT22(param_4,puVar2);
  uVar3 = (uint)param_4 | (uint)puVar2;
  if (uVar3 == 0x0) {
    *param_1 = 0x0;
  }
  else {
    puVar2 = local_26;
    struct_op_1008_3f92(paStack42,(astruct_83 *)CONCAT22(param_3,puVar2));
    *(undefined **)param_1 = puVar2;
    *(uint *)(iVar4 + 0x2) = uVar3;
  }
  ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0x14);
  (**ppcVar1)();
  *(uint *)(iVar4 + 0x4) = (uint)puVar2;
  *(undefined2 *)(iVar4 + 0x6) = extraout_DX;
  close_file_1008_496c(local_26,param_3);
  return;
}



void __stdcall16far close_file_1008_496c(undefined2 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  undefined4 uVar3;
  code **ppcVar4;
  int iVar5;
  undefined2 uVar6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *param_1 = (int)&PTR_LOOP_1050_4c4c;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  puVar1 = (undefined4 *)*(uint *)(iVar5 + 0x4);
  uVar2 = *(uint *)(iVar5 + 0x6);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar5 + 0x8),0x1000);
  if (*(long *)(iVar5 + 0x1a) != 0x0) {
    uVar3 = *(undefined4 *)(iVar5 + 0x1a);
    call_fn_ptr_1000_0dc6((u16)uVar3,(u16)((ulong)uVar3 >> 0x10),0x1000);
  }
  if (*(int *)(iVar5 + 0xc) != -0x1) {
    _lclose16(0x1000);
  }
  *param_1 = 0x389a;
  *(undefined2 *)(iVar5 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uint16_t __stdcall16far read_file_1008_49e8(ulong param_1,uint16_t param_2,uint16_t param_3)

{
  HFILE16 HVar1;
  int iVar2;
  ulong uVar3;
  ulong uVar4;
  uchar *puVar5;
  uchar *puVar6;
  uchar *extraout_DX;
  int iVar7;
  int unaff_DI;
  undefined2 uVar8;
  uint16_t h_file;
  ushort unaff_SS;
  long lVar9;
  int local_18;
  undefined4 uStack22;
  undefined2 uStack10;
  uchar *puStack8;
  undefined4 uStack6;
  
  uVar8 = (undefined2)(param_1 >> 0x10);
  iVar7 = (int)param_1;
  if (*(long *)(iVar7 + 0x8) != 0x0) {
    if (*(int *)(iVar7 + 0x1e) != 0x0) {
      return (uint16_t)(uchar *)param_3;
    }
    h_file = param_2;
    if (*(int *)(iVar7 + 0xc) == -0x1) {
      h_file = (uint16_t)s_tile2_bmp_1050_1538;
      HVar1 = _lopen16((LPCSTR)param_2,0x0);
      *(HFILE16 *)(iVar7 + 0xc) = HVar1;
      if (HVar1 == 0xffff) {
        return (uint16_t)(uchar *)param_3;
      }
    }
    uStack6 = 0x0;
    lVar9 = WIN16_hread(h_file,0xe,ZEXT24(&local_18) << 0x10);
    param_3 = (uint16_t)(uchar *)((ulong)lVar9 >> 0x10);
    if (((int)lVar9 == 0xe) && ((uchar *)param_3 == (uchar *)0x0)) {
      uStack6 = uStack22;
      if (local_18 == (int)&PTR_LOOP_1050_4d42) {
        _llseek16((HFILE16)s_tile2_bmp_1050_1538,0x0,0x0);
        lVar9 = mem_op_1000_0a48(0x1,(uint)uStack6,(int)((ulong)uStack6 >> 0x10),_PTR_LOOP_1050_5f2c,0x1000);
        puVar6 = (uchar *)((ulong)lVar9 >> 0x10);
        *(undefined2 *)(iVar7 + 0x1a) = (int)lVar9;
        *(uchar **)(iVar7 + 0x1c) = puVar6;
        if (((uint)puVar6 | *(uint *)(iVar7 + 0x1a)) == 0x0) {
          return (uint16_t)puVar6;
        }
        lVar9 = WIN16_hread(0x1000,(SEGPTR)uStack6,
                            CONCAT22((int)*(undefined4 *)(iVar7 + 0x1a),(int)((ulong)uStack6 >> 0x10)));
        puVar5 = (uchar *)((ulong)lVar9 >> 0x10);
        uStack10 = (undefined2)lVar9;
        puStack8 = puVar5;
        _lclose16((HFILE16)s_tile2_bmp_1050_1538);
        *(undefined2 *)(iVar7 + 0xc) = 0xffff;
        *(undefined2 *)(iVar7 + 0x1e) = 0x1;
        *(undefined4 *)(iVar7 + 0xe) = *(undefined4 *)(iVar7 + 0x1a);
        uVar3 = *(ulong *)(iVar7 + 0x1a);
        iVar2 = (int)uVar3;
        uVar3 = uVar3 & 0xffff0000;
        *(ulong *)(iVar7 + 0x12) = uVar3 | iVar2 + 0xe;
        uVar3 = uVar3 | iVar2 + 0x436;
        *(ulong *)(iVar7 + 0x16) = uVar3;
        mem_op_1000_179c(0x14,puVar5,0x1000);
        puVar6 = (uchar *)((uint)puVar5 | (uint)uVar3);
        if (puVar6 == (uchar *)0x0) {
          *(undefined4 *)(iVar7 + 0x4) = 0x0;
        }
        else {
          uVar4 = *(ulong *)(iVar7 + 0x12);
          uVar4 = uVar4 & 0xffff0000 | (ulong)((int)uVar4 + 0x28);
          struct_op_1008_4c98((astruct_76 *)(uVar3 & 0xffff | ZEXT24(puVar5) << 0x10),uVar4,0x100);
          *(undefined2 *)(iVar7 + 0x4) = (int)uVar4;
          *(uchar **)(iVar7 + 0x6) = extraout_DX;
          puVar6 = extraout_DX;
        }
        if (*(int *)(iVar7 + 0x22) != 0x0) {
          pass1_1008_4b8e(param_1,puVar6,unaff_DI,unaff_SS);
          return (uint16_t)puVar6;
        }
        return (uint16_t)puVar6;
      }
    }
    _lclose16((HFILE16)s_tile2_bmp_1050_1538);
    *(undefined2 *)(iVar7 + 0xc) = 0xffff;
  }
  return param_3;
}


ulong __stdcall16far file_1008_4c26(ulong param_1,byte param_2)

{
  close_file_1008_496c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}

void __stdcall16far save_file_1008_3178(ulong param_1,int param_2,ushort param_3)

{
  char cVar1;
  undefined4 uVar2;
  int iVar3;
  undefined *puVar4;
  ushort uVar5;
  BOOL16 BVar6;
  uchar *in_DX;
  uint extraout_DX;
  ushort uVar7;
  int unaff_DI;
  undefined2 uVar8;
  char *pcVar9;
  INT16 in_buf_len_2;
  uint uVar10;
  char local_782 [0x104];
  undefined local_67e [0x8];
  astruct_18 *paStack1654;
  LPCSTR pCStack1650;
  UINT16 UStack1648;
  astruct_18 *paStack1646;
  undefined local_666 [0x100];
  char *pcStack1382;
  undefined4 local_562;
  undefined2 uStack1374;
  char *pcStack1370;
  ushort uStack1326;
  char acStack1305 [0x101];
  uint uStack1048;
  char local_416 [0x8];
  undefined2 uStack1038;
  undefined local_40c [0x102];
  ulong uStack778;
  ushort *puStack774;
  undefined local_302;
  undefined local_202 [0xff];
  char acStack259 [0x101];
  
  acStack259[1] = 0x0;
  local_302 = 0x0;
  local_202[0] = 0x0;
  puStack774 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,in_DX,unaff_DI);
  uVar8 = (undefined2)((ulong)puStack774 >> 0x10);
  iVar3 = (int)puStack774;
  uStack778 = *(undefined4 *)(iVar3 + 0x1a);
  uVar10 = *(uint *)(iVar3 + 0x1c);
  if ((uVar10 | (uint)uStack778) == 0x0) {
    paStack1646 = *(astruct_18 **)(iVar3 + 0x64);
    uVar10 = *(uint *)(iVar3 + 0x66);
    if ((uVar10 | (uint)paStack1646) != 0x0) {
      pass1_1008_5784((ulong *)CONCAT22(param_3,local_67e),(ulong)paStack1646 & 0xffff | (ulong)uVar10 << 0x10);
      puVar4 = local_67e;
      pass1_1008_5b12(puVar4,param_3);
      paStack1654 = (astruct_18 *)CONCAT22(extraout_DX,puVar4);
      if ((extraout_DX | (uint)puVar4) != 0x0) {
        uVar2 = *(undefined4 *)(puVar4 + 0x4);
        uStack778._0_2_ = (uint)uVar2;
        uVar10 = (uint)((ulong)uVar2 >> 0x10);
        goto LAB_1008_3206;
      }
    }
  }
  else {
LAB_1008_3206:
    unk_str_op_1000_3d3e((char *)CONCAT22(param_3,acStack259 + 0x1),(char *)CONCAT22(uVar10,(uint)uStack778));
  }
  pass1_1000_5008((ushort)local_40c,param_3,0x100,(int)&stack0xfffe);
  uStack1038 = str_op_1000_3da4((char *)CONCAT22(param_3,local_40c));
  if (local_40c[uStack1038 - 0x1] == '\\') {
    local_40c[uStack1038 - 0x1] = 0x0;
  }
  uStack1038 = str_op_1000_3da4((char *)CONCAT22(param_3,acStack259 + 0x1));
  if (acStack259[uStack1038] == '\\') {
    acStack259[uStack1038] = '\0';
  }
  pass1_1000_4f2e((ushort)&stack0xfffe);
  uVar8 = (undefined2)((ulong)puStack774 >> 0x10);
  uStack778 = *(ulong *)((int)puStack774 + 0x12);
  uVar10 = *(uint *)((int)puStack774 + 0x14);
  if ((uVar10 | (uint)uStack778) != 0x0) {
    unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_202),(char *)(uStack778 & 0xffff | (ulong)uVar10 << 0x10));
  }
  local_416[0] = '\0';
  pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_416),pcVar9);
  uStack1048 = str_op_1000_3da4((char *)CONCAT22(param_3,local_416));
  uStack1038 = uStack1048;
  for (; -0x1 < (int)uStack1048; uStack1048 = uStack1048 - 0x1) {
    if (local_416[uStack1048] == '.') {
      unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_67e),(char *)CONCAT22(param_3,local_416 + uStack1048 + 0x1));
      unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_416),(char *)CONCAT22(param_3,local_67e));
    }
  }
  acStack1305[1] = 0x0;
  pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  uVar5 = (ushort)((ulong)pcVar9 >> 0x10);
  unk_str_op_1000_3d3e((char *)CONCAT22(param_3,acStack1305 + 0x1),pcVar9);
  uStack1038 = str_op_1000_3da4((char *)CONCAT22(param_3,acStack1305 + 0x1));
  cVar1 = acStack1305[uStack1038];
  uStack1048 = 0x0;
  while (acStack1305[uStack1048 + 0x1] != '\0') {
    if (acStack1305[uStack1048 + 0x1] == cVar1) {
      acStack1305[uStack1048 + 0x1] = '\0';
    }
    uStack1048 = uStack1048 + 0x1;
  }
  pass1_1000_4906((astruct_20 *)CONCAT22(param_3,&local_562),(WNDCLASS16 *)0x0,0x48);
  local_562 = 0x48;
  uStack1374 = *(undefined2 *)((int)param_1 + 0x8);
  pcStack1370 = acStack1305 + 0x1;
  pcStack1382 = (char *)0x0;
  local_666[0] = 0x0;
  in_buf_len_2 = (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10);
  if (param_2 == 0x1) {
    pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,in_buf_len_2,0x1010);
    uVar5 = (ushort)((ulong)pcVar9 >> 0x10);
    unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_666),pcVar9);
    BVar6 = GetOpenFileName16(0x1000);
  }
  else {
    if (param_2 != 0x2) {
      debug_print_1008_6048((ulong)s_Unsupported_FileStructType_in_Op_1050_01ca,0x1000,param_3);
      goto LAB_1008_3461;
    }
    pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,in_buf_len_2,0x1010);
    uVar5 = (ushort)((ulong)pcVar9 >> 0x10);
    unk_str_op_1000_3d3e((char *)CONCAT22(param_3,local_666),pcVar9);
    BVar6 = GetSaveFileName16(0x1000);
  }
  if (BVar6 != 0x0) {
    pcStack1382 = (char *)CONCAT22(param_3,local_202);
  }
LAB_1008_3461:
  if (pcStack1382 != (char *)0x0) {
    if ((int)uStack1326 < 0x0) {
      paStack1654 = (astruct_18 *)
                    load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      uVar7 = (ushort)((ulong)paStack1654 >> 0x10);
      uVar5 = str_op_1008_60e8((char *)paStack1654,uVar7);
      paStack1654 = (astruct_18 *)CONCAT22(uVar7,uVar5);
      pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      UStack1648 = (UINT16)((ulong)pcVar9 >> 0x10);
      pCStack1650 = (LPCSTR)pcVar9;
      MessageBox16(0x1010,(LPCSTR)&PTR_LOOP_1050_0010,pCStack1650,UStack1648);
      pcStack1382 = (char *)0x0;
      paStack1646 = paStack1654;
      fn_ptr_1000_17ce(paStack1654,0x1000);
    }
    else {
      str_op_1000_3dbe((char *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,local_782)),
                       (char *)CONCAT22(param_3,local_202),uStack1326);
      local_782[uStack1326] = '\0';
      if (local_782[0] != '\0') {
        pass1_1010_60cc((ulong)puStack774,(char *)CONCAT22(param_3,local_782),uVar5);
      }
    }
  }
  pass1_1000_4f2e((ushort)&stack0xfffe);
  return;
}


