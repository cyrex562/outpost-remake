
void __stdcall16far pass1_1028_4aca(ulong param_1, uchar *param_2, int param_3, ushort param_4)

{
    ushort *puVar1;

    if(*(int *)((int)param_1 + 0x20) != 0x0)
    {
        puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_4, param_2, param_3);
        pass1_1010_ed3e((ulong)puVar1);
        pass1_1030_2a2c((ulong)puVar1, (uchar *)((ulong)puVar1 >> 0x10), param_3, param_4);
    }
    return;
}


void __stdcall16far pass1_1028_2e84(ushort param_1, ulong param_2, uchar *param_3, ushort param_4, ushort param_5, uchar param_6)

{
    uchar      *puVar1;
    astruct_67 *paVar2;
    ushort     *puVar3;
    undefined2  uVar4;
    undefined2  uVar5;
    int         iVar6;
    ushort      uVar7;
    undefined2  uVar8;
    undefined2  uVar9;
    int         iVar10;

    pass1_1028_09b8(CONCAT22((int)param_2, param_1));
    if((int)(param_2 >> 0x10) == 0x0)
    {
        uVar9  = 0x0;
        iVar10 = 0x8;
        uVar7  = 0x1;
        uVar8  = 0x0;
        uVar5  = 0x0;
        iVar6  = 0x0;
        uVar4  = 0x0;
        paVar2 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_5, param_3, param_4);
        puVar1 = (uchar *)((ulong)paVar2 >> 0x10);
        post_win_msg_1008_a0e4(paVar2, CONCAT22(uVar5, uVar4), iVar6, uVar7, CONCAT22(uVar9, uVar8), iVar10, 0x1008, param_5);
        uVar5  = 0x400;
        iVar6  = 0x3;
        uVar4  = 0x1;
        puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_5, puVar1, param_4);
        puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
        pass1_1010_043a((ulong)puVar3, CONCAT22(uVar5, uVar4), iVar6, param_5);
        pass1_1010_043a((ulong)puVar3, 0x4000001, 0x4, param_5);
        puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_5, puVar1, param_4);
        puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
        pass1_1010_ec84((ulong)puVar3, param_5, param_6);
        puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x8, param_5, puVar1, param_4);
        pass1_1010_9766((ulong)puVar3, (ushort)((ulong)puVar3 >> 0x10), param_4, param_5);
    }
    return;
}


void __stdcall16far struct_1028_37a6(ushort *param_1, uchar *param_2, ushort param_3, ushort param_4)

{
    uint         uVar1;
    uint         uVar2;
    astruct_189 *iVar3;
    undefined2   uVar3;

    struct_1028_b354(param_1);
    uVar3                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar3                             = (astruct_189 *)param_1;
    uVar1                             = 0x0;
    iVar3->field_0x20                 = 0x0;
    iVar3->field_0x24                 = 0x0;
    *(undefined4 *)&iVar3->field_0x28 = 0x0;
    *param_1                          = 0x3e2c;
    iVar3->field_0x2                  = (int)&USHORT_1050_1028;
    mem_op_1000_179c(0xa, param_2, 0x1000);
    uVar2 = (uint)param_2 | uVar1;
    if(uVar2 == 0x0)
    {
        *(undefined4 *)&iVar3->field_0x28 = 0x0;
    }
    else
    {
        pass1_1020_ba3e((long *)CONCAT22(param_2, uVar1), 0x5, 0x5, param_4, param_3);
        iVar3->field_0x28 = uVar1;
        iVar3->field_0x2a = uVar2;
    }
    return;
}


void __stdcall16far pass1_1028_3816(int param_1, ushort param_2, int param_3, ulong param_4, uchar *param_5, ushort param_6, ushort param_7)

{
    uint uVar1;
    uint uVar2;

    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, (ushort)param_5);
    uVar1                                     = 0x0;
    *(undefined4 *)(param_1 + 0x20)           = 0x0;
    *(undefined4 *)(param_1 + 0x24)           = 0x0;
    *(undefined4 *)(param_1 + 0x28)           = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x3e2c;
    *(undefined2 *)(param_1 + 0x2)            = (int)&USHORT_1050_1028;
    mem_op_1000_179c(0xa, param_5, 0x1000);
    uVar2 = (uint)param_5 | uVar1;
    if(uVar2 == 0x0)
    {
        *(undefined4 *)(param_1 + 0x28) = 0x0;
    }
    else
    {
        pass1_1020_ba3e((long *)CONCAT22(param_5, uVar1), 0x5, 0x5, param_7, param_6);
        *(uint *)(param_1 + 0x28) = uVar1;
        *(uint *)(param_1 + 0x2a) = uVar2;
    }
    return;
}


void __stdcall16far struct_1028_1f56(ushort *param_1, uchar *param_2)

{
    undefined4   uVar1;
    uint         uVar2;
    undefined2   extraout_DX;
    astruct_186 *iVar3;
    undefined2   uVar3;

    struct_1028_b354(param_1);
    uVar3                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar3                             = (astruct_186 *)param_1;
    uVar2                             = 0x0;
    *(undefined4 *)&iVar3->field_0x20 = 0x0;
    iVar3->field_0x24                 = 0x0;
    *param_1                          = 0x2572;
    iVar3->field_0x2                  = (int)&USHORT_1050_1028;
    mem_op_1000_179c(0xc, param_2, 0x1000);
    if(((uint)param_2 | uVar2) == 0x0)
    {
        *(undefined4 *)&iVar3->field_0x20 = 0x0;
    }
    else
    {
        set_struct_1008_574a((astruct_21 *)CONCAT22(param_2, uVar2));
        iVar3->field_0x20 = uVar2;
        iVar3->field_0x22 = extraout_DX;
    }
    uVar1                             = *(undefined4 *)&iVar3->field_0x20;
    *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
    return;
}


void __stdcall16far pass1_1028_1fc8(int param_1, ushort param_2, int param_3, ulong param_4, uchar *param_5)

{
    undefined4 uVar1;
    uint       uVar2;
    undefined2 extraout_DX;

    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, (ushort)param_5);
    uVar2                                     = 0x0;
    *(undefined4 *)(param_1 + 0x20)           = 0x0;
    *(undefined2 *)(param_1 + 0x24)           = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x2572;
    *(undefined2 *)(param_1 + 0x2)            = (int)&USHORT_1050_1028;
    mem_op_1000_179c(0xc, param_5, 0x1000);
    if(((uint)param_5 | uVar2) == 0x0)
    {
        *(undefined4 *)(param_1 + 0x20) = 0x0;
    }
    else
    {
        set_struct_1008_574a((astruct_21 *)CONCAT22(param_5, uVar2));
        *(uint *)(param_1 + 0x20)       = uVar2;
        *(undefined2 *)(param_1 + 0x22) = extraout_DX;
    }
    uVar1                             = *(undefined4 *)(param_1 + 0x20);
    *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
    return;
}


void __stdcall16far pass1_1028_1824(ulong param_1, ulong *param_2, ulong param_3, ulong param_4, int param_5, ushort param_6, int param_7, ushort param_8)

{
    BOOL16  BVar1;
    ulong  *puVar2;
    uint    uVar3;
    ushort  uVar4;
    uchar  *puVar5;
    ushort  uVar6;
    ulong   uVar7;
    ushort  uVar8;
    ushort  uVar9;
    ulong   local_2a;
    int     iStack38;
    int     iStack36;
    uint    uStack34;
    uint    uStack32;
    uchar  *puStack30;
    ushort  uStack28;
    int     iStack24;
    ushort *puStack22;
    ushort  uStack16;
    uint    uStack14;
    ulong   local_c;
    int     iStack8;
    ulong   uStack6;

    uVar8 = (ushort)param_1;
    uVar9 = (ushort)(param_1 >> 0x10);
    pass1_1028_c3aa(uVar8, uVar9, (ushort *)param_2, param_3, param_4, param_8);
    if(param_5 == 0x0)
    {
        return;
    }
    BVar1 = pass1_1028_c314(param_8, param_5, param_6, uVar8, uVar9, (ushort *)param_2, (ushort)param_3, (ushort)(param_3 >> 0x10), param_4);
    if(BVar1 == 0x0)
    {
        return;
    }
    puVar2 = &local_c;
    pass1_1030_64ce(param_8, puVar2, param_6, _PTR_LOOP_1050_5740, (ushort *)param_2, param_4, (ulong *)CONCAT22(param_8, puVar2));
    uStack6   = *puVar2;
    puVar5    = *(uchar **)((int)puVar2 + 0x2);
    uVar6     = (ushort)((ulong)param_2 >> 0x10);
    iStack8   = *(int *)((int)param_2 + 0x4);
    puStack22 = (ushort *)(uStack6 & 0xffff | ZEXT24(puVar5) << 0x10);
    uStack32  = (ushort)uStack6;
    uVar3     = (uint)puVar5 >> 0x8;
    if((char)((uint)puVar5 >> 0x8) != '\0')
    {
        puStack30 = puVar5;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack6, (uint)puVar5);
        uStack32  = uVar3;
        puStack30 = puVar5;
        uStack28  = pass1_1030_6fa0(CONCAT22(puVar5, uVar3));
        if(uStack28 == 0x10)
        {
            if(iStack8 != 0x0)
            {
                PTR_LOOP_1050_50ca = (undefined *)0x6ab;
                return;
            }
            return;
        }
        if((uStack28 == 0x60) || (uStack28 == 0x61))
        {
            puStack22 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_8, puVar5, param_7);
            uVar7     = pass1_1018_04b8((ulong)puStack22);
            uStack34  = (uint)(uVar7 >> 0x10);
            uStack16  = (ushort)uVar7;
            iStack36  = *(int *)((int)puStack22 + 0x1e);
            iStack24  = iStack36;
            uStack14  = uStack34;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack16, uStack34);
            uVar4 = pass1_1030_2fac(CONCAT22(uStack34, iStack36));
            if((int)uVar4 <= iStack24)
            {
                PTR_LOOP_1050_50ca = (undefined *)0x6ac;
                return;
            }
            local_2a = *param_2;
            iStack38 = iStack8 + 0x1;
            puVar2   = &local_2a;
            pass1_1028_c7b6(param_8, uVar6, uVar8, uVar9, (ushort *)CONCAT22(param_8, puVar2), param_4);
            if(puVar2 == (ulong *)0x0)
            {
                return;
            }
            if(puVar2 == (ulong *)((int)&DAT_1050_0004 + 0x2))
            {
                return;
            }
            return;
        }
    }
    PTR_LOOP_1050_50ca = (undefined *)0x6aa;
    return;
}


void __stdcall16far pass1_1020_ea20(ulong param_1, ushort *param_2, ulong param_3, ulong param_4, uint param_5, ushort param_6, uchar param_7, ushort param_8)

