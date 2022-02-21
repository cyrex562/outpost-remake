
void __cdecl16far pass1_1010_abd2(ushort param_1, ushort param_2, int param_3, uchar *param_4, int param_5, ushort param_6)

{
    bool    bVar1;
    int    *piVar2;
    ushort *puVar3;
    int     iStack20;
    int     iStack16;
    int     iStack14;

    puVar3    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x35, param_6, param_4, param_5);
    bVar1     = false;
    iStack16  = param_3;
    _iStack20 = (int *)CONCAT22(param_6, &stack0x000a);
    while(true)
    {
        piVar2 = _iStack20;
        if(iStack16 == 0x0)
        {
            return;
        }
        if(bVar1)
            break;
        if(*(int *)(iStack16 * 0x2 + (int)puVar3 + 0xa) != 0x0)
        {
            bVar1    = true;
            iStack14 = iStack16;
        }
        _iStack20 = (int *)((ulong)_iStack20 & 0xffff0000 | (ulong)(iStack20 + 0x2));
        iStack16  = *piVar2;
    }
    pass1_1010_682e((ulong)puVar3, 0x0, iStack14);
    pass1_1010_682e((ulong)puVar3, 0x1, iStack16);
    return;
}

void __stdcall16far pass1_1010_ae92(ulong param_1, ushort param_2, uint param_3, ulong param_4, int param_5, ushort param_6)

{
    ushort      uVar1;
    uchar      *puVar2;
    ulong       uVar3;
    astruct_67 *paVar4;
    undefined2  uVar5;
    undefined2  uVar6;
    int         iVar7;
    undefined   uVar8;
    undefined   uVar9;
    undefined2  uVar10;
    undefined2  uVar11;
    int         iVar12;

    if(param_3 == 0x15)
    {
        uVar3 = struct_op_1030_73a8(param_4);
        if(uVar3 != 0x0)
        {
            *(ushort *)((int)uVar3 + 0x20) = param_2;
            return;
        }
    }
    else
    {
        if(param_3 < 0x16)
        {
            if((char)param_3 == '\x06')
            {
                pass1_1030_7f1a(param_4, param_2, param_6);
                uVar3  = struct_op_1030_73a8(param_4);
                uVar1  = pass1_1010_b028((ushort)param_1, (ushort)(param_1 >> 0x10), uVar3);
                uVar3  = pass1_1030_8326();
                puVar2 = (uchar *)(uVar3 >> 0x10);
                if(((uVar1 == 0xe) && ((puVar2 != (uchar *)0x0 || (0x32 < (uint)uVar3)))) && ((param_2 == 0x1 || (((param_2 == 0x2 || (param_2 == 0x4)) || (param_2 == 0x3))))))
                {
                    uVar11 = 0x0;
                    iVar12 = 0xb;
                    uVar8  = 0x1;
                    uVar9  = 0x0;
                    uVar10 = 0x0;
                    uVar6  = 0x0;
                    iVar7  = 0x0;
                    uVar5  = 0x0;
                    paVar4 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_6, puVar2, param_5);
                    post_win_msg_1008_a0e4(paVar4, CONCAT22(uVar6, uVar5), iVar7, CONCAT11(uVar9, uVar8), CONCAT22(uVar11, uVar10), iVar12, 0x1008, param_6);
                    return;
                }
            }
            else
            {
                if((char)param_3 == '\a')
                {
                    pass1_1030_7eda(param_4, param_2, param_6);
                    return;
                }
            }
        }
    }
    return;
}

ulong __stdcall16far pass1_1010_8c32(astruct_640 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int         unaff_DI;
    astruct_79 *paVar1;
    ushort     *puVar2;

    paVar1                                    = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)&param_1->field_0xa        = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x8ee2;
    param_1->field_0x2                        = 0x1010;
    puVar2                                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_4, (uchar *)((ulong)paVar1 >> 0x10), unaff_DI);
    param_1->field_0xa                        = (int)puVar2;
    param_1->field_0xc                        = (int)((ulong)puVar2 >> 0x10);
    return CONCAT22(param_2, param_1);
}


ulong __stdcall16far unk_load_str_op_1010_8c96(ulong param_1, ulong param_2, ulong param_3, ushort param_4, ushort param_5)

{
    undefined4  uVar1;
    INT16       IVar2;
    undefined4 *puVar3;
    int         iVar4;
    uint        uVar5;
    uchar      *in_DX;
    int         iVar6;
    undefined2  uVar7;
    undefined2  uVar8;
    uchar       in_AF;
    ulong       uVar9;
    char       *pcVar10;
    LPCSTR      spec;
    WORD       *valist;
    ulong       uStack46;
    undefined4  local_10;
    int         iStack12;
    int         iStack10;
    uchar      *puStack8;
    undefined2  uStack6;
    undefined2  uStack4;

    uVar7  = (undefined2)(param_3 >> 0x10);
    iVar6  = (int)param_3;
    uVar5  = *(uint *)(iVar6 + 0x6);
    uVar9  = (ulong)uVar5;
    spec   = (LPCSTR)param_2;
    valist = (WORD *)(param_2 >> 0x10);
    if(uVar5 != 0x0)
    {
        uVar8 = (undefined2)(param_1 >> 0x10);
        if(uVar5 == 0x1)
        {
            uVar9   = param_3 & 0xffff;
            iVar4   = (int)uVar9;
            param_4 = 0x1010;
            switch(*(int *)(iVar6 + 0x4) + -0x1)
            {
            case 0x0:
            case 0x1:
                uVar1 = *(undefined4 *)(iVar6 + 0x8);
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
                local_10 = *(undefined4 *)(iVar4 + 0xc);
                iStack12 = *(int *)(iVar4 + 0x10);
                iStack10 = iVar4;
                if(0x0 < iStack12)
                {
                    pcVar10 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), (HINSTANCE16)&USHORT_1050_1028);
                    uStack4 = (undefined2)((ulong)pcVar10 >> 0x10);
                    uStack6 = SUB42(pcVar10, 0x0);
                    IVar2   = wsprintf16((LPSTR)&USHORT_1050_1028, spec, valist);
                    return CONCAT22(IVar2, uStack4);
                }
                break;
            case 0x2:
                uVar1 = *(undefined4 *)(iVar6 + 0x8);
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
                local_10 = *(undefined4 *)(iVar4 + 0xc);
                iStack12 = *(int *)(iVar4 + 0x10);
                if(0x0 < iStack12)
                {
                    iStack12 = 0x0;
                    uVar9    = struct_op_1030_73a8(CONCAT22(in_DX, iVar4));
                    uVar9    = pass1_1028_bb24(uVar9);
                    in_DX    = (uchar *)(uVar9 >> 0x10);
                    iStack10 = (int)uVar9;
                    puVar3   = &local_10;
                    puStack8 = in_DX;
                    pass1_1030_627e(param_5, (uint)puVar3, (uint)in_DX, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_5, puVar3), uVar9);
                    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)puVar3, (uint)in_DX);
                    uVar1 = *(undefined4 *)((int)param_1 + 0xa);
                    pass1_1010_c3c2((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), 0x0, CONCAT22(in_DX, puVar3), in_DX, in_AF, param_5);
                    uStack46 = CONCAT22(in_DX, puVar3);
                    pcVar10  = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), (HINSTANCE16)&USHORT_1050_1028);
                    uStack4  = (undefined2)((ulong)pcVar10 >> 0x10);
                    uStack6  = SUB42(pcVar10, 0x0);
                    wsprintf16((LPSTR)&USHORT_1050_1028, spec, valist);
                    goto LAB_1010_8def;
                }
                break;
            default:
                goto switchD_1010_8e11_caseD_4;
            case 0x4:
            case 0x7:
            case 0x8:
            case 0xa:
                goto LAB_1010_8ea5;
            }
            uVar9   = ZEXT24(&local_10);
            param_4 = (ushort)&USHORT_1050_1028;
        }
        else
        {
            uVar9 = (ulong)(uVar5 - 0x2);
            if(uVar5 - 0x2 == 0x0)
            {
                iVar4 = *(int *)(iVar6 + 0x4);
                uVar5 = iVar4 - 0x4;
                if(uVar5 != 0x0)
                {
                    uVar5 = iVar4 - 0xc;
                    uVar9 = (ulong)uVar5;
                    if(uVar5 != 0x0)
                        goto LAB_1010_8ea5;
                }
                uVar1 = *(undefined4 *)(iVar6 + 0x8);
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
                uVar1 = *(undefined4 *)((int)param_1 + 0xa);
                pass1_1010_c3c2((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), 0x0, CONCAT22(in_DX, uVar5), in_DX, in_AF, param_5);
                uStack46 = CONCAT22(in_DX, uVar5);
                pcVar10  = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), (HINSTANCE16)&USHORT_1050_1028);
                uStack4  = (undefined2)((ulong)pcVar10 >> 0x10);
                uStack6  = SUB42(pcVar10, 0x0);
                wsprintf16((LPSTR)&USHORT_1050_1028, spec, valist);
            LAB_1010_8def:
                fn_ptr_1000_17ce((astruct_18 *)(uStack46 & 0xffff | ZEXT24(in_DX) << 0x10), 0x1000);
                return CONCAT22((int)uStack46, in_DX);
            }
        }
    }
LAB_1010_8ea5:
    load_string_1010_84e0(param_4, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, spec, (short)valist);
switchD_1010_8e11_caseD_4:
    return CONCAT22((int)uVar9, in_DX);
}

void __stdcall16far pass1_1010_8ef2(ushort *param_1, uchar *param_2, ushort param_3)

{
    uint         uVar1;
    uchar       *puVar2;
    uchar       *extraout_DX;
    astruct_170 *iVar3;
    int          unaff_DI;
    undefined2   uVar3;
    ushort      *puVar4;

    uVar3                            = (undefined2)((ulong)param_1 >> 0x10);
    iVar3                            = (astruct_170 *)param_1;
    *param_1                         = 0x389a;
    iVar3->field_0x2                 = 0x1008;
    uVar1                            = 0x0;
    *(undefined4 *)&iVar3->field_0x4 = 0x0;
    *(undefined4 *)&iVar3->field_0x8 = 0x0;
    *param_1                         = 0x9254;
    iVar3->field_0x2                 = 0x1010;
    mem_op_1000_179c(0x18, param_2, 0x1000);
    puVar2 = (uchar *)((uint)param_2 | uVar1);
    if(puVar2 == (uchar *)0x0)
    {
        *(undefined4 *)&iVar3->field_0x4 = 0x0;
    }
    else
    {
        struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_2, uVar1), 0x5, 0x5);
        iVar3->field_0x4 = uVar1;
        iVar3->field_0x6 = extraout_DX;
        puVar2           = extraout_DX;
    }
    puVar4           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_3, puVar2, unaff_DI);
    iVar3->field_0x8 = (int)puVar4;
    iVar3->field_0xa = (int)((ulong)puVar4 >> 0x10);
    return;
}

