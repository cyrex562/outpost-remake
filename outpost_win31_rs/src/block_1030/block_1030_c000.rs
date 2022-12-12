
pub unsafe fn pass1_1030_c09c(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  (param_2 + 0x20) = 0;
  (param_2 + 0x24) = 0;
  param_2.field0_0x0 = 0xc68e;
  (param_2 + 0x2) = 0x1030;
  return;
}



pub unsafe fn pass1_1030_c0d2(mut param_1: u32) -> u16

{
  if (0x0 < (param_1 + 0x24)) {
    return 0x1;
  }
  return 0x0;
}



pub unsafe fn pass1_1030_c0ec(mut param_1: u32) -> u16

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if (((param_1 + 0xc) != 0xb) && ((param_1 + 0x24) < 1)) {
    return 0x0;
  }
  return 0x1;
}
pub unsafe fn pass1_1030_c10e(mut param_1: u32)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (0x0 < (iVar2 + 0x24)) {
    piVar1 = (iVar2 + 0x24);
    *piVar1 = *piVar1 + -0x1;
    return;
  }
  (iVar2 + 0xc) = 0x37;
  return;
}
pub unsafe fn pass1_1030_c12e(mut param_1: i16,mut param_2: u32,mut param_3: i16)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut extraout_DX: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut paStack6: *mut astruct_397;

  pass1_1028_b58e(param_2);
  paStack6 = CONCAT22(extraout_DX,param_1);
  uVar2 = (param_1 + 0x2e);
  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  iVar3 = uVar2;
  if ((iVar4 + 0x24) < 1) {
    pass1_1030_7d1c(iVar3,extraout_DX,paStack6,0x0,0x230000);
  }
  else {
    if (param_3 == 0) {
      uVar6 = 0;
    }
    else {
      uVar6 = 0x32;
    }
    pass1_1030_7d1c(iVar3,extraout_DX,paStack6,uVar6,0x230000);
    piVar1 = (iVar4 + 0x24);
    *piVar1 = *piVar1 + -0x1;
  }
  if ((0x0 < (iVar4 + 0x24)) && ((iVar4 + 0x24) < 0x19)) {
    (iVar3 + 0x1fe) = 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_c1b2(param_1: *mut u8,param_2: *mut astruct_695)

{
  let mut iVar1: i16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut iVar2: *mut astruct_695;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc4: u16;
  let mut iVar5: i16;
  let mut in_stack_0000ffee: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  pass1_1028_be9e(param_2);
  uVar3 = (param_2 >> 0x10);
  iVar2 = param_2;
  if (iVar2.field17_0x12 == 0x5) {
    if (iVar2.field12_0xc == 0xb) {
      pass1_1030_c652(paVar2,0xc1d7);
      iVar5 = 0x82;
      puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,0x820008,in_stack_0000fe94,in_stack_0000ffb8,
                               in_stack_0000ffbe,in_stack_0000ffc2);
      paVar2 = (paVar2 & 0xffff0000 | puVar4 >> 0x10);
      iVar1 = puVar4;
      pass1_1010_9f8c(puVar4,iVar5);
      iVar2.field34_0x24 = iVar1 * 0x3;
      mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(in_stack_0000ffee,0x2),in_stack_0000fe96,
                      in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
      if (PTR_LOOP_1050_13ae < 0x3) {
        iVar1 = iVar2.field34_0x24;
        if (iVar1 < 0x32) {
          iVar1 = 0x32;
        }
        iVar2.field34_0x24 = iVar1;
        return;
      }
    }
    else {
      iVar2.field34_0x24 = 0x64;
    }
  }
  return;
}
pub unsafe fn pass1_1030_c230(mut param_1: u32,mut param_2: u32)

