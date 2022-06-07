//
// Created by cyrex on 6/6/2022.
//

#include "ops_1.h"
#include "types.h"
#include "func_ptrs.h"
#include "globals.h"
#include "sys_api.h"
#include "utils.h"
#include "structs_2.h"
#include "ops_3.h"
#include "ops_5.h"

void pass1_1000_24db(u16 param_1)
{
    code *pcVar1;
    //    i16 BP_REG;
    i16 iVar2;

    iVar2 = BP_REG + 0x1;
    u8_1050_5fc9 = '\0';
    fn_ptr_op_1000_2594();
    fn_ptr_op_1000_2594();
    dos3_op_1000_256b();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)(iVar2);
}

void fn_ptr_op_1000_2594()
{
    code **ppcVar1;
    //    code **SI_REG;
    //    code **DI_REG;
    code *ppcVar2;
    code *fn_ptr_1;

    while (SI_REG < DI_REG) {
        ppcVar2 = DI_REG + -0x2;
        ppcVar1 = DI_REG + -0x1;
        DI_REG = ppcVar2;
        //        if ((ppcVar2 | ppcVar1) != 0x0) {
        if (ppcVar2 != 0) {
            fn_ptr_1 = ppcVar2;
            (*fn_ptr_1)();
        }
    }
}

void dos3_op_1000_256b()
{
    code *pfn_var1;

    if (PTR_LOOP_1050_6202 != NULL) {
        (PTR_LOOP_1050_6200)();
    }
    pfn_var1 = (code *) swi(0x21);
    (*pfn_var1)();
}

//dos3_call_1000_23ea( param_4,
//                         &DAT_1050_1050,
//                        0x0,
//                         &DAT_1050_1050);
void dos3_call_1000_23ea(u8 *a,
                         u16 b,
                         u16 c,
                         u16 d)
{

}

