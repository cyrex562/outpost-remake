pub unsafe fn struct_1028_0068(param_1: *mut astruct_57,param_2: *mut astruct_180)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut iVar2: *mut astruct_180;
  let mut uVar3: u16;

  struct_1028_b354(param_2);
  uVar3 = (param_2 >> 0x10);
  iVar2 = param_2;
  uVar1 = 0;
  (iVar2 + 1).field0_0x0 = 0;
  iVar2[0x1].field1_0x2 = 0;
  param_2.field0_0x0 = 0x8ec;
  iVar2.field1_0x2 = 0x1028;
  mem_op_1000_179c(0xc,param_1);
  uVar2 = param_1 | uVar1;
  if (uVar2 == 0) {
    iVar2[0x1].field1_0x2 = 0;
  }
  else {
    set_struct_1008_574a(CONCAT22(param_1,uVar1));
    iVar2[0x1].field1_0x2 = uVar1;
    iVar2[0x1].field_0x4 = uVar2;
  }
  return;
}
pub unsafe fn pass1_1028_00cc(param_1: *mut StructD,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;

  paVar3 = CONCAT22(in_register_0000000a,param_1);
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  uVar1 = 0;
  (param_2 + 0x20) = 0;
  (param_2 + 0x22) = 0;
  param_2.field0_0x0 = 0x8ec;
  (param_2 + 0x2) = 0x1028;
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | uVar1;
  if (uVar2 == 0) {
    (param_2 + 0x22) = 0;
  }
  else {
    set_struct_1008_574a(CONCAT22(paVar3,uVar1));
    (param_2 + 0x22) = uVar1;
    (param_2 + 0x24) = uVar2;
  }
  return;
}
pub unsafe fn pass1_1028_0138(param_1: *mut u16)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x8ec;
  (iVar4 + 0x2) = 0x1028;
  puVar1 = (iVar4 + 0x22);
  uVar2 = (iVar4 + 0x24);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  pass1_1028_b418(param_1);
  return;
}
pub unsafe fn pass1_1028_0176(mut param_1: u16 ,param_2: *mut astruct_15,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_EDX: u32;
  let mut uVar7: u16;
  let mut iVar9: *mut astruct_15;
  let mut uVar8: *mut astruct_15;
  let mut iVar8: *mut astruct_298;
  let mut paVar6: *mut Struct57;

  uVar3 = (in_EDX >> 0x10);
  iVar9 = param_2;
  uVar8 = (param_2 >> 0x10);
  pass1_1028_b46e(param_1,param_2,param_3);
  puVar2 = iVar9.field25_0x22;
  uVar5 = (iVar9 + 1).field0_0x0;
  uVar4 = uVar5 | puVar2;
  paVar6 = CONCAT22(uVar3,uVar4);
  if (uVar4 != 0) {
    ppcVar1 = *puVar2;
    puVar2 = (**ppcVar1)();
  }
  mem_op_1000_179c(0xc,paVar6);
  uVar5 = paVar6 | puVar2;
  if (uVar5 == 0) {
    uVar4 = 0;
    uVar5 = 0;
  }
  else {
    uVar4 = set_struct_1008_574a(CONCAT22(paVar6,puVar2));
  }
  iVar9.field25_0x22 = uVar4;
  (iVar9 + 1).field0_0x0 = uVar5;
  uVar7 = 0x14;
  uVar3 = pass1_1028_b58e(param_2);
  pass1_1030_7f1a(CONCAT22(uVar5,uVar3),uVar7);
  return;
}
pub unsafe fn pass1_1028_01ec(param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (((iVar2 + 0x12) == 0x6) || ((iVar2 + 0x12) == 0x5)) {
    uVar1 = (iVar2 + 0x14);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    if (((iVar2 + 0xa6) == 0x14) || ((iVar2 + 0xa8) == 0x10)) {
      pass1_1028_bdac(param_1,0x6);
      return;
    }
    pass1_1028_be2a(param_1);
  }
  return;
}