ulong __stdcall16far pass1_1010_9298(astruct_79 *param_1, astruct_79 *param_2, ushort param_3, ushort param_4, uchar *param_5, ushort param_6)

{
    struct_1010_2cd2(param_1, param_2, param_3, param_6);
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x9566;
    param_1->field_0x2                        = 0x1010;
    mem_op_1000_179c(0x20c, param_5, 0x1000);
    param_1[0x9].field_0x2             = param_4;
    *(uchar **)&param_1[0x9].field_0x4 = param_5;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5, param_1[0x9].field_0x2), (WNDCLASS16 *)0x0, 0x20c);
    return CONCAT22(param_2, param_1);
}

void __stdcall16far pass1_1010_9304(ushort param_1, ushort param_2, int param_3, uint param_4, uchar *param_5)

{
    if(param_3 != 0x0)
    {
        mem_op_1000_179c(param_3 << 0x2, param_5, 0x1000);
        return;
    }
    mem_op_1000_179c(0x1a, param_5, 0x1000);
    if(((uint)param_5 | param_4) != 0x0)
    {
        pass1_1010_9258((ushort *)CONCAT22(param_5, param_4));
        return;
    }
    return;
}

void __stdcall16far pass1_1010_9372(ulong *param_1, uint param_2, int param_3, int param_4, int param_5)

{
    code     **ppcVar1;
    char       cVar2;
    uint       uVar3;
    uint       uVar4;
    int        unaff_DI;
    ushort     unaff_SS;
    bool       bVar5;
    undefined4 uVar6;
    ulong      uVar7;

    if(0x0 < param_4)
    {
        if(_PTR_LOOP_1050_3528 == (ushort *)0x0)
        {
            ppcVar1             = (code **)((int)*param_1 + 0x18);
            uVar6               = (**ppcVar1)();
            _PTR_LOOP_1050_3528 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, (ushort)uVar6, unaff_SS, (uchar *)((ulong)uVar6 >> 0x10), unaff_DI);
        }
        uVar6 = *(undefined4 *)((int)param_1 + 0xc);
        uVar7 = pass1_1010_2e02((ulong)_PTR_LOOP_1050_3528, *(int *)((int)uVar6 + 0x12));
        uVar3 = param_2 + 0x1;
        uVar4 = param_3 + (uint)(0xfffe < param_2);
        for(cVar2 = ((char)param_4 + -0x1) * '\x04'; cVar2 != '\0'; cVar2 = cVar2 + -0x1)
        {
            bVar5 = CARRY2(uVar3, uVar3);
            uVar3 = uVar3 * 0x2;
            uVar4 = uVar4 * 0x2 + (uint)bVar5;
        }
        pass1_1010_2e30((ulong)_PTR_LOOP_1050_3528, uVar3 | (uint)uVar7, uVar4 | (uint)(uVar7 >> 0x10), param_5);
    }
    return;
}

void __stdcall16far pass1_1010_9794(ulong param_1, ushort param_2)

{
    int          iVar1;
    code       **ppcVar2;
    undefined4  *puVar3;
    uint         uVar4;
    astruct_251 *puVar5;
    undefined4  *puVar6;
    uchar       *puVar7;
    uchar       *extraout_DX;
    undefined2   extraout_DX_00;
    undefined2   uVar8;
    uint         extraout_DX_01;
    astruct_250 *iVar9;
    undefined2   uVar9;
    undefined8   local_a;

    uVar9 = (undefined2)(param_1 >> 0x10);
    iVar9 = (astruct_250 *)param_1;
    if(iVar9->field_0x18 == 0x0)
    {
        iVar9->field_0x18 = 0x1;
        puVar6            = iVar9->field_0xe;
        uVar4             = *(uint *)((int)&iVar9->field_0xe + 0x2);
        puVar7            = (uchar *)(uVar4 | (uint)(undefined4 *)puVar6);
        if(puVar7 != (uchar *)0x0)
        {
            ppcVar2 = (code **)*(undefined4 *)puVar6;
            (**ppcVar2)();
            puVar7 = extraout_DX;
        }
        mem_op_1000_179c(0xc, puVar7, 0x1000);
        uVar4 = (uint)puVar6;
        if(((uint)puVar7 | uVar4) == 0x0)
        {
            uVar4 = 0x0;
            uVar8 = 0x0;
        }
        else
        {
            set_struct_1008_574a((astruct_21 *)((ulong)puVar6 & 0xffff | ZEXT24(puVar7) << 0x10));
            uVar8 = extraout_DX_00;
        }
        *(uint *)&iVar9->field_0xe                    = uVar4;
        *(undefined2 *)((int)&iVar9->field_0xe + 0x2) = uVar8;
        pass1_1008_5784((ulong *)CONCAT22(param_2, &local_a), (ulong)iVar9->field_0xa);
        while(true)
        {
            puVar5 = (astruct_251 *)&local_a;
            pass1_1008_5b12(puVar5, param_2);
            if((extraout_DX_01 | (uint)puVar5) == 0x0)
                break;
            iVar1 = puVar5->field_0x4;
            if((iVar1 == 0x3e) || (iVar1 == 0x41))
            {
                puVar6                             = iVar9->field_0xa;
                *(undefined2 *)((int)puVar6 + 0xa) = 0x0;
                puVar6                             = iVar9->field_0xa;
                ppcVar2                            = (code **)((int)*iVar9->field_0xa + 0xc);
                (**ppcVar2)();
                puVar3                             = iVar9->field_0xa;
                *(undefined2 *)((int)puVar3 + 0xa) = 0x1;
                local_a._4_4_                      = 0x0;
                ppcVar2                            = (code **)((int)*iVar9->field_0xe + 0x4);
                (**ppcVar2)(0x1008, iVar9->field_0xe, CONCAT22(extraout_DX_01, puVar5), puVar6);
            }
        }
    }
    return;
}

void __stdcall16far pass1_1010_866c(ushort param_1, ushort param_2, ushort param_3, ulong param_4, uint param_5)

{
    ulong uVar1;
    char  cVar2;
    int   iVar3;
    bool  bVar4;

    if((int)param_5 < 0x28)
    {
        if(((int)param_5 < 0x25) && (param_5 != 0x23))
        {
            if(0x23 < param_5)
            {
                return;
            }
            cVar2 = (char)param_5;
            if(((cVar2 != '\v') && (cVar2 != '\x0f')) && (cVar2 != '!'))
            {
                return;
            }
        }
    }
    else
    {
        if(param_5 != 0x37)
        {
            if((int)param_5 < 0x38)
            {
                if((int)param_5 < 0x33)
                {
                    return;
                }
                bVar4 = SBORROW2(param_5 - 0x33, 0x1);
                iVar3 = param_5 - 0x34;
            }
            else
            {
                if(param_5 == 0x49)
                    goto LAB_1010_8691;
                if((int)(param_5 - 0x49) < 0x2a)
                {
                    return;
                }
                bVar4 = SBORROW2(param_5 - 0x73, 0x5);
                iVar3 = param_5 - 0x78;
            }
            if(iVar3 != 0x0 && bVar4 == iVar3 < 0x0)
            {
                return;
            }
        }
    }
LAB_1010_8691:
    uVar1 = *(ulong *)(param_5 * 0x4 + (int)param_4);
    memcpy_op_1008_676e(uVar1, (int)uVar1, param_1);
    return;
}

void __stdcall16far pass1_1010_878c(astruct_87 **param_1, int param_2, HINSTANCE16 param_3)

{
    uint        uVar1;
    uint        uVar2;
    ushort      uVar4;
    uchar      *puVar3;
    uchar      *puVar4;
    astruct_87 *uVar6;
    int         iVar5;
    ushort      uVar7;
    ushort      unaff_SS;
    astruct_87 *paVar8;
    astruct_87 *paVar9;

    uVar7 = (ushort)((ulong)param_1 >> 0x10);
    uVar6 = (astruct_87 *)param_1;
    if(uVar6->field_0x680 == param_2)
    {
        return;
    }
    uVar1  = uVar6->field_0x67c;
    puVar4 = uVar6->field_0x67e;
    puVar3 = (uchar *)((uint)puVar4 | uVar1);
    uVar2  = uVar1;
    if(puVar3 != (uchar *)0x0)
    {
        pass1_1008_64a2((uint *)CONCAT22(puVar4, uVar1));
        param_3 = 0x1000;
        fn_ptr_1000_17ce((astruct_18 *)CONCAT22(puVar4, uVar1), 0x1000);
    }
    if((param_2 == 0x1) || (param_2 == 0x2))
    {
        mem_op_1000_179c(0x8, puVar3, 0x1000);
        puVar4 = (uchar *)((uint)puVar3 | uVar2);
        if(puVar4 == (uchar *)0x0)
        {
            *(undefined4 *)&uVar6->field_0x67c = 0x0;
            goto LAB_1010_8869;
        }
        paVar8 = *param_1;
        paVar9 = (astruct_87 *)CONCAT22(puVar3, uVar2);
    LAB_1010_8853:
        uVar4 = (ushort)paVar9;
        file_1008_6414((ulong *)paVar9, (ulong)paVar8, unaff_SS, puVar4);
    }
    else
    {
        iVar5  = param_2 * 0x4;
        paVar8 = (astruct_87 *)set_err_mode_1010_8b14((ulong)param_1, *(ULONG *)(iVar5 + 0x172a), unaff_SS);
        paVar9 = paVar8;
        if((*(int *)(iVar5 + 0x172a) == (int)paVar8) && (*(int *)(iVar5 + 0x172c) == (int)((ulong)paVar8 >> 0x10)))
        {
            msg_box_op_1010_8bb4((ushort)uVar6, uVar7, (ulong)paVar8, param_3, unaff_SS);
        }
        mem_op_1000_179c(0x8, (uchar *)((ulong)paVar9 >> 0x10), 0x1000);
        puVar4 = (uchar *)((uint)((ulong)paVar9 >> 0x10) | (uint)paVar9);
        if(paVar9 != (astruct_87 *)0x0)
            goto LAB_1010_8853;
        uVar4  = 0x0;
        puVar4 = (uchar *)0x0;
    }
    uVar6->field_0x67c = uVar4;
    uVar6->field_0x67e = puVar4;
LAB_1010_8869:
    uVar6->field_0x680 = param_2;
    return;
}

