
pub fn pass1_1020_808e(StructD *param_1)

{
  let mut puVar1: *mut u8;
  let mut uVar2: u16;
  StructD *iVar3;
  let mut uVar3: u16;
  let mut puStack6: *mut u16;

  uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  param_1->address_offset_field_0x0 = 0x82bc;
  iVar3->address_offset_field_0x2 = 0x1020;
  &iVar3->field_0xe2 = 0x8358;
  &iVar3->field_0xe4 = 0x1020;
  if (param_1 == NULL) {
    puVar1 = NULL;
    uVar2 = 0;
  }
  else {
    puVar1 = &iVar3->field_0xe2;
    uVar2 = uVar3;
  }
  puStack6 = CONCAT22(uVar2,puVar1);
  *puStack6 = 0x389a;
  (puVar1 + 0x2) = 0x1008;
  pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(&iVar3->field192_0xd2)));
  param_1->address_offset_field_0x0 = 0x380a;
  iVar3->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar3->address_offset_field_0x2 = 0x1008;
  return;
}
pub fn pass1_1020_8106(mut param_1: u32)

{
  code **ppcVar1;

  ppcVar1 = (code **)((param_1 + 0x4) + 0x60);
  (**ppcVar1)();
  return;
}
pub fn realize_palette_1020_8128(mut param_1: u32,mut param_2: i16)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut puVar3: *mut u8;
  let mut puVar4: *mut u32;
  let mut puVar5: *mut u32;
  let mut extraout_DX: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  u8 local_12 [0x8];
  HDC16 hdc_10;
  HGDIOBJ16 HStack8;
  let mut puStack6: *mut u32;

  if (param_2 != 0) {
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    uVar2 = (iVar6 + 0xe6);
    puVar5 = (uVar2 + 0xa);
    ppcVar1 = (code **)(*puVar5 + 0x18);
    puStack6 = puVar5;
    (**ppcVar1)();
    HStack8 = (HGDIOBJ16)puVar5;
    UnrealizeObject16(HStack8);
    uVar2 = (iVar6 + 0xe6);
    hdc_10 = *(HDC16 *)(uVar2 + 0x14);
    RealizePalette16(hdc_10);
    pass1_1008_57a4(CONCAT22(0x1050,local_12),param_1 & 0xffff0000 | (iVar6 + 0xd2));
    while( true ) {
      puVar3 = local_12;
      pass1_1008_5b12(CONCAT22(0x1050,puVar3));
      if ((extraout_DX | puVar3) == 0) break;
      uVar2 = (puVar3 + 0x4);
      uVar7 = (puVar3 + 0x6);
      puVar4 = uVar2;
      ppcVar1 = (code **)(*puVar4 + 0x90);
      (**ppcVar1)(0x1008,puVar4,uVar7,0x1,uVar2);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_palette_op_1020_81c0(mut param_1: u32)

{
  astruct_13 *in_struct_1;
  HDC16 hdc;
  HDC16 hpal;
  HPALETTE16 hpal_00;
  let mut uVar1: u16;
  let mut uVar2: u16;
  HDC16 hdc_00;
  let mut uStack6: u16;

  uVar2 = (_PTR_LOOP_1050_4230 >> 0x10);
  in_struct_1 = *(astruct_13 **)(_PTR_LOOP_1050_4230 + 0xe);
  uVar1 = (_PTR_LOOP_1050_4230 + 0x10);
  uStack6 = in_struct_1;
  if ((uVar1 | uStack6) == 0) {
    return;
  }
  hdc = GetDC16(*(HWND16 *)(param_1 + 0x8));
  hpal = hdc;
  hdc_00 = hdc;
  create_palette_1008_4e38(in_struct_1,uVar1);
  hpal_00 = SelectPalette16(0x0,hpal,hdc_00);
  RealizePalette16(hdc);
  SelectPalette16(0x1,hpal_00,hdc);
  RealizePalette16(hdc);
  DeleteObject16(hpal);
  if (0x0 < hpal) {
    InvalidateRect16(0x1,NULL,0x0);
  }
  return;
}
pub fn destroy_window_1020_8250(param_1: *mut astruct_879)

{
  let mut BVar1: bool;
  astruct_879 *iVar2;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (astruct_879 *)param_1;
  if (iVar2->field236_0xec != 0) {
    BVar1 = IsWindow16(iVar2->field236_0xec);
    if (BVar1 != 0) {
      DestroyWindow16(iVar2->field236_0xec);
      iVar2->field236_0xec = 0;
    }
  }
  return;
}



StructD * pass1_1020_8288(StructD *param_1,param_2: u8)

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1020_808e(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



StructD * FUN_1020_8296(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1020_808e(param_2);
  if ((param_3 & 1) != 0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_8360(param_1: *mut astruct_20)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut unaff_SI: u16;
  let mut puVar4: *mut u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut uVar6: u16;
  astruct_20 *iVar4;

  uVar3 = (in_EDX >> 0x10);
  iVar4 = (astruct_20 *)param_1;
  uVar6 = (param_1 >> 0x10);
  struct_1020_847a(param_1,1);
  puVar4 = pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | ZEXT24(&iVar4->field9_0x16)));
  &iVar4->field12_0x1c = 0;
  param_1->offset_0x0 = 0x8462;
  iVar4->base_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar3,(puVar4 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_SI,0x29),in_stack_0000fe96,in_stack_0000ffba,in_stack_0000ffc0,
                           in_stack_0000ffc4);
  uVar2 = (puVar5 >> 0x10);
  iVar4->field12_0x1c = puVar5;
  &iVar4->field13_0x1e = uVar2;
  pass1_1018_26f8(iVar4->field12_0x1c,uVar2,(param_1 & 0xffff0000 | ZEXT24(&iVar4->field9_0x16)));
  uVar1 = &iVar4->field12_0x1c;
  pass1_1020_8712(param_1 & 0xffff | uVar6 << 0x10,&iVar4->field3_0x8,
                  *(astruct_76 **)(uVar1 + 0x2a),(param_1 & 0xffff0000 | ZEXT24(&iVar4->field9_0x16))
                 );
  return;
}
pub fn pass1_1020_83f8(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if ((iVar3 + 0x4) != 0) {
    uVar1 = (iVar3 + 0x1c);
    uVar2 = (iVar3 + 0x1c);
    pass1_1008_4480(*(astruct_76 **)(uVar1 + 0xa),(param_1 & 0xffff0000 | (iVar3 + 0x16)),
                    *(astruct_76 **)(uVar2 + 0x2a));
  }
  return;
}
pub fn FUN_1020_8438()

{
  return;
}



StructD * pass1_1020_843c(StructD *param_1,param_2: u8)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1020_847a(param_1: *mut astruct_20,mut param_2: i16)

{
  let mut uVar1: u16;
  astruct_20 *paVar2;
  let mut in_EDX: u32;
  let mut uVar4: u16;
  let mut paVar3: *mut Struct57;
  astruct_20 *iVar3;
  let mut unaff_SI: u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut puVar7: *mut u32;
  let mut in_stack_0000fe92: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc0: u16;

  uVar4 = (in_EDX >> 0x10);
  uVar5 = (param_1 >> 0x10);
  iVar3 = (astruct_20 *)param_1;
  param_1->offset_0x0 = 0x389a;
  iVar3->base_0x2 = 0x1008;
  &iVar3->field2_0x4 = 0;
  (&iVar3->field2_0x4 + 0x2) = param_2;
  &iVar3->field3_0x8 = 0;
  &iVar3->field5_0xc = 0;
  puVar6 = pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | ZEXT24(&iVar3->field7_0x10)));
  paVar3 = (astruct_57 *)CONCAT22(uVar4,(puVar6 >> 0x10));
  param_1->offset_0x0 = 0x87aa;
  iVar3->base_0x2 = 0x1020;
  puVar7 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x48),in_stack_0000fe92,
                           in_stack_0000ffb6,in_stack_0000ffbc,in_stack_0000ffc0);
  paVar3 = (astruct_57 *)(paVar3 & 0xffff0000 | puVar7 >> 0x10);
  pass1_1008_3f62((param_1 & 0xffff0000 | ZEXT24(&iVar3->field7_0x10)),
                  (puVar7 & 0xffff0000 | (puVar7 + 0xe)));
  uVar1 = (&iVar3->field2_0x4 + 0x2) << 0x3;
  mem_op_1000_179c(uVar1,paVar3);
  iVar3->field3_0x8 = uVar1;
  iVar3->field4_0xa = paVar3;
  paVar2 = (astruct_20 *)((&iVar3->field2_0x4 + 0x2) << 0x2);
  mem_op_1000_179c(paVar2,paVar3);
  iVar3->field5_0xc = paVar2;
  iVar3->field6_0xe = paVar3;
  pass1_1000_4906(*(StructD **)&iVar3->field3_0x8,NULL,(&iVar3->field2_0x4 + 0x2) << 0x3);
  pass1_1000_4906(*(StructD **)&iVar3->field5_0xc,NULL,(&iVar3->field2_0x4 + 0x2) << 0x2);
  return;
}



