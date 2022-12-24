use crate::unk::block_1000_1000::fn_ptr_1000_17ce;
use crate::unk::block_1010_0000::pass1_1010_038e;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_a000::pass1_1010_a5ca;
use crate::unk::block_1018_5000::pass1_1018_5742;
use crate::draw_ops::draw_e::unk_draw_op_1040_b0f8;
use crate::draw_ops::draw_a::draw_text_1040_94fc;
use crate::draw_ops::draw_c::{draw_line_1040_c302, draw_op_1040_c38e};
use crate::unk::block_1040_b000::{pass1_1040_b316, pass1_1040_b54a, pass1_1040_bfde, struct_1040_bf3e};
use crate::unk::block_1040_c000::{pass1_1040_c5ac, pass1_1040_ca74};
use crate::unk::block_1040_d000::pass1_1040_d1bc;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_37::Struct37;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_d::StructD;
use crate::utils::CONCAT22;
use crate::gui;
use crate::gui::{dialog, msg_box};
use crate::gui::cleanup::destroy_window_1040_b726;
use crate::windef16::{HDC16, LRESULT};

pub unsafe fn pass1_1040_b864(param_1: u32, param_2: *mut i16, mut param_3: u16, mut param_4: u16, mut param_5: i16) -> u16

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    if (param_5 == 0x2b) {
        if (*param_2 == 0x4) {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2));
        }
    } else {
        if (param_5 != 0x111) {
            uVar2 = pass1_1040_b316(param_1, param_2, param_3, param_4, param_5);
            return uVar2;
        }
        ppcVar1 = (*param_1 + 0x7c);
        (**ppcVar1)();
    }
    return 0x1;
}


pub unsafe fn pass1_1040_b8be(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x80);
    (**ppcVar1)();
    return;
}


pub unsafe fn pass1_1040_bb5a(mut param_1: u32) -> u16

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x94) + 0x8);
    (**ppcVar1)();
    return 0x0;
}


pub unsafe fn pass1_1040_be94(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_c518(mut param_1: u32, param_2: u8) -> u32

{
    pass1_1040_bf92(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_c71e(param_1: *mut Struct65)

{
    let mut iVar1: *mut Struct65;
    let mut uVar1: *mut Struct65;

    pass1_1040_9252(param_1);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1[0x1].field1_0x2 = iVar1.field17_0x24 / 0x2 - iVar1[0x1].field3_0x6 / 0x2;
    return;
}


pub unsafe fn pass1_1040_c94a(param_1: *mut u8, pstruct37_param_2: *mut Struct37, hdc16_param_3: HDC16, mut param_4: u16)

{
    let mut var7: u16;
    let mut var8: u16;
    let mut var9: u16;
    let mut var10: u16;

    if (pstruct37_param_2.field_0x48) != 0 {
        let var6 = mixed_1010_20ba(param_1, 0xed0,
                                   CONCAT22(param_4, 0x3), var7, var8, var9,
                                   var10);
        let var3 = (var6 >> 0x10);
        let var2 = (pstruct37_param_2.field_0x42);
        let var1 = (var2 + 0x12);
        let var4 = var1;
        pass1_1010_a5ca(var1, var3, var6, var3, var1);
        if var4 == 0xffff {
            (pstruct37_param_2.field_0x3c) = 0xf9;
        } else if var4 == 0 {
            (pstruct37_param_2.field_0x3c) = 0x25;
            if (var1 == 0x5b) || (var1 == 0x9) {
                (pstruct37_param_2.field_0x3c) = 0xfe;
            }
        } else {
            (pstruct37_param_2.field_0x3c) = 0xfb;
        }
    }
    draw_text_1040_94fc(pstruct37_param_2, hdc16_param_3);
    return;
}


pub unsafe fn pass1_1040_c9cc(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_c5ac(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_caa6(param_1: *mut u8, mut param_2: u16, mut param_3: u32)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct27;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut iVar2: i16;

    iVar2 = 0;
    paVar1 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0, 0x2b,
                             in_stack_0000fea0, in_stack_0000ffc4, in_stack_0000ffca, in_stack_0000ffce);
    pass1_1010_038e(paVar1, iVar2);
    destroy_window_1040_b726(CONCAT22(param_3, param_2), (param_3 >> 0x10));
    return;
}


pub unsafe fn pass1_1040_cc66(mut param_1: u32) -> LRESULT

{
    let mut ppcVar1: *mut *mut code;
    let mut extraout_DX: u16;
    let mut LVar2: LRESULT;

    ppcVar1 = ((param_1 + 0x98) + 0x10);
    (**ppcVar1)();
    LVar2 = dialog::send_dlg_msg_1040_cf1c(extraout_DX, param_1);
    return LVar2;
}


pub unsafe fn pass1_1040_cc8c(param_1: *mut u8, param_2: *mut Struct903, mut param_3: u16, mut param_4: u16, mut param_5: u16)

{
    if (param_5 == 0xeb) {
        dialog::send_dlg_msg_1040_cf1c(param_1, param_2);
    } else {
        // just 0x1756
        if (param_5 == s_vrpal_bmp_1050_183a + 0x7) {
            msg_box::msg_box_op_1040_cce4(0x0, param_1, param_2);
        } else {
            if (param_5 != s_vrpal_bmp_1050_183a + 0x8) {
                pass1_1040_b54a(param_1, param_2, param_3, _param_4);
                return;
            }
            if (param_4 == 1) {
                dialog::send_dlg_item_1040_ce76(param_2);
            }
        }
    }
    return;
}


pub unsafe fn pass1_1040_d056(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_ca74(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_d29c(mut param_1: u32)

{
    let mut in_DX: u16;

    dialog::send_dlg_item_msg_1040_d79c(in_DX, param_1);
    return;
}


pub unsafe fn pass1_1040_d76e(mut param_1: u32)

{
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    uVar1 = (iVar2 + 0x94);
    pass1_1018_5742(uVar1, (uVar1 >> 0x10), (iVar2 + 0x9c), (iVar2 + 0x98),
    );
    (iVar2 + 0x9c) = 0;
    return;
}


pub unsafe fn pass1_1040_d89e(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_d1bc(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
