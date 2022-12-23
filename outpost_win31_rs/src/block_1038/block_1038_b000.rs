use std::ffi::c_void;
use crate::block_1010::block_1010_2000::mixed_1010_20ba;
use crate::block_1040::block_1040_7000::get_sys_metrics_1040_7728;
use crate::structs::struct_57::Struct57;
use crate::utils::CONCAT22;
use crate::winbase::ShowWindow16;

pub unsafe fn pass1_1038_b6e0(mut param_1: u16, mut param_2: u16)

{
  let mut var1: u16;
  let mut var2: u16;
  let mut var4: u16;

  var4 = 0x1;
  loop {
    if 0x2a < var4 {
      return;
    }
    var2 = param_1;
      var1 = var4 * 0x4 + var2;
    if (((var4 * 0x4 + var2 + 0x2) | (var4 * 0x4 + var2)) != 0) &&
       ((var1 + 0x6) == param_2) { break; }
    var4 += 0x1;
  }
  (var4 * 0x4 + var2) = 0;
  return;
}



pub unsafe fn pass1_1038_b772(param_1: *mut u8,param_2: *mut Struct57,mut param_3: u16 )

{
  let mut var1: u16;
  let mut var2: *mut Struct57;
  let mut var3: *mut Struct57;
  // let mut unaff_BP: u16;
  let mut var4: *mut Struct57;
  let mut var5: *mut u32;
  let mut var6: u16;
  let mut var7: u16;
  let mut var8: u16;
  let mut var9: u16;
  let mut var10: *mut *mut u8;

  var2 = CONCAT22(var1, param_1);
  get_sys_metrics_1040_7728(param_2,0x9a,0x0,0xfbf,param_3);
  var4 = (param_2 >> 0x10);
  // var3 = param_2;
  let mut var3: *mut astruct_57 = param_2 + 1;
    var3.field0_0x0 = 0;
  var3.field4_0x8 = 0x1;
  var3.field5_0xa = 0;
  param_2.field0_0x0 = 0xbd70;
  param_2.field1_0x2 = 0x1038;
  var10 = CONCAT22(unaff_BP, 0x36);
  var5 = mixed_1010_20ba(var2, _u16_1050_0ed0, var10, var6, var7, var8,
                         var9);
    (param_2 + 1).field0_0x0 = var5;
  param_2.field1_0x2 = (var5 >> 0x10);
  var5 = mixed_1010_20ba((var2 & 0xffff0000 | var5 >> 0x10), _u16_1050_0ed0,
                         CONCAT22((var10 >> 0x10), 0x6), var6, var7
                         , var8, var9);
  param_2.field2_0x4 = var5;
  param_2.field3_0x6 = (var5 >> 0x10);
  return;
}




pub unsafe fn pass1_1038_b7f0(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xbd70;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



pub unsafe fn pass1_1038_bca8(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut uVar4: u32;
  let mut paVar5: *mut astruct_394;
  let mut puVar6: *mut u32;
  let mut uVar7: u16;
  let mut puVar8: *mut u8;
  let mut in_EDX: u32;
  let mut paVar9: *mut Struct57;
  let mut iVar10: i16;
  let mut iVar11: i16;
  let mut uVar12: u16;
  let mut uVar13: u16;

  uVar12 = (param_1 >> 0x10);
  iVar10 = param_1;
  uVar3 = (iVar10 + 0x8e);
  uVar13 = (uVar3 >> 0x10);
  iVar11 = uVar3;
  puVar6 = (iVar11 + 0xa);
  paVar9 = (in_EDX & 0xffff0000 | (iVar11 + 0xc));
  ppcVar2 = (*puVar6 + 0x14);
  (**ppcVar2)();
  paVar5 = puVar6;
  uVar4 = paVar9 << 0x10;
  if ((iVar10 + 0x70) != 0) {
    paVar5 = (iVar10 + 0x70);
    uVar1 = (iVar10 + 0x72);
    uVar7 = uVar1 | paVar5;
    paVar9 = (paVar9 & 0xffff0000 | uVar7);
    if (uVar7 != 0) {
      ppcVar2 = paVar5;
      (**ppcVar2)();
    }
  }
  mem_op_1000_179c(0x14,paVar9);
  puVar8 = (paVar9 | paVar5);
  if (puVar8.is_null()) {
    paVar5 = null_mut();
    puVar8 = null_mut();
  }
  else {
    struct_1008_4c58(paVar5);
  }
  (iVar10 + 0x70) = paVar5;
  (iVar10 + 0x72) = puVar8;
  pass1_1008_4d84(puVar8,(iVar10 + 0x70),puVar6 & 0xffff | uVar4);
  return;
}



pub unsafe fn pass1_1038_bd4a(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1038_b7f0(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}




pub unsafe fn pass1_1038_bddc(param_1: *mut StructD,param_2: *mut Struct57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar1: *mut Struct57;
  let mut unaff_BP: u16;
  let mut uVar1: *mut Struct57;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x176,param_6);
  uVar1 = (param_2 >> 0x10);
  iVar1 = param_2;
  (iVar1 + 1) = 0;
  iVar1[0x1].field2_0x4 = 0;
  iVar1[0x1].field3_0x6 = 0;
  iVar1[0x1].field4_0x8 = 0;
  iVar1[0x1].field5_0xa = 0;
  iVar1[0x1].field6_0xc = 0;
  iVar1[0x1].field7_0xe = 0;
  param_2.field0_0x0 = 0xc436;
  iVar1.field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,CONCAT22(unaff_BP,0x3b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 1).field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
  return;
}




pub unsafe fn pass1_1038_be4a(param_1: *mut StructD)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0xc436;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