pub unsafe fn write_to_file_1028_0234(mut param_1: u32,mut param_2: u32) -> u16

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut iVar3: i16;
  let mut uVar4: u16;
  in_stack_0000ffba: HFILE16;
  let mut local_1a: [u16;0x3] = [0;0x3];
  let mut local_14: [u16;0x2] = [0;0x2];
  let mut uStack16: u16;
  let mut lStack14: i32;
  let mut local_a: [u8;0x8] = [0;0x8];

  BVar2 = write_to_file_1028_b5ec(param_1,param_2);
  if (BVar2 != 0) {
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    local_1a[0] = (iVar3 + 0x20);
    BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_1a),0x2,in_stack_0000ffba);
    if (BVar2 != 0) {
      pass1_1008_5784(CONCAT22(0x1050,local_a),(iVar3 + 0x22));
      uVar1 = (iVar3 + 0x22);
      local_14[0] = (uVar1 + 0x8);
      uStack16 = local_14[0];
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_14),0x2,in_stack_0000ffba);
      while (BVar2 != 0) {
        lStack14 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
        if (lStack14 == 0) {
          return 0x1;
        }
        local_14[0] = (lStack14 + 0x4);
        BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_14),0x2,in_stack_0000ffba);
        if (BVar2 == 0) { break; }
        local_14[0] = (lStack14 + 0x6);
        BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_14),0x2,in_stack_0000ffba);
        if (BVar2 == 0) { break; }
        local_14[0] = (lStack14 + 0x8);
        BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_14),0x2,in_stack_0000ffba);
        if (BVar2 == 0) { break; }
        local_14[0] = (lStack14 + 0xa);
        BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_14),0x2,in_stack_0000ffba);
        if (BVar2 == 0) { break; }
        local_14[0] = (lStack14 + 0xc);
        BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_14),0x2,in_stack_0000ffba);
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_0374(mut param_1: i16,param_2: *mut u8,param_3: *mut astruct_373,param_4: *mut HFILE16)

{
  let mut ppcVar1: *mut *mut code;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut local_18: [u16;0x2] = [0;0x2];
  let mut paStack20: *mut astruct_99;
  let mut local_10: [u16;0x2] = [0;0x2];
  let mut local_c: [u16;0x3] = [0;0x3];
  let mut uStack6: u16;
  let mut local_4: u16;
  let mut uVar2: *mut astruct_728;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0) {
    BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | (param_3 + 0x20)),0x2);
    if (BVar2 != 0) {
      BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2);
      if (BVar2 != 0) {
        uStack6 = 0;
        loop {
          if (local_4 <= uStack6) {
            return;
          }
          paStack20 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
          uVar4 = (paStack20 >> 0x10);
          uVar2 = paStack20;
          if ((uVar4 | uVar2) == 0) {
            paStack20 = NULL;
          }
          else {
            paStack20.field0_0x0 = 0x389a;
            uVar2.field2_0x2 = 0x1008;
            uVar2.field3_0x4 = 0;
            uVar2.field4_0x6 = 0;
            uVar2.field5_0x8 = 0;
            uVar2.field6_0xa = 0;
            uVar2.field7_0xc = 0;
            paStack20.field0_0x0 = 0x56ce;
            uVar2.field2_0x2 = 0x1018;
          }
          BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_10),0x2);
          if (BVar2 == 0) { break; }
          BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_c),0x2);
          if (BVar2 == 0) { break; }
          BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_18),0x2);
          if (BVar2 == 0) { break; }
          BVar2 = read_file_1008_7dee(param_4,(paStack20 & 0xffff0000 | (paStack20 + 0xa)),0x2)
          ;
          if (BVar2 == 0) { break; }
          BVar2 = read_file_1008_7dee(param_4,(paStack20 & 0xffff0000 | (paStack20 + 0xc)),0x2)
          ;
          if (BVar2 == 0) { break; }
          (paStack20 + 0x4) = local_10[0];
          uVar3 = switch_1008_72bc(param_4,local_c[0]);
          uVar5 = (paStack20 >> 0x10);
          (paStack20 + 0x6) = uVar3;
          (paStack20 + 0x8) = local_18[0];
          ppcVar1 = ((param_3 + 0x22) + 0x8);
          (**ppcVar1)();
          uStack6 += 0x1;
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



