pub unsafe fn pass1_1038_11b0(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,param_4: *mut u32,param_5: *mut u32)

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
    pass1_1038_0f8c(uVar3,param_2,param_3,(param_3 >> 0x10),param_4,uVar4);
    if (uVar3 == 0) { break; }
    uStack10 += 0x1;
  }
  return;
}







pub unsafe fn pass1_1038_134a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u32,param_5: *mut u32,param_6: *mut u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar3: u16;
  let mut unaff_CS: u16;
  let mut uVar4: u32;
  let mut puVar5: *mut u32;
  let mut uStack6: u32;

  ppcVar1 = (*param_6 + 0x10);
  puVar5 = param_6;
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_1);
  *param_4 = 0;
  loop {
    if (uStack6 <= *param_5) {
      return;
    }
    uVar4 = *param_5;
    *param_5 = *param_5 + 1;
    ppcVar1 = (*param_6 + 0x4);
    (**ppcVar1)(unaff_CS,param_6,uVar4,puVar5);
    uVar2 = uVar4;
    uVar3 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | extraout_DX_00 << 0x10);
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3,uVar2),uVar2,uVar3);
    uVar3 = (uVar4 >> 0x10);
    unaff_CS = 0x1028;
    uVar4 = pass1_1028_45e2(uVar4,uVar3,uVar4 & 0xffff | uVar3 << 0x10);
    uVar3 = (uVar4 >> 0x10);
    param_4 = uVar4;
    (param_4 + 0x2) = uVar3;
    if ((uVar3 | param_4) == 0) {break;}
  }
  return;
}




pub unsafe fn pass1_1038_13da(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u32,param_5: *mut u32,param_6: *mut u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar3: u16;
  let mut unaff_CS: u16;
  let mut uVar4: u32;
  let mut puVar5: *mut u32;
  let mut uStack6: u32;

  ppcVar1 = (*param_6 + 0x10);
  puVar5 = param_6;
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_1);
  *param_4 = 0;
  loop {
    if (uStack6 <= *param_5) {
      return;
    }
    uVar4 = *param_5;
    *param_5 = *param_5 + 1;
    ppcVar1 = (*param_6 + 0x4);
    (**ppcVar1)(unaff_CS,param_6,uVar4,puVar5);
    uVar2 = uVar4;
    uVar3 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | extraout_DX_00 << 0x10);
    if ((uVar3 | uVar2) == 0) {
      return;
    }
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3,uVar2),uVar2,uVar3 | uVar2);
    uVar3 = (uVar4 >> 0x10);
    if ((uVar3 | uVar4) == 0) {
      return;
    }
    unaff_CS = 0x1028;
    uVar4 = pass1_1028_3c32((uVar4 & 0xffff | uVar3 << 0x10));
    uVar3 = (uVar4 >> 0x10);
    param_4 = uVar4;
    (param_4 + 0x2) = uVar3;
    if  ((uVar3 | param_4) == 0) {break;}
  }
  return;
}





