
u16 * pass1_1008_b05a(StructD *param_1)

{
  StructD *iVar1;
  StructD *uVar1;

  uVar1 = (StructD *)(param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  &iVar1->hfile_0x4 = 0x0;
  param_1->address_offset_field_0x0 = 0xbdc8;
  iVar1->address_offset_field_0x2 = 0x1008;
  return &param_1->address_offset_field_0x0;
}
pub fn pass1_1008_b08c(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0xbdc8;
  (iVar1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(*(char **)(iVar1 + 0x4));
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  return;
}
pub fn set_stuct_1008_b0bc(StructD *param_1)

{
  StructD *iVar1;
  StructD *uVar1;

  pass1_1008_b05a(param_1);
  uVar1 = (StructD *)(param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  iVar1->field5_0x8 = 0x0;
  &iVar1->field6_0xa = 0x0;
  iVar1->field8_0xe = 0x0;
  &iVar1->field_0x10 = 0x0;
  param_1->address_offset_field_0x0 = 0xbdc4;
  iVar1->address_offset_field_0x2 = 0x1008;
  return;
}



u16 * pass1_1008_b0f2(StructD *param_1)

{
  StructD *uVar1;

  pass1_1008_b05a(param_1);
  uVar1 = (StructD *)(param_1 >> 0x10);
  (param_1 + 0x8) = 0x0;
  param_1->address_offset_field_0x0 = 0xbdc0;
  (param_1 + 0x2) = 0x1008;
  return &param_1->address_offset_field_0x0;
}



u16 * pass1_1008_b11e(StructD *param_1)

{
  StructD *uVar1;

  pass1_1008_b05a(param_1);
  uVar1 = (StructD *)(param_1 >> 0x10);
  (param_1 + 0x8) = 0x0;
  param_1->address_offset_field_0x0 = 0xbddc;
  (param_1 + 0x2) = 0x1008;
  return &param_1->address_offset_field_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_b146(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_3 >> 0x10);
  iVar2 = param_3;
  if (*(i32 *)(iVar2 + 0x16) != 0x0) {
    uVar1 = (iVar2 + 0x16);
    pass1_1030_8344(_u16_1050_5748,(uVar1 + 0xa));
    pass1_1038_3608(CONCAT22(param_2,param_1));
    uVar1 = (iVar2 + 0x16);
    (uVar1 + 0x8) = 0x0;
    uVar1 = (iVar2 + 0x16);
    (uVar1 + 0xa) = 0x0;
    uVar1 = (iVar2 + 0x16);
    (uVar1 + 0xe) = 0x0;
    uVar1 = (iVar2 + 0x16);
    (uVar1 + 0x10) = 0x0;
  }
  return;
}
pub fn pass1_1008_b1a6(mut param_1: u32,char *param_2)

{
  i32 lVar1;
  let mut uVar2: u16;
  let mut in_DX: u16;
  astruct_467 *iVar3;
  astruct_466 *iVar4;
  let mut uVar3: u16;
  let mut uVar4: u16;

  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_467 *)param_1;
  if (iVar3->field22_0x16 != 0x0) {
    lVar1 = iVar3->field22_0x16;
    fn_ptr_1000_17ce(*(char **)(lVar1 + 0x4));
    uVar2 = str_op_1008_60e8(in_DX,param_2);
    lVar1 = iVar3->field22_0x16;
    uVar4 = (lVar1 >> 0x10);
    iVar4 = (astruct_466 *)lVar1;
    iVar4->field4_0x4 = uVar2;
    iVar4->field5_0x6 = in_DX;
    iVar3->field22_0x16 = 0x0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * load_string_1008_b1f0(void)

{
  let mut pcVar1: *mut c_char;

  pcVar1 = load_string_1010_847e(_u16_1050_14cc,0x531);
  return pcVar1;
}
pub fn pass1_1008_b200(param_1: *mut astruct_194)

{
  let mut uVar1: u32;
  let mut puVar2: *mut u32;
  code **ppcVar3;
  let mut puVar4: *mut u32;
  astruct_92 *paVar5;
  astruct_195 *uVar5;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut in_EDX: u32;
  let mut paVar11: *mut Struct57;
  let mut paVar13: *mut Struct57;
  let mut uVar14: u32;
  astruct_194 *iVar12;
  let mut uVar15: u16;
  let mut puVar16: *mut u16;
  let mut pcVar17: *mut c_char;
  let mut pcStack24: *mut c_char;
  astruct_92 local_14;
  let mut paVar12: *mut Struct57;

  uVar15 = (param_1 >> 0x10);
  iVar12 = (astruct_194 *)param_1;
  if (iVar12->field14_0xe != NULL) {
    return;
  }
    // WARNING: Load size is inaccurate
  puVar4 = iVar12->field14_0xe;
  uVar9 = (&iVar12->field14_0xe + 0x2);
  paVar11 = (astruct_57 *)(in_EDX & 0xffff0000 | uVar9);
  if ((uVar9 | puVar4) != 0x0) {
    ppcVar3 = (code **)*puVar4;
    (**ppcVar3)();
  }
  mem_op_1000_179c(0xc,paVar11);
  uVar9 = paVar11 | puVar4;
  paVar13 = (astruct_57 *)(paVar11 & 0xffff0000);
  paVar12 = (astruct_57 *)(paVar13 | uVar9);
  if (uVar9 == 0x0) {
    puVar4 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar11,puVar4));
    paVar13 = paVar12;
  }
  (u32*)&iVar12->field14_0xe = puVar4;
  (&iVar12->field14_0xe + 0x2) = paVar13;
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
  while( true ) {
    paVar5 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar5));
    uVar9 = paVar13;
    pcStack24 = CONCAT22(uVar9,paVar5);
    paVar13 = (astruct_57 *)(paVar13 & 0xffff0000 | (uVar9 | paVar5));
    if ((uVar9 | paVar5) == 0x0) break;
    uVar1 = paVar5->field3_0x4;
    if (paVar5[0x1c].field4_0x8 == 0x8000001) {
      uVar8 = uVar1;
      mem_op_1000_179c(0xc,paVar13);
      uVar5 = (astruct_195 *)uVar8;
      uVar14 = paVar13 & 0xffff0000;
      if ((paVar13 | uVar5) == 0x0) {
        iVar6 = 0x0;
      }
      else {
        puVar16 = pass1_1008_b0f2((StructD *)(uVar8 & 0xffff | (long)paVar13 << 0x10));
        uVar14 = uVar14 & 0xffff0000 | puVar16 >> 0x10;
        iVar6 = puVar16;
      }
      uVar10 = uVar14;
      pcVar17 = pass1_1038_4d28(pcStack24);
      paVar13 = (astruct_57 *)(uVar14 & 0xffff0000 | pcVar17 >> 0x10);
      uVar7 = (pcVar17 >> 0x10);
      uVar7 = str_op_1008_60e8(uVar7,(pcVar17 & 0xffff | uVar7 << 0x10));
      (iVar6 + 0x4) = uVar7;
      (iVar6 + 0x6) = paVar13;
      (iVar6 + 0x8) = uVar1;
      puVar2 = iVar12->field14_0xe;
      ppcVar3 = (code **)(*iVar12->field14_0xe + 0x8);
      (**ppcVar3)(0x38,puVar2,(puVar2 >> 0x10));
    }
  }
  return;
}



