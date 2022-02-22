
u16 * struct_1028_406c(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0x42ec;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_4354(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0x446a;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_44d2(u16 *param_1)

{
    u16 uVar1;

    struct_1028_b354(param_1);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x20) = 0x0;
    *param_1         = 0x4836;
    (param_1 + 0x2)  = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_489e(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = &PTR_LOOP_1050_4942;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_49aa(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0x4b1c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (param_1 + 0x20)), 0x0, 0xa);
    return param_1;
}


u16 * struct_1028_4b84(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = s_SCi16ernalPutBldg2_site_0x_08lx__1050_506f + 0x1;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


void  pass1_1028_4db2(u16 param_1, u16 param_2, i16 param_3, u8 *param_4, i16 param_5, u16 param_6, u8 param_7)

{
    BOOL16     BVar1;
    i16       *piVar2;
    u16        extraout_DX;
    u16       *puVar3;
    i16       *piVar4;
    u16        uVar5;
    u16       *puVar6;
    u16        uVar7;
    u8         local_14e[0x124];
    u32        uStack42;
    undefined4 uStack38;
    i16        local_22;
    u8         local_20[0x2];
    u8         local_1e[0x2];
    u32        local_1c;
    i16        iStack24;
    undefined4 uStack22;
    i16       *piStack18;
    u16        uStack16;
    i16        local_e;
    u16        local_c;
    u32        uStack10;
    u16       *puStack6;

    BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, (param_1 + 0xc), 0x29);
    if(BVar1 != 0x0)
    {
        pass1_1028_bd38(CONCAT22(param_2, param_1), param_4, param_6);
        if((param_3 == 0x0) && ((param_1 + 0xc) == 0x13))
        {
            puVar3  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x8, param_6, param_4, param_5);
            param_4 = (puVar3 >> 0x10);
            pass1_1010_988c(puVar3, (param_1 + 0xc));
        }
        puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_6, param_4, param_5);
        uStack16 = (puStack6 >> 0x10);
        uStack10 = *(puStack6 + 0x20);
        puVar6   = &local_c;
        piVar2   = &local_e;
        piVar4   = piVar2;
        uVar5    = param_6;
        uVar7    = param_6;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack10, (uStack10 >> 0x10));
        piStack18 = piVar2;
        pass1_1030_5b1c(CONCAT22(uStack16, piVar2), CONCAT22(uVar5, piVar4), CONCAT22(uVar7, puVar6));
        pass1_1028_b58e(CONCAT22(param_2, param_1));
        uStack22 = CONCAT22(extraout_DX, piVar2);
        local_1c = *(piVar2 + 0x6);
        iStack24 = piVar2[0x8];
        pass1_1028_c8ee(param_6, param_1, param_2, 0x1, CONCAT22(param_6, &local_1c));
        pass1_1008_3eb4(CONCAT22(param_6, &local_1c), CONCAT22(param_6, &local_22), CONCAT22(param_6, local_20), CONCAT22(param_6, local_1e));
        if(local_e < local_22)
        {
            pass1_1030_5b3e(CONCAT22(uStack16, piStack18), local_22, local_c);
            pass1_1030_5b1c(CONCAT22(uStack16, piStack18), CONCAT22(param_6, &local_e), CONCAT22(param_6, &local_c));
        }
        uStack38 = (uStack22 + 0x2e);
        uStack42 = *(uStack38 + 0x4);
        struct_op_1028_87f0(param_6, param_7, (astruct_97 *)CONCAT22(param_6, local_14e), 0x0, 0x0, 0x62, &local_1c, param_6, uStack42, uStack10);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, CONCAT22(param_6, local_14e));
        pass1_1028_ccd0(param_7, param_6, CONCAT22(param_2, param_1), CONCAT22(param_6, &local_1c));
    }
    return;
}


u32  pass1_1028_4faa(u32 param_1, u16 param_2)

{
    u16         uVar1;
    undefined4 *puVar2;
    u16         uVar3;
    u32         uVar4;
    undefined4  local_c;
    u16         uStack8;
    undefined4  uStack6;

    uVar1 = pass1_1028_4f62(param_1);
    if(uVar1 != 0x0)
    {
        return param_1;
    }
    uStack6 = pass1_1028_b58e(param_1);
    local_c = (uStack6 + 0xc);
    uStack8 = 0x0;
    uVar4   = pass1_1028_bb24(param_1);
    uVar3   = (uVar4 >> 0x10);
    puVar2  = &local_c;
    pass1_1030_627e(param_2, puVar2, uVar3, globals->_PTR_LOOP_1050_5740, CONCAT22(param_2, puVar2), uVar4 & 0xffff | uVar3 << 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, puVar2, uVar3);
    if((uVar3 | puVar2) == 0x0)
    {
        return 0x0;
    }
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3, puVar2));
    return uVar4;
}


u16 * struct_1028_3484(u16 *param_1)

