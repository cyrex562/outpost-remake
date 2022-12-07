
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_3246(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut in_stack_0000ffd8: u16;
  let mut local_20: [u8;0x6] = [0;0x6];
  let mut puStack26: *mut u32;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  paVar3 = CONCAT22(in_register_0000000a,param_2);
  pass1_1028_b58e(param_3);
  uStack6 = CONCAT22(paVar3,param_1);
  uStack10 = (param_1 + 0x2e);
  uVar2 = (uStack10 + 0x10);
  uVar5 = 0;
  iVar6 = 0x1;
  uVar4 = 0x1;
  uStack14 = uVar2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  uVar1 = uVar2;
  uStack16 = SUB42(paVar3,0x0);
  uStack18 = uVar1;
  pass1_1030_7c50(uVar1,paVar3,(uVar2 & 0xffff | paVar3 << 0x10),CONCAT22(uVar5,uVar4),iVar6);
  pass1_1030_7c50(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x1,0x2);
  pass1_1030_7c50(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x1,0x3);
  pass1_1030_7c50(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x1,0x5);
  puStack26 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,CONCAT22(in_stack_0000ffd8,0x2),in_stack_0000fe80,
                              in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
  paVar3 = (paVar3 & 0xffff0000 | puStack26 >> 0x10);
  uVar1 = puStack26;
  if ((uVar1 + 0x82) == 0) {
    pass1_1030_7c50(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x4,0x4);
  }
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x11);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x12);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x13);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x14);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x14,0x15);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x14,0x16);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x17);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x18);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x19);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x14,0x1a);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x14,0x1b);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x14,0x1c);
  if ((uStack10 + 0x200) == 0x8000002) {
    pass1_1020_a43e(paVar3,CONCAT22(0x1050,local_20));
    pass1_1020_a89e(CONCAT22(0x1050,local_20),(uStack6 + 0xc),(uStack6 >> 0x10));
  }
  return;
}



