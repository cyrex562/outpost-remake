
pub fn pass1_1028_90aa(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_90e6(param_1: *mut astruct_97,mut param_2: u16 )

{
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x1387);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x108) = param_2;
  param_1->offset_0x0 = 0x932c;
  (param_1 + 0x2) = 0x1028;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_9114(param_1: *mut StructD,mut param_2: u32,mut param_3: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  paVar4: *mut astruct_67;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000fe92: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc0: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uStack10: u16;

  paVar4 =
           mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(param_3,0x37),in_stack_0000fe92,
                           in_stack_0000ffb6,in_stack_0000ffbc,in_stack_0000ffc0);
  uVar2 = (param_1 >> 0x10);
  uVar3 = param_2;
  iVar1 = (uVar3 + 0x108);
  if (iVar1 - 0x1 < 0x8) {
    uStack10 = *_PTR_LOOP_1050_65e2;
    iVar8 = (*_PTR_LOOP_1050_65e2 >> 0x10);
    switch(iVar1) {
    case 0x1:
      iVar1 = 0x16;
      break;
    case 0x2:
      iVar1 = 0x17;
      break;
    case 0x3:
      iVar1 = 0x18;
      break;
    case 0x4:
      iVar1 = 0x1b;
      break;
    case 0x5:
      iVar1 = 0x1f;
      break;
    case 0x6:
      iVar1 = 0x24;
      break;
    case 0x7:
      pass1_1008_612e(uVar3,0x0,0x14);
      iVar1 = (uVar3 >> 0xf) + (0xff91 < uVar3);
      uVar6 = uStack10 + uVar3 + 0x6e;
      uVar7 = iVar8 + iVar1 + CARRY2(uStack10,uVar3 + 0x6e);
      iVar8 = 0x7;
      puVar5 = mixed_1010_20ba(CONCAT22(uVar2,iVar1),_u16_1050_0ed0,CONCAT22(uVar6,0x2f),
                               in_stack_0000fe8c,in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
      uVar2 = (puVar5 >> 0x10);
      uVar3 = puVar5;
      pass1_1010_ebf8(puVar5,uVar6,uVar7,iVar8);
      pass1_1008_612e(uVar3,0x1,0x64);
      if (0x32 < uVar3) {
        return;
      }
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
      pass1_1038_4900(CONCAT22(uVar2,uVar3));
      iVar1 = 0x2c;
      break;
    case 0x8:
      pass1_1008_612e(uVar3,0x0,0x14);
      iVar1 = (uVar3 >> 0xf) + (0xff9b < uVar3);
      uVar6 = uStack10 + uVar3 + 0x64;
      uVar7 = iVar8 + iVar1 + CARRY2(uStack10,uVar3 + 0x64);
      iVar9 = 0x8;
      puVar5 = mixed_1010_20ba(CONCAT22(uVar2,iVar1),_u16_1050_0ed0,CONCAT22(uVar6,0x2f),
                               in_stack_0000fe8c,in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
      iVar1 = (puVar5 >> 0x10);
      iVar8 = puVar5;
      pass1_1010_ebf8(puVar5,uVar6,uVar7,iVar9);
      if (0x19 < uVar3) {
        return;
      }
      uVar3 = 0x1;
      uVar10 = 0x2;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
      pass1_1038_43cc(iVar8,iVar1,iVar8,iVar1,uVar3,uVar10);
      iVar1 = 0x2d;
    }
    post_win_msg_1008_a0e4(paVar4,0x0,0x0,0x1,0x0,iVar1);
  }
  return;
}
pub fn pass1_1028_9264(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut puVar3: *mut u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut iVar7: i16;
  let mut puVar8: *mut u32;
  let mut uVar9: u16;
  let mut puStack10: *mut u16;

  paVar6 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10a,paVar6);
  uVar5 = paVar6;
  puStack10 = CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar9 = (param_3 >> 0x10);
    iVar7 = param_3;
    (param_1 + 0x4) = (iVar7 + 0x4);
    puVar3 = (iVar7 + 0x8);
    puVar8 = (param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0; iVar4 += -1) {
      puVar2 = puVar8;
      puVar8 = puVar8 + 1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    (param_1 + 0x108) = (iVar7 + 0x108);
    *puStack10 = 0x932c;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



pub fn pass1_1028_9300(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_op_1028_933c(param_1: *mut astruct_97,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u32,mut param_6: u16 ,
                        mut param_7: u32,mut param_8: u32)

{
  iVar1: *mut astruct_97;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x3e8);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  &iVar1.field259_0x108 = param_8;
  &iVar1.field262_0x10c = param_7;
  iVar1.field264_0x110 = 0;
  iVar1.field265_0x114 = *param_5;
  iVar1.field266_0x118 = (param_5 + 1);
  iVar1.field267_0x11a = param_4;
  iVar1.field268_0x11c = param_2;
  iVar1.field270_0x120 = 0;
  iVar1.field269_0x11e = 0;
  iVar1.field271_0x122 = param_3;
  param_1->offset_0x0 = 0x9934;
  iVar1->segment_0x2 = 0x1028;
  sys_1000_3f9c((param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),
                s_SCPutBldg_site_0x_08lx__bldg__u__1050_50ce,param_8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_93d4(mut param_1: u16 ,mut param_2: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut puVar4: *mut u8;
  let mut in_register_0000000a: u16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut local_112: [u8;0x10c] = [0;0x10c];
  let mut uStack6: u32;

  PTR_LOOP_1050_50ca = NULL;
  PTR_LOOP_1050_50cc = NULL;
  uVar6 = (param_2 >> 0x10);
  iVar5 = param_2;
  uStack6 = pass1_1028_e2e0(CONCAT22(in_register_0000000a,param_1),_PTR_LOOP_1050_65e2,0x7);
  puVar4 = (uStack6 >> 0x10);
  uVar2 = uStack6;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | ZEXT24(puVar4) << 0x10);
  (iVar5 + 0x11e) = uVar2;
  (iVar5 + 0x120) = puVar4;
  uVar3 = iVar5 + 0x114;
  ppcVar1 = ((iVar5 + 0x11e) + 0x1c);
  (**ppcVar1)();
  if (uVar3 != 0) {
    pass1_1028_9624(uVar3,puVar4,param_2);
    ppcVar1 = ((iVar5 + 0x11e) + 0x20);
    (**ppcVar1)();
    ppcVar1 = ((iVar5 + 0x11e) + 0x18);
    (**ppcVar1)();
    pass1_1028_9600(puVar4,param_2);
    return;
  }
  (iVar5 + 0x11e) = 0;
  struct_1030_e4fa(CONCAT22(0x1050,local_112),uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_112));
  if (PTR_LOOP_1050_50ca.is_null()) {
    PTR_LOOP_1050_50ca = 0x6ad;
  }
  return;
}
pub fn pass1_1028_94e4(param_1: *mut astruct_328,param_2: *mut u8,param_3: *mut astruct_329)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  iVar5: *mut astruct_329;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x124,paVar5);
  uVar4 = paVar5;
  puStack10 = CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0) {
    *puStack10 = 0x389a;
    param_1.field2_0x2 = 0x1008;
    uVar8 = (param_3 >> 0x10);
    iVar5 = param_3;
    param_1.field3_0x4 = iVar5.field4_0x4;
    puVar6 = &iVar5.field5_0x8;
    puVar7 = &param_1.field4_0x8;
    for (iVar3 = 0x40; iVar3 != 0; iVar3 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1.field2_0x2 = 0x1028;
    param_1.field257_0x108 = iVar5.field258_0x108;
    param_1.field258_0x10c = iVar5.field259_0x10c;
    param_1.field259_0x110 = iVar5.field260_0x110;
    param_1.field260_0x114 = iVar5.field261_0x114;
    param_1.field261_0x118 = iVar5.field262_0x118;
    param_1.field262_0x11a = iVar5.field263_0x11a;
    param_1.field263_0x11c = iVar5.field264_0x11c;
    param_1.field264_0x11e = iVar5.field265_0x11e;
    param_1.field265_0x122 = iVar5.field266_0x122;
    *puStack10 = 0x9934;
    param_1.field2_0x2 = 0x1028;
  }
  return;
}
pub fn pass1_1028_9600(param_1: *mut u8,mut param_2: u32)

