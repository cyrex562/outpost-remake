//
// Created by cyrex on 2022-05-22.
//

#include "types.h"
#include "structs.h"
#include "globals.h"
#include "sys_api.h"
#include "block_1000.h"
#include "utils.h"
#include "structs_2.h"
#include "func_ptrs.h"
#include "entry.h"
#include <stdbool.h>
#include <stdio.h>

void pass1_1000_0000(u16 *param_1,
                     u16 *param_2,
                     u16 param_3)
{
    u16 *puVar1;
    u16 *puVar2;
    u16 uVar3;

    for (uVar3 = param_3 >> 0x1; uVar3 != 0x0; uVar3 -= 0x1) {
        puVar2 = param_2;
        param_2 = param_2 + 0x1;
        puVar1 = param_1;
        param_1 = param_1 + 0x1;
        *puVar2 = *puVar1;
    }
}

u32 mem_1000_0016(u32 param_1)
{
    u16 unaff_CS;
    u32 uVar1;

    if (( param_1 + 0x14) != -0x4153) {
        pass1_1000_1e61(unaff_CS,
                        0xa,
                        0x0);
        return 0xffffffff;
    }
    uVar1 = mem_op_1000_0052(0x0);
    return uVar1;
}














u32 mem_1000_0668(void)
{
    u32 uVar1;

    uVar1 = mem_op_1000_0510(0x0,
                             0x0);
    return uVar1;
}

u16 mem_1000_0670(u16 param_1,
                  astruct_7 *param_2,
                  i16 param_3,
                  u16 param_4,
                  i16 *param_5)
{
    u16 *puVar1;
    i16 *piVar2;
    u16 UVar3;
    u16 UVar4;
    i16 iVar5;
    i16 iVar6;
    u32 uVar7;
    u16 uVar8;
    u32 *puVar9;
    u16 UVar10;
    BOOL16 BVar11;
    u16 uVar12;
    u16 uVar13;
    u16 unaff_CS;
    DWORD DVar14;
    DWORD DVar15;
    u16 uStack16;

    UVar3 = param_2;
    UVar4 = ( param_2 + 0x2);
    // 0x1050
    DVar14 = mem_op_1000_1532(param_2,
                              0x1050);
    iVar6 = param_3 +  (0xffeb < param_1);
    uVar7 = *param_2;
    uVar8 = - ((param_4 & 0x1) != 0x0) & 0x100 | - ((param_4 & 0x4) != 0x0) & 0x400 | ( uVar7 + 0x16);
    if (param_5 == NULL) {
        //  0x1050
        BVar11 = mem_op_1000_14f2(uVar8 | 0x2000,
                                  param_1 + 0x14,
                                  param_2,
                                  0x1050);
        if (BVar11 == 0x0) {
            return 0x0;
        }
        // 0x1050
        uStack16 = 0x1050;
    } else {
        iVar5 = (param_2 + 0x1);
        uVar12 = ( param_2 + 0x6);
        uVar13 = uVar12;
        do {
            uStack16 = uVar13;
            puVar9 = (u32 *) (uVar8 | 0x2000);
            // 0x1050
            mem_op_1000_1408(puVar9,
                             CONCAT22(iVar6,
                                      param_1 + 0x14),
                             param_2,
                             0x1050);
            uVar13 = uStack16 | puVar9;
            if (uVar13 != 0x0) {
                break;
            }
            UVar10 = pass1_1000_1e61(unaff_CS,
                                     0x2,
                                     UVar3);
        } while (UVar10 != 0x0);
        if ((uStack16 |  puVar9) == 0x0) {
            ( param_5 + 0x2) = 0x0;
            *param_5 = 0x0;
            return 0x0;
        }
        (u32*) (iVar5 + 0x8) = puVar9;
        (iVar5 + 0xa) = uStack16;
        *param_5 =  (puVar9 + 0x5);
        ( param_5 + 0x2) = uStack16;
        param_2 = puVar9;
    }
    DVar15 = mem_op_1000_1532( param_2,
                              uStack16);
    uVar12 =  (DVar15 - DVar14);
    puVar1 = (u16 *) (UVar3 + 0x1e);
    uVar8 = *puVar1;
    *puVar1 = *puVar1 + uVar12;
    piVar2 = (i16 *) (UVar3 + 0x20);
    *piVar2 = *piVar2 +  (DVar15 - DVar14 >> 0x10) +  CARRY2(uVar8,
                                                                       uVar12);
    return 0x1;
}





struct astruct_99 *pass1_1000_07fc(u32 param_1)
{
    u16 unaff_CS;
    struct astruct_99 *paVar1;

    if (( param_1 + 0x14) != -0x4153) {
        pass1_1000_1e61(unaff_CS,
                        0xa,
                        0x0);
        return NULL;
    }
    paVar1 = (astruct_99 *) mem_op_1000_0838(0x0);
    return paVar1;
}


u16 pass1_1000_093a(astruct_611 *param_1)
{
    i16 *piVar1;
    u16 unaff_CS;

    if (&PTR_LOOP_1050_000c != -0x352f) {
        pass1_1000_1e61(unaff_CS,
                        0xe,
                        0x0);
        return 0x0;
    }
    param_1 = (astruct_611 *) PTR_LOOP_1050_000e;
    if (param_1 == 0x0) {
        (u32) &u32_1050_0004 = 0x1;
    }
    PTR_LOOP_1050_000e = (astruct_611 *) param_1;
    piVar1 = (i16 *) &PTR_LOOP_1050_000a;
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 == 0x0) {
        mem_op_1000_0510(0x1,
                         0x0);
    }
    return 0x1;
}

u8 *pass1_1000_09a0(u16 *param_1)
{
    u8 *puVar1;
    u32 uVar2;

    *param_1 = PTR_LOOP_1050_000e;
    if (PTR_LOOP_1050_000e == NULL) {
        u32_1050_0004 = 0x1;
    }
    PTR_LOOP_1050_000a = PTR_LOOP_1050_000a + -0x1;
    puVar1 = PTR_LOOP_1050_000e;
    PTR_LOOP_1050_000e = (u8 *) param_1;
    if (PTR_LOOP_1050_000a == NULL) {
        uVar2 = mem_op_1000_0510(0x1,
                                 0x0);
        puVar1 = (u8 *) uVar2;
    }
    return puVar1;
}









BOOL16 call_fn_ptr_1000_0dc6(char *param_1)
{
//    u16 unaff_CS;

    if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
        pass1_1000_1e61(unaff_CS,
                        0xe,
                        0x0);
        return 0x0;
    }
    // 0x1050
    (u16_1050_0008)(0x1050);
    return 0x1;
}

u16 pass1_1000_0e08(i16 param_1)
{
    u16 *puVar1;
    u8 *pbVar2;
    u16 uVar3;
    u16 *puVar4;
    u16 *puVar5;
    bool bVar6;
    u32 uVar7;

    puVar5 = (u16 *) (param_1 + -0x2);
    bVar6 = (*(u8 *) puVar5 & 0x2) != 0x0;
    if (bVar6) {
        puVar1 = puVar5;
        *(u8 *) puVar1 = *(u8 *) puVar1 & 0xfe;
    } else {
        puVar4 = (u16 *) ( puVar5 - (param_1 + -0x4));
        puVar1 = puVar4;
        *puVar1 = *puVar1 + (*puVar5 & 0xfffc);
        puVar5 = puVar4;
    }
    puVar4 = (u16 *) ((*puVar5 & 0xfffc) +  puVar5);
    if ((*(u8 *) puVar4 & 0x1) == 0x0) {
        puVar1 = puVar5;
        *puVar1 = *puVar1 + (*puVar4 & 0xfffc);
        if (puVar4 == (u16 *) PTR_LOOP_1050_000e) {
            PTR_LOOP_1050_000e = (u8 *) puVar5;
        }
        (puVar4[0x2] + 0x2) = puVar4[0x1];
        (puVar4[0x1] + 0x4) = puVar4[0x2];
        puVar4 = (u16 *) ((*puVar5 & 0xfffc) +  puVar5);
    }
    puVar4[-0x1] = *puVar5 & 0xfffc;
    uVar3 = u32_1050_0004;
    puVar1 = puVar4 + -0x1;
    if (uVar3 <= *puVar1 && *puVar1 != uVar3) {
        uVar3 = *puVar5 & 0xfffc;
        u32_1050_0004 = uVar3;
    }
    puVar1 = puVar4;
    *(u8 *) puVar1 = *(u8 *) puVar1 & 0xfd;
    if (bVar6) {
        if ((StructD *) PTR_LOOP_1050_0010->address_offset_field_0x2 != PTR_LOOP_1050_0010) {
            pbVar2 = (u8 *) ( u32_1050_0004 + 0x3);
            *pbVar2 = *pbVar2 & 0x7f;
        }
        puVar5[0x2] =  PTR_LOOP_1050_0010;
        uVar3 = PTR_LOOP_1050_0010->address_offset_field_0x2;
        puVar5[0x1] = uVar3;
        (u16 *) (PTR_LOOP_1050_0010->address_offset_field_0x2 + 0x4) = puVar5;
        PTR_LOOP_1050_0010->address_offset_field_0x2 =  puVar5;
    }
    PTR_LOOP_1050_000a = PTR_LOOP_1050_000a + -0x1;
    if (PTR_LOOP_1050_000a == NULL) {
        uVar7 = mem_op_1000_0510(0x1,
                                 0x0);
        uVar3 =  uVar7;
    }
    return uVar3;
}

i32 pass1_1000_0ed4(u16 param_1,
                    u16 param_2,
                    u16 param_3,
                    astruct_172 *param_4,
                    astruct_172 *param_5)
{
    struct astruct_172 *paVar1;
    u16 *puVar2;
    u16 uVar3;
    struct astruct_172 **ppaVar4;
    u16 uVar5;
    u16 uVar6;
    struct astruct_172 *paVar7;
    u16 *puVar8;
    u16 unaff_CS;
    i32 lVar9;
    u16 UVar10;
    u16 UVar11;
    u16 UVar12;

    if ((&PTR_LOOP_1050_000c & 0xfff8) == 0xcad0) {
        UVar11 = *NULL;
        UVar12 = &u16_1050_0002;
        if ((param_1 & 0x8) == 0x0) {
            ppaVar4 = &param_4;
            //  0x1050;
            uVar6 = 0x1050;
        } else {
            ppaVar4 = NULL;
            uVar6 = 0x0;
        }
        uVar6 = pass1_1000_0fb8(param_2,
                                 param_4,
                                param_3,
                                param_1,
                                (u16 *) ppaVar4,
                                uVar6);
        if (uVar6 == 0x0) {
            return CONCAT22(param_5,
                            param_4);
        }
        if ((param_1 & 0x8) == 0x0) {
            lVar9 = mem_op_1000_0a48((u8) param_1,
                                     param_2,
                                     param_3,
                                     CONCAT22(UVar12,
                                              UVar11));
            uVar3 =  ((u32) lVar9 >> 0x10);
            puVar8 = (u16 *) lVar9;
            if (lVar9 != 0x0) {
                paVar7 = param_4;
                for (uVar5 = uVar6 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
                    puVar2 = puVar8;
                    puVar8 = puVar8 + 0x1;
                    paVar1 = paVar7;
                    paVar7 = (astruct_172 *) &paVar7->field1_0x2;
                    *puVar2 = paVar1->field0_0x0;
                }
                for (uVar6 =  ((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
                    puVar2 = puVar8;
                    puVar8 = (u16 *) ( puVar8 + 0x1);
                    paVar1 = paVar7;
                    paVar7 = (astruct_172 *) ( &paVar7->field0_0x0 + 0x1);
                    *(u8 *) puVar2 = *(u8 *) &paVar1->field0_0x0;
                }
                call_fn_ptr_1000_0dc6((char *) CONCAT22(param_5,
                                                        param_4));
            }
            return lVar9;
        }
        if ((param_3 | param_2) == 0x0) {
            return 0x0;
        }
        UVar10 = 0x5;
    } else {
        UVar11 = 0x0;
        UVar12 = 0x0;
        UVar10 = 0xe;
    }
    pass1_1000_1e61(unaff_CS,
                    UVar10,
                    UVar11);
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x10001126)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1000_0fb8(u16 param_1,
                    i16 param_2,
                    u16 param_3,
                    u16 param_4,
                    u16 *param_5,
                    u16 param_6)
{
    u16 *puVar1;
    u8 bVar2;
    u16 uVar3;
    BOOL16 BVar4;
    i16 iVar5;
    u16 uVar6;
    u16 *puVar7;
    u16 *puVar8;
    u16 unaff_CS;
    u32 uVar9;
    u16 uStack4;

    if ((param_3 | param_1) == 0x0) {
        pass1_1000_1e61(unaff_CS,
                        0x4,
                         PTR_LOOP_1050_0000);
        if ((param_6 |  param_5) != 0x0) {
            param_5[0x1] = 0x0;
            *param_5 = 0x0;
            return 0x0;
        }
        return 0x1;
    }
    bVar2 = (u8) PTR_LOOP_1050_000c & 0x7;
    if (((u8) PTR_LOOP_1050_000c & 0x7) != 0x0) {
        if (bVar2 == 0x1) {
            uVar3 = (PTR_LOOP_1050_0000 + 0x18);
            if (param_3 != 0x0) {
                return uVar3;
            }
            if (param_1 <= uVar3) {
                return 0x0;
            }
            return uVar3;
        }
        if (bVar2 != 0x2) {
            if (bVar2 != 0x3) {
                if ((param_6 |  param_5) != 0x0) {
                    param_5[0x1] = 0x0;
                    *param_5 = 0x0;
                    return 0x0;
                }
                return 0x1;
            }
            if ((((param_6 |  param_5) != 0x0) && (param_3 == 0x0)) && (param_1 <= (PTR_LOOP_1050_0000 + 0x1c))) {
                uVar9 = pass1_1000_1284(CONCAT22(0x1050,
                                                 param_2));
                if (CONCAT22(param_3,
                             param_1) < uVar9) {
                    return param_1;
                }
                return  uVar9;
            }
            iVar5 = mem_1000_0670(param_1,
                                  NULL,
                                  param_3,
                                  param_4,
                                  (i16 *) CONCAT22(param_6,
                                                   param_5));
            if (iVar5 != 0x0) {
                return 0x0;
            }
            if ((param_6 |  param_5) != 0x0) {
                return 0x0;
            }
            return 0x1;
        }
    }
    puVar8 = (u16 *) (param_2 + -0x2);
    uVar3 = *puVar8 & 0x7ffc;
    uStack4 = uVar3 - 0x2;
    if ((*(u8 *) (param_2 + -0x1) & 0x80) != 0x0) {
        uStack4 = uVar3 - 0x6;
    }
    if ((((param_3 == 0x0) && (param_1 <= uStack4)) || ((param_3 == 0x0 && (param_1 <= (PTR_LOOP_1050_0000 + 0x1c)))))
        && (BVar4 = pass1_1000_115c(param_1,
                                    puVar8), BVar4 != 0x0)) {
        if ((param_4 & 0x1) != 0x0) {
            uVar3 = (*puVar8 & 0x7ffc) - 0x2;
            if (uStack4 < param_1) {
                puVar7 = (u16 *) (uStack4 + param_2);
                iVar5 = -uStack4;
            } else {
                if (uVar3 <= param_1) {
                    return 0x0;
                }
                puVar7 = (u16 *) (param_1 + param_2);
                iVar5 = -param_1;
            }
            uVar3 += iVar5;
            for (uVar6 = uVar3 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
                puVar1 = puVar7;
                puVar7 = puVar7 + 0x1;
                *puVar1 = 0x0;
            }
            if ((uVar3 & 0x1) != 0x0) {
                *(u8 *) puVar7 = 0x0;
            }
        }
        return 0x0;
    }
    return uStack4;
}

BOOL16 pass1_1000_115c(i16 param_1,
                       u16 *param_2)
{
    u8 *pbVar1;
    u16 *puVar2;
    u16 uVar3;
    u16 uVar4;
    u16 *puVar5;
    u16 UVar6;
    u16 uStack4;

    uVar3 = *param_2 & 0x7ffc;
    uVar4 = param_1 + 0x5U & 0xfffc;
    uVar4 = (uVar4 - 0x8 & ~- (uVar4 < 0x8)) + 0x8;
    if (uVar3 < uVar4) {
        puVar5 = (u16 *) (uVar3 +  param_2);
        if (((*(u8 *) puVar5 & 0x1) != 0x0) || ((*puVar5 & 0xfffc) + uVar3 < uVar4)) {
            return 0x0;
        }
        if (puVar5 == (u16 *) PTR_LOOP_1050_000e) {
            PTR_LOOP_1050_000e = (u8 *) puVar5[0x1];
        }
        (puVar5[0x2] + 0x2) = puVar5[0x1];
        (puVar5[0x1] + 0x4) = puVar5[0x2];
        uStack4 = ((*puVar5 & 0xfffc) + uVar3) - uVar4;
        if (uStack4 < s_version__d__d_1050_0012._0_2_) {
            puVar2 = param_2;
            *puVar2 = *puVar2 + (*puVar5 & 0xfffc);
            pbVar1 = (u8 *) ( puVar5 + (*puVar5 & 0xfffc));
            *pbVar1 = *pbVar1 | 0x2;
            return 0x1;
        }
    } else {
        uStack4 = uVar3 - uVar4;
        if (uStack4 < s_version__d__d_1050_0012._0_2_) {
            return 0x1;
        }
        puVar5 = (u16 *) (uVar3 +  param_2);
        if ((*(u8 *) puVar5 & 0x1) == 0x0) {
            uStack4 += *puVar5 & 0xfffc;
            if (puVar5 == (u16 *) PTR_LOOP_1050_000e) {
                PTR_LOOP_1050_000e = (u8 *) puVar5[0x1];
            }
            (puVar5[0x2] + 0x2) = puVar5[0x1];
            (puVar5[0x1] + 0x4) = puVar5[0x2];
        }
        if (u32_1050_0004 < uStack4) {
            u32_1050_0004 = uStack4;
        }
    }
    *param_2 = *param_2 & 0x8003 | uVar4;
    (uVar4 +  param_2) = uStack4 | 0x2;
    UVar6 = uVar4 +  param_2;
    *(StructD * *)(UVar6 + 0x4) = PTR_LOOP_1050_0010;
    (UVar6 + 0x2) = PTR_LOOP_1050_0010->address_offset_field_0x2;
    (PTR_LOOP_1050_0010->address_offset_field_0x2 + 0x4) = UVar6;
    PTR_LOOP_1050_0010->address_offset_field_0x2 = UVar6;
    ((u8 *) (UVar6 + uStack4) + -0x2) = uStack4;
    pbVar1 = (u8 *) (UVar6 + uStack4);
    *pbVar1 = *pbVar1 & 0xfd;
    return 0x1;
}

u32 pass1_1000_1284(u32 param_1)
{
    u8 bVar1;
    u16 uVar2;
    u32 uVar3;
    u8 bVar4;
    u16 uVar5;
    u16 unaff_CS;
    bool bVar6;
    DWORD DVar7;
    u16 uStack6;
    i16 iStack4;

    if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
        pass1_1000_1e61(unaff_CS,
                        0xe,
                        0x0);
        return 0xffffffff;
    }
    bVar1 = *(u8 *) &PTR_LOOP_1050_000c;
    bVar4 = bVar1 & 0x7;
    if ((bVar1 & 0x7) != 0x0) {
        if (bVar4 == 0x1) {
            uVar3 = *NULL;
            return (u32) ( uVar3 + 0x18);
        }
        if (bVar4 != 0x2) {
            if (bVar4 != 0x3) {
                return 0xffffffff;
            }
            DVar7 = mem_op_1000_1532(0x0,
                                     param_1);
            return CONCAT22( (DVar7 >> 0x10) -  ( DVar7 < 0x14),
                             DVar7 - 0x14);
        }
    }
    uVar2 = ( param_1 + -0x2);
    uVar5 = uVar2 & 0x7ffc;
    uStack6 = uVar5 - 0x2;
    iStack4 = 0x0;
    if ((uVar2 & 0x8000) != 0x0) {
        bVar6 = uStack6 < 0x4;
        uStack6 = uVar5 - 0x6;
        iStack4 = - bVar6;
    }
    return CONCAT22(iStack4,
                    uStack6);
}











// WARNING: Unable to use type for symbol uVar3






u16 pass1_1000_16aa(u16 param_1,
                    u16 *param_2,
                    u16 param_3,
                    u16 param_4)
{
    u16 uVar1;
    i32 lVar2;

    if ((param_3 |  param_2) == 0x0) {
        uVar1 = mem_1000_167a(param_1,
                              param_4);
        return uVar1;
    }
    if (param_4 == 0x0) {
        pass1_1000_16ee( param_2,
                        param_3);
        return 0x0;
    }
    lVar2 = pass1_1000_0ed4(0x0,
                            param_4,
                            0x0,
                            (astruct_172 *) param_2,
                            (astruct_172 *) param_3);
    return  lVar2;
}

void pass1_1000_16ee(u16 param_1,
                     u16 param_2)
{
    if ((param_2 | param_1) != 0x0) {
        call_fn_ptr_1000_0dc6((char *) CONCAT22(param_2,
                                                param_1));
    }
    return;
}



void fn_ptr_1000_17ce(char *param_1)
{
    if (param_1 != NULL) {
        call_fn_ptr_1000_0dc6(param_1);
    }
    return;
}

u8 *pass1_1000_17e8(u8 *param_1,
                    u8 *param_2)
{
    u8 *puVar1;

    puVar1 = PTR_LOOP_1050_5f34;
    PTR_LOOP_1050_5f34 = param_1;
    PTR_LOOP_1050_5f36 = param_2;
    return puVar1;
}

u16 pass1_1000_180c(u16 param_1,
                    u16 param_2)
{
    u8 *puVar1;
    u16 in_register_0000000a;
    StructD *pSVar2;
    i32 lVar3;

    pSVar2 = (StructD *) CONCAT22(in_register_0000000a,
                                  param_1);
    if (( PTR_LOOP_1050_5f2e |  PTR_LOOP_1050_5f2c) == 0x0) {
        puVar1 = mem_op_1000_160a(pSVar2);
        if (( pSVar2 |  puVar1) == 0x0) {
            return 0x0;
        }
    }
    lVar3 = mem_op_1000_0a48(0x0,
                             param_2,
                             0x0,
                             CONCAT22(PTR_LOOP_1050_5f2e,
                                      PTR_LOOP_1050_5f2c));
    return  lVar3;
}

u16 pass1_1000_183c(u16 param_1,
                    u16 param_2)
{
    u32 in_EDX;
    StructD *pSVar1;
    i32 lVar2;

    pSVar1 = (StructD * )(in_EDX & 0xffff0000);
    if ( ((u32) param_2 * (u32) param_1 >> 0x10) != 0x0) {
        return 0x0;
    }
    if (( PTR_LOOP_1050_5f2e |  PTR_LOOP_1050_5f2c) == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar1);
        PTR_LOOP_1050_5f2e = (u8 *) pSVar1;
        if (( PTR_LOOP_1050_5f2e |  PTR_LOOP_1050_5f2c) == 0x0) {
            return 0x0;
        }
    }
    lVar2 = mem_op_1000_0a48(0x1,
                              ((u32) param_2 * (u32) param_1),
                             0x0,
                             CONCAT22(PTR_LOOP_1050_5f2e,
                                      PTR_LOOP_1050_5f2c));
    return  lVar2;
}

