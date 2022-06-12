
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_str_op_1008_d1c6(param_1: *mut astruct_263,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut puVar2: *mut u32;
  code **ppcVar3;
  let mut bVar4: bool;
  let mut puVar5: *mut u32;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut in_EDX: u32;
  let mut paVar14: *mut Struct57;
  let mut paVar15: *mut Struct57;
  let mut paVar17: *mut Struct57;
  let mut paVar18: *mut Struct57;
  astruct_5 *iVar15;
  let mut uVar19: u16;
  let mut uVar20: u16;
  let mut puVar21: *mut u32;
  let mut uVar22: u32;
  u8 uVar23;
  let mut uStack52: u16;
  let mut uStack20: u32;
  let mut uStack14: u32;
  let mut puStack10: *mut u32;
  let mut uVar16: u32;

  uVar19 = (param_1 >> 0x10);
  iVar15 = (astruct_5 *)param_1;
    // WARNING: Load size is inaccurate
  puVar5 = iVar15->field18_0x12;
  uVar9 = (&iVar15->field18_0x12 + 0x2);
  paVar14 = (astruct_57 *)(in_EDX & 0xffff0000 | uVar9);
  if ((uVar9 | puVar5) != 0x0) {
    ppcVar3 = (code **)*puVar5;
    (**ppcVar3)();
  }
  mem_op_1000_179c(0xc,paVar14);
  uVar9 = paVar14 | puVar5;
  uVar16 = paVar14 & 0xffff0000;
  uVar22 = uVar16 | uVar9;
  if (uVar9 == 0x0) {
    puVar5 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar14,puVar5));
    uVar16 = uVar22;
  }
  (u32*)&iVar15->field18_0x12 = puVar5;
  (&iVar15->field18_0x12 + 0x2) = uVar16;
  puVar21 = pass1_1008_c6fa(_u16_1050_06e0,0x2);
  paVar15 = (astruct_57 *)(uVar16 & 0xffff0000 | puVar21 >> 0x10);
  uVar9 = puVar21;
  uVar20 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4e78(uVar9,paVar15,param_2,puVar21);
  uVar10 = paVar15;
  puStack10 = CONCAT22(uVar10,uVar9);
  ppcVar3 = (code **)(*puStack10 + 0x10);
  paVar14 = paVar15;
  uVar6 = uVar9;
  (**ppcVar3)(&u16_1050_1038,uVar9,uVar10);
  uStack14 = CONCAT22(paVar14,uVar6);
  bVar4 = false;
  for (uStack20 = 0x0; uStack20 < uStack14; uStack20 += 0x1) {
    uVar20 = 0x1030;
    uVar22 = pass1_1030_1d7c(uVar6,paVar14,puStack10);
    uVar16 = paVar14 & 0xffff0000;
    uVar13 = uVar22;
    uVar11 = (uVar22 >> 0x10);
    paVar14 = (astruct_57 *)(uVar16 | (uVar11 | uVar13));
    if ((((uVar11 | uVar13) != 0x0) && (*(i32 *)(uVar13 + 0x1c) != 0x8000002)) &&
       ((iVar1 = (uVar13 + 0x12), iVar1 == 0x5 || (iVar1 == 0x6)))) {
      uVar22 = (uVar13 + 0x6) & 0xffff00ff;
      paVar17 = (astruct_57 *)(uVar16 | uVar22);
      uVar20 = uVar22;
      uVar23 = (u8)(uVar13 + 0x4);
      uVar7 = pass1_1020_bd80((uVar13 + 0xc));
      wsprintf16(&iVar15->field31_0x22,CONCAT13(0xc,CONCAT12(0xea,uVar19)),0xea,0x50,CONCAT22(uVar7,0x1050),
                 (char)paVar17,uVar23,uVar20);
      uVar8 = str_op_1008_60e8(paVar17,(param_1 & 0xffff0000 | ZEXT24(&iVar15->field31_0x22)));
      uVar20 = 0x1000;
      paVar18 = paVar17;
      uVar7 = uVar8;
      mem_op_1000_179c(0x12,paVar17);
      uVar12 = paVar18 | uVar7;
      paVar14 = (astruct_57 *)(paVar18 & 0xffff0000 | uVar12);
      if (uVar12 == 0x0) {
        uVar23 = 0x0;
        uStack52 = 0x0;
      }
      else {
        uVar20 = 0x1018;
        pass1_1018_4808((astruct_203 *)CONCAT22(paVar18,uVar7),0x1,
                        CONCAT13((char)(paVar17 >> 0x8),CONCAT12((char)paVar17,uVar8)),(uVar13 + 0x4));
        uVar23 = (u8)uVar7;
        uStack52 = SUB42(paVar14,0x0);
      }
      puVar2 = iVar15->field18_0x12;
      ppcVar3 = (code **)(*iVar15->field18_0x12 + 0x4);
      (**ppcVar3)((char)uVar20,puVar2,(char)(puVar2 >> 0x10),uVar23,uStack52);
      bVar4 = true;
    }
  }
  if (!bVar4) {
    load_string_1010_84ac(_u16_1050_14cc,(INT16)(_u16_1050_14cc >> 0x10),0x441);
    uVar20 = 0x1000;
    paVar18 = paVar14;
    uVar6 = uStack14;
    mem_op_1000_179c(0x12,paVar14);
    uVar13 = paVar18 | uVar6;
    if (uVar13 == 0x0) {
      uVar23 = 0x0;
      uVar13 = 0x0;
    }
    else {
      uVar20 = 0x1018;
      pass1_1018_4808((astruct_203 *)CONCAT22(paVar18,uVar6),0x0,
                      CONCAT13((char)(paVar14 >> 0x8),CONCAT12((char)paVar14,uStack14)),0x0);
      uVar23 = (u8)uVar6;
    }
    puVar2 = iVar15->field18_0x12;
    ppcVar3 = (code **)(*iVar15->field18_0x12 + 0x4);
    (**ppcVar3)((char)uVar20,puVar2,(char)(puVar2 >> 0x10),uVar23,uVar13);
  }
  if ((uVar10 | uVar9) != 0x0) {
    ppcVar3 = (code **)*puStack10;
    (**ppcVar3)(uVar20,(char)uVar9,(char)paVar15,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_d3ae(param_1: *mut astruct_263)

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut bVar3: bool;
  let mut puVar4: *mut u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut in_EDX: u32;
  let mut paVar11: *mut Struct57;
  let mut paVar13: *mut Struct57;
  astruct_263 *iVar13;
  let mut uVar14: u16;
  u8 uVar15;
  let mut uStack6: u16;
  let mut paVar12: *mut Struct57;

  uVar14 = (param_1 >> 0x10);
  iVar13 = (astruct_263 *)param_1;
  puVar4 = iVar13->field8_0xa;
  uVar9 = iVar13->field9_0xc;
  paVar11 = (astruct_57 *)(in_EDX & 0xffff0000 | uVar9);
  if ((uVar9 | puVar4) != 0x0) {
    ppcVar2 = (code **)*puVar4;
    puVar4 = (**ppcVar2)();
  }
  mem_op_1000_179c(0xc,paVar11);
  uVar9 = paVar11 | puVar4;
  paVar13 = (astruct_57 *)(paVar11 & 0xffff0000);
  paVar12 = (astruct_57 *)(paVar13 | uVar9);
  if (uVar9 == 0x0) {
    uVar9 = 0x0;
  }
  else {
    uVar9 = set_struct_1008_574a((astruct_57 *)CONCAT22(paVar11,puVar4));
    paVar13 = paVar12;
  }
  iVar13->field8_0xa = uVar9;
  iVar13->field9_0xc = paVar13;
  bVar3 = false;
  for (uStack6 = 0x21; 0x10 < uStack6; uStack6 -= 0x1) {
    empty_1038_540a();
    uVar10 = paVar13;
    uVar5 = uVar10 | uVar9;
    if (uVar5 != 0x0) {
      bVar3 = true;
      string_1020_c0ca(uStack6);
      uVar6 = str_op_1008_60e8(paVar13,CONCAT22(paVar13,uVar5));
      uVar8 = SUB42(paVar13,0x0);
      uVar15 = 0x0;
      uVar7 = uVar6;
      mem_op_1000_179c(0x10,paVar13);
      uVar5 = paVar13 | uVar7;
      paVar11 = (astruct_57 *)(paVar13 & 0xffff0000 | uVar5);
      if (uVar5 == 0x0) {
        uVar8 = 0x0;
        paVar13 = (astruct_57 *)(paVar13 & 0xffff0000);
      }
      else {
        uVar15 = 0x18;
        uVar8 = struct_1018_4790(CONCAT22(paVar13,uVar7),CONCAT22(uVar10,uVar9),CONCAT22(uVar8,uVar6),uStack6);
        paVar13 = paVar11;
      }
      uVar1 = &iVar13->field8_0xa;
      ppcVar2 = (code **)(*(u32*)&iVar13->field8_0xa + 0x4);
      uVar5 = (**ppcVar2)(uVar15,uVar1,(uVar1 >> 0x10),uVar8,paVar13);
    }
    uVar9 = uVar5;
  }
  if (!bVar3) {
    load_string_1010_84ac(_u16_1050_14cc,(INT16)(_u16_1050_14cc >> 0x10),0x43e);
    uVar8 = SUB42(paVar13,0x0);
    uVar15 = 0x0;
    uVar5 = uVar9;
    mem_op_1000_179c(0x10,paVar13);
    uVar10 = paVar13 | uVar5;
    if (uVar10 == 0x0) {
      uVar8 = 0x0;
      uVar10 = 0x0;
    }
    else {
      uVar15 = 0x18;
      uVar8 = struct_1018_4790(CONCAT22(paVar13,uVar5),0x0,CONCAT22(uVar8,uVar9),0x0);
    }
    uVar1 = &iVar13->field8_0xa;
    ppcVar2 = (code **)(*(u32*)&iVar13->field8_0xa + 0x4);
    (**ppcVar2)(uVar15,uVar1,(uVar1 >> 0x10),uVar8,uVar10);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_str_op_1008_d4f6(param_1: *mut astruct_263,param_2: *mut astruct_6)

{
  let mut iVar1: i16;
  i32 lVar2;
  let mut puVar3: *mut u32;
  let mut uVar4: u32;
  let mut bVar5: bool;
  let mut puVar6: *mut u32;
  let mut BVar7: bool;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut puVar10: *mut u32;
  let mut uVar11: u32;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut in_EDX: u32;
  let mut paVar15: *mut Struct57;
  let mut uVar16: u32;
  let mut paVar18: *mut Struct57;
  astruct_6 *iVar18;
  StructD *structd_v20;
  let mut uVar19: u16;
  let mut structd_v20_hi: u16;
  let mut uVar20: u16;
  let mut uVar21: u32;
  u8 uVar22;
  let mut uStack58: u16;
  let mut uStack20: u32;
  let mut uStack12: u16;
  let mut uVar17: u32;
  code **fn_ptr_v5;

  uVar19 = (param_2 >> 0x10);
  iVar18 = (astruct_6 *)param_2;
  lVar2 = iVar18->field509_0x200;
  structd_v20_hi = (param_1 >> 0x10);
  structd_v20 = (StructD *)param_1;
  puVar6 = structd_v20->field8_0xe;
  uVar12 = &structd_v20->field_0x10;
  paVar15 = (astruct_57 *)(in_EDX & 0xffff0000 | uVar12);
  if ((uVar12 | puVar6) != 0x0) {
    fn_ptr_v5 = (code **)*puVar6;
    (**fn_ptr_v5)();
  }
  mem_op_1000_179c(0xc,paVar15);
  uVar12 = paVar15 | puVar6;
  uVar17 = paVar15 & 0xffff0000;
  uVar11 = uVar17 | uVar12;
  if (uVar12 == 0x0) {
    puVar6 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar15,puVar6));
    uVar17 = uVar11;
  }
  structd_v20->field8_0xe = puVar6;
  &structd_v20->field_0x10 = uVar17;
  puVar3 = iVar18->field12_0xc;
  uVar12 = (&iVar18->field12_0xc + 0x2);
  uVar16 = uVar17 & 0xffff0000 | uVar12;
  fn_ptr_v5 = (code **)(*puVar3 + 0x10);
  puVar10 = puVar3;
  (**fn_ptr_v5)(0x1000,puVar3,uVar12);
  uVar11 = puVar10 & 0xffff | uVar16 << 0x10;
  bVar5 = false;
  uStack20 = 0x0;
  uVar17 = uVar16;
  while( true ) {
    uStack12 = uVar16;
    paVar15 = (astruct_57 *)(uVar17 & 0xffff0000 | uVar16 & 0xffff);
    if (uVar11 <= uStack20) break;
    uVar21 = pass1_1030_1d7c((puVar10 & 0xffff),uStack12,puVar3);
    uVar17 = paVar15 & 0xffff0000;
    uVar12 = uVar21;
    uVar14 = (uVar21 >> 0x10);
    if ((((uVar14 | uVar12) != 0x0) && (*(i32 *)(uVar12 + 0x1c) != 0x8000002)) &&
       ((iVar1 = (uVar12 + 0x12), iVar1 == 0x5 || (iVar1 == 0x6)))) {
      uVar8 = (uVar12 + 0xc);
      BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar8,0x34);
      if ((BVar7 == 0x0) && (*(i32 *)(uVar12 + 0x1c) != lVar2)) {
        uVar21 = (uVar12 + 0x6) & 0xffff00ff;
        uVar17 = uVar17 & 0xffff0000 | uVar21;
        uVar19 = uVar21;
        uVar22 = (u8)(uVar12 + 0x4);
        uVar8 = pass1_1020_bd80(uVar8);
        paVar15 = (astruct_57 *)(uVar17 & 0xffff0000 | structd_v20_hi);
        wsprintf16(&structd_v20->field20_0x22,CONCAT22(0xcf4,structd_v20_hi),CONCAT22(uVar8,0x1050),
                   (char)uVar17,uVar22,uVar19);
        uVar9 = str_op_1008_60e8(paVar15,
                                 (param_1 & 0xffff0000 | ZEXT24(&structd_v20->field20_0x22)));
        uVar19 = SUB42(paVar15,0x0);
        uVar20 = 0x1000;
        uVar8 = uVar9;
        mem_op_1000_179c(0x14,paVar15);
        uVar13 = paVar15 | uVar8;
        uVar17 = paVar15 & 0xffff0000 | uVar13;
        if (uVar13 == 0x0) {
          uVar22 = 0x0;
          uStack58 = 0x0;
        }
        else {
          uVar20 = 0x1018;
          struct_1018_47c8((astruct_203 *)CONCAT22(paVar15,uVar8),0x1,CONCAT22(uVar19,uVar9),
                           (uVar12 + 0xc),(uVar12 + 0x4));
          uVar22 = (u8)uVar8;
          uStack58 = uVar17;
        }
        uVar4 = &structd_v20->field8_0xe;
        fn_ptr_v5 = (code **)(*(u32*)&structd_v20->field8_0xe + 0x4);
        (**fn_ptr_v5)(uVar20,uVar4,(char)(uVar4 >> 0x10),uVar22,uStack58);
        bVar5 = true;
      }
    }
    uStack20 += 0x1;
  }
  if (!bVar5) {
    load_string_1010_84ac(_u16_1050_14cc,(INT16)(_u16_1050_14cc >> 0x10),0x442);
    uVar12 = uVar11;
    uVar19 = 0x1000;
    paVar18 = paVar15;
    mem_op_1000_179c(0x14,paVar15);
    uVar14 = paVar18 | uVar12;
    if (uVar14 == 0x0) {
      uVar22 = 0x0;
      uVar14 = 0x0;
    }
    else {
      uVar19 = 0x1018;
      struct_1018_47c8((astruct_203 *)CONCAT22(paVar18,uVar12),0x0,uVar11 & 0xffff | (long)paVar15 << 0x10,0x0,0x0
                      );
      uVar22 = (u8)uVar12;
    }
    uVar4 = &structd_v20->field8_0xe;
    fn_ptr_v5 = (code **)(*(u32*)&structd_v20->field8_0xe + 0x4);
    (**fn_ptr_v5)(uVar19,uVar4,(char)(uVar4 >> 0x10),uVar22,uVar14);
  }
  return;
}



