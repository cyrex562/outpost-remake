
pub unsafe fn pass1_1018_427c(param_1: *mut astruct_263,mut param_2: u16 ,mut param_3: u16 )

{
  let mut uVar1: u16;
  pstruct263_1: *mut astruct_263;
  pstruct263_2: *mut astruct_263;
  let mut uVar2: u32;
  let mut lVar3: i32;

  pstruct263_2 = (param_1 >> 0x10);
  pstruct263_1 = param_1;
  uVar2 = switch_1018_3b9e(param_2,param_3,param_1,pstruct263_1[0x1].field4_0x4);
  uVar1 = pstruct263_1[0x1].field4_0x4;
  if (uVar1 == 0x188) {
    lVar3 = pass1_1008_57f0(uVar2,pstruct263_1[0x1].field5_0x6);
    pass1_1018_456a(pstruct263_1,pstruct263_2,(lVar3 + 0xe));
  }
  else if (uVar1 == 0x18b) {
    lVar3 = pass1_1008_57f0(uVar2,pstruct263_1[0x1].field5_0x6);
    pass1_1018_45d4(pstruct263_1,pstruct263_2,(lVar3 + 0xe));
  }
  else if (uVar1 == 0x18c) {
    lVar3 = pass1_1008_57f0(uVar2,pstruct263_1[0x1].field5_0x6);
    pass1_1018_451e(pstruct263_1,pstruct263_2,(lVar3 + 0xe));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1018_435e(mut param_1: u16 ,mut param_2: u32,param_3: i32,mut param_4: i16,mut param_5: i16)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;

  if (param_4 < param_5) {
    param_5 = param_4;
  }
  uVar2 = 0;
  uVar4 = (param_2 >> 0x10);
  uVar1 = (param_2 + 0x122);
  pass1_1008_e852(param_1,uVar1,(uVar1 >> 0x10),*(param_2 + 0x126));
  pass1_1030_8344(_u16_1050_5748,CONCAT22(param_1,uVar2));
  loop {
    loop {
      uVar3 = uVar2;
      pass1_1008_612e(uVar3,param_5,param_4);
      uVar2 = (uVar3 * 0x2 + 0x411c);
    } while (uVar2 == 0);
    if (uVar2 != 1) {
      pass1_1008_612e(uVar2,0x1,uVar2);
    }
    uVar2 -= 1;
    switch_1018_3ee6(param_1,param_2,param_3,uVar2,uVar3);
    param_1 |= uVar2;
  } while (param_1 == 0);
  return;
}



pub unsafe fn switch_1018_43ec(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ) -> u16

{
  let mut uStack6: u16;

  switch(param_3) {
  0xf =>
  0x35 =>
  0x36 =>
    uStack6 = 0x7;
    break;
  _ =>
    uStack6 = 0x1;
    break;
  0x11 =>
  0x13 =>
  0x14 =>
  0x15 =>
  0x2d =>
  0x2e =>
  0x6e =>
    uStack6 = 0x9;
    break;
  0x12 =>
  0x31 =>
  0x32 =>
  0x52 =>
  0x53 =>
  0x54 =>
  0x55 =>
  0x56 =>
  0x5a =>
  0x5b =>
  0x5c =>
  0x5d =>
  0x5e =>
  0x5f =>
    uStack6 = 0x4;
    break;
  0x1b =>
  0x1c =>
  0x1d =>
  0x28 =>
  0x29 =>
  0x2c =>
  0x2f =>
  0x30 =>
  0x68 =>
  0x69 =>
    uStack6 = 0x5;
    break;
  0x1e =>
  0x1f =>
  0x20 =>
  0x33 =>
  0x34 =>
    uStack6 = 0x6;
    break;
  0x22 =>
  0x23 =>
  0x24 =>
    uStack6 = 0x8;
    break;
  0x25 =>
  0x26 =>
  0x27 =>
    uStack6 = 0x2;
    break;
  0x38 =>
  0x39 =>
  0x4f =>
  0x50 =>
  0x51 =>
  0x57 =>
  0x58 =>
  0x59 =>
  0x66 =>
  0x67 =>
  0x6c =>
  0x6d =>
    uStack6 = 0x3;
  }
  return uStack6;
}



pub unsafe fn pass1_1018_451e(param_1: *mut astruct_263,param_2: *mut astruct_263,mut param_3: i16) -> u16

{
  let mut uStack6: u16;

  if (param_3 == 0x7) {
    uStack6 = 0x9;
  }
  else if (param_3 == 0x8) {
    uStack6 = 0xa;
  }
  else if (param_3 == 0xc) {
    uStack6 = 0x19;
  }
  else if (param_3 == 0xd) {
    uStack6 = 0x3;
  }
  else {
    uStack6 = 0x8;
  }
  return uStack6;
}



pub unsafe fn pass1_1018_456a(param_1: *mut astruct_263,param_2: *mut astruct_263,mut param_3: u16 ) -> u16

{
  let mut uStack6: u16;

  switch(param_3) {
  0x11 =>
  0x12 =>
  0x13 =>
  0x14 =>
  0x15 =>
    uStack6 = 0x2;
    break;
  0x16 =>
  0x1e =>
    uStack6 = 0x3;
    break;
  _ =>
    uStack6 = 0x1;
    break;
  0x1d =>
  0x21 =>
    uStack6 = 0x4;
  }
  return uStack6;
}



pub unsafe fn pass1_1018_45d4(param_1: *mut astruct_263,param_2: *mut astruct_263,mut param_3: i16) -> u16

{
  let mut uStack6: u16;

  if (param_3 == 0x3) {
    uStack6 = 0x16;
  }
  else if (param_3 == 0x4) {
    uStack6 = 0x17;
  }
  else {
    uStack6 = 0x14;
  }
  return uStack6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pass1_1018_4608: i32(mut param_1: u32,param_2: *mut c_char,param_3: *mut c_char)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut lVar7: i32;
  let mut pcVar8: *mut c_char;
  let mut pcVar9: *mut c_char;
  let mut pcStack26: *mut c_char;
  let mut pcStack22: *mut c_char;
  let mut local_a: [u8;0x8] = [0;0x8];

  uVar1 = (param_1 + 0x122);
  pass1_1008_5784(CONCAT22(0x1050,local_a),(uVar1 + 0xa));
  loop {
    lVar7 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    uVar5 = (lVar7 >> 0x10);
    uVar2 = lVar7;
    uVar6 = uVar5 | uVar2;
    if (lVar7 == 0) {
      return 0x0;
    }
    uVar3 = uVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(uVar2 + 0x4));
    pcStack22 = CONCAT22(uVar6,uVar3);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(uVar2 + 0x8));
    pcStack26 = CONCAT22(uVar6,uVar3);
    pcVar8 = pass1_1038_4d28(pcStack22);
    pcVar9 = pass1_1038_4d28(pcStack26);
    iVar4 = pass1_1000_3d7a(param_3,pcVar8);
    if ((iVar4 == 0) && (iVar4 = pass1_1000_3d7a(param_2,pcVar9), iVar4 == 0)) break;
    iVar4 = pass1_1000_3d7a(param_2,pcVar8);
    if ((iVar4 == 0) && (iVar4 = pass1_1000_3d7a(param_3,pcVar9), iVar4 == 0)) {
      return lVar7;
    }
  }
  return lVar7;
}



