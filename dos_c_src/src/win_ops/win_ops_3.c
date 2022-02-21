
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

void __stdcall16far pass1_1008_aa28(ulong param_1,uint param_2,WNDCLASS16 *param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint extraout_DX;
  int iVar3;
  undefined2 uVar4;
  undefined4 *puStack6;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(int *)(iVar3 + 0x414) != 0x0) {
    uVar2 = *(undefined4 *)(iVar3 + 0x410);
    if (*(int *)((int)uVar2 + 0x8) == 0x0) {
      *(undefined2 *)(iVar3 + 0x414) = 0x0;
      return;
    }
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x410) + 0x10);
    (**ppcVar1)();
    puStack6 = (undefined4 *)CONCAT22(extraout_DX,param_2);
    if ((extraout_DX | param_2) != 0x0) {
      win_1008_5c5c(param_3,param_2,extraout_DX | param_2,_PTR_LOOP_1050_02a0,*(ushort *)(param_2 + 0x4));
      if (puStack6 != (undefined4 *)0x0) {
        ppcVar1 = (code **)*puStack6;
        (**ppcVar1)();
      }
      return;
    }
  }
  return;
}
WPARAM16 __cdecl16far win_msg_op_1008_9498(MSG *in_msg_1,MSG16 *in_msg_2)

