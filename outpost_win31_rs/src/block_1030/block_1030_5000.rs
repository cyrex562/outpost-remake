
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_5044(param_1: u16,param_2: *mut astruct_117)

{
  let mut pcVar1: *mut c_char;
  i32 *plVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut pcVar7: *mut c_char;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut extraout_var: u16;
  let mut uStack28: u32;
  let mut uStack24: u16;
  let mut uStack22: u32;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut local_a: i32;
  let mut local_6: *mut c_char;
  let mut uVar8: u32;
  let mut pSVar11: *mut StructD;
  pstruct117_10: *mut astruct_117;
  pstruct117_10_hi: *mut astruct_117;

  plVar2 = &local_a;
  uVar9 = read_file_1030_4e70(param_2,CONCAT22(0x1050,plVar2),CONCAT22(0x1050,&local_6),
                              (i32)s_bldgops_dat_1050_5708,param_1);
  pcVar1 = local_6;
  pSVar11 = CONCAT22(extraout_var,uVar9);
  if (plVar2.is_null() == false) {
    pstruct117_10 = param_2;
    pstruct117_10_hi = (param_2 >> 0x10);
    pcVar7 = local_6;
    pass1_1030_4e34(pstruct117_10,pstruct117_10_hi,local_a,local_6);
    uVar3 = pcVar7;
    if (_PTR_LOOP_1050_5f2c == 0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar11);
      PTR_LOOP_1050_5f2e = pSVar11;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(uVar3 * 0xae,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
    uVar8 = uVar4;
    uStack28 = CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
    uVar10 = PTR_LOOP_1050_5f2e | uVar4;
    if (uVar10 == 0) {
      pstruct117_10[0xd].field10_0xa = 0;
    }
    else {
      pass1_1000_5586(0x51f0,0x1030,uVar3,0xae,uVar4,PTR_LOOP_1050_5f2e);
      pstruct117_10[0xd].field10_0xa = uStack28;
      uVar8 = uStack28;
    }
    uVar5 = uVar8;
    pass1_1030_4dbc(param_2,local_6,pcVar7 & 0xffff);
    uStack22 = CONCAT22(uVar10,uVar5);
    for (uStack24 = 0; uStack24 < uVar3; uStack24 += 1) {
      uVar10 = (&pstruct117_10[0xd].field10_0xa + 2);
      iVar6 = &pstruct117_10[0xd].field10_0xa + uStack24 * 0xae;
      pass1_1030_4c52(pstruct117_10,pstruct117_10_hi,CONCAT22(uVar10,iVar6),uStack22,uVar10);
      pass1_1030_4dbc(param_2,0x0,0x0);
      uStack22 = CONCAT22(uVar10,iVar6);
    }
    uStack12 = (pcVar1 >> 0x10);
    uStack14 = pcVar1;
    if ((uStack12 | uStack14) != 0) {
      call_fn_ptr_1000_0dc6(pcVar1);
    }
  }
  return;
}



char * pass1_1030_5164(param_1: *mut astruct_117,mut param_2: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut lVar4: i32;
  let mut local_a: [u8;0x8] = [0;0x8];

  uVar3 = (param_1 >> 0x10);
  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x568));
  loop {
    lVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (lVar4 == 0) {
      return param_2;
    }
    uVar1 = param_1 + 0x168;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | uVar1),*(lVar4 + 0x4));
    pass1_1000_3cea(param_1 & 0xffff0000 | uVar1,param_2);
    uVar2 = dos3_call_1000_51aa(uVar1,uVar3,1);
  } while (uVar2 != 0);
  return (param_1 & 0xffff0000 | uVar1);
}
pub fn pass1_1030_51eb()

{
  pass1_1030_3b28();
  return;
}



pub fn pass1_1030_51f0(mut param_1: u32) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0xa4) = 0;
  (iVar1 + 0xa6) = 0;
  (iVar1 + 0xa8) = 0;
  (iVar1 + 0xaa) = 0;
  (iVar1 + 0xac) = 0;
  return param_1;
}
pub fn pass1_1030_521c(param_1: *mut astruct_97,mut param_2: u32)

