use crate::app_context::AppContext;
use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::unk::block_1008_3000::{pass1_1008_3bd6, pass1_1008_3e94};
use crate::unk::block_1008_4000::{pass1_1008_4d72, pass1_1008_4d84, struct_1008_4c58};
use crate::unk::block_1008_5000::{pass1_1008_5fd8, win_1008_5c5c, win_1008_5c7c};
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_4000::pass1_1010_4f30;
use crate::unk::block_1010_6000::{
    pass1_1010_60fa, pass1_1010_6118, pass1_1010_659a, pass1_1010_6604,
};
use crate::unk::block_1010_7000::pass1_1010_7b8c;
use crate::gui::window::win_d::set_window_text_1018_6086;
use crate::unk::block_1030_2000::{pass1_1030_2f1a, pass1_1030_2fac};
use crate::unk::block_1030_5000::pass1_1030_532e;
use crate::unk::block_1030_7000::struct_op_1030_73a8;
use crate::unk::block_1030_8000::{
    fn_ptr_1030_835a, pass1_1030_8334, pass1_1030_8344, pass1_1030_838e,
};
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1038_b000::pass1_1038_b6e0;
use crate::gui::cleanup::destroy_win_1038_ef3a;
use crate::gui::get_sys_metrics_1040_7728;
use crate::gui::window::move_win_1040_826c;
use crate::unk::block_1040_a000::pass1_1040_a5d0;
use crate::globals::u32_1050_0004;
use crate::structs::struct_394::astruct_394;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_915::astruct_915;
use crate::structs::struct_d::StructD;
use crate::winapi16::{DestroyWindow16, GetDlgItem16, GetStockObject16, GetWindowRect16, MapDialogRect16, SetFocus16, SetWindowPos16, ShowWindow16};
use crate::utils::{CONCAT11, CONCAT22, SUB42};
use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::draw_ops::draw_e::ui_cleanup_op_1040_782c;
use crate::resources::{load_string_1010_847e, load_string_1010_84e0};
use crate::gui::dialog::dialog_ui_fn_1040_78e2;
use crate::winapp::winapp_b::post_win_msg_1040_7b3c;
use crate::windef16::{HCURSOR16, HGDIOBJ16, HWND16, LRESULT, WPARAM16};


pub unsafe fn pass1_1040_06e8(
    param_1: *mut StructD,
    param_2: *mut Struct57,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
) -> *mut Struct57 {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut iVar1: *mut Struct57;
    let mut unaff_BP: u16;
    let mut uVar1: *mut Struct57;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_1);
    get_sys_metrics_1040_7728(param_2, 0x1, param_3, 0xfbc, param_6);
    uVar1 = (param_2 >> 0x10);
    iVar1 = param_2;
    (iVar1 + 1) = 0;
    param_2.field0_0x0 = 0xb90;
    iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
    puVar2 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x7),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    (iVar1 + 1).field0_0x0 = puVar2;
    iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
    return param_2;
}


pub unsafe fn pass1_1040_073a(param_1: *mut StructD) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0xb90;
    (param_1 + 0x2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1);
    return;
}


pub unsafe fn pass1_1040_0bfc(
    param_1: *mut StructD,
    param_2: *mut Struct57,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
) -> *mut Struct57 {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut iVar1: *mut Struct57;
    let mut unaff_BP: u16;
    let mut uVar1: *mut Struct57;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_1);
    get_sys_metrics_1040_7728(param_2, 0x1, param_3, 0xfcd, param_6);
    uVar1 = (param_2 >> 0x10);
    iVar1 = param_2;
    (iVar1 + 1) = 0;
    param_2.field0_0x0 = 0xdb0;
    iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
    puVar2 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x39),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    (iVar1 + 1).field0_0x0 = puVar2;
    iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
    iVar1.field86_0x74 = 0x1;
    return param_2;
}
pub unsafe fn pass1_1040_0c54(param_1: *mut StructD) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0xdb0;
    (param_1 + 0x2) = &PTR_LOOP_1050_1040;
    (param_1 + 0x8e) = 0;
    ui_cleanup_op_1040_782c(param_1);
    return;
}









pub unsafe fn pass1_1040_0e1c(
    param_1: *mut StructD,
    param_2: *mut Struct57,
    mut param_3: u16,
    mut param_4: u32,
    mut param_5: u16,
) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut iVar1: *mut Struct57;
    let mut unaff_BP: u16;
    let mut uVar1: *mut Struct57;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_1);
    get_sys_metrics_1040_7728(param_2, 0x1, 0x0, 0x1c0, param_5);
    uVar1 = (param_2 >> 0x10);
    iVar1 = param_2;
    (iVar1 + 1) = 0;
    iVar1[0x1].field2_0x4 = param_4;
    iVar1[0x1].field4_0x8 = 0;
    iVar1[0x1].field5_0xa = param_3;
    // just 0x11d2
    param_2.field0_0x0 = s_overflow_on_node__d_1050_11ca + 0x8;
    // just 0x1040
    iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
    puVar2 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x3a),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    (iVar1 + 1).field0_0x0 = puVar2;
    iVar1[0x1].field1_0x2 = (puVar2 >> 0x10);
    return;
}


pub unsafe fn pass1_1040_0e86(param_1: *mut StructD) {
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;
    let mut uVar3: u16;
    let mut in_EDX: u32;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffee: u16;
    let mut paVar4: *mut Struct57;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.address_offset_field_0x0 = s_overflow_on_node__d_1050_11ca + 0x8;
    (iVar5 + 0x2) = &PTR_LOOP_1050_1040;
    pcVar2 = *(iVar5 + 0x92);
    uVar1 = (iVar5 + 0x94);
    uVar3 = uVar1 | pcVar2;
    paVar4 = (in_EDX & 0xffff0000 | uVar3);
    if (uVar3 != 0) {
        pass1_1040_a5d0((pcVar2 & 0xffff | uVar1 << 0x10));
        fn_ptr_1000_17ce(pcVar2);
    }
    PTR_LOOP_1050_5b82 = (iVar5 + 0x96);
    if ((iVar5 + 0x92) == 0) {
        pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (iVar5 + 0x6));
    } else {
        puVar7 = mixed_1010_20ba(
            paVar4,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000ffee, 0x32),
            in_stack_0000fe96,
            in_stack_0000ffba,
            in_stack_0000ffc0,
            in_stack_0000ffc4,
        );
        pass1_1010_7b8c(puVar7, (iVar5 + 0x6));
    }
    ui_cleanup_op_1040_782c(param_1);
    return;
}
