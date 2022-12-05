//
// Created by cyrex on 2022-06-06.
//

#include "ops_4.h"
#include "structs_2.h"
#include "sys_api.h"
#include "ops_1.h"
#include "globals.h"
#include "string_defs.h"
#include "utils.h"
#include "ops_5.h"

void pass1_1000_0368(u16 param_1,
                     u16 param_2,
                     u16 param_3)
{
    u16 *puVar1;

    if ((param_1 + 0x4) == param_1) {
        //        (param_3 + param_2 * 0x2) = 0x0;
    } else {
        //        ((param_1 + 0x6) + 0x4) = (param_1 + 0x4);
        //        ((param_1 + 0x4) + 0x6) = (param_1 + 0x6);
        puVar1 = (u16 *) (param_2 * 0x2 + param_3);
        if (*puVar1 == param_1) {
            *puVar1 = (param_1 + 0x4);
        }
    }
    //    (param_1 + 0x4) = (param_3 + 0xa);
    //    (param_3 + 0xa) = param_1;
}

void pass1_1000_20a2(u16 param_1,
                     u16 param_2)
{
    i16 iVar1;
    u16 uVar2;
    u16 uVar3;
    u16 uVar4;
    u16 uVar5;
    u16 uVar6;
    u16 uVar7;
    u16 uStack8;
    u16 uStack4;

    iVar1 = (param_1 + 0x2e);
    uVar2 = (param_1 + 0x30);
    uStack8 = 0x0;
    uVar3 = (iVar1 + 0x4);
    uStack4 = (iVar1 + 0x6);
    uVar7 = 0x0;
    if ((uStack4 | uVar3) != 0x0) {
        while ((uVar6 = uVar3, uVar4 = uStack4, uVar6 != param_1 || (uStack4 != param_2))) {
            uVar3 = (uVar6 + 0x2a);
            uStack4 = (uVar6 + 0x2c);
            uVar7 = uVar6;
            uStack8 = uVar4;
            if ((uStack4 | uVar3) == 0x0) {
                return;
            }
        }
        if ((uStack8 | uVar7) != 0x0) {
            uVar2 = (uVar6 + 0x2c);
            //            (uVar7 + 0x2a) = (uVar6 + 0x2a);
            //            (uVar7 + 0x2c) = uVar2;
            return;
        }
        uVar5 = (uVar6 + 0x2c);
        //        (iVar1 + 0x4) = (uVar6 + 0x2a);
        //        (iVar1 + 0x6) = uVar5;
    }
}

void mem_op_1000_1408(u16 param_1,
                      re_alloc_size: u32,
                      Struct7 *param_3,
                      selector: i16)
{
    HGLOBAL16 handle;
    u32 global_handle_1;
    u16 realloc_flags;
    HGLOBAL16 global_handle_2;

    global_handle_1 = GlobalHandle16(selector);
    //  global_handle_1 = (HGLOBAL16)global_handle_1;
    realloc_flags = 0x32;
    // (((param_1 & 0x1000) != 0x0) && ((re_alloc_size != 0x0 || (0xfff0 < re_alloc_size))))
    if (((param_1 & 0x1000) != 0x0) && ((re_alloc_size != 0x0 || 0xfff0 < re_alloc_size))) {
        re_alloc_size = 0xfff0;
    }
    if ((param_1 & 0x100) != 0x0) {
        realloc_flags = 0x72;
    }
    if ((param_1 & 0x804) != 0x0) {
        realloc_flags &= 0xfffd;
    }
    if ((HGLOBAL16) global_handle_1 != 0x0) {
        if ((param_1 & 0x4) != 0x0) {
            GlobalPageUnlock16((HGLOBAL16) global_handle_1);
        }
        do {
            global_handle_2 = (HGLOBAL16) global_handle_1;
            handle = GlobalReAlloc16(realloc_flags,
                                     re_alloc_size,
                                     (HGLOBAL16) global_handle_1);
            if (handle != 0x0) {
                break;
            }
            realloc_flags &= 0xffcf;
        } while (global_handle_2 != 0x0);
        if ((handle != 0x0) && ((param_1 & 0x4) != 0x0)) {
            GlobalPageLock16(handle);
        }
        if (handle != 0x0) {
            WIN16_GlobalLock16(handle);
            return;
        }
    }
    return;
}

