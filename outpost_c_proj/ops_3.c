//
// Created by cyrex on 6/6/2022.
//

#include "ops_2.h"
#include "structs_2.h"
#include "utils.h"
#include "sys_api.h"
#include "globals.h"
#include "func_ptrs.h"
#include "types.h"
#include "ops_1.h"
#include "ops_3.h"
#include "string_defs.h"
#include "ops_4.h"
#include "ops_5.h"

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

u32 mem_op_1000_0838(astruct_7 *param_1)
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
//    u16 CS_REG;
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
                    PTR_LOOP_1050_000e = puVar4;
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
            pass1_1000_1e61(CS_REG,
                            0x4,
                            param_1, 0);
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
                UVar7 = pass1_1000_1e61(CS_REG,
                                        0x2,
                                        param_1, 0);
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

DWORD mem_op_1000_1532(astruct_7 *param_1,
                       i16 selector)
{
    DWORD mem_size;

    // get handle to global memory
    mem_size = GlobalHandle16(selector);
    if ((HGLOBAL16) mem_size != 0x0) {
        // get size of memory area
        mem_size = GlobalSize16((HGLOBAL16) mem_size);
        return mem_size;
    }
    return 0x0;
}

u16 pass1_1000_0782(u16 param_1,
                    u16 param_2,
                    astruct_1000_0782 *param_3,
                    u16 param_4)
{
    (param_3->field_0xe) = 0x0;
    (param_3->field_0x10) = param_3->field_0x14;
    (param_3->field_0x8) = 0x9a0;
    pass1_1000_07ac((param_1 + 0x18),
                    param_2,
                    param_3);
    return 0x1;
}

