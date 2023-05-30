
// #include "sys_ops_6.h"

// #include "address_tables/function_tables.h"
// #include "fn_ptr_ops/fn_ptr_ops_7.h"
// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "struct_20.h"
// #include "struct_43.h"
// #include "struct_ops/struct_ops_3.h"
// #include "struct_ops/struct_ops_4.h"
// #include "sys_ops_12.h"
// #include "sys_ops_7.h"
// #include "unk/unk_15.h"

pub fn pass1_1020_04f6(param_1: *mut u16, param_2: u16, param_3: *mut u8, param_4: i16, param_5: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut dx_var1: *mut u8;
    let mut iVar4: *mut Struct662;
    let mut u_var4: u16;
    let mut puVar5: *mut u16;

    u_var4 = (param_1 >> 0x10);
    iVar4 =  param_1;
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a
    iVar4->fld2_segment = SEG_1008;
    param_1.field_0x0 = addr_table_1008_3aa0[2];//0x3aa8;
    iVar4->fld2_segment = SEG_1008;
    iVar4.field_0x4 = param_2;
    param_1.field_0x0 = addr_table_1008_3aa0[4]; // 0x3ab0;
    iVar4->fld2_segment = SEG_1008;
    iVar4.field_0x6 = 0x0;
    iVar4.field_0xa = 0x0;
    iVar4.field_0xc = 0x0;
    iVar4.field_0xe = 0x0;
    iVar4.field_0x10 = 0x0;
    param_1.field_0x0 = addr_table_1020_075a;//0x75a;
    iVar4->fld2_segment = SEG_1020;
    puVar5 = mixed_1010_20ba(globals.data_1050_0ed0, 0x1, param_5, param_3, param_4);
    uVar3 = (puVar5 >> 0x10);
    &iVar4.field_0x6 = puVar5;
    (&iVar4.field_0x6 + 0x2) = uVar3;
    ppcVar1 = (*iVar4.field_0x6 + 0x4);
    (**ppcVar1)(SEG_1010, &iVar4.field_0x6, uVar3, 0x0, param_1);
    puVar5 = mixed_1010_20ba(globals.data_1050_0ed0, 0x48, param_5, dx_var1, param_4);
    iVar2 = puVar5;
    iVar4.field_0xa = (iVar2 + 0xa);
    iVar4.field_0xc = (iVar2 + 0xc);
    pass1_1008_3e94((puVar5 & 0xffff0000 | (iVar2 + 0xe)), (param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x10)), (param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0xe)));
}

u32 pass1_1018_dd1e(param_1: u16, param_2: u16, param_3: *mut u8, param_4: u16, param_5: u16, param_6: i16, param_7: u32)

{
    let mut uVar1: u16;
    let mut u_stack6: u32;

    pass1_1010_81f6(SEG_1010, param_1, globals.dat_1050_14cc, 0x0, param_7);
    u_stack6 = str_var1(param_3, param_2);
    mem_op_1000_179c(0x46, param_3, 0);
    uVar1 = param_3 | param_2;
    if(uVar1 == 0x0)
    {
        param_2 = 0x0;
        uVar1   = 0x0;
    }
    else
    {
        pass1_1008_87cc(str_var1(param_3, param_2), param_6, param_7, param_7, u_stack6, 0x0, param_1);
    }
    pass1_1008_8bc6(param_1, uVar1, str_var1(uVar1, param_2));
    return str_var1(uVar1, param_2);
}

u16 *struct_1018_e100(param_1: *mut u16, param_2: u16, param_3: *mut u8, param_4: u16) {
    let mut iVar1: *mut Struct268;
    let mut unaff_DI: i16;
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a
    ivar1.fld2_segment = SEG_1008;
    param_1.field_0x0 = addr_table_1008_3aa0[2];//0x3aa8;
    ivar1.fld2_segment = SEG_1008;
    iVar1.field_0x4 = param_2;
    param_1.field_0x0 = addr_table_1008_3aa0[4]; // 0x3ab0;
    ivar1.fld2_segment = SEG_1008;
    &iVar1.field_0x6 = 0x0;
    param_1.field_0x0 = addr_table_1018_e228;//0xe228;
    ivar1.fld2_segment = SEG_1018;
    pu_var2 = mixed_1010_20ba(globals.data_1050_0ed0, 0x36, param_4, param_3, unaff_DI);
    iVar1.field_0x6 = pu_var2;
    iVar1.field_0x8 = (pu_var2 >> 0x10);
    return param_1;
}

pub fn pass1_1018_c402(param_1: &mut Struct20, param_2: u16, param_3: u16, param_4: u16, param_5: u32, param_6: u32, param_7: u32, param_8: u32, param_9: u16, param_10: u16)

{
    let mut iVar1: i16;
    let mut pu_var2: *mut u16;
    let mut iVar4: *mut Struct20;
    let mut unaff_DI: i16;
    let mut u_var4: *mut Struct20;
    let mut puVar3: *mut u16;

    struct_1020_0762(param_1,
                     str_var1(param_5, param_4),
                     str_var1(param_6, (param_5 >> 0x10)), (param_6 >> 0x10), param_7, param_8, param_9);
    u_var4                 = (param_1 >> 0x10);
    iVar4                 = param_1;
    iVar4[0x1].field_0x14 = 0x0;
    iVar4[0x1].field_0x16 = 0x0;
    iVar4[0x1].field_0x18 = 0x0;
    iVar4[0x1].field_0x1a_addr_offset = 0x0;
    iVar4[0x1].field_0x1c_addr_base = 0x2;
    iVar4[0x1].field_0x26 = 0x0;
    iVar4[0x1].field_0x2a = 0x0;
    iVar4[0x1].field_0x2c = 0x1e0190;
    iVar4[0x1].field_0x30 = 0x0;
    param_1.field_0x0    = addr_table_1018_c8bc;//0xc8bc;
    iVar4.field_0x2      = SEG_1018;
    pu_var2                = pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x1e)), 0x0, 0x8);
    if((param_3 == 0x0) || (param_2 != 0x0))
    {
        if((param_2 & param_3) == 0x0)
            //goto LAB_1018_c4bb;
        pu_var2 = pass1_1008_5fd8(param_9, param_10);
    }
    else
    {
        load_string_1010_84acglobals.dat_1050_14cc, SEG_1010);
    }
    *(u16 **)&iVar4[0x1].field_0x26 = pu_var2;
    (&iVar4[0x1].field_0x26 + 0x2)  = param_10;
// LAB_1018_c4bb:
    puVar3                = mixed_1010_20ba(globals.data_1050_0ed0, 0x48, param_9, param_10, unaff_DI);
    iVar1                 = puVar3;
    iVar4[0x1].field_0x14 = (iVar1 + 0xa);
    iVar4[0x1].field_0x16 = (iVar1 + 0xc);
    pass1_1008_3e94((puVar3 & 0xffff0000 | (iVar1 + 0xe)), (param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x1a_addr_offset)), (param_1 & 0xffff0000 | ZEXT24(&iVar4[0x1].field_0x18)));
}

Struct57 *pass1_1018_5e26(param_1: *mut Struct57, param_2: u16)

{
    let mut uVar1: u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfd0, param_2);
    uVar1            = (param_1 >> 0x10);
    param_1          = addr_table_1018_6128;//0x6128;
    param_1.field_0x2 = SEG_1018;
    (param_1 + 0x74) = 0x1;
    return param_1;
}

pub fn pass1_1018_6198(param_1: *mut u16, param_2: u32, param_3: u16, param_4: *mut u8, param_5: i16, param_6: u16) {
    let mut iVar1: *mut Struct657;
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a
    iVar1.field_0x2 = SEG_1008;
    param_1.field_0x0 = adddr_1008_3aa0[2];//0x3aa8;
    iVar1.field_0x2 = SEG_1008;
    iVar1.field_0x4 = param_3;
    param_1.field_0x0 = addr_table_1008_3aa0[4]; // 0x3ab0;
    iVar1.field_0x2 = SEG_1008;
    &iVar1.field_0x6 = 0x0;
    iVar1.field_0xa = param_2;
    param_1.field_0x0 = addr_table_1018_66c0;//0x66c0;
    iVar1.field_0x2 = SEG_1018;
    pu_var2 = mixed_1010_20ba(globals.data_1050_0ed0, 0x39, param_6, param_4, param_5);
    iVar1.field_0x6 = pu_var2;
    iVar1.field_0x8 = (pu_var2 >> 0x10);
    return;
}

u32 pass1_1018_659a(param_1: u16, param_2: u16,param_3: *mut u16, param_4: *mut u8, param_5: u16)

{
    let mut pi_var1: *mut i16;
    let mut iStack18: i16;
    let mut local_6: i16;
    let mut local_4: i16;

    pi_var1 = &local_6;
    pass1_1008_3e94(param_3, str_var1(param_5, pi_var1), str_var1(param_5, &local_4));
    mem_op_1000_179c(0xc, param_4, 0);
    for(iStack18 = 0x0; iStack18 < 0x3; iStack18 = iStack18 + 0x1)
    {
        pi_var1[iStack18 * 0x2]       = (iStack18 * 0x4 + 0x4248) + local_4;
        pi_var1[iStack18 * 0x2 + 0x1] = (iStack18 * 0x4 + 0x424a) + local_6;
    }
    return str_var1(param_4, pi_var1);
}

u16 *get_sys_metrics_1018_4b1e(param_1: *mut Struct55, param_2: u16, param_3: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u16;

    struct_op_1010_1d48(param_1, param_3);
    u_var2              = (param_1 >> 0x10);
    iVar1              = param_1;
    (iVar1 + 0x12)     = param_2;
    (iVar1 + 0x14)     = 0x0;
    param_1.field_0x0 = addr_table_1018_4c9e;//0x4c9e; // &PTR_LOOP_1050_4c9e;
    (iVar1 + 0x2)      = SEG_1018;
    if(globals.PTR_LOOP_1050_416c == 0x0)
    {
        globals.PTR_LOOP_1050_416c = GetSystemMetrics16(SEG_1010);
        globals.PTR_LOOP_1050_416e = GetSystemMetrics16(LAST_SEGMENT);
        globals.PTR_LOOP_1050_4170 = GetSystemMetrics16(LAST_SEGMENT);
    }
    return &param_1.field_0x0;
}

