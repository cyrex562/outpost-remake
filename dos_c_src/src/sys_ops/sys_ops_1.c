
#include "../types.h"
#include "sys_ops_1.h"
#include "../globals.h"

void struct_1040_b082(Globals *globals, Struct57 *param_1, u32 param_2)

{
//    Struct57 *iVar1;
//    u16          uVar1;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, param_2, (param_2 >> 0x10));
//    uVar1             = (param_1 >> 0x10);
//    iVar1             = (Struct57 *)param_1;
    param_1->field_0x8e = 0x0;
    param_1->field_0x90 = 0x0;
    param_1->field_0x1           = 0xb772;
    param_1->field_0x2  = &globals->PTR_LOOP_1050_1040;
}

void  pass1_1040_b040(Struct57 *param_1, u32 param_2, u16 param_3)

{
    i16 iVar1;
    u16 uVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, (param_2 + 0x12), param_3);
    uVar2           = (param_1 >> 0x10);
    iVar1           = param_1;
    (iVar1 + 0x8e)  = 0x0;
    *(iVar1 + 0x90) = param_2;
    param_1         = 0xb772;
    (iVar1 + 0x2)   = &PTR_LOOP_1050_1040;
    return;
}

void  pass1_1040_b0bc(Struct57 *param_1, u32 param_2, u32 param_3)

{
    i16 iVar1;
    u16 uVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, param_3, (param_3 >> 0x10));
    uVar2           = (param_1 >> 0x10);
    iVar1           = param_1;
    (iVar1 + 0x8e)  = 0x0;
    *(iVar1 + 0x90) = param_2;
    param_1         = 0xb772;
    (iVar1 + 0x2)   = &PTR_LOOP_1050_1040;
    return;
}

void  pass1_1040_4068(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    u8          *puVar1;
    astruct_723 *iVar2;
    u16          uVar2;
    u16         *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfb7, param_5);
    uVar2              = (param_1 >> 0x10);
    iVar2              = (astruct_723 *)param_1;
    &iVar2->field_0x8e = 0x0;
    iVar2->field_0x92  = 0x0;
    iVar2->field_0x9a  = 0x0;
    param_1            = 0x4466;
    iVar2->field_0x2   = &PTR_LOOP_1050_1040;
    iVar2->field_0x76  = 0x1;
    puVar3             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_8, param_6, param_7);
    puVar1             = (puVar3 >> 0x10);
    iVar2->field_0x8e  = puVar3;
    iVar2->field_0x90  = puVar1;
    puVar3             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, param_8, puVar1, param_7);
    iVar2->field_0x96  = puVar3;
    iVar2->field_0x98  = (puVar3 >> 0x10);
    return;
}

void  get_sys_metrics_1040_7728(Struct57 *param_1, u16 param_2, u32 param_3, u16 param_4, u16 param_5)

{
    u16       IVar1;
    Struct57 *iVar2;
    u16         uVar2;

    uVar2             = (param_1 >> 0x10);
    iVar2             = param_1;
    param_1           = 0x389a;
    iVar2->field_0x2  = 0x1008;
    param_1           = 0x3aa8;
    iVar2->field_0x2  = 0x1008;
    iVar2->field_0x4  = 0x0;
    iVar2->field_0x6  = 0x0;
    iVar2->field_0x8  = param_5;
    iVar2->field_0xa  = param_4;
    iVar2->field_0xc  = 0x0;
    iVar2->field_0x60 = 0x0;
    iVar2->field_0x62 = 0x0;
    iVar2->field_0x64 = 0x0;
    iVar2->field_0x66 = 0x0;
    iVar2->field_0x68 = 0x0;
    iVar2->field_0x6a = param_3;
    iVar2->field_0x6e = param_2;
    iVar2->field_0x70 = 0x0;
    iVar2->field_0x74 = 0x0;
    iVar2->field_0x76 = 0x0;
    iVar2->field_0x78 = 0x0;
    iVar2->field_0x8a = 0x0;
    iVar2->field_0x8c = 0x0;
    param_1           = 0x840c;
    iVar2->field_0x2  = &PTR_LOOP_1050_1040;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | &iVar2->field_0x10), 0x10505db0);
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar2->field_0x7a), 0x0, 0x8);
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar2->field_0x82), 0x0, 0x8);
    IVar1             = GetSystemMetrics16(0x1000);
    iVar2->field_0x62 = IVar1;
    IVar1             = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
    iVar2->field_0x64 = IVar1;
    IVar1             = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
    iVar2->field_0x66 = IVar1;
    return;
}

void  pass1_1040_44d2(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u8 *param_5)

{
    u32 uVar1;
    u16        uVar2;
    u8        *puVar3;
    i16        iVar4;
    u16        uVar5;
    i16        iVar6;
    u16        uVar7;
    i16       *piStack8;

    iVar6 = param_1;
    uVar7 = (param_1 >> 0x10);
    struct_1040_b082(NULL;
    param_1       = 0x4824;
    (iVar6 + 0x2) = &PTR_LOOP_1050_1040;
    mem_op_1000_179c(0x18, param_5, 0x1000);
    puVar3 = (param_5 | param_4);
    if(puVar3 == 0x0)
    {
        (iVar6 + 0x90) = 0x0;
    }
    else
    {
        struct_1040_a598(CONCAT22(param_5, param_4));
        (iVar6 + 0x90) = param_4;
        (iVar6 + 0x92) = puVar3;
    }
    (iVar6 + 0x90) = 0x14;
    iVar4          = (iVar6 + 0x90);
    uVar2          = iVar4 * 0xa + 0x2;
    mem_op_1000_179c(uVar2, puVar3, 0x1000);
    piStack8 = CONCAT22(puVar3, uVar2);
    if((puVar3 | uVar2) == 0x0)
    {
        uVar1         = (iVar6 + 0x90);
        (uVar1 + 0x2) = 0x0;
    }
    else
    {
        *piStack8 = iVar4;
        pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iVar4, 0xa, uVar2 + 0x2, puVar3);
        uVar1         = (iVar6 + 0x90);
        uVar5         = (uVar1 >> 0x10);
        iVar4         = uVar1;
        (iVar4 + 0x2) = uVar2 + 0x2;
        (iVar4 + 0x4) = puVar3;
    }
    uVar1          = (iVar6 + 0x90);
    *(uVar1 + 0x6) = param_2;
    uVar1          = (iVar6 + 0x90);
    (uVar1 + 0xa)  = 0x1;
    uVar1          = (iVar6 + 0x90);
    (uVar1 + 0x12) = (iVar6 + 0xa);
    return;
}

void  pass1_1040_45e8(i16 param_1, u16 param_2, u16 param_3, u32 param_4, u8 *param_5, u16 param_6, u16 param_7)

