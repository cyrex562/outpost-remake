use crate::cleanup::{
    destroy_win_1040_7b98, destroy_window_1040_b726, ui_cleanup_op_1040_782c,
    unk_destroy_win_op_1010_2fa0,
};
use crate::defines::{
    Struct161, Struct18, Struct19, Struct20, Struct57, Struct65, Struct_1040_0a1a, Struct_311,
    U32Ptr,
};
use crate::draw::draw_1040::{
    draw_text_1040_94fc, draw_text_1040_9650, load_icon_1040_8b92, unk_draw_op_1040_b0f8,
};
use crate::file::file_1010::unk_io_op_1010_830a;
use crate::fn_ptr::fn_ptr_1000::fn_ptr_1000_17ce;
use crate::global::AppContext;
use crate::mem_1000::mem_op_1000_179c;
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{pass1_1000_3e2c, pass1_1000_54e8, pass1_1000_5586};
use crate::pass::pass_1008::{pass1_1008_4772, pass1_1008_4d84, pass1_1008_5fd8};
use crate::pass::pass_1010::{
    pass1_1010_038e, pass1_1010_0538, pass1_1010_1ea6, pass1_1010_2ee2, pass1_1010_7b8c,
    pass1_1010_7d38, pass1_1010_8ef2, pass1_1010_a50c, pass1_1010_a5ca, pass1_1010_ad64,
    pass1_1010_ae12, pass1_1010_ae92, pass1_1010_debe, pass1_1010_e964,
};
use crate::pass::pass_1018::pass1_1018_5742;
use crate::pass::pass_1028::pass1_1028_4ab2;
use crate::pass::pass_1038::{pass1_1038_b6e0, pass1_1040_073a};
use crate::string::string_1000::{string_1000_3cea, string_1000_3d3e};
use crate::string::string_1008::str_op_1008_60e8;
use crate::string::string_1010::load_string_1010_84ac;
use crate::struct_ops::struct_1008::struct_1008_4c58;
use crate::struct_ops::struct_1030::struct_op_1030_73a8;
use crate::struct_ops::struct_1040::{
    mixed_struct_op_1040_8fb8, struct_1040_a598, struct_1040_b082,
};
use crate::sys_api::{
    free_proc_inst_1040_a294, get_sys_metrics_1040_7728, unk_win_msg_op_1008_9510,
};
use crate::ui::ui_1008::{pass1_1008_a9ec, win_1008_5c5c, win_1008_5c7c};
use crate::ui::ui_1010::ui_op_1010_79aa;
use crate::ui::ui_1040::{
    check_dialog_btn_1040_1b8a, mix_win_ui_op_1040_911e, msg_box_op_1040_cce4,
    msg_box_ui_op_1040_ad66, post_win_msg_1040_7b3c, send_dlg_item_1040_ce76,
    send_dlg_msg_1040_cf1c, send_ldg_item_msg_1040_d79c, unk_win_ui_op_1040_b230,
    win_ui_dlg_op_1040_a94a, win_ui_get_prop_op_1040_9566, win_ui_op_1040_ae04,
};
use crate::util::{read_struct_from_addr, CONCAT12, CONCAT13, CONCAT22, SBORROW2, SUB42, ZEXT24};
use crate::win_struct::{HWND16, LRESULT, WNDCLASS16};
use crate::winapi::{GetDlgItem16, SendMessage16, SetDlgItemInt16, SetDlgItemText16};

pub fn pass1_1040_0a1a(
    ctx: &mut AppContext,
    struct_6: &mut Struct_1040_0a1a,
    struct_7: &mut Struct18,
    struct_8: &mut Struct18,
) {
    let var1: u16;
    let var2: u32;
    let var3: u32;
    let struct_2: &mut Struct_311;
    let var4: u16;
    // let extraout_dx: U32Ptr;
    let struct_5: &mut Struct18;
    // let extraout_dx_00: U32Ptr;
    let opt_struct_4: Option<&mut Struct18>;
    let struct_1: &mut Struct_1040_0a1a;
    let struct_3: &mut Struct_311;
    let var5: u16;
    let var6: u32;
    let var7: u32;

    // u_var10 = (param_1 >> 0x10);
    struct_1 = struct_6;
    struct_2 = &mut (struct_6.field_0x8e);
    // u_var11 = (u_var4 >> 0x10);
    struct_3 = struct_2;
    var2 = (struct_3.field_0xa);
    var7 = var2;
    var4 = ((struct_3.field_0xc) | var7);
    if var4 == 0x0 {
        return;
    }
    var3 = (*var2 + 0x14);
    (**var3)();
    var6 = CONCAT22(struct_7.field_0x0, var4);
    struct_5 = struct_7;
    if (struct_1.field_0x70) != 0x0 {
        var4 = (struct_1.field_0x70);
        var1 = (struct_1.field_0x72);
        struct_5 = read_struct_from_addr::<Struct18>((var1 | var4) as u32);
        if struct_5 != 0x0 {
            var3 = *var4;
            (**var3)();
            struct_5 = struct_8;
        }
    }

    mem_op_1000_179c(ctx, 0x14, struct_5, 0x1000);
    opt_struct_4 = (struct_5 | var4);
    if opt_struct_4 == None {
        var4 = 0x0;
        opt_struct_4 = None;
    } else {
        struct_1008_4c58(CONCAT22(struct_5.field_0x0, var4));
    }
    struct_1.field_0x70 = var4;
    struct_1.field_0x72 = opt_struct_4.unwrap().field_0x0;
    pass1_1008_4d84(struct_1.field_0x70, var6, opt_struct_4);
    return;
}

pub fn pass1_1040_0b6a(ctx: &mut AppContext, param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
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
) -> &mut Struct57 {
    let struct_4: &mut Struct57;
    let u_var1: u16;
    let struct_5: &mut Struct19;

    get_sys_metrics_1040_7728(struct_2, 0x1, param_2, 0xfcd, param_5);
    struct_4 = struct_2;
    struct_4.field_0x8e = None;
    struct_2.field_0x0 = 0xdb0;
    struct_4.field_0x2 = ctx.PTR_LOOP_1050_1040;
    struct_5 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x39,
        struct_3,
        struct_1,
        param_7,
        0,
    );
    struct_4.field_0x8e = Some(struct_5.clone());
    // struct_4.field_0x90 = (struct_5 >> 0x10);
    struct_4.field_0x74 = 0x1;
    return struct_2;
}

pub fn pass1_1040_0c54(ctx: &mut AppContext, struct_1: &mut Struct18, param_2: u16) {
    struct_1.field_0x0 = 0xdb0;
    struct_1.field_0x2 = ctx.PTR_LOOP_1050_1040;
    struct_1.field_0x8e = 0x0;
    ui_cleanup_op_1040_782c(struct_1, param_2);
    return;
}

pub fn pass1_1040_0d80() -> u16 {
    return 0x1;
}

pub fn pass1_1040_0d8a(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8,
    param_3: u16,
) -> &mut Struct18 {
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
    param_7: &mut WNDCLASS16,
) {
    let struct_2: &mut Struct57;
    let pu_var2: &mut Struct20;

    get_sys_metrics_1040_7728(struct_1, 0x1, 0x0, 0x1c0, param_4);
    struct_2 = struct_1;
    struct_2.field_0x92 = param_3;
    struct_2.field_0x96 = None;
    struct_2.field_0x98 = param_2;
    struct_1.field_0x0 = (ctx.s_overflow_on_node__d_1050_11ca[0x8..].to_string());
    struct_2.field_0x2 = ctx.PTR_LOOP_1050_1040;
    pu_var2 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x3a,
        param_7,
        param_5,
        param_6,
        0,
    );
    struct_2.field_0x8e = pu_var2.clone();
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_0e86(
    ctx: &mut AppContext,
    struct_1: &mut Struct18,
    param_2: &mut WNDCLASS16,
    unaff_di: i16,
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
    struct_2 = read_struct_from_addr(struct_3.field_0x92);
    u_var1 = (struct_3.field_0x94);
    pu_var3 = (u_var1 | struct_2);
    if pu_var3 != 0x0 {
        pass1_1040_a5d0(struct_2 & 0xffff | u_var1 << 0x10);
        fn_ptr_1000_17ce(ctx, struct_2, 0x1000);
    }
    ctx.PTR_LOOP_1050_5b82 = (struct_3 + 0x96);
    if (struct_3 + 0x92) == 0x0 {
        u_var6 = ctx.PTR_LOOP_1050_1038;
        pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (struct_3 + 0x6));
    } else {
        pu_var7 = mixed_1010_20ba(
            ctx,
            ctx.PTR_LOOP_1050_0ed0,
            0x32,
            param_2,
            pu_var3,
            unaff_di,
            0,
        );
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
    param_8: &mut WNDCLASS16,
) {
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
                    win_handle,
                );
                return;
            }
            pu_var4 = mixed_1010_20ba(
                ctx,
                ctx.PTR_LOOP_1050_0ed0,
                0x32,
                param_8,
                param_5,
                param_6,
                0,
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
    param_6: u16,
) {
    let u_var1: u16;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let pu_var5: &mut Struct20;

    if (param_1 + 0x92) != 0x0 {
        u_var2 = (param_1 + 0x8e);
        u_var1 = (u_var2 + 0xa);
        pu_var5 = mixed_1010_20ba(ctx, ctx.PTR_LOOP_1050_0ed0, param_6, param_3, param_4, 0, 0);
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
            param_6,
        );
    }
    destroy_win_1040_7b98(CONCAT22(param_2, param_1), param_5);
    ctx.PTR_LOOP_1050_5b80 = 0x0;
    return;
}

pub unsafe fn pass1_1040_11ac(
    ctx: &mut AppContext,
    param_1: &mut Struct18,
    param_2: u8,
    param_3: &mut WNDCLASS16,
) -> &mut Struct18 {
    pass1_1040_0e86(ctx, param_1, param_3, 0);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_123e(
    ctx: &mut AppContext,
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: &mut Struct19,
    param_7: i16,
    param_8: &mut WNDCLASS16,
) -> &mut Struct57 {
    let i_var1: &mut Struct57;
    let u_var1: u16;
    let pu_var2: &mut Struct20;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfd1, param_5);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x8e = None;
    param_1.field_0x0 = 0x17b0;
    i_var1.field_0x2 = ctx.PTR_LOOP_1050_1040;
    pu_var2 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x46,
        param_8,
        param_6,
        param_7,
        0,
    );
    i_var1.field_0x8e = pu_var2.clone();
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_1290(param_1: &mut Struct18) {
    param_1.field_0x0 = 0x17b0;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, ctx.PTR_LOOP_1050_1038);
    return;
}

pub fn pass1_1040_178a(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_1290(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_181c(
    ctx: &mut AppContext,
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: &mut Struct19,
    param_7: &mut WNDCLASS16,
    unaff_id: i16,
) {
    let i_var1: &mut Struct57;
    let pu_var2: &mut Struct20;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfbb, param_5);

    i_var1 = param_1;
    i_var1.field_0x8e = None;
    i_var1.field_0x92 = 0x0;
    i_var1.field_0x94 = 0x0;
    param_1.field_0x0 = 0x1c48;
    i_var1.field_0x2 = ctx.PTR_LOOP_1050_1040;
    pu_var2 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x2,
        param_7,
        param_6,
        unaff_di,
        0,
    );
    i_var1.field_0x8e = Some(pu_var2.clone());
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_1876(ctx: &mut AppContext, param_1: &mut Struct18) {
    param_1.field_0x0 = 0x1c48;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, ctx.PTR_LOOP_1050_1038);
    return;
}

