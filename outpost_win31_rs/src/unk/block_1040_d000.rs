use crate::unk::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::unk::block_1000_3000::pass1_1000_3cea;
use crate::unk::block_1010_2000::mixed_1010_20ba;
use crate::unk::block_1010_8000::pass1_1010_8ef2;
use crate::unk::block_1010_9000::{pass1_1010_9044, pass1_1010_9130, pass1_1010_91cc, pass1_1010_9210, struct_1010_9172};
use crate::unk::block_1010_c000::pass1_1010_c3c2;
use crate::unk::block_1018_5000::{pass1_1018_5732, pass1_1018_5742, pass1_1018_57d2};
use crate::unk::block_1030_8000::pass1_1030_8344;
use crate::unk::block_1038_a000::pass1_1038_af40;
use crate::unk::block_1038_b000::pass1_1038_b6e0;
use crate::unk::block_1040_b000::{pass1_1040_b54a, struct_1040_b082};
use crate::unk::block_1040_c000::pass1_1040_ca74;
use crate::draw_ops::draw_e::unk_draw_op_1040_b0f8;
use crate::globals::PTR_LOOP_1050_1040;
use crate::resources::load_string_1010_84e0;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_d::StructD;
use crate::utils::{CONCAT22, SUB42};
use crate::gui;
use crate::winapi16::{EnableWindow16, GetDlgItem16, MessageBox16, PostMessage16, SendDlgItemMessage16};
use crate::windef16::{HWND16, LRESULT};



pub fn pass1_1040_d0f8(param_1: *mut Struct57, mut param_2: u16, mut param_3: u16, param_4: *mut StructD, mut param_5: u16,
                              mut param_6: u16, mut param_7: u16, mut param_8: u16, mut param_9: u16, mut param_10: u16,
                              mut param_11: u16, mut param_12: u16, mut param_13: u16)

{
    struct_1040_b082(param_1, CONCAT22(param_2, 0x1845));
    // pstruct57_var7 = (param_1 >> 0x10);
    let mut pstruct57_var5 = param_1;
    pstruct57_var5.field3_0x6 = 0;
    pstruct57_var5.field5_0xa = _PTR_LOOP_1050_5f16;
    pstruct57_var5.field7_0xe = 0;
    pstruct57_var5.field9_0x12 = 0;
    param_1.field0_0x0 = 0xd8c4;
    pstruct57_var5.field1_0x2 = PTR_LOOP_1050_1040;
    let mut pu32_var8 = mixed_1010_20ba(param_4, _u16_1050_0ed0, CONCAT22(param_3, 0x47), param_6, param_10,
                                        param_11, param_12);
    let mut var5 = param_4 & 0xffff0000;
    pstruct57_var5.field3_0x6 = pu32_var8;
    let mut uVar2 = (pu32_var8 >> 0x10);
    pstruct57_var5.field4_0x8 = uVar2;
    let mut uVar9 = pass1_1018_5732(pu32_var8, uVar2, pstruct57_var5.field3_0x6, uVar2, pstruct57_var5.field5_0xa as u32);
    let mut paVar4 = (var5 & 0xffff0000 | uVar9 >> 0x10);
    pstruct57_var5.field7_0xe = uVar9;
    let mut uVar1 = (uVar9 >> 0x10);
    pstruct57_var5.field8_0x10 = uVar1;
    uVar1 |= pstruct57_var5.field7_0xe;
    if uVar1 == 0 {
        mem_op_1000_179c(0xc, paVar4);
        let mut uVar3 = paVar4 | uVar1;
        let mut paVar6 = (paVar4 & 0xffff0000 | uVar3);
        if uVar3 == 0 {
            pstruct57_var5[0x1].field7_0xe = 0;
        } else {
            pass1_1010_8ef2(paVar6, CONCAT22(paVar4, uVar1), param_13, param_5, param_7, param_8, param_9);
            pstruct57_var5[0x1].field7_0xe = uVar1;
            pstruct57_var5[0x1].field8_0x10 = paVar6;
        }
    }
    return;
}



pub fn pass1_1040_d1bc(param_1: *mut StructD)

{
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut StructD;
    let mut uVar4: u16;
    let mut in_stack_0000ffd4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.address_offset_field_0x0 = 0xd8c4;
    iVar4.address_offset_field_0x2 = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, &iVar4.field_0x6);
    puVar1 = &iVar4.field_0x9c;
    uVar2 = &iVar4.field_0x9e;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(&u16_1050_1038, puVar1, uVar2, 1);
    }
    unk_draw_op_1040_b0f8(in_stack_0000ffd4, param_1);
    return;
}
