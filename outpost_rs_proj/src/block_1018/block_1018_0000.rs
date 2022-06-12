pub fn pass1_1018_0000(mut param_1: i16,u8 *param_2,mut param_3: u32,mut param_4: u32)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut BVar5: bool;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  StructD *pSVar7;
  let mut uVar8: u16;
  u8 local_20 [0x10];
  let mut iStack16: i16;

    // Segment:    4
    // Offset:     00024460
    // Length:     ee6a
    // Min Alloc:  ee6a
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
  pSVar7 = (StructD *)CONCAT22(in_register_0000000a,param_2);
  read_file_1008_7cfe(param_4,(param_4 >> 0x10),0x2);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
  }
  else {
    iVar4 = param_3;
    BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | (iVar4 + 0x16)),0x4);
    if ((((BVar5 != 0x0) &&
         (BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | (iVar4 + 0x1a)),0x4),
         BVar5 != 0x0)) &&
        (BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | (iVar4 + 0x20)),0x4),
        BVar5 != 0x0)) &&
       (((BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | (iVar4 + 0x24)),0x4),
         BVar5 != 0x0 &&
         (BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | (iVar4 + 0x30)),0x2),
         BVar5 != 0x0)) &&
        (BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | (iVar4 + 0x32)),0x2),
        BVar5 != 0x0)))) {
      uVar8 = (param_3 >> 0x10);
      if ((iVar4 + 0x30) != 0x0) {
        iVar2 = (iVar4 + 0x32);
        if (_PTR_LOOP_1050_5f2c == 0x0) {
          PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
          PTR_LOOP_1050_5f2e = pSVar7;
        }
        else {
        }
        uVar6 = fn_ptr_op_1000_1708(iVar2 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
        (iVar4 + 0x2c) = uVar6;
        (iVar4 + 0x2e) = PTR_LOOP_1050_5f2e;
        pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_20));
        for (iStack16 = 0x0; piVar1 = (iVar4 + 0x30), *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 += 0x1
            ) {
          BVar5 = read_file_1008_7bc8(param_4,(u16 *)CONCAT22(0x1050,local_20));
          if (BVar5 == 0x0) {
            u16_1050_0310 = 0x6d0;
            return;
          }
          uVar3 = (iVar4 + 0x2c);
          pass1_1008_3f62((u16 *)(uVar3 & 0xffff0000 | (uVar3 + iStack16 * 0x6)),
                          (u16 *)CONCAT22(0x1050,local_20));
        }
      }
      return;
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}
pub fn pass1_1018_017c(mut param_1: u32,mut param_2: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x1e) = param_2;
  pass1_1010_1f62((astruct_27 *)(param_1 & 0xffff | uVar1 << 0x10),0x4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_0196(mut param_1: u16 ,u8 *param_2,mut param_3: u32,mut param_4: u32,mut param_5: u32)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  StructD *pSVar7;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u32;

  pSVar7 = (StructD *)CONCAT22(in_register_0000000a,param_2);
  pass1_1030_8344(_u16_1050_5748,param_5);
  uVar9 = (param_3 >> 0x10);
  iVar8 = param_3;
  if (*(i32 *)(iVar8 + 0x2c) == 0x0) {
    (iVar8 + 0x32) = 0x5;
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
    }
    else {
      pSVar7 = (StructD *)(_PTR_LOOP_1050_5f2c >> 0x10);
    }
    uVar4 = fn_ptr_op_1000_1708(0x1e,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar7);
  }
  else {
    uVar4 = (iVar8 + 0x30) + 0x1;
    if (uVar4 < (iVar8 + 0x32)) goto LAB_1018_022a;
    piVar1 = (iVar8 + 0x32);
    *piVar1 = *piVar1 + 0x5;
    uVar3 = (iVar8 + 0x2c);
    uVar10 = pass1_1000_0ed4(0x1,(iVar8 + 0x32) * 0x6,0x0,(astruct_172 *)uVar3,
                             (astruct_172 *)(uVar3 >> 0x10));
    pSVar7 = (StructD *)(uVar10 >> 0x10);
    uVar4 = uVar10;
  }
  (iVar8 + 0x2c) = uVar4;
  (iVar8 + 0x2e) = pSVar7;//