{
  let mut puVar1: *mut u16;
  let mut local_6: [u8;0x4] = [0;0x4];

  puVar1 = pass1_1020_a43e(param_1,CONCAT22(0x1050,local_6));
  pass1_1020_a80e(local_6,(puVar1 >> 0x10),local_6,&DAT_1050_1050,
                  (param_2 + 0x11a));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_9624(mut param_1: u16 ,param_2: *mut u8,param_3: *mut astruct_688)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut BVar5: bool;
  let mut uVar7: u32;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  iVar9: *mut astruct_688;
  let mut unaff_SI: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut in_stack_0000fd54: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000fe7e: u16;
  let mut in_stack_0000fe82: u16;
  let mut puVar11: *mut u8;
  let mut puVar12: *mut u8;
  let mut uStack332: u16;
  let mut uStack330: u16;
  let mut uStack64: u16;
  let mut uStack62: u32;
  let mut iStack58: i16;
  let mut uStack56: u32;
  let mut puStack46: *mut u32;
  let mut uStack42: u32;
  let mut local_26: [u8;0x4] = [0;0x4];
  let mut uStack34: u16;
  let mut puStack32: *mut u8;
  let mut uStack30: u32;
  let mut uStack26: u32;
  let mut puStack22: *mut u32;
  let mut local_12: [u8;0x2] = [0;0x2];
  let mut local_10: [u8;0x2] = [0;0x2];
  let mut local_e: [u8;0x2] = [0;0x2];
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut puStack6: *mut u32;
  let mut puVar6: *mut u32;

  paVar8 = CONCAT22(in_register_0000000a,param_2);
  uVar9 = (param_3 >> 0x10);
  iVar9 = param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar9.field265_0x10c);
  &iVar9.field266_0x110 = param_1;
  (&iVar9.field266_0x110 + 0x2) = paVar8;
  puStack6 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2f),in_stack_0000fd54,
                             in_stack_0000fe78,in_stack_0000fe7e,in_stack_0000fe82);
  uStack10 = (puStack6 >> 0x10);
  puVar2 = &iVar9.field267_0x114;
  pass1_1030_64ce(puVar2,uStack10,_PTR_LOOP_1050_5740,(param_3 & 0xffff0000 | ZEXT24(puVar2)),
                  iVar9.field264_0x108,CONCAT22(0x1050,local_26));
  uStack56 = *puVar2;
  uStack56._3_1_ = (uStack56 >> 0x18);
  uStack12 = (uStack56._3_1_ != '\0');
  uVar10 = 0x1008;
  puStack46 = uStack56;
  uStack10 = uStack56;
  pass1_1008_3eb4((param_3 & 0xffff0000 | ZEXT24(&iVar9.field267_0x114)),
                  CONCAT22(0x1050,local_12),CONCAT22(0x1050,local_10),
                  CONCAT22(0x1050,local_e));
  if (uStack12 == 0) {
    puVar2 = &iVar9.field267_0x114;
    pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x500);
    puStack22 = CONCAT22(uStack10,puVar2);
    uVar10 = 0x1030;
    pass1_1030_61fe(puVar2,uStack10,_PTR_LOOP_1050_5740,CONCAT22(uStack10,puVar2),
                    param_3 & 0xffff0000 | ZEXT24(&iVar9.field267_0x114),iVar9.field264_0x108);
    if ((iVar9.field270_0x11a == 0xa) || (iVar9.field270_0x11a == 0x37)) {
      if (iVar9.field270_0x11a == 0x37) {
        uStack56 = iVar9.field273_0x11e;
        uStack10 = (&iVar9.field273_0x11e + 2);
        uStack42 = iVar9.field265_0x10c;
        (uStack56 + 0x20) = uStack42;
      }
      puVar2 = &iVar9.field267_0x114;
      pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x400);
      &iVar9.field265_0x10c = puVar2;
      (&iVar9.field265_0x10c + 0x2) = uStack10;
      uVar10 = 0x1018;
      pass1_1018_0196(puVar2,uStack10,puStack6,
                      CONCAT22(uStack10,&iVar9.field265_0x10c),iVar9.field264_0x108);
      if (iVar9.field270_0x11a == 0xa) {
        uVar10 = 0x1010;
        pass1_1010_ed22(puStack6,iVar9.field265_0x10c);
      }
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar9.field265_0x10c);
    &iVar9.field266_0x110 = puVar2;
    (&iVar9.field266_0x110 + 0x2) = uStack10;
    uVar4 = uStack10 | &iVar9.field266_0x110;
    puVar6 = uVar4;
    if (uVar4 == 0) goto LAB_1028_9807;
    uVar3 = SUB42(puStack22,0x0);
    puVar12 = (puStack22 >> 0x10);
    puVar11 = uStack10;
  }
  else {
    puStack22 = uStack10;
    puVar6 = uStack10;
    if (iVar9.field270_0x11a != 0x75) goto LAB_1028_9807;
    uVar3 = SUB42(uStack10,0x0);
    puVar12 = uStack10;
    puVar11 = (&iVar9.field266_0x110 + 2);
  }
  ppcVar1 = (*iVar9.field266_0x110 + 0x8);
  (**ppcVar1)(uVar10,&iVar9.field266_0x110,puVar11,0x0,uVar3,puVar12,0x0);//
