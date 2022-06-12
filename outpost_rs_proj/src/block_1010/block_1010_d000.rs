




// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_d24a(uchar param_1,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,param_5: *mut astruct_104)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  char *pcVar3;
  let mut puVar4: *mut u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut puVar7: *mut u16;
  let mut uVar8: u16;
  let mut unaff_SI: i16;
  astruct_104 *iVar9;
  let mut uVar9: u16;
  i32 lVar10;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uStack320: u16;
  i32 lStack318;
  u16 *local_13a [0x4a];
  let mut local_a6: u32;
  let mut iStack18: i16;
  let mut uStack16: u32;
  char *pcStack12;
  let mut uStack10: u16;
  let mut puStack8: *mut u16;
  let mut iStack4: i16;

  iStack4 = 0x0;
  do {
    uVar9 = ((u32)param_5 >> 0x10);
    iVar9 = (astruct_104 *)param_5;
    uVar2 = iVar9->field1_0x2;
    (iStack4 * 0xa + (int)uVar2 + 0x4) = (iStack4 * 0x2 + (int)param_4);
    iStack4 += 0x1;
  } while (iStack4 < 0x8);
  puStack8 = (u16 *)iVar9->field1_0x2;
  iStack4 = 0x0;
  do {
    uVar1 = (u32)(iVar9 + 0x1);
    pcVar3 = pass1_1010_b038(param_3,uVar1,((u32)uVar1 >> 0x10),
                             *(u8 **)((int)puStack8 + 0x4),unaff_SI);
    string_1040_a626(param_2,puStack8,(char *)CONCAT22(param_2,pcVar3));
    iStack4 += 0x1;
    puStack8 = (u16 *)((u32)puStack8 & 0xffff0000 | (u32)((int)puStack8 + 0xa));
  } while (iStack4 < 0x8);
  uVar11 = param_3;
  uVar12 = (param_3 >> 0x10);
  struct_1010_dd5e(uVar11,uVar12,(u32)(iVar9 + 0x1));
  puVar6 = (u16 *)(param_2 | pcVar3);
  if (puVar6 != NULL) {
    pcStack12 = pcVar3;
    uStack10 = param_2;
    pass1_1010_e8f6(pcVar3,puVar6,uVar11,uVar12,(u32)(iVar9 + 0x1));
    uStack16 = CONCAT22(puVar6,pcVar3);
    iStack18 = 0x0;
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_a6),NULL,0x94);
    puVar4 = pass1_1000_4906((StructD *)CONCAT22(0x1050,local_13a),NULL,0x94);
    lStack318 = 0x0;
    for (uStack320 = 0x1; (int)uStack320 < 0x25; uStack320 += 0x1) {
      lVar10 = pass1_1030_7c28(puVar4,puVar6,uStack16,uStack320);
      puVar7 = (u16 *)((u32)lVar10 >> 0x10);
      puVar4 = (u16 *)lVar10;
      puVar6 = (u16 *)(puVar7 | puVar4);
      if (lVar10 != 0x0) {
        if (iStack18 == 0x0) {
          iStack18 = 0x1;
        }
        pcVar3 = string_1020_c0d8(uStack320);
        uVar8 = puVar6 | pcVar3;
        if (uVar8 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_a6)[(int)lStack318],s_Null_Ptr_1050_3917);
        }
        else {
          uVar5 = str_op_1008_60e8(uVar8,(char *)CONCAT22(puVar6,pcVar3));
          (&local_a6 + (int)lStack318) = uVar5;
          ((int)&local_a6 + (int)lStack318 * 0x4 + 0x2) = uVar8;
        }
        local_13a[(int)lStack318 * 0x2] = puVar4;
        local_13a[(int)lStack318 * 0x2 + 0x1] = puVar7;
        lStack318 += 0x1;
        puVar6 = puVar7;
      }
    }
    uVar8 = pass1_1010_db2e(uVar11,uVar12,0x8,CONCAT22(0x1050,&local_a6),CONCAT22(0x1050,local_13a),param_4,param_5);
    if (iStack18 != 0x0) {
      ((int)&iVar9[0x3].field1_0x2 + 0x2) = 0x1;
    }
    while (lStack318 != 0x0) {
      lStack318._0_2_ = (int)(lStack318 + -0x1);
      fn_ptr_1000_17ce((char *)(&local_a6)[(int)lStack318]);
      lStack318 = lStack318 + -0x1;
    }
    pass1_1010_dc36(uVar11,uVar12,uVar8,param_4,param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_d448(mut param_1: u16 ,mut param_2: u16 ,u8 *param_3,mut param_4: u32,param_5: *mut astruct_104,mut param_6: i16
                    )

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u32;
  let mut uVar4: u32;
  let mut puVar5: *mut u16;
  char *pcVar6;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  astruct_104 *iVar10;
  astruct_104 *uVar10;
  let mut uVar11: u32;
  let mut uVar12: u16;
  let mut local_40c: u16;
  let mut uStack1034: u32;
  let mut uStack1030: u32;
  u8 local_402 [0x400];

  uVar10 = (astruct_104 *)((u32)param_5 >> 0x10);
  iVar10 = (astruct_104 *)param_5;
  uStack1030 = struct_op_1030_73a8(*(astruct_419 **)(iVar10 + 0x1),param_1,param_2);
  uVar8 = (uStack1030 >> 0x10);
  uVar1 = uStack1030;
  if ((uVar8 | uVar1) != 0x0) {
    uStack1034 = (u32)(uVar1 + 0x20);
    uVar1 = (uVar1 + 0x22);
    if ((uVar1 | uStack1034) != 0x0) {
      local_40c = 0x0;
      puVar5 = &local_40c;
      uVar12 = ((u32)param_3 >> 0x10);
      pass1_1010_d984(param_3,uVar12,CONCAT22(0x1050,puVar5),0x3,
                      uStack1034 & 0xffff | (u32)uVar1 << 0x10,(u32)&PTR_DAT_1050_1805_1050_368e,(u32)param_5);
      puVar2 = (u16 *)iVar10->field1_0x2;
      uVar9 = ((int)&iVar10->field1_0x2 + 0x2);
      ((int)puVar2 + 0x4) = PTR_DAT_1050_1805_1050_368e;
      uVar3 = (u32)(iVar10 + 0x1);
      pcVar6 = pass1_1010_b038(param_3,uVar3,((u32)uVar3 >> 0x10),*(u8 **)((int)puVar2 + 0x4),
                               param_6);
      unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_402),(char *)CONCAT22(uVar9,pcVar6));
      string_1040_a626(uVar9,puVar2,(char *)CONCAT22(0x1050,local_402));
      uVar4 = iVar10->field1_0x2;
      uVar9 = ((int)&iVar10->field1_0x2 + 0x2);
      iVar7 = (int)uVar4;
      (iVar7 + 0xe) = PTR_DAT_1050_1822_1050_3690;
      sys_1000_3f9c((char *)CONCAT22(0x1050,local_402),(char *)0x10503920,local_40c);
      string_1040_a626(uVar9,(u16 *)(uVar4 & 0xffff0000 | (u32)(iVar7 + 0xa)),(char *)CONCAT22(0x1050,local_402));
      uVar4 = iVar10->field1_0x2;
      iVar7 = (int)uVar4;
      (iVar7 + 0x18) = PTR_DAT_1050_1823_1050_3692;
      uVar11 = pass1_1028_62c8(uStack1030);
      uVar9 = (uVar11 >> 0x10);
      sys_1000_3f9c((char *)CONCAT22(0x1050,local_402),(char *)0x10503923,uVar11);
      string_1040_a626(uVar9,(u16 *)(uVar4 & 0xffff0000 | (u32)(iVar7 + 0x14)),(char *)CONCAT22(0x1050,local_402));
      pass1_1010_dc36(param_3,uVar12,puVar5,param_4,param_5);
    }
  }
  return;
}
pub fn pass1_1010_d5ae(mut param_1: u16 ,mut param_2: u16 ,u8 *param_3,mut param_4: u32,param_5: *mut astruct_104,mut param_6: i16
                    )

