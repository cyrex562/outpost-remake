use std::os::raw::c_char;
use crate::globals::PTR_LOOP_1050_1040;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_903::Struct903;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::unk::block_1000_3000::pass1_1000_3cea;
use crate::unk::block_1040_b000::pass1_1040_b0bc;
use crate::utils::CONCAT22;
use crate::winapi16::MessageBox16;

pub fn find_n_load_rsrc_1010_4e9e(struct_param_1: *mut astruct_812) {
    let mut BVar1: bool;
    let mut h_rsrc: HRSRC16;
    let mut handle: HGLOBAL16;
    let mut struct_1: *mut astruct_812;
    let mut uVar3: u16;
    let mut uVar1: u32;
    let mut uVar2: u32;

    uVar3 = (struct_param_1 >> 0x10);
    struct_1 = struct_param_1;
    if (struct_1.field29_0x20 != 0) {
        if (struct_1.hglobal_0x2a != 0) {
            BVar1 = GlobalUnlock16(struct_1.hglobal_0x2a);
            if (BVar1 == 0) {
                FreeResource16(struct_1.hglobal_0x2a);
            }
        }
        uVar1 = struct_1.field18_0x12;
        uVar2 = (uVar1 + 0x4);
        h_rsrc = FindResource16(
            0xa,
            ((uVar2 + struct_1.field29_0x20 * 0x2) * 0x2 + 0x1384),
            HINSTANCE16_1050_038c,
        );
        handle = LoadResource16(h_rsrc, HINSTANCE16_1050_038c);
        struct_1.hglobal_0x2a = handle;
        if (handle != 0) {
            WIN16_LockResource16(handle);
            return;
        }
    }
    return;
}

pub fn free_rsrc_1010_4b3e(param_1: *mut StructD) {
    let mut puVar3: *mut u32;
    let mut uVar5: u32;
    let mut BVar6: bool;
    let mut pstructd_1: *mut StructD;
    let mut iVar7: *mut astruct_818;
    let mut pstructd_1_hi: *mut StructD;
    let mut uVar4: u16;
    let mut unaff_CS: u16;
    let mut iStack4: i16;
    let mut puVar2: *mut u32;
    let mut piVar1: *mut i16;
    let mut uVar1: u16;
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut fn_ptr_1: *mut *mut code;

    pstructd_1_hi = (param_1 >> 0x10);
    pstructd_1 = param_1;
    // really just 0x5024
    param_1.address_offset_field_0x0 = s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
    pstructd_1.address_offset_field_0x2 = 0x1010;
    if (pstructd_1.field_0x2a != 0) {
        unaff_CS = SUB42(0x1538, 0x0);
        BVar6 = GlobalUnlock16(&pstructd_1.field_0x2a);
        if (BVar6 == 0) {
            unaff_CS = SUB42(0x1538, 0x0);
            FreeResource16(&pstructd_1.field_0x2a);
        }
    }
    pstructd_1.field_0x2a = 0;
    if (&pstructd_1.field11_0x12 != 0) {
        iStack4 = 0;
        loop {
            puVar3 = &pstructd_1.field11_0x12;
            piVar1 = (puVar3 + 0x8);
            if (*piVar1 == iStack4 || *piVar1 < iStack4) {
                break;
            }
            uVar4 = (*puVar3 >> 0x10);
            iVar7 = *puVar3;
            puVar2 = (iVar7 + iStack4 * 0x4);
            uVar1 = (iVar7 + iStack4 * 0x4 + 2);
            if ((uVar1 | puVar2) != 0) {
                fn_ptr_1 = *puVar2;
                (**fn_ptr_1)(unaff_CS, puVar2, uVar1, 1);
            }
            iStack4 += 0x1;
        }
    }
    uVar5 = &pstructd_1.field11_0x12;
    fn_ptr_1000_17ce(*(uVar5 + 0x4));
    fn_ptr_1000_17ce(*&pstructd_1.field11_0x12);
    puVar1 = (&pstructd_1.field12_0x14 + 2);
    uVar2 = pstructd_1.field13_0x18;
    if ((uVar2 | puVar1) != 0) {
        fn_ptr_1 = *puVar1;
        (**fn_ptr_1)(0x1000, puVar1, uVar2, 1);
    }
    fn_ptr_1000_17ce(*&pstructd_1.field14_0x1a);
    pass1_1010_1d80(param_1);
    return;
}

pub fn load_icon_1040_8b92(param_1: *mut Struct57)