{
    Struct18 *paVar1;
    code      **ppcVar2;
    u32  uVar3;
    u16         uVar4;
    Struct18 *paVar5;
    u8         *puVar6;
    u8         *puVar7;
    i16         iVar8;
    i16         unaff_DI;
    u16         uVar9;
    astruct_20 *paVar10;
    i16        *piStack16;

    if(param_4._2_2_ != 0xeb)
    {
        pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, param_6, param_7);
        return;
    }
    paVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_7, param_5, unaff_DI);
    puVar6  = (paVar10 >> 0x10);
    paVar1  = (param_1 + 0x90);
    if(paVar1 != (Struct18 *)0x0)
    {
        paVar5 = paVar1;
        mem_op_1000_179c(0x18, puVar6, 0x1000);
        uVar4  = paVar5;
        puVar7 = (puVar6 | uVar4);
        if(puVar7 == 0x0)
        {
            uVar4  = 0x0;
            puVar7 = 0x0;
        }
        else
        {
            struct_1040_a598((paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
        }
        (param_1 + 0x90) = uVar4;
        (param_1 + 0x92) = puVar7;
        (param_1 + 0x90) = 0x14;
        iVar8            = (param_1 + 0x90);
        uVar4            = iVar8 * 0xa + 0x2;
        mem_op_1000_179c(uVar4, puVar7, 0x1000);
        piStack16 = CONCAT22(puVar7, uVar4);
        if((puVar7 | uVar4) == 0x0)
        {
            uVar3         = (param_1 + 0x90);
            (uVar3 + 0x2) = 0x0;
        }
        else
        {
            *piStack16 = iVar8;
            pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iVar8, 0xa, uVar4 + 0x2, puVar7);
            uVar3         = (param_1 + 0x90);
            uVar9         = (uVar3 >> 0x10);
            iVar8         = uVar3;
            (iVar8 + 0x2) = uVar4 + 0x2;
            (iVar8 + 0x4) = puVar7;
        }
        uVar3          = (param_1 + 0x90);
        (uVar3 + 0x6)  = (paVar1 + 0x6);
        uVar3          = (param_1 + 0x90);
        (uVar3 + 0xa)  = 0x1;
        uVar3          = (param_1 + 0x90);
        (uVar3 + 0x12) = (param_1 + 0xa);
        pass1_1010_a50c(paVar10, 0x10505d40, *(param_1 + 0x90));
        if(paVar1 != (Struct18 *)0x0)
        {
            pass1_1040_a5d0(paVar1);
            fn_ptr_1000_17ce(paVar1, 0x1000);
        }
        ppcVar2 = (CONCAT22(param_2, param_1) + 0x70);
        (**ppcVar2)();
    }
    return;
}

void  pass1_1040_48a0(Struct57 *param_1, u16 param_2, u32 param_3, u16 param_4, u8 *param_5, u16 param_6)

{
    i16          iVar1;
    i16         *piVar2;
    u16          uVar3;
    u8          *puVar4;
    u8          *puVar5;
    astruct_444 *iVar5;
    astruct_445 *iVar6;
    i16          unaff_DI;
    u16          uVar6;
    u16          uVar7;
    u16         *puVar8;
    i16         *piStack8;

    struct_1040_b082(NULL;
    uVar6                      = (param_1 >> 0x10);
    iVar5                      = (astruct_444 *)param_1;
    iVar5->field_0x94          = 0x0;
    param_1                    = &PTR_LOOP_1050_4e18;
    iVar5->field_0x2           = &PTR_LOOP_1050_1040;
    puVar8                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, param_5, unaff_DI);
    puVar4                     = (puVar8 >> 0x10);
    uVar3                      = puVar8;
    &iVar5->field_0x94         = uVar3;
    (&iVar5->field_0x94 + 0x2) = puVar4;
    mem_op_1000_179c(0x18, puVar4, 0x1000);
    puVar5 = (puVar4 | uVar3);
    if(puVar5 == 0x0)
    {
        iVar5->field_0x90 = 0x0;
    }
    else
    {
        struct_1040_a598(CONCAT22(puVar4, uVar3));
        &iVar5->field_0x90         = uVar3;
        (&iVar5->field_0x90 + 0x2) = puVar5;
    }
    *iVar5->field_0x90 = 0x7;
    iVar1              = *iVar5->field_0x90;
    uVar3              = iVar1 * 0xa + 0x2;
    mem_op_1000_179c(uVar3, puVar5, 0x1000);
    piStack8 = CONCAT22(puVar5, uVar3);
    if((puVar5 | uVar3) == 0x0)
    {
        piVar2         = iVar5->field_0x90;
        (piVar2 + 0x2) = 0x0;
    }
    else
    {
        *piStack8 = iVar1;
        pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iVar1, 0xa, uVar3 + 0x2, puVar5);
        piVar2           = iVar5->field_0x90;
        uVar7            = (piVar2 >> 0x10);
        iVar6            = (astruct_445 *)piVar2;
        iVar6->field_0x2 = uVar3 + 0x2;
        iVar6->field_0x4 = puVar5;
    }
    piVar2          = iVar5->field_0x90;
    *(piVar2 + 0x6) = param_3;
    piVar2          = iVar5->field_0x90;
    (piVar2 + 0xa)  = param_2;
    piVar2          = iVar5->field_0x90;
    (piVar2 + 0x12) = iVar5->field_0xa;
    iVar1           = &iVar5->field_0x90;
    uVar7           = (&iVar5->field_0x90 + 0x2);
    pass1_1010_debe(iVar5->field_0x94, (iVar1 + 0xa), CONCAT22(uVar7, iVar1 + 0x10), CONCAT22(uVar7, iVar1 + 0xc), param_3, param_6);
    return;
}

void  pass1_1040_23ea(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u16 param_6, u16 param_7)

{
    u32   uVar1;
    astruct_436 *iVar2;
    i16          unaff_DI;
    u16          uVar2;
    u16         *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x9a, param_2, 0xfbd, param_5);
    uVar2              = (param_1 >> 0x10);
    iVar2              = (astruct_436 *)param_1;
    &iVar2->field_0x8e = 0x0;
    iVar2->field_0x92  = 0x0;
    iVar2->field_0x94  = 0x0;
    param_1            = 0x2956;
    iVar2->field_0x2   = &PTR_LOOP_1050_1040;
    iVar2->field_0x8a  = 0x26;
    puVar3             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x6, param_6, param_7, unaff_DI);
    iVar2->field_0x8e  = puVar3;
    iVar2->field_0x90  = (puVar3 >> 0x10);
    uVar1              = &iVar2->field_0x8e;
    iVar2->field_0x92  = (uVar1 + 0x28);
    return;
}

void  pass1_1040_2ea2(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    astruct_720 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x180, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_720 *)param_1;
    iVar1->field_0x8e  = 0x0;
    iVar1->field_0x90  = 0x0;
    iVar1->field_0x92  = 0x0;
    iVar1->field_0x94  = 0x0;
    &iVar1->field_0x96 = 0x0;
    param_1            = 0x3436;
    iVar1->field_0x2   = &PTR_LOOP_1050_1040;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3c, param_8, param_6, param_7);
    iVar1->field_0x96  = puVar2;
    iVar1->field_0x98  = (puVar2 >> 0x10);
    return;
}

void  pass1_1040_34a2(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    astruct_721 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x192, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_721 *)param_1;
    &iVar1->field_0x8e = 0x0;
    iVar1->field_0x92  = 0x0;
    iVar1->field_0x94  = 0x0;
    iVar1->field_0x96  = 0x0;
    iVar1->field_0x98  = 0x0;
    param_1            = s_Null_Ptr_1050_38f3 + 0x7;
    iVar1->field_0x2   = &PTR_LOOP_1050_1040;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3c, param_8, param_6, param_7);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    return;
}

Struct57 * pass1_1040_123e(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    astruct_717 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfd1, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_717 *)param_1;
    &iVar1->field_0x8e = 0x0;
    param_1            = 0x17b0;
    iVar1->field_0x2   = &PTR_LOOP_1050_1040;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x46, param_8, param_6, param_7);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    return param_1;
}

void  pass1_1040_181c(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, u16 param_7)

{
    astruct_433 *iVar1;
    i16          unaff_DI;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfbb, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_433 *)param_1;
    &iVar1->field_0x8e = 0x0;
    iVar1->field_0x92  = 0x0;
    iVar1->field_0x94  = 0x0;
    param_1            = 0x1c48;
    iVar1->field_0x2   = &PTR_LOOP_1050_1040;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_7, param_6, unaff_DI);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    return;
}