u16 pass1_1000_188e(u16 param_1,
                    u16 *param_2,
                    u16 param_3,
                    u16 param_4)
{
    u16 uVar1;
    i32 lVar2;

    if ((param_3 |  param_2) == 0x0) {
        uVar1 = pass1_1000_180c(param_1,
                                param_4);
        return uVar1;
    }
    if (param_4 == 0x0) {
        pass1_1000_18d2( param_2,
                        param_3);
        return 0x0;
    }
    lVar2 = pass1_1000_0ed4(0x0,
                            param_4,
                            0x0,
                            (astruct_172 *) param_2,
                            (astruct_172 *) param_3);
    return  lVar2;
}

void pass1_1000_18d2(u16 param_1,
                     u16 param_2)
{
    if ((param_2 | param_1) != 0x0) {
        call_fn_ptr_1000_0dc6((char *) CONCAT22(param_2,
                                                param_1));
    }
    return;
}





// WARNING: Globals starting with '_' overlap smaller symbols at the same address















// WARNING: Globals starting with '_' overlap smaller symbols at the same address






void pass1_1000_1f68()
{
    PTR_LOOP_1050_5f26 -= 1;
    if (PTR_LOOP_1050_5f26 < 0x0) {
        PTR_LOOP_1050_5f26 = 0;
    }
}



// WARNING: Removing unreachable block (ram,0x10001f92)

BOOL16 pass1_1000_1f7e(u16 *param_1)
{
    char cVar1;
    BOOL16 BVar2;
    u16 uVar3;
    i16 iVar4;
    char *pcVar5;

    uVar3 = *param_1;
    if (uVar3 == 0xf) {
        LAB_1000_1fb6:
        iVar4 = 0x1;
    } else {
        if (uVar3 < 0x10) {
            cVar1 = (char) uVar3;
            if (cVar1 == '\x02') {
                goto LAB_1000_1fb6;
            }
            if (('\0' < (char) (cVar1 + -0x2)) && ((char) (cVar1 + -0x3) < '\f')) {
                iVar4 = 0x0;
                goto LAB_1000_1fbe;
            }
        }
        iVar4 = 0x0;
        uVar3 = 0x1;
    }
    LAB_1000_1fbe:
    pcVar5 = pass1_1000_1fd2(uVar3);
    BVar2 = msg_box_op_1000_214c(0x0,
                                 iVar4,
                                  pcVar5,
                                  ((u32) pcVar5 >> 0x10));
    return BVar2;
}

char *pass1_1000_1fd2(i16 param_1)
{
    if (param_1 == 0x2) {
        return "Out of memory.  Please free some memory, then choose retry.";
    }
    return (char *) CONCAT22(0x1000,
                             param_1 * 0x17 + 0x1c7a);
}



// WARNING: Removing unreachable block (ram,0x10002018)


void pass1_1000_201c(i16 param_1,
                     i16 param_2)
{
    u16 uVar1;
    u32 uVar2;
    u16 uVar3;
    BOOL16 BVar4;
    i16 iVar5;
    u16 uVar6;

    if (param_1 == 0x0) {
        (param_2 + 0x6) = 0x0;
        (param_2 + 0x4) = 0x0;
    }
    uVar3 = (param_2 + 0x6) | (param_2 + 0x4);
    while (uVar3 != 0x0) {
        BVar4 = pass1_1000_206c((param_2 + 0x4),
                                (param_2 + 0x6));
        if (BVar4 == 0x0) {
            uVar2 = (u32) (param_2 + 0x4);
            uVar6 =  ((u32) uVar2 >> 0x10);
            iVar5 =  uVar2;
            uVar1 = (iVar5 + 0x2c);
            (param_2 + 0x4) = (iVar5 + 0x2a);
            (param_2 + 0x6) = uVar1;
        } else {
            mem_op_1000_1b9a(0x1,
                             *(u16 *) (param_2 + 0x4),
                             (param_2 + 0x6));
        }
        uVar3 = (param_2 + 0x6) | (param_2 + 0x4);
    }
    return;
}

BOOL16 pass1_1000_206c(u16 param_1,
                       u16 param_2)
{
    u16 uVar1;

    uVar1 = pass1_1000_21d2(0x2,
                            0x42,
                            param_1,
                            param_2,
                            0x1);
    if ((uVar1 != 0x0) && ((param_1 + 0x14) == -0x4153)) {
        return 0x1;
    }
    return 0x0;
}




void empty_fn_1000_214a()
{
    return;
}






// WARNING: Removing unreachable block (ram,0x100021de)

u16 pass1_1000_21d2(u8 param_1,
                    i32 param_2,
                    u16 param_3,
                    u16 param_4,
                    u8 param_5)
{
    u32 uVar1;
    BOOL16 BVar2;

    BVar2 = mem_op_1000_1dfa(0x0,
                             param_1,
                             param_3,
                             param_4);
    if (BVar2 == 0x0) {
        if ((param_1 & 0x4) == 0x0) {
            uVar1 = SegmentLimit((u32) param_4);
            if ((bool) ((u8) ((u16) uVar1 >> 0x10) & 0x1)) {
                if (param_2 == 0x0) {
                    return 0x1;
                }
                if ((!CARRY4((u32) param_3,
                             param_2 - 0x1U)) && ((u32) param_3 + (param_2 - 0x1U) <= (u32)  uVar1)) {
                    return 0x1;
                }
            }
        } else {
            BVar2 = pass1_1000_22c0( param_2,
                                    _param_1,
                                    param_2,
                                    param_3,
                                    param_4);
            if (BVar2 != 0x0) {
                return 0x1;
            }
        }
    }
    return 0x0;
}

u32 pass1_1000_2242(u16 param_1,
                    u8 *param_2,
                    u16 param_3,
                    u16 param_4,
                    u16 param_5,
                    i16 param_6)
{
    u16 uVar1;
    u16 uVar2;
    bool bVar3;

    uVar1 = param_4 | param_3;
    while (true) {
        if (uVar1 == 0x0) {
            return 0x0;
        }
        uVar1 = param_3;
        if (param_4 != 0x0) {
            uVar1 = 0xffff;
        }
        if (CARRY2(param_5,
                   uVar1) != false) {
            uVar1 = -param_5;
        }
        bVar3 = param_3 < uVar1;
        param_3 -= uVar1;
        param_4 -= bVar3;
        uVar2 = ((code) param_2)(uVar1,
                                 param_1,
                                 param_5,
                                 param_6);
        if (uVar2 != 0x0) {
            break;
        }
        bVar3 = CARRY2(param_5,
                       uVar1);
        param_5 += uVar1;
        param_6 +=  bVar3 * 0x100;
        uVar1 = param_4 | param_3;
    }
    return CONCAT22(param_4 + CARRY2(uVar2,
                                     param_3),
                    uVar2 + param_3);
}