void pass1_1000_262c(u8 *param_1,
                     u8 *param_2,
                     u8 *param_3)
{
    char *pcVar1;
    char cVar2;
    i16 iVar3;
    i16 iVar5;
    u16 uVar4;
    u16 uVar5;
    u16 uVar6;
    u16 uVar7;
    u16 uVar8;
    u16 uVar9;
    u16 uVar10;
    u16 uVar11;
    i16 iVar4;
    u8 *ppcVar6;
    u16 pcVar7;
    char *pcVar8;
    u8 *pcVar9;
    //    u16 ES_REG;
    u16 uVar12;
    u16 uVar3;
    u8 *puVar3;
    u16 stack0x0004;
    u16 stack0x0006;
    u16 stack0x0008;
    u16 stack0xfffe;

    PTR_LOOP_1050_5fd2 = param_2;
    PTR_LOOP_1050_5fd4 = param_3;
    param_3 = (u8 *) 0x263d;
    param_2 = (u8 *) pass1_1000_2950(0x8,
                                     param_1,
                                     0x104);
    param_3 = param_1;
    PTR_LOOP_1050_5fc2 = param_2;
    PTR_LOOP_1050_5fc4 = param_1;
    iVar5 = GetModuleFileName16(0x104,
                                (char *) CONCAT22(param_1,
                                                  param_2),
                                HINSTANCE16_1050_5f4c);
    param_2[iVar5] = '\0';
    iVar4 = 0x1;
    //    PTR_LOOP_1050_5fb8 = (u8 *) ( &PTR_LOOP_1050_0000 + 0x1);
    PTR_LOOP_1050_5fb8 = 0x1;
    //    pcVar7 = (char *) ( s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
    pcVar7 = 0x0081;
    LAB_1000_266c:
    do {
        do {
            pcVar1 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
            cVar2 = *pcVar1;
        } while (cVar2 == ' ');
    } while (cVar2 == '\t');
    if ((cVar2 != '\r') && (cVar2 != '\0')) {
        PTR_LOOP_1050_5fb8 = PTR_LOOP_1050_5fb8 + 0x1;
        do {
            pcVar7 = pcVar7 + -0x1;
            LAB_1000_267f:
            pcVar1 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
            cVar2 = *pcVar1;
            if ((cVar2 == ' ') || (cVar2 == '\t')) {
                goto LAB_1000_266c;
            }
            if ((cVar2 == '\r') || (cVar2 == '\0')) {
                break;
            }
            if (cVar2 == '\"') {
                LAB_1000_26b8:
                do {
                    while (true) {
                        while (true) {
                            pcVar1 = pcVar7;
                            pcVar7 = pcVar7 + 0x1;
                            cVar2 = *pcVar1;
                            if ((cVar2 == '\r') || (cVar2 == '\0')) {
                                goto LAB_1000_26e8;
                            }
                            if (cVar2 == '\"') {
                                goto LAB_1000_267f;
                            }
                            if (cVar2 == '\\') {
                                break;
                            }
                            iVar4 += 0x1;
                        }
                        uVar7 = 0x0;
                        do {
                            pcVar9 = pcVar7;
                            uVar7 += 0x1;
                            pcVar7 = pcVar9 + 0x1;
                            // TODO
                            //                            cVar2 = *pcVar9;
                        } while (cVar2 == '\\');
                        if (cVar2 == '\"') {
                            break;
                        }
                        iVar4 += uVar7;
                        pcVar7 = pcVar9;
                    }
                    iVar4 = iVar4 + (uVar7 >> 0x1) + ((uVar7 & 0x1) != 0x0);
                } while ((uVar7 & 0x1) != 0x0);
                goto LAB_1000_267f;
            }
            if (cVar2 != '\\') {
                iVar4 += 0x1;
                goto LAB_1000_267f;
            }
            uVar6 = 0x0;
            do {
                uVar6 += 0x1;
                pcVar1 = pcVar7;
                pcVar7 = pcVar7 + 0x1;
                cVar2 = *pcVar1;
            } while (cVar2 == '\\');
            if (cVar2 == '\"') {
                iVar4 = iVar4 + (uVar6 >> 0x1) + ((uVar6 & 0x1) != 0x0);
                if ((uVar6 & 0x1) == 0x0) {
                    goto LAB_1000_26b8;
                }
                goto LAB_1000_267f;
            }
            iVar4 += uVar6;
        } while (true);
    }
    LAB_1000_26e8:
    //    param_3 = (u8 *) &DAT_1050_1050;
    param_3 = 0x1050;
    // TODO
    //    iVar3 = -((PTR_LOOP_1050_5fb8 + (PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + iVar4 + 0x1) & 0xfffe);
    PTR_LOOP_1050_5fba = &stack0x0004 + iVar3;
    //    PTR_LOOP_1050_5fbc = (u8 *) &DAT_1050_1050;
    PTR_LOOP_1050_5fbc = 0x1050;
    // TODO
    //    pcVar9 = &stack0x0004 + (PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + iVar3;
    //    (&param_3 + iVar3) = &DAT_1050_1050;
    *(u16 *) (param_3 + iVar3) = 0x1050;
    puVar3 = PTR_LOOP_1050_5fc4;
    uVar12 = (param_3 + iVar3);
    *(u8 **) (&stack0x0006 + iVar3) = puVar3;
    *(u8 **) (&stack0x0004 + iVar3) = PTR_LOOP_1050_5fc2;
    ppcVar6 = (u8 *) (stack0x0008 + iVar3);
    *(u8 **) (&param_3 + iVar3) = &stack0x0004 + iVar3;
    //    (&param_2 + iVar3) = s_tile2_bmp_1050_1538;
    *(u16 *) (param_2 + iVar3) = 0x1538;
    *(u16 *) (stack0xfffe + iVar3) = 0x271f;
    //   0 uVar4 = pass1_1000_29dc(&DAT_1050_1050);
    uVar4 = pass1_1000_29dc(0x1050);
    uVar3 = &PTR_LOOP_1050_5f7e;
    //    pcVar7 = (char *) (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
    pcVar7 = 0x0081;
    LAB_1000_272e:
    do {
        do {
            pcVar1 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
            cVar2 = *pcVar1;
        } while (cVar2 == ' ');
    } while (cVar2 == '\t');
    if ((cVar2 == '\r') || (cVar2 == '\0')) {
        LAB_1000_27c1:
        // TODO:
        //        ( &param_3 + iVar3) =  s_tile2_bmp_1050_1538;
        *(u16 *) (param_3 + iVar3) = 0x1538;
        *(u16 *) (param_2 + iVar3) = 0x27c5;
        //        uVar5 = pass1_1000_29dc( &DAT_1050_1050);
        uVar5 = pass1_1000_29dc(0x1050);
        *ppcVar6 = NULL;
        ppcVar6[0x1] = NULL;
        // WARNING: Could not recover jumptable at 0x100027d2. Too many branches
        // WARNING: Treating indirect jump as call
        ((code) (u32) &PTR_LOOP_1050_5fd2)();
        //        PTR_LOOP_1050_5fc2 = CONCAT22(PTR_LOOP_1050_5fc4,
        //                                       PTR_LOOP_1050_5fc2);
        return;
    }
    ppcVar6[0] = pcVar9;
    //    ppcVar6[0x1] = (char *) &DAT_1050_1050;
    ppcVar6[1] = 0x1050;
    ppcVar6 = ppcVar6 + 0x2;
    do {
        pcVar7 = pcVar7 + -0x1;
        LAB_1000_274f:
        pcVar1 = pcVar7;
        pcVar7 = pcVar7 + 0x1;
        cVar2 = *pcVar1;
        if ((cVar2 == ' ') || (cVar2 == '\t')) {
            pcVar1 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
            *pcVar1 = '\0';
            goto LAB_1000_272e;
        }
        if ((cVar2 == '\r') || (cVar2 == '\0')) {
            LAB_1000_27be:

            //            *pcVar9 = '\0';
            pcVar9[0] = 0;
            goto LAB_1000_27c1;
        }
        pcVar8 = pcVar7;
        if (cVar2 == '\"') {
            LAB_1000_278b:
            while (true) {
                pcVar7 = pcVar8 + 0x1;
                cVar2 = *pcVar8;
                if ((cVar2 == '\r') || (cVar2 == '\0')) {
                    goto LAB_1000_27be;
                }
                if (cVar2 == '\"') {
                    break;
                }
                if (cVar2 == '\\') {
                    uVar10 = 0x0;
                    do {
                        pcVar8 = pcVar7;
                        uVar10 += 0x1;
                        pcVar7 = pcVar8 + 0x1;
                        cVar2 = *pcVar8;
                    } while (cVar2 == '\\');
                    if (cVar2 == '\"') {
                        for (uVar11 = uVar10 >> 0x1; uVar11 != 0x0; uVar11 -= 0x1) {
                            pcVar1 = pcVar9;
                            pcVar9 = pcVar9 + 0x1;
                            *pcVar1 = '\\';
                        }
                        if ((uVar10 & 0x1) == 0x0) {
                            break;
                        }
                        pcVar1 = pcVar9;
                        pcVar9 = pcVar9 + 0x1;
                        *pcVar1 = '\"';
                        pcVar8 = pcVar7;
                    } else {
                        for (; uVar10 != 0x0; uVar10 -= 0x1) {
                            pcVar1 = pcVar9;
                            pcVar9 = pcVar9 + 0x1;
                            *pcVar1 = '\\';
                        }
                    }
                } else {
                    pcVar1 = pcVar9;
                    pcVar9 = pcVar9 + 0x1;
                    *pcVar1 = cVar2;
                    pcVar8 = pcVar7;
                }
            }
            goto LAB_1000_274f;
        }
        if (cVar2 != '\\') {
            pcVar1 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
            *pcVar1 = cVar2;
            goto LAB_1000_274f;
        }
        uVar8 = 0x0;
        do {
            uVar8 += 0x1;
            pcVar1 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
            cVar2 = *pcVar1;
        } while (cVar2 == '\\');
        if (cVar2 == '\"') {
            for (uVar9 = uVar8 >> 0x1; uVar9 != 0x0; uVar9 -= 0x1) {
                pcVar1 = pcVar9;
                pcVar9 = pcVar9 + 0x1;
                *pcVar1 = '\\';
            }
            pcVar8 = pcVar7;
            if ((uVar8 & 0x1) == 0x0) {
                goto LAB_1000_278b;
            }
            pcVar1 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
            *pcVar1 = '\"';
            goto LAB_1000_274f;
        }
        for (; uVar8 != 0x0; uVar8 -= 0x1) {
            pcVar1 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
            *pcVar1 = '\\';
        }
    } while (true);
}

