// #include "sys_ops_4.h"

// #include "address_tables/function_tables.h"
// #include "fn_ptr_ops/fn_ptr_ops_6.h"
// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "struct_ops/struct_ops_3.h"
// #include "structs/structs_3xx/struct_323.h"
// #include "sys_ops_12.h"

pub fn pass1_1028_a79c(param_1: u32, param_2: u16, u8 *param_3) {
    u32 * puVar1;
    u32 * pu_var2;
    u32 * puVar3;
    let mut iVar4: i16;
    u32 * puVar5;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x108, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10      = addr_table_1008_380a[36]; // 0x389a
        (param_2 + 0x2) = SEG_1008;
        uVar6           = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        puVar3 = (param_1 + 0x8);
        puVar5 = (param_2 + 0x8);
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        (param_2 + 0x2) = SEG_1028;
        *puStack10 = addr_table_1028_a856;//0xa856;
        (param_2 + 0x2) = SEG_1028;
    }
}


pub fn pass1_1028_a8f4(param_1: u32, param_2: *mut Struct335, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x108, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        param_2.field_0x4 = (param_1 + 0x4);
        puVar3 = (param_1 + 0x8);
        puVar5 = &param_2.field_0x8;
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        *puStack10 = addr_table_1028_a9ae;//0xa9ae;
        param_2->fld2_segment = SEG_1028;
    }
    return;
}


pub fn pass1_1028_aa68(param_1: u32, param_2: *mut Struct336, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x108, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        param_2.field_0x4 = (param_1 + 0x4);
        puVar3 = (param_1 + 0x8);
        puVar5 = &param_2.field_0x8;
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        *puStack10 = addr_table_1028_ab22;//0xab22;
        param_2->fld2_segment = SEG_1028;
    }
    return;
}


pub fn pass1_1028_abec(param_1: u32, param_2: *mut Struct337, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x108, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        param_2.field_0x4 = (param_1 + 0x4);
        puVar3 = (param_1 + 0x8);
        puVar5 = &param_2.field_0x8;
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        *puStack10 = addr_table_1028_aca6;//0xaca6;
        param_2->fld2_segment = SEG_1028;
    }
    return;
}


pub fn pass1_1028_ad9c(param_1: u32, param_2: *mut Struct338, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x108, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        param_2.field_0x4 = (param_1 + 0x4);
        puVar3 = (param_1 + 0x8);
        puVar5 = &param_2.field_0x8;
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        *puStack10 = addr_table_1028_ae56;//0xae56;
        param_2->fld2_segment = SEG_1028;
    }
}


u16 pass1_1028_af08(param_1: *mut Struct693, param_2: *mut u8, param_3: i16, param_4: u16) {
    let mut uVar1: u32;
    let mut pu_var2: *mut u8;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u8;
    let mut puVar5: *mut u8;
    let mut iVar6: *mut Struct693;
    let mut uVar6: u16;
    let mut puVar7: *mut u16;
    let mut paVar8: *mut Struct67;
    let mut paVar9: *mut Struct67;
    let mut iStack12: i16;
    let mut iStack10: i16;

    puVar7 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_4, param_2, param_3);
    puVar4 = (puVar7 >> 0x10);
    pu_var2 = globals.PTR_LOOP_1050_13ae + -0x1;
    if((globals.PTR_LOOP_1050_13ae < 0x1) || (SBORROW2(globals.PTR_LOOP_1050_13ae, 0x1))) {
        // LAB_1028_af27:
        iStack10 = 0x1;
    } else {
        puVar3 = globals.PTR_LOOP_1050_13ae + -0x2;
        if(puVar3 == 0x0 || pu_var2 < 0x1) {
            iStack12 = 0x1;
            iStack10 = 0x1;
            goto LAB_1028_af42;
        }
        pu_var2 = globals.PTR_LOOP_1050_13ae + -0x4;
        if(pu_var2 != 0x0)
            goto LAB_1028_af27;
        iStack10 = 0x2;
    }
    iStack12 = 0x3;
    puVar3   = pu_var2;
// LAB_1028_af42:
    pass1_1008_612e(iStack10, iStack12, puVar3);
    uVar6              = (param_1 >> 0x10);
    iVar6              = param_1;
    iVar6.field_0x114 = puVar3;
    paVar8             = mixed_1010_20ba(globals.data_1050_0ed0, 0x37, param_4, puVar4, param_3);
    uVar1              = iVar6.field_0x108;
    paVar9             = paVar8;
    pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    puVar5 = (paVar9 >> 0x10);
    puVar4 = puVar5;
    post_win_msg_1008_a0e4(paVar8, 0x0, iVar6.field_0x114, (paVar9 + 0x208), iVar6.field_0x108, 0x2, SEG_1008, param_4);
    puVar7 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2b, param_4, puVar4, param_3);
    pass1_1010_043a(puVar7, (paVar9 + 0x4), 0xd, param_4);
    return 0x1;
}


pub fn pass1_1028_afce(param_1: *mut Struct340, param_2: *mut Struct339, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut iVar3: i16;
    let mut iVar5: *mut Struct340;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x116, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        iVar5              = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        puVar5 = &param_2.field_0x8;
        for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x110 = iVar5.field_0x110;
        param_2.field_0x114 = iVar5.field_0x114;
        *puStack10 = addr_table_1028_b0ce;//0xb0ce;
        param_2->fld2_segment = SEG_1028;
    }
    return;
}


pub fn pass1_1028_b108(param_1: *mut Struct342, param_2: *mut Struct341, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut iVar3: i16;
    let mut iVar5: *mut Struct342;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x110, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        iVar5              = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        puVar5 = &param_2.field_0x8;
        for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        *puStack10 = addr_table_1028_6e50;//0x6e50;
        param_2->fld2_segment = SEG_1028;
        *puStack10 = addr_table_1028_b1f4;//0xb1f4;
        param_2->fld2_segment = SEG_1028;
    }
}


pub fn pass1_1028_94e4(param_1: *mut Struct329, param_2: *mut Struct328, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut iVar3: i16;
    let mut iVar5: *mut Struct329;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x124, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        iVar5              = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        puVar5 = &param_2.field_0x8;
        for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x110 = iVar5.field_0x110;
        param_2.field_0x114 = iVar5.field_0x114;
        param_2.field_0x118 = iVar5.field_0x118;
        param_2.field_0x11a = iVar5.field_0x11a;
        param_2.field_0x11c = iVar5.field_0x11c;
        param_2.field_0x11e = iVar5.field_0x11e;
        param_2.field_0x122 = iVar5.field_0x122;
        *puStack10 = addr_table_1028_9934;//0x9934;
        param_2->fld2_segment = SEG_1028;
    }
    return;
}