{
  iVar1: *mut astruct_97;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x32c7);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  &iVar1.field259_0x108 = param_2;
  param_1->offset_0x0 = 0x55fe;
  iVar1->segment_0x2 = 0x1030;
  sys_1000_3f9c((param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCGenKids_0x_08lx_1050_5714,
                param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1030_5260(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut puStack6: *mut u32;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(param_3 + 0x108));
  puStack6 = CONCAT22(param_2,param_1);
  ppcVar1 = (*puStack6 + 0x14);
  (**ppcVar1)();
  return 0x1;
}
pub fn pass1_1030_5290(param_1: *mut astruct_376,param_2: *mut u8,param_3: *mut astruct_377)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  iVar5: *mut astruct_377;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  uVar8: *mut astruct_377;
  let mut puStack10: *mut u16;

  paVar5 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10c,paVar5);
  uVar4 = paVar5;
  puStack10 = CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0) {
    *puStack10 = 0x389a;
    param_1.field2_0x2 = 0x1008;
    uVar8 = (param_3 >> 0x10);
    iVar5 = param_3;
    param_1.field3_0x4 = iVar5.field4_0x4;
    puVar6 = &iVar5.field5_0x8;
    puVar7 = &param_1.field4_0x8;
    for (iVar3 = 0x40; iVar3 != 0; iVar3 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1.field2_0x2 = 0x1028;
    param_1.field257_0x108 = iVar5.field258_0x108;
    *puStack10 = 0x55fe;
    param_1.field2_0x2 = 0x1030;
  }
  return;
}
pub fn pass1_1030_532e(param_1: *mut astruct_97,mut param_2: u32)

{
  iVar1: *mut astruct_97;
  uVar1: *mut astruct_97;

  struct_op_1028_d1dc(param_1,0x32c7);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  &iVar1.field259_0x108 = param_2;
  param_1->offset_0x0 = 0x55ee;
  iVar1->segment_0x2 = 0x1030;
  sys_1000_3f9c((param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCSelect__u___d_1050_5726,
                &iVar1.field_0x4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1030_538a(param_1: *mut astruct_694) -> u16

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  iVar4: *mut astruct_694;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000fff0: u16;

  uVar3 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar1 = (&iVar4.field264_0x108 + 2);
  uVar2 = uVar1 >> 0x8;
  puVar4 = mixed_1010_20ba((in_EDX & 0xffff0000 | uVar1),_u16_1050_0ed0,
                           CONCAT22(in_stack_0000fff0,0x2f),in_stack_0000fe98,in_stack_0000ffbc,
                           in_stack_0000ffc2,in_stack_0000ffc6);
  if (uVar2 == 1) {
    pass1_1018_04ca(puVar4,iVar4.field264_0x108);
  }
  else if (uVar2 == 0x2) {
    pass1_1018_04a4(puVar4,iVar4.field264_0x108);
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_53f4(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut bStack291: u8;
  let mut local_11e: [u8;0x10e] = [0;0x10e];
  let mut uStack16: u32;
  let mut uStack12: u32;

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  uVar6 = (param_2 >> 0x10);
  iVar5 = param_2;
  uStack12 = (iVar5 + 0x108);
  uStack12._3_1_ = (uStack12 >> 0x18);
  if (uStack12._3_1_ == -1) {
    uVar7 = pass1_1028_e2e0(paVar4,_PTR_LOOP_1050_65e2,((iVar5 + 0x108) >> 0x18));
    uVar3 = (uVar7 >> 0x10);
  }
  else {
    uStack16 = (iVar5 + 0x108);
    uStack16._3_1_ = (uStack16 >> 0x18);
    if (uStack16._3_1_ == '\x03') {
      pass1_1028_e44a(_PTR_LOOP_1050_65e2,(iVar5 + 0x108));
      uVar3 = SUB42(paVar4,0x0);
    }
    else {
      uVar1 = (iVar5 + 0x108);
      pass1_1028_e372(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
      uVar3 = SUB42(paVar4,0x0);
    }
  }
  uStack12 = (iVar5 + 0x108);
  uStack12._3_1_ = (uStack12 >> 0x18);
  if (uStack12._3_1_ != '\x03') {
    pass1_1030_521c(CONCAT13(0x10,CONCAT12(0x50,local_11e)),(iVar5 + 0x108));
    uStack16 = *_u16_1050_5748;
    fn_ptr_1028_d566(uStack16,CONCAT22(0x1050,local_11e));
    bStack291 = ((iVar5 + 0x108) >> 0x18);
    uVar2 = bStack291;
    if (bStack291 == 0x2) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(iVar5 + 0x108));
      pass1_1010_82f8(_u16_1050_14cc,*(u16*)(uVar2 + 0x10));
    }
  }
  return;
}
pub fn pass1_1030_54f8(param_1: *mut astruct_378,param_2: *mut u8,param_3: *mut astruct_379)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  iVar5: *mut astruct_379;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  uVar8: *mut astruct_379;
  let mut puStack10: *mut u16;

  paVar5 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10c,paVar5);
  uVar4 = paVar5;
  puStack10 = CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0) {
    *puStack10 = 0x389a;
    param_1.field2_0x2 = 0x1008;
    uVar8 = (param_3 >> 0x10);
    iVar5 = param_3;
    param_1.field3_0x4 = iVar5.field4_0x4;
    puVar6 = &iVar5.field5_0x8;
    puVar7 = &param_1.field4_0x8;
    for (iVar3 = 0x40; iVar3 != 0; iVar3 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1.field2_0x2 = 0x1028;
    param_1.field257_0x108 = iVar5.field258_0x108;
    *puStack10 = 0x55ee;
    param_1.field2_0x2 = 0x1030;
  }
  return;
}



pub fn pass1_1030_5596(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub fn pass1_1030_55c2(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1030_560e(param_1: *mut astruct_180)

{
  let mut in_EDX: u32;
  iVar1: *mut astruct_180;
  uVar2: *mut astruct_180;

  struct_1030_17ce(param_1,0x64,0x1f4,in_EDX);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  &iVar1.field12_0x10 = 0;
  pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar1.field14_0x14)));
  iVar1.field17_0x1a = 0;
  &iVar1.field18_0x1c = 0;
  param_1.field0_0x0 = s_procLo_1050_5bd0;
  iVar1.field1_0x2 = 0x1030;
  return &param_1.field0_0x0;
}