void __stdcall16far pass1_1010_6abc(astruct_635 *param_1, ushort param_2, ushort param_3)

{
    code      **ppcVar1;
    uchar      *extraout_DX;
    int         unaff_DI;
    ushort      unaff_SS;
    astruct_79 *paVar2;
    ushort     *puVar3;

    paVar2                                           = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                               = 0x389a;
    param_1->field_0xc                               = 0x1008;
    param_1->field_0xa                               = 0x3aa8;
    param_1->field_0xc                               = 0x1008;
    param_1->field_0xe                               = 0x0;
    param_1->field_0x10                              = 0x0;
    param_1->field_0x14                              = (undefined4 *)0x0;
    param_1->field_0x1c                              = 0x0;
    param_1->field_0x20                              = 0x0;
    param_1->field_0x22                              = (undefined4 *)0x0;
    *(undefined2 *)CONCAT22(param_2, param_1)        = 0x7e28;
    param_1->field_0x2                               = 0x1010;
    param_1->field_0xa                               = 0x7e38;
    param_1->field_0xc                               = 0x1010;
    puVar3                                           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, unaff_SS, (uchar *)((ulong)paVar2 >> 0x10), unaff_DI);
    *(int *)&param_1->field_0x14                     = (int)puVar3;
    *(undefined2 *)((int)&param_1->field_0x14 + 0x2) = (int)((ulong)puVar3 >> 0x10);
    ppcVar1                                          = (code **)((int)*param_1->field_0x14 + 0x4);
    (**ppcVar1)();
    puVar3                                           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, unaff_SS, extraout_DX, unaff_DI);
    *(int *)&param_1->field_0x22                     = (int)puVar3;
    *(undefined2 *)((int)&param_1->field_0x22 + 0x2) = (int)((ulong)puVar3 >> 0x10);
    ppcVar1                                          = (code **)((int)*param_1->field_0x22 + 0x4);
    (**ppcVar1)();
    return;
}

ushort pass1_1010_6cf8(ushort param_1, ulong param_2, int param_3, ushort param_4, ushort param_5, ushort param_6, ushort param_7)

{
    ushort uVar1;

    switch(param_3)
    {
    case 0x1:
        pass1_1010_715c(param_2, 0x1, param_6, param_5, param_7, param_4);
        send_msg_1010_7c9e(param_2, 0x12, param_4);
        return 0x1;
    default:
        return 0x0;
    case 0x4:
        uVar1 = 0x2;
        break;
    case 0x5:
        uVar1 = 0x3;
        break;
    case 0x6:
        uVar1 = 0x4;
        break;
    case 0x7:
        uVar1 = 0x5;
        break;
    case 0x9:
        pass1_1010_715c(param_2, 0x6, param_6, param_5, param_7, param_4);
    case 0x2e:
        uVar1 = 0x38;
        break;
    case 0xa:
    case 0x80:
        uVar1 = 0x2d;
        break;
    case 0xb:
        uVar1 = 0x7;
        break;
    case 0xc:
    case 0x17:
    case 0x18:
    case 0x19:
    case 0x21:
    case 0x75:
    case 0x81:
        if(param_3 == 0x75)
        {
            pass1_1010_715c(param_2, 0x8, param_6, param_5, param_7, param_4);
            pass1_1010_715c(param_2, 0x9, param_6, param_5, param_7, param_4);
        }
        uVar1 = pass1_1010_6ca2(param_2, 0x7, param_5, param_4);
        if(uVar1 != 0x0)
        {
            pass1_1010_715c(param_2, 0x10, uVar1, param_5, param_7, param_4);
        }
        param_6 = pass1_1010_6ca2(param_2, 0x3, param_5, param_4);
        if(param_6 != 0x0)
        {
            pass1_1010_715c(param_2, 0x11, param_6, param_5, param_7, param_4);
        }
        if(param_3 == 0x21)
        {
            pass1_1010_715c(param_2, 0x14, param_6, param_5, param_7, param_4);
        }
        if(param_3 != 0xc)
        {
            return 0x1;
        }
        uVar1 = 0x9;
        goto code_r0x10106d4c;
    case 0xe:
        uVar1 = 0xc;
        goto code_r0x10106d4c;
    case 0x10:
    case 0x11:
    case 0x13:
        uVar1 = 0xd;
        break;
    case 0x12:
        uVar1 = 0xe;
        break;
    case 0x1b:
    case 0x1f:
    case 0x5b:
    case 0x78:
    case 0x7e:
    case 0x7f:
        if(param_3 == 0x7e)
        {
            pass1_1010_715c(param_2, 0x2c, param_6, param_5, param_7, param_4);
        }
        if(param_3 == 0x5b)
        {
            pass1_1010_715c(param_2, 0x38, param_6, param_5, param_7, param_4);
        }
        if(param_3 == 0x1f)
        {
            pass1_1010_715c(param_2, 0x3f, param_6, param_5, param_7, param_4);
        }
        if(param_3 == 0x7f)
        {
            pass1_1010_715c(param_2, 0x42, param_6, param_5, param_7, param_4);
        }
        param_6 = pass1_1010_6ca2(param_2, 0x5, param_5, param_4);
        if((param_6 == 0x0) && (param_6 = pass1_1010_6ca2(param_2, 0x5, param_5, param_4), param_6 == 0x0))
        {
            return 0x1;
        }
        uVar1 = 0x37;
        break;
    case 0x1d:
    case 0x2a:
    case 0x2c:
    case 0x3c:
    case 0x3d:
    case 0x4b:
    case 0x53:
    case 0x54:
    case 0x55:
    case 0x5a:
        uVar1 = pass1_1010_6ca2(param_2, 0x2, param_5, param_4);
        if(uVar1 != 0x0)
        {
            pass1_1010_715c(param_2, 0x12, uVar1, param_5, param_7, param_4);
        }
        uVar1 = pass1_1010_6ca2(param_2, 0x8, param_5, param_4);
        if(uVar1 != 0x0)
        {
            pass1_1010_715c(param_2, 0x1a, uVar1, param_5, param_7, param_4);
        }
        if(param_3 == 0x2c)
        {
            pass1_1010_715c(param_2, 0x1d, uVar1, param_5, param_7, param_4);
        }
        param_6 = pass1_1010_6ca2(param_2, 0x2, param_5, param_4);
        if(param_6 == 0x0)
        {
            return 0x1;
        }
        uVar1 = 0x1c;
        break;
    case 0x22:
        uVar1 = 0x15;
        break;
    case 0x25:
        uVar1 = 0x16;
        break;
    case 0x26:
        pass1_1010_715c(param_2, 0x17, param_6, param_5, param_7, param_4);
    case 0x1e:
        uVar1 = 0x13;
        break;
    case 0x27:
        uVar1 = 0x18;
        break;
    case 0x29:
        uVar1 = 0x19;
        break;
    case 0x2b:
        uVar1 = 0x1b;
        break;
    case 0x2f:
    case 0x36:
        param_6 = pass1_1010_6ca2(param_2, 0x2, param_5, param_4);
        if(param_6 == 0x0)
        {
            return 0x0;
        }
        uVar1 = 0x1e;
        break;
    case 0x30:
        uVar1 = 0x1f;
        break;
    case 0x31:
        uVar1 = 0x35;
        break;
    case 0x33:
        uVar1 = 0x21;
        break;
    case 0x34:
        uVar1 = 0x22;
        break;
    case 0x35:
        pass1_1010_715c(param_2, 0x23, param_6, param_5, param_7, param_4);
    case 0x65:
    case 0x66:
    case 0x6b:
    case 0x6c:
    case 0x6d:
    case 0x6e:
    case 0x6f:
        uVar1 = 0x34;
        break;
    case 0x38:
        pass1_1010_715c(param_2, 0x24, param_6, param_5, param_7, param_4);
        uVar1 = 0x3d;
        break;
    case 0x39:
        uVar1 = 0x25;
        break;
    case 0x3e:
        pass1_1010_715c(param_2, 0x26, param_6, param_5, param_7, param_4);
        pass1_1010_715c(param_2, 0x28, param_6, param_5, param_7, param_4);
        uVar1 = 0x27;
        break;
    case 0x40:
        uVar1 = 0x2a;
        break;
    case 0x41:
        uVar1 = 0x39;
        break;
    case 0x42:
        uVar1 = 0x3a;
        break;
    case 0x44:
        uVar1 = 0x36;
        break;
    case 0x45:
        uVar1 = 0x3b;
        break;
    case 0x49:
        uVar1 = 0x29;
        break;
    case 0x50:
        uVar1 = 0x2b;
        break;
    case 0x56:
        pass1_1010_715c(param_2, 0x3c, param_6, param_5, param_7, param_4);
        uVar1 = 0x3e;
        break;
    case 0x5d:
        pass1_1010_715c(param_2, 0x2f, param_6, param_5, param_7, param_4);
        uVar1 = 0x40;
        break;
    case 0x5e:
    case 0x60:
        uVar1 = 0x2f;
        break;
    case 0x5f:
        pass1_1010_715c(param_2, 0x34, param_6, param_5, param_7, param_4);
        uVar1 = 0x41;
        break;
    case 0x61:
        uVar1 = 0x30;
        break;
    case 0x63:
        uVar1 = 0x31;
        break;
    case 0x64:
        uVar1 = 0x24;
        break;
    case 0x68:
        uVar1 = 0x32;
        break;
    case 0x69:
        uVar1 = 0x33;
        break;
    case 0x76:
        uVar1 = 0xa;
    code_r0x10106d4c:
        pass1_1010_715c(param_2, uVar1, param_6, param_5, param_7, param_4);
        uVar1 = 0xb;
    }
    pass1_1010_715c(param_2, uVar1, param_6, param_5, param_7, param_4);
    return 0x1;
}

