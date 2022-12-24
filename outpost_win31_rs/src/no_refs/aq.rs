use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::app_context::AppContext;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::unk::block_1008_3000::{pass1_1008_3bd6, pass1_1008_3e94};
use crate::unk::block_1008_4000::{pass1_1008_4d84, struct_1008_4c58};
use crate::unk::block_1008_5000::{pass1_1008_5fd8, win_1008_5c5c, win_1008_5c7c};
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::gui::dialog::dlg_b::enable_win_1040_060e;
use crate::unk::block_1030_2000::{pass1_1030_2f1a, pass1_1030_2fac};
use crate::unk::block_1030_5000::pass1_1030_532e;
use crate::unk::block_1030_8000::{fn_ptr_1030_835a, pass1_1030_8334, pass1_1030_8344, pass1_1030_838e};
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1038_e000::{pass1_1038_e9ec, pass1_1038_ebd6};
use crate::unk::block_1040_0000::pass1_1040_073a;
use crate::winapp::winapp_b::post_win_msg_1040_7b3c;
use crate::gui::window::win_e::{enable_window_1040_0acc, win_ui_op_1040_0558};
use crate::globals::u32_1050_0004;
use crate::resources::{load_string_1010_847e, load_string_1010_84e0};
use crate::structs::struct_394::astruct_394;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_915::astruct_915;
use crate::structs::struct_d::StructD;
use crate::utils::{CONCAT11, CONCAT22, SUB42};
use crate::gui::cleanup::destroy_win_1038_ef3a;
use crate::gui::dialog::dialog_ui_fn_1040_78e2;
use crate::gui::window::move_win_1040_826c;
use crate::gui::window::win_d::set_window_text_1018_6086;
use crate::winapi16::{DestroyWindow16, GetDlgItem16, GetWindowRect16, MapDialogRect16, SetFocus16, SetWindowPos16, ShowWindow16};
use crate::windef16::{HCURSOR16, HWND16, LRESULT, WPARAM16};

pub unsafe fn FUN_1038_eb08()

{
    return;
}


pub unsafe fn pass1_1038_eb0c(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1038_e9ec(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1038_ee48(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1038_ebd6(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_0656(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    destroy_win_1038_ef3a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn show_win_1040_0766(struct_b_param_1: *mut StructB, mut param_2: u16) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb4: u16;
    let mut piVar3: *mut i16;
    let mut uVar4: u16;
    let mut piVar5: *mut i16;
    let mut uVar6: u16;
    let mut in_stack_0000ffde: u16;
    let mut local_a: i16;
    let mut local_8: i16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_2);
    dialog_ui_fn_1040_78e2(struct_b_param_1);
    puVar2 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffde, 0x2),
        in_stack_0000fe86,
        in_stack_0000ffaa,
        in_stack_0000ffb0,
        in_stack_0000ffb4,
    );
    paVar1 = (paVar1 & 0xffff0000 | puVar2 >> 0x10);
    uStack6 = SUB42(puVar2, 0x0);
    uStack4 = (puVar2 >> 0x10);
    pass1_1010_6118(puVar2);
    piVar5 = &local_8;
    uVar6 = SUB42(0x1050, 0x0);
    piVar3 = &local_a;
    uVar4 = SUB42(0x1050, 0x0);
    puVar2 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0,
        CONCAT22(piVar3, 0x48),
        in_stack_0000fe7e,
        in_stack_0000ffa2,
        in_stack_0000ffa8,
        in_stack_0000ffac,
    );
    pass1_1008_3e94(
        (puVar2 & 0xffff0000 | (puVar2 + 0xe)),
        CONCAT22(uVar4, piVar3),
        CONCAT22(uVar6, piVar5),
    );
    move_win_1040_826c(struct_b_param_1, local_a + 0x8c, local_8 + 0xb9);
    ShowWindow16(0x5, (struct_b_param_1 + 0x6));
    return;
}


pub unsafe fn pass1_1040_0a1a(mut param_1: u32) {
    let mut uVar1: u16;
    let mut puVar2: *mut u32;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut paVar5: *mut astruct_394;
    let mut uVar6: u16;
    let mut puVar7: *mut u8;
    let mut in_EDX: u32;
    let mut paVar8: *mut Struct57;
    let mut iVar9: i16;
    let mut iVar10: i16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uStack10: u32;
    let mut uStack6: u16;

    uVar11 = (param_1 >> 0x10);
    iVar9 = param_1;
    uVar4 = (iVar9 + 0x8e);
    uVar12 = (uVar4 >> 0x10);
    iVar10 = uVar4;
    puVar2 = (iVar10 + 0xa);
    uVar1 = (iVar10 + 0xc);
    paVar8 = (in_EDX & 0xffff0000 | uVar1);
    uStack6 = puVar2;
    paVar5 = (uVar1 | uStack6);
    if (paVar5.is_null()) {
        return;
    }
    ppcVar3 = (*puVar2 + 0x14);
    (**ppcVar3)();
    uStack10 = CONCAT22(paVar8, paVar5);
    if ((iVar9 + 0x70) != 0) {
        paVar5 = (iVar9 + 0x70);
        uVar1 = (iVar9 + 0x72);
        uVar6 = uVar1 | paVar5;
        paVar8 = (paVar8 & 0xffff0000 | uVar6);
        if (uVar6 != 0) {
            ppcVar3 = paVar5;
            (**ppcVar3)();
        }
    }
    mem_op_1000_179c(0x14, paVar8);
    puVar7 = (paVar8 | paVar5);
    if (puVar7.is_null()) {
        paVar5 = null_mut();
        puVar7 = null_mut();
    } else {
        struct_1008_4c58(paVar5);
    }
    (iVar9 + 0x70) = paVar5;
    (iVar9 + 0x72) = puVar7;
    pass1_1008_4d84(puVar7, (iVar9 + 0x70), uStack10);
    return;
}


pub unsafe fn pass1_1040_0b6a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1040_073a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
