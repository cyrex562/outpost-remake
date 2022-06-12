
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_b0aa(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  let mut puVar1: *mut u32;
  code **ppcVar2;
  let mut iVar3: i16;
  let mut puVar4: *mut u32;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut uStack20: u32;

  uVar7 = (_PTR_LOOP_1050_4e74 >> 0x10);
  if ((_PTR_LOOP_1050_4e74 + param_4 * 0x6 + 0x4) == 0x0) {
    return;
  }
  if ((_PTR_LOOP_1050_4e74 + param_4 * 0x6 + 0x4) != -0x1) {
    if (PTR_LOOP_1050_4e78 == NULL) {
      iVar3 = param_4;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
      puVar1 = (iVar3 + 0xc);
      ppcVar2 = (code **)(*puVar1 + 0x10);
      puVar4 = puVar1;
      (**ppcVar2)();
      uVar6 = extraout_DX;
      for (uStack20 = 0x0; uStack20 < (puVar4 & 0xffff | extraout_DX << 0x10); uStack20 += 0x1) {
        uVar8 = pass1_1030_1d7c((puVar4 & 0xffff),uVar6,puVar1);
        uVar5 = (uVar8 >> 0x10);
        uVar6 = uVar5 | uVar8;
        if ((uVar6 != 0x0) && ((iVar3 = (uVar8 + 0xc), iVar3 == 0x2a || (iVar3 == 0x2b)))) {
          PTR_LOOP_1050_4e78 = (&PTR_LOOP_1050_0000 + 0x1);
          break;
        }
      }
      if (PTR_LOOP_1050_4e78 == NULL) {
        PTR_LOOP_1050_4e78 = (&PTR_LOOP_1050_0000 + 0x1);
        return;
      }
    }
    uVar6 = (_PTR_LOOP_1050_4e74 + param_4 * 0x6 + 0x4) - 0x1;
    pass1_1008_612e(uVar6,0x0,uVar6);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1020_b1ae(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,mut param_6: u32)

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

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6);
  iStack6 = param_1;
  uStack4 = param_2;
  puVar1 = pass1_1030_5b5c(param_1,param_2);
  local_c = *puVar1;
  uStack8 = (puVar1 + 0x4);
  pass1_1008_3e94(param_5,(u16 *)CONCAT22(0x1050,&local_10),CONCAT22(0x1050,&local_e));
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_14),CONCAT22(0x1050,&local_12))
  ;
  if ((((0xb < local_e) && (0xb < local_10)) && (local_e < local_12 + -0xb)) && (local_10 < local_14 + -0xb)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_b240(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,mut param_4: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  u8 bStack31;
  let mut local_a: u32;
  let mut uStack6: u32;

  puVar1 = &local_a;
  uVar6 = (param_4 >> 0x10);
  pass1_1030_64ce(puVar1,param_1,_PTR_LOOP_1050_5740,(u16 *)param_3,*(i32 *)(param_4 + 0x4),
                  CONCAT22(0x1050,puVar1));
  uStack6 = *puVar1;
  uVar5 = (puVar1 + 0x2);
  bStack31 = (u8)(uStack6 >> 0x18);
  uVar2 = bStack31;
  if (bStack31 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | uVar5 << 0x10);
    uVar7 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar5,uVar2),uVar2,uVar5);
    uVar4 = (uVar7 >> 0x10);
    uVar2 = uVar7;
    uVar5 = uVar4 | uVar2;
    if ((uVar5 != 0x0) && (uVar2 = (uVar2 + 0xc), 0x9 < uVar2)) {
      return;
    }
  }
  uVar3 = pass1_1020_b1ae(uVar2,uVar5,param_2,(param_2 >> 0x10),(u16 *)param_3,
                          (param_4 + 0x4));
  if (uVar3 == 0x0) {
    return;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1020_b2da(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,param_4: *mut u16,mut param_5: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  u8 *puVar4;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut in_stack_0000fe78: u16;
  astruct_19 **ppaVar7;
  let mut iStack28: i16;
  u8 local_1a [0x6];
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut piStack16: *mut i16;
  let mut piStack12: *mut i16;
  let mut local_8: u16;
  let mut local_6: u32;

  if (param_3 == 0x0) {
    uVar2 = 0x4e6a;
  }
  else {
    uVar2 = 0x4e6e;
  }
  piStack12 = CONCAT22(0x1050,uVar2);
  if (param_3 == 0x0) {
    uStack20 = 0x4e68;
  }
  else {
    uStack20 = 0x4e6c;
  }
  uStack18 = SUB42(&DAT_1050_1050,0x0);
  piStack16 = CONCAT22(0x1050,uStack20);
  do {
    if (param_3 == 0x0) {
      ppaVar7 = &PTR_1048_4230;
    }
    else {
      ppaVar7 = (astruct_19 **)0x10484236;
    }
    pass1_1008_3eb4((astruct_615 *)ppaVar7,(u16 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_6),
                    (u16 *)CONCAT22(0x1050,&local_6 + 0x2));
    iVar1 = *piStack12;
    if (iVar1 == 0x0) {
      local_6 = CONCAT22(local_6 + *piStack16,local_6 + -0x1);
    }
    else if (iVar1 == 0x1) {
      local_6 = CONCAT22(local_6 + -0x1,local_6 + *piStack16);
    }
    else if (iVar1 == 0x2) {
      local_6 = CONCAT22(local_6 - *piStack16,local_6 + -0x1);
    }
    puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_1a),local_8,local_6,(local_6 >> 0x10));
    uVar5 = (puVar6 >> 0x10);
    uVar2 = (param_5 >> 0x10);
    uVar3 = pass1_1020_b1ae(local_1a,uVar5,param_1,param_2,(u16 *)CONCAT22(0x1050,local_1a),
                            (param_5 + 0x4));
    if (uVar3 != 0x0) {
      puVar4 = local_1a;
      pass1_1020_b240(uVar5,CONCAT22(param_2,param_1),CONCAT22(0x1050,puVar4),param_5);
      if (puVar4 != NULL) {//
LAB_1020_b46e:
        pass1_1008_3e76(param_4,local_8,local_6,(local_6 >> 0x10));
        return;
      }
    }
    iVar1 = *piStack12;
    if (iVar1 == 0x0) {//
LAB_1020_b45e:
      local_6 = local_6 & 0xffff0000 | (local_6 + 0x2);
    }
    else if (iVar1 == 0x1) {
      local_6 = local_6 & 0xffff | (local_6 + 0x2) << 0x10;
    }
    else if (iVar1 == 0x2) goto LAB_1020_b45e;
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,local_1a),local_8,local_6,(local_6 >> 0x10));
    uVar3 = pass1_1020_b1ae(local_1a,uVar5,param_1,param_2,(u16 *)CONCAT22(0x1050,local_1a),
                            (param_5 + 0x4));
    if (uVar3 != 0x0) {
      puVar4 = local_1a;
      pass1_1020_b240(uVar5,CONCAT22(param_2,param_1),CONCAT22(0x1050,puVar4),param_5);
      if (puVar4 != NULL) goto LAB_1020_b46e;
    }
    iStack28 = *piStack12 + 0x1;
    if (0x2 < iStack28) {
      iStack28 = 0x0;
      *piStack16 = *piStack16 + 0x1;
    }
    *piStack12 = iStack28;
    pass1_1020_ac6e(in_stack_0000fe78,CONCAT22(param_2,param_1),param_3,*piStack16,iStack28);
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_b482(mut param_1: u32,u32 *param_2,mut param_3: u32)