{
  let mut BVar1: bool;
  let mut uVar2: u16;
  in_stack_0000ffd8: HFILE16;
  let mut local_10: [u32;0x2] = [0;0x2];
  let mut local_8: [u16;0x3] = [0;0x3];

  BVar1 = write_to_file_1028_b5ec(param_1,param_2);
  if (BVar1 != 0) {
    uVar2 = (param_1 >> 0x10);
    local_10[0] = (param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_10),0x4,in_stack_0000ffd8);
    if (BVar1 != 0) {
      local_8[0] = (param_1 + 0x24);
      BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffd8);
      if (BVar1 != 0) {
        return;
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}
pub unsafe fn pass1_1030_c29c(mut param_1: i16,param_2: *mut u8,param_3: *mut astruct_373,param_4: *mut HFILE16)

{
  let mut BVar1: bool;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0) {
    BVar1 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | (param_3 + 0x20)),0x4);
    if (BVar1 != 0) {
      BVar1 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | (param_3 + 0x24)),0x2);
      if (BVar1 != 0) {
        return;
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_c2fa(mut param_1: i16,param_2: *mut u8,mut param_3: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut unaff_SI: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut puVar11: *mut u32;
  let mut paVar12: *mut astruct_27;
  let mut in_stack_0000fe4e: u16;
  let mut in_stack_0000ff72: u16;
  let mut in_stack_0000ff78: u16;
  let mut in_stack_0000ff7c: u16;
  let mut uVar13: u16;
  let mut uStack84: u16;
  let mut lStack80: i32;
  let mut iStack56: i16;
  let mut paStack10: *mut astruct_305;
  let mut uStack6: u32;
  let mut iVar5: *mut astruct_698;

  paVar8 = CONCAT22(in_register_0000000a,param_2);
  uVar9 = (param_3 >> 0x10);
  if ((param_3 + 0xc) != 0xb) {
    pass1_1028_bd38(param_2,param_3);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(param_3 + 0x20));
    uVar7 = paVar8;
    uStack6 = CONCAT22(uVar7,param_1);
    iVar4 = param_1;
    pass1_1028_b58e(param_3);
    paStack10 = CONCAT22(paVar8,iVar4);
    uVar1 = (iVar4 + 0x2e);
    puVar11 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2f),in_stack_0000fe4e,
                              in_stack_0000ff72,in_stack_0000ff78,in_stack_0000ff7c);
    paVar8 = (paVar8 & 0xffff0000 | puVar11 >> 0x10);
    uVar10 = (uVar1 >> 0x10);
    pass1_1010_ed22(puVar11,(uVar1 + 0x4));
    uVar2 = (param_1 + 0x1f6);
    uVar6 = uVar2;
    pass1_1030_3694(paVar8,uVar2,0x3,0x2);
    uVar9 = (uVar2 >> 0x10);
    uVar3 = (uVar1 + 0x1f6);
    pass1_1030_355c(uVar3,uVar6 & 0xffff | paVar8 << 0x10);
    uVar10 = (uVar3 >> 0x10);
    iStack56 = 0;
    loop {
      iVar5 = (iStack56 * 0x2);
      (iVar5 + uVar3 + 0x174) = (iVar5 + uVar2 + 0x174);
      uVar5 = (iVar5 + uVar2 + 0x180);
      uVar6 = uVar5;
      (iVar5 + uVar3 + 0x180) = uVar5;
      iStack56 += 0x1;
    } while (iStack56 < 0x6);
    for (uStack84 = 0x11; uVar5 = uVar6, uStack84 < 0x25; uStack84 += 1) {
      if (0x0 < (uStack84 * 0x2 + _PTR_LOOP_1050_580e)) {
        empty_1038_540a();
        lStack80 = CONCAT22(paVar8,uVar5);
        uVar9 = (_PTR_LOOP_1050_580e >> 0x10);
        iVar4 = (uStack84 * 0x2 + _PTR_LOOP_1050_580e);
        paVar8 = (iVar4 >> 0x10);
        uVar13 = uStack84;
        if (lStack80 < iVar4) {
          iVar4 = (uStack84 * 0x2 + _PTR_LOOP_1050_580e);
          paVar8 = (iVar4 >> 0xf);
          uVar13 = 0x21;
        }
        pass1_1038_52b8(uStack6,CONCAT22(paVar8,iVar4),uVar13);
        uVar5 = (uStack84 * 0x2 + _PTR_LOOP_1050_580e);
        pass1_1030_7ddc(uVar5,paVar8,paStack10,uVar5,uStack84);
        iVar4 = (_PTR_LOOP_1050_580e + uStack84 * 0x2);
        uVar6 = iVar4;
        pass1_1038_5694(uVar1,iVar4,uStack84);
      }
    }
    pass1_1030_7c50(uVar5,paVar8,paStack10,0x2,1);
    pass1_1030_7c50(uVar5,paVar8,paStack10,0x2,0x2);
    pass1_1030_7c50(uVar5,paVar8,paStack10,0x2,0x3);
    pass1_1030_7c50(uVar5,paVar8,paStack10,0x2,0x4);
    pass1_1038_44d8(uVar5,paVar8,param_1,uVar7,0x2,1);
    pass1_1038_44d8(uVar5,paVar8,param_1,uVar7,0x2,0x2);
    pass1_1038_44d8(uVar5,paVar8,param_1,uVar7,0x2,0x3);
    pass1_1038_44d8(uVar5,paVar8,param_1,uVar7,0x2,0x4);
    paVar12 =
              mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2b),in_stack_0000fe4e,
                              in_stack_0000ff72,in_stack_0000ff78,in_stack_0000ff7c);
    pass1_1010_043a(paVar12,(param_1 + 0x4),0x7);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_c52e(mut param_1: i16,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,mut param_5: u32,mut param_6: u32)