{
    u8 *in_DX;

    struct_1028_0068(param_1, in_DX);
    *param_1        = 0x34f6;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_355e(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0x3608;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_3670(u16 *param_1, u8 *param_2, u16 param_3, u16 param_4)

{
    struct_1028_37a6(param_1, param_2, param_3, param_4);
    *param_1        = 0x373e;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_3e94(u16 *param_1)

{
    u16 uVar1;

    struct_1028_b354(param_1);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x20) = 0x0;
    *param_1         = 0x4004;
    (param_1 + 0x2)  = &USHORT_1050_1028;
    pass1_1028_3fa2(param_1 & 0xffff | uVar1 << 0x10);
    return param_1;
}


u16 * struct_1028_1bbc(u16 *param_1)

{
    astruct_190 *iVar1;
    u16          uVar1;

    struct_1028_b354(param_1);
    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_190 *)param_1;
    iVar1->field_0x20 = 0x0;
    iVar1->field_0x22 = 0x0;
    *param_1          = 0x1eee;
    iVar1->field_0x2  = &USHORT_1050_1028;
    return param_1;
}


u16  pass1_1028_1e14(u16 param_1, u16 param_2, i16 param_3, u16 *param_4, long param_5, u16 param_6, u16 param_7, u16 param_8)

{
    i16 iVar1;
    u16 uVar2;
    u32 uVar3;

    pass1_1030_627e(param_8, param_6, param_7, globals->_PTR_LOOP_1050_5740, param_4, param_5);
    uVar2 = param_7 | param_6;
    if(uVar2 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_6, param_7);
        if((uVar2 | param_6) != 0x0)
        {
            uVar3 = struct_op_1030_73a8(CONCAT22(uVar2, param_6));
            if(uVar3 != 0x0)
            {
                iVar1 = (uVar3 + 0xc);
                if(((iVar1 == 0x18) || (iVar1 == 0x3f)) || (iVar1 == param_3))
                {
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}


u16  pass1_1028_21ba(u16 param_1, u16 param_2, u16 *param_3, long param_4, u16 param_5, u16 param_6, u16 param_7)

{
    u16 uVar1;
    u32 uVar2;

    pass1_1030_627e(param_7, param_5, param_6, globals->_PTR_LOOP_1050_5740, param_3, param_4);
    uVar1 = param_6 | param_5;
    if(uVar1 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5, param_6);
        if((uVar1 | param_5) != 0x0)
        {
            uVar2 = struct_op_1030_73a8(CONCAT22(uVar1, param_5));
            if((uVar2 != 0x0) && ((uVar2 + 0xc) == 0x40))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16  pass1_1028_2220(u16 param_1, u16 param_2, i16 param_3, u16 *param_4, long param_5, u16 param_6, u16 param_7, u16 param_8)

{
    i16 iVar1;
    u16 uVar2;
    u32 uVar3;

    pass1_1030_627e(param_8, param_6, param_7, globals->_PTR_LOOP_1050_5740, param_4, param_5);
    uVar2 = param_7 | param_6;
    if(uVar2 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_6, param_7);
        if((uVar2 | param_6) != 0x0)
        {
            uVar3 = struct_op_1030_73a8(CONCAT22(uVar2, param_6));
            if((uVar3 != 0x0) && ((iVar1 = (uVar3 + 0xc), iVar1 == 0x40 || (iVar1 == param_3))))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}


u16 * struct_1028_25da(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = s_fem16_wav_1050_2644 + 0x8;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_26b4(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0x2788;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_27f0(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0x2a92;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_2afa(u16 *param_1)

{
    struct_1030_dc96(param_1);
    *param_1        = 0x2b74;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_2bdc(u16 *param_1)

{
    struct_1028_0954(param_1);
    *param_1        = 0x341c;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_0b42(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0xbbc;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1028_0c24(u16 *param_1)

{
    astruct_191 *iVar1;
    u16          uVar1;

    struct_1028_b354(param_1);
    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_191 *)param_1;
    iVar1->field_0x20 = 0x0;
    iVar1->field_0x22 = 0x0;
    *param_1          = s_480_bmp_1050_1721 + 0x3;
    iVar1->field_0x2  = &USHORT_1050_1028;
    return param_1;
}


u32  pass1_1028_121e(u32 param_1, u16 param_2)

{
    bool        bVar1;
    u8          extraout_AH;
    undefined4 *puVar2;
    u16         uVar3;
    u32         uVar4;
    undefined4  local_c;
    u16         uStack8;
    undefined4  uStack6;

    bVar1 = pass1_1028_11de(param_1);
    if(CONCAT11(extraout_AH, bVar1) != 0x0)
    {
        return param_1;
    }
    uStack6 = pass1_1028_b58e(param_1);
    local_c = (uStack6 + 0xc);
    uStack8 = 0x0;
    uVar4   = pass1_1028_bb24(param_1);
    uVar3   = (uVar4 >> 0x10);
    puVar2  = &local_c;
    pass1_1030_627e(param_2, puVar2, uVar3, globals->_PTR_LOOP_1050_5740, CONCAT22(param_2, puVar2), uVar4 & 0xffff | uVar3 << 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, puVar2, uVar3);
    if((uVar3 | puVar2) == 0x0)
    {
        return 0x0;
    }
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3, puVar2));
    return uVar4;
}


u16 * struct_1028_178c(u16 *param_1)

{
    struct_1030_dc96(param_1);
    *param_1        = 0x1b54;
    (param_1 + 0x2) = &USHORT_1050_1028;
    return param_1;
}


u16 * struct_1020_e8f6(u16 *param_1)

{
    u16 uVar1;

    struct_1030_dc96(param_1);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x24) = 0x0;
    *param_1         = 0xeef6;
    (param_1 + 0x2)  = 0x1020;
    return param_1;
}

u16 * struct_1028_0954(u16 *param_1)

{
    astruct_185 *iVar1;
    u16          uVar1;

    struct_1028_b354(param_1);
    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_185 *)param_1;
    iVar1->field_0x20 = 0x0;
    *param_1          = 0xada;
    iVar1->field_0x2  = &USHORT_1050_1028;
    iVar1->field_0xe  = 0x4b;
    return param_1;
}

u16 * struct_1020_d866(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0xd8ec;
    (param_1 + 0x2) = 0x1020;
    return param_1;
}

u16 * struct_1020_e7fa(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0xe88e;
    (param_1 + 0x2) = 0x1020;
    return param_1;
}

u16 * struct_1020_c9ea(u16 *param_1)

{
    struct_1028_0954(param_1);
    *param_1        = 0xcc7c;
    (param_1 + 0x2) = 0x1020;
    return param_1;
}

u16 * struct_1020_cce4(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0xcd7e;
    (param_1 + 0x2) = 0x1020;
    return param_1;
}

u16 * struct_1020_cde6(u16 *param_1)

{
    struct_1028_0954(param_1);
    *param_1        = 0xd004;
    (param_1 + 0x2) = 0x1020;
    return param_1;
}

u16 * struct_1020_d06c(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0xd314;
    (param_1 + 0x2) = 0x1020;
    return param_1;
}

u16 * struct_1020_d37c(u16 *param_1)

{
    u16 uVar1;

    struct_1028_b354(param_1);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x20) = 0x0;
    *param_1         = 0xd53e;
    (param_1 + 0x2)  = 0x1020;
    return param_1;
}

u16 * struct_1020_d5a6(u16 *param_1)

{
    struct_1028_b354(param_1);
    *param_1        = 0xd7fe;
    (param_1 + 0x2) = 0x1020;
    return param_1;
}

void  struct_1020_c444(astruct_75 *param_1, u32 param_2, u32 param_3)

{
    astruct_75 *iVar1;
    astruct_75 *uVar1;

    struct_op_1030_1cd8(param_1, param_2, param_3);
    uVar1                 = (param_1 >> 0x10);
    iVar1                 = param_1;
    (iVar1 + 0x1)         = 0x0;
    &iVar1[0x1].field_0x4 = 0x0;
    param_1->field_0x0    = 0xc834;
    iVar1->field_0x2      = 0x1020;
    return;
}

void  pass1_1020_afc4(u16 param_1, u16 param_2, u16 param_3, u16 param_4, u16 *param_5, long param_6)

{
    undefined4 *puVar1;
    u16         uVar2;
    u16         uVar3;
    u32         uVar4;
    u8        bStack27;
    undefined4  local_a;
    undefined4  uStack6;

    puVar1 = &local_a;
    pass1_1030_64ce(param_1, puVar1, param_2, globals->_PTR_LOOP_1050_5740, param_5, param_6, CONCAT22(param_1, puVar1));
    uStack6  = *puVar1;
    uVar3    = (puVar1 + 0x2);
    bStack27 = (u8)(uStack6 >> 0x18);
    uVar2    = bStack27;
    if(bStack27 == 0x0)
    {
        return;
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack6, uVar3);
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3, uVar2));
    uVar3 = (uVar4 >> 0x10);
    if((uVar3 | uVar4) != 0x0)
    {
        switch((uVar4 + 0xc))
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


void  pass1_1020_289a(u16 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    i16 iVar1;
    u16 uVar2;

    struct_1020_790e(param_1, s_SCPOPMENU_1050_427c, param_2, param_3, param_4);
    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    (iVar1 + 0xf2) = 0x0;
    (iVar1 + 0xf6) = 0x0;
    (iVar1 + 0xfa) = 0x0;
    (iVar1 + 0xfc) = 0x0;
    *param_1       = 0x2e4a;
    (iVar1 + 0x2)  = 0x1020;
    unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (iVar1 + 0x5b)), s_VrMode_1050_4286);
    (iVar1 + 0xac) = 0x44c00000;
    return;
}

void  struct_1020_0baa(u16 *param_1, u16 param_2, u8 *param_3, u16 param_4)

{
    u8          *puVar1;
    astruct_276 *iVar2;
    i16          unaff_DI;
    u16          uVar2;
    u16         *puVar3;
    u16         *puVar4;
    u16         *puVar5;
    u16          uVar6;

    uVar2             = (param_1 >> 0x10);
    iVar2             = (astruct_276 *)param_1;
    *param_1          = 0x389a;
    iVar2->field_0x2  = 0x1008;
    *param_1          = 0x3aa8;
    iVar2->field_0x2  = 0x1008;
    iVar2->field_0x4  = param_2;
    *param_1          = 0x3ab0;
    iVar2->field_0x2  = 0x1008;
    &iVar2->field_0x6 = 0x0;
    iVar2->field_0xa  = 0x0;
    iVar2->field_0xc  = 0x0;
    *param_1          = 0xdbc;
    iVar2->field_0x2  = 0x1020;
    puVar3            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x7, param_4, param_3, unaff_DI);
    puVar1            = (puVar3 >> 0x10);
    iVar2->field_0x6  = puVar3;
    iVar2->field_0x8  = puVar1;
    puVar5            = &iVar2->field_0xa;
    puVar4            = &iVar2->field_0xc;
    uVar6             = uVar2;
    puVar3            = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, puVar1, unaff_DI);
    pass1_1008_3e94((puVar3 & 0xffff0000 | (puVar3 + 0xe)), CONCAT22(uVar2, puVar4), CONCAT22(uVar6, puVar5));
    return;
}

astruct_20 * struct_1018_6d02(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0xb, 0x9c, 0x8b, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa27e;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6d38(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0xc, 0x9d, 0xd0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb562;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6d6e(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0xd, 0x9e, 0xd1, param_2, param_3, param_4);
    param_1->field_0x0 = 0x9822;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6da4(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0xe, 0x9f, 0xd2, param_2, param_3, param_4);
    param_1->field_0x0 = 0xab06;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6dda(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0xf, 0xa0, 0xd4, param_2, param_3, param_4);
    param_1->field_0x0 = 0xbdea;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6e10(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x10, 0xa1, 0xda, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa0aa;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6e46(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x11, 0xa2, 0xdc, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb38e;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6e7c(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x12, 0xa3, 0xd3, param_2, param_3, param_4);
    param_1->field_0x0 = 0x964e;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6eb2(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x13, 0xa4, 0xdb, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa932;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6ee8(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x14, 0xa5, 0xa5, param_2, param_3, param_4);
    param_1->field_0x0 = 0xbc16;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6f1e(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x15, 0xa7, 0xb2, param_2, param_3, param_4);
    param_1->field_0x0 = 0x9e3a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6f54(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x16, 0xa8, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb11e;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6f8a(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x17, 0xaf, 0xc0, param_2, param_3, param_4);
    param_1->field_0x0 = 0x93de;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6fc0(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x18, 0xb0, 0xc1, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa6c2;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_6ff6(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x19, 0xb1, 0x80, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb9a6;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_702c(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1ec, 0x1a, 0xb2, 0xc3, param_2, param_3, param_4);
    param_1->field_0x0 = 0x9c66;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7062(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x1b, 0xb3, 0xc4, param_2, param_3, param_4);
    param_1->field_0x0 = 0xaf4a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7098(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x1c, 0xb4, 0xd8, param_2, param_3, param_4);
    param_1->field_0x0 = 0xc22e;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_70ce(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x1d, 0xb5, 0x7b, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa4ee;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7104(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x1e, 0xb6, 0xd9, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb7d2;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_713a(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x1f, 0xb7, 0x7d, param_2, param_3, param_4);
    param_1->field_0x0 = 0x9a92;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7170(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x21, 0xb9, 0xdd, param_2, param_3, param_4);
    param_1->field_0x0 = 0xad76;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_71a6(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x23, 0xd3, 0xd6, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb69a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_71dc(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1ed, 0x24, 0xd4, 0xd7, param_2, param_3, param_4);
    param_1->field_0x0 = 0x995a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7212(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x25, 0xe9, 0xee, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa452;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7248(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x63, 0xa6, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xc05a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_727e(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x64, 0xa9, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa31a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_72b4(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x65, 0xaa, 0xbb, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb5fe;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_72ea(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x66, 0xab, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0x98be;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7320(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x67, 0xac, 0xbd, param_2, param_3, param_4);
    param_1->field_0x0 = 0xaba2;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7356(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x68, 0xad, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xbe86;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_738c(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x69, 0xae, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xac3e;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_73c2(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x35, 0xba, 0x81, param_2, param_3, param_4);
    param_1->field_0x0 = 0xbf22;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_73f8(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x39, 0xbb, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa146;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_745e(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x22, 0xbc, 0xd5, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb42a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7494(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x36, 0xbd, 0xcd, param_2, param_3, param_4);
    param_1->field_0x0 = 0x96ea;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_74ca(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x37, 0xbe, 0x83, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa9ce;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7500(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x38, 0xbf, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xbcb2;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7536(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x3a, 0xc0, 0x85, param_2, param_3, param_4);
    param_1->field_0x0 = 0x9f72;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_756c(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1e2, 0x3b, 0xc1, 0x86, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb256;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * pass1_1018_75a2(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x3c, 0xc2, 0x87, param_2, param_3, param_4);
    param_1->field_0x0 = 0x9516;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * pass1_1018_75d8(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x3d, 0xc3, 0x88, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa7fa;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_760e(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x3e, 0xc4, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xbade;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7644(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x3f, 0xc5, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0x9d02;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_767a(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x40, 0xc6, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xafe6;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_76b0(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x41, 0xc7, 0x8d, param_2, param_3, param_4);
    param_1->field_0x0 = 0xc2ca;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_76e6(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x42, 0xc8, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa58a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_771c(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x43, 0xc9, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb86e;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7752(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x44, 0xcc, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0x9b2e;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7788(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x45, 0xcd, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xae12;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_77be(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x46, 0xd1, 0x92, param_2, param_3, param_4);
    param_1->field_0x0 = 0xc0f6;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_77f4(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x47, 0xd2, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa3b6;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_782a(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x48, 0xd5, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xacda;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7860(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x49, 0xd6, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xbfbe;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7896(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1f4, 0x4a, 0xd7, 0x98, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa1e2;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_78cc(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x4b, 0xd8, 0x99, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb4c6;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7902(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x4c, 0xd9, 0xee, param_2, param_3, param_4);
    param_1->field_0x0 = 0x9786;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7938(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x4d, 0xda, 0x9c, param_2, param_3, param_4);
    param_1->field_0x0 = 0xaa6a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_796e(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x4e, 0xdb, 0x9d, param_2, param_3, param_4);
    param_1->field_0x0 = 0xbd4e;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_79a4(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x4f, 0xdc, 0x9e, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa00e;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_79da(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x50, 0xdd, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb2f2;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7a10(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1d9, 0x51, 0xde, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0x95b2;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7a46(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x52, 0xdf, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa896;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7a7c(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x53, 0xe0, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xbb7a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7ab2(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1e4, 0x55, 0xe2, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb082;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7ae8(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1e4, 0x56, 0xe3, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xc366;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7b1e(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1da, 0x57, 0xe4, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa626;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7b54(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1d8, 0x58, 0xe5, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb90a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7b8a(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x59, 0xe6, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0x9bca;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7bc0(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1ef, 0x5a, 0xe7, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xaeae;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7bf6(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x5b, 0xe8, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xc192;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7c2c(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x5c, 0xea, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb736;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7c62(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x5d, 0xeb, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0x99f6;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7c98(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1e6, 0x5e, 0xec, 0xee, param_2, param_3, param_4);
    param_1->field_0x0 = 0xba42;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7cce(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1da, 0x5f, 0xed, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0x9ed6;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7d04(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x0, 0x60, 0xee, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0xb1ba;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7d3a(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1f0, 0x61, 0xef, 0x0, param_2, param_3, param_4);
    param_1->field_0x0 = 0x947a;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


astruct_20 * struct_1018_7d70(astruct_20 *param_1, u16 param_2, u32 param_3, u16 param_4)

{
    struct_op_1018_6a0e(param_1, 0x1f7, 0x62, 0xf0, 0xcc, param_2, param_3, param_4);
    param_1->field_0x0 = 0xa75e;
    (param_1 + 0x2)    = 0x1018;
    return param_1;
}


void  pass1_1018_620c(u16 *param_1)

{
    astruct_509 *iVar1;
    u16          uVar1;

    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_509 *)param_1;
    *param_1         = 0x66c0;
    iVar1->field_0x2 = 0x1018;
    *param_1         = 0x3ab0;
    iVar1->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    iVar1->field_0x2 = 0x1008;
    return;
}

void  pass1_1018_673c(u16 *param_1)

{
    astruct_510 *iVar1;
    u16          uVar1;

    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_510 *)param_1;
    *param_1          = 0x6880;
    iVar1->field_0x2  = 0x1018;
    iVar1->field_0xe2 = 0x691c;
    iVar1->field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}

astruct_20 * struct_op_1018_6a0e(astruct_20 *param_1, u16 param_2, u16 param_3, u16 param_4, u16 param_5, u16 param_6, u32 param_7, u16 param_8)

{
    i16 iVar1;
    u16 uVar2;

    unk_draw_op_1008_61b2(param_1, param_3, param_6, param_7, param_8);
    uVar2              = (param_1 >> 0x10);
    iVar1              = param_1;
    (iVar1 + 0xea)     = param_5;
    (iVar1 + 0xec)     = param_4;
    (iVar1 + 0xee)     = param_2;
    (iVar1 + 0xf0)     = 0x0;
    param_1->field_0x0 = 0x6c66;
    (iVar1 + 0x2)      = 0x1018;
    (iVar1 + 0xe0)     = 0x1;
    (iVar1 + 0xe2)     = 0x0;
    (iVar1 + 0xe4)     = 0x0;
    (iVar1 + 0xe6)     = 0x1df027f;
    return param_1;
}

void  pass1_1018_4aaa(i16 param_1, u16 param_2, u16 param_3, u8 *param_4, u16 param_5)

{
    struct_op_1018_4cda(param_1, param_2, param_3);
    CONCAT22(param_2, param_1) = 0x4b06;
    (param_1 + 0x2)            = 0x1018;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x9a, param_4, param_5);
    globals->_PTR_LOOP_1050_4230 = CONCAT22(param_2, param_1);
    return;
}

void  struct_op_1018_4cda(i16 param_1, u16 param_2, u16 param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    (param_1 + 0xa)            = 0x0;
    (param_1 + 0xe)            = 0x0;
    (param_1 + 0x12)           = 0x0;
    (param_1 + 0x14)           = 0x0;
    (param_1 + 0x16)           = 0x0;
    (param_1 + 0x18)           = 0x1;
    (param_1 + 0x1a)           = 0x0;
    CONCAT22(param_2, param_1) = s_SCi16ernalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
    (param_1 + 0x2)            = 0x1018;
    return;
}

void  pass1_1018_5070(astruct_641 *param_1, u16 param_2, u16 param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa         = 0x0;
    param_1->field_0xe         = 0x0;
    param_1->field_0x12        = 0x0;
    param_1->field_0x16        = 0x0;
    CONCAT22(param_2, param_1) = 0x56d2;
    param_1->field_0x2         = 0x1018;
    return;
}

u16 * pass1_1018_56e6(i16 param_1, u16 param_2, u16 param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    (param_1 + 0xa)            = 0x0;
    CONCAT22(param_2, param_1) = 0x5830;
    (param_1 + 0x2)            = 0x1018;
    return CONCAT22(param_2, param_1);
}

void  pass1_1018_58b6(u16 *param_1)

{
    i16 iVar1;
    u16 uVar2;

    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    *param_1       = s_Alloc__s_1050_5a5b + 0x7;
    (iVar1 + 0x2)  = 0x1018;
    (iVar1 + 0xe2) = 0x5afe;
    (iVar1 + 0xe4) = 0x1018;
    pass1_1020_808e(param_1);
    return;
}

void  struct_1018_4720(u16 *param_1, u32 param_2, u32 param_3)

{
    astruct_204 *iVar1;
    u16          uVar1;

    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_204 *)param_1;
    *param_1         = 0x389a;
    iVar1->field_0x2 = 0x1008;
    iVar1->field_0x4 = param_3;
    iVar1->field_0x8 = param_2;
    iVar1->field_0xc = 0x0;
    *param_1         = &PTR_LOOP_1050_4aa6;
    iVar1->field_0x2 = 0x1018;
    return;
}
u16 * struct_1018_4790(u16 *param_1, u32 param_2, u32 param_3, u16 param_4)

{
    astruct_266 *iVar1;
    u16          uVar1;

    struct_1018_4720(param_1, param_2, param_3);
    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_266 *)param_1;
    iVar1->field_0xe = param_4;
    *param_1         = 0x4a92;
    iVar1->field_0x2 = 0x1018;
    iVar1->field_0xc = 0x1;
    return param_1;
}


void  struct_1018_47c8(u16 *param_1, u32 param_2, u32 param_3, u16 param_4, u32 param_5)

{
    astruct_264 *iVar1;
    u16          uVar1;

    struct_1018_4720(param_1, param_2, param_3);
    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_264 *)param_1;
    iVar1->field_0xe  = param_5;
    iVar1->field_0x12 = param_4;
    *param_1          = &PTR_LOOP_1050_4a9a;
    iVar1->field_0x2  = 0x1018;
    iVar1->field_0xc  = 0x2;
    return;
}


void  pass1_1018_4808(u16 *param_1, u32 param_2, u32 param_3, u32 param_4)

{
    i16 iVar1;
    u16 uVar2;

    struct_1018_4720(param_1, param_2, param_3);
    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    *(iVar1 + 0xe) = param_4;
    *param_1       = &PTR_LOOP_1050_4aa2;
    (iVar1 + 0x2)  = 0x1018;
    (iVar1 + 0xc)  = 0x3;
    return;
}


u16 * struct_1018_4842(u16 *param_1, u32 param_2, u32 param_3, u16 param_4)

{
    astruct_265 *iVar1;
    u16          uVar1;

    struct_1018_4720(param_1, param_2, param_3);
    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_265 *)param_1;
    iVar1->field_0xe  = param_4;
    iVar1->field_0x10 = 0x0;
    *param_1          = &PTR_LOOP_1050_4a8e;
    iVar1->field_0x2  = 0x1018;
    iVar1->field_0xc  = 0x4;
    return param_1;
}

u16 * struct_1018_48b0(u16 *param_1, u32 param_2, u32 param_3, u16 param_4)

{
    astruct_207 *iVar1;
    u16          uVar1;

    struct_1018_4720(param_1, param_2, param_3);
    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_207 *)param_1;
    iVar1->field_0xe = param_4;
    *param_1         = &PTR_LOOP_1050_4a96;
    iVar1->field_0x2 = 0x1018;
    iVar1->field_0xc = 0x5;
    return param_1;
}


u16 * struct_1018_48e8(u16 *param_1, u32 param_2, u32 param_3, u16 param_4)

{
    i16 iVar1;
    u16 uVar2;

    struct_1018_4720(param_1, param_2, param_3);
    uVar2         = (param_1 >> 0x10);
    iVar1         = param_1;
    (iVar1 + 0xe) = param_4;
    *param_1      = 0x4a9e;
    (iVar1 + 0x2) = 0x1018;
    (iVar1 + 0xc) = 0x6;
    return param_1;
}


void  struct_1018_4920(u16 *param_1, u32 param_2, u32 param_3, u32 param_4)

{
    astruct_203 *iVar1;
    u16          uVar1;

    struct_1018_4720(param_1, param_2, param_3);
    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_203 *)param_1;
    iVar1->field_0xe = param_4;
    *param_1         = &PTR_LOOP_1050_4a8a;
    iVar1->field_0x2 = 0x1018;
    iVar1->field_0xc = 0x7;
    return;
}
void  struct_1018_2b10(astruct_55 *param_1, u16 param_2, u16 param_3)

{
    undefined4  *puVar1;
    code       **ppcVar2;
    u16         *puVar3;
    u16          uVar4;
    i16          unaff_DI;
    u16         *puVar5;
    astruct_43  *paVar6;
    u32          uVar7;
    u16          uVar8;
    astruct_626 *uVar9;

    uVar9                       = (astruct_626 *)param_1;
    uVar8                       = (param_1 >> 0x10);
    puVar5                      = get_sys_metrics_1018_4b1e(param_1, 0x1, param_2);
    uVar9->field_0x20           = 0x389a;
    uVar9->field_0x22           = 0x1008;
    uVar9->field_0x20           = 0x3aa8;
    uVar9->field_0x22           = 0x1008;
    uVar9->field_0x24           = (astruct_76 *)0x0;
    uVar9->field_0x174          = 0x0;
    uVar9->field_0x176          = 0x0;
    uVar9->field_0x178          = 0x0;
    uVar9->field_0x17a          = 0x0;
    uVar9->field_0x17e          = 0x0;
    uVar9->field_0x182          = 0x0;
    uVar9->field_0x186          = 0x0;
    param_1->field_0x0          = 0x32d8;
    uVar9->field_0x2            = 0x1018;
    uVar9->field_0x20           = 0x3314;
    uVar9->field_0x22           = 0x1018;
    puVar5                      = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_3, (puVar5 >> 0x10), unaff_DI);
    &uVar9->field_0x182         = puVar5;
    (&uVar9->field_0x182 + 0x2) = (puVar5 >> 0x10);
    if(param_1 == (astruct_55 *)0x0)
    {
        puVar3 = 0x0;
        uVar4  = 0x0;
    }
    else
    {
        puVar3 = &uVar9->field_0x20;
        uVar4  = uVar8;
    }
    puVar1  = uVar9->field_0x182;
    ppcVar2 = (*uVar9->field_0x182 + 0x4);
    (**ppcVar2)(0x1010, puVar1, (puVar1 >> 0x10), 0x0, puVar3, uVar4);
    puVar1                     = uVar9->field_0x182;
    uVar9->field_0x17a         = (puVar1 + 0x24);
    paVar6                     = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x6e, param_3);
    &uVar9->field_0x24         = paVar6;
    (&uVar9->field_0x24 + 0x2) = (paVar6 >> 0x10);
    uVar9->field_0x28          = 0x0;
    uVar7                      = pass1_1008_4772(uVar9->field_0x24);
    uVar4                      = (uVar7 >> 0x10);
    pass1_1018_4b78(param_1, param_3);
    uVar9->field_0x16c = 0x1;
    uVar9->field_0x16e = 0x1;
    uVar9->field_0x170 = (uVar7 + 0x4) + uVar9->field_0x16c;
    uVar9->field_0x172 = (uVar7 + 0x8) + -0x19;
    return;
}

void  struct_1018_229c(astruct_632 *param_1, u8 *param_2, u16 param_3, u8 *param_4, u16 param_5)

{
    i16        *piVar1;
    astruct_43 *paVar2;
    i16         iStack4;

    struct_op_1018_4cda(param_1, param_2, param_3);
    param_1->field_0x1c        = 0x389a;
    param_1->field_0x1e        = 0x1008;
    param_1->field_0x1c        = 0x3aa8;
    param_1->field_0x1e        = 0x1008;
    param_1->field_0x20        = 0x0;
    param_1->field_0x24        = 0x0;
    param_1->field_0x26        = 0x0;
    &param_1->field_0x2a       = 0x0;
    param_1->field_0x3e        = 0x0;
    param_1->field_0x40        = 0x0;
    param_1->field_0x42        = 0x0;
    param_1->field_0x44        = 0x0;
    &param_1->field_0x6e       = 0x0;
    CONCAT22(param_2, param_1) = 0x2ada;
    param_1->field_0x2         = 0x1018;
    param_1->field_0x1c        = s_fem132_wav_1050_2aec + 0x6;
    param_1->field_0x1e        = 0x1018;
    globals->PTR_LOOP_1050_4230         = param_1;
    globals->PTR_LOOP_1050_4232         = param_2;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x105, param_4, param_5);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1a8, param_5);
    param_1->field_0x2a = paVar2;
    param_1->field_0x2c = (paVar2 >> 0x10);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0x2e), 0x0, 0x10);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0x46), 0x0, 0x28);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x6c, param_5);
    param_1->field_0x2e = paVar2;
    param_1->field_0x30 = (paVar2 >> 0x10);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x68, param_5);
    param_1->field_0x32 = paVar2;
    param_1->field_0x34 = (paVar2 >> 0x10);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x66, param_5);
    param_1->field_0x36 = paVar2;
    param_1->field_0x38 = (paVar2 >> 0x10);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x6a, param_5);
    param_1->field_0x3a = paVar2;
    param_1->field_0x3c = (paVar2 >> 0x10);
    paVar2              = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1cd, param_5);
    param_1->field_0x6e = paVar2;
    param_1->field_0x70 = (paVar2 >> 0x10);
    iStack4             = 0x0;
    do
    {
        paVar2                                 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, iStack4 + 0x8f, param_5);
        (&param_1->field_0x46 + iStack4 * 0x4) = paVar2;
        (&param_1->field_0x48 + iStack4 * 0x4) = (paVar2 >> 0x10);
        iStack4                                = iStack4 + 0x1;
    } while(iStack4 < 0xa);
    if(CONCAT22(param_2, param_1) == 0x0)
    {
        piVar1  = 0x0;
        param_2 = 0x0;
    }
    else
    {
        piVar1 = &param_1->field_0x1c;
    }
    pass1_1008_9262(_PTR_LOOP_1050_0388, (_PTR_LOOP_1050_0388 >> 0x10), 0x73, CONCAT22(param_2, piVar1), piVar1, param_2);
    return;
}


void  struct_1010_dd5e(u16 param_1, u16 param_2, u32 param_3)

{
    i16   iVar1;
    i16   iVar2;
    u16   uVar3;
    u32   uVar4;
    long *plStack16;

    if(param_3 != 0x0)
    {
        uVar4     = struct_op_1030_73a8(param_3);
        uVar3     = (uVar4 >> 0x10);
        iVar2     = uVar4;
        plStack16 = (long *)(uVar4 & 0xffff0000 | (iVar2 + 0x14U));
        if((uVar3 | iVar2 + 0x14U) != 0x0)
        {
            iVar1 = (iVar2 + 0x12);
            iVar2 = (iVar2 + 0x18);
            if(((((iVar1 == 0x4) || ((((iVar1 == 0x6 && (iVar2 == 0x4)) || (iVar1 == 0x5)) || ((iVar1 == 0x6 && (iVar2 == 0x5)))))) || (iVar1 == 0x8)) || ((iVar1 == 0x6 && (iVar2 == 0x8)))) && (*plStack16 != 0x0))
            {
                return;
            }
        }
    }
    return;
}

void  pass1_1010_e8f6(u16 param_1, u16 param_2, u32 param_3, u16 param_4)

{
    u16    uVar1;
    BOOL16 BVar2;
    u16    uVar3;
    u32    uVar4;

    uVar4 = struct_op_1030_73a8(param_3);
    uVar1 = (uVar4 + 0xc);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x13);
    if(BVar2 == 0x0)
    {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x14);
        if(BVar2 == 0x0)
        {
            return;
        }
        uVar4 = pass1_1028_4faa(uVar4, param_4);
        uVar3 = (uVar4 >> 0x10);
        uVar1 = uVar4;
    }
    else
    {
        uVar4 = pass1_1028_121e(uVar4, param_4);
        uVar3 = (uVar4 >> 0x10);
        uVar1 = uVar4;
    }
    pass1_1028_b58e(CONCAT22(uVar3, uVar1));
    return;
}

void  struct_1010_a1d8(astruct_627 *param_1, u16 param_2, u16 param_3, u16 param_4)

{
    i16         iVar1;
    code      **ppcVar2;
    i16         unaff_DI;
    astruct_79 *paVar3;
    u16        *puVar4;
    u16         uStack4;

    paVar3                        = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa            = 0x389a;
    param_1->field_0xc            = 0x1008;
    param_1->field_0xa            = 0x3aa8;
    param_1->field_0xc            = 0x1008;
    param_1->field_0x138          = 0x0;
    CONCAT22(param_2, param_1)    = 0xe9cc;
    param_1->field_0x2            = 0x1010;
    param_1->field_0xa            = 0xe9dc;
    param_1->field_0xc            = 0x1010;
    puVar4                        = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_4, (paVar3 >> 0x10), unaff_DI);
    &param_1->field_0x138         = puVar4;
    (&param_1->field_0x138 + 0x2) = (puVar4 >> 0x10);
    ppcVar2                       = (*param_1->field_0x138 + 0x4);
    (**ppcVar2)();
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0xa4), 0x0, 0x94);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0xe), 0x0, 0x96);
    uStack4 = 0x0;
    do
    {
        iVar1          = &param_1->field_0x0 + uStack4 * 0x6;
        *(iVar1 + 0xe) = pass1_1010_c7e2;
        (iVar1 + 0x12) = 0x0;
        uStack4        = uStack4 + 0x1;
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

u16  pass1_1010_a5ac(u16 param_1, u16 param_2, u32 param_3)

{
    u32 uVar1;

    uVar1 = struct_op_1030_73a8(param_3);
    return (uVar1 + 0x20);
}

u16  pass1_1010_acc0(u16 param_1, u16 param_2, u32 param_3)

{
    u32 uVar1;

    uVar1 = struct_op_1030_73a8(param_3);
    if((uVar1 + 0x12) != 0x4)
    {
        return 0x1;
    }
    return 0x0;
}

void  struct_1010_9172(u32 param_1)

{
    undefined4  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    astruct_249 *iVar4;
    u16          uVar4;
    astruct_75  *paVar5;
    undefined4   uVar6;

    uVar4  = (param_1 >> 0x10);
    iVar4  = (astruct_249 *)param_1;
    puVar1 = iVar4->field_0x4;
    uVar2  = iVar4->field_0x6;
    paVar5 = CONCAT22(uVar2, puVar1);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        paVar5  = (**ppcVar3)();
    }
    mem_op_1000_179c(0x18, (paVar5 >> 0x10), 0x1000);
    if(paVar5 == 0x0)
    {
        uVar6 = 0x0;
    }
    else
    {
        uVar6 = struct_op_1030_1cd8(paVar5, 0x5, 0x5);
    }
    iVar4->field_0x4 = uVar6;
    iVar4->field_0x6 = (uVar6 >> 0x10);
    return;
}

u16 * pass1_1010_9258(u16 *param_1)

{
    struct_1010_383a(param_1);
    *param_1        = 0x958e;
    (param_1 + 0x2) = 0x1010;
    return param_1;
}

void  struct_1010_95aa(astruct_629 *param_1, u16 param_2, u16 param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa         = 0x0;
    param_1->field_0xe         = 0x0;
    param_1->field_0x12        = 0x0;
    param_1->field_0x16        = 0x0;
    param_1->field_0x18        = 0x0;
    param_1->field_0x1a        = 0x0;
    param_1->field_0x1c        = 0xa;
    param_1->field_0x1e        = 0x0;
    CONCAT22(param_2, param_1) = 0xa1c8;
    param_1->field_0x2         = 0x1010;
    return;
}

void  struct_1010_6326(astruct_630 *param_1, u16 param_2, u16 param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa         = 0x0;
    param_1->field_0xe         = 0x0;
    param_1->field_0x12        = 0x0;
    param_1->field_0x16        = 0x0;
    param_1->field_0x1a        = 0x0;
    param_1->field_0x1e        = 0x0;
    param_1->field_0x22        = 0x0;
    CONCAT22(param_2, param_1) = 0x66f0;
    param_1->field_0x2         = 0x1010;
    return;
}

u32  pass1_1010_6700(astruct_636 *param_1, u16 param_2, u16 param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0x148       = 0x33;
    CONCAT22(param_2, param_1) = 0x6aac;
    param_1->field_0x2         = 0x1010;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0xa), 0x0, 0x114);
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
    pass1_1000_4906((astruct_20 *)CONCAT22(param_2, &param_1->field_0x11e), 0x0, 0x2a);
    param_1->field_0x120 = 0x1;
    param_1->field_0x122 = 0x1;
    param_1->field_0x124 = 0x1;
    param_1->field_0x126 = 0x1;
    param_1->field_0x128 = 0x1;
    param_1->field_0x12c = 0x1;
    param_1->field_0x138 = 0x1;
    return CONCAT22(param_2, param_1);
}


void  struct_1010_50b2(astruct_646 *param_1, u16 param_2, u16 param_3)

{
    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa         = 0x0;
    param_1->field_0xc         = 0x0;
    param_1->field_0x10        = 0x0;
    param_1->field_0x12        = 0x0;
    param_1->field_0x16        = 0x0;
    CONCAT22(param_2, param_1) = 0x53f4;
    param_1->field_0x2         = 0x1010;
    return;
}