u16 *pass1_1000_2950(i16 param_1,
                     u16 param_2,
                     u16 param_3)
{
    u16 *puVar1;
    u16 uVar2;
    char *pcVar3;
    u8 *puVar4;
    char *pcVar5;
    i16 iVar6;
    u16 *puVar7;
    u16 *puVar8;
    //    u16 BP_REG;
    char *pcVar9;
    //    u16 ES_REG;
    struct astruct_825 *paVar10;

    puVar4 = PTR_LOOP_1050_6066;
    //    PTR_LOOP_1050_6066 = (u8 *) &PTR_LOOP_1050_1000;
    PTR_LOOP_1050_6066 = 0;
    puVar8 = (u16 *) mem_1000_167a(param_2,
                                   param_3);
    PTR_LOOP_1050_6066 = puVar4;
    //    if ((param_2 | puVar8) != 0x0) {
    if (puVar8 != 0) {
        return puVar8;
    }
    paVar10 = (astruct_825 *) CONCAT22(ES_REG,
                                       param_1);
    pass1_1000_25a8();
    pass1_1000_2913(param_1);
    pcVar5 = poss_str_op_1000_28dc(paVar10);
    if (pcVar5 != NULL) {
        iVar6 = 0x9;
        if (*pcVar5 == 'M') {
            iVar6 = 0xf;
        }
        pcVar5 = pcVar5 + iVar6;
        iVar6 = 0x22;
        pcVar9 = pcVar5;
        do {
            if (iVar6 == 0x0) {
                break;
            }
            iVar6 += -0x1;
            pcVar3 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
        } while (*pcVar3 != '\r');
        pcVar9[-0x1] = '\0';
    }
    FatalAppExit16((char *) CONCAT22(0x1050,
                                     pcVar5),
                   0x0);
    FatalExit();
    puVar8 = (u16 *) &PTR_LOOP_1050_63fe;
    do {
        puVar1 = puVar8;
        puVar8 = puVar8 + 0x1;
        uVar2 = *puVar1;
        puVar7 = puVar8;
        if ((uVar2 == BP_REG) || (puVar7 = (u16 *) (uVar2 + 0x1), puVar7 == NULL)) {
            return puVar7;
        }
        iVar6 = -0x1;
        do {
            if (iVar6 == 0x0) {
                break;
            }
            iVar6 += -0x1;
            puVar1 = puVar8;
            puVar8 = (u16 *) (puVar8 + 0x1);
        } while (*(char *) puVar1 != '\0');
    } while (true);
}