// LAB_1028_9807:
  uVar10 = SUB42(puVar6,0x0);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,puStack22);
  uStack26 = CONCAT22(uStack10,uVar10);
  pass1_1030_73ee(uStack10,CONCAT22(uStack10,uVar10),iVar9.field265_0x10c);
  BVar5 = pass1_1008_c6ae(_u16_1050_06e0,iVar9.field270_0x11a,0x31);
  if ((BVar5 == 0) && (iVar9.field274_0x122 == 0)) {
    uStack62 = (uStack26 + 0xc);
    iStack58 = (uStack26 + 0x10);
    uStack56 = (uStack56 & 0xffff0000 | ZEXT24(&uStack62));
    if (iStack58 < 1) {
      uStack64 = 0x5;
    }
    else {
      uStack64 = 0x6;
    }
    (uStack26 + 0x14) = uStack64;
    uStack10 = uStack26;
  }
  uVar7 = (uStack26 + 0x16);
  uStack30 = uVar7;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7);
  uStack34 = uVar7;
  puStack32 = uStack10;
  if (uStack30 != 0) {
    struct_1030_e4fa(CONCAT22(0x1050,&uStack332),uStack30);
    fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&uStack332));
    uStack332 = 0x389a;
    uStack330 = 0x1008;
  }
  ppcVar1 = (*iVar9.field273_0x11e + 0x4);
  (**ppcVar1)();
  puVar6 = iVar9.field273_0x11e;
  pass1_1030_7e5a(uStack10,uStack26,(puVar6 + 0x4));
  return;
}



