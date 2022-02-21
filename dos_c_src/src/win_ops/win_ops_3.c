
void __stdcall16far window_op_1018_e6c6(astruct *param_1)

{
    astruct_660 *in_AX;
    uchar       *in_DX;
    uint         uVar1;
    int          iVar2;
    int          unaff_DI;
    undefined2   uVar3;
    ushort       unaff_SS;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar3 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (int)param_1;
    get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2), *(ushort *)(iVar2 + 0x8), 0x1008);
    mem_op_1000_179c(0x18, in_DX, 0x1000);
    uVar1 = (uint)in_DX | (uint)in_AX;
    if(uVar1 != 0x0)
    {
        pass1_1018_e834(in_AX, (ushort)in_DX, *(ushort *)(iVar2 + 0x8), unaff_DI, unaff_SS);
        *(astruct_660 **)(iVar2 + 0xee) = in_AX;
        *(uint *)(iVar2 + 0xf0)         = uVar1;
        return;
    }
    *(undefined4 *)(iVar2 + 0xee) = 0x0;
    return;
}

void __stdcall16far pass1_1018_e72a(ULONG param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    undefined2  uVar4;

    uVar4  = (undefined2)(param_1 >> 0x10);
    puVar1 = (undefined4 *)*(uint *)((int)param_1 + 0xee);
    uVar2  = *(uint *)((int)param_1 + 0xf0);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

void __stdcall16far post_win_msg_1018_ea0a(ushort param_1, ushort param_2, int param_3, HWND16 param_4)

{
    if(param_3 == 0xed)
    {
        PostMessage16(param_4, 0x0, 0x0, 0x11100c6);
    }
    return;
}

void __stdcall16far pass1_1018_ea66(ulong param_1, ushort param_2)

{
    code     **ppcVar1;
    undefined *puVar2;
    int        iVar3;
    undefined2 uVar4;
    ushort    *puVar5;
    undefined  local_6[0x4];

    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    if(*(long *)(iVar3 + 0xee) != 0x0)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0xee) + 0x8);
        (**ppcVar1)();
    }
    if(*(int *)(iVar3 + 0xea) == 0x0)
    {
        *(undefined2 *)(iVar3 + 0xea) = 0x1;
        puVar5                        = pass1_1008_941a((ushort *)CONCAT22(param_2, local_6), 0x1, 0x95);
        puVar2                        = local_6;
        win_1008_5c9e(_PTR_LOOP_1050_02a0, (ulong *)CONCAT22(param_2, puVar2), puVar2, (int)((ulong)puVar5 >> 0x10), param_2);
        *(int *)(iVar3 + 0xec) = (int)puVar2;
        unk_win_op_1010_7300(*(ulong *)(iVar3 + 0xf6), 0x0, 0x8, 0x0);
    }
    return;
}

void __stdcall16far window_op_1018_eada(astruct *param_1)

{
    astruct_661 *in_AX;
    uchar       *in_DX;
    uint         uVar1;
    int          iVar2;
    undefined2   uVar3;
    ushort       unaff_SS;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar3 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (int)param_1;
    get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2), *(ushort *)(iVar2 + 0x8), 0x1008);
    mem_op_1000_179c(0x28, in_DX, 0x1000);
    uVar1 = (uint)in_DX | (uint)in_AX;
    if(uVar1 != 0x0)
    {
        pass1_1018_ec74(in_AX, (int)in_DX, *(ushort *)(iVar2 + 0x8), unaff_SS);
        *(astruct_661 **)(iVar2 + 0xee) = in_AX;
        *(uint *)(iVar2 + 0xf0)         = uVar1;
        return;
    }
    *(undefined4 *)(iVar2 + 0xee) = 0x0;
    return;
}

void __stdcall16far pass1_1018_eb3e(ULONG param_1, ushort param_2)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;
    int         iVar6;
    undefined2  uVar7;

    uVar7  = (undefined2)(param_1 >> 0x10);
    iVar6  = (int)param_1;
    puVar1 = (undefined4 *)*(uint *)(iVar6 + 0xee);
    uVar2  = *(uint *)(iVar6 + 0xf0);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    if(*(long *)(iVar6 + 0xf6) != 0x0)
    {
        if(param_1 == 0x0)
        {
            iVar4 = 0x0;
            uVar5 = 0x0;
        }
        else
        {
            iVar4 = iVar6 + 0xe2;
            uVar5 = uVar7;
        }
        pass1_1010_1ea6(*(ulong *)(iVar6 + 0xf6), CONCAT22(uVar5, iVar4), param_2);
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

void __stdcall16far pass1_1020_02ae(ULONG param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    pass1_1010_3cd0(*(long *)(iVar4 + 0xe2));
    destroy_win_1008_628e(param_1, 0x1008);
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xe6);
    uVar2  = *(uint *)(iVar4 + 0xe8);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)(0x1008, puVar1, uVar2, 0x1);
    }
    *(undefined4 *)(iVar4 + 0xe6) = 0x0;
    pass1_1010_1dda(*(ulong *)(iVar4 + 0xe2));
    *(undefined4 *)(iVar4 + 0xe2) = 0x0;
    return;
}

void __stdcall16far win_1020_0316(astruct *param_1, uchar *param_2, ushort param_3)

