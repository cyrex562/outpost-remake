use crate::block_1018::block_1018_5000::{pass1_1018_50ac, pass1_1018_5714, pass1_1018_58b6, pass1_1018_5b06, pass1_1018_5cc8};
use crate::block_1018::block_1018_6000::{pass1_1018_642e, pass1_1018_6630, pass1_1018_673c, pass1_1018_6924, pass1_1018_69ac};
use crate::draw_ops::{draw_line_1018_6444, draw_op_1018_6544};

pub unsafe fn pass1_1018_567c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        pass1_1000_093a(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_56a8(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1018_50ac(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_580a(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1018_5714(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1018_5932(mut param_1: u16, mut param_2: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;

    uVar4 = (param_2 >> 0x10);
    uVar3 = param_2;
    uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
    if (uVar2 != 0) {
        ppcVar1 = ((uVar3 + 0xee) + 0x8);
        uVar5 = (**ppcVar1)();
        param_1 = (uVar5 >> 0x10);
        uVar2 = uVar5;
    }
    if ((uVar3 + 0xea) == 0) {
        (uVar3 + 0xea) = 0x1;
        uVar5 = pass1_1038_af40(
            uVar3,
            param_1,
            _PTR_LOOP_1050_5b7c,
            (uVar3 + 0x8),
            ((uVar3 + 0xf6) * 0x2 + 0x4238),
        );
        uVar2 = uVar5;
    }
    return uVar2;
}


pub unsafe fn win_1018_598c(
    mut param_1: u16,
    param_2: *mut Struct57,
    struct_param_1: *mut StructA,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut uVar1: u16;
    let struct_1: *mut StructA;
    let mut uVar3: u16;
    let mut in_stack_0000fe68: u16;
    let mut in_stack_0000ff8c: u16;
    let mut in_stack_0000ff92: u16;
    let mut in_stack_0000ff96: u16;
    let mut uVar2: u32;

    create_window_ex_1008_9760(struct_param_1);
    uVar3 = (struct_param_1 >> 0x10);
    struct_1 = struct_param_1;
    get_dc_1018_4db0(&struct_1[0x1].field20_0x26, struct_1.field4_0x8);
    mem_op_1000_179c(0x2a, param_2);
    uVar1 = param_2 | param_1;
    uVar2 = param_2 & 0xffff0000 | uVar1;
    if (uVar1 != 0) {
        pass1_1018_5b06(
            uVar2,
            CONCAT22(param_2, param_1),
            struct_1.field4_0x8,
            param_4,
            param_5,
            in_stack_0000fe68,
            in_stack_0000ff8c,
            in_stack_0000ff92,
            in_stack_0000ff96,
        );
        struct_1[0x1].field18_0x22 = param_1;
        struct_1[0x1].field19_0x24 = uVar2;
        return;
    }
    struct_1[0x1].field18_0x22 = 0;
    return;
}


pub unsafe fn FUN_1018_59f0(mut param_1: u16, mut param_2: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_2 >> 0x10);
    iVar4 = param_2;
    puVar1 = (iVar4 + 0xee);
    uVar2 = (iVar4 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0xee) = 0;
    destroy_win_1008_628e(param_2 & 0xffff | uVar5 << 0x10);
    return;
}


pub unsafe fn pass1_1018_5a2e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_58b6(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn FUN_1018_5a3c(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1018_58b6(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}


pub unsafe fn pass1_1018_5df4(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1018_5cc8(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1018_5e86(param_1: u32) {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x6c);
    (**ppcVar1)();
    return;
}

pub unsafe fn pass1_1018_5ffa(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 0x92);
    uVar2 = (iVar4 + 0x94);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0x92) = 0;
    pass1_1010_1dda((iVar4 + 0x8e));
    (iVar4 + 0x8e) = 0;
    return;
}

pub unsafe fn pass1_1018_669a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_620c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1018_6768(mut param_1: u16, mut param_2: u16, mut param_3: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;

    uVar4 = (param_3 >> 0x10);
    uVar3 = param_3;
    uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
    if (uVar2 != 0) {
        ppcVar1 = ((uVar3 + 0xee) + 0x8);
        uVar5 = (**ppcVar1)();
        param_1 = (uVar5 >> 0x10);
        uVar2 = uVar5;
    }
    if ((uVar3 + 0xea) == 0) {
        (uVar3 + 0xea) = 0x1;
        uVar5 = pass1_1038_af40(uVar3, param_1, _PTR_LOOP_1050_5b7c, (uVar3 + 0x8), 0x16);
        uVar2 = uVar5;
    }
    return uVar2;
}


pub unsafe fn window_op_1018_67b6(param_1: *mut StructD, param_2: *mut StructA) {
    let mut paVar1: *mut astruct_666;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;
    let struct_1: *mut StructA;
    let mut uVar2: u16;

    paVar1 = (param_1 >> 0x10);
    paVar4 = (param_1 & 0xffff0000 | param_1 & 0xffff);
    create_window_ex_1008_9760(param_2);
    uVar2 = (param_2 >> 0x10);
    struct_1 = param_2;
    get_dc_1018_4db0(&struct_1[0x1].field20_0x26, struct_1.field4_0x8);
    mem_op_1000_179c(0x18, paVar4);
    uVar3 = paVar4 | paVar1;
    if (uVar3 != 0) {
        pass1_1018_6924(uVar3, CONCAT22(paVar4, paVar1), struct_1.field4_0x8);
        struct_1[0x1].field18_0x22 = paVar1;
        struct_1[0x1].field19_0x24 = uVar3;
        return;
    }
    struct_1[0x1].field18_0x22 = 0;
    return;
}


pub unsafe fn pass1_1018_681a(mut param_1: u32) {
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

pub unsafe fn pass1_1018_684c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_673c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1018_685a(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1018_673c(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}


pub unsafe fn pass1_1018_69dc(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_69ac(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1018_6a76() {
    return;
}

pub unsafe fn pass1_1018_6c1e(param_1: *mut StructD, param_2: u8) {
    let mut uVar1: *mut StructD;
    let mut uVar2: u16;

    uVar1 = param_1;
    uVar1 = &uVar1.field192_0xd2;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    uVar2 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0x380a;
    uVar1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    uVar1.address_offset_field_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_6048(mut param_1: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x92) + 0x8);
    (**ppcVar1)();
    return 0x0;
}

pub unsafe fn FUN_1018_60ea() {
    return;
}

pub unsafe fn FUN_1018_60ee() -> u16 {
    return 0x0;
}

pub unsafe fn FUN_1018_60f4() -> u16 {
    return 0x0;
}

pub unsafe fn FUN_1018_60fa() {
    return;
}

pub unsafe fn FUN_1018_60fe() {
    return;
}

pub unsafe fn pass1_1018_6102(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_5e5a(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn unk_draw_op_1018_623e(param_1: *mut astruct_742) {
    todo!()
}

pub unsafe fn FUN_1018_742e(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    param_5: *mut astruct_28,
) {
    mixed_draw_op_1018_6a7a(param_2, param_3, param_5);
    if (PTR_LOOP_1050_4254.is_null()) {
        win_1008_5c5c(param_1, param_2, _u16_1050_02a0, 0x1e9);
        if (param_1 != 0) {
            PTR_LOOP_1050_4254 = (&PTR_LOOP_1050_0000 + 1);
        }
    }
    return;
}

pub unsafe fn pass1_1018_7da6(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7dee(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7e36(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7e7e(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7ec6(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7f0e(param_1: *mut StructD, param_2: u8) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    iVar1 = param_1;
    iVar1 = &iVar1.field192_0xd2;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0x380a;
    iVar1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    iVar1.address_offset_field_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}


pub unsafe fn pass1_1018_7f56(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7f9e(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_7fe6(param_1: *mut StructD, param_2: u8) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    iVar1 = param_1;
    iVar1 = &iVar1.field192_0xd2;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0x380a;
    iVar1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    iVar1.address_offset_field_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1018_802e(param_1: *mut u16, param_2: u8)

{
    let mut iVar1: i16;
    let mut uVar2: u16;

    iVar1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
    uVar2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}
