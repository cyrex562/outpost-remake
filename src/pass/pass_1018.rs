
fn pass1_1018_0000(param_1: u32,param_2: u32,param_3: i16,uchar *param_4,param_5: u16)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u32;
  let uVar4: u16;
  let iVar5: i16;
  let BVar6: bool;
  let uVar7: u16;
  let uVar8: u16;
  let local_20: [u8;10];
  let iStack16: i16;
  
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
  uVar8 = param_2;
  uVar7 = (param_2 >> 0x10);
  read_file_1008_7cfe(uVar8,uVar7,0x2,0x1008,param_5);
  if (param_3 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d4;
  }
  else {
    iVar5 = param_1;
    uVar4 = (param_1 >> 0x10);
    BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x16,0x0,uVar4,0x4,0x1008);
    if ((((BVar6 != 0x0) &&
         (BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x1a,0x0,uVar4,0x4,0x1008),
         BVar6 != 0x0)) &&
        (BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x20,0x0,uVar4,0x4,0x1008),
        BVar6 != 0x0)) &&
       (((BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x24,0x0,uVar4,0x4,0x1008),
         BVar6 != 0x0 &&
         (BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x30,0x0,uVar4,0x2,0x1008),
         BVar6 != 0x0)) &&
        (BVar6 = read_file_1008_7dee(uVar8,uVar7,iVar5 + 0x32,0x0,uVar4,0x2,0x1008),
        BVar6 != 0x0)))) {
      if ((iVar5 + 0x30) != 0x0) {
        iVar2 = (iVar5 + 0x32);
        if (_PTR_LOOP_1050_5f2c == 0x0) {
          PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4,0x1000);
          PTR_LOOP_1050_5f2e = param_4;
        }
        else {
        }
        uVar7 = fn_ptr_op_1000_1708(iVar2 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,
                                    PTR_LOOP_1050_5f2e,0x1000);
        (iVar5 + 0x2c) = uVar7;
        *(uchar **)(iVar5 + 0x2e) = PTR_LOOP_1050_5f2e;
        pass1_1008_3e38((u16 *)CONCAT22(param_5,local_20));
        for (iStack16 = 0x0; piVar1 = (i16 *)(iVar5 + 0x30),
            *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 += 0x1) {
          BVar6 = read_file_1008_7bc8(param_2,CONCAT22(param_5,local_20),0x1008,
                                      param_5);
          if (BVar6 == 0x0) {
            PTR_LOOP_1050_0310 = 0x6d0;
            return;
          }
          uVar3 = (iVar5 + 0x2c);
          pass1_1008_3f62((u16 *)
                          (uVar3 & 0xffff0000 | (uVar3 + iStack16 * 0x6)
                          ),CONCAT22(param_5,local_20));
        }
      }
      return;
    }
    PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}



fn pass1_1018_017c(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x1e) = param_2;
  pass1_1010_1f62(param_3,param_1 & 0xffff | uVar1 << 0x10,0x4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_0196(param_1: u32,param_2: u32,param_3: u32,param_4: u16,uchar *param_5,
               param_6: u16)

{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u32;
  let uVar4: u32;
  let uVar5: u16;
  let uVar6: u32;
  let iVar7: i16;
  let uVar8: u16;
  let lVar9: i32;
  
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,param_3);
  uVar8 = (param_1 >> 0x10);
  iVar7 = param_1;
  if (*(long *)(iVar7 + 0x2c) == 0x0) {
    (iVar7 + 0x32) = 0x5;
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_5,0x1000);
      PTR_LOOP_1050_5f2e = param_5;
    }
    else {
    }
    uVar5 = fn_ptr_op_1000_1708(0x1e,0x0,0x1,PTR_LOOP_1050_5f2c,
                                PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
    uVar5 = (iVar7 + 0x30) + 0x1;
    PTR_LOOP_1050_5f2e = param_5;
    if (uVar5 < (iVar7 + 0x32)) goto LAB_1018_022a;
    piVar1 = (i16 *)(iVar7 + 0x32);
    *piVar1 = *piVar1 + 0x5;
    uVar3 = (iVar7 + 0x2c);
    lVar9 = pass1_1000_0ed4(0x1000,param_6,0x1,(iVar7 + 0x32) * 0x6,0x0,
                            uVar3,(uVar3 >> 0x10));
    PTR_LOOP_1050_5f2e = (lVar9 >> 0x10);
    uVar5 = lVar9;
  }
  (iVar7 + 0x2c) = uVar5;
  *(uchar **)(iVar7 + 0x2e) = PTR_LOOP_1050_5f2e;
LAB_1018_022a:
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,param_2);
  uVar6 = (uVar5 + 0x10);
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,uVar6);
  iVar2 = (iVar7 + 0x30);
  piVar1 = (i16 *)(iVar7 + 0x30);
  *piVar1 = *piVar1 + 0x1;
  uVar4 = (iVar7 + 0x2c);
  pass1_1008_3f62((u16 *)(uVar4 & 0xffff0000 | (uVar4 + iVar2 * 0x6))
                  ,CONCAT22(PTR_LOOP_1050_5f2e,uVar6 + 0xc));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_028c(param_1: u32,param_2: u32,param_3: u16,param_4: u16,param_5: u16)
{
  let uVar1: u32;
  code **ppcVar2;
  let puVar3: *mut u8;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u32;
  let puVar7: *mut u8
  let puVar8: *mut u8
  let uVar9: u16;
  let extraout_DX: u16;
  let uVar10: u16;
  let iVar11: i16;
  let unaff_DI: i16;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u16;
  let uVar15: u16;
  let iStack36: i16;
  let puStack28: u32;
  let local_18: [u8;4];
  let uStack20: u16;
  let puStack12: *mut u16;
  let uStack8: u16;
  let uStack6: u16;
  let puStack4: *mut u8
  
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,param_2);
  uStack6 = param_3;
  puStack4 = param_4;
  uStack8 = pass1_1030_5b00(CONCAT22(param_4,param_3));
  puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uStack8,param_5,param_4,
                              unaff_DI);
  pass1_1008_6c90((u16 *)CONCAT22(param_5,local_18));
  pass1_1018_0b1e(puStack12,CONCAT22(param_5,local_18),param_5);
  puVar7 = (uStack20 >> 0xf);
  if ((puVar7 | uStack20) == 0x0) {
    puVar3 = local_18;
    pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(param_5,puVar3),param_2,param_5);
  }
  else {
    puVar3 = local_18;
    pass1_1030_62e4(_PTR_LOOP_1050_5740,CONCAT22(param_5,puVar3),param_2,param_5
                   );
  }
  puStack28 = CONCAT22(puVar7,puVar3);
  uVar4 = puVar7 | puVar3;
  if (uVar4 == 0x0) {
    return;
  }
  puVar8 = puVar7;
  pass1_1018_04f2(param_1);
  uVar14 = 0x1c;
  uVar13 = 0x1000;
  mem_op_1000_179c(0x1c,puVar8,0x1000);
  uVar9 = puVar8 | uVar4;
  iVar11 = param_1;
  uVar12 = (param_1 >> 0x10);
  uVar15 = uVar4;
  if (uVar9 == 0x0) {
    (iVar11 + 0x12) = 0x0;
  }
  else {
    uVar13 = 0x1008;
    struct_op_1008_8e9e((astruct_78 *)CONCAT22(puVar8,uVar4),0x6,0x24);
    (iVar11 + 0x12) = uVar4;
    (iVar11 + 0x14) = uVar9;
  }
  ppcVar2 = (code **)(*puStack28 + 0x10);
  (**ppcVar2)(uVar13,puVar3,puVar7,uVar14,uVar15);
  for (iStack36 = 0x0; iStack36 < uVar4; iStack36 += 0x1) {
    uVar6 = SEXT24(iStack36);
    ppcVar2 = (code **)(*puStack28 + 0x4);
    (**ppcVar2)();
    if ((extraout_DX | uVar6) != 0x0) {
      iVar5 = iStack36 / 0x6;
      uVar10 = iStack36 % 0x6;
      uVar1 = (iVar11 + 0xe);
      pass1_1018_dd7c(uVar1,(uVar1 >> 0x10),
                      CONCAT22(iStack36 % 0x6,iVar5),
                      uVar6 & 0xffff | extraout_DX << 0x10,uVar10,param_5);
      pass1_1008_8faa((iVar11 + 0x12),CONCAT22(uVar10,iVar5));
    }
  }
  return;
}



fn pass1_1018_03ea(param_1: u32,param_2: i16,param_3: u16)
{
  if (param_2 != 0x5) {
    return;
  }
  pass1_1010_1f62(param_3,param_1 & 0xffff0000 | (param_1 - 0xa),0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_0412(param_1: u32,param_2: u16,param_3: u32,param_4: u16,param_5: u32,
               param_6: u16,param_7: u8)

{
  let puVar1: *mut u8;
  let local_128: [u8;124];
  let uStack4: u16;
  
  uStack4 = 0x0;
  if (((0x72 < param_4) && (!SBORROW2(param_4,0x73))) &&
     ((param_4 == 0x75 || (param_4 - 0x74) < 0x1 ||
      ((0x0 < (param_4 - 0x76) && ((param_4 - 0x77) < 0x2)))))) {
    uStack4 = 0x1;
  }
  struct_op_1028_933c((astruct_100 *)CONCAT22(param_6,local_128),param_2,uStack4,param_4,
                      param_3,(param_3 >> 0x10),
                      (param_1 + 0x24),param_5,param_6,param_7);
  puVar1 = local_128;
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_6,puVar1));
  if (puVar1 != 0x0) {
    pass1_1010_1f62(param_6,param_1,0x6);
  }
  return;
}



fn pass1_1018_04a4(param_1: u32,param_2: u32)
{
  (param_1 + 0x16) = param_2;
  return;
}



fn pass1_1018_04b8(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x18),
                  (param_1 + 0x16));
}



fn pass1_1018_04ca(param_1: u32,param_2: u32)
{
  (param_1 + 0x1a) = param_2;
  return;
}



fn pass1_1018_04de(param_1: u32,param_2: u32)
{
  (param_1 + 0x20) = param_2;
  return;
}



fn pass1_1018_04f2(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
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



fn pass1_1018_0526(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xa));
  pass1_1010_eb66(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1018_078e(param_1: *mut u16,param_2: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let puVar3: *mut u16;
  let uVar4: u16;
  astruct_500 *uVar5;
  let uVar6: u16;
  let puStack26: *mut u16;
  astruct_18 *paStack6;
  
  uVar6 = (param_1 >> 0x10);
  uVar5 = (astruct_500 *)param_1;
  *param_1 = 0x1874;
  uVar5->field_0x2 = 0x1018;
  uVar5->field_0x20 = 0x18b0;
  uVar5->field_0x22 = 0x1018;
  PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + -0x1;
  (_PTR_LOOP_1050_3962 + uVar5->field_0x12 * 0x2 + -0x4) = 0x0;
  if (PTR_LOOP_1050_3960 == 0x0) {
    fn_ptr_1000_17ce(_PTR_LOOP_1050_3962,0x1000);
    _PTR_LOOP_1050_3962 = (astruct_18 *)0x0;
  }
  fn_ptr_1000_17ce((astruct_18 *)uVar5->field_0x94,0x1000);
  fn_ptr_1000_17ce((astruct_18 *)uVar5->field_0x9a,0x1000);
  fn_ptr_1000_17ce((astruct_18 *)uVar5->field_0x88,0x1000);
  fn_ptr_1000_17ce((astruct_18 *)uVar5->field_0x8e,0x1000);
  if (uVar5->field_0x2c != 0x0) {
    if (param_1 == 0x0) {
      puVar3 = 0x0;
      uVar4 = 0x0;
    }
    else {
      puVar3 = &uVar5->field_0x20;
      uVar4 = uVar6;
    }
    pass1_1010_1ea6(uVar5->field_0x2c,CONCAT22(uVar4,puVar3),param_2);
  }
  if (uVar5->field_0x9e != 0x0) {
    if (param_1 == 0x0) {
      puVar3 = 0x0;
      uVar4 = 0x0;
    }
    else {
      puVar3 = &uVar5->field_0x20;
      uVar4 = uVar6;
    }
    pass1_1010_1ea6(uVar5->field_0x9e,CONCAT22(uVar4,puVar3),param_2);
  }
  uVar1 = uVar5->field_0x60;
  uVar2 = uVar5->field_0x62;
  paStack6 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack6,0x1000);
  }
  uVar5->field_0x4c = 0x0;
  if (param_1 == 0x0) {
    puVar3 = 0x0;
    uVar6 = 0x0;
  }
  else {
    puVar3 = &uVar5->field_0x20;
  }
  puStack26 = CONCAT22(uVar6,puVar3);
  *puStack26 = 0x389a;
  puVar3[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_0902(param_1: *mut u32,param_2: u32)
{
  let uVar1: u32;
  code **ppcVar2;
  astruct_76 **ppaVar3;
  astruct_76 **ppaVar4;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u32;
  let puVar8: u32;
  let puVar9: u32;
  
  puVar9 = (param_1 & 0xffff0000 | (param_1 + 0x28));
  ppaVar3 = (astruct_76 **)(param_1 + 0x24);
  puVar8 = (param_1 & 0xffff0000 | ZEXT24(ppaVar3));
  uVar6 = param_1._2_2_;
  ppaVar4 = ppaVar3;
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,param_2);
  pass1_1030_5a52(CONCAT22(uVar6,ppaVar4),puVar8,puVar9);
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
  uVar7 = pass1_1010_ec18(uVar1,(uVar1 >> 0x10),
                          param_2 & 0xffff0000 | (param_1 + 0x3c),
                          param_2,param_2._2_2_);
  (param_1 + 0x7c) = uVar7;
  (param_1 + 0x7e) = (uVar7 >> 0x10);
  return;
}


fn pass1_1018_0a50(param_1: u32) -> u32

{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0x84) == 0x2) {
    return CONCAT22((iVar1 + 0x2a),(iVar1 + 0x28));
  }
  return CONCAT22((iVar1 + 0x26),(iVar1 + 0x24));
}



fn pass1_1018_0a76(param_1: u32,param_2: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0x84) == 0x1) {
    uVar1 = 0x2;
  }
  else {
    uVar1 = 0x1;
  }
  (param_1 + 0x84) = uVar1;
  pass1_1010_1f62(param_2,param_1 & 0xffff | uVar2 << 0x10,0x4);
  return;
}



fn pass1_1018_0aa0(param_1: u32,param_2: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x14) = param_2;
  pass1_1018_04de((iVar1 + 0x2c),(iVar1 + 0x3c));
  return;
}



fn pass1_1018_0ac0(param_1: u32,param_2: u32)
{
  (param_1 + 0x80) = param_2;
  return;
}



fn pass1_1018_0ad4(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x82),
                  (param_1 + 0x80));
}



fn pass1_1018_0ae8(param_1: u32,param_2: u16)
{
  (param_1 + 0x5e) = param_2;
  return;
}



fn pass1_1018_0afa(param_1: u32) -> u16

{
  return (param_1 + 0x5e);
}



fn pass1_1018_0b08(param_1: u32) -> u32

{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar1 = (param_1 + 0x7c);
  uVar3 = (uVar1 >> 0x10);
  iVar2 = uVar1;
  return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
}



fn pass1_1018_0b1e(param_1: *mut u16,param_2: *mut u16,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u32;
  astruct_74 *iVar3;
  let uVar3: u16;
  let local_8: u16;
  let local_6: i16;
  let local_4: i16;
  
  iVar3 = (astruct_74 *)param_1;
  iVar3 = (astruct_74 *)&iVar3->field_0x30;
  pass1_1008_3eb4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar3)),
                  CONCAT22(param_3,&local_8),
                  CONCAT22(param_3,&local_6),
                  CONCAT22(param_3,&local_4));
  if (local_4 + -0x3 < 0x1) {
    local_4 = 0x3;
  }
  if (local_6 + -0x3 < 0x1) {
    local_6 = 0x3;
  }
  uVar3 = (param_1 >> 0x10);
  uVar2 = iVar3->field_0x5a;
  iVar1 = (uVar2 + 0x4);
  if (iVar1 <= local_4 + 0x2) {
    local_4 = iVar1 + -0x3;
  }
  uVar2 = iVar3->field_0x5a;
  iVar1 = (uVar2 + 0x8);
  if (iVar1 <= local_6 + 0x2) {
    local_6 = iVar1 + -0x3;
  }
  pass1_1008_6cec((u16 *)
                  (param_1 & 0xffff0000 | &iVar3->field_0x40),local_8,
                  CONCAT22(local_4 + 0x2,local_6 + 0x2),local_8,
                  CONCAT22(local_4 + -0x3,local_6 + -0x3));
  pass1_1008_3f62(param_2,
                          (param_1 & 0xffff0000 | &iVar3->field_0x40))
  ;
  pass1_1008_3f62((u16 *)(param_2 & 0xffff0000 | (param_2 + 0x6)),

                  (param_1 & 0xffff0000 | &iVar3->field_0x46));
  return;
}



fn pass1_1018_0bf4(param_1: u16,param_2: i16,param_3: u32,param_4: i16)
{
  let uVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let lVar4: i32;
  let uVar5: u16;
  u8 local_14 [0xc];
  let local_8: u16;
  let local_6: u32;
  
  if (false) {
    return;
  }
  switch(param_4) {
  case 0x1:
    (param_3 + 0xc) = 0x0;
    (param_3 + 0x7e) = 0x0;
    return;
  case 0x4:
    pass1_1008_3eb4((u16 *)(param_3 & 0xffff0000 | (param_3 + 0x10)),
                    CONCAT22(param_1,&local_8),
                    CONCAT22(param_1,&local_6),
                    CONCAT22(param_1,&local_6 + 0x2));
    uVar1 = (param_3 + 0xc);
    local_8 = (uVar1 + 0x1e);
    pass1_1008_3e76((u16 *)(param_3 & 0xffff0000 | (param_3 + 0x10)),
                    local_8,local_6,(local_6 >> 0x10));
    pass1_1008_6c90((u16 *)CONCAT22(param_1,local_14));
    pass1_1018_0b1e((u16 *)(param_3 & 0xffff0000 | (param_3 - 0x20)),
                    CONCAT22(param_1,local_14),param_1);
    goto LAB_1018_0c71;
  case 0x5:
  case 0x6:
    uVar2 = param_3 - 0x20;
    uVar5 = param_3._2_2_;
    pass1_1018_0dc6(param_3 & 0xffff0000 | uVar2,param_1);
    pass1_1018_10c4(param_1,uVar5,param_3 & 0xffff0000 | uVar2);
    pass1_1018_1346(param_1,uVar5,(astruct_93 *)(param_3 & 0xffff0000 | uVar2));
LAB_1018_0c71:
    (param_3 + 0x2c) = 0x0;
    lVar4 = *(long *)(param_3 + 0x1c);
    uVar3 = (param_3 + 0x1e);
    uVar1 = (param_3 + 0xc);
    if (*(long *)(uVar1 + 0x20) == lVar4) {
      pass1_1018_028c((param_3 + 0xc),(param_3 + 0x1c),
                      lVar4,uVar3,param_1);
      (param_3 + 0x2c) = lVar4;
      (param_3 + 0x2e) = uVar3;
      pass1_1010_1f62(param_1,param_3 & 0xffff0000 | (param_3 - 0x20),param_4)
      ;
      return;
    }
    break;
  case 0x14:
    uVar1 = (param_3 + 0xc);
    if (*(long *)(uVar1 + 0x20) != *(long *)(param_3 + 0x1c)) {
      post_win_msg_1020_291a(0x1020);
      return;
    }
    break;
  case 0x15:
    uVar3 = pass1_1010_65d0(param_1,(param_3 + 0x7e),0x88);
    if (uVar3 != 0x0) {
      (param_3 + 0x88) = 0x1;
      return;
    }
  }
  return;
}



fn pass1_1018_0d76(param_1: u32)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x2c);
  if (*(long *)(uVar1 + 0x20) == *(long *)(param_1 + 0x3c)) {
    return;
  }
  return;
}



fn pass1_1018_0d9a(param_1: u32,param_2: *mut u16,param_3: *mut u32)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x7c);
  *param_3 = (uVar1 + 0x16);
  uVar1 = (param_1 + 0x7c);
  *param_2 = (uVar1 + 0x1a);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_0dc6(param_1: u32,param_2: u16)
{
  let puVar1: *mut u16;
  let puVar2: *mut u8;
  let uVar3: u16;
  let iVar4: i16;
  let puVar5: u32;
  astruct_18 *paVar6;
  let in_DX: *mut u8
  let puVar7: *mut u8
  let puVar8: *mut u8
  let uVar9: u16;
  astruct_91 *iVar13;
  let uVar10: u16;
  let local_32: u32;
  let uStack46: u16;
  let uStack44: u32;
  astruct_18 *paStack40;
  astruct_18 *paStack36;
  astruct_18 *paStack32;
  let uStack28: u32;
  let uStack24: u32;
  let local_14: [u8;8];
  let uStack12: u16;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u16;
  let iStack4: i16;
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x400);
  uVar10 = (param_1 >> 0x10);
  iVar13 = (astruct_91 *)param_1;
  paStack36 = (astruct_18 *)iVar13->field_0x94;
  fn_ptr_1000_17ce(paStack36,0x1000);
  paStack40 = (astruct_18 *)iVar13->field_0x9a;
  paStack32 = paStack40;
  fn_ptr_1000_17ce(paStack40,0x1000);
  iVar13->field_0x94 = 0x0;
  iVar13->field_0x9a = 0x0;
  iVar13->field_0x92 = 0x0;
  iVar13->field_0x98 = 0x0;
  while( true ) {
    puVar2 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar2));
    uStack24 = CONCAT22(in_DX,puVar2);
    puVar7 = (in_DX | puVar2);
    if (puVar7 == 0x0) break;
    paVar6 = *(astruct_18 **)(puVar2 + 0x200);
    paStack40 = paVar6;
    if (paVar6 == (astruct_18 *)0x8000001) {
      puVar1 = &iVar13->field_0x92;
      *puVar1 = *puVar1 + 0x1;
      in_DX = puVar7;
    }
    else {
      if ((iVar13->field_0xa8 != 0x0) ||
         (pass1_1008_dfa6(iVar13->field_0xa2,*(long *)(puVar2 + 0x4),0x4000001,param_2),
         in_DX = puVar7, paVar6 != 0x0)) {
        in_DX = puVar7;
        puVar1 = &iVar13->field_0x98;
        *puVar1 = *puVar1 + 0x1;
      }
    }
  }
  puVar8 = puVar7;
  if (iVar13->field_0x92 != 0x0) {
    uVar9 = iVar13->field_0x92;
    uStack44 = uStack44 & 0xffff0000 | uVar9;
    uVar3 = uVar9 * 0x6;
    mem_op_1000_179c(uVar3,0x0,0x1000);
    paStack32 = (astruct_18 *)CONCAT22(puVar7,uVar3);
    puVar8 = (puVar7 | uVar3);
    if (puVar8 == 0x0) {
      iVar13->field_0x94 = 0x0;
    }
    else {
      pass1_1000_5586((uchar *)0x3e38,0x1008,uStack44,0x6,uVar3,puVar7);
      iVar13->field_0x94 = paStack32;
    }
  }
  if (iVar13->field_0x98 != 0x0) {
    uVar9 = iVar13->field_0x98;
    uStack44 = uStack44 & 0xffff0000 | uVar9;
    uVar3 = uVar9 * 0x6;
    mem_op_1000_179c(uVar3,puVar8,0x1000);
    paStack32 = (astruct_18 *)CONCAT22(puVar8,uVar3);
    if ((puVar8 | uVar3) == 0x0) {
      iVar13->field_0x9a = 0x0;
    }
    else {
      pass1_1000_5586((uchar *)0x3e38,0x1008,uStack44,0x6,uVar3,puVar8);
      iVar13->field_0x9a = paStack32;
    }
  }
  if (iStack4 != 0x0) {
    uStack8 = 0x1;
    uStack6 = 0x0;
  }
  uStack28 = 0x0;
  uStack12 = uStack8;
  uStack10 = uStack6;