pub fn pass1_1008_b340(mut param_1: u32) -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0x16) != 0x0) {
    uVar1 = (param_1 + 0x16);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
  }
  return 0x0;
}



pub fn pass1_1008_b366(mut param_1: u32) -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0x1a) != 0x0) {
    uVar1 = (param_1 + 0x1a);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_b38c(StructD *param_1,param_2: *mut astruct_196) -> u32

{
  let mut puVar1: *mut u32;
  code **ppcVar2;
  let mut uVar3: u16;
  astruct_197 *iVar3;
  astruct_197 *paVar4;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut paVar7: *mut Struct57;
  let mut uVar9: u32;
  astruct_196 *iVar4;
  astruct_196 *uVar4;
  let mut puVar10: *mut u16;
  let mut uStack4: u16;
  let mut paVar8: *mut Struct57;

  uVar3 = (param_1 >> 0x10);
  paVar7 = (astruct_57 *)(param_1 & 0xffff0000 | param_1 & 0xffff);
  uVar4 = (astruct_196 *)(param_2 >> 0x10);
  iVar4 = (astruct_196 *)param_2;
  if (iVar4->field18_0x12 == NULL) {
    mem_op_1000_179c(0xc,paVar7);
    uVar5 = paVar7 | uVar3;
    paVar8 = (astruct_57 *)(paVar7 & 0xffff0000 | uVar5);
    if (uVar5 == 0x0) {
      iVar4->field18_0x12 = NULL;
    }
    else {
      uVar3 = set_struct_1008_574a((astruct_57 *)CONCAT22(paVar7,uVar3));
      &iVar4->field18_0x12 = uVar3;
      (&iVar4->field18_0x12 + 0x2) = paVar8;
    }
    for (uStack4 = 0x6d9; uStack4 < 0x6e7; uStack4 += 0x1) {
      if (uStack4 == 0x6e3) {
        pass1_1030_8344(_u16_1050_5748,0x8000001);
        if ((uVar3 + 0x136) != 0x0) goto LAB_1008_b44a;
      }
      else {//
LAB_1008_b44a:
        mem_op_1000_179c(0xa,paVar8);
        uVar9 = paVar8 & 0xffff0000;
        if ((paVar8 | uVar3) == 0x0) {
          iVar3 = NULL;
          paVar8 = (astruct_57 *)(paVar8 & 0xffff0000);
        }
        else {
          puVar10 = pass1_1008_b11e((StructD *)CONCAT22(paVar8,uVar3));
          paVar8 = (astruct_57 *)(uVar9 & 0xffff0000 | puVar10 >> 0x10);
          iVar3 = (astruct_197 *)puVar10;
        }
        uVar6 = SUB42(paVar8,0x0);
        paVar4 = iVar3;
        load_string_1010_84ac(_u16_1050_14cc,(INT16)(_u16_1050_14cc >> 0x10),uStack4);
        iVar3->field4_0x4 = paVar4;
        iVar3->field5_0x6 = paVar8;
        iVar3->field6_0x8 = uStack4 - 0x6d8;
        puVar1 = iVar4->field18_0x12;
        ppcVar2 = (code **)(*iVar4->field18_0x12 + 0x8);
        uVar3 = (**ppcVar2)(0x1010,puVar1,(puVar1 >> 0x10),iVar3,uVar6);
      }
    }
  }
  return CONCAT22((&iVar4->field18_0x12 + 0x2),&iVar4->field18_0x12);
}



pub fn pass1_1008_b47a(mut param_1: u32) -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0x1e) != 0x0) {
    uVar1 = (param_1 + 0x1e);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
  }
  return 0x0;
}
pub fn pass1_1008_b4a0(mut param_1: u16 ,u8 *param_2,mut param_3: u32,i32 param_4)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  i32 lVar7;

  uVar6 = param_1;
  iVar4 = param_3;
  uVar5 = (param_3 >> 0x10);
  if (param_4 == 0x0) {
    (iVar4 + 0x16) = 0x0;
  }
  else {
    pass1_1008_b9ce(param_3,param_4);
    (iVar4 + 0x16) = uVar6;
    *(u8 **)(iVar4 + 0x18) = param_2;
  }
  uVar1 = (iVar4 + 0x16);
  if ((uVar1 + 0x8) != 0x0) {
    pass1_1008_b200((astruct_194 *)param_3);
    uVar6 = pass1_1008_b38c((StructD *)CONCAT22(uVar6,param_2),(astruct_196 *)param_3);
    uVar3 = (uVar6 >> 0x10);
    uVar2 = uVar6;
    uVar1 = (iVar4 + 0x16);
    pass1_1008_b85c(param_3,*(i32 *)(uVar1 + 0xa));
    (iVar4 + 0x1a) = uVar2;
    (iVar4 + 0x1c) = uVar3;
    uVar1 = (iVar4 + 0x16);
    lVar7 = pass1_1008_b8ac(param_3,(uVar1 + 0xe));
    (iVar4 + 0x1e) = lVar7;
    (iVar4 + 0x20) = (lVar7 >> 0x10);
    return;
  }
  (iVar4 + 0x1a) = 0x0;
  (iVar4 + 0x1e) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_b544(mut param_1: u16 ,mut param_2: u32,mut param_3: i16)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut unaff_CS: u16;

  iVar7 = param_2;
  uVar8 = (param_2 >> 0x10);
  if (param_3 != 0x0) {
    if (*(i32 *)(iVar7 + 0x1a) != 0x0) {
      uVar4 = (iVar7 + 0x16);
      (uVar4 + 0x8) = 0x1;
      uVar4 = (iVar7 + 0x1a);
      uVar5 = (iVar7 + 0x16);
      (uVar5 + 0xa) = (uVar4 + 0x8);
      uVar4 = (iVar7 + 0x1e);
      uVar6 = (uVar4 + 0x8);
      uVar4 = (iVar7 + 0x16);
      (uVar4 + 0xe) = uVar6;
      uVar4 = (iVar7 + 0x16);
      pass1_1030_8344(_u16_1050_5748,(uVar4 + 0xa));
      unaff_CS = SUB42(&u16_1050_1038,0x0);
      pass1_1038_3608(CONCAT22(param_1,uVar6));
    }
  }
  (iVar7 + 0x1e) = 0x0;
  (iVar7 + 0x1a) = 0x0;
  (iVar7 + 0x16) = 0x0;
  puVar1 = (iVar7 + 0xe);
  uVar2 = (iVar7 + 0x10);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(unaff_CS,puVar1,uVar2,0x1);
  }
  (iVar7 + 0xe) = 0x0;
  puVar1 = (iVar7 + 0x12);
  uVar2 = (iVar7 + 0x14);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(unaff_CS,puVar1,uVar2,0x1);
  }
  (iVar7 + 0x12) = 0x0;
  return;
}
pub fn pass1_1008_b61a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u16;

  pass1_1008_b8fa(param_2,param_3,param_4);
  uVar1 = (param_3 >> 0x10);
  (param_3 + 0x1a) = param_1;
  (param_3 + 0x1c) = param_2;
  return;
}
pub fn pass1_1008_b63a(mut param_1: u32,mut param_2: u32)