u16 * struct_1030_565a(mut param_1: u16 ,param_2: *mut astruct_57,param_3: *mut astruct_352,mut param_4: u32)

{
  pstruct_1: *mut astruct_353;
  pstruct_1_hi: *mut astruct_352;

  pass1_1030_183c(param_1,param_2,&param_3->u16_field_0x0,0x64,0x1f4,0x3000000,param_4);
  pstruct_1_hi = (param_3 >> 0x10);
  pstruct_1 = param_3;
  pstruct_1.field15_0x10 = 0;
  pass1_1008_3e38((param_3 & 0xffff0000 | ZEXT24(&pstruct_1.field16_0x14)));
  pstruct_1.field21_0x1a = 0;
  pstruct_1.field22_0x1c = 0;
    // 0x5bd0
  param_3->u16_field_0x0 = s_procLo_1050_5bd0;
  pstruct_1.field2_0x2 = 0x1030;
  return &param_3->u16_field_0x0;
}
pub fn pass1_1030_56b0(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut pcVar2: *mut c_char;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  *param_1 = s_procLo_1050_5bd0;
  (iVar3 + 0x2) = 0x1030;
  pcVar2 = *(iVar3 + 0x10);
  uVar1 = (iVar3 + 0x12);
  if ((uVar1 | pcVar2) != 0) {
    fn_ptr_1030_84d0(pcVar2 & 0xffff | uVar1 << 0x10);
    fn_ptr_1000_17ce(pcVar2);
  }
  pass1_1030_18b2(param_1);
  return;
}
pub fn pass1_1030_56f6(param_1: u16,param_2: *mut astruct_731,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut BVar5: bool;
  let mut iVar6: i16;
  iVar7: *mut astruct_731;
  uVar8: *mut astruct_731;
  in_stack_0000ffd6: mut HFILE16;
  u16 local_e [0x3];
  u16 local_8 [0x2];
  let mut iStack4: i16;

  uVar4 = pass1_1030_1978(param_1,param_2,param_3);
  if (uVar4 != 0) {
    uVar8 = (param_2 >> 0x10);
    iVar7 = param_2;
    local_e[0] = *(u16*)&iVar7.field_0x10;
    BVar5 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_e),0x2,in_stack_0000ffd6);
    if (BVar5 != 0) {
      uVar3 = &iVar7.field_0x10;
      local_8[0] = (uVar3 + 2);
      BVar5 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffd6);
      if ((BVar5 != 0) &&
         (uVar3 = &iVar7.field_0x10, BVar5 = pass1_1008_7c2a(param_3,*(uVar3 + 0x4)),
         BVar5 != 0)) {
        uVar3 = &iVar7.field_0x10;
        local_8[0] = (uVar3 + 0x1a);
        BVar5 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffd6);
        if (BVar5 != 0) {
          for (iStack4 = 0; uVar3 = &iVar7.field_0x10, piVar1 = (uVar3 + 0x1a),
              *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 += 1) {
            uVar3 = &iVar7.field_0x10;
            uVar2 = (uVar3 + 0x16);
            iVar6 = write_to_file_1008_7b4c
                              (param_3,(uVar2 & 0xffff0000 | (uVar2 + iStack4 * 0x6)));
            if (iVar6 == 0) goto LAB_1030_5734;
          }
          iVar6 = write_to_file_1008_7b4c
                            (param_3,(param_2 & 0xffff0000 | ZEXT24(&iVar7.field19_0x14)));
          if (iVar6 != 0) {
            local_8[0] = &iVar7.field_0x1c;
            BVar5 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_8),0x2,in_stack_0000ffd6);
            if (BVar5 != 0) {
              return;
            }
          }
        }
      }
    }//
