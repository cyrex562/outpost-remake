use crate::unk::block_1010_6000::{pass1_1010_6ca2, switch_1010_6646};
use crate::unk::block_1010_7000::{pass1_1010_7818, ui_op_1010_79aa, unk_win_op_1010_7300};

pub fn struct_1010_5f1e(mut param_1: u16, param_2: *mut astruct_73, param_3: *mut c_char) {
    let mut uVar1: u16;
    let mut iVar3: *mut astruct_73;
    let mut uVar3: *mut astruct_73;

    uVar3 = (param_2 >> 0x10);
    iVar3 = param_2;
    fn_ptr_1000_17ce(iVar3.field22_0x16);
    uVar1 = str_op_1008_60e8(param_1, param_3);
    iVar3.field22_0x16 = uVar1;
    (&iVar3.field22_0x16 + 0x2) = param_1;
    return;
}

pub fn pass1_1010_5f4c(mut param_1: u16, param_2: *mut astruct_484, param_3: *mut c_char) {
    let mut uVar1: u16;
    let mut iVar3: *mut astruct_484;
    let mut uVar2: *mut astruct_484;

    uVar2 = (param_2 >> 0x10);
    iVar3 = param_2;
    fn_ptr_1000_17ce(*&iVar3.field18_0x12);
    uVar1 = str_op_1008_60e8(param_1, param_3);
    iVar3.field18_0x12 = uVar1;
    iVar3.field19_0x14 = param_1;
    return;
}

pub fn pass1_1010_5f7a(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
) -> u32 {
    let mut iVar1: i16;

    iVar1 = param_4 * 0x8 + param_1;
    if (((iVar1 + 0x26) == 0) && ((iVar1 + 0x28) == 0)) {
        return 0x0;
    }
    return CONCAT22(param_2, param_4 * 0x8 + param_1 + 0x22);
}

pub fn pass1_1010_5fb0(
    mut param_1: u32,
    mut param_2: u16,
    param_3: *mut u32,
    mut param_4: u16,
    mut param_5: i16,
) {
    let mut uVar1: u16;
    let mut iVar1: *mut astruct_656;

    uVar1 = (param_1 >> 0x10);
    iVar1 = (param_1 + param_5 * 0x8);
    iVar1.field34_0x22 = *param_3;
    iVar1.field35_0x26 = param_3[0x1];
    return;
}

pub fn pass1_1010_5fd8(mut param_1: u16, param_2: *mut astruct_485, param_3: *mut c_char) {
    let mut uVar1: u16;
    let mut iVar3: *mut astruct_485;
    let mut uVar2: *mut astruct_485;

    uVar2 = (param_2 >> 0x10);
    iVar3 = param_2;
    fn_ptr_1000_17ce(*&iVar3.field104_0x68);
    uVar1 = str_op_1008_60e8(param_1, param_3);
    iVar3.field104_0x68 = uVar1;
    iVar3.field105_0x6a = param_1;
    return;
}

pub fn pass1_1010_6006(mut param_1: u16, param_2: *mut astruct_486, param_3: *mut c_char)

{
    let mut uVar1: u16;
    let mut iVar3: *mut astruct_486;
    let mut uVar2: *mut astruct_486;

    uVar2 = (param_2 >> 0x10);
    iVar3 = param_2;
    fn_ptr_1000_17ce(*&iVar3.field108_0x6c);
    uVar1 = str_op_1008_60e8(param_1, param_3);
    iVar3.field108_0x6c = uVar1;
    iVar3.field109_0x6e = param_1;
    return;
}


pub fn pass1_1010_60b4() -> u16

{
    return 0x1;
}


pub fn pass1_1010_60ba() -> u16

{
    return 0x1;
}


pub fn pass1_1010_60c0() -> u16

{
    return 0x1;
}


pub fn pass1_1010_60c6() -> u16

{
    return 0x1;
}

pub fn pass1_1010_60cc(mut param_1: u16, param_2: *mut astruct_487, param_3: *mut c_char)

{
    let mut uVar1: u16;
    let mut iVar3: *mut astruct_487;
    let mut uVar2: *mut astruct_487;

    uVar2 = (param_2 >> 0x10);
    iVar3 = param_2;
    fn_ptr_1000_17ce(*&iVar3.field26_0x1a);
    uVar1 = str_op_1008_60e8(param_1, param_3);
    iVar3.field26_0x1a = uVar1;
    iVar3.field27_0x1c = param_1;
    return;
}


