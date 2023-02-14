//
// Created by cyrex on 3/2/2022.
//

#ifndef OUTPOST_1_SRC_STRUCTS_ADDR_STRUCT_H_
#define OUTPOST_1_SRC_STRUCTS_ADDR_STRUCT_H_

#include "op_int.h"
#include "structs/structs_0xx/structs_x.h"
#include "structs/structs_3xx/struct_318.h"
#include "structs/structs_3xx/structs_30x.h"
#include "structs/structs_3xx/structs_32x.h"
#include "structs/structs_3xx/structs_33x.h"
#include "structs/structs_3xx/structs_34x.h"
#include "structs/structs_3xx/structs_35x.h"
#include "structs/structs_3xx/structs_36x.h"
#include "structs/structs_3xx/structs_37x.h"
#include "structs/structs_3xx/structs_38x.h"
#include "structs/structs_4xx/structs_44x.h"
#include "structs/structs_4xx/structs_49x.h"
typedef struct AddrStruct {
    u16 offset;
    u16 base;
} AddrStruct;
typedef struct Address1 {
    u16 offset;
    u16 base;
} Address1;
typedef struct Address2 {
    u16 base;
    u16 offset;
} Address2;
#endif // OUTPOST_1_SRC_STRUCTS_ADDR_STRUCT_H_
