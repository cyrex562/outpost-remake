
pub fn pass1_1018_2076(param_1: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x21e8;
  (param_1 + 0x2) = 0x1018;
  pass1_1018_209c(param_1 & 0xffff | uVar1 << 0x10);
  pass1_1010_1d80((StructD *)param_1);
  return;
}
pub fn pass1_1018_209c(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iStack4: i16;

  iStack4 = 0x0;
  do {
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1 + 0x12;
    puVar1 = (iVar4 + iStack4 * 0x4);
    uVar2 = (iVar4 + iStack4 * 0x4 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    (param_1 + iStack4 * 0x4 + 0x12) = 0x0;
    iStack4 += 0x1;
  } while (iStack4 < 0x1fd);
  return;
}
pub fn pass1_1018_20ee(mut param_1: u32,i16 *param_2)

{
  let mut BVar1: bool;
  let mut in_DX: u16;
  let mut uVar2: u16;

  BVar1 = pass1_1008_aed8(param_2);
  if (BVar1 == 0x0) {
    return;
  }
  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + *param_2 * 0x4 + 0x12) == 0x0) {
    pass1_1018_216e(in_DX,param_1 & 0xffff | uVar2 << 0x10,param_2);
  }
  pass1_1008_ae26(param_2);
  return;
}
pub fn pass1_1018_214e(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: i16)