LAB_1018_0f74:
  puVar2 = local_14;
  pass1_1028_e4ec(CONCAT22(param_2,puVar2));
  uStack24 = CONCAT22(uStack6,puVar2);
  uVar9 = uStack6 | puVar2;
  if (uVar9 == 0x0) {
    return;
  }
  uStack44 = (puVar2 + 0x200);
  paVar6 = *(astruct_18 **)(puVar2 + 0x10);
  paStack40 = paVar6;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,paVar6,(paVar6 >> 0x10));
  paStack36 = (astruct_18 *)(paVar6 & 0xffff | uVar9 << 0x10);
  local_32 = (paVar6 + 0xc);
  uStack46 = (paVar6 + 0x10);
  puVar5 = &local_32;
  if (uStack44 != 0x8000001) goto LAB_1018_0ffc;
  iVar4 = &iVar13->field_0x94;
  uStack6 = (&iVar13->field_0x94 + 0x2);
  uStack28 = uStack28 & 0xffff | (uStack28._2_2_ + 0x1) << 0x10;
  goto LAB_1018_0fe8;
LAB_1018_0ffc:
  if ((iVar13->field_0xa8 != 0x0) ||
     (pass1_1008_dfa6(iVar13->field_0xa2,*(long *)(uStack24 + 0x4),0x4000001,param_2)
     , uStack6 = uVar9, puVar5 != 0x0)) {
    iVar4 = &iVar13->field_0x9a;
    uStack6 = (&iVar13->field_0x9a + 0x2);
    uStack28 = uStack28 & 0xffff0000 | (uStack28 + 0x1);
    uStack28._2_2_ = uStack28;
LAB_1018_0fe8:
    pass1_1008_3f62((u16 *)CONCAT22(uStack6,iVar4 + uStack28._2_2_ * 0x6),
                    CONCAT22(param_2,&local_32));
  }
  goto LAB_1018_0f74;
}



fn pass1_1018_1054(param_1: u32,param_2: *mut u16,param_3: *mut u32,param_4: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (*(long *)(iVar1 + 0x94) == 0x0) {
    pass1_1018_0dc6(param_1 & 0xffff | uVar2 << 0x10,param_4);
  }
  *param_3 = (iVar1 + 0x94);
  *param_2 = (iVar1 + 0x92);
  return;
}



fn pass1_1018_108c(param_1: u32,param_2: *mut u16,param_3: *mut u32,param_4: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (*(long *)(iVar1 + 0x9a) == 0x0) {
    pass1_1018_0dc6(param_1 & 0xffff | uVar2 << 0x10,param_4);
  }
  *param_3 = (iVar1 + 0x9a);
  *param_2 = (iVar1 + 0x98);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_10c4(param_1: u16,param_2: u16,param_3: u32)
{
  let uVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let iVar4: i16;
  let puVar5: *mut u8;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u32;
  let uVar9: u16;
  let puVar10: *mut u8
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let extraout_DX_02: u16;
  let iVar11: i16;
  let uVar12: u16;
  let uVar13: u8;
  let bVar14: bool;
  let puVar15: u32;
  let uStack60: u32;
  let uStack56: u32;
  let uStack52: u32;
  let puStack48: u32;
  let puStack40: u32;
  let uStack30: u16;
  let uStack28: u16;
  let local_16: [u8;8];
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u16;
  let uStack8: u16;
  let iStack6: i16;
  let iStack4: i16;
  
  uVar12 = (param_3 >> 0x10);
  iVar11 = param_3;
  iStack4 = (iVar11 + 0x86);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar11 + 0x88),0x1000);
  (iVar11 + 0x86) = 0x0;
  (iVar11 + 0x88) = 0x0;
  pass1_1028_dc52((astruct_92 *)
                  CONCAT13((char)(param_1 >> 0x8),CONCAT12((char)param_1,local_16)),0x1,
                  0x0,0x400);
  uStack30 = 0x0;
  uStack28 = 0x0;
  while( true ) {
    uVar9 = param_2;
    puVar5 = local_16;
    pass1_1028_e4ec(CONCAT22(param_1,puVar5));
    param_2 = uVar9 | puVar5;
    if (param_2 == 0x0) break;
    if (*(long *)(iVar11 + 0x3c) == *(long *)(puVar5 + 0x8)) {
      puVar15 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
      puVar10 = (puVar15 >> 0x10);
      uVar6 = puVar15;
      pass1_1038_4e78(uVar6,puVar10,CONCAT22(uVar9,puVar5),puVar15);
      puStack48 = CONCAT22(puVar10,uVar6);
      uVar3 = *puStack48;
      ppcVar2 = (code **)uVar3 + 0x8;
      uVar9 = uVar6;
      (**ppcVar2)(&PTR_LOOP_1050_1038,uVar6,puVar10);
      bVar14 = CARRY2(uStack30,uVar9);
      uStack30 += uVar9;
      uStack28 = uStack28 + extraout_DX + bVar14;
      param_2 = extraout_DX;
      if (puStack48 != 0x0) {
        ppcVar2 = (code **)uVar3;
        (**ppcVar2)(0x38,uVar6,puVar10,0x1);
        param_2 = extraout_DX_00;
      }
    }
  }
  if ((uStack28 | uStack30) != 0x0) {
    (iVar11 + 0x86) = uStack30;
    uVar7 = uStack30 * 0x6;
    mem_op_1000_179c(uVar7,0x0,0x1000);
    uStack52 = CONCAT22(param_2,uVar7);
    if ((param_2 | uVar7) == 0x0) {
      (iVar11 + 0x88) = 0x0;
    }
    else {
      pass1_1000_5586((uchar *)0x3e38,0x1008,uStack30,0x6,uVar7,param_2);
      (iVar11 + 0x88) = uStack52;
    }
    if (iStack6 != 0x0) {
      uStack10 = 0x1;
      uStack8 = 0x0;
    }
    iVar4 = 0x0;
    uStack14 = uStack10;
    uStack12 = uStack8;
    while( true ) {
      uVar9 = uStack8;
      puVar5 = local_16;
      pass1_1028_e4ec(CONCAT22(param_1,puVar5));
      if ((uVar9 | puVar5) == 0x0) break;
      uStack8 = uVar9 | puVar5;
      if (*(long *)(iVar11 + 0x3c) == *(long *)(puVar5 + 0x8)) {
        puVar15 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
        puVar10 = (puVar15 >> 0x10);
        uVar6 = puVar15;
        uVar13 = 0x38;
        pass1_1038_4e78(uVar6,puVar10,
                        CONCAT13((char)(uVar9 >> 0x8),CONCAT12((char)uVar9,puVar5)),
                        puVar15);
        puStack40 = CONCAT22(puVar10,uVar6);
        ppcVar2 = (code **)(*puStack40 + 0x10);
        uVar9 = uVar6;
        (**ppcVar2)(0x38,uVar6,puVar10);
        uStack56 = CONCAT22(extraout_DX_01,uVar9);
        uStack8 = extraout_DX_01;
        for (uStack60 = 0x0; uStack60 < uStack56; uStack60 += 0x1) {
          uVar8 = uStack56;
          pass1_1030_1d58(puStack40);
          uVar1 = (iVar11 + 0x88);
          uVar13 = 0x8;
          pass1_1008_3f62((u16 *)
                          (uVar1 & 0xff000000 |
                          CONCAT12((char)(uVar1 >> 0x10),uVar1 + iVar4 * 0x6))
                          ,CONCAT22(uStack8,uVar8 + 0xc));
          iVar4 += 0x1;
        }
        if (puStack40 != 0x0) {
          ppcVar2 = (code **)*puStack40;
          (**ppcVar2)(uVar13,uVar6,puVar10,0x1);
          uStack8 = extraout_DX_02;
        }
      }
    }
    if ((iVar11 + 0x86) != iStack4) {
      pass1_1010_1f62(param_1,param_3,0x6);
    }
  }
  return;
}



fn pass1_1018_1320(param_1: u32,param_2: *mut u16,param_3: *mut u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_3 = (param_1 + 0x88);
  *param_2 = (param_1 + 0x86);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_1346(param_1: u16,param_2: u16,astruct_93 *param_3)
{
  code **ppcVar1;
  let iVar2: i16;
  let puVar3: *mut u8;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: *mut u8
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let uVar7: u16;
  let extraout_DX_02: u16;
  astruct_93 *iVar9;
  let uVar8: u16;
  let uVar9: u8;
  let puVar10: u32;
  let uVar11: u32;
  let uVar12: u32;
  let uStack70: u32;
  let puStack56: u32;
  let uStack52: u32;
  let puStack48: u32;
  let uStack30: u32;
  let local_16: [u8;8];
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u16;
  let uStack8: u16;
  let iStack6: i16;
  let uStack4: u16;
  
  uVar8 = (param_3 >> 0x10);
  iVar9 = (astruct_93 *)param_3;
  uStack4 = iVar9->field_0x8c;
  fn_ptr_1000_17ce((astruct_18 *)iVar9->field_0x8e,0x1000);
  iVar9->field_0x8c = 0x0;
  iVar9->field_0x8e = 0x0;
  pass1_1028_dc52((astruct_92 *)
                  CONCAT13((char)(param_1 >> 0x8),CONCAT12((char)param_1,local_16)),0x1,
                  0x0,0x400);
  uStack30 = 0x0;
  while( true ) {
    uVar7 = param_2;
    puVar3 = local_16;
    pass1_1028_e4ec(CONCAT22(param_1,puVar3));
    param_2 = uVar7 | puVar3;
    if (param_2 == 0x0) break;
    if (iVar9->field_0x3c == *(long *)(puVar3 + 0x8)) {
      puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
      puVar6 = (puVar10 >> 0x10);
      uVar4 = puVar10;
      uVar9 = 0x38;
      pass1_1038_4e78(uVar4,puVar6,CONCAT22(uVar7,puVar3),puVar10);
      puStack48 = CONCAT22(puVar6,uVar4);
      ppcVar1 = (code **)(*puStack48 + 0x10);
      uVar7 = uVar4;
      (**ppcVar1)(&PTR_LOOP_1050_1038,uVar4,puVar6);
      uStack52 = CONCAT22(extraout_DX,uVar7);
      param_2 = extraout_DX;
      for (puStack56 = 0x0; puStack56 < uStack52;
          puStack56 = ((long)puStack56 + 0x1)) {
        uVar9 = 0x30;
        uVar11 = pass1_1030_1d7c(uVar7,param_2,puStack48);
        param_2 = (uVar11 >> 0x10);
        if ((uVar11 + 0x12) == 0x9) {
          uStack30 += 0x1;
        }
      }
      if (puStack48 != 0x0) {
        ppcVar1 = (code **)*puStack48;
        (**ppcVar1)(uVar9,uVar4,puVar6,0x1);
        param_2 = extraout_DX_00;
      }
    }
  }
  if ((uStack30._2_2_ | uStack30) != 0x0) {
    iVar9->field_0x8c = uStack30;
    uVar5 = uStack30 * 0x6;
    mem_op_1000_179c(uVar5,0x0,0x1000);
    uStack70 = CONCAT22(param_2,uVar5);
    if ((param_2 | uVar5) == 0x0) {
      iVar9->field_0x8e = 0x0;
    }
    else {
      pass1_1000_5586((uchar *)0x3e38,0x1008,uStack30,0x6,uVar5,param_2);
      iVar9->field_0x8e = uStack70;
    }
    if (iStack6 != 0x0) {
      uStack10 = 0x1;
      uStack8 = 0x0;
    }
    iVar2 = 0x0;
    uStack14 = uStack10;
    uStack12 = uStack8;
    while( true ) {
      uVar7 = uStack8;
      puVar3 = local_16;
      pass1_1028_e4ec(CONCAT22(param_1,puVar3));
      if ((uVar7 | puVar3) == 0x0) break;
      uStack8 = uVar7 | puVar3;
      if (iVar9->field_0x3c == *(long *)(puVar3 + 0x8)) {
        puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
        puVar6 = (puVar10 >> 0x10);
        uVar4 = puVar10;
        uVar9 = 0x38;
        pass1_1038_4e78(uVar4,puVar6,
                        CONCAT13((char)(uVar7 >> 0x8),CONCAT12((char)uVar7,puVar3)),
                        puVar10);
        puStack56 = CONCAT22(puVar6,uVar4);
        ppcVar1 = (code **)(*puStack56 + 0x10);
        uVar7 = uVar4;
        (**ppcVar1)(0x38,uVar4,puVar6);
        uStack52 = CONCAT22(extraout_DX_01,uVar7);
        uStack8 = extraout_DX_01;
        for (puStack48 = 0x0; puStack48 < uStack52;
            puStack48 = ((long)puStack48 + 0x1)) {
          uVar11 = uStack52;
          pass1_1030_1d58(puStack56);
          uVar9 = 0x30;
          uVar12 = struct_op_1030_73a8(uVar11 & 0xffff | uStack8 << 0x10);
          uVar7 = (uVar12 >> 0x10);
          if ((uVar12 + 0x12) == 0x9) {
            uVar12 = iVar9->field_0x8e;
            uVar9 = 0x8;
            pass1_1008_3f62((u16 *)
                            (uVar12 & 0xff000000 |
                            CONCAT12((char)(uVar12 >> 0x10),
                                            uVar12 + iVar2 * 0x6)),
                            CONCAT22(uStack8,uVar11 + 0xc));
            iVar2 += 0x1;
          }
          uStack8 = uVar7;
        }
        if (puStack56 != 0x0) {
          ppcVar1 = (code **)*puStack56;
          (**ppcVar1)(uVar9,uVar4,puVar6,0x1);
          uStack8 = extraout_DX_02;
        }
      }
    }
    if (iVar9->field_0x8c != uStack4) {
      pass1_1010_1f62(param_1,param_3,0x6);
    }
  }
  return;
}



fn pass1_1018_15f6(param_1: u32,param_2: *mut u16,param_3: *mut u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_3 = (param_1 + 0x8e);
  *param_2 = (param_1 + 0x8c);
  return;
}



fn pass1_1018_161c(param_1: u16,param_2: u32,param_3: *mut u16,param_4: i16,param_5: i16)
{
  let uVar1: u16;
  let uVar2: u16;
  let local_6: u32;
  
  pass1_1008_3e94((u16 *)(param_2 & 0xffff0000 | (param_2 + 0x36)),
                  CONCAT22(param_1,&local_6),
                  CONCAT22(param_1,&local_6 + 0x2));
  uVar1 = local_6._2_2_ + param_5 + -0x3;
  uVar2 = local_6 + param_4 + -0x3;
  local_6 = CONCAT22(uVar1,uVar2);
  pass1_1008_3e76(param_3,(param_2 + 0x44),uVar2,uVar1);
  return;
}



fn pass1_1018_1662(param_1: u32,param_2: i16,param_3: i16,param_4: u16)
{
  let local_6: i16;
  let local_4: i16;
  
  pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (param_1 + 0x36)),
                  CONCAT22(param_4,&local_6),
                  CONCAT22(param_4,&local_4));
  pass1_1018_16b8(param_1,(param_1 + 0x44),
                  CONCAT22(local_4 + param_3,local_6 + param_2),param_4);
  return;
}



fn pass1_1018_169e(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  pass1_1018_16b8(param_1 & 0xffff | uVar1 << 0x10,(param_1 + 0x44)
                  ,param_2,param_3);
  return;
}



// WARNING: Unable to use type for symbol uVar2

fn pass1_1018_16b8(param_1: u32,param_2: u16,param_3: u32,param_4: u16)
{
  let iVar1: i16;
  let uVar3: u32;
  let lVar4: i32;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let local_6: [u8;2];
  let local_4: [u8;2];
  let uVar2: u32;
  
  if (param_3._2_2_ + -0x3 < 0x1) {
    param_3 = CONCAT22(0x3,param_3);
  }
  if (param_3 + -0x3 < 0x1) {
    param_3 = CONCAT22(param_3._2_2_,0x3);
  }
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  uVar2 = (iVar6 + 0x5a);
  iVar1 = (uVar2 + 0x4);
  if (iVar1 <= param_3._2_2_ + 0x2) {
    param_3 = param_3 & 0xffff | (iVar1 - 0x3) << 0x10;
  }
  uVar3 = (iVar6 + 0x5a);
  iVar1 = (uVar3 + 0x8);
  if (iVar1 <= param_3 + 0x2) {
    param_3 = param_3 & 0xffff0000 | (iVar1 - 0x3);
  }
  uVar8 = (param_3 >> 0x10);
  pass1_1008_3e76((u16 *)(param_1 & 0xffff0000 | (iVar6 + 0x30)),param_2,
                  param_3,uVar8);
  uVar5 = uVar7;
  pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (iVar6 + 0x36U)),
                  CONCAT22(param_4,local_6),CONCAT22(param_4,local_4))
  ;
  pass1_1008_3e76((u16 *)(param_1 & 0xffff0000 | (iVar6 + 0x36U)),0x0,
                  param_3,uVar8);
  (iVar6 + 0x4c) = 0x0;
  lVar4 = *(long *)(iVar6 + 0x3c);
  uVar3 = (iVar6 + 0x2c);
  if (*(long *)(uVar3 + 0x20) == lVar4) {
    pass1_1018_028c((iVar6 + 0x2c),(iVar6 + 0x3c),lVar4,uVar5,
                    param_4);
    (iVar6 + 0x4c) = lVar4;
    (iVar6 + 0x4e) = uVar5;
    pass1_1010_1f62(param_4,param_1,0x4);
  }
  return;
}



fn pass1_1018_179e(param_1: u32,param_2: u32,param_3: u16,param_4: u16)
{
  let local_8: u16;
  let local_6: u32;
  
  pass1_1008_3eb4((u16 *)param_2,CONCAT22(param_4,&local_8),
                  CONCAT22(param_4,&local_6),
                  CONCAT22(param_4,&local_6 + 0x2));
  pass1_1018_16b8(param_1,local_8,local_6,param_4);
  return;
}



fn pass1_1018_17ce(param_1: u32,param_2: u32,param_3: u32,param_4: u16,param_5: u8)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  pass1_1018_0412((param_1 + 0x2c),param_2,
                  CONCAT22(param_3,(param_2 >> 0x10)),(param_3 >> 0x10),
                  (param_1 + 0x3c),param_4,param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16  pass1_1018_17f0(void)

{
  let iStack4: i16;
  
  iStack4 = 0x0;
  while ((iStack4 < 0x4 && ((iStack4 * 0x2 + _PTR_LOOP_1050_3962) != 0x0))) {
    iStack4 += 0x1;
  }
  return iStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_181c(param_1: u32,char *param_2,param_3: u8,param_4: u16)
{
  let in_AH: u8;
  let uVar1: u16;
  
  uVar1 = CONCAT11(in_AH,param_3);
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,(param_1 + 0x3c));
  pass1_1030_5b6c(CONCAT22(param_4,uVar1),param_2,param_4);
  return;
}



fn pass1_1018_1842(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0x20));
  pass1_1018_078e(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_18b8(param_1: u16,astruct_55 *param_2,param_3: u16)
{
  let puVar1: *mut u8
  astruct_55 *iVar3;
  let unaff_DI: i16;
  astruct_55 *uVar3;
  let puVar2: *mut u16;
  astruct_43 *paVar3;
  let uVar4: u32;
  let piVar5: *mut i16;
  let uVar6: u16;
  let piVar7: *mut i16;
  let uVar8: u16;
  let local_6: i16;
  let local_4: i16;
  let uVar1: u16;
  
  get_sys_metrics_1018_4b1e(param_2,0x0,param_3);
  uVar3 = (astruct_55 *)(param_2 >> 0x10);
  iVar3 = (astruct_55 *)param_2;
  iVar3->field_0x20 = 0x389a;
  iVar3->field_0x22 = 0x1008;
  iVar3->field_0x20 = 0x3aa8;
  iVar3->field_0x22 = 0x1008;
  &iVar3->field_0x24 = 0x0;
  iVar3->field_0x28 = 0x4;
  puVar2 = pass1_1008_3e38((u16 *)(param_2 & 0xffff0000 | ZEXT24(iVar3 + 0x1)));
  puVar1 = (puVar2 >> 0x10);
  &iVar3[0x1].field_0x6 = 0x0;
  iVar3[0x1].field_0xa = 0x0;
  &iVar3[0x1].field_0xc = 0x0;
  iVar3[0x1].field_0x10 = 0x0;
  iVar3[0x1].field_0x1c = 0x0;
  param_2->field_0x0 = 0x1fb0;
  iVar3->field_0x2 = 0x1018;
  iVar3->field_0x20 = 0x1fec;
  iVar3->field_0x22 = 0x1018;
  pass1_1000_4906((astruct_20 *)
                  (param_2 & 0xffff0000 | &iVar3[0x1].field_0x14),
                  (WNDCLASS16 *)0x0,0x8);
  piVar7 = &local_4;
  piVar5 = &local_6;
  uVar6 = param_1;
  uVar8 = param_1;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_1,puVar1,unaff_DI);
  pass1_1008_3e94((u16 *)(puVar2 & 0xffff0000 | (puVar2 + 0xe)),
                  CONCAT22(uVar6,piVar5),CONCAT22(uVar8,piVar7));
  paVar3 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x9a,param_1);
  iVar3->field_0x24 = paVar3;
  &iVar3->field_0x26 = (paVar3 >> 0x10);
  uVar4 = pass1_1008_4772((astruct_76 *)
                          (paVar3 & 0xffff0000 | iVar3->field_0x24));
  uVar1 = (uVar4 >> 0x10);
  pass1_1000_4906((astruct_20 *)
                  (param_2 & 0xffff0000 | &iVar3->field_0x32),
                  (WNDCLASS16 *)0x0,0x8);
  iVar3->field_0x36 = (uVar4 + 0x4);
  iVar3->field_0x38 = (uVar4 + 0x8);
  iVar3->field_0x2a = local_4 + 0x14;
  iVar3->field_0x2c = local_6 + 0x14;
  get_sys_metrics_1018_1ea0(param_2,0x1000);
  pass1_1008_3e76((u16 *)(param_2 & 0xffff0000 | ZEXT24(iVar3 + 0x1)),0x0,0x88,
                  0x99);
  return;
}