pub fn pass1_1018_4b78(param_1: *mut u32, param_2: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut pu_var2: *mut u8;
    let mut uVar3: u16;
    let mut pu_var4: *mut u16;
    let mut puVar5: *mut u32;

    pu_var2 = param_1;
    pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24((param_1 + 0xa))), 0x0, 0x8);
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x18)), 0x0, 0x8);
    pu_var4 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_2, pu_var2, param_1);
    puVar5 = pass1_1010_5f7a(pu_var4, (pu_var4 >> 0x10), 0x0, (param_1 + 0x12));
    uVar3  = (puVar5 >> 0x10);
    if((uVar3 | puVar5) != 0x0)
    {
        (param_1 + 0xa) = *puVar5;
        (param_1 + 0xe) = (puVar5 + 0x4);
    }
    ppcVar1 = (*param_1 + 0x20);
    (**ppcVar1)(SEG_1010, param_1);
    if(((param_1 + 0xe) == 0x0) && ((param_1 + 0x10) == 0x0))
    {
        (param_1 + 0xa) = (param_1 + 0x18);
        (param_1 + 0xc) = (param_1 + 0x1a);
    }
    (param_1 + 0xe)  = (param_1 + 0x1c);
    (param_1 + 0x10) = (param_1 + 0x1e);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_4c2c(param_1: u32, param_2: *mut u32, param_3: u16, param_4: u16)

{
    let mut puVar1: *mut u16;

    *(param_1 + 0xa) = *param_2;
    *(param_1 + 0xe) = param_2[0x1];
    puVar1           = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_4, param_1, param_1);
    pass1_1010_5fb0(puVar1, 0x0, (param_1 + 0xa), param_1, (param_1 + 0x12));
    return;
}
pub fn pass1_1018_4dce(param_1: *mut u32, param_2: u16, param_3: *mut u8, param_4: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u16;
    let mut unaff_DI: i16;
    let mut puVar3: *mut u16;

    puVar3  = mixed_1010_20ba(globals.data_1050_0ed0, 0x48, param_4, param_3, unaff_DI);
    u_var2   = (puVar3 >> 0x10);
    ppcVar1 = (*param_1 + 0x10);
    (**ppcVar1)(SEG_1010, param_1, param_2, (puVar3 + 0xc), (puVar3 + 0xa));
    return;
}

pub fn pass1_1018_5292(param_1: u32, param_2: u32, param_3: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u32;
    let mut ppcVar3: *mut *mut c_void;
    let mut u_var4: u16;
    let mut BVar5: BOOL16;
    let mut puVar6: *mut u8;
    let mut iVar7: i16;
    let mut pcVar8: *mut c_char;
    let mut uVar9: u16;
    let mut puVar10: *mut u32;
    let mut puVar11: *mut u32;
    let mut uVar12: u32;
    let mut dx_var1: *mut u8;
    let mut dx_var1_00: *mut u8;
    let mut uVar13: u16;
    let mut dx_var1_01: *mut u8;
    let mut puVar14: *mut u8;
    let mut dx_var1_02: u16;
    let mut dx_var1_03: u16;
    let mut puVar15: *mut u8;
    let mut dx_var1_04: *mut u8;
    let mut uVar16: u16;
    let mut dx_var1_05: u16;
    let mut dx_var1_06: u16;
    let mut dx_var1_07: *mut u8;
    let mut dx_var1_08: *mut u8;
    let mut iVar17: i16;
    let mut uVar18: u16;
    let mut uVar19: u16;
    let mut pu_var20: *mut u16;
    let mut uStack50: u16;
    let mut local_26: [u8;8] = [0;8];
    let mut uStack30: u32;
    let mut uStack26: u32;
    let mut uStack22: u32;
    let mut puStack18: *mut u32;
    let mut puStack16: *mut u8;
    let mut puStack14: *mut u32;
    let mut puStack12: *mut u8;
    let mut uStack10: u16;
    let mut uStack8: u32;
    let mut uStack4: u16;

    uVar18    = (param_1 >> 0x10);
    iVar17    = param_1;
    puStack18 = (iVar17 + 0xe);
    uVar12    = ZEXT24(puStack18);
    puVar14   = (iVar17 + 0x10);
    puStack16 = puVar14;
    puStack14 = puStack18;
    puStack12 = puVar14;
    if((puVar14 | puStack18) != 0x0)
    {
        ppcVar3 = *puStack18;
        (**ppcVar3)();
        puVar14 = dx_var1;
    }
    mem_op_1000_179c(0xc, puVar14, 0);
    puStack18 = uVar12;
    puStack16 = puVar14;
    if((puVar14 | puStack18) == 0x0)
    {
        uVar12  = 0x0;
        puVar14 = 0x0;
    }
    else
    {
        set_struct_1008_574a((uVar12 & 0xffff | ZEXT24(puVar14) << 0x10));
        puVar14 = dx_var1_00;
    }
    (iVar17 + 0xe)  = uVar12;
    (iVar17 + 0x10) = puVar14;
    for(uStack4 = 0x21; - 0x1 < uStack4; uStack4 = uStack4 - 0x1)
    {
        uStack22 = pass1_1030_7c28(param_2, uStack4, uVar12, puVar14, param_3);
        uVar12   = uStack22 & 0xffff;
        uVar13   = uVar12;
        puVar14  = ((uStack22 >> 0x10) | uVar13);
        if(puVar14 != 0x0)
        {
            string_1020_c0ca(uStack4);
            u_var4    = str_op_1008_60e8(str_var1(puVar14, uVar13));
            uVar12   = u_var4;
            uStack26 = str_var1(puVar14, u_var4);
            mem_op_1000_179c(0x10, puVar14, 0);
            puStack14 = uVar12;
            puStack12 = puVar14;
            if((puVar14 | puStack14) == 0x0)
            {
                uVar12 = 0x0;
                uVar13 = 0x0;
            }
            else
            {
                struct_1018_4790(uVar12 & 0xffff | ZEXT24(puVar14) << 0x10, uStack22, uStack26, uStack4);
                uVar13 = dx_var1_02;
            }
            uStack30 = uVar12 & 0xffff | uVar13 << 0x10;
            u_var2    = (iVar17 + 0xe);
            ppcVar3  = ((iVar17 + 0xe) + 0x4);
            (**ppcVar3)(SEG_1000, u_var2, (u_var2 >> 0x10), uVar12, uVar13);
            puVar14 = dx_var1_01;
        }
    }
    uStack8  = struct_op_1030_73a8(param_2);
    uStack10 = (uStack8 + 0xc);
    BVar5    = pass1_1008_c6ae(globals.dat_1050_06e0, uStack10, 0x4);
    if(BVar5 != 0x0)
    {
        uStack30 = uStack8;
        uStack26 = *(uStack8 + 0x20);
        pass1_1008_5784(str_var1(param_3, local_26), uStack26);
        while(true)
        {
            puVar6 = local_26;
            pass1_1008_5b12(puVar6, param_3);
            uStack22 = str_var1(dx_var1_03, puVar6);
            puVar14  = (dx_var1_03 | puVar6);
            if(puVar14 == 0x0)
                break;
            iVar1 = (puVar6 + 0x6);
            iVar7 = iVar1 + -0x7;
            if(iVar7 == 0x0)
            {
            // LAB_1018_53f0:
                pcVar8  = string_op_1020_c222((puVar6 + 0x6));
                uVar9   = str_op_1008_60e8(str_var1(puVar14, pcVar8));
                puVar15 = puVar14;
                u_var4 = uVar9;
                mem_op_1000_179c(0x10, puVar14, 0);
                puStack18 = u_var4;
                puStack16 = puVar15;
                if((puVar15 | u_var4) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    uVar19  = (uStack22 >> 0x10);
                    pu_var20 = struct_1018_48b0(str_var1(puVar15, u_var4), (uStack22 + 0xa),
                                                str_var1(puVar14, uVar9), (uStack22 + 0x6));
                    uVar16  = (pu_var20 >> 0x10);
                    uVar19  = SUB42(pu_var20, 0x0);
                }
                u_var2   = (iVar17 + 0xe);
                ppcVar3 = ((iVar17 + 0xe) + 0x4);
                (**ppcVar3)(SEG_1000, u_var2, (u_var2 >> 0x10), uVar19, uVar16);
                puVar14 = dx_var1_04;
            }
            else
            {
                if(((0x5 < iVar7) && (!SBORROW2(iVar7, 0x6))) && (iVar1 + -0xd < 0x2))
                    //goto LAB_1018_53f0;
            }
            uVar19 = (uStack22 >> 0x10);
            if((uStack22 + 0x8) != 0x0)
            {
                pcVar8  = string_op_1020_c2f8((uStack22 + 0x8));
                puVar10 = str_op_1008_60e8(str_var1(puVar14, pcVar8));
                puVar15 = puVar14;
                puVar11 = puVar10;
                mem_op_1000_179c(0x10, puVar14, 0);
                puStack14 = puVar11;
                puStack12 = puVar15;
                if((puVar15 | puVar11) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    uVar19  = (uStack22 >> 0x10);
                    pu_var20 = struct_1018_48e8(str_var1(puVar15, puVar11), (uStack22 + 0xa),
                                                str_var1(puVar14, puVar10), (uStack22 + 0x8));
                    uVar16  = (pu_var20 >> 0x10);
                    uVar19  = SUB42(pu_var20, 0x0);
                }
                u_var2   = (iVar17 + 0xe);
                ppcVar3 = ((iVar17 + 0xe) + 0x4);
                (**ppcVar3)(SEG_1000, u_var2, (u_var2 >> 0x10), uVar19, uVar16);
            }
        }
    }
    uVar19   = (param_2 >> 0x10);
    uVar12   = *(param_2 + 0x3e);
    uVar13   = (param_2 + 0x40);
    uStack50 = uVar12;
    if((uVar13 | uStack50) != 0x0)
    {
        pass1_1008_5784(str_var1(param_3, local_26), uVar12 & 0xffff | uVar13 << 0x10);
        while(true)
        {
            puVar6 = local_26;
            pass1_1008_5b12(puVar6, param_3);
            puVar14 = (dx_var1_05 | puVar6);
            if(puVar14 == 0x0)
                break;
            if((puVar6 + 0x4) != 0x0)
            {
                pcVar8   = string_1020_c0d8((puVar6 + 0x4));
                uVar13   = str_op_1008_60e8(str_var1(puVar14, pcVar8));
                uStack30 = str_var1(puVar14, uVar13);
                mem_op_1000_179c(0x10, puVar14, 0);
                puStack18 = uVar13;
                puStack16 = puVar14;
                if((puVar14 | uVar13) == 0x0)
                {
                    uVar13 = 0x0;
                    uVar19 = 0x0;
                }
                else
                {
                    struct_1018_4790(str_var1(puVar14, uVar13), (puVar6 + 0xa), uStack30, (puVar6 + 0x4));
                    uVar19 = dx_var1_06;
                }
                uStack26 = str_var1(uVar19, uVar13);
                u_var2    = (iVar17 + 0xe);
                ppcVar3  = ((iVar17 + 0xe) + 0x4);
                (**ppcVar3)(SEG_1000, u_var2, (u_var2 >> 0x10), uVar13, uVar19);
                puVar14 = dx_var1_07;
            }
            if((puVar6 + 0x6) != 0x0)
            {
                pcVar8   = string_op_1020_c222((puVar6 + 0x6));
                puVar11  = str_op_1008_60e8(str_var1(puVar14, pcVar8));
                uStack30 = str_var1(puVar14, puVar11);
                mem_op_1000_179c(0x10, puVar14, 0);
                puStack14 = puVar11;
                puStack12 = puVar14;
                if((puVar14 | puVar11) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    pu_var20 = struct_1018_48b0(str_var1(puVar14, puVar11), (puVar6 + 0xa), uStack30, (puVar6 + 0x6));
                    uVar16  = (pu_var20 >> 0x10);
                    uVar19  = SUB42(pu_var20, 0x0);
                }
                uStack26 = str_var1(uVar16, uVar19);
                u_var2    = (iVar17 + 0xe);
                ppcVar3  = ((iVar17 + 0xe) + 0x4);
                (**ppcVar3)(SEG_1000, u_var2, (u_var2 >> 0x10), uVar19, uVar16);
                puVar14 = dx_var1_08;
            }
            if((puVar6 + 0x8) != 0x0)
            {
                pcVar8   = string_op_1020_c2f8((puVar6 + 0x8));
                uVar13   = str_op_1008_60e8(str_var1(puVar14, pcVar8));
                uStack30 = str_var1(puVar14, uVar13);
                mem_op_1000_179c(0x10, puVar14, 0);
                puStack18 = uVar13;
                puStack16 = puVar14;
                if((puVar14 | uVar13) == 0x0)
                {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                }
                else
                {
                    pu_var20 = struct_1018_48e8(str_var1(puVar14, uVar13), (puVar6 + 0xa), uStack30, (puVar6 + 0x8));
                    uVar16  = (pu_var20 >> 0x10);
                    uVar19  = SUB42(pu_var20, 0x0);
                }
                uStack26 = str_var1(uVar16, uVar19);
                u_var2    = (iVar17 + 0xe);
                ppcVar3  = ((iVar17 + 0xe) + 0x4);
                (**ppcVar3)(SEG_1000, u_var2, (u_var2 >> 0x10), uVar19, uVar16);
            }
        }
    }
    return;
}