{
  let mut BVar1: bool;
  let mut puVar2: *mut u32;
  let mut paVar3: *mut astruct_92;
  let mut puVar4: *mut u32;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut uVar9: u16;
  let mut puVar10: *mut u16;
  let mut local_32: *mut astruct_92;
  let mut local_20: u32;
  let mut uStack28: u32;
  let mut puStack24: *mut u32;
  let mut uStack22: u32;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut uStack6: u32;

  uVar9 = (param_3 >> 0x10);
  BVar1 = pass1_1028_c314(param_1,param_2,param_3,uVar9,param_4,param_5,(param_5 >> 0x10),
                          param_6);
  if (BVar1 != 0) {
    puVar2 = &local_c;
    pass1_1030_64ce(puVar2,param_2,_PTR_LOOP_1050_5740,param_4,param_6,CONCAT22(0x1050,puVar2));
    local_20 = *puVar2;
    local_20._3_1_ = (local_20 >> 0x18);
    uStack8 = local_20._3_1_;
    if (local_20._3_1_ == 0) {
      uStack22 = local_20;
      uStack6 = local_20;
      pass1_1028_c7b6(param_2,param_3,uVar9,param_4,param_6);
      if ((uStack8 != 0x4) && (uStack8 != 0)) {
        uVar8 = pass1_1030_bcae(&local_20,&DAT_1050_1050);
        uVar6 = (uVar8 >> 0x10);
        pass1_1028_dc52(CONCAT22(0x1050,&local_32),0x1,0x0,0x400);
        loop {
          paVar3 = &local_32;
          pass1_1028_e4ec(CONCAT22(0x1050,paVar3));
          uStack28 = CONCAT22(uVar6,paVar3);
          uVar7 = uVar6 | paVar3;
          if (uVar7 == 0) {
            return;
          }
          uVar5 = &paVar3.field6_0x10;
          uVar8 = param_6;
          uStack22 = uVar5;
          puVar10 = param_4;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5);
          uStack18 = uVar5;
          puVar4 = &local_20;
          uStack16 = uVar7;
          pass1_1030_bcde(puVar4,&DAT_1050_1050,uVar5 & 0xffff | uVar7 << 0x10,puVar10,uVar8);
          if (puVar4 < 0x0) { break; }
          uVar6 = uVar7;
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
// WARNING: Restarted to delay deadcode elimination for space: stack
pub unsafe fn pass1_1030_c652(param_1: *mut u8,mut param_2: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut astruct_250;
  let mut in_stack_0000fea8: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd2: u16;
  let mut in_stack_0000ffd6: u16;

  paVar1 =
           mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           CONCAT22(param_2,0x8),in_stack_0000fea8,in_stack_0000ffcc,in_stack_0000ffd2,
                           in_stack_0000ffd6);
  pass1_1010_9794(paVar1);
  return;
}



pub unsafe fn pass1_1030_c668(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn struct_1030_c6f6(param_1: *mut astruct_180) -> *mut astruct_180

{
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = 0;
  param_1.field0_0x0 = 0xc940;
  (param_1 + 0x2) = 0x1030;
  return param_1;
}



pub unsafe fn pass1_1030_c71e(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> *mut u16

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  (param_2 + 0x20) = 0;
  param_2.field0_0x0 = 0xc940;
  (param_2 + 0x2) = 0x1030;
  return &param_2.field0_0x0;
}
pub unsafe fn pass1_1030_c74e(mut param_1: u16 ,param_2: *mut astruct_15,mut param_3: u32)

{
  pass1_1028_b46e(param_1,param_2,param_3);
  (param_2 + 0x20) = 0x70;
  return;
}
pub unsafe fn pass1_1030_c76c(param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (((iVar1 + 0x12) != 0x6) && ((iVar1 + 0x12) != 0x5)) {
    return;
  }
  iVar1 = (iVar1 + 0x20);
  if (iVar1 != 0) {
    if (((iVar1 < 0x70) || (SBORROW2(iVar1,0x70))) || (0x1 < iVar1 + -0x70)) {
      pass1_1028_be2a(param_1);
      return;
    }
  }
  pass1_1028_bdac(param_1,0x6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_c7b0(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut iVar5: i16;
  let mut BVar6: bool;
  let mut uVar7: u32;
  let mut extraout_DX: *mut u8;
  let mut puVar8: *mut u8;
  let mut iVar9: i16;
  let mut uVar10: u16;

  uVar10 = (param_1 >> 0x10);
  iVar9 = param_1;
  iVar1 = (iVar9 + 0x20);
  if (iVar1 != 0) {
    iVar4 = iVar1 + -0x70;
    iVar5 = iVar4;
    if (((iVar1 < 0x70) || (SBORROW2(iVar1,0x70))) || (iVar5 = iVar1 + -0x71, iVar5 != 0x0 && 0x0 < iVar4)) {
      pass1_1028_b58e((param_1 & 0xffff | uVar10 << 0x10));
      uVar2 = (iVar5 + 0x2e);
      uVar7 = (uVar2 + 0x200);
      puVar8 = extraout_DX;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7);
      uVar3 = uVar7 & 0xffff | ZEXT24(puVar8) << 0x10;
      BVar6 = pass1_1008_c6ae(_u16_1050_06e0,(iVar9 + 0xc),0x11);
      pass1_1030_23e2(BVar6,puVar8,uVar3,BVar6,(iVar9 + 0x20));
      if (BVar6 != 0) {
        if ((iVar9 + 0x20) == 1) {
          pass1_1030_25d8(uVar3,0x64,(iVar9 + 0x20));
          return;
        }
        (iVar9 + 0x20) = 0x70;
      }
    }
  }
  return;
}



pub unsafe fn pass1_1030_c84e(mut param_1: u32,mut param_2: u32) -> BOOL16

{
  let mut BVar1: bool;
  in_stack_0000ffde: HFILE16;
  let mut local_c: [u16;0x5] = [0;0x5];

  BVar1 = write_to_file_1028_b5ec(param_1,param_2);
  if (BVar1 != 0) {
    local_c[0] = (param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffde);
    if (BVar1 == 0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}



pub unsafe fn pass1_1030_c894(param_1: BOOL16,param_2: *mut u8,param_3: *mut astruct_373,param_4: *mut HFILE16) -> BOOL16

{
  let mut BVar1: bool;
  let mut local_4: u16;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0) {
    BVar1 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2);
    if (BVar1 == 0) {
      u16_1050_0310 = 0x6d2;
      return BVar1;
    }
    (param_3 + 0x20) = local_4;
    param_1 = 0x1;
  }
  return param_1;
}



pub unsafe fn pass1_1030_c8da(mut param_1: u32,mut param_2: u32,mut param_3: u32) -> u32

{
  let mut uVar1: u32;

  uVar1 = 0;
  if (param_3 == 1) {
    (param_1 + 0x20) = param_2;
  }
  else {
    uVar1 = FUN_1030_178e();
  }
  return uVar1;
}



pub unsafe fn pass1_1030_c91a(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn struct_1030_c9a8(param_1: *mut astruct_180) -> *mut astruct_180

{
  let mut iVar1: *mut astruct_180;
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x4].field16_0x18 = 0x1;
  param_1.field0_0x0 = 0xd88e;
  iVar1.field1_0x2 = 0x1030;
  pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(iVar1 + 1)),NULL,0x78);
  return param_1;
}



pub unsafe fn pass1_1030_c9e4(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> u32

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  (param_2 + 0x98) = 0x1;
  param_2.field0_0x0 = 0xd88e;
  (param_2 + 0x2) = 0x1030;
  pass1_1000_4906((param_2 & 0xffff0000 | (param_2 + 0x20)),NULL,0x78);
  return param_2;
}
pub unsafe fn pass1_1030_ca26(mut param_1: u16 ,param_2: *mut astruct_15,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut extraout_DX: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uStack4: u16;

  for (uStack4 = 0; iVar2 = param_2, uVar3 = (param_2 >> 0x10), uStack4 < 0xa;
      uStack4 += 1) {
    if (((iVar2 + uStack4 * 0xc + 0x26) == 0x2) || ((iVar2 + uStack4 * 0xc + 0x26) == 1)) {
      (iVar2 + uStack4 * 0xc + 0x26) = 0x4;
      param_1 = uStack4;
    }
    else {
      uVar1 = uStack4;
      pass1_1028_b58e(param_2);
      iVar2 = uStack4 * 0xc + iVar2;
      pass1_1030_6e9c(CONCAT22(extraout_DX,uVar1),0x1,(iVar2 + 0x24));
      param_1 = 0;
      (iVar2 + 0x20) = 0;
      (iVar2 + 0x24) = 0;
      (iVar2 + 0x26) = 0;
    }
  }
  pass1_1028_b46e(param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_cac2(mut param_1: i16,param_2: *mut u32)

{
  let mut uVar1: u32;
  let mut puVar2: *mut u32;
  let mut ppcVar3: *mut *mut code;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut puVar7: *mut u32;
  let mut uVar8: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut extraout_DX_01: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uStack34: u32;
  let mut iStack30: i16;
  let mut iStack28: i16;

  pass1_1028_be9e(param_2);
  uVar10 = (param_2 >> 0x10);
  if (((param_2 + 0x12) == 0x5) && (PTR_LOOP_1050_5812.is_null())) {
    PTR_LOOP_1050_5812 = (&PTR_LOOP_1050_0000 + 1);
    pass1_1028_b58e((param_2 & 0xffff | uVar10 << 0x10));
    uVar1 = (param_1 + 0x2e);
    uVar6 = (uVar1 + 0x10);
    uVar10 = extraout_DX;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6);
    puVar2 = (uVar6 + 0x1e);
    ppcVar3 = (*puVar2 + 0x10);
    puVar7 = puVar2;
    (**ppcVar3)(0x1028,puVar2,(uVar6 + 0x20));
    uVar4 = puVar7 & 0xffff | extraout_DX_00 << 0x10;
    iStack28 = 0;
    iStack30 = pass1_1030_d144(param_2);
    uStack34 = 0;
    while ((uStack34 < uVar4 && (iStack30 != 0))) {
      ppcVar3 = (*puVar2 + 0x4);
      uVar8 = uVar4;
      (**ppcVar3)(0x1028,puVar2,(puVar2 >> 0x10),uStack34,(uStack34 >> 0x10));
      uVar5 = uVar8;
      uVar9 = extraout_DX_01 | uVar5;
      if (uVar9 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar8 & 0xffff | extraout_DX_01 << 0x10);
        uVar5 = (uVar5 + 0xc);
        if ((0x0 < uVar5) && (!SBORROW2(uVar5,1))) {
          if (uVar5 != 0x3 && 0x0 < (uVar5 - 0x2)) {
//            if (uVar5 != 0x4) goto LAB_1030_cbbc;
            iStack28 += 0x1;
          }
          pass1_1030_6e9c((uVar6 & 0xffff | uVar10 << 0x10),0x1,uVar5);
          pass1_1030_d180(param_2,uVar5);
          iStack30 += -0x1;
        }
      }//
// LAB_1030_cbbc:
      uStack34 += 0x1;
    }
    while (iStack28 < 0x4) {
      pass1_1030_d180(param_2,0x4);
      iStack28 = iStack28 + 1;
    }
  }
  return;
}



pub unsafe fn pass1_1030_cbf0(mut param_1: i16,mut param_2: u16 ,mut param_3: i16) -> u16

{
  let mut iVar1: *mut astruct_595;
  let mut iStack4: i16;

  iStack4 = 0;
  loop {
    if (0x9 < iStack4) {
      return 0x0;
    }
    iVar1 = (iStack4 * 0xc + param_1);
    if ((iVar1.field36_0x24 == param_3) && (iVar1.field37_0x26 == 0x3)) { break; }
    iStack4 += 0x1;
  }
  iVar1.field37_0x26 = 0;
  iVar1.field38_0x28 = 0;
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_cc44(mut param_1: i16,mut param_2: u16 ,mut param_3: i16,mut param_4: u32,mut param_5: i16)

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut puVar3: *mut u8;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut puVar7: *mut u8;
  let mut extraout_DX_01: u16;
  let mut iVar7: *mut astruct_304;
  let mut iVar8: *mut astruct_303;
  let mut uVar8: u8;
  let mut puVar9: *mut u32;
  let mut puVar10: *mut u32;
  let mut puVar11: *mut u8;
  let mut local_32: [u8;0x8] = [0;0x8];
  let mut puStack42: *mut u32;
  let mut uStack38: u32;
  let mut uStack34: u32;
  let mut puStack30: *mut u32;
  let mut uStack26: u16;
  let mut puStack24: *mut u8;
  let mut uStack22: u16;
  let mut puStack20: *mut u8;
  let mut puStack18: *mut u32;
  let mut iStack14: i16;
  let mut uStack12: u16;
  let mut iStack10: i16;
  let mut uStack8: u32;
  let mut iStack4: i16;

  iStack4 = 0;
  uStack8 = (param_4 + 0x4);
  iStack10 = 0;
  loop {
    if (0x9 < iStack10) {
      return;
    }
    iVar8 = (iStack10 * 0xc + param_1);
    if (((iVar8.field35_0x28 == uStack8) && (iVar8.field36_0x2a == uStack8)) &&
       (iVar8.field33_0x24 == param_5)) {
      if (iVar8.field34_0x26 == 0x4) {
        iVar2 = param_5;
        pass1_1028_b58e(CONCAT22(param_2,param_1));
        iStack14 = iVar2;
        uStack12 = extraout_DX_00;
        pass1_1030_6e9c(
                        CONCAT13((extraout_DX_00 >> 0x8),CONCAT12(extraout_DX_00,iStack14)),0x1,
                        iVar8.field33_0x24);
        iVar8.field32_0x20 = 0;
        iVar8.field33_0x24 = 0;
        iVar8.field34_0x26 = 0;
        puStack42 = null_mut();
        puStack18 = null_mut();
        _DAT_0000_0006 = param_5;
        uRam0000000a = 0x1;
        uVar4 = switch_1020_c3b4(param_5);
        (puStack18 + 0xc) = uVar4;
        puVar10 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
        puVar7 = (puVar10 >> 0x10);
        uVar6 = puVar10;
        uVar5 = uVar6;
        puVar11 = puVar7;
        uStack22 = uVar6;
        puStack20 = puVar7;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
        uVar8 = 0x38;
        uStack26 = uVar6;
        puStack24 = puVar7;
        pass1_1038_4d6e(uVar6,puVar7,CONCAT22(puVar7,uVar6),CONCAT22(puVar11,uVar5));
        puStack30 = CONCAT22(puVar7,uVar6);
        ppcVar1 = (*puStack30 + 0x10);
        (**ppcVar1)(&u16_1050_1038,uVar6,puVar7);
        uStack34 = CONCAT22(extraout_DX_01,uVar6);
        uVar6 = extraout_DX_01;
        for uStack38 in 0 .. uStack34 {
          puVar9 = pass1_1030_1d7c(uStack34,uVar6,puStack30);
          uVar5 = (puVar9 >> 0x10);
          uVar6 = uVar5 | puVar9;
          if (uVar6 != 0) {
            puVar3 = local_32;
            ppcVar1 = (*puVar9 + 0x40);
            (**ppcVar1)(0x38,puVar9,uVar5,puVar3,&DAT_1050_1050);
            uVar6 = extraout_DX;
            if (puVar3.is_null()) {
              uVar8 = 0x28;
              pass1_1028_6408(puVar9,puStack18);
              break;
            }
          }
        }
        puStack42 = puStack30;
        if (puStack30.is_null() == false) {
          ppcVar1 = *puStack30;
          (**ppcVar1)(uVar8,puStack30,(puStack30 >> 0x10),1);
        }
      }
      else {
        iVar7 = (iStack10 * 0xc + param_1);
        iVar7.field38_0x26 = 0;
        iVar7.field39_0x28 = 0;
      }
      iStack4 += 0x1;
      param_3 += -0x1;
      if (param_3 == 0) {
        return;
      }
    }
    iStack10 += 0x1;
  }
}



pub unsafe fn pass1_1030_cde8(mut param_1: i16,mut param_2: u16 ,mut param_3: i16) -> i16

{
  let mut iVar1: i16;
  let mut iStack4: i16;

  iStack4 = 0;
  loop {
    if (0x9 < iStack4) {
      return -0x1;
    }
    iVar1 = iStack4 * 0xc + param_1;
    if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0)) { break; }
    iStack4 += 0x1;
  }
  return iStack4;
}



// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1030_ce2e(mut param_1: i16,mut param_2: u16 ,mut param_3: i16) -> i16

{
  let mut iVar1: i16;
  let mut uStack6: u32;

  for (uStack6 = 0; uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (uStack6 + 1)) {
    iVar1 = uStack6 * 0xc + param_1;
    if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0)) {
      uStack6 = uStack6 & 0xffff | (uStack6 + 1) << 0x10;
    }
  }
  return uStack6;
}
pub unsafe fn pass1_1030_ce72(mut param_1: u32,mut param_2: i16,mut param_3: u32,mut param_4: i16)