fn pass1_1018_1a04(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let puVar4: *mut u16;
  astruct_501 *iVar5;
  let uVar5: u16;
  let puStack14: *mut u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_501 *)param_1;
  *param_1 = 0x1fb0;
  iVar5->field_0x2 = 0x1018;
  iVar5->field_0x20 = 0x1fec;
  iVar5->field_0x22 = 0x1018;
  puVar1 = iVar5->field_0x24;
  uVar2 = iVar5->field_0x26;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x40,0x1000);
  if (param_1 == 0x0) {
    puVar4 = 0x0;
    uVar5 = 0x0;
  }
  else {
    puVar4 = &iVar5->field_0x20;
  }
  puStack14 = CONCAT22(uVar5,puVar4);
  *puStack14 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_1a8e(param_1: u32,uchar *param_2,param_3: i16,param_4: u16)
{
  let lVar1: i32;
  astruct_653 *iVar2;
  let uVar2: u16;
  let puVar3: *mut u16;
  let piVar4: *mut i16;
  let local_8: i16;
  let uStack6: u32;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_653 *)param_1;
  if (iVar2->field_0x44 != 0x0) {
    if (iVar2->field_0x46 != 0x0) {
      lVar1 = iVar2->field_0x46;
      (lVar1 + 0xe) = 0x0;
      iVar2->field_0x46 = 0x0;
    }
    piVar4 = &iVar2->field_0x4a;
    *piVar4 = *piVar4 + 0x1;
    return;
  }
  piVar4 = (i16 *)CONCAT22(param_4,&local_8);
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,param_2,param_3);
  pass1_1010_bf1e(puVar3,piVar4,puVar3,(puVar3 >> 0x10),
                  param_4);
  iVar2->field_0x44 = local_8;
  iVar2->field_0x40 = uStack6;
  pass1_1018_1ce8(param_4,param_1 & 0xffff | uVar2 << 0x10);
  return;
}



fn pass1_1018_1b02(param_1: u16,param_2: u32,param_3: i16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let uVar3: u32;
  astruct_96 *uVar4;
  astruct_95 *iVar5;
  let uVar5: u16;
  let iStack12: i16;
  let local_6: u16;
  let local_4: [u8;2];
  
  iStack12 = 0x0;
  while( true ) {
    uVar5 = (param_2 >> 0x10);
    iVar5 = (astruct_95 *)param_2;
    piVar1 = &iVar5->field_0x44;
    if (*piVar1 == iStack12 || *piVar1 < iStack12) break;
    uVar2 = iVar5->field_0x40;
    uVar4 = (astruct_96 *)uVar2;
    uVar4 = uVar4 + iStack12;
    uVar2 &= 0xffff0000;
    uVar3 = ZEXT24(uVar4);
    piVar1 = &uVar4->field_0x6;
    *piVar1 = *piVar1 + param_3 * 0x2 + -0x1;
    uVar5 = (uVar2 >> 0x10);
    if (0x23 < uVar4->field_0x6) {
      uVar4->field_0x6 = 0x0;
    }
    if (uVar4->field_0x6 < 0x0) {
      uVar4->field_0x6 = 0x23;
    }
    pass1_1008_3f62((u16 *)(uVar2 | &uVar4->field_0x10),(uVar2 | uVar3)
                   );
    uVar4->field_0x16 = uVar4->field_0xa;
    pass1_1008_3e94((u16 *)(uVar2 | uVar3),CONCAT22(param_1,&local_6),
                    CONCAT22(param_1,local_4));
    pass1_1008_3e76((u16 *)(uVar2 | uVar3),0x0,local_6,
                    
                     ((uVar4->field_0x8 * 0x24 + uVar4->field_0x6) * 0x2 + 0x3c20));
    uVar4->field_0xa = (uVar4->field_0x6 * 0x2 + 0x3966);
    iStack12 += 0x1;
  }
  pass1_1010_1f62(param_1,param_2,0xd);
  return;
}


fn pass1_1018_1c9a(param_1: u32,param_2: i16) -> u16

{
  let piVar1: *mut i16;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let unaff_SS: u16;
  let uStack10: i16;
  
  iStack10 = 0x0;
  while( true ) {
    uVar4 = (param_1 >> 0x10);
    piVar1 = (i16 *)(param_1 + 0x44);
    if (*piVar1 == iStack10 || *piVar1 < iStack10) {
      return 0x0;
    }
    uVar2 = (param_1 + 0x40);
    uVar3 = uVar2 + iStack10 * 0x18;
    if (((uVar3 + 0xc) * 0x1e + 0x3c32) == param_2) break;
    iStack10 += 0x1;
  }
  pass1_1018_1eda(param_1,uVar2 & 0xffff0000 | uVar3,unaff_SS);
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1018_1ce8(param_1: u16,param_2: u32)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let iStack26: i16;
  let local_18: [u8;2];
  let local_16: [u8;2];
  let iStack20: i16;
  let iStack18: i16;
  let iStack16: i16;
  let local_e: u16;
  let local_c: i16;
  let local_a: i16;
  let iStack8: i16;
  let uStack6: u32;
  
  uVar5 = (param_2 >> 0x10);
  iVar3 = param_2;
  uStack6 = (iVar3 + 0x40);
  iStack8 = 0x0;
  do {
    piVar1 = (i16 *)(iVar3 + 0x44);
    if (*piVar1 == iStack8 || *piVar1 < iStack8) {
      return;
    }
    pass1_1008_3eb4((u16 *)
                    (uStack6 & 0xffff0000 | (iStack8 * 0x18 + uStack6)),
                    CONCAT22(param_1,&local_e),
                    CONCAT22(param_1,&local_c),
                    CONCAT22(param_1,&local_a));
    local_a /= 0xa;
    iStack16 = local_c % 0xa;
    if (iStack16 != 0x0) {
      if (iStack16 < 0x6) {
        local_c -= iStack16;
      }
      else {
        local_c += 0xa - iStack16;
      }
    }
    iStack18 = pass1_1000_49b2(local_e);
    iStack18 /= 0x5;
    if (0x14 < iStack18) {
      iStack18 = 0x14;
      iVar2 = pass1_1000_49b2(local_e);
      local_e = (local_e / iVar2) * 0x64;
    }
    iStack16 = pass1_1000_49b2(local_e);
    iStack16 %= 0x5;
    if (iStack16 != 0x0) {
      if (local_e < 0x0) {
        iVar2 = iStack16;
        if (0x2 < iStack16) {
          iVar2 = iStack16 + -0x5;
        }
        local_e += iVar2;
      }
      else {
        if (iStack16 < 0x3) {
          local_e -= iStack16;
        }
        else {
          local_e += 0x5 - iStack16;
        }
      }
    }
    iStack20 = (iStack18 * 0x48 + 0x3c20);
    if (local_c < iStack20) {
      for (iStack26 = iStack18; iStack26 < 0x15; iStack26 += 0x1) {
        piVar1 = (i16 *)(iStack26 * 0x48 + 0x3c20);
        if (*piVar1 == local_c || *piVar1 < local_c) {
          iStack18 = iStack26;
          break;
        }
      }
    }
    pass1_1008_3e94((u16 *)(param_2 & 0xffff0000 | (iVar3 + 0x3a)),
                    CONCAT22(param_1,local_18),
                    CONCAT22(param_1,local_16));
    uVar4 = iStack8 * 0x18 + uStack6;
    (uVar4 + 0x6) = local_a;
    (uVar4 + 0x8) = iStack18;
    pass1_1008_3e76((u16 *)(uStack6 & 0xffff0000 | uVar4),0x0,local_e,
                    ((iStack18 * 0x24 + local_a) * 0x2 + 0x3c20));
    (uVar4 + 0xa) = (local_a * 0x2 + 0x3966);
    iStack8 += 0x1;
  } while( true );
}



fn pass1_1018_1e78(param_1: u32,param_2: i16) -> u32

{
  let uVar1: u32;
  
  if (param_2 == -0x1) {
    uVar1 = (param_1 + 0x46);
    param_2 = (uVar1 + 0xc);
  }
  return CONCAT22(0x1050,param_2 * 0x1e + 0x3c18);
}


fn pass1_1018_1eda(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(long *)(iVar2 + 0x46) != 0x0) {
    uVar1 = (iVar2 + 0x46);
    (uVar1 + 0xe) = 0x2;
  }
  (iVar2 + 0x46) = param_2;
  (param_2 + 0xe) = 0x1;
  pass1_1010_1f62(param_3,param_1,0xd);
  return;
}



fn pass1_1018_1f1a(param_1: u32,param_2: i16) -> u16

{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u16;
  let iStack6: i16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0x56) == 0x0) {
    return 0x0;
  }
  iStack6 = 0x0;
  while( true ) {
    piVar1 = (i16 *)(iVar2 + 0x56);
    if (*piVar1 == iStack6 || *piVar1 < iStack6) {
      return 0x0;
    }
    if ((iVar2 + 0x4e + iStack6 * 0x2) == param_2) break;
    iStack6 += 0x1;
  }
  return 0x1;
}



astruct_18 *
pass1_1018_1f6a(param_1: u16,astruct_18 *param_2,param_3: u8,param_4: u16)

{
  param_2 = (astruct_18 *)(param_2 & 0xffff0000 | (param_2 - 0x20));
  pass1_1018_1a04((u16 *)param_2,param_4);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2,0x1000);
  }
  return param_2;
}



fn pass1_1018_1f7a(param_1: i16,param_2: u16) -> u32

{
  return CONCAT22(param_2,param_1 + 0x2a);
}



fn pass1_1018_1f8a(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1018_1a04(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_1ff4(astruct_634 *param_1,param_2: u16,param_3: u16) -> u32

{
  let piVar1: *mut i16;
  let unaff_DI: i16;
  let unaff_SS: u16;
  astruct_79 *paVar2;
  let piVar3: *mut i16;
  let piVar4: *mut i16;
  let uVar5: u16;
  let local_a: i16;
  let local_8: i16;
  let uStack6: u32;
  
  paVar2 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  &param_1->field_0xa = 0xb9010b;
  param_1->field_0xe = 0x170035;
  CONCAT22(param_2,param_1) = 0x21e8;
  param_1->field_0x2 = 0x1018;
  piVar4 = &local_8;
  piVar3 = &local_a;
  uVar5 = unaff_SS;
  uStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,
                            (paVar2 >> 0x10),unaff_DI);
  pass1_1008_3e94((u16 *)(uStack6 & 0xffff0000 | (uStack6 + 0xe)),
                  CONCAT22(unaff_SS,piVar3),CONCAT22(uVar5,piVar4));
  piVar1 = &param_1->field_0xa;
  *piVar1 = *piVar1 + local_8;
  piVar1 = &param_1->field_0xc;
  *piVar1 = *piVar1 + local_a;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,param_1 + 0x1),(WNDCLASS16 *)0x0,0x7f4);
  return CONCAT22(param_2,param_1);
}



fn pass1_1018_2076(param_1: *mut u16,param_2: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x21e8;
  (param_1 + 0x2) = 0x1018;
  pass1_1018_209c(param_1 & 0xffff | uVar1 << 0x10);
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1018_209c(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  do {
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1 + 0x12;
    puVar1 = (iVar4 + iStack4 * 0x4);
    uVar2 = (iVar4 + iStack4 * 0x4 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    (param_1 + iStack4 * 0x4 + 0x12) = 0x0;
    iStack4 += 0x1;
  } while (iStack4 < 0x1fd);
  return;
}



fn pass1_1018_20ee(param_1: u32,i16 *param_2)
{
  let BVar1: bool;
  let in_DX: u16;
  let uVar2: u16;
  
  BVar1 = pass1_1008_aed8(param_2);
  if (BVar1 == 0x0) {
    return;
  }
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + *param_2 * 0x4 + 0x12) == 0x0) {
    pass1_1018_216e(param_1 & 0xffff | uVar2 << 0x10,param_2,in_DX);
  }
  pass1_1008_ae26(param_2);
  return;
}



fn pass1_1018_214e(param_1: u16,param_2: u16,param_3: *mut u16,param_4: i16)
{
  pass1_1008_3e76(param_3,0x0,(param_4 * 0x4 + 0x3e90),
                  (param_4 * 0x4 + 0x3e8e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_216e(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let uStack8: u16;
  
  uStack8 = pass1_1008_adf2(param_2);
  uVar1 = pass1_1008_ae0c(param_2);
  for (; uStack8 <= uVar1; uStack8 += 0x1) {
    uVar2 = uVar1;
    pass1_1010_8018(_PTR_LOOP_1050_14cc,uStack8,param_3,0x1010);
    uVar3 = (param_1 >> 0x10);
    (param_1 + uStack8 * 0x4 + 0x12) = uVar2;
    *(uchar **)(param_1 + uStack8 * 0x4 + 0x14) = param_3;
  }
  return;
}



fn pass1_1018_21c2(param_1: u32,param_2: u8,param_3: u16) -> u32

{
  pass1_1018_2076((u16 *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_21f8(void) -> u16

{
  let puVar1: *mut u16;
  
  pass1_1008_3e54(&USHORT_1048_4210,0x0,0x195,0x1);
  pass1_1008_3e54(&USHORT_1050_65ca,0x0,0xe0,0x1b1);
  pass1_1008_3e54(&USHORT_1050_65d0,0x0,0x17a,0x72);
  pass1_1008_3e54(&USHORT_1050_65d6,0x0,0xde,0x93);
  pass1_1008_3e54(&USHORT_1050_65dc,0x0,0x177,0x1da);
  pass1_1008_3e54(&USHORT_1048_4216,0x0,0x195,0x21c);
  pass1_1008_3e54(&USHORT_1048_421c,0x0,0x1b6,0x22c);
  pass1_1008_3e54(&USHORT_1048_4222,0x0,0x109,0x5);
  puVar1 = pass1_1008_3e54(&USHORT_1048_4228,0x0,0xfd,0x1fd);
  return puVar1;
}


fn pass1_1018_2440(astruct_11 *param_1,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let piVar4: *mut i16;
  let uVar6: u16;
  astruct_502 *uVar5;
  let uVar7: u16;
  let puStack6: *mut u16;
  
  uVar7 = (param_1 >> 0x10);
  uVar5 = (astruct_502 *)param_1;
  param_1 = 0x2ada;
  uVar5->field_0x2 = 0x1018;
  uVar5->field_0x1c = (s_fem132_wav_1050_2aec + 0x6);
  uVar5->field_0x1e = 0x1018;
  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (param_1 == (astruct_11 *)0x0) {
      piVar4 = (i16 *)0x0;
      uVar6 = 0x0;
    }
    else {
      piVar4 = &uVar5->field_0x1c;
      uVar6 = uVar7;
    }
    param_2 = 0x1008;
    pass1_1008_92b2(_PTR_LOOP_1050_0388,0x73,CONCAT22(uVar6,piVar4));
  }
  puVar1 = uVar5->field_0x2a;
  uVar2 = uVar5->field_0x2c;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_2,puVar1,uVar2,0x1);
  }
  puVar1 = uVar5->field_0x6e;
  uVar2 = uVar5->field_0x70;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_2,puVar1,uVar2,0x1);
  }
  if (param_1 == (astruct_11 *)0x0) {
    piVar4 = (i16 *)0x0;
    uVar7 = 0x0;
  }
  else {
    piVar4 = &uVar5->field_0x1c;
  }
  puStack6 = CONCAT22(uVar7,piVar4);
  *puStack6 = 0x389a;
  piVar4[0x1] = 0x1008;
  clenaup_win_ui_1018_4d22(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_2504(param_1: u16,param_2: u16)
{
  let uVar1: u16;
  
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,0x4000001);
  if ((param_2 | param_1) != 0x0) {
    uVar1 = pass1_1028_d69e(**_PTR_LOOP_1050_5748);
    if (uVar1 == 0x0) {
      return;
    }
  }
  return;
}



fn pass1_1018_2548(param_1: u16,param_2: u16,param_3: *mut u16)
{
  pass1_1008_3f62(param_3,&USHORT_1048_4228);
  return;
}



fn pass1_1018_255e(param_1: u32) -> u16

{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x26) != 0x0) {
    uVar1 = (param_1 + 0x26);
    return (uVar1 + 0xa);
  }
  return 0x0;
}



uchar * 
pass1_1018_2580(param_1: u32,param_2: u16,param_3: u32,param_4: u16,param_5: u16,
               param_6: u8)

{
  let puVar1: *mut u8
  let iVar2: i16;
  let uVar3: u16;
  uchar local_8 [0x6];
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(long *)(iVar2 + 0x20) == 0x0) {
    return 0x6ad;
  }
  pass1_1008_3e38((u16 *)CONCAT22(param_5,local_8));
  pass1_1018_161c(param_5,(iVar2 + 0x20),CONCAT22(param_5,local_8),
                  param_3,(param_3 >> 0x10));
  puVar1 = local_8;
  pass1_1018_17ce((iVar2 + 0x20),CONCAT22(puVar1,param_2),
                  CONCAT22(param_4,param_5),param_5,param_6);
  return puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_25d2(param_1: u32,param_2: u16,param_3: u32,param_4: i16,param_5: u16) -> u16

{
  let uVar1: u16;
  let puVar2: *mut u8
  let uVar3: u16;
  let puVar4: *mut u16;
  let puVar5: *mut u16;
  let local_8: [u8;6];
  
  uVar3 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x20) == 0x0) {
    return 0x0;
  }
  puVar4 = pass1_1008_3e38((u16 *)CONCAT22(param_5,local_8));
  puVar2 = (puVar4 >> 0x10);
  pass1_1018_161c(param_5,(param_1 + 0x20),
                  CONCAT22(param_5,local_8),param_3,(param_3 >> 0x10))
  ;
  puVar5 = CONCAT22(param_5,local_8);
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_5,puVar2,param_4);
  uVar1 = puVar4;
  pass1_1010_71d6(puVar4,param_2,puVar5,uVar1,(puVar4 >> 0x10),param_5
                 );
  return uVar1;
}



fn pass1_1018_262e(param_1: u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x44) = 0x1;
  (param_1 + 0x3e) = 0x0;
  return;
}



fn pass1_1018_2646(param_1: u16,param_2: u16,param_3: *mut u16)
{
  pass1_1008_3f62(param_3,&USHORT_1048_4222);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_265c(void) -> u32

{
  let uVar1: u32;
  
  uVar1 = pass1_1030_8326();
  return uVar1;
}



fn pass1_1018_266a(param_1: u32) -> u16

{
  return (param_1 + 0x44);
}



fn pass1_1018_2678(param_1: u16,param_2: u16,param_3: *mut u16)
{
  pass1_1008_3f62(param_3,&USHORT_1048_4216);
  return;
}



fn pass1_1018_268e(param_1: u32) -> u32

{
  astruct_287 *iVar1;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar1 = (astruct_287 *)param_1;
  if (iVar1->field_0x42 != 0x0) {
    &iVar1->field_0x40 = 0x0;
    iVar1->field_0x44 = 0x1;
  }
  iVar2 = iVar1->field_0x3e * 0x4;
  return CONCAT22((&iVar1[0x1].field_0x2 + iVar2),
                  (&iVar1[0x1].field_0x0 + iVar2));
}



fn pass1_1018_26c2(param_1: u16,param_2: u16,param_3: *mut u16)
{
  pass1_1008_3f62(param_3,&USHORT_1048_421c);
  return;
}



fn pass1_1018_26d8(param_1: u16,param_2: u16,param_3: i16,param_4: *mut u16)
{
  pass1_1008_3f62(param_4,
                          CONCAT22(0x1050,&USHORT_1050_65ca + param_3 * 0x6));
  return;
}



fn pass1_1018_26f8(param_1: u16,param_2: u16,param_3: *mut u16)
{
  pass1_1008_3f62(param_3,&USHORT_1048_4210);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_270e(param_1: u32,param_2: i16,param_3: u16,uchar *param_4,param_5: i16,
               param_6: u16)

{
  code **ppcVar1;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  let extraout_DX: *mut u8
  astruct_655 *iVar5;
  let uVar5: u16;
  let puVar6: *mut u16;
  
  iVar5 = (astruct_655 *)param_1;
  uVar5 = (param_1 >> 0x10);
  if (param_2 == 0x0) {
    if ((iVar5->field_0x20 == 0x0) ||
       (uVar2 = iVar5->field_0x20, (uVar2 + 0x8) != param_3)) {
      puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,param_3,param_6,param_4,param_5);
      if (iVar5->field_0x20 != 0x0) {
        if (param_1 == 0x0) {
          iVar3 = 0x0;
          uVar4 = 0x0;
        }
        else {
          iVar3 = &iVar5->field_0x1c;
          uVar4 = uVar5;
        }
        pass1_1010_1ea6(iVar5->field_0x20,CONCAT22(uVar4,iVar3),param_6);
      }
      iVar5->field_0x20 = puVar6;
      if (param_1 == 0x0) {
        param_3 = 0x0;
        uVar4 = 0x0;
      }
      else {
        param_3 = &iVar5->field_0x1c;
        uVar4 = uVar5;
      }
      uVar2 = iVar5->field_0x20;
      ppcVar1 = (code **)(iVar5->field_0x20 + 0x4);
      (**ppcVar1)(0x1010,uVar2,(uVar2 >> 0x10),0x0,param_3,uVar4);
      param_4 = extraout_DX;
    }
    pass1_1018_2862(param_1);
    if ((param_4 | param_3) != 0x0) {
      iVar5->field_0x24 = 0x1;
    }
    pass1_1010_1f62(param_6,param_1,0x7);
  }
  else {
    if (((&iVar5->field_0x20 + 0x2) | &iVar5->field_0x20) != 0x0) {
      if (param_1 == 0x0) {
        iVar3 = 0x0;
        uVar4 = 0x0;
      }
      else {
        iVar3 = &iVar5->field_0x1c;
        uVar4 = uVar5;
      }
      pass1_1010_1ea6(iVar5->field_0x20,CONCAT22(uVar4,iVar3),param_6);
      iVar5->field_0x20 = 0x0;
      return;
    }
  }
  return;
}



fn pass1_1018_280c(param_1: u32)
{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0x24) == 0x0) {
    return;
  }
  (iVar2 + 0x24) = 0x0;
  if (*(long *)(iVar2 + 0x20) == 0x0) {
    (iVar2 + 0x26) = 0x0;
  }
  else {
    uVar1 = (iVar2 + 0x20);
    (iVar2 + 0x26) = (uVar1 + 0x4c);
  }
  return;
}