// WARNING: Unable to use type for symbol uVar4
pub fn pass1_1020_8556(StructD *param_1)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut pcVar3: *mut c_char;
  StructD *iVar5;
  astruct_589 *iVar4;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut iStack12: i16;
  let mut uVar4: u32;

  uVar7 = (param_1 >> 0x10);
  iVar5 = param_1;
  param_1->address_offset_field_0x0 = 0x87aa;
  iVar5->address_offset_field_0x2 = 0x1020;
  fn_ptr_1000_17ce(*&iVar5->field5_0x8);
  if ((iVar5->field8_0xe | iVar5->field7_0xc) != 0) {
    iStack12 = 0;
    while( true ) {
      piVar1 = &iVar5->field_0x6;
      if (*piVar1 == iStack12 || *piVar1 < iStack12) break;
      iVar6 = iStack12 * 0x4;
      uVar4 = &iVar5->field7_0xc;
      uVar8 = (uVar4 >> 0x10);
      iVar4 = (astruct_589 *)uVar4;
      if (*(i32 *)(iVar4 + iVar6) != 0) {
        pcVar3 = *(iVar4 + iVar6);
        uVar2 = (iVar4 + iVar6 + 2);
        if ((uVar2 | pcVar3) != 0) {
          pass1_1008_5118(pcVar3 & 0xffff | uVar2 << 0x10);
          fn_ptr_1000_17ce(pcVar3);
        }
      }
      iStack12 += 0x1;
    }
    fn_ptr_1000_17ce(*&iVar5->field7_0xc);
  }
  param_1->address_offset_field_0x0 = 0x389a;
  iVar5->address_offset_field_0x2 = 0x1008;
  return;
}
pub fn pass1_1020_85f6(param_1: *mut astruct_590)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut pcVar3: *mut c_char;
  let mut uVar4: u32;
  let mut iVar5: i16;
  astruct_590 *iVar6;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iStack4: i16;

  iStack4 = 0;
  while( true ) {
    uVar7 = (param_1 >> 0x10);
    iVar6 = (astruct_590 *)param_1;
    piVar1 = &iVar6->field6_0x6;
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    uVar4 = iVar6->field11_0xc;
    uVar6 = (uVar4 >> 0x10);
    iVar5 = uVar4;
    pcVar3 = *(iVar5 + iStack4 * 0x4);
    uVar2 = (iVar5 + iStack4 * 0x4 + 2);
    if ((uVar2 | pcVar3) != 0) {
      pass1_1008_5118(pcVar3 & 0xffff | uVar2 << 0x10);
      fn_ptr_1000_17ce(pcVar3);
    }
    uVar4 = iVar6->field11_0xc;
    (uVar4 + iStack4 * 0x4) = 0;
    iStack4 += 0x1;
  }
  return;
}
pub fn pass1_1020_865a(mut param_1: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut pcVar3: *mut c_char;
  let mut uVar4: u32;
  let mut iVar5: i16;
  astruct_592 *iVar7;
  astruct_591 *iVar6;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut iStack4: i16;

  iStack4 = 0;
  while( true ) {
    uVar9 = (param_1 >> 0x10);
    iVar5 = param_1;
    piVar1 = (iVar5 + 0x6);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    iVar8 = iStack4 * 0x4;
    uVar4 = (iVar5 + 0xc);
    uVar10 = (uVar4 >> 0x10);
    iVar7 = (astruct_592 *)uVar4;
    if (*(i32 *)(iVar7 + iVar8) != 0) {
      pass1_1008_5236(*(astruct_109 **)(iVar7 + iVar8));
      uVar4 = (iVar5 + 0xc);
      uVar10 = (uVar4 >> 0x10);
      iVar6 = (astruct_591 *)uVar4;
      pcVar3 = *(iVar6 + iVar8);
      uVar2 = (iVar6 + iVar8 + 2);
      if ((uVar2 | pcVar3) != 0) {
        pass1_1008_5118(pcVar3 & 0xffff | uVar2 << 0x10);
        fn_ptr_1000_17ce(pcVar3);
      }
      uVar4 = (iVar5 + 0xc);
      (uVar4 + iStack4 * 0x4) = 0;
    }
    iStack4 += 0x1;
  }
  return;
}
pub fn pass1_1020_86d8(mut param_1: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut iStack4: i16;

  iStack4 = 0;
  while( true ) {
    uVar4 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x6);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    uVar2 = (param_1 + 0xc);
    uVar4 = (uVar2 >> 0x10);
    iVar3 = uVar2;
    if (*(i32 *)(iVar3 + iStack4 * 0x4) != 0) {
      pass1_1008_5236(*(astruct_109 **)(iVar3 + iStack4 * 0x4));
    }
    iStack4 += 0x1;
  }
  return;
}
pub fn pass1_1020_8712(mut param_1: u32,param_2: *mut i16,param_3: *mut astruct_76,param_4: *mut u16)