{
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar1: u16;

  pass1_1008_b964(param_1,param_2);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x1e) = in_AX;
  (param_1 + 0x20) = in_DX;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn load_string_1008_b65a(mut param_1: u32,LPSTR in_string_2,mut param_3: u32,mut param_4: u16 )

{
  pass1_1008_b9ce(param_1,CONCAT22(param_4,param_3));
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,in_string_2,(short)param_3)
  ;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn load_str_and_sprintf_1008_b69c(mut param_1: u16 ,param_2: *mut astruct_25)

{
  code **ppcVar1;
  let mut in_buffer_4: *mut c_char;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut paVar10: *mut Struct57;
  astruct_25 *iVar5;
  astruct_25 *uVar5;
  let mut iStack516: i16;
  char local_202 [0x100];
  WORD local_102 [0x80];
  let mut paVar9: *mut Struct57;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  in_buffer_4 = local_202;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,in_buffer_4,(short)&DAT_1050_1050);
  uVar5 = (astruct_25 *)(param_2 >> 0x10);
  iVar5 = (astruct_25 *)param_2;
  if (iVar5->field10_0xa == NULL) {
    mem_op_1000_179c(0xc,paVar8);
    uVar4 = paVar8 | in_buffer_4;
    paVar10 = (astruct_57 *)(paVar8 & 0xffff0000);
    paVar9 = (astruct_57 *)(paVar10 | uVar4);
    if (uVar4 == 0x0) {
      uVar4 = 0x0;
    }
    else {
      uVar4 = set_struct_1008_574a((astruct_57 *)CONCAT22(paVar8,in_buffer_4));
      paVar10 = paVar9;
    }
    &iVar5->field10_0xa = uVar4;
    (&iVar5->field10_0xa + 0x2) = paVar10;
    for (iStack516 = 0x1; iStack516 < 0x6; iStack516 += 0x1) {
      mem_op_1000_179c(0x12,paVar10);
      uVar7 = paVar10 | uVar4;
      paVar8 = (astruct_57 *)(paVar10 & 0xffff0000 | uVar7);
      if (uVar7 == 0x0) {
        iVar3 = 0x0;
        paVar10 = (astruct_57 *)(paVar10 & 0xffff0000);
      }
      else {
        iVar3 = set_stuct_1008_b0bc((StructD *)CONCAT22(paVar10,uVar4));
        paVar10 = paVar8;
      }
      uVar6 = SUB42(paVar10,0x0);
      wsprintf16(local_102,CONCAT22(local_202,0x1050),CONCAT22(iStack516,0x1050),iVar3,uVar6,uVar4);
      uVar2 = str_op_1008_60e8(paVar10,CONCAT22(0x1050,local_102));
      (iVar3 + 0x4) = uVar2;
      (iVar3 + 0x6) = paVar10;
      ppcVar1 = (code **)(*iVar5->field10_0xa + 0x8);
      uVar4 = (**ppcVar1)();
    }
    iVar5->field31_0x22 = 0x5;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn load_str_and_sprintf_1008_b78a(StructD *param_1,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut paVar6: *mut Struct57;
  let mut iVar7: i16;
  let mut uVar8: u16;
  char local_206 [0x100];
  WORD local_106 [0x80];
  let mut iStack6: i16;
  let mut uStack4: u16;

  uVar3 = (param_1 >> 0x10);
  paVar6 = (astruct_57 *)(param_1 & 0xffff0000 | param_1 & 0xffff);
  mem_op_1000_179c(0x12,paVar6);
  uVar5 = paVar6 | uVar3;
  if (uVar5 == 0x0) {
    iStack6 = 0x0;
    uVar5 = 0x0;
  }
  else {
    iStack6 = set_stuct_1008_b0bc((StructD *)CONCAT22(paVar6,uVar3));
  }
  uStack4 = uVar5;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,(short)&DAT_1050_1050);
  uVar8 = (param_2 >> 0x10);
  iVar7 = param_2;
  piVar1 = (iVar7 + 0x22);
  *piVar1 = *piVar1 + 0x1;
  wsprintf16(local_106,CONCAT22(local_206,0x1050),CONCAT22((iVar7 + 0x22),0x1050),uVar3);
  uVar4 = str_op_1008_60e8(uVar5,CONCAT22(0x1050,local_106));
  (iStack6 + 0x4) = uVar4;
  (iStack6 + 0x6) = uVar5;
  ppcVar2 = (code **)((iVar7 + 0xa) + 0x8);
  (**ppcVar2)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_b820(mut param_1: i16,mut param_2: u16 ,mut param_3: u32) -> u32

{
  let mut uVar1: u16;

  pass1_1030_8344(_u16_1050_5748,0x8000001);
  if ((param_1 + 0x152) == 0x0) {
    return 0x0;
  }
  uVar1 = (param_3 >> 0x10);
  return CONCAT22((param_3 + 0xc),(param_3 + 0xa));
}
pub fn pass1_1008_b85c(mut param_1: u32,i32 param_2)

{
  let mut puVar1: *mut u8,
  let mut extraout_DX: u16;
  u8 local_a [0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0xe));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,puVar1));
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
  } while (*(i32 *)(puVar1 + 0x8) != param_2);
  return;
}