fn pass1_1018_2862(param_1: u32)
{
  let lVar1: i32;
  astruct_654 *iVar2;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_654 *)param_1;
  if (iVar2->field_0x20 == 0x0) {
    iVar2->field_0x26 = 0x0;
  }
  else {
    lVar1 = iVar2->field_0x20;
    iVar2->field_0x26 = (lVar1 + 0x4c);
  }
  return;
}



fn pass1_1018_289c(param_1: u32,param_2: i16,param_3: u16,param_4: u16)
{
  let uVar1: u16;
  
  if (param_2 == 0x1) {
    (param_1 + 0x4) = 0x0;
    return;
  }
  if (param_2 == 0x2) {
    pass1_1018_2922(param_1 & 0xffff0000 | (param_1 - 0x1c));
  }
  else {
    if ((((param_2 + -0x3 < 0x1) || (SBORROW2(param_2 + -0x3,0x1))) ||
        (0x1 < param_2 + -0x5)) || (*(long *)(param_1 + 0x4) == 0x0)) {
      return;
    }
    uVar1 = param_1 - 0x1c;
    pass1_1018_2862(param_1 & 0xffff0000 | uVar1);
    if ((param_3 | uVar1) != 0x0) {
      (param_1 + 0x8) = 0x1;
    }
  }
  pass1_1010_1f62(param_4,param_1 & 0xffff0000 | (param_1 - 0x1c),param_2);
  return;
}



fn pass1_1018_2922(param_1: u32)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (((iVar2 + 0x40) != 0x0) &&
     (piVar1 = (i16 *)(iVar2 + 0x3e), *piVar1 = *piVar1 + 0x1,
     0x9 < (iVar2 + 0x3e))) {
    (iVar2 + 0x3e) = 0x0;
    (iVar2 + 0x42) = 0x1;
  }
  return;
}


fn pass1_1018_2aa3(void)
{
  pass1_1018_21f8();
  return;
}



astruct_11 * 
pass1_1018_2aa8(astruct_11 *param_1,param_2: u8,param_3: u16)

{
  param_1 = (astruct_11 *)(param_1 & 0xffff0000 | (param_1 - 0x1c));
  pass1_1018_2440(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_2afa(param_1: *mut u32)
{
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  return;
}


fn pass1_1018_2c60(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let puVar4: *mut u16;
  let uVar5: u16;
  astruct_503 *uVar6;
  let uVar7: u16;
  let puStack6: *mut u16;
  
  uVar7 = (param_1 >> 0x10);
  uVar6 = (astruct_503 *)param_1;
  *param_1 = 0x32d8;
  uVar6->field_0x2 = 0x1018;
  uVar6->field_0x20 = 0x3314;
  uVar6->field_0x22 = 0x1018;
  if (uVar6->field_0x182 != 0x0) {
    if (param_1 == 0x0) {
      puVar4 = 0x0;
      uVar5 = 0x0;
    }
    else {
      puVar4 = &uVar6->field_0x20;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6(uVar6->field_0x182,CONCAT22(uVar5,puVar4),param_2);
  }
  fn_ptr_1000_17ce((astruct_18 *)uVar6->field_0x186,0x1000);
  puVar1 = uVar6->field_0x24;
  uVar2 = uVar6->field_0x26;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1000,puVar1,uVar2,0x1);
  }
  if (param_1 == 0x0) {
    puVar4 = 0x0;
    uVar7 = 0x0;
  }
  else {
    puVar4 = &uVar6->field_0x20;
  }
  puStack6 = CONCAT22(uVar7,puVar4);
  *puStack6 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1018_2d22(param_1: u32,i16 *param_2,param_3: *mut u16,param_4: i16)
{
  let uVar1: u32;
  
  *param_3 = 0x0;
  *param_2 = 0x0;
  uVar1 = pass1_1008_4772(*(astruct_76 **)(param_1 + 0x24));
  *param_2 = (uVar1 + 0x8) + -0x14;
  if (param_4 == 0xbb8) {
    *param_3 = 0x5;
  }
  if (param_4 == 0xbba) {
    *param_3 = 0x23;
  }
  if (param_4 == 0xbb9) {
    *param_3 = 0x75;
  }
  return;
}



fn pass1_1018_2d84(param_1: u16,param_2: u32)
{
  pass1_1018_2e28(param_2);
  pass1_1020_bd80(param_1);
  return;
}



fn pass1_1018_2d9a(param_1: u32)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar3 = (iVar4 + 0x180) | (iVar4 + 0x17e);
  if (uVar3 != 0x0) {
    piVar1 = (i16 *)(iVar4 + 0x174);
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
      uVar2 = (iVar4 + 0x17e);
      uVar3 = (uVar2 + 0xa) - 0x1;
      (iVar4 + 0x174) = uVar3;
    }
    pass1_1018_2e28(param_1);
    (iVar4 + 0x176) = uVar3;
  }
  return;
}



fn pass1_1018_2dde(param_1: u32)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let iVar3: i16;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (((iVar4 + 0x180) | (iVar4 + 0x17e)) != 0x0) {
    piVar1 = (i16 *)(iVar4 + 0x174);
    *piVar1 = *piVar1 + 0x1;
    iVar3 = (iVar4 + 0x174);
    uVar2 = (iVar4 + 0x17e);
    piVar1 = (i16 *)(uVar2 + 0xa);
    if (*piVar1 == iVar3 || *piVar1 < iVar3) {
      (iVar4 + 0x174) = 0x0;
    }
    pass1_1018_2e28(param_1);
    (iVar4 + 0x176) = iVar3;
  }
  return;
}



fn pass1_1018_2e28(param_1: u32)
{
  let lVar1: i32;
  let extraout_DX: u16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  lVar1 = (long)(param_1 + 0x174);
  empty_1008_8fc4((param_1 + 0x17e),lVar1);
  if ((extraout_DX | lVar1) != 0x0) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_2e5e(param_1: u16,param_2: u16,param_3: u16,param_4: u32)
{
  let uVar1: u16;
  let lVar1: i32;
  astruct_126 *iVar4;
  let uVar2: u16;
  
  uVar2 = (param_4 >> 0x10);
  iVar4 = (astruct_126 *)param_4;
  if (iVar4->field_0x17e == 0x0) {
    pass1_1030_82f0(param_1,_PTR_LOOP_1050_5748,iVar4->field_0x17a);
    &iVar4->field_0x17e = param_2;
    (&iVar4->field_0x17e + 0x2) = param_3;
  }
  if ((iVar4->field_0x17e != 0x0) &&
     (lVar1 = iVar4->field_0x17e, (lVar1 + 0xa) != 0x0)) {
    lVar1 = (long)iVar4->field_0x174;
    empty_1008_8fc4(iVar4->field_0x17e,lVar1);
    uVar1 = lVar1;
    pass1_1018_2e28(param_4);
    iVar4->field_0x176 = uVar1;
    return;
  }
  return;
}



fn pass1_1018_2ee4(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let cVar2: u8;
  let uVar3: u16;
  
  if (param_2 != 0x12) {
    if (param_2 < 0x13) {
      cVar2 = param_2;
      if (cVar2 == '\x01') {
        (param_1 + 0x162) = 0x0;
        return;
      }
      if (('\x02' < (cVar2 + -0x1)) && ((char)(cVar2 + -0x4) < '\x03'))
      goto LAB_1018_2f06;
    }
    return;
  }
  uVar1 = (param_1 + 0x162);
  (param_1 + 0x15a) = (uVar1 + 0x24);
LAB_1018_2f06:
  uVar3 = param_1 - 0x20;
  pass1_1018_31fa(param_1 & 0xffff0000 | uVar3,uVar3,param_1._2_2_,param_3);
  pass1_1010_1f62(param_3,param_1 & 0xffff0000 | uVar3,param_2);
  return;
}


fn pass1_1018_2fe8(param_1: u32,param_2: u16,param_3: u16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let iVar7: i16;
  let extraout_DX: u16;
  let uVar8: u16;
  let iVar9: i16;
  let uVar10: u16;
  
  uVar10 = (param_1 >> 0x10);
  iVar9 = param_1;
  uVar6 = (iVar9 + 0x174);
  uVar2 = (iVar9 + 0x17e);
  iVar7 = (uVar2 + 0xa);
  if (iVar7 != 0x0) {
    if (*(long *)(iVar9 + 0x186) != 0x0) {
      uVar3 = str_op_1000_3da4(*(char **)(iVar9 + 0x186));
      (iVar9 + 0x174) = 0x0;
      while( true ) {
        if (iVar7 <= (iVar9 + 0x174)) break;
        uVar4 = (iVar9 + 0x174);
        uVar2 = (iVar9 + 0x17e);
        empty_1008_8fc4(uVar2,(uVar2 >> 0x10),uVar4,uVar4 >> 0xf);
        uVar8 = extraout_DX;
        pass1_1018_2e28(param_1);
        uVar4 = pass1_1020_bd80(uVar4);
        uVar5 = pass1_1000_3de8(CONCAT22(uVar8,uVar4),*(char **)(iVar9 + 0x186),
                                uVar3,param_2,param_3);
        if (uVar5 == 0x0) break;
        piVar1 = (i16 *)(iVar9 + 0x174);
        *piVar1 = *piVar1 + 0x1;
      }
      if ((iVar9 + 0x174) < iVar7) {
        pass1_1018_2e28(param_1);
        (iVar9 + 0x176) = iVar7;
        return;
      }
      (iVar9 + 0x174) = uVar6;
      pass1_1018_2e28(param_1);
      (iVar9 + 0x176) = uVar6;
    }
  }
  return;
}



fn pass1_1018_30ca(param_1: u32,char *param_2,param_3: u16)
{
  let uVar1: u16;
  astruct_504 *iVar3;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar3 = (astruct_504 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x186,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x186 = uVar1;
  iVar3->field_0x188 = param_3;
  return;
}



fn pass1_1018_30fc(param_1: u32,u16 **param_2,uchar *param_3)
{
  let uVar1: u16;
  let uVar2: u32;
  let puVar3: *mut u16;
  let uVar4: u16;
  let uVar5: u16;
  let lVar6: i32;
  let puVar7: *mut u8
  let extraout_DX: u16;
  let uVar8: u16;
  let puStack18: u32;
  let iStack6: i16;
  
  *param_2 = 0x0;
  uVar8 = (param_1 >> 0x10);
  uVar2 = (param_1 + 0x17e);
  uVar1 = (uVar2 + 0xa);
  if (uVar1 != 0x0) {
    uVar4 = uVar1;
    mem_op_1000_179c(0x6,param_3,0x1000);
    puStack18 = CONCAT22(param_3,uVar4);
    puVar7 = (param_3 | uVar4);
    if (puVar7 == 0x0) {
      *param_2 = 0x0;
    }
    else {
      *puStack18 = 0x0;
      (uVar4 + 0x4) = 0x0;
      *param_2 = puStack18;
    }
    uVar5 = uVar1 * 0x2;
    mem_op_1000_179c(uVar5,puVar7,0x1000);
    puVar3 = *param_2;
    *puVar3 = uVar5;
    *(uchar **)(puVar3 + 0x2) = puVar7;
    (*param_2 + 0x4) = uVar1;
    for (iStack6 = 0x0; iStack6 < uVar1; iStack6 += 0x1) {
      lVar6 = (long)iStack6;
      empty_1008_8fc4((param_1 + 0x17e),lVar6);
      (*param_2 + iStack6 * 0x2) =
           (lVar6 + 0x2e);
    }
  }
  return;
}



fn pass1_1018_31d0(param_1: u32) -> u16

{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if ((*(long *)(param_1 + 0x17e) != 0x0) &&
     (uVar1 = (param_1 + 0x17e), *(long *)(uVar1 + 0xa) != 0x0))
  {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_31fa(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let iVar3: i16;
  let iVar4: i16;
  let lVar5: i32;
  let iVar6: i16;
  let uVar7: u16;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  pass1_1030_82f0(param_4,_PTR_LOOP_1050_5748,(iVar6 + 0x17a));
  (iVar6 + 0x17e) = param_2;
  (iVar6 + 0x180) = param_3;
  if (((param_3 | (iVar6 + 0x17e)) != 0x0) &&
     (uVar2 = (iVar6 + 0x17e), iVar4 = (uVar2 + 0xa),
     iVar4 != 0x0)) {
    (iVar6 + 0x174) = 0x0;
    while( true ) {
      if (iVar4 <= (iVar6 + 0x174)) break;
      lVar5 = (long)(iVar6 + 0x174);
      empty_1008_8fc4((iVar6 + 0x17e),lVar5);
      iVar3 = lVar5;
      pass1_1018_2e28(param_1);
      if ((iVar6 + 0x176) == iVar3) break;
      piVar1 = (i16 *)(iVar6 + 0x174);
      *piVar1 = *piVar1 + 0x1;
    }
    if (iVar4 <= (iVar6 + 0x174)) {
      (iVar6 + 0x174) = 0x0;
    }
    pass1_1018_2e28(param_1);
    (iVar6 + 0x176) = iVar4;
  }
  return;
}



fn pass1_1018_32a6(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0x20));
  pass1_1018_2c60(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_331c(astruct_638 *param_1,param_2: u16,param_3: u16,param_4: u16,
               uchar *param_5)

{
  let uVar1: u16;
  let unaff_DI: i16;
  let puVar2: *mut u16;
  
  pass1_1008_ca5a((astruct_639 *)param_1,param_2,param_3);
  &param_1->field_0x122 = 0x0;
  param_1->field_0x126 = 0x0;
  param_1->field_0x12a = 0x0;
  param_1->field_0x12e = 0x0;
  param_1->field_0x130 = 0x0;
  param_1->field_0x132 = 0x0;
  param_1->field_0x136 = 0x0;
  param_1->field_0x13a = 0x0;
  param_1->field_0x13c = 0x0;
  param_1->field_0x13e = 0x0;
  param_1->field_0x142 = 0x0;
  CONCAT22(param_2,param_1) = &PTR_LOOP_1050_470c;
  param_1->field_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_4,param_5,unaff_DI);
  uVar1 = puVar2;
  param_1->field_0x122 = uVar1;
  param_1->field_0x124 = (puVar2 >> 0x10);
  *&param_1->field_0x22 = 0x0;
  pass1_1008_612e(0x8,0xc,uVar1);
  param_1->field_0x13c = uVar1;
  return;
}



fn pass1_1018_33b4(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_505 *iVar5;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_505 *)param_1;
  *param_1 = &PTR_LOOP_1050_470c;
  iVar5->field_0x2 = 0x1018;
  puVar1 = iVar5->field_0x136;
  uVar2 = iVar5->field_0x138;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  &iVar5->field_0x136 = 0x0;
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x126,0x1000);
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x12a,0x1000);
  pass1_1008_caa0(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_3424(param_1: u32,param_2: i16,param_3: u16,param_4: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar1 = (iVar3 + 0x122);
  pass1_1008_e852(uVar1,(uVar1 >> 0x10),(iVar3 + 0x126),
                  param_4,param_3);
  uStack6 = CONCAT22(param_3,param_2);
  uVar1 = (iVar3 + 0x122);
  pass1_1008_e852(uVar1,(uVar1 >> 0x10),(iVar3 + 0x12a),
                  param_4,param_3);
  uStack10 = CONCAT22(param_3,param_2);
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,uStack6);
  uVar2 = param_3;
  iVar3 = param_2;
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,uStack10);
  if (*(long *)(iVar3 + 0x200) == *(long *)(param_2 + 0x200)) {
    return;
  }
  return;
}



fn pass1_1018_34a6(param_1: u32)
{
  pass1_1018_3d6c(param_1);
  return;
}


fn pass1_1018_36e6(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x12e) = param_4;
  (iVar1 + 0x130) = param_3;
  (iVar1 + 0x132) = param_2;
  (iVar1 + 0x134) = 0x0;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_3710(param_1: *mut u32,param_2: u16,param_3: u16,param_4: u16)
{
  let uVar1: u32;
  code **ppcVar2;
  let iVar3: i16;
  let uVar4: u16;
  let puVar5: *mut u8
  let iVar6: i16;
  let uVar7: u16;
  let in_AF: u8;
  let lVar8: i32;
  let puVar9: *mut u16;
  let local_12a: [u8;118];
  let uStack18: u32;
  let puStack14: *mut u16;
  let uStack10: u32;
  let puStack6: *mut u16;
  
  puStack6 = 0x0;
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  uStack10 = switch_1018_3b9e(param_1,(iVar6 + 0x12e),param_3,param_4,
                              param_2);
  uVar4 = (iVar6 + 0x12e) - 0x188;
  uStack18 = (uStack10 & 0xffff0000 | uVar4);
  switch(uVar4) {
  case 0x0:
    lVar8 = pass1_1008_57f0(uStack10,(iVar6 + 0x130),param_2);
    puVar5 = (lVar8 >> 0x10);
    iVar3 = lVar8;
    mem_op_1000_179c(0x10,puVar5,0x1000);
    if (lVar8 != 0x0) {
      uStack18 =
                 struct_1018_4790(lVar8,(iVar6 + 0x132),0x0,
                                  (iVar3 + 0xe));
      puStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x1:
    puVar9 = pass1_1008_57f0(uStack10,(iVar6 + 0x130),param_2);
    puVar5 = (puVar9 >> 0x10);
    iVar3 = puVar9;
    mem_op_1000_179c(0x14,puVar5,0x1000);
    uVar4 = (puVar9 >> 0x10) | puVar9;
    if (puVar9 != 0x0) {
      struct_1018_47c8(puVar9,(iVar6 + 0x132),0x0,(iVar3 + 0x12),
                       (iVar3 + 0xe));
      uStack18 = (puVar9 & 0xffff | uVar4 << 0x10);
      puStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x2:
    puVar9 = pass1_1008_57f0(uStack10,(iVar6 + 0x130),param_2);
    puVar5 = (puVar9 >> 0x10);
    iVar3 = puVar9;
    mem_op_1000_179c(0x12,puVar5,0x1000);
    uVar4 = (puVar9 >> 0x10) | puVar9;
    if (puVar9 != 0x0) {
      pass1_1018_4808(puVar9,(iVar6 + 0x132),0x0,(iVar3 + 0xe));
      uStack18 = (puVar9 & 0xffff | uVar4 << 0x10);
      puStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x3:
    puVar9 = pass1_1008_57f0(uStack10,(iVar6 + 0x130),param_2);
    puVar5 = (puVar9 >> 0x10);
    iVar3 = puVar9;
    mem_op_1000_179c(0x14,puVar5,0x1000);
    if (puVar9 != 0x0) {
      uStack18 = struct_1018_4842(puVar9,(iVar6 + 0x132),0x0,
                                  (iVar3 + 0xe));
      puStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x4:
    puVar9 = pass1_1008_57f0(uStack10,(iVar6 + 0x130),param_2);
    puVar5 = (puVar9 >> 0x10);
    iVar3 = puVar9;
    mem_op_1000_179c(0x10,puVar5,0x1000);
    if (puVar9 != 0x0) {
      uStack18 = struct_1018_48b0(puVar9,(iVar6 + 0x132),0x0,
                                  (iVar3 + 0xe));
      puStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x5:
    puVar9 = pass1_1008_57f0(uStack10,(iVar6 + 0x130),param_2);
    puVar5 = (puVar9 >> 0x10);
    iVar3 = puVar9;
    mem_op_1000_179c(0x12,puVar5,0x1000);
    if (puVar9 != 0x0) {
      uStack18 =
                 struct_1018_4920(puVar9,(iVar6 + 0x132),0x0,
                                  (iVar3 + 0xe));
      puStack6 = uStack18;
    }
    break;
  default:
    goto switchD_1018_393f_caseD_6;
  }
  uStack18 = 0x0;
  puStack6 = uStack18;
switchD_1018_393f_caseD_6:
  uVar1 = (iVar6 + 0x122);
  pass1_1008_e852(uVar1,(uVar1 >> 0x10),(iVar6 + 0x126),
                  param_2,(uStack18 >> 0x10));
  uVar1 = (iVar6 + 0x122);
  puStack14 = uStack18;
  pass1_1008_e852(uVar1,(uVar1 >> 0x10),(iVar6 + 0x12a),
                  param_2,(uStack18 >> 0x10));
  pass1_1038_2a0e((astruct_100 *)CONCAT22(param_2,local_12a),(iVar6 + 0x136),
                  puStack6,uStack18,puStack14,param_2,in_AF);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_2,local_12a));
  (iVar6 + 0x136) = 0x0;
  ppcVar2 = (code **)(*param_1 + 0x10);
  (**ppcVar2)(0x1030,param_1);
  pass1_1038_2a5c((u16 *)CONCAT22(param_2,local_12a));
  return;
}



fn pass1_1018_3a42(param_1: u32,param_2: u32,param_3: u16,param_4: u16)
{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0x122);
  pass1_1008_e852(uVar1,(uVar1 >> 0x10),param_2,param_4,param_3);
  return;
}



fn pass1_1018_3a5c(param_1: u32,param_2: u32,param_3: u32,param_4: u16)
{
  pass1_1008_e320(*(astruct_102 **)(param_1 + 0x122),param_2,param_3,param_4);
  return;
}



fn pass1_1018_3a7a(param_1: u32,param_2: u32,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u32;
  let uVar2: u32;
  
  uVar1 = (param_1 + 0x122);
  uVar2 = string_1008_e586(uVar1,(uVar1 >> 0x10),param_2,param_3,
                           param_4);
  return uVar2;
}



fn pass1_1018_3a94(param_1: u32,param_2: *mut u32,param_3: *mut u32,param_4: u16)
{
  pass1_1008_e3ec((param_1 + 0x122),param_2,param_3,param_4);
  return;
}



fn pass1_1018_3ab2(param_1: u32,param_2: i16,param_3: i16,param_4: u16) -> u16

{
  let uVar1: u16;
  let uVar2: u16;
  let iVar3: i16;
  let lVar4: i32;
  let uStack22: u16;
  let local_10: [u8;8];
  let iStack8: i16;
  let uStack6: u32;
  
  if (0x5 < param_3 - 0x188U) {
    return 0x0;
  }
  iVar3 = param_1;
  uVar2 = (param_1 >> 0x10);
  switch(param_3) {
  case 0x188:
    uVar1 = (iVar3 + 0xa);
    uVar2 = (iVar3 + 0xc);
    break;
  case 0x189:
    uVar1 = (iVar3 + 0xe);
    uVar2 = (iVar3 + 0x10);
    break;
  case 0x18a:
    uVar1 = (iVar3 + 0x12);
    uVar2 = (iVar3 + 0x14);
    break;
  case 0x18b:
    uVar1 = (iVar3 + 0x16);
    uVar2 = (iVar3 + 0x18);
    break;
  case 0x18c:
    uVar1 = (iVar3 + 0x1a);
    uVar2 = (iVar3 + 0x1c);
    break;
  case 0x18d:
    uVar1 = (iVar3 + 0x1e);
    uVar2 = (iVar3 + 0x20);
  }
  uStack6 = CONCAT22(uVar2,uVar1);
  iStack8 = 0x0;
  pass1_1008_5784(CONCAT22(param_4,local_10),uStack6);
  while( true ) {
    lVar4 = pass1_1008_5b12(local_10,param_4);
    uVar2 = (lVar4 >> 0x10);
    if ((lVar4 == 0x0) || (iStack8 == param_2)) break;
    iStack8 += 0x1;
  }
  uStack22 = 0x0;
  if (lVar4 != 0x0) {
    if ((lVar4 + 0xa) == 0x0) {
      uStack22 = (lVar4 + 0x8);
    }
    else {
      uStack22 = 0xffff;
    }
  }
  return uStack22;
}


fn pass1_1018_3cda(param_1: *mut u32,char *param_2,char *param_3)
{
  code **ppcVar1;
  let uVar2: u16;
  let extraout_DX: u16;
  let uVar3: u16;
  astruct_506 *iVar5;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_506 *)param_1;
  ppcVar1 = (code **)(*param_1 + 0x10);
  (**ppcVar1)();
  uVar3 = extraout_DX;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar5->field_0x126,0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar5->field_0x12a,0x1000);
  uVar2 = str_op_1008_60e8(param_3,uVar3);
  iVar5->field_0x126 = uVar2;
  iVar5->field_0x128 = uVar3;
  uVar2 = str_op_1008_60e8(param_2,uVar3);
  iVar5->field_0x12a = uVar2;
  iVar5->field_0x12c = uVar3;
  return;
}



