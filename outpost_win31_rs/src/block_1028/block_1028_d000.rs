
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_d078(mut param_1: u32,mut param_2: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut in_EDX: u32;
  let mut paVar2: *mut Struct57;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut uVar7: u32;
  let mut local_16: [u8;0x4] = [0;0x4];
  let mut puStack18: *mut u32;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut puStack6: *mut u32;
  let mut uStack4: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puStack6 = (iVar4 + 0x4);
  uStack16 = (iVar4 + 0x6);
  paVar2 = (in_EDX & 0xffff0000 | uStack16);
  uStack14 = CONCAT22(uStack16,puStack6);
  puStack18 = puStack6;
  if ((uStack16 | puStack6) != 0) {
    ppcVar1 = *puStack6;
    (**ppcVar1)();
  }
  mem_op_1000_179c(0x1c,paVar2);
  uStack16 = paVar2;
  uVar7 = paVar2 & 0xffff0000;
  uVar3 = uVar7 | (uStack16 | puStack6);
  puStack18 = puStack6;
  if ((uStack16 | puStack6) == 0) {
    puStack6 = NULL;
  }
  else {
    struct_op_1008_8e9e(CONCAT22(uStack16,puStack6),0x6,0x24);
    uVar7 = uVar3;
  }
  (iVar4 + 0x4) = puStack6;
  (iVar4 + 0x6) = uVar7;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_2);
  uStack4 = uVar7;
  uVar7 &= 0xffff0000;
  if ((uStack4 | puStack6).is_null()) {
    puVar6 = pass1_1018_dcf6(CONCAT22(0x1050,local_16));
    uVar7 = pass1_1018_dd1e(local_16,(uVar7 & 0xffff0000 | puVar6 >> 0x10),local_16,
                            &DAT_1050_1050,0x0,0xa0000);
    pass1_1008_8faa(*(astruct_78 **)(iVar4 + 0x4),uVar7);
    return;
  }
  uVar7 = pass1_1038_565e((uStack4 | puStack6),CONCAT22(uStack4,puStack6));
  uStack8 = (uVar7 >> 0x10);
  uStack10 = uVar7;
  if ((uStack8 | uStack10) != 0) {
    pass1_1028_d172(param_1,uVar7 & 0xffff | uStack8 << 0x10);
  }
  return;
}
pub fn pass1_1028_d172(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  let mut uVar3: u32;
  let mut puVar6: *mut u16;
  let mut uVar7: u32;
  let mut local_e: [u8;0x8] = [0;0x8];
  let mut local_6: [u8;0x4] = [0;0x4];
  let mut paVar4: *mut Struct57;

  uVar5 = (in_EDX >> 0x10);
  puVar6 = pass1_1018_dcf6(CONCAT22(0x1050,local_6));
  uVar3 = CONCAT22(uVar5,(puVar6 >> 0x10));
  pass1_1008_5784(CONCAT22(0x1050,local_e),param_2);
  while( true ) {
    uVar1 = pass1_1008_5b12(CONCAT22(0x1050,local_e));
    uVar2 = uVar3 | uVar1;
    paVar4 = (uVar3 & 0xffff0000 | uVar2);
    if (uVar2 == 0) break;
    uVar7 = pass1_1018_dd1e(local_6,paVar4,local_6,&DAT_1050_1050,0x0,
                            (uVar1 + 0x4) << 0x10);
    uVar3 = paVar4 & 0xffff0000 | uVar7 >> 0x10;
    pass1_1008_8faa(*(astruct_78 **)(param_1 + 0x4),uVar7);
  }
  return;
}



astruct_97 * struct_op_1028_d1dc(param_1: *mut astruct_97,mut param_2: u16 )

{
  iVar1: *mut astruct_97;
  let mut uVar1: u16;
  let mut in_stack_0000fffa: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.offset_0x0 = 0x389a;
  iVar1.segment_0x2 = 0x1008;
  &iVar1.field_0x4 = param_2;
  &iVar1.field_0x6 = 0;
  param_1.offset_0x0 = 0x6ad2;
  iVar1.segment_0x2 = 0x1028;
  sys_1000_3f9c((param_1 & 0xffff0000 | ZEXT24(&iVar1.string_0x8)),s_ctor_1050_5160,in_stack_0000fffa);
  return param_1;
}