{
  let mut puVar1: *mut u16;
  let mut uVar2: u32;
  u8 *puVar3;
  let mut puVar4: *mut u16;
  char *pcVar5;
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
  astruct_15 *paStack1030;
  char string_402 [0x400];
  let mut uVar9: u32;

  uVar11 = ((u32)param_5 >> 0x10);
  iVar10 = (int)param_5;
  paStack1030 = (astruct_15 *)struct_op_1030_73a8(*(astruct_419 **)(iVar10 + 0x6),param_1,param_2);
  uStack1034 = paStack1030;
  uVar7 = ((u32)paStack1030 >> 0x10) | uStack1034;
  uVar9 = CONCAT22(in_register_0000000a,uVar7);
  if (uVar7 != 0x0) {
    pass1_1028_45fe(uStack1034,paStack1030,uVar9);
    uStack1032 = uVar9;
    if ((uStack1032 | uStack1034) != 0x0) {
      local_40c = 0x0;
      puVar4 = &local_40c;
      uVar13 = ((u32)param_3 >> 0x10);
      pass1_1010_d984(param_3,uVar13,CONCAT22(0x1050,puVar4),0x3,CONCAT22(uStack1032,uStack1034),
                      (u32)&PTR_DAT_1050_1805_1050_3706,(u32)param_5);
      puVar1 = (u16*)(iVar10 + 0x2);
      uVar8 = (iVar10 + 0x4);
      ((int)puVar1 + 0x4) = PTR_DAT_1050_1805_1050_3706;
      uVar9 = (u32)(iVar10 + 0x6);
      pcVar5 = pass1_1010_b038(param_3,uVar9,((u32)uVar9 >> 0x10),*(u8 **)((int)puVar1 + 0x4),
                               param_6);
      unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,string_402),(char *)CONCAT22(uVar8,pcVar5));
      string_1040_a626(uVar8,puVar1,(char *)CONCAT22(0x1050,string_402));
      uVar2 = (u32)(iVar10 + 0x2);
      uVar8 = (iVar10 + 0x4);
      iVar6 = (int)uVar2;
      (iVar6 + 0xe) = PTR_DAT_1050_1822_1050_3708;
      sys_1000_3f9c((char *)CONCAT22(0x1050,string_402),s__u_1050_3926,local_40c);
      string_1040_a626(uVar8,(u16 *)(uVar2 & 0xffff0000 | (u32)(iVar6 + 0xa)),(char *)CONCAT22(0x1050,string_402));
      puVar3 = PTR_DAT_1050_1823_1050_370a;
      uVar2 = (u32)(iVar10 + 0x2);
      iVar10 = (iVar10 + 0x4);
      iVar6 = (int)uVar2;
      (iVar6 + 0x18) = PTR_DAT_1050_1823_1050_370a;
      uVar12 = pass1_1028_45e2(puVar3,iVar10,(u32)paStack1030);
      uVar8 = (uVar12 >> 0x10);
      sys_1000_3f9c((char *)CONCAT22(0x1050,string_402),(char *)0x10503929,uVar12);
      string_1040_a626(uVar8,(u16 *)(uVar2 & 0xffff0000 | (u32)(iVar6 + 0x14)),(char *)CONCAT22(0x1050,string_402))
      ;
      pass1_1010_dc36(param_3,uVar13,puVar4,param_4,param_5);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_d710(uchar param_1,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,param_5: *mut astruct_104)

{
  i32 lVar1;
  let mut uVar2: u32;
  char *pcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut unaff_SI: i16;
  let mut iVar8: i16;
  astruct_104 *iVar9;
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
  u16 local_136 [0x4a];
  let mut local_a2: u32;
  let mut iStack14: i16;
  let mut uStack12: u32;
  let mut puStack8: *mut u16;
  let mut iStack4: i16;

  iStack4 = 0x0;
  do {
    uVar9 = (param_4 >> 0x10);
    iVar8 = (int)param_4;
    uVar10 = ((u32)param_5 >> 0x10);
    iVar9 = (astruct_104 *)param_5;
    uVar2 = iVar9->field1_0x2;
    (iStack4 * 0xa + (int)uVar2 + 0x4) = (iStack4 * 0x2 + iVar8);
    iStack4 += 0x1;
  } while (iStack4 < 0x4);
  puStack8 = (u16 *)iVar9->field1_0x2;
  iStack4 = 0x0;
  do {
    uVar12 = (u32)(iVar9 + 0x1);
    pcVar3 = pass1_1010_b038(param_3,uVar12,((u32)uVar12 >> 0x10),
                             *(u8 **)((int)puStack8 + 0x4),unaff_SI);
    string_1040_a626(param_2,puStack8,(char *)CONCAT22(param_2,pcVar3));
    iStack4 += 0x1;
    puStack8 = (u16 *)((u32)puStack8 & 0xffff0000 | (u32)((int)puStack8 + 0xa));
  } while (iStack4 < 0x4);
  uVar13 = param_3;
  uVar14 = (param_3 >> 0x10);
  struct_1010_dd5e(uVar13,uVar14,(u32)(iVar9 + 0x1));
  uStack12 = CONCAT22(param_2,pcVar3);
  uVar6 = param_2 | pcVar3;
  if (uVar6 != 0x0) {
    iStack14 = 0x0;
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_a2),NULL,0x94);
    pass1_1000_4906((StructD *)CONCAT22(0x1050,local_136),NULL,0x94);
    iStack314 = 0x0;
    iStack312 = 0x0;
    iStack316 = 0x0;
    uVar12 = (u32)(iVar9 + 0x1);
    lVar1 = *(i32 *)((int)uVar12 + 0x26);
    for (uStack322 = 0x1; (int)uStack322 < 0x25; uStack322 += 0x1) {
      if (*(i32 *)(uStack322 * 0x4 + (int)uStack12) != 0x0) {
        if (iStack14 == 0x0) {
          iStack14 = 0x1;
        }
        pcVar3 = string_1020_c0d8(uStack322);
        uVar7 = uVar6 | pcVar3;
        if (uVar7 == 0x0) {
          unk_str_op_1000_3d3e((char *)(&local_a2)[iStack312],s_Null_Ptr_1050_392c);
        }
        else {
          uVar5 = str_op_1008_60e8(uVar7,(char *)CONCAT22(uVar6,pcVar3));
          (&local_a2 + iStack312) = uVar5;
          ((int)&local_a2 + iStack312 * 0x4 + 0x2) = uVar7;
        }
        uVar11 = ((u32)uStack12 >> 0x10);
        uVar7 = (uStack322 * 0x4 + (int)uStack12);
        uVar6 = (uStack322 * 0x4 + (int)uStack12 + 0x2);
        local_136[iStack312 * 0x2] = uVar7;
        local_136[iStack312 * 0x2 + 0x1] = uVar6;
        iStack312 += 0x1;
        if (lVar1 == 0x0) {
          iVar4 = 0x0;
        }
        else {
          uVar12 = pass1_1020_bae6(uVar7,uVar6,lVar1,CONCAT22(uStack322,(int)((u32)lVar1 >> 0x10)));
          uVar6 = ((u32)uVar12 >> 0x10);
          iVar4 = (int)uVar12;
        }
        if (iVar4 == 0x0) {
          iStack316 += 0x2;
        }
        else {
          (uVar13 + iStack314 * 0x2 + 0xa4) = (iVar8 + iStack316 * 0x2 + 0x8);
          (uVar13 + (iStack314 + 0x1) * 0x2 + 0xa4) =
               (iVar8 + (iStack316 + 0x1) * 0x2 + 0x8);
          iStack316 += 0x2;
          iStack314 += 0x2;
        }
      }
    }
    uVar6 = pass1_1010_db2e(uVar13,uVar14,0x4,CONCAT22(0x1050,&local_a2),CONCAT22(0x1050,local_136),param_4,param_5);
    if (iStack14 != 0x0) {
      ((int)&iVar9[0x3].field1_0x2 + 0x2) = 0x1;
    }
    while (iStack312 != 0x0) {
      fn_ptr_1000_17ce((char *)(&local_a2)[iStack312 + -0x1]);
      iStack312 = iStack312 + -0x1;
    }
    pass1_1010_dc36(uVar13,uVar14,uVar6,param_4,param_5);
  }
  return;
}
pub fn pass1_1010_d984(mut param_1: u16 ,mut param_2: u16 ,i16 *param_3,mut param_4: i16,mut param_5: u32,mut param_6: u32,mut param_7: u32)