fn pass1_1018_3d44(param_1: u32,param_2: *mut u32,param_3: *mut u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_3 = (param_1 + 0x126);
  *param_2 = (param_1 + 0x12a);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_3d6c(param_1: u32)
{
  let bVar1: u8;
  let uVar2: u16;
  let puVar3: *mut u8;
  let uVar4: u16;
  astruct_679 *iVar6;
  let uVar5: u16;
  let uVar6: u32;
  let uVar7: u32;
  
  uVar5 = (param_1 >> 0x10);
  iVar6 = (astruct_679 *)param_1;
  uVar4 = iVar6->field_0x142;
  uVar2 = uVar4 + 0x1e;
  if (iVar6->field_0x144 + 0x1U == (uVar4 < 0xffe2)) {
    if (uVar2 != 0x3c) {
      if (0x3c < uVar2) {
        return;
      }
      bVar1 = (byte)uVar2;
      if (bVar1 == 0x14) {
        iVar6->field_0x142 = 0xffec;
LAB_1018_3e3d:
        iVar6->field_0x144 = -0x1;
        return;
      }
      if (0x14 < bVar1) {
        if (bVar1 == 0x1e) {
          if (PTR_LOOP_1050_13ae < 0x1) {
            return;
          }
          if (SBORROW2(PTR_LOOP_1050_13ae,0x1)) {
            return;
          }
          if (PTR_LOOP_1050_13ae != &PTR_LOOP_1050_0002 &&
              0x0 < (PTR_LOOP_1050_13ae + -0x1)) {
            puVar3 = PTR_LOOP_1050_13ae + -0x3;
            if (puVar3 == 0x0) {
              pass1_1008_612e(0x1,0x64,0x0);
              if (puVar3 < 0x32) {
                uVar4 = 0xa;
              }
              else {
                uVar4 = 0xfff6;
              }
              iVar6->field_0x142 = uVar4;
              iVar6->field_0x144 = uVar4 >> 0xf;
              return;
            }
            if (puVar3 != (&PTR_LOOP_1050_0000 + 0x1)) {
              return;
            }
            iVar6->field_0x142 = 0xfff6;
            goto LAB_1018_3e3d;
          }
          iVar6->field_0x142 = 0xa;
        }
        else {
          if (bVar1 == 0x28) {
            iVar6->field_0x142 = 0x14;
          }
          else {
            if (bVar1 != 0x32) {
              return;
            }
            iVar6->field_0x142 = 0x1e;
          }
        }
        iVar6->field_0x144 = 0x0;
        return;
      }
      if (bVar1 != 0x0) {
        if (bVar1 != 0xa) {
          return;
        }
        iVar6->field_0x142 = 0xffe2;
        goto LAB_1018_3e3d;
      }
    }
    uVar7 = 0x5;
    uVar6 = pass1_1030_8326();
    if (uVar6 % uVar7 == 0x0) {
      &iVar6->field_0x142 = 0x0;
      return;
    }
  }
  return;
}



fn pass1_1018_3e8c(param_1: u16,param_2: u16,param_3: *mut u16,param_4: *mut u16)
{
  *param_4 = 0x1;
  *param_3 = 0x19;
  return;
}



fn pass1_1018_3ea4(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  pass1_1008_cac6(param_1);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0x136);
  uVar2 = (iVar4 + 0x138);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  (iVar4 + 0x136) = 0x0;
  return;
}


fn pass1_1018_427c(param_1: u32)
{
  let iVar1: i16;
  let in_AX: u16;
  let in_DX: u16;
  let uVar2: u16;
  let uVar3: u16;
  let unaff_SS: u16;
  let uVar4: u32;
  let lVar5: i32;
  
  uVar3 = (param_1 >> 0x10);
  uVar2 = param_1;
  uVar4 = switch_1018_3b9e(param_1,(uVar2 + 0x12e),in_AX,in_DX,unaff_SS);
  iVar1 = (uVar2 + 0x12e);
  if (iVar1 == 0x188) {
    lVar5 = pass1_1008_57f0(uVar4,(uVar2 + 0x130),unaff_SS);
    pass1_1018_456a(uVar2,uVar3,(lVar5 + 0xe));
  }
  else {
    if (iVar1 == 0x18b) {
      lVar5 = pass1_1008_57f0(uVar4,(uVar2 + 0x130),unaff_SS);
      pass1_1018_45d4(uVar2,uVar3,(lVar5 + 0xe));
    }
    else {
      if (iVar1 == 0x18c) {
        lVar5 = pass1_1008_57f0(uVar4,(uVar2 + 0x130),unaff_SS);
        pass1_1018_451e(uVar2,uVar3,(lVar5 + 0xe));
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_435e(param_1: u32,param_2: i32,param_3: i16,param_4: i16,param_5: u16,
               param_6: u16)

{
  let uVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  
  if (param_3 < param_4) {
    param_4 = param_3;
  }
  uVar2 = 0x0;
  uVar4 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x122);
  pass1_1008_e852(uVar1,(uVar1 >> 0x10),
                  (param_1 + 0x126),param_6,param_5);
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,CONCAT22(param_5,uVar2));
  do {
    do {
      uVar3 = uVar2;
      pass1_1008_612e(param_4,param_3,uVar3);
      uVar2 = (uVar3 * 0x2 + 0x411c);
    } while (uVar2 == 0x0);
    if (uVar2 != 0x1) {
      pass1_1008_612e(0x1,uVar2,uVar2);
    }
    uVar2 -= 0x1;
    switch_1018_3ee6(param_1,param_2,uVar2,uVar3,param_5);
    param_5 |= uVar2;
  } while (param_5 == 0x0);
  return;
}


fn pass1_1018_451e(param_1: u16,param_2: u16,param_3: i16) -> u16

{
  let uStack6: u16;
  
  if (param_3 == 0x7) {
    uStack6 = 0x9;
  }
  else {
    if (param_3 == 0x8) {
      uStack6 = 0xa;
    }
    else {
      if (param_3 == 0xc) {
        uStack6 = 0x19;
      }
      else {
        if (param_3 == 0xd) {
          uStack6 = 0x3;
        }
        else {
          uStack6 = 0x8;
        }
      }
    }
  }
  return uStack6;
}



fn pass1_1018_456a(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  let uStack6: u16;
  
  if (false) {
switchD_1018_45a3_caseD_17:
    uStack6 = 0x1;
  }
  else {
    switch(param_3) {
    case 0x11:
    case 0x12:
    case 0x13:
    case 0x14:
    case 0x15:
      uStack6 = 0x2;
      break;
    case 0x16:
    case 0x1e:
      uStack6 = 0x3;
      break;
    default:
      goto switchD_1018_45a3_caseD_17;
    case 0x1d:
    case 0x21:
      uStack6 = 0x4;
    }
  }
  return uStack6;
}



fn pass1_1018_45d4(param_1: u16,param_2: u16,param_3: i16) -> u16

{
  let uStack6: u16;
  
  if (param_3 == 0x3) {
    uStack6 = 0x16;
  }
  else {
    if (param_3 == 0x4) {
      uStack6 = 0x17;
    }
    else {
      uStack6 = 0x14;
    }
  }
  return uStack6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

long 
pass1_1018_4608(param_1: u16,param_2: u32,param_3: u32,param_4: u32)

{
  let uVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let lVar7: i32;
  char *pcVar8;
  char *pcVar9;
  let uStack26: u32;
  let uStack22: u32;
  let local_a: [u8;8];
  
  uVar1 = (param_2 + 0x122);
  pass1_1008_5784(CONCAT22(param_1,local_a),(uVar1 + 0xa));
  while( true ) {
    lVar7 = pass1_1008_5b12(local_a,param_1);
    uVar5 = (lVar7 >> 0x10);
    uVar2 = lVar7;
    uVar6 = uVar5 | uVar2;
    if (lVar7 == 0x0) {
      return 0x0;
    }
    uVar1 = (uVar2 + 0x4);
    uVar3 = uVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
    uStack22 = CONCAT22(uVar6,uVar3);
    uVar1 = (uVar2 + 0x8);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
    uStack26 = CONCAT22(uVar6,uVar3);
    pcVar8 = pass1_1038_4d28(uStack22);
    pcVar9 = pass1_1038_4d28(uStack26);
    iVar4 = pass1_1000_3d7a(param_4,pcVar8);
    if ((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_3,pcVar9), iVar4 == 0x0))
    break;
    iVar4 = pass1_1000_3d7a(param_3,pcVar8);
    if ((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_4,pcVar9), iVar4 == 0x0))
    {
      return lVar7;
    }
  }
  return lVar7;
}



fn pass1_1018_46e6(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1018_33b4(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_4760(param_1: *mut u16)
{
  astruct_507 *iVar2;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar2 = (astruct_507 *)param_1;
  *param_1 = &PTR_LOOP_1050_4aa6;
  iVar2->field_0x2 = 0x1018;
  fn_ptr_1000_17ce((astruct_18 *)iVar2->field_0x4,0x1000);
  *param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  return;
}


fn pass1_1018_4808(param_1: *mut u16,param_2: u32,param_3: u32,param_4: u32)
{
  let iVar1: i16;
  let uVar2: u16;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0xe) = param_4;
  *param_1 = &PTR_LOOP_1050_4aa2;
  (iVar1 + 0x2) = 0x1018;
  (iVar1 + 0xc) = 0x3;
  return;
}


fn pass1_1018_4882(param_1: *mut u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = &PTR_LOOP_1050_4a8e;
  (param_1 + 0x2) = 0x1018;
  fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x10),0x1000);
  pass1_1018_4760(param_1);
  return;
}