{
  BOOL16 BVar1;
  INT16 IVar2;
  MSG16 local_msg_1;
  
LAB_1008_949c:
  BVar1 = GetMessage16((MSG16 *)in_msg_1,0x0,0x0,0x0);
  if (BVar1 == 0x0) {
    return local_msg_1.wparam;
  }
  if (*(int *)((int)_PTR_LOOP_1050_5bc8 + 0x8) != 0x0) goto code_r0x100894cd;
  goto LAB_1008_94dc;
code_r0x100894cd:
  in_msg_1 = (MSG *)s_tile2_bmp_1050_1538;
  BVar1 = IsDialogMessage16((HWND16)s_tile2_bmp_1050_1538,&local_msg_1);
  if (BVar1 == 0x0) {
LAB_1008_94dc:
    if (PTR_LOOP_1050_0398 != (undefined *)0x0) {
      in_msg_1 = (MSG *)s_tile2_bmp_1050_1538;
      IVar2 = TranslateAccelerator16((HWND16)s_tile2_bmp_1050_1538,(HACCEL16)&local_msg_1,in_msg_2);
      if (IVar2 != 0x0) goto LAB_1008_949c;
    }
    TranslateMessage16((MSG16 *)s_tile2_bmp_1050_1538);
    in_msg_1 = (MSG *)s_tile2_bmp_1050_1538;
    DispatchMessage16((MSG16 *)s_tile2_bmp_1050_1538);
  }
  goto LAB_1008_949c;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl16far unk_win_msg_op_1008_9510(int *param_1,MSG16 *param_2,MSG16 *param_3)

{
  BOOL16 has_message;
  INT16 IVar1;
  MSG16 local_14;
  
LAB_1008_9578:
  if (*param_1 != 0x0) {
    has_message = GetMessage16(param_2,0x0,0x0,0x0);
    if (has_message != 0x0) {
      if (*(int *)((int)_PTR_LOOP_1050_5bc8 + 0x8) != 0x0) goto code_r0x10089538;
      goto LAB_1008_9547;
    }
  }
  return;
code_r0x10089538:
  param_2 = (MSG16 *)s_tile2_bmp_1050_1538;
  has_message = IsDialogMessage16((HWND16)s_tile2_bmp_1050_1538,&local_14);
  if (has_message == 0x0) {
LAB_1008_9547:
    if (PTR_LOOP_1050_0398 != (undefined *)0x0) {
      param_2 = (MSG16 *)s_tile2_bmp_1050_1538;
      IVar1 = TranslateAccelerator16((HWND16)s_tile2_bmp_1050_1538,(HACCEL16)&local_14,param_3);
      if (IVar1 != 0x0) goto LAB_1008_9578;
    }
    TranslateMessage16((MSG16 *)s_tile2_bmp_1050_1538);
    param_2 = (MSG16 *)s_tile2_bmp_1050_1538;
    DispatchMessage16((MSG16 *)s_tile2_bmp_1050_1538);
  }
  goto LAB_1008_9578;
}

void __stdcall16far send_msg_1008_9640(ulong param_1,ushort param_2,HWND16 param_3)

{
  if (*(int *)((int)param_1 + 0x8) != 0x0) {
    SendMessage16(param_3,0x0,0x0,CONCAT22(0x86,param_2));
  }
  return;
}


void __stdcall16far win_ui_reg_class_1008_96d2(astruct_20 *param_1,HINSTANCE16 in_h_inst_2,WNDCLASS16 *in_wnd_class_3)

{
  BOOL16 BVar1;
  ATOM AVar2;
  undefined2 name_1c;
  undefined2 uStack26;
  undefined2 uStack24;
  undefined4 uStack22;
  undefined *puStack18;
  undefined2 uStack16;
  undefined2 uStack14;
  undefined2 uStack12;
  undefined4 uStack10;
  int iStack6;
  undefined2 uStack4;
  
  iStack6 = (int)param_1 + 0x5b;
  BVar1 = GetClassInfo16(in_h_inst_2,(SEGPTR)&name_1c,in_wnd_class_3);
  if (BVar1 == 0x0) {
    name_1c = *(undefined2 *)((int)param_1 + 0xc8);
    uStack26 = 0x5632;
    uStack24 = 0x1008;
    uStack22 = 0x40000;
    puStack18 = PTR_LOOP_1050_038c;
    uStack16 = *(undefined2 *)((int)param_1 + 0xc2);
    uStack14 = *(undefined2 *)((int)param_1 + 0xc4);
    uStack12 = *(undefined2 *)((int)param_1 + 0xc6);
    uStack10 = 0x0;
    uStack4 = param_1._2_2_;
    AVar2 = RegisterClass16((WNDCLASS16 *)s_tile2_bmp_1050_1538);
    if (AVar2 == 0x0) {
      fn_ptr_op_1000_24cd(0x0,&stack0xfffe);
    }
  }
  return;
}


void __stdcall16far create_window_ex_1008_9760(astruct *in_struct_1,undefined2 param_2)

{
  undefined4 uVar1;
  HWND16 window_handle;
  astruct *struct_1;
  LPCSTR class_name;
  
  class_name = (LPCSTR)((ulong)in_struct_1 >> 0x10);
  struct_1 = (astruct *)in_struct_1;
  if (struct_1->field_0x8 == 0x0) {
    uVar1 = struct_1->field_0xac;
    window_handle =
         CreateWIndowEx16(CONCAT22(struct_1,param_2),class_name,PTR_LOOP_1050_038c,
                          CONCAT22(struct_1->field_0xbc,struct_1->field_0xca),struct_1->field_0xba,struct_1->field_0xb8,
                          struct_1->field_0xb6,struct_1->field_0xb4,(HWND16)uVar1,(HMENU16)((ulong)uVar1 >> 0x10),0x39e,
                          (LPVOID)&USHORT_1050_1050);
    struct_1->field_0x8 = window_handle;
  }
  if (struct_1->field_0x8 == 0x0) {
    fn_ptr_op_1000_24cd(0x0,&stack0xfffe);
  }
  return;
}


ulong __stdcall16far
unk_win_op_1008_97f2(ulong *param_1,int *param_2,WPARAM16 param_3,uchar *param_4,uint param_5,HWND16 param_6)

{
  code **ppcVar1;
  BOOL16 BVar2;
  undefined2 uVar3;
  int iVar4;
  uint uVar5;
  UINT16 msg;
  UINT16 wparam;
  ushort unaff_SS;
  ulong uVar6;
  undefined uVar7;
  undefined uVar8;
  char cVar9;
  
  msg = (UINT16)param_1;
  wparam = (UINT16)((ulong)param_1 >> 0x10);
  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566((int *)CONCAT22(param_3,param_2),(int)&PTR_LOOP_1050_1040);
    }
    else {
      ppcVar1 = (code **)((int)*param_1 + 0x70);
      (**ppcVar1)();
    }
    uVar5 = 0x1;
    goto LAB_1008_9a95;
  }
  uVar8 = (undefined)((ulong)param_1 >> 0x10);
  uVar7 = SUB41(param_1,0x0);
  if (param_5 < 0x2c) {
    param_6 = 0x1008;
    switch(param_5) {
    case 0x1:
      break;
    case 0x2:
      ppcVar1 = (code **)((int)*param_1 + 0x3c);
      (**ppcVar1)(0x1008,param_1);
      SetWindowLong16(0x1008,0x0,0x0);
      BVar2 = IsWindow16((HWND16)s_tile2_bmp_1050_1538);
      if (BVar2 != 0x0) {
        PostMessage16((HWND16)s_tile2_bmp_1050_1538,msg,wparam,0x11100c7);
      }
      break;
    case 0x3:
      ppcVar1 = (code **)((int)*param_1 + 0x54);
      (**ppcVar1)(0x8,uVar7,wparam,param_3,param_2);
      break;
    default:
      goto switchD_1008_9b30_caseD_4;
    case 0x5:
      ppcVar1 = (code **)((int)*param_1 + 0x58);
      (**ppcVar1)(0x8,uVar7,uVar8,param_3,param_2,param_4);
      break;
    case 0x7:
      ppcVar1 = (code **)((int)*param_1 + 0x50);
      (**ppcVar1)(0x8,param_1,param_4);
      break;
    case 0x8:
      ppcVar1 = (code **)((int)*param_1 + 0x74);
      (**ppcVar1)(0x8,param_1,param_4);
      break;
    case 0xd:
      ppcVar1 = (code **)((int)*param_1 + 0x84);
      iVar4 = (**ppcVar1)(0x8,uVar7,uVar8,param_2,CONCAT12(param_4._0_1_,param_3));
      goto LAB_1008_9ada;
    case 0xf:
      ppcVar1 = (code **)((int)*param_1 + 0x34);
      (**ppcVar1)(0x1008,param_1);
      break;
    case 0x10:
      ppcVar1 = (code **)((int)*param_1 + 0x38);
      uVar6 = (**ppcVar1)(0x1008,param_1);
      return uVar6;
    case 0x19:
      ppcVar1 = (code **)((int)*param_1 + 0x78);
      uVar3 = (**ppcVar1)(0x8,uVar7,uVar8,param_2,CONCAT12(param_4._0_1_,param_3));
      return CONCAT22(0x1050,uVar3);
    case 0x1c:
      ppcVar1 = (code **)((int)*param_1 + 0x30);
      (**ppcVar1)(0x8,param_1,param_4);
    }
  }
  else {
    cVar9 = (char)param_5;
    if (param_5 == 0x112) {
      if ((PTR_LOOP_1050_039a == (undefined *)0x0) &&
         (ppcVar1 = (code **)((int)*param_1 + 0x48), iVar4 = (**ppcVar1)(), iVar4 != 0x0)) {
        make_def_wnd_proc_1008_9ce6(msg,wparam,(UINT16)param_2,param_3,CONCAT13(0x1,CONCAT12(cVar9,param_4)),param_6);
      }
    }
    else {
      if (param_5 < 0x113) {
        if (param_5 == 0x86) {
          ppcVar1 = (code **)((int)*param_1 + 0x80);
          uVar6 = (**ppcVar1)();
          return uVar6;
        }
        if (param_5 < 0x87) {
          if (param_5 == 0x85) {
            ppcVar1 = (code **)((int)*param_1 + 0x7c);
            uVar6 = (**ppcVar1)();
            return uVar6;
          }
          if (param_5 < 0x86) {
            if (cVar9 == '7') {
              return (ulong)*(uint *)(msg + 0xc2);
            }
            if (cVar9 == 'A') {
              ppcVar1 = (code **)((int)*param_1 + 0x2c);
              (**ppcVar1)(param_6,param_1);
              goto switchD_1008_9b30_caseD_1;
            }
          }
switchD_1008_9b30_caseD_4:
          if ((param_5 < 0x400) || (0x7ffe < param_5)) {
            uVar6 = make_def_wnd_proc_1008_9ce6(msg,wparam,(UINT16)param_2,param_3,CONCAT22(param_5,param_4),param_6);
            return uVar6;
          }
          ppcVar1 = (code **)((int)*param_1 + 0x28);
          (**ppcVar1)((char)param_6,uVar7,uVar8,(char)param_2,param_3,CONCAT22(param_5,param_4));
        }
        else {
          if (param_5 == 0x100) {
            if (PTR_LOOP_1050_039a == (undefined *)0x0) {
              ppcVar1 = (code **)((int)*param_1 + 0x6c);
              (**ppcVar1)();
            }
          }
          else {
            if (param_5 == 0x102) {
              if (PTR_LOOP_1050_039a == (undefined *)0x0) {
                ppcVar1 = (code **)((int)*param_1 + 0x68);
                (**ppcVar1)();
              }
            }
            else {
              if (param_5 != 0x111) goto switchD_1008_9b30_caseD_4;
              if ((param_4 != PTR_LOOP_1050_039c) && (PTR_LOOP_1050_039a == (undefined *)0x0)) {
                if (param_2 == (int *)0x0) {
                  ppcVar1 = (code **)((int)*param_1 + 0x40);
                  (**ppcVar1)();
                }
                else {
                  ppcVar1 = (code **)((int)*param_1 + 0x44);
                  (**ppcVar1)();
                }
              }
            }
          }
        }
      }
      else {
        if (param_5 == 0x204) {
          if (PTR_LOOP_1050_039a == (undefined *)0x0) {
            ppcVar1 = (code **)((int)*param_1 + 0x60);
            (**ppcVar1)();
          }
        }
        else {
          if (param_5 < 0x205) {
            if (param_5 == 0x113) {
              if (_PTR_LOOP_1050_0388 != 0x0) {
                pass1_1008_932a(_PTR_LOOP_1050_0388,unaff_SS);
              }
            }
            else {
              if (param_5 == 0x117) {
                if (param_3 == 0x0) {
                  ppcVar1 = (code **)((int)*param_1 + 0x4c);
                  (**ppcVar1)();
                }
                else {
                  ppcVar1 = (code **)((int)*param_1 + 0x20);
                  (**ppcVar1)();
                }
              }
              else {
                if (param_5 != 0x201) goto switchD_1008_9b30_caseD_4;
                if (PTR_LOOP_1050_039a == (undefined *)0x0) {
                  ppcVar1 = (code **)((int)*param_1 + 0x5c);
                  (**ppcVar1)();
                }
              }
            }
          }
          else {
            if (param_5 == 0x210) {
              ppcVar1 = (code **)((int)*param_1 + 0x64);
              (**ppcVar1)();
            }
            else {
              if (param_5 == 0x30f) {
LAB_1008_9af8:
                ppcVar1 = (code **)((int)*param_1 + 0x8c);
                iVar4 = (**ppcVar1)(param_6,param_1);
LAB_1008_9ada:
                return (ulong)(long)iVar4;
              }
              if (param_5 == 0x311) {
                ppcVar1 = (code **)((int)*param_1 + 0x88);
                iVar4 = (**ppcVar1)();
                if (iVar4 != 0x0) goto LAB_1008_9af8;
              }
              else {
                if (param_5 != 0x3b9) goto switchD_1008_9b30_caseD_4;
                ppcVar1 = (code **)((int)*param_1 + 0x24);
                (**ppcVar1)();
              }
            }
          }
        }
      }
    }
  }
