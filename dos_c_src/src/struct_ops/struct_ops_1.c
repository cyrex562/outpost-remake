#include <types.h>
#include <structs/structs_37.h>
#include <structs/structs_45.h>
#include <structs/structs_44.h>
#include "globals.h"
#include "utils.h"
#include "unk/unk_5.h"
#include "struct_ops_5.h"
#include <structs/structs_14.h>

astruct_442* struct_1040_bf3e(Globals* globals, astruct_442 *param_1, u16 param_2)

{
    //astruct_442 *iVar1;
    //u16          uVar1;

    //uVar1            = (param_1 >> 0x10);
    //iVar1            = (astruct_442 *)param_1;
    param_1->field_0x0         = 0x389a;
    param_1->field_0x2 = 0x1008;
    param_1->field_0x0         = 0x3aa8;
    param_1->field_0x2         = 0x1008;
    param_1->field_0x4         = param_2;
    param_1->field_0x0         = 0x3ab0;
    param_1->field_0x2         = 0x1008;
    param_1->field_0x6         = 0x0;
    param_1->field_0x0         = 0xc53e;
    param_1->field_0x2 = &globals->PTR_LOOP_1050_1040;
    return param_1;
}


void pass1_1040_b7ee(Globals* globals, Struct57 *param_1, long param_2, u16 param_3)

{
    //i16 iVar1;
    //u16 uVar2;
    u16 uVar3;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_3, 0xfab));
    //uVar2          = (param_1 >> 0x10);
    //iVar1          = param_1;
    (param_1->field_0x94) = 0x0;
    (param_1->field_0x98) = 0x0;
    (param_1->field_0xb0) = 0x0;
    (param_1->field_0xb4) = 0x0;
    (param_1->field_0xb6) = 0x0;
    param_1->field_0x0        = 0xbeba;
    (param_1->field_0x2)  = &globals->PTR_LOOP_1050_1040;
    if(param_2 != 0x0)
    {
        uVar3          = (param_2 >> 0x10);
        (param_1->field_0xb0) = (param_2 + 0x6);
        (param_1->field_0xb4) = (param_2 + 0x14);
    }
    return;
}


void pass1_1040_a640(Globals* globals, Struct57 *param_1, u32 param_2, u16 param_3)

{
    /*i16 iVar1;
    u16 uVar2;*/

    struct_1040_b082(param_1, CONCAT22(param_3, 0x1f1));
    //uVar2           = (param_1 >> 0x10);
    //iVar1           = param_1;
    *(param_1->field_0x94) = param_2;
    (param_1->field_0x98)  = 0x0;
    (param_1->field_0xea)  = 0x0;
    param_1         = 0xac08;
    (param_1->field_0x2)   = &globals->PTR_LOOP_1050_1040;
    return;
}


void  struct_1040_a598(u16 *param_1)

{
    astruct_259 *iVar1;
    u16          uVar1;

    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_259 *)param_1;
    *param_1          = 0x0;
    iVar1->field_0x2  = 0x0;
    iVar1->field_0x6  = 0x0;
    iVar1->field_0xa  = 0x0;
    iVar1->field_0xc  = 0x0;
    iVar1->field_0x10 = 0x0;
    iVar1->field_0x12 = 0x0;
    iVar1->field_0x14 = 0x0;
    iVar1->field_0x16 = 0x0;
    return;
}


void  pass1_1040_a564(u32 *param_1)

{
    u16 uVar1;

    uVar1           = (param_1 >> 0x10);
    *param_1        = 0x0;
    (param_1 + 0x4) = 0x0;
    (param_1 + 0x6) = 0x0;
    return;
}


void  pass1_1040_9824(u32 *param_1)

{
    i16 iVar1;
    u16 uVar2;

    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    *param_1       = 0x0;
    (iVar1 + 0x4)  = 0x0;
    (iVar1 + 0x56) = 0x0;
    (iVar1 + 0x5a) = 0x0;
    (iVar1 + 0x5c) = 0x0;
    *(iVar1 + 0x6) = 0x0;
    return;
}


void  pass1_1040_4e94(Struct57 *param_1, long param_2, u16 param_3)

{
    i16 iVar1;
    u16 uVar2;
    u16 uVar3;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_3, 0xfab));
    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    (iVar1 + 0x94) = 0x0;
    (iVar1 + 0x98) = 0x0;
    (iVar1 + 0xb0) = 0x0;
    (iVar1 + 0xb4) = 0x0;
    (iVar1 + 0xb6) = 0x0;
    param_1        = 0x55a2;
    (iVar1 + 0x2)  = &PTR_LOOP_1050_1040;
    if(param_2 != 0x0)
    {
        uVar3          = (param_2 >> 0x10);
        (iVar1 + 0xb0) = (param_2 + 0x6);
        (iVar1 + 0xb4) = (param_2 + 0x14);
    }
    return;
}


Address2 pass1_1040_5d12(Globals *globals, Struct382 *param_1)

{
    u16          u16_addr_offset_1;
    u16          u16_addr_base_2;
    astruct_440*   pstruct_440_var_3;
    astruct_440 *pstruct_440_var_4;
    //u16          uVar4;
    Address2 address_2_var_5 = {.offset=0,.base=0};

    pstruct_440_var_3 = (param_1->field_0x90);
    //uVar4 = (pstruct_440_var_3 >> 0x10);
    pstruct_440_var_4 = pstruct_440_var_3;
    u16_addr_offset_1 = pstruct_440_var_4->field_0x6_addr_offset;
    u16_addr_base_2 = pstruct_440_var_4->field_0x8_addr_base;
    if((u16_addr_base_2 | u16_addr_offset_1) != 0x0)
    {
        Struct383* struct_addr_1 = (Struct383*)( CONCAT22(u16_addr_base_2, u16_addr_offset_1));
        address_2_var_5          = struct_op_1030_73a8(globals, struct_addr_1);
        return address_2_var_5;
    }
    return address_2_var_5;
}


void  pass1_1040_3966(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    astruct_722 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x185, param_5);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_722 *)param_1;
    &iVar1->field_0x8e = 0x0;
    iVar1->field_0x92  = 0x0;
    iVar1->field_0x96  = 0x0;
    iVar1->field_0x9a  = 0x0;
    iVar1->field_9c    = 0x0;
    iVar1->field_9e  = 0x0;
    iVar1->field_0xa0  = 0x0;
    iVar1->field_0xa2  = 0x0;
    iVar1->field_0xa4  = 0x5;
    param_1            = 0x3ffc;
    iVar1->field_0x2   = &PTR_LOOP_1050_1040;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3c, param_8, param_6, param_7);
    iVar1->field_0x8e  = puVar2;
    iVar1->field_0x90  = (puVar2 >> 0x10);
    return;
}


Struct57 * pas1_1040_29c2(Struct57 *param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5)

{
    i16 iVar1;
    u16 uVar2;

    pass1_1040_b0bc(param_1, param_2, CONCAT22(param_3, 0x157));
    uVar2         = (param_1 >> 0x10);
    iVar1         = param_1;
    param_1       = 0x2e26;
    (iVar1 + 0x2) = &PTR_LOOP_1050_1040;
    load_string_1010_84ac(_PTR_LOOP_1050_14cc, (u16)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    (iVar1 + 0x94) = param_4;
    (iVar1 + 0x96) = param_5;
    load_string_1010_84ac(_PTR_LOOP_1050_14cc, (u16)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    (iVar1 + 0x98) = param_4;
    (iVar1 + 0x9a) = param_5;
    return param_1;
}


void  pass1_1040_2dac(u32 param_1)

{
    u32 uVar1;
    u32        uVar2;
    i16        iStack10;

    uVar1 = (param_1 + 0x90);
    uVar2 = struct_op_1030_73a8(globals,NULL;
    for(iStack10 = 0x0; iStack10 < 0x5; iStack10 = iStack10 + 0x1)
    {
        pass1_1028_4ab2(uVar2, (&PTR_LOOP_1050_5d04 + iStack10 * 0xc), (iStack10 * 0xc + 0x5d02));
    }
    return;
}


void  pass1_1038_88f2(Struct57 *param_1, u16 param_2)

{
    i16 iVar1;
    u16 uVar2;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x184c));
    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    (iVar1 + 0x94) = globals->_PTR_LOOP_1050_5a68;
    (iVar1 + 0x98) = 0x0;
    (iVar1 + 0x9a) = 0x0;
    (iVar1 + 0x9c) = 0x0;
    (iVar1 + 0x9e) = 0x0;
    param_1        = 0x8c2e;
    (iVar1 + 0x2)  = &PTR_LOOP_1050_1038;
    return;
}


Struct57 * pass1_1038_8caa(Struct57 *param_1, u16 param_2, u8 *param_3, i16 param_4, u16 param_5)

{
    astruct_704 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x185a));
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_704 *)param_1;
    &iVar1->field_0x94 = 0x0;
    param_1            = 0x90c8;
    iVar1->field_0x2   = &PTR_LOOP_1050_1038;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3f, param_5, param_3, param_4);
    iVar1->field_0x94  = puVar2;
    iVar1->field_0x96  = (puVar2 >> 0x10);
    return param_1;
}


void  struct_1038_6520(u16 *param_1)

{
    astruct_308 *iVar1;
    u16          uVar1;

    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_308 *)param_1;
    *param_1          = 0x389a;
    iVar1->field_0x2  = 0x1008;
    iVar1->field_0x4  = 0x0;
    iVar1->field_0x8  = 0x0;
    iVar1->field_0xc  = 0x0;
    iVar1->field_0xe  = 0x0;
    iVar1->field_0x12 = 0x0;
    iVar1->field_0x14 = 0x0;
    iVar1->field_0x16 = 0x0;
    pass1_1008_3e38((param_1 & 0xffff0000 | &iVar1->field_0x1a_addr_offset));
    iVar1->field_0x20 = 0x0;
    iVar1->field_0x24 = 0x0;
    iVar1->field_0x26 = 0x0;
    iVar1->field_0x28 = 0x0;
    *param_1          = 0x78de;
    iVar1->field_0x2  = &PTR_LOOP_1050_1038;
    return;
}