{
  pass1_1008_3e76(param_3,0x0,(param_4 * 0x4 + 0x3e90),(param_4 * 0x4 + 0x3e8e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_216e(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uStack8: u16;

  uStack8 = pass1_1008_adf2(param_3);
  uVar1 = pass1_1008_ae0c(param_3);
  for (; uStack8 <= uVar1; uStack8 += 0x1) {
    uVar2 = uVar1;
    pass1_1010_8018(param_1,_u16_1050_14cc,uStack8);
    uVar3 = (param_2 >> 0x10);
    (param_2 + uStack8 * 0x4 + 0x12) = uVar2;
    *(u8 **)(param_2 + uStack8 * 0x4 + 0x14) = param_1;
  }
  return;
}



pub fn pass1_1018_21c2(mut param_1: u32,param_2: u8) -> u32

{
  pass1_1018_2076((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 pass1_1018_21f8(void)

{
  let mut puVar1: *mut u16;

  pass1_1008_3e54(&u16_1048_4210,0x0,0x195,0x1);
  pass1_1008_3e54(&u16_1050_65ca,0x0,0xe0,0x1b1);
  pass1_1008_3e54(&u16_1050_65d0,0x0,0x17a,0x72);
  pass1_1008_3e54(&u16_1050_65d6,0x0,0xde,0x93);
  pass1_1008_3e54(&u16_1050_65dc,0x0,0x177,0x1da);
  pass1_1008_3e54(&u16_1048_4216,0x0,0x195,0x21c);
  pass1_1008_3e54((u16 *)&u32_1048_421c,0x0,0x1b6,0x22c);
  pass1_1008_3e54((u16 *)((long)&u32_1048_4220 + 0x2),0x0,0x109,0x5);
  puVar1 = pass1_1008_3e54((u16 *)&u32_1048_4228,0x0,0xfd,0x1fd);
  return puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1018_229c(u8 *param_1,param_2: *mut astruct_19,mut param_3: u16 )

{
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut paVar3: *mut Struct57;
  let mut in_register_0000000a: u16;
  let mut uVar4: u32;
  let mut paVar5: *mut Struct57;
  let mut unaff_CS: u16;
  let mut iVar6: i16;

  uVar4 = CONCAT22(in_register_0000000a,param_1);
  struct_op_1018_4cda(param_2,param_3);
  (param_2 + 0x1c) = 0x389a;
  (param_2 + 0x1e) = 0x1008;
  (param_2 + 0x1c) = 0x3aa8;
  (param_2 + 0x1e) = 0x1008;
  uVar1 = 0x0;
  (param_2 + 0x20) = 0x0;
  (param_2 + 0x24) = 0x0;
  (param_2 + 0x26) = 0x0;
  (param_2 + 0x2a) = 0x0;
  (param_2 + 0x3e) = 0x0;
  (param_2 + 0x40) = 0x0;
  (param_2 + 0x42) = 0x0;
  (param_2 + 0x44) = 0x0;
  (param_2 + 0x6e) = 0x0;
  param_2.offset_0x0 = 0x2ada;
  (param_2 + 0x2) = 0x1018;
  (param_2 + 0x1c) = s_fem132_wav_1050_2aec + 0x6;
  (param_2 + 0x1e) = 0x1018;
  _PTR_LOOP_1050_4230 = param_2;
  pass1_1018_4dce(uVar4,param_2,0x105);
  uVar1 = FUN_1010_830a(uVar1,uVar4,unaff_CS,_u16_1050_14cc,0x1a8);
  (param_2 + 0x2a) = uVar1;
  (param_2 + 0x2c) = uVar4;
  pass1_1000_4906((StructD *)(param_2 & 0xffff0000 | ZEXT24((u16 *)(param_2 + 0x2eU))),NULL,0x10);
  puVar2 = pass1_1000_4906((StructD *)(param_2 & 0xffff0000 | (param_2 + 0x46)),NULL,0x28);
  uVar1 = FUN_1010_830a(puVar2,uVar4,0x1000,_u16_1050_14cc,0x6c);
  (param_2 + 0x2eU) = uVar1;
  (param_2 + 0x30) = uVar4;
  uVar1 = FUN_1010_830a(uVar1,uVar4,0x1010,_u16_1050_14cc,0x68);
  (param_2 + 0x32) = uVar1;
  (param_2 + 0x34) = uVar4;
  uVar1 = FUN_1010_830a(uVar1,uVar4,0x1010,_u16_1050_14cc,0x66);
  (param_2 + 0x36) = uVar1;
  (param_2 + 0x38) = uVar4;
  uVar1 = FUN_1010_830a(uVar1,uVar4,0x1010,_u16_1050_14cc,0x6a);
  (param_2 + 0x3a) = uVar1;
  (param_2 + 0x3c) = uVar4;
  uVar1 = FUN_1010_830a(uVar1,uVar4,0x1010,_u16_1050_14cc,0x1cd);
  (param_2 + 0x6e) = uVar1;
  (param_2 + 0x70) = uVar4;
  iVar6 = 0x0;
  do {
    uVar1 = FUN_1010_830a(iVar6 + 0x8f,uVar4,0x1010,_u16_1050_14cc,iVar6 + 0x8f);
    (param_2 + iVar6 * 0x4 + 0x46) = uVar1;
    (param_2 + iVar6 * 0x4 + 0x48) = uVar4;
    iVar6 += 0x1;
  } while (iVar6 < 0xa);
  if (param_2 == NULL) {
    paVar3 = NULL;
    paVar5 = (astruct_57 *)(uVar4 & 0xffff0000);
  }
  else {
    paVar5 = (astruct_57 *)(uVar4 & 0xffff0000 | param_2);
    paVar3 = (astruct_57 *)(param_2 + 0x1c);
  }
  pass1_1008_9262(paVar3,paVar5,_PTR_LOOP_1050_0388,(_PTR_LOOP_1050_0388 >> 0x10),0x73,
                  CONCAT22(paVar5,paVar3));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_2440(StructD *param_1)

{
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  code **ppcVar4;
  u8 *puVar5;
  u8 *puVar4;
  let mut uVar6: u16;
  StructD *uVar5;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  let mut puStack6: *mut u16;
  let mut uVar2: u16;
  let mut puVar1: *mut u32;

  uVar7 = (param_1 >> 0x10);
  uVar5 = (StructD *)param_1;
  param_1.address_offset_field_0x0 = 0x2ada;
  uVar5.address_offset_field_0x2 = 0x1018;
  &uVar5.field_0x1c = s_fem132_wav_1050_2aec + 0x6;
  &uVar5.field_0x1e = 0x1018;
  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (param_1 == NULL) {
      puVar5 = NULL;
      uVar6 = 0x0;
    }
    else {
      puVar5 = &uVar5.field_0x1c;
      uVar6 = uVar7;
    }
    unaff_CS = 0x1008;
    pass1_1008_92b2(_PTR_LOOP_1050_0388,0x73,CONCAT22(uVar6,puVar5));
  }
  puVar1 = (u32*)&uVar5.field_0x2a;
  uVar2 = &uVar5.field29_0x2c;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)(unaff_CS,puVar1,uVar2,0x1);
  }
  puVar2 = (u32*)&uVar5.field_0x6e;
  uVar3 = &uVar5.field_0x70;
  if ((uVar3 | puVar2) != 0x0) {
    ppcVar4 = (code **)*puVar2;
    (**ppcVar4)(unaff_CS,puVar2,uVar3,0x1);
  }
  if (param_1 == NULL) {
    puVar4 = NULL;
    uVar7 = 0x0;
  }
  else {
    puVar4 = &uVar5.field_0x1c;
  }
  puStack6 = (u16 *)CONCAT22(uVar7,puVar4);
  *puStack6 = 0x389a;
  (puVar4 + 0x2) = 0x1008;
  clenaup_win_ui_1018_4d22(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_2504(mut param_1: u16 ,mut param_2: u16 )

{
  let mut uVar1: u16;

  pass1_1030_8344(_u16_1050_5748,0x4000001);
  if ((param_2 | param_1) != 0x0) {
    uVar1 = pass1_1028_d69e(**_u16_1050_5748);
    if (uVar1 == 0x0) {
      return;
    }
  }
  return;
}
pub fn pass1_1018_2548(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16)

{
  pass1_1008_3f62(param_3,(u16 *)&u32_1048_4228);
  return;
}



u16 pass1_1018_255e(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0x26) != 0x0) {
    uVar1 = (param_1 + 0x26);
    return (uVar1 + 0xa);
  }
  return 0x0;
}



u8 * pass1_1018_2580(undefined1 param_1,mut param_2: u32,mut param_3: u16 ,mut param_4: u32,mut param_5: u16 )

{
  u8 *puVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;
  uchar local_8 [0x6];

  uVar3 = (param_2 >> 0x10);
  iVar2 = param_2;
  if (*(i32 *)(iVar2 + 0x20) == 0x0) {
    return 0x6ad;
  }
  pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_8));
  pass1_1018_161c((iVar2 + 0x20),(u16 *)CONCAT22(0x1050,local_8),param_4,(param_4 >> 0x10));
  puVar1 = local_8;
  pass1_1018_17ce((iVar2 + 0x20),CONCAT22(puVar1,param_3),CONCAT22(param_5,0x1050));
  return puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1018_25d2(mut param_1: u32,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut paVar2: *mut Struct57;
  let mut uVar4: u16;
  let mut puVar5: *mut u16;
  let mut puVar6: *mut u32;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut puVar7: *mut u16;
  u8 local_8 [0x6];

  uVar3 = (in_EDX >> 0x10);
  uVar4 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0x20) == 0x0) {
    return 0x0;
  }
  puVar5 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_8));
  paVar2 = (astruct_57 *)CONCAT22(uVar3,(puVar5 >> 0x10));
  pass1_1018_161c((param_1 + 0x20),(u16 *)CONCAT22(0x1050,local_8),param_3,(param_3 >> 0x10)
                 );
  puVar7 = (u16 *)CONCAT22(0x1050,local_8);
  puVar6 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x32),in_stack_0000fe96,
                           in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
  uVar1 = puVar6;
  pass1_1010_71d6(uVar1,(puVar6 >> 0x10),puVar6,param_2,puVar7,&DAT_1050_1050);
  return uVar1;
}
pub fn pass1_1018_262e(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x44) = 0x1;
  (param_1 + 0x3e) = 0x0;
  return;
}
pub fn pass1_1018_2646(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16)

