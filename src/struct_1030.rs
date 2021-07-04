
fn struct_1030_11aa(param_1: *mut u16,param_2: i32,param_3: i32,param_4: u16)
{
  astruct_156 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_156 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x6 = 0x0;
  iVar1->field_0xa = 0x0;
  iVar1->field_0xe = param_3;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x16 = param_2;
  iVar1->field_0x1a = 0x1;
  *param_1 = (s_462_bmp_1050_1620 + 0x4);
  iVar1->field_0x2 = 0x1030;
  if (iVar1->field_0xe == 0x0) {
    iVar1->field_0xe = 0x5;
  }
  if (iVar1->field_0x16 == 0x0) {
    iVar1->field_0x16 = 0x5;
  }
  struct_1030_1550(param_1,param_4);
  *iVar1->field_0x6 = 0x0;
  return;
}



fn struct_1030_1550(param_1: u32,param_2: u16)
{
  let puVar1: *mut u16;
  let uVar2: u16;
  let uVar3: u16;
  astruct_157 *iVar5;
  let uVar4: u16;
  let lVar5: i32;
  let lStack10: i32;
  let uStack6: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_157 *)param_1;
  if (*(long *)&iVar5->field_0x12 == 0x0) {
    uVar3 = iVar5->field_0xe;
    PTR_LOOP_1050_5f2e = iVar5->field_0x10;
  }
  else {
    uVar2 = &iVar5->field_0x12;
    puVar1 = &iVar5->field_0x16;
    uVar3 = uVar2 + *puVar1;
    PTR_LOOP_1050_5f2e =
         
         (iVar5->field_0x14 + iVar5->field_0x18 + CARRY2(uVar2,*puVar1));
  }
  uStack6 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
  if (iVar5->field_0x6 == 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar3 = fn_ptr_op_1000_1708(uVar3 << 0x2,0x0,0x1,PTR_LOOP_1050_5f2c,
                                PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
    lVar5 = iVar5->field_0x6;
    lVar5 = pass1_1000_0ed4(0x1000,param_2,0x1,uVar3 * 0x4,
                            (PTR_LOOP_1050_5f2e * 0x2 + CARRY2(uVar3,uVar3)) *
                            0x2 + CARRY2(uVar3 * 0x2,uVar3 * 0x2),(u16 *)lVar5,
                            (lVar5 >> 0x10));
    PTR_LOOP_1050_5f2e = (lVar5 >> 0x10);
    uVar3 = lVar5;
  }
  lStack10 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
  if ((PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
    &iVar5->field_0x12 = uStack6;
    iVar5->field_0x6 = lStack10;
  }
  return;
}


fn struct_1030_1628(param_1: *mut u16)
{
  astruct_181 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_181 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = 0x0;
  *param_1 = 0x17ba;
  iVar1->field_0x2 = 0x1030;
  return;
}


fn struct_1030_17ce(param_1: *mut u16,param_2: u32,param_3: u32) -> u16

{
  astruct_75 *paVar1;
  let uVar2: u32;
  let uVar3: u16;
  astruct_343 *iVar3;
  
  iVar3 = (astruct_343 *)param_1;
  uVar3 = (param_1 >> 0x10);
  paVar1 = (astruct_75 *)struct_1030_1628(param_1);
  &iVar3->field_0xc = 0x0;
  *param_1 = 0x1a16;
  iVar3->field_0x2 = 0x1030;
  if ((param_3 != 0x0) || (param_2 != 0x0)) {
    mem_op_1000_179c(0x18,(uchar *)(paVar1 >> 0x10),0x1000);
    if (paVar1 == (astruct_75 *)0x0) {
      uVar2 = 0x0;
    }
    else {
      uVar2 = struct_op_1030_1cd8(paVar1,param_2,param_3);
    }
    iVar3->field_0xc = uVar2;
    iVar3->field_0xe = (uVar2 >> 0x10);
  }
  return param_1;
}


fn struct_op_1030_1cd8(astruct_75 *param_1,param_2: u32,param_3: u32)
{
  astruct_75 *struct_var1;
  astruct_75 *struct_var2;
  
  struct_var2 = (astruct_75 *)(param_1 >> 0x10);
  struct_var1 = (astruct_75 *)param_1;
  param_1->field_0x0 = 0x389a;
  struct_var1->field_0x2 = 0x1008;
  struct_var1->field_0x4 = 0x0;
  struct_var1->field_0x8 = 0x0;
  struct_var1->field_0xc = param_3;
  struct_var1->field_0x10 = 0x0;
  struct_var1->field_0x14 = param_2;
  param_1->field_0x0 = 0x2044;
  struct_var1->field_0x2 = 0x1030;
  return;
}



fn struct_1030_2112(param_1: *mut u16,param_2: u32,param_3: u16,uchar *param_4)
{
  astruct_366 *iVar1;
  astruct_367 *iVar2;
  let uVar1: u16;
  let iStack4: i16;
  
  pass1_1030_183c(param_1,0x1,0x1,0x8000000,param_2,param_3,param_4);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_366 *)param_1;
  *param_1 = 0x293c;
  iVar1->field_0x2 = 0x1030;
  iStack4 = 0x0;
  do {
    iVar2 = (astruct_367 *)(&iVar1->field_0x0 + iStack4 * 0x2);
    iVar2->field_0x10 = 0xffff;
    iVar2->field_0x1a6 = 0x19;
    iStack4 += 0x1;
  } while (iStack4 < 0x83);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar1->field_0x116),
                  (WNDCLASS16 *)0x0,0x86);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar1->field_0x19c),
                  (WNDCLASS16 *)0x0,0xa);
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | ZEXT24(iVar1 + 0x1)),
                  (WNDCLASS16 *)0x0,0x106);
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  iVar1->field_0x20 = 0x0;
  iVar1->field_0x44 = 0x0;
  iVar1->field_0x50 = 0x0;
  iVar1->field_0x6a = 0x0;
  iVar1->field_0x84 = 0x0;
  iVar1->field_0xc8 = 0x0;
  iVar1->field_0xe4 = 0x0;
  iVar1->field_0xf0 = 0x0;
  iVar1->field_0xf4 = 0x0;
  iVar1->field_0xf6 = 0x0;
  iVar1->field_0x102 = 0x0;
  iVar1->field_0xfe = 0x0;
  iVar1->field_0x1a6 = 0x0;
  iVar1->field_0x1aa = 0x0;
  iVar1->field_0x1ac = 0x0;
  iVar1->field_0x1b6 = 0x0;
  iVar1->field_0x1da = 0x0;
  iVar1->field_0x1e6 = 0x0;
  iVar1->field_0x200 = 0x0;
  iVar1->field_0x21a = 0x0;
  iVar1->field_0x25e = 0x0;
  iVar1->field_0x27a = 0x0;
  iVar1->field_0x286 = 0x0;
  iVar1->field_0x28a = 0x0;
  iVar1->field_0x28c = 0x0;
  iVar1->field_0x298 = 0x0;
  iVar1->field_0x294 = 0x0;
  return;
}