{
    uint        uVar1;
    code      **ppcVar2;
    ushort      uVar3;
    char        cVar4;
    ulong      *puVar5;
    ushort      uVar6;
    uchar      *puVar7;
    uchar      *extraout_DX;
    int         unaff_DI;
    undefined2  uVar8;
    ushort      uVar9;
    undefined   local_146[0x10c];
    uint        uStack58;
    uchar      *puStack56;
    ulong       uStack50;
    ushort     *puStack46;
    undefined4 *puStack42;
    undefined4  uStack38;
    undefined4  uStack34;
    ulong       uStack28;
    ulong       local_12;
    int         iStack14;
    undefined4 *puStack12;
    undefined4  uStack8;
    BOOL16      BStack4;

    uVar9 = (ushort)param_1;
    uVar3 = (ushort)(param_1 >> 0x10);
    pass1_1028_c3aa(uVar9, uVar3, param_2, param_3, param_4, param_6);
    if(param_5 == 0x0)
    {
        return;
    }
    pass1_1028_c23e(uVar9, uVar3, param_2, param_3, param_4, param_5, param_8, param_6);
    if(param_5 == 0x0)
    {
        return;
    }
    BStack4 = pass1_1028_c314(param_6, param_5, param_8, uVar9, uVar3, param_2, (ushort)param_3, (ushort)(param_3 >> 0x10), param_4);
    if(BStack4 == 0x0)
    {
        return;
    }
    pass1_1028_c7b6(param_6, param_8, uVar9, uVar3, param_2, param_4);
    if((((BStack4 == 0x5) || (BStack4 == 0x4)) || (BStack4 == 0x6)) || (BStack4 == 0x9))
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6a8;
        return;
    }
    if(BStack4 != 0x0)
    {
        return;
    }
    puVar5 = &local_12;
    pass1_1030_64ce(param_6, puVar5, param_8, _PTR_LOOP_1050_5740, param_2, param_4, (ulong *)CONCAT22(param_6, puVar5));
    uStack38       = *puVar5;
    puStack56      = *(uchar **)((int)puVar5 + 0x2);
    uStack38._3_1_ = (byte)(uStack38 >> 0x18);
    uStack58       = (uint)uStack38._3_1_;
    uStack28       = uStack38;
    uStack8        = uStack38;
    if(uStack38._3_1_ == 0x0)
        goto LAB_1020_eb4e;
    uStack8._0_2_ = (ushort)uStack38;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack8, (uint)puStack56);
    uStack38 = CONCAT22(puStack56, uStack58);
    uStack34 = *(undefined4 *)(uStack58 + 0x2e);
    if(*(ulong *)((int)uStack34 + 0x4) != param_3)
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6b7;
        return;
    }
    uStack28  = struct_op_1030_73a8(CONCAT22(puStack56, uStack58));
    puStack56 = (uchar *)(uStack28 >> 0x10);
    uVar1     = *(uint *)((int)uStack28 + 0xc);
    uStack58  = uVar1;
    if(uVar1 != 0x41)
    {
        if(0x41 < (int)uVar1)
        {
            if(uVar1 == 0x6b)
            {
                PTR_LOOP_1050_50ca = (undefined *)0x6b1;
                return;
            }
            if((int)uVar1 < 0x6c)
            {
                if(uVar1 == 0x42)
                {
                    PTR_LOOP_1050_50ca = (undefined *)0x6b1;
                    return;
                }
                uStack58 = uVar1 - 0x4b;
                if(uStack58 == 0x0)
                {
                    PTR_LOOP_1050_50ca = (undefined *)0x6b1;
                    return;
                }
            }
            else
            {
                if(uVar1 == 0x6e)
                {
                    return;
                }
                uStack58 = uVar1 - 0x73;
                if((0x4 < (int)(uVar1 - 0x6e)) && (uStack58 = uVar1 - 0x79, uStack58 == 0x0 || (int)(uVar1 - 0x73) < 0x6))
                {
                    PTR_LOOP_1050_50ca = (undefined *)0x6b0;
                    return;
                }
            }
            goto LAB_1020_eb4e;
        }
        if(uVar1 != 0x3e)
        {
            if(uVar1 < 0x3f)
            {
                cVar4 = (char)uVar1;
                if(cVar4 != '\v')
                {
                    if(cVar4 == '\x10')
                    {
                        return;
                    }
                    uStack58 = uVar1 & 0xff00 | (uint)(byte)(cVar4 - 0x37U);
                    if((byte)(cVar4 - 0x37U) != 0x0)
                        goto LAB_1020_eb4e;
                }
                PTR_LOOP_1050_50ca = (undefined *)0x6b4;
                return;
            }
            goto LAB_1020_eb4e;
        }
    }
    if(*(int *)((int)uStack28 + 0x12) == 0x4)
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6b1;
        return;
    }
LAB_1020_eb4e:
    uVar8 = 0x1000;
    mem_op_1000_179c(0xb4, puStack56, 0x1000);
    puVar7 = (uchar *)((uint)puStack56 | uStack58);
    if(puVar7 == (uchar *)0x0)
    {
        iStack14 = 0x0;
        puVar7   = (uchar *)0x0;
    }
    else
    {
        uVar8    = SUB42(&PTR_LOOP_1050_1040, 0x0);
        iStack14 = string_1040_8520((astruct_57 *)CONCAT13((char)((uint)puStack56 >> 0x8), CONCAT12((char)puStack56, uStack58)), (ushort)PTR_LOOP_1050_0396, 0x24, 0x2, 0x57b, 0x5e8, puVar7, param_6);
    }
    puStack12 = (undefined4 *)CONCAT22(puVar7, iStack14);
    ppcVar2   = (code **)((int)*puStack12 + 0x74);
    (**ppcVar2)(uVar8, iStack14, puVar7);
    if(iStack14 != 0x7)
    {
        puStack46      = (ushort *)uStack8;
        uStack34       = uStack8;
        uStack34._3_1_ = (byte)(uStack8 >> 0x18);
        uVar6          = (ushort)uStack34._3_1_;
        if(uStack34._3_1_ != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack8, (uint)uStack8._2_2_);
            uStack50 = CONCAT22(uStack8._2_2_, uVar6);
            fn_ptr_1030_7296(CONCAT22(uStack8._2_2_, uVar6));
            pass1_1030_730a(uStack50, uVar6, 0x1030, param_6);
            puStack46 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_6, uStack8._2_2_, unaff_DI);
            pass1_1010_ec68((ulong)puStack46, uStack50, param_6);
            puStack42 = (undefined4 *)struct_op_1030_73a8(uStack50);
            puVar5    = (ulong *)puStack42;
            ppcVar2   = (code **)((int)*puStack42 + 0x24);
            (**ppcVar2)(0x1030, (char)puStack42, (int)((ulong)puStack42 >> 0x10));
            uVar6                     = pass1_1028_bc4a((ulong)puStack42, puVar5, extraout_DX, param_6);
            *(ushort *)(uVar9 + 0x24) = uVar6;
            struct_1030_e4fa((astruct_100 *)CONCAT22(param_6, local_146), *(ulong *)((int)uStack50 + 0x16), param_6, param_7);
            fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_6, local_146));
        }
        return;
    }
    PTR_LOOP_1050_50ca = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
    return;
}


void __stdcall16far struct_1028_0068(ushort *param_1, uchar *param_2)

{
    uint         uVar1;
    undefined2   extraout_DX;
    astruct_183 *iVar2;
    undefined2   uVar2;

    struct_1028_b354(param_1);
    uVar2                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                             = (astruct_183 *)param_1;
    uVar1                             = 0x0;
    iVar2->field_0x20                 = 0x0;
    *(undefined4 *)&iVar2->field_0x22 = 0x0;
    *param_1                          = 0x8ec;
    iVar2->field_0x2                  = (int)&USHORT_1050_1028;
    mem_op_1000_179c(0xc, param_2, 0x1000);
    if(((uint)param_2 | uVar1) == 0x0)
    {
        *(undefined4 *)&iVar2->field_0x22 = 0x0;
    }
    else
    {
        set_struct_1008_574a((astruct_21 *)CONCAT22(param_2, uVar1));
        iVar2->field_0x22 = uVar1;
        iVar2->field_0x24 = extraout_DX;
    }
    return;
}


void __stdcall16far pass1_1028_00cc(int param_1, ushort param_2, int param_3, ulong param_4, uchar *param_5)

{
    uint       uVar1;
    undefined2 extraout_DX;

    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, (ushort)param_5);
    uVar1                                     = 0x0;
    *(undefined2 *)(param_1 + 0x20)           = 0x0;
    *(undefined4 *)(param_1 + 0x22)           = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x8ec;
    *(undefined2 *)(param_1 + 0x2)            = (int)&USHORT_1050_1028;
    mem_op_1000_179c(0xc, param_5, 0x1000);
    if(((uint)param_5 | uVar1) == 0x0)
    {
        *(undefined4 *)(param_1 + 0x22) = 0x0;
    }
    else
    {
        set_struct_1008_574a((astruct_21 *)CONCAT22(param_5, uVar1));
        *(uint *)(param_1 + 0x22)       = uVar1;
        *(undefined2 *)(param_1 + 0x24) = extraout_DX;
    }
    return;
}

void __stdcall16far pass1_1028_0176(ulong param_1, ulong param_2, ushort param_3, ushort param_4)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    ulong        uVar4;
    uint         uVar5;
    astruct_21  *paVar6;
    undefined4   uVar7;
    undefined2   uVar8;
    ushort       uVar9;
    astruct_306 *iVar9;
    astruct_298 *iVar8;

    iVar9 = (astruct_306 *)param_1;
    uVar8 = (undefined2)(param_1 >> 0x10);
    pass1_1028_b46e(param_1, param_2, param_4);
    puVar1 = iVar9->field_0x22;
    uVar2  = iVar9->field_0x24;
    uVar5  = uVar2 | (uint)puVar1;
    paVar6 = (astruct_21 *)CONCAT22(uVar5, puVar1);
    if(uVar5 != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        paVar6  = (astruct_21 *)(**ppcVar3)();
    }
    mem_op_1000_179c(0xc, (uchar *)((ulong)paVar6 >> 0x10), 0x1000);
    if(paVar6 == (astruct_21 *)0x0)
    {
        uVar7 = 0x0;
    }
    else
    {
        uVar7 = set_struct_1008_574a(paVar6);
    }
    iVar9->field_0x22 = (undefined4 *)uVar7;
    iVar9->field_0x24 = (uint)((ulong)uVar7 >> 0x10);
    uVar9             = 0x14;
    uVar4             = pass1_1028_b58e(param_1);
    pass1_1030_7f1a(uVar4, uVar9, param_3);
    return;
}

void __stdcall16far struct_1020_d954(ushort *param_1)

{
    uchar       *extraout_DX;
    astruct_182 *iVar1;
    int          unaff_DI;
    undefined2   uVar1;
    ushort       unaff_SS;
    ushort      *puVar2;

    struct_1030_dc96(param_1);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_182 *)param_1;
    iVar1->field_0x24                 = 0x0;
    iVar1->field_0x26                 = 0x0;
    *(undefined4 *)&iVar1->field_0x28 = 0x0;
    *param_1                          = 0xe792;
    iVar1->field_0x2                  = 0x1020;
    puVar2                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, unaff_SS, extraout_DX, unaff_DI);
    iVar1->field_0x28                 = (int)puVar2;
    iVar1->field_0x2a                 = (int)((ulong)puVar2 >> 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort *__stdcall16far struct_1020_d99e(ushort *param_1, ushort param_2, int param_3, ulong param_4, ushort param_5, ushort param_6)

{
    int          unaff_DI;
    ushort      *puVar1;
    ushort       uVar2;
    astruct_178 *iVar2;

    iVar2                             = (astruct_178 *)param_1;
    uVar2                             = (ushort)((ulong)param_1 >> 0x10);
    puVar1                            = pass1_1030_dcc2((int)iVar2, uVar2, param_3, param_4, param_5);
    iVar2->field_0x24                 = param_2;
    iVar2->field_0x26                 = 0x0;
    *(undefined4 *)&iVar2->field_0x28 = 0x0;
    *param_1                          = 0xe792;
    iVar2->field_0x2                  = 0x1020;
    puVar1                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_6, (uchar *)((ulong)puVar1 >> 0x10), unaff_DI);
    iVar2->field_0x28                 = (int)puVar1;
    iVar2->field_0x2a                 = (int)((ulong)puVar1 >> 0x10);
    iVar2->field_0x10                 = 0x49;
    return param_1;
}