switchD_1008_9b30_caseD_1:
  uVar5 = 0x0;
LAB_1008_9a95:
  return (ulong)uVar5;
}

LRESULT __stdcall16far
make_def_wnd_proc_1008_9ce6
          (UINT16 param_1,UINT16 param_2,UINT16 in_msg_3,WPARAM16 param_4,LPARAM param_5,HWND16 in_hwnd_5)

{
  LRESULT LVar1;
  
  LVar1 = DefWindowProc16(in_hwnd_5,in_msg_3,param_4,param_5);
  return LVar1;
}


void __stdcall16far pass1_1008_9e5a(astruct_11 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined2 *puVar4;
  undefined2 uVar6;
  astruct_464 *uVar5;
  undefined2 uVar7;
  undefined2 *puStack8;
  int iStack4;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  uVar5 = (astruct_464 *)param_1;
  *(undefined2 *)param_1 = 0x9fb2;
  uVar5->field_0x2 = 0x1008;
  uVar5->field_0x1c = 0x9fca;
  uVar5->field_0x1e = 0x1008;
  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (param_1 == (astruct_11 *)0x0) {
      puVar4 = (undefined2 *)0x0;
      uVar6 = 0x0;
    }
    else {
      puVar4 = &uVar5->field_0x1c;
      uVar6 = uVar7;
    }
    pass1_1008_92b2(_PTR_LOOP_1050_0388,0x50,CONCAT22(uVar6,puVar4));
  }
  iStack4 = 0x0;
  do {
    puVar1 = (undefined4 *)*(uint *)(&uVar5[0x1].field_0x0 + iStack4 * 0x4);
    uVar2 = (&uVar5[0x1].field_0x2)[iStack4 * 0x2];
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0xc);
  if (param_1 == (astruct_11 *)0x0) {
    puVar4 = (undefined2 *)0x0;
    uVar7 = 0x0;
  }
  else {
    puVar4 = &uVar5->field_0x1c;
  }
  puStack8 = (undefined2 *)CONCAT22(uVar7,puVar4);
  *puStack8 = 0x389a;
  puVar4[0x1] = 0x1008;
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  return;
}


