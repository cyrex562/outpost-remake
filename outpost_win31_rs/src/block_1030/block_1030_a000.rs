

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_a044(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,mut param_5: u32)

{
  code **ppcVar1;
  let mut puVar2: *mut u16;
  let mut puVar3: *mut u8;
  let mut iVar4: i16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut extraout_DX: u16;
  let mut uVar7: u16;
  let mut puVar8: *mut u32;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut local_17e: u16;
  let mut uStack380: u16;
  let mut iStack90: i16;
  let mut puStack78: *mut u32;
  let mut uStack70: u16;
  let mut iStack68: i16;
  let mut uStack66: u32;
  let mut puStack62: *mut u32;
  u8 local_3a [0xc];
  let mut local_2e: u32;
  let mut uStack42: u16;
  let mut iStack40: i16;
  let mut uStack38: u16;
  let mut local_24: i16;
  let mut local_22: i16;
  let mut uStack32: u32;
  let mut uStack28: u32;
  let mut uStack24: u32;
  let mut puStack20: *mut u16;
  let mut uStack18: u16;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut uStack12: u32;
  let mut local_8: u16;
  let mut local_6: i16;
  let mut local_4: i16;

  puVar2 = &local_8;
  pass1_1008_3eb4((astruct_615 *)param_4,CONCAT22(0x1050,puVar2),CONCAT22(0x1050,&local_6),
                  CONCAT22(0x1050,&local_4));
  pass1_1030_627e(puVar2,param_1,_PTR_LOOP_1050_5740,param_4,param_5);
  puStack20 = puVar2;
  uStack18 = param_1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_1,puVar2));
  uStack24 = CONCAT22(param_1,puVar2);
  uStack28 = (puVar2 + 0x17);
  uVar5 = (uStack28 + 0x4);
  uStack32 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  iStack40 = uVar5;
  uStack38 = param_1;
  puVar8 = pass1_1030_5b5c(iStack40,param_1);
  uVar6 = (puVar8 >> 0x10);
  local_2e = *puVar8;
  uStack42 = (puVar8 + 0x4);
  puStack78 = &local_2e;
  pass1_1008_3e94(CONCAT22(0x1050,&local_2e),CONCAT22(0x1050,&local_24),CONCAT22(0x1050,&local_22)
                 );
  iStack14 = local_4 + 1;
  uStack12 = CONCAT22(local_4 + -0x1,local_6 - 1);
  iStack16 = local_6 + 1;
  if (local_4 + -0x1 < 0x0) {
    uStack12 = (local_6 - 1);
  }
  if (local_22 <= iStack14) {
    iStack14 = local_22 + -0x1;
  }
  if (uStack12 < 0x0) {
    uStack12 &= 0xffff0000;
  }
  if (local_24 <= iStack16) {
    iStack16 = local_24 + -0x1;
  }
  pass1_1008_6c90(CONCAT22(0x1050,local_3a));
  uVar7 = 0x1008;
  pass1_1008_6cec(CONCAT22(0x1050,local_3a),local_8,CONCAT22(iStack14,iStack16),local_8,uStack12);
  puVar3 = local_3a;
  pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),param_5);
  puStack62 = CONCAT22(uVar6,puVar3);
  if ((uVar6 | puVar3) != 0) {
    uStack66 = 0;
    iStack68 = 0;
    for (uStack70 = uStack12; uStack70 <= iStack16; uStack70 += 1) {
      for (puStack78 = uStack12; puStack78 <= iStack14;
          puStack78 = (puStack78 + 1)) {
        ppcVar1 = (code **)(*puStack62 + 0x4);
        iVar4 = iStack68;
        iStack68 = iStack68 + 1;
        (**ppcVar1)(uVar7,puStack62,(puStack62 >> 0x10));
        uStack66 = CONCAT22(extraout_DX,iVar4);
        uStack66._3_1_ = (extraout_DX >> 0x8);
        if (uStack66._3_1_ == '\0') {
          iStack90 = iVar4;
          if (iVar4 == 0x7) {
            pass1_1008_3e76(param_4,local_8,uStack70,puStack78);
            uVar10 = uStack32;
            uVar11 = (uStack32 >> 0x10);
            uVar9 = 0x6;
          }
          else if (iVar4 == 0x8) {
            pass1_1008_3e76(param_4,local_8,uStack70,puStack78);
            uVar10 = uStack32;
            uVar11 = (uStack32 >> 0x10);
            uVar9 = 0x7;
          }
          else {
            if (iVar4 != 0x9) goto LAB_1030_a1d0;
            pass1_1008_3e76(param_4,local_8,uStack70,puStack78);
            uVar10 = uStack32;
            uVar11 = (uStack32 >> 0x10);
            uVar9 = 0x8;
          }
          uVar7 = 0x1028;
          struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,&local_17e),0x0,0x0,uVar9,param_4,
                              (param_4 >> 0x10),CONCAT22(uVar11,uVar10),param_5);
          fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_17e));
          local_17e = 0x389a;
          uStack380 = 0x1008;
        }//