void __stdcall16far pass1_1020_cac2(ulong param_1, uchar *param_2, ushort param_3, ushort param_4, ushort param_5)

{
    int        *piVar1;
    code      **ppcVar2;
    undefined  *puVar3;
    undefined  *puVar4;
    uint        uVar5;
    int         iVar6;
    uint        extraout_DX;
    undefined2  extraout_DX_00;
    undefined2  uVar7;
    ushort     *puVar8;
    int         iStack52;
    undefined  *puStack36;
    undefined   local_1c[0x4];
    undefined4  uStack24;
    undefined4 *puStack20;
    undefined4 *puStack16;
    ushort     *puStack12;
    undefined  *puStack8;
    undefined2  uStack6;
    uchar      *puStack4;

    puVar8   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_5, param_2, param_4);
    puStack4 = (uchar *)((ulong)puVar8 >> 0x10);
    uStack6  = SUB42(puVar8, 0x0);
    puStack8 = PTR_LOOP_1050_13ae;
    if(PTR_LOOP_1050_13ae == (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1))
    {
        puStack8 = (undefined *)&PTR_LOOP_1050_0002;
    }
    puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x8, param_5, puStack4, param_4);
    uVar7     = (undefined2)((ulong)puStack12 >> 0x10);
    puStack16 = (undefined4 *)*(ulong *)((int)puStack12 + 0xa);
    puStack20 = (undefined4 *)*(undefined4 *)((int)puStack12 + 0xe);
    pass1_1008_5784((ulong *)CONCAT22(param_5, local_1c), (ulong)puStack16);
    do
    {
        do
        {
            while(true)
            {
                do
                {
                    puVar3 = local_1c;
                    pass1_1008_5b12(puVar3, param_5);
                    if((extraout_DX | (uint)puVar3) == 0x0)
                    {
                        return;
                    }
                    iVar6 = *(int *)(puVar3 + 0x4);
                } while((iVar6 < 0x12) || (SBORROW2(iVar6, 0x12)));
                if(iVar6 != 0x13 && 0x0 < iVar6 + -0x12)
                    break;
                iStack52 = 0x0;
                if(puStack8 == (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1))
                {
                    iStack52 = *(int *)(puVar3 + 0x6) / 0x2;
                }
                else
                {
                    if(puStack8 == (undefined *)&DAT_1050_0004)
                    {
                        iVar6    = *(int *)(puVar3 + 0x6) * 0x3;
                        iStack52 = (int)(iVar6 + (iVar6 >> 0xf & 0x3U)) >> 0x2;
                    }
                }
                piVar1                                = (int *)(puVar3 + 0x6);
                *piVar1                               = *piVar1 - iStack52;
                uVar7                                 = (undefined2)((ulong)puStack16 >> 0x10);
                *(undefined2 *)((int)puStack16 + 0xa) = 0x0;
                ppcVar2                               = (code **)((int)*puStack16 + 0xc);
                (**ppcVar2)(0x1008, (char)puStack16, uVar7, puVar3, extraout_DX);
                *(undefined2 *)((int)puStack16 + 0xa) = 0x1;
                uStack24                              = 0x0;
                ppcVar2                               = (code **)((int)*puStack20 + 0x4);
                (**ppcVar2)(0x1008, (int)puStack20, (int)((ulong)puStack20 >> 0x10), (char)puVar3, extraout_DX);
            }
        } while(iVar6 != 0x81);
        puStack36 = (undefined *)0x0;
        if(puStack8 == (undefined *)&PTR_LOOP_1050_0002)
        {
            iVar6 = *(int *)(puVar3 + 0x6);
        LAB_1020_cba7:
            puVar4    = (undefined *)((int)(iVar6 + (iVar6 >> 0xf & 0x3U)) >> 0x2);
            puStack36 = puVar4;
        }
        else
        {
            if(puStack8 == (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1))
            {
                puVar4    = (undefined *)(*(int *)(puVar3 + 0x6) / 0x2);
                puStack36 = puVar4;
            }
            else
            {
                puVar4 = puStack8 + -0x4;
                if(puVar4 == (undefined *)0x0)
                {
                    iVar6 = *(int *)(puVar3 + 0x6) * 0x3;
                    goto LAB_1020_cba7;
                }
            }
        }
        pass1_1028_b58e(param_1);
        uVar5 = *(int *)(puVar3 + 0x6) - (int)puStack36;
        pass1_1030_7ddc(CONCAT13((char)((uint)extraout_DX_00 >> 0x8), CONCAT12((char)extraout_DX_00, puVar4)), (long)(int)uVar5, 0x1e, uVar5, (uchar *)((int)uVar5 >> 0xf), param_3, param_4, param_5);
        ppcVar2 = (code **)((int)*puStack16 + 0xc);
        (**ppcVar2)(0x1030, (char)puStack16, (int)((ulong)puStack16 >> 0x10), puVar3, extraout_DX);
        uStack24 = 0x0;
    } while(true);
}

void __stdcall16far pass1_1020_ce32(int param_1, ushort param_2, int param_3, ushort param_4)

{
    uchar      *puVar1;
    ulong       uVar2;
    ushort     *puVar3;
    astruct_67 *paVar4;
    undefined2  uVar5;
    undefined2  uVar6;
    int         iVar7;
    ushort      uVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    int         iVar11;

    pass1_1028_09b8(CONCAT22(param_2, param_1));
    uVar2  = pass1_1028_b4f2(CONCAT22(param_2, param_1));
    puVar1 = (uchar *)(uVar2 >> 0x10);
    if(*(long *)((int)uVar2 + 0x200) != 0x8000002)
    {
        puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x8, param_4, puVar1, param_3);
        puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
        pass1_1010_988c((ulong)puVar3, *(int *)(param_1 + 0xc));
        uVar10 = 0x0;
        iVar11 = 0x9;
        uVar8  = 0x1;
        uVar9  = 0x0;
        uVar6  = 0x0;
        iVar7  = 0x0;
        uVar5  = 0x0;
        paVar4 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_4, puVar1, param_3);
        post_win_msg_1008_a0e4(paVar4, CONCAT22(uVar6, uVar5), iVar7, uVar8, CONCAT22(uVar10, uVar9), iVar11, 0x1008, param_4);
    }
    return;
}

void __stdcall16far pass1_1020_ce9e(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    uchar  *puVar1;
    ulong   uVar2;
    ushort *puVar3;

    pass1_1028_be9e((ulong *)param_1, param_4, param_2, (ushort)&USHORT_1050_1028, param_3);
    if(*(int *)((int)param_1 + 0x12) == 0x5)
    {
        pass1_1020_cefc(param_1, param_2, param_3);
        uVar2  = pass1_1028_b4f2(param_1);
        puVar1 = (uchar *)(uVar2 >> 0x10);
        if(*(long *)((int)uVar2 + 0x200) != 0x8000002)
        {
            puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_3, puVar1, param_2);
            pass1_1010_043a((ulong)puVar3, *(long *)((int)uVar2 + 0x4), 0x5, param_3);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_cefc(ulong param_1, int param_2, ushort param_3)

{
    uchar  *puVar1;
    ulong   uVar2;
    ushort *puVar3;
    ushort  uStack12;

    uVar2  = pass1_1028_b4f2(param_1);
    puVar1 = (uchar *)(uVar2 >> 0x10);
    if(*(long *)((int)uVar2 + 0x200) == 0x8000002)
    {
        uStack12 = 0x32;
    }
    else
    {
        puVar3   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x8, param_3, puVar1, param_2);
        uStack12 = pass1_1010_96c2((ulong)puVar3);
        if(0x32 < (int)uStack12)
        {
            uStack12 = 0x32;
        }
        pass1_1010_96a8((ulong)puVar3, uStack12);
    }
    pass1_1020_cf6c((ushort)param_1, (ushort)(param_1 >> 0x10), uStack12, uVar2);
    return;
}

void __stdcall16far pass1_1020_d194(ulong param_1, int param_2, ushort param_3)

{
    code      **ppcVar1;
    undefined2  uVar2;
    undefined  *puVar3;
    ushort      uVar4;
    undefined2  uVar5;
    ulong       uVar6;
    uint        uVar7;
    uchar      *extraout_DX;
    uchar      *puVar8;
    uchar      *puVar9;
    uint        extraout_DX_00;
    uint        uVar10;
    undefined2  uVar11;
    ulong       uVar12;
    ushort     *puVar13;
    ulong      *puVar14;
    undefined   uVar15;
    undefined   uVar16;
    uchar      *puVar17;
    uint        uVar18;
    uint        uVar19;
    ulong       uStack42;
    ulong       uStack38;
    undefined4 *puStack34;
    undefined   local_4[0x2];

    pass1_1030_bcae((ushort)local_4, param_3);
    uVar12 = pass1_1028_b4f2(param_1);
    uVar7  = (uint)(uVar12 >> 0x10);
    uVar6  = *(ulong *)((int)uVar12 + 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar6, (uint)(uVar6 >> 0x10));
    uVar2 = (undefined2)uVar6;
    pass1_1028_b58e(param_1);
    puVar3 = local_4;
    puVar8 = extraout_DX;
    pass1_1030_bd74((ushort)puVar3, param_3, uVar6 & 0xffff | (ulong)uVar7 << 0x10, CONCAT22(extraout_DX, uVar2), param_3);
    if((int)puVar3 < 0x0)
    {
        return;
    }
    if(0x1e < (int)puVar3)
    {
        uVar4   = 0x87;
        puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x9, param_3, puVar8, param_2);
        uVar4   = pass1_1010_65d0(param_3, (ulong)puVar13, uVar4);
        if(uVar4 == 0x0)
        {
            puVar14 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x15);
            puVar9  = (uchar *)((ulong)puVar14 >> 0x10);
            uVar7   = (uint)puVar14;
            uVar11  = SUB42(&PTR_LOOP_1050_1038, 0x0);
            pass1_1038_4e78(uVar7, puVar9, uVar12, puVar14);
            puStack34 = (undefined4 *)CONCAT22(puVar9, uVar7);
            ppcVar1   = (code **)((int)*puStack34 + 0x10);
            uVar10    = uVar7;
            uVar18    = uVar7;
            puVar8    = puVar9;
            (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar7, puVar9);
            uStack38 = CONCAT22(extraout_DX_00, uVar10);
            uStack42 = 0x0;
            uVar10   = extraout_DX_00;
            while(true)
            {
                if(uStack38 <= uStack42)
                {
                    if(puStack34 == (undefined4 *)0x0)
                    {
                        return;
                    }
                    ppcVar1 = (code **)*puStack34;
                    (**ppcVar1)(uVar11, uVar7, (char)puVar9, 0x1, uVar18, puVar8, puStack34, puStack34);
                    return;
                }
                uVar15  = (undefined)uVar2;
                uVar16  = (undefined)((uint)uVar2 >> 0x8);
                uVar6   = uStack38;
                puVar17 = extraout_DX;
                pass1_1030_1d58((ulong)puStack34);
                uVar5  = (undefined2)uVar6;
                puVar3 = local_4;
                uVar11 = 0x1030;
                uVar19 = uVar10;
                pass1_1030_bd74((ushort)puVar3, param_3, uVar6 & 0xffff | (ulong)uVar10 << 0x10, CONCAT22(puVar17, CONCAT11(uVar16, uVar15)), param_3);
                if((0x0 < (int)puVar3) && ((int)puVar3 < 0x1f))
                    break;
                uStack42 = uStack42 + 0x1;
            }
            if(puStack34 == (undefined4 *)0x0)
            {
                return;
            }
            ppcVar1 = (code **)*puStack34;
            (**ppcVar1)(0x1030, uVar7, (char)puVar9, 0x1, uVar18, puVar8, puStack34, puStack34, uVar5, uVar19);
            return;
        }
    }
    return;
}

