//
// Created by cyrex on 6/6/2022.
//

#include "ops_2.h"
#include "globals.h"
#include "utils.h"

u32 mem_op_1000_0838(u16 param_1)
{
    u16 *puVar1;
    i16 *piVar2;
    i16 iVar3;
    u16 *puVar4;
    u16 uVar5;
    u16 uVar6;
    u16 UVar7;
    u16 UVar8;
    i16 *piVar9;
    u16 unaff_CS;
    bool bVar10;
    u16 uStack6;
    i16 *piStack4;

    piVar9 = *(i16 **) (param_1 + 0x2);
    piStack4 = piVar9;
    if ((param_1 + 0x2) == 0x0) {
        goto LAB_1000_085b;
    }
    do {
        do {
            if (*piVar9 != 0x0) {
                iVar3 = piVar9[0x5];
                puVar4 = (u16 *) &PTR_LOOP_1050_000e;
                if (puVar4 != NULL) {
                    &PTR_LOOP_1050_000e = *puVar4;
                    piVar2 = (i16 *) &PTR_LOOP_1050_000a;
                    *piVar2 = *piVar2 + 0x1;
                    *(i16 **) (param_1 + 0x2) = piVar9;
                    return CONCAT22(iVar3,
                                    puVar4);
                }
                *piVar9 = 0x0;
            }
            piVar9 = (i16 *) piVar9[0x2];
        } while (piVar9 != piStack4);
        LAB_1000_085b:
        if ((param_1 + 0x18) == 0x0) {
            // &DAT_1050_1050
            pass1_1000_1e61(unaff_CS,
                            0x4,
                            param_1);
            return 0x0;
        }
        uVar5 = (param_1 + 0x1a);
        while (true) {
            uStack6 = uVar5;
            uVar5 = 0x1;
            UVar8 = mem_op_1000_03c6(uStack6,
                                     0x0,
                                     0x1,
                                     param_1,
                                     0x0,
                                     '\0');
            if ((UVar8 | uVar5) != 0x0) {
                break;
            }
            uVar5 = (param_1 + 0x1e);
            uVar6 = uVar5 + uStack6;
            uVar5 = (param_1 + 0x20) +  CARRY2(uVar5,
                                                    uStack6);
            puVar1 = (u16 *) (param_1 + 0x28);
            bVar10 = *puVar1 <= uVar5;
            if (bVar10) {
                if (bVar10 && uVar5 != *puVar1) {
                    return 0x0;
                }
                puVar1 = (u16 *) (param_1 + 0x26);
                if (*puVar1 <= uVar6 && uVar6 != *puVar1) {
                    return 0x0;
                }
            }
            uVar5 = uStack6 >> 0x1;
            if (uStack6 >> 0x1 < (param_1 + 0x18) + 0x14U) {
                // &DAT_1050_1050
                UVar7 = pass1_1000_1e61(unaff_CS,
                                        0x2,
                                        param_1);
                uVar5 = uStack6 & 0xfffe;
                if (UVar7 == 0x0) {
                    return 0x0;
                }
            }
        }
        piVar9 = *(i16 **) (param_1 + 0x2);
        piStack4 = (i16 *) piVar9[0x2];
    } while (true);
}