{
    undefined4   uVar1;
    uint         uVar2;
    uchar       *puVar3;
    uchar       *puVar4;
    astruct_273 *iVar1;
    int          unaff_DI;
    undefined2   uVar5;
    ushort      *puVar6;

    create_window_ex_1008_9760(param_1, 0x1008);
    puVar6                             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x1, param_3, param_2, unaff_DI);
    puVar3                             = (uchar *)((ulong)puVar6 >> 0x10);
    uVar5                              = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                              = (astruct_273 *)param_1;
    iVar1->field_0xe2                  = (int)puVar6;
    iVar1->field_0xe4                  = (int)puVar3;
    uVar1                              = *(undefined4 *)&iVar1->field_0xe2;
    *(undefined2 *)((int)uVar1 + 0x16) = iVar1->field_0xea;
    uVar2                              = iVar1->field_0xee;
    uVar1                              = *(undefined4 *)&iVar1->field_0xe2;
    *(uint *)((int)uVar1 + 0x12)       = uVar2;
    struct_1010_3c52(*(ulong *)&iVar1->field_0xe2, iVar1->field_0xec, param_3);
    mem_op_1000_179c(0x12, puVar3, 0x1000);
    puVar4 = (uchar *)((uint)puVar3 | uVar2);
    if(puVar4 != (uchar *)0x0)
    {
        pass1_1020_04f6((ushort *)CONCAT22(puVar3, uVar2), iVar1->field_0x8, puVar4, unaff_DI, param_3);
        iVar1->field_0xe6 = uVar2;
        iVar1->field_0xe8 = (int)puVar4;
        return;
    }
    *(undefined4 *)&iVar1->field_0xe6 = 0x0;
    return;
}

void __stdcall16far post_msg_1020_03b2(ulong param_1, HWND16 param_2)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0xe2);
    PostMessage16(param_2, 0x0, 0x0, CONCAT22(0x111, *(undefined2 *)((int)uVar1 + 0x16)));
    return;
}


void __stdcall16far post_msg_1020_03d6(ulong param_1, HWND16 param_2)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0xe2);
    PostMessage16(param_2, 0x0, 0x0, CONCAT22(0x111, *(undefined2 *)((int)uVar1 + 0x16)));
    return;
}


void __stdcall16far post_msg_1020_03fa(ulong param_1, HWND16 param_2)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0xe2);
    PostMessage16(param_2, 0x0, 0x0, CONCAT22(0x111, *(undefined2 *)((int)uVar1 + 0x16)));
    return;
}


void __stdcall16far post_win_msg_1020_061c(ulong param_1, int param_2, HWND16 param_3)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    if(param_2 == 0x1)
    {
        *(undefined4 *)((int)param_1 + 0x6) = 0x0;
        return;
    }
    if(param_2 != 0x2)
    {
        return;
    }
    uVar1 = *(undefined4 *)((int)param_1 + 0x6);
    PostMessage16(param_3, 0x0, 0x0, CONCAT22(0x111, *(undefined2 *)((int)uVar1 + 0x16)));
    return;
}

void __stdcall16far pass1_1020_08b6(WNDCLASS16 *param_1, astruct_20 *param_2, UINT16 param_3, ULONG param_4)

{
    astruct_20 *iVar1;
    undefined2  uVar1;
    astruct_20 *paVar2;

    paVar2                                            = unk_draw_op_1008_61b2(param_2, 0x1, param_3, param_4, (UINT16)param_1);
    uVar1                                             = (undefined2)((ulong)param_2 >> 0x10);
    iVar1                                             = (astruct_20 *)param_2;
    *(undefined2 *)&iVar1[0x1].field_0x4              = 0x0;
    *(undefined4 *)((int)&iVar1[0x1].field_0x4 + 0x2) = 0x0;
    param_2->field_0x0                                = 0xb0e;
    iVar1->field_0x2                                  = 0x1020;
    win_1008_5c5c(param_1, 0x0, (ushort)((ulong)paVar2 >> 0x10), _PTR_LOOP_1050_02a0, 0x1d4);
    return;
}


void __stdcall16far win_1018_df40(astruct *param_1, uint param_2, uchar *param_3, ushort param_4)

{
    astruct_267 *iVar1;
    undefined2   uVar1;
    ushort      *puVar2;

    create_window_ex_1008_9760(param_1, 0x1008);
    mem_op_1000_179c(0xa, param_3, 0x1000);
    iVar1 = (astruct_267 *)param_1;
    uVar1 = (undefined2)((ulong)param_1 >> 0x10);
    if((uchar *)((uint)param_3 | param_2) != (uchar *)0x0)
    {
        puVar2            = struct_1018_e100((ushort *)CONCAT22(param_3, param_2), iVar1->field_0x8, (uchar *)((uint)param_3 | param_2), param_4);
        iVar1->field_0xe2 = (int)puVar2;
        iVar1->field_0xe4 = (int)((ulong)puVar2 >> 0x10);
        return;
    }
    *(undefined4 *)&iVar1->field_0xe2 = 0x0;
    return;
}


void __stdcall16far pass1_1018_df92(ULONG param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    destroy_win_1008_628e(param_1, 0x1008);
    uVar5  = (undefined2)(param_1 >> 0x10);
    iVar4  = (int)param_1;
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0xe2);
    uVar2  = *(uint *)(iVar4 + 0xe4);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)(0x1008, puVar1, uVar2, 0x1);
    }
    *(undefined4 *)(iVar4 + 0xe2) = 0x0;
    return;
}

void __stdcall16far pass1_1018_e2cc(ulong param_1, UINT16 param_2)