void  pass1_1040_1cb4(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    u8          *puVar1;
    astruct_718 *iVar2;
    u16          uVar2;
    u16         *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xe8, param_5);
    uVar2              = (param_1 >> 0x10);
    iVar2              = (astruct_718 *)param_1;
    &iVar2->field_0x8e = 0x0;
    &iVar2->field_0x92 = 0x0;
    param_1            = 0x1eee;
    iVar2->field_0x2   = &PTR_LOOP_1050_1040;
    puVar3             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_8, param_6, param_7);
    puVar1             = (puVar3 >> 0x10);
    iVar2->field_0x8e  = puVar3;
    iVar2->field_0x90  = puVar1;
    puVar3             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_8, puVar1, param_7);
    iVar2->field_0x92  = puVar3;
    iVar2->field_0x94  = (puVar3 >> 0x10);
    return;
}

void  pass1_1040_1f5a(Struct57 *param_1, u16 param_2, i16 param_3, u16 param_4)

{
    i16         *piVar1;
    u8          *puVar2;
    astruct_719 *iVar3;
    astruct_43  *paVar3;
    u32          uVar4;
    u16         *puVar5;
    i16          iVar6;
    u16          uVar7;
    i16          iVar8;
    i16          iVar9;
    u16          uVar10;
    u32   local_16;
    u32   uStack18;

    iVar6 = param_1;
    uVar7 = (param_1 >> 0x10);
    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfcf, param_2);
    (iVar6 + 0x8e) = 0x0;
    (iVar6 + 0xa2) = 0x0;
    (iVar6 + 0xa6) = 0x0;
    param_1        = 0x237e;
    (iVar6 + 0x2)  = &PTR_LOOP_1050_1040;
    paVar3         = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1cc, param_4);
    (iVar6 + 0x8e) = paVar3;
    (iVar6 + 0x90) = (paVar3 >> 0x10);
    uVar4          = pass1_1008_4772((astruct_76 *)(paVar3 & 0xffff0000 | (iVar6 + 0x8e)));
    puVar2         = (uVar4 >> 0x10);
    puVar5         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, puVar2, param_3);
    local_16       = CONCAT22((uVar4 + 0x8) + 0xa, 0xa);
    uStack18       = CONCAT22(0x1d6, (uVar4 + 0x4) + -0xa);
    (iVar6 + 0x92) = local_16;
    (iVar6 + 0x96) = uStack18;
    (iVar6 + 0x9a) = local_16;
    (iVar6 + 0x9e) = uStack18;
    piVar1         = (iVar6 + 0x9c);
    *piVar1        = *piVar1 + 0x14;
    iVar9          = iVar6 + 0xa2;
    iVar8          = iVar6 + 0xa6;
    uVar10         = uVar7;
    puVar5         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_4, (puVar5 >> 0x10), iVar6 + 0xa2);
    pass1_1010_0538(puVar5, (char **)CONCAT22(uVar7, iVar8), (char **)CONCAT22(uVar10, iVar9), 0x1010, param_4);
    return;
}

void  pass1_1038_eeda(Struct57 *param_1, u16 param_2, u8 *param_3, i16 param_4, u16 param_5)

{
    astruct_714 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x166, param_2);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_714 *)param_1;
    &iVar1->field_0x8e = 0x0;
    iVar1->field_0x92  = 0x0;
    iVar1->field_0x94  = 0x0;
    param_1            = 0x67c;
    iVar1->field_0x2   = &PTR_LOOP_1050_1040;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x9, param_5, param_3, param_4);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    iVar1->field_0x74  = 0x1;
    return;
}

Struct57 * pass1_1040_06e8(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, u16 param_7)

{
    i16  iVar1;
    i16  unaff_DI;
    u16  uVar2;
    u16 *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfbc, param_5);
    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    (iVar1 + 0x8e) = 0x0;
    param_1        = 0xb90;
    (iVar1 + 0x2)  = &PTR_LOOP_1050_1040;
    puVar3         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x7, param_7, param_6, unaff_DI);
    (iVar1 + 0x8e) = puVar3;
    (iVar1 + 0x90) = (puVar3 >> 0x10);
    return param_1;
}

void  pass1_1040_0a1a(u32 param_1)

{
    u16         uVar1;
    u32 *puVar2;
    code      **ppcVar3;
    u32  uVar4;
    u32 *puVar5;
    u8         *extraout_DX;
    u8         *puVar6;
    u8         *extraout_DX_00;
    u8         *puVar7;
    i16         iVar8;
    i16         iVar9;
    u16         uVar10;
    u16         uVar11;
    u32         uStack10;
    u16         uStack6;

    uVar10  = (param_1 >> 0x10);
    iVar8   = param_1;
    uVar4   = (iVar8 + 0x8e);
    uVar11  = (uVar4 >> 0x10);
    iVar9   = uVar4;
    puVar2  = (iVar9 + 0xa);
    uStack6 = puVar2;
    puVar5  = ((iVar9 + 0xc) | uStack6);
    if(puVar5 == 0x0)
    {
        return;
    }
    ppcVar3 = (*puVar2 + 0x14);
    (**ppcVar3)();
    uStack10 = CONCAT22(extraout_DX, puVar5);
    puVar6   = extraout_DX;
    if((iVar8 + 0x70) != 0x0)
    {
        puVar5 = (iVar8 + 0x70);
        uVar1  = (iVar8 + 0x72);
        puVar6 = (uVar1 | puVar5);
        if(puVar6 != 0x0)
        {
            ppcVar3 = *puVar5;
            (**ppcVar3)();
            puVar6 = extraout_DX_00;
        }
    }
    mem_op_1000_179c(0x14, puVar6, 0x1000);
    puVar7 = (puVar6 | puVar5);
    if(puVar7 == 0x0)
    {
        puVar5 = 0x0;
        puVar7 = 0x0;
    }
    else
    {
        struct_1008_4c58(CONCAT22(puVar6, puVar5));
    }
    (iVar8 + 0x70) = puVar5;
    (iVar8 + 0x72) = puVar7;
    pass1_1008_4d84((iVar8 + 0x70), uStack10, puVar7);
    return;
}

Struct57 * pass1_1040_0bfc(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    astruct_715 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfcd, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_715 *)param_1;
    &iVar1->field_0x8e = 0x0;
    param_1            = 0xdb0;
    iVar1->field_0x2   = &PTR_LOOP_1050_1040;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x39, param_8, param_6, param_7);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    iVar1->field_0x74  = 0x1;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same
// address

void  pass1_1040_0e1c(Struct57 *param_1, u16 param_2, u32 param_3, u16 param_4, u8 *param_5, i16 param_6, u16 param_7)

{
    astruct_716 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1c0, param_4);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_716 *)param_1;
    &iVar1->field_0x8e = 0x0;
    iVar1->field_0x92  = param_3;
    iVar1->field_0x96  = 0x0;
    iVar1->field_0x98  = param_2;
    param_1            = s_overflow_on_node__d_1050_11ca + 0x8;
    iVar1->field_0x2   = &PTR_LOOP_1050_1040;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3a, param_7, param_5, param_6);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    return;
}

void  pass1_1038_de20(u32 param_1, u16 param_2, u16 param_3, i16 param_4, u8 *param_5, u16 param_6, u16 param_7)

