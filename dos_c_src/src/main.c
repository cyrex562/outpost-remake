
#include "main.h"

#include "sys_ops/sys_ops_11.h"
#include "types.h"
#include "globals.h"
#include "unk/unk_15.h"
#include "unk/unk_16.h"



int main(int argc, char **argv)
{
    return 0;
}


void init_op_1008_54aa(u8 *param_1, char *param_2, u8 *param_3, u8 *param_4, u16 param_5, u16 param_6, u16 param_7, u16 param_8)

{
    fn_ptr_1 *ppcVar1;
    u16       uVar3;
    u16       in_CX;
    u16       in_DX;
    u8       *puVar4;
    u16       extraout_DX;
    u16       uVar5;
    u16       extraout_DX_00;
    u16       uVar6;
    u16       extraout_DX_01;
    u32       uVar7;
    u32      *puStack12;
    u32       uVar2;
    u16       stack0xfffe;

    if(param_3 != 0x0)
    {
        return;
    }
    dos3_call_op_1000_435c(0x0, in_CX, in_DX, &stack0xfffe, param_8);
    pass1_1000_4d0c(param_5);
    pass1_1000_1fea();
    _PTR_LOOP_1050_03a0 = mem_op_1000_1902(0x0, 0x32, 0x0, 0x12, 0x1000, in_DX);
    _PTR_LOOP_1050_029c = mem_op_1000_1902(0x0, 0x64, 0x0, 0xc, 0x1000, (_PTR_LOOP_1050_03a0 >> 0x10));
    _PTR_LOOP_1050_4fb8 = mem_op_1000_1902(0x0, 0x64, 0x0, 0x10, 0x1000, (_PTR_LOOP_1050_029c >> 0x10));
    _PTR_LOOP_1050_68a2 = mem_op_1000_1902(0x0, 0x64, 0x0, 0xe, 0x1000, (_PTR_LOOP_1050_4fb8 >> 0x10));
    _PTR_LOOP_1050_5744 = mem_op_1000_1902(0x0, 0x1f4, 0x0, 0x42, 0x1000, (_PTR_LOOP_1050_68a2 >> 0x10));
    uVar7               = mem_op_1000_1902(0x0, 0x32, 0x0, 0x6, 0x1000, (_PTR_LOOP_1050_5744 >> 0x10));
    puVar4              = (uVar7 >> 0x10);
    PTR_LOOP_1050_5768  = uVar7;
    PTR_LOOP_1050_038c  = param_4;
    PTR_LOOP_1050_038e  = param_3;
    PTR_LOOP_1050_0390  = param_1;
    PTR_LOOP_1050_576a  = puVar4;
    uVar3               = str_op_1008_60e8(param_2, puVar4);
    _PTR_LOOP_1050_0392 = CONCAT22(puVar4, uVar3);
    mem_op_1000_179c(0xc, puVar4, 0x1000);
    if((puVar4 | uVar3) == 0x0)
    {
        uVar3 = 0x0;
        uVar5 = 0x0;
    }
    else
    {
        struct_op_1008_0000(CONCAT13((puVar4 >> 0x8), CONCAT12(puVar4, uVar3)));
        uVar5 = extraout_DX;
    }
    puStack12 = CONCAT22(uVar5, uVar3);
    if(_PTR_LOOP_1050_0392 != 0x0)
    {
        ppcVar1 = (*puStack12 + 0x4);
        (**ppcVar1)(0x1000, uVar3, uVar5, _PTR_LOOP_1050_0392, (_PTR_LOOP_1050_0392 >> 0x10));
    }
    uVar2   = *puStack12;
    ppcVar1 = uVar2 + 0x4;
    (**ppcVar1)(0x1000, uVar3, uVar5);
    uVar6 = extraout_DX_00;
    win_msg_op_1008_9498(&PTR_LOOP_1050_1000, param_8);
    if(puStack12 != 0x0)
    {
        ppcVar1 = uVar2;
        (**ppcVar1)(0x1000, uVar3, uVar5, 0x1);
        uVar6 = extraout_DX_01;
    }
    uVar7 = mem_op_1000_1b68(uVar6, 0x1000, _PTR_LOOP_1050_03a0, (_PTR_LOOP_1050_03a0 >> 0x10));
    uVar7 = mem_op_1000_1b68((uVar7 >> 0x10), 0x1000, _PTR_LOOP_1050_029c, (_PTR_LOOP_1050_029c >> 0x10));
    uVar7 = mem_op_1000_1b68((uVar7 >> 0x10), 0x1000, _PTR_LOOP_1050_4fb8, (_PTR_LOOP_1050_4fb8 >> 0x10));
    uVar7 = mem_op_1000_1b68((uVar7 >> 0x10), 0x1000, _PTR_LOOP_1050_68a2, (_PTR_LOOP_1050_68a2 >> 0x10));
    mem_op_1000_1b68((uVar7 >> 0x10), 0x1000, _PTR_LOOP_1050_5744, (_PTR_LOOP_1050_5744 >> 0x10));
    return;
}


void init_1000_23be(u16 param_1, u16 param_2, u16 param_3)

{
    init_op_1008_54aa(PTR_LOOP_1050_5f52, CONCAT22(PTR_LOOP_1050_5f50, PTR_LOOP_1050_5f4e), PTR_LOOP_1050_5f4a, PTR_LOOP_1050_5f4c, &u16_1050_1050, param_1, param_2, param_3);
    return;
}