void pass1_1000_27d6(u16 param_1)
{
    i16 *piVar2;
    char *pcVar3;
    char cVar4;
    u16 *puVar5;
    u16 **ppuVar6;
    i16 iVar7;
    u16 uVar7;
    i16 iVar8;
    u16 *puVar7;
    u16 *puVar8;
    i16 iVar9;
    char *piVar9;
    i16 *piVar10;
    char *pcVar11;
    char *piVar12;
    bool bVar13;
    void *dos_env;
    u16 *puVar14;
    i16 *piVar1;
    u16 *puVar4;
    i16 *piVar4;

    dos_env = GetDOSEnvironment16();
    puVar7 = (u16 *) ((u32) dos_env >> 0x10);
    if (dos_env != 0x0) {
        puVar7 = NULL;
    }
    iVar9 = 0x0;
    pcVar11 = NULL;
    iVar7 = -0x1;
    if (puVar7 != NULL) {
        //        cVar4 = *NULL;
        while (cVar4 != '\0') {
            do {
                if (iVar7 == 0x0) {
                    break;
                }
                iVar7 += -0x1;
                pcVar3 = pcVar11;
                pcVar11 = pcVar11 + 0x1;
            } while (*pcVar3 != '\0');
            iVar9 += 0x1;
            pcVar3 = pcVar11;
            pcVar11 = pcVar11 + 0x1;
            cVar4 = *pcVar3;
        }
    }
    uVar7 = 0x9;
    puVar8 = puVar7;
    //    puVar5 = pass1_1000_2950(0x9,
    //                             puVar7,
    //                              (pcVar11 + 0x1) & 0xfffe);
    puVar14 = puVar8;
    ppuVar6 = (u16 **) pass1_1000_2950(uVar7,
                                       puVar8,
                                       (iVar9 + 0x1) * 0x4);
    piVar9 = NULL;
    PTR_LOOP_1050_5fbe = (u8 *) ppuVar6;
    PTR_LOOP_1050_5fc0 = (u8 *) puVar8;
    do {
        if (iVar9 == 0x0) {
            *ppuVar6 = NULL;
            ppuVar6[0x1] = NULL;
            return;
        }
        bVar13 = piVar9 == s__C_FILE_INFO__1050_5f5c;
        if (bVar13) {
            piVar12 = s__C_FILE_INFO__1050_5f5c;
            iVar8 = 0x6;
            piVar10 = piVar9;
            do {
                if (iVar8 == 0x0) {
                    break;
                }
                iVar8 += -0x1;
                piVar4 = piVar12;
                piVar12 = piVar12 + 0x1;
                piVar1 = piVar10;
                piVar10 = piVar10 + 0x1;
                bVar13 = *piVar1 == *piVar4;
            } while (bVar13);
            if (!bVar13) {
                goto LAB_1000_2867;
            }
        } else {
            LAB_1000_2867:
            *ppuVar6 = puVar5;
            ppuVar6[0x1] = puVar14;
            ppuVar6 = ppuVar6 + 0x2;
        }
        do {
            piVar2 = piVar9;
            piVar9 = (i16 *) (piVar9 + 0x1);
            cVar4 = *(char *) piVar2;
            puVar4 = puVar5;
            puVar5 = (u16 *) (puVar5 + 0x1);
            *(char *) puVar4 = cVar4;
        } while (cVar4 != '\0');
        iVar9 += -0x1;
    } while (true);
}

u32 ret_op_1000_55ac(void)
{
}

void init_1000_23be(u16 param_1,
                    u16 param_2)
{
    init_op_1008_54aa(0x1050, // 0x1050
                      param_1,
                      param_2,
                      PTR_LOOP_1050_5f52,
                      CONCAT22(PTR_LOOP_1050_5f50,
                               PTR_LOOP_1050_5f4e),
                      PTR_LOOP_1050_5f4a,
                      (u8 *) HINSTANCE16_1050_5f4c);
    return;
}

