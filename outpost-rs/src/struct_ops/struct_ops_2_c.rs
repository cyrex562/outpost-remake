// #include "struct_ops_2.h"

// #include "address_tables/function_tables.h"
// #include "fn_ptr_ops/fn_ptr_ops_7.h"
// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "struct_20.h"
// #include "struct_382.h"
// #include "struct_ops_3.h"
// #include "structs/structs_0xx/structs_9x.h"
// #include "structs/structs_1xx/structs_12x.h"
// #include "structs/structs_6xx/struct_612.h"
// #include "sys_ops/sys_ops_12.h"
// #include "unk/unk_15.h"
// #include "unk/unk_19.h"

u16 * struct_1030_be34(param_1: *mut u16)

{
    struct_1028_b354(param_1);
    param_1.field_0x0 = addr_table_1030_c006;//0xc006;
    param_1.field_0x2 = SEG_1030;
    return param_1;
}


void  struct_1030_c06e(param_1: *mut Struct188)

{
    let mut iVar1: *mut Struct188;
    let mut uVar1: u16;

    struct_1028_b354(param_1);
    uVar1             = (param_1 >> 0x10);
    iVar1             = param_1;
    iVar1.field_0x20 = 0x0;
    iVar1.field_0x24 = 0x0;
    param_1.field_0x0 = addr_table_1030_c68e;//0xc68e;
    iVar1.field_0x2  = SEG_1030;
    return;
}


u16 * struct_1030_c6f6(param_1: *mut u16)

{
    let mut uVar1: u16;

    struct_1028_b354(param_1);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x20) = 0x0;
    param_1.field_0x0 = addr_table_1030_c940;//0xc940;
    param_1.field_0x2 = SEG_1030;
    return param_1;
}


u16 * struct_1030_c9a8(param_1: *mut u16)

{
    let mut iVar1: i16;
    let mut u_var2: u16;

    struct_1028_b354(param_1);
    u_var2          = (param_1 >> 0x10);
    iVar1          = param_1;
    (iVar1 + 0x98) = 0x1;
    param_1.field_0x0 = addr_table_1030_d88e;//0xd88e;
    (iVar1 + 0x2)  = SEG_1030;
    pass1_1000_4906((param_1 & 0xffff0000 | (iVar1 + 0x20)), 0x0, 0x78);
    return param_1;
}


BOOL16  pass1_1030_acbe(param_1: u16, param_2: u16,param_3: *mut u16, long param_4, param_5: u16, param_6: u16, param_7: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u16;
    let mut uVar3: u32;

    pass1_1030_627e(param_7, param_5, param_6, globals._PTR_LOOP_1050_5740, param_3, param_4);
    u_var2 = param_6 | param_5;
    if(u_var2 != 0x0)
    {
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, param_5, param_6);
        if((u_var2 | param_5) != 0x0)
        {
            uVar3 = struct_op_1030_73a8(str_var1(u_var2, param_5));
            if((uVar3 != 0x0) && ((iVar1 = (uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9))))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}


void  pass1_1030_afa6(param_1: *mut Struct614)

{
    let mut puVar1: *mut u32;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut u_var4: u32;
    let mut iVar5: *mut Struct614;
    let mut uVar5: u16;

    uVar5            = (param_1 >> 0x10);
    iVar5            = param_1;
    param_1.field_0x0 = addr_table_1030_b932;//0xb932;
    iVar5->fld2_segment = SEG_1030;
    if(&iVar5.field_0x10 != 0x0)
    {
        u_var4 = &iVar5.field_0x10;
        (u_var4 + 0xa) = 0x1;
    }
    puVar1 = &iVar5.field_0x10;
    u_var2 = iVar5.field_0x12;
    if ((u_var2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a
    iVar5->fld2_segment = SEG_1008;
    return;
}


void  pass1_1030_affc(param_1: u32, param_2: i16, param_3: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut BVar4: BOOL16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut iStack12: i16;
    let mut uStack10: u32;
    let mut local_6: u32;

    uVar8 = ZEXT24(&local_6);
    pass1_1030_b718(param_1, param_1, (param_1 & 0xffff0000 | (param_1 + 0x8)),
                    str_var1(param_3, &local_6), param_1, param_2, param_3);
    pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, local_6, (local_6 >> 0x10));
    uStack10 = uVar8 & 0xffff | param_1 << 0x10;
    uVar5    = param_1 | uVar8;
    if(uVar5 != 0x0)
    {
        uVar7 = struct_op_1030_73a8(uVar8 & 0xffff | param_1 << 0x10);
        uVar5 = (uVar7 >> 0x10);
        iVar1 = (uVar7 + 0xc);
        uVar8 = (iVar1 - 0x16U);
        if((0x15 < iVar1) && (!SBORROW2(iVar1, 0x16)))
        {
            uVar8 = (iVar1 - 0x17U);
            if(iVar1 - 0x17U != 0x0 && 0x0 < (iVar1 - 0x16U))
            {
                uVar8 = (iVar1 - 0x19U);
                if((iVar1 + -0x18 < 0x1) || (uVar8 = (iVar1 - 0x1aU), iVar1 - 0x1aU != 0x0 && 0x0 < (iVar1 - 0x19U)))
                    //goto LAB_1030_b064;
            }
            (uVar7 + 0x20) = 0x0;
        }
    }
// LAB_1030_b064:
    iStack12 = 0x6;
    do
    {
        uVar3 = uVar8;
        if(iStack12 == 0x0)
        {
        // LAB_1030_b0fc:
            if((uStack10 | uStack10) != 0x0)
            {
                uVar8 = struct_op_1030_73a8(uStack10);
                u_var2 = (uVar8 >> 0x10);
                iVar1 = (uVar8 + 0xc);
                if(((0x15 < iVar1) && (!SBORROW2(iVar1, 0x16))) && ((iVar1 == 0x17 || iVar1 + -0x16 < 0x1 || ((0x0 < iVar1 + -0x18 && (iVar1 + -0x19 < 0x2))))))
                {
                    (uVar8 + 0x20) = 0x1;
                }
            }
            return;
        }
        pass1_1030_b578(param_1, param_2, param_3);
        if((uVar5 | uVar3) == 0x0)
            //goto LAB_1030_b0fc;
        uStack10 = str_var1(uVar5, uVar3);
        uVar8    = struct_op_1030_73a8(str_var1(uVar5, uVar3));
        uVar6    = (uVar8 >> 0x10);
        iVar1    = (uVar8 + 0xc);
        pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x8)),
                        str_var1(uVar5, uVar3 + 0xc));
        if((iVar1 == 0x18) || (uVar5 = uVar6, iVar1 == 0x3f))
        {
            pass1_1030_b142(param_1, uStack10);
            uVar5 = uVar6;
        }
        BVar4 = pass1_1008_c6ae(globals.dat_1050_06e0, iVar1, 0x40);
        uVar8 = BVar4;
        if(BVar4 != 0x0)
        {
            pass1_1030_b454(param_1, uStack10, param_3);
            //goto LAB_1030_b0fc;
        }
        iStack12 = iStack12 + -0x1;
    } while(true);
}


void  pass1_1030_b2aa(param_1: u32,param_2: *mut u16, param_3: *mut u8, param_4: i16, param_5: u16)

{
    let mut uVar1: u16;
    let mut BVar2: BOOL16;
    let mut uVar3: u32;
    let mut bStack23: u8;
    let mut local_6: u32;

    pass1_1030_b718(param_1, (param_1 >> 0x10), param_2,
                    str_var1(param_5, &local_6), param_3, param_4, param_5);
    bStack23 = (local_6 >> 0x18);
    uVar1    = bStack23;
    if(bStack23 != 0x0)
    {
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, local_6, local_6);
        if((local_6 | uVar1) != 0x0)
        {
            uVar3 = struct_op_1030_73a8(str_var1(local_6, uVar1));
            BVar2 = pass1_1008_c6ae(globals.dat_1050_06e0, (uVar3 + 0xc), 0x42);
            if(BVar2 != 0x0)
            {
                pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x8)),
                                str_var1(local_6, uVar1 + 0xc));
                return;
            }
        }
    }
    return;
}


