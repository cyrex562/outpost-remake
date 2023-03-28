//
// Created by cyrex on 2/17/2023.
//

#ifndef OUTPOST_1_SRC_UI_OPS_UI_OPS_6_H_
#define OUTPOST_1_SRC_UI_OPS_UI_OPS_6_H_

// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "structs/structs_0xx/struct_43.h"
// #include "structs/structs_0xx/struct_1.h"
// #include "structs/structs_0xx/structs_3x.h"


void destroy_window_1020_3b3e(param_1: *mut Struct30, param_2: HWND16);



void pass1_1020_3c8c(param_1: u32, param_2: u32, param_3: u16);



Struct3 *pass1_1020_3ca6(param_1: *mut Struct3, param_2: u8, param_3: u16);


void enable_window_1020_1bd4(globals: &mut Globals,
                             param_1: i16,
                             param_2: u16,
                             param_3: u16,
                             param_4: u32,
                            param_5: u16);



void set_win_tet_1020_1d2a(param_1: u16, param_2: u16, SEGPTR in_win_text_3, param_4: u16, in_dlg_id_5: u16, in_hwnd_6: HWND16);



void pass1_1020_1d8e(param_1: u32, param_2: u32);



BOOL16 destroy_win_1020_1dea(param_1: HWND16);




u16 destroy_win_1020_1e1e(param_1: HWND16);


Struct18 *pass1_1020_1e54(globals: &mut Globals,param_1: *mut Struct18, param_2: u8);


void pass1_1020_26a6(param_1: u32);



void pass1_1020_28fc(param_1: *mut Struct3, param_2: u16);



void pass1_1020_2a6a(param_1: u32, param_2: u16);



void bring_window_to_top_1020_2aae(param_1: u32, param_2: u32);



void pass1_1020_0aa6(param_1: u32, param_2: u16);



void win_ui_palette_op_1020_0cd2(param_1: u32, param_2: HWND16);



void pass1_1020_0e2c(param_1: u32, param_2: u16);



void pass1_1020_0e8e(param_1: i16, param_2: u16, param_3: i16, param_4: i16, param_5: i16, param_6: u16, param_7: u16);



void enable_menu_1020_1000(HMENparam_1: u16, i16 param_2);


void win_ui_cursor_op_1020_1294(globals: &mut Globals,
                                param_1: u32,
                                short           param_2,
                                short           param_3,
                                param_4: u16,
                               param_5: u16);




Struct3 *pass1_1020_135e(param_1: *mut Struct18, param_2: u8, param_3: u16);




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_win_ui_op_1020_1418(param_1: *mut Struct40, param_2: u32, param_3: u16);




void win_ui_op_1020_150e(u16 *param_1, param_2: HDC16);



Struct18 *pass1_1020_170a(param_1: *mut Struct18, param_2: u8, param_3: u16);



void pass1_1020_1780(u32 *param_1);




void mixed_ui_op_1020_179c(param_1: *mut Struct1);



void pass1_1018_5e5a(u16 *param_1);



void win_ui_op_1018_5e9a(param_1: *mut Struct1, param_2: u16);



void mix_ui_op_1018_6adc(param_1: *mut Struct28);



Struct11 *pass1_1018_4ae0(param_1: *mut Struct11, param_2: u8, param_3: u16);



void unk_win_ui_op_1018_4f18(param_1: *mut Struct39, param_2: u16, param_3: u32);




Struct11 *pass1_1018_5032(param_1: *mut Struct11, param_2: u8, param_3: u16);



void pass1_1018_57e6(param_1: u32, long param_2, param_3: u16);



void pt_in_rect_1018_1bda(param_1: u32, param_2: u16, param_3: u16, param_4: u16);


void pass1_1018_2440(param_1: *mut Struct11, param_2: u16);



void msg_box_op_1010_8bb4(param_1: u16, param_2: u16, param_3: u32, HINSTANCE16 param_4, param_5: u16);



void ui_op_1010_79aa(param_1: u32, param_2: i16, long param_3, param_4: u16);



void show_win_1010_7a76(param_1: u32, param_2: u16);



void show_window_1010_7ace(param_1: u32, param_2: u16);




u32 destroy_window_1010_7b26(param_1: u32, long param_2, param_3: u16, param_4: u16);



void pass1_1010_8096(param_1: *mut u32, i16 param_2);


struct Struct43 *unk_io_op_1010_830a(param_1: u32, param_2: u16, param_3: u16);



void pass1_1010_71d6(param_1: u32, param_2: i16,param_3: *mut u16, param_4: u16, param_5: u16, param_6: u16);



Struct11 *pass1_1010_5074(param_1: *mut Struct11, param_2: u8);



void pass1_1010_29c6(param_1: *mut Struct11);



void win_ui_op_1010_3202(param_1: u32, param_2: i16, param_3: HWND16);



Struct11 *pass1_1010_0ee6(param_1: *mut Struct11, param_2: u8);




#endif // OUTPOST_1_SRC_UI_OPS_UI_OPS_6_H_