{
    int         *piVar1;
    code       **ppcVar2;
    uint         uVar3;
    ulong        uVar4;
    uchar       *puVar5;
    uchar       *puVar6;
    astruct_269 *iVar7;
    undefined2   uVar7;
    ushort      *puVar8;
    undefined4  *puStack10;
    undefined    local_6[0x4];

    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar7 = (astruct_269 *)param_1;
    if(iVar7->field_0xee != (undefined4 *)0x0)
    {
        ppcVar2 = (code **)((int)*iVar7->field_0xee + 0x8);
        (**ppcVar2)();
    }
    if(iVar7->field_0xea == 0x0)
    {
        iVar7->field_0xea = 0x1;
        puVar8            = pass1_1008_941a((ushort *)CONCAT22(param_2, local_6), 0x1, 0x7a);
        puVar5            = (uchar *)((ulong)puVar8 >> 0x10);
        uVar4             = ZEXT24(local_6);
        win_1008_5c9e(_PTR_LOOP_1050_02a0, (ulong *)CONCAT22(param_2, local_6), local_6, puVar5, param_2);
        iVar7->field_0xec = (int)uVar4;
        mem_op_1000_179c(0x112, puVar5, 0x1000);
        puVar6 = (uchar *)((uint)puVar5 | (uint)uVar4);
        if(puVar6 == (uchar *)0x0)
        {
            uVar3     = 0x0;
            puStack10 = (undefined4 *)0x0;
        }
        else
        {
            piVar1  = &iVar7->field_0xcc;
            *piVar1 = *piVar1 + 0x1;
            struct_1020_3644((ushort *)(uVar4 & 0xffff | ZEXT24(puVar5) << 0x10), iVar7->field_0xcc, param_1, param_2);
            uVar3     = (uint)uVar4;
            puStack10 = (undefined4 *)(uVar4 & 0xffff | ZEXT24(puVar6) << 0x10);
        }
        pass1_1008_6978(param_1, 0x0, (ulong)puStack10 & 0xffff0000 | (ulong)uVar3, uVar3, puVar6);
        ppcVar2 = (code **)((int)*puStack10 + 0xc);
        (**ppcVar2)();
    }
    return;
}

void __stdcall16far window_op_1018_e384(astruct *param_1)

{
    astruct_659 *in_AX;
    uchar       *in_DX;
    uint         uVar1;
    int          iVar2;
    int          unaff_DI;
    undefined2   uVar3;
    ushort       unaff_SS;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar3 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (int)param_1;
    get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2), *(ushort *)(iVar2 + 0x8), 0x1008);
    mem_op_1000_179c(0x18, in_DX, 0x1000);
    uVar1 = (uint)in_DX | (uint)in_AX;
    if(uVar1 != 0x0)
    {
        pass1_1018_e4f2(in_AX, (ushort)in_DX, *(ushort *)(iVar2 + 0x8), unaff_DI, unaff_SS);
        *(astruct_659 **)(iVar2 + 0xee) = in_AX;
        *(uint *)(iVar2 + 0xf0)         = uVar1;
        return;
    }
    *(undefined4 *)(iVar2 + 0xee) = 0x0;
    return;
}


void __stdcall16far pass1_1018_e3e8(ULONG param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    undefined2  uVar4;

    uVar4  = (undefined2)(param_1 >> 0x10);
    puVar1 = (undefined4 *)*(uint *)((int)param_1 + 0xee);
    uVar2  = *(uint *)((int)param_1 + 0xf0);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

void __stdcall16far destroy_window_1018_c518(astruct_29 *param_1)

{
    BOOL16      BVar1;
    astruct_29 *iVar2;
    UINT16      uVar3;

    uVar3                  = (UINT16)((ulong)param_1 >> 0x10);
    iVar2                  = (astruct_29 *)param_1;
    *(undefined2 *)param_1 = 0xc8bc;
    iVar2->field_0x2       = 0x1018;
    fn_ptr_1000_17ce(*(astruct_18 **)&iVar2->field_0x108, 0x1000);
    if(iVar2->field_0x112 != 0x0)
    {
        BVar1 = IsWindow16(0x1000);
        if(BVar1 != 0x0)
        {
            DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
            iVar2->field_0x112 = 0x0;
        }
    }
    pass1_1020_022c((ushort *)param_1);
    return;
}

astruct_29 *__stdcall16far pass1_1018_c896(astruct_29 *param_1, byte param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}
void __stdcall16far unk_destroy_window_op_1018_6bb6(astruct_28 *param_1, HWND16 param_2)

{
    BOOL16      BVar1;
    astruct_28 *iVar2;
    undefined2  uVar2;
    HWND16      hwnd;

    uVar2 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (astruct_28 *)param_1;
    hwnd  = param_2;
    if(iVar2->field_0xea != 0x0)
    {
        hwnd = (HWND16)s_tile2_bmp_1050_1538;
        PostMessage16(param_2, 0x0, 0x0, CONCAT22(0x111, iVar2->field_0xea));
    }
    PostMessage16(hwnd, 0x0, 0x0, 0x1110079);
    if(iVar2->field_0xf0 != 0x0)
    {
        BVar1 = IsWindow16((HWND16)s_tile2_bmp_1050_1538);
        if(BVar1 != 0x0)
        {
            DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
            iVar2->field_0xf0 = 0x0;
        }
    }
    return;
}

void __stdcall16far win_1018_598c(astruct *param_1, ushort param_2, ushort param_3)

{
    uint         uVar1;
    astruct_131 *iVar1;
    undefined2   uVar2;
    ushort       unaff_SS;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar2 = (undefined2)((ulong)param_1 >> 0x10);
    iVar1 = (astruct_131 *)param_1;
    get_dc_1018_4db0(iVar1->field_0xf2, iVar1->field_0x8, 0x1008);
    mem_op_1000_179c(0x2a, (uchar *)param_3, 0x1000);
    uVar1 = param_3 | param_2;
    if(uVar1 != 0x0)
    {
        pass1_1018_5b06((astruct_132 *)param_2, param_3, iVar1->field_0x8, unaff_SS);
        iVar1->field_0xee = param_2;
        iVar1->field_0xf0 = uVar1;
        return;
    }
    *(undefined4 *)&iVar1->field_0xee = 0x0;
    return;
}


void __stdcall16far window_op_1018_67b6(astruct *param_1)

{
    astruct_658 *in_AX;
    uchar       *in_DX;
    uint         uVar1;
    int          iVar2;
    int          unaff_DI;
    undefined2   uVar3;
    ushort       unaff_SS;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar3 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (int)param_1;
    get_dc_1018_4db0(*(ULONG *)(iVar2 + 0xf2), *(ushort *)(iVar2 + 0x8), 0x1008);
    mem_op_1000_179c(0x18, in_DX, 0x1000);
    uVar1 = (uint)in_DX | (uint)in_AX;
    if(uVar1 != 0x0)
    {
        pass1_1018_6924(in_AX, (ushort)in_DX, *(ushort *)(iVar2 + 0x8), unaff_DI, unaff_SS);
        *(astruct_658 **)(iVar2 + 0xee) = in_AX;
        *(uint *)(iVar2 + 0xf0)         = uVar1;
        return;
    }
    *(undefined4 *)(iVar2 + 0xee) = 0x0;
    return;
}


void __stdcall16far pass1_1018_681a(ULONG param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    undefined2  uVar4;

    uVar4  = (undefined2)(param_1 >> 0x10);
    puVar1 = (undefined4 *)*(uint *)((int)param_1 + 0xee);
    uVar2  = *(uint *)((int)param_1 + 0xf0);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

void __stdcall16far win_op_1018_294a(int param_1, UINT16 param_2, UINT16 param_3, ULONG param_4, UINT16 param_5, LPCSTR in_string_6)

{
    if((*(int *)(param_1 + 0x18) != 0x0) && (param_4._2_2_ == 0x280))
    {
        *(undefined2 *)(param_1 + 0x18) = 0x0;
    }
    create_dc_1018_4e04((astruct_8 **)CONCAT22(param_2, param_1), param_3, (int)param_4, param_4._2_2_, in_string_6, param_5);
    return;
}

ulong __stdcall16far set_err_mode_1010_8b14(ulong param_1, ULONG param_2, ushort param_3)

{
    uint      uVar1;
    uint      uVar2;
    long      lVar3;
    undefined local_a[0x8];

    pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)((int)param_1 + 0xe84));
    SetErrorMode16(0x1008);
    do
    {
        lVar3 = pass1_1008_5b12(local_a, param_3);
        if(lVar3 == 0x0)
        {
            SetErrorMode16(0x1008);
            return (ulong)param_2;
        }
        uVar1 = (int)param_1 + 0xa82;
        unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (ulong)uVar1), *(char **)((int)lVar3 + 0x4));
        pass1_1000_3cea(param_1 & 0xffff0000 | (ulong)uVar1, param_2);
        uVar2 = dos3_call_1000_51aa((ushort)&stack0xfffe);
    } while(uVar2 != 0x0);
    SetErrorMode16(0x1000);
    return param_1 & 0xffff0000 | (ulong)uVar1;
}