pub fn pass1_1028_9908(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1028_9944(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32,mut param_4: u32)

{
  iVar1: *mut astruct_97;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x1387);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  &iVar1.field259_0x108 = param_4;
  &iVar1.field262_0x10c = param_3;
  iVar1.field264_0x110 = param_2;
  iVar1.field265_0x114 = 0;
  param_1->offset_0x0 = 0x9c52;
  iVar1->segment_0x2 = 0x1028;
  return;
}
pub fn pass1_1028_9992(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x9c52;
  (iVar1 + 0x2) = 0x1028;
  fn_ptr_1000_17ce(*(iVar1 + 0x114));
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_99c4(mut param_1: i16,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_3 >> 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(param_3 + 0x10c));
  pass1_1030_355c((param_1 + 0x1f6),(param_3 + 0x114));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_9a02(mut param_1: u32,mut param_2: i16,param_3: *mut StructD,mut param_4: u16 )

{
  let mut lVar1: i32;
  let mut bVar2: bool;
  let mut uVar3: u16;
  paVar4: *mut astruct_92;
  let mut uVar5: u32;
  let mut puVar6: *mut u8;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut iVar10: i16;
  let mut uVar11: u16;
  paVar12: *mut astruct_27;
  paVar13: *mut astruct_67;
  let mut in_stack_0000fe62: u16;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff86: u16;
  let mut in_stack_0000ff8c: u16;
  let mut in_stack_0000ff90: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  let mut uVar14: u8;
  let mut uVar15: u8;
  let mut uVar16: u16;
  let mut uVar17: u16;
  let mut iVar18: i16;
  astruct_92 local_30;
  let mut paVar9: *mut Struct57;

  uVar11 = (param_1 >> 0x10);
  iVar10 = param_1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(iVar10 + 0x108));
  puVar6 = param_3;
  uVar5 = (param_2 + 0x1f6);
  pass1_1030_3694(puVar6,uVar5,0x0,(iVar10 + 0x110));
  uVar7 = uVar5;
  (iVar10 + 0x114) = uVar7;
  (iVar10 + 0x116) = param_3;
  pass1_1030_38b8();
  uVar7 = param_3 | uVar7;
  paVar9 = (param_3 & 0xffff0000 | uVar7);
  if (uVar7 == 0) {
    lVar1 = (param_2 + 0x200);
    paVar12 =
              mixed_1010_20ba(paVar9,_u16_1050_0ed0,CONCAT22(param_4,0x2b),in_stack_0000fe70,
                              in_stack_0000ff94,in_stack_0000ff9a,in_stack_0000ff9e);
    uVar11 = (paVar9 >> 0x10);
    if (lVar1 == 0x8000002) {
      iVar10 = 0x1f;
    }
    else {
      iVar10 = 0xb;
    }
    pass1_1010_043a(paVar12,(param_2 + 0x4),iVar10);
    if (lVar1 == 0x8000001) {
      uVar3 = 0x2;
    }
    else {
      uVar3 = 0x1;
    }
    paVar9 = CONCAT22(uVar11,0x800);
    pass1_1038_349e(CONCAT22(puVar6,param_2),CONCAT22(0x800,uVar3));
    bVar2 = false;
    pass1_1028_dc52(CONCAT13(0x10,CONCAT12(0x50,&local_30)),0x1,0x0,0x400);
    while( true ) {
      paVar4 = &local_30;
      pass1_1028_e4ec(CONCAT22(0x1050,paVar4));
      uVar7 = paVar9;
      uVar8 = uVar7 | paVar4;
      paVar9 = (paVar9 & 0xffff0000 | uVar8);
      if (uVar8 == 0) break;
      if (paVar4[0x1c].field4_0x8 != 0x8000002) {
        bVar2 = true;
      }
    }
    if (!bVar2) {
      uVar17 = 0;
      iVar18 = 0x3c;
      uVar14 = 0x1;
      uVar15 = 0;
      uVar16 = 0;
      uVar3 = 0;
      iVar10 = 0;
      uVar11 = 0;
      paVar13 =
                mixed_1010_20ba(paVar9,_u16_1050_0ed0,0x37,in_stack_0000fe62,in_stack_0000ff86,
                                in_stack_0000ff8c,in_stack_0000ff90);
      post_win_msg_1008_a0e4
                (paVar13,CONCAT22(uVar3,uVar11),iVar10,CONCAT11(uVar15,uVar14),CONCAT22(uVar17,uVar16),iVar18);
    }
  }
  return;
}
pub fn pass1_1028_9b48(param_1: *mut astruct_330,param_2: *mut u8,param_3: *mut astruct_331)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  iVar5: *mut astruct_331;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x118,paVar5);
  uVar4 = paVar5;
  puStack10 = CONCAT22(uVar4,param_1);
  iVar5 = param_3;
  uVar8 = (param_3 >> 0x10);
  if ((uVar4 | param_1) != 0) {
    *puStack10 = 0x389a;
    param_1.field2_0x2 = 0x1008;
    param_1.field3_0x4 = iVar5.field4_0x4;
    puVar6 = &iVar5.field5_0x8;
    puVar7 = &param_1.field4_0x8;
    for (iVar3 = 0x40; iVar3 != 0; iVar3 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1.field2_0x2 = 0x1028;
    param_1.field257_0x108 = iVar5.field258_0x108;
    param_1.field258_0x10c = iVar5.field259_0x10c;
    param_1.field259_0x110 = iVar5.field260_0x110;
    param_1.field260_0x114 = iVar5.field261_0x114;
    *puStack10 = 0x9c52;
    param_1.field2_0x2 = 0x1028;
  }
  iVar5.field261_0x114 = 0;
  return;
}



