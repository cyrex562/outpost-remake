
void pass1_1000_29af(param_1: u16)

{
    pass1_1000_29b5(param_1 & 0xff);
    return;
}


void  pass1_1000_29b5(param_1: u16)

{
     let mut cVar1: u8;

    globals.PTR_LOOP_1050_5f88._0_1_ = (u8)param_1;
    cVar1                             = (param_1 >> 0x8);
    if(cVar1 != '\0')
        goto LAB_1000_29d2;
    if((u8)PTR_LOOP_1050_5f88 < 0x22)
    {
        if((u8)PTR_LOOP_1050_5f88 < 0x20)
        {
            if(0x13 < (u8)PTR_LOOP_1050_5f88)
                goto LAB_1000_29cc;
        }
        else
        {
            param_1 = 0x5;
        }
    }
    else
    {
    LAB_1000_29cc:
        param_1 = 0x13;
    }
    cVar1 = *((param_1 & 0xff) + 0x5fd6);
LAB_1000_29d2:
    globals.PTR_LOOP_1050_5f78 = cVar1;
    return;
}


u16 pass1_1000_29dc(param_1: u16)

{
    if(___EXPORTEDSTUB != (code)0xb8)
    {
        return param_1;
    }
    return uRam100029ed;
}


u16 ___EXPORTEDSTUB(void)

{
    return 0x0;
}


u16 *pass1_1000_2b02(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u8, param_6: u16, param_7: i16)

{
    u16 *puVar1;
    i16  iStack2;

    iStack2 = param_7 + 0x1;
    puVar1  = pass1_1000_35aa();
    if((param_6 | puVar1) == 0x0)
    {
        puVar1 = 0x0;
    }
    else
    {
        puVar1 = pass1_1000_2d34(param_1, param_2, (u8 *)CONCAT22(param_4, param_3), param_5, puVar1, &iStack2);
    }
    return puVar1;
}


void pass1_1000_2b3c(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16, param_6: i16)

{
    i16 iStack2;

    iStack2 = param_6 + 0x1;
    pass1_1000_2b02(param_1, param_2, param_3, param_4, 0x0, param_5, &iStack2);
    return;
}


void pass1_1000_2ba0(param_1: u16, param_2: u16, param_3: u16, param_4: u8)

{
    pass1_1000_3024(param_1, param_2, param_3, param_4);
    if(PTR_LOOP_1050_5fc9 != '\0')
    {
        pass1_1000_3f5c(&stack0xfffe, param_1, param_2, param_3, param_4);
    }
    return;
}


void  pass1_1000_2cb0(mut param_1: *mut u16, param_2: u16)

{
    u16 *puVar1;
     let mut bVar2: u8;

    bVar2 = *(u8 *)(param_1 + 0x5);
    if(((bVar2 & 0x83) != 0x0) && ((bVar2 & 0x8) != 0x0))
    {
        pass1_1000_16ee(param_1[0x3], param_1[0x4], param_2);
        puVar1        = param_1 + 0x5;
        *(u8 *)puVar1 = *(u8 *)puVar1 & 0xf7;
        param_1[0x3]  = 0x0;
        param_1[0x4]  = 0x0;
        *param_1      = 0x0;
        param_1[0x1]  = 0x0;
        param_1[0x2]  = 0x0;
    }
    return;
}


void pass1_1000_1f68(void)

{
    globals.PTR_LOOP_1050_5f26 = globals.PTR_LOOP_1050_5f26 + -0x1;
    if(PTR_LOOP_1050_5f26 < 0x0)
    {
        globals.PTR_LOOP_1050_5f26 = 0x0;
    }
    return;
}


char * pass1_1000_1fd2(param_1: i16)

{
    if(param_1 == 0x2)
    {
        return "Out of memory.  Please free some memory, then choose retry.";
    }
    return CONCAT22(0x1000, param_1 * 0x17 + 0x1c7a);
}


BOOL16 pass1_1000_1fea(void)

