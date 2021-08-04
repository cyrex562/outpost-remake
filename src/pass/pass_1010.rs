use std::default::default;

use crate::bad::bad_1010_bf08;
use crate::cleanup::{clenaup_win_ui_1018_4d22, destroy_window_1010_7b26, unk_destroy_win_op_1010_2fa0};
use crate::defines::{Struct11, Struct18, Struct19, Struct20, Struct27, Struct65, Struct79, Struct87, U32Ptr};
use crate::file::file_1008::{file_1008_6414, read_file_1008_7c6e, read_file_1008_7dee, write_to_file_1008_7cac};
use crate::fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708};
use crate::fn_ptr::fn_ptr_1028::fn_ptr_1030_835a;
use crate::global::AppContext;
use crate::mem_1000::{mem_op_1000_160a, mem_op_1000_179c};
use crate::mem_1008::memcpy_op_1008_676e;
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{pass1_1000_472c, pass1_1000_48a8, pass1_1000_4906, pass1_1000_4aea, pass1_1000_5586};
use crate::pass::pass_1008::{pass1_1008_3e54, pass1_1008_3e94, pass1_1008_3eb4, pass1_1008_3f62, pass1_1008_4544, pass1_1008_4772, pass1_1008_5784, pass1_1008_5b12, pass1_1008_64a2, pass1_1008_64c8, pass1_1008_6562, pass1_1008_7c2a, pass1_1008_92b2};
use crate::pass::pass_1018::{pass1_1018_04f2, pass1_1018_209c, pass1_1018_4b78, pass1_1018_4dce};
use crate::pass::pass_1020::{pass1_1020_bae6, string_1020_c0d8, string_op_1020_c222};
use crate::pass::pass_1028::{pass1_1028_121e, pass1_1028_4faa, pass1_1028_b58e, pass1_1028_dc52, pass1_1028_e0a0, pass1_1028_e1ec, pass1_1028_e4ec};
use crate::pass::pass_1030::{fn_ptr_1030_84d0, pass1_1030_25d8, pass1_1030_301a, pass1_1030_38f2, pass1_1030_532e, pass1_1030_6ddc, pass1_1030_70f4, pass1_1030_7c28, pass1_1030_7f5a, pass1_1030_7f98, pass1_1030_809c, pass1_1030_8326, pass1_1030_8344};
use crate::pass::pass_1038::{load_string_1038_4d28, pass1_1038_4e78, pass1_1038_5050, pass1_1038_50e0};
use crate::resources::free_rsrc_1010_4b3e;
use crate::string::string_1000::{string_1000_3cea, string_1000_3d3e, string_1000_475e};
use crate::string::string_1008::str_op_1008_60e8;
use crate::string::string_1010::{load_string_1010_84ac, string_1010_5286};
use crate::string::string_1020::string_op_1020_c2f8;
use crate::string::string_1040::string_1040_a626;
use crate::struct_ops::struct_1008::{clear_struct_1008_3e38, pass1_1008_c6ae, pass1_1008_c6fa, set_struct_1008_574a};
use crate::struct_ops::struct_1010::{struct_1010_383a, struct_1010_38f8, struct_1010_dd5e};
use crate::struct_ops::struct_1018::struct_op_1018_4cda;
use crate::struct_ops::struct_1030::{struct_op_1030_1cd8, struct_op_1030_73a8};
use crate::switch_ops::switch_1008::{switch_1008_72bc, switch_1008_73ea};
use crate::switch_ops::switch_1010::switch_1010_6646;
use crate::sys_api::{get_sys_metrics_1018_4b1e, set_err_mode_1010_8b14, write_private_profile_str_1010_5b10};
use crate::ui::ui_1010::{msg_box_op_1010_8bb4, send_msg_1010_7c42};
use crate::ui::ui_1040::mov_update_win_1040_93aa;
use crate::util::{CARRY2, CONCAT12, CONCAT13, CONCAT22, SUB42, ZEXT24};
use crate::win_struct::{HINSTANCE16, SEGPTR, WNDCLASS16};

pub unsafe fn pass1_1010_0000(param_1: &mut Struct645, param_3: u16, param_4: u16, unaff_di: &mut Struct19) -> u32