void  pass1_1038_6590(u16 *param_1, u16 param_2, u16 param_3, u16 param_4, u16 param_5, u32 param_6)

{
    u16          uVar1;
    i16          iVar2;
    astruct_410 *iVar3;
    u16          uVar3;
    u16          unaff_SS;
    u16         *puVar4;
    u32          uVar5;

    uVar3              = (param_1 >> 0x10);
    iVar3              = (astruct_410 *)param_1;
    *param_1           = 0x389a;
    iVar3->field_0x2   = 0x1008;
    &iVar3->field_0x4  = 0x0;
    iVar3->field_0x8   = param_6;
    iVar3->field_0xc   = param_4;
    iVar3->field_0xe   = 0x0;
    iVar3->field_0x12  = 0x0;
    iVar3->field_0x14  = 0x0;
    iVar3->field_0x16  = param_2;
    iVar3->field_0x18  = param_3;
    puVar4             = pass1_1008_3e38((param_1 & 0xffff0000 | &iVar3->field_0x1a_addr_offset));
    uVar1              = (puVar4 >> 0x10);
    &iVar3->field_0x20 = 0x0;
    iVar3->field_0x24  = 0x0;
    iVar3->field_0x26  = param_5;
    iVar3->field_0x28  = 0x0;
    *param_1           = 0x78de;
    iVar3->field_0x2   = &PTR_LOOP_1050_1038;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_6, (param_6 >> 0x10));
    uVar5            = pass1_1030_6d4e(CONCAT22(uVar1, param_5), param_5, uVar1, unaff_SS);
    iVar2            = (uVar5 >> 0x10);
    iVar3->field_0x4 = uVar5;
    iVar3->field_0x6 = iVar2;
    puVar4           = (param_1 & 0xffff0000 | &iVar3->field_0x1a_addr_offset);
    pass1_1008_3f62(puVar4, CONCAT22(uVar1, param_5 + 0xc));
    uVar1 = puVar4;
    pass1_1010_8fba(*&iVar3->field_0x4, uVar1);
    iVar3->field_0x20 = uVar1;
    iVar3->field_0x22 = iVar2;
    return;
}


void  pass1_1038_666e(u16 *param_1, long *param_2, u16 param_3, u32 param_4)

{
    u16          uVar1;
    u16          uVar2;
    astruct_420 *iVar3;
    u16          uVar3;
    u16          unaff_SS;
    u16         *puVar4;
    u32          uVar5;

    uVar3              = (param_1 >> 0x10);
    iVar3              = (astruct_420 *)param_1;
    *param_1           = 0x389a;
    iVar3->field_0x2   = 0x1008;
    iVar3->field_0x4   = 0x0;
    iVar3->field_0x8   = param_4;
    iVar3->field_0xc   = 0x0;
    iVar3->field_0xe   = param_2;
    iVar3->field_0x12  = 0x0;
    iVar3->field_0x14  = 0x0;
    iVar3->field_0x18  = 0x0;
    iVar3->field_0x16  = 0x0;
    puVar4             = pass1_1008_3e38((param_1 & 0xffff0000 | &iVar3->field_0x1a_addr_offset));
    uVar1              = (puVar4 >> 0x10);
    &iVar3->field_0x20 = 0x0;
    iVar3->field_0x24  = 0x0;
    iVar3->field_0x26  = param_3;
    iVar3->field_0x28  = 0x0;
    *param_1           = 0x78de;
    iVar3->field_0x2   = &PTR_LOOP_1050_1038;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_4, (param_4 >> 0x10));
    uVar5                     = pass1_1030_6d4e(CONCAT22(uVar1, param_3), param_3, uVar1, unaff_SS);
    uVar2                     = (uVar5 >> 0x10);
    &iVar3->field_0x4         = uVar5;
    (&iVar3->field_0x4 + 0x2) = uVar2;
    puVar4                    = (param_1 & 0xffff0000 | &iVar3->field_0x1a_addr_offset);
    pass1_1008_3f62(puVar4, CONCAT22(uVar1, param_3 + 0xc));
    uVar1 = puVar4;
    pass1_1010_8fba(iVar3->field_0x4, uVar1);
    iVar3->field_0x20 = uVar1;
    iVar3->field_0x22 = uVar2;
    pass1_1020_ba94(param_2);
    iVar3->field_0x16 = uVar1;
    iVar3->field_0x18 = uVar2;
    return;
}


void  pass1_1038_675c(u16 *param_1, u32 param_2, u16 param_3, u16 param_4, u32 param_5)

{
    u16          uVar1;
    u16          uVar2;
    astruct_414 *iVar3;
    u16          uVar3;
    u16          unaff_SS;
    u16         *puVar4;
    u32          uVar5;

    uVar3              = (param_1 >> 0x10);
    iVar3              = (astruct_414 *)param_1;
    *param_1           = 0x389a;
    iVar3->field_0x2   = 0x1008;
    iVar3->field_0x4   = 0x0;
    iVar3->field_0x8   = param_5;
    iVar3->field_0xc   = 0x0;
    iVar3->field_0xe   = 0x0;
    iVar3->field_0x12  = param_3;
    iVar3->field_0x14  = 0x0;
    iVar3->field_0x16  = param_2;
    puVar4             = pass1_1008_3e38((param_1 & 0xffff0000 | &iVar3->field_0x1a_addr_offset));
    uVar1              = (puVar4 >> 0x10);
    &iVar3->field_0x20 = 0x0;
    iVar3->field_0x24  = 0x0;
    iVar3->field_0x26  = param_4;
    iVar3->field_0x28  = 0x0;
    *param_1           = 0x78de;
    iVar3->field_0x2   = &PTR_LOOP_1050_1038;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, (param_5 >> 0x10));
    uVar5                     = pass1_1030_6d4e(CONCAT22(uVar1, param_4), param_4, uVar1, unaff_SS);
    uVar2                     = (uVar5 >> 0x10);
    &iVar3->field_0x4         = uVar5;
    (&iVar3->field_0x4 + 0x2) = uVar2;
    puVar4                    = (param_1 & 0xffff0000 | &iVar3->field_0x1a_addr_offset);
    pass1_1008_3f62(puVar4, CONCAT22(uVar1, param_4 + 0xc));
    uVar1 = puVar4;
    pass1_1010_8fba(iVar3->field_0x4, uVar1);
    iVar3->field_0x20 = uVar1;
    iVar3->field_0x22 = uVar2;
    return;
}


void  pass1_1038_6838(u16 *param_1, u32 param_2, u16 param_3, u16 param_4, u32 param_5)

{
    u16          uVar1;
    u16          uVar2;
    astruct_415 *iVar3;
    u16          uVar3;
    u16          unaff_SS;
    u16         *puVar4;
    u32          uVar5;

    uVar3              = (param_1 >> 0x10);
    iVar3              = (astruct_415 *)param_1;
    *param_1           = 0x389a;
    iVar3->field_0x2   = 0x1008;
    iVar3->field_0x4   = 0x0;
    iVar3->field_0x8   = param_5;
    iVar3->field_0xc   = 0x0;
    iVar3->field_0xe   = 0x0;
    iVar3->field_0x12  = 0x0;
    iVar3->field_0x14  = param_3;
    iVar3->field_0x16  = param_2;
    puVar4             = pass1_1008_3e38((param_1 & 0xffff0000 | &iVar3->field_0x1a_addr_offset));
    uVar1              = (puVar4 >> 0x10);
    &iVar3->field_0x20 = 0x0;
    iVar3->field_0x24  = 0x0;
    iVar3->field_0x26  = param_4;
    iVar3->field_0x28  = 0x0;
    *param_1           = 0x78de;
    iVar3->field_0x2   = &PTR_LOOP_1050_1038;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, (param_5 >> 0x10));
    uVar5                     = pass1_1030_6d4e(CONCAT22(uVar1, param_4), param_4, uVar1, unaff_SS);
    uVar2                     = (uVar5 >> 0x10);
    &iVar3->field_0x4         = uVar5;
    (&iVar3->field_0x4 + 0x2) = uVar2;
    puVar4                    = (param_1 & 0xffff0000 | &iVar3->field_0x1a_addr_offset);
    pass1_1008_3f62(puVar4, CONCAT22(uVar1, param_4 + 0xc));
    uVar1 = puVar4;
    pass1_1010_8fba(iVar3->field_0x4, uVar1);
    iVar3->field_0x20 = uVar1;
    iVar3->field_0x22 = uVar2;
    return;
}


void  pass1_1038_69fe(u32 param_1)

{
    (param_1 + 0x28) = 0x0;
    return;
}


void  pass1_1038_6c68(u32 param_1, u16 *param_2, u32 *param_3, i16 param_4, u8 *param_5, i16 param_6, u16 param_7)

{
    i16  iVar1;
    u16  uVar2;
    u16  uVar3;
    u16 *puVar4;
    u16  uVar5;
    u16  uVar6;
    u8  *puVar7;
    u16  uVar8;
    u16 *puVar9;
    u32  uVar10;
    i16  iStack30;

    uVar2 = param_1;
    pass1_1008_3f62(param_2, (param_1 & 0xffff0000 | (uVar2 + 0x1a)));
    puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_7, param_5, param_6);
    uVar5  = (puVar9 >> 0x10);
    puVar4 = (param_1 & 0xffff0000 | (uVar2 + 0x1a));
    pass1_1030_627e(param_7, uVar2 + 0x1a, uVar5, globals->_PTR_LOOP_1050_5740, puVar4, (puVar9 + 0x20));
    uVar3 = puVar4;
    uVar6 = uVar5 | uVar3;
    if(uVar6 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, uVar5);
        uVar10 = struct_op_1030_73a8(globals,NULL;
        puVar7 = (uVar10 >> 0x10);
        iVar1  = (uVar10 + 0xc);
        if((iVar1 == 0x47) || (iVar1 == 0x6a))
        {
            uVar8    = (param_1 >> 0x10);
            iStack30 = (uVar2 + 0x1e);
            if(param_4 < 0x0)
            {
                iStack30 = iStack30 + -0x1;
            }
            else
            {
                iStack30 = iStack30 + 0x1;
            }
            (param_2 + 0x4) = iStack30;
            pass1_1038_6b88(uVar2, uVar8, param_2, param_3, puVar7, param_6, param_7);
        }
    }
    return;
}