{
  pass1_1008_3f62(param_3,(u16 *)((long)&u32_1048_4220 + 0x2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_265c(void) -> u32

{
  let mut uVar1: u32;

  uVar1 = pass1_1030_8326();
  return uVar1;
}



u16 pass1_1018_266a(mut param_1: u32)

{
  return (param_1 + 0x44);
}
pub fn pass1_1018_2678(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16)

{
  pass1_1008_3f62(param_3,&u16_1048_4216);
  return;
}



pub fn pass1_1018_268e(param_1: *mut astruct_287) -> u32

{
  astruct_287 *iVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar1 = (astruct_287 *)param_1;
  if (iVar1.field65_0x42 != 0x0) {
    &iVar1.field_0x40 = 0x0;
    iVar1.field66_0x44 = 0x1;
  }
  iVar2 = iVar1.field62_0x3e * 0x4;
  return CONCAT22((&iVar1[0x1].field_0x2 + iVar2),(&iVar1[0x1].field_0x0 + iVar2));
}
pub fn pass1_1018_26c2(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16)

{
  pass1_1008_3f62(param_3,(u16 *)&u32_1048_421c);
  return;
}
pub fn pass1_1018_26d8(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,param_4: *mut u16)

{
  pass1_1008_3f62(param_4,(u16 *)CONCAT22(0x1050,&u16_1050_65ca + param_3 * 0x6));
  return;
}
pub fn pass1_1018_26f8(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16)

{
  pass1_1008_3f62(param_3,&u16_1048_4210);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_270e(u8 *param_1,param_2: *mut astruct_27,mut param_3: i16,mut param_4: u16 )

{
  code **ppcVar1;
  let mut uVar2: u32;
  u8 *puVar3;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  astruct_27 *iVar5;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000fff4: u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  iVar5 = (astruct_27 *)param_2;
  uVar6 = (param_2 >> 0x10);
  if (param_3 == 0x0) {
    if ((*(i32 *)&iVar5.field_0x20 == 0x0) ||
       (uVar2 = &iVar5.field_0x20, (uVar2 + 0x8) != param_4)) {
      puVar7 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff4,param_4),in_stack_0000fe9c
                               ,in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
      if (*(i32 *)&iVar5.field_0x20 != 0x0) {
        if (param_2 == NULL) {
          puVar3 = NULL;
          uVar4 = 0x0;
        }
        else {
          puVar3 = &iVar5.field_0x1c;
          uVar4 = uVar6;
        }
        pass1_1010_1ea6(&iVar5.field_0x20,(StructD *)CONCAT22(uVar4,puVar3));
      }
      (u32*)&iVar5.field_0x20 = puVar7;
      if (param_2 == NULL) {
        param_4 = 0x0;
        uVar4 = 0x0;
      }
      else {
        param_4 = &iVar5.field_0x1c;
        uVar4 = uVar6;
      }
      paVar5 = (astruct_57 *)uVar4;
      uVar2 = &iVar5.field_0x20;
      ppcVar1 = (code **)(*(u32*)&iVar5.field_0x20 + 0x4);
      (**ppcVar1)(0x1010,uVar2,(uVar2 >> 0x10),0x0,param_4,uVar4);
    }
    uVar4 = paVar5;
    pass1_1018_2862((astruct_654 *)param_2);
    if ((uVar4 | param_4) != 0x0) {
      (&iVar5.field30_0x22 + 0x2) = 0x1;
    }
    pass1_1010_1f62(param_2,0x7);
  }
  else if ((&iVar5.field30_0x22 | &iVar5.field_0x20) != 0x0) {
    if (param_2 == NULL) {
      puVar3 = NULL;
      uVar4 = 0x0;
    }
    else {
      puVar3 = &iVar5.field_0x1c;
      uVar4 = uVar6;
    }
    pass1_1010_1ea6(&iVar5.field_0x20,(StructD *)CONCAT22(uVar4,puVar3));
    &iVar5.field_0x20 = 0x0;
    return;
  }
  return;
}
pub fn pass1_1018_280c(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0x24) == 0x0) {
    return;
  }
  (iVar2 + 0x24) = 0x0;
  if (*(i32 *)(iVar2 + 0x20) == 0x0) {
    (iVar2 + 0x26) = 0x0;
  }
  else {
    uVar1 = (iVar2 + 0x20);
    (iVar2 + 0x26) = (uVar1 + 0x4c);
  }
  return;
}
pub fn pass1_1018_2862(param_1: *mut astruct_654)

{
  i32 lVar1;
  astruct_654 *iVar2;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_654 *)param_1;
  if (iVar2.field32_0x20 == 0x0) {
    iVar2.field35_0x26 = 0x0;
  }
  else {
    lVar1 = iVar2.field32_0x20;
    iVar2.field35_0x26 = (lVar1 + 0x4c);
  }
  return;
}
pub fn pass1_1018_289c(mut param_1: u16 ,mut param_2: u32,mut param_3: i16)

