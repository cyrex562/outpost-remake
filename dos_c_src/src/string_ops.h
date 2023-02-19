//
// Created by cyrex on 2/22/2022.
//

#ifndef OUTPOST_1_SRC_STRING_OPS_H_
#define OUTPOST_1_SRC_STRING_OPS_H_

#include "globals.h"
#include "op_int.h"


void string_1040_a626(struct Struct381 *param_1, char *param_2, u16 param_3);




char *pass1_1040_4dcc(u32 param_1, i16 param_2, u16 param_3);




void pass1_1040_5d42(Globals *globals, struct Struct382 *param_1);




void pass1_1038_4d3c(struct Struct385 *param_1, char *param_2, u16 param_3);




void pass1_1030_4dbc(struct Struct386 *param_1, u32 param_2, long param_3);



cstring pass1_1020_bd80(Globals *globals, u16 param_1);



void string_1020_c0ca(Globals *globals, u16 param_1);




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

cstring string_1020_c0d8(Globals *globals, u16 param_1);




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

cstring *string_op_1020_c222(u16 param_1, Globals *globals);




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

cstring *string_op_1020_c2f8(Globals *globals, u16 param_1);



void pass1_1020_6e52(Globals *globals, u16 param_1, u16 param_2, u16 param_3, i16 param_4, u16 param_5, i16 param_6);



void spri16f_op_1018_34b6(u32 param_1, u8 param_2);




// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_str_op_1018_35b0(u32 param_1, u16 param_2, u16 param_3);



BOOL16 string_1018_39d8(u16 param_1, u32 param_2, u32 param_3, u32 param_4);



u32 pass1_1018_3a7a(u32 param_1, u32 param_2, u16 param_3, u16 param_4);



void pass1_1010_dc36(u16 param_1, u16 param_2, u16 param_3, u32 param_4, u16 *param_5, u16 param_6);



void load_str_1010_ddf6(u32 param_1, u32 param_2);


void pass1_1010_de78(u32 param_1, u32 param_2);



char *load_string_1010_ac92(HINSTANCE16 param_1, u16 param_2, u16 param_3, i16 param_4);



char *string_op_1010_ada6(HINSTANCE16 param_1, u16 param_2, u16 param_3, u16 param_4, i16 param_5, i16 param_6);



u16 pass1_1010_ae12(u16 param_1, u16 param_2, u32 param_3, i16 param_4, u16 param_5);



char *load_string_1010_9432(Globals *globals, HINSTANCE16 param_1);



char *load_string_1010_847e(cstring param_1_str_buf, u16 param_2_buf_len, HINSTANCE16 param_3_hinstance);




void load_string_1010_84ac(i16 param_1, u16 param_2, HINSTANCE16 param_3);




void load_string_1010_84e0(HINSTANCE16 in_hinstance_5, u16 param_2, u16 param_3, u16 in_resc_id_3, char *in_buffer_4, short in_buf_len_5);




void pass1_1010_84f8(u32 param_1, i16 param_2, u16 param_3);



void pass1_1010_85be(u32 param_1, i16 param_2, i16 param_3, u16 param_4);



void pass1_1010_6034(u32 param_1, u16 param_2);



char *load_string_1008_ee56(void);



u16 pass1_1008_e2a4(u32 param_1, u32 param_2, u32 param_3);




void pass1_1008_e320(Struct102 *param_1, u32 param_2, u32 param_3, u16 param_4);



void load_str_and_spri16f_1008_b69c(Struct25 *param_1, WORD *param_2, u8 *param_3);




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void load_str_and_spri16f_1008_b78a(u32 param_1, WORD *param_2, u8 *param_3, u16 param_4);




char *load_string_1008_b1f0(void);




void pass1_1008_9c86(u32 param_1, char *param_2, i16 param_3);



u32 *str_1008_6d8a(u32 *param_1, char *param_2, u16 param_3, u16 param_4, u8 param_5);




void struct_op_1008_48fe(Struct81 *param_1, u16 param_2, char *param_3, u16 param_4);




void  pass1_1008_48de(u16 param_1, u32 param_2, i16 param_3, u16 param_4, u16 *param_5, i16 param_6, i16 param_7, u8 *param_8, u16 param_9, u16 param_10, char param_11, u16 param_12, u8 param_13);



void pass1_1008_049c(u16 param_1, u16 param_2, char *param_3);




void str_1000_4d58(char *in_string_1, char *in_string_2, u32 param_3, u32 param_4, WNDCLASS16 *param_5);



u16 str_op_1000_3da4(char *param_1);




u8 str_op_1000_3dbe(char *param_1, char *param_2, u16 param_3);


i16 pass1_1000_3ec0(Globals *globals, u16 param_1, u16 param_2);


char* poss_str_op_1000_28dc(Globals *globals, i16 param_1);


void pass1_1000_2913(Globals *globals, i16 param_1, u16 param_2, u16 param_3);




#endif // OUTPOST_1_SRC_STRING_OPS_H_