{
  let mut lVar1: i32;
  let mut iVar2: *mut astruct_300;
  let mut iStack10: i16;

  lVar1 = (param_3 + 0x4);
  iStack10 = 0;
  loop {
    if (0x9 < iStack10) {
      return;
    }
    iVar2 = (iStack10 * 0xc + param_1);
    if ((iVar2.field36_0x24 == param_4) && (iVar2.field38_0x28 == 0)) {
      iVar2.field38_0x28 = lVar1;
      if (param_4 == 0x4) {
        iVar2.field37_0x26 = 0x2;
      }
      else {
        (param_1 + iStack10 * 0xc + 0x26) = 0x1;
      }
      param_2 += -0x1;
      if (param_2 == 0) {
        return;
      }
    }
    iStack10 += 0x1;
  }
}
pub unsafe fn pass1_1030_cef8(mut param_1: u32,mut param_2: u32,mut param_3: u16 ,mut param_4: i16)

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  (iVar2 + param_4 * 0xc + 0x26) = param_3;
  uVar4 = (param_2 >> 0x10);
  uVar1 = (param_2 + 0x6);
  (iVar2 + param_4 * 0xc + 0x28) = (param_2 + 0x4);
  (iVar2 + param_4 * 0xc + 0x2a) = uVar1;
  return;
}