{
  let mut uVar1: u16;

  if (param_3 == 0x1) {
    (param_2 + 0x4) = 0x0;
    return;
  }
  if (param_3 == 0x2) {
    pass1_1018_2922(param_2 & 0xffff0000 | (param_2 - 0x1c));
  }
  else {
    if ((((param_3 + -0x3 < 0x1) || (SBORROW2(param_3 + -0x3,0x1))) || (0x1 < param_3 + -0x5)) ||
       (*(i32 *)(param_2 + 0x4) == 0x0)) {
      return;
    }
    uVar1 = param_2 - 0x1c;
    pass1_1018_2862((astruct_654 *)(param_2 & 0xffff0000 | uVar1));
    if ((param_1 | uVar1) != 0x0) {
      (param_2 + 0x8) = 0x1;
    }
  }
  pass1_1010_1f62((astruct_27 *)(param_2 & 0xffff0000 | (param_2 - 0x1c)),param_3);
  return;
}
pub fn pass1_1018_2922(mut param_1: u32)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (((iVar2 + 0x40) != 0x0) &&
     (piVar1 = (iVar2 + 0x3e), *piVar1 = *piVar1 + 0x1, 0x9 < (iVar2 + 0x3e))) {
    (iVar2 + 0x3e) = 0x0;
    (iVar2 + 0x42) = 0x1;
  }
  return;
}
pub fn win_op_1018_294a(mut param_1: u16 ,u16 in_string_6,param_3: *mut astruct_8,mut param_4: u16 ,mut param_5: u32)