{
  let mut uVar1: u16;
  let mut uVar2: u32;

  pass1_1008_3f32(param_4,(param_1 & 0xffff0000 | (param_1 + 0x10)));
  uVar2 = pass1_1008_4772(param_3);
  uVar1 = (uVar2 >> 0x10);
  pass1_1008_3e94(param_4,(param_2 & 0xffff0000 | ZEXT24((param_2 + 0x2))),
                  (param_2 & 0xffff | param_2 << 0x10));
  (param_2 + 0x4) = (uVar2 + 0x4) + *param_2;
  (param_2 + 0x6) = (uVar2 + 0x8) + (param_2 + 2);
  return;
}
pub fn FUN_1020_8780()

{
  return;
}



StructD * pass1_1020_8784(StructD *param_1,param_2: u8)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_87c2(param_1: *mut astruct_20)

{
  let mut uVar1: u32;
  astruct_20 *iVar2;
  let mut in_EDX: i32;
  let mut uVar2: u16;
  let mut unaff_SI: u16;
  let mut uVar3: u16;
  let mut puVar4: *mut u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  u8 local_12 [0x8];
  let mut iStack10: i16;
  astruct_19 *paStack8;
  let mut iStack4: i16;

  struct_1020_847a(param_1,0x4);
  iStack4 = 0x4;
  iVar2 = (astruct_20 *)param_1;
  iVar2 = (astruct_20 *)&iVar2->field9_0x16;
  paStack8 = (astruct_19 *)(param_1 & 0xffff0000 | ZEXT24(iVar2));
  loop {
    uVar2 = (in_EDX >> 0x10);
    pass1_1008_3e38(paStack8);
    in_EDX = uVar2 << 0x10;
    paStack8 = (astruct_19 *)(paStack8 & 0xffff0000 | (paStack8 + 0x6));
    iStack4 += -0x1;
  } while (iStack4 != 0);
  uVar3 = (param_1 >> 0x10);
  (&iVar2->field17_0x2c + 0x2) = 0;
  puVar4 = pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x32)));
  &iVar2->field_0x38 = 0;
  param_1->offset_0x0 = 0x8a84;
  iVar2->base_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar2,(puVar4 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_SI,0x29),in_stack_0000fe80,in_stack_0000ffa4,in_stack_0000ffaa,
                           in_stack_0000ffae);
  (&iVar2->field17_0x2c + 0x2) = puVar5;
  iVar2->field18_0x30 = (puVar5 >> 0x10);
  iStack10 = 0;
  loop {
    uVar1 = (&iVar2->field17_0x2c + 2);
    pass1_1018_26d8(uVar1,(uVar1 >> 0x10),iStack10,
                    (param_1 & 0xffff0000 | ZEXT24(&iVar2->field9_0x16 + iStack10 * 0x3)));
    uVar1 = (&iVar2->field17_0x2c + 2);
    pass1_1020_8712(param_1 & 0xffff | uVar3 << 0x10,
                    CONCAT22(iVar2->field4_0xa,iVar2->field3_0x8 + iStack10 * 0x8),
                    *(astruct_76 **)(uVar1 + 0x2e + iStack10 * 0x4),
                    (param_1 & 0xffff0000 | ZEXT24(&iVar2->field9_0x16 + iStack10 * 0x3)));
    iStack10 += 0x1;
  } while (iStack10 < 0x4);
  uVar1 = (&iVar2->field17_0x2c + 2);
  pass1_1018_2548(uVar1,(uVar1 >> 0x10),
                  (param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x32)));
  uVar1 = (&iVar2->field17_0x2c + 2);
  &iVar2->field_0x38 = (uVar1 + 0x6e);
  pass1_1020_8712(param_1 & 0xffff | uVar3 << 0x10,CONCAT22(0x1050,local_12),
                  *(astruct_76 **)&iVar2->field_0x38,(param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x32)));
  return;
}
pub fn pass1_1020_8908(param_1: *mut astruct_284,mut param_2: u32,mut param_3: u32)

