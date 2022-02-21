
void __stdcall16far pass1_1010_28e6(astruct_631 *param_1, uchar *param_2, ushort param_3, uchar *param_4, ushort param_5)

{
    undefined4  uVar1;
    ushort      uVar2;
    int         iVar3;
    undefined2  uVar4;
    astruct_43 *paVar5;
    int         iStack6;

    struct_op_1018_4cda((int)param_1, (ushort)param_2, param_3);
    *(undefined4 *)&param_1->field_0x1c = 0x0;
    param_1->field_0x20                 = 0x0;
    param_1->field_0x22                 = 0x0;
    param_1->field_0x24                 = 0x0;
    param_1->field_0x26                 = 0x0;
    *(int *)CONCAT22(param_2, param_1)  = (int)s_add16_wav_1050_2bdc + 0x8;
    param_1->field_0x2                  = 0x1010;
    PTR_LOOP_1050_4230                  = (undefined *)param_1;
    PTR_LOOP_1050_4232                  = param_2;
    pass1_1018_4dce((ulong *)CONCAT13((char)((uint)param_2 >> 0x8), CONCAT12((char)param_2, param_1)), 0x56, param_4, param_5);
    paVar5              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x4, param_5);
    PTR_LOOP_1050_5f2e  = (undefined *)((ulong)paVar5 >> 0x10);
    param_1->field_0x1c = (int)paVar5;
    param_1->field_0x1e = PTR_LOOP_1050_5f2e;
    if(_PTR_LOOP_1050_5f2c == 0x0)
    {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e, 0x1000);
    }
    else
    {
    }
    uVar2               = fn_ptr_op_1000_1708(0x40, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
    param_1->field_0x28 = uVar2;
    param_1->field_0x2a = PTR_LOOP_1050_5f2e;
    for(iStack6 = 0x0; iStack6 < 0x10; iStack6 = iStack6 + 0x1)
    {
        paVar5                                       = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, iStack6 + 0x56, param_5);
        uVar1                                        = *(undefined4 *)&param_1->field_0x28;
        uVar4                                        = (undefined2)((ulong)uVar1 >> 0x10);
        iVar3                                        = (int)uVar1;
        *(undefined2 *)(iVar3 + iStack6 * 0x4)       = (int)paVar5;
        *(undefined2 *)(iVar3 + iStack6 * 0x4 + 0x2) = (int)((ulong)paVar5 >> 0x10);
    }
    return;
}

ushort *__stdcall16far unk_load_str_op_1010_2c34(void)

{
    UINT16 *pUVar1;
    uchar  *in_DX;
    short   in_buf_len_5;
    int     unaff_DI;
    ushort  unaff_SS;
    ushort *puVar2;

    puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, unaff_SS, in_DX, unaff_DI);
    mem_op_1000_179c(0x80, (uchar *)((ulong)puVar2 >> 0x10), 0x1000);
    in_buf_len_5 = (short)((ulong)puVar2 >> 0x10);
    load_string_1010_84e0(0x1000, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x80, (char *)puVar2, in_buf_len_5);
    pUVar1 = pass1_1000_3cea((ULONG)puVar2, (ULONG)s__1050_11c8);
    pass1_1010_e964(in_buf_len_5, unaff_SS, unaff_DI);
    pass1_1000_3cea((ULONG)puVar2, CONCAT22(in_buf_len_5, pUVar1));
    return puVar2;
}

void __stdcall16far pass1_1010_32f4(ulong *param_1, ulong *param_2, ushort param_3, ushort param_4)

{
    uint        *puVar1;
    undefined4  *puVar2;
    undefined4   uVar3;
    undefined4   uVar4;
    code       **ppcVar5;
    astruct_65  *paVar6;
    ulong        uVar7;
    uint         uVar8;
    ushort       uVar9;
    uint         uVar10;
    int          iVar11;
    undefined   *extraout_DX;
    undefined2   extraout_DX_00;
    astruct_166 *iVar10;
    int          iVar12;
    int          iVar13;
    undefined2   uVar14;
    undefined2   uVar15;
    undefined2   uVar16;
    uint        *puStack48;
    uint         uStack16;
    int          iStack12;

    uVar14 = (undefined2)((ulong)param_1 >> 0x10);
    iVar10 = (astruct_166 *)param_1;
    if(iVar10->field_0x52 != (astruct_65 *)0x0)
    {
        param_4 = 0x1000;
        fn_ptr_1000_17ce((astruct_18 *)iVar10->field_0x52, 0x1000);
        iVar10->field_0x52 = (astruct_65 *)0x0;
        iVar10->field_0x18 = 0x0;
    }
    uVar8 = param_2._2_2_ | (uint)param_2;
    if((param_2 != (ulong *)0x0) && (ppcVar5 = (code **)((int)*param_1 + 0x24), (**ppcVar5)(param_4, param_1, param_2), uVar8 != 0x0))
    {
        ppcVar5 = (code **)((int)*param_2 + 0x4);
        (**ppcVar5)(param_4, param_2);
        iVar10->field_0x18 = uVar8;
        if(uVar8 != 0x0)
        {
            iVar10->field_0x24 = 0x0;
            iVar10->field_0x26 = 0x0;
            puVar1             = &iVar10->field_0x18;
            *puVar1            = *puVar1 - iVar10->field_0x28;
            if(0xa < (int)iVar10->field_0x18)
            {
                iVar10->field_0x26 = 0x1;
                iVar10->field_0x18 = 0xa;
            }
            if(0x1 < iVar10->field_0x28)
            {
                iVar10->field_0x24 = 0x1;
            }
            if(_PTR_LOOP_1050_5f2c == 0x0)
            {
                PTR_LOOP_1050_5f2e = extraout_DX;
                PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)extraout_DX, 0x1000);
            }
            else
            {
            }
            uVar16                                          = 0x1000;
            uVar9                                           = fn_ptr_op_1000_1708(0x28, 0x0, 0x1, (uint)PTR_LOOP_1050_5f2c, (uint)PTR_LOOP_1050_5f2e, 0x1000);
            *(ushort *)&iVar10->field_0x52                  = uVar9;
            *(undefined2 *)((int)&iVar10->field_0x52 + 0x2) = PTR_LOOP_1050_5f2e;
            if((*(uint *)((int)&iVar10->field_0x52 + 0x2) | *(uint *)&iVar10->field_0x52) != 0x0)
            {
                uVar3    = *(undefined4 *)((uint)param_2 + 0x8);
                iVar11   = iVar10->field_0x10;
                iStack12 = 0x0;
                for(uStack16 = 0x0; puVar1 = &iVar10->field_0x18, *puVar1 != uStack16 && (int)uStack16 <= (int)*puVar1; uStack16 = uStack16 + 0x1)
                {
                    paVar6    = iVar10->field_0x52;
                    uVar8     = (int)paVar6 + uStack16 * 0x4;
                    uVar7     = (ulong)paVar6 & 0xffff0000;
                    puStack48 = (uint *)(uVar7 | uVar8);
                    uVar4     = *(undefined4 *)((iVar10->field_0x28 + uStack16) * 0x4 + (int)uVar3);
                    ppcVar5   = (code **)((int)*param_1 + 0x1c);
                    uVar10    = uStack16;
                    (**ppcVar5)(uVar16, param_1, (int)uVar4, (char)((ulong)uVar4 >> 0x10), iVar10->field_0x22);
                    *puStack48                   = uVar10;
                    *(undefined2 *)(uVar8 + 0x2) = extraout_DX_00;
                    paVar6                       = iVar10->field_0x52;
                    uVar4                        = *(undefined4 *)((int)paVar6 + uStack16 * 0x4);
                    iStack12                     = iStack12 + *(int *)((int)uVar4 + 0x24) + 0x8;
                    if(iVar11 + -0xa < iStack12)
                    {
                        uVar16 = 0x1008;
                        debug_print_1008_6048((ulong)s_overflow_on_node__d_1050_11ca, 0x1008, param_3);
                        iVar10->field_0x18 = uStack16 - 0x1;
                        iVar10->field_0x26 = 0x1;
                        paVar6             = iVar10->field_0x52;
                        uVar15             = (undefined2)((ulong)paVar6 >> 0x10);
                        iVar13             = (int)paVar6;
                        puVar2             = (undefined4 *)*(uint *)(iVar13 + uStack16 * 0x4);
                        uVar8              = *(uint *)(iVar13 + uStack16 * 0x4 + 0x2);
                        if((uVar8 | (uint)puVar2) != 0x0)
                        {
                            ppcVar5 = (code **)*puVar2;
                            (**ppcVar5)(0x1008, puVar2, (char)uVar8, 0x1);
                        }
                        paVar6                                = iVar10->field_0x52;
                        iVar13                                = uStack16 * 0x4;
                        *(undefined4 *)((int)paVar6 + iVar13) = 0x0;
                        if(0x0 < (int)uStack16)
                        {
                            paVar6 = iVar10->field_0x52;
                            uVar15 = (undefined2)((ulong)paVar6 >> 0x10);
                            iVar12 = (int)paVar6;
                            puVar2 = (undefined4 *)*(uint *)(iVar12 + iVar13 + -0x4);
                            uVar8  = *(uint *)(iVar12 + iVar13 + -0x2);
                            if((uVar8 | (uint)puVar2) != 0x0)
                            {
                                ppcVar5 = (code **)*puVar2;
                                (**ppcVar5)(0x1008, puVar2, (char)uVar8, 0x1);
                            }
                            paVar6                                               = iVar10->field_0x52;
                            *(undefined4 *)(uStack16 * 0x4 + (int)paVar6 + -0x4) = 0x0;
                        }
                    }
                }
                iVar10->field_0x20 = 0xa;
                uVar9              = iVar10->field_0x1e;
                mov_update_win_1040_93aa(*(astruct_65 **)iVar10->field_0x52, 0xa, uVar9, (int)&PTR_LOOP_1050_1040);
                for(uStack16 = 0x1; puVar1 = &iVar10->field_0x18, *puVar1 != uStack16 && (int)uStack16 <= (int)*puVar1; uStack16 = uStack16 + 0x1)
                {
                    paVar6 = iVar10->field_0x52;
                    uVar3  = *(undefined4 *)(uStack16 * 0x4 + (int)paVar6 + -0x4);
                    iVar11 = (int)uVar3;
                    uVar16 = (undefined2)((ulong)uVar3 >> 0x10);
                    paVar6 = iVar10->field_0x52;
                    mov_update_win_1040_93aa(*(astruct_65 **)((int)paVar6 + uStack16 * 0x4), *(int *)(iVar11 + 0x20) + *(int *)(iVar11 + 0x24) + 0x8, uVar9, (int)&PTR_LOOP_1050_1040);
                }
            }
        }
    }
    return;
}

