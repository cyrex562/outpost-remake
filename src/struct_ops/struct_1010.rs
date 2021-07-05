

fn struct_1010_02e0(astruct_79 *param_1,astruct_79 *param_2,param_3: u16)
{
  let uVar1: u16;
  let puVar2: *mut u8
  let extraout_DX: u16;
  astruct_79 *paVar3;
  
  paVar3 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  puVar2 = (paVar3 >> 0x10);
  uVar1 = 0x0;
  (param_1 + 0x1) = 0x0;
  &param_1[0x1].field_0x4 = 0x0;
  (&param_1[0x1].field_0x4 + 0x2) = 0x0;
  &param_1[0x2].field_0x4 = 0x0;
  CONCAT22(param_2,param_1) = 0xe98;
  param_1->field_0x2 = 0x1010;
  mem_op_1000_179c(0xc,puVar2,0x1000);
  if ((puVar2 | uVar1) == 0x0) {
    (param_1 + 0x1) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar2,uVar1));
    ((astruct_79 *)(param_1 + 0x1))->field_0x0 = uVar1;
    param_1[0x1].field_0x2 = extraout_DX;
  }
  return;
}


fn struct_1010_0f9c(param_1: *mut u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let puVar3: *mut u8;
  let puVar4: *mut u8;
  let uVar5: u32;
  let extraout_DX: *mut u8
  let puVar6: *mut u8
  let puVar7: *mut u8
  let extraout_DX_00: *mut u8
  let extraout_DX_01: u16;
  let extraout_DX_02: u16;
  let extraout_DX_03: u16;
  let extraout_DX_04: *mut u8
  astruct_232 *iVar8;
  astruct_231 *iVar9;
  astruct_230 *iVar10;
  astruct_233 *iVar11;
  let uVar8: u16;
  let uVar9: u16;
  let puVar10: u32;
  let uVar11: u32;
  let puVar12: u32;
  let uVar13: u8;
  let uStack36: u32;
  let iStack32: i16;
  let uStack30: u16;
  let puStack28: *mut u16;
  let uStack24: u32;
  let local_14: [u8;12];
  
  uVar8 = (param_1 >> 0x10);
  iVar8 = (astruct_232 *)param_1;
  ppcVar1 = (code **)(*param_1 + 0x38);
  (**ppcVar1)();
  iVar8->field_0x68 = param_2;
  if ((*(long *)&iVar8->field_0x60 != 0x0) && (iVar8->field_0x68 == 0x1)) {
    return;
  }
  if (iVar8->field_0x68 == 0x0) {
    return;
  }
  puVar7 = extraout_DX;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_3,local_14),0x1,0x0,0x700);
  uVar2 = iVar8->field_0x68 * 0x18;
  mem_op_1000_179c(uVar2,puVar7,0x1000);
  iVar8->field_0x60 = uVar2;
  iVar8->field_0x62 = puVar7;
  puStack28 = CONCAT22(puVar7,iVar8->field_0x60);
  uStack30 = iVar8->field_0x68;
  do {
    do {
      puVar6 = puVar7;
      puVar3 = local_14;
      pass1_1028_e4ec(CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,puVar3)));
      uStack24 = CONCAT22(puVar6,puVar3);
      puVar7 = (puVar6 | puVar3);
      if (puVar7 == 0x0) goto LAB_1010_10ca;
      iVar9 = (astruct_231 *)*param_1;
      ppcVar1 = (code **)&iVar9->field_0x40;
      puVar4 = puVar3;
      (**ppcVar1)();
      puVar7 = extraout_DX_00;
    } while (puVar4 == 0x0);
    uVar13 = SUB21(puVar6,0x0);
    pass1_1028_b58e(CONCAT13((char)(puVar6 >> 0x8),CONCAT12(uVar13,puVar3)));
    uStack36 = CONCAT22(extraout_DX_01,puVar4);
    ppcVar1 = (code **)&iVar9->field_0x2c;
    puVar12 = param_1;
    (**ppcVar1)();
    uVar9 = (puStack28 >> 0x10);
    iVar10 = (astruct_230 *)puStack28;
    *puStack28 = puVar4;
    iVar10->field_0x2 = extraout_DX_02;
    ppcVar1 = (code **)&iVar9->field_0x30;
    puVar10 = param_1;
    uVar11 = uStack24;
    (**ppcVar1)();
    iVar10->field_0x8 = puVar4;
    iVar10->field_0xa = extraout_DX_03;
    iVar10->field_0xc = uStack36;
    ppcVar1 = (code **)&iVar9->field_0x3c;
    uVar5 = uStack36;
    (**ppcVar1)(&USHORT_1050_1028,param_1,uStack24,puVar10,uVar11,puVar12,puVar3,
                uVar13);
    iVar10->field_0x10 = uVar5;
    iVar10->field_0x12 = extraout_DX_04;
    iVar10->field_0x14 = uStack36;
    puStack28 = (puStack28 & 0xffff0000 | ZEXT24(iVar10 + 0x1));
    uStack30 -= 0x1;
    puVar7 = extraout_DX_04;
  } while (uStack30 != 0x0);