pub unsafe fn pass1_1038_1482(mut param_1: u16 ,mut param_2: u32,param_3: *mut u32,param_4: *mut u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut sqsVar2: u16;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut paVar11: *mut Struct57;
  let mut paVar13: *mut Struct57;
  let mut paVar14: *mut Struct57;
  let mut uVar15: u32;
  let mut unaff_DI: u16;
  let mut uVar16: u16;
  let mut uVar17: u8;
  let mut uVar18: u8;
  let mut lStack74: i32;
  let mut local_46: u32;
  let mut local_42: [u16;0x4] = [0;0x4];
  let mut uStack58: u16;
  let mut uStack56: u16;
  let mut puStack54: *mut u32;
  let mut puStack50: *mut u32;
  let mut uStack46: u32;
  let mut uStack42: u16;
  let mut uStack40: u16;
  let mut uStack38: u16;
  let mut uStack36: u16;
  let mut uStack34: u32;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut uStack26: u16;
  let mut uStack24: u16;
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut local_a: u32;
  let mut local_6: u32;
  let mut paVar12: *mut Struct57;

  local_6 = 0;
  local_a = 0;
  puVar4 = &local_a;
  uVar16 = (param_2 >> 0x10);
  uVar3 = param_2;
  pass1_1038_134a(puVar4,uVar3,uVar16,CONCAT22(0x1050,puVar4),CONCAT22(0x1050,&local_6),param_4);
  uStack14 = CONCAT22(param_1,puVar4);
  ppcVar1 = (*param_3 + 0x10);
  (**ppcVar1)();
  uStack18 = CONCAT22(param_1,puVar4);
  uStack22 = 0;
  loop {
    if (uStack18 <= uStack22) {
      return;
    }
    uStack14 |= uStack14;
    if (uStack14 == 0) {
      return;
    }
    pass1_1028_b58e(uStack14);
    uStack26 = uStack14;
    uStack24 = uStack18;
    pass1_1038_1a30(uVar3,uVar16,CONCAT22(uStack18,uStack14));
    uStack30 = uStack14;
    uStack28 = uStack18;
    if ((uStack18 | uStack14) != 0) {
      sVar2 = CONCAT22(uStack18,uStack14) * 0x64;
      uVar15 = (sVar2 >> 0x20);
      uVar7 = sVar2 >> 0x1;
      ppcVar1 = (*param_3 + 0x4);
      uStack34 = uVar7;
      (**ppcVar1)(0x1028,param_3,uStack22,(uStack22 >> 0x10));
      uVar6 = uVar7;
      uStack36 = uVar15;
      uStack38 = uVar6;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7 & 0xffff | uVar15 << 0x10);
      uStack40 = uVar15;
      uStack42 = uVar6;
      uStack46 = struct_op_1030_73a8(CONCAT22(uStack40,uVar6),uVar6,uStack40);
      paVar14 = (uVar15 & 0xffff0000);
      puStack50 = (uStack46 + 0x28);
      puStack54 = null_mut();
      uStack56 = (puStack50 + 0x4);
    //   for (uStack58 = 0; uVar5 = uStack56, uStack58 < uStack56; uStack58 += 1)

      uStack58 = 0;
      uVar5 = uStack56;
      while uStack58 < uStack56
      {
        pass1_1020_bb16(puStack50,CONCAT22(0x1050,&local_46),CONCAT13(0x10,CONCAT12(0x50,local_42)),
                        uStack58);
        if (((local_46 != 0) && (0xd < local_42[0])) && (local_42[0] < 0x1d)) {
          uVar15 = local_46;
          uVar7 = local_46;
          if (uStack34 < local_46) {
            uVar15 = uStack34 & 0xffff;
            uVar7 = uStack34;
          }
          paVar11 = (paVar14 & 0xffff0000 | uVar7 >> 0x10);
          uVar5 = uVar15;
          uVar10 = (uVar7 >> 0x10);
          if ((local_a <= uVar10) && ((local_a < uVar10 || (local_a < uVar5)))) {
            paVar11 = (paVar14 & 0xffff0000 | local_a);
            uVar5 = local_a;
          }
          iVar8 = paVar11;
          lStack74 = CONCAT22(iVar8,uVar5);
          uStack34 = CONCAT22(((uStack34 >> 0x10) - iVar8) - (uStack34 < uVar5),uStack34 - uVar5)
          ;
          local_a = CONCAT22((local_a - iVar8) - (local_a < uVar5),local_a - uVar5);
          paVar13 = paVar11;
          if (puStack54.is_null()) {
            paVar14 = paVar11;
            uVar10 = uVar5;
            mem_op_1000_179c(0xa,paVar11);
            uVar9 = paVar14 | uVar10;
            paVar13 = (paVar14 & 0xffff0000);
            paVar12 = (paVar13 | uVar9);
            if (uVar9 == 0) {
              uVar10 = 0;
            }
            else {
              pass1_1020_ba3e(CONCAT22(paVar14,uVar10),0x5,0x5);
              paVar13 = paVar12;
            }
            puStack54 = CONCAT22(paVar13,uVar10);
          }
          pass1_1020_bb8a(puStack54,uVar5,paVar11 & 0xffff | local_42[0] << 0x10);
          uVar7 = local_46 - lStack74;
          pass1_1020_bb8a(puStack50,uVar7,CONCAT22(local_42[0],(uVar7 >> 0x10)));
          paVar14 = paVar13;
          if (local_a == 0) {
            pass1_1038_1b3a(uVar7,uVar3,uVar16,uStack14,puStack54,unaff_DI);
            puStack54 = null_mut();
            uVar7 = ZEXT24(&local_a);
            pass1_1038_134a(&local_a,uVar3,uVar16,CONCAT22(0x1050,&local_a),CONCAT22(0x1050,&local_6),
                            param_4);
            uVar5 = uVar7;
            uStack14 = (uVar7 & 0xffff | paVar13 << 0x10);
            uVar10 = paVar13 | uVar5;
            paVar14 = (paVar13 & 0xffff0000 | uVar10);
            if (uVar10 != 0) {
              uVar17 = 0x64;
              uVar18 = 0;
              uVar6 = 0;
              pass1_1028_b58e((uVar7 & 0xffff | paVar13 << 0x10));
              uVar10 = paVar14;
              uStack26 = uVar5;
              uStack24 = uVar10;
              pass1_1038_1a30(uVar3,uVar16,CONCAT22(uVar10,uVar5));
              sVar2 = CONCAT22(uVar10,uVar5) * CONCAT22(uVar6,CONCAT11(uVar18,uVar17));
              paVar14 = (sVar2 >> 0x20);
              uVar7 = sVar2 >> 0x1;
              uStack34 = uVar7;
              uStack30 = uVar5;
              uStack28 = uVar10;
            }
          }
          uVar5 = uVar7;
          if ((uStack34 == 0) || (local_a == 0)) { break; }
        }
        uStack58 += 1;
      }
      if (puStack54.is_null() == false) {
        pass1_1038_1b3a(uVar5,uVar3,uVar16,uStack14,puStack54,unaff_DI);
        puStack54 = null_mut();
      }

    }
    uStack22 += 0x1;
  }
}





