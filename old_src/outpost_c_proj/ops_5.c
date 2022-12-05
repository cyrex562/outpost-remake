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
//        pu_var1[0x18] =  0x1050;
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
    let mut lVar2: i32;
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
                    Struct7 *param_3,
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

//    uVar3 =  0x1050;
    uVar3 = 0x1050;
    UStack62 = param_3;
    UStack60 = param_4;
    UStack64 = param_2;
//    puStack4 = (u8 *) 0x1050;
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
//                                  0x1050,
//                                 0x0);
BVar2 = msg_box_op_1000_1f24( &PTR_PTR_1050_5f1a,
                                  0x1050,
                                 0x0);
    if (BVar2 == 0x0) {
//        u16_var3 = (*pfn_stack6)(0x1000,
//                            &UStack64,
//                             0x1050,
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
    let mut lVar7: i32;
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

void fn_ptr_op_1000_24cd(u16 param_1)
{
    code *pcVar1;
    i16 iVar2;
    u16 uVar2;
    u16 uVar6;
    u16 uVar5;
    u16 uVar3;
    u16 uVar4;

    u8_1050_5fc9 = '\0';
    fn_ptr_op_1000_2594();
    fn_ptr_op_1000_2594();
    ret_op_1000_55ac();
    fn_ptr_op_1000_2594();
    fn_ptr_op_1000_2594();
    dos3_op_1000_256b();
    pcVar1 = (code *) swi(0x21);
    (*pcVar1)();
}


u32 * mixed_1010_20ba(Struct57*param_1,param_2: u32,u8 **param_3,u16 param_4,u16 param_5,
                     u16 param_6,u16 param_7)
{
    code **ppcVar1;
    u8 *puVar2;
    u16 uVar3;
    u16 uVar4;
    Struct638 *paVar5;
    u16 unaff_SI;
    i16 iVar6;
    u16 uVar7;
    u16 uVar8;
    u16 *puVar9;
    u32 uVar10;
    u16 *puVar11;
    u32 uVar12;
    Struct19 *paVar13;
    u16 in_stack_0000fe8a;
    u16 in_stack_0000ffae;
    u16 in_stack_0000ffb4;
    u16 in_stack_0000ffb8;
    u32 *puStack6;

    pass1_1010_2816(param_2);
    paVar5 = (Struct638 *) (param_3 * 0x4);
    uVar7 = (param_2 >> 0x10);
    iVar6 = (int) param_2;
    puStack6 = (u32 *) ((int) &paVar5->field_0x0 + iVar6);
    if (puStack6 != NULL) {
        return puStack6;
    }
    switch (param_3) {
        case 0x1:
            mem_op_1000_179c(0x18,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL) {
                LAB_1010_2126:
                paVar5 = NULL;
                puVar2 = NULL;
            } else {
                struct_1010_3b7a((astruct_19 *) CONCAT22(param_1,
                                                         paVar5),
                                 param_3);
            }
            break;
        case 0x2:
            mem_op_1000_179c(0x84,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            win_sys_op_1010_5404(param_4,
                                 (astruct_19 *) CONCAT22(param_1,
                                                         paVar5),
                                 param_3);
            break;
        case 0x3:
            mem_op_1000_179c(0x53c,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            struct_1010_a1d8((astruct_19 *) CONCAT22(param_1,
                                                     paVar5),
                             param_3);
            break;
        case 0x4:
            mem_op_1000_179c(0x18a,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            struct_1018_2b10((astruct_19 *) CONCAT22(param_1,
                                                     paVar5),
                             param_3);
            break;
        case 0x5:
            mem_op_1000_179c(0x14,
                             param_1);
            if ((param_1 | paVar5) == 0x0)
                goto LAB_1010_2126;
            puVar11 = pass1_1008_eabc((astruct_19 *) CONCAT22(param_1,
                                                              paVar5),
                                      param_3);
            puVar2 = (u8 *) ((u32) puVar11 >> 0x10);
            paVar5 = (Struct638 *) puVar11;
            break;
        case 0x6:
            mem_op_1000_179c(0x58,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1018_18b8((astruct_19 *) CONCAT22(param_1,
                                                    paVar5),
                            param_3);
            break;
        case 0x7:
            mem_op_1000_179c(0xe,
                             param_1);
            uVar4 = (astruct_19 *) param_1 | paVar5;
            if (uVar4 == 0x0)
                goto LAB_1010_2126;
            uVar10 = pass1_1010_3d82((u32) param_1 & 0xffff0000 | (u32) uVar4,
                                     (astruct_19 *) paVar5,
                                     (astruct_19 *) param_1,
                                     param_3);
            puVar2 = (u8 *) (uVar10 >> 0x10);
            paVar5 = (Struct638 *) uVar10;
            break;
        case 0x8:
            mem_op_1000_179c(0x20,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            struct_1010_95aa((astruct_19 *) CONCAT22(param_1,
                                                     paVar5),
                             param_3);
            break;
        case 0x9:
            mem_op_1000_179c(0x26,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            struct_1010_6326((astruct_19 *) CONCAT22(param_1,
                                                     paVar5),
                             param_3);
            break;
        case 0xa:
            mem_op_1000_179c(0x1c,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            paVar13 = pass1_1010_0eac(puVar2,
                                      (astruct_19 *) CONCAT22(param_1,
                                                              paVar5),
                                      param_3);
            puVar2 = (u8 *) ((u32) paVar13 >> 0x10);
            paVar5 = (Struct638 *) paVar13;
            break;
        case 0xb:
            mem_op_1000_179c(0x1c,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            uVar10 = pass1_1008_aefe(puVar2,
                                     (astruct_19 *) CONCAT22(param_1,
                                                             paVar5),
                                     param_3);
            puVar2 = (u8 *) (uVar10 >> 0x10);
            paVar5 = (Struct638 *) uVar10;
            break;
        case 0xc:
        case 0xd:
        case 0xe:
        case 0xf:
        case 0x10:
        case 0x11:
        case 0x12:
        case 0x13:
        case 0x14:
        case 0x15:
        case 0x16:
        case 0x17:
        case 0x18:
        case 0x19:
        case 0x1a:
        case 0x1b:
        case 0x1c:
        case 0x1d:
        case 0x1e:
        case 0x1f:
        case 0x20:
        case 0x21:
        case 0x22:
        case 0x23:
        case 0x24:
            mem_op_1000_179c(0xaa,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            struct_1018_0570((astruct_19 *) CONCAT22(param_1,
                                                     paVar5),
                             param_3,
                             param_5);
            break;
        case 0x25:
            mem_op_1000_179c(0x1c,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1018_4aaa(puVar2,
                            (astruct_19 *) CONCAT22(param_1,
                                                    paVar5),
                            param_3);
            break;
        case 0x26:
            mem_op_1000_179c(0x1c,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1008_d99e(puVar2,
                            (astruct_19 *) CONCAT22(param_1,
                                                    paVar5),
                            param_3);
            break;
        case 0x27:
            mem_op_1000_179c(0x58,
                             param_1);
            puVar2 = (u8 *) ((astruct_19 *) param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1008_9d36((astruct_19 *) paVar5,
                            (astruct_19 *) param_1,
                            param_3);
            break;
        case 0x28:
            mem_op_1000_179c(0x2c,
                             param_1);
            uVar4 = (astruct_19 *) param_1 | paVar5;
            uVar10 = (u32) param_1 & 0xffff0000 | (u32) uVar4;
            if (uVar4 == 0x0)
                goto LAB_1010_2126;
            pass1_1010_28e6(uVar10,
                            0x1000,
                            (astruct_19 *) paVar5,
                            (astruct_19 *) param_1,
                            param_3);
            puVar2 = (u8 *) uVar10;
            break;
        case 0x29:
            mem_op_1000_179c(0x72,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            struct_1018_229c(puVar2,
                             (astruct_19 *) CONCAT22(param_1,
                                                     paVar5),
                             param_3);
            break;
        case 0x2a:
            mem_op_1000_179c(0x1c,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1010_503e(puVar2,
                            (astruct_19 *) CONCAT22(param_1,
                                                    paVar5),
                            param_3);
            break;
        case 0x2b:
            mem_op_1000_179c(0x1a,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            struct_1010_02e0((astruct_19 *) CONCAT22(param_1,
                                                     paVar5),
                             param_3);
            break;
        case 0x2c:
            mem_op_1000_179c(0x10,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1008_eb2a((astruct_19 *) CONCAT22(param_1,
                                                    paVar5),
                            param_3);
            break;
        case 0x2d:
            mem_op_1000_179c(0x80,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1010_3e3c((astruct_19 *) CONCAT22(param_1,
                                                    paVar5),
                            param_3,
                            param_6);
            break;
        case 0x2e:
            mem_op_1000_179c(0x806,
                             param_1);
            if ((param_1 | paVar5) == 0x0)
                goto LAB_1010_2126;
            uVar10 = pass1_1018_1ff4((astruct_19 *) CONCAT22(param_1,
                                                             paVar5),
                                     param_3);
            puVar2 = (u8 *) (uVar10 >> 0x10);
            paVar5 = (Struct638 *) uVar10;
            break;
        case 0x2f:
            mem_op_1000_179c(0x58,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            struct_1010_e9e4((astruct_19 *) CONCAT22(param_1,
                                                     paVar5),
                             param_3);
            break;
        case 0x30:
            mem_op_1000_179c(0xe,
                             param_1);
            if ((param_1 | paVar5) == 0x0)
                goto LAB_1010_2126;
            puVar9 = pass1_1010_3702((astruct_19 *) CONCAT22(param_1,
                                                             paVar5),
                                     param_3);
            puVar2 = (u8 *) ((u32) puVar9 >> 0x10);
            paVar5 = (Struct638 *) puVar9;
            break;
        case 0x31:uVar8 = 0x1000;
            mem_op_1000_179c(0x60,
                             param_1);
            uVar4 = param_1 | paVar5;
            if (uVar4 == 0x0) {
                LAB_1010_2680:
                uVar8 = 0x1000;
                paVar5 = NULL;
                puVar2 = NULL;
            } else {
                uVar10 = pass1_1010_9298((StructD *) CONCAT22(paVar5,
                                                              uVar4),
                                         (astruct_19 *) CONCAT22(param_1,
                                                                 paVar5),
                                         param_3);
                puVar2 = (u8 *) (uVar10 >> 0x10);
                paVar5 = (Struct638 *) uVar10;
            }
            goto LAB_1010_2683;
        case 0x32:
            mem_op_1000_179c(0x26,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1010_6abc((astruct_19 *) CONCAT22(param_1,
                                                    paVar5),
                            param_3,
                            param_6);
            break;
        case 0x33:
            mem_op_1000_179c(0x72,
                             param_1);
            uVar4 = (astruct_19 *) param_1 | paVar5;
            if (uVar4 == 0x0) {
                LAB_1010_25b8:
                uVar8 = 0x0;
                uVar3 = 0x0;
            } else {
                paVar13 = pass1_1010_195e((u32) param_1 & 0xffff0000 | (u32) uVar4,
                                          (astruct_19 *) paVar5,
                                          (astruct_19 *) param_1,
                                          param_3);
                uVar3 = ((u32) paVar13 >> 0x10);
                uVar8 = SUB42(paVar13,
                              0x0);
            }
            goto LAB_1010_25bb;
        case 0x34:
            mem_op_1000_179c(0x72,
                             param_1);
            uVar4 = (astruct_19 *) param_1 | paVar5;
            if (uVar4 == 0x0)
                goto LAB_1010_25b8;
            paVar13 = pass1_1010_1b6e((StructD *) ((u32) param_1 & 0xffff0000 | (u32) uVar4),
                                      (astruct_19 *) paVar5,
                                      (astruct_19 *) param_1,
                                      param_3);
            uVar3 = ((u32) paVar13 >> 0x10);
            uVar8 = SUB42(paVar13,
                          0x0);
        LAB_1010_25bb:
            puStack6 = (u32 *) CONCAT22(uVar3,
                                        uVar8);
            pass1_1010_1146(CONCAT22(uVar3,
                                     uVar8),
                            0x0);
            goto switchD_1010_2765_caseD_38;
        case 0x35:
            mem_op_1000_179c(0x14a,
                             param_1);
            if ((param_1 | paVar5) == 0x0)
                goto LAB_1010_2126;
            paVar13 = pass1_1010_6700((astruct_19 *) CONCAT22(param_1,
                                                              paVar5),
                                      param_3);
            puVar2 = (u8 *) ((u32) paVar13 >> 0x10);
            paVar5 = (Struct638 *) paVar13;
            break;
        case 0x36:
            mem_op_1000_179c(0x10,
                             param_1);
            puVar2 = (u8 *) ((astruct_19 *) param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1008_d790((astruct_19 *) paVar5,
                            (astruct_19 *) param_1,
                            param_3);
            break;
        case 0x37:
            mem_op_1000_179c(0x420,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            struct_1008_9fd2((astruct_19 *) CONCAT22(param_1,
                                                     paVar5),
                             param_3);
            break;
        default:goto switchD_1010_2765_caseD_38;
        case 0x39:
            mem_op_1000_179c(0x36,
                             param_1);
            uVar4 = (astruct_19 *) param_1 | paVar5;
            uVar10 = (u32) param_1 & 0xffff0000 | (u32) uVar4;
            if (uVar4 == 0x0)
                goto LAB_1010_2126;
            pass1_1010_4a8a(uVar10,
                            (astruct_19 *) paVar5,
                            (astruct_19 *) param_1,
                            param_3,
                            param_7,
                            in_stack_0000fe8a,
                            in_stack_0000ffae,
                            in_stack_0000ffb4,
                            in_stack_0000ffb8);
            puVar2 = (u8 *) uVar10;
            break;
        case 0x3a:
            mem_op_1000_179c(0xc,
                             param_1);
            if ((param_1 | paVar5) == 0x0)
                goto LAB_1010_2126;
            puVar9 = pass1_1008_d72e((astruct_19 *) CONCAT22(param_1,
                                                             paVar5),
                                     param_3);
            puVar2 = (u8 *) ((u32) puVar9 >> 0x10);
            paVar5 = (Struct638 *) puVar9;
            break;
        case 0x3b:
            mem_op_1000_179c(0x22,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            struct_1008_dd4e((astruct_19 *) CONCAT22(param_1,
                                                     paVar5),
                             param_3);
            break;
        case 0x3c:
            mem_op_1000_179c(0x146,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1018_331c(puVar2,
                            paVar5,
                            param_1,
                            param_3);
            break;
        case 0x3d:
            mem_op_1000_179c(0xe,
                             param_1);
            if ((param_1 | paVar5) == 0x0)
                goto LAB_1010_2126;
            uVar10 = pass1_1010_8c32((astruct_19 *) CONCAT22(param_1,
                                                             paVar5),
                                     param_3);
            puVar2 = (u8 *) (uVar10 >> 0x10);
            paVar5 = (Struct638 *) uVar10;
            break;
        case 0x3e:
            mem_op_1000_179c(0x18,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1018_5070((astruct_19 *) CONCAT22(param_1,
                                                    paVar5),
                            param_3);
            break;
        case 0x3f:
            mem_op_1000_179c(0x12,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1008_c72a((astruct_19 *) CONCAT22(param_1,
                                                    paVar5),
                            param_3,
                            unaff_SI);
            break;
        case 0x40:
            mem_op_1000_179c(0x24,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            pass1_1008_af94((astruct_19 *) CONCAT22(param_1,
                                                    paVar5),
                            param_3,
                            unaff_SI);
            break;
        case 0x41:
            mem_op_1000_179c(0x60,
                             param_1);
            uVar4 = param_1 | paVar5;
            if (uVar4 == 0x0)
                goto LAB_1010_2680;
            uVar8 = 0x1008;
            uVar12 = struct_1008_ecb2(paVar5,
                                      uVar4,
                                      (astruct_19 *) CONCAT22(param_1,
                                                              paVar5),
                                      param_3);
            puVar2 = (u8 *) ((u32) uVar12 >> 0x10);
            paVar5 = (Struct638 *) uVar12;
        LAB_1010_2683:
            *(Struct638 **) (param_3 * 0x4 + iVar6) = paVar5;
            *(u8 **) (param_3 * 0x4 + iVar6 + 0x2) = puVar2;
            ppcVar1 = (code **) ((int) (u32) paVar5 + 0x10);
            (**ppcVar1)(uVar8,
                        paVar5,
                        puVar2);
            break;
        case 0x42:
            mem_op_1000_179c(0xc,
                             param_1);
            if ((param_1 | paVar5) == 0x0)
                goto LAB_1010_2126;
            puVar9 = pass1_1008_ec10((int) paVar5,
                                     param_1,
                                     param_3);
            puVar2 = (u8 *) ((u32) puVar9 >> 0x10);
            paVar5 = (Struct638 *) puVar9;
            break;
        case 0x43:
            mem_op_1000_179c(0x12,
                             param_1);
            if ((param_1 | paVar5) == 0x0)
                goto LAB_1010_2126;
            puVar9 = pass1_1010_2bfc((astruct_19 *) CONCAT22(param_1,
                                                             paVar5),
                                     param_3);
            puVar2 = (u8 *) ((u32) puVar9 >> 0x10);
            paVar5 = (Struct638 *) puVar9;
            break;
        case 0x45:
            mem_op_1000_179c(0xe,
                             param_1);
            if ((param_1 | paVar5) == 0x0)
                goto LAB_1010_2126;
            paVar13 = pass1_1010_0000((astruct_19 *) CONCAT22(param_1,
                                                              paVar5),
                                      param_3);
            puVar2 = (u8 *) ((u32) paVar13 >> 0x10);
            paVar5 = (Struct638 *) paVar13;
            break;
        case 0x46:
            mem_op_1000_179c(0x1a,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            struct_1010_50b2((astruct_19 *) CONCAT22(param_1,
                                                     paVar5),
                             param_3);
            break;
        case 0x47:
            mem_op_1000_179c(0xe,
                             param_1);
            if ((param_1 | paVar5) == 0x0)
                goto LAB_1010_2126;
            puVar9 = pass1_1018_56e6((astruct_19 *) CONCAT22(param_1,
                                                             paVar5),
                                     param_3);
            puVar2 = (u8 *) ((u32) puVar9 >> 0x10);
            paVar5 = (Struct638 *) puVar9;
            break;
        case 0x48:
            mem_op_1000_179c(0x1c,
                             param_1);
            puVar2 = (u8 *) (param_1 | paVar5);
            if (puVar2 == NULL)
                goto LAB_1010_2126;
            unk_draw_op_1008_da12((astruct_19 *) CONCAT22(param_1,
                                                          paVar5),
                                  param_3);
    }
    puStack6 = (u32 *) CONCAT22(puVar2,
                                paVar5);
    switchD_1010_2765_caseD_38:
    (u32 *) (param_3 * 0x4 + iVar6) = puStack6;
    return puStack6;
}


void pass1_1010_2816(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  i16 iVar5;
  u16 uVar6;

  uVar6 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  if ((iVar4 + 0x124) != 0x0) {
    iVar5 = (iVar4 + 0x124) * 0x4;
    puVar1 = (u32 *)(iVar5 + iVar4);
    uVar2 = (iVar5 + iVar4 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    (u32)((iVar4 + 0x124) * 0x4 + iVar4) = 0x0;
    (iVar4 + 0x124) = 0x0;
  }

}


void fn_ptr_1000_17ce(char *param_1)
{
    if (param_1 != NULL) {
        call_fn_ptr_1000_0dc6(param_1);
    }
    return;
}



u32 pass1_1008_4772(Struct76 *param_1)

{
  bool bVar1;
  Struct76 *iVar2;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (Struct76 *)param_1;
  if (*(i32 *)&iVar2->field3_0x6 == 0x0) {
    pass1_1008_47cc((Struct76 *)((u32)param_1 & 0xffff | (u32)uVar2 << 0x10));
  }
  if (*(i32 *)&iVar2->field3_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(i32 *)&iVar2->field5_0xa == 0x0) {
      pass1_1008_4834((Struct76 *)((u32)param_1 & 0xffff | (u32)uVar2 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(((int)&iVar2->field8_0x10 + 0x2),&iVar2->field8_0x10);
}



void pass1_1008_47cc(Struct76 *param_1)

{
  u16 uVar1;
  u32 uVar2;
  u32 uVar3;
  u16 uVar4;
  Struct76 *iVar5;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  u32 uStack14;

  uVar7 = ((u32)param_1 >> 0x10);
  iVar5 = (Struct76 *)param_1;
  if (*(i32 *)&iVar5->field3_0x6 != 0x0) {
    uVar2 = (u32)&iVar5->field3_0x6;
    uVar1 = iVar5->field4_0x8;
    iVar6 = (int)uVar2;
    uVar4 = iVar6 + 0xe;
    iVar5->field8_0x10 = uVar2 & 0xffff0000 | (u32)uVar4;
    iVar5->field9_0x14 = iVar6 + 0x436;
    iVar5->field10_0x16 = uVar1 + (-(0xfbd7 < uVar4) & 0x6c);
    uVar3 = iVar5->field8_0x10;
    uVar8 = (uVar3 >> 0x10);
    iVar6 = (int)uVar3;
    uStack14 = (u32)(iVar6 + 0xe);
    *(i32 *)&iVar5->field11_0x18 = (long)(uStack14 * *(i32 *)(iVar6 + 0x4) + 0x1f) / 0x20 << 0x2;
  }
  return;
}
