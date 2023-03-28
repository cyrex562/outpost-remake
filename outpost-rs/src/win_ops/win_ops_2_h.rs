//
// Created by cyrex on 2/17/2023.
//

#ifndef OUTPOST_1_SRC_WIN_OPS_WIN_OPS_2_H_
#define OUTPOST_1_SRC_WIN_OPS_WIN_OPS_2_H_
// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "structs/structs_2xx/struct_283.h"


void  send_msg_1038_ed8a(param_1: u16, param_2: u32, param_3: u32, param_4: HWND16);




void  post_win_msg_1040_0d5e(param_1: u16, param_2: u16, param_3: i16, param_4: HWND16);




void  unk_win_sys_op_1038_da68(param_1: i16, param_2: u16, param_3: u16, param_4: u32, WNDCLASS16 *param_5, u8 *param_6);




void  post_win_msg_1038_dcb0(param_1: u32, param_2: u16, param_3: *mut u8, param_4: u16);




void  destroy_win_1038_e1dc(param_1: u16, param_2: u16, param_3: i16, param_4: HWND16);




void  pass1_1038_e4bc(param_1: u16, param_2: u32, param_3: u32, param_4: *mut u8, param_5: i16, param_6: u16);




long  call_win_proc_1038_d020(win_handle_1: HWND16, param_2: u32, LPARAM l_param, param_4: u16, win_handle_2: HWND16);




void  win_prop_op_1038_d118(param_1: u32, param_2: u32, param_3: u16, param_4: u16, param_5: HWND16);




void  post_win_msg_1038_d840(param_1: *mut Struct70, param_2: u16, param_3: HWND16);




LRESULT  send_msg_1038_c228(param_1: u32, param_2: HWND16);




void  send_msg_1038_c374(param_1: u32, param_2: *mut u32, param_3: u16, param_4: HWND16);




void  destroy_win_1038_a3d2(param_1: u32, param_2: HWND16);




void  send_dlg_item_msg_1038_8d22(param_1: u32, param_2: HWND16, param_3: u16);




LRESULT  pass1_1038_8d7e(param_1: u32, param_2: u16);




void  win_msg_op_1038_95fc(param_1: u32, param_2: u16);




void  win_ui_op_1038_977a(param_1: i16, param_2: u16, param_3: i16, param_4: *mut u8, param_5: HWND16, param_6: u16);




long  unk_win_ui_op_1038_9820(param_1: *mut Struct51, param_2: i16, param_3: i16, param_4: HWND16, BOOL16 param_5);




void  win_ui_dlg_op_1038_98b4(param_1: *mut Struct51, param_2: HWND16, BOOL16 param_3);




void  pass1_1038_362e(param_1: u32);




void  pass1_1038_095e(param_1: u16, param_2: u16, param_3: i16, param_4: u32, param_5: *mut u8, param_6: i16, param_7: u16);




u16  pass1_1030_e67c(param_1: u32, param_2: *mut u8, param_3: i16, param_4: u16);




void  pass1_1030_838e(param_1: *mut u32, param_2: u16, param_3: u8);




void  pass1_1030_83ba(u32 **param_1, long param_2, param_3: u16, param_4: u8);




void  send_msg_1028_e242(param_1: *mut u32, param_2: i16, param_3: HWND16);




void  pass1_1028_9a02(param_1: u32, param_2: i16, param_3: u16, param_4: u16, i16 param_5);




void  pass1_1028_a188(param_1: u16, param_2: u16, param_3: i16, param_4: i16, param_5: u32, param_6: u16);




void  pass1_1028_86c2(param_1: u32, param_2: *mut u8, param_3: i16, param_4: u16);




void  pass1_1028_9114(param_1: u32, param_2: *mut u8, param_3: i16, param_4: u16);



void  post_msg_1028_76da(void);




void  pass1_1028_6ff6(param_1: u32, param_2: u16, param_3: i16, param_4: u16);



void  win_1020_75f0(param_1: *mut Struct283, param_2: u16);



void  window_op_1020_76aa(param_1: *mut Struct0);




void  post_win_msg_1020_79fc(param_1: *mut Struct69, param_2: u16, param_3: u16, param_4: i16, param_5: HWND16);


void window_op_1020_6c3a(globals: &mut Globals, param_1: *mut Struct0);



void  post_win_msg_1020_7308(param_1: u32, param_2: u16, param_3: HWND16);



u16  post_msg_1020_55b0(param_1: u32, param_2: u16);



void  post_msg_1020_4394(param_1: u32, param_2: u16, param_3: HWND16);



void  win_1020_43f6(param_1: *mut Struct0, param_2: *mut u8, param_3: u16);



void  struct_1020_3644(u16 *param_1, param_2: u16, param_3: u32, param_4: u16);



BOOL16  post_win_msg_1020_1ca4(param_1: u32);



void  destroy_window_1020_1d4a(param_1: u32, param_2: i16, param_3: HWND16);



void  post_win_msg_1020_291a(param_1: HWND16);



u32  send_msg_1020_29d8(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16, param_6: u16, param_7: u16);



void  send_win_msg_1020_08fe(param_1: *mut Struct63, param_2: HWND16);




void  send_msg_1020_097e(param_1: u32, param_2: HWND16);



void  win_1020_09ba(param_1: *mut Struct0, param_2: u16, param_3: *mut u8, param_4: u16);




void  pass1_1020_0a0c(param_1: u32);




Struct63 * pass1_1020_0ae8(param_1: *mut Struct63, param_2: u8, param_3: u16);



void  pass1_1020_0dc4(u16 *param_1, param_2: u16, param_3: u32, param_4: u16);



void  win_help_op_1020_0ec4(param_1: *mut u32, param_2: u16, param_3: u16);




#endif // OUTPOST_1_SRC_WIN_OPS_WIN_OPS_2_H_
