use crate::{
    pass::pass_1010::pass1_1010_1f62,
    winapi::{mciGetErrorString16, mciSendCommand16},
};
use crate::util::{CONCAT22, SUB42, read_struct_from_addr};
use crate::pass::pass_1000::pass1_1000_4906;
use crate::global::AppContext;

pub fn mci_send_command_1008_53ae(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u16,
    param_3: u16,
    param_4: u16) {

    let local_32: u16;
    let u_stack48: u16;
    let mut local_2e = 0u16;
    let u_stack44: u16;
    let mut u_stack32: u16 = 0;
    let i_stack26: i16;

    let mut local_432 = String::new();
    let mut local_1e = 0u16;
    let mut u_stack22 = 0x28cu16;
    let mut u_stack20 = ctx.data_seg;
    let u_stack18 = param_1;
    let u_stack14 = 0x0;
    let u_stack10 = 0x0;
    let u_stack8 = 0x4000;
    let u_stack6 = param_2;
    let mut send_cmd_result = mciSendCommand16(param_3, local_1e, CONCAT22(0x200, param_4), 0x8030003);
   // u_stack32 = (dvar1 >> 0x10);
    let mut u_stack34 = send_cmd_result;
    if i_stack26 != 0x0 {
        if (u_stack32 | u_stack34) != 0x0 {
            mciGetErrorString16(0x4001538, &mut local_432, param_4);
        }
        pass1_1000_4906(read_struct_from_addr(CONCAT22(param_4, local_2e)), None, 0xc);
        local_2e = param_2;
        u_stack44 = 0x0;
        send_cmd_result = mciSendCommand16(0x1000, local_2e, CONCAT22(0x2, param_4), 0x8060000);
        u_stack34 = send_cmd_result;
        if (u_stack32 | u_stack34) != 0x0 {
            mciGetErrorString16(0x4001538, &mut local_432, param_4);
        }
        local_32 = param_2;
        u_stack48 = 0x0;
        send_cmd_result = mciSendCommand16(
            ctx.s_tile2_bmp_1050_1538 as u16,
            local_32,
            CONCAT22(0x1, param_4),
            0x8040000,
        );
        u_stack34 = send_cmd_result;
        if (u_stack32 | u_stack34) != 0x0 {
            mciGetErrorString16(0x4001538, &mut local_432, param_4);
        }
    }
    return;
}

pub fn mci_send_command_1008_5cb6(param_1: u32, param_2: i16, param_3: u16, unaff_SS: u16) {
    let iVar1: i16;
    let uVar2: u16;
    // let unaff_SS: u16;

    mciSendCommand16(param_3, 0x0, 0x0, 0x8040000);
   // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((iVar1 + 0xa) == 0x0) || ((iVar1 + 0xa) != param_2) {
        (iVar1 + 0x12) = 0x0;
        iVar1 = 0x11;
    } else {
        (iVar1 + 0x10) = 0x0;
        iVar1 = 0x10;
    }
    pass1_1010_1f62(unaff_SS, param_1 & 0xffff | uVar2 << 0x10, iVar1);
    return;
}
