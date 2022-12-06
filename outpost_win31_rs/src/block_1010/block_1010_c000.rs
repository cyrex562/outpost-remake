
pub fn pass1_1010_c1ba(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  astruct_92 *paVar1;
  let mut iStack28: i16;
  astruct_92 local_16;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_16),0x1,0x0,0x200);
  iStack28 = 0x1;
  while( true ) {
    paVar1 = &local_16;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
    param_1 |= paVar1;
    if ((param_1 == 0x0) || (iStack28 == param_4)) break;
    iStack28 += 0x1;
  }
  return;
}



char * pass1_1010_c234(mut param_1: u16 ,u8 *param_2)

{
  let mut pcVar1: *mut c_char;

  pass1_1010_c28a(param_2);
  if ((param_2 | param_1) == 0x0) {
    return NULL;
  }
  pcVar1 = pass1_1038_4d28(CONCAT22(param_2,param_1));
  return pcVar1;
}
pub fn pass1_1010_c25e(mut param_1: u16 ,u8 *param_2,mut param_3: u16 ,mut param_4: u16 ,char *param_5)

{
  if (param_5 != NULL) {
    pass1_1010_c28a(param_2);
    if ((param_2 | param_1) != 0x0) {
      pass1_1038_4d3c(CONCAT22(param_2,param_1),param_5,param_2 | param_1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_c28a(u8 *param_1)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffee: u16;

  puVar5 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000ffee,0x2f),in_stack_0000fe96,in_stack_0000ffba,
                           in_stack_0000ffc0,in_stack_0000ffc4);
  uVar2 = (puVar5 >> 0x10);
  uVar1 = (puVar5 + 0x24);
  uVar4 = (puVar5 + 0x26);
  uVar3 = uVar4 | uVar1;
  if (uVar3 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar4,uVar1));
    if ((uVar4 | uVar3) != 0x0) {
      return;
    }
  }
  return;
}
pub fn pass1_1010_c2d8(mut param_1: u16 ,mut param_2: u16 ,i32 param_3)