u16 mem_1000_167a(u16 param_1,
                  u16 param_2)
{
    u8 *puVar1;
    //    u16 in_register_0000000a;
    StructD *pSVar2;
    i32 lVar3;

    //    pSVar2 = (StructD *) CONCAT22(in_register_0000000a,
    //                                  param_1);
    pSVar2 = (StructD *) param_1;
    //    if (( PTR_LOOP_1050_5f2e |  PTR_LOOP_1050_5f2c) == 0x0) {
    if (PTR_LOOP_1050_5f2c == 0) {
        puVar1 = mem_op_1000_160a(pSVar2);
        //        if (( pSVar2 |  puVar1) == 0x0) {
        if (pSVar2 == 0) {
            return 0x0;
        }
    }
    lVar3 = mem_op_1000_0a48(0x0,
                             param_2,
                             0x0,
                             CONCAT22(PTR_LOOP_1050_5f2e,
                                      PTR_LOOP_1050_5f2c));
    return lVar3;
}

u8 *mem_op_1000_160a(StructD *param_1)
{
    u8 *puVar1;
    u16 uVar1;
    u32 uVar2;

    uVar1 = param_1;
    puVar1 = (u8 *) ret_true_1000_2146();
    if (puVar1 == NULL) {
        return puVar1;
    }
//    if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0x0) {
        if (PTR_LOOP_1050_5f2c == 0x0) {
        DAT_1050_5f30 = 0x1;
        DAT_1050_5f32 = 0x1;
        uVar2 = mem_op_1000_18ec(DAT_1050_5f46,
                                 uVar1);
        PTR_LOOP_1050_5f2e = (u8 *) (uVar2 >> 0x10);
        PTR_LOOP_1050_5f2c = (u8 *) uVar2;
//        if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) != 0x0) {
        if (PTR_LOOP_1050_5f2c != 0) {
            if (PTR_LOOP_1050_5f42 != NULL) {
                pass1_1000_1a54(PTR_LOOP_1050_5f42,
                                PTR_LOOP_1050_5f2c,
                                PTR_LOOP_1050_5f2e);
            }
            if (DAT_1050_5f44 != 0xffff) {
                pass1_1000_1afe(DAT_1050_5f44,
                                PTR_LOOP_1050_5f2c,
                                PTR_LOOP_1050_5f2e);
            }
        }
    }
    empty_fn_1000_214a();
    return PTR_LOOP_1050_5f2c;
}

void empty_fn_1000_214a()
{
    return;
}

i32 mem_op_1000_0a48(u8 param_1,
                     u16 param_2,
                     i16 param_3,
                     astruct_7 *param_4)
{
    u16 uVar1;
    u16 *puVar2;
    u16 uVar4;
    u16 uVar3;
    //    u16 UVar4;
//    u16 CS_REG;
    u32 uVar5;
    u8 in_stack_00000005;
    u16 *puVar1;

    //    UVar4 =  (param_4 >> 0x10);
    if ((param_4->field_0x14) == -0x4153) {
        // (s_version__d__d_1050_0012 + 0x6)
        if ((param_3 == 0x0) && (param_2 <= 0x0018)) {
            if (param_2 == 0x0) {
                pass1_1000_1e61(CS_REG,
                                0x4,
                                param_4,
                                0);
                uVar5 = 0x0;
            } else {
                uVar5 = mem_op_1000_0838(0x0);
                uVar3 = (uVar5 >> 0x10);
                puVar2 = (u16 *) uVar5;
                if ((uVar5 != 0x0) && ((param_1 & 0x1) != 0x0)) {
                    // (s_version__d__d_1050_0012 + 0x6)
                    uVar1 = 0x0018;
                    for (uVar4 = uVar1 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
                        puVar1 = puVar2;
                        puVar2 = puVar2 + 0x1;
                        *puVar1 = 0x0;
                    }
                    if ((uVar1 & 0x1) != 0x0) {
                        *(u8 *) puVar2 = 0x0;
                    }
                }
            }
        }
            // (s_version__d__d_1050_0012 + 0xa))
        else if ((param_3 == 0x0) && (param_2 <= 0x001c)) {
            uVar5 = mem_op_1000_0b20(param_1 & 0xfffd,
                                     0x0,
                                     param_2);
        } else {
            uVar5 = mem_op_1000_05e2(param_2,
                                     param_3,
                                     param_1 & 0xfffd,
                                     0x0);
        }
        return uVar5;
    }
    pass1_1000_1e61(CS_REG,
                    0xa,
                    0x0,
                    0);
    return 0x0;
}

