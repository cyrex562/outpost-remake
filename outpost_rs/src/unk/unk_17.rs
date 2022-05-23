//
// Created by cyrex on 2/25/2022.
//

#include "types.h"
#include "unk_2.h"
#include "unk_17.h"
void  pass1_1038_b6e0(param_1: u32, param_2: i16)

{
    u32 uVar1;
    i16        iVar2;
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
            break;
        uStack4 = uStack4 + 0x1;
    }
    (uStack4 * 0x4 + iVar2) = 0x0;
    return;
}