LAB_1010_10ca:
  uVar2 = iVar8->field_0x68 << 0x2;
  mem_op_1000_179c(uVar2,puVar7,0x1000);
  iVar8->field_0x64 = uVar2;
  iVar8->field_0x66 = puVar7;
  iStack32 = 0x0;
  uStack30 = 0x0;
  while( true ) {
    if ((iVar8->field_0x68 * 0x3) <= uStack30) break;
    puVar7 = iVar8->field_0x62;
    uVar5 = &iVar8->field_0x64;
    uVar9 = (uVar5 >> 0x10);
    iVar11 = (astruct_233 *)uVar5;
    (iVar11 + iStack32 * 0x4) = iVar8->field_0x60 + uStack30 * 0x8;
    *(uchar **)(iVar11 + iStack32 * 0x4 + 0x2) = puVar7;
    uStack30 += 0x3;
    iStack32 += 0x1;
  }
  return;
}


astruct_79 *  struct_op_1010_1d48(astruct_79 *param_1,param_2: u16)

{
  astruct_79 *iVar1;
  astruct_79 *uVar1;
  
  uVar1 = (astruct_79 *)(param_1 >> 0x10);
  iVar1 = (astruct_79 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = param_2;
  param_1->field_0x0 = 0x2014;
  iVar1->field_0x2 = 0x1010;
  return param_1;
}


fn struct_1010_2cd2(astruct_79 *param_1,astruct_79 *param_2,param_3: u16,param_4: u16)
{
  let unaff_DI: i16;
  astruct_79 *paVar1;
  let puVar2: *mut u16;
  let piVar3: *mut i16;
  let piVar4: *mut i16;
  let uVar5: u16;
  let local_6: i16;
  let local_4: i16;
  
  paVar1 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  &param_1[0x1].field_0x8 = 0x0;
  param_1[0x2].field_0x2 = 0x0;
  &param_1[0x2].field_0x4 = 0x0;
  &param_1[0x3].field_0x4 = 0x0;
  (&param_1[0x3].field_0x4 + 0x2) = 0x0;
  param_1[0x3].field_0x8 = 0x0;
  ((astruct_79 *)(param_1 + 0x4))->field_0x0 = 0x0;
  &param_1[0x8].field_0x2 = 0x0;
  (&param_1[0x8].field_0x4 + 0x2) = 0x0;
  ((astruct_79 *)(param_1 + 0x9))->field_0x0 = 0x0;
  &param_1[0x9].field_0x4 = 0x0;
  param_1[0x9].field_0x2 = 0x0;
  CONCAT22(param_2,param_1) = 0x36da;
  param_1->field_0x2 = 0x1010;
  piVar4 = &local_4;
  piVar3 = &local_6;
  uVar5 = param_4;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,
                           (paVar1 >> 0x10),unaff_DI);
  pass1_1008_3e94((u16 *)(puVar2 & 0xffff0000 | (puVar2 + 0xe)),
                  CONCAT22(param_4,piVar3),CONCAT22(uVar5,piVar4));
  param_1[0x1].field_0x4 = 0x19001db;
  ((astruct_79 *)(param_1 + 0x1))->field_0x0 =
       0x140 - (&param_1[0x1].field_0x4 / 0x2 - local_4);
  param_1[0x1].field_0x2 =
       0xf0 - ((&param_1[0x1].field_0x4 + 0x2) / 0x2 - local_6);
  (&param_1[0x2].field_0x4 + 0x2) = 0xa006e;
  (param_1 + 0x3) = 0xa012c;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1[0x4].field_0x2),
                  (WNDCLASS16 *)0x0,0x28);
  return;
}