u16 pass1_1000_29dc(u16 param_1)
{
    if (___EXPORTEDSTUB != (code) 0xb8) {
        //        return (u16) & DAT_1050_1050;
        return 0x1050;
    }
    //    return uRam100029ed;
    return 0x29ed;
}

u16 ___EXPORTEDSTUB()
{
    return 0x0;
}

u32 mem_op_1000_0b20(u16 param_1,
                     struct_1000_0b20 *pstruct_param_2,
                     u16 param_3)
{
    u16 *puVar1;
    u16 uVar2;
    u16 uVar3;
    u16 uVar4;
    u16 uVar5;
    u16 UVar6;
    u16 *pu16_var7;
    u16 uVar8;
    bool bVar9;
    u32 uVar10;
    u16 uStack20;
    u16 *puStack6;

    uVar8 = 0x1050;
    uVar2 = param_1 & 0x2;
    uVar4 = param_3 + 0x5 & 0xfffc;
    uVar4 = uVar4 - 0x8 & ~-(uVar4 < 0x8);
    uVar5 = uVar4 + 0x8;
    pu16_var7 = (u16 *) (uVar2 * 0x2 + pstruct_param_2);
    uStack20 = param_1;
    puStack6 = pu16_var7;
    if (pu16_var7 == NULL) {
        goto LAB_1000_0b64;
    }
    do {
        do {
            if ((uVar5 <= *pu16_var7) && (uVar10 = pass1_1000_0c32(uVar5,
                                                                   uStack20,
                                                                   0x0), uVar10 != 0x0)) {
                // TODO:
                //                (u16 *) (uVar2 * 0x2 + pstruct_param_2.field_0x0) = pu16_var7;
                return uVar10;
            }
            pu16_var7 = (u16 *) pu16_var7[0x2];
        } while (pu16_var7 != puStack6);
        LAB_1000_0b64:
        if (((((uStack20 & 0x2) == 0x0) || ((uStack20 & 0x40) != 0x0)) || ((pstruct_param_2->field_0x32) == 0x0))
            || (uVar3 = pstruct_param_2->field_0x32(), uVar3 < uVar5)) {
            if (((uStack20 & 0x10) != 0x0) || (uVar3 = uVar2, UVar6 = mem_op_1000_03c6((pstruct_param_2->field_0x1a),
                                                                                       0x0,
                                                                                       uVar2,
                                                                                       pstruct_param_2,
                                                                                       0x0,
                                                                                       '\0'), (UVar6 | uVar3) == 0x0)) {
                if ((uStack20 & 0x20) == 0x0) {
                    uVar2 = uVar4 + 0x1007 & 0xf000;
                    puVar1 = (u16 *) (pstruct_param_2 + 0x1e);
                    uVar4 = uVar2 + *puVar1;
                    uVar2 = (pstruct_param_2 + 0x20) + CARRY2(uVar2,
                                                              *puVar1);
                    puVar1 = (u16 *) (pstruct_param_2 + 0x28);
                    bVar9 = uVar2 < *puVar1;
                    if ((bVar9 || uVar2 == *puVar1)
                        && ((bVar9 || (puVar1 = (u16 *) (pstruct_param_2 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) {
                        uVar10 = mem_op_1000_05e2(uVar5,
                                                  0x0,
                                                  uStack20,
                                                  pstruct_param_2);
                        return uVar10;
                    }
                }
                return 0x0;
            }
        } else {
            uStack20 |= 0x40;
        }
        pu16_var7 = (u16 *) (uVar2 * 0x2 + pstruct_param_2);
        puStack6 = (u16 *) pu16_var7[0x2];
    } while (true);
}

u16 mem_op_1000_03c6(u16 param_1,
                     i16 param_2,
                     u16 param_3,
                     u16 param_4,
                     u8 param_5,
                     u16 param_6)
{
    u16 *puVar1;
    i16 *piVar2;
    u16 uVar3;
    u16 uVar4;
    u16 *puVar5;
    u16 UVar6;
    u16 uVar7;
    u16 unaff_CS;
    bool bVar8;
    DWORD DVar9;
    u16 uVar10;
    u16 uStack20;
    u16 uVar9;

    uVar7 = CONCAT11(param_6,
                     param_5);
    uVar3 = param_1 + 0xfff & 0xf000;
    puVar1 = (u16 *) (param_4 + 0x1e);
    uVar4 = uVar3 + *puVar1;
    uVar3 = param_2 + (0xf000 < param_1) + (param_4 + 0x20) + CARRY2(uVar3,
                                                                     *puVar1);
    puVar1 = (u16 *) (param_4 + 0x28);
    bVar8 = uVar3 < *puVar1;
    if ((bVar8)
        || ((bVar8 || uVar3 == *puVar1 && (puVar1 = (u16 *) (param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) {
        if (param_3 == 0x3) {
            uStack20 = ((u8) (-((param_5 & 0x1) != 0x0) >> 0x8) & 0x1 | 0x20) << 0x8;
        } else {
            uStack20 = 0x1000;
        }
        uStack20 = (param_4 + 0x16) | uStack20;
        mem_op_1000_131c(uStack20,
                         CONCAT22(param_2,
                                  param_1));
        if ((uVar3 | uStack20) != 0x0) {
            puVar5 = (u16 *) mem_op_1000_0308(param_3,
                                              param_4);
            if (puVar5 != NULL) {
                puVar5[0x4] = uStack20;
                puVar5[0x5] = uVar3;
                PTR_LOOP_1050_000c = param_3 | 0xcad0;
                //        *NULL = param_4;
                //        &u16_1050_0002 = &DAT_1050_1050;
                u16_1050_0002 = 0x1050;
                u32_1050_0004 = puVar5;
                // (&u32_1050_0004 + 0x2) = &DAT_1050_1050;
                u32_1050_0006 = 0x1050;
                PTR_LOOP_1050_000a = 0x0;
                // SUB42(&DAT_1050_1050,0x0);
                uVar10 = 0x1050;
                DVar9 = mem_op_1000_1532(uStack20,
                                         uVar3);
                UVar6 = DVar9;
                if (param_3 == 0x1) {
                    // &DAT_1050_1050
                    uVar7 = pass1_1000_0782(param_4,
                                            UVar6,
                                            0x0,
                                            0x1050);
                } else if (param_3 == 0x3) {
                    pass1_1000_05b4(param_5,
                                    0x0);
                } else {
                    uVar7 = pass1_1000_09ca(UVar6,
                                            NULL);
                }
                param_2 = (DVar9 >> 0x10);
                *puVar5 = uVar7;
                puVar5[0x1] = 0x8000;
                puVar1 = (u16 *) (param_4 + 0x1e);
                uVar4 = *puVar1;
                *puVar1 = *puVar1 + UVar6;
                piVar2 = (i16 *) (param_4 + 0x20);
                *piVar2 = *piVar2 + param_2 + CARRY2(uVar4,
                                                     UVar6);
                return uVar3;
            }
            mem_op_1000_13ce(uStack20,
                             uVar3);
        }
    } else {
        // &DAT_1050_1050
        pass1_1000_1e61(unaff_CS,
                        0x7,
                        param_4,
                        0);
    }
    return 0x0;
}

u32 mem_op_1000_05e2(u16 param_1,
                     i16 param_2,
                     u16 param_3,
                     astruct_7 *param_4)
{
    u16 *puVar1;
    i16 iVar2;
    u16 uVar3;
    u16 uVar4;
    u16 UVar5;
    u32 UVar6;
    u16 unaff_CS;
    bool bVar5;
    u32 uVar6;

    iVar2 = param_2 + (0xffeb < param_1);
    do {
        uVar3 = 0x3;
        UVar6 = param_3;
        UVar6 = mem_op_1000_03c6(param_1 + 0x14,
                                 iVar2,
                                 0x3,
                                 param_4,
                                 UVar6,
                                 UVar6);
        if ((UVar6 | uVar3) != 0x0) {
            return CONCAT22(UVar6,
                            uVar3 + 0x14);
        }
        uVar6 = mem_op_1000_0052(param_4);
        uVar3 = param_1 + 0x1013 & 0xf000;
        puVar1 = (u16 *) (param_4 + 0x1e);
        uVar4 = uVar3 + *puVar1;
        uVar3 = iVar2 + (0xf000 < param_1 + 0x14) + (param_4 + 0x20) + CARRY2(uVar3,
                                                                              *puVar1);
        puVar1 = (u16 *) (param_4 + 0x28);
        bVar5 = uVar3 < *puVar1;
        // &DAT_1050_1050)
    } while (((bVar5 || uVar3 == *puVar1)
        && ((bVar5 || (puVar1 = (u16 *) (param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) && ((uVar6 != 0x0
        || (UVar5 = pass1_1000_1e61(unaff_CS,
                                    0x2,
                                    param_4,
                                    0), UVar5 != 0x0))));
    return 0x0;
}

i32 mem_op_1000_13ce(u16 param_1,
                     u16 param_2)
{
    HGLOBAL16 HVar1;
    u16 uVar2;
    DWORD DVar3;

    DVar3 = GlobalHandle16(param_2);
    uVar2 = (DVar3 >> 0x10);
    if ((HGLOBAL16) DVar3 != 0x0) {
        HVar1 = GlobalFree16((HGLOBAL16) DVar3);
        return CONCAT22(uVar2,
                        (HVar1 == 0x0));
    }
    return (u32) uVar2 << 0x10;
}

void pass1_1000_25a8()
{
    pass1_1000_2913(0xfc);
    pass1_1000_2913(0xff);
    return;
}

void pass1_1000_2913(i16 param_1)
{
    char *pcVar1;
    char *pcVar2;
    i16 iVar3;
    u16 unaff_DI;
    u16 unaff_ES;
    struct astruct_825 *paVar4;
    i16 iVar5;

//    iVar5 = (i16) &DAT_1050_1050;
    iVar5 = 0x1050;
    if (u16_1050_61ec != 0x0) {
        paVar4 = (astruct_825 *) CONCAT22(unaff_DI,
                                          param_1);
        pcVar2 = poss_str_op_1000_28dc(paVar4);
        if (pcVar2 != NULL) {
            iVar3 = -0x1;
            do {
                if (iVar3 == 0x0) {
                    break;
                }
                iVar3 += -0x1;
                pcVar1 = pcVar2;
                pcVar2 = pcVar2 + 0x1;
            } while (*pcVar1 != '\0');
            pass1_1000_55b1(((u32) paVar4 >> 0x10),
                            iVar5);
        }
    }
    return;
}

char *poss_str_op_1000_28dc(astruct_825 *param_1)
{
    astruct_825 **ppaVar1;
    char *piVar2;
    i16 iVar3;
    char *string_var3;
    astruct_825 *pstruct_var2;

    string_var3 = PTR_LOOP_1050_63fe;
    do {
        ppaVar1 = (astruct_825 **) string_var3;
        string_var3 = (char*)(string_var3 + 0x2);
        pstruct_var2 = *ppaVar1;
        piVar2 = string_var3;
        if ((pstruct_var2 == (astruct_825 *) param_1) || (piVar2 = (char*)(pstruct_var2->field_0x1), piVar2 == NULL)) {
            return (char*)(astruct_825 * *)
            piVar2;
        }
        iVar3 = -0x1;
        do {
            if (iVar3 == 0x0) {
                break;
            }
            iVar3 += -0x1;
            ppaVar1 = (astruct_825 **) string_var3;
            string_var3 = (char*)(string_var3 + 0x1);
        } while (*(char *) ppaVar1 != '\0');
    } while (true);
}

u16 ret_true_1000_2146()
{
    return 0x1;
}

u32 mem_op_1000_18ec(u16 param_1,
                     u16 param_2)
{
    u32 uVar1;

    uVar1 = mem_op_1000_1902(param_2,
                             param_1,
                             0x0,
                             0x0,
                             0xc);
    return uVar1;
}

u16 pass1_1000_1a54(u16 param_1,
                    struct_1000_1a54 *param_2,
                    u16 param_3)
{
    u16 uVar1;
    u16 uVar2;
//    u16 CS_REG;

    if ((param_2->field_0x14) != -0x4153) {
        pass1_1000_1e61(CS_REG,
                        0xa,
                        0x0,
                        0);
        return 0x0;
    }
    uVar1 = pass1_1000_1ab0(param_1);
    if (uVar1 < (param_2->field_0x18) + 0x14U) {
        uVar2 = 0x0;
    } else {
        uVar2 = (param_2->field_0x1a);
        (param_2->field_0x1a) = uVar1;
        (param_2->field_0x1c) = uVar1 >> 0x2;
    }
    return uVar2;
}

BOOL16 pass1_1000_1afe(u16 param_1,
                       astruct_7 *param_2,
                       u16 param_3)
{
    u16 uVar1;
//    u16 CS_REG;

    if (param_1 == 0x0) {
        uVar1 = 0x0;
    } else {
        uVar1 = param_1 + 0x1 & 0xfffe;
    }
    if ((param_2->field_0x14) == -0x4153) {
        if ((uVar1 < param_1) || ((param_2->field_0x1a) - 0x14U < uVar1)) {
            pass1_1000_1e61(CS_REG,
                            0x3,
                            param_2,
                            0);
        } else if ((param_2 + 0x2) == 0x0) {
            param_2->field_0x18 = uVar1;
            return 0x1;
        }
        return 0x0;
    }
    pass1_1000_1e61(CS_REG,
                    0xa,
                    0x0,
                    0);
    return 0x0;
}

u32 pass1_1000_0c32(u16 param_1,
                    u16 param_2,
                    u16 param_3)
{
    u16 *puVar1;
    u8 *pbVar2;
    i16 *piVar3;
    u32 uVar4;
    u16 uVar5;
    u16 *puVar6;
    i16 iVar7;
    u16 *puVar8;
    u16 uVar9;
    u16 uStack14;
    u16 *puStack8;
    u16 uStack6;

    puVar8 = (u16 *) (param_3 + 0xe);
    uStack6 = 0x0;
    puVar6 = puVar8;
    while (true) {
        do {
            uVar5 = *puVar6;
            if (param_1 <= uVar5) {
                uVar5 = (uVar5 & 0xfffc) - param_1;
                puVar1 = (u16 *) (param_3 + 0x12);
                puStack8 = puVar6;
                if (*puVar1 < uVar5 || *puVar1 == uVar5) {
                    uStack14 = param_1;
                    if ((param_2 & 0x6) == 0x0) {
                        puStack8 = (u16 *) (uVar5 +  puVar6);
                        puStack8[-0x1] = uVar5;
                        *puVar6 = uVar5 | 0x2;
                        puVar8 = (u16 *) puVar6[0x1];
                        pbVar2 = (u8 *) ( puStack8 + param_1);
                        *pbVar2 = *pbVar2 | 0x2;
                        *puStack8 = param_1 | 0x1;
                    } else {
                        *puVar6 = param_1 & 0xff00 | *(u8 *) puVar6 & 0x2 | param_1 & 0xff | 0x1;
//                        (puVar6[0x2] + 0x2) = puVar6[0x1];
//                        (puVar6[0x1] + 0x4) = puVar6[0x2];
                        puVar8 = (u16 *) ( puVar6 + param_1);
//                        ( puVar8 + (uVar5 - 0x2)) = uVar5;
                        *puVar8 = uVar5 | 0x2;
                        uVar5 = (param_3 + 0x10);
                        puVar8[0x2] = uVar5;
                        puVar8[0x1] = (uVar5 + 0x2);
//                        (u16 *) ((uVar5 + 0x2) + 0x4) = puVar8;
//                        (u16 *) (uVar5 + 0x2) = puVar8;
                    }
                } else {
                    puVar8 = (u16 *) puVar6[0x1];
//                    (u16 *) (puVar6[0x2] + 0x2) = puVar8;
//                    (puVar6[0x1] + 0x4) = puVar6[0x2];
                    puVar1 = puVar6;
                    *(u8 *) puVar1 = *(u8 *) puVar1 | 0x1;
                    uStack14 = *puVar6 & 0xfffc;
//                    ( puVar6 + uStack14) = ( puVar6 + uStack14) | 0x2;
                }
//                (u16 *) (param_3 + 0xe) = puVar8;
                if ((param_2 & 0x1) != 0x0) {
                    puVar6 = puStack8;
                    for (uVar5 = uStack14 - 0x2 >> 0x1; puVar6 = puVar6 + 0x1, uVar5 != 0x0; uVar5 -= 0x1) {
                        *puVar6 = 0x0;
                    }
                    if ((uStack14 - 0x2 & 0x1) != 0x0) {
                        *(u8 *) puVar6 = 0x0;
                    }
                }
                if (((param_2 & 0x2) != 0x0) && (puVar8[0x1] == puVar8[0x2])) {
                    *(u16 *) (param_3 + 0x4) = *(u16 *) ((param_3 + 0x10) + 0x2) & 0xfffc;
                    uVar4 = (u32) (param_3 + 0x4);
                    pbVar2 = (u8 *) ( uVar4 + 0x3);
                    *pbVar2 = *pbVar2 | 0x80;
                }
                piVar3 = (i16 *) (param_3 + 0xa);
                *piVar3 = *piVar3 + 0x1;
                return CONCAT22(0x1050,
                                puStack8 + 0x1);
            }
            if (uStack6 < uVar5) {
                uStack6 = uVar5;
            }
            puVar6 = (u16 *) puVar6[0x1];
        } while (puVar6 != puVar8);
        if (((param_2 & 0x2) == 0x0) || ((param_2 & 0x40) != 0x0)) {
            break;
        }
        uVar4 = (u32) param_3;
        uVar9 =  ((u32) uVar4 >> 0x10);
        iVar7 =  uVar4;
        if ((iVar7 + 0x34) == 0x0) {
            break;
        }
//        uStack6 = ((code) (iVar7 + 0x34))();
        if ((uStack6 < param_1) || (puVar6 = (u16 *) (param_3 + 0xe), puVar6 == NULL)) {
            break;
        }
    }
    *(u16 *) (param_3 + 0x4) = uStack6 & 0xfffc;
    return 0x0;
}

u8 * mem_op_1000_0308(i16 param_1,
                      astruct_7 *pstruct7_param_2)
{
    u8 *pu8_var1;
    u8* i16_var2;
    bool b_var3;
    u8 extraout_AH;
    i16 *pi16_var4;

    if ((pstruct7_param_2 + 0xa) == 0x0) {
        b_var3 = mem_op_1000_01b0(pstruct7_param_2);
        if (CONCAT11(extraout_AH,
                     b_var3) == 0x0) {
            return 0x0;
        }
    }
    pu8_var1 = (pstruct7_param_2->field_0xa);
    (pstruct7_param_2->field_0xa) = (pu8_var1 + 0x4);
    pi16_var4 = (i16 *) (param_1 * 0x2 + pstruct7_param_2);
    if (*pi16_var4 == 0x0) {
        *(pu8_var1 + 0x6) = pu8_var1;
        *(pu8_var1 + 0x4) = pu8_var1;
    } else {
        i16_var2 = *pi16_var4;
        *(pu8_var1 + 0x6) = i16_var2;
        *(pu8_var1 + 0x4) = (i16_var2 + 0x4);
//        (u8*)((i16_var2 + 0x4) + 0x6) = pu8_var1;
//        (u8*)(i16_var2 + 0x4) = pu8_var1;
    }
    *pi16_var4 = pu8_var1;
    return pu8_var1;
}