{
    code      **ppcVar1;
    i16         iVar2;
    u8         *puVar3;
    u16         uVar4;
    u8          local_12[0x4];
    u16         uStack14;
    u8         *puStack12;
    u32 *puStack10;
    u16         uStack6;
    i16         iStack4;

    iStack4 = 0x644;
    uStack6 = 0x0;
    switch(param_4 + -0x11c)
    {
    case 0x0:
        iStack4 = 0x635;
        uStack6 = 0x3a;
        break;
    case 0x1:
        iStack4 = 0x636;
        uStack6 = 0x3b;
        break;
    case 0x2:
        iStack4 = 0x637;
        uStack6 = 0x3c;
        break;
    case 0x4:
        iStack4 = 0x639;
        uStack6 = 0x3e;
        break;
    case 0x5:
        iStack4 = 0x63a;
        uStack6 = 0x3f;
        break;
    case 0x6:
        iStack4 = 0x63b;
        uStack6 = 0x40;
        break;
    case 0x7:
        iStack4 = 0x640;
        uStack6 = 0x45;
        break;
    case 0x9:
        iStack4 = 0x642;
        uStack6 = 0x47;
        break;
    case 0xa:
        iStack4 = 0x641;
        uStack6 = 0x46;
        break;
    case 0xb:
        iStack4 = 0x63f;
        uStack6 = 0x44;
    }
    if(iStack4 != 0x0)
    {
        uVar4 = 0x1000;
        mem_op_1000_179c(0xb4, param_5, 0x1000);
        puVar3    = (param_5 | param_6);
        uStack14  = param_6;
        puStack12 = param_5;
        if(puVar3 == 0x0)
        {
            iVar2  = 0x0;
            puVar3 = 0x0;
        }
        else
        {
            uVar4 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            iVar2 = string_1040_8520(CONCAT22(param_5, param_6), (param_1 + 0x6), 0x0, 0x2, 0x634, iStack4, puVar3, param_7);
        }
        puStack10 = CONCAT22(puVar3, iVar2);
        if(uStack6 == 0x0)
        {
            ppcVar1 = (*puStack10 + 0x74);
            (**ppcVar1)(uVar4, iVar2, puVar3);
        }
        else
        {
            pass1_1008_941a(CONCAT22(param_7, local_12), 0x1, uStack6);
            ppcVar1 = (*puStack10 + 0x6c);
            (**ppcVar1)(0x1008, puStack10, (puStack10 >> 0x10), local_12, param_7);
        }
    }
    return;
}

void  pass1_1038_df86(u32 param_1, u8 *param_2, i16 param_3, u16 param_4)

{
    char       *pcVar1;
    code      **ppcVar2;
    BOOL16      BVar3;
    u16         uVar4;
    u16         uVar5;
    u8         *puVar6;
    u16         uVar7;
    u16         uVar8;
    u16         uVar9;
    u8          uVar10;
    u16        *puVar11;
    char       *pcVar12;
    Struct57 *paVar13;
    u32 *puStack22;

    puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    uVar5   = (puVar11 >> 0x10);
    pcVar1  = (puVar11 + 0x68);
    uVar9   = (param_1 >> 0x10);
    uVar8   = param_1;
    BVar3   = pass1_1010_041a();
    if(BVar3 != 0x0)
    {
        pass1_1010_038e(*(uVar8 + 0x92), 0x1, param_4);
        pass1_1038_af40(_PTR_LOOP_1050_5b7c, (uVar8 + 0x8), 0x1e, uVar5, uVar8, 0x1010, param_4);
        return;
    }
    pcVar12 = load_string_1010_847e(_PTR_LOOP_1050_14cc, (u16)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    puVar6  = (pcVar12 >> 0x10);
    uVar4   = pcVar12;
    uVar10  = 0x0;
    mem_op_1000_179c(0xb4, puVar6, 0x1000);
    if((puVar6 | uVar4) == 0x0)
    {
        uVar9 = 0x0;
        uVar7 = 0x0;
    }
    else
    {
        uVar10  = 0x40;
        paVar13 = pass1_1040_8478(CONCAT22(puVar6, uVar4), 0x20, pcVar1, pcVar12, (uVar8 + 0x6), puVar6 | uVar4);
        uVar7   = (paVar13 >> 0x10);
        uVar9   = SUB42(paVar13, 0x0);
    }
    puStack22 = CONCAT22(uVar7, uVar9);
    ppcVar2   = (*puStack22 + 0x74);
    (**ppcVar2)(uVar10, uVar9, uVar7);
    return;
}

Struct57 * pass1_1038_e140(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5)

{
    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfc2, param_5);
    param_1         = 0xe264;
    (param_1 + 0x2) = &PTR_LOOP_1050_1038;
    return param_1;
}

Struct57 * pass1_1038_e2d0(Struct57 *param_1, u16 param_2)

{
    u16 uVar1;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1c3, param_2);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x8e) = 0x0;
    param_1          = 0xe62e;
    (param_1 + 0x2)  = &PTR_LOOP_1050_1038;
    return param_1;
}

void  pass1_1038_e69a(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    astruct_713 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfcb, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_713 *)param_1;
    &iVar1->field_0x8e = 0x0;
    iVar1->field_0x92  = 0x0;
    param_1            = 0xe92e;
    iVar1->field_0x2   = &PTR_LOOP_1050_1038;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x43, param_8, param_6, param_7);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    return;
}

Struct57 * pass1_1038_e99a(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, u16 param_7)

{
    astruct_434 *iVar1;
    i16          unaff_DI;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfb9, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_434 *)param_1;
    &iVar1->field_0x8e = 0x0;
    param_1            = 0xeb32;
    iVar1->field_0x2   = &PTR_LOOP_1050_1038;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x30, param_7, param_6, unaff_DI);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    return param_1;
}

Struct57 * pass1_1038_eb9e(Struct57 *param_1, u16 param_2)

{
    u16 uVar1;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1c7, param_2);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x8e) = 0x0;
    param_1          = 0xee6e;
    (param_1 + 0x2)  = &PTR_LOOP_1050_1038;
    return param_1;
}

Struct57 * pass1_1038_cad8(Struct57 *param_1, u16 param_2, u8 *param_3, i16 param_4, u16 param_5)

{
    astruct_709 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1cb, param_2);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_709 *)param_1;
    &iVar1->field_0x8e = 0x0;
    param_1            = 0xcc9a;
    iVar1->field_0x2   = &PTR_LOOP_1050_1038;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2c, param_5, param_3, param_4);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    iVar1->field_0x74  = 0x0;
    return param_1;
}


void  pass1_1038_cd06(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    astruct_710 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfcc, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_710 *)param_1;
    &iVar1->field_0x8e = 0x0;
    iVar1->field_0x92  = 0x0;
    param_1            = 0xcf00;
    iVar1->field_0x2   = &PTR_LOOP_1050_1038;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x42, param_8, param_6, param_7);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    return;
}


void  make_proc_inst_1038_cf6c(u16 *param_1, u8 *param_2, LPVOID param_3)

{
    LPVOID pvVar1;
    i16    iVar2;
    u16    uVar3;

    uVar3                    = (param_1 >> 0x10);
    iVar2                    = param_1;
    *param_1                 = 0x389a;
    (iVar2 + 0x2)            = 0x1008;
    (iVar2 + 0x4)            = 0x0;
    (iVar2 + 0x8)            = 0x0;
    *param_1                 = 0xd23e;
    (iVar2 + 0x2)            = &PTR_LOOP_1050_1038;
    globals->_PTR_LOOP_1050_5bc8      = param_1;
    pvVar1                   = MakeProcInstance16(param_3, globals->PTR_LOOP_1050_038c);
    *(LPVOID *)(iVar2 + 0x4) = pvVar1;
    (iVar2 + 0x6)            = param_2;
    globals->PTR_LOOP_1050_5bcc       = MakeProcInstance16(s_tile2_bmp_1050_1538, globals->PTR_LOOP_1050_038c);
    globals->PTR_LOOP_1050_5bce       = param_2;
    return;
}


void  free_proc_inst_1038_cfda(u16 *param_1, LPVOID param_2)

{
    i16 iVar1;
    u16 uVar2;

    uVar2         = (param_1 >> 0x10);
    iVar1         = param_1;
    *param_1      = 0xd23e;
    (iVar1 + 0x2) = &PTR_LOOP_1050_1038;
    FreeProcInstance16(param_2);
    FreeProcInstance16(s_tile2_bmp_1050_1538);
    (iVar1 + 0x4) = 0x0;
    *param_1      = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    return;
}


Struct57 * pass1_1038_d242(Struct57 *param_1, u16 param_2)

{
    u16 uVar1;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x13e, param_2);
    uVar1            = (param_1 >> 0x10);
    param_1          = 0xd6ea;
    (param_1 + 0x2)  = &PTR_LOOP_1050_1038;
    (param_1 + 0x74) = 0x1;
    return param_1;
}


