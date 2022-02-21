
// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address


void __stdcall16far pass1_1008_ee72(ushort param_1, ushort param_2, int param_3)

{
    code **ppcVar1;
    ulong  uVar2;

    if(*(long *)(param_1 + 0x56) == 0x0)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)CONCAT22(param_2, param_1) + 0x10);
        (**ppcVar1)();
    }
    uVar2 = pass1_1010_2e02(CONCAT22(param_2, param_1), param_3);
    pass1_1010_2e5c(param_1, param_2, uVar2);
    return;
}


ushort __stdcall16far pass1_1008_eea6(void)

{
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool __stdcall16far pass1_1008_eeac(ushort param_1, ushort param_2, ulong param_3, uchar *param_4, int param_5, ushort param_6)

{
    ushort  uVar1;
    char    cVar2;
    uint    uVar3;
    ushort  uVar4;
    ushort  uVar5;
    ushort *puVar6;
    uint    uVar7;

    uVar7  = *(uint *)((int)param_3 + 0x12);
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, param_4, param_5);
    uVar4  = (ushort)((ulong)puVar6 >> 0x10);
    uVar1  = (ushort)puVar6;
    uVar5  = uVar4;
    if(uVar7 == 0x7d)
    {
        pass1_1010_a5ca(uVar1, uVar4, 0x7c, 0x7d, uVar4);
        if(uVar7 != 0x0)
        {
            return false;
        }
        pass1_1010_a5ca(uVar1, uVar4, 0x7d, 0x0, uVar5);
        if(uVar7 != 0x0)
        {
            return false;
        }
        uVar3 = uVar7;
        uVar7 = 0x78;
    }
    else
    {
        uVar3 = uVar7;
        if(uVar7 < 0x7e)
        {
            cVar2 = (char)uVar7;
            uVar3 = uVar7 & 0xff00;
            if((byte)(cVar2 + 0x8dU) == 0x0)
            {
                uVar7 = 0x9;
                uVar3 = uVar3 | (byte)(cVar2 + 0x8dU);
            }
            else
            {
                if((byte)(cVar2 + 0x89U) == 0x0)
                {
                    uVar7 = 0x2e;
                    uVar3 = uVar3 | (byte)(cVar2 + 0x89U);
                }
                else
                {
                    uVar3 = uVar3 | (byte)(cVar2 + 0x87U);
                    if((byte)(cVar2 + 0x87U) == 0x0)
                    {
                        uVar7 = 0x5b;
                    }
                }
            }
        }
    }
    pass1_1010_a5ca(uVar1, uVar4, uVar7, uVar3, uVar5);
    return uVar3 == 0x0;
}


ushort __stdcall16far pass1_1008_ef38(ulong param_1)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0x16);
    return *(ushort *)((int)uVar1 + 0x2);
}


ushort __stdcall16far pass1_1008_ef4a(void)

{
    return 0x41;
}


ushort *__stdcall16far pass1_1008_ef50(ushort *param_1, byte param_2)

