use crate::globals::PTR_LOOP_1050_1040;
use crate::gui::cleanup::unk_destroy_win_op_1010_2fa0;
use crate::gui::get_sys_metrics_1040_7728;
use crate::gui::window::ui_op_1010_79aa;
use crate::no_refs::j::pass1_1010_7d38;
use crate::no_refs::l::pass1_1010_ad64;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::fn_ptr_1000_17ce;
use crate::unk::block_1010_1000::pass1_1010_1ea6;
use crate::unk::block_1010_2000::{mixed_1010_20ba, pass1_1010_2ee2};
use crate::unk::block_1040_7000::pass1_1040_79c0;
use crate::unk::block_1040_a000::pass1_1040_a5d0;
use crate::utils::CONCAT22;
use crate::winapp::winapp_a::unk_win_op_1010_7300;
use crate::winapp::winapp_b::post_win_msg_1040_7b3c;

pub fn struct_1040_b082(param_1: *mut Struct57, mut param_2: u32)

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,param_2,(param_2 >> 0x10));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 1).field0_0x0 = 0;
  iVar1[0x1].field1_0x2 = 0;
  param_1.field0_0x0 = 0xb772;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}
pub fn pass1_1040_b0bc(param_1: *mut Struct57,mut param_2: u32,mut param_3: u32)

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: u16;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,param_3,(param_3 >> 0x10));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 1).field0_0x0 = 0;
  iVar1[0x1].field1_0x2 = param_2;
  param_1.field0_0x0 = 0xb772;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return;
}


pub fn pass1_1040_b316(param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut uStack4: u16;

  if (param_5 == 0xf) {
    ppcVar1 = (*param_1 + 0x60);
    uStack4 = (**ppcVar1)();
  }
  else if (param_5 == 0x111) {
    ppcVar1 = (*param_1 + 0x10);
    (**ppcVar1)();
    uStack4 = 0x1;
  }
  else {
    uStack4 = pass1_1040_79c0(param_1,param_2,param_3,param_4,param_5);
  }
  return uStack4;
}


pub fn pass1_1040_b54a(param_1: *mut u8,param_2: *mut Struct903,mut param_3: u16 ,mut param_4: u32)

{
  let mut pSVar1: *mut StructD;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut iVar5: i16;
  let mut pSVar6: *mut StructD;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u32;
  let mut iVar6: *mut astruct_515;
  let mut uVar10: u16;
  let mut puVar11: *mut u32;
  let mut paVar12: *mut Struct57;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut uVar13: u8;
  let mut uVar14: u8;
  let mut uVar15: u16;
  let mut uVar16: u16;
  let mut uVar17: u16;
  let mut in_stack_0000ffe6: u16;

  paVar8 = CONCAT22(in_register_0000000a,param_1);
  if (param_4 == 0xea) {
    ppcVar2 = (param_2 + 0x5c);
    (**ppcVar2)();
  }
  else if (param_4 == 0xeb) {
    puVar11 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x3),in_stack_0000fe8e,
                              in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
    uVar7 = (puVar11 >> 0x10);
    pSVar1 = (param_2 + 0x90);
    if (pSVar1.is_null() == false) {
      uVar10 = (pSVar1 >> 0x10);
      uVar15 = 0x1010;
      pSVar6 = pSVar1;
      pass1_1010_ad64(pSVar1,uVar7,puVar11,CONCAT22((pSVar1 + 0xa),uVar7),
                      (pSVar1 + 0x6));
      (param_2 + 0x90) = pSVar6;
      (param_2 + 0x92) = uVar7;
      if ((uVar7 | (param_2 + 0x90)) == 0) {
        (param_2 + 0x90) = pSVar1;
      }
      else {
        if (pSVar1.is_null() == false) {
          pass1_1040_a5d0(pSVar1);
          uVar15 = 0x1000;
          fn_ptr_1000_17ce(pSVar1);
        }
        ppcVar2 = (param_2 + 0x70);
        (**ppcVar2)(uVar15,param_2);
      }
    }
  }
  else {
    if (param_4 == 0x1790) {
      paVar12 =
                mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x32),in_stack_0000fe8e,
                                in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
      uVar9 = paVar8 & 0xffff0000 | paVar12 >> 0x10;
      uVar3 = (param_2 + 0x90);
      uVar3 = (uVar3 + 0x6);
      iVar4 = pass1_1010_7d38(paVar12,(paVar12 >> 0x10),uVar3,(uVar3 >> 0x10)
                             );
      iVar5 = iVar4;
      ui_op_1010_79aa(paVar12,0xfab,0x0);
      if (iVar5 != 0) {
        return;
      }
      if (iVar4 == 0) {
        uVar3 = (param_2 + 0x90);
        uVar10 = (uVar3 >> 0x10);
        iVar6 = uVar3;
        uVar3 = iVar6.field6_0x6;
        uVar16 = uVar3;
        uVar17 = (uVar3 >> 0x10);
        uVar15 = 0x14;
      }
      else {
        uVar3 = (param_2 + 0x90);
        uVar10 = (uVar3 >> 0x10);
        iVar6 = uVar3;
        uVar3 = iVar6.field6_0x6;
        uVar16 = uVar3;
        uVar17 = (uVar3 >> 0x10);
        uVar15 = 0x9;
      }
      uVar13 = uVar10;
      uVar14 = (uVar10 >> 0x8);
    }
    else if (param_4 == 0x1824) {
      paVar12 =
                mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x32),in_stack_0000fe8e,
                                in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
      uVar9 = paVar8 & 0xffff0000 | paVar12 >> 0x10;
      iVar6 = paVar12;
      uVar3 = (param_2 + 0x90);
      ui_op_1010_79aa(paVar12,0xfc5,(uVar3 + 0x6));
      if (iVar6.is_null() == false) {
        return;
      }
      uVar3 = (param_2 + 0x90);
      uVar3 = (uVar3 + 0x6);
      uVar16 = uVar3;
      uVar17 = (uVar3 >> 0x10);
      uVar15 = 0x12;
      uVar13 = 0;
      uVar14 = 0;
    }
    else {
      if (param_4 != 0x1830) {
        post_win_msg_1040_7b3c(param_2,param_3,param_4,param_4);
        return;
      }
      paVar12 =
                mixed_1010_20ba(paVar8,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x32),in_stack_0000fe8e,
                                in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
      uVar9 = paVar8 & 0xffff0000 | paVar12 >> 0x10;
      iVar6 = paVar12;
      uVar3 = (param_2 + 0x90);
      ui_op_1010_79aa(paVar12,0xfb6,(uVar3 + 0x6));
      if (iVar6.is_null() == false) {
        return;
      }
      uVar3 = (param_2 + 0x90);
      uVar3 = (uVar3 + 0x6);
      uVar16 = uVar3;
      uVar17 = (uVar3 >> 0x10);
      uVar15 = 0xc;
      uVar13 = 0;
      uVar14 = 0;
    }
    unk_win_op_1010_7300(uVar9,paVar12,CONCAT13(uVar14,CONCAT12(uVar13,iVar6)),uVar15,CONCAT22(uVar17,uVar16));
  }
  return;
}