void __stdcall16far send_msg_1010_7c42(ulong param_1, ushort param_2)

{
    int        iVar1;
    undefined2 uVar2;
    long       lVar3;
    undefined  local_a[0x8];

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if((*(uint *)(iVar1 + 0x1e) | *(uint *)(iVar1 + 0x1c)) != 0x0)
    {
        pass1_1008_5784((ulong *)CONCAT22(param_2, local_a), *(ulong *)(iVar1 + 0x1c));
        while(true)
        {
            lVar3 = pass1_1008_5b12(local_a, param_2);
            if(lVar3 == 0x0)
                break;
            SendMessage16(0x1008, 0x0, 0x0, 0x11100eb);
        }
    }
    return;
}

void __stdcall16far send_msg_1010_7c9e(ulong param_1, int param_2, ushort param_3)

{
    BOOL16     BVar1;
    int        iVar2;
    undefined2 uVar3;
    long       lVar4;
    ulong      uVar5;
    undefined  local_a[0x8];

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(((*(uint *)(iVar2 + 0x1e) | *(uint *)(iVar2 + 0x1c)) != 0x0) && (param_2 != 0x0))
    {
        pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)(iVar2 + 0x1c));
        while(true)
        {
            lVar4 = pass1_1008_5b12(local_a, param_3);
            uVar3 = (undefined2)((ulong)lVar4 >> 0x10);
            if(lVar4 == 0x0)
                break;
            if(*(long *)((int)lVar4 + 0x4) != 0x0)
            {
                uVar5 = struct_op_1030_73a8(*(ulong *)((int)lVar4 + 0x4));
                BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((int)uVar5 + 0xc), param_2);
                if(BVar1 != 0x0)
                {
                    SendMessage16(0x1008, 0x0, 0x0, 0x11100eb);
                }
            }
        }
    }
    return;
}

void __stdcall16far pass1_1010_71b0(int param_1, ushort param_2)

{
    ulong uVar1;

    uVar1 = *(ulong *)(param_1 + 0x6);
    send_msg_1010_7c42(uVar1 & 0xffff0000 | (ulong)((int)uVar1 - 0xa), param_2);
    return;
}


void __stdcall16far pass1_1010_71c2(uint param_1, ushort param_2, int param_3, UINT16 param_4)

{
    ulong      uVar1;
    undefined4 uVar2;
    int        iVar3;
    undefined2 uVar4;

    if(param_1 == 0x13)
    {
        uVar2 = *(undefined4 *)(param_3 + 0x6);
        uVar2 = *(undefined4 *)((int)uVar2 + 0x18);
        uVar1 = *(ulong *)(param_3 + 0x6);
        destroy_window_1010_7b26(uVar1 & 0xffff0000 | (ulong)((int)uVar1 - 0xa), *(long *)((int)uVar2 + 0x28), param_4, param_2);
        return;
    }
    if(param_1 < 0x14)
    {
        if((char)param_1 == '\x01')
        {
            uVar2                         = *(undefined4 *)(param_3 + 0x6);
            uVar4                         = (undefined2)((ulong)uVar2 >> 0x10);
            iVar3                         = (int)uVar2;
            *(undefined4 *)(iVar3 + 0xa)  = 0x0;
            *(undefined4 *)(iVar3 + 0x18) = 0x0;
            return;
        }
        if((char)param_1 == '\x05')
        {
            uVar1 = *(ulong *)(param_3 + 0x6);
            send_msg_1010_7c42(uVar1 & 0xffff0000 | (ulong)((int)uVar1 - 0xa), param_4);
            return;
        }
    }
    return;
}

