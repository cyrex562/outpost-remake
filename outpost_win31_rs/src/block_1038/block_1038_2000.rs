

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_201a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut astruct_412)

{
  let mut puVar1: *mut u16;
  let mut iVar2: i16;
  let mut ppcVar3: *mut *mut code;
  let mut lVar4: i32;
  let mut uVar6: u32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut paVar10: *mut Struct57;
  let mut uVar11: u32;
  let mut puVar12: *mut u8;
  let mut paVar13: *mut Struct57;
  iVar12: *mut astruct_412;
  let mut uVar14: u16;
  let mut puVar15: *mut u8;
  uVar16: *mut astruct_412;
  let mut uVar17: u16;
  paVar18: *mut astruct_385;
  let mut lStack24: i32;
  let mut lStack20: i32;
  let mut uStack10: u16;
  uVar7: *mut astruct_385;
  uVar5: *mut astruct_385;

  uVar17 = 0x1030;
  paVar18 = pass1_1030_6b16(param_5);
  uVar7 = (paVar18 >> 0x10);
  uVar5 = paVar18;
  if ((uVar7 | uVar5) == 0) {
    return;
  }
  uVar16 = (param_5 >> 0x10);
  iVar12 = param_5;
  iVar2 = &iVar12.field_0x34;
  lVar4 = iVar2;
  uVar6 = lVar4 * 0x64;
  puVar12 = (uVar6 >> 0x10);
  uVar8 = uVar6;
  uStack10 = 0;
  lStack20 = 0;
  if (uVar5.field4_0x4 == 0) {
    if (uVar5.field5_0x6 == 0) {
      if (uVar5.field6_0x8 == 0) goto LAB_1038_2102;
      uVar9 = pass1_1020_c42e(uVar5.field6_0x8);
      uVar11 = uVar5.field7_0xa * uVar9;
      puVar15 = (uVar11 >> 0x10);
      if (uVar11 + lVar4 * -0x64 != 0x0 && uVar6 <= uVar11) {
        uVar11 = uVar6 & 0xffff;
        puVar15 = puVar12;
      }
      uVar6 = uVar11 & 0xffff | ZEXT24(puVar15) << 0x10;
      uVar11 = (uVar11 & 0xffff | ZEXT24(puVar15) << 0x10) / uVar9;
      puVar1 = &uVar5.field7_0xa;
      *puVar1 = *puVar1 - uVar11;
      uStack10 = (uVar6 / 0x64);
      paVar13 = (uVar6 % 0x64);
      paVar10 = paVar13;
      if (paVar13.is_null() == false) {
        uStack10 += 0x1;
        paVar10 = uStack10;
      }
      uVar8 = paVar10;
      mem_op_1000_179c(0x2a,paVar13);
      puVar12 = (paVar13 | uVar8);
      if (puVar12.is_null()) goto LAB_1038_20fa;
      pass1_1038_6838(CONCAT22(paVar13,uVar8),uVar11,uVar5.field6_0x8,uStack10,
                      iVar12.field_0x4);
    }
    else {
      uVar9 = switch_1020_c3b4(uVar5.field5_0x6);
      uVar11 = uVar5.field7_0xa * uVar9;
      puVar15 = (uVar11 >> 0x10);
      if (uVar11 + lVar4 * -0x64 != 0x0 && uVar6 <= uVar11) {
        uVar11 = uVar6 & 0xffff;
        puVar15 = puVar12;
      }
      uVar6 = uVar11 & 0xffff | ZEXT24(puVar15) << 0x10;
      uVar11 = (uVar11 & 0xffff | ZEXT24(puVar15) << 0x10) / uVar9;
      puVar1 = &uVar5.field7_0xa;
      *puVar1 = *puVar1 - uVar11;
      uStack10 = (uVar6 / 0x64);
      paVar13 = (uVar6 % 0x64);
      paVar10 = paVar13;
      if (paVar13.is_null() == false) {
        uStack10 += 0x1;
        paVar10 = uStack10;
      }
      uVar8 = paVar10;
      mem_op_1000_179c(0x2a,paVar13);
      puVar12 = (paVar13 | uVar8);
      if (puVar12.is_null()) goto LAB_1038_20fa;
      pass1_1038_675c(CONCAT22(paVar13,uVar8),uVar11,uVar5.field5_0x6,uStack10,
                      iVar12.field_0x4);
    }
  }
  else {
    uVar14 = uVar5.field7_0xa;
    puVar15 = NULL;
    if ((puVar12 < 1) && ((0x7fff < puVar12 || (uVar8 < uVar14)))) {
      uVar14 = uVar8;
      puVar15 = puVar12;
    }
    lStack24 = CONCAT22(puVar15,uVar14);
    puVar1 = &uVar5.field7_0xa;
    *puVar1 = *puVar1 - uVar14;
    uStack10 = (lStack24 / 0x64);
    paVar13 = (lStack24 % 0x64);
    paVar10 = paVar13;
    if (paVar13.is_null() == false) {
      uStack10 += 0x1;
      paVar10 = uStack10;
    }
    uVar8 = paVar10;
    mem_op_1000_179c(0x2a,paVar13);
    puVar12 = (paVar13 | uVar8);
    if (puVar12.is_null()) {//
// LAB_1038_20fa:
      uVar17 = 0x1000;
      lStack20 = 0;
  // TODO: goto LAB_1038_2102;
    }
    pass1_1038_6590(CONCAT22(paVar13,uVar8),uVar14,puVar15,uVar5.field4_0x4,uStack10,
                    iVar12.field_0x4);
  }
  uVar17 = 0x1000;
  lStack20 = CONCAT22(puVar12,uVar8);//
// LAB_1038_2102:
  if (lStack20 != 0) {
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    uVar17 = 0x1030;
    uVar8 = uStack10;
    pass1_1030_6c4c(param_5,iVar2 - uStack10);
  }
  if (uVar5.field7_0xa == 0) {
    if ((uVar7 | uVar5) != 0) {
      ppcVar3 = paVar18;
      (**ppcVar3)(uVar17,uVar5,uVar7,1);
    }
  }
  else {
    pass1_1030_6c66(uVar8,puVar12,param_5,0x0,paVar18);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_2306(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_419)

{
  let mut piVar1: *mut i16;
  let mut paVar2: *mut Struct57;
  let mut ppcVar3: *mut *mut code;
  qqVar4: u16;
  let mut puVar5: *mut u32;
  let mut in_AX: u16;
  uVar9: *mut astruct_417;
  let mut puVar7: *mut Struct57;
  let mut in_DX: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut paVar8: *mut Struct57;
  let mut paVar9: *mut Struct57;
  iVar11: *mut astruct_419;
  iVar12: *mut astruct_417;
  uVar10: *mut astruct_419;
  let mut uVar11: u16;
  let mut uVar12: u32;
  let mut uStack42: u32;
  let mut uStack34: u32;
  let mut uStack30: u16;
  let mut uStack24: u32;
  let mut uStack12: u32;
  let mut iStack8: i16;
  uVar13: *mut astruct_417;

  uVar11 = 0x1030;
  uVar12 = struct_op_1030_73a8(param_3,in_AX,in_DX);
  paVar8 = (uVar12 >> 0x10);
  uVar10 = (param_3 >> 0x10);
  iVar11 = param_3;
  iStack8 = iVar11.field49_0x34;
  uStack12 = 0x64;
  paVar2 = *(astruct_57 **)(uVar12 + 0x22);
  puVar7 = paVar2;
  loop {
    uVar6 = paVar8;
    ppcVar3 = (paVar2 + 0x10);
    (**ppcVar3)(uVar11,paVar2,(paVar2 >> 0x10));
    uVar9 = puVar7;
    uVar13 = (puVar7 & 0xffff);
    puVar5 = (uVar13 | uVar6 << 0x10);
    if ((uVar6 | uVar9) == 0) break;
    if (uVar9.field8_0xa == 0) {
      paVar8 = (uVar6 | uVar9);
      if ((uVar6 | uVar9) != 0) {
        ppcVar3 = *puVar5;
        (**ppcVar3)(uVar11,uVar9,uVar6,1);
      }
    }
    else {
      uStack24 = 0;
      uStack30 = 0;
      if (uVar9.field6_0x6 == 0) {
        if (uVar9.field7_0x8 != 0) {
          uStack30 = pass1_1020_c42e(uVar9.field7_0x8);
      // TODO: goto LAB_1038_2385;
        }
      }
      else {
        uStack30 = switch_1020_c3b4(uVar9.field6_0x6);//
// LAB_1038_2385:
        uVar11 = 0x1020;
        uStack24 = (uVar9.field8_0xa * uStack30);
      }
      uStack12 = 0;
      if (uStack12 < uStack24) {
        uStack24 = uStack12 & 0xffff;
      }
      uStack34 = uStack24 | uStack12 << 0x10;
      uStack24 |= uStack12 << 0x10;
      qVar4 = uStack24 / uStack30;
      uVar12 = qVar4;
      paVar8 = (uStack24 % uStack30);
      piVar1 = &uVar9.field8_0xa;
      *piVar1 = *piVar1 - qVar4;
      if (*piVar1 == 0) {
        paVar8 = (uVar6 | uVar9);
        if ((uVar6 | uVar9) != 0) {
          ppcVar3 = *puVar5;
          (**ppcVar3)(uVar11,uVar9,uVar6,1);
        }
      }
      else {
        ppcVar3 = (paVar2 + 0x8);
        (**ppcVar3)();
      }
      uStack12 -= uStack34;
      puVar7 = NULL;
      uStack42 = 0;
      iVar12 = uVar13;
      if (iVar12.field6_0x6 == 0) {
        if (iVar12.field7_0x8 != 0) {
          mem_op_1000_179c(0x2a,paVar8);
          uVar7 = paVar8 | puVar7;
          paVar9 = uVar7;
          if (uVar7 == 0) goto LAB_1038_244e;
          pass1_1038_6838((puVar7 & 0xffff | paVar8 << 0x10),uVar12,iVar12.field7_0x8,0x1,
                          iVar11.field4_0x4);
      // TODO: goto LAB_1038_24b3;
        }
      }
      else {
        mem_op_1000_179c(0x2a,paVar8);
        uVar7 = paVar8 | puVar7;
        paVar9 = uVar7;
        if (uVar7 == 0) {//
// LAB_1038_244e:
          uVar11 = 0x1000;
          uStack42 = 0;
          paVar8 = paVar9;
        }
        else {
          pass1_1038_675c((puVar7 & 0xffff | paVar8 << 0x10),uVar12,iVar12.field6_0x6,0x1,
                          iVar11.field4_0x4);//
// LAB_1038_24b3:
          uVar11 = 0x1000;
          uStack42 = puVar7 & 0xffff | paVar9 << 0x10;
          paVar8 = paVar9;
        }
      }
      if (uStack42 != 0) {
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        iStack8 += -0x1;
        if (iStack8 == 0) break;
        uStack12 = 0x64;
      }
    }
  }
  pass1_1030_6c4c(param_3,iStack8);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_24e8(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  let mut in_AX: u16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_EDX: u32;
  let mut paVar6: *mut Struct57;
  let mut paVar8: *mut Struct57;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uStack30: u16;
  let mut puStack28: *mut u32;
  let mut local_18: u32;
  let mut local_14: u16;
  let mut uStack18: u16;
  let mut uStack16: u32;
  let mut puStack12: *mut u32;
  let mut iStack8: i16;
  let mut uStack6: u32;
  let mut paVar7: *mut Struct57;

  uStack6 = struct_op_1030_73a8(param_3,in_AX,in_EDX);
  paVar6 = (in_EDX & 0xffff0000 | uStack6 >> 0x10);
  uVar10 = (param_3 >> 0x10);
  iVar9 = param_3;
  iStack8 = (iVar9 + 0x34);
  puStack12 = (uStack6 + 0x28);
  uStack16 = 0x64;
  uStack18 = (puStack12 + 0x4);
  uVar3 = uStack18;
  mem_op_1000_179c(0xa,paVar6);
  uVar5 = uVar3;
  uVar4 = paVar6 | uVar5;
  paVar8 = (paVar6 & 0xffff0000);
  paVar7 = (paVar8 | uVar4);
  if (uVar4 == 0) {
    uVar5 = 0;
  }
  else {
    pass1_1020_ba3e((uVar3 & 0xffff | paVar6 << 0x10),0x5,0x5);
    paVar8 = paVar7;
  }
  puStack28 = CONCAT22(paVar8,uVar5);
  for (uStack30 = 0; uVar3 = uStack18, uStack30 < uStack18; uStack30 += 1) {
    pass1_1020_bb16(puStack12,CONCAT22(0x1050,&local_18),CONCAT22(0x1050,&local_14),uStack30);
    if (local_18 != 0) {
      uVar3 = local_18;
      uVar2 = local_18;
      if (uStack16 < local_18) {
        uVar3 = uStack16 & 0xffff;
        uVar2 = uStack16;
      }
      paVar8 = (paVar8 & 0xffff0000 | uVar2 >> 0x10);
      uVar5 = uVar3;
      uVar3 = uVar3 & 0xffff | uVar2 & 0xffff0000;
      iVar1 = ((local_18 >> 0x10) - (uVar2 >> 0x10)) - (local_18 < uVar5);
      local_18 = CONCAT22(iVar1,local_18 - uVar5);
      pass1_1020_bb8a(puStack12,local_18 - uVar5,CONCAT22(local_14,iVar1));
      pass1_1020_bb70(puStack28,uVar5,uVar2 >> 0x10 | local_14 << 0x10);
      uStack16 -= uVar3;
      if (uStack16 == 0) {
        mem_op_1000_179c(0x2a,paVar8);
        uVar5 = paVar8 | uVar3;
        paVar6 = (paVar8 & 0xffff0000 | uVar5);
        if (uVar5 == 0) {
          uVar3 = 0;
        }
        else {
          pass1_1038_666e((uVar3 & 0xffff | paVar8 << 0x10),puStack28,0x1,
                          (iVar9 + 0x4));
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        mem_op_1000_179c(0xa,paVar6);
        uVar5 = paVar6 | uVar3;
        paVar8 = (paVar6 & 0xffff0000);
        paVar7 = (paVar8 | uVar5);
        if (uVar5 == 0) {
          uVar3 = 0;
        }
        else {
          pass1_1020_ba3e((uVar3 & 0xffff | paVar6 << 0x10),0x5,0x5);
          paVar8 = paVar7;
        }
        puStack28 = (uVar3 & 0xffff | paVar8 << 0x10);
        iStack8 += -0x1;
        if (iStack8 == 0) break;
        uStack16 = 0x64;
      }
    }
  }
  pass1_1020_ba94(puStack28);
  uVar5 = paVar8 | uVar3;
  paVar8 = (paVar8 & 0xffff0000 | uVar5);
  if (uVar5 == 0) {
    if (puStack28.is_null() == false) {
      fn_ptr_1020_ba7e(puStack28);
      fn_ptr_1000_17ce(puStack28);
    }
  }
  else {
    mem_op_1000_179c(0x2a,paVar8);
    if ((paVar8 | uVar3) != 0) {
      pass1_1038_666e((uVar3 & 0xffff | paVar8 << 0x10),puStack28,0x1,
                      (iVar9 + 0x4));
    }
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
  }
  pass1_1030_6c4c(param_3,iStack8);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_26ee(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_AX: u16;
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut in_EDX: u32;
  let mut paVar6: *mut Struct57;
  let mut paVar8: *mut Struct57;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u32;
  let mut uVar12: u32;
  let mut uStack36: u32;
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut uStack16: u32;
  let mut uStack12: u16;
  let mut uStack10: u16;
  let mut iStack8: i16;
  let mut paVar7: *mut Struct57;

  uVar11 = struct_op_1030_73a8(param_3,in_AX,in_EDX);
  paVar6 = (in_EDX & 0xffff0000 | uVar11 >> 0x10);
  uVar10 = (param_3 >> 0x10);
  iVar9 = param_3;
  iStack8 = (iVar9 + 0x34);
  uStack12 = pass1_1028_0d80(uVar11);
  uVar2 = uStack12;
  uStack16 = 0x64;
  mem_op_1000_179c(0xa,paVar6);
  uVar3 = paVar6 | uVar2;
  paVar8 = (paVar6 & 0xffff0000);
  paVar7 = (paVar8 | uVar3);
  if (uVar3 == 0) {
    uVar2 = 0;
  }
  else {
    pass1_1020_ba3e((uVar2 & 0xffff | paVar6 << 0x10),0x5,0x5);
    paVar8 = paVar7;
  }
  uStack20 = uVar2;
  uStack18 = paVar8;
  uStack10 = uStack12;
  loop {
    uVar12 = pass1_1030_7c28(uVar2,paVar8,param_3,uStack10);
    uVar2 = paVar8 & 0xffff0000;
    uVar3 = uVar12;
    uVar4 = (uVar12 >> 0x10);
    paVar8 = (uVar2 | (uVar4 | uVar3));
    if ((uVar4 | uVar3) != 0) {
      paVar6 = (uVar2 | uVar4);
      uVar1 = uVar3;
      if ((uStack16 <= uVar4) && ((uStack16 < uVar4 || (uStack16 < uVar3)))) {
        paVar6 = (uVar2 | uStack16);
        uVar1 = uStack16;
      }
      iVar5 = paVar6;
      uStack36 = CONCAT22(iVar5,uVar1);
      paVar8 = paVar6;
      pass1_1030_7d1c(uVar1,iVar5,param_3,uVar3 - uVar1,
                      CONCAT22(uStack10,(uVar4 - iVar5) - (uVar3 < uVar1)));
      pass1_1020_bb70(CONCAT22(uStack18,uStack20),uVar1,paVar6 & 0xffff | uStack10 << 0x10);
      uStack16 -= uStack36;
      if (uStack16 == 0) {
        mem_op_1000_179c(0x2a,paVar8);
        uStack10 = uStack36;
        uVar3 = paVar8 | uStack10;
        paVar6 = (paVar8 & 0xffff0000 | uVar3);
        if (uVar3 == 0) {
          uStack10 = 0;
        }
        else {
          pass1_1038_666e((uStack36 & 0xffff | paVar8 << 0x10),CONCAT22(uStack18,uStack20),
                          0x1,(iVar9 + 0x4));
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        mem_op_1000_179c(0xa,paVar6);
        uVar3 = paVar6 | uStack10;
        paVar8 = (paVar6 & 0xffff0000);
        paVar7 = (paVar8 | uVar3);
        if (uVar3 == 0) {
          uStack10 = 0;
        }
        else {
          pass1_1020_ba3e(CONCAT22(paVar6,uStack10),0x5,0x5);
          paVar8 = paVar7;
        }
        uStack18 = paVar8;
        iStack8 += -0x1;
        uStack20 = uStack10;
        if (iStack8 == 0) break;
        uStack16 = 0x64;
      }
    }
    uStack10 = pass1_1028_0d80(uVar11);
    uVar2 = uStack10;
    if (uStack12 == 0) {
      uStack12 = uStack10;
    }
  } while (uStack12 != uStack10);
  pass1_1020_ba94(CONCAT22(uStack18,uStack20));
  uVar3 = paVar8 | uStack10;
  paVar8 = (paVar8 & 0xffff0000 | uVar3);
  if (uVar3 == 0) {
    if ((uStack18 | uStack20) != 0) {
      fn_ptr_1020_ba7e(CONCAT22(uStack18,uStack20));
      fn_ptr_1000_17ce(CONCAT22(uStack18,uStack20));
    }
  }
  else {
    mem_op_1000_179c(0x2a,paVar8);
    if ((paVar8 | uStack10) != 0) {
      pass1_1038_666e(CONCAT22(paVar8,uStack10),CONCAT22(uStack18,uStack20),0x1,
                      (iVar9 + 0x4));
    }
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
  }
  pass1_1030_6c4c(param_3,iStack8);
  return;
}



astruct_97 * pass1_1038_28d8(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0x3a97);
  param_1.offset_0x0 = 0x29fe;
    // just 0x1038
  (param_1 + 0x2) = &u16_1050_1038;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)),s_SCRoboMove_1050_59f8);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1038_290e(mut param_1: u16 ,undefined1 param_2,mut param_3: u16 ) -> u16

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
pub fn pass1_1038_2944(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32)

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
    for (iVar4 = 0x40; iVar4 != 0; iVar4 += -1) {
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



pub fn pass1_1038_29d2(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1.address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1038_2a0e(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32,mut param_4: u32,mut param_5: u32)

{
  iVar1: *mut astruct_97;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x2af7);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field259_0x108 = param_5;
  iVar1.field262_0x10c = param_4;
  iVar1.field264_0x110 = param_3;
  iVar1.field265_0x114 = param_2;
  param_1.offset_0x0 = 0x309a;
  iVar1.segment_0x2 = &u16_1050_1038;
  return;
}
pub fn pass1_1038_2a5c(param_1: *mut u16)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x309a;
  (iVar4 + 0x2) = &u16_1050_1038;
  puVar1 = (iVar4 + 0x114);
  uVar2 = (iVar4 + 0x116);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  puVar1 = (iVar4 + 0x110);
  uVar2 = (iVar4 + 0x112);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  *param_1 = 0x389a;
  (iVar4 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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
  pass1_1038_2c82(param_3,uVar1,uVar2,*(astruct_702 **)(uVar1 + 0x110),CONCAT22(param_2,param_1),uStack6);
  pass1_1038_2c82(param_3,uVar1,uVar2,*(astruct_702 **)(uVar1 + 0x114),uStack6,uStack10);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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
pub fn pass1_1038_2b9a(param_1: *mut astruct_422,param_2: *mut u8,param_3: *mut astruct_421)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut uVar4: *mut Struct57;
  iVar5: *mut astruct_421;
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
    for (iVar3 = 0x40; iVar3 != 0; iVar3 += -1) {
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



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_2c82(param_1: u8,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut astruct_702,mut param_5: u32,mut param_6: u32)

{
  let mut puVar1: *mut u16;
  let mut piVar2: *mut i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut ppcVar6: *mut *mut code;
  let mut uVar7: u32;
  let mut uVar8: u16;
  let mut puVar9: *mut u32;
  let mut iVar10: i16;
  let mut uVar11: u32;
  let mut in_EDX: u32;
  let mut paVar12: *mut Struct57;
  let mut pSVar13: *mut StructD;
  let mut paVar14: *mut Struct57;
  let mut uVar15: u16;
  let mut iVar16: i16;
  iVar15: *mut astruct_702;
  let mut uVar17: u16;
  let mut uVar18: u16;
  let mut puVar19: *mut u8;
  paVar20: *mut astruct_211;
  let mut puVar21: *mut u32;
  let mut in_stack_0000fe64: u16;
  let mut in_stack_0000fe76: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ffa0: u16;
  let mut in_stack_0000ffa4: u16;
  let mut uVar22: u8;
  let mut in_stack_0000ffce: u16;
  let mut uStack22: u32;
  let mut local_12: u32;
  paStack14: *mut astruct_702;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar17 = (param_6 >> 0x10);
  uVar15 = param_6;
  uStack6 = (uVar15 + 0x200);
  uVar18 = (param_5 >> 0x10);
  iVar16 = param_5;
  uStack10 = (iVar16 + 0x200);
  uVar3 = (iVar16 + 0x202);
  paVar12 = (in_EDX & 0xffff0000 | uVar3);
  puVar19 = (param_4 >> 0x10);
  iVar15 = param_4;
  iVar10 = iVar15.field10_0xc;
  if (iVar10 == 1) {
    paStack14 = param_4;
    pass1_1038_52b8(param_6,&iVar15.field8_0x8,&iVar15.field11_0xe);
    return;
  }
  if (iVar10 == 0x2) {
    paStack14 = param_4;
    if (iVar15.field11_0xe != 0) {
      pass1_1038_3efc(uVar15,uVar17,param_5,iVar15.field11_0xe,iVar15,puVar19);
      return;
    }
    pass1_1020_a43e(puVar19,CONCAT22(0x1050,&local_12));
    uStack22 = (paStack14 + 0x8);
    loop {
      uStack22 += -0x1;
      if ((uStack22 | uStack22) == 0) break;
      pass1_1020_a6ee(&local_12,uStack22 | uStack22,in_stack_0000fe64,
                      CONCAT13(0x10,CONCAT12(0x50,&local_12)),(paStack14 + 0x12));
    }
  }
  else {
    if (iVar10 == 0x3) {
      pass1_1038_3f38(param_6,param_5,iVar15.field11_0xe,0x0,uVar3);
      return;
    }
    if (iVar10 == 0x4) {
      uVar7 = uStack6 >> 0x10 & 0xff;
      pSVar13 = (in_EDX & 0xffff0000 | uVar7);
      if ((uStack6 == 1) && (uVar7 == 0)) {
        local_12 = (uVar15 + 0x1f6);
        pass1_1030_3694(NULL,local_12,&iVar15.field11_0xe,&iVar15.field8_0x8);
        (&iVar15.field11_0xe + 0x2) = local_12;
        iVar15.field12_0x12 = pSVar13;
      }
      else {
        if (_PTR_LOOP_1050_5f2c == 0) {
          PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar13);
          PTR_LOOP_1050_5f2e = pSVar13;
        }
        else {
        }
        uVar15 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
        (&iVar15.field11_0xe + 0x2) = uVar15;
        iVar15.field12_0x12 = PTR_LOOP_1050_5f2e;
        iVar10 = &iVar15.field11_0xe;
        if (iVar10 != 0x3) {
          if (iVar10 != 0x4) {
            uVar5 = (&iVar15.field11_0xe + 2);
            (uVar5 + 0x28) = &iVar15.field8_0x8;
            return;
          }
          uVar5 = (&iVar15.field11_0xe + 2);
          (uVar5 + 0xdc) = &iVar15.field8_0x8;
          return;
        }
        uVar5 = (&iVar15.field11_0xe + 2);
        (uVar5 + 0x64) = &iVar15.field8_0x8;
      }
    }
    else if (iVar10 == 0x5) {
      if (&iVar15.field11_0xe == 0xc) {
        if ((uStack6 == 1) && ((uStack6 & 0xff0000) == 0)) {
          uVar5 = (uVar15 + 0x1f6);
          uVar3 = iVar15.field8_0x8;
          iVar10 = iVar15.field9_0xa;
          uVar8 = -uVar3;
          uVar18 = (uVar5 >> 0x10);
          iVar16 = uVar5;
          puVar1 = (iVar16 + 0x170);
          uVar4 = *puVar1;
          *puVar1 = *puVar1 + uVar8;
          piVar2 = (iVar16 + 0x172);
          *piVar2 = (*piVar2 - (iVar10 + (uVar3 != 0))) + CARRY2(uVar4,uVar8);
        }
      }
      else {
        pass1_1038_43cc(iVar15,puVar19,uVar15,uVar17,iVar15.field8_0x8,&iVar15.field11_0xe);
      }
    }
    else {
      iVar10 += -0x7;
      if (iVar10 == 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar15.field11_0xe);
        paVar14 = paVar12;
        pass1_1038_349e(CONCAT22(paVar12,iVar10),(iVar16 + 0x200));
        uVar22 = (paVar12 >> 0x8);
        pass1_1038_4d0e(CONCAT13(uVar22,CONCAT12(paVar12,iVar10)),0x258);
        pass1_1038_4d0e(CONCAT13(uVar22,CONCAT12(paVar12,iVar10)),0x258);
        paVar20 =
                  mixed_1010_20ba(paVar14,_u16_1050_0ed0,CONCAT22(in_stack_0000ffce,0x3b),
                                  in_stack_0000fe76,in_stack_0000ff9a,in_stack_0000ffa0,in_stack_0000ffa4);
        paVar12 = (paVar14 & 0xffff0000 | paVar20 >> 0x10);
        pass1_1008_de58(paVar20,iVar15.field11_0xe,0x4000001);
        puVar21 = mixed_1010_20ba(paVar12,_u16_1050_0ed0,CONCAT22(in_stack_0000ffce,0x2f),
                                  in_stack_0000fe76,in_stack_0000ff9a,in_stack_0000ffa0,in_stack_0000ffa4);
        paVar12 = (paVar12 & 0xffff0000 | puVar21 >> 0x10);
        uVar11 = (puVar21 + 0x20);
        pass1_1030_8344(_u16_1050_5748,uVar11);
        local_12 = uVar11 & 0xffff | paVar12 << 0x10;
        iVar10 = pass1_1030_5b00(uVar11 & 0xffff | paVar12 << 0x10);
        paStack14 =
                    mixed_1010_20ba(paVar12,_u16_1050_0ed0,CONCAT22(in_stack_0000ffce,iVar10),
                                    in_stack_0000fe76,in_stack_0000ff9a,in_stack_0000ffa0,in_stack_0000ffa4);
        puVar9 = (paStack14 + 0x20);
        ppcVar6 = (*puVar9 + 0x4);
        (**ppcVar6)(0x1010,puVar9,(paStack14 >> 0x10),0x6);
      }
    }
  }
  return;
}
pub fn pass1_1038_2f92(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut puVar1: *mut u16;
  let mut piVar2: *mut i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar7: u32;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut iStack10: i16;

  uVar10 = (param_4 >> 0x10);
  iVar8 = param_4;
  uVar6 = (iVar8 + 0x200);
  uVar11 = (param_3 >> 0x10);
  iVar9 = param_3;
  iVar3 = (iVar9 + 0xc);
  if (iVar3 == 1) {
    uVar7 = (iVar9 + 0x8);
    pass1_1038_3cc0(param_4,uVar7,(uVar7 >> 0x10),(iVar9 + 0xe));
    return;
  }
  if (iVar3 == 0x4) {
    pass1_1030_355c((iVar8 + 0x1f6),(iVar9 + 0x10));
    return;
  }
  if (iVar3 == 0x5) {
    if ((iVar9 + 0xe) != 0xc) {
      pass1_1038_5798(param_4,(iVar9 + 0x8),(iVar9 + 0xe));
      return;
    }
    iStack10 = uVar6;
    if ((iStack10 == 1) && ((uVar6 & 0xff0000) == 0)) {
      uVar7 = (iVar8 + 0x1f6);
      uVar4 = (iVar9 + 0x8);
      iVar3 = (iVar9 + 0xa);
      uVar10 = (uVar7 >> 0x10);
      iVar8 = uVar7;
      puVar1 = (iVar8 + 0x170);
      uVar5 = *puVar1;
      *puVar1 = *puVar1 + uVar4;
      piVar2 = (iVar8 + 0x172);
      *piVar2 = *piVar2 + iVar3 + CARRY2(uVar5,uVar4);
      return;
    }
  }
  return;
}