u32  pass1_1030_b344(param_1: u32, param_2: u16)

{
    let mut puVar1: *mut u8;
    let mut puStack18: *mut u32;
    let mut puStack16: *mut u8;
    let mut local_e: [u8;2] = [0;2];
    let mut local_c: i16;
    let mut local_a: i16;
    let mut local_8: u32;
    let mut uStack4: u16;

    local_8 = *(param_1 + 0x8);
    uStack4 = (param_1 + 0xc);
    puVar1  = param_1;
    pass1_1008_3eb4(str_var1(param_2, &local_8),
                    str_var1(param_2, local_e),
                    str_var1(param_2, &local_c),
                    str_var1(param_2, &local_a));
    local_8   = local_8 & 0xffff | (local_c - 0x1) << 0x10;
    puStack18 = &local_8;
    pass1_1030_b2aa(param_1, str_var1(param_2, puStack18), puVar1, &stack0xfffe, param_2);
    puStack16 = (puVar1 | puStack18);
    if(puStack16 == 0x0)
    {
        local_8   = local_8 & 0xffff | (local_c + 0x1) << 0x10;
        puStack18 = &local_8;
        pass1_1030_b2aa(param_1, str_var1(param_2, puStack18), 0x0, &stack0xfffe, param_2);
        puVar1 = (puStack16 | puStack18);
        if(puVar1 == 0x0)
        {
            local_8 = local_a + -0x1;
            local_8 = local_c;
            puStack18     = &local_8;
            pass1_1030_b2aa(param_1, str_var1(param_2, puStack18), 0x0, &stack0xfffe, param_2);
            puStack16 = (puVar1 | puStack18);
            if(puStack16 == 0x0)
            {
                local_8   = str_var1(local_8, local_a + 0x1);
                puStack18 = &local_8;
                pass1_1030_b2aa(param_1, str_var1(param_2, puStack18), 0x0, &stack0xfffe, param_2);
                if((puStack16 | puStack18) == 0x0)
                {
                    return 0x0;
                }
                (param_1 + 0xe) = 0x2;
            }
            else
            {
                (param_1 + 0xe) = 0x4;
                puStack16       = puVar1;
            }
        }
        else
        {
            (param_1 + 0xe) = 0x3;
        }
    }
    else
    {
        (param_1 + 0xe) = 0x1;
        puStack16       = puVar1;
    }
    return str_var1(puStack16, puStack18);
}


void  pass1_1030_b454(param_1: u32, param_2: u32, param_3: u16)

{
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut puVar3: *mut u8;
    let mut dx_var1: u16;
    let mut iVar4: i16;
    let mut dx_var1_00: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut uVar9: u32;
    let mut lStack38 = 0i32;
    let mut uStack30: u32;
    let mut local_12: [u8;4] = [0;4];
    let mut uStack14: u32;
    let mut uStack10: u32;
    let mut lStack6 = 0i32;

    lStack6 = (param_2 + 0x4);
    uVar7   = (param_1 >> 0x10);
    iVar6   = param_1;
    pass1_1008_5784(str_var1(param_3, local_12), *(iVar6 + 0x10));
    while(true)
    {
        puVar3 = local_12;
        pass1_1008_5b12(puVar3, param_3);
        uStack10 = str_var1(dx_var1, puVar3);
        if((dx_var1 | puVar3) == 0x0)
            break;
        if((puVar3 + 0x20) == lStack6)
        {
            ppcVar2 = ((iVar6 + 0x10) + 0xc);
            (**ppcVar2)();
            uStack14 = 0x0;
            pass1_1038_69fe(uStack10);
        }
    }
    uVar8  = struct_op_1030_73a8(param_2);
    iVar4  = (uVar8 >> 0x10);
    puVar1 = *(uVar8 + 0x20);
    puVar3 = local_12;
    pass1_1008_5784(str_var1(param_3, puVar3), puVar1);
    pass1_1030_b13c();
    uStack30 = str_var1(-((s_Unsupported_FileStructType_in_Op_1050_01ca + 0x2aU) < puVar3) - iVar4, 0x1f4 - puVar3);
    do
    {
        puVar3 = local_12;
        pass1_1008_5b12(puVar3, param_3);
        uStack10 = str_var1(dx_var1_00, puVar3);
        uVar5    = dx_var1_00 | puVar3;
        if(uVar5 == 0x0)
        {
            return;
        }
        pass1_1038_6984(str_var1(dx_var1_00, puVar3));
        lStack38 = str_var1(uVar5, puVar3);
        if((uVar5 <= uStack30) && ((uVar5 < uStack30 || (puVar3 <= uStack30))))
        {
            uVar9   = (iVar6 + 0x10);
            ppcVar2 = ((iVar6 + 0x10) + 0x8);
            (**ppcVar2)();
            uStack30 = uStack30 - lStack38;
            ppcVar2  = (*puVar1 + 0xc);
            (**ppcVar2)(SEG_1038, puVar1, (puVar1 >> 0x10), uStack10, uVar9);
            uStack14 = 0x0;
        }
    } while(0x0 < uStack30);
    return;
}


void  pass1_1030_b578(param_1: u32, param_2: i16, param_3: u16)

{
    let mut iVar1: i16;
    let mut pu_var2: *mut u32;
    let mut uVar3: u16;
    let mut pu_var4: *mut u8;
    let mut bVar5: bool;
    let mut uVar6: u32;
    let mut uStack48: u32;
    let mut local_1c: [u8;2] = [0;2];
    let mut local_1a: i16;
    let mut local_18: i16;
    let mut local_16: u32;
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut uStack14: u32;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut local_6: u32;

    pass1_1030_b718(param_1, param_1, (param_1 & 0xffff0000 | (param_1 + 0x8)),
                    str_var1(param_3, &local_6), param_1, param_2, param_3);
    uStack48._3_1_ = (local_6 >> 0x18);
    uStack10       = uStack48._3_1_;
    if(uStack48._3_1_ == 0x0)
    {
        return;
    }
    pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, local_6, local_6);
    uStack8  = local_6;
    uStack14 = struct_op_1030_73a8(str_var1(local_6, uStack10));
    uStack16 = (uStack14 + 0xc);
    local_16 = *(param_1 + 0x8);
    uStack18 = (param_1 + 0xc);
    pu_var4   = param_1;
    pass1_1008_3eb4(str_var1(param_3, &local_16),
                    str_var1(param_3, local_1c),
                    str_var1(param_3, &local_1a),
                    str_var1(param_3, &local_18));
    iVar1 = (param_1 + 0xe);
    if(iVar1 == 0x0)
    {
        pass1_1030_b344(param_1 & 0xffff | ZEXT24(param_1) << 0x10, param_3);
        return;
    }
    if(iVar1 == 0x1)
    {
        uVar3 = local_1a - 0x1;
    // LAB_1030_b63e:
        local_16 = local_16 & 0xffff | uVar3 << 0x10;
        pu_var2   = &local_16;
        pass1_1030_b2aa(param_1 & 0xffff | ZEXT24(param_1) << 0x10,
                        str_var1(param_3, pu_var2), pu_var4, &uStack16, param_3);
        uStack48 = str_var1(pu_var4, pu_var2);
        if((pu_var4 | pu_var2) == 0x0)
        {
            return;
        }
        uVar6 = struct_op_1030_73a8(str_var1(pu_var4, pu_var2));
        uVar3 = (uVar6 + 0xc);
        if(uVar3 == 0x3f)
            //goto LAB_1030_b6e0;
        if(0x3f < uVar3)
        {
            return;
        }
        if(uVar3 == '\x16')
            //goto LAB_1030_b6e0;
        bVar5 = uVar3 == '\x18';
    }
    else
    {
        if(iVar1 == 0x2)
        {
            uVar3 = local_18 + 0x1;
        }
        else
        {
            if(iVar1 == 0x3)
            {
                uVar3 = local_1a + 0x1;
                //goto LAB_1030_b63e;
            }
            if(iVar1 != 0x4)
            {
                return;
            }
            uVar3 = local_18 - 0x1;
        }
        local_16 = local_16 & 0xffff0000 | uVar3;
        pu_var2   = &local_16;
        pass1_1030_b2aa(param_1 & 0xffff | ZEXT24(param_1) << 0x10,
                        str_var1(param_3, pu_var2), pu_var4, &uStack16, param_3);
        uStack48 = str_var1(pu_var4, pu_var2);
        if((pu_var4 | pu_var2) == 0x0)
        {
            return;
        }
        uVar6 = struct_op_1030_73a8(str_var1(pu_var4, pu_var2));
        iVar1 = (uVar6 + 0xc);
        if(iVar1 < 0x17)
        {
            return;
        }
        if(SBORROW2(iVar1, 0x17))
        {
            return;
        }
        if(iVar1 == 0x18 || iVar1 + -0x17 < 0x1)
            //goto LAB_1030_b6e0;
        bVar5 = iVar1 == 0x3f;
    }
    if(!bVar5)
    {
        return;
    }