fn pass1_1018_495a(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_4980(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_49a6(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_49cc(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_49f2(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1018_4882(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_4a18(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_4a3e(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_4a64(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_4aaa(param_1: i16,param_2: u16,param_3: u16,uchar *param_4,param_5: u16)
{
  struct_op_1018_4cda(param_1,param_2,param_3);
  CONCAT22(param_2,param_1) = 0x4b06;
  (param_1 + 0x2) = 0x1018;
  pass1_1018_4dce(CONCAT22(param_2,param_1),0x9a,param_4,param_5);
  _PTR_LOOP_1050_4230 = CONCAT22(param_2,param_1);
  return;
}



astruct_11 * 
pass1_1018_4ae0(astruct_11 *param_1,param_2: u8,param_3: u16)

{
  clenaup_win_ui_1018_4d22(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1018_4b78(param_1: *mut u32,param_2: u16)
{
  code **ppcVar1;
  let puVar2: *mut u8
  let uVar3: u16;
  let puVar4: *mut u16;
  let puVar5: u32;
  
  puVar2 = param_1._2_2_;
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 |
                  ZEXT24((param_1 + 0xa))),(WNDCLASS16 *)0x0,0x8);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | (param_1 + 0x18)),
                  (WNDCLASS16 *)0x0,0x8);
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,puVar2,param_1._2_2_);
  puVar5 = 
           pass1_1010_5f7a(puVar4,(puVar4 >> 0x10),0x0,
                           (param_1 + 0x12));
  uVar3 = (puVar5 >> 0x10);
  if ((uVar3 | puVar5) != 0x0) {
    (param_1 + 0xa) = *puVar5;
    (param_1 + 0xe) = (puVar5 + 0x4);
  }
  ppcVar1 = (code **)(*param_1 + 0x20);
  (**ppcVar1)(0x1010,param_1);
  if (((param_1 + 0xe) == 0x0) && ((param_1 + 0x10) == 0x0)) {
    (param_1 + 0xa) = (param_1 + 0x18);
    (param_1 + 0xc) = (param_1 + 0x1a);
  }
  (param_1 + 0xe) = (param_1 + 0x1c);
  (param_1 + 0x10) = (param_1 + 0x1e);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_4c2c(param_1: u32,param_2: *mut u32,param_3: u16,param_4: u16)
{
  let puVar1: *mut u16;
  
  (param_1 + 0xa) = *param_2;
  (param_1 + 0xe) = param_2[0x1];
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_1._2_2_,
                           param_1._2_2_);
  pass1_1010_5fb0(puVar1,0x0,(param_1 + 0xa),param_1._2_2_,
                  (param_1 + 0x12));
  return;
}



fn pass1_1018_4c78(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1018_4dce(param_1: *mut u32,param_2: u16,uchar *param_3,param_4: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let unaff_DI: i16;
  let puVar3: *mut u16;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,param_3,unaff_DI);
  uVar2 = (puVar3 >> 0x10);
  ppcVar1 = (code **)(*param_1 + 0x10);
  (**ppcVar1)(0x1010,param_1,param_2,(puVar3 + 0xc),
              (puVar3 + 0xa));
  return;
}


astruct_11 * 
pass1_1018_5032(astruct_11 *param_1,param_2: u8,param_3: u16)

{
  clenaup_win_ui_1018_4d22(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_5070(astruct_641 *param_1,param_2: u16,param_3: u16)
{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  param_1->field_0x16 = 0x0;
  CONCAT22(param_2,param_1) = 0x56d2;
  param_1->field_0x2 = 0x1018;
  return;
}



fn pass1_1018_50ac(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x56d2;
  (iVar4 + 0x2) = 0x1018;
  puVar1 = (iVar4 + 0xe);
  uVar2 = (iVar4 + 0x10);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_50ea(param_1: u32,param_2: u16,param_3: u32)
{
  let iVar1: i16;
  code **ppcVar2;
  let uVar3: u32;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  astruct_99 *paStack6;
  
  paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
  uVar6 = (paStack6 >> 0x10);
  uVar4 = paStack6;
  if ((uchar *)(uVar6 | uVar4) == 0x0) {
    paStack6 = (astruct_99 *)0x0;
  }
  else {
    paStack6->field_0x0 = 0x389a;
    (uVar4 + 0x2) = 0x1008;
    (uVar4 + 0x4) = 0x0;
    (uVar4 + 0x6) = 0x0;
    (uVar4 + 0x8) = 0x0;
    (uVar4 + 0xa) = 0x0;
    (uVar4 + 0xc) = 0x0;
    paStack6->field_0x0 = 0x56ce;
    (uVar4 + 0x2) = 0x1018;
  }
  uVar9 = (paStack6 >> 0x10);
  uVar7 = paStack6;
  (uVar7 + 0xa) = param_2;
  uVar10 = (param_1 >> 0x10);
  iVar8 = param_1;
  uVar3 = (iVar8 + 0xa);
  iVar1 = (uVar3 + 0xc);
  if (iVar1 == 0x1) {
    uVar3 = (iVar8 + 0xa);
    uVar5 = (uVar3 + 0xe);
    (uVar7 + 0x4) = uVar5;
  }
  else {
    if (iVar1 == 0x5) {
      uVar3 = (iVar8 + 0xa);
      uVar5 = (uVar3 + 0xe);
      (uVar7 + 0x6) = uVar5;
    }
    else {
      if (iVar1 != 0x6) {
        if ((uVar9 | uVar7) == 0x0) {
          return;
        }
        ppcVar2 = (code **)paStack6;
        (**ppcVar2)();
        return;
      }
      uVar3 = (iVar8 + 0xa);
      uVar5 = (uVar3 + 0xe);
      (uVar7 + 0x8) = uVar5;
    }
  }
  pass1_1030_6c66(param_3,0x1,paStack6,uVar5,(uVar6 | uVar4),0x1030);
  return;
}



fn pass1_1018_51d2(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0xe);
  uVar2 = (iVar4 + 0x10);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0xe) = 0x0;
  return;
}



fn pass1_1018_5206(param_1: u32,param_2: u32,param_3: u16) -> u32

{
  let iVar1: i16;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u32;
  let local_a: [u8;8];
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  (iVar3 + 0xa) = 0x0;
  pass1_1008_5784(CONCAT22(param_3,local_a),(iVar3 + 0xe));
  do {
    uVar5 = pass1_1008_5b12(local_a,param_3);
    uVar2 = (uVar5 >> 0x10);
    (iVar3 + 0xa) = uVar5;
    (iVar3 + 0xc) = uVar2;
    if ((uVar2 | (iVar3 + 0xa)) == 0x0) break;
    uVar5 = (iVar3 + 0xa);
    iVar1 = pass1_1000_3d7a((uVar5 + 0x4),param_2);
  } while (iVar1 != 0x0);
  return CONCAT22((iVar3 + 0xc),(iVar3 + 0xa));
}



fn pass1_1018_526a(param_1: u32,param_2: u32,param_3: u16) -> u32

{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (*(long *)(iVar1 + 0xe) == 0x0) {
    pass1_1018_5292(param_1 & 0xffff | uVar2 << 0x10,param_2,param_3);
  }
  return CONCAT22((iVar1 + 0x10),(iVar1 + 0xe));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_5292(param_1: u32,param_2: u32,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u32;
  code **ppcVar3;
  let uVar4: u16;
  let BVar5: bool;
  let puVar6: *mut u8;
  let iVar7: i16;
  char *pcVar8;
  let uVar9: u16;
  let puVar10: u32;
  let puVar11: u32;
  let uVar12: u32;
  let extraout_DX: *mut u8
  let extraout_DX_00: *mut u8
  let uVar13: u16;
  let extraout_DX_01: *mut u8
  let puVar14: *mut u8
  let extraout_DX_02: u16;
  let extraout_DX_03: u16;
  let puVar15: *mut u8
  let extraout_DX_04: *mut u8
  let uVar16: u16;
  let extraout_DX_05: u16;
  let extraout_DX_06: u16;
  let extraout_DX_07: *mut u8
  let extraout_DX_08: *mut u8
  let iVar17: i16;
  let uVar18: u16;
  let uVar19: u16;
  let puVar20: *mut u16;
  let uStack50: u16;
  let local_26: [u8;8];
  let uStack30: u32;
  let uStack26: u32;
  let uStack22: u32;
  let puStack18: u32;
  let puStack16: *mut u8
  let puStack14: u32;
  let puStack12: *mut u8
  let uStack10: u16;
  let uStack8: u32;
  let uStack4: u16;
  
  uVar18 = (param_1 >> 0x10);
  iVar17 = param_1;
  puStack18 = (iVar17 + 0xe);
  uVar12 = ZEXT24(puStack18);
  puVar14 = *(uchar **)(iVar17 + 0x10);
  puStack16 = puVar14;
  puStack14 = puStack18;
  puStack12 = puVar14;
  if ((puVar14 | puStack18) != 0x0) {
    ppcVar3 = (code **)*puStack18;
    (**ppcVar3)();
    puVar14 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar14,0x1000);
  puStack18 = uVar12;
  puStack16 = puVar14;
  if ((puVar14 | puStack18) == 0x0) {
    uVar12 = 0x0;
    puVar14 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)(uVar12 & 0xffff | ZEXT24(puVar14) << 0x10));
    puVar14 = extraout_DX_00;
  }
  (iVar17 + 0xe) = uVar12;
  *(uchar **)(iVar17 + 0x10) = puVar14;
  for (uStack4 = 0x21; -0x1 < uStack4; uStack4 -= 0x1) {
    uStack22 = pass1_1030_7c28(param_2,uStack4,uVar12,puVar14,param_3);
    uVar12 = uStack22 & 0xffff;
    uVar13 = uVar12;
    puVar14 = ((uStack22 >> 0x10) | uVar13);
    if (puVar14 != 0x0) {
      string_1020_c0ca(uStack4);
      uVar4 = str_op_1008_60e8(CONCAT22(puVar14,uVar13),puVar14);
      uVar12 = uVar4;
      uStack26 = CONCAT22(puVar14,uVar4);
      mem_op_1000_179c(0x10,puVar14,0x1000);
      puStack14 = uVar12;
      puStack12 = puVar14;
      if ((puVar14 | puStack14) == 0x0) {
        uVar12 = 0x0;
        uVar13 = 0x0;
      }
      else {
        struct_1018_4790(uVar12 & 0xffff | ZEXT24(puVar14) << 0x10,uStack22,uStack26,
                         uStack4);
        uVar13 = extraout_DX_02;
      }
      uStack30 = uVar12 & 0xffff | uVar13 << 0x10;
      uVar2 = (iVar17 + 0xe);
      ppcVar3 = (code **)((iVar17 + 0xe) + 0x4);
      (**ppcVar3)(0x1000,uVar2,(uVar2 >> 0x10),uVar12,uVar13);
      puVar14 = extraout_DX_01;
    }
  }
  uStack8 = struct_op_1030_73a8(param_2);
  uStack10 = (uStack8 + 0xc);
  BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack10,0x4);
  if (BVar5 != 0x0) {
    uStack30 = uStack8;
    uStack26 = (uStack8 + 0x20);
    pass1_1008_5784(CONCAT22(param_3,local_26),uStack26);
    while( true ) {
      puVar6 = local_26;
      pass1_1008_5b12(puVar6,param_3);
      uStack22 = CONCAT22(extraout_DX_03,puVar6);
      puVar14 = (extraout_DX_03 | puVar6);
      if (puVar14 == 0x0) break;
      iVar1 = (puVar6 + 0x6);
      iVar7 = iVar1 + -0x7;
      if (iVar7 == 0x0) {
LAB_1018_53f0:
        pcVar8 = string_op_1020_c222((puVar6 + 0x6));
        uVar9 = str_op_1008_60e8(CONCAT22(puVar14,pcVar8),puVar14);
        puVar15 = puVar14;
        uVar4 = uVar9;
        mem_op_1000_179c(0x10,puVar14,0x1000);
        puStack18 = uVar4;
        puStack16 = puVar15;
        if ((puVar15 | uVar4) == 0x0) {
          uVar19 = 0x0;
          uVar16 = 0x0;
        }
        else {
          uVar19 = (uStack22 >> 0x10);
          puVar20 = struct_1018_48b0((u16 *)CONCAT22(puVar15,uVar4),
                                     (uStack22 + 0xa),
                                     CONCAT22(puVar14,uVar9),
                                     (uStack22 + 0x6));
          uVar16 = (puVar20 >> 0x10);
          uVar19 = SUB42(puVar20,0x0);
        }
        uVar2 = (iVar17 + 0xe);
        ppcVar3 = (code **)((iVar17 + 0xe) + 0x4);
        (**ppcVar3)(0x1000,uVar2,(uVar2 >> 0x10),uVar19,uVar16);
        puVar14 = extraout_DX_04;
      }
      else {
        if (((0x5 < iVar7) && (!SBORROW2(iVar7,0x6))) && (iVar1 + -0xd < 0x2))
        goto LAB_1018_53f0;
      }
      uVar19 = (uStack22 >> 0x10);
      if ((uStack22 + 0x8) != 0x0) {
        pcVar8 = string_op_1020_c2f8((uStack22 + 0x8));
        puVar10 = 
                  str_op_1008_60e8(CONCAT22(puVar14,pcVar8),puVar14);
        puVar15 = puVar14;
        puVar11 = puVar10;
        mem_op_1000_179c(0x10,puVar14,0x1000);
        puStack14 = puVar11;
        puStack12 = puVar15;
        if ((puVar15 | puVar11) == 0x0) {
          uVar19 = 0x0;
          uVar16 = 0x0;
        }
        else {
          uVar19 = (uStack22 >> 0x10);
          puVar20 = struct_1018_48e8((u16 *)CONCAT22(puVar15,puVar11),
                                     (uStack22 + 0xa),
                                     CONCAT22(puVar14,puVar10),
                                     (uStack22 + 0x8));
          uVar16 = (puVar20 >> 0x10);
          uVar19 = SUB42(puVar20,0x0);
        }
        uVar2 = (iVar17 + 0xe);
        ppcVar3 = (code **)((iVar17 + 0xe) + 0x4);
        (**ppcVar3)(0x1000,uVar2,(uVar2 >> 0x10),uVar19,uVar16);
      }
    }
  }
  uVar19 = (param_2 >> 0x10);
  uVar12 = (param_2 + 0x3e);
  uVar13 = (param_2 + 0x40);
  uStack50 = uVar12;
  if ((uVar13 | uStack50) != 0x0) {
    pass1_1008_5784(CONCAT22(param_3,local_26),
                    uVar12 & 0xffff | uVar13 << 0x10);
    while( true ) {
      puVar6 = local_26;
      pass1_1008_5b12(puVar6,param_3);
      puVar14 = (extraout_DX_05 | puVar6);
      if (puVar14 == 0x0) break;
      if ((puVar6 + 0x4) != 0x0) {
        pcVar8 = string_1020_c0d8((puVar6 + 0x4));
        uVar13 = str_op_1008_60e8(CONCAT22(puVar14,pcVar8),puVar14);
        uStack30 = CONCAT22(puVar14,uVar13);
        mem_op_1000_179c(0x10,puVar14,0x1000);
        puStack18 = uVar13;
        puStack16 = puVar14;
        if ((puVar14 | uVar13) == 0x0) {
          uVar13 = 0x0;
          uVar19 = 0x0;
        }
        else {
          struct_1018_4790(CONCAT22(puVar14,uVar13),(puVar6 + 0xa),
                           uStack30,(puVar6 + 0x4));
          uVar19 = extraout_DX_06;
        }
        uStack26 = CONCAT22(uVar19,uVar13);
        uVar2 = (iVar17 + 0xe);
        ppcVar3 = (code **)((iVar17 + 0xe) + 0x4);
        (**ppcVar3)(0x1000,uVar2,(uVar2 >> 0x10),uVar13,uVar19);
        puVar14 = extraout_DX_07;
      }
      if ((puVar6 + 0x6) != 0x0) {
        pcVar8 = string_op_1020_c222((puVar6 + 0x6));
        puVar11 = 
                  str_op_1008_60e8(CONCAT22(puVar14,pcVar8),puVar14);
        uStack30 = CONCAT22(puVar14,puVar11);
        mem_op_1000_179c(0x10,puVar14,0x1000);
        puStack14 = puVar11;
        puStack12 = puVar14;
        if ((puVar14 | puVar11) == 0x0) {
          uVar19 = 0x0;
          uVar16 = 0x0;
        }
        else {
          puVar20 = struct_1018_48b0((u16 *)CONCAT22(puVar14,puVar11),
                                     (puVar6 + 0xa),uStack30,
                                     (puVar6 + 0x6));
          uVar16 = (puVar20 >> 0x10);
          uVar19 = SUB42(puVar20,0x0);
        }
        uStack26 = CONCAT22(uVar16,uVar19);
        uVar2 = (iVar17 + 0xe);
        ppcVar3 = (code **)((iVar17 + 0xe) + 0x4);
        (**ppcVar3)(0x1000,uVar2,(uVar2 >> 0x10),uVar19,uVar16);
        puVar14 = extraout_DX_08;
      }
      if ((puVar6 + 0x8) != 0x0) {
        pcVar8 = string_op_1020_c2f8((puVar6 + 0x8));
        uVar13 = str_op_1008_60e8(CONCAT22(puVar14,pcVar8),puVar14);
        uStack30 = CONCAT22(puVar14,uVar13);
        mem_op_1000_179c(0x10,puVar14,0x1000);
        puStack18 = uVar13;
        puStack16 = puVar14;
        if ((puVar14 | uVar13) == 0x0) {
          uVar19 = 0x0;
          uVar16 = 0x0;
        }
        else {
          puVar20 = struct_1018_48e8((u16 *)CONCAT22(puVar14,uVar13),
                                     (puVar6 + 0xa),uStack30,
                                     (puVar6 + 0x8));
          uVar16 = (puVar20 >> 0x10);
          uVar19 = SUB42(puVar20,0x0);
        }
        uStack26 = CONCAT22(uVar16,uVar19);
        uVar2 = (iVar17 + 0xe);
        ppcVar3 = (code **)((iVar17 + 0xe) + 0x4);
        (**ppcVar3)(0x1000,uVar2,(uVar2 >> 0x10),uVar19,uVar16);
      }
    }
  }
  return;
}



fn pass1_1018_567c(param_1: *mut u16,param_2: u8) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x389a;
  ((i16 *)param_1)[0x1] = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((i16 *)param_1,uVar1,0x1000);
  }
  return param_1;
}



fn pass1_1018_56a8(param_1: u32,param_2: u8,param_3: u16) -> u32

{
  pass1_1018_50ac((u16 *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_56e6(param_1: i16,param_2: u16,param_3: u16) -> u16

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  (param_1 + 0xa) = 0x0;
  CONCAT22(param_2,param_1) = 0x5830;
  (param_1 + 0x2) = 0x1018;
  return CONCAT22(param_2,param_1);
}



fn pass1_1018_5714(param_1: *mut u16,param_2: u16)
{
  *param_1 = 0x5830;
  (param_1 + 0x2) = 0x1018;
  pass1_1010_1d80(param_1,param_2);
  return;
}



u32 
pass1_1018_5732(param_1: u16,param_2: u16,param_3: u32,param_4: u16,param_5: u16,
               param_6: u16)

{
  let uVar1: u32;
  
  uVar1 = pass1_1030_6d4e(param_3,param_4,param_5,param_6);
  return uVar1;
}



fn pass1_1018_5742(param_1: u16,param_2: u16,param_3: *mut u32,param_4: u32)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let bVar4: bool;
  let puVar5: u32;
  let uVar6: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uStack16: u32;
  
  bVar4 = false;
  puVar1 = (param_3 + 0x4);
  ppcVar2 = (code **)(*puVar1 + 0x10);
  puVar5 = puVar1;
  (**ppcVar2)();
  uVar3 = puVar5 & 0xffff | extraout_DX << 0x10;
  uStack16 = 0x0;
  do {
    if (uVar3 <= uStack16) {
LAB_1018_579f:
      if (!bVar4) {
        if (param_3 != 0x0) {
          ppcVar2 = (code **)*param_3;
          (**ppcVar2)();
        }
        param_3 = 0x0;
      }
      pass1_1030_6d80(param_4,param_3);
      return;
    }
    ppcVar2 = (code **)(*puVar1 + 0x4);
    uVar6 = uVar3;
    (**ppcVar2)();
    if ((extraout_DX_00 | uVar6) != 0x0) {
      bVar4 = true;
      goto LAB_1018_579f;
    }
    uStack16 += 0x1;
  } while( true );
}



fn pass1_1018_57d2(param_1: u32,param_2: u32)
{
  (param_1 + 0xa) = param_2;
  return;
}



fn pass1_1018_57e6(param_1: u32,param_2: i32,param_3: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  send_dlg_item_msg_1040_d20c
            ((param_1 + 0xa),param_2,&PTR_LOOP_1050_1040,param_3);
  (param_1 + 0xa) = 0x0;
  return;
}



fn pass1_1018_580a(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1018_5714(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn struct_1018_5840(astruct_20 *param_1,param_2: u16,param_3: u16,param_4: u16)
{
  let extraout_DX: *mut u8
  let uVar1: u16;
  astruct_130 *iVar2;
  let unaff_DI: i16;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  unk_draw_op_1020_7f7a(param_1,0x6,CONCAT22(param_3,param_2));
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_130 *)param_1;
  iVar2->field_0xee = 0x0;
  &iVar2->field_0xf2 = 0x0;
  iVar2->field_0xf6 = 0x0;
  param_1->field_0x0 = (s_Alloc__s_1050_5a5b + 0x7);
  iVar2->field_0x2 = 0x1018;
  iVar2->field_0xe2 = 0x5afe;
  iVar2->field_0xe4 = 0x1018;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x27,param_4,extraout_DX,unaff_DI);
  uVar1 = (puVar3 >> 0x10);
  iVar2->field_0xf2 = puVar3;
  iVar2->field_0xf4 = uVar1;
  iVar2->field_0xe6 = iVar2->field_0xf2;
  iVar2->field_0xe8 = uVar1;
  return;
}



fn pass1_1018_58b6(param_1: *mut u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = (s_Alloc__s_1050_5a5b + 0x7);
  (iVar1 + 0x2) = 0x1018;
  (iVar1 + 0xe2) = 0x5afe;
  (iVar1 + 0xe4) = 0x1018;
  pass1_1020_808e(param_1);
  return;
}


fn pass1_1018_5932(param_1: u32,param_2: u16,param_3: u16) -> u16

{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u32;
  
  uVar4 = (param_1 >> 0x10);
  uVar3 = param_1;
  uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
  if (uVar2 != 0x0) {
    ppcVar1 = (code **)((uVar3 + 0xee) + 0x8);
    uVar5 = (**ppcVar1)();
    param_2 = (uVar5 >> 0x10);
    uVar2 = uVar5;
  }
  if ((uVar3 + 0xea) == 0x0) {
    (uVar3 + 0xea) = 0x1;
    uVar5 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar3 + 0x8),
                            ((uVar3 + 0xf6) * 0x2 + 0x4238),param_2,uVar3,
                            &PTR_LOOP_1050_1038,param_3);
    uVar2 = uVar5;
  }
  return uVar2;
}


astruct_18 *  pass1_1018_5a2e(astruct_18 *param_1,param_2: u8)

{
  param_1 = (astruct_18 *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1018_58b6((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar4
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_5b06(astruct_132 *param_1,param_2: u16,param_3: u16,param_4: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let puVar3: u32;
  let uVar5: u16;
  let uVar6: u32;
  let puVar7: *mut u8
  let uVar8: u16;
  let puVar9: *mut u8
  let uVar10: u16;
  let unaff_DI: i16;
  let puVar11: *mut u16;
  astruct_76 *paVar12;
  let uVar13: u32;
  let uVar14: u16;
  let uVar15: u16;
  astruct_132 *paVar16;
  let uVar17: u16;
  let local_c: [u8;6];
  let puStack6: *mut u16;
  let uVar4: u16;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0x14 = 0x0;
  param_1->field_0x18 = 0x0;
  puVar11 = pass1_1008_3e38((u16 *)CONCAT22(param_2,&param_1->field_0x1c));
  CONCAT22(param_2,param_1) = &PTR_LOOP_1050_5e1a;
  param_1->field_0x2 = 0x1018;
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,
                             (puVar11 >> 0x10),unaff_DI);
  puVar11 = pass1_1008_3e38((u16 *)CONCAT22(param_4,local_c));
  puVar7 = (puVar11 >> 0x10);
  pass1_1008_3f62((u16 *)CONCAT22(param_4,local_c),
                  (puStack6 & 0xffff0000 | (puStack6 + 0xe)))
  ;
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x27,param_4,puVar7,unaff_DI);
  uVar8 = (puVar11 >> 0x10);
  &param_1->field_0x14 = puVar11;
  (&param_1->field_0x14 + 0x2) = uVar8;
  uVar15 = 0x0;
  uVar14 = &param_1->field_0x14;
  ppcVar2 = (code **)(*param_1->field_0x14 + 0x4);
  paVar16 = param_1;
  uVar17 = param_2;
  (**ppcVar2)();
  param_1->field_0x6 = param_1->field_0x14;
  puVar3 = param_1->field_0x14;
  puVar1 = (puVar3 + 0xa);
  uVar6 = CONCAT22(param_2,&param_1->field_0xa);
  ppcVar2 = (code **)(*puVar1 + 0x8);
  (**ppcVar2)(0x1010,puVar1,(puVar1 >> 0x10),uVar6,uVar14,uVar8,uVar15,
              paVar16,uVar17);
  param_1->field_0x12 = uVar6;
  draw_op_1020_9364((astruct_7 *)CONCAT22(param_2,param_1),0x1020,param_4);
  puVar3 = param_1->field_0x14;
  pass1_1008_3f62((u16 *)CONCAT22(param_2,&param_1->field_0x1c),
                  (puVar3 & 0xffff0000 | (puVar3 + 0x52)));
  pass1_1008_3f32((i16 *)CONCAT22(param_2,&param_1->field_0x1c),
                  (i16 *)CONCAT22(param_4,local_c));
  paVar12 = (astruct_76 *)pass1_1008_9f48(param_1->field_0x14);
  uVar13 = pass1_1008_4772(paVar12);
  puVar9 = (uVar13 >> 0x10);
  uVar4 = uVar13;
  puVar7 = puVar9;
  uVar5 = uVar4;
  mem_op_1000_179c(0x14,puVar9,0x1000);
  uVar10 = puVar7 | uVar5;
  if (uVar10 == 0x0) {
    param_1->field_0x18 = 0x0;
  }
  else {
    pass1_1008_50c2((astruct_110 *)CONCAT22(puVar7,uVar5),(uVar4 + 0x8),
                    (uVar4 + 0x4),CONCAT22(param_2,&param_1->field_0x1c)
                    ,puVar1);
    &param_1->field_0x18 = uVar5;
    (&param_1->field_0x18 + 0x2) = uVar10;
  }
  pass1_1008_5134(param_1->field_0x18);
  param_1->field_0x22 = param_1->field_0x1c;
  param_1->field_0x24 = param_1->field_0x1e;
  param_1->field_0x26 = (uVar4 + 0x4) + param_1->field_0x22 + 0x1;
  param_1->field_0x28 = (uVar4 + 0x8) + param_1->field_0x24 + 0x1;
  return;
}



fn pass1_1018_5cc8(param_1: *mut u16,param_2: u16)
{
  let uVar1: u16;
  astruct_18 *paVar2;
  astruct_508 *iVar3;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_508 *)param_1;
  *param_1 = &PTR_LOOP_1050_5e1a;
  iVar3->field_0x2 = 0x1018;
  paVar2 = *(astruct_18 **)&iVar3->field_0x18;
  uVar1 = iVar3->field_0x1a;
  if ((uVar1 | paVar2) != 0x0) {
    pass1_1008_5118(paVar2 & 0xffff | uVar1 << 0x10);
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  if (iVar3->field_0x14 != 0x0) {
    pass1_1010_1ea6(iVar3->field_0x14,param_1 & 0xffff | uVar3 << 0x10,
                    param_2);
    pass1_1010_1dda(iVar3->field_0x14);
  }
  palette_op_1020_92c4(param_1,0x1020);
  return;
}


fn pass1_1018_5df4(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1018_5cc8(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_57 *  pass1_1018_5e26(astruct_57 *param_1,param_2: u16)

{
  let uVar1: u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfd0,param_2);
  uVar1 = (param_1 >> 0x10);
  param_1 = 0x6128;
  (param_1 + 0x2) = 0x1018;
  (param_1 + 0x74) = 0x1;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_5e5a(param_1: *mut u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x6128;
  (param_1 + 0x2) = 0x1018;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c((astruct_18 *)param_1,&PTR_LOOP_1050_1040);
  return;
}



fn pass1_1018_5e86(param_1: *mut u32)
{
  code **ppcVar1;
  
  ppcVar1 = (code **)(*param_1 + 0x6c);
  (**ppcVar1)();
  return;
}


fn pass1_1018_5ffa(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0x92);
  uVar2 = (iVar4 + 0x94);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0x92) = 0x0;
  pass1_1010_1dda((iVar4 + 0x8e));
  (iVar4 + 0x8e) = 0x0;
  return;
}



fn pass1_1018_6048(param_1: u32) -> u16

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((param_1 + 0x92) + 0x8);
  (**ppcVar1)();
  return 0x0;
}


fn pass1_1018_6102(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1018_5e5a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_6198(param_1: *mut u16,param_2: u32,param_3: u16,uchar *param_4,param_5: i16,
               param_6: u16)

{
  astruct_657 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_657 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_3;
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  &iVar1->field_0x6 = 0x0;
  iVar1->field_0xa = param_2;
  *param_1 = 0x66c0;
  iVar1->field_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,param_6,param_4,param_5);
  iVar1->field_0x6 = puVar2;
  iVar1->field_0x8 = (puVar2 >> 0x10);
  return;
}



fn pass1_1018_620c(param_1: *mut u16)
{
  astruct_509 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_509 *)param_1;
  *param_1 = 0x66c0;
  iVar1->field_0x2 = 0x1018;
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}


fn pass1_1018_642e(param_1: u16,param_2: u16,i16 *param_3,param_4: i16)
{
  *param_3 = 0x64 - param_4 >> 0x1;
  return;
}



u32 
pass1_1018_659a(param_1: u16,param_2: u16,param_3: *mut u16,uchar *param_4,
               param_5: u16)

{
  let piVar1: *mut i16;
  let iStack18: i16;
  let local_6: i16;
  let local_4: i16;
  
  piVar1 = &local_6;
  pass1_1008_3e94(param_3,CONCAT22(param_5,piVar1),
                  CONCAT22(param_5,&local_4));
  mem_op_1000_179c(0xc,param_4,0x1000);
  for (iStack18 = 0x0; iStack18 < 0x3; iStack18 += 0x1) {
    piVar1[iStack18 * 0x2] = (iStack18 * 0x4 + 0x4248) + local_4;
    piVar1[iStack18 * 0x2 + 0x1] = (iStack18 * 0x4 + 0x424a) + local_6;
  }
  return CONCAT22(param_4,piVar1);
}


fn pass1_1018_6630(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let dialog_id_5: u16;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let iStack12: i16;
  SEGPTR SStack10;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  find_n_load_rsrc_1010_4e9e((iVar3 + 0x6),0x1010);
  if ((param_3 | param_2) != 0x0) {
    SStack10 = param_2;
    for (iStack12 = 0x0; iStack12 < 0x9; iStack12 += 0x1) {
      uVar1 = (iVar3 + 0x6);
      dialog_id_5 = pass1_1010_4f20(uVar1,(uVar1 >> 0x10),iStack12)
      ;
      uVar1 = (iVar3 + 0xa);
      set_window_text_1018_6066
                (uVar1,(uVar1 >> 0x10),SStack10,param_3,dialog_id_5
                 ,0x1010);
      uVar2 = str_op_1000_3da4(CONCAT22(param_3,SStack10));
      SStack10 += uVar2 + 0x1;
    }
  }
  return;
}



fn pass1_1018_669a(param_1: u32,param_2: u8) -> u32

{
  pass1_1018_620c((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1018_673c(param_1: *mut u16)
{
  astruct_510 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_510 *)param_1;
  *param_1 = 0x6880;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xe2 = 0x691c;
  iVar1->field_0xe4 = 0x1018;
  pass1_1020_808e(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_6768(param_1: u32,param_2: u16,param_3: u16) -> u16

{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u32;
  
  uVar4 = (param_1 >> 0x10);
  uVar3 = param_1;
  uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
  if (uVar2 != 0x0) {
    ppcVar1 = (code **)((uVar3 + 0xee) + 0x8);
    uVar5 = (**ppcVar1)();
    param_2 = (uVar5 >> 0x10);
    uVar2 = uVar5;
  }
  if ((uVar3 + 0xea) == 0x0) {
    (uVar3 + 0xea) = 0x1;
    uVar5 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar3 + 0x8),0x16,param_2,
                            uVar3,&PTR_LOOP_1050_1038,param_3);
    uVar2 = uVar5;
  }
  return uVar2;
}


fn pass1_1018_681a(Uparam_1: i32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  puVar1 = (param_1 + 0xee);
  uVar2 = (param_1 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  destroy_win_1008_628e(param_1,0x1008);
  return;
}



fn pass1_1018_684c(param_1: *mut u16,param_2: u8) -> u16

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1018_673c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_6924(astruct_658 *param_1,param_2: u16,param_3: u16,param_4: i16,
               param_5: u16)

{
  code **ppcVar1;
  let uVar2: u32;
  let iVar3: i16;
  let extraout_DX: *mut u8
  let uVar4: u16;
  let puVar5: *mut u16;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  &param_1->field_0x14 = 0x0;
  CONCAT22(param_2,param_1) = 0x6a02;
  param_1->field_0x2 = 0x1018;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0xb,param_5,extraout_DX,param_4);
  uVar4 = (puVar5 >> 0x10);
  param_1->field_0x14 = puVar5;
  param_1->field_0x16 = uVar4;
  param_1->field_0x6 = param_1->field_0x14;
  param_1->field_0x8 = uVar4;
  uVar2 = &param_1->field_0x14;
  iVar3 = &param_1->field_0xa;
  ppcVar1 = (code **)((uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_1->field_0x12 = iVar3;
  draw_op_1020_9364((astruct_7 *)CONCAT22(param_2,param_1),0x1020,param_5);
  return;
}



fn pass1_1018_69ac(param_1: *mut u16)
{
  astruct_511 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_511 *)param_1;
  *param_1 = 0x6a02;
  iVar1->field_0x2 = 0x1018;
  if (iVar1->field_0x14 != 0x0) {
    pass1_1010_1dda(iVar1->field_0x14);
  }
  palette_op_1020_92c4(param_1,0x1020);
  return;
}



fn pass1_1018_69dc(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1018_69ac(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1018_6c1e(param_1: *mut u16,param_2: u8)
{
  astruct_512 *uVar1;
  let uVar2: u16;
  
  uVar1 = (astruct_512 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}


fn pass1_1018_7da6(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_7dee(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_7e36(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_7e7e(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_7ec6(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_7f0e(astruct_18 *param_1,param_2: u8)
{
  astruct_671 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_671 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_7f56(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_7f9e(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_7fe6(astruct_18 *param_1,param_2: u8)
{
  astruct_672 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_672 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_802e(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_8076(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_80be(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_8106(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_814e(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_8196(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_81de(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_8226(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_826e(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_82b6(param_1: *mut u16,param_2: u8)
{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1018_82fe(astruct_18 *param_1,param_2: u8)
{
  astruct_517 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_517 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8346(astruct_18 *param_1,param_2: u8)
{
  astruct_518 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_518 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_838e(astruct_18 *param_1,param_2: u8)
{
  astruct_519 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_519 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_83d6(astruct_18 *param_1,param_2: u8)
{
  astruct_673 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_673 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_841e(astruct_18 *param_1,param_2: u8)
{
  astruct_520 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_520 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8466(astruct_18 *param_1,param_2: u8)
{
  astruct_521 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_521 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_84ae(astruct_18 *param_1,param_2: u8)
{
  astruct_522 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_522 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_84f6(astruct_18 *param_1,param_2: u8)
{
  astruct_523 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_523 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_853e(astruct_18 *param_1,param_2: u8)
{
  astruct_524 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_524 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8586(astruct_18 *param_1,param_2: u8)
{
  astruct_525 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_525 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_85ce(astruct_18 *param_1,param_2: u8)
{
  astruct_526 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_526 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8616(astruct_18 *param_1,param_2: u8)
{
  astruct_527 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_527 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_865e(astruct_18 *param_1,param_2: u8)
{
  astruct_528 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_528 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_86a6(astruct_18 *param_1,param_2: u8)
{
  astruct_529 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_529 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_86ee(astruct_18 *param_1,param_2: u8)
{
  astruct_530 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_530 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8736(astruct_18 *param_1,param_2: u8)
{
  astruct_531 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_531 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_877e(astruct_18 *param_1,param_2: u8)
{
  astruct_532 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_532 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_87c6(astruct_18 *param_1,param_2: u8)
{
  astruct_533 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_533 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_880e(astruct_18 *param_1,param_2: u8)
{
  astruct_534 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_534 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8856(astruct_18 *param_1,param_2: u8)
{
  astruct_535 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_535 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_889e(astruct_18 *param_1,param_2: u8)
{
  astruct_536 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_536 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_88e6(astruct_18 *param_1,param_2: u8)
{
  astruct_537 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_537 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_892e(astruct_18 *param_1,param_2: u8)
{
  astruct_538 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_538 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8976(astruct_18 *param_1,param_2: u8)
{
  astruct_539 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_539 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_89be(astruct_18 *param_1,param_2: u8)
{
  astruct_540 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_540 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8a06(astruct_18 *param_1,param_2: u8)
{
  astruct_541 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_541 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8a4e(astruct_18 *param_1,param_2: u8)
{
  astruct_674 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_674 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8a96(astruct_18 *param_1,param_2: u8)
{
  astruct_542 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_542 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8ade(astruct_18 *param_1,param_2: u8)
{
  astruct_543 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_543 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8b26(astruct_18 *param_1,param_2: u8)
{
  astruct_544 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_544 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8b6e(astruct_18 *param_1,param_2: u8)
{
  astruct_545 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_545 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8bb6(astruct_18 *param_1,param_2: u8)
{
  astruct_546 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_546 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8bfe(astruct_18 *param_1,param_2: u8)
{
  astruct_547 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_547 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8c46(astruct_18 *param_1,param_2: u8)
{
  astruct_548 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_548 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8c8e(astruct_18 *param_1,param_2: u8)
{
  astruct_549 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_549 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8cd6(astruct_18 *param_1,param_2: u8)
{
  astruct_675 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_675 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8d1e(astruct_18 *param_1,param_2: u8)
{
  astruct_550 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_550 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8d66(astruct_18 *param_1,param_2: u8)
{
  astruct_551 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_551 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8dae(astruct_18 *param_1,param_2: u8)
{
  astruct_552 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_552 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8df6(astruct_18 *param_1,param_2: u8)
{
  astruct_553 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_553 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8e3e(astruct_18 *param_1,param_2: u8)
{
  astruct_554 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_554 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8e86(astruct_18 *param_1,param_2: u8)
{
  astruct_555 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_555 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8ece(astruct_18 *param_1,param_2: u8)
{
  astruct_676 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_676 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8f16(astruct_18 *param_1,param_2: u8)
{
  astruct_556 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_556 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8f5e(astruct_18 *param_1,param_2: u8)
{
  astruct_677 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_677 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8fa6(astruct_18 *param_1,param_2: u8)
{
  astruct_557 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_557 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_8fee(astruct_18 *param_1,param_2: u8)
{
  astruct_558 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_558 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_9036(astruct_18 *param_1,param_2: u8)
{
  astruct_559 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_559 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_907e(astruct_18 *param_1,param_2: u8)
{
  astruct_560 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_560 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_90c6(astruct_18 *param_1,param_2: u8)
{
  astruct_561 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_561 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_910e(astruct_18 *param_1,param_2: u8)
{
  astruct_562 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_562 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_9156(astruct_18 *param_1,param_2: u8)
{
  astruct_563 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_563 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_919e(astruct_18 *param_1,param_2: u8)
{
  astruct_564 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_564 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_91e6(astruct_18 *param_1,param_2: u8)
{
  astruct_565 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_565 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_922e(astruct_18 *param_1,param_2: u8)
{
  astruct_566 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_566 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_9276(astruct_18 *param_1,param_2: u8)
{
  astruct_567 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_567 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_92be(astruct_18 *param_1,param_2: u8)
{
  astruct_568 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_568 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_9306(astruct_18 *param_1,param_2: u8)
{
  astruct_569 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_569 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_934e(astruct_18 *param_1,param_2: u8)
{
  astruct_570 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_570 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



fn pass1_1018_9396(astruct_18 *param_1,param_2: u8)
{
  astruct_571 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_571 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_c402(astruct_20 *param_1,param_2: u16,param_3: u16,param_4: u16,param_5: u32
               ,param_6: u32,param_7: u32,param_8: u32,param_9: u16,param_10: u16)

{
  let iVar1: i16;
  let puVar2: *mut u16;
  astruct_20 *iVar4;
  let unaff_DI: i16;
  astruct_20 *uVar4;
  let puVar3: *mut u16;
  
  struct_1020_0762(param_1,CONCAT22(param_5,param_4),
                   CONCAT22(param_6,(param_5 >> 0x10)),
                   (param_6 >> 0x10),param_7,param_8,param_9);
  uVar4 = (astruct_20 *)(param_1 >> 0x10);
  iVar4 = (astruct_20 *)param_1;
  iVar4[0x1].field_0x14 = 0x0;
  iVar4[0x1].field_0x16 = 0x0;
  iVar4[0x1].field_0x18 = 0x0;
  iVar4[0x1].field_0x1a = 0x0;
  iVar4[0x1].field_0x1c = 0x2;
  iVar4[0x1].field_0x26 = 0x0;
  iVar4[0x1].field_0x2a = 0x0;
  iVar4[0x1].field_0x2c = 0x1e0190;
  iVar4[0x1].field_0x30 = 0x0;
  param_1->field_0x0 = 0xc8bc;
  iVar4->field_0x2 = 0x1018;
  puVar2 = pass1_1000_4906((astruct_20 *)
                           (param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x1e)),
                           (WNDCLASS16 *)0x0,0x8);
  if ((param_3 == 0x0) || (param_2 != 0x0)) {
    if ((param_2 & param_3) == 0x0) goto LAB_1018_c4bb;
    puVar2 = pass1_1008_5fd8(param_9,param_10);
  }
  else {
    load_string_1010_84ac
              (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),0x1010
              );
  }
  *(u16 **)&iVar4[0x1].field_0x26 = puVar2;
  *(uchar **)(&iVar4[0x1].field_0x26 + 0x2) = param_10;
LAB_1018_c4bb:
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_9,param_10,unaff_DI);
  iVar1 = puVar3;
  iVar4[0x1].field_0x14 = (iVar1 + 0xa);
  iVar4[0x1].field_0x16 = (iVar1 + 0xc);
  pass1_1008_3e94((u16 *)(puVar3 & 0xffff0000 | (iVar1 + 0xe)),
                  (param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x1a))
                  ,
                   (param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x18)));
  return;
}


astruct_29 *  pass1_1018_c896(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_20 * 
pass1_1018_c958(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xf1;
  uVar4 = 0x9a;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x8d);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x732,0x26,CONCAT22(puVar2,0x1f40),
                  CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xdc5a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_c9a6(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xf2;
  uVar4 = 0xa0;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x8e);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x733,0x27,CONCAT22(puVar2,0x1b58),
                  CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xd6de;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_c9f4(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let piVar1: *mut i16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u16;
  let uVar5: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xf3;
  uVar5 = 0xa6;
  puVar4 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x8f);
  uVar2 = (puVar4 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x734,0x28,CONCAT22(puVar4,0x32c8),
                  CONCAT22(uVar3,uVar2),CONCAT22(param_2,uVar5),param_3,param_4,uVar2);
  uVar3 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xda86;
  (param_1 + 0x2) = 0x1018;
  piVar1 = (i16 *)(param_1 + 0x10e);
  *piVar1 = *piVar1 + -0x19;
  return param_1;
}



astruct_20 * 
pass1_1018_ca48(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xf4;
  uVar4 = 0xa1;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x90);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x735,0x29,CONCAT22(puVar2,0x36b0),
                  CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xd50a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_ca96(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let piVar1: *mut i16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u16;
  let uVar5: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xf5;
  uVar5 = 0xbf;
  puVar4 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x92);
  uVar2 = (puVar4 >> 0x10);
  pass1_1018_c402(param_1,0x737,0x736,0x2a,CONCAT22(puVar4,0x6590),
                  CONCAT22(uVar3,uVar2),CONCAT22(param_2,uVar5),param_3,param_4,uVar2);
  uVar3 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xd8b2;
  (param_1 + 0x2) = 0x1018;
  piVar1 = (i16 *)(param_1 + 0x10e);
  *piVar1 = *piVar1 + 0x64;
  return param_1;
}



astruct_20 * 
pass1_1018_caea(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xf6;
  uVar4 = 0x93;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x93);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x738,0x2b,CONCAT22(puVar2,0x2328),
                  CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xdbbe;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_cb38(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xf7;
  uVar4 = 0x94;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x94);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x739,0x2c,CONCAT22(puVar2,0x32c8),
                  CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xd642;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_cb86(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let piVar1: *mut i16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u16;
  let uVar5: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xf8;
  uVar5 = 0xc2;
  puVar4 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x96);
  uVar2 = (puVar4 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x73a,0x2d,CONCAT22(puVar4,0x2328),
                  CONCAT22(uVar3,uVar2),CONCAT22(param_2,uVar5),param_3,param_4,uVar2);
  uVar3 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xd9ea;
  (param_1 + 0x2) = 0x1018;
  piVar1 = (i16 *)(param_1 + 0x10e);
  *piVar1 = *piVar1 + 0x64;
  return param_1;
}



astruct_20 * 
pass1_1018_cbda(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xf9;
  uVar4 = 0xc5;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x97);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x73b,0x2e,CONCAT22(puVar2,0x2af8),
                  CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xd46e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_cc28(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let local_6: [u8;4];
  let uVar3: u16;
  let uVar4: u16;
  
  uVar3 = 0xfa;
  uVar4 = 0xa3;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x98);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x73c,0x2f,CONCAT22(puVar2,0x2710),
                  CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xd816;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_cc76(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xfb;
  uVar4 = 0xa8;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x99);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x73e,0x73d,0x30,CONCAT22(puVar2,0x61a8),
                  CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xdb22;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_ccc4(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xfc;
  uVar4 = 0xa9;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x9b);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x740,0x73f,0x31,CONCAT22(puVar2,0x59d8),
                  CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xd5a6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_cd12(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xfd;
  uVar4 = 0x7c;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x9c);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x741,0x32,CONCAT22(puVar2,0x2710),
                  CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xd94e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_cd60(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xfe;
  uVar4 = 0xc9;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x0);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x0,0x33,CONCAT22(puVar2,0x2710),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xd3d2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}


astruct_20 * 
pass1_1018_cf74(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_6: [u8;4];
  
  uVar3 = 0xfe;
  uVar4 = 0xcf;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(param_4,local_6),0x1,0x80);
  uVar1 = (puVar2 >> 0x10);
  pass1_1018_c402(param_1,0x0,0x0,0x34,CONCAT22(puVar2,0x2710),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3,param_4,uVar1);
  param_1->field_0x0 = 0xd77a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}


astruct_29 *  pass1_1018_d198(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d1be(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d1e4(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d20a(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d230(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d256(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d27c(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d2a2(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d2c8(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d2ee(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d314(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d33a(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d360(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d386(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_29 *  pass1_1018_d3ac(astruct_29 *param_1,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1018_dcf6(param_1: *mut u16) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  *param_1 = 0xdf3c;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 
pass1_1018_dd1e(param_1: u16,param_2: u16,uchar *param_3,param_4: u16,param_5: u16,
               param_6: i16,param_7: u32)

{
  let uVar1: u16;
  let uStack6: u32;
  
  pass1_1010_81f6(0x1010,param_1,_PTR_LOOP_1050_14cc,0x0,param_7._2_2_);
  uStack6 = CONCAT22(param_3,param_2);
  mem_op_1000_179c(0x46,param_3,0x1000);
  uVar1 = param_3 | param_2;
  if (uVar1 == 0x0) {
    param_2 = 0x0;
    uVar1 = 0x0;
  }
  else {
    pass1_1008_87cc((astruct_86 *)CONCAT22(param_3,param_2),param_6,param_7,
                    param_7._2_2_,uStack6,0x0,param_1);
  }
  pass1_1008_8bc6(param_1,uVar1,CONCAT22(uVar1,param_2));
  return CONCAT22(uVar1,param_2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_dd7c(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: u16,
               param_6: u16)

{
  let uVar1: u16;
  let uVar2: u32;
  code **ppcVar3;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let puVar8: *mut u8
  let uVar9: u16;
  let extraout_DX: *mut u8
  let puVar10: *mut u8
  let unaff_DI: i16;
  let puVar11: *mut u16;
  let puVar12: u32;
  let iVar13: i16;
  let uVar14: u16;
  let lStack32: i32;
  let uStack20: u16;
  let uStack12: u16;
  
  uVar5 = param_4._3_1_;
  iVar13 = (param_3 >> 0x10);
  if (param_4._3_1_ == 0x0) {
    puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,param_5,unaff_DI);
    puVar8 = (puVar11 >> 0x10);
    if ((puVar11 + 0x1e) == 0x0) {
      uStack20 = param_4;
      uVar14 = param_4;
    }
    else {
      if (param_4 - 0x7 == 0x0) {
        uStack20 = 0x6;
        param_4._0_2_ = param_4 - 0x7;
      }
      else {
        if (param_4 - 0x8 == 0x0) {
          uStack20 = 0x7;
          param_4._0_2_ = param_4 - 0x8;
        }
        else {
          uStack20 = 0x8;
          param_4._0_2_ = param_4 - 0x9;
        }
      }
      uVar14 = 0x6;
    }
    pass1_1010_81f6(0x1010,param_6,_PTR_LOOP_1050_14cc,0x0,uVar14);
    uVar5 = puVar8 | param_4;
    if ((uVar5 != 0x0) &&
       (puVar10 = puVar8, mem_op_1000_179c(0x46,puVar8,0x1000),
       (puVar10 | uVar5) != 0x0)) {
      pass1_1008_87cc((astruct_86 *)CONCAT22(puVar10,uVar5),param_3,iVar13,uStack20,
                      CONCAT13((char)(puVar8 >> 0x8),
                               CONCAT12((char)puVar8,param_4)),param_4,param_6);
    }
  }
  else {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4,(param_4 >> 0x10));
    puVar12 = struct_op_1030_73a8(CONCAT22(param_5,uVar5));
    uVar9 = (puVar12 >> 0x10);
    uVar4 = puVar12;
    if ((uVar9 | uVar4) != 0x0) {
      uVar2 = (uVar5 + 0x2e);
      uStack12 = uVar2;
      if (((uVar5 + 0x30) | uStack12) == 0x0) {
        lStack32 = 0x0;
      }
      else {
        lStack32 = *(long *)(uStack12 + 0x200);
      }
      uVar5 = (uVar4 + 0x1c);
      uVar1 = (uVar4 + 0x1e);
      uVar6 = uVar1 | uVar5;
      if (uVar6 != 0x0) {
        lStack32 = CONCAT22(uVar1,uVar5);
        uVar6 = uVar5;
      }
      ppcVar3 = (code **)(*puVar12 + 0x14);
      (**ppcVar3)(0x1030,uVar4,uVar9);
      puVar8 = extraout_DX;
      uVar7 = uVar6;
      pass1_1010_81f6(0x1010,param_6,_PTR_LOOP_1050_14cc,lStack32,uVar6);
      puVar10 = puVar8;
      uVar14 = uVar7;
      mem_op_1000_179c(0x46,puVar8,0x1000);
      uVar5 = puVar10 | uVar14;
      if (uVar5 == 0x0) {
        uVar14 = 0x0;
        uVar5 = 0x0;
      }
      else {
        pass1_1008_87cc((astruct_86 *)CONCAT22(puVar10,uVar14),param_3,iVar13,uVar6,
                        CONCAT22(puVar8,uVar7),param_4,param_6);
      }
      pass1_1008_8bc6(param_6,uVar5,
                      CONCAT13((char)(uVar5 >> 0x8),CONCAT12((char)uVar5,uVar14)));
    }
  }
  return;
}



astruct_18 *  pass1_1018_df10(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1018_df92(Uparam_1: i32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  destroy_win_1008_628e(param_1,0x1008);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0xe2);
  uVar2 = (iVar4 + 0xe4);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  (iVar4 + 0xe2) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_dfd4(param_1: u32,uchar *param_2,param_3: i16,param_4: u16,
               param_5: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = param_1;
  delete_palette_1018_e16c((uVar1 + 0xe2),param_4);
  if ((uVar1 + 0xe6) == 0x0) {
    (uVar1 + 0xe6) = 0x1;
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x36,param_5,param_2,param_3);
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar1 + 0x8),0x8,
                    (puVar3 >> 0x10),uVar1,&PTR_LOOP_1050_1038,
                    param_5);
  }
  return;
}



fn pass1_1018_e01c(astruct_18 *param_1,param_2: u8)
{
  astruct_572 *iVar1;
  let uVar1: u16;
  
  iVar1 = (astruct_572 *)param_1;
  iVar1 = iVar1 + 0x1;
  pass1_1008_57c4((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn struct_1018_e100(param_1: *mut u16,param_2: u16,uchar *param_3,param_4: u16) -> u16

{
  astruct_268 *iVar1;
  let unaff_DI: i16;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_268 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_2;
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  &iVar1->field_0x6 = 0x0;
  *param_1 = 0xe228;
  iVar1->field_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x36,param_4,param_3,unaff_DI);
  iVar1->field_0x6 = puVar2;
  iVar1->field_0x8 = (puVar2 >> 0x10);
  return param_1;
}


astruct_18 *  pass1_1018_e1ee(astruct_18 *param_1,param_2: u8)

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x3ab0;
  (param_1 + 0x2) = 0x1008;
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_e230(param_1: u16,astruct_20 *param_2,param_3: u16,param_4: u16)
{
  let extraout_DX: *mut u8
  let uVar1: u16;
  astruct_128 *iVar2;
  let unaff_DI: i16;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  unk_draw_op_1020_7f7a(param_2,0x4,CONCAT22(param_4,param_3));
  uVar2 = (param_2 >> 0x10);
  iVar2 = (astruct_128 *)param_2;
  iVar2->field_0xee = 0x0;
  &iVar2->field_0xf2 = 0x0;
  param_2->field_0x0 = 0xe44e;
  iVar2->field_0x2 = 0x1018;
  iVar2->field_0xe2 = 0xe4ea;
  iVar2->field_0xe4 = 0x1018;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x26,param_1,extraout_DX,unaff_DI);
  uVar1 = (puVar3 >> 0x10);
  iVar2->field_0xf2 = puVar3;
  iVar2->field_0xf4 = uVar1;
  iVar2->field_0xe6 = iVar2->field_0xf2;
  iVar2->field_0xe8 = uVar1;
  return;
}



fn pass1_1018_e2a0(param_1: *mut u16)
{
  astruct_573 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_573 *)param_1;
  *param_1 = 0xe44e;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xe2 = 0xe4ea;
  iVar1->field_0xe4 = 0x1018;
  pass1_1020_808e(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_e2cc(param_1: u32,param_2: u16)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u32;
  let puVar5: *mut u8
  let puVar6: *mut u8
  astruct_269 *iVar7;
  let uVar7: u16;
  let puVar8: *mut u16;
  let puStack10: u32;
  let local_6: [u8;4];
  
  uVar7 = (param_1 >> 0x10);
  iVar7 = (astruct_269 *)param_1;
  if (iVar7->field_0xee != 0x0) {
    ppcVar2 = (code **)(*iVar7->field_0xee + 0x8);
    (**ppcVar2)();
  }
  if (iVar7->field_0xea == 0x0) {
    iVar7->field_0xea = 0x1;
    puVar8 = pass1_1008_941a((u16 *)CONCAT22(param_2,local_6),0x1,0x7a);
    puVar5 = (puVar8 >> 0x10);
    uVar4 = ZEXT24(local_6);
    win_1008_5c9e(_PTR_LOOP_1050_02a0,CONCAT22(param_2,local_6),local_6,puVar5,
                  param_2);
    iVar7->field_0xec = uVar4;
    mem_op_1000_179c(0x112,puVar5,0x1000);
    puVar6 = (puVar5 | uVar4);
    if (puVar6 == 0x0) {
      uVar3 = 0x0;
      puStack10 = 0x0;
    }
    else {
      piVar1 = &iVar7->field_0xcc;
      *piVar1 = *piVar1 + 0x1;
      struct_1020_3644((u16 *)(uVar4 & 0xffff | ZEXT24(puVar5) << 0x10),
                       iVar7->field_0xcc,param_1,param_2);
      uVar3 = uVar4;
      puStack10 = (uVar4 & 0xffff | ZEXT24(puVar6) << 0x10);
    }
    pass1_1008_6978(param_1,0x0,puStack10 & 0xffff0000 | uVar3,uVar3,puVar6)
    ;
    ppcVar2 = (code **)(*puStack10 + 0xc);
    (**ppcVar2)();
  }
  return;
}


fn pass1_1018_e3e8(Uparam_1: i32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  puVar1 = (param_1 + 0xee);
  uVar2 = (param_1 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  destroy_win_1008_628e(param_1,0x1008);
  return;
}



astruct_18 *  pass1_1018_e41a(astruct_18 *param_1,param_2: u8)

{
  param_1 = (astruct_18 *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1018_e2a0((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_e4f2(astruct_659 *param_1,param_2: u16,param_3: u16,param_4: i16,
               param_5: u16)

{
  code **ppcVar1;
  let uVar2: u32;
  let iVar3: i16;
  let extraout_DX: *mut u8
  let uVar4: u16;
  let puVar5: *mut u16;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  &param_1->field_0x14 = 0x0;
  CONCAT22(param_2,param_1) = 0xe5d0;
  param_1->field_0x2 = 0x1018;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x26,param_5,extraout_DX,param_4);
  uVar4 = (puVar5 >> 0x10);
  param_1->field_0x14 = puVar5;
  param_1->field_0x16 = uVar4;
  param_1->field_0x6 = param_1->field_0x14;
  param_1->field_0x8 = uVar4;
  uVar2 = &param_1->field_0x14;
  iVar3 = &param_1->field_0xa;
  ppcVar1 = (code **)((uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_1->field_0x12 = iVar3;
  draw_op_1020_9364((astruct_7 *)CONCAT22(param_2,param_1),0x1020,param_5);
  return;
}



fn pass1_1018_e57a(param_1: *mut u16)
{
  astruct_575 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_575 *)param_1;
  *param_1 = 0xe5d0;
  iVar1->field_0x2 = 0x1018;
  if (iVar1->field_0x14 != 0x0) {
    pass1_1010_1dda(iVar1->field_0x14);
  }
  palette_op_1020_92c4(param_1,0x1020);
  return;
}



astruct_18 *  pass1_1018_e5aa(astruct_18 *param_1,param_2: u8)

{
  pass1_1018_e57a((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_e5dc(param_1: u16,astruct_20 *param_2,param_3: u16,param_4: u16)
{
  let extraout_DX: *mut u8
  let uVar1: u16;
  astruct_20 *iVar2;
  let unaff_DI: i16;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  unk_draw_op_1020_7f7a(param_2,0x9,CONCAT22(param_4,param_3));
  uVar2 = (param_2 >> 0x10);
  iVar2 = (astruct_20 *)param_2;
  &iVar2[0x1].field_0xc = 0x0;
  iVar2[0x1].field_0x10 = 0x0;
  param_2->field_0x0 = 0xe790;
  iVar2->field_0x2 = 0x1018;
  ((astruct_20 *)(iVar2 + 0x1))->field_0x0 = 0xe82c;
  iVar2[0x1].field_0x2 = 0x1018;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0xa,param_1,extraout_DX,unaff_DI);
  uVar1 = (puVar3 >> 0x10);
  &iVar2[0x1].field_0x10 = puVar3;
  (&iVar2[0x1].field_0x10 + 0x2) = uVar1;
  &iVar2[0x1].field_0x4 = &iVar2[0x1].field_0x10;
  (&iVar2[0x1].field_0x4 + 0x2) = uVar1;
  return;
}



fn pass1_1018_e64c(param_1: *mut u16)
{
  astruct_576 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_576 *)param_1;
  *param_1 = 0xe790;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xe2 = 0xe82c;
  iVar1->field_0xe4 = 0x1018;
  pass1_1020_808e(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_e678(param_1: u32,param_2: u16,param_3: u16) -> u16

{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u32;
  
  uVar4 = (param_1 >> 0x10);
  uVar3 = param_1;
  uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
  if (uVar2 != 0x0) {
    ppcVar1 = (code **)((uVar3 + 0xee) + 0x8);
    uVar5 = (**ppcVar1)();
    param_2 = (uVar5 >> 0x10);
    uVar2 = uVar5;
  }
  if ((uVar3 + 0xea) == 0x0) {
    (uVar3 + 0xea) = 0x1;
    uVar5 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar3 + 0x8),0x15,param_2,
                            uVar3,&PTR_LOOP_1050_1038,param_3);
    uVar2 = uVar5;
  }
  return uVar2;
}


fn pass1_1018_e72a(Uparam_1: i32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  puVar1 = (param_1 + 0xee);
  uVar2 = (param_1 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  destroy_win_1008_628e(param_1,0x1008);
  return;
}



astruct_18 *  pass1_1018_e75c(astruct_18 *param_1,param_2: u8)

{
  param_1 = (astruct_18 *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1018_e64c((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1018_e834(astruct_660 *param_1,param_2: u16,param_3: u16,param_4: i16,
               param_5: u16)

{
  code **ppcVar1;
  let uVar2: u32;
  let iVar3: i16;
  let extraout_DX: *mut u8
  let uVar4: u16;
  let puVar5: *mut u16;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  &param_1->field_0x14 = 0x0;
  CONCAT22(param_2,param_1) = 0xe912;
  param_1->field_0x2 = 0x1018;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0xa,param_5,extraout_DX,param_4);
  uVar4 = (puVar5 >> 0x10);
  param_1->field_0x14 = puVar5;
  param_1->field_0x16 = uVar4;
  param_1->field_0x6 = param_1->field_0x14;
  param_1->field_0x8 = uVar4;
  uVar2 = &param_1->field_0x14;
  iVar3 = &param_1->field_0xa;
  ppcVar1 = (code **)((uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_1->field_0x12 = iVar3;
  draw_op_1020_9364((astruct_7 *)CONCAT22(param_2,param_1),0x1020,param_5);
  return;
}



fn pass1_1018_e8bc(param_1: *mut u16)
{
  astruct_577 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_577 *)param_1;
  *param_1 = 0xe912;
  iVar1->field_0x2 = 0x1018;
  if (iVar1->field_0x14 != 0x0) {
    pass1_1010_1dda(iVar1->field_0x14);
  }
  palette_op_1020_92c4(param_1,0x1020);
  return;
}



astruct_18 *  pass1_1018_e8ec(astruct_18 *param_1,param_2: u8)

{
  pass1_1018_e8bc((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_e91e(astruct_20 *param_1,param_2: u16,param_3: u16,param_4: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let puVar3: *mut u16;
  let extraout_DX: *mut u8
  let puVar4: *mut u8
  let uVar5: u16;
  let unaff_DI: i16;
  let puVar6: *mut u16;
  let uVar7: u16;
  astruct_127 *iVar7;
  
  iVar7 = (astruct_127 *)param_1;
  uVar7 = (param_1 >> 0x10);
  unk_draw_op_1020_7f7a(param_1,0x3,CONCAT22(param_3,param_2));
  iVar7->field_0xee = 0x0;
  &iVar7->field_0xf2 = 0x0;
  iVar7->field_0xf6 = 0x0;
  param_1->field_0x0 = 0xebd0;
  iVar7->field_0x2 = 0x1018;
  iVar7->field_0xe2 = 0xec6c;
  iVar7->field_0xe4 = 0x1018;
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x28,param_4,extraout_DX,unaff_DI);
  puVar4 = (puVar6 >> 0x10);
  iVar7->field_0xf2 = puVar6;
  iVar7->field_0xf4 = puVar4;
  iVar7->field_0xe6 = iVar7->field_0xf2;
  iVar7->field_0xe8 = puVar4;
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_4,puVar4,unaff_DI);
  &iVar7->field_0xf6 = puVar6;
  (&iVar7->field_0xf6 + 0x2) = (puVar6 >> 0x10);
  if (param_1 == (astruct_20 *)0x0) {
    puVar3 = 0x0;
    uVar5 = 0x0;
  }
  else {
    puVar3 = &iVar7->field_0xe2;
    uVar5 = uVar7;
  }
  puVar1 = iVar7->field_0xf6;
  ppcVar2 = (code **)(*iVar7->field_0xf6 + 0x4);
  (**ppcVar2)(0x1010,puVar1,(puVar1 >> 0x10),0xb,puVar3,uVar5);
  return;
}



fn pass1_1018_e9de(param_1: *mut u16)
{
  astruct_578 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_578 *)param_1;
  *param_1 = 0xebd0;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xe2 = 0xec6c;
  iVar1->field_0xe4 = 0x1018;
  pass1_1020_808e(param_1);
  return;
}


fn pass1_1018_ea66(param_1: u32,param_2: u16)
{
  code **ppcVar1;
  let puVar2: *mut u8;
  let iVar3: i16;
  let uVar4: u16;
  let puVar5: *mut u16;
  let local_6: [u8;4];
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (*(long *)(iVar3 + 0xee) != 0x0) {
    ppcVar1 = (code **)((iVar3 + 0xee) + 0x8);
    (**ppcVar1)();
  }
  if ((iVar3 + 0xea) == 0x0) {
    (iVar3 + 0xea) = 0x1;
    puVar5 = pass1_1008_941a((u16 *)CONCAT22(param_2,local_6),0x1,0x95);
    puVar2 = local_6;
    win_1008_5c9e(_PTR_LOOP_1050_02a0,CONCAT22(param_2,puVar2),puVar2,
                  (puVar5 >> 0x10),param_2);
    (iVar3 + 0xec) = puVar2;
    unk_win_op_1010_7300((iVar3 + 0xf6),0x0,0x8,0x0);
  }
  return;
}


fn pass1_1018_eb3e(Uparam_1: i32,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  puVar1 = (iVar6 + 0xee);
  uVar2 = (iVar6 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  if (*(long *)(iVar6 + 0xf6) != 0x0) {
    if (param_1 == 0x0) {
      iVar4 = 0x0;
      uVar5 = 0x0;
    }
    else {
      iVar4 = iVar6 + 0xe2;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((iVar6 + 0xf6),CONCAT22(uVar5,iVar4),param_2);
  }
  destroy_win_1008_628e(param_1,0x1008);
  return;
}



astruct_18 *  pass1_1018_eb9c(astruct_18 *param_1,param_2: u8)

{
  param_1 = (astruct_18 *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1018_e9de((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1018_ec74(astruct_661 *param_1,param_2: i16,param_3: u16,param_4: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let uVar4: u16;
  let extraout_DX: *mut u8
  let puVar5: *mut u8
  let puVar6: *mut u16;
  let uVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  astruct_661 *paVar10;
  let iVar11: i16;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0x14 = 0x0;
  pass1_1008_3e38((u16 *)CONCAT22(param_2,&param_1->field_0x18));
  puVar6 = pass1_1008_3e38((u16 *)CONCAT22(param_2,&param_1->field_0x1e));
  &param_1->field_0x24 = 0x0;
  CONCAT22(param_2,param_1) = 0x1cc;
  param_1->field_0x2 = 0x1020;
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x28,param_4,
                           (puVar6 >> 0x10),param_2);
  uVar4 = (puVar6 >> 0x10);
  &param_1->field_0x14 = puVar6;
  (&param_1->field_0x14 + 0x2) = uVar4;
  uVar9 = 0x0;
  uVar8 = &param_1->field_0x14;
  ppcVar2 = (code **)(*param_1->field_0x14 + 0x4);
  paVar10 = param_1;
  iVar11 = param_2;
  (**ppcVar2)();
  param_1->field_0x6 = param_1->field_0x14;
  puVar1 = param_1->field_0x14;
  pass1_1010_2b50(puVar1,(puVar1 >> 0x10),
                  CONCAT22(param_2,&param_1->field_0x18));
  uVar7 = pass1_1010_2b66(param_1->field_0x14);
  param_1->field_0x24 = uVar7;
  param_1->field_0x26 = (uVar7 >> 0x10);
  puVar1 = param_1->field_0x14;
  puVar1 = (puVar1 + 0xa);
  uVar3 = CONCAT22(param_2,&param_1->field_0xa);
  ppcVar2 = (code **)(*puVar1 + 0x8);
  (**ppcVar2)(0x1010,puVar1,(puVar1 >> 0x10),uVar3,uVar8,uVar4,uVar9,
              paVar10,iVar11);
  param_1->field_0x12 = uVar3;
  puVar5 = extraout_DX;
  draw_op_1020_9364((astruct_7 *)CONCAT22(param_2,param_1),0x1020,param_4);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,puVar5,param_2);
  pass1_1008_3f62((u16 *)CONCAT22(param_2,&param_1->field_0x1e),
                  (puVar6 & 0xffff0000 | (puVar6 + 0xe)));
  pass1_1008_3f32((i16 *)CONCAT22(param_2,&param_1->field_0x18),
                  (i16 *)CONCAT22(param_2,&param_1->field_0x1e));
  return;
}



fn pass1_1018_ed98(param_1: *mut u16,param_2: u16)
{
  astruct_579 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_579 *)param_1;
  *param_1 = 0x1cc;
  iVar1->field_0x2 = 0x1020;
  if (iVar1->field_0x14 != 0x0) {
    pass1_1010_1ea6(iVar1->field_0x14,param_1 & 0xffff | uVar1 << 0x10,
                    param_2);
    pass1_1010_1dda(iVar1->field_0x14);
  }
  palette_op_1020_92c4(param_1,0x1020);
  return;
}


astruct_18 * 
pass1_1020_01a6(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  pass1_1018_ed98((u16 *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void 
pass1_1020_01d8(astruct_20 *param_1,param_2: u16,param_3: u16,param_4: u16,
               param_5: u16,param_6: u16,param_7: u16,param_8: u32,param_9: u16)

{
  unk_draw_op_1008_61b2
            ((astruct_20 *)CONCAT22(param_2,param_1),param_3,param_7,param_8,param_9);
  (param_1 + 0x1) = 0x0;
  param_1[0x1].field_0x4 = 0x0;
  param_1[0x1].field_0x8 = param_6;
  param_1[0x1].field_0xa = param_5;
  param_1[0x1].field_0xc = param_4;
  CONCAT22(param_2,param_1) = 0x45a;
  param_1->field_0x2 = 0x1020;
  return;
}



fn pass1_1020_022c(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_580 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_580 *)param_1;
  *param_1 = 0x45a;
  iVar4->field_0x2 = 0x1020;
  puVar1 = iVar4->field_0xe6;
  uVar2 = iVar4->field_0xe8;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1008_57c4((u16 *)
                  (param_1 & 0xffff0000 | &iVar4->field_0xd2));
  *param_1 = 0x380a;
  iVar4->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar4->field_0x2 = 0x1008;
  return;
}



fn pass1_1020_028c(param_1: u16,param_2: u16,param_3: i16)
{
  pass1_1010_3c9e(*(long *)(param_1 + 0xe2));
  pass1_1008_68c6(param_1,param_2,param_3,0x1008);
  return;
}



fn pass1_1020_02ae(Uparam_1: i32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  pass1_1010_3cd0(*(long *)(iVar4 + 0xe2));
  destroy_win_1008_628e(param_1,0x1008);
  puVar1 = (iVar4 + 0xe6);
  uVar2 = (iVar4 + 0xe8);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  (iVar4 + 0xe6) = 0x0;
  pass1_1010_1dda((iVar4 + 0xe2));
  (iVar4 + 0xe2) = 0x0;
  return;
}


astruct_18 *  pass1_1020_0434(astruct_18 *param_1,param_2: u8)

{
  pass1_1020_022c((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_04f6(param_1: *mut u16,param_2: u16,uchar *param_3,param_4: i16,param_5: u16)
{
  code **ppcVar1;
  let iVar2: i16;
  let uVar3: u16;
  let extraout_DX: *mut u8
  astruct_662 *iVar4;
  let uVar4: u16;
  let puVar5: *mut u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_662 *)param_1;
  *param_1 = 0x389a;
  iVar4->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar4->field_0x2 = 0x1008;
  iVar4->field_0x4 = param_2;
  *param_1 = 0x3ab0;
  iVar4->field_0x2 = 0x1008;
  iVar4->field_0x6 = 0x0;
  iVar4->field_0xa = 0x0;
  iVar4->field_0xc = 0x0;
  iVar4->field_0xe = 0x0;
  iVar4->field_0x10 = 0x0;
  *param_1 = 0x75a;
  iVar4->field_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x1,param_5,param_3,param_4);
  uVar3 = (puVar5 >> 0x10);
  &iVar4->field_0x6 = puVar5;
  (&iVar4->field_0x6 + 0x2) = uVar3;
  ppcVar1 = (code **)(*iVar4->field_0x6 + 0x4);
  (**ppcVar1)(0x1010,&iVar4->field_0x6,uVar3,0x0,param_1);
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_5,extraout_DX,param_4);
  iVar2 = puVar5;
  iVar4->field_0xa = (iVar2 + 0xa);
  iVar4->field_0xc = (iVar2 + 0xc);
  pass1_1008_3e94((u16 *)(puVar5 & 0xffff0000 | (iVar2 + 0xe)),
                  (param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x10)),
                  (param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0xe)));
  return;
}



fn pass1_1020_05d6(param_1: *mut u16,param_2: u16)
{
  astruct_581 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_581 *)param_1;
  *param_1 = 0x75a;
  iVar1->field_0x2 = 0x1020;
  if (iVar1->field_0x6 != 0x0) {
    pass1_1010_1ea6(iVar1->field_0x6,param_1 & 0xffff | uVar1 << 0x10,
                    param_2);
  }
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}


astruct_18 * 
pass1_1020_0734(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  pass1_1020_05d6((u16 *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_07aa(param_1: u32,param_2: u16,param_3: i16,param_4: u16,param_5: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  let local_16: [u8;14];
  
  draw_op_1020_041e(param_1,param_4);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0xf0) == 0x0) {
    (iVar1 + 0xf0) = 0x1;
    pass1_1008_5bdc((astruct_79 *)CONCAT22(param_5,local_16),param_3,param_5);
    win_1008_5c9e(CONCAT22(param_5,local_16),
                  (param_1 & 0xffff0000 | (iVar1 + 0xf2)),local_16,param_2
                  ,param_5);
    pass1_1008_5c34((u16 *)CONCAT22(param_5,local_16));
  }
  return;
}



astruct_18 *  pass1_1020_07f4(astruct_18 *param_1,param_2: u8)

{
  pass1_1020_022c((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_08b6(WNDCLASS16 *param_1,astruct_20 *param_2,param_3: u16,Uparam_4: i32)
{
  astruct_20 *iVar1;
  let uVar1: u16;
  astruct_20 *paVar2;
  
  paVar2 = unk_draw_op_1008_61b2(param_2,0x1,param_3,param_4,param_1);
  uVar1 = (param_2 >> 0x10);
  iVar1 = (astruct_20 *)param_2;
  &iVar1[0x1].field_0x4 = 0x0;
  (&iVar1[0x1].field_0x4 + 0x2) = 0x0;
  param_2->field_0x0 = 0xb0e;
  iVar1->field_0x2 = 0x1020;
  win_1008_5c5c(param_1,0x0,(paVar2 >> 0x10),_PTR_LOOP_1050_02a0,0x1d4);
  return;
}


fn pass1_1020_0a0c(Uparam_1: i32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  destroy_win_1008_628e(param_1,0x1008);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0xe2);
  uVar2 = (iVar4 + 0xe4);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  (iVar4 + 0xe2) = 0x0;
  (iVar4 + 0xe6) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_0a52(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u32;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = param_1;
  unk_draw_op_1020_0c3e((uVar1 + 0xe2),param_3);
  if ((uVar1 + 0xe6) == 0x0) {
    (uVar1 + 0xe6) = 0x1;
    (_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
    uVar3 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar1 + 0x8),0x6,param_2,uVar1
                            ,&PTR_LOOP_1050_1038,param_4);
    (uVar1 + 0xe8) = uVar3;
    (uVar1 + 0xea) = (uVar3 >> 0x10);
  }
  return;
}


fn pass1_1020_0aa6(param_1: u32,param_2: u16)
{
  win_ui_palette_op_1020_0cd2((param_1 + 0xe2),param_2);
  return;
}



fn pass1_1020_0abc(param_1: u32)
{
  code **ppcVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0xe6) != 0x0) {
    ppcVar1 = (code **)((param_1 + 0xe8) + 0x10);
    (**ppcVar1)();
  }
  return;
}



astruct_63 * 
pass1_1020_0ae8(astruct_63 *param_1,param_2: u8,param_3: u16)

{
  send_win_msg_1020_08fe(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}