void  pass1_1038_709c(u32 param_1, u32 param_2, u8 *param_3, u16 param_4)

{
    u32         *puVar1;
    long         lVar2;
    u16          uVar7;
    u16          uVar8;
    u8          *puVar9;
    astruct_618 *iVar8;
    i16          iVar10;
    u16          uVar11;
    u16          uVar12;
    astruct_99  *paStack40;
    astruct_99  *paStack16;
    u16          uStack12;
    long         local_a;
    u16          local_6;
    u16          uStack4;
    astruct_617 *uVar3;
    astruct_619 *uVar4;
    astruct_620 *uVar5;
    astruct_621 *uVar6;

    uVar11 = (param_1 >> 0x10);
    iVar8  = (astruct_618 *)param_1;
    if(((&iVar8->field_0xe + 0x2) | &iVar8->field_0xe) == 0x0)
    {
        if(iVar8->field_0xc == 0x0)
        {
            if(iVar8->field_0x12 == 0x0)
            {
                if(iVar8->field_0x14 == 0x0)
                {
                    return;
                }
                paStack40 = pass1_1000_07fc(0x1000, globals->PTR_LOOP_1050_68a2);
                uVar8     = (paStack40 >> 0x10);
                uVar3     = (astruct_617 *)paStack40;
                if((uVar8 | uVar3) == 0x0)
                {
                    paStack40 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack40->field_0x0 = 0x389a;
                    uVar3->field_0x2     = 0x1008;
                    uVar3->field_0x4     = 0x0;
                    uVar3->field_0x6     = 0x0;
                    uVar3->field_0x8     = 0x0;
                    uVar3->field_0xa     = 0x0;
                    uVar3->field_0xc     = 0x0;
                    paStack40->field_0x0 = 0x56ce;
                    uVar3->field_0x2     = 0x1018;
                }
                uVar12            = (paStack40 >> 0x10);
                (paStack40 + 0x8) = iVar8->field_0x14;
                (paStack40 + 0xa) = &iVar8->field_0x16;
                uVar8             = pass1_1020_c42e(iVar8->field_0x14);
            }
            else
            {
                pass1_1030_7c50(param_2, iVar8->field_0x16, iVar8->field_0x12, 0x0, param_3);
                paStack40 = pass1_1000_07fc(0x1000, globals->PTR_LOOP_1050_68a2);
                uVar8     = (paStack40 >> 0x10);
                uVar4     = (astruct_619 *)paStack40;
                if((uVar8 | uVar4) == 0x0)
                {
                    paStack40 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack40->field_0x0 = 0x389a;
                    uVar4->field_0x2     = 0x1008;
                    uVar4->field_0x4     = 0x0;
                    uVar4->field_0x6     = 0x0;
                    uVar4->field_0x8     = 0x0;
                    uVar4->field_0xa     = 0x0;
                    uVar4->field_0xc     = 0x0;
                    paStack40->field_0x0 = 0x56ce;
                    uVar4->field_0x2     = 0x1018;
                }
                uVar12            = (paStack40 >> 0x10);
                (paStack40 + 0x6) = iVar8->field_0x12;
                (paStack40 + 0xa) = &iVar8->field_0x16;
                uVar8             = switch_1020_c3b4(iVar8->field_0x12);
            }
            uVar12 = (paStack40 >> 0x10);
            iVar10 = paStack40;
            lVar2  = uVar8 * (iVar10 + 0xa);
            puVar9 = (lVar2 >> 0x10);
            uVar8  = lVar2;
        }
        else
        {
            paStack40 = pass1_1000_07fc(0x1000, globals->PTR_LOOP_1050_68a2);
            uVar8     = (paStack40 >> 0x10);
            uVar5     = (astruct_620 *)paStack40;
            puVar9    = (uVar8 | uVar5);
            if(puVar9 == 0x0)
            {
                paStack40 = (astruct_99 *)0x0;
            }
            else
            {
                paStack40->field_0x0 = 0x389a;
                uVar5->field_0x2     = 0x1008;
                uVar5->field_0x4     = 0x0;
                uVar5->field_0x6     = 0x0;
                uVar5->field_0x8     = 0x0;
                uVar5->field_0xa     = 0x0;
                uVar5->field_0xc     = 0x0;
                paStack40->field_0x0 = 0x56ce;
                uVar5->field_0x2     = 0x1018;
            }
            uVar12         = (paStack40 >> 0x10);
            iVar10         = paStack40;
            (iVar10 + 0x4) = iVar8->field_0xc;
            uVar8          = &iVar8->field_0x16;
            (iVar10 + 0xa) = uVar8;
        }
        (iVar10 + 0xc) = uVar8;
        pass1_1030_6a2c(param_2, CONCAT22(uVar12, iVar10), uVar8, puVar9, param_4);
    }
    else
    {
        puVar1  = iVar8->field_0xe;
        uStack4 = (puVar1 + 0x4);
        for(uStack12 = 0x0; uStack12 < uStack4; uStack12 = uStack12 + 0x1)
        {
            pass1_1020_bb16(iVar8->field_0xe, CONCAT22(param_4, &local_a), CONCAT22(param_4, &local_6), uStack12);
            if(local_a != 0x0)
            {
                paStack16 = pass1_1000_07fc(0x1000, globals->PTR_LOOP_1050_68a2);
                uVar8     = (paStack16 >> 0x10);
                uVar6     = (astruct_621 *)paStack16;
                if((uVar8 | uVar6) == 0x0)
                {
                    paStack16 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack16->field_0x0 = 0x389a;
                    uVar6->field_0x2     = 0x1008;
                    uVar6->field_0x4     = 0x0;
                    uVar6->field_0x6     = 0x0;
                    uVar6->field_0x8     = 0x0;
                    uVar6->field_0xa     = 0x0;
                    uVar6->field_0xc     = 0x0;
                    paStack16->field_0x0 = 0x56ce;
                    uVar6->field_0x2     = 0x1018;
                }
                uVar12         = (paStack16 >> 0x10);
                iVar10         = paStack16;
                (iVar10 + 0x4) = local_6;
                (iVar10 + 0xa) = local_a;
                uVar7          = pass1_1020_c3ae();
                lVar2          = uVar7 * (iVar10 + 0xa);
                uVar8          = lVar2;
                (iVar10 + 0xc) = uVar8;
                pass1_1030_6a2c(param_2, (long)paStack16, uVar8, (lVar2 >> 0x10), param_4);
            }
        }
    }
    return;
}


void  pass1_1038_7356(u32 param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5)

{
    u8         **ppuVar1;
    u16         *puVar2;
    u32   uVar3;
    Struct18  *paVar4;
    long         lVar5;
    BOOL16       BVar6;
    u16          uVar7;
    u16          uVar9;
    u8          *puVar10;
    u8          *puVar11;
    astruct_615 *iVar9;
    i16          iVar12;
    u16          uVar13;
    u16          uVar14;
    bool         bVar15;
    u32          uVar16;
    u32          uVar17;
    astruct_99  *paStack50;
    astruct_99  *paStack26;
    astruct_616 *uVar8;
    astruct_622 *uVar10;

    uVar16  = struct_op_1030_73a8(globals,NULL;
    puVar10 = (uVar16 >> 0x10);
    uVar7   = uVar16;
    puVar11 = puVar10;
    BVar6   = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar7 + 0xc), 0x4);
    iVar9   = (astruct_615 *)param_1;
    uVar13  = (param_1 >> 0x10);
    if(BVar6 == 0x0)
    {
        uVar9 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar7 + 0xc), 0x3);
        if(uVar9 == 0x0)
        {
        code_r0x10387545:
            pass1_1038_6f5a(param_1, param_2, uVar9, puVar11, param_4, param_5, param_3);
            goto LAB_1038_7549;
        }
        if((iVar9->field_0xc != 0x0) || (&iVar9->field_0xe != 0x0))
        {
            uVar16  = pass1_1028_45e2(uVar16, uVar7, puVar11, param_3);
            puVar11 = (uVar16 >> 0x10);
            uVar9   = uVar16;
            ppuVar1 = (u8 **)&iVar9->field_0x18;
            bVar15  = *ppuVar1 < puVar11;
            if((bVar15 || *ppuVar1 == puVar11) && ((bVar15 || (puVar2 = &iVar9->field_0x16, *puVar2 < uVar9 || *puVar2 == uVar9))))
                goto code_r0x10387545;
        }
    }
    else
    {
        uVar17  = pass1_1028_62c8(uVar16, param_3);
        puVar11 = (uVar17 >> 0x10);
        uVar9   = uVar17;
        ppuVar1 = (u8 **)&iVar9->field_0x18;
        bVar15  = *ppuVar1 < puVar11;
        if((bVar15 || *ppuVar1 == puVar11) && ((bVar15 || (puVar2 = &iVar9->field_0x16, *puVar2 < uVar9 || *puVar2 == uVar9))))
        {
            if(iVar9->field_0x12 == 0x0)
            {
                if(iVar9->field_0x14 == 0x0)
                    goto LAB_1038_74e0;
                paStack50 = pass1_1000_07fc(0x1000, globals->PTR_LOOP_1050_68a2);
                uVar7     = (paStack50 >> 0x10);
                uVar10    = (astruct_622 *)paStack50;
                if((uVar7 | uVar10) == 0x0)
                {
                    paStack50 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack50->field_0x0 = 0x389a;
                    uVar10->field_0x2    = 0x1008;
                    uVar10->field_0x4    = 0x0;
                    uVar10->field_0x6    = 0x0;
                    uVar10->field_0x8    = 0x0;
                    uVar10->field_0xa    = 0x0;
                    uVar10->field_0xc    = 0x0;
                    paStack50->field_0x0 = 0x56ce;
                    uVar10->field_0x2    = 0x1018;
                }
                uVar14         = (paStack50 >> 0x10);
                iVar12         = paStack50;
                (iVar12 + 0x8) = iVar9->field_0x14;
                (iVar12 + 0xa) = iVar9->field_0x16;
                uVar7          = pass1_1020_c42e(iVar9->field_0x14);
            }
            else
            {
                paStack26 = pass1_1000_07fc(0x1000, globals->PTR_LOOP_1050_68a2);
                uVar7     = (paStack26 >> 0x10);
                uVar8     = (astruct_616 *)paStack26;
                if((uVar7 | uVar8) == 0x0)
                {
                    paStack26 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack26->field_0x0 = 0x389a;
                    uVar8->field_0x2     = 0x1008;
                    uVar8->field_0x4     = 0x0;
                    uVar8->field_0x6     = 0x0;
                    uVar8->field_0x8     = 0x0;
                    uVar8->field_0xa     = 0x0;
                    uVar8->field_0xc     = 0x0;
                    paStack26->field_0x0 = 0x56ce;
                    uVar8->field_0x2     = 0x1018;
                }
                uVar14         = (paStack26 >> 0x10);
                iVar12         = paStack26;
                (iVar12 + 0x6) = iVar9->field_0x12;
                (iVar12 + 0xa) = iVar9->field_0x16;
                uVar7          = switch_1020_c3b4(iVar9->field_0x12);
            }
            lVar5          = uVar7 * (iVar12 + 0xa);
            puVar11        = (lVar5 >> 0x10);
            uVar9          = lVar5;
            (iVar12 + 0xc) = uVar9;
            pass1_1028_6408(uVar16, CONCAT22(uVar14, iVar12), param_3);
            goto LAB_1038_7549;
        }
    }