ulong __stdcall16far string_1010_5286(ushort param_1, ushort param_2, ulong param_3, char *param_4, uint param_5)

{
    char  *in_buffer_4;
    uchar *in_buf_len_5;
    char  *pcVar1;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_3, (uint)(param_3 >> 0x10));
    in_buf_len_5 = (uchar *)(param_5 | (uint)param_4);
    if(in_buf_len_5 == (uchar *)0x0)
    {
        return 0x0;
    }
    in_buffer_4 = param_4;
    mem_op_1000_179c(0x80, in_buf_len_5, 0x1000);
    load_string_1010_84e0(0x1000, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x80, in_buffer_4, (short)in_buf_len_5);
    pass1_1000_3cea(CONCAT22(in_buf_len_5, in_buffer_4), 0x105013ac);
    pcVar1 = pass1_1038_4d28(CONCAT22(param_5, param_4));
    pass1_1000_3cea(CONCAT22(in_buf_len_5, in_buffer_4), (ULONG)pcVar1);
    return CONCAT22(in_buf_len_5, in_buffer_4);
}

void __stdcall16far win_sys_op_1010_5404(astruct_54 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int        *piVar1;
    uint      **ppuVar2;
    undefined4  uVar3;
    undefined4 *puVar4;
    code      **ppcVar5;
    LPCSTR      pCVar6;
    int         iVar7;
    ushort      uVar8;
    uint       *puVar9;
    uint        uVar10;
    uchar      *puVar11;
    uchar      *extraout_DX;
    uchar      *puVar12;
    uchar      *extraout_DX_00;
    uchar      *extraout_DX_01;
    undefined  *puVar13;
    undefined2 *puVar14;
    int         unaff_DI;
    undefined2  uVar15;
    LPCSTR      pCVar16;
    INT16       index;
    astruct_79 *paVar17;
    char       *pcVar18;
    ushort     *puVar19;
    undefined2  uVar20;
    undefined   local_134[0x102];
    undefined2 *puStack50;
    ushort      uStack46;
    uchar      *puStack44;
    int         iStack42;
    int         iStack26;
    uchar      *puStack24;
    int         iStack22;
    undefined2 *puStack20;
    undefined4  uStack16;
    int         iStack12;
    int         iStack10;
    undefined2  uStack8;
    uchar      *puStack6;
    uint        uStack4;

    paVar17                                   = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    puVar12                                   = (uchar *)((ulong)paVar17 >> 0x10);
    uVar15                                    = 0x0;
    *(undefined4 *)&param_1->field_0xa        = 0x0;
    param_1->field_0xe                        = (char *)0x0;
    param_1->field_0x12                       = 0x0;
    param_1->field_0x16                       = 0x0;
    *(undefined4 *)&param_1->field_0x1a       = 0x0;
    param_1->field_0x62                       = 0x0;
    param_1->field_0x64                       = (undefined4 *)0x0;
    *(undefined4 *)&param_1->field_0x68       = 0x0;
    *(undefined4 *)&param_1->field_0x6c       = 0x0;
    param_1->field_0x70                       = 0x1;
    param_1->field_0x7a                       = 0x0;
    param_1->field_0x7c                       = 0x0;
    param_1->field_0x7e                       = 0x0;
    param_1->field_0x80                       = 0x0;
    param_1->field_0x82                       = 0x1;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x6312;
    param_1->field_0x2                        = 0x1010;
    pass1_1010_6034(CONCAT22(param_2, param_1), (ushort)puVar12);
    mem_op_1000_179c(0x101, puVar12, 0x1000);
    *(undefined2 *)&param_1->field_0xe          = uVar15;
    *(uchar **)((int)&param_1->field_0xe + 0x2) = puVar12;
    pass1_1000_5008(*(ushort *)&param_1->field_0xe, (ushort)puVar12, 0x100, (int)&stack0xfffe);
    uStack4 = str_op_1000_3da4(param_1->field_0xe);
    pcVar18 = param_1->field_0xe;
    uVar15  = (undefined2)((ulong)pcVar18 >> 0x10);
    puVar13 = (undefined *)((int)pcVar18 + uStack4);
    if(puVar13[-0x1] != '\\')
    {
        *puVar13                                     = 0x5c;
        pcVar18                                      = param_1->field_0xe;
        *(undefined *)((int)pcVar18 + uStack4 + 0x1) = 0x0;
    }
    pcVar18  = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1000);
    puVar12  = (uchar *)((ulong)pcVar18 >> 0x10);
    uStack8  = SUB42(pcVar18, 0x0);
    puStack6 = puVar12;
    pass1_1000_3cea((ULONG)param_1->field_0xe, (ULONG)pcVar18);
    pCVar6             = (LPCSTR)str_op_1008_60e8(param_1->field_0xe, (ushort)puVar12);
    param_1->field_0xa = pCVar6;
    param_1->field_0xc = puVar12;
    pcVar18            = param_1->field_0xe;
    pCVar6             = (LPCSTR)s_tile2_bmp_1050_1538;
    GetPrivateProfileString16((LPCSTR)0x1008, param_1->field_0xa, (LPCSTR)puVar12, (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    if(*param_1->field_0xe != '\0')
    {
        pCVar6              = (LPCSTR)&PTR_LOOP_1050_1000;
        iStack22            = pass1_1000_3e2c((ulong)param_1->field_0xe);
        puVar19             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, puVar12, unaff_DI);
        puVar12             = (uchar *)((ulong)puVar19 >> 0x10);
        iStack26            = (int)puVar19;
        iStack10            = *(int *)(iStack26 + 0xa);
        iStack12            = *(int *)(iStack26 + 0xc);
        param_1->field_0x62 = (uint)(iStack22 != iStack10);
        puStack24           = puVar12;
    }
    pcVar18 = param_1->field_0xe;
    uVar3   = *(undefined4 *)&param_1->field_0xa;
    pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar6, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    if(*param_1->field_0xe != '\0')
    {
        pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
        iVar7   = pass1_1000_475e((ulong)param_1->field_0xe, 0x105013c4);
        if(iVar7 == 0x0)
        {
            param_1->field_0x80 = 0x1;
        }
    }
    pcVar18 = param_1->field_0xe;
    uVar3   = *(undefined4 *)&param_1->field_0xa;
    pCVar6  = (LPCSTR)s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar16, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    if(*param_1->field_0xe != '\0')
    {
        pCVar6 = (LPCSTR)&PTR_LOOP_1050_1000;
        iVar7  = pass1_1000_475e((ulong)param_1->field_0xe, 0x105013c8);
        if(iVar7 == 0x0)
        {
            param_1->field_0x74 = 0x0;
        }
    }
    pcVar18 = param_1->field_0xe;
    uVar3   = *(undefined4 *)&param_1->field_0xa;
    pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar6, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    if(*param_1->field_0xe != '\0')
    {
        pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
        iVar7   = pass1_1000_475e((ulong)param_1->field_0xe, 0x105013c8);
        if(iVar7 == 0x0)
        {
            param_1->field_0x72 = 0x0;
        }
    }
    pcVar18 = param_1->field_0xe;
    uVar3   = *(undefined4 *)&param_1->field_0xa;
    pCVar6  = (LPCSTR)s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar16, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    if(*param_1->field_0xe != '\0')
    {
        pCVar6 = (LPCSTR)&PTR_LOOP_1050_1000;
        iVar7  = pass1_1000_475e((ulong)param_1->field_0xe, 0x105013c8);
        if(iVar7 == 0x0)
        {
            param_1->field_0x1e = 0x0;
        }
    }
    pcVar18 = param_1->field_0xe;
    uVar3   = *(undefined4 *)&param_1->field_0xa;
    pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar6, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    if(*param_1->field_0xe != '\0')
    {
        pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
        iVar7   = pass1_1000_475e((ulong)param_1->field_0xe, 0x105013c8);
        if(iVar7 == 0x0)
        {
            param_1->field_0x20 = 0x0;
        }
    }
    pcVar18 = param_1->field_0xe;
    uVar3   = *(undefined4 *)&param_1->field_0xa;
    pCVar6  = (LPCSTR)s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar16, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    puVar11 = puVar12;
    if(*param_1->field_0xe != '\0')
    {
        pCVar6    = (LPCSTR)&PTR_LOOP_1050_1000;
        uStack46  = pass1_1000_3e2c((ulong)param_1->field_0xe);
        puVar11   = (uchar *)((uint)puVar12 | uStack46);
        puStack44 = puVar12;
        if((uchar *)((uint)puVar12 | uStack46) != (uchar *)0x0)
        {
            param_1->field_0x76 = uStack46;
            param_1->field_0x78 = puVar12;
            puVar11             = puVar12;
        }
    }
    pcVar18 = param_1->field_0xe;
    uVar3   = *(undefined4 *)&param_1->field_0xa;
    pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar6, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    if(*param_1->field_0xe != '\0')
    {
        pCVar16 = (LPCSTR)&PTR_LOOP_1050_1000;
        iVar7   = pass1_1000_475e((ulong)param_1->field_0xe, 0x105013c4);
        if(iVar7 == 0x0)
        {
            param_1->field_0x7a = 0x1;
        }
    }
    pcVar18 = param_1->field_0xe;
    uVar3   = *(undefined4 *)&param_1->field_0xa;
    pCVar6  = (LPCSTR)s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar16, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    if(*param_1->field_0xe != '\0')
    {
        pCVar6              = (LPCSTR)0x1008;
        uVar8               = str_op_1008_60e8(param_1->field_0xe, (ushort)puVar11);
        param_1->field_0x1a = uVar8;
        param_1->field_0x1c = puVar11;
    }
    pcVar18 = param_1->field_0xe;
    uVar3   = *(undefined4 *)&param_1->field_0xa;
    pCVar16 = (LPCSTR)s_tile2_bmp_1050_1538;
    GetPrivateProfileString16(pCVar6, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    if(*param_1->field_0xe != '\0')
    {
        pCVar16             = (LPCSTR)0x1008;
        uVar8               = str_op_1008_60e8(param_1->field_0xe, (ushort)puVar11);
        param_1->field_0x68 = uVar8;
        param_1->field_0x6a = puVar11;
    }
    pcVar18 = param_1->field_0xe;
    uVar3   = *(undefined4 *)&param_1->field_0xa;
    index   = (INT16)s_tile2_bmp_1050_1538;
    puVar9  = (uint *)GetPrivateProfileString16(pCVar16, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    if(*param_1->field_0xe != '\0')
    {
        index               = 0x1008;
        puVar9              = (uint *)str_op_1008_60e8(param_1->field_0xe, (ushort)puVar11);
        param_1->field_0x6c = (undefined *)puVar9;
        param_1->field_0x6e = puVar11;
    }
    if(param_1->field_0x62 == 0x0)
    {
        uVar15   = SUB42(s_tile2_bmp_1050_1538, 0x0);
        uStack46 = GetSystemMetrics16(index);
        iStack42 = 0x1;
        do
        {
            get_private_profile_string_1010_6132(CONCAT22(param_2, param_1), iStack42, uVar15);
            puVar14 = &param_1->field_0x0 + iStack42 * 0x4;
            if(((((int)puVar14[0x11] < 0x0) || ((int)puVar14[0x12] < 0x0)) || (piVar1 = puVar14 + 0x11, *piVar1 != iStack10 - uStack46 && (int)(iStack10 - uStack46) <= *piVar1))
               || (puVar9 = (uint *)(iStack12 - uStack46), ppuVar2 = (uint **)(puVar14 + 0x12), *ppuVar2 != puVar9 && (int)puVar9 <= (int)*ppuVar2))
            {
                uVar15 = 0x1000;
                puVar9 = pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0x22 + iStack42 * 0x8), (WNDCLASS16 *)0x0, 0x8);
            }
            iStack42 = iStack42 + 0x1;
        } while(iStack42 < 0x8);
    }
    mem_op_1000_179c(0xc, puVar11, 0x1000);
    puStack50 = (undefined2 *)CONCAT22(puVar11, puVar9);
    if(((uint)puVar11 | (uint)puVar9) == 0x0)
    {
        puVar9  = (uint *)0x0;
        puVar12 = (uchar *)0x0;
    }
    else
    {
        set_struct_1008_574a((astruct_21 *)CONCAT22(puVar11, puVar9));
        puVar12 = extraout_DX;
    }
    *(uint **)&param_1->field_0x64               = puVar9;
    *(uchar **)((int)&param_1->field_0x64 + 0x2) = puVar12;
    pcVar18                                      = param_1->field_0xe;
    pass1_1000_5008((ushort)pcVar18, (ushort)((ulong)pcVar18 >> 0x10), 0x100, (int)&stack0xfffe);
    uStack4 = str_op_1000_3da4(param_1->field_0xe);
    pcVar18 = param_1->field_0xe;
    uVar15  = (undefined2)((ulong)pcVar18 >> 0x10);
    puVar13 = (undefined *)((int)pcVar18 + uStack4);
    if(puVar13[-0x1] != '\\')
    {
        *puVar13                                     = 0x5c;
        pcVar18                                      = param_1->field_0xe;
        *(undefined *)((int)pcVar18 + uStack4 + 0x1) = 0x0;
    }
    uVar10   = str_op_1008_60e8(param_1->field_0xe, (ushort)puVar12);
    uStack16 = CONCAT22(puVar12, uVar10);
    mem_op_1000_179c(0x8, puVar12, 0x1000);
    puStack50 = (undefined2 *)CONCAT22(puVar12, uVar10);
    if(((uint)puVar12 | uVar10) == 0x0)
    {
        puStack20 = (undefined2 *)0x0;
    }
    else
    {
        *puStack50                    = 0x389a;
        *(undefined2 *)(uVar10 + 0x2) = 0x1008;
        *(undefined4 *)(uVar10 + 0x4) = uStack16;
        *puStack50                    = 0x6322;
        *(undefined2 *)(uVar10 + 0x2) = 0x1010;
        puStack20                     = puStack50;
    }
    puVar4  = param_1->field_0x64;
    ppcVar5 = (code **)((int)*param_1->field_0x64 + 0x4);
    (**ppcVar5)(0x1000, (int)puVar4, (int)((ulong)puVar4 >> 0x10), (int)puStack20, (int)((ulong)puStack20 >> 0x10));
    pcVar18 = param_1->field_0xe;
    uVar3   = *(undefined4 *)&param_1->field_0xa;
    puVar12 = extraout_DX_00;
    GetPrivateProfileString16((LPCSTR)&PTR_LOOP_1050_1000, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)pcVar18, (LPCSTR)((ulong)pcVar18 >> 0x10));
    if(*param_1->field_0xe != '\0')
    {
        pcVar18 = param_1->field_0xe;
        uVar15  = SUB42(pcVar18, 0x0);
        uVar20  = (undefined2)((ulong)pcVar18 >> 0x10);
        while(uStack46 = pass1_1000_47a4(CONCAT22(uVar20, uVar15), 0x105013f8, param_4), ((uint)puVar12 | uStack46) != 0x0)
        {
            puStack44 = puVar12;
            unk_str_op_1000_3d3e((char *)CONCAT22(param_4, local_134), (char *)CONCAT22(puVar12, uStack46));
            uStack4 = str_op_1000_3da4((char *)CONCAT22(param_4, local_134));
            if(local_134[uStack4 - 0x1] != '\\')
            {
                local_134[uStack4]       = 0x5c;
                local_134[uStack4 + 0x1] = 0x0;
            }
            uVar10   = str_op_1008_60e8((char *)CONCAT22(param_4, local_134), (ushort)puVar12);
            uStack16 = CONCAT22(puVar12, uVar10);
            mem_op_1000_179c(0x8, puVar12, 0x1000);
            puStack50 = (undefined2 *)CONCAT22(puVar12, uVar10);
            if(((uint)puVar12 | uVar10) == 0x0)
            {
                puStack20 = (undefined2 *)0x0;
            }
            else
            {
                *puStack50                    = 0x389a;
                *(undefined2 *)(uVar10 + 0x2) = 0x1008;
                *(undefined4 *)(uVar10 + 0x4) = uStack16;
                *puStack50                    = 0x6322;
                *(undefined2 *)(uVar10 + 0x2) = 0x1010;
                puStack20                     = puStack50;
            }
            puVar4  = param_1->field_0x64;
            ppcVar5 = (code **)((int)*param_1->field_0x64 + 0x8);
            (**ppcVar5)(0x1000, (int)puVar4, (int)((ulong)puVar4 >> 0x10), (int)puStack20, (int)((ulong)puStack20 >> 0x10));
            uVar15  = 0x0;
            uVar20  = 0x0;
            puVar12 = extraout_DX_01;
        }
    }
    return;
}