{
     let mut puVar1: *mut u8;
    bool bVar2;

    puVar1                      = globals.PTR_LOOP_1050_5f22 + 0x1;
    bVar2                       = globals.PTR_LOOP_1050_5f22 == 0x0;
    globals.PTR_LOOP_1050_5f22 = puVar1;
    if((bVar2) && ((PTR_LOOP_1050_5f20 | globals.PTR_LOOP_1050_5f1e) != 0x0))
    {
        globals.PTR_LOOP_1050_5f22 = &PTR_LOOP_1050_0002;
    }
    return 0x1;
}


BOOL16  pass1_1000_206c(param_1: u16, param_2: u16)

{
     let mut uVar1: u16;

    uVar1 = pass1_1000_21d2(0x2, 0x42, param_1, param_2, 0x1);
    if((uVar1 != 0x0) && ((param_1 + 0x14) == -0x4153))
    {
        return 0x1;
    }
    return 0x0;
}


void  pass1_1000_20a2(param_1: u16, param_2: u16)

{
    i16 iVar1;
     let mut uVar2: u16;
     let mut uVar3: u16;
     let mut uVar4: u16;
     let mut uVar5: u16;
     let mut uVar6: u16;
     let mut uVar7: u16;
     let mut uStack8: u16;
     let mut uStack4: u16;

    iVar1   = (param_1 + 0x2e);
    uVar2   = (param_1 + 0x30);
    uStack8 = 0x0;
    uVar3   = (iVar1 + 0x4);
    uStack4 = (iVar1 + 0x6);
    uVar7   = 0x0;
    if((uStack4 | uVar3) != 0x0)
    {
        while((uVar6 = uVar3, uVar4 = uStack4, uVar6 != param_1 || (uStack4 != param_2)))
        {
            uVar3   = (uVar6 + 0x2a);
            uStack4 = (uVar6 + 0x2c);
            uVar7   = uVar6;
            uStack8 = uVar4;
            if((uStack4 | uVar3) == 0x0)
            {
                return;
            }
        }
        if((uStack8 | uVar7) != 0x0)
        {
            uVar2          = (uVar6 + 0x2c);
            (uVar7 + 0x2a) = (uVar6 + 0x2a);
            (uVar7 + 0x2c) = uVar2;
            return;
        }
        uVar5         = (uVar6 + 0x2c);
        (iVar1 + 0x4) = (uVar6 + 0x2a);
        (iVar1 + 0x6) = uVar5;
    }
    return;
}


u16 ret_true_1000_2146(void)

{
    return 0x1;
}


void empty_fn_1000_214a(void)

{
    return;
}

u32 pass1_1000_2242(param_1: u16, param_2: u16, param_3: u16, param_4: i16, param_5: u16, mut param_6: *mut u8)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
    bool bVar3;

    uVar1 = param_2 | param_1;
    while(true)
    {
        if(uVar1 == 0x0)
        {
            return 0x0;
        }
        uVar1 = param_1;
        if(param_2 != 0x0)
        {
            uVar1 = 0xffff;
        }
        if(CARRY2(param_3, uVar1) != false)
        {
            uVar1 = -param_3;
        }
        bVar3   = param_1 < uVar1;
        param_1 = param_1 - uVar1;
        param_2 = param_2 - bVar3;
        uVar2   = (*(fn_ptr_1)param_6)(uVar1, param_5, param_3, param_4);
        if(uVar2 != 0x0)
            break;
        bVar3   = CARRY2(param_3, uVar1);
        param_3 = param_3 + uVar1;
        param_4 = param_4 + bVar3 * 0x100;
        uVar1   = param_2 | param_1;
    }
    return CONCAT22(param_2 + CARRY2(uVar2, param_1), uVar2 + param_1);
}


BOOL16 pass1_1000_22c0(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16)

{
    u32 uVar1;

    uVar1 = pass1_1000_2242(param_3, param_4, param_1, param_2, param_5, 0x1dfa);
    if(uVar1 == 0x0)
    {
        return 0x1;
    }
    return 0x0;
}