LAB_1018_022a:
  uVar6 = SUB42(pSVar7,0x0);
  pass1_1030_8344(_u16_1050_5748,param_4);
  uVar5 = (uVar4 + 0x10);
  pass1_1030_8344(_u16_1050_5748,uVar5);
  iVar2 = (iVar8 + 0x30);
  piVar1 = (iVar8 + 0x30);
  *piVar1 = *piVar1 + 0x1;
  uVar10 = (iVar8 + 0x2c);
  pass1_1008_3f62((u16 *)(uVar10 & 0xffff0000 | (uVar10 + iVar2 * 0x6)),
                  (u16 *)CONCAT22(uVar6,uVar5 + 0xc));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_028c(mut param_1: u16 ,StructD *param_2,mut param_3: u32,mut param_4: u32,mut param_5: u16 )

{
  let mut uVar1: u32;
  code **ppcVar2;
  u8 *puVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar13: u16;
  let mut paVar10: *mut Struct57;
  let mut uVar12: u32;
  let mut iVar14: i16;
  let mut uVar15: u16;
  let mut in_stack_0000fe32: u16;
  let mut in_stack_0000fe76: u16;
  let mut in_stack_0000ff56: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ff60: u16;
  let mut in_stack_0000ff8a: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ffa0: u16;
  let mut in_stack_0000ffa4: u16;
  let mut iStack36: i16;
  let mut puStack28: *mut u32;
  u8 local_18 [0x4];
  let mut uStack20: u16;
  astruct_74 *paStack12;
  let mut iStack8: i16;
  let mut uStack6: u16;
  let mut uStack4: u16;
  let mut uVar11: u32;

  pass1_1030_8344(_u16_1050_5748,param_4);
  uStack4 = SUB42(param_2,0x0);
  uStack6 = param_1;
  iStack8 = pass1_1030_5b00(CONCAT22(uStack4,param_1));
  paStack12 = (astruct_74 *)
              mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_5,iStack8),
                              in_stack_0000fe76,in_stack_0000ff9a,in_stack_0000ffa0,in_stack_0000ffa4);
  uVar13 = (param_2 >> 0x10);
  pass1_1008_6c90((u16 *)CONCAT22(0x1050,local_18));
  pass1_1018_0b1e(paStack12,(u16 *)CONCAT22(0x1050,local_18));
  paVar10 = (astruct_57 *)CONCAT22(uVar13,uStack20 >> 0xf);
  if ((uStack20 >> 0xf | uStack20) == 0x0) {
    puVar3 = local_18;
    pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),param_4);
  }
  else {
    puVar3 = local_18;
    pass1_1030_62e4(_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar3),param_4);
  }
  uVar8 = paVar10;
  puStack28 = CONCAT22(uVar8,puVar3);
  uVar4 = uVar8 | puVar3;
  if (uVar4 == 0x0) {
    return;
  }
  pass1_1018_04f2((StructD *)param_3);
  uVar15 = 0x1000;
  mem_op_1000_179c(0x1c,paVar10);
  uVar9 = paVar10 | uVar4;
  uVar11 = paVar10 & 0xffff0000 | uVar9;
  iVar14 = param_3;
  uVar13 = (param_3 >> 0x10);
  uVar5 = uVar4;
  if (uVar9 == 0x0) {
    (iVar14 + 0x12) = 0x0;
  }
  else {
    uVar15 = 0x1008;
    struct_op_1008_8e9e((astruct_78 *)CONCAT22(paVar10,uVar4),0x6,0x24);
    (iVar14 + 0x12) = uVar5;
    (iVar14 + 0x14) = uVar11;
  }
  ppcVar2 = (code **)(*puStack28 + 0x10);
  (**ppcVar2)(uVar15,puVar3,uVar8,uVar4);
  for (iStack36 = 0x0; iStack36 < uVar5; iStack36 += 0x1) {
    uVar7 = iStack36;
    ppcVar2 = (code **)(*puStack28 + 0x4);
    (**ppcVar2)();
    uVar4 = uVar11 | uVar7;
    uVar12 = uVar11 & 0xffff0000 | uVar4;
    if (uVar4 != 0x0) {
      iVar6 = iStack36 / 0x6;
      uVar12 = uVar11 & 0xffff0000 | (long)iStack36 % 0x6 & 0xffffU;
      uVar1 = (iVar14 + 0xe);
      pass1_1018_dd7c(uVar12,uVar1,(uVar1 >> 0x10),CONCAT22(iStack36 % 0x6,iVar6),
                      uVar7 & 0xffff | uVar11 << 0x10,in_stack_0000ff8a,in_stack_0000ff5c,in_stack_0000ff60,
                      in_stack_0000ff56,in_stack_0000fe32);
      pass1_1008_8faa(*(astruct_78 **)(iVar14 + 0x12),CONCAT22(uVar12,iVar6));
    }
    uVar11 = uVar12;
  }
  return;
}
pub fn pass1_1018_03ea(mut param_1: u32,mut param_2: i16)