LAB_1038_74e0:
    pass1_1038_709c(param_1, param_2, puVar11, param_3);
LAB_1038_7549:
    uVar3 = iVar9->field_0x8;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, (uVar3 >> 0x10));
    pass1_1030_6c4c(CONCAT22(puVar11, uVar9), (uVar9 + 0x34) + iVar9->field_0x26);
    iVar9->field_0xc   = 0x0;
    iVar9->field_0x12  = 0x0;
    iVar9->field_0x14  = 0x0;
    &iVar9->field_0x16 = 0x0;
    paVar4             = &iVar9->field_0xe;
    uVar7              = iVar9->field_0x10;
    if((uVar7 | paVar4) != 0x0)
    {
        fn_ptr_1020_ba7e((paVar4 & 0xffff | uVar7 << 0x10));
        fn_ptr_1000_17ce(paVar4, 0x1000);
    }
    &iVar9->field_0xe = 0x0;
    return;
}


void  pass1_1038_58e6(u16 param_1, u16 param_2, u32 param_3, u32 param_4, u32 param_5, i16 param_6, u16 param_7)

{
    i16         iVar1;
    code      **ppcVar2;
    u32  uVar3;
    BOOL16      BVar4;
    u32 *puVar5;
    u16         uVar6;
    i16         iVar7;
    u16         uVar8;
    u16         uVar9;
    u32         uVar10;
    u32  local_12;
    i16         iStack14;
    i16         iStack12;
    u32         uStack6;

    for(uStack6 = 0x0; uStack6 < param_3; uStack6 = uStack6 + 0x1)
    {
        uVar9 = (param_4 >> 0x10);
        iVar7 = param_4;
        if(((uStack6 * 0x4 + iVar7) != 0x0) && (uVar3 = (uStack6 * 0x4 + iVar7), BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar3 + 0xc), 0x2e), BVar4 != 0x0))
        {
            uVar8    = (param_5 >> 0x10);
            iVar1    = (uStack6 * 0x4 + param_5);
            uVar8    = (uStack6 * 0x4 + param_5 + 0x2);
            local_12 = (iVar1 + 0xc);
            iStack12 = (iVar1 + 0x10);
            iStack14 = iStack12;
            if(iStack12 == param_6)
            {
                iStack14 = iStack12 + -0x1;
                uVar10   = pass1_1028_bb24(*(uStack6 * 0x4 + iVar7));
                uVar6    = (uVar10 >> 0x10);
                puVar5   = &local_12;
                pass1_1030_627e(param_7, puVar5, uVar6, globals->_PTR_LOOP_1050_5740, CONCAT22(param_7, puVar5), uVar10 & 0xffff | uVar6 << 0x10);
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, puVar5, uVar6);
                if((uVar6 | puVar5) != 0x0)
                {
                    uVar10 = struct_op_1030_73a8(globals,NULL;
                    uVar6  = (uVar10 + 0x1a);
                    if(((uVar6 & 0x2) != 0x0) && ((uVar6 & 0x1) != 0x0))
                    {
                        uVar3          = (uStack6 * 0x4 + iVar7);
                        (uVar3 + 0x1a) = 0x3;
                        ppcVar2        = ((uStack6 * 0x4 + iVar7) + 0x28);
                        (**ppcVar2)();
                    }
                }
            }
        }
    }
    return;
}


u16  pass1_1038_5be8(u32 param_1, u16 param_2, i16 param_3, u16 *param_4, u16 param_5, u16 param_6, u16 param_7)

{
    i16    iVar1;
    u16    uVar2;
    i16    iVar3;
    BOOL16 BVar4;
    u16    uVar5;
    u32    uVar6;
    i16    iStack14;
    u32    uStack10;

    pass1_1030_627e(param_7, param_5, param_6, globals->_PTR_LOOP_1050_5740, param_4, (param_1 + 0x8));
    uVar5 = param_6 | param_5;
    if(uVar5 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, param_6);
        uStack10 = CONCAT22(uVar5, param_5);
        iStack14 = 0x7a;
        if(0x0 < (param_4 + 0x4))
        {
            if(param_3 == 0x7b)
            {
                param_3 = 0x7e;
            }
            else
            {
                if(param_3 == 0x7c)
                {
                    param_3 = 0x7d;
                }
            }
            iStack14 = 0x7f;
        }
        uVar6 = struct_op_1030_73a8(globals,NULL;
        uVar2 = (uVar6 >> 0x10);
        iVar3 = uVar6;
        if(((((iVar3 + 0x1a) & param_2) == 0x0) && (((iVar1 = (iVar3 + 0xc), iVar1 == iStack14 || (iVar1 == param_3)) || (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar1, 0x2b), BVar4 != 0x0)))) && ((iVar3 + 0x12) != 0x7))
        {
            (iVar3 + 0x1a) = (iVar3 + 0x1a) | param_2;
            return 0x1;
        }
    }
    return 0x0;
}


void  pass1_1038_4b40(u32 param_1, u16 param_2, u16 param_3)

{
    code **ppcVar1;
    u16    uVar2;
    u32    uVar3;
    u16    extraout_DX;
    u16    uVar4;
    u16    extraout_DX_00;
    u16    uVar5;
    i16    iVar6;
    u16    uVar7;
    u32    uStack14;
    u32    uStack10;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if((iVar6 + 0xc) == 0x0)
    {
        param_2 = 0x0;
        uVar4   = 0x0;
    }
    else
    {
        ppcVar1 = ((iVar6 + 0xc) + 0x10);
        (**ppcVar1)();
        uVar4 = extraout_DX;
    }
    uStack10 = CONCAT22(uVar4, param_2);
    for(uStack14 = 0x0; uStack14 < uStack10; uStack14 = uStack14 + 0x1)
    {
        ppcVar1 = ((iVar6 + 0xc) + 0x4);
        uVar3   = uStack10;
        (**ppcVar1)(param_3, (iVar6 + 0xc));
        uVar2 = uVar3;
        uVar5 = extraout_DX_00 | uVar2;
        if(uVar5 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00);
            param_3 = 0x1030;
            struct_op_1030_73a8(globals,NULL;
        }
    }
    return;
}


void  pass1_1038_4cd0(u32 param_1, u32 param_2, u16 param_3)

{
    u16 uVar1;

    uVar1             = (param_1 >> 0x10);
    (param_1 + 0x1c)  = param_3;
    *(param_1 + 0x1e) = param_2;
    return;
}


void  pass1_1038_4cea(u32 param_1, u32 *param_2, u16 *param_3)

{
    u16 uVar1;

    uVar1    = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x1c);
    *param_2 = *(param_1 + 0x1e);
    return;
}


void  pass1_1038_4d0e(u32 param_1, u16 param_2)

{
    astruct_686 *iVar1;
    u16          uVar1;

    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_686 *)param_1;
    iVar1->field_0x1a_addr_offset = iVar1->field_0x18;
    iVar1->field_0x18 = param_2;
    return;
}


void  pass1_1038_4d6e(u32 param_1, u32 *param_2, u16 param_3, u8 *param_4)

