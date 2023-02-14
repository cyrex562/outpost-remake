//
// Created by cyrex on 2/22/2022.
//

#ifndef OUTPOST_1_SRC_FN_PTR_OPS_FN_PTR_OPS_6_H_
#define OUTPOST_1_SRC_FN_PTR_OPS_FN_PTR_OPS_6_H_

#include "fn_ptr_defs.h"
#include "globals.h"
#include "op_int.h"
#include "structs_1.h"

void pass1_1000_24db(u16 param_1, u16 param_2, Globals* globals);

void fn_ptr_op_1000_24cd(u16 param_1, i16 param_2);

void mem_op_1000_179c(u16 param_1, u8 *param_2, u16 param_3);

void fn_ptr_op_1000_2594(fn_ptr_1 *param_1, fn_ptr_1 *param_2);

BOOL16 call_fn_ptr_1000_0dc6(u16 param_1, u16 param_2, u16 param_3);

void pass1_1010_3880(AddrStruct *param_1);

void pass1_1010_2db2(Struct473 *param_1, u16 param_2);

#endif // OUTPOST_1_SRC_FN_PTR_OPS_FN_PTR_OPS_6_H_
