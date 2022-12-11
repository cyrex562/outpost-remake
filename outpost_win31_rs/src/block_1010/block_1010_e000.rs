


pub unsafe fn pass1_1010_e128(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: i16,mut param_5: u32) -> i16

{
  let mut iStack6: i16;
  let mut iStack4: i16;

  iStack4 = 0;
  for (iStack6 = param_4; iStack6 <= param_3; iStack6 += 1) {
    if ((iStack6 * 0x2 + param_5) != 0) {
      iStack4 += 0x1;
    }
  }
  return iStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_e15e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut unaff_CS: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut puStack10: *mut u32;

  pass1_1010_afde(param_3,0xc);
  puStack10 = CONCAT22(param_2,param_1);
  ppcVar1 = (*puStack10 + 0x10);
  uVar2 = param_1;
  uVar6 = param_1;
  uVar7 = param_2;
  (**ppcVar1)();
  uStack14 = CONCAT22(extraout_DX,uVar2);
  for (uStack18 = 0; uStack18 < uStack14; uStack18 += 1) {
    ppcVar1 = (*puStack10 + 0x4);
    uVar4 = uStack14;
    (**ppcVar1)(unaff_CS,param_1,param_2,uStack18,(uStack18 >> 0x10));
    uVar3 = uVar4;
    uVar5 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | extraout_DX_00 << 0x10);
    unaff_CS = 0x1030;
    pass1_1030_7c28(uVar3,uVar5,CONCAT13((uVar5 >> 0x8),CONCAT12(uVar5,uVar3)),0x23);
  }
  if (puStack10.is_null() == false) {
    ppcVar1 = *puStack10;
    (**ppcVar1)(unaff_CS,param_1,param_2,0x1,uVar6,uVar7);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_e1f4(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut in_AX: u16;
  let mut BVar2: bool;
  let mut pcVar3: *mut c_char;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut in_buf_len_5: i16;
  let mut uVar7: u32;

  in_buf_len_5 = (param_2 >> 0x10);
  iVar6 = param_2;
  *(iVar6 + 0x13c) = 0;
  uVar7 = struct_op_1030_73a8(param_3,in_AX,param_1);
  uVar5 = (uVar7 >> 0x10);
  uVar1 = (uVar7 + 0xc);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0xc);
  if ((((((((BVar2 == 0) && (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x14), BVar2 == 0)) &&
          (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0xa), BVar2 == 0)) &&
         ((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x15), BVar2 == 0x0 &&
          (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0xb), BVar2 == 0)))) &&
        (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x16), BVar2 == 0)) &&
       (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x17), BVar2 == 0x0 &&
         (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x21), BVar2 == 0)) &&
        ((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1c), BVar2 == 0x0 &&
         (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1d), BVar2 == 0x0 &&
           (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x18), BVar2 == 0)) &&
          (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x19), BVar2 == 0)))))))) &&
      ((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x4), BVar2 == 0x0 &&
       (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x3), BVar2 == 0)))) &&
     (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1e), BVar2 == 0x0 &&
       (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x23), BVar2 == 0x0 &&
         (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1b), BVar2 == 0)) &&
        ((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1f), BVar2 == 0x0 &&
         (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,1), BVar2 == 0x0 &&
           (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x2), BVar2 == 0)) &&
          (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x13), BVar2 == 0)))))))) &&
      (((BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x20), BVar2 == 0x0 &&
        (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0xe), BVar2 == 0)) &&
       (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x10), BVar2 == 0)))))) {
    pcVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x12);
    if ((pcVar3.is_null()) && (pcVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x11), pcVar3.is_null())) {
      BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x6);
      if (BVar2 == 0) {
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x5);
        if (BVar2 == 0) {
          pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x40);
      // TODO: goto LAB_1010_e241;
        }
        uVar4 = pass1_1030_7f98(param_3);
        pcVar3 = string_op_1020_c222(uVar4);
      }
      else {
        uVar4 = pass1_1030_7f5a(param_3);
        pcVar3 = string_op_1020_c2f8(uVar4);
      }
    }
    else {
      pass1_1010_e58a(uVar5,param_2,uVar7);
    }
    unk_str_op_1000_3d3e((param_2 & 0xffff0000 | (iVar6 + 0x13c)),CONCAT22(uVar5,pcVar3));
  }
  else {//
// LAB_1010_e241:
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,(iVar6 + 0x13c),in_buf_len_5
              );
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_e58a(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut BVar2: bool;
  let mut puVar3: *mut u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut iVar7: i16;
  let mut in_buf_len_5: i16;
  let mut uVar8: u16;
  let mut puVar9: *mut u32;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffee: u16;

  paVar6 = CONCAT22(in_register_0000000a,param_1);
  in_buf_len_5 = (param_2 >> 0x10);
  iVar7 = param_2;
  *(iVar7 + 0x13c) = 0;
  uVar8 = (param_3 >> 0x10);
  puVar3 = (param_3 + 0x20);
  uVar8 = (param_3 + 0xc);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar8,0x11);
  if (BVar2 == 0) {
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar8,0x12);
    if (BVar2 == 0) {
      return;
    }
    puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,CONCAT22(in_stack_0000ffee,0x31),in_stack_0000fe96,
                             in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
    uVar4 = (puVar9 >> 0x10);
    ppcVar1 = (*puVar9 + 0x14);
    (**ppcVar1)(0x1008,puVar9,uVar4,puVar3,puVar3 >> 0xf);
    uVar5 = uVar4 | puVar3;
  }
  else {
    puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,CONCAT22(in_stack_0000ffee,0x41),in_stack_0000fe96,
                             in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
    uVar4 = (puVar9 >> 0x10);
    ppcVar1 = (*puVar9 + 0x14);
    (**ppcVar1)(0x1008,puVar9,uVar4,puVar3,puVar3 >> 0xf);
    uVar5 = uVar4 | puVar3;
  }
  if (uVar5 == 0) {
    load_string_1010_84e0
              (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,(iVar7 + 0x13c),in_buf_len_5
              );
  }
  else {
    ppcVar1 = (*puVar3 + 0x14);
    (**ppcVar1)(0x1008,puVar3,uVar4);
    unk_str_op_1000_3d3e((param_2 & 0xffff0000 | (iVar7 + 0x13c)),CONCAT22(uVar5,puVar3));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_e682(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut in_buf_len_5: u16;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut local_1e: u16;
  let mut uStack28: u16;
  let mut local_1a: u16;
  let mut uStack24: u16;
  let mut uStack22: u16;
  let mut uStack20: u32;
  let mut uStack16: u32;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut uStack6: u32;

  in_buf_len_5 = (param_3 >> 0x10);
  uVar5 = param_3;
  *(uVar5 + 0x13c) = 0;
  uStack6 = struct_op_1030_73a8(param_4,param_1,param_2);
  uVar3 = (uStack6 >> 0x10);
  uStack8 = (uStack6 + 0xc);
  uVar1 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,1);
  if (((uVar1 == 0) && (uVar1 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x13), uVar1 == 0)) &&
     (uVar1 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x2), uVar1 == 0)) {
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0xe);
    if (BVar2 != 0) {
      uVar6 = (uVar5 + 0x138);
      uVar4 = (uVar6 + 0x24);
      uStack16 = uVar4;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4);
      uStack12 = uVar4 & 0xffff | uVar3 << 0x10;
      uStack20 = (uVar4 + 0x1f6);
      uVar3 = (uStack20 + 0x1a8);
      uVar7 = 0x3947;
      uStack22 = uVar3;//