{
    i16        *piVar1;
    code      **ppcVar2;
    u16         uVar3;
    u16         uVar4;
    u16         extraout_DX;
    u16         extraout_DX_00;
    u16         uVar5;
    u16         extraout_DX_01;
    u16         uVar6;
    i16         iVar7;
    u16         uVar8;
    u32         uVar9;
    i16         iStack30;
    u32         uStack26;
    u32         uStack14;
    u32         uStack10;
    u32 *puStack6;

    mem_op_1000_179c(0x18, param_4, 0x1000);
    if((param_4 | param_3) == 0x0)
    {
        param_3 = 0x0;
        uVar8   = 0x0;
    }
    else
    {
        struct_op_1030_1cd8(CONCAT22(param_4, param_3), 0x5, 0x5);
        uVar8 = extraout_DX;
    }
    puStack6 = CONCAT22(uVar8, param_3);
    uVar8    = (param_1 >> 0x10);
    iVar7    = param_1;
    if((iVar7 + 0xc) == 0x0)
    {
        param_3 = 0x0;
        uVar5   = 0x0;
    }
    else
    {
        ppcVar2 = ((iVar7 + 0xc) + 0x10);
        (**ppcVar2)();
        uVar5 = extraout_DX_00;
    }
    uStack10 = CONCAT22(uVar5, param_3);
    uStack14 = 0x0;
    do
    {
        if(uStack10 <= uStack14)
        {
            return;
        }
        ppcVar2 = ((iVar7 + 0xc) + 0x4);
        uVar9   = uStack10;
        (**ppcVar2)();
        uVar3 = uVar9;
        uVar6 = extraout_DX_01 | uVar3;
        if(uVar6 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, extraout_DX_01);
            uStack26 = CONCAT22(uVar6, uVar3);
            uVar4    = pass1_1030_6fa0(CONCAT22(uVar6, uVar3));
            iStack30 = 0x0;
            while(true)
            {
                piVar1 = (param_2 + 0x4);
                if(*piVar1 == iStack30 || *piVar1 < iStack30)
                    break;
                if((*param_2 + iStack30 * 0x2) == uVar4)
                {
                    uVar9 = struct_op_1030_73a8(globals,NULL;
                    if((uVar9 + 0x12) == 0x5)
                    {
                        ppcVar2 = (*puStack6 + 0xc);
                        (**ppcVar2)();
                    }
                    break;
                }
                iStack30 = iStack30 + 0x1;
            }
        }
        uStack14 = uStack14 + 0x1;
    } while(true);
}


void  pass1_1038_4e78(u16 param_1, u8 *param_2, u32 param_3, u32 *param_4)

{
    i16        *piVar1;
    code      **ppcVar2;
    u16         uVar3;
    u32         uVar4;
    u16         extraout_DX;
    u16         extraout_DX_00;
    u16         uVar5;
    u16         extraout_DX_01;
    u16         uVar6;
    i16         iVar7;
    u16         uVar8;
    i16         iStack26;
    u32         uStack14;
    u32         uStack10;
    u32 *puStack6;

    mem_op_1000_179c(0x18, param_2, 0x1000);
    if((param_2 | param_1) == 0x0)
    {
        param_1 = 0x0;
        uVar8   = 0x0;
    }
    else
    {
        struct_op_1030_1cd8(CONCAT22(param_2, param_1), 0x5, 0x5);
        uVar8 = extraout_DX;
    }
    puStack6 = CONCAT22(uVar8, param_1);
    uVar8    = (param_3 >> 0x10);
    iVar7    = param_3;
    if((iVar7 + 0xc) == 0x0)
    {
        param_1 = 0x0;
        uVar5   = 0x0;
    }
    else
    {
        ppcVar2 = ((iVar7 + 0xc) + 0x10);
        (**ppcVar2)();
        uVar5 = extraout_DX_00;
    }
    uStack10 = CONCAT22(uVar5, param_1);
    uStack14 = 0x0;
    do
    {
        if(uStack10 <= uStack14)
        {
            return;
        }
        uVar4 = uStack10;
        pass1_1030_1d58(*(iVar7 + 0xc));
        uVar6 = uVar5 | uVar4;
        if(uVar6 != 0x0)
        {
            uVar3    = pass1_1030_6fa0(uVar4 & 0xffff | uVar5 << 0x10);
            iStack26 = 0x0;
            while(true)
            {
                piVar1 = (param_4 + 0x4);
                if(*piVar1 == iStack26 || *piVar1 < iStack26)
                    break;
                if((*param_4 + iStack26 * 0x2) == uVar3)
                {
                    ppcVar2 = (*puStack6 + 0xc);
                    (**ppcVar2)();
                    uVar6 = extraout_DX_01;
                    break;
                }
                iStack26 = iStack26 + 0x1;
            }
        }
        uStack14 = uStack14 + 0x1;
        uVar5    = uVar6;
    } while(true);
}


astruct_100 * pass1_1038_28d8(astruct_100 *param_1, u16 param_2, u8 param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x3a97);
    param_1->field_0x0 = 0x29fe;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1038;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)), s_SCRoboMove_1050_59f8);
    return param_1;
}


void  pass1_1038_2a0e(astruct_100 *param_1, u32 param_2, u32 param_3, u32 param_4, u32 param_5, u16 param_6, u8 param_7)

{
    i16 iVar1;
    u16 uVar2;

    struct_op_1028_d1dc(param_6, param_7, param_1, 0x2af7);
    uVar2              = (param_1 >> 0x10);
    iVar1              = param_1;
    *(iVar1 + 0x108)   = param_5;
    *(iVar1 + 0x10c)   = param_4;
    *(iVar1 + 0x110)   = param_3;
    *(iVar1 + 0x114)   = param_2;
    param_1->field_0x0 = 0x309a;
    (iVar1 + 0x2)      = &PTR_LOOP_1050_1038;
    return;
}


void  pass1_1038_1c3e(u32 param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5, u16 param_6)

{
    u16         uVar1;
    u32 *puVar2;
    code      **ppcVar3;
    u32         uVar4;
    u16         uVar5;
    i16         iVar6;
    BOOL16      BVar7;
    u32 *puVar8;
    u16         extraout_DX;
    u16         extraout_DX_00;
    u16         uVar9;
    u16         uVar10;
    u32         uVar11;
    u16         uVar12;
    u16         uVar13;
    u16         uVar14;
    u32         uStack26;
    u32         uStack14;

    uVar10  = (param_2 >> 0x10);
    puVar2  = (param_2 + 0xc);
    uVar10  = (param_2 + 0xe);
    ppcVar3 = (*puVar2 + 0x10);
    puVar8  = puVar2;
    uVar14  = puVar2;
    (**ppcVar3)();
    uVar4    = puVar8 & 0xffff | extraout_DX << 0x10;
    uStack14 = 0x0;
    do
    {
        if(uVar4 <= uStack14)
        {
            return;
        }
        ppcVar3 = (*puVar2 + 0x4);
        uVar11  = uVar4;
        (**ppcVar3)(param_5, puVar2, (puVar2 >> 0x10), uStack14, uVar14, uVar10);
        uVar5 = uVar11;
        uVar9 = extraout_DX_00 | uVar5;
        if(uVar9 != 0x0)
        {
            param_5 = &USHORT_1050_1028;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5, extraout_DX_00);
            uStack26 = CONCAT22(uVar9, uVar5);
            iVar6    = (uVar5 + 0x34);
            if((iVar6 != 0x0) && ((uVar5 + 0x36) != 0x0))
            {
                uVar12 = param_1;
                uVar13 = (param_1 >> 0x10);
                pass1_1038_201a(uVar12, uVar13, CONCAT22(uVar9, uVar5), iVar6, uVar9);
                if(iVar6 == 0x0)
                {
                    uVar11  = struct_op_1030_73a8(globals,NULL;
                    uVar1   = (uVar11 + 0xc);
                    param_5 = 0x1008;
                    BVar7   = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x1);
                    if(BVar7 == 0x0)
                    {
                        param_5 = 0x1008;
                        BVar7   = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x2);
                        if(BVar7 == 0x0)
                        {
                            BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x5);
                            if(BVar7 == 0x0)
                            {
                                param_5 = 0x1008;
                                BVar7   = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x6);
                                if(BVar7 == 0x0)
                                    goto LAB_1038_1c76;
                            }
                            param_5 = 0x1008;
                            pass1_1038_2306(uVar12, uVar13, uStack26);
                        }
                        else
                        {
                            pass1_1038_26ee(uVar12, uVar13, uStack26, param_3, param_4, param_6);
                        }
                    }
                    else
                    {
                        pass1_1038_24e8(uVar12, uVar13, uStack26, param_3, param_4, param_6);
                    }
                }
            }
        }
    LAB_1038_1c76:
        uStack14 = uStack14 + 0x1;
    } while(true);
}


void  pass1_1038_1d68(u16 param_1, u16 param_2, u32 *param_3, u32 param_4, u16 param_5, u16 param_6, u16 param_7, u32 param_8)

