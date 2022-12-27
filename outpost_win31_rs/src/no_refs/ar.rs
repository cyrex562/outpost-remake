use crate::unk::block_1000_1000::fn_ptr_1000_17ce;
use crate::unk::block_1008_4000::pass1_1008_4d72;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1030_7000::struct_op_1030_73a8;
use crate::unk::block_1040_0000::pass1_1040_0c54;
use crate::unk::block_1040_1000::{pass1_1040_1290, pass1_1040_1876, pass1_1040_1d24};
use crate::unk::block_1040_2000::pass1_1040_205e;
use crate::gui::dialog::dlg_a::check_dialog_btn_1040_1b8a;
use crate::structs::struct_d::StructD;
use crate::utils::{CONCAT11, CONCAT22};
use crate::gui::dialog::dialog_ui_fn_1040_78e2;
use crate::winapi16::{GetDlgItem16, GetStockObject16, GetWindowRect16, SetWindowPos16};
use crate::winapp::winapp_c::send_msg_1040_1696;
use crate::windef16::{HGDIOBJ16, HWND16};

pub fn pass1_1040_0d80() -> u16 {
    return 0x1;
}

pub fn FUN_1040_0d86() {
    return;
}

pub fn pass1_1040_0d8a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1040_0c54(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_109c(param_1: *mut u8, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut uVar1: u32;
    let mut bVar2: bool;
    let mut iVar3: i16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u32;
    let mut paVar6: *mut Struct57;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000fff2: u16;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    bVar2 = false;
    if (param_5 == 0x1c1) {
        (param_2 + 0x96) = 0x2;
        bVar2 = true;
    } else if (param_5 == 0x1c2) {
        (param_2 + 0x96) = 0x1;
        bVar2 = true;
    } else {
        if (param_5 != 0x1830) {
            post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
            return;
        }
        paVar6 = mixed_1010_20ba(paVar4, _u16_1050_0ed0, CONCAT22(in_stack_0000fff2, 0x32), in_stack_0000fe9a,
                                 in_stack_0000ffbe, in_stack_0000ffc4, in_stack_0000ffc8);
        uVar5 = paVar4 & 0xffff0000 | paVar6 >> 0x10;
        iVar3 = paVar6;
        uVar1 = (param_2 + 0x92);
        ui_op_1010_79aa(paVar6, 0xfb6, (uVar1 + 0x6));
        if (iVar3 == 0) {
            uVar1 = (param_2 + 0x92);
            unk_win_op_1010_7300(uVar5, paVar6, 0x0, 0xc, (uVar1 + 0x6));
        }
    }
    if (bVar2) {
        uVar1 = (param_2 + 0x8e);
        (uVar1 + 0xa) = (param_2 + 0x96);
    }
    return;
}


pub fn pass1_1040_1152(param_1: *mut u8, mut param_2: i16, mut param_3: u16)

{
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000fff4: u16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    if ((param_2 + 0x92) != 0) {
        uVar2 = (param_2 + 0x8e);
        uVar1 = (uVar2 + 0xa);
        puVar6 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000fff4, 0x3), in_stack_0000fe9c,
                                 in_stack_0000ffc0, in_stack_0000ffc6, in_stack_0000ffca);
        uVar2 = (param_2 + 0x92);
        uVar5 = (uVar2 >> 0x10);
        iVar4 = uVar2;
        pass1_1010_ae92(puVar6, uVar1, (iVar4 + 0xa), (iVar4 + 0x6),
                        paVar3 & 0xffff0000 | puVar6 >> 0x10);
    }
    destroy_win_1040_7b98(CONCAT22(param_3, param_2));
    PTR_LOOP_1050_5b80 = null_mut();
    return;
}


pub fn pass1_1040_11ac(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_0e86(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn FUN_1040_1786()

{
    return;
}


pub fn pass1_1040_178a(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_1290(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_1ab0(mut param_1: u16, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32) -> u32

{
    let mut BStack6: bool;
    let mut uStack4: u16;

    BStack6 = 0;
    uStack4 = 0;
    if (param_5 == 0x1831) {
        (param_2 + 0x92) = 0x1;
        (param_2 + 0x94) = 0x1;
        check_dialog_btn_1040_1b8a(CONCAT22(param_3, param_2));
    } else {
        BStack6 = post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
        uStack4 = param_1;
    }
    return CONCAT22(uStack4, BStack6);
}

pub fn FUN_1040_1c1e()

{
    return;
}

pub fn pass1_1040_1c22(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_1876(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_1e80(mut param_1: u16, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32) -> u32

{
    let mut BStack6: bool;
    let mut uStack4: u16;

    BStack6 = 0;
    uStack4 = 0;
    if (param_5 == 0xe4) {
        pass1_1008_a9ec((param_2 + 0x92));
    } else {
        BStack6 = post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
        uStack4 = param_1;
    }
    return CONCAT22(uStack4, BStack6);
}

pub fn FUN_1040_1ec4()

{
    return;
}

pub fn pass1_1040_1ec8(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_1d24(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_1ec8(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_1d24(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_2358(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_205e(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_2464(param_1: *mut StructD)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0x2956;
    (param_1 + 0x2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1);
    return;
}