pub unsafe fn pass1_1028_04ee(mut param_1: u32,param_2: *mut u32) -> u16

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut lVar5: i32;
  let mut local_a: [u8;0x8] = [0;0x8];

  *param_2 = 0;
  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x22));
  loop {
    lVar5 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (lVar5 == 0) {
      return 0x0;
    }
    uVar2 = (lVar5 + 0xc);
    uVar4 = (param_2 >> 0x10);
    uVar3 = param_2;
    param_2 = param_2 + uVar2;
    piVar1 = (param_2 + 2);
    *piVar1 = *piVar1 + CARRY2(uVar3,uVar2);
  } while (((param_2 + 0x2) == 0) && (param_2 < 0x1e));
  return 0x1;
}
pub unsafe fn pass1_1028_0550(param_1: *mut astruct_15)

{
  let mut in_EDX: *mut Struct57;
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;

  pass1_1028_be9e(param_1);
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x12) == 0x5) {
    uVar3 = 0;
    iVar4 = 0x4;
    uVar2 = 0x1;
    uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar1 << 0x10));
    pass1_1030_7c50(uVar1,in_EDX,CONCAT22(in_EDX,uVar1),CONCAT22(uVar3,uVar2),iVar4);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_0582(param_1: u32,mut param_2: u16 ,param_3: *mut astruct_15)

