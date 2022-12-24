use crate::{draw_ops, win_ui};

pub unsafe fn pass1_1020_3540(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
    param_5: *mut u16,
) {
    let mut iVar1: i16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar2: *mut Struct279;
    let mut iStack18: i16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut local_6: i16;
    let mut local_4: i16;

    paVar2 = CONCAT22(in_register_0000000a, param_1);
    pass1_1008_3e94(
        param_5,
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    if (param_4 == 0) {
        iStack12 = 0x3;
        iStack10 = 0x42a6;
    } else if (param_4 == 1) {
        iStack12 = 0x4;
        iStack10 = s_SITEICON_1050_428d + 0x9;
    } else {
        if (param_4 != 0x2) {
            return;
        }
        iStack12 = 0x4;
        iStack10 = 0x42b2;
    }
    iVar1 = iStack12 << 0x2;
    mem_op_1000_179c(iVar1, paVar2);
    for iStack18 in 0..iStack12 {
        iVar2 = (iStack18 * 0x4);
        (iVar2 + iVar1) = (iVar2 + iStack10) + local_4;
        (iVar2 + iVar1 + 0x2) = (iVar2 + iStack10 + 0x2) + local_6;
    }
    return;
}


pub unsafe fn struct_1020_3644(
    mut param_1: u32,
    param_2: *mut StructA,
    mut param_3: u16,
    mut param_4: u32,
    mut param_5: u16,
    mut param_6: u16,
) {
    let iVar2: *mut StructA;
    let mut in_buf_len_5: u16;
    let mut in_stack_0000fe52: u16;
    let mut in_stack_0000ff76: u16;
    let mut in_stack_0000ff7c: u16;
    let mut in_stack_0000ff80: u16;
    let mut iVar1: *mut Struct270;

    struct_1020_790e(&param_2.field0_0x0, NULL, param_3, param_4);
    in_buf_len_5 = (param_2 >> 0x10);
    iVar2 = param_2;
    iVar2[0x1].field20_0x26 = 0x389a;
    iVar2[0x1].field21_0x28 = 0x1008;
    iVar2[0x1].field20_0x26 = 0x3aa8;
    iVar2[0x1].field21_0x28 = 0x1008;
    iVar2[0x1].field29_0x34 = 0;
    iVar2[0x1].field37_0x3e = 0;
    iVar2[0x1].field38_0x42 = 0;
    param_2.field0_0x0 = 0x3d08;
    iVar2.field1_0x2 = 0x1020;
    iVar2[0x1].field20_0x26 = 0x3d9c;
    iVar2[0x1].field21_0x28 = 0x1020;
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x3ff,
        &iVar2.field5_0xa,
        in_buf_len_5,
    );
    unk_str_op_1000_3d3e(
        (param_2 & 0xffff0000 | ZEXT24(&iVar2.field60_0x5b)),
        s_VrMode_1050_42ca,
    );
    iVar2.field140_0xac = 0x44c00000;
    win_ui::window_op_1020_38aa(
        param_1,
        (param_2 & 0xffff | in_buf_len_5 << 0x10),
        param_6,
        param_5,
        in_stack_0000ff7c,
        in_stack_0000ff80,
        in_stack_0000fe52,
        in_stack_0000ff76,
    );
    return;
}


pub unsafe fn pass1_1020_3c32(mut param_1: i16, mut param_2: u16, mut param_3: u16) {
    let mut cVar1: u8;
    let mut iVar2: i16;

    if (param_3 == 0xf5) {
        iVar2 = 0x1; //
                     // LAB_1020_3c52:
        pass1_1018_1b02((param_1 + 0xfa), iVar2);
        return;
    }
    if ((param_3 < 0xf6) && (cVar1 = param_3, cVar1 != '\0')) {
        if (cVar1 == '\x01' || cVar1 == '\x02') {
            return;
        }
        if (cVar1 == -0xc) {
            iVar2 = 0;
            // TODO: goto LAB_1020_3c52;
        }
    }
    pass1_1020_3c32(param_1, param_2, param_3);
    return;
}

pub unsafe fn pass1_1020_3c8c(mut param_1: u32, mut param_2: u32) {
    pt_in_rect_1018_1bda((param_1 + 0xfa), param_2, (param_2 >> 0x10));
    return;
}


// WARNING: Instruction at (ram,0x10203dab) overlaps instruction at (ram,0x10203da8)
//




// WARNING: Unable to use type for symbol uVar4