void __stdcall16far write_private_profile_str_1010_5b10(ushort *param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    undefined4  uVar3;
    code      **ppcVar4;
    LPCSTR      pCVar5;
    uchar      *in_DX;
    int         iVar6;
    int         unaff_DI;
    undefined2  uVar7;
    ushort      unaff_SS;
    undefined   in_AF;
    ushort     *puVar8;
    int         iStack12;

    uVar7                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar6                        = (int)param_1;
    *param_1                     = 0x6312;
    *(undefined2 *)(iVar6 + 0x2) = 0x1010;
    puVar8                       = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    uVar3                        = *(undefined4 *)(iVar6 + 0xe);
    sys_1000_3f9c((uchar *)uVar3, (uchar *)((ulong)uVar3 >> 0x10), 0x149c, (ushort)&USHORT_1050_1050, *(ushort *)((int)puVar8 + 0xa), &stack0xfffe, uVar7, 0x1000, unaff_SS, in_AF);
    if(*(int *)(iVar6 + 0x80) == 0x0)
    {
        pCVar5 = (LPCSTR)0x13c8;
    }
    else
    {
        pCVar5 = (LPCSTR)0x13c4;
    }
    uVar3 = *(undefined4 *)(iVar6 + 0xa);
    WritePrivateProfileString16((LPCSTR)&PTR_LOOP_1050_1000, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), pCVar5);
    uVar3 = *(undefined4 *)(iVar6 + 0xa);
    WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPCSTR) * (undefined4 *)(iVar6 + 0xe));
    if(*(int *)(iVar6 + 0x1e) == 0x0)
    {
        pCVar5 = (LPCSTR)0x13c8;
    }
    else
    {
        pCVar5 = (LPCSTR)0x13c4;
    }
    uVar3 = *(undefined4 *)(iVar6 + 0xa);
    WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), pCVar5);
    if(*(int *)(iVar6 + 0x74) == 0x0)
    {
        pCVar5 = (LPCSTR)0x13c8;
    }
    else
    {
        pCVar5 = (LPCSTR)0x13c4;
    }
    uVar3 = *(undefined4 *)(iVar6 + 0xa);
    WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), pCVar5);
    if(*(int *)(iVar6 + 0x72) == 0x0)
    {
        pCVar5 = (LPCSTR)0x13c8;
    }
    else
    {
        pCVar5 = (LPCSTR)0x13c4;
    }
    uVar3 = *(undefined4 *)(iVar6 + 0xa);
    WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), pCVar5);
    if(*(int *)(iVar6 + 0x20) == 0x0)
    {
        pCVar5 = (LPCSTR)0x13c8;
    }
    else
    {
        pCVar5 = (LPCSTR)0x13c4;
    }
    uVar3 = *(undefined4 *)(iVar6 + 0xa);
    WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), pCVar5);
    uVar3 = *(undefined4 *)(iVar6 + 0xe);
    sys_1000_3f9c((uchar *)uVar3, (uchar *)((ulong)uVar3 >> 0x10), 0x14a2, (ushort)&USHORT_1050_1050, (ushort) * (undefined4 *)(iVar6 + 0x76), &stack0xfffe, uVar7, 0x1000, unaff_SS, in_AF);
    uVar3 = *(undefined4 *)(iVar6 + 0xa);
    WritePrivateProfileString16((LPCSTR)&PTR_LOOP_1050_1000, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPCSTR) * (undefined4 *)(iVar6 + 0xe));
    if(*(int *)(iVar6 + 0x7a) == 0x0)
    {
        pCVar5 = (LPCSTR)0x13c8;
    }
    else
    {
        pCVar5 = (LPCSTR)0x13c4;
    }
    uVar3 = *(undefined4 *)(iVar6 + 0xa);
    WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), pCVar5);
    uVar3 = *(undefined4 *)(iVar6 + 0xa);
    WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPCSTR) * (undefined4 *)(iVar6 + 0x1a));
    uVar3 = *(undefined4 *)(iVar6 + 0xa);
    WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPCSTR) * (undefined4 *)(iVar6 + 0x68));
    uVar3 = *(undefined4 *)(iVar6 + 0xa);
    WritePrivateProfileString16((LPCSTR)s_tile2_bmp_1050_1538, (LPCSTR)uVar3, (LPCSTR)((ulong)uVar3 >> 0x10), (LPCSTR) * (undefined4 *)(iVar6 + 0x6c));
    iStack12 = 0x1;
    do
    {
    switchD_1010:
        2ab5 ::caseD_13((ulong)param_1, iStack12);
        iStack12 = iStack12 + 0x1;
    } while(iStack12 < 0x8);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0xa), 0x1000);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0xe), 0x1000);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x12), 0x1000);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x16), 0x1000);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x1a), 0x1000);
    puVar1 = (undefined4 *)*(uint *)(iVar6 + 0x64);
    uVar2  = *(uint *)(iVar6 + 0x66);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar4 = (code **)*puVar1;
        (**ppcVar4)(0x1000, puVar1, uVar2, 0x1);
    }
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x68), 0x1000);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar6 + 0x6c), 0x1000);
    pass1_1010_1d80(param_1, unaff_SS);
    return;
}