pub fn switch_1018_3ee6(param_1: u32, long param_2, param_3: i16, param_4: u16, param_5: u8)

{
    let mut iVar1: i16;
    let mut pcVar2: *mut c_char;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut puVar8: *mut u8;
    let mut ss_var1: u16;
    let mut puVar9: *mut u16;
    let mut lVar10 = 0i32;
    let mut iVar11: i16;
    let mut IVar12: u16;
    let mut uVar13: u16;
    let mut uStack26: u32;
    let mut puStack22: *mut u16;
    let mut lStack18 = 0i32;
    let mut lStack14 = 0i32;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut piStack6: *mut i16;

    switch(param_4)
    {
    0x1 =>
        iVar1 = param_3 * 0x4 + 0x40b6;
        break;
    _ =>
        iVar1 = param_3 * 0x4 + 0x40ce;
        break;
     3 =>
        iVar1 = param_3 * 0x4 + 0x40e2;
        break;
    0x4 =>
        iVar1 = param_3 * 0x4 + 0x40ee;
        break;
    0x8 =>
        iVar1 = param_3 * 0x4 + 0x40f2;
        break;
    0x9 =>
        iVar1 = param_3 * 0x4 + 0x4106;
        break;
    0xa =>
        iVar1 = param_3 * 0x4 + 0x410a;
        break;
    0x14 =>
        iVar1 = param_3 * 0x4 + 0x410e;
        break;
    0x16 =>
        iVar1 = param_3 * 0x4 + 0x4112;
        break;
    0x17 =>
        iVar1 = param_3 * 0x4 + 0x4116;
        break;
    0x19 =>
        iVar1 = param_3 * 0x4 + 0x411a;
    }
    piStack6 = str_var1(0x1050, iVar1);
    if(piStack6 == 0x0)
    {
        return;
    }
    iStack10 = 0x0;
    uStack8  = 0x0;
    iVar11   = *piStack6;
    uVar13   = param_1;
    uVar3    = (param_1 >> 0x10);
    if(iVar11 == 0x1)
    {
        uVar13   = pass1_1018_456a(uVar13, uVar3, (iVar1 + 0x2));
        lStack14 = str_var1(param_5, uVar13);
        pcVar2   = string_1020_c0d8((iVar1 + 0x2));
        uVar3    = str_op_1008_60e8(str_var1(param_5, pcVar2));
        puVar8   = param_5;
        uVar13 = uVar3;
        mem_op_1000_179c(0x10, param_5, 0);
        puStack22 = str_var1(puVar8, uVar13);
        if((puVar8 | uVar13) != 0x0)
        {
            lVar10  = param_2 / lStack14;
            uStack8 = (param_2 % lStack14);
            struct_1018_4790(puStack22, lVar10, str_var1(param_5, uVar3), (iVar1 + 0x2));
            iStack10 = lVar10;
            //goto LAB_1018_425e;
        }
    }
    else
    {
        if(iVar11 == 0x2)
        {
            uVar13   = pass1_1018_451e(uVar13, uVar3, (iVar1 + 0x2));
            lStack18 = str_var1(param_5, uVar13);
            pcVar2   = string_op_1020_c222((iVar1 + 0x2));
            uVar3    = str_op_1008_60e8(str_var1(param_5, pcVar2));
            puVar8   = param_5;
            uVar13 = uVar3;
            mem_op_1000_179c(0x10, param_5, 0);
            puStack22 = str_var1(puVar8, uVar13);
            if((puVar8 | uVar13) != 0x0)
            {
                puVar9   = struct_1018_48b0(puStack22, param_2 / lStack18, str_var1(param_5, uVar3), (iVar1 + 0x2));
                uStack8  = (puVar9 >> 0x10);
                iStack10 = puVar9;
                //goto LAB_1018_425e;
            }
        }
        else
        {
            if(iVar11 == 0x3)
            {
                u_var4 = pass1_1008_c646(globals.dat_1050_06e0,
                  str_var1((iVar1 + 0x2), (globals.dat_1050_06e0 >> 0x10)), ss_var1);
                if(u_var4 == 0x0)
                {
                    u_var4 = 0x4f;
                }
                uVar13   = switch_1018_43ec(uVar13, uVar3, u_var4);
                lStack14 = str_var1(param_5, uVar13);
                uVar13   = pass1_1020_bd80(u_var4);
                uVar5    = str_op_1008_60e8(str_var1(param_5, uVar13));
                uStack26 = str_var1(param_5, uVar5);
                mem_op_1000_179c(0x14, param_5, 0);
                puStack22 = str_var1(param_5, uVar5);
                if((param_5 | uVar5) != 0x0)
                {
                    uVar7   = param_2 / lStack14;
                    uStack8 = (param_2 % lStack14);
                    struct_1018_47c8(puStack22, uVar7, uStack26, u_var4, 0x0);
                    iStack10 = uVar7;
                    //goto LAB_1018_425e;
                }
            }
            else
            {
                if(iVar11 != 0x4)
                    //goto LAB_1018_425e;
                iVar1  = (iVar1 + 0x2);
                uVar5  = iVar1 - 0x1;
                iVar11 = globals.dat_1050_14cc;
                IVar12 = globals.dat_1050_14cc >> 0x10);
                if(uVar5 == 0x0)
                {
                    load_string_1010_84ac(iVar11, SEG_1010);
                    uVar6  = uVar5;
                    puVar8 = param_5;
                    mem_op_1000_179c(0x14, param_5, 0);
                    puStack22 = str_var1(puVar8, uVar6);
                    if((puVar8 | uVar6) != 0x0)
                    {
                        uVar13 = 0x2;
                        lVar10 = 0x14;
                    // LAB_1018_4230:
                        puVar9   = struct_1018_4842(puStack22, param_2 / lVar10, str_var1(param_5, uVar5), uVar13);
                        uStack8  = (puVar9 >> 0x10);
                        iStack10 = puVar9;
                        //goto LAB_1018_425e;
                    }
                }
                else
                {
                    uVar5 = iVar1 - 0x2;
                    if(uVar5 == 0x0)
                    {
                        load_string_1010_84ac(iVar11, SEG_1010);
                        uVar6  = uVar5;
                        puVar8 = param_5;
                        mem_op_1000_179c(0x14, param_5, 0);
                        puStack22 = str_var1(puVar8, uVar6);
                        if((puVar8 | uVar6) != 0x0)
                        {
                            uVar13 = 0x3;
                            lVar10 = 0x16;
                            //goto LAB_1018_4230;
                        }
                    }
                    else
                    {
                        uVar5 = iVar1 - 0x3;
                        if(uVar5 == 0x0)
                        {
                            load_string_1010_84ac(iVar11, SEG_1010);
                            uVar6  = uVar5;
                            puVar8 = param_5;
                            mem_op_1000_179c(0x14, param_5, 0);
                            puStack22 = str_var1(puVar8, uVar6);
                            if((puVar8 | uVar6) != 0x0)
                            {
                                uVar13 = 0x4;
                                lVar10 = 0x17;
                                //goto LAB_1018_4230;
                            }
                        }
                        else
                        {
                            uVar5 = iVar1 - 0x4;
                            if(uVar5 != 0x0)
                                //goto LAB_1018_425e;
                            load_string_1010_84ac(iVar11, SEG_1010);
                            uVar6  = uVar5;
                            puVar8 = param_5;
                            mem_op_1000_179c(0x14, param_5, 0);
                            puStack22 = str_var1(puVar8, uVar6);
                            if((puVar8 | uVar6) != 0x0)
                            {
                                uVar13 = 0x4;
                                lVar10 = 0xa;
                                //goto LAB_1018_4230;
                            }
                        }
                    }
                }
            }
        }
    }
    iStack10 = 0x0;
    uStack8  = 0x0;
// LAB_1018_425e:
    if((iStack10 + 0x8) == 0x0)
    {
        (iStack10 + 0x8) = 0x1;
    }
    return;
}

pub fn get_sys_metrics_1018_2f56(param_1: u32)

{
    let mut uVar1: u16;
    let mut IVar2: u16;
    let mut IVar3: u16;
    let mut in_DX: *mut u8;
    let mut iVar4: i16;
    let mut unaff_DI: i16;
    let mut uVar5: u16;
    let mut ss_var1: u16;
    let mut puVar6: *mut u16;
    let mut uVar7: u32;
    let mut puVar8: *mut u16;
    let mut puVar9: *mut u16;
    let mut local_6: i16;
    let mut local_4: i16;

    puVar9 = str_var1(ss_var1, &local_4);
    puVar8 = str_var1(ss_var1, &local_6);
    puVar6 = mixed_1010_20ba(globals.data_1050_0ed0, 0x48, ss_var1, in_DX, unaff_DI);
    pass1_1008_3e94((puVar6 & 0xffff0000 | (puVar6 + 0xe)), puVar8, puVar9);
    uVar5          = (param_1 >> 0x10);
    iVar4          = param_1;
    uVar7          = pass1_1008_4772((iVar4 + 0x24));
    uVar1          = (uVar7 >> 0x10);
    (iVar4 + 0x18) = local_4 + 0xb5;
    (iVar4 + 0x1a) = local_6 + 0x9;
    IVar2          = GetSystemMetrics16(SEG_1008);
    (iVar4 + 0x1c) = IVar2 * 0x2 + (uVar7 + 0x4);
    IVar2          = GetSystemMetrics16(LAST_SEGMENT);
    IVar3          = GetSystemMetrics16(LAST_SEGMENT);
    (iVar4 + 0x1e) = IVar3 + IVar2 + (uVar7 + 0x8);
    return;
}