pub fn pass1_1028_9b48(param_1: *mut Struct331, param_2: *mut Struct330, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut iVar3: i16;
    let mut iVar5: *mut Struct331;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x118, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    iVar5     = param_1;
    uVar6     = (param_1 >> 0x10);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        puVar5 = &param_2.field_0x8;
        for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x110 = iVar5.field_0x110;
        param_2.field_0x114 = iVar5.field_0x114;
        *puStack10 = addr_table_1028_9c52;//0x9c52;
        param_2->fld2_segment = SEG_1028;
    }
    iVar5.field_0x114 = 0x0;
    return;
}


pub fn pass1_1028_9dee(param_1: *mut Struct333, param_2: *mut Struct332, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut iVar3: i16;
    let mut iVar5: *mut Struct333;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x10a, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        iVar5              = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        puVar5 = &param_2.field_0x8;
        for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        *puStack10 = addr_table_1028_9eb6;//0x9eb6;
        param_2->fld2_segment = SEG_1028;
    }
}


pub fn pass1_1028_9efc(param_1: u32,param_2: *mut u16, param_3: u16, param_4: i16, param_5: u16, param_6: u8) {
    long        lVar1;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut iVar4: i16;
    long        lVar5;
    let mut puVar6: *mut u8;
    let mut in_register_0000000a: u16;
    let mut uVar7: u32;
    let mut paVar8: *mut Struct67;
    let mut puVar9: *mut u16;
    let mut uVar10: u16;
    let mut local_18: u16;
    let mut uStack22: u16;
    let mut pu_stack6: *mut u16;
    let mut puStack4: *mut u8;

    pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, 0x1, 0x400);
    uVar7 = str_var1(in_register_0000000a, param_3 | param_2);
    if((param_3 | param_2) != 0x0) {
        pu_stack6 = param_2;
        puStack4 = param_3;
        pass1_1028_dc52(CONCAT13((param_5 >> 0x8), CONCAT12(param_5, &local_18)), 0x1, 0x0, 0x400);
        while(true) {
            pu_var2 = &local_18;
            pass1_1028_e4ec(str_var1(param_5, pu_var2));
            puStack4 = uVar7;
            pu_stack6 = pu_var2;
            if((puStack4 | pu_var2) == 0x0)
                break;
            lVar1 = (pu_var2 + 0x100);
            uVar3 = pu_var2[0x101];
            uVar7 = uVar7 & 0xffff0000 | uVar3;
            if(pu_var2[0xff] != 0x0) {
                uVar10 = (param_1 >> 0x10);
                lVar5  = lVar1;
                if((lVar1 != 0x2) || (uVar3 != 0x800)) {
                    pass1_1028_a3ae(param_1, uVar10,
                                    str_var1(puStack4, pu_var2), uVar7, param_4, param_5, param_6, lVar1);
                }
                uVar3 = lVar5;
                pass1_1028_a28a(param_1, uVar10, str_var1(puStack4, pu_stack6));
                if((uVar7 < 0x1) && ((uVar7 < 0x0 || (uVar3 < 0x64)))) {
                    pass1_1028_a4ee(param_1, str_var1(puStack4, pu_stack6), param_4, param_5);
                }
                if(lVar1 != 0x8000002) {
                    pass1_1038_42cc(str_var1(puStack4, pu_stack6), param_5);
                    puVar6 = (uVar7 | uVar3);
                    uVar7  = uVar7 & 0xffff0000 | ZEXT24(puVar6);
                    if(puVar6 != 0x0) {
                        paVar8 = mixed_1010_20ba(globals.data_1050_0ed0, 0x37, param_5, puVar6, param_4);
                        uVar7  = uVar7 & 0xffff0000 | paVar8 >> 0x10;
                        post_win_msg_1008_a0e4(paVar8, 0x0, uVar3, pu_stack6[0x104], *(pu_stack6 + 0x2), 0x2, SEG_1008, param_5);
                    }
                }
            }
        }
        local_18 = addr_table_1008_380a[36]; // 0x389a
        uStack22 = SEG_1008;
        puVar9   = mixed_1010_20ba(globals.data_1050_0ed0, 0x8, param_5, 0x0, param_4);
        puVar6   = (puVar9 >> 0x10);
        iVar4    = puVar9;
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, 0x1, 0x400);
        pu_stack6 = iVar4;
        puStack4 = puVar6;
        pass1_1010_9f72(puVar9, 0x3e, param_5);
        if(iVar4 != 0x0) {
            iVar4 = pass1_1010_96d0(puVar9);
            if(iVar4 < 0x1) {
                if(iVar4 < 0x0) {
                    iVar4 = (pu_stack6 + 0x1f6);
                    pass1_1030_38b8();
                    if((puVar6 < 0x1) && ((puVar6 < 0x0 || (iVar4 == 0x0)))) {
                        paVar8 = mixed_1010_20ba(globals.data_1050_0ed0, 0x37, param_5, puVar6, param_4);
                        post_win_msg_1008_a0e4(paVar8, 0x0, 0x0, 0x1, *(pu_stack6 + 0x4), 0x6, SEG_1008, param_5);
                    }
                }
            } else {
                puVar9 = mixed_1010_20ba(globals.data_1050_0ed0, 0x37, param_5, puVar6, param_4);
                puVar6 = (puVar9 >> 0x10);
                post_win_msg_1008_a0e4((puVar9 & 0xffff | ZEXT24(puVar6) << 0x10), 0x0, iVar4, (pu_stack6 + 0x208), 0x4000001, 0x2, SEG_1008, param_5);
                puVar9 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2b, param_5, puVar6, param_4);
                pass1_1010_043a(puVar9, (pu_stack6 + 0x4), 0x14, param_5);
            }
        }
    }
    return;
}


pub fn pass1_1028_a0fa(param_1: u32, param_2: *mut Struct334, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x108, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        param_2.field_0x4 = (param_1 + 0x4);
        puVar3 = (param_1 + 0x8);
        puVar5 = &param_2.field_0x8;
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        *puStack10 = addr_table_1028_a6f6;//0xa6f6;
        param_2->fld2_segment = SEG_1028;
    }
    return;
}


pub fn pass1_1028_8400(param_1: u32, param_2: u16, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x108, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10      = addr_table_1008_380a[36]; // 0x389a
        (param_2 + 0x2) = SEG_1008;
        uVar6           = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        puVar3 = (param_1 + 0x8);
        puVar5 = (param_2 + 0x8);
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        (param_2 + 0x2) = SEG_1028;
        *puStack10 = addr_table_1028_84ba;//0x84ba;
        (param_2 + 0x2) = SEG_1028;
    }
    return;
}


