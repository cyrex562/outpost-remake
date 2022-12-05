//
// Created by cyrex on 2/22/2022.
//

#ifndef OUTPOST_1_SRC_FN_PTR_DEFS_H_
#define OUTPOST_1_SRC_FN_PTR_DEFS_H_

#include "types.h"

typedef void (*fn_ptr_1)();

typedef u32 (*fn_ptr_2)(u16);

typedef void(*fn_ptr_3)(u16, u16, u16, u16, u16);

typedef void (*fn_ptr_4)(u32*, u32*, u16, u8);

#endif // OUTPOST_1_SRC_FN_PTR_DEFS_H_