void __stdcall16far unk_win_op_1010_7300(ulong param_1, ulong param_2, uint param_3, ulong param_4)

{
    undefined4  uVar1;
    code      **ppcVar2;
    char        cVar3;
    uint        uVar4;
    uchar      *in_DX;
    uchar      *puVar5;
    uchar      *extraout_DX;
    uchar      *puVar6;
    uchar      *extraout_DX_00;
    uchar      *puVar7;
    uchar      *puVar8;
    int         unaff_DI;
    undefined2  uVar9;
    HWND16      HVar10;
    HWND16      HVar11;
    ushort      unaff_SS;
    ulong       uVar12;
    astruct_57 *paVar13;
    ushort     *puVar14;
    LRESULT     LVar15;
    undefined2  uVar16;
    undefined   uVar17;
    undefined2  uVar18;
    undefined2 *puStack20;
    undefined2 *puStack14;
    undefined4 *puStack10;
    undefined4  uStack6;

    if(param_3 == 0x0)
    {
        return;
    }
    uStack6 = param_2;
    puVar8  = (uchar *)param_1;
    uVar9   = (undefined2)(param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        uVar1 = *(undefined4 *)(puVar8 + 0x14);
        pass1_1010_ad64((ushort)uVar1, CONCAT22(param_3, (int)((ulong)uVar1 >> 0x10)), param_4, 0x0, (ushort)in_DX);
        uStack6 = param_2 & 0xffff | ZEXT24(in_DX) << 0x10;
    }
    switch(param_3)
    {
    case 0x1:
    case 0x4:
    case 0x6:
    case 0x7:
    case 0x8:
    case 0x9:
    case 0xd:
    case 0xe:
    case 0x14:
    case 0x18:
        break;
    default:
        if((uStack6._2_2_ | (uint)uStack6) == 0x0)
        {
            return;
        }
    }
    pass1_1010_1f62(unaff_SS, param_1, 0xb);
    if(*(int *)(puVar8 + 0xe) == 0x0)
    {
        return;
    }
    puVar6 = puVar8;
    switch(param_3 - 0x1)
    {
    case 0x0:
        mem_op_1000_179c(0x94, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
        {
        LAB_1010_73fe:
            HVar10 = 0x1000;
            puVar6 = (uchar *)0x0;
            puVar5 = (uchar *)0x0;
        }
        else
        {
            HVar10 = (HWND16)&PTR_LOOP_1050_1040;
            pass1_1040_44d2((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8), CONCAT12((char)in_DX, puVar6)), param_4, *(ushort *)(puVar8 + 0xe), (uint)puVar6, puVar5);
        }
        break;
    default:
        mem_op_1000_179c(0x94, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_b040((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8), CONCAT12((char)in_DX, puVar6)), uStack6, *(ushort *)(puVar8 + 0xe));
        break;
    case 0x3:
        mem_op_1000_179c(0x9e, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_5626((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8), CONCAT12((char)in_DX, puVar6)), param_4, *(ushort *)(puVar8 + 0xe), puVar5);
        break;
    case 0x4:
        mem_op_1000_179c(0x94, in_DX, 0x1000);
        if(((uint)in_DX | (uint)puVar6) == 0x0)
            goto LAB_1010_73fe;
        HVar10  = (HWND16)&PTR_LOOP_1050_1040;
        puVar14 = pass1_1040_8e58((int)puVar6, (ushort)in_DX, (uint)uStack6, CONCAT22(*(undefined2 *)(puVar8 + 0xe), uStack6._2_2_));
        puVar5  = (uchar *)((ulong)puVar14 >> 0x10);
        puVar6  = (uchar *)puVar14;
        break;
    case 0x5:
    case 0x6:
        mem_op_1000_179c(0x98, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_48a0(CONCAT22(in_DX, puVar6), param_3, param_4, *(ushort *)(puVar8 + 0xe), puVar5, unaff_SS);
        break;
    case 0x7:
        mem_op_1000_179c(0x9c, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1038;
        pass1_1038_9144((ushort *)CONCAT22(in_DX, puVar6), *(ushort *)(puVar8 + 0xe), unaff_SS);
        break;
    case 0x8:
        mem_op_1000_179c(0xb8, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_b7ee((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8), CONCAT12((char)in_DX, puVar6)), uStack6, *(ushort *)(puVar8 + 0xe));
        break;
    case 0x9:
    case 0xa:
        mem_op_1000_179c(0x94, in_DX, 0x1000);
        if(((uint)in_DX | (uint)puVar6) == 0x0)
            goto LAB_1010_73fe;
        HVar10  = (HWND16)&PTR_LOOP_1050_1038;
        puVar14 = pass1_1038_9a1e((int)puVar6, (ushort)in_DX, (uint)uStack6, CONCAT22(*(undefined2 *)(puVar8 + 0xe), uStack6._2_2_));
        puVar5  = (uchar *)((ulong)puVar14 >> 0x10);
        puVar6  = (uchar *)puVar14;
        break;
    case 0xb:
        mem_op_1000_179c(0x12a, in_DX, 0x1000);
        if(((uint)in_DX | (uint)puVar6) == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1038;
        uVar12 = pass1_1038_9b72((int)puVar6, (ushort)in_DX, (uint)uStack6, CONCAT22(*(undefined2 *)(puVar8 + 0xe), uStack6._2_2_));
        puVar5 = (uchar *)(uVar12 >> 0x10);
        puVar6 = (uchar *)uVar12;
        break;
    case 0xc:
        mem_op_1000_179c(0x9c, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_6826((astruct_57 *)CONCAT22(in_DX, puVar6), *(ushort *)(puVar8 + 0xe));
        break;
    case 0xd:
        mem_op_1000_179c(0x9c, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_6fb6((astruct_57 *)CONCAT22(in_DX, puVar6), *(ushort *)(puVar8 + 0xe));
        break;
    case 0x12:
        mem_op_1000_179c(0x94, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        make_proc_inst_1040_a234(puVar6, in_DX, (uint)uStack6, CONCAT22(*(undefined2 *)(puVar8 + 0xe), uStack6._2_2_), (int)&PTR_LOOP_1050_1040);
        break;
    case 0x13:
        mem_op_1000_179c(0xb8, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_4e94((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8), CONCAT12((char)in_DX, puVar6)), uStack6, *(ushort *)(puVar8 + 0xe));
        break;
    case 0x14:
        mem_op_1000_179c(0x9a, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_0e1c((astruct_57 *)CONCAT22(in_DX, puVar6), 0x1, uStack6, *(ushort *)(puVar8 + 0xe), puVar5, unaff_DI, unaff_SS);
        break;
    case 0x15:
        mem_op_1000_179c(0x9c, in_DX, 0x1000);
        if(((uint)in_DX | (uint)puVar6) == 0x0)
            goto LAB_1010_73fe;
        HVar10  = (HWND16)&PTR_LOOP_1050_1040;
        paVar13 = pas1_1040_29c2((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8), CONCAT12((char)in_DX, puVar6)), uStack6, *(ushort *)(puVar8 + 0xe), (ushort)puVar6, (uint)in_DX | (uint)puVar6);
        puVar5  = (uchar *)((ulong)paVar13 >> 0x10);
        puVar6  = (uchar *)paVar13;
        break;
    case 0x16:
        mem_op_1000_179c(0x12a, in_DX, 0x1000);
        if(((uint)in_DX | (uint)puVar6) == 0x0)
            goto LAB_1010_73fe;
        HVar10  = (HWND16)&PTR_LOOP_1050_1038;
        puVar14 = pass1_1038_adde((int)puVar6, (ushort)in_DX, (uint)uStack6, CONCAT22(*(undefined2 *)(puVar8 + 0xe), uStack6._2_2_));
        puVar5  = (uchar *)((ulong)puVar14 >> 0x10);
        puVar6  = (uchar *)puVar14;
        break;
    case 0x17:
        mem_op_1000_179c(0xec, in_DX, 0x1000);
        puVar5 = (uchar *)((uint)in_DX | (uint)puVar6);
        if(puVar5 == (uchar *)0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_a640((astruct_57 *)CONCAT13((char)((uint)in_DX >> 0x8), CONCAT12((char)in_DX, puVar6)), param_4, *(ushort *)(puVar8 + 0xe));
    }
    puStack10 = (undefined4 *)CONCAT22(puVar5, puVar6);
    ppcVar2   = (code **)((int)*puStack10 + 0x8);
    (**ppcVar2)(HVar10, puVar6, puVar5);
    puVar7 = extraout_DX;
    HVar11 = HVar10;
    if(param_3 != 0x17)
    {
        if(0x17 < param_3)
            goto LAB_1010_7710;
        cVar3 = (char)param_3;
        if((cVar3 != '\x05') && (((char)(cVar3 + -0x5) < '\x05' || ('\x02' < (char)(cVar3 + -0xa)))))
            goto LAB_1010_7710;
    }
    if(*(int *)((uint)uStack6 + 0x16) != 0x0)
    {
        HVar11 = (HWND16)s_tile2_bmp_1050_1538;
        LVar15 = SendMessage16(HVar10, 0x0, 0x0, 0x11100f8);
        puVar7 = (uchar *)((ulong)LVar15 >> 0x10);
    }
LAB_1010_7710:
    HVar10 = HVar11;
    if(puStack10 != (undefined4 *)0x0)
    {
        uVar18 = *(undefined2 *)(puVar6 + 0x6);
        uVar17 = (undefined)HVar11;
        HVar10 = (HWND16)s_tile2_bmp_1050_1538;
        uVar4  = IsWindow16(HVar11);
        if(uVar4 != 0x0)
        {
            puVar6 = puVar7;
            if(*(long *)(puVar8 + 0x1c) == 0x0)
            {
                uVar17 = 0xc;
                mem_op_1000_179c(0xc, puVar7, 0x1000);
                puVar6 = (uchar *)((uint)puVar7 | uVar4);
                if(puVar6 == (uchar *)0x0)
                {
                    *(undefined4 *)(puVar8 + 0x1c) = 0x0;
                }
                else
                {
                    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar7, uVar4));
                    *(uint *)(puVar8 + 0x1c)   = uVar4;
                    *(uchar **)(puVar8 + 0x1e) = extraout_DX_00;
                    puVar6                     = extraout_DX_00;
                }
            }
            uVar16 = 0xc;
            mem_op_1000_179c(0xc, puVar6, 0x1000);
            puStack14 = (undefined2 *)CONCAT22(puVar6, uVar4);
            if(((uint)puVar6 | uVar4) == 0x0)
            {
                puStack20 = (undefined2 *)0x0;
            }
            else
            {
                *puStack14                   = 0x389a;
                *(undefined2 *)(uVar4 + 0x2) = 0x1008;
                *(ulong *)(uVar4 + 0x4)      = param_4;
                *(undefined4 *)(uVar4 + 0x8) = puStack10;
                *puStack14                   = 0x7e24;
                *(undefined2 *)(uVar4 + 0x2) = 0x1010;
                puStack20                    = puStack14;
            }
            ppcVar2 = (code **)((int)**(undefined4 **)(puVar8 + 0x1c) + 0x4);
            (**ppcVar2)(0x1000, *(undefined4 *)(puVar8 + 0x1c), (char)puStack20, (int)((ulong)puStack20 >> 0x10), uVar16, uVar17, uVar18);
            return;
        }
    }
    if(((uint)puVar5 | (uint)puVar6) != 0x0)
    {
        ppcVar2 = (code **)*puStack10;
        (**ppcVar2)(HVar10, puVar6, (char)puVar5, 0x1);
    }
    return;
}

void __stdcall16far free_rsrc_1010_4b3e(ushort *param_1, HGLOBAL16 param_2)

{
    int        *piVar1;
    undefined4 *puVar2;
    uint        uVar3;
    code      **ppcVar4;
    undefined4 *puVar5;
    undefined4  uVar6;
    BOOL16      BVar7;
    int         iVar8;
    int         iVar9;
    undefined2  uVar10;
    undefined2  uVar11;
    HGLOBAL16   HVar12;
    ushort      unaff_SS;
    int         iStack4;

    uVar10                       = (undefined2)((ulong)param_1 >> 0x10);
    iVar8                        = (int)param_1;
    *param_1                     = (int)s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
    *(undefined2 *)(iVar8 + 0x2) = 0x1010;
    HVar12                       = param_2;
    if(*(int *)(iVar8 + 0x2a) != 0x0)
    {
        HVar12 = (HGLOBAL16)s_tile2_bmp_1050_1538;
        BVar7  = GlobalUnlock16(param_2);
        if(BVar7 == 0x0)
        {
            HVar12 = (HGLOBAL16)s_tile2_bmp_1050_1538;
            FreeResource16((HGLOBAL16)s_tile2_bmp_1050_1538);
        }
    }
    *(undefined2 *)(iVar8 + 0x2a) = 0x0;
    if(**(long **)(iVar8 + 0x12) != 0x0)
    {
        iStack4 = 0x0;
        while(true)
        {
            puVar5 = (undefined4 *)*(undefined4 *)(iVar8 + 0x12);
            piVar1 = (int *)((int)puVar5 + 0x8);
            if(*piVar1 == iStack4 || *piVar1 < iStack4)
                break;
            uVar11 = (undefined2)((ulong)*puVar5 >> 0x10);
            iVar9  = (int)*puVar5;
            puVar2 = (undefined4 *)*(uint *)(iVar9 + iStack4 * 0x4);
            uVar3  = *(uint *)(iVar9 + iStack4 * 0x4 + 0x2);
            if((uVar3 | (uint)puVar2) != 0x0)
            {
                ppcVar4 = (code **)*puVar2;
                (**ppcVar4)(HVar12, puVar2, uVar3, 0x1);
            }
            iStack4 = iStack4 + 0x1;
        }
    }
    uVar6 = *(undefined4 *)(iVar8 + 0x12);
    fn_ptr_1000_17ce(*(astruct_18 **)((int)uVar6 + 0x4), 0x1000);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar8 + 0x12), 0x1000);
    puVar2 = (undefined4 *)*(uint *)(iVar8 + 0x16);
    uVar3  = *(uint *)(iVar8 + 0x18);
    if((uVar3 | (uint)puVar2) != 0x0)
    {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)(0x1000, puVar2, uVar3, 0x1);
    }
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar8 + 0x1a), 0x1000);
    pass1_1010_1d80(param_1, unaff_SS);
    return;
}

