//
// Created by cyrex on 2022-06-07.
//

#ifndef OUTPOST_C_PROJ__WIN_OPS_H_
#define OUTPOST_C_PROJ__WIN_OPS_H_

#include "types.h"
#include "structs_2.h"
#include "structs_3.h"

HWND16 create_window_1008_5e7e(void);

LRESULT make_def_win_proc_1008_5f44
                  (u16 param_1,u16 param_2,LPARAM param_3,WPARAM16 in_wparam_2,u16 param_5,HWND16 param_6);

void win_ui_fn_1020_6e98(u16 param_1,StructA *param_2);

void pass1_1018_2afa(u32 param_1);

void create_win_1040_20d4(param_1: u32,StructB *struct_b_param_2,u16 param_3);

void dialog_ui_fn_1040_78e2(StructB *in_struct_1);

void win_ui_op_1008_5cfe(Struct27 *param_1,char *param_2,WNDCLASS16 *in_wnd_class);

#endif //OUTPOST_C_PROJ__WIN_OPS_H_