pub fn pass1_1010_60fa(mut param_1: u32)

{
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x7e) = 0x1;
    (iVar1 + 0x7c) = (iVar1 + 0x20);
    (iVar1 + 0x20) = 0x1;
    return;
}


pub fn pass1_1010_6118(mut param_1: u32)

{
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((iVar1 + 0x7e) != 0) {
        (iVar1 + 0x20) = (iVar1 + 0x7c);
    }
    return;
}

pub fn pass1_1010_62a4(param_1: *mut astruct_488, param_2: u8)

{
    let mut uVar2: *mut astruct_488;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    uVar2 = param_1;
    param_1 = 0x6322;
    uVar2.field2_0x2 = 0x1010;
    fn_ptr_1000_17ce(uVar2.field3_0x4);
    param_1 = 0x389a;
    uVar2.field2_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}


pub fn pass1_1010_62ec(param_1: u8, mut param_2: u16, param_3: *mut StructD, param_4: u8) -> *mut StructD

{
    write_private_profile_str_1010_5b10(param_2, param_3);
    if ((param_4 & 1) != 0) {
        fn_ptr_1000_17ce(param_3);
    }
    return param_3;
}

pub fn pass1_1010_6566(mut param_1: u32, mut param_2: u16, mut param_3: u16, mut param_4: u16)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut local_4: i16;

    uVar1 = param_1;
    uVar2 = (param_1 >> 0x10);
    switch_1010_6646(uVar1, uVar2, CONCAT22(0x1050, &local_4), param_4);
    if (local_4 != 0) {
        (uVar1 + local_4) = param_3;
        (uVar1 + local_4 + 0x2) = param_2;
    }
    return;
}


pub fn pass1_1010_659a(mut param_1: u32, mut param_2: u16) -> i16

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut local_4: i16;

    uVar1 = param_1;
    uVar2 = (param_1 >> 0x10);
    switch_1010_6646(uVar1, uVar2, CONCAT22(0x1050, &local_4), param_2);
    if (local_4 == 0) {
        return 0x0;
    }
    return (uVar1 + local_4) - (uVar1 + local_4 + 2);
}


pub fn pass1_1010_65d0(mut param_1: u32, mut param_2: u16) -> u16

{
    let mut uVar1: u16;
    let mut local_4: i16;

    uVar1 = (param_1 >> 0x10);
    switch_1010_6646(param_1, uVar1, CONCAT22(0x1050, &local_4), param_2);
    if (local_4 == 0) {
        return 0x0;
    }
    return (param_1 + local_4 + 2);
}


pub fn pass1_1010_6604(mut param_1: u32, mut param_2: u16)

{
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut local_4: i16;

    uVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    switch_1010_6646(uVar2, uVar3, CONCAT22(0x1050, &local_4), param_2);
    if (local_4 != 0) {
        iVar1 = (uVar2 + local_4 + 2);
        (uVar2 + local_4) = (uVar2 + local_4);
        (uVar2 + local_4 + 0x2) = iVar1 + 1;
        pass1_1010_1f62((param_1 & 0xffff | uVar3 << 0x10), 0x15);
    }
    return;
}


pub fn switch_1010_6646(mut param_1: u16, mut param_2: u16, param_3: *mut u16, mut param_4: u16)

{
//   switch(param_4)
    match param_4 {
//   0x83 =>
        0x83 => {
            *param_3 = 0xa;
        }
        // break;
//   0x84 =>
        0x84 => {
            *param_3 = 0xe;
        }
        // break;
//   0x85 =>
        0x85 => {
            *param_3 = 0x12;
        }
        // break;
//   0x86 =>
        0x86 => {
            *param_3 = 0x16;
        }
        // return;
//   0x87 =>
        0x87 => {
            *param_3 = 0x1a;
        }
        // return;
//   0x88 =>
        0x88 => {
            *param_3 = 0x1e;
        }
        // return;
//   0x89 =>
        0x89 => {
            *param_3 = 0x22;
        }
        // return;
//   _ =>
        _ => {
            *param_3 = 0;
        }
        // return;
    }
    return;
}

pub fn pass1_1010_6bb2(param_1: *mut u16)

