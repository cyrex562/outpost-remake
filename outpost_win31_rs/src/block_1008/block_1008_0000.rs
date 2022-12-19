use std::os::raw::c_char;
use crate::block_1000::block_1000_1000::fn_ptr_1000_17ce;
use crate::block_1008::block_1008_5000::pass1_1008_53aa;
use crate::block_1008::block_1008_9000::pass1_1008_9466;
use crate::block_1010::block_1010_2000::pass1_1010_2050;
use crate::block_1010::block_1010_7000::pass1_1010_7efc;
use crate::block_1030::block_1030_8000::pass1_1030_8210;
use crate::block_1038::block_1038_a000::pass1_1038_af34;

pub unsafe fn struct_op_1008_0000(param_1: *mut u16) {
    let mut i_var1: i16;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x52a;
    (i_var1 + 0x2) = 0x1008;
    (i_var1 + 0x4) = 0;
    (i_var1 + 0x8) = 0;
    *param_1 = 0x51e;
    (i_var1 + 0x2) = 0x1008;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_0036(param_1: *mut u16) {
    let mut pu_var1: *mut u32;
    let mut u_var2: u16;
    let mut pc_var3: *mut c_char;
    let mut ppcVar4: *mut *mut code;
    let mut pu_var5: *mut u32;
    let mut pu_var6: *mut u16;
    let mut iVar6: *mut astruct_449;
    let mut u_var7: u16;
    let mut unaff_cs: u16;

    u_var7 = (param_1 >> 0x10);
    iVar6 = param_1;
    *param_1 = 0x51e;
    iVar6.field2_0x2 = 0x1008;
    pc_var3 = *&iVar6.field_0x8;
    if (iVar6.field7_0xa | pc_var3) != 0 {
        pass1_1008_53aa();
        unaff_cs = 0x1000;
        fn_ptr_1000_17ce(pc_var3);
    }
    pu_var6 = _u16_1050_5748;
    _PTR_LOOP_1050_0298 = 0;
    if _u16_1050_5748.is_null() == false {
        pass1_1030_8210(_u16_1050_5748);
        unaff_cs = 0x1000;
        fn_ptr_1000_17ce(pu_var6);
    }
    pc_var3 = _u16_1050_0ed0;
    if _u16_1050_0ed0.is_null() == false {
        pass1_1010_2050(_u16_1050_0ed0);
        unaff_cs = 0x1000;
        fn_ptr_1000_17ce(pc_var3);
    }
    pu_var5 = _u16_1050_14cc;
    if (_u16_1050_14cc.is_null() == false) {
        pass1_1010_7efc(_u16_1050_14cc);
        unaff_cs = 0x1000;
        fn_ptr_1000_17ce(pu_var5);
    }
    pc_var3 = _PTR_LOOP_1050_5b7c;
    if (_PTR_LOOP_1050_5b7c.is_null() == false) {
        pass1_1038_af34();
        unaff_cs = 0x1000;
        fn_ptr_1000_17ce(pc_var3);
    }
    if (_u16_1050_5bc8.is_null() == false) {
        ppcVar4 = *_u16_1050_5bc8;
        (**ppcVar4)(unaff_cs, _u16_1050_5bc8, (_u16_1050_5bc8 >> 0x10), 1);
    }
    if (_u16_1050_02a0.is_null() == false) {
        ppcVar4 = *_u16_1050_02a0;
        (**ppcVar4)(unaff_cs, _u16_1050_02a0, (_u16_1050_02a0 >> 0x10), 1);
    }
    pu_var1 = iVar6.field3_0x4;
    u_var2 = iVar6.field4_0x6;
    if ((u_var2 | pu_var1) != 0) {
        ppcVar4 = *pu_var1;
        (**ppcVar4)(unaff_cs, pu_var1, u_var2, 1);
    }
    pass1_1008_9466(param_1);
    return;
}

pub unsafe fn set_struct_op_1008_0536(param_1: *mut astruct_20, mut param_2: u16) {
    let mut hicon_1: HICON16;
    let mut hcursor_1: HCURSOR16;
    let mut hbrush_1: HGDIOBJ16;
    let mut in_EDX: u32;
    let mut uVar2: u16;
    let mut paVar1: *mut Struct57;
    let mut paVar3: *mut astruct_20;
    let mut puVar4: *mut u32;
    let mut in_stack_0000feac: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd6: u16;
    let mut in_stack_0000ffda: u16;

    uVar2 = (in_EDX >> 0x10);
    paVar3 = pass1_1008_3ab8(param_1);
    paVar1 = CONCAT22(uVar2, (paVar3 >> 0x10));
    (param_1 + 0xe0) = 0;
    (param_1 + 0xe4) = 0;
    (param_1 + 0xe8) = 0;
    (param_1 + 0xec) = 0;
    (param_1 + 0xee) = 0;
    (param_1 + 0xf2) = 0;
    (param_1 + 0xf4) = 0;
    (param_1 + 0xf8) = 0;
    param_1.offset_0x0 = 0x389e;
    (param_1 + 0x2) = 0x1008;
    (param_1 + 0xc8) = 0x2008;
    (param_1 + 0xac) = 0;
    (param_1 + 0xae) = 0x8700;
    hicon_1 = LoadIcon16(s_Op_1050_00d4, HINSTANCE16_1050_038c);
    (param_1 + 0xc2) = hicon_1;
    hcursor_1 = LoadCursor16(0x7f00, 0x0);
    (param_1 + 0xc4) = hcursor_1;
    hbrush_1 = GetStockObject16(BLACK_BRUSH);
    (param_1 + 0xc6) = hbrush_1;
    puVar4 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0 & 0xffff,
        CONCAT22(param_1, 0x48),
        in_stack_0000feac,
        in_stack_0000ffd0,
        in_stack_0000ffd6,
        in_stack_0000ffda,
    );
    paVar1 = (paVar1 & 0xffff0000 | puVar4 >> 0x10);
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0xa)),
        s_Outpost_1050_00d7,
    );
    puVar4 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0 & 0xffff,
        CONCAT22(param_1, 0x32),
        in_stack_0000feac,
        in_stack_0000ffd0,
        in_stack_0000ffd6,
        in_stack_0000ffda,
    );
    (param_1 + 0xf4) = puVar4;
    (param_1 + 0xf6) = (puVar4 >> 0x10);
    set_sys_color_1008_357e(param_1, 0x1, paVar1 & 0xffff0000 | puVar4 >> 0x10);
    return;
}
pub unsafe fn cleanup_ui_op_1008_0618(param_1: *mut astruct_53) {
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut in_EDX: u32;
    let mut iVar4: *mut astruct_53;
    let mut uVar3: u16;
    let mut puVar1: *mut u32;

    uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1 = 0x389e;
    iVar4.field_0x2 = 0x1008;
    set_sys_color_1008_357e(param_1, 0x0, in_EDX);
    fn_ptr_1000_17ce(*&iVar4.field248_0xf8);
    if (&iVar4.field_0xec != 0) {
        DestroyMenu16(&iVar4.field_0xec);
    }
    DestroyIcon16(&iVar4.field_0xc2);
    iVar4.field_0xc2 = 0;
    puVar1 = &iVar4.field_0xe0;
    uVar1 = &iVar4.field_0xe2;
    if ((uVar1 | puVar1) != 0) {
        ppcVar2 = *puVar1;
        (**ppcVar2)(s_tile2_bmp_1050_1538, puVar1, uVar1, 1);
    }
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0xd2)));
    param_1 = 0x380a;
    iVar4.field_0x2 = 0x1008;
    param_1 = 0x389a;
    iVar4.field_0x2 = 0x1008;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_07d8(
    mut param_1: u16,
    param_2: *mut Struct57,
    param_3: *mut Struct72,
) -> BOOL16 {
    let mut uVar2: u16;
    let mut uVar1: u16;
    let mut uVar4: u32;
    let mut paVar3: *mut Struct57;

    if (_u16_1050_5748.is_null()) {
        mem_op_1000_179c(0xa, param_2);
        uVar2 = param_2 | param_1;
        paVar3 = (param_2 & 0xffff0000 | uVar2);
        if (uVar2 != 0) {
            struct_1030_8128(paVar3, CONCAT22(param_2, param_1));
        }
        if (_u16_1050_5748.is_null()) {
            debug_print_1008_6048(paVar3, s_New_failed_in_Op__Op__Simulator_1050_0110);
            fn_ptr_op_1000_24cd(1);
        }
        uVar4 = pass1_1028_e2e0(paVar3, _PTR_LOOP_1050_65e2, 0x8);
        paVar3 = (paVar3 & 0xffff0000 | uVar4 >> 0x10);
        uVar4 = pass1_1028_e2e0(paVar3, _PTR_LOOP_1050_65e2, 0x8);
        pass1_1028_e2e0(
            (paVar3 & 0xffff0000 | uVar4 >> 0x10),
            _PTR_LOOP_1050_65e2,
            0xff,
        );
        pass1_1030_838e(_u16_1050_5748);
        param_1 = pass1_1030_8334();
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_087e(param_1: u8, mut param_2: u16, param_3: *mut StructD) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut uVar2: u32;
    let mut local_112: u16;
    let mut uStack272: u16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_3);
    mem_op_1000_179c(0xa, paVar1);
    uStack4 = paVar1;
    paVar1 = (paVar1 & 0xffff0000 | (uStack4 | param_2));
    uStack6 = param_2;
    if ((uStack4 | param_2) != 0) {
        struct_1030_8128(paVar1, CONCAT22(uStack4, param_2));
    }
    if (_u16_1050_5748.is_null()) {
        debug_print_1008_6048(paVar1, s_New_failed_in_Op__Op__Simulator_1050_0130);
        fn_ptr_op_1000_24cd(1);
    }
    uVar2 = pass1_1028_e2e0(paVar1, _PTR_LOOP_1050_65e2, 0x8);
    pass1_1028_e2e0(
        (paVar1 & 0xffff0000 | uVar2 >> 0x10),
        _PTR_LOOP_1050_65e2,
        0x8,
    );
    pass1_1030_532e(CONCAT22(0x1050, &local_112), 0xff000000);
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &local_112));
    pass1_1030_838e(_u16_1050_5748);
    local_112 = 0x389a;
    uStack272 = 0x1008;
    pass1_1030_8334();
    return;
}

pub unsafe fn caseD_a7(mut param_1: u16, mut param_2: u16) {
    let mut unaff_BP: i16;
    let mut uVar1: *mut Struct72;

    ui_op_1008_2c4e(param_1, param_2, (unaff_BP + 0x6), 0x57);
    return;
}
