//
// Created by cyrex on 2/17/2023.
//

#ifndef OUTPOST_1_SRC_UI_OPS_UI_OPS_4_H_
#define OUTPOST_1_SRC_UI_OPS_UI_OPS_4_H_


// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "structs/structs_0xx/struct_20.h"
// #include "structs/structs_0xx/struct_1.h"
// #include "structs/structs_0xx/structs_x.h"


void win_dlg_op_1038_bea4(Globals        *globals,
                          unsigned int    param_1,
                          unsigned short *param_2);


void show_win_1038_c044(globals: &mut Globals, param_1: *mut Struct1);




void  msg_box_op_1038_c07a(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16);




void  enable_win_1038_c294(param_1: u32);




BOOL16  set_win_pos_1038_c31a(param_1: u32, param_2: u16, param_3: i16, param_4: HWND16);




void  pass1_1038_c4fe(param_1: *mut Struct18);




void  pass1_1038_c52a(param_1: u16, param_2: u32, param_3: *mut u8, param_4: i16, param_5: u16);




void  show_win_1038_c558(param_1: *mut Struct1);




void  win_dlg_op_1038_c58e(param_1: u32, WORD *param_2);




void  message_box_op_1038_c672(param_1: i16, param_2: u16, param_3: u16, param_4: u32, short param_5);




void  pass1_1038_c80a(param_1: *mut Struct18);




void  destroy_window_1038_c836(param_1: i16, param_2: u32, param_3: u32, param_4: u16);




void  win_ui_op_1038_c89c(param_1: *mut Struct1);




void  enable_window_1038_9cec(param_1: i16, param_2: u16, param_3: u16, param_4: u16, param_5: i16, param_6: HWND16);




void  pass1_1038_9fa4(param_1: *mut Struct18);




void  show_win_1038_9fd0(param_1: *mut Struct1);




void  destroy_window_1038_a072(param_1: u16, param_2: u16, param_3: i16, param_4: HWND16);




void  pass1_1038_a156(param_1: *mut Struct18);




void  unk_win_ui_op_1038_a18c(param_1: *mut Struct1, param_2: u16);




void  show_win_1038_a396(param_1: *mut Struct1, param_2: u16, param_3: u16);




void  win_ui_op_1038_a4ee(param_1: *mut Struct1, param_2: u16);




void  win_ui_op_1038_a584(param_1: u16, param_2: i16, param_3: HWND16, param_4: u16);




void  win_ui_op_1038_a6f4(param_1: *mut Struct1);




void  win_ui_op_1038_a788(param_1: u32, param_2: i16, param_3: HWND16, param_4: u16);



void  enable_win_1038_a8f8(param_1: u16, param_2: u16, param_3: u16, TwoWords param_4, in_hwnd_5: HWND16);



void  win_ui_op_1038_a972(param_1: *mut Struct1);




void  win_sys_op_1038_a9fa(param_1: u32, i16 param_2);




void  pass1_1038_abb0(param_1: *mut Struct18);




void  set_win_pos_1038_abdc(param_1: HWND16);


struct Struct20 * pass1_1038_aeca(param_1: &mut Struct20, param_2: u16);




u16  pass1_1038_8966(param_1: u32, param_2: u16, param_3: u16, param_4: i16, param_5: HWND16);




void  pass1_1038_89e8(param_1: u32, param_2: u16);




void  pass1_1038_89f8(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: *mut u8, param_6: u16);




void  msg_box_ui_op_1038_8a3a(param_1: u32, char *param_2, param_3: *mut u8, param_4: u16);




void  unk_win_ui_op_1038_8afe(param_1: *mut Struct50, param_2: HWND16, BOOL16 param_3);




void  send_dlg_item_msg_1038_8b58(param_1: u32, param_2: u16);




void  pass1_1038_8d98(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: *mut u8, param_6: u16, param_7: u16);




void  msg_box_op_1038_8dda(param_1: u32, char *param_2, param_3: *mut u8, param_4: u16);




LRESULT  send_dlg_item_msg_1038_8f74(param_1: u32, param_2: HWND16, WORD *param_3);




void  win_dlg_op_1038_9294(param_1: *mut Struct1, param_2: u16);




BOOL16  send_dlg_item_int_1038_94da(param_1: i16, param_2: u16, param_3: u16, param_4: u16, param_5: i16, param_6: HWND16, BOOL16 param_7);




void  enable_win_1038_9a66(param_1: u16, param_2: u16, in_b_enable_3: u16, param_4: u32, in_hwnd_5: HWND16);




void  unk_win_ui_op_1038_9bc8(param_1: *mut Struct1);




void  destroy_window_1038_7d88(param_1: u32, param_2: u16);




LRESULT  pass1_1038_7dac(param_1: u32, param_2: u16);




void  pass1_1038_7dc6(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: *mut u8, param_6: u16, param_7: u16, param_8: u16);




LRESULT  send_dlg_item_msg_1038_7eac(param_1: u32);




void  send_dlg_item_msg_1038_7fae(param_1: u32);




void  enable_win_1038_806a(param_1: u32, param_2: HWND16);




u16  send_dlg_item_msg_1038_8164(param_1: u16, param_2: u16, param_3: *mut u8, param_4: u16, param_5: HWND16);




void  msg_box_op_1038_81be(param_1: u32, char *param_2, param_3: *mut u8, param_4: u16);




void  set_win_text_1038_8358(param_1: u32, param_2: HWND16, param_3: u16);




void  send_dlg_item_msg_1038_8400(param_1: u16, param_2: u16, param_3: u32, param_4: u16, param_5: u16);




LRESULT  send_dlg_item_msg_1038_844a(param_1: u32, param_2: HWND16, param_3: u16);




u16  send_dlg_item_msg_1038_8618(param_1: u32, param_2: u16);




u16  send_dlg_item_msg_1038_87b2(param_1: u32, param_2: u16, param_3: u16);




void  pass1_1038_8810(param_1: u32, param_2: u16, param_3: u16);



void  pass1_1020_de32(param_1: u32, param_2: u16, param_3: *mut u8, param_4: i16, param_5: u16);



Struct29 * pass1_1018_d198(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d1be(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d1e4(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d20a(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d230(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d256(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d27c(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d2a2(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d2c8(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d2ee(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d314(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d33a(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d360(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d386(param_1: *mut Struct29, param_2: u8);




Struct29 * pass1_1018_d3ac(param_1: *mut Struct29, param_2: u8);



void  pass1_1020_8bcc(param_1: *mut Struct285, param_2: u16);



void  invalidate_rect_1020_8d90(param_1: u32, param_2: u16, param_3: u32, param_4: u16, param_5: u16, param_6: u16);



void  invalidate_rect_1020_8fb4(param_1: u32, param_2: u16);



void  set_struct_op_1020_921c(param_1: *mut Struct7, param_2: u16);



void  pass1_1020_770e(param_1: u32);



void  cleanup_menu_ui_op_1020_795c(in_struct_1: *mut Struct3, HMENin_menu_handle_2: u16);



void  get_win_ui_info_op_1020_7a50(param_1: u32, param_2: HWND16);




void  win_ui_menu_op_1020_7ad2(param_1: u32, param_2: HWND16, RECT16 *param_3, param_4: HWND16);




Struct3 * pass1_1020_7b60(param_1: *mut Struct3, param_2: u8, param_3: u16);



void  destroy_window_1020_8250(param_1: u32, param_2: HWND16);




#endif // OUTPOST_1_SRC_UI_OPS_UI_OPS_4_H_