{
  let mut bVar1: u8;
  let mut HVar2: HICON16;
  let mut uVar3: u16;
  let mut uVar4: u16;

  uVar3 = (param_1 >> 0x10);
  bVar1 = *(param_1 + 0x98) & 0xf0;
  if (bVar1 == 0x30) {
    uVar4 = 0x7f03;
  }
  else if ((bVar1 == 0x10) || (bVar1 == 0x10)) {
    uVar4 = 0x7f01;
  }
  else if ((bVar1 == 0x40) || (bVar1 == 0x40)) {
    uVar4 = 0x7f04;
  }
  else {
    if (bVar1 != 0x20) {
      return;
    }
    uVar4 = 0x7f02;
  }
  HVar2 = LoadIcon16(uVar4,0x0);
  (param_1 + 0x8e) = HVar2;
  return;
}

pub fn load_string_1010_847e(mut param_1: u32, mut param_2: u16) -> *mut c_char {
    LoadString16(
        0x3ff,
        (param_1 & 0xffff0000 | (param_1 + 0x682)),
        param_2,
        HINSTANCE16_1050_038c,
    );
    return (param_1 & 0xffff0000 | (param_1 + 0x682));
}

pub fn load_string_1010_84ac(mut param_1: i16, param_2: INT16, mut param_3: u16) {
    let mut uVar1: u16;

    uVar1 = param_2;
    LoadString16(
        0x3ff,
        CONCAT22(param_2, param_1 + 0x682),
        param_3,
        HINSTANCE16_1050_038c,
    );
    str_op_1008_60e8(uVar1, CONCAT22(param_2, param_1 + 0x682));
    return;
}

pub fn load_string_1010_84e0(
    mut param_1: u16,
    mut param_2: u16,
    in_resc_id_3: u16,
    in_buffer_4: *mut c_char,
    in_buf_len_5: i16,
) {
    let mut in_stack_0000000e: u16;

    LoadString16(
        in_resc_id_3,
        CONCAT22(in_buf_len_5, in_buffer_4),
        in_stack_0000000e,
        HINSTANCE16_1050_038c,
    );
    return;
}


pub fn msg_box_op_1038_81be(param_1: *mut c_char, mut param_2: u16, param_3: *mut Struct903)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u16;
  let mut msg_box_title: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, msg_box_title, 0x1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  // uVar2 = (param_3 >> 0x10);
  MessageBox16(0x0, CONCAT22(0x1050, msg_box_title), CONCAT22(in_buf_len_5, param_1),
               (param_3 + 0x6));
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0, CONCAT22(0x1050, msg_box_title), CONCAT22(in_buf_len_5, param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}


pub fn msg_box_op_1038_8dda(param_1: *mut c_char, mut param_2: u16, param_3: *mut Struct903)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uVar2: u16;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,0x1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  uVar2 = (param_3 >> 0x10);
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}


pub fn msg_box_ui_op_1040_64ca(param_1: *mut c_char, mut param_2: u16, mut param_3: u32)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,0x1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}


pub fn msg_box_op_1040_a85a(param_1: *mut c_char, mut param_2: u16, mut param_3: u32)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_206,0x1050);
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),CONCAT22(0x1050,local_104));
  MessageBox16(0x0,CONCAT22(0x1050,local_206),CONCAT22(in_buf_len_5,param_1),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}


pub fn msg_box_ui_op_1038_8a3a(param_1: *mut c_char, mut param_2: u16, param_3: *mut Struct903, mut param_4: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut local_20a: [u8;0x102] = [0;0x102];
  let mut pcStack264: *mut c_char;
  let mut sStack262: i16;
  let mut local_104: [u8;0x102] = [0;0x102];

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  sStack262 = paVar1;
  pcStack264 = param_1;
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,param_1,sStack262);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(sStack262,pcStack264),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_104,0x1050);
  pass1_1000_3cea(CONCAT22(sStack262,pcStack264),CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x101,local_20a,0x1050);
  MessageBox16(0x0,CONCAT22(0x1050,local_20a),CONCAT22(sStack262,pcStack264),
               (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(sStack262,pcStack264));
  return;
}


pub fn pas1_1040_29c2(mut param_1: u16, mut param_2: u16, param_3: *mut Struct57, mut param_4: u32, mut param_5: u16) -> *mut Struct57

{
    let mut iVar1: *mut Struct57;
    let mut uVar1: *mut Struct57;

    pass1_1040_b0bc(param_3, param_4, CONCAT22(param_5, 0x157));
    uVar1 = (param_3 >> 0x10);
    iVar1 = param_3;
    param_3.field0_0x0 = 0x2e26;
    iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
    load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x64b);
    iVar1[0x1].field3_0x6 = param_1;
    iVar1[0x1].field4_0x8 = param_2;
    load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x64a);
    iVar1[0x1].field5_0xa = param_1;
    iVar1[0x1].field6_0xc = param_2;
    return param_3;
}