void pass1_1000_25a8(param_1: u16, param_2: u16)

{
    pass1_1000_2913(0xfc, param_1, param_2);
    pass1_1000_2913(0xff, param_1, param_2);
    return;
}


BOOL16  pass1_1000_115c(param_1: i16, mut param_2: *mut u16)

{
     let mut pbVar1: *mut u8;
    u16 *puVar2;
     let mut uVar3: u16;
     let mut uVar4: u16;
    u16 *puVar5;
    i16  iVar6;
     let mut uStack4: u16;

    uVar3 = *param_2 & 0x7ffc;
    uVar4 = param_1 + 0x5U & 0xfffc;
    uVar4 = (uVar4 - 0x8 & ~-(uVar4 < 0x8)) + 0x8;
    if(uVar3 < uVar4)
    {
        puVar5 = (uVar3 + param_2);
        if(((*(u8 *)puVar5 & 0x1) != 0x0) || ((*puVar5 & 0xfffc) + uVar3 < uVar4))
        {
            return 0x0;
        }
        if(puVar5 == PTR_LOOP_1050_000e)
        {
            globals.PTR_LOOP_1050_000e = puVar5[0x1];
        }
        (puVar5[0x2] + 0x2) = puVar5[0x1];
        (puVar5[0x1] + 0x4) = puVar5[0x2];
        uStack4             = ((*puVar5 & 0xfffc) + uVar3) - uVar4;
        if(uStack4 < s_version__d__d_1050_0012._0_2_)
        {
            puVar2  = param_2;
            *puVar2 = *puVar2 + (*puVar5 & 0xfffc);
            pbVar1  = (u8 *)(puVar5 + (*puVar5 & 0xfffc));
            *pbVar1 = *pbVar1 | 0x2;
            return 0x1;
        }
    }
    else
    {
        uStack4 = uVar3 - uVar4;
        if(uStack4 < s_version__d__d_1050_0012._0_2_)
        {
            return 0x1;
        }
        puVar5 = (uVar3 + param_2);
        if((*(u8 *)puVar5 & 0x1) == 0x0)
        {
            uStack4 = uStack4 + (*puVar5 & 0xfffc);
            if(puVar5 == PTR_LOOP_1050_000e)
            {
                globals.PTR_LOOP_1050_000e = puVar5[0x1];
            }
            (puVar5[0x2] + 0x2) = puVar5[0x1];
            (puVar5[0x1] + 0x4) = puVar5[0x2];
        }
        if(*DAT_1050_0004 < uStack4)
        {
            *DAT_1050_0004 = uStack4;
        }
    }
    *param_2                           = *param_2 & 0x8003 | uVar4;
    (uVar4 + param_2)                  = uStack4 | 0x2;
    iVar6                              = uVar4 + param_2;
    (iVar6 + 0x4)                      = globals.PTR_LOOP_1050_0010;
    (iVar6 + 0x2)                      = (PTR_LOOP_1050_0010 + 0x2);
    ((PTR_LOOP_1050_0010 + 0x2) + 0x4) = iVar6;
    (PTR_LOOP_1050_0010 + 0x2)         = iVar6;
    ((u8 *)(iVar6 + uStack4) + -0x2)   = uStack4;
    pbVar1                             = (u8 *)(iVar6 + uStack4);
    *pbVar1                            = *pbVar1 & 0xfd;
    return 0x1;
}


u8 *pass1_1000_17e8(mut param_1: *mut u8, mut param_2: *mut u8)

{
     let mut puVar1: *mut u8;

    puVar1                      = globals.PTR_LOOP_1050_5f34;
    globals.PTR_LOOP_1050_5f34 = param_1;
    globals.PTR_LOOP_1050_5f36 = param_2;
    return puVar1;
}


u16 pass1_1000_188e(mut param_1: *mut u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16, param_6: u16)