{
    i16        *piVar1;
    u16         uVar2;
    i16         iVar3;
    u16         uVar4;
    u16         uVar5;
    code      **ppcVar6;
    u32         uVar7;
    u16         uVar8;
    bool        bVar9;
    u8         *puVar10;
    u32         uVar11;
    u16         uVar12;
    u16         uVar13;
    i16         iVar14;
    u32        *puVar15;
    astruct_99 *paStack82;
    u16         uStack78;
    u32  uStack52;
    u8          local_30[0x4];
    u32  uStack44;
    u32 *puStack40;
    u32  uStack36;
    u8          local_20[0x4];
    u32 *puStack28;
    u16         uStack24;
    u16         uStack22;
    u16         uStack20;
    u16         uStack18;
    u32         uStack16;
    u32         uStack12;
    u16         uStack8;
    u32  uStack6;

    uStack6 = 0x64;
    uStack8 = 0x0;
    ppcVar6 = (*param_3 + 0x10);
    puVar15 = param_3;
    (**ppcVar6)();
    uStack12 = CONCAT22(param_8, param_5);
    uStack16 = 0x0;
    do
    {
        if(uStack12 <= uStack16)
        {
            return;
        }
        ppcVar6 = (*param_3 + 0x4);
        uVar11  = uStack12;
        uVar13  = param_8;
        (**ppcVar6)(param_6, param_3, uStack16, puVar15);
        uStack18 = uVar13;
        uVar12   = uVar11;
        uVar13   = uStack18 | uVar12;
        param_8  = uVar13;
        uStack20 = uVar12;
        if(uVar13 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar12, uStack18);
            uStack22  = uVar13;
            param_6   = 0x1030;
            uStack24  = uVar12;
            puStack28 = struct_op_1030_73a8(globals,NULL;
            param_8   = puStack28 >> 0x10;
            puVar10   = local_20;
            ppcVar6   = (*puStack28 + 0x40);
            (**ppcVar6)(0x1030, puStack28, (puStack28 >> 0x10), puVar10, param_7);
            if(puVar10 == 0x0)
            {
                uStack36  = pass1_1028_62c8(puStack28, param_7);
                uVar11    = uStack36 >> 0x10;
                uStack8   = 0x1;
                puStack40 = *(param_4 + 0x22);
                pass1_1008_5784(CONCAT22(param_7, local_30), puStack40);
                while(true)
                {
                    uVar13  = uVar11;
                    puVar10 = local_30;
                    param_6 = 0x1008;
                    pass1_1008_5b12(puVar10, param_7);
                    uStack52 = CONCAT22(uVar13, puVar10);
                    param_8  = (uVar13 | puVar10);
                    if((uVar13 | puVar10) == 0x0)
                        break;
                    uVar2  = (puVar10 + 0x4);
                    iVar3  = (puVar10 + 0x6);
                    uVar4  = (puVar10 + 0x8);
                    uVar12 = (puVar10 + 0xc);
                    uVar5  = (puVar10 + 0xa);
                    uVar8  = uVar12 / uVar5;
                    uVar11 = uVar12 % uVar5;
                    bVar9  = false;
                    if(((0x0 < iVar3) && (!SBORROW2(iVar3, 0x1))) && ((iVar3 == 0x5 || iVar3 + -0x1 < 0x4 || (iVar3 == 0x8))))
                    {
                        bVar9 = true;
                    }
                    if(bVar9)
                    {
                        uVar11 = uStack36;
                        if(uStack6 < uStack36)
                        {
                            uVar11         = uStack6 & 0xffff;
                            uStack36._2_2_ = uStack6._2_2_;
                        }
                        uVar12  = uStack36._2_2_ | uVar11;
                        param_8 = uVar12;
                        if(uVar12 == 0x0)
                            break;
                        uStack78 = ((uVar11 & 0xffff | uStack36._2_2_ << 0x10) / uVar8);
                        if(uStack78 < uVar5)
                        {
                            piVar1  = (puVar10 + 0xc);
                            *piVar1 = *piVar1 - uVar11;
                            piVar1  = (puVar10 + 0xa);
                            *piVar1 = *piVar1 - uStack78;
                        }
                        else
                        {
                            ppcVar6 = (*puStack40 + 0xc);
                            (**ppcVar6)(0x1008, puStack40, (puStack40 >> 0x10), uStack52);
                            uStack44 = 0x0;
                            uStack78 = uVar5;
                        }
                        paStack82 = pass1_1000_07fc(0x1000, globals->PTR_LOOP_1050_68a2);
                        uVar12    = (paStack82 >> 0x10);
                        uVar13    = paStack82;
                        if((uVar12 | uVar13) == 0x0)
                        {
                            paStack82 = (astruct_99 *)0x0;
                        }
                        else
                        {
                            paStack82->field_0x0 = 0x389a;
                            (uVar13 + 0x2)       = 0x1008;
                            (uVar13 + 0x4)       = 0x0;
                            (uVar13 + 0x6)       = 0x0;
                            (uVar13 + 0x8)       = 0x0;
                            (uVar13 + 0xa)       = 0x0;
                            (uVar13 + 0xc)       = 0x0;
                            paStack82->field_0x0 = 0x56ce;
                            (uVar13 + 0x2)       = 0x1018;
                        }
                        uVar13         = (paStack82 >> 0x10);
                        iVar14         = paStack82;
                        (iVar14 + 0xa) = uStack78;
                        uVar7          = uStack78 * uVar8;
                        uVar11         = uVar7 >> 0x10;
                        (iVar14 + 0xc) = uVar7;
                        (iVar14 + 0x4) = uVar2;
                        (iVar14 + 0x6) = iVar3;
                        (iVar14 + 0x8) = uVar4;
                        pass1_1028_6408(puStack28, (paStack82 & 0xffff | uVar13 << 0x10), param_7);
                    }
                }
            }
            else
            {
                ppcVar6 = (*param_3 + 0x8);
                (**ppcVar6)(0x1030, param_3, 0x0, uStack16);
            }
        }
        uStack16 = uStack16 + 0x1;
    } while(true);
}


void  pass1_1038_1faa(u32 param_1, u32 *param_2, u32 *param_3, u16 param_4, u32 param_5, u16 param_6)

{
    fn_ptr_1 *ppcVar1;
    u16       uVar2;
    u32       uVar3;
    u32       uStack10;
    u32       uStack6;

    ppcVar1 = (*param_3 + 0x10);
    (**ppcVar1)();
    uStack6  = CONCAT22(param_5, param_4);
    uStack10 = 0x0;
    while(true)
    {
        if(uStack6 <= uStack10)
        {
            return;
        }
        ppcVar1 = (*param_3 + 0x4);
        uVar3   = uStack6;
        (**ppcVar1)();
        uVar2 = uVar3;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, param_5);
        uVar3   = struct_op_1030_73a8(globals,NULL;
        param_5 = param_5 & 0xffff0000 | uVar3 >> 0x10;
        uVar2   = uVar3;
        pass1_1038_1d68(param_1, (param_1 >> 0x10), param_2, uVar3, uVar2, 0x1030, param_6, param_5);
        if(uVar2 == 0x0)
            break;
        uStack10 = uStack10 + 0x1;
    }
    return;
}


void  pass1_1038_201a(u16 param_1, u16 param_2, u32 param_3, u16 param_4, u16 param_5)

{
    u16         *puVar1;
    i16          iVar2;
    code       **ppcVar3;
    long         lVar4;
    u16          uVar6;
    u16          uVar7;
    u16          uVar8;
    u32          uVar9;
    u8          *puVar10;
    u32          uVar11;
    u32          uVar12;
    astruct_416 *iVar12;
    u16          uVar13;
    u8          *puVar14;
    u16          uVar15;
    u32  *puVar16;
    u16          uVar17;
    long         lStack24;
    long         lStack20;
    u16          uStack10;
    astruct_413 *uVar5;

    uVar17  = (param_3 >> 0x10);
    uVar15  = 0x1030;
    puVar16 = pass1_1030_6b16(param_3);
    uVar6   = (puVar16 >> 0x10);
    uVar5   = (astruct_413 *)puVar16;
    if((uVar6 | uVar5) == 0x0)
    {
        return;
    }
    iVar12   = (astruct_416 *)param_3;
    iVar2    = iVar12->field_0x34;
    lVar4    = (long)iVar2;
    uVar12   = lVar4 * 0x64;
    puVar10  = (uVar12 >> 0x10);
    uVar7    = uVar12;
    uStack10 = 0x0;
    lStack20 = 0x0;
    if(uVar5->field_0x4 == 0x0)
    {
        if(uVar5->field_0x6 == 0x0)
        {
            if(uVar5->field_0x8 == 0x0)
                goto LAB_1038_2102;
            uVar8   = pass1_1020_c42e(uVar5->field_0x8);
            uVar11  = uVar5->field_0xa * uVar8;
            puVar14 = (uVar11 >> 0x10);
            if(uVar11 + lVar4 * -0x64 != 0x0 && (long)uVar12 <= (long)uVar11)
            {
                uVar11  = uVar12 & 0xffff;
                puVar14 = puVar10;
            }
            uVar12   = uVar11 & 0xffff | ZEXT24(puVar14) << 0x10;
            uVar9    = (long)(uVar11 & 0xffff | ZEXT24(puVar14) << 0x10) / (long)uVar8;
            puVar1   = &uVar5->field_0xa;
            *puVar1  = *puVar1 - uVar9;
            uStack10 = ((long)uVar12 / 0x64);
            uVar12   = (long)uVar12 % 0x64;
            uVar11   = uVar12;
            if(uVar12 != 0x0)
            {
                uStack10 = uStack10 + 0x1;
                uVar11   = uStack10;
            }
            uVar7 = uVar11;
            mem_op_1000_179c(0x2a, uVar12, 0x1000);
            puVar10 = (uVar12 | uVar7);
            if(puVar10 == 0x0)
                goto LAB_1038_20fa;
            pass1_1038_6838(CONCAT22(uVar12, uVar7), uVar9, uVar5->field_0x8, uStack10, iVar12->field_0x4);
        }
        else
        {
            uVar8   = switch_1020_c3b4(uVar5->field_0x6);
            uVar11  = uVar5->field_0xa * uVar8;
            puVar14 = (uVar11 >> 0x10);
            if(uVar11 + lVar4 * -0x64 != 0x0 && (long)uVar12 <= (long)uVar11)
            {
                uVar11  = uVar12 & 0xffff;
                puVar14 = puVar10;
            }
            uVar12   = uVar11 & 0xffff | ZEXT24(puVar14) << 0x10;
            uVar9    = (long)(uVar11 & 0xffff | ZEXT24(puVar14) << 0x10) / (long)uVar8;
            puVar1   = &uVar5->field_0xa;
            *puVar1  = *puVar1 - uVar9;
            uStack10 = ((long)uVar12 / 0x64);
            uVar12   = (long)uVar12 % 0x64;
            uVar11   = uVar12;
            if(uVar12 != 0x0)
            {
                uStack10 = uStack10 + 0x1;
                uVar11   = uStack10;
            }
            uVar7 = uVar11;
            mem_op_1000_179c(0x2a, uVar12, 0x1000);
            puVar10 = (uVar12 | uVar7);
            if(puVar10 == 0x0)
                goto LAB_1038_20fa;
            pass1_1038_675c(CONCAT22(uVar12, uVar7), uVar9, uVar5->field_0x6, uStack10, iVar12->field_0x4);
        }
    }
    else
    {
        uVar13  = uVar5->field_0xa;
        puVar14 = 0x0;
        if((puVar10 < 0x1) && ((0x7fff < puVar10 || (uVar7 < uVar13))))
        {
            uVar13  = uVar7;
            puVar14 = puVar10;
        }
        lStack24 = CONCAT22(puVar14, uVar13);
        puVar1   = &uVar5->field_0xa;
        *puVar1  = *puVar1 - uVar13;
        uStack10 = (lStack24 / 0x64);
        uVar11   = lStack24 % 0x64;
        uVar12   = uVar11;
        if(uVar11 != 0x0)
        {
            uStack10 = uStack10 + 0x1;
            uVar12   = uStack10;
        }
        uVar7 = uVar12;
        mem_op_1000_179c(0x2a, uVar11, 0x1000);
        puVar10 = (uVar11 | uVar7);
        if(puVar10 == 0x0)
        {
        LAB_1038_20fa:
            uVar15   = 0x1000;
            lStack20 = 0x0;
            goto LAB_1038_2102;
        }
        pass1_1038_6590(CONCAT22(uVar11, uVar7), uVar13, puVar14, uVar5->field_0x4, uStack10, iVar12->field_0x4);
    }
    uVar15   = 0x1000;
    lStack20 = CONCAT22(puVar10, uVar7);
LAB_1038_2102:
    if(lStack20 != 0x0)
    {
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        uVar15 = 0x1030;
        uVar7  = uStack10;
        pass1_1030_6c4c(param_3, iVar2 - uStack10);
    }
    if(uVar5->field_0xa == 0x0)
    {
        if((uVar6 | uVar5) != 0x0)
        {
            ppcVar3 = *puVar16;
            (**ppcVar3)(uVar15, uVar5, uVar6, 0x1);
        }
    }
    else
    {
        pass1_1030_6c66(param_3, 0x0, puVar16, uVar7, puVar10, 0x1030);
    }
    return;
}


