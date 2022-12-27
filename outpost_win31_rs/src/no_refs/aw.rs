use crate::unk::block_1040_8000::{pass1_1040_869a, pass1_1040_8e82};
use crate::unk::block_1040_9000::pass1_1040_9824;
use crate::unk::block_1040_a000::{msg_box_ui_op_1040_ad66, pass1_1040_ace8};
use crate::draw_ops::draw_e::unk_draw_op_1040_b0f8;
use crate::draw_ops::draw_a::draw_op_1040_9948;
use crate::winapp::free_proc_inst_1040_a294;
use crate::resources::msg_box_op_1040_a85a;
use crate::gui::dialog::dlg_a::win_ui_op_1040_ae04;
use crate::gui::dialog::dlg_b::win_ui_dlg_op_1040_a94a;
use crate::winapp::winapp_c::win_msg_1040_a308;
use crate::windef16::RECT16;

pub fn pass1_1040_89a4(param_1: *mut u8, param_2: *mut u32, param_3: *mut u16)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar7: u16;
    let mut puVar8: *mut u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000fff0: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
    uVar1 = (param_3 + 2);
    uVar2 = *param_3;
    uVar7 = 0x1010;
    puVar8 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000fff0, 0x2), in_stack_0000fe98,
                             in_stack_0000ffbc, in_stack_0000ffc2, in_stack_0000ffc6);
    uVar4 = (puVar8 >> 0x10);
    uVar5 = puVar8;
    if ((uVar5 + 0x72) != 0) {
        uVar7 = 0x1008;
        win_1008_5c7c(uVar5, uVar4, _u16_1050_02a0, CONCAT22(uVar1, uVar2));
        (param_2 + 0x8c) = uVar5;
    }
    ppcVar3 = (*param_2 + 0x74);
    (**ppcVar3)(uVar7, param_2);
    return;
}


pub fn pass1_1040_8b3c(mut param_1: u16, mut param_2: u32, mut param_3: u32)

{
    if ((param_3.is_null() == false) && ((param_3 == (&PTR_LOOP_1050_0000 + 1) || param_3 == &u16_1050_0002 || (((&u16_1050_0002 + 1) < param_3 - 0x2 && (param_3 - 0x6 < &u16_1050_0002)))))) {
        PTR_LOOP_1050_5df4 = null_mut();
        PTR_LOOP_1050_5df8 = param_3;
        return;
    }
    post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), (param_2 >> 0x10), param_3, param_3);
    return;
}


