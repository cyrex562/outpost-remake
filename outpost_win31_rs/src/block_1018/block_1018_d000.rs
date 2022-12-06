
astruct_29 * pass1_1018_d198(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d1be(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d1e4(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d20a(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d230(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d256(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d27c(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d2a2(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d2c8(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d2ee(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d314(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d33a(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d360(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d386(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d3ac(param_1: *mut astruct_29,param_2: u8)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1018_dcf6(param_1: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  *param_1 = 0xdf3c;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_dd1e(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16,mut param_6: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;

  pass1_1010_81f6(_u16_1050_14cc,0x0,param_6);
  uVar2 = SUB42(param_2,0x0);
  uVar1 = param_1;
  mem_op_1000_179c(0x46,param_2);
  uVar3 = param_2 | uVar1;
  uVar4 = param_2 & 0xffff0000 | uVar3;
  if (uVar3 == 0) {
    uVar1 = 0;
    uVar3 = 0;
  }
  else {
    pass1_1008_87cc((astruct_86 *)CONCAT22(param_2,uVar1),param_5,param_6,param_6,
                    (astruct_76 *)CONCAT22(uVar2,param_1),0x0,uVar4);
    uVar3 = uVar4;
  }
  pass1_1008_8bc6(uVar3,CONCAT22(uVar3,uVar1));
  return CONCAT22(uVar3,uVar1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_dd7c(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: u32,mut param_6: u16 ,
                    mut param_7: u16 ,mut param_8: u16 ,mut param_9: u16 ,mut param_10: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  code **ppcVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut paVar11: *mut Struct57;
  let mut uVar12: u32;
  let mut puVar13: *mut u32;
  let mut puVar14: *mut u32;
  let mut iVar15: i16;
  let mut lStack32: i32;
  let mut uStack20: u16;
  let mut uStack12: u16;

  uVar4 = param_5._3_1_;
  iVar15 = (param_4 >> 0x10);
  if (param_5._3_1_ == 0) {
    puVar13 = mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_6,0x2f),param_10,param_9
                              ,param_7,param_8);
    paVar11 = (astruct_57 *)(param_1 & 0xffff0000 | puVar13 >> 0x10);
    if ((puVar13 + 0x1e) == 0) {
      uStack20 = param_5;
      uVar4 = param_5;
    }
    else {
      if (param_5 - 0x7 == 0) {
        uStack20 = 0x6;
        param_5 = param_5 - 0x7;
      }
      else if (param_5 - 0x8 == 0) {
        uStack20 = 0x7;
        param_5 = param_5 - 0x8;
      }
      else {
        uStack20 = 0x8;
        param_5 = param_5 - 0x9;
      }
      uVar4 = 0x6;
    }
    pass1_1010_81f6(_u16_1050_14cc,0x0,uVar4);
    uVar8 = paVar11;
    uVar4 = uVar8 | param_5;
    if (uVar4 != 0) {
      mem_op_1000_179c(0x46,paVar11);
      uVar9 = paVar11 | uVar4;
      if (uVar9 != 0) {
        pass1_1008_87cc((astruct_86 *)CONCAT22(paVar11,uVar4),param_4,iVar15,uStack20,
                        (astruct_76 *)CONCAT22(uVar8,param_5),param_5,paVar11 & 0xffff0000 | uVar9);
      }
    }
  }
  else {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
    uVar10 = param_1;
    puVar14 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar10,uVar4),uVar4,uVar10);
    uVar8 = puVar14;
    uVar9 = (puVar14 >> 0x10);
    if ((uVar9 | uVar8) != 0) {
      uVar2 = (uVar4 + 0x2e);
      uStack12 = uVar2;
      if (((uVar4 + 0x30) | uStack12) == 0) {
        lStack32 = 0;
      }
      else {
        lStack32 = *(i32 *)(uStack12 + 0x200);
      }
      uVar4 = (uVar8 + 0x1c);
      uVar1 = (uVar8 + 0x1e);
      paVar11 = (astruct_57 *)(param_1 & 0xffff0000 | uVar1);
      uVar5 = uVar1 | uVar4;
      if ((uVar1 | uVar4) != 0) {
        lStack32 = CONCAT22(uVar1,uVar4);
        uVar5 = uVar4;
      }
      ppcVar3 = (code **)(*puVar14 + 0x14);
      (**ppcVar3)(0x1030,uVar8,uVar9);
      uVar6 = uVar5;
      pass1_1010_81f6(_u16_1050_14cc,lStack32,uVar5);
      uVar10 = SUB42(paVar11,0x0);
      uVar7 = uVar6;
      mem_op_1000_179c(0x46,paVar11);
      uVar4 = paVar11 | uVar7;
      uVar12 = paVar11 & 0xffff0000 | uVar4;
      if (uVar4 == 0) {
        uVar7 = 0;
        uVar4 = 0;
      }
      else {
        pass1_1008_87cc((astruct_86 *)CONCAT22(paVar11,uVar7),param_4,iVar15,uVar5,
                        (astruct_76 *)CONCAT22(uVar10,uVar6),param_5,uVar12);
        uVar4 = uVar12;
      }
      pass1_1008_8bc6(uVar4,CONCAT22(uVar4,uVar7));
    }
  }
  return;
}



StructD * pass1_1018_df10(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn win_1018_df40(mut param_1: u16 ,param_2: *mut u8,param_3: *mut StructA)

{
  let mut puVar2: *mut u8;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let struct_1: *mut StructA;
  let mut struct_1_hi: u16;
  let mut puVar1: *mut u16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  create_window_ex_1008_9760(param_3);
  mem_op_1000_179c(0xa,paVar3);
  puVar2 = (paVar3 | param_1);
  struct_1 = (StructA *)param_3;
  struct_1_hi = (param_3 >> 0x10);
  if (puVar2 != NULL) {
    puVar1 = struct_1018_e100(puVar2,CONCAT22(paVar3,param_1),struct_1->field4_0x8);
    struct_1[0x1].field11_0x16 = puVar1;
    struct_1[0x1].field12_0x18 = (puVar1 >> 0x10);
    return;
  }
  &struct_1[0x1].field11_0x16 = 0;
  return;
}
pub fn pass1_1018_df92(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  destroy_win_1008_628e(param_1);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0xe2);
  uVar2 = (iVar4 + 0xe4);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,1);
  }
  (iVar4 + 0xe2) = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_dfd4(param_1: *mut u8,mut param_2: u32)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000fff4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar3 = (param_2 >> 0x10);
  uVar2 = param_2;
  delete_palette_1018_e16c(*(astruct_795 **)(uVar2 + 0xe2));
  if ((uVar2 + 0xe6) == 0) {
    (uVar2 + 0xe6) = 0x1;
    puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff4,0x36),in_stack_0000fe9c,
                             in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
    pass1_1038_af40(uVar2,(puVar4 >> 0x10),_PTR_LOOP_1050_5b7c,(uVar2 + 0x8),0x8);
  }
  return;
}