void __stdcall16far pass1_1010_5d9c(ulong param_1, int param_2, uchar *param_3, int param_4, ushort param_5)

{
    ushort *puVar1;

    *(int *)((int)param_1 + 0x1e) = param_2;
    if(param_2 == 0x0)
    {
        puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2e, param_5, param_3, param_4);
        pass1_1018_209c((ulong)puVar1);
    }
    return;
}

void __stdcall16far get_private_profile_string_1010_6132(ulong param_1, int param_2, LPCSTR param_3)

{
    undefined4 uVar1;
    undefined4 uVar2;
    ushort     uVar3;
    int        iVar4;
    uint       in_DX;
    uint       uVar5;
    uint       uVar6;
    int        iVar7;
    undefined2 uVar8;
    ushort     unaff_SS;

    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar7 = (int)param_1;
    uVar1 = *(undefined4 *)(iVar7 + 0xe);
    uVar2 = *(undefined4 *)(iVar7 + 0xa);
    GetPrivateProfileString16(param_3, (LPCSTR)uVar2, (LPCSTR)((ulong)uVar2 >> 0x10), (LPSTR)((int)s_You_may_not_run_a_turn__The_game_1050_00df + 0x21), (UINT16)uVar1, (LPCSTR)((ulong)uVar1 >> 0x10));
    if(**(char **)(iVar7 + 0xe) != '\0')
    {
        uVar3 = pass1_1000_47a4(*(ulong *)(iVar7 + 0xe), 0x105014a6, unaff_SS);
        uVar5 = in_DX | uVar3;
        if(uVar5 != 0x0)
        {
            iVar4                  = pass1_1000_3e2c(CONCAT22(in_DX, uVar3));
            iVar7                  = param_2 * 0x8 + iVar7;
            *(int *)(iVar7 + 0x22) = iVar4;
            uVar3                  = pass1_1000_47a4(0x0, 0x105014a8, unaff_SS);
            uVar6                  = uVar5 | uVar3;
            if(uVar6 != 0x0)
            {
                iVar4                  = pass1_1000_3e2c(CONCAT22(uVar5, uVar3));
                *(int *)(iVar7 + 0x24) = iVar4;
                uVar3                  = pass1_1000_47a4(0x0, 0x105014aa, unaff_SS);
                uVar5                  = uVar6 | uVar3;
                if(uVar5 != 0x0)
                {
                    iVar4                  = pass1_1000_3e2c(CONCAT22(uVar6, uVar3));
                    *(int *)(iVar7 + 0x26) = iVar4;
                    uVar3                  = pass1_1000_47a4(0x0, 0x105014ac, unaff_SS);
                    if((uVar5 | uVar3) != 0x0)
                    {
                        iVar4                  = pass1_1000_3e2c(CONCAT22(uVar5, uVar3));
                        *(int *)(iVar7 + 0x28) = iVar4;
                    }
                }
            }
        }
    }
    return;
}

void __stdcall16far switchD_1010: 2ab5 ::caseD_13(ulong param_1, int param_2)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 unaff_SS;
    undefined1 in_AF;

    iVar2 = param_2 * 0x8 + (int)param_1;
    if((((*(int *)(iVar2 + 0x22) != 0x0) || (*(int *)(iVar2 + 0x24) != 0x0)) || (*(int *)(iVar2 + 0x26) != 0x0)) || (*(int *)(iVar2 + 0x28) != 0x0))
    {
        uVar1 = *(undefined4 *)((int)param_1 + 0xe);
        sys_1000_3f9c(
          (uchar *)uVar1, (uchar *)((ulong)uVar1 >> 0x10), (ushort)s__d__d__d__d_1050_14ae, (ushort)&USHORT_1050_1050, (ushort) * (undefined4 *)(param_2 * 0x8 + (int)param_1 + 0x22), &stack0xfffe, param_1._2_2_, 0x1000, unaff_SS, in_AF);
        uVar1 = *(undefined4 *)((int)param_1 + 0xa);
        WritePrivateProfileString16((LPCSTR)&PTR_LOOP_1050_1000, (LPCSTR)uVar1, (LPCSTR)((ulong)uVar1 >> 0x10), (LPCSTR) * (undefined4 *)((int)param_1 + 0xe));
    }
    return;
}
ushort *__stdcall16far pass1_1010_62ec(ushort *param_1, byte param_2)

{
    write_private_profile_str_1010_5b10(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ulong __stdcall16far pass1_1010_451a(ulong param_1, uchar *param_2, int param_3, ushort param_4)

{
    ushort     uVar1;
    undefined2 uVar2;
    ushort    *puVar3;
    ulong      uVar4;

    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_4, param_2, param_3);
    uVar1  = (ushort)((ulong)puVar3 >> 0x10);
    uVar4  = pass1_1010_ec40((ushort)puVar3, uVar1, *(ulong *)((int)param_1 + 0x6c), (ushort)puVar3, uVar1);
    uVar2  = (undefined2)(uVar4 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)uVar4 + 0x4), *(undefined2 *)((int)uVar4 + 0x2));
}

void __stdcall16far get_sys_metrics_1010_46f6(ulong param_1)

{
    undefined2 uVar1;
    INT16      IVar2;
    INT16      IVar3;
    uchar     *in_DX;
    int        iVar4;
    int        unaff_DI;
    undefined2 uVar5;
    ushort     unaff_SS;
    ushort    *puVar6;
    ulong      uVar7;
    ushort    *puVar8;
    ushort    *puVar9;
    int        local_6;
    int        local_4;

    puVar9 = (ushort *)CONCAT22(unaff_SS, &local_4);
    puVar8 = (ushort *)CONCAT22(unaff_SS, &local_6);
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, unaff_SS, in_DX, unaff_DI);
    pass1_1008_3e94((ushort *)((ulong)puVar6 & 0xffff0000 | (ulong)((int)puVar6 + 0xe)), puVar8, puVar9);
    uVar5                  = (undefined2)(param_1 >> 0x10);
    iVar4                  = (int)param_1;
    uVar7                  = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x66));
    uVar1                  = (undefined2)(uVar7 >> 0x10);
    *(int *)(iVar4 + 0x18) = local_4 + 0x8;
    *(int *)(iVar4 + 0x1a) = local_6 + 0x9;
    IVar2                  = GetSystemMetrics16(0x1008);
    *(int *)(iVar4 + 0x1c) = IVar2 * 0x2 + *(int *)((int)uVar7 + 0x4);
    IVar2                  = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    IVar3                  = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    *(int *)(iVar4 + 0x1e) = IVar3 + IVar2 + *(int *)((int)uVar7 + 0x8);
    return;
}

void __stdcall16far pass1_1010_4a8a(astruct_637 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    uchar      *puVar1;
    int         unaff_DI;
    astruct_43 *paVar2;
    ushort     *puVar3;

    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0x16                          = (astruct_76 *)0x0;
    param_1->field_0x1a                          = 0x0;
    param_1->field_0x1e                          = 0x0;
    param_1->field_0x20                          = 0x1;
    param_1->field_0x22                          = 0x0;
    param_1->field_0x24                          = 0x0;
    *(undefined4 *)&param_1->field_0x26          = 0x0;
    param_1->field_0x2a                          = 0x0;
    param_1->field_0x2c                          = 0x1;
    param_1->field_0x2e                          = 0x0;
    param_1->field_0x30                          = 0x0;
    param_1->field_0x32                          = 0x0;
    *(int *)CONCAT22(param_2, param_1)           = (int)s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
    param_1->field_0x2                           = 0x1010;
    paVar2                                       = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1b3, param_4);
    puVar1                                       = (uchar *)((ulong)paVar2 >> 0x10);
    *(int *)&param_1->field_0x16                 = (int)paVar2;
    *(uchar **)((int)&param_1->field_0x16 + 0x2) = puVar1;
    puVar3                                       = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_4, puVar1, unaff_DI);
    param_1->field_0x26                          = (int)puVar3;
    param_1->field_0x28                          = (int)((ulong)puVar3 >> 0x10);
    pass1_1008_4772(param_1->field_0x16);
    param_1->field_0xe  = 0x13c;
    param_1->field_0xa  = 0x0;
    param_1->field_0x10 = 0x0;
    param_1->field_0xc  = 0x0;
    return;
}

void __stdcall16far struct_1010_4d5c(ulong param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5, int param_6, uchar *param_7)

{
    undefined4   uVar1;
    ushort       uVar2;
    astruct_245 *iVar3;
    int          iVar4;
    undefined2   uVar5;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar3 = (astruct_245 *)param_1;
    if(*(long *)&iVar3->field_0x1a == 0x0)
    {
        uVar2 = iVar3->field_0x30 << 0x3;
        mem_op_1000_179c(uVar2, param_7, 0x1000);
        *(ushort *)&iVar3->field_0x1a = uVar2;
        iVar3->field_0x1c             = param_7;
    }
    uVar1                                 = *(undefined4 *)&iVar3->field_0x1a;
    iVar4                                 = param_6 * 0x8;
    *(ushort *)((int)uVar1 + iVar4)       = param_5;
    uVar1                                 = *(undefined4 *)&iVar3->field_0x1a;
    *(ushort *)((int)uVar1 + iVar4 + 0x2) = param_4;
    uVar1                                 = *(undefined4 *)&iVar3->field_0x1a;
    *(ushort *)((int)uVar1 + iVar4 + 0x4) = param_3;
    uVar1                                 = *(undefined4 *)&iVar3->field_0x1a;
    *(ushort *)((int)uVar1 + iVar4 + 0x6) = param_2;
    return;
}