pub unsafe fn pass1_1038_16f2(mut param_1: u16 ,mut param_2: u32,param_3: *mut u32,param_4: *mut u32)

{
   let mut plVar1: *mut i32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut puVar5: *mut u32;
  let mut iVar6: i16;
  let mut puVar7: *mut u32;
  let mut puVar8: *mut u32;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut paVar13: *mut Struct57;
  let mut paVar14: *mut Struct57;
  let mut paVar15: *mut Struct57;
  let mut unaff_CS: u16;
  let mut uVar16: u32;
  let mut lVar17: i32;
  let mut uVar18: u16;
  let mut puVar19: *mut u32;
  let mut lStack68: i32;
  let mut puStack56: *mut u32;
  let mut puStack52: *mut u32;
   let mut plStack50: *mut i32;
  let mut uStack46: u16;
  let mut uStack42: u32;
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut local_a: u32;
  let mut local_6: u32;

  local_6 = 0;
  local_a = 0;
  puVar7 = &local_a;
  uVar18 = (param_2 >> 0x10);
  uVar4 = param_2;
  pass1_1038_13da(puVar7,uVar4,uVar18,CONCAT22(0x1050,puVar7),CONCAT22(0x1050,&local_6),
                  param_4);
  uStack14 = CONCAT22(param_1,puVar7);
  uVar9 = param_1 | puVar7;
  if (uVar9 != 0) {
    ppcVar2 = (*param_3 + 0x10);
    puVar19 = param_3;
    (**ppcVar2)();
    uStack18 = CONCAT22(uVar9,puVar7);
    for uStack22 in 0 .. uStack18 {
      ppcVar2 = (*param_3 + 0x4);
      uVar16 = uStack18;
      uVar10 = uVar9;
      (**ppcVar2)(unaff_CS,param_3,uStack22,(uStack22 >> 0x10),puVar19);
      iVar6 = uVar16;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar16 & 0xffff | uVar10 << 0x10);
      unaff_CS = 0x1030;
      uVar16 = struct_op_1030_73a8(CONCAT22(uVar10,iVar6),iVar6,uVar10);
      uVar11 = (uVar16 >> 0x10);
      uVar12 = uVar16;
      pass1_1038_1a30(uVar4,uVar18,CONCAT22(uVar10,iVar6));
      if ((uVar11 | uVar12) != 0) {
        uStack42 = (CONCAT22(uVar11,uVar12) * 0x64) >> 0x1;
        plVar1 = (iVar6 + 0x22);
        uVar12 = (iVar6 + 0x24);
        paVar13 = uVar12;
        uStack46 = plVar1;
        if ((uVar12 | uStack46) != 0) {
          plStack50 = null_mut();
          puVar7 = pass1_1028_0d80(uVar16);
          puStack56 = null_mut();
          puStack52 = puVar7;
          loop {
            lVar17 = pass1_1020_bae6(puStack52,paVar13,uStack46,
                                     CONCAT22(puStack52,(plVar1 >> 0x10)));
            uVar3 = paVar13 & 0xffff0000;
            puVar8 = lVar17;
            uVar12 = (lVar17 >> 0x10);
            paVar13 = (uVar3 | (uVar12 | puVar8));
            if ((uVar12 | puVar8) != 0) {
              paVar14 = (uVar3 | uVar12);
              if ((uStack42 <= uVar12) && ((uStack42 < uVar12 || (uStack42 < puVar8)))) {
                paVar14 = (uVar3 | uStack42);
                puVar8 = uStack42;
              }
              if ((local_a <= paVar14) &&
                 ((local_a < paVar14 || (local_a < puVar8)))) {
                paVar14 = (paVar14 & 0xffff0000 | local_a);
                puVar8 = local_a;
              }
              iVar6 = paVar14;
              lStack68 = CONCAT22(iVar6,puVar8);
              uStack42 = CONCAT22((uStack42 - iVar6) - (uStack42 < puVar8),
                                  uStack42 - puVar8);
              local_a = CONCAT22((local_a - iVar6) - (local_a < puVar8),
                                 local_a - puVar8);
              paVar13 = paVar14;
              if (plStack50.is_null()) {
                paVar15 = paVar14;
                puVar5 = puVar8;
                mem_op_1000_179c(0xa,paVar14);
                uVar12 = paVar15 | puVar5;
                paVar13 = (paVar15 & 0xffff0000 | uVar12);
                if (uVar12 == 0) {
                  puVar5 = null_mut();
                  paVar13 = (paVar15 & 0xffff0000);
                }
                else {
                  pass1_1020_ba3e(CONCAT22(paVar15,puVar5),0x5,0x5);
                }
                plStack50 = CONCAT22(paVar13,puVar5);
              }
              pass1_1020_bb8a(plStack50,puVar8,paVar14 & 0xffff | ZEXT24(puStack52) << 0x10);
              pass1_1020_bb8a(plVar1,(lVar17 - lStack68),
                              CONCAT22(puStack52,((lVar17 - lStack68) >> 0x10)));
              puStack56 = puStack52;
              puVar8 = puStack52;
              if (local_a == 0) {
                pass1_1038_1ac6(puStack52,uVar4,uVar18,uStack14,plStack50);
                plStack50 = null_mut();
                puVar8 = &local_a;
                pass1_1038_13da(puVar8,uVar4,uVar18,CONCAT22(0x1050,puVar8),
                                CONCAT22(0x1050,&local_6),param_4);
                uStack14 = CONCAT22(paVar13,puVar8);
                uVar12 = paVar13 | puVar8;
                paVar13 = (paVar13 & 0xffff0000 | uVar12);
                if (uVar12 == 0) {
                  return;
                }
              }
            }
            unaff_CS = 0x1020;
            if ((uStack42 == 0) || (local_a == 0)) { break; }
            unaff_CS = 0x1028;
            puVar8 = pass1_1028_0d80(uVar16);
            if ((puVar8 == puStack56) || ((puStack52 = puVar8, puStack56.is_null() && (puVar8 == puVar7)))) { break; }
          }
          if (plStack50.is_null() == false) {
            pass1_1038_1ac6(puVar8,uVar4,uVar18,uStack14,plStack50);
          }
        }
      }
    }
  }
  return;
}