pub fn pass1_1018_30fc(param_1: u32, u16 **param_2, param_3: u8)

{
    let mut uVar1: u16;
    let mut u_var2: u32;
    let mut puVar3: *mut u16;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut lVar6 = 0i32;
    let mut puVar7: *mut u8;
    let mut dx_var1: u16;
    let mut uVar8: u16;
    let mut puStack18: *mut u32;
    let mut iStack6: i16;

    *param_2 = 0x0;
    uVar8    = (param_1 >> 0x10);
    u_var2    = (param_1 + 0x17e);
    uVar1    = (u_var2 + 0xa);
    if(uVar1 != 0x0)
    {
        u_var4 = uVar1;
        mem_op_1000_179c(0x6, param_3, 0);
        puStack18 = str_var1(param_3, u_var4);
        puVar7    = (param_3 | u_var4);
        if(puVar7 == 0x0)
        {
            *param_2 = 0x0;
        }
        else
        {
            *puStack18    = 0x0;
            (u_var4 + 0x4) = 0x0;
            *param_2      = puStack18;
        }
        uVar5 = uVar1 * 0x2;
        mem_op_1000_179c(uVar5, puVar7, 0);
        puVar3 = *param_2;
        *puVar3          = uVar5;
        (puVar3 + 0x2)   = puVar7;
        (*param_2 + 0x4) = uVar1;
        for(iStack6 = 0x0; iStack6 < uVar1; iStack6 = iStack6 + 0x1)
        {
            lVar6 = iStack6;
            empty_1008_8fc4((param_1 + 0x17e), lVar6);
            (*param_2 + iStack6 * 0x2) = (lVar6 + 0x2e);
        }
    }
    return;
}

pub fn pass1_1018_3710(param_1: *mut u32, param_2: u16, param_3: u16, param_4: u16)

{
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut iVar3: i16;
    let mut u_var4: u16;
    let mut puVar5: *mut u8;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut in_AF: u8;
    let mut lVar8 = 0i32;
    let mut puVar9: *mut u16;
    let mut local_12a: [u8;118] = [0;118];
    let mut uStack18: u32;
    let mut puStack14: *mut u16;
    let mut uStack10: u32;
    let mut pu_stack6: *mut u16;

    pu_stack6 = 0x0;
    uVar7    = (param_1 >> 0x10);
    iVar6    = param_1;
    uStack10 = switch_1018_3b9e(param_1, (iVar6 + 0x12e), param_3, param_4, param_2);
    u_var4    = (iVar6 + 0x12e) - 0x188;
    uStack18 = (uStack10 & 0xffff0000 | u_var4);
    switch(u_var4)
    {
    0x0 =>
        lVar8  = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (lVar8 >> 0x10);
            iVar3 = lVar8;
            mem_op_1000_179c(0x10, puVar5, 0);
        if(lVar8 != 0x0)
        {
            uStack18 = struct_1018_4790(lVar8, (iVar6 + 0x132), 0x0, (iVar3 + 0xe));
            pu_stack6 = uStack18;
            //goto switchD_1018_393f_caseD_6;
        }
        break;
    0x1 =>
        puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (puVar9 >> 0x10);
            iVar3 = puVar9;
            mem_op_1000_179c(0x14, puVar5, 0);
            u_var4 = (puVar9 >> 0x10) | puVar9;
        if(puVar9 != 0x0)
        {
            struct_1018_47c8(puVar9, *(iVar6 + 0x132), 0x0, (iVar3 + 0x12), *(iVar3 + 0xe));
            uStack18 = (puVar9 & 0xffff | u_var4 << 0x10);
            pu_stack6 = uStack18;
            //goto switchD_1018_393f_caseD_6;
        }
        break;
    2 =>
        puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (puVar9 >> 0x10);
            iVar3 = puVar9;
            mem_op_1000_179c(0x12, puVar5, 0);
            u_var4 = (puVar9 >> 0x10) | puVar9;
        if(puVar9 != 0x0)
        {
            pass1_1018_4808(puVar9, *(iVar6 + 0x132), 0x0, *(iVar3 + 0xe));
            uStack18 = (puVar9 & 0xffff | u_var4 << 0x10);
            pu_stack6 = uStack18;
            //goto switchD_1018_393f_caseD_6;
        }
        break;
     3 =>
        puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (puVar9 >> 0x10);
            iVar3 = puVar9;
            mem_op_1000_179c(0x14, puVar5, 0);
        if(puVar9 != 0x0)
        {
            uStack18 = struct_1018_4842(puVar9, *(iVar6 + 0x132), 0x0, (iVar3 + 0xe));
            pu_stack6 = uStack18;
            //goto switchD_1018_393f_caseD_6;
        }
        break;
    0x4 =>
        puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (puVar9 >> 0x10);
            iVar3 = puVar9;
            mem_op_1000_179c(0x10, puVar5, 0);
        if(puVar9 != 0x0)
        {
            uStack18 = struct_1018_48b0(puVar9, *(iVar6 + 0x132), 0x0, (iVar3 + 0xe));
            pu_stack6 = uStack18;
            //goto switchD_1018_393f_caseD_6;
        }
        break;
    0x5 =>
        puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
        puVar5 = (puVar9 >> 0x10);
            iVar3 = puVar9;
            mem_op_1000_179c(0x12, puVar5, 0);
        if(puVar9 != 0x0)
        {
            uStack18 = struct_1018_4920(puVar9, *(iVar6 + 0x132), 0x0, *(iVar3 + 0xe));
            pu_stack6 = uStack18;
        }
        break;
    _ =>
        //goto switchD_1018_393f_caseD_6;
    }
    uStack18 = 0x0;
    pu_stack6 = uStack18;
switchD_1018_393f_caseD_6:
    uVar1 = (iVar6 + 0x122);
    pass1_1008_e852(uVar1, (uVar1 >> 0x10), *(iVar6 + 0x126), param_2, (uStack18 >> 0x10));
    uVar1     = (iVar6 + 0x122);
    puStack14 = uStack18;
    pass1_1008_e852(uVar1, (uVar1 >> 0x10), *(iVar6 + 0x12a), param_2, (uStack18 >> 0x10));
    pass1_1038_2a0e(str_var1(param_2, local_12a), *(iVar6 + 0x136), pu_stack6, uStack18, puStack14, param_2, in_AF);
    fn_ptr_1030_835a(globals._PTR_LOOP_1050_5748, str_var1(param_2, local_12a));
    (iVar6 + 0x136) = 0x0;
    ppcVar2         = (*param_1 + 0x10);
    (**ppcVar2)(0x1030, param_1);
    pass1_1038_2a5c(str_var1(param_2, local_12a));
    return;
}

pub fn get_sys_metrics_1018_1ea0(param_1: *mut Struct55, param_2: u16)

{
    let mut IVar1: u16;
    let mut IVar2: u16;
    let mut iVar3: *mut Struct55;
    let mut uVar3: u16;

    IVar1             = GetSystemMetrics16(param_2);
    uVar3             = (param_1 >> 0x10);
    iVar3             = param_1;
    iVar3.field_0x2e = IVar1 * 0x2 + iVar3.field_0x36;
    IVar1             = GetSystemMetrics16(LAST_SEGMENT);
    IVar2             = GetSystemMetrics16(LAST_SEGMENT);
    iVar3.field_0x30 = IVar1 + iVar3.field_0x38 + IVar2;
    return;
}

u32 pass1_1018_1ff4(param_1: *mut Struct634, param_2: u16, param_3: u16)

{
    let mut pi_var1: *mut i16;
    let mut unaff_DI: i16;
    let mut ss_var1: u16;
    let mut p_var2: *mut Struct79;
    let mut piVar3: *mut i16;
    let mut piVar4: *mut i16;
    let mut uVar5: u16;
    let mut local_a: i16;
    let mut local_8: i16;
    let mut u_stack6: u32;

    p_var2                     = struct_op_1010_1d48(str_var1(param_2, param_1), param_3);
    &param_1.field_0xa        = 0xb9010b;
    param_1.field_0xe         = 0x170035;
    param_1 =  0x21e8;
    param_1.fld2_segment      = SEG_1018;
    piVar4                     = &local_8;
    piVar3                     = &local_a;
    uVar5                      = ss_var1;
    u_stack6                    = mixed_1010_20ba(globals.data_1050_0ed0, 0x48, ss_var1, (p_var2 >> 0x10), unaff_DI);
    pass1_1008_3e94((u_stack6 & 0xffff0000 | (u_stack6 + 0xe)),
                    str_var1(ss_var1, piVar3),
                    str_var1(uVar5, piVar4));
    pi_var1  = &param_1.field_0xa;
    *pi_var1 = *pi_var1 + local_8;
    pi_var1  = &param_1.field_0xc;
    *pi_var1 = *pi_var1 + local_a;
    pass1_1000_4906(str_var1(param_2, param_1 + 0x1), 0x0, 0x7f4);
    return param_1;
}

pub fn pass1_1018_270e(param_1: *mut Struct655, param_2: i16, param_3: u16, param_4: *mut u8, param_5: i16, param_6: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u32;
    let mut iVar3: i16;
    let mut u_var4: u16;
    let mut dx_var1: *mut u8;
    let mut iVar5: *mut Struct655;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;

    iVar5 = param_1;
    uVar5 = (param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        if((iVar5.field_0x20 == 0x0) || (u_var2 = iVar5.field_0x20, (u_var2 + 0x8) != param_3))
        {
            puVar6 = mixed_1010_20ba(globals.data_1050_0ed0, param_3, param_6, param_4, param_5);
            if(iVar5.field_0x20 != 0x0)
            {
                if(param_1 == 0x0)
                {
                    iVar3 = 0x0;
                    u_var4 = 0x0;
                }
                else
                {
                    iVar3 = &iVar5.field_0x1c_addr_base;
                    u_var4 = uVar5;
                }
                pass1_1010_1ea6(iVar5.field_0x20, str_var1(u_var4, iVar3), param_6);
            }
            iVar5.field_0x20 = puVar6;
            if(param_1 == 0x0)
            {
                param_3 = 0x0;
                u_var4   = 0x0;
            }
            else
            {
                param_3 = &iVar5.field_0x1c_addr_base;
                u_var4   = uVar5;
            }
            u_var2   = iVar5.field_0x20;
            ppcVar1 = (iVar5.field_0x20 + 0x4);
            (**ppcVar1)(SEG_1010, u_var2, (u_var2 >> 0x10), 0x0, param_3, u_var4);
            param_4 = dx_var1;
        }
        pass1_1018_2862(param_1);
        if((param_4 | param_3) != 0x0)
        {
            iVar5.field_0x24 = 0x1;
        }
        pass1_1010_1f62(param_6, param_1, 0x7);
    }
    else
    {
        if(((&iVar5.field_0x20 + 0x2) | &iVar5.field_0x20) != 0x0)
        {
            if(param_1 == 0x0)
            {
                iVar3 = 0x0;
                u_var4 = 0x0;
            }
            else
            {
                iVar3 = &iVar5.field_0x1c_addr_base;
                u_var4 = uVar5;
            }
            pass1_1010_1ea6(iVar5.field_0x20, str_var1(u_var4, iVar3), param_6);
            iVar5.field_0x20 = 0x0;
            return;
        }
    }
    return;
}

