
void  file_1008_6414(u32 *param_1, param_2: u32, param_3: u16, mut param_4: *mut u8)

{
    code      **ppcVar1;
     let mut puVar2: *mut u8;
     let mut uVar3: u16;
     let mut extraout_DX: u16;
    i16         iVar4;
     let mut uVar5: u16;
    astruct_76 *paStack42;
    u8          local_26[0x24];

    uVar5         = (param_1 >> 0x10);
    iVar4         = param_1;
    *param_1      = 0x0;
    (iVar4 + 0x4) = 0x0;
    puVar2        = local_26;
    struct_op_1008_48fe((astruct_81 *)CONCAT22(param_3, puVar2), 0x1, param_2, param_4);
    mem_op_1000_179c(0x1e, param_4, 0x1000);
    paStack42 = (astruct_76 *)CONCAT22(param_4, puVar2);
    uVar3     = param_4 | puVar2;
    if(uVar3 == 0x0)
    {
        *param_1 = 0x0;
    }
    else
    {
        puVar2 = local_26;
        struct_op_1008_3f92(paStack42, (astruct_83 *)CONCAT22(param_3, puVar2));
        param_1       = puVar2;
        (iVar4 + 0x2) = uVar3;
    }
    ppcVar1 = (*param_1 + 0x14);
    (**ppcVar1)();
    (iVar4 + 0x4) = puVar2;
    (iVar4 + 0x6) = extraout_DX;
    close_file_1008_496c(local_26, param_3);
    return;
}


void  close_file_1008_496c(mut param_1: *mut u16)

