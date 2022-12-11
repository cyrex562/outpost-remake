
pub unsafe fn pass1_1030_8030(mut param_1: u16 ,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut paVar2: *mut astruct_15;
  let mut uVar3: u32;
  let mut in_AX: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;

  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  if ((iVar4 + 0x1a) == 0) {
    uVar6 = struct_op_1030_73a8((param_2 & 0xffff | uVar5 << 0x10),in_AX,param_1);
    param_1 = (uVar6 >> 0x10);
  }
  uVar3 = (iVar4 + 0x1a);
  iVar1 = (uVar3 + 0xc);
  if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
     ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
    paVar2 = *(astruct_15 **)(iVar4 + 0x1a);
    pass1_1028_1106(paVar2,param_1,paVar2);
  }
  return;
}



pub unsafe fn pass1_1030_8086(mut param_1: u32) -> u32

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x18),(param_1 + 0x16)) & 0xffffff;
}
pub unsafe fn pass1_1030_809c(mut param_1: u32)

{
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x1a) == 0) {
    struct_op_1030_73a8((param_1 & 0xffff | uVar1 << 0x10),in_AX,in_DX);
  }
  return;
}



astruct_611 * pass1_1030_80ee(param_1: *mut astruct_611,param_2: u8)

{
  pass1_1030_68dc(param_1);
  if ((param_2 & 1) != 0) {
    pass1_1000_093a(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn struct_1030_8128(param_1: *mut astruct_57,param_2: *mut astruct_135)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut pSVar3: *mut StructD;
  let mut paVar5: *mut Struct57;
  let mut iVar4: *mut astruct_135;
  let mut uVar5: u16;
  let mut paVar4: *mut Struct57;

  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  uVar1 = 0;
  param_2 = 0;
  iVar4.field2_0x4 = 0;
  iVar4.field4_0x8 = 0;
  _u16_1050_5748 = param_2;
  mem_op_1000_179c(0x56,param_1);
  uVar2 = param_1 | uVar1;
  paVar4 = (param_1 & 0xffff0000 | uVar2);
  if (uVar2 != 0) {
    pass1_1028_d81c(paVar4,CONCAT22(param_1,uVar1),param_2);
  }
  mem_op_1000_179c(0x8,paVar4);
  uVar2 = paVar4 | uVar1;
  paVar5 = (paVar4 & 0xffff0000 | uVar2);
  if (uVar2 == 0) {
    param_2 = 0;
  }
  else {
    struct_1028_d22e(paVar5,CONCAT22(paVar4,uVar1),param_2);
    param_2.field0_0x0 = uVar1;
    iVar4.field1_0x2 = paVar5;
  }
  mem_op_1000_179c(0x8,paVar5);
  uVar2 = paVar5 | uVar1;
  paVar4 = (paVar5 & 0xffff0000 | uVar2);
  if (uVar2 == 0) {
    iVar4.field2_0x4 = 0;
  }
  else {
    pass1_1028_cfd2(CONCAT22(paVar5,uVar1),param_2);
    iVar4.field2_0x4 = uVar1;
    iVar4.field3_0x6 = paVar4;
  }
  mem_op_1000_179c(0x24,paVar4);
  uVar2 = paVar4 | uVar1;
  paVar5 = (paVar4 & 0xffff0000 | uVar2);
  if (uVar2 != 0) {
    pass1_1030_5bec(CONCAT22(paVar4,uVar1));
  }
  mem_op_1000_179c(0x8,paVar5);
  pSVar3 = (paVar5 | uVar1);
  if (pSVar3.is_null() == false) {
    pass1_1038_78e2(pSVar3,CONCAT22(paVar5,uVar1));
  }
  u16_1050_574a = (_u16_1050_5748 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_8210(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut pcVar3: *mut c_char;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut pcStack10: *mut c_char;
  let mut pcStack6: *mut c_char;

  pcVar3 = _PTR_LOOP_1050_65e2;
  if (_PTR_LOOP_1050_65e2.is_null() == false) {
    pass1_1028_daba(_PTR_LOOP_1050_65e2);
    fn_ptr_1000_17ce(pcVar3);
  }
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar1 = *param_1;
  uVar2 = (iVar4 + 2);
  pcStack10 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0) {
    pass1_1028_d282(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack10);
  }
  uVar1 = (iVar4 + 0x4);
  uVar2 = (iVar4 + 0x6);
  pcStack6 = CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0) {
    pass1_1028_cff2(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack6);
  }
  pcVar3 = _PTR_LOOP_1050_5736;
  if (_PTR_LOOP_1050_5736.is_null() == false) {
    pass1_1030_5c0e();
    fn_ptr_1000_17ce(pcVar3);
  }
  pcVar3 = _PTR_LOOP_1050_5a64;
  if ((PTR_LOOP_1050_5a66 | _PTR_LOOP_1050_5a64) != 0) {
    pass1_1038_7964((_PTR_LOOP_1050_5a64 & 0xffff | ZEXT24(PTR_LOOP_1050_5a66) << 0x10));
    fn_ptr_1000_17ce(pcVar3);
  }
  _u16_1050_5748 = 0;
  return;
}
pub unsafe fn pass1_1030_82f0(mut param_1: u32,mut param_2: u32)

{
  pass1_1028_d078((param_1 + 0x4),param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_8308(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,param_6: *mut u16,
                    mut param_7: u32)

{
  pass1_1028_e198(param_1,param_2,_PTR_LOOP_1050_65e2,param_5,param_6,param_7);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1030_8326() -> u32

{
  return CONCAT22((_PTR_LOOP_1050_65e2 + 0x2),*_PTR_LOOP_1050_65e2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_8334()

{
  *_PTR_LOOP_1050_65e2 = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_8344(mut param_1: u32,mut param_2: u32)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_2);
  return;
}
pub unsafe fn fn_ptr_1030_835a(u32 **param_1,param_2: *mut c_char)

{
  fn_ptr_1028_d566(*param_1,param_2);
  return;
}
pub unsafe fn pass1_1030_8372(u32 **param_1,mut param_2: u32,param_3: *mut u32)

{
  pass1_1028_d52c(*param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_838e(param_1: u32)

{
  struct_1028_d2b0(*param_1);
  pass1_1028_d01a((param_1 + 0x4));
  send_msg_1028_e242(_PTR_LOOP_1050_65e2,1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_83ba(param_1: u8,u32 **param_2,param_3: i32)

{
  let mut lVar1: i32;

  while (lVar1 = param_3 + -0x1, param_3 != 0) {
    struct_1028_d2b0(*param_2);
    pass1_1028_d01a((param_2 + 0x4));
    param_3 = lVar1;
    if (lVar1 != 0) {
      send_msg_1028_e242(_PTR_LOOP_1050_65e2,0x0);
    }
  }
  send_msg_1028_e242(_PTR_LOOP_1050_65e2,1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn vsprintf_op_1030_840a(mut param_1: u16 ,mut param_2: u32)

{
  WORD *pWVar1;
  let mut local_106: [u8;0x100] = [0;0x100];
  WORD *pWStack6;
  let mut uStack4: u16;

  if (u16_1050_574c != 0) {
    pWVar1 = &stack0x0008;
    uStack4 = SUB42(&DAT_1050_1050,0x0);
    pWStack6 = pWVar1;
    if (u16_1050_5750 == 0) {
      pass1_1000_2b3c(param_1,s_simres_out_1050_5758,&DAT_1050_1050,s_w_1050_5756,
                      &DAT_1050_1050,&stack0xfffe);
      _u16_1050_5752 = CONCAT22(param_1,pWVar1);
      u16_1050_5750 = 0x1;
    }
    wvsprintf16(pWStack6,CONCAT22(param_2,uStack4),CONCAT22(local_106,(param_2 >> 0x10)));
    pass1_1000_2b5c(_u16_1050_5752,(_u16_1050_5752 >> 0x10),s__s_1050_5763,
                    &DAT_1050_1050);
    pass1_1000_2f48(_u16_1050_5752);
  }
  return;
}
pub unsafe fn pass1_1030_8480(param_1: *mut StructD)

{
  fn_ptr_1000_17ce(*param_1);
  return;
}
pub unsafe fn pass1_1030_8496(mut param_1: u32)

{
  fn_ptr_1000_17ce(*(param_1 + 0x2));
  return;
}
pub unsafe fn pass1_1030_84ae(mut param_1: u32)

{
  pass1_1008_3e38((param_1 & 0xffff0000 | (param_1 + 0x8)));
  (param_1 + 0x1e) = 0x1;
  return;
}
pub unsafe fn fn_ptr_1030_84d0(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if ((iVar4 + 0x1e) != 0) {
    puVar1 = (iVar4 + 0xe);
    uVar2 = (iVar4 + 0x10);
    if ((uVar2 | puVar1) != 0) {
      ppcVar3 = *puVar1;
      (**ppcVar3)();
    }
    puVar1 = (iVar4 + 0x12);
    uVar2 = (iVar4 + 0x14);
    if ((uVar2 | puVar1) != 0) {
      ppcVar3 = *puVar1;
      (**ppcVar3)();
    }
    fn_ptr_1000_17ce(*(iVar4 + 0x4));
    fn_ptr_1000_17ce(*(iVar4 + 0x16));
  }
  return;
}
pub unsafe fn struct_1030_8544(param_1: *mut astruct_355,param_2: *mut astruct_356)

{
  let mut iVar1: *mut astruct_356;
  let mut iVar2: *mut astruct_355;
  let mut uVar1: *mut astruct_356;
  let mut uVar2: *mut astruct_355;

  param_1.field0_0x0 = param_2.field0_0x0;
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  iVar2.field2_0x4 = iVar1.field3_0x4;
  pass1_1008_3f62((param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x8)),
                  (param_2 & 0xffff0000 | ZEXT24(&iVar1.field_0x8)));
  iVar2.field9_0xe = iVar1.field10_0xe;
  iVar2.field10_0x12 = iVar1.field11_0x12;
  iVar2.field11_0x16 = iVar1.field12_0x16;
  iVar2.field12_0x1a = iVar1.field13_0x1a;
  iVar2.field15_0x1e = 0;
  return;
}
pub unsafe fn pass1_1030_85be(param_1: *mut astruct_172,mut param_2: u16 ,mut param_3: i16)

{
  let mut iVar1: *mut astruct_172;
  let mut uVar1: *mut astruct_172;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1 = 0;
  iVar1.field2_0x4 = 0;
  iVar1.field3_0x6 = param_3;
  iVar1.field4_0x8 = param_2;
  iVar1[0x1].field2_0x4 = 0;
  if (iVar1.field3_0x6 == 0) {
    iVar1.field3_0x6 = 0x5;
  }
  pass1_1030_878c(param_1);
  return;
}
pub unsafe fn pass1_1030_8604(param_1: *mut StructD)

{
  fn_ptr_1000_17ce(*param_1);
  return;
}
pub unsafe fn pass1_1030_861a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  let mut puStack6: *mut u32;

  pass1_1030_8854(param_3,param_4,param_5);
  puStack6 = CONCAT22(param_2,param_1);
  if ((param_2 | param_1) == 0) {
    (param_3 + 0xa) = 0;
  }
  else {
    (param_3 + 0xa) = *puStack6;
  }
  return;
}
pub unsafe fn pass1_1030_8660(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u32,mut param_5: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut puStack6: *mut u32;

  uVar2 = param_3;
  uVar3 = (param_3 >> 0x10);
  pass1_1030_8854(uVar2,uVar3,param_5);
  puStack6 = CONCAT22(param_2,param_1);
  uVar1 = param_2 | param_1;
  if (uVar1 == 0) {
    pass1_1030_8854(uVar2,uVar3,0x0);
    puStack6 = CONCAT22(uVar1,param_1);
    uVar1 |= param_1;
    if (uVar1 == 0) {
      pass1_1030_878c(param_3);
      pass1_1030_8854(uVar2,uVar3,0x0);
      puStack6 = CONCAT22(uVar1,param_1);
      if ((uVar1 | param_1) == 0) {
        return;
      }
    }
    (puStack6 + 0x4) = param_5;
    *puStack6 = *param_4;
    pass1_1030_8834(param_3);
  }
  else {
    *puStack6 = *param_4;
  }
  return;
}
pub unsafe fn pass1_1030_86ec(param_1: *mut astruct_612,mut param_2: u16 )

{
  let mut iVar1: *mut astruct_612;
  let mut uVar1: u16;

  fn_ptr_1000_17ce(*param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1 = 0;
  iVar1.field4_0x4 = 0;
  iVar1.field5_0x6 = param_2;
  iVar1.field12_0xe = 0;
  return;
}
pub unsafe fn pass1_1030_871e(param_1: *mut i32,param_2: *mut u32,mut param_3: u16 )

{
  let mut piVar1: *mut i16;
  let mut iVar2: *mut astruct_681;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*param_1 == 0) {
    pass1_1030_878c((param_1 & 0xffff | uVar2 << 0x10));
  }
  piVar1 = &iVar2.field14_0xe;
  *piVar1 = *piVar1 + 1;
  (*param_1 + iVar2.field14_0xe * 0x6 + 0x4) = param_3;
  (iVar2.field14_0xe * 0x6 + *param_1) = *param_2;
  return;
}
pub unsafe fn pass1_1030_877c(param_1: *mut u16)

{
  pass1_1030_8834(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_878c(param_1: *mut astruct_172)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_EDX: u32;
  let mut pSVar4: *mut StructD;
  let mut iVar4: *mut astruct_172;
  let mut uVar4: *mut astruct_172;
  let mut lVar5: i32;
  let mut uStack12: u32;

  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (iVar4.field2_0x4 == 0) {
    pSVar4 = (in_EDX & 0xffff0000);
    uVar2 = iVar4.field3_0x6;
  }
  else {
    uVar3 = iVar4.field2_0x4;
    puVar1 = &iVar4.field4_0x8;
    uVar2 = uVar3 + *puVar1;
    pSVar4 = (in_EDX & 0xffff0000 | CARRY2(uVar3,*puVar1));
  }
  if (pSVar4 == 0) {
    if (param_1 == 0) {
      if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar4);
        PTR_LOOP_1050_5f2e = pSVar4;
      }
      else {
      }
      uVar3 = fn_ptr_op_1000_1708(uVar2 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
    }
    else {
      lVar5 = pass1_1000_0ed4(0x1,uVar2 * 0x6,0x0,param_1,
                              (param_1 >> 0x10));
      PTR_LOOP_1050_5f2e = (lVar5 >> 0x10);
      uVar3 = lVar5;
    }
    uStack12 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    if ((PTR_LOOP_1050_5f2e | uVar3) != 0) {
      iVar4.field2_0x4 = uVar2;
      param_1 = uStack12;
      pass1_1030_8834((param_1 & 0xffff | ZEXT24(uVar4) << 0x10));
    }
  }
  return;
}
pub unsafe fn pass1_1030_8834(param_1: *mut u16)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 2);
  pass1_1000_4aea(*param_1,uVar1,(uVar1 >> 0x10),0x6,0x888e);
  return;
}
pub unsafe fn pass1_1030_8854(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut local_c: u32;
  let mut uStack8: u16;

  uStack8 = param_3;
  local_c = 0;
  uVar1 = (param_1 + 2);
  pass1_1000_49c6(&local_c,&DAT_1050_1050,*_param_1,uVar1,(uVar1 >> 0x10),0x6,
                  0x888e);
  return;
}



pub unsafe fn pass1_1030_888e(mut param_1: u32,mut param_2: u32) -> u16

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (param_1 + 0x4);
  uVar4 = (param_2 >> 0x10);
  piVar1 = (param_2 + 0x4);
  if (*piVar1 != iVar2 && iVar2 <= *piVar1) {
    return 0xffff;
  }
  if ((param_2 + 0x4) < (param_1 + 0x4)) {
    return 0x1;
  }
  return 0x0;
}
pub unsafe fn pass1_1030_88ce(param_1: *mut u16,mut param_2: u32,mut param_3: u32)

{
  let mut uVar1: u32;
  let mut puVar2: *mut u8;
  let mut uVar3: u16;
  let mut in_EDX: u32;
  let mut uVar6: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar4: *mut astruct_354;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut puStack38: *mut u16;
  let mut iStack34: i16;
  let mut local_20: [u8;0x2] = [0;0x2];
  let mut local_1e: i16;
  let mut local_1c: i16;
  let mut local_1a: [u8;0x6] = [0;0x6];
  let mut local_14: [u8;0x6] = [0;0x6];
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut iStack6: i16;
  let mut uStack4: u16;
  let mut paVar5: *mut Struct57;

  uVar7 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x389a;
  iVar4.field2_0x2 = 0x1008;
  uVar6 = (in_EDX >> 0x10);
  pass1_1030_84ae(param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x4));
  iVar4.field32_0x24 = param_3;
  puStack38 = (param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x28));
  pass1_1008_6c90((param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x28)));
  iVar4.field45_0x34 = 0;
  *param_1 = 0x8e38;
  iVar4.field2_0x2 = 0x1030;
  struct_1030_8544((param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x4)),param_2);
  uVar8 = pass1_1008_4772(iVar4.field17_0x12);
  uStack4 = (uVar8 >> 0x10);
  iStack6 = uVar8;
  uStack10 = (iStack6 + 0x4);
  uStack14 = (iStack6 + 0x8);
  pass1_1008_3e54(CONCAT22(0x1050,local_14),0x0,uStack14 - 0x1,uStack10 - 1);
  pass1_1008_3e54(CONCAT22(0x1050,local_1a),0x0,0x0,0x0);
  pass1_1008_6d18(puStack38,CONCAT22(0x1050,local_14),CONCAT22(0x1050,local_1a));
  pass1_1008_6d64(puStack38,CONCAT22(0x1050,local_1a));
  pass1_1008_3eb4(CONCAT22(0x1050,local_1a),CONCAT22(0x1050,local_20),
                  CONCAT22(0x1050,&local_1e),CONCAT22(0x1050,&local_1c));
  puVar2 = ((local_1e * local_1c) >> 0x10);
  uVar1 = local_1e * local_1c & 0xffff;
  iVar4.field45_0x34 = uVar1;
  iVar4.field46_0x36 = puVar2;
  paVar4 = CONCAT22(uVar6,puVar2);
  for (iStack34 = 0; iStack34 < 0x5; iStack34 += 1) {
    mem_op_1000_179c(0x10,paVar4);
    uVar3 = paVar4 | uVar1;
    paVar5 = (paVar4 & 0xffff0000 | uVar3);
    if (uVar3 == 0) {
      (&iVar4[0x1].field_0x0 + iStack34 * 0x4) = 0;
    }
    else {
      pass1_1030_85be((uVar1 & 0xffff | paVar4 << 0x10),0x19,0x64);
      (&iVar4[0x1].field_0x0 + iStack34 * 0x4) = uVar1;
      (&iVar4[0x1].field2_0x2)[iStack34 * 0x2] = paVar5;
    }
    paVar4 = paVar5;
  }
  return;
}
pub unsafe fn pass1_1030_8a2c(param_1: *mut StructD)

{
  let mut uVar1: u16;
  let mut pcVar2: *mut c_char;
  let mut iVar3: *mut StructD;
  let mut uVar3: u16;
  let mut iStack4: i16;

  uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  param_1.address_offset_field_0x0 = 0x8e38;
  iVar3.address_offset_field_0x2 = 0x1030;
  iStack4 = 0;
  loop {
    pcVar2 = *(&iVar3.field_0x38 + iStack4 * 0x4);
    uVar1 = (&iVar3.field_0x3a + iStack4 * 0x4);
    if ((uVar1 | pcVar2) != 0) {
      pass1_1030_8604((pcVar2 & 0xffff | uVar1 << 0x10));
      fn_ptr_1000_17ce(pcVar2);
    }
    iStack4 += 0x1;
  } while (iStack4 < 0x5);
  fn_ptr_1030_84d0(param_1 & 0xffff0000 | ZEXT24(&iVar3.hfile_0x4));
  param_1.address_offset_field_0x0 = 0x389a;
  iVar3.address_offset_field_0x2 = 0x1008;
  return;
}
pub unsafe fn pass1_1030_8aa0(mut param_1: u32,mut param_2: u32,param_3: *mut u16,mut param_4: u16 )

{
  let mut uVar1: u16;
  let mut local_12: u32;
  let mut puStack14: *mut u8;
  let mut uStack12: u32;
  let mut local_8: [u8;0x2] = [0;0x2];
  let mut local_6: [u8;0x2] = [0;0x2];
  let mut local_4: [u8;0x2] = [0;0x2];

  puStack14 = local_8;
  pass1_1008_3eb4(param_3,CONCAT13(0x10,CONCAT12(0x50,puStack14)),
                  CONCAT22(0x1050,local_6),CONCAT22(0x1050,local_4));
  bad_1030_8cd2();
  uStack12 = CONCAT22(param_4,puStack14);
  uVar1 = param_4 | puStack14;
  if (uVar1 != 0) {
    pass1_1030_8d9e(param_1);
    local_12 = param_2;
    pass1_1030_8660(&local_12,uVar1,uStack12,CONCAT22(0x1050,&local_12),puStack14);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub unsafe fn pass1_1030_8b00(mut param_1: u32,param_2: *mut u16,param_3: *mut u16)

{
  let mut puVar1: *mut u32;
  let mut piVar2: *mut i16;
  let mut uVar3: u16;
  let mut local_2a: u32;
  let mut uStack38: u32;
  let mut uStack28: u32;
  let mut puStack18: *mut u32;
  let mut puStack16: *mut u32;
  let mut piStack14: *mut i16;
  let mut local_c: i16;
  let mut local_a: [u8;0x4] = [0;0x4];
  let mut uStack6: u32;

  uStack6 = 0;
  puVar1 = (local_a + 2);
  piVar2 = &local_c;
  pass1_1008_3eb4(param_2,CONCAT13(0x10,CONCAT12(0x50,piVar2)),
                  CONCAT22(0x1050,local_a),CONCAT22(0x1050,puVar1));
  bad_1030_8cd2();
  puStack16 = puVar1;
  piStack14 = piVar2;
  pass1_1030_8d9e(param_1);
  puStack18 = puVar1;
  pass1_1030_861a(puVar1,piVar2,puStack16,piStack14,puVar1);
  uStack38 = *puVar1;
  uVar3 = (puVar1 + 2);
  uStack38._3_1_ = (uStack38 >> 0x18);
  uStack6 = uStack38;
  if (uStack38._3_1_ == '\0') {
    puVar1 = &local_2a;
    uStack28 = uStack38;
    pass1_1030_8c66(param_1,local_c,local_a,(local_a >> 0x10),CONCAT22(0x1050,puVar1),
                    uVar3);
    uStack6 = *puVar1;
    uVar3 = (puVar1 + 2);
  }
  *param_3 = uStack6;
  (param_3 + 0x2) = uVar3;
  return;
}
pub unsafe fn pass1_1030_8bac(mut param_1: u32,mut param_2: u16 )

{
  let mut iStack4: i16;

  iStack4 = 0;
  loop {
    pass1_1030_86ec(*(astruct_612 **)(param_1 + 0x38 + iStack4 * 0x4),param_2);
    iStack4 += 0x1;
  } while (iStack4 < 0x5);
  return;
}
pub unsafe fn pass1_1030_8bdc(mut param_1: u32,mut param_2: u32,param_3: *mut u16)

{
  let mut puVar1: *mut u8;
  let mut local_12: u32;
  let mut puStack14: *mut u8;
  plStack12: *mut i32;
  let mut local_8: [u8;0x2] = [0;0x2];
  let mut local_6: [u8;0x2] = [0;0x2];
  let mut local_4: [u8;0x2] = [0;0x2];

  puStack14 = local_4;
  puVar1 = local_8;
  pass1_1008_3eb4(param_3,CONCAT13(0x10,CONCAT12(0x50,puVar1)),
                  CONCAT22(0x1050,local_6),CONCAT22(0x1050,puStack14));
  bad_1030_8cd2();
  plStack12 = CONCAT22(puVar1,puStack14);
  pass1_1030_8d9e(param_1);
  local_12 = param_2;
  pass1_1030_871e(plStack12,CONCAT22(0x1050,&local_12),puStack14);
  return;
}
pub unsafe fn pass1_1030_8c38(mut param_1: u32)

{
  let mut iStack4: i16;

  iStack4 = 0;
  loop {
    pass1_1030_877c((param_1 + 0x38 + iStack4 * 0x4));
    iStack4 += 0x1;
  } while (iStack4 < 0x5);
  return;
}
pub unsafe fn pass1_1030_8c66(mut param_1: u32,mut param_2: i16,param_3: *mut u8,mut param_4: u16 ,param_5: *mut u32,mut param_6: u16 )

{
  let mut bVar1: u8;
  let mut uVar2: u16;
  let mut uStack6: u32;

  pass1_1008_4544((param_1 + 0x12));
  bVar1 = *param_3;
  uVar2 = bVar1;
  uStack6 = (uVar2 + 1);
  if (0x0 < param_2) {
    if (uVar2 == 0) {
      uStack6 = 0x7;
    }
    else if (((bVar1 == 0) || (SBORROW2(uVar2,1))) || (0x1 < (uVar2 - 1))) {
      uStack6 = 0x9;
    }
    else {
      uStack6 = 0x8;
    }
  }
  *param_5 = uStack6;
  return;
}
pub unsafe fn bad_1030_8cd2()

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_8d08(mut param_1: u32,mut param_2: u16 )

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut paStack16: *mut astruct_358;
  let mut iStack4: i16;

  paVar5 = CONCAT22(in_register_0000000a,param_2);
  iStack4 = 0;
  loop {
    uVar6 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x1e);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    iVar3 = iStack4 * 0x6;
    uVar2 = (param_1 + 0x1a);
    (uVar2 + iVar3 + 0x4) = 0;
    pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x500);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(paVar5,iVar3));
    paStack16 = CONCAT22(paVar5,iVar3);
    uVar7 = pass1_1028_e2e0(paVar5,_PTR_LOOP_1050_65e2,0x7);
    paVar5 = (paVar5 & 0xffff0000 | uVar7 >> 0x10);
    uVar4 = (uVar7 >> 0x10);
    pass1_1030_7e5a(uVar4,paStack16,uVar7 & 0xffff | uVar4 << 0x10);
    iStack4 += 0x1;
  }
  return;
}
pub unsafe fn pass1_1030_8d9e(mut param_1: u32)

{
  let mut local_c: [u8;0x2] = [0;0x2];
  let mut local_a: [u8;0x2] = [0;0x2];
  let mut local_8: [u8;0x6] = [0;0x6];

  pass1_1008_3e38(CONCAT22(0x1050,local_8));
  pass1_1008_6d64((param_1 & 0xffff0000 | (param_1 + 0x28)),CONCAT22(0x1050,local_8));
  pass1_1008_3e94(CONCAT22(0x1050,local_8),CONCAT22(0x1050,local_c),CONCAT22(0x1050,local_a));
  return;
}



pub unsafe fn pass1_1030_8e12(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1030_8a2c(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1030_8e3c(param_1: *mut StructD,mut param_2: u32,mut param_3: u32,mut param_4: u16 ) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut paVar7: *mut Struct57;
  let mut paVar9: *mut Struct57;
  let mut puVar10: *mut u32;
  let mut in_stack_0000fe8a: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb8: u16;
  let mut uVar11: u16;
  let mut paVar8: *mut Struct57;

  uVar1 = (param_1 >> 0x10);
  paVar7 = (param_1 & 0xffff0000 | param_1 & 0xffff);
  mem_op_1000_179c(0xc,paVar7);
  uVar4 = paVar7 | uVar1;
  paVar9 = (paVar7 & 0xffff0000);
  paVar8 = (paVar9 | uVar4);
  if (uVar4 == 0) {
    uVar2 = 0;
  }
  else {
    uVar2 = set_struct_1008_574a(CONCAT22(paVar7,uVar1));
    paVar9 = paVar8;
  }
  uVar5 = SUB42(paVar9,0x0);
  if (param_3._3_1_ == '\x04') {
    puVar10 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,CONCAT22(param_4,0x2f),in_stack_0000fe8a,
                              in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
    uVar4 = (puVar10 >> 0x10);
    uVar1 = (puVar10 + 0x1e);
    uVar3 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3);
    uVar11 = (param_2 >> 0x10);
    uVar6 = uVar4;
    if (uVar1 < 1) {
      pass1_1030_9296(uVar4,param_2,CONCAT22(uVar5,uVar2),CONCAT22(uVar4,uVar3));
      pass1_1030_951a(uVar6,param_2,CONCAT22(uVar5,uVar2),CONCAT22(uVar4,uVar3));
    }
    else {
      pass1_1030_9adc(uVar3,uVar4,param_2,uVar11,CONCAT22(uVar5,uVar2),CONCAT22(uVar4,uVar3));
      pass1_1030_9c1c(param_2,CONCAT22(uVar5,uVar2),CONCAT22(uVar4,uVar3));
    }
    pass1_1030_9d42(uVar6,param_2,uVar11,CONCAT22(uVar5,uVar2),CONCAT22(uVar4,uVar3));
  }
  return CONCAT22(uVar5,uVar2);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1030_8f04(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut iStack8: i16;
  let mut uStack6: u32;

  pass1_1038_53ba(param_5,1);
  if ((((param_2 != 0) || (0x1 < param_1)) &&
      ((pass1_1038_53ba(param_5,0x2), param_2 != 0x0 || (0x1 < param_1)))) &&
     ((pass1_1038_53ba(param_5,0x3), param_2 != 0x0 || (0x1 < param_1)))) {
    pass1_1038_53ba(param_5,0x4);
    uVar5 = param_2;
    if ((param_2 != 0) || (0x1 < param_1)) {
      empty_1038_540a();
      uStack6 = param_1 & 0xffff | uVar5 << 0x10;
      iStack8 = 0;
      loop {
        uVar3 = uVar5;
        uVar2 = param_1;
        if (0x0 < (iStack8 * 0x2 + _PTR_LOOP_1050_580e)) {
          empty_1038_540a();
          uVar6 = (_PTR_LOOP_1050_580e >> 0x10);
          uVar1 = (iStack8 * 0x2 + _PTR_LOOP_1050_580e);
          param_1 = uVar1;
          uVar4 = uVar1 >> 0xf;
          uVar5 = uVar4;
          if ((uVar3 <= uVar4) && ((uVar3 < uVar4 || (uVar2 < uVar1)))) {
            if (0x1c < iStack8) {
              return;
            }
            uVar2 = (iStack8 * 0x2 + _PTR_LOOP_1050_580e);
            param_1 = uVar2;
            uVar5 = param_1 >> 0x10;
            if (uStack6 < param_1) {
              return;
            }
            uStack6 = CONCAT22(((uStack6 >> 0x10) - (uVar2 >> 0xf)) - (uStack6 < uVar2),
                               uStack6 - uVar2);
          }
        }
        iStack8 += 0x1;
        if (0x24 < iStack8) {
          return;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1030_8fe4(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,param_6: i32) -> BOOL16

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_5,param_6);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar2 | param_1) != 0) {
      uVar3 = struct_op_1030_73a8(CONCAT22(uVar2,param_1),param_1,uVar2 | param_1);
      if ((uVar3 != 0) && ((iVar1 = (uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}
