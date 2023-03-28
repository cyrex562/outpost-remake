//
// Created by cyrex on 2/17/2023.
//

#ifndef OUTPOST_1_SRC_WIN_OPS_WIN_OPS_3_H_
#define OUTPOST_1_SRC_WIN_OPS_WIN_OPS_3_H_


// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "structs/structs_0xx/struct_20.h"
// #include "structs/structs_2xx/structs_26x.h"


void window_op_1018_e6c6(param_1: *mut Struct0);


void pass1_1018_e72a(param_1: u32);


void post_win_msg_1018_ea0a(param_1: u16, param_2: u16, param_3: i16, param_4: HWND16);


void pass1_1018_ea66(param_1: u32, param_2: u16);


void window_op_1018_eada(param_1: *mut Struct0);


void pass1_1018_eb3e(param_1: u32, param_2: u16);


void pass1_1020_02ae(param_1: u32);


void win_1020_0316(param_1: *mut Struct0, param_2: *mut u8, param_3: u16);


void post_msg_1020_03b2(param_1: u32, param_2: HWND16);


void post_msg_1020_03d6(param_1: u32, param_2: HWND16);


void post_msg_1020_03fa(param_1: u32, param_2: HWND16);


void post_win_msg_1020_061c(param_1: u32, param_2: i16, param_3: HWND16);


void pass1_1020_08b6(WNDCLASS16      *param_1,
                     struct param_2: *mut Struct20,
                     param_3: u16,
                     u32              param_4);


void win_1018_df40(param_1: *mut Struct0, param_2: u16, param_3: *mut u8, param_4: u16);


void pass1_1018_df92(param_1: u32);


void pass1_1018_e2cc(param_1: *mut Struct269, param_2: u16);


void window_op_1018_e384(param_1: *mut Struct0);


void pass1_1018_e3e8(param_1: u32);


void destroy_window_1018_c518(param_1: *mut Struct29);


Struct29 *pass1_1018_c896(param_1: *mut Struct29, param_2: u8);


void unk_destroy_window_op_1018_6bb6(param_1: *mut Struct28, param_2: HWND16);


void win_1018_598c(param_1: *mut Struct0, param_2: u16, param_3: u16);


void window_op_1018_67b6(param_1: *mut Struct0);


void pass1_1018_681a(param_1: u32);


void win_op_1018_294a(param_1: i16,
                      param_2: u16,
                      param_3: u16,
                      param_4: u32,
                      param_5: u16,
                      LPCSTR in_string_6);


u32 set_err_mode_1010_8b14(param_1: u32, param_2: u32, param_3: u16);


void send_msg_1010_7c42(param_1: u32, param_2: u16);


void send_msg_1010_7c9e(param_1: u32, param_2: i16, param_3: u16);


void pass1_1010_71b0(param_1: i16, param_2: u16);


void pass1_1010_71c2(param_1: u16, param_2: u16, param_3: i16, param_4: u16);


void unk_win_op_1010_7300(param_1: u32, param_2: u32, param_3: u16, param_4: u32);


void free_rsrc_1010_4b3e(u16 *param_1, HGLOBAL16 param_2);


void unk_destroy_win_op_1010_2fa0(param_1: u32, param_2: HWND16);


void unk_destroy_win_op_1010_305a(param_1: *mut Struct27,
                                  param_2: i16,
                                  param_3: *mut Struct65,
                                 param_4: u16);


void pass1_1010_1656(param_1: i16,
                     param_2: u16,
                     param_3: u16,
                     param_4: u32,
                     param_5: u16,
                     param_6: *mut u8,
                     param_7: i16,
                     param_8: u16);


void set_window_placement_1010_0070(param_1: u32,
                                    param_2: i16,
                                    param_3: u16,
                                    param_4: HWND16,
                                   param_5: u16);


void set_win_placement_1010_010e(param_1: u16, param_2: u16, param_3: u16, param_4: HWND16);


void enum_child_windows_1010_01be(LPVOID param_1);


void pass1_1008_aa28(param_1: u32, param_2: u16, WNDCLASS16 *param_3);


WPARAM16 main_win_msg_loop_1008_9498(globals: &mut Globals, u16_arg1: u16, u16_arg2: u16);


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_win_msg_op_1008_9510(i16 *param_1, MSG16 *param_2, MSG16 *param_3);


void send_msg_1008_9640(param_1: u32, param_2: u16, param_3: HWND16);


ATOM win_ui_reg_class_1008_96d2(Globals         *globals,
                                struct param_1: &mut Struct20,
                                HINSTANCE16      in_h_inst_2,
                                WNDCLASS16      *in_wnd_class_3);


void create_window_ex_1008_9760(in_struct_1: *mut Struct0, param_2: u16);


u32 unk_win_op_1008_97f2(u32     *param_1,
                         i16     *param_2,
                         WPARAM16 param_3,
                         u8      *param_4,
                         param_5: u16,
                         HWND16   param_6);


LRESULT make_def_wnd_proc_1008_9ce6(param_1: u16,
                                    param_2: u16,
                                    in_msg_3: u16,
                                    WPARAM16 param_4,
                                    LPARAM   param_5,
                                    HWND16   in_hwnd_5);


void pass1_1008_9e5a(param_1: *mut Struct11);


void post_win_msg_1008_a0e4(param_1: *mut Struct67,
                            long      param_2,
                            param_3: i16,
                            param_4: u16,
                            param_5: u32,
                            param_6: i16,
                            HWND16    param_7,
                           param_8: u16);


u16 *pass1_1008_91ba(u16 *param_1, param_2: HWND16);


void kill_timer_1008_921c(u16 *param_1, param_2: HWND16);


void send_msg_1008_84ba(param_1: u16, param_2: u32, param_3: HWND16);


void win_1008_5c9e(param_1: u32,
                  param_2: *mut u32,
                   param_3: u16,
                   param_4: u16,
                   WNDCLASS16 *param_5);


HWND16
create_window_1008_5e7e(globals: &mut Globals, in_stock_obj_id: u16, WNDCLASS16 *in_wnd_class);


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT make_def_win_proc_1008_5f44(param_1: u16,
                                    WPARAM16 in_wparam_2,
                                    LPARAM   param_3,
                                    HWND16   in_hwnd_4);


void destroy_win_1008_628e(param_1: u32, param_2: HWND16);


#endif // OUTPOST_1_SRC_WIN_OPS_WIN_OPS_3_H_