pub unsafe fn pass1_1030_cf3a(mut param_1: u32,mut param_2: i16) -> u16

{
  let mut iStack4: i16;

  iStack4 = 0;
  loop {
    if (0x9 < iStack4) {
      return 0x0;
    }
    if ((param_1 + iStack4 * 0xc + 0x24) == param_2) { break; }
    iStack4 += 0x1;
  }
  return 0x1;
}
pub unsafe fn pass1_1030_cf78(param_1: *mut astruct_15,mut param_2: u16 )

{
  let mut uVar1: u32;
  let mut extraout_DX: u16;
  let mut iVar3: *mut astruct_680;
  let mut uVar2: u16;
  let mut iStack4: i16;

  iStack4 = 0;
  loop {
    if (0x9 < iStack4) {
      return;
    }
    uVar1 = param_2;
    uVar2 = (param_1 >> 0x10);
    if ((param_1 + iStack4 * 0xc + 0x24) == param_2) { break; }
    iStack4 += 0x1;
  }
  pass1_1028_b58e(param_1);
  if (param_2 == 0x5) {
    pass1_1038_4900((uVar1 + 0x2e));
  }
  else {
    pass1_1030_6e9c((uVar1 & 0xffff | extraout_DX << 0x10),0x1,param_2);
  }
  iVar3 = (iStack4 * 0xc + param_1);
  iVar3.field32_0x20 = 0;
  iVar3.field33_0x24 = 0;
  iVar3.field34_0x26 = 0;
  return;
}
