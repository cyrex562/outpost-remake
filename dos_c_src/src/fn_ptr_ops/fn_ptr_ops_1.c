#include "fn_ptr_ops_1.h"
#include "types.h"
#include "structs_1.h"
#include "fn_ptr_ops_6.h"
#include "fn_ptr_ops_7.h"

Struct18 * pass1_1040_d89e(Struct18 *param_1, u8 param_2)

{
    pass1_1040_d1bc(globals, param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_d056(Struct18 *param_1, u8 param_2)

{
    pass1_1040_ca74(globals, param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_af9e(Struct18 *param_1, u8 param_2)

{
    pass1_1040_ace8(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void  pass1_1040_a5d0(u32 param_1)

{
    u16          uVar1;
    u16          uVar2;
    astruct_258 *iVar4;
    u16          uVar3;

    uVar3 = (param_1 >> 0x10);
    iVar4 = (astruct_258 *)param_1;
    uVar1 = iVar4->field_0x2;
    uVar2 = iVar4->field_0x4;
    if((uVar2 | uVar1) != 0x0)
    {
        pass1_1000_54e8(0xa582, &PTR_LOOP_1050_1040, (uVar1 - 0x2), 0xa, uVar1, uVar2);
        fn_ptr_1000_17ce((Struct18 *)CONCAT22(uVar2, uVar1 - 0x2), 0x1000);
    }
    fn_ptr_1000_17ce((Struct18 *)iVar4->field_0xc, 0x1000);
    return;
}

void  pass1_1040_a582(u32 *param_1)

{
    fn_ptr_1000_17ce((Struct18 *)*param_1, 0x1000);
    return;
}

Struct18 * pass1_1040_8db6(Struct18 *param_1, u8 param_2)

{
    pass1_1040_869a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_8f16(Struct18 *param_1, u8 param_2)

{
    pass1_1040_8e82(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

u16  pass1_1040_746c(u32 param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = ((param_1 + 0x98) + 0x8);
    (**ppcVar1)();
    return 0x1;
}

void  pass1_1040_692e(u32 *param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = (*param_1 + 0x7c);
    (**ppcVar1)();
    return;
}

u16  pass1_1040_6cfa(u32 param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = ((param_1 + 0x98) + 0x8);
    (**ppcVar1)();
    return 0x1;
}

Struct18 * pass1_1040_6f0c(Struct18 *param_1, u8 param_2)

{
    pass1_1040_6862(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void  pass1_1040_70a0(u32 *param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = (*param_1 + 0x7c);
    (**ppcVar1)();
    return;
}

void  pass1_1040_4f82(u32 *param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = (*param_1 + 0x80);
    (**ppcVar1)();
    return;
}

u16  pass1_1040_5238(u32 param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = ((param_1 + 0x94) + 0x8);
    (**ppcVar1)();
    return 0x0;
}

Struct18 * pass1_1040_557c(Struct18 *param_1, u8 param_2)

{
    pass1_1040_4f0a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_38d4(Struct18 *param_1, u8 param_2)

{
    pass1_1040_3506(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_3fd6(Struct18 *param_1, u8 param_2)

{
    pass1_1040_39e2(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_4440(Struct18 *param_1, u8 param_2)

{
    pass1_1040_40e2(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void  pass1_1040_4766(u32 *param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = (*param_1 + 0x74);
    (**ppcVar1)();
    return;
}

Struct18 * pass1_1040_2358(Struct18 *param_1, u8 param_2)

{
    pass1_1040_205e(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_2930(Struct18 *param_1, u8 param_2)

{
    pass1_1040_2464(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_2e00(Struct18 *param_1, u8 param_2)

{
    pass1_1040_2a22(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_3410(Struct18 *param_1, u8 param_2)

{
    pass1_1040_2f06(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_11ac(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1040_0e86(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_178a(Struct18 *param_1, u8 param_2)

{
    pass1_1040_1290(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_1c22(Struct18 *param_1, u8 param_2)

{
    pass1_1040_1876(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_1ec8(Struct18 *param_1, u8 param_2)

{
    pass1_1040_1d24(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1038_ee48(Struct18 *param_1, u8 param_2)

{
    pass1_1038_ebd6(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_31 * pass1_1040_0656(astruct_31 *param_1, u8 param_2)

{
    destroy_win_1038_ef3a(param_1, &PTR_LOOP_1050_1038);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_0b6a(Struct18 *param_1, u8 param_2)

{
    pass1_1040_073a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1040_0d8a(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1040_0c54(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1038_e0ae(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1038_d7d0(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1038_e23e(Struct18 *param_1, u8 param_2)

{
    pass1_1038_e16e(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1038_e608(Struct18 *param_1, u8 param_2)

{
    pass1_1038_e308(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1038_e908(Struct18 *param_1, u8 param_2)

{
    pass1_1038_e6f0(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1038_eb0c(Struct18 *param_1, u8 param_2)

{
    pass1_1038_e9ec(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1038_ca46(Struct18 *param_1, u8 param_2)

{
    pass1_1038_c80a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1038_cc74(Struct18 *param_1, u8 param_2)

{
    pass1_1038_cb30(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_ceda(Struct18 *param_1, u8 param_2)

{
    pass1_1038_cd5c(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_d218(Struct18 *param_1, u8 param_2, u16 param_3)

{
    free_proc_inst_1038_cfda(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_d6c4(Struct18 *param_1, u8 param_2)

{
    pass1_1038_d276(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_bd4a(Struct18 *param_1, u8 param_2)

{
    pass1_1038_b7f0(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_c410(Struct18 *param_1, u8 param_2)

{
    pass1_1038_be4a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_c726(Struct18 *param_1, u8 param_2)

{
    pass1_1038_c4fe(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_9ed4(Struct18 *param_1, u8 param_2)

{
    unk_draw_op_1040_b0f8(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_a090(Struct18 *param_1, u8 param_2)

{
    pass1_1038_9fa4(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


{
    pass1_1038_a156(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_a402(Struct18 *param_1, u8 param_2)

{
    pass1_1038_a36a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_a608(Struct18 *param_1, u8 param_2)

{
    pass1_1038_a4c2(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_a80c(Struct18 *param_1, u8 param_2)

{
    pass1_1038_a6c8(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_aaf0(Struct18 *param_1, u8 param_2)

{
    pass1_1038_a8cc(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_ad4c(Struct18 *param_1, u8 param_2)

{
    pass1_1038_abb0(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_ae28(Struct18 *param_1, u8 param_2)

{
    pass1_1038_ae08(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_8c08(Struct18 *param_1, u8 param_2)

{
    pass1_1038_893a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_90a2(Struct18 *param_1, u8 param_2)

{
    pass1_1038_8cf6(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1038_927c(u32 *param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = (*param_1 + 0x74);
    (**ppcVar1)();
    return;
}


u32  pass1_1038_9ad0(u32 param_1, u8 param_2)

{
    pass1_1038_9a48((Struct18 *)param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_78b8(Struct18 *param_1, u8 param_2)

{
    pass1_1038_6912(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1038_7964(u16 *param_1)

{
    u16         uVar1;
    u32 *puVar2;
    code      **ppcVar3;
    i16         iVar4;
    u16         uVar5;

    globals->_PTR_LOOP_1050_5a64 = 0x0;
    uVar5               = (param_1 >> 0x10);
    iVar4               = param_1;
    uVar1               = (iVar4 + 0x2);
    if((uVar1 | *param_1) != 0x0)
    {
        ppcVar3 = *param_1;
        (**ppcVar3)();
    }
    puVar2 = (iVar4 + 0x4);
    uVar1  = (iVar4 + 0x6);
    if((uVar1 | puVar2) != 0x0)
    {
        ppcVar3 = *puVar2;
        (**ppcVar3)();
    }
    return;
}


void  pass1_1038_7a5a(u32 *param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = (*param_1 + 0x4);
    (**ppcVar1)();
    return;
}


Struct18 * pass1_1038_8850(Struct18 *param_1, u8 param_2)

{
    pass1_1038_7d5c(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1038_64de(Struct18 *param_1, u8 param_2)

{
    pass1_1038_33f8(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1038_6912(u16 *param_1)

{
    u16         uVar1;
    u16         uVar2;
    code      **ppcVar3;
    u32 *puVar4;
    i16         iVar5;
    u16         uVar6;
    Struct18 *paStack10;

    uVar6         = (param_1 >> 0x10);
    iVar5         = param_1;
    *param_1      = 0x78de;
    (iVar5 + 0x2) = &PTR_LOOP_1050_1038;
    uVar1         = (iVar5 + 0x6);
    puVar4        = (iVar5 + 0x4);
    if((uVar1 | puVar4) != 0x0)
    {
        ppcVar3 = *puVar4;
        (**ppcVar3)();
    }
    uVar1     = (iVar5 + 0xe);
    uVar2     = (iVar5 + 0x10);
    paStack10 = (Struct18 *)CONCAT22(uVar2, uVar1);
    if((uVar2 | uVar1) != 0x0)
    {
        fn_ptr_1020_ba7e(CONCAT22(uVar2, uVar1));
        fn_ptr_1000_17ce(paStack10, 0x1000);
    }
    *param_1      = 0x389a;
    (iVar5 + 0x2) = 0x1008;
    return;
}


void  pass1_1038_5a16(u16 param_1, u16 param_2, u32 param_3, u32 param_4)

{
    code     **ppcVar1;
    u32 uVar2;
    BOOL16     BVar3;
    i16        iVar4;
    u16        uVar5;
    u32        uStack6;

    for(uStack6 = 0x0; uStack6 < param_3; uStack6 = uStack6 + 0x1)
    {
        uVar5 = (param_4 >> 0x10);
        iVar4 = param_4;
        if(((uStack6 * 0x4 + iVar4) != 0x0) && (uVar2 = (uStack6 * 0x4 + iVar4), BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar2 + 0xc), 0x2f), BVar3 != 0x0))
        {
            uVar2          = (uStack6 * 0x4 + iVar4);
            (uVar2 + 0x1a) = 0x3;
            ppcVar1        = ((uStack6 * 0x4 + iVar4) + 0x28);
            (**ppcVar1)();
        }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1038_5a96(u16 param_1, u16 param_2, u32 param_3, u32 param_4)

{
    code     **ppcVar1;
    u32 uVar2;
    BOOL16     BVar3;
    i16        iVar4;
    u16        uVar5;
    u32        uStack6;

    for(uStack6 = 0x0; uStack6 < param_3; uStack6 = uStack6 + 0x1)
    {
        uVar5 = (param_4 >> 0x10);
        iVar4 = param_4;
        if(((uStack6 * 0x4 + iVar4) != 0x0) && (uVar2 = (uStack6 * 0x4 + iVar4), BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar2 + 0xc), 0x2c), BVar3 != 0x0))
        {
            ppcVar1 = ((uStack6 * 0x4 + iVar4) + 0x54);
            (**ppcVar1)();
            if(BVar3 != 0x0)
            {
                uVar2          = (iVar4 + uStack6 * 0x4);
                (uVar2 + 0x1a) = 0x3;
                ppcVar1        = ((uStack6 * 0x4 + iVar4) + 0x28);
                (**ppcVar1)();
                uVar2          = (iVar4 + uStack6 * 0x4);
                (uVar2 + 0x1a) = 0x2;
            }
        }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1038_5b3c(u16 param_1, u16 param_2, u32 param_3, u32 param_4)

{
    code     **ppcVar1;
    u32 uVar2;
    u32 uVar3;
    BOOL16     BVar4;
    i16        iVar5;
    u16        uVar6;
    u32        uStack6;

    for(uStack6 = 0x0; uStack6 < param_3; uStack6 = uStack6 + 0x1)
    {
        uVar6 = (param_4 >> 0x10);
        iVar5 = param_4;
        if((((uStack6 * 0x4 + iVar5) != 0x0) && (uVar2 = (uStack6 * 0x4 + iVar5), BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar2 + 0xc), 0x2d), BVar4 != 0x0))
           && (ppcVar1 = ((uStack6 * 0x4 + iVar5) + 0x50), (**ppcVar1)(), BVar4 != 0x0))
        {
            uVar2          = (uStack6 * 0x4 + iVar5);
            uVar3          = (uStack6 * 0x4 + iVar5);
            (uVar3 + 0x1a) = (uVar2 + 0x1a) | 0x1;
            ppcVar1        = ((uStack6 * 0x4 + iVar5) + 0x28);
            (**ppcVar1)();
        }
    }
    return;
}


void  pass1_1038_582c(u32 param_1, u32 param_2)

{
    u32 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    i16         iVar4;
    u16         uVar5;

    uVar5  = (param_1 >> 0x10);
    iVar4  = param_1;
    puVar1 = (iVar4 + 0x14);
    uVar2  = (iVar4 + 0x16);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    *(iVar4 + 0x14) = param_2;
    return;
}


void  pass1_1038_5860(u32 param_1, u16 param_2, u32 param_3, i16 param_4)

{
    code **ppcVar1;
    u32    uVar2;
    u32    uVar3;
    u16    extraout_DX;
    i16    extraout_DX_00;
    i16    iVar4;
    u16    uVar5;
    u32    uStack14;
    i16    iStack6;
    i16    iStack4;

    if(param_4 == 0x0)
    {
        uVar5   = (param_1 >> 0x10);
        iVar4   = param_1;
        ppcVar1 = ((iVar4 + 0xc) + 0x10);
        uVar2   = param_3;
        (**ppcVar1)();
        uVar2 = uVar2 & 0xffff | extraout_DX << 0x10;
        for(uStack14 = 0x0; uStack14 < uVar2; uStack14 = uStack14 + 0x1)
        {
            ppcVar1 = ((iVar4 + 0xc) + 0x4);
            uVar3   = uVar2;
            (**ppcVar1)();
            iStack6 = param_3;
            if((uVar3 == iStack6) && (iStack4 = (param_3 >> 0x10), extraout_DX_00 == iStack4))
            {
                return;
            }
        }
        ppcVar1 = ((iVar4 + 0xc) + 0xc);
        (**ppcVar1)();
    }
    return;
}


void  pass1_1038_4918(u32 param_1, i16 param_2, u16 param_3, u16 param_4, u8 param_5)

{
    i16       *piVar1;
    u32 uVar2;
    i16        iVar3;
    u32       *puVar4;
    u16        uVar5;
    u16        uVar6;
    i16        iVar7;
    i16        iVar8;
    u16        uVar9;
    u16        uVar10;
    u32        uVar11;
    u8       bStack347;
    u8         local_14a[0x4];
    u32       *puStack326;
    u8         local_144[0x124];
    u32        local_20;
    u16        uStack28;
    u32        uStack26;
    u32        uStack18;
    u32        uStack14;
    u32 uStack10;
    u32 uStack6;

    uVar9 = (param_1 >> 0x10);
    iVar7 = param_1;
    if((iVar7 + 0x4) != 0x4000001)
    {
        return;
    }
    uVar2 = (iVar7 + 0x8);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
    uStack6  = CONCAT22(param_3, param_2);
    uStack10 = (param_2 + 0x10);
    uVar10   = (uStack10 >> 0x10);
    iVar8    = uStack10;
    if((iVar8 + 0x1c) == 0x0)
    {
        return;
    }
    uStack14 = 0x0;
    switch((iVar7 + 0x20e))
    {
    case 0x1:
        uStack14._0_2_ = 0x1e;
        break;
    case 0x2:
        uStack14._0_2_ = 0x1c;
        break;
    case 0x3:
        uStack14._0_2_ = 0x1a;
        break;
    case 0x4:
        uStack14._0_2_ = 0x18;
        break;
    case 0x5:
        uStack14._0_2_ = 0x16;
        break;
    case 0x6:
        uStack14._0_2_ = 0x14;
        break;
    case 0x7:
        uStack14._0_2_ = 0x12;
        break;
    case 0x8:
        uStack14._0_2_ = 0x10;
        break;
    case 0x9:
        uStack14._0_2_ = 0xe;
        break;
    case 0xa:
        uStack14._0_2_ = 0xc;
        break;
    default:
        goto switchD_1038_49cf_caseD_a;
    }
    uStack14 = uStack14;
switchD_1038_49cf_caseD_a:
    uStack18 = *_PTR_LOOP_1050_65e2;
    if((uStack14 != 0x0) && (((uStack18 & 0xffff | (_PTR_LOOP_1050_65e2 + 0x2) << 0x10) % uStack14) == 0x0))
    {
        piVar1     = (iVar8 + 0x1c);
        *piVar1    = *piVar1 + -0x1;
        piVar1     = (iVar8 + 0x1a);
        *piVar1    = *piVar1 + 0x1;
        iVar3      = (iVar8 + 0x1a) * 0x6 + (iVar8 + 0x16);
        uVar10     = (iVar8 + 0x18);
        local_20   = *(iVar3 + -0x6);
        uStack28   = (iVar3 + -0x2);
        puStack326 = &local_20;
        puVar4     = &local_20;
        pass1_1030_64ce(param_4, puVar4, uVar10, globals->_PTR_LOOP_1050_5740, CONCAT22(param_4, puVar4), (iVar7 + 0x8), CONCAT22(param_4, local_14a));
        uStack26  = *puVar4;
        uVar6     = (puVar4 + 0x2);
        bStack347 = (u8)(uStack26 >> 0x18);
        uVar5     = bStack347;
        if(bStack347 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack26, uVar6);
            uVar11 = struct_op_1030_73a8(CONCAT22(uVar6, uVar5));
            uVar6  = (uVar11 >> 0x10);
            if((uVar6 | uVar11) != 0x0)
            {
                iVar8 = (uVar11 + 0xc);
                if(iVar8 < 0x1)
                {
                    return;
                }
                if(SBORROW2(iVar8, 0x1))
                {
                    return;
                }
                if(0x8 < iVar8 + -0x1)
                {
                    return;
                }
            }
        }
        struct_op_1028_87f0(param_4, param_5, (astruct_97 *)CONCAT22(param_4, local_144), 0x0, 0x0, 0x10, &local_20, param_4, *(iVar7 + 0x4), *(iVar7 + 0x8));
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_4, local_144));
    }
    return;
}


void  pass1_1038_3608(u32 param_1)

{
    u16 uVar1;

    uVar1 = (param_1 >> 0x10);
    fn_ptr_1000_17ce(*(Struct18 **)(param_1 + 0x21a), 0x1000);
    (param_1 + 0x21a) = 0x0;
    return;
}


void  pass1_1038_3efc(u16 param_1, u16 param_2, u32 param_3, u32 param_4, i16 param_5, u16 param_6)

{
    code      **ppcVar1;
    u32 *puStack6;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_4, (param_4 >> 0x10));
    puStack6         = CONCAT22(param_6, param_5);
    (param_5 + 0x1c) = (param_3 + 0x4);
    ppcVar1          = (*puStack6 + 0x58);
    (**ppcVar1)(&USHORT_1050_1028, param_5, param_6, param_3);
    return;
}


void  pass1_1038_3f38(u32 *param_1, u32 *param_2, u32 param_3, i16 param_4, u16 param_5)

{
    code      **ppcVar1;
    i16         iVar2;
    u16         extraout_DX;
    u16         extraout_DX_00;
    u32        *puVar3;
    u16         uVar4;
    u32  uVar5;
    u16         uVar6;
    u32         uStack10;
    u32 *puStack6;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_3, (param_3 >> 0x10));
    puStack6 = CONCAT22(param_5, param_4);
    iVar2    = param_4;
    pass1_1028_b58e(CONCAT22(param_5, param_4));
    uStack10 = CONCAT22(extraout_DX, iVar2);
    uVar5    = (iVar2 + 0x4);
    ppcVar1  = (*param_1 + 0x18);
    (**ppcVar1)(&USHORT_1050_1028, param_1, uVar5);
    uVar6   = 0x0;
    uVar4   = 0x0;
    ppcVar1 = (*param_2 + 0x8);
    puVar3  = param_2;
    (**ppcVar1)();
    pass1_1030_73ee(uStack10, *(param_2 + 0x4), extraout_DX_00);
    ppcVar1 = (*puStack6 + 0x58);
    (**ppcVar1)(0x1030, param_4, param_5, param_2, puVar3, uVar4, uVar5, uVar6);
    return;
}


void  pass1_1038_3fca(u32 param_1, u16 param_2, u16 param_3)

{
    u32  uVar1;
    code      **ppcVar2;
    u16         uVar3;
    u16         extraout_DX;
    u16         uVar4;
    u16         extraout_DX_00;
    u16         uVar5;
    i16         iVar6;
    i16         unaff_DI;
    u16         uVar7;
    u16         uVar8;
    u16         uVar9;
    u32         uVar10;
    u16        *puVar11;
    u8          uVar12;
    u8          uVar13;
    u8          uVar14;
    u8          uVar15;
    u16         uVar16;
    i16         iStack38;
    i16         local_24;
    u8          local_22[0x2];
    i16        *piStack32;
    u16         uStack30;
    u8         *puStack28;
    u16         uStack26;
    u16         uStack24;
    u32         uStack22;
    u16         uStack18;
    u16         uStack16;
    Struct18 *paStack14;
    Struct18 *paStack10;
    u32         uStack6;

    uVar7 = (param_1 >> 0x10);
    uVar5 = param_1;
    if((uVar5 + 0xc) == 0x0)
    {
        param_2 = 0x0;
        uVar4   = 0x0;
    }
    else
    {
        ppcVar2 = ((uVar5 + 0xc) + 0x10);
        (**ppcVar2)();
        uVar4 = extraout_DX;
    }
    uStack6            = CONCAT22(uVar4, param_2);
    globals->PTR_LOOP_1050_5f2e = (uVar4 | param_2);
    if(PTR_LOOP_1050_5f2e != 0x0)
    {
        if(_PTR_LOOP_1050_5f2c == 0x0)
        {
            globals->PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e, 0x1000);
        }
        else
        {
        }
        uVar3     = fn_ptr_op_1000_1708(uStack6 << 0x2, 0x0, 0x1, globals->PTR_LOOP_1050_5f2c, globals->PTR_LOOP_1050_5f2e, 0x1000);
        paStack10 = (Struct18 *)CONCAT22(PTR_LOOP_1050_5f2e, uVar3);
        if(_PTR_LOOP_1050_5f2c == 0x0)
        {
            globals->PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e, 0x1000);
        }
        else
        {
        }
        uVar9     = 0x1000;
        uVar3     = fn_ptr_op_1000_1708(uStack6 << 0x2, 0x0, 0x1, globals->PTR_LOOP_1050_5f2c, globals->PTR_LOOP_1050_5f2e, 0x1000);
        paStack14 = (Struct18 *)CONCAT22(PTR_LOOP_1050_5f2e, uVar3);
        for(uStack22 = 0x0; uStack22 < uStack6; uStack22 = uStack22 + 0x1)
        {
            uVar1   = (uVar5 + 0xc);
            ppcVar2 = ((uVar5 + 0xc) + 0x4);
            uVar10  = uStack6;
            (**ppcVar2)(uVar9, uVar1, (uVar1 >> 0x10), uStack22, (uStack22 >> 0x10));
            uVar4              = uVar10;
            globals->PTR_LOOP_1050_5f2e = (extraout_DX_00 | uVar4);
            uStack18           = uVar4;
            uStack16           = extraout_DX_00;
            if(PTR_LOOP_1050_5f2e != 0x0)
            {
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, extraout_DX_00);
                uStack22._0_2_               = uStack22 * 0x4;
                uVar8                        = (paStack10 >> 0x10);
                iVar6                        = paStack10;
                (uStack22 + iVar6)           = uVar4;
                (uStack22 + iVar6 + 0x2)     = globals->PTR_LOOP_1050_5f2e;
                uVar9                        = 0x1030;
                uVar10                       = struct_op_1030_73a8(CONCAT22(PTR_LOOP_1050_5f2e, (uStack22 + iVar6)));
                globals->PTR_LOOP_1050_5f2e           = (uVar10 >> 0x10);
                uVar8                        = (paStack14 >> 0x10);
                (paStack14 + uStack22)       = uVar10;
                (paStack14 + uStack22 + 0x2) = globals->PTR_LOOP_1050_5f2e;
            }
        }
        for(uStack22 = 0x0; uStack22 < uStack6; uStack22 = uStack22 + 0x1)
        {
            uVar9 = (paStack14 >> 0x10);
            iVar6 = paStack14;
            if(((uStack22 * 0x4 + iVar6) != 0x0) && (uVar1 = (uStack22 * 0x4 + iVar6), (uVar1 + 0x1a) = 0x0, uVar1 = (uStack22 * 0x4 + iVar6), (uVar1 + 0x12) == 0x5))
            {
                pass1_1028_bdac((uStack22 * 0x4 + iVar6), 0x6, &USHORT_1050_1028);
            }
        }
        (uVar5 + 0x204) = 0x0;
        puVar11         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_3, globals->PTR_LOOP_1050_5f2e, unaff_DI);
        uStack30        = (puVar11 >> 0x10);
        uStack26        = SUB42(puVar11, 0x0);
        puStack28       = globals->PTR_LOOP_1050_13ae;
        if(PTR_LOOP_1050_13ae == (&PTR_LOOP_1050_0000 + 0x1))
        {
            (uVar5 + 0x204) = 0x1;
        }
        uStack24 = uStack30;
        pass1_1038_5a96(uVar5, uVar7, uStack6, paStack14);
        pass1_1038_5cc6(param_1, uStack6, paStack14, paStack10, 0x0, 0x2);
        pass1_1038_5b3c(uVar5, uVar7, uStack6, paStack14);
        pass1_1038_5cc6(param_1, uStack6, paStack14, paStack10, 0x0, 0x1);
        uVar14    = SUB21(local_22, 0x0);
        uVar15    = (undefined)(local_22 >> 0x8);
        piStack32 = &local_24;
        uVar12    = SUB21(piStack32, 0x0);
        uVar13    = (undefined)(piStack32 >> 0x8);
        uVar1     = (uVar5 + 0x8);
        uVar3     = param_3;
        uVar16    = param_3;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        pass1_1030_5b1c(CONCAT22(uStack30, piStack32), CONCAT22(uVar3, CONCAT11(uVar13, uVar12)), CONCAT22(uVar16, CONCAT11(uVar15, uVar14)));
        for(iStack38 = 0x1; iStack38 <= local_24; iStack38 = iStack38 + 0x1)
        {
            pass1_1038_58e6(uVar5, uVar7, uStack6, paStack14, paStack10, iStack38, param_3);
            pass1_1038_5cc6(param_1, uStack6, paStack14, paStack10, iStack38, 0x3);
        }
        pass1_1038_5a16(uVar5, uVar7, uStack6, paStack14);
        for(uStack22 = 0x0; uStack22 < uStack6; uStack22 = uStack22 + 0x1)
        {
            uVar9 = (paStack14 >> 0x10);
            iVar6 = paStack14;
            if(((uStack22 * 0x4 + iVar6) != 0x0) && (uVar1 = (uStack22 * 0x4 + iVar6), (uVar1 + 0x12) != 0x5))
            {
                uVar1   = (uStack22 * 0x4 + iVar6);
                ppcVar2 = ((uStack22 * 0x4 + iVar6) + 0x28);
                (**ppcVar2)(0x1030, uVar1, (uVar1 >> 0x10));
            }
        }
        fn_ptr_1000_17ce(paStack10, 0x1000);
        fn_ptr_1000_17ce(paStack14, 0x1000);
    }
    return;
}


void  pass1_1038_42cc(u32 param_1, u16 param_2)

{
    code      **ppcVar1;
    u32  uVar2;
    bool        bVar3;
    u16         uVar4;
    u16         uVar5;
    u16         uVar6;
    u8         *puVar7;
    u16         extraout_DX;
    u16         uVar8;
    u16         extraout_DX_00;
    i16         iVar9;
    u16         uVar10;
    u16         uVar11;
    u32        *puVar12;
    u32 *puVar13;
    u32         uStack24;
    u32         uStack18;
    u32 *puStack10;

    uVar10 = (param_1 >> 0x10);
    iVar9  = param_1;
    if((iVar9 + 0x1f6) == 0x0)
    {
        return;
    }
    uVar11  = 0x1008;
    puVar12 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2d);
    puVar7  = (puVar12 >> 0x10);
    uVar4   = puVar12;
    pass1_1038_4d6e(param_1, puVar12, uVar4, puVar7);
    puStack10 = CONCAT22(puVar7, uVar4);
    ppcVar1   = (*puStack10 + 0x10);
    uVar5     = uVar4;
    (**ppcVar1)(0x1008, uVar4, puVar7);
    uStack18 = CONCAT22(extraout_DX, uVar5);
    bVar3    = false;
    uVar8    = extraout_DX;
    for(uStack24 = 0x0; uStack24 < uStack18; uStack24 = uStack24 + 0x1)
    {
        uVar11  = 0x1030;
        puVar13 = pass1_1030_1d7c(uVar5, uVar8, puStack10);
        uVar6   = puVar13;
        uVar8   = (puVar13 >> 0x10) | uVar6;
        if(uVar8 != 0x0)
        {
            ppcVar1 = (*puVar13 + 0x50);
            (**ppcVar1)();
            uVar8 = extraout_DX_00;
            if(uVar6 != 0x0)
            {
                bVar3 = true;
            }
        }
    }
    if(bVar3)
    {
        uVar2           = (iVar9 + 0x1f6);
        (uVar2 + 0x1aa) = 0x0;
    }
    else
    {
        uVar11 = 0x1030;
        pass1_1030_38b8();
        uVar8 = uVar8 | uStack18;
        if(uVar8 != 0x0)
        {
            uVar11 = 0x1030;
            pass1_1030_326a(*(iVar9 + 0x1f6), uStack18, uVar8, param_2);
        }
    }
    if(puStack10 != 0x0)
    {
        ppcVar1 = *puStack10;
        (**ppcVar1)(uVar11, uVar4, puVar7, 0x1);
    }
    return;
}


void  pass1_1038_2306(u16 param_1, u16 param_2, u32 param_3)

{
    i16         *piVar1;
    u32  *puVar2;
    code       **ppcVar3;
    qword        qVar4;
    u32  *puVar5;
    astruct_417 *uVar9;
    u32          uVar6;
    u32  *puVar7;
    u16          uVar8;
    u16          uVar10;
    astruct_419 *iVar11;
    astruct_418 *iVar12;
    u16          uVar11;
    u16          uVar12;
    u16          uVar13;
    u32          uVar14;
    u32          uStack42;
    u32          uStack34;
    u16          uStack30;
    u32          uStack24;
    u32   uStack12;
    i16          iStack8;

    uVar13   = 0x1030;
    uVar14   = struct_op_1030_73a8(param_3);
    uStack24 = uVar14 >> 0x10;
    uVar11   = (param_3 >> 0x10);
    iVar11   = (astruct_419 *)param_3;
    iStack8  = iVar11->field_0x34;
    uStack12 = 0x64;
    puVar2   = (uVar14 + 0x22);
    puVar7   = puVar2;
    while(true)
    {
        uVar8   = uStack24;
        uVar12  = (puVar2 >> 0x10);
        ppcVar3 = (*puVar2 + 0x10);
        (**ppcVar3)(uVar13, puVar2, uVar12);
        uVar9  = (astruct_417 *)puVar7;
        uVar14 = puVar7 & 0xffff;
        puVar5 = (uVar14 | uVar8 << 0x10);
        if((uVar8 | uVar9) == 0x0)
            break;
        if(uVar9->field_0xa == 0x0)
        {
            uStack24 = (uVar8 | uVar9);
            if((uVar8 | uVar9) != 0x0)
            {
                ppcVar3 = *puVar5;
                (**ppcVar3)(uVar13, uVar9, uVar8, 0x1);
            }
        }
        else
        {
            uStack24 = 0x0;
            uStack30 = 0x0;
            if(uVar9->field_0x6 == 0x0)
            {
                if(uVar9->field_0x8 != 0x0)
                {
                    uStack30 = pass1_1020_c42e(uVar9->field_0x8);
                    goto LAB_1038_2385;
                }
            }
            else
            {
                uStack30 = switch_1020_c3b4(uVar9->field_0x6);
            LAB_1038_2385:
                uVar13   = 0x1020;
                uStack24 = (uVar9->field_0xa * uStack30);
            }
            uStack12._2_2_ = 0x0;
            if(uStack12 < uStack24)
            {
                uStack24 = uStack12 & 0xffff;
            }
            uStack34 = uStack24 | uStack12._2_2_ << 0x10;
            uStack24 = uStack24 | uStack12._2_2_ << 0x10;
            qVar4    = (qword)uStack24 / (qword)uStack30;
            uVar6    = qVar4;
            uStack24 = uStack24 % uStack30;
            piVar1   = &uVar9->field_0xa;
            *piVar1  = *piVar1 - qVar4;
            if(*piVar1 == 0x0)
            {
                uStack24 = (uVar8 | uVar9);
                if((uVar8 | uVar9) != 0x0)
                {
                    ppcVar3 = *puVar5;
                    (**ppcVar3)(uVar13, uVar9, uVar8, 0x1);
                }
            }
            else
            {
                ppcVar3 = (*puVar2 + 0x8);
                (**ppcVar3)(uVar13, puVar2, uVar12, uVar9, uVar8);
            }
            uStack12 = uStack12 - uStack34;
            puVar7   = 0x0;
            uStack42 = 0x0;
            iVar12   = (astruct_418 *)uVar14;
            if(iVar12->field_0x6 == 0x0)
            {
                if(iVar12->field_0x8 != 0x0)
                {
                    mem_op_1000_179c(0x2a, uStack24, 0x1000);
                    uVar10 = uStack24 | puVar7;
                    uVar14 = uVar10;
                    if(uVar10 == 0x0)
                        goto LAB_1038_244e;
                    pass1_1038_6838((puVar7 & 0xffff | uStack24 << 0x10), uVar6, iVar12->field_0x8, 0x1, iVar11->field_0x4);
                    goto LAB_1038_24b3;
                }
            }
            else
            {
                mem_op_1000_179c(0x2a, uStack24, 0x1000);
                uVar10 = uStack24 | puVar7;
                uVar14 = uVar10;
                if(uVar10 == 0x0)
                {
                LAB_1038_244e:
                    uVar13   = 0x1000;
                    uStack42 = 0x0;
                    uStack24 = uVar14;
                }
                else
                {
                    pass1_1038_675c((puVar7 & 0xffff | uStack24 << 0x10), uVar6, iVar12->field_0x6, 0x1, iVar11->field_0x4);
                LAB_1038_24b3:
                    uVar13   = 0x1000;
                    uStack42 = puVar7 & 0xffff | uVar14 << 0x10;
                    uStack24 = uVar14;
                }
            }
            if(uStack42 != 0x0)
            {
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                iStack8 = iStack8 + -0x1;
                if(iStack8 == 0x0)
                    break;
                uStack12 = 0x64;
            }
        }
    }
    pass1_1030_6c4c(param_3, iStack8);
    return;
}


void  pass1_1038_24e8(u16 param_1, u16 param_2, u32 param_3, u16 param_4, u16 param_5, u16 param_6)

{
    u16         uVar1;
    u32         uVar2;
    u8         *puVar3;
    u8         *puVar4;
    i16         iVar5;
    u16         uVar6;
    u16         uStack30;
    Struct18 *paStack28;
    u32  local_18;
    u16         local_14;
    u16         uStack18;
    u32  uStack16;
    u32        *puStack12;
    i16         iStack8;
    u32         uStack6;

    uStack6   = struct_op_1030_73a8(param_3);
    puVar4    = (uStack6 >> 0x10);
    uVar6     = (param_3 >> 0x10);
    iVar5     = param_3;
    iStack8   = (iVar5 + 0x34);
    puStack12 = (uStack6 + 0x28);
    uStack16  = 0x64;
    uStack18  = (puStack12 + 0x4);
    uVar2     = uStack18;
    mem_op_1000_179c(0xa, puVar4, 0x1000);
    uVar1  = uVar2;
    puVar3 = (puVar4 | uVar1);
    if(puVar3 == 0x0)
    {
        uVar1  = 0x0;
        puVar3 = 0x0;
    }
    else
    {
        pass1_1020_ba3e((long *)(uVar2 & 0xffff | ZEXT24(puVar4) << 0x10), 0x5, 0x5, param_5, param_4);
    }
    paStack28 = (Struct18 *)CONCAT22(puVar3, uVar1);
    for(uStack30 = 0x0; uVar2 = uStack18, uStack30 < uStack18; uStack30 = uStack30 + 0x1)
    {
        pass1_1020_bb16(puStack12, CONCAT22(param_6, &local_18), CONCAT22(param_6, &local_14), uStack30);
        if(local_18 != 0x0)
        {
            uVar2          = local_18;
            uStack16._2_2_ = local_18._2_2_;
            if(uStack16 < local_18)
            {
                uVar2 = uStack16 & 0xffff;
            }
            uVar1    = uVar2;
            uVar2    = uVar2 & 0xffff | ZEXT24(uStack16._2_2_) << 0x10;
            local_18 = CONCAT22(local_18._2_2_ + (-(local_18 < uVar1) - uStack16._2_2_), local_18 - uVar1);
            puVar3   = uStack16._2_2_;
            pass1_1020_bb8a((long *)puStack12, local_18 - uVar1, CONCAT22(local_14, local_18._2_2_ + (-(local_18 < uVar1) - uStack16._2_2_)), param_5, param_6);
            pass1_1020_bb70((long *)paStack28, uVar1, CONCAT22(local_14, uStack16._2_2_), param_5, param_4, param_6);
            uStack16 = uStack16 - uVar2;
            if(uStack16 == 0x0)
            {
                mem_op_1000_179c(0x2a, puVar3, 0x1000);
                puVar4 = (puVar3 | uVar2);
                if(puVar4 == 0x0)
                {
                    uVar2 = 0x0;
                }
                else
                {
                    pass1_1038_666e((uVar2 & 0xffff | ZEXT24(puVar3) << 0x10), (long *)paStack28, 0x1, *(iVar5 + 0x4));
                }
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                mem_op_1000_179c(0xa, puVar4, 0x1000);
                puVar3 = (puVar4 | uVar2);
                if(puVar3 == 0x0)
                {
                    uVar2  = 0x0;
                    puVar3 = 0x0;
                }
                else
                {
                    pass1_1020_ba3e((long *)(uVar2 & 0xffff | ZEXT24(puVar4) << 0x10), 0x5, 0x5, param_5, param_4);
                }
                paStack28 = (Struct18 *)(uVar2 & 0xffff | ZEXT24(puVar3) << 0x10);
                iStack8   = iStack8 + -0x1;
                if(iStack8 == 0x0)
                    break;
                uStack16 = 0x64;
            }
        }
    }
    pass1_1020_ba94((long *)paStack28);
    puVar3 = (puVar3 | uVar2);
    if(puVar3 == 0x0)
    {
        if(paStack28 != (Struct18 *)0x0)
        {
            fn_ptr_1020_ba7e(paStack28);
            fn_ptr_1000_17ce(paStack28, 0x1000);
        }
    }
    else
    {
        mem_op_1000_179c(0x2a, puVar3, 0x1000);
        if((puVar3 | uVar2) != 0x0)
        {
            pass1_1038_666e((uVar2 & 0xffff | ZEXT24(puVar3) << 0x10), (long *)paStack28, 0x1, *(iVar5 + 0x4));
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    }
    pass1_1030_6c4c(param_3, iStack8);
    return;
}


void  pass1_1038_26ee(u16 param_1, u16 param_2, u32 param_3, u16 param_4, u16 param_5, u16 param_6)

{
    u16        uVar1;
    u16        uVar2;
    u32        uVar3;
    u8        *puVar4;
    u8        *puVar5;
    u8        *puVar6;
    i16        iVar7;
    u16        uVar8;
    u32        uVar9;
    u32 uVar10;
    u32        uStack36;
    u16        uStack20;
    u8        *puStack18;
    u32 uStack16;
    u16        uStack12;
    u16        uStack10;
    i16        iStack8;

    uVar9    = struct_op_1030_73a8(param_3);
    puVar6   = (uVar9 >> 0x10);
    uVar8    = (param_3 >> 0x10);
    iVar7    = param_3;
    iStack8  = (iVar7 + 0x34);
    uStack12 = pass1_1028_0d80(uVar9);
    uVar3    = uStack12;
    uStack16 = 0x64;
    mem_op_1000_179c(0xa, puVar6, 0x1000);
    puVar4 = (puVar6 | uVar3);
    if(puVar4 == 0x0)
    {
        uVar3  = 0x0;
        puVar4 = 0x0;
    }
    else
    {
        pass1_1020_ba3e((long *)(uVar3 & 0xffff | ZEXT24(puVar6) << 0x10), 0x5, 0x5, param_5, param_4);
    }
    uStack20  = uVar3;
    uStack10  = uStack12;
    puStack18 = puVar4;
    do
    {
        uVar10 = pass1_1030_7c28(param_3, uStack10, uVar3, puVar4, param_6);
        puVar6 = (uVar10 >> 0x10);
        uVar1  = uVar10;
        puVar4 = (puVar6 | uVar1);
        if(puVar4 != 0x0)
        {
            puVar5 = puVar6;
            uVar2  = uVar1;
            if((uStack16._2_2_ <= puVar6) && ((uStack16._2_2_ < puVar6 || (uStack16 < uVar1))))
            {
                puVar5 = uStack16._2_2_;
                uVar2  = uStack16;
            }
            uStack36 = CONCAT22(puVar5, uVar2);
            puVar4   = puVar5;
            pass1_1030_7d1c(param_3, uVar1 - uVar2, CONCAT22(uStack10, puVar6 + (-(uVar1 < uVar2) - puVar5)), uVar2, puVar5, param_4, param_5, param_6);
            pass1_1020_bb70((long *)CONCAT22(puStack18, uStack20), uVar2, CONCAT22(uStack10, puVar5), param_5, param_4, param_6);
            uStack16 = uStack16 - uStack36;
            if(uStack16 == 0x0)
            {
                mem_op_1000_179c(0x2a, puVar4, 0x1000);
                uStack10 = uStack36;
                puVar6   = (puVar4 | uStack10);
                if(puVar6 == 0x0)
                {
                    uStack10 = 0x0;
                }
                else
                {
                    pass1_1038_666e((uStack36 & 0xffff | ZEXT24(puVar4) << 0x10), (long *)CONCAT22(puStack18, uStack20), 0x1, *(iVar7 + 0x4));
                }
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                mem_op_1000_179c(0xa, puVar6, 0x1000);
                puVar4 = (puVar6 | uStack10);
                if(puVar4 == 0x0)
                {
                    uStack10 = 0x0;
                    puVar4   = 0x0;
                }
                else
                {
                    pass1_1020_ba3e((long *)CONCAT22(puVar6, uStack10), 0x5, 0x5, param_5, param_4);
                }
                iStack8   = iStack8 + -0x1;
                uStack20  = uStack10;
                puStack18 = puVar4;
                if(iStack8 == 0x0)
                    break;
                uStack16 = 0x64;
            }
        }
        uStack10 = pass1_1028_0d80(uVar9);
        uVar3    = uStack10;
        if(uStack12 == 0x0)
        {
            uStack12 = uStack10;
        }
    } while(uStack12 != uStack10);
    pass1_1020_ba94((long *)CONCAT22(puStack18, uStack20));
    puVar4 = (puVar4 | uStack10);
    if(puVar4 == 0x0)
    {
        if((puStack18 | uStack20) != 0x0)
        {
            fn_ptr_1020_ba7e(CONCAT22(puStack18, uStack20));
            fn_ptr_1000_17ce((Struct18 *)CONCAT22(puStack18, uStack20), 0x1000);
        }
    }
    else
    {
        mem_op_1000_179c(0x2a, puVar4, 0x1000);
        if((puVar4 | uStack10) != 0x0)
        {
            pass1_1038_666e(CONCAT22(puVar4, uStack10), (long *)CONCAT22(puStack18, uStack20), 0x1, *(iVar7 + 0x4));
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    }
    pass1_1030_6c4c(param_3, iStack8);
    return;
}


Struct18 * pass1_1038_29d2(Struct18 *param_1, u8 param_2)

{
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1038_2a5c(u16 *param_1)

{
    u32 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    i16         iVar4;
    u16         uVar5;

    uVar5         = (param_1 >> 0x10);
    iVar4         = param_1;
    *param_1      = 0x309a;
    (iVar4 + 0x2) = &PTR_LOOP_1050_1038;
    puVar1        = (iVar4 + 0x114);
    uVar2         = (iVar4 + 0x116);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    puVar1 = (iVar4 + 0x110);
    uVar2  = (iVar4 + 0x112);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    *param_1      = 0x389a;
    (iVar4 + 0x2) = 0x1008;
    return;
}


void  pass1_1038_2c82(u16 param_1, u16 param_2, u32 param_3, u32 param_4, u32 param_5, u16 param_6, u16 param_7, u16 param_8, u16 param_9, u8 param_10)

{
    u16         *puVar1;
    i16         *piVar2;
    u16          uVar3;
    u16          uVar4;
    u32   uVar5;
    long         lVar6;
    code       **ppcVar7;
    u16          uVar8;
    u32  *puVar9;
    i16          iVar10;
    u32          uVar11;
    u8          *puVar12;
    u8          *puVar13;
    u16          uVar14;
    i16          iVar16;
    astruct_702 *iVar15;
    u16          uVar17;
    u16          uVar18;
    u8          *puVar19;
    u16         *puVar20;
    u8           uVar21;
    u32   uStack22;
    u32          local_12;
    u16         *puStack14;
    u32   uStack10;
    u32   uStack6;

    uVar17   = (param_5 >> 0x10);
    uVar14   = param_5;
    uStack6  = *(uVar14 + 0x200);
    uVar18   = (param_4 >> 0x10);
    iVar16   = param_4;
    uStack10 = (iVar16 + 0x200);
    puVar13  = (iVar16 + 0x202);
    puVar19  = (param_3 >> 0x10);
    iVar15   = (astruct_702 *)param_3;
    iVar10   = iVar15->field_0xc;
    if(iVar10 == 0x1)
    {
        puStack14 = param_3;
        pass1_1038_52b8(param_5, *&iVar15->field_0x8, &iVar15->field_0xe, param_6, param_7, param_8, param_9);
        return;
    }
    if(iVar10 == 0x2)
    {
        puStack14 = param_3;
        if(iVar15->field_0xe != 0x0)
        {
            pass1_1038_3efc(uVar14, uVar17, param_4, iVar15->field_0xe, iVar15, puVar19);
            return;
        }
        pass1_1020_a43e(param_9, puVar19, CONCAT22(param_9, &local_12));
        uStack22 = (puStack14 + 0x8);
        while(true)
        {
            uStack22 = uStack22 + -0x1;
            if((uStack22._2_2_ | uStack22) == 0x0)
                break;
            pass1_1020_a6ee(CONCAT13((param_9 >> 0x8), CONCAT12(param_9, &local_12)), (puStack14 + 0x12), &local_12, uStack22._2_2_ | uStack22, param_7, param_9, param_10);
        }
    }
    else
    {
        if(iVar10 == 0x3)
        {
            pass1_1038_3f38(param_5, param_4, iVar15->field_0xe, 0x0, puVar13);
            return;
        }
        uStack6._2_2_ = (uStack6 >> 0x10);
        if(iVar10 == 0x4)
        {
            globals->PTR_LOOP_1050_5f2e = (uStack6._2_2_ & 0xff);
            if((uStack6 == 0x1) && ((uStack6 & 0xff0000) == 0x0))
            {
                local_12 = *(uVar14 + 0x1f6);
                pass1_1030_3694(local_12, &iVar15->field_0xe, &iVar15->field_0x8, 0x0, 0x1030, param_9);
                (&iVar15->field_0xe + 0x2) = local_12;
                iVar15->field_0x12         = globals->PTR_LOOP_1050_5f2e;
            }
            else
            {
                if(_PTR_LOOP_1050_5f2c == 0x0)
                {
                    globals->PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e, 0x1000);
                }
                else
                {
                }
                uVar14                     = fn_ptr_op_1000_1708(0x16c, 0x0, 0x1, globals->PTR_LOOP_1050_5f2c, globals->PTR_LOOP_1050_5f2e, 0x1000);
                (&iVar15->field_0xe + 0x2) = uVar14;
                iVar15->field_0x12         = globals->PTR_LOOP_1050_5f2e;
                iVar10                     = &iVar15->field_0xe;
                if(iVar10 != 0x3)
                {
                    if(iVar10 != 0x4)
                    {
                        uVar5          = (&iVar15->field_0xe + 0x2);
                        (uVar5 + 0x28) = &iVar15->field_0x8;
                        return;
                    }
                    uVar5          = (&iVar15->field_0xe + 0x2);
                    (uVar5 + 0xdc) = &iVar15->field_0x8;
                    return;
                }
                uVar5          = (&iVar15->field_0xe + 0x2);
                (uVar5 + 0x64) = &iVar15->field_0x8;
            }
        }
        else
        {
            if(iVar10 == 0x5)
            {
                if(&iVar15->field_0xe == 0xc)
                {
                    if((uStack6 == 0x1) && ((uStack6 & 0xff0000) == 0x0))
                    {
                        uVar5   = (uVar14 + 0x1f6);
                        uVar3   = iVar15->field_0x8;
                        iVar10  = iVar15->field_0xa;
                        uVar8   = -uVar3;
                        uVar18  = (uVar5 >> 0x10);
                        iVar16  = uVar5;
                        puVar1  = (iVar16 + 0x170);
                        uVar4   = *puVar1;
                        *puVar1 = *puVar1 + uVar8;
                        piVar2  = (iVar16 + 0x172);
                        *piVar2 = (*piVar2 - (iVar10 + (uVar3 != 0x0))) + CARRY2(uVar4, uVar8);
                    }
                }
                else
                {
                    pass1_1038_43cc(uVar14, uVar17, iVar15->field_0x8, &iVar15->field_0xe, iVar15, puVar19);
                }
            }
            else
            {
                iVar10 = iVar10 + -0x7;
                if(iVar10 == 0x0)
                {
                    lVar6 = iVar15->field_0xe;
                    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, lVar6, (lVar6 >> 0x10));
                    puVar12 = puVar13;
                    pass1_1038_349e(CONCAT22(puVar13, iVar10), *(iVar16 + 0x200));
                    uVar21 = (undefined)(puVar13 >> 0x8);
                    pass1_1038_4d0e(CONCAT13(uVar21, CONCAT12(puVar13, iVar10)), 0x258);
                    pass1_1038_4d0e(CONCAT13(uVar21, CONCAT12(puVar13, iVar10)), 0x258);
                    puVar20 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3b, param_9, puVar12, param_7);
                    puVar13 = (puVar20 >> 0x10);
                    pass1_1008_de58(puVar20, iVar15->field_0xe, 0x4000001, param_9);
                    puVar20 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_9, puVar13, param_7);
                    puVar13 = (puVar20 >> 0x10);
                    uVar11  = *(puVar20 + 0x20);
                    pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), uVar11);
                    local_12  = uVar11 & 0xffff | ZEXT24(puVar13) << 0x10;
                    uVar14    = pass1_1030_5b00(uVar11 & 0xffff | ZEXT24(puVar13) << 0x10);
                    puStack14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, uVar14, param_9, puVar13, param_7);
                    puVar9    = (puStack14 + 0x20);
                    ppcVar7   = (*puVar9 + 0x4);
                    (**ppcVar7)(0x1010, puVar9, (puStack14 >> 0x10), 0x6);
                }
            }
        }
    }
    return;
}