{
  u8 *puVar1;
  astruct_92 **ppaVar2;
  let mut puVar3: *mut u32;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;
  let mut uVar8: u32;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut puVar11: *mut u32;
  let mut iStack46: i16;
  let mut local_2a: u32;
  let mut local_26: u16;
  let mut local_24: u32;
  let mut uStack32: u16;
  i32 lStack30;
  let mut uStack26: u32;
  astruct_92 *local_16;
  u8 local_4 [0x2];

  uVar8 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar5 = (uVar8 >> 0x10);
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_16),0x1,0x0,0x400);
  while( true ) {
    ppaVar2 = &local_16;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,ppaVar2));
    uStack26 = CONCAT22(uVar5,ppaVar2);
    uVar6 = uVar5 | ppaVar2;
    if (uVar6 == 0x0) {
      pass1_1020_b240(0x0,param_1,param_2,param_3);
      if (ppaVar2 != NULL) {
        lStack30 = *(i32 *)(param_3 + 0x4);
        local_24 = *param_2;
        uStack32 = (param_2 + 0x4);
        puVar7 = &local_2a;
        pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_24),(u16 *)CONCAT22(0x1050,puVar7),
                        (u16 *)CONCAT22(0x1050,&local_2a + 0x2),(u16 *)CONCAT22(0x1050,&local_26));
        pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x1,local_26 - 0x1);
        puVar3 = &local_24;
        uVar9 = param_1;
        uVar10 = (param_1 >> 0x10);
        pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
        if (puVar3 != NULL) {
          pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,(local_2a >> 0x10),local_26 - 0x1)
          ;
          puVar3 = &local_24;
          pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
          if (puVar3 != NULL) {
            pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a + 0x1,local_26 - 0x1);
            puVar3 = &local_24;
            pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
            if (puVar3 != NULL) {
              pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x1,local_26);
              puVar3 = &local_24;
              pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
              if (puVar3 != NULL) {
                pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a + 0x1,local_26);
                puVar3 = &local_24;
                pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                if (puVar3 != NULL) {
                  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a + 0x1,local_26 + 0x1);
                  puVar3 = &local_24;
                  pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                  if (puVar3 != NULL) {
                    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,(local_2a >> 0x10),
                                    local_26 + 0x1);
                    puVar3 = &local_24;
                    pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                    if (puVar3 != NULL) {
                      pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x1,
                                      local_26 + 0x1);
                      puVar3 = &local_24;
                      pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                      if (puVar3 != NULL) {
                        pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x2,
                                        local_26 - 0x2);
                        puVar3 = &local_24;
                        pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                        if (puVar3 != NULL) {
                          pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a + 0x2,
                                          local_26 - 0x2);
                          puVar3 = &local_24;
                          pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                          if (puVar3 != NULL) {
                            pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x2,
                                            local_26 + 0x2);
                            puVar3 = &local_24;
                            pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                            if (puVar3 != NULL) {
                              pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a + 0x2,
                                              local_26 + 0x2);
                              puVar3 = &local_24;
                              pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                              if (puVar3 != NULL) {
                                pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x1,
                                                local_26 + 0x2);
                                puVar3 = &local_24;
                                pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30);
                                if (puVar3 != NULL) {
                                  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),local_2a,local_2a - 0x1,
                                                  local_26 + 0x3);
                                  puVar3 = &local_24;
                                  pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),lStack30
                                                 );
                                  if (puVar3 != NULL) {
                                    iStack46 = 0x3;
                                    while( true ) {
                                      if (0x9 < iStack46) {
                                        return;
                                      }
                                      pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_24),0x0,local_2a - iStack46,
                                                      local_26);
                                      puVar3 = &local_24;
                                      pass1_1020_afc4(puVar7,uVar9,uVar10,(u16 *)CONCAT22(0x1050,puVar3),
                                                      lStack30);
                                      if (puVar3 == NULL) break;
                                      iStack46 += 0x1;
                                    }
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
      return;
    }
    uVar4 = (ppaVar2 + 0x8);
    puVar11 = param_2;
    uVar8 = param_3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4);
    puVar1 = local_4;
    pass1_1030_bcbc(puVar1,CONCAT22(uVar4,0x1050),CONCAT22(puVar11,uVar6),
                    (puVar11 >> 0x10),uVar8);
    if (puVar1 < 0x0) break;
    uVar5 = uVar6;
    if (puVar1 < 0x65) {
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_b872(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  u8 *puVar5;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u16;
  let mut uVar8: u16;
  u8 local_136 [0x124];
  let mut local_12: u32;
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  uVar8 = (param_2 >> 0x10);
  puVar6 = pass1_1030_5b5c(param_2,uVar8);
  local_8 = *puVar6;
  uStack4 = (puVar6 + 0x4);
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_c),CONCAT22(0x1050,&local_a));
  uVar1 = local_a - 0xa;
  pass1_1008_612e(uVar1,0xa,uVar1);
  uVar2 = local_c - 0xa;
  pass1_1008_612e(uVar2,0xa,uVar2);
  puVar7 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,&local_12),0x0,uVar2,uVar1);
  uVar2 = (puVar7 >> 0x10);
  while( true ) {
    puVar4 = &local_12;
    pass1_1020_b482(param_1,CONCAT22(0x1050,puVar4),param_2);
    if (puVar4 != NULL) break;
    uVar1 = local_a - 0xa;
    pass1_1008_612e(uVar1,0xa,uVar1);
    uVar3 = local_c - 0xa;
    pass1_1008_612e(uVar3,0xa,uVar3);
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_12),0x0,uVar3,uVar1);
  }
  struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,local_136),0x0,0xa,&local_12,&DAT_1050_1050,0x8000002,0x0,
                      (param_2 + 0x4));
  puVar5 = local_136;
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,puVar5));
  pass1_1020_b97e(puVar5,uVar2,param_1,(param_1 >> 0x10),0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_b97e(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16)

{
  let mut uVar1: u32;
  let mut local_e: i16;
  let mut local_c: u16;
  let mut iStack10: i16;
  let mut uStack8: u16;
  let mut uStack6: u32;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
  _PTR_LOOP_1050_4e70 = CONCAT22(param_2,param_1);
  uVar1 = (param_1 + 0x10);
  uStack6 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  iStack10 = uVar1;
  uStack8 = param_2;
  pass1_1008_3f62((u16 *)&PTR_1048_4230,(u16 *)CONCAT22(param_2,iStack10 + 0xc));
  pass1_1008_3e94((u16 *)&PTR_1048_4230,(u16 *)CONCAT22(0x1050,&local_e),CONCAT22(0x1050,&local_c));
  if (param_5 == 0x0) {
    pass1_1008_3e76((u16 *)&PTR_1048_4230,0x0,local_e + 0x1,local_c - 0x1);
    pass1_1008_3e94((u16 *)&PTR_1048_4230,(u16 *)CONCAT22(0x1050,&local_e),CONCAT22(0x1050,&local_c));
  }
  pass1_1008_3e76((u16 *)0x10484236,0x1,local_e - 0x2,local_c);
  return;
}
pub fn pass1_1020_ba2b(void)

{
  init_globals_1020_96d4();
  pass1_1020_a426();
  return;
}
pub fn pass1_1020_ba3e(param_1: *mut astruct_172,mut param_2: u16 ,mut param_3: i16)

{
  astruct_172 *iVar1;
  astruct_172 *uVar1;

  uVar1 = (astruct_172 *)(param_1 >> 0x10);
  iVar1 = (astruct_172 *)param_1;
  param_1 = 0x0;
  iVar1->field2_0x4 = 0x0;
  iVar1->field3_0x6 = param_3;
  iVar1->field4_0x8 = param_2;
  if (iVar1->field3_0x6 == 0x0) {
    iVar1->field3_0x6 = 0x5;
  }
  pass1_1020_bcc4(param_1);
  return;
}
pub fn fn_ptr_1020_ba7e(u32 *param_1)

{
  fn_ptr_1000_17ce(*param_1);
  return;
}
pub fn pass1_1020_ba94(i32 *param_1)

{
  let mut puVar1: *mut u16;
  let mut uStack8: u16;

  if (*param_1 == 0x0) {
    return;
  }
  uStack8 = 0x0;
  while( true ) {
    puVar1 = (u16 *)(param_1 + 0x4);
    if (*puVar1 < uStack8 || *puVar1 == uStack8) break;
    uStack8 += 0x1;
  }
  return;
}



pub fn pass1_1020_bae6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32) -> u32

