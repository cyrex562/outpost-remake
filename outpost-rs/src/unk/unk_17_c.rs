//
// Created by cyrex on 2/25/2022.
//

// #include "unk_17.h"

// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "unk_2.h"

// #include <stdbool.h>

void  pass1_1038_b6e0(param_1: u32, param_2: i16)

{
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uStack4: u16;

    uStack4 = 0x1;
    while(true)
    {
        if(0x2a < uStack4)
        {
            return;
        }
        uVar3 = (param_1 >> 0x10);
        iVar2 = param_1;
        if((((uStack4 * 0x4 + iVar2 + 0x2) | (uStack4 * 0x4 + iVar2)) != 0x0) && (uVar1 = (uStack4 * 0x4 + iVar2), (uVar1 + 0x6) == param_2))
        {
            break;
        }
        uStack4 = uStack4 + 0x1;
    }
    // TODO
//    (uStack4 * 0x4 + iVar2) = 0x0;
    return;
}