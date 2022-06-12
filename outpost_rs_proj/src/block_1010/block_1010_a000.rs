
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_a0a0(mut param_1: u16 ,param_2: *mut astruct_252)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u32;
  let mut bVar10: bool;
  let mut bVar11: bool;
  let mut puVar12: *mut u32;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffe4: u16;
  let mut iStack12: i16;
  u8 local_a [0x8];

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_2 + 0xa));
  iStack12 = 0x4;
  puVar12 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe4,0x2),in_stack_0000fe8c,
                            in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  uVar9 = puVar12 >> 0x10;
  if ((PTR_LOOP_1050_13ae != &u16_1050_0002) &&
     (PTR_LOOP_1050_13ae != (&PTR_LOOP_1050_0000 + 0x1))) {
    iStack12 = 0x2;
  }
  do {
    while( true ) {
      uVar6 = uVar9;
      uVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
      uVar9 = (uVar6 | uVar4);
      if ((uVar6 | uVar4) == 0x0) {
        return;
      }
      iVar2 = (uVar4 + 0x4);
      if (iVar2 != 0x12) break;
      piVar1 = (uVar4 + 0x6);
      bVar11 = SBORROW2(*piVar1,0x2);
      iVar3 = *piVar1 + -0x2;
      bVar10 = iVar3 == 0x0;//
LAB_1010_a151:
      if (!bVar10 && bVar11 == iVar3 < 0x0) {//
LAB_1010_a153:
        iVar2 = (uVar4 + 0x6);
        uVar9 = (iVar2 % iStack12);
        piVar1 = (uVar4 + 0x6);
        *piVar1 = *piVar1 - iVar2 / iStack12;
      }
    }
    if (((iVar2 != 0x3e) && (iVar2 != 0x41)) && (iVar2 != 0x80)) {
      if (iVar2 == 0x83) {
        piVar1 = (uVar4 + 0x6);
        bVar11 = SBORROW2(*piVar1,0x1);
        iVar2 = *piVar1;
        iVar3 = iVar2 + -0x1;
        bVar10 = iVar2 == 0x1;
        goto LAB_1010_a151;
      }
      goto LAB_1010_a153;
    }
    iVar2 = (uVar4 + 0x6);
    uVar7 = iVar2 >> 0xf;
    uVar5 = iVar2 / 0x2;
    piVar1 = (uVar4 + 0x6);
    *piVar1 = *piVar1 - uVar5;
    if (uVar5 == 0x0) {
      uVar5 = 0x1;
    }
    uVar9 = uVar7;
    pass1_1010_9fee((StructD *)CONCAT22(uVar5,uVar7),param_2,uVar5,(uVar4 + 0x4));
  } while( true );
}



u16 * pass1_1010_a172(param_1: *mut u16,param_2: u8)