pub fn pass1_1028_84ca(param_1: *mut Struct100, param_2: u32, param_3: u16, param_4: u16, param_5: u16, param_6: u16, param_7: u8) {
    let mut uVar1: u16;
    let mut iVar2: i16;
    let mut puVar3: *mut u8;

    struct_op_1028_d1dc(param_6, param_7, param_1, 0x3e7);
    puVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    (iVar2 + 0x108) = param_5;
    (iVar2 + 0x10a) = param_4;
    (iVar2 + 0x10c) = param_3;
    *(iVar2 + 0x10e) = param_2;
    param_1.fld0_addr_table = addr_table_1028_8688;//0x8688;
    (iVar2 + 0x2) = SEG_1028;
    if ((iVar2 + 0x108) == 0x1) {
        uVar1 = s_max_1050_501c;
    } else {
        uVar1 = s_min_1050_5020;
    }
    sys_1000_3f9c((iVar2 + 0x8),
                  puVar3,
                  s_SCForceMorale__s_for_colony__08l_1050_5024,
                  uVar1,
                  &stack0xfffe,
                  puVar3,
                  SEG_1000,
                  param_6,
                  param_7);
    return;
}


pub fn pass1_1028_86f4(param_1: *mut Struct321, param_2: *mut Struct320, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut iVar3: i16;
    let mut iVar5: *mut Struct321;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x110, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        iVar5              = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        puVar5 = &param_2.field_0x8;
        for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        *puStack10 = addr_table_1028_6e50;//0x6e50;
        param_2->fld2_segment = SEG_1028;
        *puStack10 = addr_table_1028_87e0;//0x87e0;
        param_2->fld2_segment = SEG_1028;
    }
    return;
}


pub fn pass1_1028_8c46(param_1: *mut Struct323, param_2: *mut Struct322, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut iVar3: i16;
    let mut iVar5: *mut Struct323;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x124, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        iVar5              = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        puVar5 = &param_2.field_0x8;
        for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x110 = iVar5.field_0x110;
        param_2.field_0x114 = iVar5.field_0x114;
        param_2.field_0x118 = iVar5.field_0x118;
        param_2.field_0x11a = iVar5.field_0x11a;
        param_2.field_0x11c = iVar5.field_0x11c;
        param_2.field_0x11e = iVar5.field_0x11e;
        param_2.field_0x120 = iVar5.field_0x120;
        *puStack10 = addr_table_1028_8d8e;//0x8d8e;
        param_2->fld2_segment = SEG_1028;
    }
    return;
}


pub fn pass1_1028_8ea6(param_1: *mut Struct325, param_2: *mut Struct324, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut iVar3: i16;
    let mut iVar5: *mut Struct325;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x118, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    iVar5     = param_1;
    uVar6     = (param_1 >> 0x10);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        puVar5 = &param_2.field_0x8;
        for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x110 = iVar5.field_0x110;
        param_2.field_0x114 = iVar5.field_0x114;
        *puStack10 = addr_table_1028_8fb0;//0x8fb0;
        param_2->fld2_segment = SEG_1028;
    }
    iVar5.field_0x114 = 0x0;
}


pub fn pass1_1028_8fea(param_1: *mut Struct327, param_2: *mut Struct326, u8 *param_3) {
    let mut pu_var2: *mut u32;
    let mut iVar3: i16;
    let mut iVar5: *mut Struct327;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;
    let mut puVar1: *mut u32;

    mem_op_1000_179c(0x110, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        iVar5              = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        puVar5 = &param_2.field_0x8;
        for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        *puStack10 = addr_table_1028_6e50;//0x6e50;
        param_2->fld2_segment = SEG_1028;
        *puStack10 = addr_table_1028_90d6;//0x90d6;
        param_2->fld2_segment = SEG_1028;
    }
    return;
}


pub fn pass1_1028_9264(param_1: u32, param_2: u16, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut puVar6: *mut u32;
    let mut uVar7: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x10a, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10      = addr_table_1008_380a[36]; // 0x389a
        (param_2 + 0x2) = SEG_1008;
        uVar7           = (param_1 >> 0x10);
        iVar5           = param_1;
        (param_2 + 0x4) = (iVar5 + 0x4);
        puVar3 = (iVar5 + 0x8);
        puVar6 = (param_2 + 0x8);
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar6;
            puVar6 = puVar6 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        (param_2 + 0x2) = SEG_1028;
        (param_2 + 0x108) = (iVar5 + 0x108);
        *puStack10 = addr_table_1028_932c;//0x932c;
        (param_2 + 0x2) = SEG_1028;
    }
    return;
}


pub fn struct_op_1028_933c(param_1: *mut Struct100, param_2: u16, param_3: u16, param_4: u16, param_5: *mut u32, param_6: u16, param_7: u32, param_8: u32, param_9: u16, param_10: u8) {
    let mut iVar1: i16;
    let mut pu_var2: *mut u8;

    struct_op_1028_d1dc(param_9, param_10, param_1, 0x3e8);
    pu_var2             = (param_1 >> 0x10);
    iVar1              = param_1;
    *(iVar1 + 0x108) = param_8;
    *(iVar1 + 0x10c) = param_7;
    (iVar1 + 0x110) = 0x0;
    *(iVar1 + 0x114) = *param_5;
    (iVar1 + 0x118) = (param_5 + 0x1);
    (iVar1 + 0x11a) = param_4;
    (iVar1 + 0x11c) = param_2;
    (iVar1 + 0x120) = 0x0;
    (iVar1 + 0x11e) = 0x0;
    (iVar1 + 0x122) = param_3;
    param_1.fld0_addr_table = addr_table_1028_9934;//0x9934;
    (iVar1 + 0x2) = SEG_1028;
    sys_1000_3f9c((iVar1 + 0x8),
                  pu_var2,
                  s_SCPutBldg_site_0x_08lx__bldg__u__1050_50ce,
                  param_8,
                  &stack0xfffe,
                  pu_var2,
                  SEG_1000,
                  param_9,
                  param_10);
}

pub fn pass1_1028_752e(param_1: u32, param_2: u16, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x108, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10      = addr_table_1008_380a[36]; // 0x389a
        (param_2 + 0x2) = SEG_1008;
        uVar6           = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        puVar3 = (param_1 + 0x8);
        puVar5 = (param_2 + 0x8);
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        (param_2 + 0x2) = SEG_1028;
        *puStack10 = addr_table_1028_819a;//0x819a;
        (param_2 + 0x2) = SEG_1028;
    }
}

pub fn pass1_1028_767e(param_1: i16, param_2: u16, param_3: i16, param_4: u16) {
    let mut puVar1: *mut u16;

    pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, 0x1, 0x800);
    if(((param_1 + 0x152) != 0x0) && (((qword)*_PTR_LOOP_1050_65e2 % 0x64) == 0x0)) {
        puVar1 = mixed_1010_20ba(globals.data_1050_0ed0, 0x40, param_4, 0x0, param_3);
        load_str_and_spri16f_1008_b78a(NULL, puVar1, param_4, (puVar1 >> 0x10), puVar1);
    }
}