void init_op_1008_54aa(u16 param_1,
                       u16 param_2,
                       u16 param_3,
                       u8 *param_4,
                       char *param_5,
                       u8 *param_6,
                       u8 *param_7)
{
    code **ppcVar1;
    u16 uVar3;
    //  u16 in_CX;
    //  u16 DX_REG;
    //  u16 DX_REG;
    u16 uVar4;
    //  u16 in_register_0000000a;
    Struct57 *paVar5;
    //  u16 SI_REG;
    //  u16 DI_REG;
    //  u16 CS_REG;
    u32 uVar6;
    u32 uVar7;
    u16 in_stack_0000ffea;
    u16 in_stack_0000ffec;
    u32 *puStack12;
    u32 uVar2;

    if (param_6 != NULL) {
        return;
    }
    dos3_call_op_1000_435c(CS_REG,
                           NULL,
                           SI_REG,
                           DI_REG,
                           in_stack_0000ffea,
                           in_stack_0000ffec);
    pass1_1000_4d0c(param_1);
    pass1_1000_1fea();
    PTR_LOOP_1050_03a0 = mem_op_1000_1902(DX_REG,
                                          0x0,
                                          0x32,
                                          0x0,
                                          0x12);
    PTR_LOOP_1050_029c = mem_op_1000_1902((int) (PTR_LOOP_1050_03a0 >> 0x10),
                                          0x0,
                                          0x64,
                                          0x0,
                                          0xc);
    PTR_LOOP_1050_4fb8 = mem_op_1000_1902((int) (PTR_LOOP_1050_029c >> 0x10),
                                          0x0,
                                          0x64,
                                          0x0,
                                          0x10);
    PTR_LOOP_1050_68a2 = mem_op_1000_1902((int) (PTR_LOOP_1050_4fb8 >> 0x10),
                                          0x0,
                                          0x64,
                                          0x0,
                                          0xe);
    PTR_LOOP_1050_5744 = mem_op_1000_1902((int) (PTR_LOOP_1050_68a2 >> 0x10),
                                          0x0,
                                          0x1f4,
                                          0x0,
                                          0x42);
    uVar6 = mem_op_1000_1902((int) (PTR_LOOP_1050_5744 >> 0x10),
                             0x0,
                             0x32,
                             0x0,
                             0x6);
    PTR_LOOP_1050_576a = (u8 *) (uVar6 >> 0x10);
    paVar5 = (Struct57 *) CONCAT22(AX_REG,
                                   PTR_LOOP_1050_576a);
    PTR_LOOP_1050_5768 = (u8 *) uVar6;
    HINSTANCE16_1050_038c = (HINSTANCE16) param_7;
    PTR_LOOP_1050_038e = param_6;
    PTR_LOOP_1050_0390 = param_4;
    uVar3 = str_op_1008_60e8(PTR_LOOP_1050_576a,
                             param_5);
    PTR_LOOP_1050_0392 = CONCAT22((int) paVar5,
                                  uVar3);
    mem_op_1000_179c(0xc,
                     paVar5);
    //  DX_REG = paVar5 | uVar3;
    if (DX_REG == 0x0) {
        uVar3 = 0x0;
        DX_REG = 0x0;
    } else {
        struct_op_1008_0000((u16 *) CONCAT22(paVar5,
                                             uVar3));
    }
    puStack12 = (u32 *) CONCAT22(DX_REG,
                                 uVar3);
    uVar4 = DX_REG;
    if (PTR_LOOP_1050_0392 != 0x0) {
        ppcVar1 = (code **) ((int) *puStack12 + 0x4);
        (**ppcVar1)(0x1000,
                    uVar3,
                    DX_REG,
                    PTR_LOOP_1050_0392);
    }
    uVar7 = CONCAT22(DX_REG,
                     uVar3);
    uVar2 = *puStack12;
    ppcVar1 = (code **) uVar2 + 0x4;
    (**ppcVar1)();
    win_msg_op_1008_9498();
    if (puStack12 != NULL) {
        ppcVar1 = (code **) uVar2;
        (**ppcVar1)(0x1000,
                    uVar3,
                    DX_REG,
                    0x1,
                    uVar7);
    }
    uVar6 = mem_op_1000_1b68(uVar4,
                             PTR_LOOP_1050_03a0,
                             (PTR_LOOP_1050_03a0 >> 0x10));
    uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),
                             PTR_LOOP_1050_029c,
                             (PTR_LOOP_1050_029c >> 0x10));
    uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),
                             PTR_LOOP_1050_4fb8,
                             (PTR_LOOP_1050_4fb8 >> 0x10));
    uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),
                             PTR_LOOP_1050_68a2,
                             (PTR_LOOP_1050_68a2 >> 0x10));
    mem_op_1000_1b68((uVar6 >> 0x10),
                     PTR_LOOP_1050_5744,
                     (PTR_LOOP_1050_5744 >> 0x10));
    return;
}

