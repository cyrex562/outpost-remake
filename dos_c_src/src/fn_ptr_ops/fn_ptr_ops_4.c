
Struct18 * pass1_1028_7472(Struct18 *param_1, u8 param_2)

{
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_816e(Struct18 *param_1, u8 param_2)

{
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_8342(Struct18 *param_1, u8 param_2)

{
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_6850(Struct18 *param_1, u8 param_2)

{
    pass1_1028_6186(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_6a7a(Struct18 *param_1, u8 param_2)

{
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_6aa6(Struct18 *param_1, u8 param_2)

{
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_6e24(Struct18 *param_1, u8 param_2)

{
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_6f84(Struct18 *param_1, u8 param_2)

{
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1028_504a(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1028_5128(u16 param_1, u16 param_2, u8 *param_3, i16 param_4, u16 param_5, u8 param_6)

{
    i16       *piVar1;
    u16        extraout_DX;
    i16       *piVar2;
    u16        uVar3;
    u16       *puVar4;
    u16        uVar5;
    u8         local_14e[0x124];
    u32        uStack42;
    u32 uStack38;
    i16        local_22;
    u8         local_20[0x2];
    u8         local_1e[0x2];
    u32        local_1c;
    i16        iStack24;
    u32 uStack22;
    i16       *piStack18;
    u16        uStack16;
    i16        local_e;
    u16        local_c;
    u32        uStack10;
    u16       *puStack6;

    pass1_1028_bd38(CONCAT22(param_2, param_1), param_3, param_5);
    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_5, param_3, param_4);
    uStack16 = (puStack6 >> 0x10);
    uStack10 = *(puStack6 + 0x20);
    puVar4   = &local_c;
    piVar1   = &local_e;
    piVar2   = piVar1;
    uVar3    = param_5;
    uVar5    = param_5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack10, (uStack10 >> 0x10));
    piStack18 = piVar1;
    pass1_1030_5b1c(CONCAT22(uStack16, piVar1), CONCAT22(uVar3, piVar2), CONCAT22(uVar5, puVar4));
    pass1_1028_b58e(CONCAT22(param_2, param_1));
    uStack22 = CONCAT22(extraout_DX, piVar1);
    local_1c = *(piVar1 + 0x6);
    iStack24 = piVar1[0x8];
    pass1_1028_c8ee(param_5, param_1, param_2, 0x1, CONCAT22(param_5, &local_1c));
    pass1_1008_3eb4(CONCAT22(param_5, &local_1c), CONCAT22(param_5, &local_22), CONCAT22(param_5, local_20), CONCAT22(param_5, local_1e));
    if(local_e < local_22)
    {
        pass1_1030_5b3e(CONCAT22(uStack16, piStack18), local_22, local_c);
        pass1_1030_5b1c(CONCAT22(uStack16, piStack18), CONCAT22(param_5, &local_e), CONCAT22(param_5, &local_c));
    }
    uStack38 = (uStack22 + 0x2e);
    uStack42 = *(uStack38 + 0x4);
    struct_op_1028_87f0(param_5, param_6, (astruct_97 *)CONCAT22(param_5, local_14e), 0x0, 0x0, 0x6f, &local_1c, param_5, uStack42, uStack10);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_5, local_14e));
    pass1_1028_ccd0(param_6, param_5, CONCAT22(param_2, param_1), CONCAT22(param_5, &local_1c));
    return;
}


Struct18 * pass1_1028_525a(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


u32  pass1_1028_533c(i16 param_1, u16 param_2)

{
    pass1_1028_b418((param_1 + 0x6));
    if((*(u8 *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}


Struct18 * pass1_1028_5496(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_55a2(Struct18 *param_1, u8 param_2)

{
    pass1_1028_0138(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


u32  pass1_1028_568a(i16 param_1, u16 param_2)

{
    pass1_1028_b418((param_1 + 0x6));
    if((*(u8 *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}


u32  pass1_1028_571c(i16 param_1, u16 param_2)

{
    pass1_1028_b418((param_1 + 0x6));
    if((*(u8 *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}


u32  pass1_1028_57fa(i16 param_1, u16 param_2)

{
    pass1_1028_b418((param_1 + 0x6));
    if((*(u8 *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}


u32  pass1_1028_58dc(i16 param_1, u16 param_2)

{
    pass1_1028_b418((param_1 + 0x6));
    if((*(u8 *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}


u32  pass1_1028_59be(i16 param_1, u16 param_2)

{
    pass1_1028_b418((param_1 + 0x6));
    if((*(u8 *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}


Struct18 * pass1_1028_5bc6(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1028_5d12(u16 param_1, i16 param_2, u16 param_3, u8 param_4)

{
    u32        uVar1;
    u32 uVar2;
    u16        extraout_DX;

    pass1_1028_b58e(*(param_2 + 0x6));
    (param_2 + -0x4)  = param_1;
    (param_2 + -0x2)  = extraout_DX;
    uVar2             = (param_2 + -0x4);
    (param_2 + -0x8)  = (uVar2 + 0x2e);
    uVar2             = (param_2 + -0x8);
    uVar1             = *(uVar2 + 0x4);
    *(param_2 + -0xc) = uVar1;
    pass1_1028_68de((astruct_100 *)CONCAT22(param_3, param_2 + -0x11a), 0x1, uVar1, param_4, param_3);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_3, param_2 + -0x11a));
    (param_2 + -0x11a) = 0x389a;
    (param_2 + -0x118) = 0x1008;
    return;
}


Struct18 * pass1_1028_5d68(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


u32  pass1_1028_5e4e(i16 param_1, u16 param_2)

{
    pass1_1028_b418((param_1 + 0x6));
    if((*(u8 *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}


Struct18 * pass1_1028_602e(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1028_6186(u16 *param_1)

{
    u32  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    astruct_603 *iVar4;
    u16          uVar4;

    uVar4            = (param_1 >> 0x10);
    iVar4            = (astruct_603 *)param_1;
    *param_1         = 0x6876;
    iVar4->field_0x2 = &USHORT_1050_1028;
    puVar1           = iVar4->field_0x20;
    uVar2            = iVar4->field_0x22;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1028_b418(param_1);
    return;
}


void  pass1_1028_61c4(u32 param_1, u32 param_2, u16 param_3)

{
    u32  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    u16          uVar4;
    Struct21  *paVar5;
    u32   uVar6;
    u16          uVar7;
    astruct_315 *iVar7;

    iVar7 = (astruct_315 *)param_1;
    uVar7 = (param_1 >> 0x10);
    pass1_1028_b46e(param_1, param_2, param_3);
    puVar1 = iVar7->field_0x20;
    uVar2  = iVar7->field_0x22;
    uVar4  = uVar2 | puVar1;
    paVar5 = CONCAT22(uVar4, puVar1);
    if(uVar4 != 0x0)
    {
        ppcVar3 = *puVar1;
        paVar5  = (**ppcVar3)();
    }
    mem_op_1000_179c(0xc, (paVar5 >> 0x10), 0x1000);
    if(paVar5 == 0x0)
    {
        uVar6 = 0x0;
    }
    else
    {
        uVar6 = set_struct_1008_574a(paVar5);
    }
    iVar7->field_0x20 = uVar6;
    iVar7->field_0x22 = (uVar6 >> 0x10);
    return;
}


void  pass1_1028_6228(u32 param_1, u16 param_2, i16 param_3, i16 param_4, u16 param_5)

{
    u16        uVar1;
    u16        uVar2;
    code     **ppcVar3;
    i16        iVar4;
    u16        uVar5;
    i16        iVar6;
    u16        uVar7;
    bool       bVar8;
    long       lVar9;
    u8         local_a[0x4];
    u32 uStack6;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pass1_1008_5784(CONCAT22(param_5, local_a), *(iVar6 + 0x20));
    while(true)
    {
        do
        {
            lVar9 = pass1_1008_5b12(local_a, param_5);
            uVar5 = (lVar9 >> 0x10);
            iVar4 = lVar9;
            if(lVar9 == 0x0)
            {
                return;
            }
        } while((iVar4 + 0x6) != param_4);
        uVar1 = (iVar4 + 0xa);
        if((param_3 == 0x0) && (param_2 < uVar1))
            break;
        bVar8   = param_2 < uVar1;
        param_2 = param_2 - uVar1;
        param_3 = param_3 - bVar8;
        ppcVar3 = ((iVar6 + 0x20) + 0xc);
        (**ppcVar3)(0x1008, (iVar6 + 0x20));
        uStack6 = 0x0;
    }
    uVar2         = (iVar4 + 0xc);
    (iVar4 + 0xa) = uVar1 - param_2;
    (iVar4 + 0xc) = -(param_2 * (uVar2 / uVar1) - (iVar4 + 0xc));
    return;
}


Struct18 * pass1_1028_3fde(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


u32  pass1_1028_42ca(i16 param_1, u16 param_2)

{
    pass1_1028_b418((param_1 + 0x6));
    if((*(u8 *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}


Struct18 * pass1_1028_4444(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_4810(Struct18 *param_1, u8 param_2)

{
    pass1_1028_4530(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


u32  pass1_1028_4920(i16 param_1, u16 param_2)

{
    pass1_1028_b418((param_1 + 0x6));
    if((*(u8 *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}


Struct18 * pass1_1028_4af6(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1028_2f18(u16 param_1, i16 param_2, u8 param_3, u32 param_4)

{
    i16        iVar1;
    u32       *puVar2;
    u16        extraout_DX;
    u16        uVar3;
    u16        uVar4;
    u16       *puVar5;
    u8         local_944[0x124];
    u8         local_820[0x124];
    u8         local_6fc[0x124];
    u8         local_5d8[0x124];
    u8         local_4b4[0x124];
    u32        local_390;
    u8         local_38a[0x124];
    u8         local_266[0x124];
    u8         local_142[0x124];
    u32 local_1e;
    u16        local_1a;
    u32        local_18;
    u16        uStack20;
    u32        uStack18;
    u32 uStack14;
    u32 uStack10;
    u32        uStack6;

    uStack6 = pass1_1028_bb24(param_4);
    iVar1   = uStack6;
    pass1_1028_b58e(param_4);
    uStack10 = CONCAT22(extraout_DX, iVar1);
    uStack14 = (iVar1 + 0x2e);
    uStack18 = *(uStack14 + 0x4);
    local_18 = *(iVar1 + 0xc);
    uStack20 = (iVar1 + 0x10);
    pass1_1008_3eb4(CONCAT22(param_1, &local_18), CONCAT22(param_1, &local_1e), CONCAT22(param_1, &local_1e + 0x2), CONCAT22(param_1, &local_1a));
    pass1_1008_3e76(CONCAT22(param_1, &local_18), local_1e, local_1e._2_2_ - 0x1, local_1a - 0x1);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_142), 0x0, 0x0, 0xd, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_1, local_142));
    pass1_1008_3e76(CONCAT22(param_1, &local_18), local_1e, local_1e._2_2_ + 0x1, local_1a + 0x1);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_266), 0x0, 0x0, 0xc, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_1, local_266));
    pass1_1008_3e76(CONCAT22(param_1, &local_18), local_1e, local_1e._2_2_ + 0x1, local_1a - 0x1);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_38a), 0x0, 0x0, 0xe, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_1, local_38a));
    puVar5 = pass1_1008_3e54(CONCAT22(param_1, &local_390), local_1e, local_1e._2_2_ - 0x1, local_1a + 0x1);
    uVar3  = (puVar5 >> 0x10);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_4b4), 0x0, 0x0, 0xb, &local_390, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_1, local_4b4));
    pass1_1008_3e76(CONCAT22(param_1, &local_18), local_1e, local_1e._2_2_ - 0x1, local_1a);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_5d8), 0x0, 0x0, 0x7a, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_1, local_5d8));
    pass1_1008_3e76(CONCAT22(param_1, &local_18), local_1e, (local_1e >> 0x10), local_1a + 0x1);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_6fc), 0x0, 0x0, 0x7a, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_1, local_6fc));
    pass1_1008_3e76(CONCAT22(param_1, &local_18), local_1e, local_1e._2_2_ + 0x1, local_1a);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_820), 0x0, 0x0, 0x7a, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_1, local_820));
    pass1_1008_3e76(CONCAT22(param_1, &local_18), local_1e, (local_1e >> 0x10), local_1a - 0x1);
    struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, local_944), 0x0, 0x0, 0x7a, &local_18, param_1, uStack18, uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_1, local_944));
    puVar2 = &local_390;
    pass1_1030_627e(param_1, puVar2, uVar3, globals->_PTR_LOOP_1050_5740, CONCAT22(param_1, puVar2), uStack6);
    uVar4                      = (uStack14 >> 0x10);
    (uStack14 + 0x10) = puVar2;
    (uStack14 + 0x12)          = uVar3;
    return;
}


Struct18 * pass1_1028_33f6(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_34d0(Struct18 *param_1, u8 param_2)

{
    pass1_1028_0138(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_35e2(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_3718(Struct18 *param_1, u8 param_2)

{
    pass1_1028_388e(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1028_388e(u16 *param_1)

{
    u16         uVar1;
    Struct18 *paVar2;
    i16         iVar3;
    u16         uVar4;

    uVar4         = (param_1 >> 0x10);
    iVar3         = param_1;
    *param_1      = 0x3e2c;
    (iVar3 + 0x2) = &USHORT_1050_1028;
    paVar2        = (iVar3 + 0x28);
    uVar1         = (iVar3 + 0x2a);
    if((uVar1 | paVar2) != 0x0)
    {
        fn_ptr_1020_ba7e((paVar2 & 0xffff | uVar1 << 0x10));
        fn_ptr_1000_17ce(paVar2, 0x1000);
    }
    pass1_1028_b418(param_1);
    return;
}


Struct18 * pass1_1028_3e06(Struct18 *param_1, u8 param_2)

{
    pass1_1028_388e(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1028_199a(u16 param_1, i16 param_2, u8 param_3, u32 param_4)

{
    i16       *piVar1;
    u32        uVar2;
    u8        *extraout_DX;
    u8        *puVar3;
    u16        uVar4;
    i16       *piVar5;
    u16        uVar6;
    u16       *puVar7;
    u16        uVar8;
    u16        local_15e;
    u16        uStack348;
    u32       *puStack50;
    u32        uStack42;
    u16        uStack38;
    i16       *piStack36;
    i16        local_22;
    u16        local_20;
    u32        uStack30;
    u16       *puStack26;
    i16        local_16;
    u32 local_14;
    u32        local_10;
    u16        uStack12;
    u32        uStack10;
    i16        iStack6;
    u8        *puStack4;

    piVar1  = (param_4 + 0x14);
    *piVar1 = *piVar1 + -0x1;
    if(*piVar1 == 0x0)
    {
        pass1_1028_b58e(param_4);
        uStack10 = *(param_2 + 0x2e);
        iStack6  = param_2;
        puStack4 = extraout_DX;
        pass1_1038_5804(uStack10, 0x1, 0x3);
        local_10  = *(iStack6 + 0xc);
        uStack12  = (iStack6 + 0x10);
        puStack50 = &local_10;
        puVar3    = puStack4;
        pass1_1008_3eb4(CONCAT22(param_1, &local_10), CONCAT22(param_1, &local_16), CONCAT22(param_1, &local_14), CONCAT22(param_1, &local_14 + 0x2));
        puStack26 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_1, puVar3, &uStack10);
        uVar2     = *(puStack26 + 0x20);
        puVar7    = &local_20;
        piStack36 = &local_22;
        piVar5    = piStack36;
        uVar6     = param_1;
        uVar8     = param_1;
        uStack30  = uVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, (uVar2 >> 0x10));
        uStack38 = uVar2;
        pass1_1030_5b1c(uVar2 & 0xffff | ZEXT24(piStack36) << 0x10, CONCAT22(uVar6, piVar5), CONCAT22(uVar8, puVar7));
        if(local_22 < local_16 + 0x1)
        {
            pass1_1030_5b3e(CONCAT22(piStack36, uStack38), local_16 + 0x1, local_20);
            pass1_1030_5b1c(CONCAT22(piStack36, uStack38), CONCAT22(param_1, &local_22), CONCAT22(param_1, &local_20));
        }
        uVar4    = (uStack10 >> 0x10);
        uStack42 = *(uStack10 + 0x4);
        struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, &local_15e), 0x0, 0x0, (-(local_16 == 0x0) & 0xffd3) + 0x60, &local_10, param_1, uStack42 & 0xffff | (uStack10 + 0x6) << 0x10, uStack30);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_1, &local_15e));
        local_15e = 0x389a;
        uStack348 = 0x1008;
        pass1_1008_3e76(CONCAT22(param_1, &local_10), local_16 + 0x1, local_14, (local_14 >> 0x10));
        struct_op_1028_87f0(param_1, param_3, (astruct_97 *)CONCAT22(param_1, &local_15e), 0x0, 0x0, 0x60, &local_10, param_1, uStack42, uStack30);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_1, &local_15e));
    }
    return;
}


Struct18 * pass1_1028_1b2e(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1030_dcf4(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_1ec8(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_254c(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1028_2042(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_2626(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_2762(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_2a6c(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_2b4e(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1030_dcf4(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_0ab4(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1028_0b96(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1028_16fe(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1020_e868(Struct18 *param_1, u8 param_2)

{
    pass1_1020_e846(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void  pass1_1020_ee3a(u32 param_1, u16 param_2, i16 param_3, u16 param_4, u8 param_5)

{
    u16        extraout_DX;
    u8         local_13c[0x124];
    u32        uStack24;
    u32 uStack20;
    u32        uStack16;
    u32        local_c;
    u16        uStack8;
    i16        iStack6;
    u16        uStack4;

    pass1_1028_b58e(param_1);
    local_c  = *(param_3 + 0xc);
    uStack8  = (param_3 + 0x10);
    iStack6  = param_3;
    uStack4  = extraout_DX;
    uStack16 = pass1_1028_bb24(param_1);
    uStack20 = (iStack6 + 0x2e);
    uStack24 = *(uStack20 + 0x4);
    struct_op_1028_87f0(param_4, param_5, (astruct_97 *)CONCAT22(param_4, local_13c), 0x0, 0x1, param_2, &local_c, param_4, uStack24, uStack16);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_4, local_13c));
    return;
}


Struct18 * pass1_1020_eed0(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1030_dcf4(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1020_ef94(Struct18 *param_1, u8 param_2)

{
    pass1_1020_ef5e(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void  pass1_1028_0582(u32 *param_1, u32 *param_2, u16 param_3, u16 param_4, u8 param_5, u16 param_6)

{
    u32       **ppuVar1;
    long       *plVar2;
    u32  uVar3;
    code      **ppcVar4;
    u8         *puVar5;
    u16         uVar6;
    u16         uVar7;
    u32         uVar8;
    u16         extraout_DX;
    u16         uVar9;
    u16         extraout_DX_00;
    u16         extraout_DX_01;
    i16         iVar10;
    i16         iVar11;
    u16         uVar12;
    u16         uVar13;
    u16         uVar14;
    u8          local_138[0x10e];
    u32         local_2a;
    astruct_99 *paStack38;
    astruct_99 *paStack34;
    u32         uStack30;
    u32         uStack18;
    u32         uStack14;
    u8          local_a[0x4];
    u32         uStack6;

    uVar12  = (param_1 >> 0x10);
    iVar10  = param_1;
    uVar8   = *(iVar10 + 0x14);
    uVar13  = (uVar8 >> 0x10);
    iVar11  = uVar8;
    uStack6 = uVar8 & 0xffff0000 | (iVar11 + 0xa4);
    if(((iVar11 + 0xa6) != 0x0) && ((iVar11 + 0xac) != 0x0))
    {
        pass1_1028_081e(param_1, param_2, param_6);
        param_2 = (iVar10 + 0x20);
        ppuVar1 = (u32 **)(uStack6 + 0x8);
        if(*ppuVar1 < param_2 || *ppuVar1 == param_2)
        {
            puVar5  = local_a;
            ppcVar4 = (*param_1 + 0x40);
            (**ppcVar4)(param_3, param_1);
            uVar8   = ZEXT24(puVar5);
            param_6 = extraout_DX;
            if(puVar5 == 0x0)
            {
                if((uStack6 + 0x2) == 0xc)
                {
                    uStack14 = pass1_1028_b4f2(param_1);
                    param_6  = (uStack14 >> 0x10);
                    uVar8    = *(uStack14 + 0x1f6);
                    plVar2   = (long *)(uVar8 + 0x170);
                    *plVar2  = *plVar2 + 0x1;
                    uStack18 = uVar8;
                }
                else
                {
                    uStack18  = globals->PTR_LOOP_1050_68a2;
                    paStack38 = pass1_1000_07fc(0x1000, globals->PTR_LOOP_1050_68a2);
                    uVar9     = (paStack38 >> 0x10);
                    uVar6     = paStack38;
                    if((uVar9 | uVar6) == 0x0)
                    {
                        paStack34 = (astruct_99 *)0x0;
                    }
                    else
                    {
                        paStack38->field_0x0 = 0x389a;
                        (uVar6 + 0x2)        = 0x1008;
                        (uVar6 + 0x4)        = 0x0;
                        (uVar6 + 0x6)        = 0x0;
                        (uVar6 + 0x8)        = 0x0;
                        (uVar6 + 0xa)        = 0x0;
                        (uVar6 + 0xc)        = 0x0;
                        paStack38->field_0x0 = 0x56ce;
                        (uVar6 + 0x2)        = 0x1018;
                        paStack34            = paStack38;
                    }
                    uVar13            = (uStack6 >> 0x10);
                    iVar11            = uStack6;
                    uVar14            = (paStack34 >> 0x10);
                    (paStack34 + 0x6) = (iVar11 + 0x2);
                    (paStack34 + 0xa) = (iVar11 + 0x6);
                    param_3           = 0x1020;
                    uVar7             = switch_1020_c3b4((iVar11 + 0x2));
                    uVar6             = uVar7 * (uStack6 + 0x6);
                    uVar8             = uVar6;
                    (paStack34 + 0xc) = uVar6;
                    uVar3             = (iVar10 + 0x22);
                    ppcVar4           = ((iVar10 + 0x22) + 0x4);
                    (**ppcVar4)(0x1020, uVar3, (uVar3 >> 0x10));
                    param_6 = extraout_DX_00;
                }
            }
            param_2         = uVar8;
            (iVar10 + 0x20) = 0x0;
        }
    }
    uVar13 = (uStack6 >> 0x10);
    if(((uStack6 + 0x4) != 0x0) && ((uStack6 + 0x8) != 0x0))
    {
        pass1_1028_081e(param_1, param_2, param_6);
        param_2 = (iVar10 + 0x20);
        ppuVar1 = (u32 **)(uStack6 + 0x8);
        if(*ppuVar1 < param_2 || *ppuVar1 == param_2)
        {
            param_2 = &local_2a;
            ppcVar4 = (*param_1 + 0x40);
            (**ppcVar4)(param_3, param_1);
            if(param_2 == 0x0)
            {
                uStack18  = globals->PTR_LOOP_1050_68a2;
                paStack38 = pass1_1000_07fc(0x1000, globals->PTR_LOOP_1050_68a2);
                uVar9     = (paStack38 >> 0x10);
                uVar6     = paStack38;
                if((uVar9 | uVar6) == 0x0)
                {
                    paStack34 = (astruct_99 *)0x0;
                }
                else
                {
                    paStack38->field_0x0 = 0x389a;
                    (uVar6 + 0x2)        = 0x1008;
                    (uVar6 + 0x4)        = 0x0;
                    (uVar6 + 0x6)        = 0x0;
                    (uVar6 + 0x8)        = 0x0;
                    (uVar6 + 0xa)        = 0x0;
                    (uVar6 + 0xc)        = 0x0;
                    paStack38->field_0x0 = 0x56ce;
                    (uVar6 + 0x2)        = 0x1018;
                    paStack34            = paStack38;
                }
                uVar13                     = (uStack6 >> 0x10);
                iVar11                     = uStack6;
                uVar14                     = (paStack34 >> 0x10);
                (paStack34 + 0x8)          = (iVar11 + 0x4);
                (paStack34 + 0xa)          = (iVar11 + 0x6);
                uVar7                      = pass1_1020_c42e((iVar11 + 0x4));
                param_2                    = (uVar7 * (uStack6 + 0x6));
                (paStack34 + 0xc) = param_2;
                uVar3                      = (iVar10 + 0x22);
                ppcVar4                    = ((iVar10 + 0x22) + 0x4);
                (**ppcVar4)(0x1020, uVar3, (uVar3 >> 0x10));
            }
            (iVar10 + 0x20) = 0x0;
        }
    }
    if((iVar10 + 0xc) != 0xe)
    {
        pass1_1028_b58e(param_1 & 0xffff | uVar12 << 0x10);
        local_2a  = CONCAT22(extraout_DX_01, param_2);
        paStack34 = (astruct_99 *)(param_2 + 0x2e);
        uStack30  = *(paStack34 + 0x4);
        pass1_1028_68de((astruct_100 *)CONCAT22(param_4, local_138), 0x1, uStack30, param_5, param_4);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_4, local_138));
    }
    return;
}


Struct18 * pass1_1028_08c6(Struct18 *param_1, u8 param_2)

{
    pass1_1028_0138(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_d7d8(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

u32  pass1_1020_d8ca(i16 param_1, u16 param_2)

{
    pass1_1028_b418((param_1 + 0x6));
    if((*(u8 *)(param_1 + 0xa) & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((param_1 + 0x6), 0x1000);
    }
    return CONCAT22((param_1 + 0x8), (param_1 + 0x6));
}

void  pass1_1020_e294(u32 param_1, u16 param_2, u8 param_3)

{
    u32 uVar1;
    u32       *puVar2;
    u32        uVar3;
    u16        extraout_DX;
    u16        uVar4;
    u16        uVar5;
    u16        uVar6;
    char       cStack347;
    u8         local_150[0xc];
    u32       *puStack324;
    u8         local_140[0x124];
    u32        uStack28;
    u32 uStack24;
    u32        uStack20;
    u32        local_10;
    u16        uStack12;
    i16        iStack10;
    u16        uStack8;
    u32        uStack6;

    uVar6 = (param_1 >> 0x10);
    uVar5 = param_1;
    if((0x1 < (uVar5 + 0x24)) && ((uVar5 + 0x24) < 0x6))
    {
        uVar1   = (uVar5 + 0x28);
        uVar3   = *(uVar1 + 0x20);
        uStack6 = uVar3;
        pass1_1028_b58e(param_1);
        iStack10   = uVar3;
        local_10   = *(iStack10 + 0xc);
        uStack12   = (iStack10 + 0x10);
        puStack324 = &local_10;
        uVar4      = extraout_DX;
        uStack8    = extraout_DX;
        pass1_1028_c8ee(param_2, uVar5, uVar6, (uVar5 + 0x24), CONCAT22(param_2, puStack324));
        puVar2 = &local_10;
        pass1_1028_c89c(param_1, CONCAT22(param_2, puVar2), CONCAT22(param_2, local_150), puVar2, param_2);
        uStack20  = *puVar2;
        cStack347 = (uStack20 >> 0x18);
        if((cStack347 == '\0') && (uStack20 == 0x9))
        {
            (uVar5 + 0x14) = 0x1;
        }
        uStack24 = (iStack10 + 0x2e);
        uStack28 = *(uStack24 + 0x4);
        struct_op_1028_87f0(param_2, param_3, (astruct_97 *)CONCAT22(param_2, local_140), 0x0, (uVar5 + 0x14) + 0x1, 0x79, &local_10, param_2, uStack28, uStack6);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_2, local_140));
    }
    (uVar5 + 0x26) = 0x1;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1020_e39c(u32 param_1, u16 param_2, i16 param_3, u16 param_4, u8 param_5)

{
    u32 uVar1;
    u16        uVar2;
    u16        extraout_DX;
    u8         local_13c[0x124];
    u32        uStack24;
    u32 uStack20;
    u32        uStack16;
    u32        local_c;
    i16        iStack8;
    u32 uStack6;

    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(extraout_DX, param_3);
    local_c = *(param_3 + 0xc);
    iStack8 = (param_3 + 0x10);
    if(iStack8 < 0x1)
    {
        uVar2 = 0x5;
    }
    else
    {
        uVar2 = 0x6;
    }
    (param_3 + 0x14) = uVar2;
    uVar1            = (param_1 + 0x28);
    uStack16         = *(uVar1 + 0x20);
    uStack20         = (param_3 + 0x2e);
    uStack24         = *(uStack20 + 0x4);
    struct_op_1028_87f0(param_4, param_5, (astruct_97 *)CONCAT22(param_4, local_13c), 0x0, 0x1, param_2, &local_c, param_4, uStack24, uStack16);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_4, local_13c));
    return;
}

Struct18 * pass1_1020_e76c(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1030_dcf4(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1020_c80e(Struct18 *param_1, u8 param_2)

{
    pass1_1020_c47a(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_cc56(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_cd58(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_cfde(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_d2ee(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_d518(Struct18 *param_1, u8 param_2)

{
    pass1_1028_b418(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void  pass1_1020_b872(u16 param_1, u8 param_2, u32 param_3, u32 param_4)

{
    u16         uVar1;
    u16         uVar2;
    u16         uVar3;
    u32        *puVar4;
    u8         *puVar5;
    u32 *puVar6;
    u16        *puVar7;
    u16         uVar8;
    u8          local_136[0x124];
    u32         local_12;
    i16         local_c;
    i16         local_a;
    u32  local_8;
    u16         uStack4;

    uVar8   = (param_4 >> 0x10);
    puVar6  = pass1_1030_5b5c(param_4, uVar8);
    local_8 = *puVar6;
    uStack4 = (puVar6 + 0x4);
    pass1_1008_3e94(CONCAT22(param_1, &local_8), CONCAT22(param_1, &local_c), CONCAT22(param_1, &local_a));
    uVar1 = local_a - 0xa;
    pass1_1008_612e(0xa, uVar1, uVar1);
    uVar2 = local_c - 0xa;
    pass1_1008_612e(0xa, uVar2, uVar2);
    puVar7 = pass1_1008_3e54(CONCAT22(param_1, &local_12), 0x0, uVar2, uVar1);
    uVar1  = (puVar7 >> 0x10);
    while(true)
    {
        puVar4 = &local_12;
        pass1_1020_b482(param_1, param_3, CONCAT22(param_1, puVar4), param_4);
        if(puVar4 != 0x0)
            break;
        uVar2 = local_a - 0xa;
        pass1_1008_612e(0xa, uVar2, uVar2);
        uVar3 = local_c - 0xa;
        pass1_1008_612e(0xa, uVar3, uVar3);
        pass1_1008_3e76(CONCAT22(param_1, &local_12), 0x0, uVar3, uVar2);
    }
    struct_op_1028_8888(param_1, param_2, (astruct_100 *)CONCAT22(param_1, local_136), 0x0, 0xa, &local_12, param_1, 0x8000002, 0x0, *(param_4 + 0x4));
    puVar5 = local_136;
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_1, puVar5));
    pass1_1020_b97e(param_1, puVar5, uVar1, param_3, (param_3 >> 0x10), 0x1);
    return;
}
void  fn_ptr_1020_ba7e(u32 *param_1)

{
    fn_ptr_1000_17ce((Struct18 *)*param_1, 0x1000);
    return;
}

void  pass1_1020_bcc4(long *param_1, u16 param_2, u16 param_3)

{
    u16 *puVar1;
    i16  iVar2;
    u16  uVar3;
    i16  iVar4;
    u16  uVar5;
    long lVar6;
    long lStack12;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if((iVar4 + 0x4) == 0x0)
    {
        globals->PTR_LOOP_1050_5f2e = 0x0;
        iVar2                       = (iVar4 + 0x6);
    }
    else
    {
        uVar3                       = (iVar4 + 0x4);
        puVar1                      = (iVar4 + 0x8);
        iVar2                       = uVar3 + *puVar1;
        globals->PTR_LOOP_1050_5f2e = CARRY2(uVar3, *puVar1);
    }
    if(PTR_LOOP_1050_5f2e == 0x0)
    {
        if(*param_1 == 0x0)
        {
            if(_PTR_LOOP_1050_5f2c == 0x0)
            {
                globals->PTR_LOOP_1050_5f2c = mem_op_1000_160a(0x0, 0x1000);
            }
            else
            {
            }
            uVar3 = fn_ptr_op_1000_1708(iVar2 * 0x6, 0x0, 0x1, globals->PTR_LOOP_1050_5f2c, globals->PTR_LOOP_1050_5f2e, 0x1000);
        }
        else
        {
            lVar6                       = pass1_1000_0ed4(0x1000, param_3, 0x1, iVar2 * 0x6, 0x0, *param_1, (*param_1 >> 0x10));
            globals->PTR_LOOP_1050_5f2e = (lVar6 >> 0x10);
            uVar3                       = lVar6;
        }
        lStack12 = CONCAT22(PTR_LOOP_1050_5f2e, uVar3);
        if((PTR_LOOP_1050_5f2e | uVar3) != 0x0)
        {
            (iVar4 + 0x4) = iVar2;
            *param_1      = lStack12;
            pass1_1020_bc72((u16 *)(param_1 & 0xffff | uVar5 << 0x10), param_2, param_3);
        }
    }
    return;
}

void  pass1_1020_c47a(u16 *param_1)

{
    u16 uVar1;

    uVar1           = (param_1 >> 0x10);
    *param_1        = 0xc834;
    (param_1 + 0x2) = 0x1020;
    fn_ptr_1000_17ce((param_1 + 0x18), 0x1000);
    pass1_1030_1d28(param_1);
    return;
}

void  pass1_1020_c73a(u32 param_1, u16 param_2)

{
    u16       *puVar1;
    u16        uVar2;
    u32 uVar3;
    u16        uVar4;
    i16        iVar5;
    u16        uVar6;
    long       lVar7;
    u32 uStack10;
    u32 uStack6;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if((iVar5 + 0x10) == 0x0)
    {
        uVar4                       = (iVar5 + 0xc);
        globals->PTR_LOOP_1050_5f2e = (iVar5 + 0xe);
    }
    else
    {
        uVar2                       = (iVar5 + 0x10);
        puVar1                      = (iVar5 + 0x14);
        uVar4                       = uVar2 + *puVar1;
        globals->PTR_LOOP_1050_5f2e = ((iVar5 + 0x12) + (iVar5 + 0x16) + CARRY2(uVar2, *puVar1));
    }
    uStack6 = CONCAT22(PTR_LOOP_1050_5f2e, uVar4);
    if((iVar5 + 0x18) == 0x0)
    {
        if(_PTR_LOOP_1050_5f2c == 0x0)
        {
            globals->PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e, 0x1000);
        }
        else
        {
        }
        uVar4 = fn_ptr_op_1000_1708(uVar4 * 0x6, 0x0, 0x1, globals->PTR_LOOP_1050_5f2c, globals->PTR_LOOP_1050_5f2e, 0x1000);
    }
    else
    {
        uVar3                       = (iVar5 + 0x18);
        lVar7                       = pass1_1000_0ed4(0x1000, param_2, 0x1, uVar4 * 0x6, (PTR_LOOP_1050_5f2e * 0x3 + CARRY2(uVar4, uVar4) + CARRY2(uVar4 * 0x2, uVar4)) * 0x2 + CARRY2(uVar4 * 0x3, uVar4 * 0x3), uVar3, (uVar3 >> 0x10));
        globals->PTR_LOOP_1050_5f2e = (lVar7 >> 0x10);
        uVar4                       = lVar7;
    }
    uStack10 = CONCAT22(PTR_LOOP_1050_5f2e, uVar4);
    if((PTR_LOOP_1050_5f2e | uVar4) != 0x0)
    {
        (iVar5 + 0x10) = uStack6;
        (iVar5 + 0x18) = uStack10;
    }
    (iVar5 + 0x1c) = 0x1;
    return;
}

Struct18 * pass1_1020_8784(Struct18 *param_1, u8 param_2)

{
    pass1_1020_8556(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_8a5e(Struct18 *param_1, u8 param_2)

{
    pass1_1020_8556(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_8e6c(Struct18 *param_1, u8 param_2)

{
    pass1_1020_8bae(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_91de(Struct18 *param_1, u8 param_2)

{
    pass1_1020_8f74(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_774c(Struct18 *param_1, u8 param_2)

{
    param_1 = (Struct18 *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1020_75c4(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_78dc(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1020_78ac(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_7f38(Struct18 *param_1, u8 param_2)

{
    u16 uVar1;

    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x3ab0;
    (param_1 + 0x2)    = 0x1008;
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1020_8288(Struct18 *param_1, u8 param_2)

{
    param_1 = (Struct18 *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1020_808e(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_843c(Struct18 *param_1, u8 param_2)

{
    pass1_1020_8556(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void  pass1_1020_8556(u16 *param_1)

{
    i16         *piVar1;
    u16          uVar2;
    Struct18  *paVar3;
    astruct_588 *iVar5;
    astruct_589 *iVar4;
    i16          iVar6;
    u16          uVar7;
    u16          uVar8;
    i16          iStack12;

    uVar7            = (param_1 >> 0x10);
    iVar5            = (astruct_588 *)param_1;
    *param_1         = 0x87aa;
    iVar5->field_0x2 = 0x1020;
    fn_ptr_1000_17ce(iVar5->field_0x8, 0x1000);
    if(((&iVar5->field_0xc + 0x2) | &iVar5->field_0xc) != 0x0)
    {
        iStack12 = 0x0;
        while(true)
        {
            piVar1 = &iVar5->field_0x6;
            if(*piVar1 == iStack12 || *piVar1 < iStack12)
                break;
            iVar6  = iStack12 * 0x4;
            paVar3 = iVar5->field_0xc;
            uVar8  = (paVar3 >> 0x10);
            iVar4  = (astruct_589 *)paVar3;
            if((iVar4 + iVar6) != 0x0)
            {
                paVar3 = (iVar4 + iVar6);
                uVar2  = (iVar4 + iVar6 + 0x2);
                if((uVar2 | paVar3) != 0x0)
                {
                    pass1_1008_5118(paVar3 & 0xffff | uVar2 << 0x10);
                    fn_ptr_1000_17ce(paVar3, 0x1000);
                }
            }
            iStack12 = iStack12 + 0x1;
        }
        fn_ptr_1000_17ce(iVar5->field_0xc, 0x1000);
    }
    *param_1         = 0x389a;
    iVar5->field_0x2 = 0x1008;
    return;
}


void  pass1_1020_85f6(u32 param_1)

{
    i16         *piVar1;
    u16          uVar2;
    Struct18  *paVar3;
    u32   uVar4;
    i16          iVar5;
    astruct_590 *iVar6;
    u16          uVar6;
    u16          uVar7;
    i16          iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar7  = (param_1 >> 0x10);
        iVar6  = (astruct_590 *)param_1;
        piVar1 = &iVar6->field_0x6;
        if(*piVar1 == iStack4 || *piVar1 < iStack4)
            break;
        uVar4  = iVar6->field_0xc;
        uVar6  = (uVar4 >> 0x10);
        iVar5  = uVar4;
        paVar3 = (iVar5 + iStack4 * 0x4);
        uVar2  = (iVar5 + iStack4 * 0x4 + 0x2);
        if((uVar2 | paVar3) != 0x0)
        {
            pass1_1008_5118(paVar3 & 0xffff | uVar2 << 0x10);
            fn_ptr_1000_17ce(paVar3, 0x1000);
        }
        uVar4                   = iVar6->field_0xc;
        (uVar4 + iStack4 * 0x4) = 0x0;
        iStack4                 = iStack4 + 0x1;
    }
    return;
}


void  pass1_1020_865a(u32 param_1)

{
    i16         *piVar1;
    u16          uVar2;
    Struct18  *paVar3;
    u32   uVar4;
    i16          iVar5;
    astruct_592 *iVar7;
    astruct_591 *iVar6;
    i16          iVar8;
    u16          uVar9;
    u16          uVar10;
    i16          iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar9  = (param_1 >> 0x10);
        iVar5  = param_1;
        piVar1 = (iVar5 + 0x6);
        if(*piVar1 == iStack4 || *piVar1 < iStack4)
            break;
        iVar8  = iStack4 * 0x4;
        uVar4  = (iVar5 + 0xc);
        uVar10 = (uVar4 >> 0x10);
        iVar7  = (astruct_592 *)uVar4;
        if((iVar7 + iVar8) != 0x0)
        {
            pass1_1008_5236(*(iVar7 + iVar8));
            uVar4  = (iVar5 + 0xc);
            uVar10 = (uVar4 >> 0x10);
            iVar6  = (astruct_591 *)uVar4;
            paVar3 = (iVar6 + iVar8);
            uVar2  = (iVar6 + iVar8 + 0x2);
            if((uVar2 | paVar3) != 0x0)
            {
                pass1_1008_5118(paVar3 & 0xffff | uVar2 << 0x10);
                fn_ptr_1000_17ce(paVar3, 0x1000);
            }
            uVar4                   = (iVar5 + 0xc);
            (uVar4 + iStack4 * 0x4) = 0x0;
        }
        iStack4 = iStack4 + 0x1;
    }
    return;
}

Struct18 * pass1_1020_6208(Struct18 *param_1, u8 param_2, u16 param_3)

{
    param_1 = (Struct18 *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
    destroy_cursor_1020_42f4(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}
Struct18 * pass1_1020_679c(Struct18 *param_1, u8 param_2, u16 param_3, u16 param_4)

{
    pass1_1020_6466(&param_1->field_0x0, param_3, param_4);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

astruct_3 * pass1_1020_2e24(astruct_3 *param_1, u8 param_2)

{
    u16 unaff_CS;

    pass1_1020_28fc(param_1, unaff_CS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_24f2(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1020_1f74(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_26d8(Struct18 *param_1, u8 param_2)

{
    param_1 = (Struct18 *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1020_2594(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_2868(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1020_2838(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void  pass1_1020_0abc(u32 param_1)

{
    code **ppcVar1;
    u16    uVar2;

    uVar2 = (param_1 >> 0x10);
    if((param_1 + 0xe6) != 0x0)
    {
        ppcVar1 = ((param_1 + 0xe8) + 0x10);
        (**ppcVar1)();
    }
    return;
}

Struct18 * pass1_1020_0d82(Struct18 *param_1, u8 param_2)

{
    u16 uVar1;

    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x3ab0;
    (param_1 + 0x2)    = 0x1008;
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1018_e5aa(Struct18 *param_1, u8 param_2)

{
    pass1_1018_e57a(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1018_e75c(Struct18 *param_1, u8 param_2)

{
    param_1 = (Struct18 *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_e64c(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1018_e8ec(Struct18 *param_1, u8 param_2)

{
    pass1_1018_e8bc(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


Struct18 * pass1_1018_eb9c(Struct18 *param_1, u8 param_2)

{
    param_1 = (Struct18 *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_e9de(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_01a6(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1018_ed98(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_0434(Struct18 *param_1, u8 param_2)

{
    pass1_1020_022c(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_0734(Struct18 *param_1, u8 param_2, u16 param_3)

{
    pass1_1020_05d6(&param_1->field_0x0, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1020_07f4(Struct18 *param_1, u8 param_2)

{
    pass1_1020_022c(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1018_df10(Struct18 *param_1, u8 param_2)

{
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void  pass1_1018_e01c(Struct18 *param_1, u8 param_2)

{
    astruct_572 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_572 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}

Struct18 * pass1_1018_e1ee(Struct18 *param_1, u8 param_2)

{
    u16 uVar1;

    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x3ab0;
    (param_1 + 0x2)    = 0x1008;
    param_1->field_0x0 = 0x389a;
    (param_1 + 0x2)    = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

Struct18 * pass1_1018_e41a(Struct18 *param_1, u8 param_2)

{
    param_1 = (Struct18 *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_e2a0(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

void  pass1_1018_8c46(Struct18 *param_1, u8 param_2)

{
    astruct_548 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_548 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8c8e(Struct18 *param_1, u8 param_2)

{
    astruct_549 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_549 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8cd6(Struct18 *param_1, u8 param_2)

{
    Struct675 *iVar1;
    u16          uVar1;

    iVar1 = (Struct675 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8d1e(Struct18 *param_1, u8 param_2)

{
    astruct_550 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_550 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8d66(Struct18 *param_1, u8 param_2)

{
    astruct_551 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_551 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8dae(Struct18 *param_1, u8 param_2)

{
    astruct_552 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_552 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8df6(Struct18 *param_1, u8 param_2)

{
    astruct_553 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_553 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8e3e(Struct18 *param_1, u8 param_2)

{
    astruct_554 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_554 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8e86(Struct18 *param_1, u8 param_2)

{
    astruct_555 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_555 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8ece(Struct18 *param_1, u8 param_2)

{
    Struct676 *iVar1;
    u16          uVar1;

    iVar1 = (Struct676 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8f16(Struct18 *param_1, u8 param_2)

{
    astruct_556 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_556 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8f5e(Struct18 *param_1, u8 param_2)

{
    Struct677 *iVar1;
    u16          uVar1;

    iVar1 = (Struct677 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8fa6(Struct18 *param_1, u8 param_2)

{
    astruct_557 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_557 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_8fee(Struct18 *param_1, u8 param_2)

{
    astruct_558 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_558 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_9036(Struct18 *param_1, u8 param_2)

{
    Struct559 *iVar1;
    u16          uVar1;

    iVar1 = (Struct559 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_907e(Struct18 *param_1, u8 param_2)

{
    astruct_560 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_560 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_90c6(Struct18 *param_1, u8 param_2)

{
    astruct_561 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_561 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_910e(Struct18 *param_1, u8 param_2)

{
    astruct_562 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_562 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_9156(Struct18 *param_1, u8 param_2)

{
    astruct_563 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_563 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_919e(Struct18 *param_1, u8 param_2)

{
    astruct_564 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_564 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_91e6(Struct18 *param_1, u8 param_2)

{
    astruct_565 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_565 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_922e(Struct18 *param_1, u8 param_2)

{
    astruct_566 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_566 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_9276(Struct18 *param_1, u8 param_2)

{
    astruct_567 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_567 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_92be(Struct18 *param_1, u8 param_2)

{
    astruct_568 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_568 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_9306(Struct18 *param_1, u8 param_2)

{
    astruct_569 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_569 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_934e(Struct18 *param_1, u8 param_2)

{
    astruct_570 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_570 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}


void  pass1_1018_9396(Struct18 *param_1, u8 param_2)

{
    astruct_571 *iVar1;
    u16          uVar1;

    iVar1 = (astruct_571 *)param_1;
    iVar1 = iVar1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1              = (param_1 >> 0x10);
    param_1->field_0x0 = 0x380a;
    iVar1->field_0x2   = 0x1008;
    param_1->field_0x0 = 0x389a;
    iVar1->field_0x2   = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return;
}