void __stdcall16far unk_destroy_win_op_1010_2fa0(ULONG param_1, HWND16 param_2)

{
    int       *piVar1;
    undefined4 uVar2;
    int        iVar3;
    undefined2 uVar4;
    HWND16     HVar5;
    int        iStack4;

    uVar4                         = (undefined2)(param_1 >> 0x10);
    iVar3                         = (int)param_1;
    *(undefined2 *)(iVar3 + 0x28) = 0x0;
    iStack4                       = 0x0;
    while(true)
    {
        piVar1 = (int *)(iVar3 + 0x16);
        if(*piVar1 == iStack4 || *piVar1 < iStack4)
            break;
        DestroyWindow16(param_2);
        iStack4 = iStack4 + 0x1;
        param_2 = (HWND16)s_tile2_bmp_1050_1538;
    }
    *(undefined2 *)(iVar3 + 0x16) = 0x0;
    if((*(uint *)(iVar3 + 0x54) | *(uint *)(iVar3 + 0x52)) != 0x0)
    {
        iStack4 = 0x0;
        do
        {
            uVar2 = *(undefined4 *)(iVar3 + 0x52);
            HVar5 = param_2;
            if(*(long *)((int)uVar2 + iStack4 * 0x4) != 0x0)
            {
                HVar5 = (HWND16)s_tile2_bmp_1050_1538;
                DestroyWindow16(param_2);
                uVar2                                       = *(undefined4 *)(iVar3 + 0x52);
                *(undefined4 *)((int)uVar2 + iStack4 * 0x4) = 0x0;
            }
            iStack4 = iStack4 + 0x1;
            param_2 = HVar5;
        } while(iStack4 < 0xa);
        fn_ptr_1000_17ce(*(astruct_18 **)(iVar3 + 0x52), 0x1000);
        *(undefined4 *)(iVar3 + 0x52) = 0x0;
    }
    return;
}