pub fn pass1_1028_78b8(param_1: u32, long param_2, param_3: i16, param_4: u16, param_5: u8) {
    let mut puVar1: *mut u32;
    u32 **ppu_var2;
    let mut puVar3: *mut u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u32;
    let mut puVar6: *mut u8;
    let mut uVar7: u16;
    let mut puVar8: *mut u8;
    let mut iVar9: i16;
    let mut uVar10: u32;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let mut puVar13: *mut u16;
    let mut uVar14: u16;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut iVar17: i16;
    let mut uStack340: u16;
    let mut uStack338: u16;
    let mut puStack74: *mut u16;
    let mut puStack70: *mut u8;
    let mut pu_stack68: *mut u8;
    u32  *local_42[0x4];
    let mut local_30: *mut u8;
    let mut puStack46: *mut u8;
    u8   *local_1e[0x3];
    let mut local_18: u32;
    let mut puStack20: *mut u8;
    let mut puStack18: *mut u8;
    let mut uStack16: u32;
    let mut puStack12: *mut u8;
    let mut uStack10: u16;
    let mut puStack8: *mut u8;
    let mut pu_stack6: *mut u32;

    puVar6   = param_2;
    puVar5   = *_PTR_LOOP_1050_65e2;
    pu_stack6 = puVar5;
    if(puVar5 == 0x98) {
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, 0x2, 0x400);
        puVar6   = param_2;
        uStack16 = puVar5 & 0xffff | param_2 << 0x10;
        if((puVar5 + 0x200) == 0x8000002) {
            pass1_1020_a43e(param_4, puVar6, str_var1(param_4, &local_18));
            puVar13 = pass1_1008_3e38(str_var1(param_4, local_1e));
            puVar6  = (puVar13 >> 0x10);
            puVar1  = &local_18;
            pass1_1020_a49a(param_4, param_5, puVar6,
                            str_var1(param_4, puVar1),
                            str_var1(param_4, local_1e), 0x7a);
            pass1_1038_4f54(uStack16, 0x1, puVar1);
            if(puVar1 == 0x0) {
                pass1_1020_a49a(param_4, param_5, puVar6, str_var1(param_4, &local_18), 0x0, 0x35);
            }
        }
    }
    if((0xe < pu_stack6) && (pu_stack6 < 0x16)) {
        puVar13  = pass1_1020_a43e(param_4, puVar6, str_var1(param_4, local_1e));
        local_18 = (long)pu_stack6 - 0xf;
        pass1_1020_a54c(param_4, param_5, (puVar13 >> 0x10), local_1e, param_4, local_18);
    }
    uVar10 = (pu_stack6) % 0x7d;
    puVar8 = (pu_stack6) % 0x7d;
    puVar6 = puVar8;
    if(uVar10 == 0x0) {
        local_1e[0] = puVar8;
        pass1_1008_612e(0x1, 0x64, puVar8);
        puVar8 = uVar10;
        puVar6 = local_1e[0];
        if(local_1e[0] < 0x1a) {
            pass1_1028_dc52(str_var1(param_4, &local_30), 0x1, 0x0, 0x400);
            do {
                uVar7  = uVar10;
                uVar10 = ZEXT24(&local_30);
                pass1_1028_e4ec(str_var1(param_4, &local_30));
                puVar6   = uVar10;
                local_18 = uVar10 & 0xffff | uVar7 << 0x10;
                puVar8   = (uVar7 | puVar6);
                uVar10   = ZEXT24(puVar8);
                if(puVar8 == 0x0)
                    goto LAB_1028_79d6;
            } while((puVar6 + 0x200) == 0x8000002);
            pass1_1038_43cc(puVar6, uVar7, 0x1, 0x4, puVar6, puVar8);
            // LAB_1028_79d6:
            local_30  = addr_table_1008_380a[36]; // 0x389a
            puStack46 = SEG_1008;
        }
    }
    if(pu_stack6 == 0x5) {
        uVar16 = SUB42(SEG_1050, 0x0);
        uVar15 = SUB42(s_Rebel_1050_4ffc, 0x0);
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, 0x2, 0x400);
        local_30  = puVar6;
        puStack46 = puVar8;
        pass1_1038_4d3c(str_var1(puVar8, puVar6), str_var1(uVar16, uVar15), puVar8);
    }
    if(pu_stack6 == 0x12c) {
        uVar16    = 0x400;
        iVar9     = 0xf;
        uVar15    = 0x1;
        puVar13   = mixed_1010_20ba(globals.data_1050_0ed0, 0x2b, param_4, puVar8, param_3);
        puVar8    = (puVar13 >> 0x10);
        puVar6    = puVar13;
        local_30  = puVar6;
        puStack46 = puVar8;
        pass1_1010_043a(puVar13, str_var1(uVar16, uVar15), iVar9, param_4);
    }
    if(pu_stack6 == 0x3d) {
        puVar13     = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_4, puVar8, param_3);
        uVar10      = puVar13 >> 0x10;
        local_30    = puVar13;
        puVar8      = (puVar13 >> 0x10);
        local_1e[0] = globals.PTR_LOOP_1050_13ae;
        puVar6      = globals.PTR_LOOP_1050_13ae;
        puStack46   = puVar8;
        if(globals.PTR_LOOP_1050_13ae != (&PTR_LOOP_1050_0000 + 0x1)) {
            pass1_1028_dc52(str_var1(param_4, local_42), 0x1, 0x0, 0x400);
            while(true) {
                uVar7   = uVar10;
                ppu_var2 = local_42;
                pass1_1028_e4ec(str_var1(param_4, ppu_var2));
                local_18 = str_var1(uVar7, ppu_var2);
                uVar10   = (uVar7 | ppu_var2);
                if((uVar7 | ppu_var2) == 0x0)
                    break;
                uStack16 = *(ppu_var2 + 0x1f6);
                pass1_1030_34da(uStack16);
            }
            uVar16    = 0x400;
            iVar9     = 0x10;
            uVar15    = 0x1;
            puVar13   = mixed_1010_20ba(globals.data_1050_0ed0, 0x2b, param_4, 0x0, param_3);
            puVar8    = (puVar13 >> 0x10);
            puVar6    = puVar13;
            puStack20 = puVar6;
            puStack18 = puVar8;
            pass1_1010_043a(puVar13, str_var1(uVar16, uVar15), iVar9, param_4);
            local_42[0] = &u32_1008_389a;
        }
    }
    if(pu_stack6 == 0x96) {
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, 0x1, 0x400);
        puStack74 = str_var1(puVar8, puVar6);
        uVar14    = (param_1 >> 0x10);
        pass1_1028_780c(param_1, uVar14, str_var1(puVar8, puVar6));
        if(puVar6 != 0x0) {
            uVar16    = 0x400;
            iVar9     = 0x1d;
            uVar15    = 0x1;
            puVar13   = mixed_1010_20ba(globals.data_1050_0ed0, 0x2b, param_4, puVar8, param_3);
            puVar8    = (puVar13 >> 0x10);
            puVar6    = puVar13;
            puStack70 = puVar6;
            pu_stack68 = puVar8;
            pass1_1010_043a(puVar13, str_var1(uVar16, uVar15), iVar9, param_4);
        }
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, 0x2, 0x400);
        puStack74 = str_var1(puVar8, puVar6);
        pass1_1028_780c(param_1, uVar14, str_var1(puVar8, puVar6));
    }
    puVar13   = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_4, puVar8, param_3);
    puStack8  = (puVar13 >> 0x10);
    uStack10  = SUB42(puVar13, 0x0);
    puStack12 = globals.PTR_LOOP_1050_13ae;
    if(0x2 < globals.PTR_LOOP_1050_13ae) {
        puStack74 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2f, param_4, puStack8, param_3);
        for(puStack70 = 0x1; puStack70 < 0x9; puStack70 = (puStack70 + 0x1)) {
            local_42[0] = (puStack74 + 0x34 + puStack70 * 0x4);
            if(local_42[0] == pu_stack6) {
                puVar6   = (&PTR_LOOP_1050_0000 + 0x1);
                local_30 = 0x1;
                pass1_1008_612e(0x1, 0x64, 0x1);
                puVar4 = (puStack70 - 0x7);
                if(puVar4 == 0x0) {
                    bVar12 = SBORROW2(puVar6, 0x32);
                    puVar8 = puVar6 + -0x32;
                    bVar11 = puVar6 == (s_New_failed_in_Op__Op_1050_0020 + 0x12);
                    // LAB_1028_7b74:
                    if(!bVar11 && bVar12 == puVar8 < 0x0) {
                        local_30 = 0x0;
                    }
                } else {
                    puVar4 = (puStack70 - 0x8);
                    if(puVar4 == 0x0) {
                        bVar12 = SBORROW2(puVar6, 0x19);
                        puVar8 = puVar6 + -0x19;
                        bVar11 = puVar8 == 0x0;
                        goto LAB_1028_7b74;
                    }
                }
                local_1e[0] = puVar6;
                if(local_30 != 0x0) {
                    pass1_1028_90e6(str_var1(param_4, &uStack340), puStack70, param_4, param_5);
                    puVar4 = &uStack340;
                    fn_ptr_1030_835a(globals._PTR_LOOP_1050_5748,
                                     str_var1(param_4, puVar4));
                    uStack340 = addr_table_1008_380a[36]; // 0x389a
                    uStack338 = SEG_1008;
                }
                pass1_1008_612e(0x0, 0xa, puVar4);
                local_18 = local_18 & 0xffff0000 | ZEXT24(puVar4);
                if(puStack70 == 0x7) {
                    iVar17 = 0x7;
                    puVar3 = puVar4 + 0x37;
                    iVar9  = puVar3 >> 0xf;
                } else {
                    if(puStack70 != 0x8)
                        goto LAB_1028_7ba0;
                    iVar17 = 0x8;
                    puVar3 = puVar4 + 0x32;
                    iVar9  = (puVar4 >> 0xf) + (0xff9b < puVar4);
                }
                uVar14      = (local_42[0] >> 0x10) + iVar9 + CARRY2(local_42[0], puVar3);
                local_42[0] = str_var1(uVar14, local_42[0] + puVar3);
                pass1_1010_ebf8(puStack74, local_42[0] + puVar3, uVar14, iVar17);
            }
            // LAB_1028_7ba0:
        }
    }
    return;
}