{
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut puStack14: *mut u16;

    uVar7 = (param_1 >> 0x10);
    uVar6 = param_1;
    *param_1 = 0x7e28;
    (uVar6 + 0x2) = 0x1010;
    (uVar6 + 0xa) = 0x7e38;
    (uVar6 + 0xc) = 0x1010;
    puVar1 = (uVar6 + 0x1c);
    uVar3 = (uVar6 + 0x1e);
    if ((uVar3 | puVar1) != 0) {
        ppcVar2 = *puVar1;
        (**ppcVar2)();
    }
    (uVar6 + 0x1c) = 0;
    if ((uVar6 + 0x14) != 0) {
        uVar3 = uVar7 | uVar6;
        if (param_1.is_null()) {
            uVar5 = 0;
        } else {
            uVar3 = uVar6 + 0xa;
            uVar5 = uVar7;
        }
        pass1_1010_1ea6((uVar6 + 0x14), CONCAT22(uVar5, uVar3));
    }
    if ((uVar6 + 0x22) != 0) {
        uVar3 = uVar7 | uVar6;
        if (param_1.is_null()) {
            uVar5 = 0;
        } else {
            uVar3 = uVar6 + 0xa;
            uVar5 = uVar7;
        }
        pass1_1010_1ea6((uVar6 + 0x22), CONCAT22(uVar5, uVar3));
    }
    (uVar6 + 0x14) = 0;
    (uVar6 + 0x22) = 0;
    if (param_1.is_null()) {
        iVar4 = 0;
        uVar7 = 0;
    } else {
        iVar4 = uVar6 + 0xa;
    }
    puStack14 = CONCAT22(uVar7, iVar4);
    *puStack14 = 0x389a;
    (iVar4 + 0x2) = 0x1008;
    pass1_1010_1d80(param_1);
    return;
}


pub fn pass1_1010_6cf8(mut param_1: u16, mut param_2: u16, mut param_3: u16, mut param_4: u32, mut param_5: i16) -> u16