pub fn pass1_1028_9c2c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1028_9992(&param_1->address_offset_field_0x0);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_97 * struct_1028_9c62(param_1: *mut astruct_97,mut param_2: u16 )

{
  struct_op_1028_d1dc(param_1,param_2);
  (param_1 + 0x108) = param_2;
  param_1->offset_0x0 = 0x9eb6;
  (param_1 + 0x2) = 0x1028;
  return param_1;
}



pub unsafe fn pass1_1028_9c90(mut param_1: u32) -> u16

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar1 = (param_1 + 0x108) - 0x3e8;
  if ((uVar1 < 0x3a99) && (uVar1 % 0x3e8 == 0)) {
    // WARNING: Could not recover jumptable at 0x10289dc0. Too many branches
    // WARNING: Treating indirect jump as call
    uVar2 = ((uVar1 / 0x3e8) * 0x2 + -0x623a))();
    return uVar2;
  }
  return 0x1;
}
pub fn pass1_1028_9dee(param_1: *mut astruct_332,mut param_2: u16 ,param_3: *mut astruct_333)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  iVar5: *mut astruct_333;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10a,paVar5);
  uVar4 = paVar5;
  puStack10 = CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0) {
    *puStack10 = 0x389a;
    param_1.field2_0x2 = 0x1008;
    uVar8 = (param_3 >> 0x10);
    iVar5 = param_3;
    param_1.field3_0x4 = iVar5.field4_0x4;
    puVar6 = &iVar5.field5_0x8;
    puVar7 = &param_1.field4_0x8;
    for (iVar3 = 0x40; iVar3 != 0; iVar3 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1.field2_0x2 = 0x1028;
    param_1.field257_0x108 = iVar5.field258_0x108;
    *puStack10 = 0x9eb6;
    param_1.field2_0x2 = 0x1028;
  }
  return;
}