{
  astruct_76 *paVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut puVar5: *mut u8;
  let mut uVar6: u16;
  let mut uVar7: u16;
  astruct_284 *pstruct284_8;
  let mut iVar9: i16;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u32;
  astruct_110 *paStack28;
  let mut iStack4: i16;
  let mut paVar8: *mut Struct57;

  for (iStack4 = 0; uVar12 = (param_3 >> 0x10), pstruct284_8 = (astruct_284 *)param_1,
      uVar11 = (param_1 >> 0x10), iStack4 < 0x4; iStack4 += 1) {
    if (pstruct284_8->field4_0x4 == 0) {
      uVar2 = pstruct284_8->field11_0xc;
      uVar12 = (uVar2 >> 0x10);
      iVar10 = uVar2;
      iVar9 = iStack4 * 0x4;
      if (((iVar10 + iVar9 + 0x2) | (iVar10 + iVar9)) != 0) {
        pass1_1008_5236(*(astruct_109 **)(iVar10 + iVar9));
      }
    }
    else {
      uVar2 = pstruct284_8->field42_0x2e;
      paVar1 = *(astruct_76 **)(uVar2 + 0x2e + iStack4 * 0x4);
      uVar13 = pass1_1008_4772(paVar1);
      uVar6 = (uVar13 >> 0x10);
      paVar8 = (astruct_57 *)CONCAT22(uVar12,uVar6);
      uVar3 = uVar13;
      uVar2 = pstruct284_8->field11_0xc;
      iVar10 = iStack4 * 0x4;
      if (*(i32 *)(uVar2 + iVar10) == 0) {
        uVar4 = uVar3;
        mem_op_1000_179c(0x14,paVar8);
        uVar7 = paVar8;
        paStack28 = (astruct_110 *)CONCAT22(uVar7,uVar4);
        paVar8 = (astruct_57 *)(paVar8 & 0xffff0000);
        if ((uVar7 | uVar4) == 0) {
          uVar2 = pstruct284_8->field11_0xc;
          (uVar2 + iStack4 * 0x4) = 0;
        }
        else {
          paVar8 = (astruct_57 *)(paVar8 | uVar11);
          puVar5 = &pstruct284_8->field_0x16 + iStack4 * 0x6;
          pass1_1008_50c2(paStack28,(uVar3 + 0x8),(uVar3 + 0x4),
                          (param_1 & 0xffff0000 | ZEXT24(puVar5)),(astruct_76 *)param_2);
          uVar2 = pstruct284_8->field11_0xc;
          uVar12 = (uVar2 >> 0x10);
          iVar9 = uVar2;
          (iVar9 + iVar10) = puVar5;
          (iVar9 + iVar10 + 0x2) = paVar8;
        }
        uVar2 = pstruct284_8->field11_0xc;
        pass1_1008_5134((uVar2 + iStack4 * 0x4));
      }
      uVar12 = (paVar8 >> 0x10);
      uVar2 = pstruct284_8->field11_0xc;
      pass1_1008_5236(*(astruct_109 **)(uVar2 + iStack4 * 0x4));
      param_3 = CONCAT22(uVar12,uVar11);
      pass1_1008_4480((astruct_76 *)param_2,
                      (param_1 & 0xffff0000 | ZEXT24(&pstruct284_8->field_0x16 + iStack4 * 0x6)),paVar1);
    }
  }
  if (pstruct284_8->field4_0x4 != 0) {
    pass1_1008_4480((astruct_76 *)param_2,(param_1 & 0xffff0000 | ZEXT24(&pstruct284_8->field_0x32)),
                    pstruct284_8->field49_0x38);
  }
  return;
}