{
     let mut uVar1: u16;
    long lVar2;

    if((param_2 | param_1) == 0x0)
    {
        uVar1 = pass1_1000_180c(param_3, param_4, param_5);
        return uVar1;
    }
    if(param_3 == 0x0)
    {
        pass1_1000_18d2(param_1, param_2, param_5);
        return 0x0;
    }
    lVar2 = pass1_1000_0ed4(param_5, param_6, 0x0, param_3, 0x0, param_1, param_2);
    return lVar2;
}


u16  pass1_1000_1a54(param_1: u16, param_2: i16, param_3: u16, param_4: u16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;

    if((param_2 + 0x14) != -0x4153)
    {
        pass1_1000_1e61(param_4, 0xa, 0x0, 0x0);
        return 0x0;
    }
    uVar1 = pass1_1000_1ab0(param_1);
    if(uVar1 < (param_2 + 0x18) + 0x14U)
    {
        uVar2 = 0x0;
    }
    else
    {
        uVar2            = (param_2 + 0x1a);
        (param_2 + 0x1a) = uVar1;
        (param_2 + 0x1c) = uVar1 >> 0x2;
    }
    return uVar2;
}


u16  pass1_1000_1ab0(param_1: u16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;

    if(param_1 == 0x2000)
    {
        return 0x2000;
    }
    if(param_1 < 0xfff0)
    {
        if(param_1 < 0x1001)
        {
            return 0x1000;
        }
        uVar1 = 0x2000;
        if(param_1 < 0x2001)
        {
            do
            {
                uVar2 = uVar1;
                uVar1 = uVar2 >> 0x1;
            } while(param_1 <= uVar1);
            return uVar2 & 0xfffe;
        }
        while(uVar1 = uVar1 * 0x2, uVar1 != 0x0)
        {
            if(param_1 <= uVar1)
            {
                return (uVar1 + 0x10 & -(uVar1 < 0xfff0)) - 0x10;
            }
        }
    }
    return 0xfff0;
}


BOOL16  pass1_1000_1afe(param_1: u16, u1632 param_2, param_3: u16)

{
     let mut uVar1: u16;
     let mut unaff_CS: u16;

    if(param_1 == 0x0)
    {
        uVar1 = 0x0;
    }
    else
    {
        uVar1 = param_1 + 0x1 & 0xfffe;
    }
    if((param_2 + 0x14) == -0x4153)
    {
        if((uVar1 < param_1) || ((param_2 + 0x1a) - 0x14U < uVar1))
        {
            pass1_1000_1e61(unaff_CS, 0x3, param_2, param_3);
        }
        else
        {
            if((param_2 + 0x2) == 0x0)
            {
                (param_2 + 0x18) = uVar1;
                return 0x1;
            }
        }
        return 0x0;
    }
    pass1_1000_1e61(unaff_CS, 0xa, 0x0, 0x0);
    return 0x0;
}
void  pass1_1000_0000(mut param_1: *mut u16, mut param_2: *mut u16, param_3: u16)

{
    u16 *puVar1;
    u16 *puVar2;
     let mut uVar3: u16;

    // Segment:    1
    // Offset:     00000a20
    // Length:     55b7
    // Min Alloc:  55b7
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    for(uVar3 = param_3 >> 0x1; uVar3 != 0x0; uVar3 = uVar3 - 0x1)
    {
        puVar2  = param_2;
        param_2 = param_2 + 0x1;
        puVar1  = param_1;
        param_1 = param_1 + 0x1;
        *puVar2 = *puVar1;
    }
    return;
}


void  pass1_1000_0368(param_1: u16, param_2: u16, param_3: u16)

{
    u16 *puVar1;

    if((param_1 + 0x4) == param_1)
    {
        (param_3 + param_2 * 0x2) = 0x0;
    }
    else
    {
        ((param_1 + 0x6) + 0x4) = (param_1 + 0x4);
        ((param_1 + 0x4) + 0x6) = (param_1 + 0x6);
        puVar1                  = (param_2 * 0x2 + param_3);
        if(*puVar1 == param_1)
        {
            *puVar1 = (param_1 + 0x4);
        }
    }
    (param_1 + 0x4) = (param_3 + 0xa);
    (param_3 + 0xa) = param_1;
    return;
}