// LAB_1010_e76d:
      sys_1000_3f9c((param_3 & 0xffff0000 | (uVar5 + 0x13c)),CONCAT22(0x1050,uVar7),uVar3);
      return;
    }
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x5);
    if ((BVar2 == 0) && (BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x6), BVar2 == 0)) {
      BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x10);
      if (BVar2 == 0) {
        local_1e = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0xc);
        if ((local_1e == 0) && (local_1e = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x14), local_1e == 0)) {
          BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0xa);
          if (BVar2 == 0) {
            uVar3 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x1e);
            if (uVar3 == 0) {
              load_string_1010_84e0
                        (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,(uVar5 + 0x13c),
                         in_buf_len_5);
              return;
            }
            pass1_1030_6ddc(param_4);
            uVar7 = 0x395c;
            local_1e = uVar3;
        // TODO: goto LAB_1010_e76d;
          }
          uVar6 = pass1_1030_7c28(BVar2,uVar3,param_4,0x21);
          uStack28 = (uVar6 >> 0x10);
          uVar1 = uVar6;
          uVar7 = 0x3958;
          local_1e = uVar1;
        }
        else {
          pass1_1010_e8f6(local_1e,uVar3,uVar5,in_buf_len_5,param_4);
          uStack28 = uVar3;
          uVar6 = pass1_1030_7c28(local_1e,uVar3,CONCAT22(uVar3,local_1e),0x23);
          uStack24 = (uVar6 >> 0x10);
          uVar1 = uVar6;
          uVar7 = 0x3954;
          local_1a = uVar1;
        }
      }
      else {
        uVar6 = pass1_1030_7c28(BVar2,uVar3,param_4,0x1e);
        uStack28 = (uVar6 >> 0x10);
        uVar1 = uVar6;
        uVar7 = 0x3950;
        local_1e = uVar1;
      }
    }
    else {
      local_1e = 0;
      local_1a = 0;
      pass1_1010_e8d0(&local_1e,uVar5,in_buf_len_5,CONCAT22(0x1050,&local_1a),
                      CONCAT22(0x1050,&local_1e),param_4);
      uVar7 = 0x394a;
      uVar1 = local_1e;
    }
  }
  else {
    pass1_1010_e8f6(uVar1,uVar3,uVar5,in_buf_len_5,param_4);
    uStack12 = CONCAT22(uVar3,uVar1);
    pass1_1030_70f4(CONCAT22(uVar3,uVar1));
    uStack16 = CONCAT22(uVar3,uVar1);
    uVar7 = 0x3943;
  }
  sys_1000_3f9c((param_3 & 0xffff0000 | (uVar5 + 0x13c)),CONCAT22(0x1050,uVar7),uVar1);
  return;
}
pub unsafe fn pass1_1010_e8d0(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,param_5: *mut u16,mut param_6: u32)

