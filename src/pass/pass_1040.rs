use crate::cleanup::{destroy_win_1040_7b98, destroy_window_1040_b726, unk_destroy_win_op_1010_2fa0};
use crate::defines::{Struct161, Struct18, Struct19, Struct20, Struct57, Struct65, U32Ptr};
use crate::draw::draw_1040::{draw_text_1040_94fc, draw_text_1040_9650, unk_draw_op_1040_b0f8};
use crate::fn_ptr::fn_ptr_1000::fn_ptr_1000_17ce;
use crate::global::AppContext;
use crate::mem_1000::mem_op_1000_179c;
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{pass1_1000_3cea, pass1_1000_54e8, pass1_1000_5586};
use crate::pass::pass_1008::{pass1_1008_4772, pass1_1008_4d84, pass1_1008_5fd8};
use crate::pass::pass_1010::{pass1_1010_038e, pass1_1010_0538, pass1_1010_1ea6, pass1_1010_2ee2, pass1_1010_7b8c, pass1_1010_7d38, pass1_1010_8ef2, pass1_1010_a50c, pass1_1010_a5ca, pass1_1010_ad64, pass1_1010_ae12, pass1_1010_ae92, pass1_1010_debe, pass1_1010_e964};
use crate::pass::pass_1018::pass1_1018_5742;
use crate::pass::pass_1028::pass1_1028_4ab2;
use crate::pass::pass_1038::{pass1_1038_b6e0, pass1_1040_073a};
use crate::string::string_1000::unk_str_op_1000_3d3e;
use crate::struct_ops::struct_1008::struct_1008_4c58;
use crate::struct_ops::struct_1030::struct_op_1030_73a8;
use crate::struct_ops::struct_1040::{mixed_struct_op_1040_8fb8, struct_1040_a598, struct_1040_b082};
use crate::sys_api::{get_sys_metrics_1040_7728, unk_win_msg_op_1008_9510};
use crate::ui::ui_1008::{win_1008_5c5c, win_1008_5c7c};
use crate::ui::ui_1010::ui_op_1010_79aa;
use crate::ui::ui_1040::{mix_win_ui_op_1040_911e, msg_box_op_1040_cce4, msg_box_ui_op_1040_ad66, send_dlg_item_1040_ce76, send_ldg_item_msg_1040_d79c, ui_cleanup_op_1040_782c, unk_win_ui_op_1040_b230, win_ui_dlg_op_1040_a94a, win_ui_get_prop_op_1040_9566, win_ui_op_1040_ae04, post_win_msg_1040_7b3c};
use crate::util::{CONCAT12, CONCAT13, CONCAT22, SUB42, ZEXT24, get_struct_from_addr};
use crate::win_struct::{HWND16, LRESULT, WNDCLASS16};
use crate::winapi::{GetDlgItem16, SendMessage16, SetDlgItemInt16, SetDlgItemText16};

pub fn pass1_1040_0a1a(
    ctx: &mut AppContext,
    param_1: u32,
    extraout_dx: U32Ptr,
    extraout_dx_00: U32Ptr) {
    let u_var1: u16;
    let pu_var2: u32;
    let ppc_var3: u32;
    let u_var4: u32;
    let pu_var5: u32;
    // let extraout_dx: *mut u8;
    let pu_var6: *mut u8;
    // let extraout_dx_00: *mut u8;
    let pu_var7: *mut u8;
    let i_var8: i16;
    let i_var9: i16;
    let u_var11: u16;
    let u_stack10: u32;
    let u_stack6: u16;

    // uVar10 = (param_1 >> 0x10);
    i_var8 = param_1;
    u_var4 = (i_var8 + 0x8e);
    // u_var11 = (u_var4 >> 0x10);
    i_var9 = u_var4;
    pu_var2 = (i_var9 + 0xa);
    u_stack6 = pu_var2;
    pu_var5 = ((i_var9 + 0xc) | u_stack6);
    if pu_var5 == 0x0 {
        return;
    }
    ppc_var3 = (*pu_var2 + 0x14);
    (**ppc_var3)();
    u_stack10 = CONCAT22(extraout_dx, pu_var5);
    pu_var6 = extraout_dx;
    if (i_var8 + 0x70) != 0x0 {
        pu_var5 = (i_var8 + 0x70);
        u_var1 = (i_var8 + 0x72);
        pu_var6 = (u_var1 | pu_var5);
        if pu_var6 != 0x0 {
            ppc_var3 = *pu_var5;
            (**ppc_var3)();
            pu_var6 = extraout_dx_00;
        }
    }
    mem_op_1000_179c(0x14, pu_var6, 0x1000);
    pu_var7 = (pu_var6 | pu_var5);
    if pu_var7 == 0x0 {
        pu_var5 = 0x0;
        pu_var7 = 0x0;
    } else {
        struct_1008_4c58(CONCAT22(pu_var6, pu_var5));
    }
    (i_var8 + 0x70) = pu_var5;
    (i_var8 + 0x72) = pu_var7;
    pass1_1008_4d84((i_var8 + 0x70), u_stack10, pu_var7);
    return;
}


pub fn pass1_1040_0b6a(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8) -> &mut Struct18

