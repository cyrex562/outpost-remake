use crate::unk::block_1020_2000::{pass1_1020_2286, pass1_1020_239c, pass1_1020_2488, pass1_1020_2594, pass1_1020_27b0, pass1_1020_2838, pass1_1020_28fc};
use crate::unk::block_1020_3000::pass1_1020_3c8c;
use crate::draw_ops::draw_c::draw_line_1020_229c;
use crate::draw_ops::draw_d::{load_draw_op_1020_2ede, mixed_draw_op_1020_3fa0};
use crate::utils::CONCAT22;
use crate::gui::cleanup::destroy_icon_1020_2c88;
use crate::gui::cleanup::destroy_window_1020_3b3e;
use crate::windef16::{HDC16, HGDIOBJ16, HPEN16};

pub unsafe fn pass1_1020_24f2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_1f74(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_25c0(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut piVar1: *mut i16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut iVar3: *mut Struct277;
    let mut uVar5: u16;
    let mut paStack10: *mut Struct57;
    let mut puStack6: *mut u32;

    paVar4 = CONCAT22(in_register_0000000a, param_2);
    uVar5 = (param_3 >> 0x10);
    iVar3 = param_3;
    if (iVar3.field236_0xee.is_null() == false) {
        ppcVar2 = (*iVar3.field236_0xee + 0x8);
        param_1 = (**ppcVar2)();
    }
    if (iVar3.field233_0xea == 0) {
        iVar3.field233_0xea = 0x1;
        mem_op_1000_179c(0x98, paVar4);
        paStack10 = CONCAT22(paVar4, param_1);
        uVar3 = paVar4 | param_1;
        if (uVar3 == 0) {
            puStack6 = null_mut();
        } else {
            piVar1 = &iVar3.field204_0xcc;
            *piVar1 = *piVar1 + 1;
            struct_1020_1738(
                paStack10,
                iVar3.field204_0xcc,
                param_3 & 0xffff | uVar5 << 0x10,
            );
            puStack6 = CONCAT22(uVar3, param_1);
        }
        ppcVar2 = (*puStack6 + 0x8);
        (**ppcVar2)(0x1000, puStack6, (puStack6 >> 0x10));
    }
    return;
}

pub unsafe fn window_op_1020_2642(
    param_1: *mut astruct_664,
    param_2: *mut Struct57,
    param_3: *mut StructA,
) {
    let mut uVar1: u16;
    let iVar2: *mut StructA;
    let mut uVar2: u16;

    create_window_ex_1008_9760(param_3);
    uVar2 = (param_3 >> 0x10);
    iVar2 = param_3;
    get_dc_1018_4db0(&iVar2[0x1].field20_0x26, iVar2.field4_0x8);
    mem_op_1000_179c(0x18, param_2);
    uVar1 = param_2 | param_1;
    if (uVar1 != 0) {
        pass1_1020_27b0(uVar1, param_1, param_2, iVar2.field4_0x8);
        iVar2[0x1].field18_0x22 = param_1;
        iVar2[0x1].field19_0x24 = uVar1;
        return;
    }
    iVar2[0x1].field18_0x22 = 0;
    return;
}

pub unsafe fn pass1_1020_26a6(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    puVar1 = (param_1 + 0xee);
    uVar2 = (param_1 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1);
    return;
}

pub unsafe fn pass1_1020_26d8(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1020_2594(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1020_26e6(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1020_2594(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn pass1_1020_2868(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_2838(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_2a46(mut param_1: u16, mut param_2: u16, mut param_3: i16) {
    let mut in_DX: u16;

    pass1_1018_0ae8((param_1 + 0xf2), 1);
    pass1_1008_68c6(in_DX, param_1, param_2, param_3);
    return;
}

pub unsafe fn pass1_1020_2a6a(mut param_1: u32) {
    get_win_ui_info_op_1020_7a50(param_1);
    pass1_1018_0ae8((param_1 + 0xf2), 0x0);
    destroy_icon_1020_2c88(param_1);
    return;
}

pub unsafe fn bring_window_to_top_1020_2aae(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u16;
    let mut unaff_SS: u16;

    pass1_1008_3e0e(param_1);
    uVar1 = (param_1 >> 0x10);
    BringWindowToTop16((param_1 + 0x8));
    pass1_1018_169e((param_1 + 0xf2), param_2);
    return;
}

pub unsafe fn pass1_1020_2c72(mut param_1: u32) {
    draw_op_1020_30be((param_1 + 0xf6));
    return;
}


pub unsafe fn pass1_1020_2e24(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_28fc(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1020_3616(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    cleanup_win_ui_1020_2fea(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_3898(param_1: *mut astruct_30) {
    destroy_window_1020_3b3e(param_1);
    return;
}

pub unsafe fn pass1_1020_3bd6(mut param_1: u16, mut param_2: u32) {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u32;

    uVar3 = (param_2 >> 0x10);
    uVar2 = param_2;
    mixed_draw_op_1020_3fa0((uVar2 + 0xf6));
    if ((uVar2 + 0x100) == 0) {
        (uVar2 + 0x100) = 0x1;
        uVar4 = (uVar2 + 0xfa);
        if ((uVar4 + 0x56) == 0) {
            iVar1 = 0x5;
        } else {
            iVar1 = 0x8;
        }
        uVar4 = pass1_1038_af40(uVar2, param_1, _PTR_LOOP_1050_5b7c, (uVar2 + 0x8), iVar1);
        (uVar2 + 0x10e) = uVar4;
        (uVar2 + 0x110) = (uVar4 >> 0x10);
    }
    return;
}

pub unsafe fn pass1_1020_3c74(mut param_1: u16, mut param_2: u32, mut param_3: u16) {
    pass1_1020_3c8c(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, (param_2 >> 0x10)),
    );
    return;
}

pub unsafe fn pass1_1020_3ca6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    let mut uVar1: u32;
    let mut puStack10: *mut StructD;

    uVar1 = param_1 & 0xffff0000;
    param_1 = (uVar1 | param_1 - 0xf2);
    param_1 = (uVar1 >> 0x10);
    if (param_1.is_null()) {
        param_1 = 0;
        param_1 = 0;
    }
    puStack10 = CONCAT22(param_1, param_1);
    puStack10.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    cleanup_menu_ui_op_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1020_3cb8(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    let mut iVar1: i16;
    let mut puStack10: *mut u16;

    if (param_2.is_null()) {
        iVar1 = 0;
        param_2 = 0;
    } else {
        iVar1 = param_2 + 0xf2;
    }
    puStack10 = CONCAT22(param_2, iVar1);
    *puStack10 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    cleanup_menu_ui_op_1020_795c(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}
