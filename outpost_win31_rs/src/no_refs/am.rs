
pub unsafe fn pass1_1038_1faa(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,param_4: *mut u32,param_5: *mut u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  ppcVar1 = (*param_5 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(param_2,param_1);
  uStack10 = 0;
  loop {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (*param_5 + 0x4);
    uVar4 = uStack6;
    (**ppcVar1)();
    uVar2 = uVar4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | param_2 << 0x10);
    uVar4 = struct_op_1030_73a8(CONCAT22(param_2,uVar2),uVar2,param_2);
    param_2 = param_2 & 0xffff0000 | uVar4 >> 0x10;
    uVar3 = uVar4;
    pass1_1038_1d68(uVar3,param_2,param_3,(param_3 >> 0x10),param_4,uVar4);
    if (uVar3 == 0) { break; }
    uStack10 += 0x1;
  }
  return;
}


pub unsafe fn pass1_1038_28d8(param_1: *mut astruct_97) -> *mut astruct_97

{
  struct_op_1028_d1dc(param_1,0x3a97);
  param_1.offset_0x0 = 0x29fe;
    // just 0x1038
  (param_1 + 0x2) = &u16_1050_1038;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)),s_SCRoboMove_1050_59f8);
  return param_1;
}


pub unsafe fn pass1_1038_290e(mut param_1: u16 ,param_2: u8,mut param_3: u16 ) -> u16

{
  let mut unaff_SI: u16;
  let mut unaff_DI: u16;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
  if ((param_3 | param_1) != 0) {
    pass1_1038_4918(param_1,param_3 | param_1,CONCAT22(param_3,param_1));
  }
  pass1_1038_7a76(_PTR_LOOP_1050_5a64,unaff_SI,unaff_DI,&DAT_1050_1050);
  return 0x1;
}


pub unsafe fn pass1_1038_2944(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut puVar3: *mut u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar6 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = paVar6;
  puStack10 = CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    (param_1 + 0x4) = (param_3 + 0x4);
    puVar3 = (param_3 + 0x8);
    puVar7 = (param_1 + 0x8);
    for iVar4 in 0x40 .. 0 {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0x29fe;
    (param_1 + 0x2) = &u16_1050_1038;
  }
  return;
}


pub unsafe fn pass1_1038_29d2(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1.address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1038_2ac2(mut param_1: u16 ,mut param_2: u16 ,param_3: u8,mut param_4: u32) -> u16

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar2 = (param_4 >> 0x10);
  uVar1 = param_4;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(uVar1 + 0x108));
  uStack6 = CONCAT22(param_2,param_1);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(uVar1 + 0x10c));
  uStack10 = CONCAT22(param_2,param_1);
  pass1_1038_2c82(param_3,uVar1,uVar2,(uVar1 + 0x110),CONCAT22(param_2,param_1),uStack6);
  pass1_1038_2c82(param_3,uVar1,uVar2,(uVar1 + 0x114),uStack6,uStack10);
  return 0x1;
}


pub unsafe fn pass1_1038_2b2e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32) -> u16

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uStack6: u32;

  uVar2 = (param_3 >> 0x10);
  uVar1 = param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(uVar1 + 0x108));
  uStack6 = CONCAT22(param_2,param_1);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(uVar1 + 0x10c));
  pass1_1038_2f92(uVar1,uVar2,(uVar1 + 0x110),CONCAT22(param_2,param_1));
  pass1_1038_2f92(uVar1,uVar2,(uVar1 + 0x114),uStack6);
  return 0x1;
}


