use crate::windef16::HWND16;











pub unsafe fn pass1_1038_a122(param_1: *mut Struct57,param_2: *mut Struct57,mut param_3: u16 ,mut param_4: u32,mut param_5: u32) -> *mut Struct57

{
  get_sys_metrics_1040_7728
            (CONCAT22(param_2,param_1),param_3,param_4,param_5,(param_5 >> 0x10));
  (param_1 + 1).field0_0x0 = 0;
  (CONCAT22(param_2,param_1)).field0_0x0 = 0xa2d0;
  param_1.field1_0x2 = &u16_1050_1038;
  return CONCAT22(param_2,param_1);
}
pub unsafe fn pass1_1038_a156(param_1: *mut StructD)

{
  param_1.address_offset_field_0x0 = 0xa2d0;
  (param_1 + 0x2) = &u16_1050_1038;
  ui_cleanup_op_1040_782c(param_1);
  return;
}


pub unsafe fn pass1_1038_a33c(param_1: *mut u16,mut param_2: u16 ) -> *mut u16

{
  let mut paVar1: *mut Struct57;

  paVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfc7));
  *param_1 = 0xa428;
  (param_1).field1_0x2 = &u16_1050_1038;
  return param_1;
}




pub unsafe fn pass1_1038_a36a(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xa428;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}













pub unsafe fn pass1_1038_a494(param_1: *mut u16,mut param_2: u16 ) -> *mut u16

{
  let mut paVar1: *mut Struct57;

  paVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfc8));
  *param_1 = 0xa62e;
  (param_1).field1_0x2 = &u16_1050_1038;
  return param_1;
}




pub unsafe fn pass1_1038_a4c2(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xa62e;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}











pub unsafe fn pass1_1038_a69a(param_1: *mut u16,mut param_2: u16 ) -> *mut u16

{
  let mut paVar1: *mut Struct57;

  paVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfc9));
  *param_1 = 0xa832;
  (param_1).field1_0x2 = &u16_1050_1038;
  return param_1;
}




pub unsafe fn pass1_1038_a6c8(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xa832;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}











pub unsafe fn pass1_1038_a89e(param_1: *mut u16,mut param_2: u16 ) -> *mut u16

{
  let mut paVar1: *mut Struct57;

  paVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfca));
  *param_1 = 0xab16;
  (param_1).field1_0x2 = &u16_1050_1038;
  return param_1;
}




pub unsafe fn pass1_1038_a8cc(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xab16;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}











pub unsafe fn pass1_1038_ab82(param_1: *mut Struct57,mut param_2: u16 ) -> *mut Struct57

{
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfd3,param_2);
  param_1.field0_0x0 = 0xad72;
  (param_1 + 0x2) = &u16_1050_1038;
  return param_1;
}




pub unsafe fn pass1_1038_abb0(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xad72;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}


// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar5






pub unsafe fn pass1_1038_adde(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32) -> *mut u16

{
  pass1_1038_9b72(param_1,param_2,param_3,param_4);
  CONCAT22(param_2,param_1) = 0xae4e;
  (param_1 + 0x2) = &u16_1050_1038;
  return CONCAT22(param_2,param_1);
}
pub unsafe fn pass1_1038_ae08(param_1: *mut StructD)

{
  let mut in_stack_0000ffda: u16;

  param_1.address_offset_field_0x0 = 0xae4e;
  (param_1 + 0x2) = &u16_1050_1038;
  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  return;
}







pub unsafe fn pass1_1038_aeca(param_1: *mut StructD) -> *mut StructD

{
  let mut uVar1: u16;
  let mut addr_offset_b6: u16;
  let mut seg_addr_180: u16;
  let mut u8_array_5c: [u8;0x5a] = [0;0x5a];

  uVar1 = (param_1 >> 0x10);
  (param_1 + 0xac) = 0;
  (param_1 + 0xae) = 0;
  if (_PTR_LOOP_1050_5b7c.is_null()) {
    _PTR_LOOP_1050_5b7c = param_1;
  }
  pass1_1000_4906(param_1,NULL,0xac);
  unk_draw_op_1008_80ee(CONCAT22(0x1050,u8_array_5c));
  unk_win_ui_op_1040_9854(CONCAT22(0x1050,&addr_offset_b6));
  addr_offset_b6 = 0x389a;
  seg_addr_180 = 0x1008;
  pass1_1008_8168(CONCAT22(0x1050,u8_array_5c));
  return param_1;
}




