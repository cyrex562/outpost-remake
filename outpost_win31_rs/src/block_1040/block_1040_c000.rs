use std::ptr::null_mut;
use crate::block_1000::block_1000_1000::fn_ptr_1000_17ce;
use crate::block_1010::block_1010_0000::pass1_1010_038e;
use crate::block_1010::block_1010_2000::mixed_1010_20ba;
use crate::block_1010::block_1010_8000::load_string_1010_84e0;
use crate::block_1010::block_1010_a000::pass1_1010_a5ca;
use crate::block_1018::block_1018_5000::pass1_1018_50ea;
use crate::block_1038::block_1038_a000::pass1_1038_af40;
use crate::block_1038::block_1038_b000::pass1_1038_b6e0;
use crate::block_1040::block_1040_9000::draw_text_1040_94fc;
use crate::block_1040::block_1040_b000::{destroy_window_1040_b726, pass1_1040_b54a, struct_1040_b082, unk_draw_op_1040_b0f8};
use crate::globals::PTR_LOOP_1050_1040;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_37::Struct37;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_d::StructD;
use crate::utils::CONCAT22;
use crate::win_ui;
use crate::winbase::{GetDlgItemInt16, MessageBox16, SetDlgItemInt16};
use crate::windef::{HDC16, LRESULT};

// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2


// WARNING: Unable to use type for symbol uVar4
pub unsafe fn draw_line_1040_c302(param_1: *mut astruct_772, param_2: HDC16)

{
    let mut uVar3: u32;
    let mut uVar5: u16;
    let mut iVar7: *mut astruct_794;
    let mut iVar6: *mut astruct_793;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uVar2: u32;
    let mut uVar4: u32;
    let mut iVar1: i16;
    let mut uVar1: u32;

    uVar6 = (param_1 >> 0x10);
    uVar4 = (param_1 + 0x6);
    iVar1 = (uVar4 + 0x16);
    if (0x1 < iVar1) {
        uVar2 = (param_1 + 0x6);
        uVar5 = uVar2;
        uVar5 = uVar5 + 0x2a;
        uVar1 = (uVar2 & 0xffff0000 | uVar5);
        iVar7 = uVar1;
        iVar7 = &iVar7.field_0x1e;
        uVar7 = ((uVar1 & 0xffff0000) >> 0x10);
        MoveTo16(iVar7.field32_0x20 + iVar7.field34_0x24,
                 iVar7.field33_0x22 / 0x2 + (uVar1 & 0xffff0000 | ZEXT24(iVar7)), param_2);
        uVar3 = (uVar5 + iVar1 * 0x4 - 0x4);
        iVar6 = uVar3;
        iVar6 = &iVar6.field_0x1e;
        uVar3 &= 0xffff0000;
        uVar8 = (uVar3 >> 0x10);
        LineTo16(iVar6.field32_0x20, iVar6.field33_0x22 / 0x2 + (uVar3 | ZEXT24(iVar6)), param_2);
    }
    return;
}


// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar5
// WARNING: Unable to use type for symbol uVar4
// WARNING: Unable to use type for symbol uVar6
// WARNING: Unable to use type for symbol uVar7
pub unsafe fn draw_op_1040_c38e(param_1: *mut astruct_772)