pub fn pass1_1028_33f6(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_3484(param_1: *mut u16)

{
  let mut in_EDX: *mut Struct57;

  struct_1028_0068(in_EDX,param_1);
  *param_1 = 0x34f6;
  (param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_34a6(param_1: *mut u8,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_00cc(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x34f6;
  (param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



pub fn pass1_1028_34d0(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_0138(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_355e(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x3608;
  (param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_3580(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x3608;
  (param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



pub unsafe fn pass1_1028_35aa() -> u16

{
  return 0x1;
}
pub fn pass1_1028_35b0(param_1: *mut astruct_15,mut param_2: i16)

{
  paVar1: *mut astruct_397;
  let mut uVar2: u16;

  paVar1 = pass1_1028_b58e(param_1);
  if (param_2 == 0) {
    uVar2 = 0;
  }
  else {
    uVar2 = 0x32;
  }
  pass1_1030_7d1c(paVar1,(paVar1 >> 0x10),paVar1,uVar2,0x230000);
  return;
}



pub fn pass1_1028_35e2(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_3670(param_1: *mut u8,param_2: *mut u16)

{
  let mut in_register_0000000a: u16;

  struct_1028_37a6(CONCAT22(in_register_0000000a,param_1),param_2);
  *param_2 = 0x373e;
  (param_2 + 0x2) = 0x1028;
  return param_2;
}



u16 * pass1_1028_3692(param_1: *mut u8,mut param_2: i16,mut param_3: u16 ,mut param_4: i16,mut param_5: u32)

{
  let mut in_register_0000000a: u16;

  pass1_1028_3816(CONCAT22(in_register_0000000a,param_1),CONCAT22(param_3,param_2),param_4,
                  param_5);
  CONCAT22(param_3,param_2) = 0x373e;
  (param_2 + 0x2) = 0x1028;
  return CONCAT22(param_3,param_2);
}



pub unsafe fn pass1_1028_36bc(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u32) -> u16

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut iStack4: i16;

  uVar5 = CONCAT22(param_2,param_1);
  *param_4 = 0;
  uVar4 = (param_3 >> 0x10);
  if ((param_3 + 0x28) != 0) {
    iStack4 = 0x4;
    while( true ) {
      if (0x1c < iStack4) break;
      uVar3 = (param_3 + 0x28);
      uVar5 = pass1_1020_bae6(uVar5,(uVar5 >> 0x10),uVar3,
                              CONCAT22(iStack4,(uVar3 >> 0x10)));
      uVar2 = param_4;
      param_4 = param_4 + uVar5;
      piVar1 = (param_4 + 2);
      *piVar1 = *piVar1 + (uVar5 >> 0x10) + CARRY2(uVar2,uVar5);
      if (0xf9 < *param_4) {
        return 0x1;
      }
      iStack4 += 0x1;
    }
  }
  return 0x0;
}



pub fn pass1_1028_3718(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_388e(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1028_37a6(param_1: *mut astruct_57,param_2: *mut astruct_180)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  iVar3: *mut astruct_180;
  let mut uVar3: u16;

  struct_1028_b354(param_2);
  uVar3 = (param_2 >> 0x10);
  iVar3 = param_2;
  uVar1 = 0;
  (iVar3 + 1) = 0;
  &iVar3[0x1].field_0x4 = 0;
  &iVar3[0x1].field_0x8 = 0;
  param_2.field0_0x0 = 0x3e2c;
  iVar3.field1_0x2 = 0x1028;
  mem_op_1000_179c(0xa,param_1);
  uVar2 = param_1 | uVar1;
  if (uVar2 == 0) {
    &iVar3[0x1].field_0x8 = 0;
  }
  else {
    pass1_1020_ba3e(CONCAT22(param_1,uVar1),0x5,0x5);
    &iVar3[0x1].field_0x8 = uVar1;
    &iVar3[0x1].field_0xa = uVar2;
  }
  return;
}
pub fn pass1_1028_3816(param_1: *mut astruct_57,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  uVar1 = 0;
  (param_2 + 0x20) = 0;
  (param_2 + 0x24) = 0;
  (param_2 + 0x28) = 0;
  param_2.field0_0x0 = 0x3e2c;
  (param_2 + 0x2) = 0x1028;
  mem_op_1000_179c(0xa,param_1);
  uVar2 = param_1 | uVar1;
  if (uVar2 == 0) {
    (param_2 + 0x28) = 0;
  }
  else {
    pass1_1020_ba3e(CONCAT22(param_1,uVar1),0x5,0x5);
    (param_2 + 0x28) = uVar1;
    (param_2 + 0x2a) = uVar2;
  }
  return;
}
pub fn pass1_1028_388e(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut pcVar2: *mut c_char;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  *param_1 = 0x3e2c;
  (iVar3 + 0x2) = 0x1028;
  pcVar2 = *(iVar3 + 0x28);
  uVar1 = (iVar3 + 0x2a);
  if ((uVar1 | pcVar2) != 0) {
    fn_ptr_1020_ba7e((pcVar2 & 0xffff | uVar1 << 0x10));
    fn_ptr_1000_17ce(pcVar2);
  }
  pass1_1028_b418(param_1);
  return;
}



pub unsafe fn pass1_1028_38d4(mut param_1: i16,mut param_2: u16 ,param_3: *mut u32,param_4: *mut u16,mut param_5: u32,mut param_6: u32) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut BVar2: bool;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;

  uVar4 = param_3;
  uVar5 = (param_3 >> 0x10);
  pass1_1028_c7b6(param_2,uVar4,uVar5,param_4,param_6);
  if ((param_1 == 0x5) || (param_1 == 0x6)) {
    ppcVar1 = (*param_3 + 0x60);
    uVar3 = (**ppcVar1)();
    if (uVar3 != 0) {
      pass1_1028_c23e(uVar3,(uVar3 >> 0x10),uVar4,uVar5,param_4,param_5,param_6);
      if (uVar3 != 0) {
        BVar2 = pass1_1028_c314(uVar3,(uVar3 >> 0x10),uVar4,uVar5,param_4,param_5,
                                (param_5 >> 0x10),param_6);
        if (BVar2 != 0) {
          return 0x1;
        }
      }
    }
  }
  else {
    PTR_LOOP_1050_50ca = 0x6a8;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_3958(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  i32 *plVar1;
  let mut iVar2: i16;
  qqVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uStack52: u32;
  u16 local_2c [0x2];
  let mut local_28: u32;
  let mut iStack36: i16;
  let mut uStack34: u32;
  let mut uStack30: u32;
  let mut uStack22: u16;
  let mut uStack20: u16;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut puStack10: *mut u32;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b58e(param_3);
  puStack10 = (param_1 + 0x22);
  uVar6 = (param_1 + 0x24);
  uVar7 = uVar6;
  if ((uVar6 | puStack10) != 0) {
    iStack6 = param_1;
    uStack4 = param_2;
    if (u16_1050_574c != 0) {
      uStack30 = (puStack10 + 0x4);
      for (uStack34 = 0; uStack34 < uStack30; uStack34 += 1) {
        pass1_1020_bb16(puStack10,CONCAT22(0x1050,local_2c),CONCAT22(0x1050,&local_28),uStack34
                       );
      }
    }
    uStack14 = (iStack6 + 0x2e);
    uStack18 = *_PTR_LOOP_1050_65e2;
    uStack20 = uStack18 & 0x1;
    for (uStack22 = 0x4; uStack22 < 0xe; uStack22 += 1) {
      local_2c[0] = uStack22;
      local_28 = pass1_1020_bae6(uStack22,uVar7,puStack10,
                                 CONCAT22(uStack22,(puStack10 >> 0x10)));
      uVar6 = (local_28 >> 0x10) | local_28;
      uVar7 = uVar6;
      if (uVar6 != 0) {
        pass1_1020_bb8a(puStack10,0x0,local_2c[0] << 0x10);
        uVar6 = -(local_28 + (local_28 != 0));
        uVar7 = uVar6;
        uStack34 = CONCAT22(uVar6,-local_28);
        pass1_1038_5694(uStack14,CONCAT22(uVar6,-local_28),local_2c[0]);
        uStack30 = 0;
        iStack36 = 0;
        iVar8 = param_3;
        uVar9 = (param_3 >> 0x10);
        switch(uStack22) {
        case 0x4:
          uStack30 = local_28 >> 0x1;
          if ((uStack30 == 0) && (uStack20 != 0)) {
            uStack30 = 0x1;
          }
          iStack36 = 0x11;
          break;
        case 0x5:
          uStack30 = local_28 >> 0x1;
          if ((uStack30 == 0) && (uStack20 != 0)) {
            uStack30 = 0x1;
          }
          iStack36 = 0x12;
          break;
        case 0x6:
          uStack30 = local_28 >> 0x1;
          if ((uStack30 == 0) && (uStack20 != 0)) {
            uStack30 = 0x1;
          }
          iStack36 = 0x13;
          break;
        case 0x7:
          uStack30 = local_28 >> 0x1;
          if ((uStack30 == 0) && (uStack20 != 0)) {
            uStack30 = 0x1;
          }
          iStack36 = 0x14;
          break;
        case 0x8:
          uStack30 = local_28;
          iStack36 = 0x1a;
          break;
        case 0x9:
          uStack30 = local_28;
          iStack36 = 0x1b;
          break;
        case 0xa:
          uStack30 = local_28;
          iStack36 = 0x1c;
          break;
        case 0xb:
          uStack30 = local_28;
          iStack36 = 0x17;
          break;
        case 0xc:
          iStack36 = 0x18;
          uStack30 = local_28;
          plVar1 = (iVar8 + 0x20);
          *plVar1 = *plVar1 + local_28;
          uVar6 = (iVar8 + 0x20);
          uVar4 = (iVar8 + 0x22);
          uVar5 = uVar6 >> 0x1 | ((uVar4 & 1) != 0) << 0xf;
          uStack52 = CONCAT22(uVar4 >> 0x1,uVar5);
          uVar5 = (uVar4 & 0xfffe) + CARRY2(uVar5,uVar5);
          uVar7 = uVar5;
          (iVar8 + 0x20) = uVar6 - (uVar6 & 0xfffe);
          (iVar8 + 0x22) = (uVar4 - uVar5) - (uVar6 < (uVar6 & 0xfffe));
          if (uStack52 != 0) {
            uVar10 = 0x15;//
// LAB_1028_3b14:
            uStack30 = local_28;
            pass1_1020_bb8a(*(i32 **)(iVar8 + 0x28),uStack52,CONCAT22(uVar10,(uStack52 >> 0x10)));
          }
          break;
        case 0xd:
          iStack36 = 0x19;
          uStack30 = local_28;
          plVar1 = (iVar8 + 0x24);
          *plVar1 = *plVar1 + local_28;
          uVar6 = (iVar8 + 0x24);
          iVar2 = (iVar8 + 0x26);
          qVar3 = (local_28 & 0xffff0000 | uVar6) / 0x3;
          uStack52 = qVar3;
          uStack52 = (qVar3 >> 0x10);
          uVar4 = qVar3;
          uVar5 = uStack52 * 0x3 + CARRY2(uVar4,uVar4) + CARRY2(uVar4 * 0x2,uVar4);
          uVar7 = uVar5;
          (iVar8 + 0x24) = uVar6 + uVar4 * -0x3;
          (iVar8 + 0x26) = (iVar2 - uVar5) - (uVar6 < uVar4 * 0x3);
          if (uStack52 != 0) {
            uVar10 = 0x16;
        // TODO: goto LAB_1028_3b14;
          }
        }
        if (((uStack30 | uStack30) != 0) && (iStack36 != 0)) {
          pass1_1020_bb70(*(i32 **)(iVar8 + 0x28),uStack30,CONCAT22(iStack36,uStack30));
        }
      }
    }
  }
  return;
}



pub fn pass1_1028_3c32(param_1: u32) -> u32

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut local_6: u16;
  let mut iStack4: i16;

  ppcVar1 = (*param_1 + 0x40);
  iVar2 = (**ppcVar1)();
  if (iVar2 != 0) {
    return 0x0;
  }
  return CONCAT22(-(0x3e8 < local_6) - iStack4,0x3e8 - local_6);
}
pub fn pass1_1028_3c60(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut local_10: i32;
  let mut local_c: [u8;0x4] = [0;0x4];
  let mut iStack8: i16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  uVar6 = CONCAT22(param_2,param_1);
  *param_4 = 0;
  uVar5 = (param_3 >> 0x10);
  iVar4 = param_3;
  if ((iVar4 + 0x28) != 0) {
    iStack8 = 0x4;
    while( true ) {
      if (0x1c < iStack8) break;
      uVar3 = (iVar4 + 0x28);
      uVar6 = pass1_1020_bae6(uVar6,(uVar6 >> 0x10),uVar3,
                              CONCAT22(iStack8,(uVar3 >> 0x10)));
      uVar2 = param_4;
      param_4 = param_4 + uVar6;
      piVar1 = (param_4 + 2);
      *piVar1 = *piVar1 + (uVar6 >> 0x10) + CARRY2(uVar2,uVar6);
      if (0x3e7 < *param_4) {
        return;
      }
      iStack8 += 0x1;
    }
  }
  uVar6 = (iVar4 + 0x28);
  uStack4 = (uVar6 + 0x4);
  uStack6 = 0;
  while( true ) {
    if (uStack4 <= uStack6) {
      return;
    }
    pass1_1020_bb16((iVar4 + 0x28),CONCAT22(0x1050,&local_10),CONCAT22(0x1050,local_c),
                    uStack6);
    *param_4 = *param_4 + local_10;
    if (0x3e7 < *param_4) break;
    uStack6 += 0x1;
  }
  return;
}
pub fn write_to_file_1028_3d0e(mut param_1: u16 ,param_2: *mut astruct_731,param_3: *mut u8)

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  let mut uVar3: u16;
  in_stack_0000ffd8: mut HFILE16;
  u32 local_10 [0x2];
  let mut local_8: u32;

  BVar1 = write_to_file_1028_b5ec(param_2,param_3);
  if (BVar1 != 0) {
    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    local_10[0] = (iVar2 + 0x20);
    BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_10),0x4,in_stack_0000ffd8);
    if (BVar1 != 0) {
      local_8 = (iVar2 + 0x24);
      BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffd8);
      if (BVar1 != 0) {
        write_to_file_1008_7a22(param_3,(iVar2 + 0x28));
        if (BVar1 != 0) {
          return;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}
pub fn pass1_1028_3d92(mut param_1: i16,param_2: *mut u8,param_3: *mut astruct_373,HFILE16 *param_4)

{
  let mut iVar1: i16;
  let mut BVar2: bool;
  let mut uVar3: u16;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0) {
    iVar1 = param_3;
    BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | (iVar1 + 0x20)),0x4);
    if (BVar2 != 0) {
      BVar2 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | (iVar1 + 0x24)),0x4);
      if (BVar2 != 0) {
        uVar3 = pass1_1008_7ad4(param_4,*(i32 **)(iVar1 + 0x28));
        if (uVar3 != 0) {
          return;
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



pub fn pass1_1028_3e06(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_388e(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_3e94(param_1: *mut astruct_180)

{
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = 0;
  param_1.field0_0x0 = 0x4004;
  (param_1 + 0x2) = 0x1028;
  pass1_1028_3fa2((param_1 & 0xffff | uVar1 << 0x10));
  return &param_1.field0_0x0;
}



pub fn pass1_1028_3ec8(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> u32

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  (param_2 + 0x20) = 0;
  param_2.field0_0x0 = 0x4004;
  (param_2 + 0x2) = 0x1028;
  pass1_1028_3fa2((param_2 & 0xffff | param_2 << 0x10));
  return param_2;
}
pub fn pass1_1028_3f04(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  paStack6: *mut astruct_397;

  uVar6 = 0x1f;
  pass1_1028_b58e(param_3);
  paStack6 = CONCAT22(param_2,param_1);
  uStack10 = pass1_1030_7c28(param_1,param_2,CONCAT22(param_2,param_1),uVar6);
  uVar5 = (uStack10 >> 0x10);
  paVar3 = CONCAT22(in_register_0000000a,uVar5);
  uVar2 = uStack10 & 0xffff;
  pass1_1030_7d1c(uVar2,uVar5,paStack6,0x0,0x1f0000);
  uVar5 = (param_3 >> 0x10);
  iVar4 = param_3;
  if ((iVar4 + 0xc) != 0x22) {
    if ((iVar4 + 0xc) == 0x23) {
      uVar1 = 0xa;
    }
    else {
      uVar1 = 0x5;
    }
    uStack14 = uVar1;
    uStack10 += (iVar4 + 0x20);
    (iVar4 + 0x20) = uStack10 % uVar1;
    uVar2 = uStack10 / uStack14;
    paVar3 = (uStack10 % uStack14);
    uStack10 += uVar2;
  }
  pass1_1030_7ddc(uVar2,paVar3,paStack6,uStack10,0x21);
  return;
}
pub fn pass1_1028_3fa2(param_1: *mut astruct_180)

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0xc) != 0x22) {
    if ((iVar2 + 0xc) == 0x23) {
      uVar1 = 0xa;
    }
    else {
      uVar1 = 0x5;
    }
    uVar1 >>= 0x1;
    pass1_1008_612e(uVar1,0x0,uVar1);
    (iVar2 + 0x20) = uVar1;
    (iVar2 + 0x22) = uVar1 >> 0xf;
  }
  return;
}



pub fn pass1_1028_3fde(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