void  pass1_1000_05b4(param_1: u8, param_2: i16)

{
    (param_2 + 0xa)         = 0x1;
    (param_2 + 0x8)         = 0x668;
    *(u8 *)(param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
    (param_2 + 0x10)        = 0x0;
    (param_2 + 0xe)         = 0x0;
    return;
}


u16 pass1_1000_0782(u1632 param_1, param_2: u16, param_3: i16)

{
     let mut in_stack_00000004: u16;

    (param_3 + 0xe)  = 0x0;
    (param_3 + 0x10) = param_3 + 0x14;
    (param_3 + 0x8)  = 0x9a0;
    pass1_1000_07ac((param_1 + 0x18), param_2, param_3);
    return 0x1;
}


void  pass1_1000_07ac(param_1: u16, param_2: i16, param_3: i16)

{
    u16 *puVar1;
    i16  iVar2;
     let mut uVar3: u16;

    puVar1                   = *(u16 **)(param_3 + 0x10);
    *(u16 **)(param_3 + 0xe) = puVar1;
    uVar3                    = param_2 + (param_3 - puVar1);
    iVar2                    = puVar1 + (uVar3 - uVar3 % param_1);
    (param_3 + 0x10)         = iVar2;
    while(puVar1 < (iVar2 - param_1))
    {
        *puVar1 = (puVar1 + param_1);
        puVar1  = (puVar1 + param_1);
    }
    *puVar1 = 0x0;
    return;
}

u16  pass1_1000_09ca(param_1: i16, u1632 *param_2)

{
    u16       *puVar1;
    i16        iVar2;
    u32 uVar3;
    u16       *puVar4;

    puVar1       = param_2 + 0xa;
    puVar4       = ((param_2 + (param_1 - puVar1) + -0x6 & 0xfffcU) + puVar1);
    *puVar4      = 0x1;
    param_2[0x7] = (u1632)puVar1;
    puVar4[0x2]  = puVar4;
    puVar4[0x1]  = puVar4;
    param_2[0x8] = (u1632)puVar4;
    if((*(u8 *)(param_2 + 0x6) & 0x7) == 0x2)
    {
        param_2[0x9] = 0x8;
    }
    else
    {
        uVar3        = param_2;
        iVar2        = (uVar3 + 0x18);
        param_2[0x9] = (iVar2 - 0x5U & ~-(iVar2 + 0x3U < 0x8)) + 0x8;
    }
    puVar4[-0x1]                 = puVar4 - puVar1;
    *puVar1                      = puVar4 - puVar1 | 0x2;
    param_2[0xc]                 = (u1632)puVar4;
    param_2[0xb]                 = puVar4[0x1];
    *(u16 **)(puVar4[0x1] + 0x4) = puVar1;
    puVar4[0x1]                  = puVar1;
    param_2[0x4]                 = 0xe08;
    return *puVar1 & 0xfffc;
}


u32  pass1_1000_0c32(param_1: u16, param_2: u16, param_3: u16)

{
    u16       *puVar1;
     let mut pbVar2: *mut u8;
     let mut piVar3: *mut i16;
    u32 uVar4;
     let mut uVar5: u16;
    u16       *puVar6;
    i16        iVar7;
    u16       *puVar8;
     let mut uVar9: u16;
     let mut uStack14: u16;
    u16       *puStack8;
     let mut uStack6: u16;

    puVar8  = *(u16 **)(param_3 + 0xe);
    uStack6 = 0x0;
    puVar6  = puVar8;
    while(true)
    {
        do
        {
            uVar5 = *puVar6;
            if(param_1 <= uVar5)
            {
                uVar5    = (uVar5 & 0xfffc) - param_1;
                puVar1   = (param_3 + 0x12);
                puStack8 = puVar6;
                if(*puVar1 < uVar5 || *puVar1 == uVar5)
                {
                    uStack14 = param_1;
                    if((param_2 & 0x6) == 0x0)
                    {
                        puStack8       = (uVar5 + puVar6);
                        puStack8[-0x1] = uVar5;
                        *puVar6        = uVar5 | 0x2;
                        puVar8         = puVar6[0x1];
                        pbVar2         = (u8 *)(puStack8 + param_1);
                        *pbVar2        = *pbVar2 | 0x2;
                        *puStack8      = param_1 | 0x1;
                    }
                    else
                    {
                        *puVar6                        = param_1 & 0xff00 | *(u8 *)puVar6 & 0x2 | param_1 & 0xff | 0x1;
                        (puVar6[0x2] + 0x2)            = puVar6[0x1];
                        (puVar6[0x1] + 0x4)            = puVar6[0x2];
                        puVar8                         = (puVar6 + param_1);
                        (puVar8 + (uVar5 - 0x2))       = uVar5;
                        *puVar8                        = uVar5 | 0x2;
                        uVar5                          = (param_3 + 0x10);
                        puVar8[0x2]                    = uVar5;
                        puVar8[0x1]                    = (uVar5 + 0x2);
                        *(u16 **)((uVar5 + 0x2) + 0x4) = puVar8;
                        *(u16 **)(uVar5 + 0x2)         = puVar8;
                    }
                }
                else
                {
                    puVar8                       = puVar6[0x1];
                    *(u16 **)(puVar6[0x2] + 0x2) = puVar8;
                    (puVar6[0x1] + 0x4)          = puVar6[0x2];
                    puVar1                       = puVar6;
                    *(u8 *)puVar1                = *(u8 *)puVar1 | 0x1;
                    uStack14                     = *puVar6 & 0xfffc;
                    (puVar6 + uStack14)          = (puVar6 + uStack14) | 0x2;
                }
                *(u16 **)(param_3 + 0xe) = puVar8;
                if((param_2 & 0x1) != 0x0)
                {
                    puVar6 = puStack8;
                    for(uVar5 = uStack14 - 0x2 >> 0x1; puVar6 = puVar6 + 0x1, uVar5 != 0x0; uVar5 = uVar5 - 0x1)
                    {
                        *puVar6 = 0x0;
                    }
                    if((uStack14 - 0x2 & 0x1) != 0x0)
                    {
                        *puVar6 = 0x0;
                    }
                }
                if(((param_2 & 0x2) != 0x0) && (puVar8[0x1] == puVar8[0x2]))
                {
                    **(u16 **)(param_3 + 0x4) = **(u16 **)((param_3 + 0x10) + 0x2) & 0xfffc;
                    uVar4                     = (param_3 + 0x4);
                    pbVar2                    = (u8 *)(uVar4 + 0x3);
                    *pbVar2                   = *pbVar2 | 0x80;
                }
                piVar3  = (param_3 + 0xa);
                *piVar3 = *piVar3 + 0x1;
                return CONCAT22(0x1050, puStack8 + 0x1);
            }
            if(uStack6 < uVar5)
            {
                uStack6 = uVar5;
            }
            puVar6 = puVar6[0x1];
        } while(puVar6 != puVar8);
        if(((param_2 & 0x2) == 0x0) || ((param_2 & 0x40) != 0x0))
            break;
        uVar4 = param_3;
        uVar9 = (uVar4 >> 0x10);
        iVar7 = uVar4;
        if((iVar7 + 0x34) == 0x0)
            break;
        uStack6 = (**(iVar7 + 0x34))();
        if((uStack6 < param_1) || (puVar6 = *(u16 **)(param_3 + 0xe), puVar6 == 0x0))
            break;
    }
    **(u16 **)(param_3 + 0x4) = uStack6 & 0xfffc;
    return 0x0;
}