LAB_1030_a1d0:
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_a278(mut param_1: u16 ,mut param_2: i16,param_3: *mut u16,param_4: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  let mut extraout_DX: u16;
  let mut puVar3: *mut u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  u8 local_134 [0x120];
  astruct_15 *paStack20;
  let mut uStack16: u32;
  astruct_15 *paStack12;
  let mut uStack6: u16;
  let mut uStack4: u16;

  uStack4 = 0x1;
  pass1_1030_a39a(param_3,param_4);
  if (param_1 != 0) {
    return;
  }
  uStack6 = param_1;
  pass1_1030_a3ae(param_3,param_4);
  puVar3 = param_4;
  uVar5 = (param_4 >> 0x10);
  if (param_1 == 0) {
    pass1_1030_a57e(0x0,param_2,param_3,param_4);
    if (param_1 == 0) {
      pass1_1030_a844(0x0,param_2,(astruct_426 *)param_3,param_4);
      if (param_1 == 0) {
        uStack4 = 0;
    // TODO: goto LAB_1030_a305;
      }
      iVar1 = (puVar3 + 1);
    }
    else {
      iVar1 = (puVar3 + 1);
    }
    if (iVar1 < 1) {
      uStack6 = 0x73;
    }
    else {
      uStack6 = 0x77;
    }
  }
  else if ((puVar3 + 1) < 1) {
    uStack6 = 0x7a;
  }
  else {
    uStack6 = 0x7f;
  }//
LAB_1030_a305:
  if (uStack6 != 0) {
    uVar6 = (param_3 >> 0x10);
    uVar4 = param_3;
    uVar2 = (uVar4 + 0x4);
    uStack16 = (uVar2 + 0x8);
    uVar2 = (uVar4 + 0x4);
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_134),0x0,0x0,uStack6,puVar3,uVar5,
                        (uVar2 + 0x4),uStack16);
    fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_134));
    paStack12 = paStack20;
    pass1_1028_b58e(paStack20);
    *param_3 = paStack20;
    (uVar4 + 0x2) = extraout_DX;
    if (0x0 < (puVar3 + 1)) {
      pass1_1030_a044(extraout_DX,uVar4,uVar6,(param_4 & 0xffff | uVar5 << 0x10),uStack16);
    }
  }
  return;
}
pub fn pass1_1030_a39a(mut param_1: u32,param_2: *mut u16)

{
  pass1_1030_aa18(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_a3ae(mut param_1: u32,param_2: *mut u16)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut BVar5: bool;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut in_EDX: u32;
  let mut uVar10: u16;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u32;
  let mut uVar11: u16;
  let mut iVar12: i16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut puVar15: *mut u32;
  let mut puVar16: *mut u16;
  let mut uVar17: u16;
  let mut uStack44: u32;
  let mut local_28: i16;
  let mut local_26: i16;
  let mut local_24: i16;
  u8 local_22 [0x6];
  let mut local_1c: i16;
  let mut iStack26: i16;
  let mut lStack22: i32;
  let mut uStack18: u32;
  let mut puStack14: *mut u32;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  uVar10 = (in_EDX >> 0x10);
  uStack4 = 0;
  iStack6 = (param_2 + 0x4);
  puVar15 = pass1_1008_c6fa(_u16_1050_06e0,0x45);
  uStack8 = (puVar15 >> 0x10);
  paVar8 = (astruct_57 *)CONCAT22(uVar10,uStack8);
  uVar3 = puVar15;
  uVar13 = (param_1 >> 0x10);
  uVar11 = param_1;
  uStack10 = uVar3;
  pass1_1038_4e78(uVar3,paVar8,(uVar11 + 0x4),puVar15);
  uVar14 = SUB42(paVar8,0x0);
  puStack14 = CONCAT22(uVar14,uVar3);
  ppcVar1 = (code **)(*puStack14 + 0x10);
  uVar17 = uVar3;
  uVar10 = uVar14;
  (**ppcVar1)(&u16_1050_1038,uVar3,uVar14);
  uStack18 = CONCAT22(uVar14,uVar3);
  uVar2 = (uVar11 + 0x4);
  lStack22 = *(i32 *)(uVar2 + 0x8);
  pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_1c));
  puVar16 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_22));
  uVar9 = puVar16 >> 0x10;
  uStack44 = 0;
  loop {
    uVar3 = uVar9;
    if (uStack18 <= uStack44) {//
LAB_1030_a4e7:
      if (puStack14 != NULL) {
        ppcVar1 = (code **)*puStack14;
        (**ppcVar1)(0x1008,puStack14,(puStack14 >> 0x10),0x1,uVar17,uVar10,puStack14,puStack14);
      }
      return;
    }
    uVar6 = uStack18;
    pass1_1030_1d58(puStack14);
    uVar7 = uVar3 | uVar6;
    uVar9 = uVar7;
    if (uVar7 != 0) {
      uVar9 = uVar3;
      pass1_1008_3f62(CONCAT22(0x1050,&local_1c),CONCAT22(uVar3,uVar6 + 0xc));
      pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_1c),CONCAT22(0x1050,&local_28),
                      CONCAT22(0x1050,&local_26),CONCAT22(0x1050,&local_24));
      if ((local_28 == iStack6) &&
         (uVar2 = (uVar11 + 0x4), uVar14 = (uVar2 >> 0x10), iVar12 = uVar2,
         uVar2 = (iVar12 + 0x4),
         uVar4 = pass1_1030_addc(&local_1c,uVar9,uVar11,uVar13,CONCAT22(0x1050,&local_1c),
                                 uVar2,(uVar2 >> 0x10),(iVar12 + 0x8)), uVar4 != 0))
      {
        pass1_1008_3f62(CONCAT22(0x1050,local_22),CONCAT22(0x1050,&local_1c));
        iStack26 = local_26 + -0x1;
        BVar5 = pass1_1030_ad22(&local_1c,uVar9,uVar11,uVar13,CONCAT22(0x1050,&local_1c),lStack22);
        if (BVar5 == 0) {
          iStack26 = local_26 + 1;
          BVar5 = pass1_1030_ad22(&local_1c,uVar9,uVar11,uVar13,CONCAT22(0x1050,&local_1c),lStack22);
          if (BVar5 == 0) {
            iStack26 = local_26;
            local_1c = local_24 + -0x1;
            BVar5 = pass1_1030_ad22(&local_1c,uVar9,uVar11,uVar13,CONCAT22(0x1050,&local_1c),lStack22);
            if (BVar5 == 0) {
              local_1c = local_24 + 1;
              BVar5 = pass1_1030_ad22(&local_1c,uVar9,uVar11,uVar13,CONCAT22(0x1050,&local_1c),lStack22);
              if (BVar5 == 0) goto LAB_1030_a45b;
            }
          }
        }
        pass1_1008_3f62(param_2,CONCAT22(0x1050,local_22));
        uStack4 = 0x1;
    // TODO: goto LAB_1030_a4e7;
      }
    }//