pub unsafe fn pass1_1018_46e6(param_1: *mut u16,param_2: u8) -> *mut u16

{
  pass1_1018_33b4(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub unsafe fn struct_1018_4720(param_1: *mut astruct_203,mut param_2: u32,mut param_3: u32)

{
  let mut iVar1: *mut astruct_203;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field0_0x0 = 0x389a;
  iVar1.field1_0x2 = 0x1008;
  iVar1.field2_0x4 = param_3;
  iVar1.field4_0x8 = param_2;
  iVar1.field6_0xc = 0;
  param_1.field0_0x0 = &PTR_LOOP_1050_4aa6;
  iVar1.field1_0x2 = 0x1018;
  return;
}
pub unsafe fn pass1_1018_4760(param_1: *mut StructD)

{
  let mut iVar2: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar2 = param_1;
  param_1.address_offset_field_0x0 = &PTR_LOOP_1050_4aa6;
  iVar2.address_offset_field_0x2 = 0x1018;
  fn_ptr_1000_17ce(*&iVar2.hfile_0x4);
  param_1.address_offset_field_0x0 = 0x389a;
  iVar2.address_offset_field_0x2 = 0x1008;
  return;
}



astruct_203 * struct_1018_4790(param_1: *mut astruct_203,mut param_2: u32,mut param_3: u32,mut param_4: u16 )

{
  let mut iVar1: *mut astruct_203;
  let mut uVar1: u16;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field7_0xe = param_4;
  param_1.field0_0x0 = 0x4a92;
  iVar1.field1_0x2 = 0x1018;
  iVar1.field6_0xc = 0x1;
  return param_1;
}
pub unsafe fn struct_1018_47c8(param_1: *mut astruct_203,mut param_2: u32,mut param_3: u32,mut param_4: u16 ,mut param_5: u32)

{
  let mut iVar1: *mut astruct_203;
  let mut uVar1: u16;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field7_0xe = param_5;
  (iVar1 + 1).field0_0x0 = param_4;
  param_1.field0_0x0 = &PTR_LOOP_1050_4a9a;
  iVar1.field1_0x2 = 0x1018;
  iVar1.field6_0xc = 0x2;
  return;
}
pub unsafe fn pass1_1018_4808(param_1: *mut astruct_203,mut param_2: u32,mut param_3: u32,mut param_4: u32)

{
  let mut iVar1: *mut astruct_203;
  let mut uVar1: u16;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field7_0xe = param_4;
  param_1.field0_0x0 = &PTR_LOOP_1050_4aa2;
  iVar1.field1_0x2 = 0x1018;
  iVar1.field6_0xc = 0x3;
  return;
}



astruct_203 * struct_1018_4842(param_1: *mut astruct_203,mut param_2: u32,mut param_3: u32,mut param_4: u16 )

{
  let mut iVar1: *mut astruct_203;
  let mut uVar1: u16;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field7_0xe = param_4;
  (&iVar1.field7_0xe + 0x2) = 0;
  param_1.field0_0x0 = &PTR_LOOP_1050_4a8e;
  iVar1.field1_0x2 = 0x1018;
  iVar1.field6_0xc = 0x4;
  return param_1;
}
pub unsafe fn pass1_1018_4882(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = &PTR_LOOP_1050_4a8e;
  (param_1 + 0x2) = 0x1018;
  fn_ptr_1000_17ce(*(param_1 + 0x10));
  pass1_1018_4760(param_1);
  return;
}



astruct_203 * struct_1018_48b0(param_1: *mut astruct_203,mut param_2: u32,mut param_3: u32,mut param_4: u16 )

{
  let mut iVar1: *mut astruct_203;
  let mut uVar1: u16;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field7_0xe = param_4;
  param_1.field0_0x0 = &PTR_LOOP_1050_4a96;
  iVar1.field1_0x2 = 0x1018;
  iVar1.field6_0xc = 0x5;
  return param_1;
}



pub unsafe fn struct_1018_48e8(param_1: *mut astruct_203,mut param_2: u32,mut param_3: u32,mut param_4: u16 ) -> *mut u16

{
  let mut iVar1: *mut astruct_203;
  let mut uVar1: u16;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field7_0xe = param_4;
  param_1.field0_0x0 = 0x4a9e;
  iVar1.field1_0x2 = 0x1018;
  iVar1.field6_0xc = 0x6;
  return &param_1.field0_0x0;
}
pub unsafe fn struct_1018_4920(param_1: *mut astruct_203,mut param_2: u32,mut param_3: u32,mut param_4: u32)

{
  let mut iVar1: *mut astruct_203;
  let mut uVar1: u16;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field7_0xe = param_4;
    // just 0x4a8a
  param_1.field0_0x0 = &PTR_LOOP_1050_4a8a;
  iVar1.field1_0x2 = 0x1018;
  iVar1.field6_0xc = 0x7;
  return;
}



pub unsafe fn pass1_1018_495a(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1018_4760(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn pass1_1018_4980(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1018_4760(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn pass1_1018_49a6(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1018_4760(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn pass1_1018_49cc(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1018_4760(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn pass1_1018_49f2(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1018_4882(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn pass1_1018_4a18(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1018_4760(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn pass1_1018_4a3e(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1018_4760(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub unsafe fn pass1_1018_4a64(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1018_4760(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1018_4aaa(param_1: *mut u8,param_2: *mut astruct_19,mut param_3: u16 )

{
  struct_op_1018_4cda(param_2,param_3);
  param_2.offset_0x0 = 0x4b06;
  (param_2 + 0x2) = 0x1018;
  pass1_1018_4dce(param_1,param_2,0x9a);
  _PTR_LOOP_1050_4230 = param_2;
  return;
}



struct * pass1_1018_4ae0(param_1: *mut StructD,param_2: u8)

{
  clenaup_win_ui_1018_4d22(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return (struct *)param_1;
}



pub unsafe fn get_sys_metrics_1018_4b1e(param_1: *mut astruct_19,mut param_2: u16 ,mut param_3: u16 ) -> *mut u16

{
  pstruct19_1: *mut astruct_19;
  pstruct19_param_1_hi: *mut astruct_19;

  struct_op_1010_1d48(param_1,param_3);
  pstruct19_param_1_hi = (param_1 >> 0x10);
  pstruct19_1 = param_1;
  pstruct19_1.field9_0x12 = param_2;
  pstruct19_1.field10_0x14 = 0;
    // 0x4c9e val
  param_1.offset_0x0 = &PTR_LOOP_1050_4c9e;
  pstruct19_1.segment_0x2 = 0x1018;
  if (G_SM_CYCAPTION_1050_416c == 0) {
    G_SM_CYCAPTION_1050_416c = GetSystemMetrics16(SM_CYCAPTION);
    G_SM_CXBORDER_1050_416e = GetSystemMetrics16(SM_CXBORDER);
    G_SM_CYBORDER_1050_4170 = GetSystemMetrics16(SM_CYBORDER);
  }
  return &param_1.offset_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1018_4b78(param_1: *mut astruct_19)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  let mut paVar3: *mut Struct57;
  let mut unaff_SI: u16;
  let mut puVar4: *mut u32;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;

  paVar3 = (in_EDX & 0xffff0000 | param_1);
  pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24((param_1 + 0xa))),NULL,0x8);
  pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x18)),NULL,0x8);
  puVar4 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,CONCAT22(unaff_SI,0x2),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  puVar5 = pass1_1010_5f7a(puVar4,(puVar4 >> 0x10),0x0,(param_1 + 0x12));
  uVar2 = (puVar5 >> 0x10);
  if ((uVar2 | puVar5) != 0) {
    (param_1 + 0xa) = *puVar5;
    (param_1 + 0xe) = (puVar5 + 0x4);
  }
  ppcVar1 = (param_1 + 0x20);
  (**ppcVar1)(0x1010,param_1);
  if (((param_1 + 0xe) == 0) && ((param_1 + 0x10) == 0)) {
    (param_1 + 0xa) = (param_1 + 0x18);
    (param_1 + 0xc) = (param_1 + 0x1a);
  }
  (param_1 + 0xe) = (param_1 + 0x1c);
  (param_1 + 0x10) = (param_1 + 0x1e);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1018_4c2c(mut param_1: u32,param_2: *mut u32,mut param_3: u16 )

{
  let mut in_EDX: u32;
  let mut unaff_SI: u16;
  let mut puVar1: *mut u32;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffcc: u16;

  (param_1 + 0xa) = *param_2;
  (param_1 + 0xe) = param_2[0x1];
  puVar1 = mixed_1010_20ba((in_EDX & 0xffff0000 | param_1),_u16_1050_0ed0,
                           CONCAT22(unaff_SI,0x2),in_stack_0000fe9e,in_stack_0000ffc2,in_stack_0000ffc8,
                           in_stack_0000ffcc);
  pass1_1010_5fb0(puVar1,0x0,(param_1 + 0xa),param_1,(param_1 + 0x12));
  return;
}



pub unsafe fn pass1_1018_4c78(param_1: *mut u16,param_2: u8) -> *mut u16

{
  pass1_1010_1d80(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub unsafe fn struct_op_1018_4cda(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0;
  (param_1 + 0xe) = 0;
  (param_1 + 0x12) = 0;
  (param_1 + 0x14) = 0;
  (param_1 + 0x16) = 0;
  (param_1 + 0x18) = 0x1;
  (param_1 + 0x1a) = 0;
  param_1.offset_0x0 = s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
  (param_1 + 0x2) = 0x1018;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn clenaup_win_ui_1018_4d22(in_struct_1: *mut StructD)

{
  let mut obj: HPALETTE16;
  let mut local_struct_1: *mut StructD;
  let mut uVar4: *mut StructD;
  let mut uVar3: u16;
  let mut unaff_SS: u16;
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut puVar2: *mut u32;
  let mut puVar1: *mut u32;
  let mut fn_ptr_1: *mut *mut code;

  uVar3 = 0x1018;
  uVar4 = (in_struct_1 >> 0x10);
  local_struct_1 = in_struct_1;
    // just 0x5058
  in_struct_1.address_offset_field_0x0 = s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
  local_struct_1.address_offset_field_0x2 = 0x1018;
  if (local_struct_1.field11_0x12 != 0) {
    obj = SelectPalette16(0x0,local_struct_1.field14_0x1a,local_struct_1.field11_0x12);
    DeleteObject16(obj);
    uVar3 = SUB42(s_tile2_bmp_1050_1538,0x0);
    DeleteDC16(local_struct_1.field11_0x12);
  }
  puVar1 = local_struct_1.field6_0xa;
  uVar1 = local_struct_1.field7_0xc;
  if ((uVar1 | puVar1) != 0) {
    fn_ptr_1 = *puVar1;
    (**fn_ptr_1)(uVar3,puVar1,uVar1,1);
  }
  puVar2 = local_struct_1.field8_0xe;
  uVar2 = &local_struct_1.field_0x10;
  if ((uVar2 | puVar2) != 0) {
    fn_ptr_1 = *puVar2;
    (**fn_ptr_1)(uVar3,puVar2,uVar2,1);
  }
  _PTR_LOOP_1050_4230 = 0;
  pass1_1010_1d80(in_struct_1);
  return;
}
pub unsafe fn get_dc_1018_4db0(param_1: *mut astruct_126,mut param_2: u16 )

{
  let mut HVar1: HDC16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  (param_1 + 0x16) = param_2;
  HVar1 = GetDC16(param_2);
  *(param_1 + 0x14) = HVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1018_4dce(param_1: *mut u8,param_2: *mut astruct_19,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe8a: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffe0: u32;
  u8 **ppuVar4;

  ppuVar4 = CONCAT22((in_stack_0000ffe0 >> 0x10),0x48);
  puVar3 = mixed_1010_20ba(CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,ppuVar4,in_stack_0000fe8a
                           ,in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
  uVar2 = (puVar3 >> 0x10);
  ppcVar1 = (param_2 + 0x10);
  (**ppcVar1)(0x1010,param_2,param_3,(puVar3 + 0xc),(puVar3 + 0xa),
              (ppuVar4 >> 0x10));
  return;
}
pub unsafe fn create_dc_1018_4e04(in_string_6: u16,in_string_5: u16,param_3: *mut astruct_8,mut param_4: u16 ,mut param_5: i16,mut param_6: i16)

{
  let mut HVar1: HDC16;
  pHVar2: *mut HDC16;
  let mut iVar4: *mut astruct_8;
  let mut uVar3: u16;
  let mut devmodea_init_data: *mut DEVMODEA;
  let mut iStack16: i16;
  let mut fn_ptr_1: *mut *mut code;

  uVar3 = (param_3 >> 0x10);
  iVar4 = param_3;
  fn_ptr_1 = (param_3 + 0x14);
  (**fn_ptr_1)();
  devmodea_init_data = pass1_1008_4772(iVar4.field10_0xa);
  pass1_1008_43cc(iVar4.field10_0xa);
  HVar1 = CreateDC16(devmodea_init_data,NULL,NULL,s_dib_1050_4234);
  iVar4.field15_0x12 = HVar1;
  pHVar2 = &iVar4.field15_0x12;
  fn_ptr_1 = (iVar4.field10_0xa + 0x8);
  (**fn_ptr_1)();
  iVar4.field22_0x1a = pHVar2;
  if ((DAT_1050_422e != 0) && (0x280 < param_6)) {
    for (iStack16 = 0; iStack16 < DAT_1050_4216 + 1; iStack16 += 1) {
      (&u16_1050_4172 + iStack16 * 0x2) =
           (((&u16_1050_4172 + iStack16 * 0x2) * (param_6 + 1)) / 0x280);
    }
    for (iStack16 = 0; iStack16 < DAT_1050_422c + 1; iStack16 += 1) {
      (&u16_1050_419a + iStack16 * 0x2) =
           (((&u16_1050_419a + iStack16 * 0x2) * (param_5 + 1)) / 0x1e0);
    }
  }
  DAT_1050_422e = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn unk_win_ui_op_1018_4f18(mut param_1: u32,param_2: *mut astruct_57,param_3: *mut astruct_39,mut param_4: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut paVar3: *mut astruct_394;
  rect: *mut RECT16;
  let mut iVar4: i16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut paVar9: *mut Struct57;
  let mut iVar6: *mut astruct_39;
  let mut uVar10: u16;
  let mut uVar11: u8;
  let mut unaff_CS: u16;
  let mut HVar12: HWND16;
  let mut local_12: RECT16;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut uStack10: u32;
  let mut puStack6: *mut u32;
  let mut uVar5: u32;
  let mut paVar8: *mut Struct57;

  uVar2 = FUN_1010_830a(param_1,param_2,unaff_CS,_u16_1050_14cc,param_4);
  uVar5 = uVar2;
  uVar6 = SUB42(param_2,0x0);
  puStack6 = CONCAT22(uVar6,uVar2);
  ppcVar1 = (*puStack6 + 0x14);
  (**ppcVar1)();
  paVar3 = uVar5;
  uStack10 = uVar5 & 0xffff | param_2 << 0x10;
  uVar10 = (param_3 >> 0x10);
  iVar6 = param_3;
  if (&iVar6.field12_0xe != 0) {
    uVar7 = iVar6.field13_0x10;
    paVar3 = &iVar6.field12_0xe;
    param_2 = (param_2 & 0xffff0000 | (uVar7 | paVar3));
    if ((uVar7 | paVar3) != 0) {
      ppcVar1 = paVar3;
      (**ppcVar1)(0x1010,paVar3,uVar7,0x1,uVar2,uVar6);
    }
  }
  mem_op_1000_179c(0x14,param_2);
  uVar7 = param_2 | paVar3;
  paVar9 = (param_2 & 0xffff0000);
  paVar8 = (paVar9 | uVar7);
  if (uVar7 == 0) {
    paVar3 = NULL;
  }
  else {
    struct_1008_4c58(paVar3);
    paVar9 = paVar8;
  }
  iVar6.field12_0xe = paVar3;
  iVar6.field13_0x10 = paVar9;
  pass1_1008_4d84(paVar9,&iVar6.field12_0xe,uStack10);
  rect = &local_12;
  HVar12 = HWND16_1050_0396;
  GetClientRect16(rect,&DAT_1050_1050);
  uVar11 = 0;
  mem_op_1000_179c(0x1e,paVar9);
  uVar7 = paVar9 | rect;
  if (uVar7 == 0) {
    iVar6.field10_0xa = 0;
  }
  else {
    iVar4 = (iStack12 - local_12.y) + 1;
    uVar11 = 0x8;
    pass1_1008_405c(CONCAT22(paVar9,rect),&iVar6.field12_0xe,iVar4,
                    (iStack14 - local_12.x) + 1);
    iVar6.field10_0xa = iVar4;
    iVar6.field11_0xc = uVar7;
  }
  if (puStack6.is_null() == false) {
    ppcVar1 = *puStack6;
    (**ppcVar1)(uVar11,puStack6,(puStack6 >> 0x10),0x1,HVar12,uVar2,uVar6,puStack6,puStack6);
  }
  return;
}