// LAB_1030_b6e0:
    pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x8)), (uStack48 & 0xffff0000 | (uStack48 + 0xc)));
}


void  pass1_1030_b936(param_1: *mut Struct365, param_2: u16, param_3: u16, param_4: u32, param_5: u16)

{
    pass1_1028_b22c(str_var1(param_2, param_1), param_3, param_4, param_5);
    param_1.field_0xe         = 0x0;
    param_1.field_0x12        = 0x0;
    param_1 =  addr_table_1030_bc0c;//0xbc0c;
    param_1.field_0x2         = SEG_1030;
}


void  pass1_1030_9adc(param_1: u16, param_2: u16, param_3: *mut u32, param_4: u32, param_5: u16, param_6: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut p_var2: *mut Struct99;
    let mut u_var4: u16;
    let mut dx_var1: u16;
    let mut dx_var1_00: u16;
    let mut iVar7: *mut Struct121;
    let mut iVar6: *mut Struct119;
    let mut paStack6: *mut Struct99;
    let mut uVar3: *mut Struct120;

    pass1_1038_53ba(param_4, 0x1);
    u_var4 = param_6 | param_5;
    if(u_var4 != 0x0)
    {
        paStack6 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
        u_var4    = (paStack6 >> 0x10);
        p_var2   = (paStack6 & 0xffff);
        if((u_var4 | p_var2) == 0x0)
        {
            paStack6 = 0x0;
        }
        else
        {
            iVar7               = paStack6;
            paStack6.field_0x0 = addr_table_1008_380a[36]; // 0x389a
            iVar7.field_0x2    = SEG_1008;
            iVar7.field_0x4    = 0x77;
            paStack6.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
            iVar7.field_0x2    = SEG_1030;
            p_var2              = paStack6;
        }
        param_5 = p_var2;
        ppcVar1 = (*param_3 + 0x4);
        (**ppcVar1)(SEG_1000, param_3, paStack6, (paStack6 >> 0x10));
        u_var4 = dx_var1;
    }
    pass1_1038_53ba(param_4, 0x2);
    u_var4 = u_var4 | param_5;
    if(u_var4 != 0x0)
    {
        paStack6 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
        u_var4    = (paStack6 >> 0x10);
        p_var2   = (paStack6 & 0xffff);
        if((u_var4 | p_var2) == 0x0)
        {
            paStack6 = 0x0;
        }
        else
        {
            iVar6               = paStack6;
            paStack6.field_0x0 = addr_table_1008_380a[36]; // 0x389a
            iVar6.field_0x2    = SEG_1008;
            iVar6.field_0x4    = 0x78;
            paStack6.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
            iVar6.field_0x2    = SEG_1030;
            p_var2              = paStack6;
        }
        param_5 = p_var2;
        ppcVar1 = (*param_3 + 0x8);
        (**ppcVar1)(SEG_1000, param_3, paStack6, (paStack6 >> 0x10));
        u_var4 = dx_var1_00;
    }
    pass1_1038_53ba(param_4, 0x3);
    if((u_var4 | param_5) != 0x0)
    {
        paStack6 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
        u_var4    = (paStack6 >> 0x10);
        uVar3    = paStack6;
        if((u_var4 | uVar3) == 0x0)
        {
            paStack6 = 0x0;
        }
        else
        {
            paStack6.field_0x0 = addr_table_1008_380a[36]; // 0x389a
            uVar3.field_0x2    = SEG_1008;
            uVar3.field_0x4    = 0x75;
            paStack6.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
            uVar3.field_0x2    = SEG_1030;
        }
        ppcVar1 = (*param_3 + 0x8);
        (**ppcVar1)(SEG_1000, param_3, paStack6, (paStack6 >> 0x10));
    }
    return;
}


u16  pass1_1030_9ef2(u32 *param_1)

{
    let mut iVar1: i16;
    let mut u_var2: u16;
    let mut uVar3: u32;

    if(*param_1 != 0x0)
    {
        uVar3 = struct_op_1030_73a8(*param_1);
        u_var2 = (uVar3 >> 0x10);
        iVar1 = (uVar3 + 0xc);
        if(((iVar1 != 0x5) && (iVar1 != 0x9)) && ((uVar3 + 0x12) < 0x5))
        {
            return 0x0;
        }
        pass1_1030_9f64(param_1);
    }
    return 0x1;
}


BOOL16  pass1_1030_8fe4(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16,param_6: *mut u16, long param_7)