pub unsafe fn pass1_1038_af34()

{
  _PTR_LOOP_1050_5b7c = 0;
  return;
}



pub unsafe fn pass1_1038_af40(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: i16) -> u32

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut puVar3: *mut u8;
  let mut uVar4: u16;
  let mut puVar5: *mut u8;
  let mut pSVar6: *mut StructD;
  let mut in_register_0000000a: u16;
  let mut pSVar7: *mut StructD;
  let mut uVar8: u32;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut paVar12: *mut Struct57;
  let mut puVar13: *mut u16;
  let mut in_stack_0000fe74: u16;
  let mut in_stack_0000fe86: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd4: u32;
  let mut in_stack_0000ffde: u16;

  paVar12 = CONCAT22(in_register_0000000a,param_2);
  puVar3 = bring_win_to_top_1038_b72e(param_3,param_5);
  iVar9 = param_3;
  uVar10 = (param_3 >> 0x10);
//  if (puVar3.is_null() == false) goto LAB_1038_b61f;
  uVar11 = SUB42(&u16_1050_1038,0x0);
  PTR_LOOP_1050_5b82 = puVar3;
  match param_5 {
  0x1 =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
    if ((paVar12 | param_1) == 0) {//
// LAB_1038_afa0:
      uVar11 = 0x1000;
      param_1 = 0;
      pSVar6 = null_mut();
    }
    else {
      paVar12 = pass1_1038_9f76(CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
      pSVar6 = (paVar12 >> 0x10);
      param_1 = paVar12;
    }}
    // break;
  0x2 =>{
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_181c(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);}
    // break;
  0x3 =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    puVar5 = (paVar12 | param_1);
//    if (puVar5.is_null()) goto LAB_1038_afa0;
    paVar12 = pass1_1038_e99a(puVar5,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x4 =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    puVar5 = (paVar12 | param_1);
//    if (puVar5.is_null()) goto LAB_1038_afa0;
    paVar12 = pass1_1038_c7b8(puVar5,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x5 =>{
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_23ea(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4,in_stack_0000ffd4);}
    // break;
  0x6 =>{
    mem_op_1000_179c(0x92,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar12 = pass1_1040_06e8(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x7 =>{
    mem_op_1000_179c(0x9c,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_4068(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);}
    // break;
  0x8 =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_b772(pSVar6,CONCAT22(paVar12,param_1),param_4);}
    // break;
  0x9 =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
//    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_e140(CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0xa =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
//    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a33c(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (puVar13 >> 0x10);
    param_1 = puVar13;}
    // break;
  0xb =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
//    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a494(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (puVar13 >> 0x10);
    param_1 = puVar13;}
    // break;
  0xc =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
//    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a69a(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (puVar13 >> 0x10);
    param_1 = puVar13;}
    // break;
  0xd =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
//    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a89e(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (puVar13 >> 0x10);
    param_1 = puVar13;}
    // break;
  0xe =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x94,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_e69a(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);}
    // break;
  0xf =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x94,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_cd06(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);}
    // break;
  0x10 =>{
    mem_op_1000_179c(0x92,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar12 = pass1_1040_0bfc(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x11 =>{
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_0e1c(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,param_4);}
    // break;
  0x12 =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    paVar12 = pass1_1038_d756(pSVar6,CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x13 =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    puVar5 = (paVar12 | param_1);
//    if (puVar5.is_null()) goto LAB_1038_afa0;
    paVar12 = pass1_1038_cad8(puVar5,CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x14 =>{
    mem_op_1000_179c(0xaa,paVar12);
    uVar4 = paVar12 | param_1;
    uVar8 = paVar12 & 0xffff0000 | uVar4;
//    if (uVar4 == 0) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_1f5a(CONCAT22(paVar12,param_1),param_4,uVar8);
    pSVar6 = uVar8;}
    // break;
  0x15 =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
//    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_d242(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x16 =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_eeda(pSVar6,CONCAT22(paVar12,param_1),param_4);}
    // break;
  0x17 =>{
    mem_op_1000_179c(0x96,paVar12);
//    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    uVar11 = 0x1018;
    paVar12 = pass1_1018_5e26(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  _ => {}
// TODO: goto switchD_1038_b581_caseD_18;
  0x19 =>{
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_1cb4(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);}
    // break;
  0x1a =>{
    mem_op_1000_179c(0x92,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar12 = pass1_1040_123e(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x1b =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
//    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_ab82(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x1c =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
//    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_e2d0(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x1d =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
//    if ((paVar12 | param_1) == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_eb9e(CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x1e =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x29e,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_bddc(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);}
    // break;
  0x1f =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_c4a2(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);}
    // break;
  0x20 =>{
    mem_op_1000_179c(0x29a,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_2ea2(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);}
    // break;
  0x21 =>{
    mem_op_1000_179c(0xa6,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_3966(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);}
    // break;
  0x22 =>{
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_34a2(pSVar6,CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);}
    // break;
  0x23 =>{
    mem_op_1000_179c(0x9c,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_ac84(pSVar6,CONCAT22(paVar12,param_1),param_4);}
    // break;
  0x25 =>{
    mem_op_1000_179c(0xa0,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_ca16(pSVar6,CONCAT22(paVar12,param_1),param_4);}
    // break;
  0x26 =>{
    mem_op_1000_179c(0xa2,paVar12);
    uVar4 = paVar12 | param_1;
    pSVar7 = (paVar12 & 0xffff0000 | uVar4);
//    if (uVar4 == 0) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_d0f8(CONCAT22(paVar12,param_1),param_4,in_stack_0000ffde,pSVar7,in_stack_0000fe74,
                    in_stack_0000fe86,in_stack_0000ff98,in_stack_0000ff9e,in_stack_0000ffa2,in_stack_0000ffaa,
                    in_stack_0000ffb0,in_stack_0000ffb4,in_stack_0000ffcc);
    pSVar6 = pSVar7;}
    // break;
  0x27 =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0xa0,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    pass1_1038_88f2(CONCAT22(paVar12,param_1),param_4);}
    // break;
  0x28 =>{
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (paVar12 | param_1);
//    if (pSVar6.is_null()) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_6402(pSVar6,CONCAT22(paVar12,param_1),param_4);}
    // break;
  0x29 =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x98,paVar12);
    uVar4 = paVar12 | param_1;
//    if (uVar4 == 0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_7d10(uVar4,CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
    // break;
  0x2a =>{
    uVar11 = 0x1000;
    mem_op_1000_179c(0x98,paVar12);
    puVar5 = (paVar12 | param_1);
//    if (puVar5.is_null()) goto LAB_1038_afa0;
    paVar12 = pass1_1038_8caa(puVar5,CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (paVar12 >> 0x10);
    param_1 = paVar12;}
  };
  (param_5 * 0x4 + iVar9) = param_1;
  (param_5 * 0x4 + iVar9 + 0x2) = pSVar6;
// switchD_1038_b581_caseD_18:
  if ((param_5 * 0x4 + iVar9) != 0) {
    if ((iVar9 + 0xae) != 0) {
      uVar2 = (param_5 * 0x4 + iVar9);
      (uVar2 + 0x6e) = (iVar9 + 0xae);
    }
    (iVar9 + 0xae) = 0;
    uVar2 = (param_5 * 0x4 + iVar9);
    ppcVar1 = ((param_5 * 0x4 + iVar9) + 0x8);
    (**ppcVar1)(uVar11,uVar2,(uVar2 >> 0x10));
  }//
// LAB_1038_b61f:
  return CONCAT22((param_5 * 0x4 + iVar9 + 0x2),(param_5 * 0x4 + iVar9));
}
