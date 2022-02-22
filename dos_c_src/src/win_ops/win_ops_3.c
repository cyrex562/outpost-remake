
void window_op_1018_e6c6(astruct *param_1)

{
    astruct_660 *in_AX;
    u8          *in_DX;
    u16          uVar1;
    i16          iVar2;
    i16          unaff_DI;
    u16          uVar3;
    u16          unaff_SS;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    get_dc_1018_4db0(*(iVar2 + 0xf2), (iVar2 + 0x8), 0x1008);
    mem_op_1000_179c(0x18, in_DX, 0x1000);
    uVar1 = in_DX | in_AX;
    if(uVar1 != 0x0)
    {
        pass1_1018_e834(in_AX, in_DX, (iVar2 + 0x8), unaff_DI, unaff_SS);
        *(astruct_660 **)(iVar2 + 0xee) = in_AX;
        *(iVar2 + 0xf0)                 = uVar1;
        return;
    }
    (iVar2 + 0xee) = 0x0;
    return;
}

void pass1_1018_e72a(u32 param_1)

{
    undefined4 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    u16         uVar4;

    uVar4  = (param_1 >> 0x10);
    puVar1 = *(param_1 + 0xee);
    uVar2  = *(param_1 + 0xf0);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

void post_win_msg_1018_ea0a(u16 param_1, u16 param_2, i16 param_3, HWND16 param_4)

{
    if(param_3 == 0xed)
    {
        PostMessage16(param_4, 0x0, 0x0, 0x11100c6);
    }
    return;
}

void pass1_1018_ea66(u32 param_1, u16 param_2)

{
    code **ppcVar1;
    u8    *puVar2;
    i16    iVar3;
    u16    uVar4;
    u16   *puVar5;
    u8     local_6[0x4];

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if((iVar3 + 0xee) != 0x0)
    {
        ppcVar1 = ((iVar3 + 0xee) + 0x8);
        (**ppcVar1)();
    }
    if((iVar3 + 0xea) == 0x0)
    {
        (iVar3 + 0xea) = 0x1;
        puVar5         = pass1_1008_941a(CONCAT22(param_2, local_6), 0x1, 0x95);
        puVar2         = local_6;
        win_1008_5c9e(_PTR_LOOP_1050_02a0, CONCAT22(param_2, puVar2), puVar2, (puVar5 >> 0x10), param_2);
        (iVar3 + 0xec) = puVar2;
        unk_win_op_1010_7300(*(iVar3 + 0xf6), 0x0, 0x8, 0x0);
    }
    return;
}

void window_op_1018_eada(astruct *param_1)

{
    astruct_661 *in_AX;
    u8          *in_DX;
    u16          uVar1;
    i16          iVar2;
    u16          uVar3;
    u16          unaff_SS;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    get_dc_1018_4db0(*(iVar2 + 0xf2), (iVar2 + 0x8), 0x1008);
    mem_op_1000_179c(0x28, in_DX, 0x1000);
    uVar1 = in_DX | in_AX;
    if(uVar1 != 0x0)
    {
        pass1_1018_ec74(in_AX, in_DX, (iVar2 + 0x8), unaff_SS);
        *(astruct_661 **)(iVar2 + 0xee) = in_AX;
        *(iVar2 + 0xf0)                 = uVar1;
        return;
    }
    (iVar2 + 0xee) = 0x0;
    return;
}

void pass1_1018_eb3e(u32 param_1, u16 param_2)

{
    undefined4 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    i16         iVar4;
    u16         uVar5;
    i16         iVar6;
    u16         uVar7;

    uVar7  = (param_1 >> 0x10);
    iVar6  = param_1;
    puVar1 = *(iVar6 + 0xee);
    uVar2  = *(iVar6 + 0xf0);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    if((iVar6 + 0xf6) != 0x0)
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
        pass1_1010_1ea6(*(iVar6 + 0xf6), CONCAT22(uVar5, iVar4), param_2);
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

void pass1_1020_02ae(u32 param_1)

{
    undefined4 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    i16         iVar4;
    u16         uVar5;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_3cd0((iVar4 + 0xe2));
    destroy_win_1008_628e(param_1, 0x1008);
    puVar1 = *(iVar4 + 0xe6);
    uVar2  = *(iVar4 + 0xe8);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1008, puVar1, uVar2, 0x1);
    }
    (iVar4 + 0xe6) = 0x0;
    pass1_1010_1dda(*(iVar4 + 0xe2));
    (iVar4 + 0xe2) = 0x0;
    return;
}

void win_1020_0316(astruct *param_1, u8 *param_2, u16 param_3)

{
    undefined4   uVar1;
    u16          uVar2;
    u8          *puVar3;
    u8          *puVar4;
    astruct_273 *iVar1;
    i16          unaff_DI;
    u16          uVar5;
    u16         *puVar6;

    create_window_ex_1008_9760(param_1, 0x1008);
    puVar6            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x1, param_3, param_2, unaff_DI);
    puVar3            = (puVar6 >> 0x10);
    uVar5             = (param_1 >> 0x10);
    iVar1             = (astruct_273 *)param_1;
    iVar1->field_0xe2 = puVar6;
    iVar1->field_0xe4 = puVar3;
    uVar1             = &iVar1->field_0xe2;
    (uVar1 + 0x16)    = iVar1->field_0xea;
    uVar2             = iVar1->field_0xee;
    uVar1             = &iVar1->field_0xe2;
    *(uVar1 + 0x12)   = uVar2;
    struct_1010_3c52(*&iVar1->field_0xe2, iVar1->field_0xec, param_3);
    mem_op_1000_179c(0x12, puVar3, 0x1000);
    puVar4 = (puVar3 | uVar2);
    if(puVar4 != 0x0)
    {
        pass1_1020_04f6(CONCAT22(puVar3, uVar2), iVar1->field_0x8, puVar4, unaff_DI, param_3);
        iVar1->field_0xe6 = uVar2;
        iVar1->field_0xe8 = puVar4;
        return;
    }
    &iVar1->field_0xe6 = 0x0;
    return;
}

void post_msg_1020_03b2(u32 param_1, HWND16 param_2)

{
    undefined4 uVar1;

    uVar1 = (param_1 + 0xe2);
    PostMessage16(param_2, 0x0, 0x0, CONCAT22(0x111, (uVar1 + 0x16)));
    return;
}


void post_msg_1020_03d6(u32 param_1, HWND16 param_2)

{
    undefined4 uVar1;

    uVar1 = (param_1 + 0xe2);
    PostMessage16(param_2, 0x0, 0x0, CONCAT22(0x111, (uVar1 + 0x16)));
    return;
}


void post_msg_1020_03fa(u32 param_1, HWND16 param_2)

{
    undefined4 uVar1;

    uVar1 = (param_1 + 0xe2);
    PostMessage16(param_2, 0x0, 0x0, CONCAT22(0x111, (uVar1 + 0x16)));
    return;
}


void post_win_msg_1020_061c(u32 param_1, i16 param_2, HWND16 param_3)

{
    undefined4 uVar1;
    u16        uVar2;

    uVar2 = (param_1 >> 0x10);
    if(param_2 == 0x1)
    {
        (param_1 + 0x6) = 0x0;
        return;
    }
    if(param_2 != 0x2)
    {
        return;
    }
    uVar1 = (param_1 + 0x6);
    PostMessage16(param_3, 0x0, 0x0, CONCAT22(0x111, (uVar1 + 0x16)));
    return;
}

void pass1_1020_08b6(WNDCLASS16 *param_1, astruct_20 *param_2, u1616 param_3, u32 param_4)

{
    astruct_20 *iVar1;
    u16         uVar1;
    astruct_20 *paVar2;

    paVar2                        = unk_draw_op_1008_61b2(param_2, 0x1, param_3, param_4, (u1616)param_1);
    uVar1                         = (param_2 >> 0x10);
    iVar1                         = (astruct_20 *)param_2;
    &iVar1[0x1].field_0x4         = 0x0;
    (&iVar1[0x1].field_0x4 + 0x2) = 0x0;
    param_2->field_0x0            = 0xb0e;
    iVar1->field_0x2              = 0x1020;
    win_1008_5c5c(param_1, 0x0, (paVar2 >> 0x10), _PTR_LOOP_1050_02a0, 0x1d4);
    return;
}


void win_1018_df40(astruct *param_1, u16 param_2, u8 *param_3, u16 param_4)