pub fn pass1_1028_7c4e(param_1: u32, param_2: *mut u8, param_3: i16, param_4: u16) {
    let mut ppcVar1: *mut *mut c_void;
    let mut pu_var2: *mut u8;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut uVar6: u8;
    let mut in_AF: u8;
    let mut puVar7: *mut u16;
    let mut puVar8: *mut u32;
    let mut paVar9: *mut Struct100;
    let mut uVar10: u8;
    let mut local_156: u16;
    let mut uStack340: u16;
    let mut uStack70: u16;
    let mut u_stack68: u16;
    let mut iStack66: i16;
    let mut u_stack64: u32;
    let mut uStack56: u32;
    let mut uStack52: u16;
    let mut uStack50: u32;
    let mut puStack46: *mut u32;
    let mut uStack42: u16;
    let mut puStack40: *mut u8;
    let mut uStack38: u32;
    let mut local_22: [u8;12] = [0;12];
    let mut iStack16: i16;
    let mut iStack14: i16;
    let mut uStack12: u32;
    let mut puStack8: *mut u8;
    let mut u_stack6: u16;
    let mut uStack4: u16;

    puVar7   = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_4, param_2, param_3);
    uStack4  = (puVar7 >> 0x10);
    u_stack6  = SUB42(puVar7, 0x0);
    puStack8 = globals.PTR_LOOP_1050_13ae;
    if(0x2 < globals.PTR_LOOP_1050_13ae) {
        uStack12       = *_PTR_LOOP_1050_65e2;
        uStack12 = (uStack12 >> 0x10);
        if(0x2 < uStack12) {
            iStack16 = uStack12 - 0x2;
            iStack14 = uStack12 - (uStack12 < 0x2);
            uVar5    = str_var1(iStack14, iStack16) % 0x14;
            if(uVar5 == 0x0) {
                uVar10 = (param_4 >> 0x8);
                pass1_1028_dc52(CONCAT13(uVar10, CONCAT12(param_4, local_22)), 0x1, 0x0, 0x400);
                while(true) {
                    uVar4  = uVar5;
                    pu_var2 = local_22;
                    pass1_1028_e4ec(str_var1(param_4, pu_var2));
                    uStack38 = str_var1(uVar4, pu_var2);
                    uVar5    = (uVar4 | pu_var2);
                    if((uVar4 | pu_var2) == 0x0)
                        break;
                    if((pu_var2 + 0x200) != 0x8000002) {
                        puVar8    = pass1_1008_c6fa(globals.dat_1050_06e0, 0x2a);
                        uVar5     = puVar8 >> 0x10;
                        uVar4     = puVar8;
                        puStack40 = (puVar8 >> 0x10);
                        uVar6     = 0x38;
                        uStack42  = uVar4;
                        pass1_1038_4d6e(uStack38, puVar8, uVar4, puStack40);
                        puStack46 = str_var1(uVar5, uVar4);
                        ppcVar1   = (*puStack46 + 0x10);
                        (**ppcVar1)(SEG_1038, uVar4, uVar5);
                        uStack50 = str_var1(uVar5, uVar4);
                        if(puStack8 == (&PTR_LOOP_1050_0002 + 0x1)) {
                            uStack52 = 0x6;
                        } else {
                            uStack52 = 0xc;
                        }
                        for(uStack56 = 0x0; uStack56 < uStack50; uStack56 = uStack56 + 0x1) {
                            u_stack64 = pass1_1030_1d7c(uStack50, uVar5, puStack46);
                            uVar5    = u_stack64 >> 0x10;
                            iVar3    = u_stack64;
                            pass1_1028_7742(param_1, (param_1 >> 0x10), 0x4, u_stack64, param_4);
                            uVar4 = uStack52;
                            if(iVar3 == 0x0) {
                                uVar4 = 0x19;
                            }
                            uVar6    = 0x8;
                            u_stack68 = uVar4;
                            iStack66 = iVar3;
                            pass1_1008_612e(0x1, 0x64, uVar4);
                            uStack70 = uVar4;
                            if(uVar4 <= u_stack68) {
                                paVar9 = pass1_1028_8fc0(CONCAT13(uVar10, CONCAT12(param_4, &local_156)), *(u_stack64 + 0x4), *(uStack38 + 0x4), param_4, in_AF);
                                uVar5  = paVar9 >> 0x10;
                                uVar6  = 0x30;
                                fn_ptr_1030_835a(globals._PTR_LOOP_1050_5748,
                                                 str_var1(param_4, &local_156));
                                local_156 = addr_table_1008_380a[36]; // 0x389a
                                uStack340 = SEG_1008;
                            }
                        }
                        if(puStack46 != 0x0) {
                            ppcVar1 = *puStack46;
                            (**ppcVar1)(uVar6, puStack46, (puStack46 >> 0x10), 0x1, puStack46, puStack46);
                        }
                    }
                }
            }
        }
    }
    return;
}