ushort *__stdcall16far pass1_1020_a43e(ushort param_1, uchar *param_2, ushort *param_3)

{
    int unaff_DI;

    *param_3                            = 0xba36;
    *(undefined2 *)((int)param_3 + 0x2) = 0x1020;
    if(_PTR_LOOP_1050_4e74 != 0x0)
    {
        return param_3;
    }
    mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_1, param_2, unaff_DI);
    if((0x0 < (int)PTR_LOOP_1050_13ae) && (!SBORROW2((int)PTR_LOOP_1050_13ae, 0x1)))
    {
        if(PTR_LOOP_1050_13ae == (undefined *)&PTR_LOOP_1050_0002 || (int)(PTR_LOOP_1050_13ae + -0x1) < 0x1)
        {
            PTR_LOOP_1050_4e74 = (undefined *)0x44b4;
            goto LAB_1020_a482;
        }
        if(PTR_LOOP_1050_13ae == (undefined *)&DAT_1050_0004)
        {
            PTR_LOOP_1050_4e74 = (undefined *)0x4b2c;
            goto LAB_1020_a482;
        }
    }
    PTR_LOOP_1050_4e74 = (undefined *)0x47f0;
LAB_1020_a482:
    _PTR_LOOP_1050_4e74 = CONCAT22(0x1050, PTR_LOOP_1050_4e74);
    return param_3;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_a49a(ushort param_1, uchar param_2, uchar *param_3, ulong param_4, int *param_5, ushort param_6)

{
    ulong      uVar1;
    int        unaff_DI;
    ushort     uVar2;
    undefined2 uVar3;
    undefined  local_136[0x128];
    undefined2 uStack14;
    uint       uStack12;
    ulong      uStack10;
    ushort    *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_1, param_3, unaff_DI);
    uStack12 = (uint)((ulong)puStack6 >> 0x10);
    uVar1    = *(ulong *)((int)puStack6 + 0x20);
    uStack10 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)(uVar1 >> 0x10));
    uStack14 = (undefined2)uVar1;
    if(param_5 != (int *)0x0)
    {
        uVar2 = (ushort)((ulong)param_5 >> 0x10);
        if(*(int *)((ulong *)param_5 + 0x1) == 0x0)
        {
            uVar3 = SUB42(&PTR_LOOP_1050_4230, 0x0);
        }
        else
        {
            uVar3 = 0x4236;
        }
        pass1_1008_3f32(param_5, (int *)CONCAT22(0x1048, uVar3));
        struct_op_1028_87f0(param_1, param_2, (astruct_97 *)CONCAT22(param_1, local_136), 0x0, 0x0, param_6, (ulong *)param_5, uVar2, *(ulong *)((int)_PTR_LOOP_1050_4e70 + 0x4), uStack10);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_136));
        return;
    }
    pass1_1020_abc0(param_1, param_2, param_4, param_6, uVar1 & 0xffff | (ulong)uStack12 << 0x10);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_a54c(ushort param_1, uchar param_2, uchar *param_3, ushort param_4, ushort param_5, int param_6)

{
    undefined4 uVar1;
    int        unaff_DI;
    ushort     uVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    undefined2 uVar5;
    undefined2 uVar6;
    undefined  local_140[0x124];
    ulong     *puStack28;
    int        local_18;
    ushort     local_16;
    ulong      local_14;
    undefined *puStack16;
    undefined2 uStack14;
    undefined2 uStack12;
    undefined4 uStack10;
    ushort    *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_1, param_3, unaff_DI);
    uStack12 = (undefined2)((ulong)puStack6 >> 0x10);
    uVar1    = *(undefined4 *)((int)puStack6 + 0x20);
    uStack10 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    uStack14  = (undefined2)uVar1;
    local_14  = _PTR_LOOP_1048_4230;
    puStack16 = PTR_LOOP_1048_4234;
    puStack28 = &local_14;
    pass1_1008_3e94((ushort *)CONCAT22(param_1, &local_14), (ushort *)CONCAT22(param_1, &local_18), (ushort *)CONCAT22(param_1, &local_16));
    if((param_6 < 0x0) || (0x5 < param_6))
    {
        pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_14), 0x0, local_18 - 0x9, local_16);
        uVar5 = (undefined2)uStack10;
        uVar6 = (undefined2)((ulong)uStack10 >> 0x10);
        uVar1 = *(undefined4 *)((int)_PTR_LOOP_1050_4e70 + 0x4);
        uVar3 = (undefined2)uVar1;
        uVar4 = (undefined2)((ulong)uVar1 >> 0x10);
        uVar2 = 0x14;
    }
    else
    {
        pass1_1008_3e76((ushort *)CONCAT22(param_1, &local_14), 0x0, (local_18 - param_6) - 0x3, local_16);
        uVar5 = (undefined2)uStack10;
        uVar6 = (undefined2)((ulong)uStack10 >> 0x10);
        uVar1 = *(undefined4 *)((int)_PTR_LOOP_1050_4e70 + 0x4);
        uVar3 = (undefined2)uVar1;
        uVar4 = (undefined2)((ulong)uVar1 >> 0x10);
        uVar2 = 0x7b;
    }
    struct_op_1028_87f0(param_1, param_2, (astruct_97 *)CONCAT22(param_1, local_140), 0x0, 0x0, uVar2, &local_14, param_1, CONCAT22(uVar4, uVar3), CONCAT22(uVar6, uVar5));
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_140));
    return;
}

void __stdcall16far pass1_1020_a6ee(ulong param_1, ushort param_2, uint param_3, uint param_4, int param_5, ushort param_6, uchar param_7)

{
    undefined4 uVar1;
    ushort     uVar2;
    ushort    *puVar3;
    ushort     uVar4;
    undefined  local_13e[0x120];
    undefined4 uStack30;
    BOOL16     BStack26;
    ulong      local_18;
    undefined2 uStack20;
    int        iStack18;
    undefined2 uStack16;
    undefined4 uStack14;
    ushort    *puStack10;
    undefined4 uStack6;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x2, 0x400);
    uStack6 = CONCAT22(param_4, param_3);
    if(((uchar *)(param_4 | param_3) == (uchar *)0x0) || (*(long *)(param_3 + 0x200) == 0x8000002))
    {
        puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_6, (uchar *)(param_4 | param_3), param_5);
        uStack16  = (undefined2)((ulong)puStack10 >> 0x10);
        uVar1     = *(undefined4 *)((int)puStack10 + 0x20);
        uStack14  = uVar1;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
        iStack18 = (int)uVar1;
        puVar3   = pass1_1008_3e38((ushort *)CONCAT22(param_6, &local_18));
        uVar2    = (ushort)((ulong)puVar3 >> 0x10);
        BStack26 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, param_2, 0x28);
        if(BStack26 != 0x0)
        {
            uStack20 = 0x1;
        }
        uVar4 = (ushort)(param_1 >> 0x10);
        pass1_1020_b2da(param_6, (ushort)param_1, uVar4, (uint)(BStack26 != 0x0), (ushort *)CONCAT22(param_6, &local_18), CONCAT22(uStack16, iStack18));
        struct_op_1028_87f0(param_6, param_7, (astruct_97 *)CONCAT22(param_6, local_13e), 0x0, 0x0, param_2, &local_18, param_6, *(ulong *)((int)_PTR_LOOP_1050_4e70 + 0x4), *(ulong *)(iStack18 + 0x4));
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_6, local_13e));
        if(BStack26 != 0x0)
        {
            pass1_1020_ad90(param_6, uVar2, (ushort)param_1, uVar4, (ushort *)CONCAT22(param_6, &local_18), *(ulong *)(iStack18 + 0x4));
        }
        *(undefined4 *)((int)uStack30 + 0x1c) = 0x8000001;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_a80e(ushort param_1, ushort param_2, int param_3, uint param_4, uint param_5, ushort param_6, uchar param_7, int param_8)

