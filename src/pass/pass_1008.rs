// WARNING: Globals starting with '_' overlap smaller symbols at the same address

use crate::cleanup::{cleanup_ui_op_1008_0618, clenaup_win_ui_1018_4d22};
use crate::debug::debug_print_1008_6048;
use crate::defines::{Struct11, Struct18, Struct19, Struct194, Struct20, Struct449, Struct65, Struct76, Struct79, Struct99, Struct_1008_49e8, Struct_1008_4cdc, Struct_1008_4d26, Struct_1008_4d84, U32Ptr, Struct110, Struct197};
use crate::file::file_1008::{file_1008_76e4, read_file_1008_7dee, write_to_file_1008_7cac};
use crate::fn_ptr::fn_ptr_1000::{call_fn_ptr_1000_0dc6, fn_ptr_1000_17ce, fn_ptr_op_1000_1708, fn_ptr_op_1000_24cd};
use crate::fn_ptr::fn_ptr_1000;
use crate::fn_ptr::fn_ptr_1010::fn_ptr_1010_905e;
use crate::global::AppContext;
use crate::mem_1000::{mem_op_1000_0a48, mem_op_1000_160a, mem_op_1000_179c};
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{pass1_1000_07fc, pass1_1000_093a, pass1_1000_0ed4, pass1_1000_3cea, pass1_1000_3d7a, pass1_1000_48a8, pass1_1000_4906, pass1_1000_4d24, pass1_1000_545a, pass1_1000_3de8};
use crate::pass::pass_1010::{pass1_1010_1d80, pass1_1010_1f62, pass1_1010_2050, pass1_1010_2db2, pass1_1010_2e02, pass1_1010_2e30, pass1_1010_3880, pass1_1010_398e, pass1_1010_5f4c, pass1_1010_7efc, pass1_1010_7fd6, pass1_1010_8ef2, pass1_1010_c312};
use crate::pass::pass_1018::{pass1_1018_20ee, pass1_1018_214e, pass1_1018_4dce};
use crate::pass::pass_1020::{pass1_1020_bb8a, pass1_1020_c4a8, string_1020_c0ca};
use crate::pass::pass_1028::{pass1_1028_dc52, pass1_1028_e2e0, pass1_1028_e4ec};
use crate::pass::pass_1030::{fn_ptr_1030_835a, pass1_1030_532e, pass1_1030_8210, pass1_1030_8326, pass1_1030_8334, pass1_1030_8344, pass1_1030_8372, pass1_1030_838e, struct_1030_8128, struct_1030_e2be};
use crate::pass::pass_1038::{empty_1038_540a, load_string_1038_4d28, pass1_1038_3608, pass1_1038_af34};
use crate::pass::pass_1040::pass1_1040_9252;
use crate::string::string_1000::{str_1000_4d58, str_op_1000_3da4, unk_str_op_1000_3d3e, str_op_1000_3dbe};
use crate::string::string_1008::str_op_1008_60e8;
use crate::string::string_1010::{load_string_1010_847e, load_string_1010_84ac};
use crate::struct_ops::struct_1008::{pass1_1008_caa0, set_struct_1008_574a, set_struct_1008_687a, struct_1008_dc90, struct_1008_ec72, struct_op_1008_4214, struct_op_1008_4c98, struct_op_1008_56b4, struct_op_1008_6604, struct_op_1008_9174};
use crate::struct_ops::struct_1008;
use crate::struct_ops::struct_1018::{struct_1018_4790, struct_op_1018_4cda};
use crate::struct_ops::struct_1030::struct_op_1030_1cd8;
use crate::struct_ops::struct_1040::mixed_struct_op_1040_8fb8;
use crate::sys_api::{kill_timer_1008_921c, sys_1000_3f9c};
use crate::ui::ui_1008::{message_box_op_1008_12dc, pass1_1008_ada2, pass1_1008_add2, pass1_1008_aed8, pass1_1008_afde, pass1_1008_b08c, pass1_1008_b0f2, pass1_1008_b11e, post_quit_msg_1008_3af4, save_file_1008_3178, set_sys_color_1008_357e, show_win_1008_96ae};
use crate::ui::ui_1040::{create_window_1040_92dc, mix_win_ui_op_1040_911e, mov_update_win_1040_93aa};
use crate::util::{CARRY2, CONCAT12, CONCAT13, CONCAT22, struct_from_addr, SUB42, ZEXT24, SBORROW2, read_string_from_addr};
use crate::win_struct::{ATOM, HFILE16, HINSTANCE16, HWND16, LRESULT, WNDCLASS16};
use crate::winapi::{GetClassInfo16, RegisterClass16, SetTimer16, swi};
use crate::file::file_1010::unk_io_op_1010_830a;

pub fn pass1_1008_0036(
    ctx: &mut AppContext,
    param_1: &mut Struct449,
    param_2: &mut u16) {
    let u_var1: u16;
    let pu_var2: u32;
    let struct_1: Struct18;
    let ppc_var4: u32;
    let u_var5: u16;
    let mut struct_2: Struct449;

    // uVar6 = (param_1 >> 0x10);
    // i_var6 = param_1;
    param_1.field_0x0 = 0x51e;
    param_1.field_0x2 = 0x1008;
    struct_1 = param_1.field_0x8;
    u_var1 = param_1.field_0xa;
    u_var5 = struct_1;
    if (u_var1 | u_var5) != 0x0 {
        pass1_1008_53aa(u_var5, u_var1);
        *param_2 = 0x1000;
        fn_ptr_1000_17ce(ctx, &mut struct_1, 0x1000);
    }
    struct_1 = ctx._PTR_LOOP_1050_5748;
    ctx._PTR_LOOP_1050_0298 = 0x0;
    if (ctx.PTR__LOOP_1050_5748 != 0x0) {
        pass1_1030_8210(ctx.PTR__LOOP_1050_5748);
        param_2 = 0x1000;
        fn_ptr_1000_17ce(ctx, struct_1, 0x1000);
    }
    struct_1 = ctx._PTR_LOOP_1050_0ed0;
    if (ctx.PTR__LOOP_1050_0ed0 != 0x0) {
        pass1_1010_2050(ctx.PTR__LOOP_1050_0ed0);
        param_2 = 0x1000;
        fn_ptr_1000_17ce(ctx, struct_1, 0x1000);
    }
    struct_1 = ctx._PTR_LOOP_1050_14cc;
    if (ctx.PTR__LOOP_1050_14cc != 0x0) {
        pass1_1010_7efc(ctx.PTR__LOOP_1050_14cc, 0x1010);
        param_2 = 0x1000;
        fn_ptr_1000_17ce(ctx, struct_1, 0x1000);
    }
    struct_1 = ctx._PTR_LOOP_1050_5b7c;
    if (ctx.PTR__LOOP_1050_5b7c != 0x0) {
        pass1_1038_af34();
        param_2 = 0x1000;
        fn_ptr_1000_17ce(ctx, struct_1, 0x1000);
    }
    if (ctx.PTR__LOOP_1050_5bc8 != 0x0) {
        ppc_var4 = *_PTR_LOOP_1050_5bc8;
        (**ppc_var4)(param_2, _PTR_LOOP_1050_5bc8, (ctx.PTR__LOOP_1050_5bc8 >> 0x10), 0x1);
    }
    if (ctx.PTR__LOOP_1050_02a0 != 0x0) {
        ppc_var4 = *_PTR_LOOP_1050_02a0;
        (**ppc_var4)(param_2, _PTR_LOOP_1050_02a0, (ctx.PTR__LOOP_1050_02a0 >> 0x10), 0x1);
    }
    pu_var2 = struct_2.field_0x4;
    u_var1 = struct_2.field_0x6;
    if ((u_var1 | pu_var2) != 0x0) {
        ppc_var4 = *pu_var2;
        (**ppc_var4)(param_2, pu_var2, u_var1, 0x1);
    }
    pass1_1008_9466(param_1);
    return;
}


pub fn pass1_1008_049c(param_1: u16, param_2: u16, param_3: &mut String) {
    let u_var1: u16;
    let pu_var2: U32Ptr;

    if param_3 != 0x0 {
        u_var1 = str_op_1000_3da4(param_3);
        if u_var1 != 0x0 {
            pu_var2 = pass1_1000_545a(param_3 & 0xffff0000 | (param_3 + 0x1),
                                      0x105000cc) as u32;
            if pu_var2 == 0x0 {
                ctx.PTR_LOOP_1050_02ec = pu_var2;
            }
        }
    }
    return;
}


pub fn pass1_1008_04d2(param_1: U32Ptr, param_2: u8) -> u16