pub fn pass1_1028_7dfc(param_1: u32, param_2: *mut u8, param_3: i16, param_4: u16, param_5: u8) {
    let mut ppcVar1: *mut *mut c_void;
    let mut pu_var2: *mut u8;
    let mut uVar3: u16;
    let mut puVar4: *mut u8;
    let mut uVar5: u32;
    let mut uVar6: u8;
    let mut puVar7: *mut u16;
    let mut puVar8: *mut u32;
    let mut paVar9: *mut Struct100;
    let mut uVar10: u8;
    let mut local_158: u16;
    let mut uStack342: u16;
    let mut uStack72: u16;
    let mut uStack70: u16;
    let mut u_stack68: u32;
    let mut u_stack60: u32;
    let mut uStack56: u16;
    let mut uStack54: u16;
    let mut iStack52: i16;
    let mut uStack50: u32;
    let mut puStack46: *mut u32;
    let mut uStack42: u16;
    let mut puStack40: *mut u8;
    let mut uStack38: u32;
    let mut local_22: [u8;12] = [0;12];
    let mut iStack16: i16;
    let mut iStack14: i16;
    let mut uStack12: u32;
    let mut puStack8: *mut u8;
    let mut u_stack6: u16;
    let mut uStack4: u16;

    puVar7   = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_4, param_2, param_3);
    uStack4  = (puVar7 >> 0x10);
    u_stack6  = SUB42(puVar7, 0x0);
    puStack8 = globals.PTR_LOOP_1050_13ae;
    if(0x2 < globals.PTR_LOOP_1050_13ae) {
        uStack12       = *_PTR_LOOP_1050_65e2;
        uStack12 = (uStack12 >> 0x10);
        if(0x3 < uStack12) {
            iStack16 = uStack12 - 0x3;
            iStack14 = uStack12 - (uStack12 < 0x3);
            uVar5    = uStack12 % 0x14;
            if(uVar5 == 0x0) {
                uVar10 = (param_4 >> 0x8);
                pass1_1028_dc52(CONCAT13(uVar10, CONCAT12(param_4, local_22)), 0x1, 0x0, 0x400);
                while(true) {
                    uVar3  = uVar5;
                    pu_var2 = local_22;
                    pass1_1028_e4ec(str_var1(param_4, pu_var2));
                    uStack38 = str_var1(uVar3, pu_var2);
                    uVar5    = (uVar3 | pu_var2);
                    if((uVar3 | pu_var2) == 0x0)
                        break;
                    if((pu_var2 + 0x200) != 0x8000002) {
                        puVar8    = pass1_1008_c6fa(globals.dat_1050_06e0, 0x29);
                        puVar4    = (puVar8 >> 0x10);
                        uVar3     = puVar8;
                        uStack42  = uVar3;
                        puStack40 = puVar4;
                        pass1_1038_4d6e(uStack38, puVar8, uVar3, puVar4);
                        puStack46 = str_var1(puVar4, uVar3);
                        ppcVar1   = (*puStack46 + 0x10);
                        (**ppcVar1)(SEG_1038, uVar3, puVar4);
                        uStack50 = str_var1(puVar4, uVar3);
                        uVar6    = 0x10;
                        puVar7   = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_4, puVar4, param_3);
                        uVar5    = puVar7 >> 0x10;
                        uStack56 = SUB42(puVar7, 0x0);
                        uStack54 = (puVar7 >> 0x10);
                        if(puStack8 == (&PTR_LOOP_1050_0002 + 0x1)) {
                            iStack52 = 0x5;
                        } else {
                            iStack52 = 0x1e;
                        }
                        for(u_stack60 = 0x0; u_stack60 < uStack50; u_stack60 = u_stack60 + 0x1) {
                            u_stack68 = pass1_1030_1d7c(uStack50, uVar5, puStack46);
                            uVar5    = u_stack68 >> 0x10;
                            uVar3    = u_stack68;
                            uVar6    = 0x8;
                            pass1_1008_612e(0x1, 0x64, uVar3);
                            uStack70 = uVar3;
                            if((uVar3 <= iStack52) && (pass1_1028_7742(param_1, (param_1 >> 0x10), 0x4, u_stack68, param_4), uStack72 = uVar3, uVar3 == 0x0)) {
                                paVar9 = pass1_1028_b0de(CONCAT13(uVar10, CONCAT12(param_4, &local_158)), *(u_stack68 + 0x4), *(uStack38 + 0x4), param_4, param_5);
                                uVar5  = paVar9 >> 0x10;
                                uVar6  = 0x30;
                                fn_ptr_1030_835a(globals._PTR_LOOP_1050_5748,
                                                 str_var1(param_4, &local_158));
                                local_158 = addr_table_1008_380a[36]; // 0x389a
                                uStack342 = SEG_1008;
                            }
                        }
                        if(puStack46 != 0x0) {
                            ppcVar1 = *puStack46;
                            (**ppcVar1)(uVar6, puStack46, (puStack46 >> 0x10), 0x1, puStack46, puStack46);
                        }
                    }
                }
            }
        }
    }
    return;
}