{
    astruct_267 *iVar1;
    u16          uVar1;
    u16         *puVar2;

    create_window_ex_1008_9760(param_1, 0x1008);
    mem_op_1000_179c(0xa, param_3, 0x1000);
    iVar1 = (astruct_267 *)param_1;
    uVar1 = (param_1 >> 0x10);
    if((param_3 | param_2) != 0x0)
    {
        puVar2            = struct_1018_e100(CONCAT22(param_3, param_2), iVar1->field_0x8, (param_3 | param_2), param_4);
        iVar1->field_0xe2 = puVar2;
        iVar1->field_0xe4 = (puVar2 >> 0x10);
        return;
    }
    &iVar1->field_0xe2 = 0x0;
    return;
}


void pass1_1018_df92(u32 param_1)

{
    undefined4 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    i16         iVar4;
    u16         uVar5;

    destroy_win_1008_628e(param_1, 0x1008);
    uVar5  = (param_1 >> 0x10);
    iVar4  = param_1;
    puVar1 = *(iVar4 + 0xe2);
    uVar2  = *(iVar4 + 0xe4);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1008, puVar1, uVar2, 0x1);
    }
    (iVar4 + 0xe2) = 0x0;
    return;
}

void pass1_1018_e2cc(u32 param_1, u1616 param_2)

{
    i16         *piVar1;
    code       **ppcVar2;
    u16          uVar3;
    u32          uVar4;
    u8          *puVar5;
    u8          *puVar6;
    astruct_269 *iVar7;
    u16          uVar7;
    u16         *puVar8;
    undefined4  *puStack10;
    u8           local_6[0x4];

    uVar7 = (param_1 >> 0x10);
    iVar7 = (astruct_269 *)param_1;
    if(iVar7->field_0xee != 0x0)
    {
        ppcVar2 = (*iVar7->field_0xee + 0x8);
        (**ppcVar2)();
    }
    if(iVar7->field_0xea == 0x0)
    {
        iVar7->field_0xea = 0x1;
        puVar8            = pass1_1008_941a(CONCAT22(param_2, local_6), 0x1, 0x7a);
        puVar5            = (puVar8 >> 0x10);
        uVar4             = ZEXT24(local_6);
        win_1008_5c9e(_PTR_LOOP_1050_02a0, CONCAT22(param_2, local_6), local_6, puVar5, param_2);
        iVar7->field_0xec = uVar4;
        mem_op_1000_179c(0x112, puVar5, 0x1000);
        puVar6 = (puVar5 | uVar4);
        if(puVar6 == 0x0)
        {
            uVar3     = 0x0;
            puStack10 = 0x0;
        }
        else
        {
            piVar1  = &iVar7->field_0xcc;
            *piVar1 = *piVar1 + 0x1;
            struct_1020_3644((uVar4 & 0xffff | ZEXT24(puVar5) << 0x10), iVar7->field_0xcc, param_1, param_2);
            uVar3     = uVar4;
            puStack10 = (uVar4 & 0xffff | ZEXT24(puVar6) << 0x10);
        }
        pass1_1008_6978(param_1, 0x0, puStack10 & 0xffff0000 | uVar3, uVar3, puVar6);
        ppcVar2 = (*puStack10 + 0xc);
        (**ppcVar2)();
    }
    return;
}

void window_op_1018_e384(astruct *param_1)

{
    astruct_659 *in_AX;
    u8          *in_DX;
    u16          uVar1;
    i16          iVar2;
    i16          unaff_DI;
    u16          uVar3;
    u16          unaff_SS;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    get_dc_1018_4db0(*(iVar2 + 0xf2), (iVar2 + 0x8), 0x1008);
    mem_op_1000_179c(0x18, in_DX, 0x1000);
    uVar1 = in_DX | in_AX;
    if(uVar1 != 0x0)
    {
        pass1_1018_e4f2(in_AX, in_DX, (iVar2 + 0x8), unaff_DI, unaff_SS);
        *(astruct_659 **)(iVar2 + 0xee) = in_AX;
        *(iVar2 + 0xf0)                 = uVar1;
        return;
    }
    (iVar2 + 0xee) = 0x0;
    return;
}


void pass1_1018_e3e8(u32 param_1)