{
  u32 **ppuVar1;
  plVar2: *mut i32;
  let mut uVar3: u32;
  let mut ppcVar4: *mut *mut code;
  let mut puVar5: *mut u8;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut extraout_DX: u16;
  let mut uVar9: u16;
  let mut extraout_DX_00: u16;
  let mut extraout_DX_01: u16;
  let mut iVar10: i16;
  let mut iVar11: i16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut unaff_CS: u16;
  let mut local_138: [u8;0x10e] = [0;0x10e];
  let mut local_2a: u32;
  let mut paStack38: *mut astruct_99;
  let mut paStack34: *mut astruct_99;
  let mut uStack30: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut local_a: [u8;0x4] = [0;0x4];
  let mut uStack6: u32;

  uVar12 = (param_3 >> 0x10);
  iVar10 = param_3;
  uVar8 = (iVar10 + 0x14);
  uVar13 = (uVar8 >> 0x10);
  iVar11 = uVar8;
  uStack6 = uVar8 & 0xffff0000 | (iVar11 + 0xa4);
  if (((iVar11 + 0xa6) != 0) && ((iVar11 + 0xac) != 0)) {
    pass1_1028_081e(param_1,param_2,param_3);
    param_1 = (iVar10 + 0x20);
    ppuVar1 = (u32 **)(uStack6 + 0x8);
    if (*ppuVar1 < param_1 || *ppuVar1 == param_1) {
      puVar5 = local_a;
      ppcVar4 = (param_3 + 0x40);
      (**ppcVar4)();
      uVar8 = ZEXT24(puVar5);
      param_2 = extraout_DX;
      if (puVar5.is_null()) {
        if ((uStack6 + 0x2) == 0xc) {
          uStack14 = pass1_1028_b4f2(param_3);
          param_2 = (uStack14 >> 0x10);
          uVar8 = (uStack14 + 0x1f6);
          plVar2 = (uVar8 + 0x170);
          *plVar2 = *plVar2 + 1;
          uStack18 = uVar8;
        }
        else {
          uStack18 = _PTR_LOOP_1050_68a2;
          paStack38 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
          uVar9 = (paStack38 >> 0x10);
          uVar6 = paStack38;
          if ((uVar9 | uVar6) == 0) {
            paStack34 = NULL;
          }
          else {
            paStack38.field0_0x0 = 0x389a;
            (uVar6 + 0x2) = 0x1008;
            (uVar6 + 0x4) = 0;
            (uVar6 + 0x6) = 0;
            (uVar6 + 0x8) = 0;
            (uVar6 + 0xa) = 0;
            (uVar6 + 0xc) = 0;
            paStack38.field0_0x0 = 0x56ce;
            (uVar6 + 0x2) = 0x1018;
            paStack34 = paStack38;
          }
          uVar13 = (uStack6 >> 0x10);
          iVar11 = uStack6;
          uVar14 = (paStack34 >> 0x10);
          (paStack34 + 0x6) = (iVar11 + 2);
          (paStack34 + 0xa) = (iVar11 + 0x6);
          unaff_CS = 0x1020;
          uVar7 = switch_1020_c3b4((iVar11 + 0x2));
          uVar6 = uVar7 * (uStack6 + 0x6);
          uVar8 = uVar6;
          (paStack34 + 0xc) = uVar6;
          uVar3 = (iVar10 + 0x22);
          ppcVar4 = ((iVar10 + 0x22) + 0x4);
          (**ppcVar4)(0x1020,uVar3,(uVar3 >> 0x10));
          param_2 = extraout_DX_00;
        }
      }
      param_1 = uVar8;
      (iVar10 + 0x20) = 0;
    }
  }
  uVar13 = (uStack6 >> 0x10);
  if (((uStack6 + 0x4) != 0) && ((uStack6 + 0x8) != 0)) {
    pass1_1028_081e(param_1,param_2,param_3);
    param_1 = (iVar10 + 0x20);
    ppuVar1 = (u32 **)(uStack6 + 0x8);
    if (*ppuVar1 < param_1 || *ppuVar1 == param_1) {
      param_1 = &local_2a;
      ppcVar4 = (param_3 + 0x40);
      (**ppcVar4)(unaff_CS,param_3);
      if (param_1.is_null()) {
        uStack18 = _PTR_LOOP_1050_68a2;
        paStack38 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar9 = (paStack38 >> 0x10);
        uVar6 = paStack38;
        if ((uVar9 | uVar6) == 0) {
          paStack34 = NULL;
        }
        else {
          paStack38.field0_0x0 = 0x389a;
          (uVar6 + 0x2) = 0x1008;
          (uVar6 + 0x4) = 0;
          (uVar6 + 0x6) = 0;
          (uVar6 + 0x8) = 0;
          (uVar6 + 0xa) = 0;
          (uVar6 + 0xc) = 0;
          paStack38.field0_0x0 = 0x56ce;
          (uVar6 + 0x2) = 0x1018;
          paStack34 = paStack38;
        }
        uVar13 = (uStack6 >> 0x10);
        iVar11 = uStack6;
        uVar14 = (paStack34 >> 0x10);
        (paStack34 + 0x8) = (iVar11 + 0x4);
        (paStack34 + 0xa) = (iVar11 + 0x6);
        uVar7 = pass1_1020_c42e((iVar11 + 0x4));
        param_1 = (uVar7 * (uStack6 + 0x6));
        (paStack34 + 0xc) = param_1;
        uVar3 = (iVar10 + 0x22);
        ppcVar4 = ((iVar10 + 0x22) + 0x4);
        (**ppcVar4)(0x1020,uVar3,(uVar3 >> 0x10));
      }
      (iVar10 + 0x20) = 0;
    }
  }
  if ((iVar10 + 0xc) != 0xe) {
    pass1_1028_b58e((param_3 & 0xffff | uVar12 << 0x10));
    local_2a = CONCAT22(extraout_DX_01,param_1);
    paStack34 = (param_1 + 0x2e);
    uStack30 = (paStack34 + 0x4);
    pass1_1028_68de(CONCAT22(0x1050,local_138),0x1,uStack30);
    fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_138));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_081e(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;

  pass1_1028_b58e(param_3);
  uVar4 = (param_1 + 0x2e);
  iVar2 = (uVar4 + 0x18);
  uVar7 = (param_3 >> 0x10);
  iVar6 = param_3;
  piVar1 = (iVar6 + 0x20);
  *piVar1 = *piVar1 + 1;
  uVar5 = *_PTR_LOOP_1050_65e2;
  uVar3 = (_PTR_LOOP_1050_65e2 + 2);
  if (iVar2 < 0xfa) {
    uVar5 &= 0x1;
  }
  else {
    if (0x1c1 < iVar2) {
      if (iVar2 < 0x226) {
        return;
      }
      if ((iVar2 < 0x2ee) && (CONCAT22(uVar3,uVar5) % 0x3 != 0)) {
        return;
      }
      piVar1 = (iVar6 + 0x20);
      *piVar1 = *piVar1 + 1;
      return;
    }
    uVar5 = (CONCAT22(uVar3,uVar5) % 0x3);
  }
  if (uVar5 != 0) {
    return;
  }
  piVar1 = (iVar6 + 0x20);
  *piVar1 = *piVar1 + -0x1;
  return;
}