void  pass1_1038_01c0(u16 param_1, u16 param_2, u32 param_3, u16 param_4)

{
    i16         iVar1;
    u32 *puVar2;
    code      **ppcVar3;
    u32         uVar4;
    u32         uVar5;
    BOOL16      BVar6;
    u8         *puVar7;
    u32 *puVar8;
    u32         uVar9;
    u8         *puVar10;
    u16         extraout_DX;
    u16         extraout_DX_00;
    u16         uVar11;
    u16         uVar12;
    u16         uVar13;
    u16         uVar14;
    u32        *puVar15;
    u32         uVar16;
    u32         uVar17;
    u8          uVar18;
    u32         uStack50;
    u32         uStack30;
    u32         uStack18;
    u8          local_e[0x2];
    u32 *puStack12;
    u16         uStack8;
    u8         *puStack6;
    i16         iStack4;

    iStack4  = 0x0;
    puVar15  = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x29);
    puVar10  = (puVar15 >> 0x10);
    uVar12   = puVar15;
    uStack8  = uVar12;
    puStack6 = puVar10;
    pass1_1038_4e78(uVar12, puVar10, param_3, puVar15);
    puStack12 = CONCAT22(puVar10, uVar12);
    uVar14    = 0x1030;
    uVar16    = pass1_1030_bcae(local_e, param_4);
    uVar13    = uVar16;
    ppcVar3   = (*puStack12 + 0x10);
    (**ppcVar3)(0x1030, puStack12, (puStack12 >> 0x10));
    uStack18 = CONCAT22(extraout_DX, uVar13);
    uVar13   = (param_3 >> 0x10);
    puVar2   = *(param_3 + 0xc);
    uVar13   = (param_3 + 0xe);
    uVar18   = SUB41(puVar2, 0x0);
    ppcVar3  = (*puVar2 + 0x10);
    puVar8   = puVar2;
    (**ppcVar3)();
    uVar16   = puVar8 & 0xffff | extraout_DX_00 << 0x10;
    uStack30 = 0x0;
    uVar12   = extraout_DX_00;
    do
    {
        if(uStack18 <= uStack30)
        {
            if(puStack12 != 0x0)
            {
                ppcVar3 = *puStack12;
                (**ppcVar3)(uVar14, puStack12, (puStack12 >> 0x10), 0x1, uVar18, uVar13);
            }
            return;
        }
        uVar14 = 0x1030;
        uVar9  = uStack18;
        pass1_1030_1d58(puStack12);
        uVar5  = uVar12;
        iVar1  = (uVar9 + 0x10);
        uVar11 = uVar12;
        for(uStack50 = 0x0; uVar12 = uVar11, uStack50 < uVar16; uStack50 = uStack50 + 0x1)
        {
            uVar14 = 0x1030;
            uVar17 = uVar16;
            pass1_1030_1d58(puVar2);
            uVar4  = uVar17 & 0xffff | uVar11 << 0x10;
            uVar12 = uVar11 | uVar17;
            if((uVar12 != 0x0) && (uVar12 = uVar11, (uVar17 + 0x10) == iVar1))
            {
                uVar17 = struct_op_1030_73a8(globals,NULL;
                uVar12 = (uVar17 >> 0x10);
                uVar14 = 0x1008;
                BVar6  = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar17 + 0xc), 0x30);
                if(BVar6 == 0x0)
                {
                    puVar7 = local_e;
                    uVar14 = 0x1030;
                    pass1_1030_bd74(puVar7, param_4, uVar4, uVar9 & 0xffff | uVar5 << 0x10, param_4);
                    if(puVar7 < 0x6)
                    {
                        iStack4 = iStack4 + 0x1;
                        break;
                    }
                }
            }
            uVar11 = uVar12;
        }
        uStack30 = uStack30 + 0x1;
    } while(true);
}


void  pass1_1038_0e00(u32 param_1, u32 *param_2, u32 param_3, u16 param_4, u16 param_5)

{
    code **ppcVar1;
    u16    extraout_DX;
    u16    extraout_DX_00;
    u16    uVar2;
    u16    uVar3;
    u32    uVar4;
    u32    uStack10;
    u32    uStack6;

    ppcVar1 = (*param_2 + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX, param_4);
    for(uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1)
    {
        ppcVar1 = (*param_2 + 0x4);
        uVar4   = uStack6;
        (**ppcVar1)();
        uVar3 = uVar4;
        uVar2 = extraout_DX_00 | uVar3;
        if(uVar2 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, extraout_DX_00);
            uVar4 = struct_op_1030_73a8(globals,NULL;
            uVar3 = (uVar4 >> 0x10);
            if((uVar3 | uVar4) != 0x0)
            {
                pass1_1038_0d8e(param_1, (param_1 >> 0x10), uVar4 & 0xffff | uVar3 << 0x10, param_3, param_5);
            }
        }
    }
    return;
}


void  pass1_1038_0e78(u32 param_1, u32 param_2, u16 param_3)

{
    code      **ppcVar1;
    u16         uVar2;
    u16         uVar3;
    u16         uVar4;
    u8         *puVar5;
    u16         extraout_DX;
    u8         *puVar6;
    u16         extraout_DX_00;
    u16         extraout_DX_01;
    u16         uVar7;
    u16         uVar8;
    u32        *puVar9;
    u32         uVar10;
    u32         uStack22;
    u32         uStack18;
    u32 *puStack14;
    u32        *puStack10;

    puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x4);
    puVar5 = (puVar9 >> 0x10);
    uVar2  = puVar9;
    pass1_1038_4d6e(param_2, puVar9, uVar2, puVar5);
    puStack10 = CONCAT22(puVar5, uVar2);
    uVar10    = *puStack10;
    ppcVar1   = uVar10 + 0x8;
    uVar3     = uVar2;
    (**ppcVar1)(0x1008, uVar2, puVar5);
    if((extraout_DX | uVar3) == 0x0)
    {
        if(puStack10 != 0x0)
        {
            ppcVar1 = uVar10;
            (**ppcVar1)(0x8, uVar2, puVar5, 0x1);
            return;
        }
    }
    else
    {
        uVar8  = 0x1008;
        puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x1e);
        puVar6 = (puVar9 >> 0x10);
        uVar3  = puVar9;
        pass1_1038_4d6e(param_2, puVar9, uVar3, puVar6);
        puStack14 = CONCAT22(puVar6, uVar3);
        ppcVar1   = (*puStack14 + 0x10);
        uVar4     = uVar3;
        (**ppcVar1)(0x1008, uVar3, puVar6);
        uStack18 = CONCAT22(extraout_DX_00, uVar4);
        for(uStack22 = 0x0; uStack22 < uStack18; uStack22 = uStack22 + 0x1)
        {
            ppcVar1 = (*puStack14 + 0x4);
            uVar10  = uStack18;
            (**ppcVar1)();
            uVar4 = uVar10;
            uVar7 = extraout_DX_01 | uVar4;
            if(uVar7 != 0x0)
            {
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar4, extraout_DX_01);
                uVar8  = 0x1030;
                uVar10 = struct_op_1030_73a8(globals,NULL;
                if(((uVar10 >> 0x10) | uVar10) != 0x0)
                {
                    pass1_1038_0e00(param_1, puStack10, uVar10, uVar10, param_3);
                }
            }
        }
        if(puStack10 != 0x0)
        {
            ppcVar1 = *puStack10;
            (**ppcVar1)(uVar8, uVar2, puVar5, 0x1);
        }
        if(puStack14 != 0x0)
        {
            ppcVar1 = *puStack14;
            (**ppcVar1)(uVar8, uVar3, puVar6, 0x1);
        }
    }
    return;
}


void  pass1_1038_0f8c(u16 param_1, u16 param_2, u32 *param_3, u32 param_4, u16 param_5, u32 param_6, u16 param_7, u16 param_8)

