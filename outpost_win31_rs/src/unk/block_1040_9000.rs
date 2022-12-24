use crate::{draw_ops, program_lifecycle, winapp};
use crate::draw_ops::draw_a;
use crate::windef16::RECT16;

pub unsafe fn pass1_1040_9252(param_1: *mut Struct65)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut iVar4: i16;
  let mut iVar3: *mut Struct65;
  let mut uVar5: *mut Struct65;

  uVar5 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (&iVar3.field2_0x4 != 0) {
    draw_a::draw_text_1040_9650((param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
  }
  if (&iVar3.field4_0x8 != 0) {
    pass1_1040_9618((param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
  }
  uVar2 = iVar3[0x1].field6_0xc;
  if (iVar3.field16_0x22 < uVar2) {
    iVar3.field16_0x22 = uVar2;
  }
  uVar2 = iVar3[0x1].field7_0xe;
  if (iVar3.field17_0x24 < uVar2) {
    iVar3.field17_0x24 = uVar2;
  }
  iVar4 = (iVar3 + 1).field0_0x0 + iVar3[0x1].field2_0x4;
  if (iVar3.field16_0x22 < iVar4) {
    iVar3.field16_0x22 = iVar4;
  }
  iVar4 = iVar3[0x1].field1_0x2 + iVar3[0x1].field3_0x6;
  if (iVar3.field17_0x24 < iVar4) {
    iVar3.field17_0x24 = iVar4;
  }
  piVar1 = &iVar3.field16_0x22;
  *piVar1 = *piVar1 + iVar3[0x1].field8_0x10;
  piVar1 = &iVar3.field17_0x24;
  *piVar1 = *piVar1 + iVar3[0x1].field8_0x10;
  return;
}

pub unsafe fn pass1_1040_9618(param_1: *mut Struct65)

{
  let mut uVar1: u16;
  let mut iVar2: *mut Struct65;
  let mut uVar2: u16;
  let mut uVar3: u32;

  uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar3 = pass1_1008_4772(&iVar2.field4_0x8);
  uVar1 = (uVar3 >> 0x10);
  iVar2[0x1].field2_0x4 = (uVar3 + 0x4);
  iVar2[0x1].field3_0x6 = (uVar3 + 0x8);
  return;
}


pub unsafe fn pass1_1040_97da(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  winapp::mix_win_ui_op_1040_911e(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub unsafe fn pass1_1040_9824(param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0;
  (iVar1 + 0x4) = 0;
  (iVar1 + 0x56) = 0;
  (iVar1 + 0x5a) = 0;
  (iVar1 + 0x5c) = 0;
  *(iVar1 + 0x6) = 0;
  return;
}
