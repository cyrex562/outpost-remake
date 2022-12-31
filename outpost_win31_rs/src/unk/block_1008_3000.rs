use std::os::raw::c_char;
use crate::{file_ops, gui};
use crate::app_context::AppContext;
use crate::structs::struct_1001::Struct1001;
use crate::structs::struct_57::Struct57;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::utils::CONCAT22;

pub fn pass1_1008_3018(ctx: &mut AppContext, pstruct57_1: *mut Struct57, mut param_2: u32) {
    let mut pc_var1: *mut c_char;
    let mut u16_var2: u16;
    let mut i16_var3: i16;
    let mut u16_var4: u16;
    let mut in_register_0000000a = 0u16;
    // let mut unaff_SI: u16;
    let mut in_stack_0000fd8a: u16;
    let mut in_stack_0000feae: u16;
    let mut in_stack_0000feb4: u16;
    let mut in_stack_0000feb8: u16;
    let mut u16_var266: u16;
    let mut u16_var262: u32;
    let mut au8_var100: [u8; 0x100] = [0; 0x100];

    au8_var100[0] = '\0' as u8;
    u16_var262 = mixed_1010_20ba(
        pstruct57_1,
        _u16_1050_0ed0,
        CONCAT22(ctx.SI_REG, 0x2),
        in_stack_0000fd8a,
        in_stack_0000feae,
        in_stack_0000feb4,
        in_stack_0000feb8,
    );
    // u16_var2 = (uStack262 >> 0x10);
    i16_var3 = u16_var262;
    pc_var1 = *(i16_var3 + 0x12);
    u16_var4 = (i16_var3 + 0x14);
    u16_var266 = pc_var1;
    if (u16_var4 | u16_var266) == 0 {
        pass1_1008_30cc(0x0, u16_var4, param_2);
    } else {
        unk_str_op_1000_3d3e(CONCAT22(0x1050, au8_var100), *(i16_var3 + 0x1a));
        u16_var4 = str_op_1000_3da4(CONCAT22(0x1050, au8_var100));
        if (au8_var100[u16_var4 - 0x1] != '\\') {
            au8_var100[u16_var4] = '\\';
            au8_var100[u16_var4 + 0x1] = '\0';
        }
        pass1_1000_3cea(CONCAT22(0x1050, au8_var100), pc_var1);
        if (au8_var100[0] != '\0') {
            message_box_op_1008_12dc(param_2, CONCAT22(0x1050, au8_var100));
            return;
        }
    }
    return;
}


pub fn pass1_1008_30cc(mut param_1: u16, mut param_2: u16, param_3: *mut Struct72) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut paVar2: *mut astruct_484;
    let mut in_stack_0000fc90: u16;
    let mut in_stack_0000fdb4: u16;
    let mut in_stack_0000fdba: u16;
    let mut in_stack_0000fdbe: u16;
    let mut puVar3: *mut u8;
    let mut uVar4: u16;
    let mut local_210: [u8; 0xa] = [0; 0xa];
    let mut local_206: [u8; 0x100] = [0; 0x100];
    let mut uStack262: u16;
    let mut uStack260: u16;
    let mut local_102: [u8; 0x100] = [0; 0x100];

    local_102[0] = '\0';
    file_ops::save_file_1008_3178(param_2, param_3, 0x2);
    paVar1 = CONCAT22(in_register_0000000a, param_2 | param_1);
    if ((param_2 | param_1) != 0) {
        uStack262 = param_1;
        uStack260 = param_2;
        unk_str_op_1000_3d3e(CONCAT22(0x1050, local_102), CONCAT22(param_2, param_1));
        str_1000_4d58(
            CONCAT22(0x1050, local_102),
            NULL,
            0x0,
            CONCAT22(0x1050, local_206),
            CONCAT22(0x1050, local_210),
        );
        if (local_210[0] != '\0') {
            pass1_1000_3cea(CONCAT22(0x1050, local_206), CONCAT22(0x1050, local_210));
        }
        puVar3 = local_206;
        uVar4 = SUB42(0x1050, 0x0);
        paVar2 = mixed_1010_20ba(
            paVar1,
            _u16_1050_0ed0,
            CONCAT22(puVar3, 0x2),
            in_stack_0000fc90,
            in_stack_0000fdb4,
            in_stack_0000fdba,
            in_stack_0000fdbe,
        );
        pass1_1010_5f4c((paVar2 >> 0x10), paVar2, CONCAT22(uVar4, puVar3));
        if (local_102[0] != '\0') {
            message_box_op_1008_12dc(param_3, CONCAT22(0x1050, local_102));
        }
    }
    return;
}


pub fn pass1_1008_377e(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_37e4(mut param_1: u32, param_2: u8) -> u32 {
    cleanup_ui_op_1008_0618(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1008_392e(param_1: *mut u16, mut param_2: u16) -> *mut u16 {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x3aa8;
    (iVar1 + 0x2) = 0x1008;
    (iVar1 + 0x4) = param_2;
    *param_1 = 0x3ab0;
    (iVar1 + 0x2) = 0x1008;
    *param_1 = 0x3aa0;
    (iVar1 + 0x2) = 0x1008;
    return param_1;
}
pub fn pass1_1008_397a(param_1: *mut astruct_452) {
    let mut iVar1: *mut astruct_452;
    let mut uVar1: *mut astruct_452;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field0_0x0 = 0x3aa0;
    iVar1.field1_0x2 = 0x1008;
    param_1.field0_0x0 = 0x3ab0;
    iVar1.field1_0x2 = 0x1008;
    param_1.field0_0x0 = 0x389a;
    iVar1.field1_0x2 = 0x1008;
    return;
}

pub fn pass1_1008_3ab8(param_1: *mut Struct20) -> *mut Struct20 {
    let mut iVar1: *mut Struct20;
    let mut uVar1: u16;

    set_struct_1008_687a(param_1, 0x0);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field164_0xde = 0;
    param_1.offset_0x0 = 0x3b46;
    iVar1.base_0x2 = 0x1008;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.field60_0x5b)),
        s_SOLDefaultWindowClass_1050_01fe,
    );
    return param_1;
}