{
  let mut uVar1: u16;
  let mut uStack6: u16;

  if ((param_3 != 0x0) &&
     (uVar1 = (param_3 >> 0x10), uStack6 = (param_3 + 0x2e),
     ((param_3 + 0x30) | uStack6) != 0x0)) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_c312() -> u32

{
  return CONCAT22((_PTR_LOOP_1050_65e2 + 0x2),*_PTR_LOOP_1050_65e2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_c320(char *param_1,mut param_2: u16 ,mut param_3: u16 ,u8 *param_4,mut param_5: u32)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u32;
  let mut puStack6: *mut u8;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puStack6 = param_4;
  if (param_4 == NULL) {
    mem_op_1000_179c(0x100,paVar1);
    puStack6 = (param_4 & 0xffff | (long)paVar1 << 0x10);
  }
  uVar2 = struct_op_1030_73a8((astruct_419 *)param_5,param_4,paVar1);
  switch((uVar2 + 0x12)) {
  case 0x1:
  case 0x2:
  case 0x4:
    break;
  case 0x3:
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x7:
    break;
  case 0x8:
    break;
  case 0x9:
    break;
  default:
    *puStack6 = '\0';
    return;
  }
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,puStack6,
             (short)(puStack6 >> 0x10));
  return;
}
pub fn pass1_1010_c3c2(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  u8 local_40c [0x400];
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut pcStack6: *mut c_char;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pcStack6 = param_4;
  if (param_4 == 0x0) {
    mem_op_1000_179c(0x100,paVar3);
    pcStack6 = (param_4 & 0xffff | (long)paVar3 << 0x10);
  }
  uStack10 = struct_op_1030_73a8((astruct_419 *)param_5,param_4,paVar3);
  uVar2 = (uStack10 >> 0x10);
  uStack12 = (uStack10 + 0xc);
  uVar1 = pass1_1020_bd80(uStack12);
  unk_str_op_1000_3d3e(CONCAT22(0x1050,local_40c),CONCAT22(uVar2,uVar1));
  pass1_1030_8086(param_5);
  sys_1000_3f9c(pcStack6,0x105038c5,local_40c);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn string_op_1010_c446(mut param_1: u16 ,mut param_2: u32,char *param_3,param_4: *mut astruct_419)

{
  let mut iVar1: i16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut uVar3: u32;
  let mut pcVar4: *mut c_char;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut in_buffer_4: *mut c_char;
  short in_buf_len_5;
  let mut uStack22: u16;
  let mut pcStack6: *mut c_char;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pcStack6 = param_3;
  if (param_3 == NULL) {
    mem_op_1000_179c(0x100,paVar2);
    pcStack6 = (param_3 & 0xffff | (long)paVar2 << 0x10);
  }
  uVar3 = struct_op_1030_73a8(param_4,param_3,paVar2);
  struct_1010_dd5e(param_2,(param_2 >> 0x10),param_4);
  iVar1 = (uVar3 + 0x12);
  if (0x6 < iVar1 - 0x3) {
    return;
  }
  in_buffer_4 = pcStack6;
  in_buf_len_5 = (short)(pcStack6 >> 0x10);
  uVar6 = _u16_1050_14cc;
  uVar7 = (_u16_1050_14cc >> 0x10);
  switch(iVar1) {
  default:
    break;
  case 0x6:
    load_string_1010_84e0(uVar6,uVar7,0x3ff,in_buffer_4,in_buf_len_5);
    uStack22 = str_op_1000_3da4(pcStack6);
    pcVar4 = load_string_1010_847e(_u16_1050_14cc,0x61e);
    uVar8 = pcVar4;
    uVar5 = SUB42(s_____s__lu_1050_38d7,0x0);
// TODO: goto LAB_1010_c4f9;
  case 0x7:
  case 0x9:
    break;
  case 0x8:
    load_string_1010_84e0(uVar6,uVar7,0x3ff,in_buffer_4,in_buf_len_5);
    uStack22 = str_op_1000_3da4(pcStack6);
    pcVar4 = load_string_1010_847e(_u16_1050_14cc,0x61e);
    uVar8 = pcVar4;
    uVar5 = SUB42(s_____s__lu_1050_38cd,0x0);//
LAB_1010_c4f9:
    sys_1000_3f9c((pcStack6 & 0xffff0000 | ZEXT24(in_buffer_4 + uStack22)),CONCAT22(0x1050,uVar5)
                  ,uVar8);
    return;
  }
  load_string_1010_84e0(uVar6,uVar7,0x3ff,in_buffer_4,in_buf_len_5);
  return;
}
pub fn pass1_1010_c58as(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_20,mut param_4: u16 ,mut param_5: u32)

{
  StructD *pstructd_1_lo;
  let mut pstructd_lo_11: u16;
  let mut uVar1: u32;
  StructD *pstructd_1_hi;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut paVar6: *mut Struct57;
  StructD *pUStack26;
  HDC16 HStack18;
  let mut UStack16: u16;
  u8 **ppuStack14;
  let mut pstructd32_1: *mut u16;
  let mut paVar5: *mut Struct57;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar1 = param_1;
  mem_op_1000_179c(0x18,paVar4);
  pstructd_1_lo = uVar1;
  pstructd_1_hi = (paVar4 | pstructd_1_lo);
  paVar6 = (astruct_57 *)(paVar4 & 0xffff0000);
  paVar5 = (astruct_57 *)(paVar6 | ZEXT24(pstructd_1_hi));
  if (pstructd_1_hi == NULL) {
    pstructd_1_lo = NULL;
  }
  else {
    struct_1040_a598((astruct_259 *)(uVar1 & 0xffff | (long)paVar4 << 0x10));
    paVar6 = paVar5;
  }
  uVar2 = paVar6;
  pstructd32_1 = CONCAT22(uVar2,pstructd_1_lo);
  paVar4 = (astruct_57 *)(paVar6 & 0xffff0000 | (uVar2 | pstructd_1_lo));
  if ((uVar2 | pstructd_1_lo) == 0x0) {
    return;
  }
  ppuStack14 = NULL;
  HStack18 = 0x0;
  UStack16 = 0x0;
  switch(param_4) {
  case 0x5:
    ppuStack14 = (u8 **)&u16_1050_352c;
    HStack18 = 0xfa4;
    UStack16 = 0x30;
    break;
  default:
    if (pstructd32_1 == NULL) {
      return;
    }
    pass1_1040_a5d0(CONCAT22(uVar2,pstructd_1_lo));
    fn_ptr_1000_17ce(pstructd32_1);
    return;
  case 0xa:
    ppuStack14 = (u8 **)&u16_1050_358c;
    HStack18 = 0xfb3;
    UStack16 = 0x51;
    break;
  case 0xb:
    ppuStack14 = (u8 **)&u16_1050_358c;
    HStack18 = 0xfb4;
    UStack16 = 0x51;
    break;
  case 0xc:
    ppuStack14 = (u8 **)&u16_1050_362e;
    HStack18 = 0xfb6;
    UStack16 = 0x30;
    break;
  case 0x10:
    ppuStack14 = &PTR_DAT_1050_1805_1050_368e;
    HStack18 = 0xfc4;
    UStack16 = 0x3c;
    break;
  case 0x11:
    ppuStack14 = &PTR_DAT_1050_1805_1050_3706;
    HStack18 = 0xfc1;
    UStack16 = 0x4b;
    break;
  case 0x12:
    ppuStack14 = (u8 **)&u16_1050_379c;
    HStack18 = 0xfc5;
    UStack16 = 0x8;
    break;
  case 0x13:
    pass1_1010_debe(param_3,param_4,CONCAT22(uVar2,&pstructd_1_lo->field_0x10),
                    CONCAT22(uVar2,&pstructd_1_lo->field7_0xc),param_5);
    ppuStack14 = (u8 **)&u16_1050_37ac;
    HStack18 = 0xfc6;
    UStack16 = 0x1;
    paVar4 = paVar6;
    break;
  case 0x15:
    &pstructd_1_lo->field_0x6 = param_5;
    pstructd_1_lo->field6_0xa = param_4;
    break;
  case 0x16:
    ppuStack14 = (u8 **)&u16_1050_37ae;
    HStack18 = 0x157;
    UStack16 = 0x4;
    break;
  case 0x17:
    ppuStack14 = (u8 **)&u16_1050_37b6;
    UStack16 = 0x2c;
    HStack18 = 0xfd8;
  }
  if (ppuStack14 != NULL) {
    *pstructd32_1 = UStack16;
    pstructd_lo_11 = UStack16 * 0xa + 0x2;
    mem_op_1000_179c(pstructd_lo_11,paVar4);
    uVar3 = paVar4;
    pUStack26 = CONCAT22(uVar3,pstructd_lo_11);
    if ((uVar3 | pstructd_lo_11) == 0x0) {
      &pstructd_1_lo->address_offset_field_0x2 = 0x0;
    }
    else {
      pUStack26->address_offset_field_0x0 = UStack16;
      pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,UStack16,0xa,pstructd_lo_11 + 0x2,uVar3);
      pstructd_1_lo->address_offset_field_0x2 = pstructd_lo_11 + 0x2;
      pstructd_1_lo->hfile_0x4 = uVar3;
    }
    &pstructd_1_lo->field_0x6 = param_5;
    pstructd_1_lo->field6_0xa = param_4;
    pstructd_1_lo->field11_0x12 = HStack18;
    pass1_1010_a50c(param_3,ppuStack14,CONCAT22(uVar2,pstructd_1_lo));
  }
  return;
}
pub fn pass1_1010_c7e2(mut param_1: u32,mut param_2: u32,param_3: *mut i16)

{
  let mut uVar1: u32;
  let mut pcVar2: *mut c_char;
  let mut in_DX: u16;
  let mut iVar3: i16;
  let mut unaff_SI: i16;
  let mut uVar4: u16;
  let mut puStack8: *mut u16;
  let mut iStack4: i16;

  iStack4 = 0x0;
  while( true ) {
    uVar4 = (param_3 >> 0x10);
    iVar3 = param_3;
    if (*param_3 == iStack4 || *param_3 < iStack4) break;
    uVar1 = (iVar3 + 0x2);
    (iStack4 * 0xa + uVar1 + 0x4) = (iStack4 * 0x2 + param_2);
    iStack4 += 0x1;
  }
  puStack8 = (u16*)(iVar3 + 0x2);
  for (iStack4 = 0x0; *param_3 != iStack4 && iStack4 <= *param_3; iStack4 += 0x1) {
    uVar1 = (iVar3 + 0x6);
    pcVar2 = pass1_1010_b038(param_1,uVar1,(uVar1 >> 0x10),
                             (puStack8 + 0x4),unaff_SI);
    string_1040_a626(in_DX,puStack8,CONCAT22(in_DX,pcVar2));
    puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_c864(char *param_1,mut param_2: u32,param_3: *mut u16,param_4: *mut astruct_104)

{
  StructD **ppSVar1;
  let mut lVar2: i32;
  code **ppcVar3;
  let mut uVar4: u32;
  let mut uVar12: *mut c_char;
  let mut pcVar5: *mut c_char;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut uVar8: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar9: u16;
  astruct_104 *struct_1;
  let mut iVar10: i16;
  astruct_104 *struct_1_seg;
  let mut uVar11: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut uVar15: u16;
  let mut local_1f0: u32;
  let mut pcStack412: *mut c_char;
  let mut uStack408: u32;
  let mut uStack404: *mut c_char;
  let mut uStack402: u16;
  let mut uStack400: u16;
  let mut uStack398: u32;
  StructD *local_18a;
  let mut local_f6: u32;
  let mut puStack98: *mut u16;
  let mut iStack94: i16;
  let mut uStack92: u32;
  astruct_104 *struct_2;
  let mut puStack86: *mut u8;
  char string_1 [0x52];

  puStack86 = NULL;
  struct_1_seg = (astruct_104 *)(param_4 >> 0x10);
  struct_1 = (astruct_104 *)param_4;
  struct_2 = (astruct_104 *)param_4->field0_0x0;
  uVar12 = NULL;
  uStack92 = 0x0;
  uVar14 = param_2;
  uVar15 = (param_2 >> 0x10);
  pass1_1010_c320(param_1,uVar14,uVar15,NULL,(struct_1 + 0x1));
  unk_str_op_1000_3d3e(CONCAT22(0x1050,string_1),CONCAT22(param_1,uVar12));
  puStack98 = struct_1->field1_0x2;
  uStack402 = (&struct_1->field1_0x2 + 0x2);
  (puStack98 + 0x4) = *param_3;
  string_1040_a626(uStack402,puStack98,CONCAT22(0x1050,string_1));
  iStack94 = 0x0;
  pass1_1000_4906(CONCAT22(0x1050,&local_f6),NULL,0x94);
  uStack404 = pass1_1000_4906(CONCAT22(0x1050,&local_18a),NULL,0x94);
  uStack398 = 0x0;
  for (uStack400 = 0x1; uStack400 < 0x25; uStack400 += 0x1) {
    _uStack404 = pass1_1030_7c28(uStack404,uStack402,(struct_1 + 0x1),uStack400);
    uStack402 = (_uStack404 >> 0x10) | uStack404;
    if (_uStack404 != NULL) {
      pcVar5 = string_1020_c0d8(uStack400);
      uStack408 = CONCAT22(uStack402,pcVar5);
      uVar8 = uStack402 | pcVar5;
      if (uVar8 == 0x0) {
        unk_str_op_1000_3d3e((&local_f6)[uStack398],s_Null_Ptr_1050_38e1);
      }
      else {
        uVar6 = str_op_1008_60e8(uVar8,CONCAT22(uStack402,pcVar5));
        (&local_f6 + uStack398) = uVar6;
        (&local_f6 + uStack398 * 0x4 + 0x2) = uVar8;
      }
      *(&local_18a + uStack398) = uStack404;
      (&local_18a + uStack398 * 0x4 + 0x2) = uStack402;
      uStack398 += 0x1;
    }
  }
  uStack92 = uStack398;
  if (0x13 < uStack398) {
    iStack94 = 0x1;
  }
  puStack86 =
              pass1_1010_db2e(uVar14,uVar15,0x1,CONCAT22(0x1050,&local_f6),CONCAT22(0x1050,&local_18a),param_3,
                              param_4);
  while (uVar7 = uStack398 - 0x1, uStack398 != 0x0) {
    uStack398 = uVar7;
    pcStack412 = (&local_f6)[uStack398];
    _uStack404 = pcStack412;
    uStack398 = uVar7;
    fn_ptr_1000_17ce(pcStack412);
  }
  uVar13 = 0x1000;
  uStack398 = uVar7;
  pass1_1000_4906(CONCAT22(0x1050,&local_18a),NULL,0x54);
  uVar4 = (struct_1 + 0x1);
  uVar11 = (uVar4 >> 0x10);
  iVar10 = uVar4;
  _uStack404 = (iVar10 + 0x1e);
  uVar8 = (iVar10 + 0x20) | uStack404;
  uVar7 = uVar8;
  if (uVar8 != 0x0) {
    uStack398 = 0x0;
    while( true ) {
      uVar4 = *_uStack404;
      ppcVar3 = (code **)(uVar4 + 0x10);
      (**ppcVar3)(uVar13,_uStack404,(_uStack404 >> 0x10));
      if ((extraout_DX < uStack398) || ((extraout_DX <= uStack398 && (uVar7 <= uStack398))))
      break;
      ppcVar3 = (code **)(uVar4 + 0x4);
      (**ppcVar3)(uVar13,_uStack404,uStack398,uStack398);
      uVar8 = uVar7;
      uVar9 = extraout_DX_00 | uVar8;
      if (uVar9 != 0x0) {
        uVar13 = 0x1028;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7 & 0xffff | extraout_DX_00 << 0x10);
        uStack408 = CONCAT22(uVar9,uVar8);
        if ((uVar9 | uVar8) == 0x0) {
          return;
        }
        iVar10 = (uVar8 + 0xc);
        uVar7 = (iVar10 - 0x1);
        if (((0x0 < iVar10) && (!SBORROW2(iVar10,0x1))) &&
           (uVar7 = (iVar10 - 0x6U), iVar10 - 0x6U == 0x0 || (iVar10 - 0x1) < 0x5)) {
          ppSVar1 = &local_18a + iVar10;
          *ppSVar1 = ((long)&(*ppSVar1)->address_offset_field_0x0 + 0x1);
        }
      }
      uStack398 += 0x1;
    }
    uVar8 = extraout_DX;
    pass1_1000_4906(CONCAT22(0x1050,&local_f6),NULL,0x54);
    pass1_1000_4906(CONCAT22(0x1050,&local_1f0),NULL,0x54);
    uStack398 = 0x0;
    for (uStack400 = 0x1; uStack400 < 0x15; uStack400 += 0x1) {
      if ((&local_18a)[uStack400] != NULL) {
        pcVar5 = string_op_1020_c222(uStack400);
        uVar9 = uVar8 | pcVar5;
        if (uVar9 == 0x0) {
          unk_str_op_1000_3d3e((&local_f6)[uStack398],s_Null_Ptr_1050_38ea);
        }
        else {
          uVar6 = str_op_1008_60e8(uVar9,CONCAT22(uVar8,pcVar5));
          (&local_f6 + uStack398) = uVar6;
          (&local_f6 + uStack398 * 0x4 + 0x2) = uVar9;
        }
        uVar8 = (&local_18a + uStack400 * 0x4 + 0x2);
        (&local_1f0 + uStack398) = (&local_18a + uStack400);
        (&local_1f0 + uStack398 * 0x4 + 0x2) = uVar8;
        uStack398 += 0x1;
      }
    }
    if (iStack94 == 0x0) {
      iVar10 = (uStack92 >> 0x10) + CARRY2(uStack92,uStack398);
      uStack92 = CONCAT22(iVar10,uStack92 + uStack398);
      if ((iVar10 != 0x0) || (0x13 < uStack92 + uStack398)) {
        iStack94 = 0x1;
      }
    }
    if ((puStack86 < (&struct_2[-0x1].field1_0x2 + 0x2U)) && (local_1f0 != 0x0)) {
      iVar10 = string_1010_dcac(uVar14,uVar15,puStack86,param_3,param_4);
      puStack86 = (iVar10 + 0x1);
      puStack86 =
                  pass1_1010_db2e(uVar14,uVar15,puStack86,CONCAT22(0x1050,&local_f6),CONCAT22(0x1050,&local_1f0),
                                  param_3,param_4);
    }
    while (lVar2 = uStack398 + -0x1, uStack398 != 0x0) {
      uStack398 = lVar2;
      pcStack412 = (&local_f6)[uStack398];
      uStack398 = lVar2;
      fn_ptr_1000_17ce(pcStack412);
    }
    if (iStack94 != 0x0) {
      (&struct_1[0x3].field1_0x2 + 0x2) = 0x1;
    }
    uStack398 = lVar2;
    pass1_1010_dc36(uVar14,uVar15,puStack86,param_3,param_4);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_cc56(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,param_4: *mut astruct_104)

{
  StructD **ppSVar1;
  let mut uVar2: u32;
  let mut pcVar3: *mut c_char;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  StructD *local_1a0;
  let mut uStack332: u32;
  let mut uStack328: u16;
  let mut uStack326: u16;
  let mut uStack324: u32;
  let mut uStack320: u16;
  StructD *local_13e;
  StructD *local_aa;
  let mut uStack22: u16;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar9 = (param_2 >> 0x10);
  uVar8 = param_2;
  uVar2 = (uVar8 + 0x138);
  uVar6 = (uVar2 + 0x24);
  uStack6 = uVar6;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6);
  uStack10 = uVar6 & 0xffff | param_1 << 0x10;
  uStack324 = param_1 | uVar6;
  if (uStack324 != 0x0) {
    uStack14 = param_4->field0_0x0;
    iStack18 = 0x0;
    pass1_1000_4906(CONCAT22(0x1050,&local_aa),NULL,0x94);
    pass1_1000_4906(CONCAT22(0x1050,&local_13e),NULL,0x94);
    uStack12 = 0x0;
    uStack16 = 0x0;
    uStack22 = 0x0;
    for (uStack320 = 0x1; uStack320 < 0x25; uStack320 += 0x1) {
      uStack324 = *(i32 *)(uStack10 + 0x26 + uStack320 * 0x4);
      if (uStack324 != 0x0) {
        pcVar3 = string_1020_c0d8(uStack320);
        uStack332 = uStack332 & 0xffff | ZEXT24(pcVar3) << 0x10;
        uVar7 = uStack324 | pcVar3;
        uStack328 = uStack324;
        if (uVar7 == 0x0) {
          unk_str_op_1000_3d3e((&local_aa)[uStack22],s_Null_Ptr_1050_38f3);
        }
        else {
          uVar4 = str_op_1008_60e8(uVar7,CONCAT22(uStack324,pcVar3));
          (&local_aa + uStack22) = uVar4;
          (&local_aa + uStack22 * 0x4 + 0x2) = uVar7;
        }
        (&local_13e + uStack22) = uStack324;
        (&local_13e + uStack22 * 0x4 + 0x2) = uStack324;
        uStack22 += 0x1;
      }
    }
    uStack16 = uStack22;
    if (0x13 < uStack22) {
      iStack18 = 0x1;
    }
    uStack12 = pass1_1010_db2e(uVar8,uVar9,0x1,CONCAT22(0x1050,&local_aa),CONCAT22(0x1050,&local_13e),param_3,param_4);
    pass1_1000_4906(CONCAT22(0x1050,&local_13e),NULL,0x54);
    for (uStack332 = 0x1; uStack332 < 0x15; uStack332 += 0x1) {
      uStack326 = uStack332;
      if (*(i32 *)(uStack10 + 0x14e + uStack332 * 0x4) != 0x0) {
        if (((0x0 < uStack332) && (!SBORROW2(uStack332,0x1))) && ((uStack332 - 0x1) < 0x6))
        {
          ppSVar1 = &local_13e + uStack332;
          *ppSVar1 = ((long)&(*ppSVar1)->address_offset_field_0x0 + 0x1);
        }
      }
    }
    pass1_1000_4906(CONCAT22(0x1050,&local_aa),NULL,0x54);
    pass1_1000_4906(CONCAT22(0x1050,&local_1a0),NULL,0x54);
    for (uStack332 = 0x10000; uStack332 < 0x15;
        uStack332 = uStack332 & 0xffff | (uStack332 + 0x1) << 0x10) {
      if ((&local_13e)[uStack332] != NULL) {
        pcVar3 = string_op_1020_c222(uStack332);
        uStack324 = CONCAT22(uStack324,pcVar3);
        uVar7 = uStack324 | pcVar3;
        if (uVar7 == 0x0) {
          unk_str_op_1000_3d3e((&local_aa)[uStack332],s_Null_Ptr_1050_38fc);
        }
        else {
          uVar4 = str_op_1008_60e8(uVar7,CONCAT22(uStack324,pcVar3));
          (&local_aa + uStack332) = uVar4;
          (&local_aa + uStack332 * 0x4 + 0x2) = uVar7;
        }
        uStack324 = (&local_13e + uStack332 * 0x4 + 0x2);
        (&local_1a0 + uStack332) = (&local_13e + uStack332);
        (&local_1a0 + uStack332 * 0x4 + 0x2) = uStack324;
        uStack332 = uStack332 & 0xffff0000 | (uStack332 + 0x1);
      }
    }
    if (iStack18 == 0x0) {
      uStack16 += uStack332;
      if (0x13 < uStack16) {
        iStack18 = 0x1;
      }
    }
    if ((uStack12 < uStack14 - 0x2) && (local_1a0 != NULL)) {
      iVar5 = string_1010_dcac(uVar8,uVar9,uStack12,param_3,param_4);
      uStack12 = iVar5 + 0x1;
      uStack12 = pass1_1010_db2e(uVar8,uVar9,uStack12,CONCAT22(0x1050,&local_aa),CONCAT22(0x1050,&local_1a0),param_3,
                                 param_4);
    }
    if (iStack18 != 0x0) {
      (param_4 + 0x16) = 0x1;
    }
    pass1_1010_dc36(uVar8,uVar9,uStack12,param_3,param_4);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_cf36(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,param_4: *mut astruct_104)

{
  let mut uVar1: u32;
  let mut pcVar2: *mut c_char;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut unaff_SI: i16;
  let mut iVar7: i16;
  astruct_104 *struct_104_1;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u32;
  let mut uVar12: u32;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut uVar15: u16;
  let mut uStack326: u16;
  let mut iStack316: i16;
  let mut uStack314: u16;
  let mut iStack312: i16;
  u16 local_136 [0x4a];
  let mut local_a2: u32;
  let mut iStack14: i16;
  let mut uStack12: u32;
  let mut puStack8: *mut u16;
  let mut iStack4: i16;

  iStack4 = 0x0;
  loop {
    uVar8 = (param_3 >> 0x10);
    iVar7 = param_3;
    uVar9 = (param_4 >> 0x10);
    struct_104_1 = (astruct_104 *)param_4;
    uVar1 = struct_104_1->field1_0x2;
    (iStack4 * 0xa + uVar1 + 0x4) = (iStack4 * 0x2 + iVar7);
    iStack4 += 0x1;
  } while (iStack4 < 0x8);
  puStack8 = struct_104_1->field1_0x2;
  iStack4 = 0x0;
  loop {
    uVar11 = (struct_104_1 + 0x1);
    pcVar2 = pass1_1010_b038(param_2,uVar11,(uVar11 >> 0x10),
                             (puStack8 + 0x4),unaff_SI);
    string_1040_a626(param_1,puStack8,CONCAT22(param_1,pcVar2));
    iStack4 += 0x1;
    puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
  } while (iStack4 < 0x8);
  uVar13 = param_2;
  uVar14 = (param_2 >> 0x10);
  struct_1010_dd5e(uVar13,uVar14,(struct_104_1 + 0x1));
  uStack12 = CONCAT22(param_1,pcVar2);
  uVar4 = param_1 | pcVar2;
  if (uVar4 != 0x0) {
    iStack14 = 0x0;
    pass1_1000_4906(CONCAT22(0x1050,&local_a2),NULL,0x94);
    pass1_1000_4906(CONCAT22(0x1050,local_136),NULL,0x94);
    uStack314 = 0x0;
    iStack312 = 0x0;
    iStack316 = 0x0;
    uVar11 = (struct_104_1 + 0x1);
    uVar1 = (uVar11 + 0x26);
    uVar12 = uVar1;
    for (uStack326 = 0x1; uStack326 < 0x25; uStack326 += 0x1) {
      uVar15 = (uVar1 >> 0x10);
      if (*(i32 *)(uStack326 * 0x4 + uStack12) == 0x0) {
        if (uVar1 != 0x0) {
          uVar12 = pass1_1020_bae6(uVar12,uVar4,uVar1,CONCAT22(uStack326,uVar15));
          uVar6 = (uVar12 >> 0x10);
          uVar12 &= 0xffff;
          uVar4 = uVar6 | uVar12;
          if (uVar4 != 0x0) {
            if (iStack14 == 0x0) {
              iStack14 = 0x1;
            }
            pcVar2 = string_1020_c0d8(uStack326);
            uVar5 = uVar4 | pcVar2;
            if (uVar5 == 0x0) {
              unk_str_op_1000_3d3e((&local_a2)[iStack312],s_Null_Ptr_1050_390e);
            }
            else {
              uVar3 = str_op_1008_60e8(uVar5,CONCAT22(uVar4,pcVar2));
              (&local_a2 + iStack312) = uVar3;
              (&local_a2 + iStack312 * 0x4 + 0x2) = uVar5;
            }
            local_136[iStack312 * 0x2] = uVar12;
            local_136[iStack312 * 0x2 + 0x1] = uVar6;
        // TODO: goto LAB_1010_d11d;
          }
        }
      }
      else {
        if (iStack14 == 0x0) {
          iStack14 = 0x1;
        }
        pcVar2 = string_1020_c0d8(uStack326);
        uVar6 = uVar4 | pcVar2;
        if (uVar6 == 0x0) {
          unk_str_op_1000_3d3e((&local_a2)[iStack312],s_Null_Ptr_1050_3905);
        }
        else {
          uVar3 = str_op_1008_60e8(uVar6,CONCAT22(uVar4,pcVar2));
          (&local_a2 + iStack312) = uVar3;
          (&local_a2 + iStack312 * 0x4 + 0x2) = uVar6;
        }
        uVar10 = (uStack12 >> 0x10);
        uVar4 = (uStack326 * 0x4 + uStack12);
        uVar6 = (uStack326 * 0x4 + uStack12 + 0x2);
        local_136[iStack312 * 0x2] = uVar4;
        local_136[iStack312 * 0x2 + 0x1] = uVar6;
        if (uVar1 == 0x0) {
          uVar4 = 0x0;
        }
        else {
          uVar11 = pass1_1020_bae6(uVar4,uVar6,uVar1,CONCAT22(uStack326,uVar15));
          uVar6 = (uVar11 >> 0x10);
          uVar4 = uVar11;
        }
        uVar12 = uVar4;
        if (uVar4 == 0x0) {
          iStack316 += 0x2;
          uVar4 = uVar6;
          iStack312 = iStack312 + 0x1;
        }
        else {//
LAB_1010_d11d:
          iStack312 += 0x1;
          (uVar13 + uStack314 * 0x2 + 0xa4) = (iVar7 + iStack316 * 0x2 + 0x10);
          (uVar13 + (uStack314 + 0x1) * 0x2 + 0xa4) =
               (iVar7 + (iStack316 + 0x1) * 0x2 + 0x10);
          iStack316 += 0x2;
          uStack314 += 0x2;
          uVar12 = uStack314;
          uVar4 = uVar6;
        }
      }
    }
    uVar4 = pass1_1010_db2e(uVar13,uVar14,0x8,CONCAT22(0x1050,&local_a2),CONCAT22(0x1050,local_136),param_3,param_4);
    if (iStack14 != 0x0) {
      (&struct_104_1[0x3].field1_0x2 + 0x2) = 0x1;
    }
    while (iStack312 != 0x0) {
      fn_ptr_1000_17ce((&local_a2)[iStack312 + -0x1]);
      iStack312 = iStack312 + -0x1;
    }
    pass1_1010_dc36(uVar13,uVar14,uVar4,param_3,param_4);
  }
  return;
}