{
  if (param_2 != 0x5) {
    return;
  }
  pass1_1010_1f62((astruct_27 *)(param_1 & 0xffff0000 | (param_1 - 0xa)),0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_0412(param_1: *mut astruct_27,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: u32)

{
  astruct_97 *pstruct97_1;
  astruct_97 struct97_128;
  let mut uStack4: u16;

  uStack4 = 0x0;
  if (((0x72 < param_4) && (!SBORROW2(param_4,0x73))) &&
     ((param_4 == 0x75 || (param_4 - 0x74) < 0x1 ||
      ((0x0 < (param_4 - 0x76) && ((param_4 - 0x77) < 0x2)))))) {
    uStack4 = 0x1;
  }
  struct_op_1028_933c((astruct_97 *)CONCAT22(0x1050,&struct97_128),param_2,uStack4,param_4,param_3,
                      (param_3 >> 0x10),(param_1 + 0x24),param_5);
  pstruct97_1 = &struct97_128;
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,pstruct97_1));
  if (pstruct97_1 != NULL) {
    pass1_1010_1f62(param_1,0x6);
  }
  return;
}
pub fn pass1_1018_04a4(mut param_1: u32,mut param_2: u32)

{
  (param_1 + 0x16) = param_2;
  return;
}



pub fn pass1_1018_04b8(mut param_1: u32) -> u32

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x18),(param_1 + 0x16));
}
pub fn pass1_1018_04ca(mut param_1: u32,mut param_2: u32)

{
  (param_1 + 0x1a) = param_2;
  return;
}
pub fn pass1_1018_04de(mut param_1: u32,mut param_2: u32)

{
  (param_1 + 0x20) = param_2;
  return;
}
pub fn pass1_1018_04f2(StructD *param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0x12);
  uVar2 = (iVar4 + 0x14);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0x12) = 0x0;
  return;
}



u16 * pass1_1018_0526(param_1: *mut u16,param_2: u8)