pub fn pass1_1040_1ab0(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: u16,
    param_6: u16,
) -> u32 {
    let bstack6: bool;
    let u_stack4: u16;

    bstack6 = 0x0;
    u_stack4 = 0x0;
    if param_4._2_2_ == 0x1831 {
        (param_1 + 0x92) = 0x1;
        (param_1 + 0x94) = 0x1;
        check_dialog_btn_1040_1b8a(param_1, param_2);
    } else {
        bstack6 = post_win_msg_1040_7b3c(
            CONCAT13((param_2 >> 0x8), CONCAT12(param_2, param_1)),
            param_3,
            param_4,
            param_4._2_2_,
            param_6,
        );
        u_stack4 = param_5;
    }
    return CONCAT22(u_stack4, bstack6);
}

pub fn pass1_1040_1c22(ctx: &mut AppContext, param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_1876(ctx, param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_1cb4(
    ctx: &mut AppContext,
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: &mut WNDCLASS16,
    param_5: u16,
    param_6: &mut Struct19,
    param_7: i16,
    param_8: &mut WNDCLASS16,
) {
    let pu_var1: &mut Struct19;
    let i_var2: &mut Struct57;
    let pu_var3: &mut Struct19;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xe8, param_5);
    i_var2 = param_1;
    i_var2.field_0x8e = None;
    i_var2.field_0x92 = None;
    param_1.field_0x0 = 0x1eee;
    i_var2.field_0x2 = ctx.PTR_LOOP_1050_1040;
    pu_var3 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x2,
        param_8,
        param_6,
        param_7,
        0,
    );
    pu_var1 = pu_var3;
    i_var2.field_0x8e = Some(pu_var3.clone());
    pu_var3 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x37,
        param_8,
        pu_var1,
        param_7,
        0,
    );
    i_var2.field_0x92 = pu_var3.clone;
    // i_var2.field_0x94 = (pu_var3 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_1d24(ctx: &mut AppContext, param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x1eee;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, ctx.PTR_LOOP_1050_1038);
    return;
}

pub fn pass1_1040_1e80(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: u16,
    param_6: u16,
) -> u32 {
    let b_stack6: bool;
    let u_stack4: u16;

    b_stack6 = false;
    u_stack4 = 0x0;
    if param_4._2_2_ == 0xe4 {
        pass1_1008_a9ec((param_1 + 0x92));
    } else {
        b_stack6 = post_win_msg_1040_7b3c(
            CONCAT22(param_2, param_1),
            param_3,
            param_4,
            param_4._2_2_,
            param_6,
        );
        u_stack4 = param_5;
    }
    return CONCAT22(u_stack4, b_stack6);
}

pub fn pass1_1040_1ec8(ctx: &mut AppContext, param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_1d24(ctx, param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1040_1f5a(
    ctx: &mut AppContext,
    param_1: &mut Struct57,
    param_2: u16,
    param_3: i16,
    param_4: &mut WNDCLASS16,
) {
    let pi_var1: U32Ptr;
    let pu_var2: &mut Struct19;
    let i_var3: &mut Struct719;
    let pa_var3: Option<&mut Struct43>;
    let u_var4: &mut Struct19;
    let pu_var5: &mut Struct19;
    let i_var6: &mut Struct57;
    let u_var7: u16;
    let i_var8: i16;
    let i_var9: i16;
    let u_var10: u16;
    let local_16: u32;
    let u_stack18: u32;

    i_var6 = param_1;
    // u_var7 = (param_1 >> 0x10);
    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfcf, param_2);
    (i_var6 + 0x8e) = 0x0;
    (i_var6 + 0xa2) = 0x0;
    (i_var6 + 0xa6) = 0x0;
    param_1.field_0x0 = 0x237e;
    (i_var6 + 0x2) = ctx.PTR_LOOP_1050_1040;
    pa_var3 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x1cc, param_4);
    (i_var6 + 0x8e) = pa_var3;
    // (i_var6 + 0x90) = (pa_var3 >> 0x10);
    u_var4 = pass1_1008_4772(pa_var3.clone());
    // pu_var2 = (u_var4 >> 0x10);
    pu_var2 = u_var4;
    pu_var5 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x48,
        param_4,
        pu_var2,
        param_3,
        0,
    );
    local_16 = CONCAT22((u_var4 + 0x8) + 0xa, 0xa);
    u_stack18 = CONCAT22(0x1d6, (u_var4 + 0x4) + -0xa);
    (i_var6 + 0x92) = local_16;
    (i_var6 + 0x96) = u_stack18;
    (i_var6 + 0x9a) = local_16;
    (i_var6 + 0x9e) = u_stack18;
    pi_var1 = (i_var6 + 0x9c);
    *pi_var1 = *pi_var1 + 0x14;
    i_var9 = i_var6 + 0xa2;
    i_var8 = i_var6 + 0xa6;
    u_var10 = u_var7;
    pu_var5 = mixed_1010_20ba(
        ctx.PTR_LOOP_1050_0ed0,
        0x2b,
        param_4,
        (pu_var5 >> 0x10),
        i_var6 + 0xa2,
    );
    pass1_1010_0538(
        pu_var5,
        CONCAT22(u_var7, i_var8),
        CONCAT22(u_var10, i_var9),
        0x1010,
        param_4,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_205e(param_1: &mut Struct18) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct624;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    param_1.field_0x0 = 0x237e;
    i_var4.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pu_var1 = i_var4.field_0x8e;
    u_var2 = i_var4.field_0x90;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    fn_ptr_1000_17ce(ctx, i_var4.field_0xa2, 0x1000);
    fn_ptr_1000_17ce(ctx, i_var4.field_0xa6, 0x1000);
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, i_var4.field_0x6);
    ui_cleanup_op_1040_782c(param_1, ctx.PTR_LOOP_1050_1038);
    return;
}

