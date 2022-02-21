
ushort *__stdcall16far struct_1028_406c(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x42ec;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_4354(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x446a;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_44d2(ushort *param_1)

{
    undefined2 uVar1;

    struct_1028_b354(param_1);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined4 *)((int)param_1 + 0x20) = 0x0;
    *param_1                             = 0x4836;
    *(undefined2 *)((int)param_1 + 0x2)  = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_489e(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = (ushort)&PTR_LOOP_1050_4942;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_49aa(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x4b1c;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x20)), (WNDCLASS16 *)0x0, 0xa);
    return param_1;
}


ushort *__stdcall16far struct_1028_4b84(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 0x1;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


void __stdcall16far pass1_1028_4db2(ushort param_1, ushort param_2, int param_3, uchar *param_4, int param_5, ushort param_6, uchar param_7)

{
    BOOL16     BVar1;
    int       *piVar2;
    undefined2 extraout_DX;
    ushort    *puVar3;
    int       *piVar4;
    ushort     uVar5;
    ushort    *puVar6;
    ushort     uVar7;
    undefined  local_14e[0x124];
    ulong      uStack42;
    undefined4 uStack38;
    int        local_22;
    undefined  local_20[0x2];
    undefined  local_1e[0x2];
    ulong      local_1c;
    int        iStack24;
    undefined4 uStack22;
    int       *piStack18;
    undefined2 uStack16;
    int        local_e;
    ushort     local_c;
    ulong      uStack10;
    ushort    *puStack6;

    BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)(param_1 + 0xc), 0x29);
    if(BVar1 != 0x0)
    {
        pass1_1028_bd38(CONCAT22(param_2, param_1), (ushort)param_4, param_6);
        if((param_3 == 0x0) && (*(int *)(param_1 + 0xc) == 0x13))
        {
            puVar3  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x8, param_6, param_4, param_5);
            param_4 = (uchar *)((ulong)puVar3 >> 0x10);
            pass1_1010_988c((ulong)puVar3, *(int *)(param_1 + 0xc));
        }
        puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_6, param_4, param_5);
        uStack16 = (undefined2)((ulong)puStack6 >> 0x10);
        uStack10 = *(ulong *)((int)puStack6 + 0x20);
        puVar6   = &local_c;
        piVar2   = &local_e;
        piVar4   = piVar2;
        uVar5    = param_6;
        uVar7    = param_6;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack10, (uint)(uStack10 >> 0x10));
        piStack18 = piVar2;
        pass1_1030_5b1c(CONCAT22(uStack16, piVar2), (ushort *)CONCAT22(uVar5, piVar4), (ushort *)CONCAT22(uVar7, puVar6));
        pass1_1028_b58e(CONCAT22(param_2, param_1));
        uStack22 = CONCAT22(extraout_DX, piVar2);
        local_1c = *(ulong *)(piVar2 + 0x6);
        iStack24 = piVar2[0x8];
        pass1_1028_c8ee(param_6, param_1, param_2, 0x1, (ushort *)CONCAT22(param_6, &local_1c));
        pass1_1008_3eb4((ushort *)CONCAT22(param_6, &local_1c), (ushort *)CONCAT22(param_6, &local_22), (ushort *)CONCAT22(param_6, local_20), (ushort *)CONCAT22(param_6, local_1e));
        if(local_e < local_22)
        {
            pass1_1030_5b3e(CONCAT22(uStack16, piStack18), local_22, local_c);
            pass1_1030_5b1c(CONCAT22(uStack16, piStack18), (ushort *)CONCAT22(param_6, &local_e), (ushort *)CONCAT22(param_6, &local_c));
        }
        uStack38 = *(undefined4 *)((int)uStack22 + 0x2e);
        uStack42 = *(ulong *)((int)uStack38 + 0x4);
        struct_op_1028_87f0(param_6, param_7, (astruct_97 *)CONCAT22(param_6, local_14e), 0x0, 0x0, 0x62, &local_1c, param_6, uStack42, uStack10);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_6, local_14e));
        pass1_1028_ccd0(param_7, param_6, CONCAT22(param_2, param_1), (ushort *)CONCAT22(param_6, &local_1c));
    }
    return;
}


ulong __stdcall16far pass1_1028_4faa(ulong param_1, ushort param_2)

{
    ushort      uVar1;
    undefined4 *puVar2;
    uint        uVar3;
    ulong       uVar4;
    undefined4  local_c;
    undefined2  uStack8;
    undefined4  uStack6;

    uVar1 = pass1_1028_4f62(param_1);
    if(uVar1 != 0x0)
    {
        return param_1;
    }
    uStack6 = pass1_1028_b58e(param_1);
    local_c = *(undefined4 *)((int)uStack6 + 0xc);
    uStack8 = 0x0;
    uVar4   = pass1_1028_bb24(param_1);
    uVar3   = (uint)(uVar4 >> 0x10);
    puVar2  = &local_c;
    pass1_1030_627e(param_2, (uint)puVar2, uVar3, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_2, puVar2), uVar4 & 0xffff | (ulong)uVar3 << 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)puVar2, uVar3);
    if((uVar3 | (uint)puVar2) == 0x0)
    {
        return 0x0;
    }
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3, puVar2));
    return uVar4;
}


ushort *__stdcall16far struct_1028_3484(ushort *param_1)