{
  let mut uVar1: u16;
  u8 *puVar2;
  char *pcVar3;
  let mut iVar4: i16;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  u8 local_418 [0x400];
  let mut uStack24: u16;
  char *pcStack22;
  let mut uStack18: u16;
  let mut uStack16: u32;
  u8 local_c [0x8];
  let mut iStack4: i16;

  iStack4 = param_4;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_c),param_5);
  do {
    puVar2 = local_c;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar2));
    uStack16 = CONCAT22(extraout_DX,puVar2);
    uVar5 = extraout_DX | puVar2;
    if (uVar5 == 0x0) {
      return;
    }
    uStack18 = (puVar2 + 0xa);
    pcStack22 = NULL;
    if ((puVar2 + 0x4) == 0x0) {
      if ((puVar2 + 0x6) == 0x0) {
        if ((puVar2 + 0x8) == 0x0) {
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
    pcStack22 = (char *)CONCAT22(uVar5,pcVar3);
    uStack24 = ((int)uStack16 + 0xc);
    *param_3 = *param_3 + uStack24;
    uVar9 = (param_7 >> 0x10);
    iVar7 = (int)param_7;
    uVar1 = (iVar7 + 0x4);
    iVar4 = (iVar7 + 0x2) + iStack4 * 0xa;
    uVar10 = (param_6 >> 0x10);
    iVar8 = (int)param_6;
    (iVar4 + 0x4) = (iStack4 * 0x2 + iVar8);
    uVar6 = uVar1;
    sys_1000_3f9c((char *)CONCAT22(0x1050,local_418),(char *)0x10503935,uStack18);
    string_1040_a626(uVar6,(u16 *)CONCAT22(uVar1,iVar4),(char *)CONCAT22(0x1050,local_418));
    uVar1 = (iVar7 + 0x4);
    iStack4 += 0x1;
    iVar4 = (iVar7 + 0x2) + iStack4 * 0xa;
    (iVar4 + 0x4) = (iStack4 * 0x2 + iVar8);
    uVar6 = uVar1;
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_418),pcStack22);
    string_1040_a626(uVar6,(u16 *)CONCAT22(uVar1,iVar4),(char *)CONCAT22(0x1050,local_418));
    iVar4 = (iStack4 + 0x1) * 0xa + (iVar7 + 0x2);
    uVar1 = (iVar7 + 0x4);
    (iVar4 + 0x4) = ((iStack4 + 0x1) * 0x2 + iVar8);
    iStack4 += 0x2;
    uVar6 = uVar1;
    sys_1000_3f9c((char *)CONCAT22(0x1050,local_418),(char *)0x10503938,uStack24);
    string_1040_a626(uVar6,(u16 *)CONCAT22(uVar1,iVar4),(char *)CONCAT22(0x1050,local_418));
  } while( true );
}