{
    i16        *piVar1;
    u16         uVar2;
    u16         uVar3;
    u16         uVar4;
    code      **ppcVar5;
    u32         uVar6;
    qword       qVar7;
    u8         *puVar8;
    u32         uVar9;
    u16         uVar10;
    u16         uVar11;
    u16         uVar12;
    i16         iVar13;
    u16         uVar14;
    astruct_99 *paStack80;
    u16         uStack76;
    u8          local_30[0x4];
    u32  uStack44;
    u32 *puStack40;
    u32  uStack36;
    u8          local_20[0x4];
    u32 *puStack28;
    u16         uStack24;
    u16         uStack22;
    u16         uStack20;
    u16         uStack18;
    u32         uStack16;
    u32         uStack12;
    u16         uStack8;
    u32  uStack6;

    uStack6 = 0x64;
    uStack8 = 0x0;
    ppcVar5 = (*param_3 + 0x10);
    (**ppcVar5)(param_7, param_3);
    uStack12 = CONCAT22(param_6, param_5);
    uStack16 = 0x0;
    do
    {
        if(uStack12 <= uStack16)
        {
            return;
        }
        ppcVar5 = (*param_3 + 0x4);
        uVar9   = uStack12;
        uVar11  = param_6;
        (**ppcVar5)(param_7, param_3, (param_3 >> 0x10), uStack16, (uStack16 >> 0x10));
        uStack18 = uVar11;
        uVar12   = uVar9;
        uVar11   = uStack18 | uVar12;
        param_6  = uVar11;
        uStack20 = uVar12;
        if(uVar11 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar12, uStack18);
            uStack22  = uVar11;
            param_7   = 0x1030;
            uStack24  = uVar12;
            puStack28 = struct_op_1030_73a8(globals,NULL;
            param_6   = puStack28 >> 0x10;
            puVar8    = local_20;
            ppcVar5   = (*puStack28 + 0x40);
            (**ppcVar5)(0x1030, puStack28, (puStack28 >> 0x10), puVar8, param_8);
            if(puVar8 == 0x0)
            {
                uStack36  = pass1_1028_62c8(puStack28, param_8);
                uVar9     = uStack36 >> 0x10;
                uStack8   = 0x1;
                puStack40 = *(param_4 + 0x22);
                pass1_1008_5784(CONCAT22(param_8, local_30), puStack40);
                while(true)
                {
                    uVar11  = uVar9;
                    puVar8  = local_30;
                    param_7 = 0x1008;
                    pass1_1008_5b12(puVar8, param_8);
                    param_6 = (uVar11 | puVar8);
                    if((uVar11 | puVar8) == 0x0)
                        break;
                    uVar2  = (puVar8 + 0x4);
                    uVar3  = (puVar8 + 0x6);
                    uVar4  = (puVar8 + 0x8);
                    uVar12 = (puVar8 + 0xa);
                    uVar6  = (puVar8 + 0xc) / uVar12;
                    uVar9  = uStack36;
                    if(uStack6 < uStack36)
                    {
                        uVar9          = uStack6 & 0xffff;
                        uStack36._2_2_ = uStack6._2_2_;
                    }
                    uVar10  = uStack36._2_2_ | uVar9;
                    param_6 = uVar10;
                    if(uVar10 == 0x0)
                        break;
                    qVar7    = (qword)(uVar9 & 0xffff | uStack36._2_2_ << 0x10) / (qword)uVar6;
                    param_6  = qVar7 >> 0x10;
                    uStack76 = qVar7;
                    if(uStack76 == 0x0)
                        break;
                    if(uStack76 < uVar12)
                    {
                        piVar1  = (puVar8 + 0xc);
                        *piVar1 = *piVar1 - uVar9;
                        piVar1  = (puVar8 + 0xa);
                        *piVar1 = *piVar1 - uStack76;
                    }
                    else
                    {
                        ppcVar5 = (*puStack40 + 0xc);
                        (**ppcVar5)(0x1008, puStack40, (puStack40 >> 0x10), puVar8, uVar11);
                        uStack44 = 0x0;
                        uStack76 = uVar12;
                    }
                    paStack80 = pass1_1000_07fc(0x1000, globals->PTR_LOOP_1050_68a2);
                    uVar12    = (paStack80 >> 0x10);
                    uVar11    = paStack80;
                    if((uVar12 | uVar11) == 0x0)
                    {
                        paStack80 = (astruct_99 *)0x0;
                    }
                    else
                    {
                        paStack80->field_0x0 = 0x389a;
                        (uVar11 + 0x2)       = 0x1008;
                        (uVar11 + 0x4)       = 0x0;
                        (uVar11 + 0x6)       = 0x0;
                        (uVar11 + 0x8)       = 0x0;
                        (uVar11 + 0xa)       = 0x0;
                        (uVar11 + 0xc)       = 0x0;
                        paStack80->field_0x0 = 0x56ce;
                        (uVar11 + 0x2)       = 0x1018;
                    }
                    uVar14         = (paStack80 >> 0x10);
                    iVar13         = paStack80;
                    (iVar13 + 0xa) = uStack76;
                    uVar6          = uStack76 * uVar6;
                    uVar9          = uVar6 >> 0x10;
                    (iVar13 + 0xc) = uVar6;
                    (iVar13 + 0x4) = uVar2;
                    (iVar13 + 0x6) = uVar3;
                    (iVar13 + 0x8) = uVar4;
                    pass1_1028_6408(puStack28, paStack80, param_8);
                }
            }
            else
            {
                ppcVar5 = (*param_3 + 0x8);
                (**ppcVar5)(0x1030, param_3, 0x0, 0x0, uStack16, (uStack16 >> 0x10));
            }
        }
        uStack16 = uStack16 + 0x1;
    } while(true);
}


astruct_100 * pass1_1030_e09e(astruct_100 *param_1, u16 param_2, u8 param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x2af7);
    param_1->field_0x0 = 0xe2ae;
    (param_1 + 0x2)    = 0x1030;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)), s_SCAiInput_1050_5972);
    return param_1;
}


void  struct_1030_e2be(astruct_100 *param_1, u16 param_2, u32 param_3, u32 param_4, u16 param_5, u8 param_6)

{
    astruct_217 *iVar1;
    u16          uVar1;

    struct_op_1028_d1dc(param_5, param_6, param_1, 0x2af7);
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_217 *)param_1;
    iVar1->field_0x108 = param_4;
    iVar1->field_0x10c = param_3;
    iVar1->field_0x110 = param_2;
    param_1->field_0x0 = 0xe4ea;
    iVar1->field_0x2   = 0x1030;
    return;
}


astruct_100 * pass1_1030_e63e(astruct_100 *param_1, u16 param_2, u16 param_3, u8 param_4)

{
    i16 iVar1;
    u16 uVar2;

    iVar1 = param_1;
    uVar2 = (param_1 >> 0x10);
    struct_op_1028_d1dc(param_3, param_4, param_1, 0xf9f);
    (iVar1 + 0x108)    = param_2;
    param_1->field_0x0 = 0xe78a;
    (iVar1 + 0x2)      = 0x1030;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (iVar1 + 0x8)), s_SCKillColony_1050_5990);
    return param_1;
}


astruct_100 * pass1_1030_e79a(astruct_100 *param_1, u16 param_2, u8 param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0xf9f);
    param_1->field_0x0 = 0xe890;
    (param_1 + 0x2)    = 0x1030;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)), s_SCKillRebelColony_1050_599e);
    return param_1;
}


void  pass1_1030_ea50(u32 param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5)

{
    u32 uVar1;
    BOOL16     BVar2;
    i16        iVar3;
    u16        uVar4;
    u32        uVar5;
    u32        local_12;
    u16        local_e;
    i16        iStack12;
    u16        uStack10;
    u16        uStack8;
    u32 uStack6;

    uStack6 = 0x1869f;
    uVar4   = (param_1 >> 0x10);
    iVar3   = param_1;
    BVar2   = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (iVar3 + 0x110), 0x3);
    if(BVar2 != 0x0)
    {
        uVar5    = struct_op_1030_73a8(globals,NULL;
        iStack12 = (uVar5 >> 0x10);
        local_e  = uVar5;
        uStack6  = pass1_1028_45e2(uVar5, local_e, iStack12, param_5);
    }
    uVar1    = (iVar3 + 0x108);
    uStack8  = (uVar1 + 0x4);
    uStack10 = 0x0;
    while(true)
    {
        if(uStack8 <= uStack10)
        {
            return;
        }
        pass1_1020_bb16((iVar3 + 0x108), CONCAT22(param_5, &local_12), CONCAT22(param_5, &local_e), uStack10);
        if(uStack6 < local_12)
        {
            pass1_1030_7ddc(param_2, uStack6, local_e, uStack6, uStack6._2_2_, param_3, param_4, param_5);
            uStack6 = 0x0;
        }
        else
        {
            uStack6 = uStack6 - local_12;
            pass1_1030_7ddc(param_2, local_12, local_e, local_12, uStack6._2_2_, param_3, param_4, param_5);
        }
        if((uStack6._2_2_ | uStack6) == 0x0)
            break;
        uStack10 = uStack10 + 0x1;
    }
    return;
}


astruct_100 * pass1_1030_eb50(astruct_100 *param_1, u16 param_2, u8 param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x1f3f);
    param_1->field_0x0 = 0xecb2;
    (param_1 + 0x2)    = 0x1030;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)), s_SCMines_1050_59c6);
    return param_1;
}


astruct_100 * pass1_1030_ecc2(astruct_100 *param_1, u16 param_2, u8 param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0xf9f);
    param_1->field_0x0 = 0xb96;
    (param_1 + 0x2)    = &PTR_LOOP_1050_1038;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)), s_SCMorale_1050_59ce);
    return param_1;
}


u16 * struct_1030_d8f6(u16 *param_1)

{
    astruct_184 *iVar1;
    u16          uVar1;

    struct_1028_b354(param_1);
    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_184 *)param_1;
    *param_1         = 0xdc2e;
    iVar1->field_0x2 = 0x1030;
    if(iVar1->field_0xc == 0x4c)
    {
        iVar1->field_0xe = 0x43;
    }
    else
    {
        if(iVar1->field_0xc == 0x4d)
        {
            iVar1->field_0xe = 0x44;
        }
        else
        {
            iVar1->field_0xe = 0x45;
        }
    }
    return param_1;
}


u16 * struct_1030_dc96(u16 *param_1)

{
    u16 uVar1;

    struct_1028_b354(param_1);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x20) = 0x0;
    *param_1         = 0xe036;
    (param_1 + 0x2)  = 0x1030;
    return param_1;
}
