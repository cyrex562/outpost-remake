//
// Created by cyrex on 5/23/2022.
//

#ifndef OUTPOST_C_PROJ_TYPES_H_FUNC_PTRS_H_
#define OUTPOST_C_PROJ_TYPES_H_FUNC_PTRS_H_

#include "types.h"

// typedef void (*printer_t)(int);
typedef void (*code)();

typedef i16* (*code2)();

typedef void(*code3)(void*);

typedef u8(*code4)();

typedef i16(*code5)();

typedef bool(*code6)(u16);

typedef u16(*code7)(i16);

#endif //OUTPOST_C_PROJ_TYPES_H_FUNC_PTRS_H_