void __stdcall16far unk_destroy_win_op_1010_305a(astruct_27 *param_1, int param_2, astruct_65 *param_3, UINT16 param_4)

{
    int        *piVar1;
    ULONG       UVar2;
    long        lVar3;
    bool        bVar4;
    ushort      uVar5;
    astruct_27 *iVar4;
    int         iVar6;
    astruct_27 *uVar7;
    undefined2  uVar8;
    HWND16      hwnd;
    HWND16      hwnd_00;
    ushort      unaff_SS;
    int         iStack10;
    int         iStack8;
    int         iStack6;

    hwnd              = (HWND16)&PTR_LOOP_1050_1040;
    uVar5             = pass1_1040_c60e((ulong)param_3);
    uVar7             = (astruct_27 *)((ulong)param_1 >> 0x10);
    iVar4             = (astruct_27 *)param_1;
    iVar4->field_0x12 = uVar5;
    iVar4->field_0x14 = 0x0;
    iStack6           = 0x0;
    bVar4             = false;
    iVar4->field_0x28 = 0x0;
    iStack8           = 0x0;
    do
    {
        piVar1 = &iVar4->field_0x16;
        if(*piVar1 == iStack8 || *piVar1 < iStack8)
        {
        LAB_1010_30ad:
            iVar6 = iStack6;
            if(bVar4)
            {
                while(iStack8 = iVar6 + 0x1, piVar1 = &iVar4->field_0x16, *piVar1 != iStack8 && iStack8 <= *piVar1)
                {
                    DestroyWindow16(hwnd);
                    (&iVar4->field_0x2e)[iVar6] = 0x0;
                    hwnd                        = (HWND16)s_tile2_bmp_1050_1538;
                    iVar6                       = iStack8;
                }
                iVar4->field_0x16 = iStack6 + 0x1;
                pass1_1010_1f62(unaff_SS, (ulong)param_1, 0x9);
            }
            else
            {
                iVar6                             = iVar4->field_0x16;
                (&iVar4->field_0x2a)[iVar6 * 0x2] = (UINT16)param_3;
                (&iVar4->field_0x2c)[iVar6 * 0x2] = (UINT16)((ulong)param_3 >> 0x10);
                iStack10                          = 0xa;
                piVar1                            = &iVar4->field_0x16;
                *piVar1                           = *piVar1 + 0x1;
                if(0x1 < iVar4->field_0x16)
                {
                    UVar2    = (&iVar4->field_0x22)[iVar4->field_0x16];
                    iVar6    = (int)UVar2;
                    uVar8    = (undefined2)(UVar2 >> 0x10);
                    iStack10 = *(int *)(iVar6 + 0x20) + *(int *)(iVar6 + 0x24) + 0x8;
                }
                hwnd = (HWND16)&PTR_LOOP_1050_1040;
                mov_update_win_1040_93aa(param_3, iStack10, iVar4->field_0x1a, (int)&PTR_LOOP_1050_1040);
            }
            if(!bVar4)
            {
                pass1_1010_1f62(unaff_SS, (ulong)param_1, 0xa);
            }
            if(param_2 == 0x0)
            {
                if(iVar4->field_0x52 != 0x0)
                {
                    iStack8 = 0x0;
                    hwnd_00 = hwnd;
                    do
                    {
                        lVar3 = iVar4->field_0x52;
                        uVar8 = (undefined2)((ulong)lVar3 >> 0x10);
                        iVar6 = (int)lVar3;
                        hwnd  = hwnd_00;
                        if((*(long *)(iVar6 + iStack8 * 0x4) != 0x0) && (*(astruct_65 **)(iVar6 + iStack8 * 0x4) != param_3))
                        {
                            hwnd = (HWND16)s_tile2_bmp_1050_1538;
                            DestroyWindow16(hwnd_00);
                        }
                        lVar3                                       = iVar4->field_0x52;
                        *(undefined4 *)((int)lVar3 + iStack8 * 0x4) = 0x0;
                        iStack8                                     = iStack8 + 0x1;
                        hwnd_00                                     = hwnd;
                    } while(iStack8 < 0xa);
                }
                pass1_1010_32da((ulong *)param_1, (ulong)param_3, hwnd, unaff_SS);
                pass1_1010_1f62(unaff_SS, (ulong)param_1, 0x8);
            }
            return;
        }
        if(*(astruct_65 **)(&iVar4->field_0x2a + iStack8 * 0x2) == param_3)
        {
            bVar4   = true;
            iStack6 = iStack8;
            goto LAB_1010_30ad;
        }
        iStack8 = iStack8 + 0x1;
    } while(true);
}