Struct57 * pass1_1038_d756(Struct57 *param_1, u16 param_2, u8 *param_3, i16 param_4, u16 param_5)

{
    code       **ppcVar1;
    astruct_711 *iVar2;
    u16          uVar2;
    u16         *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x11b, param_2);
    uVar2                      = (param_1 >> 0x10);
    iVar2                      = (astruct_711 *)param_1;
    iVar2->field_0x8e          = 0x0;
    iVar2->field_0x90          = 0x0;
    iVar2->field_0x92          = 0x0;
    iVar2->field_0x96          = 0x0;
    param_1                    = 0xe0d4;
    iVar2->field_0x2           = &PTR_LOOP_1050_1038;
    puVar3                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    &iVar2->field_0x92         = puVar3;
    (&iVar2->field_0x92 + 0x2) = (puVar3 >> 0x10);
    ppcVar1                    = (*iVar2->field_0x92 + 0x4);
    (**ppcVar1)();
    return param_1;
}


void  pass1_1038_b772(Struct57 *param_1, u8 *param_2, i16 param_3, u16 param_4, u16 param_5)

{
    u8          *puVar1;
    astruct_705 *iVar2;
    u16          uVar2;
    u16         *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x9a, 0x0, 0xfbf, param_5);
    uVar2              = (param_1 >> 0x10);
    iVar2              = (astruct_705 *)param_1;
    &iVar2->field_0x8e = 0x0;
    &iVar2->field_0x92 = 0x0;
    iVar2->field_0x96  = 0x1;
    iVar2->field_0x98  = 0x0;
    param_1            = 0xbd70;
    iVar2->field_0x2   = &PTR_LOOP_1050_1038;
    puVar3             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x36, param_4, param_2, param_3);
    puVar1             = (puVar3 >> 0x10);
    iVar2->field_0x8e  = puVar3;
    iVar2->field_0x90  = puVar1;
    puVar3             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x6, param_4, puVar1, param_3);
    iVar2->field_0x92  = puVar3;
    iVar2->field_0x94  = (puVar3 >> 0x10);
    return;
}


void  pass1_1038_bca8(u32 param_1)

{
    u16         uVar1;
    code      **ppcVar2;
    u32  uVar3;
    u32 *puVar4;
    u32 *puVar5;
    u8         *extraout_DX;
    u8         *puVar6;
    u8         *extraout_DX_00;
    u8         *puVar7;
    i16         iVar8;
    u16         uVar9;

    uVar9   = (param_1 >> 0x10);
    iVar8   = param_1;
    uVar3   = (iVar8 + 0x8e);
    puVar5  = (uVar3 + 0xa);
    ppcVar2 = (*puVar5 + 0x14);
    (**ppcVar2)();
    puVar4 = puVar5;
    puVar6 = extraout_DX;
    if((iVar8 + 0x70) != 0x0)
    {
        puVar4 = (iVar8 + 0x70);
        uVar1  = (iVar8 + 0x72);
        puVar6 = (uVar1 | puVar4);
        if(puVar6 != 0x0)
        {
            ppcVar2 = *puVar4;
            (**ppcVar2)();
            puVar6 = extraout_DX_00;
        }
    }
    mem_op_1000_179c(0x14, puVar6, 0x1000);
    puVar7 = (puVar6 | puVar4);
    if(puVar7 == 0x0)
    {
        puVar4 = 0x0;
        puVar7 = 0x0;
    }
    else
    {
        struct_1008_4c58(CONCAT22(puVar6, puVar4));
    }
    (iVar8 + 0x70) = puVar4;
    (iVar8 + 0x72) = puVar7;
    pass1_1008_4d84((iVar8 + 0x70), puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10, puVar7);
    return;
}


void  pass1_1038_bddc(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    astruct_706 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x176, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_706 *)param_1;
    &iVar1->field_0x8e = 0x0;
    iVar1->field_0x92  = 0x0;
    iVar1->field_0x94  = 0x0;
    iVar1->field_0x96  = 0x0;
    iVar1->field_0x98  = 0x0;
    iVar1->field_0x9a  = 0x0;
    iVar1->field_0x9c  = 0x0;
    param_1            = 0xc436;
    iVar1->field_0x2   = &PTR_LOOP_1050_1038;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3b, param_8, param_6, param_7);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    return;
}


void  pass1_1038_c4a2(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    astruct_708 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x17c, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_708 *)param_1;
    &iVar1->field_0x8e = 0x0;
    iVar1->field_0x92  = 0x0;
    iVar1->field_0x96  = 0x0;
    param_1            = 0xc74c;
    iVar1->field_0x2   = &PTR_LOOP_1050_1038;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3b, param_8, param_6, param_7);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    return;
}


Struct57 * pass1_1038_c7b8(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, u16 param_7)

{
    astruct_435 *iVar1;
    i16          unaff_DI;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfb8, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_435 *)param_1;
    &iVar1->field_0x8e = 0x0;
    param_1            = 0xca6c;
    iVar1->field_0x2   = &PTR_LOOP_1050_1038;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x5, param_7, param_6, unaff_DI);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    return param_1;
}


Struct57 * pass1_1038_9f76(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5)

{
    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfba, param_5);
    param_1         = 0xa0b6;
    (param_1 + 0x2) = &PTR_LOOP_1050_1038;
    return param_1;
}


u16 * pass1_1038_a122(i16 param_1, u16 param_2, u16 param_3, u32 param_4, u32 param_5)

{
    get_sys_metrics_1040_7728(CONCAT22(param_2, param_1), param_3, param_4, param_5, (param_5 >> 0x10));
    (param_1 + 0x8e)           = 0x0;
    CONCAT22(param_2, param_1) = 0xa2d0;
    (param_1 + 0x2)            = &PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}


Struct57 * pass1_1038_ab82(Struct57 *param_1, u16 param_2)

{
    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfd3, param_2);
    param_1         = 0xad72;
    (param_1 + 0x2) = &PTR_LOOP_1050_1038;
    return param_1;
}


void  pass1_1038_9144(u16 *param_1, u16 param_2, u16 param_3)

{
    u32   uVar1;
    u16          uVar2;
    u8          *in_DX;
    u8          *puVar3;
    u8          *puVar4;
    i16          iVar5;
    i16          iVar6;
    i16          unaff_DI;
    u16          uVar7;
    u16          uVar8;
    u16         *puVar9;
    i16         *piStack8;
    astruct_432 *iVar8;

    struct_1040_b082(NULL;
    uVar7          = (param_1 >> 0x10);
    iVar5          = param_1;
    (iVar5 + 0x94) = 0x0;
    (iVar5 + 0x96) = 0x0;
    (iVar5 + 0x98) = 0x0;
    *param_1       = 0x99a2;
    (iVar5 + 0x2)  = &PTR_LOOP_1050_1038;
    (iVar5 + 0x8a) = 0x27;
    puVar9         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x28, param_3, in_DX, unaff_DI);
    puVar3         = (puVar9 >> 0x10);
    uVar2          = puVar9;
    (iVar5 + 0x98) = uVar2;
    (iVar5 + 0x9a) = puVar3;
    mem_op_1000_179c(0x18, puVar3, 0x1000);
    puVar4 = (puVar3 | uVar2);
    if(puVar4 == 0x0)
    {
        (iVar5 + 0x90) = 0x0;
    }
    else
    {
        struct_1040_a598(CONCAT22(puVar3, uVar2));
        (iVar5 + 0x90) = uVar2;
        (iVar5 + 0x92) = puVar4;
    }
    (iVar5 + 0x90) = 0x11;
    iVar6          = (iVar5 + 0x90);
    uVar2          = iVar6 * 0xa + 0x2;
    mem_op_1000_179c(uVar2, puVar4, 0x1000);
    piStack8 = CONCAT22(puVar4, uVar2);
    if((puVar4 | uVar2) == 0x0)
    {
        uVar1         = (iVar5 + 0x90);
        (uVar1 + 0x2) = 0x0;
    }
    else
    {
        *piStack8 = iVar6;
        pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iVar6, 0xa, uVar2 + 0x2, puVar4);
        uVar1         = (iVar5 + 0x90);
        uVar8         = (uVar1 >> 0x10);
        iVar6         = uVar1;
        (iVar6 + 0x2) = uVar2 + 0x2;
        (iVar6 + 0x4) = puVar4;
    }
    uVar1          = (iVar5 + 0x90);
    (uVar1 + 0xa)  = 0x18;
    uVar1          = (iVar5 + 0x90);
    (uVar1 + 0x12) = (iVar5 + 0xa);
    return;
}


