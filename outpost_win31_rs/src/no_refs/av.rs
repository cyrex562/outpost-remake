use crate::unk::block_1040_6000::pass1_1040_6862;
use crate::winapp::winapp_a::create_window_1040_7620;
use crate::draw_ops::draw_f::get_sys_metrics_1040_8c66;
use crate::gui::cleanup::destroy_win_1040_8b7e;
use crate::gui::dialog::dialog_ui_fn_1040_78e2;
use crate::winapp::winapp_a::create_window_1040_8bea;

pub fn pass1_1040_6cac(mut param_1: u32)

{
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_1ea6((iVar4 + 0x94), (param_1 & 0xffff | uVar5 << 0x10));
    puVar1 = (iVar4 + 0x98);
    uVar2 = (iVar4 + 0x9a);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1010, puVar1, uVar2, 1);
    }
    (iVar4 + 0x98) = 0;
    (iVar4 + 0x94) = 0;
    return;
}


pub fn pass1_1040_6cfa(mut param_1: u32) -> u16

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x98) + 0x8);
    (**ppcVar1)();
    return 0x1;
}


pub fn pass1_1040_6f0c(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_6862(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_7044(param_1: u32, param_2: *mut i16, mut param_3: u16, mut param_4: u16, mut param_5: i16) -> u16

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
        ppcVar1 = (*param_1 + 0x80);
        (**ppcVar1)();
    }
    return 0x1;
}


pub fn pass1_1040_70a0(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x7c);
    (**ppcVar1)();
    return;
}


pub fn pass1_1040_741e(mut param_1: u32)

{
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_1ea6((iVar4 + 0x94), (param_1 & 0xffff | uVar5 << 0x10));
    puVar1 = (iVar4 + 0x98);
    uVar2 = (iVar4 + 0x9a);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1010, puVar1, uVar2, 1);
    }
    (iVar4 + 0x98) = 0;
    (iVar4 + 0x94) = 0;
    return;
}


pub fn pass1_1040_746c(mut param_1: u32) -> u16

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x98) + 0x8);
    (**ppcVar1)();
    return 0x1;
}


pub fn pass1_1040_767e(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1040_8054() -> u16

{
    return 0x0;
}


pub fn pass1_1040_807e(param_1: *mut astruct_395, mut param_2: u16)

{
    let mut ppcVar1: *mut *mut code;
    let mut in_AX: u16;
    let mut paVar2: *mut astruct_394;
    let mut paVar3: *mut astruct_394;
    let mut paVar4: *mut astruct_394;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puVar7: *mut u8;
    let mut in_EDX: u32;
    let mut paVar8: *mut Struct57;
    let mut iVar9: *mut astruct_395;
    let mut uVar9: *mut astruct_395;
    let mut unaff_CS: u16;
    let mut puStack6: *mut u32;

    if (param_2 == 1) {
        pass1_1040_805a(in_EDX);
        return;
    }
    paVar2 = FUN_1010_830a(in_AX, in_EDX, unaff_CS, _u16_1050_14cc, param_2);
    uVar5 = in_EDX;
    puStack6 = CONCAT22(uVar5, paVar2);
    paVar8 = (in_EDX & 0xffff0000 | (uVar5 | paVar2));
    if ((uVar5 | paVar2) != 0) {
        ppcVar1 = (*puStack6 + 0x14);
        paVar3 = paVar2;
        (**ppcVar1)(0x1010, paVar2, uVar5);
        uVar6 = SUB42(paVar8, 0x0);
        uVar9 = (param_1 >> 0x10);
        iVar9 = param_1;
        paVar4 = paVar3;
        if (iVar9.field112_0x70.is_null() == false) {
            paVar4 = &iVar9.field112_0x70;
            uVar5 = (&iVar9.field112_0x70 + 2);
            paVar8 = (paVar8 & 0xffff0000 | (uVar5 | paVar4));
            if ((uVar5 | paVar4) != 0) {
                ppcVar1 = paVar4;
                (**ppcVar1)(0x1010, paVar4, uVar5, 1);
            }
        }
        mem_op_1000_179c(0x14, paVar8);
        puVar7 = (paVar8 | paVar4);
        if (puVar7.is_null()) {
            paVar4 = null_mut();
            puVar7 = null_mut();
        } else {
            struct_1008_4c58(paVar4);
        }
        iVar9.field112_0x70 = paVar4;
        (&iVar9.field112_0x70 + 0x2) = puVar7;
        pass1_1008_4d84(puVar7, iVar9.field112_0x70, CONCAT22(uVar6, paVar3));
        if (puStack6.is_null() == false) {
            ppcVar1 = *puStack6;
            (**ppcVar1)(0x1008, paVar2, in_EDX, 1);
        }
        return;
    }
    return;
}

pub fn destroy_win_1040_8212(param_1: *mut astruct_899)

{
    let mut is_window: bool;
    let mut struct_1: *mut astruct_899;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    struct_1 = param_1;
    if (struct_1.field140_0x8c != 0) {
        is_window = IsWindow16(struct_1.field140_0x8c);
        if (is_window != 0) {
            DestroyWindow16(struct_1.field140_0x8c);
            struct_1.field140_0x8c = 0;
        }
    }
    return;
}


pub fn pass1_1040_824a(mut param_1: u32, mut param_2: i16) -> u16

{
    if ((param_1 + 0x6) != param_2) {
        return 0x1;
    }
    return 0x0;
}


pub fn FUN_1040_8260() -> u16

{
    return 0x0;
}

pub fn FUN_1040_8266() -> u16

{
    return 0x0;
}


pub fn pass1_1040_8978(mut param_1: u16, mut param_2: u16, param_3: *mut u32, mut param_4: u16)

{
    let mut ppcVar1: *mut *mut code;

    unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5df4);
    win_1008_5c5c(param_1, param_2, _u16_1050_02a0, param_4);
    ppcVar1 = (*param_3 + 0x74);
    (**ppcVar1)(0x1008, param_3);
    return;
}
