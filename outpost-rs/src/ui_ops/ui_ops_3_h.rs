//
// Created by cyrex on 2/17/2023.
//

#ifndef OUTPOST_1_SRC_UI_OPS_UI_OPS_3_H_
#define OUTPOST_1_SRC_UI_OPS_UI_OPS_3_H_


// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"

void  destroy_win_1038_ef3a(param_1: *mut Struct31, param_2: HWND16);



void  win_ui_op_1040_0000(param_1: *mut Struct1, param_2: *mut u8, param_3: HWND16);


void win_ui_op_1040_0170(Globals           *globals,
                         short              param_1,
                         param_2: u16,
                         param_3: u16,
                         param_4: u32,
                         param_5: u16,
                         WNDCLASS16 *param_6);



LRESULT  win_ui_op_1040_0558(param_1: u32, param_2: i16, param_3: HWND16);



void enable_win_1040_060e(param_1: u32, param_2: i16, param_3: HWND16, param_4: u16);



void  pass1_1040_073a(param_1: *mut Struct18);



void  show_win_1040_0766(param_1: *mut Struct1, param_2: u16);



void  win_ui_op_1040_07dc(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16, param_6: u16, param_7: HWND16, param_8: u16);



void  enable_window_1040_0acc(param_1: u16, param_2: u16, BOOL16 param_3, param_4: HWND16);



void  pass1_1040_0c54(param_1: *mut Struct18, param_2: u16);



void  show_win_1040_0c7c(param_1: *mut Struct1, param_2: u16, param_3: u16);



void  pass1_1038_e03e(param_1: u32);



void  pass1_1038_e16e(param_1: *mut Struct18);



void  check_radio_btn_show_win_1038_e19a(param_1: *mut Struct1);



void  pass1_1038_e308(param_1: *mut Struct18);



void  win_ui_op_1038_e348(param_1: *mut Struct1);



void  pass1_1038_e6f0(param_1: *mut Struct18);



void  unk_win_ui_op_1038_e71c(param_1: *mut Struct1, param_2: u16);



void  chk_is_dlg_btn_checked_1038_e7a0(param_1: *mut Struct62, i16 param_2);



void  pass1_1038_e9ec(param_1: *mut Struct18);



void  win_ui_op_1038_ea18(param_1: *mut Struct1);



void  win_ui_op_1038_eaa2(param_1: u32, param_2: i16, param_3: HWND16, WPARAM16 param_4);



void  win_dlg_op_1038_c95e(param_1: u32, i16 param_2);



void  pass1_1038_cb30(param_1: *mut Struct18);




void  show_win_1038_cb5c(param_1: *mut Struct1);




void  destroy_window_1038_cc00(param_1: i16, param_2: u16, param_3: u16, param_4: u32);




void  pass1_1038_cd5c(param_1: *mut Struct18);




void  destroy_window_1038_cd88(param_1: *mut Struct1);




void  check_dlg_btn_checked_1038_cdd6(param_1: *mut Struct61, param_2: i16, param_3: HWND16);




void  pass1_1038_d276(param_1: *mut Struct18);




void  win_ui_op_1038_d2a2(param_1: *mut Struct1);




void  unk_win_ui_op_1038_d400(param_1: u8, param_2: u16, param_3: u16, param_4: u32);




void  pass1_1038_d7d0(param_1: *mut Struct18, param_2: u16);


void window_op_1038_d8b2(globals: &mut Globals, param_1: i16, HINSTANCE16 param_2, param_3: u16);




void  show_win_1038_b634(param_1: u32, param_2: HWND16);




void  show_win_1038_b68a(param_1: u32, param_2: HWND16);




BOOL16  bring_win_to_top_1038_b72e(param_1: u32, param_2: i16, in_win_handle_3: HWND16);




void  pass1_1038_b7f0(param_1: *mut Struct18);




void  win_ui_op_1038_b81c(param_1: *mut Struct1);




u32  win_ui_op_1038_b922(param_1: *mut u32, param_2: u32, param_3: u16, param_4: u16, param_5: HWND16, WNDCLASS16 *param_6);


void win_ui_cursor_op_1038_bc30(globals: &mut Globals, param_1: u32, HINSTANCE16 param_2, param_3: u16);




void  pass1_1038_be4a(param_1: *mut Struct18);




void  pass1_1038_be76(param_1: u16, param_2: u32, param_3: *mut u8, param_4: i16, param_5: u16);




#endif // OUTPOST_1_SRC_UI_OPS_UI_OPS_3_H_