void dos3_call_op_1000_435c(u16 param_1,
                            u16 *param_2,
                            u16 param_3,
                            u16 param_4,
                            u16 param_5,
                            u16 param_6)
{
    code11 pfn_var1;
    u16 uVar2;
    //    u16 CX_REG;
    u16 uVar3;
    //    u16 DX_REG;
    u16 DX_REG_00;
    //    u16 DX_REG;
    u16 uVar4;
    //    u16 SS_REG;
    u16 uVar6;
    char cVar7;
    u16 uVar5;
    //    u16 in_stack_00000002;

    pfn_var1 = (code11) swi(0x21);
    //    (*pfn_var1)( 0x1050);
    pfn_var1(0x1050);
    code pfn_var2 = (code) swi(0x21);
    uVar3 = CX_REG;
    uVar2 = DX_REG;
    (pfn_var2)();
    uVar6 = DX_REG_00 >> 0x8;
    cVar7 = (char) uVar3;
    pfn_var1 = (code11) swi(0x21);
    (pfn_var1)(uVar3 >> 0x8);
    uVar4 = DX_REG;
    if ((uVar2 != DX_REG) && (uVar4 = DX_REG, cVar7 == '\x17')) {
        uVar3 = CX_REG;
        uVar4 = uVar2;
    }
    uVar2 = pass1_1000_462e(uVar4,
                            uVar3 - 0x7bc,
                            uVar4 >> 0x8,
                            uVar4 & 0xff,
                            uVar6,
                            param_1,
                            param_2);
    if (param_2 != 0x0) {
        //        ( param_2 + 0x2) = uVar4;
        *param_2 = uVar2;
    }
    return;
}

void pass1_1000_4d0c(u16 param_1)
{
    DAT_1050_61e8 = param_1;
    PTR_LOOP_1050_61ea = NULL;
    return;
}

BOOL16 pass1_1000_1fea(void)
{
    u8 *puVar1;
    bool bVar2;

    puVar1 = PTR_LOOP_1050_5f22 + 0x1;
    bVar2 = PTR_LOOP_1050_5f22 == NULL;
    PTR_LOOP_1050_5f22 = puVar1;
    if ((bVar2) && ((PTR_LOOP_1050_5f20 | PTR_LOOP_1050_5f1e) != 0x0)) {
        PTR_LOOP_1050_5f22 = (u8 *) &u16_1050_0002;
    }
    return 0x1;
}

u16 str_op_1008_60e8(u16 param_1,
                     char *param_2)
{
    u16 uVar1;
    u16 in_register_0000000a;
    Struct57 *paVar2;

    paVar2 = (Struct57 *) CONCAT22(in_register_0000000a,
                                   param_1);
    if (param_2 != NULL) {
        uVar1 = str_op_1000_3da4(param_2);
        uVar1 += 0x1;
        mem_op_1000_179c(uVar1,
                         paVar2);
        if ((paVar2->field0_0x0 | uVar1) != 0x0) {
            unk_str_op_1000_3d3e((char *) CONCAT22(paVar2,
                                                   uVar1),
                                 param_2);
            return uVar1;
        }
    }
    return 0x0;
}

u16 str_op_1000_3da4(char *param_1)
{
    char *pcVar1;
    u16 uVar2;
    char *pcVar3;
    bool bVar4;

    pcVar3 = (char *) param_1;
    bVar4 = true;
    uVar2 = 0xffff;
    do {
        if (uVar2 == 0x0) {
            break;
        }
        uVar2 -= 0x1;
        pcVar1 = pcVar3;
        pcVar3 = pcVar3 + 0x1;
        bVar4 = *pcVar1 == '\0';
    } while (!bVar4);
    uVar2 = ~uVar2;
    if (bVar4) {
        uVar2 -= 0x1;
    }
    return uVar2;
}

void mem_op_1000_179c(i16 param_1,
                      Struct57 *param_2)
{
    u8 *puVar1;
    u8 *puVar2;

    puVar1 = PTR_LOOP_1050_5f2c;
    puVar2 = PTR_LOOP_1050_5f2e;
    if ((*PTR_LOOP_1050_5f2e | *PTR_LOOP_1050_5f2c) == 0x0) {
        puVar1 = mem_op_1000_160a((StructD *) param_2);
        puVar2 = (u8 *) param_2;
    }
    fn_ptr_op_1000_1708(param_1,
                        0x0,
                        0x0,
                        puVar1,
                        puVar2);
    return;
}

void struct_op_1008_0000(u16 *param_1)
{
    i16 iVar1;
    u16 uVar2;

    uVar2 = ((u32) param_1 >> 0x10);
    //  iVar1 = (int)param_1;
    *param_1 = 0x52a;
    //  (iVar1 + 0x2) = 0x1008;
    //  (u32)(iVar1 + 0x4) = 0x0;
    //  (u32)(iVar1 + 0x8) = 0x0;
    //  *param_1 = 0x51e;
    //  (iVar1 + 0x2) = 0x1008;
    return;
}

