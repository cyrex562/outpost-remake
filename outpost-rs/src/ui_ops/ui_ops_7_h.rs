//
// Created by cyrex on 2/17/2023.
//

#ifndef OUTPOST_1_SRC_UI_OPS_UI_OPS_7_H_
#define OUTPOST_1_SRC_UI_OPS_UI_OPS_7_H_

// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "structs/structs_0xx/structs_7x.h"

// #include <stdbool.h>

void  cursor_op_1008_2dcc(param_1: i16, param_2: u16, param_3: u16, HINSTANCE16 in_hinstance);




void  win_ui_cursor_op_1008_2e9a(Struct72 **param_1, param_2: u16);



void  pass1_1008_3018(param_1: u32, param_2: *mut u8, param_3: i16, param_4: u16);




void  menu_ui_op_1008_09ba(param_1: u32, param_2: HWND16, RECT16 *param_3, HWND16 param_4);




void  switchD_1008_1091_caseD_a7(void);




void  switchD_1008_1091_caseD_aa(void);




void  switchD_1008_1091_caseD_ac(void);




void  switchD_1008_1091_caseD_ad(void);




void  switchD_1008_1091_caseD_ae(void);




void  switchD_1008_1091_caseD_b1(void);




void  switchD_1008_1091_caseD_b3(void);




void  draw_op_1008_1230(HWND16 param_1);


void message_box_op_1008_12dc(
  param_1: u32, param_2: u32, u16             hinst_arg3, param_4: u16, Globals *globals);




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1008_1414(Globals   *globals,
                         Struct72 **param_1,
                         u32        param_2,
                         LPCSTR     param_3,
                         u16        param_4,
                         u8         param_5,
                         u16        param_6);



void  cleanup_ui_op_1008_0618(u16 *param_1);


void win_ui_cursor_op_1008_06c0(globals: &mut Globals, param_1: *mut u32, param_2: u32, param_3: u16, i16 param_4);


BOOL16 msg_box_op_1000_1f24(globals: &mut Globals, param_1: i16, param_2: u16, param_3: u16, param_4: u16);




BOOL16  pass1_1000_1f7e(u16 *param_1, param_2: u16);



BOOL16  msg_box_op_1000_214c(param_1: u16, param_2: i16, param_3: u16, param_4: u16, param_5: u16);




bool  mem_op_1000_21b6(param_1: u16, param_2: u16);




#endif // OUTPOST_1_SRC_UI_OPS_UI_OPS_7_H_