fn struct_1030_299a(param_1: *mut u16,param_2: u32,param_3: u16,uchar *param_4)
{
  astruct_352 *iVar1;
  let uVar1: u16;
  
  pass1_1030_183c(param_1,0x5,0xf,0x2000000,param_2,param_3,param_4);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_352 *)param_1;
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  iVar1->field_0x18 = 0x2710;
  iVar1->field_0x1a = 0x0;
  *param_1 = 0x3130;
  iVar1->field_0x2 = 0x1030;
  return;
}


fn struct_1030_44be(param_1: *mut u32,param_2: u16)
{
  astruct_138 *iVar1;
  let unaff_DI: i16;
  let uVar1: u16;
  let unaff_SS: u16;
  let puVar2: *mut u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_138 *)param_1;
  *param_1 = 0x0;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x152 = 0x0;
  iVar1->field_0x154 = 0x0;
  iVar1->field_0x156 = 0x0;
  iVar1->field_0x158 = 0x0;
  iVar1->field_0x15a = 0x0;
  iVar1->field_0x15c = 0x0;
  iVar1->field_0x160 = 0x0;
  iVar1->field_0x164 = 0x0;
  iVar1->field_0x568 = 0x0;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,unaff_SS,(uchar *)param_2,unaff_DI);
  iVar1->field_0x568 = (puVar2 + 0x64);
  return;
}


fn struct_1030_4574(param_1: u32) -> u32

{
  astruct_159 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_159 *)param_1;
  iVar1->field_0xc = DAT_1050_518c;
  iVar1->field_0xe = 0x518e;
  iVar1->field_0x10 = &USHORT_1050_1050;
  return param_1 & 0xffff0000 | ZEXT24(&iVar1->field_0xc);
}


fn struct_1030_565a(param_1: *mut u16,param_2: u32,param_3: u16,uchar *param_4) -> *mut u16