WPARAM16 win_msg_op_1008_9498(void)
{
    BOOL16 BVar1;
    INT16 IVar2;
    MSG16 local_msg_1;

    LAB_1008_949c:
    BVar1 = GetMessage16(0x0,
                         0x0,
                         0x0,
                         &local_msg_1);
    if (BVar1 == 0x0) {
        return local_msg_1.wparam;
    }
    if (((int) u16_1050_5bc8 + 0x8) != 0x0) {
        goto code_r0x100894cd;
    }
    goto LAB_1008_94dc;
    code_r0x100894cd:
    BVar1 = IsDialogMessage16(&local_msg_1,
                              (HWND16) 0x1050);
    if (BVar1 == 0x0) {
        LAB_1008_94dc:
        if (PTR_LOOP_1050_0398 != NULL) {
            IVar2 = TranslateAccelerator16(&local_msg_1,
                                           (HACCEL16) 0x1050,
                                           (HWND16) PTR_LOOP_1050_0398);
            if (IVar2 != 0x0) {
                goto LAB_1008_949c;
            }
        }
        TranslateMessage16(&local_msg_1);
        DispatchMessage16(&local_msg_1);
    }
    goto LAB_1008_949c;
}

u32 mem_op_1000_1b68(u16 param_1,
                     u16 param_2,
                     u16 param_3)
{
    //    u16 unaff_CS;
    u32 uVar1;

    if ((param_2 + 0x14) != -0x4153) {
        pass1_1000_1e61(CS_REG,
                        0xa,
                        0x0,
                        0);
        return (u32) param_1 << 0x10;
    }
    uVar1 = mem_op_1000_1b9a(0x0,
                             param_2,
                             param_3);
    return uVar1;
}

i16 pass1_1000_462e(u16 param_1,
                    u16 param_2,
                    i16 param_3,
                    u16 param_4,
                    u16 param_5,
                    u16 param_6,
                    param_7: i16)
{
    u16 uVar1;
    u16 uVar2;
    u16 uVar3;
    u16 UVar4;
    u16 uVar5;
    u16 uVar6;
    i16 unaff_BP;
    u16 uVar7;
    u32 uVar8;
    i16 iStack26;
    u8 local_16[0x4];
    u16 uStack18;
    i16 iStack14;
    i16 iStack12;
    i16 iStack8;
    u16 local_4;
    i16 iStack2;
    u16 uVar10;
    u16 uVar11;
    u16 uVar12;
    u16 uVar13;

    iStack2 = BP_REG + 0x1;
    local_4 = 0x1050; //0x1050;
    uVar7 = (param_3 * 0x2 + 0x61ae);
    if (((param_2 & 0x3) == 0x0) && (0x2 < param_3)) {
        uVar7 += 0x1;
    }
    pass1_1000_43f0(param_1);
    uVar13 = 0x0;
    uVar12 = 0x3c;
    uVar11 = 0x0;
    uVar10 = 0x3c;
    uVar1 = ((long) param_2 * 0x16d);
    uVar2 = (param_2 + 0x3) >> 0x2;
    uVar3 = uVar2 + param_4;
    uVar5 = uVar1 + uVar3;
    uVar6 = uVar5 + uVar7;
    uVar8 = pass1_1000_52be(uVar6 + 0xe44,
                            ((u32) ((long) param_2 * 0x16d) >> 0x10) + ((param_2 + 0x3) >> 0xf) + (param_4 >> 0xf)
                                + CARRY2(uVar2,
                                         param_4) + CARRY2(uVar1,
                                                           uVar3) + (uVar7 >> 0xf) + CARRY2(uVar5,
                                                                                            uVar7) + (0xf1bb < uVar6),
                            0x18,
                            0x0);
    uVar8 = pass1_1000_52be((uVar8 + (long) param_5),
                            (uVar8 + (long) param_5 >> 0x10),
                            uVar10,
                            uVar11);
    uVar8 = pass1_1000_52be((uVar8 + (long) param_6),
                            (uVar8 + (long) param_6 >> 0x10),
                            uVar12,
                            uVar13);
    iStack26 = (uVar8 + (long) param_7 + CONCAT22(PTR_LOOP_1050_61d0,
                                                  DAT_1050_61ce));
    iStack8 = param_4 + uVar7;
    iStack12 = param_2 + 0x50;
    iStack14 = param_3 + -0x1;
    uStack18 = param_5;
    if (DAT_1050_61d2 != 0x0) {
        UVar4 = pass1_1000_455a(local_16,
                                0x1050);
        if (UVar4 != 0x0) {
            iStack26 += -0xe10;
        }
    }
    return iStack26;
}