pub unsafe fn pass1_1038_2b9a(param_1: *mut astruct_422,param_2: *mut u8,param_3: *mut astruct_421)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut uVar4: *mut Struct57;
  let mut iVar5: *mut astruct_421;
  let mut puVar5: *mut u32;
  let mut puVar6: *mut u32;
  let mut uVar7: u16;
  let mut puStack10: *mut u16;

  paVar4 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x118,paVar4);
  uVar4 = paVar4;
  puStack10 = CONCAT22(uVar4,param_1);
  iVar5 = param_3;
  uVar7 = (param_3 >> 0x10);
  if ((uVar4 | param_1) != 0) {
    *puStack10 = 0x389a;
    param_1.field2_0x2 = 0x1008;
    param_1.field3_0x4 = iVar5.field4_0x4;
    puVar5 = &iVar5.field5_0x8;
    puVar6 = &param_1.field4_0x8;
    for iVar3 in 0x40 .. 0 {
      puVar2 = puVar6;
      puVar6 = puVar6 + 1;
      puVar1 = puVar5;
      puVar5 = puVar5 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1.field2_0x2 = 0x1028;
    param_1.field257_0x108 = iVar5.field258_0x108;
    param_1.field258_0x10c = iVar5.field259_0x10c;
    param_1.field259_0x110 = iVar5.field260_0x110;
    param_1.field260_0x114 = iVar5.field261_0x114;
    *puStack10 = 0x309a;
    param_1.field2_0x2 = &u16_1050_1038;
  }
  iVar5.field261_0x114 = 0;
  iVar5.field260_0x110 = 0;
  return;
}


pub unsafe fn pass1_1038_3074(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_2a5c(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1038_4b40(mut param_1: u16 ,mut param_2: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;

  uVar7 = (param_2 >> 0x10);
  iVar6 = param_2;
  if ((iVar6 + 0xc) == 0) {
    param_1 = 0;
    uVar4 = 0;
  }
  else {
    ppcVar1 = ((iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_1);
  for uStack14 in 0 .. uStack10 {
    ppcVar1 = ((iVar6 + 0xc) + 0x4);
    uVar3 = uStack10;
    (**ppcVar1)(unaff_CS,(iVar6 + 0xc));
    uVar2 = uVar3;
    uVar5 = extraout_DX_00 | uVar2;
    if (uVar5 != 0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3 & 0xffff | extraout_DX_00 << 0x10);
      unaff_CS = 0x1030;
      struct_op_1030_73a8(CONCAT22(uVar5,uVar2),uVar2,uVar5);
    }
  }
  return;
}


pub unsafe fn pass1_1038_4c1a(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  let mut uVar8: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;

  uVar7 = (param_3 >> 0x10);
  iVar6 = param_3;
  uVar8 = (iVar6 + 0xc);
  ppcVar1 = ((iVar6 + 0xc) + 0x10);
  (**ppcVar1)();
  uStack10 = CONCAT22(param_2,param_1);
//   for (uStack14 = 0; uVar5 = param_2, uStack14 < uStack10; uStack14 += 1)
  uStack14 = 0;
  uVar5 = param_2;
  while uStack14 < uStack10
  {
    ppcVar1 = ((iVar6 + 0xc) + 0x4);
    uVar4 = uStack10;
    (**ppcVar1)(unaff_CS,(iVar6 + 0xc),uStack14,uVar8);
    uVar2 = uVar4;
    param_2 = (uVar5 | uVar2);
    if ((uVar5 | uVar2) != 0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | uVar5 << 0x10);
      uVar3 = pass1_1030_6fa0(CONCAT22(param_2,uVar2));
      unaff_CS = 0x1008;
      pass1_1008_c6ae(_u16_1050_06e0,uVar3,0xe);
    }
    uStack14 += 1;
  }
  return;
}

pub unsafe fn pass1_1038_4cba()

{
  pass1_1030_38b8();
  return;
}

pub unsafe fn pass1_1038_4cd0(mut param_1: u32,mut param_2: u32,mut param_3: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x1c) = param_3;
  (param_1 + 0x1e) = param_2;
  return;
}

pub unsafe fn pass1_1038_4cea(mut param_1: u32,param_2: *mut u32,param_3: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_3 = (param_1 + 0x1c);
  *param_2 = (param_1 + 0x1e);
  return;
}

pub unsafe fn pass1_1038_5860(mut param_1: u32,mut param_2: u16 ,mut param_3: u32,mut param_4: i16)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: i16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uStack14: u32;
  let mut iStack6: i16;
  let mut iStack4: i16;

  if (param_4 == 0) {
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    ppcVar1 = ((iVar4 + 0xc) + 0x10);
    uVar2 = param_3;
    (**ppcVar1)();
    uVar2 = uVar2 & 0xffff | extraout_DX << 0x10;
    for uStack14 in 0 .. uVar2 {
      ppcVar1 = ((iVar4 + 0xc) + 0x4);
      uVar3 = uVar2;
      (**ppcVar1)();
      iStack6 = param_3;
      if ((uVar3 == iStack6) && (iStack4 = (param_3 >> 0x10), extraout_DX_00 == iStack4)) {
        return;
      }
    }
    ppcVar1 = ((iVar4 + 0xc) + 0xc);
    (**ppcVar1)();
  }
  return;
}