{
  let mut puStack6: *mut u16;

  pass1_1020_bc92((u16 *)CONCAT22(param_4,param_3),(param_4 >> 0x10));
  puStack6 = (u16 *)CONCAT22(param_2,param_1);
  if ((param_2 | param_1) != 0x0) {
    return CONCAT22((param_1 + 0x2),*puStack6);
  }
  return 0x0;
}
pub fn pass1_1020_bb16(u32 *param_1,u32 *param_2,param_3: *mut u16,mut param_4: u16 )

{
  if ((param_1 + 0x4) < param_4) {
    *param_3 = 0x0;
    *param_2 = 0x0;
    return;
  }
  *param_3 = (param_4 * 0x6 + *param_1 + 0x4);
  *param_2 = (*param_1 + param_4 * 0x6);
  return;
}
pub fn pass1_1020_bb70(i32 *param_1,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_AX: u16;
  let mut in_DX: u16;

  pass1_1020_bba4((astruct_172 *)param_1,0x1,param_2,param_3,(param_3 >> 0x10),in_AX,in_DX);
  return;
}
pub fn pass1_1020_bb8a(i32 *param_1,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_AX: u16;
  let mut in_DX: u16;

  pass1_1020_bba4((astruct_172 *)param_1,0x0,param_2,param_3,(param_3 >> 0x10),in_AX,in_DX);
  return;
}



