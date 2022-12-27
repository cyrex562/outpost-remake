use crate::unk::block_1040_2000::pass1_1040_2a22;
use crate::unk::block_1040_3000::pass1_1040_3506;
use crate::gui::dialog::dlg_a::send_dlg_item_msg_1040_3f12;
use crate::gui::dialog::dlg_a::dialog_item_ui_op_1040_3e08;
use crate::gui::window::set_win_pos_1040_331a;
use crate::winapp::winapp_c::send_msg_1040_323c;


pub fn pass1_1040_288e(mut param_1: u32)

{
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut paVar5: *mut astruct_394;
    let mut puVar6: *mut u32;
    let mut uVar7: u16;
    let mut puVar8: *mut u8;
    let mut in_EDX: u32;
    let mut paVar9: *mut Struct57;
    let mut iVar10: i16;
    let mut iVar11: i16;
    let mut uVar12: u16;
    let mut uVar13: u16;

    uVar12 = (param_1 >> 0x10);
    iVar10 = param_1;
    uVar3 = (iVar10 + 0x8e);
    uVar13 = (uVar3 >> 0x10);
    iVar11 = uVar3;
    puVar6 = (iVar11 + 0x24);
    paVar9 = (in_EDX & 0xffff0000 | (iVar11 + 0x26));
    ppcVar2 = (*puVar6 + 0x14);
    (**ppcVar2)();
    paVar5 = puVar6;
    uVar4 = paVar9 << 0x10;
    if ((iVar10 + 0x70) != 0) {
        paVar5 = (iVar10 + 0x70);
        uVar1 = (iVar10 + 0x72);
        uVar7 = uVar1 | paVar5;
        paVar9 = (paVar9 & 0xffff0000 | uVar7);
        if (uVar7 != 0) {
            ppcVar2 = paVar5;
            (**ppcVar2)();
        }
    }
    mem_op_1000_179c(0x14, paVar9);
    puVar8 = (paVar9 | paVar5);
    if (puVar8.is_null()) {
        paVar5 = null_mut();
        puVar8 = null_mut();
    } else {
        struct_1008_4c58(paVar5);
    }
    (iVar10 + 0x70) = paVar5;
    (iVar10 + 0x72) = puVar8;
    pass1_1008_4d84(puVar8, (iVar10 + 0x70), puVar6 & 0xffff | uVar4);
    return;
}


pub fn pass1_1040_2930(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_2464(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_2dac(mut param_1: u32)

{
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar2: u32;
    let mut iStack10: i16;

    uVar1 = (param_1 + 0x90);
    uVar2 = struct_op_1030_73a8((uVar1 + 0x6), in_AX, in_DX);
    for iStack10 in 0..0x5 {
        pass1_1028_4ab2(uVar2, (&PTR_LOOP_1050_5d04 + iStack10 * 0xc), (iStack10 * 0xc + 0x5d02));
    }
    return;
}


pub fn pass1_1040_2e00(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_2a22(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_2ea2(param_1: *mut StructD, param_2: *mut Struct57, mut param_3: u32, mut param_4: u16, mut param_5: u16, mut param_6: u16)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut iVar1: *mut Struct57;
    let mut unaff_BP: u16;
    let mut uVar1: *mut Struct57;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_1);
    get_sys_metrics_1040_7728(param_2, 0x1, param_3, 0x180, param_6);
    uVar1 = (param_2 >> 0x10);
    iVar1 = param_2;
    (iVar1 + 1).field0_0x0 = 0;
    iVar1[0x1].field1_0x2 = 0;
    iVar1[0x1].field2_0x4 = 0;
    iVar1[0x1].field3_0x6 = 0;
    iVar1[0x1].field4_0x8 = 0;
    param_2.field0_0x0 = 0x3436;
    iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
    puVar2 = mixed_1010_20ba(paVar1, _u16_1050_0ed0, CONCAT22(unaff_BP, 0x3c), in_stack_0000fea6,
                             in_stack_0000ffca, in_stack_0000ffd0, in_stack_0000ffd4);
    iVar1[0x1].field4_0x8 = puVar2;
    iVar1[0x1].field5_0xa = (puVar2 >> 0x10);
    return;
}


pub fn pass1_1040_2f32(mut param_1: u16, mut param_2: u16, mut param_3: u16)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct27;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut iVar2: i16;

    iVar2 = 0;
    paVar1 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0, 0x2b,
                             in_stack_0000fea0, in_stack_0000ffc4, in_stack_0000ffca, in_stack_0000ffce);
    pass1_1010_038e(paVar1, iVar2);
    destroy_win_1040_7b98(CONCAT22(param_3, param_2));
    return;
}


pub fn pass1_1040_3410(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_2f06(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_3532(param_1: *mut u8, mut param_2: u16, mut param_3: u16)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct27;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut iVar2: i16;

    iVar2 = 0;
    paVar1 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0, 0x2b,
                             in_stack_0000fea0, in_stack_0000ffc4, in_stack_0000ffca, in_stack_0000ffce);
    pass1_1010_038e(paVar1, iVar2);
    destroy_win_1040_7b98(CONCAT22(param_3, param_2));
    return;
}


pub fn pass1_1040_38d4(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_3506(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1040_3a0e(param_1: *mut u8, mut param_2: u16, mut param_3: u16)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct27;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut iVar2: i16;

    iVar2 = 0;
    paVar1 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0, 0x2b,
                             in_stack_0000fea0, in_stack_0000ffc4, in_stack_0000ffca, in_stack_0000ffce);
    pass1_1010_038e(paVar1, iVar2);
    destroy_win_1040_7b98(CONCAT22(param_3, param_2));
    return;
}