{
  pass1_1030_7064(param_6);
  *param_5 = param_1;
  pass1_1030_70ac(param_6);
  *param_4 = param_1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_e8f6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut uVar6: u16;
  let mut uVar5: u32;
  let mut paVar4: *mut astruct_15;
  let mut uVar7: u32;
  let mut uVar1: u16;

  uVar5 = struct_op_1030_73a8(param_5,param_1,param_2);
  uVar1 = (uVar5 + 0xc);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x13);
  if (BVar3 == 0) {
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x14);
    if (BVar3 == 0) {
      return;
    }
    uVar7 = pass1_1028_4faa(uVar5,&DAT_1050_1050);
    uVar6 = (uVar7 >> 0x10);
    uVar2 = uVar7;
  }
  else {
    paVar4 = pass1_1028_121e(&DAT_1050_1050,uVar5);
    uVar6 = (paVar4 >> 0x10);
    uVar2 = SUB42(paVar4,0x0);
  }
  pass1_1028_b58e(CONCAT22(uVar6,uVar2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_e964(mut param_1: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe92: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffea: u16;

  puVar3 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           CONCAT22(in_stack_0000ffea,0x2f),in_stack_0000fe92,in_stack_0000ffb6,
                           in_stack_0000ffbc,in_stack_0000ffc0);
  uVar2 = (puVar3 >> 0x10);
  uVar1 = (puVar3 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  pass1_1038_4d28((uVar1 & 0xffff | uVar2 << 0x10));
  return;
}



pub unsafe fn pass1_1010_e99a(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xa));
  pass1_1010_a478(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn FUN_1010_e9a6(mut param_1: u16 ,param_2: *mut StructD,param_3: u8) -> *mut StructD

{
  pass1_1010_a478(param_2);
  if ((param_3 & 1) != 0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}
pub unsafe fn struct_1010_e9e4(param_1: *mut astruct_19,mut param_2: u16 )

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut in_EDX: u32;
  let mut uVar8: u16;
  let mut paVar7: *mut Struct57;
  let mut iVar9: i16;
  let mut paVar10: *mut astruct_19;
  let mut puVar11: *mut u16;
  let mut iStack4: i16;

  uVar8 = (in_EDX >> 0x10);
  paVar10 = struct_op_1010_1d48(param_1,param_2);
  paVar7 = CONCAT22(uVar8,(paVar10 >> 0x10));
  (param_1 + 0xa) = 0x389a;
  (param_1 + 0xc) = 0x1008;
  (param_1 + 0xa) = 0x3aa8;
  (param_1 + 0xc) = 0x1008;
  uVar5 = 0;
  (param_1 + 0xe) = 0;
  (param_1 + 0x12) = 0;
  (param_1 + 0x16) = 0;
  (param_1 + 0x1a) = 0;
  (param_1 + 0x1e) = 0;
  (param_1 + 0x20) = 0;
  (param_1 + 0x24) = 0;
  (param_1 + 0x28) = 0;
  (param_1 + 0x2c) = 0;
  (param_1 + 0x30) = 0;
  (param_1 + 0x32) = 0;
  param_1.offset_0x0 = 0x558;
  (param_1 + 0x2) = 0x1018;
  (param_1 + 0xa) = 0x568;
  (param_1 + 0xc) = 0x1018;
  mem_op_1000_179c(0x4,paVar7);
  if ((paVar7 | uVar5) == 0) {
    (param_1 + 0xe) = 0;
  }
  else {
    puVar11 = pass1_1018_dcf6(CONCAT22(paVar7,uVar5));
    (param_1 + 0xe) = puVar11;
    (param_1 + 0x10) = (puVar11 >> 0x10);
  }
  pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x34)),NULL,0x24);
  (param_1 + 0x38) = 0xfa;
  (param_1 + 0x3c) = 0x15e;
  uVar6 = 0x1c2;
  (param_1 + 0x40) = 0x1c2;
  (param_1 + 0x44) = 0x1c2;
  (param_1 + 0x46) = 0x2260000;
  (param_1 + 0x4a) = 0x28a0000;
  (param_1 + 0x4e) = 0x730000;
  (param_1 + 0x52) = 0x960000;
  (param_1 + 0x56) = 0;
  for (iStack4 = 0x1; iStack4 < 0x9; iStack4 += 1) {
    pass1_1008_612e(uVar6,0x0,0x1d);
    uVar5 = uVar6;
    pass1_1008_612e(uVar5,0x1,0x2);
    if ((uVar6 & 1) != 0) {
      uVar5 = -uVar5;
    }
    iVar9 = iStack4 * 0x4;
    puVar1 = (param_1 + 0x34 + iVar9);
    uVar2 = *puVar1;
    uVar4 = uVar5 + *puVar1;
    uVar6 = uVar4;
    iVar3 = (param_1 + 0x34 + iVar9 + 2);
    (param_1 + iVar9 + 0x34) = uVar4;
    (param_1 + iVar9 + 0x36) = (uVar5 >> 0xf) + iVar3 + CARRY2(uVar5,uVar2);
  }
  return;
}
pub unsafe fn pass1_1010_eb66(param_1: *mut StructD)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut puVar4: *mut u16;
  let mut iVar5: *mut StructD;
  let mut uVar5: u16;
  let mut puStack14: *mut u16;

  uVar5 = (param_1 >> 0x10);
  iVar5 = param_1;
  param_1.address_offset_field_0x0 = 0x558;
  iVar5.address_offset_field_0x2 = 0x1018;
  iVar5.field6_0xa = 0x568;
  iVar5.field7_0xc = 0x1018;
  puVar1 = iVar5.field8_0xe;
  uVar2 = &iVar5.field_0x10;
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  pass1_1018_04f2(param_1);
  fn_ptr_1000_17ce(iVar5.field29_0x2c);
  if (param_1.is_null()) {
    puVar4 = NULL;
    uVar5 = 0;
  }
  else {
    puVar4 = &iVar5.field6_0xa;
  }
  puStack14 = CONCAT22(uVar5,puVar4);
  *puStack14 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}