pub unsafe fn FUN_1028_d222() -> u16

{
  return 0x1;
}



pub unsafe fn FUN_1028_d228() -> u16

{
  return 0x1;
}
pub fn struct_1028_d22e(param_1: *mut astruct_57,param_2: *mut u32,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut pSVar2: *mut StructD;
  let mut uVar3: u16;

  uVar3 = (param_2 >> 0x10);
  *param_2 = 0;
  (param_2 + 0x4) = param_3;
  mem_op_1000_179c(0xc,param_1);
  uVar1 = param_3;
  pSVar2 = (param_1 | uVar1);
  if (pSVar2.is_null()) {
    *param_2 = 0;
  }
  else {
    struct_1028_d59c(pSVar2,(param_3 & 0xffff | param_1 << 0x10));
    param_2 = uVar1;
    *(StructD **)(param_2 + 0x2) = pSVar2;
  }
  return;
}
pub fn pass1_1028_d282(param_1: *mut astruct_446)

{
  let mut pcStack6: *mut c_char;
  uVar1: *mut astruct_446;
  uVar2: *mut astruct_446;

  uVar1 = *(astruct_446 **)param_1;
  uVar2 = *(astruct_446 **)(param_1 + 2);
  pcStack6 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0) {
    pass1_1028_d658(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack6);
  }
  return;
}
pub fn struct_1028_d2b0(param_1: u32)

{
  let mut local_10c: u16;
  let mut uStack266: u16;

  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x3e80);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x3a98);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x36b0);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x32c8);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x2ee0);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x2af8);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x2710);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
    // just 0x2328
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),s_noth_bmp_1050_2321 + 0x7);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x1f40);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x1b58);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x1770);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x1388);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0xfa0);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0xbb8);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62(CONCAT22(0x1050,&local_10c),0x3e8);
  fn_ptr_1028_d566(param_1,CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  pass1_1028_d6b2(*param_1);
  return;
}



pub fn pass1_1028_d52c(param_1: u32,mut param_2: u32,param_3: *mut u32) -> BOOL16

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut BVar3: bool;

  ppcVar1 = (*param_3 + 0x8);
  iVar2 = (**ppcVar1)();
  if (iVar2 != 0) {
    BVar3 = pass1_1028_d776(*param_1,param_2,param_3);
    if (BVar3 != 0) {
      return 0x1;
    }
  }
  return 0x0;
}



pub fn fn_ptr_1028_d566(param_1: u32,param_2: *mut astruct_97) -> BOOL16

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut uVar3: u16;

  ppcVar1 = (param_2 + 0x8);
  iVar2 = (**ppcVar1)();
  if (iVar2 != 0) {
    uVar3 = fn_ptr_1028_d742(*param_1,param_2);
    if (uVar3 != 0) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1028_d59c(param_1: *mut StructD,param_2: *mut astruct_158)

{
  let mut puVar1: *mut u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  iVar5: *mut astruct_158;
  uVar5: *mut astruct_158;
  let mut puStack14: *mut u16;
  let mut paVar6: *mut Struct57;

  paVar5 = CONCAT22(in_register_0000000a,param_1);
  uVar5 = (param_2 >> 0x10);
  iVar5 = param_2;
  param_2 = 0;
  iVar5.field3_0x4 = NULL;
  iVar5.field4_0x8 = NULL;
  puVar2 = *_u16_1050_5748;
  (u16*)param_2 = puVar2;
  mem_op_1000_179c(0xc,paVar5);
  puVar1 = (puVar2 & 0xffff | paVar5 << 0x10);
  uVar3 = paVar5 | puVar2;
  paVar6 = (paVar5 & 0xffff0000 | uVar3);
  if (uVar3 == 0) {
    iVar5.field3_0x4 = NULL;
  }
  else {
    set_struct_1008_574a((puVar2 & 0xffff | paVar5 << 0x10));
    *puVar1 = 0xd804;
    (puVar2 + 0x2) = 0x1028;
    iVar5.field3_0x4 = puVar1;
    puVar2 = puVar1;
  }
  uVar3 = puVar2;
  mem_op_1000_179c(0xc,paVar6);
  uVar4 = paVar6;
  puStack14 = CONCAT22(uVar4,uVar3);
  if ((uVar4 | uVar3) == 0) {
    iVar5.field4_0x8 = NULL;
  }
  else {
    set_struct_1008_574a(CONCAT22(uVar4,uVar3));
    *puStack14 = 0xd804;
    (uVar3 + 0x2) = 0x1028;
    iVar5.field4_0x8 = puStack14;
  }
  return;
}
pub fn pass1_1028_d658(param_1: *mut astruct_446)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  iVar4: *mut astruct_446;
  uVar4: *mut astruct_446;

  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = iVar4.field4_0x4;
  uVar2 = iVar4.field5_0x6;
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4.field6_0x8;
  uVar2 = iVar4.field7_0xa;
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  return;
}