{
    let mut iVar1: i16;
    let mut u_var2: u16;
    let mut uVar3: u32;

    pass1_1030_627e(param_1, param_2, param_3, globals._PTR_LOOP_1050_5740, param_6, param_7);
    u_var2 = param_3 | param_2;
    if(u_var2 != 0x0)
    {
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, param_2, param_3);
        if((u_var2 | param_2) != 0x0)
        {
            uVar3 = struct_op_1030_73a8(str_var1(u_var2, param_2));
            if((uVar3 != 0x0) && ((iVar1 = (uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9))))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}


void  pass1_1030_9048(param_1: u16, param_2: u32, param_3: i16, param_4: u32)

{
    let mut iVar1: i16;
    let mut u_var2: u32;
    let mut ppcVar3: *mut *mut c_void;
    let mut BVar4: BOOL16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar7: *mut u8;
    let mut dx_var1: u16;
    let mut dx_var1_00: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut puVar10: *mut u32;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut puVar13: *mut u32;
    let mut uVar14: u32;
    let mut uVar15: u16;
    let mut uVar16: u8;
    let mut uStack36: u32;
    let mut local_18: [u8;2] = [0;2];
    let mut local_16: i16;
    let mut local_14: i16;
    let mut local_12: i16;
    let mut iStack16: i16;
    let mut uStack12: u32;
    let mut uStack8: u16;
    let mut pu_stack6: *mut u8;
    let mut iStack4: i16;

    iStack4  = 0x8 - (param_3 == 0x0);
    puVar13  = pass1_1008_c6fa(globals.dat_1050_06e0, iStack4);
    puVar7   = (puVar13 >> 0x10);
    uVar8    = puVar13;
    uStack8  = uVar8;
    pu_stack6 = puVar7;
    pass1_1038_4e78(uVar8, puVar7, param_4, puVar13);
    uStack12 = str_var1(puVar7, uVar8);
    uVar12   = SEG_1008;
    pass1_1008_3e38(str_var1(param_1, &local_12));
    u_var2   = *(param_4 + 0x8);
    uVar11  = (uStack12 >> 0x10);
    uVar9   = SUB42(uStack12, 0x0);
    ppcVar3 = (*uStack12 + 0x10);
    uVar6   = u_var2;
    (**ppcVar3)(SEG_1008, uVar9, uVar11);
    uVar6    = uVar6 & 0xffff | dx_var1 << 0x10;
    uStack36 = 0x0;
    while(true)
    {
        if(uVar6 <= uStack36)
        {
            if(uStack12 != 0x0)
            {
                ppcVar3 = *uStack12;
                (**ppcVar3)(uVar12, uStack12, (uStack12 >> 0x10), 0x1, uVar9, uVar11, uStack12, uStack12);
            }
            return;
        }
        ppcVar3 = (*uStack12 + 0x4);
        uVar14  = uVar6;
        (**ppcVar3)();
        uVar5 = uVar14;
        uVar8 = dx_var1_00;
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, uVar5, dx_var1_00);
        pass1_1008_3f62(str_var1(param_1, &local_12), str_var1(uVar8, uVar5 + 0xc));
        uVar12 = SEG_1008;
        pass1_1008_3eb4(str_var1(param_1, &local_12),
                        str_var1(param_1, local_18),
                        str_var1(param_1, &local_16),
                        str_var1(param_1, &local_14));
        uVar14 = struct_op_1030_73a8(str_var1(uVar8, uVar5));
        uVar8  = (uVar14 >> 0x10);
        iVar1  = (uVar14 + 0xc);
        if(iVar1 - 0x7aU < 0x6)
            break;
    // LAB_1030_91fa:
        uStack36 = uStack36 + 0x1;
    }
    uVar12 = SEG_1030;
    uVar5  = param_2;
    uVar15 = (param_2 >> 0x10);
    switch(iVar1)
    {
    _ =>
        iStack16 = local_16 + -0x1;
        BVar4    = pass1_1030_8fe4(param_1, &local_12, uVar8, uVar5, uVar15, str_var1(param_1, &local_12), u_var2);
        if(BVar4 != 0x0)
            //goto LAB_1030_91cb;
        iStack16 = local_16 + 0x1;
        BVar4    = pass1_1030_8fe4(param_1, &local_12, uVar8, uVar5, uVar15, str_var1(param_1, &local_12), u_var2);
        if(BVar4 == 0x0)
        {
            iStack16 = local_16;
            local_12 = local_14 + -0x1;
            BVar4    = pass1_1030_8fe4(param_1, &local_12, uVar8, uVar5, uVar15,
                                    str_var1(param_1, &local_12), u_var2);
            //goto joined_r0x1030911e;
        }
    // LAB_1030_9144:
        break;
    0x7b =>
    0x7e =>
        iStack16 = local_16 + -0x1;
        BVar4    = pass1_1030_8fe4(param_1, &local_12, uVar8, uVar5, uVar15, str_var1(param_1, &local_12), u_var2);
        if(BVar4 == 0x0)
        {
            iStack16 = local_16 + 0x1;
            //goto LAB_1030_912c;
        }
        if(uStack12 == 0x0)
        {
            return;
        }
        uVar12  = (uStack12 >> 0x10);
        puVar10 = uStack12;
        uVar16  = (uStack12 >> 0x10);
        //goto LAB_1030_90e6;
    0x7c =>
    0x7d =>
        local_12 = local_14 + -0x1;
        BVar4    = pass1_1030_8fe4(param_1, &local_12, uVar8, uVar5, uVar15, str_var1(param_1, &local_12), u_var2);
    joined_r0x1030911e:
        if(BVar4 == 0x0)
        {
            local_12 = local_14 + 0x1;
        // LAB_1030_912c:
            BVar4 = pass1_1030_8fe4(param_1, &local_12, uVar8, uVar5, uVar15,
                                    str_var1(param_1, &local_12), u_var2);
            if(BVar4 != 0x0)
                //goto LAB_1030_9144;
            //goto LAB_1030_91fa;
        }
    // LAB_1030_91cb:
    }
    puVar10 = uStack12;
    if((uStack12 | puVar10) != 0x0)
    {
        uVar12 = (uStack12 >> 0x10);
        uVar16 = (uStack12 >> 0x10);
    // LAB_1030_90e6:
        ppcVar3 = *puVar10;
        (**ppcVar3)(0x1030, puVar10, uVar16, 0x1, uVar9, uVar11, uStack12, uStack12);
    }
    return;
}


void  pass1_1030_9296(param_1: u32, param_2: *mut u32, param_3: u32, param_4: u16, param_5: u16, param_6: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u16;
    let mut puVar3: *mut u8;
    let mut in_register_00000002: u16;
    let mut paVar4: *mut Struct99;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut dx_var1: u16;
    let mut dx_var1_00: u16;
    let mut puVar8: *mut u8;
    let mut dx_var1_01: *mut u8;
    let mut dx_var1_02: *mut u8;
    let mut dx_var1_03: u16;
    let mut uVar9: u16;
    let mut iVar11: *mut Struct116;
    let mut iVar10: *mut Struct115;
    let mut iVar9: *mut Struct114;
    let mut unaff_DI: i16;
    let mut uVar10: u16;
    let mut uVar11: u8;
    let mut local_3a: [u8;c] = [0;c];
    let mut uStack46: u32;
    let mut uStack36: u32;
    let mut uStack30: u32;
    let mut uStack26: u16;
    let mut paStack18: *mut Struct99;
    let mut uStack14: u32;
    let mut puStack10: *mut u16;
    let mut paStack6: *mut Struct99;
    let mut uVar5: *mut Struct113;

    paVar4 = str_var1(in_register_00000002, param_5);
    pass1_1038_53ba(param_3, 0x1);
    uVar7  = param_6 | paVar4;
    uVar10 = (param_2 >> 0x10);
    uVar11 = SUB41(param_2, 0x0);
    if(uVar7 != 0x0)
    {
        uStack30  = globals._PTR_LOOP_1050_5768;
        uVar6     = globals._PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
        uVar7     = (paStack18 >> 0x10);
        paVar4    = (uVar6 & 0xffff0000 | paStack18 & 0xffff);
        if((uVar7 | (paStack18 & 0xffff)) == 0x0)
        {
            paStack6 = 0x0;
        }
        else
        {
            iVar11               = paStack18;
            paStack18.field_0x0 = addr_table_1008_380a[36]; // 0x389a
            iVar11.field_0x2    = SEG_1008;
            iVar11.field_0x4    = 0x73;
            paStack18.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
            iVar11.field_0x2    = SEG_1030;
            paVar4               = paStack18;
            paStack6             = paStack18;
        }
        ppcVar1 = (*param_2 + 0x4);
        (**ppcVar1)(SEG_1000, uVar11, uVar10, paStack6, (paStack6 >> 0x10));
        uVar7 = dx_var1;
    }
    pass1_1038_53ba(param_3, 0x2);
    uVar7 = uVar7 | paVar4;
    if(uVar7 != 0x0)
    {
        uStack30  = globals._PTR_LOOP_1050_5768;
        uVar6     = globals._PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
        uVar7     = (paStack18 >> 0x10);
        paVar4    = (uVar6 & 0xffff0000 | paStack18 & 0xffff);
        if((uVar7 | (paStack18 & 0xffff)) == 0x0)
        {
            paStack6 = 0x0;
        }
        else
        {
            iVar10               = paStack18;
            paStack18.field_0x0 = addr_table_1008_380a[36]; // 0x389a
            iVar10.field_0x2    = SEG_1008;
            iVar10.field_0x4    = 0x74;
            paStack18.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
            iVar10.field_0x2    = SEG_1030;
            paVar4               = paStack18;
            paStack6             = paStack18;
        }
        ppcVar1 = (*param_2 + 0x8);
        (**ppcVar1)(SEG_1000, uVar11, uVar10, paStack6, (paStack6 >> 0x10));
        uVar7 = dx_var1_00;
    }
    pass1_1038_53ba(param_3, 0x3);
    puVar8 = (uVar7 | paVar4);
    if(puVar8 != 0x0)
    {
        uStack36  = globals._PTR_LOOP_1050_5768;
        uVar6     = globals._PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
        uVar7     = (paStack18 >> 0x10);
        paVar4    = (uVar6 & 0xffff0000 | paStack18 & 0xffff);
        if((uVar7 | (paStack18 & 0xffff)) == 0x0)
        {
            paStack6 = 0x0;
        }
        else
        {
            iVar9                = paStack18;
            paStack18.field_0x0 = addr_table_1008_380a[36]; // 0x389a
            iVar9.field_0x2     = SEG_1008;
            iVar9.field_0x4     = 0x75;
            paStack18.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
            iVar9.field_0x2     = SEG_1030;
            paVar4               = paStack18;
            paStack6             = paStack18;
        }
        ppcVar1 = (*param_2 + 0x8);
        (**ppcVar1)(SEG_1000, uVar11, uVar10, paStack6, (paStack6 >> 0x10));
        puVar8 = dx_var1_01;
    }
    pass1_1030_8f04(param_1, (param_1 >> 0x10), param_3, paVar4, puVar8);
    if(paVar4 != 0x0)
    {
        uStack36  = globals._PTR_LOOP_1050_5768;
        paStack18 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
        uVar7     = (paStack18 >> 0x10);
        uVar5     = paStack18;
        if((uVar7 | uVar5) == 0x0)
        {
            paStack6 = 0x0;
        }
        else
        {
            paStack18.field_0x0 = addr_table_1008_380a[36]; // 0x389a
            uVar5.field_0x2     = SEG_1008;
            uVar5.field_0x4     = 0x37;
            paStack18.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
            uVar5.field_0x2     = SEG_1030;
            paStack6             = paStack18;
        }
        ppcVar1 = (*param_2 + 0x8);
        (**ppcVar1)(SEG_1000, uVar11, uVar10, paStack6, (paStack6 >> 0x10));
        puVar8 = dx_var1_02;
    }
    puStack10 = mixed_1010_20ba(globals.data_1050_0ed0, 0x8, param_4, puVar8, unaff_DI);
    u_var2     = (puStack10 >> 0x10);
    uStack14  = *(puStack10 + 0xe);
    uVar7     = (puStack10 + 0x10);
    if((uVar7 | uStack14) != 0x0)
    {
        pass1_1008_5784(str_var1(param_4, local_3a), uStack14 & 0xffff | uVar7 << 0x10);
        while(true)
        {
            puVar3 = local_3a;
            pass1_1008_5b12(puVar3, param_4);
            uStack46 = str_var1(dx_var1_03, puVar3);
            if((dx_var1_03 | puVar3) == 0x0)
                break;
            if(((puVar3 + 0x4) == 0x3e) || ((puVar3 + 0x4) == 0x41))
            {
                uStack30  = globals._PTR_LOOP_1050_5768;
                paStack18 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
                uVar9     = (paStack18 >> 0x10);
                uVar7     = paStack18;
                if((uVar9 | uVar7) == 0x0)
                {
                    paStack6 = 0x0;
                }
                else
                {
                    uStack26             = (uStack46 + 0x4);
                    paStack18.field_0x0 = addr_table_1008_380a[36]; // 0x389a
                    (uVar7 + 0x2)        = SEG_1008;
                    (uVar7 + 0x4)        = uStack26;
                    paStack18.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
                    (uVar7 + 0x2)        = SEG_1030;
                    paStack6             = paStack18;
                }
                ppcVar1 = (*param_2 + 0x8);
                (**ppcVar1)(SEG_1000, uVar11, uVar10, paStack6, (paStack6 >> 0x10));
            }
        }
    }
    return;
}


