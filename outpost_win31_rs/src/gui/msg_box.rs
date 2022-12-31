use std::os::raw::c_char;
use crate::resources::{load_string_1010_847e, load_string_1010_84e0};
use crate::structs::struct_57::Struct57;
use crate::structs::struct_903::Struct903;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::unk::block_1000_3000::{pass1_1000_3cea, unk_str_op_1000_3d3e};
use crate::utils::CONCAT22;
use crate::winapi16::{WinAPI16_MessageBox16, PostMessage16};

pub fn msg_box_op_1040_d3d0(param_1: *mut c_char, mut param_2: u16, mut param_3: u32)

{
    let mut in_buf_len_5: i16;
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut uVar2: u16;
    let mut local_206: [u8; 0x102] = [0; 0x102];
    let mut local_104: [u8; 0x102] = [0; 0x102];

    paVar1 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x1000, paVar1);
    in_buf_len_5 = paVar1;
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x100, local_206, 0x1050);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, param_1, in_buf_len_5);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, 0x1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, 0x1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, 0x1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, 0x1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, 0x1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    uVar2 = (param_3 >> 0x10);
    WinAPI16_MessageBox16(0x0, CONCAT22(0x1050, local_206), CONCAT22(in_buf_len_5, param_1),
                          (param_3 + 0x6));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, param_1, in_buf_len_5);
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, 0x1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, 0x1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, 0x1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, 0x1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    load_string_1010_84e0(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), 0x3ff, local_104, 0x1050);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, param_1), CONCAT22(0x1050, local_104));
    WinAPI16_MessageBox16(0x0, CONCAT22(0x1050, local_206), CONCAT22(in_buf_len_5, param_1),
                          (param_3 + 0x6));
    fn_ptr_1000_17ce(CONCAT22(in_buf_len_5, param_1));
    return;
}


pub fn msg_box_op_1040_cce4(param_1: *mut c_char, mut param_2: u16, param_3: *mut Struct903)

{
  let mut in_buf_len_5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut uStack522: u32;
  let mut local_206: [u8;0x102] = [0;0x102];
  let mut local_104: [u8;0x102] = [0;0x102];
  let mut uVar2: u16;
  let mut uVar3: u16;

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
  WinAPI16_MessageBox16(0x0, CONCAT22(0x1050, local_206), CONCAT22(in_buf_len_5, param_1),
                        (param_3 + 0x6));
  fn_ptr_1000_17ce(CONCAT22(in_buf_len_5,param_1));
  return;
}


pub fn msg_box_op_1010_8bb4(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut title_string: *mut c_char;
    let mut message: [u8; 0x400] = [0; 0x400];

    title_string = load_string_1010_847e(_u16_1050_14cc, 0x3fa);
    unk_str_op_1000_3d3e(message, title_string);
    pass1_1000_3cea( param_3, message);
    title_string = load_string_1010_847e(_u16_1050_14cc, 0x57b);
    WinAPI16_MessageBox16(
        0x1010,
        title_string,
        message,
        HWND16_1050_0396,
    );
    PostMessage16(0x0, 0xee, 0x111, HWND16_1050_0396);
    return;
}