pub fn mixed_sys_op_1018_2978(param_1: u32, param_2: u16, param_3: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut pu_var2: *mut u8;
    let mut puVar3: *mut u8;
    let mut rect: *mut RECT16;
    let mut iVar4: i16;
    let mut in_DX: *mut u8;
    let mut uVar5: u16;
    let mut dx_var1: *mut u8;
    let mut puVar6: *mut u8;
    let mut puVar7: *mut u8;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut uVar12: u8;
    let mut paStack62: *mut Struct76;
    let mut local_3a: RECT16;
    let mut iStack54: i16;
    let mut iStack52: i16;
    let mut uStack50: u32;
    let mut puStack46: *mut u32;
    let mut local_2a: [u8;24] = [0;24];
    let mut u_stack6: u16;

    pass1_1010_8096globals.dat_1050_14cc, 0x1);
    pu_var2  = local_2a;
    u_stack6 = param_2;
    struct_op_1008_48fe(str_var1(param_3, pu_var2), 0x1, str_var1(in_DX, param_2), in_DX);
    uVar9 = SEG_1000;
    mem_op_1000_179c(0x1e, in_DX, 0);
    uVar5 = in_DX | pu_var2;
    if(uVar5 == 0x0)
    {
        puVar3 = 0x0;
        uVar5  = 0x0;
    }
    else
    {
        puVar3 = local_2a;
        uVar9  = SEG_1008;
        struct_op_1008_3f92(str_var1(in_DX, pu_var2), str_var1(param_3, puVar3));
    }
    puStack46 = str_var1(uVar5, puVar3);
    ppcVar1   = (*puStack46 + 0x14);
    (**ppcVar1)(uVar9, puVar3, uVar5);
    uStack50 = str_var1(dx_var1, puVar3);
    puVar6 = dx_var1;
    mem_op_1000_179c(0x14, dx_var1, 0);
    puVar7 = (puVar6 | puVar3);
    if(puVar7 == 0x0)
    {
        puVar3 = 0x0;
        puVar7 = 0x0;
    }
    else
    {
        struct_1008_4c58(CONCAT13((puVar6 >> 0x8), CONCAT12(puVar6, puVar3)));
    }
    uVar9          = (param_1 >> 0x10);
    iVar8          = param_1;
    (iVar8 + 0xe)  = puVar3;
    (iVar8 + 0x10) = puVar7;
    pass1_1008_4d84((iVar8 + 0xe), uStack50, puVar7);
    uVar12 = SUB21(globals.PTR_LOOP_1050_0396, 0x0);
    rect   = &local_3a;
    GetClientRect16(SEG_1008, rect);
    uVar11 = 0x1e;
    uVar10 = SEG_1000;
    mem_op_1000_179c(0x1e, puVar7, 0);
    paStack62 = str_var1(puVar7, rect);
    uVar5     = puVar7 | rect;
    if(uVar5 == 0x0)
    {
        (iVar8 + 0xa) = 0x0;
    }
    else
    {
        iVar4  = (iStack52 - local_3a.y) + 0x1;
        uVar10 = SEG_1008;
        pass1_1008_405c(paStack62, *(iVar8 + 0xe), iVar4, (iStack54 - local_3a.x) + 0x1);
        (iVar8 + 0xa) = iVar4;
        (iVar8 + 0xc) = uVar5;
    }
    if(puStack46 != 0x0)
    {
        ppcVar1 = *puStack46;
        (**ppcVar1)(uVar10, puStack46, (puStack46 >> 0x10), 0x1, uVar11, uVar12);
    }
    close_file_1008_496c(local_2a, param_3);
    return;
}

pub fn pass1_1018_10c4(param_1: u16, param_2: u16, param_3: u32)

{
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut puVar5: *mut u8;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut uVar9: u16;
    let mut puVar10: *mut u8;
    let mut dx_var1: u16;
    let mut dx_var1_00: u16;
    let mut dx_var1_01: u16;
    let mut dx_var1_02: u16;
    let mut iVar11: i16;
    let mut uVar12: u16;
    let mut uVar13: u8;
    let mut bVar14: bool;
    let mut puVar15: *mut u32;
    let mut u_stack60: u32;
    let mut uStack56: u32;
    let mut uStack52: u32;
    let mut puStack48: *mut u32;
    let mut puStack40: *mut u32;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut local_16: [u8;8] = [0;8];
    let mut uStack14: u16;
    let mut uStack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut iStack6: i16;
    let mut iStack4: i16;

    uVar12  = (param_3 >> 0x10);
    iVar11  = param_3;
    iStack4 = (iVar11 + 0x86);
    fn_ptr_1000_17ce((iVar11 + 0x88), SEG_1000);
    (iVar11 + 0x86) = 0x0;
    (iVar11 + 0x88) = 0x0;
    pass1_1028_dc52(CONCAT13((param_1 >> 0x8), CONCAT12(param_1, local_16)), 0x1, 0x0, 0x400);
    uStack30 = 0x0;
    uStack28 = 0x0;
    while(true)
    {
        uVar9  = param_2;
        puVar5 = local_16;
        pass1_1028_e4ec(str_var1(param_1, puVar5));
        param_2 = uVar9 | puVar5;
        if(param_2 == 0x0)
            break;
        if((iVar11 + 0x3c) == (puVar5 + 0x8))
        {
            puVar15 = pass1_1008_c6fa(globals.dat_1050_06e0, 0x2);
            puVar10 = (puVar15 >> 0x10);
            uVar6   = puVar15;
            pass1_1038_4e78(uVar6, puVar10, str_var1(uVar9, puVar5), puVar15);
            puStack48 = str_var1(puVar10, uVar6);
            uVar3     = *puStack48;
            ppcVar2   = uVar3 + 0x8;
            uVar9     = uVar6;
            (**ppcVar2)(SEG_1038, uVar6, puVar10);
            bVar14   = CARRY2(uStack30, uVar9);
            uStack30 = uStack30 + uVar9;
            uStack28 = uStack28 + dx_var1 + bVar14;
            param_2  = dx_var1;
            if(puStack48 != 0x0)
            {
                ppcVar2 = uVar3;
                (**ppcVar2)(0x38, uVar6, puVar10, 0x1);
                param_2 = dx_var1_00;
            }
        }
    }
    if((uStack28 | uStack30) != 0x0)
    {
        (iVar11 + 0x86) = uStack30;
        uVar7 = uStack30 * 0x6;
        mem_op_1000_179c(uVar7, 0x0, 0);
        uStack52 = str_var1(param_2, uVar7);
        if((param_2 | uVar7) == 0x0)
        {
            (iVar11 + 0x88) = 0x0;
        }
        else
        {
            pass1_1000_5586(0x3e38, SEG_1008, uStack30, 0x6, uVar7, param_2);
            (iVar11 + 0x88) = uStack52;
        }
        if(iStack6 != 0x0)
        {
            uStack10 = 0x1;
            uStack8  = 0x0;
        }
        iVar4    = 0x0;
        uStack14 = uStack10;
        uStack12 = uStack8;
        while(true)
        {
            uVar9  = uStack8;
            puVar5 = local_16;
            pass1_1028_e4ec(str_var1(param_1, puVar5));
            if((uVar9 | puVar5) == 0x0)
                break;
            uStack8 = uVar9 | puVar5;
            if((iVar11 + 0x3c) == (puVar5 + 0x8))
            {
                puVar15 = pass1_1008_c6fa(globals.dat_1050_06e0, 0x2);
                puVar10 = (puVar15 >> 0x10);
                uVar6   = puVar15;
                uVar13  = 0x38;
                pass1_1038_4e78(uVar6, puVar10, CONCAT13((uVar9 >> 0x8), CONCAT12(uVar9, puVar5)), puVar15);
                puStack40 = str_var1(puVar10, uVar6);
                ppcVar2   = (*puStack40 + 0x10);
                uVar9     = uVar6;
                (**ppcVar2)(0x38, uVar6, puVar10);
                uStack56 = str_var1(dx_var1_01, uVar9);
                uStack8  = dx_var1_01;
                for(u_stack60 = 0x0; u_stack60 < uStack56; u_stack60 = u_stack60 + 0x1)
                {
                    uVar8 = uStack56;
                    pass1_1030_1d58(puStack40);
                    uVar1  = *(iVar11 + 0x88);
                    uVar13 = 0x8;
                    pass1_1008_3f62((uVar1 & 0xff000000 | CONCAT12((uVar1 >> 0x10), uVar1 + iVar4 * 0x6)),
                                    str_var1(uStack8, uVar8 + 0xc));
                    iVar4 = iVar4 + 0x1;
                }
                if(puStack40 != 0x0)
                {
                    ppcVar2 = *puStack40;
                    (**ppcVar2)(uVar13, uVar6, puVar10, 0x1);
                    uStack8 = dx_var1_02;
                }
            }
        }
        if((iVar11 + 0x86) != iStack4)
        {
            pass1_1010_1f62(param_1, param_3, 0x6);
        }
    }
    return;
}

pub fn pass1_1018_1320(param_1: u32,param_2: *mut u16, u32 *param_3)