{
    let pa_var1: &mut Struct79;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let u_var6: u16;

    pa_var1 = struct_op_1010_1d48(param_1, param_3);
    param_1.field_0xa = 0x0;
    param_1.field_0xc = 0x0;
    param_1.field_0x0 = 0x2c8;
    param_1.field_0x2 = 0x1010;
    pu_var5 = &param_1.field_0xa;
    pu_var3 = &param_1.field_0xc;
    u_var4 = param_2;
    u_var6 = param_2;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x48, param_4,
                              (pa_var1 >> 0x10), unaff_di);
    pass1_1008_3e94((pu_var2 & 0xffff0000 | (pu_var2 + 0xe)),
                    CONCAT22(u_var4, pu_var3), CONCAT22(u_var6, pu_var5));
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1010_0052(param_1: &mut Struct18, param_2: u16) {
    param_1.field_0x0 = 0x2c8;
    (param_1.field_0x2) = 0x1010;
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1010_01f8(param_1: u32, param_2: u32, param_3: i16) {
    let i_var1: i16;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u16;

    i_var2 = (param_3 * 0x4 + 0xe02) * 0x4;
    i_var1 = (i_var2 + 0xdfc);
    // u_var3 = (param_1 >> 0x10);
    // u_var4 = (param_2 >> 0x10);
    (param_2 + 0x6) = (param_3 * 0x4 + 0xe04) * 0x28 + (i_var2 + 0xdfa) + (param_1 + 0xa);
    (param_2 + 0x8) = (param_1 + 0xc) + i_var1;
    return;
}


pub fn pass1_1010_02a2(param_1: &mut Struct18, param_2: u8, param_3: u16) {
    pass1_1010_0052(param_1, param_3);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
}


pub fn pass1_1010_0350(param_1: &mut Struct18, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;

    param_1.field_0x0 = 0xe98;
    param_1.field_0x2 = 0x1010;
    pu_var1 = param_1.field_0xa;
    u_var2 = param_1.field_0xc;
    if (u_var2 | pu_var1) != 0x0 {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1010_038e(param_1: &mut Struct19, param_2: i16, param_3: &mut WNDCLASS16) {
    let bVar1: bool;
    let i_var2: &mut Struct707;
    let u_var2: u16;

    bVar1 = false;
    i_var2 = param_1;
    // u_var2 = (param_1 >> 0x10);
    if (param_2 != 0x0) {
        if (i_var2.field_0x18 == 0x0) {
            i_var2.field_0x12 = ctx.DAT_1050_0e28;
            i_var2.field_0x14 = ctx.PTR_LOOP_1050_0e30;
            i_var2.field_0x16 = ctx.PTR_LOOP_1050_0ea8;
            ctx.DAT_1050_0e28 = 0x0;
            ctx.PTR_LOOP_1050_0e30 = 0x0;
            ctx.PTR_LOOP_1050_0ea8 = 0x0;
            i_var2.field_0x18 = 0x1;
            bVar1 = true;
//       TODO: goto LAB_1010_0404;
        }
    }
    if (param_2 == 0x0) {
        if (i_var2.field_0x18 != 0x0) {
            ctx.DAT_1050_0e28 = i_var2.field_0x12;
            ctx.PTR_LOOP_1050_0e30 = i_var2.field_0x14;
            ctx.PTR_LOOP_1050_0ea8 = i_var2.field_0x16;
            i_var2.field_0x18 = 0x0;
            bVar1 = true;
        }
    }
//LAB_1010_0404:
    if (bVar1) {
        pass1_1010_1f62(param_3, param_1, 0x3);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_041a() -> bool

{
    let u_var1: u32;

    u_var1 = pass1_1030_8326();
    if (((u_var1 >> 0x10) == 0x0) && (u_var1 < 0x64)) {
        return 0x0;
    }
    return 0x1;
}


pub fn pass1_1010_043a(param_1: u32, param_2: i32, param_3: i16, param_4: u16) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let pu_var3: &mut Struct225;
    let extraout_dx: u16;
    let u_var3: u16;
    let i_var4: &mut Struct226;
    let iVar5: &mut Struct227;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let puStack18: U32Ptr;
    let puStack14: U32Ptr;
    let local_a: [u8; 8];

    i_var4 = param_1;
    // u_var4 = (param_1 >> 0x10);
    if (param_3 == 0xc) {
        if (i_var4.field_0xe != 0x0) {
            return;
        }
        i_var4.field_0xe = 0x1;
    } else {
        if (param_3 == 0xd) {
            if (i_var4.field_0x10 != 0x0) {
                return;
            }
            i_var4.field_0x10 = 0x1;
        } else {
            if (param_3 == 0x12) {
                return;
            }
        }
    }
    pass1_1010_089e(param_4, param_1, 0x1, 0x6);
    pass1_1008_5784(CONCAT22(param_4, local_a), i_var4.field_0xa);
    loop {
        pu_var3 = local_a;
        pass1_1008_5b12(pu_var3, param_4);
        u_var3 = extraout_dx | pu_var3;
        if (u_var3 == 0x0) {
            u_var6 = 0xa;
            mem_op_1000_179c(ctx, 0xa, 0x0, 0x1000);
            puStack18 = CONCAT22(u_var3, pu_var3);
            if ((u_var3 | pu_var3) == 0x0) {
                puStack14 = 0x0;
            } else {
                *puStack18 = 0x389a;
                (&pu_var3.field_0x0 + 0x2) = 0x1008;
                *puStack18 = 0xea8;
                (&pu_var3.field_0x0 + 0x2) = 0x1010;
                puStack14 = puStack18;
            }
            // u_var5 = (puStack14 >> 0x10);
            iVar5 = puStack14;
            iVar5.field_0x4 = param_3;
            iVar5.field_0x6 = param_2;
            pu_var1 = i_var4.field_0xa;
            ppcVar2 = (*i_var4.field_0xa + 0x8);
            (**ppcVar2)(0x1000, pu_var1, (pu_var1 >> 0x10), iVar5, u_var5, u_var6);
            return;
        }
        if ((pu_var3.field_0x4 != param_3) || (pu_var3.field_0x6 != param_2)) == false { break; }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_0538(param_1: u32, param_2: &mut String, param_3: &mut String, param_4: u16, param_5: u16) {
    let i_var1: i16;
    let u_var2: u32;
    let ppc_var3: u32;
    let u_var4: u16;
    let iVar5: i16;
    let mut pcVar6: String;
    let puVar7: U32Ptr;
    let extraout_dx: u16;
    let puVar8: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let uVar12: u32;
    let puStack6: u32;

    u_var4 = 0x0;
    *param_3 = 0x0;
    *param_2 = 0x0;
    // u_var10 = (param_1 >> 0x10);
    uVar9 = param_1;
    uVar12 = (uVar9 + 0xa);
    ppc_var3 = ((uVar9 + 0xa) + 0x10);
    (**ppc_var3)();
    puStack6 = CONCAT22(extraout_dx, u_var4);
    puVar8 = (extraout_dx | u_var4);
    if (puVar8 == 0x0) {
        return;
    }
    i_var1 = (u_var4 + 0x4);
    u_var2 = (u_var4 + 0x6);
    if ((extraout_dx | u_var4) != 0x0) {
        ppc_var3 = *puStack6;
        (**ppc_var3)(param_4, u_var4, extraout_dx, 0x1, uVar12);
        puVar8 = extraout_DX_00;
    }
    uVar12 = (uVar9 + 0xa);
    if ((uVar12 + 0x8) == 0x0) {
        pass1_1010_089e(param_5, param_1, 0x0, 0x6);
        pass1_1010_1f62(param_5, param_1, 0x3);
    }
    iVar5 = i_var1 + 0x757;
    load_string_1010_84ac(ctx.PTR_LOOP_1050_14cc, param_4);
    param_3 = iVar5;
    (param_3 + 0x2) = puVar8;
    while (pcVar6 = pass1_1000_472c(*param_3, '%'),
           (puVar8 | pcVar6) != 0x0) {
        pass1_1010_09b4(uVar9, u_var10, CONCAT22(puVar8, pcVar6), param_3, u_var2, puVar8,
                        param_5);
    }
    if (0x1e < i_var1 - 0x1) {
        // goto
        // LAB_1010_0850;
    }
    // u_var11 = (param_2 >> 0x10);
    match(i_var1)
    {
        0x1 =>{
//     TODO: goto LAB_1010_0619;
}
        0x2 =>{
//     TODO: goto LAB_1010_0619;
}
        0x3 => {}
        0x4 =>{
//     TODO: goto LAB_1010_0619;
}
        0x5 =>{
//     TODO: goto LAB_1010_0619;
}
        0x6 => {}
        0x7 =>{}
//     TODO: goto LAB_1010_0619;
        0x8 =>{}
//     TODO: goto LAB_1010_0619;
        0x9 => {}
        0xa =>{}
//     TODO: goto LAB_1010_0619;
        0xb =>{}
//     TODO: goto LAB_1010_0619;
        0xc => {}
        0xd =>{}
//     TODO: goto LAB_1010_0619;
        0xe => {}
        0xf =>{}
//     TODO: goto LAB_1010_0619;
        0x10 => {}
        0x11 => {}
        0x12 => {}
        0x13 => {}
        0x14 => {}
        0x15 => {}
        0x16 =>{
//LAB_1010_0619:
        puVar7 = pass1_1008_5fd8(param_5, puVar8);}
//     TODO: goto LAB_1010_0621;
        0x17 => {}
        0x18 => {}
        0x19 => {}
        0x1f => {
//LAB_1010_0785:
        puVar7 = pass1_1008_5fd8(param_5, puVar8);
        }
//     TODO: goto LAB_1010_0621;
        0x1a =>{}
//     TODO: goto LAB_1010_0785;
        0x1b =>{}
//     TODO: goto LAB_1010_0785;
        0x1c => {}
        0x1d => {}
        0x1e => {puVar7 = pass1_1008_5fd8(param_5, puVar8);
        param_2 = puVar7;
        (param_2 + 0x2) = puVar8;}
//     TODO: goto LAB_1010_0785;
            _ => {}
    }
    puVar7 = pass1_1008_5fd8(param_5, puVar8);
//LAB_1010_0621:
    param_2 = puVar7;
    (param_2 + 0x2) = puVar8;
//LAB_1010_0850:
    while (pcVar6 = pass1_1000_472c(*param_2, '%'),
           (puVar8 | pcVar6) != 0x0) {
        pass1_1010_09b4(uVar9, u_var10, CONCAT22(puVar8, pcVar6), param_2, u_var2, puVar8,
                        param_5);
    }
    return;
}


pub fn pass1_1010_0886() -> u16

{
    return 0xa;
}


pub fn pass1_1010_088c() -> u16

{
    return 0x3;
}


pub fn pass1_1010_0892() -> u16

{
    return 0x3;
}


pub fn pass1_1010_0898() -> u16

{
    return 0x3;
}


pub fn pass1_1010_089e(param_1: u16, param_2: u32, param_3: u16, param_4: i16) {
    ((param_4 + -0x1) * 0x8 + 0xe28) = param_3;
    pass1_1010_1f62(param_1, param_2, 0x3);
    return;
}


pub fn pass1_1010_08c0(param_1: u32, param_2: u16, param_3: i16, param_4: u16) {
    ((param_3 + -0x1) * 0x8 + 0xea8) = param_2;
    pass1_1010_1f62(param_4, param_1, 0x3);
    return;
}


pub fn pass1_1010_08e2(param_1: u16, param_2: u16, param_3: i16) -> u32

{
    if (ctx.PTR_LOOP_1050_4fe8 != 0x0) {
        ctx.DAT_1050_0e28 = 0x0;
        ctx.PTR_LOOP_1050_0e30 = 0x0;
        ctx.PTR_LOOP_1050_0e38 = 0x0;
        ctx.PTR_LOOP_1050_0e40 = 0x0;
        ctx.PTR_LOOP_1050_0e48 = 0x0;
        ctx.DAT_1050_0e58 = 0x0;
        ctx.DAT_1050_0e60 = 0x0;
        ctx.PTR_LOOP_1050_0e70 = 0x0;
    }
    return CONCAT22(0x1050, (param_3 + -0x1) * 0x8 + 0xe22);
}


pub fn pass1_1010_091e(param_1: u16, param_2: u16, param_3: i16) -> u32

{
    return CONCAT22(0x1050, (param_3 + -0x1) * 0x8 + 0xe72);
}


pub fn pass1_1010_0932(param_1: u16, param_2: u16, param_3: i16) -> u32

{
    return CONCAT22(0x1050, (param_3 + -0x1) * 0x8 + 0xe8a);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn 
pass1_1010_0946(param_1: u16,param_2: u16,param_3: i16,param_4: * mut u8,
param_5: i16,param_6: u16) -> u32

{
let pu_var1: * mut u16; let lVar2: i32; let lVar3: i32;

ctx.PTR_LOOP_1050_0ea8 = 0x0; lVar3 = 0x4000001; lVar2 = 0x4000002; pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3b, param_6, param_4, param_5); pass1_1008_dfa6(pu_var1, lVar2, lVar3, param_6); if (pu_var1 != 0x0) {
pass1_1030_8344(ctx.PTR_LOOP_1050_5748,
(ctx.PTR_LOOP_1050_5748 >> 0x10), 0x4000002); if ((pu_var1 + 0x200) == 0x8000002) {
ctx.PTR_LOOP_1050_0ea8 = ( & ctx.PTR_LOOP_1050_0000 + 0x1);
}
}
return CONCAT22(0x1050, (param_3 + - 0x1) * 0x8 + 0xea2);
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_09b4(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: &String, param_5: u32,
                       param_6: U32Ptr, param_7: u16)

{
    let bVar1: u8;
    let bVar2: bool;
    let bVar3: bool;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let unaff_DI: i16;
    let puVar8: U32Ptr;
    let mut pcStack18: String;
    let uStack10: u32;

    bVar3 = false;
    bVar2 = false;
    bVar1 = (param_3 + 0x1);
    if (bVar1 == 0x70) {
//LAB_1010_0a06:
        bVar3 = false;
        bVar2 = true;
    } else {
        if (bVar1 < 0x71) {
            if (bVar1 != 0x43) {
                if (bVar1 == 0x50) {
                    // goto
                    // LAB_1010_0a06;
                }
                if (bVar1 != 0x63) {
                    // goto
                    // LAB_1010_09db;
                }
            }
            bVar3 = true;
            bVar2 = false;
        }
    }
//LAB_1010_09db:
    u_var4 = 0x0;
    uStack10 = 0x0;
    if (bVar2) {
        puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_7, param_6, unaff_DI);
        // u_var4 = (puVar8 >> 0x10);
        uStack10 = CONCAT22((puVar8 + 0x6e),
                            (puVar8 + 0x6c));
    } else {
        if (!bVar3) {
            // goto
            // LAB_1010_0a36;
        }
        pass1_1030_8344(
            ctx, ctx.PTR_LOOP_1050_5748, param_5);
        uStack10 = load_string_1038_4d28(CONCAT22(param_6, u_var4));
    }
    // param_6 = (uStack10 >> 0x10);
//LAB_1010_0a36:
    if ((uStack10._2_2_ | uStack10) != 0x0) {
        u_var5 = str_op_1000_3da4(*param_4);
        u_var6 = str_op_1000_3da4(uStack10);
        uVar7 = u_var6 + 0xa + u_var5;
        mem_op_1000_179c(uVar7, param_6, 0x1000);
        pcStack18 = CONCAT22(param_6, uVar7);
        *param_3 = '\0';
        string_1000_3d3e(CONCAT22(param_6, uVar7), *param_4);
        string_1000_3cea(CONCAT22(param_6, uVar7), uStack10);
        string_1000_3cea(CONCAT22(param_6, uVar7),
                         param_3 & 0xffff0000 | (param_3 + 0x2));
        fn_ptr_1000_17ce(ctx, *param_4, 0x1000);
        *param_4 = pcStack18;
    }
    return;
}


pub fn pass1_1010_0ad2(param_1: u32, param_2: u32, param_3: u16) {
    let u_var1: u32;
    let BVar2: bool;
    let pu_var3: U32Ptr;
    let extraout_dx: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let local_2a: [u32; 0x2];
    let local_22: [u16; 0x2];
    let local_1e: [u16; 0x3];
    let local_18: [u16; 0x3];
    let uStack18: u32;
    let local_e: [u8; 8];
    let uStack6: u16;
    let i_stack4: i16;

    BVar2 = write_to_file_1008_7cac(param_2, param_3);
    if (BVar2 == 0x0) {
        return;
    }
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0xa) == 0x0) {
        uStack6 = 0x0;
    } else {
        u_var1 = (i_var4 + 0xa);
        uStack6 = (u_var1 + 0x8);
    }
    local_1e[0] = uStack6;
    u_var6 = param_2;
    // uVar7 = (param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c(u_var6, uVar7, local_1e, param_3, 0x2, 0x1008);
    if (BVar2 != 0x0) {
        pass1_1008_5784(CONCAT22(param_3, local_e), (i_var4 + 0xa));
        loop {
            pu_var3 = local_e;
            pass1_1008_5b12(pu_var3, param_3);
            uStack18 = CONCAT22(extraout_dx, pu_var3);
            if ((extraout_dx | pu_var3) == 0x0) {
                local_22[0] = (i_var4 + 0xe);
                BVar2 = write_to_file_1008_7e1c(u_var6, uVar7, local_22, param_3, 0x2, 0x1008);
                if (BVar2 == 0x0) {
                    ctx.PTR_LOOP_1050_0310 = 0x6d0;
                    return;
                }
                local_22[0] = (i_var4 + 0x10);
                BVar2 = write_to_file_1008_7e1c(u_var6, uVar7, local_22, param_3, 0x2, 0x1008);
                if (BVar2 == 0x0) {
                    ctx.PTR_LOOP_1050_0310 = 0x6d0;
                    return;
                }
                if ((i_var4 + 0x18) != 0x0) {
                    ctx.DAT_1050_0e28 = (i_var4 + 0x12);
                    ctx.PTR_LOOP_1050_0e30 = (i_var4 + 0x14);
                    ctx.PTR_LOOP_1050_0ea8 = (i_var4 + 0x16);
                }
                i_stack4 = 0x0;
                loop {
                    if (0x9 < i_stack4) {
                        i_stack4 = 0x0;
                        loop {
                            if (0x2 < i_stack4) {
                                if ((i_var4 + 0x18) != 0x0) {
                                    ctx.DAT_1050_0e28 = 0x0;
                                    ctx.PTR_LOOP_1050_0e30 = 0x0;
                                    ctx.PTR_LOOP_1050_0ea8 = 0x0;
                                }
                                return;
                            }
                            local_1e[0] = (i_stack4 * 0x8 + 0xea8);
                            BVar2 = write_to_file_1008_7e1c(u_var6, uVar7, local_1e, param_3, 0x2, 0x1008);
                            if (BVar2 == 0x0) { break; }
                            i_stack4 += 0x1;
                        }
                        ctx.PTR_LOOP_1050_0310 = 0x6d0;
                        return;
                    }
                    local_1e[0] = (i_stack4 * 0x8 + 0xe28);
                    BVar2 = write_to_file_1008_7e1c(u_var6, uVar7, local_1e, param_3, 0x2, 0x1008);
                    if (BVar2 == 0x0) { break; }
                    i_stack4 += 0x1;
                }
                ctx.PTR_LOOP_1050_0310 = 0x6d0;
                return;
            }
            local_18[0] = (pu_var3 + 0x4);
            BVar2 = write_to_file_1008_7e1c(u_var6, uVar7, local_18, param_3, 0x2, 0x1008);
            if (BVar2 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d0;
                return;
            }
            local_2a[0] = (uStack18 + 0x6);
            BVar2 = write_to_file_1008_7e1c(u_var6, uVar7, local_2a, param_3, 0x4, 0x1008);
            if (BVar2 != 0x0) == false { break; }
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
    return;
}


pub fn pass1_1010_0e46(param_1: &mut Struct18, param_2: u8, param_3: u16) {
    pass1_1010_0350(param_1, param_3);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
}


pub fn pass1_1010_0e6c(param_1: U32Ptr, param_2: u8) -> u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}



pub fn
pass1_1010_0eac(param_1: * mut u8,param_2: * mut u8,param_3: u16,param_4: * mut u8,param_5: u16
) -> u32

{
struct_op_1018_4cda(param_1, param_2, param_3); CONCAT22(param_2, param_1) = 0xf0c; (param_1 + 0x2) = 0x1010; ctx.PTR_LOOP_1050_4230 = param_1; ctx.PTR_LOOP_1050_4232 = param_2; pass1_1018_4dce(CONCAT22(param_2, param_1), 0xff, param_4, param_5); return CONCAT22(param_2, param_1);
}



pub fn  pass1_1010_0ee6(param_1: & mut Struct11,param_2: u8) -> &mut Struct11

{
clenaup_win_ui_1018_4d22(param_1, 0x1018); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_0f24(param_1: &mut Struct79, param_2: &mut Struct79, param_3: u16, param_4: U32Ptr,
                       param_5: u16)

{
    let unaff_DI: i16;
    let pu_var1: U32Ptr;

    struct_1010_2cd2(param_1, param_2, param_3, param_5);
    (&param_1[0x9].field_0x4 + 0x2) = 0x0;
    (param_1 + 0xa) = 0x0;
    &param_1[0xa].field_0x4 = 0x0;
    (&param_1[0xa].field_0x4 + 0x2) = 0x0;
    CONCAT22(param_2, param_1) = (s_648_bmp_1050_1919 + 0x1);
    param_1.field_0x2 = 0x1010;
    pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_5, param_4, unaff_DI);
    (&param_1[0xa].field_0x4 + 0x2) = pu_var1;
    param_1[0xa].field_0x8 = (pu_var1 >> 0x10);
    return;
}


pub fn pass1_1010_0f76(param_1: U32Ptr, param_2: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = (s_648_bmp_1050_1919 + 0x1);
    (param_1 + 0x2) = 0x1010;
    pass1_1010_17c0(param_1 & 0xffff | u_var1 << 0x10);
    pass1_1010_2db2(param_1, param_2);
    return;
}


pub fn pass1_1010_1146(param_1: u32, param_2: u16, param_3: i16, param_4: u16) {
    let u_var1: u32;
    let u_var2: u16;

    ctx.DAT_1050_0ecc = param_2;
    // u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x66);
    pass1_1000_4aea((param_1 + 0x64), u_var1, (u_var1 >> 0x10),
                    0x4, (s_dibtext_bmp_1050_1844 + 0x6), &stack0xfffe, param_3,
                    u_var2, 0x1000, param_4);
    return;
}


pub fn pass1_1010_116c(param_1: U32Ptr, param_2: i16, param_3: u16) {
    let ppcVar1: u32;
    let i_var2: i16;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u32;
    let uStack4: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x56) != 0x0) {
        ppcVar1 = (*param_1 + 0x34);
        (**ppcVar1)();
    }
    ppcVar1 = (*param_1 + 0x28);
    u_var6 = (**ppcVar1)();
    // u_var3 = (u_var6 >> 0x10);
    if (u_var6 != 0x0) {
        uStack4 = ctx.DAT_1050_0ecc;
        i_var2 = ctx.DAT_1050_0ecc + 0x1;
        if (i_var2 == 0x0) {
            uStack4 = 0x0;
        }
        pass1_1010_1146(param_1, uStack4, param_2, param_3);
        pass1_1010_11c6(param_1, i_var2, u_var3);
        (i_var4 + 0x56) = i_var2;
        (i_var4 + 0x58) = u_var3;
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1010_11c6(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pi_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let ppc_var3: u32;
    let u_var4: u32;
    let u_var5: u32;
    let iVar6: &mut Struct239;
    let iVar7: i16;
    let i_var8: i16;
    let uVar9: u16;
    let u_var10: u16;
    let puVar11: U32Ptr;
    let puVar12: U32Ptr;
    let puVar13: U32Ptr;
    let puVar14: U32Ptr;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let uVar15: u16;
    let extraout_DX_01: U32Ptr;
    let puVar16: U32Ptr;
    let puVar17: u32;
    let iVar18: &mut Struct234;
    let iVar19: i16;
    let iVar21: i16;
    let iVar20: &mut Struct238;
    let uVar22: u16;
    let uVar23: u16;
    let pu_var24: U32Ptr;
    let puStack50: u32;
    let iStack42: i16;
    let iStack40: i16;
    let paStack38: &mut Struct20;
    let iStack28: i16;
    let puStack26: u32;
    let puStack22: u32;
    let uStack14: u32;
    let uStack10: u32;

    if (ctx.DAT_1050_0ecc == -0x1) {
        return;
    }
    mem_op_1000_179c(0x1a, param_3, 0x1000);
    if ((param_3 | param_2) == 0x0) {
        iVar6 = 0x0;
        puVar11 = 0x0;
    } else {
        pu_var24 = pass1_1010_37d4(CONCAT22(param_3, param_2));
        // puVar11 = (pu_var24 >> 0x10);
        iVar6 = pu_var24;
    }
    uStack10 = 0x10500ece;
    uStack14 = 0x0;
    puVar12 = puVar11;
    loop {
        // uVar22 = (param_1 >> 0x10);
        iVar18 = param_1;
        pi_var1 = &iVar18.field_0x68;
        if (*pi_var1 == uStack14 || *pi_var1 < uStack14) { break; }
        u_var5 = iVar18.field_0x64;
        u_var4 = (u_var5 + uStack14 * 0x4);
        puVar17 = (u_var4 + ctx.DAT_1050_0ecc * 0x8);
        puStack50 = (u_var4 & 0xffff0000 | ZEXT24(puVar17));
        iVar7 = string_1000_475e(ctx, uStack10, *puVar17);
        if (iVar7 != 0x0) {
            uStack10 = *puStack50;
            uStack14 = uStack14 & 0xffff | (uStack14._2_2_ + 0x1) << 0x10;
        }
        uStack14 = uStack14 & 0xffff0000 | (uStack14 + 0x1);
    }
    iVar6.field_0x10 = uStack14._2_2_;
    pu_var24 = struct_1010_38f8(CONCAT22(puVar11, iVar6), uStack14._2_2_, uStack14._2_2_, puVar12,
    );
    // puVar13 = (pu_var24 >> 0x10);
    i_var8 = 0x0;
    mem_op_1000_179c(0x400, puVar13, 0x1000);
    puVar12 = puVar13;
    iVar7 = i_var8;
    mem_op_1000_179c(0x400, puVar13, 0x1000);
    paStack38 = CONCAT22(puVar12, iVar7);
    iStack28 = 0x0;
    pass1_1000_4906(CONCAT22(puVar13, i_var8), 0x0, 0x400);
    pass1_1000_4906(CONCAT22(puVar12, iVar7), 0x0, 0x400);
    iStack42 = 0x0;
    u_var10 = 0x0;
    loop {
        pu_var2 = &iVar6.field_0x10;
        if (*pu_var2 == u_var10 || *pu_var2 < u_var10) {
            return;
        }
        u_var5 = iVar18.field_0x64;
        // uVar23 = (u_var5 >> 0x10);
        iVar19 = u_var5;
        iVar21 = (iVar19 + iStack28 * 0x4);
        puVar16 = (iVar19 + iStack28 * 0x4 + 0x2);
        iVar19 = iVar21 + (ctx.DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
        puStack22 = CONCAT22(puVar16, iVar19);
        uVar9 = iVar21 + (ctx.DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
        puVar14 = puVar16;
        mem_op_1000_179c(0x1a, puVar16, 0x1000);
        if ((puVar14 | uVar9) == 0x0) {
            u_var5 = iVar6.field_0x8;
            (u_var5 + u_var10 * 0x4) = 0x0;
        } else {
            pu_var24 = pass1_1010_37d4(CONCAT22(puVar14, uVar9));
            u_var5 = iVar6.field_0x8;
            // uVar23 = (u_var5 >> 0x10);
            iVar21 = u_var5;
            (iVar21 + u_var10 * 0x4) = pu_var24;
            (iVar21 + u_var10 * 0x4 + 0x2) = (pu_var24 >> 0x10);
        }
        iStack42 += 0x1;
        u_var5 = iVar6.field_0x8;
        // uVar23 = (u_var5 >> 0x10);
        iVar21 = u_var5;
        u_var5 = (iVar21 + u_var10 * 0x4);
        ppc_var3 = ((iVar21 + u_var10 * 0x4) + 0x1c);
        (**ppc_var3)(0x1000, u_var5, (u_var5 >> 0x10), iStack42, iVar19, puVar16);
        uStack14 = u_var10;
        puVar16 = extraout_dx;
        loop {
            pi_var1 = &iVar18.field_0x68;
            if (*pi_var1 == iStack28 || *pi_var1 < iStack28) { break; }
            iVar19 = iStack28 * 0x4;
            u_var5 = iVar18.field_0x64;
            u_var5 = (u_var5 + iVar19);
            iVar21 = string_1000_475e(
                ctx, *puStack22,
                (u_var5 + (ctx.DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
            if (iVar21 != 0x0) { break; }
            u_var5 = iVar18.field_0x64;
            // uVar23 = (u_var5 >> 0x10);
            iVar21 = u_var5;
            puVar16 = (iVar21 + iVar19 + 0x2);
            u_var10 = (iVar21 + iVar19) + (ctx.DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
            puStack26 = CONCAT22(puVar16, u_var10);
            mem_op_1000_179c(0x1a, puVar16, 0x1000);
            if ((puVar16 | u_var10) == 0x0) {
                uVar23 = 0x0;
                uVar15 = 0x0;
            } else {
                pu_var24 = pass1_1010_37d4(CONCAT22(puVar16, u_var10));
                // uVar15 = (pu_var24 >> 0x10);
                uVar23 = SUB42(pu_var24, 0x0);
            }
            (uStack14._2_2_ * 0x4 + i_var8) = uVar23;
            (uStack14._2_2_ * 0x4 + i_var8 + 0x2) = uVar15;
            u_var5 = iVar18.field_0x64;
            // uVar23 = (u_var5 >> 0x10);
            iVar21 = u_var5;
            iStack42 += 0x1;
            u_var5 = (uStack14._2_2_ * 0x4 + i_var8);
            ppc_var3 = (
                (uStack14._2_2_ * 0x4 + i_var8) + 0x1c);
            (**ppc_var3)(0x1000, u_var5, (u_var5 >> 0x10), iStack42,
                         (iVar21 + iStack28 * 0x4) + (ctx.DAT_1050_0ecc * 0x6 + 0xebc) * 0x8,
                         (iVar21 + iStack28 * 0x4 + 0x2));
            iStack40 = 0x0;
            puVar16 = extraout_DX_00;
            loop {
                pi_var1 = &iVar18.field_0x68;
                if (*pi_var1 == iStack28 || *pi_var1 < iStack28) { break; }
                u_var5 = iVar18.field_0x64;
                u_var5 = (u_var5 + iStack28 * 0x4);
                iVar21 = string_1000_475e(
                    ctx, *puStack26,
                    (u_var5 + (ctx.DAT_1050_0ecc * 0x6 + 0xebc) * 0x8));
                if (iVar21 != 0x0) { break; }
                u_var5 = iVar18.field_0x64;
                u_var5 = (u_var5 + iStack28 * 0x4);
                u_var10 = string_1000_475e(
                    ctx, *puStack22,
                    (u_var5 + (ctx.DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
                if (u_var10 != 0x0) { break; }
                mem_op_1000_179c(0x1a, puVar16, 0x1000);
                if ((puVar16 | u_var10) == 0x0) {
                    uVar23 = 0x0;
                    uVar15 = 0x0;
                } else {
                    pu_var24 = pass1_1010_37d4(CONCAT22(puVar16, u_var10));
                    // uVar15 = (pu_var24 >> 0x10);
                    uVar23 = SUB42(pu_var24, 0x0);
                }
                (iStack40 * 0x4 + iVar7) = uVar23;
                (iStack40 * 0x4 + iVar7 + 0x2) = uVar15;
                u_var5 = iVar18.field_0x64;
                // uVar23 = (u_var5 >> 0x10);
                iVar20 = u_var5;
                iStack42 += 0x1;
                u_var5 = (iStack40 * 0x4 + iVar7);
                ppc_var3 = ((iStack40 * 0x4 + iVar7) + 0x1c);
                (**ppc_var3)(0x1000, u_var5, (u_var5 >> 0x10), iStack42,
                             (iVar20 + iStack28 * 0x4) + (ctx.DAT_1050_0ecc * 0x6 + 0xebe) * 0x8,
                             (iVar20 + iStack28 * 0x4 + 0x2));
                iStack28 += 0x1;
                iStack40 += 0x1;
                puVar16 = extraout_DX_01;
            }
            u_var5 = (uStack14._2_2_ * 0x4 + i_var8);
            (u_var5 + 0x10) = iStack40;
            u_var10 = iStack40 << 0x2;
            iVar21 = iVar7;
            puVar14 = puVar12;
            pu_var24 = struct_1010_38f8((uStack14._2_2_ * 0x4 + i_var8), iStack40, u_var10,
                                        puVar16);
            // puVar16 = (pu_var24 >> 0x10);
            pass1_1000_48a8(pu_var24, CONCAT22(puVar14, iVar21), u_var10);
            pass1_1000_4906(paStack38, 0x0, 0x400);
            uStack14 = uStack14 & 0xffff | (uStack14._2_2_ + 0x1) << 0x10;
        }
        u_var5 = iVar6.field_0x8;
        u_var5 = (u_var5 + uStack14 * 0x4);
        (u_var5 + 0x10) = uStack14._2_2_;
        u_var10 = uStack14._2_2_ << 0x2;
        u_var5 = iVar6.field_0x8;
        iVar21 = i_var8;
        puVar14 = puVar13;
        pu_var24 = struct_1010_38f8((u_var5 + uStack14 * 0x4), uStack14._2_2_, u_var10, puVar16);
        pass1_1000_48a8(pu_var24, CONCAT22(puVar14, iVar21), u_var10);
        pass1_1000_4906(CONCAT22(puVar13, i_var8), 0x0, 0x400);
        u_var10 = uStack14 + 0x1;
    }
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_1656(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16,
                       param_6: U32Ptr, param_7: i16, param_8: u16)

{
    let u_var1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let uVar7: u32;

    unk_destroy_win_op_1010_305a(CONCAT22(param_2, param_1), param_3, param_4, param_5);
    if ((param_1 + 0x16) == 0x3) {
        puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x32, param_8, param_6, param_7);
        u_var1 = (param_1 + 0x32);
        u_var1 = (u_var1 + 0x42);
        // u_var5 = (u_var1 >> 0x10);
        i_var4 = u_var1;
        u_var1 = (i_var4 + 0x16);
        uVar7 = struct_op_1030_73a8((u_var1 + 0x4));
        u_var2 = pass1_1010_7818(puVar6, uVar7);
        u_var1 = (i_var4 + 0x16);
        u_var3 = u_var2;
        ui_op_1010_79aa(puVar6, 0x0, (u_var1 + 0x4), param_8);
        if (u_var3 == 0x0) {
            u_var1 = (i_var4 + 0x16);
            unk_win_op_1010_7300(puVar6, 0x0, u_var2, (u_var1 + 0x4));
        }
    }
    return;
}


pub fn pass1_1010_16ee(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16,
                       param_6: U32Ptr)

{
    let unaff_SS: u16;

    mem_op_1000_179c(0x4a, param_6, 0x1000);
    if ((param_6 | param_5) != 0x0) {
        pass1_1040_c54a(CONCAT22(param_6, param_5), 0x0,
                        CONCAT22(param_4, param_3), &ctx.PTR_LOOP_1050_1040,
                        unaff_SS);
        return;
    }
    return;
}


pub fn pass1_1010_1788(param_1: u16, param_2: u16, param_3: u32, param_4: U32Ptr, param_5: i16,
                       param_6: u16)

{
    let mut pcVar1: String;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let u_var4: u32;
    let pu_var5: U32Ptr;
    let in_stack_0000fff6: i16;

    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_6, param_4, param_5);
    pu_var5 = 0x1778;
    u_var4 = pass1_1028_b58e(param_3);
    // u_var2 = (u_var4 >> 0x10);
    pcVar1 = pass1_1010_b038(pu_var3, u_var4, u_var2, pu_var5, in_stack_0000fff6);
    str_op_1008_60e8(CONCAT22(u_var2, pcVar1), u_var2);
    return;
}


pub fn pass1_1010_17c0(
    ctx: &mut AppContext,
    param_1: i32,
    unaff_cs: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;

    unk_destroy_win_op_1010_2fa0(ctx, param_1, unaff_cs);
    pu_var1 = param_1.field_0x56;
    u_var2 = param_1.field_0x58;
    if (u_var2 | pu_var1) != 0x0 {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    &param_1.field_0x56 = 0x0;
    fn_ptr_1000_17ce(ctx, param_1.field_0x60, 0x1000);
    pass1_1000_4906(param_1.field_0x64, None, param_1.field_0x68 << 0x2);
    fn_ptr_1000_17ce(ctx, param_1.field_0x64, 0x1000);
    param_1.field_0x60 = 0x0;
    param_1.field_0x64 = 0x0;
    return;
}


pub fn pass1_1010_184a(param_1: U32Ptr, param_2: U32Ptr) {
    let i_var1: i16;
    let i_var2: i16;

    i_var2 = ctx.DAT_1050_0ecc;
    i_var1 = (ctx.DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
    i_var1 = string_1000_475e(
        ctx, (i_var1 + *param_1),
        (i_var1 + *param_2));
    if (i_var1 == 0x0) {
        i_var1 = (i_var2 * 0x6 + 0xebc) * 0x8;
        i_var1 = string_1000_475e(
            ctx, (i_var1 + *param_1),
            (i_var1 + *param_2));
        if (i_var1 == 0x0) {
            i_var2 = (i_var2 * 0x6 + 0xebe) * 0x8;
            string_1000_475e(ctx, (i_var2 + *param_1), (i_var2 + *param_2));
        }
    }
    return;
}


pub fn pass1_1010_18f4(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_0f76(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_195e(param_1: &mut Struct19, param_2: &mut Struct19, param_3: u16) -> u32

{
    let in_DX: U32Ptr;
    let unaff_DI: i16;
    let unaff_SS: u16;
    let pu_var1: U32Ptr;

    pass1_1010_0f24(param_1, param_2, param_3, in_DX, unaff_SS);
    (param_1 + 0xb) = 0x0;
    CONCAT22(param_2, param_1) = 0x1b2a;
    param_1.field_0x2 = 0x1010;
    pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, unaff_SS, in_DX, unaff_DI);
    ((param_1 + 0xb)).field_0x0 = pu_var1;
    param_1[0xb].field_0x2 = (pu_var1 >> 0x10);
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1010_19a4(param_1: U32Ptr, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let extraout_dx: u16;
    let local_14: [u8; 12];

    pass1_1028_dc52(CONCAT22(param_3, local_14), 0x1, 0x0, 0x700);
    loop {
        pu_var2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_3, pu_var2));
        if ((param_2 | pu_var2) == 0x0) { break; }
        ppcVar1 = (*param_1 + 0x40);
        (**ppcVar1)(&USHORT_1050_1028, param_1);
        param_2 = extraout_dx;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_1a06(param_1: u32, param_2: u32, param_3: i16, param_4: u16) {
    let mut pcVar1: String;
    let i_var2: i16;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u32;
    let puVar8: U32Ptr;
    let mut pcVar9: String;
    let in_stack_0000ffee: i16;

    uVar7 = pass1_1028_b58e(param_2);
    // puVar4 = (uVar7 >> 0x10);
    // u_var6 = (param_1 >> 0x10);
    pcVar1 = pass1_1010_b038((param_1 + 0x6e), uVar7, puVar4,
                             0x1770, in_stack_0000ffee);
    i_var2 = pass1_1000_3e2c(CONCAT22(puVar4, pcVar1));
    puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x32, param_4, puVar4, param_3);
    // u_var5 = (puVar8 >> 0x10);
    u_var3 = pass1_1010_7818(puVar8, param_2);
    uVar7 = (param_1 + 0x6e);
    pcVar9 = string_op_1010_ada6(0x1000, u_var5, uVar7, (uVar7 >> 0x10),
                                 i_var2, u_var3);
    str_op_1008_60e8(pcVar9, (pcVar9 >> 0x10));
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_1a66(param_1: u32, param_2: u32) -> u8

{
    let u_var1: u32;
    let u_var2: u8;
    let u_var3: u16;
    let b_var4: bool;
    let u_var5: u16;
    // let u_var6: u16;
    // let u_var7: u32;
    let u_var5 = param_2;
// u_var5 = (param_2 >> 0x10);
    if ((param_2 + 0x1c) != 0x2) || (((param_2 + 0x1e) & 0xff) != 0x0) {
        let u_var7 = pass1_1028_b58e(ctx, param_2);
// u_var6 = (param_1 >> 0x10);
        let mut u_var1 = (param_1 + 0x6e);
        pass1_1010_c2d8(u_var1, u_var7);
        if (u_var7 != 0x2) || ((u_var7 & 0xff0000) != 0x0) {
            u_var1 = (param_1 + 0x6e);
            let u_var3 = pass1_1010_b028(u_var1, param_2);
            BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var3, 0x5);
            if (BVar4 == 0x0) & &(BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var3, 0x6), BVar4 == 0x0) {
                u_var2 = '\0' as u8;
            } else {
                u_var2 = '\x01' as u8;
            }
            return u_var2;
        }
    }
    return '\0' as u8;
}


pub fn pass1_1010_1b04(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_0f76(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn 
pass1_1010_1b6e(param_1: & mut Struct79,param_2: & mut Struct79,param_3: u16,param_4: u16,
param_5: * mut u8) -> u32

{
let unaff_DI: i16; let pu_var1: * mut u16;

pass1_1010_0f24(param_1, param_2, param_3,param_5, param_4); (param_1 + 0xb) = 0x0; CONCAT22(param_2, param_1) = 0x1d04; param_1.field_0x2 = 0x1010; pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_4,param_5, unaff_DI); ((param_1 + 0xb)).field_0x0 = pu_var1; param_1[0xb].field_0x2 = (pu_var1 >> 0x10); return CONCAT22(param_2, param_1);
}



pub fn pass1_1010_1bb4(param_1: U32Ptr, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let extraout_dx: u16;
    let local_14: [u8; 12];

    pass1_1028_dc52(CONCAT22(param_3, local_14), 0x1, 0x0, 0x700);
    loop {
        pu_var2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_3, pu_var2));
        if ((param_2 | pu_var2) == 0x0) { break; }
        ppcVar1 = (*param_1 + 0x40);
        (**ppcVar1)(&USHORT_1050_1028, param_1);
        param_2 = extraout_dx;
    }
    return;
}


pub fn pass1_1010_1c16(param_1: u32, param_2: u32, param_3: i16) {
    let mut pcVar1: String;
    let u_var2: u16;
    let u_var3: u32;

    u_var3 = pass1_1028_b58e(param_2);
    // u_var2 = (u_var3 >> 0x10);
    pcVar1 = pass1_1010_b038((param_1 + 0x6e), u_var3, u_var2,
                             0x178a, param_3);
    str_op_1008_60e8(CONCAT22(u_var2, pcVar1), u_var2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_1c40(param_1: u32, param_2: u32) -> u8

{
// let u_var1: u32; let u_var2: u8;
    let u_var3: u16;
    let b_var4: bool;
// let u_var5: u16; let u_var6: u16;
// let uVar7: u32;
    let u_var5 = (param_2 >> 0x10);
// ;
    if ((param_2 + 0x1c) != 0x2) || (((param_2 + 0x1e) & 0xff) != 0x0) {
        let u_var7 = pass1_1028_b58e(ctx, param_2);
// u_var6 = (param_1 >> 0x10);
        let mut u_var1 = (param_1 + 0x6e);
        pass1_1010_c2d8(u_var1, u_var7);
        if (u_var7 != 0x2) || ((u_var7 & 0xff0000) != 0x0) {
            u_var1 = (param_1 + 0x6e);
            u_var3 = pass1_1010_b028(u_var1, param_2);
            BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var3, 0x11);
            if (BVar4 == 0x0) & &(BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var3, 0x12), BVar4 == 0x0) {
                u_var2 = '\0';
            } else {
                u_var2 = '\x01';
            }
            return u_var2;
        }
    }
    return '\0';
}


pub fn pass1_1010_1cde(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_0f76(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_1d80(param_1: &mut Struct18, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct455;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x2014;
    i_var4.field_0x2 = 0x1010;
    pass1_1010_1f62(param_2, param_1 & 0xffff | u_var4 << 0x10, 0x1);
    pu_var1 = i_var4.field_0x4;
    u_var2 = i_var4.field_0x6;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    *param_1 = 0x389a;
    i_var4.field_0x2 = 0x1008;
    return;
}


pub fn pass1_1010_1dce() -> u16

{
    return 0x0;
}


pub fn pass1_1010_1dd4() -> u16

{
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_1dda(param_1: u32) {
    pass1_1010_209e(ctx.PTR_LOOP_1050_0ed0, (param_1 + 0x8));
    return;
}


pub fn pass1_1010_1df2(param_1: u32, param_2: u16, param_3: u32, param_4: u16, param_5: u16) {
    let ppcVar1: u32;
    let in_AX: &mut Struct241;
    let pu_var2: U32Ptr;
    let extraout_dx: U32Ptr;
    let i_var3: &mut Struct242;
    let u_var3: u16;
    let puStack10: U32Ptr;
    let puStack6: U32Ptr;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    pu_var2 = param_5;
    if (i_var3.field_0x4 == 0x0) {
        mem_op_1000_179c(0xc, param_5, 0x1000);
        pu_var2 = (param_5 | in_AX);
        if (pu_var2 == 0x0) {
            i_var3.field_0x4 = 0x0;
        } else {
            set_struct_1008_574a(CONCAT22(param_5, in_AX));
            &i_var3.field_0x4 = in_AX;
            (&i_var3.field_0x4 + 0x2) = extraout_dx;
            pu_var2 = extraout_dx;
        }
    }
    mem_op_1000_179c(0xa, pu_var2, 0x1000);
    puStack10 = CONCAT22(pu_var2, in_AX);
    if ((pu_var2 | in_AX) == 0x0) {
        puStack6 = 0x0;
    } else {
        *puStack10 = 0x389a;
        in_AX.field_0x2 = 0x1008;
        in_AX.field_0x4 = param_3;
        in_AX.field_0x8 = param_2;
        *puStack10 = 0x2010;
        in_AX.field_0x2 = 0x1010;
        puStack6 = puStack10;
    }
    ppcVar1 = (*i_var3.field_0x4 + 0x4);
    (**ppcVar1)(0x1000, i_var3.field_0x4, puStack6, (puStack6 >> 0x10));
    return;
}


pub fn pass1_1010_1ea6(param_1: u32, param_2: i32, param_3: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let puVar4: u32;
    let pu_var5: U32Ptr;
    let extraout_dx: u16;
    let iVar6: &mut Struct498;
    let u_var6: u16;
    let local_c: [u8; 4];
    let uStack8: u32;
    let uStack4: u16;

    // u_var6 = (param_1 >> 0x10);
    iVar6 = param_1;
    if (iVar6.field_0x4 == 0x0) {
        return;
    }
    uStack4 = 0x0;
    pass1_1008_5784(CONCAT22(param_3, local_c), iVar6.field_0x4);
    loop {
        pu_var5 = local_c;
        pass1_1008_5b12(pu_var5, param_3);
        if ((extraout_dx | pu_var5) == 0x0) { break; }
        if ((pu_var5 + 0x4) == param_2) {
            uStack4 = 0x1;
            ppc_var3 = (*iVar6.field_0x4 + 0xc);
            (**ppc_var3)(0x1008);
            uStack8 = 0x0;
        }
    }
    puVar4 = iVar6.field_0x4;
    if ((puVar4 + 0x8) == 0x0) {
        // WARNING: Load size is inaccurate
        pu_var1 = iVar6.field_0x4;
        u_var2 = (&iVar6.field_0x4 + 0x2);
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)(0x1008, pu_var1, u_var2, 0x1, pu_var1, u_var2, pu_var1, u_var2);
        }
        iVar6.field_0x4 = 0x0;
    }
    return;
}


pub fn pass1_1010_1f62(param_1: u16, param_2: &mut Struct27, param_3: i16) {
    let u_var1: u32;
    let ppcVar2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let lVar5: i32;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_1, local_a), (param_2 + 0x4));
    loop {
        lVar5 = pass1_1008_5b12(local_a, param_1);
        // u_var4 = (lVar5 >> 0x10);
        i_var3 = lVar5;
        if (lVar5 == 0x0) { break; }
        if ((((i_var3 + 0x8) == 0x0) || (param_3 == 0x0)) || ((i_var3 + 0x8) == param_3)) {
            u_var1 = (i_var3 + 0x4);
            ppcVar2 = ((i_var3 + 0x4) + 0x4);
            (**ppcVar2)(0x8, u_var1, (u_var1 >> 0x10), param_3);
        }
    }
    return;
}


pub fn pass1_1010_1fbe(param_1: U32Ptr, param_2: u8) -> u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_1fea(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_1d80(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_2024(param_1: u32) -> u32

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x124) = 0x0;
    ctx._PTR_LOOP_1050_0ed0 = param_1;
    pass1_1000_4906((param_1 & 0xffff | u_var1 << 0x10),
                    0x0, 0x124);
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_2050(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u16;
    let i_stack4: i16;

    pass1_1010_2816(param_1);
    i_stack4 = 0x0;
    loop {
        // u_var4 = (param_1 >> 0x10);
        pu_var1 = (i_stack4 * 0x4 + param_1);
        u_var2 = (i_stack4 * 0x4 + param_1 + 0x2);
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
        i_stack4 += 0x1;
        if i_stack4 < 0x49 { break; }
    }
    ctx._PTR_LOOP_1050_0ed0 = 0x0;
    return;
}


pub fn pass1_1010_209e(param_1: u32, param_2: u16) {
    pass1_1010_2816(param_1);
    (param_1 + 0x124) = param_2;
    return;
}


pub fn pass1_1010_2816(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let iVar5: i16;
    let u_var6: u16;

    // u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x124) != 0x0) {
        iVar5 = (i_var4 + 0x124) * 0x4;
        pu_var1 = (iVar5 + i_var4);
        u_var2 = (iVar5 + i_var4 + 0x2);
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
        ((i_var4 + 0x124) * 0x4 + i_var4) = 0x0;
        (i_var4 + 0x124) = 0x0;
    }
    return;
}


pub fn pass1_1010_286c() -> u16

{
    let pu_var1: U32Ptr;

    pass1_1008_3e54(&ctx.PTR_LOOP_1048_0000, 0x0, 0x5, 0x12c);
    pass1_1008_3e54(0x105065a6, 0x0, 0x9b, 0x20);
    pass1_1008_3e54(0x105065ac, 0x0, 0xf5, 0x3f);
    pass1_1008_3e54(0x105065b2, 0x0, 0x114, 0x4a);
    pass1_1008_3e54(0x105065b8, 0x0, 0x135, 0x45);
    pass1_1008_3e54(0x105065be, 0x0, 0xf5, 0x7b);
    pu_var1 = pass1_1008_3e54(0x105065c4, 0x0, 0x117, 0x91);
    return pu_var1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_28e6(param_1: &mut Struct631, param_2: U32Ptr, param_3: u16, param_4: U32Ptr,
                       param_5: u16)

{
    let u_var1: u32;
    let u_var2: u16;
    let i_var3: i16;
    let u_var4: u16;
    let paVar5: &mut Struct43;
    let i_stack6: i16;

    struct_op_1018_4cda(param_1, param_2, param_3);
    &param_1.field_0x1c = 0x0;
    param_1.field_0x20 = 0x0;
    param_1.field_0x22 = 0x0;
    param_1.field_0x24 = 0x0;
    param_1.field_0x26 = 0x0;
    CONCAT22(param_2, param_1) = (s_add16_wav_1050_2bdc + 0x8);
    param_1.field_0x2 = 0x1010;
    ctx.PTR_LOOP_1050_4230 = param_1;
    ctx.PTR_LOOP_1050_4232 = param_2;
    pass1_1018_4dce(CONCAT13((param_2 >> 0x8),
                             CONCAT12(param_2, param_1)), 0x56, param_4, param_5);
    paVar5 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x4, param_5);
    ctx.PTR_LOOP_1050_5f2e = (paVar5 >> 0x10);
    param_1.field_0x1c = paVar5;
    param_1.field_0x1e = ctx.PTR_LOOP_1050_5f2e;
    if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
        ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
    } else {}
    u_var2 = fn_ptr_op_1000_1708(0x40, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                 ctx.PTR_LOOP_1050_5f2e, 0x1000);
    param_1.field_0x28 = u_var2;
    param_1.field_0x2a = ctx.PTR_LOOP_1050_5f2e;
// TODO: refactor for loop
    // for (i_stack6 = 0x0; i_stack6 < 0x10; i_stack6 += 0x1) {
    //   paVar5 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc,i_stack6 + 0x56,param_5);
    //   u_var1 = &param_1.field_0x28;
    //   u_var4 = (u_var1 >> 0x10);
    //   i_var3 = u_var1;
    //   (i_var3 + i_stack6 * 0x4) = paVar5;
    //   (i_var3 + i_stack6 * 0x4 + 0x2) = (paVar5 >> 0x10);
    // }
    return;
}


pub fn pass1_1010_29c6(param_1: &mut Struct11) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let iVar5: &mut Struct476;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1 = (s_add16_wav_1050_2bdc + 0x8);
    iVar5.field_0x2 = 0x1010;
    if (&iVar5.field_0x1c != 0x0) {
        pu_var1 = &iVar5.field_0x1c;
        u_var2 = iVar5.field_0x1e;
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
        &iVar5.field_0x1c = 0x0;
        fn_ptr_1000_17ce(ctx, iVar5.field_0x28, 0x1000);
        iVar5.field_0x28 = 0x0;
    }
    clenaup_win_ui_1018_4d22(param_1, 0x1018);
    return;
}


pub fn pass1_1010_2b50(param_1: u16, param_2: u16, param_3: U32Ptr) {
    pass1_1008_3f62(param_3, &ctx.PTR_LOOP_1048_0000);
    return;
}


pub fn pass1_1010_2b66(param_1: u32) -> u32

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x1e),
                    (param_1 + 0x1c));
}


pub fn pass1_1010_2b78(param_1: u16, param_2: u16, param_3: i16, param_4: u32) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;

    pu_var3 = (param_3 * 0x7c + 0xed4);
    pu_var5 = param_4;
    // TODO: refactor for loop
    // for (i_var4 = 0x1f; i_var4 != 0x0; i_var4 += -0x1) {
    //   pu_var2 = pu_var5;
    //   pu_var5 = pu_var5 + 0x1;
    //   pu_var1 = pu_var3;
    //   pu_var3 = pu_var3 + 0x1;
    //   *pu_var2 = *pu_var1;
    // }
    return;
}


pub fn pass1_1010_2b98(param_1: u32, param_2: i16) -> u32

{
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    u_var1 = (param_1 + 0x28);
    // u_var3 = (u_var1 >> 0x10);
    i_var2 = u_var1;
    return CONCAT22((param_2 * 0x4 + i_var2 + -0x156),
                    (param_2 * 0x4 + i_var2 + -0x158));
}


pub fn pass1_1010_2bb9() {
    pass1_1010_286c();
    return;
}


pub fn pass1_1010_2bbe(param_1: &mut Struct11, param_2: u8)

{
    pass1_1010_29c6(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_2bfc(param_1: &mut Struct644, param_2: u16, param_3: u16) -> u16

{
    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0xa = 0x0;
    param_1.field_0xc = 0x0;
    param_1.field_0xe = 0x0;
    param_1.field_0x10 = 0x0;
    CONCAT22(param_2, param_1) = 0x2cc2;
    param_1.field_0x2 = 0x1010;
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1010_2c9c(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_1d80(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_2db2(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let iVar5: &mut Struct473;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = 0x36da;
    iVar5.field_0x2 = 0x1010;
    pu_var1 = iVar5.field_0x56;
    u_var2 = iVar5.field_0x58;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    fn_ptr_1000_17ce(ctx, iVar5.field_0x5c, 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1010_2e02(param_1: u32, param_2: i16) -> u32

{
    let u_var1: u32;
    let i_var2: &mut Struct163;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x5c) != 0x0) {
        u_var1 = (param_1 + 0x5c);
        // u_var2 = (u_var1 >> 0x10);
        i_var2 = u_var1;
        return CONCAT22((i_var2 + param_2 * 0x4 + 0x2),
                        (i_var2 + param_2 * 0x4));
    }
    return 0x0;
}


pub fn pass1_1010_2e30(param_1: u32, param_2: u16, param_3: u16, param_4: i16) {
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    if ((param_1 + 0x5c) != 0x0) {
        u_var1 = (param_1 + 0x5c);
        // u_var3 = (u_var1 >> 0x10);
        i_var2 = u_var1;
        (i_var2 + param_4 * 0x4) = param_2;
        (i_var2 + param_4 * 0x4 + 0x2) = param_3;
    }
    return;
}


pub fn pass1_1010_2e5c(param_1: u16, param_2: u16, param_3: u32) {
    let uStack12: u32;

    uStack12 = param_3;
    if (param_3 == 0x0) {
        return;
    }
    // TODO: refactor for loop
    // for (; (uStack12 & 0xf) != 0x0; uStack12 >>= 0x4) {
    // }
    return;
}


pub fn pass1_1010_2ee2(param_1: U32Ptr, param_2: u16, param_3: u16) {
    let u_var1: u32;
    let ppcVar2: u32;
    let i_var3: i16;
    let extraout_dx: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u32;
    let puStack6: u32;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x52) != 0x0) {
        return;
    }
    i_var3 = 0x0;
    (i_var4 + 0x28) = 0x0;
    u_var6 = *param_1;
    ppcVar2 = (u_var6 + 0x20);
    (**ppcVar2)(param_3, param_1, (i_var4 + 0x12));
    if (i_var3 == 0x0) {
        puStack6 = (i_var4 + 0x56);
    } else {
        u_var1 = (i_var4 + 0x12);
        ppcVar2 = (u_var6 + 0x14);
        (**ppcVar2)(param_3, param_1, u_var1, (u_var1 >> 0x10));
        puStack6 = CONCAT22(extraout_dx, i_var3);
        u_var6 = pass1_1010_2e02(param_1, (i_var3 + 0x12));
        pass1_1010_35a4(param_1, u_var6, (u_var6 >> 0x10));
    }
    pass1_1010_32f4(param_1, puStack6, param_2, param_3);
    pass1_1010_1f62(param_2, param_1, 0x8);
    if ((i_var4 + 0x52) != 0x0) {
        return;
    }
    return;
}


pub fn pass1_1010_32c0(param_1: u32, param_2: u32) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x28) = 0x0;
    (param_1 + 0x12) = param_2;
    return;
}


pub fn pass1_1010_32da(param_1: &mut Struct27, param_2: &mut Struct65, param_3: u16, param_4: u16) {
    pass1_1010_32f4(param_1, (param_2 + 0x42), param_4, param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_32f4(param_1: U32Ptr, param_2: U32Ptr, param_3: u16, param_4: u16) {
    let pu_var1: U32Ptr;
    let pu_var2: u32;
    let u_var3: u32;
    let u_var4: u32;
    let ppcVar5: u32;
    let paVar6: &mut Struct65;
    let uVar7: u32;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let iVar11: i16;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: u16;
    let iVar10: &mut Struct166;
    let iVar12: i16;
    let iVar13: i16;
    let uVar14: u16;
    let uVar15: u16;
    let uVar16: u16;
    let puStack48: U32Ptr;
    let uStack16: u16;
    let iStack12: i16;

    // uVar14 = (param_1 >> 0x10);
    iVar10 = param_1;
    if (iVar10.field_0x52 != 0x0) {
        param_4 = 0x1000;
        fn_ptr_1000_17ce(ctx, iVar10.field_0x52, 0x1000);
        iVar10.field_0x52 = 0x0;
        iVar10.field_0x18 = 0x0;
    }
    uVar8 = param_2._2_2_ | param_2;
    if ((param_2 != 0x0) && (ppcVar5 = (*param_1 + 0x24), (**ppcVar5)(param_4, param_1, param_2),
                             uVar8 != 0x0)) {
        ppcVar5 = (*param_2 + 0x4);
        (**ppcVar5)(param_4, param_2);
        iVar10.field_0x18 = uVar8;
        if (uVar8 != 0x0) {
            iVar10.field_0x24 = 0x0;
            iVar10.field_0x26 = 0x0;
            pu_var1 = &iVar10.field_0x18;
            *pu_var1 = *pu_var1 - iVar10.field_0x28;
            if (0xa < iVar10.field_0x18) {
                iVar10.field_0x26 = 0x1;
                iVar10.field_0x18 = 0xa;
            }
            if (0x1 < iVar10.field_0x28) {
                iVar10.field_0x24 = 0x1;
            }
            if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
                ctx.PTR_LOOP_1050_5f2e = extraout_dx;
                ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(extraout_dx, 0x1000);
            } else {}
            uVar16 = 0x1000;
            uVar9 = fn_ptr_op_1000_1708(0x28, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                        ctx.PTR_LOOP_1050_5f2e, 0x1000);
            &iVar10.field_0x52 = uVar9;
            (&iVar10.field_0x52 + 0x2) = ctx.PTR_LOOP_1050_5f2e;
            if (((&iVar10.field_0x52 + 0x2) | &iVar10.field_0x52) != 0x0) {
                u_var3 = (param_2 + 0x8);
                iVar11 = iVar10.field_0x10;
                iStack12 = 0x0;
                // TODO: refactor for loop
                // for (uStack16 = 0x0; pu_var1 = &iVar10.field_0x18,
                //     *pu_var1 != uStack16 && uStack16 <= *pu_var1; uStack16 += 0x1) {
                //   paVar6 = iVar10.field_0x52;
                //   uVar8 = paVar6 + uStack16 * 0x4;
                //   uVar7 = paVar6 & 0xffff0000;
                //   puStack48 = (uVar7 | uVar8);
                //   u_var4 = ((iVar10.field_0x28 + uStack16) * 0x4 + u_var3);
                //   ppcVar5 = (*param_1 + 0x1c);
                //   u_var10 = uStack16;
                //   (**ppcVar5)(uVar16,param_1,u_var4,(u_var4 >> 0x10),
                //               iVar10.field_0x22);
                //   *puStack48 = u_var10;
                //   (uVar8 + 0x2) = extraout_DX_00;
                //   paVar6 = iVar10.field_0x52;
                //   u_var4 = (paVar6 + uStack16 * 0x4);
                //   iStack12 += (u_var4 + 0x24) + 0x8;
                //   if (iVar11 + -0xa < iStack12) {
                //     uVar16 = 0x1008;
                //     debug_print_1008_6048(s_overflow_on_node__d_1050_11ca,0x1008,param_3);
                //     iVar10.field_0x18 = uStack16 - 0x1;
                //     iVar10.field_0x26 = 0x1;
                //     paVar6 = iVar10.field_0x52;
                //     uVar15 = (paVar6 >> 0x10);
                //     iVar13 = paVar6;
                //     pu_var2 = (iVar13 + uStack16 * 0x4);
                //     uVar8 = (iVar13 + uStack16 * 0x4 + 0x2);
                //     if ((uVar8 | pu_var2) != 0x0) {
                //       ppcVar5 = *pu_var2;
                //       (**ppcVar5)(0x1008,pu_var2,uVar8,0x1);
                //     }
                //     paVar6 = iVar10.field_0x52;
                //     iVar13 = uStack16 * 0x4;
                //     (paVar6 + iVar13) = 0x0;
                //     if (0x0 < uStack16) {
                //       paVar6 = iVar10.field_0x52;
                //       uVar15 = (paVar6 >> 0x10);
                //       iVar12 = paVar6;
                //       pu_var2 = (iVar12 + iVar13 + -0x4);
                //       uVar8 = (iVar12 + iVar13 + -0x2);
                //       if ((uVar8 | pu_var2) != 0x0) {
                //         ppcVar5 = *pu_var2;
                //         (**ppcVar5)(0x1008,pu_var2,uVar8,0x1);
                //       }
                //       paVar6 = iVar10.field_0x52;
                //       (uStack16 * 0x4 + paVar6 + -0x4) = 0x0;
                //     }
                //   }
                // }
                iVar10.field_0x20 = 0xa;
                uVar9 = iVar10.field_0x1e;
                mov_update_win_1040_93aa(ctx, iVar10.field_0x52, 0xa, uVar9, &ctx.PTR_LOOP_1050_1040);
                // TODO: refactor for loop
                // for (uStack16 = 0x1; pu_var1 = &iVar10.field_0x18,
                //     *pu_var1 != uStack16 && uStack16 <= *pu_var1; uStack16 += 0x1) {
                //   paVar6 = iVar10.field_0x52;
                //   u_var3 = (uStack16 * 0x4 + paVar6 + -0x4);
                //   iVar11 = u_var3;
                //   uVar16 = (u_var3 >> 0x10);
                //   paVar6 = iVar10.field_0x52;
                //   mov_update_win_1040_93aa
                //             ()(paVar6 + uStack16 * 0x4),
                //              (iVar11 + 0x20) + (iVar11 + 0x24) + 0x8,uVar9,
                //              &ctx.PTR_LOOP_1050_1040);
                // }
            }
        }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1010_35a4(param_1: U32Ptr, param_2: u32, param_3: U32Ptr) {
    let ppcVar1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let puVar4: u32;
    let u_var5: u16;
    let extraout_dx: U32Ptr;
    let puVar6: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let uVar7: u16;
    let unaff_SS: u16;
    let uStack12: u32;
    let puStack8: u32;

    // uVar7 = (param_1 >> 0x10);
    u_var2 = (param_1 + 0x56);
    u_var2 = (u_var2 + 0x8);
    puStack8 = (u_var2 + (param_1 + 0x5a) * 0x4);
    uStack12 = param_2;
    if (param_2 != 0x0) {
        uVar7 = 0x1000;
        mem_op_1000_179c(0x4a, param_3, 0x1000);
        u_var3 = param_2;
        u_var5 = param_3 | u_var3;
        if (u_var5 == 0x0) {
            u_var3 = 0x0;
            u_var5 = 0x0;
        } else {
            uVar7 = SUB42(&ctx.PTR_LOOP_1050_1040, 0x0);
            pass1_1040_c54a((param_2 & 0xffff | ZEXT24(param_3) << 0x10), 0x1, puStack8,
                            &ctx.PTR_LOOP_1050_1040, unaff_SS);
        }
        ppcVar1 = (*param_1 + 0x18);
        (**ppcVar1)(uVar7, param_1, 0x1, u_var3, u_var5);
        puVar6 = extraout_dx;
        // TODO: refactor for loop
        // for (; (uStack12 & 0xf) != 0x0; uStack12 >>= 0x4) {
        //   u_var2 = (puStack8 + 0x8);
        //   puStack8 = *(u32 **)(((uStack12 & 0xf) - 0x1) * 0x4 + u_var2);
        //   uVar7 = 0x1000;
        //   puVar4 = puStack8;
        //   mem_op_1000_179c(0x4a,puVar6,0x1000);
        //   u_var3 = puVar4;
        //   u_var5 = puVar6 | u_var3;
        //   if (u_var5 == 0x0) {
        //     u_var3 = 0x0;
        //     u_var5 = 0x0;
        //   }
        //   else {
        //     uVar7 = SUB42(&ctx.PTR_LOOP_1050_1040,0x0);
        //     pass1_1040_c54a((puVar4 & 0xffff | ZEXT24(puVar6) << 0x10),0x1,
        //                     puStack8,&ctx.PTR_LOOP_1050_1040,unaff_SS);
        //   }
        //   ppcVar1 = (*param_1 + 0x18);
        //   (**ppcVar1)(uVar7,param_1,0x1,u_var3,u_var5);
        //   puVar6 = extraout_DX_00;
        // }
    }
    return;
}


pub fn pass1_1010_3680(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16,
                       param_6: U32Ptr, param_7: u16)

{
    mem_op_1000_179c(0x4a, param_6, 0x1000);
    if ((param_6 | param_5) != 0x0) {
        pass1_1040_c54a(CONCAT22(param_6, param_5), 0x1,
                        CONCAT22(param_4, param_3), &ctx.PTR_LOOP_1050_1040, param_7,
        );
        return;
    }
    return;
}


pub fn pass1_1010_36b4(param_1: U32Ptr, param_2: u8) -> u16

{
    let unaff_SS: u16;

    pass1_1010_2db2(param_1, unaff_SS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_3702(param_1: &mut Struct19, param_2: &mut Struct19, param_3: u16) -> &mut Struct19 {
    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 0xa) = 0x0;
    CONCAT22(param_2, param_1) = 0x37c4;
    (param_1 + 0x2) = 0x1010;
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1010_3730(param_1: U32Ptr, param_2: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x37c4;
    (param_1 + 0x2) = 0x1010;
    fn_ptr_1000_17ce(ctx, (param_1 + 0xa), 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1010_375e(param_1: u32) -> u32

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0xc), (param_1 + 0xa));
}


pub fn pass1_1010_3770(param_1: u32, param_2: &mut String, param_3: u16) {
    let u_var1: u16;
    let i_var3: &mut Struct477;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var3 = param_1;
    fn_ptr_1000_17ce(ctx, &i_var3.field_0xa, 0x1000);
    u_var1 = str_op_1008_60e8(param_2, param_3);
    i_var3.field_0xa = u_var1;
    i_var3.field_0xc = param_3;
    return;
}


pub fn pass1_1010_379e(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_3730(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_37d4(param_1: U32Ptr) -> u16

{
    let u_var1: u16;

    struct_1010_383a(param_1);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x16) = 0x0;
    *param_1 = 0x3b3e;
    (param_1 + 0x2) = 0x1010;
    return param_1;
}


pub fn pass1_1010_3800(param_1: U32Ptr) {
    let i_var2: &mut Struct478;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var2 = param_1;
    *param_1 = 0x3b3e;
    i_var2.field_0x2 = 0x1010;
    if (i_var2.field_0x16 != 0x0) {
        fn_ptr_1000_17ce(ctx, i_var2.field_0x16, 0x1000);
    }
    pass1_1010_3880(param_1);
    return;
}


pub fn pass1_1010_3880(param_1: U32Ptr) {
    let pi_var1: U32Ptr;
    let pu_var2: u32;
    let u_var3: u16;
    let ppc_var4: u32;
    let lVar5: i32;
    let iVar6: &mut Struct472;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u16;
    let i_stack4: i16;

    // uVar8 = (param_1 >> 0x10);
    iVar6 = param_1;
    *param_1 = 0x3b5e;
    iVar6.field_0x2 = 0x1010;
    if (iVar6.field_0x8 != 0x0) {
        i_stack4 = 0x0;
        loop {
            pi_var1 = &iVar6.field_0x10;
            if (*pi_var1 == i_stack4 || *pi_var1 < i_stack4) { break; }
            lVar5 = iVar6.field_0x8;
            // uVar9 = (lVar5 >> 0x10);
            iVar7 = lVar5;
            pu_var2 = (iVar7 + i_stack4 * 0x4);
            u_var3 = (iVar7 + i_stack4 * 0x4 + 0x2);
            if ((u_var3 | pu_var2) != 0x0) {
                ppc_var4 = *pu_var2;
                (**ppc_var4)();
            }
            i_stack4 += 0x1;
        }
        fn_ptr_1000_17ce(ctx, iVar6.field_0x8, 0x1000);
    }
    *param_1 = 0x389a;
    iVar6.field_0x2 = 0x1008;
    return;
}


pub fn pass1_1010_394a(param_1: u16, param_2: u16, param_3: i16, param_4: u16, param_5: U32Ptr) {
    if (param_3 != 0x0) {
        mem_op_1000_179c(param_3 << 0x2, param_5, 0x1000);
        return;
    }
    mem_op_1000_179c(0x16, param_5, 0x1000);
    if ((param_5 | param_4) != 0x0) {
        struct_1010_383a(CONCAT22(param_5, param_4));
        return;
    }
    return;
}


pub fn pass1_1010_398e(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u32, param_5: u16) {
    let pi_var1: U32Ptr;
    let ppcVar2: u32;
    let u_var3: u32;
    let u_var4: u32;
    let iVar5: i16;
    let u_var6: u16;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let iVar7: i16;
    let i_var8: i16;
    let uVar9: u16;
    let u_var10: u16;
    let uStack12: u16;
    let puStack6: u32;

    // uVar9 = (param_1 >> 0x10);
    u_var3 = *param_1;
    ppcVar2 = (u_var3 + 0x8);
    (**ppcVar2)();
    puStack6 = CONCAT22(extraout_dx, param_5);
    if ((extraout_dx | param_5) == 0x0) {
        return;
    }
    (param_5 + 0xc) = param_4;
    iVar7 = *puStack6;
    ppcVar2 = (iVar7 + 0xc);
    (**ppcVar2)();
    iVar5 = (param_1 + 0x14);
    pi_var1 = (param_1 + 0x14);
    *pi_var1 = *pi_var1 + 0x1;
    ppcVar2 = (iVar7 + 0x10);
    (**ppcVar2)();
    ppcVar2 = (iVar7 + 0x4);
    (**ppcVar2)();
    if (iVar5 != 0x0) {
        ppcVar2 = (u_var3 + 0x8);
        iVar7 = iVar5;
        (**ppcVar2)();
        (param_5 + 0x8) = iVar7;
        (param_5 + 0xa) = extraout_DX_00;
        ctx.PTR_LOOP_1050_11de = ctx.PTR_LOOP_1050_11de + 0x1;
        uVar9 = extraout_DX_00;
        // TODO: refactor for loop
        // for (uStack12 = 0x0; uStack12 < iVar5; uStack12 += 0x1) {
        //   u_var6 = uStack12;
        //   pass1_1010_398e(param_1,uStack12,uStack12 >> 0xf,puStack6,uStack12);
        //   u_var4 = (param_5 + 0x8);
        //   u_var10 = (u_var4 >> 0x10);
        //   iVar7 = u_var4;
        //   i_var8 = uStack12 * 0x4;
        //   (iVar7 + i_var8) = u_var6;
        //   (iVar7 + i_var8 + 0x2) = uVar9;
        //   u_var4 = (param_5 + 0x8);
        //   if ((u_var4 + i_var8) == 0x0) break;
        // }
        ctx.PTR_LOOP_1050_11de = ctx.PTR_LOOP_1050_11de + -0x1;
    }
    return;
}


pub fn pass1_1010_3a86(param_1: u32) -> u16

{
    return (param_1 + 0x10);
}


pub fn pass1_1010_3a94(param_1: u32, param_2: u16) {
    (param_1 + 0x12) = param_2;
    return;
}


pub fn pass1_1010_3aaa(param_1: u32) -> u32

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x6), (param_1 + 0x4));
}


pub fn pass1_1010_3ac2(param_1: u32, param_2: u16, param_3: u32) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x16) = param_3;
    (param_1 + 0x12) = param_2;
    return;
}


pub fn pass1_1010_3adc(param_1: u32) -> u32

{
    let pu_var1: U32Ptr;

    pu_var1 = (param_1 + 0x16);
    return CONCAT22((pu_var1 + 0x2), *pu_var1);
}


pub fn pass1_1010_3af2(param_1: U32Ptr, param_2: u8) -> u16

{
    pass1_1010_3800(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_3b18(param_1: U32Ptr, param_2: u8) -> u16

{
    pass1_1010_3880(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_3bde(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let puVar4: U32Ptr;
    let i_var4: &mut Struct479;
    let u_var5: u16;
    let puStack14: U32Ptr;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x3d6a;
    i_var4.field_0x2 = 0x1010;
    i_var4.field_0xa = 0x3d7a;
    i_var4.field_0xc = 0x1010;
    pu_var1 = i_var4.field_0xe;
    u_var2 = i_var4.field_0x10;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    if (param_1 == 0x0) {
        puVar4 = 0x0;
        u_var5 = 0x0;
    } else {
        puVar4 = &i_var4.field_0xa;
    }
    puStack14 = CONCAT22(u_var5, puVar4);
    *puStack14 = 0x389a;
    puVar4[0x1] = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1010_3c9e(param_1: i32) {
    let u_var1: u16;
    let pu_var2: U32Ptr;

    if (param_1 == 0x0) {
        u_var1 = 0x0;
        pu_var2 = 0x0;
    } else {
        u_var1 = param_1 + 0xa;
        pu_var2 = param_1._2_2_;
    }
    pass1_1008_9262(ctx.PTR_LOOP_1050_0388, (ctx.PTR_LOOP_1050_0388 >> 0x10),
                    (param_1 + 0x12), CONCAT22(pu_var2, u_var1), u_var1,
                    pu_var2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_3cd0(param_1: i32) {
    let i_var1: i16;
    let u_var2: u16;

    if (ctx.PTR_LOOP_1050_0388 != 0x0) {
        if (param_1 == 0x0) {
            i_var1 = 0x0;
            u_var2 = 0x0;
        } else {
            i_var1 = param_1 + 0xa;
            u_var2 = param_1._2_2_;
        }
        pass1_1008_92b2(ctx.PTR_LOOP_1050_0388, (param_1 + 0x12),
                        CONCAT22(u_var2, i_var1));
    }
    return;
}


pub fn pass1_1010_3d0a(param_1: i16, param_2: u16, param_3: i16, param_4: u16) {
    if (param_3 == 0x2) {
        pass1_1010_3cd0(CONCAT22(param_2, param_1 + -0xa));
        pass1_1010_1f62(param_4, CONCAT22(param_2, param_1 + -0xa), 0x2);
    }
    return;
}


pub fn pass1_1010_3d38(param_1: U32Ptr, param_2: u8) -> u16

{
    let unaff_SS: u16;

    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xa));
    pass1_1010_3bde(param_1, unaff_SS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_3d82(param_1: &mut Struct628, param_2: &mut Struct19, param_3: u16, param_4: u16) -> u32

{
    let paVar1: &mut Struct43;

    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    &param_1.field_0xa = 0x0;
    CONCAT22(param_2, param_1) = 0x3e2c;
    param_1.field_0x2 = 0x1010;
    paVar1 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x99, param_4);
    param_1.field_0xa = paVar1;
    param_1.field_0xc = (paVar1 >> 0x10);
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1010_3dc8(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct480;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x3e2c;
    i_var4.field_0x2 = 0x1010;
    pu_var1 = i_var4.field_0xa;
    u_var2 = i_var4.field_0xc;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1010_3e06(param_1: U32Ptr, param_2: u8) -> u16

{
    let unaff_SS: u16;

    pass1_1010_3dc8(param_1, unaff_SS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_3e3c(param_1: &mut Struct55, param_2: u16, param_3: u16) {
    let i_var1: &mut Struct633;
    let u_var1: u16;
    let paVar2: &mut Struct43;

    get_sys_metrics_1018_4b1e(param_1, 0x6, param_2);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x20 = 0x389a;
    i_var1.field_0x22 = 0x1008;
    i_var1.field_0x20 = 0x3aa8;
    i_var1.field_0x22 = 0x1008;
    i_var1.field_0x24 = 0x0;
    &i_var1.field_0x66 = 0x0;
    i_var1.field_0x6a = 0x4;
    i_var1.field_0x6c = 0x0;
    i_var1.field_0x70 = 0x0;
    i_var1.field_0x74 = 0x0;
    pass1_1008_3e54(
        (param_1 & 0xffff0000 | &i_var1.field_0x76), 0x0, 0x3,
        0x5);
    i_var1.field_0x7c = 0x0;
    param_1.field_0x0 = &ctx.PTR_LOOP_1050_4a46;
    i_var1.field_0x2 = 0x1010;
    i_var1.field_0x20 = &ctx.PTR_LOOP_1050_4a82;
    i_var1.field_0x22 = 0x1010;
    pass1_1000_4906(
        (param_1 & 0xffff0000 | &i_var1.field_0x26),
        0x0, 0x40);
    paVar2 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x1a1, param_3);
    i_var1.field_0x66 = paVar2;
    i_var1.field_0x68 = (paVar2 >> 0x10);
    pass1_1018_4b78(param_1, param_3);
    return;
}


pub fn pass1_1010_3f00(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let piVar4: U32Ptr;
    let iVar5: &mut Struct481;
    let u_var5: u16;
    let puStack16: U32Ptr;
    let i_stack4: i16;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = &ctx.PTR_LOOP_1050_4a46;
    iVar5.field_0x2 = 0x1010;
    iVar5.field_0x20 = &ctx.PTR_LOOP_1050_4a82;
    iVar5.field_0x22 = 0x1010;
    i_stack4 = 0x0;
    loop {
        pu_var1 = (&iVar5.field_0x26 + i_stack4 * 0x4);
        u_var2 = (&iVar5.field_0x26 + i_stack4 * 0x4 + 0x2);
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
        i_stack4 += 0x1;
        if (i_stack4 < 0x10) == false { break; }
    }
    pu_var1 = iVar5.field_0x66;
    u_var2 = iVar5.field_0x68;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    fn_ptr_1000_17ce(ctx, iVar5.field_0x70, 0x1000);
    if (param_1 == 0x0) {
        piVar4 = 0x0;
        u_var5 = 0x0;
    } else {
        piVar4 = &iVar5.field_0x20;
    }
    puStack16 = CONCAT22(u_var5, piVar4);
    *puStack16 = 0x389a;
    piVar4[0x1] = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1010_404a(param_1: u32, param_2: u32, param_3: i16, param_4: u16) {
    let u_var1: u16;
    let i_var2: i16;
    let BVar3: bool;
    let u_var4: u16;
    let u_var5: u16;
    let local_4: u16;

    u_var4 = param_2;
    // u_var5 = (param_2 >> 0x10);
    read_file_1008_7cfe(u_var4, u_var5, 0x5, 0x1008, param_4);
    if (param_3 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d4;
    } else {
        i_var2 = param_1;
        // u_var1 = (param_1 >> 0x10);
        BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x24, 0x0, u_var1, 0x2, 0x1008);
        if (BVar3 != 0x0) {
            BVar3 = read_file_1008_7dee(u_var4, u_var5, &local_4, 0x0, param_4, 0x2, 0x1008);
            if (BVar3 != 0x0) {
                BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x7e, 0x0, u_var1, 0x2, 0x1008);
                if (BVar3 != 0x0) {
                    (i_var2 + 0x6a) = local_4;
                    return;
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_40cc(param_1: u32, param_2: i16, param_3: u16) -> u32

{
    pass1_1030_8344(
        ctx, ctx.PTR_LOOP_1050_5748, (param_1 + 0x6c));
    return CONCAT22((param_2 + 0xe), (param_2 + 0xc));
}


pub fn pass1_1010_41d6(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: u16, param_5: u8) {
    let pu_var1: U32Ptr;
    let piVar2: U32Ptr;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let iVar7: i16;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let i_var9: &mut Struct243;
    let iVar10: &mut Struct244;
    let unaff_DI: i16;
    let u_var10: u16;
    let u_var11: u16;
    let puVar12: U32Ptr;
    let iStack50: i16;
    let local_30: i16;
    let local_2e: &mut Struct18;
    let iStack42: i16;
    let paStack40: &mut Struct18;
    let paStack34: &mut Struct18;
    let paStack30: &mut Struct18;
    let iStack26: i16;
    let uStack24: u16;
    let iStack22: i16;
    let uStack20: u32;
    let uStack16: u16;
    let uStack14: u32;
    let uStack10: u32;
    let uStack6: u16;
    let uStack4: u16;

    // u_var10 = (param_1 >> 0x10);
    i_var9 = param_1;
    i_var9.field_0x6c = param_2;
    puVar12 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_4, param_3, unaff_DI);
    // uStack4 = (puVar12 >> 0x10);
    uStack6 = puVar12;
    uStack10 = pass1_1010_ec40(uStack6, uStack4, i_var9.field_0x6c, uStack6, uStack4);
    // puVar8 = (uStack10 >> 0x10);
    i_var9.field_0x74 = (uStack10 + 0x22);
    if (&i_var9.field_0x70 != 0x0) {
        paStack34 = &i_var9.field_0x70;
        paStack30 = paStack34;
        fn_ptr_1000_17ce(ctx, paStack34, 0x1000);
        &i_var9.field_0x70 = 0x0;
    }
    u_var4 = i_var9.field_0x74 << 0x7;
    mem_op_1000_179c(u_var4, puVar8, 0x1000);
    &i_var9.field_0x70 = u_var4;
    i_var9.field_0x72 = puVar8;
    pass1_1030_8344(
        ctx, ctx.PTR_LOOP_1050_5748, i_var9.field_0x6c);
    uStack14 = CONCAT22(puVar8, u_var4);
    uStack16 = (*(u_var4 + 0x10) == 0x9);
    iStack22 = (uStack10 + 0x22);
    u_var4 = iStack22 * 0x6;
    mem_op_1000_179c(u_var4, puVar8, 0x1000);
    paStack30 = CONCAT22(puVar8, u_var4);
    puVar9 = (puVar8 | u_var4);
    if (puVar9 == 0x0) {
        uStack20 = 0x0;
    } else {
        pass1_1000_5586(0x3e38, 0x1008, iStack22, 0x6, u_var4, puVar8);
        uStack20 = paStack30;
    }
    uStack24 = 0x0;
    loop {
        // u_var11 = (uStack10 >> 0x10);
        pu_var1 = (uStack10 + 0x22);
        if (*pu_var1 < uStack24 || *pu_var1 == uStack24) { break; }
        u_var3 = (uStack10 + 0x24);
        u_var5 = uStack24;
        pass1_1028_e0a0(ctx.PTR_LOOP_1050_65e2,
                        (u_var3 + uStack24 * 0x2) << 0x10, puVar9, param_4,
                        param_5);
        paStack34 = CONCAT22(puVar9, u_var5);
        pass1_1008_3f62(
            (uStack20 & 0xffff0000 | (uStack24 * 0x6 + uStack20)),
            CONCAT22(puVar9, u_var5 + 0x8));
        paStack40 = paStack34;
        paStack30 = paStack34;
        if (paStack34 != 0x0) {
            fn_ptr_1030_84d0(paStack34);
            fn_ptr_1000_17ce(ctx, paStack34, 0x1000);
        }
        uStack24 += 0x1;
        puVar9 = uStack20._2_2_;
    }
    // TODO: refactor for loop
    // for (iStack26 = 0x0; piVar2 = &i_var9.field_0x74,
    //     *piVar2 != iStack26 && iStack26 <= *piVar2; iStack26 += 0x1) {
    //   pass1_1008_3e94(
    //                   (uStack20 & 0xffff0000 |
    //                   (iStack26 * 0x6 + uStack20)),
    //                   CONCAT22(param_4,&local_2e),
    //                   CONCAT22(param_4,&local_30));
    //   iStack42 = pass1_1000_49b2(local_2e);
    //   iStack42 /= 0x5;
    //   if (0xc < iStack42) {
    //     iStack42 = 0xc;
    //     iVar6 = pass1_1000_49b2(local_2e);
    //     local_2e =
    //                (local_2e & 0xffff0000 |
    //                ((local_2e / iVar6) * 0x3c));
    //   }
    //   iVar7 = pass1_1000_49b2(local_2e);
    //   iVar6 = (iVar7 % 0x5);
    //   paStack34 =
    //               (paStack34 & 0xffff0000 | iVar7 % 0x5 & 0xffff);
    //   if (local_2e < 0x0) {
    //     if (0x2 < iVar6) {
    //       iVar6 += -0x5;
    //     }
    //     local_2e =
    //                (local_2e & 0xffff0000 | (local_2e + iVar6));
    //   }
    //   else {
    //     if (iVar6 < 0x3) {
    //       local_2e =
    //                  (local_2e & 0xffff0000 | (local_2e - iVar6));
    //     }
    //     else {
    //       local_2e =
    //                  (local_2e & 0xffff0000 | (local_2e + (0x5 - iVar6))
    //                  );
    //     }
    //   }
    //   iStack50 = local_30 / 0x16;
    //   for (iVar6 = 0x0; iVar6 < 0x10; iVar6 += 0x1) {
    //     if (0xf < iStack50) {
    //       iStack50 = 0x0;
    //     }
    //     if (((uStack16 != 0x0) < iStack50) && (iStack50 < 0x8)) {
    //       iVar7 = ((iStack42 * 0x10 + iStack50) * 0x2 + 0x11e0);
    //       iVar10 = ((iStack26 * 0x10 + iVar6) * 0x8);
    //       u_var3 = &i_var9.field_0x70;
    //       (iVar10 + u_var3) = iVar7 + 0x49;
    //       u_var3 = &i_var9.field_0x70;
    //       (iVar10 + u_var3 + 0x2) = local_2e + 0x49;
    //       u_var3 = &i_var9.field_0x70;
    //       (iVar10 + u_var3 + 0x4) = iVar7 + 0x4e;
    //       u_var3 = &i_var9.field_0x70;
    //       (iVar10 + u_var3 + 0x6) = local_2e + 0x4e;
    //     }
    //     else {
    //       iVar7 = (iStack26 * 0x10 + iVar6) * 0x8;
    //       u_var3 = &i_var9.field_0x70;
    //       (iVar7 + u_var3) = 0x0;
    //       u_var3 = &i_var9.field_0x70;
    //       (u_var3 + iVar7 + 0x2) = 0x0;
    //       u_var3 = &i_var9.field_0x70;
    //       (u_var3 + iVar7 + 0x4) = 0x1;
    //       u_var3 = &i_var9.field_0x70;
    //       (u_var3 + iVar7 + 0x6) = 0x1;
    //     }
    //     iStack50 += 0x1;
    //   }
    // }
    paStack40 = uStack20;
    local_2e = uStack20;
    fn_ptr_1000_17ce(ctx, uStack20, 0x1000);
    draw_1010_47ae(param_1, 0x1000, param_4);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_451a(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) -> u32

{
    let u_var1: u16;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let u_var4: u32;

    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_4, param_2, param_3);
    // u_var1 = (pu_var3 >> 0x10);
    u_var4 = pass1_1010_ec40(pu_var3, u_var1, (param_1 + 0x6c),
                             pu_var3, u_var1);
    // u_var2 = (u_var4 >> 0x10);
    return CONCAT22((u_var4 + 0x4), (u_var4 + 0x2));
}


pub fn pass1_1010_454a(param_1: u32) -> u32

{
    let i_var1: i16;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var2 = (i_var1 + 0x24) * 0x4;
    return CONCAT22((i_var1 + i_var2 + 0x28),
                    (i_var1 + i_var2 + 0x26));
}


pub fn pass1_1010_4566(param_1: i16, param_2: u16, param_3: i16, param_4: u16) {
    if (param_3 != 0x2) {
        return;
    }
    pass1_1010_4956(CONCAT22(param_2, param_1 + -0x20));
    pass1_1010_1f62(param_4, CONCAT22(param_2, param_1 + -0x20), 0x2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_459e(param_1: i32) {
    let u_var1: u16;
    let pu_var2: U32Ptr;

    if (param_1 == 0x0) {
        u_var1 = 0x0;
        pu_var2 = 0x0;
    } else {
        u_var1 = param_1 + 0x20;
        pu_var2 = param_1._2_2_;
    }
    pass1_1008_9262(ctx.PTR_LOOP_1050_0388, (ctx.PTR_LOOP_1050_0388 >> 0x10),
                    0x1f4, CONCAT22(pu_var2, u_var1), u_var1, pu_var2);
    (param_1 + 0x7e) = 0x1;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_45d6(param_1: i32, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let i_stack4: i16;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0x7e) != 0x0) {
        if (ctx.PTR_LOOP_1050_0388 != 0x0) {
            if (param_1 == 0x0) {
                i_var4 = 0x0;
                u_var5 = 0x0;
            } else {
                i_var4 = iVar6 + 0x20;
                u_var5 = uVar7;
            }
            param_2 = 0x1008;
            pass1_1008_92b2(ctx.PTR_LOOP_1050_0388, 0x1f4, CONCAT22(u_var5, i_var4));
        }
        // TODO: refactor for loop
        // for (i_stack4 = 0x0; i_stack4 < 0x10; i_stack4 += 0x1) {
        //   if ((iVar6 + 0x24) != i_stack4) {
        //     pu_var1 = (iVar6 + 0x26 + i_stack4 * 0x4);
        //     u_var2 = (iVar6 + 0x26 + i_stack4 * 0x4 + 0x2);
        //     if ((u_var2 | pu_var1) != 0x0) {
        //       ppc_var3 = *pu_var1;
        //       (**ppc_var3)(param_2,pu_var1,u_var2,0x1);
        //     }
        //     (iVar6 + i_stack4 * 0x4 + 0x26) = 0x0;
        //   }
        // }
        (iVar6 + 0x7e) = 0x0;
    }
    return;
}


pub fn pass1_1010_4674(param_1: u32, param_2: i16, param_3: u16, param_4: u16) {
    let pi_var1: U32Ptr;
    let UVar2: u32;
    let UVar3: u16;

    UVar2 = param_1;
    // UVar3 = (param_1 >> 0x10);
    if (param_2 == 0x1) {
        pi_var1 = (UVar2 + 0x24);
        *pi_var1 = *pi_var1 + 0x1;
        if (0xf < (UVar2 + 0x24)) {
            (UVar2 + 0x24) = 0x0;
        }
//LAB_1010_469a:
        draw_op_1010_47d0(UVar2, UVar3, (UVar2 + 0x24), param_3, param_4);
    } else {
        if (param_2 != 0x2) {
            if (param_2 != 0x3) {
                if (((UVar2 + 0x6a) != 0x0) && ((UVar2 + 0x6a) != 0x4)) {
                    pass1_1010_459e(param_1);
                }
//         TODO: goto LAB_1010_46e8;
            }
            pi_var1 = (UVar2 + 0x24);
            *pi_var1 = *pi_var1 + -0x1;
            if (*pi_var1 < 0x0) {
                (UVar2 + 0x24) = 0xf;
            }
//       TODO: goto LAB_1010_469a;
        }
    }
    pass1_1010_1f62(param_4, param_1, 0x2);
    pass1_1010_45d6(param_1, param_3);
//LAB_1010_46e8:
    (UVar2 + 0x6a) = param_2;
    return;
}


pub fn pass1_1010_4788(param_1: u32, param_2: &mut String, param_3: u16, param_4: u16) {
    pass1_1030_8344(
        ctx, ctx.PTR_LOOP_1050_5748, (param_1 + 0x6c));
    pass1_1030_301a(CONCAT22(param_4, param_3), param_2, param_4);
    return;
}


pub fn pass1_1010_4956(param_1: u32) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    i_var2 = (i_var3 + 0x6a);
    if (i_var2 == 0x0) {
        pi_var1 = (i_var3 + 0x24);
        *pi_var1 = *pi_var1 + 0x1;
        if (0xf < (i_var3 + 0x24)) {
            (i_var3 + 0x24) = 0x0;
            return;
        }
    } else {
        if (i_var2 != 0x4) {
            return;
        }
        pi_var1 = (i_var3 + 0x24);
        *pi_var1 = *pi_var1 + -0x1;
        if (*pi_var1 < 0x0) {
            (i_var3 + 0x24) = 0xf;
        }
    }
    return;
}



pub fn pass1_1010_4994(param_1: u16,param_2: & mut Struct18,param_3: u8,param_4: u16) -> &mut Struct18

{
param_2 = (param_2 & 0xffff0000 | (param_2 - 0x20)); pass1_1010_3f00(param_2, param_4); if ((param_3 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_2, 0x1000);
}
return param_2;
}



pub fn pass1_1010_49a0(param_1: i16, param_2: u16) -> u32

{
    return CONCAT22(param_2, param_1 + 0xa);
}


pub fn pass1_1010_49b0(param_1: i16, param_2: u16) -> u32

{
    return CONCAT22(param_2, param_1 + 0x18);
}


pub fn pass1_1010_49c0(param_1: u32) -> u16

{
    return (param_1 + 0x14);
}


pub fn pass1_1010_49ce(param_1: u32, param_2: u16) {
    (param_1 + 0x14) = param_2;
    return;
}


pub fn pass1_1010_49e0(param_1: u32) -> u16

{
    return (param_1 + 0x16);
}


pub fn pass1_1010_49ee(param_1: u32, param_2: u16) {
    (param_1 + 0x16) = param_2;
    return;
}


pub fn pass1_1010_4a00(param_1: u32, param_2: u16) {
    (param_1 + 0x12) = param_2;
    return;
}


pub fn pass1_1010_4a12(param_1: u32) -> u16

{
    return (param_1 + 0x12);
}


pub fn pass1_1010_4a20(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_3f00(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_4a8a(param_1: &mut Struct637, param_2: &mut Struct19, param_3: u16, param_4: u16) {
    let pu_var1: U32Ptr;
    let unaff_DI: i16;
    let paVar2: &mut Struct43;
    let pu_var3: U32Ptr;

    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0x16 = 0x0;
    param_1.field_0x1a = 0x0;
    param_1.field_0x1e = 0x0;
    param_1.field_0x20 = 0x1;
    param_1.field_0x22 = 0x0;
    param_1.field_0x24 = 0x0;
    &param_1.field_0x26 = 0x0;
    param_1.field_0x2a = 0x0;
    param_1.field_0x2c = 0x1;
    param_1.field_0x2e = 0x0;
    param_1.field_0x30 = 0x0;
    param_1.field_0x32 = 0x0;
    CONCAT22(param_2, param_1) = (s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6);
    param_1.field_0x2 = 0x1010;
    paVar2 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x1b3, param_4);
    // pu_var1 = (paVar2 >> 0x10);
    &param_1.field_0x16 = paVar2;
    (&param_1.field_0x16 + 0x2) = pu_var1;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_4, pu_var1, unaff_DI);
    param_1.field_0x26 = pu_var3;
    param_1.field_0x28 = (pu_var3 >> 0x10);
    pass1_1008_4772(param_1.field_0x16);
    param_1.field_0xe = 0x13c;
    param_1.field_0xa = 0x0;
    param_1.field_0x10 = 0x0;
    param_1.field_0xc = 0x0;
    return;
}


pub fn pass1_1010_4c2c(param_1: u32) -> u32

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x18),
                    (param_1 + 0x16));
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_4c3e(param_1: u32, param_2: i16, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let i_var3: i16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let paVar7: &mut Struct43;
    let uVar8: u32;
    let iStack14: i16;
    let local_c: [u8; 6];
    let uStack6: u16;
    let i_stack4: i16;

    // u_var5 = (param_1 >> 0x10);
    i_var3 = param_1;
    pass1_1010_bffa((i_var3 + 0x26), param_3, param_4, param_5);
    (i_var3 + 0x12) = param_3;
    (i_var3 + 0x14) = param_4;
    if ((param_4 | (i_var3 + 0x12)) != 0x0) {
        if (param_2 == 0x0) {
            u_var2 = (i_var3 + 0x12);
            (i_var3 + 0x30) = (u_var2 + 0x8);
        } else {
            (i_var3 + 0x2e) = 0x1;
            u_var2 = (i_var3 + 0x12);
            u_var2 = (u_var2 + 0x4);
            i_var4 = (u_var2 + 0x2);
            if ((i_var4 == 0x5) || (i_var4 == 0x6)) {
                (i_var3 + 0x30) = 0x1;
                (i_var3 + 0x20) = 0x0;
            } else {
                (i_var3 + 0x30) = 0x2;
                u_var2 = (i_var3 + 0x12);
                (i_var3 + 0x32) = (u_var2 + 0x4);
                paVar7 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x1bf, param_5);
                u_var2 = (i_var3 + 0x12);
                // u_var6 = (u_var2 >> 0x10);
                i_var4 = u_var2;
                (i_var4 + 0x4) = paVar7;
                (i_var4 + 0x6) = (paVar7 >> 0x10);
            }
        }
        i_stack4 = 0x14;
        clear_struct_1008_3e38(CONCAT22(param_5, local_c));
        uStack6 = 0x0;
        iStack14 = 0x0;
        loop {
            pi_var1 = (i_var3 + 0x30);
            if (*pi_var1 == iStack14 || *pi_var1 < iStack14) { break; }
            u_var2 = (i_var3 + 0x12);
            uVar8 = pass1_1008_4772((u_var2 + iStack14 * 0x4));
            i_stack4 += (-(iStack14 == 0x0) & 0x5) + 0x14 + (uVar8 + 0x4);
            iStack14 += 0x1;
        }
        if ((i_var3 + 0xe) < i_stack4) {
            (i_var3 + 0xe) = i_stack4;
        }
    }
    return;
}


pub fn pass1_1010_4dc8(param_1: u32) -> u32

{
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x20) == 0x0) {
        return 0x0;
    }
    return CONCAT22((i_var1 + 0x1c),
                    (i_var1 + 0x20) * 0x8 + (i_var1 + 0x1a));
}


pub fn pass1_1010_4df0(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x26);
    pass1_1010_c1ba(u_var1, (u_var1 >> 0x10),
                    (param_1 + 0x20), param_2, param_3);
    return;
}


pub fn pass1_1010_4e8c(param_1: u32, param_2: u16) {
    pass1_1010_1f62(param_2, param_1, 0xd);
    return;
}


pub fn pass1_1010_4f20(param_1: u16, param_2: u16, param_3: i16) -> u16

{
    return (param_3 * 0x2 + 0x139a);
}


pub fn pass1_1010_4f30(param_1: U32Ptr, param_3: U32Ptr, param_4: U32Ptr) {
    *param_4 = 0xa;
    *param_3 = 0x73;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_4f48(param_1: u32, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let puVar4: u32;
    let u_var5: u32;
    let iVar6: &mut Struct482;
    let iVar7: &mut Struct483;
    let u_var6: u16;
    let uVar7: u16;
    let paVar8: &mut Struct43;

    // u_var6 = (param_1 >> 0x10);
    iVar6 = param_1;
    puVar4 = iVar6.field_0x12;
    iVar6.field_0x30 = (puVar4 + 0x8);
    if (iVar6.field_0x32 != 0x0) {
        u_var5 = *iVar6.field_0x12;
        // uVar7 = (u_var5 >> 0x10);
        iVar7 = u_var5;
        puVar4 = iVar7.field_0x4;
        iVar7.field_0x4 = iVar6.field_0x32;
        if (puVar4 != 0x0) {
            ppc_var3 = *puVar4;
            (**ppc_var3)();
        }
        iVar6.field_0x32 = 0x0;
    }
    pu_var1 = iVar6.field_0x16;
    u_var2 = iVar6.field_0x18;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    paVar8 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x1b3, param_2);
    iVar6.field_0x16 = paVar8;
    iVar6.field_0x18 = (paVar8 >> 0x10);
    fn_ptr_1000_17ce(ctx, iVar6.field_0x1a, 0x1000);
    iVar6.field_0x1a = 0x0;
    iVar6.field_0x2e = 0x0;
    return;
}


pub fn pass1_1010_5004(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    free_rsrc_1010_4b3e(ctx, param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_503e(param_1: &mut Struct19, param_2: &mut Struct19, param_3: u16, param_4: U32Ptr, param_5: u16) {
    struct_op_1018_4cda(param_1, param_2, param_3);
    CONCAT22(param_2, param_1) = (s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1);
    (param_1 + 0x2) = 0x1010;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x1b3, param_4, param_5);
    ctx._PTR_LOOP_1050_4230 = CONCAT22(param_2, param_1);
    return;
}



pub fn  pass1_1010_5074(param_1: & mut Struct11,param_2: u8) -> &mut Struct11

{
clenaup_win_ui_1018_4d22(param_1, 0x1018); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1010_50f2(param_1: U32Ptr, param_2: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x53f4;
    (param_1 + 0x2) = 0x1010;
    fn_ptr_1000_17ce(ctx, (param_1 + 0xc), 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_5120(param_1: u32, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;
    let u_var6: u16;
    let uVar7: u16;
    let i_var8: i16;
    let i_var9: i16;
    let u_var10: u16;

    // u_var10 = (param_1 >> 0x10);
    i_var9 = param_1;
    if ((i_var9 + 0x16) != 0x0) {
        u_var1 = (i_var9 + 0x16);
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
        u_var6 = param_4 | param_3;
        if (u_var6 != 0x0) {
            u_var2 = (param_3 + 0x1f6);
            u_var5 = u_var2;
            pass1_1030_38f2(u_var2, 0x3, param_5);
            u_var3 = u_var5;
            uVar7 = u_var6;
            u_var4 = u_var3;
            pass1_1030_38f2(u_var2, 0x4, param_5);
            i_var8 = uVar7 + u_var6 + CARRY2(u_var4, u_var3);
            if ((0x0 < i_var8) || ((-0x1 < i_var8 && (param_2 <= u_var4 + u_var3)))) {
                (i_var9 + 0xa) = param_2;
                return;
            }
        }
    }
    return;
}


pub fn pass1_1010_519a(param_1: u32, param_2: &mut i16, param_3: U32Ptr, param_4: u16) {
    let u_var1: u32;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let iVar5: &mut Struct246;
    let iVar6: &mut Struct247;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u16;
    let piStack44: U32Ptr;
    let local_18:[u8;0xc];
    let iStack12: i16;
    let uStack6: u32;

    uStack6 = 0x0;
    pass1_1028_dc52(CONCAT22(param_4, local_18), 0x1, 0x0, 0x400);
    // uVar8 = (param_1 >> 0x10);
    iVar5 = param_1;
    iVar5.field_0x10 = iStack12;
    fn_ptr_1000_17ce(ctx, &iVar5.field_0xc, 0x1000);
    u_var2 = iVar5.field_0x10 << 0x2;
    mem_op_1000_179c(u_var2, param_3, 0x1000);
    iVar5.field_0xc = u_var2;
    iVar5.field_0xe = param_3;
    iVar5.field_0x10 = 0x0;
    loop {
        puVar4 = param_3;
        pu_var3 = local_18;
        pass1_1028_e4ec(CONCAT22(param_4, pu_var3));
        uStack6 = CONCAT22(puVar4, pu_var3);
        if ((puVar4 | pu_var3) == 0x0) { break; }
        param_3 = (puVar4 | pu_var3);
        if ((pu_var3 + 0x200) != 0x8000002) {
            param_3 = (pu_var3 + 0x6);
            u_var1 = &iVar5.field_0xc;
            // uVar9 = (u_var1 >> 0x10);
            iVar7 = u_var1;
            iVar6 = (iVar5.field_0x10 * 0x4);
            piStack44 = (param_1 & 0xffff0000 | ZEXT24(&iVar5.field_0x10));
            (iVar6 + iVar7) = (pu_var3 + 0x4);
            (iVar6 + iVar7 + 0x2) = param_3;
            *piStack44 = *piStack44 + 0x1;
        }
    }
    *param_2 = iVar5.field_0x10;
    return;
}


pub fn pass1_1010_52fc(param_1: u32, param_2: u32, param_3: u16, param_4: U32Ptr, param_5: u16) {
    let u_var1: u16;

    pass1_1010_533c(param_1, param_2, param_4, param_5);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x12) = param_3;
    (param_1 + 0x14) = param_4;
    return;
}


pub fn pass1_1010_531c(param_1: u32, param_2: u32, param_3: u16, param_4: U32Ptr, param_5: u16) {
    let u_var1: u16;

    pass1_1010_533c(param_1, param_2, param_4, param_5);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x16) = param_3;
    (param_1 + 0x18) = param_4;
    return;
}


pub fn pass1_1010_533c(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let paVar7: &mut Struct18;
    let uStack6: u16;
    let local_4: [u8; 2];

    pass1_1010_519a(param_1, CONCAT22(param_4, local_4), param_3, param_4);
    uStack6 = 0x0;
    loop {
        // u_var6 = (param_1 >> 0x10);
        u_var5 = param_1;
        pu_var1 = (u_var5 + 0x10);
        if (*pu_var1 < uStack6 || *pu_var1 == uStack6) {
            return;
        }
        u_var3 = (u_var5 + 0xc);
        u_var2 = (u_var3 + uStack6 * 0x4);
        paVar7 = string_1010_5286(u_var5, u_var6, u_var2, u_var2, param_3);
        // param_3 = (paVar7 >> 0x10);
        i_var4 = pass1_1000_3d7a(param_2, paVar7 & 0xffff | ZEXT24(param_3) << 0x10);
        if (i_var4 == 0x0) { break; }
        fn_ptr_1000_17ce(ctx, paVar7, 0x1000);
        uStack6 += 0x1;
    }
    return;
}


pub fn pass1_1010_53ce(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_50f2(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_5d9c(param_1: u32, param_2: i16, param_3: U32Ptr, param_4: i16, param_5: u16) {
    let pu_var1: U32Ptr;

    (param_1 + 0x1e) = param_2;
    if (param_2 == 0x0) {
        pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2e, param_5, param_3, param_4);
        pass1_1018_209c(pu_var1);
    }
    return;
}


pub fn pass1_1010_5dc6(param_1: u32, param_2: u32, param_3: u16) -> u16

{
    let b_var1: bool;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u16;
    u8 * local_c[0x3];
    let local_6: [u16; 0x2];

    b_var1 = write_to_file_1008_7cac(param_2, param_3);
    if (b_var1 != 0x0) {
        // u_var3 = (param_1 >> 0x10);
        i_var2 = param_1;
        b_var1 = pass1_1008_7c2a(param_2, (i_var2 + 0x68), 0x1008);
        if (b_var1 != 0x0) {
            b_var1 = pass1_1008_7c2a(param_2, (i_var2 + 0x6c), 0x1008);
            if (b_var1 != 0x0) {
                local_c[0] = ctx.PTR_LOOP_1050_13ae;
                // u_var4 = (param_2 >> 0x10);
                b_var1 = write_to_file_1008_7e1c(param_2, u_var4, local_c, param_3, 0x2,
                                                 0x1008);
                if (b_var1 != 0x0) {
                    local_6[0] = (i_var2 + 0x82);
                    b_var1 = write_to_file_1008_7e1c(param_2, u_var4, local_6, param_3, 0x2,
                                                     0x1008);
                    if (b_var1 != 0x0) {
                        return 0x1;
                    }
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return 0x0;
}


pub fn pass1_1010_5e56(param_1: u32, param_2: u32, param_3: i16, param_4: u16, param_5: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let BVar3: bool;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let local_404: U32Ptr;
    let local_402: [u8; 400];

    u_var6 = param_2;
    // uVar7 = (param_2 >> 0x10);
    read_file_1008_7cfe(u_var6, uVar7, 0x4, 0x1008, param_5);
    if (param_3 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d4;
    } else {
        pu_var1 = local_402;
        read_file_1008_7c6e(u_var6, uVar7, CONCAT22(param_5, pu_var1), 0x1008);
        if (pu_var1 != 0x0) {
            u_var2 = str_op_1008_60e8(CONCAT22(param_5, local_402), param_4);
            // u_var5 = (param_1 >> 0x10);
            i_var4 = param_1;
            (i_var4 + 0x68) = u_var2;
            (i_var4 + 0x6a) = param_4;
            pu_var1 = local_402;
            read_file_1008_7c6e(u_var6, uVar7, CONCAT22(param_5, pu_var1), 0x1008);
            if (pu_var1 != 0x0) {
                u_var2 = str_op_1008_60e8(CONCAT22(param_5, local_402), param_4);
                (i_var4 + 0x6c) = u_var2;
                (i_var4 + 0x6e) = param_4;
                BVar3 = read_file_1008_7dee(u_var6, uVar7, &local_404, 0x0, param_5, 0x2, 0x1008);
                if (BVar3 != 0x0) {
                    ctx.PTR_LOOP_1050_13ae = local_404;
                    if (ctx.PTR_LOOP_1050_0312 < 0x2) {
                        return;
                    }
                    BVar3 = read_file_1008_7dee(u_var6, uVar7, i_var4 + 0x82, 0x0, u_var5, 0x2, 0x1008);
                    if (BVar3 != 0x0) {
                        return;
                    }
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}


pub fn pass1_1010_5f4c(param_1: u32, param_2: &mut String, param_3: u16) {
    let u_var1: u16;
    let i_var3: &mut Struct484;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var3 = param_1;
    fn_ptr_1000_17ce(ctx, &i_var3.field_0x12, 0x1000);
    u_var1 = str_op_1008_60e8(param_2, param_3);
    i_var3.field_0x12 = u_var1;
    i_var3.field_0x14 = param_3;
    return;
}


pub fn pass1_1010_5f7a(param_1: i16, param_2: u16, param_3: u16, param_4: i16) -> u32

{
    let i_var1: i16;

    i_var1 = param_4 * 0x8 + param_1;
    if (((i_var1 + 0x26) == 0x0) && ((i_var1 + 0x28) == 0x0)) {
        return 0x0;
    }
    return CONCAT22(param_2, param_4 * 0x8 + param_1 + 0x22);
}


pub fn pass1_1010_5fb0(param_1: u32, param_2: u16, param_3: U32Ptr, param_4: u16, param_5: i16) {
    let u_var1: u16;
    let i_var1: &mut Struct656;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = (param_1 + param_5 * 0x8);
    i_var1.field_0x22 = *param_3;
    i_var1.field_0x26 = param_3[0x1];
    return;
}


pub fn pass1_1010_5fd8(param_1: u32, param_2: &mut String, param_3: u16) {
    let u_var1: u16;
    let i_var3: &mut Struct485;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var3 = param_1;
    fn_ptr_1000_17ce(ctx, &i_var3.field_0x68, 0x1000);
    u_var1 = str_op_1008_60e8(param_2, param_3);
    i_var3.field_0x68 = u_var1;
    i_var3.field_0x6a = param_3;
    return;
}


pub fn pass1_1010_6006(param_1: u32, param_2: &mut String, param_3: u16) {
    let u_var1: u16;
    let i_var3: &mut Struct486;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var3 = param_1;
    fn_ptr_1000_17ce(ctx, &i_var3.field_0x6c, 0x1000);
    u_var1 = str_op_1008_60e8(param_2, param_3);
    i_var3.field_0x6c = u_var1;
    i_var3.field_0x6e = param_3;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_6034(param_1: u32, param_2: u16) {
    let pu_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    (i_var2 + 0x1e) = 0x1;
    (i_var2 + 0x20) = 0x1;
    (i_var2 + 0x72) = 0x1;
    (i_var2 + 0x74) = 0x1;
    pass1_1010_60a0(param_1);
    pu_var1 = pass1_1000_4906((param_1 & 0xffff0000 | (i_var2 + 0x22)),
                              0x0, 0x40);
    load_string_1010_84ac(ctx.PTR_LOOP_1050_14cc, 0x1000);
    (i_var2 + 0x68) = pu_var1;
    (i_var2 + 0x6a) = param_2;
    load_string_1010_84ac(ctx.PTR_LOOP_1050_14cc, 0x1000);
    (i_var2 + 0x6c) = pu_var1;
    (i_var2 + 0x6e) = param_2;
    return;
}


pub fn pass1_1010_60a0(param_1: u32) {
    (param_1 + 0x76) = 0x5;
    return;
}


pub fn pass1_1010_60b4() -> u16

{
    return 0x1;
}


pub fn pass1_1010_60ba() -> u16

{
    return 0x1;
}


pub fn pass1_1010_60c0() -> u16

{
    return 0x1;
}


pub fn pass1_1010_60c6() -> u16

{
    return 0x1;
}


pub fn pass1_1010_60cc(param_1: u32, param_2: &mut String, param_3: u16) {
    let u_var1: u16;
    let i_var3: &mut Struct487;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var3 = param_1;
    fn_ptr_1000_17ce(ctx, &i_var3.field_0x1a, 0x1000);
    u_var1 = str_op_1008_60e8(param_2, param_3);
    i_var3.field_0x1a = u_var1;
    i_var3.field_0x1c = param_3;
    return;
}


pub fn pass1_1010_60fa(param_1: u32) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x7e) = 0x1;
    (i_var1 + 0x7c) = (i_var1 + 0x20);
    (i_var1 + 0x20) = 0x1;
    return;
}


pub fn pass1_1010_6118(param_1: &mut Struct19) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x7e) != 0x0) {
        (i_var1 + 0x20) = (i_var1 + 0x7c);
    }
    return;
}


pub fn pass1_1010_62a4(param_1: U32Ptr, param_2: u8) {
    let u_var2: &mut Struct488;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    u_var2 = param_1;
    *param_1 = 0x6322;
    u_var2.field_0x2 = 0x1010;
    fn_ptr_1000_17ce(ctx, u_var2.field_0x4, 0x1000);
    *param_1 = 0x389a;
    u_var2.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}


pub fn pass1_1010_62ec(param_1: U32Ptr, param_2: u8) -> u16

{
    write_private_profile_str_1010_5b10(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_648a(param_1: u32, param_2: u32, param_3: i16, param_4: u16) {
    let u_var1: u16;
    let i_var2: i16;
    let BVar3: bool;
    let u_var4: u16;
    let u_var5: u16;

    u_var4 = param_2;
    // u_var5 = (param_2 >> 0x10);
    read_file_1008_7cfe(u_var4, u_var5, 0x7, 0x1008, param_4);
    if (param_3 != 0x0) {
        i_var2 = param_1;
        // u_var1 = (param_1 >> 0x10);
        BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0xa, 0x0, u_var1, 0x4, 0x1008);
        if (BVar3 != 0x0) {
            BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0xe, 0x0, u_var1, 0x4, 0x1008);
            if (BVar3 != 0x0) {
                BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x12, 0x0, u_var1, 0x4, 0x1008);
                if (BVar3 != 0x0) {
                    BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x16, 0x0, u_var1, 0x4, 0x1008);
                    if (BVar3 != 0x0) {
                        BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x1a, 0x0, u_var1, 0x4, 0x1008);
                        if (BVar3 != 0x0) {
                            BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x1e, 0x0, u_var1, 0x4, 0x1008);
                            if (BVar3 != 0x0) {
                                BVar3 = read_file_1008_7dee(u_var4, u_var5, i_var2 + 0x22, 0x0, u_var1, 0x4, 0x1008);
                                if (BVar3 != 0x0) {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}


pub fn pass1_1010_6566(param_1: u32, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let local_4: i16;

    u_var1 = param_1;
    // u_var2 = (param_1 >> 0x10);
    switch_1010_6646(u_var1, u_var2, CONCAT22(param_5, &local_4), param_4);
    if (local_4 != 0x0) {
        (u_var1 + local_4) = param_3;
        (u_var1 + local_4 + 0x2) = param_2;
    }
    return;
}



pub fn  pass1_1010_659a(param_1: u32,param_2: u16,param_3: u16) -> i16

{
let u_var1: u16; let u_var2: u16;
let local_4: i16;

u_var1 = param_1;
// u_var2 = (param_1 >> 0x10);
switch_1010_6646(u_var1, u_var2, CONCAT22(param_3, &local_4), param_2); if (local_4 == 0x0) {
return 0x0;
}
return (u_var1 + local_4) - (u_var1 + local_4 + 0x2);
}



pub fn pass1_1010_65d0(param_1: u16, param_2: u32, param_3: u16) -> u16

{
    let u_var1: u16;
    let local_4: i16;

    // u_var1 = (param_2 >> 0x10);
    switch_1010_6646(param_2, u_var1, CONCAT22(param_1, &local_4), param_3);
    if (local_4 == 0x0) {
        return 0x0;
    }
    return (param_2 + local_4 + 0x2);
}


pub fn pass1_1010_6604(param_1: u32, param_2: u16, param_3: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u16;
    let local_4: i16;

    u_var2 = param_1;
    // u_var3 = (param_1 >> 0x10);
    switch_1010_6646(u_var2, u_var3, CONCAT22(param_3, &local_4), param_2);
    if (local_4 != 0x0) {
        i_var1 = (u_var2 + local_4 + 0x2);
        (u_var2 + local_4) = (u_var2 + local_4);
        (u_var2 + local_4 + 0x2) = i_var1 + 0x1;
        pass1_1010_1f62(param_3, param_1 & 0xffff | u_var3 << 0x10, 0x15);
    }
    return;
}


pub fn pass1_1010_66ca(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_1d80(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_6700(param_1: &mut Struct636, param_2: &mut Struct19, param_3: u16) -> &mut Struct19

{
    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0x148 = 0x33;
    CONCAT22(param_2, param_1) = 0x6aac;
    param_1.field_0x2 = 0x1010;
    pass1_1000_4906(CONCAT22(param_2, &param_1.field_0xa), 0x0,
                    0x114);
    param_1.field_0x32 = 0x1;
    param_1.field_0x40 = 0x1;
    param_1.field_0x46 = 0x1;
    param_1.field_0x4e = 0x1;
    param_1.field_0x54 = 0x1;
    param_1.field_0x5e = 0x1;
    param_1.field_0x68 = 0x1;
    param_1.field_0x6c = 0x1;
    param_1.field_0x74 = 0x1;
    param_1.field_0x78 = 0x1;
    param_1.field_0x7a = 0x1;
    param_1.field_0x7e = 0x1;
    param_1.field_0x82 = 0x1;
    param_1.field_0xa2 = 0x1;
    param_1.field_0xa4 = 0x1;
    param_1.field_0xa6 = 0x1;
    param_1.field_0xa8 = 0x1;
    param_1.field_0xae = 0x1;
    param_1.field_0xb2 = 0x1;
    param_1.field_0xb8 = 0x1;
    param_1.field_0xbe = 0x1;
    param_1.field_0xc0 = 0x1;
    param_1.field_0xc4 = 0x1;
    param_1.field_0xd4 = 0x1;
    param_1.field_0xda = 0x1;
    param_1.field_0xe2 = 0x1;
    param_1.field_0xfe = 0x1;
    param_1.field_0x100 = 0x1;
    param_1.field_0x102 = 0x1;
    param_1.field_0x104 = 0x1;
    param_1.field_0x106 = 0x1;
    param_1.field_0x108 = 0x1;
    pass1_1000_4906(CONCAT22(param_2, &param_1.field_0x11e), 0x0,
                    0x2a);
    param_1.field_0x120 = 0x1;
    param_1.field_0x122 = 0x1;
    param_1.field_0x124 = 0x1;
    param_1.field_0x126 = 0x1;
    param_1.field_0x128 = 0x1;
    param_1.field_0x12c = 0x1;
    param_1.field_0x138 = 0x1;
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1010_6814(param_1: u32, param_2: u16, param_3: i16) {
    (param_1 + param_3 * 0x2 + 0x11e) = param_2;
    return;
}


pub fn pass1_1010_682e(param_1: u32, param_2: u16, param_3: i16) {
    (param_1 + param_3 * 0x2 + 0xa) = param_2;
    return;
}


pub fn pass1_1010_68c6(param_1: u32, param_2: u32, param_3: u16, param_4: U32Ptr, param_5: u16) {
    let i_var2: &mut Struct248;
    let b_var1: bool;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let uVar7: u16;
    let uVar8: u16;
    let SVar9: SEGPTR;
    let u_var10: u16;
    let paStack18: &mut Struct18;
    let paStack10: &mut Struct18;
    let local_6: i16;
    let uStack4: u16;

    uVar8 = param_2;
    // u_var10 = (param_2 >> 0x10);
    read_file_1008_7cfe(uVar8, u_var10, 0x3, 0x1008, param_5);
    if (param_3 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d4;
        return;
    }
    i_var2 = param_1;
    // uVar7 = (param_1 >> 0x10);
    if (ctx.PTR_LOOP_1050_0312 < 0x2) {
        u_var4 = 0x102;
        SVar9 = 0x102;
        mem_op_1000_179c(0x102, param_4, 0x1000);
        paStack10 = CONCAT22(param_4, param_3);
        puVar6 = param_4;
        b_var1 = read_file_1008_7dee(uVar8, u_var10, param_3, u_var4, param_4, SVar9, 0x1008);
        paStack18 = paStack10;
        if (b_var1 == 0x0) {
            // goto
            // LAB_1010_692c;
        }
        uStack4 = 0x1;
        loop {
            i_var3 = switch_1008_73ea(uVar8, u_var10, uStack4);
            (&i_var2.field_0xa + i_var3 * 0x2) = (uStack4 * 0x2 + param_3);
            uStack4 += 0x1;
            if (uStack4 < 0x81) { break; }
        }
        fn_ptr_1000_17ce(ctx, paStack10, 0x1000);
        u_var4 = paStack10;
        param_4 = puVar6;
    } else {
        u_var4 = read_file_1008_7dee(uVar8, u_var10, &i_var2.field_0xa, 0x0, uVar7, 0x114, 0x1008);
        if (u_var4 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d2;
            return;
        }
    }
    if (ctx.PTR_LOOP_1050_0312 < 0x2) {
        u_var5 = 0x2a;
        SVar9 = 0x2a;
        mem_op_1000_179c(0x2a, param_4, 0x1000);
        paStack18 = CONCAT22(param_4, u_var4);
        b_var1 = read_file_1008_7dee(uVar8, u_var10, u_var4, u_var5, param_4, SVar9, 0x1008);
        if (b_var1 == 0x0) {
//LAB_1010_692c:
            ctx.PTR_LOOP_1050_0310 = 0x6d2;
            fn_ptr_1000_17ce(
                ctx, (paStack18 & 0xffff | ZEXT24(param_4) << 0x10), 0x1000);
            return;
        }
        uStack4 = 0x0;
        loop {
            u_var5 = switch_1008_72bc(uVar8, u_var10, uStack4);
            (&i_var2.field_0x11e + u_var5 * 0x2) = (uStack4 * 0x2 + u_var4);
            uStack4 += 0x1;
            if (uStack4 < 0x15) == false { break; }
        }
        fn_ptr_1000_17ce(ctx, paStack18, 0x1000);
    } else {
        b_var1 = read_file_1008_7dee(uVar8, u_var10, &i_var2.field_0x11e, 0x0, uVar7, 0x2a, 0x1008);
        if (b_var1 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d2;
            return;
        }
    }
    b_var1 = read_file_1008_7dee(uVar8, u_var10, &local_6, 0x0, param_5, 0x2, 0x1008);
    if (b_var1 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
        return;
    }
    b_var1 = switch_1008_73ea(uVar8, u_var10, local_6);
    i_var2.field_0x148 = b_var1;
    return;
}


pub fn pass1_1010_6a86(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_1d80(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_6abc(param_1: &mut Struct635, param_2: &mut Struct19, param_3: u16) {
    let ppcVar1: u32;
    let extraout_dx: U32Ptr;
    let unaff_DI: i16;
    let unaff_SS: u16;
    let paVar2: &mut Struct79;
    let pu_var3: U32Ptr;

    paVar2 = struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0xa = 0x389a;
    param_1.field_0xc = 0x1008;
    param_1.field_0xa = 0x3aa8;
    param_1.field_0xc = 0x1008;
    param_1.field_0xe = 0x0;
    param_1.field_0x10 = 0x0;
    param_1.field_0x14 = 0x0;
    param_1.field_0x1c = 0x0;
    param_1.field_0x20 = 0x0;
    param_1.field_0x22 = 0x0;
    CONCAT22(param_2, param_1) = 0x7e28;
    param_1.field_0x2 = 0x1010;
    param_1.field_0xa = 0x7e38;
    param_1.field_0xc = 0x1010;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, unaff_SS,
                              (paVar2 >> 0x10), unaff_DI);
    &param_1.field_0x14 = pu_var3;
    (&param_1.field_0x14 + 0x2) = (pu_var3 >> 0x10);
    ppcVar1 = (*param_1.field_0x14 + 0x4);
    (**ppcVar1)();
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, unaff_SS, extraout_dx, unaff_DI);
    &param_1.field_0x22 = pu_var3;
    (&param_1.field_0x22 + 0x2) = (pu_var3 >> 0x10);
    ppcVar1 = (*param_1.field_0x22 + 0x4);
    (**ppcVar1)();
    return;
}


pub fn pass1_1010_6bb2(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let puStack14: U32Ptr;

    // uVar7 = (param_1 >> 0x10);
    u_var6 = param_1;
    *param_1 = 0x7e28;
    (u_var6 + 0x2) = 0x1010;
    (u_var6 + 0xa) = 0x7e38;
    (u_var6 + 0xc) = 0x1010;
    pu_var1 = (u_var6 + 0x1c);
    u_var3 = (u_var6 + 0x1e);
    if ((u_var3 | pu_var1) != 0x0) {
        ppcVar2 = *pu_var1;
        (**ppcVar2)();
    }
    (u_var6 + 0x1c) = 0x0;
    if ((u_var6 + 0x14) != 0x0) {
        u_var3 = uVar7 | u_var6;
        if (param_1 == 0x0) {
            u_var5 = 0x0;
        } else {
            u_var3 = u_var6 + 0xa;
            u_var5 = uVar7;
        }
        pass1_1010_1ea6((u_var6 + 0x14), CONCAT22(u_var5, u_var3), param_2);
    }
    if ((u_var6 + 0x22) != 0x0) {
        u_var3 = uVar7 | u_var6;
        if (param_1 == 0x0) {
            u_var5 = 0x0;
        } else {
            u_var3 = u_var6 + 0xa;
            u_var5 = uVar7;
        }
        pass1_1010_1ea6((u_var6 + 0x22), CONCAT22(u_var5, u_var3), param_2);
    }
    (u_var6 + 0x14) = 0x0;
    (u_var6 + 0x22) = 0x0;
    if (param_1 == 0x0) {
        i_var4 = 0x0;
        uVar7 = 0x0;
    } else {
        i_var4 = u_var6 + 0xa;
    }
    puStack14 = CONCAT22(uVar7, i_var4);
    *puStack14 = 0x389a;
    (i_var4 + 0x2) = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1010_6ca2(param_1: u32, param_2: i16, param_3: u16, param_4: u16) -> u16

{
    let u_var1: u32;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let uStack10: i16;
    let iStack8: i16;
    let uStack4: u16;

    uStack4 = 0x1;
    _iStack8 = CONCAT22(param_4, &stack0x000a);
    iStack10 = param_2;
    loop {
        pu_var2 = _iStack8;
        if (iStack10 == 0x0) {
            return uStack4;
        }
        _iStack8 = (_iStack8 & 0xffff0000 | (iStack8 + 0x2));
        u_var3 = *pu_var2;
        u_var1 = (param_1 + 0x14);
        iStack10 = iStack10 + -0x1;
        pass1_1010_a5ca(u_var1, (u_var1 >> 0x10), u_var3, u_var3, param_3);
        if (u_var3 == 0x0) == false { break; }
    }
    return 0x0;
}



pub fn pass1_1010_6cf8(
    param_1: u16,
    param_2: u32,
    param_3: i16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16) -> u16

{
let u_var1: u16;

if (false) {
// code_r0x1010703e: 
return 0x0;
}
match(param_3) {
0x1 => {pass1_1010_715c(param_2, 0x1, param_6,param_5, param_7, param_4); send_msg_1010_7c9e(param_2, 0x12, param_4); return 0x1; 
}
// default:
//     TODO: goto code_r0x1010703e;
_ => {}
0x4 => {u_var1 = 0x2;} 
0x5 => {u_var1 = 0x3;} 
0x6 => {u_var1 = 0x4;} 
 0x7 => {u_var1 = 0x5;} 
 0x9 => {pass1_1010_715c(param_2, 0x6, param_6, param_5, param_7, param_4);}
0x2e => {u_var1 = 0x38;} 
0xa => {}
0x80 => {u_var1 = 0x2d;} 
0xb =>{
u_var1 = 0x7;} 
0xc => {}
0x17 => {}
0x18 => {}
0x19 => {}
0x21 => {}
0x75 => {}
0x81 => {
if (param_3 == 0x75) {
pass1_1010_715c(param_2, 0x8, param_6, param_5,param_7, param_4); pass1_1010_715c(param_2, 0x9, param_6, param_5, param_7, param_4);
}
u_var1 = pass1_1010_6ca2(param_2, 0x7, param_5,param_4); if (u_var1 != 0x0) {
pass1_1010_715c(param_2, 0x10, u_var1, param_5,param_7, param_4);
}
param_6 = pass1_1010_6ca2(param_2, 0x3, param_5,param_4); if (param_6 != 0x0) {
pass1_1010_715c(param_2, 0x11, param_6, param_5,param_7, param_4);
}
if (param_3 == 0x21) {
pass1_1010_715c(param_2, 0x14, param_6, param_5,param_7, param_4);
}
if (param_3 != 0xc) {
return 0x1;
}
u_var1 = 0x9;
}
//     TODO: goto code_r0x10106d4c;
0xe => {u_var1 = 0xc;}
//     TODO: goto code_r0x10106d4c;
0x10 => {} 
0x11 => {}
0x13 => {u_var1 = 0xd;}
0x12 => {u_var1 = 0xe;}
0x1b => {}
0x1f => {}
0x5b => {}
0x78 => {}
0x7e =>{}
0x7f => {if (param_3 == 0x7e) {
pass1_1010_715c(param_2, 0x2c, param_6, param_5,param_7, param_4);
}
if (param_3 == 0x5b) {
pass1_1010_715c(param_2, 0x38, param_6, param_5,param_7, param_4);
}
if (param_3 == 0x1f) {
pass1_1010_715c(param_2, 0x3f, param_6, param_5,param_7, param_4);
}
if (param_3 == 0x7f) {
pass1_1010_715c(param_2, 0x42, param_6, param_5,param_7, param_4);
}
param_6 = pass1_1010_6ca2(param_2, 0x5, param_5,param_4); if ((param_6 == 0x0) & & (param_6 = pass1_1010_6ca2(param_2, 0x5, param_5, param_4), param_6 == 0x0)) {
return 0x1;
}
u_var1 = 0x37;  }
0x1d => {}
0x2a =>{}
0x2c => {}
0x3c => {}
0x3d => {}
0x4b => {}
0x53 =>{}
0x54 => {}
0x55 =>{}
 0x5a =>{ u_var1 = pass1_1010_6ca2(param_2, 0x2, param_5, param_4); if (u_var1 != 0x0) {
pass1_1010_715c(param_2, 0x12, u_var1, param_5,param_7, param_4);
}
u_var1 = pass1_1010_6ca2(param_2, 0x8, param_5,param_4); if (u_var1 != 0x0) {
pass1_1010_715c(param_2, 0x1a, u_var1, param_5,param_7, param_4);
}
if (param_3 == 0x2c) {
pass1_1010_715c(param_2, 0x1d, u_var1, param_5,param_7, param_4);
}
param_6 = pass1_1010_6ca2(param_2, 0x2, param_5,param_4); if (param_6 == 0x0) {
return 0x1;
}
u_var1 = 0x1c; }
0x22 => {u_var1 = 0x15;} 
0x25 => {u_var1 = 0x16;}
0x26 => {pass1_1010_715c(param_2, 0x17,param_6, param_5, param_7, param_4);} 
0x1e => {u_var1 = 0x13;} 
0x27 => {u_var1 = 0x18;} 
0x29 => {u_var1 = 0x19;} 
0x2b => {u_var1 = 0x1b;} 
0x2f => {}
0x36 => {param_6 = pass1_1010_6ca2(param_2, 0x2, param_5, param_4); if (param_6 == 0x0) {
return 0x0;
}
u_var1 = 0x1e; } 
0x30 => {u_var1 = 0x1f;} 
0x31 => {u_var1 = 0x35;}
0x33 => {u_var1 = 0x21;} 
0x34 => {u_var1 = 0x22;} 
0x35 =>{
pass1_1010_715c(param_2, 0x23, param_6, param_5,param_7, param_4);} 
0x65 => {}
0x66 =>{} 
0x6b =>{} 
0x6c => {}
0x6d => {}
0x6e => {}
0x6f => {u_var1 = 0x34; } 
0x38 => {pass1_1010_715c(param_2, 0x24, param_6, param_5, param_7, param_4); u_var1 = 0x3d; } 
0x39 =>{
u_var1 = 0x25; } 
0x3e => {pass1_1010_715c(param_2, 0x26, param_6, param_5, param_7,param_4); pass1_1010_715c(param_2, 0x28, param_6, param_5, param_7, param_4); u_var1 = 0x27; }
 0x40 => {u_var1 = 0x2a;
} 
0x41 => {u_var1 = 0x39; }
0x42 => {u_var1 = 0x3a;}  
0x44 =>{
u_var1 = 0x36;} 
0x45 => {u_var1 = 0x3b;} 
0x49 => {u_var1 = 0x29;}
0x50 => {u_var1 = 0x2b;} 
0x56 => {pass1_1010_715c(param_2, 0x3c, param_6,param_5, param_7, param_4); u_var1 = 0x3e; } 
0x5d => {pass1_1010_715c(param_2, 0x2f, param_6, param_5, param_7, param_4);
u_var1 = 0x40;} 
0x5e => {} 
0x60 => {
u_var1 = 0x2f;} 
0x5f => {pass1_1010_715c(param_2, 0x34, param_6, param_5, param_7,param_4); u_var1 = 0x41; } 
0x61 => {u_var1 = 0x30;} 
0x63 => {u_var1 = 0x31;} 
0x64 => {u_var1 = 0x24;} 
0x68 => {u_var1 = 0x32;} 
0x69 => {u_var1 = 0x33;} 
0x76 => {u_var1 = 0xa; 
    // code_r0x10106d4c: 
    pass1_1010_715c(param_2, u_var1, param_6, param_5, param_7, param_4); u_var1 = 0xb;
}}
pass1_1010_715c(param_2, u_var1, param_6, param_5,param_7, param_4); return 0x1;
}



pub fn pass1_1010_715c(param_1: u32, param_2: u16, param_3: u16, param_4: u16, param_5: u16,
                       param_6: u16)

{
    pass1_1010_a69c((param_1 + 0x14), param_2, param_3, param_4, param_5, param_6);
    return;
}


pub fn pass1_1010_71b0(param_1: i16, param_2: u16) {
    let u_var1: u32;

    u_var1 = (param_1 + 0x6);
    send_msg_1010_7c42(u_var1 & 0xffff0000 | (u_var1 - 0xa), param_2);
    return;
}


pub fn pass1_1010_71c2(param_1: u16, param_2: u16, param_3: i16, param_4: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;

    if (param_1 == 0x13) {
        u_var2 = (param_3 + 0x6);
        u_var2 = (u_var2 + 0x18);
        u_var1 = (param_3 + 0x6);
        destroy_window_1010_7b26(u_var1 & 0xffff0000 | (u_var1 - 0xa), (u_var2 + 0x28), param_4, param_2, 0);
        return;
    }
    if (param_1 < 0x14) {
        if (param_1 == '\x01') {
            u_var2 = (param_3 + 0x6);
            // u_var4 = (u_var2 >> 0x10);
            i_var3 = u_var2;
            (i_var3 + 0xa) = 0x0;
            (i_var3 + 0x18) = 0x0;
            return;
        }
        if (param_1 == '\x05') {
            u_var1 = (param_3 + 0x6);
            send_msg_1010_7c42(u_var1 & 0xffff0000 | (u_var1 - 0xa), param_4);
            return;
        }
    }
    return;
}


pub fn pass1_1010_71d6(param_1: u32, param_2: i16, param_3: U32Ptr, param_4: u16, param_5: u16,
                       param_6: u16)

{
    let i_var1: i16;
    let u_var2: u32;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u32;
    let uStack20: u16;
    let uStack14: u16;
    let uStack6: u32;

    // uVar8 = (param_1 >> 0x10);
    u_var2 = (param_1 + 0x14);
    pass1_1010_ad22(u_var2, param_5, param_6, (u_var2 >> 0x10), *param_3);
    uStack6 = CONCAT22(param_5, param_4);
    if ((param_5 | param_4) == 0x0) {
        return;
    }
    uVar9 = struct_op_1030_73a8(CONCAT22(param_5, param_4));
    // uVar7 = (uVar9 >> 0x10);
    u_var3 = uVar9;
    if (((uVar7 | u_var3) != 0x0) && ((u_var3 + 0x1c) == 0x8000002)) {
        return;
    }
    u_var2 = (param_4 + 0x2e);
    uStack14 = u_var2;
    if ((((param_4 + 0x30) | uStack14) != 0x0) && ((uStack14 + 0x200) == 0x8000002)) {
        return;
    }
    u_var2 = (param_1 + 0x14);
    u_var5 = pass1_1010_b028(u_var2, uVar9);
    i_var1 = (u_var3 + 0x12);
    i_var4 = i_var1;
    if ((i_var1 != 0x4) && (i_var4 = param_2, i_var1 == 0x7)) {
        param_2 = 0x5;
        i_var4 = param_2;
    }
    param_2 = i_var4;
    u_var6 = param_2 - 0x2;
    if (u_var6 != 0x0) {
        if (param_2 == 0x3) {
            u_var6 = u_var5 - 0xb;
            if ((u_var6 == 0x0) || (u_var6 = u_var5 - 0x37, u_var6 == 0x0)) {
                uStack20 = 0xb;
            } else {
                uStack20 = 0xa;
            }
//       TODO: goto LAB_1010_72a7;
        }
        u_var6 = param_2 - 0x4;
        if (u_var6 == 0x0) {
            uStack20 = 0x17;
//       TODO: goto LAB_1010_72a7;
        }
        u_var6 = param_2 - 0x5;
        if (u_var6 != 0x0) {
            u_var6 = pass1_1010_7818(param_1, uVar9);
            uStack20 = u_var6;
//       TODO: goto LAB_1010_72a7;
        }
    }
    uStack20 = 0xc;
//LAB_1010_72a7:
    if (uStack20 == 0x0) {
        return;
    }
    ui_op_1010_79aa(param_1, 0x0, uStack6, param_6);
    if (u_var6 == 0x0) {
        unk_win_op_1010_7300(param_1, 0x0, uStack20, uStack6);
    }
    return;
}


/*
Unable to decompile 'unk_win_op_1010_7300'
Cause: Exception while decompiling 1010:7300: The pipe is being closed

*/


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_7818(param_1: u32, param_2: u32) -> u16

{
    let u_var1: u32;
    let u_var2: u16;
    let BVar3: bool;
    let u_var4: u16;
    let uStack6: u16;

    // u_var4 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x14);
    u_var2 = pass1_1010_b028(u_var1, param_2);
    BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x1e);
    if (BVar3 == 0x0) {
        BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0xb);
        if (((BVar3 == 0x0) && (BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x20), BVar3 == 0x0)) && (BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x1c), BVar3 == 0x0)) {
            BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x2);
            if ((BVar3 != 0x0) || (BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x13), BVar3 != 0x0)) {
                return 0x5;
            }
            BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x11);
            if ((BVar3 != 0x0) || (BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x12), BVar3 != 0x0)) {
                return 0x4;
            }
            BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x5);
            if (BVar3 != 0x0) {
                return 0x6;
            }
            BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x6);
            if (BVar3 != 0x0) {
                return 0x7;
            }
            BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x4);
            if (BVar3 != 0x0) {
                return 0x10;
            }
            BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x3);
            if (BVar3 != 0x0) {
                return 0x11;
            }
            BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x19);
            if (BVar3 != 0x0) {
                return 0x15;
            }
            BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, 0x1d);
            if (BVar3 != 0x0) {
                return 0x16;
            }
            u_var2 = pass1_1010_7d7e(param_1, u_var4, 0x1, u_var2);
            if (u_var2 == 0x0) {
                return 0x0;
            }
            return 0xc;
        }
        uStack6 = 0x1;
    } else {
        uStack6 = 0x18;
    }
    return uStack6;
}


pub fn pass1_1010_7b8c(param_1: &mut Struct20, param_2: i16, param_3: &mut WNDCLASS16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u32;
    let pu_var5: U32Ptr;
    let extraout_dx: u16;
    let iVar6: i16;
    let uVar7: u16;
    let uStack14: u32;
    let local_a: [u8; 8];

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if (((iVar6 + 0x1e) | (iVar6 + 0x1c)) != 0x0) {
        pass1_1008_5784(CONCAT22(param_3, local_a), (iVar6 + 0x1c));
        loop {
            pu_var5 = local_a;
            pass1_1008_5b12(pu_var5, param_3);
            uStack14 = CONCAT22(extraout_dx, pu_var5);
            if ((extraout_dx | pu_var5) == 0x0) { break; }
            u_var4 = (pu_var5 + 0x8);
            if ((u_var4 + 0x6) != param_2) == false { break; }
        }
        if ((extraout_dx | pu_var5) != 0x0) {
            ppc_var3 = ((iVar6 + 0x1c) + 0xc);
            (**ppc_var3)(0x1008, (iVar6 + 0x1c), uStack14);
        }
        u_var4 = (iVar6 + 0x1c);
        if ((u_var4 + 0x8) == 0x0) {
            pu_var1 = (iVar6 + 0x1c);
            u_var2 = (iVar6 + 0x1e);
            if ((u_var2 | pu_var1) != 0x0) {
                ppc_var3 = *pu_var1;
                (**ppc_var3)(0x1008, pu_var1, u_var2, 0x1, pu_var1, u_var2, pu_var1, u_var2);
            }
            (iVar6 + 0x1c) = 0x0;
        }
    }
    return;
}


pub fn pass1_1010_7d38(param_1: u16, param_2: u16, param_3: i16, param_4: u16, param_5: u16) -> u16

{
    let local_e: u32;
    let uStack10: u16;
    let local_8: u16;
    let local_6: [u8; 2];
    let local_4: [u8; 2];

    local_e = (param_3 + 0xc);
    uStack10 = (param_3 + 0x10);
    pass1_1008_3eb4(CONCAT22(param_5, &local_e),
                    CONCAT22(param_5, &local_8), CONCAT22(param_5, local_6), CONCAT22(param_5, local_4));
    return local_8;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_7d7e(param_1: u16, param_2: u16, param_3: i16, param_4: i16) -> u16

{
    let b_var1: bool;

    if (param_3 != 0x3) {
        if ((param_4 < 0xa) || (0x7f < param_4)) {
            return 0x0;
        }
        b_var1 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, param_4, 0x3c);
        if (b_var1 != 0x0) {
            return 0x0;
        }
        if (((param_4 == 0x6a) && (param_3 != 0x4)) && (param_3 != 0x5)) {
            return 0x0;
        }
    }
    return 0x1;
}


pub fn pass1_1010_7dc6(param_1: u32, param_2: u8) -> u32

{
    let unaff_SS: u16;

    param_1 = param_1 & 0xffff0000 | (param_1 - 0xa);
    pass1_1010_6bb2(param_1, unaff_SS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_7dd2(param_1: U32Ptr, param_2: u8) -> u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_7e40(param_1: U32Ptr, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let u_var1: u32;
    let pu_var2: &mut Struct652;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    // u_var2 = (param_1 >> 0x10);
    pu_var2 = param_1;
    *param_1 = 0x0;
    pu_var2.field_0x67c = 0x0;
    pu_var2.field_0x680 = 0x0;
    pu_var2.field_0xe82 = 0x0;
    pu_var2.field_0xe84 = 0x0;
    &pu_var2.field_0xe88 = 0x0;
    pass1_1000_4906(
        (param_1 & 0xffff0000 | &pu_var2.field_0x4),
        0x0, 0x228);
    pass1_1000_4906(
        (param_1 & 0xffff0000 | &pu_var2.field_0x22c),
        0x0, 0x228);
    pass1_1000_4906(
        (param_1 & 0xffff0000 | &pu_var2.field_0x454),
        0x0, 0x228);
    *&pu_var2.field_0x682 = 0x0;
    *&pu_var2.field_0xa82 = 0x0;
    ctx._PTR_LOOP_1050_14cc = param_1;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    pu_var2.field_0xe88 = pu_var3;
    pu_var2.field_0xe8a = (pu_var3 >> 0x10);
    u_var1 = &pu_var2.field_0xe88;
    pu_var2.field_0xe84 = (u_var1 + 0x64);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_7efc(param_1: U32Ptr, param_2: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let pu_var3: u32;
    let ppc_var4: u32;
    let iVar5: &mut Struct448;
    let u_var5: u16;
    let paStack8: &mut Struct18;
    let i_stack4: i16;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    u_var1 = iVar5.field_0x67c;
    u_var2 = iVar5.field_0x67e;
    paStack8 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        pass1_1008_64a2(CONCAT22(u_var2, u_var1));
        param_2 = 0x1000;
        fn_ptr_1000_17ce(ctx, paStack8, 0x1000);
    }
    // TODO: refactor for loop
    // for (i_stack4 = 0x0; i_stack4 < 0x8a; i_stack4 += 0x1) {
    //   pu_var3 = (&iVar5.field_0x4 + i_stack4 * 0x4);
    //   u_var1 = (&iVar5.field_0x4 + i_stack4 * 0x4 + 0x2);
    //   if ((u_var1 | pu_var3) != 0x0) {
    //     ppc_var4 = *pu_var3;
    //     (**ppc_var4)(param_2,pu_var3,u_var1,0x1);
    //   }
    //   pu_var3 = (&iVar5.field_0x22c + i_stack4 * 0x4);
    //   u_var1 = (&iVar5.field_0x22c + i_stack4 * 0x4 + 0x2);
    //   if ((u_var1 | pu_var3) != 0x0) {
    //     ppc_var4 = *pu_var3;
    //     (**ppc_var4)(param_2,pu_var3);
    //   }
    //   pu_var3 = (&iVar5.field_0x454 + i_stack4 * 0x4);
    //   u_var1 = (&iVar5.field_0x454 + i_stack4 * 0x4 + 0x2);
    //   if ((u_var1 | pu_var3) != 0x0) {
    //     ppc_var4 = *pu_var3;
    //     (**ppc_var4)(param_2,pu_var3);
    //   }
    // }
    fn_ptr_1000_17ce(ctx, *param_1, 0x1000);
    ctx._PTR_LOOP_1050_14cc = 0x0;
    return;
}


pub fn pass1_1010_7fd6(param_1: u32) {
    let u_var1: u16;
    let u_var2: u16;
    let i_var3: &mut Struct489;
    let u_var3: u16;
    let paStack6: &mut Struct18;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var1 = i_var3.field_0x67c;
    u_var2 = i_var3.field_0x67e;
    paStack6 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        pass1_1008_64a2(CONCAT22(u_var2, u_var1));
        fn_ptr_1000_17ce(ctx, paStack6, 0x1000);
    }
    &i_var3.field_0x67c = 0x0;
    i_var3.field_0x680 = 0x0;
    return;
}


pub fn pass1_1010_8018(param_1: u32, param_2: u16, param_3: U32Ptr, param_4: u16) {
    let i_var1: i16;
    let u_var2: U32Ptr;

    if ((param_2 * 0xa + 0x1fa0) != 0x0) {
        pass1_1010_878c(ctx, param_1, (param_2 * 0xa + 0x1fa0), param_4);
        // u_var2 = (param_1 >> 0x10);
        if ((param_1 + 0x67c) != 0x0) {
            i_var1 = param_2 * 0xa;
            pass1_1008_64c8((param_1 + 0x67c),
                            CONCAT22((i_var1 + 0x1fa6),
                                     (i_var1 + 0x1fa8)), (i_var1 + 0x1fa4),
                            param_2, param_3);
            return;
        }
    }
    return;
}


pub fn pass1_1010_8096(param_1: U32Ptr, param_2: i16) {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let unaff_SS: u16;
    let mut pcVar5: String;
    let puVar6: U32Ptr;
    let local_306: [u8; 100];
    let local_206: [u8; 100];
    let local_106: [u8; 104];

    // u_var4 = (param_1 >> 0x10);
    u_var3 = param_1;
    str_1000_4d58(((u_var3 + 0xe82) * 0x4 + 0x2526), 0x0, 0x0,
                  CONCAT22(unaff_SS, local_206), CONCAT22(unaff_SS, local_306));
    string_1000_3d3e(CONCAT22(unaff_SS, local_106), CONCAT22(unaff_SS, local_206));
    if (param_2 == 0x2) {
        puVar6 = &USHORT_1050_3194;
    } else {
        puVar6 = &USHORT_1050_3196;
    }
    string_1000_3cea(CONCAT22(unaff_SS, local_106), puVar6);
    string_1000_3cea(CONCAT22(unaff_SS, local_106), CONCAT22(unaff_SS, local_306));
    pcVar5 = set_err_mode_1010_8b14(param_1, CONCAT22(unaff_SS, local_106), unaff_SS);
    // u_var2 = (pcVar5 >> 0x10);
    if ((pcVar5 == local_106) && (u_var2 == unaff_SS)) {
        msg_box_op_1010_8bb4(u_var3, u_var4, pcVar5 & 0xffff | u_var2 << 0x10, 0x1000, unaff_SS);
    }
    fn_ptr_1000_17ce(ctx, *param_1, 0x1000);
    u_var1 = str_op_1008_60e8(pcVar5, u_var2);
    param_1 = u_var1;
    (u_var3 + 0x2) = u_var2;
    return;
}


pub fn pass1_1010_8170(param_1: i16, param_2: i16, param_3: U32Ptr, param_4: u16) {
    let u_var1: u16;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var2 = param_1;
    u_var1 = (i_var2 + 0x680);
    i_var3 = param_2 * 0x10;
    if ((i_var3 + 0x16) != u_var1) {
        pass1_1010_8096(param_1, (i_var3 + 0x16));
        pass1_1010_878c(param_1, (i_var3 + 0x16), param_4);
        if ((i_var2 + 0x67c) == 0x0) {
            return;
        }
    }
    i_var3 = param_2 * 0x10;
    pass1_1008_6562((i_var2 + 0x67c),
                    CONCAT22((i_var3 + 0x1c), (i_var3 + 0x1e)),
                    (i_var3 + 0x1a), u_var1, param_3);
    return;
}


pub fn pass1_1010_81f6(
    param_1: HINSTANCE16, 
    param_2: u16, 
    param_3: &mut Struct87, 
    param_4: i32,
    param_5: i16)

{
    let u_var1: u16;
    let u_var2: u16;
    let uStack12: u16;
    let uStack10: u32;

    if (param_4 == 0x8000001) {
        uStack10 = param_3 & 0xffff0000 | (param_3 + 0x22c);
        uStack12 = 0xfa;
    } else {
        if (param_4 == 0x8000002) {
            uStack10 = param_3 & 0xffff0000 | (param_3 + 0x454);
            uStack12 = 0xfc;
        } else {
            uStack10 = param_3 & 0xffff0000 | (param_3 + 0x4);
            uStack12 = 0x2;
        }
    }
    // u_var2 = (uStack10 >> 0x10);
    u_var1 = param_3._2_2_;
    if ((param_5 * 0x4 + uStack10) == 0x0) {
        if ((0x0 < param_5) && (param_5 < 0xa)) {
            pass1_1010_89f0(param_3, param_3._2_2_, uStack12, uStack10, param_1, param_2);
            return;
        }
        if (param_5 < 0xa) {
            return;
        }
        if (0x7f < param_5) {
            return;
        }
        if ((uStack10 + 0x14) == 0x0) {
            pass1_1010_89f0(param_3, param_3._2_2_, uStack12, uStack10, param_1, param_2);
        }
        pass1_1010_887a(param_3, uStack10, param_5, param_1, param_2);
    }
    pass1_1010_866c(u_var1, param_3, param_3._2_2_, uStack10, param_5);
    return;
}


pub fn pass1_1010_82f8(param_1: u32, param_2: u16) {
    (param_1 + 0xe82) = param_2;
    return;
}


pub fn pass1_1010_84f8(param_1: u32, param_2: i16, param_3: u16) {
    let u_var1: u32;
    let uStack780: u16;
    let local_308: [u8;0x100];
    let local_208: [u8; 100];
    let local_108: [u8; 104];
    let i_stack4: i16;

    if ((param_2 * 0x10 + 0x10) != 0x3) {
        return;
    }
    u_var1 = (param_1 + 0xe88);
    i_stack4 = (u_var1 + 0x70);
    str_1000_4d58((param_2 * 0x10 + 0x12), 0x0, 0x0,
                  CONCAT22(param_3, local_208), CONCAT22(param_3, local_308));
    string_1000_3d3e(CONCAT22(param_3, local_108), CONCAT22(param_3, local_208));
    if (local_308[0] == '\0') {
        if (i_stack4 == 0x0) {
            uStack780 = 0x14c0;
        } else {
            uStack780 = 0x14ba;
        }
        _uStack780 = CONCAT22(0x1050, uStack780);
    } else {
        _uStack780 = CONCAT22(param_3, local_308);
    }
    string_1000_3cea(CONCAT22(param_3, local_108), _uStack780);
    set_err_mode_1010_8b14(param_1, CONCAT22(param_3, local_108), param_3);
    return;
}


pub fn pass1_1010_85be(param_1: u32, param_2: i16, param_3: i16, param_4: u16) {
    let u_var1: u32;
    let local_30a: [u8; 100];
    let local_20a: [u8; 100];
    let local_10a: [u8; 108];

    if (param_2 == 0x2) {
        u_var1 = (param_3 * 0x4 + 0x2e34);
        str_1000_4d58((u_var1 & 0xffff0000 | (u_var1 + 0x3)), 0x0, 0x0, CONCAT22(param_4, local_20a), CONCAT22(param_4, local_30a));
        string_1000_3d3e(CONCAT22(param_4, local_10a), s_male_1050_14c6);
        string_1000_3cea(CONCAT22(param_4, local_10a), CONCAT22(param_4, local_20a));
        string_1000_3cea(CONCAT22(param_4, local_10a), CONCAT22(param_4, local_30a));
        set_err_mode_1010_8b14(param_1, CONCAT22(param_4, local_10a), param_4);
        return;
    }
    set_err_mode_1010_8b14(param_1, (param_3 * 0x4 + 0x2e34), param_4);
    return;
}


pub fn pass1_1010_866c(param_1: u16, param_2: u16, param_3: u16, param_4: u32, param_5: u16) {
    let u_var1: u32;
    let cVar2: u8;
    let i_var3: i16;
    let bVar4: bool;

    if (param_5 < 0x28) {
        if ((param_5 < 0x25) && (param_5 != 0x23)) {
            if (0x23 < param_5) {
                return;
            }
            cVar2 = param_5;
            if (((cVar2 != '\x0b') && (cVar2 != '\x0f')) && (cVar2 != '!')) {
                return;
            }
        }
    } else {
        if (param_5 != 0x37) {
            if (param_5 < 0x38) {
                if (param_5 < 0x33) {
                    return;
                }
                bVar4 = SBORROW2(param_5 - 0x33, 0x1);
                i_var3 = param_5 - 0x34;
            } else {
                if (param_5 == 0x49) {
                    // goto
                    // LAB_1010_8691;
                }
                if ((param_5 - 0x49) < 0x2a) {
                    return;
                }
                bVar4 = SBORROW2(param_5 - 0x73, 0x5);
                i_var3 = param_5 - 0x78;
            }
            if (((i_var3 != 0x0) && bVar4) == (i_var3 < 0x0)) {
                return;
            }
        }
    }
//LAB_1010_8691:
    u_var1 = (param_5 * 0x4 + param_4);
    memcpy_op_1008_676e(u_var1, u_var1, param_1);
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1010_86de(param_1: u16, param_2: u16, param_3: u8, param_4: u32) {
    let mut plVar1: long;
    let i_var2: i16;
    let bVar3: bool;
    let u_var4: u16;
    let lVar5: i32;
    let u_var6: u32;
    let lStack20: i32;
    let uStack10: u32;

    u_var6 = pass1_1008_4772(param_4);
    // u_var4 = (u_var6 >> 0x10);
    uStack10 = 0x0;
    loop {
        plVar1 = (u_var6 + 0x8);
        if (*plVar1 == uStack10 || *plVar1 < uStack10) {
            return;
        }
        lVar5 = uStack10;
        pass1_1008_4544(param_4);
        i_var2 = lVar5;
        bVar3 = false;
        // TODO: refactor for loop
//     for (lStack20 = 0x0; plVar1 = (u_var6 + 0x4),
//         *plVar1 != lStack20 && lStack20 <= *plVar1; lStack20 += 0x1) {
//       if (bVar3) {
// //LAB_1010_86fc:
//         if (bVar3) {
//           if (*(lStack20 + i_var2) == -0x1) {
//             *(lStack20 + i_var2) = param_3;
//             break;
//           }
//         }
//       }
//       else {
//         if (*(lStack20 + i_var2) == -0x1) goto LAB_1010_86fc;
//         *(lStack20 + i_var2 + -0x1) = param_3;
//         bVar3 = true;
//       }
//     }
        uStack10 += 0x1;
    }
}


pub fn pass1_1010_878c(
    ctx: &mut AppContext,
    struct_1: &mut Struct87,
    param_2: i16,
    param_3: &mut HINSTANCE16,
    unaff_ss: u16,
) {
    let u_var1: u16;
    let u_var2: u16;
    let u_var4: &mut Struct18;
    let pu_var3: &mut Struct18;
    let pu_var4: &mut Struct18;
    let u_var6: &mut Struct87;
    let i_var5: i16;
    let u_var7: u16 = 0;
    let pa_var8: &mut Struct18;
    let pa_var9: &mut Struct18;

    u_var6 = struct_1;
    if u_var6.field_0x680 == param_2 {
        return;
    }
    u_var1 = u_var6.field_0x67c;
    pu_var4 = &mut u_var6.field_0x67e;
    pu_var3 = pu_var4;
    u_var2 = u_var1;
    if pu_var3 != 0x0 {
        pass1_1008_64a2(pu_var4);
        *param_3 = 0x1000;
        fn_ptr_1000_17ce(ctx, pu_var4, 0x1000);
    }
    if (param_2 == 0x1) || (param_2 == 0x2) {
        mem_op_1000_179c(ctx, 0x8, pu_var3, 0x1000);
        pu_var4 = pu_var3;
        if pu_var4 == 0x0 {
            u_var6.field_0x67c = 0x0;
//       TODO: goto LAB_1010_8869;
        }
        pa_var8 = struct_1.field_0x0;
        pa_var9 = pu_var3;
//LAB_1010_8853:
        u_var4 = pa_var9;
        file_1008_6414(pa_var9, pa_var8, unaff_ss, pu_var4);
    } else {
        i_var5 = param_2 * 0x4;
        pa_var8 = set_err_mode_1010_8b14(struct_1, (i_var5 + 0x172a), unaff_ss);
        pa_var9 = pa_var8;
        if ((i_var5 + 0x172a) == pa_var8) && ((i_var5 + 0x172c) == (pa_var8 >> 0x10)) {
            msg_box_op_1010_8bb4(u_var6, u_var7, pa_var8, param_3, unaff_ss);
        }
        mem_op_1000_179c(ctx, 0x8, (pa_var9 >> 0x10), 0x1000);
        pu_var4 = ((pa_var9 >> 0x10) | pa_var9);
        if pa_var9 != 0x0 {
            // goto
            // LAB_1010_8853;
        }
        u_var4 = None;
        pu_var4 = None;
    }
    u_var6.field_0x67c = u_var4;
    u_var6.field_0x67e = pu_var4;
//LAB_1010_8869:
    u_var6.field_0x680 = param_2;
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1010_887a(ctx: &mut AppContext, param_1: &mut Struct87, param_2: u32, param_3: i16, param_4: u16,
                       param_5: u16)

{
    let u_var1: u16;
    let u_var2: u32;
    let u_var3: u32;
    let in_DX: U32Ptr;
    let puVar4: U32Ptr;
    let extraout_dx: u16;
    let iVar5: i16;
    let u_var6: u16;
    let uVar7: u16;
    let bVar8: u8;
    let local_26: [u8; 6];
    let uStack32: u16;
    let uStack30: u16;
    let uStack28: u32;
    let uStack24: u32;
    let uStack20: u32;
    let uStack16: u32;
    let paStack12: &mut Struct76;
    let paStack8: &mut Struct76;
    let uStack4: u16;

    uStack4 = param_3 - 0xa;
    pass1_1010_878c(param_1, (uStack4 * 0xa + 0x3382), param_4);
    // u_var6 = (param_1 >> 0x10);
    if ((param_1 + 0x67c) != 0x0) {
        iVar5 = uStack4 * 0xa;
        u_var1 = uStack4;
        pass1_1008_6562((param_1 + 0x67c),
                        CONCAT22((iVar5 + 0x3388),
                                 (iVar5 + 0x338a)), (iVar5 + 0x3386),
                        uStack4, in_DX);
        paStack8 = CONCAT22(in_DX, u_var1);
        // u_var6 = (param_2 >> 0x10);
        paStack12 = (param_2 + 0x14);
        uStack16 = pass1_1008_4772(paStack12);
        uStack20 = pass1_1008_4772(paStack8);
        // puVar4 = (uStack20 >> 0x10);
        u_var2 = (uStack20 + 0x4);
        // uVar7 = (uStack16 >> 0x10);
        iVar5 = uStack16;
        if (u_var2 < (iVar5 + 0x4)) {
            u_var2 = (iVar5 + 0x4);
        }
        u_var3 = (uStack20 + 0x8);
        if (u_var3 < (iVar5 + 0x8)) {
            u_var3 = (iVar5 + 0x8);
        }
        u_var1 = u_var3;
        uStack24 = u_var3 & 0xffff | u_var2 << 0x10;
        bVar8 = 0x1e;
        mem_op_1000_179c(0x1e, puVar4, 0x1000);
        if ((puVar4 | u_var1) == 0x0) {
            u_var1 = 0x0;
            uVar7 = 0x0;
        } else {
            struct_op_1008_6604(CONCAT22(puVar4, u_var1), uStack24,
                                (uStack24 >> 0x10));
            uVar7 = extraout_dx;
        }
        uStack28 = CONCAT22(uVar7, u_var1);
        pass1_1008_431c(CONCAT22(uVar7, u_var1), bVar8);
        // uVar7 = (uStack16 >> 0x10);
        uStack30 = (uStack24._2_2_ - (uStack16 + 0x4)) / 0x2;
        uStack32 = uStack24 - (uStack16 + 0x8);
        pass1_1008_3e54(CONCAT22(param_5, local_26), 0x0, uStack32, uStack30);
        pass1_1008_4480(uStack28, CONCAT22(param_5, local_26), paStack12, param_5);
        pass1_1008_3e76(CONCAT22(param_5, local_26), 0x0, 0x0, 0x7);
        pass1_1008_4480(uStack28, CONCAT22(param_5, local_26), paStack8, param_5);
        (param_3 * 0x4 + param_2) = uStack28;
    }
    return;
}


pub fn pass1_1010_89f0(param_1: u16, param_2: u16, param_3: u16, param_4: u32,
                       param_5: HINSTANCE16, param_6: u16)

{
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let pu_var5: U32Ptr;
    let iVar6: i16;
    let uVar7: u32;
    let uStack22: u32;
    let uStack8: u16;

    u_var3 = (param_1 + 0x67c);
    u_var1 = (param_1 + 0x67e);
    if ((u_var1 | u_var3) != 0x0) {
        pass1_1008_64a2(CONCAT22(u_var1, u_var3));
        param_5 = 0x1000;
        fn_ptr_1000_17ce(ctx, CONCAT22(u_var1, u_var3), 0x1000);
    }
    uVar7 = set_err_mode_1010_8b14(CONCAT22(param_2, param_1),
                                   ((param_1 + 0xe82) * 0x4 + 0x24be), param_6);
    // puVar4 = (uVar7 >> 0x10);
    u_var3 = uVar7;
    iVar6 = (param_1 + 0xe82) * 0x4;
    if (((iVar6 + 0x24be) == u_var3) && ((iVar6 + 0x24c0) == puVar4)) {
        msg_box_op_1010_8bb4(param_1, param_2, uVar7, param_5, param_6);
    }
    mem_op_1000_179c(0x8, puVar4, 0x1000);
    pu_var5 = (puVar4 | u_var3);
    if (pu_var5 == 0x0) {
        u_var3 = 0x0;
        pu_var5 = 0x0;
    } else {
        file_1008_6414(CONCAT13((puVar4 >> 0x8),
                                CONCAT12(puVar4, u_var3)), uVar7, param_6, pu_var5);
    }
    (param_1 + 0x67c) = u_var3;
    (param_1 + 0x67e) = pu_var5;
    (param_1 + 0x680) = 0x0;
    if (((param_1 + 0x67e) | (param_1 + 0x67c)) != 0x0) {
// TODO: refactor for loop
        // for (uStack8 = 0x1; uStack8 < 0xa; uStack8 += 0x1) {
        //   iVar6 = uStack8 * 0xa;
        //   u_var2 = (iVar6 + 0x2558);
        //   u_var3 = uStack8;
        //   pass1_1008_64c8(*(u32 **)(param_1 + 0x67c),
        //                   CONCAT13((u_var2 >> 0x8),
        //                            CONCAT12(u_var2,(iVar6 + 0x255a))),
        //                   (iVar6 + 0x2556),uStack8,pu_var5);
        //   uStack22 = CONCAT22(pu_var5,u_var3);
        //   pass1_1010_86de(param_1,param_2,param_3,CONCAT22(pu_var5,u_var3));
        //   (uStack8 * 0x4 + param_4) = uStack22;
        // }
    }
    return;
}


pub fn pass1_1010_8c32(param_1: &mut Struct640, param_2: u16, param_3: u16, param_4: u16) -> u32

{
    let unaff_DI: i16;
    let paVar1: &mut Struct79;
    let pu_var2: U32Ptr;

    paVar1 = struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    &param_1.field_0xa = 0x0;
    CONCAT22(param_2, param_1) = 0x8ee2;
    param_1.field_0x2 = 0x1010;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_4,
                              (paVar1 >> 0x10), unaff_DI);
    param_1.field_0xa = pu_var2;
    param_1.field_0xc = (pu_var2 >> 0x10);
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1010_8c78(param_1: U32Ptr, param_2: u16) {
    *param_1 = 0x8ee2;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1010_8ebc(param_1: u32, param_2: u8) -> u32

{
    let unaff_SS: u16;

    pass1_1010_8c78(param_1, unaff_SS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_8ef2(param_1: U32Ptr, param_2: U32Ptr, param_3: u16) {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let extraout_dx: U32Ptr;
    let i_var3: &mut Struct170;
    let unaff_DI: i16;
    let u_var3: u16;
    let puVar4: U32Ptr;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = 0x389a;
    i_var3.field_0x2 = 0x1008;
    u_var1 = 0x0;
    &i_var3.field_0x4 = 0x0;
    &i_var3.field_0x8 = 0x0;
    *param_1 = 0x9254;
    i_var3.field_0x2 = 0x1010;
    mem_op_1000_179c(0x18, param_2, 0x1000);
    pu_var2 = (param_2 | u_var1);
    if (pu_var2 == 0x0) {
        &i_var3.field_0x4 = 0x0;
    } else {
        struct_op_1030_1cd8(CONCAT22(param_2, u_var1), 0x5, 0x5);
        i_var3.field_0x4 = u_var1;
        i_var3.field_0x6 = extraout_dx;
        pu_var2 = extraout_dx;
    }
    puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_3, pu_var2, unaff_DI);
    i_var3.field_0x8 = puVar4;
    i_var3.field_0xa = (puVar4 >> 0x10);
    return;
}


pub fn pass1_1010_8f78(param_1: U32Ptr) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct490;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x9254;
    i_var4.field_0x2 = 0x1010;
    pu_var1 = i_var4.field_0x4;
    u_var2 = i_var4.field_0x6;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    *param_1 = 0x389a;
    i_var4.field_0x2 = 0x1008;
    return;
}


pub fn pass1_1010_8fba(param_1: u32, param_2: u16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let i_var3: &mut Struct411;
    let u_var3: u16;
    let uStack14: u32;
    let uStack10: u32;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    ppcVar1 = (*i_var3.field_0x4 + 0x10);
    (**ppcVar1)();
    uStack10 = CONCAT22(extraout_dx, param_2);
    uStack14 = 0x0;
    loop {
        if (uStack10 <= uStack14) {
            return;
        }
        ppcVar1 = (*i_var3.field_0x4 + 0x4);
        u_var2 = uStack10;
        (**ppcVar1)();
        if ((extraout_DX_00 | u_var2) != 0x0) { break; }
        uStack14 += 0x1;
    }
    ppcVar1 = (*i_var3.field_0x4 + 0x8);
    (**ppcVar1)();
    return;
}


pub fn pass1_1010_9044(param_1: u32) {
    let ppcVar1: u32;

    ppcVar1 = ((param_1 + 0x4) + 0x10);
    (**ppcVar1)();
    return;
}


pub fn pass1_1010_9092(param_1: u32, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let extraout_dx: U32Ptr;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let extraout_DX_00: u16;
    let iVar5: i16;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u32;
    let uStack14: u32;
    let uStack6: u32;

    // u_var6 = (param_1 >> 0x10);
    iVar5 = param_1;
    uVar8 = (iVar5 + 0x4);
    ppcVar1 = ((iVar5 + 0x4) + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_dx, param_2);
    uVar7 = 0xc;
    pu_var3 = extraout_dx;
    mem_op_1000_179c(0xc, extraout_dx, 0x1000);
    puVar4 = (pu_var3 | param_2);
    if (puVar4 == 0x0) {
        param_2 = 0x0;
        puVar4 = 0x0;
    } else {
        pass1_1010_8ef2(CONCAT22(pu_var3, param_2), puVar4, param_3);
    }
    // TODO: refactor for loop
    // for (uStack14 = 0x0; uStack14 < uStack6; uStack14 += 0x1) {
    //   ppcVar1 = ((iVar5 + 0x4) + 0x4);
    //   u_var2 = uStack6;
    //   (**ppcVar1)(0x1000,(iVar5 + 0x4),uStack14,uVar7,uVar8);
    //   if ((extraout_DX_00 | u_var2) != 0x0) {
    //     ppcVar1 = ((param_2 + 0x4) + 0xc);
    //     (**ppcVar1)(0x1000,(param_2 + 0x4),u_var2,extraout_DX_00);
    //   }
    // }
    return;
}


pub fn pass1_1010_9130(param_1: u32, param_2: U32Ptr, param_3: u16, param_4: u16, param_5: u16,
                       param_6: u8)

{
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    pass1_1030_1d58((param_1 + 0x4));
    if ((param_4 | param_3) != 0x0) {
        u_var1 = (param_1 + 0x8);
        pass1_1010_c3c2(u_var1, (u_var1 >> 0x10), param_2,
                        CONCAT22(param_4, param_3), (param_4 | param_3), param_6, param_5,
        );
        return;
    }
    *param_2 = '\0';
    return;
}


pub fn pass1_1010_91cc(param_1: u32) {
    let ppcVar1: u32;
    let u_var2: u16;
    let lVar3: i32;

    // u_var2 = (param_1 >> 0x10);
    ppcVar1 = ((param_1 + 0x4) + 0x10);
    lVar3 = (**ppcVar1)();
    if (lVar3 != 0x0) {
        ppcVar1 = ((param_1 + 0x4) + 0x8);
        (**ppcVar1)();
    }
    return;
}


pub fn pass1_1010_9210(param_1: u32) {
    let ppcVar1: u32;

    ppcVar1 = ((param_1 + 0x4) + 0xc);
    (**ppcVar1)();
    return;
}


pub fn pass1_1010_922e(param_1: U32Ptr, param_2: u8) -> u16

{
    pass1_1010_8f78(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_9258(param_1: U32Ptr) -> u16

{
    struct_1010_383a(param_1);
    *param_1 = 0x958e;
    (param_1 + 0x2) = 0x1010;
    return param_1;
}


pub fn pass1_1010_927a(param_1: U32Ptr) {
    *param_1 = 0x958e;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_3880(param_1);
    return;
}



pub fn pass1_1010_9298(
    param_1: & mut Struct79,
    param_2: & mut Struct79,
    param_3: u16,
    param_4: u16,
param_5: * mut u8,param_6: u16) -> u32

{
struct_1010_2cd2(param_1, param_2, param_3, param_6); CONCAT22(param_2, param_1) = 0x9566;
param_1.field_0x2 = 0x1010; mem_op_1000_179c(0x20c,param_5, 0x1000); param_1[0x9].field_0x2 = param_4; & param_1[0x9].field_0x4 = param_5; pass1_1000_4906(CONCAT22(param_5,param_1[0x9].field_0x2), 0x0,0x20c); return CONCAT22(param_2, param_1);
}



pub fn pass1_1010_92e6(param_1: U32Ptr, param_2: u16) {
    *param_1 = 0x9566;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_2db2(param_1, param_2);
    return;
}


pub fn pass1_1010_9304(param_1: u16, param_2: u16, param_3: i16, param_4: u16, param_5: U32Ptr) {
    if (param_3 != 0x0) {
        mem_op_1000_179c(param_3 << 0x2, param_5, 0x1000);
        return;
    }
    mem_op_1000_179c(0x1a, param_5, 0x1000);
    if ((param_5 | param_4) != 0x0) {
        pass1_1010_9258(CONCAT22(param_5, param_4));
        return;
    }
    return;
}


pub fn pass1_1010_9348(param_1: u32, param_2: i16) {
    let i_var1: i16;
    let u_var2: u16;

    (param_2 * 0x8 + 0x319e) = param_2;
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x16) = param_2 * 0x8 + 0x3198;
    (i_var1 + 0x18) = ctx.data_seg;
    (i_var1 + 0x12) = param_2;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_9372(param_1: U32Ptr, param_2: u16, param_3: i16, param_4: i16, param_5: i16) {
    let ppcVar1: u32;
    let cVar2: u8;
    let u_var3: u16;
    let u_var4: u16;
    let unaff_DI: i16;
    let unaff_SS: u16;
    let bVar5: bool;
    let u_var6: u32;
    let uVar7: u32;

    if (0x0 < param_4) {
        if (ctx.PTR_LOOP_1050_3528 == 0x0) {
            ppcVar1 = (*param_1 + 0x18);
            u_var6 = (**ppcVar1)();
            ctx._PTR_LOOP_1050_3528 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, u_var6, unaff_SS,
                                                      (u_var6 >> 0x10), unaff_DI);
        }
        u_var6 = (param_1 + 0xc);
        uVar7 = pass1_1010_2e02(ctx.PTR_LOOP_1050_3528, (u_var6 + 0x12));
        u_var3 = param_2 + 0x1;
        u_var4 = param_3 + (0xfffe < param_2);
        // TODO: refactor for loop
        // for (cVar2 = (param_4 + -0x1) * '\x04'; cVar2 != '\0'; cVar2 += -0x1) {
        //   bVar5 = CARRY2(u_var3,u_var3);
        //   u_var3 *= 0x2;
        //   u_var4 = u_var4 * 0x2 + bVar5;
        // }
        pass1_1010_2e30(ctx.PTR_LOOP_1050_3528, u_var3 | uVar7,
                        u_var4 | (uVar7 >> 0x10), param_5);
    }
    return;
}


pub fn pass1_1010_93f0(param_1: u32, param_2: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let i_var3: i16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let local_1c:[u8;0x1a];

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x56) == 0x0) {
        pu_var5 = pass1_1010_9258(CONCAT22(param_2, local_1c));
        // u_var2 = (pu_var5 >> 0x10);
        pu_var1 = local_1c;
        pass1_1010_398e(CONCAT22(param_2, pu_var1), 0x0, 0x0, 0x0, pu_var1);
        (i_var3 + 0x56) = pu_var1;
        (i_var3 + 0x58) = u_var2;
        pass1_1010_927a(CONCAT22(param_2, local_1c));
    }
    return;
}


pub fn pass1_1010_944e(param_1: u16, param_2: u16, param_3: i16) {
    let ppcVar1: u32;
    let u_var2: u32;

    if ((param_1 + 0x56) == 0x0) {
        ppcVar1 = (CONCAT22(param_2, param_1) + 0x10);
        (**ppcVar1)();
    }
    u_var2 = pass1_1010_2e02(CONCAT22(param_2, param_1), param_3);
    pass1_1010_2e5c(param_1, param_2, u_var2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn
pass1_1010_9488(param_1: u16,param_2: u16,param_3: u32,param_4: * mut u8,param_5: i16,
param_6: u16) -> bool

{
let u_var1: u16; let u_var2: u16;
let u_var3: u16; let puVar4: * mut u16; let u_var5: u16; let uStack10: u16;

u_var5 = (param_3 + 0x12);
puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3,param_6, param_4, param_5);
// u_var2 = (puVar4 >> 0x10);
u_var1 = u_var5 - 0x32; uStack10 = puVar4; u_var3 = u_var2; if (u_var1 == 0x0) {
pass1_1010_a5ca(uStack10, u_var2, 0x32, 0x0,u_var2); if (u_var1 != 0x0) {
return false;
}
u_var5 = 0x4d;
}
else {
u_var1 = u_var5 - 0x3f; if (u_var1 == 0x0) {
pass1_1010_a5ca(uStack10, u_var2, 0x3f, 0x0,u_var2); if (u_var1 != 0x0) {
return false;
}
u_var5 = 0x4e;
}
}
pass1_1010_a5ca(uStack10, u_var2, u_var5, u_var1,u_var3); return u_var1 == 0x0;
}



pub fn pass1_1010_9502(param_1: u32) -> u16

{
    let u_var1: u32;

    u_var1 = (param_1 + 0x16);
    return (u_var1 + 0x2);
}


pub fn pass1_1010_9514() -> u16

{
    return 0x31;
}


pub fn pass1_1010_951a(param_1: U32Ptr, param_2: u8) -> u16

{
    pass1_1010_927a(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_9540(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_92e6(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_95f8(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct491;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0xa1c8;
    i_var4.field_0x2 = 0x1010;
    pu_var1 = i_var4.field_0xa;
    u_var2 = i_var4.field_0xc;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pu_var1 = i_var4.field_0xe;
    u_var2 = i_var4.field_0x10;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pu_var1 = i_var4.field_0x12;
    u_var2 = i_var4.field_0x14;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1010_9674(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 0x12);
    u_var2 = (i_var4 + 0x14);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    (i_var4 + 0x12) = 0x0;
    return;
}


pub fn pass1_1010_96a8(param_1: u32, param_2: i16) {
    let pi_var1: U32Ptr;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    pi_var1 = (param_1 + 0x1e);
    *pi_var1 = *pi_var1 - param_2;
    if (*pi_var1 < 0x0) {
        (param_1 + 0x1e) = 0x0;
    }
    return;
}


pub fn pass1_1010_96c2(param_1: u32) -> u16

{
    return (param_1 + 0x1e);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn  pass1_1010_96d0(param_1: u32) -> i16

{
let pi_var1: U32Ptr; 
let i_var2: i16; 
let i_var3: & mut Struct690; 
let u_var3: u16; 
let u_var4: u32; 
let iStack8: i16;

// u_var3 = (param_1 >> 0x10);
i_var3 = param_1; 
if (i_var3.field_0x1a != 0x0) {
    if (0x0 < i_var3.field_0x1c) {
        pi_var1 = & i_var3.field_0x1c; pi_var1 = pi_var1 + - 0x1;
    }
    if ((i_var3.field_0x1c == 0x0) && (i_var3.field_0x1e != 0x0)) {
        iStack8 = 0x1; u_var4 = pass1_1030_8326();
        // i_var2 = (u_var4 >> 0x10); if ((i_var2 != 0x0) | | (0x32 < u_var4)) {
        iStack8 = 0x5;
    }
    if ((i_var2 != 0x0) || (0x3c < u_var4)) {
        iStack8 = 0xa;
    }
    if (i_var3.field_0x1e < iStack8) {
        iStack8 = i_var3.field_0x1e;
    }
    pi_var1 = & i_var3.field_0x1e; * pi_var1 = * pi_var1 - iStack8; if ( * pi_var1 < 0x0) {
    i_var3.field_0x1e = 0x0;
    }
    if (0x0 < i_var3.field_0x1e) {
        return iStack8;
    }
    return - 0x1;
}
 return 0x0; }



pub fn pass1_1010_9766(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let in_AX: i16;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x1a) = 0x1;
    pass1_1010_a0a0(param_1, param_2, param_3, param_4);
    pass1_1010_9f8c(param_1, 0x80, param_4);
    (param_1 + 0x1e) = in_AX * 0x32;
    return;
}


pub fn pass1_1010_9794(param_1: u32, param_2: u16) {
    let i_var1: i16;
    let ppcVar2: u32;
    let pu_var3: u32;
    let u_var4: u16;
    let pu_var5: &mut Struct251;
    let puVar6: u32;
    let puVar7: U32Ptr;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: u16;
    let uVar8: u16;
    let extraout_DX_01: u16;
    let i_var9: &mut Struct250;
    let uVar9: u16;
    let local_a: U32Ptr;
    

    // uVar9 = (param_1 >> 0x10);
    i_var9 = param_1;
    if (i_var9.field_0x18 == 0x0) {
        i_var9.field_0x18 = 0x1;
        puVar6 = i_var9.field_0xe;
        u_var4 = (&i_var9.field_0xe + 0x2);
        puVar7 = (u_var4 | puVar6);
        if (puVar7 != 0x0) {
            ppcVar2 = puVar6;
            (**ppcVar2)();
            puVar7 = extraout_dx;
        }
        mem_op_1000_179c(0xc, puVar7, 0x1000);
        u_var4 = puVar6;
        if ((puVar7 | u_var4) == 0x0) {
            u_var4 = 0x0;
            uVar8 = 0x0;
        } else {
            set_struct_1008_574a((puVar6 & 0xffff | ZEXT24(puVar7) << 0x10));
            uVar8 = extraout_DX_00;
        }
        &i_var9.field_0xe = u_var4;
        (&i_var9.field_0xe + 0x2) = uVar8;
        pass1_1008_5784(CONCAT22(param_2, &local_a), i_var9.field_0xa);
        loop {
            pu_var5 = &local_a;
            pass1_1008_5b12(pu_var5, param_2);
            if ((extraout_DX_01 | pu_var5) == 0x0) { break; }
            i_var1 = pu_var5.field_0x4;
            if ((i_var1 == 0x3e) || (i_var1 == 0x41)) {
                puVar6 = i_var9.field_0xa;
                (puVar6 + 0xa) = 0x0;
                puVar6 = i_var9.field_0xa;
                ppcVar2 = (*i_var9.field_0xa + 0xc);
                (**ppcVar2)();
                pu_var3 = i_var9.field_0xa;
                (pu_var3 + 0xa) = 0x1;
                local_a._4_4_ = 0x0;
                ppcVar2 = (*i_var9.field_0xe + 0x4);
                (**ppcVar2)(0x1008, i_var9.field_0xe, CONCAT22(extraout_DX_01, pu_var5), puVar6);
            }
        }
    }
    return;
}


pub fn pass1_1010_988c(param_1: u32, param_2: i16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let i_var3: i16;
    let i_var4: i16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let unaff_SS: u16;
    let lVar8: i32;
    let local_a: [u8; 8];

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pass1_1008_5784(CONCAT22(unaff_SS, local_a), (iVar6 + 0xe));
    loop {
        lVar8 = pass1_1008_5b12(local_a, unaff_SS);
        // u_var5 = (lVar8 >> 0x10);
        i_var3 = lVar8;
        if (lVar8 == 0x0) {
            return;
        }
        if ((i_var3 + 0x4) != param_2) == false { break; }
    }
    i_var4 = (i_var3 + 0x6) + -0x1;
    (i_var3 + 0x6) = i_var4;
    if ((i_var4 < 0x1) && (ppcVar1 = ((iVar6 + 0xe) + 0xc),
                           (**ppcVar1)(0x1008, (iVar6 + 0xe), lVar8),
                           u_var2 = (iVar6 + 0xe), (u_var2 + 0x8) == 0x0)) {
        (iVar6 + 0x16) = 0x1;
    }
    return;
}


pub fn pass1_1010_9f72(param_1: u32, param_2: i16, param_3: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    pass1_1010_9fa6(param_1, u_var1, (param_1 + 0xe), param_2, param_3);
    return;
}


pub fn pass1_1010_9f8c(param_1: u32, param_2: i16, param_3: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    pass1_1010_9fa6(param_1, u_var1, (param_1 + 0xa), param_2, param_3);
    return;
}


pub fn pass1_1010_9fa6(param_1: u16, param_2: u16, param_3: u32, param_4: i16, param_5: u16) -> u16

{
    let u_var1: u16;
    let lVar2: i32;
    let local_a: [u8; 8];

    if (param_3 != 0x0) {
        pass1_1008_5784(CONCAT22(param_5, local_a), param_3);
        loop {
            lVar2 = pass1_1008_5b12(local_a, param_5);
            // u_var1 = (lVar2 >> 0x10);
            if (lVar2 == 0x0) { break; }
            if ((lVar2 + 0x4) == param_4) {
                return (lVar2 + 0x6);
            }
        }
    }
    return 0x0;
}


pub fn pass1_1010_9fee(param_1: u32, param_2: u16, param_3: u16, param_4: u16, param_5: U32Ptr) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let extraout_dx: U32Ptr;
    let i_var3: &mut Struct252;
    let i_var4: &mut Struct253;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let puStack10: U32Ptr;
    let puStack6: U32Ptr;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    pu_var2 = param_5;
    if (i_var3.field_0x12 == 0x0) {
        mem_op_1000_179c(0xc, param_5, 0x1000);
        pu_var2 = (param_5 | param_4);
        if (pu_var2 == 0x0) {
            i_var3.field_0x12 = 0x0;
        } else {
            set_struct_1008_574a(CONCAT22(param_5, param_4));
            &i_var3.field_0x12 = param_4;
            (&i_var3.field_0x12 + 0x2) = extraout_dx;
            pu_var2 = extraout_dx;
        }
    }
    u_var5 = 0x8;
    mem_op_1000_179c(0x8, pu_var2, 0x1000);
    puStack10 = CONCAT22(pu_var2, param_4);
    if ((pu_var2 | param_4) == 0x0) {
        puStack6 = 0x0;
    } else {
        *puStack10 = 0x389a;
        (param_4 + 0x2) = 0x1008;
        *puStack10 = 0xa1c4;
        (param_4 + 0x2) = 0x1010;
        puStack6 = puStack10;
    }
    // u_var4 = (puStack6 >> 0x10);
    i_var4 = puStack6;
    i_var4.field_0x4 = param_3;
    i_var4.field_0x6 = param_2;
    ppcVar1 = (*i_var3.field_0x12 + 0x4);
    (**ppcVar1)(0x1000, i_var3.field_0x12, i_var4, u_var4, u_var5);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_a0a0(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let i_var3: i16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let bVar7: bool;
    let bVar8: bool;
    let lVar9: i32;
    let iStack12: i16;
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_4, local_a), (param_1 + 0xa));
    iStack12 = 0x4;
    mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    if ((ctx.PTR_LOOP_1050_13ae != &ctx.PTR_LOOP_1050_0002) && (ctx.PTR_LOOP_1050_13ae != (&ctx.PTR_LOOP_1050_0000 + 0x1))) {
        iStack12 = 0x2;
    }
    loop {
        loop {
            lVar9 = pass1_1008_5b12(local_a, param_4);
            // u_var6 = (lVar9 >> 0x10);
            i_var4 = lVar9;
            if (lVar9 == 0x0) {
                return;
            }
            i_var2 = (i_var4 + 0x4);
            if (i_var2 != 0x12) { break; }
            pi_var1 = (i_var4 + 0x6);
            bVar8 = SBORROW2(*pi_var1, 0x2);
            i_var3 = *pi_var1 + -0x2;
            bVar7 = i_var3 == 0x0;
//LAB_1010_a151:
            if ((!bVar7 && bVar8) == (i_var3 < 0x0)) {
//LAB_1010_a153:
                pi_var1 = (i_var4 + 0x6);
                *pi_var1 = *pi_var1 - (i_var4 + 0x6) / iStack12;
            }
        }
        if (((i_var2 != 0x3e) && (i_var2 != 0x41)) && (i_var2 != 0x80)) {
            if (i_var2 == 0x83) {
                pi_var1 = (i_var4 + 0x6);
                bVar8 = SBORROW2(*pi_var1, 0x1);
                i_var2 = *pi_var1;
                i_var3 = i_var2 + -0x1;
                bVar7 = i_var2 == 0x1;
//         TODO: goto LAB_1010_a151;
            }
//       TODO: goto LAB_1010_a153;
        }
        i_var2 = (i_var4 + 0x6);
        u_var5 = i_var2 / 0x2;
        pi_var1 = (i_var4 + 0x6);
        *pi_var1 = *pi_var1 - u_var5;
        if (u_var5 == 0x0) {
            u_var5 = 0x1;
        }
        pass1_1010_9fee(param_1, u_var5, (i_var4 + 0x4), u_var5, i_var2 >> 0xf);
    }
}


pub fn pass1_1010_a172(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16

{
    pass1_1010_95f8(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_a198(param_1: U32Ptr, param_2: u8) -> u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_a478(param_1: U32Ptr, param_2: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: &mut Struct497;
    let u_var4: u16;
    let puStack6: U32Ptr;

    // u_var4 = (param_1 >> 0x10);
    u_var3 = param_1;
    *param_1 = 0xe9cc;
    u_var3.field_0x2 = 0x1010;
    u_var3.field_0xa = 0xe9dc;
    u_var3.field_0xc = 0x1010;
    if (u_var3.field_0x138 != 0x0) {
        if (param_1 == 0x0) {
            pu_var1 = 0x0;
            u_var2 = 0x0;
        } else {
            pu_var1 = &u_var3.field_0xa;
            u_var2 = u_var4;
        }
        pass1_1010_1ea6(u_var3.field_0x138, CONCAT22(u_var2, pu_var1), param_2);
    }
    u_var3.field_0x138 = 0x0;
    if (param_1 == 0x0) {
        pu_var1 = 0x0;
        u_var4 = 0x0;
    } else {
        pu_var1 = &u_var3.field_0xa;
    }
    puStack6 = CONCAT22(u_var4, pu_var1);
    *puStack6 = 0x389a;
    pu_var1[0x1] = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1010_a50c(param_1: &mut Struct20, param_2: u32, param_3: u32) {
    let i_var1: i16;
    let i_var2: &mut Struct260;
    let local_8: u32;
    let i_stack4: i16;

    i_var1 = param_1;
    pass1_1000_4906((param_1 & 0xffff0000 | (i_var1 + 0xa4)),
                    0x0, 0x94);
    i_var2 = ((param_3 + 0xa) * 0x6 + i_var1);
    local_8 = i_var2.field_0xe;
    i_stack4 = i_var2.field_0x12;
    (*local_8)(0x1000, i_var1 + i_stack4, param_1._2_2_, param_2, param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_a568(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16,
)

{
    pass1_1030_8344(
        ctx, ctx.PTR_LOOP_1050_5748, 0x8000001);
    pass1_1030_2622(CONCAT22(param_5, param_4), param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_a58a(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16,
)

{
    pass1_1030_8344(
        ctx, ctx.PTR_LOOP_1050_5748, 0x8000001);
    pass1_1030_266c(param_4, CONCAT22(param_3, param_5));
    return;
}


pub fn pass1_1010_a5ac(param_1: u16, param_2: u16, param_3: u32) -> u16

{
    let u_var1: u32;

    u_var1 = struct_op_1030_73a8(param_3);
    return (u_var1 + 0x20);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_a5ca(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16,
)

{
    pass1_1030_8344(
        ctx, ctx.PTR_LOOP_1050_5748, 0x8000001);
    pass1_1030_2242(CONCAT22(param_5, param_4), param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_a5ec(param_1: u16, param_2: u16, param_3: u16, param_4: u32, param_5: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let extraout_dx: u16;
    let puVar6: u32;
    let uStack6: u32;

    u_var2 = param_4._2_2_ | param_4;
    if (param_4 != 0x0) {
        pass1_1030_8344(
            ctx, ctx.PTR_LOOP_1050_5748, 0x8000001);
        uStack6 = CONCAT22(param_5, u_var2);
        puVar6 = struct_op_1030_73a8(param_4);
        // u_var5 = (puVar6 >> 0x10);
        u_var4 = (puVar6 + 0x20);
        if (u_var4 != param_3) {
            u_var3 = param_3;
            pass1_1010_a5ca(param_1, param_2, u_var4, param_3, u_var5);
            if ((u_var4 != 0x70) && (u_var3 < 0x0)) {
                pass1_1030_25d8(CONCAT22(param_5, u_var2), u_var3 + 0x1, u_var4);
            }
            ppcVar1 = (*puVar6 + 0x8);
            u_var4 = param_3;
            (**ppcVar1)();
            if (param_3 != 0x70) {
                pass1_1010_a5ca(param_1, param_2, param_3, u_var4, extraout_dx);
                if (u_var4 < 0x0) {
                    pass1_1030_25d8(uStack6, u_var4 - 0x1, param_3);
                }
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_a69c(param_1: u32, param_2: u16, param_3: i16, param_4: U32Ptr, param_5: i16,
                       param_6: u16)

{
    let i_var1: i16;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let paVar5: &mut Struct25;
    let paVar6: &mut Struct67;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let uStack22: u16;
    let iStack20: i16;

    pass1_1030_8344(
        ctx, ctx.PTR_LOOP_1050_5748, 0x8000001);
    if (param_2 == 0x1) {
        pu_var3 = param_4;
// TODO: refactor for loop
        // for (iStack20 = 0x0; iStack20 < 0x83; iStack20 += 0x1) {
        //   i_var1 = pass1_1030_2242(CONCAT22(param_4,param_3),iStack20);
        //   if (0x19 < i_var1) {
        //     uStack22 = i_var1 - 0x5;
        //     if (uStack22 < 0x19) {
        //       uStack22 = 0x19;
        //     }
        //     pass1_1030_25d8(CONCAT22(param_4,param_3),uStack22,iStack20);
        //   }
        // }
//     TODO: goto switchD_1010_aaef_caseD_b;
    }
    pu_var3 = param_4;
    pass1_1030_25f0(CONCAT22(param_4, param_3), 0x0, param_2);
    puVar7 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x35, param_6, pu_var3, param_5);
    // pu_var3 = (puVar7 >> 0x10);
    if (false) {
        // goto
        // switchD_1010_aaef_caseD_b;
    }
    u_var2 = param_1;
    // uVar8 = (param_1 >> 0x10);
    puVar4 = pu_var3;
    match(param_2)
    {
        0xa | 0xc => {i_var1 = 0x1b;
}        
        _ => {}
//     TODO: goto switchD_1010_aaef_caseD_b;
        0x10 => {pass1_1010_682e(puVar7, 0x1, 0x2d);
        if ((param_3 + 0x160) == 0x0) {
            // goto
            // switchD_1010_aaef_caseD_b;
        }
        i_var1 = 0x2d;}
//     TODO: goto LAB_1010_a91f;
        0x12 => {pass1_1010_682e(puVar7, 0x1, 0x16);
        pass1_1010_682e(puVar7, 0x1, 0x17);
        pass1_1010_682e(puVar7, 0x1, 0x18);
        pass1_1010_682e(puVar7, 0x1, 0x40);
        i_var1 = 0x3f;}
//     TODO: goto LAB_1010_a96c;
        0x13 => {i_var1 = 0x35;}
//     TODO: goto LAB_1010_a91f;
        0x19 =>{}
//     TODO: goto switchD_1010_aaef_caseD_19;
        0x1a => {i_var1 = 0xf;}
//     TODO: goto LAB_1010_a96c;
        0x1c => {i_var1 = 0x11;}
//     TODO: goto LAB_1010_a96c;
        0x1d | 0x24 => {pass1_1010_abd2(u_var2, uVar8, 0x1e, pu_var3, param_5, param_6);
        i_var1 = 0x5b;}
//     TODO: goto LAB_1010_a91f;
        0x1e => {u_var2 = 0x1;
        i_var1 = 0x2;
        puVar7 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_6, pu_var3, param_5);
        // pu_var3 = (puVar7 >> 0x10);
        pass1_1010_08c0(puVar7, u_var2, i_var1, param_6);
        paVar5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x40, param_6, pu_var3, param_5);
        // pu_var3 = (paVar5 >> 0x10);
        load_str_and_sprintf_1008_b69c(paVar5, param_6, pu_var3);}
//     TODO: goto switchD_1010_aaef_caseD_b;
        0x22 => {i_var1 = 0x8;}
//     TODO: goto LAB_1010_aabe;
        0x23 => {i_var1 = 0xc;}
//     TODO: goto LAB_1010_aabe;
        0x25 => {pass1_1010_abd2(u_var2, uVar8, 0x14, pu_var3, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x1b, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x1e, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x22, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x25, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x28, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x2a, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x2d, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x2f, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x31, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x35, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x38, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x3a, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x3c, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x48, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x4f, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x52, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x54, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x57, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x5b, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x5d, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x62, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x66, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x68, puVar4, param_5, param_6);
        pass1_1010_abd2(u_var2, uVar8, 0x6c, puVar4, param_5, param_6);}
//     TODO: goto switchD_1010_aaef_caseD_19;
        0x29 => {i_var1 = 0x25;}
        
        0x2a =>{ i_var1 = 0xf;}
//     TODO: goto LAB_1010_aabe;
        0x2b => {i_var1 = 0x6e;}
//     TODO: goto LAB_1010_a96c;
        0x30 => {i_var1 = 0x54;}
        
        0x33 => {pass1_1010_abd2(u_var2, uVar8, 0x31, pu_var3, param_5, param_6);
        i_var1 = 0x6c;}
//     TODO: goto LAB_1010_a91f;
        0x36 => {i_var1 = 0x13;}
//     TODO: goto LAB_1010_aabe;
        0x37 => { 
//LAB_1010_a96c:
        pass1_1010_682e(puVar7, 0x1, i_var1);}
//     TODO: goto switchD_1010_aaef_caseD_b;
        0x38 => {pass1_1010_682e(puVar7, 0x1, 0x28);
        if ((param_3 + 0x160) == 0x0) {
            // goto
            // switchD_1010_aaef_caseD_b;
        }
        i_var1 = 0x28;}
//     TODO: goto LAB_1010_a91f;
        0x39 => {i_var1 = 0x10;}
//     TODO: goto LAB_1010_aabe;
        0x3a => {i_var1 = 0x11;}
//     TODO: goto LAB_1010_aabe;
        0x3b =>{ i_var1 = 0x12;
//LAB_1010_aabe:
        pass1_1010_6814(puVar7, 0x1, i_var1);}
//     TODO: goto switchD_1010_aaef_caseD_b;
        0x3c => {pass1_1010_abd2(u_var2, uVar8, 0x14, pu_var3, param_5, param_6);
        i_var1 = 0x62;}
//     TODO: goto LAB_1010_a91f;
        0x3d => {pass1_1010_682e(puVar7, 0x1, 0x66);
        if ((param_3 + 0x160) == 0x0) {
            // goto
            // switchD_1010_aaef_caseD_b;
        }
        i_var1 = 0x66;
//LAB_1010_a91f:
        pass1_1010_abd2(u_var2, uVar8, i_var1, pu_var3, param_5, param_6);}
//     TODO: goto switchD_1010_aaef_caseD_b;
        0x3e => {i_var1 = 0x5d;}
        
        0x3f => {i_var1 = 0x22;}
        
        0x40 => {i_var1 = 0x57;}
        
        0x41 => {i_var1 = 0x4f;}
    }
    pass1_1010_abd2(u_var2, uVar8, i_var1, pu_var3, param_5, param_6);
    // switchD_1010_aaef_caseD_b:
     paVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_6, pu_var3, param_5);
    pu_var3 = (paVar6 >> 0x10);
    u_var2 = pass1_1008_ab12(paVar6, pu_var3, param_2);
    if (u_var2 != 0x0) {
        post_win_msg_1008_a0e4(paVar6, 0x0, 0x0, 0x1, 0x0, u_var2, 0x1008, param_6);
    }
    post_win_msg_1008_a0e4(paVar6, 0x0, 0x0, 0x1, 0x0, 0x3d, 0x1008, param_6);
    u_var10 = 0x400;
    i_var1 = 0x6;
    uVar9 = 0x1;
    puVar7 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_6, pu_var3, param_5);
    pass1_1010_043a(puVar7, CONCAT22(u_var10, uVar9), i_var1, param_6);
    return;
    // switchD_1010_aaef_caseD_19: 
    puVar7 + 0x148 = 0x34;
    pu_var3 = puVar4;
//   TODO: goto switchD_1010_aaef_caseD_b;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_abd2(param_1: u16, param_2: u16, param_3: i16, param_4: U32Ptr, param_5: i16,
                       param_6: u16)

{
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let iStack20: i16;
    let iStack16: i16;
    let iStack14: i16;
    let iStack12: i16;
    let uStack10: u32;
    let puStack6: U32Ptr;

    puStack6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x35, param_6, param_4, param_5);
    u_var2 = puStack6 + 0xa;
    uStack10 = puStack6 & 0xffff0000 | u_var2;
    iStack12 = 0x0;
    iStack16 = param_3;
    _iStack20 = CONCAT22(param_6, &stack0x000a);
    loop {
        pi_var1 = _iStack20;
        if (iStack16 == 0x0) {
            return;
        }
        if (iStack12 != 0x0) { break; }
        if ((iStack16 * 0x2 + u_var2) != 0x0) {
            iStack12 = 0x1;
            iStack14 = iStack16;
        }
        _iStack20 = (_iStack20 & 0xffff0000 | (iStack20 + 0x2));
        iStack16 = *pi_var1;
    }
    pass1_1010_682e(puStack6, 0x0, iStack14);
    pass1_1010_682e(puStack6, 0x1, iStack16);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_ac62(param_1: u16, param_2: u16, param_3: i16, param_4: u16, param_5: u16) -> u16

{
    pass1_1030_8344(
        ctx, ctx.PTR_LOOP_1050_5748, 0x8000001);
    return (param_4 + 0x116 + param_3 * 0x2);
}


pub fn pass1_1010_acc0(param_1: u16, param_2: u16, param_3: u32) -> u16

{
    let u_var1: u32;

    u_var1 = struct_op_1030_73a8(param_3);
    if ((u_var1 + 0x12) != 0x4) {
        return 0x1;
    }
    return 0x0;
}


pub fn pass1_1010_acec(param_1: u32, param_2: i16, param_3: u16) {
    if (param_2 == 0x1) {
        (param_1 + 0x12e) = 0x0;
    } else {
        if (param_2 != 0x5) {
            return;
        }
    }
    pass1_1010_1f62(param_3, param_1 & 0xffff0000 | (param_1 - 0xa), param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_ad22(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u32;
    let pu_var2: U32Ptr;
    let uStack8: u16;

    u_var1 = (param_1 + 0x138);
    pu_var2 = &param_5;
    pass1_1030_627e(param_3, pu_var2, param_2, _PTR_LOOP_1050_5740,
                    CONCAT22(param_3, pu_var2), (u_var1 + 0x20));
    if ((param_2 | pu_var2) == 0x0) {
        return;
    }
    uStack8 = param_2;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, pu_var2);
    return;
}


pub fn pass1_1010_ad64(param_1: u16, param_2: u32, param_3: u32, param_4: u32, param_5: u16) {
    if (param_3 != 0x0) {
        param_4 = (param_3 + 0x2e);
        if ((param_4 + 0x200) == 0x8000002) {
            return;
        }
    }
    pass1_1010_c58as(param_1, param_2, (param_2 >> 0x10), param_3,
                     param_4, param_5);
    return;
}


pub fn pass1_1010_ae12(param_1: u16, param_2: u16, param_3: u32, param_4: i16, param_5: u16) -> u16

{
    let mut pcVar1: String;
    let i_var2: i16;
    let uStack4: u16;

    if (param_4 == 0x6) {
        // TODO: refactor for loop
        // for (uStack4 = 0x0; uStack4 < 0x15; uStack4 += 0x1) {
        //   pcVar1 = string_op_1020_c222(uStack4);
        //   i_var2 = pass1_1000_3d7a(param_3,CONCAT22(param_5,pcVar1));
        //   if (i_var2 == 0x0) {
        //     return uStack4;
        //   }
        // }
    } else {
        if (param_4 == 0x7) {
            // TODO: refactor for loop
            // for (uStack4 = 0x0; uStack4 < 0x11; uStack4 += 0x1) {
            //   pcVar1 = string_op_1020_c2f8(uStack4);
            //   i_var2 = pass1_1000_3d7a(param_3,CONCAT22(param_5,pcVar1));
            //   if (i_var2 == 0x0) {
            //     return uStack4;
            //   }
            // }
        }
    }
    return 0xffff;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_ae92(param_1: u32, param_2: u16, param_3: u16, param_4: u32, param_5: i16,
                       param_6: u16)

{
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u32;
    let paVar4: &mut Struct67;
    let u_var5: u16;
    let u_var6: u16;
    let iVar7: i16;
    let uVar8: u8;
    let uVar9: u8;
    let u_var10: u16;
    let u_var11: u16;
    let iVar12: i16;

    if (param_3 == 0x15) {
        u_var3 = struct_op_1030_73a8(param_4);
        if (u_var3 != 0x0) {
            (u_var3 + 0x20) = param_2;
            return;
        }
    } else {
        if (param_3 < 0x16) {
            if (param_3 == '\x06') {
                pass1_1030_7f1a(param_4, param_2, param_6);
                u_var3 = struct_op_1030_73a8(param_4);
                u_var1 = pass1_1010_b028(param_1, u_var3);
                u_var3 = pass1_1030_8326();
                // pu_var2 = (u_var3 >> 0x10);
                if (((u_var1 == 0xe) && ((pu_var2 != 0x0 || (0x32 < u_var3)))) && ((param_2 == 0x1 || (((param_2 == 0x2 || (param_2 == 0x4)) || (param_2 == 0x3)))))) {
                    u_var11 = 0x0;
                    iVar12 = 0xb;
                    uVar8 = 0x1;
                    uVar9 = 0x0;
                    u_var10 = 0x0;
                    u_var6 = 0x0;
                    iVar7 = 0x0;
                    u_var5 = 0x0;
                    paVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_6, pu_var2, param_5);
                    post_win_msg_1008_a0e4(paVar4, CONCAT22(u_var6, u_var5), iVar7, CONCAT11(uVar9, uVar8),
                                           CONCAT22(u_var11, u_var10), iVar12, 0x1008, param_6);
                    return;
                }
            } else {
                if (param_3 == '\x07') {
                    pass1_1030_7eda(param_4, param_2, param_6);
                    return;
                }
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_af66(param_1: u32, param_2: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let in_stack_00000008: u16;

    u_var1 = (param_1 + 0x138);
    u_var2 = (u_var1 + 0x24);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
    u_var3 = param_2 | u_var2;
    if (u_var3 == 0x0) {
        return;
    }
    pass1_1038_5050(u_var2 & 0xffff | param_2 << 0x10, in_stack_00000008, u_var2,
                    u_var3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_afa2(param_1: u32, param_2: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let in_stack_00000008: u16;

    u_var1 = (param_1 + 0x138);
    u_var2 = (u_var1 + 0x24);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
    if ((param_2 | u_var2) == 0x0) {
        return;
    }
    pass1_1038_50e0(u_var2 & 0xffff | param_2 << 0x10, in_stack_00000008, u_var2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_afde(param_1: u32, param_2: i16) {
    let u_var1: u32;
    let u_var2: u32;
    let in_DX: u16;
    let pu_var3: u32;

    u_var1 = (param_1 + 0x138);
    u_var2 = (u_var1 + 0x24);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
    if ((in_DX | u_var2) == 0x0) {
        return;
    }
    pu_var3 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, param_2);
    pass1_1038_4e78(pu_var3, (pu_var3 >> 0x10),
                    u_var2 & 0xffff | in_DX << 0x10, pu_var3);
    return;
}


pub fn pass1_1010_b028(param_1: u32, param_3: u32) -> u16

{
    return (param_3 + 0xc);
}


pub fn pass1_1010_bf1e(param_1: u32, param_2: &mut i16, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let u_var1: u32;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let i_var4: i16;
    let u_var5: u16;
    let uStack40: u32;
    let iStack36: i16;
    let uStack32: u16;
    let puStack26: U32Ptr;
    let local_16: [u8; 12];
    let i_stack4: i16;

    bad_1010_bf08(param_1, (param_1 >> 0x10), 0x1000000);
    i_stack4 = param_3 + -0x1;
    *param_2 = i_stack4;
    u_var2 = i_stack4 * 0x18;
    mem_op_1000_179c(u_var2, param_4, 0x1000);
    uStack40 = CONCAT22(param_4, u_var2);
    uStack32 = param_4 | u_var2;
    i_var4 = param_2;
    // u_var5 = (param_2 >> 0x10);
    if (uStack32 == 0x0) {
        (i_var4 + 0x2) = 0x0;
    } else {
        pass1_1000_5586(0x4092, 0x1020, i_stack4, 0x18, u_var2, param_4);
        (i_var4 + 0x2) = uStack40;
    }
    pass1_1028_dc52(CONCAT22(param_5, local_16), 0x1, 0x0, 0x100);
    puStack26 = (i_var4 + 0x2);
    iStack36 = 0x0;
    loop {
        pu_var3 = local_16;
        pass1_1028_e4ec(CONCAT22(param_5, pu_var3));
        if ((uStack32 | pu_var3) == 0x0) { break; }
        u_var1 = (pu_var3 + 0x10);
        uStack32 = uStack32 | pu_var3;
        if (u_var1 != 0x0) {
            // uStack32 = (u_var1 >> 0x10);
            pass1_1008_3f62(puStack26, (u_var1 & 0xffff0000 | (u_var1 + 0x4)),
            );
            (puStack26 + 0xc) = iStack36;
            iStack36 += 0x1;
            puStack26 = (puStack26 & 0xffff0000 | (puStack26 + 0x18));
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_bffa(param_1: u32, param_2: i16, param_3: U32Ptr, param_4: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let pu_var5: &mut Struct257;
    let iVar6: &mut Struct254;
    let iVar7: &mut Struct255;
    let i_var8: &mut Struct256;
    let iVar5: i16;
    let u_var6: u16;
    let uVar7: u16;
    let paVar8: &mut Struct43;
    let uStack34: u16;
    let iStack30: i16;
    let local_18: [u8; 16];

    mem_op_1000_179c(0xa, param_3, 0x1000);
    local_18._18_4_ = CONCAT22(param_3, param_2);
    bad_1010_bf08(param_1, (param_1 >> 0x10), 0x2000000);
    u_var6 = (local_18._18_4_ >> 0x10);
    iVar6 = local_18._18_4_;
    iVar6.field_0x8 = param_2;
    if (param_2 == 0x0) {
        iVar6.field_0x8 = 0x1;
    }
    u_var3 = iVar6.field_0x8 << 0x2;
    mem_op_1000_179c(u_var3, param_3, 0x1000);
    u_var6 = (local_18._18_4_ >> 0x10);
    iVar7 = local_18._18_4_;
    local_18._18_4_ = u_var3;
    iVar7.field_0x2 = param_3;
    if ((param_3 | local_18._18_4_) == 0x0) {
        iVar7.field_0x8 = 0x0;
    } else {
        u_var4 = iVar7.field_0x8 * 0x2;
        mem_op_1000_179c(u_var4, param_3, 0x1000);
        u_var6 = (local_18._18_4_ >> 0x10);
        i_var8 = local_18._18_4_;
        i_var8.field_0x4 = u_var4;
        i_var8.field_0x6 = param_3;
        if ((param_3 | i_var8.field_0x4) != 0x0) {
            paVar8 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x1b4, param_4);
            // u_var3 = (paVar8 >> 0x10);
            pu_var1 = *local_18._18_4_;
            *pu_var1 = paVar8;
            (pu_var1 + 0x2) = u_var3;
            (local_18._18_4_ + 0x4) = 0x0;
            pass1_1028_dc52(CONCAT22(param_4, local_18), 0x1, 0x0, 0x200);
            iStack30 = 0x1;
            loop {
                pu_var5 = local_18;
                pass1_1028_e4ec(CONCAT22(param_4, pu_var5));
                if ((u_var3 | pu_var5) == 0x0) { break; }
                u_var6 = *pu_var5.field_0x10;
                uStack34 = 0x0;
                match(u_var6)
                {
                    0x1 => {uStack34 = 0x1b5;}
                    
                    0x2 => {uStack34 = 0x1b6;}
                    
                    0x3 => {uStack34 = 0x1b7;}
                    
                    0x4 => {uStack34 = 0x1b8;}
                    
                    0x5 => {uStack34 = 0x1b9;}
                    
                    0x6 => {uStack34 = 0x1ba;}
                    
                    0x7 => {uStack34 = 0x1bb;}
                    
                    0x8 => {uStack34 = 0x1bc;}
                    
                    0x9 => {uStack34 = 0x1bd;}
                    
                    0xa => {uStack34 = 0x1be;}
                }
                paVar8 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, uStack34, param_4);
                // u_var3 = (paVar8 >> 0x10);
                uVar7 = (*local_18._18_4_ >> 0x10);
                iVar5 = *local_18._18_4_;
                (iVar5 + iStack30 * 0x4) = paVar8;
                (iVar5 + iStack30 * 0x4 + 0x2) = u_var3;
                u_var2 = (local_18._18_4_ + 0x4);
                (u_var2 + iStack30 * 0x2) = u_var6;
                iStack30 += 0x1;
            }
            return;
        }
    }
    return;
}


pub fn pass1_1010_c1ba(param_1: u16, param_2: u16, param_3: i16, param_4: u16, param_5: u16) {
    let pu_var1: U32Ptr;
    let iStack28: i16;
    let local_16: [u8; 12];
    let uStack4: u16;

    uStack4 = 0x0;
    pass1_1028_dc52(CONCAT22(param_5, local_16), 0x1, 0x0, 0x200);
    iStack28 = 0x1;
    loop {
        pu_var1 = local_16;
        pass1_1028_e4ec(CONCAT22(param_5, pu_var1));
        param_4 |= pu_var1;
        if ((param_4 == 0x0) || (iStack28 == param_3)) { break; }
        iStack28 += 0x1;
    }
    return;
}


pub fn pass1_1010_c234(param_1: u16, param_2: U32Ptr, param_3: i16, param_4: u16) -> U32Ptr

{
    let mut pcVar1: String;

    pass1_1010_c28a(param_2, param_3, param_4);
    if ((param_2 | param_1) == 0x0) {
        return 0x0;
    }
    pcVar1 = load_string_1038_4d28(CONCAT22(param_2, param_1));
    return pcVar1;
}


pub fn pass1_1010_c25e(param_1: u16, param_2: u16, param_3: &mut String, param_4: u16, param_5: U32Ptr,
                       param_6: i16, param_7: u16)

{
    if (param_3 != 0x0) {
        pass1_1010_c28a(param_5, param_6, param_7);
        if ((param_5 | param_4) != 0x0) {
            pass1_1038_4d3c(CONCAT22(param_5, param_4), param_3, param_5 | param_4);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_c28a(param_1: U32Ptr, param_2: i16, param_3: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let pu_var5: U32Ptr;

    pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_3, param_1, param_2);
    // u_var2 = (pu_var5 >> 0x10);
    u_var1 = (pu_var5 + 0x24);
    u_var4 = (pu_var5 + 0x26);
    u_var3 = u_var4 | u_var1;
    if (u_var3 != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
        if ((u_var4 | u_var3) != 0x0) {
            return;
        }
    }
    return;
}


pub fn pass1_1010_c2d8(param_1: u32, param_3: i32) {
    let u_var1: u16;
    let uStack6: u16;

    if ((param_3 != 0x0) && (u_var1 = (param_3 >> 0x10),
                             uStack6 = (param_3 + 0x2e),
                             ((param_3 + 0x30) | uStack6) != 0x0)) {
        return;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_c312() -> u32

{
    return CONCAT22((ctx.PTR_LOOP_1050_65e2 + 0x2), *_PTR_LOOP_1050_65e2);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_c320(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: u32, param_5: U32Ptr) {
    let u_var1: u32;
    let puStack6: U32Ptr;

    puStack6 = param_3;
    if (param_3 == 0x0) {
        mem_op_1000_179c(0x100, param_5, 0x1000);
        puStack6 = (param_3 & 0xffff | ZEXT24(param_5) << 0x10);
    }
    u_var1 = struct_op_1030_73a8(param_4);
    match((u_var1 + 0x12))
    {
        0x1 | 0x2 | 0x4 => {}
        0x3 | 0x5 => {}
        0x6 => {}
        0x7 => {}
        0x8 => {}
        0x9 => {}
        _ => {puStack6 = '\0';
        return;}
    }
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                          (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x3ff, puStack6,
                          (puStack6 >> 0x10));
    return;
}


pub fn pass1_1010_c3c2(param_1: u16, param_2: u16, param_3: u32, param_4: u32, param_5: U32Ptr,
                       param_6: u8, param_7: u16)

{
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let local_40c: [u8; 400];
    let uStack12: u16;
    let uStack10: u32;
    let uStack6: u32;

    uStack6 = param_3;
    if (param_3 == 0x0) {
        mem_op_1000_179c(0x100, param_5, 0x1000);
        uStack6 = param_3 & 0xffff | ZEXT24(param_5) << 0x10;
    }
    uStack10 = struct_op_1030_73a8(param_4);
    // u_var2 = (uStack10 >> 0x10);
    uStack12 = (uStack10 + 0xc);
    u_var3 = u_var2;
    u_var1 = pass1_1020_bd80(uStack12);
    string_1000_3d3e(CONCAT22(param_7, local_40c), CONCAT22(u_var3, u_var1));
    pass1_1030_8086(param_4);
    sys_1000_3f9c(uStack6, (uStack6 >> 0x10), 0x38c5,
                  ctx.data_seg, local_40c, &stack0xfffe, u_var2, 0x1000,
                  param_7, param_6);
    return;
}


pub fn pass1_1010_c58as(
    param_1: u16, 
    param_2: u16, 
    param_3: u16, 
    param_4: u32, 
    param_5: u16, 
    param_6: u16)
{
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u32;
    let pu_var4: U32Ptr;
    let pu_var5: U32Ptr;
    let unaff_s_s: u16;
    let pi_stack26: U32Ptr;
    let u_stack18: u16;
    let i_stack16: i16;
    let ppu_stack14: U32Ptr;
    let pi_stack6: U32Ptr;

    u_var3 = param_5;
    mem_op_1000_179c(0x18, param_6, 0x1000);
    u_var1 = u_var3;
    pu_var4 = (param_6 | u_var1);
    if (pu_var4 == 0x0) {
        u_var1 = 0x0;
        pu_var4 = 0x0;
    } else {
        struct_1040_a598((u_var3 & 0xffff | param_6 << 0x10));
    }
    pi_stack6 = CONCAT22(pu_var4, u_var1);
    pu_var5 = (pu_var4 | u_var1);
    if (pu_var5 == 0x0) {
        return;
    }
    ppu_stack14 = 0x0;
    u_stack18 = 0x0;
    i_stack16 = 0x0;
    if (true) {
        match(param_3)
        {
            0x5 => {ppu_stack14 = &ctx.USHORT_1050_352c;
            u_stack18 = 0xfa4;
            i_stack16 = 0x30;
            }
                        _ => {}
//       TODO: goto switchD_1010_c717_caseD_6;
            0xa =>{ ppu_stack14 =  &ctx.USHORT_1050_358c;
            u_stack18 = 0xfb3;
            i_stack16 = 0x51;
            }
            0xb => {ppu_stack14 =  &ctx.USHORT_1050_358c;
            u_stack18 = 0xfb4;
            i_stack16 = 0x51;
            }
            0xc => {ppu_stack14 =  &ctx.USHORT_1050_362e;
            u_stack18 = 0xfb6;
            i_stack16 = 0x30;
            }
            0x10 => {ppu_stack14 = &ctx.PTR_DAT_1050_1805_1050_368e;
            u_stack18 = 0xfc4;
            i_stack16 = 0x3c;
            }
            0x11 => {ppu_stack14 = &ctx.PTR_DAT_1050_1805_1050_3706;
            u_stack18 = 0xfc1;
            i_stack16 = 0x4b;
            }
            0x12 => {ppu_stack14 =  &ctx.USHORT_1050_379c;
            u_stack18 = 0xfc5;
            i_stack16 = 0x8;
            }
            0x13 => {pu_var5 = pu_var4;
            pass1_1010_debe(CONCAT22(param_2, param_1), param_3,
                            CONCAT22(pu_var4, u_var1 + 0x10),
                            CONCAT22(pu_var4, u_var1 + 0xc), param_4, unaff_s_s);
            ppu_stack14 =  &ctx.USHORT_1050_37ac;
            u_stack18 = 0xfc6;
            i_stack16 = 0x1;
            }
            0x15 => {(u_var1 + 0x6) = param_4;
            (u_var1 + 0xa) = param_3;
            }
            0x16 => {ppu_stack14 =  &ctx.USHORT_1050_37ae;
            u_stack18 = 0x157;
            i_stack16 = 0x4;
            }
            0x17 => {ppu_stack14 =  & USHORT_1050_37b6;
            i_stack16 = 0x2c;
            u_stack18 = 0xfd8;
        if ppu_stack14 != 0x0 {
            pi_stack6 = i_stack16;
            u_var2 = i_stack16 * 0xa + 0x2;
            mem_op_1000_179c(u_var2, pu_var5, 0x1000);
            pi_stack26 = CONCAT22(pu_var5, u_var2);
            if ((pu_var5 | u_var2) == 0x0) {
                (u_var1 + 0x2) = 0x0;
            } else {
                *pi_stack26 = i_stack16;
                pass1_1000_5586(0xa564, &ctx.PTR_LOOP_1050_1040, i_stack16, 0xa,
                                u_var2 + 0x2, pu_var5);
                (u_var1 + 0x2) = u_var2 + 0x2;
                (u_var1 + 0x4) = pu_var5;
            }
            (u_var1 + 0x6) = param_4;
            (u_var1 + 0xa) = param_3;
            (u_var1 + 0x12) = u_stack18;
            pass1_1010_a50c(CONCAT22(param_2, param_1), ppu_stack14,
                            CONCAT22(pu_var4, u_var1));}
        
        return;}
    }
    // switchD_1010_c717_caseD_6: 
    if (pi_stack6 != 0x0) {
    pass1_1040_a5d0(CONCAT22(pu_var4, u_var1));
    fn_ptr_1000_17ce(ctx, CONCAT22(pu_var4, u_var1), 0x1000);
}}
    return;
}


pub fn pass1_1010_c7e2(param_1: u32, param_2: u32, param_3: &mut i16) {
    let u_var1: u32;
    let mut pcVar2: String;
    let in_DX: u16;
    let i_var3: i16;
    let unaff_SI: i16;
    let u_var4: u16;
    let puStack8: U32Ptr;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        // u_var4 = (param_3 >> 0x10);
        i_var3 = param_3;
        if (*param_3 == i_stack4 || *param_3 < i_stack4) { break; }
        u_var1 = (i_var3 + 0x2);
        (i_stack4 * 0xa + u_var1 + 0x4) = (i_stack4 * 0x2 + param_2);
        i_stack4 += 0x1;
    }
    puStack8 = (i_var3 + 0x2);
    // TODO: refactor for loop
    // for (i_stack4 = 0x0; *param_3 != i_stack4 && i_stack4 <= *param_3; i_stack4 += 0x1) {
    //   u_var1 = (i_var3 + 0x6);
    //   pcVar2 = pass1_1010_b038(param_1,u_var1,(u_var1 >> 0x10)
    //                            ,(puStack8 + 0x4),unaff_SI);
    //   string_1040_a626(puStack8,CONCAT22(in_DX,pcVar2),in_DX);
    //   puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
    // }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_c864(param_1: u32, param_2: U32Ptr, param_3: &mut Struct104, param_4: U32Ptr,
                       param_5: U32Ptr, param_6: u8)

{
    let mut plVar1: long;
    let lVar2: i32;
    let ppc_var3: u32;
    let u_var4: u32;
    let mut pcVar5: String;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u32;
    let uVar9: u16;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var10: u16;
    let iVar11: i16;
    let iVar12: i16;
    let uVar13: u16;
    let uVar14: u16;
    let uVar15: u16;
    let uVar16: u16;
    let uVar17: u16;
    let local_1f0: u32;
    let paStack412: &mut Struct18;
    let uStack408: u32;
    let uStack404: u32;
    let uStack400: u16;
    let uStack398: u32;
    let local_18a: u32;
    let local_f6: u32;
    let puStack98: U32Ptr;
    let iStack94: i16;
    let uStack92: u32;
    let iStack88: i16;
    let uStack86: u16;
    let local_54: [u8; 52];

    uStack86 = 0x0;
    // uVar13 = (param_3 >> 0x10);
    iVar11 = param_3;
    iStack88 = param_3;
    uVar14 = 0x0;
    uStack92 = 0x0;
    uVar16 = param_1;
    // uVar17 = (param_1 >> 0x10);
    pass1_1010_c320(uVar16, uVar17, 0x0, (iVar11 + 0x6), param_4);
    string_1000_3d3e(CONCAT22(param_5, local_54), CONCAT22(param_4, uVar14));
    puStack98 = (iVar11 + 0x2);
    uStack404._2_2_ = (iVar11 + 0x4);
    (puStack98 + 0x4) = *param_2;
    string_1040_a626(puStack98, CONCAT22(param_5, local_54), uStack404._2_2_);
    iStack94 = 0x0;
    pass1_1000_4906(CONCAT22(param_5, &local_f6), 0x0, 0x94);
    uStack404._0_2_ = pass1_1000_4906(CONCAT22(param_5, &local_18a), 0x0, 0x94);
    uStack398 = 0x0;
    // TODO: refactor for loop
    // for (uStack400 = 0x1; uStack400 < 0x25; uStack400 += 0x1) {
    //   uStack404 =
    //               pass1_1030_7c28((iVar11 + 0x6),uStack400,uStack404
    //                               ,uStack404._2_2_,param_5);
    //   uStack404._2_2_ = (uStack404 >> 0x10) | uStack404;
    //   if (uStack404 != 0x0) {
    //     pcVar5 = string_1020_c0d8(uStack400);
    //     uStack408 = CONCAT22(uStack404._2_2_,pcVar5);
    //     uVar9 = uStack404._2_2_ | pcVar5;
    //     if (uVar9 == 0x0) {
    //       unk_str_op_1000_3d3e((&local_f6)[uStack398],s_Null_Ptr_1050_38e1);
    //     }
    //     else {
    //       u_var6 = str_op_1008_60e8(CONCAT22(uStack404._2_2_,pcVar5),uVar9);
    //       (&local_f6 + uStack398) = u_var6;
    //       (&local_f6 + uStack398 * 0x4 + 0x2) = uVar9;
    //     }
    //     (&local_18a + uStack398) = uStack404;
    //     (&local_18a + uStack398 * 0x4 + 0x2) = uStack404._2_2_;
    //     uStack398 += 0x1;
    //   }
    // }
    uStack92 = uStack398;
    if (0x13 < uStack398) {
        iStack94 = 0x1;
    }
    uStack86 = pass1_1010_db2e(uVar16, uVar17, 0x1, CONCAT22(param_5, &local_f6),
                               CONCAT22(param_5, &local_18a), param_2, param_3,
                               param_5, param_6);
    while (uVar8 = uStack398 - 0x1, uStack398 != 0x0) {
        uStack398._0_2_ = uVar8;
        paStack412 = (&local_f6)[uStack398];
        uStack404 = paStack412;
        uStack398 = uVar8;
        fn_ptr_1000_17ce(ctx, paStack412, 0x1000);
    }
    uVar15 = 0x1000;
    uStack398 = uVar8;
    pass1_1000_4906(CONCAT22(param_5, &local_18a), 0x0, 0x54);
    u_var4 = (iVar11 + 0x6);
    // uVar14 = (u_var4 >> 0x10);
    iVar12 = u_var4;
    uStack404 = (iVar12 + 0x1e);
    uVar9 = (iVar12 + 0x20) | uStack404;
    uVar8 = uVar9;
    if (uVar9 != 0x0) {
        uStack398 = 0x0;
        loop {
            u_var4 = uStack404;
            ppc_var3 = (u_var4 + 0x10);
            (**ppc_var3)(uVar15, uStack404, (uStack404 >> 0x10));
            if ((extraout_dx < uStack398._2_2_) || ((extraout_dx <= uStack398._2_2_ && (uVar8 <= uStack398)))) { break; }
            ppc_var3 = (u_var4 + 0x4);
            (**ppc_var3)(uVar15, uStack404, uStack398, uStack398._2_2_);
            uVar9 = uVar8;
            u_var10 = extraout_DX_00 | uVar9;
            if (u_var10 != 0x0) {
                uVar15 = SUB42(&USHORT_1050_1028, 0x0);
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uVar9);
                uStack408 = CONCAT22(u_var10, uVar9);
                if ((u_var10 | uVar9) == 0x0) {
                    return;
                }
                iVar12 = (uVar9 + 0xc);
                uVar8 = (iVar12 - 0x1);
                if (((0x0 < iVar12) && (!SBORROW2(iVar12, 0x1))) && (uVar8 = (iVar12 - 0x6),
                                                                     iVar12 - 0x6 == 0x0 || (iVar12 - 0x1) < 0x5)) {
                    plVar1 = &local_18a + iVar12;
                    *plVar1 = *plVar1 + 0x1;
                }
            }
            uStack398 += 0x1;
        }
        uVar9 = extraout_dx;
        pass1_1000_4906(CONCAT22(param_5, &local_f6), 0x0, 0x54);
        u_var6 = 0x1000;
        pass1_1000_4906(CONCAT22(param_5, &local_1f0), 0x0, 0x54);
        uStack398 = 0x0;
        // TODO: refactor for loop
        // for (uStack400 = 0x1; uStack400 < 0x15; uStack400 += 0x1) {
        //   if ((&local_18a)[uStack400] != 0x0) {
        //     pcVar5 = string_op_1020_c222(uStack400);
        //     u_var10 = uVar9 | pcVar5;
        //     if (u_var10 == 0x0) {
        //       u_var6 = 0x1000;
        //       unk_str_op_1000_3d3e((&local_f6)[uStack398],s_Null_Ptr_1050_38ea);
        //     }
        //     else {
        //       u_var6 = 0x1008;
        //       uVar7 = str_op_1008_60e8(CONCAT22(uVar9,pcVar5),u_var10);
        //       (&local_f6 + uStack398) = uVar7;
        //       (&local_f6 + uStack398 * 0x4 + 0x2) = u_var10;
        //     }
        //     uVar9 = (&local_18a + uStack400 * 0x4 + 0x2);
        //     (&local_1f0 + uStack398) =
        //          (&local_18a + uStack400);
        //     (&local_1f0 + uStack398 * 0x4 + 0x2) = uVar9;
        //     uStack398 += 0x1;
        //   }
        // }
        if (iStack94 == 0x0) {
            iVar12 = (uStack92 >> 0x10) + CARRY2(uStack92, uStack398);
            uStack92 = CONCAT22(iVar12, uStack92 + uStack398);
            if ((iVar12 != 0x0) || (0x13 < uStack92 + uStack398)) {
                iStack94 = 0x1;
            }
        }
        if ((uStack86 < iStack88 - 0x2) && (local_1f0 != 0x0)) {
            iVar12 = string_1010_dcac(u_var6, uVar16, uVar17, uStack86, param_2, param_3);
            uStack86 = iVar12 + 0x1;
            uStack86 = pass1_1010_db2e(uVar16, uVar17, uStack86, CONCAT22(param_5, &local_f6),
                                       CONCAT22(param_5, &local_1f0), param_2,
                                       param_3, param_5, param_6);
        }
        while (lVar2 = uStack398 + -0x1, uStack398 != 0x0) {
            uStack398._0_2_ = lVar2;
            paStack412 = (&local_f6)[uStack398];
            uStack398 = lVar2;
            fn_ptr_1000_17ce(ctx, paStack412, 0x1000);
        }
        if (iStack94 != 0x0) {
            (iVar11 + 0x16) = 0x1;
        }
        uStack398 = lVar2;
        pass1_1010_dc36(uVar16, uVar17, uStack86, param_2, param_3, param_5);
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_cc56(param_1: u32, param_2: u32, param_3: &mut Struct104, param_4: u16,
                       param_5: U32Ptr, param_6: u8)

{
    let mut plVar1: long;
    let u_var2: u32;
    let mut pcVar3: String;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u32;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let local_1a0: u32;
    let uStack332: u32;
    let uStack328: u16;
    let uStack326: u16;
    let uStack324: u32;
    let uStack320: u16;
    let local_13e: u32;
    let local_aa: u32;
    let uStack22: u16;
    let iStack18: i16;
    let uStack16: u16;
    let iStack14: i16;
    let uStack12: u16;
    let uStack10: u32;
    let uStack6: u32;

    // u_var10 = (param_1 >> 0x10);
    uVar9 = param_1;
    u_var2 = (uVar9 + 0x138);
    uVar7 = (u_var2 + 0x24);
    uStack6 = uVar7;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uVar7);
    uStack10 = uVar7 & 0xffff | param_4 << 0x10;
    uStack324._2_2_ = param_4 | uVar7;
    if (uStack324._2_2_ != 0x0) {
        iStack14 = param_3;
        iStack18 = 0x0;
        pass1_1000_4906(CONCAT22(param_5, &local_aa), 0x0, 0x94);
        pass1_1000_4906(CONCAT22(param_5, &local_13e), 0x0, 0x94);
        uStack12 = 0x0;
        uStack16 = 0x0;
        uStack22 = 0x0;
// TODO: refactor for loop
        // for (uStack320 = 0x1; uStack320 < 0x25; uStack320 += 0x1) {
        //   uStack324 = (uStack10 + 0x26 + uStack320 * 0x4);
        //   if (uStack324 != 0x0) {
        //     pcVar3 = string_1020_c0d8(uStack320);
        //     uStack332 = uStack332 & 0xffff | ZEXT24(pcVar3) << 0x10;
        //     uVar8 = uStack324._2_2_ | pcVar3;
        //     uStack328 = uStack324._2_2_;
        //     if (uVar8 == 0x0) {
        //       unk_str_op_1000_3d3e((&local_aa)[uStack22],s_Null_Ptr_1050_38f3);
        //     }
        //     else {
        //       u_var4 = str_op_1008_60e8(CONCAT22(uStack324._2_2_,pcVar3),uVar8);
        //       (&local_aa + uStack22) = u_var4;
        //       (&local_aa + uStack22 * 0x4 + 0x2) = uVar8;
        //     }
        //     (&local_13e + uStack22) = uStack324;
        //     (&local_13e + uStack22 * 0x4 + 0x2) = uStack324._2_2_;
        //     uStack22 += 0x1;
        //   }
        // }
        uStack16 = uStack22;
        if (0x13 < uStack22) {
            iStack18 = 0x1;
        }
        uStack12 = pass1_1010_db2e(uVar9, u_var10, 0x1, CONCAT22(param_5, &local_aa),
                                   CONCAT22(param_5, &local_13e), param_2, param_3, param_5, param_6);
        pass1_1000_4906(CONCAT22(param_5, &local_13e), 0x0, 0x54);
// TODO: refactor for loop
        // for (uStack332._0_2_ = 0x1; uStack332 < 0x15; uStack332 += 0x1) {
        //   uStack326 = uStack332;
        //   if ((uStack10 + 0x14e + uStack332 * 0x4) != 0x0) {
        //     if (((0x0 < uStack332) && (!SBORROW2(uStack332,0x1))) &&
        //        ((uStack332 - 0x1) < 0x6)) {
        //       plVar1 = &local_13e + uStack332;
        //       *plVar1 = *plVar1 + 0x1;
        //     }
        //   }
        // }
        pass1_1000_4906(CONCAT22(param_5, &local_aa), 0x0, 0x54);
        u_var4 = 0x1000;
        pass1_1000_4906(CONCAT22(param_5, &local_1a0), 0x0, 0x54);
// TODO: refactor for loop
        // for (uStack332 = 0x10000; uStack332._2_2_ < 0x15;
        //     uStack332 = uStack332 & 0xffff | (uStack332._2_2_ + 0x1) << 0x10) {
        //   if ((&local_13e)[uStack332._2_2_] != 0x0) {
        //     pcVar3 = string_op_1020_c222(uStack332._2_2_);
        //     uStack324 = CONCAT22(uStack324._2_2_,pcVar3);
        //     uVar8 = uStack324._2_2_ | pcVar3;
        //     if (uVar8 == 0x0) {
        //       u_var4 = 0x1000;
        //       unk_str_op_1000_3d3e((&local_aa)[uStack332],s_Null_Ptr_1050_38fc);
        //     }
        //     else {
        //       u_var4 = 0x1008;
        //       u_var5 = str_op_1008_60e8(CONCAT22(uStack324._2_2_,pcVar3),uVar8);
        //       (&local_aa + uStack332) = u_var5;
        //       (&local_aa + uStack332 * 0x4 + 0x2) = uVar8;
        //     }
        //     uStack324._2_2_ = (&local_13e + uStack332._2_2_ * 0x4 + 0x2);
        //     (&local_1a0 + uStack332) =
        //          (&local_13e + uStack332._2_2_);
        //     (&local_1a0 + uStack332 * 0x4 + 0x2) = uStack324._2_2_;
        //     uStack332 = uStack332 & 0xffff0000 | (uStack332 + 0x1);
        //   }
        // }
        if (iStack18 == 0x0) {
            uStack16 += uStack332;
            if (0x13 < uStack16) {
                iStack18 = 0x1;
            }
        }
        if ((uStack12 < iStack14 - 0x2) && (local_1a0 != 0x0)) {
            iVar6 = string_1010_dcac(u_var4, uVar9, u_var10, uStack12, param_2, param_3);
            uStack12 = iVar6 + 0x1;
            uStack12 = pass1_1010_db2e(uVar9, u_var10, uStack12, CONCAT22(param_5, &local_aa),
                                       CONCAT22(param_5, &local_1a0), param_2, param_3,
                                       param_5, param_6);
        }
        if (iStack18 != 0x0) {
            (param_3 + 0x16) = 0x1;
        }
        pass1_1010_dc36(uVar9, u_var10, uStack12, param_2, param_3, param_5);
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1010_cf36(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: u8, param_5: U32Ptr) {
    let u_var1: u32;
    let pu_var2: U32Ptr;
    let mut pcVar3: String;
    let u_var4: u16;
    let u_var5: u16;
    let in_DX: u16;
    let u_var6: u16;
    let unaff_SI: i16;
    let iVar7: i16;
    let i_var9: &mut Struct494;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u32;
    let uVar12: u32;
    let uVar13: u16;
    let uVar14: u16;
    let uVar15: u16;
    let uStack326: u16;
    let iStack316: i16;
    let uStack314: u16;
    let iStack312: i16;
    let local_136:[u16;0x4a];
    let local_a2: u32;
    let iStack14: i16;
    let uStack12: u32;
    let puStack8: U32Ptr;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        // uVar8 = (param_2 >> 0x10);
        iVar7 = param_2;
        // uVar9 = (param_3 >> 0x10);
        i_var9 = param_3;
        pu_var2 = i_var9.field_0x2;
        (i_stack4 * 0xa + pu_var2 + 0x4) = (i_stack4 * 0x2 + iVar7);
        i_stack4 += 0x1;
        if (i_stack4 < 0x8) == false { break; }
    }
    puStack8 = i_var9.field_0x2;
    i_stack4 = 0x0;
    loop {
        u_var1 = i_var9.field_0x6;
        pcVar3 = pass1_1010_b038(param_1, u_var1, (u_var1 >> 0x10),
                                 (puStack8 + 0x4), unaff_SI);
        string_1040_a626(puStack8, CONCAT22(in_DX, pcVar3), in_DX);
        i_stack4 += 0x1;
        puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
        if (i_stack4 < 0x8) == false { break; }
    }
    uVar13 = param_1;
    // uVar14 = (param_1 >> 0x10);
    struct_1010_dd5e(uVar13, uVar14, i_var9.field_0x6);
    uStack12 = CONCAT22(in_DX, pcVar3);
    in_DX |= pcVar3;
    if (in_DX != 0x0) {
        iStack14 = 0x0;
        pass1_1000_4906(CONCAT22(param_5, &local_a2), 0x0, 0x94);
        pass1_1000_4906(CONCAT22(param_5, local_136), 0x0, 0x94);
        uStack314 = 0x0;
        iStack312 = 0x0;
        iStack316 = 0x0;
        u_var1 = i_var9.field_0x6;
        u_var1 = (u_var1 + 0x26);
        uVar12 = u_var1;
        // TODO: refactor for loop
//     for (uStack326 = 0x1; uStack326 < 0x25; uStack326 += 0x1) {
//       uVar15 = (u_var1 >> 0x10);
//       if ((uStack326 * 0x4 + uStack12) == 0x0) {
//         if (u_var1 != 0x0) {
//           uVar12 = pass1_1020_bae6(u_var1,CONCAT22(uStack326,uVar15),uVar12,
//                                    in_DX,param_5);
//           u_var6 = (uVar12 >> 0x10);
//           uVar12 &= 0xffff;
//           in_DX = u_var6 | uVar12;
//           if (in_DX != 0x0) {
//             if (iStack14 == 0x0) {
//               iStack14 = 0x1;
//             }
//             pcVar3 = string_1020_c0d8(uStack326);
//             u_var4 = in_DX | pcVar3;
//             if (u_var4 == 0x0) {
//               unk_str_op_1000_3d3e((&local_a2)[iStack312],s_Null_Ptr_1050_390e);
//             }
//             else {
//               u_var5 = str_op_1008_60e8(CONCAT22(in_DX,pcVar3),u_var4);
//               (&local_a2 + iStack312) = u_var5;
//               (&local_a2 + iStack312 * 0x4 + 0x2) = u_var4;
//             }
//             local_136[iStack312 * 0x2] = uVar12;
//             local_136[iStack312 * 0x2 + 0x1] = u_var6;
// //             TODO: goto LAB_1010_d11d;
//           }
//         }
//       }
//       else {
//         if (iStack14 == 0x0) {
//           iStack14 = 0x1;
//         }
//         pcVar3 = string_1020_c0d8(uStack326);
//         u_var6 = in_DX | pcVar3;
//         if (u_var6 == 0x0) {
//           unk_str_op_1000_3d3e((&local_a2)[iStack312],s_Null_Ptr_1050_3905);
//         }
//         else {
//           u_var5 = str_op_1008_60e8(CONCAT22(in_DX,pcVar3),u_var6);
//           (&local_a2 + iStack312) = u_var5;
//           (&local_a2 + iStack312 * 0x4 + 0x2) = u_var6;
//         }
//         u_var10 = (uStack12 >> 0x10);
//         u_var4 = (uStack326 * 0x4 + uStack12);
//         u_var6 = (uStack326 * 0x4 + uStack12 + 0x2);
//         local_136[iStack312 * 0x2] = u_var4;
//         local_136[iStack312 * 0x2 + 0x1] = u_var6;
//         if (u_var1 == 0x0) {
//           u_var4 = 0x0;
//         }
//         else {
//           u_var11 = pass1_1020_bae6(u_var1,CONCAT22(uStack326,uVar15),u_var4,u_var6,
//                                    param_5);
//           u_var6 = (u_var11 >> 0x10);
//           u_var4 = u_var11;
//         }
//         uVar12 = u_var4;
//         if (u_var4 == 0x0) {
//           iStack316 += 0x2;
//           in_DX = u_var6;
//           iStack312 = iStack312 + 0x1;
//         }
//         else {
// //LAB_1010_d11d:
//           iStack312 += 0x1;
//           (uVar13 + uStack314 * 0x2 + 0xa4) =
//                (iVar7 + iStack316 * 0x2 + 0x10);
//           (uVar13 + (uStack314 + 0x1) * 0x2 + 0xa4) =
//                (iVar7 + (iStack316 + 0x1) * 0x2 + 0x10);
//           iStack316 += 0x2;
//           uStack314 += 0x2;
//           uVar12 = uStack314;
//           in_DX = u_var6;
//         }
//       }
//     }
        u_var6 = pass1_1010_db2e(uVar13, uVar14, 0x8, CONCAT22(param_5, &local_a2),
                                 CONCAT22(param_5, local_136), param_2, param_3, param_5,
                                 param_4);
        if (iStack14 != 0x0) {
            i_var9.field_0x16 = 0x1;
        }
        while (iStack312 != 0x0) {
            fn_ptr_1000_17ce(ctx, (&local_a2)[iStack312 + -0x1], 0x1000);
            iStack312 = iStack312 + -0x1;
        }
        pass1_1010_dc36(uVar13, uVar14, u_var6, param_2, param_3, param_5);
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1010_d24a(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: U32Ptr, param_5: u8) {
    let u_var1: u32;
    let pu_var2: U32Ptr;
    let mut pcVar3: String;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let in_DX: u16;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let unaff_SI: i16;
    let i_var9: &mut Struct495;
    let uVar9: u16;
    let lVar10: i32;
    let u_var11: u16;
    let uVar12: u16;
    let uStack320: u16;
    let lStack318: i32;
    let local_13a: [U32Ptr;0x4a];
    let local_a6: u32;
    let iStack18: i16;
    let uStack16: u32;
    let mut pcStack12: String;
    let puStack8: U32Ptr;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        // uVar9 = (param_3 >> 0x10);
        i_var9 = param_3;
        pu_var2 = i_var9.field_0x2;
        (i_stack4 * 0xa + pu_var2 + 0x4) = (i_stack4 * 0x2 + param_2);
        i_stack4 += 0x1;
        if (i_stack4 < 0x8) == false { break; }
    }
    puStack8 = i_var9.field_0x2;
    i_stack4 = 0x0;
    loop {
        u_var1 = i_var9.field_0x6;
        pcVar3 = pass1_1010_b038(param_1, u_var1, (u_var1 >> 0x10),
                                 (puStack8 + 0x4), unaff_SI);
        string_1040_a626(puStack8, CONCAT22(in_DX, pcVar3), in_DX);
        i_stack4 += 0x1;
        puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
        if (i_stack4 < 0x8) == false { break; }
    }
    u_var11 = param_1;
    // uVar12 = (param_1 >> 0x10);
    struct_1010_dd5e(u_var11, uVar12, i_var9.field_0x6);
    puVar6 = (in_DX | pcVar3);
    if (puVar6 != 0x0) {
        pcStack12 = pcVar3;
        pass1_1010_e8f6(u_var11, uVar12, i_var9.field_0x6, param_4);
        uStack16 = CONCAT22(puVar6, pcVar3);
        iStack18 = 0x0;
        pass1_1000_4906(CONCAT22(param_4, &local_a6), 0x0, 0x94);
        puVar4 = pass1_1000_4906(CONCAT22(param_4, local_13a), 0x0,
                                 0x94);
        lStack318 = 0x0;
        // TODO: refactor for loop
        // for (uStack320 = 0x1; uStack320 < 0x25; uStack320 += 0x1) {
        //   lVar10 = pass1_1030_7c28(uStack16,uStack320,puVar4,puVar6,
        //                            param_4);
        //   puVar7 = (lVar10 >> 0x10);
        //   puVar4 = lVar10;
        //   puVar6 = (puVar7 | puVar4);
        //   if (lVar10 != 0x0) {
        //     if (iStack18 == 0x0) {
        //       iStack18 = 0x1;
        //     }
        //     pcVar3 = string_1020_c0d8(uStack320);
        //     uVar8 = puVar6 | pcVar3;
        //     if (uVar8 == 0x0) {
        //       unk_str_op_1000_3d3e((&local_a6)[lStack318],s_Null_Ptr_1050_3917);
        //     }
        //     else {
        //       u_var5 = str_op_1008_60e8(CONCAT22(puVar6,pcVar3),uVar8);
        //       (&local_a6 + lStack318) = u_var5;
        //       (&local_a6 + lStack318 * 0x4 + 0x2) = uVar8;
        //     }
        //     local_13a[lStack318 * 0x2] = puVar4;
        //     local_13a[lStack318 * 0x2 + 0x1] = puVar7;
        //     lStack318 += 0x1;
        //     puVar6 = puVar7;
        //   }
        // }
        uVar8 = pass1_1010_db2e(u_var11, uVar12, 0x8, CONCAT22(param_4, &local_a6),
                                CONCAT22(param_4, local_13a), param_2, param_3, param_4,
                                param_5);
        if (iStack18 != 0x0) {
            i_var9.field_0x16 = 0x1;
        }
        while (lStack318 != 0x0) {
            lStack318._0_2_ = (lStack318 + -0x1);
            fn_ptr_1000_17ce(ctx, (&local_a6)[lStack318], 0x1000);
            lStack318 = lStack318 + -0x1;
        }
        pass1_1010_dc36(u_var11, uVar12, uVar8, param_2, param_3, param_4);
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1010_d448(param_1: U32Ptr, param_2: u32, param_3: U32Ptr, param_4: U32Ptr, param_5: u8,
                       param_6: i16)

{
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u32;
    let u_var4: u32;
    let pu_var5: U32Ptr;
    let mut pcVar6: String;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u16;
    let iVar10: i16;
    let u_var11: u16;
    let uVar12: u32;
    let uVar13: u16;
    let local_40c: u16;
    let uStack1034: u32;
    let uStack1030: u32;
    let ulocal_402: [u8;0x400];

    // u_var11 = (param_3 >> 0x10);
    iVar10 = param_3;
    uStack1030 = struct_op_1030_73a8((iVar10 + 0x6));
    // uVar8 = (uStack1030 >> 0x10);
    u_var1 = uStack1030;
    if ((uVar8 | u_var1) != 0x0) {
        uStack1034 = (u_var1 + 0x20);
        u_var1 = (u_var1 + 0x22);
        if ((u_var1 | uStack1034) != 0x0) {
            local_40c = 0x0;
            pu_var5 = &local_40c;
            // uVar13 = (param_1 >> 0x10);
            pass1_1010_d984(param_1, uVar13, CONCAT22(param_4, pu_var5), 0x3,
                            uStack1034 & 0xffff | u_var1 << 0x10,
                            &ctx.PTR_DAT_1050_1805_1050_368e, param_3, param_4, param_5);
            pu_var2 = (iVar10 + 0x2);
            uVar9 = (iVar10 + 0x4);
            (pu_var2 + 0x4) = ctx.PTR_DAT_1050_1805_1050_368e;
            u_var3 = (iVar10 + 0x6);
            pcVar6 = pass1_1010_b038(param_1, u_var3, (u_var3 >> 0x10),
                                     (pu_var2 + 0x4), param_6);
            string_1000_3d3e(CONCAT22(param_4, local_402), CONCAT22(uVar9, pcVar6));
            string_1040_a626(pu_var2, CONCAT22(param_4, local_402), uVar9);
            u_var4 = (iVar10 + 0x2);
            uVar9 = (iVar10 + 0x4);
            iVar7 = u_var4;
            (iVar7 + 0xe) = ctx.PTR_DAT_1050_1822_1050_3690;
            sys_1000_3f9c(local_402, param_4, 0x3920, ctx.data_seg, local_40c,
                          &stack0xfffe, uVar9, 0x1000, param_4, param_5);
            string_1040_a626((u_var4 & 0xffff0000 | (iVar7 + 0xa)),
                             CONCAT22(param_4, local_402), uVar9);
            u_var4 = (iVar10 + 0x2);
            u_var11 = (iVar10 + 0x4);
            iVar10 = u_var4;
            (iVar10 + 0x18) = ctx.PTR_DAT_1050_1823_1050_3692;
            uVar12 = pass1_1028_62c8(uStack1030, param_4);
            // uVar9 = (uVar12 >> 0x10);
            sys_1000_3f9c(local_402, param_4, 0x3923, ctx.data_seg, uVar12,
                          &stack0xfffe, u_var11, 0x1000, param_4, param_5);
            string_1040_a626((u_var4 & 0xffff0000 | (iVar10 + 0x14)),
                             CONCAT22(param_4, local_402), uVar9);
            pass1_1010_dc36(param_1, uVar13, pu_var5, param_2, param_3, param_4);
        }
    }
    return;
}


pub fn pass1_1010_d5ae(param_1: U32Ptr, param_2: u32, param_3: U32Ptr, param_4: U32Ptr, param_5: u8,
                       param_6: i16)

{
    let pu_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u32;
    let puVar4: U32Ptr;
    let pu_var5: U32Ptr;
    let mut pcVar6: String;
    let iVar7: i16;
    let uVar8: u16;
    let i_var9: i16;
    let u_var10: u16;
    let u_var11: u32;
    let uVar12: u16;
    let local_40c: u16;
    let uStack1034: u16;
    let uStack1032: u16;
    let uStack1030: u32;
    let ulocal_402: [u8;0x400];

    // u_var10 = (param_3 >> 0x10);
    i_var9 = param_3;
    uStack1030 = struct_op_1030_73a8((i_var9 + 0x6));
    uStack1034 = uStack1030;
    uStack1032 = (uStack1030 >> 0x10) | uStack1034;
    if (uStack1032 != 0x0) {
        pass1_1028_45fe(uStack1030, uStack1034, param_4);
        if ((uStack1032 | uStack1034) != 0x0) {
            local_40c = 0x0;
            pu_var5 = &local_40c;
            // uVar12 = (param_1 >> 0x10);
            pass1_1010_d984(param_1, uVar12, CONCAT22(param_4, pu_var5), 0x3,
                            CONCAT22(uStack1032, uStack1034), &ctx.PTR_DAT_1050_1805_1050_3706,
                            param_3, param_4, param_5);
            pu_var1 = (i_var9 + 0x2);
            uVar8 = (i_var9 + 0x4);
            (pu_var1 + 0x4) = ctx.PTR_DAT_1050_1805_1050_3706;
            u_var2 = (i_var9 + 0x6);
            pcVar6 = pass1_1010_b038(param_1, u_var2, (u_var2 >> 0x10),
                                     (pu_var1 + 0x4), param_6);
            string_1000_3d3e(CONCAT22(param_4, local_402), CONCAT22(uVar8, pcVar6));
            string_1040_a626(pu_var1, CONCAT22(param_4, local_402), uVar8);
            u_var3 = (i_var9 + 0x2);
            uVar8 = (i_var9 + 0x4);
            iVar7 = u_var3;
            (iVar7 + 0xe) = ctx.PTR_DAT_1050_1822_1050_3708;
            sys_1000_3f9c(local_402, param_4, 0x3926, ctx.data_seg, local_40c,
                          &stack0xfffe, uVar8, 0x1000, param_4, param_5);
            string_1040_a626((u_var3 & 0xffff0000 | (iVar7 + 0xa)),
                             CONCAT22(param_4, local_402), uVar8);
            puVar4 = ctx.PTR_DAT_1050_1823_1050_370a;
            u_var3 = (i_var9 + 0x2);
            i_var9 = (i_var9 + 0x4);
            iVar7 = u_var3;
            (iVar7 + 0x18) = ctx.PTR_DAT_1050_1823_1050_370a;
            u_var11 = pass1_1028_45e2(uStack1030, puVar4, i_var9, param_4);
            // uVar8 = (u_var11 >> 0x10);
            sys_1000_3f9c(local_402, param_4, 0x3929, ctx.data_seg, u_var11,
                          &stack0xfffe, i_var9, 0x1000, param_4, param_5);
            string_1040_a626((u_var3 & 0xffff0000 | (iVar7 + 0x14)),
                             CONCAT22(param_4, local_402), uVar8);
            pass1_1010_dc36(param_1, uVar12, pu_var5, param_2, param_3, param_4);
        }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1010_d710(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: U32Ptr, param_5: u8) {
    let u_var1: u32;
    let lVar2: i32;
    let pu_var3: U32Ptr;
    let mut pcVar4: String;
    let iVar5: i16;
    let u_var6: u16;
    let in_DX: u16;
    let uVar7: u16;
    let unaff_SI: i16;
    let i_var8: i16;
    let i_var9: &mut Struct496;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let uVar12: u32;
    let uVar13: u16;
    let uVar14: u16;
    let uStack322: u16;
    let iStack316: i16;
    let iStack314: i16;
    let iStack312: i16;
    let local_136:[u16;0x4a];
    let local_a2: u32;
    let iStack14: i16;
    let uStack12: u32;
    let puStack8: U32Ptr;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        // uVar9 = (param_2 >> 0x10);
        i_var8 = param_2;
        // u_var10 = (param_3 >> 0x10);
        i_var9 = param_3;
        pu_var3 = i_var9.field_0x2;
        (i_stack4 * 0xa + pu_var3 + 0x4) = (i_stack4 * 0x2 + i_var8);
        i_stack4 += 0x1;
        if (i_stack4 < 0x4) == false { break; }
    }
    puStack8 = i_var9.field_0x2;
    i_stack4 = 0x0;
    loop {
        u_var1 = i_var9.field_0x6;
        pcVar4 = pass1_1010_b038(param_1, u_var1, (u_var1 >> 0x10),
                                 (puStack8 + 0x4), unaff_SI);
        string_1040_a626(puStack8, CONCAT22(in_DX, pcVar4), in_DX);
        i_stack4 += 0x1;
        puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
        if (i_stack4 < 0x4) == false { break; }
    }
    uVar13 = param_1;
    // uVar14 = (param_1 >> 0x10);
    struct_1010_dd5e(uVar13, uVar14, i_var9.field_0x6);
    uStack12 = CONCAT22(in_DX, pcVar4);
    in_DX |= pcVar4;
    if (in_DX != 0x0) {
        iStack14 = 0x0;
        pass1_1000_4906(CONCAT22(param_4, &local_a2), 0x0, 0x94);
        pass1_1000_4906(CONCAT22(param_4, local_136), 0x0, 0x94);
        iStack314 = 0x0;
        iStack312 = 0x0;
        iStack316 = 0x0;
        u_var1 = i_var9.field_0x6;
        lVar2 = (u_var1 + 0x26);
        // TODO: refactor for loop
        // for (uStack322 = 0x1; uStack322 < 0x25; uStack322 += 0x1) {
        //   if ((uStack322 * 0x4 + uStack12) != 0x0) {
        //     if (iStack14 == 0x0) {
        //       iStack14 = 0x1;
        //     }
        //     pcVar4 = string_1020_c0d8(uStack322);
        //     uVar7 = in_DX | pcVar4;
        //     if (uVar7 == 0x0) {
        //       unk_str_op_1000_3d3e((&local_a2)[iStack312],s_Null_Ptr_1050_392c);
        //     }
        //     else {
        //       u_var6 = str_op_1008_60e8(CONCAT22(in_DX,pcVar4),uVar7);
        //       (&local_a2 + iStack312) = u_var6;
        //       (&local_a2 + iStack312 * 0x4 + 0x2) = uVar7;
        //     }
        //     u_var11 = (uStack12 >> 0x10);
        //     uVar7 = (uStack322 * 0x4 + uStack12);
        //     in_DX = (uStack322 * 0x4 + uStack12 + 0x2);
        //     local_136[iStack312 * 0x2] = uVar7;
        //     local_136[iStack312 * 0x2 + 0x1] = in_DX;
        //     iStack312 += 0x1;
        //     if (lVar2 == 0x0) {
        //       iVar5 = 0x0;
        //     }
        //     else {
        //       uVar12 = pass1_1020_bae6(lVar2,
        //                                CONCAT22(uStack322,(lVar2 >> 0x10)),uVar7,
        //                                in_DX,param_4);
        //       in_DX = (uVar12 >> 0x10);
        //       iVar5 = uVar12;
        //     }
        //     if (iVar5 == 0x0) {
        //       iStack316 += 0x2;
        //     }
        //     else {
        //       (uVar13 + iStack314 * 0x2 + 0xa4) =
        //            (i_var8 + iStack316 * 0x2 + 0x8);
        //       (uVar13 + (iStack314 + 0x1) * 0x2 + 0xa4) =
        //            (i_var8 + (iStack316 + 0x1) * 0x2 + 0x8);
        //       iStack316 += 0x2;
        //       iStack314 += 0x2;
        //     }
        //   }
        // }
        uVar7 = pass1_1010_db2e(uVar13, uVar14, 0x4, CONCAT22(param_4, &local_a2),
                                CONCAT22(param_4, local_136), param_2, param_3, param_4,
                                param_5);
        if (iStack14 != 0x0) {
            i_var9.field_0x16 = 0x1;
        }
        while (iStack312 != 0x0) {
            fn_ptr_1000_17ce(ctx, (&local_a2)[iStack312 + -0x1], 0x1000);
            iStack312 = iStack312 + -0x1;
        }
        pass1_1010_dc36(uVar13, uVar14, uVar7, param_2, param_3, param_4);
    }
    return;
}


pub fn pass1_1010_d984(param_1: u16, param_2: u16, param_3: &mut i16, param_4: i16, param_5: u32,
                       param_6: u32, param_7: u32, param_8: U32Ptr, param_9: u8)

{
    let pu_var1: U32Ptr;
    let mut pcVar2: String;
    let i_var3: i16;
    let extraout_dx: u16;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u16;
    let puStack1052: U32Ptr;
    let ulocal_418: [u8;0x400];
    let uStack24: u16;
    let mut pcStack22: String;
    let uStack18: u16;
    let uStack16: u32;
    let local_c: [u8; 8];
    let i_stack4: i16;

    i_stack4 = param_4;
    pass1_1008_5784(CONCAT22(param_8, local_c), param_5);
    loop {
        pu_var1 = local_c;
        pass1_1008_5b12(pu_var1, param_8);
        uStack16 = CONCAT22(extraout_dx, pu_var1);
        u_var4 = extraout_dx | pu_var1;
        if u_var4 == 0x0 {
            return;
        }
        uStack18 = (pu_var1 + 0xa);
        pcStack22 = 0x0;
        if (pu_var1 + 0x4) == 0x0 {
            if (pu_var1 + 0x6) == 0x0 {
                if (pu_var1 + 0x8) == 0x0 {
                    return;
                }
                pcVar2 = string_op_1020_c2f8((pu_var1 + 0x8));
            } else {
                pcVar2 = string_op_1020_c222((pu_var1 + 0x6));
            }
        } else {
            pcVar2 = string_1020_c0d8((pu_var1 + 0x4));
        }
        pcStack22 = CONCAT22(u_var4, pcVar2);
        uStack24 = (uStack16 + 0xc);
        *param_3 = *param_3 + uStack24;
        // uVar8 = (param_7 >> 0x10);
        iVar6 = param_7;
        u_var5 = (iVar6 + 0x4);
        i_var3 = (iVar6 + 0x2) + i_stack4 * 0xa;
        puStack1052 = CONCAT22(u_var5, i_var3);
        // uVar9 = (param_6 >> 0x10);
        iVar7 = param_6;
        (i_var3 + 0x4) = (i_stack4 * 0x2 + iVar7);
        sys_1000_3f9c(local_418, param_8, 0x3935, ctx.data_seg, uStack18, &stack0xfffe, u_var5, 0x1000, param_8, param_9);
        string_1040_a626(puStack1052, CONCAT22(param_8, local_418), u_var5);
        u_var5 = (iVar6 + 0x4);
        i_stack4 += 0x1;
        i_var3 = (iVar6 + 0x2) + i_stack4 * 0xa;
        puStack1052 = CONCAT22(u_var5, i_var3);
        (i_var3 + 0x4) = (i_stack4 * 0x2 + iVar7);
        string_1000_3d3e(CONCAT22(param_8, local_418), pcStack22);
        string_1040_a626(puStack1052, CONCAT22(param_8, local_418), u_var5);
        i_var3 = (i_stack4 + 0x1) * 0xa + (iVar6 + 0x2);
        u_var5 = (iVar6 + 0x4);
        puStack1052 = CONCAT22(u_var5, i_var3);
        (i_var3 + 0x4) = ((i_stack4 + 0x1) * 0x2 + iVar7);
        i_stack4 += 0x2;
        sys_1000_3f9c(local_418, param_8, 0x3938, ctx.data_seg, uStack24, &stack0xfffe, u_var5, 0x1000, param_8, param_9);
        string_1040_a626(puStack1052, CONCAT22(param_8, local_418), u_var5);
    }
}



pub fn pass1_1010_db2e(param_1: u16,param_2: u16,param_3: u16,param_4: u32,param_5: u32,
param_6: u32,param_7: & mut i16,param_8: * mut u8,param_9: u8) -> u16

{
let u_var1: u16; let i_var2: & mut Struct493; let i_var3: i16; let u_var4: u16; let i_var4: & mut Struct492; let u_var5: u16; let u_var6: u16; let uVar7: u16; let uStack94: u16; let iStack92: i16; let uStack90: u16; let puStack88: * mut u16; let ulocal_54:  [u8;0x52];

uStack94 = param_3; uStack90 = param_3; iStack92 = 0x0; loop {
// uVar7 = (param_7 >> 0x10);
i_var4 = param_7; if ( * param_7 - 0x1 < uStack94) {
return uStack94;
}
u_var1 = i_var4.field_0x4; i_var2 = (i_var4.field_0x2 + uStack94 * 0xa);
// u_var5 = (param_5 >> 0x10);
// u_var6 = (param_4 >> 0x10); if (((iStack92 * 0x4 + param_5) == 0x0) & & ((iStack92 * 0x4 + param_4) == 0x0)) { break; }
u_var4 = u_var1; unk_str_op_1000_3d3e
(CONCAT22(param_8, local_54),
(iStack92 * 0x4 + param_4));
// u_var6 = (param_6 >> 0x10);
i_var2.field_0x4 = (uStack90 * 0x2 + param_6); string_1040_a626(CONCAT22(u_var1, i_var2), CONCAT22(param_8, local_54),
u_var4); sys_1000_3f9c(local_54, param_8, 0x393b, ctx.data_seg,
(param_5 + iStack92 * 0x4), & stack0xfffe,
u_var5, 0x1000, param_8, param_9); u_var1 = i_var4.field_0x4; i_var3 = i_var4.field_0x2 + (uStack94 + 0x1) * 0xa; puStack88 = CONCAT22(u_var1, i_var3); (i_var3 + 0x4) = ((uStack90 + 0x1) * 0x2 + param_6); string_1040_a626(puStack88, CONCAT22(param_8, local_54), u_var1); uStack94 += 0x2; uStack90 += 0x2; iStack92 += 0x1;
}
return uStack94;
}



pub fn pass1_1010_dc36(param_1: u16, param_2: u16, param_3: u16, param_4: u32, param_5: U32Ptr,
                       param_6: u16)

{
    let pu_var1: u32;
    let u_var2: u16;
    let u_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let puVar6: u32;
    let uVar7: u16;
    let uStack90: u16;
    let local_54: U32Ptr;
    let local_52: [u32; 0x14];

    local_54 = ctx.PTR_s_New_failed_in_Op_Op_1050_0020_1050_393f;
    puVar6 = local_52;
// TODO: refactor for loop
    // for (i_var4 = 0x13; i_var4 != 0x0; i_var4 += -0x1) {
    //   pu_var1 = puVar6;
    //   puVar6 = puVar6 + 0x1;
    //   *pu_var1 = 0x0;
    // }
    puVar6 = 0x0;
    *(puVar6 + 0x2) = 0x0;
    uStack90 = param_3;
    loop {
        // uVar7 = (param_5 >> 0x10);
        if (*param_5 < uStack90 || *param_5 == uStack90) { break; }
        u_var3 = (param_5 + 0x2);
        u_var2 = (param_5 + 0x4);
        u_var5 = u_var3 + uStack90 * 0xa;
        (u_var5 + 0x4) = (uStack90 * 0x2 + param_4);
        uStack90 += 0x1;
        string_1040_a626((u_var3 & 0xffff0000 | u_var5),
                         CONCAT22(param_6, &local_54), u_var2);
    }
    return;
}


pub fn pass1_1010_de78(param_1: u32, param_2: u32) {
    let in_buf_len_5: i16 = 0;

    in_buf_len_5 = (param_1 >> 0x10);
    *(param_1 + 0x13c) = 0x0;
    pass1_1030_809c(param_2);
    load_string_1010_84e0(0x1030, _PTR_LOOP_1050_14cc,
                          (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x3ff,
                          (param_1 + 0x13c), in_buf_len_5);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_debe(param_1: u32, param_2: u16, param_3: U32Ptr, param_4: U32Ptr, param_5: u32,
                       param_6: u16)

{
    let bVar1: u8;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let iVar7: i16;
    let unaff_DI: i16;
    let uVar8: u16;
    let uVar9: u32;
    let puVar10: U32Ptr;
    let u_var11: u16;
    let iStack34: i16;
    let uStack30: u16;
    let iStack26: i16;
    let iStack24: i16;
    let iStack22: i16;
    let iStack20: i16;

    *param_4 = 0x0;
    *param_3 = 0x0;
    uVar9 = struct_op_1030_73a8(param_5);
    // puVar6 = (uVar9 >> 0x10);
    i_var4 = (uVar9 + 0x12);
    u_var5 = param_1;
    // u_var11 = (param_1 >> 0x10);
    u_var2 = pass1_1010_b028(u_var5, uVar9);
    puVar10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x35, param_6, puVar6, unaff_DI);
    // puVar6 = (puVar10 >> 0x10);
    iVar7 = param_4;
    // uVar8 = (param_4 >> 0x10);
    if (param_2 == 0x13) {
        iStack34 = 0x0;
        while (iStack34 += 0x1, iStack34 < 0x43) {
            param_2 = pass1_1010_ac62(u_var5, u_var11, iStack34, param_2, puVar6);
            if (param_2 != 0x0) {
                *param_3 = *param_3 + 0x1;
            }
        }
        u_var2 = *param_3 * 0x2;
        mem_op_1000_179c(u_var2, puVar6, 0x1000);
        param_4 = u_var2;
        (iVar7 + 0x2) = puVar6;
        if ((puVar6 | param_4) != 0x0) {
            iStack34 = 0x0;
// TODO: refactor for loop
//       for (uStack30 = 0x0; u_var2 = uStack30,
//           *param_3 != uStack30 && uStack30 <= *param_3; uStack30 += 0x1) {
//         loop {
//           iStack34 += 0x1;
//           if (0x42 < iStack34) goto LAB_1010_e0d4;
//           u_var2 = pass1_1010_ac62(u_var5,u_var11,iStack34,u_var2,puVar6);
//         } while (u_var2 == 0x0);
//         (uStack30 * 0x2 + *param_4) = iStack34;
// //LAB_1010_e0d4:
//       }
        }
    } else {
        if (param_2 < 0x14) {
            if (param_2 == '\x06') {
                if (((i_var4 == 0x5) || (i_var4 == 0x6)) || (i_var4 == 0x8)) {
                    u_var3 = puVar10 + 0x11e;
                    if (u_var2 == 0xf) {
                        iStack20 = 0xf;
                        iStack22 = 0x13;
                    } else {
                        if (u_var2 == 0xe) {
                            iStack22 = 0x4;
                            iStack20 = 0x1;
                        } else {
                            iStack22 = 0xe;
                            iStack20 = 0x1;
                        }
                    }
                    i_var4 = pass1_1010_e128(u_var5, u_var11, iStack22, iStack20,
                                             puVar10 & 0xffff0000 | u_var3);
                    *param_3 = i_var4 + 0x1;
                    if (i_var4 + 0x1 != 0x0) {
                        u_var2 = *param_3 * 0x2;
                        mem_op_1000_179c(u_var2, puVar6, 0x1000);
                        param_4 = u_var2;
                        (iVar7 + 0x2) = puVar6;
                        iStack24 = 0x0;
// TODO: refactor for loop
                        // for (iStack26 = iStack20; iStack26 <= iStack22; iStack26 += 0x1) {
                        //   if ((iStack26 * 0x2 + u_var3) != 0x0) {
                        //     (*param_4 + iStack24 * 0x2) = iStack26;
                        //     iStack24 += 0x1;
                        //   }
                        // }
                        (*param_4 + iStack24 * 0x2) = 0x14;
                        return;
                    }
                }
            } else {
                bVar1 = param_2 - 0x7;
                if ((bVar1 == 0x0) && (((i_var4 == 0x5 || (i_var4 == 0x6)) || (i_var4 == 0x8)))) {
                    u_var5 = pass1_1010_ac62(u_var5, u_var11, 0x7, param_2 & 0xff00 | bVar1,
                                             puVar6);
                    u_var2 = -(u_var5 == 0x0) + 0x10;
                    *param_3 = u_var2;
                    u_var2 *= 0x2;
                    mem_op_1000_179c(u_var2, puVar6, 0x1000);
                    param_4 = u_var2;
                    (iVar7 + 0x2) = puVar6;
                    if ((puVar6 | param_4) == 0x0) {
                        *param_3 = 0x0;
                        return;
                    }
// TODO: refactor for loop
                    // for (iStack26 = 0x0; iStack26 < (-(u_var5 == 0x0) + 0xf);
                    //     iStack26 += 0x1) {
                    //   (iStack26 * 0x2 + *param_4) = iStack26 + 0x1;
                    // }
                    (iStack26 * 0x2 + *param_4) = 0x10;
                    return;
                }
            }
        }
    }
    return;
}



pub fn pass1_1010_e128(param_1: u16,param_2: u16,param_3: i16,param_4: i16,param_5: u32) -> i16

{
let i_stack6: i16; let i_stack4: i16;

i_stack4 = 0x0;
// TODO: refactor for loop
// for (i_stack6 = param_4; i_stack6 <= param_3; i_stack6 += 0x1) {
//   if ((i_stack6 * 0x2 + param_5) != 0x0) {
//     i_stack4 += 0x1;
//   }
// } return i_stack4;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_e15e(param_1: u32, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var5: u16;
    let uStack18: u32;
    let uStack14: u32;
    let puStack10: u32;

    pass1_1010_afde(param_1, 0xc);
    puStack10 = CONCAT22(param_3, param_2);
    ppcVar1 = (*puStack10 + 0x10);
    u_var2 = param_2;
    (**ppcVar1)(param_4, param_2, param_3);
    uStack14 = CONCAT22(extraout_dx, u_var2);
    // TODO: refactor for loop
    // for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
    //   ppcVar1 = (*puStack10 + 0x4);
    //   u_var4 = uStack14;
    //   (**ppcVar1)(param_4,param_2,param_3,uStack18,(uStack18 >> 0x10));
    //   u_var3 = u_var4;
    //   u_var5 = extraout_DX_00;
    //   pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var3,extraout_DX_00);
    //   param_4 = 0x1030;
    //   pass1_1030_7c28(CONCAT13((u_var5 >> 0x8),CONCAT12(u_var5,u_var3)),0x23,u_var3,
    //                   u_var5,param_5);
    // }
    if (puStack10 != 0x0) {
        ppcVar1 = *puStack10;
        (**ppcVar1)(param_4, param_2, param_3, 0x1);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_e1f4(param_1: u32, param_2: u32, param_3: u16, param_4: u16) {
    let u_var1: u16;
    let BVar2: bool;
    let mut pcVar3: String;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let in_buf_len_5: i16;
    let uVar7: u32;

    in_buf_len_5 = (param_1 >> 0x10);
    iVar6 = param_1;
    *(iVar6 + 0x13c) = 0x0;
    uVar7 = struct_op_1030_73a8(param_2);
    // u_var5 = (uVar7 >> 0x10);
    u_var1 = (uVar7 + 0xc);
    BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0xc);
    if ((((((((BVar2 == 0x0) && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x14), BVar2 == 0x0)) && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0xa), BVar2 == 0x0)) && ((BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x15), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0xb), BVar2 == 0x0)))) && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x16), BVar2 == 0x0)) && (((BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x17), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x21), BVar2 == 0x0)) && ((BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x1c), BVar2 == 0x0 && (((BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x1d), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x18), BVar2 == 0x0)) && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x19), BVar2 == 0x0)))))))) && ((BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x4), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x3), BVar2 == 0x0)))) && (((BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x1e), BVar2 == 0x0 && (((BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x23), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x1b), BVar2 == 0x0)) && ((BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x1f), BVar2 == 0x0 && (((BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x1), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x2), BVar2 == 0x0)) && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x13), BVar2 == 0x0)))))))) && (((BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x20), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0xe), BVar2 == 0x0)) && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x10), BVar2 == 0x0)))))) {
        pcVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x12);
        if ((pcVar3 == 0x0) && (pcVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x11),
                                pcVar3 == 0x0)) {
            BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x6);
            if (BVar2 == 0x0) {
                BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x5);
                if (BVar2 == 0x0) {
                    pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x40);
//           TODO: goto LAB_1010_e241;
                }
                u_var4 = pass1_1030_7f98(param_2);
                pcVar3 = string_op_1020_c222(u_var4);
            } else {
                u_var4 = pass1_1030_7f5a(param_2);
                pcVar3 = string_op_1020_c2f8(u_var4);
            }
        } else {
            pass1_1010_e58a(param_1, uVar7, u_var5, param_3, param_4);
        }
        string_1000_3d3e((param_1 & 0xffff0000 | (iVar6 + 0x13c)),
                         CONCAT22(u_var5, pcVar3));
    } else {
//LAB_1010_e241:
        load_string_1010_84e0(0x1008, _PTR_LOOP_1050_14cc,
                              (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x3ff, (iVar6 + 0x13c),
                              in_buf_len_5);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_e58a(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: i16, param_5: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let BVar3: bool;
    let puVar4: u32;
    let extraout_dx: u16;
    let u_var5: u16;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let iVar6: i16;
    let in_buf_len_5: i16;
    let uVar7: u16;
    let puVar8: u32;

    in_buf_len_5 = (param_1 >> 0x10);
    iVar6 = param_1;
    *(iVar6 + 0x13c) = 0x0;
    // uVar7 = (param_2 >> 0x10);
    puVar4 = (param_2 + 0x20);
    uVar7 = (param_2 + 0xc);
    BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uVar7, 0x11);
    if (BVar3 == 0x0) {
        BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uVar7, 0x12);
        if (BVar3 == 0x0) {
            return;
        }
        puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x31, param_5, param_3, param_4);
        ppcVar1 = (*puVar8 + 0x14);
        (**ppcVar1)(0x1008, puVar8, (puVar8 >> 0x10), puVar4, puVar4 >> 0xf);
        u_var5 = extraout_DX_01 | puVar4;
        u_var2 = extraout_DX_01;
    } else {
        puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x41, param_5, param_3, param_4);
        ppcVar1 = (*puVar8 + 0x14);
        (**ppcVar1)(0x1008, puVar8, (puVar8 >> 0x10), puVar4, puVar4 >> 0xf);
        u_var5 = extraout_dx | puVar4;
        u_var2 = extraout_dx;
    }
    if (u_var5 == 0x0) {
        load_string_1010_84e0(0x1008, _PTR_LOOP_1050_14cc,
                              (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x3ff, (iVar6 + 0x13c),
                              in_buf_len_5);
    } else {
        ppcVar1 = (*puVar4 + 0x14);
        (**ppcVar1)(0x1008, puVar4, u_var2);
        string_1000_3d3e((param_1 & 0xffff0000 | (iVar6 + 0x13c)),
                         CONCAT22(extraout_DX_00, puVar4));
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_e682(param_1: u32, param_2: u32, param_3: u16, param_4: u8) {
    let u_var1: u16;
    let BVar2: bool;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let in_buf_len_5: U32Ptr;
    let u_var6: u16;
    let uVar7: u32;
    let uVar8: u16;
    let uVar9: u16;
    let local_1e: u16;
    let uStack28: u16;
    let local_1a: u16;
    let uStack24: u16;
    let uStack22: u16;
    let uStack20: u32;
    let uStack16: u32;
    let uStack12: u32;
    let uStack8: u16;
    let uStack6: u32;

    // in_buf_len_5 = (param_1 >> 0x10);
    u_var5 = param_1;
    *(u_var5 + 0x13c) = 0x0;
    uStack6 = struct_op_1030_73a8(param_2);
    // u_var6 = (uStack6 >> 0x10);
    uStack8 = (uStack6 + 0xc);
    u_var4 = u_var6;
    u_var1 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0x1);
    if (((u_var1 == 0x0) && (u_var1 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0x13), u_var1 == 0x0)) && (u_var1 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0x2), u_var1 == 0x0)) {
        BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0xe);
        if (BVar2 != 0x0) {
            uVar7 = (u_var5 + 0x138);
            u_var3 = (uVar7 + 0x24);
            uStack16 = u_var3;
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var3);
            uStack12 = u_var3 & 0xffff | u_var4 << 0x10;
            uStack20 = (u_var3 + 0x1f6);
            // u_var6 = (uStack20 >> 0x10);
            uVar8 = (uStack20 + 0x1a8);
            uVar9 = 0x3947;
            uStack22 = uVar8;
//LAB_1010_e76d:
            sys_1000_3f9c((u_var5 + 0x13c), in_buf_len_5, uVar9, ctx.data_seg,
                          uVar8, &stack0xfffe, u_var6, 0x1000, param_3, param_4);
            return;
        }
        BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0x5);
        if ((BVar2 == 0x0) && (BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0x6), BVar2 == 0x0)) {
            BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0x10);
            if (BVar2 == 0x0) {
                local_1e = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0xc);
                if ((local_1e == 0x0) && (local_1e = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0x14), local_1e == 0x0)
                ) {
                    BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0xa);
                    if (BVar2 == 0x0) {
                        uVar8 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack8, 0x1e);
                        if (uVar8 == 0x0) {
                            load_string_1010_84e0(0x1008, _PTR_LOOP_1050_14cc,
                                                  (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x3ff,
                                                  (u_var5 + 0x13c), in_buf_len_5);
                            return;
                        }
                        pass1_1030_6ddc(param_2);
                        uVar9 = 0x395c;
                        local_1e = uVar8;
//             TODO: goto LAB_1010_e76d;
                    }
                    uVar7 = pass1_1030_7c28(param_2, 0x21, BVar2, u_var4, param_3);
                    // uStack28 = (uVar7 >> 0x10);
                    u_var1 = uVar7;
                    uVar8 = 0x3958;
                    local_1e = u_var1;
                } else {
                    pass1_1010_e8f6(u_var5, in_buf_len_5, param_2, param_3);
                    uStack28 = u_var4;
                    uVar7 = pass1_1030_7c28(CONCAT22(u_var4, local_1e), 0x23, local_1e, u_var4, param_3);
                    // uStack24 = (uVar7 >> 0x10);
                    u_var1 = uVar7;
                    uVar8 = 0x3954;
                    local_1a = u_var1;
                }
            } else {
                uVar7 = pass1_1030_7c28(param_2, 0x1e, BVar2, u_var4, param_3);
                // uStack28 = (uVar7 >> 0x10);
                u_var1 = uVar7;
                uVar8 = 0x3950;
                local_1e = u_var1;
            }
        } else {
            local_1e = 0x0;
            local_1a = 0x0;
            pass1_1010_e8d0(u_var5, in_buf_len_5, CONCAT22(param_3, &local_1a),
                            CONCAT22(param_3, &local_1e), param_2, &local_1e);
            uVar8 = 0x394a;
            u_var1 = local_1e;
        }
    } else {
        pass1_1010_e8f6(u_var5, in_buf_len_5, param_2, param_3);
        uStack12 = CONCAT22(u_var4, u_var1);
        pass1_1030_70f4(CONCAT22(u_var4, u_var1));
        uStack16 = CONCAT22(u_var4, u_var1);
        uVar8 = 0x3943;
    }
    sys_1000_3f9c((u_var5 + 0x13c), in_buf_len_5, uVar8, ctx.data_seg,
                  u_var1, &stack0xfffe, u_var6, 0x1000, param_3, param_4);
    return;
}


pub fn pass1_1010_e8d0(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: U32Ptr,
                       param_5: u32, param_6: u16)

{
    pass1_1030_7064(param_5);
    *param_4 = param_6;
    pass1_1030_70ac(param_5);
    *param_3 = param_6;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_e8f6(param_1: u16, param_2: u16, param_3: u32, param_4: u16) {
    let u_var1: u16;
    let BVar2: bool;
    let u_var3: u16;
    let u_var4: u32;

    u_var4 = struct_op_1030_73a8(param_3);
    u_var1 = (u_var4 + 0xc);
    BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x13);
    if (BVar2 == 0x0) {
        BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x14);
        if (BVar2 == 0x0) {
            return;
        }
        u_var4 = pass1_1028_4faa(u_var4, param_4);
        // u_var3 = (u_var4 >> 0x10);
        u_var1 = u_var4;
    } else {
        u_var4 = pass1_1028_121e(u_var4, param_4);
        // u_var3 = (u_var4 >> 0x10);
        u_var1 = u_var4;
    }
    pass1_1028_b58e(CONCAT22(u_var3, u_var1));
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_e964(param_1: U32Ptr, param_2: u16, param_3: i16) {
    let u_var1: u32;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_2, param_1, param_3);
    // u_var2 = (pu_var3 >> 0x10);
    u_var1 = (pu_var3 + 0x24);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    load_string_1038_4d28(u_var1 & 0xffff | u_var2 << 0x10);
    return;
}