// LAB_1030_5734:
    u16_1050_0310 = 0x6d0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn file_1030_581e(mut param_1: i16,param_2: *mut u8,param_3: *mut astruct_381,mut param_4: u32)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut uVar3: u32;
  paVar4: *mut astruct_380;
  let mut BVar5: bool;
  let mut puVar6: *mut u8;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut in_register_0000000a: u16;
  let mut pSVar11: *mut StructD;
  let mut paVar13: *mut Struct57;
  iVar9: *mut astruct_380;
  let mut uVar14: u16;
  let mut in_stack_0000fae2: u16;
  let mut uStack1040: u32;
  let mut iStack1036: i16;
  let mut local_408: [u8;0x400] = [0;0x400];
  let mut uStack8: u32;
  let mut local_4: i16;
  uVar15: *mut astruct_381;
  iVar12: *mut astruct_381;
  let mut paVar12: *mut Struct57;

  pSVar11 = CONCAT22(in_register_0000000a,param_2);
  iVar12 = param_3;
  uVar15 = (param_3 >> 0x10);
  file_1030_19b4(param_1,param_2,param_3,(HFILE16 *)param_4);
  if (param_1 != 0) {
    if (_PTR_LOOP_1050_5f2c == 0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar11);
    }
    else {
      pSVar11 = (pSVar11 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    paVar4 = fn_ptr_op_1000_1708(0x20,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar11);
    uVar9 = pSVar11 | paVar4;
    paVar13 = (pSVar11 & 0xffff0000);
    paVar12 = (paVar13 | uVar9);
    if (uVar9 == 0) {
      paVar4 = NULL;
    }
    else {
      pass1_1030_84ae(CONCAT22(pSVar11,paVar4));
      paVar13 = paVar12;
    }
    iVar12.field16_0x10 = paVar4;
    iVar12.field17_0x12 = paVar13;
    BVar5 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(0x1050,&local_4),0x2);
    if (BVar5 != 0) {
      uVar8 = (_PTR_LOOP_1050_65e2 + 0x52);
      uStack8 = uVar8;
      pass1_1030_4782(paVar13,uVar8,(uVar8 >> 0x10),0x0,0x1,local_4,in_stack_0000fae2);
      iVar12.field16_0x10 = uVar8;
      iVar12.field17_0x12 = paVar13;
      BVar5 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(paVar13,&iVar12.field16_0x10.field_0x2),
                                  0x2);
      if (BVar5 != 0) {
        puVar6 = local_408;
        read_file_1008_7c6e(param_4,(param_4 >> 0x10),CONCAT22(0x1050,puVar6));
        if (puVar6.is_null() == false) {
          uVar8 = &iVar12.field16_0x10;
          fn_ptr_1000_17ce(*(uVar8 + 0x4));
          uVar7 = str_op_1008_60e8(paVar13,CONCAT22(0x1050,local_408));
          uVar8 = &iVar12.field16_0x10;
          uVar14 = (uVar8 >> 0x10);
          iVar9 = uVar8;
          iVar9.field4_0x4 = uVar7;
          iVar9.field5_0x6 = paVar13;
          uVar3 = &iVar12.field16_0x10;
          BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(uVar3 & 0xffff0000 | (uVar3 + 0x1a)),0x2);
          if (BVar5 != 0) {
            uVar8 = &iVar12.field16_0x10;
            iVar2 = (uVar8 + 0x1a);
            uVar9 = iVar2 * 0x6;
            mem_op_1000_179c(uVar9,paVar13);
            uVar10 = paVar13;
            uStack1040 = CONCAT22(uVar10,uVar9);
            if ((uVar10 | uVar9) == 0) {
              uVar8 = &iVar12.field16_0x10;
              (uVar8 + 0x16) = 0;
            }
            else {
              pass1_1000_5586(0x3e38,0x1008,iVar2,0x6,uVar9,uVar10);
              uVar8 = &iVar12.field16_0x10;
              (uVar8 + 0x16) = uStack1040;
            }
            for (iStack1036 = 0; uVar8 = &iVar12.field16_0x10, piVar1 = (uVar8 + 0x1a),
                *piVar1 != iStack1036 && iStack1036 <= *piVar1; iStack1036 += 1) {
              uVar8 = &iVar12.field16_0x10;
              uVar3 = (uVar8 + 0x16);
              BVar5 = read_file_1008_7bc8(param_4,
                                                  (uVar3 & 0xffff0000 | (uVar3 + iStack1036 * 0x6)));
              if (BVar5 == 0) goto LAB_1030_58a7;
            }
            BVar5 = read_file_1008_7bc8(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar12.field_0x14)));
            if ((BVar5 != 0) &&
               (BVar5 = read_file_1008_7dee((HFILE16 *)param_4,
                                            (param_3 & 0xffff0000 | ZEXT24(&iVar12.field_0x1c)),0x2),
               BVar5 != 0)) {
              return;
            }
          }
        }
      }
    }//