void  pass1_1038_78e2(u32 *param_1, u8 *param_2)

{
    u16          uVar1;
    u8          *puVar2;
    u8          *extraout_DX;
    u16          extraout_DX_00;
    u16          uVar3;
    astruct_431 *iVar4;
    u16          uVar4;

    uVar4               = (param_1 >> 0x10);
    iVar4               = (astruct_431 *)param_1;
    uVar1               = 0x0;
    *param_1            = 0x0;
    &iVar4->field_0x4   = 0x0;
    globals->_PTR_LOOP_1050_5a64 = param_1;
    mem_op_1000_179c(0xc, param_2, 0x1000);
    puVar2 = (param_2 | uVar1);
    if(puVar2 == 0x0)
    {
        *param_1 = 0x0;
    }
    else
    {
        set_struct_1008_574a(CONCAT22(param_2, uVar1));
        param_1          = uVar1;
        iVar4->field_0x2 = extraout_DX;
        puVar2           = extraout_DX;
    }
    mem_op_1000_179c(0xc, puVar2, 0x1000);
    if((puVar2 | uVar1) == 0x0)
    {
        uVar1 = 0x0;
        uVar3 = 0x0;
    }
    else
    {
        set_struct_1008_574a(CONCAT22(puVar2, uVar1));
        uVar3 = extraout_DX_00;
    }
    iVar4->field_0x4 = uVar1;
    iVar4->field_0x6 = uVar3;
    return;
}


void  pass1_1038_79b2(u32 param_1, u32 param_2, u16 param_3, u8 *param_4)

{
    code **ppcVar1;
    u16    uVar2;
    u16    uVar3;
    u16    uVar4;

    uVar4 = 0x1000;
    mem_op_1000_179c(0x14, param_4, 0x1000);
    uVar2 = param_4 | param_3;
    if(uVar2 == 0x0)
    {
        param_3 = 0x0;
        uVar2   = 0x0;
    }
    else
    {
        uVar4 = 0x1030;
        pass1_1030_aefa(CONCAT22(param_4, param_3), param_2);
    }
    uVar3   = (param_1 >> 0x10);
    ppcVar1 = ((param_1 + 0x4) + 0x4);
    (**ppcVar1)(uVar4, (param_1 + 0x4), param_3, uVar2);
    return;
}


Struct57 * pass1_1038_7d10(Struct57 *param_1, u16 param_2, u8 *param_3, i16 param_4, u16 param_5)

{
    astruct_703 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    struct_1040_b082(NULL;
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_703 *)param_1;
    &iVar1->field_0x94 = 0x0;
    param_1            = 0x8876;
    iVar1->field_0x2   = &PTR_LOOP_1050_1038;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x40, param_5, param_3, param_4);
    iVar1->field_0x94  = puVar2;
    iVar1->field_0x96  = (puVar2 >> 0x10);
    return param_1;
}


u32  pass1_1038_801a(u32 param_1, u8 *param_2, i16 param_3, u16 param_4)

{
    u16   uVar1;
    u16   uVar2;
    u16   uVar3;
    u16  *puVar4;
    char *pcVar5;
    u32   uVar6;

    puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x30, param_4, param_2, param_3);
    uVar3  = (param_1 >> 0x10);
    uVar2  = param_1;
    pcVar5 = pass1_1008_b340(*(uVar2 + 0x94));
    uVar1  = (pcVar5 >> 0x10) | pcVar5;
    uVar6  = pcVar5 & 0xffff | uVar1 << 0x10;
    if(pcVar5 != 0x0)
    {
        pass1_1010_3770(puVar4, pcVar5, uVar1);
        uVar6 = pass1_1038_af40(_PTR_LOOP_1050_5b7c, (uVar2 + 0x6), 0x3, uVar1, uVar2, 0x1010, param_4);
    }
    return uVar6;
}


void  pass1_1038_6b88(u16 param_1, u16 param_2, u16 *param_3, u32 *param_4, u8 *param_5, i16 param_6, u16 param_7)

{
    u32 *puVar1;
    u16  uVar2;
    u32  local_12[0x2];
    long lStack10;
    u16 *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_7, param_5, param_6);
    uVar2    = (puStack6 >> 0x10);
    lStack10 = (puStack6 + 0x20);
    puVar1   = local_12;
    pass1_1030_64ce(param_7, puVar1, uVar2, globals->_PTR_LOOP_1050_5740, param_3, lStack10, CONCAT22(param_7, puVar1));
    *param_4 = *puVar1;
    return;
}


void  pass1_1038_354a(u32 param_1, u16 param_2, u8 *param_3)

{
    u16          uVar1;
    astruct_424 *iVar1;
    u16          uVar2;

    uVar2 = (param_1 >> 0x10);
    iVar1 = (astruct_424 *)param_1;
    if(&iVar1->field_0x21a == 0x0)
    {
        mem_op_1000_179c(0xa, param_3, 0x1000);
        uVar1 = param_3 | param_2;
        if(uVar1 == 0x0)
        {
            &iVar1->field_0x21a = 0x0;
        }
        else
        {
            pass1_1030_9ecc(CONCAT22(param_3, param_2), param_1);
            &iVar1->field_0x21a = param_2;
            iVar1->field_0x21c  = uVar1;
        }
    }
    pass1_1030_9ef2(&iVar1->field_0x21a);
    return;
}


void  pass1_1038_35a8(u32 param_1, u16 param_2, u16 param_3, u8 *param_4)

{
    u16          uVar1;
    astruct_425 *iVar3;
    u16          uVar2;
    u16          unaff_SS;
    u8           in_AF;

    uVar2 = (param_1 >> 0x10);
    iVar3 = (astruct_425 *)param_1;
    if(&iVar3->field_0x21a == 0x0)
    {
        mem_op_1000_179c(0xa, param_4, 0x1000);
        uVar1 = param_4 | param_3;
        if(uVar1 == 0x0)
        {
            &iVar3->field_0x21a = 0x0;
        }
        else
        {
            pass1_1030_9ecc(CONCAT22(param_4, param_3), param_1);
            &iVar3->field_0x21a = param_3;
            iVar3->field_0x21c  = uVar1;
        }
    }
    pass1_1030_9f40(*&iVar3->field_0x21a, param_2, unaff_SS, in_AF);
    return;
}


void  pass1_1038_2944(u32 param_1, u16 param_2, u8 *param_3)