{
    let mut uVar1: u16;

//   switch(param_5)
    match param_5 {
//   0x1 =>
        0x1 => {
            pass1_1010_715c(param_1, param_2, param_4, 1);
            send_msg_1010_7c9e(param_4, 0x12);
            return 0x1;
        }
//   _ =>
        _ => {
            return 0x0;
        }
//   0x4 =>
        0x4 => {
            uVar1 = 0x2;
        }
        // break;
//   case 0x5:
        0x5 => {
            uVar1 = 0x3;
        }
        // break;
//   case 0x6:
        0x6 => {
            uVar1 = 0x4;
        }
        // break;
//   case 0x7:
        0x7 => {
            uVar1 = 0x5;
        }
        // break;
//   case 0x9:
        0x9 => {
            pass1_1010_715c(param_1, param_2, param_4, 0x6);
        }
//   case 0x2e:
        0x2e => {
            uVar1 = 0x38;
        }
        // break;
//   case 0xa:
//   case 0x80:
        0xa | 0x80 => {
            uVar1 = 0x2d;
        }
        // break;
//   case 0xb:
        0xb => {
            uVar1 = 0x7;
        }
        // break;
//   case 0xc:
//   case 0x17:
//   case 0x18:
//   case 0x19:
//   case 0x21:
//   case 0x75:
//   case 0x81:
        0xc | 0x17 | 0x18 | 0x19 | 0x21 | 0x75 | 0x81 => {
            if (param_5 == 0x75) {
                pass1_1010_715c(param_1, param_2, param_4, 0x8);
                pass1_1010_715c(param_1, param_2, param_4, 0x9);
            }
            uVar1 = pass1_1010_6ca2(param_2, param_4, 0x7);
            if (uVar1 != 0) {
                pass1_1010_715c(uVar1, param_2, param_4, 0x10);
            }
            param_1 = pass1_1010_6ca2(param_2, param_4, 0x3);
            if (param_1 != 0) {
                pass1_1010_715c(param_1, param_2, param_4, 0x11);
            }
            if (param_5 == 0x21) {
                pass1_1010_715c(param_1, param_2, param_4, 0x14);
            }
            if (param_5 != 0xc) {
                return 0x1;
            }
            uVar1 = 0x9;
        }
// TODO: goto code_r0x10106d4c;
//   case 0xe:
        0xe => {
            uVar1 = 0xc;
        }
// TODO: goto code_r0x10106d4c;
//   case 0x10:
//   case 0x11:
//   case 0x13:
        0x10 | 0x11 | 0x13 => {
            uVar1 = 0xd;
        }
        // break;
//   case 0x12:
        0x12 => {
            uVar1 = 0xe;
        }
        // break;
//   case 0x1b:
//   case 0x1f:
//   case 0x5b:
//   case 0x78:
//   case 0x7e:
//   case 0x7f:
        0x1b | 0x1f | 0x5b | 0x78 | 0x7e | 0x7f => {
            if (param_5 == 0x7e) {
                pass1_1010_715c(param_1, param_2, param_4, 0x2c);
            }
            if (param_5 == 0x5b) {
                pass1_1010_715c(param_1, param_2, param_4, 0x38);
            }
            if (param_5 == 0x1f) {
                pass1_1010_715c(param_1, param_2, param_4, 0x3f);
            }
            if (param_5 == 0x7f) {
                pass1_1010_715c(param_1, param_2, param_4, 0x42);
            }
            param_1 = pass1_1010_6ca2(param_2, param_4, 0x5);
            if ((param_1 == 0) && (param_1 = pass1_1010_6ca2(param_2, param_4, 0x5), param_1 == 0)) {
                return 0x1;
            }
            uVar1 = 0x37;
        }
        // break;
//   case 0x1d:
//   case 0x2a:
//   case 0x2c:
//   case 0x3c:
//   case 0x3d:
//   case 0x4b:
//   case 0x53:
//   case 0x54:
//   case 0x55:
//   case 0x5a:
        0x1d | 0x2a | 0x2c | 0x3c | 0x3d | 0x4b | 0x53 | 0x54 | 0x55 | 0x5a => {
            uVar1 = pass1_1010_6ca2(param_2, param_4, 0x2);
            if (uVar1 != 0) {
                pass1_1010_715c(uVar1, param_2, param_4, 0x12);
            }
            uVar1 = pass1_1010_6ca2(param_2, param_4, 0x8);
            if (uVar1 != 0) {
                pass1_1010_715c(uVar1, param_2, param_4, 0x1a);
            }
            if (param_5 == 0x2c) {
                pass1_1010_715c(uVar1, param_2, param_4, 0x1d);
            }
            param_1 = pass1_1010_6ca2(param_2, param_4, 0x2);
            if (param_1 == 0) {
                return 0x1;
            }
            uVar1 = 0x1c;
        }
        // break;
//   case 0x22:
        0x22 => {
            uVar1 = 0x15;
        }
        // break;
//   case 0x25:
        0x25 => {
            uVar1 = 0x16;
        }
        // break;
//   case 0x26:
        0x26 => {
            pass1_1010_715c(param_1, param_2, param_4, 0x17);
        }
//   case 0x1e:
        0x1e => {
            uVar1 = 0x13;
        }
        // break;
//   case 0x27:
        0x27 => {
            uVar1 = 0x18;
        }
        // break;
//   case 0x29:
        0x29 => {
            uVar1 = 0x19;
        }
        // break;
//   case 0x2b:
        0x2b => {
            uVar1 = 0x1b;
        }
        // break;
//   case 0x2f:
//   case 0x36:
        0x2f | 0x36 => {
            param_1 = pass1_1010_6ca2(param_2, param_4, 0x2);
            if (param_1 == 0) {
                return 0x0;
            }
            uVar1 = 0x1e;
        }
        // break;
//   case 0x30:
        0x30 => {
            uVar1 = 0x1f;
        }
        // break;
//   case 0x31:
        0x31 => {
            uVar1 = 0x35;
        }
        // break;
//   case 0x33:
        0x33 => {
            uVar1 = 0x21;
        }
        // break;
//   case 0x34:
        0x34 => {
            uVar1 = 0x22;
        }
        // break;
//   case 0x35:
        0x35 => {
            pass1_1010_715c(param_1, param_2, param_4, 0x23);
        }
        0x65 | 0x66 | 0x6b | 0x6c | 0x6d | 0x6e | 0x6f => {
            uVar1 = 0x34;
        }
        // break;
        0x38 => {
            pass1_1010_715c(param_1, param_2, param_4, 0x24);
            uVar1 = 0x3d;
        }
        // break;
        0x39 => {
            uVar1 = 0x25;
        }
        // break;
        0x3e => {
            pass1_1010_715c(param_1, param_2, param_4, 0x26);
            pass1_1010_715c(param_1, param_2, param_4, 0x28);
            uVar1 = 0x27;
        }
        // break;
        0x40 => {
            uVar1 = 0x2a;
        }
        // break;
        0x41 => {
            uVar1 = 0x39;
        }
        // break;
        0x42 => {
            uVar1 = 0x3a;
        }
        // break;
        0x44 => {
            uVar1 = 0x36;
        }
        // break;
        0x45 => {
            uVar1 = 0x3b;
        }
        // break;
        0x49 => {
            uVar1 = 0x29;
        }
        // break;
        0x50 => {
            uVar1 = 0x2b;
        }
        // break;
        0x56 => {
            pass1_1010_715c(param_1, param_2, param_4, 0x3c);
            uVar1 = 0x3e;
        }
        // break;
        0x5d => {
            pass1_1010_715c(param_1, param_2, param_4, 0x2f);
            uVar1 = 0x40;
        }
        // break;
        0x5e | 0x60 => {
            uVar1 = 0x2f;
        }
        // break;
        0x5f => {
            pass1_1010_715c(param_1, param_2, param_4, 0x34);
            uVar1 = 0x41;
        }
        // break;
        0x61 => {
            uVar1 = 0x30;
        }
        // break;
        0x63 => {
            uVar1 = 0x31;
        }
        // break;
        0x64 => {
            uVar1 = 0x24;
        }
        // break;
        0x68 => {
            uVar1 = 0x32;
        }
        // break;
        0x69 => {
            uVar1 = 0x33;
        }
        // break;
        0x76 => {
            uVar1 = 0xa;
// code_r0x10106d4c:
            pass1_1010_715c(param_1, param_2, param_4, uVar1);
            uVar1 = 0xb;
        }
    };
    pass1_1010_715c(param_1, param_2, param_4, uVar1);
    return 0x1;
}