{
  if (((param_3 + 0x18) != 0x0) && (param_5 == 0x280)) {
    (param_3 + 0x18) = 0x0;
  }
  create_dc_1018_4e04(param_1,in_string_6,param_3,param_4,param_5,param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn mixed_sys_op_1018_2978(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_931)

{
  code **ppcVar1;
  u8 *puVar2;
  astruct_394 *paVar2;
  RECT16 *rect;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut paVar7: *mut Struct57;
  astruct_931 *iVar9;
  let mut uVar8: u16;
  let mut uVar7: u16;
  let mut uVar9: u16;
  let mut HVar8: HWND16;
  astruct_394 *paVar9;
  let mut local_3a: RECT16;
  let mut iStack54: i16;
  let mut iStack52: i16;
  let mut uStack50: u32;
  let mut puStack46: *mut u32;
  astruct_394 local_2a;
  let mut uStack6: u16;
  let mut uStack4: u16;
  u8 uVar10;
  let mut paVar6: *mut Struct57;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  pass1_1010_8096(_u16_1050_14cc,0x1);
  uStack4 = SUB42(paVar5,0x0);
  puVar2 = &local_2a;
  uStack6 = param_1;
  struct_op_1008_48fe(paVar5,(astruct_81 *)CONCAT13(0x10,CONCAT12(0x50,puVar2)),0x1,CONCAT22(uStack4,param_1));
  uVar7 = 0x1000;
  mem_op_1000_179c(0x1e,paVar5);
  uVar5 = paVar5 | puVar2;
  paVar7 = (astruct_57 *)(paVar5 & 0xffff0000);
  paVar6 = (astruct_57 *)(paVar7 | uVar5);
  if (uVar5 == 0x0) {
    paVar2 = NULL;
  }
  else {
    paVar2 = &local_2a;
    uVar7 = 0x1008;
    struct_op_1008_3f92((astruct_76 *)CONCAT22(paVar5,puVar2),CONCAT22(0x1050,paVar2));
    paVar7 = paVar6;
  }
  uVar3 = SUB42(paVar7,0x0);
  puStack46 = CONCAT22(uVar3,paVar2);
  ppcVar1 = (code **)(*puStack46 + 0x14);
  paVar9 = paVar2;
  (**ppcVar1)(uVar7,paVar2,uVar3);
  uStack50 = CONCAT22(paVar7,paVar2);
  mem_op_1000_179c(0x14,paVar7);
  uVar4 = paVar7 | paVar2;
  paVar7 = (astruct_57 *)(paVar7 & 0xffff0000);
  paVar5 = (astruct_57 *)(paVar7 | uVar4);
  if (uVar4 == 0x0) {
    paVar2 = NULL;
  }
  else {
    struct_1008_4c58(paVar2);
    paVar7 = paVar5;
  }
  uVar8 = (param_3 >> 0x10);
  iVar9 = (astruct_931 *)param_3;
  *(astruct_394 **)&iVar9.field12_0xe = paVar2;
  *(u8 **)(&iVar9.field12_0xe + 0x2) = paVar7;
  pass1_1008_4d84(paVar7,iVar9.field12_0xe,uStack50);
  rect = &local_3a;
  HVar8 = HWND16_1050_0396;
  GetClientRect16(rect,(HWND16)&DAT_1050_1050);
  uVar9 = 0x1000;
  mem_op_1000_179c(0x1e,paVar7);
  uVar6 = paVar7 | rect;
  if (uVar6 == 0x0) {
    &iVar9.field10_0xa = 0x0;
  }
  else {
    iVar4 = (iStack52 - local_3a.y) + 0x1;
    uVar9 = 0x1008;
    pass1_1008_405c((astruct_76 *)CONCAT22(paVar7,rect),iVar9.field12_0xe,iVar4,
                    (iStack54 - local_3a.x) + 0x1);
    iVar9.field10_0xa = iVar4;
    iVar9.field11_0xc = uVar6;
  }
  if (puStack46 != NULL) {
    ppcVar1 = (code **)*puStack46;
    (**ppcVar1)(uVar9,puStack46,(puStack46 >> 0x10),0x1,HVar8,paVar9,uVar3,puStack46,puStack46);
  }
  close_file_1008_496c((astruct_803 *)CONCAT13(0x10,CONCAT12(0x50,&local_2a)));
  return;
}
pub fn pass1_1018_2aa3(void)

{
  pass1_1018_21f8();
  return;
}



StructD * pass1_1018_2aa8(StructD *param_1,param_2: u8)

{
  param_1 = (StructD *)(param_1 & 0xffff0000 | (param_1 - 0x1c));
  pass1_1018_2440(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1018_2afa(u32 *param_1)

{
  fn_ptr_1000_17ce(*param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1018_2b10(param_1: *mut astruct_19,mut param_2: u16 )

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut uVar7: u16;
  let mut paVar5: *mut Struct57;
  let mut uVar6: u32;
  let mut puVar8: *mut u16;
  let mut puVar9: *mut u32;
  let mut uVar10: u32;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut uVar12: u16;
  let mut in_stack_0000fff0: u32;
  let mut uVar14: u16;
  u8 **ppuVar13;
  astruct_19 *uVar11;
  astruct_19 *uVar9;

  uVar7 = (in_EDX >> 0x10);
  uVar14 = (in_stack_0000fff0 >> 0x10);
  uVar9 = (astruct_19 *)param_1;
  uVar12 = (param_1 >> 0x10);
  puVar8 = get_sys_metrics_1018_4b1e(param_1,0x1,param_2);
  paVar5 = (astruct_57 *)CONCAT22(uVar7,(puVar8 >> 0x10));
  uVar9.field17_0x20 = 0x389a;
  uVar9.field18_0x22 = 0x1008;
  uVar9.field17_0x20 = 0x3aa8;
  uVar9.field18_0x22 = 0x1008;
  &uVar9.field19_0x24 = 0x0;
  &uVar9[0x3].field_0x54 = 0x0;
  uVar9[0x3].field47_0x56 = 0x0;
  &uVar9[0x3].field_0x58 = 0x0;
  &uVar9[0x3].field_0x5a = 0x0;
  &uVar9[0x3].field53_0x5e = 0x0;
  &uVar9[0x4].segment_0x2 = 0x0;
  (&uVar9[0x4].field2_0x4 + 0x2) = 0x0;
  param_1.offset_0x0 = 0x32d8;
  uVar9.segment_0x2 = 0x1018;
  uVar9.field17_0x20 = 0x3314;
  uVar9.field18_0x22 = 0x1018;
  ppuVar13 = (u8 **)CONCAT22(uVar14,0x2f);
  puVar9 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,ppuVar13,in_stack_0000fe9a,in_stack_0000ffbe,in_stack_0000ffc4,
                           in_stack_0000ffc8);
  uVar6 = paVar5 & 0xffff0000;
  uVar9[0x4].segment_0x2 = puVar9;
  &uVar9[0x4].field2_0x4 = (puVar9 >> 0x10);
  if (param_1 == NULL) {
    puVar3 = NULL;
  }
  else {
    uVar6 |= uVar12;
    puVar3 = &uVar9.field17_0x20;
  }
  uVar1 = &uVar9[0x4].segment_0x2;
  ppcVar2 = (code **)(*(u32*)&uVar9[0x4].segment_0x2 + 0x4);
  (**ppcVar2)(0x1010,uVar1,(uVar1 >> 0x10),0x0,puVar3,uVar6,(ppuVar13 >> 0x10));
  uVar1 = &uVar9[0x4].segment_0x2;
  uVar1 = (uVar1 + 0x24);
  &uVar9[0x3].field_0x5a = uVar1;
  uVar4 = FUN_1010_830a(uVar1,uVar6,0x1010,_u16_1050_14cc,0x6e);
  uVar9.field19_0x24 = uVar4;
  &uVar9.field20_0x26 = uVar6;
  (&uVar9.field20_0x26 + 0x2) = 0x0;
  uVar10 = pass1_1008_4772(*(astruct_76 **)&uVar9.field19_0x24);
  uVar14 = (uVar10 >> 0x10);
  pass1_1018_4b78(param_1);
  &uVar9[0x3].field_0x4c = 0x1;
  &uVar9[0x3].field_0x4e = 0x1;
  &uVar9[0x3].field_0x50 = (uVar10 + 0x4) + &uVar9[0x3].field_0x4c;
  uVar9[0x3].field44_0x52 = (uVar10 + 0x8) - 0x19;
  return;
}
pub fn pass1_1018_2c60(StructD *param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut puVar4: *mut u16;
  let mut uVar5: u16;
  StructD *uVar6;
  let mut uVar7: u16;
  let mut puStack6: *mut u16;

  uVar7 = (param_1 >> 0x10);
  uVar6 = (StructD *)param_1;
  param_1.address_offset_field_0x0 = 0x32d8;
  uVar6.address_offset_field_0x2 = 0x1018;
  uVar6.field19_0x20 = 0x3314;
  uVar6.field20_0x22 = 0x1018;
  if (*(i32 *)&uVar6[0x1].field_0x94 != 0x0) {
    if (param_1 == NULL) {
      puVar4 = NULL;
      uVar5 = 0x0;
    }
    else {
      puVar4 = &uVar6.field19_0x20;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6(&uVar6[0x1].field_0x94,(StructD *)CONCAT22(uVar5,puVar4));
  }
  fn_ptr_1000_17ce(*(char **)&uVar6[0x1].field_0x98);
  puVar1 = (u32*)&uVar6.field_0x24;
  uVar2 = &uVar6.field_0x26;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1000,puVar1,uVar2,0x1);
  }
  if (param_1 == NULL) {
    puVar4 = NULL;
    uVar7 = 0x0;
  }
  else {
    puVar4 = &uVar6.field19_0x20;
  }
  puStack6 = (u16 *)CONCAT22(uVar7,puVar4);
  *puStack6 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}
pub fn pass1_1018_2d22(mut param_1: u32,i16 *param_2,param_3: *mut u16,mut param_4: i16)

{
  let mut uVar1: u32;

  *param_3 = 0x0;
  *param_2 = 0x0;
  uVar1 = pass1_1008_4772(*(astruct_76 **)(param_1 + 0x24));
  *param_2 = (uVar1 + 0x8) + -0x14;
  if (param_4 == 0xbb8) {
    *param_3 = 0x5;
  }
  if (param_4 == 0xbba) {
    *param_3 = 0x23;
  }
  if (param_4 == 0xbb9) {
    *param_3 = 0x75;
  }
  return;
}
pub fn pass1_1018_2d84(mut param_1: u16 ,param_2: *mut astruct_126)

{
  pass1_1018_2e28(param_2);
  pass1_1020_bd80(param_1);
  return;
}
pub fn pass1_1018_2d9a(param_1: *mut astruct_126)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  astruct_126 *iVar4;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_126 *)param_1;
  uVar3 = iVar4.field375_0x180 | iVar4.field374_0x17e;
  if (uVar3 != 0x0) {
    piVar1 = &iVar4.field369_0x174;
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
      uVar2 = &iVar4.field374_0x17e;
      uVar3 = (uVar2 + 0xa) - 0x1;
      iVar4.field369_0x174 = uVar3;
    }
    pass1_1018_2e28(param_1);
    iVar4.field370_0x176 = uVar3;
  }
  return;
}
pub fn pass1_1018_2dde(param_1: *mut astruct_126)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  astruct_126 *iVar4;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_126 *)param_1;
  if ((iVar4.field375_0x180 | iVar4.field374_0x17e) != 0x0) {
    piVar1 = &iVar4.field369_0x174;
    *piVar1 = *piVar1 + 0x1;
    iVar3 = iVar4.field369_0x174;
    uVar2 = &iVar4.field374_0x17e;
    piVar1 = (uVar2 + 0xa);
    if (*piVar1 == iVar3 || *piVar1 < iVar3) {
      iVar4.field369_0x174 = 0x0;
    }
    pass1_1018_2e28(param_1);
    iVar4.field370_0x176 = iVar3;
  }
  return;
}
pub fn pass1_1018_2e28(param_1: *mut astruct_126)