void unk_str_op_1000_3d3e(char *param_1,
                          char *in_string_2)
{
    u16 *puVar4;
    u16 *puVar5;
    u16 uVar6;
    u16 uVar7;
    char *l_string_2;
    char *puVar6;
    char *puVar7;
    u16 uVar8;
    char *l_string_1;
    bool l_b_var8;
    char *puVar3;
    char *puVar2;
    char *puVar1;

    l_string_1 = (char *) ((u32) in_string_2 >> 0x10);
    l_string_2 = (char *) in_string_2;
    l_b_var8 = true;
    uVar6 = 0xffff;
    puVar6 = l_string_2;
    do {
        if (uVar6 == 0x0) {
            break;
        }
        uVar6 -= 0x1;
        puVar1 = puVar6;
        puVar6 = puVar6 + 0x1;
        l_b_var8 = *puVar1 == '\0';
    } while (!l_b_var8);
    uVar6 = ~uVar6;
    uVar8 = ((u32) param_1 >> 0x10);
    puVar7 = (char *) param_1;
    if (l_b_var8) {
        if (((u32) param_1 & 0x1) != 0x0) {
            puVar7 = puVar7 + 0x1;
            l_string_2 = l_string_2 + 0x1;
            *param_1 = *in_string_2;
            uVar6 -= 0x1;
        }
    } else {
        puVar7 = puVar7 + 0x2;
        l_string_2 = l_string_2 + 0x2;
        param_1 = in_string_2;
        uVar6 -= 0x1;
    }
    for (uVar7 = uVar6 >> 0x1; uVar7 != 0x0; uVar7 -= 0x1) {
        puVar5 = (u16 *) puVar7;
        puVar7 = (char *) (puVar7 + 0x2);
        puVar4 = (u16 *) l_string_2;
        l_string_2 = (char *) (l_string_2 + 0x2);
        *puVar5 = *puVar4;
    }
    for (uVar6 = ((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
        puVar5 = (u16 *) puVar7;
        puVar7 = (char *) (puVar7 + 0x1);
        puVar4 = (u16 *) l_string_2;
        l_string_2 = (char *) (l_string_2 + 0x1);
        *(u8 *) puVar5 = *(u8 *) puVar4;
    }
    return;
}

u16 fn_ptr_op_1000_1708(u16 param_1,
                        u16 param_2,
                        u16 param_3,
                        u16 param_4,
                        u16 param_5)
{
    i16 iVar1;
    bool bVar2;
    let mut lVar3: i32;

    if ((param_2 | param_1) == 0x0) {
        bVar2 = 0xfffe < param_1;
        param_1 += 0x1;
        param_2 += bVar2;
    }
    LAB_1000_1724:
    do {
        if ((param_5 | param_4) != 0x0) {
            lVar3 = mem_op_1000_0a48((u8) param_3,
                                     param_1,
                                     param_2,
                                     CONCAT22(param_5,
                                              param_4));
            if (lVar3 != 0x0) {
                return lVar3;
            }
        }
        if (((param_3 & 0x8000) == 0x0) || ((PTR_LOOP_1050_5f3a | PTR_LOOP_1050_5f38) == 0x0)) {
            if ((PTR_LOOP_1050_5f36 | PTR_LOOP_1050_5f34) == 0x0) {
                if ((PTR_LOOP_1050_5f3e | PTR_LOOP_1050_5f3c) == 0x0) {
                    return 0x0;
                }
                ((code) PTR_LOOP_1050_5f3c)();
                goto LAB_1000_1724;
            }
            iVar1 = ((code5) PTR_LOOP_1050_5f34)();
        } else {
            iVar1 = ((code5) PTR_LOOP_1050_5f38)();
        }
        if (iVar1 == 0x0) {
            return 0x0;
        }
    } while (true);
}

void pass1_1000_43f0(u16 param_1)
{
    if (PTR_LOOP_1050_68b4 == NULL) {
        pass1_1000_440c(param_1);
        PTR_LOOP_1050_68b4 = PTR_LOOP_1050_68b4 + 0x1;
    }
    return;
}

void pass1_1000_440c(u16 param_1)
{
    char cVar1;
    char *pcVar2;
    u16 uVar3;
    i16 iVar4;
    u16 uVar5;
    u16 uVar6;
    u32 uVar7;
    u16 uVar8;
    u16 uVar9;
    char *pcStack8;

    uVar3 = pass1_1000_3ec0(0x61ca,
                            0x1050);
    pcStack8 = (char *) CONCAT22(param_1,
                                 uVar3);
    if (((param_1 | uVar3) != 0x0) && (DAT_1050_61ce = CONCAT22(PTR_LOOP_1050_61d0,
                                                                DAT_1050_61ce), *pcStack8 != '\0')) {
        str_op_1000_3dbe((char *) CONCAT22(PTR_DAT_1050_61de,
                                           PTR_1050_61dc),
                         (char *) CONCAT22(param_1,
                                           uVar3),
                         0x3);
        pcStack8 = (char *) CONCAT22(param_1,
                                     uVar3 + 0x3);
        cVar1 = *pcStack8;
        if (cVar1 == '-') {
            pcStack8 = (char *) CONCAT22(param_1,
                                         uVar3 + 0x4);
        }
        uVar5 = 0x0;
        uVar9 = 0x0;
        uVar8 = 0xe10;
        uVar3 = pass1_1000_3e2c((u32) pcStack8 & 0xffff | (u32) param_1 << 0x10);
        _DAT_1050_61ce = pass1_1000_52be(uVar3,
                                         uVar5,
                                         uVar8,
                                         uVar9);
        for (; (pcVar2 = pcStack8, *pcStack8 == '+' || (('/' < *pcStack8 && (*pcStack8 < ':'))));
               pcStack8 = (char *) ((u32) pcStack8 & 0xffff0000 | (u32) (pcStack8 + 0x1))) {
        }
        if (*pcStack8 == ':') {
            uVar5 = 0x0;
            uVar9 = 0x0;
            uVar8 = 0x3c;
            pcStack8 = (char *) ((u32) pcStack8 & 0xffff0000 | (u32) (pcStack8 + 0x1));
            uVar3 = pass1_1000_3e2c((u32) pcVar2 & 0xffff0000 | (u32) (pcStack8 + 0x1));
            uVar7 = pass1_1000_52be(uVar3,
                                    uVar5,
                                    uVar8,
                                    uVar9);
            uVar6 = (uVar7 >> 0x10);
            _DAT_1050_61ce = uVar7 + _DAT_1050_61ce;
            for (; (pcVar2 = pcStack8, '/' < *pcStack8 && (*pcStack8 < ':'));
                   pcStack8 = (char *) ((u32) pcStack8 & 0xffff0000 | (u32) (pcStack8 + 0x1))) {
            }
            if (*pcStack8 == ':') {
                pcStack8 = (char *) ((u32) pcStack8 & 0xffff0000 | (u32) (pcStack8 + 0x1));
                iVar4 = pass1_1000_3e2c((u32) pcVar2 & 0xffff0000 | (u32) (pcStack8 + 0x1));
                _DAT_1050_61ce += CONCAT22(uVar6,
                                           iVar4);
                for (; ('/' < *pcStack8 && (*pcStack8 < ':'));
                       pcStack8 = (char *) ((u32) pcStack8 & 0xffff0000 | (u32) (pcStack8 + 0x1))) {
                }
            }
        }
        PTR_LOOP_1050_61d0 = (u8 *) (_DAT_1050_61ce >> 0x10);
        if (cVar1 == '-') {
            _DAT_1050_61ce = CONCAT22(-(PTR_LOOP_1050_61d0 + (DAT_1050_61ce != 0x0)),
                                      -DAT_1050_61ce);
        }
        DAT_1050_61d2 = *pcStack8;
        if (DAT_1050_61d2 == 0x0) {
            *_PTR_PTR_1050_61e0 = '\0';
        } else {
            str_op_1000_3dbe(_PTR_PTR_1050_61e0,
                             pcStack8,
                             0x3);
        }
    }
    PTR_LOOP_1050_61d0 = (u8 *) (_DAT_1050_61ce >> 0x10);
    return;
}

u16 pass1_1000_455a(u16 param_1,
                    u16 param_2)
{
    i16 *piVar1;
    i16 iVar2;
    u16 uVar3;
    i16 iVar4;
    u16 UVar5;
    u32 uVar6;
    i16 iStack6;

    if ((((param_1 + 0xa) < 0x43) || ((param_1 + 0x8) < 0x3)) || (0x9 < (param_1 + 0x8))) {
        goto LAB_1000_4623;
    }
    if (((param_1 + 0x8) < 0x4) || (0x8 < (param_1 + 0x8))) {
        uVar3 = (param_1 + 0xa);
        if ((uVar3 < 0x57) || ((param_1 + 0x8) != 0x3)) {
            iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b2);
        } else {
            iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b0) + 0x7;
        }
        if ((uVar3 & 0x3) == 0x0) {
            iStack6 += 0x1;
        }
        uVar3 = (uVar3 - 0x46) * 0x16d + ((uVar3 - 0x1) >> 0x2) + iStack6;
        uVar6 = pass1_1000_52f0(uVar3 - 0xd,
                                (uVar3 >> 0xf) - (uVar3 < 0xd),
                                0x7,
                                0x0);
        iStack6 = uVar6 - iStack6;
        iVar4 = -iStack6;
        if ((param_1 + 0x8) == 0x3) {
            iVar2 = (param_1 + 0xe);
            if ((iVar4 < iVar2) || ((-iVar2 == iStack6 && (0x1 < (param_1 + 0x4))))) {
                goto LAB_1000_460e;
            }
        } else {
            piVar1 = (i16 *) (param_1 + 0xe);
            iVar2 = *piVar1;
            if ((SBORROW2(*piVar1,
                          iVar4) != iVar2 + iStack6 < 0x0) || ((iVar2 == iVar4 && ((param_1 + 0x4) < 0x1)))) {
                goto LAB_1000_460e;
            }
        }
        LAB_1000_4623:
        UVar5 = 0x0;
    } else {
        LAB_1000_460e:
        UVar5 = 0x1;
    }
    return UVar5;
}