pub fn pass1_1040_8db6(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_869a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_8f16(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_8e82(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn send_msg_1040_9404(mut param_1: u32) -> LRESULT

{
    let mut uVar1: u16;
    let mut LVar2: LRESULT;

    uVar1 = (param_1 >> 0x10);
    LVar2 = SendMessage16(0x0, (param_1 + 0x1c), 0x111, (param_1 + 0x1a));
    return LVar2;
}


pub fn pass1_1040_9422(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x8) != 0) {
        ppcVar1 = (*param_1 + 0x10);
        (**ppcVar1)();
    }
    if ((param_1 + 0x4) != 0) {
        ppcVar1 = (*param_1 + 0x14);
        (**ppcVar1)();
    }
    return;
}


pub fn pass1_1040_a204(param_1: *mut u16, param_2: u8) -> *mut u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_a2cc(mut param_1: u16, param_2: *mut u8, mut param_3: i16, mut param_4: u32, mut param_5: u32) -> u32

{
    let mut uVar1: u16;

    if (param_5 == 0x1826) {
        if ((param_5 == 1) || ((0x1 < param_5 - 0x1 && (param_5 - 0x3 < 0x2)))) {
            uVar1 = 0x1;
        } else {
            uVar1 = 0;
        }
        return uVar1;
    }
    pass1_1040_b54a(param_2, CONCAT22(param_4, param_3), (param_4 >> 0x10), param_5);
    return CONCAT22(param_2, param_1);
}


pub fn pass1_1040_a4c2(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    free_proc_inst_1040_a294(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_a564(param_1: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_1 = 0;
    (param_1 + 0x4) = 0;
    (param_1 + 0x6) = 0;
    return;
}

pub fn pass1_1040_a582(param_1: u32)

{
    fn_ptr_1000_17ce(*param_1);
    return;
}


pub fn pass1_1040_a84a(param_1: u8, mut param_2: u16, mut param_3: u32)

{
    win_ui_dlg_op_1040_a94a(param_2, param_3);
    return;
}


pub fn pass1_1040_abe2(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_ad14(mut param_1: u32)

{
    let mut in_DX: u16;

    win_ui_op_1040_ae04(in_DX, param_1);
    return;
}

pub fn pass1_1040_ad24(param_1: *mut u8, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    if (param_5 == 0xeb) {
        win_ui_op_1040_ae04(param_1, CONCAT22(param_3, param_2));
    } else {
        if (param_5 != 0x1f0) {
            pass1_1040_b54a(param_1, CONCAT22(param_3, param_2), param_4, param_5);
            return;
        }
        msg_box_ui_op_1040_ad66(0x0, param_1, CONCAT22(param_3, param_2));
    }
    return;
}


pub fn pass1_1040_af9e(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_ace8(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_b17c(param_1: *mut u8, mut param_2: u32, mut param_3: u32)

{
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut pcVar3: *mut c_char;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u32;
    let mut iVar6: i16;
    let mut unaff_SI: u16;
    let mut uVar7: u16;
    let mut puVar8: *mut u32;
    let mut in_stack_0000fe94: u16;
    let mut in_stack_0000ffb8: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc2: u16;
    let mut ppuVar9: *mut *mut u8;
    let mut puStack12: *mut u16;
    let mut iStack4: i16;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    iStack4 = 0;
    loop {
        uVar7 = (param_2 >> 0x10);
        iVar6 = param_2;
        piVar1 = (iVar6 + 0x90);
        if (*piVar1 == iStack4 || *piVar1 < iStack4) { break; }
        paVar4 = (paVar4 & 0xffff0000 | piVar1 >> 0x10);
        uVar2 = (piVar1 + 2);
        (iStack4 * 0xa + uVar2 + 0x4) = (iStack4 * 0x2 + param_3);
        iStack4 += 0x1;
    }
    ppuVar9 = CONCAT22(unaff_SI, 0x3);
    puVar8 = mixed_1010_20ba(paVar4, _u16_1050_0ed0, ppuVar9, in_stack_0000fe94, in_stack_0000ffb8, in_stack_0000ffbe,
                             in_stack_0000ffc2);
    uVar5 = puVar8 >> 0x10;
    uVar2 = (iVar6 + 0x90);
    puStack12 = (uVar2 + 2);
//   for (iStack4 = 0; piVar1 = (iVar6 + 0x90), *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 += 1)
    iStack4 = 0;
    piVar1 = iVar6 + 0x90;
    while *piVar1 != iStack4 && iStack4 <= *piVar1 {
        ppuVar9 = (ppuVar9 & 0xffff0000);
        uVar2 = (iVar6 + 0x90);
        uVar2 = (uVar2 + 0x6);
        pcVar3 = pass1_1010_b038(puVar8, uVar2, (uVar2 >> 0x10),
                                 (puStack12 + 0x4), (ppuVar9 >> 0x10));
        string_1040_a626(uVar5, puStack12, CONCAT22(uVar5, pcVar3));
        puStack12 = (puStack12 & 0xffff0000 | (puStack12 + 0xa));
        iStack4 += 1;
    }
    return;
}


pub fn pass1_1040_b4c8(param_1: *mut u8, mut param_2: u32)

{
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut puVar8: *mut u32;
    let mut in_stack_0000fea2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000fffa: u16;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    uVar7 = (param_2 >> 0x10);
    if ((param_2 + 0x90) != 0) {
        puVar8 = mixed_1010_20ba(paVar5, _u16_1050_0ed0, CONCAT22(in_stack_0000fffa, 0x32), in_stack_0000fea2,
                                 in_stack_0000ffc6, in_stack_0000ffcc, in_stack_0000ffd0);
        uVar6 = paVar5 & 0xffff0000 | puVar8 >> 0x10;
        uVar3 = puVar8;
        uVar2 = (param_2 + 0x90);
        iVar1 = (uVar2 + 0xa);
        iVar4 = iVar1 - 0x4;
        if (iVar4 == 0) {
            ui_op_1010_79aa(puVar8, 0xfd9, 0x0);
            if (iVar4 == 0) {
                uVar7 = 0xe;//
// LAB_1040_b50f:
                unk_win_op_1010_7300(uVar6, (puVar8 & 0xffff0000 | uVar3), CONCAT22(iVar4, iVar4), uVar7,
                                     CONCAT22(iVar4, iVar4));
                return;
            }
        } else if (((0x0 < iVar1 - 0x5) && (!SBORROW2(iVar1 - 0x5, 1))) && (iVar4 = iVar1 - 0x7, iVar4 == 0x0 || iVar1 - 0x6 < 1)) {
            ui_op_1010_79aa(puVar8, 0xfda, 0x0);
            if (iVar4 == 0) {
                uVar7 = 0xd;
                // TODO: goto LAB_1040_b50f;
            }
        }
    }
    return;
}


pub fn pass1_1040_b74c(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffde: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffde, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