pub unsafe fn pass1_1038_1a30(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut unaff_CS: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uStack18: u32;
  let mut uStack10: u32;
  let mut uStack6: u16;

  uVar5 = (param_3 >> 0x10);
  puVar1 = (param_3 + 0x1e);
  uVar7 = (param_3 + 0x20);
  uStack6 = puVar1;
  uVar3 = uVar7 | uStack6;
  if (uVar3 != 0) {
    ppcVar2 = (*puVar1 + 0x10);
    uVar6 = uStack6;
    (**ppcVar2)();
    uStack10 = CONCAT22(extraout_DX,uVar3);
    for uStack18 in 0 .. uStack10 {
      ppcVar2 = (*puVar1 + 0x4);
      uVar4 = uStack10;
      (**ppcVar2)(unaff_CS,uStack6,(puVar1 >> 0x10),uStack18,uVar6,uVar7);
      if ((extraout_DX_00 | uVar4) != 0) {
        unaff_CS = 0x1028;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | extraout_DX_00 << 0x10);
      }
    }
    return;
  }
  return;
}




pub unsafe fn pass1_1038_1ac6(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: u32)

{
  let mut extraout_DX: u16;
  let mut local_118: [u8;0x112] = [0;0x112];
  let mut uStack6: u32;

  pass1_1028_b58e(param_4);
  uStack6 = CONCAT22(extraout_DX,param_1);
  pass1_1030_e8a0(CONCAT22(0x1050,local_118),param_5,(param_4 + 0xc),
                  (param_1 + 0x4));
  pass1_1028_d52c(*_u16_1050_5748,*_PTR_LOOP_1050_65e2 + 0x1,CONCAT22(0x1050,local_118));
  return;
}