// LAB_1030_58a7:
    u16_1050_0310 = 0x6d2;
  }
  return;
}
pub fn pass1_1030_5a52(mut param_1: u32,param_2: *mut u32,param_3: *mut u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x10);
  *param_3 = (uVar1 + 0xe);
  uVar1 = (param_1 + 0x10);
  *param_2 = (uVar1 + 0x12);
  return;
}
pub fn pass1_1030_5a80(mut param_1: u32,mut param_2: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut local_20: [u8;0xc] = [0;0xc];
  let mut local_14: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut iStack6: i16;
  let mut uStack4: u16;

  uVar2 = (param_1 >> 0x10);
  (param_1 + 0x10) = param_2;
  uVar3 = pass1_1008_4772((param_2 + 0xe));
  uStack4 = (uVar3 >> 0x10);
  iStack6 = uVar3;
  uStack10 = (iStack6 + 0x4);
  uStack14 = (iStack6 + 0x8);
  pass1_1008_3e54(CONCAT22(0x1050,&local_14),0x0,uStack14 - 0x1,uStack10 - 1);
  puVar1 = (param_1 + 0x14);
  pass1_1008_6cb4(CONCAT22(0x1050,local_20),&local_14,&DAT_1050_1050,puVar1,uVar2);
  pass1_1008_6d64(CONCAT22(0x1050,local_20),(param_1 & 0xffff0000 | ZEXT24(puVar1)));
  return;
}



pub unsafe fn pass1_1030_5b00(mut param_1: u32) -> i16

{
  return (param_1 + 0x4) + 0xb;
}
pub fn pass1_1030_5b1c(mut param_1: u32,param_2: *mut u16,param_3: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_3 = (param_1 + 0x1a);
  *param_2 = (param_1 + 0x1c);
  return;
}
pub fn pass1_1030_5b3e(mut param_1: u32,mut param_2: i16,mut param_3: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x1a) = param_3;
  if ((iVar1 + 0x1c) < param_2) {
    (iVar1 + 0x1c) = param_2;
  }
  return;
}



