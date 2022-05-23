//
// Created by cyrex on 2/22/2022.
//

#ifndef OUTPOST_1_SRC_FN_PTR_OPS_FN_PTR_OPS_6_H_
#define OUTPOST_1_SRC_FN_PTR_OPS_FN_PTR_OPS_6_H_

#include "types.h"
#include "globals.h"
#include "fn_ptr_defs.h"
#include "structs_1.h"

void pass1_1000_24db(param_1: u16, param_2: u16, globals: &mut Globals);

void fn_ptr_op_1000_24cd(param_1: u16, param_2: i16);

void mem_op_1000_179c(param_1: u16, mut param_2: *mut u8, param_3: u16);

void fn_ptr_op_1000_2594(fn_ptr_1 *param_1, fn_ptr_1 *param_2);

BOOL16 call_fn_ptr_1000_0dc6(param_1: u16, param_2: u16, param_3: u16);

void pass1_1010_3880(AddrStruct *param_1);

void pass1_1010_2db2(astruct_473 *param_1, param_2: u16);

#endif // OUTPOST_1_SRC_FN_PTR_OPS_FN_PTR_OPS_6_H_