{
    pass1_1008_9466(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_04f8(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1008_0036(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_07d8(param_1: u16, param_2: bool, param_3: U32Ptr, param_4: u16) -> bool

{
    let uVar2: u16;
    let uVar1: u16;
    let in_AF: u8;
    let uVar3: u32;

    if (ctx.PTR__LOOP_1050_5748 == 0x0) {
        uVar1 = 0x1000;
        mem_op_1000_179c(0xa, param_3, 0x1000);
        uVar2 = param_3 | param_2;
        if (uVar2 != 0x0) {
            uVar1 = 0x1030;
            struct_1030_8128(CONCAT22(param_3 as u16, param_2), uVar2, param_4);
        }
        if (ctx.PTR__LOOP_1050_5748 == 0x0) {
            debug_print_1008_6048(s_New_failed_in_Op__Op__Simulator_1050_0110, uVar1, param_4);
            fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
        }
        uVar3 = pass1_1028_e2e0(ctx.PTR__LOOP_1050_65e2, uVar2, 0x8);
        uVar3 = pass1_1028_e2e0(ctx.PTR__LOOP_1050_65e2, ((uVar3 >> 0x10) as u16), 0x8);
        pass1_1028_e2e0(ctx.PTR__LOOP_1050_65e2, ((uVar3 >> 0x10) as u16), 0xff);
        pass1_1030_838e(ctx.PTR__LOOP_1050_5748, param_4, in_AF);
        param_2 = pass1_1030_8334(ctx.PTR__LOOP_1050_5748);
    }
    return param_2;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_087e(param_1: u16, param_2: U32Ptr, param_3: u16, param_4: u8) {
    let uVar1: u16;
    let uVar2: u16;
    let uVar3: u32;
    let local_112: u16;
    let uStack272: u16;
    let uStack6: u16;
    let puStack4: U32Ptr;

    uVar2 = 0x1000;
    mem_op_1000_179c(0xa, param_2, 0x1000);
    uVar1 = (param_2 | param_1) as u16;
    uStack6 = param_1;
    puStack4 = param_2;
    if (uVar1 != 0x0) {
        uVar2 = 0x1030;
        struct_1030_8128(CONCAT22(param_2 as u16, param_1), uVar1, param_3);
    }
    if (ctx.PTR__LOOP_1050_5748 ==  0x0) {
        debug_print_1008_6048(s_New_failed_in_Op__Op__Simulator_1050_0130, uVar2, param_3);
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    uVar3 = pass1_1028_e2e0(ctx.PTR__LOOP_1050_65e2, uVar1, 0x8);
    pass1_1028_e2e0(ctx.PTR__LOOP_1050_65e2, ((uVar3 >> 0x10) as u16), 0x8);
    pass1_1030_532e(CONCAT22(param_3, &local_112), 0xff000000, param_3, param_4);
    fn_ptr_1030_835a(ctx.PTR__LOOP_1050_5748, CONCAT22(param_3, &local_112));
    pass1_1030_838e(ctx.PTR__LOOP_1050_5748, param_3, param_4);
    local_112 = 0x389a;
    uStack272 = 0x1008;
    pass1_1030_8334(ctx.PTR__LOOP_1050_5748);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_0932() -> u32

{
    let uVar1: u32;

    if (ctx.PTR__LOOP_1050_14cc != 0x0) {
        pass1_1010_7fd6(ctx.PTR__LOOP_1050_14cc);
    }
    mem_1000_0016(ctx.PTR__LOOP_1050_03a0, 0x1000);
    mem_1000_0016(ctx.PTR__LOOP_1050_029c, 0x1000);
    mem_1000_0016(ctx.PTR__LOOP_1050_4fb8, 0x1000);
    mem_1000_0016(ctx.PTR__LOOP_1050_68a2, 0x1000);
    mem_1000_0016(ctx.PTR__LOOP_1050_5744, 0x1000);
    uVar1 = mem_1000_0016(ctx.PTR__LOOP_1050_5f2c, 0x1000);
    return uVar1;
}


pub fn pass1_1008_0984(param_1: i16, param_2: u16, param_3: i16, param_4: u16, param_5: u16) {
    let uVar1: u32;
    let ppcVar2: u32;

    set_sys_color_1008_357e(CONCAT22(param_2, param_1 as u16), param_3, param_4 as i16, param_5);
    if ((param_1 + 0xe8) != 0x0) {
        uVar1 = (param_1 + 0xe8) as u32;
        ppcVar2 = ((param_1 + 0xe8) + 0x98) as u32;
        (**ppcVar2)(param_4, uVar1, (uVar1 >> 0x10), param_3);
    }
    return;
}


pub fn pass1_1008_0a92(param_1: u32, param_2: i16) {
    let ppcVar1: u32;
    let iVar2: i16;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    iVar2 = param_1 as i16;
    if ((iVar2 + 0xee) != 0x0) {
        ppcVar1 = ((iVar2 + 0xee) + 0x90) as u32;
        (**ppcVar1)(param_2, (iVar2 + 0xee));
    }
    if ((iVar2 + 0xe8) != 0x0) {
        ppcVar1 = ((iVar2 + 0xe8) + 0x90) as u32;
        (**ppcVar1)(param_2, (iVar2 + 0xe8));
    }
    if (ctx.PTR__LOOP_1050_0388 != 0x0) {
        ppcVar1 = *_PTR_LOOP_1050_0388;
        (**ppcVar1)(param_2, _PTR_LOOP_1050_0388, (ctx.PTR__LOOP_1050_0388 >> 0x10), 0x1);
    }
    post_quit_msg_1008_3af4(param_2);
    return;
}


pub fn pass1_1008_1246(param_1: u32) {
    let ppcVar1: u32;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0xe8) != 0x0) {
        ppcVar1 = ((param_1 + 0xe8) + 0x4c);
        (**ppcVar1)();
    }
    return;
}


pub fn pass1_1008_1272(param_1: u32, param_2: i16) {
    let ppcVar1: u32;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0xe8) != 0x0) {
        ppcVar1 = ((param_1 + 0xe8) + 0x88);
        (**ppcVar1)();
        return;
    }
    pass1_1008_9cc4(param_1 & 0xffff | uVar2 << 0x10, param_2);
    return;
}


pub fn pass1_1008_12aa(param_1: u32) {
    let ppcVar1: u32;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0xe8) != 0x0) {
        ppcVar1 = ((param_1 + 0xe8) + 0x8c);
        (**ppcVar1)();
        return;
    }
    pass1_1008_9ce0();
    return;
}


pub fn pass1_1008_3018(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let UVar1: i32;
    let uVar2: u16;
    let iVar3: i16;
    let uVar4: u16;
    let uStack266: u16;
    let uStack262: u32;
    local_102: u8[0x100];

    local_102[0] = '\0';
    uStack262 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    // uVar2 = (uStack262 >> 0x10);
    iVar3 = uStack262 as i16;
    UVar1 = (iVar3 + 0x12) as i32;
    uVar4 = (iVar3 + 0x14) as u16;
    uStack266 = UVar1 as u16;
    if ((uVar4 | uStack266) == 0x0) {
        pass1_1008_30cc(param_1, 0x0, uVar4, param_3, param_4);
    } else {
        unk_str_op_1000_3d3e(CONCAT22(param_4, local_102), (iVar3 + 0x1a));
        uVar4 = str_op_1000_3da4(CONCAT22(param_4, local_102));
        if (local_102[uVar4 - 0x1] != '\\') {
            local_102[uVar4] = '\\';
            local_102[uVar4 + 0x1] = '\0';
        }
        pass1_1000_3cea(CONCAT22(param_4, local_102) as i32, UVar1);
        if (local_102[0] != '\0') {
            message_box_op_1008_12dc(param_1, CONCAT22(param_4, local_102), 0x1000, param_4);
            return;
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_30cc(param_1: u32, param_2: u16, param_3: u16, param_4: i16, param_5: u16) {
    let puVar1: U32Ptr;
    let puVar2: U32Ptr;
    let puVar3: U32Ptr;
    let uVar4: u16;
    local_210: u8[0xa];
    let local_206: [u8; 100];
    let uStack262: u16;
    let uStack260: u16;
    local_102: u8[0x100];

    local_102[0] = '\0';
    save_file_1008_3178(param_1, 0x2, param_5);
    puVar1 = (param_3 | param_2) as u32;
    if (puVar1 != 0x0) {
        uStack262 = param_2;
        uStack260 = param_3;
        unk_str_op_1000_3d3e(CONCAT22(param_5, local_102), CONCAT22(param_3, param_2));
        str_1000_4d58(CONCAT22(param_5, local_102), 0x0, 0x0,
                      CONCAT22(param_5, local_206), CONCAT22(param_5, local_210));
        if (local_210[0] != '\0') {
            pass1_1000_3cea(CONCAT22(param_5, local_206), CONCAT22(param_5, local_210));
        }
        puVar3 = local_206;
        uVar4 = param_5;
        puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_5, puVar1, param_4);
        pass1_1010_5f4c(puVar2, CONCAT22(uVar4, puVar3 as u16),
                        ((puVar2 >> 0x10) as u16));
        if (local_102[0] != '\0') {
            message_box_op_1008_12dc(param_1, CONCAT22(param_5, local_102), 0x1010, param_5);
        }
    }
    return;
}


pub fn pass1_1008_3714(param_1: u32) {
    fn_ptr_1000::call_fn_ptr_1008_3e0e(param_1);
    return;
}


pub fn pass1_1008_372c(param_1: i16, param_2: u16) -> u32

{
    return CONCAT22(param_2, (param_1 + 0xa) as u16);
}


pub fn pass1_1008_373c() {
    return;
}


pub fn pass1_1008_3740() {
    return;
}


pub fn pass1_1008_3744() {
    return;
}


pub fn pass1_1008_3748() {
    return;
}


pub fn pass1_1008_374c() {
    return;
}


pub fn pass1_1008_3750() {
    return;
}


pub fn pass1_1008_3754() {
    return;
}


pub fn pass1_1008_3758() -> u16

{
    return 0x1;
}


pub fn pass1_1008_375e() {
    return;
}


pub fn pass1_1008_3762() {
    return;
}


pub fn pass1_1008_3766() {
    return;
}


pub fn pass1_1008_377a() {
    return;
}


pub fn pass1_1008_377e(param_1: U32Ptr, param_2: u8) -> u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_37aa(param_1: U32Ptr, param_2: u8) -> u16

{
    let uVar1: &mut Struct450;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    uVar1 = param_1;
    *param_1 = 0x380a;
    uVar1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    uVar1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_37e4(param_1: u32, param_2: u8) -> u32

{
    cleanup_ui_op_1008_0618(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_392e(param_1: U32Ptr, param_2: u16) -> u16

{
    let iVar1: i16;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1 as i16;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x3aa8;
    (iVar1 + 0x2) = 0x1008;
    (iVar1 + 0x4) = param_2 as i16;
    *param_1 = 0x3ab0;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x3aa0;
    (iVar1 + 0x2) = 0x1008;
    return param_1 as u16;
}


pub fn pass1_1008_397a(param_1: U32Ptr) {
    let iVar1: &mut Struct452;
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0x3aa0;
    iVar1.field_0x2 = 0x1008;
    *param_1 = 0x3ab0;
    iVar1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    iVar1.field_0x2 = 0x1008;
    return;
}


pub fn pass1_1008_3a10() {
    return;
}


pub fn pass1_1008_3a14(param_1: U32Ptr, param_2: u8) -> u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_3a40(param_1: U32Ptr, param_2: u8) -> u16

{
    let uVar1: &mut Struct451;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    uVar1 = param_1;
    *param_1 = 0x3ab0;
    uVar1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    uVar1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_3a7a(param_1: u32, param_2: u8) -> u32

{
    pass1_1008_397a(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_3afe(param_1: &mut Struct18, param_2: u8) {
    pass1_1008_57c4((param_1 & 0xffff0000 | (param_1.field_0xd2)));
    param_1.field_0x0 = 0x380a;
    param_1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    param_1.field_0x2 = 0x1008;
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}


pub fn pass1_1008_3bd6(param_1: &mut Struct160, param_2: u16, param_3: u16, param_4: u32,
                       param_5: u16, param_6: u32, param_7: u32, param_8: u16, param_9: u16)

{
    mixed_struct_op_1040_8fb8(CONCAT22(param_2, param_1), param_3, 0x0, param_5,
                              param_6 as u16, ((param_6 >> 0x10) as u16), param_7 as u16,
                              ((param_7 >> 0x10) as u16), param_8, &ctx.PTR_LOOP_1050_1040, param_9);
    CONCAT22(param_2, param_1) = 0x3cfc;
    param_1.field_0x2 = 0x1008;
    param_1.field_0x36 = 0x0;
    param_1.field_0x26 = 0x0;
    pass1_1040_9252(CONCAT22(param_2, param_1), &ctx.PTR_LOOP_1050_1040);
    create_window_1040_92dc(CONCAT22(param_2, param_1), &ctx.PTR_LOOP_1050_1040);
    mov_update_win_1040_93aa(CONCAT22(param_2, param_1), param_4 as i16,
                             ((param_4 >> 0x10) as u16), &ctx.PTR_LOOP_1050_1040);
    return;
}


pub fn pass1_1008_3cd6(param_1: &mut Struct18, param_2: u8) -> &mut Struct18

{
    mix_win_ui_op_1040_911e(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass_1008_3d44(param_1: &mut Struct18, param_2: u8) -> &mut Struct18

{
    param_1.field_0x0 = 0x380a;
    param_1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    param_1.field_0x2 = 0x1008;
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_3e54(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16) -> u16

{
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    *param_1 = param_4;
    (param_1 + 0x2) = param_3 as u32;
    (param_1 + 0x4) = param_2 as u32;
    return param_1 as u16;
}


pub fn pass1_1008_3e76(param_1: &mut Vec<u16>, param_2: u16, param_3: i16, param_4: u16) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    *param_1 = param_4;
    (param_1 + 0x2) = param_3;
    (param_1 + 0x4) = param_2;
    return;
}


pub fn pass1_1008_3e94(param_1: U32Ptr, param_2: U32Ptr, param_3: U32Ptr) {
    *param_3 = *param_1;
    *param_2 = (param_1 + 0x2);
    return;
}


pub fn pass1_1008_3eb4(param_1: U32Ptr, param_2: U32Ptr, param_3: U32Ptr, param_4: U32Ptr) {
    let uVar1: u16;

    *param_4 = *param_1;
    // uVar1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x2);
    *param_2 = (param_1 + 0x4);
    return;
}


pub fn pass1_1008_3ee2(param_1: &mut i16, param_2: &mut i16) {
    let iVar1: i16;
    let iVar2: i16;
    let uVar3: u16;
    let uVar4: u16;

    iVar1 = *param_2 - *param_1;
    if (iVar1 < 0x0) {
        iVar1 = -iVar1;
    }
    *param_1 = iVar1 + 0x1;
    // uVar3 = (param_2 >> 0x10);
    // uVar4 = (param_1 >> 0x10);
    iVar2 = param_1;
    iVar1 = (param_2 + 0x2) - (iVar2 + 0x2);
    if (iVar1 < 0x0) {
        iVar1 = -iVar1;
    }
    (iVar2 + 0x2) = iVar1 + 0x1;
    iVar1 = (param_2 + 0x4) - (iVar2 + 0x4);
    if (iVar1 < 0x0) {
        iVar1 = -iVar1;
    }
    (iVar2 + 0x4) = iVar1 + 0x1;
    return;
}


pub fn pass1_1008_3f32(param_1: &mut i16, param_2: &mut i16) {
    let piVar1: U32Ptr;
    let uVar2: u16;
    let uVar3: u16;

    *param_1 = *param_1 + *param_2;
    // uVar2 = (param_2 >> 0x10);
    // uVar3 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x2);
    *piVar1 = *piVar1 + (param_2 + 0x2);
    piVar1 = (param_1 + 0x4);
    *piVar1 = *piVar1 + (param_2 + 0x4);
    return;
}


pub fn pass1_1008_3f62(param_1: U32Ptr, param_2: U32Ptr) {
    let uVar1: u16;
    let uVar2: u16;

    *param_1 = *param_2;
    // uVar1 = (param_2 >> 0x10);
    // uVar2 = (param_1 >> 0x10);
    (param_1 + 0x2) = (param_2 + 0x2);
    (param_1 + 0x4) = (param_2 + 0x4);
    return;
}


pub fn pass1_1008_4016(
    ctx: &mut AppContext,
    param_1: &mut Struct76) {
    struct_op_1008_56b4(param_1);
    (param_1.field_0x6) = 0x0;
    (param_1.field_0xa) = 0x0;
    (param_1.field_0xe) = 0x0;
    (param_1.field_0x10) = 0x0;
    (param_1.field_0x14) = 0x0;
    (param_1.field_0x18) = 0x0;
    (param_1.field_0x1c) = 0x0;
    param_1.field_0x0 = &ctx.PTR_LOOP_1050_48de;
    param_1.field_0x2 = 0x1008;
    return;
}


pub unsafe fn pass1_1008_405c(
    ctx: &mut AppContext,
    param_1: &mut Struct76,
    param_2: u32,
    param_3: i16,
    param_4: i16) {
    let u_var1: u32;
    let s_var2: i64;
    let i_var3: i16;
    let l_var4: i32;
    let pu_var5: &mut Struct79;
    let u_var6: u16;
    let u_stack10: u32;

    struct_op_1008_56b4(param_1);

    &param_1.field_0x6 = 0x0;
    (&param_1.field_0x8 + 0x2) = 0x0;
    &param_1.field_0xe = 0x0;
    (&param_1.field_0xe + 0x2) = 0x0;
    param_1.field_0x14 = 0x0;
    &param_1.field_0x18 = 0x0;
    param_1.field_0x1c = 0x0;
    param_1.field_0x0 = &ctx.PTR_LOOP_1050_48de;
    param_1.field_0x2 = 0x1008;
    i_var3 = param_4 * 0x8 + 0x1f;
    i_var3 = ((i_var3 + (i_var3 >> 0xf & 0x1f)) >> 0x5) << 0x2;
    u_stack10 = param_3 as u32;
    l_var4 = (i_var3 * param_3 + 0x436) as i32;
    l_var4 = mem_op_1000_0a48(0x1, l_var4 as u8, ((l_var4 >> 0x10) as u32), ctx._PTR_LOOP_1050_5f2c, 0x1000);
    param_1.field_0x6 = l_var4;
    &param_1.field_0x8 = (l_var4 >> 0x10);
    pass1_1008_47cc((param_1 & 0xffff | u_var6 << 0x10));
    &param_1.field_0x18 = i_var3;
    param_1.field_0x1a = i_var3 >> 0xf;
    (&param_1.field_0xe + 0x2) = 0x28;
    u_var1 = (&param_1.field_0xe + 0x2);
    (u_var1 + 0x4) = param_4 as u32;
    u_var1 = (&param_1.field_0xe + 0x2);
    (u_var1 + 0x8) = u_stack10;
    u_var1 = (&param_1.field_0xe + 0x2);
    (u_var1 + 0xc) = 0x1;
    u_var1 = (&param_1.field_0xe + 0x2);
    (u_var1 + 0xe) = 0x8;
    u_var1 = (&param_1.field_0xe + 0x2);
    (u_var1 + 0x10) = 0x0;
    s_var2 = param_1.field_0x18 * u_stack10;
    pu_var5 = s_var2 >> 0x20;
    u_var1 = (&param_1.field_0xe + 0x2);
    (u_var1 + 0x14) = s_var2 as u32;
    u_var1 = (&param_1.field_0xe + 0x2);
    (u_var1 + 0x20) = 0x100;
    u_var1 = (&param_1.field_0xe + 0x2);
    (u_var1 + 0x24) = 0x100;
    pass1_1008_4834(param_1);
    pass1_1008_4d84((&param_1.field_0x8 + 0x2), param_2, pu_var5);
    return;
}


pub fn pass1_1008_41bc(
    ctx: &mut AppContext,
    struct_1: &mut Struct18) {
    let mut struct_2 = struct_1;
    struct_1.field_0x0 = &ctx.PTR_LOOP_1050_48de;
    struct_2.field_0x2 = 0x1008;
    let pu_var1 = struct_2.field_0xa;
    let u_var2 = struct_2.field_0xc;
    if (u_var2 | pu_var1) != 0x0 {
        let fn_ptr = *pu_var1;
        (**fn_ptr)();
    }
    if struct_2.field_0x6 != 0x0 {
        let struct_3 = struct_2.field_0x6;
        call_fn_ptr_1000_0dc6(ctx, struct_3, 0x1000);
    }
    struct_1.field_0x0 = 0x389a;
    struct_2.field_0x2 = 0x1008;
    return;
}


pub fn pass1_1008_431c(param_1: u32, param_2: u8) {
    let pu_var1: u32;
    let u_var2: u32;
    let b_var3: bool;
    let u_var4: u32;
    let i_var5: i16;
    let u_var6: u16;
    let u_stack6: u32;

    i_var5 = param_1 as i16;
    if (i_var5 + 0x6) == 0x0 {
        pass1_1008_47cc((param_1 & 0xffff | u_var6 << 0x10));
    }
    if ((i_var5 + 0x8) | (i_var5 + 0x6)) == 0x0 {
        b_var3 = false;
    } else {
        if ((i_var5 + 0xc) | (i_var5 + 0xa)) == 0x0 {
            pass1_1008_4834((param_1 & 0xffff | u_var6 << 0x10));
        }
        b_var3 = true;
    }
    if b_var3 {
        if ((i_var5 + 0x16) | (i_var5 + 0x14)) == 0x0 {
            return;
        }
        u_stack6 = 0x0;
        loop {
            u_var2 = (i_var5 + 0x10) as u32;
            pu_var1 = (u_var2 + 0x8);
            if *pu_var1 == u_stack6 || *pu_var1 < u_stack6 { break; }
            u_var4 = u_stack6;
            pass1_1008_4544(param_1);
            u_var2 = (i_var5 + 0x10) as u32;
            pass1_1000_4906((u_var4 & 0xffff | u_stack6._2_2_ << 0x10),
                            param_2, ((u_var2 + 0x4) as u16));
            u_stack6 += 0x1;
        }
    }
    return;
}


pub fn pass1_1008_43cc(param_1: u32) -> u32

{
    let b_var1: bool;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1 as i16;
    if (i_var2 + 0x6) == 0x0 {
        pass1_1008_47cc((param_1 & 0xffff | u_var3 << 0x10));
    }
    if (i_var2 + 0x6) == 0x0 {
        b_var1 = false;
    } else {
        if (i_var2 + 0xa) == 0x0 {
            pass1_1008_4834((param_1 & 0xffff | u_var3 << 0x10));
        }
        b_var1 = true;
    }
    if !b_var1 {
        return 0x0;
    }
    return CONCAT22(((i_var2 + 0x16) as u16), ((i_var2 + 0x14) as u16));
}


pub fn pass1_1008_4426(param_1: u32) -> u32

{
    let b_var1: bool;
    let u_var3: u16;

    let mut i_var2 = param_1;
    if (i_var2 + 0x6) == 0x0 {
        pass1_1008_47cc((param_1 & 0xffff | u_var3 << 0x10));
    }
    if (i_var2 + 0x6) == 0x0 {
        b_var1 = false;
    } else {
        if (i_var2 + 0xa) == 0x0 {
            pass1_1008_4834((param_1 & 0xffff | u_var3 << 0x10));
        }
        b_var1 = true;
    }
    if !b_var1 {
        return 0x0;
    }
    return CONCAT22(((i_var2 + 0xc) as u16), ((i_var2 + 0xa) as u16));
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1008_4480(param_1: u32, param_2: U32Ptr, param_3: &mut Struct76, param_4: u16) {
    let i_var1: i16;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let i_stack26: i16;
    let mut pc_stack24: String;
    let mut pc_stack20: String;
    let i_stack16: i16;
    let local_6: i16;
    let local_4: [u8; 2];

    pass1_1008_3e94(param_2, CONCAT22(param_4, &local_6),
                    CONCAT22(param_4, local_4));
    u_var6 = pass1_1008_4772(param_3);
    // u_var4 = (u_var6 >> 0x10);
    i_var1 = (u_var6 + 0x4) as i16;
    i_var2 = (u_var6 + 0x8) as i16;
    // TODO: refactor for loop
    // for (i_stack16 = 0x0; i_stack16 < i_var2; i_stack16 += 0x1) {
    //   u_var5 = local_6 >> 0xf;
    //   i_var3 = local_6;
    //   local_6 = local_6 + 0x1;
    //   pass1_1008_4544(param_1);
    //   pc_stack20 = CONCAT22(u_var5,i_var3);
    //   u_var6 = SEXT24(i_stack16);
    //   pass1_1008_4544(param_3);
    //   pc_stack24 = (u_var6 & 0xffff | u_var5 << 0x10);
    //   i_stack26 = i_var1;
    //   while (i_stack26 != 0x0) {
    //     if (*pc_stack24 != -0x1) {
    //       *pc_stack20 = *pc_stack24;
    //     }
    //     pc_stack24 = CONCAT22((pc_stack24 >> 0x10) +
    //                                  (-(0xfffe < pc_stack24) & 0x6c),
    //                                  pc_stack24 + 0x1);
    //     pc_stack20 = CONCAT22((pc_stack20 >> 0x10) +
    //                                  (-(0xfffe < pc_stack20) & 0x6c),
    //                                  pc_stack20 + 0x1);
    //     i_stack26 = i_stack26 + -0x1;
    //   }
    // }
    return;
}


pub fn pass1_1008_4544(param_1: u32) {
    let b_var1: bool;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1 as i16;
    if (i_var2 + 0x6) == 0x0 {
        pass1_1008_47cc((param_1 & 0xffff | u_var3 << 0x10));
    }
    if (i_var2 + 0x6) == 0x0 {
        b_var1 = false;
    } else {
        if (i_var2 + 0xa) == 0x0 {
            pass1_1008_4834((param_1 & 0xffff | u_var3 << 0x10));
        }
        b_var1 = true;
    }
    if !b_var1 {
        return;
    }
    return;
}


pub fn pass1_1008_4772(param_1: Option<&mut Struct43>) -> &mut Struct19

{
    let b_var1: bool;

    if &param_1.field_0x6 == 0x0 {
        pass1_1008_47cc(param_1);
    }
    if &param_1.field_0x6 == 0x0 {
        b_var1 = false;
    } else {
        if (&param_1.field_0x8 + 0x2) == 0x0 {
            pass1_1008_4834(param_1);
        }
        b_var1 = true;
    }
    if !b_var1 {
        return 0x0;
    }
    return CONCAT22(param_1.field_0x12, (&param_1.field_0xe + 0x2));
}


pub fn pass1_1008_47cc(param_1: &mut Struct76) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let i_var6: i16;
    let u_stack14: u32;
    let i_var4: i16;

    if (param_1 + 0x6) != 0x0 {
        u_var1 = (param_1.field_0x6);
        i_var6 = (param_1.field_0x8);
        i_var4 = u_var1 as i16;
        u_var3 = (i_var4 + 0xe) as u16;
        (param_1.field_0x10) = u_var1 & 0xffff0000 | u_var3;
        (param_1.field_0x14) = i_var4 + 0x436;
        (param_1.field_0x16) = i_var6 + (-(0xfbd7 < u_var3) & 0x6c);
        u_var2 = (param_1.field_0x10);
        // u_var8 = (u_var2 >> 0x10);
        i_var6 = u_var2 as i16;
        u_stack14 = (i_var6 + 0xe) as u32;
        (param_1.field_0x18) = (u_stack14 * (i_var6 + 0x4) + 0x1f) / 0x20 << 0x2;
    }
    return;
}


pub fn pass1_1008_4834(
    ctx: &mut AppContext,
    param_1: &mut Struct76,
    extraout_dx: u16,
    extraout_dx_00: u16) {
    let ppc_var1: u32;
    let pu_var2: u32;
    let u_var3: u32;
    // let extraout_dx: U32Ptr;
    let pu_var4: U32Ptr;
    // let extraout_dx_00: u16;
    let struct_var5_1: &mut Struct76;
    let struct_var5: &mut Struct76;
    let pa_stack10: &mut Struct76;

    // struct_var5 = (param_1 >> 0x10);
    struct_var5_1 = param_1;
    pu_var2 = (&struct_var5_1.field_0x8 + 0x2);
    pu_var4 = struct_var5_1.field_0xc;
    if (pu_var4 | pu_var2) != 0x0 {
        ppc_var1 = *pu_var2;
        (**ppc_var1)();
        pu_var4 = extraout_dx as u32;
    }
    mem_op_1000_179c(ctx, 0x14, pu_var4, 0x1000);
    pa_stack10 = CONCAT22(pu_var4 as u16, pu_var2 as u16);
    if (pu_var4 | pu_var2) != 0x0 {
        u_var3 = (&struct_var5_1.field_0xe + 0x2);
        u_var3 = u_var3 & 0xffff0000 | (u_var3 + 0x28);
        struct_op_1008_4c98(pa_stack10, u_var3, 0x100);
        (&struct_var5_1.field_0x8 + 0x2) = u_var3;
        struct_var5_1.field_0xc = extraout_dx_00;
        return;
    }
    (&struct_var5_1.field_0x8 + 0x2) = 0x0;
    return;
}


pub fn pass1_1008_48aa(param_1: u32) -> u16

{
    return (param_1 + 0xe) as u16;
}


pub fn pass1_1008_48b8(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8) -> &mut Struct18 {
    pass1_1008_41bc(ctx, param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Instruction at (ram,0x10084942) overlaps instruction at (ram,0x10084941)
//

pub fn pass1_1008_48de(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u32,
    param_3: i16,
    param_4: u16,
    param_5: U32Ptr,
    param_6: i16,
    param_7: i16,
    param_8: U32Ptr,
    param_9: u16,
    param_10: u16,
    param_11: u8,
    param_12: u16,
    param_13: u8) {
    let pb_var1: U32Ptr;
    let u_var2: u32;
    let b_var3: u8;
    let u_var4: u16;
    let b_var5: u8;
    let u_var6: u16;
    let pu_var7: U32Ptr;
    let i_var8: i16;
    let u_var9: u16;

    u_var6 = param_4 & 0xff | ((param_4 >> 0x8) + param_4 + param_11) << 0x8;
    pu_var7 = (param_6 + 0x1) as u32;
    pb_var1 = (param_5 + param_7);
    b_var5 = (param_4 & 0xff) as u8;
    *pb_var1 = *pb_var1 | b_var5;
    b_var3 = 0x46;
    pb_var1 = (param_5 + param_7);
    *pb_var1 = *pb_var1 | b_var5;
    if param_3 == 0x1 {
        pb_var1 = (param_5 + param_7);
        *pb_var1 = *pb_var1 | b_var5;
        i_var8 = param_7 + 0x1;
        pb_var1 = (param_5 + i_var8);
        b_var5 = param_12 as u8;
        *pb_var1 = *pb_var1 | b_var5;
        pb_var1 = (param_5 + i_var8);
        *pb_var1 = *pb_var1 | b_var5;
        *param_8 = b_var3;
        pb_var1 = (param_5 + i_var8);
        *pb_var1 = *pb_var1 | b_var5;
        u_var6 = param_12;
        if *pb_var1 != 0x0 {
            pb_var1 = (param_5 + i_var8);
            *pb_var1 = *pb_var1 | b_var5;
            pu_var7 = (&param_12 + 0x1) as u32;
            param_5 = (param_2 >> 0x8);
            CONCAT13(param_13 as u16, param_2._1_3_) = 0x389a;
            param_5[0x1] = 0x1008;
            param_9 = (CONCAT13(param_13 as u16, param_2._1_3_) >> 0x10);
            (param_5 + 0x2) = 0x0;
            (param_5 + 0x4) = 0x0;
            param_5[0x6] = 0xffff;
            (param_5 + 0x7) = 0x0;
            (param_5 + 0x9) = 0x0;
            (param_5 + 0xb) = 0x0;
            (param_5 + 0xd) = 0x0;
            param_5[0xf] = 0x0;
        }
    } else {
        param_5[0x11] = b_var3 | 0x800;
    }
    param_5[0x11] = (pu_var7 + 0xa);
    *param_5 = &ctx.PTR_LOOP_1050_4c4c;
    param_5[0x1] = 0x1008;
    u_var4 = str_op_1008_60e8(ctx, (pu_var7 + 0xc), u_var6);
    u_var2 = (pu_var7 + 0x6);
    i_var8 = u_var2 as i16;
    (i_var8 + 0x8) = u_var4 as i16;
    (i_var8 + 0xa) = u_var6 as i16;
    return;
}


pub fn pass1_1008_4b5e(param_1: U32Ptr) -> u32

{
    let ppc_var1: u32;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1 as i16;
    if (i_var3 + 0x1e) == 0x0 {
        ppc_var1 = (*param_1 + 0x8);
        i_var2 = (**ppc_var1)();
        if i_var2 == 0x0 {
            return 0x0;
        }
    }
    return CONCAT22(((i_var3 + 0x6) as u16), ((i_var3 + 0x4) as u16));
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_4b8e(
    ctx: &mut AppContext,
    param_1: &mut Struct_1008_49e8,
    param_2: &mut Struct19,
    param_3: i16,
    param_4: &mut WNDCLASS16) {
    let u_var1: u32;
    let u_var2: u16;
    let pu_var3: &mut Struct20;
    let i_stack18: i16;
    let i_stack16: i16;
    let u_stack10: i16;

    pu_var3 = mixed_1010_20ba(ctx,
                              ctx.PTR__LOOP_1050_0ed0,
                              0x48,
                              param_4,
                              param_2,
                              param_3,
                              0);
    u_var1 = (pu_var3 + 0x18);
    i_stack18 = (pu_var3 + 0x16) / 0x2;
    // TODO: refactor for loop
    // for (i_stack16 = 0x0; iStack10 = u_var1, u_var2 = (param_1 >> 0x10),
    //     i_stack16 < i_stack18; i_stack16 += 0x1) {
    //   pass1_1008_4d26((param_1 + 0x4),
    //
    //                   (u_var1 & 0xffff0000 | (i_stack16 * 0x4 + iStack10)),
    //                   i_stack16);
    // }
    // for (i_stack18 = 0x100 - i_stack18; i_stack18 < 0x100; i_stack18 += 0x1) {
    //   pass1_1008_4d26((param_1 + 0x4),
    //
    //                   (u_var1 & 0xffff0000 | (i_stack16 * 0x4 + iStack10)),
    //                   i_stack18);
    //   i_stack16 += 0x1;
    // }
    return;
}


pub fn pass1_1008_4cdc(param_1: &mut Struct18) {
    param_1.field_0x0 = 0x4f1c;
    param_1.field_0x2 = 0x1008;
    fn_ptr_1000_17ce(ctx, &mut param_1.field_0xe, 0x1000);
    if param_1.field_0x12 != 0x0 {
        fn_ptr_1000_17ce(ctx, &mut param_1.field_0x4, 0x1000);
    }
    param_1.field_0x0 = 0x389a;
    param_1.field_0x2 = 0x1008;
    return;
}


pub unsafe fn pass1_1008_4d26(param_1: &mut Struct_1008_4d26, param_2: U32Ptr, param_3: i16) -> bool

{
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let l_var3: i32;
    let i_var4: &mut Struct649;
    if ((param_1.field_0x4 != 0x0) && (-0x1 < param_3)) && (pi_var1 = &param_1.field_0xc, *pi_var1 != param_3 && param_3 <= *pi_var1) {
        u_var2 = (param_2 + 0x2) as u16;
        l_var3 = param_1.field_0x4 as i32;
        // u_var4 = (l_var3 >> 0x10);
        i_var4 = l_var3;
        (i_var4 + param_3 * 0x4) = *param_2;
        (i_var4 + param_3 * 0x4 + 0x2) = u_var2;
        return true;
    }
    return false;
}


pub fn pass1_1008_4d72(param_1: u32) -> u32

{
    return CONCAT22(((param_1 + 0x6) as u16), ((param_1 + 0x4) as u16));
}


pub unsafe fn pass1_1008_4d84(
    param_1: &mut Struct_1008_4d84,
    param_2: u32,
    param_3: &mut Struct19) {
    let u_var1: u16;

    if param_1.field_0x12 != 0x0 {
        param_1.field_0xc = (param_2 + 0xc);
        fn_ptr_1000_17ce(ctx, param_1.field_0x4, 0x1000);
        param_1.field_0x4 = 0x0;
        u_var1 = param_1.field_0xc << 0x2;
        mem_op_1000_179c(ctx, u_var1, param_3, 0x1000);
        param_1.field_0x4 = u_var1;
        (param_1.field_0x4 + 0x2) = param_3;
    }
    if param_1.field_0xc != 0x100 {
        return;
    }
    pass1_1000_48a8(param_1.field_0x4, (param_2 + 0x4), 0x400);
    return;
}


pub fn pass1_1008_4ef6(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8)

{
    pass1_1008_4cdc(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
}


pub fn pass1_1008_5068(param_1: &mut Struct76, param_2: &mut Struct83) {
    struct_op_1008_4214(param_1, param_2);
    return;
}


pub fn pass1_1008_507c(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8) -> &mut Struct18 {
    pass1_1008_41bc(ctx, param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub unsafe fn pass1_1008_50c2(
    param_1: &mut Struct110,
    param_2: u32,
    param_3: u32,
    param_4: U32Ptr,
    param_5: u32) {
    param_1.field_0x0 = *param_4;
    let i_var1 = param_1;
    i_var1.field_0x2 = (param_4 + 0x2);
    i_var1.field_0x4 = param_3;
    i_var1.field_0x8 = param_2;
    i_var1.field_0xc = param_5;
    i_var1.field_0x10 = 0x0;
    pass1_1008_52fc(param_1);
    return;
}


pub fn pass1_1008_5118(
    ctx: &mut AppContext,
    param_1: u32) {
    let u_var1: &mut Struct18;

    if (param_1 + 0x10) != 0x0 {
        u_var1 = (param_1 + 0x10);
        call_fn_ptr_1000_0dc6(ctx, u_var1, 0x1000);
    }
    return;
}


pub unsafe fn pass1_1008_5134(
    ctx: &mut AppContext,
    param_1: u32) {
    let pu_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;
    let l_var4: i32;
    let i_var5: i16;
    let i_var6: i16;
    let u_var7: u16;
    let i_stack16: i16;
    let l_stack14: i32;
    let u_stack10: u32;

    // u_var7 = (param_1 >> 0x10);
    i_var6 = param_1 as i16;
    l_var4 = ((i_var6 + 0x4) * (i_var6 + 0x8)) as i32;
    l_var4 = mem_op_1000_0a48(ctx, 0x1, l_var4 as u32, ctx._PTR_LOOP_1050_5f2c, 0x1000, 0);
    // u_var3 = (l_var4 >> 0x10);
    (i_var6 + 0x10) = l_var4 as i16;
    (i_var6 + 0x12) = u_var3 as i16;
    if (u_var3 | (i_var6 + 0x10)) == 0x0 {
        return;
    }
    i_var5 = (i_var6 + 0x8);
    i_var2 = (i_var6 + 0xa);
    l_var4 = CONCAT22(i_var2 - (i_var5 == 0x0), (i_var5 + -0x1) as u16) * (i_var6 + 0x4);
    pu_var1 = (i_var6 + 0x10) as u32;
    u_var3 = l_var4 as u16;
    u_stack10 = CONCAT22(((l_var4 >> 0x10) + CARRY2(u_var3, *pu_var1)) * 0x100 + (i_var6 + 0x12), u_var3 + *pu_var1);
    l_stack14 = CONCAT22(i_var2 as u16, i_var5 as u16);
    i_stack16 = (i_var6 + 0x2);
    while l_stack14 != 0x0 {
        i_var2 = i_stack16 + 0x1;
        i_var5 = i_stack16 >> 0xf;
        pass1_1008_4544(((i_var6 + 0xc) as u32));
        pass1_1000_48a8(u_stack10, CONCAT22(i_var5 as u16, i_stack16 as u16), (i_var6 + 0x4));
        i_var5 = (i_var6 + 0x4);
        u_var3 = -i_var5 as u16;
        u_stack10 = CONCAT22((u_stack10 >> 0x10) + (CARRY2(u_stack10 as u16, u_var3) - ((i_var6 + 0x6) + (i_var5 != 0x0))) * 0x100,
                             (u_stack10 + u_var3) as u16);
        i_stack16 = i_var2;
        l_stack14 = l_stack14 + -0x1;
    }
    return;
}


pub fn pass1_1008_5236(param_1: u32) {
    let pu_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;
    let l_var4: i32;
    let i_var5: i16;
    let i_var6: &mut Struct109;
    let u_var6: u16;
    let b_var7: bool;
    let i_stack12: i16;
    let l_stack10: i32;
    let u_stack6: u16;
    let i_stack4: i16;

    // u_var6 = (param_1 >> 0x10);
    i_var6 = param_1;
    i_var5 = i_var6.field_0x8;
    i_var2 = i_var6.field_0xa;
    l_var4 = CONCAT22(i_var2 - (i_var5 == 0x0), (i_var5 + -0x1) as u16) * &i_var6.field_0x4;
    pu_var1 = &i_var6.field_0x10;
    u_var3 = l_var4 as u16;
    u_stack6 = u_var3 + *pu_var1;
    i_stack4 = ((l_var4 >> 0x10) + CARRY2(u_var3, *pu_var1)) * 0x100 + i_var6.field_0x12;
    l_stack10 = CONCAT22(i_var2 as u16, i_var5 as u16);
    i_stack12 = i_var6.field_0x2;
    while l_stack10 != 0x0 {
        i_var2 = i_stack12 + 0x1;
        i_var5 = i_stack12 >> 0xf;
        pass1_1008_4544(i_var6.field_0xc);
        pass1_1000_48a8(CONCAT22(i_var5 as u16, i_stack12 as u16), CONCAT22(i_stack4 as u16, u_stack6),
                        &i_var6.field_0x4);
        i_var5 = &i_var6.field_0x4;
        u_var3 = -i_var5 as u16;
        b_var7 = CARRY2(u_stack6, u_var3);
        u_stack6 += u_var3;
        i_stack4 += (b_var7 - (i_var6.field_0x6 + (i_var5 != 0x0))) * 0x100;
        i_stack12 = i_var2;
        l_stack10 = l_stack10 + -0x1;
    }
    return;
}


pub fn pass1_1008_52fc(param_1: &mut Struct110) {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u16;
    let l_var4: i32;
    let u_var5: u16;
    let i_var6: i16;
    let i_var7: i16;
    let i_var8: &mut Struct111;
    let u_var8: u16;
    let u_var9: u32;
    let u_stack14: u16;
    let i_stack12: i16;

    // u_var8 = (param_1 >> 0x10);
    i_var8 = param_1;
    u_var9 = pass1_1008_4772(i_var8.field_0xc);
    // u_var5 = (u_var9 >> 0x10);
    i_var7 = u_var9 as i16;
    i_var6 = (i_var7 + 0x4);
    u_var3 = (i_var6 - 0x1) as u16;
    i_var6 = (i_var7 + 0x6) - (i_var6 == 0x0);
    l_var4 = ((i_var7 + 0x8) + -0x1) as i32;
    u_var2 = param_1.field_0x0;
    pu_var1 = &i_var8.field_0x4;
    i_var7 = (u_var2 >> 0xf) + i_var8.field_0x6 + CARRY2(u_var2, *pu_var1);
    if (i_var6 <= i_var7) && (i_var6 < i_var7 || (u_var3 < u_var2 + *pu_var1)) {
        i_var8.field_0x4 = u_var3 - u_var2;
        i_var8.field_0x6 = (i_var6 - (u_var2 >> 0xf)) - (u_var3 < u_var2);
    }
    u_var2 = i_var8.field_0x2;
    pu_var1 = &i_var8.field_0x8;
    i_var6 = (u_var2 >> 0xf) + i_var8.field_0xa + CARRY2(u_var2, *pu_var1);
    // i_stack12 = (l_var4 >> 0x10);
    if (i_stack12 <= i_var6) && ((u_stack14 = l_var4 as u16, i_stack12 < i_var6 || (u_stack14 < u_var2 + *pu_var1))) {
        i_var8.field_0x8 = u_stack14 - u_var2;
        i_var8.field_0xa = (i_stack12 - (u_var2 >> 0xf)) - (u_stack14 < u_var2);
    }
    return;
}


pub fn pass1_1008_5394(param_1: &mut u32) -> u32

{
    *param_1 = 0x0;
    return *param_1;
}


pub fn pass1_1008_53aa() {
    return;
}


pub fn pass1_1008_570e(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8) -> &mut Struct18 {
    param_1.field_0x0 = 0x389a;
    param_1.field_0x2 = 0x1008;
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_5784(param_1: U32Ptr, param_2: u32) {
    *param_1 = param_2;
    (param_1 + 0x4) = 0x0;
    return;
}


pub fn pass1_1008_57a4(param_1: U32Ptr, param_2: u32) {
    *param_1 = param_2;
    (param_1 + 0x4) = 0x0;
    return;
}


pub fn pass1_1008_57c4(param_1: U32Ptr) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    *param_1 = 0x5bc4;
    (param_1 + 0x2) = 0x1008;
    pass1_1008_5830(param_1 & 0xffff | uVar1 << 0x10);
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    return;
}


pub fn pass1_1008_57f0(param_1: u32, param_2: i16, param_3: u16) -> u32

{
    let bVar1: bool;
    let lVar2: i32;
    let iStack12: i16;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_3, local_a), param_1);
    iStack12 = 0x0;
    loop {
        lVar2 = pass1_1008_5b12(local_a, param_3);
        if (lVar2 == 0x0) {
            return 0x0;
        }
        bVar1 = iStack12 != param_2;
        iStack12 = iStack12 + 0x1;
        if !bVar1 { break; }
    }
    return lVar2 as u32;
}


pub fn pass1_1008_5830(param_1: u32) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let uVar4: u32;
    let puVar5: u32;
    let iVar6: i16;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u16;

    loop {
        // uVar8 = (param_1 >> 0x10);
        iVar6 = param_1 as i16;
        if ((iVar6 + 0x4) == 0x0) { break; }
        if ((iVar6 + 0xa) != 0x0) {
            uVar4 = (iVar6 + 0x4) as u32;
            // uVar9 = (uVar4 >> 0x10);
            iVar7 = uVar4 as i16;
            puVar1 = (iVar7 + 0x8) as u32;
            uVar2 = (iVar7 + 0xa) as u16;
            if ((uVar2 | puVar1) != 0x0) {
                ppcVar3 = *puVar1;
                (**ppcVar3)();
            }
        }
        puVar5 = (iVar6 + 0x4) as u32;
        (iVar6 + 0x4) = (puVar5 + 0x4) as i16;
        if (puVar5 != 0x0) {
            ppcVar3 = *puVar5;
            (**ppcVar3)();
        }
    }
    (iVar6 + 0x8) = 0x0;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_58a6(param_1: u32, param_2: u32) {
    let piVar1: U32Ptr;
    let uVar2: u16;
    let uVar3: u16;
    let iVar4: i16;
    let uVar5: u16;
    let uVar6: u16;
    let paStack6: &mut Struct99;

    paStack6 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_029c);
    // uVar3 = (paStack6 >> 0x10);
    uVar2 = paStack6;
    if ((uVar3 | uVar2) == 0x0) {
        paStack6 = 0x0;
    } else {
        paStack6.field_0x0 = 0x389a;
        (uVar2 + 0x2) = 0x1008;
        (uVar2 + 0x4) = 0x0;
        (uVar2 + 0x8) = 0x0;
        paStack6.field_0x0 = 0x5bc0;
        (uVar2 + 0x2) = 0x1008;
    }
    if (paStack6 == 0x0) {
        return;
    }
    // uVar5 = (paStack6 >> 0x10);
    (paStack6 + 0x8) = param_2;
    // uVar6 = (param_1 >> 0x10);
    iVar4 = param_1 as i16;
    (paStack6 + 0x4) = (iVar4 + 0x4);
    (iVar4 + 0x4) = paStack6;
    piVar1 = (iVar4 + 0x8) as u32;
    *piVar1 = *piVar1 + 0x1;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_593c(param_1: U32Ptr, param_2: u32) {
    let piVar1: U32Ptr;
    let ppcVar2: u32;
    let uVar3: u16;
    let uVar4: u16;
    let iVar5: i16;
    let uVar6: u16;
    let uVar7: u16;
    let paStack6: &mut Struct99;

    // uVar6 = (param_1 >> 0x10);
    iVar5 = param_1 as i16;
    if ((iVar5 + 0x8) == 0x0) {
        ppcVar2 = (*param_1 + 0x4);
        (**ppcVar2)();
        return;
    }
    paStack6 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_029c);
    // uVar4 = (paStack6 >> 0x10);
    uVar3 = paStack6;
    if ((uVar4 | uVar3) == 0x0) {
        paStack6 = 0x0;
    } else {
        paStack6.field_0x0 = 0x389a;
        (uVar3 + 0x2) = 0x1008;
        (uVar3 + 0x4) = 0x0;
        (uVar3 + 0x8) = 0x0;
        paStack6.field_0x0 = 0x5bc0;
        (uVar3 + 0x2) = 0x1008;
    }
    if (paStack6 == 0x0) {
        return;
    }
    (paStack6 + 0x8) = param_2;
    loop {
        param_1 = (param_1 + 0x4);
        // uVar7 = (param_1 >> 0x10);
        if ((param_1 + 0x4) != 0x0) == false { break; }
    }
    (param_1 + 0x4) = paStack6;
    piVar1 = (iVar5 + 0x8) as u32;
    *piVar1 = *piVar1 + 0x1;
    return;
}


pub fn pass1_1008_59f4(param_1: u32, param_2: i32) {
    let piVar1: U32Ptr;
    let puVar2: u32;
    let uVar3: u16;
    let puVar4: u32;
    let ppcVar5: u32;
    let puVar6: u32;
    let uVar7: u16;
    let iVar8: i16;
    let uVar9: u16;
    let uVar10: u16;
    let uVar11: u16;
    let uVar12: u16;
    let uStack10: u16;
    let puStack6: u32;

    puStack6 = 0x0;
    // uVar9 = (param_1 >> 0x10);
    puVar6 = puStack6;
    puVar4 = param_1;
    loop {
        puStack6 = puVar6;
        // uVar10 = (puVar4 >> 0x10);
        iVar8 = puVar4 as i16;
        puVar4 = (iVar8 + 0x4) as u32;
        uStack10 = puVar4 as u16;
        // uVar11 = (puVar4 >> 0x10);
        if (((iVar8 + 0x6) | uStack10) == 0x0) { break; }
        puVar6 = puVar4;
    }
    if (puVar4 != 0x0) {
        if (puStack6 == 0x0) {
            uVar10 = (uStack10 + 0x4);
            uVar7 = (uStack10 + 0x6);
            puStack6 = param_1;
        } else {
            uVar10 = (uStack10 + 0x4);
            uVar7 = (uStack10 + 0x6);
        }
        // uVar12 = (puStack6 >> 0x10);
        (puStack6 + 0x4) = uVar10 as u32;
        (puStack6 + 0x6) = uVar7 as u32;
        if ((param_1 + 0xa) != 0x0) {
            puVar2 = (uStack10 + 0x8) as u32;
            uVar3 = (uStack10 + 0xa);
            if ((uVar3 | puVar2) != 0x0) {
                ppcVar5 = *puVar2;
                (**ppcVar5)();
            }
        }
        if (puVar4 != 0x0) {
            ppcVar5 = *puVar4;
            (**ppcVar5)();
        }
        piVar1 = (param_1 + 0x8);
        *piVar1 = *piVar1 + -0x1;
        return;
    }
    return;
}


pub fn pass1_1008_5ab8(param_1: u32) {
    let piVar1: U32Ptr;
    let ppcVar2: u32;
    let puVar3: u32;
    let iVar4: i16;
    let uVar5: u16;
    let uVar6: u16;

    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1 as i16;
    if ((iVar4 + 0x4) == 0x0) {
        return;
    }
    puVar3 = (iVar4 + 0x4) as u32;
    // uVar6 = (puVar3 >> 0x10);
    (iVar4 + 0x4) = (puVar3 + 0x4) as i16;
    if ((uVar6 | puVar3) != 0x0) {
        ppcVar2 = *puVar3;
        (**ppcVar2)();
    }
    piVar1 = (iVar4 + 0x8) as u32;
    *piVar1 = *piVar1 + -0x1;
    return;
}


pub fn pass1_1008_5b12(param_1: &i32) {
    let u_var1: u32;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;

    if (*param_1 != 0x0) && ((*param_1 + 0x8) != 0x0) {
        // u_var4 = (param_1 >> 0x10);
        i_var2 = *param_1 as i16;
        if (i_var2 + 0x4) == 0x0 {
            u_var5 = (*param_1 >> 0x10) as u16;
            i_var3 = *param_1 as i16;
        } else {
            u_var1 = (i_var2 + 0x4) as u32;
            // u_var5 = (u_var1 >> 0x10);
            i_var3 = u_var1 as i16;
        }
        (i_var2 + 0x4) = (i_var3 + 0x4);
        if (i_var2 + 0x4) != 0x0 {
            return;
        }
    }
    return;
}


pub fn pass1_1008_5b6e(param_1: U32Ptr, param_2: u8) -> u16

{
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    *param_1 = 0x389a;
    (param_1)[0x1] = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        pass1_1000_093a(param_1, uVar1 as u32, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_5b9a(param_1: U32Ptr, param_2: u8) -> u16

{
    pass1_1008_57c4(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_5bdc(param_1: &mut Struct79, param_2: i16, param_3: u16) {
    let puVar1: &mut Struct651;
    let uVar1: u16;
    let paVar2: &mut Struct79;
    let puVar3: U32Ptr;

    paVar2 = struct_op_1010_1d48(param_1, 0x44);
    // uVar1 = (param_1 >> 0x10);
    puVar1 = param_1;
    puVar1.field_0xa = 0x0;
    &puVar1.field_0xc = 0x0;
    puVar1.field_0x10 = 0x0;
    puVar1.field_0x12 = 0x0;
    param_1.field_0x0 = 0x5fc8;
    puVar1.field_0x2 = 0x1008;
    ctx._PTR_LOOP_1050_02a0 = param_1;
    puVar3 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_3,
                             (paVar2 >> 0x10), param_2);
    puVar1.field_0xc = puVar3;
    puVar1.field_0xe = (puVar3 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_5c34(param_1: U32Ptr) {
    let unaff_SS: u16;

    *param_1 = 0x5fc8;
    (param_1 + 0x2) = 0x1008;
    ctx._PTR_LOOP_1050_02a0 = 0x0;
    pass1_1010_1d80(param_1, unaff_SS);
    return;
}


pub fn pass1_1008_5fa2(param_1: u32, param_2: u8) -> u32

{
    pass1_1008_5c34(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_5fd8(param_1: u16, param_2: U32Ptr) -> U32Ptr

{
    let piVar1: U32Ptr;
    let mut pcVar2: String;
    let puStack10: U32Ptr;
    let puStack8: U32Ptr;
    let iStack6: i16;

    puStack10 = &stack0x0006;
    _iStack6 = CONCAT22(param_1, puStack10 as u16);
    mem_op_1000_179c(0x1000, param_2, 0x1000);
    puStack8 = param_2;
    pcVar2 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
    unk_str_op_1000_3d3e(CONCAT22(puStack8 as u16, puStack10 as u16), pcVar2);
    loop {
        piVar1 = _iStack6;
        _iStack6 = (_iStack6 & 0xffff0000 | (iStack6 + 0x2));
        if (*piVar1 == 0x0) { break; }
        pcVar2 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc,
                                       (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
        pass1_1000_3cea(CONCAT22(puStack8 as u16, puStack10 as u16), pcVar2);
    }
    return puStack10;
}


pub fn pass1_1008_612e(param_1: i16, param_2: i16, param_3: u16) {
    let uVar1: u16;
    let uVar2: u16;
    let lVar3: i32;
    let iVar4: i16;
    let iStack18: i16;
    let iStack16: i16;

    uVar1 = pass1_1000_4d24();
    uVar2 = ((param_2 - param_1) + 0x1) as u16;
    if ((uVar2 >> 0xf | uVar2) == 0x0) {
        return;
    }
    iStack16 = 0x1;
    iStack18 = param_1;
    loop {
        if (param_2 < iStack18) {
            return;
        }
        lVar3 = (iStack16 * (0x7fff / uVar2)) as i32;
        // iVar4 = (lVar3 >> 0x10);
        if (uVar1 >> 0xf <= iVar4 as u16) {
            if (uVar1 >> 0xf < iVar4 as u16) {
                return;
            }
            if (uVar1 <= lVar3 as u16) {
                return;
            }
        }
        iStack18 += 0x1;
        iStack16 += 0x1;
    }
}


pub fn pass1_1008_6330(param_1: U32Ptr, param_2: u8) {
    let uVar1: &mut Struct456;
    let uVar2: u16;

    uVar1 = param_1;
    uVar1 = uVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    // uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    uVar1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    uVar1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}


pub fn pass1_1008_64a2(param_1: &mut Struct18) {
    let uVar1: u16;
    let ppcVar2: u32;

    uVar1 = (param_1 + 0x2);
    if ((uVar1 | *param_1) != 0x0) {
        ppcVar2 = *param_1;
        (**ppcVar2)();
    }
    return;
}


pub fn pass1_1008_64c8(param_1: U32Ptr, param_2: u32, param_3: i16, param_4: u16, param_5: U32Ptr) {
    let iVar1: i16;
    let iVar2: i16;
    let extraout_DX: u16;
    let uVar3: u16;
    let iVar4: i16;
    let iVar5: i16;
    let iStack8: i16;
    let uStack6: u32;

    if (*param_1 == 0x0) {
        return;
    }
    mem_op_1000_179c(0x1e, param_5, 0x1000);
    if ((param_5 | param_4) == 0x0) {
        param_4 = 0x0;
        uVar3 = 0x0;
    } else {
        struct_op_1008_6604(CONCAT22(param_5 as u16, param_4), param_2 as i16,
                            ((param_2 >> 0x10) as i16));
        uVar3 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar3, param_4);
    iStack8 = 0x0;
    while (param_2 = param_2 & 0xffff0000 | (param_2 - 0x1), param_2 != 0x0
    ) {
        iVar1 = param_3 + 0x1;
        iVar4 = param_3 >> 0xf;
        pass1_1008_4544(*param_1);
        iVar2 = iStack8 + 0x1;
        iVar5 = iStack8 >> 0xf;
        pass1_1008_4544(uStack6);
        pass1_1000_48a8(CONCAT22(iVar5 as u16, iStack8 as u16), CONCAT22(iVar4 as u16, param_3 as u16), param_2._2_2_);
        param_3 = iVar1;
        iStack8 = iVar2;
    }
    return;
}


pub fn pass1_1008_6562(param_1: U32Ptr, param_2: u32, param_3: i16, param_4: u16, param_5: U32Ptr) {
    let iVar1: i16;
    let iVar2: i16;
    let uVar3: u16;
    let iVar4: i16;
    let iVar5: i16;
    let iStack8: i16;
    let uStack6: u32;

    if (*param_1 == 0x0) {
        return;
    }
    mem_op_1000_179c(0x1e, param_5, 0x1000);
    uVar3 = (param_5 | param_4) as u16;
    if (uVar3 == 0x0) {
        param_4 = 0x0;
        uVar3 = 0x0;
    } else {
        pass1_1008_405c(ctx, CONCAT22(param_5 as u16, param_4), (param_1 + 0x4), param_2 as i16, ((param_2 >> 0x10) as i16));
    }
    uStack6 = CONCAT22(uVar3, param_4);
    iStack8 = 0x0;
    while (param_2 = param_2 & 0xffff0000 | (param_2 - 0x1), param_2 != 0x0
    ) {
        iVar1 = param_3 + 0x1;
        iVar4 = param_3 >> 0xf;
        pass1_1008_4544(*param_1);
        iVar2 = iStack8 + 0x1;
        iVar5 = iStack8 >> 0xf;
        pass1_1008_4544(uStack6);
        pass1_1000_48a8(CONCAT22(iVar5 as u16, iStack8 as u16), CONCAT22(iVar4 as u16, param_3 as u16), param_2._2_2_);
        param_3 = iVar1;
        iStack8 = iVar2;
    }
    return;
}


pub fn pass1_1008_6732(param_1: U32Ptr) {
    let lVar1: i32;
    let iVar2: &mut Struct457;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    *param_1 = 0x685a;
    iVar2.field_0x2 = 0x1008;
    if (iVar2.field_0x10 != 0x0) {
        lVar1 = iVar2.field_0x10;
        call_fn_ptr_1000_0dc6(lVar1, (lVar1 >> 0x10), 0x1000);
    }
    iVar2.field_0x10 = 0x0;
    pass1_1008_41bc(param_1);
    return;
}


pub fn pass1_1008_6834(param_1: u32, param_2: u8) -> u32

{
    pass1_1008_6732(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_68c6(param_1: u16, param_2: u16, param_3: i16, param_4: u16) -> bool

{
    let BVar1: bool;
    let in_DX: u16;
    let unaff_SS: u16;

    BVar1 = show_win_1008_96ae(CONCAT22(param_2, param_1), param_3, param_4);
    pass1_1008_6a04(CONCAT22(param_2, param_1), in_DX, unaff_SS);
    return BVar1;
}


pub fn pass1_1008_68ea(param_1: i16, param_2: u16, param_3: U32Ptr, param_4: u16, param_5: u16,
                       param_6: i16)

{
    let ppcVar1: u32;

    if (param_6 == 0x0) {
        if ((param_1 + 0xce) != CONCAT22(param_4, param_3 as u16)) {
            if ((param_1 + 0xce) != 0x0) {
                ppcVar1 = ((param_1 + 0xce) + 0x10) as u32;
                (**ppcVar1)();
            }
            (param_1 + 0xce) = CONCAT22(param_4, param_3 as u16);
            ppcVar1 = (*param_3 + 0x10);
            (**ppcVar1)();
            ppcVar1 = ((param_1 + 0xce) + 0xc) as u32;
            (**ppcVar1)();
            return;
        }
    } else {
        fn_ptr_1000::call_fn_ptr_1008_3e0e(CONCAT13((param_2 >> 0x8), CONCAT12(param_2 as u8, param_1 as u16)));
    }
    return;
}


pub fn pass1_1008_6978(param_1: u32, param_2: i16, param_3: u32, param_4: u16, param_5: U32Ptr) {
    let ppcVar1: u32;
    let puStack10: U32Ptr;
    let puStack6: U32Ptr;

    mem_op_1000_179c(0xa, param_5, 0x1000);
    puStack10 = CONCAT22(param_5 as u16, param_4);
    if ((param_5 | param_4) == 0x0) {
        puStack6 = 0x0;
    } else {
        if (param_2 == 0x0) {
            param_2 = (param_1 + 0xcc) as i16;
        }
        *puStack10 = 0x389a;
        (param_4 + 0x2) = 0x1008;
        (param_4 + 0x4) = param_3 as u16;
        (param_4 + 0x8) = param_2 as u16;
        *puStack10 = 0x6c8c;
        (param_4 + 0x2) = 0x1008;
        puStack6 = puStack10;
    }
    ppcVar1 = ((param_1 + 0xd2) + 0x4);
    (**ppcVar1)(0x1000, (param_1 + 0xd2), param_1._2_2_, puStack6);
    return;
}


pub fn pass1_1008_6a04(param_1: u32, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let puVar2: U32Ptr;
    let extraout_DX: u16;
    let local_a: [u8; 8];

    pass1_1008_57a4(CONCAT22(param_3, local_a),
                    param_1 & 0xffff0000 | (param_1 + 0xd2));
    loop {
        puVar2 = local_a;
        pass1_1008_5b12(puVar2, param_3);
        if ((extraout_DX | puVar2) == 0x0) { break; }
        ppcVar1 = (*(puVar2 + 0x4) + 0xc);
        (**ppcVar1)();
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1008_6a4a(param_1: u32, param_2: i16, param_3: u16, param_4: i16, param_5: u16) {
    let ppcVar1: u32;
    let iVar2: i16;
    let puVar3: U32Ptr;
    let extraout_DX: u16;
    let extraout_DX_00: u16;
    let local_e: [u8; 4];
    let uStack10: u32;
    let uStack6: u32;

    if (param_4 == 0x2) {
        iVar2 = param_1 as i16;
        pass1_1008_57a4(CONCAT22(param_5, local_e),
                        param_1 & 0xffff0000 | (iVar2 + 0xd2));
        loop {
            puVar3 = local_e;
            pass1_1008_5b12(puVar3, param_5);
            uStack6 = CONCAT22(extraout_DX, puVar3 as u16);
            if ((extraout_DX | puVar3) == 0x0) { break; }
            if ((puVar3 + 0x8) != param_2 as u32) { break; }
        }
        if (uStack6 != 0x0) {
            ppcVar1 = ((iVar2 + 0xd2) + 0xc) as u32;
            (**ppcVar1)();
            uStack10 = 0x0;
            uStack6._0_2_ = local_e;
            pass1_1008_5b12();
            if ((extraout_DX_00 | uStack6) != 0x0) {
                ppcVar1 = (*(uStack6 + 0x4) + 0x10);
                uStack6._2_2_ = extraout_DX_00;
                (**ppcVar1)();
                (iVar2 + 0xce) = (uStack6 + 0x4) as i16;
                return;
            }
            (iVar2 + 0xce) = 0x0;
        }
    }
    return;
}


pub fn pass1_1008_6b02(param_1: u32) {
    let ppcVar1: u32;
    let iVar2: i16;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    iVar2 = param_1 as i16;
    if (((iVar2 + 0xd0) | (iVar2 + 0xce)) != 0x0) {
        ppcVar1 = ((iVar2 + 0xce) + 0x6c) as u32;
        (**ppcVar1)();
    }
    return;
}


pub fn pass1_1008_6b2e(param_1: u32) {
    let ppcVar1: u32;
    let iVar2: i16;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    iVar2 = param_1 as i16;
    if (((iVar2 + 0xd0) | (iVar2 + 0xce)) != 0x0) {
        ppcVar1 = ((iVar2 + 0xce) + 0x68) as u32;
        (**ppcVar1)();
    }
    return;
}


pub fn pass1_1008_6b5a(param_1: U32Ptr, param_2: u8) -> u16

{
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let uVar4: &mut Struct458;
    let uVar5: u16;

    // uVar5 = (param_1 >> 0x10);
    uVar4 = param_1;
    *param_1 = 0x6c8c;
    uVar4.field_0x2 = 0x1008;
    puVar1 = uVar4.field_0x4;
    uVar2 = uVar4.field_0x6;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    *param_1 = 0x389a;
    uVar4.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_6bb4(param_1: U32Ptr, param_2: u8) {
    let uVar1: &mut Struct459;
    let uVar2: u16;

    uVar1 = param_1;
    uVar1 = uVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    // uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    uVar1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    uVar1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}


pub fn pass1_1008_6c90(param_1: U32Ptr) {
    struct_1008::clear_struct_1008_3e38(param_1);
    struct_1008::clear_struct_1008_3e38((param_1 & 0xffff0000 | (param_1 + 0x6)));
    return;
}


pub fn pass1_1008_6cb4(param_1: U32Ptr, param_2: U32Ptr, param_3: u16, param_4: U32Ptr, param_5: u16,
) -> * mut 32

{
let iVar1: & mut Struct362; let uVar1: u16;

// uVar1 = (param_1 >> 0x10);
iVar1 = param_1; * param_1 = *param_4; iVar1.field_0x4 = (param_4 + 0x1); iVar1.field_0x6 = * param_2; iVar1.field_0xa = (param_2 + 0x1); return param_1;
}



pub fn pass1_1008_6cec(param_1: U32Ptr, param_2: u16, param_3: u32, param_4: u16, param_5: u32) {
    pass1_1008_3e76(param_1, param_4, param_5 as i16, ((param_5 >> 0x10) as u16));
    pass1_1008_3e76((param_1 & 0xffff0000 | (param_1 + 0x6)),
                    param_2, param_3 as i16, ((param_3 >> 0x10) as u16));
    return;
}


pub fn pass1_1008_6d18(param_1: U32Ptr, param_2: U32Ptr, param_3: U32Ptr) {
    pass1_1008_3f62(param_1, param_3);
    pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x6)),
                    param_2);
    return;
}


pub fn pass1_1008_6d3e(param_1: U32Ptr, param_2: U32Ptr, param_3: U32Ptr) {
    pass1_1008_3f62(param_3, param_1);
    pass1_1008_3f62(param_2,
                    (param_1 & 0xffff0000 | (param_1 + 0x6)));
    return;
}


pub fn pass1_1008_6d64(param_1: U32Ptr, param_2: U32Ptr) {
    pass1_1008_3f62(param_2, param_1);
    pass1_1008_3ee2(param_2,
                    (param_1 & 0xffff0000 | (param_1 + 0x6)));
    return;
}


pub fn pass1_1008_72a8(param_1: U32Ptr, param_2: u16) -> u16

{
    *param_1 = param_2;
    return param_1 as u16;
}

pub fn pass1_1008_7006(param_1: u32, param_2: u16, param_3: u32, param_4: u16, param_5: u16, param_6: u16) -> i16 {
    unimplemented!()
}

pub fn pass1_1008_766e(param_1: u32, param_2: U32Ptr, param_3: u16, param_4: u16, param_5: U32Ptr) {
    let puVar1: u32;
    let puVar2: U32Ptr;
    let local_6: u32;

    *param_2 = 0x0;
    local_6 = 0x0;
    puVar1 = &local_6;
    file_1008_76e4(param_1, CONCAT22(param_3, puVar1 as u16), param_4, param_3, param_5 as u16);
    if (puVar1 != 0x0) {
        if (local_6 != 0x0) {
            mem_op_1000_179c(0xc, param_5, 0x1000);
            puVar2 = (param_5 | puVar1);
            if (puVar2 == 0x0) {
                puVar1 = 0x0;
                puVar2 = 0x0;
            } else {
                pass1_1010_8ef2(CONCAT22(param_5 as u16, puVar1 as u16), puVar2, param_3);
            }
            param_2 = puVar1;
            (param_2 + 0x2) = puVar2;
            fn_ptr_1010_905e(*param_2, local_6);
        }
        return;
    }
    return;
}


pub fn pass1_1008_7898(param_1: u32, param_2: U32Ptr, param_3: u16, param_4: u16, param_5: HFILE16, param_6: u16)

{
    let ppcVar1: u32;
    let BVar2: bool;
    let extraout_DX: u16;
    let uVar3: u16;
    let uVar4: u16;
    let uVar5: u16;
    let local_26: u16;
    let local_24: [u32; 0x3];
    let local_18: u32;
    let local_14: [u16; 0x5];
    let uStack10: u32;
    let uStack6: u32;

    if (param_2 == 0x0) {
        param_3 = 0x0;
        uVar3 = 0x0;
    } else {
        ppcVar1 = (*param_2 + 0x10);
        (**ppcVar1)();
        uVar3 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar3, param_3);
    local_18 = CONCAT22(uVar3, param_3);
    uVar4 = param_1 as u16;
    // uVar5 = (param_1 >> 0x10);
    BVar2 = write_to_file_1008_7e1c(uVar4, uVar5, &local_18, param_6, 0x4, param_5);
    if (BVar2 != 0x0) {
        uStack10 = 0x0;
        loop {
            if (uStack6 <= uStack10) {
                return;
            }
            pass1_1020_c4a8(param_2, CONCAT22(param_6, local_14),
                            CONCAT22(param_6, &local_18), uStack10 as i16, param_4, param_6);
            local_24[0] = local_18;
            BVar2 = write_to_file_1008_7e1c(uVar4, uVar5, local_24, param_6, 0x4, 0x1020);
            if (BVar2 == 0x0) { break; }
            local_26 = local_14[0];
            BVar2 = write_to_file_1008_7e1c(uVar4, uVar5, &local_26, param_6, 0x2, 0x1020);
            if (BVar2 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d0;
                return;
            }
            uStack10 += 0x1;
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
    return;
}


pub fn pass1_1008_79f0(param_1: u32, param_2: i32, param_3: HFILE16, param_4: u16) {
    let uVar1: u16;
    let uVar2: u16;
    let uStack4: u16;

    if (param_2 == 0x0) {
        uVar1 = 0x0;
        uStack4 = 0x0;
    } else {
        // uVar2 = (param_2 >> 0x10);
        uVar1 = (param_2 + 0x4) as u16;
        uStack4 = (param_2 + 0x6) as u16;
    }
    write_to_file_1008_7954(param_1, CONCAT22(uStack4, uVar1), uVar1, param_3, param_4);
    return;
}


pub fn pass1_1008_7ad4(param_1: u32, param_2: &i32, param_3: u16, param_4: HFILE16, param_5: u16) -> u16

{
    let BVar1: bool;
    let uVar2: u16;
    let uVar3: u16;
    let local_14: [u16; 0x2];
    let local_10: [u32; 0x2];
    let uStack6: u16;
    let local_4: u16;

    uVar2 = param_1 as u16;
    // uVar3 = (param_1 >> 0x10);
    BVar1 = read_file_1008_7dee(uVar2, uVar3, &local_4, 0x0, param_5, 0x2, param_4);
    if (BVar1 != 0x0) {
        uStack6 = 0x0;
        loop {
            if (local_4 <= uStack6) {
                return 0x1;
            }
            BVar1 = read_file_1008_7dee(uVar2, uVar3, local_14, 0x0, param_5, 0x2, param_4);
            if ((BVar1 == 0x0) || (BVar1 = read_file_1008_7dee(uVar2, uVar3, local_10, 0x0, param_5, 0x4, param_4,
            ), BVar1 == 0x0)) { break; }
            param_4 = 0x1020;
            pass1_1020_bb8a(param_2, local_10[0] as u16,
                            CONCAT22(local_14[0], ((local_10[0] >> 0x10) as u16)), param_3,
                            param_5);
            uStack6 += 0x1;
        }
    }
    return 0x0;
}


pub fn pass1_1008_7c2a(param_1: u32, param_2: &mut String, param_3: HFILE16) -> bool

{
    let uVar1: u16;
    let BVar2: bool;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    if (param_2 != 0x0) {
        uVar1 = str_op_1000_3da4(param_2);
        BVar2 = write_to_file_1008_7e1c(param_1, uVar3, param_2,
                                        (param_2 >> 0x10), (uVar1 + 0x1),
                                        0x1000);
        return BVar2;
    }
    write_to_file_1008_7e1c(param_1, uVar3, (s_playerName_1050_148e + 0xc),
                            ctx.data_seg, 0x1, param_3);
    return 0x1;
}


pub fn pass1_1008_7e4a(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: U32Ptr,
    param_3: u8,
    param_4: &mut String,
    param_5: U32Ptr,
    param_6: u16,
    stack0xfffe: i16,
) -> bool

{
    let mut u_var1 = 0u16;

    sys_1000_3f9c(
        ctx,
        param_5,
        param_2,
        0x347,
        ctx.data_seg,
        &mut ctx._PTR_s_dcbSC_1050_0336_1050_033c,
        stack0xfffe,
        param_1,
        0x1000,
        param_2,
        param_3,
    );
    u_var1 = str_op_1000_3da4(read_string_from_addr(CONCAT22(param_2 as u16, param_5 as u16)));
    u_var1 = pass1_1000_3de8(
        param_4,
        read_string_from_addr(CONCAT22(param_2 as u16, param_5 as u16)),
        &mut u_var1,
        param_5 as u16,
        param_6,
    );
    if u_var1 == 0x0 {
        return true;
    }
    return false;
}


pub fn pass1_1008_7e98(param_1: U32Ptr, param_2: u8) -> u16

{
    let uVar1: &mut Struct460;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    uVar1 = param_1;
    *param_1 = 0x380a;
    uVar1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    uVar1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_7ffa(param_1: U32Ptr, param_2: u8) {
    let uVar1: &mut Struct461;
    let uVar2: u16;

    uVar1 = param_1;
    uVar1 = uVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    // uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    uVar1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    uVar1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}


pub fn pass1_1008_80d2(param_1: U32Ptr) -> u32

{
    *param_1 = 0x0;
    (param_1 + 0x4) = 0x0;
    return param_1;
}


pub fn pass1_1008_8168(param_1: U32Ptr) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    *param_1 = 0x87c8;
    (param_1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    return;
}


pub fn pass1_1008_818c(ctx: &mut AppContext, param_1: &mut Struct23, param_2: HINSTANCE16, param_3: &WNDCLASS16, stack0xfffe: u16) {
    let BVar1: bool;
    let AVar2: ATOM;
    let name: String;
    let uStack26: u16;
    let uStack24: u16;
    let uStack22: u32;
    let puStack18: U32Ptr;
    let uStack16: u16;
    let uStack14: u16;
    let uStack12: u16;
    let uStack10: u32;
    let iStack6: i16;
    let uStack4: u16;

    iStack6 = param_1 + 0x4;
    BVar1 = GetClassInfo16(param_2, &name, param_3);
    if BVar1 == false {
        name = (param_1 + 0x54);
        uStack26 = 0x84f2;
        uStack24 = 0x1008;
        uStack22 = 0x40000;
        puStack18 = ctx.PTR_LOOP_1050_038c;
        uStack16 = 0x0;
        uStack14 = (param_1 + 0x58);
        uStack12 = (param_1 + 0x56);
        uStack10 = 0x0;
        uStack4 = param_1._2_2_;
        AVar2 = RegisterClass16(&struct_from_addr::<WNDCLASS16>(ctx.s_tile2_bmp_1050_1538));
        if AVar2 == 0x0 {
            fn_ptr_op_1000_24cd(0x0, stack0xfffe as i16);
        }
    }
    return;
}


pub fn pass1_1008_87a2(param_1: u32, param_2: u8) -> u32

{
    pass1_1008_8168(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1008_87cc(param_1: &mut Struct86, param_2: i16, param_3: i16, param_4: u16, param_5: u32,
                       param_6: u32, param_7: u16)

{
    let lVar1: i32;
    let uVar2: u16;
    let BVar3: bool;
    let piVar4: U32Ptr;
    let puVar5: U32Ptr;
    let iVar5: &mut Struct86;
    let iVar6: i16;
    let unaff_DI: i16;
    let uVar7: u16;
    let uVar8: u16;
    let puVar9: U32Ptr;
    let piStack48: U32Ptr;
    let local_24: u32;
    let uStack32: u16;
    let uStack30: u32;
    let paStack26: &mut Struct18;
    let uStack18: u32;
    let iStack14: i16;
    let iStack12: i16;
    let uStack10: i16;
    let iStack8: i16;
    let uStack6: u32;

    // uVar7 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.field_0x0 = 0x389a;
    iVar5.field_0x2 = 0x1008;
    iVar5.field_0x4 = param_5;
    &iVar5.field_0x8 = 0x0;
    iVar5.field_0xc = param_3;
    iVar5.field_0xe = param_2;
    iVar5.field_0x10 = 0x0;
    iVar5.field_0x12 = 0x0;
    struct_1008::clear_struct_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar5.field_0x1c)));
    struct_1008::clear_struct_1008_3e38(
        (param_1 & 0xffff0000 | &iVar5.field_0x22));
    puVar9 = struct_1008::clear_struct_1008_3e38(
        (param_1 & 0xffff0000 | &iVar5.field_0x28)
    );
    iVar5.field_0x2e = param_4;
    iVar5.field_0x30 = 0xffff;
    iVar5.field_0x3a = 0x0;
    iVar5.field_0x3e = 0x1;
    iVar5.field_0x40 = 0x1;
    iVar5.field_0x42 = param_6;
    param_1.field_0x0 = 0x8e9a;
    iVar5.field_0x2 = 0x1008;
    if (ctx.PTR__LOOP_1050_0382 == 0x0) {
        ctx._PTR_LOOP_1050_0382 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2e, param_7, (puVar9 >> 0x10), unaff_DI);
    }
    uStack6 = pass1_1008_4772(iVar5.field_0x4);
    iVar5.field_0x12 = 0x2f - (uStack6 + 0x8);
    uVar8 = (ctx.PTR__LOOP_1050_0382 >> 0x10);
    iVar6 = ctx._PTR_LOOP_1050_0382;
    iStack8 = (iVar6 + 0xa);
    iStack10 = (iVar6 + 0xc);
    iStack12 = (iVar6 + 0xe);
    iStack14 = (iVar6 + 0x10);
    iVar6 = iVar5.field_0xc;
    lVar1 = (iVar6 + iVar5.field_0xe) * iStack14;
    // puVar5 = (lVar1 >> 0x10);
    pass1_1008_3e76((param_1 & 0xffff0000 | ZEXT24(&iVar5.field_0x1c)), 0x0, lVar1 + iVar5.field_0x12 + iStack10,
                    (iVar6 - iVar5.field_0xe) * iStack12 + iVar5.field_0x10 + iStack8);
    iVar5.field_0x14 = iVar5.field_0x1c + 0x20;
    iVar5.field_0x16 = (uStack6 + 0x8) + iVar5.field_0x1e + -0x25;
    iVar5.field_0x18 = iVar5.field_0x14 + 0x32;
    uVar2 = iVar5.field_0x16 + 0x19;
    iVar5.field_0x1a = uVar2;
    mem_op_1000_179c(0x6, puVar5, 0x1000);
    paStack26 = CONCAT22(puVar5 as u16, uVar2);
    uStack18._2_2_ = puVar5 | uVar2;
    if (uStack18._2_2_ == 0x0) {
        &iVar5.field_0x8 = 0x0;
    } else {
        puVar9 = pass1_1008_ada2(CONCAT22(puVar5 as u16, uVar2), iVar5.field_0x2e);
        uStack18._2_2_ = (puVar9 >> 0x10);
        iVar5.field_0x8 = puVar9;
        iVar5.field_0xa = uStack18._2_2_;
    }
    BVar3 = pass1_1008_aed8(&iVar5.field_0x8);
    if (BVar3 == 0x0) {
        paStack26 = &iVar5.field_0x8;
        uStack18 = paStack26;
        fn_ptr_1000_17ce(ctx, paStack26, 0x1000);
        &iVar5.field_0x8 = 0x0;
    } else {
        piVar4 = &iVar5.field_0x8;
        pass1_1018_20ee(ctx.PTR__LOOP_1050_0382, piVar4);
        uStack18._0_2_ = SUB42(piVar4 as u16, 0x0);
        pass1_1008_add2(&iVar5.field_0x8);
        uStack30 = pass1_1008_4772(CONCAT22(uStack18._2_2_, uStack18 as u16)
        );
        pass1_1018_214e(ctx.PTR__LOOP_1050_0382,
                        (ctx.PTR__LOOP_1050_0382 >> 0x10),
                        (param_1 & 0xffff0000 | &iVar5.field_0x28),
                        iVar5.field_0x2e);
        local_24 = &iVar5.field_0x1c;
        uStack32 = iVar5.field_0x20;
        pass1_1008_3f32(CONCAT22(param_7, &local_24),
                        (param_1 & 0xffff0000 | &iVar5.field_0x28),
        );
        piStack48 = (param_1 & 0xffff0000 | &iVar5.field_0x32);
        pass1_1008_3e94(CONCAT22(param_7, &local_24),
                        (param_1 & 0xffff0000 | ZEXT24(&iVar5.field_0x34)),
                        (param_1 & 0xffff0000 | &iVar5.field_0x32));
        // uVar8 = (uStack30 >> 0x10);
        iVar5.field_0x36 = (uStack30 + 0x4) + *piStack48;
        uVar2 = (uStack30 + 0x8) + iVar5.field_0x34;
        iVar5.field_0x38 = uVar2;
        pass1_1008_612e(0x2, 0x5, uVar2);
        iVar5.field_0x3e = uVar2;
    }
    return;
}


pub fn pass1_1008_8aa2(param_1: U32Ptr) {
    let puVar1: u32;
    let uVar2: u16;
    let uVar3: u16;
    let ppcVar4: u32;
    let uVar5: u32;
    let iVar6: &mut Struct462;
    let uVar6: u16;
    let paStack16: &mut Struct18;

    // uVar6 = (param_1 >> 0x10);
    iVar6 = param_1;
    *param_1 = 0x8e9a;
    iVar6.field_0x2 = 0x1008;
    uVar5 = &iVar6.field_0x4;
    if ((uVar5 + 0x1c) != 0x0) {
        puVar1 = iVar6.field_0x4;
        uVar2 = iVar6.field_0x6;
        if ((uVar2 | puVar1) != 0x0) {
            ppcVar4 = *puVar1;
            (**ppcVar4)();
        }
    }
    uVar2 = iVar6.field_0x3a;
    uVar3 = iVar6.field_0x3c;
    paStack16 = CONCAT22(uVar3, uVar2);
    if ((uVar3 | uVar2) != 0x0) {
        pass1_1008_5118(CONCAT22(uVar3, uVar2));
        fn_ptr_1000_17ce(ctx, paStack16, 0x1000);
    }
    *param_1 = 0x389a;
    iVar6.field_0x2 = 0x1008;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_8b20(param_1: u32, param_2: u16) {
    let iVar1: i16;
    let piVar2: U32Ptr;
    let uVar3: u16;
    let iVar4: i16;
    let uVar5: u16;
    let local_a: [u8; 2];
    let local_8: [u8; 2];
    let paStack6: &mut Struct76;

    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1 as i16;
    if ((iVar4 + 0x8) != 0x0) {
        iVar1 = (iVar4 + 0x40);
        piVar2 = (iVar4 + 0x40) as u32;
        *piVar2 = *piVar2 + 0x1;
        uVar3 = (iVar1 % (iVar4 + 0x3e)) as u16;
        if (uVar3 == 0x0) {
            (iVar4 + 0x40) = 0x1;
            piVar2 = (iVar4 + 0x8) as u32;
            pass1_1018_20ee(ctx.PTR__LOOP_1050_0382, piVar2);
            paStack6 = (piVar2 & 0xffff | uVar3 << 0x10);
            pass1_1008_3e94((param_1 & 0xffff0000 | (iVar4 + 0x28)),
                            CONCAT22(param_2, local_a),
                            CONCAT22(param_2, local_8));
            pass1_1008_8d8a(param_1 & 0xffff | uVar5 << 0x10, paStack6,
                            ((iVar4 + 0x4) as u32));
            pass1_1008_4480(((iVar4 + 0x4) as u32),
                            (param_1 & 0xffff0000 | (iVar4 + 0x28)), paStack6,
                            param_2);
            return;
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_8bc6(param_1: u16, param_2: u16, param_3: u32) {
    let piVar1: U32Ptr;
    let iVar2: i16;
    let uVar3: u16;
    let local_a: [u8; 2];
    let local_8: [u8; 2];
    let paStack6: &mut Struct76;

    // uVar3 = (param_3 >> 0x10);
    iVar2 = param_3 as i16;
    if ((iVar2 + 0x8) == 0x0) {
        return;
    }
    piVar1 = (iVar2 + 0x8) as u32;
    pass1_1018_20ee(ctx.PTR__LOOP_1050_0382, piVar1);
    paStack6 = (piVar1 & 0xffff | param_2 << 0x10);
    pass1_1008_3e94((param_3 & 0xffff0000 | (iVar2 + 0x28)),
                    CONCAT22(param_1, local_a), CONCAT22(param_1, local_8));
    pass1_1008_8d8a(param_3 & 0xffff | uVar3 << 0x10, paStack6, ((iVar2 + 0x4) as u32),
    );
    pass1_1008_4480(((iVar2 + 0x4) as u32),
                    (param_3 & 0xffff0000 | (iVar2 + 0x28)), paStack6,
                    param_1);
    return;
}


pub fn pass1_1008_8c4e(param_1: u32, param_2: u32, param_3: u16) {
    let uVar1: u16;
    let puVar2: U32Ptr;
    let puVar3: U32Ptr;
    let puVar4: U32Ptr;
    let uVar5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let uVar8: u32;
    Struct110 * paStack14;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1 as i16;
    uVar8 = pass1_1008_4772((iVar6 + 0x4));
    // puVar3 = (uVar8 >> 0x10);
    uVar5 = 0x0;
    if (((iVar6 + 0xc) == 0x0) || ((iVar6 + 0xe) == 0x0)) {
        puVar4 = puVar3;
        mem_op_1000_179c(0x14, puVar3, 0x1000);
        paStack14 =
        CONCAT22(puVar4 as u16, uVar5);
        uVar5 = (puVar4 | uVar5) as u16;
        if (uVar5 == 0x0) {
            uVar1 = 0x0;
            uVar5 = 0x0;
        } else {
            puVar2 = (param_1 & 0xffff0000 | (iVar6 + 0x1c));
            pass1_1008_50c2(paStack14, (uVar8 + 0x8), (uVar8 + 0x4),
                            puVar2, param_2);
            uVar1 = SUB42(puVar2 as u16, 0x0);
        }
        pass1_1008_5134(CONCAT22(uVar5, uVar1));
    }
    pass1_1008_4480(param_2, (param_1 & 0xffff0000 | (iVar6 + 0x1c)),
                    (iVar6 + 0x4), param_3);
    return;
}


pub fn pass1_1008_8ce4(param_1: u32, param_2: U32Ptr, param_3: u32, param_4: u16) {
    let puVar1: U32Ptr;
    let puVar2: U32Ptr;
    let uVar3: u16;
    let iVar4: i16;
    let uVar5: u16;
    let uVar6: u16;
    let puVar7: U32Ptr;
    let local_10: [u8; 6];
    let uStack10: u32;
    let uStack6: u32;

    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1 as i16;
    uStack6 = pass1_1008_4772((iVar4 + 0x4));
    uStack10 = 0x0;
    puVar7 = pass1_1008_3e54(CONCAT22(param_4, local_10), 0x0,
                             ((iVar4 + 0x12) as u16), ((iVar4 + 0x10) as u16));
    // puVar2 = (puVar7 >> 0x10);
    puVar1 = local_10;
    pass1_1008_3f32(param_2, CONCAT22(param_4, puVar1 as u16));
    mem_op_1000_179c(0x14, puVar2, 0x1000);
    uVar3 = (puVar2 | puVar1) as u16;
    if (uVar3 == 0x0) {
        puVar1 = 0x0;
        uVar3 = 0x0;
    } else {
        // uVar6 = (uStack6 >> 0x10);
        pass1_1008_50c2(CONCAT22(puVar2 as u16, puVar1 as u16), (uStack6 + 0x8),
                        (uStack6 + 0x4), param_2, param_3);
    }
    uStack10 = CONCAT22(uVar3, puVar1 as u16);
    pass1_1008_5134(CONCAT22(uVar3, puVar1 as u16));
    pass1_1008_4480(param_3, param_2, (iVar4 + 0x4), param_4);
    return;
}


pub fn pass1_1008_8d8a(param_1: u32, param_2: &mut Struct76, param_3: u32) {
    let uVar1: u16;
    let cVar2: u8;
    let puVar3: U32Ptr;
    let puVar4: U32Ptr;
    let puVar5: U32Ptr;
    let uVar6: u16;
    Struct112 * iVar7;
    let uVar7: u16;
    let uVar8: u32;
    Struct110 * paStack10;

    // uVar7 = (param_1 >> 0x10);
    iVar7 =  param_1;
    uVar1 = iVar7.field_0x2e;
    if (uVar1 < 0x28) {
        if ((uVar1 < 0x25) && (uVar1 != 0x23)) {
            if (0x23 < uVar1) {
                return;
            }
            cVar2 = uVar1 as u8;
            if (((cVar2 != '\v') && (cVar2 != '\x0f')) && (cVar2 != '!')) {
                return;
            }
        }
    } else {
        if (uVar1 < 0x46) {
            if (uVar1 < 0x43) {
                if (uVar1 < 0x33) {
                    return;
                }
                if ((uVar1 != 0x34 && 0x0 < (uVar1 - 0x33)) && (uVar1 != 0x37)) {
                    return;
                }
            }
        } else {
            if (uVar1 != 0x49) {
                if ((uVar1 - 0x49) < 0x2a) {
                    return;
                }
                if (0x5 < (uVar1 - 0x73)) {
                    return;
                }
            }
        }
    }
    if (iVar7.field_0x3a == 0x0) {
        uVar8 = pass1_1008_4772(param_2);
        // puVar4 = (uVar8 >> 0x10);
        uVar1 = uVar8 as u16;
        puVar5 = puVar4;
        uVar6 = uVar1;
        mem_op_1000_179c(0x14, puVar4, 0x1000);
        paStack10 =
        CONCAT22(puVar5 as u16, uVar6);
        uVar6 = (puVar5 | uVar6) as u16;
        if (uVar6 == 0x0) {
            iVar7.field_0x3a = 0x0;
        } else {
            puVar3 = (param_1 & 0xffff0000 | &iVar7.field_0x28);
            pass1_1008_50c2(paStack10, ((uVar1 + 0x8) as u32), ((uVar1 + 0x4) as u32), puVar3,
                            param_3);
            &iVar7.field_0x3a = puVar3;
            (&iVar7.field_0x3a + 0x2) = uVar6;
        }
        pass1_1008_5134(iVar7.field_0x3a);
        return;
    }
    pass1_1008_5236(iVar7.field_0x3a);
    return;
}


pub fn pass1_1008_8e74(param_1: U32Ptr, param_2: u8) -> u16

{
    pass1_1008_8aa2(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_8f24(param_1: U32Ptr) {
    let puVar1: u32;
    let puVar2: u32;
    let uVar3: u16;
    let ppcVar4: u32;
    let uVar5: u32;
    let iVar6: &mut Struct463;
    let iVar7: i16;
    let iVar8: i16;
    let uVar9: u16;
    let uVar10: u16;
    let uStack6: u32;

    // uVar9 = (param_1 >> 0x10);
    iVar6 = param_1;
    *param_1 = 0x9170;
    iVar6.field_0x2 = 0x1008;
    if (iVar6.field_0x1a != 0x0) {
        uStack6 = 0x0;
        loop {
            puVar1 = &iVar6.field_0xa;
            if (*puVar1 < uStack6 || *puVar1 == uStack6) { break; }
            iVar8 = (uStack6 * 0x4) as i16;
            uVar5 = iVar6.field_0x6;
            // uVar10 = (uVar5 >> 0x10);
            iVar7 = uVar5 as i16;
            puVar2 = (iVar7 + iVar8) as u32;
            uVar3 = (iVar7 + iVar8 + 0x2) as u16;
            if ((uVar3 | puVar2) != 0x0) {
                ppcVar4 = *puVar2;
                (**ppcVar4)();
            }
            uStack6 += 0x1;
        }
    }
    fn_ptr_1000_17ce(ctx, iVar6.field_0x6, 0x1000);
    *param_1 = 0x389a;
    iVar6.field_0x2 = 0x1008;
    return;
}


pub fn pass1_1008_8faa(param_1: u32, param_2: u32) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    pass1_1008_9004(param_1 & 0xffff | uVar1 << 0x10, param_2 as u16,
                    ((param_2 >> 0x10) as u16), (param_1 + 0xa));
    return;
}


pub fn pass1_1008_9004(param_1: u32, param_2: u16, param_3: u16, param_4: u32) {
    let puVar1: u32;
    let puVar2: U32Ptr;
    let lVar3: i32;
    let iVar4: &mut Struct107;
    let iVar5: &mut Struct108;
    let uVar4: u16;
    let uVar5: u16;
    let unaff_SS: u16;
    let bVar6: bool;

    // uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = &iVar4.field_0xa;
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4.field_0x6 == 0x0)) {
        puVar2 = (&iVar4.field_0x12 + 0x2);
        bVar6 = *puVar2 < param_4._2_2_;
        if ((bVar6 || *puVar2 == param_4._2_2_) && ((bVar6 || (puVar1 = &iVar4.field_0x12,
                                                               puVar1 < param_4 || puVar1 == param_4)))) {
            pass1_1008_909c(param_1 & 0xffff | uVar4 << 0x10, unaff_SS);
        }
        puVar1 = &iVar4.field_0x12;
        if ((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4.field_0x6 == 0x0)) {
            return;
        }
        puVar2 = &iVar4.field_0xc;
        bVar6 = *puVar2 < param_4._2_2_;
        if ((bVar6 || *puVar2 == param_4._2_2_) && ((bVar6 || (puVar2 = &iVar4.field_0xa,
                                                               *puVar2 < param_4 || *puVar2 == param_4)))) {
            iVar4.field_0xa = (param_4 + 0x1);
            iVar4.field_0xc = (param_4 + 0x1 >> 0x10);
        }
    }
    lVar3 = iVar4.field_0x6;
    // uVar5 = (lVar3 >> 0x10);
    iVar5 = lVar3;
    (iVar5 + param_4 * 0x4) = param_2;
    (iVar5 + param_4 * 0x4 + 0x2) = param_3;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_909c(param_1: u32, param_2: u16) {
    let puVar1: U32Ptr;
    let uVar2: u16;
    let uVar3: u16;
    let iVar5: &mut Struct106;
    let uVar4: u16;
    let lVar5: i32;
    let lStack10: i32;
    let uStack6: u32;

    // uVar4 = (param_1 >> 0x10);
    iVar5 = param_1;
    if (&iVar5.field_0x12 == 0x0) {
        uVar3 = iVar5.field_0xe;
        ctx.PTR_LOOP_1050_5f2e = iVar5.field_0x10;
    } else {
        uVar2 = &iVar5.field_0x12;
        puVar1 = &iVar5.field_0x16;
        uVar3 = uVar2 + *puVar1;
        ctx.PTR_LOOP_1050_5f2e =

            (iVar5.field_0x14 + iVar5.field_0x18 + CARRY2(uVar2, *puVar1));
    }
    uStack6 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, uVar3);
    if (iVar5.field_0x6 == 0x0) {
        if (ctx.PTR__LOOP_1050_5f2c == 0x0) {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
        } else {}
        uVar3 = fn_ptr_op_1000_1708(uVar3 << 0x2, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                    ctx.PTR_LOOP_1050_5f2e, 0x1000);
    } else {
        lVar5 = iVar5.field_0x6;
        lVar5 = pass1_1000_0ed4(
            ctx, 0x1000, param_2, 0x1, uVar3 * 0x4,
            (ctx.PTR_LOOP_1050_5f2e * 0x2 + CARRY2(uVar3, uVar3)) * 0x2 + CARRY2(uVar3 * 0x2, uVar3 * 0x2), lVar5,
            ((lVar5 >> 0x10) as u16));
        ctx.PTR_LOOP_1050_5f2e = (lVar5 >> 0x10);
        uVar3 = lVar5 as u16;
    }
    lStack10 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, uVar3) as i32;
    if ((ctx.PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
        &iVar5.field_0x12 = uStack6;
        iVar5.field_0x6 = lStack10;
    }
    return;
}


pub fn pass1_1008_914a(param_1: U32Ptr, param_2: u8) -> u16

{
    pass1_1008_8f24(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_91ba(param_1: U32Ptr, param_2: HWND16) -> u16

{
    let UVar1: u16;
    let iVar2: i16;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    iVar2 = param_1 as i16;
    *param_1 = 0x389a;
    (iVar2 + 0x2) = 0x1008;
    (iVar2 + 0x4) = 0x0;
    set_struct_1008_574a((param_1 & 0xffff0000 | (iVar2 + 0x6)));
    *param_1 = 0x9416;
    (iVar2 + 0x2) = 0x1008;
    ctx._PTR_LOOP_1050_0388 = param_1;
    UVar1 = SetTimer16(param_2, 0x0, 0x0, (&ctx.PTR_LOOP_1050_0000 + 0x1));
    if (UVar1 == 0x0) {
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    ctx.PTR_LOOP_1050_038a = (ctx.PTR__LOOP_1050_0388 >> 0x10);
    return param_1 as u16;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


pub fn pass1_1008_9262(param_1: i16, param_2: u16, param_3: u32, param_4: u32, param_5: u16,
                       param_6: U32Ptr)

{
    let ppcVar1: u32;
    let uVar2: u16;

    mem_op_1000_179c(0x12, param_6, 0x1000);
    uVar2 = (param_6 | param_5) as u16;
    if (uVar2 == 0x0) {
        param_5 = 0x0;
        uVar2 = 0x0;
    } else {
        struct_op_1008_9174(CONCAT22(param_6 as u16, param_5), param_3, param_4);
    }
    if ((uVar2 | param_5) != 0x0) {
        ppcVar1 = ((param_1 + 0x6) + 0x4) as u32;
        (**ppcVar1)();
    }
    return;
}


pub fn pass1_1008_92b2(param_1: u32, param_2: i32, param_3: i32) {
    let ppcVar1: u32;
    let puVar2: U32Ptr;
    let extraout_DX: u16;
    let unaff_SS: u16;
    let local_c: [u8; 4];
    let uStack8: u32;
    let uStack4: u16;

    uStack4 = 0x0;
    pass1_1008_57a4(CONCAT22(unaff_SS, local_c),
                    param_1 & 0xffff0000 | (param_1 + 0x6));
    loop {
        puVar2 = local_c;
        pass1_1008_5b12(puVar2, unaff_SS);
        if ((extraout_DX | puVar2) == 0x0) { break; }
        if (((puVar2 + 0x4) == param_3 as u32) && ((puVar2 + 0x8) == param_2 as u32)) {
            uStack4 = 0x1;
            ppcVar1 = ((param_1 + 0x6) + 0xc);
            (**ppcVar1)();
            uStack8 = 0x0;
        }
    }
    return;
}


pub fn pass1_1008_932a(param_1: u32, param_2: u16) {
    let uVar1: u16;
    let ppcVar2: u32;
    let puVar3: U32Ptr;
    let extraout_DX: u16;
    let iVar4: i16;
    let iVar5: i16;
    let uVar6: u16;
    let local_a: [u8; 8];

    // uVar6 = (param_1 >> 0x10);
    iVar5 = param_1 as i16;
    if ((iVar5 + 0x4) == 0x0) {
        (iVar5 + 0x4) = 0x1;
        pass1_1008_57a4(CONCAT22(param_2, local_a),
                        param_1 & 0xffff0000 | (iVar5 + 0x6));
        loop {
            puVar3 = local_a;
            pass1_1008_5b12(puVar3, param_2);
            if ((extraout_DX | puVar3) == 0x0) { break; }
            uVar1 = (puVar3 + 0xc) as u16;
            iVar4 = (puVar3 + 0xe) - (uVar1 < 0x37);
            (puVar3 + 0xc) = (uVar1 - 0x37) as u32;
            (puVar3 + 0xe) = iVar4 as u32;
            if ((iVar4 < 0x1) && (((iVar4 < 0x0 || ((puVar3 + 0xc) == 0x0)) && ((puVar3 + 0x10) == 0x0)))) {
                ppcVar2 = (*(puVar3 + 0x4) + 0x4);
                (**ppcVar2)();
                (puVar3 + 0xc) = (puVar3 + 0x8);
            }
        }
        (iVar5 + 0x4) = 0x0;
    }
    return;
}


pub fn pass1_1008_93c0(param_1: U32Ptr, param_2: u8) -> u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_93ec(param_1: U32Ptr, param_2: u8) -> u16

{
    let unaff_CS: u16;

    kill_timer_1008_921c(param_1, unaff_CS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_941a(param_1: U32Ptr, param_2: u16, param_3: u16) -> u16

{
    *param_1 = param_2;
    (param_1 + 0x2) = param_3 as u32;
    return param_1 as u16;
}


pub fn pass1_1008_9436(param_1: U32Ptr) -> u16

{
    *param_1 = 0x0;
    (param_1 + 0x2) = 0x0;
    return param_1 as u16;
}


pub fn pass1_1008_944e(param_1: U32Ptr, param_2: u16, param_3: u16) {
    (param_1 + 0x2) = param_3 as u32;
    *param_1 = param_2;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_9466(param_1: U32Ptr) {
    *param_1 = 0x52a;
    (param_1 + 0x2) = 0x1008;
    fn_ptr_1000_17ce(ctx, ctx.PTR__LOOP_1050_0392, 0x1000);
    ctx._PTR_LOOP_1050_0392 = 0x0;
    return;
}


pub fn pass1_1008_9628(param_1: u32, param_2: u16) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x8) == 0x0) {
        (param_1 + 0x8) = param_2 as u32;
    }
    return;
}


pub fn pass1_1008_9c16(param_1: u16, param_2: u32, param_3: u32, param_4: HWND16) -> LRESULT {
    let LVar1: LRESULT;

    LVar1 = make_def_wnd_proc_1008_9ce6(param_1, param_2, (param_2 >> 0x10), param_3,
                                        CONCAT22(0x85, ((param_3 >> 0x10) as u16)), param_4);
    return LVar1;
}


pub fn pass1_1008_9c30(param_1: u16, param_2: u32, param_3: u32, param_4: HWND16) -> LRESULT {
    let lvar1: LRESULT;
    lvar1 = make_def_wnd_proc_1008_9ce6(param_1, param_2, (param_2 >> 0x10), param_3,
                                        CONCAT22(0x86, ((param_3 >> 0x10) as u16)), param_4);
    return lvar1;
}


pub fn pass1_1008_9c4a() {
    return;
}


pub fn pass1_1008_9c4e() {
    return;
}


pub fn pass1_1008_9c52() {
    return;
}


pub fn pass1_1008_9c60(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: i16) {
    let ppcVar1: u32;

    if ((param_4 == 0xc7) && (param_3 != 0x0)) {
        ppcVar1 = *param_3;
        (**ppcVar1)();
    }
    return;
}


pub fn pass1_1008_9c86(param_1: u32, param_2: &mut String, param_3: i16) {
    let uVar1: u16;

    uVar1 = str_op_1000_3da4((param_1 & 0xffff0000 | (param_1 + 0xa)));
    if (param_3 < uVar1 as i16) {
        uVar1 = (param_3 - 0x1) as u16;
    }
    str_op_1000_3dbe(param_2, (param_1 & 0xffff0000 | (param_1 + 0xa)),
                     uVar1);
    return;
}


pub fn pass1_1008_9cc4(param_1: u32, param_2: i16) -> bool

{
    if ((param_1 + 0x8) != param_2 as u32) {
        return 0x1;
    }
    return 0x0;
}


pub fn pass1_1008_9ce0() -> u16

{
    return 0x0;
}


pub fn pass1_1008_9d02(param_1: U32Ptr, param_2: u8) -> u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_9d36(param_1: &mut Struct19, param_2: &mut Struct19, param_3: u16, param_4: u16) {
    let puVar1: U32Ptr;
    let uVar2: u16;
    let puVar3: U32Ptr;
    let paVar4: &mut Struct43;
    let uVar5: u32;
    let iStack4: i16;

    struct_op_1018_4cda(param_1, param_2, param_3);
    (param_1 + 0x1c) = 0x389a;
    (param_1 + 0x1e) = 0x1008;
    (param_1 + 0x1c) = 0x3aa8;
    (param_1 + 0x1e) = 0x1008;
    (param_1 + 0x20) = 0x0;
    puVar3 = struct_1008::clear_struct_1008_3e38(CONCAT22(param_2, param_1 + 0x52));
    // puVar1 = (puVar3 >> 0x10);
    CONCAT22(param_2, param_1) = 0x9fb2;
    (param_1 + 0x2) = 0x1008;
    (param_1 + 0x1c) = 0x9fca;
    (param_1 + 0x1e) = 0x1008;
    ctx.PTR_LOOP_1050_4230 = param_1;
    ctx.PTR_LOOP_1050_4232 = param_2;
    pass1_1000_4906(CONCAT22(param_2, param_1 + 0x22), 0x0, 0x30);
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x1c0, puVar1, param_4);
    iStack4 = 0x0;
    loop {
        paVar4 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc, (iStack4 + 0x1c0) as u16, param_4);
        (param_1 + iStack4 * 0x4 + 0x22) = paVar4;
        (param_1 + iStack4 * 0x4 + 0x24) = (paVar4 >> 0x10);
        iStack4 += 0x1;
        if (iStack4 < 0xc) == false { break; }
    }
    uVar5 = pass1_1008_4772((param_1 + 0x22));
    // uVar2 = (uVar5 >> 0x10);
    pass1_1008_3e76(CONCAT22(param_2, param_1 + 0x52), 0x0,
                    ((0x1e0 - (uVar5 + 0x8)) / 0x2 - 0x32) as i16,
                    ((0x280 - (uVar5 + 0x4)) / 0x2) as u16);
    if CONCAT22(param_2, param_1) == 0x0 {
        puVar1 = 0x0;
        param_2 = 0x0;
    } else {
        puVar1 = param_1 + 0x1c;
    }
    pass1_1008_9262(ctx.PTR__LOOP_1050_0388, (ctx.PTR__LOOP_1050_0388 >> 0x10),
                    0x50, CONCAT22(param_2, puVar1 as u16), puVar1 as u16, param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_9e5a(param_1: &mut Struct18) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let puVar4: U32Ptr;
    let uVar6: u16;
    let uVar5: &mut Struct464;
    let uVar7: u16;
    let puStack8: U32Ptr;
    let iStack4: i16;

    // uVar7 = (param_1 >> 0x10);
    uVar5 = param_1;
    param_1 = 0x9fb2;
    uVar5.field_0x2 = 0x1008;
    uVar5.field_0x1c = 0x9fca;
    uVar5.field_0x1e = 0x1008;
    if (ctx.PTR__LOOP_1050_0388 != 0x0) {
        if (param_1 == 0x0) {
            puVar4 = 0x0;
            uVar6 = 0x0;
        } else {
            puVar4 = &uVar5.field_0x1c;
            uVar6 = uVar7;
        }
        pass1_1008_92b2(ctx.PTR__LOOP_1050_0388, 0x50, CONCAT22(uVar6, puVar4 as u16));
    }
    iStack4 = 0x0;
    loop {
        puVar1 = (&uVar5[0x1].field_0x0 + iStack4 * 0x4);
        uVar2 = (&uVar5[0x1].field_0x2)[iStack4 * 0x2];
        if (uVar2 | puVar1) != 0x0 {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        iStack4 += 0x1;
        if (iStack4 < 0xc) == false { break; }
    }
    if param_1 == 0x0 {
        puVar4 = 0x0;
        uVar7 = 0x0;
    } else {
        puVar4 = &uVar5.field_0x1c;
    }
    puStack8 = CONCAT22(uVar7, puVar4 as u16);
    *puStack8 = 0x389a;
    puVar4[0x1] = 0x1008;
    clenaup_win_ui_1018_4d22(param_1, 0x1018);
    return;
}


pub fn pass1_1008_9f18(param_1: i16, param_2: u16, param_3: i16, param_4: u16) {
    if (param_3 == 0x2) {
        pass1_1008_9f64(CONCAT22(param_2, (param_1 + -0x1c) as u16));
        pass1_1010_1f62(param_4, CONCAT22(param_2, (param_1 + -0x1c) as u16), 0x2);
    }
    return;
}


pub fn pass1_1008_9f48(param_1: u32) -> u32

{
    let iVar1: &mut Struct134;
    let iVar2: i16;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar2 = iVar1.field_0x20 * 0x4;
    return CONCAT22((&iVar1[0x1].field_0x2 + iVar2),
                    (&iVar1[0x1].field_0x0 + iVar2));
}


pub fn pass1_1008_9f64(param_1: u32) {
    let piVar1: U32Ptr;
    let iVar2: i16;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    iVar2 = param_1 as i16;
    piVar1 = (iVar2 + 0x20) as u32;
    *piVar1 = *piVar1 + 0x1;
    if (0xb < (iVar2 + 0x20)) {
        (iVar2 + 0x20) = 0x0;
    }
    return;
}


pub fn pass1_1008_9f80(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8,
) -> &mut Struct18

{
    // param_1 = (param_1 & 0xffff0000 | (param_1 - 0x1c));
    pass1_1008_9e5a(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_9fb2(param_1: u16, param_2: i16, param_3: u16, param_4: u16, param_5: u16, param_6: u8, param_7: u8, param_8: i16, param_9: i16, param_10: u8)

{
    let mut pcVar1: String;
    let pbVar2: U32Ptr;
    let bVar3: u8;
    let uVar4: u16;
    let pcVar5: u32;
    let bVar6: u8;
    let uVar7: u16;
    let uVar8: u16;
    let extraout_DL: u8;
    let puVar9: U32Ptr;
    let puVar10: U32Ptr;
    let extraout_DX: U32Ptr;
    let extraout_DX_00: u16;
    let uVar11: u16;
    let bVar12: u8;
    let bVar13: bool;
    let bVar14: bool;
    let paVar15: &mut Struct79;

    (param_8 + 0x1008) = ctx.data_seg;
    uVar8 = param_10 as u16;
    uVar4 = param_5 + 0xeff0;
    bVar12 = param_5 < 0x1010 || uVar4 < uVar8;
    uVar7 = uVar4 - uVar8;
    pcVar5 = swi(0x4);
    if (SBORROW2(param_5 as u32, 0x1010) != SBORROW2(uVar4 as u32, uVar8 as u32)) {
        (*pcVar5)();
        param_7 = extraout_DL;
    }
    bVar6 = (((uVar7 + 0xeff0) - bVar12) % 0x1d) as u8;
    pcVar1 = (param_8 + param_9);
    *pcVar1 = *pcVar1 + param_7 + (uVar7 < 0x1010 || uVar7 + 0xeff0 < bVar12 as u16);
    pbVar2 = (param_8 + param_9) as u32;
    bVar13 = *pbVar2 < param_7 || (*pbVar2 - param_7) < (0xb1 < bVar6);
    *pbVar2 = (*pbVar2 - param_7) - (0xb1 < bVar6);
    pbVar2 = (param_8 + 0x18) as u32;
    bVar14 = *pbVar2 < param_6 || (*pbVar2 - param_6) < bVar13;
    *pbVar2 = (*pbVar2 - param_6) - bVar13;
    pbVar2 = (param_8 + param_9 + 0x89f) as u32;
    bVar12 = *pbVar2;
    bVar3 = *pbVar2 + bVar6 + 0x4e;
    *pbVar2 = bVar3 + bVar14;
    pcVar1 = (param_8 + param_9);
    *pcVar1 = *pcVar1 + param_8 + (CARRY1(bVar12, bVar6 + 0x4e) || CARRY1(bVar3, bVar14));
    pbVar2 = (param_8 + param_9) as u32;
    *pbVar2 = *pbVar2 | param_7;
    paVar15 = struct_op_1010_1d48(CONCAT22(param_3, param_2 as u16), param_4);
    // puVar9 = (paVar15 >> 0x10);
    uVar8 = 0x0;
    (param_2 + 0xa) = 0x0;
    (param_2 + 0x410) = 0x0;
    (param_2 + 0x414) = 0x0;
    (param_2 + 0x416) = 0x0;
    (param_2 + 0x418) = 0x0;
    (param_2 + 0x41a) = 0x0;
    (param_2 + 0x41c) = 0x0;
    (param_2 + 0x41e) = 0x0;
    CONCAT22(param_3, param_2 as u16) = 0xad92;
    (param_2 + 0x2) = 0x1008;
    mem_op_1000_179c(0xc, puVar9, 0x1000);
    puVar10 = (puVar9 | uVar8);
    if (puVar10 == 0x0) {
        (param_2 + 0xa) = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(puVar9 as u16, uVar8));
        (param_2 + 0xa) = uVar8 as i16;
        (param_2 + 0xc) = extraout_DX as i16;
        puVar10 = extraout_DX;
    }
    mem_op_1000_179c(0xc, puVar10, 0x1000);
    if ((puVar10 | uVar8) == 0x0) {
        uVar8 = 0x0;
        uVar11 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(puVar10 as u16, uVar8));
        uVar11 = extraout_DX_00;
    }
    (param_2 + 0x410) = uVar8 as i16;
    (param_2 + 0x412) = uVar11 as i16;
    return;
}


pub fn pass1_1008_a086(param_1: U32Ptr, param_2: u16) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar4: &mut Struct465;
    let uVar4: u16;

    // uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    *param_1 = 0xad92;
    iVar4.field_0x2 = 0x1008;
    puVar1 = iVar4.field_0xa;
    uVar2 = iVar4.field_0xc;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    puVar1 = iVar4.field_0x410;
    uVar2 = iVar4.field_0x412;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub unsafe fn pass1_1008_b200(
    ctx: &mut AppContext,
    param_1: &mut Struct194,
    param_2: u16,
    extraout_dx: u16,
    extraout_dx_00: u16,
    extraout_dx_01: u16,
    extraout_dx_02: u16,
) {
    let u_var1: u32;
    let ppc_var2: u32;
    let pu_var3: u32;
    let pu_var4: U32Ptr;
    let u_var5: &mut Struct195;
    let u_var6: u16;
    let u_var7: u32;
    // let extraout_dx: U32Ptr;
    // let extraout_dx_00: U32Ptr;
    let pu_var8: U32Ptr;
    let pu_var9: U32Ptr;
    // let extraout_dx_01: u16;
    let u_var10: u16;
    let u_var11: u16;
    // let extraout_dx_02: U32Ptr;
    // let i_var12: Struct194;
    let mut pc_var13: String;
    let local_14: [u8; 12];

    // uVar12 = (param_1 >> 0x10);
    //  i_var12 = param_1;
    if param_1.field_0xe != 0x0 {
        return;
    }
    // WARNING: Load size is inaccurate
    pu_var3 = param_1.field_0xe as u32;
    pu_var9 = (param_1.field_0xe + 0x2) as u32;
    if (pu_var9 | pu_var3) != 0x0 {
        ppc_var2 = *pu_var3;
        (**ppc_var2)();
        pu_var9 = extraout_dx as u32;
    }
    mem_op_1000_179c(ctx, 0xc, pu_var9, 0x1000);
    if (pu_var9 | pu_var3) == 0x0 {
        pu_var3 = 0x0;
        pu_var9 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(pu_var9 as u16, pu_var3 as u16));
        pu_var9 = extraout_dx_00 as u32;
    }
    i_var12.field_0xe = pu_var3;
    (i_var12.field_0xe + 0x2) = pu_var9;
    pass1_1028_dc52(CONCAT22(param_2, local_14), 0x1, 0x0, 0x400);
    loop {
        pu_var8 = pu_var9;
        pu_var4 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, pu_var4 as u16));
        pu_var9 = (pu_var8 | pu_var4);
        if pu_var9 == 0x0 { break; }
        u_var1 = (pu_var4 + 0x4);
        if (pu_var4 + 0x200) == 0x8000001 {
            u_var7 = u_var1;
            mem_op_1000_179c(0xc, pu_var9, 0x1000);
            u_var5 = u_var7;
            if ((pu_var9 | u_var5) == 0x0) {
                u_var5 = 0x0;
                u_var10 = 0x0;
            } else {
                pass1_1008_b0f2((u_var7 & 0xffff | ZEXT24(pu_var9 as u16) << 0x10));
                u_var10 = extraout_dx_01;
            }
            pc_var13 = load_string_1038_4d28(CONCAT22(pu_var8 as u16, pu_var4 as u16));
            // u_var11 = (pc_var13 >> 0x10);
            u_var6 = str_op_1008_60e8(ctx, pc_var13, u_var11);
            u_var5.field_0x4 = u_var6;
            u_var5.field_0x6 = u_var11;
            u_var5.field_0x8 = u_var1;
            ppc_var2 = (*i_var12.field_0xe + 0x8);
            (**ppc_var2)(&ctx.PTR_LOOP_1050_1038, i_var12.field_0xe, u_var5, u_var10);
            pu_var9 = extraout_dx_02 as u32;
        }
    }
    return;
}


pub fn pass1_1008_b340(param_1: u32) -> u32

{
    let uVar1: u32;
    let iVar2: i16;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x16) != 0x0) {
        uVar1 = (param_1 + 0x16);
        // uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1 as i16;
        return CONCAT22(((iVar2 + 0x6) as u16), ((iVar2 + 0x4) as u16));
    }
    return 0x0;
}


pub fn pass1_1008_b366(param_1: u32) -> u32

{
    let uVar1: u32;
    let iVar2: i16;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) != 0x0) {
        uVar1 = (param_1 + 0x1a);
        // uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1 as i16;
        return CONCAT22(((iVar2 + 0x6) as u16), ((iVar2 + 0x4) as u16));
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_b38c(param_1: u32, param_2: u16, param_3: U32Ptr) -> u32

{
    let puVar1: u32;
    let ppcVar2: u32;
    let uVar3: u16;
    let iVar3: &mut Struct197;
    let iVar4: &mut Struct196;
    let uVar4: u16;
    let puVar5: U32Ptr;
    let iStack4: i16;
    let iVar5: &mut Struct198;

    // uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    if (iVar4.field_0x12 == 0x0) {
        mem_op_1000_179c(0xc, param_3, 0x1000);
        puVar5 = CONCAT22((param_3 | param_2) as u16, param_2);
        if ((param_3 | param_2) == 0x0) {
            iVar4.field_0x12 = 0x0;
        } else {
            puVar5 = set_struct_1008_574a(CONCAT22(param_3 as u16, param_2));
            &iVar4.field_0x12 = puVar5;
            (&iVar4.field_0x12 + 0x2) = (puVar5 >> 0x10);
        }
        // TODO: refactor for loop
//     for (iStack4 = 0x6d9; iStack4 < 0x6e7; iStack4 += 0x1) {
//       if (iStack4 == 0x6e3) {
//         pass1_1030_8344(ctx.PTR__LOOP_1050_5748,
//                         (ctx.PTR__LOOP_1050_5748 >> 0x10),0x8000001);
//         if ((puVar5 + 0x136) != 0x0) goto LAB_1008_b44a;
//       }
//       else {
// //LAB_1008_b44a:
//         mem_op_1000_179c(0xa,(puVar5 >> 0x10),0x1000);
//         if (puVar5 == 0x0) {
//           puVar5 = 0x0;
//         }
//         else {
//           puVar5 = pass1_1008_b11e(puVar5);
//         }
//         uVar3 = (puVar5 >> 0x10);
//         iVar5 = puVar5;
//         load_string_1010_84ac
//                   (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10),
//                    0x1010);
//         iVar5.field_0x4 = puVar5;
//         iVar5.field_0x6 = (puVar5 >> 0x10);
//         iVar5.field_0x8 = iStack4 + -0x6d8;
//         puVar1 = iVar4.field_0x12;
//         ppcVar2 = (*iVar4.field_0x12 + 0x8);
//         puVar5 =
//                  (**ppcVar2)(0x1010,puVar1,(puVar1 >> 0x10),iVar5,uVar3);
//       }
//     }
    }
    return CONCAT22((&iVar4.field_0x12 + 0x2),
                    &iVar4.field_0x12);
}


pub fn pass1_1008_b47a(param_1: u32) -> u32

{
    let uVar1: u32;
    let iVar2: i16;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x1e) != 0x0) {
        uVar1 = (param_1 + 0x1e);
        // uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1 as i16;
        return CONCAT22(((iVar2 + 0x6) as u16), ((iVar2 + 0x4) as u16));
    }
    return 0x0;
}


pub fn pass1_1008_b4a0(param_1: u32, param_2: i32, param_3: u16, param_4: U32Ptr, param_5: u16) {
    let uVar1: u32;
    let uVar2: u16;
    let uVar3: u16;
    let iVar4: i16;
    let uVar5: u16;
    let uVar6: u32;
    let lVar7: i32;

    iVar4 = param_1 as i16;
    // uVar5 = (param_1 >> 0x10);
    if (param_2 == 0x0) {
        (iVar4 + 0x16) = 0x0;
    } else {
        pass1_1008_b9ce(param_1, param_2 as u32, param_5);
        (iVar4 + 0x16) = param_3 as i16;
        (iVar4 + 0x18) = param_4 as i16;
    }
    uVar1 = (iVar4 + 0x16) as u32;
    if ((uVar1 + 0x8) != 0x0) {
        pass1_1008_b200(ctx, param_1, param_5);
        uVar6 = pass1_1008_b38c(param_1, param_3, param_4);
        // uVar3 = (uVar6 >> 0x10);
        uVar2 = uVar6 as u16;
        uVar1 = (iVar4 + 0x16) as u32;
        pass1_1008_b85c(param_1, ((uVar1 + 0xa) as i32));
        (iVar4 + 0x1a) = uVar2 as i16;
        (iVar4 + 0x1c) = uVar3 as i16;
        uVar1 = (iVar4 + 0x16) as u32;
        lVar7 = pass1_1008_b8ac(param_1, ((uVar1 + 0xe) as i16), param_5);
        (iVar4 + 0x1e) = lVar7 as i16;
        (iVar4 + 0x20) = (lVar7 >> 0x10) as i16;
        return;
    }
    (iVar4 + 0x1a) = 0x0;
    (iVar4 + 0x1e) = 0x0;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_b544(param_1: u32, param_2: i16, param_3: u16, param_4: u16) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let uVar4: u32;
    let uVar5: u32;
    let uVar6: u16;
    let iVar7: i16;
    let uVar8: u16;

    iVar7 = param_1 as i16;
    // uVar8 = (param_1 >> 0x10);
    if (param_2 != 0x0) {
        if ((iVar7 + 0x1a) != 0x0) {
            uVar4 = (iVar7 + 0x16) as u32;
            (uVar4 + 0x8) = 0x1;
            uVar4 = (iVar7 + 0x1a) as u32;
            uVar5 = (iVar7 + 0x16) as u32;
            (uVar5 + 0xa) = (uVar4 + 0x8);
            uVar4 = (iVar7 + 0x1e) as u32;
            uVar6 = (uVar4 + 0x8) as u16;
            uVar4 = (iVar7 + 0x16) as u32;
            (uVar4 + 0xe) = uVar6 as u32;
            uVar4 = (iVar7 + 0x16) as u32;
            pass1_1030_8344(
                ctx, ctx.PTR__LOOP_1050_5748,
                (uVar4 + 0xa));
            param_4 = &ctx.PTR_LOOP_1050_1038;
            pass1_1038_3608(CONCAT22(param_3, uVar6));
        }
    }
    (iVar7 + 0x1e) = 0x0;
    (iVar7 + 0x1a) = 0x0;
    (iVar7 + 0x16) = 0x0;
    puVar1 = (iVar7 + 0xe) as u32;
    uVar2 = (iVar7 + 0x10) as u16;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(param_4, puVar1, uVar2, 0x1);
    }
    (iVar7 + 0xe) = 0x0;
    puVar1 = (iVar7 + 0x12) as u32;
    uVar2 = (iVar7 + 0x14) as u16;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(param_4, puVar1, uVar2, 0x1);
    }
    (iVar7 + 0x12) = 0x0;
    return;
}


pub fn pass1_1008_b61a(param_1: u32, param_2: u32, param_3: u16, param_4: u16, param_5: u16) {
    let uVar1: u16;

    pass1_1008_b8fa(param_1, param_2, param_4, param_5);
    // uVar1 = (param_1 >> 0x10);
    (param_1 + 0x1a) = param_3 as u32;
    (param_1 + 0x1c) = param_4 as u32;
    return;
}


pub fn pass1_1008_b63a(param_1: u32, param_2: u32) {
    let in_AX: u16;
    let in_DX: u16;
    let uVar1: u16;
    let unaff_SS: u16;

    pass1_1008_b964(param_1, param_2, unaff_SS);
    // uVar1 = (param_1 >> 0x10);
    (param_1 + 0x1e) = in_AX as u32;
    (param_1 + 0x20) = in_DX as u32;
    return;
}


pub fn pass1_1008_b820(param_1: u32, param_2: i16, param_3: u16) -> u32

{
    let uVar1: u16;

    pass1_1030_8344(
        ctx, ctx.PTR__LOOP_1050_5748, 0x8000001);
    if ((param_2 + 0x152) == 0x0) {
        return 0x0;
    }
    // uVar1 = (param_1 >> 0x10);
    return CONCAT22(((param_1 + 0xc) as u16), ((param_1 + 0xa) as u16));
}


pub fn pass1_1008_b85c(param_1: u32, param_2: i32) {
    let puVar1: U32Ptr;
    let extraout_DX: u16;
    let unaff_SS: u16;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (param_1 + 0xe));
    loop {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, unaff_SS);
        if ((extraout_DX | puVar1) == 0x0) {
            return;
        }
        if ((puVar1 + 0x8) != param_2 as u32) == false { break; }
    }
    return;
}


pub fn pass1_1008_b8ac(param_1: u32, param_2: i16, param_3: u16) -> i32 {
    let var_1: i32;
    let var_2: [u8; 8] = [0;8];

    pass1_1008_5784(CONCAT22(param_3, var_2[0] as u16), (param_1 + 0x12));
    loop {
        var_1 = pass1_1008_5b12(var_2, param_3);
        if (var_1 == 0x0) {
            return 0x0;
        }
        if ((var_1 + 0x8) != param_2 as i32) == false { break; }
    }
    return var_1;
}


pub fn pass1_1008_b8fa(param_1: u32, param_2: u32, param_3: u16, param_4: u16) {
    let puVar1: U32Ptr;
    let iVar2: i16;
    let extraout_DX: u16;
    let local_a: [u8; 8];

    if (param_2 == 0x0) {
        return;
    }
    pass1_1008_5784(CONCAT22(param_4, local_a), (param_1 + 0xe));
    loop {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, param_4);
        if ((extraout_DX | puVar1) == 0x0) {
            return;
        }
        iVar2 = pass1_1000_3d7a((puVar1 + 0x4), param_2);
        if (iVar2 != 0x0) == false { break; }
    }
    return;
}


pub fn pass1_1008_b964(param_1: u32, param_2: u32, param_3: u16) {
    let puVar1: U32Ptr;
    let iVar2: i16;
    let extraout_DX: u16;
    let local_a: [u8; 8];

    if (param_2 == 0x0) {
        return;
    }
    pass1_1008_5784(CONCAT22(param_3, local_a), (param_1 + 0x12));
    loop {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, param_3);
        if ((extraout_DX | puVar1) == 0x0) {
            return;
        }
        iVar2 = pass1_1000_3d7a((puVar1 + 0x4), param_2);
        if (iVar2 != 0x0) { break; }
    }
    return;
}


pub fn pass1_1008_b9ce(param_1: u32, param_2: u32, param_3: u16) {
    let puVar1: U32Ptr;
    let iVar2: i16;
    let extraout_DX: u16;
    let local_a: [u8; 8];

    if (param_2 == 0x0) {
        return;
    }
    pass1_1008_5784(CONCAT22(param_3, local_a), (param_1 + 0xa));
    loop {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, param_3);
        if ((extraout_DX | puVar1) == 0x0) {
            return;
        }
        iVar2 = pass1_1000_3d7a((puVar1 + 0x4), param_2);
        if (iVar2 != 0x0) == false { break; }
    }
    return;
}


pub fn pass1_1008_ba38(param_1: u32, param_2: u32, param_3: HFILE16, param_4: u16) {
    let uVar1: u32;
    let BVar2: bool;
    let puVar3: U32Ptr;
    let extraout_DX: u16;
    let iVar4: i16;
    let uVar5: u16;
    let uVar6: u16;
    let uVar7: u16;
    let local_2a: [u32; 0x3];
    let local_1e: [u16; 0x5];
    let local_14: [u8; 8];
    let local_c: u16;
    let uStack10: u32;
    let local_6: [u16; 0x2];

    BVar2 = write_to_file_1008_7cac(param_2, param_4);
    if BVar2 != 0x0 {
        // uVar5 = (param_1 >> 0x10);
        iVar4 = param_1 as i16;
        local_c = (iVar4 + 0x22) as u16;
        uVar6 = param_2 as u16;
        // uVar7 = (param_2 >> 0x10);
        BVar2 = write_to_file_1008_7e1c(uVar6, uVar7, &local_c, param_4, 0x2, param_3);
        if BVar2 != 0x0 {
            if (iVar4 + 0xa) == 0x0 {
                local_c = 0x0;
            } else {
                uVar1 = (iVar4 + 0xa) as u32;
                local_c = (uVar1 + 0x8) as u16;
            }
            local_1e[0] = local_c;
            BVar2 = write_to_file_1008_7e1c(uVar6, uVar7, local_1e, param_4, 0x2, param_3);
            if BVar2 != 0x0 {
                pass1_1008_5784(CONCAT22(param_4, local_14), ((iVar4 + 0xa) as u32));
                loop {
                    puVar3 = local_14;
                    pass1_1008_5b12(puVar3, param_4);
                    uStack10 = CONCAT22(extraout_DX, puVar3 as u16);
                    if ((extraout_DX | puVar3) == 0x0) {
                        return;
                    }
                    BVar2 = pass1_1008_7c2a(param_2, (puVar3 + 0x4), param_3);
                    if BVar2 == 0x0 { break; }
                    local_6[0] = (uStack10 + 0x8) as u16;
                    BVar2 = write_to_file_1008_7e1c(uVar6, uVar7, local_6, param_4, 0x2, param_3);
                    if BVar2 == 0x0 { break; }
                    local_2a[0] = (uStack10 + 0xa);
                    BVar2 = write_to_file_1008_7e1c(uVar6, uVar7, local_2a, param_4, 0x4, param_3);
                    if BVar2 == 0x0 { break; }
                    local_6[0] = (uStack10 + 0xe) as u16;
                    BVar2 = write_to_file_1008_7e1c(uVar6, uVar7, local_6, param_4, 0x2, param_3);
                    if (BVar2 != 0x0) == false { break; }
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return;
}


pub fn pass1_1008_bd02(param_1: u32, param_2: u8) -> u32

{
    let unaff_SS: u16;

    pass1_1008_afde(param_1, unaff_SS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_bd28(param_1: &mut Struct18, param_2: u8) -> &mut Struct18

{
    pass1_1008_b08c(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_bd4e(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8,
) -> &mut Struct18

{
    pass1_1008_b08c(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_bd74(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8) -> &mut Struct18 {
    pass1_1008_b08c(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_bd9a(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8,
) -> &mut Struct18

{
    pass1_1008_b08c(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_d3ae(
    ctx: &mut AppContext,
    param_1: u32) {
    let puVar1: u32;
    let puVar2: u32;
    let ppcVar3: u32;
    let bVar4: bool;
    let uVar5: u16;
    let uVar6: u16;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let iVar13: &mut Struct208;
    let uVar10: u16;
    let uVar11: u16;
    let paVar12: &mut Struct21;
    let uVar13: u32;
    let uVar14: u32;
    let uVar15: u32;
    let uStack6: u16;

    // uVar10 = (param_1 >> 0x10);
    iVar13 = param_1;
    // WARNING: Load size is inaccurate
    puVar1 = iVar13.field_0xa;
    uVar5 = (&iVar13.field_0xa + 0x2);
    paVar12 = CONCAT22(uVar5, puVar1 as u16);
    if ((uVar5 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        paVar12 = (**ppcVar3)();
    }
    mem_op_1000_179c(ctx, 0xc, (paVar12 >> 0x10), 0x1000);
    if (paVar12 == 0x0) {
        uVar13 = 0x0;
    } else {
        uVar13 = set_struct_1008_574a(paVar12);
    }
    &iVar13.field_0xa = uVar13;
    (&iVar13.field_0xa + 0x2) = (uVar13 >> 0x10);
    bVar4 = false;
    // TODO: refactor for loop
    // for (uStack6 = 0x21; 0x10 < uStack6; uStack6 -= 0x1) {
    //   uVar15 = uVar13;
    //   empty_1038_540a();
    //   puVar8 = (uVar15 >> 0x10);
    //   uVar5 = puVar8 | uVar15;
    //   uVar13 = uVar15 & 0xffff0000 | uVar5;
    //   if (uVar15 != 0x0) {
    //     bVar4 = true;
    //     string_1020_c0ca(uStack6);
    //     uVar6 = str_op_1008_60e8(CONCAT22(puVar8,uVar5),puVar8);
    //     uVar11 = 0x1000;
    //     uVar7 = uVar6;
    //     puVar9 = puVar8;
    //     mem_op_1000_179c(0x10,puVar8,0x1000);
    //     if ((puVar9 | uVar7) == 0x0) {
    //       uVar14 = 0x0;
    //     }
    //     else {
    //       uVar11 = 0x1018;
    //       uVar14 = struct_1018_4790(CONCAT22(puVar9,uVar7),uVar15,CONCAT22(puVar8,uVar6),
    //                                 uStack6);
    //     }
    //     puVar2 = iVar13.field_0xa;
    //     ppcVar3 = (*iVar13.field_0xa + 0x4);
    //     uVar13 = (**ppcVar3)(uVar11,puVar2,(puVar2 >> 0x10),uVar14);
    //   }
    // }
    if (!bVar4) {
        load_string_1010_84ac(ctx.PTR__LOOP_1050_14cc as i16, ((ctx.PTR__LOOP_1050_14cc >> 0x10) as i16), 0x1010,
        );
        uVar11 = 0x1000;
        uVar15 = uVar13;
        mem_op_1000_179c(0x10, (uVar13 >> 0x10), 0x1000);
        if (uVar15 == 0x0) {
            uVar14 = 0x0;
        } else {
            uVar11 = 0x1018;
            uVar14 = struct_1018_4790(uVar15, 0x0, uVar13, 0x0) as u32;
        }
        puVar2 = iVar13.field_0xa;
        ppcVar3 = (*iVar13.field_0xa + 0x4);
        (**ppcVar3)(uVar11, puVar2, (puVar2 >> 0x10), uVar14);
    }
    return;
}


pub fn pass1_1008_d6f4(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1008_caa0(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_d72e(param_1: &mut Struct19, param_2: &mut Struct19, param_3: u16) -> &mut Struct19

{
    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 0xa) = 0x0;
    CONCAT22(param_2, param_1) = 0xd780;
    (param_1 + 0x2) = 0x1008;
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1008_d75a(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_1d80(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_d790(param_1: &mut Struct647, param_2: &mut Struct19, param_3: u16, param_4: u16) {
    let paVar1: &mut Struct43;

    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    &param_1.field_0xa = 0x0;
    param_1.field_0xe = 0x0;
    CONCAT22(param_2, param_1) = 0xd98e;
    param_1.field_0x2 = 0x1008;
    paVar1 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc, 0x9b, param_4);
    param_1.field_0xa = paVar1;
    param_1.field_0xc = (paVar1 >> 0x10);
    return;
}


pub fn pass1_1008_d7da(param_1: U32Ptr, param_2: u16) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar4: i16;
    let uVar5: u16;

    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1 as i16;
    *param_1 = 0xd98e;
    (iVar4 + 0x2) = 0x1008;
    puVar1 = (iVar4 + 0xa) as u32;
    uVar2 = (iVar4 + 0xc) as u16;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1008_d818(param_1: u32, param_2: i16) {
    let iVar1: &mut Struct732;
    let uVar1: u16;

    if (param_2 - 0x1a0 < 0x15) {
        iVar1 = param_1;
        // uVar1 = (param_1 >> 0x10);
        match param_2
        {
            0x1a0 => { iVar1.field_0xe = 0x14; }

            0x1a1 => { iVar1.field_0xe = 0x3; }

            0x1a2 => { iVar1.field_0xe = 0x2; }

            0x1a3 => { iVar1.field_0xe = 0xe; }

            0x1a4 => { iVar1.field_0xe = 0xc; }

            0x1a5 => { iVar1.field_0xe = 0x4; }

            0x1a6 => { iVar1.field_0xe = 0xb; }

            0x1a7 => { iVar1.field_0xe = 0x6; }

            0x1a8 => { iVar1.field_0xe = 0xa; }

            0x1a9 => { iVar1.field_0xe = 0xd; }

            0x1aa => { iVar1.field_0xe = 0x13; }

            0x1ab => { iVar1.field_0xe = 0x5; }

            0x1ac => { iVar1.field_0xe = 0x9; }

            0x1ad => { iVar1.field_0xe = 0x8; }

            0x1ae => { iVar1.field_0xe = 0x12; }

            0x1af => { iVar1.field_0xe = 0x11; }

            0x1b0 => {
                iVar1.field_0xe = 0x7;
                return;
            }
            0x1b1 => {
                iVar1.field_0xe = 0x10;
                return;
            }
            0x1b2 => {
                iVar1.field_0xe = 0x1;
                return;
            }
            0x1b3 => {
                iVar1.field_0xe = 0xf;
                return;
            }
            0x1b4 => {
                iVar1.field_0xe = 0x15;
                return;
            }
            _ => {}
        }
    }
    return;
}


pub fn pass1_1008_d968(param_1: U32Ptr, param_2: u8) -> u16

{
    let unaff_SS: u16;

    pass1_1008_d7da(param_1, unaff_SS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_d99e(param_1: &mut Struct19, param_2: &mut Struct19, param_3: u16, param_4: U32Ptr, param_5: u16) {
    struct_op_1018_4cda(param_1, param_2, param_3);
    CONCAT22(param_2, param_1) = 0xd9fa;
    (param_1 + 0x2) = 0x1008;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x9a, param_4, param_5);
    ctx._PTR_LOOP_1050_4230 = CONCAT22(param_2, param_1);
    return;
}


pub fn pass1_1008_d9d4(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8,
) -> &mut Struct18

{
    clenaup_win_ui_1018_4d22(ctx, param_1, 0x1018, 0);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_dc2c(param_1: U32Ptr, param_2: u16) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    *param_1 = 0xdc80;
    (param_1 + 0x2) = 0x1008;
    fn_ptr_1000_17ce(ctx, (param_1 + 0x18), 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1008_dc5a(param_1: U32Ptr, param_2: u8) -> u16

{
    let unaff_SS: u16;

    pass1_1008_dc2c(param_1, unaff_SS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


// WARNING: Variable defined which should be unmapped: param_10

pub fn pass1_1008_dc80(param_1: u16, param_2: U32Ptr, param_3: u32, param_4: u32, param_5: u16,
                       param_6: u8, param_7: i16, param_8: i16, param_9: u8, param_10: u16)

{
    let mut pcVar1: String;
    let uVar2: u16;
    let uVar3: u16;
    let pcVar4: u32;
    let uVar5: u16;
    let cVar6: u8;
    let extraout_DL: u8;
    let bVar7: u8;
    let iVar8: i16;
    let uVar9: u16;
    let bVar10: u8;

    bVar7 = (param_10 >> 0x8) as u8;
    bVar10 = (param_10 + bVar7) as u8;
    cVar6 = bVar10 + param_9;
    uVar2 = (CARRY1(param_10, bVar7) || CARRY1(bVar10, param_9));
    uVar3 = param_5 + 0xeff0;
    bVar10 = param_5 < 0x1010 || uVar3 < uVar2;
    uVar5 = uVar3 - uVar2;
    pcVar4 = swi(0x4);
    if (SBORROW2(param_5 as u32, 0x1010) != SBORROW2(uVar3 as u32, uVar2 as u32)) {
        (*pcVar4)();
        cVar6 = extraout_DL;
    }
    pcVar1 = (param_7 + param_8);
    *pcVar1 = *pcVar1 + cVar6 + (uVar5 < 0x1010 || uVar5 + 0xeff0 < bVar10 as u16);
    // uVar9 = (param_2 >> 0x10);
    iVar8 = param_2 as i16;
    *param_2 = 0x389a;
    (iVar8 + 0x2) = 0x1008;
    (iVar8 + 0x4) = param_4 as i16;
    (iVar8 + 0x8) = param_3 as i16;
    (iVar8 + 0xc) = 0x0;
    (iVar8 + 0xe) = 0x0;
    (iVar8 + 0x12) = 0x0;
    *param_2 = 0xdd4a;
    (iVar8 + 0x2) = 0x1008;
    return;
}


pub fn pass1_1008_dd1e(param_1: U32Ptr, param_2: u8) -> u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_ddca(param_1: U32Ptr, param_2: u16) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar5: &mut Struct471;
    let uVar4: u16;

    // uVar4 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = 0xeaac;
    iVar5.field_0x2 = 0x1008;
    puVar1 = iVar5.field_0xe;
    uVar2 = iVar5.field_0x10;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    puVar1 = iVar5.field_0x12;
    uVar2 = iVar5.field_0x14;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    puVar1 = iVar5.field_0xa;
    uVar2 = iVar5.field_0xc;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    fn_ptr_1000_17ce(ctx, iVar5.field_0x1e, 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_de58(param_1: u32, param_2: i32, param_3: i32, param_4: u16) {
    let ppcVar1: u32;
    let bVar2: bool;
    let puVar4: &mut Struct210;
    let extraout_DX: u16;
    let puVar3: U32Ptr;
    let uVar4: u16;
    let iVar6: &mut Struct211;
    let paVar5: &mut Struct210;
    let uVar6: u16;
    let uVar7: u32;
    let local_a: [u8; 8];

    // uVar6 = (param_1 >> 0x10);
    iVar6 = param_1;
    pass1_1008_5784(CONCAT22(param_4, local_a), iVar6.field_0xa);
    bVar2 = false;
    loop {
        puVar4 = local_a;
        pass1_1008_5b12(puVar4, param_4);
        puVar3 = (extraout_DX | puVar4);
        paVar5 = puVar4;
        if (puVar3 == 0x0) {
            // goto
            // LAB_1008_dedb;
        }
        if (((puVar4.field_0x4 != param_3) || (puVar4.field_0x8 != param_2)) && ((puVar4.field_0x8 != param_3 || (puVar4.field_0x4 != param_2)))) == false {  }
    }
    puVar4.field_0xc = 0x1;
    uVar7 = pass1_1030_8326();
    // puVar3 = (uVar7 >> 0x10);
    paVar5 = uVar7;
    puVar4.field_0xe = paVar5;
    puVar4.field_0x10 = puVar3;
    bVar2 = true;
//LAB_1008_dedb:
    if (!bVar2) {
        mem_op_1000_179c(0x14, puVar3, 0x1000);
        uVar4 = puVar3 | paVar5;
        if (uVar4 == 0x0) {
            paVar5 = 0x0;
            uVar4 = 0x0;
        } else {
            struct_1008_dc90(CONCAT22(puVar3 as u16, paVar5), param_2 as u32, param_3 as u32);
        }
        paVar5.field_0xc = 0x1;
        uVar7 = pass1_1030_8326();
        paVar5.field_0xe = uVar7;
        paVar5.field_0x10 = (uVar7 >> 0x10);
        ppcVar1 = (*iVar6.field_0xa + 0x4);
        (**ppcVar1)();
    }
    return;
}


pub fn pass1_1008_df4a(param_1: u32, param_2: i16, param_3: u16) {
    let uVar1: u16;
    let uVar2: u16;
    let uVar3: u32;
    let local_a: [u8; 8];

    // uVar2 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(param_3, local_a), (param_1 + 0xa));
    loop {
        uVar3 = pass1_1008_5b12(local_a, param_3);
        // uVar1 = (uVar3 >> 0x10);
        if (uVar3 == 0x0) { break; }
        if (((uVar3 + 0xc) == 0x2) || ((uVar3 + 0xc) == 0x3)) {
            pass1_1008_e9a4(param_1 as u16, uVar2, uVar3, param_2, param_3);
        }
    }
    return;
}


pub fn pass1_1008_dfa6(param_1: u32, param_2: i32, param_3: i32, param_4: u16) {
    let puVar1: U32Ptr;
    let extraout_DX: u16;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_4, local_a), (param_1 + 0xa));
    loop {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, param_4);
        if ((extraout_DX | puVar1) == 0x0) {
            return;
        }
        if ((((puVar1 + 0x4) != param_3 as u32) || ((puVar1 + 0x8) != param_2 as u32)) && (((puVar1 + 0x8) != param_3 as u32 || ((puVar1 + 0x4) != param_2 as u32))
        )) == false { break; }
    }
    if ((puVar1 + 0xc) != 0x1) {
        return;
    }
    return;
}


pub fn pass1_1008_e01c(param_1: u32, param_2: u32, param_3: u32) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    (param_1 + 0x16) = param_3;
    (param_1 + 0x1a) = param_2;
    return;
}


pub fn pass1_1008_e038(param_1: u32, param_2: U32Ptr, param_3: U32Ptr) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x16);
    *param_2 = (param_1 + 0x1a);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_e05e(param_1: u32, param_2: u16, param_3: u32, param_4: u32, param_5: u16,
                       param_6: u8)

{
    let iVar1: i16;
    let uVar2: u16;
    let uVar3: u32;
    let local_122: [u8; 112];
    let iStack16: i16;
    let local_e: [u8; 8];
    let lStack6: i32;

    lStack6 = pass1_1008_e8cc(param_5, param_1, param_3, param_4) as i32;
    if (lStack6 != 0x0) {
        uVar3 = pass1_1030_8326();
        // uVar2 = (lStack6 >> 0x10);
        iVar1 = lStack6 as i16;
        (iVar1 + 0xe) = uVar3 as i16;
        (iVar1 + 0x10) = (uVar3 >> 0x10) as i16;
        (iVar1 + 0xc) = param_2 as i16;
    }
    pass1_1008_5784(CONCAT22(param_5, local_e), (param_1 + 0xa));
    iStack16 = 0x0;
    loop {
        lStack6 = pass1_1008_5b12(local_e, param_5);
        if lStack6 == 0x0 {
            // goto
            // LAB_1008_e0d3;
        }
        if ((lStack6 + 0xc) != 0x1) == false { break; }
    }
    iStack16 = 0x1;
//LAB_1008_e0d3:
    if iStack16 == 0x0 {
        struct_1030_e2be(CONCAT22(param_5, local_122), 0x0, 0x0, 0x0, param_5,
                         param_6);
        fn_ptr_1030_835a(ctx.PTR__LOOP_1050_5748, CONCAT22(param_5, local_122));
    }
    return;
}



pub fn
pass1_1008_e10c(param_1: u32,param_2: u32,param_3: u32,param_4: i16,param_5: u16) -> i16

{
let iVar1: i16; let iVar2: i16;
let uVar3: u32;

uVar3 = pass1_1008_e8cc(param_5, param_1, param_2, param_3); if (uVar3 == 0x0) {
return 0x1;
}
iVar1 = (uVar3 + 0xc) as i16; if (( - 0x1 < iVar1) & & (true)) {
if (iVar1 < 0x2) {
return 0x1;
}
if ((0x0 < iVar1 + - 0x1) & & (iVar2 = iVar1 + - 0x3, iVar2 == 0x0 || iVar1 + - 0x2 < 0x1)
) {
pass1_1008_e9a4(param_1 as u16, (param_1 >> 0x10), uVar3, param_4, param_5); return iVar2;
}
}
return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_e164(param_1: u32, param_2: u16, param_3: u8) {
    let puVar1: u32;
    let ppcVar2: u32;
    let uVar5: &mut Struct215;
    let paVar3: &mut Struct215;
    let paVar4: &mut Struct216;
    let puVar5: U32Ptr;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let uVar11: &mut Struct214;
    let paVar9: &mut Struct215;
    let iVar12: &mut Struct213;
    let uVar10: u16;
    let uVar12: u16;
    let uVar13: u32;
    let local_118: [u8; 112];
    let lStack6: i32;
    let iVar1: &mut Struct216;

    // uVar10 = (param_1 >> 0x10);
    uVar11 = param_1;
    lStack6 = pass1_1008_e8cc(param_2, param_1, uVar11.field_0x1a, uVar11.field_0x16) as i32;
    // uVar8 = (lStack6 >> 0x10);
    uVar5 = lStack6;
    puVar5 = (uVar8 | uVar5);
    if (lStack6 == 0x0) {
        pass1_1008_e852(uVar11, uVar10, uVar11.field_0x16, param_2, puVar5 as u16);
        paVar3 = uVar5;
        puVar6 = puVar5;
        pass1_1008_e852(uVar11, uVar10, uVar11.field_0x1a, param_2, puVar5 as u16);
        paVar9 = paVar3;
        puVar7 = puVar6;
        mem_op_1000_179c(0x14, puVar6, 0x1000);
        uVar8 = puVar7 | paVar9;
        if (uVar8 == 0x0) {
            paVar9 = 0x0;
            uVar8 = 0x0;
        } else {
            struct_1008_dc90(CONCAT22(puVar7 as u16, paVar9),
                             CONCAT13(((puVar6 >> 0x8) as u16), CONCAT12(puVar6 as u8, paVar3)), CONCAT22(puVar5 as u16, uVar5));
        }
        lStack6 = CONCAT22(uVar8, paVar9);
        paVar9.field_0xc = 0x1;
        uVar13 = pass1_1030_8326();
        // uVar12 = (lStack6 >> 0x10);
        iVar12 = lStack6;
        iVar12.field_0xe = uVar13;
        iVar12.field_0x10 = (uVar13 >> 0x10);
        puVar1 = uVar11.field_0xa;
        ppcVar2 = (*uVar11.field_0xa + 0x4);
        (**ppcVar2)(0x1030, puVar1, (puVar1 >> 0x10), iVar12, uVar12);
    } else {
        iVar1 = uVar5.field_0xc;
        paVar4 = iVar1 + -0x1;
        if (paVar4 == 0x0) {
            return;
        }
        if (((0x0 < paVar4) && (!SBORROW2(paVar4, 0x1))) && ((iVar1 + -0x2) < 0x2)) {
            uVar5.field_0x12 = 0x1;
        }
        uVar5.field_0xc = 0x1;
    }
    // uVar12 = (lStack6 >> 0x10);
    struct_1030_e2be(CONCAT22(param_2, local_118), 0x1,
                     ((lStack6 + 0x8) as u32), ((lStack6 + 0x4) as u32), param_2,
                     param_3);
    uVar13 = pass1_1030_8326();
    pass1_1030_8372(ctx.PTR__LOOP_1050_5748, uVar13 + 0x1, CONCAT22(param_2, local_118));
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_e2a4(param_1: u32, param_2: u32, param_3: u32) -> u16

{
    let iVar1: i16;
    let iVar2: i16;
    let unaff_SS: u16;
    let mut pcVar3: String;
    let lVar4: i32;
    let uVar5: u32;

    uVar5 = param_2;
    pcVar3 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
    iVar1 = pass1_1000_3d7a(pcVar3, uVar5);
    if ((iVar1 == 0x0) || (iVar1 = pass1_1000_3d7a(param_3, param_2), iVar1 == 0x0)) {
        return 0x0;
    }
    lVar4 = pass1_1008_e8cc(unaff_SS, param_1, param_2, param_3) as i32;
    if (lVar4 != 0x0) {
        iVar1 = (lVar4 + 0xc) as i16;
        iVar2 = iVar1 + -0x1;
        if (iVar2 == 0x0) {
            return 0x2;
        }
        if (iVar2 < 0x1) {
            return 0x0;
        }
        if (SBORROW2(iVar2 as u32, 0x1)) {
            return 0x0;
        }
        if (0x1 < iVar1 + -0x2) {
            return 0x0;
        }
    }
    return 0x1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_e320(param_1: &mut Struct102, param_2: u32, param_3: u32, param_4: u16) {
    let paVar1: &mut Struct103;
    let uVar2: &mut Struct103;
    let uVar3: u16;
    let uVar4: u16;
    let iVar5: &mut Struct102;
    let uVar6: &mut Struct102;
    let mut pcVar5: String;
    let lVar6: i32;
    let uVar7: u32;

    // uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    fn_ptr_1000_17ce(ctx, &iVar5.field_0x1e, 0x1000);
    &iVar5.field_0x1e = 0x0;
    uVar7 = param_2;
    pcVar5 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
    // uVar4 = (pcVar5 >> 0x10);
    uVar2 = pass1_1000_3d7a(pcVar5, uVar7);
    if ((uVar2 != 0x0) && (uVar2 = pass1_1000_3d7a(param_3, param_2), uVar2 != 0x0
    )) {
        lVar6 = pass1_1008_e8cc(param_4, param_1, param_2, param_3);
        // uVar3 = (lVar6 >> 0x10);
        uVar2 = lVar6;
        uVar4 = uVar3 | uVar2;
        if ((uVar4 != 0x0) && (((paVar1 = uVar2.field_0xc, uVar2 = paVar1,
                                 paVar1 != 0x0 && (uVar2 = (&paVar1[-0x1].field_0xc + 0x1),
                                                   uVar2 != 0x0)) && (uVar2 = &paVar1[-0x1].field_0xc, uVar2 != 0x0)))) {
            uVar2 = &paVar1[-0x1].field_0xb;
        }
    }
    load_string_1010_84ac(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
    iVar5.field_0x1e = uVar2;
    iVar5.field_0x20 = uVar4;
    return;
}


pub fn pass1_1008_e3ec(param_1: u32, param_2: U32Ptr, param_3: U32Ptr, param_4: u16) {
    let uVar1: u32;
    let puVar2: u32;
    let ppcVar3: u32;
    let paVar4: &mut Struct219;
    let puVar5: u32;
    let puVar4: &mut Struct219;
    let extraout_DX: U32Ptr;
    let extraout_DX_00: u16;
    let uVar6: u16;
    let uVar7: u16;
    let extraout_DX_01: u16;
    let puVar8: U32Ptr;
    let extraout_DX_02: U32Ptr;
    let extraout_DX_03: u16;
    let uVar9: u16;
    let extraout_DX_04: u16;
    let iVar10: &mut Struct218;
    let uVar10: u16;
    let local_14: astruct_219;
    let uStack12: u16;
    let uStack10: u16;
    let uStack8: u16;
    let uStack6: u16;
    let iStack4: i16;

    // uVar10 = (param_1 >> 0x10);
    iVar10 = param_1;
    // WARNING: Load size is inaccurate
    puVar5 = iVar10.field_0xe;
    puVar8 = (&iVar10.field_0xe + 0x2);
    if ((puVar8 | puVar5) != 0x0) {
        ppcVar3 = *puVar5;
        (**ppcVar3)();
        puVar8 = extraout_DX;
    }
    mem_op_1000_179c(0x18, puVar8, 0x1000);
    if ((puVar8 | puVar5) == 0x0) {
        puVar5 = 0x0;
        uVar6 = 0x0;
    } else {
        struct_op_1030_1cd8(
            CONCAT13(((puVar8 >> 0x8) as u16), CONCAT12(puVar8 as u8, puVar5 as u16),
            ), 0x5, 0x5);
        uVar6 = extraout_DX_00;
    }
    &iVar10.field_0xe = puVar5;
    (&iVar10.field_0xe + 0x2) = uVar6;
    pass1_1028_dc52(
        CONCAT13((param_4 >> 0x8), CONCAT12(param_4 as u8, &local_14)), 0x1,
        0x0, 0x400);
    loop {
        uVar7 = uVar6;
        paVar4 = &local_14;
        pass1_1028_e4ec(CONCAT22(param_4, paVar4));
        if ((uVar7 | paVar4) == 0x0) { break; }
        uVar6 = uVar7 | paVar4;
        if ((paVar4 + 0x40) != 0x8000002) {
            uVar1 = paVar4.field_0x4;
            puVar2 = iVar10.field_0xe;
            ppcVar3 = (*iVar10.field_0xe + 0xc);
            (**ppcVar3)(0x28, puVar2, (puVar2 >> 0x10), uVar1,
                        (uVar1 >> 0x10));
            uVar6 = extraout_DX_01;
        }
    }
    *param_3 = iVar10.field_0xe;
    uVar6 = (&iVar10.field_0x12 + 0x2);
    puVar5 = iVar10.field_0x12;
    puVar8 = (uVar6 | puVar5) as u32;
    if (puVar8 != 0x0) {
        ppcVar3 = *puVar5;
        (**ppcVar3)(0x28, puVar5, uVar6, 0x1);
        puVar8 = extraout_DX_02;
    }
    mem_op_1000_179c(0x18, puVar8, 0x1000);
    if ((puVar8 | puVar5) == 0x0) {
        puVar5 = 0x0;
        uVar9 = 0x0;
    } else {
        struct_op_1030_1cd8(
            CONCAT13(((puVar8 >> 0x8) as u16), CONCAT12(puVar8 as u8, puVar5 as u16),
            ), 0x5, 0x5);
        uVar9 = extraout_DX_03;
    }
    &iVar10.field_0x12 = puVar5;
    (&iVar10.field_0x12 + 0x2) = uVar9;
    uStack12 = uStack8;
    uStack10 = uStack6;
    if (iStack4 != 0x0) {
        uStack12 = 0x1;
        uStack6 = 0x0;
        uStack10 = uStack6;
    }
    loop {
        puVar4 = &local_14;
        pass1_1028_e4ec(CONCAT22(param_4, puVar4));
        if ((uStack6 | puVar4) == 0x0) { break; }
        uVar1 = puVar4.field_0x4;
        puVar2 = iVar10.field_0x12;
        ppcVar3 = (*iVar10.field_0x12 + 0xc);
        (**ppcVar3)(0x28, puVar2, (puVar2 >> 0x10), uVar1,
                    (uVar1 >> 0x10));
        uStack6 = extraout_DX_04;
    }
    *param_2 = iVar10.field_0x12;
    return;
}


pub fn pass1_1008_e5da(param_1: u32, param_2: u32, param_3: HFILE16, param_4: u16) {
    let uVar1: u32;
    let BVar2: bool;
    let puVar3: U32Ptr;
    let extraout_DX: u16;
    let iVar4: i16;
    let uVar5: u16;
    let uVar6: u16;
    let uVar7: u16;
    let local_30: [u32; 0x2];
    let local_28: u32;
    let local_24: [u32; 0x2];
    let local_1c: [u16; 0x3];
    let local_16: [u16; 0x3];
    let uStack16: u32;
    let local_c: [u8; 8];
    let uStack4: u16;

    BVar2 = write_to_file_1008_7cac(param_2, param_4);
    if (BVar2 != 0x0) {
        // uVar5 = (param_1 >> 0x10);
        iVar4 = param_1 as i16;
        if ((iVar4 + 0xa) == 0x0) {
            uStack4 = 0x0;
        } else {
            uVar1 = (iVar4 + 0xa) as u32;
            uStack4 = (uVar1 + 0x8) as u16;
        }
        local_1c[0] = uStack4;
        uVar6 = param_2 as u16;
        // uVar7 = (param_2 >> 0x10);
        BVar2 = write_to_file_1008_7e1c(uVar6, uVar7, local_1c, param_4, 0x2, param_3);
        if (BVar2 != 0x0) {
            pass1_1008_5784(CONCAT22(param_4, local_c), ((iVar4 + 0xa) as u32));
            loop {
                puVar3 = local_c;
                pass1_1008_5b12(puVar3, param_4);
                uStack16 = CONCAT22(extraout_DX, puVar3 as u16);
                if ((extraout_DX | puVar3) == 0x0) {
                    return;
                }
                local_24[0] = (puVar3 + 0x4);
                BVar2 = write_to_file_1008_7e1c(uVar6, uVar7, local_24, param_4, 0x4, param_3);
                if (BVar2 == 0x0) { break; }
                local_28 = (uStack16 + 0x8);
                BVar2 = write_to_file_1008_7e1c(uVar6, uVar7, &local_28, param_4, 0x4, param_3);
                if (BVar2 == 0x0) { break; }
                local_16[0] = (uStack16 + 0xc) as u16;
                BVar2 = write_to_file_1008_7e1c(uVar6, uVar7, local_16, param_4, 0x2, param_3);
                if (BVar2 == 0x0) { break; }
                local_30[0] = (uStack16 + 0xe);
                BVar2 = write_to_file_1008_7e1c(uVar6, uVar7, local_30, param_4, 0x4, param_3);
                if (BVar2 == 0x0) { break; }
                local_16[0] = (uStack16 + 0x12) as u16;
                BVar2 = write_to_file_1008_7e1c(uVar6, uVar7, local_16, param_4, 0x2, param_3);
                if (BVar2 != 0x0) == false { break; }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return;
}


pub fn pass1_1008_e852(param_1: u16, param_2: u16, param_3: u32, param_4: u16, param_5: u16) {
    let puVar1: U32Ptr;
    let iVar2: i16;
    let mut pcVar3: String;
    let local_14: [u8; 12];

    pass1_1028_dc52(CONCAT22(param_4, local_14), 0x1, 0x0, 0x400);
    loop {
        puVar1 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4, puVar1 as u16));
        if ((param_5 | puVar1) == 0x0) {
            return;
        }
        pcVar3 = load_string_1038_4d28(CONCAT22(param_5, puVar1 as u16));
        // param_5 = (pcVar3 >> 0x10);
        iVar2 = pass1_1000_3d7a(param_3, pcVar3 & 0xffff | param_5 << 0x10);
        if (iVar2 != 0x0) { break; }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_e8cc(param_1: u16, param_2: u32, param_3: u32, param_4: u32) -> u32

{
    let uVar1: u32;
    let uVar2: u16;
    let uVar3: u16;
    let iVar4: i16;
    let uVar5: u16;
    let uVar6: u16;
    let lVar7: i32;
    let mut pcVar8: String;
    let mut pcVar9: String;
    let uStack22: u32;
    let uStack18: u32;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_1, local_a), (param_2 + 0xa));
    loop {
        lVar7 = pass1_1008_5b12(local_a, param_1);
// uVar5 = (lVar7 >> 0x10);
        uVar2 = lVar7 as u16;
        uVar6 = uVar5 | uVar2;
        if (lVar7 == 0x0) {
            return 0x0;
        }
        uVar1 = (uVar2 + 0x4) as u32;
        uVar3 = uVar2;
        pass1_1028_e1ec(ctx.PTR__LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        uStack18 = CONCAT22(uVar6, uVar3);
        uVar1 = (uVar2 + 0x8) as u32;
        pass1_1028_e1ec(ctx.PTR__LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        uStack22 = CONCAT22(uVar6, uVar3);
        pcVar8 = pass1_1038_4d28(uStack18);
        pcVar9 = pass1_1038_4d28(uStack22);
        iVar4 = pass1_1000_3d7a(param_4, pcVar8);
        if ((iVar4 == 0x0) & &(iVar4 = pass1_1000_3d7a(param_3, pcVar9), iVar4 == 0x0)) {
            break;
        }
        iVar4 = pass1_1000_3d7a(param_3, pcVar8);
        if ((iVar4 == 0x0) & &(iVar4 = pass1_1000_3d7a(param_4, pcVar9), iVar4 == 0x0)) {
            return lVar7 as u32;
        }
    }
    return lVar7 as u32;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_e9a4(param_1: u16, param_2: u16, param_3: u32, param_4: i16, param_5: u16) {
    let puVar1: U32Ptr;
    let uVar2: u16;
    let uVar3: u16;
    let puVar4: U32Ptr;
    let iVar5: i16;
    let uVar6: u16;
    let uVar7: u32;
    let iStack20: i16;
    let uStack16: u32;
    let uStack6: u32;

    uVar7 = pass1_1030_8326();
    // uVar6 = (param_3 >> 0x10);
    iVar5 = param_3 as i16;
    puVar1 = (iVar5 + 0xe) as u32;
    uVar2 = uVar7 - *puVar1;
    puVar4 = (((uVar7 >> 0x10) - (iVar5 + 0x10)) - (uVar7 < *puVar1));
    uStack6 = CONCAT22(puVar4 as u16, uVar2);
    mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_5, puVar4, param_4);
    uStack16 = 0x0;
    if (ctx.PTR_LOOP_1050_13ae < 0x1) || (SBORROW2(ctx.PTR_LOOP_1050_13ae, 0x1)) > 0 {}
//   TODO: goto LAB_1008_ea2b;
    if ctx.PTR_LOOP_1050_13ae == &ctx.PTR_LOOP_1050_0002 || (ctx.PTR_LOOP_1050_13ae + -0x1) < 0x1 {
        if (iVar5 + 0x12) == 0x0 {
//LAB_1008_ea20:
            uVar3 = 0x1e;
        } else {
            uVar3 = 0xa;
        }
    } else {
        if (ctx.PTR_LOOP_1050_13ae == (&ctx.PTR_LOOP_1050_0002 + 0x1)) {
            if ((iVar5 + 0x12) == 0x0) {
                uVar3 = 0x28;
            } else {
                uVar3 = 0x14;
            }
        } else {
            if (ctx.PTR_LOOP_1050_13ae != &DAT_1050_0004) {
                // goto
                // LAB_1008_ea2b;
            }
            if ((iVar5 + 0x12) != 0x0) {
                // goto
                // LAB_1008_ea20;
            }
            uVar3 = 0x32;
        }
    }
    uStack16 = uVar3 as u32;
//LAB_1008_ea2b:
    if (uStack16 < uStack6) {
        pass1_1008_612e(0x1, 0x64, uVar2);
        iStack20 = 0x0;
        iVar5 = (iVar5 + 0xc);
        if (iVar5 == 0x2) {
            iStack20 = 0x32;
        } else {
            if (iVar5 == 0x3) {
                iStack20 = 0x19;
            }
        }
        if (uStack6 < iStack20 as u32) {
            return;
        }
    }
    return;
}


pub fn pass1_1008_ea86(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1008_ddca(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_eabc(param_1: &mut Struct19, param_2: &mut Struct19, param_3: u16) -> u16

{
    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 0xa) = 0x0;
    struct_1008::clear_struct_1008_3e38(CONCAT22(param_2, param_1 + 0xc));
    CONCAT22(param_2, param_1) = 0xeb1a;
    (param_1 + 0x2) = 0x1008;
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1008_eaf4(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_1d80(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_eb2a(param_1: &mut Struct19, param_2: &mut Struct19, param_3: u16) {
    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 0xa) = 0x0;
    (param_1 + 0xc) = 0x0;
    CONCAT22(param_2, param_1) = 0xec00;
    (param_1 + 0x2) = 0x1008;
    return;
}


pub fn pass1_1008_eb5c(param_1: u16, param_2: u16, param_3: i16) -> u32

{
    return CONCAT22(0x1050, (param_3 * 0x10 + 0xd0e) as u16);
}


pub fn pass1_1008_eb6e() -> u16

{
    return 0x5;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_eb74(param_1: u32, param_2: i16, param_3: U32Ptr, param_4: i16, param_5: u16) {
    (param_1 + 0xa) = param_2 as u32;
    if (param_2 != 0x0) {
        mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_5, param_3, param_4);
        pass1_1010_c312();
    }
    return;
}


pub fn pass1_1008_ebda(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_1d80(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_ec10(param_1: &mut Struct19, param_2: u16, param_3: u16) -> u16

{
    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 0xa) = 0x0;
    CONCAT22(param_2, param_1) = 0xec62;
    (param_1 + 0x2) = 0x1008;
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1008_ec3c(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_1d80(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}


pub fn pass1_1008_ec94(param_1: U32Ptr) {
    *param_1 = 0xefc4;
    (param_1 + 0x2) = 0x1008;
    pass1_1010_3880(param_1);
    return;
}


pub fn pass1_1008_ed00(param_1: U32Ptr, param_2: u16) {
    *param_1 = 0xef9c;
    (param_1 + 0x2) = 0x1008;
    pass1_1010_2db2(param_1, param_2);
    return;
}


pub fn pass1_1008_ed62(param_1: u32, param_2: i16) {
    let iVar1: i16;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1 as i16;
    (iVar1 + 0x16) = param_2 * 0x8 + 0xd5e;
    (iVar1 + 0x18) = ctx.data_seg;
    (iVar1 + 0x12) = (param_2 * 0x8 + 0xd64);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_ed8a(param_1: U32Ptr, param_2: u16, param_3: i16, param_4: i16, param_5: i16,
                       param_6: i16, param_7: u16)

{
    let ppcVar1: u32;
    let cVar2: u8;
    let uVar3: u16;
    let uVar4: u16;
    let bVar5: bool;
    let uVar6: u32;
    let uVar7: u32;

    if (0x0 < param_4) {
        if (ctx.PTR__LOOP_1050_0df6 == 0x0) {
            ppcVar1 = (*param_1 + 0x18);
            uVar6 = (**ppcVar1)();
            ctx._PTR_LOOP_1050_0df6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, uVar6, param_7,
                                                      (uVar6 >> 0x10), param_6);
        }
        uVar6 = (param_1 + 0xc);
        uVar7 = pass1_1010_2e02(ctx.PTR__LOOP_1050_0df6, ((uVar6 + 0x12) as i16));
        uVar3 = param_2 + 0x1;
        uVar4 = param_3 + (0xfffe < param_2);
// TODO: refactor for loop
        // for (cVar2 = (param_4 + -0x1) * '\x04'; cVar2 != '\0'; cVar2 += -0x1) {
        //   bVar5 = CARRY2(uVar3,uVar3);
        //   uVar3 *= 0x2;
        //   uVar4 = uVar4 * 0x2 + bVar5;
        // }
        pass1_1010_2e30(ctx.PTR__LOOP_1050_0df6, uVar3 | uVar7,
                        uVar4 | (uVar7 >> 0x10), (param_5 * 0x8 + 0xd64));
    }
    return;
}


pub fn pass1_1008_ee14(param_1: u32, param_2: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let i_var3: i16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let local_1c: [u8; 0x1a];

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1 as i16;
    if (i_var3 + 0x56) == 0x0 {
        pu_var5 = struct_1008_ec72(CONCAT22(param_2, local_1c));
        // u_var2 = (pu_var5 >> 0x10);
        pu_var1 = local_1c;
        pass1_1010_398e(CONCAT22(param_2, pu_var1 as u16), 0x0, 0x0, 0x0, pu_var1 as u16);
        (i_var3 + 0x56) = pu_var1 as i16;
        (i_var3 + 0x58) = u_var2 as i16;
        pass1_1008_ec94(CONCAT22(param_2, local_1c));
    }
    return;
}