void  pass1_1030_951a(param_1: u16, param_2: u16, param_3: u32, param_4: *mut u32, param_5: u32)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut uVar6: u16;
    let mut puVar7: *mut u16;
    let mut uVar8: u16;
    let mut puVar9: *mut u8;
    let mut dx_var1: u16;
    let mut uVar10: u16;
    let mut dx_var1_00: u16;
    let mut dx_var1_01: u16;
    let mut iVar11: i16;
    let mut puVar12: *mut u16;
    let mut unaff_DI: i16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut uVar15: u8;
    let mut puVar16: *mut u32;
    let mut uVar17: u32;
    let mut uVar18: u8;
    let mut uVar19: u8;
    let mut u_var20: u8;
    let mut puStack76: *mut u32;
    let mut uStack70: u32;
    let mut u_stack62: u32;
    let mut paStack58: *mut Struct99;
    let mut local_36: u16;
    let mut uStack52: u16;
    let mut uStack46: u32;
    let mut uStack42: u16;
    let mut uStack40: u16;
    let mut iStack38: i16;
    let mut puStack36: *mut u16;
    let mut puStack32: *mut u16;
    let mut iStack28: i16;
    let mut iStack20: i16;
    let mut uStack18: u32;
    let mut uStack14: u32;
    let mut puStack10: *mut u16;
    let mut paStack6: *mut Struct99;
    let mut u_var2: *mut Struct122;
    let mut uVar3: *mut Struct123;
    let mut u_var4: *mut Struct124;
    let mut uVar5: *mut Struct125;

    puStack10 = mixed_1010_20ba(globals.data_1050_0ed0, 0x35, param_1, param_2, unaff_DI);
    puVar9    = (puStack10 >> 0x10);
    uVar6     = puStack10 + 0xa;
    uStack14  = puStack10 & 0xffff0000 | uVar6;
    pass1_1030_9048(param_1, param_3, 0x0, param_5);
    uVar13 = (param_4 >> 0x10);
    u_var20 = SUB41(param_4, 0x0);
    if(uVar6 != 0x0)
    {
        iStack28  = 0x0;
        puStack32 = mixed_1010_20ba(globals.data_1050_0ed0, 0x8, param_1, puVar9, unaff_DI);
        uVar14    = (puStack32 >> 0x10);
        puStack36 = *(u16 **)(puStack32 + 0xe);
        uVar6     = (puStack32 + 0x10);
        if((uVar6 | puStack36) != 0x0)
        {
            pass1_1008_5784(str_var1(param_1, &local_36), puStack36 & 0xffff | uVar6 << 0x10);
            while(true)
            {
                puVar7 = &local_36;
                pass1_1008_5b12(puVar7, param_1);
                uStack46 = str_var1(dx_var1, puVar7);
                if((dx_var1 | puVar7) == 0x0)
                    break;
                if((puVar7[0x2] != 0x3e) && (puVar7[0x2] != 0x41))
                {
                    paStack6 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
                    uVar10   = (paStack6 >> 0x10);
                    uVar6    = paStack6;
                    if((uVar10 | uVar6) == 0x0)
                    {
                        paStack6 = 0x0;
                    }
                    else
                    {
                        uVar14              = (uStack46 + 0x4);
                        paStack6.field_0x0 = addr_table_1008_380a[36]; // 0x389a
                        (uVar6 + 0x2)       = SEG_1008;
                        (uVar6 + 0x4)       = uVar14;
                        paStack6.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
                        (uVar6 + 0x2)       = SEG_1030;
                    }
                    ppcVar1 = (*param_4 + 0x8);
                    (**ppcVar1)(0x0, u_var20, uVar13, paStack6, (paStack6 >> 0x10));
                    if((uStack46 + 0x4) == 0x13)
                    {
                        iStack28 = 0x1;
                    }
                }
            }
        }
        for(iStack38 = 0xa; iStack38 < 0x41; iStack38 = iStack38 + 0x1)
        {
            if((((((iStack38 != 0x37) && (iStack38 != 0x35)) && (iStack38 != 0x36)) && ((iStack38 != 0x25 && (iStack38 != 0x26)))) && ((iStack38 != 0x27 && (((iStack38 * 0x2 + uStack14) != 0x0 && (iStack38 != 0x13))))))
               && ((iStack38 != 0x14 || (iStack28 == 0x0))))
            {
                paStack6 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
                uVar10   = (paStack6 >> 0x10);
                uVar6    = paStack6;
                if((uVar10 | uVar6) == 0x0)
                {
                    paStack6 = 0x0;
                }
                else
                {
                    paStack6.field_0x0 = addr_table_1008_380a[36]; // 0x389a
                    (uVar6 + 0x2)       = SEG_1008;
                    (uVar6 + 0x4)       = iStack38;
                    paStack6.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
                    (uVar6 + 0x2)       = SEG_1030;
                }
                ppcVar1 = (*param_4 + 0x8);
                (**ppcVar1)(0x0, u_var20, uVar13, paStack6, (paStack6 >> 0x10));
            }
        }
    }
    uVar14 = (uStack14 >> 0x10);
    if((uStack14 + 0x6a) == 0x0)
    {
        if((uStack14 + 0x6c) != 0x0)
        {
            paStack58 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
            uVar6     = (paStack58 >> 0x10);
            puVar12   = paStack58;
            if((uVar6 | puVar12) == 0x0)
                //goto LAB_1030_973e;
            paStack58.field_0x0 = addr_table_1008_380a[36]; // 0x389a
            puVar12[0x1]         = SEG_1008;
            puVar12[0x2]         = 0x36;
            //goto LAB_1030_9728;
        }
    }
    else
    {
        paStack58 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
        uVar6     = (paStack58 >> 0x10);
        puVar12   = paStack58;
        if((uVar6 | puVar12) == 0x0)
        {
        // LAB_1030_973e:
            paStack6 = 0x0;
        }
        else
        {
            paStack58.field_0x0 = addr_table_1008_380a[36]; // 0x389a
            puVar12[0x1]         = SEG_1008;
            puVar12[0x2]         = 0x35;
        // LAB_1030_9728:
            *puVar12     = addr_table_1030_9ec8;//0x9ec8;
            puVar12[0x1] = SEG_1030;
            paStack6     = paStack58;
        }
        ppcVar1 = (*param_4 + 0x8);
        (**ppcVar1)(0x0, u_var20, uVar13, paStack6, (paStack6 >> 0x10));
    }
    uVar14 = (uStack14 >> 0x10);
    iVar11 = uStack14;
    if((iVar11 + 0x4a) == 0x0)
    {
        if((iVar11 + 0x4c) == 0x0)
        {
            if((iVar11 + 0x4e) == 0x0)
                //goto LAB_1030_97e8;
            paStack58 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
            uVar6     = (paStack58 >> 0x10);
            puVar12   = paStack58;
            if((uVar6 | puVar12) != 0x0)
            {
                paStack58.field_0x0 = addr_table_1008_380a[36]; // 0x389a
                puVar12[0x1]         = SEG_1008;
                puVar12[0x2]         = 0x27;
                //goto LAB_1030_9879;
            }
        }
        else
        {
            paStack58 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
            uVar6     = (paStack58 >> 0x10);
            puVar12   = paStack58;
            if((uVar6 | puVar12) != 0x0)
            {
                paStack58.field_0x0 = addr_table_1008_380a[36]; // 0x389a
                puVar12[0x1]         = SEG_1008;
                puVar12[0x2]         = 0x26;
                //goto LAB_1030_9879;
            }
        }
    // LAB_1030_97d0:
        paStack6 = 0x0;
    }
    else
    {
        paStack58 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
        uVar6     = (paStack58 >> 0x10);
        puVar12   = paStack58;
        if((uVar6 | puVar12) == 0x0)
            //goto LAB_1030_97d0;
        paStack58.field_0x0 = addr_table_1008_380a[36]; // 0x389a
        puVar12[0x1]         = SEG_1008;
        puVar12[0x2]         = 0x25;
    // LAB_1030_9879:
        *puVar12     = addr_table_1030_9ec8;//0x9ec8;
        puVar12[0x1] = SEG_1030;
        paStack6     = paStack58;
    }
    ppcVar1 = (*param_4 + 0x8);
    (**ppcVar1)(0x0, u_var20, uVar13, paStack6, (paStack6 >> 0x10));
