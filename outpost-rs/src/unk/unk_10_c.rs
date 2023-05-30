// #include "unk_10.h"

// #include "address_tables/function_tables.h"
// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "struct_20.h"
// #include "struct_ops/struct_ops_3.h"
// #include "structs/structs_5xx/structs_59x.h"

u16 * pass1_1020_d3a4(param_1: *mut u16, param_2: u16, param_3: i16, param_4: u32, param_5: u16)

{
    let mut uVar1: u16;

    pass1_1028_b39e(param_1, param_3, param_4, param_5);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x20) = param_2;
    param_1.field_0x0 = addr_table_1020_d53e;//0xd53e;
    param_1.field_0x2 = SEG_1020;
    return param_1;
}

u16  pass1_1020_d460(param_1: *mut u32,param_2: *mut u16, param_3: u32, param_4: u32, param_5: i16, param_6: u16, param_7: i16, u64 param_8)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u8;
    let mut ss_var1: u16;
    let mut uVar3: u32;
    let mut pu_var4: *mut u16;

    uVar1 = pass1_1028_bc90(param_1, param_2, param_3, param_4, param_5, param_6, ss_var1);
    if(uVar1 != 0x0)
    {
        uVar3                       = pass1_1038_af40(globals.ptr_1050_5b7c, (globals._PTR_LOOP_1050_4230 + 0x16), 0x11, param_6, globals._PTR_LOOP_1050_4230, SEG_1038, ss_var1);
        pu_var2                      = (uVar3 >> 0x10);
        globals.PTR_LOOP_1050_5b80 = (&PTR_LOOP_1050_0000 + 0x1);
        unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5b80, SEG_1008, ss_var1);
        pu_var4           = mixed_1010_20ba(globals.data_1050_0ed0, 0x3a, ss_var1, pu_var2, param_7);
        (param_1 + 0x20) = (pu_var4 + 0xa);
        uVar1            = 0x1;
    }
    return uVar1;
}


void  pass1_1020_d4ca(param_1: u32, param_2: i16)

{
    let mut BVar1: BOOL16;
    let mut u_var2: u32;
    let mut dx_var1: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;

    if((param_1 + 0x20) == 0x2)
    {
        return;
    }
    pass1_1028_b58e(param_1);
    u_var2 = *(param_2 + 0x2e);
    iVar4 = 0x63;
    uVar3 = dx_var1;
    pass1_1038_3fb0(u_var2);
    BVar1 = pass1_1030_25b2(u_var2 & 0xffff | uVar3 << 0x10, iVar4);
    if(BVar1 != 0x0)
    {
        return;
    }
    return;
}

u16 * pass1_1020_d5c8(param_1: i16, param_2: u16, param_3: i16, param_4: u32, param_5: u16)