void __stdcall16far pass1_1010_16ee(ushort param_1, ushort param_2, ushort param_3, ushort param_4, uint param_5, uchar *param_6)

{
    ushort unaff_SS;

    mem_op_1000_179c(0x4a, param_6, 0x1000);
    if(((uint)param_6 | param_5) != 0x0)
    {
        pass1_1040_c54a((ushort *)CONCAT22(param_6, param_5), 0x0, (ulong *)CONCAT22(param_4, param_3), (ushort)&PTR_LOOP_1050_1040, unaff_SS);
        return;
    }
    return;
}

void __stdcall16far pass1_1010_1788(ushort param_1, ushort param_2, ulong param_3, uchar *param_4, int param_5, ushort param_6)

{
    char      *pcVar1;
    ushort     uVar2;
    ushort    *puVar3;
    undefined4 uVar4;
    uchar     *puVar5;
    int        in_stack_0000fff6;

    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, param_4, param_5);
    puVar5 = (uchar *)0x1778;
    uVar4  = pass1_1028_b58e(param_3);
    uVar2  = (ushort)((ulong)uVar4 >> 0x10);
    pcVar1 = pass1_1010_b038((uchar *)puVar3, (ushort)uVar4, uVar2, puVar5, in_stack_0000fff6);
    str_op_1008_60e8((char *)CONCAT22(uVar2, pcVar1), uVar2);
    return;
}


void __stdcall16far pass1_1010_17c0(ULONG param_1)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_475 *iVar5;
    undefined2   uVar4;
    undefined2   unaff_CS;

    unk_destroy_win_op_1010_2fa0(param_1, unaff_CS);
    uVar4  = (undefined2)(param_1 >> 0x10);
    iVar5  = (astruct_475 *)param_1;
    puVar1 = iVar5->field_0x56;
    uVar2  = iVar5->field_0x58;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    *(undefined4 *)&iVar5->field_0x56 = 0x0;
    fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x60, 0x1000);
    pass1_1000_4906(iVar5->field_0x64, (WNDCLASS16 *)0x0, iVar5->field_0x68 << 0x2);
    fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x64, 0x1000);
    iVar5->field_0x60 = 0x0;
    iVar5->field_0x64 = (astruct_20 *)0x0;
    return;
}

undefined4 __stdcall16far pass1_1010_195e(astruct_79 *param_1, astruct_79 *param_2, ushort param_3)

{
    uchar  *in_DX;
    int     unaff_DI;
    ushort  unaff_SS;
    ushort *puVar1;

    pass1_1010_0f24(param_1, param_2, param_3, in_DX, unaff_SS);
    *(undefined4 *)(param_1 + 0xb)             = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1)  = 0x1b2a;
    param_1->field_0x2                         = 0x1010;
    puVar1                                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, unaff_SS, in_DX, unaff_DI);
    ((astruct_79 *)(param_1 + 0xb))->field_0x0 = (ushort)puVar1;
    param_1[0xb].field_0x2                     = (ushort)((ulong)puVar1 >> 0x10);
    return CONCAT22(param_2, param_1);
}

ulong __stdcall16far pass1_1010_1b6e(astruct_79 *param_1, astruct_79 *param_2, ushort param_3, ushort param_4, uchar *param_5)

{
    int     unaff_DI;
    ushort *puVar1;

    pass1_1010_0f24(param_1, param_2, param_3, param_5, param_4);
    *(undefined4 *)(param_1 + 0xb)             = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1)  = 0x1d04;
    param_1->field_0x2                         = 0x1010;
    puVar1                                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_4, param_5, unaff_DI);
    ((astruct_79 *)(param_1 + 0xb))->field_0x0 = (ushort)puVar1;
    param_1[0xb].field_0x2                     = (ushort)((ulong)puVar1 >> 0x10);
    return CONCAT22(param_2, param_1);
}

void __stdcall16far pass1_1010_1df2(ulong param_1, ushort param_2, ulong param_3, ushort param_4, ushort param_5)