// LAB_1030_97e8:
    uStack18 = puStack10 & 0xffff0000 | (puStack10 + 0x11e);
    if((puStack10 + 0x138) != 0x0)
    {
        puVar16 = pass1_1008_c6fa(globals.dat_1050_06e0, 0x4);
        puVar9  = (puVar16 >> 0x10);
        uVar6   = puVar16;
        uVar15  = 0x38;
        pass1_1038_4d6e(param_5, puVar16, uVar6, puVar9);
        puStack76 = str_var1(puVar9, uVar6);
        ppcVar1   = (*puStack76 + 0x10);
        uVar10    = uVar6;
        (**ppcVar1)(SEG_1038, uVar6, puVar9);
        uStack70 = str_var1(dx_var1_00, uVar10);
        for(u_stack62 = 0x0; u_stack62 < uStack70; u_stack62 = u_stack62 + 0x1)
        {
            ppcVar1 = (*puStack76 + 0x4);
            uVar17  = uStack70;
            (**ppcVar1)(uVar15, uVar6, puVar9, u_stack62, (u_stack62 >> 0x10));
            uVar8    = uVar17;
            iVar11   = 0xd;
            uVar10   = dx_var1_01;
            local_36 = uVar8;
            uStack52 = dx_var1_01;
            pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, uVar8, dx_var1_01);
            uStack46 = str_var1(uVar10, uVar8);
            uVar17   = struct_op_1030_73a8(str_var1(uVar10, uVar8));
            uVar10   = (uVar17 >> 0x10);
            uStack42 = uVar17;
            uVar15   = 0x28;
            uStack40 = uVar10;
            uVar8    = pass1_1028_6744(param_1, uVar17, iVar11);
            if((uVar10 | uVar8) != 0x0)
            {
                puStack32 = globals._PTR_LOOP_1050_5768;
                paStack6  = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
                uVar10    = (paStack6 >> 0x10);
                uVar5     = paStack6;
                if((uVar10 | uVar5) == 0x0)
                {
                    paStack6 = 0x0;
                }
                else
                {
                    paStack6.field_0x0 = addr_table_1008_380a[36]; // 0x389a
                    uVar5.field_0x2    = SEG_1008;
                    uVar5.field_0x4    = 0x4c;
                    paStack6.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
                    uVar5.field_0x2    = SEG_1030;
                }
                ppcVar1 = (*param_4 + 0x8);
                (**ppcVar1)(0x0, u_var20, uVar13, paStack6, (paStack6 >> 0x10));
                puStack36 = globals._PTR_LOOP_1050_5768;
                paStack6  = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
                uVar10    = (paStack6 >> 0x10);
                u_var4     = paStack6;
                if((uVar10 | u_var4) == 0x0)
                {
                    paStack6 = 0x0;
                }
                else
                {
                    paStack6.field_0x0 = addr_table_1008_380a[36]; // 0x389a
                    u_var4.field_0x2    = SEG_1008;
                    u_var4.field_0x4    = 0x4d;
                    paStack6.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
                    u_var4.field_0x2    = SEG_1030;
                }
                uVar18  = SUB41(paStack6, 0x0);
                uVar19  = (paStack6 >> 0x10);
                ppcVar1 = (*param_4 + 0x8);
                puVar16 = param_4;
                (**ppcVar1)();
                puStack36 = globals._PTR_LOOP_1050_5768;
                uVar15    = 0x0;
                paStack6  = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
                uVar10    = (paStack6 >> 0x10);
                uVar3     = paStack6;
                if((uVar10 | uVar3) == 0x0)
                {
                    paStack6 = 0x0;
                }
                else
                {
                    paStack6.field_0x0 = addr_table_1008_380a[36]; // 0x389a
                    uVar3.field_0x2    = SEG_1008;
                    uVar3.field_0x4    = 0x4e;
                    paStack6.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
                    uVar3.field_0x2    = SEG_1030;
                }
                ppcVar1 = (*param_4 + 0x8);
                (**ppcVar1)(SEG_1000, param_4, paStack6, puVar16, uVar18, uVar19);
                break;
            }
        }
        if(puStack76 != 0x0)
        {
            ppcVar1 = *puStack76;
            (**ppcVar1)(uVar15, uVar6, puVar9, 0x1);
        }
    }
    for(iStack20 = 0x7a; iStack20 < 0x7d; iStack20 = iStack20 + 0x1)
    {
        if((iStack20 * 0x2 + uStack14) != 0x0)
        {
            paStack6 = pass1_1000_07fc(SEG_1000, globals._PTR_LOOP_1050_5768);
            uVar6    = (paStack6 >> 0x10);
            u_var2    = paStack6;
            if((uVar6 | u_var2) == 0x0)
            {
                paStack6 = 0x0;
            }
            else
            {
                paStack6.field_0x0 = addr_table_1008_380a[36]; // 0x389a
                u_var2.field_0x2    = SEG_1008;
                u_var2.field_0x4    = iStack20;
                paStack6.field_0x0 = addr_table_1030_9ec8;//0x9ec8;
                u_var2.field_0x2    = SEG_1030;
            }
            ppcVar1 = (*param_4 + 0x8);
            (**ppcVar1)(0x0, u_var20, uVar13, paStack6, (paStack6 >> 0x10));
        }
    }
    return;
}