BOOL16 pass1_1020_bba4(param_1: *mut astruct_172,mut param_2: i16,mut param_3: u16 ,mut param_4: i16,mut param_5: u16 ,param_6: *mut u16,
                      mut param_7: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut bVar3: bool;
  let mut puStack6: *mut u16;

  pass1_1020_bc92(&param_1->field0_0x0,param_5);
  puStack6 = (u16 *)CONCAT22(param_7,param_6);
  uVar1 = param_7 | param_6;
  if (uVar1 == 0x0) {
    pass1_1020_bc92(&param_1->field0_0x0,0x0);
    uVar2 = uVar1 | param_6;
    if (uVar2 == 0x0) {
      pass1_1020_bcc4(param_1);
      pass1_1020_bc92(&param_1->field0_0x0,0x0);
      if ((uVar2 | param_6) == 0x0) {
        return 0x0;
      }
      param_6[0x2] = param_5;
      uVar1 = uVar2;
    }
    else {
      param_6[0x2] = param_5;
    }
    if (param_2 != 0x0) {
      uVar2 = *param_6;
      bVar3 = CARRY2(uVar2,param_3);
      param_3 = uVar2 + param_3;
      param_4 = param_6[0x1] + param_4 + bVar3;
    }
    *param_6 = param_3;
    param_6[0x1] = param_4;
    pass1_1020_bc72(param_1);
  }
  else {
    if (param_2 != 0x0) {
      bVar3 = CARRY2(*puStack6,param_3);
      param_3 = *puStack6 + param_3;
      param_4 = param_6[0x1] + param_4 + bVar3;
    }
    *param_6 = param_3;
    param_6[0x1] = param_4;
  }
  return 0x1;
}
pub fn pass1_1020_bc72(param_1: *mut astruct_172)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0x2);
  pass1_1000_4aea(param_1->field0_0x0,uVar1,(uVar1 >> 0x10),0x6,0xbd6c);
  return;
}
pub fn pass1_1020_bc92(param_1: *mut u16,mut param_2: u16 )

