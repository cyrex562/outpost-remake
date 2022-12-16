
pub unsafe fn pass1_1038_3074(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_2a5c(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub unsafe fn pass1_1038_30aa(param_1: *mut astruct_180,mut param_2: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut paVar3: *mut Struct57;
  let mut iVar5: *mut astruct_180;
  let mut uVar5: *mut astruct_180;
  let mut puVar5: *mut u16;
  let mut paVar4: *mut Struct57;

  puVar5 = struct_1030_17ce(param_1,0x0,0x0,param_2);
  paVar3 = (param_2 & 0xffff0000 | puVar5 >> 0x10);
  uVar5 = (param_1 >> 0x10);
  iVar5 = param_1;
  iVar5.field12_0x10 = 0;
  iVar5.field14_0x14 = 0;
  iVar5.field16_0x18 = 0x258;
  iVar5.field17_0x1a = 0x258;
  iVar5.field18_0x1c = 0;
  (&iVar5.field18_0x1c + 0x2) = 0;
  iVar5[0x1].field1_0x2 = 0;
  iVar5[0x1].field_0x4 = 0x32;
  iVar5[0xf].field15_0x16 = 0;
  iVar5[0xf].field17_0x1a = 0;
  (&iVar5[0xf].field18_0x1c + 0x2) = 0;
  (iVar5 + 0x10) = 0x8000001;
  iVar5[0x10].field_0x4 = 0;
  iVar5[0x10].field_0x6 = 0;
  iVar5[0x10].field_0x8 = 0x1;
  iVar5[0x10].field_0xa = 0;
  iVar5[0x10].field10_0xc = 0;
  iVar5[0x10].field11_0xe = 0;
  iVar5[0x10].field12_0x10 = 0;
  iVar5[0x10].field14_0x14 = 0;
  iVar5[0x10].field15_0x16 = 0;
  iVar5[0x10].field17_0x1a = 0;
  param_1.field0_0x0 = 0x6504;
  iVar5.field1_0x2 = &u16_1050_1038;
  pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(&iVar5[0x1].field_0x6)),NULL,0x94);
  pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(&iVar5[0x5].field17_0x1a)),NULL,0x94);
  pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(&iVar5[0xa].field11_0xe)),NULL,0x54);
  puVar1 = pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(&iVar5[0xd].field1_0x2)),NULL,0x54);
  mem_op_1000_179c(0x1b0,paVar3);
  uVar2 = paVar3 | puVar1;
  paVar4 = (paVar3 & 0xffff0000 | uVar2);
  if (uVar2 == 0) {
    iVar5[0xf].field15_0x16 = 0;
  }
  else {
    pass1_1030_314c(uVar2,CONCAT22(paVar3,puVar1),&iVar5.field_0x4);
    iVar5[0xf].field15_0x16 = puVar1;
    iVar5[0xf].field16_0x18 = paVar4;
  }
  mem_op_1000_179c(0x1e,paVar4);
  uVar2 = paVar4 | puVar1;
  if (uVar2 == 0) {
    puVar1 = null_mut();
    uVar2 = 0;
  }
  else {
    struct_1020_c444(CONCAT22(paVar4,puVar1),0x64,0xc8);
  }
  iVar5.field10_0xc = puVar1;
  iVar5.field11_0xe = uVar2;
  return;
}
pub unsafe fn pass1_1038_3222(mut param_1: u16 ,param_2: *mut Struct57,param_3: *mut astruct_363,mut param_4: u32,mut param_5: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar5: *mut astruct_363;
  let mut uVar6: u16;
  let mut puVar7: *mut u16;
  let mut local_16: [u8;0x14] = [0;0x14];
  let mut uVar5: u32;

  puVar7 = pass1_1030_183c(param_1,param_2,param_3,0x0,0x0,0x4000000,param_5);
  paVar4 = (param_2 & 0xffff0000 | puVar7 >> 0x10);
  uVar6 = (param_3 >> 0x10);
  iVar5 = param_3;
  iVar5.field12_0x10 = param_4;
  iVar5.field13_0x14 = 0;
  iVar5.field14_0x18 = 0x258;
  iVar5.field15_0x1a = 0x258;
  iVar5.field16_0x1c = 0;
  iVar5.field17_0x1e = 0;
  iVar5.field18_0x22 = 0;
  iVar5.field19_0x24 = 0x32;
  iVar5.field484_0x1f6 = 0;
  iVar5.field486_0x1fa = 0;
  iVar5.field488_0x1fe = 0;
  iVar5.field489_0x200 = 0x8000001;
  iVar5.field490_0x204 = 0;
  iVar5.field491_0x206 = 0;
  iVar5.field492_0x208 = 0x1;
  iVar5.field493_0x20a = 0;
  iVar5.field494_0x20c = 0;
  iVar5.field495_0x20e = 0;
  iVar5.field496_0x210 = 0;
  iVar5.field497_0x214 = 0;
  iVar5.field498_0x216 = 0;
  iVar5.field499_0x21a = 0;
  param_3 = 0x6504;
  iVar5.field2_0x2 = &u16_1050_1038;
  pass1_1000_4906((param_3 & 0xffff0000 | ZEXT24(&iVar5.field_0x26)),NULL,0x94);
  pass1_1000_4906((param_3 & 0xffff0000 | ZEXT24(&iVar5.field_0xba)),NULL,0x94);
  pass1_1000_4906((param_3 & 0xffff0000 | ZEXT24(&iVar5.field_0x14e)),NULL,0x54);
  puVar1 = pass1_1000_4906((param_3 & 0xffff0000 | ZEXT24(&iVar5.field_0x1a2)),NULL,0x54);
  mem_op_1000_179c(0x1b0,paVar4);
  uVar2 = paVar4 | puVar1;
  uVar5 = paVar4 & 0xffff0000 | uVar2;
  if (uVar2 == 0) {
    iVar5.field484_0x1f6 = 0;
  }
  else {
    pass1_1030_314c(uVar2,CONCAT22(paVar4,puVar1),&iVar5.field_0x4);
    iVar5.field484_0x1f6 = puVar1;
    iVar5.field485_0x1f8 = uVar5;
  }
  paVar4 = (uVar5 & 0xffff0000 | iVar5.field5_0x6 & 0xffff00ff);
  sys_1000_3f9c(CONCAT22(0x1050,local_16),0x10505a1a,&iVar5.field_0x4);
  uVar2 = str_op_1008_60e8(paVar4,CONCAT22(0x1050,local_16));
  iVar5.field486_0x1fa = uVar2;
  iVar5.field487_0x1fc = paVar4;
  mem_op_1000_179c(0x1e,paVar4);
  uVar3 = paVar4 | uVar2;
  if (uVar3 == 0) {
    iVar5.field10_0xc = 0;
  }
  else {
    struct_1020_c444(CONCAT22(paVar4,uVar2),0x64,0xc8);
    iVar5.field10_0xc = uVar2;
    iVar5.field11_0xe = uVar3;
  }
  return;
}
pub unsafe fn pass1_1038_33f8(param_1: *mut u16)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x6504;
  (iVar4 + 0x2) = &u16_1050_1038;
  puVar1 = (iVar4 + 0x14);
  uVar2 = (iVar4 + 0x16);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  puVar1 = (iVar4 + 0x1f6);
  uVar2 = (iVar4 + 0x1f8);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*(iVar4 + 0x1fa));
  puVar1 = (iVar4 + 0x210);
  uVar2 = (iVar4 + 0x212);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)(0x1000,puVar1,uVar2,1);
  }
  fn_ptr_1000_17ce(*(iVar4 + 0x21a));
  pass1_1030_18b2(param_1);
  return;
}
pub unsafe fn pass1_1038_349e(param_1: *mut astruct_685,mut param_2: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut extraout_DX_00: u16;
  let mut iVar7: *mut astruct_685;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar6 = (param_1 >> 0x10);
  iVar7 = param_1;
  iVar7.field509_0x200 = param_2;
  pass1_1038_4d0e(param_1,0x258);
  uVar3 = param_2;
  pass1_1038_4d0e(param_1,0x258);
  iVar7.field510_0x204 = 0;
  iVar7.field511_0x206 = 0;
  puVar7 = iVar7.field12_0xc;
  uVar8 = SUB42(puVar7,0x0);
  uVar9 = (puVar7 >> 0x10);
  ppcVar1 = (*iVar7.field12_0xc + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,uVar3);
  uVar5 = extraout_DX;
  for uStack10 in 0 .. uStack6 {
    puVar7 = pass1_1030_1d7c(uVar3,uVar5,iVar7.field12_0xc);
    uVar4 = (puVar7 >> 0x10);
    uVar2 = puVar7;
    uVar5 = uVar4 | uVar2;
    if (uVar5 != 0) {
      ppcVar1 = (*puVar7 + 0x58);
      (**ppcVar1)(0x1030,uVar2,uVar4,param_1,uVar6,uVar8,uVar9);
      (uVar2 + 0x1c) = 0;
      uVar5 = extraout_DX_00;
    }
  }
  return;
}
pub unsafe fn pass1_1038_354a(param_1: *mut astruct_424,mut param_2: u16 ,param_3: *mut u8)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut iVar1: *mut astruct_424;
  let mut uVar3: *mut astruct_424;

  paVar2 = CONCAT22(in_register_0000000a,param_3);
  uVar3 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (&iVar1.field_0x21a == 0) {
    mem_op_1000_179c(0xa,paVar2);
    uVar1 = paVar2 | param_2;
    if (uVar1 == 0) {
      iVar1.field_0x21a = 0;
    }
    else {
      pass1_1030_9ecc(CONCAT22(paVar2,param_2),param_1);
      iVar1.field_0x21a = param_2;
      iVar1.field540_0x21c = uVar1;
    }
  }
  pass1_1030_9ef2(&iVar1.field_0x21a);
  return;
}
pub unsafe fn pass1_1038_35a8(mut param_1: u16 ,param_2: *mut u8,mut param_3: u16 ,param_4: *mut astruct_425)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut iVar3: *mut astruct_425;
  let mut in_AF: u8;
  let mut in_stack_00000006: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_2);
  if (&param_4.field_0x21a == 0) {
    mem_op_1000_179c(0xa,paVar2);
    uVar1 = paVar2 | param_1;
    if (uVar1 == 0) {
      param_4.field_0x21a = 0;
    }
    else {
      pass1_1030_9ecc(CONCAT22(paVar2,param_1),CONCAT22(in_stack_00000006,param_4));
      param_4.field_0x21a = param_1;
      param_4.field540_0x21c = uVar1;
    }
  }
  pass1_1030_9f40(in_AF,&param_4.field_0x21a,param_3);
  return;
}
pub unsafe fn pass1_1038_3608(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  fn_ptr_1000_17ce(*(param_1 + 0x21a));
  (param_1 + 0x21a) = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_362e(mut param_1: u32,mut param_2: u16 ,param_3: *mut *mut u8,param_4: *mut Struct57)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut paVar3: *mut astruct_67;
  let mut in_stack_0000fea2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd0: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0x214) == 0) {
    pass1_1038_4f54(param_2,param_1 & 0xffff | uVar2 << 0x10,0x1f);
    if (param_2 == 0) {
      (iVar1 + 0x214) = 0x14;
    }
    else {
      (iVar1 + 0x214) = 0x28;
    }
    param_3 = (param_3 & 0xffff0000 | 0x37);
    paVar3 =
             mixed_1010_20ba(param_4,_u16_1050_0ed0,param_3,in_stack_0000fea2,in_stack_0000ffc6,in_stack_0000ffcc,
                             in_stack_0000ffd0);
    post_win_msg_1008_a0e4(paVar3,0x0,0x0,0x1,(iVar1 + 0x4),0x38);
    (iVar1 + 0x216) = 0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_3698(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  let mut puVar2: *mut u16;
  let mut uVar3: u32;
  let mut ppcVar4: *mut *mut code;
  let mut uVar5: u16;
  let mut BVar6: bool;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut lVar9: i32;
  let mut uVar10: u32;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u32;
  let mut iVar15: i16;
  let mut uVar16: u16;
  let mut uVar17: u32;
  let mut uStack32: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar16 = (param_3 >> 0x10);
  iVar15 = param_3;
  if ((iVar15 + 0x214) == 0) {
    return;
  }
  pass1_1030_38b8();
  uStack6 = CONCAT22(param_2,param_1);
  uStack6 -= (iVar15 + 0x216);
  if (0x0 < uStack6) {
    uStack6 += 0x3;
    uStack10 = uStack6 / 0x5;
    uVar14 = uStack6 % 0x5;
    if ((iVar15 + 0xc) == 0) {
      uVar5 = 0;
      uVar14 = 0;
    }
    else {
      uVar3 = (iVar15 + 0xc);
      ppcVar4 = ((iVar15 + 0xc) + 0x10);
      lVar9 = uStack10;
      (**ppcVar4)(0x1030,uVar3,(uVar3 >> 0x10));
      uVar5 = lVar9;
    }
    uStack14 = CONCAT22(uVar14,uVar5);
    // for (uStack18 = 0; uVar12 = uVar14, uVar10 = uStack14, uStack18 < uStack14; uStack18 += 1)
    uStack18 = 0;
    uVar12 = uVar14;
    uVar10 = uStack14;
    while uStack18 < uStack14
    {
      uVar17 = pass1_1030_1d7c(uVar5,uVar12,(iVar15 + 0xc));
      uVar8 = (uVar17 >> 0x10);
      uVar13 = uVar8 | uVar17;
      uVar14 = uVar13;
      if (uVar13 != 0) {
        BVar6 = pass1_1008_c6ae(_u16_1050_06e0,(uVar17 + 0xc),0x4);
        uVar8 = uVar14;
        uVar10 = BVar6;
        if (BVar6 != 0) {
          uVar7 = pass1_1028_678c(uVar17,0xf);
          uStack32 = CONCAT22(uVar8,uVar7);
          uVar14 = (uVar8 | uVar7);
          uVar10 = uVar7;
          if ((uVar8 | uVar7) != 0) {
            if (uStack10 < uStack32) {
              uVar8 = uStack10;
              pass1_1028_6356(uVar17,0xf,uVar8,uStack10);
              uVar13 = uVar8 * 0x5;
              uVar11 = uStack10 * 0x5 + CARRY2(uVar8,uVar8) * 0x2 + CARRY2(uVar8 * 0x2,uVar8 * 0x2) +
                       CARRY2(uVar8 * 0x4,uVar8);
              uVar14 = uVar11;
              puVar2 = (iVar15 + 0x216);
              uVar8 = *puVar2;
              *puVar2 = *puVar2 + uVar13;
              piVar1 = (iVar15 + 0x218);
              *piVar1 = *piVar1 + uVar11 + CARRY2(uVar8,uVar13);
              uStack10 = 0;
              uVar10 = uVar13;
            }
            else {
              pass1_1028_6356(uVar17,0xf,uVar7,uVar8);
              uVar13 = uVar8 * 0x5 + CARRY2(uVar7,uVar7) * 0x2 + CARRY2(uVar7 * 0x2,uVar7 * 0x2) +
                       CARRY2(uVar7 * 0x4,uVar7);
              uVar14 = uVar13;
              puVar2 = (iVar15 + 0x216);
              uVar8 = *puVar2;
              *puVar2 = *puVar2 + uVar7 * 0x5;
              piVar1 = (iVar15 + 0x218);
              *piVar1 = *piVar1 + uVar13 + CARRY2(uVar8,uVar7 * 0x5);
              uStack10 -= uStack32;
              uVar10 = uStack32;
            }
          }
        }
        uVar12 = uVar14;
        if (uStack10 == 0) { break; }
      }
      uStack18 += 1;
    }
    uVar5 = uVar10;
    pass1_1030_38b8();
    uStack6 = CONCAT22(uVar12,uVar5);
    uStack6 -= (iVar15 + 0x216);
    uStack6 = (uStack6 >> 0x10);
    if ((uStack6 | uStack6) != 0) {
      uStack32 = uStack6 / (iVar15 + 0x214);
      if (uStack32 < 1) {
        uStack32 = 0x1;
      }
      pass1_1030_375a((iVar15 + 0x1f6),0x0,uStack32);
    }
  }
  piVar1 = (iVar15 + 0x214);
  *piVar1 = *piVar1 -0x1;
  return;
}
pub unsafe fn pass1_1038_387e(param_1: *mut Struct57,param_2: *mut astruct_302,mut param_3: i16,mut param_4: i16,mut param_5: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut lVar2: i32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u32;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut iVar10: *mut astruct_302;
  let mut uVar10: u16;
  let mut iStack22: i16;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;
  let mut paVar9: *mut Struct57;

  if (param_3 != param_4) {
    iVar10 = param_2;
    uVar10 = (param_2 >> 0x10);
    if (param_3 < param_4) {
      uStack12 = param_4 - param_3;
      if ((iVar10.field525_0x210 == 0) || (lVar2 = iVar10.field525_0x210, (lVar2 + 0xa) == 0)) {
        if (iVar10.field12_0xc.is_null()) {
          uVar7 = 0;
          param_1 = (param_1 & 0xffff0000);
        }
        else {
          ppcVar1 = (*iVar10.field12_0xc + 0x10);
          uVar7 = uStack12;
          (**ppcVar1)();
        }
        uStack6 = CONCAT22(param_1,uVar7);
        for uStack10 in 0 .. uStack6 {
          uVar5 = uStack6;
          pass1_1030_1d58(iVar10.field12_0xc);
          uVar7 = param_1 | uVar5;
          paVar9 = (param_1 & 0xffff0000 | uVar7);
          if ((uVar7 != 0) && (uVar3 = pass1_1030_6fa0(uVar5 & 0xffff | param_1 << 0x10), uVar3 == 0xb)) {
            pass1_1030_7c50(uStack12,paVar9,
                            CONCAT13((param_1 >> 0x8),CONCAT12(param_1,uVar5)),
                            uStack12,0x4);
            return;
          }
          param_1 = paVar9;
        }
      }
      else {
        lVar2 = iVar10.field525_0x210;
        uVar5 = (lVar2 + 0xa);
        // for (uStack10 = 0; uVar7 = param_1, uStack10 < uVar5; uStack10 += 1)
        uStack10 = 0;
        uVar7 = param_1;
        while uStack10 < uVar5
        {
          uVar6 = uVar5;
          bad_1030_1312();
          uVar8 = uVar6;
          param_1 = (uVar7 | uVar8);
          if ((((uVar7 | uVar8) != 0) && (pass1_1030_cc44(uVar8,uVar7,uStack12,param_5,0x4), uVar8 != 0)) &&
             (uStack12 -= uVar8, uStack12 == 0)) {
            return;
          }
          uStack10 += 1;
        }
      }
    }
    else {
      iStack22 = param_3 - param_4;
      if ((iVar10.field525_0x210 == 0) || (lVar2 = iVar10.field525_0x210, (lVar2 + 0xa) == 0)) {
        if (iVar10.field12_0xc.is_null()) {
          iVar4 = 0;
          param_1 = null_mut();
        }
        else {
          ppcVar1 = (*iVar10.field12_0xc + 0x10);
          iVar4 = iStack22;
          (**ppcVar1)();
        }
        uStack6 = CONCAT22(param_1,iVar4);
        // for (uStack10 = 0; uVar7 = param_1, uStack10 < uStack6; uStack10 += 1)
        uStack10 = 0;
        uVar7 = param_1;
        while uStack10 < uStack6
        {
          uVar5 = uStack6;
          pass1_1030_1d58(iVar10.field12_0xc);
          uVar8 = uVar7 | uVar5;
          param_1 = uVar8;
          if ((uVar8 != 0) && (uVar3 = pass1_1030_6fa0(uVar5 & 0xffff | uVar7 << 0x10), uVar3 == 0xb)) {
            pass1_1030_6e9c(CONCAT13((uVar7 >> 0x8),CONCAT12(uVar7,uVar5)),
                            iStack22,0x4);
            return;
          }
          uStack10 += 1;
        }
      }
      else {
        lVar2 = iVar10.field525_0x210;
        uVar5 = (lVar2 + 0xa);
        // for (uStack10 = 0; uVar7 = param_1, uStack10 < uVar5; uStack10 += 1)
        uStack10 = 0;
        uVar7 = param_1;
        while uStack10 < uVar5
        {
          uVar6 = uVar5;
          bad_1030_1312();
          uVar8 = uVar6;
          param_1 = (uVar7 | uVar8);
          if ((uVar7 | uVar8) != 0) {
            pass1_1030_ce72(uVar7 << 0x10 | uVar6 & 0xffff,iStack22,param_5,0x4);
            iStack22 -= uVar8;
            if (iStack22 == 0) {
              return;
            }
          }
          uStack10 += 1;
        }
      }
    }
  }
  return;
}
pub unsafe fn pass1_1038_3aa6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut extraout_DX: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uStack12: u32;
  let mut uStack8: u32;

  uVar9 = (param_3 >> 0x10);
  iVar8 = param_3;
  if (((iVar8 + 0x210) == 0) || (uVar2 = (iVar8 + 0x210), (uVar2 + 0xa) == 0))
  {
    if ((iVar8 + 0xc) == 0) {
      param_1 = 0;
      uVar6 = 0;
    }
    else {
      ppcVar1 = ((iVar8 + 0xc) + 0x10);
      (**ppcVar1)();
      uVar6 = extraout_DX;
    }
    uStack8 = CONCAT22(uVar6,param_1);
    for uStack12 in 0 .. uStack8 {
      uVar4 = uStack8;
      pass1_1030_1d58((iVar8 + 0xc));
      uVar7 = uVar6 | uVar4;
      if ((uVar7 != 0) && (uVar3 = pass1_1030_6fa0(uVar4 & 0xffff | uVar6 << 0x10), uVar3 == 0xb)) {
        pass1_1030_6b86(0xb,uVar4 & 0xffff | uVar6 << 0x10);
        return;
      }
      uVar6 = uVar7;
    }
  }
  else {
    uVar2 = (iVar8 + 0x210);
    uVar4 = (uVar2 + 0xa);
    for uStack12 in 0 .. uVar4 {
      uVar5 = uVar4;
      bad_1030_1312();
      uVar6 = param_2 | uVar5;
      if (uVar6 != 0) {
        pass1_1030_ce2e(uVar5,param_2,0x4);
      }
      param_2 = uVar6;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_3ba0(param_1: *mut astruct_428)

{
  let mut puVar1: *mut u32;
  let mut ppcVar2: *mut *mut code;
  let mut puVar3: *mut u32;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut puVar7: *mut u8;
  let mut puVar8: *mut u8;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut in_EDX: u32;
  let mut paVar11: *mut Struct57;
  let mut iVar13: *mut astruct_428;
  let mut uVar13: *mut astruct_428;
  let mut uVar14: u16;
  let mut puVar15: *mut u32;
  let mut uVar16: u32;
  let mut uStack20: u32;
  let mut uVar12: u32;

  uVar13 = (param_1 >> 0x10);
  iVar13 = param_1;
  puVar1 = &iVar13.field528_0x210;
  uVar5 = (&iVar13.field528_0x210 + 2);
  uVar14 = (in_EDX >> 0x10);
  if ((uVar5 | puVar1) != 0) {
    ppcVar2 = *puVar1;
    (**ppcVar2)();
  }
  puVar15 = pass1_1008_c6fa(_u16_1050_06e0,0x1e);
  puVar7 = (puVar15 >> 0x10);
  uVar6 = puVar15 & 0xffff;
  pass1_1038_4d6e(uVar6,puVar7,param_1,puVar15);
  uVar4 = uVar6 & 0xffff;
  puVar3 = (uVar4 | ZEXT24(puVar7) << 0x10);
  ppcVar2 = (*puVar3 + 0x10);
  puVar8 = puVar7;
  (**ppcVar2)(0x1008,uVar6,puVar7);
  uVar5 = uVar6;
  paVar11 = CONCAT22(uVar14,puVar8);
  if ((puVar8.is_null()) && (uVar5 < 0x5)) {
    uVar5 = 0x5;
  }
  uVar5 += 0x1;
  uVar14 = 0x1000;
  uVar10 = uVar5;
  mem_op_1000_179c(0x1c,paVar11);
  uVar9 = paVar11 | uVar10;
  uVar12 = uVar9;
  if (uVar9 == 0) {
    iVar13.field528_0x210 = 0;
  }
  else {
    uVar12 = (uVar5 >> 0xf);
    uVar14 = 0x1030;
    struct_1030_11aa(CONCAT22(paVar11,uVar10),0x5,uVar5);
    iVar13.field528_0x210 = uVar5;
    (&iVar13.field528_0x210 + 0x2) = uVar12;
  }
  uVar16 = iVar13.field528_0x210;
  (uVar16 + 0x1a) = 0;
//   for (uStack20 = 0; uStack20 < (uVar6 & 0xffff | ZEXT24(puVar8) << 0x10); uStack20 += 1)
for uStack20 in 0 ..  (uVar6 & 0xffff | ZEXT24(puVar8) << 0x10)
{
    uVar16 = pass1_1030_1d7c((uVar6 & 0xffff),uVar12,puVar3);
    uVar5 = (uVar16 >> 0x10);
    uVar10 = uVar5 | uVar16;
    uVar12 = uVar10;
    if (uVar10 != 0) {
      pass1_1030_1358(iVar13.field528_0x210,uVar16,uVar5,uStack20 + 1);
    }
    uVar14 = 0x1030;
  }
  if (puVar3.is_null() == false) {
    ppcVar2 = *puVar3;
    (**ppcVar2)(uVar14,uVar4,puVar7,1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_3cc0(mut param_1: u32,mut param_2: u16 ,param_3: *mut u8,mut param_4: u16 )

{
  let mut lVar1: i32;
  let mut ppcVar2: *mut *mut code;
  let mut puVar3: *mut u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut in_EDX: u32;
  let mut paVar10: *mut Struct57;
  let mut puVar11: *mut u32;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut puVar14: *mut u32;
  let mut paVar15: *mut astruct_15;
  let mut uVar16: u32;
  let mut uVar17: u8;
  let mut uVar18: u8;
  let mut uVar19: u8;
  let mut uVar20: u8;
  let mut puStack26: *mut u32;
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut puStack10: *mut u32;

  uVar12 = (in_EDX >> 0x10);
  if (param_4 == 0x1e) {
    uVar13 = 0x1008;
    puVar14 = pass1_1008_c6fa(_u16_1050_06e0,0x27);
    paVar10 = CONCAT22(uVar12,(puVar14 >> 0x10));
    puVar11 = puVar14;
    pass1_1038_4e78(puVar11,paVar10,param_1,puVar14);
    uVar12 = SUB42(paVar10,0x0);
    puStack10 = CONCAT22(uVar12,puVar11);
    ppcVar2 = (*puStack10 + 0x10);
    puVar3 = puVar11;
    (**ppcVar2)(0x1008,puVar11,uVar12);
    uStack14 = CONCAT22(paVar10,puVar3);
    for uStack18 in 0 .. uStack14 {
      paVar15 = pass1_1030_1d7c(puVar3,paVar10,puStack10);
      uVar6 = (paVar15 >> 0x10) | paVar15;
      paVar10 = (paVar10 & 0xffff0000 | uVar6);
      if (uVar6 != 0) {
        uVar4 = pass1_1030_bfb8(paVar15);
        puStack26 = CONCAT22(paVar10,uVar4);
        uVar6 = paVar10 | uVar4;
        paVar10 = (paVar10 & 0xffff0000 | uVar6);
        if (uVar6 != 0) {
          pass1_1028_b58e(paVar15);
          uVar7 = SUB42(paVar10,0x0);
          paVar10 = (paVar10 & 0xffff0000 | ZEXT24(param_3));
          if (CONCAT22(param_3,param_2) <= puStack26) {
            uVar13 = 0x1030;
            pass1_1030_7ddc(param_2,paVar10,CONCAT22(uVar7,uVar4),
                            CONCAT13((param_3 >> 0x8),CONCAT12(param_3,param_2)),0x1e);
            break;
          }
          pass1_1030_7ddc(param_2,paVar10,CONCAT22(uVar7,uVar4),puStack26,0x1e);
          lVar1 = CONCAT22(param_3,param_2) - puStack26;
          param_2 = lVar1;
          param_3 = (lVar1 >> 0x10);
        }
      }
      uVar13 = 0x1030;
    }
    puStack26 = puStack10;
    if (puStack10.is_null()) {
      return;
    }
  }
  else {
    if (param_4 != 0x21) {
      uVar13 = 0x1008;
      puVar14 = pass1_1008_c6fa(_u16_1050_06e0,0x3);
      paVar10 = CONCAT22(uVar12,(puVar14 >> 0x10));
      uVar6 = puVar14;
      pass1_1038_4e78(uVar6,paVar10,param_1,puVar14);
      puStack26 = CONCAT22(paVar10,uVar6);
      ppcVar2 = (*puStack26 + 0x10);
      (**ppcVar2)(0x1008,uVar6,paVar10);
      uStack22 = CONCAT22(paVar10,uVar6);
      uStack18 = 0;//
// LAB_1038_3e9c:
      if (uStack18 < uStack22) {
        uVar13 = 0x1030;
        paVar15 = pass1_1030_1d7c(uVar6,paVar10,puStack26);
        uVar8 = (paVar15 >> 0x10) | paVar15;
        paVar10 = (paVar10 & 0xffff0000 | uVar8);
//        if (uVar8 == 0) goto LAB_1038_3e98;
        uVar13 = 0x1028;
        uVar16 = pass1_1028_45e2(paVar15,uVar8,paVar15);
        uVar8 = uVar16;
        uVar9 = (uVar16 >> 0x10) | uVar8;
        paVar10 = (paVar10 & 0xffff0000 | uVar9);
//        if (uVar9 == 0) goto LAB_1038_3e98;
        pass1_1028_b58e(paVar15);
        uVar12 = SUB42(paVar10,0x0);
        uVar5 = CONCAT22(param_3,param_2);
        paVar10 = (paVar10 & 0xffff0000 | ZEXT24(param_3));
        if (uVar16 < uVar5) {
          uVar13 = 0x1030;
          pass1_1030_7ddc(param_2,paVar10,CONCAT22(uVar12,uVar8),uVar16,param_4);
          lVar1 = CONCAT22(param_3,param_2) - uVar16;
          param_2 = lVar1;
          param_3 = (lVar1 >> 0x10);
      // TODO: goto LAB_1038_3e98;
        }
        uVar19 = SUB21(param_3,0x0);
        uVar20 = (param_3 >> 0x8);
        uVar17 = uVar8;
        uVar18 = (uVar8 >> 0x8);//
// LAB_1038_3e67:
        uVar13 = 0x1030;
        pass1_1030_7ddc(uVar5,paVar10,CONCAT22(uVar12,CONCAT11(uVar18,uVar17)),
                        CONCAT13(uVar20,CONCAT12(uVar19,param_2)),param_4);
      }
  // TODO: goto LAB_1038_3e6c;
    }
    uVar13 = 0x1008;
    puVar14 = pass1_1008_c6fa(_u16_1050_06e0,0xa);
    paVar10 = CONCAT22(uVar12,(puVar14 >> 0x10));
    uVar6 = puVar14;
    pass1_1038_4e78(uVar6,paVar10,param_1,puVar14);
    puStack26 = CONCAT22(paVar10,uVar6);
    ppcVar2 = (*puStack26 + 0x10);
    (**ppcVar2)(0x1008,uVar6,paVar10);
    uStack22 = CONCAT22(paVar10,uVar6);
    for uStack18 in 0 .. uStack22 {
      uVar13 = 0x1030;
      paVar15 = pass1_1030_1d7c(uVar6,paVar10,puStack26);
      uVar5 = paVar15 & 0xffff;
      uVar8 = (paVar15 >> 0x10) | uVar5;
      paVar10 = (paVar10 & 0xffff0000 | uVar8);
      if (uVar8 != 0) {
        uVar19 = SUB21(param_3,0x0);
        uVar20 = (param_3 >> 0x8);
        pass1_1028_b58e(paVar15);
        uVar12 = SUB42(paVar10,0x0);
        uVar17 = uVar5;
        uVar18 = (uVar5 >> 0x8);
    // TODO: goto LAB_1038_3e67;
      }
    }//
// LAB_1038_3e6c:
    if (puStack26.is_null()) {
      return;
    }
    uVar12 = (puStack26 >> 0x10);
    puVar11 = puStack26;
  }
  ppcVar2 = *puVar11;
  (**ppcVar2)(uVar13,puStack26,uVar12,1);
  return;//
// LAB_1038_3e98:
  uStack18 += 0x1;
//   goto LAB_1038_3e9c;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_3efc(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,mut param_5: i16,mut param_6: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut puStack6: *mut u32;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  puStack6 = CONCAT22(param_6,param_5);
  (param_5 + 0x1c) = (param_3 + 0x4);
  ppcVar1 = (*puStack6 + 0x58);
  (**ppcVar1)(0x1028,param_5,param_6,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_3f38(param_1: u32,param_2: *mut u32,mut param_3: u32,mut param_4: i16,mut param_5: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut puVar3: *mut u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut paStack10: *mut astruct_294;
  let mut puStack6: *mut u32;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3);
  puStack6 = CONCAT22(param_5,param_4);
  iVar2 = param_4;
  pass1_1028_b58e(CONCAT22(param_5,param_4));
  paStack10 = CONCAT22(extraout_DX,iVar2);
  uVar5 = (iVar2 + 0x4);
  ppcVar1 = (*param_1 + 0x18);
  (**ppcVar1)(0x1028,param_1,uVar5);
  uVar6 = 0;
  uVar4 = 0;
  ppcVar1 = (*param_2 + 0x8);
  puVar3 = param_2;
  (**ppcVar1)();
  pass1_1030_73ee(extraout_DX_00,paStack10,(param_2 + 0x4));
  ppcVar1 = (*puStack6 + 0x58);
  (**ppcVar1)(0x1030,param_4,param_5,param_2,puVar3,uVar4,uVar5,uVar6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_3fb0(mut param_1: u32)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(param_1 + 0x200));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1038_3fca(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut pSVar6: *mut StructD;
  let mut uVar7: u16;
  let mut unaff_SI: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u32;
  let mut puVar14: *mut u32;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000ff90: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9a: u16;
  let mut uVar15: u8;
  let mut uVar16: u8;
  let mut uVar17: u8;
  let mut uVar18: u8;
  let mut iStack38: i16;
  let mut local_24: i16;
  let mut local_22: [u8;0x2] = [0;0x2];
  let mut piStack32: *mut i16;
  let mut uStack30: u16;
  let mut puStack28: *mut u8;
  let mut uStack26: u16;
  let mut uStack24: u16;
  let mut uStack22: u32;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut pcStack14: *mut c_char;
  let mut pcStack10: *mut c_char;
  let mut uStack6: u32;
  let mut pSVar5: *mut StructD;

  uVar9 = (param_2 >> 0x10);
  uVar7 = param_2;
  if ((uVar7 + 0xc) == 0) {
    param_1 = 0;
    in_EDX &= 0xffff0000;
  }
  else {
    ppcVar2 = ((uVar7 + 0xc) + 0x10);
    (**ppcVar2)();
  }
  uStack6 = CONCAT22(in_EDX,param_1);
  uVar4 = in_EDX | param_1;
  pSVar5 = (in_EDX & 0xffff0000 | uVar4);
  if (uVar4 != 0) {
    if (_PTR_LOOP_1050_5f2c == 0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
    }
    else {
      pSVar5 = (in_EDX & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    uVar3 = fn_ptr_op_1000_1708(uStack6 << 0x2,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar5);
    pcStack10 = CONCAT22(pSVar5,uVar3);
    if (_PTR_LOOP_1050_5f2c == 0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
    }
    else {
      pSVar5 = (pSVar5 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    uVar12 = 0x1000;
    uVar3 = fn_ptr_op_1000_1708(uStack6 << 0x2,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar5);
    pcStack14 = CONCAT22(pSVar5,uVar3);
    for uStack22 in 0 .. uStack6 {
      uVar1 = (uVar7 + 0xc);
      ppcVar2 = ((uVar7 + 0xc) + 0x4);
      uVar13 = uStack6;
      (**ppcVar2)(uVar12,uVar1,(uVar1 >> 0x10),uStack22,(uStack22 >> 0x10));
      uVar4 = uVar13;
      uStack16 = pSVar5;
      pSVar6 = (pSVar5 & 0xffff0000 | (uStack16 | uVar4));
      uStack18 = uVar4;
      if ((uStack16 | uVar4) != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar13 & 0xffff | pSVar5 << 0x10);
        uStack22 *= 0x4;
        uVar10 = (pcStack10 >> 0x10);
        iVar8 = pcStack10;
        (uStack22 + iVar8) = uVar4;
        uVar11 = SUB42(pSVar6,0x0);
        (uStack22 + iVar8 + 0x2) = uVar11;
        uVar12 = 0x1030;
        uVar13 = struct_op_1030_73a8(CONCAT22(uVar11,(uStack22 + iVar8)),uVar4,uVar11
                                    );
        pSVar6 = (pSVar6 & 0xffff0000 | uVar13 >> 0x10);
        uVar11 = (pcStack14 >> 0x10);
        (pcStack14 + uStack22) = uVar13;
        (pcStack14 + uStack22 + 0x2) = (uVar13 >> 0x10);
      }
      pSVar5 = pSVar6;
    }
    for uStack22 in 0 .. uStack6 {
      uVar12 = (pcStack14 >> 0x10);
      iVar8 = pcStack14;
      if (((uStack22 * 0x4 + iVar8) != 0) &&
         (uVar1 = (uStack22 * 0x4 + iVar8), (uVar1 + 0x1a) = 0x0,
         uVar1 = (uStack22 * 0x4 + iVar8), (uVar1 + 0x12) == 0x5)) {
        pass1_1028_bdac((uStack22 * 0x4 + iVar8),0x6);
      }
    }
    (uVar7 + 0x204) = 0;
    puVar14 = mixed_1010_20ba(pSVar5,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2),in_stack_0000fe6c
                              ,in_stack_0000ff90,in_stack_0000ff96,in_stack_0000ff9a);
    uStack30 = (puVar14 >> 0x10);
    uStack26 = SUB42(puVar14,0x0);
    puStack28 = PTR_LOOP_1050_13ae;
    if (PTR_LOOP_1050_13ae == (&PTR_LOOP_1050_0000 + 1)) {
      (uVar7 + 0x204) = 0x1;
    }
    uStack24 = uStack30;
    pass1_1038_5a96(uVar7,uVar9,uStack6,pcStack14);
    pass1_1038_5cc6(param_2,uStack6,pcStack14,pcStack10,0x0,0x2);
    pass1_1038_5b3c(uVar7,uVar9,uStack6,pcStack14);
    pass1_1038_5cc6(param_2,uStack6,pcStack14,pcStack10,0x0,1);
    uVar11 = SUB42(&DAT_1050_1050,0x0);
    uVar17 = SUB21(local_22,0x0);
    uVar18 = (local_22 >> 0x8);
    piStack32 = &local_24;
    uVar12 = SUB42(&DAT_1050_1050,0x0);
    uVar15 = SUB21(piStack32,0x0);
    uVar16 = (piStack32 >> 0x8);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(uVar7 + 0x8));
    pass1_1030_5b1c(CONCAT22(uStack30,piStack32),CONCAT22(uVar12,CONCAT11(uVar16,uVar15)),
                    CONCAT22(uVar11,CONCAT11(uVar18,uVar17)));
    // for (iStack38 = 0x1; iStack38 <= local_24; iStack38 += 1)
    for iStack38 in 1 .. local_24
    {
      pass1_1038_58e6(uVar7,uVar9,uStack6,pcStack14,pcStack10,iStack38);
      pass1_1038_5cc6(param_2,uStack6,pcStack14,pcStack10,iStack38,0x3);
    }
    pass1_1038_5a16(uVar7,uVar9,uStack6,pcStack14);
    for uStack22 in 0 .. uStack6 {
      uVar12 = (pcStack14 >> 0x10);
      iVar8 = pcStack14;
      if (((uStack22 * 0x4 + iVar8) != 0) &&
         (uVar1 = (uStack22 * 0x4 + iVar8), (uVar1 + 0x12) != 0x5)) {
        uVar1 = (uStack22 * 0x4 + iVar8);
        ppcVar2 = ((uStack22 * 0x4 + iVar8) + 0x28);
        (**ppcVar2)(0x1030,uVar1,(uVar1 >> 0x10));
      }
    }
    fn_ptr_1000_17ce(pcStack10);
    fn_ptr_1000_17ce(pcStack14);
  }
  return;
}