{
    u32 *puVar1;
     let mut uVar2: u16;
    u32  uVar3;
    code      **ppcVar4;
    i16         iVar5;
     let mut uVar6: u16;

    uVar6         = (param_1 >> 0x10);
    iVar5         = param_1;
    *param_1      = &PTR_LOOP_1050_4c4c;
    (iVar5 + 0x2) = 0x1008;
    puVar1        = (iVar5 + 0x4);
    uVar2         = (iVar5 + 0x6);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    fn_ptr_1000_17ce((iVar5 + 0x8), 0x1000);
    if((iVar5 + 0x1a) != 0x0)
    {
        uVar3 = (iVar5 + 0x1a);
        call_fn_ptr_1000_0dc6(uVar3, (uVar3 >> 0x10), 0x1000);
    }
    if((iVar5 + 0xc) != -0x1)
    {
        _lclose16(0x1000);
    }
    *param_1      = 0x389a;
    (iVar5 + 0x2) = 0x1008;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16  read_file_1008_49e8(param_1: u32, param_2: u16, param_3: u16)

{
    HFILE16    HVar1;
    i16        iVar2;
    u32        uVar3;
    u32        uVar4;
     let mut puVar5: *mut u8;
     let mut puVar6: *mut u8;
     let mut extraout_DX: *mut u8;
    i16        iVar7;
    i16        unaff_DI;
     let mut uVar8: u16;
     let mut h_file: u16;
     let mut unaff_SS: u16;
    long       lVar9;
    i16        local_18;
    u32 uStack22;
     let mut uStack10: u16;
     let mut puStack8: *mut u8;
    u32 uStack6;

    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    if((iVar7 + 0x8) != 0x0)
    {
        if((iVar7 + 0x1e) != 0x0)
        {
            return param_3;
        }
        h_file = param_2;
        if((iVar7 + 0xc) == -0x1)
        {
            h_file                    = s_tile2_bmp_1050_1538;
            HVar1                     = _lopen16(param_2, 0x0);
            *(HFILE16 *)(iVar7 + 0xc) = HVar1;
            if(HVar1 == 0xffff)
            {
                return param_3;
            }
        }
        uStack6 = 0x0;
        lVar9   = WIN16_hread(h_file, 0xe, ZEXT24(&local_18) << 0x10);
        param_3 = (lVar9 >> 0x10);
        if((lVar9 == 0xe) && (param_3 == 0x0))
        {
            uStack6 = uStack22;
            if(local_18 == &PTR_LOOP_1050_4d42)
            {
                _llseek16((HFILE16)s_tile2_bmp_1050_1538, 0x0, 0x0);
                lVar9          = mem_op_1000_0a48(0x1, uStack6, (uStack6 >> 0x10), globals._PTR_LOOP_1050_5f2c, 0x1000);
                puVar6         = (lVar9 >> 0x10);
                (iVar7 + 0x1a) = lVar9;
                (iVar7 + 0x1c) = puVar6;
                if((puVar6 | (iVar7 + 0x1a)) == 0x0)
                {
                    return puVar6;
                }
                lVar9    = WIN16_hread(0x1000, (SEGPTR)uStack6, CONCAT22((iVar7 + 0x1a), (uStack6 >> 0x10)));
                puVar5   = (lVar9 >> 0x10);
                uStack10 = lVar9;
                puStack8 = puVar5;
                _lclose16((HFILE16)s_tile2_bmp_1050_1538);
                (iVar7 + 0xc)   = 0xffff;
                (iVar7 + 0x1e)  = 0x1;
                (iVar7 + 0xe)   = (iVar7 + 0x1a);
                uVar3           = *(iVar7 + 0x1a);
                iVar2           = uVar3;
                uVar3           = uVar3 & 0xffff0000;
                *(iVar7 + 0x12) = uVar3 | iVar2 + 0xe;
                uVar3           = uVar3 | iVar2 + 0x436;
                *(iVar7 + 0x16) = uVar3;
                mem_op_1000_179c(0x14, puVar5, 0x1000);
                puVar6 = (puVar5 | uVar3);
                if(puVar6 == 0x0)
                {
                    (iVar7 + 0x4) = 0x0;
                }
                else
                {
                    uVar4 = *(iVar7 + 0x12);
                    uVar4 = uVar4 & 0xffff0000 | (uVar4 + 0x28);
                    struct_op_1008_4c98((astruct_76 *)(uVar3 & 0xffff | ZEXT24(puVar5) << 0x10), uVar4, 0x100);
                    (iVar7 + 0x4) = uVar4;
                    (iVar7 + 0x6) = extraout_DX;
                    puVar6        = extraout_DX;
                }
                if((iVar7 + 0x22) != 0x0)
                {
                    pass1_1008_4b8e(param_1, puVar6, unaff_DI, unaff_SS);
                    return puVar6;
                }
                return puVar6;
            }
        }
        _lclose16((HFILE16)s_tile2_bmp_1050_1538);
        (iVar7 + 0xc) = 0xffff;
    }
    return param_3;
}


u32  file_1008_4c26(param_1: u32, param_2: u8)

{
    close_file_1008_496c(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}

void  save_file_1008_3178(param_1: u32, param_2: i16, param_3: u16)

{
     let mut cVar1: u8;
    u32  uVar2;
    i16         iVar3;
     let mut puVar4: *mut u8;
     let mut uVar5: u16;
    BOOL16      BVar6;
     let mut in_DX: *mut u8;
     let mut extraout_DX: u16;
     let mut uVar7: u16;
    i16         unaff_DI;
     let mut uVar8: u16;
     let mut pcVar9: String;
     let mut in_buf_len_2: u16;
     let mut uVar10: u16;
    char        local_782[0x104];
    u8          local_67e[0x8];
    Struct18 *paStack1654;
    LPCSTR      pCStack1650;
     let mut UStack1648: u16;
    Struct18 *paStack1646;
    u8          local_666[0x100];
     let mut pcStack1382: String;
    u32  local_562;
     let mut uStack1374: u16;
     let mut pcStack1370: String;
     let mut uStack1326: u16;
    char        acStack1305[0x101];
     let mut uStack1048: u16;
    char        local_416[0x8];
     let mut uStack1038: u16;
    u8          local_40c[0x102];
    u32         uStack778;
    u16        *puStack774;
     let mut local_302: u8;
    u8          local_202[0xff];
    char        acStack259[0x101];

    acStack259[1] = 0x0;
    local_302     = 0x0;
    local_202[0]  = 0x0;
    puStack774    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_3, in_DX, unaff_DI);
    uVar8         = (puStack774 >> 0x10);
    iVar3         = puStack774;
    uStack778     = (iVar3 + 0x1a);
    uVar10        = (iVar3 + 0x1c);
    if((uVar10 | uStack778) == 0x0)
    {
        paStack1646 = (iVar3 + 0x64);
        uVar10      = (iVar3 + 0x66);
        if((uVar10 | paStack1646) != 0x0)
        {
            pass1_1008_5784(CONCAT22(param_3, local_67e), paStack1646 & 0xffff | uVar10 << 0x10);
            puVar4 = local_67e;
            pass1_1008_5b12(puVar4, param_3);
            paStack1654 = (Struct18 *)CONCAT22(extraout_DX, puVar4);
            if((extraout_DX | puVar4) != 0x0)
            {
                uVar2           = (puVar4 + 0x4);
                uStack778._0_2_ = uVar2;
                uVar10          = (uVar2 >> 0x10);
                goto LAB_1008_3206;
            }
        }
    }
    else
    {
    LAB_1008_3206:
        unk_str_op_1000_3d3e(CONCAT22(param_3, acStack259 + 0x1), CONCAT22(uVar10, uStack778));
    }
    pass1_1000_5008(local_40c, param_3, 0x100, &stack0xfffe);
    uStack1038 = str_op_1000_3da4(CONCAT22(param_3, local_40c));
    if(local_40c[uStack1038 - 0x1] == '\\')
    {
        local_40c[uStack1038 - 0x1] = 0x0;
    }
    uStack1038 = str_op_1000_3da4(CONCAT22(param_3, acStack259 + 0x1));
    if(acStack259[uStack1038] == '\\')
    {
        acStack259[uStack1038] = '\0';
    }
    pass1_1000_4f2e(&stack0xfffe);
    uVar8     = (puStack774 >> 0x10);
    uStack778 = *(puStack774 + 0x12);
    uVar10    = (puStack774 + 0x14);
    if((uVar10 | uStack778) != 0x0)
    {
        unk_str_op_1000_3d3e(CONCAT22(param_3, local_202), (uStack778 & 0xffff | uVar10 << 0x10));
    }
    local_416[0] = '\0';
    pcVar9       = load_string_1010_847e(_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    unk_str_op_1000_3d3e(CONCAT22(param_3, local_416), pcVar9);
    uStack1048 = str_op_1000_3da4(CONCAT22(param_3, local_416));
    uStack1038 = uStack1048;
    for(; - 0x1 < uStack1048; uStack1048 = uStack1048 - 0x1)
    {
        if(local_416[uStack1048] == '.')
        {
            unk_str_op_1000_3d3e(CONCAT22(param_3, local_67e), CONCAT22(param_3, local_416 + uStack1048 + 0x1));
            unk_str_op_1000_3d3e(CONCAT22(param_3, local_416), CONCAT22(param_3, local_67e));
        }
    }
    acStack1305[1] = 0x0;
    pcVar9         = load_string_1010_847e(_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    uVar5          = (pcVar9 >> 0x10);
    unk_str_op_1000_3d3e(CONCAT22(param_3, acStack1305 + 0x1), pcVar9);
    uStack1038 = str_op_1000_3da4(CONCAT22(param_3, acStack1305 + 0x1));
    cVar1      = acStack1305[uStack1038];
    uStack1048 = 0x0;
    while(acStack1305[uStack1048 + 0x1] != '\0')
    {
        if(acStack1305[uStack1048 + 0x1] == cVar1)
        {
            acStack1305[uStack1048 + 0x1] = '\0';
        }
        uStack1048 = uStack1048 + 0x1;
    }
    pass1_1000_4906((Struct20 *)CONCAT22(param_3, &local_562), 0x0, 0x48);
    local_562    = 0x48;
    uStack1374   = (param_1 + 0x8);
    pcStack1370  = acStack1305 + 0x1;
    pcStack1382  = 0x0;
    local_666[0] = 0x0;
    in_buf_len_2 = (_PTR_LOOP_1050_14cc >> 0x10);
    if(param_2 == 0x1)
    {
        pcVar9 = load_string_1010_847e(_PTR_LOOP_1050_14cc, in_buf_len_2, 0x1010);
        uVar5  = (pcVar9 >> 0x10);
        unk_str_op_1000_3d3e(CONCAT22(param_3, local_666), pcVar9);
        BVar6 = GetOpenFileName16(0x1000);
    }
    else
    {
        if(param_2 != 0x2)
        {
            debug_pri16_1008_6048(s_Unsupported_FileStructType_in_Op_1050_01ca, 0x1000, param_3);
            goto LAB_1008_3461;
        }
        pcVar9 = load_string_1010_847e(_PTR_LOOP_1050_14cc, in_buf_len_2, 0x1010);
        uVar5  = (pcVar9 >> 0x10);
        unk_str_op_1000_3d3e(CONCAT22(param_3, local_666), pcVar9);
        BVar6 = GetSaveFileName16(0x1000);
    }
    if(BVar6 != 0x0)
    {
        pcStack1382 = CONCAT22(param_3, local_202);
    }
LAB_1008_3461:
    if(pcStack1382 != 0x0)
    {
        if(uStack1326 < 0x0)
        {
            paStack1654 = (Struct18 *)load_string_1010_847e(_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
            uVar7       = (paStack1654 >> 0x10);
            uVar5       = str_op_1008_60e8(paStack1654, uVar7);
            paStack1654 = (Struct18 *)CONCAT22(uVar7, uVar5);
            pcVar9      = load_string_1010_847e(_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
            UStack1648  = (pcVar9 >> 0x10);
            pCStack1650 = pcVar9;
            MessageBox16(0x1010, &PTR_LOOP_1050_0010, pCStack1650, UStack1648);
            pcStack1382 = 0x0;
            paStack1646 = paStack1654;
            fn_ptr_1000_17ce(paStack1654, 0x1000);
        }
        else
        {
            str_op_1000_3dbe(CONCAT13((param_3 >> 0x8), CONCAT12(param_3, local_782)), CONCAT22(param_3, local_202), uStack1326);
            local_782[uStack1326] = '\0';
            if(local_782[0] != '\0')
            {
                pass1_1010_60cc(puStack774, CONCAT22(param_3, local_782), uVar5);
            }
        }
    }
    pass1_1000_4f2e(&stack0xfffe);
    return;
}