{
  let mut uVar1: u32;
  u8 local_c [0x4];
  let mut uStack8: u16;

  uStack8 = param_2;
  uVar1 = (param_1 + 0x2);
  pass1_1000_49c6(local_c,&DAT_1050_1050,*param_1,uVar1,(uVar1 >> 0x10),0x6,
                  0xbd6c);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_bcc4(param_1: *mut astruct_172)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_EDX: u32;
  StructD *pSVar4;
  astruct_172 *pstruct172_5;
  astruct_172 *pstruct172_4;
  i32 lVar5;
  let mut uStack12: u32;

  pstruct172_4 = (astruct_172 *)(param_1 >> 0x10);
  pstruct172_5 = (astruct_172 *)param_1;
  if (pstruct172_5->field2_0x4 == 0x0) {
    pSVar4 = (StructD *)(in_EDX & 0xffff0000);
    uVar2 = pstruct172_5->field3_0x6;
  }
  else {
    uVar3 = pstruct172_5->field2_0x4;
    puVar1 = &pstruct172_5->field4_0x8;
    uVar2 = uVar3 + *puVar1;
    pSVar4 = (StructD *)(in_EDX & 0xffff0000 | CARRY2(uVar3,*puVar1));
  }
  if (pSVar4 == 0x0) {
    if (*(i32 *)param_1 == 0x0) {
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar4);
        PTR_LOOP_1050_5f2e = pSVar4;
      }
      else {
      }
      uVar3 = fn_ptr_op_1000_1708(uVar2 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
    }
    else {
      lVar5 = pass1_1000_0ed4(0x1,uVar2 * 0x6,0x0,(astruct_172 *)param_1,
                              (astruct_172 *)(param_1 >> 0x10));
      PTR_LOOP_1050_5f2e = (lVar5 >> 0x10);
      uVar3 = lVar5;
    }
    uStack12 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    if ((PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
      pstruct172_5->field2_0x4 = uVar2;
      param_1 = uStack12;
      pass1_1020_bc72((astruct_172 *)(param_1 & 0xffff | ZEXT24(pstruct172_4) << 0x10));
    }
  }
  return;
}



