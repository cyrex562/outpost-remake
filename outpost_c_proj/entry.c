//
// Created by cyrex on 2022-05-24.
//

#include "types.h"
#include "structs_2.h"
#include "func_ptrs.h"
#include "entry.h"
#include "utils.h"
#include "globals.h"
#include "sys_api.h"
#include "ops_1.h"
#include "ops_4.h"

i16 *entry(u16 param_1,
           u16 param_2,
           i16 param_3,
           u8 *param_4,
           u8 *param_5,
           u16 param_6)
{
    astruct_822 *paVar1;
    i16 *piVar2;
    u16 uVar4;
    char *string_var4;
    i16 iVar5;
    i16 *piVar4;
    i16 iVar6;
//    u8 *SI_REG;
    i16 *piVar7;
//    HINSTANCE16 DI_REG;
    char *paVar8;
    bool bVar9;
    u32 u32_var10;
    u32 u32_var11;
    u32 uVar10;
    u32 u32_var12;
    astruct_825 *paVar13;
    i16 *piVar1;

    u32_var11 = CONCAT22(param_6,
                         PTR_LOOP_1050_5f84);
    do {
        InitTask16(NULL);
        PTR_LOOP_1050_5f84 = (u8 *) u32_var11;
        if (param_3 != 0x0) {
            PTR_LOOP_1050_5f7e = 0x1050;
            bVar9 = param_5 < (u8 *) 0xff00;
            param_5 = param_5 + 0x100;
            if (bVar9) {
                // (u8 *) &DAT_1050_1050
                PTR_LOOP_1050_5f50 = 0x1050;
                PTR_LOOP_1050_5f48 = param_5;
                PTR_LOOP_1050_5f4a = SI_REG;
                HINSTANCE16_1050_5f4c = DI_REG;
                PTR_LOOP_1050_5f4e = param_4;
                LockSegment16(0xffff);
//                PTR_LOOP_1050_5f52 = (u8 *) (u32_var11 >> 0x10);
                PTR_LOOP_1050_5f84 = u32_var11;
                u32_var10 = GetVersion16();
//                PTR_LOOP_1050_5f52 = (u8 *) (u32_var11 >> 0x10);
                PTR_LOOP_1050_5f84 = u32_var11;
//                PTR_LOOP_1050_5f80 = (u8 *) CONCAT11((char) u32_var10,
//                                                     (char) (u32_var10 >> 0x8));
                PTR_LOOP_1050_5f80 = u32_var10;
                code8 func_ptr_3 = (code8) swi(0x21);
                u32_var12 = u32_var11;
                u32_var11 = func_ptr_3();
//                PTR_LOOP_1050_5f52 = (u8 *) (u32_var12 >> 0x10);
                PTR_LOOP_1050_5f84 = u32_var12;
//                DAT_1050_5f82 = CONCAT11((char) u32_var11,
//                                          (char) (u32_var11 >> 0x8));
                DAT_1050_5f82 = u32_var11;
                DAT_1050_5f87 = 0x0;
                WaitEvent16(0x0);
                PTR_LOOP_1050_5f84 = u32_var11;
                param_3 = InitApp16(HINSTANCE16_1050_5f4c);
                PTR_LOOP_1050_5f84 = u32_var11;
                if (param_3 != 0x0) {
                    break;
                }
            }
        }
//        param_3 = CONCAT11((char) ( param_3 >> 0x8),
//                           0xff);
        param_3 = ((param_3 >> 0x8) << 0x8) | 0xff;
        pass1_1000_24db(param_3);
        PTR_LOOP_1050_5f84 = (u8 *) u32_var11;
    } while (true);
    // &DAT_1050_1050
    dos3_call_1000_23ea( param_4,
                         0x1050,
                        0x0,
                         0x1050);
    PTR_LOOP_1050_5f84 = u32_var11;
    // (u8 *) s_tile2_bmp_1050_1538
    pass1_1000_262c( u32_var11, 0x238d, 0x1538);
    PTR_LOOP_1050_5f84 = (u8 *) u32_var11;
    pass1_1000_27d6( u32_var11);
    u32_var10 = ret_op_1000_55ac();
    uVar4 =  u32_var10;
    init_1000_23be( param_5,
                    (uVar10 >> 0x10));
    fn_ptr_op_1000_24cd(uVar4);
    paVar13 = (astruct_825 *) CONCAT22(uVar4,
                                       0x15);
    pass1_1000_25a8();
    pass1_1000_2913(0x15);
    string_var4 = poss_str_op_1000_28dc(paVar13);
    if (string_var4 != NULL) {
        iVar5 = 0x9;
        if (string_var4[0] == 'M') {
            iVar5 = 0xf;
        }
        string_var4 = string_var4 + iVar5;
        iVar6 = 0x22;
        paVar8 = string_var4;
        do {
            if (iVar6 == 0x0) {
                break;
            }
            iVar6 += -0x1;
            paVar1 = paVar8;
            paVar8 = paVar8 + 0x1;
        } while (paVar1->field0_0x0 != '\r');
        (paVar8 + -0x1)->field0_0x0 = '\0';
    }
    FatalAppExit16((char *) CONCAT22(0x1050,
                                     string_var4),
                   0x0);
    FatalExit();
    piVar7 = (i16 *) &PTR_LOOP_1050_63fe;
    do {
        piVar1 = piVar7;
        piVar7 = piVar7 + 0x1;
        piVar4 = piVar7;
        if ((*piVar1 == param_2) || (piVar4 = (i16 *) (*piVar1 + 0x1), piVar4 == NULL)) {
            return piVar4;
        }
        iVar6 = -0x1;
        do {
            if (iVar6 == 0x0) {
                break;
            }
            iVar6 += -0x1;
            piVar2 = piVar7;
            piVar7 = (i16 *) ( piVar7 + 0x1);
        } while (*(char *) piVar2 != '\0');
    } while (true);
}