void __stdcall16far pass1_1010_4f48(ulong param_1, ushort param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    undefined4  *puVar4;
    undefined4   uVar5;
    astruct_482 *iVar6;
    astruct_483 *iVar7;
    undefined2   uVar6;
    undefined2   uVar7;
    astruct_43  *paVar8;

    uVar6             = (undefined2)(param_1 >> 0x10);
    iVar6             = (astruct_482 *)param_1;
    puVar4            = iVar6->field_0x12;
    iVar6->field_0x30 = *(undefined2 *)((int)puVar4 + 0x8);
    if(iVar6->field_0x32 != 0x0)
    {
        uVar5            = *iVar6->field_0x12;
        uVar7            = (undefined2)((ulong)uVar5 >> 0x10);
        iVar7            = (astruct_483 *)uVar5;
        puVar4           = iVar7->field_0x4;
        iVar7->field_0x4 = (undefined4 *)iVar6->field_0x32;
        if(puVar4 != (undefined4 *)0x0)
        {
            ppcVar3 = (code **)*puVar4;
            (**ppcVar3)();
        }
        iVar6->field_0x32 = 0x0;
    }
    puVar1 = iVar6->field_0x16;
    uVar2  = iVar6->field_0x18;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    paVar8            = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1b3, param_2);
    iVar6->field_0x16 = (undefined4 *)paVar8;
    iVar6->field_0x18 = (uint)((ulong)paVar8 >> 0x10);
    fn_ptr_1000_17ce((astruct_18 *)iVar6->field_0x1a, 0x1000);
    iVar6->field_0x1a = 0x0;
    iVar6->field_0x2e = 0x0;
    return;
}


ushort *__stdcall16far pass1_1010_5004(ushort *param_1, byte param_2, ushort param_3)

{
    free_rsrc_1010_4b3e(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1010_503e(int param_1, ushort param_2, ushort param_3, uchar *param_4, ushort param_5)

{
    struct_op_1018_4cda(param_1, param_2, param_3);
    *(int *)CONCAT22(param_2, param_1) = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1;
    *(undefined2 *)(param_1 + 0x2)     = 0x1010;
    pass1_1018_4dce((ulong *)CONCAT22(param_2, param_1), 0x1b3, param_4, param_5);
    _PTR_LOOP_1050_4230 = CONCAT22(param_2, param_1);
    return;
}

void __stdcall16far pass1_1010_519a(ulong param_1, int *param_2, uchar *param_3, ushort param_4)

{
    undefined4   uVar1;
    ushort       uVar2;
    undefined   *puVar3;
    uchar       *puVar4;
    astruct_246 *iVar5;
    astruct_247 *iVar6;
    int          iVar7;
    undefined2   uVar8;
    undefined2   uVar9;
    int         *piStack44;
    undefined    local_18[0xc];
    int          iStack12;
    undefined4   uStack6;

    uStack6 = 0x0;
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_4, local_18), 0x1, 0x0, 0x400);
    uVar8             = (undefined2)(param_1 >> 0x10);
    iVar5             = (astruct_246 *)param_1;
    iVar5->field_0x10 = iStack12;
    fn_ptr_1000_17ce(*(astruct_18 **)&iVar5->field_0xc, 0x1000);
    uVar2 = iVar5->field_0x10 << 0x2;
    mem_op_1000_179c(uVar2, param_3, 0x1000);
    iVar5->field_0xc  = uVar2;
    iVar5->field_0xe  = param_3;
    iVar5->field_0x10 = 0x0;
    while(true)
    {
        puVar4 = param_3;
        puVar3 = local_18;
        pass1_1028_e4ec(CONCAT22(param_4, puVar3));
        uStack6 = CONCAT22(puVar4, puVar3);
        if((uchar *)((uint)puVar4 | (uint)puVar3) == (uchar *)0x0)
            break;
        param_3 = (uchar *)((uint)puVar4 | (uint)puVar3);
        if(*(long *)(puVar3 + 0x200) != 0x8000002)
        {
            param_3                          = *(uchar **)(puVar3 + 0x6);
            uVar1                            = *(undefined4 *)&iVar5->field_0xc;
            uVar9                            = (undefined2)((ulong)uVar1 >> 0x10);
            iVar7                            = (int)uVar1;
            iVar6                            = (astruct_247 *)(iVar5->field_0x10 * 0x4);
            piStack44                        = (int *)(param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x10));
            *(undefined2 *)(iVar6 + iVar7)   = *(undefined2 *)(puVar3 + 0x4);
            *(uchar **)(iVar6 + iVar7 + 0x2) = param_3;
            *piStack44                       = *piStack44 + 0x1;
        }
    }
    *param_2 = iVar5->field_0x10;
    return;
}

void __stdcall16far pass1_1010_35a4(ulong *param_1, ulong param_2, uchar *param_3)

{
    code     **ppcVar1;
    undefined4 uVar2;
    uint       uVar3;
    ulong     *puVar4;
    uint       uVar5;
    uchar     *extraout_DX;
    uchar     *puVar6;
    uchar     *extraout_DX_00;
    undefined2 uVar7;
    ushort     unaff_SS;
    ulong      uStack12;
    ulong     *puStack8;

    uVar7    = (undefined2)((ulong)param_1 >> 0x10);
    uVar2    = *(undefined4 *)((int)param_1 + 0x56);
    uVar2    = *(undefined4 *)((int)uVar2 + 0x8);
    puStack8 = *(ulong **)((int)uVar2 + *(int *)((int)param_1 + 0x5a) * 0x4);
    uStack12 = param_2;
    if(param_2 != 0x0)
    {
        uVar7 = 0x1000;
        mem_op_1000_179c(0x4a, param_3, 0x1000);
        uVar3 = (uint)param_2;
        uVar5 = (uint)param_3 | uVar3;
        if(uVar5 == 0x0)
        {
            uVar3 = 0x0;
            uVar5 = 0x0;
        }
        else
        {
            uVar7 = SUB42(&PTR_LOOP_1050_1040, 0x0);
            pass1_1040_c54a((ushort *)(param_2 & 0xffff | ZEXT24(param_3) << 0x10), 0x1, puStack8, (ushort)&PTR_LOOP_1050_1040, unaff_SS);
        }
        ppcVar1 = (code **)((int)*param_1 + 0x18);
        (**ppcVar1)(uVar7, param_1, 0x1, uVar3, uVar5);
        puVar6 = extraout_DX;
        for(; (uStack12 & 0xf) != 0x0; uStack12 = uStack12 >> 0x4)
        {
            uVar2    = *(undefined4 *)((int)puStack8 + 0x8);
            puStack8 = *(ulong **)((((byte)uStack12 & 0xf) - 0x1) * 0x4 + (int)uVar2);
            uVar7    = 0x1000;
            puVar4   = puStack8;
            mem_op_1000_179c(0x4a, puVar6, 0x1000);
            uVar3 = (uint)puVar4;
            uVar5 = (uint)puVar6 | uVar3;
            if(uVar5 == 0x0)
            {
                uVar3 = 0x0;
                uVar5 = 0x0;
            }
            else
            {
                uVar7 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                pass1_1040_c54a((ushort *)((ulong)puVar4 & 0xffff | ZEXT24(puVar6) << 0x10), 0x1, puStack8, (ushort)&PTR_LOOP_1050_1040, unaff_SS);
            }
            ppcVar1 = (code **)((int)*param_1 + 0x18);
            (**ppcVar1)(uVar7, param_1, 0x1, uVar3, uVar5);
            puVar6 = extraout_DX_00;
        }
    }
    return;
}

void __stdcall16far pass1_1010_3680(ushort param_1, ushort param_2, ushort param_3, ushort param_4, uint param_5, uchar *param_6, ushort param_7)

{
    mem_op_1000_179c(0x4a, param_6, 0x1000);
    if(((uint)param_6 | param_5) != 0x0)
    {
        pass1_1040_c54a((ushort *)CONCAT22(param_6, param_5), 0x1, (ulong *)CONCAT22(param_4, param_3), (ushort)&PTR_LOOP_1050_1040, param_7);
        return;
    }
    return;
}

ushort *__stdcall16far struct_1010_38f8(ulong param_1, int param_2, uint param_3, uchar *param_4)

{
    ushort       uVar1;
    astruct_240 *iVar2;
    undefined2   uVar2;
    ushort      *puVar3;

    if(param_2 != 0x0)
    {
        uVar1 = param_2 << 0x2;
        mem_op_1000_179c(uVar1, param_4, 0x1000);
        uVar2            = (undefined2)(param_1 >> 0x10);
        iVar2            = (astruct_240 *)param_1;
        iVar2->field_0x8 = uVar1;
        iVar2->field_0xa = param_4;
        return (ushort *)CONCAT22(param_4, iVar2->field_0x8);
    }
    mem_op_1000_179c(0x1a, param_4, 0x1000);
    if(((uint)param_4 | param_3) != 0x0)
    {
        puVar3 = pass1_1010_37d4((ushort *)CONCAT22(param_4, param_3));
        return puVar3;
    }
    return (ushort *)0x0;
}

void __stdcall16far pass1_1010_394a(ushort param_1, ushort param_2, int param_3, uint param_4, uchar *param_5)

{
    if(param_3 != 0x0)
    {
        mem_op_1000_179c(param_3 << 0x2, param_5, 0x1000);
        return;
    }
    mem_op_1000_179c(0x16, param_5, 0x1000);
    if(((uint)param_5 | param_4) != 0x0)
    {
        struct_1010_383a((ushort *)CONCAT22(param_5, param_4));
        return;
    }
    return;
}