pub fn pass1_1028_9e8a(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_9ec6(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,s_noth_bmp_1050_2321 + 0x6);
  param_1->offset_0x0 = 0xa6f6;
  (param_1 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)),0x105050f0);
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_9efc(param_1: *mut u16,param_2: *mut StructD,mut param_3: u32,mut param_4: u16 )

{
  let mut lVar1: i32;
  let mut iVar2: i16;
  paVar3: *mut astruct_92;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut lVar7: i32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut paVar12: *mut Struct57;
  paVar13: *mut astruct_67;
  paVar14: *mut astruct_690;
  paVar15: *mut astruct_27;
  let mut in_stack_0000fe6e: u16;
  let mut in_stack_0000ff92: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ff9c: u16;
  let mut uVar16: u16;
  astruct_92 local_18;
  let mut paVar11: *mut Struct57;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
  uVar8 = param_2 | param_1;
  paVar11 = (param_2 & 0xffff0000 | uVar8);
  if (uVar8 != 0) {
    pass1_1028_dc52(CONCAT13(0x10,CONCAT12(0x50,&local_18)),0x1,0x0,0x400);
    while( true ) {
      paVar3 = &local_18;
      pass1_1028_e4ec(CONCAT22(0x1050,paVar3));
      uVar8 = paVar11;
      paVar12 = (paVar11 & 0xffff0000 | (uVar8 | paVar3));
      if ((uVar8 | paVar3) == 0) break;
      lVar1 = paVar3[0x1c].field4_0x8;
      uVar4 = (&paVar3[0x1c].field4_0x8 + 2);
      paVar11 = (paVar11 & 0xffff0000 | uVar4);
      if ((&paVar3[0x1c].field3_0x4 + 0x2) != 0) {
        uVar16 = (param_3 >> 0x10);
        lVar7 = lVar1;
        if ((lVar1 != 0x2) || (uVar4 != 0x800)) {
          pass1_1028_a3ae(lVar1,paVar11,param_3,uVar16,CONCAT22(uVar8,paVar3));
        }
        uVar4 = lVar7;
        pass1_1028_a28a(param_3,uVar16,CONCAT22(uVar8,paVar3));
        if ((paVar11 < 1) && ((paVar11 < 0x0 || (uVar4 < 0x64)))) {
          pass1_1028_a4ee(param_3,CONCAT22(uVar8,paVar3));
        }
        if (lVar1 != 0x8000002) {
          pass1_1038_42cc(CONCAT22(uVar8,paVar3));
          uVar9 = paVar11 | uVar4;
          paVar11 = (paVar11 & 0xffff0000 | uVar9);
          if (uVar9 != 0) {
            paVar13 =
                      mixed_1010_20ba(paVar11,_u16_1050_0ed0,CONCAT22(param_4,0x37),in_stack_0000fe6e,
                                      in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
            paVar11 = (paVar11 & 0xffff0000 | paVar13 >> 0x10);
            post_win_msg_1008_a0e4(paVar13,0x0,uVar4,paVar3[0x1c].field6_0x10,paVar3.field3_0x4,0x2);
          }
        }
      }
    }
    local_18 = 0x389a;
    local_18.field2_0x2 = 0x1008;
    paVar14 =
              mixed_1010_20ba(paVar12,_u16_1050_0ed0,CONCAT22(param_4,0x8),in_stack_0000fe6e,
                              in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
    paVar11 = (paVar12 & 0xffff0000 | paVar14 >> 0x10);
    iVar2 = paVar14;
    iVar5 = iVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
    uVar10 = SUB42(paVar11,0x0);
    iVar6 = iVar5;
    pass1_1010_9f72(paVar14,0x3e);
    if (iVar6 != 0) {
      iVar6 = pass1_1010_96d0(paVar14);
      if (iVar6 < 1) {
        if (iVar6 < 0x0) {
          iVar6 = (iVar5 + 0x1f6);
          pass1_1030_38b8();
          if ((paVar11 < 1) && ((paVar11 < 0x0 || (iVar6 == 0)))) {
            paVar13 =
                      mixed_1010_20ba(paVar11,_u16_1050_0ed0,CONCAT22(iVar2,0x37),in_stack_0000fe6e,
                                      in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
            post_win_msg_1008_a0e4(paVar13,0x0,0x0,0x1,(iVar5 + 0x4),0x6);
          }
        }
      }
      else {
        paVar13 =
                  mixed_1010_20ba(paVar11,_u16_1050_0ed0,CONCAT22(iVar2,0x37),in_stack_0000fe6e,
                                  in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
        paVar11 = (paVar11 & 0xffff0000 | paVar13 >> 0x10);
        post_win_msg_1008_a0e4(paVar13,0x0,iVar6,(iVar5 + 0x208),0x4000001,0x2);
        paVar15 =
                  mixed_1010_20ba(paVar11,_u16_1050_0ed0,CONCAT22(iVar2,0x2b),in_stack_0000fe6e,
                                  in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
        pass1_1010_043a(paVar15,(iVar5 + 0x4),0x14);
      }
    }
  }
  return;
}
