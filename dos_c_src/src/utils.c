//
// Created by cyrex on 2/22/2022.
//

#include "utils.h"
#include "types.h"

u32 CONCAT22(u16 a, u16 b) {
    return a << 16 | b;
}

u16 CONCAT11(u8 a, u8 b) {
    return a << 8 | b;
}
