//
// Created by cyrex on 2022-05-22.
//

#include "sys_api.h"

u16 _SHI_INVOKEERRORHANDLER1(void)
{
    i16 iVar1;
    BOOL16 BVar2;
    u16 uVar2;
    code *pcStack6;
    u16 *puStack4;
    u16 uVar3;

    puStack4 = (u16 *) 0x1050;
    if (( PTR_LOOP_1050_5f1c |  PTR_PTR_1050_5f1a) == 0x0) {
        pcStack6 = NULL;
        puStack4 = NULL;
    } else {
        iVar1 = mem_op_1000_21b6( PTR_PTR_1050_5f1a,
                                  PTR_LOOP_1050_5f1c);
        pcStack6 = (code *) PTR_PTR_1050_5f1a;
        puStack4 = PTR_LOOP_1050_5f1c;
        if (iVar1 == 0x0) {
            PTR_PTR_1050_5f1a = (u8 *) &PTR_PTR_1050_1f7e;
            PTR_LOOP_1050_5f1c = (u8 *) &PTR_LOOP_1050_1000;
            pcStack6 = (code *) &PTR_PTR_1050_1f7e;
            puStack4 = (u16 *) &PTR_LOOP_1050_1000;
        }
    }
    if (( puStack4 |  pcStack6) != 0x0) {
        BVar2 = msg_box_op_1000_1f24( &PTR_PTR_1050_5f1a,
                                      0x1050,
                                     0x0);
        if (BVar2 == 0x0) {
            uVar2 = (*pcStack6)();
        } else {
            puStack4 = NULL;
            pcStack6 = NULL;
            uVar2 = 0x0;
        }
        if (( puStack4 |  pcStack6) != 0x0) {
            pass1_1000_1f68();
        }
        return uVar2;
    }
    return 0x0;
}