{
    ushort  uVar1;
    ulong   uVar2;
    ushort  uVar3;
    ushort *puVar4;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x2, 0x400);
    if(((uchar *)(param_5 | param_4) == (uchar *)0x0) || (*(long *)(param_4 + 0x200) == 0x8000002))
    {
        puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_6, (uchar *)(param_5 | param_4), param_8);
        uVar3  = (ushort)((ulong)puVar4 >> 0x10);
        uVar2  = *(ulong *)((int)puVar4 + 0x20);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)(uVar2 >> 0x10));
        uVar1 = (ushort)uVar2;
        if(param_3 == 0xa)
        {
            pass1_1020_b872(param_6, param_7, CONCAT22(param_2, param_1), uVar2 & 0xffff | (ulong)uVar3 << 0x10);
            return;
        }
        pass1_1020_b0aa(param_1, param_2, param_3, uVar3);
        if(uVar1 != 0x0)
        {
            pass1_1020_abc0(param_6, param_7, CONCAT22(param_2, param_1), uVar1, uVar2 & 0xffff | (ulong)uVar3 << 0x10);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_a89e(ushort param_1, ulong param_2, ulong *param_3, ushort param_4)

{
    int       *piVar1;
    undefined *puVar2;
    ushort     uVar3;
    ushort     uVar4;
    ulong      uVar5;
    uchar     *in_DX;
    uint       uVar6;
    ulong     *puVar7;
    uint       extraout_DX;
    int        unaff_DI;
    uchar      in_AF;
    ushort     uVar8;
    ushort     uVar9;
    undefined  uVar10;
    undefined  uVar11;
    undefined2 local_5ee;
    undefined2 uStack1516;
    ulong     *puStack1218;
    int        iStack1214;
    undefined4 uStack1212;
    undefined  local_4b8[0x8];
    ulong      uStack1200;
    ushort    *puStack1196;
    undefined  local_4a8[0x124];
    undefined  local_384[0x124];
    undefined  local_260[0x124];
    undefined  local_13c[0x124];
    ushort     local_18;
    ushort     local_16;
    ulong      local_14;
    undefined2 uStack16;
    ulong      uStack14;
    ulong      uStack10;
    ushort    *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_1, in_DX, unaff_DI);
    uVar6    = (uint)((ulong)puStack6 >> 0x10);
    uVar5    = *(ulong *)((int)puStack6 + 0x20);
    uStack10 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar5, (uint)(uVar5 >> 0x10));
    uStack14    = uVar5 & 0xffff | (ulong)uVar6 << 0x10;
    local_14    = *param_3;
    uStack16    = *(undefined2 *)(param_3 + 0x1);
    puStack1218 = &local_14;
    puVar7      = &local_14;
    pass1_1008_3e94((ushort *)CONCAT22(param_1, puVar7), (ushort *)CONCAT22(param_1, &local_18), (ushort *)CONCAT22(param_1, &local_16));
    uVar10 = (undefined)param_1;
    uVar11 = (undefined)(param_1 >> 0x8);
    pass1_1008_3e76((ushort *)CONCAT13(uVar11, CONCAT12(uVar10, &local_14)), 0x0, local_18, local_16 + 0x2);
    struct_op_1028_8888(param_1, in_AF, (astruct_100 *)CONCAT22(param_1, local_13c), 0x0, 0x7a, &local_14, param_1, 0x8000002, 0x4000002, uStack10);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_13c));
    pass1_1008_3e76((ushort *)CONCAT13(uVar11, CONCAT12(uVar10, &local_14)), 0x0, local_18 - 0x2, local_16);
    struct_op_1028_8888(param_1, in_AF, (astruct_100 *)CONCAT22(param_1, local_260), 0x0, 0x47, &local_14, param_1, 0x8000002, 0x4000002, uStack10);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_260));
    pass1_1008_3e76((ushort *)CONCAT13(uVar11, CONCAT12(uVar10, &local_14)), 0x1, local_18 - 0x2, local_16);
    struct_op_1028_8888(param_1, in_AF, (astruct_100 *)CONCAT22(param_1, local_384), 0x0, 0x6a, &local_14, param_1, 0x8000002, 0x4000002, uStack10);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_384));
    uVar8 = (ushort)param_2;
    uVar9 = (ushort)(param_2 >> 0x10);
    pass1_1020_ad90(param_1, (ushort)puVar7, uVar8, uVar9, (ushort *)CONCAT22(param_1, &local_14), uStack10);
    pass1_1008_3e76((ushort *)CONCAT13(uVar11, CONCAT12(uVar10, &local_14)), 0x1, local_18 - 0x2, local_16 + 0x1);
    struct_op_1028_8888(param_1, in_AF, (astruct_100 *)CONCAT22(param_1, local_4a8), 0x0, 0x7f, &local_14, param_1, 0x8000002, 0x4000002, uStack10);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_4a8));
    pass1_1020_ad90(param_1, (ushort)puVar7, uVar8, uVar9, (ushort *)CONCAT22(param_1, &local_14), uStack10);
    puStack1196 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x8, param_1, (uchar *)puVar7, (int)&uStack14);
    uStack1200  = *(ulong *)((int)puStack1196 + 0x12);
    pass1_1008_5784((ulong *)CONCAT22(param_1, local_4b8), uStack1200);
    iStack1214 = 0x0;
    do
    {
        do
        {
            puVar2 = local_4b8;
            pass1_1008_5b12(puVar2, param_1);
            uStack1212 = CONCAT22(extraout_DX, puVar2);
            if((extraout_DX | (uint)puVar2) == 0x0)
            {
                pass1_1010_9674((ulong)puStack1196);
                return;
            }
        } while((*(int *)(puVar2 + 0x4) != 0x3e) && (*(int *)(puVar2 + 0x4) != 0x41));
        while(0x0 < *(int *)((int)uStack1212 + 0x6))
        {
            if(iStack1214 == 0x0)
            {
                uVar4 = local_16 - 0x2;
            LAB_1020_ab4a:
                uVar3 = local_18 - 0x2;
            LAB_1020_ab51:
                iStack1214 = iStack1214 + 0x1;
                pass1_1008_3e76((ushort *)CONCAT13(uVar11, CONCAT12(uVar10, &local_14)), 0x0, uVar3, uVar4);
            }
            else
            {
                if(iStack1214 == 0x1)
                {
                    uVar4 = local_16 + 0x2;
                    goto LAB_1020_ab4a;
                }
                if(iStack1214 == 0x2)
                {
                    uVar4 = local_16 + 0x2;
                LAB_1020_ab6e:
                    uVar3 = local_18 + 0x2;
                    goto LAB_1020_ab51;
                }
                if(iStack1214 == 0x3)
                {
                    uVar4 = local_16 - 0x2;
                    goto LAB_1020_ab6e;
                }
                iStack1214 = iStack1214 + 0x1;
                pass1_1020_b2da(param_1, uVar8, uVar9, 0x0, (ushort *)CONCAT22(param_1, &local_14), uStack14);
            }
            struct_op_1028_8888(param_1, in_AF, (astruct_100 *)CONCAT22(param_1, &local_5ee), 0x0, *(ushort *)((int)uStack1212 + 0x4), &local_14, param_1, 0x8000002, 0x4000002, uStack10);
            fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, &local_5ee));
            piVar1     = (int *)((int)uStack1212 + 0x6);
            *piVar1    = *piVar1 + -0x1;
            local_5ee  = 0x389a;
            uStack1516 = 0x1008;
        }
    } while(true);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_abc0(ushort param_1, uchar param_2, ulong param_3, ushort param_4, ulong param_5)

{
    ushort     uVar1;
    undefined2 uVar2;
    ushort    *puVar3;
    ushort     uVar4;
    undefined  local_12e[0x124];
    BOOL16     BStack10;
    ulong      local_8;
    undefined2 uStack4;

    puVar3   = pass1_1008_3e38((ushort *)CONCAT22(param_1, &local_8));
    uVar1    = (ushort)((ulong)puVar3 >> 0x10);
    BStack10 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, param_4, 0x28);
    if(BStack10 != 0x0)
    {
        uStack4 = 0x1;
    }
    uVar4 = (ushort)(param_3 >> 0x10);
    pass1_1020_b2da(param_1, (ushort)param_3, uVar4, (uint)(BStack10 != 0x0), (ushort *)CONCAT22(param_1, &local_8), param_5);
    uVar2 = (undefined2)(param_5 >> 0x10);
    struct_op_1028_87f0(param_1, param_2, (astruct_97 *)CONCAT22(param_1, local_12e), 0x0, 0x0, param_4, &local_8, param_1, *(ulong *)((int)_PTR_LOOP_1050_4e70 + 0x4), *(ulong *)((int)param_5 + 0x4));
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_12e));
    if(BStack10 != 0x0)
    {
        pass1_1020_ad90(param_1, uVar1, (ushort)param_3, uVar4, (ushort *)CONCAT22(param_1, &local_8), *(ulong *)((int)param_5 + 0x4));
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_ac6e(ushort param_1, uchar param_2, ulong param_3, int param_4, int param_5, int param_6)

{
    ushort     uVar1;
    ulong     *puVar2;
    ulong      uVar3;
    ushort     uVar4;
    int        unaff_DI;
    ushort    *puVar5;
    undefined2 uVar6;
    undefined  local_146[0x12c];
    int        iStack26;
    ushort     uStack24;
    ulong      uStack22;
    ushort    *puStack18;
    ulong      local_e;
    ushort     local_8;
    undefined4 local_6;

    if(param_4 == 0x0)
    {
        uVar6 = SUB42(&PTR_LOOP_1050_4230, 0x0);
    }
    else
    {
        uVar6 = 0x4236;
    }
    pass1_1008_3eb4((ushort *)CONCAT22(0x1048, uVar6), (ushort *)CONCAT22(param_1, &local_8), (ushort *)CONCAT22(param_1, &local_6), (ushort *)CONCAT22(param_1, (int)&local_6 + 0x2));
    if(param_6 == 0x0)
    {
        local_6 = local_6 & 0xffff | (ulong)(uint)(local_6._2_2_ + param_5) << 0x10;
    }
    else
    {
        if(param_6 == 0x1)
        {
            local_6 = local_6 & 0xffff0000 | (ulong)(uint)((int)local_6 + param_5);
        }
        else
        {
            if(param_6 == 0x2)
            {
                local_6 = local_6 & 0xffff | (ulong)(uint)(local_6._2_2_ - param_5) << 0x10;
            }
        }
    }
    puVar5    = pass1_1008_3e54((ushort *)CONCAT22(param_1, &local_e), local_8, (ushort)local_6, (ushort)(local_6 >> 0x10));
    puStack18 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_1, (uchar *)((ulong)puVar5 >> 0x10), unaff_DI);
    uVar4     = (ushort)((ulong)puStack18 >> 0x10);
    uVar3     = *(ulong *)((int)puStack18 + 0x20);
    uStack22  = uVar3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar3, (uint)(uVar3 >> 0x10));
    iStack26 = (int)uVar3;
    uStack24 = uVar4;
    uVar1    = pass1_1020_b1ae((int)&local_e, uVar4, param_1, (ushort)param_3, (ushort)(param_3 >> 0x10), (ushort *)CONCAT22(param_1, &local_e), *(ulong *)(iStack26 + 0x4));
    if(uVar1 != 0x0)
    {
        puVar2 = &local_e;
        pass1_1020_b240(param_1, uVar4, param_3, CONCAT22(param_1, puVar2), CONCAT22(uStack24, iStack26));
        if(puVar2 != (ulong *)0x0)
        {
            struct_op_1028_87f0(param_1, param_2, (astruct_97 *)CONCAT22(param_1, local_146), 0x0, 0x0, (-(uint)(param_4 == 0x0) & 0xfffb) + 0x7f, &local_e, param_1, *(ulong *)((int)_PTR_LOOP_1050_4e70 + 0x4), uStack22);
            fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, local_146));
        }
    }
    return;
}

void __stdcall16far pass1_1020_ad90(ushort param_1, ushort param_2, ushort param_3, ushort param_4, ushort *param_5, ulong param_6)