u16 pass1_1010_db2e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: u32,mut param_6: u32,
                    param_7: *mut astruct_104)

{
  let mut uVar1: u16;
  astruct_493 *iVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  astruct_104 *iVar4;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uStack94: u16;
  let mut iStack92: i16;
  let mut uStack90: u16;
  let mut puStack88: *mut u16;
  u8 local_54 [0x52];

  uStack94 = param_3;
  uStack90 = param_3;
  iStack92 = 0x0;
  while( true ) {
    uVar7 = ((u32)param_7 >> 0x10);
    iVar4 = (astruct_104 *)param_7;
    if (param_7->field0_0x0 - 0x1 < uStack94) {
      return uStack94;
    }
    uVar1 = ((int)&iVar4->field1_0x2 + 0x2);
    iVar2 = (astruct_493 *)(&iVar4->field1_0x2 + uStack94 * 0xa);
    uVar5 = (param_5 >> 0x10);
    uVar6 = (param_4 >> 0x10);
    if ((*(i32 *)(iStack92 * 0x4 + (int)param_5) == 0x0) && (*(i32 *)(iStack92 * 0x4 + (int)param_4) == 0x0)) break;
    uVar4 = uVar1;
    unk_str_op_1000_3d3e((char *)CONCAT22(0x1050,local_54),*(char **)(iStack92 * 0x4 + (int)param_4));
    uVar6 = (param_6 >> 0x10);
    iVar2->field4_0x4 = (uStack90 * 0x2 + (int)param_6);
    string_1040_a626(uVar4,(u16 *)CONCAT22(uVar1,iVar2),(char *)CONCAT22(0x1050,local_54));
    sys_1000_3f9c((char *)CONCAT22(0x1050,local_54),(char *)0x1050393b,
                  (u32)((int)param_5 + iStack92 * 0x4));
    uVar1 = ((int)&iVar4->field1_0x2 + 0x2);
    iVar3 = &iVar4->field1_0x2 + (uStack94 + 0x1) * 0xa;
    puStack88 = (u16 *)CONCAT22(uVar1,iVar3);
    (iVar3 + 0x4) = ((uStack90 + 0x1) * 0x2 + (int)param_6);
    string_1040_a626(uVar1,puStack88,(char *)CONCAT22(0x1050,local_54));
    uStack94 += 0x2;
    uStack90 += 0x2;
    iStack92 += 0x1;
  }
  return uStack94;
}
pub fn pass1_1010_dc36(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,param_5: *mut astruct_104)