void __stdcall16far pass1_1010_1656(int param_1, ushort param_2, ushort param_3, ulong param_4, UINT16 param_5, uchar *param_6, int param_7, ushort param_8)

{
    undefined4 uVar1;
    ushort     uVar2;
    ushort     uVar3;
    int        iVar4;
    undefined2 uVar5;
    ushort    *puVar6;
    ulong      uVar7;

    unk_destroy_win_op_1010_305a((astruct_27 *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    if(*(int *)(param_1 + 0x16) == 0x3)
    {
        puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_8, param_6, param_7);
        uVar1  = *(undefined4 *)(param_1 + 0x32);
        uVar1  = *(undefined4 *)((int)uVar1 + 0x42);
        uVar5  = (undefined2)((ulong)uVar1 >> 0x10);
        iVar4  = (int)uVar1;
        uVar1  = *(undefined4 *)(iVar4 + 0x16);
        uVar7  = struct_op_1030_73a8(*(ulong *)((int)uVar1 + 0x4));
        uVar2  = pass1_1010_7818((ulong)puVar6, uVar7);
        uVar1  = *(undefined4 *)(iVar4 + 0x16);
        uVar3  = uVar2;
        ui_op_1010_79aa(puVar6, 0x0, *(long *)((int)uVar1 + 0x4), param_8);
        if(uVar3 == 0x0)
        {
            uVar1 = *(undefined4 *)(iVar4 + 0x16);
            unk_win_op_1010_7300((ulong)puVar6, 0x0, uVar2, *(ulong *)((int)uVar1 + 0x4));
        }
    }
    return;
}
