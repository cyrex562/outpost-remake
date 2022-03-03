//
// Created by cyrex on 2/22/2022.
//

#ifndef OUTPOST_1_SRC_STRING_OPS_H_
#define OUTPOST_1_SRC_STRING_OPS_H_

#include "types.h"
#include "globals.h"

void pass1_1000_2913(i16 param_1, u16 param_2, u16 param_3);

cstring poss_str_op_1000_28dc(i16 param_1);

char *string_op_1010_ada6(HINSTANCE16 param_1, u16 param_2, u16 param_3, u16 param_4, i16 param_5, i16 param_6);

u16 str_op_1000_3da4(char *param_1);

u8 str_op_1000_3dbe(char *param_1, char *param_2, u16 param_3);

char *load_string_1010_847e(cstring param_1_str_buf, u16 param_2_buf_len, HINSTANCE16 param_3_hinstance);

cstring string_1020_c0d8(Globals *globals, u16 param_1);

cstring pass1_1020_bd80(Globals *globals, u16 param_1);

#endif // OUTPOST_1_SRC_STRING_OPS_H_