{
    code      **ppcVar1;
    ushort     *puVar2;
    undefined  *puVar3;
    int         iVar4;
    undefined4  uVar5;
    uint        uVar6;
    undefined2  extraout_DX;
    uchar       in_AF;
    undefined4 *puVar7;
    ushort      uVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    undefined2  local_17e;
    undefined2  uStack380;
    int         iStack90;
    undefined4 *puStack78;
    ushort      uStack70;
    int         iStack68;
    undefined4  uStack66;
    undefined4 *puStack62;
    undefined   local_3a[0xc];
    undefined4  local_2e;
    undefined2  uStack42;
    int         iStack40;
    ushort      uStack38;
    int         local_24;
    int         local_22;
    undefined4  uStack32;
    undefined4  uStack28;
    undefined4  uStack24;
    ushort     *puStack20;
    uint        uStack18;
    int         iStack16;
    int         iStack14;
    undefined4  uStack12;
    ushort      local_8;
    int         local_6;
    int         local_4;

    puVar2 = &local_8;
    pass1_1008_3eb4(param_5, (ushort *)CONCAT22(param_1, puVar2), (ushort *)CONCAT22(param_1, &local_6), (ushort *)CONCAT22(param_1, &local_4));
    pass1_1030_627e(param_1, (uint)puVar2, param_2, (ulong)_PTR_LOOP_1050_5740, param_5, param_6);
    puStack20 = puVar2;
    uStack18  = param_2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)puVar2, param_2);
    uStack24 = CONCAT22(param_2, puVar2);
    uStack28 = *(undefined4 *)(puVar2 + 0x17);
    uVar5    = *(undefined4 *)((int)uStack28 + 0x4);
    uStack32 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_6, (uint)(param_6 >> 0x10));
    iStack40  = (int)uVar5;
    uStack38  = param_2;
    puVar7    = (undefined4 *)pass1_1030_5b5c(iStack40, param_2);
    uVar6     = (uint)((ulong)puVar7 >> 0x10);
    local_2e  = *puVar7;
    uStack42  = *(undefined2 *)((int)puVar7 + 0x4);
    puStack78 = &local_2e;
    pass1_1008_3e94((ushort *)CONCAT22(param_1, &local_2e), (ushort *)CONCAT22(param_1, &local_24), (ushort *)CONCAT22(param_1, &local_22));
    iStack14 = local_4 + 0x1;
    uStack12 = CONCAT22(local_4 + -0x1, local_6 - 0x1U);
    iStack16 = local_6 + 0x1;
    if(local_4 + -0x1 < 0x0)
    {
        uStack12 = (ulong)(local_6 - 0x1U);
    }
    if(local_22 <= iStack14)
    {
        iStack14 = local_22 + -0x1;
    }
    if((int)(ushort)uStack12 < 0x0)
    {
        uStack12 = uStack12 & 0xffff0000;
    }
    if(local_24 <= iStack16)
    {
        iStack16 = local_24 + -0x1;
    }
    pass1_1008_6c90((ushort *)CONCAT22(param_1, local_3a));
    pass1_1008_6cec((ushort *)CONCAT22(param_1, local_3a), local_8, CONCAT22(iStack14, iStack16), local_8, uStack12);
    puVar3 = local_3a;
    pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(param_1, puVar3), param_6, param_1);
    puStack62 = (undefined4 *)CONCAT22(uVar6, puVar3);
    if((uVar6 | (uint)puVar3) != 0x0)
    {
        uStack66 = 0x0;
        iStack68 = 0x0;
        for(uStack70 = (ushort)uStack12; (int)uStack70 <= iStack16; uStack70 = uStack70 + 0x1)
        {
            for(puStack78 = (undefined4 *)uStack12._2_2_; (int)puStack78 <= iStack14; puStack78 = (undefined4 *)((int)puStack78 + 0x1))
            {
                ppcVar1  = (code **)((int)*puStack62 + 0x4);
                iVar4    = iStack68;
                iStack68 = iStack68 + 0x1;
                (**ppcVar1)(0x1030, (int)puStack62, (int)((ulong)puStack62 >> 0x10));
                uStack66       = CONCAT22(extraout_DX, iVar4);
                uStack66._3_1_ = (char)((uint)extraout_DX >> 0x8);
                if(uStack66._3_1_ == '\0')
                {
                    iStack90 = iVar4;
                    if(iVar4 == 0x7)
                    {
                        pass1_1008_3e76(param_5, local_8, uStack70, (ushort)puStack78);
                        uVar9  = (undefined2)uStack32;
                        uVar10 = (undefined2)((ulong)uStack32 >> 0x10);
                        uVar8  = 0x6;
                    }
                    else
                    {
                        if(iVar4 == 0x8)
                        {
                            pass1_1008_3e76(param_5, local_8, uStack70, (ushort)puStack78);
                            uVar9  = (undefined2)uStack32;
                            uVar10 = (undefined2)((ulong)uStack32 >> 0x10);
                            uVar8  = 0x7;
                        }
                        else
                        {
                            if(iVar4 != 0x9)
                                goto LAB_1020_af1c;
                            pass1_1008_3e76(param_5, local_8, uStack70, (ushort)puStack78);
                            uVar9  = (undefined2)uStack32;
                            uVar10 = (undefined2)((ulong)uStack32 >> 0x10);
                            uVar8  = 0x8;
                        }
                    }
                    struct_op_1028_87f0(param_1, in_AF, (astruct_97 *)CONCAT22(param_1, &local_17e), 0x0, 0x0, uVar8, (ulong *)param_5, (ushort)((ulong)param_5 >> 0x10), CONCAT22(uVar10, uVar9), param_6);
                    fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_1, &local_17e));
                    local_17e = 0x389a;
                    uStack380 = 0x1008;
                }
            LAB_1020_af1c:
            }
        }
    }
    return;
}

void __stdcall16far pass1_1020_87c2(ushort *param_1, ushort param_2, int param_3)

{
    undefined4   uVar1;
    astruct_281 *iVar2;
    uint         uVar2;
    ushort      *puVar3;
    undefined    local_12[0x8];
    int          iStack10;
    ushort      *puStack8;
    int          iStack4;

    struct_1020_847a(param_1, 0x4, param_2);
    iStack4  = 0x4;
    iVar2    = (astruct_281 *)param_1;
    iVar2    = (astruct_281 *)&iVar2->field_0x16;
    puStack8 = (ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar2));
    do
    {
        pass1_1008_3e38(puStack8);
        puStack8 = (ushort *)((ulong)puStack8 & 0xffff0000 | (ulong)((int)puStack8 + 0x6));
        iStack4  = iStack4 + -0x1;
    } while(iStack4 != 0x0);
    uVar2                             = (uint)((ulong)param_1 >> 0x10);
    *(undefined4 *)&iVar2->field_0x2e = 0x0;
    puVar3                            = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x32));
    iVar2->field_0x38                 = 0x0;
    *param_1                          = 0x8a84;
    iVar2->field_0x2                  = 0x1020;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, param_2, (uchar *)((ulong)puVar3 >> 0x10), param_3);
    iVar2->field_0x2e                 = (int)puVar3;
    iVar2->field_0x30                 = (int)((ulong)puVar3 >> 0x10);
    iStack10                          = 0x0;
    do
    {
        uVar1 = *(undefined4 *)&iVar2->field_0x2e;
        pass1_1018_26d8((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), iStack10, (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)(&iVar2->field_0x16 + iStack10 * 0x6)));
        uVar1 = *(undefined4 *)&iVar2->field_0x2e;
        pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10,
                        (int *)CONCAT22(iVar2->field_0xa, iVar2->field_0x8 + iStack10 * 0x8),
                        *(astruct_76 **)((int)uVar1 + 0x2e + iStack10 * 0x4),
                        (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)(&iVar2->field_0x16 + iStack10 * 0x6)));
        iStack10 = iStack10 + 0x1;
    } while(iStack10 < 0x4);
    uVar1 = *(undefined4 *)&iVar2->field_0x2e;
    pass1_1018_2548((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x32));
    uVar1             = *(undefined4 *)&iVar2->field_0x2e;
    iVar2->field_0x38 = *(undefined4 *)((int)uVar1 + 0x6e);
    pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10, (int *)CONCAT22(param_2, local_12), (astruct_76 *)iVar2->field_0x38, (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x32));
    return;
}

void __stdcall16far pass1_1020_8908(ulong param_1, ulong param_2, ushort param_3)

{
    astruct_76  *paVar1;
    undefined4   uVar2;
    uint         uVar3;
    uint         uVar4;
    uchar       *puVar5;
    uchar       *puVar6;
    undefined2   uVar7;
    astruct_284 *iVar8;
    int          iVar9;
    int          iVar10;
    undefined2   uVar11;
    undefined2   uVar12;
    ulong        uVar13;
    astruct_110 *paStack28;
    int          iStack4;

    for(iStack4 = 0x0; iVar8 = (astruct_284 *)param_1, uVar11 = (undefined2)(param_1 >> 0x10), iStack4 < 0x4; iStack4 = iStack4 + 0x1)
    {
        if(iVar8->field_0x4 == 0x0)
        {
            uVar2  = iVar8->field_0xc;
            uVar11 = (undefined2)((ulong)uVar2 >> 0x10);
            iVar10 = (int)uVar2;
            iVar9  = iStack4 * 0x4;
            if((*(uint *)(iVar10 + iVar9 + 0x2) | *(uint *)(iVar10 + iVar9)) != 0x0)
            {
                pass1_1008_5236(*(ulong *)(iVar10 + iVar9));
            }
        }
        else
        {
            uVar2  = iVar8->field_0x2e;
            paVar1 = *(astruct_76 **)((int)uVar2 + 0x2e + iStack4 * 0x4);
            uVar13 = pass1_1008_4772(paVar1);
            puVar5 = (uchar *)(uVar13 >> 0x10);
            uVar3  = (uint)uVar13;
            uVar2  = iVar8->field_0xc;
            iVar10 = iStack4 * 0x4;
            if(*(long *)((int)uVar2 + iVar10) == 0x0)
            {
                puVar6 = puVar5;
                uVar4  = uVar3;
                mem_op_1000_179c(0x14, puVar5, 0x1000);
                paStack28 = (astruct_110 *)CONCAT22(puVar6, uVar4);
                if(((uint)puVar6 | uVar4) == 0x0)
                {
                    uVar2                                       = iVar8->field_0xc;
                    *(undefined4 *)((int)uVar2 + iStack4 * 0x4) = 0x0;
                }
                else
                {
                    uVar4 = &iVar8->field_0x16 + iStack4 * 0x6;
                    uVar7 = uVar11;
                    pass1_1008_50c2(paStack28, *(ulong *)(uVar3 + 0x8), *(ulong *)(uVar3 + 0x4), (uint *)(param_1 & 0xffff0000 | (ulong)uVar4), param_2);
                    uVar2                                 = iVar8->field_0xc;
                    uVar12                                = (undefined2)((ulong)uVar2 >> 0x10);
                    iVar9                                 = (int)uVar2;
                    *(uint *)(iVar9 + iVar10)             = uVar4;
                    *(undefined2 *)(iVar9 + iVar10 + 0x2) = uVar7;
                }
                uVar2 = iVar8->field_0xc;
                pass1_1008_5134(*(ulong *)((int)uVar2 + iStack4 * 0x4));
            }
            uVar2 = iVar8->field_0xc;
            pass1_1008_5236(*(ulong *)((int)uVar2 + iStack4 * 0x4));
            pass1_1008_4480(param_2, (ushort *)(param_1 & 0xffff0000 | (ulong)(uint)(&iVar8->field_0x16 + iStack4 * 0x6)), paVar1, param_3);
        }
    }
    if(iVar8->field_0x4 != 0x0)
    {
        pass1_1008_4480(param_2, (ushort *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar8->field_0x32), iVar8->field_0x38, param_3);
    }
    return;
}

void __stdcall16far pass1_1020_8a9c(ushort *param_1)