pub unsafe fn pass1_1038_5e16(mut param_1: i16,mut param_2: u32,mut param_3: u32)

{
  let mut BVar1: bool;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
   let mut in_stack_0000ffb6: HFILE16;
  let mut local_14: [u32;0x2] = [0;0x2];
  let mut local_c: u32;
  let mut puStack6: *mut u32;

  pass1_1030_16d6(param_2,param_3);
  if (param_1 != 0) {
    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    puVar2 = (iVar3 + 0xc);
    puStack6 = puVar2;
    pass1_1008_7898(puVar2,param_3,puVar2);
    if (puVar2 != 0) {
      local_14[0] = (iVar3 + 0x10);
      BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_14),0x4,in_stack_0000ffb6);
      if (BVar1 != 0) {
        local_c = (iVar3 + 0x18);
        BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6);
        if (BVar1 != 0) {
          local_c = (iVar3 + 0x1a);
          BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6);
          if (BVar1 != 0) {
            local_c = CONCAT22(local_c,(iVar3 + 0x1c));
            BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6);
            if (BVar1 != 0) {
              local_c = (iVar3 + 0x1e);
              BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x4,in_stack_0000ffb6);
              if (BVar1 != 0) {
                local_c = local_c & 0xffff0000 | (iVar3 + 0x22);
                BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6);
                if (BVar1 != 0) {
                  local_c = local_c & 0xffff0000 | (iVar3 + 0x24);
                  BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6)
                  ;
                  if (BVar1 != 0) {
                    BVar1 = write_to_file_1008_7e1c
                                      (param_3,param_2 & 0xffff0000 | (iVar3 + 0x26),0x94,
                                       in_stack_0000ffb6);
                    if (BVar1 != 0) {
                      BVar1 = write_to_file_1008_7e1c
                                        (param_3,param_2 & 0xffff0000 | (iVar3 + 0x14e),0x54,
                                         in_stack_0000ffb6);
                      if (BVar1 != 0) {
                        BVar1 = write_to_file_1008_7e1c
                                          (param_3,param_2 & 0xffff0000 | (iVar3 + 0x1a2),0x54,
                                           in_stack_0000ffb6);
                        if (BVar1 != 0) {
                          write_to_file_1030_32e4((iVar3 + 0x1f6),param_3);
                          BVar1 = pass1_1008_7c2a(param_3,*(iVar3 + 0x1fa));
                          if (BVar1 != 0) {
                            local_c = local_c & 0xffff0000 | (iVar3 + 0x1fe);
                            BVar1 = write_to_file_1008_7e1c
                                              (param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6);
                            if (BVar1 != 0) {
                              local_c = (iVar3 + 0x200);
                              BVar1 = write_to_file_1008_7e1c
                                                (param_3,CONCAT22(0x1050,&local_c),0x4,in_stack_0000ffb6);
                              if (BVar1 != 0) {
                                local_c = local_c & 0xffff0000 | (iVar3 + 0x204);
                                BVar1 = write_to_file_1008_7e1c
                                                  (param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6
                                                  );
                                if (BVar1 != 0) {
                                  local_c = local_c & 0xffff0000 | (iVar3 + 0x206);
                                  BVar1 = write_to_file_1008_7e1c
                                                    (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                     in_stack_0000ffb6);
                                  if (BVar1 != 0) {
                                    local_c = local_c & 0xffff0000 | (iVar3 + 0x208);
                                    BVar1 = write_to_file_1008_7e1c
                                                      (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                       in_stack_0000ffb6);
                                    if (BVar1 != 0) {
                                      local_c = local_c & 0xffff0000 | (iVar3 + 0x20a);
                                      BVar1 = write_to_file_1008_7e1c
                                                        (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                         in_stack_0000ffb6);
                                      if (BVar1 != 0) {
                                        local_c = local_c & 0xffff0000 | (iVar3 + 0x20c);
                                        BVar1 = write_to_file_1008_7e1c
                                                          (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                           in_stack_0000ffb6);
                                        if (BVar1 != 0) {
                                          local_c = local_c & 0xffff0000 | (iVar3 + 0x20e);
                                          BVar1 = write_to_file_1008_7e1c
                                                            (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                             in_stack_0000ffb6);
                                          if (BVar1 != 0) {
                                            local_c = local_c & 0xffff0000 | (iVar3 + 0x214);
                                            BVar1 = write_to_file_1008_7e1c
                                                              (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                               in_stack_0000ffb6);
                                            if (BVar1 != 0) {
                                              local_c = (iVar3 + 0x216);
                                              BVar1 = write_to_file_1008_7e1c
                                                                (param_3,CONCAT22(0x1050,&local_c),0x4,
                                                                 in_stack_0000ffb6);
                                              if (BVar1 != 0) {
                                                return;
                                              }
                                            }
                                          }
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}


pub unsafe fn file_1038_6118(mut param_1: i16,param_2: *mut Struct57,param_3: *mut astruct_373,param_4: *mut HFILE16)

{
  let mut puVar1: *mut u32;
  let mut BVar2: bool;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u8;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut iVar9: *mut astruct_373;
  let mut uVar10: *mut astruct_373;
  let mut uVar9: u16;
  let mut uVar11: u16;
  let mut puStack1046: *mut u8;
  let mut uStack1042: u16;
  let mut local_408: [u8;0x400] = [0;0x400];
  let mut local_8: u16;
  let mut local_6: u32;

  paVar8 = CONCAT22(in_register_0000000a,param_2);
  file_1030_1730(param_3,param_4);
  if (param_1 == 0) {
    return;
  }
  local_6 = 0;
  puVar1 = &local_6;
  file_1008_7548(param_4,CONCAT22(0x1050,puVar1),paVar8);
  if (puVar1.is_null() == false) {
    uVar10 = (param_3 >> 0x10);
    iVar9 = param_3;
    iVar9.field_0xc = local_6;
    BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9.field13_0x10)),0x4);
    if (((((BVar2 != 0) &&
          (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9.field_0x18)),0x2),
          BVar2 != 0)) &&
         (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9.field19_0x1a)),0x2),
         BVar2 != 0)) &&
        ((BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_8),0x2), BVar2 != 0x0 &&
         (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0x2)),0x4),
         BVar2 != 0)))) &&
       (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                  ZEXT24((&iVar9[0x1].field4_0x4 + 0x2))),0x2),
       BVar2 != 0)) {
      (iVar9 + 1) = local_8;
      BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0x8)),0x2);
      if ((BVar2 != 0) &&
         (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0xa)),0x94),
         BVar2 != 0)) {
        if (u16_1050_0312 < 0x2) {
          uVar9 = 0x54;
          uVar11 = 0;
          mem_op_1000_179c(0x54,paVar8);
          uVar7 = SUB42(paVar8,0x0);
          puStack1046 = CONCAT22(uVar7,BVar2);
          BVar3 = read_file_1008_7dee(param_4,CONCAT22(uVar7,BVar2),CONCAT22(uVar11,uVar9));
          if (BVar3 == 0) {//
// LAB_1038_626a:
            u16_1050_0310 = 0x6d2;
            fn_ptr_1000_17ce(puStack1046);
            return;
          }
          uStack1042 = 0;
          loop {
            uVar4 = switch_1008_72bc(param_4,uStack1042);
            uVar9 = (uStack1042 * 0x4 + BVar2 + 2);
            (&iVar9[0xb].field19_0x1a)[uVar4 * 0x2] = (uStack1042 * 0x4 + BVar2);
            (&iVar9[0xc].field_0x0 + uVar4 * 0x4) = uVar9;
            uStack1042 += 0x1;
            if uStack1042 >= 0x15 {break;}
          }
          BVar3 = read_file_1008_7dee(param_4,puStack1046,0x54);
//          if (BVar3 == 0) goto LAB_1038_626a;
          uStack1042 = 0;
          loop {
            uVar5 = switch_1008_72bc(param_4,uStack1042);
            uVar4 = (uStack1042 * 0x4 + BVar2 + 2);
            (&iVar9[0xe].field19_0x1a)[uVar5 * 0x2] = (uStack1042 * 0x4 + BVar2);
            (&iVar9[0xf].field_0x0 + uVar5 * 0x4) = uVar4;
            uStack1042 += 0x1;
            if uStack1042 >= 0x15 {break;}
          }
          fn_ptr_1000_17ce(puStack1046);
        }
        else {
          BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0xb].field19_0x1a)),
                                      0x54);
          uVar4 = paVar8;
          if (BVar2 == 0) {
            u16_1050_0310 = 0x6d2;
            return;
          }
          BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0xe].field19_0x1a)),
                                      0x54);
          if (BVar2 == 0) {
            u16_1050_0310 = 0x6d2;
            return;
          }
        }
    // WARNING: Load size is inaccurate
        read_file_1030_33f0(iVar9[0x11].field19_0x1a,param_4);
        puVar6 = local_408;
        read_file_1008_7c6e(param_4,(param_4 >> 0x10),CONCAT22(0x1050,puVar6));
        if (puVar6.is_null() == false) {
          uVar5 = str_op_1008_60e8(uVar4,CONCAT22(0x1050,local_408));
          iVar9[0x12].field_0x2 = uVar5;
          iVar9[0x12].field4_0x4 = uVar4;
          BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                    ZEXT24((&iVar9[0x12].field4_0x4 + 0x2))),0x2);
          if (((((BVar2 != 0) &&
                (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                           CONCAT11((param_3 >> 0x8) + '\x02',
                                                                           param_3)),0x4), BVar2 != 0)) &&
               (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0x12].field_0xc))
                                            ,0x2), BVar2 != 0)) &&
              (((BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar9[0x12].field_0xe)
                                                           ),0x2), BVar2 != 0x0 &&
                (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                           ZEXT24(&iVar9[0x12].field13_0x10)),0x2), BVar2 != 0)) &&
               ((BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                           ZEXT24((&iVar9[0x12].field13_0x10 + 0x2))),
                                             0x2), BVar2 != 0x0 &&
                ((BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                            ZEXT24(&iVar9[0x12].field_0x14)),0x2), BVar2 != 0x0 &&
                 (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                            ZEXT24(&iVar9[0x12].field16_0x16)),0x2), BVar2 != 0)))))))
              ) && ((u16_1050_0312 < 0x2 ||
                    ((BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(iVar9 + 0x13)),0x2
                                                 ), BVar2 != 0x0 &&
                     (BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 |
                                                                ZEXT24(&iVar9[0x13].field_0x2)),0x4), BVar2 != 0))))))
          {
            return;
          }
          u16_1050_0310 = 0x6d0;
          return;
        }
      }
    }
  }
  u16_1050_0310 = 0x6d2;
  return;
}