{
    pass1_1008_ec94(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1008_ef76(astruct_18 *param_1, byte param_2)

{
    ushort unaff_SS;

    pass1_1008_ed00(&param_1->field_0x0, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_0000(astruct_645 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int         unaff_DI;
    astruct_79 *paVar1;
    ushort     *puVar2;
    undefined2 *puVar3;
    ushort      uVar4;
    undefined2 *puVar5;
    ushort      uVar6;

    // Segment:    3
    // Offset:     00015420
    // Length:     ee9f
    // Min Alloc:  ee9f
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    paVar1                                    = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                        = 0x0;
    param_1->field_0xc                        = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x2c8;
    param_1->field_0x2                        = 0x1010;
    puVar5                                    = &param_1->field_0xa;
    puVar3                                    = &param_1->field_0xc;
    uVar4                                     = param_2;
    uVar6                                     = param_2;
    puVar2                                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, (uchar *)((ulong)paVar1 >> 0x10), unaff_DI);
    pass1_1008_3e94((ushort *)((ulong)puVar2 & 0xffff0000 | (ulong)((int)puVar2 + 0xe)), (ushort *)CONCAT22(uVar4, puVar3), (ushort *)CONCAT22(uVar6, puVar5));
    return CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1010_0052(ushort *param_1, ushort param_2)

{
    *param_1                            = 0x2c8;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
    pass1_1010_1d80(param_1, param_2);
    return;
}


void __stdcall16far set_window_placement_1010_0070(ulong param_1, int param_2, ushort param_3, HWND16 param_4, ushort param_5)

{
    code      **ppcVar1;
    undefined2  uVar2;
    undefined4 *puVar3;
    long        lVar4;
    undefined   local_18[0x6];
    INT16       IStack18;
    int         iStack16;
    INT16       IStack14;
    INT16       IStack12;
    INT16       IStack10;
    INT16       IStack8;
    undefined2  uStack6;
    undefined2  uStack4;

    local_18._0_2_ = 0x16;
    local_18._2_4_ = 0x0;
    IStack18       = 0x0;
    iStack16       = 0x0;
    IStack14       = 0x0;
    IStack12       = 0x0;
    IStack10       = 0x0;
    IStack8        = 0x0;
    uStack6        = 0x0;
    uStack4        = 0x0;
    GetWindowPlacement16(param_4, (WINDOWPLACEMENT16 *)local_18);
    if((iStack16 == -0x1) || (param_2 != 0x0))
    {
        local_18._2_4_ = 0x50001;
        lVar4          = GetWindowLong16((HWND16)s_tile2_bmp_1050_1538, 0x0);
        uVar2          = (undefined2)((ulong)lVar4 >> 0x10);
        puVar3         = (undefined4 *)*(undefined4 *)((int)lVar4 + 0xe0);
        ppcVar1        = (code **)((int)*puVar3 + 0x38);
        (**ppcVar1)((int)s_tile2_bmp_1050_1538, (int)puVar3, *(undefined2 *)((int)lVar4 + 0xe2), param_3);
        pass1_1010_01f8(param_1, CONCAT22(param_5, local_18), (int)puVar3);
        SetWindowPlacement16((HWND16)s_tile2_bmp_1050_1538, (WINDOWPLACEMENT16 *)local_18);
    }
    return;
}


void __stdcall16far set_win_placement_1010_010e(ushort param_1, ushort param_2, ushort param_3, HWND16 param_4)

{
    code            **ppcVar1;
    int               iVar2;
    int              *piVar3;
    undefined2        uVar4;
    undefined4       *puVar5;
    uint              extraout_DX;
    long              lVar6;
    WINDOWPLACEMENT16 local_18;
    int               iStack6;
    int               iStack4;

    local_18.length               = 0x16;
    local_18.flags                = 0x0;
    local_18.show_cmd             = 0x0;
    local_18.pt_min_position.x    = 0x0;
    local_18.pt_min_position.y    = 0x0;
    local_18.pt_max_position.x    = 0x0;
    local_18.pt_max_position.y    = 0x0;
    local_18.rc_normal_position.x = 0x0;
    local_18.rc_normal_position.y = 0x0;
    iStack6                       = 0x0;
    iStack4                       = 0x0;
    GetWindowPlacement16(param_4, &local_18);
    if(local_18.rc_normal_position.x == -0x1)
    {
        lVar6   = GetWindowLong16((HWND16)s_tile2_bmp_1050_1538, 0x0);
        uVar4   = (undefined2)((ulong)lVar6 >> 0x10);
        puVar5  = (undefined4 *)*(undefined4 *)((int)lVar6 + 0xe0);
        ppcVar1 = (code **)((int)*puVar5 + 0x1c);
        (**ppcVar1)((int)s_tile2_bmp_1050_1538, (int)puVar5, *(undefined2 *)((int)lVar6 + 0xe2), param_3);
        iVar2                         = (int)puVar5;
        piVar3                        = (int *)((ulong)puVar5 & 0xffff | (ulong)extraout_DX << 0x10);
        local_18.show_cmd             = 0x9;
        local_18.rc_normal_position.x = *piVar3;
        local_18.rc_normal_position.y = *(INT16 *)(iVar2 + 0x2);
        iStack6                       = *(int *)(iVar2 + 0x4) + *piVar3;
        iStack4                       = *(int *)(iVar2 + 0x2) + *(int *)(iVar2 + 0x6);
        SetWindowPlacement16((HWND16)s_tile2_bmp_1050_1538, &local_18);
    }
    return;
}


void __stdcall16far enum_child_windows_1010_01be(LPVOID param_1)

{
    LPVOID pvVar1;

    if(PTR_LOOP_1050_0010 == (undefined *)0x0)
    {
        pvVar1 = MakeProcInstance16(param_1, (HANDLE16)PTR_LOOP_1050_038c);
        EnumChildWindows1((HWND16)s_tile2_bmp_1050_1538, (LPVOID)0x0, ZEXT24(pvVar1) << 0x10);
        FreeProcInstance16((LPVOID)s_tile2_bmp_1050_1538);
    }
    return;
}


void __stdcall16far pass1_1010_01f8(ulong param_1, ulong param_2, int param_3)

{
    int        iVar1;
    int        iVar2;
    undefined2 uVar3;
    undefined2 uVar4;

    iVar2                        = *(int *)(param_3 * 0x4 + 0xe02) * 0x4;
    iVar1                        = *(int *)(iVar2 + 0xdfc);
    uVar3                        = (undefined2)(param_1 >> 0x10);
    uVar4                        = (undefined2)(param_2 >> 0x10);
    *(int *)((int)param_2 + 0x6) = *(int *)(param_3 * 0x4 + 0xe04) * 0x28 + *(int *)(iVar2 + 0xdfa) + *(int *)((int)param_1 + 0xa);
    *(int *)((int)param_2 + 0x8) = *(int *)((int)param_1 + 0xc) + iVar1;
    return;
}


BOOL16 __stdcall16far win_ui_op_1010_0240(ushort param_1, ushort param_2, ushort param_3, HWND16 param_4, ushort param_5)

{
    code      **ppcVar1;
    BOOL16      BVar2;
    WORD        WVar3;
    uchar      *in_DX;
    int         unaff_DI;
    undefined4 *puVar4;
    ushort      uVar5;
    ushort      uVar6;
    undefined2  uVar7;

    uVar7 = SUB42(&USHORT_1050_1050, 0x0);
    uVar6 = param_3;
    BVar2 = IsWindow16(param_4);
    if(BVar2 != 0x0)
    {
        WVar3 = GetWindowWord16((HWND16)s_tile2_bmp_1050_1538, -0x6);
        if(WVar3 == *(WORD *)&PTR_LOOP_1050_038c)
        {
            uVar5 = param_3;
            BVar2 = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
            if(BVar2 != 0x0)
            {
                puVar4  = (undefined4 *)mixed_1010_20ba(*(ulong *)&PTR_LOOP_1050_0ed0, 0x45, param_5, in_DX, unaff_DI);
                ppcVar1 = (code **)((int)*puVar4 + 0x10);
                (**ppcVar1)((int)s_tile2_bmp_1050_1538, puVar4, 0x1, param_3, uVar5, uVar6, uVar7);
            }
        }
    }
    return 0x1;
}


ushort *__stdcall16far pass1_1010_02a2(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_0052(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}


void __stdcall16far struct_1010_02e0(astruct_79 *param_1, astruct_79 *param_2, ushort param_3)

{
    uint        uVar1;
    uchar      *puVar2;
    ushort      extraout_DX;
    astruct_79 *paVar3;

    paVar3                                              = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    puVar2                                              = (uchar *)((ulong)paVar3 >> 0x10);
    uVar1                                               = 0x0;
    *(undefined4 *)(param_1 + 0x1)                      = 0x0;
    *(undefined2 *)&param_1[0x1].field_0x4              = 0x0;
    *(undefined2 *)((int)&param_1[0x1].field_0x4 + 0x2) = 0x0;
    *(undefined2 *)&param_1[0x2].field_0x4              = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1)           = 0xe98;
    param_1->field_0x2                                  = 0x1010;
    mem_op_1000_179c(0xc, puVar2, 0x1000);
    if(((uint)puVar2 | uVar1) == 0x0)
    {
        *(undefined4 *)(param_1 + 0x1) = 0x0;
    }
    else
    {
        set_struct_1008_574a((astruct_21 *)CONCAT22(puVar2, uVar1));
        ((astruct_79 *)(param_1 + 0x1))->field_0x0 = uVar1;
        param_1[0x1].field_0x2                     = extraout_DX;
    }
    return;
}


void __stdcall16far pass1_1010_0350(ushort *param_1, ushort param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_474 *iVar4;
    undefined2   uVar4;

    uVar4            = (undefined2)((ulong)param_1 >> 0x10);
    iVar4            = (astruct_474 *)param_1;
    *param_1         = 0xe98;
    iVar4->field_0x2 = 0x1010;
    puVar1           = iVar4->field_0xa;
    uVar2            = iVar4->field_0xc;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}