i32 pass1_1008_b8ac(mut param_1: u32,mut param_2: i16)

{
  i32 lVar1;
  u8 local_a [0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x12));
  do {
    lVar1 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (lVar1 == 0x0) {
      return 0x0;
    }
  } while ((lVar1 + 0x8) != param_2);
  return lVar1;
}
pub fn pass1_1008_b8fa(mut param_1: u16 ,mut param_2: u32,char *param_3)

{
  let mut puVar1: *mut u8,
  let mut iVar2: i16;
  let mut extraout_DX: u16;
  u8 local_a [0x8];

  if (param_3 == NULL) {
    return;
  }
  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_2 + 0xe));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,puVar1));
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a(*(char **)(puVar1 + 0x4),param_3);
  } while (iVar2 != 0x0);
  return;
}
pub fn pass1_1008_b964(mut param_1: u32,char *param_2)

{
  let mut string_1: *mut c_char;
  let mut iVar1: i16;
  let mut extraout_DX: u16;
  char local_a [0x8];

  if (param_2 == NULL) {
    return;
  }
  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x12));
  do {
    string_1 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,string_1));
    if ((extraout_DX | string_1) == 0x0) {
      return;
    }
    iVar1 = pass1_1000_3d7a(*(char **)(string_1 + 0x4),param_2);
  } while (iVar1 != 0x0);
  return;
}
pub fn pass1_1008_b9ce(mut param_1: u32,char *param_2)

{
  let mut puVar1: *mut u8,
  let mut iVar2: i16;
  let mut extraout_DX: u16;
  u8 local_a [0x8];

  if (param_2 == NULL) {
    return;
  }
  pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0xa));
  do {
    puVar1 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,puVar1));
    if ((extraout_DX | puVar1) == 0x0) {
      return;
    }
    iVar2 = pass1_1000_3d7a(*(char **)(puVar1 + 0x4),param_2);
  } while (iVar2 != 0x0);
  return;
}
pub fn pass1_1008_ba38(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut puVar3: *mut u8,
  let mut extraout_DX: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  HFILE16 in_stack_0000ffc0;
  u32 local_2a [0x3];
  u16 local_1e [0x5];
  u8 local_14 [0x8];
  let mut local_c: u16;
  let mut uStack10: u32;
  u16 local_6 [0x2];

  BVar2 = write_to_file_1008_7cac(param_2);
  if (BVar2 != 0x0) {
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    local_c = (iVar4 + 0x22);
    BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffc0);
    if (BVar2 != 0x0) {
      if (*(i32 *)(iVar4 + 0xa) == 0x0) {
        local_c = 0x0;
      }
      else {
        uVar1 = (iVar4 + 0xa);
        local_c = (uVar1 + 0x8);
      }
      local_1e[0] = local_c;
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_1e),0x2,in_stack_0000ffc0);
      if (BVar2 != 0x0) {
        pass1_1008_5784(CONCAT22(0x1050,local_14),(iVar4 + 0xa));
        do {
          puVar3 = local_14;
          pass1_1008_5b12(CONCAT22(0x1050,puVar3));
          uStack10 = CONCAT22(extraout_DX,puVar3);
          if ((extraout_DX | puVar3) == 0x0) {
            return;
          }
          BVar2 = pass1_1008_7c2a(param_2,*(char **)(puVar3 + 0x4));
          if (BVar2 == 0x0) break;
          local_6[0] = (uStack10 + 0x8);
          BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_6),0x2,in_stack_0000ffc0);
          if (BVar2 == 0x0) break;
          local_2a[0] = (uStack10 + 0xa);
          BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_2a),0x4,in_stack_0000ffc0);
          if (BVar2 == 0x0) break;
          local_6[0] = (uStack10 + 0xe);
          BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_6),0x2,in_stack_0000ffc0);
        } while (BVar2 != 0x0);
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}
pub fn file_1008_bb5e(mut param_1: i16,StructD *param_2,param_3: *mut astruct_199,mut param_4: u32)

