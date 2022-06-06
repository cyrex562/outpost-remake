//
// Created by cyrex on 2022-05-22.
//

#include <stdbool.h>
#include "types.h"

#ifndef OUTPOST_C_PROJ__UTILS_H_
#define OUTPOST_C_PROJ__UTILS_H_

u32 CONCAT22(u16 a, u16 b);

bool CARRY2(u16 a, u16 b);

u16 CONCAT11(u8 a, u8 b);

u32 SUB42(u32 a, u32 b);

u32 CARRY4(u32 a, u32 b);

u16 SBORROW2(u16 a, u16 b);

#endif //OUTPOST_C_PROJ__UTILS_H_
