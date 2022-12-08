




// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_d24a(param_1: u8,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,param_5: *mut astruct_104)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut pcVar3: *mut c_char;
  let mut puVar4: *mut u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut puVar7: *mut u16;
  let mut uVar8: u16;
  let mut unaff_SI: i16;
  iVar9: *mut astruct_104;
  let mut uVar9: u16;
  let mut lVar10: i32;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uStack320: u16;
  let mut lStack318: i32;
  u16 *local_13a [0x4a];
  let mut local_a6: u32;
  let mut iStack18: i16;
  let mut uStack16: u32;
  let mut pcStack12: *mut c_char;
  let mut uStack10: u16;
  let mut puStack8: *mut u16;
  let mut iStack4: i16;

  iStack4 = 0;
  loop {
    uVar9 = (param_5 >> 0x10);
    iVar9 = param_5;
    uVar2 = iVar9.field1_0x2;
    (iStack4 * 0xa + uVar2 + 0x4) = (iStack4 * 0x2 + param_4);
    iStack4 += 0x1;
  } while (iStack4 < 0x8);
  puStack8 = iVar9.field1_0x2;
  iStack4 = 0;
  loop {
    uVar1 = (iVar9 + 1);
    pcVar3 = pass1_1010_b038(param_3,uVar1,(uVar1 >> 0x10),
                             (puStack8 + 0x4),unaff_SI);
    string_1040_a626(param_2,puStack8,CONCAT22(param_2,pcVar3));
    iStack4 += 0x1;
    puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
  } while (iStack4 < 0x8);
  uVar11 = param_3;
  uVar12 = (param_3 >> 0x10);
  struct_1010_dd5e(uVar11,uVar12,(iVar9 + 1));
  puVar6 = (param_2 | pcVar3);
  if (puVar6.is_null() == false) {
    pcStack12 = pcVar3;
    uStack10 = param_2;
    pass1_1010_e8f6(pcVar3,puVar6,uVar11,uVar12,(iVar9 + 1));
    uStack16 = CONCAT22(puVar6,pcVar3);
    iStack18 = 0;
    pass1_1000_4906(CONCAT22(0x1050,&local_a6),NULL,0x94);
    puVar4 = pass1_1000_4906(CONCAT22(0x1050,local_13a),NULL,0x94);
    lStack318 = 0;
    for (uStack320 = 0x1; uStack320 < 0x25; uStack320 += 1) {
      lVar10 = pass1_1030_7c28(puVar4,puVar6,uStack16,uStack320);
      puVar7 = (lVar10 >> 0x10);
      puVar4 = lVar10;
      puVar6 = (puVar7 | puVar4);
      if (lVar10 != 0) {
        if (iStack18 == 0) {
          iStack18 = 0x1;
        }
        pcVar3 = string_1020_c0d8(uStack320);
        uVar8 = puVar6 | pcVar3;
        if (uVar8 == 0) {
          unk_str_op_1000_3d3e((&local_a6)[lStack318],s_Null_Ptr_1050_3917);
        }
        else {
          uVar5 = str_op_1008_60e8(uVar8,CONCAT22(puVar6,pcVar3));
          (&local_a6 + lStack318) = uVar5;
          (&local_a6 + lStack318 * 0x4 + 0x2) = uVar8;
        }
        local_13a[lStack318 * 0x2] = puVar4;
        local_13a[lStack318 * 0x2 + 0x1] = puVar7;
        lStack318 += 0x1;
        puVar6 = puVar7;
      }
    }
    uVar8 = pass1_1010_db2e(uVar11,uVar12,0x8,CONCAT22(0x1050,&local_a6),CONCAT22(0x1050,local_13a),param_4,param_5);
    if (iStack18 != 0) {
      (&iVar9[0x3].field1_0x2 + 0x2) = 0x1;
    }
    while (lStack318 != 0) {
      lStack318 = (lStack318 + -1);
      fn_ptr_1000_17ce((&local_a6)[lStack318]);
      lStack318 = lStack318 + -0x1;
    }
    pass1_1010_dc36(uVar11,uVar12,uVar8,param_4,param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_d448(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u8,mut param_4: u32,param_5: *mut astruct_104,mut param_6: i16
                    )

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u32;
  let mut uVar4: u32;
  let mut puVar5: *mut u16;
  let mut pcVar6: *mut c_char;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  iVar10: *mut astruct_104;
  uVar10: *mut astruct_104;
  let mut uVar11: u32;
  let mut uVar12: u16;
  let mut local_40c: u16;
  let mut uStack1034: u32;
  let mut uStack1030: u32;
  let mut local_402: [u8;0x400] = [0;0x400];

  uVar10 = (param_5 >> 0x10);
  iVar10 = param_5;
  uStack1030 = struct_op_1030_73a8((iVar10 + 1),param_1,param_2);
  uVar8 = (uStack1030 >> 0x10);
  uVar1 = uStack1030;
  if ((uVar8 | uVar1) != 0) {
    uStack1034 = (uVar1 + 0x20);
    uVar1 = (uVar1 + 0x22);
    if ((uVar1 | uStack1034) != 0) {
      local_40c = 0;
      puVar5 = &local_40c;
      uVar12 = (param_3 >> 0x10);
      pass1_1010_d984(param_3,uVar12,CONCAT22(0x1050,puVar5),0x3,
                      uStack1034 & 0xffff | uVar1 << 0x10,&PTR_DAT_1050_1805_1050_368e,param_5);
      puVar2 = iVar10.field1_0x2;
      uVar9 = (&iVar10.field1_0x2 + 2);
      (puVar2 + 0x4) = PTR_DAT_1050_1805_1050_368e;
      uVar3 = (iVar10 + 1);
      pcVar6 = pass1_1010_b038(param_3,uVar3,(uVar3 >> 0x10),(puVar2 + 0x4),
                               param_6);
      unk_str_op_1000_3d3e(CONCAT22(0x1050,local_402),CONCAT22(uVar9,pcVar6));
      string_1040_a626(uVar9,puVar2,CONCAT22(0x1050,local_402));
      uVar4 = iVar10.field1_0x2;
      uVar9 = (&iVar10.field1_0x2 + 2);
      iVar7 = uVar4;
      (iVar7 + 0xe) = PTR_DAT_1050_1822_1050_3690;
      sys_1000_3f9c(CONCAT22(0x1050,local_402),0x10503920,local_40c);
      string_1040_a626(uVar9,(uVar4 & 0xffff0000 | (iVar7 + 0xa)),CONCAT22(0x1050,local_402));
      uVar4 = iVar10.field1_0x2;
      iVar7 = uVar4;
      (iVar7 + 0x18) = PTR_DAT_1050_1823_1050_3692;
      uVar11 = pass1_1028_62c8(uStack1030);
      uVar9 = (uVar11 >> 0x10);
      sys_1000_3f9c(CONCAT22(0x1050,local_402),0x10503923,uVar11);
      string_1040_a626(uVar9,(uVar4 & 0xffff0000 | (iVar7 + 0x14)),CONCAT22(0x1050,local_402));
      pass1_1010_dc36(param_3,uVar12,puVar5,param_4,param_5);
    }
  }
  return;
}
pub fn pass1_1010_d5ae(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u8,mut param_4: u32,param_5: *mut astruct_104,mut param_6: i16
                    )

{
  let mut puVar1: *mut u16;
  let mut uVar2: u32;
  let mut puVar3: *mut u8;
  let mut puVar4: *mut u16;
  let mut pcVar5: *mut c_char;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut in_register_0000000a: u16;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut uVar12: u32;
  let mut uVar13: u16;
  let mut local_40c: u16;
  let mut uStack1034: u16;
  let mut uStack1032: u16;
  paStack1030: *mut astruct_15;
  let mut string_402: [u8;0x400] = [0;0x400];
  let mut uVar9: u32;

  uVar11 = (param_5 >> 0x10);
  iVar10 = param_5;
  paStack1030 = struct_op_1030_73a8((iVar10 + 0x6),param_1,param_2);
  uStack1034 = paStack1030;
  uVar7 = (paStack1030 >> 0x10) | uStack1034;
  uVar9 = CONCAT22(in_register_0000000a,uVar7);
  if (uVar7 != 0) {
    pass1_1028_45fe(uStack1034,paStack1030,uVar9);
    uStack1032 = uVar9;
    if ((uStack1032 | uStack1034) != 0) {
      local_40c = 0;
      puVar4 = &local_40c;
      uVar13 = (param_3 >> 0x10);
      pass1_1010_d984(param_3,uVar13,CONCAT22(0x1050,puVar4),0x3,CONCAT22(uStack1032,uStack1034),
                      &PTR_DAT_1050_1805_1050_3706,param_5);
      puVar1 = (iVar10 + 2);
      uVar8 = (iVar10 + 0x4);
      (puVar1 + 0x4) = PTR_DAT_1050_1805_1050_3706;
      uVar9 = (iVar10 + 0x6);
      pcVar5 = pass1_1010_b038(param_3,uVar9,(uVar9 >> 0x10),(puVar1 + 0x4),
                               param_6);
      unk_str_op_1000_3d3e(CONCAT22(0x1050,string_402),CONCAT22(uVar8,pcVar5));
      string_1040_a626(uVar8,puVar1,CONCAT22(0x1050,string_402));
      uVar2 = (iVar10 + 2);
      uVar8 = (iVar10 + 0x4);
      iVar6 = uVar2;
      (iVar6 + 0xe) = PTR_DAT_1050_1822_1050_3708;
      sys_1000_3f9c(CONCAT22(0x1050,string_402),s__u_1050_3926,local_40c);
      string_1040_a626(uVar8,(uVar2 & 0xffff0000 | (iVar6 + 0xa)),CONCAT22(0x1050,string_402));
      puVar3 = PTR_DAT_1050_1823_1050_370a;
      uVar2 = (iVar10 + 2);
      iVar10 = (iVar10 + 0x4);
      iVar6 = uVar2;
      (iVar6 + 0x18) = PTR_DAT_1050_1823_1050_370a;
      uVar12 = pass1_1028_45e2(puVar3,iVar10,paStack1030);
      uVar8 = (uVar12 >> 0x10);
      sys_1000_3f9c(CONCAT22(0x1050,string_402),0x10503929,uVar12);
      string_1040_a626(uVar8,(uVar2 & 0xffff0000 | (iVar6 + 0x14)),CONCAT22(0x1050,string_402))
      ;
      pass1_1010_dc36(param_3,uVar13,puVar4,param_4,param_5);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_d710(param_1: u8,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,param_5: *mut astruct_104)

{
  let mut lVar1: i32;
  let mut uVar2: u32;
  let mut pcVar3: *mut c_char;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut unaff_SI: i16;
  let mut iVar8: i16;
  iVar9: *mut astruct_104;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u32;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut uStack322: u16;
  let mut iStack316: i16;
  let mut iStack314: i16;
  let mut iStack312: i16;
  let mut local_136: [u16;0x4a] = [0;0x4a];
  let mut local_a2: u32;
  let mut iStack14: i16;
  let mut uStack12: u32;
  let mut puStack8: *mut u16;
  let mut iStack4: i16;

  iStack4 = 0;
  loop {
    uVar9 = (param_4 >> 0x10);
    iVar8 = param_4;
    uVar10 = (param_5 >> 0x10);
    iVar9 = param_5;
    uVar2 = iVar9.field1_0x2;
    (iStack4 * 0xa + uVar2 + 0x4) = (iStack4 * 0x2 + iVar8);
    iStack4 += 0x1;
  } while (iStack4 < 0x4);
  puStack8 = iVar9.field1_0x2;
  iStack4 = 0;
  loop {
    uVar12 = (iVar9 + 1);
    pcVar3 = pass1_1010_b038(param_3,uVar12,(uVar12 >> 0x10),
                             (puStack8 + 0x4),unaff_SI);
    string_1040_a626(param_2,puStack8,CONCAT22(param_2,pcVar3));
    iStack4 += 0x1;
    puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
  } while (iStack4 < 0x4);
  uVar13 = param_3;
  uVar14 = (param_3 >> 0x10);
  struct_1010_dd5e(uVar13,uVar14,(iVar9 + 1));
  uStack12 = CONCAT22(param_2,pcVar3);
  uVar6 = param_2 | pcVar3;
  if (uVar6 != 0) {
    iStack14 = 0;
    pass1_1000_4906(CONCAT22(0x1050,&local_a2),NULL,0x94);
    pass1_1000_4906(CONCAT22(0x1050,local_136),NULL,0x94);
    iStack314 = 0;
    iStack312 = 0;
    iStack316 = 0;
    uVar12 = (iVar9 + 1);
    lVar1 = (uVar12 + 0x26);
    for (uStack322 = 0x1; uStack322 < 0x25; uStack322 += 1) {
      if ((uStack322 * 0x4 + uStack12) != 0) {
        if (iStack14 == 0) {
          iStack14 = 0x1;
        }
        pcVar3 = string_1020_c0d8(uStack322);
        uVar7 = uVar6 | pcVar3;
        if (uVar7 == 0) {
          unk_str_op_1000_3d3e((&local_a2)[iStack312],s_Null_Ptr_1050_392c);
        }
        else {
          uVar5 = str_op_1008_60e8(uVar7,CONCAT22(uVar6,pcVar3));
          (&local_a2 + iStack312) = uVar5;
          (&local_a2 + iStack312 * 0x4 + 0x2) = uVar7;
        }
        uVar11 = (uStack12 >> 0x10);
        uVar7 = (uStack322 * 0x4 + uStack12);
        uVar6 = (uStack322 * 0x4 + uStack12 + 2);
        local_136[iStack312 * 0x2] = uVar7;
        local_136[iStack312 * 0x2 + 0x1] = uVar6;
        iStack312 += 0x1;
        if (lVar1 == 0) {
          iVar4 = 0;
        }
        else {
          uVar12 = pass1_1020_bae6(uVar7,uVar6,lVar1,CONCAT22(uStack322,(lVar1 >> 0x10)));
          uVar6 = (uVar12 >> 0x10);
          iVar4 = uVar12;
        }
        if (iVar4 == 0) {
          iStack316 += 0x2;
        }
        else {
          (uVar13 + iStack314 * 0x2 + 0xa4) = (iVar8 + iStack316 * 0x2 + 0x8);
          (uVar13 + (iStack314 + 1) * 0x2 + 0xa4) =
               (iVar8 + (iStack316 + 1) * 0x2 + 0x8);
          iStack316 += 0x2;
          iStack314 += 0x2;
        }
      }
    }
    uVar6 = pass1_1010_db2e(uVar13,uVar14,0x4,CONCAT22(0x1050,&local_a2),CONCAT22(0x1050,local_136),param_4,param_5);
    if (iStack14 != 0) {
      (&iVar9[0x3].field1_0x2 + 0x2) = 0x1;
    }
    while (iStack312 != 0) {
      fn_ptr_1000_17ce((&local_a2)[iStack312 + -0x1]);
      iStack312 = iStack312 + -0x1;
    }
    pass1_1010_dc36(uVar13,uVar14,uVar6,param_4,param_5);
  }
  return;
}
pub fn pass1_1010_d984(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut i16,mut param_4: i16,mut param_5: u32,mut param_6: u32,mut param_7: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u8;
  let mut pcVar3: *mut c_char;
  let mut iVar4: i16;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut local_418: [u8;0x400] = [0;0x400];
  let mut uStack24: u16;
  let mut pcStack22: *mut c_char;
  let mut uStack18: u16;
  let mut uStack16: u32;
  let mut local_c: [u8;0x8] = [0;0x8];
  let mut iStack4: i16;

  iStack4 = param_4;
  pass1_1008_5784(CONCAT22(0x1050,local_c),param_5);
  loop {
    puVar2 = local_c;
    pass1_1008_5b12(CONCAT22(0x1050,puVar2));
    uStack16 = CONCAT22(extraout_DX,puVar2);
    uVar5 = extraout_DX | puVar2;
    if (uVar5 == 0) {
      return;
    }
    uStack18 = (puVar2 + 0xa);
    pcStack22 = NULL;
    if ((puVar2 + 0x4) == 0) {
      if ((puVar2 + 0x6) == 0) {
        if ((puVar2 + 0x8) == 0) {
          return;
        }
        pcVar3 = string_op_1020_c2f8((puVar2 + 0x8));
      }
      else {
        pcVar3 = string_op_1020_c222((puVar2 + 0x6));
      }
    }
    else {
      pcVar3 = string_1020_c0d8((puVar2 + 0x4));
    }
    pcStack22 = CONCAT22(uVar5,pcVar3);
    uStack24 = (uStack16 + 0xc);
    *param_3 = *param_3 + uStack24;
    uVar9 = (param_7 >> 0x10);
    iVar7 = param_7;
    uVar1 = (iVar7 + 0x4);
    iVar4 = (iVar7 + 0x2) + iStack4 * 0xa;
    uVar10 = (param_6 >> 0x10);
    iVar8 = param_6;
    (iVar4 + 0x4) = (iStack4 * 0x2 + iVar8);
    uVar6 = uVar1;
    sys_1000_3f9c(CONCAT22(0x1050,local_418),0x10503935,uStack18);
    string_1040_a626(uVar6,CONCAT22(uVar1,iVar4),CONCAT22(0x1050,local_418));
    uVar1 = (iVar7 + 0x4);
    iStack4 += 0x1;
    iVar4 = (iVar7 + 0x2) + iStack4 * 0xa;
    (iVar4 + 0x4) = (iStack4 * 0x2 + iVar8);
    uVar6 = uVar1;
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_418),pcStack22);
    string_1040_a626(uVar6,CONCAT22(uVar1,iVar4),CONCAT22(0x1050,local_418));
    iVar4 = (iStack4 + 1) * 0xa + (iVar7 + 2);
    uVar1 = (iVar7 + 0x4);
    (iVar4 + 0x4) = ((iStack4 + 1) * 0x2 + iVar8);
    iStack4 += 0x2;
    uVar6 = uVar1;
    sys_1000_3f9c(CONCAT22(0x1050,local_418),0x10503938,uStack24);
    string_1040_a626(uVar6,CONCAT22(uVar1,iVar4),CONCAT22(0x1050,local_418));
  } while( true );
}



u16 pass1_1010_db2e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: u32,mut param_6: u32,
                    param_7: *mut astruct_104)

{
  let mut uVar1: u16;
  iVar2: *mut astruct_493;
  let mut iVar3: i16;
  let mut uVar4: u16;
  iVar4: *mut astruct_104;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uStack94: u16;
  let mut iStack92: i16;
  let mut uStack90: u16;
  let mut puStack88: *mut u16;
  let mut local_54: [u8;0x52] = [0;0x52];

  uStack94 = param_3;
  uStack90 = param_3;
  iStack92 = 0;
  loop {
    uVar7 = (param_7 >> 0x10);
    iVar4 = param_7;
    if (param_7.field0_0x0 - 0x1 < uStack94) {
      return uStack94;
    }
    uVar1 = (&iVar4.field1_0x2 + 2);
    iVar2 = (&iVar4.field1_0x2 + uStack94 * 0xa);
    uVar5 = (param_5 >> 0x10);
    uVar6 = (param_4 >> 0x10);
    if (((iStack92 * 0x4 + param_5) == 0) && ((iStack92 * 0x4 + param_4) == 0)) break;
    uVar4 = uVar1;
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_54),*(iStack92 * 0x4 + param_4));
    uVar6 = (param_6 >> 0x10);
    iVar2.field4_0x4 = (uStack90 * 0x2 + param_6);
    string_1040_a626(uVar4,CONCAT22(uVar1,iVar2),CONCAT22(0x1050,local_54));
    sys_1000_3f9c(CONCAT22(0x1050,local_54),0x1050393b,
                  (param_5 + iStack92 * 0x4));
    uVar1 = (&iVar4.field1_0x2 + 2);
    iVar3 = &iVar4.field1_0x2 + (uStack94 + 1) * 0xa;
    puStack88 = CONCAT22(uVar1,iVar3);
    (iVar3 + 0x4) = ((uStack90 + 1) * 0x2 + param_6);
    string_1040_a626(uVar1,puStack88,CONCAT22(0x1050,local_54));
    uStack94 += 0x2;
    uStack90 += 0x2;
    iStack92 += 0x1;
  }
  return uStack94;
}
pub fn pass1_1010_dc36(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,param_5: *mut astruct_104)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut puVar6: *mut u32;
  let mut uVar7: u16;
  let mut uStack90: u16;
  let mut string_54: *mut c_char;
  let mut local_52: [u32;0x14] = [0;0x14];

  string_54 = PTR_s_New_failed_in_Op_Op_1050_0020_1050_393f;
  puVar6 = local_52;
  for (iVar4 = 0x13; iVar4 != 0; iVar4 += -1) {
    puVar1 = puVar6;
    puVar6 = puVar6 + 1;
    *puVar1 = 0;
  }
  puVar6 = 0;
  (puVar6 + 0x2) = 0;
  uStack90 = param_3;
  loop {
    uVar7 = (param_5 >> 0x10);
    if (param_5.field0_0x0 < uStack90 || param_5.field0_0x0 == uStack90) break;
    uVar3 = (param_5 + 2);
    uVar2 = (param_5 + 0x4);
    uVar5 = uVar3 + uStack90 * 0xa;
    (uVar5 + 0x4) = (uStack90 * 0x2 + param_4);
    uStack90 += 0x1;
    string_1040_a626(uVar2,(uVar3 & 0xffff0000 | uVar5),CONCAT22(0x1050,&string_54));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn string_1010_dcac(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: u32,param_5: *mut astruct_104) -> i16

{
  let mut uVar1: u32;
  iVar2: *mut astruct_105;
  let mut uVar3: u16;
  let mut uVar4: u16;
  struct_104_5: *mut astruct_104;
  param_5: *mut astruct_104_seg;
  let mut uVar7: u16;
  let mut string_4: *mut c_char;
  let mut uVar2: u16;

  string_4 = load_string_1010_847e(_u16_1050_14cc,0x590);
  param_5_seg = (param_5 >> 0x10);
  struct_104_5 = param_5;
  uVar2 = (&struct_104_5.field1_0x2 + 2);
  iVar2 = (&struct_104_5.field1_0x2 + param_3 * 0xa);
  uVar7 = (param_4 >> 0x10);
  iVar2.field4_0x4 = (param_3 * 0x2 + param_4);
  string_1040_a626(uVar2,CONCAT22(uVar2,iVar2),string_4);
  unk_str_op_1000_3d3e(string_4,s__1050_3941);
  uVar3 = param_3 + 1;
  uVar1 = struct_104_5.field1_0x2;
  uVar4 = uVar1 + uVar3 * 0xa;
  (uVar4 + 0x4) = (uVar3 * 0x2 + param_4);
  string_1040_a626(uVar3,(uVar1 & 0xffff0000 | uVar4),string_4);
  return uVar3;
}
pub fn struct_1010_dd5e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  plStack16: *mut i32;

  if (param_3 != 0) {
    uVar4 = struct_op_1030_73a8(param_3,in_AX,in_DX);
    uVar3 = (uVar4 >> 0x10);
    iVar2 = uVar4;
    plStack16 = (uVar4 & 0xffff0000 | (iVar2 + 0x14));
    if ((uVar3 | iVar2 + 0x14) != 0) {
      iVar1 = (iVar2 + 0x12);
      iVar2 = (iVar2 + 0x18);
      if (((((iVar1 == 0x4) ||
            ((((iVar1 == 0x6 && (iVar2 == 0x4)) || (iVar1 == 0x5)) || ((iVar1 == 0x6 && (iVar2 == 0x5)))))) ||
           (iVar1 == 0x8)) || ((iVar1 == 0x6 && (iVar2 == 0x8)))) && (*plStack16 != 0)) {
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn load_str_1010_ddf6(mut param_1: u32,mut param_2: u32)

{
  let mut in_AX: u16;
  let mut in_DX: u16;
  short in_buf_len_5;
  let mut uVar1: u32;

  in_buf_len_5 = (param_1 >> 0x10);
  *(param_1 + 0x13c) = 0;
  uVar1 = struct_op_1030_73a8(param_2,in_AX,in_DX);
  switch((uVar1 + 0x12)) {
  case 0x1:
  case 0x2:
  case 0x4:
  case 0x7:
  case 0x9:
    break;
  case 0x3:
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x8:
    break;
  default:
// TODO: goto switchD_1010_de53_caseD_9;
  }
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,(param_1 + 0x13c),
             in_buf_len_5);
switchD_1010_de53_caseD_9:
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_de78(mut param_1: u32,mut param_2: u32)

{
  short in_buf_len_5;

  in_buf_len_5 = (param_1 >> 0x10);
  *(param_1 + 0x13c) = 0;
  pass1_1030_809c(param_2);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,(param_1 + 0x13c),
             in_buf_len_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_debe(mut param_1: u32,mut param_2: u16 ,param_3: *mut u16,param_4: *mut u32,mut param_5: u32)

{
  let mut bVar1: u8;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_EDX: u32;
  let mut paVar6: *mut Struct57;
  let mut iVar7: i16;
  let mut unaff_SI: u16;
  let mut uVar8: u16;
  paVar9: *mut astruct_15;
  let mut puVar10: *mut u32;
  let mut in_stack_0000fe84: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb2: u16;
  let mut uVar11: u16;
  let mut iStack34: i16;
  let mut uStack30: u16;
  let mut iStack26: i16;
  let mut iStack24: i16;
  let mut iStack22: i16;
  let mut iStack20: i16;

  *param_4 = 0;
  *param_3 = 0;
  paVar9 = struct_op_1030_73a8(param_5,0x0,in_EDX);
  paVar6 = (in_EDX & 0xffff0000 | paVar9 >> 0x10);
  iVar4 = (paVar9 + 0x12);
  uVar5 = param_1;
  uVar11 = (param_1 >> 0x10);
  uVar2 = pass1_1010_b028(uVar5,uVar11,paVar9);
  puVar10 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,CONCAT22(unaff_SI,0x35),in_stack_0000fe84,
                            in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
  paVar6 = (paVar6 & 0xffff0000 | puVar10 >> 0x10);
  iVar7 = param_4;
  uVar8 = (param_4 >> 0x10);
  if (param_2 == 0x13) {
    iStack34 = 0;
    while (iStack34 += 0x1, iStack34 < 0x43) {
      param_2 = pass1_1010_ac62(param_2,paVar6,uVar5,uVar11,iStack34);
      if (param_2 != 0) {
        *param_3 = *param_3 + 1;
      }
    }
    iVar4 = *param_3 * 0x2;
    mem_op_1000_179c(iVar4,paVar6);
    param_4 = iVar4;
    (iVar7 + 0x2) = paVar6;
    if ((paVar6 | param_4) != 0) {
      iStack34 = 0;
      for (uStack30 = 0; uVar2 = uStack30, *param_3 != uStack30 && uStack30 <= *param_3; uStack30 += 1) {
        loop {
          iStack34 += 0x1;
          if (0x42 < iStack34) goto LAB_1010_e0d4;
          uVar2 = pass1_1010_ac62(uVar2,paVar6,uVar5,uVar11,iStack34);
        } while (uVar2 == 0);
        (uStack30 * 0x2 + *param_4) = iStack34;//
// LAB_1010_e0d4:
      }
    }
  }
  else if (param_2 < 0x14) {
    if (param_2 == '\x06') {
      if (((iVar4 == 0x5) || (iVar4 == 0x6)) || (iVar4 == 0x8)) {
        uVar3 = puVar10 + 0x11e;
        if (uVar2 == 0xf) {
          iStack20 = 0xf;
          iStack22 = 0x13;
        }
        else if (uVar2 == 0xe) {
          iStack22 = 0x4;
          iStack20 = 0x1;
        }
        else {
          iStack22 = 0xe;
          iStack20 = 0x1;
        }
        iVar4 = pass1_1010_e128(uVar5,uVar11,iStack22,iStack20,puVar10 & 0xffff0000 | uVar3);
        *param_3 = iVar4 + 1;
        if (iVar4 + 0x1 != 0) {
          iVar4 = *param_3 * 0x2;
          mem_op_1000_179c(iVar4,paVar6);
          param_4 = iVar4;
          (iVar7 + 0x2) = paVar6;
          iStack24 = 0;
          for (iStack26 = iStack20; iStack26 <= iStack22; iStack26 += 1) {
            if ((iStack26 * 0x2 + uVar3) != 0) {
              (*param_4 + iStack24 * 0x2) = iStack26;
              iStack24 += 0x1;
            }
          }
          (*param_4 + iStack24 * 0x2) = 0x14;
          return;
        }
      }
    }
    else {
      bVar1 = param_2 - 0x7;
      if ((bVar1 == 0) && (((iVar4 == 0x5 || (iVar4 == 0x6)) || (iVar4 == 0x8)))) {
        uVar2 = pass1_1010_ac62(param_2 & 0xff00 | bVar1,(puVar10 >> 0x10),uVar5,uVar11,0x7);
        uVar5 = -(uVar2 == 0) + 0x10;
        *param_3 = uVar5;
        iVar4 = uVar5 * 0x2;
        mem_op_1000_179c(iVar4,paVar6);
        param_4 = iVar4;
        (iVar7 + 0x2) = paVar6;
        if ((paVar6 | param_4) == 0) {
          *param_3 = 0;
          return;
        }
        for (iStack26 = 0; iStack26 < (-(uVar2 == 0) + 0xf); iStack26 += 1) {
          (iStack26 * 0x2 + *param_4) = iStack26 + 1;
        }
        (iStack26 * 0x2 + *param_4) = 0x10;
        return;
      }
    }
  }
  return;
}