pub unsafe fn pass1_1038_64de(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_33f8(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn pass1_1038_7dac(param_1: *mut Struct903,mut param_2: u16 ) -> LRESULT

{
  let mut LVar1: LRESULT;

  pass1_1040_78de();
  LVar1 = send_dlg_item_msg_1038_844a(param_1);
  return LVar1;
}


pub unsafe fn pass1_1038_7dc6(param_1: *mut u8,pstruct903_param_2: *mut Struct903,mut param_3: u16 ,mut param_4: u32)

{
  let mut bVar1: bool;
  let mut LVar2: LRESULT;
  let mut uVar3: u32;

  bVar1 = false;
  if (param_4 == 0x1854) {
//    if (param_4 != 1) goto LAB_1038_7e8c;
    send_dlg_item_msg_1038_8618s(param_1,pstruct903_param_2);
  }
  else {
    if (param_4 < 0x18550000) {
      if (param_4 == 0xeb) {
        LVar2 = send_dlg_item_msg_1038_844a(pstruct903_param_2);
        param_1 = (LVar2 >> 0x10);
      }
      else if (param_4 == 0xfb) {
        LVar2 = send_dlg_item_msg_1038_7eac(pstruct903_param_2);
        param_1 = (LVar2 >> 0x10);
      }
      else {
        if (param_4 != s_vrpal_bmp_1050_183a + 0x7) {//
// LAB_1038_7e77:
          pass1_1040_b54a(param_1,pstruct903_param_2,param_3,param_4);
          return;
        }
        msg_box_op_1038_81be(0x0,param_1,pstruct903_param_2);
      }
  // TODO: goto LAB_1038_7e8c;
    }
    if (param_4 == 0x1855) {
//      if (param_4 != 1) goto LAB_1038_7e8c;
      send_dlg_item_msg_1038_87b2(param_1,pstruct903_param_2);
    }
    else if (param_4 == 0x1856) {
//      if (param_4 != 1) goto LAB_1038_7e8c;
      pass1_1038_8810(pstruct903_param_2);
    }
    else if (param_4 == 0x1858) {
      send_dlg_item_msg_1038_7fae(0x0,param_1,pstruct903_param_2);
    }
    else {
//      if (param_4 != 0x1859) goto LAB_1038_7e77;
      uVar3 = pass1_1038_801a(param_1,pstruct903_param_2);
      param_1 = (uVar3 >> 0x10);
    }
  }
  bVar1 = true;//
// LAB_1038_7e8c:
  if (bVar1) {
    set_win_text_1038_8358(param_1,pstruct903_param_2);
    enable_win_1038_806a(param_1,pstruct903_param_2);
  }
  return;
}

pub unsafe fn FUN_1038_8842() -> u16

{
  return 0x0;
}
pub unsafe fn pass1_1038_8848()

{
  return;
}
pub unsafe fn pass1_1038_884c()

{
  return;
}

pub unsafe fn pass1_1038_8850(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_7d5c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn pass1_1038_89e8(mut param_1: u32)

{
  send_dlg_item_msg_1038_8b58(param_1);
  return;
}

pub unsafe fn pass1_1038_89f8(param_1: *mut Struct903,mut param_2: u16 ,mut param_3: u32,param_4: *mut u8,mut param_5: u16 )

{
  if (param_3 == 0xeb) {
    send_dlg_item_msg_1038_8b58(param_1);
  }
  else {
    if (param_3 != s_vrpal_bmp_1050_183a + 0x7) {
      pass1_1040_b54a(param_4,param_1,param_2,param_3);
      return;
    }
    msg_box_ui_op_1038_8a3a(0x0,param_4,param_1,&DAT_1050_1050);
  }
  return;
}

pub unsafe fn pass1_1038_8c08(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_893a(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn pass1_1038_8d7e(param_1: *mut Struct903) -> LRESULT

{
  let mut LVar1: LRESULT;

  pass1_1040_78de();
  LVar1 = send_dlg_item_msg_1038_8f74(param_1);
  return LVar1;
}