void __stdcall16far
post_win_msg_1008_a0e4
          (astruct_67 *param_1,long param_2,int param_3,ushort param_4,ulong param_5,int param_6,HWND16 param_7,
          ushort param_8)

{
  undefined4 *puVar1;
  code **ppcVar2;
  uint uVar3;
  bool bVar4;
  astruct_68 *puVar4;
  astruct_66 *uVar5;
  uint extraout_DX;
  uint uVar7;
  astruct_67 *iVar7;
  astruct_67 *uVar6;
  astruct_99 *paStack14;
  undefined local_a [0x8];
  
  uVar6 = (astruct_67 *)((ulong)param_1 >> 0x10);
  iVar7 = (astruct_67 *)param_1;
  pass1_1008_5784((ulong *)CONCAT22(param_8,local_a),(ulong)iVar7->field_0xa);
  bVar4 = false;
  do {
    puVar4 = (astruct_68 *)local_a;
    pass1_1008_5b12(puVar4,param_8);
    if ((extraout_DX | (uint)puVar4) == 0x0) goto LAB_1008_a146;
  } while (puVar4->field_0x4 != param_6);
  puVar4->field_0xc = puVar4->field_0xc + param_3;
  puVar4->field_0xe = puVar4->field_0xe + param_2;
  bVar4 = true;
LAB_1008_a146:
  if (!bVar4) {
    param_7 = 0x1000;
    paStack14 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_03a0);
    uVar7 = (uint)((ulong)paStack14 >> 0x10);
    uVar3 = (uint)paStack14;
    if ((uVar7 | uVar3) == 0x0) {
      paStack14 = (astruct_99 *)0x0;
    }
    else {
      paStack14->field_0x0 = 0x389a;
      *(undefined2 *)(uVar3 + 0x2) = 0x1008;
      *(int *)(uVar3 + 0x4) = param_6;
      *(ulong *)(uVar3 + 0x6) = param_5;
      *(ushort *)(uVar3 + 0xa) = param_4;
      *(int *)(uVar3 + 0xc) = param_3;
      *(long *)(uVar3 + 0xe) = param_2;
      paStack14->field_0x0 = 0xad8e;
      *(undefined2 *)(uVar3 + 0x2) = 0x1008;
    }
    puVar1 = iVar7->field_0xa;
    ppcVar2 = (code **)((int)*iVar7->field_0xa + 0x8);
    (**ppcVar2)(0x1000,(char)puVar1,(int)((ulong)puVar1 >> 0x10),(int)paStack14,(int)((ulong)paStack14 >> 0x10));
  }
  if (param_6 == 0x14) {
    PostMessage16(param_7,0x0,0x0,0x11100fc);
  }
  return;
}