{
    uchar *in_DX;

    struct_1028_0068(param_1, in_DX);
    *param_1                            = 0x34f6;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_355e(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x3608;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_3670(ushort *param_1, uchar *param_2, ushort param_3, ushort param_4)

{
    struct_1028_37a6(param_1, param_2, param_3, param_4);
    *param_1                            = 0x373e;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_3e94(ushort *param_1)

{
    uint uVar1;

    struct_1028_b354(param_1);
    uVar1                                = (uint)((ulong)param_1 >> 0x10);
    *(undefined4 *)((int)param_1 + 0x20) = 0x0;
    *param_1                             = 0x4004;
    *(undefined2 *)((int)param_1 + 0x2)  = (int)&USHORT_1050_1028;
    pass1_1028_3fa2((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    return param_1;
}


ushort *__stdcall16far struct_1028_1bbc(ushort *param_1)

{
    astruct_190 *iVar1;
    undefined2   uVar1;

    struct_1028_b354(param_1);
    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_190 *)param_1;
    iVar1->field_0x20 = 0x0;
    iVar1->field_0x22 = 0x0;
    *param_1          = 0x1eee;
    iVar1->field_0x2  = (int)&USHORT_1050_1028;
    return param_1;
}


ushort __stdcall16far pass1_1028_1e14(ushort param_1, ushort param_2, int param_3, ushort *param_4, long param_5, uint param_6, uint param_7, ushort param_8)

{
    int   iVar1;
    uint  uVar2;
    ulong uVar3;

    pass1_1030_627e(param_8, param_6, param_7, _PTR_LOOP_1050_5740, param_4, param_5);
    uVar2 = param_7 | param_6;
    if(uVar2 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_6, param_7);
        if((uVar2 | param_6) != 0x0)
        {
            uVar3 = struct_op_1030_73a8(CONCAT22(uVar2, param_6));
            if(uVar3 != 0x0)
            {
                iVar1 = *(int *)((int)uVar3 + 0xc);
                if(((iVar1 == 0x18) || (iVar1 == 0x3f)) || (iVar1 == param_3))
                {
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}


ushort __stdcall16far pass1_1028_21ba(ushort param_1, ushort param_2, ushort *param_3, long param_4, uint param_5, uint param_6, ushort param_7)

{
    uint  uVar1;
    ulong uVar2;

    pass1_1030_627e(param_7, param_5, param_6, _PTR_LOOP_1050_5740, param_3, param_4);
    uVar1 = param_6 | param_5;
    if(uVar1 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, param_6);
        if((uVar1 | param_5) != 0x0)
        {
            uVar2 = struct_op_1030_73a8(CONCAT22(uVar1, param_5));
            if((uVar2 != 0x0) && (*(int *)((int)uVar2 + 0xc) == 0x40))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined2 __stdcall16far pass1_1028_2220(undefined2 param_1, undefined2 param_2, int param_3, ushort *param_4, long param_5, uint param_6, uint param_7, ushort param_8)

{
    int   iVar1;
    uint  uVar2;
    ulong uVar3;

    pass1_1030_627e(param_8, param_6, param_7, _PTR_LOOP_1050_5740, param_4, param_5);
    uVar2 = param_7 | param_6;
    if(uVar2 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_6, param_7);
        if((uVar2 | param_6) != 0x0)
        {
            uVar3 = struct_op_1030_73a8(CONCAT22(uVar2, param_6));
            if((uVar3 != 0x0) && ((iVar1 = *(int *)((int)uVar3 + 0xc), iVar1 == 0x40 || (iVar1 == param_3))))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}


ushort *__stdcall16far struct_1028_25da(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = (int)s_fem16_wav_1050_2644 + 0x8;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_26b4(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x2788;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_27f0(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x2a92;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_2afa(ushort *param_1)

{
    struct_1030_dc96(param_1);
    *param_1                            = 0x2b74;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_2bdc(ushort *param_1)

{
    struct_1028_0954(param_1);
    *param_1                            = 0x341c;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_0b42(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0xbbc;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_0c24(ushort *param_1)

{
    astruct_191 *iVar1;
    undefined2   uVar1;

    struct_1028_b354(param_1);
    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_191 *)param_1;
    iVar1->field_0x20 = 0x0;
    iVar1->field_0x22 = 0x0;
    *param_1          = (int)s_480_bmp_1050_1721 + 0x3;
    iVar1->field_0x2  = (int)&USHORT_1050_1028;
    return param_1;
}


ulong __stdcall16far pass1_1028_121e(ulong param_1, ushort param_2)

{
    bool        bVar1;
    undefined   extraout_AH;
    undefined4 *puVar2;
    uint        uVar3;
    ulong       uVar4;
    undefined4  local_c;
    undefined2  uStack8;
    undefined4  uStack6;

    bVar1 = pass1_1028_11de(param_1);
    if(CONCAT11(extraout_AH, bVar1) != 0x0)
    {
        return param_1;
    }
    uStack6 = pass1_1028_b58e(param_1);
    local_c = *(undefined4 *)((int)uStack6 + 0xc);
    uStack8 = 0x0;
    uVar4   = pass1_1028_bb24(param_1);
    uVar3   = (uint)(uVar4 >> 0x10);
    puVar2  = &local_c;
    pass1_1030_627e(param_2, (uint)puVar2, uVar3, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_2, puVar2), uVar4 & 0xffff | (ulong)uVar3 << 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)puVar2, uVar3);
    if((uVar3 | (uint)puVar2) == 0x0)
    {
        return 0x0;
    }
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3, puVar2));
    return uVar4;
}


ushort *__stdcall16far struct_1028_178c(ushort *param_1)

{
    struct_1030_dc96(param_1);
    *param_1                            = 0x1b54;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1020_e8f6(ushort *param_1)

{
    undefined2 uVar1;

    struct_1030_dc96(param_1);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined2 *)((int)param_1 + 0x24) = 0x0;
    *param_1                             = 0xeef6;
    *(undefined2 *)((int)param_1 + 0x2)  = 0x1020;
    return param_1;
}

ushort *__stdcall16far struct_1028_0954(ushort *param_1)

{
    astruct_185 *iVar1;
    undefined2   uVar1;

    struct_1028_b354(param_1);
    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_185 *)param_1;
    iVar1->field_0x20 = 0x0;
    *param_1          = 0xada;
    iVar1->field_0x2  = (int)&USHORT_1050_1028;
    iVar1->field_0xe  = 0x4b;
    return param_1;
}

ushort *__stdcall16far struct_1020_d866(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0xd8ec;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    return param_1;
}

ushort *__stdcall16far struct_1020_e7fa(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0xe88e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    return param_1;
}

ushort *__stdcall16far struct_1020_c9ea(ushort *param_1)

{
    struct_1028_0954(param_1);
    *param_1                            = 0xcc7c;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    return param_1;
}

ushort *__stdcall16far struct_1020_cce4(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0xcd7e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    return param_1;
}

ushort *__stdcall16far struct_1020_cde6(ushort *param_1)

{
    struct_1028_0954(param_1);
    *param_1                            = 0xd004;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    return param_1;
}

ushort *__stdcall16far struct_1020_d06c(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0xd314;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    return param_1;
}

ushort *__stdcall16far struct_1020_d37c(ushort *param_1)

{
    undefined2 uVar1;

    struct_1028_b354(param_1);
    uVar1                                = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined2 *)((int)param_1 + 0x20) = 0x0;
    *param_1                             = 0xd53e;
    *(undefined2 *)((int)param_1 + 0x2)  = 0x1020;
    return param_1;
}

ushort *__stdcall16far struct_1020_d5a6(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0xd7fe;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    return param_1;
}

void __stdcall16far struct_1020_c444(astruct_75 *param_1, ulong param_2, ulong param_3)

{
    astruct_75 *iVar1;
    astruct_75 *uVar1;

    struct_op_1030_1cd8(param_1, param_2, param_3);
    uVar1                                = (astruct_75 *)((ulong)param_1 >> 0x10);
    iVar1                                = (astruct_75 *)param_1;
    *(undefined4 *)(iVar1 + 0x1)         = 0x0;
    *(undefined2 *)&iVar1[0x1].field_0x4 = 0x0;
    param_1->field_0x0                   = 0xc834;
    iVar1->field_0x2                     = 0x1020;
    return;
}

void __stdcall16far pass1_1020_afc4(ushort param_1, ushort param_2, ushort param_3, ushort param_4, ushort *param_5, long param_6)

{
    undefined4 *puVar1;
    uint        uVar2;
    uint        uVar3;
    ulong       uVar4;
    byte        bStack27;
    undefined4  local_a;
    undefined4  uStack6;

    puVar1 = &local_a;
    pass1_1030_64ce(param_1, puVar1, param_2, _PTR_LOOP_1050_5740, param_5, param_6, (ulong *)CONCAT22(param_1, puVar1));
    uStack6  = *puVar1;
    uVar3    = *(uint *)((int)puVar1 + 0x2);
    bStack27 = (byte)((ulong)uStack6 >> 0x18);
    uVar2    = (uint)bStack27;
    if(bStack27 == 0x0)
    {
        return;
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack6, uVar3);
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3, uVar2));
    uVar3 = (uint)(uVar4 >> 0x10);
    if((uVar3 | (uint)uVar4) != 0x0)
    {
        switch(*(undefined2 *)((uint)uVar4 + 0xc))
        {
        case 0x1:
            break;
        case 0x2:
            break;
        case 0x3:
            break;
        case 0x4:
            break;
        case 0x5:
            break;
        case 0x6:
            break;
        case 0x7:
            return;
        case 0x8:
            return;
        case 0x9:
            return;
        }
        return;
    }
    return;
}


void __stdcall16far pass1_1020_289a(ushort *param_1, UINT16 param_2, ulong param_3, UINT16 param_4)

{
    int        iVar1;
    undefined2 uVar2;

    struct_1020_790e(param_1, (ulong)s_SCPOPMENU_1050_427c, param_2, param_3, param_4);
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(undefined4 *)(iVar1 + 0xf2) = 0x0;
    *(undefined4 *)(iVar1 + 0xf6) = 0x0;
    *(undefined2 *)(iVar1 + 0xfa) = 0x0;
    *(undefined2 *)(iVar1 + 0xfc) = 0x0;
    *param_1                      = 0x2e4a;
    *(undefined2 *)(iVar1 + 0x2)  = 0x1020;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x5b)), s_VrMode_1050_4286);
    *(undefined4 *)(iVar1 + 0xac) = 0x44c00000;
    return;
}

void __stdcall16far struct_1020_0baa(ushort *param_1, ushort param_2, uchar *param_3, ushort param_4)

{
    uchar       *puVar1;
    astruct_276 *iVar2;
    int          unaff_DI;
    undefined2   uVar2;
    ushort      *puVar3;
    undefined2  *puVar4;
    undefined2  *puVar5;
    undefined2   uVar6;

    uVar2                            = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                            = (astruct_276 *)param_1;
    *param_1                         = 0x389a;
    iVar2->field_0x2                 = 0x1008;
    *param_1                         = 0x3aa8;
    iVar2->field_0x2                 = 0x1008;
    iVar2->field_0x4                 = param_2;
    *param_1                         = 0x3ab0;
    iVar2->field_0x2                 = 0x1008;
    *(undefined4 *)&iVar2->field_0x6 = 0x0;
    iVar2->field_0xa                 = 0x0;
    iVar2->field_0xc                 = 0x0;
    *param_1                         = 0xdbc;
    iVar2->field_0x2                 = 0x1020;
    puVar3                           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x7, param_4, param_3, unaff_DI);
    puVar1                           = (uchar *)((ulong)puVar3 >> 0x10);
    iVar2->field_0x6                 = (int)puVar3;
    iVar2->field_0x8                 = puVar1;
    puVar5                           = &iVar2->field_0xa;
    puVar4                           = &iVar2->field_0xc;
    uVar6                            = uVar2;
    puVar3                           = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, puVar1, unaff_DI);
    pass1_1008_3e94((ushort *)((ulong)puVar3 & 0xffff0000 | (ulong)((int)puVar3 + 0xe)), (ushort *)CONCAT22(uVar2, puVar4), (ushort *)CONCAT22(uVar6, puVar5));
    return;
}

astruct_20 *__stdcall16far struct_1018_6d02(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0xb, 0x9c, 0x8b, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa27e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6d38(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0xc, 0x9d, 0xd0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb562;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6d6e(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0xd, 0x9e, 0xd1, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x9822;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6da4(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0xe, 0x9f, 0xd2, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xab06;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6dda(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0xf, 0xa0, 0xd4, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xbdea;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6e10(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x10, 0xa1, 0xda, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa0aa;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6e46(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x11, 0xa2, 0xdc, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb38e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6e7c(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x12, 0xa3, 0xd3, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x964e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6eb2(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x13, 0xa4, 0xdb, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa932;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6ee8(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x14, 0xa5, 0xa5, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xbc16;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6f1e(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x15, 0xa7, 0xb2, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x9e3a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6f54(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x16, 0xa8, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb11e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6f8a(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x17, 0xaf, 0xc0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x93de;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6fc0(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x18, 0xb0, 0xc1, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa6c2;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_6ff6(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x19, 0xb1, 0x80, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb9a6;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_702c(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1ec, 0x1a, 0xb2, 0xc3, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x9c66;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7062(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x1b, 0xb3, 0xc4, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xaf4a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7098(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x1c, 0xb4, 0xd8, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xc22e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_70ce(astruct_20 *param_1, UINT16 param_2, ULONG param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x1d, 0xb5, 0x7b, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa4ee;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7104(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x1e, 0xb6, 0xd9, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb7d2;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_713a(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x1f, 0xb7, 0x7d, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x9a92;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7170(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x21, 0xb9, 0xdd, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xad76;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_71a6(astruct_20 *param_1, UINT16 param_2, ULONG param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x23, 0xd3, 0xd6, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb69a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_71dc(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1ed, 0x24, 0xd4, 0xd7, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x995a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7212(astruct_20 *param_1, UINT16 param_2, ULONG param_3, UINT16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x25, 0xe9, 0xee, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa452;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7248(astruct_20 *param_1, UINT16 param_2, ULONG param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x63, 0xa6, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xc05a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_727e(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x64, 0xa9, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa31a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_72b4(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x65, 0xaa, 0xbb, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb5fe;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_72ea(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x66, 0xab, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x98be;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7320(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x67, 0xac, 0xbd, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xaba2;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7356(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x68, 0xad, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xbe86;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_738c(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x69, 0xae, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xac3e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_73c2(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x35, 0xba, 0x81, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xbf22;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_73f8(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x39, 0xbb, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa146;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_745e(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x22, 0xbc, 0xd5, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb42a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7494(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x36, 0xbd, 0xcd, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x96ea;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_74ca(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x37, 0xbe, 0x83, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa9ce;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7500(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x38, 0xbf, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xbcb2;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7536(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x3a, 0xc0, 0x85, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x9f72;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_756c(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1e2, 0x3b, 0xc1, 0x86, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb256;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_75a2(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x3c, 0xc2, 0x87, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x9516;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far pass1_1018_75d8(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x3d, 0xc3, 0x88, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa7fa;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_760e(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x3e, 0xc4, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xbade;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7644(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x3f, 0xc5, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x9d02;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_767a(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x40, 0xc6, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xafe6;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_76b0(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x41, 0xc7, 0x8d, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xc2ca;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_76e6(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x42, 0xc8, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa58a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_771c(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x43, 0xc9, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb86e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7752(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x44, 0xcc, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x9b2e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7788(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x45, 0xcd, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xae12;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_77be(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x46, 0xd1, 0x92, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xc0f6;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_77f4(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x47, 0xd2, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa3b6;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_782a(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x48, 0xd5, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xacda;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7860(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x49, 0xd6, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xbfbe;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7896(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1f4, 0x4a, 0xd7, 0x98, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa1e2;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_78cc(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x4b, 0xd8, 0x99, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb4c6;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7902(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x4c, 0xd9, 0xee, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x9786;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7938(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x4d, 0xda, 0x9c, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xaa6a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_796e(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x4e, 0xdb, 0x9d, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xbd4e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_79a4(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x4f, 0xdc, 0x9e, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa00e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_79da(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x50, 0xdd, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb2f2;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7a10(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1d9, 0x51, 0xde, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x95b2;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7a46(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x52, 0xdf, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa896;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7a7c(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x53, 0xe0, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xbb7a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7ab2(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1e4, 0x55, 0xe2, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb082;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7ae8(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1e4, 0x56, 0xe3, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xc366;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7b1e(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1da, 0x57, 0xe4, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa626;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7b54(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1d8, 0x58, 0xe5, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb90a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7b8a(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x59, 0xe6, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x9bca;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7bc0(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1ef, 0x5a, 0xe7, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xaeae;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7bf6(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x5b, 0xe8, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xc192;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7c2c(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x5c, 0xea, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb736;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7c62(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x5d, 0xeb, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x99f6;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7c98(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1e6, 0x5e, 0xec, 0xee, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xba42;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7cce(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1da, 0x5f, 0xed, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x9ed6;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7d04(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x60, 0xee, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xb1ba;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7d3a(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1f0, 0x61, 0xef, 0x0, param_2, param_3, param_4);
    param_1->field_0x0                  = 0x947a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


astruct_20 *__stdcall16far struct_1018_7d70(astruct_20 *param_1, ushort param_2, ulong param_3, ushort param_4)

{
    struct_op_1018_6a0e(param_1, 0x1f7, 0x62, 0xf0, 0xcc, param_2, param_3, param_4);
    param_1->field_0x0                  = 0xa75e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    return param_1;
}


void __stdcall16far pass1_1018_620c(ushort *param_1)

{
    astruct_509 *iVar1;
    ushort       uVar1;

    uVar1            = (ushort)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_509 *)param_1;
    *param_1         = 0x66c0;
    iVar1->field_0x2 = 0x1018;
    *param_1         = 0x3ab0;
    iVar1->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    iVar1->field_0x2 = 0x1008;
    return;
}

void __stdcall16far pass1_1018_673c(ushort *param_1)

{
    astruct_510 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_510 *)param_1;
    *param_1          = 0x6880;
    iVar1->field_0x2  = 0x1018;
    iVar1->field_0xe2 = 0x691c;
    iVar1->field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}

astruct_20 *__stdcall16far struct_op_1018_6a0e(astruct_20 *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6, ulong param_7, ushort param_8)

{
    int    iVar1;
    ushort uVar2;

    unk_draw_op_1008_61b2(param_1, param_3, param_6, param_7, param_8);
    uVar2                         = (ushort)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(ushort *)(iVar1 + 0xea)     = param_5;
    *(ushort *)(iVar1 + 0xec)     = param_4;
    *(ushort *)(iVar1 + 0xee)     = param_2;
    *(undefined2 *)(iVar1 + 0xf0) = 0x0;
    param_1->field_0x0            = 0x6c66;
    *(undefined2 *)(iVar1 + 0x2)  = 0x1018;
    *(undefined2 *)(iVar1 + 0xe0) = 0x1;
    *(undefined2 *)(iVar1 + 0xe2) = 0x0;
    *(undefined2 *)(iVar1 + 0xe4) = 0x0;
    *(undefined4 *)(iVar1 + 0xe6) = 0x1df027f;
    return param_1;
}

void __stdcall16far pass1_1018_4aaa(int param_1, ushort param_2, ushort param_3, uchar *param_4, ushort param_5)

{
    struct_op_1018_4cda(param_1, param_2, param_3);
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x4b06;
    *(undefined2 *)(param_1 + 0x2)            = 0x1018;
    pass1_1018_4dce((ulong *)CONCAT22(param_2, param_1), 0x9a, param_4, param_5);
    _PTR_LOOP_1050_4230 = CONCAT22(param_2, param_1);
    return;
}

void __stdcall16far struct_op_1018_4cda(int param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)(param_1 + 0xa)     = 0x0;
    *(undefined4 *)(param_1 + 0xe)     = 0x0;
    *(undefined2 *)(param_1 + 0x12)    = 0x0;
    *(undefined2 *)(param_1 + 0x14)    = 0x0;
    *(undefined2 *)(param_1 + 0x16)    = 0x0;
    *(undefined2 *)(param_1 + 0x18)    = 0x1;
    *(undefined2 *)(param_1 + 0x1a)    = 0x0;
    *(int *)CONCAT22(param_2, param_1) = (int)s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
    *(undefined2 *)(param_1 + 0x2)     = 0x1018;
    return;
}

void __stdcall16far pass1_1018_5070(astruct_641 *param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                        = 0x0;
    param_1->field_0xe                        = 0x0;
    param_1->field_0x12                       = 0x0;
    param_1->field_0x16                       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x56d2;
    param_1->field_0x2                        = 0x1018;
    return;
}

ushort *__stdcall16far pass1_1018_56e6(int param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    *(undefined4 *)(param_1 + 0xa)        = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0x5830;
    *(undefined2 *)(param_1 + 0x2)        = 0x1018;
    return (ushort *)CONCAT22(param_2, param_1);
}

void __stdcall16far pass1_1018_58b6(ushort *param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *param_1                      = (int)s_Alloc__s_1050_5a5b + 0x7;
    *(undefined2 *)(iVar1 + 0x2)  = 0x1018;
    *(undefined2 *)(iVar1 + 0xe2) = 0x5afe;
    *(undefined2 *)(iVar1 + 0xe4) = 0x1018;
    pass1_1020_808e(param_1);
    return;
}

void __stdcall16far struct_1018_4720(ushort *param_1, ulong param_2, ulong param_3)

{
    astruct_204 *iVar1;
    undefined2   uVar1;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_204 *)param_1;
    *param_1         = 0x389a;
    iVar1->field_0x2 = 0x1008;
    iVar1->field_0x4 = param_3;
    iVar1->field_0x8 = param_2;
    iVar1->field_0xc = 0x0;
    *param_1         = (ushort)&PTR_LOOP_1050_4aa6;
    iVar1->field_0x2 = 0x1018;
    return;
}
ushort *__stdcall16far struct_1018_4790(ushort *param_1, ulong param_2, ulong param_3, ushort param_4)

{
    astruct_266 *iVar1;
    undefined2   uVar1;

    struct_1018_4720(param_1, param_2, param_3);
    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_266 *)param_1;
    iVar1->field_0xe = param_4;
    *param_1         = 0x4a92;
    iVar1->field_0x2 = 0x1018;
    iVar1->field_0xc = 0x1;
    return param_1;
}


void __stdcall16far struct_1018_47c8(ushort *param_1, ulong param_2, ulong param_3, ushort param_4, ulong param_5)

{
    astruct_264 *iVar1;
    undefined2   uVar1;

    struct_1018_4720(param_1, param_2, param_3);
    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_264 *)param_1;
    iVar1->field_0xe  = param_5;
    iVar1->field_0x12 = param_4;
    *param_1          = (ushort)&PTR_LOOP_1050_4a9a;
    iVar1->field_0x2  = 0x1018;
    iVar1->field_0xc  = 0x2;
    return;
}


void __stdcall16far pass1_1018_4808(ushort *param_1, ulong param_2, ulong param_3, ulong param_4)

{
    int        iVar1;
    undefined2 uVar2;

    struct_1018_4720(param_1, param_2, param_3);
    uVar2                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    *(ulong *)(iVar1 + 0xe)      = param_4;
    *param_1                     = (ushort)&PTR_LOOP_1050_4aa2;
    *(undefined2 *)(iVar1 + 0x2) = 0x1018;
    *(undefined2 *)(iVar1 + 0xc) = 0x3;
    return;
}


ushort *__stdcall16far struct_1018_4842(ushort *param_1, ulong param_2, ulong param_3, ushort param_4)

{
    astruct_265 *iVar1;
    undefined2   uVar1;

    struct_1018_4720(param_1, param_2, param_3);
    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_265 *)param_1;
    iVar1->field_0xe  = param_4;
    iVar1->field_0x10 = 0x0;
    *param_1          = (ushort)&PTR_LOOP_1050_4a8e;
    iVar1->field_0x2  = 0x1018;
    iVar1->field_0xc  = 0x4;
    return param_1;
}

ushort *__stdcall16far struct_1018_48b0(ushort *param_1, ulong param_2, ulong param_3, ushort param_4)

{
    astruct_207 *iVar1;
    undefined2   uVar1;

    struct_1018_4720(param_1, param_2, param_3);
    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_207 *)param_1;
    iVar1->field_0xe = param_4;
    *param_1         = (ushort)&PTR_LOOP_1050_4a96;
    iVar1->field_0x2 = 0x1018;
    iVar1->field_0xc = 0x5;
    return param_1;
}


ushort *__stdcall16far struct_1018_48e8(ushort *param_1, ulong param_2, ulong param_3, ushort param_4)

{
    int        iVar1;
    undefined2 uVar2;

    struct_1018_4720(param_1, param_2, param_3);
    uVar2                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    *(ushort *)(iVar1 + 0xe)     = param_4;
    *param_1                     = 0x4a9e;
    *(undefined2 *)(iVar1 + 0x2) = 0x1018;
    *(undefined2 *)(iVar1 + 0xc) = 0x6;
    return param_1;
}


void __stdcall16far struct_1018_4920(ushort *param_1, ulong param_2, ulong param_3, ulong param_4)

{
    astruct_203 *iVar1;
    undefined2   uVar1;

    struct_1018_4720(param_1, param_2, param_3);
    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_203 *)param_1;
    iVar1->field_0xe = param_4;
    *param_1         = (ushort)&PTR_LOOP_1050_4a8a;
    iVar1->field_0x2 = 0x1018;
    iVar1->field_0xc = 0x7;
    return;
}
void __stdcall16far struct_1018_2b10(astruct_55 *param_1, ushort param_2, ushort param_3)

{
    undefined4  *puVar1;
    code       **ppcVar2;
    undefined2  *puVar3;
    undefined2   uVar4;
    int          unaff_DI;
    ushort      *puVar5;
    astruct_43  *paVar6;
    ulong        uVar7;
    undefined2   uVar8;
    astruct_626 *uVar9;

    uVar9                                           = (astruct_626 *)param_1;
    uVar8                                           = (undefined2)((ulong)param_1 >> 0x10);
    puVar5                                          = get_sys_metrics_1018_4b1e(param_1, 0x1, param_2);
    uVar9->field_0x20                               = 0x389a;
    uVar9->field_0x22                               = 0x1008;
    uVar9->field_0x20                               = 0x3aa8;
    uVar9->field_0x22                               = 0x1008;
    uVar9->field_0x24                               = (astruct_76 *)0x0;
    uVar9->field_0x174                              = 0x0;
    uVar9->field_0x176                              = 0x0;
    uVar9->field_0x178                              = 0x0;
    uVar9->field_0x17a                              = 0x0;
    uVar9->field_0x17e                              = 0x0;
    uVar9->field_0x182                              = (undefined4 *)0x0;
    uVar9->field_0x186                              = 0x0;
    param_1->field_0x0                              = 0x32d8;
    uVar9->field_0x2                                = 0x1018;
    uVar9->field_0x20                               = 0x3314;
    uVar9->field_0x22                               = 0x1018;
    puVar5                                          = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_3, (uchar *)((ulong)puVar5 >> 0x10), unaff_DI);
    *(int *)&uVar9->field_0x182                     = (int)puVar5;
    *(undefined2 *)((int)&uVar9->field_0x182 + 0x2) = (int)((ulong)puVar5 >> 0x10);
    if(param_1 == (astruct_55 *)0x0)
    {
        puVar3 = (undefined2 *)0x0;
        uVar4  = 0x0;
    }
    else
    {
        puVar3 = &uVar9->field_0x20;
        uVar4  = uVar8;
    }
    puVar1  = uVar9->field_0x182;
    ppcVar2 = (code **)((int)*uVar9->field_0x182 + 0x4);
    (**ppcVar2)(0x1010, (int)puVar1, (int)((ulong)puVar1 >> 0x10), 0x0, puVar3, uVar4);
    puVar1                                         = uVar9->field_0x182;
    uVar9->field_0x17a                             = *(undefined4 *)((int)puVar1 + 0x24);
    paVar6                                         = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x6e, param_3);
    *(int *)&uVar9->field_0x24                     = (int)paVar6;
    *(undefined2 *)((int)&uVar9->field_0x24 + 0x2) = (int)((ulong)paVar6 >> 0x10);
    uVar9->field_0x28                              = 0x0;
    uVar7                                          = pass1_1008_4772(uVar9->field_0x24);
    uVar4                                          = (undefined2)(uVar7 >> 0x10);
    pass1_1018_4b78((ulong *)param_1, param_3);
    uVar9->field_0x16c = 0x1;
    uVar9->field_0x16e = 0x1;
    uVar9->field_0x170 = *(int *)((int)uVar7 + 0x4) + uVar9->field_0x16c;
    uVar9->field_0x172 = *(int *)((int)uVar7 + 0x8) + -0x19;
    return;
}

void __stdcall16far struct_1018_229c(astruct_632 *param_1, uchar *param_2, ushort param_3, uchar *param_4, ushort param_5)

{
    int        *piVar1;
    astruct_43 *paVar2;
    int         iStack4;

    struct_op_1018_4cda((int)param_1, (ushort)param_2, param_3);
    param_1->field_0x1c                       = 0x389a;
    param_1->field_0x1e                       = 0x1008;
    param_1->field_0x1c                       = 0x3aa8;
    param_1->field_0x1e                       = 0x1008;
    param_1->field_0x20                       = 0x0;
    param_1->field_0x24                       = 0x0;
    param_1->field_0x26                       = 0x0;
    *(undefined4 *)&param_1->field_0x2a       = 0x0;
    param_1->field_0x3e                       = 0x0;
    param_1->field_0x40                       = 0x0;
    param_1->field_0x42                       = 0x0;
    param_1->field_0x44                       = 0x0;
    *(undefined4 *)&param_1->field_0x6e       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x2ada;
    param_1->field_0x2                        = 0x1018;
    param_1->field_0x1c                       = (int)s_fem132_wav_1050_2aec + 0x6;
    param_1->field_0x1e                       = 0x1018;
    PTR_LOOP_1050_4230                        = (undefined *)param_1;
    PTR_LOOP_1050_4232                        = param_2;
    pass1_1018_4dce((ulong *)CONCAT22(param_2, param_1), 0x105, param_4, param_5);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1a8, param_5);
    param_1->field_0x2a = (int)paVar2;
    param_1->field_0x2c = (int)((ulong)paVar2 >> 0x10);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0x2e), (WNDCLASS16 *)0x0, 0x10);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0x46), (WNDCLASS16 *)0x0, 0x28);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x6c, param_5);
    param_1->field_0x2e = (int)paVar2;
    param_1->field_0x30 = (int)((ulong)paVar2 >> 0x10);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x68, param_5);
    param_1->field_0x32 = (int)paVar2;
    param_1->field_0x34 = (int)((ulong)paVar2 >> 0x10);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x66, param_5);
    param_1->field_0x36 = (int)paVar2;
    param_1->field_0x38 = (int)((ulong)paVar2 >> 0x10);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x6a, param_5);
    param_1->field_0x3a = (int)paVar2;
    param_1->field_0x3c = (int)((ulong)paVar2 >> 0x10);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1cd, param_5);
    param_1->field_0x6e = (int)paVar2;
    param_1->field_0x70 = (int)((ulong)paVar2 >> 0x10);
    iStack4             = 0x0;
    do
    {
        paVar2                                                = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, iStack4 + 0x8f, param_5);
        *(undefined2 *)(&param_1->field_0x46 + iStack4 * 0x4) = (int)paVar2;
        *(undefined2 *)(&param_1->field_0x48 + iStack4 * 0x4) = (int)((ulong)paVar2 >> 0x10);
        iStack4                                               = iStack4 + 0x1;
    } while(iStack4 < 0xa);
    if(CONCAT22(param_2, param_1) == 0x0)
    {
        piVar1  = (int *)0x0;
        param_2 = (uchar *)0x0;
    }
    else
    {
        piVar1 = &param_1->field_0x1c;
    }
    pass1_1008_9262((int)_PTR_LOOP_1050_0388, (ushort)((ulong)_PTR_LOOP_1050_0388 >> 0x10), 0x73, CONCAT22(param_2, piVar1), (uint)piVar1, param_2);
    return;
}


void __stdcall16far struct_1010_dd5e(ushort param_1, ushort param_2, ulong param_3)

{
    int   iVar1;
    int   iVar2;
    uint  uVar3;
    ulong uVar4;
    long *plStack16;

    if(param_3 != 0x0)
    {
        uVar4     = struct_op_1030_73a8(param_3);
        uVar3     = (uint)(uVar4 >> 0x10);
        iVar2     = (int)uVar4;
        plStack16 = (long *)(uVar4 & 0xffff0000 | (ulong)(iVar2 + 0x14U));
        if((uVar3 | iVar2 + 0x14U) != 0x0)
        {
            iVar1 = *(int *)(iVar2 + 0x12);
            iVar2 = *(int *)(iVar2 + 0x18);
            if(((((iVar1 == 0x4) || ((((iVar1 == 0x6 && (iVar2 == 0x4)) || (iVar1 == 0x5)) || ((iVar1 == 0x6 && (iVar2 == 0x5)))))) || (iVar1 == 0x8)) || ((iVar1 == 0x6 && (iVar2 == 0x8)))) && (*plStack16 != 0x0))
            {
                return;
            }
        }
    }
    return;
}

void __stdcall16far pass1_1010_e8f6(ushort param_1, ushort param_2, ulong param_3, ushort param_4)

{
    undefined2 uVar1;
    BOOL16     BVar2;
    undefined2 uVar3;
    ulong      uVar4;

    uVar4 = struct_op_1030_73a8(param_3);
    uVar1 = *(undefined2 *)((int)uVar4 + 0xc);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x13);
    if(BVar2 == 0x0)
    {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x14);
        if(BVar2 == 0x0)
        {
            return;
        }
        uVar4 = pass1_1028_4faa(uVar4, param_4);
        uVar3 = (undefined2)(uVar4 >> 0x10);
        uVar1 = (undefined2)uVar4;
    }
    else
    {
        uVar4 = pass1_1028_121e(uVar4, param_4);
        uVar3 = (undefined2)(uVar4 >> 0x10);
        uVar1 = (undefined2)uVar4;
    }
    pass1_1028_b58e(CONCAT22(uVar3, uVar1));
    return;
}

void __stdcall16far struct_1010_a1d8(astruct_627 *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int         iVar1;
    code      **ppcVar2;
    int         unaff_DI;
    astruct_79 *paVar3;
    ushort     *puVar4;
    uint        uStack4;

    paVar3                                            = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                                = 0x389a;
    param_1->field_0xc                                = 0x1008;
    param_1->field_0xa                                = 0x3aa8;
    param_1->field_0xc                                = 0x1008;
    param_1->field_0x138                              = (undefined4 *)0x0;
    *(undefined2 *)CONCAT22(param_2, param_1)         = 0xe9cc;
    param_1->field_0x2                                = 0x1010;
    param_1->field_0xa                                = 0xe9dc;
    param_1->field_0xc                                = 0x1010;
    puVar4                                            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_4, (uchar *)((ulong)paVar3 >> 0x10), unaff_DI);
    *(int *)&param_1->field_0x138                     = (int)puVar4;
    *(undefined2 *)((int)&param_1->field_0x138 + 0x2) = (int)((ulong)puVar4 >> 0x10);
    ppcVar2                                           = (code **)((int)*param_1->field_0x138 + 0x4);
    (**ppcVar2)();
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0xa4), (WNDCLASS16 *)0x0, 0x94);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0xe), (WNDCLASS16 *)0x0, 0x96);
    uStack4 = 0x0;
    do
    {
        iVar1                         = &param_1->field_0x0 + uStack4 * 0x6;
        *(code **)(iVar1 + 0xe)       = pass1_1010_c7e2;
        *(undefined2 *)(iVar1 + 0x12) = 0x0;
        uStack4                       = uStack4 + 0x1;
    } while(uStack4 < 0x19);
    param_1->field_0x4a = pass1_1010_c864;
    param_1->field_0x4e = 0x0;
    param_1->field_0x50 = pass1_1010_cc56;
    param_1->field_0x54 = 0x0;
    param_1->field_0x56 = pass1_1010_cf36;
    param_1->field_0x5a = 0x0;
    param_1->field_0x2c = pass1_1010_d24a;
    param_1->field_0x30 = 0x0;
    param_1->field_0x6e = pass1_1010_d448;
    param_1->field_0x72 = 0x0;
    param_1->field_0x74 = pass1_1010_d5ae;
    param_1->field_0x78 = 0x0;
    param_1->field_0x98 = pass1_1010_d710;
    param_1->field_0x9c = 0x0;
    return;
}

ushort __stdcall16far pass1_1010_a5ac(ushort param_1, ushort param_2, ulong param_3)

{
    ulong uVar1;

    uVar1 = struct_op_1030_73a8(param_3);
    return *(ushort *)((int)uVar1 + 0x20);
}

ushort __stdcall16far pass1_1010_acc0(ushort param_1, ushort param_2, ulong param_3)

{
    ulong uVar1;

    uVar1 = struct_op_1030_73a8(param_3);
    if(*(int *)((int)uVar1 + 0x12) != 0x4)
    {
        return 0x1;
    }
    return 0x0;
}

void __stdcall16far struct_1010_9172(ulong param_1)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_249 *iVar4;
    undefined2   uVar4;
    astruct_75  *paVar5;
    undefined4   uVar6;

    uVar4  = (undefined2)(param_1 >> 0x10);
    iVar4  = (astruct_249 *)param_1;
    puVar1 = iVar4->field_0x4;
    uVar2  = iVar4->field_0x6;
    paVar5 = (astruct_75 *)CONCAT22(uVar2, puVar1);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        paVar5  = (astruct_75 *)(**ppcVar3)();
    }
    mem_op_1000_179c(0x18, (uchar *)((ulong)paVar5 >> 0x10), 0x1000);
    if(paVar5 == (astruct_75 *)0x0)
    {
        uVar6 = 0x0;
    }
    else
    {
        uVar6 = struct_op_1030_1cd8(paVar5, 0x5, 0x5);
    }
    iVar4->field_0x4 = (undefined4 *)uVar6;
    iVar4->field_0x6 = (uint)((ulong)uVar6 >> 0x10);
    return;
}

ushort *__stdcall16far pass1_1010_9258(ushort *param_1)

{
    struct_1010_383a(param_1);
    *param_1                            = 0x958e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
    return param_1;
}

void __stdcall16far struct_1010_95aa(astruct_629 *param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                        = 0x0;
    param_1->field_0xe                        = 0x0;
    param_1->field_0x12                       = 0x0;
    param_1->field_0x16                       = 0x0;
    param_1->field_0x18                       = 0x0;
    param_1->field_0x1a                       = 0x0;
    param_1->field_0x1c                       = 0xa;
    param_1->field_0x1e                       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0xa1c8;
    param_1->field_0x2                        = 0x1010;
    return;
}

void __stdcall16far struct_1010_6326(astruct_630 *param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                        = 0x0;
    param_1->field_0xe                        = 0x0;
    param_1->field_0x12                       = 0x0;
    param_1->field_0x16                       = 0x0;
    param_1->field_0x1a                       = 0x0;
    param_1->field_0x1e                       = 0x0;
    param_1->field_0x22                       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x66f0;
    param_1->field_0x2                        = 0x1010;
    return;
}

ulong __stdcall16far pass1_1010_6700(astruct_636 *param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0x148                      = 0x33;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x6aac;
    param_1->field_0x2                        = 0x1010;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0xa), (WNDCLASS16 *)0x0, 0x114);
    param_1->field_0x32  = 0x1;
    param_1->field_0x40  = 0x1;
    param_1->field_0x46  = 0x1;
    param_1->field_0x4e  = 0x1;
    param_1->field_0x54  = 0x1;
    param_1->field_0x5e  = 0x1;
    param_1->field_0x68  = 0x1;
    param_1->field_0x6c  = 0x1;
    param_1->field_0x74  = 0x1;
    param_1->field_0x78  = 0x1;
    param_1->field_0x7a  = 0x1;
    param_1->field_0x7e  = 0x1;
    param_1->field_0x82  = 0x1;
    param_1->field_0xa2  = 0x1;
    param_1->field_0xa4  = 0x1;
    param_1->field_0xa6  = 0x1;
    param_1->field_0xa8  = 0x1;
    param_1->field_0xae  = 0x1;
    param_1->field_0xb2  = 0x1;
    param_1->field_0xb8  = 0x1;
    param_1->field_0xbe  = 0x1;
    param_1->field_0xc0  = 0x1;
    param_1->field_0xc4  = 0x1;
    param_1->field_0xd4  = 0x1;
    param_1->field_0xda  = 0x1;
    param_1->field_0xe2  = 0x1;
    param_1->field_0xfe  = 0x1;
    param_1->field_0x100 = 0x1;
    param_1->field_0x102 = 0x1;
    param_1->field_0x104 = 0x1;
    param_1->field_0x106 = 0x1;
    param_1->field_0x108 = 0x1;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0x11e), (WNDCLASS16 *)0x0, 0x2a);
    param_1->field_0x120 = 0x1;
    param_1->field_0x122 = 0x1;
    param_1->field_0x124 = 0x1;
    param_1->field_0x126 = 0x1;
    param_1->field_0x128 = 0x1;
    param_1->field_0x12c = 0x1;
    param_1->field_0x138 = 0x1;
    return CONCAT22(param_2, param_1);
}


void __stdcall16far struct_1010_50b2(astruct_646 *param_1, ushort param_2, ushort param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa                        = 0x0;
    param_1->field_0xc                        = 0x0;
    param_1->field_0x10                       = 0x0;
    param_1->field_0x12                       = 0x0;
    param_1->field_0x16                       = 0x0;
    *(undefined2 *)CONCAT22(param_2, param_1) = 0x53f4;
    param_1->field_0x2                        = 0x1010;
    return;
}