{
    let mut uVar1: u16;

    uVar1    = (param_1 >> 0x10);
    *param_3 = *(param_1 + 0x88);
    *param_2 = (param_1 + 0x86);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_1346(param_1: u16, param_2: u16, param_3: *mut Struct93)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut iVar2: i16;
    let mut puVar3: *mut u8;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut puVar6: *mut u8;
    let mut dx_var1: u16;
    let mut dx_var1_00: u16;
    let mut dx_var1_01: u16;
    let mut uVar7: u16;
    let mut dx_var1_02: u16;
    let mut iVar9: *mut Struct93;
    let mut uVar8: u16;
    let mut uVar9: u8;
    let mut puVar10: *mut u32;
    let mut uVar11: u32;
    let mut uVar12: u32;
    let mut uStack70: u32;
    let mut puStack56: *mut u32;
    let mut uStack52: u32;
    let mut puStack48: *mut u32;
    let mut uStack30: u32;
    let mut local_16: [u8;8] = [0;8];
    let mut uStack14: u16;
    let mut uStack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    uVar8   = (param_3 >> 0x10);
    iVar9   = param_3;
    uStack4 = iVar9.field_0x8c;
    fn_ptr_1000_17ce(iVar9.field_0x8e, SEG_1000);
    iVar9.field_0x8c = 0x0;
    iVar9.field_0x8e = 0x0;
    pass1_1028_dc52(CONCAT13((param_1 >> 0x8), CONCAT12(param_1, local_16)), 0x1, 0x0, 0x400);
    uStack30 = 0x0;
    while(true)
    {
        uVar7  = param_2;
        puVar3 = local_16;
        pass1_1028_e4ec(str_var1(param_1, puVar3));
        param_2 = uVar7 | puVar3;
        if(param_2 == 0x0)
            break;
        if(iVar9.field_0x3c == (puVar3 + 0x8))
        {
            puVar10 = pass1_1008_c6fa(globals.dat_1050_06e0, 0x2);
            puVar6  = (puVar10 >> 0x10);
            u_var4   = puVar10;
            uVar9   = 0x38;
            pass1_1038_4e78(u_var4, puVar6, str_var1(uVar7, puVar3), puVar10);
            puStack48 = str_var1(puVar6, u_var4);
            ppcVar1   = (*puStack48 + 0x10);
            uVar7     = u_var4;
            (**ppcVar1)(SEG_1038, u_var4, puVar6);
            uStack52 = str_var1(dx_var1, uVar7);
            param_2  = dx_var1;
            for(puStack56 = 0x0; puStack56 < uStack52; puStack56 = (puStack56 + 0x1))
            {
                uVar9   = 0x30;
                uVar11  = pass1_1030_1d7c(uVar7, param_2, puStack48);
                param_2 = (uVar11 >> 0x10);
                if((uVar11 + 0x12) == 0x9)
                {
                    uStack30 = uStack30 + 0x1;
                }
            }
            if(puStack48 != 0x0)
            {
                ppcVar1 = *puStack48;
                (**ppcVar1)(uVar9, u_var4, puVar6, 0x1);
                param_2 = dx_var1_00;
            }
        }
    }
    if((uStack30 | uStack30) != 0x0)
    {
        iVar9.field_0x8c = uStack30;
        uVar5 = uStack30 * 0x6;
        mem_op_1000_179c(uVar5, 0x0, 0);
        uStack70 = str_var1(param_2, uVar5);
        if((param_2 | uVar5) == 0x0)
        {
            iVar9.field_0x8e = 0x0;
        }
        else
        {
            pass1_1000_5586(0x3e38, SEG_1008, uStack30, 0x6, uVar5, param_2);
            iVar9.field_0x8e = uStack70;
        }
        if(iStack6 != 0x0)
        {
            uStack10 = 0x1;
            uStack8  = 0x0;
        }
        iVar2    = 0x0;
        uStack14 = uStack10;
        uStack12 = uStack8;
        while(true)
        {
            uVar7  = uStack8;
            puVar3 = local_16;
            pass1_1028_e4ec(str_var1(param_1, puVar3));
            if((uVar7 | puVar3) == 0x0)
                break;
            uStack8 = uVar7 | puVar3;
            if(iVar9.field_0x3c == (puVar3 + 0x8))
            {
                puVar10 = pass1_1008_c6fa(globals.dat_1050_06e0, 0x2);
                puVar6  = (puVar10 >> 0x10);
                u_var4   = puVar10;
                uVar9   = 0x38;
                pass1_1038_4e78(u_var4, puVar6, CONCAT13((uVar7 >> 0x8), CONCAT12(uVar7, puVar3)), puVar10);
                puStack56 = str_var1(puVar6, u_var4);
                ppcVar1   = (*puStack56 + 0x10);
                uVar7     = u_var4;
                (**ppcVar1)(0x38, u_var4, puVar6);
                uStack52 = str_var1(dx_var1_01, uVar7);
                uStack8  = dx_var1_01;
                for(puStack48 = 0x0; puStack48 < uStack52; puStack48 = (puStack48 + 0x1))
                {
                    uVar11 = uStack52;
                    pass1_1030_1d58(puStack56);
                    uVar9  = 0x30;
                    uVar12 = struct_op_1030_73a8(uVar11 & 0xffff | uStack8 << 0x10);
                    uVar7  = (uVar12 >> 0x10);
                    if((uVar12 + 0x12) == 0x9)
                    {
                        uVar12 = iVar9.field_0x8e;
                        uVar9  = 0x8;
                        pass1_1008_3f62((uVar12 & 0xff000000 | CONCAT12((uVar12 >> 0x10), uVar12 + iVar2 * 0x6)),
                          str_var1(uStack8, uVar11 + 0xc));
                        iVar2 = iVar2 + 0x1;
                    }
                    uStack8 = uVar7;
                }
                if(puStack56 != 0x0)
                {
                    ppcVar1 = *puStack56;
                    (**ppcVar1)(uVar9, u_var4, puVar6, 0x1);
                    uStack8 = dx_var1_02;
                }
            }
        }
        if(iVar9.field_0x8c != uStack4)
        {
            pass1_1010_1f62(param_1, param_3, 0x6);
        }
    }
    return;
}

pub fn pass1_1018_18b8(param_1: u16, param_2: *mut Struct55, param_3: u16)

{
    let mut puVar1: *mut u8;
    let mut iVar3: *mut Struct55;
    let mut unaff_DI: i16;
    let mut uVar3: *mut Struct55;
    let mut pu_var2: *mut u16;
    let mut paVar3: *mut Struct43;
    let mut u_var4: u32;
    let mut piVar5: *mut i16;
    let mut uVar6: u16;
    let mut piVar7: *mut i16;
    let mut uVar8: u16;
    let mut local_6: i16;
    let mut local_4: i16;
    let mut uVar1: u16;

    get_sys_metrics_1018_4b1e(param_2, 0x0, param_3);
    uVar3                 = (param_2 >> 0x10);
    iVar3                 = param_2;
    iVar3.field_0x20     = addr_table_1008_380a[36]; // 0x389a
    iVar3.field_0x22     = SEG_1008;
    iVar3.field_0x20     = addr_table_1008_3aa0[2];//0x3aa8;
    iVar3.field_0x22     = SEG_1008;
    &iVar3.field_0x24    = 0x0;
    iVar3.field_0x28     = 0x4;
    pu_var2                = pass1_1008_3e38((param_2 & 0xffff0000 | ZEXT24(iVar3 + 0x1)));
    puVar1                = (pu_var2 >> 0x10);
    &iVar3[0x1].field_0x6 = 0x0;
    iVar3[0x1].field_0xa  = 0x0;
    &iVar3[0x1].field_0xc = 0x0;
    iVar3[0x1].field_0x10 = 0x0;
    iVar3[0x1].field_0x1c_addr_base = 0x0;
    param_2.field_0x0    = addr_table_1018_1fb0;//0x1fb0;
    iVar3.field_0x2      = SEG_1018;
    iVar3.field_0x20     = addr_table_1018_1fb0[15];//0x1fec;
    iVar3.field_0x22     = SEG_1018;
    pass1_1000_4906((param_2 & 0xffff0000 | &iVar3[0x1].field_0x14), 0x0, 0x8);
    piVar7 = &local_4;
    piVar5 = &local_6;
    uVar6  = param_1;
    uVar8  = param_1;
    pu_var2 = mixed_1010_20ba(globals.data_1050_0ed0, 0x48, param_1, puVar1, unaff_DI);
    pass1_1008_3e94((pu_var2 & 0xffff0000 | (pu_var2 + 0xe)),
                    str_var1(uVar6, piVar5),
                    str_var1(uVar8, piVar7));
    paVar3             = unk_io_op_1010_830a(globals.dat_1050_14cc, 0x9a, param_1);
    iVar3.field_0x24  = paVar3;
    &iVar3.field_0x26 = (paVar3 >> 0x10);
    u_var4              = pass1_1008_4772((paVar3 & 0xffff0000 | iVar3.field_0x24));
    uVar1              = (u_var4 >> 0x10);
    pass1_1000_4906((param_2 & 0xffff0000 | &iVar3.field_0x32), 0x0, 0x8);
    iVar3.field_0x36 = (u_var4 + 0x4);
    iVar3.field_0x38 = (u_var4 + 0x8);
    iVar3.field_0x2a = local_4 + 0x14;
    iVar3.field_0x2c = local_6 + 0x14;
    get_sys_metrics_1018_1ea0(param_2, SEG_1000);
    pass1_1008_3e76((param_2 & 0xffff0000 | ZEXT24(iVar3 + 0x1)), 0x0, 0x88, 0x99);
    return;
}

pub fn pass1_1018_1a8e(param_1: *mut Struct653, param_2: *mut u8, param_3: i16, param_4: u16)

{
    let mut lVar1 = 0i32;
    let mut iVar2: *mut Struct653;
    let mut u_var2: u16;
    let mut puVar3: *mut u16;
    let mut piVar4: *mut i16;
    let mut local_8: i16;
    let mut u_stack6: u32;

    u_var2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if(iVar2.field_0x44 != 0x0)
    {
        if(iVar2.field_0x46 != 0x0)
        {
            lVar1             = iVar2.field_0x46;
            (lVar1 + 0xe)     = 0x0;
            iVar2.field_0x46 = 0x0;
        }
        piVar4  = &iVar2.field_0x4a;
        *piVar4 = *piVar4 + 0x1;
        return;
    }
    piVar4 = str_var1(param_4, &local_8);
    puVar3 = mixed_1010_20ba(globals.data_1050_0ed0, 0x3, param_4, param_2, param_3);
    pass1_1010_bf1e(puVar3, piVar4, puVar3, (puVar3 >> 0x10), param_4);
    iVar2.field_0x44 = local_8;
    iVar2.field_0x40 = u_stack6;
    pass1_1018_1ce8(param_4, param_1 & 0xffff | u_var2 << 0x10);
    return;
}

pub fn pass1_1010_e964(param_1: *mut u8, param_2: u16, i16 param_3)

{
    let mut uVar1: u32;
    let mut u_var2: u16;
    let mut puVar3: *mut u16;

    puVar3 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2f, param_2, param_1, param_3);
    u_var2  = (puVar3 >> 0x10);
    uVar1  = *(puVar3 + 0x24);
    pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    pass1_1038_4d28(uVar1 & 0xffff | u_var2 << 0x10);
    return;
}

pub fn struct_1010_e9e4(param_1: *mut Struct261, param_2: u16, param_3: u16)