pub fn pass1_1040_b7ee(param_1: *mut Struct57,param_2: i32,mut param_3: u16 )

{
  let mut iVar1: *mut Struct57;
  let mut uVar1: *mut Struct57;
  let mut uVar2: u16;

  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_3,0xfab));
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1[0x1].field3_0x6 = 0;
  iVar1[0x1].field5_0xa = 0;
  iVar1[0x1].field18_0x22 = 0;
  iVar1[0x1].field21_0x26 = 0;
  iVar1[0x1].field_0x28 = 0;
  param_1.field0_0x0 = 0xbeba;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  if (param_2 != 0) {
    uVar2 = (param_2 >> 0x10);
    iVar1[0x1].field18_0x22 = (param_2 + 0x6);
    iVar1[0x1].field21_0x26 = (param_2 + 0x14);
  }
  return;
}













pub fn struct_1040_bf3e(param_1: *mut astruct_442,mut param_2: u16 ) -> *mut u16

{
  let mut iVar1: *mut astruct_442;
  let mut uVar1: *mut astruct_442;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field0_0x0 = 0x389a;
  iVar1.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x3aa8;
  iVar1.field1_0x2 = 0x1008;
  iVar1.field2_0x4 = param_2;
  param_1.field0_0x0 = 0x3ab0;
  iVar1.field1_0x2 = 0x1008;
  iVar1.field3_0x6 = 0;
  param_1.field0_0x0 = 0xc53e;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  return &param_1.field0_0x0;
}
pub fn pass1_1040_bf92(param_1: *mut StructD)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.address_offset_field_0x0 = 0xc53e;
  iVar1.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  pass1_1010_1ea6(&iVar1.field_0x6,(param_1 & 0xffff | uVar1 << 0x10));
  unk_destroy_win_op_1010_2fa0(&iVar1.field_0x6);
  param_1.address_offset_field_0x0 = 0x3ab0;
  iVar1.address_offset_field_0x2 = 0x1008;
  param_1.address_offset_field_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}
pub fn pass1_1040_bfde(param_1: *mut c_void,param_2: *mut u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  (iVar3 + 0x6) = param_2;
  ppcVar1 = (*param_2 + 0x4);
  (**ppcVar1)();
  uVar2 = (iVar3 + 0x6);
  (uVar2 + 0x22) = (iVar3 + 0x4);
  pass1_1010_2ee2((iVar3 + 0x6));
  return;
}