pub unsafe fn pass1_1038_1b3a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,param_5: *mut u32,mut param_6: u16 )

{
  let mut in_EDX: u32;
  let mut uVar2: u16;
  let mut paVar1: *mut Struct57;
  let mut local_1a: u32;
  let mut local_16: [u16;0x2] = [0;0x2];
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  pass1_1028_b58e(param_4);
  uStack6 = CONCAT22(in_EDX,param_1);
  uStack10 = param_4;
  uStack14 = pass1_1028_45e2(param_4,in_EDX,param_4);
  paVar1 = (in_EDX & 0xffff0000);
  uStack16 = (param_5 + 0x4);
//   for (uStack18 = 0; uVar2 = (paVar1 >> 0x10), uStack18 < uStack16; uStack18 += 1)
uStack18 = 0;
uVar2 = paVar1 >> 0x10;
while uStack18 < uStack16
  {
    pass1_1020_bb16(param_5,CONCAT22(0x1050,&local_1a),CONCAT22(0x1050,local_16),uStack18);
    paVar1 = CONCAT22(uVar2,uStack14);
    if (uStack14 < local_1a) {
      pass1_1030_7ddc(uStack14,paVar1,uStack6,uStack14,local_16[0]);
      uStack14 = 0;
    }
    else {
      uStack14 -= local_1a;
      pass1_1030_7ddc(local_1a,paVar1,uStack6,local_1a,local_16[0]);
    }
    if (uStack14 == 0) { break; }
    uStack18 += 1;
  }
  if (param_5.is_null() == false) {
    fn_ptr_1020_ba7e(param_5);
    fn_ptr_1000_17ce(param_5);
  }
  return;
}