StructD * pass1_1020_8a5e(StructD *param_1,param_2: u8)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_8a9c(param_1: *mut astruct_20,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,
                    mut param_6: u16 )

{
  astruct_287 *paVar1;
  let mut uVar2: u32;
  astruct_19 *uVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut paVar6: *mut Struct57;
  let mut unaff_SI: u16;
  let mut puVar7: *mut u16;
  let mut puVar8: *mut u32;
  astruct_76 *paVar9;
  let mut uVar10: u16;
  let mut puStack76: *mut u16;
  u8 local_48 [0x1e];
  u8 local_2a [0x24];
  let mut uStack6: u16;
  let mut uStack4: u16;
  astruct_20 *iVar9;
  astruct_20 *uVar9;

  uVar4 = (param_2 >> 0x10);
  iVar9 = (astruct_20 *)param_1;
  uVar10 = (param_1 >> 0x10);
  struct_1020_847a(param_1,0x2);
  uVar3 = (astruct_19 *)&iVar9->field9_0x16;
  pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | ZEXT24(uVar3)));
  puStack76 = (param_1 & 0xffff0000 | ZEXT24(&iVar9->field12_0x1c));
  puVar7 = pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | ZEXT24(&iVar9->field12_0x1c)));
  paVar6 = (astruct_57 *)CONCAT22(uVar4,(puVar7 >> 0x10));
  iVar9->field14_0x22 = NULL;
  param_1->offset_0x0 = 0x8e92;
  iVar9->base_0x2 = 0x1020;
  puVar8 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x29),param_6,param_5,param_3,param_4);
  uVar4 = (paVar6 >> 0x10);
  &iVar9->field14_0x22 = puVar8;
  uVar5 = (puVar8 >> 0x10);
  (&iVar9->field14_0x22 + 0x2) = uVar5;
  pass1_1018_2678(&iVar9->field14_0x22,uVar5,(param_1 & 0xffff0000 | ZEXT24(uVar3)));
  paVar9 = (astruct_76 *)pass1_1018_268e(iVar9->field14_0x22);
  uStack4 = (paVar9 >> 0x10);
  paVar6 = (astruct_57 *)CONCAT22(uVar4,uStack4);
  uVar4 = SUB42(paVar9,0x0);
  uStack6 = uVar4;
  pass1_1020_8712(param_1,&iVar9->field3_0x8,paVar9,(param_1 & 0xffff0000 | ZEXT24(uVar3))
                 );
  paVar1 = iVar9->field14_0x22;
  pass1_1018_26c2(paVar1,(paVar1 >> 0x10),puStack76);
  uVar4 = FUN_1010_830a(uVar4,paVar6,0x1018,_u16_1050_14cc,0x2);
  struct_op_1008_48fe(paVar6,(astruct_81 *)CONCAT22(0x1050,local_2a),0x1,CONCAT22(paVar6,uVar4));
  struct_op_1008_3f92((astruct_76 *)CONCAT22(0x1050,local_48),CONCAT22(0x1050,local_2a));
  uVar2 = &iVar9->field3_0x8;
  pass1_1020_8712(param_1,(uVar2 & 0xffff0000 | (uVar2 + 0x8)),
                  (astruct_76 *)CONCAT22(0x1050,local_48),puStack76);
  pass1_1008_41bc((astruct_288 *)CONCAT22(0x1050,local_48));
  close_file_1008_496c((astruct_803 *)CONCAT22(0x1050,local_2a));
  return;
}
pub fn pass1_1020_8bae(param_1: *mut u16)

