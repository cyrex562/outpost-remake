//
// Created by cyrex on 2023-03-07.
//

#include "win_ops_5.h"

#include "draw_ops/draw_ops_2.h"
#include "draw_ops/draw_ops_3.h"
#include "fn_ptr_ops/fn_ptr_ops_6.h"
#include "fn_ptr_ops/fn_ptr_ops_7.h"
#include "globals.h"
#include "op_int.h"
#include "op_win_def.h"
#include "op_winapi.h"
#include "structs/structs_0xx/structs_3x.h"
#include "sys_ops/sys_ops_6.h"
#include "sys_ops/sys_ops_8.h"
#include "sys_ops/sys_ops_9.h"
#include "ui_ops/ui_ops_6.h"
#include "ui_ops_1.h"
#include "ui_ops_4.h"
#include "ui_ops_6.h"
#include "unk/unk_12.h"
#include "unk/unk_14.h"
#include "unk/unk_5.h"
#include "utils.h"
#include "win_ops/win_ops_3.h"
#include "win_ops_3.h"

#include <stdbool.h>
#include <stddef.h>


void window_op_1020_10a0(struct Globals *globals, struct Struct0 *param_1)

{
    u32               uVar1;
    void **ppcVar2;
    struct Struct160 *in_AX;
    u16               uVar3;
    u16              *pBVar4;
    u8               *in_DX;
    u8               *puVar5;
    u8               *puVar6;
    u8               *extraout_DX;
    u16               extraout_DX_00;
    short             unaff_DI;
    u16               unaff_SS;
    u8                in_AF;
    u16              *puVar7;
    u32               uVar8;
    u16               uVar9;
    u8               *puVar10;
    short             iVar11;
//    u16          uVar12;

    iVar11 = param_1;
//    uVar12 = (param_1 >> 0x10);
    create_window_ex_1008_9760(param_1, SEG_1008);
    mem_op_1000_179c(0x42, in_DX, 0x1000);
    puVar5 = (in_DX | in_AX);
    if(puVar5 != 0x0)
    {
        pass1_1008_3bd6(in_AX, in_DX, 0x0, 0x1f009b, 0x0, 0x740075, CONCAT22((iVar11 + 0x8), 0xf1), puVar5, unaff_SS);
    }
    mem_op_1000_179c(0x42, puVar5, 0x1000);
    puVar6 = (puVar5 | in_AX);
    if(puVar6 != 0x0)
    {
        pass1_1008_3bd6(in_AX, puVar5, 0x0, 0x31009b, 0x0, 0x760077, CONCAT22((iVar11 + 0x8), 0xf2), puVar6, unaff_SS);
    }
    mem_op_1000_179c(globals,0x42, puVar6, 0x1000);
    puVar5 = (puVar6 | in_AX);
    if(puVar5 != 0x0)
    {
        pass1_1008_3bd6(in_AX, puVar6, 0x0, 0x77009b, 0x0, 0x780079, CONCAT22((iVar11 + 0x8), 0xf3), puVar5, unaff_SS);
    }
    puVar7                      = mixed_1010_20ba(globals->_PTR_LOOP_1050_0ed0, 0x2d, unaff_SS, puVar5, unaff_DI);
    uVar9                       = (puVar7 >> 0x10);
    (iVar11 + 0xf2)             = puVar7;
    (iVar11 + 0xf4)             = uVar9;
    (iVar11 + 0xe0)             = (iVar11 + 0xf2);
    (iVar11 + 0xe2)             = uVar9;
    puVar10                     = globals->PTR_LOOP_1050_038c;
    uVar3                       = LoadIcon16(SEG_1010, s_PLNTICON_1050_4267);
    *(HANDLE16 *)(iVar11 + 0xc2) = uVar3;
    uVar1                       = (iVar11 + 0xf2);
    ppcVar2                     = ((iVar11 + 0xf2) + 0x30);
    (**ppcVar2)(LAST_SEGMENT, uVar1, (uVar1 >> 0x10), uVar3, puVar10);
    puVar5 = extraout_DX;
    mem_op_1000_179c(globals,0x24, extraout_DX, 0x1000);
    puVar6 = (puVar5 | uVar3);
    if(puVar6 == 0x0)
    {
        (iVar11 + 0xf6) = 0x0;
    }
    else
    {
        unk_win_ui_op_1020_1418((struct Struct40 *)CONCAT22(puVar5, uVar3), param_1, unaff_SS);
        *(iVar11 + 0xf6) = uVar3;
        (iVar11 + 0xf8)  = puVar6;
    }
    (iVar11 + 0xe8) = (iVar11 + 0xf6);
    puVar7          = mixed_1010_20ba(globals->_PTR_LOOP_1050_0ed0, 0x2f, unaff_SS, puVar6, unaff_DI);
    uVar8           = pass1_1018_04b8(puVar7);
    puVar5          = (uVar8 >> 0x10);
    pass1_1010_41d6(*(iVar11 + 0xf2), uVar8, puVar5, unaff_SS, in_AF);
    uVar8   = pass1_1010_451a(*(iVar11 + 0xf2), puVar5, unaff_DI, unaff_SS);
    pBVar4  = (u16 *)uVar8;
    uVar1   = param_1;
    ppcVar2 = (uVar1 + 0x14);
    (**ppcVar2)(SEG_1010, iVar11, uVar12, 0x0, pBVar4, (uVar8 >> 0x10));
    uVar9   = 0x1;
    ppcVar2 = (uVar1 + 0x10);
    (**ppcVar2)();
    pass1_1010_459e((iVar11 + 0xf2));
    ppcVar2 = ((iVar11 + 0xf2) + 0x10);
    (**ppcVar2)(SEG_1010, (iVar11 + 0xf2), param_1, uVar9);
    MoveWindow16(SEG_1010, 0x1, pBVar4[0x3], pBVar4[0x2], pBVar4[0x1], *pBVar4);
    UpdateWindow16((HANDLE16)LAST_SEGMENT);
}
void window_op_1020_2642(Globals *globals, Struct0 *param_1)

{
    Struct664 *in_AX;
    u8        *in_DX;
    u16        uVar1;
    //    i16          iVar2;
    i16 unaff_DI;
    //    u16          uVar3;
    u16 unaff_SS;

    create_window_ex_1008_9760(param_1, SEG_1008);
    //    uVar3 = (param_1 >> 0x10);
    //    iVar2 = param_1;
    get_dc_1018_4db0(*(param_1->field_0xf2), (param_1->field_0x8), SEG_1018);
    mem_op_1000_179c(globals, 0x18, in_DX, 0x1000);
    uVar1 = in_DX | in_AX;
    if(uVar1 != 0x0)
    {
        pass1_1020_27b0(in_AX, in_DX, (param_1->field_0x8), unaff_DI, unaff_SS);
        *(Struct664 **)(param_1->field_0xee) = in_AX;
        *(param_1->field_0xf0)               = uVar1;
        return;
    }
    (param_1->field_0xee) = 0x0;
}