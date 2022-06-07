//
// Created by cyrex on 2022-06-06.
//

#ifndef OUTPOST_C_PROJ__OPS_4_H_
#define OUTPOST_C_PROJ__OPS_4_H_

#include "types.h"
#include "structs_2.h"

void pass1_1000_0368(u16 param_1,
                     u16 param_2,
                     u16 param_3);

void pass1_1000_20a2(u16 param_1,
                     u16 param_2);

void mem_op_1000_1408(u16 param_1,
                      u32 re_alloc_size,
                      astruct_7 *param_3,
                      i16 selector);

void pass1_1000_27d6(u16 param_1);

u32 ret_op_1000_55ac(void);

void init_1000_23be(u16 param_1,
                    u16 param_2);

void init_op_1008_54aa(u16 param_1,u16 param_2,u16 param_3,u8 *param_4,char *param_5,u8 *param_6,
                      u8 *param_7);

void dos3_call_op_1000_435c(u16 param_1,
                            u16 *param_2,
                            u16 param_3,
                            u16 param_4,
                            u16 param_5,
                            u16 param_6);

void pass1_1000_4d0c(u16 param_1);

BOOL16 pass1_1000_1fea(void);

u16 str_op_1008_60e8(u16 param_1,char *param_2);

u16 str_op_1000_3da4(char *param_1);

void mem_op_1000_179c(i16 param_1,
                      astruct_57 *param_2);

void struct_op_1008_0000(u16 *param_1);

WPARAM16 win_msg_op_1008_9498(void);

u32 mem_op_1000_1b68(u16 param_1,
                     u16 param_2,
                     u16 param_3);

i16 pass1_1000_462e(u16 param_1,
                    u16 param_2,
                    i16 param_3,
                    u16 param_4,
                    u16 param_5,
                    u16 param_6,
                    i16 param_7);

void unk_str_op_1000_3d3e(char *param_1,
                          char *in_string_2);

u16 fn_ptr_op_1000_1708(u16 param_1,
                        u16 param_2,
                        u16 param_3,
                        u16 param_4,
                        u16 param_5);

void pass1_1000_440c(u16 param_1);

i16 pass1_1000_3e2c(u32 param_1);

#endif //OUTPOST_C_PROJ__OPS_4_H_