{
    let mut puVar1: *mut u16;
    let mut u_var2: u16;
    let mut iVar3: i16;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar7: *mut u8;
    let mut iVar8: i16;
    let mut paVar9: *mut Struct79;
    let mut puVar10: *mut u16;
    let mut iStack4: i16;

    paVar9                     = struct_op_1010_1d48(str_var1(param_2, param_1), param_3);
    puVar7                     = (paVar9 >> 0x10);
    param_1.field_0xa         = addr_table_1008_380a[36]; // 0x389a
    param_1.field_0xc         = SEG_1008;
    param_1.field_0xa         = addr_table_1008_3aa0[2];//0x3aa8;
    param_1.field_0xc         = SEG_1008;
    uVar5                      = 0x0;
    &param_1.field_0xe        = 0x0;
    param_1.field_0x12        = 0x0;
    param_1.field_0x16        = 0x0;
    param_1.field_0x1a_addr_offset = 0x0;
    param_1.field_0x1e        = 0x0;
    param_1.field_0x20        = 0x0;
    param_1.field_0x24        = 0x0;
    param_1.field_0x28        = 0x0;
    param_1.field_0x2c        = 0x0;
    param_1.field_0x30        = 0x0;
    param_1.field_0x32        = 0x0;
    param_1 =  addr_table_1018_0558;//0x558;
    param_1.fld2_segment           = SEG_1018;
    param_1.field_0xa         = addr_table_1018_0558[4];//0x568;
    param_1.field_0xc = SEG_1018;
    mem_op_1000_179c(0x4, puVar7, 0);
    if((puVar7 | uVar5) == 0x0)
    {
        &param_1.field_0xe = 0x0;
    }
    else
    {
        puVar10             = pass1_1018_dcf6(str_var1(puVar7, uVar5));
        param_1.field_0xe  = puVar10;
        param_1.field_0x10 = (puVar10 >> 0x10);
    }
    pass1_1000_4906(str_var1(param_2, &param_1.field_0x34), 0x0, 0x24);
    param_1.field_0x38 = 0xfa;
    param_1.field_0x3c = 0x15e;
    uVar6               = 0x1c2;
    param_1.field_0x40 = 0x1c2;
    param_1.field_0x44 = 0x1c2;
    param_1.field_0x46 = 0x2260000;
    param_1.field_0x4a = 0x28a0000;
    param_1.field_0x4e = 0x730000;
    param_1.field_0x52 = 0x960000;
    param_1.hobject_field_0x56 = 0x0;
    for(iStack4 = 0x1; iStack4 < 0x9; iStack4 = iStack4 + 0x1)
    {
        pass1_1008_612e(0x0, 0x1d, uVar6);
        uVar5 = uVar6;
        pass1_1008_612e(0x1, 0x2, uVar5);
        if((uVar6 & 0x1) != 0x0)
        {
            uVar5 = -uVar5;
        }
        iVar8                          = iStack4 * 0x4;
        puVar1                         = (&param_1.field_0x34 + iVar8);
        u_var2                          = *puVar1;
        u_var4                          = uVar5 + *puVar1;
        uVar6                          = u_var4;
        iVar3                          = (&param_1.field_0x34 + iVar8 + 0x2);
        (&param_1.field_0x34 + iVar8) = u_var4;
        (&param_1.field_0x36 + iVar8) = (uVar5 >> 0xf) + iVar3 + CARRY2(uVar5, u_var2);
    }
    return;
}

pub fn pass1_1018_0196(param_1: u32, param_2: u32, param_3: u32, param_4: u16, param_5: *mut u8, param_6: u16)

{
    let mut pi_var1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u32;
    let mut u_var4: u32;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut lVar9 = 0i32;

    pass1_1030_8344(globals._PTR_LOOP_1050_5748, (globals._PTR_LOOP_1050_5748 >> 0x10), param_3);
    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    if((iVar7 + 0x2c) == 0x0)
    {
        (iVar7 + 0x32) = 0x5;
        if(globals._PTR_LOOP_1050_5f2c == 0x0)
        {
            globals.dat_1050_5f2c      = mem_op_1000_160a(param_5, SEG_1000);
            globals.dat_1050_5f2e      = param_5;
        }
        else
        {
        }
        uVar5 = fn_ptr_op_1000_1708(0x1e, 0x0, 0x1, globals.dat_1050_5f2c, globals.dat_1050_5f2e, SEG_1000);
    }
    else
    {
        uVar5                       = (iVar7 + 0x30) + 0x1;
        globals.dat_1050_5f2e      = param_5;
        if(uVar5 < (iVar7 + 0x32))
            //goto LAB_1018_022a;
        pi_var1                      = (iVar7 + 0x32);
        *pi_var1                     = *pi_var1 + 0x5;
        uVar3                       = (iVar7 + 0x2c);
        lVar9                       = pass1_1000_0ed4(SEG_1000, param_6, 0x1, (iVar7 + 0x32) * 0x6, 0x0, uVar3, (uVar3 >> 0x10));
        globals.dat_1050_5f2e      = (lVar9 >> 0x10);
        uVar5                       = lVar9;
    }
    (iVar7 + 0x2c) = uVar5;
    (iVar7 + 0x2e) = globals.dat_1050_5f2e;
// LAB_1018_022a:
    pass1_1030_8344(globals._PTR_LOOP_1050_5748, (globals._PTR_LOOP_1050_5748 >> 0x10), param_2);
    uVar6 = *(uVar5 + 0x10);
    pass1_1030_8344(globals._PTR_LOOP_1050_5748, (globals._PTR_LOOP_1050_5748 >> 0x10), uVar6);
    iVar2   = (iVar7 + 0x30);
    pi_var1  = (iVar7 + 0x30);
    *pi_var1 = *pi_var1 + 0x1;
    u_var4   = *(iVar7 + 0x2c);
    pass1_1008_3f62((u_var4 & 0xffff0000 | (u_var4 + iVar2 * 0x6)),
                    str_var1(globals.PTR_LOOP_1050_5f2e, uVar6 + 0xc));
    return;
}

pub fn pass1_1018_028c(param_1: u32, param_2: u32, param_3: u16, param_4: u16, param_5: u16)

{
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut puVar3: *mut u8;
    let mut u_var4: u16;
    let mut iVar5: i16;
    let mut uVar6: u32;
    let mut puVar7: *mut u8;
    let mut puVar8: *mut u8;
    let mut uVar9: u16;
    let mut dx_var1: u16;
    let mut uVar10: u16;
    let mut iVar11: i16;
    let mut unaff_DI: i16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut uVar15: u16;
    let mut iStack36: i16;
    let mut puStack28: *mut u32;
    let mut local_18: [u8;4] = [0;4];
    let mut uStack20: u16;
    let mut puStack12: *mut u16;
    let mut uStack8: u16;
    let mut u_stack6: u16;
    let mut puStack4: *mut u8;

    pass1_1030_8344(globals._PTR_LOOP_1050_5748, (globals._PTR_LOOP_1050_5748 >> 0x10), param_2);
    u_stack6   = param_3;
    puStack4  = param_4;
    uStack8   = pass1_1030_5b00(str_var1(param_4, param_3));
    puStack12 = mixed_1010_20ba(globals.data_1050_0ed0, uStack8, param_5, param_4, unaff_DI);
    pass1_1008_6c90(str_var1(param_5, local_18));
    pass1_1018_0b1e(puStack12, str_var1(param_5, local_18), param_5, 0, 0);
    puVar7 = (uStack20 >> 0xf);
    if((puVar7 | uStack20) == 0x0)
    {
        puVar3 = local_18;
        pass1_1030_6522(globals._PTR_LOOP_1050_5740, str_var1(param_5, puVar3), param_2, param_5);
    }
    else
    {
        puVar3 = local_18;
        pass1_1030_62e4(globals._PTR_LOOP_1050_5740, str_var1(param_5, puVar3), param_2, param_5);
    }
    puStack28 = str_var1(puVar7, puVar3);
    u_var4     = puVar7 | puVar3;
    if(u_var4 == 0x0)
    {
        return;
    }
    puVar8 = puVar7;
    pass1_1018_04f2(param_1);
    uVar14 = 0x1c;
    uVar13 = SEG_1000;
    mem_op_1000_179c(0x1c, puVar8, 0);
    uVar9 = puVar8 | u_var4;
    iVar11 = param_1;
    uVar12 = (param_1 >> 0x10);
    uVar15 = u_var4;
    if(uVar9 == 0x0)
    {
        (iVar11 + 0x12) = 0x0;
    }
    else
    {
        uVar13 = SEG_1008;
        struct_op_1008_8e9e(str_var1(puVar8, u_var4), 0x6, 0x24);
        (iVar11 + 0x12) = u_var4;
        (iVar11 + 0x14) = uVar9;
    }
    ppcVar2 = (*puStack28 + 0x10);
    (**ppcVar2)(uVar13, puVar3, puVar7, uVar14, uVar15);
    for(iStack36 = 0x0; iStack36 < u_var4; iStack36 = iStack36 + 0x1)
    {
        uVar6   = iStack36;
        ppcVar2 = (*puStack28 + 0x4);
        (**ppcVar2)();
        if((dx_var1 | uVar6) != 0x0)
        {
            iVar5  = iStack36 / 0x6;
            uVar10 = iStack36 % 0x6;
            uVar1  = (iVar11 + 0xe);
            pass1_1018_dd7c(uVar1, (uVar1 >> 0x10),
                            str_var1(iStack36 % 0x6, iVar5), uVar6 & 0xffff | dx_var1 << 0x10, uVar10, param_5);
            pass1_1008_8faa((iVar11 + 0x12), str_var1(uVar10, iVar5));
        }
    }
    return;
}

pub fn pass1_1018_0412(param_1: u32, param_2: u16, param_3: u32, param_4: u16, param_5: u32, param_6: u16, param_7: u8)

{
    let mut puVar1: *mut u8;
    let mut local_128: [u8;124] = [0;124];
    let mut uStack4: u16;

    uStack4 = 0x0;
    if(((0x72 < param_4) && (!SBORROW2(param_4, 0x73))) && ((param_4 == 0x75 || (param_4 - 0x74) < 0x1 || ((0x0 < (param_4 - 0x76) && ((param_4 - 0x77) < 0x2))))))
    {
        uStack4 = 0x1;
    }
    struct_op_1028_933c(str_var1(param_6, local_128), param_2, uStack4, param_4, param_3, (param_3 >> 0x10), *(param_1 + 0x24), param_5, param_6, param_7);
    puVar1 = local_128;
    fn_ptr_1030_835a(globals._PTR_LOOP_1050_5748, str_var1(param_6, puVar1));
    if(puVar1 != 0x0)
    {
        pass1_1010_1f62(param_6, param_1, 0x6);
    }
    return;
}

pub fn pass1_1018_04a4(param_1: u32, param_2: u32)

{
    *(param_1 + 0x16) = param_2;
    return;
}