u16 * pass1_1008_d6f4(param_1: *mut u16,param_2: u8)

{
  pass1_1008_caa0((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1008_d72e(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0x0;
  param_1->offset_0x0 = 0xd780;
  (param_1 + 0x2) = 0x1008;
  return &param_1->offset_0x0;
}



u16 * pass1_1008_d75a(param_1: *mut u16,param_2: u8)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_d790(param_1: *mut astruct_19,param_2: *mut astruct_19,mut param_3: u16 )

{
  INT16 IVar1;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut uVar2: u32;
  astruct_19 *paVar4;

  uVar3 = (in_EDX >> 0x10);
  paVar4 = struct_op_1010_1d48((astruct_19 *)CONCAT22(param_2,param_1),param_3);
  uVar2 = CONCAT22(uVar3,(paVar4 >> 0x10));
  &param_1->horiz_res_0xa = 0x0;
  &param_1->field_0xe = 0x0;
  CONCAT22(param_2,param_1) = 0xd98e;
  param_1->segment_0x2 = 0x1008;
  IVar1 = FUN_1010_830a(0x0,uVar2,0x1010,_u16_1050_14cc,0x9b);
  param_1->horiz_res_0xa = IVar1;
  param_1->ver_res_0xc = uVar2;
  return;
}
pub fn pass1_1008_d7da(param_1: *mut u16)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0xd98e;
  (iVar4 + 0x2) = 0x1008;
  puVar1 = (iVar4 + 0xa);
  uVar2 = (iVar4 + 0xc);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80((StructD *)param_1);
  return;
}
pub fn pass1_1008_d818(param_1: *mut astruct_263,mut param_2: i16)

{
  astruct_263 *iVar1;
  let mut uVar1: u16;

  if (param_2 - 0x1a0U < 0x15) {
    iVar1 = (astruct_263 *)param_1;
    uVar1 = (param_1 >> 0x10);
    switch(param_2) {
    case 0x1a0:
      iVar1->field10_0xe = 0x14;
      break;
    case 0x1a1:
      iVar1->field10_0xe = 0x3;
      break;
    case 0x1a2:
      iVar1->field10_0xe = 0x2;
      break;
    case 0x1a3:
      iVar1->field10_0xe = 0xe;
      break;
    case 0x1a4:
      iVar1->field10_0xe = 0xc;
      break;
    case 0x1a5:
      iVar1->field10_0xe = 0x4;
      break;
    case 0x1a6:
      iVar1->field10_0xe = 0xb;
      break;
    case 0x1a7:
      iVar1->field10_0xe = 0x6;
      break;
    case 0x1a8:
      iVar1->field10_0xe = 0xa;
      break;
    case 0x1a9:
      iVar1->field10_0xe = 0xd;
      break;
    case 0x1aa:
      iVar1->field10_0xe = 0x13;
      break;
    case 0x1ab:
      iVar1->field10_0xe = 0x5;
      break;
    case 0x1ac:
      iVar1->field10_0xe = 0x9;
      break;
    case 0x1ad:
      iVar1->field10_0xe = 0x8;
      break;
    case 0x1ae:
      iVar1->field10_0xe = 0x12;
      break;
    case 0x1af:
      iVar1->field10_0xe = 0x11;
      break;
    case 0x1b0:
      iVar1->field10_0xe = 0x7;
      return;
    case 0x1b1:
      iVar1->field10_0xe = 0x10;
      return;
    case 0x1b2:
      iVar1->field10_0xe = 0x1;
      return;
    case 0x1b3:
      iVar1->field10_0xe = 0xf;
      return;
    case 0x1b4:
      iVar1->field10_0xe = 0x15;
      return;
    }
  }
  return;
}



StructD * pass1_1008_d968(StructD *param_1,param_2: u8)

{
  pass1_1008_d7da(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_d99e(u8 *param_1,param_2: *mut astruct_19,mut param_3: u16 )

{
  struct_op_1018_4cda(param_2,param_3);
  param_2->offset_0x0 = 0xd9fa;
  (param_2 + 0x2) = 0x1008;
  pass1_1018_4dce(param_1,param_2,0x9a);
  _PTR_LOOP_1050_4230 = param_2;
  return;
}



StructD * pass1_1008_d9d4(StructD *param_1,param_2: u8)

{
  clenaup_win_ui_1018_4d22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_draw_op_1008_da12(param_1: *mut astruct_19,mut param_2: u16 )

{
  u8 bVar1;
  HDC16 hdc;
  let mut horiz_res: i16;
  let mut vert_res: i16;
  let mut iVar2: i16;
  let mut raster_caps: u16;
  let mut sz_palette: i16;
  let mut num_reserved: i16;
  PALETTEENTRY *entries;
  let mut uVar4: u16;
  let mut start: u16;
  u8 *count;
  let mut count_00: u16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut puStack32: *mut u16;
  let mut iStack16: i16;
  i32 lStack8;
  let mut uVar3: u32;
  let mut piVar1: *mut i16;
  let mut puVar2: *mut u16;
  StructD *pSVar2;

  uVar5 = (in_EDX >> 0x10);
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0x0;
  (param_1 + 0xc) = 0x0;
  pass1_1008_3e38((astruct_19 *)
                  (param_1 & 0xff000000 | CONCAT12((char)(param_1 >> 0x10),param_1 + 0xeU)));
  (param_1 + 0x14) = 0x0;
  (param_1 + 0x16) = 0x0;
  (param_1 + 0x18) = 0x0;
  param_1->offset_0x0 = 0xdc80;
  (param_1 + 0x2) = 0x1008;
  hdc = GetDC16(0x0);
  horiz_res = GetDeviceCaps16(HORZRES,hdc);
  (param_1 + 0xa) = horiz_res;
  vert_res = GetDeviceCaps16(VERTRES,hdc);
  (param_1 + 0xc) = vert_res;
  iVar2 = (param_1 + 0xc) + -0x1e0;
  count = (iVar2 >> 0xf);
  pSVar2 = (StructD *)CONCAT22(uVar5,count);
  pass1_1008_3e76((u16 *)(param_1 & 0xffff0000 | (param_1 + 0xeU)),0x0,iVar2 / 0x2,
                  ((param_1 + 0xa) + -0x280) / 0x2);
  raster_caps = GetDeviceCaps16(RASTERCAPS,hdc);
  if ((raster_caps & 0x100) != 0x0) {
    sz_palette = GetDeviceCaps16(SIZEPALETTE,hdc);
    (param_1 + 0x14) = sz_palette;
    num_reserved = GetDeviceCaps16(NUMRESERVED,hdc);
    (param_1 + 0x16) = num_reserved;
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar2);
    }
    else {
      pSVar2 = (StructD *)(pSVar2 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    entries = (PALETTEENTRY *)
              fn_ptr_op_1000_1708((num_reserved + 0x1) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar2);
    count_00 = pSVar2;
    lStack8 = CONCAT22(count_00,entries);
    iVar6 = (param_1 + 0x16);
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar2);
      PTR_LOOP_1050_5f2e = pSVar2;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708((iVar6 + 0x1) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
    (param_1 + 0x18) = uVar4;
    (param_1 + 0x1a) = PTR_LOOP_1050_5f2e;
    if (lStack8 != 0x0) {
      if (*(i32 *)(param_1 + 0x18) != 0x0) {
        start = (param_1 + 0x16) / 0x2;
        GetSystemPaletteEntries(entries,count_00,start,0x0);
        GetSystemPaletteEntries(entries + start,count_00,start,(param_1 + 0x14) - start);
        puStack32 = (u16*)(param_1 + 0x18);
        for (iStack16 = 0x0; puVar2 = puStack32, piVar1 = (param_1 + 0x16),
            *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 += 0x1) {
          bVar1 = (entries + iStack16)->pe_red;
          uVar3 = puStack32 >> 0x10;
          iVar6 = puStack32;
          puStack32 = (u16 *)(puStack32 & 0xffff0000 | (iVar6 + 0x4));
          *puVar2 = CONCAT11(entries[iStack16].pe_green,entries[iStack16].pe_blue);
          (iVar6 + 0x2) = bVar1;
        }
      }
    }
    fn_ptr_1000_17ce(CONCAT22(count_00,entries));
  }
  ReleaseDC16(hdc,0x0);
  return;
}
pub fn pass1_1008_dc2c(param_1: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_1 = 0xdc80;
  (param_1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(*(char **)(param_1 + 0x18));
  pass1_1010_1d80((StructD *)param_1);
  return;
}



StructD * pass1_1008_dc5a(StructD *param_1,param_2: u8)

{
  pass1_1008_dc2c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Variable defined which should be unmapped: param_9
pub fn pass1_1008_dc80(mut param_1: u16 ,mut param_2: i16,param_3: u8,uchar param_4,mut param_5: u16 ,param_6: *mut u16,mut param_7: u32,
                    mut param_8: u32,mut param_9: u16 )

{
  let mut pcVar1: *mut c_char;
  let mut uVar2: u16;
  let mut uVar3: u16;
  code *pcVar4;
  let mut uVar5: u16;
  char cVar6;
  char extraout_DL;
  u8 bVar7;
  let mut iVar8: i16;
  let mut unaff_SI: i16;
  let mut uVar9: u16;
  u8 bVar10;

  bVar7 = (u8)(param_9 >> 0x8);
  bVar10 = (u8)param_9 + bVar7;
  cVar6 = bVar10 + param_3;
  uVar2 = (CARRY1((u8)param_9,bVar7) || CARRY1(bVar10,param_3));
  uVar3 = param_1 + 0xeff0;
  bVar10 = param_1 < 0x1010 || uVar3 < uVar2;
  uVar5 = uVar3 - uVar2;
  pcVar4 = (code *)swi(0x4);
  if (SBORROW2(param_1,0x1010) != SBORROW2(uVar3,uVar2)) {
    (*pcVar4)();
    cVar6 = extraout_DL;
  }
  pcVar1 = (param_2 + unaff_SI);
  *pcVar1 = *pcVar1 + cVar6 + (uVar5 < 0x1010 || uVar5 + 0xeff0 < bVar10);
  uVar9 = (param_6 >> 0x10);
  iVar8 = param_6;
  *param_6 = 0x389a;
  (iVar8 + 0x2) = 0x1008;
  (iVar8 + 0x4) = param_8;
  (iVar8 + 0x8) = param_7;
  (iVar8 + 0xc) = 0x0;
  (iVar8 + 0xe) = 0x0;
  (iVar8 + 0x12) = 0x0;
  *param_6 = 0xdd4a;
  (iVar8 + 0x2) = 0x1008;
  return;
}
pub fn struct_1008_dc90(param_1: *mut astruct_212,mut param_2: u32,mut param_3: u32)

{
  astruct_212 *iVar1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_212 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  iVar1->field2_0x4 = param_3;
  iVar1->field3_0x8 = param_2;
  iVar1->field4_0xc = 0x0;
  iVar1->field5_0xe = 0x0;
  iVar1->field6_0x12 = 0x0;
  param_1->field0_0x0 = 0xdd4a;
  iVar1->field1_0x2 = 0x1008;
  return;
}
pub fn struct_1008_dcdc(param_1: *mut astruct_220)

{
  astruct_220 *iVar1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_220 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  iVar1->field2_0x4 = 0x0;
  iVar1->field3_0x8 = 0x0;
  iVar1->field4_0xc = 0x0;
  iVar1->field5_0xe = 0x0;
  iVar1->field6_0x12 = 0x0;
  param_1->field0_0x0 = 0xdd4a;
  iVar1->field1_0x2 = 0x1008;
  return;
}



u16 * pass1_1008_dd1e(param_1: *mut u16,param_2: u8)

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1008_dd4e(param_1: *mut astruct_19,mut param_2: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  let mut uVar4: u16;
  let mut paVar3: *mut Struct57;
  astruct_19 *paVar5;

  uVar4 = (in_EDX >> 0x10);
  paVar5 = struct_op_1010_1d48(param_1,param_2);
  paVar3 = (astruct_57 *)CONCAT22(uVar4,(paVar5 >> 0x10));
  uVar1 = 0x0;
  (param_1 + 0xa) = 0x0;
  (param_1 + 0xe) = 0x0;
  (param_1 + 0x12) = 0x0;
  (param_1 + 0x16) = 0x0;
  (param_1 + 0x1a) = 0x0;
  (param_1 + 0x1e) = 0x0;
  param_1->offset_0x0 = 0xeaac;
  (param_1 + 0x2) = 0x1008;
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | uVar1;
  if (uVar2 == 0x0) {
    (param_1 + 0xa) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar3,uVar1));
    (param_1 + 0xa) = uVar1;
    (param_1 + 0xc) = uVar2;
  }
  return;
}
pub fn pass1_1008_ddca(param_1: *mut astruct_455)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  code **ppcVar4;
  astruct_455 *iVar5;
  astruct_455 *uVar4;

  uVar4 = (astruct_455 *)(param_1 >> 0x10);
  iVar5 = (astruct_455 *)param_1;
  param_1->field0_0x0 = 0xeaac;
  iVar5->field1_0x2 = 0x1008;
  puVar1 = iVar5[0x1].field3_0x6;
  uVar2 = (iVar5 + 0x2)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  puVar1 = iVar5[0x2].field1_0x2;
  puVar3 = iVar5[0x2].field2_0x4;
  if ((puVar3 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  puVar1 = iVar5[0x1].field1_0x2;
  puVar3 = iVar5[0x1].field2_0x4;
  if ((puVar3 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  fn_ptr_1000_17ce(*(char **)&iVar5[0x3].field3_0x6);
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_de58(param_1: *mut astruct_211,i32 param_2,i32 param_3)

{
  code **ppcVar1;
  let mut bVar2: bool;
  let mut pstring_1: *mut c_char;
  let mut uVar3: u16;
  let mut in_EDX: *mut Struct57;
  astruct_211 *pstruct211_1;
  let mut pcVar4: *mut c_char;
  astruct_211 *pstruct211_2;
  let mut uVar5: u32;
  char local_a [0x8];

  pstruct211_2 = (astruct_211 *)(param_1 >> 0x10);
  pstruct211_1 = (astruct_211 *)param_1;
  pass1_1008_5784(CONCAT22(0x1050,local_a),pstruct211_1->field10_0xa);
  bVar2 = false;
  do {
    pstring_1 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,pstring_1));
    uVar3 = in_EDX;
    in_EDX = (astruct_57 *)(in_EDX & 0xffff0000 | (uVar3 | pstring_1));
    pcVar4 = pstring_1;
    if ((uVar3 | pstring_1) == 0x0) goto LAB_1008_dedb;
  } while (((*(i32 *)(pstring_1 + 0x4) != param_3) || (*(i32 *)(pstring_1 + 0x8) != param_2)) &&
          ((*(i32 *)(pstring_1 + 0x8) != param_3 || (*(i32 *)(pstring_1 + 0x4) != param_2))));
  (pstring_1 + 0xc) = 0x1;
  uVar5 = pass1_1030_8326();
  in_EDX = (astruct_57 *)(in_EDX & 0xffff0000 | uVar5 >> 0x10);
  pcVar4 = uVar5;
  *(char **)(pstring_1 + 0xe) = pcVar4;
  (pstring_1 + 0x10) = (uVar5 >> 0x10);
  bVar2 = true;//
LAB_1008_dedb:
  if (!bVar2) {
    mem_op_1000_179c(0x14,in_EDX);
    uVar3 = in_EDX | pcVar4;
    if (uVar3 == 0x0) {
      pcVar4 = NULL;
      uVar3 = 0x0;
    }
    else {
      struct_1008_dc90((astruct_212 *)CONCAT22(in_EDX,pcVar4),param_2,param_3);
    }
    (pcVar4 + 0xc) = 0x1;
    uVar5 = pass1_1030_8326();
    (pcVar4 + 0xe) = uVar5;
    (pcVar4 + 0x10) = (uVar5 >> 0x10);
    ppcVar1 = (code **)(*pstruct211_1->field10_0xa + 0x4);
    (**ppcVar1)();
  }
  return;
}
pub fn pass1_1008_df4a(param_1: *mut astruct_102,mut param_2: i16,mut param_3: u16 )

{
  let mut uVar1: u16;
  astruct_102 *paVar2;
  let mut uVar3: u32;
  char local_a [0x8];

  paVar2 = (astruct_102 *)(param_1 >> 0x10);
  pass1_1008_5784(CONCAT22(0x1050,local_a),&((astruct_102 *)param_1)->field_0xa);
  while( true ) {
    uVar3 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    uVar1 = (uVar3 >> 0x10);
    if (uVar3 == 0x0) break;
    if (((uVar3 + 0xc) == 0x2) || ((uVar3 + 0xc) == 0x3)) {
      pass1_1008_e9a4((astruct_102 *)param_1,paVar2,uVar3);
    }
  }
  return;
}
pub fn pass1_1008_dfa6(mut param_1: u32,i32 param_2,i32 param_3)

{
  u8 *puVar1;
  let mut extraout_DX: u16;
  u8 local_a [0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0xa));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,puVar1));
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
  } while (((*(i32 *)(puVar1 + 0x4) != param_3) || (*(i32 *)(puVar1 + 0x8) != param_2)) &&
          ((*(i32 *)(puVar1 + 0x8) != param_3 || (*(i32 *)(puVar1 + 0x4) != param_2))));
  if ((puVar1 + 0xc) != 0x1) {
    return;
  }
  return;
}
