
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_bffa(ulong param_1,int param_2,uchar *param_3,ushort param_4)

{
  undefined2 *puVar1;
  undefined4 uVar2;
  uint uVar3;
  ushort uVar4;
  astruct_257 *puVar5;
  astruct_254 *iVar6;
  astruct_255 *iVar7;
  astruct_256 *iVar8;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  astruct_43 *paVar8;
  ushort uStack34;
  int iStack30;
  undefined local_18 [0x16];
  
  mem_op_1000_179c(0xa,param_3,0x1000);
  local_18._18_4_ = (undefined4 *)CONCAT22(param_3,param_2);
  bad_1010_bf08((ushort)param_1,(ushort)(param_1 >> 0x10),0x2000000);
  uVar6 = (undefined2)((ulong)local_18._18_4_ >> 0x10);
  iVar6 = (astruct_254 *)local_18._18_4_;
  iVar6->field_0x8 = param_2;
  if (param_2 == 0x0) {
    iVar6->field_0x8 = 0x1;
  }
  uVar3 = iVar6->field_0x8 << 0x2;
  mem_op_1000_179c(uVar3,param_3,0x1000);
  uVar6 = (undefined2)((ulong)local_18._18_4_ >> 0x10);
  iVar7 = (astruct_255 *)local_18._18_4_;
  *(uint *)local_18._18_4_ = uVar3;
  iVar7->field_0x2 = param_3;
  if (((uint)param_3 | *(uint *)local_18._18_4_) == 0x0) {
    iVar7->field_0x8 = 0x0;
  }
  else {
    uVar4 = iVar7->field_0x8 * 0x2;
    mem_op_1000_179c(uVar4,param_3,0x1000);
    uVar6 = (undefined2)((ulong)local_18._18_4_ >> 0x10);
    iVar8 = (astruct_256 *)local_18._18_4_;
    iVar8->field_0x4 = uVar4;
    iVar8->field_0x6 = param_3;
    if (((uint)param_3 | iVar8->field_0x4) != 0x0) {
      paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1b4,param_4);
      uVar3 = (uint)((ulong)paVar8 >> 0x10);
      puVar1 = (undefined2 *)*local_18._18_4_;
      *puVar1 = (int)paVar8;
      *(uint *)((int)puVar1 + 0x2) = uVar3;
      *(undefined2 *)*(undefined4 *)((int)local_18._18_4_ + 0x4) = 0x0;
      pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_18),0x1,0x0,0x200);
      iStack30 = 0x1;
      while( true ) {
        puVar5 = (astruct_257 *)local_18;
        pass1_1028_e4ec(CONCAT22(param_4,puVar5));
        if ((uVar3 | (uint)puVar5) == 0x0) break;
        uVar6 = *puVar5->field_0x10;
        uStack34 = 0x0;
        switch(uVar6) {
        case 0x1:
          uStack34 = 0x1b5;
          break;
        case 0x2:
          uStack34 = 0x1b6;
          break;
        case 0x3:
          uStack34 = 0x1b7;
          break;
        case 0x4:
          uStack34 = 0x1b8;
          break;
        case 0x5:
          uStack34 = 0x1b9;
          break;
        case 0x6:
          uStack34 = 0x1ba;
          break;
        case 0x7:
          uStack34 = 0x1bb;
          break;
        case 0x8:
          uStack34 = 0x1bc;
          break;
        case 0x9:
          uStack34 = 0x1bd;
          break;
        case 0xa:
          uStack34 = 0x1be;
        }
        paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,uStack34,param_4);
        uVar3 = (uint)((ulong)paVar8 >> 0x10);
        uVar7 = (undefined2)((ulong)*local_18._18_4_ >> 0x10);
        iVar5 = (int)*local_18._18_4_;
        *(undefined2 *)(iVar5 + iStack30 * 0x4) = (int)paVar8;
        *(uint *)(iVar5 + iStack30 * 0x4 + 0x2) = uVar3;
        uVar2 = *(undefined4 *)((int)local_18._18_4_ + 0x4);
        *(undefined2 *)((int)uVar2 + iStack30 * 0x2) = uVar6;
        iStack30 = iStack30 + 0x1;
      }
      return;
    }
  }
  return;
}



void __stdcall16far pass1_1010_c1ba(ushort param_1,ushort param_2,int param_3,uint param_4,ushort param_5)

{
  undefined *puVar1;
  int iStack28;
  undefined local_16 [0x12];
  undefined2 uStack4;
  
  uStack4 = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_5,local_16),0x1,0x0,0x200);
  iStack28 = 0x1;
  while( true ) {
    puVar1 = local_16;
    pass1_1028_e4ec(CONCAT22(param_5,puVar1));
    param_4 = param_4 | (uint)puVar1;
    if ((param_4 == 0x0) || (iStack28 == param_3)) break;
    iStack28 = iStack28 + 0x1;
  }
  return;
}



char * __stdcall16far pass1_1010_c234(uint param_1,uchar *param_2,int param_3,ushort param_4)

{
  char *pcVar1;
  
  pass1_1010_c28a(param_2,param_3,param_4);
  if (((uint)param_2 | param_1) == 0x0) {
    return (char *)0x0;
  }
  pcVar1 = pass1_1038_4d28(CONCAT22(param_2,param_1));
  return pcVar1;
}



void __stdcall16far
pass1_1010_c25e(ushort param_1,ushort param_2,char *param_3,uint param_4,uchar *param_5,int param_6,ushort param_7)