{
  code **ppcVar1;
  astruct_199 *iVar3;
  let mut BVar2: bool;
  StructD *uVar3;
  astruct_200 *uVar4;
  let mut puVar3: *mut u8,
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut paVar10: *mut Struct57;
  let mut uVar13: u16;
  let mut uVar14: u16;
  astruct_200 *paStack286;
  let mut puStack284: *mut u32;
  u8 local_118 [0x100];
  u16 local_18 [0x2];
  u16 local_14 [0x2];
  astruct_200 *local_10 [0x4];
  let mut local_8: u32;
  astruct_199 *uVar12;
  astruct_199 *uVar11;
  astruct_199 *uVar2;
  let mut paVar9: *mut Struct57;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if (u16_1050_0312 < 0x2) {
    return;
  }
  uVar14 = (param_4 >> 0x10);
  read_file_1008_7cfe((HFILE16)param_4,uVar14,0x16);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
  }
  else {
    iVar3 = (astruct_199 *)param_3;
    iVar3 = (astruct_199 *)&iVar3->field31_0x22;
    BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | ZEXT24(iVar3)),0x2);
    if ((BVar2 != 0x0) &&
       (uVar3 = (StructD *)read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(0x1050,local_10),0x2), uVar3 != NULL))
    {
      if (local_10[0] == NULL) {
        return;
      }
      mem_op_1000_179c(0xc,paVar8);
      uVar6 = paVar8 | uVar3;
      paVar10 = (astruct_57 *)(paVar8 & 0xffff0000);
      paVar9 = (astruct_57 *)(paVar10 | uVar6);
      if (uVar6 == 0x0) {
        uVar3 = NULL;
      }
      else {
        set_struct_1008_574a((astruct_57 *)CONCAT22(paVar8,uVar3));
        paVar10 = paVar9;
      }
      uVar13 = (param_3 >> 0x10);
      *(StructD **)&iVar3->field10_0xa = uVar3;
      (&iVar3->field10_0xa + 0x2) = paVar10;
      paStack286 = NULL;
      while( true ) {
        if (local_10[0] <= paStack286) {
          return;
        }
        uVar4 = local_10[0];
        mem_op_1000_179c(0x12,paVar10);
        uVar6 = paVar10 | uVar4;
        paVar8 = (astruct_57 *)(paVar10 & 0xffff0000 | uVar6);
        if (uVar6 == 0x0) {
          uVar4 = NULL;
          paVar10 = (astruct_57 *)(paVar10 & 0xffff0000);
        }
        else {
          set_stuct_1008_b0bc((StructD *)CONCAT22(paVar10,uVar4));
          paVar10 = paVar8;
        }
        uVar7 = SUB42(paVar10,0x0);
        puStack284 = CONCAT22(uVar7,uVar4);
        puVar3 = local_118;
        read_file_1008_7c6e((HFILE16)param_4,uVar14,CONCAT22(0x1050,puVar3));
        if ((((puVar3 == NULL) ||
             (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(0x1050,local_14),0x2), BVar2 == 0x0)) ||
            (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(0x1050,&local_8),0x4), BVar2 == 0x0)) ||
           (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(0x1050,local_18),0x2), BVar2 == 0x0)) break;
        uVar5 = str_op_1008_60e8(paVar10,CONCAT22(0x1050,local_118));
        uVar4->field3_0x4 = uVar5;
        uVar4->field4_0x6 = paVar10;
        uVar4->field5_0x8 = local_14[0];
        uVar4->field6_0xa = local_8;
        uVar4->field7_0xe = local_18[0];
        ppcVar1 = (code **)(iVar3->field10_0xa + 0x8);
        (**ppcVar1)();
        paStack286 = (astruct_200 *)&paStack286->field1_0x1;
      }
      if (puStack284 != NULL) {
        ppcVar1 = (code **)*puStack284;
        (**ppcVar1)(0x1000,uVar4,uVar7,0x1,puStack284);
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



pub fn pass1_1008_bd02(mut param_1: u32,param_2: u8) -> u32

{
  pass1_1008_afde((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



StructD * pass1_1008_bd28(StructD *param_1,param_2: u8)

{
  pass1_1008_b08c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



StructD * pass1_1008_bd4e(StructD *param_1,param_2: u8)

{
  pass1_1008_b08c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



StructD * pass1_1008_bd74(StructD *param_1,param_2: u8)

{
  pass1_1008_b08c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



StructD * pass1_1008_bd9a(StructD *param_1,param_2: u8)

{
  pass1_1008_b08c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1008_bde0(StructD *param_1,param_2: *mut astruct_139)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  StructD *pSVar2;
  astruct_139 *iVar2;
  astruct_139 *iVar3;
  astruct_139 *iVar4;
  astruct_139 *iVar5;
  astruct_139 *iVar6;
  astruct_139 *iVar7;
  astruct_139 *iVar8;
  astruct_139 *iVar9;
  astruct_139 *iVar10;
  astruct_139 *iVar11;
  astruct_139 *iVar12;
  astruct_139 *iVar2_00;
  astruct_139 *iVar2_01;
  astruct_139 *iVar2_02;
  astruct_139 *iVar2_03;
  astruct_139 *iVar2_04;
  astruct_139 *iVar2_05;
  astruct_139 *iVar2_06;
  astruct_139 *iVar2_07;
  astruct_139 *iVar2_08;
  astruct_139 *iVar2_09;
  let mut iVar13: i16;
  astruct_139 *iVar2_10;
  astruct_139 *iVar2_11;
  astruct_139 *iVar2_12;
  astruct_139 *iVar2_13;
  astruct_139 *iVar2_14;
  astruct_139 *iVar2_15;
  astruct_139 *iVar2_16;
  astruct_139 *iVar2_17;
  astruct_139 *iVar2_18;
  astruct_139 *iVar2_19;
  astruct_139 *iVar2_20;
  astruct_139 *iVar2_21;
  astruct_139 *iVar2_22;
  astruct_139 *iVar2_23;
  astruct_139 *iVar2_24;
  astruct_139 *iVar2_25;
  astruct_139 *iVar2_26;
  astruct_139 *iVar2_27;
  astruct_139 *iVar2_28;
  astruct_139 *iVar2_29;
  astruct_139 *iVar2_30;
  astruct_139 *iVar2_31;
  astruct_139 *iVar2_32;
  astruct_139 *iVar2_33;
  astruct_139 *iVar2_34;
  astruct_139 *iVar2_35;
  astruct_139 *iVar2_36;
  astruct_139 *iVar2_37;
  astruct_139 *iVar2_38;
  astruct_139 *iVar2_39;
  astruct_139 *iVar2_40;
  astruct_139 *iVar2_41;
  astruct_139 *iVar2_42;
  astruct_139 *iVar2_43;
  astruct_139 *iVar2_44;
  astruct_139 *iVar2_45;
  astruct_139 *iVar2_46;
  astruct_139 *iVar2_47;
  astruct_139 *iVar2_48;
  astruct_139 *iVar2_49;
  astruct_139 *iVar2_50;
  let mut uVar3: u16;
  let mut uVar14: u16;

  pSVar2 = (StructD *)CONCAT22(in_register_0000000a,param_1);
  _u16_1050_06e0 = param_2;
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar2);
    PTR_LOOP_1050_5f2e = pSVar2;
  }
  else {
  }
  uVar1 = fn_ptr_op_1000_1708(0x1aa,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
  param_2 = uVar1;
  (param_2 + 0x2) = PTR_LOOP_1050_5f2e;
  uVar3 = (param_2 >> 0x10);
  iVar2 = (astruct_139 *)param_2;
  iVar2->field5_0x6 = 0x6e4;
  iVar2->field6_0x8 = &DAT_1050_1050;
  (param_2 + 0xa) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar3 = (astruct_139 *)param_2;
  iVar3[0x1].field2_0x2 = 0x6ea;
  &iVar3[0x1].field_0x4 = &DAT_1050_1050;
  (param_2 + 0x10) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar4 = (astruct_139 *)param_2;
  iVar4[0x1].field6_0x8 = 0x6ee;
  (iVar4 + 0x2) = &DAT_1050_1050;
  (param_2 + 0x16) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar5 = (astruct_139 *)param_2;
  &iVar5[0x2].field_0x4 = 0x6f2;
  iVar5[0x2].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0x1c) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar6 = (astruct_139 *)param_2;
  (iVar6 + 0x3) = 0x6f6;
  iVar6[0x3].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0x22) = 0x4;
  uVar14 = (param_2 >> 0x10);
  iVar7 = (astruct_139 *)param_2;
  iVar7[0x3].field5_0x6 = 0x6fe;
  iVar7[0x3].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0x28) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar8 = (astruct_139 *)param_2;
  iVar8[0x4].field2_0x2 = 0x702;
  &iVar8[0x4].field_0x4 = &DAT_1050_1050;
  (param_2 + 0x2e) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar9 = (astruct_139 *)param_2;
  iVar9[0x4].field6_0x8 = 0x708;
  (iVar9 + 0x5) = &DAT_1050_1050;
  (param_2 + 0x34) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar10 = (astruct_139 *)param_2;
  &iVar10[0x5].field_0x4 = 0x70e;
  iVar10[0x5].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0x3a) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar11 = (astruct_139 *)param_2;
  (iVar11 + 0x6) = 0x714;
  iVar11[0x6].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0x40) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar12 = (astruct_139 *)param_2;
  iVar12[0x6].field5_0x6 = 0x71a;
  iVar12[0x6].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0x46) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_00 = (astruct_139 *)param_2;
  iVar2_00[0x7].field2_0x2 = 0x71e;
  &iVar2_00[0x7].field_0x4 = &DAT_1050_1050;
  (param_2 + 0x4c) = 0x7;
  uVar14 = (param_2 >> 0x10);
  iVar2_01 = (astruct_139 *)param_2;
  iVar2_01[0x7].field6_0x8 = 0x72c;
  (iVar2_01 + 0x8) = &DAT_1050_1050;
  (param_2 + 0x52) = 0x6;
  uVar14 = (param_2 >> 0x10);
  iVar2_02 = (astruct_139 *)param_2;
  &iVar2_02[0x8].field_0x4 = 0x738;
  iVar2_02[0x8].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0x58) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar2_03 = (astruct_139 *)param_2;
  (iVar2_03 + 0x9) = 0x73e;
  iVar2_03[0x9].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0x5e) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar2_04 = (astruct_139 *)param_2;
  iVar2_04[0x9].field5_0x6 = 0x744;
  iVar2_04[0x9].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0x64) = 0x4;
  uVar14 = (param_2 >> 0x10);
  iVar2_05 = (astruct_139 *)param_2;
  iVar2_05[0xa].field2_0x2 = 0x74c;
  &iVar2_05[0xa].field_0x4 = &DAT_1050_1050;
  (param_2 + 0x6a) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_06 = (astruct_139 *)param_2;
  iVar2_06[0xa].field6_0x8 = 0x750;
  (iVar2_06 + 0xb) = &DAT_1050_1050;
  (param_2 + 0x70) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar2_07 = (astruct_139 *)param_2;
  &iVar2_07[0xb].field_0x4 = 0x756;
  iVar2_07[0xb].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0x76) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_08 = (astruct_139 *)param_2;
  (iVar2_08 + 0xc) = 0x75a;
  iVar2_08[0xc].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0x7c) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_09 = (astruct_139 *)param_2;
  iVar2_09[0xc].field5_0x6 = 0x75e;
  iVar2_09[0xc].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0x82) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar13 = param_2;
  (iVar13 + 0x84) = 0x764;
  (iVar13 + 0x86) = &DAT_1050_1050;
  (param_2 + 0x88) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar2_10 = (astruct_139 *)param_2;
  iVar2_10[0xd].field6_0x8 = 0x76a;
  (iVar2_10 + 0xe) = &DAT_1050_1050;
  (param_2 + 0x8e) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar2_11 = (astruct_139 *)param_2;
  &iVar2_11[0xe].field_0x4 = 0x770;
  iVar2_11[0xe].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0x94) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar13 = param_2;
  (iVar13 + 0x96) = 0x774;
  (iVar13 + 0x98) = &DAT_1050_1050;
  (param_2 + 0x9a) = 0x4;
  uVar14 = (param_2 >> 0x10);
  iVar2_12 = (astruct_139 *)param_2;
  iVar2_12[0xf].field5_0x6 = 0x77c;
  iVar2_12[0xf].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0xa0) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_13 = (astruct_139 *)param_2;
  iVar2_13[0x10].field2_0x2 = 0x780;
  &iVar2_13[0x10].field_0x4 = &DAT_1050_1050;
  (param_2 + 0xa6) = 0x1;
  uVar14 = (param_2 >> 0x10);
  iVar2_14 = (astruct_139 *)param_2;
  iVar2_14[0x10].field6_0x8 = 0x782;
  (iVar2_14 + 0x11) = &DAT_1050_1050;
  (param_2 + 0xac) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar13 = param_2;
  (iVar13 + 0xae) = 0x786;
  (iVar13 + 0xb0) = &DAT_1050_1050;
  (param_2 + 0xb2) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_15 = (astruct_139 *)param_2;
  (iVar2_15 + 0x12) = 0x78a;
  iVar2_15[0x12].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0xb8) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_16 = (astruct_139 *)param_2;
  iVar2_16[0x12].field5_0x6 = 0x78e;
  iVar2_16[0x12].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0xbe) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_17 = (astruct_139 *)param_2;
  iVar2_17[0x13].field2_0x2 = 0x792;
  &iVar2_17[0x13].field_0x4 = &DAT_1050_1050;
  (param_2 + 0xc4) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar13 = param_2;
  (iVar13 + 0xc6) = 0x796;
  (iVar13 + 0xc8) = &DAT_1050_1050;
  (param_2 + 0xca) = 0x4;
  uVar14 = (param_2 >> 0x10);
  iVar2_18 = (astruct_139 *)param_2;
  &iVar2_18[0x14].field_0x4 = 0x79e;
  iVar2_18[0x14].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0xd0) = 0x1;
  uVar14 = (param_2 >> 0x10);
  iVar13 = param_2;
  (iVar13 + 0xd2) = 0x7a0;
  (iVar13 + 0xd4) = &DAT_1050_1050;
  (param_2 + 0xd6) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_19 = (astruct_139 *)param_2;
  iVar2_19[0x15].field5_0x6 = 0x7a4;
  iVar2_19[0x15].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0xdc) = 0x1;
  uVar14 = (param_2 >> 0x10);
  iVar2_20 = (astruct_139 *)param_2;
  iVar2_20[0x16].field2_0x2 = 0x7a6;
  &iVar2_20[0x16].field_0x4 = &DAT_1050_1050;
  (param_2 + 0xe2) = 0x6;
  uVar14 = (param_2 >> 0x10);
  iVar2_21 = (astruct_139 *)param_2;
  iVar2_21[0x16].field6_0x8 = 0x7b2;
  (iVar2_21 + 0x17) = &DAT_1050_1050;
  (param_2 + 0xe8) = 0x1;
  uVar14 = (param_2 >> 0x10);
  iVar2_22 = (astruct_139 *)param_2;
  &iVar2_22[0x17].field_0x4 = 0x7b4;
  iVar2_22[0x17].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0xee) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar2_23 = (astruct_139 *)param_2;
  (iVar2_23 + 0x18) = 0x7ba;
  iVar2_23[0x18].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0xf4) = 0x2d;
  uVar14 = (param_2 >> 0x10);
  iVar2_24 = (astruct_139 *)param_2;
  iVar2_24[0x18].field5_0x6 = 0x814;
  iVar2_24[0x18].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0xfa) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar2_25 = (astruct_139 *)param_2;
  iVar2_25[0x19].field2_0x2 = 0x81a;
  &iVar2_25[0x19].field_0x4 = &DAT_1050_1050;
  (param_2 + 0x100) = 0x1;
  uVar14 = (param_2 >> 0x10);
  iVar2_26 = (astruct_139 *)param_2;
  iVar2_26[0x19].field6_0x8 = 0x81c;
  (iVar2_26 + 0x1a) = &DAT_1050_1050;
  (param_2 + 0x106) = 0x4b;
  uVar14 = (param_2 >> 0x10);
  iVar2_27 = (astruct_139 *)param_2;
  &iVar2_27[0x1a].field_0x4 = 0x8b2;
  iVar2_27[0x1a].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0x10c) = 0x6;
  uVar14 = (param_2 >> 0x10);
  iVar2_28 = (astruct_139 *)param_2;
  (iVar2_28 + 0x1b) = 0x8be;
  iVar2_28[0x1b].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0x112) = 0x4;
  uVar14 = (param_2 >> 0x10);
  iVar2_29 = (astruct_139 *)param_2;
  iVar2_29[0x1c].field2_0x2 = 0x8c6;
  &iVar2_29[0x1c].field_0x4 = &DAT_1050_1050;
  (param_2 + 0x11e) = 0x35;
  uVar14 = (param_2 >> 0x10);
  iVar2_30 = (astruct_139 *)param_2;
  iVar2_30[0x1c].field6_0x8 = 0x930;
  (iVar2_30 + 0x1d) = &DAT_1050_1050;
  (param_2 + 0x124) = 0x2e;
  uVar14 = (param_2 >> 0x10);
  iVar2_31 = (astruct_139 *)param_2;
  iVar2_31[0x1b].field5_0x6 = 0x98c;
  iVar2_31[0x1b].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0x118) = 0x1;
  uVar14 = (param_2 >> 0x10);
  iVar2_32 = (astruct_139 *)param_2;
  &iVar2_32[0x1d].field_0x4 = 0x98e;
  iVar2_32[0x1d].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0x12a) = 0x9;
  uVar14 = (param_2 >> 0x10);
  iVar2_33 = (astruct_139 *)param_2;
  (iVar2_33 + 0x1e) = 0x9a0;
  iVar2_33[0x1e].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0x130) = 0x1a;
  uVar14 = (param_2 >> 0x10);
  iVar13 = param_2;
  (iVar13 + 0x132) = 0x9d4;
  (iVar13 + 0x134) = &DAT_1050_1050;
  (param_2 + 0x136) = 0x8;
  uVar14 = (param_2 >> 0x10);
  iVar2_34 = (astruct_139 *)param_2;
  iVar2_34[0x1f].field2_0x2 = 0x9e4;
  &iVar2_34[0x1f].field_0x4 = &DAT_1050_1050;
  (param_2 + 0x13c) = 0x4a;
  uVar14 = (param_2 >> 0x10);
  iVar2_35 = (astruct_139 *)param_2;
  &iVar2_35[0x20].field_0x4 = 0xa78;
  iVar2_35[0x20].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0x148) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_36 = (astruct_139 *)param_2;
  (iVar2_36 + 0x21) = 0xa7c;
  iVar2_36[0x21].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0x14e) = 0x1;
  uVar14 = (param_2 >> 0x10);
  iVar2_37 = (astruct_139 *)param_2;
  iVar2_37[0x21].field5_0x6 = 0xa7e;
  iVar2_37[0x21].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0x154) = 0x1;
  uVar14 = (param_2 >> 0x10);
  iVar2_38 = (astruct_139 *)param_2;
  iVar2_38[0x22].field2_0x2 = 0xa80;
  &iVar2_38[0x22].field_0x4 = &DAT_1050_1050;
  (param_2 + 0x15a) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar2_39 = (astruct_139 *)param_2;
  iVar2_39[0x22].field6_0x8 = 0xa86;
  (iVar2_39 + 0x23) = &DAT_1050_1050;
  (param_2 + 0x160) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_40 = (astruct_139 *)param_2;
  (iVar2_40 + 0x24) = 0xa8a;
  iVar2_40[0x24].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0x16c) = 0x1b;
  uVar14 = (param_2 >> 0x10);
  iVar2_41 = (astruct_139 *)param_2;
  iVar2_41[0x24].field5_0x6 = 0xac0;
  iVar2_41[0x24].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0x172) = 0x16;
  uVar14 = (param_2 >> 0x10);
  iVar2_42 = (astruct_139 *)param_2;
  iVar2_42[0x25].field2_0x2 = 0xaec;
  &iVar2_42[0x25].field_0x4 = &DAT_1050_1050;
  (param_2 + 0x178) = 0x3e;
  uVar14 = (param_2 >> 0x10);
  iVar2_43 = (astruct_139 *)param_2;
  iVar2_43[0x25].field6_0x8 = 0xb68;
  (iVar2_43 + 0x26) = &DAT_1050_1050;
  (param_2 + 0x17e) = 0x46;
  uVar14 = (param_2 >> 0x10);
  iVar2_44 = (astruct_139 *)param_2;
  &iVar2_44[0x26].field_0x4 = 0xbf4;
  iVar2_44[0x26].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0x184) = 0x1;
  uVar14 = (param_2 >> 0x10);
  iVar2_45 = (astruct_139 *)param_2;
  (iVar2_45 + 0x27) = 0xbf6;
  iVar2_45[0x27].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0x18a) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar2_46 = (astruct_139 *)param_2;
  iVar2_46[0x27].field5_0x6 = 0xbfc;
  iVar2_46[0x27].field6_0x8 = &DAT_1050_1050;
  (param_2 + 0x190) = 0x3;
  uVar14 = (param_2 >> 0x10);
  iVar2_47 = (astruct_139 *)param_2;
  iVar2_47[0x28].field2_0x2 = 0xc02;
  &iVar2_47[0x28].field_0x4 = &DAT_1050_1050;
  (param_2 + 0x196) = 0xa;
  uVar14 = (param_2 >> 0x10);
  iVar2_48 = (astruct_139 *)param_2;
  iVar2_48[0x28].field6_0x8 = 0xc16;
  (iVar2_48 + 0x29) = &DAT_1050_1050;
  (param_2 + 0x19c) = 0x24;
  uVar14 = (param_2 >> 0x10);
  iVar2_49 = (astruct_139 *)param_2;
  &iVar2_49[0x29].field_0x4 = 0xc5e;
  iVar2_49[0x29].field5_0x6 = &DAT_1050_1050;
  (param_2 + 0x1a2) = 0x2;
  uVar14 = (param_2 >> 0x10);
  iVar2_50 = (astruct_139 *)param_2;
  (iVar2_50 + 0x2a) = 0xc62;
  iVar2_50[0x2a].field2_0x2 = &DAT_1050_1050;
  (param_2 + 0x1a8) = 0x44;
  return;
}