{
  *param_1 = 0x8e92;
  (param_1 + 0x2) = 0x1020;
  pass1_1020_8556(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_8bcc(param_1: *mut astruct_285,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 )

{
  let mut uVar1: u32;
  astruct_109 *paVar2;
  let mut puVar3: *mut u16;
  let mut puVar4: *mut u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut paVar7: *mut Struct57;
  astruct_285 *iVar9;
  astruct_286 *iVar10;
  let mut uVar9: u16;
  let mut uVar10: u16;
  u8 local_58 [0x1e];
  u8 local_3a [0x26];
  let mut uStack20: u32;
  let mut uStack12: u16;
  astruct_76 *paStack10;
  astruct_76 *paStack6;
  let mut paVar8: *mut Struct57;

  uVar10 = (param_2 >> 0x10);
  uVar9 = (param_1 >> 0x10);
  iVar9 = (astruct_285 *)param_1;
  if (iVar9->field4_0x4 != 0) {
    uVar1 = iVar9->field30_0x22;
    paStack6 = *(astruct_76 **)(uVar1 + 0xa);
    paStack10 = (astruct_76 *)pass1_1018_268e((astruct_287 *)iVar9->field30_0x22);
    uVar6 = (paStack10 >> 0x10);
    uVar1 = iVar9->field30_0x22;
    uStack12 = (uVar1 + 0x16);
    if (*(i32 *)iVar9->field11_0xc == 0) {
      uStack20 = pass1_1008_4772(paStack10);
      paVar7 = (astruct_57 *)CONCAT22(uVar10,(uStack20 >> 0x10));
      uVar6 = uStack20;
      mem_op_1000_179c(0x14,paVar7);
      uVar5 = paVar7 | uVar6;
      paVar8 = (astruct_57 *)(paVar7 & 0xffff0000 | uVar5);
      if (uVar5 == 0) {
        iVar9->field11_0xc = 0;
      }
      else {
        puVar3 = (param_1 & 0xffff0000 | ZEXT24(&iVar9->field_0x16));
        uVar10 = (uStack20 >> 0x10);
        pass1_1008_50c2((astruct_110 *)CONCAT22(paVar7,uVar6),(uStack20 + 0x8),
                        (uStack20 + 0x4),puVar3,paStack6);
        paVar2 = iVar9->field11_0xc;
        uVar6 = puVar3;
        paVar2 = uVar6;
        (paVar2 + 0x2) = paVar8;
      }
      pass1_1008_5134(iVar9->field11_0xc);
      uVar10 = FUN_1010_830a(uVar6,paVar8,0x1008,_u16_1050_14cc,0x2);
      struct_op_1008_48fe(paVar8,(astruct_81 *)CONCAT22(0x1050,local_3a),0x1,CONCAT22(paVar8,uVar10));
      uVar10 = (paVar8 >> 0x10);
      struct_op_1008_3f92((astruct_76 *)CONCAT22(0x1050,local_58),CONCAT22(0x1050,local_3a));
      uStack20 = pass1_1008_4772((astruct_76 *)CONCAT22(0x1050,local_58));
      paVar7 = (astruct_57 *)CONCAT22(uVar10,(uStack20 >> 0x10));
      uVar6 = uStack20;
      mem_op_1000_179c(0x14,paVar7);
      uVar6 = paVar7 | uVar6;
      if (uVar6 == 0) {
        paVar2 = iVar9->field11_0xc;
        (paVar2 + 0x4) = 0;
      }
      else {
        puVar3 = (param_1 & 0xffff0000 | ZEXT24(&iVar9->field_0x16));
        uVar10 = (uStack20 >> 0x10);
        pass1_1008_50c2((astruct_110 *)CONCAT22(paVar7,(paStack6 >> 0x10)),
                        (uStack20 + 0x8),(uStack20 + 0x4),puVar3,paStack6);
        paVar2 = iVar9->field11_0xc;
        uVar10 = (paVar2 >> 0x10);
        iVar10 = (astruct_286 *)paVar2;
        iVar10->field4_0x4 = puVar3;
        iVar10->field5_0x6 = uVar6;
      }
      paVar2 = iVar9->field11_0xc;
      pass1_1008_5134((paVar2 + 0x4));
      pass1_1008_41bc((astruct_288 *)CONCAT22(0x1050,local_58));
      close_file_1008_496c((astruct_803 *)CONCAT22(0x1050,local_3a));
    }
    paVar2 = iVar9->field11_0xc;
    pass1_1008_5236(*(astruct_109 **)(paVar2 + 0x4));
    pass1_1008_5236(*(astruct_109 **)iVar9->field11_0xc);
    puVar4 = (param_1 & 0xffff0000 | ZEXT24(&iVar9->field_0x16));
    pass1_1008_4480(paStack6,puVar4,paStack10);
    invalidate_rect_1020_8d90(puVar4,uVar6,param_1,uStack12,paStack6,param_3,param_4);
  }
  return;
}
pub fn invalidate_rect_1020_8d90
               (mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: u32,mut param_6: u16 ,
               mut param_7: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut local_48: RECT16;
  let mut iStack68: i16;
  let mut iStack66: i16;
  let mut local_40: i16;
  let mut local_3e: i16;
  let mut uStack60: u32;
  astruct_288 local_38;
  u8 local_10 [0xa];
  let mut uStack6: u16;
  let mut uStack4: u16;

  uVar5 = (param_3 >> 0x10);
  iVar4 = param_3;
  uVar2 = pass1_1018_266a((iVar4 + 0x22));
  if (uVar2 != 0) {
    uVar6 = pass1_1018_265c();
    uStack4 = (uVar6 >> 0x10);
    uStack6 = uVar6;
    uVar3 = CONCAT22(in_register_0000000a,uStack4 | uStack6);
    if ((uStack4 | uStack6) != 0) {
      sys_1000_3f9c(CONCAT22(0x1050,local_10),s__03ld_1050_442a,uStack6);
      uVar1 = (iVar4 + 0x22);
      file_and_draw_op_1008_4f20
                (uVar3,(astruct_76 *)CONCAT22(0x1050,&local_38),(uVar1 + 0xe),0x25,
                 CONCAT22(0x1050,local_10),param_6,param_7);
      pass1_1008_4480((astruct_76 *)param_5,(param_3 & 0xffff0000 | (iVar4 + 0x1c)),
                      (astruct_76 *)CONCAT22(0x1050,&local_38));
      uStack60 = pass1_1008_4772((astruct_76 *)CONCAT22(0x1050,&local_38));
      pass1_1008_3e94((param_3 & 0xffff0000 | (iVar4 + 0x1c)),CONCAT22(0x1050,&local_40),
                      CONCAT22(0x1050,&local_3e));
      local_48.x = local_3e;
      local_48.y = local_40;
      uVar5 = (uStack60 >> 0x10);
      iStack68 = local_3e + (uStack60 + 0x4);
      iStack66 = local_40 + (uStack60 + 0x8);
    // just 0x0
      InvalidateRect16(0x0,&local_48,(HWND16)&DAT_1050_1050);
      pass1_1008_41bc((astruct_288 *)CONCAT22(0x1050,&local_38));
    }
  }
  return;
}



StructD * pass1_1020_8e6c(StructD *param_1,param_2: u8)

{
  pass1_1020_8bae(&param_1->address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_8eaa(param_1: *mut astruct_20,mut param_2: u32)

{
  let mut puVar1: *mut u8;
  astruct_19 *paVar2;
  let mut uVar3: u16;
  let mut uVar5: u16;
  let mut paVar6: *mut Struct57;
  astruct_20 *iVar4;
  let mut unaff_SI: u16;
  let mut unaff_DI: u16;
  astruct_20 *uVar4;
  let mut puVar8: *mut u16;
  let mut puVar9: *mut u32;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffca: u16;
  u8 local_a [0x8];
  let mut uVar7: u16;

  uVar7 = (param_2 >> 0x10);
  struct_1020_847a(param_1,0x25);
  uVar4 = (astruct_20 *)(param_1 >> 0x10);
  iVar4 = (astruct_20 *)param_1;
  &iVar4->field9_0x16 = 0;
  &iVar4->field138_0xaa = 0;
  puVar1 = (&iVar4->field139_0xac + 2);
  puVar8 = pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | ZEXT24(puVar1)));
  paVar6 = (astruct_57 *)CONCAT22(uVar7,(puVar8 >> 0x10));
  &iVar4->field141_0xb4 = 0;
  iVar4->field143_0xb8 = 0xffff;
  &iVar4->field144_0xba = 0;
  param_1->offset_0x0 = 0x9204;
  iVar4->base_0x2 = 0x1020;
  puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x29),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  paVar6 = (astruct_57 *)(paVar6 & 0xffff0000 | puVar9 >> 0x10);
  paVar2 = (astruct_19 *)puVar9;
  iVar4->field9_0x16 = paVar2;
  uVar5 = (puVar9 >> 0x10);
  iVar4->field10_0x18 = uVar5;
  pass1_1018_2646(iVar4->field9_0x16,uVar5,(param_1 & 0xffff0000 | ZEXT24(puVar1)));
  uVar3 = FUN_1010_830a(paVar2,paVar6,0x1018,_u16_1050_14cc,0x1ce);
  iVar4->field141_0xb4 = uVar3;
  iVar4->field142_0xb6 = paVar6;
  pass1_1020_8712(param_1 & 0xffff | ZEXT24(uVar4) << 0x10,CONCAT22(0x1050,local_a),
                  (astruct_76 *)CONCAT22(paVar6,iVar4->field141_0xb4),
                  (param_1 & 0xffff0000 | ZEXT24(puVar1)));
  puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_DI,0x2),in_stack_0000fe9c,
                           in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
  iVar4->field144_0xba = puVar9;
  iVar4->field145_0xbc = (puVar9 >> 0x10);
  return;
}
pub fn pass1_1020_8f74(StructD *param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  StructD *iVar4;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  param_1->address_offset_field_0x0 = 0x9204;
  iVar4->address_offset_field_0x2 = 0x1020;
  puVar1 = &iVar4->field_0xb4;
  uVar2 = &iVar4->field_0xb6;
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1020_8556(param_1);
  return;
}
pub fn invalidate_rect_1020_8fb4(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u32;
  RECT16 *rect;
  let mut uVar2: u32;
  let mut extraout_DX: u16;
  let mut hwnd: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut iStack8: i16;

  uVar4 = (param_3 >> 0x10);
  iVar3 = param_3;
  uVar1 = (iVar3 + 0xba);
  if ((uVar1 + 0x1e) != 0) {
    pass1_1018_2862(*(astruct_654 **)(iVar3 + 0x16));
    (iVar3 + 0xaa) = param_1;
    (iVar3 + 0xac) = param_2;
    if ((param_2 | (iVar3 + 0xaa)) != 0) {
      uVar1 = (iVar3 + 0xaa);
      iVar3 = (uVar1 + 0xa);
      for (iStack8 = 0; iStack8 < iVar3; iStack8 += 1) {
        uVar2 = iStack8;
        empty_1008_8fc4();
        rect = (RECT16 *)uVar2;
        hwnd = extraout_DX | rect;
        if (((hwnd != 0) && (0x9 < rect[0xb].y)) &&
           (pass1_1008_8b20(uVar2 & 0xffff | extraout_DX << 0x10), (hwnd | rect) != 0)) {
          InvalidateRect16(0x0,rect,hwnd);
        }
      }
    }
  }
  return;
}