{
  pass1_1010_95f8((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1010_a198(param_1: *mut u16,param_2: u8)

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1010_a1d8(param_1: *mut astruct_19,mut param_2: u16 )

{
  let mut iVar1: i16;
  code **ppcVar2;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut unaff_SI: u16;
  astruct_19 *paVar4;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe4e: u16;
  let mut in_stack_0000ff72: u16;
  let mut in_stack_0000ff78: u16;
  let mut in_stack_0000ff7c: u16;
  let mut uStack4: u16;

  uVar3 = (in_EDX >> 0x10);
  paVar4 = struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0x389a;
  (param_1 + 0xc) = 0x1008;
  (param_1 + 0xa) = 0x3aa8;
  (param_1 + 0xc) = 0x1008;
  (param_1 + 0x138) = 0x0;
  param_1->offset_0x0 = 0xe9cc;
  (param_1 + 0x2) = 0x1010;
  (param_1 + 0xa) = 0xe9dc;
  (param_1 + 0xc) = 0x1010;
  puVar5 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar3,(paVar4 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fe4e,in_stack_0000ff72,in_stack_0000ff78,
                           in_stack_0000ff7c);
  (param_1 + 0x138) = puVar5;
  (param_1 + 0x13a) = (puVar5 >> 0x10);
  ppcVar2 = (code **)((param_1 + 0x138) + 0x4);
  (**ppcVar2)();
  pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | (param_1 + 0xa4)),NULL,0x94);
  pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | (param_1 + 0xe)),NULL,0x96);
  uStack4 = 0x0;
  do {
    iVar1 = param_1 + uStack4 * 0x6;
    *(code **)(iVar1 + 0xe) = pass1_1010_c7e2;
    (iVar1 + 0x12) = 0x0;
    uStack4 += 0x1;
  } while (uStack4 < 0x19);
  *(code **)(param_1 + 0x4a) = pass1_1010_c864;
  (param_1 + 0x4e) = 0x0;
  *(code **)(param_1 + 0x50) = pass1_1010_cc56;
  (param_1 + 0x54) = 0x0;
  *(code **)(param_1 + 0x56) = pass1_1010_cf36;
  (param_1 + 0x5a) = 0x0;
  *(code **)(param_1 + 0x2c) = pass1_1010_d24a;
  (param_1 + 0x30) = 0x0;
  *(code **)(param_1 + 0x6e) = pass1_1010_d448;
  (param_1 + 0x72) = 0x0;
  *(code **)(param_1 + 0x74) = pass1_1010_d5ae;
  (param_1 + 0x78) = 0x0;
  *(code **)(param_1 + 0x98) = pass1_1010_d710;
  (param_1 + 0x9c) = 0x0;
  return;
}
pub fn pass1_1010_a478(StructD *param_1)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  StructD *uVar3;
  let mut uVar4: u16;
  let mut puStack6: *mut u16;

  uVar4 = (param_1 >> 0x10);
  uVar3 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xe9cc;
  uVar3->address_offset_field_0x2 = 0x1010;
  uVar3->field6_0xa = 0xe9dc;
  uVar3->field7_0xc = (astruct_589 *)0x1010;
  if (*(i32 *)&uVar3[0x1].field_0x4a != 0x0) {
    if (param_1 == NULL) {
      puVar1 = NULL;
      uVar2 = 0x0;
    }
    else {
      puVar1 = &uVar3->field6_0xa;
      uVar2 = uVar4;
    }
    pass1_1010_1ea6(&uVar3[0x1].field_0x4a,(StructD *)CONCAT22(uVar2,puVar1));
  }
  &uVar3[0x1].field_0x4a = 0x0;
  if (param_1 == NULL) {
    puVar1 = NULL;
    uVar4 = 0x0;
  }
  else {
    puVar1 = &uVar3->field6_0xa;
  }
  puStack6 = (u16 *)CONCAT22(uVar4,puVar1);
  *puStack6 = 0x389a;
  puVar1[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_a50c(param_1: *mut astruct_20,u8 **param_2,StructD *param_3)

{
  let mut iVar1: i16;
  astruct_20 *struct_1;
  let mut local_8: u32;
  let mut iStack4: i16;

  struct_1 = (astruct_20 *)param_1;
  struct_1 = (astruct_20 *)&struct_1->field133_0xa4;
  pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | ZEXT24(struct_1)),NULL,0x94);
  iVar1 = (param_3 + 0xa);
  local_8 = (&struct_1->field6_0xe + iVar1 * 0x3);
  iStack4 = (&struct_1->field7_0x10 + iVar1 * 0x6 + 0x2);
  ((code)local_8)(0x1000,&struct_1->offset_0x0 + iStack4,param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_a568(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  pass1_1030_8344(_u16_1050_5748,0x8000001);
  pass1_1030_2622(CONCAT22(param_2,param_1),param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_a58a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  pass1_1030_8344(_u16_1050_5748,0x8000001);
  pass1_1030_266c(param_1,CONCAT22(param_5,param_2));
  return;
}



u16 pass1_1010_a5ac(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar1: u32;

  uVar1 = struct_op_1030_73a8((astruct_419 *)param_3,in_AX,in_DX);
  return (uVar1 + 0x20);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_a5ca(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  pass1_1030_8344(_u16_1050_5748,0x8000001);
  pass1_1030_2242((astruct_168 *)CONCAT22(param_2,param_1),param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_a5ec(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut extraout_DX: u16;
  let mut puVar6: *mut u32;
  let mut uStack6: u32;

  uVar2 = param_5 | param_5;
  if (param_5 != 0x0) {
    pass1_1030_8344(_u16_1050_5748,0x8000001);
    uStack6 = CONCAT22(param_1,uVar2);
    puVar6 = struct_op_1030_73a8((astruct_419 *)param_5,uVar2,param_1);
    uVar5 = (puVar6 >> 0x10);
    uVar4 = (puVar6 + 0x20);
    if (uVar4 != param_4) {
      uVar3 = param_4;
      pass1_1010_a5ca(param_4,uVar5,param_2,param_3,uVar4);
      if ((uVar4 != 0x70) && (uVar3 < 0x0)) {
        pass1_1030_25d8(CONCAT22(param_1,uVar2),uVar3 + 0x1,uVar4);
      }
      ppcVar1 = (code **)(*puVar6 + 0x8);
      uVar4 = param_4;
      (**ppcVar1)();
      if (param_4 != 0x70) {
        pass1_1010_a5ca(uVar4,extraout_DX,param_2,param_3,param_4);
        if (uVar4 < 0x0) {
          pass1_1030_25d8(uStack6,uVar4 - 0x1,param_4);
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_a69c(mut param_1: i16,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  u8 *puVar4;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut puVar6: *mut u32;
  astruct_25 *paVar7;
  astruct_67 *paVar8;
  astruct_27 *paVar9;
  let mut in_stack_0000fe74: u16;
  let mut in_stack_0000fe76: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000fe7a: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffa0: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffa6: u16;
  let mut in_stack_0000ffa8: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut in_stack_0000ffd2: u16;
  let mut uStack22: u16;
  let mut iStack20: i16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  pass1_1030_8344(_u16_1050_5748,0x8000001);
  uVar3 = SUB42(paVar5,0x0);
  if (param_4 == 0x1) {
    for (iStack20 = 0x0; iStack20 < 0x83; iStack20 += 0x1) {
      iVar1 = pass1_1030_2242((astruct_168 *)CONCAT22(uVar3,param_1),iStack20);
      if (0x19 < iVar1) {
        uStack22 = iVar1 - 0x5;
        if (uStack22 < 0x19) {
          uStack22 = 0x19;
        }
        pass1_1030_25d8(CONCAT22(uVar3,param_1),uStack22,iStack20);
      }
    }
    goto switchD_1010_aaef_caseD_b;
  }
  pass1_1030_25f0(CONCAT22(uVar3,param_1),0x0,param_4);
  puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffd2,0x35),in_stack_0000fe7a,
                           in_stack_0000ff9e,in_stack_0000ffa4,in_stack_0000ffa8);
  paVar5 = (astruct_57 *)(paVar5 & 0xffff0000 | puVar6 >> 0x10);
  puVar4 = (puVar6 >> 0x10);
  uVar2 = param_3;
  uVar10 = (param_3 >> 0x10);
  switch(param_4) {
  case 0xa:
  case 0xc:
    iVar1 = 0x1b;
    break;
  default:
    goto switchD_1010_aaef_caseD_b;
  case 0x10:
    pass1_1010_682e(puVar6,0x1,0x2d);
    if ((param_1 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x2d;
    goto LAB_1010_a91f;
  case 0x12:
    pass1_1010_682e(puVar6,0x1,0x16);
    pass1_1010_682e(puVar6,0x1,0x17);
    pass1_1010_682e(puVar6,0x1,0x18);
    pass1_1010_682e(puVar6,0x1,0x40);
    iVar1 = 0x3f;
    goto LAB_1010_a96c;
  case 0x13:
    iVar1 = 0x35;
    goto LAB_1010_a91f;
  case 0x19:
    goto switchD_1010_aaef_caseD_19;
  case 0x1a:
    iVar1 = 0xf;
    goto LAB_1010_a96c;
  case 0x1c:
    iVar1 = 0x11;
    goto LAB_1010_a96c;
  case 0x1d:
  case 0x24:
    pass1_1010_abd2(puVar4,uVar2,uVar10,0x1e);
    iVar1 = 0x5b;
    goto LAB_1010_a91f;
  case 0x1e:
    uVar2 = 0x1;
    iVar1 = 0x2;
    puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe76,in_stack_0000ff9a,
                             in_stack_0000ffa0,in_stack_0000ffa4);
    paVar5 = (astruct_57 *)(paVar5 & 0xffff0000 | puVar6 >> 0x10);
    pass1_1010_08c0(puVar6,uVar2,iVar1);
    paVar7 = (astruct_25 *)
             mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffd2,0x40),in_stack_0000fe7a,
                             in_stack_0000ff9e,in_stack_0000ffa4,in_stack_0000ffa8);
    paVar5 = (astruct_57 *)(paVar5 & 0xffff0000 | paVar7 >> 0x10);
    load_str_and_sprintf_1008_b69c((paVar7 >> 0x10),paVar7);
    goto switchD_1010_aaef_caseD_b;
  case 0x22:
    iVar1 = 0x8;
    goto LAB_1010_aabe;
  case 0x23:
    iVar1 = 0xc;
    goto LAB_1010_aabe;
  case 0x25:
    pass1_1010_abd2(puVar4,uVar2,uVar10,0x14);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x1b);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x1e);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x22);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x25);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x28);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x2a);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x2d);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x2f);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x31);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x35);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x38);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x3a);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x3c);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x48);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x4f);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x52);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x54);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x57);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x5b);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x5d);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x62);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x66);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x68);
    pass1_1010_abd2(paVar5,uVar2,uVar10,0x6c);
    goto switchD_1010_aaef_caseD_19;
  case 0x29:
    iVar1 = 0x25;
    break;
  case 0x2a:
    iVar1 = 0xf;
    goto LAB_1010_aabe;
  case 0x2b:
    iVar1 = 0x6e;
    goto LAB_1010_a96c;
  case 0x30:
    iVar1 = 0x54;
    break;
  case 0x33:
    pass1_1010_abd2(puVar4,uVar2,uVar10,0x31);
    iVar1 = 0x6c;
    goto LAB_1010_a91f;
  case 0x36:
    iVar1 = 0x13;
    goto LAB_1010_aabe;
  case 0x37:
    iVar1 = 0x2c;//