{
  param_1 = (u16 *)(param_1 & 0xffff0000 | (param_1 - 0xa));
  pass1_1010_eb66((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



StructD * FUN_1018_0532(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1010_eb66(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1018_0570(param_1: *mut astruct_19,mut param_2: u16 ,mut param_3: u16 )

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut uVar6: u16;
  let mut paVar5: *mut Struct57;
  let mut puVar7: *mut u16;
  let mut puVar8: *mut u32;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut uVar10: u16;
  astruct_19 *uVar9;
  astruct_19 *uVar11;

  uVar6 = (in_EDX >> 0x10);
  uVar9 = (astruct_19 *)param_1;
  uVar10 = (param_1 >> 0x10);
  get_sys_metrics_1018_4b1e(param_1,0x0,param_2);
  uVar9.field17_0x20 = 0x389a;
  uVar9.field18_0x22 = 0x1008;
  uVar9.field17_0x20 = 0x3aa8;
  uVar9.field18_0x22 = 0x1008;
  &uVar9.field19_0x24 = 0x0;
  &uVar9.field22_0x2c = 0x0;
  pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | ZEXT24(&uVar9.field24_0x30)));
  puVar7 = pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | ZEXT24(&uVar9.field26_0x36)));
  paVar5 = (astruct_57 *)CONCAT22(uVar6,(puVar7 >> 0x10));
  &uVar9.field_0x3c = 0x0;
  pass1_1008_6c90((u16 *)(param_1 & 0xffff0000 | ZEXT24(&uVar9.field34_0x40)));
  uVar6 = 0x0;
  &uVar9.field_0x4c = 0x0;
  &uVar9.field_0x5a = 0x0;
  uVar9.field53_0x5e = 0x0;
  (uVar9 + 0x1) = 0x0;
  &uVar9[0x1].field2_0x4 = 0xff00;
  (&uVar9[0x1].field2_0x4 + 0x2) = 0x0;
  &uVar9[0x1].field3_0x8 = 0x10000fb;
  &uVar9[0x1].ver_res_0xc = 0x10000f9;
  &uVar9[0x1].field8_0x10 = 0x10000ff;
  &uVar9[0x1].field10_0x14 = 0x10000fe;
  &uVar9[0x1].field12_0x18 = 0x10000fc;
  &uVar9[0x1].field15_0x1c = 0x0;
  &uVar9[0x1].field17_0x20 = 0x0;
  uVar9[0x1].field19_0x24 = 0x1;
  &uVar9[0x1].field20_0x26 = 0x0;
  (&uVar9[0x1].field20_0x26 + 0x2) = 0x0;
  uVar9[0x1].field22_0x2c = 0x0;
  &uVar9[0x1].field23_0x2e = 0x0;
  &uVar9[0x1].field25_0x32 = 0x0;
  (&uVar9[0x1].field25_0x32 + 0x2) = 0x0;
  uVar9[0x1].field27_0x38 = 0x0;
  &uVar9[0x1].field_0x3a = 0x0;
  (&uVar9[0x1].field34_0x40 + 0x2) = 0x0;
  &uVar9[0x1].field36_0x46 = 0xffff;
  (&uVar9[0x1].field36_0x46 + 0x2) = 0x0;
  param_1.offset_0x0 = 0x1874;
  uVar9.segment_0x2 = 0x1018;
  uVar9.field17_0x20 = 0x18b0;
  uVar9.field18_0x22 = 0x1018;
  if ((PTR_LOOP_1050_3960 == NULL) && (_PTR_LOOP_1050_3962 == 0x0)) {
    mem_op_1000_179c(0x8,paVar5);
    _PTR_LOOP_1050_3962 = CONCAT22(paVar5,uVar6);
    pass1_1000_4906((StructD *)CONCAT22(paVar5,uVar6),NULL,0x8);
  }
  PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + 0x1;
  puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x2f),in_stack_0000fe98,
                           in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
  paVar5 = (astruct_57 *)(paVar5 & 0xffff0000);
  uVar9.field22_0x2c = puVar8;
  uVar9.field23_0x2e = (puVar8 >> 0x10);
  if (param_1 == NULL) {
    puVar3 = NULL;
  }
  else {
    paVar5 = (astruct_57 *)(paVar5 | uVar10);
    puVar3 = &uVar9.field17_0x20;
  }
  uVar1 = &uVar9.field22_0x2c;
  uVar6 = uVar1;
  ppcVar2 = (code **)(*(u32*)&uVar9.field22_0x2c + 0x4);
  (**ppcVar2)(0x1010,uVar6,(uVar1 >> 0x10),0x0,puVar3,paVar5);
  puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(uVar6,0x2),in_stack_0000fe8e,in_stack_0000ffb2,
                           in_stack_0000ffb8,in_stack_0000ffbc);
  paVar5 = (astruct_57 *)(paVar5 & 0xffff0000 | puVar8 >> 0x10);
  if ((puVar8 + 0x80) != 0x0) {
    uVar9[0x1].field19_0x24 = 0x2;
  }
  puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(uVar6,0x9),in_stack_0000fe8e,in_stack_0000ffb2,
                           in_stack_0000ffb8,in_stack_0000ffbc);
  paVar5 = (astruct_57 *)(paVar5 & 0xffff0000 | puVar8 >> 0x10);
  &uVar9[0x1].field_0x3e = puVar8;
  &uVar9[0x1].field34_0x40 = (puVar8 >> 0x10);
  uVar4 = pass1_1010_65d0(puVar8 & 0xffff0000 | &uVar9[0x1].field_0x3e,0x88);
  if (uVar4 != 0x0) {
    (&uVar9[0x1].field36_0x46 + 0x2) = 0x1;
  }
  puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(uVar6,0x3b),in_stack_0000fe8e,in_stack_0000ffb2,
                           in_stack_0000ffb8,in_stack_0000ffbc);
  (&uVar9[0x1].field34_0x40 + 0x2) = puVar8;
  uVar9[0x1].field35_0x44 = (puVar8 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_078e(StructD *param_1)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut puVar3: *mut u16;
  StructD *pSVar4;
  StructD *pstruct_5;
  StructD *uVar6;
  let mut puStack26: *mut u16;
  let mut pcStack6: *mut c_char;

  uVar6 = (StructD *)(param_1 >> 0x10);
  pstruct_5 = (StructD *)param_1;
  param_1.address_offset_field_0x0 = 0x1874;
  pstruct_5.address_offset_field_0x2 = 0x1018;
  pstruct_5.field19_0x20 = 0x18b0;
  pstruct_5.field20_0x22 = 0x1018;
  PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + -0x1;
  (_PTR_LOOP_1050_3962 + pstruct_5.field11_0x12 * 0x2 + -0x4) = 0x0;
  if (PTR_LOOP_1050_3960 == NULL) {
    fn_ptr_1000_17ce(_PTR_LOOP_1050_3962);
    _PTR_LOOP_1050_3962 = NULL;
  }
  fn_ptr_1000_17ce(*(char **)&pstruct_5.field_0x94);
  fn_ptr_1000_17ce(*(char **)&pstruct_5.field_0x9a);
  fn_ptr_1000_17ce(*(char **)&pstruct_5.field_0x88);
  fn_ptr_1000_17ce(*(char **)&pstruct_5.field_0x8e);
  if (pstruct_5.field29_0x2c != NULL) {
    if (param_1 == NULL) {
      puVar3 = NULL;
      pSVar4 = NULL;
    }
    else {
      puVar3 = &pstruct_5.field19_0x20;
      pSVar4 = uVar6;
    }
    pass1_1010_1ea6(pstruct_5.field29_0x2c,(StructD *)CONCAT22(pSVar4,puVar3));
  }
  if (*(i32 *)&pstruct_5.field_0x9e != 0x0) {
    if (param_1 == NULL) {
      puVar3 = NULL;
      pSVar4 = NULL;
    }
    else {
      puVar3 = &pstruct_5.field19_0x20;
      pSVar4 = uVar6;
    }
    pass1_1010_1ea6(&pstruct_5.field_0x9e,(StructD *)CONCAT22(pSVar4,puVar3));
  }
  uVar1 = &pstruct_5.field_0x60;
  uVar2 = &pstruct_5.field_0x62;
  pcStack6 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack6);
  }
  &pstruct_5.field_0x4c = 0x0;
  if (param_1 == NULL) {
    puVar3 = NULL;
    uVar6 = NULL;
  }
  else {
    puVar3 = &pstruct_5.field19_0x20;
  }
  puStack26 = (u16 *)CONCAT22(uVar6,puVar3);
  *puStack26 = 0x389a;
  puVar3[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_0902(u32 *param_1,mut param_2: u32)

{
  let mut uVar1: u32;
  code **ppcVar2;
  astruct_76 **ppaVar3;
  astruct_76 **ppaVar4;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut uVar8: u32;
  let mut puVar9: *mut u32;
  let mut puVar10: *mut u32;

  puVar10 = (param_1 & 0xffff0000 | (param_1 + 0x28));
  ppaVar3 = (astruct_76 **)(param_1 + 0x24);
  puVar9 = (param_1 & 0xffff0000 | ZEXT24(ppaVar3));
  uVar6 = param_1;
  ppaVar4 = ppaVar3;
  pass1_1030_8344(_u16_1050_5748,param_2);
  pass1_1030_5a52(CONCAT22(uVar6,ppaVar4),puVar9,puVar10);
  uVar7 = pass1_1008_4772(*ppaVar3);
  (param_1 + 0x5a) = uVar7;
  (param_1 + 0x5c) = (uVar7 >> 0x10);
  iVar5 = pass1_1018_17f0();
  (param_1 + 0x12) = iVar5 + 0x2;
  (iVar5 * 0x2 + _PTR_LOOP_1050_3962) = 0x1;
  ppcVar2 = (code **)(*param_1 + 0x18);
  (**ppcVar2)();
  (param_1 + 0x3c) = param_2;
  uVar1 = (param_1 + 0x2c);
  uVar8 = pass1_1010_ec18(param_2,param_2,uVar1,(uVar1 >> 0x10),
                          param_2 & 0xffff0000 | (param_1 + 0x3c));
  (param_1 + 0x7c) = uVar8;
  (param_1 + 0x7e) = (uVar8 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn get_sys_metrics_1018_09a8(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  INT16 IVar2;
  INT16 IVar3;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;
  let mut in_stack_0000fe7c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut in_stack_0000ffa6: u16;
  let mut in_stack_0000ffaa: u16;
  let mut piVar8: *mut i16;
  let mut uVar9: u16;
  let mut pcVar10: *mut c_char;
  let mut local_a: i16;
  let mut local_8: i16;
  let mut iStack6: i16;
  INT16 IStack4;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  IStack4 = GetSystemMetrics16(SM_CYCAPTION);
  uVar6 = (param_2 >> 0x10);
  iVar5 = param_2;
  iStack6 = (iVar5 + 0x12) + -0x2;
  pcVar10 = CONCAT22(0x1050,&local_8);
  piVar8 = &local_a;
  uVar9 = SUB42(&DAT_1050_1050,0x0);
  puVar7 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(piVar8,0x48),in_stack_0000fe7c,in_stack_0000ffa0
                           ,in_stack_0000ffa6,in_stack_0000ffaa);
  pass1_1008_3e94((u16 *)(puVar7 & 0xffff0000 | (puVar7 + 0xe)),(u16 *)CONCAT22(uVar9,piVar8),pcVar10
                 );
  (iVar5 + 0x18) = iStack6 * IStack4 + local_8 + 0x146;
  (iVar5 + 0x1a) = iStack6 * IStack4 + local_a + 0x9;
  IVar2 = GetSystemMetrics16(SM_CXBORDER);
  uVar1 = (iVar5 + 0x5a);
  (iVar5 + 0x1c) = IVar2 * 0x2 + (uVar1 + 0x4);
  IVar2 = GetSystemMetrics16(SM_CYCAPTION);
  IVar3 = GetSystemMetrics16(SM_CYBORDER);
  uVar1 = (iVar5 + 0x5a);
  (iVar5 + 0x1e) = IVar3 + IVar2 + (uVar1 + 0x8);
  return;
}



pub fn pass1_1018_0a50(mut param_1: u32) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0x84) == 0x2) {
    return CONCAT22((iVar1 + 0x2a),(iVar1 + 0x28));
  }
  return CONCAT22((iVar1 + 0x26),(iVar1 + 0x24));
}
pub fn pass1_1018_0a76(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0x84) == 0x1) {
    uVar1 = 0x2;
  }
  else {
    uVar1 = 0x1;
  }
  (param_1 + 0x84) = uVar1;
  pass1_1010_1f62((astruct_27 *)(param_1 & 0xffff | uVar2 << 0x10),0x4);
  return;
}
pub fn pass1_1018_0aa0(mut param_1: u32,mut param_2: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x14) = param_2;
  pass1_1018_04de((iVar1 + 0x2c),(iVar1 + 0x3c));
  return;
}
pub fn pass1_1018_0ac0(mut param_1: u32,param_2: *mut StructA)