pub fn pass1_1010_e99a(param_1: u32, param_2: u8) -> u32

{
    let unaff_SS: u16;

    param_1 = param_1 & 0xffff0000 | (param_1 - 0xa);
    pass1_1010_a478(param_1, unaff_SS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1010_eb66(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let puVar4: U32Ptr;
    let iVar5: &mut Struct499;
    let u_var5: u16;
    let puStack14: U32Ptr;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = 0x558;
    iVar5.field_0x2 = 0x1018;
    iVar5.field_0xa = 0x568;
    iVar5.field_0xc = 0x1018;
    pu_var1 = iVar5.field_0xe;
    u_var2 = iVar5.field_0x10;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pass1_1018_04f2(param_1);
    fn_ptr_1000_17ce(ctx, iVar5.field_0x2c, 0x1000);
    if (param_1 == 0x0) {
        puVar4 = 0x0;
        u_var5 = 0x0;
    } else {
        puVar4 = &iVar5.field_0xa;
    }
    puStack14 = CONCAT22(u_var5, puVar4);
    *puStack14 = 0x389a;
    puVar4[0x1] = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1010_ebf8(param_1: u32, param_2: u16, param_3: u16, param_4: i16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + param_4 * 0x4 + 0x34) = param_2;
    (param_1 + param_4 * 0x4 + 0x36) = param_3;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_ec18(param_1: u16, param_2: u16, param_3: u32, param_4: i16, param_5: u16) -> u32

{
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_3);
    return CONCAT22((param_4 + 0x12), (param_4 + 0x10));
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_ec40(param_1: u16, param_2: u16, param_3: u32, param_4: i16, param_5: u16) -> u32

{
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_3);
    return CONCAT22((param_4 + 0x12), (param_4 + 0x10));
}


pub fn pass1_1010_ec68(param_1: u32, param_2: u32, param_3: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x28) = param_2;
    pass1_1010_1f62(param_3, param_1 & 0xffff | u_var1 << 0x10, 0x13);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_ec84(param_1: u32, param_2: u16, param_3: u8) {
    let local_10e:[u8;0x10c];

    pass1_1010_1f62(param_2, param_1, 0x14);
    pass1_1030_532e(CONCAT22(param_2, local_10e),
                    (param_1 + 0x20), param_2, param_3);
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, local_10e));
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_ecc6(param_1: u32, param_2: U32Ptr, param_3: i32, param_4: u16, param_5: u16,
                       param_6: u16)

{
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    pass1_1030_627e(param_6, param_4, param_5, _PTR_LOOP_1050_5740, param_2, param_3);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
    u_var1 = (param_4 + 0x2e);
    // u_var3 = (u_var1 >> 0x10);
    i_var2 = u_var1;
    if ((i_var2 + 0x200) == 0x8000001) {
        pass1_1010_ed22(param_1, (i_var2 + 0x4), param_6);
    }
    return;
}


pub fn pass1_1010_ed22(param_1: u32, param_2: u32, param_3: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x24) = param_2;
    pass1_1010_1f62(param_3, param_1 & 0xffff | u_var1 << 0x10, 0x12);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_ed3e(param_1: u32) {
    let u_var1: u32;

    u_var1 = (param_1 + 0x16);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    return;
}
