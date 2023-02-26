//
// Created by cyrex on 3/2/2022.
//

#ifndef OUTPOST_1_SRC_DRAW_OPS_DRAW_OPS_2_H_
#define OUTPOST_1_SRC_DRAW_OPS_DRAW_OPS_2_H_

#include "globals.h"
#include "op_int.h"
#include "structs/structs_0xx/structs_2x.h"

void  string_1020_79b4(u16 param_1, u32 param_2, i16 param_3, char *param_4);

void  pass1_1038_9a48(Struct18 *param_1);

void  pass1_1038_7d5c(Struct18 *param_1);

void  unk_draw_op_1018_cda8(Struct36 *param_1, u16 param_2);

void  unk_draw_op_1018_cfc0(Struct36 *param_1, u16 param_2);

void  palette_op_1020_92c4(u16 *param_1, HDC16 param_2);

void  mix_draw_op_1020_9312(u32 param_1, HWND16 param_2);

void  draw_op_1020_9364(Struct7 *param_1, HWND16 in_win_handle_2, u16 param_3);

Struct18 * pass1_1020_96a2(Struct18 *param_1, u8 param_2, u16 param_3);

Struct18 * pass1_1020_7526(Struct18 *param_1, u8 param_2, u16 param_3);

void  struct_1020_7554(u16 param_1, Struct20 *param_2, u16 param_3, u16 param_4);

void  pass1_1020_7824(Struct666 *param_1, u16 param_2, u16 param_3, i16 param_4, u16 param_5);

void  pass1_1020_78ac(u16 *param_1, u16 param_2);

void  struct_1020_790e(u16 *param_1, u32 param_2, u16 param_3, u32 param_4, u16 param_5);

void  string_1020_79b4(u16 param_1, u32 param_2, i16 param_3, char *param_4);

void  pass1_1020_79e4(u32 param_1, u16 param_2, u16 param_3);

void  draw_op_1020_7cc8(u32 param_1, HWND16 in_win_handle_2, u16 param_3);

void unk_draw_op_1020_7f7a(Globals *globals, Struct20 *param_1, u16 param_2, u32 param_3);

void  realize_palette_1020_8128(u32 param_1, i16 param_2, HGDIOBJ16 param_3, u16 param_4);

void  win_ui_palette_op_1020_81c0(HWND16 param_1);

void  pass1_1020_6466(u16 *param_1, u16 param_2, u16 param_3);

void  mix_draw_op_1020_650c(Struct7 *param_1, HWND16 param_2, u16 param_3);

void  realize_palette_1020_6896(u32 param_1, i16 param_2, HGDIOBJ16 param_3);

void  pass1_1020_68de(u32 param_1, u16 param_2);

void  pt_in_rect_1020_68fc(u32 *param_1, u16 param_2, u16 param_3);

void  destroy_icon_1020_6bd2(u32 param_1, u8 param_2, HICON16 param_3);

HGDIOBJ16  draw_op_1020_7070(u16 param_1, u16 param_2);

void  palette_op_1020_7270(u16 *param_1, HDC16 param_2);

void  invalidate_rect_1020_735a(u32 param_1, HWND16 param_2);

BOOL16  win_ui_op_1020_737a(u32 param_1, HWND16 param_2, u16 param_3);

void  unk_draw_op_1020_3da4(Struct24 *param_1, u32 param_2);

void  win_ui_palette_op_1020_3e84(Struct16 *param_1);

void  validate_rect_1020_3f12(u32 param_1, i16 param_2, HWND16 param_3);

void  mixed_draw_op_1020_3fa0(u32 param_1, HWND16 param_2, u16 param_3);

Struct18 * pass1_1020_4064(Struct18 *param_1, u8 param_2);

void  draw_rect_1020_40ce(u32 param_1, i16 param_2, i16 param_3, u16 param_4);

void unk_draw_op_1020_41c8(struct Globals *globals, struct Struct20 *param_1, u16 param_2, u16 param_3, u16 param_4);

void  destroy_cursor_1020_42f4(u16 *param_1, HMENU16 param_2);

void  pass1_1020_2c72(u32 param_1, u16 param_2, u16 param_3);

void  destroy_icon_1020_2c88(u32 param_1, HICON16 param_2);

void  load_draw_op_1020_2ede(u16 *param_1, u32 param_2, u16 param_3);

void  invalidate_rect_1020_3080(u32 param_1, i16 param_2, HWND16 param_3);

void  draw_op_1020_30be(u32 param_1, HWND16 param_2, u16 param_3);

void  unk_draw_op_1020_320e(u32 param_1, HDC16 param_2, u16 param_3);

void  draw_op_1020_33c0(u32 param_1, u32 param_2, i16 param_3, u32 param_4, i16 param_5, u16 param_6, u16 param_7);

void  draw_op_1020_3488(u32 param_1);

void  draw_polygon_1020_3602(u16 param_1, u16 param_2, u32 param_3, HDC16 param_4);

void  pass1_1020_3bd6(u32 param_1, u16 param_2, u16 param_3, u16 param_4);

void  pass1_1020_3d08(u16 param_1,
                                    u16 param_2,
                                    u16 param_3,
                                    u16 param_4,
                                    u16 param_5,
                                    u16 param_6,
                                    u16 param_7,
                                    u16 param_8,
                                    u16 param_9,
                                    u8  param_10,
                                    u8  param_11,
                                    u8  param_12,
                                    u8  param_13,
                                    u8  param_14,
                                    u32 param_15,
                                    u16 param_16,
                                    u16 param_17,
                                    u16 param_18,
                                    u16 param_19);

void  invalidate_rect_1020_1fb2(u32 param_1, i16 param_2, HWND16 param_3);

void  unk_draw_op_1020_2020(u32 param_1, HWND16 param_2, u16 param_3)
;

void  draw_line_1020_229c(u32 param_1, HDC16 param_2);

void  pass1_1020_239c(u32 param_1, i16 *param_2, u16 param_3);

void  draw_polygon_1020_2474(u16 param_1, u16 param_2, u32 param_3, HDC16 param_4);

void  struct_1020_2524(Struct20 *param_1, u16 param_2, u16 param_3, u16 param_4);

void  pass1_1020_27b0(Struct664 *param_1, u16 param_2, u16 param_3, i16 param_4, u16 param_5);

void  pass1_1020_2838(u16 *param_1, u16 param_2);

#endif // OUTPOST_1_SRC_DRAW_OPS_DRAW_OPS_2_H_