u16  pass1_1030_7bee(param_1: u32)

{
    fn_ptr_1 *ppcVar1;
    let mut u_var2: u16;
    let mut iVar3: i16;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if((iVar3 + 0x16) == 0x0)
    {
        return 0x0;
    }
    if((iVar3 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | u_var4 << 0x10);
    }
    ppcVar1 = ((iVar3 + 0x1a) + 0x44);
    u_var2   = (**ppcVar1)();
    return u_var2;
}


void  pass1_1030_7e5a(param_1: *mut Struct358, param_2: u32, param_3: u16)

{
    let mut iVar1: *mut Struct358;
    let mut uVar1: u16;

    uVar1             = (param_1 >> 0x10);
    iVar1             = param_1;
    iVar1.field_0x16 = param_2;
    iVar1.field_0x1a_addr_offset = 0x0;
    pass1_1030_6fa0(param_1 & 0xffff | uVar1 << 0x10);
    if(iVar1.field_0x2e != 0x0)
    {
        pass1_1038_4b20(iVar1.field_0x2e, iVar1.field_0x16, iVar1.field_0x4, param_3);
    }
    return;
}


void  pass1_1030_7eda(param_1: u32, param_2: u16, param_3: u16)

{
    let mut uVar1: u16;
    let mut local_c: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut u_stack6: u16;
    let mut uStack4: u16;

    local_c  = 0x0;
    uStack10 = 0x0;
    u_stack6  = 0x0;
    uStack4  = 0x0;
    uStack8  = param_2;
    uVar1    = (param_1 >> 0x10);
    if((param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | uVar1 << 0x10);
    }
    pass1_1028_bb96(*(param_1 + 0x1a), &local_c, param_3);
    return;
}


void  pass1_1030_7f1a(param_1: u32, param_2: u16, param_3: u16)

{
    let mut uVar1: u16;
    let mut local_c: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut u_stack6: u16;
    let mut uStack4: u16;

    local_c  = 0x0;
    uStack8  = 0x0;
    u_stack6  = 0x0;
    uStack4  = 0x0;
    uStack10 = param_2;
    uVar1    = (param_1 >> 0x10);
    if((param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | uVar1 << 0x10);
    }
    pass1_1028_bb96(*(param_1 + 0x1a), &local_c, param_3);
    return;
}


u16  pass1_1030_7f5a(param_1: u32)

{
    let mut uVar1: u16;
    let mut u_var2: u32;

    uVar1 = (param_1 >> 0x10);
    if((param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | uVar1 << 0x10);
    }
    u_var2 = pass1_1028_bb6a(*(param_1 + 0x1a));
    if(u_var2 != 0x0)
    {
        return (u_var2 + 0x4);
    }
    return 0x0;
}


u16  pass1_1030_7f98(param_1: u32)

{
    let mut uVar1: u16;
    let mut u_var2: u32;

    uVar1 = (param_1 >> 0x10);
    if((param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | uVar1 << 0x10);
    }
    u_var2 = pass1_1028_bb6a(*(param_1 + 0x1a));
    if(u_var2 != 0x0)
    {
        return (u_var2 + 0x2);
    }
    return 0x0;
}


void  pass1_1030_7fd6(param_1: u32, param_2: u16, param_3: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u32;
    let mut iVar3: i16;
    let mut u_var4: u16;
    let mut uVar5: u32;

    u_var4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if((iVar3 + 0x1a) == 0x0)
    {
        uVar5   = struct_op_1030_73a8(param_1 & 0xffff | u_var4 << 0x10);
        param_2 = (uVar5 >> 0x10);
    }
    u_var2 = (iVar3 + 0x1a);
    iVar1 = (u_var2 + 0xc);
    if(((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33))) && ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2))))))
    {
        pass1_1028_1416(*(iVar3 + 0x1a), param_2, param_3);
    }
    return;
}


void  pass1_1030_8030(param_1: u32, param_2: u16, param_3: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u32;
    let mut iVar3: i16;
    let mut u_var4: u16;
    let mut uVar5: u32;

    u_var4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if((iVar3 + 0x1a) == 0x0)
    {
        uVar5   = struct_op_1030_73a8(param_1 & 0xffff | u_var4 << 0x10);
        param_2 = (uVar5 >> 0x10);
    }
    u_var2 = (iVar3 + 0x1a);
    iVar1 = (u_var2 + 0xc);
    if(((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33))) && ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2))))))
    {
        uVar5 = *(iVar3 + 0x1a);
        pass1_1028_1106(uVar5, uVar5, param_2, param_3);
    }
    return;
}


void  pass1_1030_809c(param_1: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if((param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | uVar1 << 0x10);
    }
    return;
}


void  struct_1030_8544(param_1: *mut Struct355, u16 *param_2)

{
    let mut iVar1: *mut Struct356;
    let mut iVar2: *mut Struct355;
    let mut uVar1: u16;
    let mut u_var2: u16;

    param_1.field_0x0 = *param_2;
    uVar1 = (param_2 >> 0x10);
    iVar1            = param_2;
    u_var2            = (param_1 >> 0x10);
    iVar2            = param_1;
    iVar2.field_0x4 = iVar1.field_0x4;
    pass1_1008_3f62((param_1 & 0xffff0000 | &iVar2.field_0x8), (param_2 & 0xffff0000 | &iVar1.field_0x8));
    iVar2.field_0xe  = iVar1.field_0xe;
    iVar2.field_0x12 = iVar1.field_0x12;
    iVar2.field_0x16 = iVar1.field_0x16;
    iVar2.field_0x1a_addr_offset = iVar1.field_0x1a_addr_offset;
    iVar2.field_0x1e = 0x0;
    return;
}


void  pass1_1030_85be(long *param_1, param_2: u16, param_3: i16, param_4: i16, param_5: u16)

{
    let mut iVar1: *mut Struct357;
    let mut uVar1: u16;

    uVar1            = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = 0x0;
    iVar1.field_0x4 = 0x0;
    iVar1.field_0x6 = param_3;
    iVar1.field_0x8 = param_2;
    iVar1.field_0xe = 0x0;
    if(iVar1.field_0x6 == 0x0)
    {
        iVar1.field_0x6 = 0x5;
    }
    pass1_1030_878c(param_1, param_4, param_5);
    return;
}


void  pass1_1030_86ec(Struct18 **param_1, param_2: u16)

{
    let mut iVar1: *mut Struct612;
    let mut uVar1: u16;

    fn_ptr_1000_17ce(*param_1, SEG_1000);
    uVar1            = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 =  0x0;
    iVar1.field_0x4 = 0x0;
    iVar1.field_0x6 = param_2;
    iVar1.field_0xe = 0x0;
    return;
}


void  pass1_1030_6a2c(struct param_1: *mut Struct382, long param_2, param_3: u16, param_4: *mut u8, param_5: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut iVar2: *mut Struct384;
    let mut u_var2: u16;
    struct Struct382 *iVar4;
    let mut iVar5: *mut Struct383;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut uVar5: u32;
    let mut lVar6 = 0i32;
    let mut local_a: [u8;8] = [0;8];

    uVar3 = (param_1 >> 0x10);
    iVar4 = (struct Struct382 *)param_1;
    if(iVar4.field_0x3e == 0x0)
    {
        mem_op_1000_179c(0xc, param_4, 0);
        if((param_4 | param_3) == 0x0)
        {
            iVar4.field_0x3e = 0x0;
        }
        else
        {
            uVar5                      = set_struct_1008_574a(str_var1(param_4, param_3));
            &iVar4.field_0x3e         = uVar5;
            (&iVar4.field_0x3e + 0x2) = (uVar5 >> 0x10);
        }
    }
    pass1_1008_5784(str_var1(param_5, local_a), iVar4.field_0x3e);
    do
    {
        do
        {
            lVar6 = pass1_1008_5b12(local_a, param_5);
            u_var2 = (lVar6 >> 0x10);
            iVar2 = lVar6;
            if(lVar6 == 0x0)
                //goto LAB_1030_6af4;
            u_var4 = (param_2 >> 0x10);
            iVar5 = param_2;
        } while((iVar2.field_0x6 != iVar5.field_0x6) || (iVar2.field_0x4 != iVar5.field_0x4));
    } while(iVar2.field_0x8 != iVar5.field_0x8);
    iVar2.field_0xa = iVar2.field_0xa + iVar5.field_0xa;
    iVar2.field_0xc = iVar2.field_0xc + iVar5.field_0xc;
    param_2          = 0x0;
// LAB_1030_6af4:
    if(param_2 != 0x0)
    {
        ppcVar1 = (*iVar4.field_0x3e + 0x8);
        (**ppcVar1)(SEG_1008, iVar4.field_0x3e, param_2);
    }
    return;
}