{
    code       **ppcVar1;
    astruct_241 *in_AX;
    uchar       *puVar2;
    uchar       *extraout_DX;
    astruct_242 *iVar3;
    undefined2   uVar3;
    undefined2  *puStack10;
    undefined2  *puStack6;

    uVar3  = (undefined2)(param_1 >> 0x10);
    iVar3  = (astruct_242 *)param_1;
    puVar2 = (uchar *)param_5;
    if(iVar3->field_0x4 == (undefined4 *)0x0)
    {
        mem_op_1000_179c(0xc, (uchar *)param_5, 0x1000);
        puVar2 = (uchar *)(param_5 | (uint)in_AX);
        if(puVar2 == (uchar *)0x0)
        {
            iVar3->field_0x4 = (undefined4 *)0x0;
        }
        else
        {
            set_struct_1008_574a((astruct_21 *)CONCAT22(param_5, in_AX));
            *(astruct_241 **)&iVar3->field_0x4        = in_AX;
            *(uchar **)((int)&iVar3->field_0x4 + 0x2) = extraout_DX;
            puVar2                                    = extraout_DX;
        }
    }
    mem_op_1000_179c(0xa, puVar2, 0x1000);
    puStack10 = (undefined2 *)CONCAT22(puVar2, in_AX);
    if(((uint)puVar2 | (uint)in_AX) == 0x0)
    {
        puStack6 = (undefined2 *)0x0;
    }
    else
    {
        *puStack10       = 0x389a;
        in_AX->field_0x2 = 0x1008;
        in_AX->field_0x4 = param_3;
        in_AX->field_0x8 = param_2;
        *puStack10       = 0x2010;
        in_AX->field_0x2 = 0x1010;
        puStack6         = puStack10;
    }
    ppcVar1 = (code **)((int)*iVar3->field_0x4 + 0x4);
    (**ppcVar1)(0x1000, iVar3->field_0x4, (int)puStack6, (int)((ulong)puStack6 >> 0x10));
    return;
}

ushort *__stdcall16far mixed_1010_20ba(ulong param_1, ushort param_2, ushort param_3, uchar *param_4, int param_5)