pub unsafe fn pass1_1028_d69e(mut param_1: u32) -> u16

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0x4);
  return (uVar1 + 0x8);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_d6b2(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u32;
  let mut ppcVar3: *mut *mut code;
  let mut puVar4: *mut u32;
  let mut uVar5: u16;
  let mut extraout_DX: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;

  uVar2 = *_PTR_LOOP_1050_65e2;
  while( true ) {
    uVar6 = (param_1 >> 0x10);
    uVar7 = pass1_1020_c860((param_1 + 0x8));
    uVar5 = (uVar7 >> 0x10);
    if (((uVar5 | uVar7) == 0) || (puVar1 = (uVar7 + 0xc), uVar2 <= *puVar1 && *puVar1 != uVar2))
    break;
    ppcVar3 = ((param_1 + 0x8) + 0x10);
    uVar7 = uVar2;
    (**ppcVar3)();
    puVar4 = (uVar7 & 0xffff | extraout_DX << 0x10);
    fn_ptr_1028_d742(param_1,(uVar7 & 0xffff | extraout_DX << 0x10));
    if (puVar4.is_null() == false) {
      ppcVar3 = *puVar4;
      (**ppcVar3)(0x1020,uVar7,extraout_DX,1);
    }
  }
  return;
}
pub fn fn_ptr_1028_d728(mut param_1: u32)

{
  let mut ppcVar1: *mut *mut code;

  ppcVar1 = ((param_1 + 0x4) + 0x10);
  (**ppcVar1)();
  return;
}



pub unsafe fn fn_ptr_1028_d742(mut param_1: u32,param_2: *mut u32) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;

  ppcVar1 = (*param_2 + 0xc);
  uVar2 = (**ppcVar1)();
  pass1_1020_c872((param_1 + 0x4),(uVar2 + 0x4),uVar2);
  return 0x1;
}



pub fn pass1_1028_d776(mut param_1: u32,mut param_2: u32,param_3: *mut u32) -> BOOL16

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;

  ppcVar1 = (*param_3 + 0xc);
  uVar2 = (**ppcVar1)();
  pass1_1020_c872((param_1 + 0x8),param_2,uVar2);
  return 0x1;
}



pub fn pass1_1028_d7a0(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32) -> BOOL16

{
  let mut BVar1: bool;

  BVar1 = write_to_file_1008_7cac(param_3);
  if (BVar1 != 0) {
    BVar1 = 0x1;
  }
  return BVar1;
}



pub unsafe fn read_file_1028_d7ba(param_1: u16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32) -> i16

{
  read_file_1008_7cfe(param_4,(param_4 >> 0x10),0x8);
  if (param_1 == 0) {
    u16_1050_0310 = 0x6d4;
    return param_1;
  }
  return 0x1;
}