{
  if (param_3 != (char *)0x0) {
    pass1_1010_c28a(param_5,param_6,param_7);
    if (((uint)param_5 | param_4) != 0x0) {
      pass1_1038_4d3c(CONCAT22(param_5,param_4),param_3,(uint)param_5 | param_4);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_c28a(uchar *param_1,int param_2,ushort param_3)

{
  uint uVar1;
  undefined2 uVar2;
  uint uVar3;
  uint uVar4;
  ushort *puVar5;
  
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,param_1,param_2);
  uVar2 = (undefined2)((ulong)puVar5 >> 0x10);
  uVar1 = *(uint *)((int)puVar5 + 0x24);
  uVar4 = *(uint *)((int)puVar5 + 0x26);
  uVar3 = uVar4 | uVar1;
  if (uVar3 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,uVar4);
    if ((uVar4 | uVar3) != 0x0) {
      return;
    }
  }
  return;
}



void __stdcall16far pass1_1010_c2d8(ushort param_1,ushort param_2,long param_3)

{
  undefined2 uVar1;
  uint uStack6;
  
  if ((param_3 != 0x0) &&
     (uVar1 = (undefined2)((ulong)param_3 >> 0x10), uStack6 = (uint)*(undefined4 *)((int)param_3 + 0x2e),
     (*(uint *)((int)param_3 + 0x30) | uStack6) != 0x0)) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_c312(void)

{
  return CONCAT22(*(undefined2 *)((int)_PTR_LOOP_1050_65e2 + 0x2),*_PTR_LOOP_1050_65e2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_c320(ushort param_1,ushort param_2,uchar *param_3,ulong param_4,uchar *param_5)

{
  ulong uVar1;
  uchar *puStack6;
  
  puStack6 = param_3;
  if (param_3 == (uchar *)0x0) {
    mem_op_1000_179c(0x100,param_5,0x1000);
    puStack6 = (uchar *)((ulong)param_3 & 0xffff | ZEXT24(param_5) << 0x10);
  }
  uVar1 = struct_op_1030_73a8(param_4);
  switch(*(undefined2 *)((int)uVar1 + 0x12)) {
  case 0x1:
  case 0x2:
  case 0x4:
    break;
  case 0x3:
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x7:
    break;
  case 0x8:
    break;
  case 0x9:
    break;
  default:
    *puStack6 = '\0';
    return;
  }
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,(char *)puStack6,
             (short)((ulong)puStack6 >> 0x10));
  return;
}



void __stdcall16far
pass1_1010_c3c2(ushort param_1,ushort param_2,ulong param_3,ulong param_4,uchar *param_5,uchar param_6,ushort param_7)

{
  ushort uVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  undefined local_40c [0x400];
  ushort uStack12;
  ulong uStack10;
  ulong uStack6;
  
  uStack6 = param_3;
  if (param_3 == 0x0) {
    mem_op_1000_179c(0x100,param_5,0x1000);
    uStack6 = param_3 & 0xffff | ZEXT24(param_5) << 0x10;
  }
  uStack10 = struct_op_1030_73a8(param_4);
  uVar2 = (undefined2)(uStack10 >> 0x10);
  uStack12 = *(ushort *)((int)uStack10 + 0xc);
  uVar3 = uVar2;
  uVar1 = pass1_1020_bd80(uStack12);
  unk_str_op_1000_3d3e((char *)CONCAT22(param_7,local_40c),(char *)CONCAT22(uVar3,uVar1));
  pass1_1030_8086(param_4);
  sys_1000_3f9c((uchar *)uStack6,(uchar *)(uStack6 >> 0x10),0x38c5,(ushort)&USHORT_1050_1050,(ushort)local_40c,
                &stack0xfffe,uVar2,0x1000,param_7,param_6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
string_op_1010_c446(ushort param_1,uchar param_2,uchar *param_3,ulong param_4,char *param_5,ulong param_6)

{
  int iVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  ulong uVar4;
  char *pcVar5;
  ushort uVar6;
  ushort uVar7;
  char *in_buffer_4;
  uchar *in_buf_len_5;
  uint uStack22;
  char *pcStack6;
  
  pcStack6 = param_5;
  if (param_5 == (char *)0x0) {
    mem_op_1000_179c(0x100,param_3,0x1000);
    pcStack6 = (char *)((ulong)param_5 & 0xffff | ZEXT24(param_3) << 0x10);
  }
  uVar4 = struct_op_1030_73a8(param_6);
  uVar2 = (undefined2)(uVar4 >> 0x10);
  uVar3 = uVar2;
  struct_1010_dd5e((ushort)param_4,(ushort)(param_4 >> 0x10),param_6);
  iVar1 = *(int *)((int)uVar4 + 0x12);
  if (0x6 < iVar1 - 0x3U) {
    return;
  }
  in_buffer_4 = (char *)pcStack6;
  in_buf_len_5 = (uchar *)((ulong)pcStack6 >> 0x10);
  uVar7 = (ushort)_PTR_LOOP_1050_14cc;
  uVar6 = (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10);
  switch(iVar1) {
  default:
    break;
  case 0x6:
    load_string_1010_84e0(0x1010,uVar7,uVar6,0x3ff,in_buffer_4,(short)in_buf_len_5);
    uStack22 = str_op_1000_3da4(pcStack6);
    pcVar5 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1000);
    uVar7 = (ushort)pcVar5;
    uVar6 = (ushort)s_____s__lu_1050_38d7;
    goto LAB_1010_c4f9;
  case 0x7:
  case 0x9:
    break;
  case 0x8:
    load_string_1010_84e0(0x1010,uVar7,uVar6,0x3ff,in_buffer_4,(short)in_buf_len_5);
    uStack22 = str_op_1000_3da4(pcStack6);
    pcVar5 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1000);
    uVar7 = (ushort)pcVar5;
    uVar6 = (ushort)s_____s__lu_1050_38cd;
LAB_1010_c4f9:
    sys_1000_3f9c((uchar *)(in_buffer_4 + uStack22),in_buf_len_5,uVar6,(ushort)&USHORT_1050_1050,uVar7,&stack0xfffe,
                  uVar3,0x1000,param_1,param_2);
    return;
  }
  load_string_1010_84e0(0x1010,uVar7,uVar6,0x3ff,in_buffer_4,(short)in_buf_len_5);
  return;
}



void __stdcall16far
pass1_1010_c58as(ushort param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5,ushort param_6)

{
  uint uVar1;
  uint uVar2;
  ulong uVar3;
  uchar *puVar4;
  uchar *puVar5;
  undefined2 unaff_SS;
  int *piStack26;
  undefined2 uStack18;
  int iStack16;
  undefined **ppuStack14;
  int *piStack6;
  
  uVar3 = (ulong)param_5;
  mem_op_1000_179c(0x18,(uchar *)param_6,0x1000);
  uVar1 = (uint)uVar3;
  puVar4 = (uchar *)(param_6 | uVar1);
  if (puVar4 == (uchar *)0x0) {
    uVar1 = 0x0;
    puVar4 = (uchar *)0x0;
  }
  else {
    struct_1040_a598((ushort *)(uVar3 & 0xffff | (ulong)param_6 << 0x10));
  }
  piStack6 = (int *)CONCAT22(puVar4,uVar1);
  puVar5 = (uchar *)((uint)puVar4 | uVar1);
  if (puVar5 == (uchar *)0x0) {
    return;
  }
  ppuStack14 = (undefined **)0x0;
  uStack18 = 0x0;
  iStack16 = 0x0;
  switch(param_3) {
  case 0x5:
    ppuStack14 = (undefined **)&USHORT_1050_352c;
    uStack18 = 0xfa4;
    iStack16 = 0x30;
    break;
  default:
    if (piStack6 == (int *)0x0) {
      return;
    }
    pass1_1040_a5d0(CONCAT22(puVar4,uVar1));
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(puVar4,uVar1),0x1000);
    return;
  case 0xa:
    ppuStack14 = (undefined **)&USHORT_1050_358c;
    uStack18 = 0xfb3;
    iStack16 = 0x51;
    break;
  case 0xb:
    ppuStack14 = (undefined **)&USHORT_1050_358c;
    uStack18 = 0xfb4;
    iStack16 = 0x51;
    break;
  case 0xc:
    ppuStack14 = (undefined **)&USHORT_1050_362e;
    uStack18 = 0xfb6;
    iStack16 = 0x30;
    break;
  case 0x10:
    ppuStack14 = &PTR_DAT_1050_1805_1050_368e;
    uStack18 = 0xfc4;
    iStack16 = 0x3c;
    break;
  case 0x11:
    ppuStack14 = &PTR_DAT_1050_1805_1050_3706;
    uStack18 = 0xfc1;
    iStack16 = 0x4b;
    break;
  case 0x12:
    ppuStack14 = (undefined **)&USHORT_1050_379c;
    uStack18 = 0xfc5;
    iStack16 = 0x8;
    break;
  case 0x13:
    puVar5 = puVar4;
    pass1_1010_debe(CONCAT22(param_2,param_1),param_3,(ushort *)CONCAT22(puVar4,uVar1 + 0x10),
                    (ulong *)CONCAT22(puVar4,uVar1 + 0xc),param_4,unaff_SS);
    ppuStack14 = (undefined **)&USHORT_1050_37ac;
    uStack18 = 0xfc6;
    iStack16 = 0x1;
    break;
  case 0x15:
    *(ulong *)(uVar1 + 0x6) = param_4;
    *(ushort *)(uVar1 + 0xa) = param_3;
    break;
  case 0x16:
    ppuStack14 = (undefined **)&USHORT_1050_37ae;
    uStack18 = 0x157;
    iStack16 = 0x4;
    break;
  case 0x17:
    ppuStack14 = (undefined **)&USHORT_1050_37b6;
    iStack16 = 0x2c;
    uStack18 = 0xfd8;
  }
  if (ppuStack14 != (undefined **)0x0) {
    *piStack6 = iStack16;
    uVar2 = iStack16 * 0xa + 0x2;
    mem_op_1000_179c(uVar2,puVar5,0x1000);
    piStack26 = (int *)CONCAT22(puVar5,uVar2);
    if (((uint)puVar5 | uVar2) == 0x0) {
      *(undefined4 *)(uVar1 + 0x2) = 0x0;
    }
    else {
      *piStack26 = iStack16;
      pass1_1000_5586((uchar *)0xa564,(ushort)&PTR_LOOP_1050_1040,iStack16,0xa,uVar2 + 0x2,(ushort)puVar5);
      *(int *)(uVar1 + 0x2) = uVar2 + 0x2;
      *(uchar **)(uVar1 + 0x4) = puVar5;
    }
    *(ulong *)(uVar1 + 0x6) = param_4;
    *(ushort *)(uVar1 + 0xa) = param_3;
    *(undefined2 *)(uVar1 + 0x12) = uStack18;
    pass1_1010_a50c((astruct_20 *)CONCAT22(param_2,param_1),(ulong)ppuStack14,CONCAT22(puVar4,uVar1));
  }
  return;
}



void __stdcall16far pass1_1010_c7e2(ulong param_1,ulong param_2,int *param_3)

{
  undefined4 uVar1;
  char *pcVar2;
  ushort in_DX;
  int iVar3;
  int unaff_SI;
  undefined2 uVar4;
  ushort *puStack8;
  int iStack4;
  
  iStack4 = 0x0;
  while( true ) {
    uVar4 = (undefined2)((ulong)param_3 >> 0x10);
    iVar3 = (int)param_3;
    if (*param_3 == iStack4 || *param_3 < iStack4) break;
    uVar1 = *(undefined4 *)(iVar3 + 0x2);
    *(undefined2 *)(iStack4 * 0xa + (int)uVar1 + 0x4) = *(undefined2 *)(iStack4 * 0x2 + (int)param_2);
    iStack4 = iStack4 + 0x1;
  }
  puStack8 = *(ushort **)(iVar3 + 0x2);
  for (iStack4 = 0x0; *param_3 != iStack4 && iStack4 <= *param_3; iStack4 = iStack4 + 0x1) {
    uVar1 = *(undefined4 *)(iVar3 + 0x6);
    pcVar2 = pass1_1010_b038((uchar *)param_1,(ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),
                             *(uchar **)((int)puStack8 + 0x4),unaff_SI);
    string_1040_a626(puStack8,(char *)CONCAT22(in_DX,pcVar2),in_DX);
    puStack8 = (ushort *)((ulong)puStack8 & 0xffff0000 | (ulong)((int)puStack8 + 0xa));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1010_c864(ulong param_1,ushort *param_2,astruct_104 *param_3,uchar *param_4,uchar *param_5,uchar param_6)

{
  long *plVar1;
  long lVar2;
  code **ppcVar3;
  undefined4 uVar4;
  char *pcVar5;
  ushort uVar6;
  ushort uVar7;
  ulong uVar8;
  uint uVar9;
  uint extraout_DX;
  uint extraout_DX_00;
  uint uVar10;
  int iVar11;
  int iVar12;
  undefined2 uVar13;
  undefined2 uVar14;
  undefined2 uVar15;
  ushort uVar16;
  ushort uVar17;
  undefined4 local_1f0;
  astruct_18 *paStack412;
  undefined4 uStack408;
  undefined4 uStack404;
  ushort uStack400;
  undefined4 uStack398;
  undefined4 local_18a;
  undefined4 local_f6;
  ushort *puStack98;
  int iStack94;
  ulong uStack92;
  int iStack88;
  uint uStack86;
  undefined local_54 [0x52];
  
  uStack86 = 0x0;
  uVar13 = (undefined2)((ulong)param_3 >> 0x10);
  iVar11 = (int)param_3;
  iStack88 = *(int *)param_3;
  uVar14 = 0x0;
  uStack92 = 0x0;
  uVar16 = (ushort)param_1;
  uVar17 = (ushort)(param_1 >> 0x10);
  pass1_1010_c320(uVar16,uVar17,(uchar *)0x0,*(ulong *)(iVar11 + 0x6),param_4);
  unk_str_op_1000_3d3e((char *)CONCAT22(param_5,local_54),(char *)CONCAT22(param_4,uVar14));
  puStack98 = *(ushort **)(iVar11 + 0x2);
  uStack404._2_2_ = *(ushort *)(iVar11 + 0x4);
  *(ushort *)((int)puStack98 + 0x4) = *param_2;
  string_1040_a626(puStack98,(char *)CONCAT22(param_5,local_54),uStack404._2_2_);
  iStack94 = 0x0;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_f6),(WNDCLASS16 *)0x0,0x94);
  uStack404._0_2_ = pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_18a),(WNDCLASS16 *)0x0,0x94);
  uStack398 = 0x0;
  for (uStack400 = 0x1; (int)uStack400 < 0x25; uStack400 = uStack400 + 0x1) {
    uStack404 = (astruct_18 *)
                pass1_1030_7c28(*(ulong *)(iVar11 + 0x6),uStack400,(uint)(uint *)uStack404,uStack404._2_2_,
                                (ushort)param_5);
    uStack404._2_2_ = (uint)((ulong)uStack404 >> 0x10) | (uint)(uint *)uStack404;
    if (uStack404 != (astruct_18 *)0x0) {
      pcVar5 = string_1020_c0d8(uStack400);
      uStack408 = CONCAT22(uStack404._2_2_,pcVar5);
      uVar9 = uStack404._2_2_ | (uint)pcVar5;
      if (uVar9 == 0x0) {
        unk_str_op_1000_3d3e((char *)(&local_f6)[(uint)uStack398],s_Null_Ptr_1050_38e1);
      }
      else {
        uVar6 = str_op_1008_60e8((char *)CONCAT22(uStack404._2_2_,pcVar5),uVar9);
        *(ushort *)(&local_f6 + (uint)uStack398) = uVar6;
        *(uint *)((int)&local_f6 + (uint)uStack398 * 0x4 + 0x2) = uVar9;
      }
      *(uint **)(&local_18a + (uint)uStack398) = (uint *)uStack404;
      *(uint *)((int)&local_18a + (uint)uStack398 * 0x4 + 0x2) = uStack404._2_2_;
      uStack398 = uStack398 + 0x1;
    }
  }
  uStack92 = uStack398;
  if (0x13 < uStack398) {
    iStack94 = 0x1;
  }
  uStack86 = pass1_1010_db2e(uVar16,uVar17,0x1,CONCAT22(param_5,&local_f6),CONCAT22(param_5,&local_18a),(ulong)param_2,
                             (int *)param_3,param_5,param_6);
  while (uVar8 = uStack398 - 0x1, uStack398 != 0x0) {
    uStack398._0_2_ = (uint)uVar8;
    paStack412 = (astruct_18 *)(&local_f6)[(uint)uStack398];
    uStack404 = paStack412;
    uStack398 = uVar8;
    fn_ptr_1000_17ce(paStack412,0x1000);
  }
  uVar15 = 0x1000;
  uStack398 = uVar8;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_18a),(WNDCLASS16 *)0x0,0x54);
  uVar4 = *(undefined4 *)(iVar11 + 0x6);
  uVar14 = (undefined2)((ulong)uVar4 >> 0x10);
  iVar12 = (int)uVar4;
  uStack404 = (astruct_18 *)*(undefined4 *)(iVar12 + 0x1e);
  uVar9 = *(uint *)(iVar12 + 0x20) | (uint)(uint *)uStack404;
  uVar8 = (ulong)uVar9;
  if (uVar9 != 0x0) {
    uStack398 = 0x0;
    while( true ) {
      uVar4 = *(undefined4 *)uStack404;
      ppcVar3 = (code **)((int)uVar4 + 0x10);
      (**ppcVar3)(uVar15,(int)uStack404,(int)((ulong)uStack404 >> 0x10));
      if ((extraout_DX < uStack398._2_2_) || ((extraout_DX <= uStack398._2_2_ && ((uint)uVar8 <= (uint)uStack398))))
      break;
      ppcVar3 = (code **)((int)uVar4 + 0x4);
      (**ppcVar3)(uVar15,uStack404,(char)uStack398,uStack398._2_2_);
      uVar9 = (uint)uVar8;
      uVar10 = extraout_DX_00 | uVar9;
      if (uVar10 != 0x0) {
        uVar15 = SUB42(&USHORT_1050_1028,0x0);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar9,extraout_DX_00);
        uStack408 = CONCAT22(uVar10,uVar9);
        if ((uVar10 | uVar9) == 0x0) {
          return;
        }
        iVar12 = *(int *)(uVar9 + 0xc);
        uVar8 = (ulong)(iVar12 - 0x1U);
        if (((0x0 < iVar12) && (!SBORROW2(iVar12,0x1))) &&
           (uVar8 = (ulong)(iVar12 - 0x6U), iVar12 - 0x6U == 0x0 || (int)(iVar12 - 0x1U) < 0x5)) {
          plVar1 = &local_18a + iVar12;
          *plVar1 = *plVar1 + 0x1;
        }
      }
      uStack398 = uStack398 + 0x1;
    }
    uVar9 = extraout_DX;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_f6),(WNDCLASS16 *)0x0,0x54);
    uVar6 = 0x1000;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_1f0),(WNDCLASS16 *)0x0,0x54);
    uStack398 = 0x0;
    for (uStack400 = 0x1; (int)uStack400 < 0x15; uStack400 = uStack400 + 0x1) {
      if ((&local_18a)[uStack400] != 0x0) {
        pcVar5 = string_op_1020_c222(uStack400);
        uVar10 = uVar9 | (uint)pcVar5;
        if (uVar10 == 0x0) {
          uVar6 = 0x1000;
          unk_str_op_1000_3d3e((char *)(&local_f6)[(uint)uStack398],s_Null_Ptr_1050_38ea);
        }
        else {
          uVar6 = 0x1008;
          uVar7 = str_op_1008_60e8((char *)CONCAT22(uVar9,pcVar5),uVar10);
          *(ushort *)(&local_f6 + (uint)uStack398) = uVar7;
          *(uint *)((int)&local_f6 + (uint)uStack398 * 0x4 + 0x2) = uVar10;
        }
        uVar9 = *(uint *)((int)&local_18a + uStack400 * 0x4 + 0x2);
        *(undefined2 *)(&local_1f0 + (uint)uStack398) = *(undefined2 *)(&local_18a + uStack400);
        *(uint *)((int)&local_1f0 + (uint)uStack398 * 0x4 + 0x2) = uVar9;
        uStack398 = uStack398 + 0x1;
      }
    }
    if (iStack94 == 0x0) {
      iVar12 = (int)(uStack92 >> 0x10) + (uint)CARRY2((uint)uStack92,(uint)uStack398);
      uStack92 = CONCAT22(iVar12,(uint)uStack92 + (uint)uStack398);
      if ((iVar12 != 0x0) || (0x13 < (uint)uStack92 + (uint)uStack398)) {
        iStack94 = 0x1;
      }
    }
    if ((uStack86 < iStack88 - 0x2U) && (local_1f0 != 0x0)) {
      iVar12 = string_1010_dcac(uVar6,uVar16,uVar17,uStack86,(ulong)param_2,param_3);
      uStack86 = iVar12 + 0x1;
      uStack86 = pass1_1010_db2e(uVar16,uVar17,uStack86,CONCAT22(param_5,&local_f6),CONCAT22(param_5,&local_1f0),
                                 (ulong)param_2,(int *)param_3,param_5,param_6);
    }
    while (lVar2 = uStack398 + -0x1, uStack398 != 0x0) {
      uStack398._0_2_ = (uint)lVar2;
      paStack412 = (astruct_18 *)(&local_f6)[(uint)uStack398];
      uStack398 = lVar2;
      fn_ptr_1000_17ce(paStack412,0x1000);
    }
    if (iStack94 != 0x0) {
      *(undefined2 *)(iVar11 + 0x16) = 0x1;
    }
    uStack398 = lVar2;
    pass1_1010_dc36(uVar16,uVar17,uStack86,(ulong)param_2,(uint *)param_3,(ushort)param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1010_cc56(ulong param_1,ulong param_2,astruct_104 *param_3,uint param_4,uchar *param_5,uchar param_6)

{
  long *plVar1;
  undefined4 uVar2;
  char *pcVar3;
  ushort uVar4;
  ushort uVar5;
  int iVar6;
  ulong uVar7;
  uint uVar8;
  ushort uVar9;
  ushort uVar10;
  undefined4 local_1a0;
  undefined4 uStack332;
  uint uStack328;
  uint uStack326;
  undefined4 uStack324;
  ushort uStack320;
  undefined4 local_13e;
  undefined4 local_aa;
  uint uStack22;
  int iStack18;
  uint uStack16;
  int iStack14;
  uint uStack12;
  ulong uStack10;
  ulong uStack6;
  
  uVar10 = (ushort)(param_1 >> 0x10);
  uVar9 = (ushort)param_1;
  uVar2 = *(undefined4 *)(uVar9 + 0x138);
  uVar7 = *(ulong *)((int)uVar2 + 0x24);
  uStack6 = uVar7;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar7,(uint)(uVar7 >> 0x10));
  uStack10 = uVar7 & 0xffff | (ulong)param_4 << 0x10;
  uStack324._2_2_ = param_4 | (uint)uVar7;
  if (uStack324._2_2_ != 0x0) {
    iStack14 = *(int *)param_3;
    iStack18 = 0x0;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_aa),(WNDCLASS16 *)0x0,0x94);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_13e),(WNDCLASS16 *)0x0,0x94);
    uStack12 = 0x0;
    uStack16 = 0x0;
    uStack22 = 0x0;
    for (uStack320 = 0x1; (int)uStack320 < 0x25; uStack320 = uStack320 + 0x1) {
      uStack324 = *(long *)((int)uStack10 + 0x26 + uStack320 * 0x4);
      if (uStack324 != 0x0) {
        pcVar3 = string_1020_c0d8(uStack320);
        uStack332 = uStack332 & 0xffff | ZEXT24(pcVar3) << 0x10;
        uVar8 = uStack324._2_2_ | (uint)pcVar3;
        uStack328 = uStack324._2_2_;
        if (uVar8 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_aa)[uStack22],s_Null_Ptr_1050_38f3);
        }
        else {
          uVar4 = str_op_1008_60e8((char *)CONCAT22(uStack324._2_2_,pcVar3),uVar8);
          *(ushort *)(&local_aa + uStack22) = uVar4;
          *(uint *)((int)&local_aa + uStack22 * 0x4 + 0x2) = uVar8;
        }
        *(undefined2 *)(&local_13e + uStack22) = (undefined2)uStack324;
        *(uint *)((int)&local_13e + uStack22 * 0x4 + 0x2) = uStack324._2_2_;
        uStack22 = uStack22 + 0x1;
      }
    }
    uStack16 = uStack22;
    if (0x13 < uStack22) {
      iStack18 = 0x1;
    }
    uStack12 = pass1_1010_db2e(uVar9,uVar10,0x1,CONCAT22(param_5,&local_aa),CONCAT22(param_5,&local_13e),param_2,
                               (int *)param_3,param_5,param_6);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_13e),(WNDCLASS16 *)0x0,0x54);
    for (uStack332._0_2_ = 0x1; (uint)uStack332 < 0x15; uStack332._0_2_ = (uint)uStack332 + 0x1) {
      uStack326 = (uint)uStack332;
      if (*(long *)((int)uStack10 + 0x14e + (uint)uStack332 * 0x4) != 0x0) {
        if (((0x0 < (int)(uint)uStack332) && (!SBORROW2((uint)uStack332,0x1))) && ((int)((uint)uStack332 - 0x1) < 0x6))
        {
          plVar1 = &local_13e + (uint)uStack332;
          *plVar1 = *plVar1 + 0x1;
        }
      }
    }
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_aa),(WNDCLASS16 *)0x0,0x54);
    uVar4 = 0x1000;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_1a0),(WNDCLASS16 *)0x0,0x54);
    for (uStack332 = 0x10000; (int)uStack332._2_2_ < 0x15;
        uStack332 = uStack332 & 0xffff | (ulong)(uStack332._2_2_ + 0x1) << 0x10) {
      if ((&local_13e)[uStack332._2_2_] != 0x0) {
        pcVar3 = string_op_1020_c222(uStack332._2_2_);
        uStack324 = CONCAT22(uStack324._2_2_,pcVar3);
        uVar8 = uStack324._2_2_ | (uint)pcVar3;
        if (uVar8 == 0x0) {
          uVar4 = 0x1000;
          unk_str_op_1000_3d3e((char *)(&local_aa)[(uint)uStack332],s_Null_Ptr_1050_38fc);
        }
        else {
          uVar4 = 0x1008;
          uVar5 = str_op_1008_60e8((char *)CONCAT22(uStack324._2_2_,pcVar3),uVar8);
          *(ushort *)(&local_aa + (uint)uStack332) = uVar5;
          *(uint *)((int)&local_aa + (uint)uStack332 * 0x4 + 0x2) = uVar8;
        }
        uStack324._2_2_ = *(uint *)((int)&local_13e + uStack332._2_2_ * 0x4 + 0x2);
        *(undefined2 *)(&local_1a0 + (uint)uStack332) = *(undefined2 *)(&local_13e + uStack332._2_2_);
        *(uint *)((int)&local_1a0 + (uint)uStack332 * 0x4 + 0x2) = uStack324._2_2_;
        uStack332 = uStack332 & 0xffff0000 | (ulong)((uint)uStack332 + 0x1);
      }
    }
    if (iStack18 == 0x0) {
      uStack16 = uStack16 + (uint)uStack332;
      if (0x13 < uStack16) {
        iStack18 = 0x1;
      }
    }
    if ((uStack12 < iStack14 - 0x2U) && (local_1a0 != 0x0)) {
      iVar6 = string_1010_dcac(uVar4,uVar9,uVar10,uStack12,param_2,param_3);
      uStack12 = iVar6 + 0x1;
      uStack12 = pass1_1010_db2e(uVar9,uVar10,uStack12,CONCAT22(param_5,&local_aa),CONCAT22(param_5,&local_1a0),param_2,
                                 (int *)param_3,param_5,param_6);
    }
    if (iStack18 != 0x0) {
      *(undefined2 *)((int)param_3 + 0x16) = 0x1;
    }
    pass1_1010_dc36(uVar9,uVar10,uStack12,param_2,(uint *)param_3,(ushort)param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1010_cf36(ulong param_1,ulong param_2,uint *param_3,uchar param_4,uchar *param_5)

{
  ulong uVar1;
  ushort *puVar2;
  char *pcVar3;
  uint uVar4;
  ushort uVar5;
  uint in_DX;
  uint uVar6;
  int unaff_SI;
  int iVar7;
  astruct_494 *iVar9;
  undefined2 uVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined4 uVar11;
  ulong uVar12;
  ushort uVar13;
  ushort uVar14;
  undefined2 uVar15;
  ushort uStack326;
  int iStack316;
  uint uStack314;
  int iStack312;
  uint local_136 [0x4a];
  undefined4 local_a2;
  int iStack14;
  undefined4 uStack12;
  ushort *puStack8;
  int iStack4;
  
  iStack4 = 0x0;
  do {
    uVar8 = (undefined2)(param_2 >> 0x10);
    iVar7 = (int)param_2;
    uVar9 = (undefined2)((ulong)param_3 >> 0x10);
    iVar9 = (astruct_494 *)param_3;
    puVar2 = iVar9->field_0x2;
    *(undefined2 *)(iStack4 * 0xa + (int)puVar2 + 0x4) = *(undefined2 *)(iStack4 * 0x2 + iVar7);
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x8);
  puStack8 = iVar9->field_0x2;
  iStack4 = 0x0;
  do {
    uVar1 = iVar9->field_0x6;
    pcVar3 = pass1_1010_b038((uchar *)param_1,(ushort)uVar1,(ushort)(uVar1 >> 0x10),*(uchar **)((int)puStack8 + 0x4),
                             unaff_SI);
    string_1040_a626(puStack8,(char *)CONCAT22(in_DX,pcVar3),in_DX);
    iStack4 = iStack4 + 0x1;
    puStack8 = (ushort *)((ulong)puStack8 & 0xffff0000 | (ulong)((int)puStack8 + 0xa));
  } while (iStack4 < 0x8);
  uVar13 = (ushort)param_1;
  uVar14 = (ushort)(param_1 >> 0x10);
  struct_1010_dd5e(uVar13,uVar14,iVar9->field_0x6);
  uStack12 = CONCAT22(in_DX,pcVar3);
  in_DX = in_DX | (uint)pcVar3;
  if (in_DX != 0x0) {
    iStack14 = 0x0;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_a2),(WNDCLASS16 *)0x0,0x94);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,local_136),(WNDCLASS16 *)0x0,0x94);
    uStack314 = 0x0;
    iStack312 = 0x0;
    iStack316 = 0x0;
    uVar1 = iVar9->field_0x6;
    uVar1 = *(ulong *)((int)uVar1 + 0x26);
    uVar12 = uVar1;
    for (uStack326 = 0x1; (int)uStack326 < 0x25; uStack326 = uStack326 + 0x1) {
      uVar15 = (undefined2)(uVar1 >> 0x10);
      if (*(long *)(uStack326 * 0x4 + (int)uStack12) == 0x0) {
        if (uVar1 != 0x0) {
          uVar12 = pass1_1020_bae6((ushort)uVar1,CONCAT22(uStack326,uVar15),(uint)uVar12,in_DX,(ushort)param_5);
          uVar6 = (uint)(uVar12 >> 0x10);
          uVar12 = uVar12 & 0xffff;
          in_DX = uVar6 | (uint)uVar12;
          if (in_DX != 0x0) {
            if (iStack14 == 0x0) {
              iStack14 = 0x1;
            }
            pcVar3 = string_1020_c0d8(uStack326);
            uVar4 = in_DX | (uint)pcVar3;
            if (uVar4 == 0x0) {
              unk_str_op_1000_3d3e((char *)(&local_a2)[iStack312],s_Null_Ptr_1050_390e);
            }
            else {
              uVar5 = str_op_1008_60e8((char *)CONCAT22(in_DX,pcVar3),uVar4);
              *(ushort *)(&local_a2 + iStack312) = uVar5;
              *(uint *)((int)&local_a2 + iStack312 * 0x4 + 0x2) = uVar4;
            }
            local_136[iStack312 * 0x2] = (uint)uVar12;
            local_136[iStack312 * 0x2 + 0x1] = uVar6;
            goto LAB_1010_d11d;
          }
        }
      }
      else {
        if (iStack14 == 0x0) {
          iStack14 = 0x1;
        }
        pcVar3 = string_1020_c0d8(uStack326);
        uVar6 = in_DX | (uint)pcVar3;
        if (uVar6 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_a2)[iStack312],s_Null_Ptr_1050_3905);
        }
        else {
          uVar5 = str_op_1008_60e8((char *)CONCAT22(in_DX,pcVar3),uVar6);
          *(ushort *)(&local_a2 + iStack312) = uVar5;
          *(uint *)((int)&local_a2 + iStack312 * 0x4 + 0x2) = uVar6;
        }
        uVar10 = (undefined2)((ulong)uStack12 >> 0x10);
        uVar4 = *(uint *)(uStack326 * 0x4 + (int)uStack12);
        uVar6 = *(uint *)(uStack326 * 0x4 + (int)uStack12 + 0x2);
        local_136[iStack312 * 0x2] = uVar4;
        local_136[iStack312 * 0x2 + 0x1] = uVar6;
        if (uVar1 == 0x0) {
          uVar4 = 0x0;
        }
        else {
          uVar11 = pass1_1020_bae6((ushort)uVar1,CONCAT22(uStack326,uVar15),uVar4,uVar6,(ushort)param_5);
          uVar6 = (uint)((ulong)uVar11 >> 0x10);
          uVar4 = (uint)uVar11;
        }
        uVar12 = (ulong)uVar4;
        if (uVar4 == 0x0) {
          iStack316 = iStack316 + 0x2;
          in_DX = uVar6;
          iStack312 = iStack312 + 0x1;
        }
        else {
LAB_1010_d11d:
          iStack312 = iStack312 + 0x1;
          *(undefined2 *)(uVar13 + uStack314 * 0x2 + 0xa4) = *(undefined2 *)(iVar7 + iStack316 * 0x2 + 0x10);
          *(undefined2 *)(uVar13 + (uStack314 + 0x1) * 0x2 + 0xa4) =
               *(undefined2 *)(iVar7 + (iStack316 + 0x1) * 0x2 + 0x10);
          iStack316 = iStack316 + 0x2;
          uStack314 = uStack314 + 0x2;
          uVar12 = (ulong)uStack314;
          in_DX = uVar6;
        }
      }
    }
    uVar6 = pass1_1010_db2e(uVar13,uVar14,0x8,CONCAT22(param_5,&local_a2),CONCAT22(param_5,local_136),param_2,
                            (int *)param_3,param_5,param_4);
    if (iStack14 != 0x0) {
      iVar9->field_0x16 = 0x1;
    }
    while (iStack312 != 0x0) {
      fn_ptr_1000_17ce((astruct_18 *)(&local_a2)[iStack312 + -0x1],0x1000);
      iStack312 = iStack312 + -0x1;
    }
    pass1_1010_dc36(uVar13,uVar14,uVar6,param_2,param_3,(ushort)param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1010_d24a(ulong param_1,ulong param_2,uint *param_3,uchar *param_4,uchar param_5)

{
  ulong uVar1;
  ushort *puVar2;
  char *pcVar3;
  uint *puVar4;
  ushort uVar5;
  uint in_DX;
  uint *puVar6;
  uint *puVar7;
  uint uVar8;
  int unaff_SI;
  astruct_495 *iVar9;
  undefined2 uVar9;
  long lVar10;
  ushort uVar11;
  ushort uVar12;
  ushort uStack320;
  long lStack318;
  uint *local_13a [0x4a];
  undefined4 local_a6;
  int iStack18;
  ulong uStack16;
  char *pcStack12;
  ushort *puStack8;
  int iStack4;
  
  iStack4 = 0x0;
  do {
    uVar9 = (undefined2)((ulong)param_3 >> 0x10);
    iVar9 = (astruct_495 *)param_3;
    puVar2 = iVar9->field_0x2;
    *(undefined2 *)(iStack4 * 0xa + (int)puVar2 + 0x4) = *(undefined2 *)(iStack4 * 0x2 + (int)param_2);
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0x8);
  puStack8 = iVar9->field_0x2;
  iStack4 = 0x0;
  do {
    uVar1 = iVar9->field_0x6;
    pcVar3 = pass1_1010_b038((uchar *)param_1,(ushort)uVar1,(ushort)(uVar1 >> 0x10),*(uchar **)((int)puStack8 + 0x4),
                             unaff_SI);
    string_1040_a626(puStack8,(char *)CONCAT22(in_DX,pcVar3),in_DX);
    iStack4 = iStack4 + 0x1;
    puStack8 = (ushort *)((ulong)puStack8 & 0xffff0000 | (ulong)((int)puStack8 + 0xa));
  } while (iStack4 < 0x8);
  uVar11 = (ushort)param_1;
  uVar12 = (ushort)(param_1 >> 0x10);
  struct_1010_dd5e(uVar11,uVar12,iVar9->field_0x6);
  puVar6 = (uint *)(in_DX | (uint)pcVar3);
  if (puVar6 != (uint *)0x0) {
    pcStack12 = pcVar3;
    pass1_1010_e8f6(uVar11,uVar12,iVar9->field_0x6,(ushort)param_4);
    uStack16 = CONCAT22(puVar6,pcVar3);
    iStack18 = 0x0;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_4,&local_a6),(WNDCLASS16 *)0x0,0x94);
    puVar4 = pass1_1000_4906((astruct_20 *)CONCAT22(param_4,local_13a),(WNDCLASS16 *)0x0,0x94);
    lStack318 = 0x0;
    for (uStack320 = 0x1; (int)uStack320 < 0x25; uStack320 = uStack320 + 0x1) {
      lVar10 = pass1_1030_7c28(uStack16,uStack320,(uint)puVar4,(uint)puVar6,(ushort)param_4);
      puVar7 = (uint *)((ulong)lVar10 >> 0x10);
      puVar4 = (uint *)lVar10;
      puVar6 = (uint *)((uint)puVar7 | (uint)puVar4);
      if (lVar10 != 0x0) {
        if (iStack18 == 0x0) {
          iStack18 = 0x1;
        }
        pcVar3 = string_1020_c0d8(uStack320);
        uVar8 = (uint)puVar6 | (uint)pcVar3;
        if (uVar8 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_a6)[(int)lStack318],s_Null_Ptr_1050_3917);
        }
        else {
          uVar5 = str_op_1008_60e8((char *)CONCAT22(puVar6,pcVar3),uVar8);
          *(ushort *)(&local_a6 + (int)lStack318) = uVar5;
          *(uint *)((int)&local_a6 + (int)lStack318 * 0x4 + 0x2) = uVar8;
        }
        local_13a[(int)lStack318 * 0x2] = puVar4;
        local_13a[(int)lStack318 * 0x2 + 0x1] = puVar7;
        lStack318 = lStack318 + 0x1;
        puVar6 = puVar7;
      }
    }
    uVar8 = pass1_1010_db2e(uVar11,uVar12,0x8,CONCAT22(param_4,&local_a6),CONCAT22(param_4,local_13a),param_2,
                            (int *)param_3,param_4,param_5);
    if (iStack18 != 0x0) {
      iVar9->field_0x16 = 0x1;
    }
    while (lStack318 != 0x0) {
      lStack318._0_2_ = (int)(lStack318 + -0x1);
      fn_ptr_1000_17ce((astruct_18 *)(&local_a6)[(int)lStack318],0x1000);
      lStack318 = lStack318 + -0x1;
    }
    pass1_1010_dc36(uVar11,uVar12,uVar8,param_2,param_3,(ushort)param_4);
  }
  return;
}

