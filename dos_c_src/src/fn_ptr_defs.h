//
// Created by cyrex on 2/22/2022.
//

#ifndef OUTPOST_1_SRC_FN_PTR_DEFS_H_
#define OUTPOST_1_SRC_FN_PTR_DEFS_H_

#include "op_int.h"

typedef void (*FnPtr1)();

typedef u32 (*FnPtr2)(u16);

typedef void(*FnPtr3)(u16, u16, u16, u16, u16);

typedef void (*FnPtr4)(u32*, u32*, u8);

typedef u16(*Int21GetDosVersInfo)(void* dos_vers_info);

#endif // OUTPOST_1_SRC_FN_PTR_DEFS_H_