{
    pass1_1040_073a(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_0bfc(
    ctx: &mut AppContext,
    struct_2: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    struct_1: &mut Struct19,
    param_7: i16,
    struct_3: &mut WNDCLASS16,
) -> &mut Struct57

{
    let struct_4: &mut Struct57;
    let u_var1: u16;
    let pu_var2: *mut u16;

    get_sys_metrics_1040_7728(struct_2, 0x1, param_2, 0xfcd, param_5);
    struct_4 = struct_2;
    struct_4.field_0x8e = 0x0;
    struct_2.field_0x0 = 0xdb0;
    struct_4.field_0x2 = ctx.PTR_LOOP_1050_1040;
    pu_var2 = mixed_1010_20ba(
        ctx,
        ctx.PTR__LOOP_1050_0ed0,
        0x39,
        struct_3,
        struct_1,
        param_7
    );
    struct_4.field_0x8e = pu_var2;
    struct_4.field_0x90 = (pu_var2 >> 0x10);
    struct_4.field_0x74 = 0x1;
    return struct_2;
}



pub fn pass1_1040_0c54(
    ctx: &mut AppContext,
    struct_1: &mut Struct18,
    param_2: u16
) {
    struct_1.field_0x0 = 0xdb0;
    struct_1.field_0x2 = ctx.PTR_LOOP_1050_1040;
    struct_1.field_0x8e = 0x0;
    ui_cleanup_op_1040_782c(struct_1, param_2);
    return;
}


pub fn pass1_1040_0d80() -> u16

{
    return 0x1;
}


pub fn pass1_1040_0d8a(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8,
    param_3: u16
) -> &mut Struct18

{
    pass1_1040_0c54(ctx, param_1, param_3);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_0e1c(
    ctx: &mut AppContext,
    struct_1: &mut Struct57,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: &mut Struct19,
    param_6: i16,
    param_7: &mut WNDCLASS16
)
{
    let struct_2: &mut Struct57;
    let pu_var2: &mut Struct20;

    get_sys_metrics_1040_7728(struct_1, 0x1, 0x0, 0x1c0, param_4);
    struct_2 = struct_1;
    struct_2.field_0x92 = param_3;
    struct_2.field_0x96 = 0x0;
    struct_2.field_0x98 = param_2;
    struct_1.field_0x0 = (ctx.s_overflow_on_node__d_1050_11ca[0x8..].to_string());
    struct_2.field_0x2 = ctx.PTR_LOOP_1050_1040;
    pu_var2 = mixed_1010_20ba(ctx, ctx.PTR__LOOP_1050_0ed0, 0x3a, param_7, param_5, param_6, 0);
    struct_2.field_0x8e = pu_var2.clone();
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_0e86(
    ctx: &mut AppContext,
    struct_1: &mut Struct18,
    param_2: &mut WNDCLASS16,
    unaff_di: i16
) {
    let u_var1: u16;
    let struct_2: &mut Struct18;
    let pu_var3: &mut Struct19;
    let struct_3: &mut Struct18;
    let u_var5: u16;
    let u_var6: u16;
    let pu_var7: &mut Struct20;

    struct_3 = struct_1;
    struct_1.field_0x0 = (ctx.s_overflow_on_node__d_1050_11ca[0x8..].to_string());
    (struct_3.field_0x2) = ctx.PTR_LOOP_1050_1040;
    struct_2 = get_struct_from_addr(struct_3.field_0x92);
    u_var1 = (struct_3.field_0x94);
    pu_var3 = (u_var1 | struct_2);
    if pu_var3 != 0x0 {
        pass1_1040_a5d0(struct_2 & 0xffff | u_var1 << 0x10);
        fn_ptr_1000_17ce(ctx, struct_2, 0x1000);
    }
    ctx.PTR_LOOP_1050_5b82 = (struct_3 + 0x96);
    if (struct_3 + 0x92) == 0x0 {
        u_var6 = ctx.PTR_LOOP_1050_1038;
        pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, (struct_3 + 0x6));
    } else {
        pu_var7 = mixed_1010_20ba(ctx, ctx.PTR__LOOP_1050_0ed0, 0x32, param_2, pu_var3, unaff_di, 0);
        u_var6 = 0x1010;
        pass1_1010_7b8c(pu_var7, (struct_3.field_0x6), param_2);
    }
    ui_cleanup_op_1040_782c(struct_1, u_var6);
    return;
}


pub unsafe fn pass1_1040_109c(
    ctx: &mut AppContext,
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: &mut Struct19,
    param_6: i16,
    win_handle: HWND16,
    param_8: &mut WNDCLASS16
)

{
    let u_var1: u32;
    let b_var2: bool;
    let i_var3: &mut Struct20;
    let pu_var4: &mut Struct20;
    b_var2 = false;
    if param_4._2_2_ == 0x1c1 {
        (param_1 + 0x96) = 0x2;
        b_var2 = true;
    } else {
        if param_4._2_2_ == 0x1c2 {
            (param_1 + 0x96) = 0x1;
            b_var2 = true;
        } else {
            if param_4._2_2_ != 0x1830 {
                post_win_msg_1040_7b3c(
                    CONCAT22(param_2, param_1),
                    param_3,
                    param_4,
                    param_4._2_2_,
                    win_handle
                );
                return;
            }
            pu_var4 = mixed_1010_20ba(
                ctx,
                ctx.PTR__LOOP_1050_0ed0,
                0x32,
                param_8,
                param_5,
                param_6,
                0
            );
            i_var3 = pu_var4;
            u_var1 = (param_1 + 0x92);
            ui_op_1010_79aa(pu_var4, 0xfb6, (u_var1 + 0x6), param_8);
            if i_var3 == 0x0 {
                u_var1 = (param_1 + 0x92);
                unk_win_op_1010_7300(pu_var4, 0x0, 0xc, (u_var1 + 0x6));
            }
        }
    }
    if b_var2 {
        u_var1 = (param_1 + 0x8e);
        (u_var1 + 0xa) = (param_1 + 0x96);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_1152(
    ctx: &mut AppContext,
    param_1: i16,
    param_2: u16,
    param_3: &mut WNDCLASS16,
    param_4: &mut Struct19,
    param_5: &mut u16,
    param_6: u16
)

{
    let u_var1: u16;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let pu_var5: &mut Struct20;

    if (param_1 + 0x92) != 0x0 {
        u_var2 = (param_1 + 0x8e);
        u_var1 = (u_var2 + 0xa);
        pu_var5 = mixed_1010_20ba(
            ctx,
            ctx.PTR__LOOP_1050_0ed0,
            param_6,
            param_3,
            param_4,
            0,0
        );
        u_var2 = (param_1 + 0x92);
        // u_var4 = (u_var2 >> 0x10);
        i_var3 = u_var2;
        *param_5 = 0x1010;
        pass1_1010_ae92(
            pu_var5,
            u_var1,
            (i_var3 + 0xa),
            (i_var3 + 0x6),
            param_4,
            param_6
        );
    }
    destroy_win_1040_7b98(CONCAT22(param_2, param_1), param_5);
    ctx.PTR_LOOP_1050_5b80 = 0x0;
    return;
}



astruct_18 *
pass1_1040_11ac(param_1: & mut Struct18,param_2: u8,param_3: u16)

{
pass1_1040_0e86(param_1, param_3); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 *
pass1_1040_123e(param_1: & mut Struct57,param_2: u32,param_3: u16,param_4: u16,
param_5: u16,param_6: * mut u8,param_7: i16,param_8: u16)

{
let iVar1: & mut Struct717; let uVar1: u16; let puVar2: * mut u16;

get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfd1, param_5);
// uVar1 = (param_1 >> 0x10);
iVar1 = param_1; & iVar1.field_0x8e = 0x0; param_1 = 0x17b0; iVar1.field_0x2 = & ctx.PTR_LOOP_1050_1040; puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x46, param_8, param_6,param_7); iVar1.field_0x8e = puVar2; iVar1.field_0x90 = (puVar2 > > 0x10);
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_1290(param_1: &mut Struct18) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x17b0;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}


astruct_18 *  pass1_1040_178a(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_1290(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_181c(param_1: &mut Struct57, param_2: u32, param_3: u16, param_4: u16,
                       param_5: u16, param_6: *mut u8, param_7: u16)

{
    let iVar1: &mut Struct433;
    let unaff_DI: i16;
    let uVar1: u16;
    let puVar2: *mut u16;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfbb, param_5);
    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    &iVar1.field_0x8e = 0x0;
    iVar1.field_0x92 = 0x0;
    iVar1.field_0x94 = 0x0;
    param_1 = 0x1c48;
    iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_7, param_6, unaff_DI);
    iVar1.field_0x8e = puVar2;
    iVar1.field_0x90 = (puVar2 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_1876(param_1: &mut Struct18) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x1c48;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}


u32
pass1_1040_1ab0(param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16,
param_6: u16)

{
let BStack6: bool; let uStack4: u16;

BStack6 = 0x0; uStack4 = 0x0; if (param_4._2_2_ == 0x1831) {
(param_1 + 0x92) = 0x1; (param_1 + 0x94) = 0x1; check_dialog_btn_1040_1b8a(param_1,param_2);
}
else {
BStack6 = post_win_msg_1040_7b3c
(CONCAT13((param_2 > > 0x8),
CONCAT12(param_2, param_1)), param_3,
param_4, param_4._2_2_, param_6); uStack4 = param_5;
}
return CONCAT22(uStack4, BStack6);
}


astruct_18 *  pass1_1040_1c22(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_1876(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_1cb4(param_1: &mut Struct57, param_2: u32, param_3: u16, param_4: u16,
                       param_5: u16, param_6: *mut u8, param_7: i16, param_8: u16)

{
    let puVar1: *mut u8;
    let iVar2: &mut Struct718;
    let uVar2: u16;
    let puVar3: *mut u16;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xe8, param_5);
    // uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    &iVar2.field_0x8e = 0x0;
    &iVar2.field_0x92 = 0x0;
    param_1 = 0x1eee;
    iVar2.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar3 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_8, param_6, param_7);
    // puVar1 = (puVar3 >> 0x10);
    iVar2.field_0x8e = puVar3;
    iVar2.field_0x90 = puVar1;
    puVar3 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x37, param_8, puVar1, param_7);
    iVar2.field_0x92 = puVar3;
    iVar2.field_0x94 = (puVar3 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_1d24(param_1: &mut Struct18) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x1eee;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}



u32
pass1_1040_1e80(param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16,
param_6: u16)

{
let BStack6: bool; let uStack4: u16;

BStack6 = 0x0; uStack4 = 0x0; if (param_4._2_2_ == 0xe4) {
pass1_1008_a9ec((param_1 + 0x92));
}
else {
BStack6 = post_win_msg_1040_7b3c
(CONCAT22(param_2, param_1), param_3, param_4,
param_4._2_2_, param_6); uStack4 = param_5;
}
return CONCAT22(uStack4, BStack6);
}



astruct_18 *  pass1_1040_1ec8(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_1d24(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_1f5a(param_1: &mut Struct57, param_2: u16, param_3: i16, param_4: u16) {
    let piVar1: *mut i16;
    let puVar2: *mut u8;
    let iVar3: &mut Struct719;
    let paVar3: &mut Struct43;
    let uVar4: u32;
    let puVar5: *mut u16;
    let iVar6: i16;
    let uVar7: u16;
    let iVar8: i16;
    let iVar9: i16;
    let uVar10: u16;
    let local_16: u32;
    let uStack18: u32;

    iVar6 = param_1;
    // uVar7 = (param_1 >> 0x10);
    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfcf, param_2);
    (iVar6 + 0x8e) = 0x0;
    (iVar6 + 0xa2) = 0x0;
    (iVar6 + 0xa6) = 0x0;
    param_1 = 0x237e;
    (iVar6 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    paVar3 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc, 0x1cc, param_4);
    (iVar6 + 0x8e) = paVar3;
    (iVar6 + 0x90) = (paVar3 >> 0x10);
    uVar4 = pass1_1008_4772(
        (paVar3 & 0xffff0000 | (iVar6 + 0x8e)));
    // puVar2 = (uVar4 >> 0x10);
    puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x48, param_4, puVar2, param_3);
    local_16 = CONCAT22((uVar4 + 0x8) + 0xa, 0xa);
    uStack18 = CONCAT22(0x1d6, (uVar4 + 0x4) + -0xa);
    (iVar6 + 0x92) = local_16;
    (iVar6 + 0x96) = uStack18;
    (iVar6 + 0x9a) = local_16;
    (iVar6 + 0x9e) = uStack18;
    piVar1 = (iVar6 + 0x9c);
    *piVar1 = *piVar1 + 0x14;
    iVar9 = iVar6 + 0xa2;
    iVar8 = iVar6 + 0xa6;
    uVar10 = uVar7;
    puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2b, param_4,
                             (puVar5 >> 0x10), iVar6 + 0xa2);
    pass1_1010_0538(puVar5, CONCAT22(uVar7, iVar8),
                    CONCAT22(uVar10, iVar9), 0x1010, param_4);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_205e(param_1: &mut Struct18) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar4: &mut Struct624;
    let uVar4: u16;

    // uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field_0x0 = 0x237e;
    iVar4.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar1 = iVar4.field_0x8e;
    uVar2 = iVar4.field_0x90;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    fn_ptr_1000_17ce(ctx, iVar4.field_0xa2, 0x1000);
    fn_ptr_1000_17ce(ctx, iVar4.field_0xa6, 0x1000);
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, iVar4.field_0x6);
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}


astruct_18 *  pass1_1040_2358(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_205e(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_23ea(param_1: &mut Struct57, param_2: u32, param_3: u16, param_4: u16,
                       param_5: u16, param_6: u16, param_7: u16)

{
    let uVar1: u32;
    let iVar2: &mut Struct436;
    let unaff_DI: i16;
    let uVar2: u16;
    let puVar3: *mut u16;

    get_sys_metrics_1040_7728(param_1, 0x9a, param_2, 0xfbd, param_5);
    // uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    &iVar2.field_0x8e = 0x0;
    iVar2.field_0x92 = 0x0;
    iVar2.field_0x94 = 0x0;
    param_1 = 0x2956;
    iVar2.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    iVar2.field_0x8a = 0x26;
    puVar3 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x6, param_6, param_7, unaff_DI);
    iVar2.field_0x8e = puVar3;
    iVar2.field_0x90 = (puVar3 >> 0x10);
    uVar1 = &iVar2.field_0x8e;
    iVar2.field_0x92 = (uVar1 + 0x28);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_2464(param_1: &mut Struct18) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x2956;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}


pub fn pass1_1040_288e(param_1: u32) {
    let uVar1: u16;
    let ppcVar2: u32;
    let uVar3: u32;
    let puVar4: u32;
    let puVar5: u32;
    let extraout_DX: *mut u8;
    let puVar6: *mut u8;
    let extraout_DX_00: *mut u8;
    let puVar7: *mut u8;
    let iVar8: i16;
    let uVar9: u16;

    // uVar9 = (param_1 >> 0x10);
    iVar8 = param_1;
    uVar3 = (iVar8 + 0x8e);
    puVar5 = (uVar3 + 0x24);
    ppcVar2 = (*puVar5 + 0x14);
    (**ppcVar2)();
    puVar4 = puVar5;
    puVar6 = extraout_DX;
    if ((iVar8 + 0x70) != 0x0) {
        puVar4 = (iVar8 + 0x70);
        uVar1 = (iVar8 + 0x72);
        puVar6 = (uVar1 | puVar4);
        if (puVar6 != 0x0) {
            ppcVar2 = *puVar4;
            (**ppcVar2)();
            puVar6 = extraout_DX_00;
        }
    }
    mem_op_1000_179c(0x14, puVar6, 0x1000);
    puVar7 = (puVar6 | puVar4);
    if (puVar7 == 0x0) {
        puVar4 = 0x0;
        puVar7 = 0x0;
    } else {
        struct_1008_4c58(CONCAT22(puVar6, puVar4));
    }
    (iVar8 + 0x70) = puVar4;
    (iVar8 + 0x72) = puVar7;
    pass1_1008_4d84((iVar8 + 0x70),
                    puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10, puVar7);
    return;
}



astruct_18 *  pass1_1040_2930(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_2464(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 *
pas1_1040_29c2(param_1: & mut Struct57,param_2: u32,param_3: u16,param_4: u16,
param_5: u16)

{
let iVar1: i16; let uVar2: u16;

pass1_1040_b0bc(param_1, param_2, CONCAT22(param_3,0x157));
// uVar2 = (param_1 >> 0x10);
iVar1 = param_1; param_1 = 0x2e26; (iVar1 + 0x2) = & ctx.PTR_LOOP_1050_1040; load_string_1010_84ac
(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc > > 0x10), 0x1010); (iVar1 + 0x94) = param_4; (iVar1 + 0x96) = param_5; load_string_1010_84ac
(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc > > 0x10), 0x1010); (iVar1 + 0x98) = param_4; (iVar1 + 0x9a) = param_5; return param_1;
}



pub fn pass1_1040_2a22(param_1: &mut Struct18) {
    let iVar1: &mut Struct625;
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field_0x0 = 0x2e26;
    iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    fn_ptr_1000_17ce(ctx, iVar1.field_0x94, 0x1000);
    fn_ptr_1000_17ce(ctx, iVar1.field_0x98, 0x1000);
    unk_draw_op_1040_b0f8(param_1);
    return;
}


pub fn pass1_1040_2dac(param_1: u32) {
    let uVar1: u32;
    let uVar2: u32;
    let uStack10: i16;

    uVar1 = (param_1 + 0x90);
    uVar2 = struct_op_1030_73a8((uVar1 + 0x6));
    // for (iStack10 = 0x0; iStack10 < 0x5; iStack10 += 0x1) {
    //   pass1_1028_4ab2(uVar2,(&ctx.PTR_LOOP_1050_5d04 + iStack10 * 0xc),
    //                   (iStack10 * 0xc + 0x5d02));
    // }
    return;
}



astruct_18 *  pass1_1040_2e00(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_2a22(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_2ea2(param_1: &mut Struct57, param_2: u32, param_3: u16, param_4: u16,
                       param_5: u16, param_6: *mut u8, param_7: i16, param_8: u16)

{
    let iVar1: &mut Struct720;
    let uVar1: u16;
    let puVar2: *mut u16;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x180, param_5);
    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field_0x8e = 0x0;
    iVar1.field_0x90 = 0x0;
    iVar1.field_0x92 = 0x0;
    iVar1.field_0x94 = 0x0;
    &iVar1.field_0x96 = 0x0;
    param_1 = 0x3436;
    iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3c, param_8, param_6, param_7);
    iVar1.field_0x96 = puVar2;
    iVar1.field_0x98 = (puVar2 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_2f06(param_1: &mut Struct18) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x3436;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_2f32(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16,
)

{
    let puVar1: *mut u16;
    let iVar2: i16;

    iVar2 = 0x0;
    puVar1 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    pass1_1010_038e(puVar1, iVar2, param_5);
    destroy_win_1040_7b98(CONCAT22(param_2, param_1), 0x1010);
    return;
}


astruct_18 *  pass1_1040_3410(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_2f06(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_34a2(param_1: &mut Struct57, param_2: u32, param_3: u16, param_4: u16,
                       param_5: u16, param_6: *mut u8, param_7: i16, param_8: u16)

{
    let iVar1: &mut Struct721;
    let uVar1: u16;
    let puVar2: *mut u16;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x192, param_5);
    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    &iVar1.field_0x8e = 0x0;
    iVar1.field_0x92 = 0x0;
    iVar1.field_0x94 = 0x0;
    iVar1.field_0x96 = 0x0;
    iVar1.field_0x98 = 0x0;
    param_1 = (s_Null_Ptr_1050_38f3 + 0x7);
    iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3c, param_8, param_6, param_7);
    iVar1.field_0x8e = puVar2;
    iVar1.field_0x90 = (puVar2 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_3506(param_1: &mut Struct18) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = (s_Null_Ptr_1050_38f3 + 0x7);
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_3532(param_1: u16, param_2: u16, param_3: *mut u8, param_4: i16, param_5: u16) {
    let puVar1: *mut u16;
    let iVar2: i16;

    iVar2 = 0x0;
    puVar1 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    pass1_1010_038e(puVar1, iVar2, param_5);
    destroy_win_1040_7b98(CONCAT22(param_2, param_1), 0x1010);
    return;
}


astruct_18 *  pass1_1040_38d4(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_3506(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_3966(param_1: &mut Struct57, param_2: u32, param_3: u16, param_4: u16,
                       param_5: u16, param_6: *mut u8, param_7: i16, param_8: u16)

{
    let iVar1: &mut Struct722;
    let uVar1: u16;
    let puVar2: *mut u16;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x185, param_5);
    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    &iVar1.field_0x8e = 0x0;
    iVar1.field_0x92 = 0x0;
    iVar1.field_0x96 = 0x0;
    iVar1.field_0x9a = 0x0;
    iVar1.field_0x9c = 0x0;
    iVar1.field_0x9e = 0x0;
    iVar1.field_0xa0 = 0x0;
    iVar1.field_0xa2 = 0x0;
    iVar1.field_0xa4 = 0x5;
    param_1 = 0x3ffc;
    iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3c, param_8, param_6, param_7);
    iVar1.field_0x8e = puVar2;
    iVar1.field_0x90 = (puVar2 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_39e2(param_1: &mut Struct18) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x3ffc;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_3a0e(param_1: u16, param_2: u16, param_3: *mut u8, param_4: i16, param_5: u16) {
    let puVar1: *mut u16;
    let iVar2: i16;

    iVar2 = 0x0;
    puVar1 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    pass1_1010_038e(puVar1, iVar2, param_5);
    destroy_win_1040_7b98(CONCAT22(param_2, param_1), 0x1010);
    return;
}



astruct_18 *  pass1_1040_3fd6(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_39e2(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_4068(param_1: &mut Struct57, param_2: u32, param_3: u16, param_4: u16,
                       param_5: u16, param_6: *mut u8, param_7: i16, param_8: u16)

{
    let puVar1: *mut u8;
    let iVar2: &mut Struct723;
    let uVar2: u16;
    let puVar3: *mut u16;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfb7, param_5);
    // uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    &iVar2.field_0x8e = 0x0;
    iVar2.field_0x92 = 0x0;
    iVar2.field_0x9a = 0x0;
    param_1 = 0x4466;
    iVar2.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    iVar2.field_0x76 = 0x1;
    puVar3 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_8, param_6, param_7);
    // puVar1 = (puVar3 >> 0x10);
    iVar2.field_0x8e = puVar3;
    iVar2.field_0x90 = puVar1;
    puVar3 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x29, param_8, puVar1, param_7);
    iVar2.field_0x96 = puVar3;
    iVar2.field_0x98 = (puVar3 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_40e2(param_1: &mut Struct18) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x4466;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}


astruct_18 *  pass1_1040_4440(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_40e2(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1040_44d2(param_1: &mut Struct57, param_2: u32, param_3: u16, param_4: u16,
                       param_5: *mut u8)

{
    let uVar1: u32;
    let uVar2: u16;
    let puVar3: *mut u8;
    let iVar4: i16;
    let uVar5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let piStack8: *mut i16;

    iVar6 = param_1;
    // uVar7 = (param_1 >> 0x10);
    struct_1040_b082(param_1, CONCAT22(param_3, 0xfa2));
    param_1 = 0x4824;
    (iVar6 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    mem_op_1000_179c(0x18, param_5, 0x1000);
    puVar3 = (param_5 | param_4);
    if (puVar3 == 0x0) {
        (iVar6 + 0x90) = 0x0;
    } else {
        struct_1040_a598(CONCAT22(param_5, param_4));
        (iVar6 + 0x90) = param_4;
        (iVar6 + 0x92) = puVar3;
    }
    (iVar6 + 0x90) = 0x14;
    iVar4 = *(iVar6 + 0x90);
    uVar2 = iVar4 * 0xa + 0x2;
    mem_op_1000_179c(uVar2, puVar3, 0x1000);
    piStack8 = CONCAT22(puVar3, uVar2);
    if ((puVar3 | uVar2) == 0x0) {
        uVar1 = (iVar6 + 0x90);
        (uVar1 + 0x2) = 0x0;
    } else {
        *piStack8 = iVar4;
        pass1_1000_5586(0xa564, &ctx.PTR_LOOP_1050_1040, iVar4, 0xa, uVar2 + 0x2,
                        puVar3);
        uVar1 = (iVar6 + 0x90);
        // uVar5 = (uVar1 >> 0x10);
        iVar4 = uVar1;
        (iVar4 + 0x2) = uVar2 + 0x2;
        (iVar4 + 0x4) = puVar3;
    }
    uVar1 = (iVar6 + 0x90);
    (uVar1 + 0x6) = param_2;
    uVar1 = (iVar6 + 0x90);
    (uVar1 + 0xa) = 0x1;
    uVar1 = (iVar6 + 0x90);
    (uVar1 + 0x12) = (iVar6 + 0xa);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_45e8(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: *mut u8,
                       param_6: u16, param_7: u16)

{
    let paVar1: &mut Struct18;
    let ppcVar2: u32;
    let uVar3: u32;
    let uVar4: u16;
    let paVar5: &mut Struct18;
    let puVar6: *mut u8;
    let puVar7: *mut u8;
    let iVar8: i16;
    let unaff_DI: i16;
    let uVar9: u16;
    let paVar10: &mut Struct20;
    let piStack16: *mut i16;

    if (param_4._2_2_ != 0xeb) {
        pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, param_6, param_7);
        return;
    }
    paVar10 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_7, param_5, unaff_DI);
    // puVar6 = (paVar10 >> 0x10);
    paVar1 = (param_1 + 0x90);
    if (paVar1 != 0x0) {
        paVar5 = paVar1;
        mem_op_1000_179c(0x18, puVar6, 0x1000);
        uVar4 = paVar5;
        puVar7 = (puVar6 | uVar4);
        if (puVar7 == 0x0) {
            uVar4 = 0x0;
            puVar7 = 0x0;
        } else {
            struct_1040_a598((paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
        }
        (param_1 + 0x90) = uVar4;
        (param_1 + 0x92) = puVar7;
        (param_1 + 0x90) = 0x14;
        iVar8 = *(param_1 + 0x90);
        uVar4 = iVar8 * 0xa + 0x2;
        mem_op_1000_179c(uVar4, puVar7, 0x1000);
        piStack16 = CONCAT22(puVar7, uVar4);
        if ((puVar7 | uVar4) == 0x0) {
            uVar3 = (param_1 + 0x90);
            (uVar3 + 0x2) = 0x0;
        } else {
            *piStack16 = iVar8;
            pass1_1000_5586(0xa564, &ctx.PTR_LOOP_1050_1040, iVar8, 0xa, uVar4 + 0x2,
                            puVar7);
            uVar3 = (param_1 + 0x90);
            // uVar9 = (uVar3 >> 0x10);
            iVar8 = uVar3;
            (iVar8 + 0x2) = uVar4 + 0x2;
            (iVar8 + 0x4) = puVar7;
        }
        uVar3 = (param_1 + 0x90);
        (uVar3 + 0x6) = (paVar1 + 0x6);
        uVar3 = (param_1 + 0x90);
        (uVar3 + 0xa) = 0x1;
        uVar3 = (param_1 + 0x90);
        (uVar3 + 0x12) = (param_1 + 0xa);
        pass1_1010_a50c(paVar10, 0x10505d40, (param_1 + 0x90));
        if (paVar1 != 0x0) {
            pass1_1040_a5d0(paVar1);
            fn_ptr_1000_17ce(ctx, paVar1, 0x1000);
        }
        ppcVar2 = (CONCAT22(param_2, param_1) + 0x70);
        (**ppcVar2)();
    }
    return;
}


pub fn pass1_1040_4766(param_1: *mut u32) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x74);
    (**ppcVar1)();
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_477e(param_1: &mut Struct1, param_2: *mut u8, param_3: u16, param_4: u16) {
    let puVar1: *mut u8;
    let pUVar2: *mut u16;
    let puVar3: *mut u8;
    let puVar4: *mut u8;
    let unaff_DI: i16;
    let puVar5: *mut u16;
    let uVar6: u16;
    let uVar7: u16;

    unk_win_ui_op_1040_b230(param_1, param_3, param_4);
    puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_4, param_2, unaff_DI);
    // puVar3 = (puVar5 >> 0x10);
    uVar7 = SUB42(ctx.data_seg, 0x0);
    uVar6 = 0x5d68;
    puVar1 = pass1_1008_5fd8(param_4, puVar3);
    puVar4 = puVar3;
    pUVar2 = pass1_1000_3cea(CONCAT22(puVar3, puVar1), CONCAT22(uVar7, uVar6));
    pass1_1010_e964(puVar4, param_4, unaff_DI);
    pass1_1000_3cea(CONCAT22(puVar3, puVar1), CONCAT22(puVar4, pUVar2));
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x10)),
                         CONCAT22(puVar3, puVar1));
    fn_ptr_1000_17ce(ctx, CONCAT22(puVar3, puVar1), 0x1000);
    return;
}



astruct_18 *  pass1_1040_47fe(param_1: & mut Struct18,param_2: u8)

{
unk_draw_op_1040_b0f8(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_48a0(param_1: &mut Struct57, param_2: u16, param_3: u32, param_4: u16,
                       param_5: *mut u8, param_6: u16)

{
    let iVar1: i16;
    let piVar2: *mut i16;
    let uVar3: u16;
    let puVar4: *mut u8;
    let puVar5: *mut u8;
    let iVar5: &mut Struct444;
    let iVar6: &mut Struct445;
    let unaff_DI: i16;
    let uVar6: u16;
    let uVar7: u16;
    let puVar8: *mut u16;
    let piStack8: *mut i16;

    struct_1040_b082(param_1, CONCAT22(param_4, 0xfa1));
    // uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    iVar5.field_0x94 = 0x0;
    param_1 = &ctx.PTR_LOOP_1050_4e18;
    iVar5.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_6, param_5, unaff_DI);
    // puVar4 = (puVar8 >> 0x10);
    uVar3 = puVar8;
    &iVar5.field_0x94 = uVar3;
    (&iVar5.field_0x94 + 0x2) = puVar4;
    mem_op_1000_179c(0x18, puVar4, 0x1000);
    puVar5 = (puVar4 | uVar3);
    if (puVar5 == 0x0) {
        iVar5.field_0x90 = 0x0;
    } else {
        struct_1040_a598(CONCAT22(puVar4, uVar3));
        &iVar5.field_0x90 = uVar3;
        (&iVar5.field_0x90 + 0x2) = puVar5;
    }
    *iVar5.field_0x90 = 0x7;
    iVar1 = *iVar5.field_0x90;
    uVar3 = iVar1 * 0xa + 0x2;
    mem_op_1000_179c(uVar3, puVar5, 0x1000);
    piStack8 = CONCAT22(puVar5, uVar3);
    if ((puVar5 | uVar3) == 0x0) {
        piVar2 = iVar5.field_0x90;
        (piVar2 + 0x2) = 0x0;
    } else {
        *piStack8 = iVar1;
        pass1_1000_5586(0xa564, &ctx.PTR_LOOP_1050_1040, iVar1, 0xa, uVar3 + 0x2,
                        puVar5);
        piVar2 = iVar5.field_0x90;
        // uVar7 = (piVar2 >> 0x10);
        iVar6 = piVar2;
        iVar6.field_0x2 = uVar3 + 0x2;
        iVar6.field_0x4 = puVar5;
    }
    piVar2 = iVar5.field_0x90;
    (piVar2 + 0x6) = param_3;
    piVar2 = iVar5.field_0x90;
    (piVar2 + 0xa) = param_2;
    piVar2 = iVar5.field_0x90;
    (piVar2 + 0x12) = iVar5.field_0xa;
    iVar1 = &iVar5.field_0x90;
    uVar7 = (&iVar5.field_0x90 + 0x2);
    pass1_1010_debe(iVar5.field_0x94, (iVar1 + 0xa),
                    CONCAT22(uVar7, iVar1 + 0x10),
                    CONCAT22(uVar7, iVar1 + 0xc), param_3, param_6);
    return;
}


pub fn pass1_1040_4cf4(param_1: u32, param_2: HWND16, param_3: u16) {
    let uVar1: u32;
    let uVar2: u32;
    let uVar3: u16;
    let iVar4: i16;
    let iVar5: i16;
    let unaff_DI: i16;
    let uVar6: u16;
    let uVar7: u16;
    let LVar8: LRESULT;
    let local_52: [u8; 50];

    // uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    GetDlgItem16(param_2, 0x1770);
    LVar8 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4070000);
    // uVar3 = (LVar8 >> 0x10);
    if (LVar8 != -0x1) {
        LVar8 = SendMessage16(ctx.s_tile2_bmp_1050_1538, local_52, param_3,
                              CONCAT22(0x408, LVar8));
        // uVar3 = (LVar8 >> 0x10);
    }
    uVar2 = (iVar4 + 0x90);
    uVar1 = (iVar4 + 0x94);
    uVar3 = pass1_1010_ae12(uVar1, (uVar1 >> 0x10),
                            CONCAT22(param_3, local_52), (uVar2 + 0xa), uVar3);
    if (uVar3 != 0xffff) {
        uVar1 = (iVar4 + 0x90);
        // uVar7 = (uVar1 >> 0x10);
        iVar5 = uVar1;
        pass1_1010_ae92((iVar4 + 0x94), uVar3, (iVar5 + 0xa),
                        (iVar5 + 0x6), unaff_DI, param_3);
    }
    return;
}


pub fn pass1_1040_4d7e(param_1: u32) {
    let uVar1: u32;
    let piVar2: *mut i16;
    let uVar3: u16;
    let iStack8: i16;
    let puStack6: u32;

    // uVar3 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x90);
    puStack6 = (uVar1 + 0x2);
    iStack8 = 0x0;
    while ((piVar2 = (param_1 + 0x90),
            *piVar2 != iStack8 && iStack8 <= *piVar2 && ((puStack6 + 0x4) != 0x1770))) {
        iStack8 += 0x1;
        puStack6 = (puStack6 & 0xffff0000 | (puStack6 + 0xa));
    }
    pass1_1000_3e2c(*puStack6);
    return;
}


pub fn pass1_1040_4dcc(param_1: u32, param_2: i16, param_3: u16) -> *mut u8

{
    let uVar1: u32;
    let uVar2: u32;
    let uVar3: u16;
    let mut pcVar4: String;

    // uVar3 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0x90);
    uVar1 = (param_1 + 0x94);
    pcVar4 = string_op_1010_ada6(0x1010, param_3, uVar1, (uVar1 >> 0x10), param_2, (uVar2 + 0xa));
    return pcVar4;
}



astruct_18 *  pass1_1040_4df2(param_1: & mut Struct18,param_2: u8)

{
unk_draw_op_1040_b0f8(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1040_4e94(param_1: &mut Struct57, param_2: i32, param_3: u16) {
    let iVar1: i16;
    let uVar2: u16;
    let uVar3: u16;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_3, 0xfab));
    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = 0x0;
    (iVar1 + 0x98) = 0x0;
    (iVar1 + 0xb0) = 0x0;
    (iVar1 + 0xb4) = 0x0;
    (iVar1 + 0xb6) = 0x0;
    param_1 = 0x55a2;
    (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    if (param_2 != 0x0) {
        // uVar3 = (param_2 >> 0x10);
        (iVar1 + 0xb0) = (param_2 + 0x6);
        (iVar1 + 0xb4) = (param_2 + 0x14);
    }
    return;
}


pub fn pass1_1040_4f0a(param_1: &mut Struct18) {
    param_1.field_0x0 = 0x55a2;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    unk_draw_op_1040_b0f8(param_1);
    return;
}



u16
pass1_1040_4f28(param_1: * mut u32,param_2: & mut i16,param_3: u16,param_4: u16,param_5: i16,
param_6: u16)

{
let ppcVar1: u32; let uVar2: u16;

if (param_5 == 0x2b) {
if ( * param_2 == 0x4) {
win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), param_6);
}
}
else {
if (param_5 != 0x111) {
uVar2 = pass1_1040_b316(param_1, param_2, param_3,param_4, param_5); return uVar2;
}
ppcVar1 = ( * param_1 + 0x7c); (* * ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
}
return 0x1;
}



pub fn pass1_1040_4f82(param_1: *mut u32) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x80);
    (**ppcVar1)();
    return;
}


pub fn pass1_1040_5238(param_1: u32) -> u16

{
    let ppcVar1: u32;

    ppcVar1 = ((param_1 + 0x94) + 0x8);
    (**ppcVar1)();
    return 0x0;
}


astruct_18 *  pass1_1040_557c(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_4f0a(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1040_5626(param_1: &mut Struct57, param_2: u32, param_3: u16, param_4: *mut u8) {
    let piVar1: *mut i16;
    let uVar2: u16;
    let puVar3: *mut u8;
    let iVar4: i16;
    let uVar5: u16;
    let uVar6: u32;
    let uVar7: u16;
    let piStack12: *mut i16;
    let iVar8: &mut Struct441;
    let iVar7: &mut Struct396;
    let iVar6: &mut Struct439;

    iVar8 = param_1;
    // uVar7 = (param_1 >> 0x10);
    struct_1040_b082(param_1, CONCAT22(param_3, 0xfa3));
    uVar2 = 0x0;
    iVar8.field_0x94 = 0x0;
    iVar8.field_0x96 = 0x0;
    iVar8.field_0x98 = 0x0;
    iVar8.field_0x9c = 0x0;
    param_1 = 0x6386;
    iVar8.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    mem_op_1000_179c(0x18, param_4, 0x1000);
    puVar3 = (param_4 | uVar2);
    if (puVar3 == 0x0) {
        iVar8.field_0x90 = 0x0;
    } else {
        struct_1040_a598(CONCAT22(param_4, uVar2));
        &iVar8.field_0x90 = uVar2;
        (&iVar8.field_0x90 + 0x2) = puVar3;
    }
    *iVar8.field_0x90 = 0x6;
    iVar4 = *iVar8.field_0x90;
    uVar2 = iVar4 * 0xa + 0x2;
    mem_op_1000_179c(uVar2, puVar3, 0x1000);
    piStack12 = CONCAT22(puVar3, uVar2);
    if ((puVar3 | uVar2) == 0x0) {
        piVar1 = iVar8.field_0x90;
        (piVar1 + 0x2) = 0x0;
    } else {
        *piStack12 = iVar4;
        pass1_1000_5586(0xa564, &ctx.PTR_LOOP_1050_1040, iVar4, 0xa, uVar2 + 0x2,
                        puVar3);
        piVar1 = iVar8.field_0x90;
        // uVar5 = (piVar1 >> 0x10);
        iVar4 = piVar1;
        (iVar4 + 0x2) = uVar2 + 0x2;
        (iVar4 + 0x4) = puVar3;
    }
    piVar1 = iVar8.field_0x90;
    (piVar1 + 0x6) = param_2;
    piVar1 = iVar8.field_0x90;
    (piVar1 + 0xa) = 0x4;
    piVar1 = iVar8.field_0x90;
    (piVar1 + 0x12) = iVar8.field_0xa;
    uVar6 = pass1_1040_5d12(param_1);
    // uVar2 = (uVar6 >> 0x10);
    if ((uVar2 | uVar6) == 0x0) {
        iVar8.field_0x9a = 0x0;
    } else {
        iVar8.field_0x9a = (uVar6 + 0x20);
    }
    return;
}


pub fn pass1_1040_57d4(param_1: &mut Struct1, param_2: *mut u8, param_3: i16, param_4: u16,
                       param_5: u16)

{
    pass1_1040_5d42(param_1);
    pass1_1040_5eaa(param_1);
    pass1_1040_5dc4(param_1, param_2, param_3, param_5);
    unk_win_ui_op_1040_b230(param_1, param_4, param_5);
    return;
}


pub fn pass1_1040_5cd6(param_1: u32) -> u16

{
    let iVar1: i16;
    let uVar2: u16;
    let uVar3: u32;

    uVar3 = pass1_1040_5d12(param_1);
    if (uVar3 != 0x0) {
        iVar1 = (uVar3 + 0x20);
        // uVar2 = (param_1 >> 0x10);
        if ((param_1 + 0x9a) != iVar1) {
            (param_1 + 0x9a) = iVar1;
            return 0x1;
        }
    }
    return 0x0;
}


pub fn pass1_1040_5d12(param_1: u32) -> u32

{
    let uVar1: u16;
    let uVar2: u16;
    let uVar3: u32;
    let iVar4: &mut Struct440;
    let uVar4: u16;
    let uVar5: u32;

    uVar3 = (param_1 + 0x90);
    // uVar4 = (uVar3 >> 0x10);
    iVar4 = uVar3;
    uVar1 = iVar4.field_0x6;
    uVar2 = iVar4.field_0x8;
    if ((uVar2 | uVar1) != 0x0) {
        uVar5 = struct_op_1030_73a8(CONCAT22(uVar2, uVar1));
        return uVar5;
    }
    return 0x0;
}


pub fn pass1_1040_5d42(param_1: u32) {
    let uVar1: u16;
    let cVar2: u8;
    let iVar3: i16;
    let uVar4: u16;
    let uVar5: u32;

    uVar5 = pass1_1040_5d12(param_1);
    if (uVar5 != 0x0) {
        uVar1 = (uVar5 + 0xc);
        iVar3 = param_1;
        // uVar4 = (param_1 >> 0x10);
        if (uVar1 == 0x5f) {
            (iVar3 + 0x96) = 0x53;
            return;
        }
        if (uVar1 < 0x60) {
            cVar2 = uVar1;
            if (cVar2 == '(') {
                (iVar3 + 0x96) = 0x54;
                return;
            }
            if (cVar2 == ')') {
                (iVar3 + 0x96) = 0x55;
                return;
            }
            if (cVar2 == ']') {
                (iVar3 + 0x96) = 0x51;
                return;
            }
            if (cVar2 == '^') {
                (iVar3 + 0x96) = 0x52;
                return;
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_5dc4(param_1: u32, param_2: *mut u8, param_3: i16, param_4: u16) {
    let ppcVar1: u32;
    let uVar2: u32;
    let uVar3: u16;
    let uVar4: u16;
    let uVar5: u16;
    let puVar6: *mut u8;
    let extraout_DX: u16;
    let iVar7: &mut Struct724;
    let uVar7: u16;
    let puVar8: *mut u16;
    let puVar9: u32;
    let uVar10: u16;
    let iStack18: i16;

    puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_4, param_2, param_3);
    // puVar6 = (puVar8 >> 0x10);
    uVar3 = puVar8;
    // uVar7 = (param_1 >> 0x10);
    iVar7 = param_1;
    pass1_1010_a5ca(uVar3, puVar6, iVar7.field_0x9a, uVar3, puVar6);
    if (uVar3 == 0x0) {
        iVar7.field_0x94 = 0x0;
        iVar7.field_0x9c = 0x1;
    }
    if (-0x1 < uVar3) {
        if (iVar7.field_0x9a < 0x72) {
            uVar10 = 0x31;
        } else {
            uVar10 = 0x41;
        }
        puVar9 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, uVar10, param_4, puVar6, param_3);
        uVar4 = iVar7.field_0x9a;
        ppcVar1 = (*puVar9 + 0x14);
        (**ppcVar1)(0x1010, puVar9, (puVar9 >> 0x10), uVar4, uVar4 >> 0xf);
        if ((extraout_DX | uVar4) == 0x0) {
            iStack18 = 0x0;
        } else {
            uVar2 = (uVar4 + 0x16);
            iStack18 = (uVar2 + 0x4);
        }
        if ((iStack18 != 0x0) && (uVar3 != 0x0)) {
            uVar5 = ((iStack18 - uVar3) * 0x64) / iStack18;
            uVar4 = uVar5 / 0xa;
            iVar7.field_0x94 = uVar4;
            if (0x4 < uVar5 % 0xa) {
                iVar7.field_0x94 = uVar4 + 0x1;
            }
        }
    }
    return;
}



i16  pass1_1040_5eaa(param_1: u32)

{
let iVar1: i16; let iVar2: i16;
let iVar3: i16; let uVar4: u16;

// uVar4 = (param_1 >> 0x10);
iVar3 = param_1; iVar1 = (iVar3 + 0x9a); iVar2 = iVar1; if (true) {
iVar2 = iVar3; switch(iVar1) {
0x0 => 0x70 => 0x71 => (iVar3 + 0x98) = 0x0; return iVar3; 0x1 => 0x2 => (iVar3 + 0x98) = 0xd; return iVar3; 0x3 => (iVar3 + 0x98) = 0xe; return iVar3; 0x4 => 0x4b => (iVar3 + 0x98) = 0xf; break; 0x5 => (iVar3 + 0x98) = 0x10; return iVar3; 0x6 => (iVar3 + 0x98) = 0x11; return iVar3; 0x7 => (iVar3 + 0x98) = 0x12; break; 0x8 => (iVar3 + 0x98) = 0x13; break; 0x9 => 0xa => 0xb => (iVar3 + 0x98) = 0x14; break; 0xc => (iVar3 + 0x98) = 0x18; break; 0xd => (iVar3 + 0x98) = 0x19; break; 0xe => 0x76 => (iVar3 + 0x98) = 0x17; break; 0xf => 0x10 => 0x11 => (iVar3 + 0x98) = 0x1a; break; 0x12 => (iVar3 + 0x98) = 0x1b; break; 0x13 => (iVar3 + 0x98) = 0x1c; break; 0x14 => (iVar3 + 0x98) = 0x1d; break; 0x15 => 0x16 => 0x17 => 0x18 => 0x19 => (iVar3 + 0x98) = 0x1e; break; 0x1a => (iVar3 + 0x98) = 0x1f; break; 0x1b => (iVar3 + 0x98) = 0x20; break; 0x1c => 0x1d => 0x1e => (iVar3 + 0x98) = 0x21; break; 0x1f => (iVar3 + 0x98) = 0x22; break; 0x20 => (iVar3 + 0x98) = 0x23; break; 0x21 => (iVar3 + 0x98) = 0x24; break; 0x22 => (iVar3 + 0x98) = 0x25; break; 0x23 => 0x24 => 0x25 => 0x26 => 0x27 => 0x28 => 0x29 => 0x2a => 0x2b => (iVar3 + 0x98) = 0x26; break; 0x2c => (iVar3 + 0x98) = 0x27; break; 0x2d => (iVar3 + 0x98) = 0x28; break; 0x2e => 0x2f => 0x30 => 0x31 => (iVar3 + 0x98) = 0x29; break; 0x32 => 0x33 => 0x34 => 0x35 => 0x4d => (iVar3 + 0x98) = 0x2a; break; 0x36 => (iVar3 + 0x98) = 0x2b; break; 0x37 => 0x38 => 0x39 => (iVar3 + 0x98) = 0x2c; break; 0x3a => (iVar3 + 0x98) = 0x2d; break; 0x3b => 0x3c => (iVar3 + 0x98) = 0x2e; break; 0x3d => (iVar3 + 0x98) = 0x2f; break; 0x3e => (iVar3 + 0x98) = 0x30; break; 0x3f => (iVar3 + 0x98) = 0x31; break; 0x40 => (iVar3 + 0x98) = 0x32; break; 0x41 => (iVar3 + 0x98) = 0x33; break; 0x42 => (iVar3 + 0x98) = 0x34; break; 0x43 => (iVar3 + 0x98) = 0x35; break; 0x44 => (iVar3 + 0x98) = 0x36; break; 0x45 => (iVar3 + 0x98) = 0x37; break; 0x46 => (iVar3 + 0x98) = 0x38; break; 0x47 => (iVar3 + 0x98) = 0x39; break; 0x48 => 0x49 => 0x4a => (iVar3 + 0x98) = 0x3a; break; 0x4c => (iVar3 + 0x98) = 0x3b; break; 0x4e => (iVar3 + 0x98) = 0x3c; break; 0x4f => 0x50 => (iVar3 + 0x98) = 0x3d; break; 0x51 => 0x52 => 0x53 => 0x54 => 0x55 => (iVar3 + 0x98) = 0x3e; break; 0x56 => 0x57 => 0x58 => 0x59 => 0x5a => (iVar3 + 0x98) = 0x3f; break; 0x5b => (iVar3 + 0x98) = 0x40; break; 0x5c => 0x5d => 0x5e => (iVar3 + 0x98) = 0x41; break; 0x5f => 0x60 => 0x61 => (iVar3 + 0x98) = 0x42; break; 0x62 => 0x63 => 0x64 => 0x65 => 0x66 => (iVar3 + 0x98) = 0x43; break; 0x67 => 0x68 => (iVar3 + 0x98) = 0x44; break; 0x69 => (iVar3 + 0x98) = 0x45; break; 0x6a => (iVar3 + 0x98) = 0x46; break; 0x6b => (iVar3 + 0x98) = 0x47; break; 0x6c => (iVar3 + 0x98) = 0x48; break; 0x6d => (iVar3 + 0x98) = 0x49; break; 0x6e => (iVar3 + 0x98) = 0x4a; break; 0x6f => (iVar3 + 0x98) = 0x4b; break; 0x74 => (iVar3 + 0x98) = 0x15; break; 0x75 => (iVar3 + 0x98) = 0x16; break; default: (iVar3 + 0x98) = 0x4c;
}
}
return iVar2;
}



astruct_18 *  pass1_1040_6360(param_1: & mut Struct18,param_2: u8)

{
unk_draw_op_1040_b0f8(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_6402(param_1: &mut Struct57, param_2: u16, param_3: *mut u8, param_4: i16,
                       param_5: u16)

{
    let ppcVar1: u32;
    let iVar2: &mut Struct725;
    let uVar2: u16;
    let puVar3: *mut u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1850, param_2);
    // uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    iVar2.field_0x8e = 0x0;
    iVar2.field_0x92 = 0x0;
    param_1 = 0x67ba;
    iVar2.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar3 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    &iVar2.field_0x92 = puVar3;
    (&iVar2.field_0x92 + 0x2) = (puVar3 >> 0x10);
    ppcVar1 = (*iVar2.field_0x92 + 0x4);
    (**ppcVar1)();
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_6470(param_1: &mut Struct18, param_2: u16) {
    let iVar1: &mut Struct18;
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field_0x0 = 0x67ba;
    iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    if (&iVar1.field_0x92 != 0x0) {
        pass1_1010_1ea6(&iVar1.field_0x92, param_1, param_2);
    }
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, iVar1.field_0x6);
    fn_ptr_1000_17ce(ctx, &iVar1.field_0x8e, 0x1000);
    ui_cleanup_op_1040_782c(param_1, 0x1000);
    return;
}


astruct_18 *
pass1_1040_6794(param_1: & mut Struct18,param_2: u8,param_3: u16)

{
pass1_1040_6470(param_1, param_3); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1040_6826(param_1: &mut Struct57, param_2: u16) {
    let iVar1: i16;
    let uVar2: u16;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_2, 0xfda));
    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = 0x0;
    (iVar1 + 0x98) = 0x0;
    param_1 = 0x6f32;
    (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    return;
}


pub fn pass1_1040_6862(param_1: &mut Struct18) {
    param_1.field_0x0 = 0x6f32;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    unk_draw_op_1040_b0f8(param_1);
    return;
}



u16
pass1_1040_68d2(param_1: * mut u32,param_2: & mut i16,param_3: u16,param_4: u16,param_5: i16,
param_6: u16)

{
let ppcVar1: u32; let uVar2: u16;

if (param_5 == 0x2b) {
if ( * param_2 == 0x4) {
win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), param_6);
}
}
else {
if (param_5 != 0x111) {
uVar2 = pass1_1040_b316(param_1, param_2, param_3,param_4, param_5); return uVar2;
}
ppcVar1 = ( * param_1 + 0x80); (* * ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
}
return 0x1;
}



pub fn pass1_1040_692e(param_1: *mut u32) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x7c);
    (**ppcVar1)();
    return;
}


pub fn pass1_1040_6cac(param_1: u32, param_2: u16) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar4: i16;
    let uVar5: u16;

    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_1ea6((iVar4 + 0x94), param_1 & 0xffff | uVar5 << 0x10, param_2,
    );
    puVar1 = (iVar4 + 0x98);
    uVar2 = (iVar4 + 0x9a);
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1010, puVar1, uVar2, 0x1);
    }
    (iVar4 + 0x98) = 0x0;
    (iVar4 + 0x94) = 0x0;
    return;
}


pub fn pass1_1040_6cfa(param_1: u32) -> u16

{
    let ppcVar1: u32;

    ppcVar1 = ((param_1 + 0x98) + 0x8);
    (**ppcVar1)();
    return 0x1;
}


astruct_18 *  pass1_1040_6f0c(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_6862(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1040_6fb6(param_1: &mut Struct57, param_2: u16) {
    let iVar1: i16;
    let uVar2: u16;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_2, 0xfd9));
    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = 0x0;
    (iVar1 + 0x98) = 0x0;
    param_1 = 0x76a4;
    (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    return;
}


u16
pass1_1040_7044(param_1: * mut u32,param_2: & mut i16,param_3: u16,param_4: u16,param_5: i16,
param_6: u16)

{
let ppcVar1: u32; let uVar2: u16;

if (param_5 == 0x2b) {
if ( * param_2 == 0x4) {
win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), param_6);
}
}
else {
if (param_5 != 0x111) {
uVar2 = pass1_1040_b316(param_1, param_2, param_3,param_4, param_5); return uVar2;
}
ppcVar1 = ( * param_1 + 0x80); (* * ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
}
return 0x1;
}



pub fn pass1_1040_70a0(param_1: *mut u32) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x7c);
    (**ppcVar1)();
    return;
}


pub fn pass1_1040_741e(param_1: u32, param_2: u16) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar4: i16;
    let uVar5: u16;

    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_1ea6((iVar4 + 0x94), param_1 & 0xffff | uVar5 << 0x10, param_2,
    );
    puVar1 = (iVar4 + 0x98);
    uVar2 = (iVar4 + 0x9a);
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1010, puVar1, uVar2, 0x1);
    }
    (iVar4 + 0x98) = 0x0;
    (iVar4 + 0x94) = 0x0;
    return;
}


pub fn pass1_1040_746c(param_1: u32) -> u16

{
    let ppcVar1: u32;

    ppcVar1 = ((param_1 + 0x98) + 0x8);
    (**ppcVar1)();
    return 0x1;
}


astruct_18 *  pass1_1040_767e(param_1: & mut Struct18,param_2: u8)

{
unk_draw_op_1040_b0f8(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1040_78de() {
    return;
}


pub fn pass1_1040_79c0(param_1: *mut u32, param_2: &mut i16, param_3: u16, param_4: u16, param_5: u16) -> u16

{
    let ppcVar1: u32;
    let cVar2: u8;
    let uVar3: u16;
    let unaff_CS: u16;

    if (param_5 == 0xa1) {
        ppcVar1 = (*param_1 + 0x38);
        uVar3 = (**ppcVar1)();
        return uVar3;
    }
    if (param_5 < 0xa2) {
        if (param_5 == 0x85) {
            ppcVar1 = (*param_1 + 0x1c);
            (**ppcVar1)();
            return 0x1;
        }
        if (param_5 < 0x86) {
            cVar2 = param_5;
            if (cVar2 == '\x02') {
                ppcVar1 = (*param_1 + 0x24);
                (**ppcVar1)();
                return 0x1;
            }
            if (cVar2 == '\f') {
                ppcVar1 = (*param_1 + 0x18);
                (**ppcVar1)();
                return 0x1;
            }
            if (cVar2 == '\x0f') {
                ppcVar1 = (*param_1 + 0x60);
                uVar3 = (**ppcVar1)();
                return uVar3;
            }
            if (cVar2 == '+') {
                if (*param_2 != 0x4) {
                    return 0x1;
                }
                win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), unaff_CS);
                return 0x1;
            }
        }
    } else {
        if (param_5 == 0x114) {
            ppcVar1 = (*param_1 + 0x58);
            uVar3 = (**ppcVar1)();
            return uVar3;
        }
        if (param_5 < 0x115) {
            if (param_5 == 0x104) {
                ppcVar1 = (*param_1 + 0x30);
                uVar3 = (**ppcVar1)();
                return uVar3;
            }
            if (param_5 == 0x111) {
                ppcVar1 = (*param_1 + 0x10);
                uVar3 = (**ppcVar1)();
                return uVar3;
            }
        } else {
            if (param_5 == 0x115) {
                ppcVar1 = (*param_1 + 0x54);
                uVar3 = (**ppcVar1)();
                return uVar3;
            }
            if (param_5 == 0x201) {
                ppcVar1 = (*param_1 + 0x44);
                (**ppcVar1)();
                return 0x1;
            }
            if (param_5 == 0x204) {
                ppcVar1 = (*param_1 + 0x28);
                (**ppcVar1)();
                return 0x1;
            }
        }
    }
    return 0x0;
}


pub fn pass1_1040_8054() -> u16

{
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_805a(param_1: *mut u8) -> u32

{
    let unaff_DI: i16;
    let uVar1: u16;
    let unaff_SS: u16;

    if (ctx.PTR__LOOP_1050_4230 == 0x0) {
        mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x28, unaff_SS, param_1, unaff_DI);
    }
    uVar1 = (ctx.PTR__LOOP_1050_4230 >> 0x10);
    return CONCAT22((ctx.PTR__LOOP_1050_4230 + 0x10),
                    (ctx.PTR__LOOP_1050_4230 + 0xe));
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_807e(param_1: u32, param_2: u16, param_3: u16) {
    let uVar1: u16;
    let ppcVar2: u32;
    let puVar3: u32;
    let puVar4: u32;
    let in_DX: *mut u8;
    let uVar5: u16;
    let extraout_DX: *mut u8;
    let puVar6: *mut u8;
    let extraout_DX_00: *mut u8;
    let puVar7: *mut u8;
    let iVar9: &mut Struct395;
    let uVar8: u16;
    let paVar9: &mut Struct43;
    let uStack10: u32;
    let iVar8: &mut Struct393;

    if (param_2 == 0x1) {
        pass1_1040_805a(in_DX);
        return;
    }
    paVar9 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc, param_2, param_3);
    // uVar5 = (paVar9 >> 0x10);
    puVar3 = paVar9;
    if ((uVar5 | puVar3) != 0x0) {
        ppcVar2 = (paVar9 + 0x14);
        puVar4 = puVar3;
        (**ppcVar2)(0x1010, puVar3, uVar5);
        uStack10 = CONCAT22(extraout_DX, puVar4);
        // uVar8 = (param_1 >> 0x10);
        iVar9 = param_1;
        puVar6 = extraout_DX;
        if (iVar9.field_0x70 != 0x0) {
            puVar4 = &iVar9.field_0x70;
            uVar1 = (&iVar9.field_0x70 + 0x2);
            puVar6 = (uVar1 | puVar4);
            if (puVar6 != 0x0) {
                ppcVar2 = *puVar4;
                (**ppcVar2)();
                puVar6 = extraout_DX_00;
            }
        }
        mem_op_1000_179c(0x14, puVar6, 0x1000);
        puVar7 = (puVar6 | puVar4);
        if (puVar7 == 0x0) {
            puVar4 = 0x0;
            puVar7 = 0x0;
        } else {
            struct_1008_4c58(CONCAT22(puVar6, puVar4));
        }
        &iVar9.field_0x70 = puVar4;
        (&iVar9.field_0x70 + 0x2) = puVar7;
        pass1_1008_4d84(iVar9.field_0x70, uStack10, puVar7);
        if (paVar9 != 0x0) {
            ppcVar2 = paVar9;
            (**ppcVar2)(0x1008, puVar3, uVar5, 0x1);
        }
        return;
    }
    return;
}


pub fn pass1_1040_824a(param_1: u32, param_2: i16) -> u16

{
    if ((param_1 + 0x6) != param_2) {
        return 0x1;
    }
    return 0x0;
}



astruct_18 *
pass1_1040_83e6(param_1: & mut Struct18,param_2: u8,param_3: u16)

{
ui_cleanup_op_1040_782c(param_1, param_3); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



astruct_57 *
pass1_1040_8478(param_1: & mut Struct57,param_2: u16,param_3: & mut String,param_4: & mut String,
param_5: u16,param_6: u16)

{
let uVar1: u16; let iVar2: & mut Struct712; let uVar2: u16;

get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfc3, param_5);
// uVar2 = (param_1 >> 0x10);
iVar2 = param_1; iVar2.field_0x8e = 0x0;
iVar2.field_0x98 = param_2; iVar2.field_0x9a = 0x0; iVar2.field_0xb2 = 0x0; param_1 = 0x8ddc; iVar2.field_0x2 = & ctx.PTR_LOOP_1050_1040; iVar2.field_0x9e = 0x0; iVar2.field_0xa2 = 0x12c; uVar1 = str_op_1008_60e8(param_4, param_6); iVar2.field_0x90 = uVar1; iVar2.field_0x92 = param_6; uVar1 = str_op_1008_60e8(param_3,param_6); iVar2.field_0x94 = uVar1; iVar2.field_0x96 = param_6; load_icon_1040_8b92(param_1, 0x1008); ctx.PTR_LOOP_1050_5df8 = 0x0; return param_1;
}


pub fn pass1_1040_869a(param_1: &mut Struct18) {
    let iVar1: i16;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field_0x0 = 0x8ddc;
    (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    fn_ptr_1000_17ce(ctx, (iVar1 + 0x90), 0x1000);
    fn_ptr_1000_17ce(ctx, (iVar1 + 0x94), 0x1000);
    ui_cleanup_op_1040_782c(param_1, 0x1000);
    return;
}


pub fn pass1_1040_8978(param_1: *mut u32, param_2: u16, param_3: u16, param_4: u16,
                       param_5: &WNDCLASS16)

{
    let ppcVar1: u32;

    unk_win_msg_op_1008_9510(&ctx.PTR_LOOP_1050_5df4, 0x1008, param_5);
    win_1008_5c5c(param_5, param_3, param_4, _PTR_LOOP_1050_02a0, param_2);
    ppcVar1 = (*param_1 + 0x74);
    (**ppcVar1)(0x1008, param_1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_89a4(param_1: *mut u32, param_2: *mut u16, param_3: *mut u8, param_4: i16,
                       param_5: &WNDCLASS16)

{
    let uVar1: u16;
    let uVar2: u16;
    let ppcVar3: u32;
    let uVar4: u16;
    let uVar5: u16;
    let uVar6: u16;
    let puVar7: *mut u16;

    unk_win_msg_op_1008_9510(&ctx.PTR_LOOP_1050_5df4, 0x1008, param_5);
    uVar1 = (param_2 + 0x2);
    uVar2 = *param_2;
    uVar6 = 0x1010;
    puVar7 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_5, param_3, param_4);
    // uVar5 = (puVar7 >> 0x10);
    uVar4 = puVar7;
    if ((uVar4 + 0x72) != 0x0) {
        uVar6 = 0x1008;
        win_1008_5c7c(ctx.PTR__LOOP_1050_02a0, CONCAT22(uVar1, uVar2), param_5, uVar4, uVar5);
        (param_1 + 0x8c) = uVar4;
    }
    ppcVar3 = (*param_1 + 0x74);
    (**ppcVar3)(uVar6, param_1);
    return;
}


pub fn pass1_1040_8b3c(param_1: u16, param_2: u32, param_3: u32, param_4: u16) {
    if ((param_3._2_2_ != 0x0) && ((param_3._2_2_ == (&ctx.PTR_LOOP_1050_0000 + 0x1) || param_3._2_2_ == &ctx.PTR_LOOP_1050_0002 || (((&ctx.PTR_LOOP_1050_0002 + 0x1) < param_3._2_2_ + -0x2 && (param_3._2_2_ + -0x6 < &ctx.PTR_LOOP_1050_0002)))))) {
        ctx.PTR_LOOP_1050_5df4 = 0x0;
        ctx.PTR_LOOP_1050_5df8 = param_3._2_2_;
        return;
    }
    post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), (param_2 >> 0x10),
                           param_3, param_3._2_2_, param_4);
    return;
}



astruct_18 *  pass1_1040_8db6(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_869a(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1040_8e58(param_1: i16, param_2: u16, param_3: u16, param_4: u32) -> u16

{
    pass1_1040_b040(CONCAT22(param_2, param_1), CONCAT22(param_4, param_3),
                    (param_4 >> 0x10));
    CONCAT22(param_2, param_1) = 0x8f3c;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1040_8e82(param_1: &mut Struct18) {
    param_1.field_0x0 = 0x8f3c;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    unk_draw_op_1040_b0f8(param_1);
    return;
}



astruct_18 *  pass1_1040_8f16(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_8e82(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


pub fn pass1_1040_9252(ctx: &mut AppContext, param_1: &mut Struct161, param_2: u16) {
    let pi_var1: *mut i16;
    let i_var2: i16;
    // let i_var3: &mut Struct161;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    //  i_var3 = param_1;
    if (param_1.field_0x4 != 0x0) {
        draw_text_1040_9650(ctx, param_1, param_2);
    }
    if (param_1.field_0x8 != 0x0) {
        pass1_1040_9618(ctx, param_1);
    }
    i_var2 = param_1.field_0x32;
    if (param_1.field_0x22 < i_var2) {
        param_1.field_0x22 = i_var2;
    }
    i_var2 = param_1.field_0x34;
    if (param_1.field_0x24 < i_var2) {
        param_1.field_0x24 = i_var2;
    }
    i_var2 = param_1.field_0x26 + param_1.field_0x2a;
    if (param_1.field_0x22 < i_var2) {
        param_1.field_0x22 = i_var2;
    }
    i_var2 = param_1.field_0x28 + param_1.field_0x2c;
    if (param_1.field_0x24 < i_var2) {
        param_1.field_0x24 = i_var2;
    }
    pi_var1 = &param_1.field_0x22;
    *pi_var1 = *pi_var1 + param_1.field_0x36;
    pi_var1 = &param_1.field_0x24;
    *pi_var1 = *pi_var1 + param_1.field_0x36;
    return;
}


LRESULT  pass1_1040_93e6(param_1: u32,param_2: HWND16)

{
let LVar1: LRESULT;

LVar1 = SendMessage16(param_2, 0x0, 0x0,
CONCAT22(0x111, (param_1 + 0x1c))); return LVar1;
}


pub fn pass1_1040_9422(param_1: *mut u32) {
    let ppcVar1: u32;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x8) != 0x0) {
        ppcVar1 = (*param_1 + 0x10);
        (**ppcVar1)();
    }
    if ((param_1 + 0x4) != 0x0) {
        ppcVar1 = (*param_1 + 0x14);
        (**ppcVar1)();
    }
    return;
}


pub fn pass1_1040_9618(ctx: &mut AppContext, param_1: &mut Struct161) {
    let u_var3: u32;
    u_var3 = pass1_1008_4772(param_1.field_0x8);
    param_1.field_0x2a = (u_var3 + 0x4);
    param_1.field_0x2c = (u_var3 + 0x8);
    return;
}


pub fn pass1_1040_97da(param_1: *mut u16, param_2: u8) -> u16

{
    mix_win_ui_op_1040_911e(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1040_9824(param_1: *mut u32) {
    let iVar1: i16;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0x0;
    (iVar1 + 0x4) = 0x0;
    (iVar1 + 0x56) = 0x0;
    (iVar1 + 0x5a) = 0x0;
    (iVar1 + 0x5c) = 0x0;
    *(iVar1 + 0x6) = 0x0;
    return;
}


pub fn pass1_1040_a204(param_1: *mut u16, param_2: u8) -> u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


u32
pass1_1040_a2cc(param_1: i16,param_2: u32,param_3: u32,param_4: u16,param_5: * mut u8,
param_6: u16,param_7: u16)

{
let uVar1: u16;

if (param_3._2_2_ == 0x1826) {
if ((param_3 == 0x1) | | ((0x1 < param_3 - 0x1 & & (param_3 - 0x3 < 0x2)))) {
uVar1 = 0x1;
}
else {
uVar1 = 0x0;
}
return uVar1;
}
pass1_1040_b54a(param_1, param_2, (param_2 > > 0x10), param_3, param_5,
param_6, param_7); return CONCAT22(param_5, param_4);
}


astruct_18 *
pass1_1040_a4c2(param_1: & mut Struct18,param_2: u8,param_3: u16)

{
free_proc_inst_1040_a294(param_1, param_3); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1040_a564(param_1: *mut u32) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    *param_1 = 0x0;
    (param_1 + 0x4) = 0x0;
    (param_1 + 0x6) = 0x0;
    return;
}


pub fn pass1_1040_a582(param_1: *mut u32) {
    fn_ptr_1000_17ce(ctx, *param_1, 0x1000);
    return;
}


pub fn pass1_1040_a5d0(param_1: u32) {
    let uVar1: u16;
    let uVar2: u16;
    let iVar4: &mut Struct258;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar1 = iVar4.field_0x2;
    uVar2 = iVar4.field_0x4;
    if ((uVar2 | uVar1) != 0x0) {
        pass1_1000_54e8(0xa582, &ctx.PTR_LOOP_1050_1040, (uVar1 - 0x2), 0xa,
                        uVar1, uVar2);
        fn_ptr_1000_17ce(ctx, CONCAT22(uVar2, uVar1 - 0x2), 0x1000);
    }
    fn_ptr_1000_17ce(ctx, iVar4.field_0xc, 0x1000);
    return;
}


pub fn pass1_1040_a640(param_1: &mut Struct57, param_2: u32, param_3: u16) {
    let iVar1: i16;
    let uVar2: u16;

    struct_1040_b082(param_1, CONCAT22(param_3, 0x1f1));
    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = param_2;
    (iVar1 + 0x98) = 0x0;
    (iVar1 + 0xea) = 0x0;
    param_1 = 0xac08;
    (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    return;
}


pub fn pass1_1040_a84a(param_1: u32, param_2: u16) {
    win_ui_dlg_op_1040_a94a(ctx, param_1, param_2);
    return;
}


astruct_18 *  pass1_1040_abe2(param_1: & mut Struct18,param_2: u8)

{
unk_draw_op_1040_b0f8(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_ac84(param_1: &mut Struct57, param_2: u16, param_3: *mut u8, param_4: i16,
                       param_5: u16)

{
    let iVar1: &mut Struct726;
    let uVar1: u16;
    let puVar2: *mut u16;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x1f3));
    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field_0x94 = 0x0;
    &iVar1.field_0x98 = 0x0;
    param_1 = 0xafc4;
    iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    iVar1.field_0x94 = ctx._PTR_LOOP_1050_5ef0;
    ctx._PTR_LOOP_1050_5ef0 = 0x0;
    puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3d, param_5, param_3, param_4);
    iVar1.field_0x98 = puVar2;
    iVar1.field_0x9a = (puVar2 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_ace8(param_1: &mut Struct18) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xafc4;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6));
    unk_draw_op_1040_b0f8(param_1);
    return;
}


pub fn pass1_1040_ad14(param_1: u32, param_2: u16) {
    win_ui_op_1040_ae04(ctx, param_1, param_2, 0, 0, 0);
    return;
}


pub fn pass1_1040_ad24(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: *mut u8,
                       param_6: u16, param_7: u16)

{
    if (param_4._2_2_ == 0xeb) {
        win_ui_op_1040_ae04(ctx, CONCAT22(param_2, param_1), param_7, 0, 0, 0);
    } else {
        if (param_4._2_2_ != 0x1f0) {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, param_6, param_7);
            return;
        }
        msg_box_ui_op_1040_ad66(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
    }
    return;
}


astruct_18 *  pass1_1040_af9e(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_ace8(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn pass1_1040_b040(param_1: &mut Struct57, param_2: u32, param_3: u16) {
    let iVar1: i16;
    let uVar2: u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, (param_2 + 0x12), param_3);
    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0x0;
    (iVar1 + 0x90) = param_2;
    param_1 = 0xb772;
    (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    return;
}


pub fn pass1_1040_b0bc(param_1: &mut Struct57, param_2: u32, param_3: u32) {
    let iVar1: i16;
    let uVar2: u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, param_3, (param_3 >> 0x10));
    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x8e) = 0x0;
    (iVar1 + 0x90) = param_2;
    param_1 = 0xb772;
    (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    return;
}


pub fn pass1_1040_b17c(param_1: u32, param_2: u32, param_3: *mut u8, param_4: i16, param_5: i16,
                       param_6: u16)

{
    let piVar1: *mut i16;
    let uVar2: u32;
    let mut pcVar3: String;
    let uVar4: u16;
    let iVar5: i16;
    let uVar6: u16;
    let puVar7: *mut u8;
    let puVar8: *mut u16;
    let puStack12: *mut u16;
    let iStack4: i16;

    iStack4 = 0x0;
    loop {
        // uVar6 = (param_1 >> 0x10);
        iVar5 = param_1;
        piVar1 = (iVar5 + 0x90);
        // puVar7 = (piVar1 >> 0x10);
        if (*piVar1 == iStack4 || *piVar1 < iStack4) { break; }
        param_5 = (iStack4 * 0x2 + param_2);
        uVar2 = (piVar1 + 0x2);
        (iStack4 * 0xa + uVar2 + 0x4) = param_5;
        iStack4 += 0x1;
        param_3 = puVar7;
    }
    puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_6, param_3, param_5);
    // uVar4 = (puVar8 >> 0x10);
    uVar2 = (iVar5 + 0x90);
    puStack12 = (uVar2 + 0x2);
    // for (iStack4 = 0x0; piVar1 = (iVar5 + 0x90),
    //     *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 += 0x1) {
    //   uVar2 = (iVar5 + 0x90);
    //   uVar2 = (uVar2 + 0x6);
    //   pcVar3 = pass1_1010_b038(puVar8,uVar2,(uVar2 >> 0x10),
    //                            (puStack12 + 0x4),param_4);
    //   string_1040_a626(puStack12,CONCAT22(uVar4,pcVar3),uVar4);
    //   puStack12 = (puStack12 & 0xffff0000 | (puStack12 + 0xa));
    // }
    return;
}


pub fn pass1_1040_b316(param_1: *mut u32, param_2: u16, param_3: u16, param_4: u16, param_5: i16) -> u16

{
    let ppcVar1: u32;
    let uStack4: u16;

    if (param_5 == 0xf) {
        ppcVar1 = (*param_1 + 0x60);
        uStack4 = (**ppcVar1)();
    } else {
        if (param_5 == 0x111) {
            ppcVar1 = (*param_1 + 0x10);
            (**ppcVar1)();
            uStack4 = 0x1;
        } else {
            uStack4 = pass1_1040_79c0(param_1, param_2, param_3, param_4, param_5);
        }
    }
    return uStack4;
}


pub fn pass1_1040_b45e(param_1: u32, param_2: HWND16) {
    let uVar1: u32;
    let piVar2: *mut i16;
    let iVar3: i16;
    let uVar4: u16;
    let iStack8: i16;
    INT16 * pIStack6;

    // uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x90) != 0x0) {
        uVar1 = (iVar3 + 0x90);
        (uVar1 + 0x14) = (iVar3 + 0x6);
        uVar1 = (iVar3 + 0x90);
        pIStack6 = (uVar1 + 0x2);
        // for (iStack8 = 0x0; piVar2 = (iVar3 + 0x90),
        //     *piVar2 != iStack8 && iStack8 <= *piVar2; iStack8 += 0x1) {
        //   SetDlgItemText16(param_2,*pIStack6,(pIStack6 + 0x2));
        //   pIStack6 = (INT16 *)(pIStack6 & 0xffff0000 | (pIStack6 + 0xa));
        //   param_2 = ctx.s_tile2_bmp_1050_1538;
        // }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_b4c8(param_1: u32, param_2: *mut u8, param_3: i16, param_4: u16) {
    let iVar1: i16;
    let uVar2: u32;
    let iVar3: i16;
    let uVar4: u16;
    let puVar5: *mut u16;

    // uVar4 = (param_1 >> 0x10);
    if ((param_1 + 0x90) != 0x0) {
        puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x32, param_4, param_2, param_3);
        uVar2 = (param_1 + 0x90);
        iVar1 = (uVar2 + 0xa);
        iVar3 = iVar1 + -0x4;
        if (iVar3 == 0x0) {
            ui_op_1010_79aa(puVar5, 0xfd9, 0x0, param_4);
            if (iVar3 == 0x0) {
                uVar4 = 0xe;
//LAB_1040_b50f:
                unk_win_op_1010_7300(puVar5, CONCAT22(iVar3, iVar3), uVar4, CONCAT22(iVar3, iVar3));
                return;
            }
        } else {
            if (((0x0 < iVar1 + -0x5) && (!SBORROW2(iVar1 + -0x5, 0x1))) && (iVar3 = iVar1 + -0x7, iVar3 == 0x0 || iVar1 + -0x6 < 0x1)) {
                ui_op_1010_79aa(puVar5, 0xfda, 0x0, param_4);
                if (iVar3 == 0x0) {
                    uVar4 = 0xd;
//           TODO: goto LAB_1040_b50f;
                }
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_b54a(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: *mut u8,
                       param_6: u16, param_7: u16)

{
    let paVar1: &mut Struct18;
    let ppcVar2: u32;
    let uVar3: u32;
    let iVar4: i16;
    let iVar5: i16;
    let paVar6: &mut Struct18;
    let uVar7: u16;
    let iVar6: &mut Struct515;
    let unaff_DI: i16;
    let uVar8: u16;
    let puVar9: *mut u16;
    let uVar10: u8;
    let uVar11: u8;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;

    if (param_4._2_2_ == 0xea) {
        ppcVar2 = (CONCAT22(param_2, param_1) + 0x5c);
        (**ppcVar2)(param_6, param_1, param_2);
    } else {
        if (param_4._2_2_ == 0xeb) {
            puVar9 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_7, param_5, unaff_DI);
            // uVar7 = (puVar9 >> 0x10);
            paVar1 = (param_1 + 0x90);
            if (paVar1 != 0x0) {
                // uVar8 = (paVar1 >> 0x10);
                uVar12 = 0x1010;
                paVar6 = paVar1;
                pass1_1010_ad64(puVar9, CONCAT22((paVar1 + 0xa), uVar7),
                                (paVar1 + 0x6), paVar1, uVar7);
                (param_1 + 0x90) = paVar6;
                (param_1 + 0x92) = uVar7;
                if ((uVar7 | (param_1 + 0x90)) == 0x0) {
                    (param_1 + 0x90) = paVar1;
                } else {
                    if (paVar1 != 0x0) {
                        pass1_1040_a5d0(paVar1);
                        uVar12 = 0x1000;
                        fn_ptr_1000_17ce(ctx, paVar1, 0x1000);
                    }
                    ppcVar2 = (CONCAT22(param_2, param_1) + 0x70);
                    (**ppcVar2)(uVar12, param_1, param_2);
                }
            }
        } else {
            if (param_4._2_2_ == 0x1790) {
                puVar9 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x32, param_7, param_5, unaff_DI);
                uVar3 = (param_1 + 0x90);
                uVar3 = (uVar3 + 0x6);
                iVar4 = pass1_1010_7d38(puVar9, (puVar9 >> 0x10), uVar3,
                                        (uVar3 >> 0x10), param_7);
                iVar5 = iVar4;
                ui_op_1010_79aa(puVar9, 0xfab, 0x0, param_7);
                if (iVar5 != 0x0) {
                    return;
                }
                if (iVar4 == 0x0) {
                    uVar3 = (param_1 + 0x90);
                    // uVar8 = (uVar3 >> 0x10);
                    iVar6 = uVar3;
                    uVar3 = iVar6.field_0x6;
                    uVar13 = uVar3;
                    // uVar14 = (uVar3 >> 0x10);
                    uVar12 = 0x14;
                } else {
                    uVar3 = (param_1 + 0x90);
                    // uVar8 = (uVar3 >> 0x10);
                    iVar6 = uVar3;
                    uVar3 = iVar6.field_0x6;
                    uVar13 = uVar3;
                    // uVar14 = (uVar3 >> 0x10);
                    uVar12 = 0x9;
                }
                uVar10 = uVar8;
                uVar11 = (uVar8 >> 0x8);
            } else {
                if (param_4._2_2_ == 0x1824) {
                    puVar9 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x32, param_7, param_5, unaff_DI);
                    iVar6 = puVar9;
                    uVar3 = (param_1 + 0x90);
                    ui_op_1010_79aa(puVar9, 0xfc5, (uVar3 + 0x6), param_7);
                    if (iVar6 != 0x0) {
                        return;
                    }
                    uVar3 = (param_1 + 0x90);
                    uVar3 = (uVar3 + 0x6);
                    uVar13 = uVar3;
                    // uVar14 = (uVar3 >> 0x10);
                    uVar12 = 0x12;
                    uVar10 = 0x0;
                    uVar11 = 0x0;
                } else {
                    if (param_4._2_2_ != 0x1830) {
                        post_win_msg_1040_7b3c(CONCAT13((param_2 >> 0x8),
                                                        CONCAT12(param_2, param_1)), param_3,
                                               param_4, param_4._2_2_, param_6);
                        return;
                    }
                    puVar9 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x32, param_7, param_5, unaff_DI);
                    iVar6 = puVar9;
                    uVar3 = (param_1 + 0x90);
                    ui_op_1010_79aa(puVar9, 0xfb6, (uVar3 + 0x6), param_7);
                    if (iVar6 != 0x0) {
                        return;
                    }
                    uVar3 = (param_1 + 0x90);
                    uVar3 = (uVar3 + 0x6);
                    uVar13 = uVar3;
                    // uVar14 = (uVar3 >> 0x10);
                    uVar12 = 0xc;
                    uVar10 = 0x0;
                    uVar11 = 0x0;
                }
            }
            unk_win_op_1010_7300(puVar9, CONCAT13(uVar11, CONCAT12(uVar10, iVar6)), uVar12,
                                 CONCAT22(uVar14, uVar13));
        }
    }
    return;
}



ULONG  pass1_1040_b74c(param_1: & mut Struct18,param_2: u8)

{
unk_draw_op_1040_b0f8(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return (ULONG)param_1;
}



pub fn pass1_1040_b7ee(param_1: &mut Struct57, param_2: i32, param_3: u16) {
    let iVar1: i16;
    let uVar2: u16;
    let uVar3: u16;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_3, 0xfab));
    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x94) = 0x0;
    (iVar1 + 0x98) = 0x0;
    (iVar1 + 0xb0) = 0x0;
    (iVar1 + 0xb4) = 0x0;
    (iVar1 + 0xb6) = 0x0;
    param_1 = 0xbeba;
    (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    if (param_2 != 0x0) {
        // uVar3 = (param_2 >> 0x10);
        (iVar1 + 0xb0) = (param_2 + 0x6);
        (iVar1 + 0xb4) = (param_2 + 0x14);
    }
    return;
}



u16
pass1_1040_b864(param_1: * mut u32,param_2: & mut i16,param_3: u16,param_4: u16,param_5: i16,
param_6: u16)

{
let ppcVar1: u32; let uVar2: u16;

if (param_5 == 0x2b) {
if ( * param_2 == 0x4) {
win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), param_6);
}
}
else {
if (param_5 != 0x111) {
uVar2 = pass1_1040_b316(param_1, param_2, param_3,param_4, param_5); return uVar2;
}
ppcVar1 = ( * param_1 + 0x7c); (* * ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
}
return 0x1;
}



pub fn pass1_1040_b8be(param_1: *mut u32) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x80);
    (**ppcVar1)();
    return;
}