void pass1_1000_05b4(u8 param_1,
                     u16 *param_2)
{
    *(param_2 + 0xa) = 0x1;
    *(param_2 + 0x8) = 0x668;
    *(param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
    *(param_2 + 0x10) = 0x0;
    *(param_2 + 0xe) = 0x0;
}

u16 pass1_1000_09ca(i16 param_1,
                    u16 *param_2)
{
    u16 *puVar1;
    i16 iVar2;
    u32 uVar3;
    u16 *puVar4;

    puVar1 = param_2 + 0xa;
//    puVar4 = (u16 *) (( param_2 + (param_1 -  puVar1) + -0x6 & 0xfffcU) +  puVar1);
    *puVar4 = 0x1;
    param_2[0x7] =  puVar1;
    puVar4[0x2] =  puVar4;
    puVar4[0x1] =  puVar4;
    param_2[0x8] =  puVar4;
    if ((*(u8 *) (param_2 + 0x6) & 0x7) == 0x2) {
        param_2[0x9] = 0x8;
    } else {
        uVar3 = (u32) param_2;
        iVar2 = ( uVar3 + 0x18);
        param_2[0x9] = (iVar2 - 0x5U & ~- (iVar2 + 0x3U < 0x8)) + 0x8;
    }
    puVar4[-0x1] =  puVar4 -  puVar1;
    *puVar1 =  puVar4 -  puVar1 | 0x2;
    param_2[0xc] =  puVar4;
    param_2[0xb] = puVar4[0x1];
//    (u16 *) (puVar4[0x1] + 0x4) = puVar1;
    puVar4[0x1] =  puVar1;
    param_2[0x4] = 0xe08;
    return *puVar1 & 0xfffc;
}


u32 mem_op_1000_0052(astruct_7 *param_1)
{
    u16 *puVar1;
    u16 uVar2;
    i16 iVar3;
    u32 uVar4;
    u32 uVar5;
    i16 iStack14;
    i16 iStack12;
    i16 iStack10;
    u16 uStack8;

    uVar2 = (param_1 + 0x1e);
    iVar3 = (param_1 + 0x20);
    uStack8 = 0x0;
    do {
        iStack10 = (uStack8 * 0x2 + param_1);
        if ((iStack10 != 0x0) && (uStack8 != 0x3)) {
            iStack14 = 0x0;
            do {
                iStack12 = (iStack10 + 0x4);
                uVar4 = (u32) (iStack10 + 0x8);
                if (( uVar4 + 0xa) == 0x0) {
                    uVar5 = mem_op_1000_0510(0x1,
                                             0x0);
                    if ( uVar5 == 0x0) {
                        goto LAB_1000_00f9;
                    }
                    if (iStack12 == iStack10) {
                        iStack12 = 0x0;
                    }
                } else if (iStack14 == 0x0) {
                    iStack14 = iStack10;
                }
                iStack10 = iStack12;
            } while (iStack12 != iStack14);
        }
        uStack8 += 0x1;
    } while (uStack8 < 0x5);
    if ((param_1 + 0x32) != 0x0) {
        ((code) (param_1 + 0x32))();
    }
    LAB_1000_00f9:
    puVar1 = (u16 *) (param_1->field_0x1e);
//    return CONCAT22((iVar3 - (param_1 + 0x20)) -  (uVar2 < *puVar1),
//                    uVar2 - *puVar1);
    return 0;
}


//  pass1_1000_55b1(((u32) paVar4 >> 0x10), iVar5)
//
void pass1_1000_55b1(u32 a, i16 b) {

}

u16 pass1_1000_1ab0(u16 param_1)
{
    u16 uVar1;
    u16 uVar2;

    if (param_1 == 0x2000) {
        return 0x2000;
    }
    if (param_1 < 0xfff0) {
        if (param_1 < 0x1001) {
            return 0x1000;
        }
        uVar1 = 0x2000;
        if (param_1 < 0x2001) {
            do {
                uVar2 = uVar1;
                uVar1 = uVar2 >> 0x1;
            } while (param_1 <= uVar1);
            return uVar2 & 0xfffe;
        }
        while (uVar1 *= 0x2, uVar1 != 0x0) {
            if (param_1 <= uVar1) {
                return (uVar1 + 0x10 & - (uVar1 < 0xfff0)) - 0x10;
            }
        }
    }
    return 0xfff0;
}

i32 mem_op_1000_1558(u16 param_1,
                     u16 param_2)
{
    u16 uVar1;
    u32 uVar3;
    u16 uStack12;
    u16 uStack10;
    u16 uStack8;
    u16 uVar2;

    uStack12 = 0x0;
    uStack10 = 0x0;
    uStack8 = 0x8;
    if ((param_2 < 0x9) && ((param_2 < 0x8 || (param_1 == 0x0)))) {
        do {
            while (true) {
                uVar3 = GlobalDOSAlloc16(CONCAT22(uStack8,
                                                  uStack10));
                uVar1 =  uVar3;
                if (uVar1 == 0x0) {
                    break;
                }
//                *NULL = 0x0;
                u16_1050_0002 = uStack12;
                uStack12 = uVar1;
            }
            uVar2 = uStack8 & 0x1;
            uStack8 >>= 0x1;
            uStack10 = uStack10 >> 0x1 |  (uVar2 != 0x0) << 0xf;
        } while ((param_2 < uStack8) || ((param_2 <= uStack8 && (param_1 <= uStack10))));
    }
    return (u32) uStack12 << 0x10;
}

void pass1_1000_15ce(u16 *param_1,
                     u16 param_2)
{
    u16 *puVar1;
    u16 uVar2;
    u16 *puVar2;
    u16 uVar3;

    uVar2 = param_2 |  *param_1;
    while (uVar2 != 0x0) {
        puVar1 = param_1;
        uVar3 = param_1[0x1];
        GlobalDOSFree16(param_2);
        param_1 = puVar1;
        param_2 = uVar3;
        uVar2 = uVar3 |  *puVar1;
    }
    return;
}


bool mem_op_1000_01b0(astruct_7 *param_1)
{
    u16 *puVar1;
    i16 *piVar2;
    BOOL16 BVar3;
    u16 UVar4;
    u16 u16_var5;
    u16 unaff_CS;
    DWORD DVar6;
    DWORD DVar7;
    u32 uVar8;
    u16 u16_var9;
    u16 *pu8_var10;
    u16 uStack14;
    u16 uStack12;
    i16 iStack10;
    u16 uStack6;
    i16 iStack4;

    uStack14 = 0x0;
    if (((param_1->field_0x40) | (param_1->field_0x3e)) == 0x0) {
        u16_var5 = param_1->field_0x36;
        DVar6 = mem_op_1000_1532(param_1,
                                 0x1050);
        DVar7 = DVar6;
    } else {
        DVar6 = mem_op_1000_1532(param_1,
                                 0x1050);
        u16_var5 =  DVar6;
        if (( (DVar6 >> 0x10) != 0x0) || (0xffef < u16_var5)) {
            pass1_1000_1e61(unaff_CS,
                            0x8,
                            param_1,0);
            return false;
        }
        if (0x1fff < u16_var5) {
            u16_var5 = 0x2000;
        }
        while (true) {
            u16_var9 = u16_var5;
            DVar7 = DVar6 + 0x18;
            if (( (DVar7 >> 0x10) != 0x0) || (0xfff0 <  DVar7)) {
                DVar7 = 0xfff0;
            }
            BVar3 = mem_op_1000_14f2((param_1->field_0x16) | 0x1000,
                                     DVar7,
                                     0x1050,
                                     0,
                                     0);
            iStack4 =  (DVar6 >> 0x10);
            uStack6 =  DVar6;
            if (BVar3 != 0x0) {
                break;
            }
            u16_var5 = u16_var9 >> 0x1;
            if (u16_var5 < 0xc) {
                UVar4 = pass1_1000_1e61(unaff_CS,
                                        0x2,
                                        param_1,0);
                if (UVar4 == 0x0) {
                    return (bool) ('\x01' - ((param_1->field_0xa) == 0x0));
                }
                DVar6 = mem_op_1000_1532(param_1,
                                          0x1050);
                u16_var5 = u16_var9 & 0xfffe;
            }
        }
        uVar8 = pass1_1000_5390(uStack6 - 0x42,
                                iStack4 -  (uStack6 < 0x42),
                                0xc,
                                0x0);
        u16_var5 =  uVar8 * 0xc + param_1 + 0x42;
    }
    puVar1 = (u16 *) (param_1 + 0x1e);
    u16_var9 = *puVar1;
    *puVar1 = *puVar1 -  DVar6;
    piVar2 = (i16 *) (param_1 + 0x20);
    *piVar2 = (*piVar2 -  (DVar6 >> 0x10)) -  (u16_var9 <  DVar6);
    if (u16_var5 != 0x0) {
        pu8_var10 = 0x0;
        u16_var9 = 0xc;
        DVar7 = mem_op_1000_1532(param_1,
                                 0x1050);
        uVar8 = pass1_1000_5390( DVar7 - 0x42,
                                 (DVar7 >> 0x10) -  ( DVar7 < 0x42),
                                u16_var9,
                                pu8_var10);
        uStack14 =  uVar8 * 0xc + param_1 + 0x36;
    }
    iStack10 =  (DVar7 >> 0x10);
    uStack12 =  DVar7;
    puVar1 = (u16 *) (param_1 + 0x1e);
    u16_var9 = *puVar1;
    *puVar1 = *puVar1 + uStack12;
    piVar2 = (i16 *) (param_1 + 0x20);
    *piVar2 = *piVar2 + iStack10 +  CARRY2(u16_var9,
                                                uStack12);
    u16_var9 = (param_1->field_0xa);
    do {
        *pu8_var10 = u16_var5;
        *(pu8_var10 + 0x4) = u16_var9;
        u16_var9 = *pu8_var10;
        u16_var5 = *(pu8_var10 + 0xc);
    } while (*pu8_var10 < uStack14);
    param_1->field_0xa = *pu8_var10;
    return true;
}

void pass1_1000_07ac(u16 param_1,
                     i16 param_2,
                     i16 param_3)
{
    u16 *puVar1;
    i16 iVar2;
    u16 uVar3;

    puVar1 = (u16 *) (param_3 + 0x10);
//    (u16 *) (param_3 + 0xe) = puVar1;
//    uVar3 = param_2 + (param_3 -  puVar1);
    iVar2 =  puVar1 + (uVar3 - uVar3 % param_1);
//    (param_3 + 0x10) = iVar2;
    while (puVar1 < (u16 *) (iVar2 - param_1)) {
        *puVar1 = (u16 *) ( puVar1 + param_1);
        puVar1 = (u16 *) ( puVar1 + param_1);
    }
    *puVar1 = 0x0;
    return;
}

u32 mem_op_1000_0510(u16 param_1,
                     astruct_7 *param_2)
{
    u16 *puVar1;
    i16 *piVar2;
    u8 bVar3;
    i16 iVar4;
    u16 uVar6;
    u16 uVar7;
    u16 uVar8;
    u16 uVar9;
    u16 uVar10;
    bool bVar11;
    DWORD DVar12;
    i32 lVar13;
    u16 uVar14;
    u16 uVar5;

    iVar4 = param_2;
    uVar5 = (param_2->field_0x2);
    uVar6 = (param_2->field_0x4);
    bVar3 = *(u8 *) (param_2 + 0xc);
    // &DAT_1050_1050
    DVar12 = mem_op_1000_1532(param_2,
                              0x1050);
    uVar9 =  (DVar12 >> 0x10);
    uVar8 =  DVar12;
    //  uVar14 = &DAT_1050_1050;
    uVar14 = 0x1050;
    if (param_1 != 0x0) {
        uVar7 = (iVar4 + 0x1e);
        uVar10 = ((iVar4 + 0x20) - uVar9) -  (uVar7 < uVar8);
        puVar1 = (u16 *) (iVar4 + 0x24);
        bVar11 = uVar10 < *puVar1;
        if ((bVar11 || uVar10 == *puVar1) && ((bVar11 || (uVar7 - uVar8 < (iVar4 + 0x22))))) {
            bVar11 = false;
            uVar9 = uVar10;
            goto LAB_1000_0595;
        }
    }
    pass1_1000_0368(uVar6,
                    bVar3 & 0x7,
                    0x0);
    puVar1 = 0x001e;
    uVar7 = *puVar1;
    *puVar1 = *puVar1 - uVar8;
    // (i16 *)s_New_failed_in_Op__Op_1050_0020;
    piVar2 = 0x0020;
    *piVar2 = (*piVar2 - uVar9) -  (uVar7 < uVar8);
    bVar11 = true;
    LAB_1000_0595:
    if (bVar11) {
        (param_2->field_0xc) = 0x0;
        lVar13 = mem_op_1000_13ce(param_2,
                                  uVar14);
        return CONCAT22( ((u32) lVar13 >> 0x10),
                        0x1);
    }
    return (u32) uVar9 << 0x10;
}

u16 pass1_1000_010c(i16 param_1,
                    u16 param_2,
                    u16 param_3,
                    u16 param_4,
                    u16 param_5)
{
    u16 uVar1;
    u16 UVar2;
    u16 uVar2;
    u16 unaff_CS;
    bool bVar3;
    u16 UVar4;
    u16 uStack8;
    u16 uStack6;
    u16 uStack4;

    uStack6 = 0x0;
    uStack8 = 0x0;
    if ((param_4 + 0x14) != -0x4153) {
        param_5 = 0x0;
        param_4 = 0x0;
        UVar4 = 0xa;
        code_r0x10000128:
        pass1_1000_1e61(unaff_CS,
                        UVar4,
                        param_4,0);
        return 0xffff;
    }
    DAT_1050_5f30 = 0x1;
    if (param_1 == 0x1) {
        uStack4 = 0x1;
        if ((param_4 + 0x18) == 0x0) {
            UVar4 = 0x4;
            goto code_r0x10000128;
        }
    } else if (param_1 == 0x2) {
        uStack4 = 0x2;
    } else {
        if (param_1 != 0x4) {
            DAT_1050_5f30 = 0x1;
            return 0xffff;
        }
        uStack4 = 0x0;
    }
    // s_version__d__d_1050_0012 + 0x8
    while ((uStack6 <= param_3 && (((uStack6 < param_3 || (uStack8 < param_2)) && (uVar1 = uStack4, UVar2 =
                                                                                                        mem_op_1000_03c6(( 0x001a),
                                                                                                                         0x0,
                                                                                                                         uStack4,
                                                                                                                         0x0,
                                                                                                                         0x0,
                                                                                                                         '\0'),
        (UVar2 | uVar1) != 0x0))))) {
        uVar1 = ( 0x00a1a);
        bVar3 = CARRY2(uStack8,
                       uVar1);
        uStack8 += uVar1;
        uStack6 += bVar3;
    }
    return uStack6;
}

BOOL16 mem_op_1000_14f2(u16 param_1,
                        u32 param_2,
                        astruct_7 *param_4,
                        u16 param_5,
                        u16 param_3)
{
//    u16 AX_REG;
//    u16 DX_REG;

    if (((param_1 & 0x1000) != 0x0) || ((param_3 == 0x0 && (param_2 < 0xfff1)))) {
        mem_op_1000_1408(param_1 & 0xfdff | 0x800,
                         CONCAT22(param_3, param_2),
                         param_4,
                         param_5);
        if ((DX_REG | AX_REG) != 0x0) {
            return 0x1;
        }
    }
    return 0x0;
}

u32 pass1_1000_5390(u16 param_1,
                    u16 param_2,
                    u16 param_3,
                    u16 param_4)
{
    u32 uVar1;
    i32 lVar2;
    u16 uVar3;
    i16 iVar4;
    u16 uVar5;
    u16 uVar6;
    u16 uVar7;
    u16 uVar8;
    u16 uVar9;

    uVar3 = param_1;
    uVar8 = param_4;
    uVar6 = param_2;
    uVar9 = param_3;
    if (param_4 == 0x0) {
        uVar3 = param_2 / param_3;
        iVar4 =  (((u32) param_2 % (u32) param_3 << 0x10 | (u32) param_1) / (u32) param_3);
    } else {
        do {
            uVar5 = uVar8 >> 0x1;
            uVar9 = uVar9 >> 0x1 |  ((uVar8 & 0x1) != 0x0) << 0xf;
            uVar7 = uVar6 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar6 & 0x1) != 0x0) << 0xf;
            uVar8 = uVar5;
            uVar6 = uVar7;
        } while (uVar5 != 0x0);
        uVar1 = CONCAT22(uVar7,
                         uVar3) / (u32) uVar9;
        iVar4 =  uVar1;
        lVar2 = (u32) param_3 * (uVar1 & 0xffff);
        uVar3 =  ((u32) lVar2 >> 0x10);
        uVar8 = uVar3 + iVar4 * param_4;
        if (((CARRY2(uVar3,
                     iVar4 * param_4)) || (param_2 < uVar8)) || ((param_2 <= uVar8 && (param_1 <  lVar2)))) {
            iVar4 += -0x1;
        }
        uVar3 = 0x0;
    }
    return CONCAT22(uVar3,
                    iVar4);
}