pub fn pass1_1008_3bd6(
    mut param_1: u32,
    param_2: *mut Struct57,
    param_3: *mut Struct57,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: u16,
    mut param_7: u32,
    mut param_8: u32,
    mut param_9: u16,
    mut param_10: u16,
    mut param_11: u16,
    mut param_12: u16,
    mut param_13: u16,
    mut param_14: u16,
) {
    mixed_struct_op_1040_8fb8(
        param_1,
        CONCAT22(param_3, param_2),
        param_4,
        NULL,
        param_6,
        param_7,
        (param_7 >> 0x10),
        param_8,
        (param_8 >> 0x10),
        param_9,
    );
    CONCAT22(param_3, param_2) = 0x3cfc;
    param_2.field1_0x2 = 0x1008;
    param_2.field_0x36 = 0;
    param_2.field21_0x26 = 0;
    pass1_1040_9252(CONCAT22(param_3, param_2));
    create_window_1040_92dc(CONCAT22(param_3, param_2));
    mov_update_win_1040_93aa(CONCAT22(param_3, param_2), param_5, (param_5 >> 0x10));
    return;
}

pub fn pass1_1008_3e0e(param_1: *mut StructA) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x4) != 0) {
        ppcVar1 = ((param_1 + 0x4) + 0x4);
        (**ppcVar1)();
    }
    return;
}

pub fn pass1_1008_3e38(param_1: *mut Struct19) -> *mut u16 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.offset_0x0 = 0;
    (param_1 + 0x2) = 0;
    (param_1 + 0x4) = 0;
    return &param_1.offset_0x0;
}

pub fn pass1_1008_3e54(
    param_1: *mut u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
) -> *mut u16 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_1 = param_4;
    (param_1 + 0x2) = param_3;
    (param_1 + 0x4) = param_2;
    return param_1;
}

pub fn pass1_1008_3e76(
    param_1: *mut u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_1 = param_4;
    (param_1 + 0x2) = param_3;
    (param_1 + 0x4) = param_2;
    return;
}


pub fn pass1_1008_3e94(param_1: *mut Struct1001, param_2: *mut u16, mut param_3: *mut c_char) {
    param_3 = param_1.field_0x1;
    unsafe { *param_2 = param_1.field_0x2; }
}

pub fn pass1_1008_3eb4(
    param_1: *mut astruct_615,
    param_2: *mut u16,
    param_3: *mut u16,
    param_4: *mut u16,
) {
    let mut uVar1: u16;

    *param_4 = param_1;
    uVar1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 2);
    *param_2 = (param_1 + 0x4);
    return;
}
pub fn pass1_1008_3ee2(param_1: *mut i16, param_2: *mut i16) {
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uVar4: u16;

    iVar1 = *param_2 - *param_1;
    if (iVar1 < 0x0) {
        iVar1 = -iVar1;
    }
    *param_1 = iVar1 + 1;
    uVar3 = (param_2 >> 0x10);
    uVar4 = (param_1 >> 0x10);
    iVar2 = param_1;
    iVar1 = (param_2 + 0x2) - (iVar2 + 2);
    if (iVar1 < 0x0) {
        iVar1 = -iVar1;
    }
    (iVar2 + 0x2) = iVar1 + 1;
    iVar1 = (param_2 + 0x4) - (iVar2 + 0x4);
    if (iVar1 < 0x0) {
        iVar1 = -iVar1;
    }
    (iVar2 + 0x4) = iVar1 + 1;
    return;
}
pub fn pass1_1008_3f32(param_1: *mut i16, param_2: *mut i16) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut uVar3: u16;

    *param_1 = *param_1 + *param_2;
    uVar2 = (param_2 >> 0x10);
    uVar3 = (param_1 >> 0x10);
    piVar1 = (param_1 + 2);
    *piVar1 = *piVar1 + (param_2 + 2);
    piVar1 = (param_1 + 0x4);
    *piVar1 = *piVar1 + (param_2 + 0x4);
    return;
}
pub fn pass1_1008_3f62(param_1: *mut u16, param_2: *mut u16) {
    let mut uVar1: u16;
    let mut uVar2: u16;

    *param_1 = *param_2;
    uVar1 = (param_2 >> 0x10);
    uVar2 = (param_1 >> 0x10);
    (param_1 + 0x2) = (param_2 + 2);
    (param_1 + 0x4) = (param_2 + 0x4);
    return;
}
pub fn struct_op_1008_3f92(param_1: *mut astruct_76, param_2: *mut c_char) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: *mut astruct_76;
    let mut uVar2: u16;

    struct_op_1008_56b4(param_1);
    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    iVar2.field3_0x6 = 0;
    iVar2.field5_0xa = 0;
    iVar2.field7_0xe = 0;
    iVar2.field8_0x10 = 0;
    iVar2.field9_0x14 = 0;
    iVar2.field11_0x18 = 0;
    iVar2.field13_0x1c = 0;
    param_1.offset_0x0 = &PTR_LOOP_1050_48de;
    iVar2.base_0x2 = 0x1008;
    if (param_2.is_null()) {
        return;
    }
    ppcVar1 = (param_2 + 0x8);
    (**ppcVar1)();
    struct_op_1008_4214(param_1, param_2);
    pass1_1008_47cc(param_1);
    pass1_1008_4834(param_1);
    return;
}