pub fn pass1_1040_2358(ctx: &mut AppContext, param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_205e(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_23ea(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) {
    let u_var1: u32;
    let i_var2: &mut Struct436;
    let unaff_DI: i16;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x9a, param_2, 0xfbd, param_5);
    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    i_var2.field_0x8e = 0x0;
    i_var2.field_0x92 = 0x0;
    i_var2.field_0x94 = 0x0;
    param_1.field_0x0 = 0x2956;
    i_var2.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    i_var2.field_0x8a = 0x26;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x6, param_6, param_7, unaff_DI);
    i_var2.field_0x8e = pu_var3;
    i_var2.field_0x90 = (pu_var3 >> 0x10);
    u_var1 = i_var2.field_0x8e;
    i_var2.field_0x92 = (u_var1 + 0x28);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_2464(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x2956;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}

pub fn pass1_1040_288e(param_1: u32) {
    let u_var1: u16;
    let ppcVar2: u32;
    let u_var3: u32;
    let puVar4: u32;
    let pu_var5: u32;
    let extraout_dx: U32Ptr;
    let puVar6: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let puVar7: U32Ptr;
    let i_var8: i16;
    let uVar9: u16;

    // uVar9 = (param_1 >> 0x10);
    i_var8 = param_1;
    u_var3 = (i_var8 + 0x8e);
    pu_var5 = (u_var3 + 0x24);
    ppcVar2 = (*pu_var5 + 0x14);
    (**ppcVar2)();
    puVar4 = pu_var5;
    puVar6 = extraout_dx;
    if ((i_var8 + 0x70) != 0x0) {
        puVar4 = (i_var8 + 0x70);
        u_var1 = (i_var8 + 0x72);
        puVar6 = (u_var1 | puVar4);
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
    (i_var8 + 0x70) = puVar4;
    (i_var8 + 0x72) = puVar7;
    pass1_1008_4d84(
        (i_var8 + 0x70),
        pu_var5 & 0xffff | ZEXT24(extraout_dx) << 0x10,
        puVar7,
    );
    return;
}

pub fn pass1_1040_2930(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_2464(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pas1_1040_29c2(
    ctx: &mut AppContext,
    struct_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) -> &mut Struct57 {
    let struct_2: &mut Struct57;
    pass1_1040_b0bc(struct_1, param_2, CONCAT22(param_3, 0x157));
    struct_2 = struct_1;
    struct_1.field_0x0 = 0x2e26;
    (struct_2.field_0x2) = ctx.PTR_LOOP_1050_1040;
    load_string_1010_84ac(ctx.PTR_LOOP_1050_14cc, 0x1010);
    (struct_2.field_0x94) = param_4;
    (struct_2.field_0x96) = param_5;
    load_string_1010_84ac(ctx.PTR_LOOP_1050_14cc, 0x1010);
    (struct_2.field_0x98) = param_4;
    (struct_2.field_0x9a) = param_5;
    return struct_1;
}

pub fn pass1_1040_2a22(ctx: &mut AppContext, struct_1: &mut Struct18) {
    let struct_2: &mut Struct18;
    struct_2 = struct_1;
    struct_1.field_0x0 = 0x2e26;
    struct_2.field_0x2 = ctx.PTR_LOOP_1050_1040;
    fn_ptr_1000_17ce(ctx, struct_2.field_0x94, 0x1000);
    fn_ptr_1000_17ce(ctx, struct_2.field_0x98, 0x1000);
    unk_draw_op_1040_b0f8(struct_1);
    return;
}

pub fn pass1_1040_2dac(param_1: u32) {
    let u_var1: u32;
    let u_var2: u32;
    let var_10: i16;

    u_var1 = (param_1 + 0x90);
    u_var2 = struct_op_1030_73a8((u_var1 + 0x6));
    // for (iStack10 = 0x0; iStack10 < 0x5; iStack10 += 0x1) {
    //   pass1_1028_4ab2(u_var2,(&ctx.PTR_LOOP_1050_5d04 + iStack10 * 0xc),
    //                   (iStack10 * 0xc + 0x5d02));
    // }
    return;
}

pub fn pass1_1040_2e00(ctx: &mut AppContext, param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_2a22(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_2ea2(
    ctx: &mut AppContext,
    struct_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    struct_3: &mut Struct19,
    param_7: i16,
    wnd_class_1: &mut WNDCLASS16,
) {
    let struct_2: &mut Struct57;
    let struct_4: &mut Struct19;

    get_sys_metrics_1040_7728(struct_1, 0x1, param_2, 0x180, param_5);
    struct_2 = struct_1;
    struct_2.field_0x8e = None;
    struct_2.field_0x90 = 0x0;
    struct_2.field_0x92 = None;
    struct_2.field_0x94 = 0x0;
    struct_2.field_0x96 = None;
    struct_1.field_0x0 = 0x3436;
    struct_2.field_0x2 = ctx.PTR_LOOP_1050_1040;
    struct_4 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x3c,
        wnd_class_1,
        struct_3,
        param_7,
        0,
    );
    struct_2.field_0x96 = struct_4.clone();
    return;
}

pub fn pass1_1040_2f06(ctx: &mut AppContext, param_1: &mut Struct18) {
    param_1.field_0x0 = 0x3436;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, ctx.PTR_LOOP_1050_1038);
    return;
}

pub fn pass1_1040_2f32(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    struct_1: &mut Struct19,
    param_4: u16,
    wnd_class_1: &mut WNDCLASS16,
) {
    let struct_2: &mut Struct19;
    let i_var2: i16;

    i_var2 = 0x0;
    struct_2 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x2b,
        wnd_class_1,
        struct_1,
        param_4 as i16,
        0,
    );
    pass1_1010_038e(struct_2, i_var2, wnd_class_1);
    destroy_win_1040_7b98(CONCAT22(param_2, param_1) as i32, 0x1010);
    return;
}

pub fn pass1_1040_3410(ctx: &mut AppContext, param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_2f06(ctx, param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_34a2(
    ctx: &mut AppContext,
    struct_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: &mut Struct19,
    param_7: i16,
    wnd_class_1: &mut WNDCLASS16,
) {
    let struct_2: &mut Struct57;
    let struct_3: &mut Struct19;

    get_sys_metrics_1040_7728(struct_1, 0x1, param_2, 0x192, param_5);
    struct_2 = struct_1;
    struct_2.field_0x8e = None;
    struct_2.field_0x94 = 0x0;
    struct_2.field_0x96 = None;
    struct_2.field_0x98 = 0x0;
    struct_1.field_0x0 = (ctx.s_Null_Ptr_1050_38f3 + 0x7);
    struct_2.field_0x2 = ctx.PTR_LOOP_1050_1040;
    struct_3 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x3c,
        wnd_class_1,
        param_6,
        param_7,
        0,
    );
    struct_2.field_0x8e = Some(struct_3.clone());
    struct_2.field_0x90 = (struct_3 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_3506(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = (s_Null_Ptr_1050_38f3 + 0x7);
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_3532(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: i16, param_5: u16) {
    let pu_var1: U32Ptr;
    let i_var2: i16;

    i_var2 = 0x0;
    pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    pass1_1010_038e(pu_var1, i_var2, param_5);
    destroy_win_1040_7b98(CONCAT22(param_2, param_1), 0x1010);
    return;
}

pub fn pass1_1040_38d4(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_3506(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_3966(
    ctx: &mut AppContext,
    struct_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    struct_3: &mut Struct19,
    param_7: i16,
    wnd_class_1: &mut WNDCLASS16,
) {
    let struct_2: &mut Struct57;
    let u_var1: u16;
    let mut struct_4: &mut Struct19;

    get_sys_metrics_1040_7728(struct_1, 0x1, param_2, 0x185, param_5);
    struct_2 = struct_1;
    struct_2.field_0x8e = None;
    struct_2.field_0x92 = 0x0;
    struct_2.field_0x96 = None;
    struct_2.field_0x9a = 0x0;
    struct_2.field_0x9c = 0x0;
    struct_2.field_0x9e = 0x0;
    struct_2.field_0xa0 = 0x0;
    struct_2.field_0xa2 = 0x0;
    struct_2.field_0xa4 = 0x5;
    struct_1.field_0x0 = 0x3ffc;
    struct_2.field_0x2 = ctx.PTR_LOOP_1050_1040;
    pu_var2 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x3c,
        wnd_class_1,
        struct_3,
        param_7,
        0,
    );
    struct_2.field_0x8e = pu_var2;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_39e2(ctx: &mut AppContext, param_1: &mut Struct18) {
    param_1.field_0x0 = 0x3ffc;
    (param_1.field_0x2) = ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, ctx.PTR_LOOP_1050_1038);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_3a0e(
    ctx: &mut AppContext,
    param_1: u32,
    struct_2: &mut Struct19,
    param_4: i16,
    wnd_class_1: &mut WNDCLASS16,
) {
    let struct_1: &mut Struct19;
    let i_var2: i16;

    i_var2 = 0x0;
    struct_1 = mixed_1010_20ba(
        ctx,
        ctx.PTR_LOOP_1050_0ed0,
        0x2b,
        wnd_class_1,
        struct_2,
        param_4,
        0,
    );
    pass1_1010_038e(struct_1, i_var2, wnd_class_1);
    destroy_win_1040_7b98(param_1 as i32, 0x1010);
    return;
}

pub fn pass1_1040_3fd6(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_39e2(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_4068(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: U32Ptr,
    param_7: i16,
    param_8: u16,
) {
    let pu_var1: U32Ptr;
    let i_var2: &mut Struct723;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfb7, param_5);
    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    i_var2.field_0x8e = 0x0;
    i_var2.field_0x92 = 0x0;
    i_var2.field_0x9a = 0x0;
    param_1.field_0x0 = 0x4466;
    i_var2.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    i_var2.field_0x76 = 0x1;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_8, param_6, param_7);
    // pu_var1 = (pu_var3 >> 0x10);
    i_var2.field_0x8e = pu_var3;
    i_var2.field_0x90 = pu_var1;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x29, param_8, pu_var1, param_7);
    i_var2.field_0x96 = pu_var3;
    i_var2.field_0x98 = (pu_var3 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_40e2(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x4466;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}

pub fn pass1_1040_4440(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_40e2(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_44d2(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: U32Ptr,
) {
    let u_var1: u32;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let i_var4: i16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let piStack8: U32Ptr;

    iVar6 = param_1;
    // uVar7 = (param_1 >> 0x10);
    struct_1040_b082(param_1, CONCAT22(param_3, 0xfa2));
    param_1.field_0x0 = 0x4824;
    (iVar6 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    mem_op_1000_179c(0x18, param_5, 0x1000);
    pu_var3 = (param_5 | param_4);
    if (pu_var3 == 0x0) {
        (iVar6 + 0x90) = 0x0;
    } else {
        struct_1040_a598(CONCAT22(param_5, param_4));
        (iVar6 + 0x90) = param_4;
        (iVar6 + 0x92) = pu_var3;
    }
    (iVar6 + 0x90) = 0x14;
    i_var4 = *(iVar6 + 0x90);
    u_var2 = i_var4 * 0xa + 0x2;
    mem_op_1000_179c(u_var2, pu_var3, 0x1000);
    piStack8 = CONCAT22(pu_var3, u_var2);
    if ((pu_var3 | u_var2) == 0x0) {
        u_var1 = (iVar6 + 0x90);
        (u_var1 + 0x2) = 0x0;
    } else {
        *piStack8 = i_var4;
        pass1_1000_5586(
            0xa564,
            &ctx.PTR_LOOP_1050_1040,
            i_var4,
            0xa,
            u_var2 + 0x2,
            pu_var3,
        );
        u_var1 = (iVar6 + 0x90);
        // u_var5 = (u_var1 >> 0x10);
        i_var4 = u_var1;
        (i_var4 + 0x2) = u_var2 + 0x2;
        (i_var4 + 0x4) = pu_var3;
    }
    u_var1 = (iVar6 + 0x90);
    (u_var1 + 0x6) = param_2;
    u_var1 = (iVar6 + 0x90);
    (u_var1 + 0xa) = 0x1;
    u_var1 = (iVar6 + 0x90);
    (u_var1 + 0x12) = (iVar6 + 0xa);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_45e8(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: u16,
    param_7: u16,
) {
    let paVar1: &mut Struct18;
    let ppcVar2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let paVar5: &mut Struct18;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let i_var8: i16;
    let unaff_DI: i16;
    let uVar9: u16;
    let paVar10: &mut Struct20;
    let piStack16: U32Ptr;

    if (param_4._2_2_ != 0xeb) {
        pass1_1040_b54a(
            param_1, param_2, param_3, param_4, param_5, param_6, param_7,
        );
        return;
    }
    paVar10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_7, param_5, unaff_DI);
    // puVar6 = (paVar10 >> 0x10);
    paVar1 = (param_1 + 0x90);
    if (paVar1 != 0x0) {
        paVar5 = paVar1;
        mem_op_1000_179c(0x18, puVar6, 0x1000);
        u_var4 = paVar5;
        puVar7 = (puVar6 | u_var4);
        if (puVar7 == 0x0) {
            u_var4 = 0x0;
            puVar7 = 0x0;
        } else {
            struct_1040_a598((paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
        }
        (param_1 + 0x90) = u_var4;
        (param_1 + 0x92) = puVar7;
        (param_1 + 0x90) = 0x14;
        i_var8 = *(param_1 + 0x90);
        u_var4 = i_var8 * 0xa + 0x2;
        mem_op_1000_179c(u_var4, puVar7, 0x1000);
        piStack16 = CONCAT22(puVar7, u_var4);
        if ((puVar7 | u_var4) == 0x0) {
            u_var3 = (param_1 + 0x90);
            (u_var3 + 0x2) = 0x0;
        } else {
            *piStack16 = i_var8;
            pass1_1000_5586(
                0xa564,
                &ctx.PTR_LOOP_1050_1040,
                i_var8,
                0xa,
                u_var4 + 0x2,
                puVar7,
            );
            u_var3 = (param_1 + 0x90);
            // uVar9 = (u_var3 >> 0x10);
            i_var8 = u_var3;
            (i_var8 + 0x2) = u_var4 + 0x2;
            (i_var8 + 0x4) = puVar7;
        }
        u_var3 = (param_1 + 0x90);
        (u_var3 + 0x6) = (paVar1 + 0x6);
        u_var3 = (param_1 + 0x90);
        (u_var3 + 0xa) = 0x1;
        u_var3 = (param_1 + 0x90);
        (u_var3 + 0x12) = (param_1 + 0xa);
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

pub fn pass1_1040_4766(param_1: U32Ptr) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x74);
    (**ppcVar1)();
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_477e(param_1: &mut Struct1, param_2: U32Ptr, param_3: u16, param_4: u16) {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let unaff_DI: i16;
    let pu_var5: U32Ptr;
    let u_var6: u16;
    let uVar7: u16;

    unk_win_ui_op_1040_b230(param_1, param_3, param_4);
    pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_4, param_2, unaff_DI);
    // pu_var3 = (pu_var5 >> 0x10);
    uVar7 = SUB42(ctx.data_seg, 0x0);
    u_var6 = 0x5d68;
    pu_var1 = pass1_1008_5fd8(param_4, pu_var3);
    puVar4 = pu_var3;
    pu_var2 = string_1000_3cea(CONCAT22(pu_var3, pu_var1), CONCAT22(uVar7, u_var6));
    pass1_1010_e964(puVar4, param_4, unaff_DI);
    string_1000_3cea(CONCAT22(pu_var3, pu_var1), CONCAT22(puVar4, pu_var2));
    string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x10)),
        CONCAT22(pu_var3, pu_var1),
    );
    fn_ptr_1000_17ce(ctx, CONCAT22(pu_var3, pu_var1), 0x1000);
    return;
}

pub fn pass1_1040_47fe(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    unk_draw_op_1040_b0f8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_48a0(
    param_1: &mut Struct57,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: U32Ptr,
    param_6: u16,
) {
    let i_var1: i16;
    let piVar2: U32Ptr;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let pu_var5: U32Ptr;
    let iVar5: &mut Struct444;
    let iVar6: &mut Struct445;
    let unaff_DI: i16;
    let u_var6: u16;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let piStack8: U32Ptr;

    struct_1040_b082(param_1, CONCAT22(param_4, 0xfa1));
    // u_var6 = (param_1 >> 0x10);
    iVar5 = param_1;
    iVar5.field_0x94 = 0x0;
    param_1 = ctx.PTR_LOOP_1050_4e18;
    iVar5.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_6, param_5, unaff_DI);
    // puVar4 = (puVar8 >> 0x10);
    u_var3 = puVar8;
    &iVar5.field_0x94 = u_var3;
    (&iVar5.field_0x94 + 0x2) = puVar4;
    mem_op_1000_179c(0x18, puVar4, 0x1000);
    pu_var5 = (puVar4 | u_var3);
    if (pu_var5 == 0x0) {
        iVar5.field_0x90 = 0x0;
    } else {
        struct_1040_a598(CONCAT22(puVar4, u_var3));
        &iVar5.field_0x90 = u_var3;
        (&iVar5.field_0x90 + 0x2) = pu_var5;
    }
    *iVar5.field_0x90 = 0x7;
    i_var1 = *iVar5.field_0x90;
    u_var3 = i_var1 * 0xa + 0x2;
    mem_op_1000_179c(u_var3, pu_var5, 0x1000);
    piStack8 = CONCAT22(pu_var5, u_var3);
    if ((pu_var5 | u_var3) == 0x0) {
        piVar2 = iVar5.field_0x90;
        (piVar2 + 0x2) = 0x0;
    } else {
        *piStack8 = i_var1;
        pass1_1000_5586(
            0xa564,
            &ctx.PTR_LOOP_1050_1040,
            i_var1,
            0xa,
            u_var3 + 0x2,
            pu_var5,
        );
        piVar2 = iVar5.field_0x90;
        // uVar7 = (piVar2 >> 0x10);
        iVar6 = piVar2;
        iVar6.field_0x2 = u_var3 + 0x2;
        iVar6.field_0x4 = pu_var5;
    }
    piVar2 = iVar5.field_0x90;
    (piVar2 + 0x6) = param_3;
    piVar2 = iVar5.field_0x90;
    (piVar2 + 0xa) = param_2;
    piVar2 = iVar5.field_0x90;
    (piVar2 + 0x12) = iVar5.field_0xa;
    i_var1 = &iVar5.field_0x90;
    uVar7 = (&iVar5.field_0x90 + 0x2);
    pass1_1010_debe(
        iVar5.field_0x94,
        (i_var1 + 0xa),
        CONCAT22(uVar7, i_var1 + 0x10),
        CONCAT22(uVar7, i_var1 + 0xc),
        param_3,
        param_6,
    );
    return;
}

pub fn pass1_1040_4cf4(param_1: u32, param_2: HWND16, param_3: u16) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let i_var4: i16;
    let iVar5: i16;
    let unaff_DI: i16;
    let u_var6: u16;
    let uVar7: u16;
    let LVar8: LRESULT;
    let local_52: [u8; 50];

    // u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    GetDlgItem16(param_2, 0x1770);
    LVar8 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4070000);
    // u_var3 = (LVar8 >> 0x10);
    if (LVar8 != -0x1) {
        LVar8 = SendMessage16(
            ctx.s_tile2_bmp_1050_1538,
            local_52,
            param_3,
            CONCAT22(0x408, LVar8),
        );
        // u_var3 = (LVar8 >> 0x10);
    }
    u_var2 = (i_var4 + 0x90);
    u_var1 = (i_var4 + 0x94);
    u_var3 = pass1_1010_ae12(
        u_var1,
        (u_var1 >> 0x10),
        CONCAT22(param_3, local_52),
        (u_var2 + 0xa),
        u_var3,
    );
    if (u_var3 != 0xffff) {
        u_var1 = (i_var4 + 0x90);
        // uVar7 = (u_var1 >> 0x10);
        iVar5 = u_var1;
        pass1_1010_ae92(
            (i_var4 + 0x94),
            u_var3,
            (iVar5 + 0xa),
            (iVar5 + 0x6),
            unaff_DI,
            param_3,
        );
    }
    return;
}

pub fn pass1_1040_4d7e(param_1: u32) {
    let u_var1: u32;
    let piVar2: U32Ptr;
    let u_var3: u16;
    let iStack8: i16;
    let puStack6: u32;

    // u_var3 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x90);
    puStack6 = (u_var1 + 0x2);
    iStack8 = 0x0;
    while ((
        piVar2 = (param_1 + 0x90),
        *piVar2 != iStack8 && iStack8 <= *piVar2 && ((puStack6 + 0x4) != 0x1770),
    )) {
        iStack8 += 0x1;
        puStack6 = (puStack6 & 0xffff0000 | (puStack6 + 0xa));
    }
    pass1_1000_3e2c(*puStack6);
    return;
}

pub fn pass1_1040_4dcc(param_1: u32, param_2: i16, param_3: u16) -> U32Ptr {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let mut pcVar4: String;

    // u_var3 = (param_1 >> 0x10);
    u_var2 = (param_1 + 0x90);
    u_var1 = (param_1 + 0x94);
    pcVar4 = string_op_1010_ada6(
        0x1010,
        param_3,
        u_var1,
        (u_var1 >> 0x10),
        param_2,
        (u_var2 + 0xa),
    );
    return pcVar4;
}

pub fn pass1_1040_4df2(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    unk_draw_op_1040_b0f8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_4e94(param_1: &mut Struct57, param_2: i32, param_3: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u16;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_3, 0xfab));
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x94) = 0x0;
    (i_var1 + 0x98) = 0x0;
    (i_var1 + 0xb0) = 0x0;
    (i_var1 + 0xb4) = 0x0;
    (i_var1 + 0xb6) = 0x0;
    param_1 = 0x55a2;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    if (param_2 != 0x0) {
        // u_var3 = (param_2 >> 0x10);
        (i_var1 + 0xb0) = (param_2 + 0x6);
        (i_var1 + 0xb4) = (param_2 + 0x14);
    }
    return;
}

pub fn pass1_1040_4f0a(param_1: &mut Struct18) {
    param_1.field_0x0 = 0x55a2;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    unk_draw_op_1040_b0f8(param_1);
    return;
}

pub fn pass1_1040_4f28(
    param_1: U32Ptr,
    param_2: &mut i16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
    param_6: u16,
) -> u16 {
    let ppcVar1: u32;
    let u_var2: u16;

    if (param_5 == 0x2b) {
        if (*param_2 == 0x4) {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), param_6);
        }
    } else {
        if (param_5 != 0x111) {
            u_var2 = pass1_1040_b316(param_1, param_2, param_3, param_4, param_5);
            return u_var2;
        }
        ppcVar1 = (*param_1 + 0x7c);
        (**ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
    }
    return 0x1;
}

pub fn pass1_1040_4f82(param_1: U32Ptr) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x80);
    (**ppcVar1)();
    return;
}

pub fn pass1_1040_5238(param_1: u32) -> u16 {
    let ppcVar1: u32;

    ppcVar1 = ((param_1 + 0x94) + 0x8);
    (**ppcVar1)();
    return 0x0;
}

pub fn pass1_1040_557c(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_4f0a(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_5626(param_1: &mut Struct57, param_2: u32, param_3: u16, param_4: U32Ptr) {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u32;
    let uVar7: u16;
    let piStack12: U32Ptr;
    let i_var8: &mut Struct441;
    let iVar7: &mut Struct396;
    let iVar6: &mut Struct439;

    i_var8 = param_1;
    // uVar7 = (param_1 >> 0x10);
    struct_1040_b082(param_1, CONCAT22(param_3, 0xfa3));
    u_var2 = 0x0;
    i_var8.field_0x94 = 0x0;
    i_var8.field_0x96 = 0x0;
    i_var8.field_0x98 = 0x0;
    i_var8.field_0x9c = 0x0;
    param_1 = 0x6386;
    i_var8.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    mem_op_1000_179c(0x18, param_4, 0x1000);
    pu_var3 = (param_4 | u_var2);
    if (pu_var3 == 0x0) {
        i_var8.field_0x90 = 0x0;
    } else {
        struct_1040_a598(CONCAT22(param_4, u_var2));
        &i_var8.field_0x90 = u_var2;
        (&i_var8.field_0x90 + 0x2) = pu_var3;
    }
    *i_var8.field_0x90 = 0x6;
    i_var4 = *i_var8.field_0x90;
    u_var2 = i_var4 * 0xa + 0x2;
    mem_op_1000_179c(u_var2, pu_var3, 0x1000);
    piStack12 = CONCAT22(pu_var3, u_var2);
    if ((pu_var3 | u_var2) == 0x0) {
        pi_var1 = i_var8.field_0x90;
        (pi_var1 + 0x2) = 0x0;
    } else {
        *piStack12 = i_var4;
        pass1_1000_5586(
            0xa564,
            &ctx.PTR_LOOP_1050_1040,
            i_var4,
            0xa,
            u_var2 + 0x2,
            pu_var3,
        );
        pi_var1 = i_var8.field_0x90;
        // u_var5 = (pi_var1 >> 0x10);
        i_var4 = pi_var1;
        (i_var4 + 0x2) = u_var2 + 0x2;
        (i_var4 + 0x4) = pu_var3;
    }
    pi_var1 = i_var8.field_0x90;
    (pi_var1 + 0x6) = param_2;
    pi_var1 = i_var8.field_0x90;
    (pi_var1 + 0xa) = 0x4;
    pi_var1 = i_var8.field_0x90;
    (pi_var1 + 0x12) = i_var8.field_0xa;
    u_var6 = pass1_1040_5d12(param_1);
    // u_var2 = (u_var6 >> 0x10);
    if ((u_var2 | u_var6) == 0x0) {
        i_var8.field_0x9a = 0x0;
    } else {
        i_var8.field_0x9a = (u_var6 + 0x20);
    }
    return;
}

pub fn pass1_1040_57d4(
    param_1: &mut Struct1,
    param_2: U32Ptr,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) {
    pass1_1040_5d42(param_1);
    pass1_1040_5eaa(param_1);
    pass1_1040_5dc4(param_1, param_2, param_3, param_5);
    unk_win_ui_op_1040_b230(param_1, param_4, param_5);
    return;
}

pub fn pass1_1040_5cd6(param_1: u32) -> u16 {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u32;

    u_var3 = pass1_1040_5d12(param_1);
    if (u_var3 != 0x0) {
        i_var1 = (u_var3 + 0x20);
        // u_var2 = (param_1 >> 0x10);
        if ((param_1 + 0x9a) != i_var1) {
            (param_1 + 0x9a) = i_var1;
            return 0x1;
        }
    }
    return 0x0;
}

pub fn pass1_1040_5d12(param_1: u32) -> u32 {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u32;
    let i_var4: &mut Struct440;
    let u_var4: u16;
    let u_var5: u32;

    u_var3 = (param_1 + 0x90);
    // u_var4 = (u_var3 >> 0x10);
    i_var4 = u_var3;
    u_var1 = i_var4.field_0x6;
    u_var2 = i_var4.field_0x8;
    if ((u_var2 | u_var1) != 0x0) {
        u_var5 = struct_op_1030_73a8(CONCAT22(u_var2, u_var1));
        return u_var5;
    }
    return 0x0;
}

pub fn pass1_1040_5d42(param_1: u32) {
    let u_var1: u16;
    let cVar2: u8;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u32;

    u_var5 = pass1_1040_5d12(param_1);
    if (u_var5 != 0x0) {
        u_var1 = (u_var5 + 0xc);
        i_var3 = param_1;
        // u_var4 = (param_1 >> 0x10);
        if (u_var1 == 0x5f) {
            (i_var3 + 0x96) = 0x53;
            return;
        }
        if (u_var1 < 0x60) {
            cVar2 = u_var1;
            if (cVar2 == '(') {
                (i_var3 + 0x96) = 0x54;
                return;
            }
            if (cVar2 == ')') {
                (i_var3 + 0x96) = 0x55;
                return;
            }
            if (cVar2 == ']') {
                (i_var3 + 0x96) = 0x51;
                return;
            }
            if (cVar2 == '^') {
                (i_var3 + 0x96) = 0x52;
                return;
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_5dc4(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let extraout_dx: u16;
    let iVar7: &mut Struct724;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let puVar9: u32;
    let u_var10: u16;
    let iStack18: i16;

    puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_4, param_2, param_3);
    // puVar6 = (puVar8 >> 0x10);
    u_var3 = puVar8;
    // uVar7 = (param_1 >> 0x10);
    iVar7 = param_1;
    pass1_1010_a5ca(u_var3, puVar6, iVar7.field_0x9a, u_var3, puVar6);
    if (u_var3 == 0x0) {
        iVar7.field_0x94 = 0x0;
        iVar7.field_0x9c = 0x1;
    }
    if (-0x1 < u_var3) {
        if (iVar7.field_0x9a < 0x72) {
            u_var10 = 0x31;
        } else {
            u_var10 = 0x41;
        }
        puVar9 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, u_var10, param_4, puVar6, param_3);
        u_var4 = iVar7.field_0x9a;
        ppcVar1 = (*puVar9 + 0x14);
        (**ppcVar1)(0x1010, puVar9, (puVar9 >> 0x10), u_var4, u_var4 >> 0xf);
        if ((extraout_dx | u_var4) == 0x0) {
            iStack18 = 0x0;
        } else {
            u_var2 = (u_var4 + 0x16);
            iStack18 = (u_var2 + 0x4);
        }
        if ((iStack18 != 0x0) && (u_var3 != 0x0)) {
            u_var5 = ((iStack18 - u_var3) * 0x64) / iStack18;
            u_var4 = u_var5 / 0xa;
            iVar7.field_0x94 = u_var4;
            if (0x4 < u_var5 % 0xa) {
                iVar7.field_0x94 = u_var4 + 0x1;
            }
        }
    }
    return;
}

pub fn pass1_1040_5eaa(param_1: u32) -> i16 {
    let i_var1: i16;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    i_var1 = (i_var3 + 0x9a);
    i_var2 = i_var1;
    if true {
        i_var2 = i_var3;
        match (i_var1) {
            0x0 | 0x70 | 0x71 => {
                (i_var3 + 0x98) = 0x0;
                return i_var3;
            }
            0x1 | 0x2 => {
                (i_var3 + 0x98) = 0xd;
                return i_var3;
            }
            0x3 => {
                (i_var3 + 0x98) = 0xe;
                return i_var3;
            }
            0x4 | 0x4b => {
                (i_var3 + 0x98) = 0xf;
            }
            0x5 => {
                (i_var3 + 0x98) = 0x10;
                return i_var3;
            }
            0x6 => {
                (i_var3 + 0x98) = 0x11;
                return i_var3;
            }
            0x7 => {
                (i_var3 + 0x98) = 0x12;
            }
            0x8 => {
                (i_var3 + 0x98) = 0x13;
            }
            0x9 | 0xa | 0xb => {
                (i_var3 + 0x98) = 0x14;
            }
            0xc => {
                (i_var3 + 0x98) = 0x18;
            }
            0xd => {
                (i_var3 + 0x98) = 0x19;
            }
            0xe | 0x76 => {
                (i_var3 + 0x98) = 0x17;
            }
            0xf | 0x10 | 0x11 => {
                (i_var3 + 0x98) = 0x1a;
            }
            0x12 => {
                (i_var3 + 0x98) = 0x1b;
            }
            0x13 => {
                (i_var3 + 0x98) = 0x1c;
            }
            0x14 => {
                (i_var3 + 0x98) = 0x1d;
            }
            0x15 | 0x16 | 0x17 | 0x18 | 0x19 => {
                (i_var3 + 0x98) = 0x1e;
            }
            0x1a => {
                (i_var3 + 0x98) = 0x1f;
            }
            0x1b => {
                (i_var3 + 0x98) = 0x20;
            }
            0x1c | 0x1d | 0x1e => {
                (i_var3 + 0x98) = 0x21;
            }
            0x1f => {
                (i_var3 + 0x98) = 0x22;
            }
            0x20 => {
                (i_var3 + 0x98) = 0x23;
            }
            0x21 => {
                (i_var3 + 0x98) = 0x24;
            }
            0x22 => {
                (i_var3 + 0x98) = 0x25;
            }
            0x23 | 0x24 | 0x25 | 0x26 | 0x27 | 0x28 | 0x29 | 0x2a | 0x2b => {
                (i_var3 + 0x98) = 0x26;
            }
            0x2c => {
                (i_var3 + 0x98) = 0x27;
            }
            0x2d => {
                (i_var3 + 0x98) = 0x28;
            }
            0x2e | 0x2f | 0x30 | 0x31 => {
                (i_var3 + 0x98) = 0x29;
            }
            0x32 | 0x33 | 0x34 | 0x35 | 0x4d => {
                (i_var3 + 0x98) = 0x2a;
            }
            0x36 => {
                (i_var3 + 0x98) = 0x2b;
            }
            0x37 | 0x38 | 0x39 => {
                (i_var3 + 0x98) = 0x2c;
            }
            0x3a => {
                (i_var3 + 0x98) = 0x2d;
            }
            0x3b | 0x3c => {
                (i_var3 + 0x98) = 0x2e;
            }
            0x3d => {
                (i_var3 + 0x98) = 0x2f;
            }
            0x3e => {
                (i_var3 + 0x98) = 0x30;
            }
            0x3f => {
                (i_var3 + 0x98) = 0x31;
            }
            0x40 => {
                (i_var3 + 0x98) = 0x32;
            }
            0x41 => {
                (i_var3 + 0x98) = 0x33;
            }
            0x42 => {
                (i_var3 + 0x98) = 0x34;
            }
            0x43 => {
                (i_var3 + 0x98) = 0x35;
            }
            0x44 => {
                (i_var3 + 0x98) = 0x36;
            }
            0x45 => {
                (i_var3 + 0x98) = 0x37;
            }
            0x46 => {
                (i_var3 + 0x98) = 0x38;
            }
            0x47 => {
                (i_var3 + 0x98) = 0x39;
            }
            0x48 | 0x49 | 0x4a => {
                (i_var3 + 0x98) = 0x3a;
            }
            0x4c => {
                (i_var3 + 0x98) = 0x3b;
            }
            0x4e => {
                (i_var3 + 0x98) = 0x3c;
            }
            0x4f | 0x50 => {
                (i_var3 + 0x98) = 0x3d;
            }
            0x51 | 0x52 | 0x53 | 0x54 | 0x55 => {
                (i_var3 + 0x98) = 0x3e;
            }
            0x56 | 0x57 | 0x58 | 0x59 | 0x5a => {
                (i_var3 + 0x98) = 0x3f;
            }
            0x5b => {
                (i_var3 + 0x98) = 0x40;
            }
            0x5c | 0x5d | 0x5e => {
                (i_var3 + 0x98) = 0x41;
            }
            0x5f | 0x60 | 0x61 => {
                (i_var3 + 0x98) = 0x42;
            }
            0x62 | 0x63 | 0x64 | 0x65 | 0x66 => {
                (i_var3 + 0x98) = 0x43;
            }
            0x67 | 0x68 => {
                (i_var3 + 0x98) = 0x44;
            }
            0x69 => {
                (i_var3 + 0x98) = 0x45;
            }
            0x6a => {
                (i_var3 + 0x98) = 0x46;
            }
            0x6b => {
                (i_var3 + 0x98) = 0x47;
            }
            0x6c => {
                (i_var3 + 0x98) = 0x48;
            }
            0x6d => {
                (i_var3 + 0x98) = 0x49;
            }
            0x6e => {
                (i_var3 + 0x98) = 0x4a;
            }
            0x6f => {
                (i_var3 + 0x98) = 0x4b;
            }
            0x74 => {
                (i_var3 + 0x98) = 0x15;
            }
            0x75 => {
                (i_var3 + 0x98) = 0x16;
            }
            _ => {
                (i_var3 + 0x98) = 0x4c;
            }
        }
    }
    return i_var2;
}

pub fn pass1_1040_6360(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    unk_draw_op_1040_b0f8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_6402(
    param_1: &mut Struct57,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
) {
    let ppcVar1: u32;
    let i_var2: &mut Struct725;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1850, param_2);
    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    i_var2.field_0x8e = 0x0;
    i_var2.field_0x92 = 0x0;
    param_1 = 0x67ba;
    i_var2.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    &i_var2.field_0x92 = pu_var3;
    (&i_var2.field_0x92 + 0x2) = (pu_var3 >> 0x10);
    ppcVar1 = (*i_var2.field_0x92 + 0x4);
    (**ppcVar1)();
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_6470(param_1: &mut Struct18, param_2: u16) {
    let i_var1: &mut Struct18;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    param_1.field_0x0 = 0x67ba;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    if (&i_var1.field_0x92 != 0x0) {
        pass1_1010_1ea6(&i_var1.field_0x92, param_1, param_2);
    }
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, i_var1.field_0x6);
    fn_ptr_1000_17ce(ctx, &i_var1.field_0x8e, 0x1000);
    ui_cleanup_op_1040_782c(param_1, 0x1000);
    return;
}

pub fn pass1_1040_6794(param_1: &mut Struct18, param_2: u8, param_3: u16) -> &mut Struct18 {
    pass1_1040_6470(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_6826(param_1: &mut Struct57, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_2, 0xfda));
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x94) = 0x0;
    (i_var1 + 0x98) = 0x0;
    param_1 = 0x6f32;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    return;
}

pub fn pass1_1040_6862(param_1: &mut Struct18) {
    param_1.field_0x0 = 0x6f32;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    unk_draw_op_1040_b0f8(param_1);
    return;
}

pub fn pass1_1040_68d2(
    param_1: *mut u32,
    param_2: &mut i16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
    param_6: u16,
) -> u16 {
    let ppcVar1: u32;
    let u_var2: u16;

    if (param_5 == 0x2b) {
        if (*param_2 == 0x4) {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), param_6);
        }
    } else {
        if (param_5 != 0x111) {
            u_var2 = pass1_1040_b316(param_1, param_2, param_3, param_4, param_5);
            return u_var2;
        }
        ppcVar1 = (param_1 + 0x80);
        (**ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
    }
    return 0x1;
}

pub fn pass1_1040_692e(param_1: U32Ptr) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x7c);
    (**ppcVar1)();
    return;
}

pub fn pass1_1040_6cac(param_1: u32, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pass1_1010_1ea6((i_var4 + 0x94), param_1 & 0xffff | u_var5 << 0x10, param_2);
    pu_var1 = (i_var4 + 0x98);
    u_var2 = (i_var4 + 0x9a);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(0x1010, pu_var1, u_var2, 0x1);
    }
    (i_var4 + 0x98) = 0x0;
    (i_var4 + 0x94) = 0x0;
    return;
}

pub fn pass1_1040_6cfa(param_1: u32) -> u16 {
    let ppcVar1: u32;

    ppcVar1 = ((param_1 + 0x98) + 0x8);
    (**ppcVar1)();
    return 0x1;
}

pub fn pass1_1040_6f0c(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_6862(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_6fb6(param_1: &mut Struct57, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_2, 0xfd9));
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x94) = 0x0;
    (i_var1 + 0x98) = 0x0;
    param_1 = 0x76a4;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    return;
}

pub fn pass1_1040_7044(
    param_1: *mut u32,
    param_2: &mut i16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
    param_6: u16,
) -> u16 {
    let ppcVar1: u32;
    let u_var2: u16;

    if (param_5 == 0x2b) {
        if (*param_2 == 0x4) {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), param_6);
        }
    } else {
        if (param_5 != 0x111) {
            u_var2 = pass1_1040_b316(param_1, param_2, param_3, param_4, param_5);
            return u_var2;
        }
        ppcVar1 = (param_1 + 0x80);
        (**ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
    }
    return 0x1;
}

pub fn pass1_1040_70a0(param_1: U32Ptr) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x7c);
    (**ppcVar1)();
    return;
}