{
  *(StructA **)(param_1 + 0x80) = param_2;
  return;
}



pub fn pass1_1018_0ad4(mut param_1: u32) -> u32

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x82),(param_1 + 0x80));
}
pub fn pass1_1018_0ae8(mut param_1: u32,mut param_2: u16 )

{
  (param_1 + 0x5e) = param_2;
  return;
}



u16 pass1_1018_0afa(mut param_1: u32)

{
  return (param_1 + 0x5e);
}



pub fn pass1_1018_0b08(mut param_1: u32) -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar1 = (param_1 + 0x7c);
  uVar3 = (uVar1 >> 0x10);
  iVar2 = uVar1;
  return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
}
pub fn pass1_1018_0b1e(param_1: *mut astruct_74,param_2: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  astruct_74 *iVar3;
  let mut uVar3: u16;
  let mut local_8: u16;
  let mut local_6: i16;
  let mut local_4: i16;

  iVar3 = (astruct_74 *)param_1;
  iVar3 = (astruct_74 *)&iVar3.field_0x30;
  pass1_1008_3eb4((astruct_615 *)(param_1 & 0xffff0000 | ZEXT24(iVar3)),(u16 *)CONCAT22(0x1050,&local_8),
                  (u16 *)CONCAT22(0x1050,&local_6),(u16 *)CONCAT22(0x1050,&local_4));
  if (local_4 + -0x3 < 0x1) {
    local_4 = 0x3;
  }
  if (local_6 + -0x3 < 0x1) {
    local_6 = 0x3;
  }
  uVar3 = (param_1 >> 0x10);
  uVar2 = iVar3.field90_0x5a;
  iVar1 = (uVar2 + 0x4);
  if (iVar1 <= local_4 + 0x2) {
    local_4 = iVar1 + -0x3;
  }
  uVar2 = iVar3.field90_0x5a;
  iVar1 = (uVar2 + 0x8);
  if (iVar1 <= local_6 + 0x2) {
    local_6 = iVar1 + -0x3;
  }
  pass1_1008_6cec((u16 *)(param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x40)),local_8,
                  CONCAT22(local_4 + 0x2,local_6 + 0x2),local_8,CONCAT22(local_4 + -0x3,local_6 + -0x3));
  pass1_1008_3f62(param_2,(u16 *)(param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x40)));
  pass1_1008_3f62((u16 *)(param_2 & 0xffff0000 | (param_2 + 0x6)),
                  (u16 *)(param_1 & 0xffff0000 | ZEXT24(&iVar3.field_0x46)));
  return;
}
pub fn pass1_1018_0bf4(mut param_1: i16,mut param_2: u32,mut param_3: i16,mut param_4: u32,mut param_5: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  i32 lVar4;
  StructD *pSVar5;
  let mut uVar6: u32;
  let mut uVar7: u16;
  u8 local_14 [0xc];
  let mut local_8: u16;
  let mut local_6: u32;

  uVar7 = (param_4 >> 0x10);
  switch(param_3) {
  case 0x1:
    (param_2 + 0xc) = 0x0;
    (param_2 + 0x7e) = 0x0;
    return;
  case 0x4:
    pass1_1008_3eb4((astruct_615 *)(param_2 & 0xffff0000 | (param_2 + 0x10)),
                    (u16 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_6),
                    (u16 *)CONCAT22(0x1050,&local_6 + 0x2));
    uVar1 = (param_2 + 0xc);
    local_8 = (uVar1 + 0x1e);
    pass1_1008_3e76((u16 *)(param_2 & 0xffff0000 | (param_2 + 0x10)),local_8,local_6,
                    (local_6 >> 0x10));
    pass1_1008_6c90((u16 *)CONCAT22(0x1050,local_14));
    pass1_1018_0b1e((astruct_74 *)(param_2 & 0xffff0000 | (param_2 - 0x20)),
                    (u16 *)CONCAT22(0x1050,local_14));
    goto LAB_1018_0c71;
  case 0x5:
  case 0x6:
    uVar6 = param_4 & 0xffff0000 | param_2;
    uVar2 = param_2 - 0x20;
    pass1_1018_0dc6(uVar6,(astruct_91 *)(param_2 & 0xffff0000 | uVar2));
    pass1_1018_10c4(uVar6,param_2 & 0xffff0000 | uVar2);
    pass1_1018_1346(uVar6,(astruct_93 *)(param_2 & 0xffff0000 | uVar2));
    uVar7 = (uVar6 >> 0x10);//
LAB_1018_0c71:
    (param_2 + 0x2c) = 0x0;
    lVar4 = *(i32 *)(param_2 + 0x1c);
    pSVar5 = (StructD *)CONCAT22(uVar7,(param_2 + 0x1e));
    uVar1 = (param_2 + 0xc);
    if (*(i32 *)(uVar1 + 0x20) == lVar4) {
      pass1_1018_028c(lVar4,pSVar5,(param_2 + 0xc),(param_2 + 0x1c),param_5);
      (param_2 + 0x2c) = lVar4;
      (param_2 + 0x2e) = pSVar5;
      pass1_1010_1f62((astruct_27 *)(param_2 & 0xffff0000 | (param_2 - 0x20)),param_3);
      return;
    }
    break;
  case 0x14:
    uVar1 = (param_2 + 0xc);
    if (*(i32 *)(uVar1 + 0x20) != *(i32 *)(param_2 + 0x1c)) {
      post_win_msg_1020_291a((param_2 + 0x60));
      return;
    }
    break;
  case 0x15:
    uVar3 = pass1_1010_65d0((param_2 + 0x7e),0x88);
    if (uVar3 != 0x0) {
      (param_2 + 0x88) = 0x1;
      return;
    }
  }
  return;
}
pub fn pass1_1018_0d76(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x2c);
  if (*(i32 *)(uVar1 + 0x20) == *(i32 *)(param_1 + 0x3c)) {
    return;
  }
  return;
}
pub fn pass1_1018_0d9a(mut param_1: u32,param_2: *mut u16,u32 *param_3)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x7c);
  *param_3 = (uVar1 + 0x16);
  uVar1 = (param_1 + 0x7c);
  *param_2 = (uVar1 + 0x1a);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_0dc6(param_1: *mut astruct_57,param_2: *mut astruct_91)