i16 pass1_1000_3ec0(u16 param_1,
                    u16 param_2)
{
    u16 uVar1;
    u16 uVar2;
    u16 uVar3;
    u16 unaff_SI;
    u16 uVar4;
    u32 *puVar4;

    puVar4 = (u32 *) CONCAT22(PTR_LOOP_1050_5fc0,
                              PTR_LOOP_1050_5fbe);
    if (((PTR_LOOP_1050_5fc0 | PTR_LOOP_1050_5fbe) != 0x0) && ((param_2 | param_1) != 0x0)) {
        uVar1 = str_op_1000_3da4((char *) CONCAT22(param_2,
                                                   param_1));
        while (true) {
            uVar4 = (u16) ((u32) puVar4 >> 0x10);
            uVar3 = (u16) puVar4;
            if (((uVar3 + 0x2) | puVar4) == 0x0) {
                break;
            }
            uVar2 = str_op_1000_3da4((char *) CONCAT22((uVar3 + 0x2),
                                                       puVar4));
            if (((uVar1 < uVar2) && (*(char *) (*puVar4 + uVar1) == '=')) && (uVar2 =
                                                                                  pass1_1000_3de8((char *) CONCAT22((uVar3
                                                                                                                        + 0x2),
                                                                                                                    puVar4),
                                                                                                  (char *) CONCAT22(param_2,
                                                                                                                    param_1),
                                                                                                  uVar1,
                                                                                                  unaff_SI,
                                                                                                  uVar3), uVar2
                == 0x0)) {
                return puVar4 + uVar1 + 0x1;
            }
            puVar4 = (u32 *) ((u32) puVar4 & 0xffff0000 | (u32) (uVar3 + 0x4));
        }
    }
    return 0x0;
}

u8 str_op_1000_3dbe(char *param_1,
                    char *param_2,
                    u16 param_3)
{
    char *pcVar1;
    char cVar2;
    char *pcVar3;
    char *pcVar4;
    u16 uVar5;

    uVar5 = ((u32) param_1 >> 0x10);
    pcVar4 = (char *) param_1;
    pcVar3 = (char *) param_2;
    if (param_3 != 0x0) {
        do {
            pcVar1 = pcVar3;
            pcVar3 = pcVar3 + 0x1;
            cVar2 = *pcVar1;
            if (cVar2 == '\0') {
                break;
            }
            pcVar1 = pcVar4;
            pcVar4 = pcVar4 + 0x1;
            *pcVar1 = cVar2;
            param_3 -= 0x1;
        } while (param_3 != 0x0);
        for (; param_3 != 0x0; param_3 -= 0x1) {
            pcVar1 = pcVar4;
            pcVar4 = pcVar4 + 0x1;
            *pcVar1 = '\0';
        }
    }
    return (u8) param_1;
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