{
    let mut iVar1: i16;
    let mut uVar8: u32;
    let mut iVar5: i16;
    let mut iVar11: i16;
    let mut y1: i16;
    let mut iVar12: i16;
    let mut in_DX: u16;
    let mut iVar10: *mut astruct_772;
    let mut uVar10: u16;
    let mut uVar9: u16;
    let mut uVar11: u16;
    let mut unaff_SS: u16;
    let mut DVar10: u32;
    let mut DVar9: u32;
    let mut in_stack_00000008: HDC16;
    let mut iStack26: i16;
    let mut x3: i16;
    let mut y5: i16;
    let mut x2: i16;
    let mut y4: i16;
    let mut uVar2: u32;
    let mut uVar1: u32;
    let mut uVar5: u32;
    let mut x1: *mut i16;
    let mut uVar4: u32;
    let mut uVar3: u32;
    let mut uVar6: u32;
    let mut uVar7: u32;

    uVar10 = (param_1 >> 0x10);
    iVar10 = param_1;
    uVar2 = iVar10.field5_0x6;
    iVar1 = (uVar2 + 0x18);
    if ((iVar1 != 0) && (uVar4 = iVar10.field5_0x6, (uVar4 + 0x16) != 0)) {
        iVar5 = iVar1;
        pass1_1010_2ee2(iVar10.field5_0x6);
        for iStack26 in 0..iVar1 {
            uVar3 = (iStack26 * 0x4 + iVar5);
            iVar11 = uVar3;
            iVar11 = iVar11 + 0x1e;
            x1 = (uVar3 & 0xffff0000 | iVar11);
            uVar9 = ((uVar3 & 0xffff0000) >> 0x10);
            y1 = (iVar11 + 0x24) / 0x2 + (iVar11 + 0x20);
            MoveTo16(y1, *x1, in_stack_00000008);
            LineTo16(y1, *x1 - 0xf, in_stack_00000008);
            DVar10 = GetCurrentPosition16(in_stack_00000008);
            y5 = (DVar10 >> 0x10);
            x3 = DVar10;
            if (iStack26 == 0) {
                x2 = x3;
                y4 = y5;
            }
        }
        uVar6 = iVar10.field5_0x6;
        if ((uVar6 + 0x24) != 0) {
            y4 += -0xd;
        }
        uVar7 = iVar10.field5_0x6;
        if ((uVar7 + 0x26) != 0) {
            y5 += 0xd;
        }
        uVar8 = iVar10.field5_0x6;
        uVar5 = iVar10.field5_0x6;
        uVar1 = (uVar8 + (uVar5 + 0x16) * 0x4 + 0x26);
        iVar12 = uVar1;
        iVar12 = iVar12 + 0x1e;
        uVar11 = ((uVar1 & 0xffff0000) >> 0x10);
        MoveTo16((iVar12 + 0x24) / 0x2 + (iVar12 + 0x20),
                 (iVar12 + 0x22) + (uVar1 & 0xffff0000 | iVar12), in_stack_00000008);
        LineTo16((iVar12 + 0x24) / 0x2 + (iVar12 + 0x20), x2, in_stack_00000008);
        DVar9 = GetCurrentPosition16(in_stack_00000008);
        DVar9 = (DVar9 >> 0x10);
        if (DVar9 < y4) {
            y4 = DVar9;
        }
        if (y5 < DVar9) {
            y5 = DVar9;
        }
        MoveTo16(y4, x2, in_stack_00000008);
        LineTo16(y5, x3, in_stack_00000008);
    }
    return;
}


pub unsafe fn pass1_1040_c54a(param_1: *mut astruct_65, mut param_2: u16, param_3: *mut u32, mut param_4: u16, mut param_5: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut iVar3: i16;
    let mut iVar2: *mut astruct_65;
    let mut uVar4: *mut astruct_65;
    let mut puVar4: *mut u32;
    let mut uVar5: u16;
    let mut uVar6: u32;

    iVar3 = (param_3 + 0x12) + 0xc8;
    uVar6 = 0;
    uVar5 = 0;
    ppcVar1 = (*param_3 + 0x14);
    puVar4 = param_3;
    (**ppcVar1)();
    mixed_struct_op_1040_8fb8(param_5, param_1, 0x0, CONCAT22(param_5, iVar3), puVar4, (puVar4 >> 0x10), uVar5,
                              uVar6, (uVar6 >> 0x10), param_4);
    uVar4 = (param_1 >> 0x10);
    iVar2 = param_1;
    iVar2[0x1].field13_0x1c = param_3;
    iVar2[0x1].field15_0x20 = 0;
    iVar2[0x1].field16_0x22 = param_2;
    param_1.field0_0x0 = 0xc9f2;
    iVar2.field1_0x2 = &PTR_LOOP_1050_1040;
    pass1_1040_c630((param_1 & 0xffff | ZEXT24(uVar4) << 0x10), param_5);
    return;
}

pub unsafe fn pass1_1040_c5ac(param_1: *mut StructD)

{
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut StructD;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.address_offset_field_0x0 = 0xc9f2;
    iVar4.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
    PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 - 0x1;
    if (PTR_LOOP_1050_5f02.is_null()) {
        puVar1 = iVar4.field5_0x8;
        uVar2 = iVar4.field6_0xa;
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        puVar1 = iVar4.field7_0xc;
        uVar2 = iVar4.field8_0xe;
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    mix_win_ui_op_1040_911e(param_1);
    return;
}


pub unsafe fn pass1_1040_c60e(param_1: *mut astruct_65) -> u16

{
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x42) != 0) {
        uVar1 = (param_1 + 0x42);
        return (uVar1 + 0x12);
    }
    return 0x0;
}