ushort * __stdcall16far pass1_1008_91ba(ushort *param_1,HWND16 param_2)

{
  UINT16 UVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar2 + 0x2) = 0x1008;
  *(undefined2 *)(iVar2 + 0x4) = 0x0;
  set_struct_1008_574a((astruct_21 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar2 + 0x6)));
  *param_1 = 0x9416;
  *(undefined2 *)(iVar2 + 0x2) = 0x1008;
  _PTR_LOOP_1050_0388 = param_1;
  UVar1 = SetTimer16(param_2,0x0,0x0,(LPVOID)((int)&PTR_LOOP_1050_0000 + 0x1));
  if (UVar1 == 0x0) {
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  PTR_LOOP_1050_038a = (undefined *)((ulong)_PTR_LOOP_1050_0388 >> 0x10);
  return param_1;
}


void __stdcall16far kill_timer_1008_921c(ushort *param_1,HWND16 param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x9416;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  KillTimer16(param_2,0x1);
  _PTR_LOOP_1050_0388 = 0x0;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x6)));
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}


void __cdecl16far send_msg_1008_84ba(ushort param_1,ulong param_2,HWND16 param_3)

{
  int iVar1;
  undefined2 uVar2;
  undefined2 uStack4;
  
  uVar2 = (undefined2)(param_2 >> 0x10);
  iVar1 = (int)param_2;
  if ((*(byte *)(iVar1 + 0x4) & 0x4) == 0x0) {
    if ((*(byte *)(iVar1 + 0x4) & 0x8) == 0x0) {
      return;
    }
    uStack4 = 0x1;
  }
  else {
    uStack4 = 0x0;
  }
  SendMessage16(param_3,*(UINT16 *)(iVar1 + 0x2),0x0,CONCAT22(0x115,uStack4));
  return;
}