pub fn pass1_1028_7fb6(param_1: u32, param_2: i16, param_3: u16, param_4: u8) {
    let mut ppcVar1: *mut *mut c_void;
    let mut pu_var2: *mut u8;
    let mut uVar3: u16;
    let mut puVar4: *mut u8;
    let mut uVar5: u32;
    let mut uVar6: u8;
    let mut puVar7: *mut u32;
    let mut puVar8: *mut u16;
    let mut paVar9: *mut Struct100;
    let mut uVar10: u8;
    let mut local_158: u16;
    let mut uStack342: u16;
    let mut uStack72: u16;
    let mut u_stack68: u16;
    let mut u_stack66: u16;
    let mut u_stack64: u32;
    let mut uStack56: u32;
    let mut iStack52: i16;
    let mut puStack50: *mut u8;
    let mut uStack48: u16;
    let mut uStack46: u16;
    let mut uStack44: u32;
    let mut puStack40: *mut u32;
    let mut uStack36: u16;
    let mut puStack34: *mut u8;
    let mut uStack32: u32;
    let mut local_1c: [u8;12] = [0;12];
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut u_stack6: u32;

    u_stack6       = *_PTR_LOOP_1050_65e2;
    u_stack6 = (u_stack6 >> 0x10);
    if(0xb < u_stack6) {
        iStack10 = u_stack6 - 0xb;
        iStack8  = u_stack6 - (u_stack6 < 0xb);
        uVar5    = u_stack6 % 0x32;
        if(uVar5 == 0x0) {
            uVar10 = (param_3 >> 0x8);
            pass1_1028_dc52(CONCAT13(uVar10, CONCAT12(param_3, local_1c)), 0x1, 0x0, 0x400);
            while(true) {
                uVar3  = uVar5;
                pu_var2 = local_1c;
                pass1_1028_e4ec(str_var1(param_3, pu_var2));
                uStack32 = str_var1(uVar3, pu_var2);
                uVar5    = (uVar3 | pu_var2);
                if((uVar3 | pu_var2) == 0x0)
                    break;
                if((pu_var2 + 0x200) != 0x8000002) {
                    puVar7    = pass1_1008_c6fa(globals.dat_1050_06e0, 0x11);
                    puVar4    = (puVar7 >> 0x10);
                    uVar3     = puVar7;
                    uStack36  = uVar3;
                    puStack34 = puVar4;
                    pass1_1038_4d6e(uStack32, puVar7, uVar3, puVar4);
                    puStack40 = str_var1(puVar4, uVar3);
                    ppcVar1   = (*puStack40 + 0x10);
                    (**ppcVar1)(SEG_1038, uVar3, puVar4);
                    uStack44  = str_var1(puVar4, uVar3);
                    uVar6     = 0x10;
                    puVar8    = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_3, puVar4, param_2);
                    uVar5     = puVar8 >> 0x10;
                    uStack48  = SUB42(puVar8, 0x0);
                    uStack46  = (puVar8 >> 0x10);
                    puStack50 = globals.PTR_LOOP_1050_13ae;
                    if(globals.PTR_LOOP_1050_13ae < 0x3) {
                        iStack52 = 0x5;
                    } else {
                        iStack52 = 0x14;
                    }
                    for(uStack56 = 0x0; uStack56 < uStack44; uStack56 = uStack56 + 0x1) {
                        uVar6    = 0x30;
                        u_stack64 = pass1_1030_1d7c(uStack44, uVar5, puStack40);
                        uVar5    = u_stack64 >> 0x10;
                        uVar3    = (u_stack64 + 0x20);
                        u_stack66 = uVar3;
                        if(((uVar3 != 0x0) && (uVar3 != 0x70)) && (uVar3 != 0x71)) {
                            uVar6 = 0x8;
                            pass1_1008_612e(0x1, 0x64, uVar3);
                            u_stack68 = uVar3;
                            if((uVar3 <= iStack52) && (pass1_1028_7742(param_1, (param_1 >> 0x10), 0x4, u_stack64, param_3), uStack72 = uVar3, uVar3 == 0x0)) {
                                paVar9 = pass1_1028_8698(CONCAT13(uVar10, CONCAT12(param_3, &local_158)), *(u_stack64 + 0x4), *(uStack32 + 0x4), param_4, param_3);
                                uVar5  = paVar9 >> 0x10;
                                uVar6  = 0x30;
                                fn_ptr_1030_835a(globals._PTR_LOOP_1050_5748,
                                                 str_var1(param_3, &local_158));
                                local_158 = addr_table_1008_380a[36]; // 0x389a
                                uStack342 = SEG_1008;
                            }
                        }
                    }
                    if(puStack40 != 0x0) {
                        ppcVar1 = *puStack40;
                        (**ppcVar1)(uVar6, puStack40, (puStack40 >> 0x10), 0x1, puStack40, puStack40);
                    }
                }
            }
        }
    }
    return;
}


pub fn pass1_1028_82b4(param_1: u32, param_2: u16, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x108, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10      = addr_table_1008_380a[36]; // 0x389a
        (param_2 + 0x2) = SEG_1008;
        uVar6           = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        puVar3 = (param_1 + 0x8);
        puVar5 = (param_2 + 0x8);
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        (param_2 + 0x2) = SEG_1028;
        *puStack10 = addr_table_1028_836e;//0x836e;
        (param_2 + 0x2) = SEG_1028;
    }
}


pub fn pass1_1028_69cc(param_1: *mut Struct316, param_2: *mut Struct317, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut iVar3: i16;
    let mut iVar5: *mut Struct316;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x10e, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10         = addr_table_1008_380a[36]; // 0x389a
        param_2->fld2_segment = SEG_1008;
        uVar6              = (param_1 >> 0x10);
        iVar5              = param_1;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        puVar5 = &param_2.field_0x8;
        for (iVar3 = 0x40; iVar3 != 0x0; iVar3 = iVar3- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar4;
            puVar4 = puVar4 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        param_2->fld2_segment = SEG_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        *puStack10 = addr_table_1028_6ad2[4];//0x6ae2;
        param_2->fld2_segment = SEG_1028;
    }
    return;
}


pub fn pass1_1028_6ef6(param_1: u32, param_2: u16, u8 *param_3) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x108, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10      = addr_table_1008_380a[36]; // 0x389a
        (param_2 + 0x2) = SEG_1008;
        uVar6           = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        puVar3 = (param_1 + 0x8);
        puVar5 = (param_2 + 0x8);
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        (param_2 + 0x2) = SEG_1028;
        *puStack10 = addr_table_1028_6fb0;//0x6fb0;
        (param_2 + 0x2) = SEG_1028;
    }
}


pub fn pass1_1028_737e(param_1: u32, param_2: u16, param_3: u16) {
    let mut puVar1: *mut u32;
    let mut pu_var2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x108, param_3, 0);
    puStack10 = str_var1(param_3, param_2);
    if((param_3 | param_2) != 0x0) {
        *puStack10      = addr_table_1008_380a[36]; // 0x389a
        (param_2 + 0x2) = SEG_1008;
        uVar6           = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        puVar3 = (param_1 + 0x8);
        puVar5 = (param_2 + 0x8);
        for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4- 1) {
            pu_var2 = puVar5;
            puVar5 = puVar5 + 0x1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 0x1;
            *pu_var2 = *puVar1;
        }
        *puStack10 = addr_table_1028_6ad2;//0x6ad2;
        (param_2 + 0x2) = SEG_1028;
        *puStack10 = addr_table_1028_749e;//0x749e;
        (param_2 + 0x2) = SEG_1028;
    }
    return;
}


