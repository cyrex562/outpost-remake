//
// Created by cyrex on 6/6/2022.
//

#include "structs_2.h"
#include "utils.h"
#include "sys_api.h"
#include "globals.h"
#include "func_ptrs.h"
#include "types.h"
#include "ops_1.h"
#include "ops_3.h"
#include "string_defs.h"

u16 pass1_1000_1e61(u16 param_1,
                    u16 param_2,
                    astruct_7 *param_3,
                    u16 param_4)
{
    i16 iVar1;
    BOOL16 BVar2;
    u16 u16_var3;
    u16 UStack64;
    u16 UStack62;
    u16 UStack60;
    code9 *pfn_stack6;
    u8 *puStack4;
    u16 uVar3;

//    uVar3 =  &DAT_1050_1050;
    uVar3 = 0x1050;
    UStack62 = param_3;
    UStack60 = param_4;
    UStack64 = param_2;
//    puStack4 = (u8 *) &DAT_1050_1050;
    puStack4 = 0x1050;
    pfn_stack6 = PTR_PTR_1050_5f1a;
//    if (( PTR_LOOP_1050_5f1c |  PTR_PTR_1050_5f1a) == 0x0) {
    if (PTR_PTR_1050_5f1a == 0) {
        pfn_stack6 = NULL;
        puStack4 = NULL;
    } else {
        iVar1 = mem_op_1000_21b6( PTR_PTR_1050_5f1a,
                                  PTR_LOOP_1050_5f1c);
        pfn_stack6 = (code *) PTR_PTR_1050_5f1a;
        puStack4 = PTR_LOOP_1050_5f1c;
        if (iVar1 == 0x0) {
            PTR_PTR_1050_5f1a = PTR_PTR_1050_1f7e;
            PTR_LOOP_1050_5f1c = 0x1000;
            pfn_stack6 = (code *) &PTR_PTR_1050_1f7e;
//            puStack4 = (u8 *) &PTR_LOOP_1050_1000;
            puStack4 = 0x1000;
        }
    }
//    if (( puStack4 | pfn_stack6) == 0x0) {
    if (puStack4 == 0) {
        return 0x0;
    }
//    BVar2 = msg_box_op_1000_1f24( &PTR_PTR_1050_5f1a,
//                                  &DAT_1050_1050,
//                                 0x0);
BVar2 = msg_box_op_1000_1f24( &PTR_PTR_1050_5f1a,
                                  0x1050,
                                 0x0);
    if (BVar2 == 0x0) {
//        u16_var3 = (*pfn_stack6)(0x1000,
//                            &UStack64,
//                             &DAT_1050_1050,
//                            uVar3);
        u16_var3 = (*pfn_stack6)(0x1000,
                                &UStack64,
                                0x1050,
                                uVar3);
    } else {
        puStack4 = NULL;
        pfn_stack6 = NULL;
        u16_var3 = 0x0;
    }
//    if (( puStack4 |  pfn_stack6) != 0x0) {
    if (puStack4 != 0) {
        pass1_1000_1f68();
    }
    return u16_var3;
}

bool mem_op_1000_21b6(u16 param_1,
                      u16 param_2)
{
    BOOL16 BVar1;

    BVar1 = mem_op_1000_1dfa(0x0,
                             0x4,
                             param_1,
                             param_2);
    return BVar1 == 0x0;
}

BOOL16 mem_op_1000_1dfa(i16 param_1,
                        u8 param_2,
                        u16 param_3,
                        u16 param_4)
{
    u32 uVar1;
    u16 uVar2;

    if ((param_2 & 0x4) == 0x0) {
        uVar2 =  (u8) (((u8) (- ((param_2 & 0x2) == 0x0) >> 0x8) & 0xfe) + 0x92) << 0x8;
    } else {
        uVar2 = 0x1800;
    }
    if ((param_4 == 0x0)
        || ((param_4 & 0xff00 &  (u8) (((u8) (- ((param_2 & 0x4) == 0x0) >> 0x8) & 0x82) + 0x18) << 0x8)
            != uVar2)) {
        return 0x1;
    }
    if (param_1 != 0x0) {
        uVar1 = SegmentLimit(param_4);
        if (CARRY2(param_3,
                   param_1 - 0x1U)) {
            return 0x1;
        }
        if ( uVar1 < param_3 + (param_1 - 0x1U)) {
            return 0x1;
        }
    }
    return 0x0;
}

BOOL16 msg_box_op_1000_1f24(i16 param_1,
                            u16 param_2,
                            u16 param_3)
{
    i16 *piVar1;
    u16 unaff_CS;

    if (param_3 < (param_1 + 0xc)) {
//        msg_box_op_1000_214c(0x0,
//                             0x0,
//                             0xd940,
//                              &PTR_LOOP_1050_1040);
msg_box_op_1000_214c(0x0,
                             0x0,
                             0xd940,
                              0x1040);
        return 0x1;
    }
    piVar1 = (i16 *) (param_1 + 0xc);
    *piVar1 = *piVar1 + 0x1;
    return 0x0;
}

void pass1_1000_1f68()
{
    PTR_LOOP_1050_5f26 -= 1;
    if (PTR_LOOP_1050_5f26 < 0x0) {
        PTR_LOOP_1050_5f26 = 0;
    }
}

BOOL16 msg_box_op_1000_214c(u16 param_1,
                            i16 param_2,
                            u16 param_3,
                            u16 param_4)
{
    INT16 IVar1;
    i16 iVar2;
    u16 type;

    type = 0x2 - (param_2 == 0x0) | 0x2110;
    MessageBeep16(0x0);
    do {
        IVar1 = MessageBox16(type,
                             "SmartHeap Library",
                             (char *) CONCAT22(param_4,
                                               param_3),
                             0x0);
        iVar2 = IVar1 + -0x1;
        if (iVar2 == 0x0) {
            return 0x0;
        }
        if ((0x0 < iVar2) && (!SBORROW2(iVar2,
                                        0x1))) {
            if (IVar1 == 0x3 || IVar1 + -0x2 < 0x1) {
                fatal_app_exit_1000_3e9e();
                return 0x0;
            }
            if (IVar1 == 0x4) {
                return 0x1;
            }
            if (IVar1 == 0x5) {
                return 0x0;
            }
        }
        if ((type & 0x2000) == 0x0) {
            return 0x0;
        }
        type = type & 0xdfef | 0x1010;
    } while (true);
}

void fatal_app_exit_1000_3e9e()
{
    FatalAppExit16(s_ABNORMAL_PROGRAM_TERMINATION_1050_6544,
                   0x0);
}
