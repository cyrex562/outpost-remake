#include "fn_ptr_ops_6.h"

#include "types.h"
#include "utils.h"
#include "globals.h"
#include "unk/unk_11.h"
#include "fn_ptr_ops_7.h"

#include <winapi.h>

u32 pass1_1010_8ebc(u32 param_1, u8 param_2)

{
    u16 unaff_SS;

    pass1_1010_8c78(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void pass1_1010_9044(u32 param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = ((param_1 + 0x4) + 0x10);
    (**ppcVar1)();
    return;
}


void fn_ptr_1010_905e(u32 param_1, u32 param_2)

{
    u32  *puVar1;
    u16          uVar2;
    fn_ptr_1       *fn_ptr_1_list_a;
    astruct_169 *iVar4;
    u16          uVar4;

    uVar4  = (param_1 >> 0x10);
    iVar4  = (astruct_169 *)param_1;
    puVar1 = &iVar4->field_0x4;
    uVar2  = *(&iVar4->field_0x4 + 0x2);
    if((uVar2 | puVar1) != 0x0)
    {
        fn_ptr_1_list_a = *puVar1;
        (*fn_ptr_1_list_a)();
    }
    iVar4->field_0x4 = param_2;
    return;
}


void pass1_1010_9092(u32 param_1, u16 param_2, u16 param_3)

{
    fn_ptr_1     *fn_ptr_1_list_a;
    u32        uVar2;
    u8        *dx_reg_1;
    u8        *puVar3;
    u8        *puVar4;
    u16           dx_reg_2;
    i16           param_1_lo;
    u16           param_1_hi;
    u16        uVar7;
    u32 uVar8;
    u32        uStack14;
    u32        uStack6;

    param_1_hi      = (param_1 >> 0x10);
    param_1_lo      = param_1;
    uVar8   = (param_1_lo + 0x4);
    fn_ptr_1_list_a = ((param_1_lo + 0x4) + 0x10);
    (*fn_ptr_1_list_a)();
    uStack6 = CONCAT22(dx_reg_1, param_2);
    uVar7   = 0xc;
    puVar3  = dx_reg_1;
    mem_op_1000_179c(0xc, dx_reg_1, 0x1000);
    puVar4 = (puVar3 | param_2);
    if(puVar4 == 0x0)
    {
        param_2 = 0x0;
        puVar4  = 0x0;
    }
    else
    {
        pass1_1010_8ef2(CONCAT22(puVar3, param_2), puVar4, param_3);
    }
    for(uStack14 = 0x0; uStack14 < uStack6; uStack14 = uStack14 + 0x1)
    {
        fn_ptr_1_list_a = ((param_1_lo + 0x4) + 0x4);
        uVar2   = uStack6;
        (**fn_ptr_1_list_a)(0x1000, (param_1_lo + 0x4), uStack14, uVar7, uVar8);
        if((dx_reg_2 | uVar2) != 0x0)
        {
            fn_ptr_1_list_a = ((param_2 + 0x4) + 0xc);
            (**fn_ptr_1_list_a)(0x1000, (param_2 + 0x4), uVar2, dx_reg_2);
        }
    }
    return;
}

u16 *pass1_1010_922e(u16 *param_1, u8 param_2)

{
    pass1_1010_8f78(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1010_951a(u16 *param_1, u8 param_2)

{
    pass1_1010_927a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1010_9540(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_92e6(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u32 pass1_1010_7dc6(u32 param_1, u8 param_2)

{
    u16 unaff_SS;

    param_1 = param_1 & 0xffff0000 | (param_1 - 0xa);
    pass1_1010_6bb2(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1010_7dd2(u16 *param_1, u8 param_2)

{
    *param_1        = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void pass1_1010_7efc(u32 *param_1, u16 param_2)

{
    u16          uVar1;
    u16          uVar2;
    u32  *puVar3;
    code       **ppcVar4;
    astruct_448 *iVar5;
    u16          uVar5;
    Struct18  *paStack8;
    i16          iStack4;

    uVar5    = (param_1 >> 0x10);
    iVar5    = (astruct_448 *)param_1;
    uVar1    = iVar5->field_0x67c;
    uVar2    = iVar5->field_0x67e;
    paStack8 = (Struct18 *)CONCAT22(uVar2, uVar1);
    if((uVar2 | uVar1) != 0x0)
    {
        pass1_1008_64a2(CONCAT22(uVar2, uVar1));
        param_2 = 0x1000;
        fn_ptr_1000_17ce(paStack8, 0x1000);
    }
    for(iStack4 = 0x0; iStack4 < 0x8a; iStack4 = iStack4 + 0x1)
    {
        puVar3 = *(&iVar5->field_0x4 + iStack4 * 0x4);
        uVar1  = *(&iVar5->field_0x4 + iStack4 * 0x4 + 0x2);
        if((uVar1 | puVar3) != 0x0)
        {
            ppcVar4 = *puVar3;
            (**ppcVar4)(param_2, puVar3, uVar1, 0x1);
        }
        puVar3 = *(&iVar5->field_0x22c + iStack4 * 0x4);
        uVar1  = *(&iVar5->field_0x22c + iStack4 * 0x4 + 0x2);
        if((uVar1 | puVar3) != 0x0)
        {
            ppcVar4 = *puVar3;
            (**ppcVar4)(param_2, puVar3);
        }
        puVar3 = *(&iVar5->field_0x454 + iStack4 * 0x4);
        uVar1  = *(&iVar5->field_0x454 + iStack4 * 0x4 + 0x2);
        if((uVar1 | puVar3) != 0x0)
        {
            ppcVar4 = *puVar3;
            (**ppcVar4)(param_2, puVar3);
        }
    }
    fn_ptr_1000_17ce((Struct18 *)*param_1, 0x1000);
    globals->_PTR_LOOP_1050_14cc = 0x0;
    return;
}


void pass1_1010_7fd6(u32 param_1)

{
    u16          uVar1;
    u16          uVar2;
    astruct_489 *iVar3;
    u16          uVar3;
    Struct18  *paStack6;

    uVar3    = (param_1 >> 0x10);
    iVar3    = (astruct_489 *)param_1;
    uVar1    = iVar3->field_0x67c;
    uVar2    = iVar3->field_0x67e;
    paStack6 = (Struct18 *)CONCAT22(uVar2, uVar1);
    if((uVar2 | uVar1) != 0x0)
    {
        pass1_1008_64a2(CONCAT22(uVar2, uVar1));
        fn_ptr_1000_17ce(paStack6, 0x1000);
    }
    &iVar3->field_0x67c = 0x0;
    iVar3->field_0x680  = 0x0;
    return;
}

u16 *pass1_1010_66ca(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1010_6a86(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void pass1_1010_533c(u32 param_1, u32 param_2, u8 *param_3, u16 param_4)

{
    u16        *puVar1;
    u32         uVar2;
    u32  uVar3;
    i16         iVar4;
    u16         uVar5;
    u16         uVar6;
    Struct18 *paVar7;
    u16         uStack6;
    u8          local_4[0x2];

    pass1_1010_519a(param_1, CONCAT22(param_4, local_4), param_3, param_4);
    uStack6 = 0x0;
    while(true)
    {
        uVar6  = (param_1 >> 0x10);
        uVar5  = param_1;
        puVar1 = (uVar5 + 0x10);
        if(*puVar1 < uStack6 || *puVar1 == uStack6)
        {
            return;
        }
        uVar3   = (uVar5 + 0xc);
        uVar2   = *(uVar3 + uStack6 * 0x4);
        paVar7  = (Struct18 *)string_1010_5286(uVar5, uVar6, uVar2, uVar2, param_3);
        param_3 = (paVar7 >> 0x10);
        iVar4   = pass1_1000_3d7a(param_2, paVar7 & 0xffff | ZEXT24(param_3) << 0x10);
        if(iVar4 == 0x0)
            break;
        fn_ptr_1000_17ce(paVar7, 0x1000);
        uStack6 = uStack6 + 0x1;
    }
    return;
}


u16 *pass1_1010_53ce(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_50f2(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void struct_1010_5f1e(astruct_73 *param_1, char *param_2, u16 param_3)

{
    u16         uVar1;
    astruct_73 *iVar3;
    astruct_73 *uVar3;

    uVar3 = (astruct_73 *)(param_1 >> 0x10);
    iVar3 = (astruct_73 *)param_1;
    fn_ptr_1000_17ce(&iVar3->field_0x16, 0x1000);
    uVar1             = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0x16 = uVar1;
    iVar3->field_0x18 = param_3;
    return;
}


void pass1_1010_5f4c(u32 param_1, char *param_2, u16 param_3)

{
    u16          uVar1;
    astruct_484 *iVar3;
    u16          uVar2;

    uVar2 = (param_1 >> 0x10);
    iVar3 = (astruct_484 *)param_1;
    fn_ptr_1000_17ce(&iVar3->field_0x12, 0x1000);
    uVar1             = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0x12 = uVar1;
    iVar3->field_0x14 = param_3;
    return;
}

void pass1_1010_5fd8(u32 param_1, char *param_2, u16 param_3)

{
    u16          uVar1;
    astruct_485 *iVar3;
    u16          uVar2;

    uVar2 = (param_1 >> 0x10);
    iVar3 = (astruct_485 *)param_1;
    fn_ptr_1000_17ce(&iVar3->field_0x68, 0x1000);
    uVar1             = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0x68 = uVar1;
    iVar3->field_0x6a = param_3;
    return;
}


void pass1_1010_6006(u32 param_1, char *param_2, u16 param_3)

{
    u16          uVar1;
    astruct_486 *iVar3;
    u16          uVar2;

    uVar2 = (param_1 >> 0x10);
    iVar3 = (astruct_486 *)param_1;
    fn_ptr_1000_17ce(&iVar3->field_0x6c, 0x1000);
    uVar1             = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0x6c = uVar1;
    iVar3->field_0x6e = param_3;
    return;
}

void pass1_1010_60cc(u32 param_1, char *param_2, u16 param_3)

{
    u16          uVar1;
    astruct_487 *iVar3;
    u16          uVar2;

    uVar2 = (param_1 >> 0x10);
    iVar3 = (astruct_487 *)param_1;
    fn_ptr_1000_17ce(&iVar3->field_0x1a_addr_offset, 0x1000);
    uVar1             = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0x1a_addr_offset = uVar1;
    iVar3->field_0x1c_addr_base = param_3;
    return;
}

void pass1_1010_62a4(u16 *param_1, u8 param_2)

{
    astruct_488 *uVar2;
    u16          uVar1;

    uVar1            = (param_1 >> 0x10);
    uVar2            = (astruct_488 *)param_1;
    *param_1         = 0x6322;
    uVar2->field_0x2 = 0x1010;
    fn_ptr_1000_17ce((Struct18 *)uVar2->field_0x4, 0x1000);
    *param_1         = 0x389a;
    uVar2->field_0x2 = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return;
}
Struct18 *pass1_1010_4994(u16 param_1, Struct18 *param_2, u8 param_3, u16 param_4)

{
    param_2 = (Struct18 *)(param_2 & 0xffff0000 | (param_2 - 0x20));
    pass1_1010_3f00(param_2, param_4);
    if((param_3 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_2, 0x1000);
    }
    return param_2;
}

u16 *pass1_1010_4a20(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_3f00(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void pass1_1010_50f2(u16 *param_1, u16 param_2)

{
    u16 uVar1;

    uVar1           = (param_1 >> 0x10);
    *param_1        = 0x53f4;
    (param_1 + 0x2) = 0x1010;
    fn_ptr_1000_17ce((param_1 + 0xc), 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}

u16 *pass1_1010_36b4(u16 *param_1, u8 param_2)

{
    u16 unaff_SS;

    pass1_1010_2db2(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void pass1_1010_3730(u16 *param_1, u16 param_2)

{
    u16 uVar1;

    uVar1           = (param_1 >> 0x10);
    *param_1        = 0x37c4;
    (param_1 + 0x2) = 0x1010;
    fn_ptr_1000_17ce((param_1 + 0xa), 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}

void pass1_1010_3770(u32 param_1, char *param_2, u16 param_3)

{
    u16          uVar1;
    astruct_477 *iVar3;
    u16          uVar2;

    uVar2 = (param_1 >> 0x10);
    iVar3 = (astruct_477 *)param_1;
    fn_ptr_1000_17ce(&iVar3->field_0xa, 0x1000);
    uVar1            = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0xa = uVar1;
    iVar3->field_0xc = param_3;
    return;
}

u16 *pass1_1010_379e(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_3730(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void pass1_1010_3800(u16 *param_1)

{
    astruct_478 *iVar2;
    u16          uVar1;

    uVar1            = (param_1 >> 0x10);
    iVar2            = (astruct_478 *)param_1;
    *param_1         = 0x3b3e;
    iVar2->field_0x2 = 0x1010;
    if(iVar2->field_0x16 != 0x0)
    {
        fn_ptr_1000_17ce(iVar2->field_0x16, 0x1000);
    }
    pass1_1010_3880(param_1);
    return;
}
void pass1_1010_3880(AddrStruct *param_1)

{
    i16         *piVar1;
    u32  *puVar2;
    u16          uVar3;
    code       **ppcVar4;
    long         lVar5;
    astruct_472 *iVar6;
    i16          iVar7;
    u16          uVar8;
    u16          uVar9;
    i16          iStack4;

    uVar8            = (param_1 >> 0x10);
    iVar6            = (astruct_472 *)param_1;
    *param_1         = 0x3b5e;
    iVar6->field_0x2 = 0x1010;
    if(iVar6->field_0x8 != 0x0)
    {
        iStack4 = 0x0;
        while(true)
        {
            piVar1 = &iVar6->field_0x10;
            if(*piVar1 == iStack4 || *piVar1 < iStack4)
                break;
            lVar5  = iVar6->field_0x8;
            uVar9  = (lVar5 >> 0x10);
            iVar7  = lVar5;
            puVar2 = *(iVar7 + iStack4 * 0x4);
            uVar3  = *(iVar7 + iStack4 * 0x4 + 0x2);
            if((uVar3 | puVar2) != 0x0)
            {
                ppcVar4 = *puVar2;
                (**ppcVar4)();
            }
            iStack4 = iStack4 + 0x1;
        }
        fn_ptr_1000_17ce((Struct18 *)iVar6->field_0x8, 0x1000);
    }
    *param_1         = 0x389a;
    iVar6->field_0x2 = 0x1008;
    return;
}

u16 *pass1_1010_3af2(u16 *param_1, u8 param_2)

{
    pass1_1010_3800(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1010_3b18(u16 *param_1, u8 param_2)

{
    pass1_1010_3880(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1010_3d38(u16 *param_1, u8 param_2)

{
    u16 unaff_SS;

    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xa));
    pass1_1010_3bde(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1010_3e06(u16 *param_1, u8 param_2)

{
    u16 unaff_SS;

    pass1_1010_3dc8(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void pass1_1010_3f00(u16 *param_1, u16 param_2)

{
    u32  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    i16         *piVar4;
    astruct_481 *iVar5;
    u16          uVar5;
    u16         *puStack16;
    i16          iStack4;

    uVar5             = (param_1 >> 0x10);
    iVar5             = (astruct_481 *)param_1;
    *param_1          = &PTR_LOOP_1050_4a46;
    iVar5->field_0x2  = 0x1010;
    iVar5->field_0x20 = &PTR_LOOP_1050_4a82;
    iVar5->field_0x22 = 0x1010;
    iStack4           = 0x0;
    do
    {
        puVar1 = *(&iVar5->field_0x26 + iStack4 * 0x4);
        uVar2  = *(&iVar5->field_0x26 + iStack4 * 0x4 + 0x2);
        if((uVar2 | puVar1) != 0x0)
        {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        iStack4 = iStack4 + 0x1;
    } while(iStack4 < 0x10);
    puVar1 = iVar5->field_0x66;
    uVar2  = iVar5->field_0x68;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    fn_ptr_1000_17ce((Struct18 *)iVar5->field_0x70, 0x1000);
    if(param_1 == 0x0)
    {
        piVar4 = 0x0;
        uVar5  = 0x0;
    }
    else
    {
        piVar4 = &iVar5->field_0x20;
    }
    puStack16   = CONCAT22(uVar5, piVar4);
    *puStack16  = 0x389a;
    piVar4[0x1] = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}

astruct_11 *pass1_1010_2bbe(astruct_11 *param_1, u8 param_2)

{
    pass1_1010_29c6(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1010_2c9c(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void pass1_1010_2db2(astruct_473 *param_1, u16 param_2)

{
    u32  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    astruct_473 *param_1_lo;
    u16          param_1_hi;

    param_1_hi              = (param_1 >> 0x10);
    param_1_lo              = (astruct_473 *)param_1;
    param_1->offset         = 0x36da;
    param_1_lo->base = 0x1010;
    puVar1           = param_1_lo->field_0x56;
    uVar2            = param_1_lo->field_0x58;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    fn_ptr_1000_17ce((Struct18 *)param_1_lo->field_0x5c, 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}

u16 *pass1_1010_18f4(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_0f76(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1010_1b04(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_0f76(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1010_1cde(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_0f76(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1010_1fbe(u16 *param_1, u8 param_2)

{
    *param_1        = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1010_1fea(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1010_0e46(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_0350(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1010_0e6c(u16 *param_1, u8 param_2)

{
    *param_1        = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1008_ea86(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1008_ddca(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1008_eaf4(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_ebda(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1008_ec3c(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1008_ef50(u16 *param_1, u8 param_2)

{
    pass1_1008_ec94(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


Struct18 *pass1_1008_ef76(Struct18 *param_1, u8 param_2)

{
    u16 unaff_SS;

    pass1_1008_ed00(&param_1->field_0x0, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1010_02a2(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_0052(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1008_d6f4(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1008_caa0(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_d75a(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_d968(u16 *param_1, u8 param_2)

{
    u16 unaff_SS;

    pass1_1008_d7da(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

astruct_11 *pass1_1008_d9d4(astruct_11 *param_1, u8 param_2)

{
    clenaup_win_ui_1018_4d22(param_1, 0x1018);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void pass1_1008_dc2c(u16 *param_1, u16 param_2)

{
    u16 uVar1;

    uVar1           = (param_1 >> 0x10);
    *param_1        = 0xdc80;
    (param_1 + 0x2) = 0x1008;
    fn_ptr_1000_17ce((param_1 + 0x18), 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}


u16 *pass1_1008_dc5a(u16 *param_1, u8 param_2)

{
    u16 unaff_SS;

    pass1_1008_dc2c(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_dd1e(u16 *param_1, u8 param_2)

{
    *param_1        = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void pass1_1008_c626(u32 *param_1)

{
    globals->_PTR_LOOP_1050_06e0 = 0x0;
    fn_ptr_1000_17ce((Struct18 *)*param_1, 0x1000);
    return;
}


u32 pass1_1008_ca24(u32 param_1, u8 param_2, u16 param_3)

{
    pass1_1008_c75c(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u32 pass1_1008_bd02(u32 param_1, u8 param_2)

{
    u16 unaff_SS;

    pass1_1008_afde(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_bd28(u16 *param_1, u8 param_2)

{
    pass1_1008_b08c(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


Struct18 *pass1_1008_bd4e(Struct18 *param_1, u8 param_2)

{
    pass1_1008_b08c(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 *pass1_1008_bd74(Struct18 *param_1, u8 param_2)

{
    pass1_1008_b08c(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 *pass1_1008_bd9a(Struct18 *param_1, u8 param_2)

{
    pass1_1008_b08c(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_ad38(u16 *param_1, u8 param_2)

{
    *param_1        = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u32 pass1_1008_ad64(u32 param_1, u8 param_2)

{
    u16 unaff_SS;

    pass1_1008_a086(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u32 pass1_1008_af56(u32 param_1, u8 param_2)

{
    pass1_1008_af38((astruct_11 *)param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void pass1_1008_b08c(u16 *param_1)

{
    i16 iVar1;
    u16 uVar2;

    uVar2         = (param_1 >> 0x10);
    iVar1         = param_1;
    *param_1      = 0xbdc8;
    (iVar1 + 0x2) = 0x1008;
    fn_ptr_1000_17ce((iVar1 + 0x4), 0x1000);
    *param_1      = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    return;
}


void pass1_1008_b1a6(u32 param_1, char *param_2)

{
    long         lVar1;
    u16          uVar2;
    u16          in_DX;
    astruct_467 *iVar3;
    astruct_466 *iVar4;
    u16          uVar3;
    u16          uVar4;

    uVar3 = (param_1 >> 0x10);
    iVar3 = (astruct_467 *)param_1;
    if(iVar3->field_0x16 != 0x0)
    {
        lVar1 = iVar3->field_0x16;
        fn_ptr_1000_17ce((lVar1 + 0x4), 0x1000);
        uVar2             = str_op_1008_60e8(param_2, in_DX);
        lVar1             = iVar3->field_0x16;
        uVar4             = (lVar1 >> 0x10);
        iVar4             = (astruct_466 *)lVar1;
        iVar4->field_0x4  = uVar2;
        iVar4->field_0x6  = in_DX;
        iVar3->field_0x16 = 0x0;
    }
    return;
}

void pass1_1008_9466(u16 *param_1)

{
    *param_1        = 0x52a;
    (param_1 + 0x2) = 0x1008;
    fn_ptr_1000_17ce(globals->PTR_LOOP_1050_0392, 0x1000);
    globals->PTR_LOOP_1050_0392 = (Struct18 *)0x0;
    return;
}


u16 *pass1_1008_9d02(u16 *param_1, u8 param_2)

{
    *param_1        = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_11 *pass1_1008_9f80(astruct_11 *param_1, u8 param_2)

{
    param_1 = (astruct_11 *)(param_1 & 0xffff0000 | (param_1 - 0x1c));
    pass1_1008_9e5a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u32 pass1_1008_87a2(u32 param_1, u8 param_2)

{
    pass1_1008_8168(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void pass1_1008_8aa2(u16 *param_1)

{
    u32  *puVar1;
    u16          uVar2;
    u16          uVar3;
    code       **ppcVar4;
    u32   uVar5;
    astruct_462 *iVar6;
    u16          uVar6;
    Struct18  *paStack16;

    uVar6            = (param_1 >> 0x10);
    iVar6            = (astruct_462 *)param_1;
    *param_1         = 0x8e9a;
    iVar6->field_0x2 = 0x1008;
    uVar5            = &iVar6->field_0x4;
    if((uVar5 + 0x1c) != 0x0)
    {
        puVar1 = iVar6->field_0x4;
        uVar2  = iVar6->field_0x6;
        if((uVar2 | puVar1) != 0x0)
        {
            ppcVar4 = *puVar1;
            (**ppcVar4)();
        }
    }
    uVar2     = iVar6->field_0x3a;
    uVar3     = iVar6->field_0x3c;
    paStack16 = (Struct18 *)CONCAT22(uVar3, uVar2);
    if((uVar3 | uVar2) != 0x0)
    {
        pass1_1008_5118(CONCAT22(uVar3, uVar2));
        fn_ptr_1000_17ce(paStack16, 0x1000);
    }
    *param_1         = 0x389a;
    iVar6->field_0x2 = 0x1008;
    return;
}


u16 *pass1_1008_8e74(u16 *param_1, u8 param_2)

{
    pass1_1008_8aa2(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void pass1_1008_8f24(u16 *param_1)

{
    u32         *puVar1;
    u32  *puVar2;
    u16          uVar3;
    code       **ppcVar4;
    u32   uVar5;
    astruct_463 *iVar6;
    i16          iVar7;
    i16          iVar8;
    u16          uVar9;
    u16          uVar10;
    u32          uStack6;

    uVar9            = (param_1 >> 0x10);
    iVar6            = (astruct_463 *)param_1;
    *param_1         = 0x9170;
    iVar6->field_0x2 = 0x1008;
    if(iVar6->field_0x1a_addr_offset != 0x0)
    {
        uStack6 = 0x0;
        while(true)
        {
            puVar1 = &iVar6->field_0xa;
            if(*puVar1 < uStack6 || *puVar1 == uStack6)
                break;
            iVar8  = uStack6 * 0x4;
            uVar5  = iVar6->field_0x6;
            uVar10 = (uVar5 >> 0x10);
            iVar7  = uVar5;
            puVar2 = *(iVar7 + iVar8);
            uVar3  = *(iVar7 + iVar8 + 0x2);
            if((uVar3 | puVar2) != 0x0)
            {
                ppcVar4 = *puVar2;
                (**ppcVar4)();
            }
            uStack6 = uStack6 + 0x1;
        }
    }
    fn_ptr_1000_17ce((Struct18 *)iVar6->field_0x6, 0x1000);
    *param_1         = 0x389a;
    iVar6->field_0x2 = 0x1008;
    return;
}


u16 *pass1_1008_914a(u16 *param_1, u8 param_2)

{
    pass1_1008_8f24(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_93c0(u16 *param_1, u8 param_2)

{
    *param_1        = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_93ec(u16 *param_1, u8 param_2)

{
    u16 unaff_CS;

    kill_timer_1008_921c(param_1, unaff_CS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void pass1_1008_766e(u32 param_1, u32 *param_2, u16 param_3, u16 param_4, u8 *param_5)

{
    u32 *puVar1;
    u8  *puVar2;
    u32  local_6;

    *param_2 = 0x0;
    local_6  = 0x0;
    puVar1   = &local_6;
    file_1008_76e4(param_1, (long *)CONCAT22(param_3, puVar1), param_4, param_3, param_5);
    if(puVar1 != 0x0)
    {
        if(local_6 != 0x0)
        {
            mem_op_1000_179c(0xc, param_5, 0x1000);
            puVar2 = (param_5 | puVar1);
            if(puVar2 == 0x0)
            {
                puVar1 = 0x0;
                puVar2 = 0x0;
            }
            else
            {
                pass1_1010_8ef2(CONCAT22(param_5, puVar1), puVar2, param_3);
            }
            param_2 = puVar1;
            (param_2 + 0x2)  = puVar2;
            fn_ptr_1010_905e(*param_2, local_6);
        }
        return;
    }
    return;
}

void pass1_1008_7ffa(u16 *param_1, u8 param_2)

{
    astruct_461 *uVar1;
    u16          uVar2;

    uVar1 = (astruct_461 *)param_1;
    uVar1 = uVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    uVar2            = (param_1 >> 0x10);
    *param_1         = 0x380a;
    uVar1->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    uVar1->field_0x2 = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return;
}


void pass1_1008_6732(u16 *param_1)

{
    long         lVar1;
    astruct_457 *iVar2;
    u16          uVar2;

    uVar2            = (param_1 >> 0x10);
    iVar2            = (astruct_457 *)param_1;
    *param_1         = 0x685a;
    iVar2->field_0x2 = 0x1008;
    if(iVar2->field_0x10 != 0x0)
    {
        lVar1 = iVar2->field_0x10;
        call_fn_ptr_1000_0dc6(lVar1, (lVar1 >> 0x10), 0x1000);
    }
    iVar2->field_0x10 = 0x0;
    pass1_1008_41bc(param_1);
    return;
}


u32 pass1_1008_6834(u32 param_1, u8 param_2)

{
    pass1_1008_6732(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_6b5a(u16 *param_1, u8 param_2)

{
    u32  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    astruct_458 *uVar4;
    u16          uVar5;

    uVar5            = (param_1 >> 0x10);
    uVar4            = (astruct_458 *)param_1;
    *param_1         = 0x6c8c;
    uVar4->field_0x2 = 0x1008;
    puVar1           = uVar4->field_0x4;
    uVar2            = uVar4->field_0x6;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    *param_1         = 0x389a;
    uVar4->field_0x2 = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void pass1_1008_6bb4(u16 *param_1, u8 param_2)

{
    astruct_459 *uVar1;
    u16          uVar2;

    uVar1 = (astruct_459 *)param_1;
    uVar1 = uVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    uVar2            = (param_1 >> 0x10);
    *param_1         = 0x380a;
    uVar1->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    uVar1->field_0x2 = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return;
}


u16 *pass1_1008_5b9a(u16 *param_1, u8 param_2)

{
    pass1_1008_57c4(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u32 pass1_1008_5fa2(u32 param_1, u8 param_2)

{
    pass1_1008_5c34(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void pass1_1008_6330(u16 *param_1, u8 param_2)

{
    astruct_456 *uVar1;
    u16          uVar2;

    uVar1 = (astruct_456 *)param_1;
    uVar1 = uVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    uVar2            = (param_1 >> 0x10);
    *param_1         = 0x380a;
    uVar1->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    uVar1->field_0x2 = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return;
}


void pass1_1008_4cdc(u16 *param_1)

{
    astruct_454 *iVar2;
    u16          uVar1;

    uVar1            = (param_1 >> 0x10);
    iVar2            = (astruct_454 *)param_1;
    *param_1         = 0x4f1c;
    iVar2->field_0x2 = 0x1008;
    fn_ptr_1000_17ce((Struct18 *)iVar2->field_0xe, 0x1000);
    if(iVar2->field_0x12 != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)iVar2->field_0x4, 0x1000);
    }
    *param_1         = 0x389a;
    iVar2->field_0x2 = 0x1008;
    return;
}


u16 *pass1_1008_4ef6(u16 *param_1, u8 param_2)

{
    pass1_1008_4cdc(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_507c(u16 *param_1, u8 param_2)

{
    pass1_1008_41bc(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void pass1_1008_5118(u32 param_1)

{
    u32 uVar1;
    u16        uVar2;

    uVar2 = (param_1 >> 0x10);
    if((param_1 + 0x10) != 0x0)
    {
        uVar1 = (param_1 + 0x10);
        call_fn_ptr_1000_0dc6(uVar1, (uVar1 >> 0x10), 0x1000);
    }
    return;
}


u16 *pass1_1008_570e(u16 *param_1, u8 param_2)

{
    *param_1        = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass_1008_3d44(u16 *param_1, u8 param_2)

{
    astruct_453 *uVar1;
    u16          uVar2;

    uVar2            = (param_1 >> 0x10);
    uVar1            = (astruct_453 *)param_1;
    *param_1         = 0x380a;
    uVar1->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    uVar1->field_0x2 = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void pass1_1008_41bc(u16 *param_1)

{
    u32  *puVar1;
    u16          uVar2;
    long         lVar3;
    code       **ppcVar4;
    astruct_288 *iVar5;
    u16          uVar5;

    uVar5            = (param_1 >> 0x10);
    iVar5            = (astruct_288 *)param_1;
    *param_1         = &PTR_LOOP_1050_48de;
    iVar5->field_0x2 = 0x1008;
    puVar1           = iVar5->field_0xa;
    uVar2            = iVar5->field_0xc;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    if(iVar5->field_0x6 != 0x0)
    {
        lVar3 = iVar5->field_0x6;
        call_fn_ptr_1000_0dc6(lVar3, (lVar3 >> 0x10), 0x1000);
    }
    *param_1         = 0x389a;
    iVar5->field_0x2 = 0x1008;
    return;
}


u16 *pass1_1008_48b8(u16 *param_1, u8 param_2)

{
    pass1_1008_41bc(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_377e(u16 *param_1, u8 param_2)

{
    *param_1        = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_37aa(u16 *param_1, u8 param_2)

{
    astruct_450 *uVar1;
    u16          uVar2;

    uVar2            = (param_1 >> 0x10);
    uVar1            = (astruct_450 *)param_1;
    *param_1         = 0x380a;
    uVar1->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    uVar1->field_0x2 = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u32 pass1_1008_37e4(u32 param_1, u8 param_2)

{
    cleanup_ui_op_1008_0618(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_3a14(u16 *param_1, u8 param_2)

{
    *param_1        = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_3a40(u16 *param_1, u8 param_2)

{
    astruct_451 *uVar1;
    u16          uVar2;

    uVar2            = (param_1 >> 0x10);
    uVar1            = (astruct_451 *)param_1;
    *param_1         = 0x3ab0;
    uVar1->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    uVar1->field_0x2 = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u32 pass1_1008_3a7a(u32 param_1, u8 param_2)

{
    pass1_1008_397a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void pass1_1008_3afe(u16 *param_1, u8 param_2)

{
    i16 iVar1;
    u16 uVar2;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2         = (param_1 >> 0x10);
    *param_1      = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1      = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return;
}

void pass1_1008_0036(u16 *param_1, u16 param_2)

{
    u16          uVar1;
    u32  *puVar2;
    Struct18  *paVar3;
    code       **ppcVar4;
    u16          uVar5;
    astruct_449 *iVar6;
    u16          uVar6;

    uVar6            = (param_1 >> 0x10);
    iVar6            = (astruct_449 *)param_1;
    *param_1         = 0x51e;
    iVar6->field_0x2 = 0x1008;
    paVar3           = &iVar6->field_0x8;
    uVar1            = iVar6->field_0xa;
    uVar5            = paVar3;
    if((uVar1 | uVar5) != 0x0)
    {
        pass1_1008_53aa(uVar5, uVar1);
        param_2 = 0x1000;
        fn_ptr_1000_17ce(paVar3, 0x1000);
    }
    paVar3                       = globals->_PTR_LOOP_1050_5748;
    globals->_PTR_LOOP_1050_0298 = 0x0;
    if(_PTR_LOOP_1050_5748 != (Struct18 *)0x0)
    {
        pass1_1030_8210(&_PTR_LOOP_1050_5748->field_0x0);
        param_2 = 0x1000;
        fn_ptr_1000_17ce(paVar3, 0x1000);
    }
    paVar3 = globals->_PTR_LOOP_1050_0ed0;
    if(_PTR_LOOP_1050_0ed0 != (Struct18 *)0x0)
    {
        pass1_1010_2050(_PTR_LOOP_1050_0ed0);
        param_2 = 0x1000;
        fn_ptr_1000_17ce(paVar3, 0x1000);
    }
    paVar3 = globals->_PTR_LOOP_1050_14cc;
    if(_PTR_LOOP_1050_14cc != (Struct18 *)0x0)
    {
        pass1_1010_7efc(_PTR_LOOP_1050_14cc, 0x1010);
        param_2 = 0x1000;
        fn_ptr_1000_17ce(paVar3, 0x1000);
    }
    paVar3 = globals->_PTR_LOOP_1050_5b7c;
    if(_PTR_LOOP_1050_5b7c != (Struct18 *)0x0)
    {
        pass1_1038_af34();
        param_2 = 0x1000;
        fn_ptr_1000_17ce(paVar3, 0x1000);
    }
    if(_PTR_LOOP_1050_5bc8 != 0x0)
    {
        ppcVar4 = *_PTR_LOOP_1050_5bc8;
        (**ppcVar4)(param_2, globals->_PTR_LOOP_1050_5bc8, (_PTR_LOOP_1050_5bc8 >> 0x10), 0x1);
    }
    if(_PTR_LOOP_1050_02a0 != 0x0)
    {
        ppcVar4 = *_PTR_LOOP_1050_02a0;
        (**ppcVar4)(param_2, globals->_PTR_LOOP_1050_02a0, (_PTR_LOOP_1050_02a0 >> 0x10), 0x1);
    }
    puVar2 = iVar6->field_0x4;
    uVar1  = iVar6->field_0x6;
    if((uVar1 | puVar2) != 0x0)
    {
        ppcVar4 = *puVar2;
        (**ppcVar4)(param_2, puVar2, uVar1, 0x1);
    }
    pass1_1008_9466(param_1);
    return;
}


u16 *pass1_1008_04d2(u16 *param_1, u8 param_2)

{
    pass1_1008_9466(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u16 *pass1_1008_04f8(u16 *param_1, u8 param_2, u16 param_3)

{
    pass1_1008_0036(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void fn_ptr_op_1000_24cd(u16 param_1, i16 param_2)

{
    swi_0x21_fn_ptr fn_ptr_1;
    i16             iVar2;
    u16             uVar6;
    char            cVar7;
    u16             uVar5;
    u16             uVar3;
    u16             uVar4;

    iVar2                             = param_2 + 0x1;
    uVar5                             = &USHORT_1050_1050;
    globals->PTR_LOOP_1050_5fc9._0_1_ = 0x0;
    uVar6                             = 0x0;
    fn_ptr_op_1000_2594(0x68b6, 0x68b6);
    fn_ptr_op_1000_2594(&PTR_LOOP_1050_6210, 0x620c);
    ret_op_1000_55ac(param_1, uVar6, uVar5, iVar2);
    cVar7 = (uVar6 >> 0x8);
    fn_ptr_op_1000_2594(&PTR_LOOP_1050_6210, &PTR_LOOP_1050_6210);
    fn_ptr_op_1000_2594(&PTR_LOOP_1050_6210, &PTR_LOOP_1050_6210);
    dos3_op_1000_256b();
    if(cVar7 == '\0')
    {
        fn_ptr_1 = swi_0x21();
        fn_ptr_1(0);
    }
}


void pass1_1000_24db(u16 param_1, u16 param_2, Globals *globals)

{
    code *pcVar1;
    i16   iVar2;
    u16   uVar3;
    u16   uVar4;

    iVar2                             = param_2 + 0x1;
    uVar4                             = SUB42(&globals->USHORT_1050_1050, 0x0);
    globals->PTR_LOOP_1050_5fc9 = 0x0;
    uVar3                             = 0x1;
    fn_ptr_op_1000_2594(&globals->PTR_LOOP_1050_6210, &globals->PTR_LOOP_1050_6210);
    fn_ptr_op_1000_2594(&globals->PTR_LOOP_1050_6210, &globals->PTR_LOOP_1050_6210);
    dos3_op_1000_256b(uVar3, uVar4, iVar2);
    if((uVar3 >> 0x8) == '\0')
    {
        pcVar1 = (fn_ptr_1)swi(0x21);
        (*pcVar1)();
    }
    return;
}


void fn_ptr_op_1000_2594(fn_ptr_1 *param_1, fn_ptr_1 *param_2)

{
    fn_ptr_1 *ppcVar1;
    fn_ptr_1 *ppcVar2;
    fn_ptr_1 *fn_ptr_1;

    while(param_2 < param_1)
    {
        ppcVar2 = param_1 + -0x2;
        ppcVar1 = param_1 + -0x1;
        param_1 = ppcVar2;
        if((*ppcVar2 | *ppcVar1) != 0x0)
        {
            fn_ptr_1 = ppcVar2;
            (**fn_ptr_1)();
        }
    }
    return;
}


BOOL16 call_fn_ptr_1000_0dc6(u16 param_1, u16 param_2, u16 param_3)

{
    if((*&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0)
    {
        pass1_1000_1e61(param_3, 0xe, 0x0, 0x0);
        return 0x0;
    }
    (**0x8)(&USHORT_1050_1050);
    return 0x1;
}


void pass1_1000_16ee(u16 param_1, u16 param_2, u16 param_3)

{
    if((param_2 | param_1) != 0x0)
    {
        call_fn_ptr_1000_0dc6(param_1, param_2, param_3);
    }
    return;
}


u16 fn_ptr_op_1000_1708(u16 param_1, u16 param_2, u16 param_3, u16 param_4, u16 param_5, u16 param_6)

{
    i16  iVar1;
    bool bVar2;
    long lVar3;

    if((param_2 | param_1) == 0x0)
    {
        bVar2   = 0xfffe < param_1;
        param_1 = param_1 + 0x1;
        param_2 = param_2 + bVar2;
    }
LAB_1000_1724:
    do
    {
        if((param_5 | param_4) != 0x0)
        {
            lVar3 = mem_op_1000_0a48((u8)param_3, param_1, param_2, CONCAT22(param_5, param_4), param_6);
            if(lVar3 != 0x0)
            {
                return lVar3;
            }
        }
        if(((param_3 & 0x8000) == 0x0) || ((PTR_LOOP_1050_5f3a | globals->PTR_LOOP_1050_5f38) == 0x0))
        {
            if((PTR_LOOP_1050_5f36 | globals->PTR_LOOP_1050_5f34) == 0x0)
            {
                if((PTR_LOOP_1050_5f3e | globals->PTR_LOOP_1050_5f3c) == 0x0)
                {
                    return 0x0;
                }
                (*(fn_ptr_1)PTR_LOOP_1050_5f3c)();
                goto LAB_1000_1724;
            }
            iVar1 = (*(fn_ptr_1)PTR_LOOP_1050_5f34)();
        }
        else
        {
            iVar1 = (*(fn_ptr_1)PTR_LOOP_1050_5f38)(param_6, param_1);
        }
        if(iVar1 == 0x0)
        {
            return 0x0;
        }
    } while(true);
}


void mem_op_1000_179c(u16 param_1, u8 *param_2, u16 param_3)

{
    u8 *puVar1;
    u8 *puVar2;

    puVar1 = globals->PTR_LOOP_1050_5f2c;
    puVar2 = globals->PTR_LOOP_1050_5f2e;
    if((PTR_LOOP_1050_5f2e | globals->PTR_LOOP_1050_5f2c) == 0x0)
    {
        puVar1 = mem_op_1000_160a(param_2, param_3);
        puVar2 = param_2;
    }
    fn_ptr_op_1000_1708(param_1, 0x0, 0x0, puVar1, puVar2, param_3);
    return;
}


void pass1_1000_18d2(u16 param_1, u16 param_2, u16 param_3)

{
    if((param_2 | param_1) != 0x0)
    {
        call_fn_ptr_1000_0dc6(param_1, param_2, param_3);
    }
    return;
}