{
    u32 *puVar1;
    u32 *puVar2;
    u32 *puVar3;
    i16         iVar4;
    u32 *puVar5;
    u16         uVar6;
    u16        *puStack10;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if((param_3 | param_2) != 0x0)
    {
        *puStack10      = 0x389a;
        (param_2 + 0x2) = 0x1008;
        uVar6           = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        puVar3          = (param_1 + 0x8);
        puVar5          = (param_2 + 0x8);
        for(iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
        {
            puVar2  = puVar5;
            puVar5  = puVar5 + 0x1;
            puVar1  = puVar3;
            puVar3  = puVar3 + 0x1;
            *puVar2 = *puVar1;
        }
        *puStack10      = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10      = 0x29fe;
        (param_2 + 0x2) = &PTR_LOOP_1050_1038;
    }
    return;
}


void  pass1_1038_2b9a(u32 param_1, astruct_422 *param_2, u8 *param_3)

{
    u32  *puVar1;
    u32  *puVar2;
    i16          iVar3;
    astruct_421 *iVar5;
    u32  *puVar4;
    u32  *puVar5;
    u16          uVar6;
    u16         *puStack10;

    mem_op_1000_179c(0x118, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    iVar5     = (astruct_421 *)param_1;
    uVar6     = (param_1 >> 0x10);
    if((param_3 | param_2) != 0x0)
    {
        *puStack10         = 0x389a;
        param_2->field_0x2 = 0x1008;
        param_2->field_0x4 = iVar5->field_0x4;
        puVar4             = &iVar5->field_0x8;
        puVar5             = &param_2->field_0x8;
        for(iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3 + -0x1)
        {
            puVar2  = puVar5;
            puVar5  = puVar5 + 0x1;
            puVar1  = puVar4;
            puVar4  = puVar4 + 0x1;
            *puVar2 = *puVar1;
        }
        *puStack10           = 0x6ad2;
        param_2->field_0x2   = &USHORT_1050_1028;
        param_2->field_0x108 = iVar5->field_0x108;
        param_2->field_0x10c = iVar5->field_0x10c;
        param_2->field_0x110 = iVar5->field_0x110;
        param_2->field_0x114 = iVar5->field_0x114;
        *puStack10           = 0x309a;
        param_2->field_0x2   = &PTR_LOOP_1050_1038;
    }
    iVar5->field_0x114 = 0x0;
    iVar5->field_0x110 = 0x0;
    return;
}


void  pass1_1038_30aa(u16 *param_1, u16 param_2)

{
    u16         *puVar1;
    u8          *puVar2;
    u8          *puVar3;
    u16          uVar4;
    astruct_423 *iVar5;
    u16          uVar5;
    u16         *puVar6;

    puVar6              = struct_1030_17ce(param_1, 0x0, 0x0);
    puVar2              = (puVar6 >> 0x10);
    uVar5               = (param_1 >> 0x10);
    iVar5               = (astruct_423 *)param_1;
    iVar5->field_0x10   = 0x0;
    iVar5->field_0x14   = 0x0;
    iVar5->field_0x18   = 0x258;
    iVar5->field_0x1a_addr_offset = 0x258;
    iVar5->field_0x1c_addr_base = 0x0;
    iVar5->field_0x1e   = 0x0;
    iVar5->field_0x22   = 0x0;
    iVar5->field_0x24   = 0x32;
    &iVar5->field_0x1f6 = 0x0;
    iVar5->field_0x1fa  = 0x0;
    iVar5->field_0x1fe  = 0x0;
    iVar5->field_0x200  = 0x8000001;
    iVar5->field_0x204  = 0x0;
    iVar5->field_0x206  = 0x0;
    iVar5->field_0x208  = 0x1;
    iVar5->field_0x20a  = 0x0;
    iVar5->field_0x20c  = 0x0;
    iVar5->field_0x20e  = 0x0;
    iVar5->field_0x210  = 0x0;
    iVar5->field_0x214  = 0x0;
    iVar5->field_0x216  = 0x0;
    iVar5->field_0x21a  = 0x0;
    *param_1            = 0x6504;
    iVar5->field_0x2    = &PTR_LOOP_1050_1038;
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5->field_0x26), 0x0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5->field_0xba), 0x0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5->field_0x14e), 0x0, 0x54);
    puVar1 = pass1_1000_4906((param_1 & 0xffff0000 | &iVar5->field_0x1a2), 0x0, 0x54);
    mem_op_1000_179c(0x1b0, puVar2, 0x1000);
    puVar3 = (puVar2 | puVar1);
    if(puVar3 == 0x0)
    {
        &iVar5->field_0x1f6 = 0x0;
    }
    else
    {
        pass1_1030_314c(CONCAT22(puVar2, puVar1), iVar5->field_0x4, puVar3, param_2);
        iVar5->field_0x1f6 = puVar1;
        iVar5->field_0x1f8 = puVar3;
    }
    mem_op_1000_179c(0x1e, puVar3, 0x1000);
    uVar4 = puVar3 | puVar1;
    if(uVar4 == 0x0)
    {
        puVar1 = 0x0;
        uVar4  = 0x0;
    }
    else
    {
        struct_1020_c444(CONCAT22(puVar3, puVar1), 0x64, 0xc8);
    }
    iVar5->field_0xc = puVar1;
    iVar5->field_0xe = uVar4;
    return;
}


void  pass1_1038_3222(u16 *param_1, u32 param_2, u32 param_3, u16 param_4, u8 *param_5, u8 param_6, u8 *param_7)

{
    u16         *puVar1;
    u8          *puVar2;
    u16          uVar3;
    u16          uVar4;
    astruct_363 *iVar5;
    u16          uVar5;
    u16         *puVar6;
    u8           local_16[0x14];

    puVar6              = pass1_1030_183c(param_1, 0x0, 0x0, 0x4000000, param_3, param_4, param_5);
    puVar2              = (puVar6 >> 0x10);
    uVar5               = (param_1 >> 0x10);
    iVar5               = (astruct_363 *)param_1;
    iVar5->field_0x10   = param_2;
    iVar5->field_0x14   = 0x0;
    iVar5->field_0x18   = 0x258;
    iVar5->field_0x1a_addr_offset = 0x258;
    iVar5->field_0x1c_addr_base = 0x0;
    iVar5->field_0x1e   = 0x0;
    iVar5->field_0x22   = 0x0;
    iVar5->field_0x24   = 0x32;
    &iVar5->field_0x1f6 = 0x0;
    &iVar5->field_0x1fa = 0x0;
    iVar5->field_0x1fe  = 0x0;
    iVar5->field_0x200  = 0x8000001;
    iVar5->field_0x204  = 0x0;
    iVar5->field_0x206  = 0x0;
    iVar5->field_0x208  = 0x1;
    iVar5->field_0x20a  = 0x0;
    iVar5->field_0x20c  = 0x0;
    iVar5->field_0x20e  = 0x0;
    iVar5->field_0x210  = 0x0;
    iVar5->field_0x214  = 0x0;
    iVar5->field_0x216  = 0x0;
    iVar5->field_0x21a  = 0x0;
    *param_1            = 0x6504;
    iVar5->field_0x2    = &PTR_LOOP_1050_1038;
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5->field_0x26), 0x0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5->field_0xba), 0x0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5->field_0x14e), 0x0, 0x54);
    puVar1 = pass1_1000_4906((param_1 & 0xffff0000 | &iVar5->field_0x1a2), 0x0, 0x54);
    mem_op_1000_179c(0x1b0, puVar2, 0x1000);
    uVar3 = puVar2 | puVar1;
    if(uVar3 == 0x0)
    {
        &iVar5->field_0x1f6 = 0x0;
    }
    else
    {
        pass1_1030_314c(CONCAT22(puVar2, puVar1), *&iVar5->field_0x4, uVar3, param_7);
        iVar5->field_0x1f6 = puVar1;
        iVar5->field_0x1f8 = uVar3;
    }
    puVar2 = (iVar5->field_0x6 & 0xff);
    sys_1000_3f9c(local_16, param_7, 0x5a1a, &USHORT_1050_1050, *&iVar5->field_0x4, &stack0xfffe, uVar5, 0x1000, param_7, param_6);
    uVar3              = str_op_1008_60e8(CONCAT22(param_7, local_16), puVar2);
    iVar5->field_0x1fa = uVar3;
    iVar5->field_0x1fc = puVar2;
    mem_op_1000_179c(0x1e, puVar2, 0x1000);
    uVar4 = puVar2 | uVar3;
    if(uVar4 == 0x0)
    {
        &iVar5->field_0xc = 0x0;
    }
    else
    {
        struct_1020_c444(CONCAT22(puVar2, uVar3), 0x64, 0xc8);
        iVar5->field_0xc = uVar3;
        iVar5->field_0xe = uVar4;
    }
    return;
}


