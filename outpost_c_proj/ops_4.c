//
// Created by cyrex on 2022-06-06.
//

#include "ops_4.h"
#include "structs_2.h"

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
                      u32 re_alloc_size,
                      astruct_7 *param_3,
                      i16 selector)
{
    HGLOBAL16 handle;
    u32 global_handle_1;
    u16 realloc_flags;
    HGLOBAL16 global_handle_2;

    global_handle_1 = GlobalHandle16(selector);
    //  global_handle_1._0_2_ = (HGLOBAL16)global_handle_1;
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
    i16 *piVar9;
    i16 *piVar10;
    char *pcVar11;
    i16 *piVar12;
    bool bVar13;
    void *dos_env;
    u16 *puVar14;
    i16 *piVar1;
    u16 *puVar4;
    i16 *piVar4;

    dos_env = GetDOSEnvironment16();
    puVar7 = (u16 *) ((u32) dos_env >> 0x10);
    if ( dos_env != 0x0) {
        puVar7 = NULL;
    }
    iVar9 = 0x0;
    pcVar11 = NULL;
    iVar7 = -0x1;
    if (puVar7 != NULL) {
        cVar4 = *NULL;
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
    puVar5 = pass1_1000_2950(0x9,
                             puVar7,
                              (pcVar11 + 0x1) & 0xfffe);
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
        bVar13 = *piVar9 == s__C_FILE_INFO__1050_5f5c._0_2_;
        if (bVar13) {
            piVar12 = (i16 *) s__C_FILE_INFO__1050_5f5c;
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
            piVar9 = (i16 *) ( piVar9 + 0x1);
            cVar4 = *(char *) piVar2;
            puVar4 = puVar5;
            puVar5 = (u16 *) ( puVar5 + 0x1);
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
    init_op_1008_54aa( &DAT_1050_1050,
                      param_1,
                      param_2,
                      PTR_LOOP_1050_5f52,
                      CONCAT22(PTR_LOOP_1050_5f50,
                               PTR_LOOP_1050_5f4e),
                      PTR_LOOP_1050_5f4a,
                      (u8 *) HINSTANCE16_1050_5f4c);
    return;
}

void init_op_1008_54aa(u16 param_1,u16 param_2,u16 param_3,u8 *param_4,char *param_5,u8 *param_6,
                      u8 *param_7)

{
  code **ppcVar1;
  u16 uVar3;
  u16 in_CX;
  u16 in_DX;
  u16 extraout_DX;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  u16 unaff_SI;
  u16 unaff_DI;
  u16 unaff_CS;
  u32 uVar6;
  u32 uVar7;
  u16 in_stack_0000ffea;
  u16 in_stack_0000ffec;
  u32 *puStack12;
  u32 uVar2;

  if (param_6 != NULL) {
    return;
  }
  dos3_call_op_1000_435c(unaff_CS,NULL,unaff_SI,unaff_DI,in_stack_0000ffea,in_stack_0000ffec);
  pass1_1000_4d0c(param_1);
  pass1_1000_1fea();
  _PTR_LOOP_1050_03a0 = mem_op_1000_1902(in_DX,0x0,0x32,0x0,0x12);
  _PTR_LOOP_1050_029c = mem_op_1000_1902((int)(_PTR_LOOP_1050_03a0 >> 0x10),0x0,0x64,0x0,0xc);
  _PTR_LOOP_1050_4fb8 = mem_op_1000_1902((int)(_PTR_LOOP_1050_029c >> 0x10),0x0,0x64,0x0,0x10);
  _PTR_LOOP_1050_68a2 = mem_op_1000_1902((int)(_PTR_LOOP_1050_4fb8 >> 0x10),0x0,0x64,0x0,0xe);
  _PTR_LOOP_1050_5744 = mem_op_1000_1902((int)(_PTR_LOOP_1050_68a2 >> 0x10),0x0,0x1f4,0x0,0x42);
  uVar6 = mem_op_1000_1902((int)(_PTR_LOOP_1050_5744 >> 0x10),0x0,0x32,0x0,0x6);
  PTR_LOOP_1050_576a = (u8 *)(uVar6 >> 0x10);
  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,PTR_LOOP_1050_576a);
  PTR_LOOP_1050_5768 = (u8 *)uVar6;
  HINSTANCE16_1050_038c = (HINSTANCE16)param_7;
  PTR_LOOP_1050_038e = param_6;
  PTR_LOOP_1050_0390 = param_4;
  uVar3 = str_op_1008_60e8(PTR_LOOP_1050_576a,param_5);
  _PTR_LOOP_1050_0392 = CONCAT22((int)paVar5,uVar3);
  mem_op_1000_179c(0xc,paVar5);
  extraout_DX = paVar5 | uVar3;
  if (extraout_DX == 0x0) {
    uVar3 = 0x0;
    extraout_DX = 0x0;
  }
  else {
    struct_op_1008_0000((u16 *)CONCAT22(paVar5,uVar3));
  }
  puStack12 = (u32 *)CONCAT22(extraout_DX,uVar3);
  uVar4 = extraout_DX;
  if (_PTR_LOOP_1050_0392 != 0x0) {
    ppcVar1 = (code **)((int)*puStack12 + 0x4);
    (**ppcVar1)(0x1000,uVar3,extraout_DX,_PTR_LOOP_1050_0392);
  }
  uVar7 = CONCAT22(extraout_DX,uVar3);
  uVar2 = *puStack12;
  ppcVar1 = (code **)uVar2 + 0x4;
  (**ppcVar1)();
  win_msg_op_1008_9498();
  if (puStack12 != NULL) {
    ppcVar1 = (code **)uVar2;
    (**ppcVar1)(0x1000,uVar3,extraout_DX,0x1,uVar7);
  }
  uVar6 = mem_op_1000_1b68(uVar4,_PTR_LOOP_1050_03a0,(_PTR_LOOP_1050_03a0 >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_029c,(_PTR_LOOP_1050_029c >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_4fb8,(_PTR_LOOP_1050_4fb8 >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_68a2,(_PTR_LOOP_1050_68a2 >> 0x10));
  mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_5744,(_PTR_LOOP_1050_5744 >> 0x10));
  return;
}