fn struct_1010_383a(param_1: *mut u16)
{
  astruct_223 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_223 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xc = 0x0;
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x14 = 0x0;
  *param_1 = 0x3b5e;
  iVar1->field_0x2 = 0x1010;
  return;
}


fn struct_1010_38f8(param_1: u32,param_2: i16,param_3: u16,uchar *param_4) -> u16

{
  let uVar1: u16;
  astruct_240 *iVar2;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  if (param_2 != 0x0) {
    uVar1 = param_2 << 0x2;
    mem_op_1000_179c(uVar1,param_4,0x1000);
    uVar2 = (param_1 >> 0x10);
    iVar2 = (astruct_240 *)param_1;
    iVar2->field_0x8 = uVar1;
    iVar2->field_0xa = param_4;
    return CONCAT22(param_4,iVar2->field_0x8);
  }
  mem_op_1000_179c(0x1a,param_4,0x1000);
  if ((param_4 | param_3) != 0x0) {
    puVar3 = pass1_1010_37d4((u16 *)CONCAT22(param_4,param_3));
    return puVar3;
  }
  return 0x0;
}


fn struct_1010_3b7a(astruct_648 *param_1,param_2: u16,param_3: u16)
{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x389a;
  param_1->field_0xc = 0x1008;
  param_1->field_0xa = 0x3aa8;
  param_1->field_0xc = 0x1008;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x14 = 0x0;
  param_1->field_0x16 = 0x0;
  CONCAT22(param_2,param_1) = 0x3d6a;
  param_1->field_0x2 = 0x1010;
  param_1->field_0xa = 0x3d7a;
  param_1->field_0xc = 0x1010;
  return;
}


fn struct_1010_3c52(param_1: u32,param_2: u16,param_3: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_274 *iVar4;
  let uVar4: u16;
  astruct_43 *paVar5;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_274 *)param_1;
  iVar4->field_0x14 = param_2;
  puVar1 = iVar4->field_0xe;
  uVar2 = iVar4->field_0x10;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  paVar5 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,iVar4->field_0x14,param_3);
  iVar4->field_0xe = paVar5;
  iVar4->field_0x10 = (paVar5 >> 0x10);
  return;
}


void 
struct_1010_4d5c(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: u16
                ,param_6: i16,uchar *param_7)

{
  let uVar1: u32;
  let uVar2: u16;
  astruct_245 *iVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar3 = (astruct_245 *)param_1;
  if (*(long *)&iVar3->field_0x1a == 0x0) {
    uVar2 = iVar3->field_0x30 << 0x3;
    mem_op_1000_179c(uVar2,param_7,0x1000);
    &iVar3->field_0x1a = uVar2;
    iVar3->field_0x1c = param_7;
  }
  uVar1 = &iVar3->field_0x1a;
  iVar4 = param_6 * 0x8;
  (uVar1 + iVar4) = param_5;
  uVar1 = &iVar3->field_0x1a;
  (uVar1 + iVar4 + 0x2) = param_4;
  uVar1 = &iVar3->field_0x1a;
  (uVar1 + iVar4 + 0x4) = param_3;
  uVar1 = &iVar3->field_0x1a;
  (uVar1 + iVar4 + 0x6) = param_2;
  return;
}



fn struct_1010_50b2(astruct_646 *param_1,param_2: u16,param_3: u16)
{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xc = 0x0;
  param_1->field_0x10 = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  CONCAT22(param_2,param_1) = 0x53f4;
  param_1->field_0x2 = 0x1010;
  return;
}