{
    undefined4 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    u16         uVar4;

    uVar4  = (param_1 >> 0x10);
    puVar1 = *(param_1 + 0xee);
    uVar2  = *(param_1 + 0xf0);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

void destroy_window_1018_c518(astruct_29 *param_1)

{
    BOOL16      BVar1;
    astruct_29 *iVar2;
    u1616       uVar3;

    uVar3            = (u1616)(param_1 >> 0x10);
    iVar2            = (astruct_29 *)param_1;
    param_1          = 0xc8bc;
    iVar2->field_0x2 = 0x1018;
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
    pass1_1020_022c(param_1);
    return;
}

astruct_29 *pass1_1018_c896(astruct_29 *param_1, byte param_2)

{
    destroy_window_1018_c518(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}
void unk_destroy_window_op_1018_6bb6(astruct_28 *param_1, HWND16 param_2)

{
    BOOL16      BVar1;
    astruct_28 *iVar2;
    u16         uVar2;
    HWND16      hwnd;

    uVar2 = (param_1 >> 0x10);
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

void win_1018_598c(astruct *param_1, u16 param_2, u16 param_3)

{
    u16          uVar1;
    astruct_131 *iVar1;
    u16          uVar2;
    u16          unaff_SS;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar2 = (param_1 >> 0x10);
    iVar1 = (astruct_131 *)param_1;
    get_dc_1018_4db0(iVar1->field_0xf2, iVar1->field_0x8, 0x1008);
    mem_op_1000_179c(0x2a, param_3, 0x1000);
    uVar1 = param_3 | param_2;
    if(uVar1 != 0x0)
    {
        pass1_1018_5b06((astruct_132 *)param_2, param_3, iVar1->field_0x8, unaff_SS);
        iVar1->field_0xee = param_2;
        iVar1->field_0xf0 = uVar1;
        return;
    }
    &iVar1->field_0xee = 0x0;
    return;
}


void window_op_1018_67b6(astruct *param_1)

{
    astruct_658 *in_AX;
    u8          *in_DX;
    u16          uVar1;
    i16          iVar2;
    i16          unaff_DI;
    u16          uVar3;
    u16          unaff_SS;

    create_window_ex_1008_9760(param_1, 0x1008);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    get_dc_1018_4db0(*(iVar2 + 0xf2), (iVar2 + 0x8), 0x1008);
    mem_op_1000_179c(0x18, in_DX, 0x1000);
    uVar1 = in_DX | in_AX;
    if(uVar1 != 0x0)
    {
        pass1_1018_6924(in_AX, in_DX, (iVar2 + 0x8), unaff_DI, unaff_SS);
        *(astruct_658 **)(iVar2 + 0xee) = in_AX;
        *(iVar2 + 0xf0)                 = uVar1;
        return;
    }
    (iVar2 + 0xee) = 0x0;
    return;
}


void pass1_1018_681a(u32 param_1)

{
    undefined4 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    u16         uVar4;

    uVar4  = (param_1 >> 0x10);
    puVar1 = *(param_1 + 0xee);
    uVar2  = *(param_1 + 0xf0);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

void win_op_1018_294a(i16 param_1, u1616 param_2, u1616 param_3, u32 param_4, u1616 param_5, LPCSTR in_string_6)

{
    if(((param_1 + 0x18) != 0x0) && (param_4._2_2_ == 0x280))
    {
        (param_1 + 0x18) = 0x0;
    }
    create_dc_1018_4e04((astruct_8 **)CONCAT22(param_2, param_1), param_3, param_4, param_4._2_2_, in_string_6, param_5);
    return;
}

u32 set_err_mode_1010_8b14(u32 param_1, u32 param_2, u16 param_3)

{
    u16  uVar1;
    u16  uVar2;
    long lVar3;
    u8   local_a[0x8];

    pass1_1008_5784(CONCAT22(param_3, local_a), *(param_1 + 0xe84));
    SetErrorMode16(0x1008);
    do
    {
        lVar3 = pass1_1008_5b12(local_a, param_3);
        if(lVar3 == 0x0)
        {
            SetErrorMode16(0x1008);
            return param_2;
        }
        uVar1 = param_1 + 0xa82;
        unk_str_op_1000_3d3e((param_1 & 0xffff0000 | uVar1), (lVar3 + 0x4));
        pass1_1000_3cea(param_1 & 0xffff0000 | uVar1, param_2);
        uVar2 = dos3_call_1000_51aa(&stack0xfffe);
    } while(uVar2 != 0x0);
    SetErrorMode16(0x1000);
    return param_1 & 0xffff0000 | uVar1;
}

void send_msg_1010_7c42(u32 param_1, u16 param_2)

{
    i16  iVar1;
    u16  uVar2;
    long lVar3;
    u8   local_a[0x8];

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if((*(iVar1 + 0x1e) | *(iVar1 + 0x1c)) != 0x0)
    {
        pass1_1008_5784(CONCAT22(param_2, local_a), *(iVar1 + 0x1c));
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

void send_msg_1010_7c9e(u32 param_1, i16 param_2, u16 param_3)

{
    BOOL16 BVar1;
    i16    iVar2;
    u16    uVar3;
    long   lVar4;
    u32    uVar5;
    u8     local_a[0x8];

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if(((*(iVar2 + 0x1e) | *(iVar2 + 0x1c)) != 0x0) && (param_2 != 0x0))
    {
        pass1_1008_5784(CONCAT22(param_3, local_a), *(iVar2 + 0x1c));
        while(true)
        {
            lVar4 = pass1_1008_5b12(local_a, param_3);
            uVar3 = (lVar4 >> 0x10);
            if(lVar4 == 0x0)
                break;
            if((lVar4 + 0x4) != 0x0)
            {
                uVar5 = struct_op_1030_73a8(*(lVar4 + 0x4));
                BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (uVar5 + 0xc), param_2);
                if(BVar1 != 0x0)
                {
                    SendMessage16(0x1008, 0x0, 0x0, 0x11100eb);
                }
            }
        }
    }
    return;
}

void pass1_1010_71b0(i16 param_1, u16 param_2)

{
    u32 uVar1;

    uVar1 = *(param_1 + 0x6);
    send_msg_1010_7c42(uVar1 & 0xffff0000 | (uVar1 - 0xa), param_2);
    return;
}


void pass1_1010_71c2(u16 param_1, u16 param_2, i16 param_3, u1616 param_4)

{
    u32        uVar1;
    undefined4 uVar2;
    i16        iVar3;
    u16        uVar4;

    if(param_1 == 0x13)
    {
        uVar2 = (param_3 + 0x6);
        uVar2 = (uVar2 + 0x18);
        uVar1 = *(param_3 + 0x6);
        destroy_window_1010_7b26(uVar1 & 0xffff0000 | (uVar1 - 0xa), (uVar2 + 0x28), param_4, param_2);
        return;
    }
    if(param_1 < 0x14)
    {
        if(param_1 == '\x01')
        {
            uVar2          = (param_3 + 0x6);
            uVar4          = (uVar2 >> 0x10);
            iVar3          = uVar2;
            (iVar3 + 0xa)  = 0x0;
            (iVar3 + 0x18) = 0x0;
            return;
        }
        if(param_1 == '\x05')
        {
            uVar1 = *(param_3 + 0x6);
            send_msg_1010_7c42(uVar1 & 0xffff0000 | (uVar1 - 0xa), param_4);
            return;
        }
    }
    return;
}

void unk_win_op_1010_7300(u32 param_1, u32 param_2, u16 param_3, u32 param_4)

{
    undefined4  uVar1;
    code      **ppcVar2;
    char        cVar3;
    u16         uVar4;
    u8         *in_DX;
    u8         *puVar5;
    u8         *extraout_DX;
    u8         *puVar6;
    u8         *extraout_DX_00;
    u8         *puVar7;
    u8         *puVar8;
    i16         unaff_DI;
    u16         uVar9;
    HWND16      HVar10;
    HWND16      HVar11;
    u16         unaff_SS;
    u32         uVar12;
    astruct_57 *paVar13;
    u16        *puVar14;
    LRESULT     LVar15;
    u16         uVar16;
    u8          uVar17;
    u16         uVar18;
    u16        *puStack20;
    u16        *puStack14;
    undefined4 *puStack10;
    undefined4  uStack6;

    if(param_3 == 0x0)
    {
        return;
    }
    uStack6 = param_2;
    puVar8  = param_1;
    uVar9   = (param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        uVar1 = (puVar8 + 0x14);
        pass1_1010_ad64(uVar1, CONCAT22(param_3, (uVar1 >> 0x10)), param_4, 0x0, in_DX);
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
        if((uStack6._2_2_ | uStack6) == 0x0)
        {
            return;
        }
    }
    pass1_1010_1f62(unaff_SS, param_1, 0xb);
    if((puVar8 + 0xe) == 0x0)
    {
        return;
    }
    puVar6 = puVar8;
    switch(param_3 - 0x1)
    {
    case 0x0:
        mem_op_1000_179c(0x94, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
        {
        LAB_1010_73fe:
            HVar10 = 0x1000;
            puVar6 = 0x0;
            puVar5 = 0x0;
        }
        else
        {
            HVar10 = (HWND16)&PTR_LOOP_1050_1040;
            pass1_1040_44d2(CONCAT13((in_DX >> 0x8), CONCAT12(in_DX, puVar6)), param_4, (puVar8 + 0xe), puVar6, puVar5);
        }
        break;
    default:
        mem_op_1000_179c(0x94, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_b040(CONCAT13((in_DX >> 0x8), CONCAT12(in_DX, puVar6)), uStack6, (puVar8 + 0xe));
        break;
    case 0x3:
        mem_op_1000_179c(0x9e, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_5626(CONCAT13((in_DX >> 0x8), CONCAT12(in_DX, puVar6)), param_4, (puVar8 + 0xe), puVar5);
        break;
    case 0x4:
        mem_op_1000_179c(0x94, in_DX, 0x1000);
        if((in_DX | puVar6) == 0x0)
            goto LAB_1010_73fe;
        HVar10  = (HWND16)&PTR_LOOP_1050_1040;
        puVar14 = pass1_1040_8e58(puVar6, in_DX, uStack6, CONCAT22((puVar8 + 0xe), uStack6._2_2_));
        puVar5  = (puVar14 >> 0x10);
        puVar6  = puVar14;
        break;
    case 0x5:
    case 0x6:
        mem_op_1000_179c(0x98, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_48a0(CONCAT22(in_DX, puVar6), param_3, param_4, (puVar8 + 0xe), puVar5, unaff_SS);
        break;
    case 0x7:
        mem_op_1000_179c(0x9c, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1038;
        pass1_1038_9144(CONCAT22(in_DX, puVar6), (puVar8 + 0xe), unaff_SS);
        break;
    case 0x8:
        mem_op_1000_179c(0xb8, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_b7ee(CONCAT13((in_DX >> 0x8), CONCAT12(in_DX, puVar6)), uStack6, (puVar8 + 0xe));
        break;
    case 0x9:
    case 0xa:
        mem_op_1000_179c(0x94, in_DX, 0x1000);
        if((in_DX | puVar6) == 0x0)
            goto LAB_1010_73fe;
        HVar10  = (HWND16)&PTR_LOOP_1050_1038;
        puVar14 = pass1_1038_9a1e(puVar6, in_DX, uStack6, CONCAT22((puVar8 + 0xe), uStack6._2_2_));
        puVar5  = (puVar14 >> 0x10);
        puVar6  = puVar14;
        break;
    case 0xb:
        mem_op_1000_179c(0x12a, in_DX, 0x1000);
        if((in_DX | puVar6) == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1038;
        uVar12 = pass1_1038_9b72(puVar6, in_DX, uStack6, CONCAT22((puVar8 + 0xe), uStack6._2_2_));
        puVar5 = (uVar12 >> 0x10);
        puVar6 = uVar12;
        break;
    case 0xc:
        mem_op_1000_179c(0x9c, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_6826(CONCAT22(in_DX, puVar6), (puVar8 + 0xe));
        break;
    case 0xd:
        mem_op_1000_179c(0x9c, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_6fb6(CONCAT22(in_DX, puVar6), (puVar8 + 0xe));
        break;
    case 0x12:
        mem_op_1000_179c(0x94, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        make_proc_inst_1040_a234(puVar6, in_DX, uStack6, CONCAT22((puVar8 + 0xe), uStack6._2_2_), &PTR_LOOP_1050_1040);
        break;
    case 0x13:
        mem_op_1000_179c(0xb8, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_4e94(CONCAT13((in_DX >> 0x8), CONCAT12(in_DX, puVar6)), uStack6, (puVar8 + 0xe));
        break;
    case 0x14:
        mem_op_1000_179c(0x9a, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_0e1c(CONCAT22(in_DX, puVar6), 0x1, uStack6, (puVar8 + 0xe), puVar5, unaff_DI, unaff_SS);
        break;
    case 0x15:
        mem_op_1000_179c(0x9c, in_DX, 0x1000);
        if((in_DX | puVar6) == 0x0)
            goto LAB_1010_73fe;
        HVar10  = (HWND16)&PTR_LOOP_1050_1040;
        paVar13 = pas1_1040_29c2(CONCAT13((in_DX >> 0x8), CONCAT12(in_DX, puVar6)), uStack6, (puVar8 + 0xe), puVar6, in_DX | puVar6);
        puVar5  = (paVar13 >> 0x10);
        puVar6  = paVar13;
        break;
    case 0x16:
        mem_op_1000_179c(0x12a, in_DX, 0x1000);
        if((in_DX | puVar6) == 0x0)
            goto LAB_1010_73fe;
        HVar10  = (HWND16)&PTR_LOOP_1050_1038;
        puVar14 = pass1_1038_adde(puVar6, in_DX, uStack6, CONCAT22((puVar8 + 0xe), uStack6._2_2_));
        puVar5  = (puVar14 >> 0x10);
        puVar6  = puVar14;
        break;
    case 0x17:
        mem_op_1000_179c(0xec, in_DX, 0x1000);
        puVar5 = (in_DX | puVar6);
        if(puVar5 == 0x0)
            goto LAB_1010_73fe;
        HVar10 = (HWND16)&PTR_LOOP_1050_1040;
        pass1_1040_a640(CONCAT13((in_DX >> 0x8), CONCAT12(in_DX, puVar6)), param_4, (puVar8 + 0xe));
    }
    puStack10 = CONCAT22(puVar5, puVar6);
    ppcVar2   = (*puStack10 + 0x8);
    (**ppcVar2)(HVar10, puVar6, puVar5);
    puVar7 = extraout_DX;
    HVar11 = HVar10;
    if(param_3 != 0x17)
    {
        if(0x17 < param_3)
            goto LAB_1010_7710;
        cVar3 = param_3;
        if((cVar3 != '\x05') && (((cVar3 + -0x5) < '\x05' || ('\x02' < (cVar3 + -0xa)))))
            goto LAB_1010_7710;
    }
    if((uStack6 + 0x16) != 0x0)
    {
        HVar11 = (HWND16)s_tile2_bmp_1050_1538;
        LVar15 = SendMessage16(HVar10, 0x0, 0x0, 0x11100f8);
        puVar7 = (LVar15 >> 0x10);
    }
LAB_1010_7710:
    HVar10 = HVar11;
    if(puStack10 != 0x0)
    {
        uVar18 = (puVar6 + 0x6);
        uVar17 = (undefined)HVar11;
        HVar10 = (HWND16)s_tile2_bmp_1050_1538;
        uVar4  = IsWindow16(HVar11);
        if(uVar4 != 0x0)
        {
            puVar6 = puVar7;
            if((puVar8 + 0x1c) == 0x0)
            {
                uVar17 = 0xc;
                mem_op_1000_179c(0xc, puVar7, 0x1000);
                puVar6 = (puVar7 | uVar4);
                if(puVar6 == 0x0)
                {
                    (puVar8 + 0x1c) = 0x0;
                }
                else
                {
                    set_struct_1008_574a(CONCAT22(puVar7, uVar4));
                    *(puVar8 + 0x1c) = uVar4;
                    (puVar8 + 0x1e)  = extraout_DX_00;
                    puVar6           = extraout_DX_00;
                }
            }
            uVar16 = 0xc;
            mem_op_1000_179c(0xc, puVar6, 0x1000);
            puStack14 = (u16 *)CONCAT22(puVar6, uVar4);
            if((puVar6 | uVar4) == 0x0)
            {
                puStack20 = (u16 *)0x0;
            }
            else
            {
                *puStack14     = 0x389a;
                (uVar4 + 0x2)  = 0x1008;
                *(uVar4 + 0x4) = param_4;
                (uVar4 + 0x8)  = puStack10;
                *puStack14     = 0x7e24;
                (uVar4 + 0x2)  = 0x1010;
                puStack20      = puStack14;
            }
            ppcVar2 = (**(undefined4 **)(puVar8 + 0x1c) + 0x4);
            (**ppcVar2)(0x1000, (puVar8 + 0x1c), puStack20, (puStack20 >> 0x10), uVar16, uVar17, uVar18);
            return;
        }
    }
    if((puVar5 | puVar6) != 0x0)
    {
        ppcVar2 = *puStack10;
        (**ppcVar2)(HVar10, puVar6, puVar5, 0x1);
    }
    return;
}

void free_rsrc_1010_4b3e(u16 *param_1, HGLOBAL16 param_2)

{
    i16        *piVar1;
    undefined4 *puVar2;
    u16         uVar3;
    code      **ppcVar4;
    undefined4 *puVar5;
    undefined4  uVar6;
    BOOL16      BVar7;
    i16         iVar8;
    i16         iVar9;
    u16         uVar10;
    u16         uVar11;
    HGLOBAL16   HVar12;
    u16         unaff_SS;
    i16         iStack4;

    uVar10        = (param_1 >> 0x10);
    iVar8         = param_1;
    *param_1      = s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
    (iVar8 + 0x2) = 0x1010;
    HVar12        = param_2;
    if((iVar8 + 0x2a) != 0x0)
    {
        HVar12 = (HGLOBAL16)s_tile2_bmp_1050_1538;
        BVar7  = GlobalUnlock16(param_2);
        if(BVar7 == 0x0)
        {
            HVar12 = (HGLOBAL16)s_tile2_bmp_1050_1538;
            FreeResource16((HGLOBAL16)s_tile2_bmp_1050_1538);
        }
    }
    (iVar8 + 0x2a) = 0x0;
    if(**(long **)(iVar8 + 0x12) != 0x0)
    {
        iStack4 = 0x0;
        while(true)
        {
            puVar5 = (iVar8 + 0x12);
            piVar1 = (puVar5 + 0x8);
            if(*piVar1 == iStack4 || *piVar1 < iStack4)
                break;
            uVar11 = (*puVar5 >> 0x10);
            iVar9  = *puVar5;
            puVar2 = *(iVar9 + iStack4 * 0x4);
            uVar3  = *(iVar9 + iStack4 * 0x4 + 0x2);
            if((uVar3 | puVar2) != 0x0)
            {
                ppcVar4 = *puVar2;
                (**ppcVar4)(HVar12, puVar2, uVar3, 0x1);
            }
            iStack4 = iStack4 + 0x1;
        }
    }
    uVar6 = (iVar8 + 0x12);
    fn_ptr_1000_17ce(*(astruct_18 **)(uVar6 + 0x4), 0x1000);
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar8 + 0x12), 0x1000);
    puVar2 = *(iVar8 + 0x16);
    uVar3  = *(iVar8 + 0x18);
    if((uVar3 | puVar2) != 0x0)
    {
        ppcVar4 = *puVar2;
        (**ppcVar4)(0x1000, puVar2, uVar3, 0x1);
    }
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar8 + 0x1a), 0x1000);
    pass1_1010_1d80(param_1, unaff_SS);
    return;
}

void unk_destroy_win_op_1010_2fa0(u32 param_1, HWND16 param_2)

{
    i16       *piVar1;
    undefined4 uVar2;
    i16        iVar3;
    u16        uVar4;
    HWND16     HVar5;
    i16        iStack4;

    uVar4          = (param_1 >> 0x10);
    iVar3          = param_1;
    (iVar3 + 0x28) = 0x0;
    iStack4        = 0x0;
    while(true)
    {
        piVar1 = (iVar3 + 0x16);
        if(*piVar1 == iStack4 || *piVar1 < iStack4)
            break;
        DestroyWindow16(param_2);
        iStack4 = iStack4 + 0x1;
        param_2 = (HWND16)s_tile2_bmp_1050_1538;
    }
    (iVar3 + 0x16) = 0x0;
    if((*(iVar3 + 0x54) | *(iVar3 + 0x52)) != 0x0)
    {
        iStack4 = 0x0;
        do
        {
            uVar2 = (iVar3 + 0x52);
            HVar5 = param_2;
            if((uVar2 + iStack4 * 0x4) != 0x0)
            {
                HVar5 = (HWND16)s_tile2_bmp_1050_1538;
                DestroyWindow16(param_2);
                uVar2                   = (iVar3 + 0x52);
                (uVar2 + iStack4 * 0x4) = 0x0;
            }
            iStack4 = iStack4 + 0x1;
            param_2 = HVar5;
        } while(iStack4 < 0xa);
        fn_ptr_1000_17ce(*(astruct_18 **)(iVar3 + 0x52), 0x1000);
        (iVar3 + 0x52) = 0x0;
    }
    return;
}

void unk_destroy_win_op_1010_305a(astruct_27 *param_1, i16 param_2, astruct_65 *param_3, u1616 param_4)

{
    i16        *piVar1;
    u32         UVar2;
    long        lVar3;
    bool        bVar4;
    u16         uVar5;
    astruct_27 *iVar4;
    i16         iVar6;
    astruct_27 *uVar7;
    u16         uVar8;
    HWND16      hwnd;
    HWND16      hwnd_00;
    u16         unaff_SS;
    i16         iStack10;
    i16         iStack8;
    i16         iStack6;

    hwnd              = (HWND16)&PTR_LOOP_1050_1040;
    uVar5             = pass1_1040_c60e(param_3);
    uVar7             = (astruct_27 *)(param_1 >> 0x10);
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
                pass1_1010_1f62(unaff_SS, param_1, 0x9);
            }
            else
            {
                iVar6                             = iVar4->field_0x16;
                (&iVar4->field_0x2a)[iVar6 * 0x2] = (u1616)param_3;
                (&iVar4->field_0x2c)[iVar6 * 0x2] = (u1616)(param_3 >> 0x10);
                iStack10                          = 0xa;
                piVar1                            = &iVar4->field_0x16;
                *piVar1                           = *piVar1 + 0x1;
                if(0x1 < iVar4->field_0x16)
                {
                    UVar2    = (&iVar4->field_0x22)[iVar4->field_0x16];
                    iVar6    = UVar2;
                    uVar8    = (UVar2 >> 0x10);
                    iStack10 = (iVar6 + 0x20) + (iVar6 + 0x24) + 0x8;
                }
                hwnd = (HWND16)&PTR_LOOP_1050_1040;
                mov_update_win_1040_93aa(param_3, iStack10, iVar4->field_0x1a, &PTR_LOOP_1050_1040);
            }
            if(!bVar4)
            {
                pass1_1010_1f62(unaff_SS, param_1, 0xa);
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
                        uVar8 = (lVar3 >> 0x10);
                        iVar6 = lVar3;
                        hwnd  = hwnd_00;
                        if(((iVar6 + iStack8 * 0x4) != 0x0) && (*(astruct_65 **)(iVar6 + iStack8 * 0x4) != param_3))
                        {
                            hwnd = (HWND16)s_tile2_bmp_1050_1538;
                            DestroyWindow16(hwnd_00);
                        }
                        lVar3                   = iVar4->field_0x52;
                        (lVar3 + iStack8 * 0x4) = 0x0;
                        iStack8                 = iStack8 + 0x1;
                        hwnd_00                 = hwnd;
                    } while(iStack8 < 0xa);
                }
                pass1_1010_32da(param_1, param_3, hwnd, unaff_SS);
                pass1_1010_1f62(unaff_SS, param_1, 0x8);
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

void pass1_1010_1656(i16 param_1, u16 param_2, u16 param_3, u32 param_4, u1616 param_5, u8 *param_6, i16 param_7, u16 param_8)

{
    undefined4 uVar1;
    u16        uVar2;
    u16        uVar3;
    i16        iVar4;
    u16        uVar5;
    u16       *puVar6;
    u32        uVar7;

    unk_destroy_win_op_1010_305a((astruct_27 *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    if((param_1 + 0x16) == 0x3)
    {
        puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_8, param_6, param_7);
        uVar1  = (param_1 + 0x32);
        uVar1  = (uVar1 + 0x42);
        uVar5  = (uVar1 >> 0x10);
        iVar4  = uVar1;
        uVar1  = (iVar4 + 0x16);
        uVar7  = struct_op_1030_73a8(*(uVar1 + 0x4));
        uVar2  = pass1_1010_7818(puVar6, uVar7);
        uVar1  = (iVar4 + 0x16);
        uVar3  = uVar2;
        ui_op_1010_79aa(puVar6, 0x0, (uVar1 + 0x4), param_8);
        if(uVar3 == 0x0)
        {
            uVar1 = (iVar4 + 0x16);
            unk_win_op_1010_7300(puVar6, 0x0, uVar2, *(uVar1 + 0x4));
        }
    }
    return;
}

void set_window_placement_1010_0070(u32 param_1, i16 param_2, u16 param_3, HWND16 param_4, u16 param_5)

{
    code      **ppcVar1;
    u16         uVar2;
    undefined4 *puVar3;
    long        lVar4;
    u8          local_18[0x6];
    i1616       IStack18;
    i16         iStack16;
    i1616       IStack14;
    i1616       IStack12;
    i1616       IStack10;
    i1616       IStack8;
    u16         uStack6;
    u16         uStack4;

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
        uVar2          = (lVar4 >> 0x10);
        puVar3         = (lVar4 + 0xe0);
        ppcVar1        = (*puVar3 + 0x38);
        (**ppcVar1)(s_tile2_bmp_1050_1538, puVar3, (lVar4 + 0xe2), param_3);
        pass1_1010_01f8(param_1, CONCAT22(param_5, local_18), puVar3);
        SetWindowPlacement16((HWND16)s_tile2_bmp_1050_1538, (WINDOWPLACEMENT16 *)local_18);
    }
    return;
}


void set_win_placement_1010_010e(u16 param_1, u16 param_2, u16 param_3, HWND16 param_4)

{
    code            **ppcVar1;
    i16               iVar2;
    i16              *piVar3;
    u16               uVar4;
    undefined4       *puVar5;
    u16               extraout_DX;
    long              lVar6;
    WINDOWPLACEMENT16 local_18;
    i16               iStack6;
    i16               iStack4;

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
        uVar4   = (lVar6 >> 0x10);
        puVar5  = (lVar6 + 0xe0);
        ppcVar1 = (*puVar5 + 0x1c);
        (**ppcVar1)(s_tile2_bmp_1050_1538, puVar5, (lVar6 + 0xe2), param_3);
        iVar2                         = puVar5;
        piVar3                        = (puVar5 & 0xffff | extraout_DX << 0x10);
        local_18.show_cmd             = 0x9;
        local_18.rc_normal_position.x = *piVar3;
        local_18.rc_normal_position.y = (iVar2 + 0x2);
        iStack6                       = (iVar2 + 0x4) + *piVar3;
        iStack4                       = (iVar2 + 0x2) + (iVar2 + 0x6);
        SetWindowPlacement16((HWND16)s_tile2_bmp_1050_1538, &local_18);
    }
    return;
}


void enum_child_windows_1010_01be(LPVOID param_1)

{
    LPVOID pvVar1;

    if(PTR_LOOP_1050_0010 == 0x0)
    {
        pvVar1 = MakeProcInstance16(param_1, PTR_LOOP_1050_038c);
        EnumChildWindows1((HWND16)s_tile2_bmp_1050_1538, 0x0, ZEXT24(pvVar1) << 0x10);
        FreeProcInstance16(s_tile2_bmp_1050_1538);
    }
    return;
}

void pass1_1008_aa28(u32 param_1, u16 param_2, WNDCLASS16 *param_3)

{
    code      **ppcVar1;
    undefined4  uVar2;
    u16         extraout_DX;
    i16         iVar3;
    u16         uVar4;
    undefined4 *puStack6;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if((iVar3 + 0x414) != 0x0)
    {
        uVar2 = (iVar3 + 0x410);
        if((uVar2 + 0x8) == 0x0)
        {
            (iVar3 + 0x414) = 0x0;
            return;
        }
        ppcVar1 = ((iVar3 + 0x410) + 0x10);
        (**ppcVar1)();
        puStack6 = CONCAT22(extraout_DX, param_2);
        if((extraout_DX | param_2) != 0x0)
        {
            win_1008_5c5c(param_3, param_2, extraout_DX | param_2, _PTR_LOOP_1050_02a0, (param_2 + 0x4));
            if(puStack6 != 0x0)
            {
                ppcVar1 = *puStack6;
                (**ppcVar1)();
            }
            return;
        }
    }
    return;
}
WPARAM16 win_msg_op_1008_9498(MSG *in_msg_1, MSG16 *in_msg_2)

{
    BOOL16 BVar1;
    i1616  IVar2;
    MSG16  local_msg_1;

LAB_1008_949c:
    BVar1 = GetMessage16(in_msg_1, 0x0, 0x0, 0x0);
    if(BVar1 == 0x0)
    {
        return local_msg_1.wparam;
    }
    if((_PTR_LOOP_1050_5bc8 + 0x8) != 0x0)
        goto code_r0x100894cd;
    goto LAB_1008_94dc;
code_r0x100894cd:
    in_msg_1 = s_tile2_bmp_1050_1538;
    BVar1    = IsDialogMessage16((HWND16)s_tile2_bmp_1050_1538, &local_msg_1);
    if(BVar1 == 0x0)
    {
    LAB_1008_94dc:
        if(PTR_LOOP_1050_0398 != 0x0)
        {
            in_msg_1 = s_tile2_bmp_1050_1538;
            IVar2    = TranslateAccelerator16((HWND16)s_tile2_bmp_1050_1538, (HACCEL16)&local_msg_1, in_msg_2);
            if(IVar2 != 0x0)
                goto LAB_1008_949c;
        }
        TranslateMessage16(s_tile2_bmp_1050_1538);
        in_msg_1 = s_tile2_bmp_1050_1538;
        DispatchMessage16(s_tile2_bmp_1050_1538);
    }
    goto LAB_1008_949c;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_win_msg_op_1008_9510(i16 *param_1, MSG16 *param_2, MSG16 *param_3)

{
    BOOL16 has_message;
    i1616  IVar1;
    MSG16  local_14;

LAB_1008_9578:
    if(*param_1 != 0x0)
    {
        has_message = GetMessage16(param_2, 0x0, 0x0, 0x0);
        if(has_message != 0x0)
        {
            if((_PTR_LOOP_1050_5bc8 + 0x8) != 0x0)
                goto code_r0x10089538;
            goto LAB_1008_9547;
        }
    }
    return;
code_r0x10089538:
    param_2     = s_tile2_bmp_1050_1538;
    has_message = IsDialogMessage16((HWND16)s_tile2_bmp_1050_1538, &local_14);
    if(has_message == 0x0)
    {
    LAB_1008_9547:
        if(PTR_LOOP_1050_0398 != 0x0)
        {
            param_2 = s_tile2_bmp_1050_1538;
            IVar1   = TranslateAccelerator16((HWND16)s_tile2_bmp_1050_1538, (HACCEL16)&local_14, param_3);
            if(IVar1 != 0x0)
                goto LAB_1008_9578;
        }
        TranslateMessage16(s_tile2_bmp_1050_1538);
        param_2 = s_tile2_bmp_1050_1538;
        DispatchMessage16(s_tile2_bmp_1050_1538);
    }
    goto LAB_1008_9578;
}

void send_msg_1008_9640(u32 param_1, u16 param_2, HWND16 param_3)

{
    if((param_1 + 0x8) != 0x0)
    {
        SendMessage16(param_3, 0x0, 0x0, CONCAT22(0x86, param_2));
    }
    return;
}


void win_ui_reg_class_1008_96d2(astruct_20 *param_1, HINSTANCE16 in_h_inst_2, WNDCLASS16 *in_wnd_class_3)

{
    BOOL16     BVar1;
    ATOM       AVar2;
    u16        name_1c;
    u16        uStack26;
    u16        uStack24;
    undefined4 uStack22;
    u8        *puStack18;
    u16        uStack16;
    u16        uStack14;
    u16        uStack12;
    undefined4 uStack10;
    i16        iStack6;
    u16        uStack4;

    iStack6 = param_1 + 0x5b;
    BVar1   = GetClassInfo16(in_h_inst_2, (SEGPTR)&name_1c, in_wnd_class_3);
    if(BVar1 == 0x0)
    {
        name_1c   = (param_1 + 0xc8);
        uStack26  = 0x5632;
        uStack24  = 0x1008;
        uStack22  = 0x40000;
        puStack18 = PTR_LOOP_1050_038c;
        uStack16  = (param_1 + 0xc2);
        uStack14  = (param_1 + 0xc4);
        uStack12  = (param_1 + 0xc6);
        uStack10  = 0x0;
        uStack4   = param_1._2_2_;
        AVar2     = RegisterClass16(s_tile2_bmp_1050_1538);
        if(AVar2 == 0x0)
        {
            fn_ptr_op_1000_24cd(0x0, &stack0xfffe);
        }
    }
    return;
}


void create_window_ex_1008_9760(astruct *in_struct_1, u16 param_2)

{
    undefined4 uVar1;
    HWND16     window_handle;
    astruct   *struct_1;
    LPCSTR     class_name;

    class_name = (LPCSTR)(in_struct_1 >> 0x10);
    struct_1   = (astruct *)in_struct_1;
    if(struct_1->field_0x8 == 0x0)
    {
        uVar1               = struct_1->field_0xac;
        window_handle       = CreateWIndowEx16(CONCAT22(struct_1, param_2),
                                         class_name,
                                         PTR_LOOP_1050_038c,
                                         CONCAT22(struct_1->field_0xbc, struct_1->field_0xca),
                                         struct_1->field_0xba,
                                         struct_1->field_0xb8,
                                         struct_1->field_0xb6,
                                         struct_1->field_0xb4,
                                         (HWND16)uVar1,
                                         (HMENU16)(uVar1 >> 0x10),
                                         0x39e,
                                         &USHORT_1050_1050);
        struct_1->field_0x8 = window_handle;
    }
    if(struct_1->field_0x8 == 0x0)
    {
        fn_ptr_op_1000_24cd(0x0, &stack0xfffe);
    }
    return;
}


u32 __stdcall16far unk_win_op_1008_97f2(u32 *param_1, i16 *param_2, WPARAM16 param_3, u8 *param_4, u16 param_5, HWND16 param_6)

{
    code **ppcVar1;
    BOOL16 BVar2;
    u16    uVar3;
    i16    iVar4;
    u16    uVar5;
    u1616  msg;
    u1616  wparam;
    u16    unaff_SS;
    u32    uVar6;
    u8     uVar7;
    u8     uVar8;
    char   cVar9;

    msg    = (u1616)param_1;
    wparam = (u1616)(param_1 >> 0x10);
    if(param_5 == 0x2b)
    {
        if(*param_2 == 0x4)
        {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), &PTR_LOOP_1050_1040);
        }
        else
        {
            ppcVar1 = (*param_1 + 0x70);
            (**ppcVar1)();
        }
        uVar5 = 0x1;
        goto LAB_1008_9a95;
    }
    uVar8 = (undefined)(param_1 >> 0x10);
    uVar7 = SUB41(param_1, 0x0);
    if(param_5 < 0x2c)
    {
        param_6 = 0x1008;
        switch(param_5)
        {
        case 0x1:
            break;
        case 0x2:
            ppcVar1 = (*param_1 + 0x3c);
            (**ppcVar1)(0x1008, param_1);
            SetWindowLong16(0x1008, 0x0, 0x0);
            BVar2 = IsWindow16((HWND16)s_tile2_bmp_1050_1538);
            if(BVar2 != 0x0)
            {
                PostMessage16((HWND16)s_tile2_bmp_1050_1538, msg, wparam, 0x11100c7);
            }
            break;
        case 0x3:
            ppcVar1 = (*param_1 + 0x54);
            (**ppcVar1)(0x8, uVar7, wparam, param_3, param_2);
            break;
        default:
            goto switchD_1008_9b30_caseD_4;
        case 0x5:
            ppcVar1 = (*param_1 + 0x58);
            (**ppcVar1)(0x8, uVar7, uVar8, param_3, param_2, param_4);
            break;
        case 0x7:
            ppcVar1 = (*param_1 + 0x50);
            (**ppcVar1)(0x8, param_1, param_4);
            break;
        case 0x8:
            ppcVar1 = (*param_1 + 0x74);
            (**ppcVar1)(0x8, param_1, param_4);
            break;
        case 0xd:
            ppcVar1 = (*param_1 + 0x84);
            iVar4   = (**ppcVar1)(0x8, uVar7, uVar8, param_2, CONCAT12(param_4._0_1_, param_3));
            goto LAB_1008_9ada;
        case 0xf:
            ppcVar1 = (*param_1 + 0x34);
            (**ppcVar1)(0x1008, param_1);
            break;
        case 0x10:
            ppcVar1 = (*param_1 + 0x38);
            uVar6   = (**ppcVar1)(0x1008, param_1);
            return uVar6;
        case 0x19:
            ppcVar1 = (*param_1 + 0x78);
            uVar3   = (**ppcVar1)(0x8, uVar7, uVar8, param_2, CONCAT12(param_4._0_1_, param_3));
            return CONCAT22(0x1050, uVar3);
        case 0x1c:
            ppcVar1 = (*param_1 + 0x30);
            (**ppcVar1)(0x8, param_1, param_4);
        }
    }
    else
    {
        cVar9 = param_5;
        if(param_5 == 0x112)
        {
            if((PTR_LOOP_1050_039a == 0x0) && (ppcVar1 = (*param_1 + 0x48), iVar4 = (**ppcVar1)(), iVar4 != 0x0))
            {
                make_def_wnd_proc_1008_9ce6(msg, wparam, (u1616)param_2, param_3, CONCAT13(0x1, CONCAT12(cVar9, param_4)), param_6);
            }
        }
        else
        {
            if(param_5 < 0x113)
            {
                if(param_5 == 0x86)
                {
                    ppcVar1 = (*param_1 + 0x80);
                    uVar6   = (**ppcVar1)();
                    return uVar6;
                }
                if(param_5 < 0x87)
                {
                    if(param_5 == 0x85)
                    {
                        ppcVar1 = (*param_1 + 0x7c);
                        uVar6   = (**ppcVar1)();
                        return uVar6;
                    }
                    if(param_5 < 0x86)
                    {
                        if(cVar9 == '7')
                        {
                            return *(msg + 0xc2);
                        }
                        if(cVar9 == 'A')
                        {
                            ppcVar1 = (*param_1 + 0x2c);
                            (**ppcVar1)(param_6, param_1);
                            goto switchD_1008_9b30_caseD_1;
                        }
                    }
                switchD_1008_9b30_caseD_4:
                    if((param_5 < 0x400) || (0x7ffe < param_5))
                    {
                        uVar6 = make_def_wnd_proc_1008_9ce6(msg, wparam, (u1616)param_2, param_3, CONCAT22(param_5, param_4), param_6);
                        return uVar6;
                    }
                    ppcVar1 = (*param_1 + 0x28);
                    (**ppcVar1)(param_6, uVar7, uVar8, param_2, param_3, CONCAT22(param_5, param_4));
                }
                else
                {
                    if(param_5 == 0x100)
                    {
                        if(PTR_LOOP_1050_039a == 0x0)
                        {
                            ppcVar1 = (*param_1 + 0x6c);
                            (**ppcVar1)();
                        }
                    }
                    else
                    {
                        if(param_5 == 0x102)
                        {
                            if(PTR_LOOP_1050_039a == 0x0)
                            {
                                ppcVar1 = (*param_1 + 0x68);
                                (**ppcVar1)();
                            }
                        }
                        else
                        {
                            if(param_5 != 0x111)
                                goto switchD_1008_9b30_caseD_4;
                            if((param_4 != PTR_LOOP_1050_039c) && (PTR_LOOP_1050_039a == 0x0))
                            {
                                if(param_2 == 0x0)
                                {
                                    ppcVar1 = (*param_1 + 0x40);
                                    (**ppcVar1)();
                                }
                                else
                                {
                                    ppcVar1 = (*param_1 + 0x44);
                                    (**ppcVar1)();
                                }
                            }
                        }
                    }
                }
            }
            else
            {
                if(param_5 == 0x204)
                {
                    if(PTR_LOOP_1050_039a == 0x0)
                    {
                        ppcVar1 = (*param_1 + 0x60);
                        (**ppcVar1)();
                    }
                }
                else
                {
                    if(param_5 < 0x205)
                    {
                        if(param_5 == 0x113)
                        {
                            if(_PTR_LOOP_1050_0388 != 0x0)
                            {
                                pass1_1008_932a(_PTR_LOOP_1050_0388, unaff_SS);
                            }
                        }
                        else
                        {
                            if(param_5 == 0x117)
                            {
                                if(param_3 == 0x0)
                                {
                                    ppcVar1 = (*param_1 + 0x4c);
                                    (**ppcVar1)();
                                }
                                else
                                {
                                    ppcVar1 = (*param_1 + 0x20);
                                    (**ppcVar1)();
                                }
                            }
                            else
                            {
                                if(param_5 != 0x201)
                                    goto switchD_1008_9b30_caseD_4;
                                if(PTR_LOOP_1050_039a == 0x0)
                                {
                                    ppcVar1 = (*param_1 + 0x5c);
                                    (**ppcVar1)();
                                }
                            }
                        }
                    }
                    else
                    {
                        if(param_5 == 0x210)
                        {
                            ppcVar1 = (*param_1 + 0x64);
                            (**ppcVar1)();
                        }
                        else
                        {
                            if(param_5 == 0x30f)
                            {
                            LAB_1008_9af8:
                                ppcVar1 = (*param_1 + 0x8c);
                                iVar4   = (**ppcVar1)(param_6, param_1);
                            LAB_1008_9ada:
                                return (long)iVar4;
                            }
                            if(param_5 == 0x311)
                            {
                                ppcVar1 = (*param_1 + 0x88);
                                iVar4   = (**ppcVar1)();
                                if(iVar4 != 0x0)
                                    goto LAB_1008_9af8;
                            }
                            else
                            {
                                if(param_5 != 0x3b9)
                                    goto switchD_1008_9b30_caseD_4;
                                ppcVar1 = (*param_1 + 0x24);
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
    return uVar5;
}

LRESULT __stdcall16far make_def_wnd_proc_1008_9ce6(u1616 param_1, u1616 param_2, u1616 in_msg_3, WPARAM16 param_4, LPARAM param_5, HWND16 in_hwnd_5)

{
    LRESULT LVar1;

    LVar1 = DefWindowProc16(in_hwnd_5, in_msg_3, param_4, param_5);
    return LVar1;
}


void pass1_1008_9e5a(astruct_11 *param_1)

{
    undefined4  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    u16         *puVar4;
    u16          uVar6;
    astruct_464 *uVar5;
    u16          uVar7;
    u16         *puStack8;
    i16          iStack4;

    uVar7             = (param_1 >> 0x10);
    uVar5             = (astruct_464 *)param_1;
    param_1           = 0x9fb2;
    uVar5->field_0x2  = 0x1008;
    uVar5->field_0x1c = 0x9fca;
    uVar5->field_0x1e = 0x1008;
    if(_PTR_LOOP_1050_0388 != 0x0)
    {
        if(param_1 == (astruct_11 *)0x0)
        {
            puVar4 = (u16 *)0x0;
            uVar6  = 0x0;
        }
        else
        {
            puVar4 = &uVar5->field_0x1c;
            uVar6  = uVar7;
        }
        pass1_1008_92b2(_PTR_LOOP_1050_0388, 0x50, CONCAT22(uVar6, puVar4));
    }
    iStack4 = 0x0;
    do
    {
        puVar1 = *(&uVar5[0x1].field_0x0 + iStack4 * 0x4);
        uVar2  = (&uVar5[0x1].field_0x2)[iStack4 * 0x2];
        if((uVar2 | puVar1) != 0x0)
        {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        iStack4 = iStack4 + 0x1;
    } while(iStack4 < 0xc);
    if(param_1 == (astruct_11 *)0x0)
    {
        puVar4 = (u16 *)0x0;
        uVar7  = 0x0;
    }
    else
    {
        puVar4 = &uVar5->field_0x1c;
    }
    puStack8    = (u16 *)CONCAT22(uVar7, puVar4);
    *puStack8   = 0x389a;
    puVar4[0x1] = 0x1008;
    clenaup_win_ui_1018_4d22(param_1, 0x1018);
    return;
}


void __stdcall16far post_win_msg_1008_a0e4(astruct_67 *param_1, long param_2, i16 param_3, u16 param_4, u32 param_5, i16 param_6, HWND16 param_7, u16 param_8)

{
    undefined4 *puVar1;
    code      **ppcVar2;
    u16         uVar3;
    bool        bVar4;
    astruct_68 *puVar4;
    astruct_66 *uVar5;
    u16         extraout_DX;
    u16         uVar7;
    astruct_67 *iVar7;
    astruct_67 *uVar6;
    astruct_99 *paStack14;
    u8          local_a[0x8];

    uVar6 = (astruct_67 *)(param_1 >> 0x10);
    iVar7 = (astruct_67 *)param_1;
    pass1_1008_5784(CONCAT22(param_8, local_a), iVar7->field_0xa);
    bVar4 = false;
    do
    {
        puVar4 = (astruct_68 *)local_a;
        pass1_1008_5b12(puVar4, param_8);
        if((extraout_DX | puVar4) == 0x0)
            goto LAB_1008_a146;
    } while(puVar4->field_0x4 != param_6);
    puVar4->field_0xc = puVar4->field_0xc + param_3;
    puVar4->field_0xe = puVar4->field_0xe + param_2;
    bVar4             = true;
LAB_1008_a146:
    if(!bVar4)
    {
        param_7   = 0x1000;
        paStack14 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_03a0);
        uVar7     = (paStack14 >> 0x10);
        uVar3     = paStack14;
        if((uVar7 | uVar3) == 0x0)
        {
            paStack14 = (astruct_99 *)0x0;
        }
        else
        {
            paStack14->field_0x0 = 0x389a;
            (uVar3 + 0x2)        = 0x1008;
            (uVar3 + 0x4)        = param_6;
            *(uVar3 + 0x6)       = param_5;
            (uVar3 + 0xa)        = param_4;
            (uVar3 + 0xc)        = param_3;
            (uVar3 + 0xe)        = param_2;
            paStack14->field_0x0 = 0xad8e;
            (uVar3 + 0x2)        = 0x1008;
        }
        puVar1  = iVar7->field_0xa;
        ppcVar2 = (*iVar7->field_0xa + 0x8);
        (**ppcVar2)(0x1000, puVar1, (puVar1 >> 0x10), paStack14, (paStack14 >> 0x10));
    }
    if(param_6 == 0x14)
    {
        PostMessage16(param_7, 0x0, 0x0, 0x11100fc);
    }
    return;
}

u16 *pass1_1008_91ba(u16 *param_1, HWND16 param_2)

{
    u1616 UVar1;
    i16   iVar2;
    u16   uVar3;

    uVar3         = (param_1 >> 0x10);
    iVar2         = param_1;
    *param_1      = 0x389a;
    (iVar2 + 0x2) = 0x1008;
    (iVar2 + 0x4) = 0x0;
    set_struct_1008_574a((param_1 & 0xffff0000 | (iVar2 + 0x6)));
    *param_1            = 0x9416;
    (iVar2 + 0x2)       = 0x1008;
    _PTR_LOOP_1050_0388 = param_1;
    UVar1               = SetTimer16(param_2, 0x0, 0x0, (&PTR_LOOP_1050_0000 + 0x1));
    if(UVar1 == 0x0)
    {
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    PTR_LOOP_1050_038a = (_PTR_LOOP_1050_0388 >> 0x10);
    return param_1;
}


void kill_timer_1008_921c(u16 *param_1, HWND16 param_2)

{
    i16 iVar1;
    u16 uVar2;

    uVar2         = (param_1 >> 0x10);
    iVar1         = param_1;
    *param_1      = 0x9416;
    (iVar1 + 0x2) = 0x1008;
    KillTimer16(param_2, 0x1);
    _PTR_LOOP_1050_0388 = 0x0;
    pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0x6)));
    *param_1      = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    return;
}


void send_msg_1008_84ba(u16 param_1, u32 param_2, HWND16 param_3)

{
    i16 iVar1;
    u16 uVar2;
    u16 uStack4;

    uVar2 = (param_2 >> 0x10);
    iVar1 = param_2;
    if((*(byte *)(iVar1 + 0x4) & 0x4) == 0x0)
    {
        if((*(byte *)(iVar1 + 0x4) & 0x8) == 0x0)
        {
            return;
        }
        uStack4 = 0x1;
    }
    else
    {
        uStack4 = 0x0;
    }
    SendMessage16(param_3, *(iVar1 + 0x2), 0x0, CONCAT22(0x115, uStack4));
    return;
}


void win_1008_5c9e(u32 param_1, u32 *param_2, u16 param_3, u16 param_4, WNDCLASS16 *param_5)

{
    win_1008_5c7c(param_1, *param_2, param_5, param_3, param_4);
    return;
}


HWND16 create_window_1008_5e7e(u1616 in_stock_obj_id, WNDCLASS16 *in_wnd_class)

{
    undefined4 *puVar1;
    undefined4 *puVar2;
    BOOL16      BVar3;
    ATOM        AVar4;
    HWND16      window_handle_1;
    i16         iVar5;
    LPCSTR      string_1;
    undefined4 *puVar6;
    u16         name;
    u16         uStack42;
    u16         uStack40;
    u16         uStack38;
    u16         uStack36;
    u8         *puStack34;
    u16         uStack32;
    u16         uStack30;
    HGDIOBJ16   HStack28;
    undefined4  uStack26;
    undefined4 *puStack22;
    undefined4  local_12[0x4];

    puVar6   = local_12;
    string_1 = (LPCSTR)s_MciSoundWindow_1050_02bd;
    for(iVar5 = 0x3; iVar5 != 0x0; iVar5 = iVar5 + -0x1)
    {
        puVar2   = puVar6;
        puVar6   = puVar6 + 0x1;
        puVar1   = string_1;
        string_1 = (LPCSTR)(string_1 + 0x4);
        *puVar2  = *puVar1;
    }
    puVar6         = string_1;
    (puVar6 + 0x2) = (string_1 + 0x2);
    name           = 0x2000;
    uStack42       = SUB42(&DAT_1050_5f44, 0x0);
    uStack40       = 0x1008;
    uStack36       = 0x2;
    puStack34      = PTR_LOOP_1050_038c;
    uStack32       = 0x0;
    uStack30       = 0x0;
    uStack38       = 0x0;
    HStack28       = GetStockObject16(in_stock_obj_id);
    uStack26       = 0x0;
    puStack22      = local_12;
    BVar3          = GetClassInfo16((HINSTANCE16)s_tile2_bmp_1050_1538, (SEGPTR)&name, in_wnd_class);
    if(BVar3 == 0x0)
    {
        AVar4 = RegisterClass16(s_tile2_bmp_1050_1538);
        if(AVar4 == 0x0)
        {
            OutputDebugString16((LPCSTR)s_tile2_bmp_1050_1538);
            return 0x0;
        }
    }
    window_handle_1 = CreateWindow16((LPCSTR)s_tile2_bmp_1050_1538, (LPCSTR)0x0, ZEXT24(PTR_LOOP_1050_038c) << 0x10, 0x0, (i1616)PTR_LOOP_1050_0396, 0x1, 0x1, 0x8000, 0x8000, 0x0, 0xcf);
    return window_handle_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT make_def_win_proc_1008_5f44(u1616 param_1, WPARAM16 in_wparam_2, LPARAM param_3, HWND16 in_hwnd_4)

{
    WORD        WVar1;
    u8         *in_DX;
    i16         unaff_DI;
    WNDCLASS16 *unaff_SS;
    LRESULT     LVar2;
    u16        *puVar3;

    if(param_3._2_2_ == 0x2)
    {
        WVar1 = GetWindowWord16(in_hwnd_4, 0x0);
        mci_send_command_1008_5cb6(_PTR_LOOP_1050_02a0, WVar1, s_tile2_bmp_1050_1538);
        puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, unaff_SS, in_DX, unaff_DI);
        pass1_1008_aa28(puVar3, puVar3, unaff_SS);
    }
    else
    {
        if(param_3._2_2_ != 0x3b9)
        {
            LVar2 = DefWindowProc16(in_hwnd_4, param_1, in_wparam_2, param_3);
            return LVar2;
        }
        DestroyWindow16(in_hwnd_4);
    }
    return 0x0;
}


void destroy_win_1008_628e(u32 param_1, HWND16 param_2)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = ((param_1 + 0xd2) + 0x14);
    (**ppcVar1)(param_2, (param_1 + 0xd2), param_1._2_2_);
    DestroyWindow16(param_2);
    (param_1 + 0x8) = 0x0;
    return;
}