{
    pass1_1028_b39e(str_var1(param_2, param_1), param_3, param_4, param_5);
    param_1 =  addr_table_1020_d7fe;//0xd7fe;
    param_1.fld2_segment = SEG_1020;
    return param_1;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1020_d5f2(param_1: u32, param_2: u32, param_3: i16, param_4: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u16;
    let mut puVar3: *mut u32;
    let mut dx_var1: u16;
    let mut u_var4: u16;
    let mut uVar5: u32;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut bStack55: u8;
    let mut local_32: [u8;a] = [0;a];
    let mut uStack40: u32;
    let mut uStack36: u32;
    let mut puStack28: *mut u32;
    let mut local_1a: u32;
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut iStack14: i16;
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_b58e(param_1);
    local_c   = (param_3 + 0xc);
    iStack18  = (param_3 + 0x10);
    puStack28 = &local_c;
    uStack16  = dx_var1;
    iStack14  = iStack18;
    iStack8   = iStack18;
    iStack6   = param_3;
    uStack4   = dx_var1;
    pass1_1028_bab6(param_1, iStack18, dx_var1);
    u_var2    = pass1_1030_2fac(str_var1(uStack16, iStack18));
    local_1a = local_c;
    iStack22 = iStack8;
    uStack36 = str_var1(uStack36, &local_1a);
    iStack14 = iStack14 + 0x1;
    uStack20 = u_var2;
    if(iStack14 < u_var2)
    {
        puVar7   = str_var1(param_4, local_32);
        iStack22 = iStack14;
        uVar5    = pass1_1028_bb24(param_1);
        u_var4    = (uVar5 >> 0x10);
        puVar3   = &local_1a;
        pass1_1030_64ce(param_4, puVar3, u_var4, globals._PTR_LOOP_1050_5740,
                        str_var1(param_4, puVar3), uVar5 & 0xffff | u_var4 << 0x10, puVar7);
        uStack40 = *puVar3;
        u_var4    = (puVar3 + 0x2);
        bStack55 = (uStack40 >> 0x18);
        u_var2    = bStack55;
        uStack36 = uStack40;
        if(bStack55 != 0x0)
        {
            pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, uStack40, u_var4);
            puVar6  = struct_op_1030_73a8(str_var1(u_var4, u_var2));
            u_var2   = puVar6;
            ppcVar1 = (*puVar6 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b46e(param_1, param_2, u_var2);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1020_d6e6(param_1: u32, param_2: i16, param_3: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut pu_var2: *mut u32;
    let mut uVar3: u16;
    let mut dx_var1: u16;
    let mut u_var4: u16;
    let mut uVar5: u32;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut bStack55: u8;
    let mut local_32: [u8;a] = [0;a];
    let mut uStack40: u32;
    let mut uStack36: u32;
    let mut puStack28: *mut u32;
    let mut local_1a: u32;
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut iStack14: i16;
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_b514(param_1);
    pass1_1028_b58e(param_1);
    local_c   = (param_2 + 0xc);
    iStack18  = (param_2 + 0x10);
    puStack28 = &local_c;
    uStack16  = dx_var1;
    iStack14  = iStack18;
    iStack8   = iStack18;
    iStack6   = param_2;
    uStack4   = dx_var1;
    pass1_1028_bab6(param_1, iStack18, dx_var1);
    uStack20 = pass1_1030_2fac(str_var1(uStack16, iStack18));
    local_1a = local_c;
    uStack36 = str_var1(uStack36, &local_1a);
    iStack22 = iStack14 + 0x1;
    if(iStack22 < uStack20)
    {
        puVar7   = str_var1(param_3, local_32);
        iStack14 = iStack22;
        uVar5    = pass1_1028_bb24(param_1);
        u_var4    = (uVar5 >> 0x10);
        pu_var2   = &local_1a;
        pass1_1030_64ce(param_3, pu_var2, u_var4, globals._PTR_LOOP_1050_5740,
                        str_var1(param_3, pu_var2), uVar5 & 0xffff | u_var4 << 0x10, puVar7);
        uStack40 = *pu_var2;
        u_var4    = (pu_var2 + 0x2);
        bStack55 = (uStack40 >> 0x18);
        uVar3    = bStack55;
        if(bStack55 != 0x0)
        {
            uStack36 = uStack40;
            pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, uStack40, u_var4);
            puVar6 = struct_op_1030_73a8(str_var1(u_var4, uVar3));
            if((puVar6 + 0xc) == 0x6a)
            {
                ppcVar1 = (*puVar6 + 0x24);
                (**ppcVar1)();
            }
        }
    }
    return;
}

void  pass1_1020_b97e(param_1: u16, param_2: i16, param_3: u16, param_4: u16, param_5: u16, i16 param_6)

{
    let mut uVar1: u32;
    let mut local_e: i16;
    let mut local_c: u16;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut u_stack6: u32;

    pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, 0x2, 0x400);
    globals._PTR_LOOP_1050_4e70 = str_var1(param_3, param_2);
    uVar1                        = (param_2 + 0x10);
    u_stack6                      = uVar1;
    pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    iStack10 = uVar1;
    uStack8  = param_3;
    pass1_1008_3f62(&PTR_LOOP_1048_4230, str_var1(param_3, iStack10 + 0xc));
    pass1_1008_3e94(&PTR_LOOP_1048_4230, str_var1(param_1, &local_e), str_var1(param_1, &local_c));
    if(param_6 == 0x0)
    {
        pass1_1008_3e76(&PTR_LOOP_1048_4230, 0x0, local_e + 0x1, local_c - 0x1);
        pass1_1008_3e94(&PTR_LOOP_1048_4230, str_var1(param_1, &local_e), str_var1(param_1, &local_c));
    }
    pass1_1008_3e76(0x10484236, 0x1, local_e - 0x2, local_c);
    return;
}

pub fn pass1_1020_ba2b(void)

{
    init_globals_1020_96d4();
    pass1_1020_a426();
    return;
}

void  pass1_1020_ba3e(long *param_1, param_2: u16, param_3: i16, param_4: u16, param_5: u16)

{
    let mut iVar1: *mut Struct172;
    let mut uVar1: u16;
    let mut ss_var1: u16;

    uVar1            = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = 0x0;
    iVar1.field_0x4 = 0x0;
    iVar1.field_0x6 = param_3;
    iVar1.field_0x8 = param_2;
    if(iVar1.field_0x6 == 0x0)
    {
        iVar1.field_0x6 = 0x5;
    }
    pass1_1020_bcc4(param_1, param_4, ss_var1);
    return;
}

void  pass1_1020_ba94(long *param_1)

{
    let mut puVar1: *mut u16;
    let mut uStack8: u16;

    if (param_1.field_0x0 == 0x0) {
        return;
    }
    uStack8 = 0x0;
    while(true)
    {
        puVar1 = (param_1 + 0x4);
        if(*puVar1 < uStack8 || *puVar1 == uStack8)
            break;
        uStack8 = uStack8 + 0x1;
    }
    return;
}


u32  pass1_1020_bae6(param_1: u16, param_2: u32, param_3: u16, param_4: u16, param_5: u16)

{
    let mut pu_stack6: *mut u16;

    pass1_1020_bc92(str_var1(param_2, param_1), (param_2 >> 0x10), param_5);
    pu_stack6 = str_var1(param_4, param_3);
    if((param_4 | param_3) != 0x0)
    {
        return str_var1((param_3 + 0x2), *pu_stack6);
    }
    return 0x0;
}


void  pass1_1020_bb16(param_1: *mut u32, param_2: *mut u32,param_3: *mut u16, param_4: u16)

{
    if((param_1 + 0x4) < param_4)
    {
        *param_3 = 0x0;
        *param_2 = 0x0;
        return;
    }
    *param_3 = (param_4 * 0x6 + *param_1 + 0x4);
    *param_2 = *(*param_1 + param_4 * 0x6);
    return;
}


void  pass1_1020_bb70(long *param_1, param_2: u16, param_3: u32, param_4: u16, param_5: u16, param_6: u16)

{
    pass1_1020_bba4(param_1, 0x1, param_2, param_3, (param_3 >> 0x10), param_4, param_6);
    return;
}


void  pass1_1020_bb8a(long *param_1, param_2: u16, param_3: u32, param_4: u16, param_5: u16)

{
    pass1_1020_bba4(param_1, 0x0, param_2, param_3, (param_3 >> 0x10), param_4, param_5);
    return;
}


BOOL16  pass1_1020_bba4(long *param_1, param_2: i16, param_3: u16, param_4: i16, param_5: u16, param_6: u16, param_7: u16)

{
    let mut in_AX: *mut u16;
    let mut in_DX: u16;
    let mut uVar1: u16;
    let mut u_var2: u16;
    let mut bVar3: bool;
    let mut pu_stack6: *mut u16;

    pass1_1020_bc92(param_1, param_5, param_7);
    pu_stack6 = str_var1(in_DX, in_AX);
    uVar1    = in_DX | in_AX;
    if(uVar1 == 0x0)
    {
        pass1_1020_bc92(param_1, 0x0, param_7);
        u_var2 = uVar1 | in_AX;
        if(u_var2 == 0x0)
        {
            pass1_1020_bcc4(param_1, param_6, param_7);
            pass1_1020_bc92(param_1, 0x0, param_7);
            if((u_var2 | in_AX) == 0x0)
            {
                return 0x0;
            }
            in_AX[0x2] = param_5;
            uVar1      = u_var2;
        }
        else
        {
            in_AX[0x2] = param_5;
        }
        if(param_2 != 0x0)
        {
            u_var2   = *in_AX;
            bVar3   = CARRY2(u_var2, param_3);
            param_3 = u_var2 + param_3;
            param_4 = in_AX[0x1] + param_4 + bVar3;
        }
        *in_AX     = param_3;
        in_AX[0x1] = param_4;
        pass1_1020_bc72(param_1, param_6, param_7);
    }
    else
    {
        if(param_2 != 0x0)
        {
            bVar3   = CARRY2(*pu_stack6, param_3);
            param_3 = *pu_stack6 + param_3;
            param_4 = in_AX[0x1] + param_4 + bVar3;
        }
        *in_AX     = param_3;
        in_AX[0x1] = param_4;
    }
    return 0x1;
}


void  pass1_1020_bc72(param_1: *mut u16, param_2: u16, param_3: u16)

{
    let mut uVar1: u32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x2);
    pass1_1000_4aea(*param_1, uVar1, (uVar1 >> 0x10), 0x6, 0xbd6c, &stack0xfffe, param_2, u_var2, SEG_1000, param_3);
    return;
}


void  pass1_1020_bc92(param_1: *mut u16, param_2: u16, param_3: u16)

{
    let mut uVar1: u32;
    let mut local_c: [u8;4] = [0;4];
    let mut uStack8: u16;

    uStack8 = param_2;
    uVar1   = (param_1 + 0x2);
    pass1_1000_49c6(local_c, param_3, *param_1, uVar1, (uVar1 >> 0x10), 0x6, 0xbd6c, &stack0xfffe);
    return;
}

i16 pass1_1020_bd6c(param_1: u32, param_2: u32)

{
    return (param_1 + 0x4) - (param_2 + 0x4);
}

u16 pass1_1020_c3ae(void)

{
    return 0x1;
}

struct Struct20 * pass1_1018_cbda(struct param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xf9;
    u_var4  = 0xc5;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x97);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x73b, 0x2e,
                    str_var1(pu_var2, 0x2af8),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d3d2[39];//0xd46e;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}


Struct20 * pass1_1018_cc28(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut local_6: [u8;4] = [0;4];
    let mut uVar3: u16;
    let mut u_var4: u16;

    uVar3  = 0xfa;
    u_var4  = 0xa3;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x98);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x73c, 0x2f,
                    str_var1(pu_var2, 0x2710),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d3d2[273];//0xd816;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}


Struct20 * pass1_1018_cc76(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xfb;
    u_var4  = 0xa8;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x99);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x73e, 0x73d, 0x30,
                    str_var1(pu_var2, 0x61a8),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d3d2[468]//0xdb22;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}


Struct20 * pass1_1018_ccc4(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xfc;
    u_var4  = 0xa9;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x9b);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x740, 0x73f, 0x31,
                    str_var1(pu_var2, 0x59d8),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d5a6[117];//0xd5a6;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}


Struct20 * pass1_1018_cd12(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xfd;
    u_var4  = 0x7c;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x9c);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x741, 0x32,
                    str_var1(pu_var2, 0x2710),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d3d2[351];//0xd94e;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}


Struct20 * pass1_1018_cd60(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xfe;
    u_var4  = 0xc9;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x0);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x0, 0x33,
                    str_var1(pu_var2, 0x2710),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d3d2;//0xd3d2;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}

Struct20 * pass1_1018_cf74(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xfe;
    u_var4  = 0xcf;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x80);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x0, 0x34,
                    str_var1(pu_var2, 0x2710),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d3d2[234];//0xd77a;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}

u16 switch_1020_c3b4(param_1: u16)

{
    let mut u_stack6: u16;

    u_stack6 = 0x1;
    switch(param_1)
    {
    0x1 =>
    2 =>
     3 =>
    0x5 =>
    0x8 =>
    0x9 =>
    0xa =>
    0xb =>
    0xc =>
        u_stack6 = 0x3;
        break;
    0x4 =>
        u_stack6 = 0x6;
        break;
    0x6 =>
    0xf =>
    0x10 =>
    0x11 =>
    0x12 =>
    0x13 =>
        u_stack6 = 0xa;
        break;
    0x7 =>
        u_stack6 = 0x2;
        break;
    0xd =>
    0xe =>
        u_stack6 = 0x1;
    }
    return u_stack6;
}


u16 pass1_1020_c42e(i16 param_1)

{
    let mut uVar1: u16;

    if(param_1 == 0xf)
    {
        uVar1 = 0x1;
    }
    else
    {
        uVar1 = 0x3;
    }
    return uVar1;
}

void  pass1_1020_c4a8(param_1: u32,param_2: *mut u16, param_3: *mut u32, param_4: i16, param_5: u16, param_6: u16)

{
    let mut uVar1: u32;
    let mut pu_var2: *mut u32;
    let mut uVar3: u16;
    let mut u_var4: u16;

    uVar3 = (param_1 >> 0x10);
    if((param_1 + 0x1c) != 0x0)
    {
        pass1_1020_c6a4(param_1 & 0xffff | uVar3 << 0x10, param_5, param_6);
    }
    uVar1    = (param_1 + 0x18);
    u_var4    = (uVar1 >> 0x10);
    pu_var2   = (uVar1 + param_4 * 0x6);
    *param_3 = *pu_var2;
    *param_2 = (pu_var2 + 0x1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1020_c4f4(param_1: u32, param_2: u16, param_3: u16, param_4: u32, param_5: *mut Struct361, param_6: u16)

{
    let mut paVar1: *mut Struct361;
    let mut u_var2: u16;
    let mut uVar3: u16;

    pass1_1020_c6de(param_1, param_4);
    uVar3 = param_6 | param_5;
    if(uVar3 != 0x0)
    {
        paVar1 = param_5;
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, param_4, (param_4 >> 0x10));
        u_var2              = pass1_1030_6fa0(str_var1(uVar3, paVar1));
        param_5.field_0x4 = (u_var2 * 0x2 + 0x4ea4);
    }
    return;
}


u32  pass1_1020_c538(param_1: u32)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return str_var1((param_1 + 0x12), (param_1 + 0x10));
}


void  pass1_1020_c54a(param_1: u32, param_2: i16, param_3: u16)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if((param_1 + 0x1c) != 0x0)
    {
        pass1_1020_c6a4(param_1 & 0xffff | uVar1 << 0x10, param_2, param_3);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1020_c5b8(param_1: u16, param_2: u16, param_3: i16, param_4: u16)

{
    long       *plVar1;
    let mut u_var2: u32;
    let mut ppcVar3: *mut *mut c_void;
    let mut pu_var4: *mut u32;
    let mut uVar5: u16;
    let mut dx_var1: u16;
    let mut uVar6: u16;
    let mut iVar7: i16;
    let mut uVar8: u16;

    u_var2 = (param_3 + 0xa);
    pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
    uVar5            = pass1_1030_6fa0(str_var1(param_2, param_1));
    (param_3 + -0x6) = uVar5;
    pass1_1020_c6de(*(param_3 + 0x6), 0x0);
    (param_3 + -0xa) = uVar5;
    (param_3 + -0x8) = param_2;
    if((param_2 | uVar5) == 0x0)
    {
        ppcVar3 = ((param_3 + 0x6) + 0x20);
        (**ppcVar3)();
        uVar6 = dx_var1;
        pass1_1020_c6de(*(param_3 + 0x6), 0x0);
        (param_3 + -0xa) = uVar5;
        (param_3 + -0x8) = uVar6;
        if((uVar6 | uVar5) == 0x0)
        {
            return;
        }
    }
    u_var2          = (param_3 + 0x6);
    uVar8          = (u_var2 >> 0x10);
    iVar7          = u_var2;
    (iVar7 + 0x1c) = 0x1;
    plVar1         = (iVar7 + 0x8);
    *plVar1        = *plVar1 + 0x1;
    pu_var4         = (param_3 + -0xa);
    *pu_var4        = (param_3 + 0xa);
    (pu_var4 + 0x4) = ((param_3 + -0x6) * 0x2 + 0x4ea4);
    return;
}


void  pass1_1020_c644(param_1: *mut u32, param_2: u16, param_3: u32)

{
    long  *plVar1;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut pu_stack6: *mut u32;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if((iVar5 + 0x18) == 0x0)
    {
        ppcVar3 = (*param_1 + 0x20);
        (**ppcVar3)();
    }
    iVar4         = (iVar5 + 0x8) * 0x6 + (iVar5 + 0x18);
    u_var2         = (iVar5 + 0x1a);
    pu_stack6      = str_var1(u_var2, iVar4);
    plVar1        = (iVar5 + 0x8);
    *plVar1       = *plVar1 + 0x1;
    *pu_stack6     = param_3;
    (iVar4 + 0x4) = param_2;
    return;
}


void  pass1_1020_c694(param_1: u32, param_2: i16, param_3: u16)

{
    pass1_1020_c6a4(param_1, param_2, param_3);
    return;
}


void  pass1_1020_c6a4(param_1: *mut Struct359, param_2: i16, param_3: u16)

{
    let mut lVar1 = 0i32;
    let mut iVar2: *mut Struct359;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if((iVar2.field_0x18 != 0x0) && (iVar2.field_0x8 != 0x0))
    {
        lVar1 = iVar2.field_0x18;
        pass1_1000_4aea(lVar1, (lVar1 >> 0x10), iVar2.field_0x10, 0x6, 0xc7fa, &stack0xfffe, param_2, u_var2, SEG_1000, param_3);
        iVar2.field_0x1c_addr_base = 0x0;
    }
    return;
}


void  pass1_1020_c6de(param_1: *mut Struct360, long param_2)

{
    let mut puVar1: *mut u32;
    let mut u_var2: u32;
    let mut iVar3: *mut Struct360;
    let mut unaff_DI: i16;
    let mut uVar3: u16;
    let mut ss_var1: u16;
    let mut u_stack6: u32;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    if(iVar3.field_0x1c_addr_base != 0x0)
    {
        pass1_1020_c6a4(param_1 & 0xffff | uVar3 << 0x10, unaff_DI, ss_var1);
    }
    u_stack6 = 0x0;
    while(true)
    {
        puVar1 = &iVar3.field_0x10;
        if(*puVar1 < u_stack6 || *puVar1 == u_stack6)
        {
            return;
        }
        u_var2 = iVar3.field_0x18;
        if((u_var2 + u_stack6 * 0x6) == param_2)
            break;
        u_stack6 = u_stack6 + 0x1;
    }
    return;
}

u16 pass1_1020_a426(void)

{
    let mut puVar1: *mut u16;

    pass1_1008_3e38(&PTR_LOOP_1048_4230);
    puVar1 = pass1_1008_3e38(0x10484236);
    return puVar1;
}

void  pass1_1020_b0aa(param_1: u16, param_2: u16, param_3: i16, param_4: u16)

{
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut iVar3: i16;
    let mut pu_var4: *mut u32;
    let mut dx_var1: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut uStack20: u32;

    uVar7 = (globals._PTR_LOOP_1050_4e74 >> 0x10);
    if((globals._PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) == 0x0)
    {
        return;
    }
    if((globals._PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) != -0x1)
    {
        if(globals.PTR_LOOP_1050_4e78 == 0x0)
        {
            iVar3 = param_3;
            pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, 0x2, 0x400);
            puVar1  = *(iVar3 + 0xc);
            ppcVar2 = (*puVar1 + 0x10);
            pu_var4  = puVar1;
            (**ppcVar2)();
            uVar6 = dx_var1;
            for(uStack20 = 0x0; uStack20 < (pu_var4 & 0xffff | dx_var1 << 0x10); uStack20 = uStack20 + 0x1)
            {
                uVar8 = pass1_1030_1d7c((pu_var4 & 0xffff), uVar6, puVar1);
                uVar5 = (uVar8 >> 0x10);
                uVar6 = uVar5 | uVar8;
                if((uVar6 != 0x0) && ((iVar3 = (uVar8 + 0xc), iVar3 == 0x2a || (iVar3 == 0x2b))))
                {
                    globals.PTR_LOOP_1050_4e78 = (&PTR_LOOP_1050_0000 + 0x1);
                    break;
                }
            }
            if(globals.PTR_LOOP_1050_4e78 == 0x0)
            {
                globals.PTR_LOOP_1050_4e78 = (&PTR_LOOP_1050_0000 + 0x1);
                return;
            }
        }
        iVar3 = (globals._PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) + -0x1;
        pass1_1008_612e(0x0, iVar3, iVar3);
    }
    return;
}

u16  pass1_1020_b1ae(param_1: i16, param_2: u16, param_3: u16, param_4: u16, param_5: u16,param_6: *mut u16, param_7: u32)

{
    let mut puVar1: *mut u32;
    let mut local_14: i16;
    let mut local_12: i16;
    let mut local_10: i16;
    let mut local_e: i16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, param_7, (param_7 >> 0x10));
    iStack6 = param_1;
    uStack4 = param_2;
    puVar1  = pass1_1030_5b5c(param_1, param_2);
    local_c = *puVar1;
    uStack8 = (puVar1 + 0x4);
    pass1_1008_3e94(param_6, str_var1(param_3, &local_10), str_var1(param_3, &local_e));
    pass1_1008_3e94(str_var1(param_3, &local_c),
                    str_var1(param_3, &local_14),
                    str_var1(param_3, &local_12));
    if((((0xb < local_e) && (0xb < local_10)) && (local_e < local_12 + -0xb)) && (local_10 < local_14 + -0xb))
    {
        return 0x1;
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1020_b240(param_1: u16, param_2: u16, param_3: u32, param_4: u32, param_5: u32)

{
    let mut puVar1: *mut u32;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut bStack31: u8;
    let mut local_a: u32;
    let mut u_stack6: u32;

    puVar1 = &local_a;
    uVar6  = (param_5 >> 0x10);
    pass1_1030_64ce(param_1, puVar1, param_2, globals._PTR_LOOP_1050_5740, param_4, (param_5 + 0x4),
                    str_var1(param_1, puVar1));
    u_stack6  = *puVar1;
    uVar5    = (puVar1 + 0x2);
    bStack31 = (u_stack6 >> 0x18);
    u_var2    = bStack31;
    if(bStack31 != 0x0)
    {
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, u_stack6, uVar5);
        uVar7 = struct_op_1030_73a8(str_var1(uVar5, u_var2));
        u_var4 = (uVar7 >> 0x10);
        u_var2 = uVar7;
        uVar5 = u_var4 | u_var2;
        if((uVar5 != 0x0) && (u_var2 = (u_var2 + 0xc), 0x9 < u_var2))
        {
            return;
        }
    }
    uVar3 = pass1_1020_b1ae(u_var2, uVar5, param_1, param_3, (param_3 >> 0x10), param_4, *(param_5 + 0x4));
    if(uVar3 == 0x0)
    {
        return;
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

void  pass1_1020_b2da(param_1: u16, param_2: u16, param_3: u16, param_4: i16,param_5: *mut u16, param_6: u32)

{
    let mut iVar1: i16;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut pu_var4: *mut u8;
    let mut uVar5: u16;
    let mut in_AF: u8;
    let mut puVar6: *mut u16;
    u8       **ppuVar7;
    let mut iStack28: i16;
    let mut local_1a: [u8;6] = [0;6];
    let mut uStack20: u16;
    let mut uStack18: u16;
    let mut piStack16: *mut i16;
    let mut piStack12: *mut i16;
    let mut local_8: u16;
    let mut local_6: u32;

    if(param_4 == 0x0)
    {
        u_var2 = 0x4e6a;
    }
    else
    {
        u_var2 = 0x4e6e;
    }
    piStack12 = str_var1(0x1050, u_var2);
    if(param_4 == 0x0)
    {
        uStack20 = 0x4e68;
    }
    else
    {
        uStack20 = 0x4e6c;
    }
    uStack18  = SUB42(SEG_1050, 0x0);
    piStack16 = str_var1(0x1050, uStack20);
    do
    {
        if(param_4 == 0x0)
        {
            ppuVar7 = &PTR_LOOP_1048_4230;
        }
        else
        {
            ppuVar7 = (u8 **)0x10484236;
        }
        pass1_1008_3eb4(ppuVar7,
                        str_var1(param_1, &local_8),
                        str_var1(param_1, &local_6),
                        str_var1(param_1, &local_6 + 0x2));
        iVar1 = *piStack12;
        if(iVar1 == 0x0)
        {
            local_6 = str_var1(local_6 + *piStack16, local_6- 1);
        }
        else
        {
            if(iVar1 == 0x1)
            {
                local_6 = str_var1(local_6 + -0x1, local_6 + *piStack16);
            }
            else
            {
                if(iVar1 == 0x2)
                {
                    local_6 = str_var1(local_6 - *piStack16, local_6- 1);
                }
            }
        }
        puVar6 = pass1_1008_3e54(
          str_var1(param_1, local_1a), local_8, local_6, (local_6 >> 0x10));
        uVar5  = (puVar6 >> 0x10);
        u_var2  = (param_6 >> 0x10);
        uVar3  = pass1_1020_b1ae(local_1a, uVar5, param_1, param_2, param_3,
                                str_var1(param_1, local_1a), *(param_6 + 0x4));
        if(uVar3 != 0x0)
        {
            pu_var4 = local_1a;
            pass1_1020_b240(param_1, uVar5,
                            str_var1(param_3, param_2),
                            str_var1(param_1, pu_var4), param_6);
            if(pu_var4 != 0x0)
            {
            // LAB_1020_b46e:
                pass1_1008_3e76(param_5, local_8, local_6, (local_6 >> 0x10));
                return;
            }
        }
        iVar1 = *piStack12;
        if(iVar1 == 0x0)
        {
        // LAB_1020_b45e:
            local_6 = local_6 & 0xffff0000 | (local_6 + 0x2);
        }
        else
        {
            if(iVar1 == 0x1)
            {
                local_6 = local_6 & 0xffff | (local_6 + 0x2) << 0x10;
            }
            else
            {
                if(iVar1 == 0x2)
                    //goto LAB_1020_b45e;
            }
        }
        pass1_1008_3e76(str_var1(param_1, local_1a), local_8, local_6, (local_6 >> 0x10));
        uVar3 = pass1_1020_b1ae(local_1a, uVar5, param_1, param_2, param_3,
                                str_var1(param_1, local_1a), *(param_6 + 0x4));
        if(uVar3 != 0x0)
        {
            pu_var4 = local_1a;
            pass1_1020_b240(param_1, uVar5,
                            str_var1(param_3, param_2),
                            str_var1(param_1, pu_var4), param_6);
            if(pu_var4 != 0x0)
                //goto LAB_1020_b46e;
        }
        iStack28 = *piStack12 + 0x1;
        if(0x2 < iStack28)
        {
            iStack28   = 0x0;
            *piStack16 = *piStack16 + 0x1;
        }
        *piStack12 = iStack28;
        pass1_1020_ac6e(param_1, in_AF, str_var1(param_3, param_2), param_4, *piStack16, iStack28);
    } while(true);
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1020_b482(param_1: u16, param_2: u32, param_3: *mut u32, param_4: u32)

{
    let mut puVar1: *mut u8;
    let mut pu_var2: *mut u32;
    let mut uVar3: u32;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut puVar6: *mut u32;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut puVar10: *mut u32;
    let mut iStack46: i16;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u32;
    let mut uStack32: u16;
    let mut lStack30 = 0i32;
    let mut uStack26: u32;
    let mut local_16: [u8;12] = [0;12];
    let mut local_4: [u8;2] = [0;2];

    uVar7 = pass1_1030_bcae(local_4, param_1);
    u_var4 = (uVar7 >> 0x10);
    pass1_1028_dc52(str_var1(param_1, local_16), 0x1, 0x0, 0x400);
    while(true)
    {
        puVar1 = local_16;
        pass1_1028_e4ec(str_var1(param_1, puVar1));
        uStack26 = str_var1(u_var4, puVar1);
        uVar5    = u_var4 | puVar1;
        if(uVar5 == 0x0)
        {
            pass1_1020_b240(param_1, 0x0, param_2, param_3, param_4);
            if(puVar1 != 0x0)
            {
                lStack30 = (param_4 + 0x4);
                local_24 = *param_3;
                uStack32 = (param_3 + 0x4);
                puVar6   = &local_2a;
                pass1_1008_3eb4(str_var1(param_1, &local_24),
                                str_var1(param_1, puVar6),
                                str_var1(param_1, &local_2a + 0x2),
                                str_var1(param_1, &local_26));
                pass1_1008_3e76(
                  str_var1(param_1, &local_24), local_2a, local_2a - 0x1, local_26 - 0x1);
                pu_var2 = &local_24;
                uVar8  = param_2;
                uVar9  = (param_2 >> 0x10);
                pass1_1020_afc4(param_1, puVar6, uVar8, uVar9, str_var1(param_1, pu_var2), lStack30);
                if(pu_var2 != 0x0)
                {
                    pass1_1008_3e76(str_var1(param_1, &local_24), local_2a, (local_2a >> 0x10), local_26 - 0x1);
                    pu_var2 = &local_24;
                    pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                    str_var1(param_1, pu_var2), lStack30);
                    if(pu_var2 != 0x0)
                    {
                        pass1_1008_3e76(str_var1(param_1, &local_24), local_2a, local_2a + 0x1, local_26 - 0x1);
                        pu_var2 = &local_24;
                        pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                        str_var1(param_1, pu_var2), lStack30);
                        if(pu_var2 != 0x0)
                        {
                            pass1_1008_3e76(str_var1(param_1, &local_24), local_2a, local_2a - 0x1, local_26);
                            pu_var2 = &local_24;
                            pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                            str_var1(param_1, pu_var2), lStack30);
                            if(pu_var2 != 0x0)
                            {
                                pass1_1008_3e76(str_var1(param_1, &local_24), local_2a, local_2a + 0x1, local_26);
                                pu_var2 = &local_24;
                                pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                                str_var1(param_1, pu_var2), lStack30);
                                if(pu_var2 != 0x0)
                                {
                                    pass1_1008_3e76(str_var1(param_1, &local_24), local_2a, local_2a + 0x1, local_26 + 0x1);
                                    pu_var2 = &local_24;
                                    pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                                    str_var1(param_1, pu_var2), lStack30);
                                    if(pu_var2 != 0x0)
                                    {
                                        pass1_1008_3e76(str_var1(param_1, &local_24), local_2a, (local_2a >> 0x10), local_26 + 0x1);
                                        pu_var2 = &local_24;
                                        pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                                        str_var1(param_1, pu_var2), lStack30);
                                        if(pu_var2 != 0x0)
                                        {
                                            pass1_1008_3e76(str_var1(param_1, &local_24), local_2a, local_2a - 0x1, local_26 + 0x1);
                                            pu_var2 = &local_24;
                                            pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                                            str_var1(param_1, pu_var2), lStack30);
                                            if(pu_var2 != 0x0)
                                            {
                                                pass1_1008_3e76(
                                                  str_var1(param_1, &local_24), local_2a, local_2a - 0x2, local_26 - 0x2);
                                                pu_var2 = &local_24;
                                                pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                                  str_var1(param_1, pu_var2), lStack30);
                                                if(pu_var2 != 0x0)
                                                {
                                                    pass1_1008_3e76(
                                                      str_var1(param_1, &local_24), local_2a, local_2a + 0x2, local_26 - 0x2);
                                                    pu_var2 = &local_24;
                                                    pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                                      str_var1(param_1, pu_var2), lStack30);
                                                    if(pu_var2 != 0x0)
                                                    {
                                                        pass1_1008_3e76(
                                                          str_var1(param_1, &local_24), local_2a, local_2a - 0x2, local_26 + 0x2);
                                                        pu_var2 = &local_24;
                                                        pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                                          str_var1(param_1, pu_var2), lStack30);
                                                        if(pu_var2 != 0x0)
                                                        {
                                                            pass1_1008_3e76(
                                                              str_var1(param_1, &local_24), local_2a, local_2a + 0x2, local_26 + 0x2);
                                                            pu_var2 = &local_24;
                                                            pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                                              str_var1(param_1, pu_var2), lStack30);
                                                            if(pu_var2 != 0x0)
                                                            {
                                                                pass1_1008_3e76(
                                                                  str_var1(param_1, &local_24), local_2a, local_2a - 0x1, local_26 + 0x2);
                                                                pu_var2 = &local_24;
                                                                pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                                                  str_var1(param_1, pu_var2), lStack30);
                                                                if(pu_var2 != 0x0)
                                                                {
                                                                    pass1_1008_3e76(
                                                                      str_var1(param_1, &local_24), local_2a, local_2a - 0x1, local_26 + 0x3);
                                                                    pu_var2 = &local_24;
                                                                    pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                                                      str_var1(param_1, pu_var2), lStack30);
                                                                    if(pu_var2 != 0x0)
                                                                    {
                                                                        iStack46 = 0x3;
                                                                        while(true)
                                                                        {
                                                                            if(0x9 < iStack46)
                                                                            {
                                                                                return;
                                                                            }
                                                                            pass1_1008_3e76(
                                                                              str_var1(param_1, &local_24), 0x0, local_2a - iStack46, local_26);
                                                                            pu_var2 = &local_24;
                                                                            pass1_1020_afc4(param_1, puVar6, uVar8, uVar9,
                                                                              str_var1(param_1, pu_var2), lStack30);
                                                                            if(pu_var2 == 0x0)
                                                                                break;
                                                                            iStack46 = iStack46 + 0x1;
                                                                        }
                                                                        return;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return;
        }
        uVar3   = (puVar1 + 0x10);
        puVar10 = param_3;
        uVar7   = param_4;
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, uVar3, (uVar3 >> 0x10));
        puVar1 = local_4;
        pass1_1030_bcbc(param_1, puVar1,
                        str_var1(uVar3, param_1),
                        str_var1(puVar10, uVar5), (puVar10 >> 0x10), uVar7);
        if(puVar1 < 0x0)
            break;
        u_var4 = uVar5;
        if(puVar1 < 0x65)
        {
            return;
        }
    }
    return;
}

void  pass1_1020_86d8(param_1: u32)

{
    let mut pi_var1: *mut i16;
    let mut u_var2: u32;
    let mut iVar3: i16;
    let mut u_var4: u16;
    let mut iStack4: i16;

    iStack4 = 0x0;
    while(true)
    {
        u_var4  = (param_1 >> 0x10);
        pi_var1 = (param_1 + 0x6);
        if(*pi_var1 == iStack4 || *pi_var1 < iStack4)
            break;
        u_var2 = (param_1 + 0xc);
        u_var4 = (u_var2 >> 0x10);
        iVar3 = u_var2;
        if((iVar3 + iStack4 * 0x4) != 0x0)
        {
            pass1_1008_5236(*(iVar3 + iStack4 * 0x4));
        }
        iStack4 = iStack4 + 0x1;
    }
    return;
}


void  pass1_1020_8712(param_1: u32, param_2: *mut i16, param_3: *mut Struct76, u16 *param_4)

{
    let mut uVar1: u16;
    let mut u_var2: u32;

    pass1_1008_3f32(param_4, (param_1 & 0xffff0000 | (param_1 + 0x10)));
    u_var2 = pass1_1008_4772(param_3);
    uVar1 = (u_var2 >> 0x10);
    pass1_1008_3e94(param_4, (param_2 & 0xffff0000 | ZEXT24((param_2 + 0x2))), (param_2 & 0xffff | param_2 << 0x10));
    (param_2 + 0x4) = (u_var2 + 0x4) + *param_2;
    (param_2 + 0x6) = (u_var2 + 0x8) + (param_2 + 0x2);
    return;
}

void  pass1_1020_8bae(param_1: *mut u16) {
    param_1.field_0x0 = addr_table_1020_8e92;//0x8e92;
    param_1.field_0x2 = SEG_1020;
    pass1_1020_8556(param_1);
}

void  pass1_1020_8f74(param_1: *mut u16) {
    u32 * puVar1;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut iVar4: *mut Struct593;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    iVar4 =  param_1;
    param_1.field_0x0 = addr_table_1020_9204;//0x9204;
    iVar4.field_0x2 = SEG_1020;
    puVar1 = iVar4.field_0xb4;
    u_var2 = iVar4.field_0xb6;
    if ((u_var2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1020_8556(param_1);
    return;
}

void  pass1_1020_9068(param_1: *mut u32, param_2: *mut u8, param_3: i16, param_4: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u32;
    let mut ppcVar3: *mut *mut c_void;
    let mut u_var4: u32;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut dx_var1: u16;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut iVar9: i16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut iStack10: i16;

    uVar10 = (param_1 >> 0x10);
    iVar8  = param_1;
    u_var4  = (iVar8 + 0x16);
    u_var2  = *(u_var4 + 0xa);
    uVar6  = u_var2;
    pass1_1018_280c(*(iVar8 + 0x16));
    (iVar8 + 0xaa) = uVar6;
    (iVar8 + 0xac) = param_2;
    uVar5          = param_2 | (iVar8 + 0xaa);
    if(uVar5 == 0x0)
    {
        pass1_1018_2862(*(iVar8 + 0x16));
        (iVar8 + 0xaa) = uVar5;
        (iVar8 + 0xac) = param_2;
    }
    if(((iVar8 + 0xac) | (iVar8 + 0xaa)) != 0x0)
    {
        pass1_1020_915a(param_1 & 0xffff | uVar10 << 0x10, param_2, param_3, param_4);
        pass1_1008_4480(u_var2, (param_1 & 0xffff0000 | (iVar8 + 0xae)), (iVar8 + 0xb4), param_4);
        ppcVar3 = (*param_1 + 0x10);
        (**ppcVar3)();
        u_var4 = (iVar8 + 0xaa);
        iVar1 = (u_var4 + 0xa);
        for(iStack10 = 0x0; iStack10 < iVar1; iStack10 = iStack10 + 0x1)
        {
            uVar6 = SEXT24(iStack10);
            empty_1008_8fc4((iVar8 + 0xaa), uVar6);
            uVar5 = uVar6;
            uVar7 = dx_var1 | uVar5;
            if(uVar7 != 0x0)
            {
                pass1_1008_8c4e(uVar6 & 0xffff | dx_var1 << 0x10, u_var2, param_4);
                u_var4                          = (iVar8 + 0xc);
                uVar11                         = (u_var4 >> 0x10);
                iVar9                          = u_var4;
                (iVar9 + iStack10 * 0x4)       = uVar5;
                (iVar9 + iStack10 * 0x4 + 0x2) = uVar7;
            }
        }
    }
    return;
}

void  pass1_1020_75c4(param_1: *mut u16) {
    let mut iVar1: *mut Struct586;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = addr_table_1020_7780;//0x7780;
    iVar1.field_0x2 = SEG_1020;
    iVar1.field_0xe2 = addr_table_1020_7780[39];//0x781c;
    iVar1.field_0xe4 = SEG_1020;
    pass1_1020_808e(param_1);
    return;
}

u16  pass1_1020_79ae(void)

{
    return 0x1;
}

void  pass1_1020_808e(param_1: *mut u16) {
    let mut puVar1: *mut u16;
    let mut u_var2: u16;
    let mut iVar3: *mut Struct574;
    let mut uVar3: u16;
    let mut pu_stack6: *mut u16;

    uVar3 = (param_1 >> 0x10);
    iVar3 =  param_1;
    param_1.field_0x0 = addr_table_1020_82bc;//0x82bc;
    iVar3.field_0x2 = SEG_1020;
    iVar3.field_0xe2 = addr_table_1020_82bc[39];//0x8358;
    iVar3.field_0xe4 = SEG_1020;
    if (param_1 == 0x0) {
        puVar1 = 0x0;
        u_var2 = 0x0;
    } else {
        puVar1 = &iVar3.field_0xe2;
        u_var2 = uVar3;
    }
    pu_stack6 = str_var1(u_var2, puVar1);
    *pu_stack6 = addr_table_1008_380a[36]; // 0x389a
    puVar1[0x1] = SEG_1008;
    pass1_1008_57c4((param_1 & 0xffff0000 | &iVar3.field_0xd2));
    param_1.field_0x0 = addr_table_1008_380a; // 0x380a
    iVar3.field_0x2 = SEG_1008;
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a
    iVar3.field_0x2 = SEG_1008;
    return;
}


void  pass1_1020_8106(param_1: u32)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = ((param_1 + 0x4) + 0x60);
    (**ppcVar1)();
    return;
}

void  pass1_1020_83f8(param_1: u32, param_2: u16)

{
    let mut uVar1: u32;
    let mut u_var2: u32;
    let mut iVar3: i16;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if((iVar3 + 0x4) != 0x0)
    {
        uVar1 = (iVar3 + 0x1c);
        u_var2 = (iVar3 + 0x1c);
        pass1_1008_4480(*(uVar1 + 0xa), (param_1 & 0xffff0000 | (iVar3 + 0x16)), (u_var2 + 0x2a), param_2);
    }
    return;
}

u32  pass1_1020_6498(param_1: u32, param_2: i16)

{
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    if((param_1 + 0x18 + param_2 * 0x4) != 0x0)
    {
        uVar1 = (param_1 + 0x18 + param_2 * 0x4);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return str_var1((iVar2 + 0xa), (iVar2 + 0x8));
    }
    return 0x0;
}


u16  pass1_1020_64d4(param_1: u32, param_2: i16)

{
    let mut uVar1: u32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    if((param_1 + 0x18 + param_2 * 0x4) != 0x0)
    {
        uVar1 = (param_1 + 0x18 + param_2 * 0x4);
        return (uVar1 + 0x4);
    }
    return 0x0;
}
void  pass1_1020_6746(param_1: u32, param_2: i16, i16 param_3)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u32;
    let mut iVar3: i16;
    let mut u_var4: u16;

    if(param_3 != 0x0)
    {
        u_var4 = (param_1 >> 0x10);
        iVar3 = param_1;
        if((iVar3 + 0x18 + param_3 * 0x4) != 0x0)
        {
            u_var2          = (iVar3 + 0x18 + param_3 * 0x4);
            (u_var2 + 0x4)  = param_2;
            (iVar3 + 0x10) = 0x1;
            if(param_2 == 0x0)
            {
                ppcVar1 = ((iVar3 + 0x18 + param_3 * 0x4) + 0x14);
                (**ppcVar1)();
            }
        }
    }
    return;
}

u16  pass1_1020_5d56(param_1: *mut u32, param_2: u32, param_3: *mut u8, param_4: i16, param_5: u16)

{
    fn_ptr_1 *ppcVar1;
    let mut u_var2: u16;
    let mut uVar3: u16;
    i16       local_12[0x2];
    let mut local_e: i16;
    let mut local_c: i16;
    i16       local_a[0x2];
    let mut iStack6: i16;

    iStack6 = (param_2 + 0x2e);
    u_var2   = param_1;
    uVar3   = (param_1 >> 0x10);
    if(iStack6 == 0x47)
    {
        pass1_1020_61c4(u_var2, uVar3,
                        str_var1(param_5, &local_c),
                        str_var1(param_5, local_a), param_3, param_4, param_5);
        if(local_a[0] == 0x0)
            //goto LAB_1020_5d8b;
        if(local_c <= local_a[0])
        {
            return 0x1;
        }
    }
    else
    {
        if(iStack6 != 0x6a)
        {
            return 0x0;
        }
        pass1_1020_61c4(u_var2, uVar3,
                        str_var1(param_5, &local_e),
                        str_var1(param_5, local_12), param_3, param_4, param_5);
        if(local_e <= local_12[0])
        {
        // LAB_1020_5d8b:
            ppcVar1 = (*param_1 + 0x40);
            (**ppcVar1)();
            return 0x1;
        }
    }
    pass1_1038_af40(globals.ptr_1050_5b7c, (u_var2 + 0x8), 0x9, param_3, u_var2, SEG_1038, param_5);
    return 0x1;
}

u16 * pass1_1020_4092(param_1: *mut u16)

{
    let mut iVar1: i16;
    let mut u_var2: u16;

    pass1_1008_3e38(param_1);
    u_var2         = (param_1 >> 0x10);
    iVar1         = param_1;
    (iVar1 + 0x6) = 0x0;
    (iVar1 + 0x8) = 0x0;
    (iVar1 + 0xa) = 0x1;
    (iVar1 + 0xc) = 0x0;
    (iVar1 + 0xe) = 0x0;
    pass1_1008_3e38((param_1 & 0xffff0000 | (iVar1 + 0x10)));
    return param_1;
}

void  pass1_1020_44b0(u32 *param_1)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if((iVar2 + 0xf6) != 0x0)
    {
        ppcVar1 = (*param_1 + 0x98);
        (**ppcVar1)();
        (iVar2 + 0x112) = 0x0;
        ppcVar1         = ((iVar2 + 0xf6) + 0x8);
        (**ppcVar1)();
    }
    return;
}

void  pass1_1020_3c32(param_1: i16, param_2: u16, param_3: u16, param_4: u16)

{
    let mut cVar1: char;
    let mut iVar2: i16;

    if(param_3 == 0xf5)
    {
        iVar2 = 0x1;
    // LAB_1020_3c52:
        pass1_1018_1b02(param_4, *(param_1 + 0xfa), iVar2);
        return;
    }
    if((param_3 < 0xf6) && (cVar1 = param_3, cVar1 != '\0'))
    {
        if(cVar1 == '\x01' || cVar1 == '\x02')
        {
            return;
        }
        if(cVar1 == -0xc)
        {
            iVar2 = 0x0;
            //goto LAB_1020_3c52;
        }
    }
    pass1_1020_3c32(param_1, param_2, param_3, param_4);
    return;
}


void  pass1_1020_3c74(param_1: u16, param_2: u32, param_3: u16, param_4: u16)

{
    pass1_1020_3c8c(
      str_var1(param_2, param_1), str_var1(param_3, (param_2 >> 0x10)), param_4);
    return;
}

void  pass1_1020_1b68(param_1: u32, param_2: u16)

{
    let mut puVar1: *mut u32;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5  = (param_1 >> 0x10);
    iVar4  = param_1;
    puVar1 = (iVar4 + 0x92);
    u_var2  = (iVar4 + 0x94);
    if((u_var2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0x92) = 0x0;
    pass1_1010_4f48(*(iVar4 + 0x8e), param_2);
    (iVar4 + 0x8e) = 0x0;
    return;
}


u16  pass1_1020_1bb6(param_1: u32)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = ((param_1 + 0x92) + 0x8);
    (**ppcVar1)();
    return 0x0;
}

u16  pass1_1020_1da8(param_1: u32, param_2: i16, param_3: u16, param_4: u16)

{
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    uVar1 = (iVar2 + 0x8e);
    if((uVar1 + 0x30) == 0x1)
    {
        return 0x1;
    }
    uVar1 = (iVar2 + 0x8e);
    if(((uVar1 + 0x30) < 0x3) && (pass1_1010_4df0(*(iVar2 + 0x8e), param_3, param_4), param_2 == 0x0))
    {
        return 0x1;
    }
    return 0x0;
}

void  pass1_1020_1f74(param_1: *mut u16, param_2: u16) {
    let mut iVar1: *mut Struct582;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = addr_table_1020_2518;//0x2518;
    iVar1.field_0x2 = SEG_1020;
    pass1_1010_1ea6(iVar1.field_0x6, param_1 & 0xffff | uVar1 << 0x10, param_2);
    param_1.field_0x0 = addr_table_1008_3aa0[4]; // 0x3ab0;
    iVar1.field_0x2 = SEG_1008;
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a
    iVar1.field_0x2 = SEG_1008;
}

void  pass1_1020_2286(param_1: u16, param_2: u16, param_3: *mut i16, i16 param_4)

{
    *param_3 = 0x64 - param_4 >> 0x1;
}

void  pass1_1020_2594(param_1: *mut u16) {
    let mut iVar1: *mut Struct583;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = addr_table_1020_270c;//0x270c;
    iVar1.field_0x2 = SEG_1020;
    iVar1.field_0xe2 = addr_table_1020_270c[39];//0x27a8;
    iVar1.field_0xe4 = SEG_1020;
    pass1_1020_808e(param_1);
    return;
}

void  pass1_1020_2936(void)

{
    pass1_1020_79ae();
    return;
}

void  pass1_1020_2a46(param_1: u16, param_2: u16, i16 param_3)

{
    pass1_1018_0ae8(*(param_1 + 0xf2), 0x1);
    pass1_1008_68c6(param_1, param_2, param_3, SEG_1008);
    return;
}

void  pass1_1020_2a94(param_1: u32, param_2: u32, param_3: u16)

{
    pass1_1018_1662(*(param_1 + 0xf2), param_2, (param_2 >> 0x10), param_3);
    return;
}


void  pass1_1018_e64c(param_1: *mut u16) {
    let mut iVar1: *mut Struct576;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = addr_table_1018_e790;//0xe790;
    iVar1.field_0x2 = SEG_1018;
    iVar1.field_0xe2 = addr_table_1018_e790[39];//0xe82c;
    iVar1.field_0xe4 = SEG_1018;
    pass1_1020_808e(param_1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16  pass1_1018_e678(param_1: u32, param_2: u16, param_3: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut uVar5: u32;

    u_var4 = (param_1 >> 0x10);
    uVar3 = param_1;
    u_var2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
    if(u_var2 != 0x0)
    {
        ppcVar1 = ((uVar3 + 0xee) + 0x8);
        uVar5   = (**ppcVar1)();
        param_2 = (uVar5 >> 0x10);
        u_var2   = uVar5;
    }
    if((uVar3 + 0xea) == 0x0)
    {
        (uVar3 + 0xea) = 0x1;
        uVar5          = pass1_1038_af40(globals.ptr_1050_5b7c, (uVar3 + 0x8), 0x15, param_2, uVar3, SEG_1038, param_3);
        u_var2          = uVar5;
    }
    return u_var2;
}

void  pass1_1018_e9de(param_1: *mut u16) {
    let mut iVar1: *mut Struct578;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = addr_table_1018_ebd0//0xebd0;
    iVar1.field_0x2 = SEG_1018;
    iVar1.field_0xe2 = addr_table_1018_ebd0[39];//0xec6c;
    iVar1.field_0xe4 = SEG_1018;
    pass1_1020_808e(param_1);
}

void  pass1_1020_022c(param_1: *mut u16) {
    u32 * puVar1;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut iVar4: *mut Struct580;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    iVar4 =  param_1;
    param_1.field_0x0 = addr_table_1020_045a;//0x45a;
    iVar4.field_0x2 = SEG_1020;
    puVar1 = iVar4.field_0xe6;
    u_var2 = iVar4.field_0xe8;
    if ((u_var2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1008_57c4((param_1 & 0xffff0000 | &iVar4.field_0xd2));
    param_1.field_0x0 = addr_table_1008_380a; // 0x380a
    iVar4.field_0x2 = SEG_1008;
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a
    iVar4.field_0x2 = SEG_1008;
    return;
}

void  pass1_1020_028c(param_1: u16, param_2: u16, i16 param_3)

{
    pass1_1010_3c9e((param_1 + 0xe2));
    pass1_1008_68c6(param_1, param_2, param_3, SEG_1008);
    return;
}

void  pass1_1020_05d6(param_1: *mut u16, param_2: u16) {
    let mut iVar1: *mut Struct581;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = addr_table_1020_075a;//0x75a;
    iVar1.field_0x2 = SEG_1020;
    if (iVar1.field_0x6 != 0x0) {
        pass1_1010_1ea6(iVar1.field_0x6, param_1 & 0xffff | uVar1 << 0x10, param_2);
    }
    param_1.field_0x0 = addr_table_1008_3aa0[4]; // 0x3ab0;
    iVar1.field_0x2 = SEG_1008;
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a
    iVar1.field_0x2 = SEG_1008;
    return;
}

void  struct_1020_0762(param_1: &mut Struct20, param_2: u32, param_3: *mut u32, param_4: u16, param_5: u32, param_6: u32, param_7: u16)

{
    let mut iVar1: *mut Struct20;
    let mut uVar1: *mut Struct20;
    let mut paVar1: *mut Struct20;
    let mut u_var2: u16;

    paVar1 = param_1;
    u_var2  = (param_1 >> 0x10);
    pass1_1020_01d8(paVar1, u_var2, param_2, (param_2 >> 0x10), param_4, param_5, (param_5 >> 0x10), param_6, param_7);
    paVar1[0x1].field_0xe  = 0x0;
    paVar1[0x1].field_0x10 = *param_3;
    param_1.field_0x0     = addr_table_1020_081a;//0x81a;
    paVar1.field_0x2      = SEG_1020;
    return;
}

u16 * pass1_1018_dcf6(param_1: *mut u16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a
    param_1.field_0x2 = SEG_1008;
    param_1.field_0x0 = addr_table_1018_df3c;//0xdf3c;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}

void  pass1_1018_dd7c(param_1: u16, param_2: u16, param_3: u32, param_4: u32, param_5: u16, param_6: u16)

{
    let mut uVar1: u16;
    let mut u_var2: u32;
    let mut ppcVar3: *mut *mut c_void;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut puVar8: *mut u8;
    let mut uVar9: u16;
    let mut dx_var1: *mut u8;
    let mut puVar10: *mut u8;
    let mut unaff_DI: i16;
    let mut puVar11: *mut u16;
    let mut puVar12: *mut u32;
    let mut iVar13: i16;
    let mut uVar14: u16;
    let mut lStack32 = 0i32;
    let mut uStack20: u16;
    let mut uStack12: u16;

    uVar5  = param_4._3_1_;
    iVar13 = (param_3 >> 0x10);
    if(param_4._3_1_ == 0x0)
    {
        puVar11 = mixed_1010_20ba(globals.data_1050_0ed0, 0x2f, param_6, param_5, unaff_DI);
        puVar8  = (puVar11 >> 0x10);
        if((puVar11 + 0x1e) == 0x0)
        {
            uStack20 = param_4;
            uVar14   = param_4;
        }
        else
        {
            if(param_4 - 0x7 == 0x0)
            {
                uStack20      = 0x6;
                param_4 = param_4 - 0x7;
            }
            else
            {
                if(param_4 - 0x8 == 0x0)
                {
                    uStack20      = 0x7;
                    param_4 = param_4 - 0x8;
                }
                else
                {
                    uStack20      = 0x8;
                    param_4 = param_4 - 0x9;
                }
            }
            uVar14 = 0x6;
        }
        pass1_1010_81f6(SEG_1010, param_6, globals.dat_1050_14cc, 0x0, uVar14);
        uVar5 = puVar8 | param_4;
        if ((uVar5 != 0x0) && (puVar10 = puVar8, mem_op_1000_179c(0x46, puVar8, 0), (puVar10 | uVar5) != 0x0)) {
            pass1_1008_87cc(str_var1(puVar10, uVar5), param_3, iVar13, uStack20,
                            CONCAT13((puVar8 >> 0x8), CONCAT12(puVar8, param_4)), param_4, param_6);
        }
    }
    else
    {
        pass1_1028_e1ec(globals._PTR_LOOP_1050_65e2, param_4, (param_4 >> 0x10));
        puVar12 = struct_op_1030_73a8(str_var1(param_5, uVar5));
        uVar9   = (puVar12 >> 0x10);
        u_var4   = puVar12;
        if((uVar9 | u_var4) != 0x0)
        {
            u_var2    = (uVar5 + 0x2e);
            uStack12 = u_var2;
            if(((uVar5 + 0x30) | uStack12) == 0x0)
            {
                lStack32 = 0x0;
            }
            else
            {
                lStack32 = (uStack12 + 0x200);
            }
            uVar5 = (u_var4 + 0x1c);
            uVar1 = (u_var4 + 0x1e);
            uVar6 = uVar1 | uVar5;
            if(uVar6 != 0x0)
            {
                lStack32 = str_var1(uVar1, uVar5);
                uVar6    = uVar5;
            }
            ppcVar3 = (*puVar12 + 0x14);
            (**ppcVar3)(SEG_1030, u_var4, uVar9);
            puVar8 = dx_var1;
            uVar7  = uVar6;
            pass1_1010_81f6(SEG_1010, param_6, globals.dat_1050_14cc, lStack32, uVar6);
            puVar10 = puVar8;
            uVar14 = uVar7;
            mem_op_1000_179c(0x46, puVar8, 0);
            uVar5 = puVar10 | uVar14;
            if(uVar5 == 0x0)
            {
                uVar14 = 0x0;
                uVar5  = 0x0;
            }
            else
            {
                pass1_1008_87cc(str_var1(puVar10, uVar14), param_3, iVar13, uVar6,
                                str_var1(puVar8, uVar7), param_4, param_6);
            }
            pass1_1008_8bc6(param_6, uVar5, CONCAT13((uVar5 >> 0x8), CONCAT12(uVar5, uVar14)));
        }
    }
    return;
}

void  pass1_1018_e2a0(param_1: *mut u16) {
    let mut iVar1: *mut Struct573;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 =  param_1;
    param_1.field_0x0 = addr_table_1018_e44e;//0xe44e;
    iVar1.field_0x2 = SEG_1018;
    iVar1.field_0xe2 = addr_table_1018_e44e[39];//0xe4ea;
    iVar1.field_0xe4 = SEG_1018;
    pass1_1020_808e(param_1);
}

Struct20 * pass1_1018_c958(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xf1;
    u_var4  = 0x9a;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x8d);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x732, 0x26,
                    str_var1(pu_var2, 0x1f40),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d3d2[546];//0xdc5a;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}


Struct20 * pass1_1018_c9a6(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xf2;
    u_var4  = 0xa0;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x8e);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x733, 0x27,
                    str_var1(pu_var2, 0x1b58),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d3d2[195];//0xd6de;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}


Struct20 * pass1_1018_c9f4(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut pi_var1: *mut i16;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut pu_var4: *mut u16;
    let mut uVar5: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xf3;
    uVar5  = 0xa6;
    pu_var4 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x8f);
    u_var2  = (pu_var4 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x734, 0x28,
                    str_var1(pu_var4, 0x32c8),
                    str_var1(uVar3, u_var2),
                    str_var1(param_2, uVar5), param_3, param_4, u_var2);
    uVar3              = (param_1 >> 0x10);
    param_1.field_0x0 = addr_table_1018_d3d2[429];//0xda86;
    param_1.field_0x2 = SEG_1018;
    pi_var1             = (param_1 + 0x10e);
    *pi_var1            = *pi_var1 + -0x19;
    return param_1;
}


Struct20 * pass1_1018_ca48(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xf4;
    u_var4  = 0xa1;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x90);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x735, 0x29,
                    str_var1(pu_var2, 0x36b0),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d3d2[78];//0xd50a;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}


Struct20 * pass1_1018_ca96(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut pi_var1: *mut i16;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut pu_var4: *mut u16;
    let mut uVar5: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xf5;
    uVar5  = 0xbf;
    pu_var4 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x92);
    u_var2  = (pu_var4 >> 0x10);
    pass1_1018_c402(param_1, 0x737, 0x736, 0x2a,
                    str_var1(pu_var4, 0x6590),
                    str_var1(uVar3, u_var2),
                    str_var1(param_2, uVar5), param_3, param_4, u_var2);
    uVar3              = (param_1 >> 0x10);
    param_1.field_0x0 = addr_table_1018_d3d2[312]//0xd8b2;
    param_1.field_0x2 = SEG_1018;
    pi_var1             = (param_1 + 0x10e);
    *pi_var1            = *pi_var1 + 0x64;
    return param_1;
}


Struct20 * pass1_1018_caea(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xf6;
    u_var4  = 0x93;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x93);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x738, 0x2b,
                    str_var1(pu_var2, 0x2328),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d3d2[507];//0xdbbe;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}


Struct20 * pass1_1018_cb38(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut uVar1: u16;
    let mut pu_var2: *mut u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xf7;
    u_var4  = 0x94;
    pu_var2 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x94);
    uVar1  = (pu_var2 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x739, 0x2c,
                    str_var1(pu_var2, 0x32c8),
                    str_var1(uVar3, uVar1),
                    str_var1(param_2, u_var4), param_3, param_4, uVar1);
    param_1.field_0x0 = addr_table_1018_d3d2[156];//0xd642;
    param_1.field_0x2 = SEG_1018;
    return param_1;
}


Struct20 * pass1_1018_cb86(param_1: &mut Struct20, param_2: u16, param_3: u32, param_4: u16)

{
    let mut pi_var1: *mut i16;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut pu_var4: *mut u16;
    let mut uVar5: u16;
    let mut local_6: [u8;4] = [0;4];

    uVar3  = 0xf8;
    uVar5  = 0xc2;
    pu_var4 = pass1_1008_941a(str_var1(param_4, local_6), 0x1, 0x96);
    u_var2  = (pu_var4 >> 0x10);
    pass1_1018_c402(param_1, 0x0, 0x73a, 0x2d,
                    str_var1(pu_var4, 0x2328),
                    str_var1(uVar3, u_var2),
                    str_var1(param_2, uVar5), param_3, param_4, u_var2);
    uVar3              = (param_1 >> 0x10);
    param_1.field_0x0 = addr_table_1018_d3d2[390];//0xd9ea;
    param_1.field_0x2 = SEG_1018;
    pi_var1             = (param_1 + 0x10e);
    *pi_var1            = *pi_var1 + 0x64;
    return param_1;
}

void  pass1_1018_642e(param_1: u16, param_2: u16, param_3: *mut i16, i16 param_4)

{
    *param_3 = 0x64 - param_4 >> 0x1;
    return;
}

u16  pass1_1018_6768(param_1: u32, param_2: u16, param_3: u16)

{
    let mut ppcVar1: *mut *mut c_void;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut uVar5: u32;

    u_var4 = (param_1 >> 0x10);
    uVar3 = param_1;
    u_var2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
    if(u_var2 != 0x0)
    {
        ppcVar1 = ((uVar3 + 0xee) + 0x8);
        uVar5   = (**ppcVar1)();
        param_2 = (uVar5 >> 0x10);
        u_var2   = uVar5;
    }
    if((uVar3 + 0xea) == 0x0)
    {
        (uVar3 + 0xea) = 0x1;
        uVar5          = pass1_1038_af40(globals.ptr_1050_5b7c, (uVar3 + 0x8), 0x16, param_2, uVar3, SEG_1038, param_3);
        u_var2          = uVar5;
    }
    return u_var2;
}

void  pass1_1018_50ac(param_1: *mut u16, param_2: u16) {
    u32 * puVar1;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field_0x0 = addr_table_1018_56ce[1];//0x56d2;
    (iVar4 + 0x2) = SEG_1018;
    puVar1 = (iVar4 + 0xe);
    u_var2 = (iVar4 + 0x10);
    if ((u_var2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1018_50ea(param_1: u32, param_2: u16, param_3: u32)

{
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u32;
    let mut u_var4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut paStack6: *mut Struct99;

    paStack6 = pass1_1000_07fc(SEG_1000, globals.u32_ptr_1050_68a2);
    uVar6 = (paStack6 >> 0x10);
    u_var4    = paStack6;
    if((uVar6 | u_var4) == 0x0)
    {
        paStack6 = 0x0;
    }
    else
    {
        paStack6->fld0_addr_table = addr_table_1008_380a[36]; // 0x389a
        (u_var4 + 0x2)       = SEG_1008;
        (u_var4 + 0x4)       = 0x0;
        (u_var4 + 0x6)       = 0x0;
        (u_var4 + 0x8)       = 0x0;
        (u_var4 + 0xa)       = 0x0;
        (u_var4 + 0xc)       = 0x0;
        paStack6->fld0_addr_table = addr_table_1018_56ce; // 0x56ce
        (u_var4 + 0x2)       = SEG_1018;
    }
    uVar9         = (paStack6 >> 0x10);
    uVar7         = paStack6;
    (uVar7 + 0xa) = param_2;
    uVar10        = (param_1 >> 0x10);
    iVar8         = param_1;
    uVar3         = (iVar8 + 0xa);
    iVar1         = (uVar3 + 0xc);
    if(iVar1 == 0x1)
    {
        uVar3         = (iVar8 + 0xa);
        uVar5         = (uVar3 + 0xe);
        (uVar7 + 0x4) = uVar5;
    }
    else
    {
        if(iVar1 == 0x5)
        {
            uVar3         = (iVar8 + 0xa);
            uVar5         = (uVar3 + 0xe);
            (uVar7 + 0x6) = uVar5;
        }
        else
        {
            if(iVar1 != 0x6)
            {
                if((uVar9 | uVar7) == 0x0)
                {
                    return;
                }
                ppcVar2 = paStack6;
                (**ppcVar2)();
                return;
            }
            uVar3         = (iVar8 + 0xa);
            uVar5         = (uVar3 + 0xe);
            (uVar7 + 0x8) = uVar5;
        }
    }
    pass1_1030_6c66(param_3, 0x1, paStack6, uVar5, (uVar6 | u_var4), SEG_1030);
    return;
}


void  pass1_1018_51d2(param_1: u32)

{
    let mut puVar1: *mut u32;
    let mut u_var2: u16;
    let mut ppcVar3: *mut *mut c_void;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5  = (param_1 >> 0x10);
    iVar4  = param_1;
    puVar1 = (iVar4 + 0xe);
    u_var2  = (iVar4 + 0x10);
    if((u_var2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0xe) = 0x0;
    return;
}


u32  pass1_1018_5206(param_1: u32, param_2: u32, param_3: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u16;
    let mut iVar3: i16;
    let mut u_var4: u16;
    let mut uVar5: u32;
    let mut local_a: [u8;8] = [0;8];

    u_var4         = (param_1 >> 0x10);
    iVar3         = param_1;
    (iVar3 + 0xa) = 0x0;
    pass1_1008_5784(str_var1(param_3, local_a), *(iVar3 + 0xe));
    do
    {
        uVar5         = pass1_1008_5b12(local_a, param_3);
        u_var2         = (uVar5 >> 0x10);
        (iVar3 + 0xa) = uVar5;
        (iVar3 + 0xc) = u_var2;
        if((u_var2 | (iVar3 + 0xa)) == 0x0)
            break;
        uVar5 = (iVar3 + 0xa);
        iVar1 = pass1_1000_3d7a(*(uVar5 + 0x4), param_2);
    } while(iVar1 != 0x0);
    return str_var1((iVar3 + 0xc), (iVar3 + 0xa));
}


u32  pass1_1018_526a(param_1: u32, param_2: u32, param_3: u16)

{
    let mut iVar1: i16;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if((iVar1 + 0xe) == 0x0)
    {
        pass1_1018_5292(param_1 & 0xffff | u_var2 << 0x10, param_2, param_3);
    }
    return str_var1((iVar1 + 0x10), (iVar1 + 0xe));
}

u16 * pass1_1018_567c(param_1: *mut u16, param_2: u8) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.field_0x0 = addr_table_1008_380a[36]; // 0x389a
    (param_1)[0x1] = SEG_1008;
    if ((param_2 & 0x1) != 0x0) {
        pass1_1000_093a(param_1, uVar1, SEG_1000);
    }
    return param_1;
}

void  pass1_1018_5714(param_1: *mut u16, param_2: u16) {
    param_1.field_0x0 = addr_table_1018_5830;//0x5830;
    param_1.field_0x2 = SEG_1018;
    pass1_1010_1d80(param_1, param_2);
    return;
}


u32  pass1_1018_5732(param_1: u16, param_2: u16, param_3: u32, param_4: u16, param_5: u16, param_6: u16)

{
    let mut uVar1: u32;

    uVar1 = pass1_1030_6d4e(param_3, param_4, param_5, param_6);
    return uVar1;
}


pub fn pass1_1018_5742(param_1: u16, param_3: u16, param_4: u16)

{
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut c_void;
    let mut uVar3: u32;
    let mut bVar4: bool;
    let mut puVar5: *mut u32;
    let mut uVar6: u32;
    let mut dx_var1: u16;
    let mut dx_var1_00: u16;
    let mut uStack16: u32;

    bVar4   = false;
    puVar1  = (param_3 + 0x4);
    ppcVar2 = (*puVar1 + 0x10);
    puVar5  = puVar1;
    (**ppcVar2)();
    uVar3    = puVar5 & 0xffff | dx_var1 << 0x10;
    uStack16 = 0x0;
    do
    {
        if(uVar3 <= uStack16)
        {
        // LAB_1018_579f:
            if(!bVar4)
            {
                if(param_3 != 0x0)
                {
                    ppcVar2 = *param_3;
                    (**ppcVar2)();
                }
                param_3 = 0x0;
            }
            pass1_1030_6d80(param_4, param_3);
            return;
        }
        ppcVar2 = (*puVar1 + 0x4);
        uVar6   = uVar3;
        (**ppcVar2)();
        if((dx_var1_00 | uVar6) != 0x0)
        {
            bVar4 = true;
            //goto LAB_1018_579f;
        }
        uStack16 = uStack16 + 0x1;
    } while(true);
}


void  pass1_1018_57d2(param_1: u32, param_2: u32)

{
    *(param_1 + 0xa) = param_2;
    return;
}