fn struct_1010_5f1e(astruct_73 *param_1,char *param_2,param_3: u16)
{
  let uVar1: u16;
  astruct_73 *iVar3;
  astruct_73 *uVar3;
  
  uVar3 = (astruct_73 *)(param_1 >> 0x10);
  iVar3 = (astruct_73 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x16,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x16 = uVar1;
  iVar3->field_0x18 = param_3;
  return;
}


fn struct_1010_6326(astruct_630 *param_1,param_2: u16,param_3: u16)
{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  param_1->field_0x1a = 0x0;
  param_1->field_0x1e = 0x0;
  param_1->field_0x22 = 0x0;
  CONCAT22(param_2,param_1) = 0x66f0;
  param_1->field_0x2 = 0x1010;
  return;
}


fn struct_1010_9172(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_249 *iVar4;
  let uVar4: u16;
  astruct_75 *paVar5;
  let uVar6: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_249 *)param_1;
  puVar1 = iVar4->field_0x4;
  uVar2 = iVar4->field_0x6;
  paVar5 = (astruct_75 *)CONCAT22(uVar2,puVar1);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    paVar5 = (astruct_75 *)(**ppcVar3)();
  }
  mem_op_1000_179c(0x18,(paVar5 >> 0x10),0x1000);
  if (paVar5 == (astruct_75 *)0x0) {
    uVar6 = 0x0;
  }
  else {
    uVar6 = struct_op_1030_1cd8(paVar5,0x5,0x5);
  }
  iVar4->field_0x4 = uVar6;
  iVar4->field_0x6 = (uVar6 >> 0x10);
  return;
}


fn struct_1010_95aa(astruct_629 *param_1,param_2: u16,param_3: u16)
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
  CONCAT22(param_2,param_1) = 0xa1c8;
  param_1->field_0x2 = 0x1010;
  return;
}


fn struct_1010_a1d8(astruct_627 *param_1,param_2: u16,param_3: u16,param_4: u16)
{
  let iVar1: i16;
  code **ppcVar2;
  let unaff_DI: i16;
  astruct_79 *paVar3;
  let puVar4: *mut u16;
  let uStack4: u16;
  
  paVar3 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x389a;
  param_1->field_0xc = 0x1008;
  param_1->field_0xa = 0x3aa8;
  param_1->field_0xc = 0x1008;
  param_1->field_0x138 = 0x0;
  CONCAT22(param_2,param_1) = 0xe9cc;
  param_1->field_0x2 = 0x1010;
  param_1->field_0xa = 0xe9dc;
  param_1->field_0xc = 0x1010;
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,
                           (paVar3 >> 0x10),unaff_DI);
  &param_1->field_0x138 = puVar4;
  (&param_1->field_0x138 + 0x2) = (puVar4 >> 0x10);
  ppcVar2 = (code **)(*param_1->field_0x138 + 0x4);
  (**ppcVar2)();
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0xa4),(WNDCLASS16 *)0x0,
                  0x94);
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0xe),(WNDCLASS16 *)0x0,
                  0x96);
  uStack4 = 0x0;
  do {
    iVar1 = &param_1->field_0x0 + uStack4 * 0x6;
    *(code **)(iVar1 + 0xe) = pass1_1010_c7e2;
    (iVar1 + 0x12) = 0x0;
    uStack4 += 0x1;
  } while (uStack4 < 0x19);
  param_1->field_0x4a = pass1_1010_c864;
  param_1->field_0x4e = 0x0;
  param_1->field_0x50 = pass1_1010_cc56;
  param_1->field_0x54 = 0x0;
  param_1->field_0x56 = pass1_1010_cf36;
  param_1->field_0x5a = 0x0;
  param_1->field_0x2c = pass1_1010_d24a;
  param_1->field_0x30 = 0x0;
  param_1->field_0x6e = pass1_1010_d448;
  param_1->field_0x72 = 0x0;
  param_1->field_0x74 = pass1_1010_d5ae;
  param_1->field_0x78 = 0x0;
  param_1->field_0x98 = pass1_1010_d710;
  param_1->field_0x9c = 0x0;
  return;
}


