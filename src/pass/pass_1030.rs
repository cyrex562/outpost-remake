
void 
pass1_1030_10b0(param_1: u16,param_2: u16,param_3: u16,param_4: u32,
               param_5: u32,uchar *param_6,astruct_179 *param_7,param_8: u16,
               param_9: u16,param_10: u16)

{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: *mut u16;
  let uStack8: u16;
  
  puVar6 = switch_1030_07ac(param_1,param_2,param_3,param_4,
                            (param_4 >> 0x10),param_5,param_6,param_7,
                            param_8,param_9,param_10);
  uVar3 = (puVar6 >> 0x10);
  uVar1 = (puVar6 + 0x4);
  uVar2 = uVar1;
  uVar4 = uVar3;
  pass1_1028_e1ec(CONCAT22(param_2,param_1),param_5,(param_5 >> 0x10));
  uVar5 = uVar4 | uVar2;
  if (uVar5 != 0x0) {
    pass1_1030_7e5a(uVar2 & 0xffff | uVar4 << 0x10,uVar1,uVar5);
  }
  uStack8 = (uVar1 >> 0x10);
  pass1_1030_1358((param_1 + 0x26),puVar6,uVar3,
                  uVar1 & 0xffff | (uStack8 & 0xff) << 0x10,param_10);
  return;
}



fn pass1_1030_1120(param_1: u32,param_2: u16,uchar *param_3,param_4: u16)
{
  let puVar1: *mut u8
  
  mem_op_1000_179c(0x3b2,param_3,0x1000);
  puVar1 = (uchar *)(param_3 | param_2);
  if (puVar1 == (uchar *)0x0) {
    param_2 = 0x0;
    puVar1 = (uchar *)0x0;
  }
  else {
    struct_1030_2112((u16 *)CONCAT22(param_3,param_2),0x0,param_2,puVar1);
  }
  pass1_1030_1358((param_1 + 0x2a),param_2,puVar1,
                  (param_2 + 0x4) & 0xffff |
                  ((param_2 + 0x6) & 0xff) << 0x10,param_4);
  return;
}



astruct_18 *  pass1_1030_117a(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1030_1244(param_1: *mut u16)
{
  let puVar1: u32;
  let puVar2: u32;
  let uVar3: u16;
  code **ppcVar4;
  astruct_18 *paVar5;
  astruct_606 *iVar6;
  let iVar7: i16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let uStack6: u32;
  
  uVar9 = (param_1 >> 0x10);
  iVar6 = (astruct_606 *)param_1;
  *param_1 = (s_462_bmp_1050_1620 + 0x4);
  iVar6->field_0x2 = 0x1030;
  if (iVar6->field_0x1a != 0x0) {
    uStack6 = 0x1;
    while( true ) {
      puVar1 = &iVar6->field_0xa;
      if (*puVar1 < uStack6 || *puVar1 == uStack6) break;
      iVar8 = uStack6 * 0x4;
      paVar5 = iVar6->field_0x6;
      uVar10 = (paVar5 >> 0x10);
      iVar7 = paVar5;
      puVar2 = (iVar7 + iVar8);
      uVar3 = (iVar7 + iVar8 + 0x2);
      if ((uVar3 | puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      uStack6 += 0x1;
    }
  }
  fn_ptr_1000_17ce(iVar6->field_0x6,0x1000);
  *param_1 = 0x389a;
  iVar6->field_0x2 = 0x1008;
  return;
}



fn pass1_1030_12ca(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u32;
  astruct_176 *iVar3;
  let uVar3: u16;
  let uStack6: u32;
  
  uStack6 = 0x1;
  while( true ) {
    uVar3 = (param_1 >> 0x10);
    iVar3 = (astruct_176 *)param_1;
    puVar1 = &iVar3->field_0xa;
    if (*puVar1 < uStack6 || *puVar1 == uStack6) {
      iVar3->field_0x4 = 0x0;
      return;
    }
    uVar2 = iVar3->field_0x6;
    if (*(long *)(uVar2 + uStack6 * 0x4) == 0x0) break;
    uStack6 += 0x1;
  }
  return;
}



fn pass1_1030_1358(param_1: u32,param_2: u16,param_3: u16,param_4: u32,param_5: u16)
{
  let puVar1: u32;
  let puVar2: *mut u16;
  let lVar3: i32;
  astruct_291 *iVar4;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  let bVar8: bool;
  
  if (param_4 == 0x0) {
    return;
  }
  uVar6 = (param_1 >> 0x10);
  iVar4 = (astruct_291 *)param_1;
  puVar1 = &iVar4->field_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4->field_0x6 == 0x0)) {
    puVar2 = (u16 *)(&iVar4->field_0x12 + 0x2);
    bVar8 = *puVar2 < param_4._2_2_;
    if ((bVar8 || *puVar2 == param_4._2_2_) &&
       ((bVar8 || (puVar1 = &iVar4->field_0x12,
                  puVar1 < param_4 || puVar1 == param_4))))
    {
      struct_1030_1550(param_1 & 0xffff | uVar6 << 0x10,param_5);
    }
    puVar1 = &iVar4->field_0x12;
    if (*puVar1 < param_4 || *puVar1 == param_4) {
      return;
    }
    if (iVar4->field_0x6 == 0x0) {
      return;
    }
    puVar2 = &iVar4->field_0xc;
    bVar8 = *puVar2 < param_4._2_2_;
    if ((bVar8 || *puVar2 == param_4._2_2_) &&
       ((bVar8 || (puVar2 = &iVar4->field_0xa,
                  *puVar2 < param_4 || *puVar2 == param_4)))) {
      iVar4->field_0xa = (param_4 + 0x1);
      iVar4->field_0xc = (param_4 + 0x1 >> 0x10);
    }
  }
  lVar3 = iVar4->field_0x6;
  uVar7 = (lVar3 >> 0x10);
  iVar5 = lVar3;
  (iVar5 + param_4 * 0x4) = param_2;
  (iVar5 + param_4 * 0x4 + 0x2) = param_3;
  return;
}



fn pass1_1030_13f6(param_1: u32,param_2: u32,param_3: u16,param_4: u16,param_5: u16) -> u16

{
  code **ppcVar1;
  let uVar2: u16;
  let puStack8: u32;
  let uStack4: u16;
  
  uStack4 = 0x0;
  bad_1030_1312();
  puStack8 = CONCAT22(param_4,param_3);
  if ((param_4 | param_3) != 0x0) {
    uStack4 = 0x1;
    uVar2 = (param_1 >> 0x10);
    if (((param_1 + 0x1a) != 0x0) && ((param_4 | param_3) != 0x0)) {
      ppcVar1 = (code **)*puStack8;
      (**ppcVar1)();
    }
    pass1_1030_1358(param_1,0x0,0x0,param_2,param_5);
    (param_1 + 0x4) = 0x1;
  }
  return uStack4;
}



fn pass1_1030_145a(param_1: u32,param_2: i32)
{
  let uVar1: u32;
  let uVar2: u16;
  astruct_346 *iVar4;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar4 = (astruct_346 *)param_1;
  fn_ptr_1000_17ce((astruct_18 *)iVar4->field_0x6,0x1000);
  iVar4->field_0x6 = 0x0;
  iVar4->field_0xa = 0x0;
  uVar1 = iVar4->field_0x16 + param_2;
  uVar2 = (uVar1 >> 0x10);
  if (uVar1 < iVar4->field_0xe) {
    uVar1 = &iVar4->field_0xe;
    uVar2 = (&iVar4->field_0xe + 0x2);
  }
  &iVar4->field_0xe = uVar1;
  (&iVar4->field_0xe + 0x2) = uVar2;
  iVar4->field_0x12 = 0x0;
  return;
}



fn pass1_1030_14b4(param_1: u32,param_2: u16,param_3: u16,param_4: u32,param_5: u16)
{
  let puVar1: u32;
  let puVar2: *mut u16;
  let lVar3: i32;
  astruct_345 *iVar5;
  astruct_344 *iVar4;
  let uVar4: u16;
  let uVar5: u16;
  let bVar6: bool;
  
  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_345 *)param_1;
  puVar1 = &iVar5->field_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar5->field_0x6 == 0x0)) {
    puVar2 = (u16 *)(&iVar5->field_0x12 + 0x2);
    bVar6 = *puVar2 < param_4._2_2_;
    if ((bVar6 || *puVar2 == param_4._2_2_) &&
       ((bVar6 || (puVar1 = &iVar5->field_0x12,
                  puVar1 < param_4 || puVar1 == param_4))))
    {
      struct_1030_1550(param_1 & 0xffff | uVar4 << 0x10,param_5);
    }
    puVar1 = &iVar5->field_0x12;
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar5->field_0x6 == 0x0)) {
      return;
    }
    puVar2 = &iVar5->field_0xc;
    bVar6 = *puVar2 < param_4._2_2_;
    if ((bVar6 || *puVar2 == param_4._2_2_) &&
       ((bVar6 || (puVar2 = &iVar5->field_0xa,
                  *puVar2 < param_4 || *puVar2 == param_4)))) {
      iVar5->field_0xa = (param_4 + 0x1);
      iVar5->field_0xc = (param_4 + 0x1 >> 0x10);
    }
  }
  lVar3 = iVar5->field_0x6;
  uVar5 = (lVar3 >> 0x10);
  iVar4 = (astruct_344 *)lVar3;
  (iVar4 + param_4 * 0x4) = param_2;
  (iVar4 + param_4 * 0x4 + 0x2) = param_3;
  return;
}



fn pass1_1030_154c(void)
{
  return;
}


astruct_18 *  pass1_1030_15fe(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_1244((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_165e(param_1: *mut u16,param_2: u32,param_3: u32,param_4: u16)
{
  astruct_175 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_175 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  &iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = param_3;
  *param_1 = 0x17ba;
  iVar1->field_0x2 = 0x1030;
  pass1_1030_5c8a(_PTR_LOOP_1050_5736,param_2);
  iVar1->field_0x4 = param_3;
  iVar1->field_0x6 = param_4;
  return;
}



fn pass1_1030_16b2(param_1: *mut u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x17ba;
  (param_1 + 0x2) = 0x1030;
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  return;
}



fn pass1_1030_16d6(param_1: u32,param_2: u32,param_3: u16)
{
  let BVar1: bool;
  let uVar2: u16;
  let uVar3: u16;
  let local_10: [u32;0x2];
  let local_8: u32;
  
  uVar2 = (param_1 >> 0x10);
  local_10[0] = (param_1 + 0x4);
  uVar3 = (param_2 >> 0x10);
  BVar1 = write_to_file_1008_7e1c
                    (param_2,uVar3,local_10,param_3,0x4,0x1008);
  if (BVar1 != 0x0) {
    local_8 = (param_1 + 0x8);
    BVar1 = write_to_file_1008_7e1c
                      (param_2,uVar3,&local_8,param_3,0x4,0x1008);
    if (BVar1 != 0x0) {
      return;
    }
  }
  PTR_LOOP_1050_0310 = 0x6d0;
  return;
}



fn pass1_1030_177a(param_1: u32,param_2: u32)
{
  (param_1 + 0x8) = param_2;
  return;
}



astruct_18 *  pass1_1030_1794(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_16b2((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


u16 * 
pass1_1030_183c(param_1: *mut u16,param_2: u32,param_3: u32,param_4: u32,param_5: u32,
               param_6: u16,uchar *param_7)

{
  let uVar1: u32;
  let uVar2: u16;
  astruct_351 *iVar2;
  
  iVar2 = (astruct_351 *)param_1;
  uVar2 = (param_1 >> 0x10);
  pass1_1030_165e(param_1,param_4,param_5,param_7);
  &iVar2->field_0xc = 0x0;
  *param_1 = 0x1a16;
  iVar2->field_0x2 = 0x1030;
  if ((param_3 != 0x0) || (param_2 != 0x0)) {
    mem_op_1000_179c(0x18,param_7,0x1000);
    if ((param_7 | param_6) == 0x0) {
      uVar1 = 0x0;
    }
    else {
      uVar1 = struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_7,param_6),param_2,param_3)
      ;
    }
    iVar2->field_0xc = uVar1;
    iVar2->field_0xe = (uVar1 >> 0x10);
  }
  return param_1;
}



fn pass1_1030_18b2(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x1a16;
  (iVar4 + 0x2) = 0x1030;
  puVar1 = (iVar4 + 0xc);
  uVar2 = (iVar4 + 0xe);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1030_16b2(param_1);
  return;
}



fn pass1_1030_18f0(param_1: u32,param_2: i16,param_3: i16,param_4: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let extraout_DX: u16;
  let extraout_DX_00: i16;
  let iVar3: i16;
  let uVar4: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (*(long *)(iVar3 + 0xc) != 0x0) {
    ppcVar1 = (code **)((iVar3 + 0xc) + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX,param_4);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
      ppcVar1 = (code **)((iVar3 + 0xc) + 0x4);
      uVar2 = uStack6;
      (**ppcVar1)();
      if ((uVar2 == param_2) && (extraout_DX_00 == param_3)) {
        ppcVar1 = (code **)((iVar3 + 0xc) + 0x8);
        (**ppcVar1)();
      }
    }
  }
  return;
}



fn pass1_1030_1972(void) -> u16

{
  return 0x1;
}



fn pass1_1030_1978(param_1: u32,param_2: u32,param_3: u16,param_4: u16) -> u16

{
  pass1_1030_16d6(param_1,param_2,param_4);
  if (param_3 != 0x0) {
    write_to_file_1008_7954
              (param_2,(param_1 + 0xc),param_3,0x1008,
               param_4);
    if (param_3 == 0x0) {
      PTR_LOOP_1050_0310 = 0x6d0;
      return param_3;
    }
    param_3 = 0x1;
  }
  return param_3;
}


astruct_18 *  pass1_1030_19f0(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_18b2((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1030_1a32(param_1: *mut u16,param_2: u16,uchar *param_3) -> u16

{
  pass1_1030_183c(param_1,0x1,0x16,0xff000000,0x0,param_2,param_3);
  PTR_LOOP_1050_5168 = (param_1 >> 0x10);
  PTR_LOOP_1050_5166 = param_1;
  (PTR_LOOP_1050_5166 + 0x10) = 0x0;
  *param_1 = 0x1cbc;
  (PTR_LOOP_1050_5166 + 0x2) = 0x1030;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_1a74(param_1: *mut u16)
{
  *param_1 = 0x1cbc;
  (param_1 + 0x2) = 0x1030;
  _PTR_LOOP_1050_5166 = 0x0;
  pass1_1030_18b2(param_1);
  return;
}



fn pass1_1030_1a9c(param_1: u32,param_2: u32,param_3: u16) -> u16

{
  let uVar1: u32;
  let piVar2: *mut i16;
  u16_t in_AX;
  let uVar3: u16;
  let Bvar4: bool;
  let iVar5: i16;
  let uVar6: u16;
  let local_c: [u16;0x5];
  
  uVar3 = pass1_1030_1978(param_1,param_2,in_AX,param_3);
  if (uVar3 != 0x0) {
    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    local_c[0] = (iVar5 + 0x10);
    uVar3 = (param_2 >> 0x10);
    BVar4 = write_to_file_1008_7e1c
                      (param_2,uVar3,local_c,param_3,0x2,0x1008);
    if (BVar4 != 0x0) {
      if (**(i16 **)(iVar5 + 0x10) == 0x0) {
        return 0x1;
      }
      piVar2 = *(i16 **)(iVar5 + 0x10);
      uVar1 = (piVar2 + 0x2);
      BVar4 = write_to_file_1008_7e1c
                        (param_2,uVar3,uVar1,
                         (uVar1 >> 0x10),
                         (*piVar2 * 0x2),0x1008);
      if (BVar4 != 0x0) {
        return 0x1;
      }
    }
    PTR_LOOP_1050_0310 = 0x6d0;
  }
  return 0x0;
}


fn pass1_1030_1be2(param_1: u32,param_2: u16,uchar *param_3)
{
  code **ppcVar1;
  let puVar2: *mut u16;
  let puVar3: *mut u8
  let extraout_DX: *mut u8
  let extraout_DX_00: *mut u8
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u32;
  let uStack4: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar3 = param_3;
  if (*(long *)(iVar4 + 0xc) == 0x0) {
    mem_op_1000_179c(0x18,param_3,0x1000);
    puVar3 = (uchar *)(param_3 | param_2);
    if (puVar3 == (uchar *)0x0) {
      (iVar4 + 0xc) = 0x0;
    }
    else {
      struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_3,param_2),0x5,0x5);
      (iVar4 + 0xc) = param_2;
      *(uchar **)(iVar4 + 0xe) = extraout_DX;
      puVar3 = extraout_DX;
    }
  }
  for (uStack4 = 0x0; puVar2 = *(u16 **)(iVar4 + 0x10),
      uStack4 <= *puVar2 && *puVar2 != uStack4; uStack4 += 0x1) {
    uVar6 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,puVar3,0x1);
    ppcVar1 = (code **)((iVar4 + 0xc) + 0x8);
    (**ppcVar1)(&USHORT_1050_1028,(iVar4 + 0xc),uVar6,
                (uVar6 >> 0x10),uStack4,0x0);
    puVar3 = extraout_DX_00;
  }
  return;
}



astruct_18 *  pass1_1030_1c96(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_1a74((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1030_1d28(param_1: *mut u16)
{
  astruct_594 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_594 *)param_1;
  *param_1 = 0x2044;
  iVar1->field_0x2 = 0x1030;
  fn_ptr_1000_17ce(iVar1->field_0x4,0x1000);
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_1d58(param_1: u32)
{
  code **ppcVar1;
  let uVar2: u32;
  
  ppcVar1 = (code **)(param_1 + 0x4);
  uVar2 = (**ppcVar1)();
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uVar2 >> 0x10));
  return;
}



fn pass1_1030_1d7c(param_1: u16,param_2: u16,param_3: u32) -> u32

{
  let uVar1: u32;
  
  pass1_1030_1d58(param_3);
  if ((param_2 | param_1) != 0x0) {
    uVar1 = struct_op_1030_73a8(CONCAT22(param_2,param_1));
    return uVar1;
  }
  return 0x0;
}



fn pass1_1030_1daa(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0xa),(param_1 + 0x8))
  ;
}



fn pass1_1030_1dbc(void)
{
  return;
}



fn pass1_1030_1dfc(param_1: *mut u32,param_2: u16,param_3: u16,param_4: u32)
{
  let puVar1: u32;
  let puVar2: *mut u16;
  code **ppcVar3;
  let uVar4: u32;
  let iVar5: i16;
  let uVar6: u16;
  let bVar7: bool;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  puVar1 = (iVar5 + 0x8);
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (*(long *)(iVar5 + 0x4) == 0x0)) {
    puVar2 = (u16 *)(iVar5 + 0x12);
    bVar7 = *puVar2 < param_4._2_2_;
    if ((bVar7 || *puVar2 == param_4._2_2_) &&
       ((bVar7 || (puVar2 = (u16 *)(iVar5 + 0x10),
                  *puVar2 < param_4 || *puVar2 == param_4)))) {
      ppcVar3 = (code **)(*param_1 + 0x20);
      (**ppcVar3)();
    }
    puVar1 = (iVar5 + 0x10);
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (*(long *)(iVar5 + 0x4) == 0x0)) {
      return;
    }
    puVar2 = (u16 *)(iVar5 + 0xa);
    bVar7 = *puVar2 < param_4._2_2_;
    if ((bVar7 || *puVar2 == param_4._2_2_) &&
       ((bVar7 || (puVar2 = (u16 *)(iVar5 + 0x8),
                  *puVar2 < param_4 || *puVar2 == param_4)))) {
      (iVar5 + 0x8) = (param_4 + 0x1);
      (iVar5 + 0xa) = (param_4 + 0x1 >> 0x10);
    }
  }
  uVar4 = (iVar5 + 0x4);
  uVar6 = (uVar4 >> 0x10);
  iVar5 = uVar4;
  (iVar5 + param_4 * 0x4) = param_2;
  (iVar5 + param_4 * 0x4 + 0x2) = param_3;
  return;
}



fn pass1_1030_1e96(param_1: *mut u32)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let uVar4: u16;
  let uStack6: u32;
  
  uStack6 = 0x0;
  while( true ) {
    uVar4 = (param_1 >> 0x10);
    puVar1 = (param_1 + 0x8);
    if ((*puVar1 < uStack6 || *puVar1 == uStack6) ||
       (uVar3 = (param_1 + 0x4),
       *(long *)(uVar3 + uStack6 * 0x4) == 0x0)) break;
    uStack6 += 0x1;
  }
  ppcVar2 = (code **)(*param_1 + 0x8);
  (**ppcVar2)();
  return;
}



fn pass1_1030_1eee(param_1: u32,param_2: u32)
{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar1 = (iVar2 + 0xc);
  param_2._2_2_ = (iVar2 + 0xe);
  if (uVar1 < param_2) {
    uVar1 = param_2 & 0xffff;
  }
  (iVar2 + 0xc) = uVar1;
  (iVar2 + 0xe) = param_2._2_2_;
  return;
}



fn pass1_1030_1f16(param_1: *mut u32,param_2: u32)
{
  long *plVar1;
  code **ppcVar2;
  let uVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if ((*(long *)(iVar4 + 0x4) == 0x0) ||
     ((iVar4 + 0x10) <= (iVar4 + 0x8))) {
    ppcVar2 = (code **)(*param_1 + 0x20);
    (**ppcVar2)();
  }
  uVar3 = (iVar4 + 0x4);
  ((iVar4 + 0x8) * 0x4 + uVar3) = param_2;
  plVar1 = (long *)(iVar4 + 0x8);
  *plVar1 = *plVar1 + 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_1f77(param_1: i16,param_2: i16,param_3: u16,param_4: u16)
{
  let puVar1: *mut u16;
  let uVar2: u16;
  let uVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let lVar7: i32;
  
  if (*(long *)(param_1 + 0x10) == 0x0) {
    iVar4 = (param_1 + 0xc);
    PTR_LOOP_1050_5f2e = (param_1 + 0xe);
  }
  else {
    uVar2 = (param_1 + 0x10);
    puVar1 = (u16 *)(param_1 + 0x14);
    iVar4 = uVar2 + *puVar1;
    PTR_LOOP_1050_5f2e =
         
         ((param_1 + 0x12) + (param_1 + 0x16) +
         CARRY2(uVar2,*puVar1));
  }
  (param_2 + -0x4) = iVar4;
  (param_2 + -0x2) = PTR_LOOP_1050_5f2e;
  (param_2 + -0x8) = 0x0;
  if (*(long *)(param_1 + 0x4) == 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar5 = fn_ptr_op_1000_1708((param_2 + -0x4) << 0x2,0x0,0x1,
                                PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
    uVar3 = (param_1 + 0x4);
    uVar2 = (param_2 + -0x4);
    lVar7 = pass1_1000_0ed4(0x1000,param_4,0x1,uVar2 * 0x4,
                            (PTR_LOOP_1050_5f2e * 0x2 + CARRY2(uVar2,uVar2)) *
                            0x2 + CARRY2(uVar2 * 0x2,uVar2 * 0x2),(u16 *)uVar3,
                            (uVar3 >> 0x10));
    PTR_LOOP_1050_5f2e = (lVar7 >> 0x10);
    uVar5 = lVar7;
  }
  (param_2 + -0x8) = uVar5;
  (param_2 + -0x6) = PTR_LOOP_1050_5f2e;
  if ((PTR_LOOP_1050_5f2e | (param_2 + -0x8)) != 0x0) {
    uVar3 = (param_2 + 0x6);
    uVar6 = (uVar3 >> 0x10);
    iVar4 = uVar3;
    (iVar4 + 0x10) = (param_2 + -0x4);
    (iVar4 + 0x4) = (param_2 + -0x8);
  }
  return;
}



astruct_18 *  pass1_1030_201e(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_1d28((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1030_2068(param_1: *mut u16)
{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u16;
  let iStack4: i16;
  
  struct_1030_17ce(param_1,0x0,0x0);
  uVar3 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x293c;
  (iVar1 + 0x2) = 0x1030;
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (iVar1 + 0x10)),
                  (WNDCLASS16 *)0x0,0x106);
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (iVar1 + 0x116)),
                  (WNDCLASS16 *)0x0,0x86);
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (iVar1 + 0x19c)),
                  (WNDCLASS16 *)0x0,0xa);
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (iVar1 + 0x2ac)),
                  (WNDCLASS16 *)0x0,0x106);
  iStack4 = 0x0;
  do {
    iVar2 = iStack4 * 0x2 + iVar1;
    (iVar2 + 0x10) = 0xffff;
    (iVar2 + 0x1a6) = 0x19;
    iStack4 += 0x1;
  } while (iStack4 < 0x83);
  return;
}


i16  pass1_1030_2242(param_1: u32,param_2: i16)

{
  let iVar1: i16;
  astruct_168 *iVar2;
  astruct_168 *paVar2;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = (astruct_168 *)param_1;
  paVar2 = (astruct_168 *)&iVar2->field_0x10;
  if (-0x1 < (&paVar2->field_0x0 + param_2 * 0x2)) {
    iVar1 = (&iVar2->field_0x10 + param_2 * 0x2);
    paVar2 = iVar2 + 0x1;
    if ((&paVar2->field_0x0 + param_2 * 0x2) <= iVar1) {
      return iVar1;
    }
  }
  return (&paVar2->field_0x0 + param_2 * 0x2);
}



fn pass1_1030_227a(param_1: u32,param_2: u32,param_3: u16,param_4: u16)
{
  let uVar1: u16;
  let iVar2: i16;
  let BVar3: bool;
  let uVar4: u16;
  let uVar5: u16;
  
  uVar1 = pass1_1030_1978(param_1,param_2,param_3,param_4);
  if (uVar1 != 0x0) {
    iVar2 = param_1;
    uVar1 = (param_1 >> 0x10);
    uVar4 = param_2;
    uVar5 = (param_2 >> 0x10);
    BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x10,uVar1,0x106,0x1008);
    if (BVar3 != 0x0) {
      BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x116,uVar1,0x86,0x1008)
      ;
      if (BVar3 != 0x0) {
        BVar3 = write_to_file_1008_7e1c
                          (uVar4,uVar5,iVar2 + 0x19c,uVar1,0xa,0x1008);
        if (BVar3 != 0x0) {
          BVar3 = write_to_file_1008_7e1c
                            (uVar4,uVar5,iVar2 + 0x1a6,uVar1,0x106,0x1008);
          if (BVar3 != 0x0) {
            BVar3 = write_to_file_1008_7e1c
                              (uVar4,uVar5,iVar2 + 0x2ac,uVar1,0x106,0x1008);
            if (BVar3 != 0x0) {
              return;
            }
          }
        }
      }
    }
    PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}



void 
pass1_1030_232e(param_1: u32,param_2: u32,param_3: i16,param_4: u16,
               param_5: u16)

{
  let uVar1: u16;
  let iVar2: i16;
  let BVar3: bool;
  let uVar4: u16;
  let uVar5: u16;
  
  file_1030_19b4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 != 0x0) {
    iVar2 = param_1;
    uVar1 = (param_1 >> 0x10);
    uVar4 = param_2;
    uVar5 = (param_2 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x10,0x0,uVar1,0x106,0x1008);
    if (BVar3 != 0x0) {
      BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x116,0x0,uVar1,0x86,0x1008);
      if (BVar3 != 0x0) {
        BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x19c,0x0,uVar1,0xa,0x1008);
        if (BVar3 != 0x0) {
          BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x1a6,0x0,uVar1,0x106,0x1008);
          if (BVar3 != 0x0) {
            BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x2ac,0x0,uVar1,0x106,0x1008);
            if (BVar3 != 0x0) {
              return;
            }
          }
        }
      }
    }
    PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_23e2(param_1: u32,param_2: i16,param_3: u16,param_4: i16,uchar *param_5,
               param_6: u16,param_7: u16)

{
  code **ppcVar1;
  let uVar2: u32;
  let bVar3: bool;
  let bVar4: bool;
  undefined3 extraout_var;
  let extraout_DX: *mut u8
  let extraout_DX_00: *mut u8
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let iVar8: i16;
  let uVar9: u16;
  let puVar10: u32;
  let puVar11: *mut u16;
  let uVar12: u16;
  let iStack8: i16;
  
  uVar9 = (param_1 >> 0x10);
  uVar6 = param_1;
  if ((uVar6 + 0x10 + param_3 * 0x2) < 0x0) {
    uVar12 = param_3;
    if (param_2 == 0x0) {
      puVar10 = 
                mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x31,param_7,param_5,param_6);
      ppcVar1 = (code **)(*puVar10 + 0x14);
      (**ppcVar1)(0x1010,puVar10,(puVar10 >> 0x10),param_3,
                  param_3 >> 0xf);
      param_5 = extraout_DX_00;
    }
    else {
      puVar10 = 
                mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x41,param_7,param_5,param_6);
      ppcVar1 = (code **)(*puVar10 + 0x14);
      (**ppcVar1)(0x1010,puVar10,(puVar10 >> 0x10),param_3,
                  param_3 >> 0xf);
      param_5 = extraout_DX;
    }
    uVar2 = (uVar12 + 0x16);
    param_4 = (uVar2 + 0x4);
    (uVar6 + param_3 * 0x2 + 0x10) = param_4;
  }
  if ((uVar6 + 0x10 + param_3 * 0x2) != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
    pass1_1030_2852();
    bVar3 = false;
    iStack8 = param_4;
    if ((uVar6 + 0x152) != 0x0) {
      bVar4 = pass1_1030_266c(uVar6,CONCAT22(param_3,uVar9));
      if (CONCAT31(extraout_var,bVar4) != 0x0) {
        iStack8 = param_4 + 0x1;
        bVar3 = true;
      }
    }
    iVar8 = param_3 * 0x2;
    iStack8 = (uVar6 + iVar8 + 0x10) - iStack8;
    (uVar6 + iVar8 + 0x10) = iStack8;
    if (iStack8 < 0x0) {
      (uVar6 + iVar8 + 0x10) = 0x0;
    }
    uVar7 = param_3 * 0x2;
    if ((uVar6 + 0x2ac + uVar7) == 0x0) {
      iVar8 = uVar7 + uVar6;
      (iVar8 + 0x2ac) = 0x1;
      param_5 = (uchar *)((uVar6 + uVar7 + 0x1a6) + -0x1);
      *(uchar **)(iVar8 + 0x1a6) = param_5;
      param_6 = uVar7;
      if ((uVar6 + uVar7 + 0x1a6) < 0x0) {
        (iVar8 + 0x1a6) = 0x0;
      }
    }
    if (((uVar6 + 0x10 + param_3 * 0x2) != 0x0) ||
       (uVar7 = uVar6 + 0x1a6, (uVar7 + param_3 * 0x2) != 0x0)) {
      if ((uVar6 + 0x10 + param_3 * 0x2) == 0x0) {
        (uVar6 + param_3 * 0x2 + 0x10) = 0x1;
      }
      return;
    }
    uVar12 = param_3;
    puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_7,param_5,param_6);
    uVar5 = (puVar11 >> 0x10);
    pass1_1010_6cf8(0x1010,puVar11,uVar12,param_7,uVar5,uVar7,param_6);
    pass1_1030_26ac(param_1,param_3,uVar5,param_7);
    if (bVar3) {
      iVar8 = pass1_1030_28dc(param_1,param_3);
      (uVar6 + iVar8 * 0x2 + 0x19c) = 0x0;
    }
  }
  return;
}



fn pass1_1030_25b2(param_1: u32,param_2: i16) -> bool

{
  if ((param_1 + 0x10 + param_2 * 0x2) == 0x0) {
    return 0x1;
  }
  return 0x0;
}



fn pass1_1030_25d8(param_1: u32,param_2: u16,param_3: i16)
{
  (param_1 + param_3 * 0x2 + 0x10) = param_2;
  return;
}



fn pass1_1030_25f0(param_1: u32,param_2: i16,param_3: i16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  if (param_2 == 0x0) {
    param_2 = (param_1 + 0x116 + param_3 * 0x2) + 0x1;
  }
  (param_1 + param_3 * 0x2 + 0x116) = param_2;
  return;
}



fn pass1_1030_2622(param_1: u32,param_2: i16) -> bool

{
  let iVar1: i16;
  
  if ((param_2 != 0x70) && (param_2 != 0x1)) {
    iVar1 = pass1_1030_28dc(param_1,0x0);
    if (-0x1 < iVar1) {
      (param_1 + iVar1 * 0x2 + 0x19c) = param_2;
    }
    return -0x1 < iVar1;
  }
  return false;
}



fn pass1_1030_266c(param_1: u16,param_2: u32) -> bool

{
  let iVar1: i16;
  
  iVar1 = pass1_1030_28dc(CONCAT22(param_2,param_1),(param_2 >> 0x10));
  return iVar1 != -0x1;
}



fn pass1_1030_2690(param_1: u32)
{
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (param_1 + 0x2ac)),
                  (WNDCLASS16 *)0x0,0x106);
  return;
}



fn pass1_1030_26ac(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  let cVar5: u8;
  let puVar6: *mut u8;
  let uVar7: u16;
  let iVar8: i16;
  let iVar9: i16;
  let uVar10: u16;
  let iVar11: i16;
  let uVar12: u16;
  let iStack38: i16;
  let local_14: [u8;12];
  
  iVar11 = param_1;
  uVar12 = (param_1 >> 0x10);
  if (param_2 != 0x13) {
    if (0x13 < param_2) {
      if (param_2 != 0x5f) {
        if ((param_2 - 0x5f) < 0x6) {
          return;
        }
        if (param_2 != 0x66 && 0x0 < (param_2 - 0x65)) {
          if ((param_2 - 0x66) < 0x5) {
            return;
          }
          if (0x4 < (param_2 - 0x6b)) {
            return;
          }
        }
      }
      pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_14),0x1,0x0,0x400);
      while( true ) {
        uVar10 = param_3;
        puVar6 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4,puVar6));
        param_3 = uVar10 | puVar6;
        if (param_3 == 0x0) break;
        if (*(long *)(iVar11 + 0x4) == *(long *)(puVar6 + 0x200)) {
          uVar7 = (puVar6 + 0x18) + 0x19;
          if (0x3e8 < uVar7) {
            uVar7 = 0x3e8;
          }
          pass1_1038_4d0e(CONCAT22(uVar10,puVar6),uVar7);
        }
      }
      return;
    }
    if (param_2 == 0x12) {
      pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_14),0x1,0x0,0x400);
      while( true ) {
        uVar10 = param_3;
        puVar6 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4,puVar6));
        param_3 = uVar10 | puVar6;
        if (param_3 == 0x0) break;
        if (*(long *)(iVar11 + 0x4) == *(long *)(puVar6 + 0x200)) {
          uVar2 = (puVar6 + 0x1f6);
          iVar9 = uVar2;
          uVar4 = (uVar2 >> 0x10);
          piVar1 = (i16 *)(iVar9 + 0x182);
          *piVar1 = *piVar1 + -0x19;
          iVar8 = (iVar9 + 0x182);
          if (iVar8 < 0x1) {
            iVar8 = 0x1;
          }
          (iVar9 + 0x182) = iVar8;
        }
      }
      return;
    }
    if (0x12 < param_2) {
      return;
    }
    cVar5 = (char)param_2;
    if (cVar5 != '\n') {
      if ((char)(cVar5 + -0xa) < '\x06') {
        return;
      }
      if ('\x01' < (char)(cVar5 + -0x10)) {
        return;
      }
    }
  }
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_14),0x1,0x0,0x400);
  while( true ) {
    uVar10 = param_3;
    puVar6 = local_14;
    pass1_1028_e4ec(CONCAT22(param_4,puVar6));
    param_3 = uVar10 | puVar6;
    if (param_3 == 0x0) break;
    if (*(long *)(iVar11 + 0x4) == *(long *)(puVar6 + 0x200)) {
      uVar2 = (puVar6 + 0x1f6);
      iVar8 = uVar2 + 0x180;
      uVar4 = (uVar2 >> 0x10);
      iStack38 = 0x1;
      do {
        iVar3 = iStack38 * 0x2;
        piVar1 = (i16 *)(iVar3 + iVar8);
        *piVar1 = *piVar1 + -0x1;
        iVar9 = (iVar3 + iVar8);
        if (iVar9 < 0x1) {
          iVar9 = 0x1;
        }
        (iVar3 + iVar8) = iVar9;
        iStack38 += 0x1;
      } while (iStack38 < 0x6);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_2852(void)
{
  return;
}



i16  pass1_1030_28dc(param_1: u32,param_2: i16)

{
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x4 < iStack4) {
      return -0x1;
    }
    if ((param_1 + 0x19c + iStack4 * 0x2) == param_2) break;
    iStack4 += 0x1;
  }
  return iStack4;
}



astruct_18 *  pass1_1030_2916(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_18b2((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1030_2958(param_1: *mut u16)
{
  astruct_347 *iVar1;
  let uVar1: u16;
  
  struct_1030_17ce(param_1,0x5,0xf);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_347 *)param_1;
  iVar1->field_0x10 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  iVar1->field_0x18 = 0x2710;
  iVar1->field_0x1a = 0x0;
  *param_1 = 0x3130;
  iVar1->field_0x2 = 0x1030;
  return;
}


fn pass1_1030_29e6(param_1: *mut u16)
{
  let uVar1: u16;
  astruct_18 *paVar2;
  astruct_607 *iVar4;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar4 = (astruct_607 *)param_1;
  *param_1 = 0x3130;
  iVar4->field_0x2 = 0x1030;
  paVar2 = *(astruct_18 **)&iVar4->field_0x10;
  uVar1 = iVar4->field_0x12;
  if ((uVar1 | paVar2) != 0x0) {
    pass1_1030_8496(paVar2 & 0xffff | uVar1 << 0x10);
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  pass1_1030_18b2(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_2a2c(param_1: u32,uchar *param_2,param_3: i16,param_4: u16)
{
  let piVar1: *mut i16;
  astruct_678 *iVar2;
  let uVar2: u16;
  astruct_67 *paVar3;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let iVar9: i16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_678 *)param_1;
  if (0x0 < iVar2->field_0x18) {
    piVar1 = &iVar2->field_0x18;
    *piVar1 = *piVar1 + -0x1;
  }
  if (iVar2->field_0x16 == 0x0) {
    iVar2->field_0x16 = 0x1;
  }
  if (iVar2->field_0x1a == 0x0) {
    iVar2->field_0x1a = 0x2;
  }
  if (iVar2->field_0x18 < 0x1) {
    iVar2->field_0x16 = 0x2;
    iVar2->field_0x1a = 0x1;
    uVar8 = 0x0;
    iVar9 = 0x21;
    uVar6 = 0x1;
    uVar7 = 0x0;
    uVar4 = 0x0;
    iVar5 = 0x0;
    uVar2 = 0x0;
    paVar3 = (astruct_67 *)
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,param_2,param_3);
    post_win_msg_1008_a0e4
              (paVar3,CONCAT22(uVar4,uVar2),iVar5,uVar6,CONCAT22(uVar8,uVar7),iVar9,0x1008
               ,param_4);
  }
  return;
}



fn pass1_1030_2a98(param_1: u32) -> u16

{
  let piVar1: *mut i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  piVar1 = (i16 *)(param_1 + 0x14);
  *piVar1 = *piVar1 + 0x1;
  return (param_1 + 0x14);
}



fn pass1_1030_2aaa(param_1: u32) -> u16

{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x10) == 0x0) {
    return 0x0;
  }
  uVar1 = (param_1 + 0x10);
  return (uVar1 + 0xc);
}



fn pass1_1030_2aca(param_1: u32,param_2: u32,param_3: u16,param_4: u16)
{
  let uVar1: u32;
  let puVar2: *mut u16;
  let uVar3: u16;
  let Bvar4: bool;
  let iVar5: i16;
  astruct_730 *iVar6;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let local_18: [u32;0x3];
  let local_c: [u16;0x3];
  let local_6: [u16;0x2];
  
  uVar3 = pass1_1030_1978(param_1,param_2,param_3,param_4);
  if (uVar3 == 0x0) {
    return;
  }
  uVar6 = (param_1 >> 0x10);
  iVar6 = (astruct_730 *)param_1;
  local_c[0] = *iVar6->field_0x10;
  uVar3 = param_2;
  uVar8 = (param_2 >> 0x10);
  BVar4 = write_to_file_1008_7e1c(uVar3,uVar8,local_c,param_4,0x2,0x1008);
  if (((BVar4 != 0x0) &&
      (puVar2 = iVar6->field_0x10,
      BVar4 = pass1_1008_7c2a(param_2,*(char **)(puVar2 + 0x2),0x1008), BVar4 != 0x0)
      ) && (puVar2 = iVar6->field_0x10,
           iVar5 = write_to_file_1008_7b4c
                             (param_2,puVar2 & 0xffff0000 |
                                      (puVar2 + 0x6),0x1008,param_4),
           iVar5 != 0x0)) {
    puVar2 = iVar6->field_0x10;
    local_6[0] = (puVar2 + 0xc);
    BVar4 = write_to_file_1008_7e1c
                      (uVar3,uVar8,local_6,param_4,0x2,0x1008);
    if (BVar4 != 0x0) {
      puVar2 = iVar6->field_0x10;
      local_18[0] = (puVar2 + 0xe);
      BVar4 = write_to_file_1008_7e1c
                        (uVar3,uVar8,local_18,param_4,0x4,0x1008);
      if ((BVar4 != 0x0) &&
         (puVar2 = iVar6->field_0x10,
         BVar4 = write_to_file_1008_7e1c
                           (uVar3,uVar8,puVar2 + 0x12,(puVar2 >> 0x10)
                            ,0x10,0x1008), BVar4 != 0x0)) {
        puVar2 = iVar6->field_0x10;
        local_c[0] = (puVar2 + 0x22);
        BVar4 = write_to_file_1008_7e1c
                          (uVar3,uVar8,local_c,param_4,0x2,0x1008);
        if ((BVar4 != 0x0) &&
           ((puVar2 = iVar6->field_0x10, (puVar2 + 0x22) == 0x0 ||
            (puVar2 = iVar6->field_0x10, uVar7 = (puVar2 >> 0x10),
            iVar5 = puVar2, uVar1 = (iVar5 + 0x24),
            BVar4 = write_to_file_1008_7e1c
                              (uVar3,uVar8,uVar1,(uVar1 >> 0x10),
                               ((iVar5 + 0x22) * 0x2),0x1008)
            , BVar4 != 0x0)))) {
          local_c[0] = iVar6->field_0x14;
          BVar4 = write_to_file_1008_7e1c
                            (uVar3,uVar8,local_c,param_4,0x2,0x1008);
          if (BVar4 != 0x0) {
            local_c[0] = iVar6->field_0x16;
            BVar4 = write_to_file_1008_7e1c
                              (uVar3,uVar8,local_c,param_4,0x2,0x1008);
            if (BVar4 != 0x0) {
              local_c[0] = iVar6->field_0x18;
              BVar4 = write_to_file_1008_7e1c
                                (uVar3,uVar8,local_c,param_4,0x2,0x1008);
              if (BVar4 != 0x0) {
                local_c[0] = iVar6->field_0x1a;
                BVar4 = write_to_file_1008_7e1c
                                  (uVar3,uVar8,local_c,param_4,0x2,0x1008)
                ;
                if (BVar4 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
  }
  PTR_LOOP_1050_0310 = 0x6d0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_2c8a(param_1: u32,param_2: u32,param_3: i16,uchar *param_4,param_5: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let BVar3: bool;
  let puVar4: *mut u8;
  let uVar5: u16;
  let puVar6: *mut u8
  astruct_374 *iVar7;
  astruct_371 *iVar8;
  astruct_372 *iVar9;
  let unaff_DI: i16;
  let uVar7: u16;
  let puVar8: *mut u16;
  let uVar9: u16;
  let uVar10: u16;
  let puStack1038: *mut u16;
  let local_406: u16;
  let local_404: u16;
  let local_402: [u8;400];
  astruct_373 *iVar14;
  
  iVar14 = (astruct_373 *)param_1;
  uVar10 = (param_1 >> 0x10);
  file_1030_19b4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 == 0x0) {
    return;
  }
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4,0x1000);
    PTR_LOOP_1050_5f2e = param_4;
  }
  else {
  }
  uVar2 = fn_ptr_op_1000_1708(0x28,0x0,0x1,PTR_LOOP_1050_5f2c,
                              PTR_LOOP_1050_5f2e,0x1000);
  puStack1038 = (u16 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar2);
  puVar6 = (uchar *)(PTR_LOOP_1050_5f2e | uVar2);
  if (puVar6 == (uchar *)0x0) {
    iVar14->field_0x10 = (u16 *)0x0;
  }
  else {
    puVar8 = pass1_1008_3e38((u16 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar2 + 0x6));
    puVar6 = (uchar *)(puVar8 >> 0x10);
    iVar14->field_0x10 = puStack1038;
  }
  puVar8 = iVar14->field_0x10;
  uVar2 = param_2;
  uVar9 = (param_2 >> 0x10);
  BVar3 = read_file_1008_7dee(uVar2,uVar9,puVar8,0x0,
                              (puVar8 >> 0x10),0x2,0x1008);
  if (BVar3 != 0x0) {
    puVar4 = local_402;
    read_file_1008_7c6e(uVar2,uVar9,CONCAT22(param_5,puVar4),0x1008);
    if (puVar4 != 0x0) {
      uVar5 = str_op_1008_60e8(CONCAT22(param_5,local_402),puVar6);
      puVar8 = iVar14->field_0x10;
      uVar7 = (puVar8 >> 0x10);
      iVar7 = (astruct_374 *)puVar8;
      iVar7->field_0x2 = uVar5;
      iVar7->field_0x4 = puVar6;
      puVar8 = iVar14->field_0x10;
      BVar3 = read_file_1008_7bc8(param_2,(u16 *)
                                          (puVar8 & 0xffff0000 |
                                          (puVar8 + 0x6)),0x1008,param_5);
      if ((((BVar3 != 0x0) &&
           (puVar8 = iVar14->field_0x10,
           BVar3 = read_file_1008_7dee(uVar2,uVar9,puVar8 + 0xc,0x0,
                                       (puVar8 >> 0x10),0x2,0x1008),
           BVar3 != 0x0)) &&
          (puVar8 = iVar14->field_0x10,
          BVar3 = read_file_1008_7dee(uVar2,uVar9,puVar8 + 0xe,0x0,
                                      (puVar8 >> 0x10),0x4,0x1008),
          BVar3 != 0x0)) &&
         ((puVar8 = iVar14->field_0x10,
          BVar3 = read_file_1008_7dee(uVar2,uVar9,puVar8 + 0x12,0x0,
                                      (puVar8 >> 0x10),0x10,0x1008),
          BVar3 != 0x0 &&
          (puVar8 = iVar14->field_0x10,
          BVar3 = read_file_1008_7dee(uVar2,uVar9,puVar8 + 0x22,0x0,
                                      (puVar8 >> 0x10),0x2,0x1008),
          BVar3 != 0x0)))) {
        puVar8 = iVar14->field_0x10;
        if ((puVar8 + 0x22) != 0x0) {
          puVar8 = iVar14->field_0x10;
          unaff_DI = (puVar8 >> 0x10);
          iVar8 = (astruct_371 *)puVar8;
          uVar5 = iVar8->field_0x22 * 0x2;
          mem_op_1000_179c(uVar5,puVar6,0x1000);
          iVar8->field_0x24 = uVar5;
          iVar8->field_0x26 = puVar6;
          puVar8 = iVar14->field_0x10;
          uVar7 = (puVar8 >> 0x10);
          iVar9 = (astruct_372 *)puVar8;
          uVar1 = iVar9->field_0x24;
          BVar3 = read_file_1008_7dee(uVar2,uVar9,uVar1,0x0,
                                      (uVar1 >> 0x10),
                                      iVar9->field_0x22 * 0x2,0x1008);
          if (BVar3 == 0x0) {
            PTR_LOOP_1050_0310 = 0x6d2;
            return;
          }
        }
        BVar3 = read_file_1008_7dee(uVar2,uVar9,&iVar14->field_0x14,0x0,uVar10,0x2,0x1008)
        ;
        if (((BVar3 != 0x0) &&
            (BVar3 = read_file_1008_7dee(uVar2,uVar9,&local_404,0x0,param_5,0x2,
                                         0x1008), BVar3 != 0x0)) &&
           ((BVar3 = read_file_1008_7dee(uVar2,uVar9,&iVar14->field_0x18,0x0,uVar10,0x2,
                                         0x1008), BVar3 != 0x0 &&
            (BVar3 = read_file_1008_7dee(uVar2,uVar9,&local_406,0x0,param_5,0x2,
                                         0x1008), BVar3 != 0x0)))) {
          iVar14->field_0x16 = local_404;
          iVar14->field_0x1a = local_406;
          puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_5,puVar6,unaff_DI);
          pass1_1018_04a4(puVar8,iVar14->field_0x4);
          pass1_1010_82f8(_PTR_LOOP_1050_14cc,*iVar14->field_0x10);
          return;
        }
      }
    }
  }
  PTR_LOOP_1050_0310 = 0x6d2;
  return;
}



i16  pass1_1030_2f1a(param_1: u32,param_2: *mut u16,param_3: *mut u16)

{
  let iVar1: i16;
  let uVar2: u32;
  let iVar3: i16;
  
  uVar2 = (param_1 + 0x10);
  iVar3 = uVar2;
  iVar1 = (iVar3 + 0xc);
  if (iVar1 - 0x1U < 0x9) {
    switch(iVar1) {
    default:
      *param_3 = 0x19;
      *param_2 = 0x2d;
      return iVar3;
    case 0x3:
    case 0x4:
    case 0x5:
      *param_3 = 0xa;
      *param_2 = 0xf;
      return iVar3;
    case 0x6:
      *param_3 = 0xa;
      *param_2 = 0x19;
      return iVar3;
    case 0x7:
      *param_3 = 0x19;
      *param_2 = 0x37;
      return iVar3;
    }
  }
  *param_3 = 0x0;
  *param_2 = 0x0;
  return 0x0;
}



fn pass1_1030_2fac(param_1: u32) -> u16

{
  let lVar1: i32;
  astruct_598 *iVar2;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_598 *)param_1;
  if (iVar2->field_0x10 == 0x0) {
    return 0x0;
  }
  lVar1 = iVar2->field_0x10;
  if ((lVar1 + 0xc) < 0x2) {
    return 0x4;
  }
  lVar1 = iVar2->field_0x10;
  if ((lVar1 + 0xc) < 0x5) {
    return 0x3;
  }
  lVar1 = iVar2->field_0x10;
  if ((lVar1 + 0xc) < 0x8) {
    return 0x2;
  }
  return 0x1;
}



fn pass1_1030_3006(param_1: u32,param_2: u32)
{
  (param_1 + 0x10) = param_2;
  return;
}



fn pass1_1030_301a(param_1: u32,char *param_2,param_3: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let iVar4: i16;
  astruct_608 *iVar3;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (*(long *)(iVar4 + 0x10) != 0x0) {
    uVar1 = (iVar4 + 0x10);
    fn_ptr_1000_17ce(*(astruct_18 **)(uVar1 + 0x2),0x1000);
    uVar2 = str_op_1008_60e8(param_2,param_3);
    uVar1 = (iVar4 + 0x10);
    uVar5 = (uVar1 >> 0x10);
    iVar3 = (astruct_608 *)uVar1;
    iVar3->field_0x2 = uVar2;
    iVar3->field_0x4 = param_3;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_3058(param_1: u32,param_2: u16,uchar *param_3) -> u16

{
  let puVar1: *mut u16;
  code **ppcVar2;
  let puVar3: *mut u8
  let extraout_DX: *mut u8
  astruct_375 *iVar4;
  let uVar4: u16;
  let uVar5: u32;
  let uVar6: u32;
  let uStack4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_375 *)param_1;
  puVar3 = param_3;
  if (iVar4->field_0xc == 0x0) {
    mem_op_1000_179c(0x18,param_3,0x1000);
    puVar3 = (uchar *)(param_3 | param_2);
    if (puVar3 == (uchar *)0x0) {
      iVar4->field_0xc = 0x0;
    }
    else {
      uVar5 = struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_3,param_2),0x5,0x5);
      puVar3 = (uchar *)(uVar5 >> 0x10);
      &iVar4->field_0xc = uVar5;
      *(uchar **)(&iVar4->field_0xc + 0x2) = puVar3;
    }
  }
  for (uStack4 = 0x0; uVar5 = iVar4->field_0x10, puVar1 = (u16 *)(uVar5 + 0x22),
      uStack4 <= *puVar1 && *puVar1 != uStack4; uStack4 += 0x1) {
    uVar6 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,puVar3,0x3);
    ppcVar2 = (code **)(*iVar4->field_0xc + 0x8);
    (**ppcVar2)(&USHORT_1050_1028,iVar4->field_0xc,uVar6,(uVar6 >> 0x10),
                uStack4,0x0);
    puVar3 = extraout_DX;
  }
  return 0x1;
}



astruct_18 *  pass1_1030_310a(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_29e6((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_314c(param_1: *mut u16,param_2: u32,uchar *param_3,param_4: u16)
{
  astruct_364 *iVar1;
  let unaff_DI: i16;
  let uVar1: u16;
  let iStack12: i16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_364 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x170 = 0x0;
  iVar1->field_0x1a4 = param_2;
  iVar1->field_0x1a8 = 0x5;
  iVar1->field_0x1aa = 0x0;
  iVar1->field_0x1ae = 0x10;
  *param_1 = 0x3af2;
  iVar1->field_0x2 = 0x1030;
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar1->field_0x4),
                  (WNDCLASS16 *)0x0,0x16c);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar1->field_0x18c),
                  (WNDCLASS16 *)0x0,0x18);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar1->field_0x174),
                  (WNDCLASS16 *)0x0,0xc);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar1->field_0x180),
                  (WNDCLASS16 *)0x0,0xc);
  mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_3,unaff_DI);
  if (PTR_LOOP_1050_13ae < 0x2) {
    pass1_1030_34da(param_1);
  }
  else {
    iVar1->field_0x176 = 0x1;
    iVar1->field_0x178 = 0x2;
    iVar1->field_0x17a = 0x2;
    iVar1->field_0x17c = 0x60001;
    for (iStack12 = 0x1; iStack12 < 0x6; iStack12 += 0x1) {
      (&iVar1->field_0x180 + iStack12 * 0x2) = 0x64;
    }
  }
  return;
}



fn pass1_1030_3258(param_1: u32,param_2: u16)
{
  (param_1 + 0x1ae) = param_2;
  return;
}



fn pass1_1030_326a(param_1: u32,param_2: u32,param_3: u16,param_4: u16)
{
  let uVar1: u16;
  let uVar2: u32;
  let uVar3: u16;
  astruct_692 *iVar4;
  let uVar4: u16;
  let lStack6: i32;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_692 *)param_1;
  if (iVar4->field_0x1aa == 0x0) {
    iVar4->field_0x1aa = 0x1;
  }
  else {
    param_2 = iVar4->field_0x1aa * 0x2;
    iVar4->field_0x1aa = param_2;
  }
  uVar1 = param_2;
  pass1_1030_38b8();
  lStack6 = CONCAT22(param_3,uVar1);
  uVar2 = iVar4->field_0x1aa;
  uVar3 = (&iVar4->field_0x1aa + 0x2);
  if (lStack6 < (long)uVar2) {
    uVar2 = uVar1;
    uVar3 = param_3;
  }
  &iVar4->field_0x1aa = uVar2;
  (&iVar4->field_0x1aa + 0x2) = uVar3;
  pass1_1030_375a(param_1 & 0xffff | uVar4 << 0x10,0x0,
                  uVar2 & 0xffff | uVar3 << 0x10,param_4);
  return;
}


fn pass1_1030_34da(param_1: u32)
{
  astruct_682 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_682 *)param_1;
  iVar1->field_0x176 = 0x1;
  iVar1->field_0x178 = 0x1;
  iVar1->field_0x17a = 0x1;
  iVar1->field_0x17c = 0x1;
  iVar1->field_0x17e = 0x4;
  iVar1->field_0x182 = 0x32;
  iVar1->field_0x184 = 0xa;
  iVar1->field_0x186 = 0xa;
  iVar1->field_0x188 = 0xa;
  iVar1->field_0x18a = 0x4b;
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | ZEXT24(iVar1 + 0x1)),
                  (WNDCLASS16 *)0x0,0x18);
  return;
}



fn pass1_1030_3534(param_1: u32,param_2: u32)
{
  (param_1 + 0x4) = param_2;
  return;
}



fn pass1_1030_3548(param_1: u32,param_2: i32)
{
  long *plVar1;
  
  plVar1 = (long *)(param_1 + 0x4);
  *plVar1 = *plVar1 + param_2;
  return;
}



fn pass1_1030_355c(param_1: u32,param_2: u32)
{
  let iVar1: i16;
  let uVar2: u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  do {
    iVar1 = iStack4 * 0x4;
    uVar2 = (param_1 >> 0x10);
    *(long *)(param_1 + iVar1 + 0x4) =
         *(long *)(iVar1 + param_2) + *(long *)(param_1 + 0x4 + iVar1);
    iStack4 += 0x1;
  } while (iStack4 < 0x5b);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_35a4(param_1: u32,param_2: i32,uchar *param_3,param_4: u16,param_5: u16)
{
  let puVar1: *mut u16;
  uchar **ppuVar2;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u32;
  let puVar7: *mut u8
  let uVar8: u16;
  let uVar9: u8;
  let uVar10: u8;
  let puStack24: *mut u8
  let puStack22: *mut u8
  let local_c: [u8;2];
  let local_a: u32;
  let uStack6: u32;
  
  vsprintf_op_1030_840a
            (s_Pop_Leaving__ld_1050_516a,param_4,param_5,param_3);
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_3,0x1000);
    PTR_LOOP_1050_5f2e = param_3;
  }
  else {
  }
  puStack24 = PTR_LOOP_1050_5f2c;
  puStack22 = PTR_LOOP_1050_5f2e;
  uVar4 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,PTR_LOOP_1050_5f2c,
                              PTR_LOOP_1050_5f2e,0x1000);
  uStack6 = CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
  uVar9 = (u8)param_5;
  uVar10 = (u8)(param_5 >> 0x8);
  pass1_1030_3948(param_1,(u16 *)CONCAT22(param_5,local_c),
                  (i16 *)CONCAT13(uVar10,CONCAT12(uVar9,&local_a)),0x3);
  uVar6 = (&local_a + 0x2U);
  pass1_1030_3948(param_1,(u16 *)CONCAT22(param_5,&local_a + 0x2U),
                  (i16 *)CONCAT13(uVar10,CONCAT12(uVar9,local_c)),0x4);
  do {
    puVar7 = (uchar *)uVar6;
    if (param_2 < 0x1) break;
    pass1_1008_612e(local_a,(local_a >> 0x10),puVar7);
    uVar6 = ZEXT24(&param_2);
    puStack24 = puVar7;
    pass1_1030_3a3a(param_1,(long *)CONCAT13(uVar10,CONCAT12(uVar9,&param_2)),puVar7)
    ;
    uVar8 = (uStack6 >> 0x10);
    puVar1 = (u16 *)(puStack24 * 0x4 + uStack6);
    uVar5 = *puVar1;
    *puVar1 = *puVar1 + uVar6;
    ppuVar2 = (uchar **)(puStack24 * 0x4 + uStack6 + 0x2);
    *ppuVar2 = PTR_LOOP_1050_5f2e + (*ppuVar2 + CARRY2(uVar5,uVar6));
    pass1_1030_38f2(param_1,0x3,param_5);
    uVar5 = uVar6;
    puVar7 = PTR_LOOP_1050_5f2e;
    pass1_1030_38f2(param_1,0x4,param_5);
    iVar3 = PTR_LOOP_1050_5f2e + puVar7;
    PTR_LOOP_1050_5f2e = puVar7;
  } while ((iVar3 + CARRY2(uVar5,uVar6) | uVar5 + uVar6) != 0x0);
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (param_1 + 0x18c)),
                  (WNDCLASS16 *)0x0,0x18);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_3694(param_1: u32,param_2: i16,param_3: i32,uchar *param_4,param_5: u16,
               param_6: u16)

{
  let puVar1: *mut u16;
  uchar **ppuVar2;
  let uVar3: u16;
  let puVar4: *mut u8
  let uVar5: u32;
  let puStack18: *mut u8
  let uStack6: u16;
  let puStack4: *mut u8
  
  vsprintf_op_1030_840a
            (s_Pop_Leaving__ld_1050_517a,param_5,param_6,param_4);
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4,0x1000);
    PTR_LOOP_1050_5f2e = param_4;
  }
  else {
  }
  puStack18 = PTR_LOOP_1050_5f2c;
  uStack6 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,PTR_LOOP_1050_5f2c,
                                PTR_LOOP_1050_5f2e,0x1000);
  uVar5 = (param_2 - 0x1U);
  puStack4 = PTR_LOOP_1050_5f2e;
  if (((param_2 < 0x1) || (SBORROW2(param_2,0x1))) ||
     (uVar5 = (param_2 - 0x5U),
     param_2 - 0x5U != 0x0 && 0x3 < (param_2 - 0x1U))) {
    while (puVar4 = (uchar *)uVar5, 0x0 < param_3) {
      pass1_1008_612e(0x0,0x5a,puVar4);
      uVar5 = ZEXT24(&param_3);
      puStack18 = puVar4;
      pass1_1030_3a3a(param_1,(long *)CONCAT13((char)(param_6 >> 0x8),
                                               CONCAT12((char)param_6,&param_3)),
                      puVar4);
      puVar1 = (u16 *)(puStack18 * 0x4 + uStack6);
      uVar3 = *puVar1;
      *puVar1 = *puVar1 + uVar5;
      ppuVar2 = (uchar **)(puStack18 * 0x4 + uStack6 + 0x2);
      *ppuVar2 = PTR_LOOP_1050_5f2e + (*ppuVar2 + CARRY2(uVar3,uVar5));
    }
  }
  else {
    pass1_1030_39dc(param_1,(long *)CONCAT22(param_6,&param_3),
                    CONCAT13((char)(PTR_LOOP_1050_5f2e >> 0x8),
                             CONCAT12((char)PTR_LOOP_1050_5f2e,uStack6)),param_2);
  }
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (param_1 + 0x18c)),
                  (WNDCLASS16 *)0x0,0x18);
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1030_375a(param_1: u32,param_2: i16,param_3: i32,param_4: u16)
{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u16;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  let iStack20: i16;
  let uStack18: u32;
  let lStack14: i32;
  let uStack10: i16;
  let iStack8: i16;
  let local_6: i16;
  let local_4: i16;
  
  iVar4 = param_1;
  if (param_2 == 0x0) {
    local_4 = 0x5a;
    while ((-0x1 < local_4 &&
           (pass1_1030_3a3a(param_1,(long *)CONCAT22(param_4,&param_3),local_4),
           param_3 != 0x0))) {
      local_4 += -0x1;
    }
  }
  else {
    pass1_1030_3948(param_1,(u16 *)CONCAT22(param_4,&local_4),
                    (i16 *)CONCAT22(param_4,&local_6),param_2);
    iStack10 = (local_4 - local_6) + 0x1;
    iStack8 = iStack10 >> 0xf;
    lStack14 = param_3 / (long)iStack10;
    uVar3 = (lStack14 * iStack10);
    uStack18 = CONCAT22(((param_3 >> 0x10) -
                        ((lStack14 * iStack10) >> 0x10)) -
                        (param_3 < uVar3),param_3 - uVar3);
    for (iStack20 = local_6; iStack20 <= local_4; iStack20 += 0x1) {
      iVar5 = iStack20 * 0x4;
      uVar6 = (param_1 >> 0x10);
      *(long *)(iVar4 + iVar5 + 0x4) = *(long *)(iVar4 + iVar5 + 0x4) - lStack14;
      iVar1 = (iVar4 + iVar5 + 0x6);
      if ((uStack18._2_2_ | uStack18) != 0x0) {
        iVar2 = (iVar4 + iVar5 + 0x4);
        (iVar4 + iVar5 + 0x4) = iVar2 + -0x1;
        (iVar4 + iVar5 + 0x6) = iVar1 - (iVar2 == 0x0);
        uStack18 += -0x1;
      }
      if ((iVar4 + iStack20 * 0x4 + 0x6) < 0x0) {
        (iVar4 + iStack20 * 0x4 + 0x4) = 0x0;
      }
    }
  }
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (iVar4 + 0x18c)),
                  (WNDCLASS16 *)0x0,0x18);
  return;
}



fn pass1_1030_387c(param_1: u32)
{
  let iStack4: i16;
  
  iStack4 = 0x5a;
  do {
    (iStack4 * 0x4 + param_1 + 0x4) =
         (iStack4 * 0x4 + param_1);
    iStack4 += -0x1;
  } while (0x0 < iStack4);
  (param_1 + 0x4) = 0x0;
  return;
}



fn pass1_1030_38b8(void)
{
  let iStack8: i16;
  
  iStack8 = 0x0;
  do {
    iStack8 += 0x1;
  } while (iStack8 < 0x5b);
  return;
}



fn pass1_1030_38f2(param_1: u32,param_2: i16,param_3: u16)
{
  let iStack12: i16;
  let local_a: i16;
  let local_8: i16;
  let uStack6: u32;
  
  uStack6 = 0x0;
  pass1_1030_3948(param_1,(u16 *)CONCAT22(param_3,&local_a),
                  (i16 *)CONCAT22(param_3,&local_8),param_2);
  for (iStack12 = local_8; iStack12 <= local_a; iStack12 += 0x1) {
  }
  return;
}



fn pass1_1030_3948(param_1: u32,param_2: *mut u16,i16 *param_3,param_4: i16)
{
  let uVar1: u16;
  
  if (param_4 == 0x1) {
    *param_3 = 0x0;
    *param_2 = 0x3;
    return;
  }
  uVar1 = (param_1 >> 0x10);
  if (param_4 == 0x2) {
    *param_3 = 0x4;
    *param_2 = (param_1 + 0x1ae);
    return;
  }
  if (param_4 == 0x3) {
    *param_3 = (param_1 + 0x1ae) + 0x1;
    *param_2 = 0x27;
    return;
  }
  if (param_4 != 0x4) {
    if (param_4 == 0x5) {
      *param_3 = 0x4c;
    }
    else {
      *param_3 = 0x0;
    }
    *param_2 = 0x5a;
    return;
  }
  *param_3 = 0x28;
  *param_2 = 0x4b;
  return;
}



fn pass1_1030_39dc(param_1: u32,long *param_2,param_3: u32,param_4: i16)
{
  let iVar1: i16;
  let in_DX: u16;
  let uVar2: u16;
  let unaff_SS: u16;
  let iStack8: i16;
  let local_6: i16;
  let local_4: i16;
  
  pass1_1030_3948(param_1,(u16 *)CONCAT22(unaff_SS,&local_6),
                  (i16 *)CONCAT22(unaff_SS,&local_4),param_4);
  iStack8 = local_6;
  while( true ) {
    if (iStack8 < local_4) {
      return;
    }
    iVar1 = local_4;
    pass1_1030_3a3a(param_1,param_2,iStack8);
    uVar2 = (param_3 >> 0x10);
    (iStack8 * 0x4 + param_3) = iVar1;
    (iStack8 * 0x4 + param_3 + 0x2) = in_DX;
    if (*param_2 == 0x0) break;
    iStack8 += -0x1;
  }
  return;
}



fn pass1_1030_3a3a(param_1: u32,long *param_2,param_3: i16)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let iVar6: i16;
  let iVar7: i16;
  let iVar8: i16;
  let uVar9: u16;
  
  iVar2 = (param_2 + 0x2);
  uVar9 = (param_1 >> 0x10);
  iVar6 = param_1;
  iVar7 = iVar6 + 0x4;
  iVar8 = param_3 * 0x4;
  piVar1 = (i16 *)(iVar7 + iVar8 + 0x2);
  iVar3 = *piVar1;
  if ((iVar3 < iVar2) ||
     ((uVar5 = *param_2, *piVar1 == iVar2 || iVar3 < iVar2 &&
      ((iVar7 + iVar8) < uVar5)))) {
    *param_2 = *param_2 - *(long *)(iVar6 + 0x4 + param_3 * 0x4);
    (iVar6 + param_3 * 0x4 + 0x4) = 0x0;
  }
  else {
    uVar4 = (iVar7 + iVar8);
    iVar3 = (iVar7 + iVar8 + 0x2);
    (iVar6 + iVar8 + 0x4) = uVar4 - uVar5;
    (iVar6 + iVar8 + 0x6) = (iVar3 - iVar2) - (uVar4 < uVar5);
    *param_2 = 0x0;
  }
  return;
}



astruct_18 *  pass1_1030_3ac6(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



u32 * 
pass1_1030_3af6(param_1: *mut u32,param_2: u16,param_3: u16,param_4: *mut u32,param_5: u16
               )

{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = *param_4;
  (iVar1 + 0x4) = (param_4 + 0x1);
  (iVar1 + 0x6) = param_3;
  (iVar1 + 0x8) = param_2;
  return param_1;
}



fn pass1_1030_3b28(param_1: u16) -> u16

{
  let puVar1: *mut u16;
  let puVar2: u32;
  let local_8: [u8;6];
  
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffc4,0x0);
  pass1_1030_3af6(&USHORT_1050_65e6,0x115,0x15b,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0x19);
  pass1_1030_3af6(&USHORT_1050_65f0,0x116,0x15c,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffdd,0x32);
  pass1_1030_3af6(&USHORT_1050_65fa,0x117,0x15d,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0x4b);
  pass1_1030_3af6(&USHORT_1050_6604,0x118,0x15e,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xf,0x64);
  pass1_1030_3af6(&USHORT_1050_660e,0x119,0x15f,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x28,0x7d);
  pass1_1030_3af6(&USHORT_1050_6618,0x11a,0x160,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffec,0x96);
  pass1_1030_3af6(&USHORT_1050_6622,0x11b,0x161,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x14,0xaf);
  pass1_1030_3af6(&USHORT_1050_662c,0x11c,0x162,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x1e,0xc8);
  pass1_1030_3af6(&USHORT_1050_6636,0x11d,0x163,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xfffb,0xe1);
  pass1_1030_3af6(&USHORT_1050_6640,0x11e,0x164,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x32,0xfa);
  pass1_1030_3af6(&USHORT_1050_664a,0x11f,0x165,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x1e,0xe1);
  pass1_1030_3af6(&USHORT_1050_6654,0x120,0x166,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffe7,0xfa);
  pass1_1030_3af6(&USHORT_1050_665e,0x121,0x167,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0x113);
  pass1_1030_3af6(&USHORT_1050_6668,0x122,0x168,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x28,0x12c);
  pass1_1030_3af6(&USHORT_1050_6672,0x123,0x169,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xf,0x145);
  pass1_1030_3af6(&USHORT_1050_667c,0x124,0x16a,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffec,0x15e);
  pass1_1030_3af6(&USHORT_1050_6686,0x125,0x16b,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0x0);
  pass1_1030_3af6(&USHORT_1050_6690,0x126,0x16c,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x2d,0x19);
  pass1_1030_3af6(&USHORT_1050_669a,0x127,0x16d,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xa,0x32);
  pass1_1030_3af6(&USHORT_1050_66a4,0x128,0x16e,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffe2,0x4b);
  pass1_1030_3af6(&USHORT_1050_66ae,0x129,0x16f,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x5,0x64);
  pass1_1030_3af6(&USHORT_1050_66b8,0x12a,0x170,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x32,0x7d);
  pass1_1030_3af6(&USHORT_1050_66c2,0x12b,0x171,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffc9,0x96);
  pass1_1030_3af6(&USHORT_1050_66cc,0x12c,0x172,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xfffb,0xaf);
  pass1_1030_3af6(&USHORT_1050_66d6,0x12d,0x173,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffe7,0xc8);
  pass1_1030_3af6(&USHORT_1050_66e0,0x12e,0x174,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x32,0x32);
  pass1_1030_3af6(&USHORT_1050_66ea,0x12f,0x175,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x3c,0x64);
  pass1_1030_3af6(&USHORT_1050_66f4,0x130,0x176,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffc4,0xe1);
  pass1_1030_3af6(&USHORT_1050_66fe,0x131,0x177,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0x19);
  pass1_1030_3af6(&USHORT_1050_6708,0x132,0x178,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x5,0xaf);
  pass1_1030_3af6(&USHORT_1050_6712,0x133,0x179,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0x19);
  pass1_1030_3af6(&USHORT_1050_671c,0x134,0x17a,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x23,0x19);
  pass1_1030_3af6(&USHORT_1050_6726,0x135,0x17b,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xfffb,0x32);
  pass1_1030_3af6(&USHORT_1050_6730,0x136,0x17c,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xf,0x32);
  pass1_1030_3af6(&USHORT_1050_673a,0x137,0x17d,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x2d,0x4b);
  pass1_1030_3af6(&USHORT_1050_6744,0x138,0x17e,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x1e,0x4b);
  pass1_1030_3af6(&USHORT_1050_674e,0x139,0x17f,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x2d,0x64);
  pass1_1030_3af6(&USHORT_1050_6758,0x13a,0x180,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffe7,0x7d);
  pass1_1030_3af6(&USHORT_1050_6762,0x13b,0x181,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x5,0xaf);
  pass1_1030_3af6(&USHORT_1050_676c,0x13c,0x182,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0xc8);
  pass1_1030_3af6(&USHORT_1050_6776,0x13d,0x183,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffce,0xc8);
  pass1_1030_3af6(&USHORT_1050_6780,0x13e,0x184,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xf,0xfa);
  pass1_1030_3af6(&USHORT_1050_678a,0x13f,0x185,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x1e,0x113);
  pass1_1030_3af6(&USHORT_1050_6794,0x140,0x186,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffe2,0x12c);
  pass1_1030_3af6(&USHORT_1050_679e,0x141,0x187,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x64,0x12c);
  pass1_1030_3af6(&USHORT_1050_67a8,0x142,0x188,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x32,0x145);
  pass1_1030_3af6(&USHORT_1050_67b2,0x143,0x189,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x64,0x145);
  pass1_1030_3af6(&USHORT_1050_67bc,0x144,0x18a,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x1e,0x15e);
  pass1_1030_3af6(&USHORT_1050_67c6,0x145,0x18b,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffd3,0x15e);
  pass1_1030_3af6(&USHORT_1050_67d0,0x146,0x18c,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x32,0xfa);
  pass1_1030_3af6(&USHORT_1050_67da,0x147,0x18d,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xf,0x19);
  pass1_1030_3af6(&USHORT_1050_67e4,0x148,0x18e,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0x32);
  pass1_1030_3af6(&USHORT_1050_67ee,0x149,0x18f,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0xaf);
  pass1_1030_3af6(&USHORT_1050_67f8,0x14a,0x190,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xfffb,0xe1);
  pass1_1030_3af6(&USHORT_1050_6802,0x14b,0x191,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xa,0x15e);
  pass1_1030_3af6(&USHORT_1050_680c,0x14c,0x192,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0x19);
  pass1_1030_3af6(&USHORT_1050_6816,0x14d,0x193,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x1e,0x32);
  pass1_1030_3af6(&USHORT_1050_6820,0x14e,0x194,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xfffb,0x64);
  pass1_1030_3af6(&USHORT_1050_682a,0x14f,0x195,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xf,0x64);
  pass1_1030_3af6(&USHORT_1050_6834,0x150,0x196,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x1e,0x7d);
  pass1_1030_3af6(&USHORT_1050_683e,0x151,0x197,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffdd,0xe1);
  pass1_1030_3af6(&USHORT_1050_6848,0x152,0x198,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0x113);
  pass1_1030_3af6(&USHORT_1050_6852,0x153,0x199,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x2d,0x12c);
  pass1_1030_3af6(&USHORT_1050_685c,0x154,0x19a,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffe7,0x145);
  pass1_1030_3af6(&USHORT_1050_6866,0x155,0x19b,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xa,0x15e);
  pass1_1030_3af6(&USHORT_1050_6870,0x156,0x19c,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0x4b);
  pass1_1030_3af6(&USHORT_1050_687a,0x157,0x19d,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x5,0x64);
  pass1_1030_3af6(&USHORT_1050_6884,0x158,0x19e,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0xffec,0x96);
  pass1_1030_3af6(&USHORT_1050_688e,0x159,0x19f,puVar1,
                  (puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_8),0x0,0x0,0x113);
  puVar2 = pass1_1030_3af6(&USHORT_1050_6898,0x15a,0x1a0,puVar1,
                           (puVar1 >> 0x10));
  return puVar2;
}


fn pass1_1030_4538(param_1: *mut u32)
{
  let uVar1: u16;
  
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  uVar1 = (param_1 >> 0x10);
  fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x12),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x15c),0x1000);
  return;
}


fn pass1_1030_4594(uchar *param_1,param_2: u16,param_3: u16,param_4: i16)
{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  let puVar7: *mut u16;
  let puStack8: *mut u16;
  
  puVar2 = (u16 *)(param_4 - 0x1U);
  mem_op_1000_179c(0x10,param_1,0x1000);
  puStack8 = (u16 *)(puVar2 & 0xffff | ZEXT24(param_1) << 0x10);
  uVar3 = param_1 | puVar2;
  if (uVar3 == 0x0) {
    puStack8 = (u16 *)0x0;
  }
  else {
    puVar7 = pass1_1008_3e38((u16 *)CONCAT22(param_1,puVar2 + 0x4));
    uVar3 = (puVar7 >> 0x10);
    puVar2 = puStack8;
  }
  uVar1 = SUB42(puVar2,0x0);
  iVar4 = (param_4 - 0x1U) * 0x12;
  load_string_1010_84ac
            (_PTR_LOOP_1050_14cc,(INT16)(_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  uVar6 = (puStack8 >> 0x10);
  iVar5 = puStack8;
  *puStack8 = uVar1;
  (iVar5 + 0x2) = uVar3;
  (iVar5 + 0xa) = (iVar4 + 0x51ba);
  pass1_1008_3e76((u16 *)(puStack8 & 0xffff0000 | (iVar5 + 0x4)),
                  (iVar4 + 0x51c0),(iVar4 + 0x51be),
                  (iVar4 + 0x51bc));
  (iVar5 + 0xc) = iVar4 + 0x51c2;
  (iVar5 + 0xe) = &USHORT_1050_1050;
  return;
}



fn pass1_1030_4628(uchar *param_1,param_2: u16,param_3: u16,param_4: i16)
{
  let uVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let iVar5: i16;
  let iVar6: i16;
  let piVar7: *mut i16;
  let uVar8: u16;
  let iStack24: i16;
  let piStack20: *mut i16;
  let uStack10: i16;
  let piStack8: *mut i16;
  
  uVar2 = param_4 - 0x1;
  uVar3 = uVar2;
  mem_op_1000_179c(0x28,param_1,0x1000);
  piStack20 = (i16 *)CONCAT22(param_1,uVar3);
  if ((param_1 | uVar3) == 0x0) {
    piStack8 = (i16 *)0x0;
  }
  else {
    pass1_1008_3e38((u16 *)CONCAT22(param_1,uVar3 + 0x6));
    piStack8 = piStack20;
  }
  uVar8 = (piStack8 >> 0x10);
  iVar5 = piStack8;
  (iVar5 + 0x2) = 0x0;
  iVar6 = uVar2 * 0x5e;
  pass1_1008_3e76((u16 *)(piStack8 & 0xffff0000 | (iVar5 + 0x6)),
                  (iVar6 + 0x5336),(iVar6 + 0x5334),
                  (iVar6 + 0x5332));
  (iVar5 + 0xc) = (iVar6 + 0x5348);
  *piStack8 = param_4;
  (iVar5 + 0xe) = (iVar6 + 0x534a);
  iStack10 = 0x0;
  do {
    uVar3 = ((uVar2 * 0x2f + iStack10) * 0x2 + 0x5338);
    (iVar5 + iStack10 * 0x2 + 0x12) = uVar3;
    iStack10 += 0x1;
  } while (iStack10 < 0x8);
  uVar1 = (&DAT_1050_5350 + uVar2 * 0x5e);
  pass1_1008_612e(uVar1,(uVar1 >> 0x10),uVar3);
  (iVar5 + 0x22) = uVar3;
  piVar7 = (i16 *)(uVar2 * 0x5e + 0x5354);
  *(i16 **)(iVar5 + 0x24) = piVar7;
  (iVar5 + 0x26) = &USHORT_1050_1050;
  iVar6 = *piVar7;
  pass1_1000_4906((astruct_20 *)CONCAT22(0x1050,piVar7),(WNDCLASS16 *)0x0,0x1e);
  iStack10 = 0x0;
LAB_1030_474c:
  if (uVar3 <= iStack10) {
    return;
  }
  do {
    iVar4 = (uVar2 * 0x5e + 0x534e) + iVar6 + -0x1;
    pass1_1008_612e(iVar6,iVar4,iVar4);
    iStack24 = 0x0;
    while( true ) {
      if (iStack10 < iStack24) {
        uVar1 = (iVar5 + 0x24);
        (uVar1 + iStack10 * 0x2) = iVar4;
        iStack10 += 0x1;
        goto LAB_1030_474c;
      }
      uVar1 = (iVar5 + 0x24);
      if ((uVar1 + iStack24 * 0x2) == iVar4) break;
      iStack24 += 0x1;
    }
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_4782(uchar *param_1,param_2: u8,uchar *param_3,param_4: u16,param_5: u16,
               param_6: i16,param_7: i16,param_8: i16)

{
  let iVar1: i16;
  let uVar2: u16;
  uchar **ppuVar3;
  let puVar4: *mut u8
  let uVar5: u16;
  let uVar6: u16;
  let iVar7: i16;
  let unaff_DI: i16;
  let uVar8: u16;
  let uVar9: u16;
  let puVar10: *mut u16;
  astruct_43 *paVar11;
  let uVar12: u32;
  let iStack220: i16;
  let local_c4: *mut u8
  let puStack194: *mut u8
  let local_c0: *mut u8;
  let uStack190: u16;
  let iStack188: i16;
  astruct_18 *paStack184;
  let iStack180: i16;
  astruct_18 *paStack178;
  astruct_18 *paStack174;
  let uStack170: u16;
  let uStack168: u16;
  let uStack166: u16;
  let uStack164: u16;
  let uStack162: u16;
  uchar **ppuStack160;
  let iStack158: i16;
  let iStack156: i16;
  let iStack154: i16;
  let uStack152: u16;
  char *pcStack150;
  uchar local_92 [0x80];
  let uStack18: u32;
  let uStack14: u32;
  let uStack10: u16;
  let uStack8: u16;
  let piStack6: *mut i16;
  
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_3,0x1000);
    PTR_LOOP_1050_5f2e = param_3;
  }
  else {
  }
  local_c4 = PTR_LOOP_1050_5f2c;
  puStack194 = PTR_LOOP_1050_5f2e;
  uVar2 = fn_ptr_op_1000_1708(0x20,0x0,0x1,PTR_LOOP_1050_5f2c,
                              PTR_LOOP_1050_5f2e,0x1000);
  paStack184 = (astruct_18 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar2);
  puVar4 = (uchar *)(PTR_LOOP_1050_5f2e | uVar2);
  if (puVar4 == (uchar *)0x0) {
    uVar2 = 0x0;
    puVar4 = (uchar *)0x0;
  }
  else {
    pass1_1030_84ae(CONCAT22(PTR_LOOP_1050_5f2e,uVar2));
  }
  piStack6 = (i16 *)CONCAT22(puVar4,uVar2);
  *piStack6 = param_8;
  pass1_1008_3f62((u16 *)
                  CONCAT13((char)(puVar4 >> 0x8),CONCAT12((char)puVar4,uVar2 + 0x8))
                  ,(u16 *)CONCAT22(0x1050,&USHORT_1050_65e6 + param_8 * 0xa));
  if (param_7 != 0x0) {
    puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,puVar4,unaff_DI);
    uStack8 = (puVar10 >> 0x10);
    uStack10 = SUB42(puVar10,0x0);
    uStack14 = pass1_1018_04b8(puVar10);
    uVar5 = (uStack14 >> 0x10);
    uVar2 = uStack14;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,uVar5);
    uStack18 = CONCAT22(uVar5,uVar2);
    pcStack150 = load_string_1010_847e
                           (_PTR_LOOP_1050_14cc,(INT16)(_PTR_LOOP_1050_14cc >> 0x10),
                            0x1010);
    uVar6 = (pcStack150 >> 0x10);
    uVar2 = pass1_1030_2a98(uStack18);
    uVar8 = (piStack6 >> 0x10);
    (piStack6 + 0x2) = uVar2;
    sys_1000_3f9c(local_92,param_1,pcStack150,(pcStack150 >> 0x10),
                  uVar2,&stack0xfffe,uVar8,0x1000,param_1,param_2);
    uVar2 = str_op_1008_60e8(CONCAT22(param_1,local_92),uVar6);
    uVar8 = (piStack6 >> 0x10);
    (piStack6 + 0x4) = uVar2;
    (piStack6 + 0x6) = uVar6;
    paVar11 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,(param_8 * 0xa + 0x65ec),
                                  param_1);
    uVar8 = (piStack6 >> 0x10);
    (piStack6 + 0xe) = paVar11;
    (piStack6 + 0x10) = (paVar11 >> 0x10);
    paVar11 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,(param_8 * 0xa + 0x65ee),
                                  param_1);
    uVar8 = (piStack6 >> 0x10);
    iVar7 = piStack6;
    (iVar7 + 0x12) = paVar11;
    (iVar7 + 0x14) = (paVar11 >> 0x10);
    uVar12 = pass1_1008_4772(*(astruct_76 **)(iVar7 + 0xe));
    uStack152 = (uVar12 >> 0x10);
    iStack154 = uVar12;
    iStack156 = (iStack154 + 0x4) + -0x1;
    iStack158 = (iStack154 + 0x8) + -0x1;
    if (param_6 != 0x0) {
      ppuStack160 = (uchar **)(&PTR_LOOP_1050_000e + 0x1);
      if (uStack14 == 0x0) {
        debug_print_1008_6048
                  (s_get_site_data_without_planet__1050_56de,0x1008,param_1);
      }
      else {
        ppuVar3 = &local_c4;
        pass1_1030_2f1a(uStack18,(u16 *)
                                 CONCAT13((char)(param_1 >> 0x8),
                                          CONCAT12((char)param_1,&local_c0)),
                        (u16 *)CONCAT22(param_1,ppuVar3));
        pass1_1008_612e(local_c4,local_c0,ppuVar3);
        ppuStack160 = ppuVar3;
      }
      iVar7 = ppuStack160 * 0xa;
      uVar8 = (piStack6 >> 0x10);
      (piStack6 + 0x1c) = iVar7;
      (piStack6 + 0x1c) = iVar7 / 0x64;
      puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_1,
                                (uchar *)(iVar7 % 0x64),unaff_DI);
      puStack194 = (uchar *)(puVar10 >> 0x10);
      local_c4 = (uchar *)puVar10;
      local_c0 = PTR_LOOP_1050_13ae;
      uVar2 = 0x84;
      puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,param_1,puStack194,
                                unaff_DI);
      uStack190 = pass1_1010_65d0(param_1,puVar10,uVar2);
      iStack188 = 0x3c;
      if (local_c0 < 0x3) {
        if (0x0 < uStack190) {
          iStack188 = 0x5a;
        }
      }
      else {
        if (uStack190 == 0x1) {
          iStack188 = 0x44;
        }
        else {
          if (uStack190 == 0x2) {
            iStack188 = 0x4b;
          }
          else {
            if (uStack190 == 0x3) {
              iStack188 = 0x53;
            }
            else {
              if (uStack190 == 0x4) {
                iStack188 = 0x5a;
              }
            }
          }
        }
      }
      iVar7 = iStack188 * ppuStack160;
      ppuStack160 = (uchar **)(iVar7 / 0x64);
      puVar4 = (uchar *)(iVar7 % 0x64);
      uVar8 = (piStack6 >> 0x10);
      (piStack6 + 0x1a) = ppuStack160;
      uStack164 = ppuStack160 + (piStack6 + 0x1c);
      uVar2 = uStack164 * 0x6;
      uStack162 = uStack164;
      mem_op_1000_179c(uVar2,puVar4,0x1000);
      paStack184 = (astruct_18 *)CONCAT22(puVar4,uVar2);
      PTR_LOOP_1050_5f2e = (puVar4 | uVar2);
      if (PTR_LOOP_1050_5f2e == 0x0) {
        (piStack6 + 0x16) = 0x0;
      }
      else {
        pass1_1000_5586((uchar *)0x3e38,0x1008,uStack164,0x6,uVar2,puVar4);
        (piStack6 + 0x16) = paStack184;
      }
      uStack170 = uStack162 * 0x2;
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
      }
      else {
      }
      uVar2 = fn_ptr_op_1000_1708(uStack170,0x0,0x1,PTR_LOOP_1050_5f2c,
                                  PTR_LOOP_1050_5f2e,0x1000);
      paStack174 = (astruct_18 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar2);
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
      }
      else {
      }
      uVar2 = fn_ptr_op_1000_1708(uStack170,0x0,0x1,PTR_LOOP_1050_5f2c,
                                  PTR_LOOP_1050_5f2e,0x1000);
      paStack178 = (astruct_18 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar2);
      iStack180 = 0x0;
LAB_1030_4b57:
      uVar5 = uStack162;
      if (iStack180 < uStack162) {
        do {
          pass1_1008_612e(0x0,iStack156,uVar5);
          uStack166 = uVar5;
          pass1_1008_612e(0x0,iStack158,uVar5);
          iStack220 = 0x0;
          while( true ) {
            iVar7 = paStack174;
            uVar8 = (paStack174 >> 0x10);
            uVar9 = (paStack178 >> 0x10);
            uStack168 = uVar5;
            if (iStack180 <= iStack220) {
              iVar1 = iStack180 * 0x2;
              (iVar1 + iVar7) = uStack166;
              (iVar1 + paStack178) = uVar5;
              uVar12 = (piStack6 + 0x16);
              pass1_1008_3e76((u16 *)
                              (uVar12 & 0xffff0000 |
                              (uVar12 + iStack180 * 0x6)),0x0,uVar5,
                              (iVar1 + iVar7));
              iStack180 += 0x1;
              goto LAB_1030_4b57;
            }
            if (((iStack220 * 0x2 + iVar7) == uStack166) &&
               ((iStack220 * 0x2 + paStack178) == uVar5)) break;
            iStack220 += 0x1;
          }
        } while( true );
      }
      fn_ptr_1000_17ce(paStack174,0x1000);
      paStack184 = paStack178;
      fn_ptr_1000_17ce(paStack178,0x1000);
    }
  }
  return;
}



fn pass1_1030_4bbe(param_1: u16,param_2: u16,param_3: u32,param_4: i16)
{
  let puVar1: u32;
  let puVar2: u32;
  let uVar3: u16;
  let iVar4: i16;
  astruct_117 *iVar5;
  let puVar5: u32;
  let puVar6: u32;
  let uVar7: u16;
  
  uVar7 = (param_3 >> 0x10);
  iVar5 = (astruct_117 *)param_3;
  if (iVar5->field_0x12 == 0x0) {
    pass1_1030_4f5a(param_1,param_2,param_3 & 0xffff | uVar7 << 0x10);
  }
  puVar6 = &iVar5->field_0x16;
  uVar3 = (&iVar5->field_0x12 + 0x2);
  puVar5 = (&iVar5->field_0x12 + param_4 * 0x98);
  for (iVar4 = 0x26; iVar4 != 0x0; iVar4 += -0x1) {
    puVar2 = puVar6;
    puVar6 = puVar6 + 0x1;
    puVar1 = puVar5;
    puVar5 = puVar5 + 0x1;
    *puVar2 = *puVar1;
  }
  return;
}



fn pass1_1030_4c06(param_1: u32,param_2: i16,param_3: u16,param_4: u16)
{
  let puVar1: u32;
  let puVar2: u32;
  let uVar3: u16;
  let puVar4: u32;
  let iVar5: i16;
  let puVar6: u32;
  let uVar7: u16;
  
  uVar7 = (param_1 >> 0x10);
  iVar5 = param_1;
  if (*(long *)(iVar5 + 0x15c) == 0x0) {
    pass1_1030_5044(param_1 & 0xffff | uVar7 << 0x10,param_4,param_3);
  }
  puVar4 = (iVar5 + 0xae);
  uVar3 = (iVar5 + 0x15e);
  puVar6 = ((iVar5 + 0x15c) + param_2 * 0xae);
  for (iVar5 = 0x2b; iVar5 != 0x0; iVar5 += -0x1) {
    puVar2 = puVar4;
    puVar4 = puVar4 + 0x1;
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    *puVar2 = *puVar1;
  }
  puVar4 = puVar6;
  return;
}



void 
pass1_1030_4c52(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: u16,
               param_6: u16)

{
  let uVar1: u16;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  char *pcStack8;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    uVar1 = pass1_1000_47a4(param_4,0x1050518a,param_6);
    pcStack8 = CONCAT22(param_5,uVar1);
    if ((param_5 | uVar1) == 0x0) break;
    if (*pcStack8 != '\"') {
      iVar2 = pass1_1000_3e2c(CONCAT22(param_5,uVar1));
      iVar3 = param_3;
      uVar4 = (param_3 >> 0x10);
      if (iStack4 < 0x25) {
        (iStack4 * 0x4 + iVar3) = iVar2;
        (iStack4 * 0x4 + iVar3 + 0x2) = param_5;
      }
      else {
        if (iStack4 == 0x25) {
          (iVar3 + 0x94) = iVar2;
        }
        else {
          if (iStack4 == 0x26) {
            (iVar3 + 0x96) = iVar2;
          }
          else {
            if (iStack4 == 0x27) {
              (iVar3 + 0x98) = iVar2;
            }
            else {
              if (iStack4 == 0x28) {
                (iVar3 + 0x9a) = iVar2;
              }
              else {
                if (iStack4 == 0x29) {
                  (iVar3 + 0x9c) = iVar2;
                }
                else {
                  if (iStack4 == 0x2a) {
                    (iVar3 + 0x9e) = iVar2;
                  }
                  else {
                    if (iStack4 == 0x2b) {
                      (iVar3 + 0xa0) = iVar2;
                    }
                    else {
                      if (iStack4 == 0x2c) {
                        (iVar3 + 0xa2) = iVar2;
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      iStack4 += 0x1;
    }
    param_4 = 0x0;
  }
  return;
}



fn pass1_1030_4d3a(param_1: u16,param_2: u16,param_3: u16,param_4: u32,param_5: u32)
{
  let uVar1: u16;
  let iVar2: i16;
  astruct_118 *iVar3;
  let uVar3: u16;
  let unaff_SS: u16;
  char *pcStack8;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    uVar1 = pass1_1000_47a4(param_5,0x1050518a,unaff_SS);
    pcStack8 = CONCAT22(param_1,uVar1);
    if ((param_1 | uVar1) == 0x0) break;
    if (*pcStack8 != '\"') {
      iVar2 = pass1_1000_3e2c(CONCAT22(param_1,uVar1));
      iVar3 = (astruct_118 *)param_4;
      uVar3 = (param_4 >> 0x10);
      if (iStack4 < 0x25) {
        (&iVar3->field_0x0 + iStack4 * 0x4) = iVar2;
        (&iVar3->field_0x2 + iStack4 * 0x4) = param_1;
      }
      else {
        if (iStack4 == 0x25) {
          iVar3->field_0x94 = iVar2;
        }
        else {
          if (iStack4 == 0x26) {
            iVar3->field_0x96 = iVar2;
          }
        }
      }
      iStack4 += 0x1;
    }
    param_5 = 0x0;
  }
  return;
}



fn pass1_1030_4dbc(param_1: u32,param_2: u32,param_3: i32)
{
  long *plVar1;
  let piVar2: *mut i16;
  let lVar3: i32;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u16;
  
  iVar5 = param_1;
  uVar6 = (param_1 >> 0x10);
  if (0x0 < param_3) {
    (iVar5 + 0x160) = param_2;
    *(long *)(iVar5 + 0x164) = param_3;
  }
  if ((*(long *)(iVar5 + 0x160) == 0x0) ||
     (lVar3 = *(long *)(iVar5 + 0x164), plVar1 = (long *)(iVar5 + 0x164),
     *plVar1 = *plVar1 + -0x1, lVar3 == 0x0)) {
    (iVar5 + 0x160) = 0x0;
  }
  else {
    uVar4 = str_op_1000_3da4(*(char **)(iVar5 + 0x160));
    piVar2 = (i16 *)(iVar5 + 0x160);
    *piVar2 = *piVar2 + uVar4 + 0x2;
  }
  return;
}



fn pass1_1030_4e34(param_1: u16,param_2: u16,param_3: i32,char *param_4)
{
  while (param_3 != 0x0) {
    if ((*param_4 == '\r') || (*param_4 == '\n')) {
      *param_4 = '\0';
    }
    param_4 = (param_4 & 0xffff0000 | (param_4 + 0x1));
    param_3 = param_3 + -0x1;
  }
  return;
}


fn pass1_1030_4f5a(param_1: u16,param_2: u16,param_3: u32)
{
  let uVar1: u16;
  char *pcVar2;
  long *plVar3;
  let uVar4: u16;
  let iVar5: i16;
  char *pcVar6;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uStack22: u16;
  let uStack20: u32;
  let uStack14: u16;
  let uStack12: u16;
  let local_a: i32;
  char *local_6;
  
  plVar3 = &local_a;
  PTR_LOOP_1050_5f2e =
       
       read_file_1030_4e70(param_3,CONCAT22(param_1,plVar3),
                           (byte **)CONCAT22(param_1,&local_6),
                           (long)s_bldgbld_dat_1050_56fc,param_2);
  pcVar2 = local_6;
  if (plVar3 != (long *)0x0) {
    uVar7 = param_3;
    uVar8 = (param_3 >> 0x10);
    pcVar6 = local_6;
    pass1_1030_4e34(uVar7,uVar8,local_a,local_6);
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(pcVar6 * 0x98,0x0,0x1,PTR_LOOP_1050_5f2c,
                                PTR_LOOP_1050_5f2e,0x1000);
    (uVar7 + 0x12) = uVar4;
    (uVar7 + 0x14) = PTR_LOOP_1050_5f2e;
    pass1_1030_4dbc(param_3,local_6,pcVar6 & 0xffff);
    uStack20 = CONCAT22(extraout_DX,uVar4);
    for (uStack22 = 0x0; uStack22 < pcVar6; uStack22 += 0x1) {
      uVar1 = (uVar7 + 0x14);
      iVar5 = (uVar7 + 0x12) + uStack22 * 0x98;
      pass1_1030_4d3a(uVar1,uVar7,uVar8,CONCAT22(uVar1,iVar5),uStack20);
      pass1_1030_4dbc(param_3,0x0,0x0);
      uStack20 = CONCAT22(extraout_DX_00,iVar5);
    }
    uStack12 = (pcVar2 >> 0x10);
    uStack14 = pcVar2;
    if ((uStack12 | uStack14) != 0x0) {
      call_fn_ptr_1000_0dc6(uStack14,uStack12,0x1000);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_5044(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  char *pcVar2;
  long *plVar3;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let iVar7: i16;
  char *pcVar8;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar10: u16;
  let uVar11: u16;
  let uStack28: u32;
  let uStack24: u16;
  let uStack22: u32;
  let uStack14: u16;
  let uStack12: u16;
  let local_a: i32;
  char *local_6;
  let uVar9: u32;
  
  plVar3 = &local_a;
  PTR_LOOP_1050_5f2e =
       
       read_file_1030_4e70(param_1,CONCAT22(param_2,plVar3),
                           (byte **)CONCAT22(param_2,&local_6),
                           (long)s_bldgops_dat_1050_5708,param_3);
  pcVar2 = local_6;
  if (plVar3 != (long *)0x0) {
    uVar10 = param_1;
    uVar11 = (param_1 >> 0x10);
    pcVar8 = local_6;
    pass1_1030_4e34(uVar10,uVar11,local_a,local_6);
    uVar4 = pcVar8;
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar5 = fn_ptr_op_1000_1708(uVar4 * 0xae,0x0,0x1,PTR_LOOP_1050_5f2c,
                                PTR_LOOP_1050_5f2e,0x1000);
    uVar9 = uVar5;
    uStack28 = CONCAT22(PTR_LOOP_1050_5f2e,uVar5);
    if ((PTR_LOOP_1050_5f2e | uVar5) == 0x0) {
      (uVar10 + 0x15c) = 0x0;
    }
    else {
      pass1_1000_5586((uchar *)0x51f0,0x1030,uVar4,0xae,uVar5,PTR_LOOP_1050_5f2e);
      (uVar10 + 0x15c) = uStack28;
      uVar9 = uStack28;
    }
    uVar6 = uVar9;
    pass1_1030_4dbc(param_1,local_6,pcVar8 & 0xffff);
    uStack22 = CONCAT22(extraout_DX,uVar6);
    for (uStack24 = 0x0; uStack24 < uVar4; uStack24 += 0x1) {
      uVar1 = (uVar10 + 0x15e);
      iVar7 = (uVar10 + 0x15c) + uStack24 * 0xae;
      pass1_1030_4c52(uVar10,uVar11,CONCAT22(uVar1,iVar7),uStack22,uVar1,param_2);
      pass1_1030_4dbc(param_1,0x0,0x0);
      uStack22 = CONCAT22(extraout_DX_00,iVar7);
    }
    uStack12 = (pcVar2 >> 0x10);
    uStack14 = pcVar2;
    if ((uStack12 | uStack14) != 0x0) {
      call_fn_ptr_1000_0dc6(uStack14,uStack12,0x1000);
    }
  }
  return;
}



fn pass1_1030_5164(param_1: u32,Uparam_2: i32,param_3: u16) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  let lVar3: i32;
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(param_3,local_a),(param_1 + 0x568));
  do {
    lVar3 = pass1_1008_5b12(local_a,param_3);
    if (lVar3 == 0x0) {
      return param_2;
    }
    uVar1 = param_1 + 0x168;
    unk_str_op_1000_3d3e
              ((param_1 & 0xffff0000 | uVar1),*(char **)(lVar3 + 0x4))
    ;
    pass1_1000_3cea(param_1 & 0xffff0000 | uVar1,param_2);
    uVar2 = dos3_call_1000_51aa(&stack0xfffe);
  } while (uVar2 != 0x0);
  return param_1 & 0xffff0000 | uVar1;
}



fn pass1_1030_51eb(void)
{
  let unaff_SS: u16;
  
  pass1_1030_3b28(unaff_SS);
  return;
}



fn pass1_1030_51f0(param_1: u32) -> u32

{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0xa4) = 0x0;
  (iVar1 + 0xa6) = 0x0;
  (iVar1 + 0xa8) = 0x0;
  (iVar1 + 0xaa) = 0x0;
  (iVar1 + 0xac) = 0x0;
  return param_1;
}



fn pass1_1030_521c(astruct_100 *param_1,param_2: u32,param_3: u16,param_4: u8)
{
  let iVar1: i16;
  let puVar2: *mut u8
  
  struct_op_1028_d1dc(param_3,param_4,param_1,0x32c7);
  puVar2 = (uchar *)(param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x108) = param_2;
  param_1->field_0x0 = 0x55fe;
  (iVar1 + 0x2) = 0x1030;
  sys_1000_3f9c((uchar *)(iVar1 + 0x8),puVar2,s_SCGenKids_0x_08lx_1050_5714,
                &USHORT_1050_1050,param_2,&stack0xfffe,puVar2,0x1000,
                param_3,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_5260(param_1: u32,param_2: u16,param_3: u16) -> u16

{
  let uVar1: u32;
  code **ppcVar2;
  let puStack6: u32;
  
  uVar1 = (param_1 + 0x108);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  puStack6 = CONCAT22(param_3,param_2);
  ppcVar2 = (code **)(*puStack6 + 0x14);
  (**ppcVar2)();
  return 0x1;
}



fn pass1_1030_5290(param_1: u32,astruct_376 *param_2,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  let iVar3: i16;
  astruct_377 *iVar5;
  let puVar4: u32;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x10c,param_3,0x1000);
  puStack10 = (u16 *)CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    uVar6 = (param_1 >> 0x10);
    iVar5 = (astruct_377 *)param_1;
    param_2->field_0x4 = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = &param_2->field_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = &USHORT_1050_1028;
    param_2->field_0x108 = iVar5->field_0x108;
    *puStack10 = 0x55fe;
    param_2->field_0x2 = 0x1030;
  }
  return;
}



fn pass1_1030_532e(astruct_100 *param_1,param_2: u32,param_3: u16,param_4: u8)
{
  let iVar1: i16;
  let puVar2: *mut u8
  
  struct_op_1028_d1dc(param_3,param_4,param_1,0x32c7);
  puVar2 = (uchar *)(param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x108) = param_2;
  param_1->field_0x0 = 0x55ee;
  (iVar1 + 0x2) = 0x1030;
  sys_1000_3f9c((uchar *)(iVar1 + 0x8),puVar2,s_SCSelect__u___d_1050_5726,
                &USHORT_1050_1050,(iVar1 + 0x4),
                &stack0xfffe,puVar2,0x1000,param_3,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_538a(param_1: u32,param_2: i16,param_3: u16) -> u16

{
  let puVar1: *mut u8
  let uVar2: u16;
  astruct_694 *iVar4;
  let uVar3: u16;
  let puVar4: *mut u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar4 = (astruct_694 *)param_1;
  puVar1 = *(uchar **)(&iVar4->field_0x108 + 0x2);
  uVar2 = puVar1 >> 0x8;
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,puVar1,param_2);
  if (uVar2 == 0x1) {
    pass1_1018_04ca(puVar4,iVar4->field_0x108);
  }
  else {
    if (uVar2 == 0x2) {
      pass1_1018_04a4(puVar4,iVar4->field_0x108);
    }
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_53f4(param_1: u32,param_2: u16,param_3: u16,param_4: u8)
{
  let uVar1: u32;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u32;
  let bStack291: u8;
  u8 local_11e [0x10e];
  let uStack16: u32;
  let uStack12: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  uStack12 = (iVar3 + 0x108);
  uStack12._3_1_ = (char)(uStack12 >> 0x18);
  if (uStack12._3_1_ == -0x1) {
    uVar5 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,param_2,
                            (byte)((iVar3 + 0x108) >> 0x18));
    param_2 = (uVar5 >> 0x10);
  }
  else {
    uStack16 = (iVar3 + 0x108);
    uStack16._3_1_ = (char)(uStack16 >> 0x18);
    if (uStack16._3_1_ == '\x03') {
      pass1_1028_e44a(_PTR_LOOP_1050_65e2,*(long *)(iVar3 + 0x108),param_3);
    }
    else {
      uVar1 = (iVar3 + 0x108);
      pass1_1028_e372(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10),
                      param_3);
    }
  }
  uStack12 = (iVar3 + 0x108);
  uStack12._3_1_ = (char)(uStack12 >> 0x18);
  if (uStack12._3_1_ != '\x03') {
    pass1_1030_521c((astruct_100 *)
                    CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,local_11e)),
                    (iVar3 + 0x108),param_3,param_4);
    uStack16 = *_PTR_LOOP_1050_5748;
    fn_ptr_1028_d566(uStack16,CONCAT22(param_3,local_11e));
    bStack291 = (byte)((iVar3 + 0x108) >> 0x18);
    uVar2 = bStack291;
    if (bStack291 == 0x2) {
      uVar1 = (iVar3 + 0x108);
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
      pass1_1010_82f8(_PTR_LOOP_1050_14cc,**(u16 **)(uVar2 + 0x10));
    }
  }
  return;
}



fn pass1_1030_54f8(astruct_378 *param_1,uchar *param_2,param_3: u32)
{
  let puVar1: u32;
  let puVar2: u32;
  let iVar3: i16;
  astruct_379 *iVar5;
  let puVar4: u32;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x10c,param_2,0x1000);
  puStack10 = (u16 *)CONCAT22(param_2,param_1);
  if ((param_2 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field_0x2 = 0x1008;
    uVar6 = (param_3 >> 0x10);
    iVar5 = (astruct_379 *)param_3;
    param_1->field_0x4 = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = &param_1->field_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field_0x2 = &USHORT_1050_1028;
    param_1->field_0x108 = iVar5->field_0x108;
    *puStack10 = 0x55ee;
    param_1->field_0x2 = 0x1030;
  }
  return;
}



fn pass1_1030_5596(astruct_18 *param_1,param_2: u8) -> *mut astruct_18

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1030_55c2(astruct_18 *param_1,param_2: u8) -> *mut astruct_18

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1030_560e(param_1: *mut u16) -> *mut u16

{
  let iVar1: i16;
  let uVar2: u16;
  
  struct_1030_17ce(param_1,0x64,0x1f4);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x10) = 0x0;
  pass1_1008_3e38((u16 *)(param_1 & 0xffff0000 | (iVar1 + 0x14)));
  (iVar1 + 0x1a) = 0x0;
  (iVar1 + 0x1c) = 0x0;
  *param_1 = s_procLo_1050_5bd0;
  (iVar1 + 0x2) = 0x1030;
  return param_1;
}


fn pass1_1030_56b0(param_1: *mut u16)
{
  let uVar1: u16;
  astruct_18 *paVar2;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  *param_1 = s_procLo_1050_5bd0;
  (iVar3 + 0x2) = 0x1030;
  paVar2 = *(astruct_18 **)(iVar3 + 0x10);
  uVar1 = (iVar3 + 0x12);
  if ((uVar1 | paVar2) != 0x0) {
    fn_ptr_1030_84d0(paVar2 & 0xffff | uVar1 << 0x10);
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  pass1_1030_18b2(param_1);
  return;
}



fn pass1_1030_56f6(param_1: u32,param_2: u32,param_3: u16,param_4: u16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let uVar3: u32;
  let uVar4: u16;
  let BVar5: bool;
  let iVar6: i16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let local_e: [u16;0x3];
  let local_8: [u16;0x2];
  let iStack4: i16;
  
  uVar4 = pass1_1030_1978(param_1,param_2,param_3,param_4);
  if (uVar4 != 0x0) {
    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    local_e[0] = (iVar7 + 0x10);
    uVar4 = param_2;
    uVar9 = (param_2 >> 0x10);
    BVar5 = write_to_file_1008_7e1c
                      (uVar4,uVar9,local_e,param_4,0x2,0x1008);
    if (BVar5 != 0x0) {
      uVar3 = (iVar7 + 0x10);
      local_8[0] = (uVar3 + 0x2);
      BVar5 = write_to_file_1008_7e1c
                        (uVar4,uVar9,local_8,param_4,0x2,0x1008);
      if ((BVar5 != 0x0) &&
         (uVar3 = (iVar7 + 0x10),
         BVar5 = pass1_1008_7c2a(param_2,*(char **)(uVar3 + 0x4),0x1008),
         BVar5 != 0x0)) {
        uVar3 = (iVar7 + 0x10);
        local_8[0] = (uVar3 + 0x1a);
        BVar5 = write_to_file_1008_7e1c
                          (uVar4,uVar9,local_8,param_4,0x2,0x1008);
        if (BVar5 != 0x0) {
          for (iStack4 = 0x0; uVar3 = (iVar7 + 0x10),
              piVar1 = (i16 *)(uVar3 + 0x1a),
              *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 += 0x1) {
            uVar3 = (iVar7 + 0x10);
            uVar2 = (uVar3 + 0x16);
            iVar6 = write_to_file_1008_7b4c
                              (param_2,uVar2 & 0xffff0000 |
                                       (uVar2 + iStack4 * 0x6),0x1008,
                               param_4);
            if (iVar6 == 0x0) goto LAB_1030_5734;
          }
          iVar6 = write_to_file_1008_7b4c
                            (param_2,param_1 & 0xffff0000 | (iVar7 + 0x14),0x1008,
                             param_4);
          if (iVar6 != 0x0) {
            local_8[0] = (iVar7 + 0x1c);
            BVar5 = write_to_file_1008_7e1c
                              (uVar4,uVar9,local_8,param_4,0x2,0x1008);
            if (BVar5 != 0x0) {
              return;
            }
          }
        }
      }
    }
LAB_1030_5734:
    PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}


fn pass1_1030_5a52(param_1: u32,param_2: *mut u32,param_3: *mut u32)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x10);
  *param_3 = (uVar1 + 0xe);
  uVar1 = (param_1 + 0x10);
  *param_2 = (uVar1 + 0x12);
  return;
}



fn pass1_1030_5a80(param_1: u32,param_2: u32,param_3: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  let uVar3: u32;
  u8 local_20 [0xc];
  let local_14: u32;
  let uStack14: u32;
  let uStack10: u32;
  let iStack6: i16;
  let uStack4: u16;
  
  uVar2 = (param_1 >> 0x10);
  (param_1 + 0x10) = param_2;
  uVar3 = pass1_1008_4772(*(astruct_76 **)(param_2 + 0xe));
  uStack4 = (uVar3 >> 0x10);
  iStack6 = uVar3;
  uStack10 = (iStack6 + 0x4);
  uStack14 = (iStack6 + 0x8);
  pass1_1008_3e54((u16 *)CONCAT22(param_3,&local_14),0x0,uStack14 - 0x1,
                  uStack10 - 0x1);
  puVar1 = (param_1 + 0x14);
  pass1_1008_6cb4(CONCAT22(param_3,local_20),&local_14,param_3,puVar1,uVar2);
  pass1_1008_6d64((u16 *)CONCAT22(param_3,local_20),
                  (u16 *)(param_1 & 0xffff0000 | ZEXT24(puVar1)));
  return;
}



i16  pass1_1030_5b00(param_1: u32)

{
  return (param_1 + 0x4) + 0xb;
}



fn pass1_1030_5b1c(param_1: u32,param_2: *mut u16,param_3: *mut u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_3 = (param_1 + 0x1a);
  *param_2 = (param_1 + 0x1c);
  return;
}



fn pass1_1030_5b3e(param_1: u32,param_2: i16,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x1a) = param_3;
  if ((iVar1 + 0x1c) < param_2) {
    (iVar1 + 0x1c) = param_2;
  }
  return;
}



fn pass1_1030_5b5c(param_1: i16,param_2: u16) -> u32

{
  return CONCAT22(param_2,param_1 + 0x14);
}



fn pass1_1030_5b6c(param_1: u32,char *param_2,param_3: u16)
{
  let lVar1: i32;
  let uVar2: u16;
  astruct_610 *iVar4;
  astruct_609 *iVar3;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar4 = (astruct_610 *)param_1;
  if (iVar4->field_0x10 != 0x0) {
    lVar1 = iVar4->field_0x10;
    fn_ptr_1000_17ce(*(astruct_18 **)(lVar1 + 0x4),0x1000);
    uVar2 = str_op_1008_60e8(param_2,param_3);
    lVar1 = iVar4->field_0x10;
    uVar3 = (lVar1 >> 0x10);
    iVar3 = (astruct_609 *)lVar1;
    iVar3->field_0x4 = uVar2;
    iVar3->field_0x6 = param_3;
  }
  return;
}



astruct_18 *  pass1_1030_5baa(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_56b0((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_5bec(param_1: u32)
{
  _PTR_LOOP_1050_5736 = param_1;
  pass1_1000_54a0(param_1,0x0,0x24);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_5c0e(void)
{
  _PTR_LOOP_1050_5736 = 0x0;
  return;
}



fn pass1_1030_5c1a(param_1: u32,param_2: u32,param_3: u16) -> bool

{
  let BVar1: bool;
  
  BVar1 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar1 != 0x0) {
    BVar1 = write_to_file_1008_7e1c
                      (param_2,(param_2 >> 0x10),param_1,
                       (param_1 >> 0x10),0x24,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = 0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}


fn pass1_1030_5c8a(param_1: u32,param_2: u32)
{
  long *plVar1;
  let uVar2: u16;
  let uVar3: u32;
  let uVar4: u16;
  astruct_177 *iVar5;
  let uVar5: u16;
  let uStack6: u32;
  
  uStack6 = 0x0;
  uVar2 = param_2._3_1_;
  if (uVar2 == 0xff) {
    return;
  }
  uVar5 = (_PTR_LOOP_1050_65e2 >> 0x10);
  iVar5 = (astruct_177 *)(_PTR_LOOP_1050_65e2 + 0xa);
  uVar3 = (iVar5 + uVar2 * 0x4);
  uVar4 = (iVar5 + uVar2 * 0x4 + 0x2);
  if ((uVar3 + 0x4) != 0x0) {
    pass1_1030_12ca(uVar3 & 0xffff | uVar4 << 0x10);
    uStack6 = uVar3 & 0xffff | uVar4 << 0x10;
  }
  if (uStack6 == 0x0) {
    plVar1 = (long *)(uVar2 * 0x4 + param_1);
    *plVar1 = *plVar1 + 0x1;
  }
  return;
}



fn pass1_1030_5d0a(param_1: *mut u16) -> u16

{
  let uVar1: u16;
  
  struct_1030_17ce(param_1,0x1,0x4);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x10) = 0x0;
  *param_1 = 0x613e;
  (param_1 + 0x2) = 0x1030;
  return param_1;
}



fn pass1_1030_5d3c(param_1: *mut u16,param_2: u32,param_3: u16,uchar *param_4) -> u16

{
  let uVar1: u16;
  
  pass1_1030_183c(param_1,0x1,0x4,0x1000000,param_2,param_3,param_4);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x10) = 0x0;
  *param_1 = 0x613e;
  (param_1 + 0x2) = 0x1030;
  return param_1;
}



fn pass1_1030_5d78(param_1: *mut u16)
{
  let uVar1: u16;
  astruct_18 *paVar2;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  *param_1 = 0x613e;
  (iVar3 + 0x2) = 0x1030;
  paVar2 = *(astruct_18 **)(iVar3 + 0x10);
  uVar1 = (iVar3 + 0x12);
  if ((uVar1 | paVar2) != 0x0) {
    pass1_1030_8480((astruct_18 **)(paVar2 & 0xffff | uVar1 << 0x10));
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  pass1_1030_18b2(param_1);
  return;
}



fn pass1_1030_5dbe(param_1: u32,param_2: u32,param_3: u16,param_4: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  let Bvar4: bool;
  let iVar5: i16;
  let iVar6: i16;
  let uVar7: u16;
  let local_c: [u16;0x5];
  
  uVar3 = pass1_1030_1978(param_1,param_2,param_3,param_4);
  if (uVar3 != 0x0) {
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    BVar4 = pass1_1008_7c2a(param_2,*(char **)*(char **)(iVar6 + 0x10),0x1008);
    if ((BVar4 != 0x0) &&
       (uVar1 = (iVar6 + 0x10),
       iVar5 = write_to_file_1008_7b4c
                         (param_2,uVar1 & 0xffff0000 | (uVar1 + 0x4),0x1008,
                          param_4), iVar5 != 0x0)) {
      uVar2 = (iVar6 + 0x10);
      local_c[0] = (uVar2 + 0xa);
      uVar3 = (param_2 >> 0x10);
      BVar4 = write_to_file_1008_7e1c
                        (param_2,uVar3,local_c,param_4,0x2,0x1008)
      ;
      if (BVar4 != 0x0) {
        uVar2 = (iVar6 + 0x10);
        if ((uVar2 + 0xa) == 0x0) {
          return;
        }
        uVar2 = (iVar6 + 0x10);
        uVar7 = (uVar2 >> 0x10);
        iVar6 = uVar2;
        uVar2 = (iVar6 + 0xc);
        BVar4 = write_to_file_1008_7e1c
                          (param_2,uVar3,uVar2,
                           (uVar2 >> 0x10),
                           ((iVar6 + 0xa) * 0x2),0x1008);
        if (BVar4 != 0x0) {
          return;
        }
      }
    }
    PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn file_1030_5e70(param_1: u32,param_2: u32,param_3: i16,param_4: *mut u8,param_5: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  let puVar4: *mut u8;
  let uVar5: u16;
  let BVar6: bool;
  let uVar7: u16;
  let puVar8: *mut u8
  let iVar9: i16;
  let unaff_DI: i16;
  let uVar10: u16;
  let puVar11: *mut u16;
  let iVar12: i16;
  let uVar13: u16;
  let uVar14: u16;
  let uStack1034: u32;
  let local_402: [u8;400];
  
  iVar12 = param_1;
  uVar13 = (param_1 >> 0x10);
  file_1030_19b4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 != 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4,0x1000);
      PTR_LOOP_1050_5f2e = param_4;
    }
    else {
    }
    uVar3 = fn_ptr_op_1000_1708(0x10,0x0,0x1,PTR_LOOP_1050_5f2c,
                                PTR_LOOP_1050_5f2e,0x1000);
    uStack1034 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    puVar8 = (uchar *)(PTR_LOOP_1050_5f2e | uVar3);
    if (puVar8 == (uchar *)0x0) {
      (iVar12 + 0x10) = 0x0;
    }
    else {
      puVar11 = pass1_1008_3e38((u16 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar3 + 0x4));
      puVar8 = (uchar *)(puVar11 >> 0x10);
      (iVar12 + 0x10) = uStack1034;
    }
    puVar4 = local_402;
    uVar3 = param_2;
    uVar14 = (param_2 >> 0x10);
    read_file_1008_7c6e(uVar3,uVar14,CONCAT22(param_5,puVar4),0x1008);
    if (puVar4 != 0x0) {
      uVar5 = str_op_1008_60e8(CONCAT22(param_5,local_402),puVar8);
      puVar11 = *(u16 **)(iVar12 + 0x10);
      *puVar11 = uVar5;
      *(uchar **)(puVar11 + 0x2) = puVar8;
      uVar1 = (iVar12 + 0x10);
      BVar6 = read_file_1008_7bc8(param_2,(u16 *)
                                          (uVar1 & 0xffff0000 | (uVar1 + 0x4))
                                  ,0x1008,param_5);
      if (BVar6 != 0x0) {
        uVar2 = (iVar12 + 0x10);
        BVar6 = read_file_1008_7dee(uVar3,uVar14,uVar2 + 0xa,0x0,
                                    (uVar2 >> 0x10),0x2,0x1008);
        if (BVar6 != 0x0) {
          uVar2 = (iVar12 + 0x10);
          uVar10 = (uVar2 >> 0x10);
          iVar9 = uVar2;
          if ((iVar9 + 0xa) == 0x0) {
LAB_1030_5fb7:
            puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_5,puVar8,unaff_DI);
            pass1_1018_04ca(puVar11,(iVar12 + 0x4));
            return;
          }
          uVar5 = (iVar9 + 0xa) * 0x2;
          uVar7 = uVar5;
          mem_op_1000_179c(uVar5,puVar8,0x1000);
          uVar2 = (iVar12 + 0x10);
          uVar10 = (uVar2 >> 0x10);
          iVar9 = uVar2;
          (iVar9 + 0xc) = uVar7;
          *(uchar **)(iVar9 + 0xe) = puVar8;
          uVar2 = (iVar12 + 0x10);
          uVar2 = (uVar2 + 0xc);
          BVar6 = read_file_1008_7dee(uVar3,uVar14,uVar2,0x0,
                                      (uVar2 >> 0x10),uVar5,0x1008);
          if (BVar6 != 0x0) goto LAB_1030_5fb7;
        }
      }
    }
    PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}



fn pass1_1030_5fe2(param_1: u32,param_2: u32)
{
  (param_1 + 0x10) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_5ff6(param_1: u32,param_2: u16,uchar *param_3,uchar *param_4,param_5: u8)
{
  let puVar1: *mut u16;
  code **ppcVar2;
  let puVar3: u32;
  let uVar4: u32;
  let iVar5: i16;
  let uVar6: u16;
  let puVar7: *mut u8
  let extraout_DX: *mut u8
  let extraout_DX_00: *mut u8
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  uchar local_6c [0x58];
  let uStack20: u32;
  let uStack16: u32;
  let uStack12: u32;
  let uStack8: u16;
  let puStack6: *mut u8
  let uStack4: u16;
  
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  puVar7 = param_3;
  if (*(long *)(iVar8 + 0xc) == 0x0) {
    mem_op_1000_179c(0x18,param_3,0x1000);
    puVar7 = (uchar *)(param_3 | param_2);
    uStack8 = param_2;
    puStack6 = param_3;
    if (puVar7 == (uchar *)0x0) {
      (iVar8 + 0xc) = 0x0;
    }
    else {
      struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_3,param_2),0x5,0x5);
      (iVar8 + 0xc) = param_2;
      *(uchar **)(iVar8 + 0xe) = extraout_DX;
      puVar7 = extraout_DX;
    }
  }
  for (uStack4 = 0x0; uVar4 = (iVar8 + 0x10),
      puVar1 = (u16 *)(uVar4 + 0xa), uStack4 <= *puVar1 && *puVar1 != uStack4;
      uStack4 += 0x1) {
    uStack12 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,puVar7,0x2);
    iVar5 = uStack12;
    ppcVar2 = (code **)((iVar8 + 0xc) + 0x8);
    (**ppcVar2)(&USHORT_1050_1028,(iVar8 + 0xc),iVar5,
                (char)(uStack12 >> 0x10),uStack4,0x0);
    puVar7 = extraout_DX_00;
    pass1_1030_8344(_PTR_LOOP_1050_5748,
                    (_PTR_LOOP_1050_5748 >> 0x10),uStack12);
    uStack16 = CONCAT22(puVar7,iVar5);
    uStack20 = (iVar5 + 0x10);
    if (*(long *)(uStack20 + 0x2) == 0x0) {
      puVar3 = (iVar8 + 0x10);
      sys_1000_3f9c(local_6c,param_4,s__s__d_1050_573a,&USHORT_1050_1050,
                    *puVar3,&stack0xfffe,(puVar3 >> 0x10),0x1000,
                    param_4,param_5);
      uVar6 = str_op_1008_60e8(CONCAT22(param_4,local_6c),puVar7);
      uVar10 = (uStack20 >> 0x10);
      (uStack20 + 0x2) = uVar6;
      *(uchar **)(uStack20 + 0x4) = puVar7;
    }
  }
  return;
}



astruct_18 *  pass1_1030_6118(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_5d78((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_615a(astruct_137 *param_1,param_2: u16)
{
  let uVar1: u16;
  let extraout_DX: u16;
  astruct_137 *iVar2;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_137 *)param_1;
  uVar1 = 0x0;
  param_1 = 0x0;
  &iVar2->field_0x4 = 0x0;
  mem_op_1000_179c(0xc,(uchar *)param_2,0x1000);
  if ((param_2 | uVar1) == 0x0) {
    &iVar2->field_0x4 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(param_2,uVar1));
    iVar2->field_0x4 = uVar1;
    iVar2->field_0x6 = extraout_DX;
  }
  _PTR_LOOP_1050_5740 = param_1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_61b0(param_1: *mut u16)
{
  let uVar1: u16;
  let puVar2: u32;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar1 = (iVar4 + 0x2);
  if ((uVar1 | *param_1) != 0x0) {
    ppcVar3 = (code **)*param_1;
    (**ppcVar3)();
  }
  puVar2 = (iVar4 + 0x4);
  uVar1 = (iVar4 + 0x6);
  if ((uVar1 | puVar2) != 0x0) {
    ppcVar3 = (code **)*puVar2;
    (**ppcVar3)();
  }
  _PTR_LOOP_1050_5740 = 0x0;
  return;
}



void 
pass1_1030_61fe(param_1: u32,param_2: u32,param_3: u32,param_4: i32,param_5: u16,
               param_6: u16,param_7: u16)

{
  pass1_1030_677a(param_1,param_4,param_7);
  pass1_1030_8aa0(CONCAT22(param_6,param_5),param_2,(u16 *)param_3,param_6,param_7);
  return;
}



u16 
pass1_1030_6222(param_1: u32,param_2: i16,param_3: u32,param_4: u32,param_5: u16,
               uchar *param_6,param_7: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  let extraout_DX: u16;
  let uStack6: u32;
  
  mem_op_1000_179c(0x4c,param_6,0x1000);
  uVar2 = param_6 | param_5;
  if (uVar2 == 0x0) {
    param_5 = 0x0;
    uVar2 = 0x0;
  }
  else {
    pass1_1030_88ce((u16 *)CONCAT22(param_6,param_5),param_3,param_4,param_7);
  }
  uStack6 = CONCAT22(uVar2,param_5);
  ppcVar1 = (code **)((param_1 + 0x4) + 0x4);
  (**ppcVar1)();
  if (param_2 != 0x0) {
    pass1_1030_8d08(uStack6,extraout_DX);
  }
  return 0x0;
}



void 
pass1_1030_627e(param_1: u16,param_2: u16,param_3: u16,param_4: u32,param_5: *mut u16,
               param_6: i32)

{
  let local_12: [u32;0x2];
  let uStack10: u32;
  let uStack6: u32;
  
  uStack6 = 0x0;
  pass1_1030_677a(param_4,param_6,param_1);
  uStack10 = CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    pass1_1030_8b00(uStack10,param_5,(u16 *)CONCAT22(param_1,local_12),param_1);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1030_62e4(param_1: *mut u32,param_2: *mut u16,param_3: i32,param_4: u16)
{
  code **ppcVar1;
  let puVar2: u32;
  let uVar3: u16;
  let extraout_DX: *mut u8
  let puVar4: *mut u8
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let uVar5: u16;
  i16 local_64 [0x3];
  let uStack94: u32;
  let uStack88: u16;
  let uStack78: u16;
  let uStack76: u16;
  let local_40: u32;
  let uStack60: u32;
  let uStack56: u16;
  let puStack54: u32;
  let puStack52: *mut u8
  let puStack50: u32;
  let puStack48: *mut u8
  let uStack46: u16;
  let iStack44: i16;
  let local_2a: [u8;2];
  let local_28: i16;
  let local_26: i16;
  let local_24: u16;
  let local_22: [u8;2];
  let local_20: [u8;2];
  let local_1e: u16;
  let local_1c: u16;
  let local_1a: u16;
  let local_18: [u8;6];
  let local_12: [u8;6];
  let local_c: [u8;6];
  let uStack6: u32;
  
  uVar5 = (param_1 >> 0x10);
  puVar2 = *(u32 **)param_1;
  puVar4 = *(uchar **)(param_1 + 0x2);
  puStack54 = puVar2;
  puStack52 = puVar4;
  puStack50 = puVar2;
  puStack48 = puVar4;
  if ((puVar4 | puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    (**ppcVar1)();
    puVar4 = extraout_DX;
  }
  mem_op_1000_179c(0x18,puVar4,0x1000);
  puStack54 = puVar2;
  puStack52 = puVar4;
  if ((puVar4 | puVar2) == 0x0) {
    puVar2 = 0x0;
    uVar3 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(puVar4,puVar2),0x5,0x5);
    uVar3 = extraout_DX_00;
  }
  *(u32 **)param_1 = puVar2;
  (param_1 + 0x2) = uVar3;
  pass1_1030_677a(param_1,param_3,param_4);
  uStack6 = CONCAT22(uVar3,puVar2);
  if ((uVar3 | puVar2) != 0x0) {
    pass1_1008_3e38((u16 *)CONCAT22(param_4,local_c));
    pass1_1008_3e38((u16 *)CONCAT22(param_4,local_12));
    pass1_1008_3e38((u16 *)CONCAT22(param_4,local_18));
    pass1_1008_6d3e(param_2,(u16 *)CONCAT22(param_4,local_12),
                    (u16 *)CONCAT22(param_4,local_c));
    pass1_1008_3eb4((u16 *)CONCAT22(param_4,local_c),
                    (u16 *)CONCAT22(param_4,&local_1e),
                    (u16 *)CONCAT22(param_4,&local_1c),
                    (u16 *)CONCAT22(param_4,&local_1a));
    pass1_1008_3eb4((u16 *)CONCAT22(param_4,local_12),
                    (u16 *)CONCAT22(param_4,&local_24),
                    (u16 *)CONCAT22(param_4,local_22),
                    (u16 *)CONCAT22(param_4,local_20));
    pass1_1008_6d64(param_2,(u16 *)CONCAT22(param_4,local_18));
    pass1_1008_3eb4((u16 *)CONCAT22(param_4,local_18),
                    (u16 *)CONCAT22(param_4,local_2a),
                    (u16 *)CONCAT22(param_4,&local_28),
                    (u16 *)CONCAT22(param_4,&local_26));
    if (local_24 == local_1e) {
      iStack44 = 0x0;
      for (uStack46 = local_1c; uVar3 = local_28 + local_1c, uStack46 < uVar3;
          uStack46 += 0x1) {
        for (uStack56 = local_1a; uStack56 < (local_26 + local_1a);
            uStack56 += 0x1) {
          uStack88 = local_1e;
          pass1_1008_3e54((u16 *)
                          CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,local_64)
                                  ),local_1e,uStack46,uStack56);
          pass1_1030_8b00(uStack6,(u16 *)CONCAT22(param_4,local_64),
                          (u16 *)CONCAT22(param_4,&local_40),param_4);
          uStack60 = local_40;
          local_64[0] = iStack44;
          uStack60._0_2_ = local_40;
          uStack78 = uStack60;
          uStack76 = local_40._2_2_;
          uStack76._1_1_ = (char)(local_40 >> 0x18);
          if (uStack76._1_1_ == '\0') {
            uStack60._0_2_ = 0x0;
            local_40._2_2_ = 0x0;
          }
          uStack94 = CONCAT22(local_40._2_2_,uStack60);
          ppcVar1 = (code **)(*param_1 + 0x8);
          iStack44 = iStack44 + 0x1;
          (**ppcVar1)();
        }
      }
      ppcVar1 = (code **)(*param_1 + 0x10);
      (**ppcVar1)(0x1008,*param_1);
      if ((extraout_DX_01 | uVar3) != 0x0) {
        return;
      }
    }
  }
  return;
}



fn pass1_1030_64ce(param_1: u16,param_2: u16,param_3: u16,param_4: u32,param_5: *mut u16, param_6: i32,param_7: *mut u32)
{
  let puVar1: u32;
  let uVar2: u16;
  let local_e: u32;
  let uStack10: u32;
  let uStack6: u32;
  
  uStack6 = 0x0;
  pass1_1030_677a(param_4,param_6,param_1);
  uStack10 = CONCAT22(param_3,param_2);
  uVar2 = param_3 | param_2;
  if (uVar2 != 0x0) {
    puVar1 = &local_e;
    pass1_1030_8b00(uStack10,param_5,(u16 *)CONCAT22(param_1,puVar1),param_1);
    uStack6 = *puVar1;
  }
  *param_7 = uStack6;
  return;
}



fn pass1_1030_6522(param_1: *mut u32,param_2: u32,param_3: u32,param_4: u16)
{
  code **ppcVar1;
  let puVar2: u32;
  let uVar3: u16;
  let extraout_DX: *mut u8
  let puVar4: *mut u8
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let uVar5: u16;
  u8 local_64 [0xc];
  let uStack88: u16;
  let local_40: u32;
  let uStack60: u32;
  let uStack56: u16;
  let puStack54: u32;
  let puStack52: *mut u8
  let puStack50: u32;
  let puStack48: *mut u8
  let uStack46: u16;
  let iStack44: i16;
  let local_2a: [u8;2];
  let local_28: i16;
  let local_26: i16;
  let local_24: u16;
  let local_22: [u8;2];
  let local_20: [u8;2];
  let local_1e: u16;
  let local_1c: u16;
  let local_1a: u16;
  let local_18: [u8;6];
  let local_12: [u8;6];
  let local_c: [u8;6];
  let uStack6: u32;
  
  uVar5 = (param_1 >> 0x10);
  puVar2 = *(u32 **)param_1;
  puVar4 = *(uchar **)(param_1 + 0x2);
  puStack54 = puVar2;
  puStack52 = puVar4;
  puStack50 = puVar2;
  puStack48 = puVar4;
  if ((puVar4 | puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    (**ppcVar1)();
    puVar4 = extraout_DX;
  }
  mem_op_1000_179c(0x18,puVar4,0x1000);
  puStack54 = puVar2;
  puStack52 = puVar4;
  if ((puVar4 | puVar2) == 0x0) {
    puVar2 = 0x0;
    uVar3 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(puVar4,puVar2),0x5,0x5);
    uVar3 = extraout_DX_00;
  }
  *(u32 **)param_1 = puVar2;
  (param_1 + 0x2) = uVar3;
  pass1_1030_677a(param_1,param_3,param_4);
  uStack6 = CONCAT22(uVar3,puVar2);
  if ((uVar3 | puVar2) != 0x0) {
    pass1_1008_3e38((u16 *)CONCAT22(param_4,local_c));
    pass1_1008_3e38((u16 *)CONCAT22(param_4,local_12));
    pass1_1008_3e38((u16 *)CONCAT22(param_4,local_18));
    pass1_1008_6d3e((u16 *)param_2,(u16 *)CONCAT22(param_4,local_12),
                    (u16 *)CONCAT22(param_4,local_c));
    pass1_1008_3eb4((u16 *)CONCAT22(param_4,local_c),
                    (u16 *)CONCAT22(param_4,&local_1e),
                    (u16 *)CONCAT22(param_4,&local_1c),
                    (u16 *)CONCAT22(param_4,&local_1a));
    pass1_1008_3eb4((u16 *)CONCAT22(param_4,local_12),
                    (u16 *)CONCAT22(param_4,&local_24),
                    (u16 *)CONCAT22(param_4,local_22),
                    (u16 *)CONCAT22(param_4,local_20));
    pass1_1008_6d64((u16 *)param_2,(u16 *)CONCAT22(param_4,local_18));
    pass1_1008_3eb4((u16 *)CONCAT22(param_4,local_18),
                    (u16 *)CONCAT22(param_4,local_2a),
                    (u16 *)CONCAT22(param_4,&local_28),
                    (u16 *)CONCAT22(param_4,&local_26));
    if (local_24 == local_1e) {
      iStack44 = 0x0;
      for (uStack46 = local_1c; uVar3 = local_28 + local_1c, uStack46 < uVar3;
          uStack46 += 0x1) {
        for (uStack56 = local_1a; uStack56 < (local_26 + local_1a);
            uStack56 += 0x1) {
          uStack88 = local_1e;
          pass1_1008_3e54((u16 *)
                          CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,local_64)
                                  ),local_1e,uStack46,uStack56);
          pass1_1030_8b00(uStack6,(u16 *)CONCAT22(param_4,local_64),
                          (u16 *)CONCAT22(param_4,&local_40),param_4);
          uStack60 = local_40;
          iStack44 += 0x1;
          ppcVar1 = (code **)(*param_1 + 0x8);
          (**ppcVar1)();
        }
      }
      ppcVar1 = (code **)(*param_1 + 0x10);
      (**ppcVar1)(0x1008,*param_1);
      if ((extraout_DX_01 | uVar3) != 0x0) {
        return;
      }
    }
  }
  return;
}



fn pass1_1030_66de(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u32;
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(param_3,local_a),(param_1 + 0x4));
  while( true ) {
    uVar1 = pass1_1008_5b12(local_a,param_3);
    if (uVar1 == 0x0) break;
    pass1_1030_8bac(uVar1,param_2);
  }
  return;
}



fn pass1_1030_671c(param_1: u32,param_2: u32,param_3: *mut u16,param_4: i32,param_5: u16, param_6: u16,param_7: i16,param_8: u16)

{
  pass1_1030_677a(param_1,param_4,param_8);
  pass1_1030_8bdc(CONCAT22(param_6,param_5),param_2,param_3,param_7,param_8);
  return;
}



fn pass1_1030_6740(param_1: u32,param_2: u16,param_3: i16)
{
  let uVar1: u32;
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(param_2,local_a),(param_1 + 0x4));
  while( true ) {
    uVar1 = pass1_1008_5b12(local_a,param_2);
    if (uVar1 == 0x0) break;
    pass1_1030_8c38(uVar1,param_3,param_2);
  }
  return;
}



fn pass1_1030_677a(param_1: u32,param_2: i32,param_3: u16)
{
  let puVar1: *mut u8;
  let extraout_DX: u16;
  let uVar2: u16;
  let local_a: [u8;8];
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x4) == 0x0) {
    return;
  }
  pass1_1008_5784(CONCAT22(param_3,local_a),(param_1 + 0x4));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(puVar1,param_3);
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
  } while (*(long *)(puVar1 + 0x24) != param_2);
  return;
}



fn pass1_1030_67cc(param_1: *mut u16)
{
  astruct_687 *iVar1;
  let uVar1: u16;
  
  struct_1030_1628(param_1);
  iVar1 = (astruct_687 *)param_1;
  iVar1 = (astruct_687 *)&iVar1->field_0xc;
  pass1_1008_3e38((u16 *)(param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (param_1 >> 0x10);
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  iVar1->field_0x1a = 0x0;
  iVar1->field_0x1e = 0x0;
  iVar1->field_0x22 = 0x0;
  iVar1->field_0x26 = 0x0;
  iVar1->field_0x2a = 0x0;
  iVar1->field_0x2e = 0x0;
  iVar1->field_0x32 = 0x0;
  iVar1->field_0x34 = 0x0;
  iVar1->field_0x38 = 0x0;
  iVar1->field_0x36 = 0x0;
  iVar1->field_0x3c = 0x0;
  iVar1->field_0x3a = 0x0;
  iVar1->field_0x40 = 0x0;
  iVar1->field_0x3e = 0x0;
  *param_1 = 0x8114;
  iVar1->field_0x2 = 0x1030;
  return;
}



fn pass1_1030_684c(param_1: *mut u16,param_2: *mut u32,param_3: u16,param_4: u16,
               param_5: u16,param_6: u32,param_7: u16)

{
  let iVar1: i16;
  let uVar2: u16;
  
  pass1_1030_165e(param_1,0x5000000,param_6,param_7);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0xc) = *param_2;
  (iVar1 + 0x10) = (param_2 + 0x1);
  (iVar1 + 0x12) = param_4;
  (iVar1 + 0x14) = param_4;
  (iVar1 + 0x16) = 0x0;
  (iVar1 + 0x1a) = 0x0;
  (iVar1 + 0x1e) = 0x0;
  (iVar1 + 0x22) = 0x0;
  (iVar1 + 0x26) = 0x0;
  (iVar1 + 0x2a) = 0x0;
  (iVar1 + 0x2e) = 0x0;
  (iVar1 + 0x32) = 0x0;
  (iVar1 + 0x34) = 0x0;
  (iVar1 + 0x36) = 0x0;
  (iVar1 + 0x3a) = 0x0;
  (iVar1 + 0x3e) = 0x0;
  *param_1 = 0x8114;
  (iVar1 + 0x2) = 0x1030;
  return;
}



fn pass1_1030_68dc(param_1: *mut u16,param_2: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let puVar3: u32;
  astruct_18 *paVar4;
  code **ppcVar5;
  astruct_611 *iVar6;
  let uVar6: u16;
  astruct_18 *paStack10;
  
  uVar6 = (param_1 >> 0x10);
  iVar6 = (astruct_611 *)param_1;
  *param_1 = 0x8114;
  iVar6->field_0x2 = 0x1030;
  paVar4 = *(astruct_18 **)&iVar6->field_0x22;
  uVar1 = iVar6->field_0x24;
  if ((uVar1 | paVar4) != 0x0) {
    fn_ptr_1020_ba7e((paVar4 & 0xffff | uVar1 << 0x10));
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar4,0x1000);
  }
  uVar1 = iVar6->field_0x26;
  uVar2 = iVar6->field_0x28;
  paStack10 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e(CONCAT22(uVar2,uVar1));
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paStack10,0x1000);
  }
  puVar3 = iVar6->field_0x1e;
  uVar1 = iVar6->field_0x20;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar6->field_0x36;
  uVar1 = iVar6->field_0x38;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar6->field_0x3a;
  uVar1 = iVar6->field_0x3c;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(param_2,puVar3,uVar1,0x1);
  }
  puVar3 = iVar6->field_0x3e;
  uVar1 = iVar6->field_0x40;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(param_2,puVar3,uVar1,0x1);
  }
  pass1_1030_16b2(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_69cc(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let uVar1: u16;
  let BVar2: bool;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (*(long *)(iVar3 + 0x3e) != 0x0) {
    return;
  }
  if ((*(long *)(iVar3 + 0x22) != 0x0) &&
     (pass1_1020_ba94(*(long **)(iVar3 + 0x22)), (param_3 | param_2) != 0x0)) {
    return;
  }
  uVar1 = pass1_1030_6fa0(param_1);
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x4);
  if ((BVar2 != 0x0) &&
     (uVar5 = pass1_1028_67d4((iVar3 + 0x1a),param_4),
     ((uVar5 >> 0x10) | uVar5) != 0x0)) {
    return;
  }
  return;
}



fn pass1_1030_6a2c(param_1: u32,param_2: i32,param_3: u16,uchar *param_4,param_5: u16)
{
  code **ppcVar1;
  astruct_384 *iVar2;
  let uVar2: u16;
  astruct_382 *iVar4;
  astruct_383 *iVar5;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u32;
  let lVar6: i32;
  let local_a: [u8;8];
  
  uVar3 = (param_1 >> 0x10);
  iVar4 = (astruct_382 *)param_1;
  if (iVar4->field_0x3e == 0x0) {
    mem_op_1000_179c(0xc,param_4,0x1000);
    if ((param_4 | param_3) == 0x0) {
      iVar4->field_0x3e = 0x0;
    }
    else {
      uVar5 = set_struct_1008_574a((astruct_21 *)CONCAT22(param_4,param_3));
      &iVar4->field_0x3e = uVar5;
      (&iVar4->field_0x3e + 0x2) = (uVar5 >> 0x10);
    }
  }
  pass1_1008_5784(CONCAT22(param_5,local_a),iVar4->field_0x3e);
  do {
    do {
      lVar6 = pass1_1008_5b12(local_a,param_5);
      uVar2 = (lVar6 >> 0x10);
      iVar2 = (astruct_384 *)lVar6;
      if (lVar6 == 0x0) goto LAB_1030_6af4;
      uVar4 = (param_2 >> 0x10);
      iVar5 = (astruct_383 *)param_2;
    } while ((iVar2->field_0x6 != iVar5->field_0x6) ||
            (iVar2->field_0x4 != iVar5->field_0x4));
  } while (iVar2->field_0x8 != iVar5->field_0x8);
  iVar2->field_0xa = iVar2->field_0xa + iVar5->field_0xa;
  iVar2->field_0xc = iVar2->field_0xc + iVar5->field_0xc;
  param_2 = 0x0;
LAB_1030_6af4:
  if (param_2 != 0x0) {
    ppcVar1 = (code **)(*iVar4->field_0x3e + 0x8);
    (**ppcVar1)(0x1008,iVar4->field_0x3e,param_2);
  }
  return;
}



fn pass1_1030_6b16(param_1: u32) -> u32

{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u32;
  astruct_412 *iVar5;
  let uVar5: u16;
  let uVar6: u32;
  
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_412 *)param_1;
  if (*(long *)&iVar5->field_0x3a == 0x0) {
    return 0x0;
  }
  ppcVar3 = (code **)(&iVar5->field_0x3a + 0x10);
  uVar6 = (**ppcVar3)();
  uVar4 = &iVar5->field_0x3a;
  if ((uVar4 + 0x8) == 0x0) {
    puVar1 = &iVar5->field_0x3a;
    uVar2 = iVar5->field_0x3c;
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    &iVar5->field_0x3a = 0x0;
  }
  return uVar6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_6b86(param_1: u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let extraout_DX: u16;
  let uVar3: u16;
  let extraout_DX_00: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uStack12: u32;
  let uStack8: u32;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (*(long *)(iVar4 + 0x1e) == 0x0) {
    param_2 = 0x0;
    uVar3 = 0x0;
  }
  else {
    ppcVar1 = (code **)((iVar4 + 0x1e) + 0x10);
    (**ppcVar1)();
    uVar3 = extraout_DX;
  }
  uStack8 = CONCAT22(uVar3,param_2);
  for (uStack12 = 0x0; uStack12 < uStack8; uStack12 += 0x1) {
    ppcVar1 = (code **)((iVar4 + 0x1e) + 0x4);
    uVar2 = uStack8;
    (**ppcVar1)(param_3,(iVar4 + 0x1e));
    if ((extraout_DX_00 | uVar2) != 0x0) {
      param_3 = SUB42(&USHORT_1050_1028,0x0);
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00);
    }
  }
  return;
}



fn pass1_1030_6c1a(param_1: u32,param_2: i16)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  iVar2 = (iVar3 + 0x32);
  (iVar3 + 0x32) = param_2;
  piVar1 = (i16 *)(iVar3 + 0x34);
  *piVar1 = *piVar1 + (param_2 - iVar2);
  iVar2 = (iVar3 + 0x32);
  if (iVar2 < 0x0) {
    iVar2 = 0x0;
  }
  (iVar3 + 0x32) = iVar2;
  return;
}



fn pass1_1030_6c4c(param_1: u32,param_2: i16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = (param_1 + 0x32);
  if (param_2 < iVar1) {
    iVar1 = param_2;
  }
  (param_1 + 0x34) = iVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_6c66(param_1: u32,param_2: i16,param_3: u32,param_4: u16,uchar *param_5,
               param_6: u16)

{
  let uVar1: u16;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u16;
  let BVar5: bool;
  let extraout_DX: u16;
  let puVar6: *mut u8
  astruct_386 *iVar7;
  astruct_385 *iVar6;
  let unaff_SI: u16;
  let unaff_DI: u16;
  let uVar7: u16;
  let uVar8: u16;
  let unaff_SS: u16;
  
  uVar7 = (param_1 >> 0x10);
  iVar7 = (astruct_386 *)param_1;
  if (iVar7->field_0x3a == 0x0) {
    param_6 = 0x1000;
    mem_op_1000_179c(0xc,param_5,0x1000);
    if ((param_5 | param_4) == 0x0) {
      iVar7->field_0x3a = 0x0;
    }
    else {
      param_6 = 0x1008;
      set_struct_1008_574a((astruct_21 *)CONCAT22(param_5,param_4));
      &iVar7->field_0x3a = param_4;
      (&iVar7->field_0x3a + 0x2) = extraout_DX;
    }
  }
  ppcVar2 = (code **)(*iVar7->field_0x3a + 0x8);
  (**ppcVar2)(param_6,iVar7->field_0x3a,param_3);
  if (param_2 != 0x0) {
    uVar8 = (param_3 >> 0x10);
    iVar6 = (astruct_385 *)param_3;
    if (iVar6->field_0x6 != 0x0) {
      pass1_1030_6e9c(param_1,iVar6->field_0xa,iVar6->field_0x6);
      return;
    }
    if (iVar6->field_0x4 != 0x0) {
      uVar1 = iVar6->field_0xa;
      uVar3 = -uVar1;
      puVar6 = (uchar *)-(uVar1 != 0x0);
      pass1_1030_7ddc(param_1,CONCAT13((char)(puVar6 >> 0x8),
                                       CONCAT12((char)puVar6,uVar3)),iVar6->field_0x4,
                      uVar3,puVar6,unaff_SI,unaff_DI,unaff_SS);
      return;
    }
    if (iVar6->field_0x8 != 0x0) {
      uVar4 = pass1_1030_6fa0(param_1);
      BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar4,0x4);
      if (BVar5 != 0x0) {
        pass1_1028_6356(iVar7->field_0x1a,0x0,iVar6->field_0xa,0x0,unaff_SS);
      }
    }
  }
  return;
}



fn pass1_1030_6d4e(param_1: u32,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u16;
  let uStack6: u16;
  let uStack4: u16;
  
  uStack6 = 0x0;
  uStack4 = 0x0;
  uVar1 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x36) != 0x0) {
    pass1_1010_9092((param_1 + 0x36),param_2,param_4);
    uStack6 = param_2;
    uStack4 = param_3;
  }
  return CONCAT22(uStack4,uStack6);
}



fn pass1_1030_6d80(param_1: u32,param_2: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_299 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_299 *)param_1;
  puVar1 = *(u32 **)&iVar4->field_0x36;
  uVar2 = (&iVar4->field_0x36 + 0x2);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  iVar4->field_0x36 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_6db4(uchar *param_1,param_2: i16,param_3: u16) -> u16

{
  let puVar1: *mut u16;
  
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,param_1,param_2);
  pass1_1010_ed3e(puVar1);
  return (puVar1 + 0x18);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_6ddc(param_1: u32)
{
  let uVar1: u16;
  let BVar2: bool;
  
  uVar1 = pass1_1030_6fa0(param_1);
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1e);
  if (BVar2 != 0x0) {
    pass1_1030_d0c6((param_1 + 0x1a));
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_6e14(param_1: u32)
{
  let uVar1: u32;
  let uVar2: u16;
  let BVar3: bool;
  
  uVar2 = pass1_1030_6fa0(param_1);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x1e);
  if (BVar3 != 0x0) {
    uVar1 = (param_1 + 0x1a);
    pass1_1030_d102(uVar1,(uVar1 >> 0x10));
    return;
  }
  return;
}



fn pass1_1030_6e4c(param_1: u32)
{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(long *)(iVar2 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar3 << 0x10);
  }
  if ((*(long *)(iVar2 + 0x1a) != 0x0) &&
     (uVar1 = (iVar2 + 0x1a), (uVar1 + 0x12) == 0x4)) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_6e9c(param_1: u32,param_2: i32,param_3: i16)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar5: u16;
  astruct_301 *iVar6;
  let uVar6: u16;
  let unaff_SS: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar6 = (param_1 >> 0x10);
  iVar6 = (astruct_301 *)param_1;
  uVar2 = (&iVar6->field_0x1e + 0x2) | &iVar6->field_0x1e;
  if (uVar2 != 0x0) {
    ppcVar1 = (code **)(*iVar6->field_0x1e + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX,uVar2);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
      ppcVar1 = (code **)(*iVar6->field_0x1e + 0x4);
      uVar4 = uStack6;
      (**ppcVar1)();
      uVar2 = uVar4;
      uVar5 = extraout_DX_00 | uVar2;
      if (uVar5 != 0x0) {
        uVar3 = uVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00);
        if ((uVar3 + 0xc) == param_3) {
          param_2 += -0x1;
          pass1_1028_e332(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00,unaff_SS);
          ppcVar1 = (code **)(*iVar6->field_0x1e + 0x8);
          (**ppcVar1)(&USHORT_1050_1028,iVar6->field_0x1e,0x0,uStack10);
        }
        if ((param_2._2_2_ | param_2) == 0x0) {
          return;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_6f5a(param_1: u32,param_2: u16)
{
  let uVar1: u16;
  let BVar2: bool;
  
  uVar1 = pass1_1030_6fa0(param_1);
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x4);
  if (BVar2 != 0x0) {
    pass1_1028_6302((param_1 + 0x1a),param_2);
  }
  return;
}



fn pass1_1030_6fa0(param_1: u32) -> u16

{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(long *)(iVar2 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar3 << 0x10);
  }
  if (*(long *)(iVar2 + 0x1a) != 0x0) {
    uVar1 = (iVar2 + 0x1a);
    return (uVar1 + 0xc);
  }
  return 0x0;
}



fn pass1_1030_6fd4(param_1: u32)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar2 << 0x10);
  }
  uVar1 = (param_1 + 0x1a);
  if ((uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



fn pass1_1030_701c(param_1: u32)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar2 << 0x10);
  }
  uVar1 = (param_1 + 0x1a);
  if ((uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



fn pass1_1030_7064(param_1: u32)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar2 << 0x10);
  }
  uVar1 = (param_1 + 0x1a);
  if ((uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



fn pass1_1030_70ac(param_1: u32)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar2 << 0x10);
  }
  uVar1 = (param_1 + 0x1a);
  if ((uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_70f4(param_1: u32)
{
  let uVar1: u16;
  let uVar2: u32;
  let BVar3: bool;
  let iVar4: i16;
  let uVar5: u16;
  long *plVar6;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (*(long *)(iVar4 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar5 << 0x10);
  }
  uVar2 = (iVar4 + 0x1a);
  uVar1 = (uVar2 + 0xc);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1);
  if (BVar3 == 0x0) {
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x2);
    if ((BVar3 == 0x0) || (*(long *)(iVar4 + 0x22) == 0x0)) {
      return;
    }
    plVar6 = *(long **)(iVar4 + 0x22);
  }
  else {
    uVar2 = (iVar4 + 0x1a);
    plVar6 = *(long **)(uVar2 + 0x28);
  }
  pass1_1020_ba94(plVar6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_7176(param_1: u32,param_2: u16)
{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  let local_1a: i32;
  i16 local_16 [0x2];
  let uStack18: u16;
  let uStack14: u16;
  let BStack10: bool;
  let uStack8: u16;
  let lStack6: i32;
  
  lStack6 = 0x0;
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(long *)(iVar2 + 0x22) == 0x0) {
    return;
  }
  if (*(long *)(iVar2 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1);
  }
  uVar1 = (iVar2 + 0x1a);
  uStack8 = (uVar1 + 0xc);
  BStack10 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x3);
  if ((BStack10 != 0x0) &&
     (uVar1 = (iVar2 + 0x1a), (uVar1 + 0x12) == 0x5)) {
    uVar1 = (iVar2 + 0x22);
    uStack14 = (uVar1 + 0x4);
    for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
      pass1_1020_bb16(*(u32 **)(iVar2 + 0x22),CONCAT22(param_2,&local_1a),
                      (u16 *)CONCAT22(param_2,local_16),uStack18);
      if (0x0 < local_16[0]) {
        lStack6 += local_1a;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_7226(param_1: u32)
{
  let uVar1: u32;
  let uVar2: u32;
  let BVar3: bool;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (*(long *)(iVar4 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar5 << 0x10);
  }
  uVar2 = (iVar4 + 0x1a);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar2 + 0xc),0x10);
  if (((BVar3 != 0x0) &&
      (uVar2 = (iVar4 + 0x1a), (uVar2 + 0x12) == 0x5)) &&
     (uVar1 = (iVar4 + 0x1a),
     uVar2 = (uVar1 & 0xffff0000 | (uVar1 + 0x14)),
     (uVar2 + 0xa4) == 0x1e)) {
    return;
  }
  return;
}



fn fn_ptr_1030_7296(param_1: u32)
{
  let uVar1: u16;
  let uVar2: u16;
  astruct_292 *iVar3;
  let uVar3: u16;
  astruct_18 *paStack6;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_292 *)param_1;
  uVar1 = iVar3->field_0x22;
  uVar2 = iVar3->field_0x24;
  paStack6 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack6,0x1000);
  }
  &iVar3->field_0x22 = 0x0;
  return;
}



fn pass1_1030_72d0(param_1: u32)
{
  let uVar1: u16;
  let uVar2: u16;
  astruct_605 *iVar3;
  let uVar3: u16;
  astruct_18 *paStack6;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_605 *)param_1;
  uVar1 = iVar3->field_0x26;
  uVar2 = iVar3->field_0x28;
  paStack6 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack6,0x1000);
  }
  &iVar3->field_0x26 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_730a(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  astruct_290 *iVar5;
  let uVar5: u16;
  let puVar6: u32;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_290 *)param_1;
  if (iVar5->field_0x1e != 0x0) {
    puVar6 = iVar5->field_0x1e;
    ppcVar3 = (code **)(*iVar5->field_0x1e + 0x10);
    (**ppcVar3)();
    uStack6 = CONCAT22(extraout_DX,param_2);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
      ppcVar3 = (code **)(*iVar5->field_0x1e + 0x4);
      uVar4 = uStack6;
      (**ppcVar3)(param_3);
      if ((extraout_DX_00 | uVar4) != 0x0) {
        param_3 = &USHORT_1050_1028;
        pass1_1028_e332(_PTR_LOOP_1050_65e2,uVar4,extraout_DX_00,param_4);
      }
    }
                    // WARNING: Load size is inaccurate
    puVar1 = iVar5->field_0x1e;
    uVar2 = (&iVar5->field_0x1e + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)(param_3,puVar1,uVar2,0x1,puVar6);
    }
    iVar5->field_0x1e = 0x0;
  }
  return;
}


fn pass1_1030_73ee(param_1: u32,param_2: u32,param_3: u16)
{
  astruct_294 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_294 *)param_1;
  iVar1->field_0x2a = param_2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_2,(param_2 >> 0x10));
  iVar1->field_0x2e = param_2;
  iVar1->field_0x30 = param_3;
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1030_7418(param_1: u32,param_2: u32,param_3: i16,param_4: u16)
{
  let uVar1: u32;
  astruct_731 *iVar2;
  let iVar3: i16;
  let Bvar4: bool;
  let puVar5: *mut u8;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uStack62: u16;
  let local_2a: [u16;0x2];
  u8 local_26 [0xe];
  let local_18: u32;
  let local_14: [u32;0x2];
  let local_c: u16;
  let local_a: u32;
  let local_6: [u16;0x2];
  
  pass1_1030_16d6(param_1,param_2,param_4);
  if (param_3 == 0x0) {
    return;
  }
  iVar2 = (astruct_731 *)param_1;
  iVar2 = (astruct_731 *)&iVar2->field_0xc;
  iVar3 = write_to_file_1008_7b4c
                    (param_2,param_1 & 0xffff0000 | ZEXT24(iVar2),0x1008,param_4);
  if (iVar3 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  uVar6 = (param_1 >> 0x10);
  local_c = iVar2->field_0x12;
  uVar7 = param_2;
  uVar8 = (param_2 >> 0x10);
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,&local_c,param_4,0x2,0x1008)
  ;
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  local_6[0] = iVar2->field_0x14;
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,local_6,param_4,0x2,0x1008);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  local_18 = iVar2->field_0x16;
  BVar4 = write_to_file_1008_7e1c
                    (uVar7,uVar8,&local_18,param_4,0x4,0x1008);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  write_to_file_1008_7954(param_2,iVar2->field_0x1e,BVar4,0x1008,param_4);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  write_to_file_1008_7a22(param_2,iVar2->field_0x22,0x1008,param_4);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  write_to_file_1008_7a22(param_2,iVar2->field_0x26,0x1008,param_4);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  local_a = iVar2->field_0x2a;
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,&local_a,param_4,0x4,0x1008)
  ;
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  local_c = iVar2->field_0x32;
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,&local_c,param_4,0x2,0x1008)
  ;
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  local_c = iVar2->field_0x34;
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,&local_c,param_4,0x2,0x1008)
  ;
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  pass1_1008_79f0(param_2,iVar2->field_0x36,0x1008,param_4);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  if (iVar2->field_0x3a == 0x0) {
    local_18 &= 0xffff0000;
  }
  else {
    uVar1 = iVar2->field_0x3a;
    local_18 = local_18 & 0xffff0000 | (uVar1 + 0x8);
  }
  local_6[0] = local_18;
  BVar4 = write_to_file_1008_7e1c(uVar7,uVar8,local_6,param_4,0x2,0x1008);
  if (BVar4 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d0;
    return;
  }
  pass1_1008_5784(CONCAT22(param_4,local_26),iVar2->field_0x3a);
  while( true ) {
    puVar5 = local_26;
    pass1_1008_5b12(puVar5,param_4);
    local_14[0] = CONCAT22(extraout_DX,puVar5);
    if ((extraout_DX | puVar5) == 0x0) {
      if (iVar2->field_0x3e == 0x0) {
        uStack62 = 0x0;
      }
      else {
        uVar1 = iVar2->field_0x3e;
        uStack62 = (uVar1 + 0x8);
      }
      local_2a[0] = uStack62;
      BVar4 = write_to_file_1008_7e1c
                        (uVar7,uVar8,local_2a,param_4,0x2,0x1008);
      if (BVar4 == 0x0) {
        PTR_LOOP_1050_0310 = 0x6d0;
        return;
      }
      pass1_1008_5784(CONCAT22(param_4,local_26),iVar2->field_0x3e);
      while( true ) {
        puVar5 = local_26;
        pass1_1008_5b12(puVar5,param_4);
        if ((extraout_DX_00 | puVar5) == 0x0) {
          return;
        }
        local_18 = local_18 & 0xffff0000 | (puVar5 + 0x4);
        BVar4 = write_to_file_1008_7e1c
                          (uVar7,uVar8,&local_18,param_4,0x2,0x1008);
        if (BVar4 == 0x0) {
          PTR_LOOP_1050_0310 = 0x6d0;
          return;
        }
        local_14[0] = local_14[0] & 0xffff0000 | (puVar5 + 0x6);
        BVar4 = write_to_file_1008_7e1c
                          (uVar7,uVar8,local_14,param_4,0x2,0x1008);
        if (BVar4 == 0x0) {
          PTR_LOOP_1050_0310 = 0x6d0;
          return;
        }
        local_c = (puVar5 + 0x8);
        BVar4 = write_to_file_1008_7e1c
                          (uVar7,uVar8,&local_c,param_4,0x2,0x1008);
        if (BVar4 == 0x0) break;
        local_c = (puVar5 + 0xa);
        BVar4 = write_to_file_1008_7e1c
                          (uVar7,uVar8,&local_c,param_4,0x2,0x1008);
        if (BVar4 == 0x0) {
          PTR_LOOP_1050_0310 = 0x6d0;
          return;
        }
        local_6[0] = (puVar5 + 0xc);
        BVar4 = write_to_file_1008_7e1c
                          (uVar7,uVar8,local_6,param_4,0x2,0x1008);
        if (BVar4 == 0x0) {
          PTR_LOOP_1050_0310 = 0x6d0;
          return;
        }
      }
      PTR_LOOP_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = (puVar5 + 0x4);
    BVar4 = write_to_file_1008_7e1c
                      (uVar7,uVar8,local_6,param_4,0x2,0x1008);
    if (BVar4 == 0x0) {
      PTR_LOOP_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = (local_14[0] + 0x6);
    BVar4 = write_to_file_1008_7e1c
                      (uVar7,uVar8,local_6,param_4,0x2,0x1008);
    if (BVar4 == 0x0) break;
    local_6[0] = (local_14[0] + 0x8);
    BVar4 = write_to_file_1008_7e1c
                      (uVar7,uVar8,local_6,param_4,0x2,0x1008);
    if (BVar4 == 0x0) {
      PTR_LOOP_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = (local_14[0] + 0xa);
    BVar4 = write_to_file_1008_7e1c
                      (uVar7,uVar8,local_6,param_4,0x2,0x1008);
    if (BVar4 == 0x0) {
      PTR_LOOP_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = (local_14[0] + 0xc);
    BVar4 = write_to_file_1008_7e1c
                      (uVar7,uVar8,local_6,param_4,0x2,0x1008);
    if (BVar4 == 0x0) {
      PTR_LOOP_1050_0310 = 0x6d0;
      return;
    }
  }
  PTR_LOOP_1050_0310 = 0x6d0;
  return;
}


fn pass1_1030_7bee(param_1: u32) -> u16

{
  code **ppcVar1;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (*(long *)(iVar3 + 0x16) == 0x0) {
    return 0x0;
  }
  if (*(long *)(iVar3 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar4 << 0x10);
  }
  ppcVar1 = (code **)((iVar3 + 0x1a) + 0x44);
  uVar2 = (**ppcVar1)();
  return uVar2;
}



fn pass1_1030_7c28(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: u16) -> u32

{
  let uVar1: u16;
  let uVar2: u32;
  
  uVar1 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x22) == 0x0) {
    return 0x0;
  }
  uVar2 = (param_1 + 0x22);
  uVar2 = pass1_1020_bae6(uVar2,CONCAT22(param_2,(uVar2 >> 0x10)),
                          param_3,param_4,param_5);
  return uVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_7c50(param_1: u32,param_2: i32,param_3: i16,param_4: u16,uchar *param_5)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u32;
  let uVar5: u16;
  let puVar6: *mut u8
  let extraout_DX: *mut u8
  let extraout_DX_00: u16;
  let uVar7: u16;
  let extraout_DX_01: *mut u8
  astruct_305 *iVar8;
  let uVar8: u16;
  let uVar9: u32;
  let puVar10: u32;
  let puStack18: u32;
  
  uVar8 = (param_1 >> 0x10);
  iVar8 = (astruct_305 *)param_1;
  puVar6 = param_5;
  if (iVar8->field_0x1e == 0x0) {
    mem_op_1000_179c(0x18,param_5,0x1000);
    puVar6 = (uchar *)(param_5 | param_4);
    if (puVar6 == (uchar *)0x0) {
      iVar8->field_0x1e = 0x0;
    }
    else {
      struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_5,param_4),0x5,0x5);
      &iVar8->field_0x1e = param_4;
      *(uchar **)(&iVar8->field_0x1e + 0x2) = extraout_DX;
      puVar6 = extraout_DX;
    }
  }
  if (param_3 == 0x4) {
    piVar1 = &iVar8->field_0x34;
    *piVar1 = *piVar1 + param_2;
  }
  while (param_2 != 0x0) {
    uVar9 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,puVar6,0x6);
    uVar3 = uVar9;
    uVar4 = uVar9 >> 0x10;
    puVar10 = iVar8->field_0x1e;
    ppcVar2 = (code **)(*iVar8->field_0x1e + 0xc);
    uVar5 = uVar3;
    (**ppcVar2)();
    uVar7 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,uVar4);
    puStack18 = CONCAT22(uVar7,uVar5);
    ppcVar2 = (code **)(*puStack18 + 0x14);
    (**ppcVar2)(&USHORT_1050_1028,uVar5,uVar7,param_1,puVar10,uVar9);
    puVar6 = extraout_DX_01;
    param_2 = param_2 + -0x1;
  }
  return;
}



void 
pass1_1030_7d1c(param_1: u32,param_2: u16,param_3: u32,param_4: u16,uchar *param_5,
               param_6: u16,param_7: u16,param_8: u16)

{
  let uVar1: u16;
  astruct_397 *iVar2;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_397 *)param_1;
  if (iVar2->field_0x22 == (long *)0x0) {
    mem_op_1000_179c(0xa,param_5,0x1000);
    uVar1 = param_5 | param_4;
    if (uVar1 == 0x0) {
      iVar2->field_0x22 = (long *)0x0;
    }
    else {
      pass1_1020_ba3e((long *)CONCAT22(param_5,param_4),0xa,0x2,param_7,param_6);
      &iVar2->field_0x22 = param_4;
      (&iVar2->field_0x22 + 0x2) = uVar1;
    }
  }
  pass1_1020_bb8a(iVar2->field_0x22,param_2,param_3,param_7,param_8);
  return;
}



void 
pass1_1030_7d7c(param_1: u32,param_2: u16,param_3: u32,param_4: u16,uchar *param_5,
               param_6: u16,param_7: u16,param_8: u16)

{
  let uVar1: u16;
  astruct_398 *iVar2;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_398 *)param_1;
  if (iVar2->field_0x26 == (long *)0x0) {
    mem_op_1000_179c(0xa,param_5,0x1000);
    uVar1 = param_5 | param_4;
    if (uVar1 == 0x0) {
      iVar2->field_0x26 = (long *)0x0;
    }
    else {
      pass1_1020_ba3e((long *)CONCAT22(param_5,param_4),0xa,0x2,param_7,param_6);
      &iVar2->field_0x26 = param_4;
      (&iVar2->field_0x26 + 0x2) = uVar1;
    }
  }
  pass1_1020_bb8a(iVar2->field_0x26,param_2,param_3,param_7,param_8);
  return;
}



void 
pass1_1030_7ddc(param_1: u32,param_2: i32,param_3: u16,param_4: u16,uchar *param_5,
               param_6: u16,param_7: u16,param_8: u16)

{
  let uVar1: u32;
  let puVar2: *mut u8
  let iVar3: i16;
  let uVar4: u16;
  let lVar5: i32;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  puVar2 = param_5;
  if (*(long *)(iVar3 + 0x22) == 0x0) {
    mem_op_1000_179c(0xa,param_5,0x1000);
    puVar2 = (uchar *)(param_5 | param_4);
    if (puVar2 == (uchar *)0x0) {
      (iVar3 + 0x22) = 0x0;
    }
    else {
      pass1_1020_ba3e((long *)CONCAT22(param_5,param_4),0xa,0x2,param_7,param_6);
      (iVar3 + 0x22) = param_4;
      *(uchar **)(iVar3 + 0x24) = puVar2;
    }
  }
  uVar1 = (iVar3 + 0x22);
  lVar5 = pass1_1020_bae6(uVar1,CONCAT22(param_3,(uVar1 >> 0x10)),
                          param_4,puVar2,param_8);
  pass1_1020_bb8a(*(long **)(iVar3 + 0x22),(lVar5 + param_2),
                  CONCAT22(param_3,((lVar5 + param_2) >> 0x10)),param_7,
                  param_8);
  return;
}



fn pass1_1030_7e5a(param_1: u32,param_2: u32,param_3: u16)
{
  astruct_358 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_358 *)param_1;
  iVar1->field_0x16 = param_2;
  iVar1->field_0x1a = 0x0;
  pass1_1030_6fa0(param_1 & 0xffff | uVar1 << 0x10);
  if (iVar1->field_0x2e != 0x0) {
    pass1_1038_4b20(iVar1->field_0x2e,iVar1->field_0x16,iVar1->field_0x4,param_3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_7ea0(param_1: u32) -> bool

{
  let uVar1: u32;
  let uVar2: u16;
  let BVar3: bool;
  
  uVar2 = pass1_1030_6fa0(param_1);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0xb);
  if (BVar3 != 0x0) {
    uVar1 = (param_1 + 0x1a);
    if ((uVar1 + 0x12) == 0x5) {
      return 0x1;
    }
    BVar3 = 0x0;
  }
  return BVar3;
}



fn pass1_1030_7eda(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  let local_c: u16;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u16;
  let uStack4: u16;
  
  local_c = 0x0;
  uStack10 = 0x0;
  uStack6 = 0x0;
  uStack4 = 0x0;
  uStack8 = param_2;
  uVar1 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar1 << 0x10);
  }
  pass1_1028_bb96((param_1 + 0x1a),&local_c,param_3);
  return;
}



fn pass1_1030_7f1a(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  let local_c: u16;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u16;
  let uStack4: u16;
  
  local_c = 0x0;
  uStack8 = 0x0;
  uStack6 = 0x0;
  uStack4 = 0x0;
  uStack10 = param_2;
  uVar1 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar1 << 0x10);
  }
  pass1_1028_bb96((param_1 + 0x1a),&local_c,param_3);
  return;
}



fn pass1_1030_7f5a(param_1: u32) -> u16

{
  let uVar1: u16;
  let uVar2: u32;
  
  uVar1 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar1 << 0x10);
  }
  uVar2 = pass1_1028_bb6a((param_1 + 0x1a));
  if (uVar2 != 0x0) {
    return (uVar2 + 0x4);
  }
  return 0x0;
}



fn pass1_1030_7f98(param_1: u32) -> u16

{
  let uVar1: u16;
  let uVar2: u32;
  
  uVar1 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar1 << 0x10);
  }
  uVar2 = pass1_1028_bb6a((param_1 + 0x1a));
  if (uVar2 != 0x0) {
    return (uVar2 + 0x2);
  }
  return 0x0;
}



fn pass1_1030_7fd6(param_1: u32,param_2: u16,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (*(long *)(iVar3 + 0x1a) == 0x0) {
    uVar5 = struct_op_1030_73a8(param_1 & 0xffff | uVar4 << 0x10);
    param_2 = (uVar5 >> 0x10);
  }
  uVar2 = (iVar3 + 0x1a);
  iVar1 = (uVar2 + 0xc);
  if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
     ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 ||
      ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
    pass1_1028_1416((iVar3 + 0x1a),param_2,param_3);
  }
  return;
}



fn pass1_1030_8030(param_1: u32,param_2: u16,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (*(long *)(iVar3 + 0x1a) == 0x0) {
    uVar5 = struct_op_1030_73a8(param_1 & 0xffff | uVar4 << 0x10);
    param_2 = (uVar5 >> 0x10);
  }
  uVar2 = (iVar3 + 0x1a);
  iVar1 = (uVar2 + 0xc);
  if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
     ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 ||
      ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
    uVar5 = (iVar3 + 0x1a);
    pass1_1028_1106(uVar5,uVar5,param_2,param_3);
  }
  return;
}



fn pass1_1030_8086(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x18),
                  (param_1 + 0x16)) & 0xffffff;
}



fn pass1_1030_809c(param_1: u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8(param_1 & 0xffff | uVar1 << 0x10);
  }
  return;
}



fn pass1_1030_80ee(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1030_68dc(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((i16 *)param_1,(param_1 >> 0x10),0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn struct_1030_8128(param_1: *mut u32,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  let puVar2: *mut u8
  let puVar3: *mut u8
  let extraout_DX: *mut u8
  astruct_135 *iVar4;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = (astruct_135 *)param_1;
  uVar1 = 0x0;
  *param_1 = 0x0;
  &iVar4->field_0x4 = 0x0;
  iVar4->field_0x8 = 0x0;
  _PTR_LOOP_1050_5748 = param_1;
  mem_op_1000_179c(0x56,(uchar *)param_2,0x1000);
  puVar2 = (uchar *)(param_2 | uVar1);
  if (puVar2 != (uchar *)0x0) {
    pass1_1028_d81c(CONCAT22(param_2,uVar1),param_1,puVar2,param_3);
  }
  mem_op_1000_179c(0x8,puVar2,0x1000);
  puVar3 = (uchar *)(puVar2 | uVar1);
  if (puVar3 == (uchar *)0x0) {
    *param_1 = 0x0;
  }
  else {
    struct_1028_d22e(CONCAT22(puVar2,uVar1),param_1,puVar3);
    param_1 = uVar1;
    iVar4->field_0x2 = puVar3;
  }
  mem_op_1000_179c(0x8,puVar3,0x1000);
  puVar2 = (uchar *)(puVar3 | uVar1);
  if (puVar2 == (uchar *)0x0) {
    &iVar4->field_0x4 = 0x0;
  }
  else {
    pass1_1028_cfd2(CONCAT22(puVar3,uVar1),param_1);
    iVar4->field_0x4 = uVar1;
    iVar4->field_0x6 = extraout_DX;
    puVar2 = extraout_DX;
  }
  mem_op_1000_179c(0x24,puVar2,0x1000);
  puVar3 = (uchar *)(puVar2 | uVar1);
  if (puVar3 != (uchar *)0x0) {
    pass1_1030_5bec(CONCAT22(puVar2,uVar1));
  }
  mem_op_1000_179c(0x8,puVar3,0x1000);
  if ((uchar *)(puVar3 | uVar1) != (uchar *)0x0) {
    pass1_1038_78e2(CONCAT22(puVar3,uVar1),(uchar *)(puVar3 | uVar1));
  }
  PTR_LOOP_1050_574a = (_PTR_LOOP_1050_5748 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_8210(param_1: *mut u16)
{
  let uVar1: u16;
  let uVar2: u16;
  astruct_18 *paVar3;
  let iVar4: i16;
  let uVar5: u16;
  astruct_18 *paStack10;
  astruct_18 *paStack6;
  
  paVar3 = _PTR_LOOP_1050_65e2;
  if (_PTR_LOOP_1050_65e2 != (astruct_18 *)0x0) {
    pass1_1028_daba(_PTR_LOOP_1050_65e2,&USHORT_1050_1028);
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar1 = *param_1;
  uVar2 = (iVar4 + 0x2);
  paStack10 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1028_d282((u16 *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack10,0x1000);
  }
  uVar1 = (iVar4 + 0x4);
  uVar2 = (iVar4 + 0x6);
  paStack6 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1028_cff2(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack6,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_5736;
  if (_PTR_LOOP_1050_5736 != (astruct_18 *)0x0) {
    pass1_1030_5c0e();
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_5a64;
  if ((PTR_LOOP_1050_5a66 | _PTR_LOOP_1050_5a64) != 0x0) {
    pass1_1038_7964((u16 *)(_PTR_LOOP_1050_5a64 & 0xffff |
                            ZEXT24(PTR_LOOP_1050_5a66) << 0x10));
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  _PTR_LOOP_1050_5748 = 0x0;
  return;
}



fn pass1_1030_82f0(param_1: u16,param_2: u32,param_3: u32)
{
  pass1_1028_d078(param_1,(param_2 + 0x4),param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_8308(param_1: u16,param_2: u16,param_3: *mut u16,param_4: *mut u16,
               param_5: u32,param_6: u16,param_7: u16)

{
  pass1_1028_e198(_PTR_LOOP_1050_65e2,param_3,param_4,param_5,param_6,param_7);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_8326(void) -> u32

{
  return CONCAT22((_PTR_LOOP_1050_65e2 + 0x2),*_PTR_LOOP_1050_65e2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_8334(void)
{
  *_PTR_LOOP_1050_65e2 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_8344(param_1: u16,param_2: u16,param_3: u32)
{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3,(param_3 >> 0x10));
  return;
}



fn fn_ptr_1030_835a(u32 **param_1,param_2: *mut u32)
{
  fn_ptr_1028_d566(*param_1,param_2);
  return;
}



fn pass1_1030_8372(u32 **param_1,param_2: u32,param_3: *mut u32)
{
  pass1_1028_d52c(*param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_838e(param_1: *mut u32,param_2: u16,param_3: u8)
{
  struct_1028_d2b0(*param_1,param_2,param_3);
  pass1_1028_d01a(*(u32 **)(param_1 + 0x4));
  send_msg_1028_e242(_PTR_LOOP_1050_65e2,0x1,&USHORT_1050_1028);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_83ba(u32 **param_1,param_2: i32,param_3: u16,param_4: u8)
{
  let lVar1: i32;
  
  while (lVar1 = param_2 + -0x1, param_2 != 0x0) {
    struct_1028_d2b0(*param_1,param_3,param_4);
    pass1_1028_d01a(*(u32 **)(param_1 + 0x4));
    param_2 = lVar1;
    if (lVar1 != 0x0) {
      send_msg_1028_e242(_PTR_LOOP_1050_65e2,0x0,&USHORT_1050_1028);
    }
  }
  send_msg_1028_e242(_PTR_LOOP_1050_65e2,0x1,&USHORT_1050_1028);
  return;
}


fn pass1_1030_8480(astruct_18 **param_1)
{
  fn_ptr_1000_17ce(*param_1,0x1000);
  return;
}



fn pass1_1030_8496(param_1: u32)
{
  fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x2),0x1000);
  return;
}



fn pass1_1030_84ae(param_1: u32)
{
  pass1_1008_3e38((u16 *)(param_1 & 0xffff0000 | (param_1 + 0x8)));
  (param_1 + 0x1e) = 0x1;
  return;
}



fn fn_ptr_1030_84d0(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if ((iVar4 + 0x1e) != 0x0) {
    puVar1 = (iVar4 + 0xe);
    uVar2 = (iVar4 + 0x10);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    puVar1 = (iVar4 + 0x12);
    uVar2 = (iVar4 + 0x14);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar4 + 0x4),0x1000);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar4 + 0x16),0x1000);
  }
  return;
}


fn pass1_1030_85be(long *param_1,param_2: u16,param_3: i16,param_4: i16,param_5: u16)
{
  astruct_357 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_357 *)param_1;
  *param_1 = 0x0;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x6 = param_3;
  iVar1->field_0x8 = param_2;
  iVar1->field_0xe = 0x0;
  if (iVar1->field_0x6 == 0x0) {
    iVar1->field_0x6 = 0x5;
  }
  pass1_1030_878c(param_1,param_4,param_5);
  return;
}



fn pass1_1030_8604(astruct_18 **param_1)
{
  fn_ptr_1000_17ce(*param_1,0x1000);
  return;
}



void 
pass1_1030_861a(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16)

{
  let puStack6: u32;
  
  pass1_1030_8854(param_1,param_2,param_3,param_6);
  puStack6 = CONCAT22(param_5,param_4);
  if ((param_5 | param_4) == 0x0) {
    (param_1 + 0xa) = 0x0;
  }
  else {
    (param_1 + 0xa) = *puStack6;
  }
  return;
}



void 
pass1_1030_8660(param_1: u32,param_2: *mut u32,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16,param_7: i16)

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let puStack6: u32;
  
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  pass1_1030_8854(uVar2,uVar3,param_3,param_6);
  puStack6 = CONCAT22(param_5,param_4);
  uVar1 = param_5 | param_4;
  if (uVar1 == 0x0) {
    pass1_1030_8854(uVar2,uVar3,0x0,param_6);
    puStack6 = CONCAT22(uVar1,param_4);
    uVar1 |= param_4;
    if (uVar1 == 0x0) {
      pass1_1030_878c((long *)param_1,param_7,param_6);
      pass1_1030_8854(uVar2,uVar3,0x0,param_6);
      puStack6 = CONCAT22(uVar1,param_4);
      if ((uVar1 | param_4) == 0x0) {
        return;
      }
    }
    (puStack6 + 0x4) = param_3;
    *puStack6 = *param_2;
    pass1_1030_8834((u16 *)param_1,param_7,param_6);
  }
  else {
    *puStack6 = *param_2;
  }
  return;
}



fn pass1_1030_86ec(astruct_18 **param_1,param_2: u16)
{
  astruct_612 *iVar1;
  let uVar1: u16;
  
  fn_ptr_1000_17ce(*param_1,0x1000);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_612 *)param_1;
  *param_1 = (astruct_18 *)0x0;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x6 = param_2;
  iVar1->field_0xe = 0x0;
  return;
}



fn pass1_1030_871e(long *param_1,param_2: *mut u32,param_3: u16,param_4: i16,param_5: u16)
{
  let piVar1: *mut i16;
  astruct_681 *iVar2;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_681 *)param_1;
  if (*param_1 == 0x0) {
    pass1_1030_878c((long *)(param_1 & 0xffff | uVar2 << 0x10),param_4,
                    param_5);
  }
  piVar1 = &iVar2->field_0xe;
  *piVar1 = *piVar1 + 0x1;
  (*param_1 + iVar2->field_0xe * 0x6 + 0x4) = param_3;
  (iVar2->field_0xe * 0x6 + *param_1) = *param_2;
  return;
}



fn pass1_1030_877c(param_1: *mut u16,param_2: i16,param_3: u16)
{
  pass1_1030_8834(param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_878c(long *param_1,param_2: i16,param_3: u16)
{
  let puVar1: *mut u16;
  let uVar2: u16;
  let uVar3: u16;
  astruct_350 *iVar4;
  let uVar4: u16;
  let lVar5: i32;
  let lStack12: i32;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_350 *)param_1;
  if (iVar4->field_0x4 == 0x0) {
    PTR_LOOP_1050_5f2e = 0x0;
    uVar2 = iVar4->field_0x6;
  }
  else {
    uVar3 = iVar4->field_0x4;
    puVar1 = &iVar4->field_0x8;
    uVar2 = uVar3 + *puVar1;
    PTR_LOOP_1050_5f2e = CARRY2(uVar3,*puVar1);
  }
  if ((false) || (PTR_LOOP_1050_5f2e == 0x0)) {
    if (*param_1 == 0x0) {
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
      }
      else {
      }
      uVar3 = fn_ptr_op_1000_1708(uVar2 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,
                                  PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
      lVar5 = pass1_1000_0ed4(0x1000,param_3,0x1,uVar2 * 0x6,0x0,(u16 *)*param_1,
                              (*param_1 >> 0x10));
      PTR_LOOP_1050_5f2e = (lVar5 >> 0x10);
      uVar3 = lVar5;
    }
    lStack12 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    if ((PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
      iVar4->field_0x4 = uVar2;
      *param_1 = lStack12;
      pass1_1030_8834((u16 *)(param_1 & 0xffff | uVar4 << 0x10),param_2,
                      param_3);
    }
  }
  return;
}



fn pass1_1030_8834(param_1: *mut u16,param_2: i16,param_3: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x2);
  pass1_1000_4aea(*param_1,uVar1,(uVar1 >> 0x10),0x6,(uchar *)0x888e,
                  &stack0xfffe,param_2,uVar2,0x1000,param_3);
  return;
}



fn pass1_1030_8854(param_1: u16,param_2: u16,param_3: u16,param_4: u16)
{
  let uVar1: u32;
  let local_c: u32;
  let uStack8: u16;
  
  uStack8 = param_3;
  local_c = 0x0;
  uVar1 = (param_1 + 0x2);
  pass1_1000_49c6(&local_c,param_4,*_param_1,uVar1,
                  (uVar1 >> 0x10),0x6,(uchar *)0x888e,&stack0xfffe);
  return;
}



fn pass1_1030_888e(param_1: u32,param_2: u32) -> u16

{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = (param_1 + 0x4);
  uVar4 = (param_2 >> 0x10);
  piVar1 = (i16 *)(param_2 + 0x4);
  if (*piVar1 != iVar2 && iVar2 <= *piVar1) {
    return 0xffff;
  }
  if ((param_2 + 0x4) < (param_1 + 0x4)) {
    return 0x1;
  }
  return 0x0;
}



fn pass1_1030_88ce(param_1: *mut u16,param_2: u32,param_3: u32,param_4: u16)
{
  let puVar1: *mut u8
  let puVar2: *mut u8
  astruct_354 *iVar4;
  let uVar3: u16;
  let uVar4: u32;
  let puStack38: *mut u16;
  let iStack34: i16;
  let local_20: [u8;2];
  let local_1e: i16;
  let local_1c: i16;
  let local_1a: [u8;6];
  let local_14: [u8;6];
  let uStack14: u32;
  let uStack10: u32;
  let iStack6: i16;
  let uStack4: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar4 = (astruct_354 *)param_1;
  *param_1 = 0x389a;
  iVar4->field_0x2 = 0x1008;
  pass1_1030_84ae(param_1 & 0xffff0000 | &iVar4->field_0x4);
  iVar4->field_0x24 = param_3;
  puStack38 = (u16 *)(param_1 & 0xffff0000 | &iVar4->field_0x28);
  pass1_1008_6c90((u16 *)
                  (param_1 & 0xffff0000 | &iVar4->field_0x28));
  &iVar4->field_0x34 = 0x0;
  *param_1 = 0x8e38;
  iVar4->field_0x2 = 0x1030;
  struct_1030_8544((u16 *)
                   (param_1 & 0xffff0000 | &iVar4->field_0x4),
                   (u16 *)param_2);
  uVar4 = pass1_1008_4772(iVar4->field_0x12);
  uStack4 = (uVar4 >> 0x10);
  iStack6 = uVar4;
  uStack10 = (iStack6 + 0x4);
  uStack14 = (iStack6 + 0x8);
  pass1_1008_3e54((u16 *)CONCAT22(param_4,local_14),0x0,uStack14 - 0x1,
                  uStack10 - 0x1);
  pass1_1008_3e54((u16 *)CONCAT22(param_4,local_1a),0x0,0x0,0x0);
  pass1_1008_6d18(puStack38,(u16 *)CONCAT22(param_4,local_14),
                  (u16 *)CONCAT22(param_4,local_1a));
  pass1_1008_6d64(puStack38,(u16 *)CONCAT22(param_4,local_1a));
  pass1_1008_3eb4((u16 *)CONCAT22(param_4,local_1a),
                  (u16 *)CONCAT22(param_4,local_20),
                  (u16 *)CONCAT22(param_4,&local_1e),
                  (u16 *)CONCAT22(param_4,&local_1c));
  puVar1 = (uchar *)(((long)local_1e * (long)local_1c) >> 0x10);
  uVar4 = (long)local_1e * (long)local_1c & 0xffff;
  iVar4->field_0x34 = uVar4;
  iVar4->field_0x36 = puVar1;
  for (iStack34 = 0x0; iStack34 < 0x5; iStack34 += 0x1) {
    mem_op_1000_179c(0x10,puVar1,0x1000);
    puVar2 = (uchar *)(puVar1 | uVar4);
    if (puVar2 == (uchar *)0x0) {
      (&iVar4[0x1].field_0x0 + iStack34 * 0x4) = 0x0;
    }
    else {
      pass1_1030_85be((long *)(uVar4 & 0xffff | ZEXT24(puVar1) << 0x10),0x19,0x64,uVar3,
                      param_4);
      (&iVar4[0x1].field_0x0 + iStack34 * 0x4) = uVar4;
      (&iVar4[0x1].field_0x2)[iStack34 * 0x2] = puVar2;
    }
    puVar1 = puVar2;
  }
  return;
}



fn pass1_1030_8a2c(param_1: *mut u16)
{
  let uVar1: u16;
  astruct_18 *paVar2;
  astruct_613 *iVar3;
  let uVar3: u16;
  let iStack4: i16;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_613 *)param_1;
  *param_1 = 0x8e38;
  iVar3->field_0x2 = 0x1030;
  iStack4 = 0x0;
  do {
    paVar2 = *(astruct_18 **)(&iVar3[0x1].field_0x0 + iStack4 * 0x4);
    uVar1 = (&iVar3[0x1].field_0x2)[iStack4 * 0x2];
    if ((uVar1 | paVar2) != 0x0) {
      pass1_1030_8604((astruct_18 **)(paVar2 & 0xffff | uVar1 << 0x10));
      fn_ptr_1000_17ce(paVar2,0x1000);
    }
    iStack4 += 0x1;
  } while (iStack4 < 0x5);
  fn_ptr_1030_84d0(param_1 & 0xffff0000 | &iVar3->field_0x4);
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  return;
}



fn pass1_1030_8aa0(param_1: u32,param_2: u32,param_3: *mut u16,param_4: u16,param_5: u16)
{
  let uVar1: u16;
  let unaff_DI: i16;
  let local_12: u32;
  let puStack14: *mut u8;
  let uStack12: u32;
  let local_8: [u8;2];
  let local_6: [u8;2];
  let local_4: [u8;2];
  
  puStack14 = local_8;
  pass1_1008_3eb4(param_3,(u16 *)
                          CONCAT13((char)(param_5 >> 0x8),
                                   CONCAT12((char)param_5,puStack14)),
                  (u16 *)CONCAT22(param_5,local_6),(u16 *)CONCAT22(param_5,local_4))
  ;
  bad_1030_8cd2();
  uStack12 = CONCAT22(param_4,puStack14);
  uVar1 = param_4 | puStack14;
  if (uVar1 != 0x0) {
    pass1_1030_8d9e(param_1,param_5);
    local_12 = param_2;
    pass1_1030_8660(uStack12,CONCAT22(param_5,&local_12),puStack14,
                    &local_12,uVar1,param_5,unaff_DI);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1030_8b00(param_1: u32,param_2: *mut u16,param_3: *mut u16,param_4: u16)
{
  let puVar1: u32;
  let piVar2: *mut i16;
  let uVar3: u16;
  let local_2a: u32;
  let uStack38: u32;
  let uStack28: u32;
  let puStack18: u32;
  let puStack16: u32;
  let piStack14: *mut i16;
  let local_c: i16;
  let local_a: [u8;4];
  let uStack6: u32;
  
  uStack6 = 0x0;
  puVar1 = (local_a + 0x2);
  piVar2 = &local_c;
  pass1_1008_3eb4(param_2,(u16 *)
                          CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,piVar2)),
                  (u16 *)CONCAT22(param_4,local_a),(u16 *)CONCAT22(param_4,puVar1));
  bad_1030_8cd2();
  puStack16 = puVar1;
  piStack14 = piVar2;
  pass1_1030_8d9e(param_1,param_4);
  puStack18 = puVar1;
  pass1_1030_861a(puStack16,piStack14,puVar1,puVar1,
                  piVar2,param_4);
  uStack38 = *puVar1;
  uVar3 = (puVar1 + 0x2);
  uStack38._3_1_ = (char)(uStack38 >> 0x18);
  uStack6 = uStack38;
  if (uStack38._3_1_ == '\0') {
    puVar1 = &local_2a;
    uStack28 = uStack38;
    pass1_1030_8c66(param_1,local_c,(byte *)local_a,(local_a >> 0x10),
                    CONCAT22(param_4,puVar1),uVar3);
    uStack6 = *puVar1;
    uVar3 = (puVar1 + 0x2);
  }
  *param_3 = uStack6;
  (param_3 + 0x2) = uVar3;
  return;
}



fn pass1_1030_8bac(param_1: u32,param_2: u16)
{
  let iStack4: i16;
  
  iStack4 = 0x0;
  do {
    pass1_1030_86ec((param_1 + 0x38 + iStack4 * 0x4),param_2);
    iStack4 += 0x1;
  } while (iStack4 < 0x5);
  return;
}



fn pass1_1030_8bdc(param_1: u32,param_2: u32,param_3: *mut u16,param_4: i16,param_5: u16)
{
  let puVar1: *mut u8;
  let local_12: u32;
  let puStack14: *mut u8;
  long *plStack12;
  let local_8: [u8;2];
  let local_6: [u8;2];
  let local_4: [u8;2];
  
  puStack14 = local_4;
  puVar1 = local_8;
  pass1_1008_3eb4(param_3,(u16 *)
                          CONCAT13((char)(param_5 >> 0x8),CONCAT12((char)param_5,puVar1)),
                  (u16 *)CONCAT22(param_5,local_6),
                  (u16 *)CONCAT22(param_5,puStack14));
  bad_1030_8cd2();
  plStack12 = (long *)CONCAT22(puVar1,puStack14);
  pass1_1030_8d9e(param_1,param_5);
  local_12 = param_2;
  pass1_1030_871e(plStack12,CONCAT22(param_5,&local_12),puStack14,param_4
                  ,param_5);
  return;
}



fn pass1_1030_8c38(param_1: u32,param_2: i16,param_3: u16)
{
  let iStack4: i16;
  
  iStack4 = 0x0;
  do {
    pass1_1030_877c(*(u16 **)(param_1 + 0x38 + iStack4 * 0x4),param_2,param_3);
    iStack4 += 0x1;
  } while (iStack4 < 0x5);
  return;
}



void 
pass1_1030_8c66(param_1: u32,param_2: i16,param_3: *mut u8,param_4: u16,param_5: *mut u32,
               param_6: u16)

{
  let bVar1: u8;
  let uVar2: u16;
  let uStack6: u32;
  
  pass1_1008_4544((param_1 + 0x12));
  bVar1 = *param_3;
  uVar2 = bVar1;
  uStack6 = (uVar2 + 0x1);
  if (0x0 < param_2) {
    if (uVar2 == 0x0) {
      uStack6 = 0x7;
    }
    else {
      if (((bVar1 == 0x0) || (SBORROW2(uVar2,0x1))) || (0x1 < (uVar2 - 0x1))) {
        uStack6 = 0x9;
      }
      else {
        uStack6 = 0x8;
      }
    }
  }
  *param_5 = uStack6;
  return;
}


fn pass1_1030_8d08(param_1: u32,param_2: u16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u32;
  let uStack16: u32;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    uVar4 = (param_1 >> 0x10);
    piVar1 = (i16 *)(param_1 + 0x1e);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    uVar3 = iStack4 * 0x6;
    uVar2 = (param_1 + 0x1a);
    (uVar2 + uVar3 + 0x4) = 0x0;
    pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x500);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,param_2);
    uStack16 = CONCAT22(param_2,uVar3);
    uVar5 = pass1_1028_e2e0(_PTR_LOOP_1050_65e2,param_2,0x7);
    param_2 = (uVar5 >> 0x10);
    pass1_1030_7e5a(uStack16,uVar5 & 0xffff | param_2 << 0x10,param_2);
    iStack4 += 0x1;
  }
  return;
}



fn pass1_1030_8d9e(param_1: u32,param_2: u16)
{
  let local_c: [u8;2];
  let local_a: [u8;2];
  let local_8: [u8;6];
  
  pass1_1008_3e38((u16 *)CONCAT22(param_2,local_8));
  pass1_1008_6d64((u16 *)(param_1 & 0xffff0000 | (param_1 + 0x28)),
                  (u16 *)CONCAT22(param_2,local_8));
  pass1_1008_3e94((u16 *)CONCAT22(param_2,local_8),(u16 *)CONCAT22(param_2,local_c),
                  (u16 *)CONCAT22(param_2,local_a));
  return;
}



astruct_18 *  pass1_1030_8e12(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_8a2c((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_8e3c(param_1: u16,param_2: u16,uchar *param_3,param_4: u32,param_5: u32) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let unaff_DI: i16;
  let puVar5: u32;
  let puVar6: *mut u16;
  let uVar7: u16;
  
  mem_op_1000_179c(0xc,param_3,0x1000);
  if ((param_3 | param_2) == 0x0) {
    puVar5 = 0x0;
  }
  else {
    puVar5 = set_struct_1008_574a((astruct_21 *)CONCAT22(param_3,param_2));
  }
  if (param_5._3_1_ == '\x04') {
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,
                             (uchar *)(puVar5 >> 0x10),unaff_DI);
    uVar3 = (puVar6 >> 0x10);
    uVar1 = (puVar6 + 0x1e);
    uVar2 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5,(param_5 >> 0x10));
    uVar7 = (param_4 >> 0x10);
    uVar4 = uVar3;
    if (uVar1 < 0x1) {
      pass1_1030_9296(param_4,puVar5,CONCAT22(uVar3,uVar2),param_1,uVar2,uVar3);
      pass1_1030_951a(param_1,uVar4,param_4,puVar5,CONCAT22(uVar3,uVar2));
    }
    else {
      pass1_1030_9adc(param_4,uVar7,puVar5,CONCAT22(uVar3,uVar2),uVar2,uVar3);
      pass1_1030_9c1c(param_4,puVar5,CONCAT22(uVar3,uVar2));
    }
    pass1_1030_9d42(param_1,uVar4,param_4,uVar7,puVar5,CONCAT22(uVar3,uVar2));
  }
  return puVar5;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_8f04(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u32;
  let uVar6: u16;
  let iStack8: i16;
  let uStack6: u32;
  
  pass1_1038_53ba(param_3,0x1);
  if ((((param_5 != 0x0) || (0x1 < param_4)) &&
      ((pass1_1038_53ba(param_3,0x2), param_5 != 0x0 || (0x1 < param_4)))) &&
     ((pass1_1038_53ba(param_3,0x3), param_5 != 0x0 || (0x1 < param_4)))) {
    pass1_1038_53ba(param_3,0x4);
    uVar5 = param_5;
    if ((param_5 != 0x0) || (0x1 < param_4)) {
      empty_1038_540a();
      uStack6 = param_4 & 0xffff | uVar5 << 0x10;
      iStack8 = 0x0;
      do {
        uVar3 = uVar5;
        uVar2 = param_4;
        if (0x0 < (iStack8 * 0x2 + _PTR_LOOP_1050_580e)) {
          empty_1038_540a();
          uVar6 = (_PTR_LOOP_1050_580e >> 0x10);
          uVar1 = (iStack8 * 0x2 + _PTR_LOOP_1050_580e);
          param_4 = uVar1;
          uVar4 = uVar1 >> 0xf;
          uVar5 = uVar4;
          if ((uVar3 <= uVar4) && ((uVar3 < uVar4 || (uVar2 < uVar1)))) {
            if (0x1c < iStack8) {
              return;
            }
            uVar2 = (iStack8 * 0x2 + _PTR_LOOP_1050_580e);
            param_4 = SEXT24(uVar2);
            uVar5 = param_4 >> 0x10;
            if ((long)uStack6 < (long)param_4) {
              return;
            }
            uStack6 = CONCAT22(((uStack6 >> 0x10) - (uVar2 >> 0xf)) -
                               (uStack6 < uVar2),uStack6 - uVar2);
          }
        }
        iStack8 += 0x1;
        if (0x24 < iStack8) {
          return;
        }
      } while( true );
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool 
pass1_1030_8fe4(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: *mut u16,param_7: i32)

{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u32;
  
  pass1_1030_627e(param_1,param_2,param_3,_PTR_LOOP_1050_5740,param_6,param_7);
  uVar2 = param_3 | param_2;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_2,param_3);
    if ((uVar2 | param_2) != 0x0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(uVar2,param_2));
      if ((uVar3 != 0x0) &&
         ((iVar1 = (uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_9048(param_1: u16,param_2: u32,param_3: i16,param_4: u32)
{
  let iVar1: i16;
  let uVar2: u32;
  code **ppcVar3;
  let Bvar4: bool;
  let uVar5: u16;
  let uVar6: u32;
  let puVar7: *mut u8
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar8: u16;
  let uVar9: u16;
  let puVar10: u32;
  let uVar11: u16;
  let uVar12: u16;
  let puVar13: u32;
  let uVar14: u32;
  let uVar15: u16;
  let uVar16: u8;
  let uStack36: u32;
  let local_18: [u8;2];
  let local_16: i16;
  let local_14: i16;
  let local_12: i16;
  let iStack16: i16;
  let uStack12: u32;
  let uStack8: u16;
  let puStack6: *mut u8
  let iStack4: i16;
  
  iStack4 = 0x8 - (param_3 == 0x0);
  puVar13 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,iStack4);
  puVar7 = (uchar *)(puVar13 >> 0x10);
  uVar8 = puVar13;
  uStack8 = uVar8;
  puStack6 = puVar7;
  pass1_1038_4e78(uVar8,puVar7,param_4,puVar13);
  uStack12 = CONCAT22(puVar7,uVar8);
  uVar12 = 0x1008;
  pass1_1008_3e38((u16 *)CONCAT22(param_1,&local_12));
  uVar2 = (param_4 + 0x8);
  uVar11 = (uStack12 >> 0x10);
  uVar9 = SUB42(uStack12,0x0);
  ppcVar3 = (code **)(*uStack12 + 0x10);
  uVar6 = uVar2;
  (**ppcVar3)(0x1008,uVar9,uVar11);
  uVar6 = uVar6 & 0xffff | extraout_DX << 0x10;
  uStack36 = 0x0;
  while( true ) {
    if (uVar6 <= uStack36) {
      if (uStack12 != 0x0) {
        ppcVar3 = (code **)*uStack12;
        (**ppcVar3)(uVar12,uStack12,(char)(uStack12 >> 0x10),0x1,uVar9,uVar11,
                    uStack12,uStack12);
      }
      return;
    }
    ppcVar3 = (code **)(*uStack12 + 0x4);
    uVar14 = uVar6;
    (**ppcVar3)();
    uVar5 = uVar14;
    uVar8 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,extraout_DX_00);
    pass1_1008_3f62((u16 *)CONCAT22(param_1,&local_12),
                    (u16 *)CONCAT22(uVar8,uVar5 + 0xc));
    uVar12 = 0x1008;
    pass1_1008_3eb4((u16 *)CONCAT22(param_1,&local_12),
                    (u16 *)CONCAT22(param_1,local_18),
                    (u16 *)CONCAT22(param_1,&local_16),
                    (u16 *)CONCAT22(param_1,&local_14));
    uVar14 = struct_op_1030_73a8(CONCAT22(uVar8,uVar5));
    uVar8 = (uVar14 >> 0x10);
    iVar1 = (uVar14 + 0xc);
    if (iVar1 - 0x7aU < 0x6) break;
LAB_1030_91fa:
    uStack36 += 0x1;
  }
  uVar12 = 0x1030;
  uVar5 = param_2;
  uVar15 = (param_2 >> 0x10);
  switch(iVar1) {
  default:
    iStack16 = local_16 + -0x1;
    BVar4 = pass1_1030_8fe4(param_1,&local_12,uVar8,uVar5,uVar15,
                            (u16 *)CONCAT22(param_1,&local_12),uVar2);
    if (BVar4 != 0x0) goto LAB_1030_91cb;
    iStack16 = local_16 + 0x1;
    BVar4 = pass1_1030_8fe4(param_1,&local_12,uVar8,uVar5,uVar15,
                            (u16 *)CONCAT22(param_1,&local_12),uVar2);
    if (BVar4 == 0x0) {
      iStack16 = local_16;
      local_12 = local_14 + -0x1;
      BVar4 = pass1_1030_8fe4(param_1,&local_12,uVar8,uVar5,uVar15,
                              (u16 *)CONCAT22(param_1,&local_12),uVar2);
      goto joined_r0x1030911e;
    }
LAB_1030_9144:
    break;
  case 0x7b:
  case 0x7e:
    iStack16 = local_16 + -0x1;
    BVar4 = pass1_1030_8fe4(param_1,&local_12,uVar8,uVar5,uVar15,
                            (u16 *)CONCAT22(param_1,&local_12),uVar2);
    if (BVar4 == 0x0) {
      iStack16 = local_16 + 0x1;
      goto LAB_1030_912c;
    }
    if (uStack12 == 0x0) {
      return;
    }
    uVar12 = (uStack12 >> 0x10);
    puVar10 = uStack12;
    uVar16 = (u8)(uStack12 >> 0x10);
    goto LAB_1030_90e6;
  case 0x7c:
  case 0x7d:
    local_12 = local_14 + -0x1;
    BVar4 = pass1_1030_8fe4(param_1,&local_12,uVar8,uVar5,uVar15,
                            (u16 *)CONCAT22(param_1,&local_12),uVar2);
joined_r0x1030911e:
    if (BVar4 == 0x0) {
      local_12 = local_14 + 0x1;
LAB_1030_912c:
      BVar4 = pass1_1030_8fe4(param_1,&local_12,uVar8,uVar5,uVar15,
                              (u16 *)CONCAT22(param_1,&local_12),uVar2);
      if (BVar4 != 0x0) goto LAB_1030_9144;
      goto LAB_1030_91fa;
    }
LAB_1030_91cb:
  }
  puVar10 = uStack12;
  if ((uStack12._2_2_ | puVar10) != 0x0) {
    uVar12 = (uStack12 >> 0x10);
    uVar16 = (u8)(uStack12 >> 0x10);
LAB_1030_90e6:
    ppcVar3 = (code **)*puVar10;
    (**ppcVar3)(0x1030,puVar10,uVar16,0x1,uVar9,uVar11,uStack12,uStack12);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_9296(param_1: u32,param_2: *mut u32,param_3: u32,param_4: u16,param_5: u16,
               param_6: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  let puVar3: *mut u8;
  let in_register_00000002: u16;
  astruct_99 *paVar4;
  let uVar6: u32;
  let uVar7: u16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let puVar8: *mut u8
  let extraout_DX_01: *mut u8
  let extraout_DX_02: *mut u8
  let extraout_DX_03: u16;
  let uVar9: u16;
  astruct_116 *iVar11;
  astruct_115 *iVar10;
  astruct_114 *iVar9;
  let unaff_DI: i16;
  let uVar10: u16;
  let uVar11: u8;
  u8 local_3a [0xc];
  let uStack46: u32;
  let uStack36: u32;
  let uStack30: u32;
  let uStack26: u16;
  astruct_99 *paStack18;
  let uStack14: u32;
  let puStack10: *mut u16;
  astruct_99 *paStack6;
  astruct_113 *uVar5;
  
  paVar4 = (astruct_99 *)CONCAT22(in_register_00000002,param_5);
  pass1_1038_53ba(param_3,0x1);
  uVar7 = param_6 | paVar4;
  uVar10 = (param_2 >> 0x10);
  uVar11 = SUB41(param_2,0x0);
  if (uVar7 != 0x0) {
    uStack30 = _PTR_LOOP_1050_5768;
    uVar6 = _PTR_LOOP_1050_5768;
    paStack18 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar7 = (paStack18 >> 0x10);
    paVar4 = (astruct_99 *)(uVar6 & 0xffff0000 | paStack18 & 0xffff);
    if ((uVar7 | (paStack18 & 0xffff)) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      iVar11 = (astruct_116 *)paStack18;
      paStack18->field_0x0 = 0x389a;
      iVar11->field_0x2 = 0x1008;
      iVar11->field_0x4 = 0x73;
      paStack18->field_0x0 = 0x9ec8;
      iVar11->field_0x2 = 0x1030;
      paVar4 = paStack18;
      paStack6 = paStack18;
    }
    ppcVar1 = (code **)(*param_2 + 0x4);
    (**ppcVar1)(0x1000,uVar11,uVar10,paStack6,(paStack6 >> 0x10));
    uVar7 = extraout_DX;
  }
  pass1_1038_53ba(param_3,0x2);
  uVar7 |= paVar4;
  if (uVar7 != 0x0) {
    uStack30 = _PTR_LOOP_1050_5768;
    uVar6 = _PTR_LOOP_1050_5768;
    paStack18 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar7 = (paStack18 >> 0x10);
    paVar4 = (astruct_99 *)(uVar6 & 0xffff0000 | paStack18 & 0xffff);
    if ((uVar7 | (paStack18 & 0xffff)) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      iVar10 = (astruct_115 *)paStack18;
      paStack18->field_0x0 = 0x389a;
      iVar10->field_0x2 = 0x1008;
      iVar10->field_0x4 = 0x74;
      paStack18->field_0x0 = 0x9ec8;
      iVar10->field_0x2 = 0x1030;
      paVar4 = paStack18;
      paStack6 = paStack18;
    }
    ppcVar1 = (code **)(*param_2 + 0x8);
    (**ppcVar1)(0x1000,uVar11,uVar10,paStack6,(paStack6 >> 0x10));
    uVar7 = extraout_DX_00;
  }
  pass1_1038_53ba(param_3,0x3);
  puVar8 = (uchar *)(uVar7 | paVar4);
  if (puVar8 != (uchar *)0x0) {
    uStack36 = _PTR_LOOP_1050_5768;
    uVar6 = _PTR_LOOP_1050_5768;
    paStack18 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar7 = (paStack18 >> 0x10);
    paVar4 = (astruct_99 *)(uVar6 & 0xffff0000 | paStack18 & 0xffff);
    if ((uVar7 | (paStack18 & 0xffff)) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      iVar9 = (astruct_114 *)paStack18;
      paStack18->field_0x0 = 0x389a;
      iVar9->field_0x2 = 0x1008;
      iVar9->field_0x4 = 0x75;
      paStack18->field_0x0 = 0x9ec8;
      iVar9->field_0x2 = 0x1030;
      paVar4 = paStack18;
      paStack6 = paStack18;
    }
    ppcVar1 = (code **)(*param_2 + 0x8);
    (**ppcVar1)(0x1000,uVar11,uVar10,paStack6,(paStack6 >> 0x10));
    puVar8 = extraout_DX_01;
  }
  pass1_1030_8f04(param_1,(param_1 >> 0x10),param_3,paVar4,
                  puVar8);
  if (paVar4 != 0x0) {
    uStack36 = _PTR_LOOP_1050_5768;
    paStack18 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar7 = (paStack18 >> 0x10);
    uVar5 = (astruct_113 *)paStack18;
    if ((uVar7 | uVar5) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      paStack18->field_0x0 = 0x389a;
      uVar5->field_0x2 = 0x1008;
      uVar5->field_0x4 = 0x37;
      paStack18->field_0x0 = 0x9ec8;
      uVar5->field_0x2 = 0x1030;
      paStack6 = paStack18;
    }
    ppcVar1 = (code **)(*param_2 + 0x8);
    (**ppcVar1)(0x1000,uVar11,uVar10,paStack6,(paStack6 >> 0x10));
    puVar8 = extraout_DX_02;
  }
  puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_4,puVar8,unaff_DI);
  uVar2 = (puStack10 >> 0x10);
  uStack14 = (puStack10 + 0xe);
  uVar7 = (puStack10 + 0x10);
  if ((uVar7 | uStack14) != 0x0) {
    pass1_1008_5784(CONCAT22(param_4,local_3a),
                    uStack14 & 0xffff | uVar7 << 0x10);
    while( true ) {
      puVar3 = local_3a;
      pass1_1008_5b12(puVar3,param_4);
      uStack46 = CONCAT22(extraout_DX_03,puVar3);
      if ((extraout_DX_03 | puVar3) == 0x0) break;
      if (((puVar3 + 0x4) == 0x3e) || ((puVar3 + 0x4) == 0x41)) {
        uStack30 = _PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        uVar9 = (paStack18 >> 0x10);
        uVar7 = paStack18;
        if ((uVar9 | uVar7) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          uStack26 = (uStack46 + 0x4);
          paStack18->field_0x0 = 0x389a;
          (uVar7 + 0x2) = 0x1008;
          (uVar7 + 0x4) = uStack26;
          paStack18->field_0x0 = 0x9ec8;
          (uVar7 + 0x2) = 0x1030;
          paStack6 = paStack18;
        }
        ppcVar1 = (code **)(*param_2 + 0x8);
        (**ppcVar1)(0x1000,uVar11,uVar10,paStack6,(paStack6 >> 0x10));
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_951a(param_1: u16,param_2: u16,param_3: u32,param_4: *mut u32,param_5: u32)
{
  code **ppcVar1;
  let uVar6: u16;
  let puVar7: *mut u16;
  let uVar8: u16;
  let puVar9: *mut u8
  let extraout_DX: u16;
  let uVar10: u16;
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let iVar11: i16;
  let puVar12: *mut u16;
  let unaff_DI: i16;
  let uVar13: u16;
  let uVar14: u16;
  let uVar15: u8;
  let puVar16: u32;
  let uVar17: u32;
  let uVar18: u8;
  let uVar19: u8;
  let uVar20: u8;
  let puStack76: u32;
  let uStack70: u32;
  let uStack62: u32;
  astruct_99 *paStack58;
  let local_36: u16;
  let uStack52: u16;
  let uStack46: u32;
  let uStack42: u16;
  let uStack40: u16;
  let iStack38: i16;
  let puStack36: *mut u16;
  let puStack32: *mut u16;
  let iStack28: i16;
  let iStack20: i16;
  let uStack18: u32;
  let uStack14: u32;
  let puStack10: *mut u16;
  astruct_99 *paStack6;
  astruct_122 *uVar2;
  astruct_123 *uVar3;
  astruct_124 *uVar4;
  astruct_125 *uVar5;
  
  puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,param_1,(uchar *)param_2,unaff_DI);
  puVar9 = (uchar *)(puStack10 >> 0x10);
  uVar6 = puStack10 + 0xa;
  uStack14 = puStack10 & 0xffff0000 | uVar6;
  pass1_1030_9048(param_1,param_3,0x0,param_5);
  uVar13 = (param_4 >> 0x10);
  uVar20 = SUB41(param_4,0x0);
  if (uVar6 != 0x0) {
    iStack28 = 0x0;
    puStack32 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_1,puVar9,unaff_DI);
    uVar14 = (puStack32 >> 0x10);
    puStack36 = *(u16 **)(puStack32 + 0xe);
    uVar6 = (puStack32 + 0x10);
    if ((uVar6 | puStack36) != 0x0) {
      pass1_1008_5784(CONCAT22(param_1,&local_36),
                      puStack36 & 0xffff | uVar6 << 0x10);
      while( true ) {
        puVar7 = &local_36;
        pass1_1008_5b12(puVar7,param_1);
        uStack46 = CONCAT22(extraout_DX,puVar7);
        if ((extraout_DX | puVar7) == 0x0) break;
        if ((puVar7[0x2] != 0x3e) && (puVar7[0x2] != 0x41)) {
          paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
          uVar10 = (paStack6 >> 0x10);
          uVar6 = paStack6;
          if ((uVar10 | uVar6) == 0x0) {
            paStack6 = (astruct_99 *)0x0;
          }
          else {
            uVar14 = (uStack46 + 0x4);
            paStack6->field_0x0 = 0x389a;
            (uVar6 + 0x2) = 0x1008;
            (uVar6 + 0x4) = uVar14;
            paStack6->field_0x0 = 0x9ec8;
            (uVar6 + 0x2) = 0x1030;
          }
          ppcVar1 = (code **)(*param_4 + 0x8);
          (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
          if ((uStack46 + 0x4) == 0x13) {
            iStack28 = 0x1;
          }
        }
      }
    }
    for (iStack38 = 0xa; iStack38 < 0x41; iStack38 += 0x1) {
      if ((((((iStack38 != 0x37) && (iStack38 != 0x35)) && (iStack38 != 0x36)) &&
           ((iStack38 != 0x25 && (iStack38 != 0x26)))) &&
          ((iStack38 != 0x27 &&
           (((iStack38 * 0x2 + uStack14) != 0x0 && (iStack38 != 0x13)))))) &&
         ((iStack38 != 0x14 || (iStack28 == 0x0)))) {
        paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        uVar10 = (paStack6 >> 0x10);
        uVar6 = paStack6;
        if ((uVar10 | uVar6) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          paStack6->field_0x0 = 0x389a;
          (uVar6 + 0x2) = 0x1008;
          (uVar6 + 0x4) = iStack38;
          paStack6->field_0x0 = 0x9ec8;
          (uVar6 + 0x2) = 0x1030;
        }
        ppcVar1 = (code **)(*param_4 + 0x8);
        (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
      }
    }
  }
  uVar14 = (uStack14 >> 0x10);
  if ((uStack14 + 0x6a) == 0x0) {
    if ((uStack14 + 0x6c) != 0x0) {
      paStack58 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
      uVar6 = (paStack58 >> 0x10);
      puVar12 = (u16 *)paStack58;
      if ((uVar6 | puVar12) == 0x0) goto LAB_1030_973e;
      paStack58->field_0x0 = 0x389a;
      puVar12[0x1] = 0x1008;
      puVar12[0x2] = 0x36;
      goto LAB_1030_9728;
    }
  }
  else {
    paStack58 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar6 = (paStack58 >> 0x10);
    puVar12 = (u16 *)paStack58;
    if ((uVar6 | puVar12) == 0x0) {
LAB_1030_973e:
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      paStack58->field_0x0 = 0x389a;
      puVar12[0x1] = 0x1008;
      puVar12[0x2] = 0x35;
LAB_1030_9728:
      *puVar12 = 0x9ec8;
      puVar12[0x1] = 0x1030;
      paStack6 = paStack58;
    }
    ppcVar1 = (code **)(*param_4 + 0x8);
    (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
  }
  uVar14 = (uStack14 >> 0x10);
  iVar11 = uStack14;
  if ((iVar11 + 0x4a) == 0x0) {
    if ((iVar11 + 0x4c) == 0x0) {
      if ((iVar11 + 0x4e) == 0x0) goto LAB_1030_97e8;
      paStack58 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
      uVar6 = (paStack58 >> 0x10);
      puVar12 = (u16 *)paStack58;
      if ((uVar6 | puVar12) != 0x0) {
        paStack58->field_0x0 = 0x389a;
        puVar12[0x1] = 0x1008;
        puVar12[0x2] = 0x27;
        goto LAB_1030_9879;
      }
    }
    else {
      paStack58 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
      uVar6 = (paStack58 >> 0x10);
      puVar12 = (u16 *)paStack58;
      if ((uVar6 | puVar12) != 0x0) {
        paStack58->field_0x0 = 0x389a;
        puVar12[0x1] = 0x1008;
        puVar12[0x2] = 0x26;
        goto LAB_1030_9879;
      }
    }
LAB_1030_97d0:
    paStack6 = (astruct_99 *)0x0;
  }
  else {
    paStack58 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar6 = (paStack58 >> 0x10);
    puVar12 = (u16 *)paStack58;
    if ((uVar6 | puVar12) == 0x0) goto LAB_1030_97d0;
    paStack58->field_0x0 = 0x389a;
    puVar12[0x1] = 0x1008;
    puVar12[0x2] = 0x25;
LAB_1030_9879:
    *puVar12 = 0x9ec8;
    puVar12[0x1] = 0x1030;
    paStack6 = paStack58;
  }
  ppcVar1 = (code **)(*param_4 + 0x8);
  (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
LAB_1030_97e8:
  uStack18 = puStack10 & 0xffff0000 | (puStack10 + 0x11e);
  if ((puStack10 + 0x138) != 0x0) {
    puVar16 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
    puVar9 = (uchar *)(puVar16 >> 0x10);
    uVar6 = puVar16;
    uVar15 = 0x38;
    pass1_1038_4d6e(param_5,puVar16,uVar6,puVar9);
    puStack76 = CONCAT22(puVar9,uVar6);
    ppcVar1 = (code **)(*puStack76 + 0x10);
    uVar10 = uVar6;
    (**ppcVar1)(&PTR_LOOP_1050_1038,uVar6,puVar9);
    uStack70 = CONCAT22(extraout_DX_00,uVar10);
    for (uStack62 = 0x0; uStack62 < uStack70; uStack62 += 0x1) {
      ppcVar1 = (code **)(*puStack76 + 0x4);
      uVar17 = uStack70;
      (**ppcVar1)(uVar15,(char)uVar6,puVar9,uStack62,(uStack62 >> 0x10));
      uVar8 = uVar17;
      iVar11 = 0xd;
      uVar10 = extraout_DX_01;
      local_36 = uVar8;
      uStack52 = extraout_DX_01;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar8,extraout_DX_01);
      uStack46 = CONCAT22(uVar10,uVar8);
      uVar17 = struct_op_1030_73a8(CONCAT22(uVar10,uVar8));
      uVar10 = (uVar17 >> 0x10);
      uStack42 = uVar17;
      uVar15 = 0x28;
      uStack40 = uVar10;
      uVar8 = pass1_1028_6744(param_1,uVar17,iVar11);
      if ((uVar10 | uVar8) != 0x0) {
        puStack32 = _PTR_LOOP_1050_5768;
        paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        uVar10 = (paStack6 >> 0x10);
        uVar5 = (astruct_125 *)paStack6;
        if ((uVar10 | uVar5) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          paStack6->field_0x0 = 0x389a;
          uVar5->field_0x2 = 0x1008;
          uVar5->field_0x4 = 0x4c;
          paStack6->field_0x0 = 0x9ec8;
          uVar5->field_0x2 = 0x1030;
        }
        ppcVar1 = (code **)(*param_4 + 0x8);
        (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
        puStack36 = _PTR_LOOP_1050_5768;
        paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        uVar10 = (paStack6 >> 0x10);
        uVar4 = (astruct_124 *)paStack6;
        if ((uVar10 | uVar4) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          paStack6->field_0x0 = 0x389a;
          uVar4->field_0x2 = 0x1008;
          uVar4->field_0x4 = 0x4d;
          paStack6->field_0x0 = 0x9ec8;
          uVar4->field_0x2 = 0x1030;
        }
        uVar18 = SUB41(paStack6,0x0);
        uVar19 = (u8)(paStack6 >> 0x10);
        ppcVar1 = (code **)(*param_4 + 0x8);
        puVar16 = param_4;
        (**ppcVar1)();
        puStack36 = _PTR_LOOP_1050_5768;
        uVar15 = 0x0;
        paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        uVar10 = (paStack6 >> 0x10);
        uVar3 = (astruct_123 *)paStack6;
        if ((uVar10 | uVar3) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          paStack6->field_0x0 = 0x389a;
          uVar3->field_0x2 = 0x1008;
          uVar3->field_0x4 = 0x4e;
          paStack6->field_0x0 = 0x9ec8;
          uVar3->field_0x2 = 0x1030;
        }
        ppcVar1 = (code **)(*param_4 + 0x8);
        (**ppcVar1)(0x1000,param_4,paStack6,puVar16,uVar18,uVar19);
        break;
      }
    }
    if (puStack76 != 0x0) {
      ppcVar1 = (code **)*puStack76;
      (**ppcVar1)(uVar15,uVar6,puVar9,0x1);
    }
  }
  for (iStack20 = 0x7a; iStack20 < 0x7d; iStack20 += 0x1) {
    if ((iStack20 * 0x2 + uStack14) != 0x0) {
      paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
      uVar6 = (paStack6 >> 0x10);
      uVar2 = (astruct_122 *)paStack6;
      if ((uVar6 | uVar2) == 0x0) {
        paStack6 = (astruct_99 *)0x0;
      }
      else {
        paStack6->field_0x0 = 0x389a;
        uVar2->field_0x2 = 0x1008;
        uVar2->field_0x4 = iStack20;
        paStack6->field_0x0 = 0x9ec8;
        uVar2->field_0x2 = 0x1030;
      }
      ppcVar1 = (code **)(*param_4 + 0x8);
      (**ppcVar1)(0x0,uVar20,uVar13,paStack6,(paStack6 >> 0x10));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_9adc(param_1: u16,param_2: u16,param_3: *mut u32,param_4: u32,param_5: u16,
               param_6: u16)

{
  code **ppcVar1;
  astruct_99 *paVar2;
  let uVar4: u16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  astruct_121 *iVar7;
  astruct_119 *iVar6;
  astruct_99 *paStack6;
  astruct_120 *uVar3;
  
  pass1_1038_53ba(param_4,0x1);
  uVar4 = param_6 | param_5;
  if (uVar4 != 0x0) {
    paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar4 = (paStack6 >> 0x10);
    paVar2 = (astruct_99 *)(paStack6 & 0xffff);
    if ((uVar4 | paVar2) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      iVar7 = (astruct_121 *)paStack6;
      paStack6->field_0x0 = 0x389a;
      iVar7->field_0x2 = 0x1008;
      iVar7->field_0x4 = 0x77;
      paStack6->field_0x0 = 0x9ec8;
      iVar7->field_0x2 = 0x1030;
      paVar2 = paStack6;
    }
    param_5 = paVar2;
    ppcVar1 = (code **)(*param_3 + 0x4);
    (**ppcVar1)(0x1000,param_3,paStack6,(paStack6 >> 0x10));
    uVar4 = extraout_DX;
  }
  pass1_1038_53ba(param_4,0x2);
  uVar4 |= param_5;
  if (uVar4 != 0x0) {
    paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar4 = (paStack6 >> 0x10);
    paVar2 = (astruct_99 *)(paStack6 & 0xffff);
    if ((uVar4 | paVar2) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      iVar6 = (astruct_119 *)paStack6;
      paStack6->field_0x0 = 0x389a;
      iVar6->field_0x2 = 0x1008;
      iVar6->field_0x4 = 0x78;
      paStack6->field_0x0 = 0x9ec8;
      iVar6->field_0x2 = 0x1030;
      paVar2 = paStack6;
    }
    param_5 = paVar2;
    ppcVar1 = (code **)(*param_3 + 0x8);
    (**ppcVar1)(0x1000,param_3,paStack6,(paStack6 >> 0x10));
    uVar4 = extraout_DX_00;
  }
  pass1_1038_53ba(param_4,0x3);
  if ((uVar4 | param_5) != 0x0) {
    paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar4 = (paStack6 >> 0x10);
    uVar3 = (astruct_120 *)paStack6;
    if ((uVar4 | uVar3) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      paStack6->field_0x0 = 0x389a;
      uVar3->field_0x2 = 0x1008;
      uVar3->field_0x4 = 0x75;
      paStack6->field_0x0 = 0x9ec8;
      uVar3->field_0x2 = 0x1030;
    }
    ppcVar1 = (code **)(*param_3 + 0x8);
    (**ppcVar1)(0x1000,param_3,paStack6,(paStack6 >> 0x10));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_9c1c(param_1: u32,param_2: *mut u32,param_3: u32)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let iVar5: i16;
  let in_DX: *mut u8
  let uVar6: u16;
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar7: *mut u16;
  let iStack24: i16;
  let iStack16: i16;
  astruct_99 *paStack6;
  
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,unaff_SS,in_DX,unaff_DI);
  iVar4 = puVar7 + 0xa;
  uVar3 = (puVar7 >> 0x10);
  iVar5 = iVar4;
  pass1_1030_9048(unaff_SS,param_1,0x1,param_3);
  if (iVar5 != 0x0) {
    for (iStack24 = 0x4f; iStack24 < 0x70; iStack24 += 0x1) {
      if ((iStack24 * 0x2 + iVar4) != 0x0) {
        paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        uVar6 = (paStack6 >> 0x10);
        uVar2 = paStack6;
        if ((uVar6 | uVar2) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          paStack6->field_0x0 = 0x389a;
          (uVar2 + 0x2) = 0x1008;
          (uVar2 + 0x4) = iStack24;
          paStack6->field_0x0 = 0x9ec8;
          (uVar2 + 0x2) = 0x1030;
        }
        ppcVar1 = (code **)(*param_2 + 0x8);
        (**ppcVar1)(0x1000,param_2,paStack6,(paStack6 >> 0x10));
      }
    }
  }
  for (iStack16 = 0x7d; iStack16 < 0x80; iStack16 += 0x1) {
    if ((iStack16 * 0x2 + iVar4) != 0x0) {
      paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
      uVar6 = (paStack6 >> 0x10);
      uVar2 = paStack6;
      if ((uVar6 | uVar2) == 0x0) {
        paStack6 = (astruct_99 *)0x0;
      }
      else {
        paStack6->field_0x0 = 0x389a;
        (uVar2 + 0x2) = 0x1008;
        (uVar2 + 0x4) = iStack16;
        paStack6->field_0x0 = 0x9ec8;
        (uVar2 + 0x2) = 0x1030;
      }
      ppcVar1 = (code **)(*param_2 + 0x8);
      (**ppcVar1)(0x1000,param_2,paStack6,(paStack6 >> 0x10));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_9d42(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: *mut u32
               ,param_6: u32)

{
  let puVar1: u32;
  let uVar2: u32;
  code **ppcVar3;
  let puVar4: *mut u16;
  let puVar5: *mut u8;
  let puVar6: *mut u8;
  let extraout_DX: u16;
  let uVar8: u16;
  let iVar9: i16;
  let uVar10: u16;
  let local_a6: [u8;4];
  let uStack162: u32;
  let uStack158: u32;
  let iStack154: i16;
  let local_98: u32;
  let uStack12: u32;
  let uStack8: u32;
  let iStack4: i16;
  let uVar7: u32;
  
  uVar10 = (param_6 >> 0x10);
  if ((param_6 + 0x206) == 0x0) {
    iStack4 = (param_6 + 0x204);
    puVar4 = pass1_1000_4906((astruct_20 *)CONCAT22(param_1,&local_98),(WNDCLASS16 *)0x0,
                             0x94);
    uVar7 = ZEXT24(puVar4);
    iStack154 = 0x11;
    do {
      empty_1038_540a();
      uVar10 = uVar7;
      (&local_98 + iStack154) = uVar10;
      (&local_98 + iStack154 * 0x4 + 0x2) = param_2;
      iStack154 += 0x1;
    } while (iStack154 < 0x25);
    empty_1038_540a();
    uStack158 = CONCAT22(param_2,uVar10);
    pass1_1008_5784(CONCAT22(param_1,local_a6),param_5);
    uVar7 = (_PTR_LOOP_1050_65e2 + 0x52);
    while( true ) {
      puVar5 = local_a6;
      pass1_1008_5b12(puVar5,param_1);
      uVar8 = extraout_DX | puVar5;
      if (uVar8 == 0x0) break;
      puVar6 = puVar5;
      pass1_1030_4bbe(param_1,uVar8,uVar7,(puVar5 + 0x4));
      if (iStack4 == 0x0) {
        for (iStack154 = 0x11; iStack154 < 0x25; iStack154 += 0x1) {
          iVar9 = iStack154 * 0x4;
          if ((*(long *)(puVar6 + iVar9) != 0x0) &&
             (uVar2 = (&local_98)[iStack154], puVar1 = (puVar6 + iVar9),
             uVar2 <= *puVar1 && *puVar1 != uVar2)) {
            puVar1 = (puVar6 + iVar9);
            if (uStack158 <= *puVar1 && *puVar1 != uStack158) goto LAB_1030_9e17;
            uStack158 -= *(long *)(puVar6 + iVar9);
          }
        }
      }
      else {
        puVar1 = (puVar6 + 0x8c);
        if ((uStack12 <= *puVar1 && *puVar1 != uStack12) ||
           (puVar1 = (puVar6 + 0x90), uStack8 <= *puVar1 && *puVar1 != uStack8))
        {
LAB_1030_9e17:
          ppcVar3 = (code **)(*param_5 + 0xc);
          (**ppcVar3)(0x1008,(char)param_5,(param_5 >> 0x10),puVar5,
                      extraout_DX);
          uStack162 = 0x0;
        }
      }
    }
  }
  return;
}



fn pass1_1030_9e9c(param_1: *mut u16,param_2: u8) -> u16

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



fn pass1_1030_9ecc(param_1: *mut u32,param_2: u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x0;
  (param_1 + 0x4) = param_2;
  (param_1 + 0x8) = 0x0;
  return;
}



fn pass1_1030_9ef2(param_1: *mut u32) -> u16

{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u32;
  
  if (*param_1 != 0x0) {
    uVar3 = struct_op_1030_73a8(*param_1);
    uVar2 = (uVar3 >> 0x10);
    iVar1 = (uVar3 + 0xc);
    if (((iVar1 != 0x5) && (iVar1 != 0x9)) && ((uVar3 + 0x12) < 0x5)) {
      return 0x0;
    }
    pass1_1030_9f64(param_1);
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_9f40(param_1: u32,param_2: u16,param_3: u16,param_4: u8)
{
  let uVar1: u16;
  
  uVar1 = pass1_1008_c646(_PTR_LOOP_1050_06e0,
                          CONCAT22(param_2,(_PTR_LOOP_1050_06e0 >> 0x10)),
                          param_3);
  (param_1 + 0x8) = uVar1;
  pass1_1030_9f7a((u16 *)param_1,uVar1,param_3,param_4);
  return;
}



fn pass1_1030_9f64(param_1: *mut u32)
{
  (param_1 + 0x8) = 0x0;
  *param_1 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_9f7a(param_1: *mut u16,param_2: u16,param_3: u16,param_4: u8)
{
  let uVar1: u32;
  let BVar2: bool;
  let puVar3: u32;
  let extraout_DX: u16;
  let uVar4: u16;
  let uVar5: u16;
  let local_130: [u8;120];
  let uStack16: u32;
  let uStack12: u32;
  let local_8: u32;
  let iStack4: i16;
  
  pass1_1008_3e38((u16 *)CONCAT22(param_3,&local_8));
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,param_2,0x28);
  if (BVar2 != 0x0) {
    iStack4 = 0x1;
  }
  puVar3 = &local_8;
  pass1_1030_a278(param_1,(u16 *)CONCAT22(param_3,puVar3),puVar3,param_3,
                  param_4);
  if (puVar3 != 0x0) {
    uVar5 = (param_1 >> 0x10);
    uVar4 = param_1;
    uVar1 = (uVar4 + 0x4);
    uStack12 = (uVar1 + 0x8);
    uVar1 = (uVar4 + 0x4);
    struct_op_1028_87f0(param_3,param_4,(astruct_97 *)CONCAT22(param_3,local_130),0x0,0x0,
                        param_2,&local_8,param_3,(uVar1 + 0x4),uStack12);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_3,local_130));
    pass1_1028_b58e(uStack16);
    *param_1 = uStack16;
    (uVar4 + 0x2) = extraout_DX;
    if (0x0 < iStack4) {
      pass1_1030_a044(param_3,extraout_DX,param_4,uVar4,uVar5,
                      (u16 *)CONCAT22(param_3,&local_8),uStack12);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_a044(param_1: u16,param_2: u16,param_3: u8,param_4: u16,param_5: u16,
               param_6: *mut u16,param_7: u32)

{
  code **ppcVar1;
  let puVar2: *mut u16;
  let puVar3: *mut u8;
  let iVar4: i16;
  let uVar5: u32;
  let uVar6: u16;
  let extraout_DX: u16;
  let uVar7: u16;
  let puVar8: u32;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let local_17e: u16;
  let uStack380: u16;
  let iStack90: i16;
  let puStack78: u32;
  let uStack70: u16;
  let iStack68: i16;
  let uStack66: u32;
  let puStack62: u32;
  u8 local_3a [0xc];
  let local_2e: u32;
  let uStack42: u16;
  let iStack40: i16;
  let uStack38: u16;
  let local_24: i16;
  let local_22: i16;
  let uStack32: u32;
  let uStack28: u32;
  let uStack24: u32;
  let puStack20: *mut u16;
  let uStack18: u16;
  let iStack16: i16;
  let iStack14: i16;
  let uStack12: u32;
  let local_8: u16;
  let local_6: i16;
  let local_4: i16;
  
  puVar2 = &local_8;
  pass1_1008_3eb4(param_6,(u16 *)CONCAT22(param_1,puVar2),
                  (u16 *)CONCAT22(param_1,&local_6),
                  (u16 *)CONCAT22(param_1,&local_4));
  pass1_1030_627e(param_1,puVar2,param_2,_PTR_LOOP_1050_5740,param_6,param_7)
  ;
  puStack20 = puVar2;
  uStack18 = param_2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,puVar2,param_2);
  uStack24 = CONCAT22(param_2,puVar2);
  uStack28 = (puVar2 + 0x17);
  uVar5 = (uStack28 + 0x4);
  uStack32 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_7,(param_7 >> 0x10));
  iStack40 = uVar5;
  uStack38 = param_2;
  puVar8 = pass1_1030_5b5c(iStack40,param_2);
  uVar6 = (puVar8 >> 0x10);
  local_2e = *puVar8;
  uStack42 = (puVar8 + 0x4);
  puStack78 = &local_2e;
  pass1_1008_3e94((u16 *)CONCAT22(param_1,&local_2e),
                  (u16 *)CONCAT22(param_1,&local_24),
                  (u16 *)CONCAT22(param_1,&local_22));
  iStack14 = local_4 + 0x1;
  uStack12 = CONCAT22(local_4 + -0x1,local_6 - 0x1U);
  iStack16 = local_6 + 0x1;
  if (local_4 + -0x1 < 0x0) {
    uStack12 = (local_6 - 0x1U);
  }
  if (local_22 <= iStack14) {
    iStack14 = local_22 + -0x1;
  }
  if (uStack12 < 0x0) {
    uStack12 &= 0xffff0000;
  }
  if (local_24 <= iStack16) {
    iStack16 = local_24 + -0x1;
  }
  pass1_1008_6c90((u16 *)CONCAT22(param_1,local_3a));
  uVar7 = 0x1008;
  pass1_1008_6cec((u16 *)CONCAT22(param_1,local_3a),local_8,CONCAT22(iStack14,iStack16)
                  ,local_8,uStack12);
  puVar3 = local_3a;
  pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(param_1,puVar3),param_7,param_1);
  puStack62 = CONCAT22(uVar6,puVar3);
  if ((uVar6 | puVar3) != 0x0) {
    uStack66 = 0x0;
    iStack68 = 0x0;
    for (uStack70 = uStack12; uStack70 <= iStack16; uStack70 += 0x1) {
      for (puStack78 = uStack12._2_2_; puStack78 <= iStack14;
          puStack78 = (puStack78 + 0x1)) {
        ppcVar1 = (code **)(*puStack62 + 0x4);
        iVar4 = iStack68;
        iStack68 = iStack68 + 0x1;
        (**ppcVar1)(uVar7,puStack62,(puStack62 >> 0x10));
        uStack66 = CONCAT22(extraout_DX,iVar4);
        uStack66._3_1_ = (char)(extraout_DX >> 0x8);
        if (uStack66._3_1_ == '\0') {
          iStack90 = iVar4;
          if (iVar4 == 0x7) {
            pass1_1008_3e76(param_6,local_8,uStack70,puStack78);
            uVar10 = uStack32;
            uVar11 = (uStack32 >> 0x10);
            uVar9 = 0x6;
          }
          else {
            if (iVar4 == 0x8) {
              pass1_1008_3e76(param_6,local_8,uStack70,puStack78);
              uVar10 = uStack32;
              uVar11 = (uStack32 >> 0x10);
              uVar9 = 0x7;
            }
            else {
              if (iVar4 != 0x9) goto LAB_1030_a1d0;
              pass1_1008_3e76(param_6,local_8,uStack70,puStack78);
              uVar10 = uStack32;
              uVar11 = (uStack32 >> 0x10);
              uVar9 = 0x8;
            }
          }
          uVar7 = SUB42(&USHORT_1050_1028,0x0);
          struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,&local_17e),
                              0x0,0x0,uVar9,param_6,
                              (param_6 >> 0x10),CONCAT22(uVar11,uVar10),
                              param_7);
          fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,&local_17e));
          local_17e = 0x389a;
          uStack380 = 0x1008;
        }
LAB_1030_a1d0:
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_a278(param_1: *mut u16,param_2: *mut u16,param_3: u16,param_4: u16,
               param_5: u8)

{
  let iVar1: i16;
  let uVar2: u32;
  let in_DX: i16;
  let extraout_DX: u16;
  let puVar3: u32;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let local_134: [u8;120];
  let uStack20: u32;
  let uStack16: u32;
  let uStack12: u32;
  let uStack6: u16;
  let uStack4: u16;
  
  uStack4 = 0x1;
  pass1_1030_a39a(param_1,param_2,param_4);
  if (param_3 != 0x0) {
    return;
  }
  uStack6 = param_3;
  pass1_1030_a3ae(param_1,param_2,param_4);
  puVar3 = param_2;
  uVar5 = (param_2 >> 0x10);
  if (param_3 == 0x0) {
    pass1_1030_a57e(param_1,param_2,0x0,in_DX,param_4);
    if (param_3 == 0x0) {
      pass1_1030_a844(param_1,param_2,0x0,in_DX,param_4);
      if (param_3 == 0x0) {
        uStack4 = 0x0;
        goto LAB_1030_a305;
      }
      iVar1 = (puVar3 + 0x1);
    }
    else {
      iVar1 = (puVar3 + 0x1);
    }
    if (iVar1 < 0x1) {
      uStack6 = 0x73;
    }
    else {
      uStack6 = 0x77;
    }
  }
  else {
    if ((puVar3 + 0x1) < 0x1) {
      uStack6 = 0x7a;
    }
    else {
      uStack6 = 0x7f;
    }
  }
LAB_1030_a305:
  if (uStack6 != 0x0) {
    uVar6 = (param_1 >> 0x10);
    uVar4 = param_1;
    uVar2 = (uVar4 + 0x4);
    uStack16 = (uVar2 + 0x8);
    uVar2 = (uVar4 + 0x4);
    struct_op_1028_87f0(param_4,param_5,(astruct_97 *)CONCAT22(param_4,local_134),0x0,0x0,
                        uStack6,puVar3,uVar5,(uVar2 + 0x4),uStack16);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_4,local_134));
    uStack12 = uStack20;
    pass1_1028_b58e(uStack20);
    *param_1 = uStack20;
    (uVar4 + 0x2) = extraout_DX;
    if (0x0 < (puVar3 + 0x1)) {
      pass1_1030_a044(param_4,extraout_DX,param_5,uVar4,uVar6,
                      (u16 *)(param_2 & 0xffff | uVar5 << 0x10),uStack16)
      ;
    }
  }
  return;
}



fn pass1_1030_a39a(param_1: u32,param_2: *mut u16,param_3: u16)
{
  pass1_1030_aa18(param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_a3ae(param_1: u32,param_2: *mut u16,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let BVar5: bool;
  let uVar6: u32;
  let puVar7: *mut u8
  let extraout_DX: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let iVar11: i16;
  let uVar12: u16;
  let uVar13: u16;
  let puVar14: u32;
  let puVar15: *mut u16;
  let uVar16: u16;
  let uStack44: u32;
  let local_28: i16;
  let local_26: i16;
  let local_24: i16;
  let local_22: [u8;6];
  let local_1c: i16;
  let iStack26: i16;
  let lStack22: i32;
  let uStack18: u32;
  let puStack14: u32;
  let uStack10: u16;
  let puStack8: *mut u8
  let iStack6: i16;
  let uStack4: u16;
  
  uStack4 = 0x0;
  iStack6 = (param_2 + 0x4);
  puVar14 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x45);
  puVar7 = (uchar *)(puVar14 >> 0x10);
  uVar3 = puVar14;
  uVar12 = (param_1 >> 0x10);
  uVar10 = param_1;
  uStack10 = uVar3;
  puStack8 = puVar7;
  pass1_1038_4e78(uVar3,puVar7,(uVar10 + 0x4),puVar14);
  puStack14 = CONCAT22(puVar7,uVar3);
  ppcVar1 = (code **)(*puStack14 + 0x10);
  uVar16 = uVar3;
  (**ppcVar1)(&PTR_LOOP_1050_1038,uVar3,puVar7);
  uStack18 = CONCAT22(extraout_DX,uVar3);
  uVar2 = (uVar10 + 0x4);
  lStack22 = *(long *)(uVar2 + 0x8);
  pass1_1008_3e38((u16 *)CONCAT22(param_3,&local_1c));
  puVar15 = pass1_1008_3e38((u16 *)CONCAT22(param_3,local_22));
  uStack44 = 0x0;
  uVar8 = (puVar15 >> 0x10);
  do {
    if (uStack18 <= uStack44) {
LAB_1030_a4e7:
      if (puStack14 != 0x0) {
        ppcVar1 = (code **)*puStack14;
        (**ppcVar1)(0x1008,puStack14,(char)(puStack14 >> 0x10),0x1,uVar16,
                    puVar7,puStack14,puStack14);
      }
      return;
    }
    uVar6 = uStack18;
    pass1_1030_1d58(puStack14);
    uVar9 = uVar8 | uVar6;
    if (uVar9 != 0x0) {
      pass1_1008_3f62((u16 *)CONCAT22(param_3,&local_1c),
                      (u16 *)CONCAT22(uVar8,uVar6 + 0xc));
      pass1_1008_3eb4((u16 *)CONCAT22(param_3,&local_1c),
                      (u16 *)CONCAT22(param_3,&local_28),
                      (u16 *)CONCAT22(param_3,&local_26),
                      (u16 *)CONCAT22(param_3,&local_24));
      uVar9 = uVar8;
      if ((local_28 == iStack6) &&
         (uVar2 = (uVar10 + 0x4),
         uVar13 = (uVar2 >> 0x10), iVar11 = uVar2,
         uVar2 = (iVar11 + 0x4),
         uVar4 = pass1_1030_addc(uVar10,uVar12,(u16 *)CONCAT22(param_3,&local_1c),
                                 uVar2,(uVar2 >> 0x10),
                                 (iVar11 + 0x8),&local_1c,uVar8,param_3),
         uVar9 = uVar8, uVar4 != 0x0)) {
        pass1_1008_3f62((u16 *)CONCAT22(param_3,local_22),
                        (u16 *)CONCAT22(param_3,&local_1c));
        iStack26 = local_26 + -0x1;
        BVar5 = pass1_1030_ad22(uVar10,uVar12,(u16 *)CONCAT22(param_3,&local_1c),
                                lStack22,&local_1c,uVar8,param_3);
        if (BVar5 == 0x0) {
          iStack26 = local_26 + 0x1;
          BVar5 = pass1_1030_ad22(uVar10,uVar12,(u16 *)CONCAT22(param_3,&local_1c),
                                  lStack22,&local_1c,uVar8,param_3);
          if (BVar5 == 0x0) {
            iStack26 = local_26;
            local_1c = local_24 + -0x1;
            BVar5 = pass1_1030_ad22(uVar10,uVar12,(u16 *)CONCAT22(param_3,&local_1c),
                                    lStack22,&local_1c,uVar8,param_3);
            if (BVar5 == 0x0) {
              local_1c = local_24 + 0x1;
              BVar5 = pass1_1030_ad22(uVar10,uVar12,(u16 *)CONCAT22(param_3,&local_1c),
                                      lStack22,&local_1c,uVar8,param_3);
              uVar9 = uVar8;
              if (BVar5 == 0x0) goto LAB_1030_a45b;
            }
          }
        }
        pass1_1008_3f62(param_2,(u16 *)CONCAT22(param_3,local_22));
        uStack4 = 0x1;
        goto LAB_1030_a4e7;
      }
    }
LAB_1030_a45b:
    uStack44 += 0x1;
    uVar8 = uVar9;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_a57e(param_1: u32,param_2: *mut u16,param_3: i16,param_4: i16,param_5: u16)
{
  let uVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let uVar4: u16;
  let piVar5: *mut i16;
  let uVar6: u32;
  let puVar7: *mut u8
  let extraout_DX: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let iVar12: i16;
  let puVar13: u32;
  let uVar14: u16;
  let uVar15: u16;
  let uVar16: u16;
  let uVar17: u16;
  let puVar18: u32;
  let uVar19: u32;
  let uVar20: u8;
  let uStack40: u32;
  let local_1c: [u8;2];
  let local_1a: i16;
  let local_18: i16;
  let local_16: i16;
  let iStack20: i16;
  let uStack16: u32;
  let uStack12: u16;
  let puStack10: *mut u8
  let iStack8: i16;
  let iStack6: i16;
  let uStack4: u16;
  
  uStack4 = 0x0;
  uVar14 = (param_1 >> 0x10);
  uVar10 = param_1;
  pass1_1038_53ba((uVar10 + 0x4),0x1);
  if ((param_4 != 0x0) || (param_3 != 0x0)) {
    iStack6 = (param_2 + 0x4);
    iStack8 = 0x8 - (iStack6 == 0x0);
    puVar18 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,iStack8);
    puVar7 = (uchar *)(puVar18 >> 0x10);
    uVar8 = puVar18;
    uStack12 = uVar8;
    puStack10 = puVar7;
    pass1_1038_4e78(uVar8,puVar7,(uVar10 + 0x4),puVar18);
    uStack16 = CONCAT22(puVar7,uVar8);
    uVar17 = 0x1008;
    pass1_1008_3e38((u16 *)CONCAT22(param_5,&local_16));
    uVar3 = (uVar10 + 0x4);
    uVar1 = (uVar3 + 0x8);
    uVar15 = (uStack16 >> 0x10);
    uVar11 = SUB42(uStack16,0x0);
    ppcVar2 = (code **)(*uStack16 + 0x10);
    uVar6 = uVar1;
    (**ppcVar2)(0x1008,uVar11,uVar15);
    uVar6 = uVar6 & 0xffff | extraout_DX << 0x10;
    uVar8 = extraout_DX;
    for (uStack40 = 0x0; uStack40 < uVar6; uStack40 += 0x1) {
      uVar19 = uVar6;
      pass1_1030_1d58(uStack16);
      uVar9 = uVar8 | uVar19;
      if (uVar9 != 0x0) {
        uVar9 = uVar8;
        pass1_1008_3f62((u16 *)CONCAT22(param_5,&local_16),
                        (u16 *)CONCAT22(uVar8,uVar19 + 0xc));
        uVar17 = 0x1008;
        pass1_1008_3eb4((u16 *)CONCAT22(param_5,&local_16),
                        (u16 *)CONCAT22(param_5,local_1c),
                        (u16 *)CONCAT22(param_5,&local_1a),
                        (u16 *)CONCAT22(param_5,&local_18));
        uVar3 = (uVar10 + 0x4);
        uVar16 = (uVar3 >> 0x10);
        iVar12 = uVar3;
        uVar3 = (iVar12 + 0x4);
        uVar4 = pass1_1030_addc(uVar10,uVar14,(u16 *)CONCAT22(param_5,&local_16),
                                uVar3,(uVar3 >> 0x10),
                                (iVar12 + 0x8),&local_16,uVar9,param_5);
        if (uVar4 == 0x0) goto LAB_1030_a660;
        uVar19 = struct_op_1030_73a8(uVar19 & 0xffff | uVar8 << 0x10);
        uVar9 = (uVar19 >> 0x10);
        iVar12 = (uVar19 + 0xc);
        if (0x5 < iVar12 - 0x7aU) goto LAB_1030_a660;
        uVar17 = 0x1030;
        switch(iVar12) {
        default:
          iStack20 = local_1a + -0x1;
          piVar5 = &local_16;
          pass1_1030_ad86(uVar10,uVar14,(u16 *)CONCAT22(param_5,piVar5),uVar1,param_5,
                          uVar9);
          if (piVar5 != (i16 *)0x0) goto LAB_1030_a7df;
          iStack20 = local_1a + 0x1;
          piVar5 = &local_16;
          pass1_1030_ad86(uVar10,uVar14,(u16 *)CONCAT22(param_5,piVar5),uVar1,param_5,
                          uVar9);
          if (piVar5 == (i16 *)0x0) {
            iStack20 = local_1a;
            local_16 = local_18 + -0x1;
            piVar5 = &local_16;
            pass1_1030_ad86(uVar10,uVar14,(u16 *)CONCAT22(param_5,piVar5),uVar1,param_5
                            ,uVar9);
            goto joined_r0x1030a722;
          }
LAB_1030_a748:
          pass1_1008_3f62(param_2,(u16 *)CONCAT22(param_5,&local_16));
          break;
        case 0x7b:
        case 0x7e:
          iStack20 = local_1a + -0x1;
          piVar5 = &local_16;
          pass1_1030_ad86(uVar10,uVar14,(u16 *)CONCAT22(param_5,piVar5),uVar1,param_5,
                          uVar9);
          if (piVar5 == (i16 *)0x0) {
            iStack20 = local_1a + 0x1;
            goto LAB_1030_a730;
          }
          pass1_1008_3f62(param_2,(u16 *)CONCAT22(param_5,&local_16));
          if (uStack16 == 0x0) {
            return;
          }
          uVar17 = (uStack16 >> 0x10);
          puVar13 = uStack16;
          uVar20 = (u8)(uStack16 >> 0x10);
          goto LAB_1030_a6ea;
        case 0x7c:
        case 0x7d:
          local_16 = local_18 + -0x1;
          piVar5 = &local_16;
          pass1_1030_ad86(uVar10,uVar14,(u16 *)CONCAT22(param_5,piVar5),uVar1,param_5,
                          uVar9);
joined_r0x1030a722:
          if (piVar5 == (i16 *)0x0) {
            local_16 = local_18 + 0x1;
LAB_1030_a730:
            piVar5 = &local_16;
            pass1_1030_ad86(uVar10,uVar14,(u16 *)CONCAT22(param_5,piVar5),uVar1,param_5
                            ,uVar9);
            if (piVar5 != (i16 *)0x0) goto LAB_1030_a748;
            goto LAB_1030_a660;
          }
LAB_1030_a7df:
          pass1_1008_3f62(param_2,(u16 *)CONCAT22(param_5,&local_16));
        }
        puVar13 = uStack16;
        if ((uStack16._2_2_ | puVar13) != 0x0) {
          uVar17 = (uStack16 >> 0x10);
          uVar20 = (u8)(uStack16 >> 0x10);
LAB_1030_a6ea:
          ppcVar2 = (code **)*puVar13;
          (**ppcVar2)(0x1008,puVar13,uVar20,0x1,uVar11,uVar15,uStack16,uStack16);
        }
        return;
      }
LAB_1030_a660:
      uVar8 = uVar9;
    }
    if (uStack16 != 0x0) {
      ppcVar2 = (code **)*uStack16;
      (**ppcVar2)(uVar17,uStack16,(char)(uStack16 >> 0x10),0x1,uVar11,uVar15,
                  uStack16,uStack16);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_a844(param_1: u32,param_2: *mut u16,param_3: i16,param_4: i16,param_5: u16)
{
  let iVar1: i16;
  let uVar2: u32;
  code **ppcVar3;
  let uVar4: u16;
  let uVar5: u16;
  let piVar6: *mut i16;
  let puVar7: u32;
  let extraout_DX: u16;
  let uVar9: u16;
  let uVar10: u16;
  astruct_426 *uVar8;
  astruct_427 *iVar8;
  let iVar11: i16;
  let uVar12: u16;
  let uVar13: u16;
  let puVar14: *mut u16;
  let uVar15: u32;
  let uStack34: u32;
  let local_1c: i16;
  let local_1a: i16;
  let local_18: i16;
  let local_16: i16;
  let iStack20: i16;
  let uStack16: u16;
  let lStack14: i32;
  let uStack10: u32;
  let puStack6: u32;
  
  uVar12 = (param_1 >> 0x10);
  uVar8 = (astruct_426 *)param_1;
  pass1_1038_53ba(uVar8->field_0x4,0x1);
  if ((param_4 != 0x0) || (param_3 != 0x0)) {
    uVar15 = uVar8->field_0x4;
    uVar13 = (uVar15 >> 0x10);
    iVar8 = (astruct_427 *)uVar15;
    puVar7 = iVar8->field_0xc;
    ppcVar3 = (code **)(*puVar7 + 0x10);
    puStack6 = puVar7;
    (**ppcVar3)(&PTR_LOOP_1050_1038,puVar7,
                (&iVar8->field_0xc + 0x2));
    uStack10 = puVar7 & 0xffff | extraout_DX << 0x10;
    uVar15 = uVar8->field_0x4;
    lStack14 = *(long *)(uVar15 + 0x8);
    uStack16 = 0x0;
    puVar14 = pass1_1008_3e38((u16 *)CONCAT22(param_5,&local_16));
    uVar9 = (puVar14 >> 0x10);
    iVar1 = (param_2 + 0x4);
    for (uStack34 = 0x0; uStack34 < uStack10; uStack34 += 0x1) {
      uVar15 = pass1_1030_1d7c(uStack10,uVar9,puStack6);
      uVar4 = (uVar15 >> 0x10);
      uVar10 = uVar4 | uVar15;
      uVar9 = uVar10;
      if ((uVar10 != 0x0) &&
         (uVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar15 + 0xc),
                                  0x46), uVar9 = uVar10, uVar4 != 0x0)) {
        pass1_1030_1d58(puStack6);
        uVar9 = uVar10 | uVar4;
        if ((uVar10 | uVar4) != 0x0) {
          pass1_1008_3f62((u16 *)CONCAT22(param_5,&local_16),
                          (u16 *)CONCAT22(uVar10,uVar4 + 0xc));
          pass1_1008_3eb4((u16 *)CONCAT22(param_5,&local_16),
                          (u16 *)CONCAT22(param_5,&local_1c),
                          (u16 *)CONCAT22(param_5,&local_1a),
                          (u16 *)CONCAT22(param_5,&local_18));
          uVar9 = uVar10;
          if ((iVar1 == local_1c) &&
             (uVar15 = uVar8->field_0x4, uVar13 = (uVar15 >> 0x10),
             iVar11 = uVar15, uVar2 = (iVar11 + 0x4),
             uVar5 = pass1_1030_addc(uVar8,uVar12,
                                     (u16 *)CONCAT22(param_5,&local_16),uVar2,
                                     (uVar2 >> 0x10),
                                     (iVar11 + 0x8),&local_16,uVar10,
                                     param_5), uVar9 = uVar10, uVar5 != 0x0)) {
            iStack20 = local_1a + -0x1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar8,uVar12,(u16 *)CONCAT22(param_5,piVar6),
                            lStack14,param_5,uVar10);
            if (piVar6 != (i16 *)0x0) {
LAB_1030_a98e:
              pass1_1008_3f62(param_2,(u16 *)CONCAT22(param_5,&local_16));
              return;
            }
            iStack20 = local_1a + 0x1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar8,uVar12,(u16 *)CONCAT22(param_5,piVar6),
                            lStack14,param_5,uVar10);
            if (piVar6 != (i16 *)0x0) goto LAB_1030_a98e;
            iStack20 = local_1a;
            local_16 = local_18 + -0x1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar8,uVar12,(u16 *)CONCAT22(param_5,piVar6),
                            lStack14,param_5,uVar10);
            if (piVar6 != (i16 *)0x0) goto LAB_1030_a98e;
            local_16 = local_18 + 0x1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar8,uVar12,(u16 *)CONCAT22(param_5,piVar6),
                            lStack14,param_5,uVar10);
            uVar9 = uVar10;
            if (piVar6 != (i16 *)0x0) goto LAB_1030_a98e;
          }
        }
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_aa18(param_1: u32,param_2: *mut u16,param_3: u16)
{
  let uVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let uVar4: u16;
  let BVar5: bool;
  let uVar6: u32;
  let puVar7: *mut u8
  let extraout_DX: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let iVar12: i16;
  let puVar13: u32;
  let uVar14: u16;
  let uVar15: u16;
  let uVar16: u16;
  let uVar17: u16;
  let puVar18: u32;
  let uVar19: u32;
  let uVar20: u8;
  let uStack38: u32;
  let local_1a: [u8;2];
  let local_18: i16;
  let local_16: i16;
  let local_14: i16;
  let iStack18: i16;
  let uStack14: u32;
  let uStack10: u16;
  let puStack8: *mut u8
  let iStack6: i16;
  let iStack4: i16;
  
  iStack4 = (param_2 + 0x4);
  iStack6 = 0x8 - (iStack4 == 0x0);
  puVar18 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,iStack6);
  puVar7 = (uchar *)(puVar18 >> 0x10);
  uVar8 = puVar18;
  uVar14 = (param_1 >> 0x10);
  uVar10 = param_1;
  uStack10 = uVar8;
  puStack8 = puVar7;
  pass1_1038_4e78(uVar8,puVar7,(uVar10 + 0x4),puVar18);
  uStack14 = CONCAT22(puVar7,uVar8);
  uVar17 = 0x1008;
  pass1_1008_3e38((u16 *)CONCAT22(param_3,&local_14));
  uVar3 = (uVar10 + 0x4);
  uVar1 = (uVar3 + 0x8);
  uVar15 = (uStack14 >> 0x10);
  uVar11 = SUB42(uStack14,0x0);
  ppcVar2 = (code **)(*uStack14 + 0x10);
  uVar6 = uVar1;
  (**ppcVar2)(0x1008,uVar11,uVar15);
  uVar6 = uVar6 & 0xffff | extraout_DX << 0x10;
  uStack38 = 0x0;
  uVar8 = extraout_DX;
  while( true ) {
    if (uVar6 <= uStack38) {
      if (uStack14 != 0x0) {
        ppcVar2 = (code **)*uStack14;
        (**ppcVar2)(uVar17,uStack14,(char)(uStack14 >> 0x10),0x1,uVar11,uVar15
                    ,uStack14,uStack14);
      }
      return;
    }
    uVar19 = uVar6;
    pass1_1030_1d58(uStack14);
    uVar9 = uVar8 | uVar19;
    if (uVar9 != 0x0) break;
LAB_1030_aadc:
    uStack38 += 0x1;
    uVar8 = uVar9;
  }
  uVar9 = uVar8;
  pass1_1008_3f62((u16 *)CONCAT22(param_3,&local_14),
                  (u16 *)CONCAT22(uVar8,uVar19 + 0xc));
  uVar17 = 0x1008;
  pass1_1008_3eb4((u16 *)CONCAT22(param_3,&local_14),
                  (u16 *)CONCAT22(param_3,local_1a),
                  (u16 *)CONCAT22(param_3,&local_18),
                  (u16 *)CONCAT22(param_3,&local_16));
  uVar3 = (uVar10 + 0x4);
  uVar16 = (uVar3 >> 0x10);
  iVar12 = uVar3;
  uVar3 = (iVar12 + 0x4);
  uVar4 = pass1_1030_addc(uVar10,uVar14,(u16 *)CONCAT22(param_3,&local_14),
                          uVar3,(uVar3 >> 0x10),
                          (iVar12 + 0x8),&local_14,uVar9,param_3);
  if (uVar4 == 0x0) goto LAB_1030_aadc;
  uVar19 = struct_op_1030_73a8(uVar19 & 0xffff | uVar8 << 0x10);
  uVar9 = (uVar19 >> 0x10);
  iVar12 = (uVar19 + 0xc);
  if (0x5 < iVar12 - 0x7aU) goto LAB_1030_aadc;
  uVar17 = 0x1030;
  switch(iVar12) {
  default:
    iStack18 = local_18 + -0x1;
    BVar5 = pass1_1030_acbe(uVar10,uVar14,(u16 *)CONCAT22(param_3,&local_14),uVar1,
                            &local_14,uVar9,param_3);
    if (BVar5 != 0x0) goto LAB_1030_ac5b;
    iStack18 = local_18 + 0x1;
    BVar5 = pass1_1030_acbe(uVar10,uVar14,(u16 *)CONCAT22(param_3,&local_14),uVar1,
                            &local_14,uVar9,param_3);
    if (BVar5 == 0x0) {
      iStack18 = local_18;
      local_14 = local_16 + -0x1;
      BVar5 = pass1_1030_acbe(uVar10,uVar14,(u16 *)CONCAT22(param_3,&local_14),uVar1,
                              &local_14,uVar9,param_3);
      goto joined_r0x1030ab9e;
    }
LAB_1030_abc4:
    pass1_1008_3f62(param_2,(u16 *)CONCAT22(param_3,&local_14));
    break;
  case 0x7b:
  case 0x7e:
    iStack18 = local_18 + -0x1;
    BVar5 = pass1_1030_acbe(uVar10,uVar14,(u16 *)CONCAT22(param_3,&local_14),uVar1,
                            &local_14,uVar9,param_3);
    if (BVar5 == 0x0) {
      iStack18 = local_18 + 0x1;
      goto LAB_1030_abac;
    }
    pass1_1008_3f62(param_2,(u16 *)CONCAT22(param_3,&local_14));
    if (uStack14 == 0x0) {
      return;
    }
    uVar17 = (uStack14 >> 0x10);
    puVar13 = uStack14;
    uVar20 = (u8)(uStack14 >> 0x10);
    goto LAB_1030_ab66;
  case 0x7c:
  case 0x7d:
    local_14 = local_16 + -0x1;
    BVar5 = pass1_1030_acbe(uVar10,uVar14,(u16 *)CONCAT22(param_3,&local_14),uVar1,
                            &local_14,uVar9,param_3);
joined_r0x1030ab9e:
    if (BVar5 == 0x0) {
      local_14 = local_16 + 0x1;
LAB_1030_abac:
      BVar5 = pass1_1030_acbe(uVar10,uVar14,(u16 *)CONCAT22(param_3,&local_14),uVar1,
                              &local_14,uVar9,param_3);
      if (BVar5 != 0x0) goto LAB_1030_abc4;
      goto LAB_1030_aadc;
    }
LAB_1030_ac5b:
    pass1_1008_3f62(param_2,(u16 *)CONCAT22(param_3,&local_14));
  }
  puVar13 = uStack14;
  if ((uStack14._2_2_ | puVar13) != 0x0) {
    uVar17 = (uStack14 >> 0x10);
    uVar20 = (u8)(uStack14 >> 0x10);
LAB_1030_ab66:
    ppcVar2 = (code **)*puVar13;
    (**ppcVar2)(0x1008,puVar13,uVar20,0x1,uVar11,uVar15,uStack14,uStack14);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool 
pass1_1030_acbe(param_1: u16,param_2: u16,param_3: *mut u16,param_4: i32,param_5: u16,
               param_6: u16,param_7: u16)

{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u32;
  
  pass1_1030_627e(param_7,param_5,param_6,_PTR_LOOP_1050_5740,param_3,param_4);
  uVar2 = param_6 | param_5;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5,param_6);
    if ((uVar2 | param_5) != 0x0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(uVar2,param_5));
      if ((uVar3 != 0x0) &&
         ((iVar1 = (uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool 
pass1_1030_ad22(param_1: u16,param_2: u16,param_3: *mut u16,param_4: i32,param_5: u16,
               param_6: u16,param_7: u16)

{
  let BVar1: bool;
  let uVar2: u16;
  let uVar3: u32;
  
  pass1_1030_627e(param_7,param_5,param_6,_PTR_LOOP_1050_5740,param_3,param_4);
  uVar2 = param_6 | param_5;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5,param_6);
    if ((uVar2 | param_5) != 0x0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(uVar2,param_5));
      if (uVar3 != 0x0) {
        BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar3 + 0xc),0x46
                               );
        return BVar1;
      }
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_ad86(param_1: u16,param_2: u16,param_3: *mut u16,param_4: i32,param_5: u16,
               param_6: u16)

{
  let uVar1: u32;
  let puVar2: u32;
  let cStack17: u8;
  let local_a: u32;
  let iStack6: i16;
  
  puVar2 = &local_a;
  pass1_1030_64ce(param_5,puVar2,param_6,_PTR_LOOP_1050_5740,param_3,param_4,
                  CONCAT22(param_5,puVar2));
  uVar1 = *puVar2;
  cStack17 = (char)(uVar1 >> 0x18);
  if (cStack17 == '\0') {
    iStack6 = uVar1;
    if (((0x0 < iStack6) && (!SBORROW2(iStack6,0x1))) &&
       ((iStack6 == 0x3 || iStack6 + -0x2 < 0x1 ||
        ((0x3 < iStack6 + -0x3 && (iStack6 + -0x7 < 0x2)))))) {
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 
pass1_1030_addc(param_1: u16,param_2: u16,param_3: *mut u16,param_4: u16,
               param_5: u16,param_6: u32,param_7: i16,param_8: u16,param_9: u16)

{
  let puVar1: u32;
  let local_14: i16;
  let local_12: i16;
  let local_10: i16;
  let local_e: i16;
  let local_c: u32;
  let uStack8: u16;
  let iStack6: i16;
  let uStack4: u16;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6,(param_6 >> 0x10));
  iStack6 = param_7;
  uStack4 = param_8;
  puVar1 = pass1_1030_5b5c(param_7,param_8);
  local_c = *puVar1;
  uStack8 = (puVar1 + 0x4);
  pass1_1008_3e94(param_3,(u16 *)CONCAT22(param_9,&local_10),
                  (u16 *)CONCAT22(param_9,&local_e));
  pass1_1008_3e94((u16 *)CONCAT22(param_9,&local_c),
                  (u16 *)CONCAT22(param_9,&local_14),
                  (u16 *)CONCAT22(param_9,&local_12));
  if ((((0x1 < local_e) && (0x1 < local_10)) && (local_e < local_12 + -0x1)) &&
     (local_10 < local_14 + -0x1)) {
    return 0x1;
  }
  return 0x0;
}



fn pass1_1030_ae6c(param_1: *mut u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u8
  let extraout_DX: u16;
  astruct_399 *iVar4;
  let uVar4: u16;
  let puVar5: *mut u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_399 *)param_1;
  *param_1 = 0x389a;
  iVar4->field_0x2 = 0x1008;
  iVar4->field_0x4 = 0x0;
  puVar5 = pass1_1008_3e38((u16 *)
                           (param_1 & 0xffff0000 | &iVar4->field_0x8))
  ;
  puVar3 = (uchar *)(puVar5 >> 0x10);
  uVar2 = 0x0;
  iVar4->field_0xe = 0x0;
  &iVar4->field_0x10 = 0x0;
  *param_1 = 0xb932;
  iVar4->field_0x2 = 0x1030;
  mem_op_1000_179c(0xc,puVar3,0x1000);
  if ((puVar3 | uVar2) == 0x0) {
    &iVar4->field_0x10 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar3,uVar2));
    iVar4->field_0x10 = uVar2;
    iVar4->field_0x12 = extraout_DX;
  }
  uVar1 = &iVar4->field_0x10;
  (uVar1 + 0xa) = 0x0;
  return;
}



fn pass1_1030_aefa(param_1: *mut u16,param_2: u32)
{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u8
  let extraout_DX: u16;
  let uVar4: u16;
  astruct_400 *iVar5;
  let uVar5: u16;
  let puVar6: *mut u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_400 *)param_1;
  *param_1 = 0x389a;
  iVar5->field_0x2 = 0x1008;
  iVar5->field_0x4 = 0x0;
  puVar6 = pass1_1008_3e38((u16 *)
                           (param_1 & 0xffff0000 | &iVar5->field_0x8))
  ;
  puVar3 = (uchar *)(puVar6 >> 0x10);
  iVar5->field_0xe = 0x0;
  &iVar5->field_0x10 = 0x0;
  *param_1 = 0xb932;
  iVar5->field_0x2 = 0x1030;
  iVar5->field_0x4 = (param_2 + 0x4);
  puVar6 = (u16 *)(param_1 & 0xffff0000 | &iVar5->field_0x8);
  pass1_1008_3f62(puVar6,(u16 *)(param_2 & 0xffff0000 | (param_2 + 0xc)));
  uVar2 = puVar6;
  mem_op_1000_179c(0xc,puVar3,0x1000);
  if ((puVar3 | uVar2) == 0x0) {
    uVar2 = 0x0;
    uVar4 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar3,uVar2));
    uVar4 = extraout_DX;
  }
  iVar5->field_0x10 = uVar2;
  iVar5->field_0x12 = uVar4;
  uVar1 = &iVar5->field_0x10;
  (uVar1 + 0xa) = 0x0;
  return;
}



fn pass1_1030_afa6(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u32;
  astruct_614 *iVar5;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_614 *)param_1;
  *param_1 = 0xb932;
  iVar5->field_0x2 = 0x1030;
  if (*(long *)&iVar5->field_0x10 != 0x0) {
    uVar4 = &iVar5->field_0x10;
    (uVar4 + 0xa) = 0x1;
  }
  puVar1 = &iVar5->field_0x10;
  uVar2 = iVar5->field_0x12;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *param_1 = 0x389a;
  iVar5->field_0x2 = 0x1008;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_affc(param_1: u32,param_2: i16,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u16;
  let Bvar4: bool;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u32;
  let uVar8: u32;
  let iStack12: i16;
  let uStack10: u32;
  let local_6: u32;
  
  uVar8 = ZEXT24(&local_6);
  pass1_1030_b718(param_1,param_1._2_2_,
                  (u16 *)(param_1 & 0xffff0000 | (param_1 + 0x8)),
                  CONCAT22(param_3,&local_6),(uchar *)param_1._2_2_,param_2,
                  param_3);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,local_6,(local_6 >> 0x10));
  uStack10 = uVar8 & 0xffff | param_1._2_2_ << 0x10;
  uVar5 = param_1._2_2_ | uVar8;
  if (uVar5 != 0x0) {
    uVar7 = struct_op_1030_73a8(uVar8 & 0xffff | param_1._2_2_ << 0x10);
    uVar5 = (uVar7 >> 0x10);
    iVar1 = (uVar7 + 0xc);
    uVar8 = (iVar1 - 0x16U);
    if ((0x15 < iVar1) && (!SBORROW2(iVar1,0x16))) {
      uVar8 = (iVar1 - 0x17U);
      if (iVar1 - 0x17U != 0x0 && 0x0 < (iVar1 - 0x16U)) {
        uVar8 = (iVar1 - 0x19U);
        if ((iVar1 + -0x18 < 0x1) ||
           (uVar8 = (iVar1 - 0x1aU),
           iVar1 - 0x1aU != 0x0 && 0x0 < (iVar1 - 0x19U))) goto LAB_1030_b064;
      }
      (uVar7 + 0x20) = 0x0;
    }
  }
LAB_1030_b064:
  iStack12 = 0x6;
  do {
    uVar3 = uVar8;
    if (iStack12 == 0x0) {
LAB_1030_b0fc:
      if ((uStack10._2_2_ | uStack10) != 0x0) {
        uVar8 = struct_op_1030_73a8(uStack10);
        uVar2 = (uVar8 >> 0x10);
        iVar1 = (uVar8 + 0xc);
        if (((0x15 < iVar1) && (!SBORROW2(iVar1,0x16))) &&
           ((iVar1 == 0x17 || iVar1 + -0x16 < 0x1 ||
            ((0x0 < iVar1 + -0x18 && (iVar1 + -0x19 < 0x2)))))) {
          (uVar8 + 0x20) = 0x1;
        }
      }
      return;
    }
    pass1_1030_b578(param_1,param_2,param_3);
    if ((uVar5 | uVar3) == 0x0) goto LAB_1030_b0fc;
    uStack10 = CONCAT22(uVar5,uVar3);
    uVar8 = struct_op_1030_73a8(CONCAT22(uVar5,uVar3));
    uVar6 = (uVar8 >> 0x10);
    iVar1 = (uVar8 + 0xc);
    pass1_1008_3f62((u16 *)(param_1 & 0xffff0000 | (param_1 + 0x8)),
                    (u16 *)CONCAT22(uVar5,uVar3 + 0xc));
    if ((iVar1 == 0x18) || (uVar5 = uVar6, iVar1 == 0x3f)) {
      pass1_1030_b142(param_1,uStack10);
      uVar5 = uVar6;
    }
    BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,iVar1,0x40);
    uVar8 = BVar4;
    if (BVar4 != 0x0) {
      pass1_1030_b454(param_1,uStack10,param_3);
      goto LAB_1030_b0fc;
    }
    iStack12 += -0x1;
  } while( true );
}



fn pass1_1030_b13c(void)
{
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1030_b142(param_1: u32,param_2: u32)
{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u16;
  let bVar4: bool;
  let uVar5: u32;
  let uStack12: u32;
  
  uVar5 = struct_op_1030_73a8(param_2);
  uVar3 = (uVar5 >> 0x10);
  iVar1 = uVar5;
  iVar2 = (iVar1 + 0xc);
  uStack12 = 0x0;
  if (iVar2 == 0x18) {
    uStack12._2_2_ = pass1_1028_1c1c();
    uVar3 = (iVar1 + 0x22);
  }
  else {
    if (iVar2 != 0x3f) goto LAB_1030_b1a6;
    uStack12._2_2_ = pass1_1028_20b0();
    uVar3 = (iVar1 + 0x24);
  }
  uStack12 = CONCAT22(uStack12._2_2_,uVar3);
LAB_1030_b1a6:
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0xe) == 0x1) {
    bVar4 = (uStack12 & 0x10000) == 0x0;
  }
  else {
    if ((iVar2 + 0xe) == 0x2) {
      bVar4 = (uStack12 & 0x20000) == 0x0;
    }
    else {
      if ((iVar2 + 0xe) == 0x3) {
        bVar4 = (uStack12 & 0x40000) == 0x0;
      }
      else {
        bVar4 = (uStack12 & 0x80000) == 0x0;
      }
    }
  }
  if ((bVar4) || (uStack12 != 0x0)) {
    bVar4 = false;
    while( true ) {
      if (((uStack12 & 0x10000) != 0x0) && (uStack12 == 0x0)) goto LAB_1030_b239;
      if (((uStack12 & 0x20000) != 0x0) && (uStack12 == 0x0)) goto LAB_1030_b247;
      if (((uStack12 & 0x40000) != 0x0) && (uStack12 == 0x0)) goto LAB_1030_b255;
      if (((uStack12 & 0x80000) != 0x0) && (uStack12 == 0x0)) goto LAB_1030_b263;
      if (bVar4) break;
      uStack12._1_3_ = (uint3)(uStack12 >> 0x8) & 0xffff00;
      iVar1 = (iVar2 + 0xe);
      if (iVar1 == 0x1) {
        uStack12 = CONCAT31(uStack12._1_3_,0x4);
      }
      else {
        if (iVar1 == 0x2) {
          uStack12 = CONCAT31(uStack12._1_3_,0x8);
        }
        else {
          if (iVar1 == 0x3) {
            uStack12 = CONCAT31(uStack12._1_3_,0x1);
          }
          else {
            uStack12 = CONCAT31(uStack12._1_3_,0x2);
          }
        }
      }
      bVar4 = true;
    }
    if ((iVar2 + 0xe) == 0x1) {
LAB_1030_b255:
      (iVar2 + 0xe) = 0x3;
      return;
    }
    if ((iVar2 + 0xe) == 0x2) {
LAB_1030_b263:
      (iVar2 + 0xe) = 0x4;
      return;
    }
    if ((iVar2 + 0xe) == 0x3) {
LAB_1030_b239:
      (iVar2 + 0xe) = 0x1;
      return;
    }
    if ((iVar2 + 0xe) == 0x4) {
LAB_1030_b247:
      (iVar2 + 0xe) = 0x2;
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_b2aa(param_1: u32,param_2: *mut u16,uchar *param_3,param_4: i16,param_5: u16)
{
  let uVar1: u16;
  let BVar2: bool;
  let uVar3: u32;
  let bStack23: u8;
  let local_6: u32;
  
  pass1_1030_b718(param_1,(param_1 >> 0x10),param_2,
                  CONCAT22(param_5,&local_6),param_3,param_4,param_5);
  bStack23 = (byte)(local_6 >> 0x18);
  uVar1 = bStack23;
  if (bStack23 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,local_6,local_6._2_2_);
    if ((local_6._2_2_ | uVar1) != 0x0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(local_6._2_2_,uVar1));
      BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar3 + 0xc),0x42);
      if (BVar2 != 0x0) {
        pass1_1008_3f62((u16 *)(param_1 & 0xffff0000 | (param_1 + 0x8)),
                        (u16 *)CONCAT22(local_6._2_2_,uVar1 + 0xc));
        return;
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1030_b344(param_1: u32,param_2: u16) -> u32

{
  let puVar1: *mut u8
  let puStack18: u32;
  let puStack16: *mut u8
  let local_e: [u8;2];
  let local_c: i16;
  let local_a: i16;
  let local_8: u32;
  let uStack4: u16;
  
  local_8 = (param_1 + 0x8);
  uStack4 = (param_1 + 0xc);
  puVar1 = param_1._2_2_;
  pass1_1008_3eb4((u16 *)CONCAT22(param_2,&local_8),(u16 *)CONCAT22(param_2,local_e)
                  ,(u16 *)CONCAT22(param_2,&local_c),
                  (u16 *)CONCAT22(param_2,&local_a));
  local_8 = local_8 & 0xffff | (local_c - 0x1) << 0x10;
  puStack18 = &local_8;
  pass1_1030_b2aa(param_1,(u16 *)CONCAT22(param_2,puStack18),puVar1,&stack0xfffe,
                  param_2);
  puStack16 = (uchar *)(puVar1 | puStack18);
  if (puStack16 == (uchar *)0x0) {
    local_8 = local_8 & 0xffff | (local_c + 0x1) << 0x10;
    puStack18 = &local_8;
    pass1_1030_b2aa(param_1,(u16 *)CONCAT22(param_2,puStack18),(uchar *)0x0,
                    &stack0xfffe,param_2);
    puVar1 = (uchar *)(puStack16 | puStack18);
    if (puVar1 == (uchar *)0x0) {
      local_8._0_2_ = local_a + -0x1;
      local_8._2_2_ = local_c;
      puStack18 = &local_8;
      pass1_1030_b2aa(param_1,(u16 *)CONCAT22(param_2,puStack18),(uchar *)0x0,
                      &stack0xfffe,param_2);
      puStack16 = (uchar *)(puVar1 | puStack18);
      if (puStack16 == (uchar *)0x0) {
        local_8 = CONCAT22(local_8._2_2_,local_a + 0x1);
        puStack18 = &local_8;
        pass1_1030_b2aa(param_1,(u16 *)CONCAT22(param_2,puStack18),(uchar *)0x0,
                        &stack0xfffe,param_2);
        if ((puStack16 | puStack18) == 0x0) {
          return 0x0;
        }
        (param_1 + 0xe) = 0x2;
      }
      else {
        (param_1 + 0xe) = 0x4;
        puStack16 = puVar1;
      }
    }
    else {
      (param_1 + 0xe) = 0x3;
    }
  }
  else {
    (param_1 + 0xe) = 0x1;
    puStack16 = puVar1;
  }
  return CONCAT22(puStack16,puStack18);
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1030_b454(param_1: u32,param_2: u32,param_3: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let puVar3: *mut u8;
  let extraout_DX: u16;
  let iVar4: i16;
  let extraout_DX_00: u16;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u32;
  let uVar9: u32;
  let lStack38: i32;
  let uStack30: u32;
  let local_12: [u8;4];
  let uStack14: u32;
  let uStack10: u32;
  let lStack6: i32;
  
  lStack6 = *(long *)(param_2 + 0x4);
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  pass1_1008_5784(CONCAT22(param_3,local_12),(iVar6 + 0x10));
  while( true ) {
    puVar3 = local_12;
    pass1_1008_5b12(puVar3,param_3);
    uStack10 = CONCAT22(extraout_DX,puVar3);
    if ((extraout_DX | puVar3) == 0x0) break;
    if (*(long *)(puVar3 + 0x20) == lStack6) {
      ppcVar2 = (code **)((iVar6 + 0x10) + 0xc);
      (**ppcVar2)();
      uStack14 = 0x0;
      pass1_1038_69fe(uStack10);
    }
  }
  uVar8 = struct_op_1030_73a8(param_2);
  iVar4 = (uVar8 >> 0x10);
  puVar1 = (uVar8 + 0x20);
  puVar3 = local_12;
  pass1_1008_5784(CONCAT22(param_3,puVar3),puVar1);
  pass1_1030_b13c();
  uStack30 = CONCAT22(-(
                              (s_Unsupported_FileStructType_in_Op_1050_01ca + 0x2a) <
                             puVar3) - iVar4,0x1f4 - puVar3);
  do {
    puVar3 = local_12;
    pass1_1008_5b12(puVar3,param_3);
    uStack10 = CONCAT22(extraout_DX_00,puVar3);
    uVar5 = extraout_DX_00 | puVar3;
    if (uVar5 == 0x0) {
      return;
    }
    pass1_1038_6984(CONCAT22(extraout_DX_00,puVar3));
    lStack38 = CONCAT22(uVar5,puVar3);
    if ((uVar5 <= uStack30._2_2_) &&
       ((uVar5 < uStack30._2_2_ || (puVar3 <= uStack30)))) {
      uVar9 = (iVar6 + 0x10);
      ppcVar2 = (code **)((iVar6 + 0x10) + 0x8);
      (**ppcVar2)();
      uStack30 -= lStack38;
      ppcVar2 = (code **)(*puVar1 + 0xc);
      (**ppcVar2)(&PTR_LOOP_1050_1038,puVar1,(puVar1 >> 0x10),
                  uStack10,uVar9);
      uStack14 = 0x0;
    }
  } while (0x0 < uStack30);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_b578(param_1: u32,param_2: i16,param_3: u16)
{
  let iVar1: i16;
  let puVar2: u32;
  let uVar3: u16;
  let puVar4: *mut u8
  let bVar5: bool;
  let uVar6: u32;
  let uStack48: u32;
  let local_1c: [u8;2];
  let local_1a: i16;
  let local_18: i16;
  let local_16: u32;
  let uStack18: u16;
  let uStack16: u16;
  let uStack14: u32;
  let uStack10: u16;
  let uStack8: u16;
  let local_6: u32;
  
  pass1_1030_b718(param_1,param_1._2_2_,
                  (u16 *)(param_1 & 0xffff0000 | (param_1 + 0x8)),
                  CONCAT22(param_3,&local_6),param_1._2_2_,param_2,param_3);
  uStack48._3_1_ = (byte)(local_6 >> 0x18);
  uStack10 = uStack48._3_1_;
  if (uStack48._3_1_ == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,local_6,local_6._2_2_);
  uStack8 = local_6._2_2_;
  uStack14 = struct_op_1030_73a8(CONCAT22(local_6._2_2_,uStack10));
  uStack16 = (uStack14 + 0xc);
  local_16 = (param_1 + 0x8);
  uStack18 = (param_1 + 0xc);
  puVar4 = param_1._2_2_;
  pass1_1008_3eb4((u16 *)CONCAT22(param_3,&local_16),
                  (u16 *)CONCAT22(param_3,local_1c),
                  (u16 *)CONCAT22(param_3,&local_1a),
                  (u16 *)CONCAT22(param_3,&local_18));
  iVar1 = (param_1 + 0xe);
  if (iVar1 == 0x0) {
    pass1_1030_b344(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10,param_3);
    return;
  }
  if (iVar1 == 0x1) {
    uVar3 = local_1a - 0x1;
LAB_1030_b63e:
    local_16 = local_16 & 0xffff | uVar3 << 0x10;
    puVar2 = &local_16;
    pass1_1030_b2aa(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10,
                    (u16 *)CONCAT22(param_3,puVar2),puVar4,&uStack16,param_3);
    uStack48 = CONCAT22(puVar4,puVar2);
    if ((puVar4 | puVar2) == 0x0) {
      return;
    }
    uVar6 = struct_op_1030_73a8(CONCAT22(puVar4,puVar2));
    uVar3 = (uVar6 + 0xc);
    if (uVar3 == 0x3f) goto LAB_1030_b6e0;
    if (0x3f < uVar3) {
      return;
    }
    if ((char)uVar3 == '\x16') goto LAB_1030_b6e0;
    bVar5 = (char)uVar3 == '\x18';
  }
  else {
    if (iVar1 == 0x2) {
      uVar3 = local_18 + 0x1;
    }
    else {
      if (iVar1 == 0x3) {
        uVar3 = local_1a + 0x1;
        goto LAB_1030_b63e;
      }
      if (iVar1 != 0x4) {
        return;
      }
      uVar3 = local_18 - 0x1;
    }
    local_16 = local_16 & 0xffff0000 | uVar3;
    puVar2 = &local_16;
    pass1_1030_b2aa(param_1 & 0xffff | ZEXT24(param_1._2_2_) << 0x10,
                    (u16 *)CONCAT22(param_3,puVar2),puVar4,&uStack16,param_3);
    uStack48 = CONCAT22(puVar4,puVar2);
    if ((puVar4 | puVar2) == 0x0) {
      return;
    }
    uVar6 = struct_op_1030_73a8(CONCAT22(puVar4,puVar2));
    iVar1 = (uVar6 + 0xc);
    if (iVar1 < 0x17) {
      return;
    }
    if (SBORROW2(iVar1,0x17)) {
      return;
    }
    if (iVar1 == 0x18 || iVar1 + -0x17 < 0x1) goto LAB_1030_b6e0;
    bVar5 = iVar1 == 0x3f;
  }
  if (!bVar5) {
    return;
  }
LAB_1030_b6e0:
  pass1_1008_3f62((u16 *)(param_1 & 0xffff0000 | (param_1 + 0x8)),
                  (u16 *)(uStack48 & 0xffff0000 | (uStack48 + 0xc)));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_b718(param_1: u16,param_2: u16,param_3: *mut u16,param_4: *mut u32,
               uchar *param_5,param_6: i16,param_7: u16)

{
  let puVar1: u32;
  let uVar2: u16;
  let local_12: [u32;0x2];
  let lStack10: i32;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_7,param_5,param_6);
  uVar2 = (puStack6 >> 0x10);
  lStack10 = *(long *)(puStack6 + 0x20);
  puVar1 = local_12;
  pass1_1030_64ce(param_7,puVar1,uVar2,_PTR_LOOP_1050_5740,param_3,lStack10,
                  CONCAT22(param_7,puVar1));
  *param_4 = *puVar1;
  return;
}



fn pass1_1030_b768(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u32;
  let BVar2: bool;
  let iVar3: i16;
  let puVar4: *mut u8;
  let extraout_DX: u16;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let local_22: [u16;0x4];
  u8 local_1a [0xa];
  let local_10: u32;
  let puStack12: *mut u8;
  let uStack10: u16;
  let local_8: [u16;0x3];
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  local_10 = (iVar5 + 0x4);
  uVar7 = param_2;
  uVar8 = (param_2 >> 0x10);
  BVar2 = write_to_file_1008_7e1c
                    (uVar7,uVar8,&local_10,param_3,0x4,0x1008);
  if ((BVar2 != 0x0) &&
     (iVar3 = write_to_file_1008_7b4c
                        (param_2,param_1 & 0xffff0000 | (iVar5 + 0x8),0x1008,
                         param_3), iVar3 != 0x0)) {
    local_8[0] = (iVar5 + 0xe);
    BVar2 = write_to_file_1008_7e1c
                      (uVar7,uVar8,local_8,param_3,0x2,0x1008);
    if (BVar2 != 0x0) {
      uVar1 = (iVar5 + 0x10);
      local_22[0] = (uVar1 + 0x8);
      local_10 = local_10 & 0xffff0000 | local_22[0];
      BVar2 = write_to_file_1008_7e1c
                        (uVar7,uVar8,local_22,param_3,0x2,0x1008);
      if (BVar2 == 0x0) {
        return;
      }
      pass1_1008_5784(CONCAT22(param_3,local_1a),(iVar5 + 0x10));
      do {
        puVar4 = local_1a;
        pass1_1008_5b12(puVar4,param_3);
        if ((extraout_DX | puVar4) == 0x0) {
          return;
        }
        puStack12 = puVar4;
        uStack10 = extraout_DX;
        pass1_1038_75ca(CONCAT22(extraout_DX,puVar4),param_2,puVar4,param_3);
      } while (puVar4 != 0x0);
      return;
    }
  }
  PTR_LOOP_1050_0310 = 0x6d0;
  return;
}



fn file_1030_b836(param_1: u32,param_2: u32,uchar *param_3,param_4: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u16;
  astruct_401 *iVar4;
  let Bvar4: bool;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let puVar8: *mut u8
  let extraout_DX: *mut u8
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u32;
  let uVar13: u16;
  let local_12: [u16;0x7];
  let local_4: u16;
  
  iVar4 = (astruct_401 *)param_1;
  iVar4 = (astruct_401 *)&iVar4->field_0x4;
  uVar3 = (param_1 >> 0x10);
  uVar9 = param_2;
  uVar10 = (param_2 >> 0x10);
  BVar4 = read_file_1008_7dee(uVar9,uVar10,iVar4,0x0,uVar3,0x4,0x1008);
  if (((BVar4 == 0x0) ||
      (BVar4 = read_file_1008_7bc8(param_2,(u16 *)
                                           (param_1 & 0xffff0000 |
                                           &iVar4->field_0x8),0x1008,param_4)
      , BVar4 == 0x0)) ||
     (BVar4 = read_file_1008_7dee(uVar9,uVar10,&local_4,0x0,param_4,0x2,0x1008),
     BVar4 == 0x0)) {
    PTR_LOOP_1050_0310 = 0x6d2;
  }
  else {
    iVar4->field_0xe = local_4;
    BVar4 = read_file_1008_7dee(uVar9,uVar10,local_12,0x0,param_4,0x2,0x1008);
    if (BVar4 != 0x0) {
      while( true ) {
        if (local_12[0] == 0x0) {
          return;
        }
        uVar11 = 0x2a;
        uVar5 = local_12[0];
        local_12[0] = local_12[0] - 0x1;
        uVar12 = param_2;
        mem_op_1000_179c(0x2a,param_3,0x1000);
        puVar8 = (uchar *)(param_3 | uVar5);
        if (puVar8 == (uchar *)0x0) {
          uVar6 = 0x0;
          puVar8 = (uchar *)0x0;
        }
        else {
          uVar6 = uVar5;
          struct_1038_6520((u16 *)CONCAT22(param_3,uVar5));
        }
        uVar13 = (uVar12 >> 0x10);
        uVar7 = uVar6;
        file_1038_774e(CONCAT22(puVar8,uVar6),CONCAT22(uVar12,uVar11),puVar8,param_4)
        ;
        if (uVar7 == 0x0) break;
        puVar1 = iVar4->field_0x10;
        ppcVar2 = (code **)(*iVar4->field_0x10 + 0x4);
        (**ppcVar2)(&PTR_LOOP_1050_1038,puVar1,(puVar1 >> 0x10),
                    uVar6,puVar8,uVar13,uVar5);
        param_3 = extraout_DX;
      }
    }
  }
  return;
}



astruct_18 *  pass1_1030_b90c(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_afa6((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void 
pass1_1030_b936(astruct_365 *param_1,param_2: u16,param_3: u16,param_4: u32,
               param_5: u16)

{
  pass1_1028_b22c((u16 *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  param_1->field_0xe = 0x0;
  param_1->field_0x12 = 0x0;
  CONCAT22(param_2,param_1) = 0xbc0c;
  param_1->field_0x2 = 0x1030;
  return;
}



fn pass1_1030_b96c(param_1: *mut u16)
{
  let uVar1: u16;
  astruct_18 *paVar2;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  *param_1 = 0xbc0c;
  (iVar3 + 0x2) = 0x1030;
  paVar2 = *(astruct_18 **)(iVar3 + 0xe);
  uVar1 = (iVar3 + 0x10);
  if ((uVar1 | paVar2) != 0x0) {
    fn_ptr_1020_ba7e((paVar2 & 0xffff | uVar1 << 0x10));
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  pass1_1028_b260(param_1);
  return;
}



fn pass1_1030_b9b2(param_1: u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0xe) = 0x0;
  (param_1 + 0x12) = 0x0;
  return;
}



void 
pass1_1030_b9da(param_1: u32,param_2: u32,param_3: u32,param_4: u32,param_5: u16,
               param_6: u16,param_7: u16)

{
  let puVar1: u32;
  let uVar2: u32;
  let puVar3: *mut u8
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u32;
  astruct_402 *iVar7;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u32;
  let uStack12: u16;
  let uStack4: u16;
  
  puVar3 = (uchar *)param_3;
  uVar9 = (param_1 >> 0x10);
  iVar7 = (astruct_402 *)param_1;
  if (iVar7->field_0xe == (long *)0x0) {
    mem_op_1000_179c(0xa,puVar3,0x1000);
    uVar4 = puVar3 | param_4;
    param_3 = uVar4;
    if (uVar4 == 0x0) {
      iVar7->field_0xe = (long *)0x0;
    }
    else {
      pass1_1020_ba3e((long *)(param_4 & 0xffff | ZEXT24(puVar3) << 0x10),0x5,0x5,param_6,
                      param_5);
      &iVar7->field_0xe = param_4;
      (&iVar7->field_0xe + 0x2) = param_3;
    }
    iVar7->field_0x12 = 0x0;
  }
  for (uStack4 = 0x4; uStack4 < 0xe; uStack4 += 0x1) {
    uVar10 = pass1_1030_7c28(param_2,uStack4,param_4,param_3,param_7);
    uVar4 = (uVar10 >> 0x10);
    param_4 = uVar10 & 0xffff;
    uVar5 = uVar4 | param_4;
    param_3 = uVar5;
    if (uVar5 != 0x0) {
      uVar2 = 0x64 - iVar7->field_0x12;
      uVar6 = uVar2 >> 0x10;
      uStack12 = uVar10;
      if (uVar10 < uVar2) {
        uVar2 = uVar10 & 0xffff;
        uVar6 = uVar4;
      }
      uVar5 = uVar2;
      param_4 = uVar2 & 0xffff | uVar6 << 0x10;
      iVar8 = (uVar4 - uVar6) - (uStack12 < uVar5);
      param_3 = uVar6;
      pass1_1030_7d1c(param_2,uStack12 - uVar5,CONCAT22(uStack4,iVar8),uVar5,uVar6,
                      iVar8,param_6,param_7);
      pass1_1020_bb8a(iVar7->field_0xe,uVar5,uVar6 | uStack4 << 0x10,param_6,
                      param_7);
      puVar1 = &iVar7->field_0x12;
      *puVar1 = *puVar1 + param_4;
      string_1020_c0ca(uStack4);
      vsprintf_op_1030_840a
                (s_truck_0x_08lx_loaded__ld_of__s_f_1050_576c,0x1020,param_7,
                 param_3);
      if (0x63 < iVar7->field_0x12) break;
    }
  }
  if (iVar7->field_0x12 != 0x0) {
    return;
  }
  return;
}



void 
pass1_1030_bb0e(param_1: u32,param_2: u32,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16)

{
  astruct_18 *paVar1;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u32;
  let in_DX: u16;
  let puVar5: *mut u8
  let uVar6: u32;
  let uStack8: u16;
  
  uVar3 = pass1_1030_7bee(param_2);
  uVar4 = uVar3;
  if (uVar3 != 0x0) {
    return;
  }
  pass1_1030_b9b2(param_1);
  uVar2 = uVar4 & 0xffff;
  paVar1 = (astruct_18 *)(uVar2 | in_DX << 0x10);
  puVar5 = (uchar *)(in_DX | uVar4);
  if (puVar5 != (uchar *)0x0) {
    for (uStack8 = 0x4; uStack8 < 0x25; uStack8 += 0x1) {
      uVar6 = pass1_1020_bae6(uVar2,CONCAT22(uStack8,in_DX),uVar4,
                              puVar5,param_6);
      uVar4 = uVar6 & 0xffff;
      puVar5 = (uchar *)((uVar6 >> 0x10) | uVar4);
      if (puVar5 != (uchar *)0x0) {
        pass1_1030_7ddc(param_2,uVar6,uStack8,uVar4,puVar5,param_4,param_5,param_6);
        uVar3 = pass1_1030_7bee(param_2);
        uVar4 = uVar3;
        if (uVar3 != 0x0) {
          return;
        }
        string_1020_c0ca(uStack8);
        vsprintf_op_1030_840a
                  (s_truck_0x_08lx_unloaded__ld_of__s_1050_5798,0x1020,param_6,
                   puVar5);
        pass1_1020_bb8a((long *)paVar1,0x0,uStack8 << 0x10,param_5,param_6);
      }
    }
    if (paVar1 != (astruct_18 *)0x0) {
      fn_ptr_1020_ba7e(paVar1);
      fn_ptr_1000_17ce(paVar1,0x1000);
    }
  }
  return;
}



astruct_18 *  pass1_1030_bbe6(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_b96c((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1030_bc24(param_1: u16,param_2: i16,param_3: u16,param_4: u16,param_5: u32) -> u16

{
  pass1_1028_b22c((u16 *)CONCAT22(param_3,param_2),param_4,param_5,param_1);
  CONCAT22(param_3,param_2) = 0xbc96;
  (param_2 + 0x2) = 0x1030;
  return (u16 *)CONCAT22(param_3,param_2);
}



fn pass1_1030_bc4e(param_1: *mut u16)
{
  *param_1 = 0xbc96;
  (param_1 + 0x2) = 0x1030;
  pass1_1028_b260(param_1);
  return;
}



astruct_18 *  pass1_1030_bc70(astruct_18 *param_1,param_2: u8)

{
  pass1_1030_bc4e((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1030_bcae(param_1: u16,param_2: u16) -> u32

{
  return CONCAT22(param_2,param_1);
}



void 
pass1_1030_bcbc(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: u16,
               param_6: u32)

{
  pass1_1030_bcde(param_1,param_2,param_3,
                  CONCAT22(param_4,param_3._2_2_),
                  (u16 *)CONCAT22(param_5,param_4._2_2_),*(long *)(param_6 + 0x4))
  ;
  return;
}



void 
pass1_1030_bcde(param_1: u16,param_2: u16,param_3: u16,param_4: u32,param_5: *mut u16
               ,param_6: i32)

{
  let iVar1: i16;
  let uVar2: u16;
  let local_14: i16;
  let local_12: i16;
  let local_10: i16;
  let local_e: i16;
  let local_c: u32;
  let uStack8: u16;
  let lStack6: i32;
  
  uVar2 = (param_4 >> 0x10);
  iVar1 = param_4;
  lStack6 = *(long *)(iVar1 + 0x8);
  if (lStack6 != param_6) {
    return;
  }
  local_c = (iVar1 + 0xc);
  uStack8 = (iVar1 + 0x10);
  pass1_1008_3e94(param_5,(u16 *)CONCAT22(param_1,&local_10),
                  (u16 *)CONCAT22(param_1,&local_e));
  pass1_1008_3e94((u16 *)CONCAT22(param_1,&local_c),
                  (u16 *)CONCAT22(param_1,&local_14),
                  (u16 *)CONCAT22(param_1,&local_12));
  pass1_1000_49b2(local_e - local_12);
  pass1_1000_49b2(local_10 - local_14);
  return;
}



fn pass1_1030_bd74(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: u16)
{
  astruct_670 *iVar1;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  let local_1e: i16;
  let local_1c: i16;
  let local_1a: i16;
  let local_18: i16;
  let local_16: u32;
  let uStack18: u16;
  let local_10: u32;
  let uStack12: u16;
  let lStack10: i32;
  let lStack6: i32;
  
  uVar3 = (param_4 >> 0x10);
  iVar1 = (astruct_670 *)param_4;
  lStack6 = iVar1->field_0x8;
  uVar4 = (param_3 >> 0x10);
  iVar2 = param_3;
  lStack10 = *(long *)(iVar2 + 0x8);
  if (lStack10 != lStack6) {
    return;
  }
  local_10 = iVar1->field_0xc;
  uStack12 = iVar1->field_0x10;
  local_16 = (iVar2 + 0xc);
  uStack18 = (iVar2 + 0x10);
  pass1_1008_3e94((u16 *)CONCAT22(param_5,&local_10),
                  (u16 *)CONCAT22(param_5,&local_1a),
                  (u16 *)CONCAT22(param_5,&local_18));
  pass1_1008_3e94((u16 *)CONCAT22(param_5,&local_16),
                  (u16 *)CONCAT22(param_5,&local_1e),
                  (u16 *)CONCAT22(param_5,&local_1c));
  pass1_1000_49b2(local_18 - local_1c);
  pass1_1000_49b2(local_1a - local_1e);
  return;
}



fn struct_1030_be34(param_1: *mut u16) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0xc006;
  (param_1 + 0x2) = 0x1030;
  return param_1;
}



fn pass1_1030_be56(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e((u16 *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xc006;
  (param_1 + 0x2) = 0x1030;
  return (u16 *)CONCAT22(param_2,param_1);
}



fn pass1_1030_be80(param_1: u32,uchar *param_2,param_3: u16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let iVar3: i16;
  let Bvar4: bool;
  let uVar5: u32;
  let extraout_DX: u16;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u16;
  let iVar9: i16;
  
  pass1_1028_bf22(param_1,param_2,param_3);
  uVar8 = (param_1 >> 0x10);
  iVar7 = param_1;
  if ((iVar7 + 0x12) == 0x5) {
    uVar2 = (iVar7 + 0x14);
    (uVar2 + 0xa4) = 0x1e;
    uVar2 = (iVar7 + 0x14);
    (uVar2 + 0xac) = 0x1;
    iVar9 = (iVar7 + 0xc);
    iVar3 = iVar9 + -0x1b;
    if (iVar3 == 0x0) {
      uVar2 = (iVar7 + 0x14);
      (uVar2 + 0xaa) = 0xa;
    }
    else {
      iVar3 = iVar9 + -0x1c;
      if (iVar3 == 0x0) {
        uVar2 = (iVar7 + 0x14);
        (uVar2 + 0xaa) = 0xb;
      }
      else {
        iVar3 = iVar9 + -0x1d;
        if (iVar3 == 0x0) {
          uVar2 = (iVar7 + 0x14);
          (uVar2 + 0xaa) = 0xc;
        }
      }
    }
    pass1_1028_b58e(param_1);
    uVar5 = (iVar3 + 0x2e);
    iVar9 = 0xc;
    uVar6 = extraout_DX;
    pass1_1038_3fb0(uVar5);
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | uVar6 << 0x10,iVar9);
    if (BVar4 != 0x0) {
      uVar2 = (iVar7 + 0x14);
      piVar1 = (i16 *)(uVar2 + 0xaa);
      *piVar1 = *piVar1 + 0x1;
    }
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | uVar6 << 0x10,0xe);
    if (BVar4 != 0x0) {
      uVar2 = (iVar7 + 0x14);
      piVar1 = (i16 *)(uVar2 + 0xaa);
      *piVar1 = *piVar1 + 0x1;
    }
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | uVar6 << 0x10,0x76);
    if (BVar4 != 0x0) {
      uVar2 = (iVar7 + 0x14);
      piVar1 = (i16 *)(uVar2 + 0xaa);
      *piVar1 = *piVar1 + 0x1;
    }
  }
  return;
}



fn pass1_1030_bf6e(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let puVar1: *mut u16;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u32;
  let uVar8: u32;
  let uVar9: u16;
  
  uVar9 = 0x1e;
  uVar7 = pass1_1028_b58e(param_1);
  uVar8 = pass1_1030_7c28(uVar7,uVar9,uVar7,(uVar7 >> 0x10),param_4);
  uVar4 = 0x3e8 - uVar8;
  uVar2 = (param_1 + 0x14);
  uVar6 = (uVar2 >> 0x10);
  iVar5 = uVar2;
  puVar1 = (u16 *)(iVar5 + 0xaa);
  uVar3 = -(uVar4 < *puVar1);
  pass1_1030_7ddc(uVar7,((uVar4 - *puVar1 & uVar3) + (iVar5 + 0xaa)),0x1e,
                  uVar3,(uchar *)(uVar8 >> 0x10),param_2,param_3,param_4);
  return;
}



fn pass1_1030_bfb8(param_1: u32,param_2: u16) -> u16

{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  
  uVar3 = 0x1e;
  uVar1 = pass1_1028_b58e(param_1);
  uVar2 = pass1_1030_7c28(uVar1,uVar3,uVar1,(uVar1 >> 0x10),param_2);
  return 0x3e8 - uVar2;
}



astruct_18 *  pass1_1030_bfe0(astruct_18 *param_1,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1030_c09c(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16)
{
  pass1_1028_b39e((u16 *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  (param_1 + 0x20) = 0x0;
  (param_1 + 0x24) = 0x0;
  CONCAT22(param_2,param_1) = 0xc68e;
  (param_1 + 0x2) = 0x1030;
  return;
}



fn pass1_1030_c0d2(param_1: u32) -> u16

{
  if (0x0 < (param_1 + 0x24)) {
    return 0x1;
  }
  return 0x0;
}



fn pass1_1030_c0ec(param_1: u32) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  if (((param_1 + 0xc) != 0xb) && ((param_1 + 0x24) < 0x1)) {
    return 0x0;
  }
  return 0x1;
}



fn pass1_1030_c10e(param_1: u32)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (0x0 < (iVar2 + 0x24)) {
    piVar1 = (i16 *)(iVar2 + 0x24);
    *piVar1 = *piVar1 + -0x1;
    return;
  }
  (iVar2 + 0xc) = 0x37;
  return;
}



void 
pass1_1030_c12e(param_1: u32,param_2: i16,param_3: i16,param_4: u16,param_5: u16,
               param_6: u16)

{
  let piVar1: *mut i16;
  let uVar2: u32;
  let iVar3: i16;
  let extraout_DX: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uStack6: u32;
  
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,param_3);
  uVar2 = (param_3 + 0x2e);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  iVar3 = uVar2;
  if ((iVar4 + 0x24) < 0x1) {
    pass1_1030_7d1c(uStack6,0x0,0x230000,iVar3,extraout_DX,param_4,param_5,param_6);
  }
  else {
    if (param_2 == 0x0) {
      uVar6 = 0x0;
    }
    else {
      uVar6 = 0x32;
    }
    pass1_1030_7d1c(uStack6,uVar6,0x230000,iVar3,extraout_DX,param_4,param_5,param_6);
    piVar1 = (i16 *)(iVar4 + 0x24);
    *piVar1 = *piVar1 + -0x1;
  }
  if ((0x0 < (iVar4 + 0x24)) && ((iVar4 + 0x24) < 0x19)) {
    (iVar3 + 0x1fe) = 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_c1b2(param_1: *mut u32,uchar *param_2,param_3: u16,param_4: u16,param_5: u16
               )

{
  let iVar1: i16;
  astruct_695 *iVar2;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  pass1_1028_be9e(param_1,param_3,param_4,&USHORT_1050_1028,param_5);
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_695 *)param_1;
  if (iVar2->field_0x12 == 0x5) {
    if (iVar2->field_0xc == 0xb) {
      pass1_1030_c652(param_2,param_4,param_5);
      iVar1 = 0x82;
      puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_5,param_2,param_4);
      pass1_1010_9f8c(puVar3,iVar1,param_5);
      iVar2->field_0x24 = puVar3 * 0x3;
      mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_5,(uchar *)(puVar3 >> 0x10),
                      param_4);
      if (PTR_LOOP_1050_13ae < 0x3) {
        iVar1 = iVar2->field_0x24;
        if (iVar1 < 0x32) {
          iVar1 = 0x32;
        }
        iVar2->field_0x24 = iVar1;
        return;
      }
    }
    else {
      iVar2->field_0x24 = 0x64;
    }
  }
  return;
}



fn pass1_1030_c230(param_1: u32,param_2: u32,param_3: u16)
{
  let BVar1: bool;
  let uVar2: u16;
  let uVar3: u16;
  let local_10: [u32;0x2];
  let local_8: [u16;0x3];
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    uVar2 = (param_1 >> 0x10);
    local_10[0] = (param_1 + 0x20);
    uVar3 = (param_2 >> 0x10);
    BVar1 = write_to_file_1008_7e1c
                      (param_2,uVar3,local_10,param_3,0x4,0x1008);
    if (BVar1 != 0x0) {
      local_8[0] = (param_1 + 0x24);
      BVar1 = write_to_file_1008_7e1c
                        (param_2,uVar3,local_8,param_3,0x2,0x1008)
      ;
      if (BVar1 != 0x0) {
        return;
      }
    }
    PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}



fn pass1_1030_c29c(param_1: u32,param_2: u32,param_3: i16,uchar *param_4,param_5: u16)
{
  let uVar1: u16;
  let BVar2: bool;
  let uVar3: u16;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 != 0x0) {
    uVar1 = (param_1 >> 0x10);
    uVar3 = (param_2 >> 0x10);
    BVar2 = read_file_1008_7dee(param_2,uVar3,param_1 + 0x20,0x0,uVar1,0x4,
                                0x1008);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee(param_2,uVar3,param_1 + 0x24,0x0,uVar1,0x2,
                                  0x1008);
      if (BVar2 != 0x0) {
        return;
      }
    }
    PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_c2fa(param_1: u32,param_2: i16,uchar *param_3,param_4: u16,param_5: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u32;
  let uVar4: u32;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u32;
  let puVar9: *mut u8
  let uVar10: u32;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  let puVar14: *mut u16;
  let uVar15: u16;
  let uStack84: u16;
  let lStack80: i32;
  let iStack56: i16;
  let uStack10: u32;
  let uStack6: u32;
  astruct_698 *iVar5;
  
  uVar12 = (param_1 >> 0x10);
  if ((param_1 + 0xc) != 0xb) {
    pass1_1028_bd38(param_1,param_3,param_5);
    uVar1 = (param_1 + 0x20);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
    uStack6 = CONCAT22(param_3,param_2);
    iVar6 = param_2;
    puVar9 = param_3;
    pass1_1028_b58e(param_1);
    uStack10 = CONCAT22(puVar9,iVar6);
    uVar2 = (iVar6 + 0x2e);
    puVar14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_5,puVar9,param_4);
    uVar10 = puVar14 >> 0x10;
    uVar13 = (uVar2 >> 0x10);
    pass1_1010_ed22(puVar14,(uVar2 + 0x4),param_5);
    uVar3 = (param_2 + 0x1f6);
    uVar8 = uVar3;
    pass1_1030_3694(uVar3,0x3,0x2,(uchar *)uVar10,0x1010,param_5);
    uVar12 = (uVar3 >> 0x10);
    uVar4 = (uVar2 + 0x1f6);
    pass1_1030_355c(uVar4,uVar8 & 0xffff | uVar10 << 0x10);
    uVar13 = (uVar4 >> 0x10);
    iStack56 = 0x0;
    do {
      iVar5 = (astruct_698 *)(iStack56 * 0x2);
      (iVar5 + uVar4 + 0x174) =
           (iVar5 + uVar3 + 0x174);
      uVar7 = (iVar5 + uVar3 + 0x180);
      uVar8 = uVar7;
      (iVar5 + uVar4 + 0x180) = uVar7;
      iStack56 += 0x1;
    } while (iStack56 < 0x6);
    uStack84 = 0x11;
    while( true ) {
      puVar9 = (uchar *)uVar10;
      uVar7 = uVar8;
      if (0x24 < uStack84) break;
      if (0x0 < (uStack84 * 0x2 + _PTR_LOOP_1050_580e)) {
        empty_1038_540a();
        lStack80 = CONCAT22(puVar9,uVar7);
        uVar12 = (_PTR_LOOP_1050_580e >> 0x10);
        uVar11 = _PTR_LOOP_1050_580e;
        iVar6 = (uStack84 * 0x2 + uVar11);
        uVar10 = (long)iVar6 >> 0x10;
        uVar15 = uStack84;
        if (lStack80 < iVar6) {
          iVar6 = (uStack84 * 0x2 + uVar11);
          uVar10 = (iVar6 >> 0xf);
          uVar15 = 0x21;
        }
        pass1_1038_52b8(uStack6,CONCAT22(uVar10,iVar6),uVar15,uVar11,param_4,
                        &PTR_LOOP_1050_1038,param_5);
        uVar15 = uStack84 * 0x2;
        uVar7 = (uVar15 + _PTR_LOOP_1050_580e);
        pass1_1030_7ddc(uStack10,(long)uVar7,uStack84,uVar7,(uchar *)uVar10,uVar15,
                        param_4,param_5);
        iVar6 = (_PTR_LOOP_1050_580e + uVar15);
        uVar8 = SEXT24(iVar6);
        pass1_1038_5694(uVar2,(long)iVar6,uStack84);
      }
      uStack84 += 0x1;
    }
    pass1_1030_7c50(uStack10,0x2,0x1,uVar7,puVar9);
    pass1_1030_7c50(uStack10,0x2,0x2,uVar7,puVar9);
    pass1_1030_7c50(uStack10,0x2,0x3,uVar7,puVar9);
    pass1_1030_7c50(uStack10,0x2,0x4,uVar7,puVar9);
    pass1_1038_44d8(param_2,param_3,0x2,0x1,uVar7,puVar9);
    pass1_1038_44d8(param_2,param_3,0x2,0x2,uVar7,puVar9);
    pass1_1038_44d8(param_2,param_3,0x2,0x3,uVar7,puVar9);
    pass1_1038_44d8(param_2,param_3,0x2,0x4,uVar7,puVar9);
    puVar14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,puVar9,param_4);
    pass1_1010_043a(puVar14,*(long *)(param_2 + 0x4),0x7,param_5);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_c52e(param_1: u32,param_2: *mut u16,param_3: u32,param_4: u32,param_5: i16,
               param_6: u16,param_7: u16)

{
  let BVar1: bool;
  let puVar2: u32;
  let puVar3: *mut u8;
  let puVar4: u32;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u32;
  let uVar8: u16;
  let puVar9: *mut u16;
  let uVar10: u32;
  let local_32: [u8;12];
  let local_20: u32;
  let uStack28: u32;
  let puStack24: u32;
  let uStack22: u32;
  let uStack18: u16;
  let uStack16: u16;
  let local_c: u32;
  let uStack8: u16;
  let uStack6: u32;
  
  uVar8 = (param_1 >> 0x10);
  BVar1 = pass1_1028_c314(param_7,param_5,param_6,param_1,uVar8,param_2,
                          param_3,(param_3 >> 0x10),param_4);
  if (BVar1 != 0x0) {
    puVar2 = &local_c;
    pass1_1030_64ce(param_7,puVar2,param_6,_PTR_LOOP_1050_5740,param_2,param_4,
                    CONCAT22(param_7,puVar2));
    local_20 = *puVar2;
    local_20._3_1_ = (byte)(local_20 >> 0x18);
    uStack8 = local_20._3_1_;
    if (local_20._3_1_ == 0x0) {
      uStack22 = local_20;
      uStack6 = local_20;
      pass1_1028_c7b6(param_7,param_6,param_1,uVar8,param_2,param_4);
      if ((uStack8 != 0x4) && (uStack8 != 0x0)) {
        uVar7 = pass1_1030_bcae(&local_20,param_7);
        uVar5 = (uVar7 >> 0x10);
        pass1_1028_dc52((astruct_92 *)CONCAT22(param_7,local_32),0x1,0x0,0x400);
        while( true ) {
          puVar3 = local_32;
          pass1_1028_e4ec(CONCAT22(param_7,puVar3));
          uStack28 = CONCAT22(uVar5,puVar3);
          uVar6 = uVar5 | puVar3;
          if (uVar6 == 0x0) {
            return;
          }
          uVar7 = (puVar3 + 0x10);
          uVar10 = param_4;
          uStack22 = uVar7;
          puVar9 = param_2;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7,(uVar7 >> 0x10));
          uStack18 = uVar7;
          puVar4 = &local_20;
          uStack16 = uVar6;
          pass1_1030_bcde(param_7,puVar4,param_7,
                          uVar7 & 0xffff | uVar6 << 0x10,puVar9,uVar10);
          if (puVar4 < 0x0) break;
          uVar5 = uVar6;
          puStack24 = puVar4;
          if (puVar4 < 0x1f) {
            PTR_LOOP_1050_50ca = 0x6ae;
            return;
          }
        }
        PTR_LOOP_1050_50ca = 0x6af;
        return;
      }
      PTR_LOOP_1050_50ca = 0x6a8;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_c652(uchar *param_1,param_2: i16,param_3: u16)
{
  let puVar1: *mut u16;
  
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_3,param_1,param_2);
  pass1_1010_9794(puVar1,param_3);
  return;
}



astruct_18 *  pass1_1030_c668(astruct_18 *param_1,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1030_c71e(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e((u16 *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  (param_1 + 0x20) = 0x0;
  CONCAT22(param_2,param_1) = 0xc940;
  (param_1 + 0x2) = 0x1030;
  return (u16 *)CONCAT22(param_2,param_1);
}



fn pass1_1030_c74e(param_1: u32,param_2: u32,param_3: u16)
{
  pass1_1028_b46e(param_1,param_2,param_3);
  (param_1 + 0x20) = 0x70;
  return;
}



fn pass1_1030_c76c(param_1: *mut u32,param_2: u16,param_3: u16,param_4: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (((iVar1 + 0x12) != 0x6) && ((iVar1 + 0x12) != 0x5)) {
    return;
  }
  iVar1 = (iVar1 + 0x20);
  if (iVar1 != 0x0) {
    if (((iVar1 < 0x70) || (SBORROW2(iVar1,0x70))) || (0x1 < iVar1 + -0x70)) {
      pass1_1028_be2a(param_1,param_2,param_3,&USHORT_1050_1028,param_4);
      return;
    }
  }
  pass1_1028_bdac(param_1,0x6,&USHORT_1050_1028);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_c7b0(param_1: u32,param_2: u16,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u32;
  let iVar3: i16;
  let iVar4: i16;
  let BVar5: bool;
  let uVar6: u32;
  let extraout_DX: *mut u8
  let puVar7: *mut u8
  let iVar8: i16;
  let uVar9: u16;
  
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  iVar1 = (iVar8 + 0x20);
  if (iVar1 != 0x0) {
    iVar3 = iVar1 + -0x70;
    iVar4 = iVar3;
    if (((iVar1 < 0x70) || (SBORROW2(iVar1,0x70))) ||
       (iVar4 = iVar1 + -0x71, iVar4 != 0x0 && 0x0 < iVar3)) {
      pass1_1028_b58e(param_1 & 0xffff | uVar9 << 0x10);
      uVar2 = (iVar4 + 0x2e);
      uVar6 = (uVar2 + 0x200);
      puVar7 = extraout_DX;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6,(uVar6 >> 0x10));
      uVar6 = uVar6 & 0xffff | ZEXT24(puVar7) << 0x10;
      BVar5 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(iVar8 + 0xc),0x11);
      pass1_1030_23e2(uVar6,BVar5,(iVar8 + 0x20),BVar5,puVar7,param_2,param_3);
      if (BVar5 != 0x0) {
        if ((iVar8 + 0x20) == 0x1) {
          pass1_1030_25d8(uVar6,0x64,(iVar8 + 0x20));
          return;
        }
        (iVar8 + 0x20) = 0x70;
      }
    }
  }
  return;
}



fn pass1_1030_c84e(param_1: u32,param_2: u32,param_3: u16) -> bool

{
  let BVar1: bool;
  let local_c: [u16;0x5];
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    local_c[0] = (param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c
                      (param_2,(param_2 >> 0x10),local_c,param_3,
                       0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = 0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}



fn pass1_1030_c894(param_1: u32,param_2: u32,bool param_3,uchar *param_4,param_5: u16) -> bool

{
  let BVar1: bool;
  let local_4: u16;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 != 0x0) {
    BVar1 = read_file_1008_7dee(param_2,(param_2 >> 0x10),&local_4
                                ,0x0,param_5,0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = 0x6d2;
      return BVar1;
    }
    (param_1 + 0x20) = local_4;
    param_3 = 0x1;
  }
  return param_3;
}



fn pass1_1030_c8da(param_1: u32,param_2: u32,param_3: u32) -> u32

{
  let uVar1: u32;
  
  uVar1 = 0x0;
  if (param_3._2_2_ == 0x1) {
    (param_1 + 0x20) = param_2._2_2_;
  }
  else {
    uVar1 = func_0x1030178e();
  }
  return uVar1;
}



astruct_18 *  pass1_1030_c91a(astruct_18 *param_1,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1030_c9e4(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u32

{
  pass1_1028_b39e((u16 *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  (param_1 + 0x98) = 0x1;
  CONCAT22(param_2,param_1) = 0xd88e;
  (param_1 + 0x2) = 0x1030;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,param_1 + 0x20),(WNDCLASS16 *)0x0,0x78);
  return CONCAT22(param_2,param_1);
}



fn pass1_1030_ca26(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  let extraout_DX: u16;
  let iVar2: i16;
  let uVar3: u16;
  let uStack4: u16;
  
  for (uStack4 = 0x0; iVar2 = param_1, uVar3 = (param_1 >> 0x10),
      uStack4 < 0xa; uStack4 += 0x1) {
    if (((iVar2 + uStack4 * 0xc + 0x26) == 0x2) ||
       ((iVar2 + uStack4 * 0xc + 0x26) == 0x1)) {
      (iVar2 + uStack4 * 0xc + 0x26) = 0x4;
      param_3 = uStack4;
    }
    else {
      uVar1 = uStack4;
      pass1_1028_b58e(param_1);
      iVar2 = uStack4 * 0xc + iVar2;
      pass1_1030_6e9c(CONCAT22(extraout_DX,uVar1),0x1,(iVar2 + 0x24));
      param_3 = 0x0;
      (iVar2 + 0x20) = 0x0;
      (iVar2 + 0x24) = 0x0;
      (iVar2 + 0x26) = 0x0;
    }
  }
  pass1_1028_b46e(param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_cac2(param_1: *mut u32,param_2: i16,param_3: u16,param_4: u16,param_5: u16)
{
  let uVar1: u32;
  let puVar2: u32;
  code **ppcVar3;
  let uVar4: u32;
  let uVar5: u16;
  let uVar6: u32;
  let puVar7: u32;
  let uVar8: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let uVar9: u16;
  let uVar10: u16;
  let uStack34: u32;
  let iStack30: i16;
  let iStack28: i16;
  
  pass1_1028_be9e(param_1,param_3,param_4,&USHORT_1050_1028,param_5);
  uVar10 = (param_1 >> 0x10);
  if (((param_1 + 0x12) == 0x5) && (PTR_LOOP_1050_5812 == 0x0))
  {
    PTR_LOOP_1050_5812 = (&PTR_LOOP_1050_0000 + 0x1);
    pass1_1028_b58e(param_1 & 0xffff | uVar10 << 0x10);
    uVar1 = (param_2 + 0x2e);
    uVar6 = (uVar1 + 0x10);
    uVar10 = extraout_DX;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6,(uVar6 >> 0x10));
    puVar2 = (uVar6 + 0x1e);
    ppcVar3 = (code **)(*puVar2 + 0x10);
    puVar7 = puVar2;
    (**ppcVar3)(&USHORT_1050_1028,puVar2,(uVar6 + 0x20));
    uVar4 = puVar7 & 0xffff | extraout_DX_00 << 0x10;
    iStack28 = 0x0;
    iStack30 = pass1_1030_d144(param_1);
    uStack34 = 0x0;
    while ((uStack34 < uVar4 && (iStack30 != 0x0))) {
      ppcVar3 = (code **)(*puVar2 + 0x4);
      uVar8 = uVar4;
      (**ppcVar3)(&USHORT_1050_1028,puVar2,(puVar2 >> 0x10),
                  (char)uStack34,(uStack34 >> 0x10));
      uVar5 = uVar8;
      uVar9 = extraout_DX_01 | uVar5;
      if (uVar9 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,extraout_DX_01);
        uVar5 = (uVar5 + 0xc);
        if ((0x0 < uVar5) && (!SBORROW2(uVar5,0x1))) {
          if (uVar5 != 0x3 && 0x0 < (uVar5 - 0x2)) {
            if (uVar5 != 0x4) goto LAB_1030_cbbc;
            iStack28 += 0x1;
          }
          pass1_1030_6e9c(uVar6 & 0xffff | uVar10 << 0x10,0x1,uVar5);
          pass1_1030_d180(param_1,uVar5);
          iStack30 += -0x1;
        }
      }
LAB_1030_cbbc:
      uStack34 += 0x1;
    }
    while (iStack28 < 0x4) {
      pass1_1030_d180(param_1,0x4);
      iStack28 = iStack28 + 0x1;
    }
  }
  return;
}



fn pass1_1030_cbf0(param_1: i16,param_2: u16,param_3: i16) -> u16

{
  astruct_595 *iVar1;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x0;
    }
    iVar1 = (astruct_595 *)(iStack4 * 0xc + param_1);
    if ((iVar1->field_0x24 == param_3) && (iVar1->field_0x26 == 0x3)) break;
    iStack4 += 0x1;
  }
  iVar1->field_0x26 = 0x0;
  iVar1->field_0x28 = 0x0;
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_cc44(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: i16)
{
  code **ppcVar1;
  let iVar2: i16;
  let puVar3: *mut u8;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let puVar7: *mut u8
  let extraout_DX_01: u16;
  astruct_304 *iVar7;
  astruct_303 *iVar8;
  let uVar8: u8;
  let unaff_SS: u16;
  let puVar9: u32;
  let puVar10: u32;
  let puVar11: *mut u8
  let local_32: [u8;8];
  let puStack42: u32;
  let uStack38: u32;
  let uStack34: u32;
  let puStack30: u32;
  let uStack26: u16;
  let puStack24: *mut u8
  let uStack22: u16;
  let puStack20: *mut u8
  let puStack18: u32;
  let iStack14: i16;
  let uStack12: u16;
  let uStack10: i16;
  let uStack8: u32;
  let iStack4: i16;
  
  iStack4 = 0x0;
  uStack8 = (param_4 + 0x4);
  iStack10 = 0x0;
  do {
    if (0x9 < iStack10) {
      return;
    }
    iVar8 = (astruct_303 *)(iStack10 * 0xc + param_1);
    if (((iVar8->field_0x28 == uStack8) && (iVar8->field_0x2a == uStack8._2_2_)) &&
       (iVar8->field_0x24 == param_5)) {
      if (iVar8->field_0x26 == 0x4) {
        iVar2 = param_5;
        pass1_1028_b58e(CONCAT22(param_2,param_1));
        iStack14 = iVar2;
        uStack12 = extraout_DX_00;
        pass1_1030_6e9c(CONCAT13((char)(extraout_DX_00 >> 0x8),
                                 CONCAT12((char)extraout_DX_00,iStack14)),0x1,
                        iVar8->field_0x24);
        iVar8->field_0x20 = 0x0;
        iVar8->field_0x24 = 0x0;
        iVar8->field_0x26 = 0x0;
        puStack42 = 0x0;
        puStack18 = 0x0;
        _DAT_0000_0006 = param_5;
        uRam0000000a = 0x1;
        uVar4 = switch_1020_c3b4(param_5);
        (puStack18 + 0xc) = uVar4;
        puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
        puVar7 = (uchar *)(puVar10 >> 0x10);
        uVar6 = puVar10;
        uVar5 = uVar6;
        puVar11 = puVar7;
        uStack22 = uVar6;
        puStack20 = puVar7;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
        uVar8 = 0x38;
        uStack26 = uVar6;
        puStack24 = puVar7;
        pass1_1038_4d6e(CONCAT22(puVar7,uVar6),CONCAT22(puVar11,uVar5),uVar6,
                        puVar7);
        puStack30 = CONCAT22(puVar7,uVar6);
        ppcVar1 = (code **)(*puStack30 + 0x10);
        (**ppcVar1)(&PTR_LOOP_1050_1038,uVar6,puVar7);
        uStack34 = CONCAT22(extraout_DX_01,uVar6);
        uVar6 = extraout_DX_01;
        for (uStack38 = 0x0; uStack38 < uStack34; uStack38 += 0x1) {
          puVar9 = pass1_1030_1d7c(uStack34,uVar6,puStack30);
          uVar5 = (puVar9 >> 0x10);
          uVar6 = uVar5 | puVar9;
          if (uVar6 != 0x0) {
            puVar3 = local_32;
            ppcVar1 = (code **)(*puVar9 + 0x40);
            (**ppcVar1)(0x38,(char)puVar9,uVar5,puVar3);
            uVar6 = extraout_DX;
            if (puVar3 == 0x0) {
              uVar8 = 0x28;
              pass1_1028_6408(puVar9,puStack18,unaff_SS);
              break;
            }
          }
        }
        puStack42 = puStack30;
        if (puStack30 != 0x0) {
          ppcVar1 = (code **)*puStack30;
          (**ppcVar1)(uVar8,puStack30,(puStack30 >> 0x10),0x1);
        }
      }
      else {
        iVar7 = (astruct_304 *)(iStack10 * 0xc + param_1);
        iVar7->field_0x26 = 0x0;
        iVar7->field_0x28 = 0x0;
      }
      iStack4 += 0x1;
      param_3 += -0x1;
      if (param_3 == 0x0) {
        return;
      }
    }
    iStack10 += 0x1;
  } while( true );
}



i16  pass1_1030_cde8(param_1: i16,param_2: u16,param_3: i16)

{
  let iVar1: i16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return -0x1;
    }
    iVar1 = iStack4 * 0xc + param_1;
    if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0x0)) break;
    iStack4 += 0x1;
  }
  return iStack4;
}



// WARNING: Could not reconcile some variable overlaps

i16  pass1_1030_ce2e(param_1: i16,param_2: u16,param_3: i16)

{
  let iVar1: i16;
  let uStack6: u32;
  
  for (uStack6 = 0x0; uStack6 < 0xa;
      uStack6 = uStack6 & 0xffff0000 | (uStack6 + 0x1)) {
    iVar1 = uStack6 * 0xc + param_1;
    if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0x0)) {
      uStack6 = uStack6 & 0xffff | (uStack6._2_2_ + 0x1) << 0x10;
    }
  }
  return uStack6._2_2_;
}



fn pass1_1030_ce72(param_1: u32,param_2: i16,param_3: u32,param_4: i16)
{
  let lVar1: i32;
  astruct_300 *iVar2;
  let uStack10: i16;
  
  lVar1 = *(long *)(param_3 + 0x4);
  iStack10 = 0x0;
  do {
    if (0x9 < iStack10) {
      return;
    }
    iVar2 = (astruct_300 *)(iStack10 * 0xc + param_1);
    if ((iVar2->field_0x24 == param_4) && (iVar2->field_0x28 == 0x0)) {
      iVar2->field_0x28 = lVar1;
      if (param_4 == 0x4) {
        iVar2->field_0x26 = 0x2;
      }
      else {
        (param_1 + iStack10 * 0xc + 0x26) = 0x1;
      }
      param_2 += -0x1;
      if (param_2 == 0x0) {
        return;
      }
    }
    iStack10 += 0x1;
  } while( true );
}



fn pass1_1030_cef8(param_1: u32,param_2: u32,param_3: u16,param_4: i16)
{
  let uVar1: u16;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  (iVar2 + param_4 * 0xc + 0x26) = param_3;
  uVar4 = (param_2 >> 0x10);
  uVar1 = (param_2 + 0x6);
  (iVar2 + param_4 * 0xc + 0x28) = (param_2 + 0x4);
  (iVar2 + param_4 * 0xc + 0x2a) = uVar1;
  return;
}



fn pass1_1030_cf3a(param_1: u32,param_2: i16) -> u16

{
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x0;
    }
    if ((param_1 + iStack4 * 0xc + 0x24) == param_2) break;
    iStack4 += 0x1;
  }
  return 0x1;
}



fn pass1_1030_cf78(param_1: u32,param_2: u16)
{
  let uVar1: u32;
  let extraout_DX: u16;
  astruct_680 *iVar3;
  let uVar2: u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return;
    }
    uVar1 = param_2;
    uVar2 = (param_1 >> 0x10);
    if ((param_1 + iStack4 * 0xc + 0x24) == param_2) break;
    iStack4 += 0x1;
  }
  pass1_1028_b58e(param_1);
  if (param_2 == 0x5) {
    pass1_1038_4900((uVar1 + 0x2e));
  }
  else {
    pass1_1030_6e9c(uVar1 & 0xffff | extraout_DX << 0x10,0x1,param_2);
  }
  iVar3 = (astruct_680 *)(iStack4 * 0xc + param_1);
  iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  iVar3->field_0x26 = 0x0;
  return;
}



fn pass1_1030_d00c(param_1: i16,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let extraout_DX: u16;
  astruct_696 *local_BX_40;
  let iVar2: i16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return;
    }
    iVar2 = iStack4 * 0xc + param_1;
    if (((iVar2 + 0x26) == 0x0) &&
       (uVar1 = param_3, (iVar2 + 0x24) == param_3)) break;
    iStack4 += 0x1;
  }
  pass1_1028_b58e(CONCAT22(param_2,param_1));
  if (param_3 == 0x5) {
    pass1_1038_4900((uVar1 + 0x2e));
  }
  else {
    pass1_1030_6e9c(uVar1 & 0xffff | extraout_DX << 0x10,0x1,param_3);
  }
  local_BX_40 = (astruct_696 *)(iStack4 * 0xc + param_1);
  local_BX_40->field_0x20 = 0x0;
  local_BX_40->field_0x24 = 0x0;
  local_BX_40->field_0x26 = 0x0;
  return;
}



fn pass1_1030_d0a8(param_1: u32) -> u16

{
  let uVar1: u16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x98);
  pass1_1030_d56a(param_1 & 0xffff | uVar2 << 0x10);
  return uVar1;
}



// WARNING: Could not reconcile some variable overlaps

i16  pass1_1030_d0c6(param_1: u32)

{
  let uStack6: u32;
  
  for (uStack6 = 0x0; uStack6 < 0xa;
      uStack6 = uStack6 & 0xffff0000 | (uStack6 + 0x1)) {
    if (*(long *)(param_1 + uStack6 * 0xc + 0x20) != 0x0) {
      uStack6 = uStack6 & 0xffff | (uStack6._2_2_ + 0x1) << 0x10;
    }
  }
  return uStack6._2_2_;
}



// WARNING: Could not reconcile some variable overlaps

i16  pass1_1030_d102(param_1: i16,param_2: u16)

{
  let iVar1: i16;
  let uStack6: u32;
  
  for (uStack6 = 0x0; uStack6 < 0xa;
      uStack6 = uStack6 & 0xffff0000 | (uStack6 + 0x1)) {
    iVar1 = uStack6 * 0xc + param_1;
    if ((*(long *)(iVar1 + 0x20) != 0x0) && ((iVar1 + 0x26) != 0x0)) {
      uStack6 = uStack6 & 0xffff | (uStack6._2_2_ + 0x1) << 0x10;
    }
  }
  return uStack6._2_2_;
}



// WARNING: Could not reconcile some variable overlaps

i16  pass1_1030_d144(param_1: u32)

{
  let uStack6: u32;
  
  for (uStack6 = 0x0; uStack6 < 0xa;
      uStack6 = uStack6 & 0xffff0000 | (uStack6 + 0x1)) {
    if (*(long *)(param_1 + uStack6 * 0xc + 0x20) == 0x0) {
      uStack6 = uStack6 & 0xffff | (uStack6._2_2_ + 0x1) << 0x10;
    }
  }
  return uStack6._2_2_;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_d180(param_1: u32,param_2: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  let extraout_DX: *mut u8
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return;
    }
    uVar5 = (param_1 >> 0x10);
    uVar3 = param_1;
    if (((uVar3 + iStack4 * 0xc + 0x22) | (uVar3 + iStack4 * 0xc + 0x20)
        ) == 0x0) break;
    iStack4 += 0x1;
  }
  uVar2 = *_PTR_LOOP_1050_65e2;
  iVar1 = (_PTR_LOOP_1050_65e2 + 0x2);
  iVar4 = iStack4 * 0xc + uVar3;
  (iVar4 + 0x20) = uVar2 + 0xc8;
  (iVar4 + 0x22) = iVar1 + (0xff37 < uVar2);
  (iVar4 + 0x24) = param_2;
  uVar2 = param_2;
  pass1_1030_d340(uVar3,uVar5,param_1 & 0xffff0000 | (iVar4 + 0x20));
  pass1_1028_b58e(param_1);
  if (param_2 == 0x5) {
    pass1_1038_48e0((uVar2 + 0x2e),0x1);
    return;
  }
  pass1_1030_7c50(CONCAT22(extraout_DX,uVar2),0x1,param_2,uVar2,extraout_DX);
  return;
}



fn pass1_1030_d230(param_1: u32) -> u16

{
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x1;
    }
    if (*(long *)(param_1 + iStack4 * 0xc + 0x20) == 0x0) break;
    iStack4 += 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_d26c(param_1: u32,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u32;
  let extraout_DX: u16;
  let iVar5: i16;
  let iStack8: i16;
  
  uVar2 = *_PTR_LOOP_1050_65e2;
  for (iStack8 = 0x0; iStack8 < 0xa; iStack8 += 0x1) {
    iVar5 = iStack8 * 0xc + param_1;
    if ((((iVar5 + 0x22) | (iVar5 + 0x20)) != 0x0) &&
       (puVar1 = (iVar5 + 0x20), *puVar1 < uVar2 || *puVar1 == uVar2)) {
      uVar4 = uVar2;
      pass1_1030_d3b2(param_1,param_1._2_2_,iStack8,uVar2,param_2);
      iVar3 = uVar4;
      if (iVar3 == 0x0) {
        pass1_1028_b58e(param_1);
        if ((iVar5 + 0x24) == 0x5) {
          pass1_1038_4900((iVar3 + 0x2e));
        }
        else {
          pass1_1030_6e9c(CONCAT22(extraout_DX,iVar3),0x1,
                          (param_1 + iStack8 * 0xc + 0x24));
        }
        iVar5 = iStack8 * 0xc + param_1;
        (iVar5 + 0x20) = 0x0;
        (iVar5 + 0x24) = 0x0;
        (iVar5 + 0x26) = 0x0;
      }
    }
  }
  return;
}



fn pass1_1030_d340(param_1: u16,param_2: u16,param_3: u32)
{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_3 >> 0x10);
  iVar2 = param_3;
  iVar1 = (iVar2 + 0x4);
  if (((0x0 < iVar1) && (!SBORROW2(iVar1,0x1))) &&
     ((iVar1 == 0x4 || iVar1 + -0x1 < 0x3 || (iVar1 == 0xc)))) {
    (iVar2 + 0x6) = 0x0;
    return;
  }
  (iVar2 + 0x6) = 0x1;
  return;
}



fn pass1_1030_d36e(param_1: u32,param_2: i16) -> u16

{
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x0;
    }
    if ((iStack4 != param_2) && ((param_1 + iStack4 * 0xc + 0x24) == 0x8))
    break;
    iStack4 += 0x1;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_d3b2(param_1: i16,param_2: u16,param_3: i16,param_4: i16,param_5: u16)
{
  let iVar1: i16;
  code **ppcVar2;
  let bVar3: bool;
  let uVar4: u16;
  let uVar5: u16;
  let extraout_DX: u16;
  let puVar6: *mut u8
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let uVar7: u16;
  let uVar8: u16;
  let puVar9: u32;
  let uVar10: u32;
  let uVar11: u32;
  let puStack26: u32;
  let uStack18: u32;
  let uStack14: u32;
  
  pass1_1028_b58e(CONCAT22(param_2,param_1));
  uVar11 = (param_4 + 0x2e);
  uVar4 = pass1_1030_d36e(CONCAT22(param_2,param_1),param_3);
  if (uVar4 == 0x0) {
    puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
    puVar6 = (uchar *)(puVar9 >> 0x10);
    uVar5 = puVar9;
    pass1_1038_4d6e(uVar11,puVar9,uVar5,puVar6);
    puStack26 = CONCAT22(puVar6,uVar5);
    ppcVar2 = (code **)(*puStack26 + 0x10);
    uVar7 = uVar5;
    (**ppcVar2)(&PTR_LOOP_1050_1038,uVar5,puVar6);
    uStack18 = CONCAT22(extraout_DX_00,uVar7);
    bVar3 = false;
    for (uStack14 = 0x0; uStack14 < uStack18; uStack14 += 0x1) {
      uVar10 = pass1_1030_1d7c(uStack14,uStack14._2_2_,puStack26);
      uVar7 = (uVar10 >> 0x10);
      if ((((uVar7 | uVar10) != 0x0) &&
          (*(long *)(uVar10 + 0x4) != *(long *)(param_1 + 0x4))) &&
         (uVar4 = pass1_1030_cf3a(uVar10,0x8), uVar4 != 0x0)) {
        bVar3 = true;
        break;
      }
    }
    if (puStack26 != 0x0) {
      ppcVar2 = (code **)*puStack26;
      (**ppcVar2)(0x38,uVar5,puVar6,0x1);
    }
    if (!bVar3) {
      return;
    }
  }
  puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
  puVar6 = (uchar *)(puVar9 >> 0x10);
  uVar5 = puVar9;
  uVar8 = SUB42(&PTR_LOOP_1050_1038,0x0);
  pass1_1038_4d6e(uVar11,puVar9,uVar5,puVar6);
  puStack26 = CONCAT22(puVar6,uVar5);
  ppcVar2 = (code **)(*puStack26 + 0x10);
  uVar7 = uVar5;
  (**ppcVar2)(&PTR_LOOP_1050_1038,uVar5,puVar6);
  uStack18 = CONCAT22(extraout_DX_01,uVar7);
  bVar3 = false;
  uStack14 = 0x0;
  do {
    if (uStack18 <= uStack14) {
LAB_1030_d51b:
      if (puStack26 != 0x0) {
        ppcVar2 = (code **)*puStack26;
        (**ppcVar2)(uVar8,(char)uVar5,(char)puVar6,0x1);
      }
      if (!bVar3) {
        return;
      }
      uVar5 = *_PTR_LOOP_1050_65e2;
      iVar1 = (_PTR_LOOP_1050_65e2 + 0x2);
      (param_1 + param_3 * 0xc + 0x20) = uVar5 + 0xc8;
      (param_1 + param_3 * 0xc + 0x22) = iVar1 + (0xff37 < uVar5);
      return;
    }
    uVar11 = pass1_1030_1d7c(uStack14,uStack14._2_2_,puStack26);
    uVar7 = (uVar11 >> 0x10) | uVar11;
    if (uVar7 != 0x0) {
      uVar8 = SUB42(&USHORT_1050_1028,0x0);
      uVar4 = pass1_1028_6744(param_5,uVar11,0x7);
      if ((uVar7 | uVar4) != 0x0) {
        uVar8 = SUB42(&USHORT_1050_1028,0x0);
        pass1_1028_6228(uVar11,0x1,0x0,0x7,param_5);
        bVar3 = true;
        goto LAB_1030_d51b;
      }
    }
    uStack14 += 0x1;
  } while( true );
}



i16  pass1_1030_d56a(param_1: u32)

{
  let iVar1: i16;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  iVar2 = (iVar3 + 0x98) + -0x1;
  iVar1 = iVar2;
  if (false) {
switchD_1030_d5fb_caseD_4:
    (iVar3 + 0x98) = 0x1;
    return iVar1;
  }
  iVar1 = iVar3;
  switch(iVar2) {
  case 0x0:
    (iVar3 + 0x98) = 0x2;
    break;
  case 0x1:
    (iVar3 + 0x98) = 0x3;
    break;
  case 0x2:
    (iVar3 + 0x98) = 0x4;
    break;
  case 0x3:
    (iVar3 + 0x98) = 0xc;
    break;
  default:
    goto switchD_1030_d5fb_caseD_4;
  case 0x7:
    (iVar3 + 0x98) = 0x9;
    return iVar3;
  case 0x8:
    (iVar3 + 0x98) = 0xb;
    return iVar3;
  case 0xa:
    (iVar3 + 0x98) = 0x5;
    return iVar3;
  case 0xb:
    (iVar3 + 0x98) = 0x8;
    return iVar3;
  }
  return iVar3;
}



fn pass1_1030_d61c(param_1: u32,param_2: u32,param_3: u16)
{
  let BVar1: bool;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let local_1a: u32;
  let local_16: *mut u8;
  let local_14: u16;
  let local_12: [u32;0x3];
  let iStack4: i16;
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    for (iStack4 = 0x0; uVar4 = param_2, uVar5 = (param_2 >> 0x10),
        iStack4 < 0xa; iStack4 += 0x1) {
      uVar3 = (param_1 >> 0x10);
      iVar2 = param_1;
      local_12[0] = (iVar2 + iStack4 * 0xc + 0x20);
      BVar1 = write_to_file_1008_7e1c
                        (uVar4,uVar5,local_12,param_3,0x4,0x1008);
      if (BVar1 == 0x0) goto LAB_1030_d701;
      local_14 = (iVar2 + iStack4 * 0xc + 0x24);
      BVar1 = write_to_file_1008_7e1c
                        (uVar4,uVar5,&local_14,param_3,0x2,0x1008);
      if (BVar1 == 0x0) goto LAB_1030_d701;
      local_16 = (iVar2 + iStack4 * 0xc + 0x26);
      BVar1 = write_to_file_1008_7e1c
                        (uVar4,uVar5,&local_16,param_3,0x2,0x1008);
      if (BVar1 == 0x0) goto LAB_1030_d701;
      local_1a = (iVar2 + iStack4 * 0xc + 0x28);
      BVar1 = write_to_file_1008_7e1c
                        (uVar4,uVar5,&local_1a,param_3,0x4,0x1008);
      if (BVar1 == 0x0) goto LAB_1030_d701;
    }
    local_16 = PTR_LOOP_1050_5812;
    BVar1 = write_to_file_1008_7e1c
                      (uVar4,uVar5,&local_16,param_3,0x2,0x1008);
    if (BVar1 != 0x0) {
      return;
    }
LAB_1030_d701:
    PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1030_d72e(param_1: u32,param_2: u32,param_3: i16,uchar *param_4,param_5: u16)
{
  let uVar1: u16;
  let BVar2: bool;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let uStack10: i16;
  let local_8: u32;
  let local_4: u16;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 == 0x0) {
    return;
  }
  iStack10 = 0x0;
  while( true ) {
    uVar4 = param_2;
    uVar5 = (param_2 >> 0x10);
    if (0x9 < iStack10) {
      if ((0x3 < PTR_LOOP_1050_0312) &&
         (BVar2 = read_file_1008_7dee(uVar4,uVar5,&PTR_LOOP_1050_5812,0x0,
                                      &USHORT_1050_1050,0x2,0x1008), BVar2 == 0x0)
         ) {
        PTR_LOOP_1050_0310 = 0x6d2;
        return;
      }
      return;
    }
    BVar2 = read_file_1008_7dee(uVar4,uVar5,&local_8,0x0,param_5,0x4,0x1008);
    if (BVar2 == 0x0) {
      PTR_LOOP_1050_0310 = 0x6d2;
      return;
    }
    BVar2 = read_file_1008_7dee(uVar4,uVar5,&local_4,0x0,param_5,0x2,0x1008);
    if (BVar2 == 0x0) break;
    iVar3 = iStack10 * 0xc + param_1;
    (iVar3 + 0x20) = local_8;
    (iVar3 + 0x22) = local_8._2_2_;
    uVar1 = switch_1008_72bc(uVar4,uVar5,local_4);
    (iVar3 + 0x24) = uVar1;
    if (PTR_LOOP_1050_0312 < 0x2) {
      iVar3 = iStack10 * 0xc + param_1;
      (iVar3 + 0x26) = 0x3;
      (iVar3 + 0x28) = 0x0;
    }
    else {
      BVar2 = read_file_1008_7dee(uVar4,uVar5,&local_4,0x0,param_5,0x2,0x1008);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = 0x6d2;
        return;
      }
      BVar2 = read_file_1008_7dee(uVar4,uVar5,&local_8,0x0,param_5,0x4,0x1008);
      if (BVar2 == 0x0) {
        PTR_LOOP_1050_0310 = 0x6d2;
        return;
      }
      iVar3 = iStack10 * 0xc + param_1;
      (iVar3 + 0x26) = local_4;
      (iVar3 + 0x28) = local_8;
    }
    iStack10 += 0x1;
  }
  PTR_LOOP_1050_0310 = 0x6d2;
  return;
}



astruct_18 *  pass1_1030_d868(astruct_18 *param_1,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1030_d942(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u32

{
  pass1_1028_b39e((u16 *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xdc2e;
  (param_1 + 0x2) = 0x1030;
  if ((param_1 + 0xc) == 0x4c) {
    (param_1 + 0xe) = 0x43;
  }
  else {
    if ((param_1 + 0xc) == 0x4d) {
      (param_1 + 0xe) = 0x44;
    }
    else {
      (param_1 + 0xe) = 0x45;
    }
  }
  return CONCAT22(param_2,param_1);
}



fn pass1_1030_d994(param_1: *mut u32,param_2: u16,param_3: u16,param_4: u16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let iVar3: i16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u32;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if ((iVar4 + 0x12) != 0x4) {
    return;
  }
  uVar6 = pass1_1028_b4f2(param_1);
  iVar3 = uVar6;
  if (*(long *)(iVar3 + 0x200) == 0x8000002) {
    uVar2 = (iVar4 + 0x14);
    piVar1 = (i16 *)(uVar2 + 0x94);
    *piVar1 = *piVar1 + -0x1;
  }
  else {
    pass1_1028_cb04(param_1,param_2,param_3,param_4);
    if (iVar3 == 0x0) {
      return;
    }
    pass1_1030_dace(param_1,param_4);
    if (iVar3 == 0x0) {
      return;
    }
    uVar2 = (iVar4 + 0x14);
    piVar1 = (i16 *)(uVar2 + 0x94);
    *piVar1 = *piVar1 + -0x1;
    pass1_1028_c952(param_1,param_2,param_3,param_4);
    pass1_1030_da22(param_1,param_4);
  }
  uVar2 = (iVar4 + 0x14);
  if ((uVar2 + 0x94) < 0x1) {
    pass1_1028_bdac(param_1,0x5,&USHORT_1050_1028);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_da22(param_1: u32,param_2: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u16;
  let Bvar4: bool;
  let uVar5: u16;
  let puVar6: u32;
  let extraout_DX: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u32;
  let uStack18: u32;
  
  uVar9 = pass1_1028_b4f2(param_1);
  uVar3 = (uVar9 >> 0x10);
  puVar1 = (uVar9 + 0xc);
  ppcVar2 = (code **)(*puVar1 + 0x10);
  puVar6 = puVar1;
  (**ppcVar2)(&USHORT_1050_1028,puVar1,(uVar9 + 0xe));
  uStack18 = 0x0;
  while( true ) {
    if ((puVar6 & 0xffff | extraout_DX << 0x10) <= uStack18) {
      return;
    }
    uVar9 = pass1_1030_1d7c((puVar6 & 0xffff),extraout_DX,puVar1);
    uVar7 = (uVar9 >> 0x10);
    uVar8 = uVar7 | uVar9;
    if (((uVar8 != 0x0) &&
        (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar9 + 0xc),
                                 0x4), BVar4 != 0x0)) &&
       (uVar5 = pass1_1028_6744(param_2,uVar9,0xd), (uVar8 | uVar5) != 0x0)) break;
    uStack18 += 0x1;
  }
  pass1_1028_6228(uVar9,0x1,0x0,0xd,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_dace(param_1: u32,param_2: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u16;
  let Bvar4: bool;
  let uVar5: u16;
  let puVar6: u32;
  let extraout_DX: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u32;
  let uStack20: u32;
  
  uVar9 = pass1_1028_b4f2(param_1);
  uVar3 = (uVar9 >> 0x10);
  puVar1 = (uVar9 + 0xc);
  ppcVar2 = (code **)(*puVar1 + 0x10);
  puVar6 = puVar1;
  (**ppcVar2)(&USHORT_1050_1028,puVar1,(uVar9 + 0xe));
  uStack20 = 0x0;
  uVar8 = extraout_DX;
  do {
    if ((puVar6 & 0xffff | extraout_DX << 0x10) <= uStack20) {
      return;
    }
    uVar9 = pass1_1030_1d7c((puVar6 & 0xffff),uVar8,puVar1);
    uVar7 = (uVar9 >> 0x10);
    uVar8 = uVar7 | uVar9;
    if ((uVar8 != 0x0) &&
       (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar9 + 0xc),0x4
                               ), BVar4 != 0x0)) {
      uVar5 = pass1_1028_6744(param_2,uVar9,0xd);
      uVar8 |= uVar5;
      if (uVar8 != 0x0) {
        return;
      }
    }
    uStack20 += 0x1;
  } while( true );
}



fn pass1_1030_db72(void) -> u16

{
  return 0x1;
}



fn pass1_1030_db78(param_1: u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x12) == 0x6) {
    pass1_1028_bdac((param_1 & 0xffff | uVar1 << 0x10),0x5,
                    &USHORT_1050_1028);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1030_db92(param_1: u16,param_2: u16,param_3: *mut u16,param_4: u32,param_5: i32,
               param_6: u16)

{
  let iVar1: i16;
  let puVar2: *mut u8;
  let uVar3: u16;
  let uVar4: u32;
  let local_4: [u8;2];
  
  uVar4 = pass1_1030_bcae(local_4,param_6);
  uVar3 = (uVar4 >> 0x10);
  iVar1 = uVar4;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4,(param_4 >> 0x10));
  uVar4 = (iVar1 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,(uVar4 >> 0x10));
  puVar2 = local_4;
  pass1_1030_bcde(param_6,puVar2,param_6,uVar4 & 0xffff | uVar3 << 0x10,
                  param_3,param_5);
  if (puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = 0x6af;
    return;
  }
  return;
}



fn pass1_1030_dc02(void) -> u16

{
  return 0x1;
}



astruct_18 *  pass1_1030_dc08(astruct_18 *param_1,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1030_dcc2(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e((u16 *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  (param_1 + 0x20) = 0x0;
  CONCAT22(param_2,param_1) = 0xe036;
  (param_1 + 0x2) = 0x1030;
  return (u16 *)CONCAT22(param_2,param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_dcf4(param_1: *mut u16,param_2: u16)
{
  let lVar1: i32;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let extraout_DX: u16;
  let uVar6: u16;
  let puVar7: *mut u8
  let extraout_DX_00: u16;
  let uVar8: u16;
  astruct_596 *iVar9;
  let uVar9: u16;
  let puVar10: u32;
  let uVar11: u32;
  let uStack28: u32;
  let uStack24: u32;
  let puStack20: u32;
  let iStack12: i16;
  
  uVar9 = (param_1 >> 0x10);
  iVar9 = (astruct_596 *)param_1;
  *param_1 = 0xe036;
  iVar9->field_0x2 = 0x1030;
  if (_PTR_LOOP_1050_65e2 != 0x0) {
    pass1_1028_b58e(param_1);
    if (iVar9->field_0x20 == 0x0) {
      uVar3 = extraout_DX | param_2;
      if (uVar3 == 0x0) {
        uVar6 = extraout_DX;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
      }
      else {
        uVar3 = (param_2 + 0x2e);
        uVar6 = (param_2 + 0x30);
      }
      puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
      puVar7 = (uchar *)(puVar10 >> 0x10);
      uVar4 = puVar10;
      pass1_1038_4d6e(CONCAT22(uVar6,uVar3),puVar10,uVar4,puVar7);
      puStack20 = CONCAT22(puVar7,uVar4);
      ppcVar2 = (code **)(*puStack20 + 0x10);
      uVar6 = uVar4;
      (**ppcVar2)(&PTR_LOOP_1050_1038,uVar4,puVar7);
      uStack24 = CONCAT22(extraout_DX_00,uVar6);
      uVar3 = extraout_DX_00;
      for (uStack28 = 0x0; uStack28 < uStack24; uStack28 += 0x1) {
        uVar11 = pass1_1030_1d7c(uVar6,uVar3,puStack20);
        uVar8 = (uVar11 >> 0x10);
        uVar3 = uVar8 | uVar11;
        if (uVar3 != 0x0) {
          uVar5 = pass1_1030_dfcc(param_1);
          uVar5 = pass1_1030_cbf0(uVar11,uVar8,uVar5);
          if (uVar5 != 0x0) break;
        }
      }
      if (puStack20 != 0x0) {
        ppcVar2 = (code **)*puStack20;
        (**ppcVar2)(0x38,uVar4,puVar7,0x1);
      }
    }
    else {
      lVar1 = iVar9->field_0x20;
      uVar3 = extraout_DX;
      uVar6 = param_2;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,lVar1,(lVar1 >> 0x10));
      if ((uVar3 | uVar6) != 0x0) {
        iStack12 = 0x0;
        switch(iVar9->field_0xc) {
        case 0x73:
        case 0x77:
          iStack12 = 0x1;
          break;
        case 0x74:
        case 0x78:
          iStack12 = 0x2;
          break;
        case 0x75:
          iStack12 = 0x3;
          break;
        case 0x76:
          iStack12 = 0x5;
        }
        if (iStack12 != 0x0) {
          pass1_1030_cc44(uVar6,uVar3,0x1,CONCAT22(extraout_DX,param_2),iStack12);
        }
      }
    }
  }
  pass1_1028_b418(param_1);
  return;
}



fn pass1_1030_de7c(param_1: u32,param_2: u32,param_3: u16)
{
  let BVar1: bool;
  let local_10: [u32;0x3];
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    local_10[0] = (param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c
                      (param_2,(param_2 >> 0x10),local_10,param_3,
                       0x4,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = 0x6d0;
      return;
    }
  }
  return;
}



fn pass1_1030_dec4(param_1: u32,param_2: u32,param_3: i16,uchar *param_4,param_5: u16)
{
  let BVar1: bool;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (((param_3 != 0x0) && (0x1 < PTR_LOOP_1050_0312)) &&
     (BVar1 = read_file_1008_7dee(param_2,(param_2 >> 0x10),
                                  param_1 + 0x20,0x0,(param_1 >> 0x10),0x4,
                                  0x1008), BVar1 == 0x0)) {
    PTR_LOOP_1050_0310 = 0x6d2;
    return;
  }
  return;
}



fn pass1_1030_df0c(param_1: u32,param_2: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let lVar3: i32;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u32;
  let extraout_DX: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let uStack24: u16;
  let uStack22: u16;
  let uStack14: u16;
  let uStack10: u16;
  
  pass1_1028_b58e(param_1);
  uVar1 = (param_2 + 0x2e);
  uStack10 = uVar1;
  if (((param_2 + 0x30) | uStack10) != 0x0) {
    uVar9 = (uVar1 >> 0x10);
    uVar1 = (uStack10 + 0x210);
    uVar7 = (uStack10 + 0x212);
    uStack14 = uVar1;
    if ((uVar7 | uStack14) != 0x0) {
      uVar2 = (uStack14 + 0xa);
      uVar4 = pass1_1030_dfcc(param_1);
      if (uVar4 != 0x0) {
        uStack24 = 0x1;
        uStack22 = 0x0;
        while (CONCAT22(uStack22,uStack24) < uVar2) {
          uVar6 = uVar2;
          uVar10 = uVar4;
          bad_1030_1312();
          uVar8 = uVar7;
          iVar5 = pass1_1030_cde8(uVar6,uVar7,uVar10);
          if (-0x1 < iVar5) {
            pass1_1030_cef8(uVar6 & 0xffff | uVar7 << 0x10,
                            CONCAT22(extraout_DX,param_2),0x1,iVar5);
            (param_1 + 0x20) = (uVar6 + 0x4);
            return;
          }
          lVar3 = CONCAT22(uStack22,uStack24) + 0x1;
          uStack24 = lVar3;
          uVar7 = uVar8;
          uStack22 = (lVar3 >> 0x10);
        }
      }
    }
  }
  return;
}



fn pass1_1030_dfcc(param_1: u32) -> u16

{
  let iVar1: i16;
  let uStack4: u16;
  
  iVar1 = (param_1 + 0xc);
  if (iVar1 == 0x73) {
LAB_1030_dfde:
    uStack4 = 0x1;
  }
  else {
    if (iVar1 != 0x74) {
      if (iVar1 == 0x75) {
        return 0x3;
      }
      if (iVar1 == 0x77) goto LAB_1030_dfde;
      if (iVar1 != 0x78) {
        return 0x0;
      }
    }
    uStack4 = 0x2;
  }
  return uStack4;
}



astruct_18 *  pass1_1030_e010(astruct_18 *param_1,param_2: u8)

{
  let in_AX: u16;
  
  pass1_1030_dcf4((u16 *)param_1,in_AX);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * 
pass1_1030_e09e(astruct_100 *param_1,param_2: u16,param_3: u8)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x2af7);
  param_1->field_0x0 = 0xe2ae;
  (param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (param_1 + 0x8)),
             s_SCAiInput_1050_5972);
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_e0d4(uchar *param_1,param_2: u16,param_3: i16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let uVar3: u16;
  let puVar4: *mut u8;
  let puVar5: *mut u8;
  let uVar6: u16;
  let extraout_DX: u16;
  let puVar7: *mut u8
  let puVar8: *mut u8
  let iVar9: i16;
  let uVar10: u16;
  let puVar11: *mut u16;
  let uStack42: u32;
  let local_1c: [u8;8];
  let uStack20: u32;
  let uStack16: u16;
  let uStack14: u32;
  let uStack10: u32;
  let iStack6: i16;
  let uStack4: u16;
  
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x40,param_2,param_1,param_3);
  uStack4 = (puVar11 >> 0x10);
  iStack6 = puVar11;
  uStack10 = pass1_1008_b820(puVar11,iStack6,uStack4);
  uVar3 = uStack10;
  uVar6 = (uStack10 >> 0x10) | uVar3;
  if (uVar6 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x800);
    uStack14 = CONCAT22(uVar6,uVar3);
    uStack16 = ((uVar3 + 0x154) != 0x0);
    pass1_1008_5784(CONCAT22(param_2,local_1c),uStack10);
    while( true ) {
      puVar4 = local_1c;
      pass1_1008_5b12(puVar4,param_2);
      uStack20 = CONCAT22(extraout_DX,puVar4);
      puVar7 = (uchar *)(extraout_DX | puVar4);
      if (puVar7 == (uchar *)0x0) break;
      if ((puVar4 + 0x8) != 0x0) {
        uVar2 = (puVar4 + 0xa);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uVar2 >> 0x10));
        puVar8 = puVar7;
        puVar5 = puVar4;
        pass1_1038_354a(CONCAT22(puVar7,puVar4),puVar4,puVar7);
        if (puVar5 != 0x0) {
          uVar10 = (uStack20 >> 0x10);
          if (uStack16 == 0x0) {
            iVar9 = (uStack20 + 0xe) * 0xc;
            uStack42 = (iVar9 + 0x58c4);
            uVar3 = (iVar9 + 0x58c8);
          }
          else {
            iVar9 = (uStack20 + 0xe) * 0xc;
            uStack42 = (iVar9 + 0x58be);
            uVar3 = (iVar9 + 0x58c2);
          }
          uVar6 = uVar3;
          pass1_1038_35a8(CONCAT22(puVar7,puVar4),
                          
                           ((uStack20 + 0x10) * 0x2 + uStack42),uVar3,
                          puVar8);
          if (uVar6 != 0x0) {
            uVar10 = (uStack20 >> 0x10);
            iVar9 = uStack20;
            piVar1 = (i16 *)(iVar9 + 0x10);
            *piVar1 = *piVar1 + 0x1;
            if (uVar3 <= (iVar9 + 0x10)) {
              (iVar9 + 0x10) = 0x0;
            }
          }
        }
      }
    }
  }
  return;
}



fn pass1_1030_e1f4(param_1: u32,param_2: u16,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  let puVar3: u32;
  let iVar4: i16;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x108,param_3,0x1000);
  puStack10 = (u16 *)CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    (param_2 + 0x2) = 0x1008;
    uVar6 = (param_1 >> 0x10);
    (param_2 + 0x4) = (param_1 + 0x4);
    puVar3 = (param_1 + 0x8);
    puVar5 = (param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_2 + 0x2) = &USHORT_1050_1028;
    *puStack10 = 0xe2ae;
    (param_2 + 0x2) = 0x1030;
  }
  return;
}



astruct_18 *  pass1_1030_e282(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void 
struct_1030_e2be(astruct_100 *param_1,param_2: u16,param_3: u32,param_4: u32,
                param_5: u16,param_6: u8)

{
  astruct_217 *iVar1;
  let uVar1: u16;
  
  struct_op_1028_d1dc(param_5,param_6,param_1,0x2af7);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_217 *)param_1;
  iVar1->field_0x108 = param_4;
  iVar1->field_0x10c = param_3;
  iVar1->field_0x110 = param_2;
  param_1->field_0x0 = 0xe4ea;
  iVar1->field_0x2 = 0x1030;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_e300(param_1: u32,uchar *param_2,param_3: i16,param_4: u16) -> u16

{
  let puVar1: *mut u16;
  
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,param_2,param_3);
  pass1_1010_089e(param_4,puVar1,(param_1 + 0x110),0x2);
  return 0x1;
}



fn pass1_1030_e328(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: u8) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x110) == 0x0) {
    pass1_1030_e4ba(param_1);
  }
  else {
    pass1_1030_e410(param_4,param_2,param_5,param_3,
                    param_1 & 0xffff | uVar1 << 0x10);
  }
  return 0x1;
}



fn pass1_1030_e34e(param_1: u32,param_2: u16,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  astruct_404 *in_AX;
  let iVar3: i16;
  astruct_403 *iVar5;
  let puVar4: u32;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x112,param_3,0x1000);
  puStack10 = (u16 *)CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    (param_2 + 0x2) = 0x1008;
    uVar6 = (param_1 >> 0x10);
    iVar5 = (astruct_403 *)param_1;
    (param_2 + 0x4) = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = (param_2 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_2 + 0x2) = &USHORT_1050_1028;
    (param_2 + 0x108) = iVar5->field_0x108;
    (param_2 + 0x10c) = iVar5->field_0x10c;
    (param_2 + 0x110) = iVar5->field_0x110;
    *puStack10 = 0xe4ea;
    (param_2 + 0x2) = 0x1030;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_e410(param_1: u16,param_2: u16,param_3: u8,param_4: u16,param_5: u32)
{
  let uVar1: u32;
  let puVar2: *mut u8
  let uVar3: u16;
  let puVar4: *mut u16;
  let local_10: [u8;6];
  let local_a: [u8;4];
  let uStack6: u16;
  let uStack4: u16;
  
  uVar1 = (param_5 + 0x10c);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  puVar2 = (uchar *)(param_4 | param_2);
  if (puVar2 != (uchar *)0x0) {
    uStack6 = param_2;
    uStack4 = param_4;
    pass1_1038_4fd8(param_2,CONCAT22(param_4,param_2),0x21);
    if (param_2 == 0x0) {
      pass1_1020_a43e(param_1,puVar2,(u16 *)CONCAT22(param_1,local_a));
      puVar4 = pass1_1008_3e54((u16 *)CONCAT22(param_1,local_10),0x0,0x2,0xfffd);
      uVar3 = (puVar4 >> 0x10);
      pass1_1020_a49a(param_1,param_3,uVar3,CONCAT22(param_1,local_a),
                      (i16 *)CONCAT22(param_1,local_10),0x7a);
      pass1_1008_3e76((u16 *)CONCAT22(param_1,local_10),0x0,0x3,0xfffe);
      pass1_1020_a49a(param_1,param_3,uVar3,CONCAT22(param_1,local_a),
                      (i16 *)CONCAT22(param_1,local_10),0x7a);
      pass1_1008_3e76((u16 *)CONCAT22(param_1,local_10),0x0,0x3,0xfffd);
      pass1_1020_a49a(param_1,param_3,uVar3,CONCAT22(param_1,local_a),
                      (i16 *)CONCAT22(param_1,local_10),0x21);
    }
  }
  return;
}



fn pass1_1030_e4ba(void)
{
  return;
}




fn pass1_1030_e540(void) -> u16

{
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_e546(param_1: u32,param_2: u16) -> u16

{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0x108);
  pass1_1028_e332(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10),param_2);
  return 0x1;
}



fn pass1_1030_e564(param_1: u32,param_2: u16,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  let iVar3: i16;
  astruct_405 *iVar5;
  let puVar4: u32;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x10c,param_3,0x1000);
  puStack10 = (u16 *)CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    (param_2 + 0x2) = 0x1008;
    uVar6 = (param_1 >> 0x10);
    iVar5 = (astruct_405 *)param_1;
    (param_2 + 0x4) = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = (param_2 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_2 + 0x2) = &USHORT_1050_1028;
    (param_2 + 0x108) = iVar5->field_0x108;
    *puStack10 = 0xe62e;
    (param_2 + 0x2) = 0x1030;
  }
  return;
}



astruct_18 *  pass1_1030_e602(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * 
pass1_1030_e63e(astruct_100 *param_1,param_2: u16,param_3: u16,param_4: u8)

{
  let iVar1: i16;
  let uVar2: u16;
  
  iVar1 = param_1;
  uVar2 = (param_1 >> 0x10);
  struct_op_1028_d1dc(param_3,param_4,param_1,0xf9f);
  (iVar1 + 0x108) = param_2;
  param_1->field_0x0 = 0xe78a;
  (iVar1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (iVar1 + 0x8)),
             s_SCKillColony_1050_5990);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_e67c(param_1: u32,uchar *param_2,param_3: i16,param_4: u16) -> u16

{
  let uVar1: u16;
  astruct_67 *paVar2;
  
  paVar2 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,param_2,param_3)
  ;
  uVar1 = pass1_1008_aaa8(paVar2,(paVar2 >> 0x10),
                          (param_1 + 0x108));
  if (uVar1 != 0x0) {
    post_win_msg_1008_a0e4(paVar2,0x0,0x0,0x1,0x0,uVar1,0x1008,param_4);
  }
  return 0x1;
}



fn pass1_1030_e6c2(param_1: u32,param_2: u16,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  let iVar3: i16;
  astruct_406 *iVar5;
  let puVar4: u32;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x10a,param_3,0x1000);
  puStack10 = (u16 *)CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    (param_2 + 0x2) = 0x1008;
    uVar6 = (param_1 >> 0x10);
    iVar5 = (astruct_406 *)param_1;
    (param_2 + 0x4) = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = (param_2 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_2 + 0x2) = &USHORT_1050_1028;
    (param_2 + 0x108) = iVar5->field_0x108;
    *puStack10 = 0xe78a;
    (param_2 + 0x2) = 0x1030;
  }
  return;
}



astruct_18 *  pass1_1030_e75e(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * 
pass1_1030_e79a(astruct_100 *param_1,param_2: u16,param_3: u8)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0xf9f);
  param_1->field_0x0 = 0xe890;
  (param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (param_1 + 0x8)),
             s_SCKillRebelColony_1050_599e);
  return param_1;
}



fn pass1_1030_e7d0(void) -> u16

{
  return 0x1;
}



fn pass1_1030_e7d6(param_1: u32,param_2: u16,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  let puVar3: u32;
  let iVar4: i16;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x108,param_3,0x1000);
  puStack10 = (u16 *)CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    (param_2 + 0x2) = 0x1008;
    uVar6 = (param_1 >> 0x10);
    (param_2 + 0x4) = (param_1 + 0x4);
    puVar3 = (param_1 + 0x8);
    puVar5 = (param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_2 + 0x2) = &USHORT_1050_1028;
    *puStack10 = 0xe890;
    (param_2 + 0x2) = 0x1030;
  }
  return;
}



astruct_18 *  pass1_1030_e864(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void 
pass1_1030_e8a0(astruct_100 *param_1,param_2: u32,param_3: u16,param_4: u32,
               param_5: u16,param_6: u8)

{
  astruct_408 *iVar1;
  let puVar1: *mut u8
  
  struct_op_1028_d1dc(param_5,param_6,param_1,0x2710);
  puVar1 = (uchar *)(param_1 >> 0x10);
  iVar1 = (astruct_408 *)param_1;
  iVar1->field_0x108 = param_2;
  iVar1->field_0x10c = param_4;
  iVar1->field_0x110 = param_3;
  param_1->field_0x0 = 0xeb40;
  iVar1->field_0x2 = 0x1030;
  sys_1000_3f9c(&iVar1->field_0x8,puVar1,s_SCMoveBas_to_0x_08lx_1050_59b0,
                &USHORT_1050_1050,iVar1->field_0x10c,&stack0xfffe,puVar1,
                0x1000,param_5,param_6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 
pass1_1030_e8f8(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u32;
  astruct_18 *paStack20;
  let uStack6: u32;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (*(long *)(iVar4 + 0x108) != 0x0) {
    uVar3 = (iVar4 + 0x10c);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,(uVar3 >> 0x10));
    uStack6 = CONCAT22(param_3,param_2);
    uVar6 = struct_op_1030_73a8(CONCAT22(param_3,param_2));
    if ((uVar6 + 0xc) == (iVar4 + 0x110)) {
      pass1_1030_ea50(param_1,uStack6,param_4,param_5,param_6);
    }
    uVar1 = (iVar4 + 0x108);
    uVar2 = (iVar4 + 0x10a);
    paStack20 = (astruct_18 *)CONCAT22(uVar2,uVar1);
    if ((uVar2 | uVar1) != 0x0) {
      fn_ptr_1020_ba7e(CONCAT22(uVar2,uVar1));
      fn_ptr_1000_17ce(paStack20,0x1000);
    }
    (iVar4 + 0x108) = 0x0;
  }
  return 0x1;
}



fn pass1_1030_e98e(param_1: u32,param_2: u16,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  let iVar3: i16;
  astruct_407 *iVar5;
  let puVar4: u32;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x112,param_3,0x1000);
  puStack10 = (u16 *)CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    (param_2 + 0x2) = 0x1008;
    uVar6 = (param_1 >> 0x10);
    iVar5 = (astruct_407 *)param_1;
    (param_2 + 0x4) = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = (param_2 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_2 + 0x2) = &USHORT_1050_1028;
    (param_2 + 0x108) = iVar5->field_0x108;
    (param_2 + 0x10c) = iVar5->field_0x10c;
    (param_2 + 0x110) = iVar5->field_0x110;
    *puStack10 = 0xeb40;
    (param_2 + 0x2) = 0x1030;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_ea50(param_1: u32,param_2: u32,param_3: u16,param_4: u16,param_5: u16)
{
  let uVar1: u32;
  let BVar2: bool;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u32;
  let local_12: u32;
  let local_e: u16;
  let iStack12: i16;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u32;
  
  uStack6 = 0x1869f;
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(iVar3 + 0x110),0x3);
  if (BVar2 != 0x0) {
    uVar5 = struct_op_1030_73a8(param_2);
    iStack12 = (uVar5 >> 0x10);
    local_e = uVar5;
    uStack6 = pass1_1028_45e2(uVar5,local_e,iStack12,param_5);
  }
  uVar1 = (iVar3 + 0x108);
  uStack8 = (uVar1 + 0x4);
  uStack10 = 0x0;
  while( true ) {
    if (uStack8 <= uStack10) {
      return;
    }
    pass1_1020_bb16(*(u32 **)(iVar3 + 0x108),CONCAT22(param_5,&local_12),
                    (u16 *)CONCAT22(param_5,&local_e),uStack10);
    if (uStack6 < local_12) {
      pass1_1030_7ddc(param_2,uStack6,local_e,uStack6,uStack6._2_2_,param_3,param_4,
                      param_5);
      uStack6 = 0x0;
    }
    else {
      uStack6 -= local_12;
      pass1_1030_7ddc(param_2,local_12,local_e,local_12,uStack6._2_2_,param_3,
                      param_4,param_5);
    }
    if ((uStack6._2_2_ | uStack6) == 0x0) break;
    uStack10 += 0x1;
  }
  return;
}



astruct_18 *  pass1_1030_eb14(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * 
pass1_1030_eb50(astruct_100 *param_1,param_2: u16,param_3: u8)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x1f3f);
  param_1->field_0x0 = 0xecb2;
  (param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (param_1 + 0x8)),
             s_SCMines_1050_59c6);
  return param_1;
}



fn pass1_1030_eb86(param_1: u16,param_2: u16) -> u16

{
  let iVar1: i16;
  code **ppcVar2;
  let puVar3: *mut u8;
  let uVar4: u16;
  let extraout_DX: u16;
  let puStack24: u32;
  let local_14: [u8;12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_2,local_14),0x1,0x0,0x700);
  while( true ) {
    uVar4 = param_1;
    puVar3 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2,puVar3));
    puStack24 = CONCAT22(uVar4,puVar3);
    param_1 = uVar4 | puVar3;
    if (param_1 == 0x0) break;
    if ((puVar3 + 0x12) == 0x5) {
      iVar1 = (puVar3 + 0xc);
      if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
         ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 ||
          ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
        ppcVar2 = (code **)(*puStack24 + 0x2c);
        (**ppcVar2)(&USHORT_1050_1028);
        param_1 = extraout_DX;
      }
    }
  }
  return 0x1;
}



fn pass1_1030_ebf8(param_1: u32,param_2: u16,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  let puVar3: u32;
  let iVar4: i16;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x108,param_3,0x1000);
  puStack10 = (u16 *)CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    (param_2 + 0x2) = 0x1008;
    uVar6 = (param_1 >> 0x10);
    (param_2 + 0x4) = (param_1 + 0x4);
    puVar3 = (param_1 + 0x8);
    puVar5 = (param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_2 + 0x2) = &USHORT_1050_1028;
    *puStack10 = 0xecb2;
    (param_2 + 0x2) = 0x1030;
  }
  return;
}



astruct_18 *  pass1_1030_ec86(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_100 * 
pass1_1030_ecc2(astruct_100 *param_1,param_2: u16,param_3: u8)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0xf9f);
  param_1->field_0x0 = 0xb96;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (param_1 + 0x8)),
             s_SCMorale_1050_59ce);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1030_ecf8(param_1: u32,param_2: u32,param_3: i16,param_4: u16,param_5: u8)
{
  let iVar1: i16;
  let puVar2: u32;
  code **ppcVar3;
  let uVar4: u16;
  let uVar5: u32;
  let puVar6: *mut u8;
  let iVar7: i16;
  let uVar8: u32;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u32;
  let uVar14: u16;
  let bVar15: bool;
  let puVar16: *mut u16;
  let puVar17: u32;
  let uVar18: u16;
  let uStack64: u32;
  let iStack56: i16;
  let uStack54: u16;
  let uStack38: u32;
  let local_22: [u8;12];
  let uStack16: u16;
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u16;
  let uStack4: u16;
  
  uStack12 = 0x0;
  puVar16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,(uchar *)param_2,param_3);
  uVar13 = param_2 & 0xffff0000 | puVar16 >> 0x10;
  uStack10 = puVar16;
  uStack4 = (puVar16 >> 0x10);
  uStack6 = uStack10;
  pass1_1010_ed3e(puVar16);
  uStack8 = uVar13;
  uVar13 = uVar13 & 0xffff0000 | (uStack8 | uStack10);
  if ((uStack8 | uStack10) != 0x0) {
    uStack12 = pass1_1030_2aaa(CONCAT22(uStack8,uStack10));
  }
  if (uStack12 < 0x2) {
    uStack12 = 0x0;
  }
  puVar16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,(uchar *)uVar13,param_3);
  uVar13 = uVar13 & 0xffff0000 | puVar16 >> 0x10;
  uStack16 = SUB42(puVar16,0x0);
  uStack14 = (puVar16 >> 0x10);
  if ((0x0 < PTR_LOOP_1050_13ae) && (!SBORROW2(PTR_LOOP_1050_13ae,0x1))) {
    if (PTR_LOOP_1050_13ae == &PTR_LOOP_1050_0002 ||
        (PTR_LOOP_1050_13ae + -0x1) < 0x1) {
      if (0x6 < uStack12) {
        uStack12 -= 0x2;
        goto LAB_1030_ed5b;
      }
      bVar15 = SBORROW2(uStack12,0x4);
      iVar1 = uStack12 - 0x4;
    }
    else {
      if (PTR_LOOP_1050_13ae != (&PTR_LOOP_1050_0002 + 0x1))
      goto LAB_1030_ed5b;
      bVar15 = SBORROW2(uStack12,0x7);
      iVar1 = uStack12 - 0x7;
    }
    if (bVar15 == iVar1 < 0x0) {
      uStack12 -= 0x1;
    }
  }
LAB_1030_ed5b:
  pass1_1028_dc52((astruct_92 *)
                  CONCAT13((char)(param_4 >> 0x8),CONCAT12((char)param_4,local_22)),0x1,
                  0x0,0x400);
  while( true ) {
    puVar6 = local_22;
    pass1_1028_e4ec(CONCAT22(param_4,puVar6));
    uVar9 = uVar13;
    uStack38 = CONCAT22(uVar9,puVar6);
    if ((uVar9 | puVar6) == 0x0) break;
    uVar10 = (puVar6 + 0x1f6);
    uVar13 = uVar13 & 0xffff0000 | (puVar6 + 0x1f8);
    if (((puVar6 + 0x1fe) != 0x0) && (*(long *)(puVar6 + 0x200) != 0x8000002)) {
      pass1_1030_38b8();
      uVar10 = uVar13 | uVar10;
      uVar8 = uVar13 & 0xffff0000;
      uVar13 = uVar8 | uVar10;
      if (uVar10 != 0x0) {
        puVar2 = *(u32 **)(puVar6 + 0xc);
        uVar10 = (puVar6 + 0xe);
        uVar8 |= uVar10;
        ppcVar3 = (code **)(*puVar2 + 0x10);
        puVar17 = puVar2;
        (**ppcVar3)(&USHORT_1050_1028,puVar2,uVar10);
        uVar5 = puVar17 & 0xffff | uVar8 << 0x10;
        uStack54 = (puVar6 + 0x18);
        uVar14 = SUB42(&PTR_LOOP_1050_1038,0x0);
        pass1_1038_4760(CONCAT22(uVar9,puVar6));
        iVar1 = (puVar6 + 0x22);
        iStack56 = iVar1 / 0xa;
        uVar13 = uVar8 & 0xffff0000 | (long)iVar1 % 0xa & 0xffffU;
        iVar1 = (puVar6 + 0x24);
        if (iVar1 < 0x33) {
          if (iVar1 < 0x32) {
            iStack56 += -0x1;
          }
        }
        else {
          uStack54 += 0x1;
        }
        for (uStack64 = 0x0; uStack64 < uVar5; uStack64 += 0x1) {
          ppcVar3 = (code **)(*puVar2 + 0x4);
          uVar8 = uVar5;
          (**ppcVar3)(uVar14,(char)puVar2,(puVar2 >> 0x10),uStack64,
                      (uStack64 >> 0x10));
          uVar10 = uVar8;
          uVar11 = uVar13;
          uVar12 = uVar11 | uVar10;
          uVar13 = uVar13 & 0xffff0000 | uVar12;
          if (uVar12 != 0x0) {
            uVar14 = SUB42(&USHORT_1050_1028,0x0);
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar10,uVar11);
            puVar17 = struct_op_1030_73a8(CONCAT22(uVar13,uVar10));
            uVar10 = puVar17;
            uVar11 = (puVar17 >> 0x10);
            uVar13 = uVar13 & 0xffff0000 | (uVar11 | uVar10);
            if (((uVar11 | uVar10) != 0x0) && ((uVar10 + 0x12) == 0x5)) {
              ppcVar3 = (code **)(*puVar17 + 0x48);
              (**ppcVar3)(&USHORT_1050_1028,uVar10,uVar11);
              if (uVar10 < 0x0) {
                iStack56 += uVar10;
              }
              else {
                uStack54 += uVar10;
              }
            }
          }
        }
        iStack56 -= uStack12;
        iVar1 = (puVar6 + 0x20a);
        uVar18 = (param_1 >> 0x10);
        uVar4 = param_1;
        iVar7 = iVar1;
        pass1_1038_01c0(uVar4,uVar18,uStack38,param_4);
        iVar7 -= iVar1;
        iStack56 -= iVar7;
        pass1_1038_008e(uVar4,uVar18,uStack38,(uchar *)uVar13,param_3,param_4);
        if (iVar7 < 0x0) {
          iStack56 += iVar7;
        }
        else {
          uStack54 += iVar7;
        }
        if (0x3e8 < uStack54) {
          uStack54 = 0x3e8;
        }
        if (uStack54 < 0x0) {
          uStack54 = 0x0;
        }
        uStack54 += iStack56;
        if (0x3e8 < uStack54) {
          uStack54 = 0x3e8;
        }
        if (uStack54 < 0x0) {
          uStack54 = 0x0;
        }
        pass1_1038_4d0e(uStack38,uStack54);
        if (*(long *)(puVar6 + 0x4) == 0x4000001) {
          pass1_1038_08d4(uVar4,CONCAT22(uStack54,uVar18),uStack38,uVar13,param_4,param_5)
          ;
        }
        pass1_1038_095e(uVar4,uVar18,uStack54,uStack38,(uchar *)uVar13,param_3,param_4);
      }
    }
  }
  return;
}