void  pass1_1038_19a0(u32 param_1, u32 *param_2, u32 param_3, u16 param_4, u8 param_5)

{
    fn_ptr_1 *ppcVar1;
    u32       uVar2;
    u16       uVar3;
    u16       uVar4;
    u8       *puVar5;
    u16       extraout_DX;
    fn_ptr_1 *ppcVar6;
    u32      *puVar7;
    u32      *puStack10;

    puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x2);
    puVar5 = (puVar7 >> 0x10);
    uVar3  = puVar7;
    pass1_1038_4d6e(param_3, puVar7, uVar3, puVar5);
    puStack10 = CONCAT22(puVar5, uVar3);
    uVar2     = *puStack10;
    ppcVar6   = uVar2;
    ppcVar1   = ppcVar6 + 0x8;
    uVar4     = uVar3;
    (**ppcVar1)(0x1008, uVar3, puVar5);
    if((extraout_DX | uVar4) == 0x0)
    {
        vspri16f_op_1030_840a(s_mineToSmelter__no_mines_1050_59df, 0x1030, param_4, 0x0);
        if(puStack10 != 0x0)
        {
            ppcVar1 = ppcVar6;
            (**ppcVar1)(0x1030, uVar3, puVar5, 0x1);
            return;
        }
    }
    else
    {
        pass1_1038_16f2(param_1, puStack10, param_2, extraout_DX | uVar4, ppcVar6, (uVar2 >> 0x10), 0x1008, param_4, param_5);
        if(puStack10 != 0x0)
        {
            ppcVar1 = *puStack10;
            (**ppcVar1)(0x1008, uVar3, puVar5, 0x1);
        }
    }
    return;
}


void  pass1_1038_008e(u16 param_1, u16 param_2, u32 param_3, u8 *param_4, i16 param_5, u16 param_6)

{
    i16        iVar1;
    u32 uVar2;
    u16        uVar3;
    u16        uVar4;
    u16        uVar5;
    u8        *puVar7;
    u8        *puVar8;
    i16        iVar9;
    u16        uVar10;
    u16       *puVar11;
    u16       *puVar12;
    i16        iStack32;
    i16        iStack12;
    u32 uVar6;

    uVar10 = (param_3 >> 0x10);
    iVar9  = param_3;
    if((iVar9 + 0x4) != 0x4000001)
    {
        return;
    }
    puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2c, param_6, param_4, param_5);
    puVar7  = (puVar11 >> 0x10);
    uVar3   = puVar11;
    puVar8  = puVar7;
    uVar4   = uVar3;
    pass1_1008_612e(0x1, 0x64, uVar3);
    iStack12 = 0x0;
    iVar1    = (uVar3 + 0xa);
    if(iVar1 == 0x1)
    {
        iStack12 = 0x15;
    }
    else
    {
        if(iVar1 != 0x2)
        {
            if(iVar1 == 0x3)
            {
                iStack12 = 0x16;
            }
            else
            {
                if(iVar1 == 0x4)
                {
                    if(uVar4 < 0x32)
                    {
                        iStack12 = 0x17;
                    }
                    else
                    {
                        iStack12 = 0x18;
                    }
                }
                else
                {
                    if(iVar1 == 0x5)
                    {
                        iStack12 = 0x19;
                    }
                }
            }
        }
    }
    if(iStack12 != 0x0)
    {
        puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_6, puVar8, param_5);
        puVar8  = (puVar12 >> 0x10);
        pass1_1010_043a(puVar12 & 0xffff | ZEXT24(puVar8) << 0x10, (iVar9 + 0x4), iStack12, param_6);
    }
    pass1_1008_eb74(puVar11, 0x0, puVar8, param_5, param_6);
    if((((uVar3 + 0xe) | (uVar3 + 0xc)) == 0x0) && ((iVar9 + 0x18) < 0xc9))
    {
        uVar2 = *_PTR_LOOP_1050_65e2;
        uVar4 = uVar2;
        uVar6 = uVar2;
        pass1_1008_612e(0x0, 0x8, uVar4);
        uVar5         = uVar6;
        iStack32      = (uVar2 >> 0x10);
        (uVar3 + 0xc) = uVar5 + uVar4 + 0x1e;
        (uVar3 + 0xe) = (uVar5 >> 0xf) + iStack32 + CARRY2(uVar5, uVar4) + (0xffe1 < uVar5 + uVar4);
    }
    return;
}


astruct_100 * pass1_1038_0ba6(astruct_100 *param_1, i16 param_2, u16 param_3, u8 param_4)

{
    u8          *puVar1;
    astruct_701 *iVar2;
    u16          uVar2;
    astruct_100 *paVar3;
    u16         *puVar4;

    paVar3              = struct_op_1028_d1dc(param_3, param_4, param_1, 0x270f);
    puVar1              = (paVar3 >> 0x10);
    uVar2               = (param_1 >> 0x10);
    iVar2               = (astruct_701 *)param_1;
    &iVar2->field_0x108 = 0x0;
    param_1->field_0x0  = 0x1c2e;
    iVar2->field_0x2    = &PTR_LOOP_1050_1038;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | &iVar2->field_0x8), s_SCMove_1050_59d8);
    puVar4             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_3, puVar1, param_2);
    iVar2->field_0x108 = puVar4;
    iVar2->field_0x10a = (puVar4 >> 0x10);
    return param_1;
}


void  pass1_1038_0cf0(u32 param_1, u16 param_2, u8 *param_3)

{
    u32 *puVar1;
    u32 *puVar2;
    u32 *puVar3;
    i16         iVar4;
    i16         iVar5;
    u32 *puVar6;
    u16         uVar7;
    u16        *puStack10;

    mem_op_1000_179c(0x10c, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if((param_3 | param_2) != 0x0)
    {
        *puStack10      = 0x389a;
        (param_2 + 0x2) = 0x1008;
        uVar7           = (param_1 >> 0x10);
        iVar5           = param_1;
        (param_2 + 0x4) = (iVar5 + 0x4);
        puVar3          = (iVar5 + 0x8);
        puVar6          = (param_2 + 0x8);
        for(iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
        {
            puVar2  = puVar6;
            puVar6  = puVar6 + 0x1;
            puVar1  = puVar3;
            puVar3  = puVar3 + 0x1;
            *puVar2 = *puVar1;
        }
        *puStack10        = 0x6ad2;
        (param_2 + 0x2)   = &USHORT_1050_1028;
        (param_2 + 0x108) = (iVar5 + 0x108);
        *puStack10        = 0x1c2e;
        (param_2 + 0x2)   = &PTR_LOOP_1050_1038;
    }
    return;
}


void  pass1_1030_e1f4(u32 param_1, u16 param_2, u8 *param_3)

{
    u32 *puVar1;
    u32 *puVar2;
    u32 *puVar3;
    i16         iVar4;
    u32 *puVar5;
    u16         uVar6;
    u16        *puStack10;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if((param_3 | param_2) != 0x0)
    {
        *puStack10      = 0x389a;
        (param_2 + 0x2) = 0x1008;
        uVar6           = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        puVar3          = (param_1 + 0x8);
        puVar5          = (param_2 + 0x8);
        for(iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
        {
            puVar2  = puVar5;
            puVar5  = puVar5 + 0x1;
            puVar1  = puVar3;
            puVar3  = puVar3 + 0x1;
            *puVar2 = *puVar1;
        }
        *puStack10      = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10      = 0xe2ae;
        (param_2 + 0x2) = 0x1030;
    }
    return;
}