pub fn pass1_1030_5b5c(mut param_1: i16,mut param_2: u16 ) -> u32

{
  return CONCAT22(param_2,param_1 + 0x14);
}
pub fn pass1_1030_5b6c(mut param_1: u16 ,param_2: *mut astruct_610,param_3: *mut c_char)

{
  let mut lVar1: i32;
  let mut uVar2: u16;
  iVar4: *mut astruct_610;
  iVar3: *mut astruct_609;
  let mut uVar3: u16;

  uVar3 = (param_2 >> 0x10);
  iVar4 = param_2;
  if (iVar4.field16_0x10 != 0) {
    lVar1 = iVar4.field16_0x10;
    fn_ptr_1000_17ce(*(lVar1 + 0x4));
    uVar2 = str_op_1008_60e8(param_1,param_3);
    lVar1 = iVar4.field16_0x10;
    uVar3 = (lVar1 >> 0x10);
    iVar3 = lVar1;
    iVar3.field4_0x4 = uVar2;
    iVar3.field5_0x6 = param_1;
  }
  return;
}



pub fn pass1_1030_5baa(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1030_56b0(&param_1->address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_5bec(mut param_1: u32)

{
  _PTR_LOOP_1050_5736 = param_1;
  pass1_1000_54a0(param_1,0x0,0x24);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_5c0e()

{
  _PTR_LOOP_1050_5736 = 0;
  return;
}



BOOL16 pass1_1030_5c1a(mut param_1: u32,mut param_2: u32)

{
  let mut BVar1: bool;
  in_stack_0000ffe8: mut HFILE16;

  BVar1 = write_to_file_1008_7cac(param_2);
  if (BVar1 != 0) {
    BVar1 = write_to_file_1008_7e1c(param_2,param_1,0x24,in_stack_0000ffe8);
    if (BVar1 == 0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}



BOOL16 read_file_1030_5c52(param_1: u16,param_2: *mut u8,HFILE16 *param_3)

{
  let mut BVar1: bool;

  read_file_1008_7cfe(param_3,(param_3 >> 0x10),0x9);
  if (param_1 != 0) {
    BVar1 = read_file_1008_7dee(param_3,param_2,0x24);
    if (BVar1 == 0) {
      u16_1050_0310 = 0x6d2;
      return BVar1;
    }
    param_1 = 0x1;
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_5c8a(mut param_1: u32,mut param_2: u32)

{
  i32 *plVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  iVar5: *mut astruct_177;
  let mut uVar5: u16;
  let mut uStack6: u32;

  uStack6 = 0;
  uVar2 = param_2._3_1_;
  if (uVar2 == 0xff) {
    return;
  }
  uVar5 = (_PTR_LOOP_1050_65e2 >> 0x10);
  iVar5 = (_PTR_LOOP_1050_65e2 + 0xa);
  uVar3 = (iVar5 + uVar2 * 0x4);
  uVar4 = (iVar5 + uVar2 * 0x4 + 2);
  if ((uVar3 + 0x4) != 0) {
    pass1_1030_12ca((uVar3 & 0xffff | uVar4 << 0x10));
    uStack6 = uVar3 & 0xffff | uVar4 << 0x10;
  }
  if (uStack6 == 0) {
    plVar1 = (uVar2 * 0x4 + param_1);
    *plVar1 = *plVar1 + 1;
  }
  return;
}



u16 * pass1_1030_5d0a(param_1: *mut u16)

{
  let mut in_EDX: u32;
  let mut uVar1: u16;

  struct_1030_17ce(param_1,0x1,0x4,in_EDX);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x10) = 0;
  *param_1 = 0x613e;
  (param_1 + 0x2) = 0x1030;
  return param_1;
}



u16 * pass1_1030_5d3c(mut param_1: u16 ,param_2: *mut u8,param_3: *mut u16,mut param_4: u32)

{
  let mut in_register_0000000a: u16;
  let mut uVar1: u16;

  pass1_1030_183c(param_1,CONCAT22(in_register_0000000a,param_2),param_3,0x1,0x4,0x1000000,param_4);
  uVar1 = (param_3 >> 0x10);
  (param_3 + 0x10) = 0;
  *param_3 = 0x613e;
  (param_3 + 0x2) = 0x1030;
  return param_3;
}
pub fn pass1_1030_5d78(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut pcVar2: *mut c_char;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  *param_1 = 0x613e;
  (iVar3 + 0x2) = 0x1030;
  pcVar2 = *(iVar3 + 0x10);
  uVar1 = (iVar3 + 0x12);
  if ((uVar1 | pcVar2) != 0) {
    pass1_1030_8480((pcVar2 & 0xffff | uVar1 << 0x10));
    fn_ptr_1000_17ce(pcVar2);
  }
  pass1_1030_18b2(param_1);
  return;
}
pub fn pass1_1030_5dbe(param_1: u16,mut param_2: u32,mut param_3: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut BVar4: bool;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  in_stack_0000ffde: mut HFILE16;
  u16 local_c [0x5];

  uVar3 = pass1_1030_1978(param_1,param_2,param_3);
  if (uVar3 != 0) {
    uVar7 = (param_2 >> 0x10);
    iVar6 = param_2;
    BVar4 = pass1_1008_7c2a(param_3,*(iVar6 + 0x10));
    if ((BVar4 != 0) &&
       (uVar1 = (iVar6 + 0x10),
       iVar5 = write_to_file_1008_7b4c(param_3,(uVar1 & 0xffff0000 | (uVar1 + 0x4))),
       iVar5 != 0)) {
      uVar2 = (iVar6 + 0x10);
      local_c[0] = (uVar2 + 0xa);
      BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffde);
      if (BVar4 != 0) {
        uVar2 = (iVar6 + 0x10);
        if ((uVar2 + 0xa) == 0) {
          return;
        }
        uVar2 = (iVar6 + 0x10);
        uVar7 = (uVar2 >> 0x10);
        iVar6 = uVar2;
        BVar4 = write_to_file_1008_7e1c
                          (param_3,(iVar6 + 0xc),((iVar6 + 0xa) * 0x2),
                           in_stack_0000ffde);
        if (BVar4 != 0) {
          return;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn file_1030_5e70(mut param_1: i16,param_2: *mut u8,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u32;
  let mut puVar2: *mut u16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut puVar5: *mut u8;
  let mut BVar6: bool;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut in_register_0000000a: u16;
  let mut pSVar9: *mut StructD;
  let mut paVar10: *mut Struct57;
  let mut iVar11: i16;
  let mut uVar12: u16;
  let mut puVar13: *mut u16;
  let mut puVar14: *mut u32;
  let mut in_stack_0000fa88: u16;
  let mut in_stack_0000fbac: u16;
  let mut in_stack_0000fbb2: u16;
  let mut in_stack_0000fbb6: u16;
  let mut iVar15: i16;
  let mut uVar16: u16;
  let mut uStack1034: u32;
  let mut local_402: [u8;0x400] = [0;0x400];

  pSVar9 = CONCAT22(in_register_0000000a,param_2);
  iVar15 = param_3;
  uVar16 = (param_3 >> 0x10);
  file_1030_19b4(param_1,param_2,param_3,(HFILE16 *)param_4);
  if (param_1 != 0) {
    if (_PTR_LOOP_1050_5f2c == 0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar9);
    }
    else {
      pSVar9 = (pSVar9 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    uVar4 = fn_ptr_op_1000_1708(0x10,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar9);
    uVar7 = pSVar9;
    uStack1034 = CONCAT22(uVar7,uVar4);
    paVar10 = (pSVar9 & 0xffff0000 | (uVar7 | uVar4));
    if ((uVar7 | uVar4) == 0) {
      (iVar15 + 0x10) = 0;
    }
    else {
      puVar13 = pass1_1008_3e38(CONCAT22(uVar7,uVar4 + 0x4));
      paVar10 = (pSVar9 & 0xffff0000 | puVar13 >> 0x10);
      (iVar15 + 0x10) = uStack1034;
    }
    puVar5 = local_402;
    read_file_1008_7c6e(param_4,(param_4 >> 0x10),CONCAT22(0x1050,puVar5));
    if (puVar5.is_null() == false) {
      uVar4 = str_op_1008_60e8(paVar10,CONCAT22(0x1050,local_402));
      puVar2 = (u16*)(iVar15 + 0x10);
      *puVar2 = uVar4;
      (puVar2 + 0x2) = paVar10;
      uVar1 = (iVar15 + 0x10);
      BVar6 = read_file_1008_7bc8(param_4,(uVar1 & 0xffff0000 | (uVar1 + 0x4)));
      if (BVar6 != 0) {
        uVar1 = (iVar15 + 0x10);
        uVar7 = uVar1 + 0xa;
        BVar6 = read_file_1008_7dee((HFILE16 *)param_4,(uVar1 & 0xffff0000 | uVar7),0x2);
        if (BVar6 != 0) {
          uVar3 = (iVar15 + 0x10);
          uVar12 = (uVar3 >> 0x10);
          iVar11 = uVar3;
          if ((iVar11 + 0xa) == 0) {//
// LAB_1030_5fb7:
            puVar14 = mixed_1010_20ba(paVar10,_u16_1050_0ed0,CONCAT22(uVar7,0x2f),in_stack_0000fa88,
                                      in_stack_0000fbac,in_stack_0000fbb2,in_stack_0000fbb6);
            pass1_1018_04ca(puVar14,(iVar15 + 0x4));
            return;
          }
          uVar8 = (iVar11 + 0xa) * 0x2;
          uVar7 = uVar8;
          mem_op_1000_179c(uVar8,paVar10);
          uVar3 = (iVar15 + 0x10);
          uVar12 = (uVar3 >> 0x10);
          iVar11 = uVar3;
          (iVar11 + 0xc) = uVar8;
          (iVar11 + 0xe) = paVar10;
          uVar3 = (iVar15 + 0x10);
          BVar6 = read_file_1008_7dee((HFILE16 *)param_4,(uVar3 + 0xc),uVar7);
          if (BVar6 != 0) goto LAB_1030_5fb7;
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}
pub fn pass1_1030_5fe2(mut param_1: u32,mut param_2: u32)

{
  (param_1 + 0x10) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_5ff6(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut local_6c: [u8;0x58] = [0;0x58];
  let mut uStack20: u32;
  let mut uStack16: u32;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  uVar7 = (param_3 >> 0x10);
  iVar6 = param_3;
  if ((iVar6 + 0xc) == 0) {
    mem_op_1000_179c(0x18,param_2);
    uStack6 = param_2;
    param_2 = (param_2 & 0xffff0000 | (uStack6 | param_1));
    uStack8 = param_1;
    if ((uStack6 | param_1) == 0) {
      (iVar6 + 0xc) = 0;
    }
    else {
      struct_op_1030_1cd8(CONCAT22(uStack6,param_1),0x5,0x5);
      (iVar6 + 0xc) = param_1;
      (iVar6 + 0xe) = param_2;
    }
  }
  for (uStack4 = 0; uVar3 = (iVar6 + 0x10), puVar1 = (uVar3 + 0xa),
      uStack4 <= *puVar1 && *puVar1 != uStack4; uStack4 += 1) {
    uStack12 = pass1_1028_e2e0(param_2,_PTR_LOOP_1050_65e2,0x2);
    param_2 = (param_2 & 0xffff0000 | uStack12 >> 0x10);
    iVar4 = uStack12;
    uVar3 = (iVar6 + 0xc);
    ppcVar2 = ((iVar6 + 0xc) + 0x8);
    (**ppcVar2)(0x1028,uVar3,(uVar3 >> 0x10),iVar4,(uStack12 >> 0x10),uStack4,0x0);
    pass1_1030_8344(_u16_1050_5748,uStack12);
    uStack16 = CONCAT22(param_2,iVar4);
    uStack20 = (iVar4 + 0x10);
    if ((uStack20 + 0x2) == 0) {
      sys_1000_3f9c(CONCAT22(0x1050,local_6c),s__s__d_1050_573a,
                    (iVar6 + 0x10));
      uVar5 = str_op_1008_60e8(param_2,CONCAT22(0x1050,local_6c));
      uVar8 = (uStack20 >> 0x10);
      (uStack20 + 0x2) = uVar5;
      (uStack20 + 0x4) = param_2;
    }
  }
  return;
}