{
  let mut uVar1: u16;
  let mut extraout_DX: u16;

  uVar1 = (param_1 + 0x174);
  empty_1008_8fc4();
  if ((extraout_DX | uVar1) != 0x0) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1018_2e5e(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_126)

{
  let mut uVar3: u32;
  let mut uVar1: u16;
  let mut iVar5: i16;
  astruct_126 *iVar4;
  astruct_126 *uVar2;

  uVar2 = (astruct_126 *)(param_3 >> 0x10);
  iVar4 = (astruct_126 *)param_3;
  if (*(i32 *)&iVar4.field374_0x17e == 0x0) {
    pass1_1030_82f0(_u16_1050_5748,iVar4.field373_0x17a);
    iVar4.field374_0x17e = param_1;
    iVar4.field375_0x180 = param_2;
  }
  if ((*(i32 *)&iVar4.field374_0x17e != 0x0) &&
     (uVar3 = &iVar4.field374_0x17e, (uVar3 + 0xa) != 0x0)) {
    iVar5 = iVar4.field369_0x174;
    empty_1008_8fc4();
    pass1_1018_2e28(param_3);
    iVar4.field370_0x176 = iVar5;
    return;
  }
  return;
}
pub fn pass1_1018_2ee4(param_1: *mut astruct_126,mut param_2: u16 )

{
  let mut uVar1: u32;
  char cVar2;
  let mut uVar3: u16;

  if (param_2 != 0x12) {
    if (param_2 < 0x13) {
      cVar2 = (char)param_2;
      if (cVar2 == '\x01') {
        (param_1 + 0x162) = 0x0;
        return;
      }
      if (('\x02' < (char)(cVar2 + -0x1)) && ((char)(cVar2 + -0x4) < '\x03')) goto LAB_1018_2f06;
    }
    return;
  }
  uVar1 = (param_1 + 0x162);
  (param_1 + 0x15a) = (uVar1 + 0x24);//
LAB_1018_2f06:
  uVar3 = param_1 - 0x20;
  pass1_1018_31fa(uVar3,param_1,(astruct_126 *)(param_1 & 0xffff0000 | uVar3));
  pass1_1010_1f62((astruct_27 *)(param_1 & 0xffff0000 | uVar3),param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn get_sys_metrics_1018_2f56(mut param_1: u32)

{
  INT16 IVar1;
  INT16 IVar2;
  let mut in_EDX: *mut Struct57;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut puVar5: *mut u32;
  let mut uVar6: u32;
  let mut in_stack_0000fe7c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut in_stack_0000ffa6: u16;
  let mut in_stack_0000ffaa: u16;
  let mut piVar7: *mut i16;
  let mut uVar8: u16;
  let mut pcVar9: *mut c_char;
  let mut local_6: i16;
  let mut local_4: i16;

  pcVar9 = CONCAT22(0x1050,&local_4);
  piVar7 = &local_6;
  uVar8 = SUB42(&DAT_1050_1050,0x0);
  puVar5 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(piVar7,0x48),in_stack_0000fe7c,in_stack_0000ffa0
                           ,in_stack_0000ffa6,in_stack_0000ffaa);
  pass1_1008_3e94((u16 *)(puVar5 & 0xffff0000 | (puVar5 + 0xe)),(u16 *)CONCAT22(uVar8,piVar7),pcVar9)
  ;
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar6 = pass1_1008_4772(*(astruct_76 **)(iVar3 + 0x24));
  uVar8 = (uVar6 >> 0x10);
  (iVar3 + 0x18) = local_4 + 0xb5;
  (iVar3 + 0x1a) = local_6 + 0x9;
  IVar1 = GetSystemMetrics16(SM_CXBORDER);
  (iVar3 + 0x1c) = IVar1 * 0x2 + (uVar6 + 0x4);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  IVar2 = GetSystemMetrics16(SM_CYBORDER);
  (iVar3 + 0x1e) = IVar2 + IVar1 + (uVar6 + 0x8);
  return;
}
pub fn pass1_1018_2fe8(param_1: *mut astruct_126,u16_t param_2,u16_t param_3)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut extraout_DX: u16;
  let mut uVar8: u16;
  astruct_126 *pstruct126_9;
  astruct_126 *uVar9;

  uVar9 = (astruct_126 *)(param_1 >> 0x10);
  pstruct126_9 = (astruct_126 *)param_1;
  iVar6 = pstruct126_9.field369_0x174;
  uVar2 = &pstruct126_9.field374_0x17e;
  iVar7 = (uVar2 + 0xa);
  if (iVar7 != 0x0) {
    if (pstruct126_9[0x1].field4_0x4 != NULL) {
      uVar3 = str_op_1000_3da4(pstruct126_9[0x1].field4_0x4);
      pstruct126_9.field369_0x174 = 0x0;
      while( true ) {
        if (iVar7 <= pstruct126_9.field369_0x174) break;
        uVar4 = pstruct126_9.field369_0x174;
        empty_1008_8fc4();
        uVar8 = extraout_DX;
        pass1_1018_2e28(param_1);
        uVar4 = pass1_1020_bd80(uVar4);
        uVar5 = pass1_1000_3de8(CONCAT22(uVar8,uVar4),pstruct126_9[0x1].field4_0x4,uVar3,param_2,param_3);
        if (uVar5 == 0x0) break;
        piVar1 = &pstruct126_9.field369_0x174;
        *piVar1 = *piVar1 + 0x1;
      }
      if (pstruct126_9.field369_0x174 < iVar7) {
        pass1_1018_2e28(param_1);
        pstruct126_9.field370_0x176 = iVar7;
        return;
      }
      pstruct126_9.field369_0x174 = iVar6;
      pass1_1018_2e28(param_1);
      pstruct126_9.field370_0x176 = iVar6;
    }
  }
  return;
}