pub unsafe fn pass1_1038_1d68(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u32,mut param_6: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut ppcVar6: *mut *mut code;
  let mut uVar7: u16;
  let mut bVar8: bool;
  let mut puVar9: *mut u8;
  let mut uVar10: u32;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u32;
  let mut iVar14: i16;
  let mut unaff_CS: u16;
  let mut puVar15: *mut u32;
  let mut paStack82: *mut astruct_99;
  let mut uStack78: u16;
  let mut uStack52: u32;
  let mut local_30: [u8;0x4] = [0;0x4];
  let mut uStack44: u32;
  let mut puStack40: *mut u32;
  let mut uStack36: u32;
  let mut local_20: [u8;0x4] = [0;0x4];
  let mut puStack28: *mut u32;
  let mut uStack24: u16;
  let mut uStack22: u16;
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut uStack16: u32;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut uStack6: u32;

  uStack6 = 0x64;
  uStack8 = 0;
  ppcVar6 = (*param_5 + 0x10);
  puVar15 = param_5;
  (**ppcVar6)();
  uStack12 = CONCAT22(param_2,param_1);
  uStack16 = 0;
  loop {
    if (uStack12 <= uStack16) {
      return;
    }
    ppcVar6 = (*param_5 + 0x4);
    uVar10 = uStack12;
    (**ppcVar6)(unaff_CS,param_5,uStack16,puVar15);
    uVar11 = uVar10;
    uStack18 = param_2;
    uVar12 = uStack18 | uVar11;
    uVar13 = uVar12;
    uStack20 = uVar11;
    if (uVar12 != 0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar10 & 0xffff | param_2 << 0x10);
      uStack22 = uVar12;
      unaff_CS = 0x1030;
      uStack24 = uVar11;
      puStack28 = struct_op_1030_73a8(CONCAT22(uStack22,uVar11),uVar11,uStack22);
      uVar13 = puStack28 >> 0x10;
      puVar9 = local_20;
      ppcVar6 = (*puStack28 + 0x40);
      (**ppcVar6)(0x1030,puStack28,(puStack28 >> 0x10),puVar9,0x1050);
      if (puVar9.is_null()) {
        uStack36 = pass1_1028_62c8(puStack28);
        uVar13 = uStack36 >> 0x10;
        uStack8 = 0x1;
        puStack40 = (param_6 + 0x22);
        pass1_1008_5784(CONCAT22(0x1050,local_30),puStack40);
        loop {
          uVar12 = uVar13;
          puVar9 = local_30;
          unaff_CS = 0x1008;
          pass1_1008_5b12(CONCAT22(0x1050,puVar9));
          uStack52 = CONCAT22(uVar12,puVar9);
          uVar13 = (uVar12 | puVar9);
          if ((uVar12 | puVar9) == 0) { break; }
          uVar2 = (puVar9 + 0x4);
          iVar3 = (puVar9 + 0x6);
          uVar4 = (puVar9 + 0x8);
          uVar11 = (puVar9 + 0xc);
          uVar5 = (puVar9 + 0xa);
          uVar7 = uVar11 / uVar5;
          uVar13 = uVar11 % uVar5;
          bVar8 = false;
          if (((0x0 < iVar3) && (!SBORROW2(iVar3,1))) && ((iVar3 == 0x5 || iVar3 -0x1 < 0x4 || (iVar3 == 0x8)))) {
            bVar8 = true;
          }
          if (bVar8) {
            uVar10 = uStack36;
            if (uStack6 < uStack36) {
              uVar10 = uStack6 & 0xffff;
              uStack36 = uStack6;
            }
            uVar11 = uStack36 | uVar10;
            uVar13 = uVar11;
            if (uVar11 == 0) { break; }
            uStack78 = ((uVar10 & 0xffff | uStack36 << 0x10) / uVar7);
            if (uStack78 < uVar5) {
              piVar1 = (puVar9 + 0xc);
              *piVar1 = *piVar1 - uVar10;
              piVar1 = (puVar9 + 0xa);
              *piVar1 = *piVar1 - uStack78;
            }
            else {
              ppcVar6 = (*puStack40 + 0xc);
              (**ppcVar6)(0x1008,puStack40,(puStack40 >> 0x10),uStack52);
              uStack44 = 0;
              uStack78 = uVar5;
            }
            paStack82 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
            uVar11 = (paStack82 >> 0x10);
            uVar12 = paStack82;
            if ((uVar11 | uVar12) == 0) {
              paStack82 = null_mut();
            }
            else {
              paStack82.field0_0x0 = 0x389a;
              (uVar12 + 0x2) = 0x1008;
              (uVar12 + 0x4) = 0;
              (uVar12 + 0x6) = 0;
              (uVar12 + 0x8) = 0;
              (uVar12 + 0xa) = 0;
              (uVar12 + 0xc) = 0;
              paStack82.field0_0x0 = 0x56ce;
              (uVar12 + 0x2) = 0x1018;
            }
            uVar12 = (paStack82 >> 0x10);
            iVar14 = paStack82;
            (iVar14 + 0xa) = uStack78;
            uVar10 = uStack78 * uVar7;
            uVar13 = uVar10 >> 0x10;
            (iVar14 + 0xc) = uVar10;
            (iVar14 + 0x4) = uVar2;
            (iVar14 + 0x6) = iVar3;
            (iVar14 + 0x8) = uVar4;
            pass1_1028_6408(puStack28,(paStack82 & 0xffff | uVar12 << 0x10));
          }
        }
      }
      else {
        ppcVar6 = (*param_5 + 0x8);
        (**ppcVar6)(0x1030,param_5,0x0,uStack16);
      }
    }
    uStack16 += 0x1;
    param_2 = uVar13;
  }
}