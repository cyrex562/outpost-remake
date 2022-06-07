//
// Created by cyrex on 2022-06-06.
//

#include "ops_4.h"
#include "string_defs.h"
#include "ops_2.h"
#include "ops_3.h"
#include "structs_2.h"
#include "utils.h"
#include "globals.h"
#include "func_ptrs.h"
#include "types.h"
#include "ops_1.h"
#include "ops_5.h"
#include "sys_api.h"



u32 mem_op_1000_1902(u16 param_1,
                     u16 param_2,
                     u16 param_3,
                     u16 param_4,
                     u16 param_5)
{
    u16 *pu_var1;
    u16 UVar2;
    BOOL16 BVar3;
    i16 iVar4;
    u16 uVar3;
    u16 UVar5;
    u16 *pUVar6;
//    u16 CS_REG;
    DWORD DVar7;
    u32 uVar8;
    u16 *puVar1;

    UVar5 = param_1;
    if (((param_2 & 0x8000) != 0x0) && (UVar5 = param_1, _SHI_INVOKEERRORHANDLER1() != -0x6f70)) {
        param_2 |= 0x1;
        UVar5 = param_1;
    }
    do {
        uVar3 = UVar5;
        pu_var1 = (u16 *) (param_2 & 0xfffb | 0x1000);
        mem_op_1000_131c(pu_var1,
                         0x100);
//        UVar5 = uVar3 |  pu_var1;
        if (UVar5 != 0x0) {
            break;
        }
        UVar2 = pass1_1000_1e61(CS_REG,
                                0x2,
                                0x0, 0);
    } while (UVar2 != 0x0);
    if ((uVar3 |  *pu_var1) != 0x0) {
        pu_var1[0x17] =  PTR_PTR_1050_5f1a;
//        pu_var1[0x18] =  &DAT_1050_1050;
        puVar1[0x18] = 0x1050;
        pu_var1[0x15] =  PTR_LOOP_1050_5f1e;
        pu_var1[0x16] =  PTR_LOOP_1050_5f20;
        pUVar6 = pu_var1;
        PTR_LOOP_1050_5f1e = (u8 *) pu_var1;
        PTR_LOOP_1050_5f20 = (u8 *) uVar3;
        for (iVar4 = 0x5; iVar4 != 0x0; iVar4 += -0x1) {
            puVar1 = pUVar6;
            pUVar6 = pUVar6 + 0x1;
            *puVar1 = 0x0;
        }
        pu_var1[0x5] = 0x0;
        pu_var1[0x7] = 0x0;
        pu_var1[0x6] = 0x0;
        pu_var1[0x9] = 0x0;
        pu_var1[0x8] = 0x0;
        pu_var1[0xa] = 0xbead;
        pu_var1[0xb] = param_2 & 0xfffd;
        pu_var1[0xc] = 0x0;
        pu_var1[0xd] = 0x2000;
        pu_var1[0xe] = 0x800;
        DVar7 = mem_op_1000_1532(pu_var1,
                                 uVar3);
        pu_var1[0xf] =  DVar7;
        pu_var1[0x10] =  (DVar7 >> 0x10);
        pu_var1[0x12] = 0x0;
        pu_var1[0x11] = 0x0;
        pu_var1[0x13] = 0xfffe;
        pu_var1[0x14] = 0xffff;
        pu_var1[0x19] = 0x0;
        pu_var1[0x1a] = 0x0;
        pu_var1[0x20] = 0x0;
        pu_var1[0x1f] = 0x0;
        BVar3 = pass1_1000_1afe(param_5,
                                pu_var1,
                                uVar3);
        if (BVar3 != 0x0) {
            if ((param_4 | param_3) != 0x0) {
                pUVar6 = pu_var1;
                UVar5 = uVar3;
                uVar8 = pass1_1000_52be(param_3,
                                        param_4,
                                        param_5,
                                        0x0);
                pass1_1000_010c(0x1,
                                 uVar8,
                                 (uVar8 >> 0x10),
                                 pUVar6,
                                UVar5);
            }
            return CONCAT22(uVar3,
                            pu_var1);
        }
        mem_op_1000_1b9a(0x0,
                         pu_var1,
                         uVar3);
    }
    return 0x0;
}