u32  pass1_1030_6b16(param_1: *mut Struct412)

{
    let mut puVar1: *mut u32;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut u_var4: u32;
    let mut iVar5: *mut Struct412;
    let mut uVar5: u16;
    let mut uVar6: u32;

    uVar5 = (param_1 >> 0x10);
    iVar5 = param_1;
    if(&iVar5.field_0x3a == 0x0)
    {
        return 0x0;
    }
    ppcVar3 = (&iVar5.field_0x3a + 0x10);
    uVar6   = (**ppcVar3)();
    u_var4   = &iVar5.field_0x3a;
    if((u_var4 + 0x8) == 0x0)
    {
        puVar1 = &iVar5.field_0x3a;
        u_var2  = iVar5.field_0x3c;
        if((u_var2 | puVar1) != 0x0)
        {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        &iVar5.field_0x3a = 0x0;
    }
    return uVar6;
}


void  pass1_1030_6c66(param_1: *mut Struct386, param_2: i16, param_3: u32, param_4: u16, param_5: *mut u8, param_6: u16)

{
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut BVar5: BOOL16;
    let mut dx_var1: u16;
    let mut puVar6: *mut u8;
    let mut iVar7: *mut Struct386;
    let mut iVar6: *mut Struct385;
    let mut unaff_SI: u16;
    let mut unaff_DI: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut ss_var1: u16;

    uVar7 = (param_1 >> 0x10);
    iVar7 = param_1;
    if(iVar7.field_0x3a == 0x0)
    {
        param_6 = SEG_1000;
        mem_op_1000_179c(0xc, param_5, 0);
        if((param_5 | param_4) == 0x0)
        {
            iVar7.field_0x3a = 0x0;
        }
        else
        {
            param_6 = SEG_1008;
            set_struct_1008_574a(str_var1(param_5, param_4));
            &iVar7.field_0x3a         = param_4;
            (&iVar7.field_0x3a + 0x2) = dx_var1;
        }
    }
    ppcVar2 = (*iVar7.field_0x3a + 0x8);
    (**ppcVar2)(param_6, iVar7.field_0x3a, param_3);
    if(param_2 != 0x0)
    {
        uVar8 = (param_3 >> 0x10);
        iVar6 = param_3;
        if(iVar6.field_0x6 != 0x0)
        {
            pass1_1030_6e9c(param_1, iVar6.field_0xa, iVar6.field_0x6);
            return;
        }
        if(iVar6.field_0x4 != 0x0)
        {
            uVar1  = iVar6.field_0xa;
            uVar3  = -uVar1;
            puVar6 = -(uVar1 != 0x0);
            pass1_1030_7ddc(param_1, CONCAT13((puVar6 >> 0x8), CONCAT12(puVar6, uVar3)), iVar6.field_0x4, uVar3, puVar6, unaff_SI, unaff_DI, ss_var1);
            return;
        }
        if(iVar6.field_0x8 != 0x0)
        {
            u_var4 = pass1_1030_6fa0(param_1);
            BVar5 = pass1_1008_c6ae(globals.dat_1050_06e0, u_var4, 0x4);
            if(BVar5 != 0x0)
            {
                pass1_1028_6356(iVar7.field_0x1a_addr_offset, 0x0, iVar6.field_0xa, 0x0, ss_var1);
            }
        }
    }
    return;
}


void  pass1_1030_6e4c(param_1: u32)

{
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if((iVar2 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | uVar3 << 0x10);
    }
    if(((iVar2 + 0x1a) != 0x0) && (uVar1 = (iVar2 + 0x1a), (uVar1 + 0x12) == 0x4))
    {
        return;
    }
    return;
}


u16  pass1_1030_6fa0(param_1: u32)

{
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if((iVar2 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | uVar3 << 0x10);
    }
    if((iVar2 + 0x1a) != 0x0)
    {
        uVar1 = (iVar2 + 0x1a);
        return (uVar1 + 0xc);
    }
    return 0x0;
}


void  pass1_1030_6fd4(param_1: u32)

{
    let mut uVar1: u32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    if((param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | u_var2 << 0x10);
    }
    uVar1 = (param_1 + 0x1a);
    if((uVar1 + 0x12) == 0x5)
    {
        return;
    }
    return;
}


void  pass1_1030_701c(param_1: u32)

{
    let mut uVar1: u32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    if((param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | u_var2 << 0x10);
    }
    uVar1 = (param_1 + 0x1a);
    if((uVar1 + 0x12) == 0x5)
    {
        return;
    }
    return;
}


void  pass1_1030_7064(param_1: u32)

{
    let mut uVar1: u32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    if((param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | u_var2 << 0x10);
    }
    uVar1 = (param_1 + 0x1a);
    if((uVar1 + 0x12) == 0x5)
    {
        return;
    }
    return;
}


void  pass1_1030_70ac(param_1: u32)

{
    let mut uVar1: u32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    if((param_1 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | u_var2 << 0x10);
    }
    uVar1 = (param_1 + 0x1a);
    if((uVar1 + 0x12) == 0x5)
    {
        return;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1030_70f4(param_1: u32)

{
    let mut uVar1: u16;
    let mut u_var2: u32;
    let mut BVar3: BOOL16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    long      *plVar6;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if((iVar4 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | uVar5 << 0x10);
    }
    u_var2 = (iVar4 + 0x1a);
    uVar1 = (u_var2 + 0xc);
    BVar3 = pass1_1008_c6ae(globals.dat_1050_06e0, uVar1, 0x1);
    if(BVar3 == 0x0)
    {
        BVar3 = pass1_1008_c6ae(globals.dat_1050_06e0, uVar1, 0x2);
        if((BVar3 == 0x0) || ((iVar4 + 0x22) == 0x0))
        {
            return;
        }
        plVar6 = *(long **)(iVar4 + 0x22);
    }
    else
    {
        u_var2  = (iVar4 + 0x1a);
        plVar6 = *(long **)(u_var2 + 0x28);
    }
    pass1_1020_ba94(plVar6);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1030_7176(param_1: u32, param_2: u16)

{
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut local_1a = 0i32;
    i16        local_16[0x2];
    let mut uStack18: u16;
    let mut uStack14: u16;
    let mut BStack10: BOOL16;
    let mut uStack8: u16;
    let mut lStack6 = 0i32;

    lStack6 = 0x0;
    uVar3   = (param_1 >> 0x10);
    iVar2   = param_1;
    if((iVar2 + 0x22) == 0x0)
    {
        return;
    }
    if((iVar2 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1);
    }
    uVar1    = (iVar2 + 0x1a);
    uStack8  = (uVar1 + 0xc);
    BStack10 = pass1_1008_c6ae(globals.dat_1050_06e0, uStack8, 0x3);
    if((BStack10 != 0x0) && (uVar1 = (iVar2 + 0x1a), (uVar1 + 0x12) == 0x5))
    {
        uVar1    = (iVar2 + 0x22);
        uStack14 = (uVar1 + 0x4);
        for(uStack18 = 0x0; uStack18 < uStack14; uStack18 = uStack18 + 0x1)
        {
            pass1_1020_bb16((iVar2 + 0x22),
                            str_var1(param_2, &local_1a),
                            str_var1(param_2, local_16), uStack18);
            if(0x0 < local_16[0])
            {
                lStack6 = lStack6 + local_1a;
            }
        }
    }
    return;
}