u32 pass1_1018_04b8(param_1: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return str_var1((param_1 + 0x18), (param_1 + 0x16));
}


pub fn pass1_1018_04ca(param_1: u32, param_2: u32)

{
    *(param_1 + 0x1a) = param_2;
    return;
}


pub fn pass1_1018_04de(param_1: u32, param_2: u32)

{
    *(param_1 + 0x20) = param_2;
    return;
}

pub fn struct_1018_0570(param_1: *mut Struct55, param_2: u16, param_3: u16)

{
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut puVar3: *mut u16;
    let mut u_var4: u16;
    let mut puVar5: *mut u8;
    let mut uVar6: u16;
    let mut dx_var1: *mut u8;
    let mut unaff_DI: i16;
    let mut puVar7: *mut u16;
    let mut uVar8: u16;
    let mut uVar9: *mut Struct262;

    uVar9 = param_1;
    uVar8 = (param_1 >> 0x10);
    get_sys_metrics_1018_4b1e(param_1, 0x0, param_2);
    uVar9.field_0x20 = addr_table_1008_380a[36]; // 0x389a
    uVar9.field_0x22 = SEG_1008;
    uVar9.field_0x20 = addr_table_1008_3aa0[2];//0x3aa8;
    uVar9.field_0x22 = SEG_1008;
    uVar9.field_0x24 = 0x0;
    uVar9.field_0x2c = 0x0;
    pass1_1008_3e38((param_1 & 0xffff0000 | &uVar9.field_0x30));
    puVar7            = pass1_1008_3e38((param_1 & 0xffff0000 | &uVar9.field_0x36));
    puVar5            = (puVar7 >> 0x10);
    uVar9.field_0x3c = 0x0;
    pass1_1008_6c90((param_1 & 0xffff0000 | &uVar9.field_0x40));
    uVar6              = 0x0;
    uVar9.field_0x4c  = 0x0;
    uVar9.field_0x5a  = 0x0;
    uVar9.field_0x5e  = 0x0;
    uVar9.field_0x60  = 0x0;
    uVar9.field_0x64  = 0xff00;
    uVar9.field_0x66  = 0x0;
    uVar9.field_0x68  = 0x10000fb;
    uVar9.field_0x6c  = 0x10000f9;
    uVar9.field_0x70  = 0x10000ff;
    uVar9.field_0x74  = 0x10000fe;
    uVar9.field_0x78  = 0x10000fc;
    uVar9.field_0x7c  = 0x0;
    uVar9.field_0x80  = 0x0;
    uVar9.field_0x84  = 0x1;
    uVar9.field_0x86  = 0x0;
    uVar9.field_0x88  = 0x0;
    uVar9.field_0x8c  = 0x0;
    uVar9.field_0x8e  = 0x0;
    uVar9.field_0x92  = 0x0;
    uVar9.field_0x94  = 0x0;
    uVar9.field_0x98  = 0x0;
    uVar9.field_0x9a  = 0x0;
    &uVar9.field_0xa2 = 0x0;
    uVar9.field_0xa6  = 0xffff;
    uVar9.field_0xa8  = 0x0;
    param_1.field_0x0 = addr_table_1018_1874;//0x1874;
    uVar9->fld2_segment = SEG_1018;
    uVar9.field_0x20  = addr_table_1018_1874[15];//0x18b0;
    uVar9.field_0x22  = SEG_1018;
    if((globals.PTR_LOOP_1050_3960 == 0x0) && (globals._PTR_LOOP_1050_3962 == 0x0))
    {
        mem_op_1000_179c(0x8, puVar5, 0);
        globals._PTR_LOOP_1050_3962 = str_var1(puVar5, uVar6);
        pass1_1000_4906(str_var1(puVar5, uVar6), 0x0, 0x8);
    }
    globals.PTR_LOOP_1050_3960 = globals.PTR_LOOP_1050_3960 + 0x1;
    puVar7                      = mixed_1010_20ba(globals.data_1050_0ed0, 0x2f, param_3, puVar5, unaff_DI);
    &uVar9.field_0x2c          = puVar7;
    (&uVar9.field_0x2c + 0x2)  = (puVar7 >> 0x10);
    if(param_1 == 0x0)
    {
        puVar3 = 0x0;
        uVar6  = 0x0;
    }
    else
    {
        puVar3 = &uVar9.field_0x20;
        uVar6  = uVar8;
    }
    puVar1  = uVar9.field_0x2c;
    ppcVar2 = (*uVar9.field_0x2c + 0x4);
    (**ppcVar2)(SEG_1010, puVar1, (puVar1 >> 0x10), 0x0, puVar3, uVar6);
    puVar7 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2, param_3, dx_var1, unaff_DI);
    puVar5 = (puVar7 >> 0x10);
    if((puVar7 + 0x80) != 0x0)
    {
        uVar9.field_0x84 = 0x2;
    }
    puVar7            = mixed_1010_20ba(globals.data_1050_0ed0, 0x9, param_3, puVar5, unaff_DI);
    puVar5            = (puVar7 >> 0x10);
    uVar9.field_9e = puVar7;
    uVar9.field_0xa0 = puVar5;
    u_var4             = pass1_1010_65d0(param_3, puVar7 & 0xffff0000 | uVar9.field_9e, 0x88);
    if(u_var4 != 0x0)
    {
        uVar9.field_0xa8 = 0x1;
    }
    puVar7            = mixed_1010_20ba(globals.data_1050_0ed0, 0x3b, param_3, puVar5, unaff_DI);
    uVar9.field_0xa2 = puVar7;
    uVar9.field_0xa4 = (puVar7 >> 0x10);
    return;
}

pub fn get_sys_metrics_1018_09a8(param_1: u32, param_2: u16)

{
    let mut uVar1: u32;
    let mut IVar2: u16;
    let mut IVar3: u16;
    let mut in_DX: *mut u8;
    let mut iVar4: i16;
    let mut unaff_DI: i16;
    let mut uVar5: u16;
    let mut ss_var1: u16;
    let mut puVar6: *mut u16;
    let mut puVar7: *mut u16;
    let mut puVar8: *mut u16;
    let mut local_a: i16;
    let mut local_8: i16;
    let mut iStack6: i16;
    let mut IStack4: u16;

    IStack4 = GetSystemMetrics16(param_2);
    uVar5   = (param_1 >> 0x10);
    iVar4   = param_1;
    iStack6 = (iVar4 + 0x12) + -0x2;
    puVar8  = str_var1(ss_var1, &local_8);
    puVar7  = str_var1(ss_var1, &local_a);
    puVar6  = mixed_1010_20ba(globals.data_1050_0ed0, 0x48, ss_var1, in_DX, unaff_DI);
    pass1_1008_3e94((puVar6 & 0xffff0000 | (puVar6 + 0xe)), puVar7, puVar8);
    (iVar4 + 0x18) = iStack6 * IStack4 + local_8 + 0x146;
    (iVar4 + 0x1a) = iStack6 * IStack4 + local_a + 0x9;
    IVar2          = GetSystemMetrics16(SEG_1008);
    uVar1          = (iVar4 + 0x5a);
    (iVar4 + 0x1c) = IVar2 * 0x2 + (uVar1 + 0x4);
    IVar2          = GetSystemMetrics16(LAST_SEGMENT);
    IVar3          = GetSystemMetrics16(LAST_SEGMENT);
    uVar1          = (iVar4 + 0x5a);
    (iVar4 + 0x1e) = IVar3 + IVar2 + (uVar1 + 0x8);
    return;
}

pub fn pass1_1010_d448(param_1: *mut u8, param_2: u32,param_3: *mut u16, param_4: *mut u8, param_5: u8, i16 param_6)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u32;
    let mut u_var4: u32;
    let mut puVar5: *mut u16;
    let mut pcVar6: *mut c_char;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut iVar10: i16;
    let mut uVar11: u16;
    let mut uVar12: u32;
    let mut uVar13: u16;
    let mut local_40c: u16;
    let mut uStack1034: u32;
    let mut uStack1030: u32;
    let mut local_402: [u8;400] = [0;400];

    uVar11     = (param_3 >> 0x10);
    iVar10     = param_3;
    uStack1030 = struct_op_1030_73a8(*(iVar10 + 0x6));
    uVar8      = (uStack1030 >> 0x10);
    uVar1      = uStack1030;
    if((uVar8 | uVar1) != 0x0)
    {
        uStack1034 = *(uVar1 + 0x20);
        uVar1      = (uVar1 + 0x22);
        if((uVar1 | uStack1034) != 0x0)
        {
            local_40c = 0x0;
            puVar5    = &local_40c;
            uVar13    = (param_1 >> 0x10);
            pass1_1010_d984(param_1, uVar13,
                            str_var1(param_4, puVar5), 0x3, uStack1034 & 0xffff | uVar1 << 0x10, &PTR_DAT_1050_1805_1050_368e, param_3, param_4, param_5);
            pu_var2         = *(u16 **)(iVar10 + 0x2);
            uVar9          = (iVar10 + 0x4);
            (pu_var2 + 0x4) = globals.PTR_DAT_1050_1805_1050_368e;
            uVar3          = (iVar10 + 0x6);
            pcVar6         = pass1_1010_b038(param_1, uVar3, (uVar3 >> 0x10), (pu_var2 + 0x4), param_6);
            unk_str_op_1000_3d3e(str_var1(param_4, local_402), str_var1(uVar9, pcVar6));
            string_1040_a626(pu_var2, str_var1(param_4, local_402), uVar9);
            u_var4         = *(iVar10 + 0x2);
            uVar9         = (iVar10 + 0x4);
            iVar7         = u_var4;
            (iVar7 + 0xe) = globals.PTR_DAT_1050_1822_1050_3690;
            sys_1000_3f9c(local_402,
                          param_4,
                          0x3920,
                          local_40c,
                          &stack0xfffe,
                          uVar9,
                          SEG_1000,
                          param_4,
                          param_5);
            string_1040_a626((u_var4 & 0xffff0000 | (iVar7 + 0xa)), str_var1(param_4, local_402), uVar9);
            u_var4           = *(iVar10 + 0x2);
            uVar11          = (iVar10 + 0x4);
            iVar10          = u_var4;
            (iVar10 + 0x18) = globals.PTR_DAT_1050_1823_1050_3692;
            uVar12          = pass1_1028_62c8(uStack1030, param_4);
            uVar9           = (uVar12 >> 0x10);
            sys_1000_3f9c(local_402,
                          param_4,
                          0x3923,
                          uVar12,
                          &stack0xfffe,
                          uVar11,
                          SEG_1000,
                          param_4,
                          param_5);
            string_1040_a626((u_var4 & 0xffff0000 | (iVar10 + 0x14)),
                             str_var1(param_4, local_402), uVar9);
            pass1_1010_dc36(NULL, param_1, uVar13, puVar5, param_2, param_3, param_4);
        }
    }
    return;
}