pub unsafe fn pass1_1028_08c6(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_0138(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_180 * struct_1028_0954(param_1: *mut astruct_180)

{
  let mut iVar1: *mut astruct_180;
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 1).field0_0x0 = 0;
  param_1.field0_0x0 = 0xada;
  iVar1.field1_0x2 = 0x1028;
  iVar1.field11_0xe = 0x4b;
  return param_1;
}



pub unsafe fn pass1_1028_0982(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> *mut u16

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  (param_2 + 0x20) = 0;
  param_2.field0_0x0 = 0xada;
  (param_2 + 0x2) = 0x1028;
  (param_2 + 0xe) = 0x4b;
  return &param_2.field0_0x0;
}
pub unsafe fn pass1_1028_09b8(param_1: *mut astruct_15)

{
  let mut uVar1: u32;

  uVar1 = pass1_1028_b58e(param_1);
  (uVar1 + 0x14) = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_09d4(mut param_1: i16,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,mut param_5: u32,param_6: i32)

{
  let mut iVar1: i16;
  let mut puVar2: *mut u8;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut local_6: [u8;0x2] = [0;0x2];
  let mut BStack4: bool;

  uVar6 = param_3;
  uVar7 = (param_3 >> 0x10);
  BStack4 = pass1_1028_c314(param_1,param_2,uVar6,uVar7,param_4,param_5,(param_5 >> 0x10),param_6);
  if (BStack4 == 0) {
    return;
  }
  pass1_1028_c7b6(param_2,uVar6,uVar7,param_4,param_6);
  if ((BStack4 != 0) && (BStack4 != 0x4)) {
    if (((BStack4 - 0x5) < 1) || ((SBORROW2(BStack4 - 0x5,1) || (0x3 < (BStack4 - 0x6))))) {
      if (((uVar6 + 0xc) != 0x3e) && ((uVar6 + 0xc) != 0x41)) {
        return;
      }
      uVar5 = pass1_1030_bcae(local_6,&DAT_1050_1050);
      uVar4 = (uVar5 >> 0x10);
      iVar1 = uVar5;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
      uVar3 = (iVar1 + 0x10);
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
      puVar2 = local_6;
      pass1_1030_bcde(puVar2,&DAT_1050_1050,uVar3 & 0xffff | uVar4 << 0x10,param_4,param_6);
      if (puVar2 < 0x0) {
        PTR_LOOP_1050_50ca = 0x6af;
        return;
      }
      if (0x5 < puVar2) {
        PTR_LOOP_1050_50ca = 0x6b5;
        return;
      }
      return;
    }
  }
  PTR_LOOP_1050_50ca = 0x6a8;
  return;
}



pub unsafe fn pass1_1028_0ab4(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn struct_1028_0b42(param_1: *mut astruct_180) -> *mut u16

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0xbbc;
  (param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



pub unsafe fn pass1_1028_0b64(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> *mut u16

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0xbbc;
  (param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub unsafe fn FUN_1028_0b8e()

{
  return;
}
pub unsafe fn FUN_1028_0b92()

{
  return;
}



pub unsafe fn pass1_1028_0b96(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn struct_1028_0c24(param_1: *mut astruct_180) -> *mut u16

{
  let mut iVar1: *mut astruct_180;
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 1).field0_0x0 = 0;
  iVar1[0x1].field1_0x2 = 0;
  param_1.field0_0x0 = s_480_bmp_1050_1721 + 0x3;
  iVar1.field1_0x2 = 0x1028;
  return &param_1.field0_0x0;
}



pub unsafe fn pass1_1028_0c50(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> *mut u16

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  (param_2 + 0x20) = 0;
  (param_2 + 0x22) = 0;
  param_2.field0_0x0 = s_480_bmp_1050_1721 + 0x3;
  (param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_0c84(mut param_1: i16,param_2: *mut astruct_15,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  let mut bStack55: u8;
  let mut local_32: [u8;0xa] = [0;0xa];
  let mut uStack40: u32;
  let mut uStack36: u32;
  let mut puStack28: *mut u32;
  let mut local_1a: u32;
  let mut iStack22: i16;
  let mut uStack20: u16;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut paStack6: *mut astruct_292;

  pass1_1028_b58e(param_2);
  paStack6 = CONCAT22(extraout_DX,param_1);
  local_c = (param_1 + 0xc);
  iStack18 = (param_1 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_DX;
  iStack14 = iStack18;
  iStack8 = iStack18;
  pass1_1028_bab6(iStack18,extraout_DX,param_2);
  uVar2 = pass1_1030_2fac(CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  iStack22 = iStack8;
  uStack36 = CONCAT22(uStack36,&local_1a);
  iStack14 += 0x1;
  uStack20 = uVar2;
  if (iStack14 <= uVar2) {
    puVar7 = CONCAT22(0x1050,local_32);
    iStack22 = iStack14;
    uVar5 = pass1_1028_bb24(param_2);
    uVar4 = (uVar5 >> 0x10);
    puVar3 = &local_1a;
    pass1_1030_64ce(puVar3,uVar4,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),
                    uVar5 & 0xffff | uVar4 << 0x10,puVar7);
    uStack40 = *puVar3;
    uVar4 = (puVar3 + 2);
    bStack55 = (uStack40 >> 0x18);
    uVar2 = bStack55;
    uStack36 = uStack40;
    if (bStack55 != 0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack40 & 0xffff | uVar4 << 0x10);
      puVar6 = struct_op_1030_73a8(CONCAT22(uVar4,uVar2),uVar2,uVar4);
      uVar2 = puVar6;
      ppcVar1 = (*puVar6 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b46e(uVar2,param_2,param_3);
  fn_ptr_1030_7296(paStack6);
  return;
}



pub unsafe fn pass1_1028_0d80(mut param_1: u32) -> u16

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x20);
  pass1_1028_1646((param_1 & 0xffff | uVar2 << 0x10));
  return uVar1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_0d9c(mut param_1: i16,param_2: *mut astruct_15)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut BVar5: bool;
  let mut extraout_DX: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut puVar8: *mut u32;
  let mut uStack58: u32;
  let mut local_32: [u8;0x6] = [0;0x6];
  let mut puStack44: *mut u32;
  let mut uStack40: u32;
  let mut uStack36: u32;
  let mut puStack28: *mut u32;
  let mut local_1a: u32;
  let mut iStack22: i16;
  let mut uStack20: u16;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b514(param_2);
  pass1_1028_b58e(param_2);
  local_c = (param_1 + 0xc);
  iStack18 = (param_1 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_DX;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_1;
  uStack4 = extraout_DX;
  pass1_1028_bab6(iStack18,extraout_DX,param_2);
  uStack20 = pass1_1030_2fac(CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  uStack36 = CONCAT22(uStack36,&local_1a);
  iStack22 = iStack14 + 1;
  if (iStack22 <= uStack20) {
    puVar8 = CONCAT22(0x1050,local_32);
    iStack14 = iStack22;
    uVar7 = pass1_1028_bb24(param_2);
    uVar6 = (uVar7 >> 0x10);
    puVar2 = &local_1a;
    pass1_1030_64ce(puVar2,uVar6,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar2),
                    uVar7 & 0xffff | uVar6 << 0x10,puVar8);
    uStack40 = *puVar2;
    uVar6 = (puVar2 + 2);
    uStack58._3_1_ = (uStack40 >> 0x18);
    uVar3 = uStack58._3_1_;
    if (uStack58._3_1_ != 0) {
      uStack36 = uStack40;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack40 & 0xffff | uVar6 << 0x10);
      uStack58 = CONCAT22(uVar6,uVar3);
      uVar4 = pass1_1030_6fa0(CONCAT22(uVar6,uVar3));
      BVar5 = pass1_1008_c6ae(_u16_1050_06e0,uVar4,0x13);
      if (BVar5 != 0) {
        puStack44 = struct_op_1030_73a8(uStack58,BVar5,uVar6);
        ppcVar1 = (*puStack44 + 0x24);
        (**ppcVar1)();
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_0ea6(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut puVar1: *mut u16;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut iVar3: *mut astruct_15;
  let mut uVar4: *mut astruct_15;

  uVar4 = (param_2 >> 0x10);
  iVar3 = param_2;
  if (iVar3.field10_0xc != 0x10) {
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,iVar3.field10_0xc,0x13);
    if (BVar2 == 0) {
      BVar2 = pass1_1008_c6ae(_u16_1050_06e0,iVar3.field10_0xc,0x2);
      if (((BVar2 != 0) && (iVar3.field15_0x12 != 0x7)) && (iVar3.field15_0x12 != 0x4)) {
        uVar3 = pass1_1028_1556(BVar2,param_1,(param_2 & 0xffff | ZEXT24(uVar4) << 0x10));
//        if (uVar3 == 0) goto LAB_1028_0f0a;
        if (iVar3.field15_0x12 == 0x9) {
          iVar3.field15_0x12 = 0x5;
        }
      }
    }
    else if (iVar3.field25_0x22 < 1) {
      if ((iVar3.field15_0x12 != 0x5) && (iVar3.field15_0x12 != 0x6)) {
        return;
      }
      fn_ptr_1000_17ce(iVar3.field16_0x14);
      iVar3.field16_0x14 = NULL;//
// LAB_1028_0f0a:
      iVar3.field15_0x12 = 0x9;
      return;
    }
    pass1_1028_be2a(param_2);
    if (iVar3.field15_0x12 == 0x5) {
      puVar1 = &iVar3.field25_0x22;
      *puVar1 = *puVar1 - 0x1;
      pass1_1028_b58e((param_2 & 0xffff | ZEXT24(uVar4) << 0x10));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_0fa4(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut BVar1: bool;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar2: *mut astruct_15;
  let mut uVar2: *mut astruct_15;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: i16;

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  pass1_1028_be9e(param_3);
  puVar5 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(param_2,0x2),in_stack_0000fe9c,in_stack_0000ffc0
                           ,in_stack_0000ffc6,in_stack_0000ffca);
  paVar4 = (paVar4 & 0xffff0000 | puVar5 >> 0x10);
  iVar8 = (puVar5 + 0x82);
  uVar2 = (param_3 >> 0x10);
  iVar2 = param_3;
  if (iVar2.field15_0x12 == 0x5) {
    BVar1 = pass1_1008_c6ae(_u16_1050_06e0,iVar2.field10_0xc,0x2);
    if ((BVar1 != 0) && ((PTR_LOOP_1050_4fbc.is_null() || (iVar8 != 0)))) {
      PTR_LOOP_1050_4fbc = (&PTR_LOOP_1050_0000 + 1);
      uVar7 = 0;
      iVar8 = 0x4;
      uVar6 = 0x1;
      uVar3 = pass1_1028_b58e(param_3);
      pass1_1030_7c50(uVar3,paVar4,CONCAT22(paVar4,uVar3),CONCAT22(uVar7,uVar6),iVar8);
    }
    iVar2.field25_0x22 = 0x64;
  }
  return;
}