i16 pass1_1020_bd6c(mut param_1: u32,mut param_2: u32)

{
  return (param_1 + 0x4) - (param_2 + 0x4);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1020_bd80(mut param_1: u16 )

{
  let mut pcVar1: *mut c_char;
  let mut uVar2: u16;
  let mut uStack6: u16;

  switch(param_1) {
  case 0x1:
  case 0x6:
    uVar2 = 0x427;
    break;
  case 0x2:
    uVar2 = 0x428;
    break;
  case 0x3:
  case 0x7:
    uVar2 = 0x429;
    break;
  case 0x4:
  case 0x8:
    uVar2 = 0x425;
    break;
  case 0x5:
  case 0x9:
    uVar2 = 0x426;
    break;
  case 0xa:
    uVar2 = 0x402;
    break;
  case 0xb:
  case 0x37:
    uVar2 = 0x418;
    break;
  case 0xc:
  case 0x35:
  case 0x36:
    uVar2 = 0x42a;
    break;
  case 0xd:
    uVar2 = 0x5f7;
    break;
  case 0xe:
    uVar2 = 0x5f6;
    break;
  case 0xf:
    uVar2 = 0x403;
    break;
  case 0x10:
    uVar2 = 0x5f8;
    break;
  case 0x11:
    uVar2 = 0x404;
    break;
  case 0x12:
    uVar2 = 0x405;
    break;
  case 0x13:
  case 0x14:
  case 0x15:
    uVar2 = 0x406;
    break;
  case 0x16:
  case 0x19:
    uVar2 = 0x5f9;
    break;
  case 0x17:
  case 0x1a:
    uVar2 = 0x5fa;
    break;
  case 0x18:
    uVar2 = 0x5fb;
    break;
  case 0x1b:
  case 0x1c:
  case 0x1d:
    uVar2 = 0x408;
    break;
  case 0x1e:
  case 0x1f:
  case 0x20:
    uVar2 = 0x409;
    break;
  case 0x21:
    uVar2 = 0x42c;
    break;
  case 0x22:
  case 0x23:
  case 0x24:
    uVar2 = 0x40a;
    break;
  case 0x25:
  case 0x26:
  case 0x27:
    uVar2 = 0x40b;
    break;
  case 0x28:
  case 0x29:
    uVar2 = 0x40c;
    break;
  case 0x2a:
  case 0x2b:
    uVar2 = 0x40d;
    break;
  case 0x2c:
    uVar2 = 0x40e;
    break;
  case 0x2d:
  case 0x2e:
    uVar2 = 0x40f;
    break;
  case 0x2f:
  case 0x30:
    uVar2 = 0x410;
    break;
  case 0x31:
  case 0x32:
    uVar2 = 0x411;
    break;
  case 0x33:
  case 0x34:
    uVar2 = 0x416;
    break;
  case 0x38:
  case 0x39:
    uVar2 = 0x5fc;
    break;
  case 0x3a:
  case 0x3b:
    uVar2 = 0x419;
    break;
  case 0x3c:
  case 0x3d:
    uVar2 = 0x5fd;
    break;
  case 0x3e:
    uVar2 = 0x5fe;
    break;
  case 0x3f:
    uVar2 = 0x5ff;
    break;
  case 0x40:
    uVar2 = 0x600;
    break;
  case 0x41:
    uVar2 = 0x601;
    break;
  case 0x42:
  case 0x46:
  case 0x6b:
    uVar2 = 0x602;
    break;
  case 0x43:
    uStack6 = s_bidLRoadConst_1050_4e7a;
    return uStack6;
  case 0x44:
    uStack6 = s_bidRRoadConst_1050_4e88;
    return uStack6;
  case 0x45:
    uStack6 = s_bidXRoadConst_1050_4e96;
    return uStack6;
  case 0x47:
    uVar2 = 0x42b;
    break;
  case 0x48:
  case 0x49:
  case 0x4a:
  case 0x70:
  case 0x71:
  case 0x72:
    uVar2 = 0x603;
    break;
  case 0x4b:
    uVar2 = 0x42d;
    break;
  case 0x4c:
    uVar2 = 0x604;
    break;
  case 0x4d:
    uVar2 = 0x605;
    break;
  case 0x4e:
    uVar2 = 0x606;
    break;
  case 0x4f:
  case 0x50:
  case 0x51:
    uVar2 = 0x41a;
    break;
  case 0x52:
  case 0x53:
    uVar2 = 0x41b;
    break;
  case 0x54:
  case 0x55:
  case 0x56:
    uVar2 = 0x41d;
    break;
  case 0x57:
  case 0x58:
  case 0x59:
    uVar2 = 0x41e;
    break;
  case 0x5a:
    uVar2 = 0x41f;
    break;
  case 0x5b:
  case 0x5c:
    uVar2 = 0x421;
    break;
  case 0x5d:
  case 0x5e:
  case 0x5f:
    uVar2 = 0x420;
    break;
  case 0x60:
  case 0x61:
    uVar2 = 0x607;
    break;
  case 0x62:
  case 0x63:
    uVar2 = 0x608;
    break;
  case 0x64:
    uVar2 = 0x609;
    break;
  case 0x65:
    uVar2 = 0x422;
    break;
  case 0x66:
  case 0x67:
    uVar2 = 0x423;
    break;
  case 0x68:
  case 0x69:
    uVar2 = 0x60a;
    break;
  case 0x6a:
    uVar2 = 0x60b;
    break;
  case 0x6c:
  case 0x6d:
    uVar2 = 0x41c;
    break;
  case 0x6e:
    uVar2 = 0x60c;
    break;
  case 0x6f:
    uVar2 = 0x60d;
    break;
  case 0x73:
  case 0x77:
    uVar2 = 0x415;
    break;
  case 0x74:
  case 0x78:
  case 0x79:
    uVar2 = 0x412;
    break;
  case 0x75:
    uVar2 = 0x413;
    break;
  case 0x76:
    uVar2 = 0x414;
    break;
  case 0x7a:
    uVar2 = 0x60e;
    break;
  case 0x7b:
    uVar2 = 0x60f;
    break;
  case 0x7c:
    uVar2 = 0x610;
    break;
  case 0x7d:
    uVar2 = 0x611;
    break;
  case 0x7e:
    uVar2 = 0x612;
    break;
  case 0x7f:
    uVar2 = 0x613;
    break;
  case 0x80:
    uVar2 = 0x614;
    break;
  case 0x81:
    uVar2 = 0x615;
    break;
  case 0x82:
    uVar2 = 0x616;
    break;
  case 0x83:
    uVar2 = 0x617;
    break;
  case 0x84:
    uVar2 = 0x618;
    break;
  case 0x85:
    uVar2 = 0x619;
    break;
  case 0x86:
    uVar2 = 0x61a;
    break;
  case 0x87:
    uVar2 = 0x61b;
    break;
  case 0x88:
    uVar2 = 0x61c;
    break;
  case 0x89:
    uVar2 = 0x61d;
    break;
  default:
    uVar2 = 0x424;
  }
  pcVar1 = load_string_1010_847e(_u16_1050_14cc,uVar2);
  return pcVar1;
}