fn struct_1010_dd5e(param_1: u16,param_2: u16,param_3: u32)
{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u32;
  long *plStack16;
  
  if (param_3 != 0x0) {
    uVar4 = struct_op_1030_73a8(param_3);
    uVar3 = (uVar4 >> 0x10);
    iVar2 = uVar4;
    plStack16 = (long *)(uVar4 & 0xffff0000 | (iVar2 + 0x14U));
    if ((uVar3 | iVar2 + 0x14U) != 0x0) {
      iVar1 = (iVar2 + 0x12);
      iVar2 = (iVar2 + 0x18);
      if (((((iVar1 == 0x4) ||
            ((((iVar1 == 0x6 && (iVar2 == 0x4)) || (iVar1 == 0x5)) ||
             ((iVar1 == 0x6 && (iVar2 == 0x5)))))) || (iVar1 == 0x8)) ||
          ((iVar1 == 0x6 && (iVar2 == 0x8)))) && (*plStack16 != 0x0)) {
        return;
      }
    }
  }
  return;
}


fn struct_1010_e9e4(astruct_261 *param_1,param_2: u16,param_3: u16)
{
  let puVar1: *mut u16;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u32;
  let puVar7: *mut u8
  let iVar8: i16;
  astruct_79 *paVar9;
  let puVar10: *mut u16;
  let iStack4: i16;
  
  paVar9 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  puVar7 = (paVar9 >> 0x10);
  param_1->field_0xa = 0x389a;
  param_1->field_0xc = 0x1008;
  param_1->field_0xa = 0x3aa8;
  param_1->field_0xc = 0x1008;
  uVar5 = 0x0;
  &param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  param_1->field_0x1a = 0x0;
  param_1->field_0x1e = 0x0;
  param_1->field_0x20 = 0x0;
  param_1->field_0x24 = 0x0;
  param_1->field_0x28 = 0x0;
  param_1->field_0x2c = 0x0;
  param_1->field_0x30 = 0x0;
  param_1->field_0x32 = 0x0;
  CONCAT22(param_2,param_1) = 0x558;
  param_1->field_0x2 = 0x1018;
  param_1->field_0xa = 0x568;
  param_1->field_0xc = 0x1018;
  mem_op_1000_179c(0x4,puVar7,0x1000);
  if ((puVar7 | uVar5) == 0x0) {
    &param_1->field_0xe = 0x0;
  }
  else {
    puVar10 = pass1_1018_dcf6((u16 *)CONCAT22(puVar7,uVar5));
    param_1->field_0xe = puVar10;
    param_1->field_0x10 = (puVar10 >> 0x10);
  }
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0x34),(WNDCLASS16 *)0x0,
                  0x24);
  param_1->field_0x38 = 0xfa;
  param_1->field_0x3c = 0x15e;
  uVar6 = 0x1c2;
  param_1->field_0x40 = 0x1c2;
  param_1->field_0x44 = 0x1c2;
  param_1->field_0x46 = 0x2260000;
  param_1->field_0x4a = 0x28a0000;
  param_1->field_0x4e = 0x730000;
  param_1->field_0x52 = 0x960000;
  param_1->field_0x56 = 0x0;
  for (iStack4 = 0x1; iStack4 < 0x9; iStack4 += 0x1) {
    pass1_1008_612e(0x0,0x1d,uVar6);
    uVar5 = uVar6;
    pass1_1008_612e(0x1,0x2,uVar5);
    if ((uVar6 & 0x1) != 0x0) {
      uVar5 = -uVar5;
    }
    iVar8 = iStack4 * 0x4;
    puVar1 = (&param_1->field_0x34 + iVar8);
    uVar2 = *puVar1;
    uVar4 = uVar5 + *puVar1;
    uVar6 = uVar4;
    iVar3 = (&param_1->field_0x34 + iVar8 + 0x2);
    (&param_1->field_0x34 + iVar8) = uVar4;
    (&param_1->field_0x36 + iVar8) =
         (uVar5 >> 0xf) + iVar3 + CARRY2(uVar5,uVar2);
  }
  return;
}
