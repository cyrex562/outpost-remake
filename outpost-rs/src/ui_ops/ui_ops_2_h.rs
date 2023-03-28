//
// Created by cyrex on 2/17/2023.
//

#ifndef OUTPOST_1_SRC_UI_OPS_UI_OPS_2_H_
#define OUTPOST_1_SRC_UI_OPS_UI_OPS_2_H_


// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "structs/structs_0xx/struct_1.h"
// #include "structs/structs_0xx/struct_18.h"

void  win_ui_op_1040_5800(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16);




void  message_box_op_1040_37f0(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16, param_6: u16);




void  pass1_1040_39e2(param_1: *mut Struct18);




void  show_win_1040_3ae8(param_1: *mut Struct1, param_2: u16);




void  win_ui_op_1040_3b1e(param_1: *mut Struct2, WORD *param_2);




void  unk_win_ui_op_1040_3c64(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16);




void  dialog_item_ui_op_1040_3e08(in_struct_1: *mut Struct2, param_2: u16);




void  pass1_1040_40e2(param_1: *mut Struct18);




void  win_ui_op_1040_410e(param_1: *mut Struct1, param_2: u16, u8 *param_3);




void  win_ui_op_1040_42b2(param_1: u32, param_2: i16, param_3: HWND16, WORD *param_4);




void  pass1_1040_477e(param_1: *mut Struct1, param_2: *mut u8, param_3: u16, param_4: u16);




void  set_win_pos_1040_4ae4(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16);




void  pass1_1040_2464(param_1: *mut Struct18);




void  show_win_1040_2490(param_1: *mut Struct1, param_2: HWND16);




u32  win_ui_op_1040_2512(param_1: *mut u32, param_2: u32, param_3: u16, param_4: HWND16, WNDCLASS16 *param_5, u8 *param_6);




void  dlg_ui_op_1040_2a64(param_1: *mut Struct1, param_2: u16, param_3: u16);




void  win_ui_op_1040_2bb2(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: HWND16);




void  win_dlg_item_1040_2d48(param_1: u32, param_2: HWND16, BOOL16 param_3);




void  pass1_1040_2f06(param_1: *mut Struct18);




void  show_win_1040_2f5a(param_1: *mut Struct1, param_2: HWND16);




void  win_dlg_op_1040_2f90(param_1: u32, WORD *param_2);




void  win_ui_op_1040_311a(param_1: i16, param_2: u16, param_3: u16, param_4: u32);




void  enable_win_1040_32a8(param_1: u32);




BOOL16  set_win_pos_1040_331a(param_1: u32, param_2: u16, param_3: i16, param_4: HWND16);




void  pass1_1040_3506(param_1: *mut Struct18);




void  show_win_1040_355a(param_1: *mut Struct1, param_2: HWND16);




void  set_win_text_1040_3590(param_1: u32, WORD *param_2);




void  pass1_1040_0e86(param_1: *mut Struct18, param_2: u16);




void  set_win_pos_1040_0f10(param_1: HWND16, param_2: u16, i16 param_3);




void  pass1_1040_1290(param_1: *mut Struct18);




void  win_ui_op_1040_12bc(param_1: *mut Struct1, param_2: u16, u8 *param_3);




void  win_msg_op_1040_13b2(param_1: u32, param_2: i16, param_3: HWND16, param_4: u16);




u32  set_win_pos_1040_162a(param_1: u16, param_2: u32, param_3: u32, param_4: u16, param_5: HWND16);




void  pass1_1040_1876(param_1: *mut Struct18);




void  show_win_1040_18a2(param_1: *mut Struct1, param_2: HWND16, WORD *param_3);




void  unk_win_ui_op_1040_19ea(param_1: *mut Struct32, param_2: i16, param_3: HWND16);




u32  pass1_1040_1ab0(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16, param_6: u16);




void  check_dialog_btn_1040_1afe(param_1: u32);




void  check_dialog_btn_1040_1b8a(void);




void  pass1_1040_1d24(param_1: *mut Struct18);




void  show_win_1040_1d50(param_1: *mut Struct1, param_2: HWND16);




void  unk_win_ui_op_1040_1d7a(param_1: *mut Struct33, param_2: i16, param_3: HWND16);




void  pass1_1040_205e(param_1: *mut Struct18);


void create_win_1040_20d4(globals: &mut Globals,
                          param_1: u16,
                          param_2: u16,
                          param_3: u16,
                          Struct1        *param_4);


void pass1_1038_ebd6(globals: &mut Globals, param_1: *mut Struct18);




void  win_ui_op_1038_ec1a(param_1: u16, i16 param_2);




#endif // OUTPOST_1_SRC_UI_OPS_UI_OPS_2_H_