{
  u32 *puVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  u32 *puVar6;
  let mut uVar7: u16;
  let mut uStack90: u16;
  char *string_54;
  u32 local_52 [0x14];

  string_54 = PTR_s_New_failed_in_Op_Op_1050_0020_1050_393f;
  puVar6 = local_52;
  for (iVar4 = 0x13; iVar4 != 0x0; iVar4 += -0x1) {
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    *puVar1 = 0x0;
  }
  puVar6 = 0x0;
  *(u16 *)((int)puVar6 + 0x2) = 0x0;
  uStack90 = param_3;
  while( true ) {
    uVar7 = ((u32)param_5 >> 0x10);
    if (param_5->field0_0x0 < uStack90 || param_5->field0_0x0 == uStack90) break;
    uVar3 = (u32)((int)param_5 + 0x2);
    uVar2 = ((int)param_5 + 0x4);
    uVar5 = (int)uVar3 + uStack90 * 0xa;
    (uVar5 + 0x4) = (uStack90 * 0x2 + (int)param_4);
    uStack90 += 0x1;
    string_1040_a626(uVar2,(u16 *)(uVar3 & 0xffff0000 | (u32)uVar5),(char *)CONCAT22(0x1050,&string_54));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 string_1010_dcac(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: u32,param_5: *mut astruct_104)

{
  let mut uVar1: u32;
  astruct_105 *iVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  astruct_104 *struct_104_5;
  param_5: *mut astruct_104_seg;
  let mut uVar7: u16;
  char *string_4;
  let mut uVar2: u16;

  string_4 = load_string_1010_847e(_u16_1050_14cc,0x590);
  param_5_seg = (astruct_104 *)((u32)param_5 >> 0x10);
  struct_104_5 = (astruct_104 *)param_5;
  uVar2 = ((int)&struct_104_5->field1_0x2 + 0x2);
  iVar2 = (astruct_105 *)(&struct_104_5->field1_0x2 + param_3 * 0xa);
  uVar7 = (param_4 >> 0x10);
  iVar2->field4_0x4 = (param_3 * 0x2 + (int)param_4);
  string_1040_a626(uVar2,(u16 *)CONCAT22(uVar2,iVar2),string_4);
  unk_str_op_1000_3d3e(string_4,s__1050_3941);
  uVar3 = param_3 + 0x1;
  uVar1 = struct_104_5->field1_0x2;
  uVar4 = (int)uVar1 + uVar3 * 0xa;
  (uVar4 + 0x4) = (uVar3 * 0x2 + (int)param_4);
  string_1040_a626(uVar3,(u16 *)(uVar1 & 0xffff0000 | (u32)uVar4),string_4);
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
  i32 *plStack16;

  if (param_3 != 0x0) {
    uVar4 = struct_op_1030_73a8((astruct_419 *)param_3,in_AX,in_DX);
    uVar3 = (uVar4 >> 0x10);
    iVar2 = (int)uVar4;
    plStack16 = (i32 *)(uVar4 & 0xffff0000 | (u32)(iVar2 + 0x14U));
    if ((uVar3 | iVar2 + 0x14U) != 0x0) {
      iVar1 = (iVar2 + 0x12);
      iVar2 = (iVar2 + 0x18);
      if (((((iVar1 == 0x4) ||
            ((((iVar1 == 0x6 && (iVar2 == 0x4)) || (iVar1 == 0x5)) || ((iVar1 == 0x6 && (iVar2 == 0x5)))))) ||
           (iVar1 == 0x8)) || ((iVar1 == 0x6 && (iVar2 == 0x8)))) && (*plStack16 != 0x0)) {
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

  in_buf_len_5 = (short)(param_1 >> 0x10);
  *((int)param_1 + 0x13c) = 0x0;
  uVar1 = struct_op_1030_73a8((astruct_419 *)param_2,in_AX,in_DX);
  switch(((int)uVar1 + 0x12)) {
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
    goto switchD_1010_de53_caseD_9;
  }
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)((int)param_1 + 0x13c),
             in_buf_len_5);
switchD_1010_de53_caseD_9:
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_de78(mut param_1: u32,mut param_2: u32)

{
  short in_buf_len_5;

  in_buf_len_5 = (short)(param_1 >> 0x10);
  *((int)param_1 + 0x13c) = 0x0;
  pass1_1030_809c(param_2);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)((int)param_1 + 0x13c),
             in_buf_len_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_debe(mut param_1: u32,mut param_2: u16 ,param_3: *mut u16,u32 *param_4,mut param_5: u32)

{
  u8 bVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar6;
  let mut iVar7: i16;
  let mut unaff_SI: u16;
  let mut uVar8: u16;
  astruct_15 *paVar9;
  u32 *puVar10;
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

  *param_4 = 0x0;
  *param_3 = 0x0;
  paVar9 = (astruct_15 *)struct_op_1030_73a8((astruct_419 *)param_5,0x0,(int)in_EDX);
  paVar6 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)paVar9 >> 0x10);
  iVar4 = ((int)paVar9 + 0x12);
  uVar5 = param_1;
  uVar11 = (param_1 >> 0x10);
  uVar2 = pass1_1010_b028(uVar5,uVar11,paVar9);
  puVar10 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x35),in_stack_0000fe84,
                            in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
  paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)puVar10 >> 0x10);
  iVar7 = (int)param_4;
  uVar8 = ((u32)param_4 >> 0x10);
  if (param_2 == 0x13) {
    iStack34 = 0x0;
    while (iStack34 += 0x1, iStack34 < 0x43) {
      param_2 = pass1_1010_ac62(param_2,paVar6,uVar5,uVar11,iStack34);
      if (param_2 != 0x0) {
        *param_3 = *param_3 + 0x1;
      }
    }
    iVar4 = *param_3 * 0x2;
    mem_op_1000_179c(iVar4,paVar6);
    param_4 = iVar4;
    (iVar7 + 0x2) = paVar6;
    if ((paVar6 | param_4) != 0x0) {
      iStack34 = 0x0;
      for (uStack30 = 0x0; uVar2 = uStack30, *param_3 != uStack30 && (int)uStack30 <= (int)*param_3; uStack30 += 0x1) {
        do {
          iStack34 += 0x1;
          if (0x42 < iStack34) goto LAB_1010_e0d4;
          uVar2 = pass1_1010_ac62(uVar2,paVar6,uVar5,uVar11,iStack34);
        } while (uVar2 == 0x0);
        (uStack30 * 0x2 + (int)*param_4) = iStack34;//
LAB_1010_e0d4:
      }
    }
  }
  else if (param_2 < 0x14) {
    if ((char)param_2 == '\x06') {
      if (((iVar4 == 0x5) || (iVar4 == 0x6)) || (iVar4 == 0x8)) {
        uVar3 = (int)puVar10 + 0x11e;
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
        iVar4 = pass1_1010_e128(uVar5,uVar11,iStack22,iStack20,(u32)puVar10 & 0xffff0000 | (u32)uVar3);
        *param_3 = iVar4 + 0x1U;
        if (iVar4 + 0x1U != 0x0) {
          iVar4 = *param_3 * 0x2;
          mem_op_1000_179c(iVar4,paVar6);
          param_4 = iVar4;
          (iVar7 + 0x2) = (int)paVar6;
          iStack24 = 0x0;
          for (iStack26 = iStack20; iStack26 <= iStack22; iStack26 += 0x1) {
            if ((iStack26 * 0x2 + uVar3) != 0x0) {
              ((int)*param_4 + iStack24 * 0x2) = iStack26;
              iStack24 += 0x1;
            }
          }
          ((int)*param_4 + iStack24 * 0x2) = 0x14;
          return;
        }
      }
    }
    else {
      bVar1 = (char)param_2 - 0x7;
      if ((bVar1 == 0x0) && (((iVar4 == 0x5 || (iVar4 == 0x6)) || (iVar4 == 0x8)))) {
        uVar2 = pass1_1010_ac62(param_2 & 0xff00 | bVar1,((u32)puVar10 >> 0x10),uVar5,uVar11,0x7);
        uVar5 = -(uVar2 == 0x0) + 0x10;
        *param_3 = uVar5;
        iVar4 = uVar5 * 0x2;
        mem_op_1000_179c(iVar4,paVar6);
        param_4 = iVar4;
        (iVar7 + 0x2) = paVar6;
        if ((paVar6 | param_4) == 0x0) {
          *param_3 = 0x0;
          return;
        }
        for (iStack26 = 0x0; iStack26 < (int)(-(uVar2 == 0x0) + 0xf); iStack26 += 0x1) {
          (iStack26 * 0x2 + (int)*param_4) = iStack26 + 0x1;
        }
        (iStack26 * 0x2 + (int)*param_4) = 0x10;
        return;
      }
    }
  }
  return;
}

