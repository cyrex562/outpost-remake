use std::os::raw::c_char;
use crate::unk::block_1000_1000::fn_ptr_1000_17ce;
use crate::unk::block_1008_5000::pass1_1008_53aa;
use crate::unk::block_1008_9000::pass1_1008_9466;
use crate::unk::block_1010_2000::pass1_1010_2050;
use crate::unk::block_1010_7000::pass1_1010_7efc;
use crate::unk::block_1030_8000::pass1_1030_8210;
use crate::unk::block_1038_a000::pass1_1038_af34;

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
