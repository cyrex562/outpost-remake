//
// Created by cyrex on 2022-05-22.
//

#include "utils.h"

u32 CONCAT22(u16 a, u16 b)
{
    u32 c = 0;
    c = a << 16 | b;
    return c;
}

u16 CONCAT11(u8 a, u8 b) {
    return a << 8 | b;
}

bool CARRY2(u16 a, u16 b) {
    // TODO: implement CARRY2 op
    return false;
}

u32 SUB42(u32 a, u32 b) {
    return a - b;
}