pub fn pass1_1040_bb5a(param_1: u32) -> u16

{
    let ppcVar1: u32;

    ppcVar1 = ((param_1 + 0x94) + 0x8);
    (**ppcVar1)();
    return 0x0;
}


astruct_18 *  pass1_1040_be94(param_1: & mut Struct18,param_2: u8)

{
unk_draw_op_1040_b0f8(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}



pub fn struct_1040_bf3e(param_1: *mut u16, param_2: u16) -> u16

{
    let iVar1: &mut Struct442;
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0x389a;
    iVar1.field_0x2 = 0x1008;
    *param_1 = 0x3aa8;
    iVar1.field_0x2 = 0x1008;
    iVar1.field_0x4 = param_2;
    *param_1 = 0x3ab0;
    iVar1.field_0x2 = 0x1008;
    iVar1.field_0x6 = 0x0;
    *param_1 = 0xc53e;
    iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    return param_1;
}


pub fn pass1_1040_bf92(param_1: *mut u16, param_2: u16) {
    let iVar1: &mut Struct514;
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0xc53e;
    iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pass1_1010_1ea6(iVar1.field_0x6, param_1 & 0xffff | uVar1 << 0x10, param_2);
    unk_destroy_win_op_1010_2fa0(iVar1.field_0x6, 0x1010);
    *param_1 = 0x3ab0;
    iVar1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    iVar1.field_0x2 = 0x1008;
    return;
}


pub fn pass1_1040_bfde(param_1: u32, param_2: *mut u32, param_3: u16) {
    let ppcVar1: u32;
    let uVar2: u32;
    let iVar3: i16;
    let uVar4: u16;

    // uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    (iVar3 + 0x6) = param_2;
    ppcVar1 = (*param_2 + 0x4);
    (**ppcVar1)();
    uVar2 = (iVar3 + 0x6);
    (uVar2 + 0x22) = (iVar3 + 0x4);
    pass1_1010_2ee2((iVar3 + 0x6), param_3, 0x1010);
    return;
}


pub fn pass1_1040_c518(param_1: u32, param_2: u8, param_3: u16) -> u32

{
    pass1_1040_bf92(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1040_c54a(param_1: *mut u16, param_2: u16, param_3: *mut u32, param_4: u16,
                       param_5: u16)

{
    let ppcVar1: u32;
    let iVar3: i16;
    let extraout_DX: u16;
    let iVar2: &mut Struct164;
    let uVar4: u16;
    let puVar5: u32;
    let uVar6: u16;
    let uVar7: u32;

    iVar3 = (param_3 + 0x12) + 0xc8;
    uVar7 = 0x0;
    uVar6 = 0x0;
    ppcVar1 = (*param_3 + 0x14);
    puVar5 = param_3;
    (**ppcVar1)();
    mixed_struct_op_1040_8fb8(param_1, 0x0, CONCAT22(extraout_DX, iVar3), puVar5,
                              (puVar5 >> 0x10), uVar6, uVar7,
                              (uVar7 >> 0x10), extraout_DX, param_4, param_5);
    // uVar4 = (param_1 >> 0x10);
    iVar2 = param_1;
    iVar2.field_0x42 = param_3;
    iVar2.field_0x46 = 0x0;
    iVar2.field_0x48 = param_2;
    *param_1 = 0xc9f2;
    iVar2.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pass1_1040_c630((param_1 & 0xffff | uVar4 << 0x10), param_4,
                    param_5);
    return;
}


pub fn pass1_1040_c5ac(param_1: *mut u16) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar4: i16;
    let uVar5: u16;

    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    *param_1 = 0xc9f2;
    (iVar4 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    ctx.PTR_LOOP_1050_5f02 = ctx.PTR_LOOP_1050_5f02 + -0x1;
    if (ctx.PTR_LOOP_1050_5f02 == 0x0) {
        puVar1 = (iVar4 + 0x8);
        uVar2 = (iVar4 + 0xa);
        if ((uVar2 | puVar1) != 0x0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        puVar1 = (iVar4 + 0xc);
        uVar2 = (iVar4 + 0xe);
        if ((uVar2 | puVar1) != 0x0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    mix_win_ui_op_1040_911e(param_1);
    return;
}


pub fn pass1_1040_c60e(param_1: &mut Struct65) -> u16

{
    let uVar1: u32;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x42) != 0x0) {
        uVar1 = (param_1 + 0x42);
        return (uVar1 + 0x12);
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_c630(param_1: *mut u32, param_2: u16, param_3: u16) {
    let iVar1: i16;
    let ppcVar2: u32;
    let uVar3: u32;
    let uVar4: u32;
    let iVar4: &mut Struct165;
    let uVar5: u16;

    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar3 = iVar4.field_0x42;
    if ((uVar3 + 0x12) != 0x71) {
        iVar4.field_0x36 = 0x5;
        iVar4.field_0x26 = 0x5;
        iVar4.field_0x28 = 0x5;
        iVar1 = iVar4.field_0x36;
        iVar4.field_0x30 = iVar1;
        iVar4.field_0x2e = iVar1;
        if (ctx.PTR_LOOP_1050_5f02 == 0x0) {
            ctx._PTR_LOOP_1050_5f04 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc, 0xff, param_3);
            param_2 = 0x1010;
            ctx._PTR_LOOP_1050_5f08 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc, 0x100, param_3);
        }
        ctx.PTR_LOOP_1050_5f02 = ctx.PTR_LOOP_1050_5f02 + 0x1;
        iVar4.field_0x8 = ctx._PTR_LOOP_1050_5f04;
        iVar4.field_0xc = ctx._PTR_LOOP_1050_5f08;
        pass1_1040_9618(ctx, param_1);
        iVar4.field_0x20 = 0x0;
        iVar4.field_0x1e = 0xc8;
        iVar4.field_0x22 = 0xa0;
        iVar4.field_0x24 = iVar4.field_0x2c + iVar4.field_0x36;
        iVar4.field_0x2e = iVar4.field_0x36 * 0x3 + iVar4.field_0x2a;
        iVar4.field_0x30 = iVar4.field_0x36;
        iVar4.field_0x32 = iVar4.field_0x22 - iVar4.field_0x36;
        iVar4.field_0x3c = 0x25;
        uVar4 = *param_1;
        ppcVar2 = (uVar4 + 0x4);
        (**ppcVar2)(param_2, param_1);
        ppcVar2 = (uVar4 + 0x8);
        (**ppcVar2)(param_2, param_1, uVar5);
    }
    return;
}


pub fn pass1_1040_c71e(param_1: u32, param_2: u16) {
    let iVar1: i16;
    let uVar2: u16;

    pass1_1040_9252(ctx, param_1, param_2);
    // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x28) = (iVar1 + 0x24) / 0x2 - (iVar1 + 0x2c) / 0x2;
    return;
}


pub fn pass1_1040_c94a(param_1: i16, param_2: u16, param_3: *mut u8, param_4: i16, param_5: u16,
                       param_6: u16)

{
    let uVar1: u16;
    let uVar2: u32;
    let uVar3: u16;
    let uVar4: u16;
    let puVar5: *mut u16;

    if ((param_1 + 0x48) != 0x0) {
        puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_6, param_3, param_4);
        // uVar4 = (puVar5 >> 0x10);
        uVar2 = (param_1 + 0x42);
        uVar1 = (uVar2 + 0x12);
        param_5 = 0x1010;
        uVar3 = uVar1;
        pass1_1010_a5ca(puVar5, uVar4, uVar1, uVar1, uVar4);
        if (uVar3 == 0xffff) {
            (param_1 + 0x3c) = 0xf9;
        } else {
            if (uVar3 == 0x0) {
                (param_1 + 0x3c) = 0x25;
                if ((uVar1 == 0x5b) || (uVar1 == 0x9)) {
                    (param_1 + 0x3c) = 0xfe;
                }
            } else {
                (param_1 + 0x3c) = 0xfb;
            }
        }
    }
    draw_text_1040_94fc(CONCAT22(param_2, param_1), param_5);
    return;
}


pub fn pass1_1040_c9cc(param_1: *mut u16, param_2: u8) -> u16

{
    pass1_1040_c5ac(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_ca16(param_1: &mut Struct57, param_2: u16, param_3: *mut u8, param_4: i16,
                       param_5: u16)

{
    let iVar1: &mut Struct727;
    let uVar1: u16;
    let puVar2: *mut u16;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x1840));
    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field_0x94 = ctx._PTR_LOOP_1050_5f0c;
    &iVar1.field_0x98 = 0x0;
    iVar1.field_0x9c = 0x0;
    iVar1.field_0x9e = 0x0;
    param_1 = 0xd07c;
    iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3e, param_5, param_3, param_4);
    iVar1.field_0x98 = puVar2;
    iVar1.field_0x9a = (puVar2 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_ca74(param_1: &mut Struct18) {
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xd07c;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6));
    ctx.PTR_LOOP_1050_5f10 = 0x0;
    unk_draw_op_1040_b0f8(param_1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_caa6(param_1: u16, param_2: u32, param_3: *mut u8, param_4: i16, param_5: u16) {
    let puVar1: *mut u16;
    let iVar2: i16;

    iVar2 = 0x0;
    puVar1 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    pass1_1010_038e(puVar1, iVar2, param_5);
    destroy_window_1040_b726((ULONG *)CONCAT22(param_2, param_1), (param_2 >> 0x10), 0x1010);
    return;
}


LRESULT  pass1_1040_cc66(param_1: u32,param_2: u16)

{
let ppcVar1: u32; let LVar2: LRESULT;

ppcVar1 = ((param_1 + 0x98) + 0x10); ( * * ppcVar1)();
LVar2 = send_dlg_msg_1040_cf1c(param_1, param_2); return LVar2;
}



pub fn pass1_1040_cc8c(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: *mut u8,
                       param_6: u16, param_7: u16)

{
    if (param_4._2_2_ == 0xeb) {
        send_dlg_msg_1040_cf1c(CONCAT22(param_2, param_1), param_7);
    } else {
        if (param_4._2_2_ == (s_vrpal_bmp_1050_183a + 0x7)) {
            msg_box_op_1040_cce4(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
        } else {
            if (param_4._2_2_ != (s_vrpal_bmp_1050_183a + 0x8)) {
                pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, param_6, param_7);
                return;
            }
            if (param_4 == 0x1) {
                send_dlg_item_1040_ce76(CONCAT22(param_2, param_1), param_6, param_7);
            }
        }
    }
    return;
}


pub fn pass1_1040_cdac(param_1: u32, param_2: u16, param_3: u16, param_4: i16, param_5: HWND16) -> u16

{
    let piVar1: *mut i16;
    let iVar2: i16;
    let bVar3: bool;
    let iVar4: i16;
    let uVar5: u16;

    bVar3 = false;
    iVar4 = param_1;
    // uVar5 = (param_1 >> 0x10);
    if (param_4 == 0x0) {
        iVar2 = (iVar4 + 0x9e);
        piVar1 = (iVar4 + 0x9c);
        if (*piVar1 == iVar2 || *piVar1 < iVar2) {
            // goto
            // LAB_1040_cdef;
        }
        piVar1 = (iVar4 + 0x9e);
        *piVar1 = *piVar1 + 0x1;
    } else {
        if (param_4 != 0x1) {
            // goto
            // LAB_1040_cdef;
        }
        if ((iVar4 + 0x9e) < 0x1) {
            // goto
            // LAB_1040_cdef;
        }
        piVar1 = (iVar4 + 0x9e);
        *piVar1 = *piVar1 + -0x1;
    }
    bVar3 = true;
//LAB_1040_cdef:
    if (bVar3) {
        SetDlgItemInt16(param_5, 0x0, (iVar4 + 0x9e), 0x18e);
    }
    return 0x0;
}


astruct_18 *  pass1_1040_d056(param_1: & mut Struct18,param_2: u8)

{
pass1_1040_ca74(param_1); if ((param_2 & 0x1) != 0x0) {
fn_ptr_1000_17ce(param_1, 0x1000);
}
return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_d0f8(param_1: &mut Struct57, param_2: u16) {
    let uVar1: u16;
    let in_DX: *mut u8;
    let uVar2: u16;
    let puVar3: *mut u8;
    let puVar4: *mut u8;
    let iVar5: &mut Struct438;
    let unaff_DI: i16;
    let uVar5: u16;
    let unaff_SS: u16;
    let puVar6: *mut u16;
    let uVar7: u32;
    let iVar8: &mut Struct392;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x1845));
    // uVar5 = (param_1 >> 0x10);
    iVar5 = param_1;
    &iVar5.field_0x94 = 0x0;
    iVar5.field_0x98 = ctx._PTR_LOOP_1050_5f16;
    &iVar5.field_0x9c = 0x0;
    iVar5.field_0xa0 = 0x0;
    param_1 = 0xd8c4;
    iVar5.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x47, unaff_SS, in_DX, unaff_DI);
    // uVar2 = (puVar6 >> 0x10);
    iVar5.field_0x94 = puVar6;
    iVar5.field_0x96 = uVar2;
    uVar7 = pass1_1018_5732(iVar5.field_0x94, uVar2, iVar5.field_0x98, puVar6, uVar2,
                            unaff_SS);
    // puVar3 = (uVar7 >> 0x10);
    iVar5.field_0x9c = uVar7;
    iVar5.field_0x9e = puVar3;
    uVar1 = puVar3 | iVar5.field_0x9c;
    if (uVar1 == 0x0) {
        mem_op_1000_179c(0xc, puVar3, 0x1000);
        puVar4 = (puVar3 | uVar1);
        if (puVar4 == 0x0) {
            &iVar5.field_0x9c = 0x0;
        } else {
            pass1_1010_8ef2(CONCAT22(puVar3, uVar1), puVar4, unaff_SS);
            iVar5.field_0x9c = uVar1;
            iVar5.field_0x9e = puVar4;
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_d1bc(param_1: *mut astruct_18) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar4: &mut Struct513;
    let uVar4: u16;

    // uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field_0x0 = 0xd8c4;
    iVar4.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, iVar4.field_0x6);
    puVar1 = iVar4.field_0x9c;
    uVar2 = iVar4.field_0x9e;
    if (uVar2 | puVar1) != 0x0 {
        ppcVar3 = *puVar1;
        (**ppcVar3)(&ctx.PTR_LOOP_1050_1038, puVar1, uVar2, 0x1);
    }
    unk_draw_op_1040_b0f8(param_1);
    return;
}


pub fn pass1_1040_d29c(param_1: u32, param_2: u16) {
    send_ldg_item_msg_1040_d79c(param_1, param_2);
    return;
}


pub fn pass1_1040_d76e(param_1: u32) {
    let uVar1: u32;
    let iVar2: i16;
    let uVar3: u16;

    // uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    uVar1 = (iVar2 + 0x94);
    pass1_1018_5742(uVar1, (uVar1 >> 0x10), (iVar2 + 0x9c),
                    (iVar2 + 0x98));
    (iVar2 + 0x9c) = 0x0;
    return;
}


pub fn pass1_1040_d89e(param_1: *mut astruct_18, param_2: u8) -> *mut astruct_18

{
    pass1_1040_d1bc(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}