void mem_op_1000_131c(u16 param_1,
                      u32 param_2)
{
    HGLOBAL16 handle;
    u16 flags;
    bool bVar1;
    i32 lVar2;
    u16 uStack10;
    u16 uStack8;
    i16 iStack6;

    lVar2 = CONCAT22(uStack8,
                     uStack10);
    flags = 0x32;
    iStack6 = 0x1;
    if (((param_1 & 0x1000) != 0x0) && ((param_2 != 0x0 || (0xfff0 <  param_2)))) {
        param_2 = 0xfff0;
    }
    if ((param_1 & 0x100) != 0x0) {
        flags = 0x72;
    }
    if ((param_1 & 0x1) != 0x0) {
        flags |= 0x2000;
    }
    if ((param_1 & 0x4) != 0x0) {
        flags &= 0xfffd;
        lVar2 = mem_op_1000_1558( param_2,
                                 param_2);
    }
    do {
        handle = GLobalAlloc16(param_2 & 0xffff | (u32) param_2 << 0x10,
                               flags);
        uStack8 =  ((u32) lVar2 >> 0x10);
        uStack10 =  lVar2;
        if (handle != 0x0) {
            break;
        }
        flags &= 0xffcf;
        bVar1 = iStack6 != 0x0;
        iStack6 = iStack6 + -0x1;
    } while (bVar1);
    if ((param_1 & 0x4) != 0x0) {
        if (handle != 0x0) {
            GlobalPageLock16(handle);
        }
        pass1_1000_15ce((u16 *) uStack10,
                        uStack8);
    }
    if (handle == 0x0) {
        return;
    }
    WIN16_GlobalLock16(handle);
    return;
}

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

u32 mem_op_1000_1b9a(u16 param_1,
                     u16 param_2,
                     u16 param_3)
{
    u16 uVar1;
    u16 uVar2;
    u32 uVar3;
    u16 uVar4;
    u16 uVar5;
    i16 iVar6;
    i32 lVar7;
    u16 *puStack8;
    u16 uStack4;

//    (param_2 + 0x14) = 0x0;
    uStack4 = 0x0;
    do {
        iVar6 = (uStack4 * 0x2);
        if (iVar6 != 0x0) {
            do {
                uVar3 = (u32) (iVar6 + 0x8);
//                ( uVar3 + 0xc) = 0x0;
                mem_op_1000_13ce((iVar6 + 0x8),
                                 (iVar6 + 0xa));
                iVar6 = (iVar6 + 0x4);
            } while ((uStack4 * 0x2) != iVar6);
        }
        uStack4 += 0x1;
    } while (uStack4 < 0x5);
    uVar4 = (param_2 + 0x10);
    uVar5 = (param_2 + 0x12);
    while (true) {
        puStack8 = (u16 *) CONCAT22(uVar5,
                                    uVar4);
        if ((uVar5 | uVar4) == 0x0) {
            break;
        }
        uVar1 = *puStack8;
        uVar2 = (uVar4 + 0x2);
        mem_op_1000_13ce(uVar4,
                         uVar5);
        uVar4 = uVar1;
        uVar5 = uVar2;
    }
    pass1_1000_20a2(param_2,
                    param_3);
    lVar7 = mem_op_1000_13ce(param_2,
                             param_3);
    return CONCAT22( ((u32) lVar7 >> 0x10),
                    0x1);
}

u32 pass1_1000_52be(u16 param_1,
                    u16 param_2,
                    u16 param_3,
                    u16 param_4)
{
    if ((param_4 | param_2) == 0x0) {
        return (u32) param_1 * (u32) param_3;
    }
    return (u32) param_1 * (u32) param_3 & 0xffff
        | (u32) ( ((u32) param_1 * (u32) param_3 >> 0x10) + param_2 * param_3 + param_1 * param_4) << 0x10;
}