pub fn pass1_1040_741e(param_1: u32, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pass1_1010_1ea6((i_var4 + 0x94), param_1 & 0xffff | u_var5 << 0x10, param_2);
    pu_var1 = (i_var4 + 0x98);
    u_var2 = (i_var4 + 0x9a);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(0x1010, pu_var1, u_var2, 0x1);
    }
    (i_var4 + 0x98) = 0x0;
    (i_var4 + 0x94) = 0x0;
    return;
}

pub fn pass1_1040_746c(param_1: u32) -> u16 {
    let ppcVar1: u32;

    ppcVar1 = ((param_1 + 0x98) + 0x8);
    (**ppcVar1)();
    return 0x1;
}

pub fn pass1_1040_767e(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    unk_draw_op_1040_b0f8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_78de() {
    return;
}

pub fn pass1_1040_79c0(
    param_1: U32Ptr,
    param_2: &mut i16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) -> u16 {
    let ppcVar1: u32;
    let cVar2: u8;
    let u_var3: u16;
    let unaff_CS: u16;

    if (param_5 == 0xa1) {
        ppcVar1 = (*param_1 + 0x38);
        u_var3 = (**ppcVar1)();
        return u_var3;
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
            if (cVar2 == '\x0c') {
                ppcVar1 = (*param_1 + 0x18);
                (**ppcVar1)();
                return 0x1;
            }
            if (cVar2 == '\x0f') {
                ppcVar1 = (*param_1 + 0x60);
                u_var3 = (**ppcVar1)();
                return u_var3;
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
            u_var3 = (**ppcVar1)();
            return u_var3;
        }
        if (param_5 < 0x115) {
            if (param_5 == 0x104) {
                ppcVar1 = (*param_1 + 0x30);
                u_var3 = (**ppcVar1)();
                return u_var3;
            }
            if (param_5 == 0x111) {
                ppcVar1 = (*param_1 + 0x10);
                u_var3 = (**ppcVar1)();
                return u_var3;
            }
        } else {
            if (param_5 == 0x115) {
                ppcVar1 = (*param_1 + 0x54);
                u_var3 = (**ppcVar1)();
                return u_var3;
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

pub fn pass1_1040_8054() -> u16 {
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_805a(param_1: U32Ptr) -> u32 {
    let unaff_DI: i16;
    let u_var1: u16;
    let unaff_SS: u16;

    if (ctx.PTR_LOOP_1050_4230 == 0x0) {
        mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x28, unaff_SS, param_1, unaff_DI);
    }
    u_var1 = (ctx.PTR_LOOP_1050_4230 >> 0x10);
    return CONCAT22(
        (ctx.PTR_LOOP_1050_4230 + 0x10),
        (ctx.PTR_LOOP_1050_4230 + 0xe),
    );
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_807e(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u16;
    let ppcVar2: u32;
    let pu_var3: u32;
    let puVar4: u32;
    let in_DX: U32Ptr;
    let u_var5: u16;
    let extraout_dx: U32Ptr;
    let puVar6: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let puVar7: U32Ptr;
    let i_var9: &mut Struct395;
    let uVar8: u16;
    let paVar9: &mut Struct43;
    let uStack10: u32;
    let i_var8: &mut Struct393;

    if (param_2 == 0x1) {
        pass1_1040_805a(in_DX);
        return;
    }
    paVar9 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, param_2, param_3);
    // u_var5 = (paVar9 >> 0x10);
    pu_var3 = paVar9;
    if ((u_var5 | pu_var3) != 0x0) {
        ppcVar2 = (paVar9 + 0x14);
        puVar4 = pu_var3;
        (**ppcVar2)(0x1010, pu_var3, u_var5);
        uStack10 = CONCAT22(extraout_dx, puVar4);
        // uVar8 = (param_1 >> 0x10);
        i_var9 = param_1;
        puVar6 = extraout_dx;
        if (i_var9.field_0x70 != 0x0) {
            puVar4 = &i_var9.field_0x70;
            u_var1 = (&i_var9.field_0x70 + 0x2);
            puVar6 = (u_var1 | puVar4);
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
        &i_var9.field_0x70 = puVar4;
        (&i_var9.field_0x70 + 0x2) = puVar7;
        pass1_1008_4d84(i_var9.field_0x70, uStack10, puVar7);
        if (paVar9 != 0x0) {
            ppcVar2 = paVar9;
            (**ppcVar2)(0x1008, pu_var3, u_var5, 0x1);
        }
        return;
    }
    return;
}

pub fn pass1_1040_824a(param_1: u32, param_2: i16) -> u16 {
    if ((param_1 + 0x6) != param_2) {
        return 0x1;
    }
    return 0x0;
}

pub fn pass1_1040_83e6(param_1: &mut Struct18, param_2: u8, param_3: u16) -> &mut Struct18 {
    ui_cleanup_op_1040_782c(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_8478(
    param_1: &mut Struct57,
    param_2: u16,
    param_3: &mut String,
    param_4: &mut String,
    param_5: u16,
    param_6: u16,
) -> &mut Struct57 {
    let u_var1: u16;
    let i_var2: &mut Struct712;
    let u_var2: u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfc3, param_5);
    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    i_var2.field_0x8e = 0x0;
    i_var2.field_0x98 = param_2;
    i_var2.field_0x9a = 0x0;
    i_var2.field_0xb2 = 0x0;
    param_1 = 0x8ddc;
    i_var2.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    i_var2.field_0x9e = 0x0;
    i_var2.field_0xa2 = 0x12c;
    u_var1 = str_op_1008_60e8(param_4, param_6);
    i_var2.field_0x90 = u_var1;
    i_var2.field_0x92 = param_6;
    u_var1 = str_op_1008_60e8(param_3, param_6);
    i_var2.field_0x94 = u_var1;
    i_var2.field_0x96 = param_6;
    load_icon_1040_8b92(param_1, 0x1008);
    ctx.PTR_LOOP_1050_5df8 = 0x0;
    return param_1;
}

pub fn pass1_1040_869a(param_1: &mut Struct18) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    param_1.field_0x0 = 0x8ddc;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    fn_ptr_1000_17ce(ctx, (i_var1 + 0x90), 0x1000);
    fn_ptr_1000_17ce(ctx, (i_var1 + 0x94), 0x1000);
    ui_cleanup_op_1040_782c(param_1, 0x1000);
    return;
}

pub fn pass1_1040_8978(
    param_1: U32Ptr,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: &WNDCLASS16,
) {
    let ppcVar1: u32;

    unk_win_msg_op_1008_9510(&ctx.PTR_LOOP_1050_5df4, 0x1008, param_5);
    win_1008_5c5c(param_5, param_3, param_4, _PTR_LOOP_1050_02a0, param_2);
    ppcVar1 = (*param_1 + 0x74);
    (**ppcVar1)(0x1008, param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_89a4(
    param_1: U32Ptr,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: i16,
    param_5: &WNDCLASS16,
) {
    let u_var1: u16;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let puVar7: U32Ptr;

    unk_win_msg_op_1008_9510(&ctx.PTR_LOOP_1050_5df4, 0x1008, param_5);
    u_var1 = (param_2 + 0x2);
    u_var2 = *param_2;
    u_var6 = 0x1010;
    puVar7 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_5, param_3, param_4);
    // u_var5 = (puVar7 >> 0x10);
    u_var4 = puVar7;
    if ((u_var4 + 0x72) != 0x0) {
        u_var6 = 0x1008;
        win_1008_5c7c(
            ctx.PTR_LOOP_1050_02a0,
            CONCAT22(u_var1, u_var2),
            param_5,
            u_var4,
            u_var5,
        );
        (param_1 + 0x8c) = u_var4;
    }
    ppc_var3 = (*param_1 + 0x74);
    (**ppc_var3)(u_var6, param_1);
    return;
}

pub fn pass1_1040_8b3c(param_1: u16, param_2: u32, param_3: u32, param_4: u16) {
    if ((param_3._2_2_ != 0x0)
        && (param_3._2_2_ == (&ctx.PTR_LOOP_1050_0000 + 0x1)
            || param_3._2_2_ == &ctx.PTR_LOOP_1050_0002
            || ((&ctx.PTR_LOOP_1050_0002 + 0x1) < param_3._2_2_ + -0x2
                && (param_3._2_2_ + -0x6 < &ctx.PTR_LOOP_1050_0002))))
    {
        ctx.PTR_LOOP_1050_5df4 = 0x0;
        ctx.PTR_LOOP_1050_5df8 = param_3._2_2_;
        return;
    }
    post_win_msg_1040_7b3c(
        CONCAT22(param_2, param_1),
        (param_2 >> 0x10),
        param_3,
        param_3._2_2_,
        param_4,
    );
    return;
}

pub fn pass1_1040_8db6(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_869a(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_8e58(param_1: i16, param_2: u16, param_3: u16, param_4: u32) -> u16 {
    pass1_1040_b040(
        CONCAT22(param_2, param_1),
        CONCAT22(param_4, param_3),
        (param_4 >> 0x10),
    );
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

pub fn pass1_1040_8f16(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_8e82(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_9252(ctx: &mut AppContext, param_1: &mut Struct161, param_2: u16) {
    let pi_var1: U32Ptr;
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

pub fn pass1_1040_9422(param_1: U32Ptr) {
    let ppcVar1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
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

pub fn pass1_1040_97da(param_1: U32Ptr, param_2: u8) -> u16 {
    mix_win_ui_op_1040_911e(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_9824(param_1: U32Ptr) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x0;
    (i_var1 + 0x4) = 0x0;
    (i_var1 + 0x56) = 0x0;
    (i_var1 + 0x5a) = 0x0;
    (i_var1 + 0x5c) = 0x0;
    *(i_var1 + 0x6) = 0x0;
    return;
}

pub fn pass1_1040_a204(param_1: U32Ptr, param_2: u8) -> u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_a2cc(
    param_1: i16,
    param_2: u32,
    param_3: u32,
    param_4: u16,
    param_5: *mut u8,
    param_6: u16,
    param_7: u16,
) -> u32 {
    let u_var1: u16;

    if (param_3._2_2_ == 0x1826) {
        if ((param_3 == 0x1) || (0x1 < param_3 - 0x1 & &(param_3 - 0x3 < 0x2))) {
            u_var1 = 0x1;
        } else {
            u_var1 = 0x0;
        }
        return u_var1;
    }
    pass1_1040_b54a(
        param_1,
        param_2,
        (param_2 >> 0x10),
        param_3,
        param_5,
        param_6,
        param_7,
    );
    return CONCAT22(param_5, param_4);
}

pub fn pass1_1040_a4c2(param_1: &mut Struct18, param_2: u8, param_3: u16) -> &mut Struct18 {
    free_proc_inst_1040_a294(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_a564(param_1: U32Ptr) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x0;
    (param_1 + 0x4) = 0x0;
    (param_1 + 0x6) = 0x0;
    return;
}

pub fn pass1_1040_a582(param_1: U32Ptr) {
    fn_ptr_1000_17ce(ctx, *param_1, 0x1000);
    return;
}

pub fn pass1_1040_a5d0(param_1: u32) {
    let u_var1: u16;
    let u_var2: u16;
    let i_var4: &mut Struct258;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var1 = i_var4.field_0x2;
    u_var2 = i_var4.field_0x4;
    if ((u_var2 | u_var1) != 0x0) {
        pass1_1000_54e8(
            0xa582,
            &ctx.PTR_LOOP_1050_1040,
            (u_var1 - 0x2),
            0xa,
            u_var1,
            u_var2,
        );
        fn_ptr_1000_17ce(ctx, CONCAT22(u_var2, u_var1 - 0x2), 0x1000);
    }
    fn_ptr_1000_17ce(ctx, i_var4.field_0xc, 0x1000);
    return;
}

pub fn pass1_1040_a640(param_1: &mut Struct57, param_2: u32, param_3: u16) {
    let i_var1: i16;
    let u_var2: u16;

    struct_1040_b082(param_1, CONCAT22(param_3, 0x1f1));
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x94) = param_2;
    (i_var1 + 0x98) = 0x0;
    (i_var1 + 0xea) = 0x0;
    param_1 = 0xac08;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    return;
}

pub fn pass1_1040_a84a(param_1: u32, param_2: u16) {
    win_ui_dlg_op_1040_a94a(ctx, param_1, param_2, 0, 0, 0);
    return;
}

pub fn pass1_1040_abe2(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    unk_draw_op_1040_b0f8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_ac84(
    param_1: &mut Struct57,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
) {
    let i_var1: &mut Struct726;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x1f3));
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x94 = 0x0;
    &i_var1.field_0x98 = 0x0;
    param_1 = 0xafc4;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    i_var1.field_0x94 = ctx._PTR_LOOP_1050_5ef0;
    ctx._PTR_LOOP_1050_5ef0 = 0x0;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3d, param_5, param_3, param_4);
    i_var1.field_0x98 = pu_var2;
    i_var1.field_0x9a = (pu_var2 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_ace8(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xafc4;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    unk_draw_op_1040_b0f8(param_1);
    return;
}

pub fn pass1_1040_ad14(param_1: u32, param_2: u16) {
    win_ui_op_1040_ae04(ctx, param_1, param_2, 0, 0, 0);
    return;
}

pub fn pass1_1040_ad24(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: u16,
    param_7: u16,
) {
    if (param_4._2_2_ == 0xeb) {
        win_ui_op_1040_ae04(ctx, CONCAT22(param_2, param_1), param_7, 0, 0, 0);
    } else {
        if (param_4._2_2_ != 0x1f0) {
            pass1_1040_b54a(
                param_1, param_2, param_3, param_4, param_5, param_6, param_7,
            );
            return;
        }
        msg_box_ui_op_1040_ad66(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
    }
    return;
}

pub fn pass1_1040_af9e(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_ace8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_b040(param_1: &mut Struct57, param_2: u32, param_3: u16) {
    let i_var1: i16;
    let u_var2: u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, (param_2 + 0x12), param_3);
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x8e) = 0x0;
    (i_var1 + 0x90) = param_2;
    param_1 = 0xb772;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    return;
}

pub fn pass1_1040_b0bc(param_1: &mut Struct57, param_2: u32, param_3: u32) {
    let i_var1: i16;
    let u_var2: u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, param_3, (param_3 >> 0x10));
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x8e) = 0x0;
    (i_var1 + 0x90) = param_2;
    param_1 = 0xb772;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    return;
}

pub fn pass1_1040_b17c(
    param_1: u32,
    param_2: u32,
    param_3: U32Ptr,
    param_4: i16,
    param_5: i16,
    param_6: u16,
) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let mut pcVar3: String;
    let u_var4: u16;
    let iVar5: i16;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let puVar8: U32Ptr;
    let puStack12: U32Ptr;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        // u_var6 = (param_1 >> 0x10);
        iVar5 = param_1;
        pi_var1 = (iVar5 + 0x90);
        // puVar7 = (pi_var1 >> 0x10);
        if (*pi_var1 == i_stack4 || *pi_var1 < i_stack4) {
            break;
        }
        param_5 = (i_stack4 * 0x2 + param_2);
        u_var2 = (pi_var1 + 0x2);
        (i_stack4 * 0xa + u_var2 + 0x4) = param_5;
        i_stack4 += 0x1;
        param_3 = puVar7;
    }
    puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_6, param_3, param_5);
    // u_var4 = (puVar8 >> 0x10);
    u_var2 = (iVar5 + 0x90);
    puStack12 = (u_var2 + 0x2);
    // for (i_stack4 = 0x0; pi_var1 = (iVar5 + 0x90),
    //     *pi_var1 != i_stack4 && i_stack4 <= *pi_var1; i_stack4 += 0x1) {
    //   u_var2 = (iVar5 + 0x90);
    //   u_var2 = (u_var2 + 0x6);
    //   pcVar3 = pass1_1010_b038(puVar8,u_var2,(u_var2 >> 0x10),
    //                            (puStack12 + 0x4),param_4);
    //   string_1040_a626(puStack12,CONCAT22(u_var4,pcVar3),u_var4);
    //   puStack12 = (puStack12 & 0xffff0000 | (puStack12 + 0xa));
    // }
    return;
}

pub fn pass1_1040_b316(
    param_1: U32Ptr,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
) -> u16 {
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
    let u_var1: u32;
    let piVar2: U32Ptr;
    let i_var3: i16;
    let u_var4: u16;
    let iStack8: i16;
    INT16 * pi_stack6;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x90) != 0x0) {
        u_var1 = (i_var3 + 0x90);
        (u_var1 + 0x14) = (i_var3 + 0x6);
        u_var1 = (i_var3 + 0x90);
        pi_stack6 = (u_var1 + 0x2);
        // for (iStack8 = 0x0; piVar2 = (i_var3 + 0x90),
        //     *piVar2 != iStack8 && iStack8 <= *piVar2; iStack8 += 0x1) {
        //   SetDlgItemText16(param_2,*pi_stack6,(pi_stack6 + 0x2));
        //   pi_stack6 = (pi_stack6 & 0xffff0000 | (pi_stack6 + 0xa));
        //   param_2 = ctx.s_tile2_bmp_1050_1538;
        // }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_b4c8(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let i_var1: i16;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let pu_var5: U32Ptr;

    // u_var4 = (param_1 >> 0x10);
    if ((param_1 + 0x90) != 0x0) {
        pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x32, param_4, param_2, param_3);
        u_var2 = (param_1 + 0x90);
        i_var1 = (u_var2 + 0xa);
        i_var3 = i_var1 + -0x4;
        if (i_var3 == 0x0) {
            ui_op_1010_79aa(pu_var5, 0xfd9, 0x0, param_4);
            if (i_var3 == 0x0) {
                u_var4 = 0xe;
                //LAB_1040_b50f:
                unk_win_op_1010_7300(
                    pu_var5,
                    CONCAT22(i_var3, i_var3),
                    u_var4,
                    CONCAT22(i_var3, i_var3),
                );
                return;
            }
        } else {
            if (((0x0 < i_var1 + -0x5) && (!SBORROW2(i_var1 + -0x5, 0x1)))
                && (i_var3 = i_var1 + -0x7, i_var3 == 0x0 || i_var1 + -0x6 < 0x1))
            {
                ui_op_1010_79aa(pu_var5, 0xfda, 0x0, param_4);
                if (i_var3 == 0x0) {
                    u_var4 = 0xd;
                    //           TODO: goto LAB_1040_b50f;
                }
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_b54a(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: u16,
    param_7: u16,
) {
    let paVar1: &mut Struct18;
    let ppcVar2: u32;
    let u_var3: u32;
    let i_var4: i16;
    let iVar5: i16;
    let paVar6: &mut Struct18;
    let uVar7: u16;
    let iVar6: &mut Struct515;
    let unaff_DI: i16;
    let uVar8: u16;
    let puVar9: U32Ptr;
    let u_var10: u8;
    let u_var11: u8;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;

    if (param_4._2_2_ == 0xea) {
        ppcVar2 = (CONCAT22(param_2, param_1) + 0x5c);
        (**ppcVar2)(param_6, param_1, param_2);
    } else {
        if (param_4._2_2_ == 0xeb) {
            puVar9 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_7, param_5, unaff_DI);
            // uVar7 = (puVar9 >> 0x10);
            paVar1 = (param_1 + 0x90);
            if (paVar1 != 0x0) {
                // uVar8 = (paVar1 >> 0x10);
                uVar12 = 0x1010;
                paVar6 = paVar1;
                pass1_1010_ad64(
                    puVar9,
                    CONCAT22((paVar1 + 0xa), uVar7),
                    (paVar1 + 0x6),
                    paVar1,
                    uVar7,
                );
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
                puVar9 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x32, param_7, param_5, unaff_DI);
                u_var3 = (param_1 + 0x90);
                u_var3 = (u_var3 + 0x6);
                i_var4 =
                    pass1_1010_7d38(puVar9, (puVar9 >> 0x10), u_var3, (u_var3 >> 0x10), param_7);
                iVar5 = i_var4;
                ui_op_1010_79aa(puVar9, 0xfab, 0x0, param_7);
                if (iVar5 != 0x0) {
                    return;
                }
                if (i_var4 == 0x0) {
                    u_var3 = (param_1 + 0x90);
                    // uVar8 = (u_var3 >> 0x10);
                    iVar6 = u_var3;
                    u_var3 = iVar6.field_0x6;
                    uVar13 = u_var3;
                    // uVar14 = (u_var3 >> 0x10);
                    uVar12 = 0x14;
                } else {
                    u_var3 = (param_1 + 0x90);
                    // uVar8 = (u_var3 >> 0x10);
                    iVar6 = u_var3;
                    u_var3 = iVar6.field_0x6;
                    uVar13 = u_var3;
                    // uVar14 = (u_var3 >> 0x10);
                    uVar12 = 0x9;
                }
                u_var10 = uVar8;
                u_var11 = (uVar8 >> 0x8);
            } else {
                if (param_4._2_2_ == 0x1824) {
                    puVar9 =
                        mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x32, param_7, param_5, unaff_DI);
                    iVar6 = puVar9;
                    u_var3 = (param_1 + 0x90);
                    ui_op_1010_79aa(puVar9, 0xfc5, (u_var3 + 0x6), param_7);
                    if (iVar6 != 0x0) {
                        return;
                    }
                    u_var3 = (param_1 + 0x90);
                    u_var3 = (u_var3 + 0x6);
                    uVar13 = u_var3;
                    // uVar14 = (u_var3 >> 0x10);
                    uVar12 = 0x12;
                    u_var10 = 0x0;
                    u_var11 = 0x0;
                } else {
                    if (param_4._2_2_ != 0x1830) {
                        post_win_msg_1040_7b3c(
                            CONCAT13((param_2 >> 0x8), CONCAT12(param_2, param_1)),
                            param_3,
                            param_4,
                            param_4._2_2_,
                            param_6,
                        );
                        return;
                    }
                    puVar9 =
                        mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x32, param_7, param_5, unaff_DI);
                    iVar6 = puVar9;
                    u_var3 = (param_1 + 0x90);
                    ui_op_1010_79aa(puVar9, 0xfb6, (u_var3 + 0x6), param_7);
                    if (iVar6 != 0x0) {
                        return;
                    }
                    u_var3 = (param_1 + 0x90);
                    u_var3 = (u_var3 + 0x6);
                    uVar13 = u_var3;
                    // uVar14 = (u_var3 >> 0x10);
                    uVar12 = 0xc;
                    u_var10 = 0x0;
                    u_var11 = 0x0;
                }
            }
            unk_win_op_1010_7300(
                puVar9,
                CONCAT13(u_var11, CONCAT12(u_var10, iVar6)),
                uVar12,
                CONCAT22(uVar14, uVar13),
            );
        }
    }
    return;
}

pub fn pass1_1040_b74c(param_1: &mut Struct18, param_2: u8) -> u32 {
    unk_draw_op_1040_b0f8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_b7ee(param_1: &mut Struct57, param_2: i32, param_3: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u16;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_3, 0xfab));
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x94) = 0x0;
    (i_var1 + 0x98) = 0x0;
    (i_var1 + 0xb0) = 0x0;
    (i_var1 + 0xb4) = 0x0;
    (i_var1 + 0xb6) = 0x0;
    param_1 = 0xbeba;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    if (param_2 != 0x0) {
        // u_var3 = (param_2 >> 0x10);
        (i_var1 + 0xb0) = (param_2 + 0x6);
        (i_var1 + 0xb4) = (param_2 + 0x14);
    }
    return;
}

pub fn pass1_1040_b864(
    param_1: *mut u32,
    param_2: &mut i16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
    param_6: u16,
) -> u16 {
    let ppcVar1: u32;
    let u_var2: u16;

    if (param_5 == 0x2b) {
        if (*param_2 == 0x4) {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), param_6);
        }
    } else {
        if (param_5 != 0x111) {
            u_var2 = pass1_1040_b316(param_1, param_2, param_3, param_4, param_5);
            return u_var2;
        }
        ppcVar1 = (param_1 + 0x7c);
        (**ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
    }
    return 0x1;
}

pub fn pass1_1040_b8be(param_1: U32Ptr) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x80);
    (**ppcVar1)();
    return;
}

pub fn pass1_1040_bb5a(param_1: u32) -> u16 {
    let ppcVar1: u32;

    ppcVar1 = ((param_1 + 0x94) + 0x8);
    (**ppcVar1)();
    return 0x0;
}

pub fn pass1_1040_be94(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    unk_draw_op_1040_b0f8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn struct_1040_bf3e(param_1: U32Ptr, param_2: u16) -> u16 {
    let i_var1: &mut Struct442;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    *param_1 = 0x3aa8;
    i_var1.field_0x2 = 0x1008;
    i_var1.field_0x4 = param_2;
    *param_1 = 0x3ab0;
    i_var1.field_0x2 = 0x1008;
    i_var1.field_0x6 = 0x0;
    *param_1 = 0xc53e;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    return param_1;
}

pub fn pass1_1040_bf92(param_1: U32Ptr, param_2: u16) {
    let i_var1: &mut Struct514;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0xc53e;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pass1_1010_1ea6(i_var1.field_0x6, param_1 & 0xffff | u_var1 << 0x10, param_2);
    unk_destroy_win_op_1010_2fa0(i_var1.field_0x6, 0x1010);
    *param_1 = 0x3ab0;
    i_var1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    return;
}

pub fn pass1_1040_bfde(param_1: u32, param_2: U32Ptr, param_3: u16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0x6) = param_2;
    ppcVar1 = (*param_2 + 0x4);
    (**ppcVar1)();
    u_var2 = (i_var3 + 0x6);
    (u_var2 + 0x22) = (i_var3 + 0x4);
    pass1_1010_2ee2((i_var3 + 0x6), param_3, 0x1010);
    return;
}

pub fn pass1_1040_c518(param_1: u32, param_2: u8, param_3: u16) -> u32 {
    pass1_1040_bf92(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1040_c54a(param_1: U32Ptr, param_2: u16, param_3: U32Ptr, param_4: u16, param_5: u16) {
    let ppcVar1: u32;
    let i_var3: i16;
    let extraout_dx: u16;
    let i_var2: &mut Struct164;
    let u_var4: u16;
    let pu_var5: u32;
    let u_var6: u16;
    let uVar7: u32;

    i_var3 = (param_3 + 0x12) + 0xc8;
    uVar7 = 0x0;
    u_var6 = 0x0;
    ppcVar1 = (*param_3 + 0x14);
    pu_var5 = param_3;
    (**ppcVar1)();
    mixed_struct_op_1040_8fb8(
        param_1,
        0x0,
        CONCAT22(extraout_dx, i_var3),
        pu_var5,
        (pu_var5 >> 0x10),
        u_var6,
        uVar7,
        (uVar7 >> 0x10),
        extraout_dx,
        param_4,
        param_5,
    );
    // u_var4 = (param_1 >> 0x10);
    i_var2 = param_1;
    i_var2.field_0x42 = param_3;
    i_var2.field_0x46 = 0x0;
    i_var2.field_0x48 = param_2;
    *param_1 = 0xc9f2;
    i_var2.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pass1_1040_c630((param_1 & 0xffff | u_var4 << 0x10), param_4, param_5);
    return;
}

pub fn pass1_1040_c5ac(param_1: U32Ptr) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0xc9f2;
    (i_var4 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    ctx.PTR_LOOP_1050_5f02 = ctx.PTR_LOOP_1050_5f02 + -0x1;
    if (ctx.PTR_LOOP_1050_5f02 == 0x0) {
        pu_var1 = (i_var4 + 0x8);
        u_var2 = (i_var4 + 0xa);
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
        pu_var1 = (i_var4 + 0xc);
        u_var2 = (i_var4 + 0xe);
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
    }
    mix_win_ui_op_1040_911e(param_1);
    return;
}

pub fn pass1_1040_c60e(param_1: &mut Struct65) -> u16 {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x42) != 0x0) {
        u_var1 = (param_1 + 0x42);
        return (u_var1 + 0x12);
    }
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_c630(param_1: U32Ptr, param_2: u16, param_3: u16) {
    let i_var1: i16;
    let ppcVar2: u32;
    let u_var3: u32;
    let u_var4: u32;
    let i_var4: &mut Struct165;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var3 = i_var4.field_0x42;
    if ((u_var3 + 0x12) != 0x71) {
        i_var4.field_0x36 = 0x5;
        i_var4.field_0x26 = 0x5;
        i_var4.field_0x28 = 0x5;
        i_var1 = i_var4.field_0x36;
        i_var4.field_0x30 = i_var1;
        i_var4.field_0x2e = i_var1;
        if (ctx.PTR_LOOP_1050_5f02 == 0x0) {
            ctx._PTR_LOOP_1050_5f04 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0xff, param_3);
            param_2 = 0x1010;
            ctx._PTR_LOOP_1050_5f08 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x100, param_3);
        }
        ctx.PTR_LOOP_1050_5f02 = ctx.PTR_LOOP_1050_5f02 + 0x1;
        i_var4.field_0x8 = ctx._PTR_LOOP_1050_5f04;
        i_var4.field_0xc = ctx._PTR_LOOP_1050_5f08;
        pass1_1040_9618(ctx, param_1);
        i_var4.field_0x20 = 0x0;
        i_var4.field_0x1e = 0xc8;
        i_var4.field_0x22 = 0xa0;
        i_var4.field_0x24 = i_var4.field_0x2c + i_var4.field_0x36;
        i_var4.field_0x2e = i_var4.field_0x36 * 0x3 + i_var4.field_0x2a;
        i_var4.field_0x30 = i_var4.field_0x36;
        i_var4.field_0x32 = i_var4.field_0x22 - i_var4.field_0x36;
        i_var4.field_0x3c = 0x25;
        u_var4 = *param_1;
        ppcVar2 = (u_var4 + 0x4);
        (**ppcVar2)(param_2, param_1);
        ppcVar2 = (u_var4 + 0x8);
        (**ppcVar2)(param_2, param_1, u_var5);
    }
    return;
}

pub fn pass1_1040_c71e(param_1: u32, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;

    pass1_1040_9252(ctx, param_1, param_2);
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x28) = (i_var1 + 0x24) / 0x2 - (i_var1 + 0x2c) / 0x2;
    return;
}

pub fn pass1_1040_c94a(
    param_1: i16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
    param_6: u16,
) {
    let u_var1: u16;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let pu_var5: U32Ptr;

    if ((param_1 + 0x48) != 0x0) {
        pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_6, param_3, param_4);
        // u_var4 = (pu_var5 >> 0x10);
        u_var2 = (param_1 + 0x42);
        u_var1 = (u_var2 + 0x12);
        param_5 = 0x1010;
        u_var3 = u_var1;
        pass1_1010_a5ca(pu_var5, u_var4, u_var1, u_var1, u_var4);
        if (u_var3 == 0xffff) {
            (param_1 + 0x3c) = 0xf9;
        } else {
            if (u_var3 == 0x0) {
                (param_1 + 0x3c) = 0x25;
                if ((u_var1 == 0x5b) || (u_var1 == 0x9)) {
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

pub fn pass1_1040_c9cc(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1040_c5ac(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_ca16(
    param_1: &mut Struct57,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
) {
    let i_var1: &mut Struct727;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x1840));
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x94 = ctx._PTR_LOOP_1050_5f0c;
    &i_var1.field_0x98 = 0x0;
    i_var1.field_0x9c = 0x0;
    i_var1.field_0x9e = 0x0;
    param_1 = 0xd07c;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3e, param_5, param_3, param_4);
    i_var1.field_0x98 = pu_var2;
    i_var1.field_0x9a = (pu_var2 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_ca74(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xd07c;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ctx.PTR_LOOP_1050_5f10 = 0x0;
    unk_draw_op_1040_b0f8(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_caa6(param_1: u16, param_2: u32, param_3: U32Ptr, param_4: i16, param_5: u16) {
    let pu_var1: U32Ptr;
    let i_var2: i16;

    i_var2 = 0x0;
    pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    pass1_1010_038e(pu_var1, i_var2, param_5);
    destroy_window_1040_b726(CONCAT22(param_2, param_1), (param_2 >> 0x10), 0x1010);
    return;
}

pub fn pass1_1040_cc8c(
    ctx: &mut AppContext,
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: u16,
    param_7: u16,
) {
    if param_4 == 0xeb {
        send_dlg_msg_1040_cf1c(CONCAT22(param_2, param_1), param_7);
    } else {
        if param_4 == (ctx.s_vrpal_bmp_1050_183a + 0x7) {
            msg_box_op_1040_cce4(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
        } else {
            if param_4 != (ctx.s_vrpal_bmp_1050_183a + 0x8) {
                pass1_1040_b54a(
                    param_1, param_2, param_3, param_4, param_5, param_6, param_7,
                );
                return;
            }
            if param_4 == 0x1 {
                send_dlg_item_1040_ce76(CONCAT22(param_2, param_1), param_6, param_7);
            }
        }
    }
    return;
}

pub fn pass1_1040_cdac(
    param_1: u32,
    param_2: u16,
    param_3: u16,
    param_4: i16,
    param_5: HWND16,
) -> u16 {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let bVar3: bool;
    let i_var4: i16;
    let u_var5: u16;

    bVar3 = false;
    i_var4 = param_1;
    // u_var5 = (param_1 >> 0x10);
    if (param_4 == 0x0) {
        i_var2 = (i_var4 + 0x9e);
        pi_var1 = (i_var4 + 0x9c);
        if (*pi_var1 == i_var2 || *pi_var1 < i_var2) {
            // goto
            // LAB_1040_cdef;
        }
        pi_var1 = (i_var4 + 0x9e);
        *pi_var1 = *pi_var1 + 0x1;
    } else {
        if (param_4 != 0x1) {
            // goto
            // LAB_1040_cdef;
        }
        if ((i_var4 + 0x9e) < 0x1) {
            // goto
            // LAB_1040_cdef;
        }
        pi_var1 = (i_var4 + 0x9e);
        *pi_var1 = *pi_var1 + -0x1;
    }
    bVar3 = true;
    //LAB_1040_cdef:
    if (bVar3) {
        SetDlgItemInt16(param_5, 0x0, (i_var4 + 0x9e), 0x18e);
    }
    return 0x0;
}

pub fn pass1_1040_d056(param_1: &mut Struct18, param_2: u8) -> &mut Struct18 {
    pass1_1040_ca74(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_d0f8(param_1: &mut Struct57, param_2: u16) {
    let u_var1: u16;
    let in_DX: U32Ptr;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let iVar5: &mut Struct438;
    let unaff_DI: i16;
    let u_var5: u16;
    let unaff_SS: u16;
    let puVar6: U32Ptr;
    let uVar7: u32;
    let i_var8: &mut Struct392;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x1845));
    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    &iVar5.field_0x94 = 0x0;
    iVar5.field_0x98 = ctx._PTR_LOOP_1050_5f16;
    &iVar5.field_0x9c = 0x0;
    iVar5.field_0xa0 = 0x0;
    param_1 = 0xd8c4;
    iVar5.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x47, unaff_SS, in_DX, unaff_DI);
    // u_var2 = (puVar6 >> 0x10);
    iVar5.field_0x94 = puVar6;
    iVar5.field_0x96 = u_var2;
    uVar7 = pass1_1018_5732(
        iVar5.field_0x94,
        u_var2,
        iVar5.field_0x98,
        puVar6,
        u_var2,
        unaff_SS,
    );
    // pu_var3 = (uVar7 >> 0x10);
    iVar5.field_0x9c = uVar7;
    iVar5.field_0x9e = pu_var3;
    u_var1 = pu_var3 | iVar5.field_0x9c;
    if (u_var1 == 0x0) {
        mem_op_1000_179c(0xc, pu_var3, 0x1000);
        puVar4 = (pu_var3 | u_var1);
        if (puVar4 == 0x0) {
            &iVar5.field_0x9c = 0x0;
        } else {
            pass1_1010_8ef2(CONCAT22(pu_var3, u_var1), puVar4, unaff_SS);
            iVar5.field_0x9c = u_var1;
            iVar5.field_0x9e = puVar4;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_d1bc(param_1: *mut astruct_18) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct513;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    param_1.field_0x0 = 0xd8c4;
    i_var4.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, i_var4.field_0x6);
    pu_var1 = i_var4.field_0x9c;
    u_var2 = i_var4.field_0x9e;
    if (u_var2 | pu_var1) != 0x0 {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(&ctx.PTR_LOOP_1050_1038, pu_var1, u_var2, 0x1);
    }
    unk_draw_op_1040_b0f8(param_1);
    return;
}

pub fn pass1_1040_d29c(param_1: u32, param_2: u16) {
    send_ldg_item_msg_1040_d79c(param_1, param_2);
    return;
}

pub fn pass1_1040_d76e(param_1: u32) {
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    u_var1 = (i_var2 + 0x94);
    pass1_1018_5742(u_var1, (u_var1 >> 0x10), (i_var2 + 0x9c), (i_var2 + 0x98));
    (i_var2 + 0x9c) = 0x0;
    return;
}

pub fn pass1_1040_d89e(param_1: *mut astruct_18, param_2: u8) -> *mut astruct_18 {
    pass1_1040_d1bc(param_1);
    if (param_2 & 0x1) != 0x0 {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}