pub fn pass1_1028_d7de(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1008_57c4(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_d81c(param_1: *mut astruct_57,param_2: *mut astruct_136,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut pSVar3: *mut StructD;
  let mut uVar4: u16;
  let mut paVar6: *mut Struct57;
  let mut paVar8: *mut Struct57;
  iVar6: *mut astruct_136;
  let mut uVar9: u16;
  let mut in_stack_0000fe88: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffde: u16;
  let mut pSVar5: *mut StructD;
  let mut paVar7: *mut Struct57;

  uVar9 = (param_2 >> 0x10);
  iVar6 = param_2;
  param_2.field0_0x0 = 0;
  iVar6.field1_0x4 = param_3;
  &iVar6.field44_0x52 = 0;
  _PTR_LOOP_1050_65e2 = param_2;
  iVar6.field28_0x32 = 0xec36;
  iVar6.field29_0x34 = 0x1028;
  iVar6.field30_0x36 = 0xecac;
  iVar6.field31_0x38 = 0x1028;
  iVar6.field32_0x3a = 0xed2c;
  iVar6.field33_0x3c = 0x1028;
  iVar6.field34_0x3e = 0xedc4;
  iVar6.field35_0x40 = 0x1028;
  iVar6.field36_0x42 = 0xee54;
  iVar6.field37_0x44 = 0x1028;
  iVar6.field38_0x46 = 0xef00;
  iVar6.field39_0x48 = 0x1028;
  iVar6.field40_0x4a = 0x10b0;
  iVar6.field41_0x4c = 0x1030;
  iVar6.field42_0x4e = 0x1120;
  iVar6.field43_0x50 = 0x1030;
  mem_op_1000_179c(0x8,param_1);
  uVar4 = param_3;
  uVar2 = param_1 | uVar4;
  pSVar5 = (param_1 & 0xffff0000 | uVar2);
  if (uVar2 != 0) {
    pass1_1030_615a(pSVar5,(param_3 & 0xffff | param_1 << 0x10));
  }
  mem_op_1000_179c(0x56c,pSVar5);
  uVar2 = pSVar5 | uVar4;
  paVar7 = (pSVar5 & 0xffff0000);
  paVar6 = (paVar7 | uVar2);
  if (uVar2 == 0) {
    uVar4 = 0;
  }
  else {
    struct_1030_44be(paVar6,CONCAT22(pSVar5,uVar4),in_stack_0000fe88,in_stack_0000ffac,
                     in_stack_0000ffb2,in_stack_0000ffb6,in_stack_0000ffde);
    paVar7 = paVar6;
  }
  iVar6.field44_0x52 = uVar4;
  iVar6.field45_0x54 = paVar7;
  mem_op_1000_179c(0x4,paVar7);
  pSVar3 = (paVar7 | uVar4);
  paVar6 = (paVar7 & 0xffff0000 | ZEXT24(pSVar3));
  if (pSVar3.is_null() == false) {
    struct_1008_bde0(pSVar3,CONCAT22(paVar7,uVar4));
  }
  puVar1 = pass1_1000_4906((param_2 & 0xffff0000 | ZEXT24(&iVar6.field_0xa)),NULL,0x24);
  mem_op_1000_179c(0x1c,paVar6);
  uVar4 = paVar6 | puVar1;
  paVar7 = (paVar6 & 0xffff0000 | uVar4);
  if (uVar4 == 0) {
    &iVar6.field8_0xe = 0;
  }
  else {
    struct_1030_11aa(CONCAT22(paVar6,puVar1),0x5,0x15);
    iVar6.field8_0xe = puVar1;
    iVar6.field9_0x10 = paVar7;
  }
  mem_op_1000_179c(0x1c,paVar7);
  uVar4 = paVar7 | puVar1;
  paVar6 = (paVar7 & 0xffff0000);
  paVar8 = (paVar6 | uVar4);
  if (uVar4 == 0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa(CONCAT22(paVar7,puVar1),0x5,0xa);
    paVar6 = paVar8;
  }
  iVar6.field10_0x12 = puVar1;
  iVar6.field11_0x14 = paVar6;
  mem_op_1000_179c(0x1c,paVar6);
  uVar4 = paVar6 | puVar1;
  paVar7 = (paVar6 & 0xffff0000);
  paVar8 = (paVar7 | uVar4);
  if (uVar4 == 0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa(CONCAT22(paVar6,puVar1),0x5,0x19);
    paVar7 = paVar8;
  }
  iVar6.field12_0x16 = puVar1;
  iVar6.field13_0x18 = paVar7;
  mem_op_1000_179c(0x1c,paVar7);
  uVar4 = paVar7 | puVar1;
  paVar6 = (paVar7 & 0xffff0000);
  paVar8 = (paVar6 | uVar4);
  if (uVar4 == 0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa(CONCAT22(paVar7,puVar1),0x5,0xa);
    paVar6 = paVar8;
  }
  iVar6.field14_0x1a = puVar1;
  iVar6.field15_0x1c = paVar6;
  mem_op_1000_179c(0x1c,paVar6);
  uVar4 = paVar6 | puVar1;
  paVar7 = (paVar6 & 0xffff0000);
  paVar8 = (paVar7 | uVar4);
  if (uVar4 == 0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa(CONCAT22(paVar6,puVar1),0x64,0x1f4);
    paVar7 = paVar8;
  }
  iVar6.field16_0x1e = puVar1;
  iVar6.field17_0x20 = paVar7;
  mem_op_1000_179c(0x1c,paVar7);
  uVar4 = paVar7 | puVar1;
  paVar6 = (paVar7 & 0xffff0000);
  paVar8 = (paVar6 | uVar4);
  if (uVar4 == 0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa(CONCAT22(paVar7,puVar1),0x19,0x64);
    paVar6 = paVar8;
  }
  iVar6.field18_0x22 = puVar1;
  iVar6.field19_0x24 = paVar6;
  mem_op_1000_179c(0x1c,paVar6);
  uVar4 = paVar6 | puVar1;
  paVar7 = (paVar6 & 0xffff0000);
  paVar8 = (paVar7 | uVar4);
  if (uVar4 == 0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa(CONCAT22(paVar6,puVar1),0x64,0x1f4);
    paVar7 = paVar8;
  }
  iVar6.field20_0x26 = puVar1;
  iVar6.field21_0x28 = paVar7;
  mem_op_1000_179c(0x1c,paVar7);
  uVar4 = paVar7 | puVar1;
  if (uVar4 == 0) {
    puVar1 = NULL;
    uVar4 = 0;
  }
  else {
    struct_1030_11aa(CONCAT22(paVar7,puVar1),0x5,0x2);
  }
  iVar6.field22_0x2a = puVar1;
  iVar6.field23_0x2c = uVar4;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_daba(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut ppcVar4: *mut *mut code;
  let mut puVar5: *mut u32;
  let mut puVar6: *mut u16;
  iVar5: *mut astruct_447;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  let mut pcStack14: *mut c_char;

  puVar6 = _PTR_LOOP_1050_5740;
  if (_PTR_LOOP_1050_5740.is_null() == false) {
    pass1_1030_61b0(_PTR_LOOP_1050_5740);
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(puVar6);
  }
  uVar7 = (param_1 >> 0x10);
  iVar5 = param_1;
  uVar1 = iVar5.field66_0x52;
  uVar2 = iVar5.field67_0x54;
  pcStack14 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0) {
    pass1_1030_4538(CONCAT22(uVar2,uVar1));
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcStack14);
  }
  if (_PTR_LOOP_1050_5166.is_null() == false) {
    ppcVar4 = *_PTR_LOOP_1050_5166;
    (**ppcVar4)(unaff_CS,_PTR_LOOP_1050_5166);
  }
  puVar5 = _u16_1050_06e0;
  _PTR_LOOP_1050_65e2 = 0;
  if (_u16_1050_06e0.is_null() == false) {
    pass1_1008_c626(_u16_1050_06e0);
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(puVar5);
  }
  puVar3 = iVar5.field14_0xe;
  uVar1 = iVar5.field15_0x10;
  if ((uVar1 | puVar3) != 0) {
    ppcVar4 = *puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,1);
  }
  puVar3 = iVar5.field16_0x12;
  uVar1 = iVar5.field17_0x14;
  if ((uVar1 | puVar3) != 0) {
    ppcVar4 = *puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,1);
  }
  puVar3 = iVar5.field18_0x16;
  uVar1 = iVar5.field19_0x18;
  if ((uVar1 | puVar3) != 0) {
    ppcVar4 = *puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,1);
  }
  puVar3 = iVar5.field20_0x1a;
  uVar1 = iVar5.field21_0x1c;
  if ((uVar1 | puVar3) != 0) {
    ppcVar4 = *puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,1);
  }
  puVar3 = iVar5.field22_0x1e;
  uVar1 = iVar5.field23_0x20;
  if ((uVar1 | puVar3) != 0) {
    ppcVar4 = *puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,1);
  }
  puVar3 = iVar5.field24_0x22;
  uVar1 = iVar5.field25_0x24;
  if ((uVar1 | puVar3) != 0) {
    ppcVar4 = *puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,1);
  }
  puVar3 = iVar5.field26_0x26;
  uVar1 = iVar5.field27_0x28;
  if ((uVar1 | puVar3) != 0) {
    ppcVar4 = *puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,1);
  }
  puVar3 = iVar5.field28_0x2a;
  uVar1 = iVar5.field29_0x2c;
  if ((uVar1 | puVar3) != 0) {
    ppcVar4 = *puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_dc52(param_1: *mut astruct_92,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 )

{
  let mut uVar1: u32;
  iVar2: *mut astruct_92;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  param_1 = 0x389a;
  iVar2.field2_0x2 = 0x1008;
  iVar2.field3_0x4 = (_PTR_LOOP_1050_65e2 + (param_4 >> 0x8) * 0x4 + 0xa);
  iVar2.field4_0x8 = 0x1;
  iVar2.field6_0x10 = param_2;
  param_1 = 0x11a6;
  iVar2.field2_0x2 = 0x1030;
  uVar1 = iVar2.field3_0x4;
  iVar2.field5_0xc = (uVar1 + 0xa);
  if (param_2 == 0) {
    iVar2.field4_0x8 = iVar2.field5_0xc;
  }
  else {
    iVar2.field4_0x8 = 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn write_to_file_1028_dce2(param_1: u16,param_2: *mut u32,param_3: *mut u8) -> u32

{
  let mut ppcVar1: *mut *mut code;
  let mut BVar2: bool;
  paVar3: *mut astruct_92;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  in_stack_0000ffc0: HFILE16;
  let mut local_26: [u32;0x2] = [0;0x2];
  let mut local_1e: [u16;0x3] = [0;0x3];
  let mut uStack24: u32;
  astruct_92 local_14;

  BVar2 = write_to_file_1008_7cac(param_3);
  if (BVar2 != 0) {
    local_26[0] = *param_2;
    BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_26),0x4,in_stack_0000ffc0);
    if (BVar2 != 0) {
      uVar6 = (param_2 >> 0x10);
      iVar5 = param_2;
      local_1e[0] = (iVar5 + 0x8);
      BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_1e),0x2,in_stack_0000ffc0);
      if (BVar2 != 0) {
        ppcVar1 = (*_PTR_LOOP_1050_5166 + 0xc);
        (**ppcVar1)(0x1008,_PTR_LOOP_1050_5166,(_PTR_LOOP_1050_5166 >> 0x10),param_3);
        param_1 = extraout_DX;
        if (BVar2 != 0) {
          BVar2 = write_to_file_1008_7cac(param_3);
          if (BVar2 != 0) {
            param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x1000000);
            if (BVar2 != 0) {
              BVar2 = write_to_file_1008_7cac(param_3);
              if (BVar2 != 0) {
                param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x2000000);
                if (BVar2 != 0) {
                  BVar2 = write_to_file_1008_7cac(param_3);
                  if (BVar2 != 0) {
                    param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x3000000);
                    if (BVar2 != 0) {
                      BVar2 = write_to_file_1008_7cac(param_3);
                      if (BVar2 != 0) {
                        param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x4000000);
                        if (BVar2 != 0) {
                          BVar2 = write_to_file_1008_7cac(param_3);
                          if (BVar2 != 0) {
                            param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x5000000);
                            if (BVar2 != 0) {
                              BVar2 = write_to_file_1008_7cac(param_3);
                              if (BVar2 != 0) {
                                param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x6000000);
                                if (BVar2 != 0) {
                                  BVar2 = write_to_file_1008_7cac(param_3);
                                  if (BVar2 != 0) {
                                    param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x7000000);
                                    if (BVar2 != 0) {
                                      BVar2 = write_to_file_1008_7cac(param_3);
                                      if (BVar2 != 0) {
                                        param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x8000000);
                                        if (BVar2 != 0) {
                                          pass1_1028_dc52(CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
                                          while( true ) {
                                            uVar4 = param_1;
                                            paVar3 = &local_14;
                                            pass1_1028_e4ec(CONCAT22(0x1050,paVar3));
                                            uStack24 = CONCAT22(uVar4,paVar3);
                                            param_1 = uVar4 | paVar3;
                                            if (param_1 == 0) break;
                                            if (paVar3[0x1c].field4_0x8 != 0x8000002) {
                                              pass1_1038_3ba0(CONCAT22(uVar4,paVar3));
                                            }
                                          }
                                          return 0x10000;
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
          }
        }
      }
    }
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn read_file_1028_def2(param_1: u16,param_2: *mut u8,param_3: *mut HFILE16)

{
  let mut ppcVar1: *mut *mut code;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;

  uVar3 = param_3;
  uVar4 = (param_3 >> 0x10);
  read_file_1008_7cfe(uVar3,uVar4,0xa);
  if (param_1 != 0) {
    BVar2 = read_file_1008_7dee(param_3,param_2,0x4);
    if (BVar2 != 0) {
      BVar2 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | (param_2 + 0x8)),0x2);
      if (BVar2 != 0) {
        ppcVar1 = (*_PTR_LOOP_1050_5166 + 0x10);
        (**ppcVar1)(0x1008,_PTR_LOOP_1050_5166,(_PTR_LOOP_1050_5166 >> 0x10),param_3);
        if (BVar2 != 0) {
          read_file_1008_7cfe(uVar3,uVar4,0xc);
          if (BVar2 != 0) {
            pass1_1028_e628(param_2,uVar3,uVar4,0x0,0x100);
            if (BVar2 != 0) {
              read_file_1008_7cfe(uVar3,uVar4,0xd);
              if (BVar2 != 0) {
                pass1_1028_e628(param_2,uVar3,uVar4,0x0,0x200);
                if (BVar2 != 0) {
                  read_file_1008_7cfe(uVar3,uVar4,0xe);
                  if (BVar2 != 0) {
                    pass1_1028_e628(param_2,uVar3,uVar4,0x0,0x300);
                    if (BVar2 != 0) {
                      read_file_1008_7cfe(uVar3,uVar4,0xf);
                      if (BVar2 != 0) {
                        pass1_1028_e628(param_2,uVar3,uVar4,0x0,0x400);
                        if (BVar2 != 0) {
                          read_file_1008_7cfe(uVar3,uVar4,0x10);
                          if (BVar2 != 0) {
                            pass1_1028_e628(param_2,uVar3,uVar4,0x0,0x500);
                            if (BVar2 != 0) {
                              read_file_1008_7cfe(uVar3,uVar4,0x11);
                              if (BVar2 != 0) {
                                pass1_1028_e628(param_2,uVar3,uVar4,0x0,0x600);
                                if (BVar2 != 0) {
                                  read_file_1008_7cfe(uVar3,uVar4,0x12);
                                  if (BVar2 != 0) {
                                    pass1_1028_e628(param_2,uVar3,uVar4,0x0,0x700);
                                    if (BVar2 != 0) {
                                      read_file_1008_7cfe(uVar3,uVar4,0x13);
                                      if (BVar2 != 0) {
                                        pass1_1028_e628(param_2,uVar3,uVar4,0x0,0x800);
                                        if (BVar2 != 0) {
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
          }
        }
      }
    }
  }
  return;
}
