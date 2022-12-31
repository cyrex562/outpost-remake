use crate::draw_ops;
use crate::draw_ops::draw_c;
use crate::utils::CONCAT22;
use crate::windef16::{COLORREF, DEVMODEA, HDC16, HGDIOBJ16, HPEN16};

// WARNING: Inlined function: struct_1010_4d5c





pub fn pass1_1020_2286(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut i16,
    mut param_4: i16,
) {
    *param_3 = 0x64 - param_4 >> 0x1;
    return;
}

pub fn pass1_1020_239c(mut param_1: u32, param_2: *mut i16) {
    let mut puVar1: *mut u16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut local_a: [u8; 0x6] = [0; 0x6];
    let mut uStack4: u16;

    if (param_2.is_null() == false) {
        uStack4 = ((param_2 + 0x4) - *param_2 >> 1) + *param_2;
        puVar1 = pass1_1008_3e54(CONCAT22(0x1050, local_a), 0x0, 0x57, uStack4);
        uVar3 = (param_1 >> 0x10);
        uVar2 = pass1_1020_23f2((puVar1 >> 0x10), param_1, uVar3, CONCAT22(0x1050, local_a));
        draw_c::draw_polygon_1020_2474(param_1, uVar3, 0x3, uVar2, (uVar2 >> 0x10));
    }
    return;
}

pub fn pass1_1020_23f2(
    param_1: *mut u8,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut u16,
) -> u32 {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut iStack18: i16;
    let mut local_6: i16;
    let mut local_4: i16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    piVar1 = &local_6;
    pass1_1008_3e94(
        param_4,
        CONCAT22(0x1050, piVar1),
        CONCAT22(0x1050, &local_4),
    );
    mem_op_1000_179c(0xc, paVar3);
    uVar2 = SUB42(paVar3, 0x0);
    for iStack18 in 0..0x3 {
        piVar1[iStack18 * 0x2] = (iStack18 * 0x4 + 0x4270) + local_4;
        piVar1[iStack18 * 0x2 + 0x1] = (iStack18 * 0x4 + 0x4272) + local_6;
    }
    return CONCAT22(uVar2, piVar1);
}

pub fn pass1_1020_2488(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_stack_0000fff2: i16;
    let mut iStack12: i16;
    let mut uStack10: u16;

    uVar5 = (param_3 >> 0x10);
    iVar4 = param_3;
    find_n_load_rsrc_1010_4e9e((iVar4 + 0x6));
    if ((param_2 | param_1) != 0) {
        uStack10 = param_1;
        for iStack12 in 0..0x9 {
            uVar1 = (iVar4 + 0x6);
            uVar2 = pass1_1010_4f20(uVar1, (uVar1 >> 0x10), iStack12);
            uVar1 = (iVar4 + 0xa);
            set_win_tet_1020_1d2a(
                uVar1,
                (uVar1 >> 0x10),
                CONCAT22(param_2, uStack10),
                uVar2,
                in_stack_0000fff2,
            );
            uVar3 = str_op_1000_3da4(CONCAT22(param_2, uStack10));
            uStack10 += uVar3 + 1;
        }
    }
    return;
}



pub fn struct_1020_2524(
    mut param_1: u16,
    param_2: *mut Struct20,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar2: *mut Struct20;
    let mut unaff_BP: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    unk_draw_op_1020_7f7a(param_2, 0x7, CONCAT22(param_4, param_3), param_5);
    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    iVar2[0x1].field5_0xc = 0;
    iVar2[0x1].field7_0x10 = null_mut();
    param_2.offset_0x0 = 0x270c;
    iVar2.base_0x2 = 0x1020;
    (iVar2 + 1).offset_0x0 = 0x27a8;
    iVar2[0x1].base_0x2 = 0x1020;
    puVar4 = mixed_1010_20ba(
        paVar2,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x2a),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    uVar1 = (puVar4 >> 0x10);
    iVar2[0x1].field7_0x10 = puVar4;
    (&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
    iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
    (&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
    return;
}

pub fn pass1_1020_2594(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x270c;
    iVar1.address_offset_field_0x2 = 0x1020;
    iVar1.field_0xe2 = 0x27a8;
    iVar1.field_0xe4 = 0x1020;
    pass1_1020_808e(param_1);
    return;
}



pub fn pass1_1020_27b0(
    mut param_1: u16,
    param_2: *mut astruct_664,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut puVar4: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffca: u32;
    let mut in_stack_0000fff2: u16;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    set_struct_op_1020_921c(
        param_1,
        CONCAT22(param_3, param_2),
        param_4,
        in_stack_0000ffca,
    );
    param_2.field16_0x14 = 0;
    CONCAT22(param_3, param_2) = 0x288e;
    param_2.field2_0x2 = 0x1020;
    puVar6 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0x2a),
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    uVar3 = (puVar6 >> 0x10);
    param_2.field16_0x14 = puVar6;
    param_2.field17_0x16 = uVar3;
    param_2.field5_0x6 = param_2.field16_0x14;
    param_2.field6_0x8 = uVar3;
    uVar2 = &param_2.field16_0x14;
    puVar4 = &param_2.field_0xa;
    ppcVar1 = ((uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_2.field15_0x12 = puVar4;
    draw_op_1020_9364(CONCAT22(param_3, param_2));
    return;
}

pub fn pass1_1020_2838(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x288e;
    iVar1.address_offset_field_0x2 = 0x1020;
    if (iVar1.field12_0x14 != 0) {
        pass1_1010_1dda(iVar1.field12_0x14);
    }
    palette_op_1020_92c4(param_1);
    return;
}


pub fn pass1_1020_289a(param_1: *mut u16, mut param_2: u16, mut param_3: u32) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    struct_1020_790e(param_1, s_SCPOPMENU_1050_427c, param_2, param_3);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xf2) = 0;
    (iVar1 + 0xf6) = 0;
    (iVar1 + 0xfa) = 0;
    (iVar1 + 0xfc) = 0;
    *param_1 = 0x2e4a;
    (iVar1 + 0x2) = 0x1020;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (iVar1 + 0x5b)), s_VrMode_1050_4286);
    (iVar1 + 0xac) = 0x44c00000;
    return;
}

pub fn pass1_1020_28fc(param_1: *mut StructD) {
    param_1.address_offset_field_0x0 = 0x2e4a;
    (param_1 + 0x2) = 0x1020;
    cleanup_menu_ui_op_1020_795c(param_1);
    return;
}

pub fn pass1_1020_2936() {
    pass1_1020_79ae();
    return;
}


pub fn pass1_1020_294a(
    param_1: *mut u8,
    param_2: *mut astruct_665,
    mut param_3: u32,
    mut param_4: u16,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut iVar3: *mut astruct_665;
    let mut unaff_BP: u16;
    let mut uVar2: *mut astruct_665;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    uVar2 = (param_2 >> 0x10);
    iVar3 = param_2;
    iVar3.field248_0xfc = param_4;
    puVar2 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, param_4),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    uVar1 = (puVar2 >> 0x10);
    iVar3.field240_0xf2 = puVar2;
    iVar3.field241_0xf4 = uVar1;
    iVar3.field224_0xe0 = iVar3.field240_0xf2;
    iVar3.field225_0xe2 = uVar1;
    pass1_1018_0902(&iVar3.field240_0xf2, param_3);
    return;
}





pub fn pass1_1020_2a94(mut param_1: u32, mut param_2: u32) {
    pass1_1018_1662((param_1 + 0xf2), param_2, (param_2 >> 0x10));
    return;
}