LAB_1030_a45b:
    uStack44 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_a57e(mut param_1: i16,mut param_2: i16,mut param_3: u32,param_4: *mut u16)

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut piVar6: *mut i16;
  let mut uVar7: u32;
  let mut uVar8: u16;
  let mut in_register_0000000a: u16;
  let mut paVar9: *mut Struct57;
  let mut uVar10: u32;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut iVar13: i16;
  let mut puVar14: *mut u32;
  let mut uVar15: u16;
  let mut uVar16: u16;
  let mut uVar17: u16;
  let mut uVar18: u16;
  let mut puVar19: *mut u32;
  let mut puVar20: *mut u16;
  let mut uVar21: u32;
  u8 uVar22;
  let mut uStack40: u32;
  u8 local_1c [0x2];
  let mut local_1a: i16;
  let mut local_18: i16;
  let mut local_16: i16;
  let mut iStack20: i16;
  let mut uStack16: u32;
  let mut uStack12: u16;
  let mut uStack10: u16;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  uStack4 = 0;
  uVar15 = (param_3 >> 0x10);
  uVar11 = param_3;
  pass1_1038_53ba((uVar11 + 0x4),1);
  if ((param_2 != 0) || (param_1 != 0)) {
    iStack6 = (param_4 + 0x4);
    iStack8 = 0x8 - (iStack6 == 0);
    puVar19 = pass1_1008_c6fa(_u16_1050_06e0,iStack8);
    uStack10 = (puVar19 >> 0x10);
    paVar9 = (astruct_57 *)CONCAT22(in_register_0000000a,uStack10);
    uVar4 = puVar19;
    uStack12 = uVar4;
    pass1_1038_4e78(uVar4,paVar9,(uVar11 + 0x4),puVar19);
    uStack16 = CONCAT22(paVar9,uVar4);
    uVar18 = 0x1008;
    puVar20 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_16));
    uVar10 = puVar20 >> 0x10;
    uVar3 = (uVar11 + 0x4);
    uVar1 = (uVar3 + 0x8);
    uVar16 = (uStack16 >> 0x10);
    uVar12 = SUB42(uStack16,0x0);
    ppcVar2 = (code **)(*uStack16 + 0x10);
    uVar7 = uVar1;
    (**ppcVar2)(0x1008,uVar12,uVar16);
    uVar7 = uVar7 & 0xffff | uVar10 << 0x10;
    for (uStack40 = 0; uVar4 = uVar10, uStack40 < uVar7; uStack40 += 1) {
      uVar21 = uVar7;
      pass1_1030_1d58(uStack16);
      uVar8 = uVar4 | uVar21;
      uVar10 = uVar8;
      if (uVar8 != 0) {
        uVar10 = uVar4;
        pass1_1008_3f62(CONCAT22(0x1050,&local_16),CONCAT22(uVar4,uVar21 + 0xc));
        uVar18 = 0x1008;
        pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_16),CONCAT22(0x1050,local_1c),
                        CONCAT22(0x1050,&local_1a),CONCAT22(0x1050,&local_18));
        uVar3 = (uVar11 + 0x4);
        uVar17 = (uVar3 >> 0x10);
        iVar13 = uVar3;
        uVar3 = (iVar13 + 0x4);
        uVar5 = pass1_1030_addc(&local_16,uVar10,uVar11,uVar15,CONCAT22(0x1050,&local_16),
                                uVar3,(uVar3 >> 0x10),(iVar13 + 0x8));
        if (uVar5 == 0) goto LAB_1030_a660;
        uVar21 = struct_op_1030_73a8((astruct_419 *)(uVar21 & 0xffff | uVar4 << 0x10),uVar5,uVar10);
        uVar10 = uVar21 >> 0x10;
        uVar5 = (uVar21 >> 0x10);
        iVar13 = (uVar21 + 0xc);
        if (0x5 < iVar13 - 0x7aU) goto LAB_1030_a660;
        uVar18 = 0x1030;
        switch(iVar13) {
        default:
          iStack20 = local_1a + -0x1;
          piVar6 = &local_16;
          pass1_1030_ad86(uVar5,uVar11,uVar15,CONCAT22(0x1050,piVar6),uVar1);
          if (piVar6 != NULL) goto LAB_1030_a7df;
          iStack20 = local_1a + 1;
          piVar6 = &local_16;
          pass1_1030_ad86(uVar10,uVar11,uVar15,CONCAT22(0x1050,piVar6),uVar1);
          if (piVar6 == NULL) {
            iStack20 = local_1a;
            local_16 = local_18 + -0x1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar10,uVar11,uVar15,CONCAT22(0x1050,piVar6),uVar1);
        // TODO: goto joined_r0x1030a722;
          }//