{
  let mut puVar1: *mut u16;
  astruct_92 *paVar2;
  let mut iVar3: i16;
  let mut puVar4: *mut u32;
  let mut pcVar5: *mut c_char;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  astruct_91 *iVar13;
  let mut uVar9: u16;
  let mut local_32: u32;
  let mut uStack46: u16;
  let mut uStack44: u32;
  let mut pcStack40: *mut c_char;
  let mut pcStack36: *mut c_char;
  let mut pcStack32: *mut c_char;
  let mut uStack28: u32;
  let mut uStack24: u32;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
  uVar9 = (param_2 >> 0x10);
  iVar13 = (astruct_91 *)param_2;
  pcStack36 = iVar13.field147_0x94;
  fn_ptr_1000_17ce(pcStack36);
  pcStack40 = iVar13.field149_0x9a;
  pcStack32 = pcStack40;
  fn_ptr_1000_17ce(pcStack40);
  iVar13.field147_0x94 = 0x0;
  iVar13.field149_0x9a = 0x0;
  iVar13.field146_0x92 = 0x0;
  iVar13.field148_0x98 = 0x0;
  while( true ) {
    paVar2 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
    uVar6 = param_1;
    uStack24 = CONCAT22(uVar6,paVar2);
    param_1 = (astruct_57 *)(param_1 & 0xffff0000 | (uVar6 | paVar2));
    if ((uVar6 | paVar2) == 0x0) break;
    pcVar5 = paVar2[0x1c].field4_0x8;
    pcStack40 = pcVar5;
    if (pcVar5 == 0x8000001) {
      puVar1 = &iVar13.field146_0x92;
      *puVar1 = *puVar1 + 0x1;
    }
    else if ((iVar13.field157_0xa8 != 0x0) ||
            (pass1_1008_dfa6(iVar13.field154_0xa2,paVar2.field3_0x4,0x4000001), pcVar5 != 0x0)) {
      puVar1 = &iVar13.field148_0x98;
      *puVar1 = *puVar1 + 0x1;
    }
  }
  if (iVar13.field146_0x92 != 0x0) {
    uVar6 = iVar13.field146_0x92;
    uStack44 = uStack44 & 0xffff0000 | uVar6;
    uVar6 *= 0x6;
    mem_op_1000_179c(uVar6,param_1);
    uVar7 = param_1;
    pcStack32 = CONCAT22(uVar7,uVar6);
    param_1 = (astruct_57 *)(param_1 & 0xffff0000 | (uVar7 | uVar6));
    if ((uVar7 | uVar6) == 0x0) {
      iVar13.field147_0x94 = 0x0;
    }
    else {
      pass1_1000_5586(0x3e38,0x1008,uStack44,0x6,uVar6,uVar7);
      iVar13.field147_0x94 = pcStack32;
    }
  }
  if (iVar13.field148_0x98 != 0x0) {
    uVar6 = iVar13.field148_0x98;
    uStack44 = uStack44 & 0xffff0000 | uVar6;
    uVar6 *= 0x6;
    mem_op_1000_179c(uVar6,param_1);
    uVar7 = param_1;
    pcStack32 = CONCAT22(uVar7,uVar6);
    if ((uVar7 | uVar6) == 0x0) {
      iVar13.field149_0x9a = 0x0;
    }
    else {
      pass1_1000_5586(0x3e38,0x1008,uStack44,0x6,uVar6,uVar7);
      iVar13.field149_0x9a = pcStack32;
    }
  }
  if (local_14.field6_0x10 != 0x0) {
    local_14.field5_0xc = 0x1;
    local_14.field5_0xc = 0x0;
  }
  uVar8 = local_14.field5_0xc;
  uStack28 = 0x0;
  local_14.field4_0x8 = local_14.field5_0xc;
  local_14.field4_0x8 = local_14.field5_0xc;//
LAB_1018_0f74:
  uVar6 = uVar8;
  paVar2 = &local_14;
  pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
  uStack24 = CONCAT22(uVar6,paVar2);
  uVar8 = (uVar6 | paVar2);
  if ((uVar6 | paVar2) == 0x0) {
    return;
  }
  uStack44 = paVar2[0x1c].field4_0x8;
  pcVar5 = *(char **)&paVar2.field6_0x10;
  pcStack40 = pcVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,pcVar5);
  pcStack36 = (pcVar5 & 0xffff | uVar8 << 0x10);
  local_32 = (pcVar5 + 0xc);
  uStack46 = (pcVar5 + 0x10);
  puVar4 = &local_32;
  if (uStack44 != 0x8000001) goto LAB_1018_0ffc;
  iVar3 = &iVar13.field147_0x94;
  uVar6 = (&iVar13.field147_0x94 + 0x2);
  uStack28 = uStack28 & 0xffff | (uStack28 + 0x1) << 0x10;
  goto LAB_1018_0fe8;//
LAB_1018_0ffc:
  if ((iVar13.field157_0xa8 != 0x0) ||
     (pass1_1008_dfa6(iVar13.field154_0xa2,*(i32 *)(uStack24 + 0x4),0x4000001), puVar4 != NULL)) {
    iVar3 = &iVar13.field149_0x9a;
    uVar6 = (&iVar13.field149_0x9a + 0x2);
    uStack28 = uStack28 & 0xffff0000 | (uStack28 + 0x1);
    uStack28 = uStack28;//
LAB_1018_0fe8:
    uVar8 = uVar6;
    pass1_1008_3f62((u16 *)CONCAT22(uVar6,iVar3 + uStack28 * 0x6),(u16 *)CONCAT22(0x1050,&local_32));
  }
  goto LAB_1018_0f74;
}