ulong __stdcall16far pass1_1010_3d82(astruct_628 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    astruct_43 *paVar1;

    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)&param_1->field_0xa        = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x3e2c;
    param_1->field_0x2                        = 0x1010;
    paVar1                                    = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x99, param_4);
    param_1->field_0xa                        = (int)paVar1;
    param_1->field_0xc                        = (int)((ulong)paVar1 >> 0x10);
    return CONCAT22(param_2, param_1);
}

void __stdcall16far pass1_1010_3e3c(astruct_55 *param_1, ushort param_2, ushort param_3)

{
    astruct_633 *iVar1;
    undefined2   uVar1;
    astruct_43  *paVar2;

    get_sys_metrics_1018_4b1e(param_1, 0x6, param_2);
    uVar1                             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                             = (astruct_633 *)param_1;
    iVar1->field_0x20                 = 0x389a;
    iVar1->field_0x22                 = 0x1008;
    iVar1->field_0x20                 = 0x3aa8;
    iVar1->field_0x22                 = 0x1008;
    iVar1->field_0x24                 = 0x0;
    *(undefined4 *)&iVar1->field_0x66 = 0x0;
    iVar1->field_0x6a                 = 0x4;
    iVar1->field_0x6c                 = 0x0;
    iVar1->field_0x70                 = 0x0;
    iVar1->field_0x74                 = 0x0;
    pass1_1008_3e54((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x76), 0x0, 0x3, 0x5);
    iVar1->field_0x7c  = 0x0;
    param_1->field_0x0 = (ushort)&PTR_LOOP_1050_4a46;
    iVar1->field_0x2   = 0x1010;
    iVar1->field_0x20  = (int)&PTR_LOOP_1050_4a82;
    iVar1->field_0x22  = 0x1010;
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x26), (WNDCLASS16 *)0x0, 0x40);
    paVar2            = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1a1, param_3);
    iVar1->field_0x66 = (int)paVar2;
    iVar1->field_0x68 = (int)((ulong)paVar2 >> 0x10);
    pass1_1018_4b78((ulong *)param_1, param_3);
    return;
}

void __stdcall16far pass1_1010_41d6(ulong param_1, ulong param_2, uchar *param_3, ushort param_4, uchar param_5)

{
    uint        *puVar1;
    int         *piVar2;
    undefined4   uVar3;
    ushort       uVar4;
    uint         uVar5;
    int          iVar6;
    int          iVar7;
    uchar       *puVar8;
    uchar       *puVar9;
    astruct_243 *iVar9;
    astruct_244 *iVar10;
    int          unaff_DI;
    undefined2   uVar10;
    undefined2   uVar11;
    ushort      *puVar12;
    int          iStack50;
    int          local_30;
    astruct_18  *local_2e;
    int          iStack42;
    astruct_18  *paStack40;
    astruct_18  *paStack34;
    astruct_18  *paStack30;
    int          iStack26;
    uint         uStack24;
    int          iStack22;
    undefined4   uStack20;
    uint         uStack16;
    undefined4   uStack14;
    ulong        uStack10;
    ushort       uStack6;
    ushort       uStack4;

    uVar10            = (undefined2)(param_1 >> 0x10);
    iVar9             = (astruct_243 *)param_1;
    iVar9->field_0x6c = param_2;
    puVar12           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_4, param_3, unaff_DI);
    uStack4           = (ushort)((ulong)puVar12 >> 0x10);
    uStack6           = (ushort)puVar12;
    uStack10          = pass1_1010_ec40(uStack6, uStack4, iVar9->field_0x6c, uStack6, uStack4);
    puVar8            = (uchar *)(uStack10 >> 0x10);
    iVar9->field_0x74 = *(int *)((int)uStack10 + 0x22);
    if(*(long *)&iVar9->field_0x70 != 0x0)
    {
        paStack34 = *(astruct_18 **)&iVar9->field_0x70;
        paStack30 = paStack34;
        fn_ptr_1000_17ce(paStack34, 0x1000);
        *(undefined4 *)&iVar9->field_0x70 = 0x0;
    }
    uVar4 = iVar9->field_0x74 << 0x7;
    mem_op_1000_179c(uVar4, puVar8, 0x1000);
    *(ushort *)&iVar9->field_0x70 = uVar4;
    iVar9->field_0x72             = puVar8;
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), iVar9->field_0x6c);
    uStack14 = CONCAT22(puVar8, uVar4);
    uStack16 = (uint)(**(int **)(uVar4 + 0x10) == 0x9);
    iStack22 = *(int *)((int)uStack10 + 0x22);
    uVar4    = iStack22 * 0x6;
    mem_op_1000_179c(uVar4, puVar8, 0x1000);
    paStack30 = (astruct_18 *)CONCAT22(puVar8, uVar4);
    puVar9    = (uchar *)((uint)puVar8 | uVar4);
    if(puVar9 == (uchar *)0x0)
    {
        uStack20 = (astruct_18 *)0x0;
    }
    else
    {
        pass1_1000_5586((uchar *)0x3e38, 0x1008, iStack22, 0x6, uVar4, (ushort)puVar8);
        uStack20 = paStack30;
    }
    uStack24 = 0x0;
    while(true)
    {
        uVar11 = (undefined2)(uStack10 >> 0x10);
        puVar1 = (uint *)((int)uStack10 + 0x22);
        if(*puVar1 < uStack24 || *puVar1 == uStack24)
            break;
        uVar3 = *(undefined4 *)((int)uStack10 + 0x24);
        uVar5 = uStack24;
        pass1_1028_e0a0(_PTR_LOOP_1050_65e2, (ulong) * (uint *)((int)uVar3 + uStack24 * 0x2) << 0x10, puVar9, param_4, param_5);
        paStack34 = (astruct_18 *)CONCAT22(puVar9, uVar5);
        pass1_1008_3f62((ushort *)((ulong)uStack20 & 0xffff0000 | (ulong)(uStack24 * 0x6 + (int)uStack20)), (ushort *)CONCAT22(puVar9, uVar5 + 0x8));
        paStack40 = paStack34;
        paStack30 = paStack34;
        if(paStack34 != (astruct_18 *)0x0)
        {
            fn_ptr_1030_84d0((ulong)paStack34);
            fn_ptr_1000_17ce(paStack34, 0x1000);
        }
        uStack24 = uStack24 + 0x1;
        puVar9   = uStack20._2_2_;
    }
    for(iStack26 = 0x0; piVar2 = &iVar9->field_0x74, *piVar2 != iStack26 && iStack26 <= *piVar2; iStack26 = iStack26 + 0x1)
    {
        pass1_1008_3e94((ushort *)((ulong)uStack20 & 0xffff0000 | (ulong)(uint)(iStack26 * 0x6 + (int)uStack20)), (ushort *)CONCAT22(param_4, &local_2e), (ushort *)CONCAT22(param_4, &local_30));
        iStack42 = pass1_1000_49b2((uint)local_2e);
        iStack42 = iStack42 / 0x5;
        if(0xc < iStack42)
        {
            iStack42 = 0xc;
            iVar6    = pass1_1000_49b2((uint)local_2e);
            local_2e = (astruct_18 *)((ulong)local_2e & 0xffff0000 | (ulong)(uint)(((int)(uint)local_2e / iVar6) * 0x3c));
        }
        iVar7     = pass1_1000_49b2((uint)local_2e);
        iVar6     = (int)((long)iVar7 % 0x5);
        paStack34 = (astruct_18 *)((ulong)paStack34 & 0xffff0000 | (long)iVar7 % 0x5 & 0xffffU);
        if((int)(uint)local_2e < 0x0)
        {
            if(0x2 < iVar6)
            {
                iVar6 = iVar6 + -0x5;
            }
            local_2e = (astruct_18 *)((ulong)local_2e & 0xffff0000 | (ulong)((uint)local_2e + iVar6));
        }
        else
        {
            if(iVar6 < 0x3)
            {
                local_2e = (astruct_18 *)((ulong)local_2e & 0xffff0000 | (ulong)((uint)local_2e - iVar6));
            }
            else
            {
                local_2e = (astruct_18 *)((ulong)local_2e & 0xffff0000 | (ulong)((uint)local_2e + (0x5 - iVar6)));
            }
        }
        iStack50 = local_30 / 0x16;
        for(iVar6 = 0x0; iVar6 < 0x10; iVar6 = iVar6 + 0x1)
        {
            if(0xf < iStack50)
            {
                iStack50 = 0x0;
            }
            if(((int)(uint)(uStack16 != 0x0) < iStack50) && (iStack50 < 0x8))
            {
                iVar7                                = *(int *)((iStack42 * 0x10 + iStack50) * 0x2 + 0x11e0);
                iVar10                               = (astruct_244 *)((iStack26 * 0x10 + iVar6) * 0x8);
                uVar3                                = *(undefined4 *)&iVar9->field_0x70;
                *(int *)(iVar10 + (int)uVar3)        = iVar7 + 0x49;
                uVar3                                = *(undefined4 *)&iVar9->field_0x70;
                *(uint *)(iVar10 + (int)uVar3 + 0x2) = (uint)local_2e + 0x49;
                uVar3                                = *(undefined4 *)&iVar9->field_0x70;
                *(int *)(iVar10 + (int)uVar3 + 0x4)  = iVar7 + 0x4e;
                uVar3                                = *(undefined4 *)&iVar9->field_0x70;
                *(uint *)(iVar10 + (int)uVar3 + 0x6) = (uint)local_2e + 0x4e;
            }
            else
            {
                iVar7                                     = (iStack26 * 0x10 + iVar6) * 0x8;
                uVar3                                     = *(undefined4 *)&iVar9->field_0x70;
                *(undefined2 *)(iVar7 + (int)uVar3)       = 0x0;
                uVar3                                     = *(undefined4 *)&iVar9->field_0x70;
                *(undefined2 *)((int)uVar3 + iVar7 + 0x2) = 0x0;
                uVar3                                     = *(undefined4 *)&iVar9->field_0x70;
                *(undefined2 *)((int)uVar3 + iVar7 + 0x4) = 0x1;
                uVar3                                     = *(undefined4 *)&iVar9->field_0x70;
                *(undefined2 *)((int)uVar3 + iVar7 + 0x6) = 0x1;
            }
            iStack50 = iStack50 + 0x1;
        }
    }
    paStack40 = uStack20;
    local_2e  = uStack20;
    fn_ptr_1000_17ce(uStack20, 0x1000);
    draw_1010_47ae(param_1, 0x1000, param_4);
    return;
}