pub unsafe fn pass1_1040_c630(param_1: *mut astruct_65, mut param_2: u32)

{
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut iVar4: *mut astruct_65;
    let mut uVar5: u16;
    let mut unaff_CS: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar3 = &iVar4[0x1].field13_0x1c;
    if ((uVar3 + 0x12) != 0x71) {
        iVar4[0x1].field8_0x10 = 0x5;
        (iVar4 + 1).field0_0x0 = 0x5;
        iVar4[0x1].field1_0x2 = 0x5;
        uVar1 = iVar4[0x1].field8_0x10;
        iVar4[0x1].field5_0xa = uVar1;
        iVar4[0x1].field4_0x8 = uVar1;
        if (PTR_LOOP_1050_5f02.is_null()) {
            uVar4 = FUN_1010_830a(uVar1, param_2, unaff_CS, _u16_1050_14cc, 0xff);
            _PTR_LOOP_1050_5f04 = CONCAT22(param_2, uVar4);
            unaff_CS = 0x1010;
            uVar4 = FUN_1010_830a(uVar4, param_2, 0x1010, _u16_1050_14cc, 0x100);
            _PTR_LOOP_1050_5f08 = CONCAT22(param_2, uVar4);
        }
        PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + 1;
        iVar4.field4_0x8 = _PTR_LOOP_1050_5f04;
        iVar4.field6_0xc = _PTR_LOOP_1050_5f08;
        pass1_1040_9618((param_1 & 0xffff | uVar5 << 0x10));
        iVar4.field15_0x20 = 0;
        iVar4.field14_0x1e = 0xc8;
        iVar4.field16_0x22 = 0xa0;
        iVar4.field17_0x24 = iVar4[0x1].field3_0x6 + iVar4[0x1].field8_0x10;
        iVar4[0x1].field4_0x8 = iVar4[0x1].field8_0x10 * 0x3 + iVar4[0x1].field2_0x4;
        iVar4[0x1].field5_0xa = iVar4[0x1].field8_0x10;
        iVar4[0x1].field6_0xc = iVar4.field16_0x22 - iVar4[0x1].field8_0x10;
        (&iVar4[0x1].field10_0x14 + 0x2) = 0x25;
        uVar3 = param_1;
        ppcVar2 = (uVar3 + 0x4);
        (**ppcVar2)(unaff_CS, param_1);
        ppcVar2 = (uVar3 + 0x8);
        (**ppcVar2)(unaff_CS, param_1, uVar5);
    }
    return;
}



pub unsafe fn FUN_1040_c882()

{
    return;
}






pub unsafe fn pass1_1040_ca16(param_1: *mut u8, param_2: *mut Struct57, mut param_3: u16)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut iVar1: *mut Struct57;
    let mut unaff_BP: u16;
    let mut uVar2: u16;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_1);
    struct_1040_b082(param_2, CONCAT22(param_3, 0x1840));
    uVar2 = (param_2 >> 0x10);
    iVar1 = param_2;
    iVar1[0x1].field3_0x6 = _PTR_LOOP_1050_5f0c;
    iVar1[0x1].field5_0xa = 0;
    iVar1[0x1].field7_0xe = 0;
    iVar1[0x1].field8_0x10 = 0;
    param_2.field0_0x0 = 0xd07c;
    iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
    puVar3 = mixed_1010_20ba(paVar1, _u16_1050_0ed0, CONCAT22(unaff_BP, 0x3e), in_stack_0000fea6,
                             in_stack_0000ffca, in_stack_0000ffd0, in_stack_0000ffd4);
    iVar1[0x1].field5_0xa = puVar3;
    iVar1[0x1].field6_0xc = (puVar3 >> 0x10);
    return;
}



pub unsafe fn pass1_1040_ca74(param_1: *mut StructD)

{
    let mut uVar1: u16;
    let mut in_stack_0000ffde: u16;

    // uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0xd07c;
    param_1.address_offset_field_0x2 = 0x1040;
    pass1_1038_b6e0(0x5b7c, param_1.field3_0x6);
    PTR_LOOP_1050_5f10 = null_mut();
    unk_draw_op_1040_b0f8(in_stack_0000ffde, param_1);
    return;
}