{
    code       **ppcVar1;
    undefined2   uVar2;
    uchar       *puVar3;
    uchar       *extraout_DX;
    astruct_636 *paVar4;
    int          iVar5;
    undefined2   uVar6;
    undefined2   uVar7;
    ushort      *puVar8;
    undefined2  *puVar9;
    undefined4   uVar10;
    ulong        uVar11;
    ushort      *puStack6;

    pass1_1010_2816(param_1);
    paVar4   = (astruct_636 *)(param_2 * 0x4);
    uVar6    = (undefined2)(param_1 >> 0x10);
    iVar5    = (int)param_1;
    puStack6 = *(ushort **)(&paVar4->field_0x0 + iVar5);
    if(puStack6 != (ushort *)0x0)
    {
        return puStack6;
    }
    switch(param_2)
    {
    case 0x1:
        mem_op_1000_179c(0x18, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
        {
        LAB_1010_2126:
            paVar4 = (astruct_636 *)0x0;
            puVar3 = (uchar *)0x0;
        }
        else
        {
            struct_1010_3b7a((astruct_648 *)paVar4, (ushort)param_4, param_2);
        }
        break;
    case 0x2:
        mem_op_1000_179c(0x84, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        win_sys_op_1010_5404((astruct_54 *)paVar4, (ushort)param_4, param_2, param_3);
        break;
    case 0x3:
        mem_op_1000_179c(0x53c, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        struct_1010_a1d8((astruct_627 *)paVar4, (ushort)param_4, param_2, param_3);
        break;
    case 0x4:
        mem_op_1000_179c(0x18a, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        struct_1018_2b10((astruct_55 *)CONCAT22(param_4, paVar4), param_2, param_3);
        break;
    case 0x5:
        mem_op_1000_179c(0x14, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2126;
        puVar9 = pass1_1008_eabc((int)paVar4, (ushort)param_4, param_2);
        puVar3 = (uchar *)((ulong)puVar9 >> 0x10);
        paVar4 = (astruct_636 *)puVar9;
        break;
    case 0x6:
        mem_op_1000_179c(0x58, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1018_18b8(param_3, (astruct_55 *)CONCAT22(param_4, paVar4), param_2);
        break;
    case 0x7:
        mem_op_1000_179c(0xe, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2126;
        uVar11 = pass1_1010_3d82((astruct_628 *)paVar4, (ushort)param_4, param_2, param_3);
        puVar3 = (uchar *)(uVar11 >> 0x10);
        paVar4 = (astruct_636 *)uVar11;
        break;
    case 0x8:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        struct_1010_95aa((astruct_629 *)paVar4, (ushort)param_4, param_2);
        break;
    case 0x9:
        mem_op_1000_179c(0x26, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        struct_1010_6326((astruct_630 *)paVar4, (ushort)param_4, param_2);
        break;
    case 0xa:
        mem_op_1000_179c(0x1c, param_4, 0x1000);
        if((uchar *)((uint)param_4 | (uint)paVar4) == (uchar *)0x0)
            goto LAB_1010_2126;
        uVar11 = pass1_1010_0eac((uchar *)paVar4, param_4, param_2, (uchar *)((uint)param_4 | (uint)paVar4), param_3);
        puVar3 = (uchar *)(uVar11 >> 0x10);
        paVar4 = (astruct_636 *)uVar11;
        break;
    case 0xb:
        mem_op_1000_179c(0x1c, param_4, 0x1000);
        if((uchar *)((uint)param_4 | (uint)paVar4) == (uchar *)0x0)
            goto LAB_1010_2126;
        uVar11 = pass1_1008_aefe((uchar *)paVar4, param_4, param_2, (uchar *)((uint)param_4 | (uint)paVar4), param_3);
        puVar3 = (uchar *)(uVar11 >> 0x10);
        paVar4 = (astruct_636 *)uVar11;
        break;
    case 0xc:
    case 0xd:
    case 0xe:
    case 0xf:
    case 0x10:
    case 0x11:
    case 0x12:
    case 0x13:
    case 0x14:
    case 0x15:
    case 0x16:
    case 0x17:
    case 0x18:
    case 0x19:
    case 0x1a:
    case 0x1b:
    case 0x1c:
    case 0x1d:
    case 0x1e:
    case 0x1f:
    case 0x20:
    case 0x21:
    case 0x22:
    case 0x23:
    case 0x24:
        mem_op_1000_179c(0xaa, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        struct_1018_0570((astruct_55 *)CONCAT22(param_4, paVar4), param_2, param_3);
        break;
    case 0x25:
        mem_op_1000_179c(0x1c, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1018_4aaa((int)paVar4, (ushort)param_4, param_2, puVar3, param_3);
        break;
    case 0x26:
        mem_op_1000_179c(0x1c, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1008_d99e((int)paVar4, (ushort)param_4, param_2, puVar3, param_3);
        break;
    case 0x27:
        mem_op_1000_179c(0x58, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1008_9d36((uchar *)paVar4, param_4, param_2, param_3);
        break;
    case 0x28:
        mem_op_1000_179c(0x2c, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1010_28e6((astruct_631 *)paVar4, param_4, param_2, puVar3, param_3);
        break;
    case 0x29:
        mem_op_1000_179c(0x72, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        struct_1018_229c((astruct_632 *)paVar4, param_4, param_2, puVar3, param_3);
        break;
    case 0x2a:
        mem_op_1000_179c(0x1c, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1010_503e((int)paVar4, (ushort)param_4, param_2, puVar3, param_3);
        break;
    case 0x2b:
        mem_op_1000_179c(0x1a, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        struct_1010_02e0((astruct_79 *)paVar4, (astruct_79 *)param_4, param_2);
        break;
    case 0x2c:
        mem_op_1000_179c(0x10, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1008_eb2a((int)paVar4, (ushort)param_4, param_2);
        break;
    case 0x2d:
        mem_op_1000_179c(0x80, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1010_3e3c((astruct_55 *)CONCAT22(param_4, paVar4), param_2, param_3);
        break;
    case 0x2e:
        mem_op_1000_179c(0x806, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2126;
        uVar11 = pass1_1018_1ff4((astruct_634 *)paVar4, (ushort)param_4, param_2);
        puVar3 = (uchar *)(uVar11 >> 0x10);
        paVar4 = (astruct_636 *)uVar11;
        break;
    case 0x2f:
        mem_op_1000_179c(0x58, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        struct_1010_e9e4((astruct_261 *)paVar4, (ushort)param_4, param_2);
        break;
    case 0x30:
        mem_op_1000_179c(0xe, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2126;
        puVar8 = pass1_1010_3702((int)paVar4, (ushort)param_4, param_2);
        puVar3 = (uchar *)((ulong)puVar8 >> 0x10);
        paVar4 = (astruct_636 *)puVar8;
        break;
    case 0x31:
        uVar2 = 0x60;
        uVar7 = 0x1000;
        mem_op_1000_179c(0x60, param_4, 0x1000);
        if((uchar *)((uint)param_4 | (uint)paVar4) == (uchar *)0x0)
        {
        LAB_1010_2680:
            uVar7  = 0x1000;
            paVar4 = (astruct_636 *)0x0;
            puVar3 = (uchar *)0x0;
        }
        else
        {
            uVar11 = pass1_1010_9298((astruct_79 *)paVar4, (astruct_79 *)param_4, param_2, (ushort)paVar4, (uchar *)((uint)param_4 | (uint)paVar4), param_3);
            puVar3 = (uchar *)(uVar11 >> 0x10);
            paVar4 = (astruct_636 *)uVar11;
        }
        goto LAB_1010_2683;
    case 0x32:
        mem_op_1000_179c(0x26, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1010_6abc((astruct_635 *)paVar4, (ushort)param_4, param_2);
        break;
    case 0x33:
        mem_op_1000_179c(0x72, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
        {
        LAB_1010_25b8:
            uVar2 = 0x0;
            uVar7 = 0x0;
        }
        else
        {
            uVar10 = pass1_1010_195e((astruct_79 *)paVar4, (astruct_79 *)param_4, param_2);
            uVar7  = (undefined2)((ulong)uVar10 >> 0x10);
            uVar2  = (undefined2)uVar10;
        }
        goto LAB_1010_25bb;
    case 0x34:
        mem_op_1000_179c(0x72, param_4, 0x1000);
        if((uchar *)((uint)param_4 | (uint)paVar4) == (uchar *)0x0)
            goto LAB_1010_25b8;
        uVar11 = pass1_1010_1b6e((astruct_79 *)paVar4, (astruct_79 *)param_4, param_2, param_3, (uchar *)((uint)param_4 | (uint)paVar4));
        uVar7  = (undefined2)(uVar11 >> 0x10);
        uVar2  = (undefined2)uVar11;
    LAB_1010_25bb:
        puStack6 = (ushort *)CONCAT22(uVar7, uVar2);
        pass1_1010_1146(CONCAT22(uVar7, uVar2), 0x0, param_5, param_3);
        goto switchD_1010_2765_caseD_38;
    case 0x35:
        mem_op_1000_179c(0x14a, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2126;
        uVar11 = pass1_1010_6700(paVar4, (ushort)param_4, param_2);
        puVar3 = (uchar *)(uVar11 >> 0x10);
        paVar4 = (astruct_636 *)uVar11;
        break;
    case 0x36:
        mem_op_1000_179c(0x10, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1008_d790((astruct_647 *)paVar4, (ushort)param_4, param_2, param_3);
        break;
    case 0x37:
        mem_op_1000_179c(0x420, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        struct_1008_9fd2((astruct_79 *)paVar4, (astruct_79 *)param_4, param_2);
        break;
    default:
        goto switchD_1010_2765_caseD_38;
    case 0x39:
        mem_op_1000_179c(0x36, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1010_4a8a((astruct_637 *)paVar4, (ushort)param_4, param_2, param_3);
        break;
    case 0x3a:
        mem_op_1000_179c(0xc, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2126;
        puVar8 = pass1_1008_d72e((int)paVar4, (ushort)param_4, param_2);
        puVar3 = (uchar *)((ulong)puVar8 >> 0x10);
        paVar4 = (astruct_636 *)puVar8;
        break;
    case 0x3b:
        mem_op_1000_179c(0x22, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        struct_1008_dd4e((astruct_209 *)paVar4, (ushort)param_4, param_2);
        break;
    case 0x3c:
        mem_op_1000_179c(0x146, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1018_331c((astruct_638 *)paVar4, (ushort)param_4, param_2, param_3, puVar3);
        break;
    case 0x3d:
        mem_op_1000_179c(0xe, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2126;
        uVar11 = pass1_1010_8c32((astruct_640 *)paVar4, (ushort)param_4, param_2, param_3);
        puVar3 = (uchar *)(uVar11 >> 0x10);
        paVar4 = (astruct_636 *)uVar11;
        break;
    case 0x3e:
        mem_op_1000_179c(0x18, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1018_5070((astruct_641 *)paVar4, (ushort)param_4, param_2);
        break;
    case 0x3f:
        mem_op_1000_179c(0x12, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1008_c72a((astruct_642 *)paVar4, (ushort)param_4, param_2);
        break;
    case 0x40:
        mem_op_1000_179c(0x24, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        pass1_1008_af94((astruct_643 *)paVar4, (ushort)param_4, param_2);
        break;
    case 0x41:
        uVar2 = 0x60;
        mem_op_1000_179c(0x60, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2680;
        uVar7 = 0x1008;
        struct_1008_ecb2((astruct_221 *)paVar4, (ushort)param_4, param_2);
        puVar3 = extraout_DX;
    LAB_1010_2683:
        *(astruct_636 **)(param_2 * 0x4 + iVar5) = paVar4;
        *(uchar **)(param_2 * 0x4 + iVar5 + 0x2) = puVar3;
        ppcVar1                                  = (code **)((int)*(undefined4 *)paVar4 + 0x10);
        (**ppcVar1)(uVar7, paVar4, puVar3, uVar2);
        break;
    case 0x42:
        mem_op_1000_179c(0xc, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2126;
        puVar8 = pass1_1008_ec10((int)paVar4, (ushort)param_4, param_2);
        puVar3 = (uchar *)((ulong)puVar8 >> 0x10);
        paVar4 = (astruct_636 *)puVar8;
        break;
    case 0x43:
        mem_op_1000_179c(0x12, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2126;
        puVar8 = pass1_1010_2bfc((astruct_644 *)paVar4, (ushort)param_4, param_2);
        puVar3 = (uchar *)((ulong)puVar8 >> 0x10);
        paVar4 = (astruct_636 *)puVar8;
        break;
    case 0x45:
        mem_op_1000_179c(0xe, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2126;
        uVar11 = pass1_1010_0000((astruct_645 *)paVar4, (ushort)param_4, param_2, param_3);
        puVar3 = (uchar *)(uVar11 >> 0x10);
        paVar4 = (astruct_636 *)uVar11;
        break;
    case 0x46:
        mem_op_1000_179c(0x1a, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        struct_1010_50b2((astruct_646 *)paVar4, (ushort)param_4, param_2);
        break;
    case 0x47:
        mem_op_1000_179c(0xe, param_4, 0x1000);
        if(((uint)param_4 | (uint)paVar4) == 0x0)
            goto LAB_1010_2126;
        puVar8 = pass1_1018_56e6((int)paVar4, (ushort)param_4, param_2);
        puVar3 = (uchar *)((ulong)puVar8 >> 0x10);
        paVar4 = (astruct_636 *)puVar8;
        break;
    case 0x48:
        mem_op_1000_179c(0x1c, param_4, 0x1000);
        puVar3 = (uchar *)((uint)param_4 | (uint)paVar4);
        if(puVar3 == (uchar *)0x0)
            goto LAB_1010_2126;
        unk_draw_op_1008_da12((astruct_19 *)paVar4, (ushort)param_4, param_2);
    }
    puStack6 = (ushort *)CONCAT22(puVar3, paVar4);
switchD_1010_2765_caseD_38:
    *(ushort **)(param_2 * 0x4 + iVar5) = puStack6;
    return puStack6;
}

void __stdcall16far pass1_1010_043a(ulong param_1, long param_2, int param_3, ushort param_4)

{
    undefined4  *puVar1;
    code       **ppcVar2;
    astruct_225 *puVar3;
    uint         extraout_DX;
    uint         uVar3;
    astruct_226 *iVar4;
    astruct_227 *iVar5;
    undefined2   uVar4;
    undefined2   uVar5;
    undefined2   uVar6;
    undefined2  *puStack18;
    undefined2  *puStack14;
    undefined    local_a[0x8];

    iVar4 = (astruct_226 *)param_1;
    uVar4 = (undefined2)(param_1 >> 0x10);
    if(param_3 == 0xc)
    {
        if(iVar4->field_0xe != 0x0)
        {
            return;
        }
        iVar4->field_0xe = 0x1;
    }
    else
    {
        if(param_3 == 0xd)
        {
            if(iVar4->field_0x10 != 0x0)
            {
                return;
            }
            iVar4->field_0x10 = 0x1;
        }
        else
        {
            if(param_3 == 0x12)
            {
                return;
            }
        }
    }
    pass1_1010_089e(param_4, param_1, 0x1, 0x6);
    pass1_1008_5784((ulong *)CONCAT22(param_4, local_a), (ulong)iVar4->field_0xa);
    do
    {
        puVar3 = (astruct_225 *)local_a;
        pass1_1008_5b12(puVar3, param_4);
        uVar3 = extraout_DX | (uint)puVar3;
        if(uVar3 == 0x0)
        {
            uVar6 = 0xa;
            mem_op_1000_179c(0xa, (uchar *)0x0, 0x1000);
            puStack18 = (undefined2 *)CONCAT22(uVar3, puVar3);
            if((uVar3 | (uint)puVar3) == 0x0)
            {
                puStack14 = (undefined2 *)0x0;
            }
            else
            {
                *puStack18                                     = 0x389a;
                *(undefined2 *)((int)&puVar3->field_0x0 + 0x2) = 0x1008;
                *puStack18                                     = 0xea8;
                *(undefined2 *)((int)&puVar3->field_0x0 + 0x2) = 0x1010;
                puStack14                                      = puStack18;
            }
            uVar5            = (undefined2)((ulong)puStack14 >> 0x10);
            iVar5            = (astruct_227 *)puStack14;
            iVar5->field_0x4 = param_3;
            iVar5->field_0x6 = param_2;
            puVar1           = iVar4->field_0xa;
            ppcVar2          = (code **)((int)*iVar4->field_0xa + 0x8);
            (**ppcVar2)(0x1000, (int)puVar1, (char)((ulong)puVar1 >> 0x10), iVar5, uVar5, uVar6);
            return;
        }
    } while((puVar3->field_0x4 != param_3) || (puVar3->field_0x6 != param_2));
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_0538(ulong param_1, char **param_2, char **param_3, ushort param_4, ushort param_5)

{
    int         iVar1;
    ulong       uVar2;
    code      **ppcVar3;
    uint        uVar4;
    int         iVar5;
    char       *pcVar6;
    uchar      *puVar7;
    uint        extraout_DX;
    uchar      *puVar8;
    uchar      *extraout_DX_00;
    ushort      uVar9;
    ushort      uVar10;
    undefined2  uVar11;
    undefined4  uVar12;
    undefined4 *puStack6;

    uVar4    = 0x0;
    *param_3 = (char *)0x0;
    *param_2 = (char *)0x0;
    uVar10   = (ushort)(param_1 >> 0x10);
    uVar9    = (ushort)param_1;
    uVar12   = *(undefined4 *)(uVar9 + 0xa);
    ppcVar3  = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar9 + 0xa) + 0x10);
    (**ppcVar3)();
    puStack6 = (undefined4 *)CONCAT22(extraout_DX, uVar4);
    puVar8   = (uchar *)(extraout_DX | uVar4);
    if(puVar8 == (uchar *)0x0)
    {
        return;
    }
    iVar1 = *(int *)(uVar4 + 0x4);
    uVar2 = *(ulong *)(uVar4 + 0x6);
    if((extraout_DX | uVar4) != 0x0)
    {
        ppcVar3 = (code **)*puStack6;
        (**ppcVar3)(param_4, uVar4, (char)extraout_DX, 0x1, uVar12);
        puVar8 = extraout_DX_00;
    }
    uVar12 = *(undefined4 *)(uVar9 + 0xa);
    if(*(int *)((int)uVar12 + 0x8) == 0x0)
    {
        pass1_1010_089e(param_5, param_1, 0x0, 0x6);
        pass1_1010_1f62(param_5, param_1, 0x3);
    }
    iVar5 = iVar1 + 0x757;
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), param_4);
    *(int *)param_3                 = iVar5;
    *(uchar **)((int)param_3 + 0x2) = puVar8;
    while(pcVar6 = pass1_1000_472c((ulong)*param_3, '%'), ((uint)puVar8 | (uint)pcVar6) != 0x0)
    {
        pass1_1010_09b4(uVar9, uVar10, (uchar *)CONCAT22(puVar8, pcVar6), param_3, uVar2, puVar8, param_5);
    }
    if(0x1e < iVar1 - 0x1U)
        goto LAB_1010_0850;
    uVar11 = (undefined2)((ulong)param_2 >> 0x10);
    switch(iVar1)
    {
    case 0x1:
        goto LAB_1010_0619;
    case 0x2:
        goto LAB_1010_0619;
    case 0x3:
        break;
    case 0x4:
        goto LAB_1010_0619;
    case 0x5:
        goto LAB_1010_0619;
    case 0x6:
        break;
    case 0x7:
        goto LAB_1010_0619;
    case 0x8:
        goto LAB_1010_0619;
    case 0x9:
        break;
    case 0xa:
        goto LAB_1010_0619;
    case 0xb:
        goto LAB_1010_0619;
    case 0xc:
        break;
    case 0xd:
        goto LAB_1010_0619;
    case 0xe:
        break;
    case 0xf:
        goto LAB_1010_0619;
    case 0x10:
        break;
    case 0x11:
        break;
    case 0x12:
        break;
    case 0x13:
        break;
    case 0x14:
        break;
    case 0x15:
        break;
    case 0x16:
    LAB_1010_0619:
        puVar7 = pass1_1008_5fd8(param_5, puVar8);
        goto LAB_1010_0621;
    case 0x17:
        break;
    case 0x18:
        break;
    case 0x19:
    case 0x1f:
    LAB_1010_0785:
        puVar7 = pass1_1008_5fd8(param_5, puVar8);
        goto LAB_1010_0621;
    case 0x1a:
        goto LAB_1010_0785;
    case 0x1b:
        goto LAB_1010_0785;
    case 0x1c:
        break;
    case 0x1d:
        break;
    case 0x1e:
        puVar7                          = pass1_1008_5fd8(param_5, puVar8);
        *(uchar **)param_2              = puVar7;
        *(uchar **)((int)param_2 + 0x2) = puVar8;
        goto LAB_1010_0785;
    }
    puVar7 = pass1_1008_5fd8(param_5, puVar8);
LAB_1010_0621:
    *(uchar **)param_2              = puVar7;
    *(uchar **)((int)param_2 + 0x2) = puVar8;
LAB_1010_0850:
    while(pcVar6 = pass1_1000_472c((ulong)*param_2, '%'), ((uint)puVar8 | (uint)pcVar6) != 0x0)
    {
        pass1_1010_09b4(uVar9, uVar10, (uchar *)CONCAT22(puVar8, pcVar6), param_2, uVar2, puVar8, param_5);
    }
    return;
}

ulong __stdcall16far pass1_1010_0946(undefined2 param_1, undefined2 param_2, int param_3, uchar *param_4, int param_5, ushort param_6)

{
    ushort *puVar1;
    long    lVar2;
    long    lVar3;

    PTR_LOOP_1050_0ea8 = (undefined *)0x0;
    lVar3              = 0x4000001;
    lVar2              = 0x4000002;
    puVar1             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3b, param_6, param_4, param_5);
    pass1_1008_dfa6((ulong)puVar1, lVar2, lVar3, param_6);
    if((int)puVar1 != 0x0)
    {
        pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), 0x4000002);
        if(*(long *)((int)puVar1 + 0x200) == 0x8000002)
        {
            PTR_LOOP_1050_0ea8 = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
        }
    }
    return CONCAT22(0x1050, (param_3 + -0x1) * 0x8 + 0xea2);
}

void __stdcall16far pass1_1010_09b4(ushort param_1, ushort param_2, uchar *param_3, char **param_4, ulong param_5, uchar *param_6, ushort param_7)

{
    byte       bVar1;
    bool       bVar2;
    bool       bVar3;
    undefined2 uVar4;
    uint       uVar5;
    uint       uVar6;
    ushort     uVar7;
    int        unaff_DI;
    ushort    *puVar8;
    char      *pcStack18;
    undefined4 uStack10;

    bVar3 = false;
    bVar2 = false;
    bVar1 = *(byte *)((int)param_3 + 0x1);
    if(bVar1 == 0x70)
    {
    LAB_1010_0a06:
        bVar3 = false;
        bVar2 = true;
    }
    else
    {
        if(bVar1 < 0x71)
        {
            if(bVar1 != 0x43)
            {
                if(bVar1 == 0x50)
                    goto LAB_1010_0a06;
                if(bVar1 != 0x63)
                    goto LAB_1010_09db;
            }
            bVar3 = true;
            bVar2 = false;
        }
    }
LAB_1010_09db:
    uVar4    = 0x0;
    uStack10 = (char *)0x0;
    if(bVar2)
    {
        puVar8   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_7, param_6, unaff_DI);
        uVar4    = (undefined2)((ulong)puVar8 >> 0x10);
        uStack10 = (char *)CONCAT22(*(undefined2 *)((int)puVar8 + 0x6e), *(undefined2 *)((int)puVar8 + 0x6c));
    }
    else
    {
        if(!bVar3)
            goto LAB_1010_0a36;
        pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), param_5);
        uStack10 = pass1_1038_4d28(CONCAT22(param_6, uVar4));
    }
    param_6 = (uchar *)((ulong)uStack10 >> 0x10);
LAB_1010_0a36:
    if((uStack10._2_2_ | (uint)uStack10) != 0x0)
    {
        uVar5 = str_op_1000_3da4(*param_4);
        uVar6 = str_op_1000_3da4(uStack10);
        uVar7 = uVar6 + 0xa + uVar5;
        mem_op_1000_179c(uVar7, param_6, 0x1000);
        pcStack18 = (char *)CONCAT22(param_6, uVar7);
        *param_3  = '\0';
        unk_str_op_1000_3d3e((char *)CONCAT22(param_6, uVar7), *param_4);
        pass1_1000_3cea(CONCAT22(param_6, uVar7), (ULONG)uStack10);
        pass1_1000_3cea(CONCAT22(param_6, uVar7), (ulong)param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x2));
        fn_ptr_1000_17ce((astruct_18 *)*param_4, 0x1000);
        *param_4 = pcStack18;
    }
    return;
}

void __stdcall16far pass1_1010_11c6(ulong param_1, uint param_2, uchar *param_3)

{
    int         *piVar1;
    uint        *puVar2;
    code       **ppcVar3;
    ulong        uVar4;
    undefined4   uVar5;
    astruct_239 *iVar6;
    int          iVar7;
    int          iVar8;
    uint         uVar9;
    uint         uVar10;
    uchar       *puVar11;
    uchar       *puVar12;
    uchar       *puVar13;
    uchar       *puVar14;
    uchar       *extraout_DX;
    uchar       *extraout_DX_00;
    undefined2   uVar15;
    uchar       *extraout_DX_01;
    uchar       *puVar16;
    ulong       *puVar17;
    astruct_234 *iVar18;
    int          iVar19;
    int          iVar21;
    astruct_238 *iVar20;
    undefined2   uVar22;
    undefined2   uVar23;
    ushort      *puVar24;
    ulong       *puStack50;
    int          iStack42;
    int          iStack40;
    astruct_20  *paStack38;
    int          iStack28;
    ulong       *puStack26;
    ulong       *puStack22;
    undefined4   uStack14;
    ulong        uStack10;

    if(DAT_1050_0ecc == -0x1)
    {
        return;
    }
    mem_op_1000_179c(0x1a, param_3, 0x1000);
    if(((uint)param_3 | param_2) == 0x0)
    {
        iVar6   = (astruct_239 *)0x0;
        puVar11 = (uchar *)0x0;
    }
    else
    {
        puVar24 = pass1_1010_37d4((ushort *)CONCAT22(param_3, param_2));
        puVar11 = (uchar *)((ulong)puVar24 >> 0x10);
        iVar6   = (astruct_239 *)puVar24;
    }
    uStack10 = 0x10500ece;
    uStack14 = 0x0;
    puVar12  = puVar11;
    while(true)
    {
        uVar22 = (undefined2)(param_1 >> 0x10);
        iVar18 = (astruct_234 *)param_1;
        piVar1 = &iVar18->field_0x68;
        if(*piVar1 == (int)uStack14 || *piVar1 < (int)uStack14)
            break;
        uVar5     = iVar18->field_0x64;
        uVar4     = *(ulong *)((int)uVar5 + (int)uStack14 * 0x4);
        puVar17   = (ulong *)((int)uVar4 + DAT_1050_0ecc * 0x8);
        puStack50 = (ulong *)(uVar4 & 0xffff0000 | ZEXT24(puVar17));
        iVar7     = pass1_1000_475e(uStack10, *puVar17);
        if(iVar7 != 0x0)
        {
            uStack10 = *puStack50;
            uStack14 = uStack14 & 0xffff | (ulong)(uStack14._2_2_ + 0x1) << 0x10;
        }
        uStack14 = uStack14 & 0xffff0000 | (ulong)((int)uStack14 + 0x1);
    }
    iVar6->field_0x10 = uStack14._2_2_;
    puVar24           = struct_1010_38f8(CONCAT22(puVar11, iVar6), uStack14._2_2_, uStack14._2_2_, puVar12);
    puVar13           = (uchar *)((ulong)puVar24 >> 0x10);
    iVar8             = 0x0;
    mem_op_1000_179c(0x400, puVar13, 0x1000);
    puVar12 = puVar13;
    iVar7   = iVar8;
    mem_op_1000_179c(0x400, puVar13, 0x1000);
    paStack38 = (astruct_20 *)CONCAT22(puVar12, iVar7);
    iStack28  = 0x0;
    pass1_1000_4906((astruct_20 *)CONCAT22(puVar13, iVar8), (WNDCLASS16 *)0x0, 0x400);
    pass1_1000_4906((astruct_20 *)CONCAT22(puVar12, iVar7), (WNDCLASS16 *)0x0, 0x400);
    iStack42 = 0x0;
    uVar10   = 0x0;
    do
    {
        puVar2 = &iVar6->field_0x10;
        if(*puVar2 == uVar10 || (int)*puVar2 < (int)uVar10)
        {
            return;
        }
        uVar5     = iVar18->field_0x64;
        uVar23    = (undefined2)((ulong)uVar5 >> 0x10);
        iVar19    = (int)uVar5;
        iVar21    = *(int *)(iVar19 + iStack28 * 0x4);
        puVar16   = *(uchar **)(iVar19 + iStack28 * 0x4 + 0x2);
        iVar19    = iVar21 + *(int *)(DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
        puStack22 = (ulong *)CONCAT22(puVar16, iVar19);
        uVar9     = iVar21 + *(int *)(DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
        puVar14   = puVar16;
        mem_op_1000_179c(0x1a, puVar16, 0x1000);
        if(((uint)puVar14 | uVar9) == 0x0)
        {
            uVar5                                      = iVar6->field_0x8;
            *(undefined4 *)((int)uVar5 + uVar10 * 0x4) = 0x0;
        }
        else
        {
            puVar24                                      = pass1_1010_37d4((ushort *)CONCAT22(puVar14, uVar9));
            uVar5                                        = iVar6->field_0x8;
            uVar23                                       = (undefined2)((ulong)uVar5 >> 0x10);
            iVar21                                       = (int)uVar5;
            *(undefined2 *)(iVar21 + uVar10 * 0x4)       = (int)puVar24;
            *(undefined2 *)(iVar21 + uVar10 * 0x4 + 0x2) = (int)((ulong)puVar24 >> 0x10);
        }
        iStack42 = iStack42 + 0x1;
        uVar5    = iVar6->field_0x8;
        uVar23   = (undefined2)((ulong)uVar5 >> 0x10);
        iVar21   = (int)uVar5;
        uVar5    = *(undefined4 *)(iVar21 + uVar10 * 0x4);
        ppcVar3  = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar21 + uVar10 * 0x4) + 0x1c);
        (**ppcVar3)(0x1000, (int)uVar5, (int)((ulong)uVar5 >> 0x10), iStack42, iVar19, puVar16);
        uStack14 = (ulong)uVar10;
        puVar16  = extraout_DX;
        while(true)
        {
            piVar1 = &iVar18->field_0x68;
            if(*piVar1 == iStack28 || *piVar1 < iStack28)
                break;
            iVar19 = iStack28 * 0x4;
            uVar5  = iVar18->field_0x64;
            uVar5  = *(undefined4 *)((int)uVar5 + iVar19);
            iVar21 = pass1_1000_475e(*puStack22, *(ulong *)((int)uVar5 + *(int *)(DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
            if(iVar21 != 0x0)
                break;
            uVar5     = iVar18->field_0x64;
            uVar23    = (undefined2)((ulong)uVar5 >> 0x10);
            iVar21    = (int)uVar5;
            puVar16   = *(uchar **)(iVar21 + iVar19 + 0x2);
            uVar10    = *(int *)(iVar21 + iVar19) + *(int *)(DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
            puStack26 = (ulong *)CONCAT22(puVar16, uVar10);
            mem_op_1000_179c(0x1a, puVar16, 0x1000);
            if(((uint)puVar16 | uVar10) == 0x0)
            {
                uVar23 = 0x0;
                uVar15 = 0x0;
            }
            else
            {
                puVar24 = pass1_1010_37d4((ushort *)CONCAT22(puVar16, uVar10));
                uVar15  = (undefined2)((ulong)puVar24 >> 0x10);
                uVar23  = SUB42(puVar24, 0x0);
            }
            *(undefined2 *)(uStack14._2_2_ * 0x4 + iVar8)       = uVar23;
            *(undefined2 *)(uStack14._2_2_ * 0x4 + iVar8 + 0x2) = uVar15;
            uVar5                                               = iVar18->field_0x64;
            uVar23                                              = (undefined2)((ulong)uVar5 >> 0x10);
            iVar21                                              = (int)uVar5;
            iStack42                                            = iStack42 + 0x1;
            uVar5                                               = *(undefined4 *)(uStack14._2_2_ * 0x4 + iVar8);
            ppcVar3                                             = (code **)((int)*(undefined4 *)*(undefined4 *)(uStack14._2_2_ * 0x4 + iVar8) + 0x1c);
            (**ppcVar3)(0x1000, (int)uVar5, (int)((ulong)uVar5 >> 0x10), iStack42, *(int *)(iVar21 + iStack28 * 0x4) + *(int *)(DAT_1050_0ecc * 0x6 + 0xebc) * 0x8, *(undefined2 *)(iVar21 + iStack28 * 0x4 + 0x2));
            iStack40 = 0x0;
            puVar16  = extraout_DX_00;
            while(true)
            {
                piVar1 = &iVar18->field_0x68;
                if(*piVar1 == iStack28 || *piVar1 < iStack28)
                    break;
                uVar5  = iVar18->field_0x64;
                uVar5  = *(undefined4 *)((int)uVar5 + iStack28 * 0x4);
                iVar21 = pass1_1000_475e(*puStack26, *(ulong *)((int)uVar5 + *(int *)(DAT_1050_0ecc * 0x6 + 0xebc) * 0x8));
                if(iVar21 != 0x0)
                    break;
                uVar5  = iVar18->field_0x64;
                uVar5  = *(undefined4 *)((int)uVar5 + iStack28 * 0x4);
                uVar10 = pass1_1000_475e(*puStack22, *(ulong *)((int)uVar5 + *(int *)(DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
                if(uVar10 != 0x0)
                    break;
                mem_op_1000_179c(0x1a, puVar16, 0x1000);
                if(((uint)puVar16 | uVar10) == 0x0)
                {
                    uVar23 = 0x0;
                    uVar15 = 0x0;
                }
                else
                {
                    puVar24 = pass1_1010_37d4((ushort *)CONCAT22(puVar16, uVar10));
                    uVar15  = (undefined2)((ulong)puVar24 >> 0x10);
                    uVar23  = SUB42(puVar24, 0x0);
                }
                *(undefined2 *)(iStack40 * 0x4 + iVar7)       = uVar23;
                *(undefined2 *)(iStack40 * 0x4 + iVar7 + 0x2) = uVar15;
                uVar5                                         = iVar18->field_0x64;
                uVar23                                        = (undefined2)((ulong)uVar5 >> 0x10);
                iVar20                                        = (astruct_238 *)uVar5;
                iStack42                                      = iStack42 + 0x1;
                uVar5                                         = *(undefined4 *)(iStack40 * 0x4 + iVar7);
                ppcVar3                                       = (code **)((int)*(undefined4 *)*(undefined4 *)(iStack40 * 0x4 + iVar7) + 0x1c);
                (**ppcVar3)(0x1000, (int)uVar5, (int)((ulong)uVar5 >> 0x10), iStack42, *(int *)(iVar20 + iStack28 * 0x4) + *(int *)(DAT_1050_0ecc * 0x6 + 0xebe) * 0x8, *(undefined2 *)(iVar20 + iStack28 * 0x4 + 0x2));
                iStack28 = iStack28 + 0x1;
                iStack40 = iStack40 + 0x1;
                puVar16  = extraout_DX_01;
            }
            uVar5                       = *(undefined4 *)(uStack14._2_2_ * 0x4 + iVar8);
            *(int *)((int)uVar5 + 0x10) = iStack40;
            uVar10                      = iStack40 << 0x2;
            iVar21                      = iVar7;
            puVar14                     = puVar12;
            puVar24                     = struct_1010_38f8(*(ulong *)(uStack14._2_2_ * 0x4 + iVar8), iStack40, uVar10, puVar16);
            puVar16                     = (uchar *)((ulong)puVar24 >> 0x10);
            pass1_1000_48a8((ulong)puVar24, CONCAT22(puVar14, iVar21), uVar10);
            pass1_1000_4906(paStack38, (WNDCLASS16 *)0x0, 0x400);
            uStack14 = uStack14 & 0xffff | (ulong)(uStack14._2_2_ + 0x1) << 0x10;
        }
        uVar5                       = iVar6->field_0x8;
        uVar5                       = *(undefined4 *)((int)uVar5 + (int)uStack14 * 0x4);
        *(int *)((int)uVar5 + 0x10) = uStack14._2_2_;
        uVar10                      = uStack14._2_2_ << 0x2;
        uVar5                       = iVar6->field_0x8;
        iVar21                      = iVar8;
        puVar14                     = puVar13;
        puVar24                     = struct_1010_38f8(*(ulong *)((int)uVar5 + (int)uStack14 * 0x4), uStack14._2_2_, uVar10, puVar16);
        pass1_1000_48a8((ulong)puVar24, CONCAT22(puVar14, iVar21), uVar10);
        pass1_1000_4906((astruct_20 *)CONCAT22(puVar13, iVar8), (WNDCLASS16 *)0x0, 0x400);
        uVar10 = (int)uStack14 + 0x1;
    } while(true);
}

ulong __stdcall16far string_1008_e586(ushort param_1, ushort param_2, ulong param_3, uint param_4, uint param_5)

{
    uint   uVar1;
    uchar *puVar2;
    char  *in_string_2;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_3, (uint)(param_3 >> 0x10));
    puVar2 = (uchar *)(param_5 | param_4);
    if(puVar2 == (uchar *)0x0)
    {
        return 0x0;
    }
    uVar1 = param_4;
    mem_op_1000_179c(0x80, puVar2, 0x1000);
    in_string_2 = pass1_1038_4d28(CONCAT22(param_5, param_4));
    unk_str_op_1000_3d3e((char *)CONCAT22(puVar2, uVar1), in_string_2);
    return CONCAT22(puVar2, uVar1);
}

void __stdcall16far pass1_1008_e9a4(ushort param_1, ushort param_2, ulong param_3, int param_4, ushort param_5)

{
    uint      *puVar1;
    uint       uVar2;
    uint       uVar3;
    uchar     *puVar4;
    int        iVar5;
    undefined2 uVar6;
    ulong      uVar7;
    int        iStack20;
    ulong      uStack16;
    ulong      uStack6;

    uVar7   = pass1_1030_8326();
    uVar6   = (undefined2)(param_3 >> 0x10);
    iVar5   = (int)param_3;
    puVar1  = (uint *)(iVar5 + 0xe);
    uVar2   = (uint)uVar7 - *puVar1;
    puVar4  = (uchar *)(((int)(uVar7 >> 0x10) - *(int *)(iVar5 + 0x10)) - (uint)((uint)uVar7 < *puVar1));
    uStack6 = CONCAT22(puVar4, uVar2);
    mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_5, puVar4, param_4);
    uStack16 = 0x0;
    if(((int)PTR_LOOP_1050_13ae < 0x1) || (SBORROW2((int)PTR_LOOP_1050_13ae, 0x1)))
        goto LAB_1008_ea2b;
    if(PTR_LOOP_1050_13ae == (undefined *)&PTR_LOOP_1050_0002 || (int)(PTR_LOOP_1050_13ae + -0x1) < 0x1)
    {
        if(*(int *)(iVar5 + 0x12) == 0x0)
        {
        LAB_1008_ea20:
            uVar3 = 0x1e;
        }
        else
        {
            uVar3 = 0xa;
        }
    }
    else
    {
        if(PTR_LOOP_1050_13ae == (undefined *)((int)&PTR_LOOP_1050_0002 + 0x1))
        {
            if(*(int *)(iVar5 + 0x12) == 0x0)
            {
                uVar3 = 0x28;
            }
            else
            {
                uVar3 = 0x14;
            }
        }
        else
        {
            if(PTR_LOOP_1050_13ae != (undefined *)&DAT_1050_0004)
                goto LAB_1008_ea2b;
            if(*(int *)(iVar5 + 0x12) != 0x0)
                goto LAB_1008_ea20;
            uVar3 = 0x32;
        }
    }
    uStack16 = (ulong)uVar3;
LAB_1008_ea2b:
    if(uStack16 < uStack6)
    {
        pass1_1008_612e(0x1, 0x64, uVar2);
        iStack20 = 0x0;
        iVar5    = *(int *)(iVar5 + 0xc);
        if(iVar5 == 0x2)
        {
            iStack20 = 0x32;
        }
        else
        {
            if(iVar5 == 0x3)
            {
                iStack20 = 0x19;
            }
        }
        if((int)uStack6 < iStack20)
        {
            return;
        }
    }
    return;
}

void __stdcall16far pass1_1008_eb74(undefined4 param_1, int param_2, uchar *param_3, int param_4, ushort param_5)

{
    *(int *)((int)param_1 + 0xa) = param_2;
    if(param_2 != 0x0)
    {
        mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_5, param_3, param_4);
        pass1_1010_c312();
    }
    return;
}

undefined4 __stdcall16far struct_1008_ecb2(astruct_221 *param_1, ushort param_2, ushort param_3)

{
    uint       in_AX;
    uchar     *in_DX;
    undefined2 unaff_SS;

    struct_1010_2cd2((astruct_79 *)param_1, (astruct_79 *)param_2, param_3, unaff_SS);
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xef9c;
    param_1->field_0x2                        = 0x1008;
    mem_op_1000_179c(0x20c, in_DX, 0x1000);
    param_1->field_0x5c = in_AX;
    param_1->field_0x5e = (int)in_DX;
    pass1_1000_4906((astruct_20 *)CONCAT22(in_DX, param_1->field_0x5c), (WNDCLASS16 *)0x0, 0x20c);
    return CONCAT22(param_2, param_1);
}

void __stdcall16far mem_1008_ed1e(ushort param_1, ushort param_2, int param_3, uint param_4, uchar *param_5)

{
    if(param_3 != 0x0)
    {
        mem_op_1000_179c(param_3 << 0x2, param_5, 0x1000);
        return;
    }
    mem_op_1000_179c(0x1a, param_5, 0x1000);
    if(((uint)param_5 | param_4) != 0x0)
    {
        struct_1008_ec72((ushort *)CONCAT22(param_5, param_4));
        return;
    }
    return;
}

void __stdcall16far pass1_1008_ed8a(ulong *param_1, uint param_2, int param_3, int param_4, int param_5, int param_6, ushort param_7)

{
    code     **ppcVar1;
    char       cVar2;
    uint       uVar3;
    uint       uVar4;
    bool       bVar5;
    undefined4 uVar6;
    ulong      uVar7;

    if(0x0 < param_4)
    {
        if(_PTR_LOOP_1050_0df6 == (ushort *)0x0)
        {
            ppcVar1             = (code **)((int)*param_1 + 0x18);
            uVar6               = (**ppcVar1)();
            _PTR_LOOP_1050_0df6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, (ushort)uVar6, param_7, (uchar *)((ulong)uVar6 >> 0x10), param_6);
        }
        uVar6 = *(undefined4 *)((int)param_1 + 0xc);
        uVar7 = pass1_1010_2e02((ulong)_PTR_LOOP_1050_0df6, *(int *)((int)uVar6 + 0x12));
        uVar3 = param_2 + 0x1;
        uVar4 = param_3 + (uint)(0xfffe < param_2);
        for(cVar2 = ((char)param_4 + -0x1) * '\x04'; cVar2 != '\0'; cVar2 = cVar2 + -0x1)
        {
            bVar5 = CARRY2(uVar3, uVar3);
            uVar3 = uVar3 * 0x2;
            uVar4 = uVar4 * 0x2 + (uint)bVar5;
        }
        pass1_1010_2e30((ulong)_PTR_LOOP_1050_0df6, uVar3 | (uint)uVar7, uVar4 | (uint)(uVar7 >> 0x10), *(int *)(param_5 * 0x8 + 0xd64));
    }
    return;
}