LAB_1010_a96c:
    pass1_1010_682e(puVar6,0x1,iVar1);
    goto switchD_1010_aaef_caseD_b;
  case 0x38:
    pass1_1010_682e(puVar6,0x1,0x28);
    if ((param_1 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x28;
    goto LAB_1010_a91f;
  case 0x39:
    iVar1 = 0x10;
    goto LAB_1010_aabe;
  case 0x3a:
    iVar1 = 0x11;
    goto LAB_1010_aabe;
  case 0x3b:
    iVar1 = 0x12;//
LAB_1010_aabe:
    pass1_1010_6814(puVar6,0x1,iVar1);
    goto switchD_1010_aaef_caseD_b;
  case 0x3c:
    pass1_1010_abd2(puVar4,uVar2,uVar10,0x14);
    iVar1 = 0x62;
    goto LAB_1010_a91f;
  case 0x3d:
    pass1_1010_682e(puVar6,0x1,0x66);
    if ((param_1 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x66;//
LAB_1010_a91f:
    pass1_1010_abd2(paVar5,uVar2,uVar10,iVar1);
    goto switchD_1010_aaef_caseD_b;
  case 0x3e:
    iVar1 = 0x5d;
    break;
  case 0x3f:
    iVar1 = 0x22;
    break;
  case 0x40:
    iVar1 = 0x57;
    break;
  case 0x41:
    iVar1 = 0x4f;
  }
  pass1_1010_abd2(puVar4,uVar2,uVar10,iVar1);
switchD_1010_aaef_caseD_b:
  paVar8 = (astruct_67 *)
           mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(param_4,0x37),in_stack_0000fe78,
                           in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
  paVar5 = (astruct_57 *)(paVar5 & 0xffff0000 | paVar8 >> 0x10);
  uVar2 = pass1_1008_ab12(paVar8,(paVar8 >> 0x10),param_4);
  if (uVar2 != 0x0) {
    post_win_msg_1008_a0e4(paVar8,0x0,0x0,0x1,0x0,uVar2);
  }
  post_win_msg_1008_a0e4(paVar8,0x0,0x0,0x1,0x0,0x3d);
  uVar11 = 0x400;
  iVar1 = 0x6;
  uVar3 = 0x1;
  paVar9 = (astruct_27 *)
           mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe74,in_stack_0000ff98,
                           in_stack_0000ff9e,in_stack_0000ffa2);
  pass1_1010_043a(paVar9,CONCAT22(uVar11,uVar3),iVar1);
  return;
switchD_1010_aaef_caseD_19:
  (puVar6 + 0x148) = 0x34;
  goto switchD_1010_aaef_caseD_b;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_abd2(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  let mut bVar1: bool;
  let mut piVar2: *mut i16;
  let mut in_register_0000000a: u16;
  let mut unaff_SI: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe8a: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb8: u16;
  let mut piStack20: *mut i16;
  let mut iStack16: i16;
  let mut iStack14: i16;

  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_SI,0x35),in_stack_0000fe8a,in_stack_0000ffae,in_stack_0000ffb4,
                           in_stack_0000ffb8);
  bVar1 = false;
  iStack16 = param_4;
  piStack20 = CONCAT22(0x1050,&stack0x000a);
  while( true ) {
    piVar2 = piStack20;
    if (iStack16 == 0x0) {
      return;
    }
    if (bVar1) break;
    if ((iStack16 * 0x2 + puVar3 + 0xa) != 0x0) {
      bVar1 = true;
      iStack14 = iStack16;
    }
    piStack20 = (piStack20 & 0xffff0000 | (piStack20 + 0x2));
    iStack16 = *piVar2;
  }
  pass1_1010_682e(puVar3,0x0,iStack14);
  pass1_1010_682e(puVar3,0x1,iStack16);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1010_ac62(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16)

{
  pass1_1030_8344(_u16_1050_5748,0x8000001);
  return (param_1 + 0x116 + param_5 * 0x2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * load_string_1010_ac92(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16)

{
  let mut pcVar1: *mut c_char;

  if ((0x0 < param_3) && (param_3 < 0x43)) {
    pcVar1 = load_string_1010_847e(_u16_1050_14cc,param_3 + 0x664);
    return pcVar1;
  }
  return NULL;
}



u16 pass1_1010_acc0(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar1: u32;

  uVar1 = struct_op_1030_73a8((astruct_419 *)param_3,in_AX,in_DX);
  if ((uVar1 + 0x12) != 0x4) {
    return 0x1;
  }
  return 0x0;
}
pub fn pass1_1010_acec(mut param_1: u32,mut param_2: i16)

{
  if (param_2 == 0x1) {
    (param_1 + 0x12e) = 0x0;
  }
  else if (param_2 != 0x5) {
    return;
  }
  pass1_1010_1f62((astruct_27 *)(param_1 & 0xffff0000 | (param_1 - 0xa)),param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_ad22(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 )

{
  let mut uVar1: u32;
  let mut puVar2: *mut u16;

  uVar1 = (param_2 + 0x138);
  puVar2 = &param_4;
  pass1_1030_627e(puVar2,param_1,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),
                  *(i32 *)(uVar1 + 0x20));
  if ((param_1 | puVar2) == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_1,puVar2));
  return;
}
pub fn pass1_1010_ad64(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: u32)

{
  if (param_5 != 0x0) {
    param_1 = (param_5 + 0x2e);
    if (*(i32 *)(param_1 + 0x200) == 0x8000002) {
      return;
    }
  }
  pass1_1010_c58as(param_1,param_2,(astruct_20 *)CONCAT22(param_4,param_3),(param_4 >> 0x10),
                   param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * string_op_1010_ada6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,mut param_5: i16)

{
  let mut pcVar1: *mut c_char;
  let mut pcStack6: *mut c_char;

  pcStack6 = NULL;
  if (param_5 == 0x6) {
    if (param_4 == 0x0) goto LAB_1010_adee;
    pcVar1 = string_op_1020_c222(param_4);
  }
  else {
    if (param_5 != 0x7) {
      return NULL;
    }
    if (param_4 == 0x0) goto LAB_1010_adee;
    pcVar1 = string_op_1020_c2f8(param_4);
  }
  pcStack6 = CONCAT22(param_1,pcVar1);//
LAB_1010_adee:
  if (pcStack6 == NULL) {
    pcStack6 = load_string_1010_847e(_u16_1050_14cc,0x531);
  }
  return pcStack6;
}



u16 pass1_1010_ae12(char *param_1,mut param_2: u16 ,mut param_3: u16 ,char *param_4,mut param_5: i16)

{
  let mut pcVar1: *mut c_char;
  let mut iVar2: i16;
  let mut uStack4: u16;

  if (param_5 == 0x6) {
    for (uStack4 = 0x0; uStack4 < 0x15; uStack4 += 0x1) {
      pcVar1 = string_op_1020_c222(uStack4);
      iVar2 = pass1_1000_3d7a(param_4,CONCAT22(param_1,pcVar1));
      if (iVar2 == 0x0) {
        return uStack4;
      }
    }
  }
  else if (param_5 == 0x7) {
    for (uStack4 = 0x0; uStack4 < 0x11; uStack4 += 0x1) {
      pcVar1 = string_op_1020_c2f8(uStack4);
      iVar2 = pass1_1000_3d7a(param_4,CONCAT22(param_1,pcVar1));
      if (iVar2 == 0x0) {
        return uStack4;
      }
    }
  }
  return 0xffff;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_ae92(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: u32)

{
  u8 bVar1;
  let mut uVar2: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  astruct_15 *paVar6;
  let mut uVar7: u32;
  astruct_67 *paVar8;
  let mut in_stack_0000fe88: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb6: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut iVar11: i16;
  u8 uVar12;
  u8 uVar13;
  let mut uVar14: u16;
  let mut uVar15: u16;
  let mut iVar16: i16;
  let mut uVar3: u16;

  if (param_3 == 0x15) {
    uVar7 = struct_op_1030_73a8((astruct_419 *)param_4,0x15,param_5);
    uVar3 = (uVar7 >> 0x10);
    if ((uVar3 | uVar7) != 0x0) {
      (uVar7 + 0x20) = param_2;
      return;
    }
  }
  else if (param_3 < 0x16) {
    bVar1 = (char)param_3 - 0x6;
    uVar3 = param_3 & 0xff00 | bVar1;
    if (bVar1 == 0x0) {
      pass1_1030_7f1a(param_4,param_2);
      paVar6 = (astruct_15 *)struct_op_1030_73a8((astruct_419 *)param_4,uVar3,param_5);
      uVar5 = (param_5 >> 0x10);
      uVar2 = pass1_1010_b028(param_1,(param_1 >> 0x10),paVar6);
      uVar7 = pass1_1030_8326();
      iVar4 = (uVar7 >> 0x10);
      if (((uVar2 == 0xe) && ((iVar4 != 0x0 || (0x32 < uVar7)))) &&
         ((param_2 == 0x1 || (((param_2 == 0x2 || (param_2 == 0x4)) || (param_2 == 0x3)))))) {
        uVar15 = 0x0;
        iVar16 = 0xb;
        uVar12 = 0x1;
        uVar13 = 0x0;
        uVar14 = 0x0;
        uVar10 = 0x0;
        iVar11 = 0x0;
        uVar9 = 0x0;
        paVar8 = (astruct_67 *)
                 mixed_1010_20ba((astruct_57 *)CONCAT22(uVar5,iVar4),_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe88
                                 ,in_stack_0000ffac,in_stack_0000ffb2,in_stack_0000ffb6);
        post_win_msg_1008_a0e4
                  (paVar8,CONCAT22(uVar10,uVar9),iVar11,CONCAT11(uVar13,uVar12),CONCAT22(uVar15,uVar14),iVar16);
        return;
      }
    }
    else if ((char)param_3 == '\a') {
      pass1_1030_7eda(param_4,param_2);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_af66(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u16;

  uVar1 = (param_2 + 0x138);
  uVar2 = (uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  uVar3 = param_1 | uVar2;
  if (uVar3 == 0x0) {
    return;
  }
  pass1_1038_5050(uVar2,uVar3,uVar2 & 0xffff | param_1 << 0x10,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_afa2(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u32;

  uVar1 = (param_2 + 0x138);
  uVar2 = (uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  if ((param_1 | uVar2) == 0x0) {
    return;
  }
  pass1_1038_50e0(uVar2,uVar2 & 0xffff | param_1 << 0x10,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_afde(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut in_EDX: u32;
  let mut uVar3: u32;
  let mut puVar4: *mut u32;

  uVar1 = (param_1 + 0x138);
  uVar2 = (uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  uVar3 = in_EDX & 0xffff0000;
  if ((in_EDX | uVar2) == 0x0) {
    return;
  }
  puVar4 = pass1_1008_c6fa(_u16_1050_06e0,param_2);
  pass1_1038_4e78(puVar4,(astruct_57 *)(uVar3 & 0xffff0000 | puVar4 >> 0x10),
                  uVar2 & 0xffff | in_EDX << 0x10,puVar4);
  return;
}