void __stdcall16far win_1008_5c9e(ulong param_1,ulong *param_2,ushort param_3,ushort param_4,WNDCLASS16 *param_5)

{
  win_1008_5c7c(param_1,*param_2,param_5,param_3,param_4);
  return;
}



HWND16 __stdcall16far create_window_1008_5e7e(UINT16 in_stock_obj_id,WNDCLASS16 *in_wnd_class)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  BOOL16 BVar3;
  ATOM AVar4;
  HWND16 window_handle_1;
  int iVar5;
  LPCSTR string_1;
  undefined4 *puVar6;
  undefined2 name;
  undefined2 uStack42;
  undefined2 uStack40;
  undefined2 uStack38;
  undefined2 uStack36;
  undefined *puStack34;
  undefined2 uStack32;
  undefined2 uStack30;
  HGDIOBJ16 HStack28;
  undefined4 uStack26;
  undefined4 *puStack22;
  undefined4 local_12 [0x4];
  
  puVar6 = local_12;
  string_1 = (LPCSTR)s_MciSoundWindow_1050_02bd;
  for (iVar5 = 0x3; iVar5 != 0x0; iVar5 = iVar5 + -0x1) {
    puVar2 = puVar6;
    puVar6 = puVar6 + 0x1;
    puVar1 = (undefined4 *)string_1;
    string_1 = (LPCSTR)((int)string_1 + 0x4);
    *puVar2 = *puVar1;
  }
  *(undefined2 *)puVar6 = *(undefined2 *)string_1;
  *(undefined *)(undefined2 *)((int)puVar6 + 0x2) = *(undefined *)(undefined2 *)((int)string_1 + 0x2);
  name = 0x2000;
  uStack42 = SUB42(&DAT_1050_5f44,0x0);
  uStack40 = 0x1008;
  uStack36 = 0x2;
  puStack34 = PTR_LOOP_1050_038c;
  uStack32 = 0x0;
  uStack30 = 0x0;
  uStack38 = 0x0;
  HStack28 = GetStockObject16(in_stock_obj_id);
  uStack26 = 0x0;
  puStack22 = local_12;
  BVar3 = GetClassInfo16((HINSTANCE16)s_tile2_bmp_1050_1538,(SEGPTR)&name,in_wnd_class);
  if (BVar3 == 0x0) {
    AVar4 = RegisterClass16((WNDCLASS16 *)s_tile2_bmp_1050_1538);
    if (AVar4 == 0x0) {
      OutputDebugString16((LPCSTR)s_tile2_bmp_1050_1538);
      return 0x0;
    }
  }
  window_handle_1 =
       CreateWindow16((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,0x0,
                      (INT16)PTR_LOOP_1050_0396,0x1,0x1,0x8000,0x8000,0x0,(LPVOID)0xcf);
  return window_handle_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT __stdcall16far make_def_win_proc_1008_5f44(UINT16 param_1,WPARAM16 in_wparam_2,LPARAM param_3,HWND16 in_hwnd_4)

{
  WORD WVar1;
  uchar *in_DX;
  int unaff_DI;
  WNDCLASS16 *unaff_SS;
  LRESULT LVar2;
  ushort *puVar3;
  
  if (param_3._2_2_ == 0x2) {
    WVar1 = GetWindowWord16(in_hwnd_4,0x0);
    mci_send_command_1008_5cb6(_PTR_LOOP_1050_02a0,WVar1,(ushort)s_tile2_bmp_1050_1538);
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,(ushort)unaff_SS,in_DX,unaff_DI);
    pass1_1008_aa28((ulong)puVar3,(uint)puVar3,unaff_SS);
  }
  else {
    if (param_3._2_2_ != 0x3b9) {
      LVar2 = DefWindowProc16(in_hwnd_4,param_1,in_wparam_2,param_3);
      return LVar2;
    }
    DestroyWindow16(in_hwnd_4);
  }
  return 0x0;
}


void __stdcall16far destroy_win_1008_628e(ULONG param_1,HWND16 param_2)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)((int)param_1 + 0xd2) + 0x14);
  (**ppcVar1)(param_2,(undefined4 *)((int)param_1 + 0xd2),param_1._2_2_);
  DestroyWindow16(param_2);
  *(undefined2 *)((int)param_1 + 0x8) = 0x0;
  return;
}