LAB_1030_a748:
          pass1_1008_3f62(param_4,CONCAT22(0x1050,&local_16));
          break;
        case 0x7b:
        case 0x7e:
          iStack20 = local_1a + -0x1;
          piVar6 = &local_16;
          pass1_1030_ad86(uVar5,uVar11,uVar15,CONCAT22(0x1050,piVar6),uVar1);
          if (piVar6 == NULL) {
            iStack20 = local_1a + 1;
        // TODO: goto LAB_1030_a730;
          }
          pass1_1008_3f62(param_4,CONCAT22(0x1050,&local_16));
          if (uStack16 == NULL) {
            return;
          }
          uVar18 = (uStack16 >> 0x10);
          puVar14 = uStack16;
          uVar22 = (uStack16 >> 0x10);
      // TODO: goto LAB_1030_a6ea;
        case 0x7c:
        case 0x7d:
          local_16 = local_18 + -0x1;
          piVar6 = &local_16;
          pass1_1030_ad86(uVar5,uVar11,uVar15,CONCAT22(0x1050,piVar6),uVar1);
joined_r0x1030a722:
          if (piVar6 == NULL) {
            local_16 = local_18 + 1;//
LAB_1030_a730:
            piVar6 = &local_16;
            pass1_1030_ad86(uVar10,uVar11,uVar15,CONCAT22(0x1050,piVar6),uVar1);
            if (piVar6 != NULL) goto LAB_1030_a748;
        // TODO: goto LAB_1030_a660;
          }//
LAB_1030_a7df:
          pass1_1008_3f62(param_4,CONCAT22(0x1050,&local_16));
        }
        puVar14 = uStack16;
        if ((uStack16 | puVar14) != 0) {
          uVar18 = (uStack16 >> 0x10);
          uVar22 = (uStack16 >> 0x10);//
LAB_1030_a6ea:
          ppcVar2 = (code **)*puVar14;
          (**ppcVar2)(0x1008,puVar14,uVar22,0x1,uVar12,uVar16,uStack16,uStack16);
        }
        return;
      }//