{
    undefined4  uVar1;
    ulong       uVar2;
    uint        uVar3;
    ushort      uVar4;
    ushort      unaff_SS;
    ushort     *puVar5;
    astruct_76 *paVar6;
    astruct_43 *paVar7;
    int         iVar8;
    uint        uVar9;
    ushort     *puStack76;
    undefined   local_48[0x1e];
    undefined   local_2a[0x24];
    undefined2  uStack6;
    undefined2  uStack4;

    iVar8 = (int)param_1;
    uVar9 = (uint)((ulong)param_1 >> 0x10);
    struct_1020_847a(param_1, 0x2, unaff_SS);
    uVar3 = iVar8 + 0x16;
    pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar3));
    puStack76                     = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar8 + 0x1cU));
    puVar5                        = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar8 + 0x1cU)));
    *(undefined4 *)(iVar8 + 0x22) = 0x0;
    *param_1                      = 0x8e92;
    *(undefined2 *)(iVar8 + 0x2)  = 0x1020;
    puVar5                        = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, unaff_SS, (uchar *)((ulong)puVar5 >> 0x10), uVar9);
    uVar4                         = (ushort)((ulong)puVar5 >> 0x10);
    *(undefined2 *)(iVar8 + 0x22) = (int)puVar5;
    *(ushort *)(iVar8 + 0x24)     = uVar4;
    pass1_1018_2678(*(ushort *)(iVar8 + 0x22), uVar4, (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar3));
    paVar6  = (astruct_76 *)pass1_1018_268e(*(ulong *)(iVar8 + 0x22));
    uStack4 = (undefined2)((ulong)paVar6 >> 0x10);
    uStack6 = SUB42(paVar6, 0x0);
    pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar9 << 0x10, *(int **)(iVar8 + 0x8), paVar6, (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar3));
    uVar1 = *(undefined4 *)(iVar8 + 0x22);
    pass1_1018_26c2((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), puStack76);
    paVar7 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x2, unaff_SS);
    struct_op_1008_48fe((astruct_81 *)CONCAT13((char)(unaff_SS >> 0x8), CONCAT12((char)unaff_SS, local_2a)), 0x1, (char *)paVar7, (ushort)((ulong)paVar7 >> 0x10));
    struct_op_1008_3f92((astruct_76 *)CONCAT22(unaff_SS, local_48), (astruct_83 *)CONCAT22(unaff_SS, local_2a));
    uVar2 = *(ulong *)(iVar8 + 0x8);
    pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar9 << 0x10, (int *)(uVar2 & 0xffff0000 | (ulong)((int)uVar2 + 0x8)), (astruct_76 *)CONCAT22(unaff_SS, local_48), puStack76);
    pass1_1008_41bc((ushort *)CONCAT22(unaff_SS, local_48));
    close_file_1008_496c(local_2a, unaff_SS);
    return;
}

void __stdcall16far pass1_1020_8eaa(ushort *param_1, ushort param_2)

{
    uint         uVar1;
    ushort       uVar2;
    uchar       *puVar3;
    astruct_668 *iVar4;
    uint         uVar4;
    ushort      *puVar5;
    astruct_43  *paVar6;
    undefined    local_a[0x8];

    struct_1020_847a(param_1, 0x25, param_2);
    uVar4                             = (uint)((ulong)param_1 >> 0x10);
    iVar4                             = (astruct_668 *)param_1;
    *(undefined4 *)&iVar4->field_0x16 = 0x0;
    iVar4->field_0xaa                 = 0x0;
    uVar1                             = &iVar4->field_0xae;
    puVar5                            = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar1));
    *(undefined4 *)&iVar4->field_0xb4 = 0x0;
    iVar4->field_0xb8                 = 0xffff;
    *(undefined4 *)&iVar4->field_0xba = 0x0;
    *param_1                          = 0x9204;
    iVar4->field_0x2                  = 0x1020;
    puVar5                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, param_2, (uchar *)((ulong)puVar5 >> 0x10), uVar4);
    uVar2                             = (ushort)((ulong)puVar5 >> 0x10);
    iVar4->field_0x16                 = (int)puVar5;
    iVar4->field_0x18                 = uVar2;
    pass1_1018_2646(iVar4->field_0x16, uVar2, (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar1));
    paVar6            = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1ce, param_2);
    puVar3            = (uchar *)((ulong)paVar6 >> 0x10);
    iVar4->field_0xb4 = (uint)paVar6;
    iVar4->field_0xb6 = puVar3;
    pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10, (int *)CONCAT22(param_2, local_a), (astruct_76 *)((ulong)paVar6 & 0xffff0000 | (ulong)iVar4->field_0xb4), (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)uVar1));
    puVar5            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_2, puVar3, uVar4);
    iVar4->field_0xba = (int)puVar5;
    iVar4->field_0xbc = (int)((ulong)puVar5 >> 0x10);
    return;
}

void __stdcall16far pass1_1020_915a(ulong param_1, uchar *param_2, int param_3, ushort param_4)

{
    int          iVar1;
    astruct_669 *iVar2;
    undefined2   uVar2;
    ushort      *puVar3;
    astruct_43  *paVar4;
    ushort       uStack12;

    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_4, param_2, param_3);
    iVar1  = *(int *)((int)puVar3 + 0x1e);
    uVar2  = (undefined2)(param_1 >> 0x10);
    iVar2  = (astruct_669 *)param_1;
    if(iVar2->field_0xb8 != iVar1)
    {
        uStack12 = 0x1ce;
        if(iVar1 == 0x1)
        {
            uStack12 = 0x1cf;
        }
        else
        {
            if(iVar1 == 0x2)
            {
                uStack12 = 0x1d0;
            }
            else
            {
                if(iVar1 == 0x3)
                {
                    uStack12 = 0x1d1;
                }
                else
                {
                    if(iVar1 == 0x4)
                    {
                        uStack12 = 0x1d2;
                    }
                }
            }
        }
        paVar4            = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, uStack12, param_4);
        iVar2->field_0xb4 = (int)paVar4;
        iVar2->field_0xb6 = (int)((ulong)paVar4 >> 0x10);
        iVar2->field_0xb8 = iVar1;
    }
    return;
}

void __stdcall16far get_sys_metrics_1020_7c1a(ushort *param_1, ulong param_2, INT16 param_3)

{
    INT16       IVar1;
    astruct_56 *iVar3;
    ushort      uVar3;
    ushort      uVar4;
    ushort      uVar1;

    uVar3             = (ushort)(param_2 >> 0x10);
    uVar1             = *(ushort *)((int)param_2 + 0x8);
    uVar4             = (ushort)((ulong)param_1 >> 0x10);
    iVar3             = (astruct_56 *)param_1;
    *param_1          = 0x389a;
    iVar3->field_0x2  = 0x1008;
    *param_1          = 0x3aa8;
    iVar3->field_0x2  = 0x1008;
    iVar3->field_0x4  = uVar1;
    *param_1          = 0x3ab0;
    iVar3->field_0x2  = 0x1008;
    iVar3->field_0x6  = param_2;
    iVar3->field_0xa  = 0x0;
    iVar3->field_0xe  = 0x0;
    iVar3->field_0x10 = 0x0;
    iVar3->field_0x12 = 0x0;
    *param_1          = 0x7f72;
    iVar3->field_0x2  = 0x1020;
    iVar3->field_0xa  = *(undefined4 *)((int)param_2 + 0xe4);
    IVar1             = GetSystemMetrics16(param_3);
    iVar3->field_0xe  = IVar1;
    IVar1             = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    iVar3->field_0x10 = IVar1;
    IVar1             = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    iVar3->field_0x12 = IVar1;
    return;
}

void __stdcall16far pass1_1020_8360(ushort *param_1, ushort param_2)

{
    undefined4   uVar1;
    ushort       uVar2;
    ushort      *puVar3;
    uint         uVar4;
    astruct_667 *iVar4;

    iVar4 = (astruct_667 *)param_1;
    uVar4 = (uint)((ulong)param_1 >> 0x10);
    struct_1020_847a(param_1, 0x1, param_2);
    puVar3                            = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x16));
    *(undefined4 *)&iVar4->field_0x1c = 0x0;
    *param_1                          = 0x8462;
    iVar4->field_0x2                  = 0x1020;
    puVar3                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, param_2, (uchar *)((ulong)puVar3 >> 0x10), uVar4);
    uVar2                             = (ushort)((ulong)puVar3 >> 0x10);
    iVar4->field_0x1c                 = (int)puVar3;
    iVar4->field_0x1e                 = uVar2;
    pass1_1018_26f8(iVar4->field_0x1c, uVar2, (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x16));
    uVar1 = *(undefined4 *)&iVar4->field_0x1c;
    pass1_1020_8712((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10, iVar4->field_0x8, *(astruct_76 **)((int)uVar1 + 0x2a), (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar4->field_0x16));
    return;
}

void __stdcall16far struct_1020_847a(undefined2 *param_1, int param_2, ushort param_3)