pub fn pass1_1010_71d6(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: i16,
    param_5: *mut u16,
    mut param_6: u16,
) {
    let mut uVar1: u16;
    let mut iVar2: i16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut in_register_0000000a: u16;
    let mut uVar9: u32;
    let mut uVar10: u16;
    let mut paVar11: *mut astruct_15;
    let mut uStack20: u16;
    let mut uStack14: u16;
    let mut uStack6: u32;

    uVar10 = (param_3 >> 0x10);
    uVar3 = (param_3 + 0x14);
    pass1_1010_ad22(param_2, uVar3, (uVar3 >> 0x10), *param_5);
    uStack6 = CONCAT22(param_2, param_1);
    if ((param_2 | param_1) == 0) {
        return;
    }
    paVar11 = struct_op_1030_73a8(CONCAT22(param_2, param_1), param_1, param_2 | param_1);
    uVar8 = (paVar11 >> 0x10);
    uVar4 = paVar11;
    if (((uVar8 | uVar4) != 0) && ((uVar4 + 0x1c) == 0x8000002)) {
        return;
    }
    uVar3 = (param_1 + 0x2e);
    uVar1 = (param_1 + 0x30);
    uVar9 = CONCAT22(in_register_0000000a, uVar1);
    uStack14 = uVar3;
    if (((uVar1 | uStack14) != 0) && ((uStack14 + 0x200) == 0x8000002)) {
        return;
    }
    uVar3 = (param_3 + 0x14);
    uVar6 = pass1_1010_b028(uVar3, (uVar3 >> 0x10), paVar11);
    iVar2 = (uVar4 + 0x12);
    iVar5 = iVar2;
    if ((iVar2 != 0x4) && (iVar5 = param_4, iVar2 == 0x7)) {
        param_4 = 0x5;
        iVar5 = param_4;
    }
    param_4 = iVar5;
    uVar7 = param_4 - 0x2;
    if (uVar7 != 0) {
        if (param_4 == 0x3) {
            uVar7 = uVar6 - 0xb;
            if ((uVar7 == 0) || (uVar7 = uVar6 - 0x37, uVar7 == 0)) {
                uStack20 = 0xb;
            } else {
                uStack20 = 0xa;
            }
            // TODO: goto LAB_1010_72a7;
        }
        uVar7 = param_4 - 0x4;
        if (uVar7 == 0) {
            uStack20 = 0x17;
            // TODO: goto LAB_1010_72a7;
        }
        uVar7 = param_4 - 0x5;
        if (uVar7 != 0) {
            uVar7 = pass1_1010_7818(param_3, paVar11);
            uStack20 = uVar7;
            // TODO: goto LAB_1010_72a7;
        }
    }
    uStack20 = 0xc; //
    // LAB_1010_72a7:
    if (uStack20 == 0) {
        return;
    }
    ui_op_1010_79aa(param_3, 0x0, uStack6);
    if (uVar7 == 0) {
        unk_win_op_1010_7300(uVar9, param_3, 0x0, uStack20, uStack6);
    }
    return;
}