LAB_1030_a660:
    }
    if (uStack16 != NULL) {
      ppcVar2 = (code **)*uStack16;
      (**ppcVar2)(uVar18,uStack16,(uStack16 >> 0x10),0x1,uVar12,uVar16,uStack16,uStack16);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_a844(mut param_1: i16,mut param_2: i16,param_3: *mut astruct_426,param_4: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  code **ppcVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut piVar6: *mut i16;
  let mut puVar7: *mut u32;
  let mut extraout_DX: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  astruct_426 *uVar8;
  astruct_427 *iVar8;
  let mut iVar11: i16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut puVar14: *mut u16;
  let mut uVar15: u32;
  let mut uStack34: u32;
  let mut local_1c: i16;
  let mut local_1a: i16;
  let mut local_18: i16;
  let mut local_16: i16;
  let mut iStack20: i16;
  let mut uStack16: u16;
  let mut lStack14: i32;
  let mut uStack10: u32;
  let mut puStack6: *mut u32;

  uVar12 = (param_3 >> 0x10);
  uVar8 = (astruct_426 *)param_3;
  pass1_1038_53ba(uVar8->field4_0x4,1);
  if ((param_2 != 0) || (param_1 != 0)) {
    uVar15 = uVar8->field4_0x4;
    uVar13 = (uVar15 >> 0x10);
    iVar8 = (astruct_427 *)uVar15;
    puVar7 = iVar8->field12_0xc;
    ppcVar3 = (code **)(*puVar7 + 0x10);
    puStack6 = puVar7;
    (**ppcVar3)(&u16_1050_1038,puVar7,(&iVar8->field12_0xc + 0x2));
    uStack10 = puVar7 & 0xffff | extraout_DX << 0x10;
    uVar15 = uVar8->field4_0x4;
    lStack14 = *(i32 *)(uVar15 + 0x8);
    uStack16 = 0;
    puVar14 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_16));
    uVar9 = (puVar14 >> 0x10);
    iVar1 = (param_4 + 0x4);
    for (uStack34 = 0; uStack34 < uStack10; uStack34 += 1) {
      uVar15 = pass1_1030_1d7c(uStack10,uVar9,puStack6);
      uVar4 = (uVar15 >> 0x10);
      uVar10 = uVar4 | uVar15;
      uVar9 = uVar10;
      if ((uVar10 != 0) &&
         (uVar4 = pass1_1008_c6ae(_u16_1050_06e0,(uVar15 + 0xc),0x46), uVar9 = uVar10, uVar4 != 0x0
         )) {
        pass1_1030_1d58(puStack6);
        uVar9 = uVar10 | uVar4;
        if ((uVar10 | uVar4) != 0) {
          pass1_1008_3f62(CONCAT22(0x1050,&local_16),CONCAT22(uVar10,uVar4 + 0xc));
          pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_16),CONCAT22(0x1050,&local_1c),
                          CONCAT22(0x1050,&local_1a),CONCAT22(0x1050,&local_18));
          uVar9 = uVar10;
          if ((iVar1 == local_1c) &&
             (uVar15 = uVar8->field4_0x4, uVar13 = (uVar15 >> 0x10), iVar11 = uVar15,
             uVar2 = (iVar11 + 0x4),
             uVar5 = pass1_1030_addc(&local_16,uVar10,uVar8,uVar12,CONCAT22(0x1050,&local_16),
                                     uVar2,(uVar2 >> 0x10),(iVar11 + 0x8)),
             uVar9 = uVar10, uVar5 != 0)) {
            iStack20 = local_1a + -0x1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar10,uVar8,uVar12,CONCAT22(0x1050,piVar6),lStack14);
            if (piVar6 != NULL) {//
LAB_1030_a98e:
              pass1_1008_3f62(param_4,CONCAT22(0x1050,&local_16));
              return;
            }
            iStack20 = local_1a + 1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar10,uVar8,uVar12,CONCAT22(0x1050,piVar6),lStack14);
            if (piVar6 != NULL) goto LAB_1030_a98e;
            iStack20 = local_1a;
            local_16 = local_18 + -0x1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar10,uVar8,uVar12,CONCAT22(0x1050,piVar6),lStack14);
            if (piVar6 != NULL) goto LAB_1030_a98e;
            local_16 = local_18 + 1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar10,uVar8,uVar12,CONCAT22(0x1050,piVar6),lStack14);
            uVar9 = uVar10;
            if (piVar6 != NULL) goto LAB_1030_a98e;
          }
        }
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_aa18(mut param_1: u32,param_2: *mut u16)

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut BVar5: bool;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut in_EDX: u32;
  let mut paVar9: *mut Struct57;
  let mut uVar10: u32;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut iVar13: i16;
  let mut puVar14: *mut u32;
  let mut uVar15: u16;
  let mut uVar16: u16;
  let mut uVar17: u16;
  let mut uVar18: u16;
  let mut puVar19: *mut u32;
  let mut puVar20: *mut u16;
  let mut uVar21: u32;
  u8 uVar22;
  let mut uStack38: u32;
  u8 local_1a [0x2];
  let mut local_18: i16;
  let mut local_16: i16;
  let mut local_14: i16;
  let mut iStack18: i16;
  let mut uStack14: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut iStack4: i16;

  uVar12 = (in_EDX >> 0x10);
  iStack4 = (param_2 + 0x4);
  iStack6 = 0x8 - (iStack4 == 0);
  puVar19 = pass1_1008_c6fa(_u16_1050_06e0,iStack6);
  uStack8 = (puVar19 >> 0x10);
  paVar9 = (astruct_57 *)CONCAT22(uVar12,uStack8);
  uVar8 = puVar19;
  uVar15 = (param_1 >> 0x10);
  uVar11 = param_1;
  uStack10 = uVar8;
  pass1_1038_4e78(uVar8,paVar9,(uVar11 + 0x4),puVar19);
  uStack14 = CONCAT22(paVar9,uVar8);
  uVar18 = 0x1008;
  puVar20 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_14));
  uVar10 = puVar20 >> 0x10;
  uVar3 = (uVar11 + 0x4);
  uVar1 = (uVar3 + 0x8);
  uVar16 = (uStack14 >> 0x10);
  uVar12 = SUB42(uStack14,0x0);
  ppcVar2 = (code **)(*uStack14 + 0x10);
  uVar6 = uVar1;
  (**ppcVar2)(0x1008,uVar12,uVar16);
  uVar6 = uVar6 & 0xffff | uVar10 << 0x10;
  uStack38 = 0;
  while( true ) {
    uVar8 = uVar10;
    if (uVar6 <= uStack38) {
      if (uStack14 != NULL) {
        ppcVar2 = (code **)*uStack14;
        (**ppcVar2)(uVar18,uStack14,(uStack14 >> 0x10),0x1,uVar12,uVar16,uStack14,uStack14);
      }
      return;
    }
    uVar21 = uVar6;
    pass1_1030_1d58(uStack14);
    uVar7 = uVar8 | uVar21;
    uVar10 = uVar7;
    if (uVar7 != 0) break;//
LAB_1030_aadc:
    uStack38 += 0x1;
  }
  uVar10 = uVar8;
  pass1_1008_3f62(CONCAT22(0x1050,&local_14),CONCAT22(uVar8,uVar21 + 0xc));
  uVar18 = 0x1008;
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_14),CONCAT22(0x1050,local_1a),
                  CONCAT22(0x1050,&local_18),CONCAT22(0x1050,&local_16));
  uVar3 = (uVar11 + 0x4);
  uVar17 = (uVar3 >> 0x10);
  iVar13 = uVar3;
  uVar3 = (iVar13 + 0x4);
  uVar4 = pass1_1030_addc(&local_14,uVar10,uVar11,uVar15,CONCAT22(0x1050,&local_14),uVar3
                          ,(uVar3 >> 0x10),(iVar13 + 0x8));
  if (uVar4 == 0) goto LAB_1030_aadc;
  uVar21 = struct_op_1030_73a8((astruct_419 *)(uVar21 & 0xffff | uVar8 << 0x10),uVar4,uVar10);
  uVar10 = uVar21 >> 0x10;
  uVar8 = (uVar21 >> 0x10);
  iVar13 = (uVar21 + 0xc);
  if (0x5 < iVar13 - 0x7aU) goto LAB_1030_aadc;
  uVar18 = 0x1030;
  switch(iVar13) {
  default:
    iStack18 = local_18 + -0x1;
    BVar5 = pass1_1030_acbe(&local_14,uVar8,uVar11,uVar15,CONCAT22(0x1050,&local_14),uVar1);
    if (BVar5 != 0) goto LAB_1030_ac5b;
    iStack18 = local_18 + 1;
    BVar5 = pass1_1030_acbe(&local_14,uVar10,uVar11,uVar15,CONCAT22(0x1050,&local_14),uVar1);
    if (BVar5 == 0) {
      iStack18 = local_18;
      local_14 = local_16 + -0x1;
      BVar5 = pass1_1030_acbe(&local_14,uVar10,uVar11,uVar15,CONCAT22(0x1050,&local_14),uVar1);
  // TODO: goto joined_r0x1030ab9e;
    }//
LAB_1030_abc4:
    pass1_1008_3f62(param_2,CONCAT22(0x1050,&local_14));
    break;
  case 0x7b:
  case 0x7e:
    iStack18 = local_18 + -0x1;
    BVar5 = pass1_1030_acbe(&local_14,uVar8,uVar11,uVar15,CONCAT22(0x1050,&local_14),uVar1);
    if (BVar5 == 0) {
      iStack18 = local_18 + 1;
  // TODO: goto LAB_1030_abac;
    }
    pass1_1008_3f62(param_2,CONCAT22(0x1050,&local_14));
    if (uStack14 == NULL) {
      return;
    }
    uVar18 = (uStack14 >> 0x10);
    puVar14 = uStack14;
    uVar22 = (uStack14 >> 0x10);
// TODO: goto LAB_1030_ab66;
  case 0x7c:
  case 0x7d:
    local_14 = local_16 + -0x1;
    BVar5 = pass1_1030_acbe(&local_14,uVar8,uVar11,uVar15,CONCAT22(0x1050,&local_14),uVar1);
joined_r0x1030ab9e:
    if (BVar5 == 0) {
      local_14 = local_16 + 1;//
LAB_1030_abac:
      BVar5 = pass1_1030_acbe(&local_14,uVar10,uVar11,uVar15,CONCAT22(0x1050,&local_14),uVar1);
      if (BVar5 != 0) goto LAB_1030_abc4;
  // TODO: goto LAB_1030_aadc;
    }//
LAB_1030_ac5b:
    pass1_1008_3f62(param_2,CONCAT22(0x1050,&local_14));
  }
  puVar14 = uStack14;
  if ((uStack14 | puVar14) != 0) {
    uVar18 = (uStack14 >> 0x10);
    uVar22 = (uStack14 >> 0x10);//
LAB_1030_ab66:
    ppcVar2 = (code **)*puVar14;
    (**ppcVar2)(0x1008,puVar14,uVar22,0x1,uVar12,uVar16,uStack14,uStack14);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1030_acbe(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,i32 param_6)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_5,param_6);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar2 | param_1) != 0) {
      uVar3 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,param_1),param_1,uVar2 | param_1);
      if ((uVar3 != 0) && ((iVar1 = (uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1030_ad22(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,i32 param_6)

{
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut uVar3: u32;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_5,param_6);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar2 | param_1) != 0) {
      uVar3 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,param_1),param_1,uVar2 | param_1);
      if (uVar3 != 0) {
        BVar1 = pass1_1008_c6ae(_u16_1050_06e0,(uVar3 + 0xc),0x46);
        return BVar1;
      }
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_ad86(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,i32 param_5)

{
  let mut uVar1: u32;
  let mut puVar2: *mut u32;
  let mut cStack17: u8;
  let mut local_a: u32;
  let mut iStack6: i16;

  puVar2 = &local_a;
  pass1_1030_64ce(puVar2,param_1,_PTR_LOOP_1050_5740,param_4,param_5,CONCAT22(0x1050,puVar2));
  uVar1 = *puVar2;
  cStack17 = (uVar1 >> 0x18);
  if (cStack17 == '\0') {
    iStack6 = uVar1;
    if (((0x0 < iStack6) && (!SBORROW2(iStack6,1))) &&
       ((iStack6 == 0x3 || iStack6 + -0x2 < 0x1 || ((0x3 < iStack6 + -0x3 && (iStack6 + -0x7 < 0x2)))))) {
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_addc(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,mut param_6: u16 ,
                      mut param_7: u16 ,mut param_8: u32)

{
  let mut puVar1: *mut u32;
  let mut local_14: i16;
  let mut local_12: i16;
  let mut local_10: i16;
  let mut local_e: i16;
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_8);
  iStack6 = param_1;
  uStack4 = param_2;
  puVar1 = pass1_1030_5b5c(param_1,param_2);
  local_c = *puVar1;
  uStack8 = (puVar1 + 0x4);
  pass1_1008_3e94(param_5,CONCAT22(0x1050,&local_10),CONCAT22(0x1050,&local_e));
  pass1_1008_3e94(CONCAT22(0x1050,&local_c),CONCAT22(0x1050,&local_14),CONCAT22(0x1050,&local_12))
  ;
  if ((((0x1 < local_e) && (0x1 < local_10)) && (local_e < local_12 + -1)) && (local_10 < local_14 + -1)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Unable to use type for symbol uVar1
pub fn pass1_1030_ae6c(param_1: *mut astruct_399)

{
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  let mut paVar4: *mut Struct57;
  astruct_399 *iVar4;
  astruct_399 *uVar4;
  let mut puVar6: *mut u16;
  let mut uVar1: u32;

  uVar5 = (in_EDX >> 0x10);
  uVar4 = (astruct_399 *)(param_1 >> 0x10);
  iVar4 = (astruct_399 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar4->field1_0x2 = 0x1008;
  iVar4->field2_0x4 = 0;
  puVar6 = pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | ZEXT24(&iVar4->field3_0x8)));
  paVar4 = (astruct_57 *)CONCAT22(uVar5,(puVar6 >> 0x10));
  uVar2 = 0;
  iVar4->field8_0xe = 0;
  &iVar4->field9_0x10 = 0;
  param_1->field0_0x0 = 0xb932;
  iVar4->field1_0x2 = 0x1030;
  mem_op_1000_179c(0xc,paVar4);
  uVar3 = paVar4 | uVar2;
  if (uVar3 == 0) {
    &iVar4->field9_0x10 = 0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar4,uVar2));
    iVar4->field9_0x10 = uVar2;
    iVar4->field10_0x12 = uVar3;
  }
  uVar1 = &iVar4->field9_0x10;
  (uVar1 + 0xa) = 0;
  return;
}



// WARNING: Unable to use type for symbol uVar1
pub fn pass1_1030_aefa(param_1: *mut astruct_400,mut param_2: u32)

{
  let mut uVar2: u16;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut uVar6: u16;
  let mut paVar5: *mut Struct57;
  astruct_400 *iVar5;
  let mut uVar7: u16;
  let mut puVar8: *mut u16;
  let mut uVar1: u32;

  uVar6 = (in_EDX >> 0x10);
  uVar7 = (param_1 >> 0x10);
  iVar5 = (astruct_400 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar5->field1_0x2 = 0x1008;
  iVar5->field2_0x4 = 0;
  puVar8 = pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | ZEXT24(&iVar5->field3_0x8)));
  paVar5 = (astruct_57 *)CONCAT22(uVar6,(puVar8 >> 0x10));
  iVar5->field8_0xe = 0;
  &iVar5->field9_0x10 = 0;
  param_1->field0_0x0 = 0xb932;
  iVar5->field1_0x2 = 0x1030;
  iVar5->field2_0x4 = (param_2 + 0x4);
  puVar3 = (param_1 & 0xffff0000 | ZEXT24(&iVar5->field3_0x8));
  pass1_1008_3f62(puVar3,(param_2 & 0xffff0000 | (param_2 + 0xc)));
  uVar2 = puVar3;
  mem_op_1000_179c(0xc,paVar5);
  uVar4 = paVar5 | uVar2;
  if (uVar4 == 0) {
    uVar2 = 0;
    uVar4 = 0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar5,uVar2));
  }
  iVar5->field9_0x10 = uVar2;
  iVar5->field10_0x12 = uVar4;
  uVar1 = &iVar5->field9_0x10;
  (uVar1 + 0xa) = 0;
  return;
}
pub fn pass1_1030_afa6(StructD *param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u32;
  StructD *iVar5;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar5 = param_1;
  param_1->address_offset_field_0x0 = 0xb932;
  iVar5->address_offset_field_0x2 = 0x1030;
  if (*(i32 *)&iVar5->field_0x10 != 0) {
    uVar4 = &iVar5->field_0x10;
    (uVar4 + 0xa) = 0x1;
  }
  puVar1 = &iVar5->field_0x10;
  uVar2 = iVar5->field11_0x12;
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  param_1->address_offset_field_0x0 = 0x389a;
  iVar5->address_offset_field_0x2 = 0x1008;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_affc(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut BVar4: bool;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut uVar8: u32;
  let mut iStack12: i16;
  let mut uStack10: u32;
  let mut local_6: u32;

  uVar8 = ZEXT24(&local_6);
  pass1_1030_b718(param_1,param_1,param_1,
                  (param_1 & 0xffff0000 | (param_1 + 0x8)),CONCAT22(0x1050,&local_6));
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,local_6);
  uStack10 = (astruct_419 *)(uVar8 & 0xffff | ZEXT24(param_1) << 0x10);
  uVar5 = param_1 | uVar8;
  if (uVar5 != 0) {
    uVar7 = struct_op_1030_73a8((astruct_419 *)(uVar8 & 0xffff | ZEXT24(param_1) << 0x10),uVar8,uVar5);
    uVar5 = (uVar7 >> 0x10);
    iVar1 = (uVar7 + 0xc);
    uVar8 = (iVar1 - 0x16U);
    if ((0x15 < iVar1) && (!SBORROW2(iVar1,0x16))) {
      uVar8 = (iVar1 - 0x17U);
      if (iVar1 - 0x17U != 0x0 && 0x0 < (iVar1 - 0x16U)) {
        uVar8 = (iVar1 - 0x19U);
        if ((iVar1 + -0x18 < 1) ||
           (uVar8 = (iVar1 - 0x1aU), iVar1 - 0x1aU != 0x0 && 0x0 < (iVar1 - 0x19U))) goto LAB_1030_b064;
      }
      (uVar7 + 0x20) = 0;
    }
  }//
LAB_1030_b064:
  iStack12 = 0x6;
  loop {
    uVar3 = uVar8;
    uVar6 = uVar5;
    if (iStack12 == 0) {//
LAB_1030_b0fc:
      if ((uStack10 | uStack10) != 0) {
        uVar8 = struct_op_1030_73a8(uStack10,uStack10 | uStack10,uVar6);
        uVar2 = (uVar8 >> 0x10);
        iVar1 = (uVar8 + 0xc);
        if (((0x15 < iVar1) && (!SBORROW2(iVar1,0x16))) &&
           ((iVar1 == 0x17 || iVar1 + -0x16 < 0x1 || ((0x0 < iVar1 + -0x18 && (iVar1 + -0x19 < 0x2)))))) {
          (uVar8 + 0x20) = 0x1;
        }
      }
      return;
    }
    pass1_1030_b578(param_1);
    uVar6 = uVar5 | uVar3;
    if (uVar6 == 0) goto LAB_1030_b0fc;
    uStack10 = (astruct_419 *)CONCAT22(uVar5,uVar3);
    uVar8 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar5,uVar3),uVar3,uVar5);
    uVar6 = (uVar8 >> 0x10);
    iVar1 = (uVar8 + 0xc);
    pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x8)),
                    CONCAT22(uVar5,uVar3 + 0xc));
    if ((iVar1 == 0x18) || (iVar1 == 0x3f)) {
      pass1_1030_b142(param_1,uStack10);
    }
    BVar4 = pass1_1008_c6ae(_u16_1050_06e0,iVar1,0x40);
    uVar8 = BVar4;
    if (BVar4 != 0) {
      pass1_1030_b454(param_1,uStack10);
  // TODO: goto LAB_1030_b0fc;
    }
    iStack12 += -0x1;
    uVar5 = uVar6;
  } while( true );
}