pub unsafe fn pass1_1010_ebf8(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + param_4 * 0x4 + 0x34) = param_2;
  (param_1 + param_4 * 0x4 + 0x36) = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ec18(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32) -> u32

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  return CONCAT22((param_1 + 0x12),(param_1 + 0x10));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ec40(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32) -> u32

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  return CONCAT22((param_1 + 0x12),(param_1 + 0x10));
}
pub unsafe fn pass1_1010_ec68(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x28) = param_2;
  pass1_1010_1f62((param_1 & 0xffff | uVar1 << 0x10),0x13);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_ec84(mut param_1: u32)

{
  let mut local_10e: [u8;0x10c] = [0;0x10c];

  pass1_1010_1f62(param_1,0x14);
  pass1_1030_532e(CONCAT22(0x1050,local_10e),(param_1 + 0x20));
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_10e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_ecc6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,param_5: i32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_4,param_5);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
  uVar1 = (param_1 + 0x2e);
  uVar3 = (uVar1 >> 0x10);
  iVar2 = uVar1;
  if ((iVar2 + 0x200) == 0x8000001) {
    pass1_1010_ed22(param_3,(iVar2 + 0x4));
  }
  return;
}
pub unsafe fn pass1_1010_ed22(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x24) = param_2;
  pass1_1010_1f62((param_1 & 0xffff | uVar1 << 0x10),0x12);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_ed3e(mut param_1: u32)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(param_1 + 0x16));
  return;
}
pub unsafe fn write_to_file_1010_ed58(mut param_1: u32,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut iVar4: i16;
  let mut puVar5: *mut u32;
  let mut iVar6: i16;
  let mut uVar7: u16;
  in_stack_0000ffc4: HFILE16;
  let mut local_22: u32;
  let mut uStack30: u16;
  let mut local_12: [u32;0x2] = [0;0x2];
  let mut local_a: u32;
  let mut iStack4: i16;

  BVar3 = write_to_file_1008_7cac(param_2);
  if (BVar3 != 0) {
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    local_12[0] = (iVar6 + 0x16);
    BVar3 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_12),0x4,in_stack_0000ffc4);
    if (BVar3 != 0) {
      local_a = (iVar6 + 0x1a);
      BVar3 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_a),0x4,in_stack_0000ffc4);
      if (BVar3 != 0) {
        local_a = (iVar6 + 0x20);
        BVar3 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_a),0x4,in_stack_0000ffc4);
        if (BVar3 != 0) {
          local_a = (iVar6 + 0x24);
          BVar3 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_a),0x4,in_stack_0000ffc4);
          if (BVar3 != 0) {
            local_a = local_a & 0xffff0000 | (iVar6 + 0x30);
            BVar3 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_a),0x2,in_stack_0000ffc4);
            if (BVar3 != 0) {
              local_a = local_a & 0xffff0000 | (iVar6 + 0x32);
              BVar3 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_a),0x2,in_stack_0000ffc4);
              if (BVar3 != 0) {
                iStack4 = 0;
                loop {
                  piVar1 = (iVar6 + 0x30);
                  if (*piVar1 == iStack4 || *piVar1 < iStack4) {
                    return;
                  }
                  uVar2 = (iVar6 + 0x2e);
                  puVar5 = ((iVar6 + 0x2c) + iStack4 * 0x6);
                  local_22 = *puVar5;
                  uStack30 = (puVar5 + 1);
                  local_12[0] = local_12[0] & 0xffff0000 | ZEXT24(&local_22);
                  iVar4 = write_to_file_1008_7b4c(param_2,CONCAT22(0x1050,&local_22));
                  if (iVar4 == 0) break;
                  iStack4 += 0x1;
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