u32 pass1_1000_53f0(u16 param_1,
                    u16 param_2,
                    u16 param_3,
                    u16 param_4)
{
    u32 uVar1;
    i32 lVar2;
    u16 uVar3;
    u16 uVar4;
    u16 uVar5;
    i16 iVar6;
    i16 iVar7;
    u16 uVar8;
    u16 uVar9;
    u16 uVar10;
    bool bVar11;

    uVar3 = param_1;
    uVar4 = param_4;
    uVar9 = param_2;
    uVar10 = param_3;
    if (param_4 == 0x0) {
        iVar6 =  (((u32) param_2 % (u32) param_3 << 0x10 | (u32) param_1) % (u32) param_3);
        iVar7 = 0x0;
    } else {
        do {
            uVar5 = uVar4 >> 0x1;
            uVar10 = uVar10 >> 0x1 |  ((uVar4 & 0x1) != 0x0) << 0xf;
            uVar8 = uVar9 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar9 & 0x1) != 0x0) << 0xf;
            uVar4 = uVar5;
            uVar9 = uVar8;
        } while (uVar5 != 0x0);
        uVar1 = CONCAT22(uVar8,
                         uVar3) / (u32) uVar10;
        uVar3 =  uVar1 * param_4;
        lVar2 = (uVar1 & 0xffff) * (u32) param_3;
        uVar9 =  ((u32) lVar2 >> 0x10);
        uVar4 =  lVar2;
        uVar10 = uVar9 + uVar3;
        if (((CARRY2(uVar9,
                     uVar3)) || (param_2 < uVar10)) || ((param_2 <= uVar10 && (param_1 < uVar4)))) {
            bVar11 = uVar4 < param_3;
            uVar4 -= param_3;
            uVar10 = (uVar10 - param_4) -  bVar11;
        }
        iVar6 = -(uVar4 - param_1);
        iVar7 = - (uVar4 - param_1 != 0x0) - ((uVar10 - param_2) -  (uVar4 < param_1));
    }
    return CONCAT22(iVar7,
                    iVar6);
}