{
  astruct_353 *iVar1;
  let uVar1: u16;
  
  pass1_1030_183c(param_1,0x64,0x1f4,0x3000000,param_2,param_3,param_4);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_353 *)param_1;
  iVar1->field_0x10 = 0x0;
  pass1_1008_3e38((u16 *)
                  (param_1 & 0xffff0000 | &iVar1->field_0x14));
  iVar1->field_0x1a = 0x0;
  iVar1->field_0x1c = 0x0;
  *param_1 = s_procLo_1050_5bd0;
  iVar1->field_0x2 = 0x1030;
  return param_1;
}


fn struct_op_1030_73a8(param_1: u32) -> u32

{
  let uVar1: u32;
  let in_AX: u16;
  let in_DX: u16;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(long *)(iVar2 + 0x16) == 0x0) {
    return 0x0;
  }
  if (*(long *)(iVar2 + 0x1a) == 0x0) {
    uVar1 = (iVar2 + 0x16);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
    (iVar2 + 0x1a) = in_AX;
    (iVar2 + 0x1c) = in_DX;
  }
  return CONCAT22((iVar2 + 0x1c),(iVar2 + 0x1a));
}


fn struct_1030_8544(param_1: *mut u16,param_2: *mut u16)
{
  astruct_356 *iVar1;
  astruct_355 *iVar2;
  let uVar1: u16;
  let uVar2: u16;
  
  *param_1 = *param_2;
  uVar1 = (param_2 >> 0x10);
  iVar1 = (astruct_356 *)param_2;
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_355 *)param_1;
  iVar2->field_0x4 = iVar1->field_0x4;
  pass1_1008_3f62((u16 *)(param_1 & 0xffff0000 | &iVar2->field_0x8)
                  ,(u16 *)
                   (param_2 & 0xffff0000 | &iVar1->field_0x8));
  iVar2->field_0xe = iVar1->field_0xe;
  iVar2->field_0x12 = iVar1->field_0x12;
  iVar2->field_0x16 = iVar1->field_0x16;
  iVar2->field_0x1a = iVar1->field_0x1a;
  iVar2->field_0x1e = 0x0;
  return;
}



fn struct_1030_c06e(param_1: *mut u16)
{
  astruct_188 *iVar1;
  let uVar1: u16;
  
  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_188 *)param_1;
  iVar1->field_0x20 = 0x0;
  iVar1->field_0x24 = 0x0;
  *param_1 = 0xc68e;
  iVar1->field_0x2 = 0x1030;
  return;
}


fn struct_1030_c6f6(param_1: *mut u16) -> u16

{
  let uVar1: u16;
  
  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = 0x0;
  *param_1 = 0xc940;
  (param_1 + 0x2) = 0x1030;
  return param_1;
}


fn struct_1030_c9a8(param_1: *mut u16) -> u16

{
  let iVar1: i16;
  let uVar2: u16;
  
  struct_1028_b354(param_1);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x98) = 0x1;
  *param_1 = 0xd88e;
  (iVar1 + 0x2) = 0x1030;
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (iVar1 + 0x20)),
                  (WNDCLASS16 *)0x0,0x78);
  return param_1;
}



fn struct_1030_d8f6(param_1: *mut u16) -> u16

{
  astruct_184 *iVar1;
  let uVar1: u16;
  
  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_184 *)param_1;
  *param_1 = 0xdc2e;
  iVar1->field_0x2 = 0x1030;
  if (iVar1->field_0xc == 0x4c) {
    iVar1->field_0xe = 0x43;
  }
  else {
    if (iVar1->field_0xc == 0x4d) {
      iVar1->field_0xe = 0x44;
    }
    else {
      iVar1->field_0xe = 0x45;
    }
  }
  return param_1;
}


fn struct_1030_dc96(param_1: *mut u16) -> u16

{
  let uVar1: u16;
  
  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = 0x0;
  *param_1 = 0xe036;
  (param_1 + 0x2) = 0x1030;
  return param_1;
}



fn struct_1030_e4fa(astruct_100 *param_1,param_2: u32,param_3: u16,param_4: u8)
{
  astruct_289 *iVar1;
  let puVar1: *mut u8
  
  struct_op_1028_d1dc(param_3,param_4,param_1,0x3e80);
  puVar1 = (uchar *)(param_1 >> 0x10);
  iVar1 = (astruct_289 *)param_1;
  iVar1->field_0x108 = param_2;
  param_1->field_0x0 = 0xe62e;
  iVar1->field_0x2 = 0x1030;
  sys_1000_3f9c(&iVar1->field_0x8,puVar1,s_SCKillBldg__0x_08lx_1050_597c,
                &USHORT_1050_1050,iVar1->field_0x108,&stack0xfffe,puVar1,
                0x1000,param_3,param_4);
  return;
}

