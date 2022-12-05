//
// Created by cyrex on 6/6/2022.
//

#include "ops_2.h"
#include "globals.h"
#include "utils.h"
#include "ops_3.h"

u32 pass1_1000_52f0(u16 param_1,
                    u16 param_2,
                    u16 param_3,
                    u16 param_4)
{
    u32 uVar1;
    let mut lVar2: i32;
    u16 uVar3;
    u16 uVar4;
    i16 iVar5;
    i16 iVar6;
    u16 uVar7;
    u16 uVar8;
    u16 uVar9;
    u16 uVar10;
    u16 uVar11;
    bool bVar12;
    bool bVar13;

    bVar13 =  param_2 < 0x0;
    if (bVar13) {
        bVar12 = param_1 != 0x0;
        param_1 = -param_1;
        param_2 = - bVar12 - param_2;
    }
    uVar11 =  bVar13;
    if ( param_4 < 0x0) {
        bVar13 = param_3 != 0x0;
        param_3 = -param_3;
        param_4 = - bVar13 - param_4;
    }
    uVar3 = param_1;
    uVar4 = param_3;
    uVar8 = param_2;
    uVar9 = param_4;
    if (param_4 == 0x0) {
        iVar5 =  (((u32) param_2 % (u32) param_3 << 0x10 | (u32) param_1) % (u32) param_3);
        iVar6 = 0x0;
        if ( (uVar11 - 0x1) < 0x0) {
            goto LAB_1000_538a;
        }
    } else {
        do {
            uVar10 = uVar9 >> 0x1;
            uVar4 = uVar4 >> 0x1 |  ((uVar9 & 0x1) != 0x0) << 0xf;
            uVar7 = uVar8 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar8 & 0x1) != 0x0) << 0xf;
            uVar8 = uVar7;
            uVar9 = uVar10;
        } while (uVar10 != 0x0);
        uVar1 = CONCAT22(uVar7,
                         uVar3) / (u32) uVar4;
        uVar3 =  uVar1 * param_4;
        lVar2 = (uVar1 & 0xffff) * (u32) param_3;
        uVar8 =  ((u32) lVar2 >> 0x10);
        uVar4 =  lVar2;
        uVar9 = uVar8 + uVar3;
        if (((CARRY2(uVar8,
                     uVar3)) || (param_2 < uVar9)) || ((param_2 <= uVar9 && (param_1 < uVar4)))) {
            bVar13 = uVar4 < param_3;
            uVar4 -= param_3;
            uVar9 = (uVar9 - param_4) -  bVar13;
        }
        iVar5 = uVar4 - param_1;
        iVar6 = (uVar9 - param_2) -  (uVar4 < param_1);
        if (-0x1 <  (uVar11 - 0x1)) {
            goto LAB_1000_538a;
        }
    }
    bVar13 = iVar5 != 0x0;
    iVar5 = -iVar5;
    iVar6 = - bVar13 - iVar6;
    LAB_1000_538a:
    return CONCAT22(iVar6,
                    iVar5);
}