{
    ushort       uVar1;
    uchar       *puVar2;
    astruct_280 *iVar3;
    int          iVar4;
    ushort      *puVar5;

    iVar4            = (int)((ulong)param_1 >> 0x10);
    iVar3            = (astruct_280 *)param_1;
    *param_1         = 0x389a;
    iVar3->field_0x2 = 0x1008;
    iVar3->field_0x4 = 0x0;
    iVar3->field_0x6 = param_2;
    iVar3->field_0x8 = (astruct_20 *)0x0;
    iVar3->field_0xc = (astruct_20 *)0x0;
    puVar5           = pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar3 + 0x1)));
    *param_1         = 0x87aa;
    iVar3->field_0x2 = 0x1020;
    puVar5           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_3, (uchar *)((ulong)puVar5 >> 0x10), iVar4);
    puVar2           = (uchar *)((ulong)puVar5 >> 0x10);
    pass1_1008_3f62((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar3 + 0x1)), (ushort *)((ulong)puVar5 & 0xffff0000 | (ulong)((int)puVar5 + 0xe)));
    uVar1 = iVar3->field_0x6 << 0x3;
    mem_op_1000_179c(uVar1, puVar2, 0x1000);
    *(ushort *)&iVar3->field_0x8              = uVar1;
    *(uchar **)((int)&iVar3->field_0x8 + 0x2) = puVar2;
    uVar1                                     = iVar3->field_0x6 << 0x2;
    mem_op_1000_179c(uVar1, puVar2, 0x1000);
    *(ushort *)&iVar3->field_0xc              = uVar1;
    *(uchar **)((int)&iVar3->field_0xc + 0x2) = puVar2;
    pass1_1000_4906(iVar3->field_0x8, (WNDCLASS16 *)0x0, iVar3->field_0x6 << 0x3);
    pass1_1000_4906(iVar3->field_0xc, (WNDCLASS16 *)0x0, iVar3->field_0x6 << 0x2);
    return;
}
void __stdcall16far pass1_1020_62e0(int param_1, ushort param_2, ushort param_3, ushort param_4)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    uint       *puVar3;
    undefined4  uVar4;
    uchar      *extraout_DX;
    uchar      *puVar5;
    uchar      *puVar6;
    undefined2  uVar7;
    uchar      *extraout_DX_00;
    int         unaff_DI;
    ushort     *puVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    int         iVar11;
    ushort      uVar12;

    set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)(param_1 + 0x14)           = 0x0;
    *(undefined4 *)(param_1 + 0x2c)           = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x67c2;
    *(undefined2 *)(param_1 + 0x2)            = 0x1020;
    puVar6                                    = extraout_DX;
    puVar3                                    = pass1_1000_4906((astruct_20 *)CONCAT22(param_2, param_1 + 0x18), (WNDCLASS16 *)0x0, 0x14);
    mem_op_1000_179c(0x3c, puVar6, 0x1000);
    puVar5 = (uchar *)((uint)puVar6 | (uint)puVar3);
    if(puVar5 == (uchar *)0x0)
    {
        *(undefined4 *)(param_1 + 0x1c) = 0x0;
    }
    else
    {
        pass1_1020_87c2((ushort *)CONCAT22(puVar6, puVar3), param_4, unaff_DI);
        *(uint **)(param_1 + 0x1c)  = puVar3;
        *(uchar **)(param_1 + 0x1e) = puVar5;
    }
    mem_op_1000_179c(0x26, puVar5, 0x1000);
    puVar6 = (uchar *)((uint)puVar5 | (uint)puVar3);
    if(puVar6 == (uchar *)0x0)
    {
        puVar3 = (uint *)0x0;
        puVar6 = (uchar *)0x0;
    }
    else
    {
        pass1_1020_8a9c((ushort *)CONCAT22(puVar5, puVar3));
    }
    *(uint **)(param_1 + 0x20)  = puVar3;
    *(uchar **)(param_1 + 0x22) = puVar6;
    mem_op_1000_179c(0xbe, puVar6, 0x1000);
    puVar5 = (uchar *)((uint)puVar6 | (uint)puVar3);
    if(puVar5 == (uchar *)0x0)
    {
        puVar3 = (uint *)0x0;
        puVar5 = (uchar *)0x0;
    }
    else
    {
        pass1_1020_8eaa((ushort *)CONCAT22(puVar6, puVar3), param_4);
    }
    *(uint **)(param_1 + 0x24)  = puVar3;
    *(uchar **)(param_1 + 0x26) = puVar5;
    mem_op_1000_179c(0x20, puVar5, 0x1000);
    puVar6 = (uchar *)((uint)puVar5 | (uint)puVar3);
    if(puVar6 == (uchar *)0x0)
    {
        puVar3 = (uint *)0x0;
        puVar6 = (uchar *)0x0;
    }
    else
    {
        pass1_1020_8360((ushort *)CONCAT22(puVar5, puVar3), param_4);
    }
    *(uint **)(param_1 + 0x28)  = puVar3;
    *(uchar **)(param_1 + 0x2a) = puVar6;
    pass1_1020_6746(CONCAT22(param_2, param_1), 0x1, 0x4);
    puVar8                          = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x29, param_4, puVar6, unaff_DI);
    uVar7                           = (undefined2)((ulong)puVar8 >> 0x10);
    *(undefined2 *)(param_1 + 0x14) = (int)puVar8;
    *(undefined2 *)(param_1 + 0x16) = uVar7;
    uVar10                          = 0x0;
    uVar9                           = *(undefined2 *)(param_1 + 0x14);
    ppcVar2                         = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0x14) + 0x4);
    iVar11                          = param_1;
    uVar12                          = param_2;
    (**ppcVar2)();
    *(undefined4 *)(param_1 + 0x6) = *(undefined4 *)(param_1 + 0x14);
    uVar4                          = *(undefined4 *)(param_1 + 0x14);
    puVar1                         = (undefined4 *)*(undefined4 *)((int)uVar4 + 0xa);
    uVar4                          = CONCAT22(param_2, param_1 + 0xa);
    ppcVar2                        = (code **)((int)*puVar1 + 0x8);
    (**ppcVar2)(0x1010, (int)puVar1, (int)((ulong)puVar1 >> 0x10), uVar4, uVar9, uVar7, uVar10, iVar11, uVar12);
    *(undefined2 *)(param_1 + 0x12) = (int)uVar4;
    *(undefined2 *)(param_1 + 0x10) = 0x1;
    puVar8                          = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_4, extraout_DX_00, unaff_DI);
    *(undefined2 *)(param_1 + 0x2c) = (int)puVar8;
    *(undefined2 *)(param_1 + 0x2e) = (int)((ulong)puVar8 >> 0x10);
    return;
}

void __stdcall16far pass1_1020_61c4(ushort param_1, ushort param_2, ulong param_3, ushort *param_4, uchar *param_5, int param_6, ushort param_7)

{
    ulong   uVar1;
    ushort  uVar2;
    ushort *puVar3;

    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_7, param_5, param_6);
    uVar2  = (ushort)((ulong)puVar3 >> 0x10);
    uVar1  = *(ulong *)((int)puVar3 + 0x20);
    pass1_1030_8308((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), (ushort *)param_3, param_4, uVar1, (ushort)uVar1, uVar2);
    *param_4 = *(ushort *)((int)puVar3 + 0x1e);
    return;
}

void __stdcall16far pass1_1020_3540(ushort param_1, ushort param_2, int param_3, ushort *param_4, uchar *param_5, undefined2 param_6)

{
    ushort       uVar1;
    astruct_279 *iVar2;
    int          iStack18;
    int          iStack12;
    int          iStack10;
    int          local_6;
    int          local_4;

    pass1_1008_3e94(param_4, (ushort *)CONCAT22(param_6, &local_6), (ushort *)CONCAT22(param_6, &local_4));
    if(param_3 == 0x0)
    {
        iStack12 = 0x3;
        iStack10 = 0x42a6;
    }
    else
    {
        if(param_3 == 0x1)
        {
            iStack12 = 0x4;
            iStack10 = (int)s_SITEICON_1050_428d + 0x9;
        }
        else
        {
            if(param_3 != 0x2)
            {
                return;
            }
            iStack12 = 0x4;
            iStack10 = 0x42b2;
        }
    }
    uVar1 = iStack12 << 0x2;
    mem_op_1000_179c(uVar1, param_5, 0x1000);
    for(iStack18 = 0x0; iStack18 < iStack12; iStack18 = iStack18 + 0x1)
    {
        iVar2                         = (astruct_279 *)(iStack18 * 0x4);
        *(int *)(iVar2 + uVar1)       = *(int *)(iVar2 + iStack10) + local_4;
        *(int *)(iVar2 + uVar1 + 0x2) = *(int *)(iVar2 + iStack10 + 0x2) + local_6;
    }
    return;
}

void __stdcall16far pass1_1020_1eea(ushort *param_1, ulong param_2, ushort param_3, uchar *param_4, int param_5, ushort param_6)

{
    code       **ppcVar1;
    undefined2   uVar2;
    astruct_663 *iVar3;
    undefined2   uVar3;
    ushort      *puVar4;

    uVar3                                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar3                                         = (astruct_663 *)param_1;
    *param_1                                      = 0x389a;
    iVar3->field_0x2                              = 0x1008;
    *param_1                                      = 0x3aa8;
    iVar3->field_0x2                              = 0x1008;
    iVar3->field_0x4                              = param_3;
    *param_1                                      = 0x3ab0;
    iVar3->field_0x2                              = 0x1008;
    iVar3->field_0x6                              = (undefined4 *)0x0;
    iVar3->field_0xa                              = param_2;
    *param_1                                      = 0x2518;
    iVar3->field_0x2                              = 0x1020;
    puVar4                                        = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x39, param_6, param_4, param_5);
    uVar2                                         = (undefined2)((ulong)puVar4 >> 0x10);
    *(int *)&iVar3->field_0x6                     = (int)puVar4;
    *(undefined2 *)((int)&iVar3->field_0x6 + 0x2) = uVar2;
    ppcVar1                                       = (code **)((int)*iVar3->field_0x6 + 0x4);
    (**ppcVar1)(0x1010, *(undefined2 *)&iVar3->field_0x6, uVar2, 0x0, param_1);
    return;
}

ulong __stdcall16far pass1_1020_23f2(ushort param_1, ushort param_2, ushort *param_3, uchar *param_4, undefined2 param_5)

{
    int *piVar1;
    int  iStack18;
    int  local_6;
    int  local_4;

    piVar1 = &local_6;
    pass1_1008_3e94(param_3, (ushort *)CONCAT22(param_5, piVar1), (ushort *)CONCAT22(param_5, &local_4));
    mem_op_1000_179c(0xc, param_4, 0x1000);
    for(iStack18 = 0x0; iStack18 < 0x3; iStack18 = iStack18 + 0x1)
    {
        piVar1[iStack18 * 0x2]       = *(int *)(iStack18 * 0x4 + 0x4270) + local_4;
        piVar1[iStack18 * 0x2 + 0x1] = *(int *)(iStack18 * 0x4 + 0x4272) + local_6;
    }
    return CONCAT22(param_4, piVar1);
}

void __stdcall16far pass1_1020_25c0(ulong param_1, ushort param_2, ushort param_3)

{
    int         *piVar1;
    code       **ppcVar2;
    uint         uVar3;
    uint         uVar4;
    astruct_277 *iVar3;
    undefined2   uVar5;
    astruct_57  *paVar6;
    undefined4  *puStack6;

    paVar6 = (astruct_57 *)CONCAT22(param_3, param_2);
    uVar5  = (undefined2)(param_1 >> 0x10);
    iVar3  = (astruct_277 *)param_1;
    if(iVar3->field_0xee != (undefined4 *)0x0)
    {
        ppcVar2 = (code **)((int)*iVar3->field_0xee + 0x8);
        paVar6  = (astruct_57 *)(**ppcVar2)();
    }
    if(iVar3->field_0xea == 0x0)
    {
        iVar3->field_0xea = 0x1;
        mem_op_1000_179c(0x98, (uchar *)((ulong)paVar6 >> 0x10), 0x1000);
        uVar3 = (uint)paVar6;
        uVar4 = (uint)((ulong)paVar6 >> 0x10) | uVar3;
        if(paVar6 == (astruct_57 *)0x0)
        {
            puStack6 = (undefined4 *)0x0;
        }
        else
        {
            piVar1  = &iVar3->field_0xcc;
            *piVar1 = *piVar1 + 0x1;
            struct_1020_1738(paVar6, iVar3->field_0xcc, param_1);
            puStack6 = (undefined4 *)CONCAT22(uVar4, uVar3);
        }
        ppcVar2 = (code **)((int)*puStack6 + 0x8);
        (**ppcVar2)(0x1000, (int)puStack6, (int)((ulong)puStack6 >> 0x10));
    }
    return;
}

void __stdcall16far pass1_1020_294a(ulong param_1, ulong param_2, ushort param_3, uchar *param_4, int param_5, ushort param_6)

{
    undefined2   uVar1;
    astruct_665 *iVar3;
    undefined2   uVar2;
    ushort      *puVar3;

    uVar2             = (undefined2)(param_1 >> 0x10);
    iVar3             = (astruct_665 *)param_1;
    iVar3->field_0xfc = param_3;
    puVar3            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, param_3, param_6, param_4, param_5);
    uVar1             = (undefined2)((ulong)puVar3 >> 0x10);
    iVar3->field_0xf2 = (int)puVar3;
    iVar3->field_0xf4 = uVar1;
    iVar3->field_0xe0 = iVar3->field_0xf2;
    iVar3->field_0xe2 = uVar1;
    pass1_1018_0902(*(ulong **)&iVar3->field_0xf2, param_2);
    return;
}


void __stdcall16far struct_1020_1738(astruct_57 *param_1, ushort param_2, ulong param_3)

{
    astruct_278 *iVar1;
    undefined2   uVar1;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfcd, *(ushort *)((int)param_3 + 0x8));
    uVar1                  = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                  = (astruct_278 *)param_1;
    iVar1->field_0x8e      = 0x0;
    iVar1->field_0x92      = 0x0;
    iVar1->field_0x96      = 0x0;
    *(undefined2 *)param_1 = 0x1e7a;
    iVar1->field_0x2       = 0x1020;
    return;
}