u32 pass1_1028_611e(param_1: i16, param_2: u16, param_3: i16, param_4: u32, param_5: u16, u8 *param_6) {
    let mut uVar1: u32;

    pass1_1028_b39e(str_var1(param_2, param_1), param_3, param_4, param_6);
    (param_1 + 0x20) = 0x0;
    param_1 = addr_table_1028_6876;//0x6876;
    param_1.fld2_segment = SEG_1028;
    mem_op_1000_179c(0xc, param_6, 0);
    if ((param_6 | param_5) == 0x0) {
        (param_1 + 0x20) = 0x0;
    } else {
        uVar1 = set_struct_1008_574a(str_var1(param_6, param_5));
        (param_1 + 0x20) = uVar1;
        (param_1 + 0x22) = (uVar1 >> 0x10);
    }
    return param_1;
}


pub fn pass1_1028_4194(param_1: *mut u32, param_2: u16, param_3: u16, param_4: u16, param_5: u16) {
    let mut puVar1: *mut u8;
    let mut u_var2: u32;
    let mut puVar3: *mut u16;

    pass1_1028_be9e(param_1, param_2, param_3, param_4, param_5);
    u_var2  = pass1_1028_b4f2(param_1);
    puVar1 = (u_var2 >> 0x10);
    if(((u_var2 + 0x200) != 0x8000002) && ((param_1 + 0x12) == 0x5)) {
        puVar3 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2b, param_5, puVar1, param_3);
        pass1_1010_043a(puVar3, (u_var2 + 0x4), 0xe, param_5);
    }
    return;
}


pub fn pass1_1028_43c2(param_1: i16, param_2: u16, param_3: i16, param_4: *mut u8, param_5: i16, param_6: u16) {
    let mut puVar1: *mut u16;

    pass1_1028_bd38(str_var1(param_2, param_1), param_4, param_6);
    if(param_3 == 0x0) {
        puVar1 = mixed_1010_20ba(globals.data_1050_0ed0, 0x8, param_6, param_4, param_5);
        pass1_1010_988c(puVar1, (param_1 + 0xc));
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_43f6(param_1: u32, param_2: i16, param_3: *mut u8, param_4: u16, param_5: i16, param_6: u16) {
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u32;
    let mut uVar4: u16;

    uVar1  = 0x83;
    pu_var2 = mixed_1010_20ba(globals.data_1050_0ed0, 0x9, param_6, param_3, param_5);
    uVar1  = pass1_1010_65d0(param_6, pu_var2, uVar1);
    if(0x0 < uVar1) {
        uVar3 = pass1_1028_b58e(param_1);
        if(param_2 == 0x0) {
            uVar4 = 0x0;
        } else {
            uVar4 = 0x3e8;
        }
        pass1_1030_7d1c(uVar3, uVar4, 0x230000, uVar3, (uVar3 >> 0x10), param_4, param_5, param_6);
    }
    return;
}


pub fn pass1_1028_45fe(param_1: *mut Struct312, param_2: i16, param_3: u16) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut puVar3: *mut u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: *mut u8;
    let mut puVar4: *mut u8;
    let mut extraout_DX_01: u16;
    let mut uVar5: u16;
    let mut iVar6: *mut Struct312;
    let mut iVar5: *mut Struct314;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut paStack44: *mut Struct99;
    long         local_28;
    let mut puStack34: *mut u32;
    let mut puStack32: *mut u8;
    let mut paStack30: *mut Struct99;
    i16          local_1a[0x4];
    let mut uStack18: u32;
    let mut uStack14: u32;
    let mut puStack10: *mut u32;
    let mut u_stack6: u32;
    let mut u_var2: *mut Struct313;

    pass1_1028_b58e(param_1);
    u_stack6   = str_var1(extraout_DX, param_2);
    puStack10 = (param_2 + 0x22);
    uVar6     = (param_1 >> 0x10);
    iVar6     = param_1;
    // WARNING: Load size is inaccurate
    puVar3    = iVar6.field_0x20;
    puVar4    = (&iVar6.field_0x20 + 0x2);
    paStack30 = str_var1(puVar4, puVar3);
    puStack34 = puVar3;
    puStack32 = puVar4;
    if((puVar4 | puVar3) != 0x0) {
        ppcVar2 = *puVar3;
        (**ppcVar2)();
        puVar4 = extraout_DX_00;
    }
    mem_op_1000_179c(0xc, puVar4, 0);
    puStack34 = puVar3;
    puStack32 = puVar4;
    if((puVar4 | puVar3) == 0x0) {
        puVar3 = 0x0;
        uVar7  = 0x0;
    } else {
        set_struct_1008_574a(str_var1(puVar4, puVar3));
        uVar7 = extraout_DX_01;
    }
    &iVar6.field_0x20 = puVar3;
    (&iVar6.field_0x20 + 0x2)         = uVar7;
    if(puStack10 != 0x0) {
        uStack14 = (puStack10 + 0x4);
        for(uStack18 = 0x0; uStack18 < uStack14; uStack18 = uStack18 + 0x1) {
            pass1_1020_bb16(puStack10, CONCAT13((param_3 >> 0x8), CONCAT12(param_3, &local_28)),
                            str_var1(param_3, local_1a), uStack18);
            if((local_28 != 0x0) && (local_1a[0] != 0x0)) {
                paStack30 = pass1_1000_07fc(SEG_1000, globals.u32_ptr_1050_68a2);
                uVar5 = (paStack30 >> 0x10);
                u_var2     = paStack30;
                if((uVar5 | u_var2) == 0x0) {
                    paStack44 = 0x0;
                } else {
                    paStack30->fld0_addr_table = addr_table_1008_380a[36]; // 0x389a
                    u_var2->fld2_segment       = SEG_1008;
                    u_var2.field_0x4     = 0x0;
                    u_var2.field_0x6     = 0x0;
                    u_var2.field_0x8     = 0x0;
                    u_var2.field_0xa     = 0x0;
                    u_var2.field_0xc     = 0x0;
                    paStack30->fld0_addr_table = addr_table_1018_56ce; // 0x56ce
                    u_var2->fld2_segment       = SEG_1018;
                    paStack44            = paStack30;
                }
                uVar7            = (paStack44 >> 0x10);
                iVar5            = paStack44;
                iVar5.field_0x4 = local_1a[0];
                iVar5.field_0xa = local_28;
                iVar5.field_0xc = local_28;
                puVar1           = iVar6.field_0x20;
                ppcVar2          = (*iVar6.field_0x20 + 0x8);
                (**ppcVar2)(SEG_1000, puVar1, (puVar1 >> 0x10), iVar5, uVar7);
            }
        }
    }
    return;
}