BOOL16 pass1_1000_22c0(u16 param_1,
                       u16 param_2,
                       u16 param_3,
                       u16 param_4,
                       u16 param_5)
{
    u32 u32_var1;

    u32_var1 = pass1_1000_2242(param_2,
                               (u8 *) 0x1dfa,
                               param_1,
                               param_3,
                               param_4,
                               param_5);
    if (u32_var1 == 0x0) {
        return 0x1;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x1000234c)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address



/*
Unable to decompile 'dos3_call_1000_23ea'
Cause:
Low-level Error: Symbol $$undef0000001e extends beyond the end of the address space
*/


// WARNING: Removing unreachable block (ram,0x10002557)





// WARNING: Removing unreachable block (ram,0x10002513)
// WARNING: Removing unreachable block (ram,0x10002557)





// WARNING: Removing unreachable block (ram,0x10002589)





void pass1_1000_25a8(void)
{
    pass1_1000_2913(0xfc);
    pass1_1000_2913(0xff);
    return;
}

/*
Unable to decompile 'exit_1000_25cc'
Cause:
Low-level Error: Symbol $$undef00000007 extends beyond the end of the address space
*/


i16 *pass1_1000_25d2(i16 param_1,
                     i16 param_2,
                     code2 fn_ptr_param_3,
                     i16 param_4)
{
    i16 *piVar1;
    char *pcVar2;
    u8 *puVar3;
    StructD *pstruct_d_var4;
    i16 *piVar5;
    char *pcVar6;
    i16 iVar7;
    i16 *piVar8;
    char *pcVar9;
    struct astruct_825 *paVar10;

    puVar3 = (u8 *) (param_1 + 0x1U & 0xfffe);
    if ((puVar3 < &param_2)
        && (pstruct_d_var4 = (StructD * ) - ( puVar3 -  &param_2), PTR_LOOP_1050_000a <= pstruct_d_var4->address_offset_field_0x0)) {
        if (pstruct_d_var4->address_offset_field_0x0 < PTR_LOOP_1050_000c) {
            PTR_LOOP_1050_000c = pstruct_d_var4->address_offset_field_0x0;
        }
        // WARNING: Could not recover jumptable at 0x100025f0. Too many branches
        // WARNING: Treating indirect jump as call
        piVar5 = (i16 *) fn_ptr_param_3();
        return piVar5;
    }
    paVar10 = (astruct_825 *) ((u32)  param_2 << 0x10);
    iVar7 = 0x0;
    pass1_1000_25a8();
    pass1_1000_2913(iVar7);
    pcVar6 = poss_str_op_1000_28dc(paVar10);
    if (pcVar6 != NULL) {
        iVar7 = 0x9;
        if (*pcVar6 == 'M') {
            iVar7 = 0xf;
        }
        pcVar6 = pcVar6 + iVar7;
        iVar7 = 0x22;
        pcVar9 = pcVar6;
        do {
            if (iVar7 == 0x0) {
                break;
            }
            iVar7 += -0x1;
            pcVar2 = pcVar9;
            pcVar9 = pcVar9 + 0x1;
        } while (*pcVar2 != '\r');
        pcVar9[-0x1] = '\0';
    }
    FatalAppExit16((char *) CONCAT22(0x1050,
                                     pcVar6),
                   0x0);
    FatalExit();
    piVar5 = (i16 *) &PTR_LOOP_1050_63fe;
    do {
        piVar1 = piVar5;
        piVar5 = piVar5 + 0x1;
        iVar7 = *piVar1;
        piVar8 = piVar5;
        if ((iVar7 == param_4) || (piVar8 = (i16 *) (iVar7 + 0x1), piVar8 == NULL)) {
            return piVar8;
        }
        iVar7 = -0x1;
        do {
            if (iVar7 == 0x0) {
                break;
            }
            iVar7 += -0x1;
            piVar1 = piVar5;
            piVar5 = (i16 *) ( piVar5 + 0x1);
        } while (*(char *) piVar1 != '\0');
    } while (true);
}


/*
Unable to decompile 'exit_1000_25f2'
Cause:
Low-level Error: Overlapping input varnodes
*/


// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 : 0x1000270c
// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram










void pass1_1000_29af(u16 param_1)
{
    pass1_1000_29b5(param_1 & 0xff);
    return;
}

void pass1_1000_29b5(u16 param_1)
{
    char cVar1;

    PTR_LOOP_1050_5f88._0_1_ = (u8) param_1;
    cVar1 = (char) (param_1 >> 0x8);
    if (cVar1 != '\0') {
        goto LAB_1000_29d2;
    }
    if ((u8) PTR_LOOP_1050_5f88 < 0x22) {
        if ((u8) PTR_LOOP_1050_5f88 < 0x20) {
            if (0x13 < (u8) PTR_LOOP_1050_5f88) {
                goto LAB_1000_29cc;
            }
        } else {
            param_1 = 0x5;
        }
    } else {
        LAB_1000_29cc:
        param_1 = 0x13;
    }
    cVar1 = *(char *) (u32) ((param_1 & 0xff) + 0x5fd6);
    LAB_1000_29d2:
    PTR_LOOP_1050_5f78 = (u8 *)  cVar1;
    return;
}





u16 pass1_1000_2a00(u16 *param_1)
{
    u16 uVar1;
    bool bVar2;
    i16 *piVar3;
    u16 uVar4;
    i16 unaff_BP;
    u16 uVar5;
    u16 unaff_CS;
    u8 *puStack20;
    char local_10;
    u8 uStack15;
    u8 local_e[0x8];
    u16 uStack6;
    u16 local_4;
    i16 iStack2;

    iStack2 = unaff_BP + 0x1;
    local_4 = SUB42(0x1050,
                    0x0);
    uVar5 = 0xffff;
    if ((*(u8 *) (param_1 + 0x5) & 0x40) != 0x0) {
        *(u8 *) (param_1 + 0x5) = 0x0;
        return 0xffff;
    }
    if ((*(u8 *) (param_1 + 0x5) & 0x83) == 0x0) {
        goto LAB_1000_2af2;
    }
    uVar5 = pass1_1000_2fa4((i16 *) param_1);
    uStack6 = param_1[0x7a];
    pass1_1000_2cb0(param_1);
    uVar1 =  *(u8 *) ( param_1 + 0xb);
    if ( u16_1050_5f8a <  uVar1) {
        piVar3 = pass1_1000_55b1(unaff_CS,
                                 uVar1);
        if ( piVar3 < 0x0) {
            goto LAB_1000_2a6a;
        }
        LAB_1000_2a82:
        bVar2 = false;
    } else {
        uVar4 = dos3_call_op_1000_35fe( *(u8 *) ( param_1 + 0xb),
                                       (i16) &iStack2);
        if (-0x1 <  uVar4) {
            goto LAB_1000_2a82;
        }
        LAB_1000_2a6a:
        bVar2 = true;
    }
    if (!bVar2) {
        if (uStack6 == 0x0) {
            goto LAB_1000_2af2;
        }
        unk_str_op_1000_3d3e((char *) CONCAT22(0x1050,
                                               &local_10),
                             s___1050_5fea);
        puStack20 = local_e;
        if (local_10 == '\\') {
            puStack20 = &uStack15;
        } else {
            pass1_1000_3cea(CONCAT22(0x1050,
                                     &local_10),
                            s___1050_5fec);
        }
        pass1_1000_3e82(uStack6,
                        (u8 *) CONCAT22(0x1050,
                                        puStack20),
                        0xa);
        uVar4 = dos3_call_1000_514e();
        if (uVar4 == 0x0) {
            goto LAB_1000_2af2;
        }
    }
    uVar5 = 0xffff;
    LAB_1000_2af2:
    *(u8 *) (param_1 + 0x5) = 0x0;
    return uVar5;
}

u16 *pass1_1000_2b02(u16 param_1,
                     u16 param_2,
                     u16 param_3,
                     u16 param_4,
                     u16 param_5,
                     u8 param_6)
{
    u16 *puVar1;

    puVar1 = pass1_1000_35aa();
    if ((param_1 |  puVar1) == 0x0) {
        puVar1 = NULL;
    } else {
        puVar1 = pass1_1000_2d34(param_2,
                                 param_3,
                                 (u8 *) CONCAT22(param_5,
                                                 param_4),
                                 param_6,
                                 puVar1);
    }
    return puVar1;
}

void pass1_1000_2b3c(u16 param_1,
                     u16 param_2,
                     u16 param_3,
                     u16 param_4,
                     u16 param_5,
                     i16 param_6)
{
    pass1_1000_2b02(param_1,
                    param_2,
                    param_3,
                    param_4,
                    param_5,
                    0x0);
    return;
}

u16 pass1_1000_2b5c(u16 param_1,
                    u16 param_2,
                    u16 param_3,
                    u16 param_4)
{
    u16 uVar1;
    u16 uVar2;

    uVar1 = pass1_1000_2e74((u16 *) param_1);
    uVar2 = FUN_1000_30b4();
    pass1_1000_2f00(uVar1,
                    (i16 *) param_1);
    return uVar2;
}

void pass1_1000_2ba0(u8 param_1)
{
    pass1_1000_3024();
    if (u8_1050_5fc9 != '\0') {
        pass1_1000_3f5c();
    }
    return;
}

u16 mem_1000_2bb6(u16 param_1,
                  u16 param_2,
                  i16 *param_3)
{
    i16 *piVar1;
    i16 iVar2;
    i16 *piVar3;
    u8 bVar4;
    u8 *puVar5;
    u8 *puVar6;
    u8 *puVar7;

    piVar3 = param_3;
    bVar4 = *(u8 *) (param_3 + 0x5);
    if (((bVar4 & 0x82) != 0x0) && ((bVar4 & 0x40) == 0x0)) {
        param_3[0x2] = 0x0;
        if ((bVar4 & 0x1) != 0x0) {
            if ((bVar4 & 0x10) == 0x0) {
                goto LAB_1000_2c37;
            }
            *param_3 = param_3[0x3];
            bVar4 &= 0xfe;
        }
        *(u8 *) (param_3 + 0x5) = bVar4 & 0xef | 0x2;
        puVar7 = (u8 *)  *(u8 *) ( param_3 + 0xb);
        if (((bVar4 & 0x8) == 0x0) && (((bVar4 & 0x4) != 0x0 || (((*(u8 *) (param_3 + 0x78) & 0x1) == 0x0 && ((
            (u16_1050_61ec != 0x0
                && (((param_3 == (i16 *) 0x621c || (param_3 == (i16 *) 0x6228)) && ((puVar7[0x5f90] & 0x40) != 0x0))))
                || (mem_1000_2ce8(param_1,
                                  param_3), (*(u8 *) (piVar3 + 0x5) & 0x8) == 0x0)))))))) {
            puVar5 = mixed_dos3_call_1000_39f2(puVar7,
                                               (char *) CONCAT22(0x1050,
                                                                 &param_2),
                                               (u8 *) ( &PTR_LOOP_1050_0000 + 0x1));
            puVar6 = (u8 *) ( &PTR_LOOP_1050_0000 + 0x1);
        } else {
            iVar2 = piVar3[0x3];
            puVar6 = (u8 *) (*piVar3 - iVar2);
            *piVar3 = iVar2 + 0x1;
            piVar3[0x2] = piVar3[0x79] + -0x1;
            if (puVar6 == NULL) {
                puVar5 = NULL;
                if ((puVar7[0x5f90] & 0x20) != 0x0) {
                    mixed_dos3_call_1000_3636( puVar7,
                                              0x0,
                                              0x0,
                                              0x2);
                    puVar5 = NULL;
                    puVar6 = puVar5;
                }
            } else {
                puVar5 = mixed_dos3_call_1000_39f2(puVar7,
                                                   (char *) CONCAT22(piVar3[0x4],
                                                                     piVar3[0x3]),
                                                   puVar6);
            }
            **(u8 **) (piVar3 + 0x3) = (char) param_2;
        }
        if (puVar5 == puVar6) {
            return param_2 & 0xff;
        }
    }
    LAB_1000_2c37:
    piVar1 = piVar3 + 0x5;
    *(u8 *) piVar1 = *(u8 *) piVar1 | 0x20;
    return 0xffff;
}

void pass1_1000_2cb0(u16 *param_1)
{
    u16 *puVar1;
    u8 bVar2;

    bVar2 = *(u8 *) (param_1 + 0x5);
    if (((bVar2 & 0x83) != 0x0) && ((bVar2 & 0x8) != 0x0)) {
        pass1_1000_16ee(param_1[0x3],
                        param_1[0x4]);
        puVar1 = param_1 + 0x5;
        *(u8 *) puVar1 = *(u8 *) puVar1 & 0xf7;
        param_1[0x3] = 0x0;
        param_1[0x4] = 0x0;
        *param_1 = 0x0;
        param_1[0x1] = 0x0;
        param_1[0x2] = 0x0;
    }
    return;
}

void mem_1000_2ce8(u16 param_1,
                   i16 *param_2)
{
    i16 *piVar1;
    u16 uVar2;

    uVar2 = mem_1000_167a(param_1,
                          0x200);
    if (param_1 == 0x0) {
        piVar1 = param_2 + 0x5;
        *(u8 *) piVar1 = *(u8 *) piVar1 | 0x4;
        param_2[0x79] = 0x1;
        param_1 =  0x1050;
        uVar2 =  param_2 + 0xf1;
    } else {
        piVar1 = param_2 + 0x5;
        *(u8 *) piVar1 = *(u8 *) piVar1 | 0x8;
        param_2[0x79] = 0x200;
    }
    param_2[0x1] = param_1;
    *param_2 = uVar2;
    param_2[0x4] = param_1;
    param_2[0x3] = uVar2;
    param_2[0x2] = 0x0;
    return;
}

u16 *pass1_1000_2d34(u16 param_1,
                     u16 param_2,
                     u8 *param_3,
                     u8 param_4,
                     u16 *param_5)
{
    u8 bVar1;
    bool bVar2;
    bool bVar3;
    u16 uVar4;
    u16 in_stack_0000ffd8;
    u8 uStack14;
    u8 bStack8;
    u8 uStack6;

    bStack8 = (u8) PTR_LOOP_1050_6062;
    bVar3 = false;
    bVar1 = *param_3;
    if (bVar1 == 0x77) {
        uVar4 = 0x301;
    } else {
        if (0x77 < bVar1) {
            return NULL;
        }
        if (bVar1 != 0x61) {
            if (bVar1 != 0x72) {
                return NULL;
            }
            uVar4 = 0x0;
            uStack6 = 0x1;
            goto LAB_1000_2d6c;
        }
        uVar4 = 0x109;
    }
    uStack6 = 0x2;
    LAB_1000_2d6c:
    bVar2 = true;
    LAB_1000_2d71:
    param_3 = (u8 *) ((u32) param_3 & 0xffff0000 | (u32) ( param_3 + 0x1));
    if ((*param_3 == 0x0) || (!bVar2)) {
        uVar4 = mixed_dos3_call_1000_370a(in_stack_0000ffd8,
                                          param_1,
                                          param_2,
                                          uVar4,
                                          param_4,
                                          0x1a4);
        if ( uVar4 < 0x0) {
            return NULL;
        }
        PTR_LOOP_1050_5fee = PTR_LOOP_1050_5fee + 0x1;
        *(u8 *) (param_5 + 0x5) = uStack6;
        param_5[0x1] = 0x0;
        *param_5 = 0x0;
        param_5[0x4] = 0x0;
        param_5[0x3] = 0x0;
        uStack14 = (u8) uVar4;
        *(u8 *) ( param_5 + 0xb) = uStack14;
        *(u8 *) (param_5 + 0x78) = bStack8;
        param_5[0x2] = 0x0;
        param_5[0x7a] = 0x0;
        return param_5;
    }
    bVar1 = *param_3;
    if (bVar1 == 0x74) {
        if ((uVar4 & 0xc000) == 0x0) {
            uVar4 |= 0x4000;
            goto LAB_1000_2d71;
        }
    } else {
        if (0x74 < bVar1) {
            goto LAB_1000_2da4;
        }
        if (bVar1 == 0x2b) {
            if ((uVar4 & 0x2) != 0x0) {
                goto LAB_1000_2da4;
            }
            uVar4 = uVar4 & 0xfffe | 0x2;
            uStack6 = 0x80;
            goto LAB_1000_2d71;
        }
        if (bVar1 == 0x62) {
            if ((uVar4 & 0xc000) == 0x0) {
                uVar4 |= 0x8000;
                goto LAB_1000_2d71;
            }
        } else {
            if (bVar1 != 0x63) {
                if ((bVar1 != 0x6e) || (bVar3)) {
                    goto LAB_1000_2da4;
                }
                bVar3 = true;
                bStack8 &= 0xbf;
                goto LAB_1000_2d71;
            }
            if (!bVar3) {
                bVar3 = true;
                bStack8 |= 0x40;
                goto LAB_1000_2d71;
            }
        }
    }
    LAB_1000_2da4:
    bVar2 = false;
    goto LAB_1000_2d71;
}

u16 pass1_1000_2e74(u16 *param_1)
{
    u16 *puVar1;
    u16 uVar2;
    u16 uVar3;
    u16 *puVar4;
    u16 *puVar5;

    if (u16_1050_61ec != 0x0) {
        puVar5 = param_1 + 0x78;
        puVar4 = (u16 *) 0x5ff2;
        if ((param_1 == (u16 *) 0x621c) || (puVar4 = (u16 *) &PTR_LOOP_1050_5ff6, param_1 == (u16 *) 0x6228)) {
            if (((*(u8 *) (param_1 + 0x5) & 0xc) == 0x0) && ((*(u8 *) puVar5 & 0x1) == 0x0)) {
                uVar2 = *puVar4;
                uVar3 = puVar4[0x1];
                if ((uVar2 | uVar3) == 0x0) {
                    uVar2 = mem_1000_167a(uVar3,
                                          0x200);
                    if (uVar3 == 0x0) {
                        return 0x0;
                    }
                    *puVar4 = uVar2;
                    puVar4[0x1] = uVar3;
                }
                param_1[0x3] = uVar2;
                param_1[0x4] = uVar3;
                *param_1 = uVar2;
                param_1[0x1] = uVar3;
                param_1[0x2] = 0x200;
                param_1[0x79] = 0x200;
                puVar1 = param_1 + 0x5;
                *(u8 *) puVar1 = *(u8 *) puVar1 | 0x2;
                *(u8 *) puVar5 = 0x11;
                return 0x1;
            }
        } else if ((u8) u16_1050_5f8a <= *(u8 *) ( param_1 + 0xb)) {
            puVar1 = puVar5;
            *(u8 *) puVar1 = *(u8 *) puVar1 | 0x10;
        }
    }
    return 0x0;
}

void pass1_1000_2f00(i16 param_1,
                     i16 *param_2)
{
    if (((*(u8 *) (param_2 + 0x78) & 0x10) != 0x0)
        && ((*(u8 *) (*(u8 *) ( param_2 + 0xb) + 0x5f90) & 0x40) != 0x0)) {
        pass1_1000_2fa4(param_2);
        if (param_1 != 0x0) {
            *(u8 *) (param_2 + 0x78) = 0x0;
            param_2[0x79] = 0x0;
            *param_2 = 0x0;
            param_2[0x1] = 0x0;
            param_2[0x3] = 0x0;
            param_2[0x4] = 0x0;
        }
    }
    return;
}

u16 pass1_1000_2f48(i32 param_1)
{
    u16 uVar1;
    u8 *puVar2;

    if (param_1 == 0x0) {
        uVar1 = pass1_1000_3038(0x0);
    } else {
        uVar1 = pass1_1000_2fa4((i16 *) param_1);
        if (uVar1 == 0x0) {
            if ((*(u8 *) ((i16 *) param_1 + 0x78) & 0x40) != 0x0) {
                puVar2 = pass1_1000_400a( *(u8 *) ( (i16 *) param_1 + 0xb));
                uVar1 = - (puVar2 != NULL);
            }
        } else {
            uVar1 = 0xffff;
        }
    }
    return uVar1;
}

u16 pass1_1000_2fa4(i16 *param_1)
{
    i16 *piVar1;
    u8 bVar2;
    i16 iVar3;
    u8 *puVar4;
    u8 *puVar5;
    u16 uVar6;

    uVar6 = 0x0;
    bVar2 = *(u8 *) (param_1 + 0x5);
    if (((bVar2 & 0x3) == 0x2) && (((bVar2 & 0x8) != 0x0 || ((*(u8 *) (param_1 + 0x78) & 0x1) != 0x0)))) {
        puVar4 = (u8 *) (*param_1 - param_1[0x3]);
        if (0x0 <  puVar4) {
            puVar5 = mixed_dos3_call_1000_39f2((u8 *)  *(u8 *) ( param_1 + 0xb),
                                               (char *) CONCAT22(param_1[0x4],
                                                                 param_1[0x3]),
                                               puVar4);
            if (puVar5 == puVar4) {
                if ((*(u8 *) (param_1 + 0x5) & 0x80) != 0x0) {
                    piVar1 = param_1 + 0x5;
                    *(u8 *) piVar1 = *(u8 *) piVar1 & 0xfd;
                }
            } else {
                piVar1 = param_1 + 0x5;
                *(u8 *) piVar1 = *(u8 *) piVar1 | 0x20;
                uVar6 = 0xffff;
            }
        }
    }
    iVar3 = param_1[0x4];
    *param_1 = param_1[0x3];
    param_1[0x1] = iVar3;
    param_1[0x2] = 0x0;
    return uVar6;
}

void pass1_1000_3024(void)
{
    pass1_1000_3038(0x1);
    return;
}

i16 pass1_1000_3038(i16 param_1)
{
    u16 uVar1;
    u8 *puVar2;
    i16 iVar3;
    i16 iStack4;

    iVar3 = 0x0;
    iStack4 = 0x0;
    for (puVar2 = (u8 *) &PTR_LOOP_1050_6210; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0xc) {
        if ((param_1 == 0x1) && ((puVar2[0xa] & 0x83) != 0x0)) {
            uVar1 = pass1_1000_2f48(CONCAT22(0x1050,
                                             puVar2));
            if (uVar1 != 0xffff) {
                iVar3 += 0x1;
            }
        } else if ((param_1 == 0x0) && (((puVar2[0xa] & 0x2) != 0x0 && (uVar1 = pass1_1000_2f48(CONCAT22(0x1050,
                                                                                                         puVar2)), uVar1
            == 0xffff)))) {
            iStack4 = -0x1;
        }
    }
    if (param_1 == 0x1) {
        iStack4 = iVar3;
    }
    return iStack4;
}

u16 FUN_1000_30b4(void)
{
    u8 bVar1;
    u8 bVar2;
    i16 unaff_BP;
    i16 iVar3;
    u16 unaff_SI;
    u16 unaff_CS;
    u8 *in_stack_00000008;
    u16 uVar4;

    iVar3 = unaff_BP + 0x1;
    uVar4 = SUB42(0x1050,
                  0x0);
    exit_1000_25f2(0x214,
                   0x30c5,
                   unaff_CS,
                    0x1050);
    bVar1 = *in_stack_00000008;
    if (bVar1 == 0x0) {
        return 0x0;
    }
    if ((u8) (bVar1 - 0x20) < 0x59) {
        bVar2 = *(u8 *) (u32) ((u8) (bVar1 - 0x20) + 0x5ffe) & 0xf;
    } else {
        bVar2 = 0x0;
    }
    // WARNING: Could not emulate address calculation at 0x10003101
    // WARNING: Treating indirect jump as call
    uVar4 = (code) ((char) (*(u8 *) (u32) ((u8) (bVar2 * '\b') + 0x5ffe) >> 0x4) * 0x2 + 0x30a4))
    (unaff_SI & 0xff00 |  bVar1, uVar4, iVar3);
    return uVar4;
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_3113(void)
{
    char cVar1;
    char *pcVar2;
    u8 bVar3;
    u16 uVar4;
    i16 unaff_BP;

    FUN_1000_3552();
    pcVar2 = *(char **) (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar1 - 0x20U) < 0x59) {
            bVar3 = *(u8 *) (u32) ((u8) (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *(u8 *) (u32) ((u8) (bVar3 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (code) ((char) bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_311e(void)
{
    char cVar1;
    char *pcVar2;
    u8 bVar3;
    u16 uVar4;
    i16 unaff_BP;

    (unaff_BP + -0x12) = 0x0;
    (unaff_BP + -0xc) = 0x0;
    (unaff_BP + -0x14) = 0x0;
    (unaff_BP + -0x6) = 0x20;
    (unaff_BP + -0xe) = 0xffff;
    pcVar2 = *(char **) (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar1 - 0x20U) < 0x59) {
            bVar3 = *(u8 *) (u32) ((u8) (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *(u8 *) (u32) ((u8) (bVar3 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (code) ((char) bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_3134(void)
{
    u8 *pbVar1;
    char cVar2;
    char *pcVar3;
    u8 bVar4;
    u16 uVar5;
    i16 unaff_BP;

    cVar2 = *(char *) (unaff_BP + -0x4);
    if (cVar2 == '-') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x4;
    } else if (cVar2 == '+') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x1;
    } else if (cVar2 == ' ') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x2;
    } else if (cVar2 == '#') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x80;
    } else {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x8;
    }
    pcVar3 = *(char **) (unaff_BP + 0xa);
    cVar2 = *pcVar3;
    (unaff_BP + 0xa) =  pcVar3 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar2;
    if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar2 - 0x20U) < 0x59) {
            bVar4 = *(u8 *) (u32) ((u8) (cVar2 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar4 = 0x0;
        }
        bVar4 = *(u8 *) (u32) ((u8) (bVar4 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 = (code) ((char) bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_3168(void)
{
    u8 *pbVar1;
    char cVar2;
    char *pcVar3;
    u8 bVar4;
    u16 uVar5;
    i16 unaff_BP;

    cVar2 = *(char *) (unaff_BP + -0x4);
    if (cVar2 == '*') {
        uVar5 = pass1_1000_34cf();
        if ( uVar5 < 0x0) {
            uVar5 = -uVar5;
            pbVar1 = (u8 *) (unaff_BP + -0x6);
            *pbVar1 = *pbVar1 | 0x4;
        }
    } else {
        uVar5 = (unaff_BP + -0xc) * 0xa +  (u8) (cVar2 - 0x30);
    }
    (unaff_BP + -0xc) = uVar5;
    pcVar3 = *(char **) (unaff_BP + 0xa);
    cVar2 = *pcVar3;
    (unaff_BP + 0xa) =  pcVar3 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar2;
    if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar2 - 0x20U) < 0x59) {
            bVar4 = *(u8 *) (u32) ((u8) (cVar2 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar4 = 0x0;
        }
        bVar4 = *(u8 *) (u32) ((u8) (bVar4 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 = (code) ((char) bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_3194(void)
{
    char cVar1;
    char *pcVar2;
    u8 bVar3;
    u16 uVar4;
    i16 unaff_BP;

    (unaff_BP + -0xe) = 0x0;
    pcVar2 = *(char **) (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar1 - 0x20U) < 0x59) {
            bVar3 = *(u8 *) (u32) ((u8) (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *(u8 *) (u32) ((u8) (bVar3 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (code) ((char) bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_319c(void)
{
    char cVar1;
    char *pcVar2;
    u8 bVar3;
    u16 uVar4;
    i16 unaff_BP;

    cVar1 = *(char *) (unaff_BP + -0x4);
    if (cVar1 == '*') {
        uVar4 = pass1_1000_34cf();
        if ( uVar4 < 0x0) {
            uVar4 = 0xffff;
        }
    } else {
        uVar4 = (unaff_BP + -0xe) * 0xa +  (u8) (cVar1 - 0x30);
    }
    (unaff_BP + -0xe) = uVar4;
    pcVar2 = *(char **) (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar1 - 0x20U) < 0x59) {
            bVar3 = *(u8 *) (u32) ((u8) (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *(u8 *) (u32) ((u8) (bVar3 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (code) ((char) bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_31c5(void)
{
    u8 *pbVar1;
    char cVar2;
    char *pcVar3;
    u8 bVar4;
    u16 uVar5;
    i16 unaff_BP;

    cVar2 = *(char *) (unaff_BP + -0x4);
    if (cVar2 == 'l') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x10;
    } else if (cVar2 == 'F') {
        pbVar1 = (u8 *) (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x20;
    } else if (cVar2 == 'N') {
        pbVar1 = (u8 *) (unaff_BP + -0x5);
        *pbVar1 = *pbVar1 | 0x10;
    } else if (cVar2 == 'L') {
        pbVar1 = (u8 *) (unaff_BP + -0x5);
        *pbVar1 = *pbVar1 | 0x4;
    } else {
        pbVar1 = (u8 *) (unaff_BP + -0x5);
        *pbVar1 = *pbVar1 | 0x8;
    }
    pcVar3 = *(char **) (unaff_BP + 0xa);
    cVar2 = *pcVar3;
    (unaff_BP + 0xa) =  pcVar3 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar2;
    if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar2 - 0x20U) < 0x59) {
            bVar4 = *(u8 *) (u32) ((u8) (cVar2 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar4 = 0x0;
        }
        bVar4 = *(u8 *) (u32) ((u8) (bVar4 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 = (code) ((char) bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_31f7(u16 param_1)
{
    i16 *piVar1;
    u8 *pbVar2;
    u16 *puVar3;
    char cVar4;
    char *pcVar5;
    u8 bVar6;
    u16 uVar7;
    i16 iVar8;
    char *pcVar9;
    u16 uVar10;
    u16 DX_REG;
    u16 DX_REG_00;
    u16 DX_REG;
    u16 DX_REG_02;
    u16 DX_REG_03;
    u16 DX_REG_04;
    i16 unaff_BP;
    u16 *unaff_DI;
    u16 *puVar11;
    i16 unaff_ES;
    u8 in_AF;
    bool bVar12;
    u32 uVar13;

    cVar4 = *(char *) (unaff_BP + -0x4);
    if ((cVar4 == 'd') || (cVar4 == 'i')) {
        pbVar2 = (u8 *) (unaff_BP + -0x6);
        *pbVar2 = *pbVar2 | 0x40;
        LAB_1000_3399:
        *(u8 *) (unaff_BP + -0x8) = 0xa;
        LAB_1000_33d4:
        if ((*(u8 *) (unaff_BP + -0x6) & 0x10) == 0x0) {
            uVar7 = pass1_1000_34cf();
            if ((*(u8 *) (unaff_BP + -0x6) & 0x40) == 0x0) {
                uVar13 = (u32) uVar7;
            } else {
                uVar13 = (u32)  uVar7;
            }
        } else {
            uVar13 = pass1_1000_34d8();
        }
        if (((*(u8 *) (unaff_BP + -0x6) & 0x40) != 0x0) && ((long) uVar13 < 0x0)) {
            pbVar2 = (u8 *) (unaff_BP + -0x5);
            *pbVar2 = *pbVar2 | 0x1;
            uVar13 = CONCAT22(-( (uVar13 >> 0x10) +  ( uVar13 != 0x0)),
                              - uVar13);
        }
        param_1 =  (uVar13 >> 0x10);
        if ((unaff_BP + -0xe) < 0x0) {
            (unaff_BP + -0xe) = 0x1;
        } else {
            pbVar2 = (u8 *) (unaff_BP + -0x6);
            *pbVar2 = *pbVar2 & 0xf7;
        }
        if (uVar13 == 0x0) {
            (unaff_BP + -0x12) = 0x0;
        }
        unaff_DI = (u16 *) (unaff_BP + -0x17);
        unaff_ES =  0x1050;
        pcVar9 = (char *)  *(u8 *) (unaff_BP + -0x8);
        pass1_1000_356e( uVar13,
                         pcVar9,
                        param_1);
        if (((*(u8 *) (unaff_BP + -0x5) & 0x2) != 0x0) && ((pcVar9 == NULL || (*(char *) unaff_DI != '0')))) {
            unaff_DI = (u16 *) (unaff_BP + -0x18);
            *(char *) unaff_DI = '0';
            pcVar9 = pcVar9 + 0x1;
        }
    } else {
        if (cVar4 == 'u') {
            goto LAB_1000_3399;
        }
        if (cVar4 == 'X') {
            *(u8 *) (unaff_BP + -0x3) = 0x7;
            LAB_1000_33a9:
            if ((*(u8 *) (unaff_BP + -0x6) & 0x80) != 0x0) {
                (unaff_BP + -0x12) = 0x2;
                *(u8 *) (unaff_BP + -0x10) = 0x30;
                *(char *) (unaff_BP + -0xf) = *(char *) (unaff_BP + -0x3) + 'Q';
            }
            *(u8 *) (unaff_BP + -0x8) = 0x10;
            goto LAB_1000_33d4;
        }
        if (cVar4 == 'x') {
            *(u8 *) (unaff_BP + -0x3) = 0x27;
            goto LAB_1000_33a9;
        }
        if (cVar4 == 'o') {
            if ((*(u8 *) (unaff_BP + -0x6) & 0x80) != 0x0) {
                pbVar2 = (u8 *) (unaff_BP + -0x5);
                *pbVar2 = *pbVar2 | 0x2;
            }
            *(u8 *) (unaff_BP + -0x8) = 0x8;
            goto LAB_1000_33d4;
        }
        if (cVar4 == 'c') {
            uVar7 = pass1_1000_34cf();
            unaff_ES =  0x1050;
            *(u8 *) (unaff_BP + -0x216) = (char) uVar7;
            unaff_DI = (u16 *) (unaff_BP + -0x216);
            pcVar9 = (char *) 0x1;
        } else if (cVar4 == 's') {
            uVar13 = pass1_1000_34e6(param_1);
            param_1 =  (uVar13 >> 0x10);
            if ((unaff_DI == NULL) && (unaff_ES == 0x0)) {
                unaff_ES =  0x1050;
                unaff_DI = (u16 *) 0x6057;
                pcVar9 = DAT_1050_605d;
            } else {
                iVar8 = (unaff_BP + -0xe);
                puVar11 = unaff_DI;
                if (iVar8 != 0x0) {
                    bVar12 = true;
                    do {
                        if (iVar8 == 0x0) {
                            break;
                        }
                        iVar8 += -0x1;
                        puVar3 = puVar11;
                        puVar11 = (u16 *) ( puVar11 + 0x1);
                        bVar12 = *(char *) puVar3 == '\0';
                    } while (!bVar12);
                    if (bVar12) {
                        puVar11 = (u16 *) ( puVar11 + -0x1);
                    }
                }
                pcVar9 = (char *) ( puVar11 -  unaff_DI);
            }
        } else {
            if (cVar4 == 'n') {
                pass1_1000_34e6(param_1);
                *unaff_DI = (unaff_BP + -0xa);
                if ((*(u8 *) (unaff_BP + -0x6) & 0x10) != 0x0) {
                    unaff_DI[0x1] = 0x0;
                }
                goto LAB_1000_30cf;
            }
            if (cVar4 != 'p') {
                if ((cVar4 == 'E') || (cVar4 == 'G')) {
                    piVar1 = (i16 *) (unaff_BP + -0x14);
                    *piVar1 = *piVar1 + 0x1;
                }
                pbVar2 = (u8 *) (unaff_BP + -0x6);
                *pbVar2 = *pbVar2 | 0x40;
                bVar6 = *(u8 *) (unaff_BP + -0x4) | 0x20;
                iVar8 = (unaff_BP + -0xe);
                if (iVar8 < 0x1) {
                    if (iVar8 == 0x0) {
                        if (bVar6 == 0x67) {
                            (unaff_BP + -0xe) = 0x1;
                        }
                    } else {
                        (unaff_BP + -0xe) = 0x6;
                    }
                }
                unaff_DI = (u16 *) (unaff_BP + -0x216);
                if ((*(u8 *) (unaff_BP + -0x5) & 0x4) == 0x0) {
                    ((code) PTR_s_3_wav_1050_25cc_1050_6068)();
                    piVar1 = (i16 *) (unaff_BP + 0xe);
                    *piVar1 = *piVar1 + 0x8;
                    param_1 = DX_REG_00;
                } else {
                    ((code) PTR_s_3_wav_1050_25cc_1050_607c)();
                    piVar1 = (i16 *) (unaff_BP + 0xe);
                    *piVar1 = *piVar1 + 0xa;
                    param_1 = DX_REG;
                }
                if (((*(u8 *) (unaff_BP + -0x6) & 0x80) != 0x0) && ((unaff_BP + -0xe) == 0x0)) {
                    ((code) PTR_s_3_wav_1050_25cc_1050_6074)();
                    param_1 = DX_REG;
                }
                if ((bVar6 == 0x67) && (((unaff_BP + -0x6) & 0x80) == 0x0)) {
                    ((code) PTR_s_3_wav_1050_25cc_1050_6070)();
                    param_1 = DX_REG_02;
                }
                unaff_ES =  0x1050;
                if (*(char *) unaff_DI == '-') {
                    unaff_DI = (u16 *) (unaff_BP + -0x215);
                    pbVar2 = (u8 *) (unaff_BP + -0x5);
                    *pbVar2 = *pbVar2 | 0x1;
                }
                iVar8 = -0x1;
                puVar11 = unaff_DI;
                do {
                    if (iVar8 == 0x0) {
                        break;
                    }
                    iVar8 += -0x1;
                    puVar3 = puVar11;
                    puVar11 = (u16 *) ( puVar11 + 0x1);
                } while (*(char *) puVar3 != '\0');
                pcVar9 = (char *) ( puVar11 + (-0x1 -  unaff_DI));
                goto LAB_1000_3444;
            }
            if ((*(u8 *) (unaff_BP + -0x6) & 0x30) == 0x0) {
                uVar7 = pass1_1000_34cf();
                uVar13 = (u32) uVar7;
                LAB_1000_32d8:
                *(u8 *) (unaff_BP + -0x3) = 0x7;
                param_1 = 0x0;
                pass1_1000_356e( uVar13,
                                0x10,
                                0x0);
                pcVar9 = (char *) 0x4;
            } else {
                uVar13 = pass1_1000_34d8();
                uVar10 =  (uVar13 >> 0x10);
                if ((*(u8 *) (unaff_BP + -0x5) & 0x18) != 0x0) {
                    goto LAB_1000_32d8;
                }
                *(u8 *) (unaff_BP + -0x3) = 0x7;
                pass1_1000_356e( uVar13,
                                0x10,
                                0x0);
                param_1 = 0x0;
                pass1_1000_356e(uVar10,
                                0x10,
                                0x0);
                *(u8 *) (unaff_BP + -0x212) = 0x3a;
                pcVar9 = (char *) 0x9;
            }
            unaff_ES =  0x1050;
            unaff_DI = (u16 *) (unaff_BP + -0x216);
        }
    }
    LAB_1000_3444:
    if ((*(u8 *) (unaff_BP + -0x6) & 0x40) != 0x0) {
        if ((*(u8 *) (unaff_BP + -0x5) & 0x1) == 0x0) {
            if ((*(u8 *) (unaff_BP + -0x6) & 0x1) == 0x0) {
                if ((*(u8 *) (unaff_BP + -0x6) & 0x2) != 0x0) {
                    *(u8 *) (unaff_BP + -0x10) = 0x20;
                    (unaff_BP + -0x12) = 0x1;
                }
            } else {
                *(u8 *) (unaff_BP + -0x10) = 0x2b;
                (unaff_BP + -0x12) = 0x1;
            }
        } else {
            *(u8 *) (unaff_BP + -0x10) = 0x2d;
            (unaff_BP + -0x12) = 0x1;
        }
    }
    if ((*(u8 *) (unaff_BP + -0x6) & 0xc) == 0x0) {
        FUN_1000_3552(pcVar9,
                      unaff_DI,
                      unaff_ES);
        param_1 = DX_REG_03;
    }
    pass1_1000_3534(in_AF,
                    (unaff_BP + -0x12),
                    param_1);
    if (((*(u8 *) (unaff_BP + -0x6) & 0x8) != 0x0) && ((*(u8 *) (unaff_BP + -0x6) & 0x4) == 0x0)) {
        FUN_1000_3552(pcVar9,
                      unaff_DI,
                      unaff_ES);
        param_1 = DX_REG_04;
    }
    pass1_1000_3534(in_AF,
                    pcVar9,
                    param_1);
    if ((*(u8 *) (unaff_BP + -0x6) & 0x4) != 0x0) {
        FUN_1000_3552();
    }
    LAB_1000_30cf:
    pcVar5 = *(char **) (unaff_BP + 0xa);
    cVar4 = *pcVar5;
    (unaff_BP + 0xa) =  pcVar5 + 0x1;
    *(char *) (unaff_BP + -0x4) = cVar4;
    if ((cVar4 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ((u8) (cVar4 - 0x20U) < 0x59) {
            bVar6 = *(u8 *) (u32) ((u8) (cVar4 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar6 = 0x0;
        }
        bVar6 = *(u8 *) (u32) ((u8) (bVar6 * '\b' + *(char *) (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *) (unaff_BP + -0x7) = bVar6;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar7 = (code) ((char) bVar6 * 0x2 + 0x30a4))();
        return uVar7;
    }
    return (unaff_BP + -0xa);
}

u16 pass1_1000_34cf(void)
{
    u16 uVar1;
    u16 *puVar2;
    i16 unaff_BP;

    puVar2 = (u16 *) (unaff_BP + 0xe);
    uVar1 = *puVar2;
    (unaff_BP + 0xe) =  puVar2 + 0x2;
    return uVar1;
}

u32 pass1_1000_34d8(void)
{
    u16 uVar1;
    u16 uVar2;
    u16 *puVar3;
    i16 unaff_BP;

    puVar3 = (u16 *) (u32) (unaff_BP + 0xe);
    uVar1 = *puVar3;
    uVar2 = ( puVar3 + 0x2);
    (unaff_BP + 0xe) =  puVar3 + 0x4;
    return CONCAT22(uVar2,
                    uVar1);
}

u32 pass1_1000_34e6(u16 param_1)
{
    u16 uVar1;
    i16 unaff_BP;
    u32 uVar2;

    if ((*(u8 *) (unaff_BP + -0x6) & 0x20) != 0x0) {
        uVar2 = pass1_1000_34d8();
        return uVar2;
    }
    uVar1 = pass1_1000_34cf();
    if (uVar1 == 0x0) {
        return (u32) param_1 << 0x10;
    }
    return CONCAT22(param_1,
                    uVar1);
}

i16 pass1_1000_3503(u16 param_1,
                    u16 param_2)
{
    i16 *piVar1;
    char *pcVar2;
    char **ppcVar3;
    u16 uVar4;
    i16 *piVar5;
    i16 unaff_BP;
    u16 uVar6;

    ppcVar3 = (char **) *(i16 **) (unaff_BP + 0x6);
    uVar6 =  ((u32) ppcVar3 >> 0x10);
    piVar5 = (i16 *) ppcVar3;
    piVar1 = piVar5 + 0x2;
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
        uVar4 = mem_1000_2bb6(param_2,
                               (char) param_1,
                              piVar5);
        if (uVar4 == 0xffff) {
            return -0x1;
        }
    } else {
        pcVar2 = *ppcVar3;
        *ppcVar3 = *ppcVar3 + 0x1;
        *pcVar2 = (char) param_1;
    }
    return 0x0;
}

void pass1_1000_3534(undefined1 param_1,
                     i16 param_2,
                     u16 param_3)
{
    i16 *piVar1;
    u8 *pbVar2;
    u16 in_AX;
    i16 unaff_BP;
    u8 *unaff_DI;
    u16 uVar3;
    u16 unaff_ES;

    if (param_2 != 0x0) {
        piVar1 = (i16 *) (unaff_BP + -0xa);
        *piVar1 = *piVar1 + param_2;
        uVar3 = 0x0;
        do {
            pbVar2 = unaff_DI;
            unaff_DI = unaff_DI + 0x1;
            in_AX = pass1_1000_3503(in_AX & 0xff00 |  *pbVar2,
                                    param_3);
            uVar3 |= in_AX;
            param_2 += -0x1;
        } while (param_2 != 0x0);
        if (uVar3 != 0x0) {
            (unaff_BP + -0xa) = 0xffff;
        }
    }
    return;
}

void FUN_1000_3552(void)
{
    i16 *piVar1;
    u16 param_3;
    i16 param_1;
    u16 param_2;
    i16 unaff_BP;
    u16 uVar2;

    if (param_1 != 0x0) {
        piVar1 = (i16 *) (unaff_BP + -0xa);
        *piVar1 = *piVar1 + param_1;
        uVar2 = 0x0;
        do {
            param_3 = pass1_1000_3503(param_3 & 0xff00 | param_2 & 0xff,
                                      param_2);
            uVar2 |= param_3;
            param_1 += -0x1;
        } while (param_1 != 0x0);
        if (uVar2 != 0x0) {
            (unaff_BP + -0xa) = 0xffff;
        }
    }
    return;
}

void pass1_1000_356e(u16 param_1,
                     u16 param_2,
                     u16 param_3)
{
    u8 *pbVar1;
    u32 uVar2;
    u8 bVar3;
    i16 unaff_BP;
    i16 unaff_SI;
    u8 *unaff_DI;
    u16 unaff_ES;

    while (((0x0 < unaff_SI || (param_1 != 0x0)) || (param_3 != 0x0))) {
        uVar2 = (u32) param_3;
        param_3 /= param_2;
        uVar2 = uVar2 % (u32) param_2 << 0x10 | (u32) param_1;
        param_1 =  (uVar2 / param_2);
        bVar3 = (char) (uVar2 % (u32) param_2) + 0x30;
        if (0x39 < bVar3) {
            bVar3 += *(char *) (unaff_BP + -0x3);
        }
        pbVar1 = unaff_DI;
        unaff_DI = unaff_DI + -0x1;
        *pbVar1 = bVar3;
        unaff_SI += -0x1;
    }
    return;
}

u16 *pass1_1000_35aa(void)
{
    u16 *puVar1;

    puVar1 = (u16 *) &PTR_LOOP_1050_6210;
    while (true) {
        if (PTR_LOOP_1050_5ff0 < puVar1) {
            return NULL;
        }
        if ((*(u8 *) (puVar1 + 0x5) & 0x83) == 0x0) {
            break;
        }
        puVar1 = puVar1 + 0x6;
    }
    *(u8 *) (puVar1 + 0x5) = 0x0;
    puVar1[0x2] = 0x0;
    puVar1[0x4] = 0x0;
    puVar1[0x3] = 0x0;
    puVar1[0x1] = 0x0;
    *puVar1 = 0x0;
    *(u8 *) ( puVar1 + 0xb) = 0xff;
    return puVar1;
}



// WARNING: Removing unreachable block (ram,0x10003622)

u16 dos3_call_op_1000_35fe(u16 param_1,
                           i16 param_2)
{
//    code *pcVar1;
    u16 uVar2;
    bool u16_var2;

    if (param_1 < u16_1050_5f8a) {
        u16_var2 = false;
        code7 fn_ptr_1 = (code7) swi(0x21);
        uVar2 = fn_ptr_1(param_2 + 0x1);
        if (!u16_var2) {
            *(u8 *) (param_1 + 0x5f90) = 0x0;
        }
    } else {
        uVar2 = 0x900;
        u16_var2 = true;
    }
    if (u16_var2) {
        pass1_1000_29b5(uVar2);
        return 0xffff;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x100036b5)
// WARNING: Removing unreachable block (ram,0x10003681)
// WARNING: Removing unreachable block (ram,0x100036f7)
// WARNING: Removing unreachable block (ram,0x100036d8)

void mixed_dos3_call_1000_3636(u16 param_1,
                               u16 param_2,
                               u16 param_3,
                               u16 param_4)
{
    u8 *pbVar1;
    code *pcVar2;
    u16 uVar3;
    i16 unaff_BP;
    i16 iVar4;
    bool bVar5;
    u32 uVar6;

    iVar4 = unaff_BP + 0x1;
    if (((param_1 < u16_1050_5f8a) || (u16_1050_61ec == 0x0)) || (0x2 < param_1)) {
        if ((u16_1050_6064 == 0x0) || ((param_3 & 0x8000) == 0x0)) {
            goto LAB_1000_36e3;
        }
        if (param_4 == 0x0) {
            goto LAB_1000_369b;
        }
        bVar5 = false;
        pcVar2 = (code *) swi(0x21);
        uVar6 = (*pcVar2)();
        uVar3 =  uVar6;
        if (bVar5) {
            goto LAB_1000_299d;
        }
        if ((param_4 & 0x2) == 0x0) {
            if (-0x1 <  ( ((u32) uVar6 >> 0x10) + param_3 +  CARRY2(uVar3,
                                                                                   param_2))) {
                LAB_1000_36e3:
                bVar5 = false;
                pcVar2 = (code *) swi(0x21);
                uVar3 = (*pcVar2)(iVar4);
                if (!bVar5) {
                    pbVar1 = (u8 *) (param_1 + 0x5f90);
                    bVar5 = false;
                    *pbVar1 = *pbVar1 & 0xfd;
                }
                goto LAB_1000_299d;
            }
        } else {
            pcVar2 = (code *) swi(0x21);
            uVar6 = (*pcVar2)();
            if (-0x1 <  ( ((u32) uVar6 >> 0x10) + param_3 +  CARRY2( uVar6,
                                                                                   param_2))) {
                goto LAB_1000_36e3;
            }
            pcVar2 = (code *) swi(0x21);
            (*pcVar2)();
        }
        LAB_1000_369b:
        uVar3 =  s_471_bmp_1050_1600;
    } else {
        uVar3 = 0x900;
    }
    bVar5 = true;
    LAB_1000_299d:
    if (bVar5) {
        pass1_1000_29b5(uVar3);
    }
    return;
}



// WARNING: Removing unreachable block (ram,0x10003989)
// WARNING: Removing unreachable block (ram,0x100038a1)
// WARNING: Removing unreachable block (ram,0x10003867)
// WARNING: Removing unreachable block (ram,0x10003967)
// WARNING: Removing unreachable block (ram,0x1000391a)
// WARNING: Removing unreachable block (ram,0x10003803)
// WARNING: Removing unreachable block (ram,0x100037b7)
// WARNING: Removing unreachable block (ram,0x10003799)
// WARNING: Removing unreachable block (ram,0x10003765)
// WARNING: Removing unreachable block (ram,0x100037ec)
// WARNING: Removing unreachable block (ram,0x100038d9)
// WARNING: Removing unreachable block (ram,0x100038f2)
// WARNING: Removing unreachable block (ram,0x1000393a)
// WARNING: Removing unreachable block (ram,0x1000384b)
// WARNING: Removing unreachable block (ram,0x1000388b)
// WARNING: Removing unreachable block (ram,0x100038ba)
// WARNING: Removing unreachable block (ram,0x100039b9)
// WARNING: Removing unreachable block (ram,0x1000381c)
// WARNING: Could not reconcile some variable overlaps

u16 mixed_dos3_call_1000_370a(u16 param_1,
                              u16 param_2,
                              u16 param_3,
                              u16 param_4,
                              u8 param_5,
                              u16 param_6)
{
    code *fn_ptr_1;
    u16 uVar3;
    u16 uVar2;
    i16 iVar3;
    u8 bVar2;
    u16 uVar7;
    u16 DX_REG;
    u16 uVar4;
    u16 uVar5;
    i16 unaff_BP;
    u16 uVar6;
    u16 uVar8;
    bool bVar10;
    u16 uStack14;
    u8 bVar9;
    u16 in_stack_0000fffa;
    u16 uVar9;
    u16 uVar10;
    u8 cVar12;

    uVar6 = unaff_BP + 0x1;
    param_5 = param_6;
    uVar3 = param_1 & 0xff00;
    param_1 = uVar3 | param_5;
    uVar9 = 0x0;
    if (((param_4 & 0x8000) == 0x0) && (((param_4 & 0x4000) != 0x0 || ((DAT_1050_6061 & 0x80) == 0x0)))) {
        uVar9 = 0x80;
    }
    bVar10 = false;
    fn_ptr_1 = (code *) swi(0x21);
    uVar7 = param_4;
    uVar2 = (*fn_ptr_1)();
    if (bVar10) {
        if ((uVar2 == 0x2) && ((uVar7 & 0x100) != 0x0)) {
            uVar10 = uVar9 & 0xff;
            // s_____s__lu_1050_38cd + 0x3
            param_1 =  0x38d0;
            pass1_1000_39e1();
            uVar7 = 0x0;
            _param_5 = param_6;
            LAB_1000_38e3:
            bVar10 = false;
            fn_ptr_1 = (code *) swi(0x21);
            uVar2 = (*fn_ptr_1)();
            if (bVar10) {
                goto LAB_1000_299d;
            }
            if (((char) param_1 != '\0') || (uVar5 = uVar2, uVar8 = uStack14, (param_4 & 0x2) == 0x0)) {
                fn_ptr_1 = (code *) swi(0x21);
                (*fn_ptr_1)();
                bVar10 = false;
                fn_ptr_1 = (code *) swi(0x21);
                uVar2 = (*fn_ptr_1)();
                if (bVar10) {
                    goto LAB_1000_299d;
                }
                uVar5 = uVar2;
                uVar8 = param_1;
                if (((uVar10 & 0x100) == 0x0) && (uVar8 = param_1, (_param_5 & 0x1) != 0x0)) {
                    uVar7 =  (u8) ((u8) uVar7 | 0x1);
                    bVar10 = false;
                    fn_ptr_1 = (code *) swi(0x21);
                    uVar4 = uVar2;
                    uVar2 = (*fn_ptr_1)();
                    uVar5 = uVar4;
                    uVar8 = uVar6;
                    if (bVar10) {
                        goto LAB_1000_299d;
                    }
                }
            }
            LAB_1000_3973:
            bVar9 = (u8) uVar10;
            if ((uVar10 & 0x40) == 0x0) {
                fn_ptr_1 = (code *) swi(0x21);
                (*fn_ptr_1)();
                bVar2 = 0x0;
                if ((uVar7 & 0x1) != 0x0) {
                    bVar2 = 0x10;
                }
                if ((param_4 & 0x8) != 0x0) {
                    bVar2 |= 0x20;
                }
            } else {
                bVar2 = 0x0;
            }
            if (uVar5 < &u16_1050_5f8a) {
                *(u8 *) (uVar5 + 0x5f90) = bVar2 | bVar9 | 0x1;
                return uVar5;
            }
            fn_ptr_1 = (code *) swi(0x21);
            (*fn_ptr_1)();
            uVar2 = 0x1800;
        }
    } else {
        if ((uVar7 & 0x500) != 0x500) {
            uVar10 = CONCAT11(0x1,
                              (char) uVar9);
            fn_ptr_1 = (code *) swi(0x21);
            (*fn_ptr_1)();
            if ((DX_REG & 0x80) != 0x0) {
                uVar10 |= 0x40;
            }
            uVar5 = uVar2;
            uVar8 = param_1;
            if ((uVar10 & 0x40) == 0x0) {
                if ((param_4 & 0x200) == 0x0) {
                    if (((uVar10 & 0x80) != 0x0) && ((param_4 & 0x2) != 0x0)) {
                        code fn_ptr_1 = (code) swi(0x21);
                        (fn_ptr_1)();
                        code5 fn_ptr_2 = (code5) swi(0x21);
                        iVar3 = (fn_ptr_2)();
                        if ((iVar3 != 0x0) && (param_1 = (char) (uVar3 >> 0x8), param_1 == '\x1a')) {
                            fn_ptr_1 = (code *) swi(0x21);
                            (*fn_ptr_1)(uVar6);
                            fn_ptr_1 = (code *) swi(0x21);
                            (*fn_ptr_1)();
                        }
                        uVar7 = 0x0;
                        fn_ptr_1 = (code *) swi(0x21);
                        (*fn_ptr_1)();
                        uVar5 = uVar2;
                        uVar8 = uStack14;
                    }
                } else {
                    if ((param_4 & 0x3) == 0x0) {
                        fn_ptr_1 = (code *) swi(0x21);
                        (*fn_ptr_1)();
                        fn_ptr_1 = (code *) swi(0x21);
                        (*fn_ptr_1)();
                        goto LAB_1000_38e3;
                    }
                    uVar7 = 0x0;
                    fn_ptr_1 = (code *) swi(0x21);
                    (*fn_ptr_1)();
                    uVar5 = uVar2;
                    uVar8 = param_1;
                }
            }
            goto LAB_1000_3973;
        }
        fn_ptr_1 = (code *) swi(0x21);
        (*fn_ptr_1)();
        uVar2 = 0x1100;
    }
    bVar10 = true;
    LAB_1000_299d:
    if (bVar10) {
        pass1_1000_29b5(uVar2);
        uVar2 = 0xffff;
    }
    return uVar2;
}

void pass1_1000_39e1()
{
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)
// WARNING: Removing unreachable block (ram,0x10003a40)
// WARNING: Removing unreachable block (ram,0x10003b7e)
// WARNING: Unable to use type for symbol uVar5

u8 *mixed_dos3_call_1000_39f2(u8 *param_1,
                              char *param_2,
                              u8 *param_3)
{
    u8 *pbVar2;
    u8 *puVar3;
    u16 uVar4;
    code *pcVar5;
    u8 *puVar6;
    u16 uVar6;
    u16 uVar7;
    i16 *piVar7;
    i16 *piVar8;
    i16 *piVar9;
    u8 *pbVar10;
    i16 iVar10;
    u8 *puVar11;
    u8 *pbVar12;
    u8 *puVar12;
    u8 *unaff_SI;
    u8 *pbVar13;
    u16 uVar14;
    u16 unaff_CS;
    u8 uVar15;
    u8 bVar16;
    char cVar17;
    char in_AF;
    bool bVar18;
    char cVar19;
    char cVar20;
    u32 uVar21;
    char *pcVar22;
    u8 *puVar23;
    i16 in_stack_0000fff6;
    i16 *piStack8;
    i16 *piStack6;
    u8 *puVar2;
    u16 uVar5;
    u8 *pbVar1;

    puVar6 = (u8 *) u16_1050_5f8a;
    if ((u16_1050_61ec != 0x0)
        && (puVar6 = PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e, param_1 < (u8 *) ( &u16_1050_0002 + 0x1U))) {
        param_1 = (u8 *) u16_1050_5f8a;
    }
    if (puVar6 <= param_1) {
        uVar15 = true;
        puVar6 = (u8 *) 0x900;
        goto LAB_1000_299d;
    }
    puVar12 = param_1;
    puVar23 = (u8 *) u16_1050_5f8a;
    if ((param_1[0x5f90] & 0x20) != 0x0) {
        uVar15 = false;
        pcVar5 = (code *) swi(0x21);
        puVar6 = (u8 *) (*pcVar5)();
        unaff_CS = 0x1000;
        if ((bool) uVar15) {
            goto LAB_1000_299d;
        }
    }
    pbVar12 = (u8 *) param_2;
    if ((puVar12[0x5f90] & 0x80) == 0x0) {
        LAB_1000_3acf:
        uVar15 = false;
        puVar6 = param_3;
        if (param_3 != NULL) {
            uVar15 = puVar12 < puVar23;
            if ((bool) uVar15) {
                uVar15 = 0x0;
                pcVar5 = (code *) swi(0x21);
                uVar21 = (*pcVar5)();
            } else {
                piVar8 = pass1_1000_55b1( 0x1050,
                                         in_stack_0000fff6);
                uVar21 = CONCAT22(pbVar12,
                                  piVar8);
            }
            puVar6 = (u8 *) uVar21;
            if ((bool) uVar15) {
                puVar6 = (u8 *) CONCAT11(0x9,
                                         (char) uVar21);
            } else {
                uVar15 = false;
                if (puVar6 == NULL) {
                    if (((puVar12[0x5f90] & 0x40) == 0x0) || (*(char *) ((u32) uVar21 >> 0x10) != '\x1a')) {
                        uVar15 = true;
                        puVar6 = (u8 *) 0x1c00;
                    } else {
                        uVar15 = false;
                    }
                }
            }
        }
    } else {
        in_stack_0000fff6 = (i16) 0x1050;
        bVar18 = true;
        piStack6 = NULL;
        piStack8 = NULL;
        piVar9 = (i16 *) param_3;
        pbVar13 = pbVar12;
        if (param_3 != NULL) {
            do {
                if (piVar9 == NULL) {
                    break;
                }
                piVar9 = (i16 *) ( piVar9 + -0x1);
                pbVar1 = pbVar13;
                pbVar13 = pbVar13 + 0x1;
                bVar18 = *pbVar1 == '\n';
            } while (!bVar18);
            puVar23 = unaff_SI;
            if (!bVar18) {
                goto LAB_1000_3acf;
            }
            pcVar22 = param_2;
            uVar6 = pass1_1000_3bac();
            pcVar22 =  ((u32) pcVar22 >> 0x10);
            pbVar10 = (u8 *) pcVar22;
            if (uVar6 < 0xa9) {
                exit_1000_25f2(-0x4,
                               0x3ad9,
                               unaff_CS,
                               pcVar22);
                if ( pbVar13 -  pbVar10 == 0x0) {
                    return unaff_SI;
                }
                bVar16 = param_1 < unaff_SI;
                uVar4 =  param_1 -  unaff_SI;
                cVar20 =  uVar4 < 0x0;
                cVar19 = uVar4 == 0x0;
                cVar17 = (POPCOUNT(uVar4 & 0xff) & 0x1U) == 0x0;
                if ((bool) bVar16) {
                    bVar16 = 0x0;
                    cVar20 = '\0';
                    cVar19 = '\x01';
                    cVar17 = '\x01';
                    pcVar5 = (code *) swi(0x21);
                    piVar7 = (i16 *) (*pcVar5)( 0x1050,
                                               piVar9,
                                               puVar12);
                } else {
                    piVar7 = pass1_1000_55b1( pbVar13 -  pbVar10,
                                             (i16) 0x1050);
                }
                if (!(bool) bVar16) {
                    bVar16 = piVar9 < piVar7;
                    uVar4 =  piVar9 -  piVar7;
                    cVar20 =  uVar4 < 0x0;
                    cVar19 = uVar4 == 0x0;
                    cVar17 = (POPCOUNT(uVar4 & 0xff) & 0x1U) == 0x0;
                    piStack6 = piVar7;
                    if ((bool) bVar16 || (bool) cVar19) {
                        return unaff_SI;
                    }
                }
                uVar4 =
                     (u8) (cVar20 << 0x7 | cVar19 << 0x6 | in_AF << 0x4 | cVar17 << 0x2 | 0x2U | bVar16) << 0x8;
                puVar6 = (u8 *) ( piVar7 & 0xff | uVar4);
                if (piStack6 == NULL) {
                    uVar15 = (uVar4 & 0x100) != 0x0;
                    if ((bool) uVar15) {
                        puVar6 = (u8 *) CONCAT11(0x9,
                                                 (char) ( piVar7 & 0xff));
                    } else if (((param_1[0x5f90] & 0x40) == 0x0) || (*param_2 != '\x1a')) {
                        uVar15 = true;
                        puVar6 = (u8 *) 0x1c00;
                    } else {
                        uVar15 = false;
                    }
                    goto LAB_1000_299d;
                }
            } else {
                puVar6 = &stack0xfff0;
                iVar10 = 0x200;
                if (uVar6 < 0x228) {
                    iVar10 = 0x80;
                }
                iVar10 = -iVar10;
                puVar11 = &stack0xfff0 + iVar10;
                puVar12 = &stack0xfff0 + iVar10;
                (&stack0xffee + iVar10) =  0x1050;
                uVar14 = (&stack0xffee + iVar10);
                do {
                    pbVar2 = pbVar12;
                    pbVar12 = pbVar12 + 0x1;
                    bVar16 = *pbVar2;
                    uVar5 = uVar6 & 0xff00;
                    uVar6 = uVar5 | bVar16;
                    if (bVar16 == 0xa) {
                        uVar7 = CONCAT11((char) (uVar5 >> 0x8),
                                         0xd);
                        if (puVar12 == puVar6) {
                            (&stack0xffee + iVar10) = 0x3abd;
                            uVar7 = mixed_dos3_call_1000_3ad9(uVar7,
                                                              (i16) puVar11,
                                                              param_3,
                                                              (&stack0xfff0 + iVar10));
                        }
                        puVar3 = puVar12;
                        puVar12 = puVar12 + 0x1;
                        *puVar3 = (u8) uVar7;
                        uVar6 = CONCAT11((char) (uVar7 >> 0x8),
                                         0xa);
                        piStack8 = (i16 *) ( piStack8 + 0x1);
                    }
                    if (puVar12 == puVar6) {
                        (&stack0xffee + iVar10) = 0x3ac8;
                        uVar6 = mixed_dos3_call_1000_3ad9(uVar6,
                                                          (i16) puVar11,
                                                          param_3,
                                                          (&stack0xfff0 + iVar10));
                    }
                    puVar2 = puVar12;
                    puVar12 = puVar12 + 0x1;
                    *puVar2 = (u8) uVar6;
                    param_3 = param_3 + -0x1;
                } while (param_3 != NULL);
                (&stack0xffee + iVar10) = 0x3ab1;
                mixed_dos3_call_1000_3ad9(uVar6,
                                          (i16) puVar11,
                                          0x0,
                                          (&stack0xfff0 + iVar10));
            }
        }
        uVar15 = piStack6 < piStack8;
        puVar6 = (u8 *) ( piStack6 -  piStack8);
    }
    LAB_1000_299d:
    if ((bool) uVar15) {
        pass1_1000_29b5(puVar6);
        puVar6 = (u8 *) 0xffff;
    }
    return puVar6;
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)

u16 mixed_dos3_call_1000_3ad9(u16 param_1,
                              i16 param_2,
                              i16 *param_3,
                              u16 param_4)
{
    u16 uVar2;
    code *pcVar3;
    u16 uVar4;
    i16 *piVar5;
    u16 uVar5;
    i16 unaff_BP;
    i16 unaff_DI;
    u8 bVar5;
    bool bVar6;
    u8 cVar7;
    char in_AF;
    u8 cVar8;
    u8 cVar9;
    u16 *puVar1;
    i16 *piVar1;
    u16 *puVar2;
    u16 uVar1;

    if (unaff_DI - param_2 == 0x0) {
        return param_4;
    }
    uVar5 = (unaff_BP + 0x6);
    puVar1 = (u16 *) (unaff_BP + -0xc);
    bVar5 = uVar5 < *puVar1;
    uVar1 = uVar5 - *puVar1;
    cVar9 =  uVar1 < 0x0;
    cVar8 = uVar1 == 0x0;
    cVar7 = (POPCOUNT(uVar1 & 0xff) & 0x1U) == 0x0;
    if ((bool) bVar5) {
        bVar5 = 0x0;
        cVar9 = '\0';
        cVar8 = '\x01';
        cVar7 = '\x01';
        pcVar3 = (code *) swi(0x21);
        piVar5 = (i16 *) (*pcVar3)();
    } else {
        piVar5 = pass1_1000_55b1(unaff_DI - param_2,
                                 (i16) 0x1050);
    }
    if (!(bool) bVar5) {
        piVar1 = (i16 *) (unaff_BP + -0x4);
        *piVar1 = *piVar1 +  piVar5;
        bVar5 = param_3 < piVar5;
        uVar2 =  param_3 -  piVar5;
        cVar9 =  uVar2 < 0x0;
        cVar8 = uVar2 == 0x0;
        cVar7 = (POPCOUNT(uVar2 & 0xff) & 0x1U) == 0x0;
        if ((bool) bVar5 || (bool) cVar8) {
            return param_4;
        }
    }
    uVar2 =  (u8) (cVar9 << 0x7 | cVar8 << 0x6 | in_AF << 0x4 | cVar7 << 0x2 | 0x2 | bVar5) << 0x8;
    uVar4 =  piVar5 & 0xff | uVar2;
    if ((unaff_BP + -0x4) == 0x0) {
        bVar6 = (uVar2 & 0x100) != 0x0;
        if (bVar6) {
            uVar4 = CONCAT11(0x9,
                             (char) ( piVar5 & 0xff));
        } else if (((*(u8 *) (uVar5 + 0x5f90) & 0x40) == 0x0) || (**(char **) (unaff_BP + 0x8) != '\x1a')) {
            bVar6 = true;
            uVar4 = 0x1c00;
        } else {
            bVar6 = false;
        }
    } else {
        uVar2 = (unaff_BP + -0x4);
        puVar2 = (u16 *) (unaff_BP + -0x6);
        bVar6 = uVar2 < *puVar2;
        uVar4 = uVar2 - *puVar2;
    }
    if (bVar6) {
        ((unaff_BP + -0xa) + 0x2) = 0x29a2;
        pass1_1000_29b5(uVar4);
        uVar4 = 0xffff;
    }
    return uVar4;
}

i16 pass1_1000_3bac(void)
{
    i16 iVar1;

    if (PTR_LOOP_1050_5f48 < &stack0x0004) {
        iVar1 = -( PTR_LOOP_1050_5f48 -  &stack0x0004);
    } else {
        iVar1 = 0x0;
    }
    return iVar1;
}

void pass1_1000_3bc0(i16 param_1,
                     i16 param_2)
{
    i16 *piVar1;
    u16 uVar2;
    u16 uVar3;
    u16 uVar4;
    u16 uVar5;
    i16 iVar6;
    u16 *unaff_SI;
    u16 *puVar7;
    u16 unaff_DI;
    bool bVar8;
    u32 uVar9;

    if ((*(u8 *) (param_2 + 0x2) & 0x1) != 0x0) {
        pass1_1000_3cb7(param_2);
        uVar5 = *unaff_SI;
        if ((uVar5 & 0x1) != 0x0) {
            param_1 = (param_1 - uVar5) + -0x1;
        }
        uVar5 = (param_2 + 0x4);
        if (uVar5 != 0x0) {
            uVar4 = param_1 + 0x2U + uVar5;
            if (!CARRY2(param_1 + 0x2U,
                        uVar5)) {
                uVar3 = pass1_1000_29dc( 0x1050);
                uVar5 = &PTR_LOOP_1050_6066;
                if (uVar5 == 0x1000) {
                    goto LAB_1000_3c12;
                }
                uVar2 = 0x8000;
                while (uVar5 <= uVar2) {
                    uVar2 >>= 0x1;
                    if (uVar2 == 0x0) {
                        goto LAB_1000_3c2b;
                    }
                }
                if (uVar2 < 0x8) {
                    goto LAB_1000_3c2b;
                }
                uVar5 = uVar2 << 0x1;
                goto LAB_1000_3c12;
            }
            uVar2 = 0x0;
            uVar5 = 0xfff0;
            if (uVar4 == 0x0) {
                while (true) {
                    bVar8 = false;
                    uVar9 = mixed_mem_op_1000_3c51(uVar2,
                                                   uVar4,
                                                   param_2,
                                                   0x3c23,
                                                   unaff_DI);
                    if (!bVar8) {
                        break;
                    }
                    if (uVar5 == 0xfff0) {
                        return;
                    }
                    LAB_1000_3c2b:
                    uVar5 = 0x10;
                    LAB_1000_3c12:
                    uVar5 -= 0x1;
                    uVar2 = uVar5 + uVar4;
                    if (CARRY2(uVar5,
                               uVar4)) {
                        uVar2 = 0x0;
                    }
                    uVar5 = ~uVar5;
                    uVar2 &= uVar5;
                }
                iVar6 =  uVar9 - (param_2 + 0x4);
                (param_2 + 0x4) =  uVar9;
                (u16 *) (param_2 + 0xa) = unaff_SI;
                piVar1 = *(i16 **) (param_2 + 0xc);
                *piVar1 = iVar6 + -0x1;
                puVar7 = (u16 *) ( piVar1 + iVar6);
                *puVar7 = 0xfffe;
                (u16 *) (param_2 + 0xc) = puVar7;
            }
        }
    }
    return;
}

/*
Unable to decompile 'mixed_mem_op_1000_3c51'
Cause:
Low-level Error: Overlapping input varnodes
*/


void pass1_1000_3cb7(i16 param_1)
{
    u16 uVar1;
    u16 *puVar2;

    puVar2 = (u16 *) (param_1 + 0xa);
    if (puVar2 == (u16 *) (param_1 + 0xc)) {
        puVar2 = (u16 *) (param_1 + 0x8);
    }
    while (true) {
        uVar1 = *puVar2;
        if (uVar1 == 0xfffe) {
            break;
        }
        puVar2 = (u16 *) ( puVar2 + (uVar1 & 0xfffe) + 0x2);
    }
    return;
}

void pass1_1000_3cd8(u16 param_1,
                     u16 param_2)
{
    free_mem_1000_407a(param_1,
                       param_2);
    return;
}

u16 *pass1_1000_3cea(u32 param_1,
                     char *param_2)
{
    u16 *pUVar1;
    char *pcVar2;
    u16 *pUVar3;
    i16 iVar4;
    u16 uVar5;
    u16 uVar6;
    u16 *pUVar7;
    char *pcVar8;
    u16 *pUVar9;
    u16 *pUVar10;
    u16 uVar11;
    u16 uVar12;
    bool bVar13;

    uVar11 =  (param_1 >> 0x10);
    bVar13 = true;
    iVar4 = -0x1;
    pUVar7 = (u16 *) param_1;
    do {
        if (iVar4 == 0x0) {
            break;
        }
        iVar4 += -0x1;
        pUVar1 = pUVar7;
        pUVar7 = (u16 *) ( pUVar7 + 0x1);
        bVar13 = *(char *) pUVar1 == '\0';
    } while (!bVar13);
    pUVar10 = (u16 *) ( pUVar7 + -0x1);
    uVar12 =  ((u32) param_2 >> 0x10);
    pcVar8 = (char *) param_2;
    uVar5 = 0xffff;
    do {
        if (uVar5 == 0x0) {
            break;
        }
        uVar5 -= 0x1;
        pcVar2 = pcVar8;
        pcVar8 = pcVar8 + 0x1;
        bVar13 = *pcVar2 == '\0';
    } while (!bVar13);
    uVar5 = ~uVar5;
    if (!bVar13) {
        pcVar8 = pcVar8 + -uVar5;
        uVar5 += 0x1;
    }
    pUVar9 = (u16 *) (pcVar8 + -uVar5);
    if (uVar5 == 0x0) {
        pUVar1 = pUVar9;
        pUVar9 = pUVar9 + 0x1;
        *pUVar10 = *pUVar1;
        uVar5 = 0xfffe;
        pUVar10 = (u16 *) ( pUVar7 + 0x1);
    } else if (( pUVar9 & 0x1) != 0x0) {
        pUVar1 = pUVar9;
        pUVar9 = (u16 *) ( pUVar9 + 0x1);
        *(u8 *) pUVar10 = *(u8 *) pUVar1;
        uVar5 -= 0x1;
        pUVar10 = pUVar7;
    }
    for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
        pUVar3 = pUVar10;
        pUVar10 = pUVar10 + 0x1;
        pUVar1 = pUVar9;
        pUVar9 = pUVar9 + 0x1;
        *pUVar3 = *pUVar1;
    }
    for (uVar5 =  ((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
        pUVar3 = pUVar10;
        pUVar10 = (u16 *) ( pUVar10 + 0x1);
        pUVar1 = pUVar9;
        pUVar9 = (u16 *) ( pUVar9 + 0x1);
        *(u8 *) pUVar3 = *(u8 *) pUVar1;
    }
    return (u16 *) param_1;
}


i16 pass1_1000_3d7a(char *param_1,
                    char *param_2)
{
    u8 *pbVar2;
    u8 *pbVar3;
    i16 iVar4;
    u16 uVar5;
    char *string_4;
    char *string_1;
    char *string_2;
    char *string_6;
    u16 uVar6;
    bool bool_1;
    bool bool_2;
    char *pbVar4;
    char *pbVar1;
    char *string_3;

    string_1 = (char *) param_1;
    uVar6 =  ((u32) param_2 >> 0x10);
    string_2 = (char *) param_2;
    iVar4 = 0x0;
    uVar5 = 0xffff;
    do {
        if (uVar5 == 0x0) {
            break;
        }
        uVar5 -= 0x1;
        string_3 = string_2;
        string_2 = string_2 + 0x1;
    } while (*string_3 != '\0');
    string_4 = (char *) ~uVar5;
    bool_1 = string_2 < string_4;
    string_6 = string_2 + - string_4;
    bool_2 = string_6 == NULL;
    do {
        if (string_4 == NULL) {
            break;
        }
        string_4 = string_4 + -0x1;
        pbVar3 = (u8 *) string_6;
        string_6 = (char *) ((u8 *) string_6 + 0x1);
        pbVar2 = (u8 *) string_1;
        string_1 = (char *) ((u8 *) string_1 + 0x1);
        bool_1 = *pbVar2 < *pbVar3;
        bool_2 = *pbVar2 == *pbVar3;
    } while (bool_2);
    if (!bool_2) {
        iVar4 = (0x1 -  bool_1) -  (bool_1 != 0x0);
    }
    return iVar4;
}



u16 pass1_1000_3de8(char *param_1,
                    char *param_2,
                    u16 param_3,
                    u16 param_4,
                    u16 param_5)
{
    u8 *pbVar1;
    char *pcVar2;
    char *pcVar3;
    u8 bVar4;
    u16 uVar5;
    i16 iVar6;
    char *pcVar7;
    char *pcVar8;
    u16 uVar9;
    u16 uVar10;
    bool bVar11;

    if (param_3 != 0x0) {
        uVar9 =  ((u32) param_1 >> 0x10);
        pcVar8 = (char *) param_1;
        uVar5 = param_3;
        pcVar7 = pcVar8;
        do {
            if (uVar5 == 0x0) {
                break;
            }
            uVar5 -= 0x1;
            pcVar2 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
        } while (*pcVar2 != '\0');
        iVar6 = param_3 - uVar5;
        uVar10 =  ((u32) param_2 >> 0x10);
        pcVar7 = (char *) param_2;
        do {
            if (iVar6 == 0x0) {
                break;
            }
            iVar6 += -0x1;
            pcVar3 = pcVar8;
            pcVar8 = pcVar8 + 0x1;
            pcVar2 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
        } while (*pcVar2 == *pcVar3);
        bVar4 = pcVar7[-0x1];
        uVar5 = 0x0;
        pbVar1 = (u8 *) (pcVar8 + -0x1);
        bVar11 = bVar4 == *pbVar1;
        if (bVar4 < *pbVar1 || bVar11) {
            if (bVar11) {
                return 0x0;
            }
            uVar5 = 0xfffe;
        }
        param_3 = ~uVar5;
    }
    return param_3;
}


i16 pass1_1000_3e2c(u32 param_1)
{
    u8 *pbVar1;
    u8 bVar2;
    u8 bVar3;
    i16 iVar4;
    u8 *pbVar5;
    u16 uVar6;

    uVar6 =  (param_1 >> 0x10);
    pbVar5 = (u8 *) param_1;
    iVar4 = 0x0;
    do {
        do {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 0x1;
            bVar2 = *pbVar1;
        } while (bVar2 == 0x20);
    } while (bVar2 == 0x9);
    if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
        goto LAB_1000_3e4d;
    }
    while (true) {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 0x1;
        bVar3 = *pbVar1;
        LAB_1000_3e4d:
        if ((0x39 < bVar3) || (bVar3 < 0x30)) {
            break;
        }
        iVar4 = iVar4 * 0xa +  (u8) (bVar3 - 0x30);
    }
    if (bVar2 == 0x2d) {
        iVar4 = -iVar4;
    }
    return iVar4;
}

i16 pass1_1000_3e2c(u32 param_1)
{
    u8 *pbVar1;
    u8 bVar2;
    u8 bVar3;
    i16 iVar4;
    u8 *pbVar5;
    u16 uVar6;

    uVar6 =  (param_1 >> 0x10);
    pbVar5 = (u8 *) param_1;
    iVar4 = 0x0;
    do {
        do {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 0x1;
            bVar2 = *pbVar1;
        } while (bVar2 == 0x20);
    } while (bVar2 == 0x9);
    if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
        goto LAB_1000_3e4d;
    }
    while (true) {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 0x1;
        bVar3 = *pbVar1;
        LAB_1000_3e4d:
        if ((0x39 < bVar3) || (bVar3 < 0x30)) {
            break;
        }
        iVar4 = iVar4 * 0xa +  (u8) (bVar3 - 0x30);
    }
    if (bVar2 == 0x2d) {
        iVar4 = -iVar4;
    }
    return iVar4;
}

u8 *pass1_1000_3e82(u16 param_1,
                    u8 *param_2,
                    u16 param_3)
{
    u8 *pbVar1;
    u32 uVar2;
    u8 bVar3;
    u16 uVar5;
    u16 uVar6;
    u16 uVar7;
    u8 *pbVar8;
    u8 *pbVar9;
    u8 *pbVar10;
    u8 *pbVar11;
    u16 uVar12;
    bool bVar13;
    char cVar4;

    uVar6 = 0x0;
    if (param_3 == 0xa) {
        uVar6 =  param_1 >> 0xf;
    }
    uVar12 =  ((u32) param_2 >> 0x10);
    pbVar9 = (u8 *) param_2;
    pbVar10 = pbVar9;
    pbVar8 = pbVar9;
    if ((param_3 == 0xa) && ( uVar6 < 0x0)) {
        pbVar10 = pbVar9 + 0x1;
        *param_2 = '-';
        bVar13 = param_1 != 0x0;
        param_1 = -param_1;
        uVar6 = -(uVar6 + bVar13);
        pbVar8 = pbVar10;
    }
    do {
        uVar7 = 0x0;
        uVar5 = uVar6;
        if (uVar6 != 0x0) {
            uVar5 = uVar6 / param_3;
            uVar7 = uVar6 % param_3;
        }
        uVar2 = CONCAT22(uVar7,
                         param_1);
        param_1 =  (uVar2 / param_3);
        cVar4 = (char) (uVar2 % (u32) param_3);
        bVar3 = cVar4 + 0x30;
        if (0x39 < bVar3) {
            bVar3 = cVar4 + 0x57;
        }
        pbVar11 = pbVar10 + 0x1;
        *pbVar10 = bVar3;
        uVar6 = uVar5;
        pbVar10 = pbVar11;
    } while ((uVar5 | param_1) != 0x0);
    *pbVar11 = 0x0;
    do {
        pbVar11 = pbVar11 + -0x1;
        pbVar1 = pbVar11;
        bVar3 = *pbVar1;
        *pbVar1 = *pbVar8;
        *pbVar8 = bVar3;
        pbVar10 = pbVar8 + 0x2;
        pbVar8 = pbVar8 + 0x1;
    } while (pbVar10 < pbVar11);
    return pbVar9;
}




i16 pass1_1000_3f5c(void)
{
    u16 uVar1;
    u16 *puVar2;
    i16 iVar3;

    iVar3 = 0x0;
    if (u16_1050_61ec == 0x0) {
        puVar2 = (u16 *) &PTR_LOOP_1050_6210;
    } else {
        puVar2 = (u16 *) 0x6234;
    }
    for (; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0x6) {
        uVar1 = pass1_1000_2a00(puVar2);
        if (uVar1 != 0xffff) {
            iVar3 += 0x1;
        }
    }
    return iVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 sys_1000_3f9c(char *param_1,
                  char *param_2,
                  u16 param_3)
{
    char *pcVar1;
    u16 uVar2;
    u16 unaff_CS;
    u16 local_4;

    PTR_LOOP_1050_68b2._0_1_ = 0x42;
    _u16_1050_68a8 = param_1;
    PTR_LOOP_1050_68ac = (u8 *) 0x7fff;
    PTR_LOOP_1050_68ae = param_1;
    uVar2 = FUN_1000_30b4();
    pcVar1 = _u16_1050_68a8;
    PTR_LOOP_1050_68ac = PTR_LOOP_1050_68ac + -0x1;
    if ( PTR_LOOP_1050_68ac < 0x0) {
        mem_1000_2bb6(param_1,
                      0x0,
                      (i16 *) &u16_1050_68a8);
    } else {
        _u16_1050_68a8 = (char *) ((u32) _u16_1050_68a8 & 0xffff0000 | (u32) (u16_1050_68a8 + 0x1));
        *pcVar1 = '\0';
    }
    PTR_LOOP_1050_68b0 = (u8 *) ((u32) PTR_LOOP_1050_68ae >> 0x10);
    return uVar2;
}

u8 *pass1_1000_400a(i16 param_1)
{
    u8 *puVar1;

    if ((param_1 < 0x0) || ( PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1)) {
        PTR_LOOP_1050_5f78 = (u8 *) ( &u16_1050_0008 + 0x1);
        puVar1 = (u8 *) 0xffff;
    } else if (((u16_1050_61ec == 0x0) || ((param_1 <  u16_1050_5f8a && (0x2 < param_1)))) && (0x31d
        < CONCAT11(DAT_1050_5f83,
                   DAT_1050_5f82))) {
        puVar1 = PTR_LOOP_1050_5f88;
        if (((*(u8 *) (param_1 + 0x5f90) & 0x1) == 0x0) || (puVar1 = (u8 *) dos3_call_1000_5174(), puVar1 != NULL)) {
            PTR_LOOP_1050_5f88 = puVar1;
            PTR_LOOP_1050_5f78 = (u8 *) ( &u16_1050_0008 + 0x1);
            puVar1 = (u8 *) 0xffff;
        }
    } else {
        puVar1 = NULL;
    }
    return puVar1;
}



// WARNING: Removing unreachable block (ram,0x10004090)
// WARNING: Removing unreachable block (ram,0x1000409a)

void free_mem_1000_407a(u16 param_1,
                        u16 param_2)
{
    GlobalFree16((HGLOBAL16) 0x1050);
    return;
}

i16 *mixed_sys_op_1000_40af(u16 param_1,
                            i16 param_2,
                            u16 param_3)
{
    struct astruct_824 **ppaVar1;
    char *pcVar2;
    u16 *puVar4;
    char *pcVar5;
    i16 iVar6;
    struct astruct_824 **ppaVar7;
    u16 uVar7;
    u16 uVar8;
    i16 iVar8;
    HGLOBAL16 hglobal_7;
    void *SVar8;
    struct astruct_824 **ppaVar8;
    i16 unaff_SI;
    i16 *piVar9;
    char *pcVar10;
    struct astruct_824 *hglobal_di;
    u16 *puVar11;
    u8 unaff_CS;
    u16 unaff_SS;
    bool bVar12;
    void *pvVar13;
    struct astruct_825 *paVar14;
    u16 *puVar3;
    u8 uVar13;
    u8 uVar14;
    i16 iVar12;
    struct astruct_824 *temp_5fa27366cb;

    do {
        uVar7 =  ((u32) param_1 * (u32) param_3);
        uVar8 = param_2 * param_3 +  ((u32) param_1 * (u32) param_3 >> 0x10);
        if ((uVar8 | uVar7) != 0x0) {
            piVar9 = NULL;
            if ((uVar8 < 0x3) && ((uVar8 < 0x2 || (uVar7 == 0x0)))) {
                if (uVar8 == 0x0) {
                    uVar7 = uVar7 + 0xfff & 0xf000;
                    if (uVar7 == 0x0) {
                        uVar8 = 0x1;
                    }
                } else if ((param_3 - 0x1 & param_3) != 0x0) {
                    piVar9 = (i16 *) (((u32) uVar8 << 0x10) % (u32) param_3);
                    bVar12 = CARRY2(uVar7,
                                     piVar9);
                    uVar7 +=  piVar9;
                    if (bVar12) {
                        goto LAB_1000_41aa;
                    }
                    uVar8 = 0x1;
                }
            } else if ((param_3 - 0x1 & param_3) != 0x0) {
                goto LAB_1000_41aa;
            }
            hglobal_7 = GLobalAlloc16(CONCAT13((char) (uVar8 >> 0x8),
                                               CONCAT12((char) uVar8,
                                                        uVar7)),
                                      0x20);
            if ((hglobal_7 != 0x0) && ((uVar7 & 0x1) != 0x0)) {
                pvVar13 = WIN16_GlobalLock16(hglobal_7);
                SVar8._0_2_ =  pvVar13;
                if (( SVar8 != 0x0) || (pvVar13 == NULL)) {
                    paVar14 = (astruct_825 *) CONCAT22(hglobal_7,
                                                       0x12);
                    uVar13 = '\x12';
                    uVar14 = '\0';
                    pass1_1000_25a8();
                    pass1_1000_2913(CONCAT11(uVar14,
                                             uVar13));
                    pcVar5 = poss_str_op_1000_28dc(paVar14);
                    if (pcVar5 == NULL) {
                        goto LAB_1000_28cb;
                    }
                    iVar6 = 0x9;
                    if (*pcVar5 == 'M') {
                        iVar6 = 0xf;
                    }
                    pcVar5 = pcVar5 + iVar6;
                    iVar6 = 0x22;
                    pcVar10 = pcVar5;
                    break;
                }
                hglobal_7 = pass1_1000_422a( ((u32) pvVar13 >> 0x10),
                                            hglobal_7);
                if (hglobal_7 == 0x0) {
                    GlobalUnlock16(uVar8);
                    GlobalFree16((HGLOBAL16) hglobal_di);
                    hglobal_7 = 0x0;
                }
            }
            unaff_CS = 0x38;
            if (hglobal_7 != 0x0) {
                puVar11 = NULL;
                for (; unaff_SI != 0x0; unaff_SI += -0x1) {
                    for (iVar6 = -0x8000; iVar6 != 0x0; iVar6 += -0x1) {
                        puVar3 = puVar11;
                        puVar11 = puVar11 + 0x1;
                        *puVar3 = 0x0;
                    }
                    hglobal_7 += 0x100;
                }
                if (hglobal_di != NULL) {
                    for (; hglobal_di != NULL; hglobal_di = hglobal_di + -0x1) {
                        puVar4 = puVar11;
                        puVar11 = (u16 *) ( puVar11 + 0x1);
                        *(u8 *) puVar4 = 0x0;
                    }
                }
                return piVar9;
            }
        }
        LAB_1000_41aa:
        if ((u16_1050_618e |  PTR_LOOP_1050_618c) == 0x0) {
            return NULL;
        }
        iVar8 = ((code) PTR_LOOP_1050_618c)(unaff_CS,
                                            param_3,
                                            param_1,
                                            param_2);
        if (iVar8 == 0x0) {
            return NULL;
        }
    } while (true);
    while (true) {
        iVar6 += -0x1;
        pcVar2 = pcVar10;
        pcVar10 = pcVar10 + 0x1;
        if (*pcVar2 == '\r') {
            break;
        }
        if (iVar6 == 0x0) {
            break;
        }
    }
    pcVar10[-0x1] = '\0';
    LAB_1000_28cb:
    FatalAppExit16((char *) CONCAT13(0x10,
                                     CONCAT12(0x50,
                                              pcVar5)),
                   0x0);
    FatalExit();
    ppaVar8 = (astruct_824 **) &PTR_LOOP_1050_63fe;
    do {
        ppaVar1 = ppaVar8;
        ppaVar8 = ppaVar8 + 0x1;
        temp_5fa27366cb = *ppaVar1;
        ppaVar7 = ppaVar8;
        if ((temp_5fa27366cb == hglobal_di) || (ppaVar7 = (astruct_824 **) (temp_5fa27366cb + 0x1), ppaVar7 == NULL)) {
            return (i16 *) ppaVar7;
        }
        iVar6 = -0x1;
        do {
            if (iVar6 == 0x0) {
                break;
            }
            iVar6 += -0x1;
            ppaVar1 = ppaVar8;
            ppaVar8 = (astruct_824 **) ( ppaVar8 + 0x1);
        } while (ppaVar1 != NULL);
    } while (true);
}



// WARNING: Could not reconcile some variable overlaps

u16 pass1_1000_41e0(i16 param_1)
{
    i16 *piStack6;

    piStack6 = (i16 *) CONCAT22(PTR_LOOP_1050_6192,
                                PTR_LOOP_1050_6190);
    while (true) {
        if (PTR_LOOP_1050_6190 + ( PTR_LOOP_1050_6194 & 0xfffc) <= (u8 *) piStack6) {
            return 0x0;
        }
        if (*piStack6 == param_1) {
            break;
        }
        piStack6 = (i16 *) ((u32) piStack6 & 0xffff0000 | ZEXT24((u8 *) piStack6 + 0x4));
    }
    *piStack6 = 0x0;
    return ((u8 *) piStack6 + 0x2);
}



// WARNING: Could not reconcile some variable overlaps

i16 pass1_1000_422a(i16 param_1,
                    u16 param_2)
{
    u8 *puVar1;
    u8 *puVar2;
    u8 *puVar3;
    u8 *puVar4;
    i16 *piStack6;

    piStack6 = (i16 *) CONCAT22(PTR_LOOP_1050_6192,
                                PTR_LOOP_1050_6190);
    while (true) {
        if (PTR_LOOP_1050_6190 + ( PTR_LOOP_1050_6194 & 0xfffc) <= (u8 *) piStack6) {
            puVar2 = PTR_LOOP_1050_6194 + 0x28;
            puVar4 = PTR_LOOP_1050_6192;
            puVar3 = (u8 *) pass1_1000_16aa( PTR_LOOP_1050_6192,
                                            (u16 *) PTR_LOOP_1050_6190,
                                             PTR_LOOP_1050_6192,
                                             puVar2);
            if (( puVar4 |  puVar3) == 0x0) {
                param_1 = 0x0;
            } else {
                puVar1 = puVar3 + ( PTR_LOOP_1050_6194 & 0xfffc);
                piStack6 = (i16 *) CONCAT22(puVar4,
                                            puVar1);
                PTR_LOOP_1050_6190 = puVar3;
                PTR_LOOP_1050_6192 = puVar4;
                *piStack6 = param_1;
                (puVar1 + 0x2) = param_2;
                PTR_LOOP_1050_6194 = puVar2;
                pass1_1000_4906((StructD *) CONCAT22(puVar4,
                                                     puVar1 + 0x4),
                                NULL,
                                0x24);
            }
            return param_1;
        }
        if (*piStack6 == 0x0) {
            break;
        }
        piStack6 = (i16 *) ((u32) piStack6 & 0xffff0000 | ZEXT24((u8 *) piStack6 + 0x4));
    }
    ((u8 *) piStack6 + 0x2) = param_2;
    *piStack6 = param_1;
    return param_1;
}



// WARNING: Removing unreachable block (ram,0x10004311)

void dos3_call_set_struct_1000_42de(astruct_811 *param_1,
                                    astruct_810 *param_2,
                                    u16 *param_3)
{
    u16 uVar3;
    code *pcVar4;
    u16 uVar4;
    u16 uVar5;
    struct astruct_811 *iVar4;
    struct astruct_810 *iVar5;
    u16 uVar6;
    u16 uVar7;
    u16 uVar8;
    bool bVar5;
    u32 uVar12;
    u16 uVar1;
    u16 uVar2;
    u16 uVar9;

    uVar6 =  ((u32) param_1 >> 0x10);
    iVar4 = (astruct_811 *) param_1;
    uVar5 = iVar4->field2_0x2;
    uVar4 = iVar4->field3_0x4;
    uVar1 = iVar4->field6_0x8;
    uVar2 = iVar4->field7_0xa;
    uVar7 =  ((u32) param_3 >> 0x10);
    uVar3 = *param_3;
    uVar9 = ( param_3 + 0x6);
    bVar5 = false;
    pcVar4 = (code *) swi(0x21);
    uVar12 = (*pcVar4)();
    *param_3 = uVar3;
    ( param_3 + 0x6) = uVar9;
    uVar8 =  ((u32) param_2 >> 0x10);
    iVar5 = (astruct_810 *) param_2;
    param_2 =  uVar12;
    iVar5->field2_0x2 = uVar5;
    iVar5->field3_0x4 = uVar4;
    iVar5->field4_0x6 =  (uVar12 >> 0x10);
    iVar5->field5_0x8 = uVar1;
    iVar5->field6_0xa = uVar2;
    if (bVar5) {
        pass1_1000_29af( uVar12);
    }
    iVar5->field7_0xc =  bVar5;
    return;
}



// WARNING: Removing unreachable block (ram,0x1000438a)
// WARNING: Removing unreachable block (ram,0x10004372)
// WARNING: Removing unreachable block (ram,0x100043aa)






// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address



char *pass1_1000_472c(u32 param_1,
                      char param_2)
{
    char *pcVar1;
    u16 uVar2;
    char *pcVar3;
    char *pcVar4;
    u16 uVar5;
    bool bVar6;

    uVar5 =  (param_1 >> 0x10);
    pcVar3 = (char *) param_1;
    bVar6 = true;
    uVar2 = 0xffff;
    pcVar4 = pcVar3;
    do {
        if (uVar2 == 0x0) {
            break;
        }
        uVar2 -= 0x1;
        pcVar1 = pcVar4;
        pcVar4 = pcVar4 + 0x1;
        bVar6 = *pcVar1 == '\0';
    } while (!bVar6);
    uVar2 = ~uVar2;
    do {
        if (uVar2 == 0x0) {
            break;
        }
        uVar2 -= 0x1;
        pcVar1 = pcVar3;
        pcVar3 = pcVar3 + 0x1;
        bVar6 = param_2 == *pcVar1;
    } while (!bVar6);
    if (!bVar6) {
        if (param_2 != '\0') {
            return NULL;
        }
        pcVar3 = pcVar3 + 0x1;
    }
    return pcVar3 + -0x1;
}

i16 pass1_1000_475e(u32 param_1,
                    u32 param_2)
{
    char *pcVar1;
    char cVar2;
    char cVar3;
    u8 bVar4;
    struct astruct_235 *bVar3;
    i16 bVar5;
    char *pcVar5;
    char *pcVar6;

    pcVar6 = (char *) param_2;
    pcVar5 = (char *) param_1;
    bVar5 = 0xff;
    do {
        do {
            cVar3 = (char) bVar5;
            if (cVar3 == '\0') {
                goto LAB_1000_479d;
            }
            pcVar1 = pcVar6;
            pcVar6 = pcVar6 + 0x1;
            cVar3 = *pcVar1;
            cVar2 = *pcVar5;
            bVar5 = CONCAT11(cVar2,
                             cVar3);
            pcVar5 = pcVar5 + 0x1;
        } while (cVar2 == cVar3);
        bVar4 = cVar3 + 0xbfU + (-((u8) (cVar3 + 0xbfU) < 0x1a) & 0x20U) + 0x41;
        bVar3._0_1_ = cVar2 + 0xbf;
        bVar5._0_1_ = (u8) bVar3 + (-((u8) bVar3 < 0x1a) & 0x20U) + 0x41;
        bVar5 = CONCAT11(bVar4,
                         (u8) bVar5);
    } while ((u8) bVar5 == bVar4);
    cVar3 = ((u8) bVar5 < bVar4) * -0x2 + '\x01';
    LAB_1000_479d:
    return  cVar3;
}

u16 pass1_1000_47a4(u32 param_1,
                    u32 param_2)
{
    u8 *pbVar1;
    u8 bVar2;
    u16 *puVar3;
    u8 *pbVar4;
    i16 iVar5;
    u8 *pbVar6;
    u16 *puVar7;
    u16 uVar8;
    u16 local_22[0x10];

    puVar7 = local_22;
    for (iVar5 = 0x10; iVar5 != 0x0; iVar5 += -0x1) {
        puVar3 = puVar7;
        puVar7 = puVar7 + 0x1;
        *puVar3 = 0x0;
    }
    pbVar6 = (u8 *) param_2;
    while (true) {
        pbVar1 = pbVar6;
        pbVar6 = pbVar6 + 0x1;
        bVar2 = *pbVar1;
        if (bVar2 == 0x0) {
            break;
        }
        pbVar1 = (u8 *) ( local_22 +  (bVar2 >> 0x3));
        *pbVar1 = *pbVar1 | '\x01' << (bVar2 & 0x7);
    }
    pbVar1 = (u8 *) param_1;
    if (param_1 == 0x0) {
        pbVar1 = pbRam105061e4;
    }
    do {
        pbRam105061e4 = pbVar1;
        uVar8 =  ((u32) pbRam105061e4 >> 0x10);
        pbVar6 = (u8 *) ( pbRam105061e4 + 0x1);
        bVar2 = *pbRam105061e4;
        if (bVar2 == 0x0) {
            return 0x0;
        }
        pbVar1 = (u8 *) ((u32) pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar6));
    } while (('\x01' << (bVar2 & 0x7) & *(u8 *) ( local_22 +  (bVar2 >> 0x3))) != 0x0);
    do {
        pbVar4 = pbVar6;
        bVar2 = *pbVar4;
        if (bVar2 == 0x0) {
            goto LAB_1000_483c;
        }
        pbVar6 = pbVar4 + 0x1;
    } while (('\x01' << (bVar2 & 0x7) & *(u8 *) ( local_22 +  (bVar2 >> 0x3))) == 0x0);
    *pbVar4 = 0x0;
    pbVar4 = pbVar4 + 0x1;
    LAB_1000_483c:
    pbRam105061e4 = (u8 *) ((u32) pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar4));
    return  pbRam105061e4;
}

u16 pass1_1000_484c(u32 param_1,
                    u32 param_2,
                    u16 param_3)
{
    u8 *pbVar1;
    u8 *pbVar2;
    i16 iVar3;
    u16 uVar4;
    u16 uVar5;
    u8 *pbVar6;
    u8 *pbVar7;
    i16 iVar8;
    bool bVar9;
    bool bVar10;

    if (param_3 == 0x0) {
        return 0x0;
    }
    do {
        iVar8 =  (param_2 >> 0x10);
        pbVar7 = (u8 *) param_2;
        iVar3 =  (param_1 >> 0x10);
        pbVar6 = (u8 *) param_1;
        uVar4 = ~ pbVar7;
        uVar4 = ((param_3 - 0x1) - uVar4 & - (param_3 - 0x1 < uVar4)) + uVar4;
        uVar5 = ~ pbVar6;
        uVar4 = (uVar4 - uVar5 & - (uVar4 < uVar5)) + uVar5 + 0x1;
        bVar9 = param_3 < uVar4;
        param_3 -= uVar4;
        bVar10 = param_3 == 0x0;
        do {
            if (uVar4 == 0x0) {
                break;
            }
            uVar4 -= 0x1;
            pbVar2 = pbVar7;
            pbVar7 = pbVar7 + 0x1;
            pbVar1 = pbVar6;
            pbVar6 = pbVar6 + 0x1;
            bVar9 = *pbVar1 < *pbVar2;
            bVar10 = *pbVar1 == *pbVar2;
        } while (bVar10);
        param_2 = param_2 & 0xffff0000 | ZEXT24(pbVar7);
        if (!bVar10) {
            return (0x1 -  bVar9) -  (bVar9 != 0x0);
        }
        if (param_3 == 0x0) {
            return uVar4;
        }
        if (pbVar6 == NULL) {
            iVar3 += 0x6c;
        }
        param_1 = CONCAT22(iVar3,
                           pbVar6);
        if (pbVar7 == NULL) {
            param_2 = (u32) (iVar8 + 0x6c) << 0x10;
            param_1 = CONCAT22(iVar3,
                               pbVar6);
        }
    } while (true);
}

u16 pass1_1000_48a8(u32 param_1,
                    u32 param_2,
                    i16 param_3)
{
    u16 *puVar1;
    u16 *puVar2;
    i16 iVar3;
    u16 uVar4;
    u16 uVar5;
    u16 *puVar6;
    u16 *puVar7;
    i16 iVar8;

    if (param_3 != 0x0) {
        while (true) {
            iVar3 =  (param_2 >> 0x10);
            puVar6 = (u16 *) param_2;
            iVar8 =  (param_1 >> 0x10);
            puVar7 = (u16 *) param_1;
            uVar4 = ~ puVar7;
            uVar4 = ((param_3 - 0x1U) - uVar4 & - (param_3 - 0x1U < uVar4)) + uVar4;
            uVar5 = ~ puVar6;
            uVar4 = (uVar4 - uVar5 & - (uVar4 < uVar5)) + uVar5 + 0x1;
            param_3 -= uVar4;
            for (uVar5 = uVar4 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
                puVar2 = puVar7;
                puVar7 = puVar7 + 0x1;
                puVar1 = puVar6;
                puVar6 = puVar6 + 0x1;
                *puVar2 = *puVar1;
            }
            for (uVar4 =  ((uVar4 & 0x1) != 0x0); uVar4 != 0x0; uVar4 -= 0x1) {
                puVar2 = puVar7;
                puVar7 = (u16 *) ( puVar7 + 0x1);
                puVar1 = puVar6;
                puVar6 = (u16 *) ( puVar6 + 0x1);
                *(u8 *) puVar2 = *(u8 *) puVar1;
            }
            if (param_3 == 0x0) {
                break;
            }
            if (puVar6 == NULL) {
                iVar3 += 0x6c;
            }
            param_1 = param_1 & 0xffff0000 | ZEXT24(puVar7);
            param_2 = CONCAT22(iVar3,
                               puVar6);
            if (puVar7 == NULL) {
                param_1 = (u32) (iVar8 + 0x6c) << 0x10;
                param_2 = CONCAT22(iVar3,
                                   puVar6);
            }
        }
    }
    return  param_1;
}

u16 *pass1_1000_4906(StructD *param_1,
                     WNDCLASS16 *in_wnd_class,
                     u16 param_3)
{
    u16 *puVar1;
    u8 uVar2;
    u16 uVar3;
    u16 uVar4;
    struct astruct_20 *struct_1;
    u16 uVar5;
    u16 *puVar6;
    struct astruct_20 *struct_1_hi;

    if (param_3 != 0x0) {
        struct_1_hi = (astruct_20 *) ((u32) param_1 >> 0x10);
        struct_1 = (astruct_20 *) - (u16 *) param_1;
        uVar5 = param_3;
        if (struct_1 != NULL) {
            uVar5 = ( struct_1 - param_3 & - (struct_1 < param_3)) + param_3;
            struct_1 = (astruct_20 *) (param_3 - uVar5);
        }
        uVar3 =  in_wnd_class & 0xff |  in_wnd_class << 0x8;
        puVar6 = (u16 *) param_1;
        for (uVar4 = uVar5 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
            puVar1 = puVar6;
            puVar6 = puVar6 + 0x1;
            *puVar1 = uVar3;
        }
        for (uVar5 =  ((uVar5 & 0x1) != 0x0);
             uVar2 = (u8) ( in_wnd_class & 0xff), uVar5 != 0x0;
             uVar5 -= 0x1) {
            puVar1 = puVar6;
            puVar6 = (u16 *) ( puVar6 + 0x1);
            *(u8 *) puVar1 = uVar2;
        }
        if (struct_1 != NULL) {
            for (uVar5 =  struct_1 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
                puVar1 = puVar6;
                puVar6 = puVar6 + 0x1;
                *puVar1 = uVar3;
            }
            for (uVar5 =  (( struct_1 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
                puVar1 = puVar6;
                puVar6 = (u16 *) ( puVar6 + 0x1);
                *(u8 *) puVar1 = uVar2;
            }
        }
    }
    return (u16 *) param_1;
}

i16 pass1_1000_49b2(u16 param_1)
{
    return (param_1 ^  param_1 >> 0xf) - ( param_1 >> 0xf);
}

u16 pass1_1000_49c6(u16 param_1,
                    u16 param_2,
                    u16 param_3,
                    u16 param_4,
                    u16 param_5,
                    u16 param_6,
                    code5 fn_ptr_param_7)
{
    u16 uVar1;
    u16 uVar2;
    u16 uVar3;
    u16 uVar4;
    i16 iVar5;
    i16 iVar6;
    u32 uVar7;
    u16 uStack20;
    u16 uStack18;
    u16 uStack8;
    u16 uStack6;

    uStack20 = param_3;
    uStack18 = param_4;
    uVar7 = pass1_1000_52be(param_5 - 0x1,
                            - (param_5 == 0x0),
                            param_6,
                            0x0);
    uStack8 =  (uVar7 + 0x8);
    uStack6 =  (uVar7 + 0x8 >> 0x10) * 0x100 + param_4;
    while (true) {
        if (uStack6 < uStack18) {
            return 0x0;
        }
        if ((uStack6 <= uStack18) && (uStack8 < uStack20)) {
            return 0x0;
        }
        uVar1 = param_5 >> 0x1;
        if (uVar1 == 0x0) {
            if ((param_5 != 0x0) && (iVar5 = ((code5) fn_ptr_param_7)(), iVar5 == 0x0)) {
                return uStack20;
            }
            return 0x0;
        }
        uVar2 = uVar1;
        if ((param_5 & 0x1) == 0x0) {
            uVar2 = uVar1 - 0x1;
        }
        uVar3 =  ((u32) uVar2 * (u32) param_6);
        uVar4 = uVar3 + uStack20;
        iVar6 = ( ((u32) uVar2 * (u32) param_6 >> 0x10) +  CARRY2(uVar3,
                                                                            uStack20)) * 0x100 + uStack18;
        iVar5 = fn_ptr_param_7();
        if (iVar5 == 0x0) {
            break;
        }
        if (iVar5 < 0x0) {
            uStack8 = -param_6 + uVar4;
            uStack6 = ( CARRY2(-param_6,
                                    uVar4) -  (param_6 != 0x0)) * 0x100 + iVar6;
            uVar2 = param_5 & 0x1;
            param_5 = uVar1;
            if (uVar2 == 0x0) {
                param_5 = uVar1 - 0x1;
            }
        } else {
            uStack20 = param_6 + uVar4;
            uStack18 =  CARRY2(param_6,
                                    uVar4) * 0x100 + iVar6;
            param_5 = uVar1;
        }
    }
    return uVar4;
}

void pass1_1000_4aea(u16 param_1,
                     u16 param_2,
                     i16 param_3,
                     u16 param_4,
                     code5 fn_ptr_param_5)
{
    u16 *puVar1;
    code **ppcVar2;
    i32 lVar3;
    u16 uVar4;
    i16 iVar5;
    i16 iVar6;
    u16 uVar7;
    u16 uVar8;
    struct astruct_171 *puVar11;
    u16 uVar9;
    u16 uVar10;
    i16 unaff_DI;
    u16 uVar11;
    u16 unaff_CS;
    bool bVar12;
    u16 uStackY26;
    u16 uStackY24;
    u16 uStackY22;
    u16 uVar13;
    u16 uVar14;
    u16 uStackY18;
    u16 uStackY16;
    u16 uStackY14;

    if ((param_4 != 0x0) && (param_3 != 0x0)) {
        uStackY14 = param_1;
        uVar11 = param_2;
        for (iVar6 = param_3 + -0x1; iVar6 != 0x0; iVar6 += -0x1) {
            uVar9 = uStackY14 + param_4;
            uVar11 += - CARRY2(uStackY14,
                                    param_4) & 0x6c;
            uStackY18 = uVar9;
            uStackY16 = uVar11;
            iVar5 = fn_ptr_param_5();
            if (iVar5 < 0x0) {
                uVar11 = param_3 - 0x1;
                iVar6 = 0x0;
                do {
                    uVar11 >>= 0x1;
                    iVar6 += -0x1;
                } while (iVar6 != 0x0 && uVar11 != 0x0);
                if (( ((u32)  -iVar6 * 0x8 >> 0x10) != 0x0)
                    || (uVar11 = pass1_1000_3bac(), uVar11 <  ((u32)  -iVar6 * 0x8))) {
                    exit_1000_25f2(-0x4,
                                   0x4b7b,
                                   unaff_CS,
                                   unaff_DI);
                    return;
                }
                puVar11 = (astruct_171 *) &stack0xfff6;
                lVar3 = (u32) (param_3 - 0x1) * (u32) param_4;
                uVar11 =  lVar3;
                uStackY14 = uVar11 + param_1;
                uVar11 = ( ((u32) lVar3 >> 0x10) +  CARRY2(uVar11,
                                                                     param_1)) * 0x100 + param_2;
                uStackY16 = param_2;
                uStackY18 = param_1;
                LAB_1000_4b7d:
                if (puVar11 <= (astruct_171 *) &uStackY18) {
                    return;
                }
                LAB_1000_4b81:
                if ((uStackY16 < uVar11) || ((uStackY16 <= uVar11 && (uStackY18 < uStackY14)))) {
                    uStackY22 = uStackY14;
                    puVar1 = &puVar11->field20_0x14;
                    uVar8 = uStackY14 + *puVar1;
                    uVar7 = uVar11 + (- CARRY2(uStackY14,
                                                    *puVar1) & 0x6c);
                    uVar9 = uStackY16;
                    uVar10 = uStackY18;
                    uStackY26 = uStackY18;
                    uStackY24 = uStackY16;
                    uVar13 = uVar11;
                    LAB_1000_4bbc:
                    do {
                        puVar1 = &puVar11->field20_0x14;
                        bVar12 = CARRY2(uVar10,
                                        *puVar1);
                        uVar10 += *puVar1;
                        uVar9 += - bVar12 & 0x6c;
                        uVar4 = uStackY22;
                        if ((uVar10 != uStackY14) || (uVar9 != uVar11)) {
//                            ppcVar2 = puVar11->field21_0x16;
                            iVar6 = puVar11->field21_0x16();
                            if (iVar6 < 0x1) {
                                if (iVar6 != 0x0) {
                                    uStackY26 = uVar10;
                                    uStackY24 = uVar9;
                                }
                                goto LAB_1000_4bbc;
                            }
                        }
                        do {
                            uVar14 = uVar13;
                            uStackY22 = uVar4;
                            puVar1 = &puVar11->field20_0x14;
                            bVar12 = uVar8 < *puVar1;
                            uVar8 -= *puVar1;
                            uVar7 -= - bVar12 & 0x6c;
//                            ppcVar2 = (code **) &puVar11->field21_0x16;
//                            iVar6 = (**ppcVar2)();
                            iVar6 = puVar11->field21_0x16();
                            if (0x0 < iVar6) {
                                break;
                            }
                            uVar4 = uVar8;
                            uVar13 = uVar7;
                        } while (((iVar6 != 0x0) || (uVar4 = uStackY22, uVar13 = uVar14, uVar8 != uStackY18))
                            || (uVar7 != uStackY16));
                        if ((uVar7 < uVar9) || ((uVar7 <= uVar9 && (uVar8 <= uVar10)))) {
                            goto LAB_1000_4c58;
                        }
                        pass1_1000_4ceb(puVar11->field20_0x14);
                        uStackY26 = uVar10;
                        uStackY24 = uVar9;
                        uVar13 = uVar7;
                        uStackY22 = uVar8;
                    } while (true);
                }
                goto LAB_1000_4b7d;
            }
            uStackY14 = uVar9;
        }
    }
    return;
    LAB_1000_4c58:
    pass1_1000_4ceb(puVar11->field20_0x14);
    uVar10 = ((uVar11 - (- (uStackY14 < uStackY22) & 0x6c)) - uVar14) + (- CARRY2(uStackY14 - uStackY22,
                                                                                            uStackY18) & 0x6c)
        + uStackY16;
    uVar9 = - ((uStackY14 - uStackY22) + uStackY18 < uStackY26) & 0x6c;
    if ((uVar10 < uVar9) || (uVar10 - uVar9 < uStackY24)) {
        uStackY14 = uStackY26;
        uVar11 = uStackY24;
    } else {
        uStackY18 = uStackY22;
        uStackY16 = uVar14;
    }
    goto LAB_1000_4b81;
}

void pass1_1000_4ceb(u16 param_1)
{
    u8 *puVar1;
    u16 *puVar2;
    u8 uVar3;
    u16 uVar4;
    i16 unaff_SI;
    i16 unaff_DI;
    u16 unaff_ES;

    if ((param_1 & 0x1) != 0x0) {
        param_1 -= 0x1;
        puVar1 = (u8 *) (param_1 + unaff_DI);
        uVar3 = *puVar1;
        *puVar1 = *(u8 *) (param_1 + unaff_SI);
        *(u8 *) (param_1 + unaff_SI) = uVar3;
        if (param_1 == 0x0) {
            return;
        }
    }
    do {
        param_1 -= 0x2;
        puVar2 = (u16 *) (param_1 + unaff_DI);
        uVar4 = *puVar2;
        *puVar2 = (param_1 + unaff_SI);
        (param_1 + unaff_SI) = uVar4;
    } while (param_1 != 0x0);
    return;
}



u16 pass1_1000_4d24(void)
{
    u32 uVar1;

    uVar1 = pass1_1000_52be(DAT_1050_61e8,
                             PTR_LOOP_1050_61ea,
                             s_TPPOPMENU_1050_43fa + 0x3,
                            0x3);
    PTR_LOOP_1050_61ea = (u8 *) (uVar1 + 0x269ec3 >> 0x10);
    DAT_1050_61e8 =  (uVar1 + 0x269ec3);
    return  PTR_LOOP_1050_61ea & 0x7fff;
}

void str_1000_4d58(char *in_string_1,
                   char *in_string_2,
                   u32 param_3,
                   u32 param_4,
                   WNDCLASS16 *param_5)
{
    u16 uVar1;
    i16 iVar2;
    u16 uVar3;
    u16 uVar4;
    u16 uVar5;
    char *pcStack18;
    u16 uStack12;
    u16 uStack10;
    u16 uStack8;
    u16 uStack6;

    uStack10 = 0x0;
    uStack12 = 0x0;
    uVar4 =  ((u32) in_string_1 >> 0x10);
    iVar2 =  in_string_1;
    if ((*in_string_1 == '\0') || (*(char *) (iVar2 + 0x1) != ':')) {
        if (in_string_2 != NULL) {
            *in_string_2 = '\0';
        }
    } else {
        if (in_string_2 != NULL) {
            *in_string_2 = *in_string_1;
            *(u8 *) ( in_string_2 + 0x1) = *(u8 *) (iVar2 + 0x1);
            *(u8 *) ( in_string_2 + 0x2) = 0x0;
        }
        in_string_1 = (char *) ((u32) in_string_1 & 0xffff0000 | (u32) (iVar2 + 0x2));
    }
    uStack6 = 0x0;
    uStack8 = 0x0;
    pcStack18 = in_string_1;
    while (true) {
        uVar5 =  ((u32) pcStack18 >> 0x10);
        uVar3 =  pcStack18;
        if (*pcStack18 == '\0') {
            break;
        }
        if ((*pcStack18 == '/') || (*pcStack18 == '\\')) {
            uStack8 = uVar3 + 0x1;
            uStack6 = uVar5;
        } else if (*pcStack18 == '.') {
            uStack12 = uVar3;
            uStack10 = uVar5;
        }
        pcStack18 = (char *) ((u32) pcStack18 & 0xffff0000 | (u32) (uVar3 + 0x1));
    }
    if ((uStack6 | uStack8) == 0x0) {
        if (param_3 != 0x0) {
            *(u8 *) param_3 = 0x0;
        }
    } else {
        if (param_3 != 0x0) {
            uVar1 = uStack8 -  in_string_1;
            if (0xff <  uVar1) {
                uVar1 = 0xff;
            }
            str_op_1000_3dbe((char *) (param_3 & 0xffff | (u32) param_3 << 0x10),
                             in_string_1,
                             uVar1);
            *(u8 *) ( param_3 + uVar1) = 0x0;
        }
        in_string_1 = (char *) CONCAT22(uStack6,
                                        uStack8);
    }
    if (((uStack10 | uStack12) != 0x0) && ( in_string_1 <= uStack12)) {
        if (param_4 != 0x0) {
            uVar1 = uStack12 -  in_string_1;
            if (0xff <  uVar1) {
                uVar1 = 0xff;
            }
            str_op_1000_3dbe((char *) (param_4 & 0xffff | (u32) param_4 << 0x10),
                             (char *) ((u32) in_string_1 & 0xffff | (u32) in_string_1 << 0x10),
                             uVar1);
            *(u8 *) ( param_4 + uVar1) = 0x0;
        }
        if (param_5 == NULL) {
            return;
        }
        uVar1 = uVar3 - uStack12;
        if (0xff <  uVar1) {
            uVar1 = 0xff;
        }
        str_op_1000_3dbe((char *) ((u32) param_5 & 0xffff | (u32) param_5 << 0x10),
                         (char *) CONCAT22(uStack10,
                                           uStack12),
                         uVar1);
        *(u8 *) ( param_5 + uVar1) = 0x0;
        return;
    }
    if (param_4 != 0x0) {
        uVar1 = uVar3 -  in_string_1;
        if (0xff <  uVar1) {
            uVar1 = 0xff;
        }
        str_op_1000_3dbe((char *) (param_4 & 0xffff | (u32) param_4 << 0x10),
                         (char *) ((u32) in_string_1 & 0xffff | (u32) in_string_1 << 0x10),
                         uVar1);
        *(u8 *) ( param_4 + uVar1) = 0x0;
    }
    if (param_5 != NULL) {
        *(u8 *) &param_5->style = 0x0;
    }
    return;
}


/*
Unable to decompile 'pass1_1000_4f1a'
Cause:
Low-level Error: Symbol $$undef00000008 extends beyond the end of the address space
*/


// WARNING: Removing unreachable block (ram,0x10004f47)

u16 dos3_call_1000_4f20(void)
{
    code *pcVar1;
    u16 uVar2;
    i16 unaff_BP;
    bool bVar2;

    bVar2 = false;
    pcVar1 = (code *) swi(0x21);
    uVar2 = (*pcVar1)( 0x1050,
                      unaff_BP + 0x1);
    if (bVar2) {
        pass1_1000_29b5(uVar2);
        return 0xffff;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x10004f47)

u16 pass1_1000_4f2e(void)
{
    code *pcVar1;
    u16 uVar2;
    i16 unaff_BP;
    bool bVar3;

    bVar3 = false;
    pcVar1 = (code *) swi(0x21);
    uVar2 = (*pcVar1)( 0x1050,
                      unaff_BP + 0x1);
    if (bVar3) {
        pass1_1000_29b5(uVar2);
        return 0xffff;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x10004f6d)

u16 dos3call_1000_4f54(u32 param_1)
{
    char cVar1;
    code *pcVar2;
    u16 uVar3;
    i16 unaff_BP;
    bool bVar3;
    u32 uVar5;

    bVar3 = false;
    pcVar2 = (code *) swi(0x21);
    uVar5 = (*pcVar2)( 0x1050,
                      unaff_BP + 0x1);
    uVar5 = (char *) (uVar5 >> 0x10);
    uVar5._0_2_ =  uVar5;
    uVar3 =  uVar5;
    if ((bVar3) && (bVar3 =  uVar5 < 0x10,  uVar5 == 0x10)) {
        do {
            cVar1 = *uVar5;
            uVar5 = uVar5 + 0x1;
            if (cVar1 == '\0') {
                goto LAB_1000_4f90;
            }
        } while ((cVar1 != '?') && (cVar1 != '*'));
        uVar3 = 0x3;
        LAB_1000_4f90:
        bVar3 = true;
    }
    if (!bVar3) {
        return 0x0;
    }
    pass1_1000_29b5(uVar3);
    return 0xffff;
}



// WARNING: Removing unreachable block (ram,0x10004fa9)

i16 dos3_call_1000_4f94()
{
    code6 fn_ptr_1 = (code6)swi(0x21);
//    bVar2 = (*pcVar1)(unaff_BP + 0x1);
    i16 bVar2 = fn_ptr_1(unaff_BP + 1);
    return bVar2 + 0x1;
}

// WARNING: Removing unreachable block (ram,0x10004fd7)
// WARNING: Removing unreachable block (ram,0x10004feb)

u16 dos3_call_1000_4fbe(u8 param_1)
{
    u8 cVar2;
    u16 uVar3;
    //    i16 unaff_BP;

    code6 fn_ptr_var1 = (code6) swi(0x21);
    (fn_ptr_var1)(unaff_BP + 0x1);
    code4 fn_ptr_var2 = (code4) swi(0x21);
    cVar2 = fn_ptr_var2();
    uVar3 = 0xffff;
    if (cVar2 + '\x01' == param_1) {
        uVar3 = 0x0;
    }
    return uVar3;
}

void pass1_1000_5008(u16 param_1,
                     u16 param_2,
                     u16 param_3)
{
    pass1_1000_5026(0x0,
                    param_1,
                    param_2,
                    param_3);
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1000_5026(i16 param_1,
                     u16 param_2,
                     u16 param_3,
                     u16 param_4)
{
    u16 uVar1;
    u16 uVar2;
    i16 unaff_BP;
    u32 uStack304;
    u16 local_12c[0x3];
    u16 uStack294;
    u8 *local_124[0x6];
    i16 iStack280;
    u8 local_116;
    u8 uStack277;
    char cStack272;
    u8 *puStack270;
    u8 local_108;
    u8 uStack263;
    u8 uStack262;
    u8 auStack261[0x101];
    u16 local_4;
    i16 iStack2;

    iStack2 = unaff_BP + 0x1;
    local_4 = SUB42(0x1050,
                    0x0);
    uStack304 = (char *) CONCAT22(0x1050,
                                  &local_108);
    if (param_1 == 0x0) {
        param_1 = dos3_call_1000_4f94();
    }
    *uStack304 = (char) param_1 + '@';
    uStack263 = 0x3a;
    puStack270 = auStack261;
    uStack262 = 0x5c;
    uStack277 = 0x47;
    cStack272 = (char) param_1;
    local_12c[0] = SUB42(0x1050,
                         0x0);
    uStack294 = SUB42(0x1050,
                      0x0);
    dos3_call_set_struct_1000_42de((astruct_811 *) CONCAT22(0x1050,
                                                            &local_116),
                                   (astruct_810 *) CONCAT22(0x1050,
                                                            local_124),
                                   (u16 *) CONCAT22(0x1050,
                                                    local_12c));
    if (iStack280 == 0x0) {
        uVar1 = str_op_1000_3da4((char *) CONCAT22(0x1050,
                                                   &local_108));
        uVar1 += 0x1;
        uStack304._0_2_ = param_2;
        uStack304 = param_3;
        uVar2 = param_3 | param_2;
        if (uVar2 == 0x0) {
            if ( param_4 <  uVar1) {
                param_4 = uVar1;
            }
            uStack304._0_2_ = mem_1000_167a(0x0,
                                            param_4);
            uStack304 = uVar2;
            if ((uVar2 |  uStack304) == 0x0) {
                PTR_LOOP_1050_5f78 = (u8 *) &PTR_LOOP_1050_000c;
                return;
            }
        }
        if ( param_4 <  uVar1) {
            PTR_LOOP_1050_5f78 = (u8 *) ( s_New_failed_in_Op__Op_1050_0020 + 0x2);
        } else {
            unk_str_op_1000_3d3e((char *) CONCAT22(uStack304,
                                                    uStack304),
                                 (char *) CONCAT22(0x1050,
                                                   &local_108));
        }
    } else {
        PTR_LOOP_1050_5f78 = (u8 *) ( &PTR_LOOP_1050_000c + 0x1);
        PTR_LOOP_1050_5f88 = local_124[0];
    }
    return;
}



// WARNING: Removing unreachable block (ram,0x10005167)

u16 dos3_call_1000_514e(void)
{
    code *pcVar1;
    u16 uVar2;
    i16 unaff_BP;
    bool bVar2;

    bVar2 = false;
    pcVar1 = (code *) swi(0x21);
    uVar2 = (*pcVar1)( 0x1050,
                      unaff_BP + 0x1);
    if (bVar2) {
        pass1_1000_29b5(uVar2);
        return 0xffff;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x1000518c)

u16 dos3_call_1000_5174(void)
{
    code *pcVar1;
    u16 uVar2;
    i16 unaff_BP;
    bool bVar2;

    bVar2 = false;
    pcVar1 = (code *) swi(0x21);
    uVar2 = (*pcVar1)(unaff_BP + 0x1);
    if (!bVar2) {
        return 0x0;
    }
    pass1_1000_29b5(uVar2);
    return uVar2 & 0xff;
}



// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)

u16 dos3_calls_1000_5198(u16 param_1,
                         u16 param_2,
                         u16 param_3,
                         u16 param_4)
{
    code *pcVar1;

    pcVar1 = (code *) swi(0x21);
    (*pcVar1)( 0x1050);
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    if ((param_2 & 0x100) == 0x0) {
        return 0x0;
    }
    pass1_1000_29b5(param_3);
    return param_3 & 0xff;
}



// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)

u16 dos3_call_1000_51aa(u16 param_1,
                        u16 param_2,
                        u16 param_3)
{
    code *pcVar1;
    u16 uStack000a;

    pcVar1 = (code *) swi(0x21);
    (*pcVar1)( 0x1050);
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
    if ((param_2 & 0x100) == 0x0) {
        return 0x0;
    }
    uStack000a = param_3;
    pass1_1000_29b5(param_3);
    return uStack000a & 0xff;
}

u32 pass1_1000_5224(u16 param_1,
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
    bool bVar10;
    char cVar11;
    u16 uVar9;

    cVar11 =  param_2 < 0x0;
    if ((bool) cVar11) {
        bVar10 = param_1 != 0x0;
        param_1 = -param_1;
        param_2 = - bVar10 - param_2;
    }
    if ( param_4 < 0x0) {
        cVar11 += '\x01';
        bVar10 = param_3 != 0x0;
        param_3 = -param_3;
        param_4 = - bVar10 - param_4;
    }
    uVar3 = param_1;
    uVar5 = param_3;
    uVar6 = param_2;
    uVar9 = param_4;
    if (param_4 == 0x0) {
        uVar3 = param_2 / param_3;
        iVar4 =  (((u32) param_2 % (u32) param_3 << 0x10 | (u32) param_1) / (u32) param_3);
    } else {
        do {
            uVar8 = uVar9 >> 0x1;
            uVar5 = uVar5 >> 0x1 |  ((uVar9 & 0x1) != 0x0) << 0xf;
            uVar7 = uVar6 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar6 & 0x1) != 0x0) << 0xf;
            uVar6 = uVar7;
            uVar9 = uVar8;
        } while (uVar8 != 0x0);
        uVar1 = CONCAT22(uVar7,
                         uVar3) / (u32) uVar5;
        iVar4 =  uVar1;
        lVar2 = (u32) param_3 * (uVar1 & 0xffff);
        uVar3 =  ((u32) lVar2 >> 0x10);
        uVar5 = uVar3 + iVar4 * param_4;
        if (((CARRY2(uVar3,
                     iVar4 * param_4)) || (param_2 < uVar5)) || ((param_2 <= uVar5 && (param_1 <  lVar2)))) {
            iVar4 += -0x1;
        }
        uVar3 = 0x0;
    }
    if (cVar11 == '\x01') {
        bVar10 = iVar4 != 0x0;
        iVar4 = -iVar4;
        uVar3 = - bVar10 - uVar3;
    }
    return CONCAT22(uVar3,
                    iVar4);
}


u32 pass1_1000_52f0(u16 param_1,
                    u16 param_2,
                    u16 param_3,
                    u16 param_4)
{
    u32 uVar1;
    i32 lVar2;
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


i16 pass1_1000_545a(u32 param_1,
                    u32 param_2)
{
    u8 *pbVar1;
    u8 bVar2;
    u8 bVar3;
    u8 bVar4;
    u8 *pbVar5;
    u8 *pbVar6;

    pbVar6 = (u8 *) param_2;
    pbVar5 = (u8 *) param_1;
    bVar4 = 0xff;
    do {
        do {
            if (bVar4 == 0x0) {
                goto LAB_1000_5499;
            }
            pbVar1 = pbVar6;
            pbVar6 = pbVar6 + 0x1;
            bVar4 = *pbVar1;
            bVar3 = *pbVar5;
            pbVar5 = pbVar5 + 0x1;
        } while (bVar3 == bVar4);
        bVar2 = bVar4 + 0xbf + (-((u8) (bVar4 + 0xbf) < 0x1a) & 0x20U) + 0x41;
        bVar3 += 0xbf;
        bVar4 = bVar3 + (-(bVar3 < 0x1a) & 0x20U) + 0x41;
    } while (bVar4 == bVar2);
    bVar4 = (bVar4 < bVar2) * -0x2 + 0x1;
    LAB_1000_5499:
    return  (char) bVar4;
}

u16 *pass1_1000_54a0(u32 param_1,
                     u16 param_2,
                     u16 param_3)
{
    u16 *puVar1;
    u8 uVar2;
    u16 uVar3;
    u16 uVar4;
    u16 uVar5;
    u16 uVar6;
    u16 *puVar7;
    i16 iVar8;

    if (param_3 != 0x0) {
        iVar8 =  (param_1 >> 0x10);
        uVar5 = - (u16 *) param_1;
        uVar6 = param_3;
        if (uVar5 != 0x0) {
            uVar6 = (uVar5 - param_3 & - (uVar5 < param_3)) + param_3;
            uVar5 = param_3 - uVar6;
        }
        uVar3 = param_2 & 0xff | param_2 << 0x8;
        puVar7 = (u16 *) param_1;
        for (uVar4 = uVar6 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
            puVar1 = puVar7;
            puVar7 = puVar7 + 0x1;
            *puVar1 = uVar3;
        }
        for (uVar6 =  ((uVar6 & 0x1) != 0x0); uVar2 = (u8) (param_2 & 0xff), uVar6 != 0x0; uVar6 -= 0x1) {
            puVar1 = puVar7;
            puVar7 = (u16 *) ( puVar7 + 0x1);
            *(u8 *) puVar1 = uVar2;
        }
        if (uVar5 != 0x0) {
            for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
                puVar1 = puVar7;
                puVar7 = puVar7 + 0x1;
                *puVar1 = uVar3;
            }
            for (uVar6 =  ((uVar5 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
                puVar1 = puVar7;
                puVar7 = (u16 *) ( puVar7 + 0x1);
                *(u8 *) puVar1 = uVar2;
            }
        }
    }
    return (u16 *) param_1;
}

void pass1_1000_54e8(u8 *param_1,
                     u16 param_2,
                     i16 param_3,
                     i16 param_4,
                     i16 param_5,
                     u16 param_6)
{
    i16 iVar1;

    iVar1 = param_3;
    while (iVar1 += -0x1, -0x1 < iVar1) {
        ((code) param_1)();
    }
    return;
}

void pass1_1000_5512(u8 *param_1,
                     u16 param_2,
                     i16 param_3,
                     i16 param_4,
                     u16 param_5)
{
    bool bVar1;
    u16 uStack4;

    pass1_1000_52be(param_3,
                    param_4,
                    param_5,
                    0x0);
    while (true) {
        bVar1 = param_3 == 0x0;
        param_3 += -0x1;
        param_4 -=  bVar1;
        if (param_4 < 0x0) {
            break;
        }
        uStack4 = param_2;
        ((code) param_1)();
    }
}

/*
Unable to decompile 'pass1_1000_55b1'
Cause:
Low-level Error: Symbol $$undef00000007 extends beyond the end of the address space
*/

void pass1_1000_5586(code param_1,
                     u16 param_2,
                     i16 param_3,
                     i16 param_4,
                     i16 param_5,
                     u16 param_6)
{
    i16 iVar1;

    iVar1 = param_3;
    while (iVar1 += -0x1, -0x1 < iVar1) {
        ((code) param_1)();
    }
}



void exit_1000_25f2(i16 a, u16 b, u16 c, u16 d) {

}


//dos3_call_1000_23ea( param_4,
//                         0x1050,
//                        0x0,
//                         0x1050);
void dos3_call_1000_23ea(u8* a, u16 b, u16 c, u16 d) {

}
