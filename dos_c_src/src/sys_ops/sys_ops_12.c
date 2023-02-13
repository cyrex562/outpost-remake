#include "sys_ops_12.h"

#include "outpost_int.h"
#include "win_types.h"

void pass1_1000_27d6(u16 *param_1)

{
    i16   *piVar1;
    char  *pcVar2;
    u16   *puVar3;
    i16   *piVar4;
    char   cVar5;
    SEGPTR SVar6;
    u16   *puVar7;
    u16  **ppuVar8;
    i16    iVar9;
    u16    uVar10;
    u16   *puVar11;
    i16    iVar12;
    i16   *piVar13;
    i16   *piVar14;
    char  *pcVar15;
    i16   *piVar16;
    bool   bVar17;
    u16   *puVar18;

    SVar6 = GetDOSEnvironment16();
    if(SVar6 != 0x0)
    {
        param_1 = 0x0;
    }
    iVar12  = 0x0;
    pcVar15 = 0x0;
    iVar9   = -0x1;
    if(param_1 != 0x0)
    {
        cVar5 = *0x0;
        while(cVar5 != '\0')
        {
            do
            {
                if(iVar9 == 0x0)
                    break;
                iVar9   = iVar9 + -0x1;
                pcVar2  = pcVar15;
                pcVar15 = pcVar15 + 0x1;
            } while(*pcVar2 != '\0');
            iVar12  = iVar12 + 0x1;
            pcVar2  = pcVar15;
            pcVar15 = pcVar15 + 0x1;
            cVar5   = *pcVar2;
        }
    }
    uVar10                      = 0x9;
    puVar11                     = param_1;
    puVar7                      = pass1_1000_2950(0x9, param_1, param_1, s_tile2_bmp_1050_1538);
    puVar18                     = puVar11;
    ppuVar8                     = (u16 **)pass1_1000_2950(uVar10, puVar11, param_1, s_tile2_bmp_1050_1538);
    piVar13                     = 0x0;
    globals->PTR_LOOP_1050_5fbe = ppuVar8;
    globals->PTR_LOOP_1050_5fc0 = puVar11;
    do
    {
        if(iVar12 == 0x0)
        {
            *ppuVar8     = 0x0;
            ppuVar8[0x1] = 0x0;
            return;
        }
        bVar17 = *piVar13 == s__C_FILE_INFO__1050_5f5c._0_2_;
        if(bVar17)
        {
            piVar16 = s__C_FILE_INFO__1050_5f5c;
            iVar9   = 0x6;
            piVar14 = piVar13;
            do
            {
                if(iVar9 == 0x0)
                    break;
                iVar9   = iVar9 + -0x1;
                piVar4  = piVar16;
                piVar16 = piVar16 + 0x1;
                piVar1  = piVar14;
                piVar14 = piVar14 + 0x1;
                bVar17  = *piVar1 == *piVar4;
            } while(bVar17);
            if(!bVar17)
                goto LAB_1000_2867;
        }
        else
        {
        LAB_1000_2867:
            *ppuVar8     = puVar7;
            ppuVar8[0x1] = puVar18;
            ppuVar8      = ppuVar8 + 0x2;
        }
        do
        {
            piVar1  = piVar13;
            piVar13 = (piVar13 + 0x1);
            cVar5   = *piVar1;
            puVar3  = puVar7;
            puVar7  = (puVar7 + 0x1);
            *puVar3 = cVar5;
        } while(cVar5 != '\0');
        iVar12 = iVar12 + -0x1;
    } while(true);
}


u16 *pass1_1000_2950(i16 param_1, u16 param_2, u16 param_3, u16 param_4)

{
    u16   *puVar1;
    u16    uVar2;
    char  *pcVar3;
    u8    *puVar4;
    LPCSTR str;
    i16    iVar5;
    u16   *puVar6;
    u16    in_AX;
    u16   *puVar7;
    char  *pcVar8;
    u16    uVar9;

    puVar4                      = globals->PTR_LOOP_1050_6066;
    globals->PTR_LOOP_1050_6066 = &globals->PTR_LOOP_1050_1000;
    uVar9                       = param_3;
    puVar7                      = mem_1000_167a(in_AX, param_4, param_2);
    globals->PTR_LOOP_1050_6066 = puVar4;
    if((param_2 | puVar7) != 0x0)
    {
        return puVar7;
    }
    iVar5 = param_1;
    pass1_1000_25a8(param_3, param_4);
    pass1_1000_2913(param_1, param_3, param_4);
    str = poss_str_op_1000_28dc(iVar5);
    if(str != (PCHAR)0x0)
    {
        iVar5 = 0x9;
        if(*str == 'M')
        {
            iVar5 = 0xf;
        }
        str    = str + iVar5;
        iVar5  = 0x22;
        pcVar8 = str;
        do
        {
            if(iVar5 == 0x0)
                break;
            iVar5  = iVar5 + -0x1;
            pcVar3 = pcVar8;
            pcVar8 = pcVar8 + 0x1;
        } while(*pcVar3 != '\r');
        pcVar8[-0x1] = '\0';
    }
    FatalAppExit16(param_4, str);
    FatalExit();
    puVar7 = &globals->PTR_LOOP_1050_63fe;
    do
    {
        puVar1 = puVar7;
        puVar7 = puVar7 + 0x1;
        uVar2  = *puVar1;
        puVar6 = puVar7;
        if((uVar2 == uVar9) || (puVar6 = (uVar2 + 0x1), puVar6 == 0x0))
        {
            return puVar6;
        }
        iVar5 = -0x1;
        do
        {
            if(iVar5 == 0x0)
                break;
            iVar5  = iVar5 + -0x1;
            puVar1 = puVar7;
            puVar7 = (puVar7 + 0x1);
        } while(*puVar1 != '\0');
    } while(true);
}


u16 pass1_1000_2a00(u16 *param_1, i16 param_2, u16 param_3, u16 param_4, u16 param_5, u8 param_6)

{
    bool bVar1;
    i16 *piVar2;
    i16  iVar3;
    u16  uVar4;
    u16  uVar5;
    u8  *puStack20;
    char local_10;
    u8   uStack15;
    u8   local_e[0x8];
    u16  uStack6;
    u16  local_4;
    i16  iStack2;

    iStack2 = param_2 + 0x1;
    local_4 = SUB42(&USHORT_1050_1050, 0x0);
    uVar5   = 0xffff;
    if((*(u8 *)(param_1 + 0x5) & 0x40) != 0x0)
    {
        *(param_1 + 0x5) = 0x0;
        return 0xffff;
    }
    if((*(u8 *)(param_1 + 0x5) & 0x83) == 0x0)
        goto LAB_1000_2af2;
    uVar5   = pass1_1000_2fa4(param_1, param_3, param_4, param_5, param_6);
    uStack6 = param_1[0x7a];
    pass1_1000_2cb0(param_1, param_4);
    if(DAT_1050_5f8a < *(u8 *)(param_1 + 0xb))
    {
        piVar2 = pass1_1000_55b1(0x2a63, param_3, param_4);
        if(piVar2 < 0x0)
            goto LAB_1000_2a6a;
    LAB_1000_2a82:
        bVar1 = false;
    }
    else
    {
        iVar3 = dos3_call_op_1000_35fe(*(u8 *)(param_1 + 0xb), &iStack2);
        if(-0x1 < iVar3)
            goto LAB_1000_2a82;
    LAB_1000_2a6a:
        bVar1 = true;
    }
    if(!bVar1)
    {
        if(uStack6 == 0x0)
            goto LAB_1000_2af2;
        unk_str_op_1000_3d3e(CONCAT22(param_5, &local_10), 0x10505fea);
        puStack20 = local_e;
        if(local_10 == '\\')
        {
            puStack20 = &uStack15;
        }
        else
        {
            pass1_1000_3cea(CONCAT22(param_5, &local_10), 0x10505fec);
        }
        pass1_1000_3e82(uStack6, CONCAT22(param_5, puStack20), 0xa);
        uVar4 = dos3_call_1000_514e(&iStack2);
        if(uVar4 == 0x0)
            goto LAB_1000_2af2;
    }
    uVar5 = 0xffff;
LAB_1000_2af2:
    *(param_1 + 0x5) = 0x0;
    return uVar5;
}


u16 pass1_1000_2b5c(u16 param_1, u16 param_2, u16 param_3, u16 param_4, u16 param_5, i16 param_6, u16 param_7, u16 param_8)

{
    u16 uVar1;
    u16 uVar2;
    u8  in_AF;
    i16 iStack2;

    iStack2 = param_6 + 0x1;
    uVar1   = pass1_1000_2e74((u16 *)param_1, param_7);
    uVar2   = sys_1000_30b4(param_1, &USHORT_1050_1050, (u8 *)CONCAT22(param_4, param_3), &iStack2, param_1, param_5, param_7, param_8);
    pass1_1000_2f00(uVar1, param_1, param_5, param_7, param_8, in_AF);
    return uVar2;
}


u16 mem_1000_2bb6(u16 param_1, i16 *param_2, i16 param_3, u16 param_4, u16 param_5, u16 param_6, u8 param_7, u16 param_8)

{
    i16 *piVar1;
    i16  iVar2;
    i16 *piVar3;
    u8   bVar4;
    u8  *puVar5;
    u8  *puVar6;
    u8  *puVar7;
    i16  iStack2;

    piVar3  = param_2;
    iStack2 = param_3 + 0x1;
    bVar4   = *(u8 *)(param_2 + 0x5);
    if(((bVar4 & 0x82) != 0x0) && ((bVar4 & 0x40) == 0x0))
    {
        param_2[0x2] = 0x0;
        if((bVar4 & 0x1) != 0x0)
        {
            if((bVar4 & 0x10) == 0x0)
                goto LAB_1000_2c37;
            *param_2 = param_2[0x3];
            bVar4    = bVar4 & 0xfe;
        }
        *(u8 *)(param_2 + 0x5) = bVar4 & 0xef | 0x2;
        puVar7                 = *(u8 *)(param_2 + 0xb);
        if(((bVar4 & 0x8) == 0x0)
           && (((bVar4 & 0x4) != 0x0
                || (((*(u8 *)(param_2 + 0x78) & 0x1) == 0x0
                     && (((PTR_LOOP_1050_61ec != 0x0 && (((param_2 == 0x621c || (param_2 == 0x6228)) && ((puVar7[0x5f90] & 0x40) != 0x0)))) || (mem_1000_2ce8(param_2, param_8, param_5), (*(u8 *)(piVar3 + 0x5) & 0x8) == 0x0))))))))
        {
            puVar5 = mixed_dos3_call_1000_39f2(puVar7, CONCAT22(param_6, &param_1), (&PTR_LOOP_1050_0000 + 0x1), param_4, param_5, param_6, param_7);
            puVar6 = (&PTR_LOOP_1050_0000 + 0x1);
        }
        else
        {
            iVar2       = piVar3[0x3];
            puVar6      = (*piVar3 - iVar2);
            *piVar3     = iVar2 + 0x1;
            piVar3[0x2] = piVar3[0x79] + -0x1;
            if(puVar6 == 0x0)
            {
                puVar5 = 0x0;
                if((puVar7[0x5f90] & 0x20) != 0x0)
                {
                    mixed_dos3_call_1000_3636(puVar7, 0x0, 0x0, 0x2, &iStack2);
                    puVar5 = 0x0;
                    puVar6 = puVar5;
                }
            }
            else
            {
                puVar5 = mixed_dos3_call_1000_39f2(puVar7, CONCAT22(piVar3[0x4], piVar3[0x3]), puVar6, param_4, param_5, param_6, param_7);
            }
            *(piVar3 + 0x3) = param_1;
        }
        if(puVar5 == puVar6)
        {
            return param_1 & 0xff;
        }
    }
LAB_1000_2c37:
    piVar1        = piVar3 + 0x5;
    *(u8 *)piVar1 = *(u8 *)piVar1 | 0x20;
    return 0xffff;
}


void mem_1000_2ce8(i16 *param_1, u16 param_2, u16 param_3)

{
    i16 *piVar1;
    u16  uVar2;

    uVar2 = mem_1000_167a(0x200, param_3, param_2);
    if(param_2 == 0x0)
    {
        piVar1        = param_1 + 0x5;
        *(u8 *)piVar1 = *(u8 *)piVar1 | 0x4;
        param_1[0x79] = 0x1;
        param_2       = &USHORT_1050_1050;
        uVar2         = param_1 + 0xf1;
    }
    else
    {
        piVar1        = param_1 + 0x5;
        *(u8 *)piVar1 = *(u8 *)piVar1 | 0x8;
        param_1[0x79] = 0x200;
    }
    param_1[0x1] = param_2;
    *param_1     = uVar2;
    param_1[0x4] = param_2;
    param_1[0x3] = uVar2;
    param_1[0x2] = 0x0;
    return;
}


u16 *pass1_1000_2d34(u16 param_1, u16 param_2, u8 *param_3, u8 param_4, u16 *param_5, i16 param_6)

{
    u8   bVar1;
    bool bVar2;
    bool bVar3;
    u16  uVar4;
    u8   uStack14;
    u8   bStack8;
    u8   uStack6;
    i16  iStack2;

    iStack2 = param_6 + 0x1;
    bStack8 = (u8)PTR_LOOP_1050_6062;
    bVar3   = false;
    bVar1   = *param_3;
    if(bVar1 == 0x77)
    {
        uVar4 = 0x301;
    }
    else
    {
        if(0x77 < bVar1)
        {
            return 0x0;
        }
        if(bVar1 != 0x61)
        {
            if(bVar1 != 0x72)
            {
                return 0x0;
            }
            uVar4   = 0x0;
            uStack6 = 0x1;
            goto LAB_1000_2d6c;
        }
        uVar4 = 0x109;
    }
    uStack6 = 0x2;
LAB_1000_2d6c:
    bVar2 = true;
LAB_1000_2d71:
    param_3 = (u8 *)(param_3 & 0xffff0000 | (param_3 + 0x1));
    if((*param_3 == 0x0) || (!bVar2))
    {
        uVar4 = mixed_dos3_call_1000_370a(param_1, param_2, uVar4, param_4, 0x1a4, &iStack2);
        if(uVar4 < 0x0)
        {
            return 0x0;
        }
        globals->PTR_LOOP_1050_5fee = globals->PTR_LOOP_1050_5fee + 0x1;
        *(param_5 + 0x5)            = uStack6;
        param_5[0x1]                = 0x0;
        *param_5                    = 0x0;
        param_5[0x4]                = 0x0;
        param_5[0x3]                = 0x0;
        uStack14                    = (undefined)uVar4;
        *(param_5 + 0xb)            = uStack14;
        *(u8 *)(param_5 + 0x78)     = bStack8;
        param_5[0x2]                = 0x0;
        param_5[0x7a]               = 0x0;
        return param_5;
    }
    bVar1 = *param_3;
    if(bVar1 == 0x74)
    {
        if((uVar4 & 0xc000) == 0x0)
        {
            uVar4 = uVar4 | 0x4000;
            goto LAB_1000_2d71;
        }
    }
    else
    {
        if(0x74 < bVar1)
            goto LAB_1000_2da4;
        if(bVar1 == 0x2b)
        {
            if((uVar4 & 0x2) != 0x0)
                goto LAB_1000_2da4;
            uVar4   = uVar4 & 0xfffe | 0x2;
            uStack6 = 0x80;
            goto LAB_1000_2d71;
        }
        if(bVar1 == 0x62)
        {
            if((uVar4 & 0xc000) == 0x0)
            {
                uVar4 = uVar4 | 0x8000;
                goto LAB_1000_2d71;
            }
        }
        else
        {
            if(bVar1 != 0x63)
            {
                if((bVar1 != 0x6e) || (bVar3))
                    goto LAB_1000_2da4;
                bVar3   = true;
                bStack8 = bStack8 & 0xbf;
                goto LAB_1000_2d71;
            }
            if(!bVar3)
            {
                bVar3   = true;
                bStack8 = bStack8 | 0x40;
                goto LAB_1000_2d71;
            }
        }
    }
LAB_1000_2da4:
    bVar2 = false;
    goto LAB_1000_2d71;
}
u32  mem_op_1000_1b68(u16 param_1, u16 param_2, u16 u16_addr_lo, u16 u16_addr_hi)

{
    u32 uVar1;

    if((u16_addr_lo + 0x14) != -0x4153)
    {
        pass1_1000_1e61(param_2, 0xa, 0x0, 0x0);
        return param_1 << 0x10;
    }
    uVar1 = mem_op_1000_1b9a(0x0, u16_addr_lo, u16_addr_hi, param_2);
    return uVar1;
}


u32  mem_op_1000_1b9a(u16 param_1, u1632 param_2, u16 param_3, u16 param_4)

{
    u16        uVar1;
    u32 uVar2;
    u16        uVar3;
    u16        uVar4;
    i16        iVar5;
    long       lVar6;
    u16       *puStack8;
    u16        uStack4;

    (param_2 + 0x14) = 0x0;
    uStack4          = 0x0;
    do
    {
        iVar5 = (uStack4 * 0x2);
        if(iVar5 != 0x0)
        {
            do
            {
                uVar2         = (iVar5 + 0x8);
                (uVar2 + 0xc) = 0x0;
                mem_op_1000_13ce(param_4);
                iVar5 = (iVar5 + 0x4);
            } while((uStack4 * 0x2) != iVar5);
        }
        uStack4 = uStack4 + 0x1;
    } while(uStack4 < 0x5);
    uVar4 = (param_2 + 0x12);
    uVar3 = (param_2 + 0x10);
    while(true)
    {
        puStack8 = CONCAT22(uVar4, uVar3);
        if((uVar4 | uVar3) == 0x0)
            break;
        uVar1 = *puStack8;
        uVar4 = (uVar3 + 0x2);
        mem_op_1000_13ce(param_4);
        uVar3 = uVar1;
    }
    pass1_1000_20a2(param_2, param_3);
    lVar6 = mem_op_1000_13ce(param_4);
    return CONCAT22((lVar6 >> 0x10), 0x1);
}


BOOL16 mem_op_1000_1dfa(i16 param_1, u8 param_2, u16 param_3, u16 param_4)

{
    undefined3 uVar1;
    u16        uVar2;

    if((param_2 & 0x4) == 0x0)
    {
        uVar2 = (u8)(((u8)(-((param_2 & 0x2) == 0x0) >> 0x8) & 0xfe) + 0x92) << 0x8;
    }
    else
    {
        uVar2 = 0x1800;
    }
    if((param_4 == 0x0) || ((param_4 & 0xff00 & (u8)(((u8)(-((param_2 & 0x4) == 0x0) >> 0x8) & 0x82) + 0x18) << 0x8) != uVar2))
    {
        return 0x1;
    }
    if(param_1 != 0x0)
    {
        uVar1 = SegmentLimit(param_4);
        if(CARRY2(param_3, param_1 - 0x1U))
        {
            return 0x1;
        }
        if(uVar1 < param_3 + (param_1 - 0x1U))
        {
            return 0x1;
        }
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1000_1e61(u16 param_1, u16 param_2, u16 param_3, u16 param_4)

{
    i16    iVar1;
    BOOL16 BVar2;
    u16    UVar3;
    u16    UStack64;
    u16    UStack62;
    u16    UStack60;
    code  *pcStack6;
    u8    *puStack4;
    u16    uVar3;

    uVar3    = &USHORT_1050_1050;
    UStack62 = param_3;
    UStack60 = param_4;
    UStack64 = param_2;
    puStack4 = &USHORT_1050_1050;
    pcStack6 = (fn_ptr_1)&PTR_PTR_1050_5f1a;
    if((PTR_LOOP_1050_5f1c | globals->PTR_PTR_1050_5f1a) == 0x0)
    {
        pcStack6 = (fn_ptr_1)0x0;
        puStack4 = 0x0;
    }
    else
    {
        iVar1    = mem_op_1000_21b6(PTR_PTR_1050_5f1a, globals->PTR_LOOP_1050_5f1c);
        pcStack6 = (fn_ptr_1)PTR_PTR_1050_5f1a;
        puStack4 = globals->PTR_LOOP_1050_5f1c;
        if(iVar1 == 0x0)
        {
            globals->PTR_PTR_1050_5f1a  = &PTR_PTR_1050_1f7e;
            globals->PTR_LOOP_1050_5f1c = &globals->PTR_LOOP_1050_1000;
            pcStack6                    = (fn_ptr_1)&PTR_PTR_1050_1f7e;
            puStack4                    = &globals->PTR_LOOP_1050_1000;
        }
    }
    if((puStack4 | pcStack6) == 0x0)
    {
        return 0x0;
    }
    BVar2 = msg_box_op_1000_1f24(&PTR_PTR_1050_5f1a, &USHORT_1050_1050, 0x0, 0x1000);
    if(BVar2 == 0x0)
    {
        UVar3 = (*pcStack6)(0x1000, &UStack64);
    }
    else
    {
        puStack4 = 0x0;
        pcStack6 = (fn_ptr_1)0x0;
        UVar3    = 0x0;
    }
    if((puStack4 | pcStack6) != 0x0)
    {
        pass1_1000_1f68(uVar3);
    }
    return UVar3;
}


u16  _SHI_INVOKEERRORHANDLER1(void)

{
    i16    iVar1;
    BOOL16 BVar2;
    u16    uVar2;
    u16    unaff_CS;
    code  *pcStack6;
    u168  *puStack4;
    u16    uVar3;

    uVar3    = &USHORT_1050_1050;
    puStack4 = (u168 *)&USHORT_1050_1050;
    if((PTR_LOOP_1050_5f1c | globals->PTR_PTR_1050_5f1a) == 0x0)
    {
        pcStack6 = (fn_ptr_1)0x0;
        puStack4 = (u168 *)0x0;
    }
    else
    {
        iVar1    = mem_op_1000_21b6(PTR_PTR_1050_5f1a, globals->PTR_LOOP_1050_5f1c);
        pcStack6 = (fn_ptr_1)PTR_PTR_1050_5f1a;
        puStack4 = globals->PTR_LOOP_1050_5f1c;
        if(iVar1 == 0x0)
        {
            globals->PTR_PTR_1050_5f1a  = &PTR_PTR_1050_1f7e;
            globals->PTR_LOOP_1050_5f1c = &globals->PTR_LOOP_1050_1000;
            pcStack6                    = (fn_ptr_1)&PTR_PTR_1050_1f7e;
            puStack4                    = (u168 *)&globals->PTR_LOOP_1050_1000;
        }
    }
    if((puStack4 | pcStack6) != 0x0)
    {
        BVar2 = msg_box_op_1000_1f24(&PTR_PTR_1050_5f1a, &USHORT_1050_1050, 0x0, unaff_CS);
        if(BVar2 == 0x0)
        {
            uVar2 = (*pcStack6)();
        }
        else
        {
            puStack4 = (u168 *)0x0;
            pcStack6 = (fn_ptr_1)0x0;
            uVar2    = 0x0;
        }
        if((puStack4 | pcStack6) != 0x0)
        {
            pass1_1000_1f68(uVar3);
        }
        return uVar2;
    }
    return 0x0;
}


void pass1_1000_201c(i16 param_1, i16 param_2, u16 param_3)

{
    u16        uVar1;
    u32 uVar2;
    u16        uVar3;
    BOOL16     BVar4;
    i16        iVar5;
    u16        uVar6;

    if(param_1 == 0x0)
    {
        (param_2 + 0x6) = 0x0;
        (param_2 + 0x4) = 0x0;
    }
    uVar3 = (param_2 + 0x6) | (param_2 + 0x4);
    while(uVar3 != 0x0)
    {
        BVar4 = pass1_1000_206c((param_2 + 0x4), (param_2 + 0x6));
        if(BVar4 == 0x0)
        {
            uVar2           = (param_2 + 0x4);
            uVar6           = (uVar2 >> 0x10);
            iVar5           = uVar2;
            uVar1           = (iVar5 + 0x2c);
            (param_2 + 0x4) = (iVar5 + 0x2a);
            (param_2 + 0x6) = uVar1;
        }
        else
        {
            mem_op_1000_1b9a(0x1, *(u1632 *)(param_2 + 0x4), (param_2 + 0x6), param_3);
        }
        uVar3 = (param_2 + 0x6) | (param_2 + 0x4);
    }
    return;
}


u16  pass1_1000_21d2(u8 param_1, long param_2, u16 param_3, u16 param_4, u8 param_5)

{
    undefined3 uVar1;
    BOOL16     BVar2;

    BVar2 = mem_op_1000_1dfa(0x0, param_1, param_3, param_4);
    if(BVar2 == 0x0)
    {
        if((param_1 & 0x4) == 0x0)
        {
            uVar1 = SegmentLimit(param_4);
            if((bool)((u8)((u163)uVar1 >> 0x10) & 0x1))
            {
                if(param_2 == 0x0)
                {
                    return 0x1;
                }
                if((!CARRY4(param_3, param_2 - 0x1U)) && (param_3 + (param_2 - 0x1U) <= uVar1))
                {
                    return 0x1;
                }
            }
        }
        else
        {
            BVar2 = pass1_1000_22c0(param_3, param_4, param_2, param_2._2_2_, _param_1);
            if(BVar2 != 0x0)
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}


i16 *entry(u16 param_1, u16 param_2, u16 param_3, u16 param_4, u16 param_5, CONTEXT *in_task_context, u16 param_7, i16 param_8)

{
    u16       *puVar1;
    u16        uVar2;
    char      *pcVar3;
    code      *pcVar4;
    u16        uVar5;
    LPCSTR     str;
    u16       *puVar6;
    u16       *puVar7;
    char      *pcVar8;
    CHAR      *unaff_SS;
    bool       bVar9;
    DWORD      DVar10;
    u32 uVar11;
    u32 uVar12;
    i16        iVar13;
    i16        iVar14;
    u8        *puVar15;
    u16        uVar16;

    uVar11 = CONCAT22(param_7, globals->PTR_LOOP_1050_5f84);
    do
    {
        uVar16 = 0x0;
        InitTask16(in_task_context);
        globals->PTR_LOOP_1050_5f84 = uVar11;
        if((param_8 != 0x0) && (bVar9 = param_1 < 0xff00, param_1 = param_1 + 0x100, globals->PTR_LOOP_1050_5f7e = param_5, bVar9))
        {
            globals->PTR_LOOP_1050_5f48 = param_1;
            globals->PTR_LOOP_1050_5f4a = param_3;
            globals->PTR_LOOP_1050_5f4c = param_4;
            globals->PTR_LOOP_1050_5f4e = param_2;
            globals->PTR_LOOP_1050_5f50 = param_5;
            LockSegment16((HGLOBAL16)s_tile2_bmp_1050_1538);
            globals->PTR_LOOP_1050_5f52     = (uVar11 >> 0x10);
            globals->PTR_LOOP_1050_5f84     = uVar11;
            DVar10                          = GetVersion16();
            globals->PTR_LOOP_1050_5f52     = (uVar11 >> 0x10);
            globals->PTR_LOOP_1050_5f84     = uVar11;
            globals->PTR_LOOP_1050_5f80     = CONCAT11(DVar10, (DVar10 >> 0x8));
            pcVar4                          = (fn_ptr_1)swi(0x21);
            uVar12                          = uVar11;
            uVar11                          = (*pcVar4)(uVar16);
            globals->PTR_LOOP_1050_5f52     = (uVar12 >> 0x10);
            globals->PTR_LOOP_1050_5f84     = uVar12;
            globals->globals->DAT_1050_5f82 = CONCAT11(uVar11, (uVar11 >> 0x8));
            globals->DAT_1050_5f87          = 0x0;
            WaitEvent16(0x1000);
            globals->PTR_LOOP_1050_5f84 = uVar11;
            puVar15                     = globals->PTR_LOOP_1050_5f4c;
            param_8                     = InitApp16((HINSTANCE16)s_tile2_bmp_1050_1538);
            globals->PTR_LOOP_1050_5f84 = uVar11;
            if(param_8 != 0x0)
                break;
        }
        in_task_context = (CONTEXT *)s_tile2_bmp_1050_1538;
        param_8         = CONCAT11((param_8 >> 0x8), 0xff);
        pass1_1000_24db(param_8, 0x0);
        globals->PTR_LOOP_1050_5f84 = uVar11;
    } while(true);
    dos3_call_1000_23ea(param_2, param_5, 0x0, unaff_SS);
    globals->PTR_LOOP_1050_5f84 = uVar11;
    pass1_1000_262c(0x238d, s_tile2_bmp_1050_1538, unaff_SS, s_tile2_bmp_1050_1538);
    globals->PTR_LOOP_1050_5f84 = uVar11;
    pass1_1000_27d6((uVar11 >> 0x10));
    uVar11 = ret_op_1000_55ac(puVar15);
    uVar5  = uVar11;
    init_1000_23be(param_1, (uVar11 >> 0x10), unaff_SS);
    fn_ptr_op_1000_24cd(uVar5, 0x0);
    iVar14 = 0x15;
    iVar13 = 0x15;
    pass1_1000_25a8(param_5, s_tile2_bmp_1050_1538);
    pass1_1000_2913(iVar13, param_5, s_tile2_bmp_1050_1538);
    str = poss_str_op_1000_28dc(iVar14);
    if(str != (PCHAR)0x0)
    {
        iVar13 = 0x9;
        if(*str == 'M')
        {
            iVar13 = 0xf;
        }
        str    = str + iVar13;
        iVar13 = 0x22;
        pcVar8 = str;
        do
        {
            if(iVar13 == 0x0)
                break;
            iVar13 = iVar13 + -0x1;
            pcVar3 = pcVar8;
            pcVar8 = pcVar8 + 0x1;
        } while(*pcVar3 != '\r');
        pcVar8[-0x1] = '\0';
    }
    FatalAppExit16(s_tile2_bmp_1050_1538, str);
    FatalExit();
    puVar7 = &globals->PTR_LOOP_1050_63fe;
    do
    {
        puVar1 = puVar7;
        puVar7 = puVar7 + 0x1;
        uVar2  = *puVar1;
        puVar6 = puVar7;
        if((uVar2 == uVar5) || (puVar6 = (uVar2 + 0x1), puVar6 == 0x0))
        {
            return puVar6;
        }
        iVar13 = -0x1;
        do
        {
            if(iVar13 == 0x0)
                break;
            iVar13 = iVar13 + -0x1;
            puVar1 = puVar7;
            puVar7 = (puVar7 + 0x1);
        } while(*puVar1 != '\0');
    } while(true);
}


i16 *dos3_call_1000_23ea(u16 param_1, u16 param_2, i16 param_3, u16 param_4)

{
    u8        *pbVar1;
    u8        *pbVar2;
    u8         bVar3;
    i16       *piVar4;
    u8        *pbVar5;
    char      *pcVar6;
    u16        uVar7;
    code     **ppcVar8;
    code      *pcVar9;
    u16        uVar10;
    u8         bVar11;
    u8         bVar12;
    u16        uVar13;
    LPCSTR     str;
    i16       *piVar14;
    u16        extraout_DX;
    u16        uVar15;
    u16        uVar16;
    u8        *pbVar17;
    i16       *piVar18;
    u8        *pbVar19;
    char      *pcVar20;
    bool       bVar21;
    u32 uVar22;
    i16        iVar23;
    i16        iVar24;

    pcVar9 = (fn_ptr_1)swi(0x21);
    (*pcVar9)(param_3 + 0x1);
    pcVar9                      = (fn_ptr_1)swi(0x21);
    globals->PTR_LOOP_1050_5f6a = param_1;
    globals->PTR_LOOP_1050_5f6c = param_2;
    (*pcVar9)();
    uVar15 = extraout_DX;
    uVar13 = pass1_1000_29dc(param_4);
    uVar22 = CONCAT22(uVar15, uVar13);
    if(&PTR_LOOP_1050_6202 != 0x0)
    {
        uVar7   = *&PTR_LOOP_1050_5f7e;
        bVar21  = false;
        ppcVar8 = &PTR_LOOP_1050_6200;
        (**ppcVar8)(0x1000);
        if(bVar21)
        {
            iVar24 = 0x2;
            iVar23 = 0x2;
            pass1_1000_25a8(uVar7, 0x1000);
            pass1_1000_2913(iVar23, uVar7, 0x1000);
            str = poss_str_op_1000_28dc(iVar24);
            if(str != (PCHAR)0x0)
            {
                iVar23 = 0x9;
                if(*str == 'M')
                {
                    iVar23 = 0xf;
                }
                str     = str + iVar23;
                iVar23  = 0x22;
                pcVar20 = str;
                do
                {
                    if(iVar23 == 0x0)
                        break;
                    iVar23  = iVar23 + -0x1;
                    pcVar6  = pcVar20;
                    pcVar20 = pcVar20 + 0x1;
                } while(*pcVar6 != '\r');
                pcVar20[-0x1] = '\0';
            }
            FatalAppExit16(0x1000, str);
            FatalExit();
            piVar18 = &globals->PTR_LOOP_1050_63fe;
            do
            {
                piVar4  = piVar18;
                piVar18 = piVar18 + 0x1;
                iVar23  = *piVar4;
                piVar14 = piVar18;
                if((iVar23 == &USHORT_1050_1050) || (piVar14 = (iVar23 + 0x1), piVar14 == 0x0))
                {
                    return piVar14;
                }
                iVar23 = -0x1;
                do
                {
                    if(iVar23 == 0x0)
                        break;
                    iVar23  = iVar23 + -0x1;
                    piVar4  = piVar18;
                    piVar18 = (piVar18 + 0x1);
                } while(*piVar4 != '\0');
            } while(true);
        }
        ppcVar8 = &PTR_LOOP_1050_6200;
        uVar22  = (**ppcVar8)(0x1000);
    }
    iVar23  = (s_New_failed_in_Op__Op_1050_0020 + 0xc);
    piVar18 = uVar22;
    if(iVar23 != 0x0)
    {
        pbVar19 = (u8 *)0x0;
        piVar14 = uVar22;
        do
        {
            bVar21  = *pbVar19 == 0x0;
            piVar18 = piVar14;
            if(bVar21)
                break;
            iVar24  = 0xd;
            pbVar17 = (u8 *)s__C_FILE_INFO__1050_5f5c;
            do
            {
                if(iVar24 == 0x0)
                    break;
                iVar24  = iVar24 + -0x1;
                pbVar5  = pbVar19;
                pbVar19 = pbVar19 + 0x1;
                pbVar1  = pbVar17;
                pbVar17 = pbVar17 + 0x1;
                bVar21  = *pbVar1 == *pbVar5;
            } while(bVar21);
            if(bVar21)
            {
                pbVar17 = (u8 *)0x5f90;
                uVar16  = (uVar22 >> 0x10);
                goto LAB_1000_2495;
            }
            iVar24  = 0x7fff;
            piVar18 = 0x0;
            bVar21  = true;
            do
            {
                if(iVar24 == 0x0)
                    break;
                iVar24  = iVar24 + -0x1;
                pbVar1  = pbVar19;
                pbVar19 = pbVar19 + 0x1;
                bVar21  = *pbVar1 == 0x0;
            } while(!bVar21);
            piVar14 = piVar18;
        } while(bVar21);
    }
LAB_1000_24a9:
    fn_ptr_op_1000_2594(0x620c, 0x620c);
    fn_ptr_op_1000_2594(0x620c, 0x620c);
    fn_ptr_op_1000_2594(0x61fe, 0x61ee);
    return piVar18;
LAB_1000_2495:
    pbVar2  = pbVar19 + 0x1;
    bVar3   = *pbVar19;
    uVar10  = piVar14 & 0xff00;
    bVar11  = bVar3 + 0xbf;
    piVar18 = (uVar10 | bVar11);
    if(bVar3 < 0x41)
        goto LAB_1000_24a9;
    pbVar19 = pbVar19 + 0x2;
    bVar3   = *pbVar2;
    piVar14 = (uVar16 & 0xff00);
    bVar12  = bVar3 + 0xbf;
    piVar18 = (piVar14 | bVar12);
    if(bVar3 < 0x41)
        goto LAB_1000_24a9;
    pbVar1  = pbVar17;
    pbVar17 = pbVar17 + 0x1;
    *pbVar1 = bVar12 | bVar11 * '\x10';
    uVar16  = uVar10;
    goto LAB_1000_2495;
}

void dos3_op_1000_256b(void)

{
    fn_ptr_1pcVar1;

    if(PTR_LOOP_1050_6202 != 0x0)
    {
        (*(fn_ptr_1)PTR_LOOP_1050_6200)();
    }
    pcVar1 = (fn_ptr_1)swi(0x21);
    (*pcVar1)();
    return;
}


i16 *exit_1000_25cc(i16 param_1, u16 param_2, u16 param_3)

{
    i16   *piVar1;
    char  *pcVar2;
    LPCSTR str;
    i16   *piVar3;
    i16   *piVar4;
    char  *pcVar5;
    i16    iVar6;
    i16    iVar7;

    iVar7 = 0x2;
    iVar6 = 0x2;
    pass1_1000_25a8(param_2, param_3);
    pass1_1000_2913(iVar6, param_2, param_3);
    str = poss_str_op_1000_28dc(iVar7);
    if(str != (PCHAR)0x0)
    {
        iVar6 = 0x9;
        if(*str == 'M')
        {
            iVar6 = 0xf;
        }
        str    = str + iVar6;
        iVar6  = 0x22;
        pcVar5 = str;
        do
        {
            if(iVar6 == 0x0)
                break;
            iVar6  = iVar6 + -0x1;
            pcVar2 = pcVar5;
            pcVar5 = pcVar5 + 0x1;
        } while(*pcVar2 != '\r');
        pcVar5[-0x1] = '\0';
    }
    FatalAppExit16(param_3, str);
    FatalExit();
    piVar4 = &globals->PTR_LOOP_1050_63fe;
    do
    {
        piVar1 = piVar4;
        piVar4 = piVar4 + 0x1;
        iVar6  = *piVar1;
        piVar3 = piVar4;
        if((iVar6 == param_1) || (piVar3 = (iVar6 + 0x1), piVar3 == 0x0))
        {
            return piVar3;
        }
        iVar6 = -0x1;
        do
        {
            if(iVar6 == 0x0)
                break;
            iVar6  = iVar6 + -0x1;
            piVar1 = piVar4;
            piVar4 = (piVar4 + 0x1);
        } while(*piVar1 != '\0');
    } while(true);
}
u16 pass1_1000_0e08(i16 param_1, u16 param_2)

{
    u16 *puVar1;
    u8  *pbVar2;
    u16  uVar3;
    u16 *puVar4;
    u16 *puVar5;
    bool bVar6;
    u32  uVar7;

    puVar5 = (param_1 + -0x2);
    bVar6  = (*(u8 *)puVar5 & 0x2) != 0x0;
    if(bVar6)
    {
        puVar1        = puVar5;
        *(u8 *)puVar1 = *(u8 *)puVar1 & 0xfe;
    }
    else
    {
        puVar4  = (puVar5 - (param_1 + -0x4));
        puVar1  = puVar4;
        *puVar1 = *puVar1 + (*puVar5 & 0xfffc);
        puVar5  = puVar4;
    }
    puVar4 = ((*puVar5 & 0xfffc) + puVar5);
    if((*(u8 *)puVar4 & 0x1) == 0x0)
    {
        puVar1  = puVar5;
        *puVar1 = *puVar1 + (*puVar4 & 0xfffc);
        if(puVar4 == PTR_LOOP_1050_000e)
        {
            globals->PTR_LOOP_1050_000e = puVar5;
        }
        (puVar4[0x2] + 0x2) = puVar4[0x1];
        (puVar4[0x1] + 0x4) = puVar4[0x2];
        puVar4              = ((*puVar5 & 0xfffc) + puVar5);
    }
    puVar4[-0x1] = *puVar5 & 0xfffc;
    uVar3        = *DAT_1050_0004;
    puVar1       = puVar4 + -0x1;
    if(uVar3 <= *puVar1 && *puVar1 != uVar3)
    {
        uVar3          = *puVar5 & 0xfffc;
        *DAT_1050_0004 = uVar3;
    }
    puVar1        = puVar4;
    *(u8 *)puVar1 = *(u8 *)puVar1 & 0xfd;
    if(bVar6)
    {
        if((PTR_LOOP_1050_0010 + 0x2) != globals->PTR_LOOP_1050_0010)
        {
            pbVar2  = (u8 *)(DAT_1050_0004 + 0x3);
            *pbVar2 = *pbVar2 & 0x7f;
        }
        puVar5[0x2]                                 = globals->PTR_LOOP_1050_0010;
        uVar3                                       = (PTR_LOOP_1050_0010 + 0x2);
        puVar5[0x1]                                 = uVar3;
        *(u16 **)((PTR_LOOP_1050_0010 + 0x2) + 0x4) = puVar5;
        *(u16 **)(PTR_LOOP_1050_0010 + 0x2)         = puVar5;
    }
    globals->PTR_LOOP_1050_000a = globals->PTR_LOOP_1050_000a + -0x1;
    if(PTR_LOOP_1050_000a == 0x0)
    {
        uVar7 = mem_op_1000_0510(0x1, 0x0, param_2);
        uVar3 = uVar7;
    }
    return uVar3;
}

long  pass1_1000_0ed4(u16 param_1, u16 param_2, u16 param_3, u16 param_4, u16 param_5, u16 *param_6, u16 param_7)

{
    u16  *puVar1;
    u16  *puVar2;
    u16   uVar3;
    u16 **ppuVar4;
    u16   uVar5;
    u16   uVar6;
    u16  *puVar7;
    u16  *puVar8;
    long  lVar9;
    u16   UVar10;
    u16   UVar11;
    u16   UVar12;

    if((&PTR_LOOP_1050_000c & 0xfff8) == 0xcad0)
    {
        UVar11 = 0x0;
        UVar12 = &PTR_LOOP_1050_0002;
        if((param_3 & 0x8) == 0x0)
        {
            ppuVar4 = &param_6;
        }
        else
        {
            ppuVar4 = (u16 **)0x0;
            param_2 = 0x0;
        }
        uVar5 = pass1_1000_0fb8(param_1, param_4, param_6, param_5, param_3, ppuVar4, param_2);
        if(uVar5 == 0x0)
        {
            return CONCAT22(param_7, param_6);
        }
        if((param_3 & 0x8) == 0x0)
        {
            lVar9  = mem_op_1000_0a48((u8)param_3, param_4, param_5, CONCAT22(UVar12, UVar11), param_1);
            uVar3  = (lVar9 >> 0x10);
            puVar8 = lVar9;
            if(lVar9 != 0x0)
            {
                puVar7 = param_6;
                for(uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 = uVar6 - 0x1)
                {
                    puVar2  = puVar8;
                    puVar8  = puVar8 + 0x1;
                    puVar1  = puVar7;
                    puVar7  = puVar7 + 0x1;
                    *puVar2 = *puVar1;
                }
                for(uVar5 = ((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 = uVar5 - 0x1)
                {
                    puVar2  = puVar8;
                    puVar8  = (puVar8 + 0x1);
                    puVar1  = puVar7;
                    puVar7  = (puVar7 + 0x1);
                    *puVar2 = *puVar1;
                }
                call_fn_ptr_1000_0dc6(param_6, param_7, param_1);
            }
            return lVar9;
        }
        if((param_5 | param_4) == 0x0)
        {
            return 0x0;
        }
        UVar10 = 0x5;
    }
    else
    {
        UVar11 = 0x0;
        UVar12 = 0x0;
        UVar10 = 0xe;
    }
    pass1_1000_1e61(param_1, UVar10, UVar11, UVar12);
    return 0x0;
}

u16 pass1_1000_0fb8(u16 param_1, u16 param_2, i16 param_3, u16 param_4, u16 param_5, u16 *param_6, u16 param_7)

{
    u16   *puVar1;
    u8     bVar2;
    u16    uVar3;
    BOOL16 BVar4;
    i16    iVar5;
    u16    uVar6;
    u16   *puVar7;
    u16   *puVar8;
    u32    uVar9;
    u16    uStack4;

    if((param_4 | param_2) == 0x0)
    {
        pass1_1000_1e61(param_1, 0x4, globals->PTR_LOOP_1050_0000, globals->PTR_LOOP_1050_0002);
        if((param_7 | param_6) != 0x0)
        {
            param_6[0x1] = 0x0;
            *param_6     = 0x0;
            return 0x0;
        }
        return 0x1;
    }
    bVar2 = (u8)PTR_LOOP_1050_000c & 0x7;
    if(((u8)PTR_LOOP_1050_000c & 0x7) != 0x0)
    {
        if(bVar2 == 0x1)
        {
            uVar3 = (PTR_LOOP_1050_0000 + 0x18);
            if(param_4 != 0x0)
            {
                return uVar3;
            }
            if(param_2 <= uVar3)
            {
                return 0x0;
            }
            return uVar3;
        }
        if(bVar2 != 0x2)
        {
            if(bVar2 != 0x3)
            {
                if((param_7 | param_6) != 0x0)
                {
                    param_6[0x1] = 0x0;
                    *param_6     = 0x0;
                    return 0x0;
                }
                return 0x1;
            }
            if((((param_7 | param_6) != 0x0) && (param_4 == 0x0)) && (param_2 <= (PTR_LOOP_1050_0000 + 0x1c)))
            {
                uVar9 = pass1_1000_1284(CONCAT22(0x1050, param_3), param_1);
                if(CONCAT22(param_4, param_2) < uVar9)
                {
                    return param_2;
                }
                return uVar9;
            }
            iVar5 = mem_1000_0670(param_5, CONCAT22(param_7, param_6), param_2, 0x0, param_4, param_1);
            if(iVar5 != 0x0)
            {
                return 0x0;
            }
            if((param_7 | param_6) != 0x0)
            {
                return 0x0;
            }
            return 0x1;
        }
    }
    puVar8  = (param_3 + -0x2);
    uVar3   = *puVar8 & 0x7ffc;
    uStack4 = uVar3 - 0x2;
    if((*(u8 *)(param_3 + -0x1) & 0x80) != 0x0)
    {
        uStack4 = uVar3 - 0x6;
    }
    if((((param_4 == 0x0) && (param_2 <= uStack4)) || ((param_4 == 0x0 && (param_2 <= (PTR_LOOP_1050_0000 + 0x1c))))) && (BVar4 = pass1_1000_115c(param_2, puVar8), BVar4 != 0x0))
    {
        if((param_5 & 0x1) != 0x0)
        {
            uVar3 = (*puVar8 & 0x7ffc) - 0x2;
            if(uStack4 < param_2)
            {
                puVar7 = (uStack4 + param_3);
                iVar5  = -uStack4;
            }
            else
            {
                if(uVar3 <= param_2)
                {
                    return 0x0;
                }
                puVar7 = (param_2 + param_3);
                iVar5  = -param_2;
            }
            uVar3 = uVar3 + iVar5;
            for(uVar6 = uVar3 >> 0x1; uVar6 != 0x0; uVar6 = uVar6 - 0x1)
            {
                puVar1  = puVar7;
                puVar7  = puVar7 + 0x1;
                *puVar1 = 0x0;
            }
            if((uVar3 & 0x1) != 0x0)
            {
                *puVar7 = 0x0;
            }
        }
        return 0x0;
    }
    return uStack4;
}


u32  pass1_1000_1284(u32 param_1, u16 param_2)

{
    u8         bVar1;
    u16        uVar2;
    u32 uVar3;
    u8         bVar4;
    u16        uVar5;
    bool       bVar6;
    DWORD      DVar7;
    u16        uStack6;
    i16        iStack4;

    if((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0)
    {
        pass1_1000_1e61(param_2, 0xe, 0x0, 0x0);
        return 0xffffffff;
    }
    bVar1 = *(u8 *)&PTR_LOOP_1050_000c;
    bVar4 = bVar1 & 0x7;
    if((bVar1 & 0x7) != 0x0)
    {
        if(bVar4 == 0x1)
        {
            uVar3 = 0x0;
            return (uVar3 + 0x18);
        }
        if(bVar4 != 0x2)
        {
            if(bVar4 != 0x3)
            {
                return 0xffffffff;
            }
            DVar7 = mem_op_1000_1532(param_2);
            return CONCAT22((DVar7 >> 0x10) - (DVar7 < 0x14), DVar7 - 0x14);
        }
    }
    uVar2   = (param_1 + -0x2);
    uVar5   = uVar2 & 0x7ffc;
    uStack6 = uVar5 - 0x2;
    iStack4 = 0x0;
    if((uVar2 & 0x8000) != 0x0)
    {
        bVar6   = uStack6 < 0x4;
        uStack6 = uVar5 - 0x6;
        iStack4 = -bVar6;
    }
    return CONCAT22(iStack4, uStack6);
}


void  mem_op_1000_131c(u16 param_1, u16 param_2, i16 param_3, u16 param_4)

{
    HGLOBAL16 HVar1;
    bool      bVar2;
    long      lVar3;
    u16       uStack10;
    u16       uStack8;
    i16       iStack6;

    lVar3   = CONCAT22(uStack8, uStack10);
    iStack6 = 0x1;
    if(((param_1 & 0x1000) != 0x0) && ((param_3 != 0x0 || (0xfff0 < param_2))))
    {
        param_2 = 0xfff0;
        param_3 = 0x0;
    }
    if((param_1 & 0x4) != 0x0)
    {
        lVar3 = mem_op_1000_1558(param_2, param_3, param_4);
    }
    do
    {
        HVar1    = GLobalAlloc16(param_4, CONCAT22(param_3, param_2));
        uStack10 = lVar3;
        if(HVar1 != 0x0)
            break;
        bVar2   = iStack6 != 0x0;
        iStack6 = iStack6 + -0x1;
        param_4 = s_tile2_bmp_1050_1538;
    } while(bVar2);
    if((param_1 & 0x4) != 0x0)
    {
        if(HVar1 != 0x0)
        {
            GlobalPageLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
        }
        pass1_1000_15ce((u16 *)uStack10, (lVar3 >> 0x10), s_tile2_bmp_1050_1538);
    }
    if(HVar1 != 0x0)
    {
        WIN16_GlobalLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
        return;
    }
    return;
}


long  mem_op_1000_13ce(WORD param_1)

{
    HGLOBAL16 HVar1;
    u16       uVar2;
    DWORD     DVar3;

    DVar3 = GlobalHandle16(param_1);
    uVar2 = (DVar3 >> 0x10);
    if(DVar3 != 0x0)
    {
        HVar1 = GlobalFree16((HGLOBAL16)s_tile2_bmp_1050_1538);
        return CONCAT22(uVar2, (HVar1 == 0x0));
    }
    return (long)(uVar2 << 0x10);
}


void  mem_op_1000_1408(u16 param_1, u16 param_2, u16 param_3, WORD param_4)

{
    HGLOBAL16 HVar1;
    bool      bVar2;
    DWORD     DVar3;
    i16       iStack12;
    u16       uStack8;

    DVar3    = GlobalHandle16(param_4);
    uStack8  = 0x32;
    iStack12 = 0x1;
    if(((param_1 & 0x1000) != 0x0) && ((param_3 != 0x0 || (0xfff0 < param_2))))
    {
        param_2 = 0xfff0;
        param_3 = 0x0;
    }
    if((param_1 & 0x100) != 0x0)
    {
        uStack8 = 0x72;
    }
    if((param_1 & 0x804) != 0x0)
    {
        uStack8 = uStack8 & 0xfffd;
    }
    if(DVar3 != 0x0)
    {
        if((param_1 & 0x4) != 0x0)
        {
            GlobalPageUnlock16((HGLOBAL16)s_tile2_bmp_1050_1538);
        }
        do
        {
            HVar1 = GlobalReAlloc16((HGLOBAL16)s_tile2_bmp_1050_1538, CONCAT22(param_2, uStack8), param_3);
            if(HVar1 != 0x0)
                break;
            uStack8  = uStack8 & 0xffcf;
            bVar2    = iStack12 != 0x0;
            iStack12 = iStack12 + -0x1;
        } while(bVar2);
        if((HVar1 != 0x0) && ((param_1 & 0x4) != 0x0))
        {
            GlobalPageLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
        }
        if(HVar1 != 0x0)
        {
            WIN16_GlobalLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
            return;
        }
    }
    return;
}


BOOL16  mem_op_1000_14f2(u16 param_1, u16 param_2, i16 param_3, u16 param_4, u16 param_5)

{
    u16  in_AX;
    u16  in_DX;
    WORD unaff_CS;

    if(((param_1 & 0x1000) != 0x0) || ((param_3 == 0x0 && (param_2 < 0xfff1))))
    {
        mem_op_1000_1408(param_1 & 0xfdff | 0x800, param_2, param_3, unaff_CS);
        if((in_DX | in_AX) != 0x0)
        {
            return 0x1;
        }
    }
    return 0x0;
}


DWORD  mem_op_1000_1532(WORD param_1)

{
    DWORD DVar1;

    DVar1 = GlobalHandle16(param_1);
    if(DVar1 != 0x0)
    {
        DVar1 = GlobalSize16((HGLOBAL16)s_tile2_bmp_1050_1538);
        return DVar1;
    }
    return 0x0;
}


long mem_op_1000_1558(u16 param_1, u16 param_2, u16 param_3)

{
    u16   uVar1;
    DWORD DVar2;
    u16   uStack12;
    u16   uStack10;
    u16   uStack8;

    uStack12 = 0x0;
    uStack10 = 0x0;
    uStack8  = 0x8;
    if((param_2 < 0x9) && ((param_2 < 0x8 || (param_1 == 0x0))))
    {
        do
        {
            while(true)
            {
                DVar2   = CONCAT22(uStack10, param_3);
                param_3 = s_tile2_bmp_1050_1538;
                DVar2   = GlobalDOSAlloc16(DVar2);
                uVar1   = DVar2;
                if(uVar1 == 0x0)
                    break;
                0x0                 = 0x0;
                &PTR_LOOP_1050_0002 = uStack12;
                uStack12            = uVar1;
            }
            uVar1    = uStack8 & 0x1;
            uStack8  = uStack8 >> 0x1;
            uStack10 = uStack10 >> 0x1 | (uVar1 != 0x0) << 0xf;
        } while((param_2 < uStack8) || ((param_2 <= uStack8 && (param_1 <= uStack10))));
    }
    return (long)(uStack12 << 0x10);
}


void pass1_1000_15ce(u16 *param_1, u16 param_2, WORD param_3)

{
    u16 *puVar1;
    u16  uVar2;

    uVar2 = param_2 | param_1;
    while(uVar2 != 0x0)
    {
        puVar1  = *param_1;
        param_2 = param_1[0x1];
        GlobalDOSFree16(param_3);
        param_1 = puVar1;
        param_3 = (WORD)s_tile2_bmp_1050_1538;
        uVar2   = param_2 | puVar1;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u8 *mem_op_1000_160a(u16 param_1, u16 param_2)

{
    u8 *puVar1;

    puVar1 = ret_true_1000_2146();
    if(puVar1 == 0x0)
    {
        return puVar1;
    }
    if((PTR_LOOP_1050_5f2e | globals->PTR_LOOP_1050_5f2c) == 0x0)
    {
        DAT_1050_5f30                = 0x1;
        DAT_1050_5f32                = 0x1;
        globals->_PTR_LOOP_1050_5f2c = mem_op_1000_18ec(DAT_1050_5f46, param_1, param_2);
        if(_PTR_LOOP_1050_5f2c != 0x0)
        {
            if(PTR_LOOP_1050_5f42 != 0x0)
            {
                pass1_1000_1a54(PTR_LOOP_1050_5f42, globals->_PTR_LOOP_1050_5f2c, (_PTR_LOOP_1050_5f2c >> 0x10), param_2);
            }
            globals->PTR_LOOP_1050_5f2e = (_PTR_LOOP_1050_5f2c >> 0x10);
            if(DAT_1050_5f44 != 0xffff)
            {
                pass1_1000_1afe(DAT_1050_5f44, (u1632)PTR_LOOP_1050_5f2c, globals->PTR_LOOP_1050_5f2e);
            }
        }
    }
    empty_fn_1000_214a();
    return globals->PTR_LOOP_1050_5f2c;
}


u16 mem_1000_167a(u16 param_1, u16 param_2, u16 param_3)

{
    u8  *puVar1;
    long lVar2;

    if((PTR_LOOP_1050_5f2e | globals->PTR_LOOP_1050_5f2c) == 0x0)
    {
        puVar1 = mem_op_1000_160a(param_3, param_2);
        if((param_3 | puVar1) == 0x0)
        {
            return 0x0;
        }
    }
    lVar2 = mem_op_1000_0a48(0x0, param_1, 0x0, CONCAT22(PTR_LOOP_1050_5f2e, globals->PTR_LOOP_1050_5f2c), param_2);
    return lVar2;
}


u16 pass1_1000_16aa(u16 *param_1, u16 param_2, u16 param_3, u16 param_4, u16 param_5, u16 param_6)

{
    u16  uVar1;
    long lVar2;

    if((param_2 | param_1) == 0x0)
    {
        uVar1 = mem_1000_167a(param_3, param_5, param_4);
        return uVar1;
    }
    if(param_3 == 0x0)
    {
        pass1_1000_16ee(param_1, param_2, param_5);
        return 0x0;
    }
    lVar2 = pass1_1000_0ed4(param_5, param_6, 0x0, param_3, 0x0, param_1, param_2);
    return lVar2;
}


u16 pass1_1000_180c(u16 param_1, u16 param_2, u16 param_3)

{
    u8  *puVar1;
    long lVar2;

    if((PTR_LOOP_1050_5f2e | globals->PTR_LOOP_1050_5f2c) == 0x0)
    {
        puVar1 = mem_op_1000_160a(param_2, param_3);
        if((param_2 | puVar1) == 0x0)
        {
            return 0x0;
        }
    }
    lVar2 = mem_op_1000_0a48(0x0, param_1, 0x0, CONCAT22(PTR_LOOP_1050_5f2e, globals->PTR_LOOP_1050_5f2c), param_3);
    return lVar2;
}


u16 pass1_1000_183c(u16 param_1, u16 param_2, u16 param_3)

{
    u8  *puVar1;
    long lVar2;

    puVar1 = 0x0;
    if((param_2 * param_1 >> 0x10) != 0x0)
    {
        return 0x0;
    }
    if(((PTR_LOOP_1050_5f2e | globals->PTR_LOOP_1050_5f2c) == 0x0) && (PTR_LOOP_1050_5f2c = mem_op_1000_160a(0x0, param_3), globals->PTR_LOOP_1050_5f2e = puVar1, (puVar1 | globals->PTR_LOOP_1050_5f2c) == 0x0))
    {
        return 0x0;
    }
    lVar2 = mem_op_1000_0a48(0x1, (param_2 * param_1), 0x0, CONCAT22(PTR_LOOP_1050_5f2e, globals->PTR_LOOP_1050_5f2c), param_3);
    return lVar2;
}


u32  mem_op_1000_18ec(u16 param_1, u16 param_2, u16 param_3)

{
    u32 uVar1;

    uVar1 = mem_op_1000_1902(param_1, 0x0, 0x0, 0xc, param_3, param_2);
    return uVar1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32  mem_op_1000_1902(u16 param_1, u16 param_2, u16 param_3, u16 param_4, u16 param_5, u16 param_6)

{
    u16   *pUVar1;
    u16    UVar2;
    BOOL16 BVar3;
    i16    iVar4;
    u16    uVar3;
    u16    UVar5;
    u16   *pUVar6;
    DWORD  DVar7;
    u32    uVar8;
    u16   *puVar1;

    UVar5 = param_6;
    if(((param_1 & 0x8000) != 0x0) && (UVar5 = param_6, _SHI_INVOKEERRORHANDLER1 != -0x6f70))
    {
        param_1 = param_1 | 0x1;
        UVar5   = param_6;
    }
    do
    {
        uVar3  = UVar5;
        pUVar1 = (param_1 & 0xfffb | 0x1000);
        mem_op_1000_131c(pUVar1, 0x100, 0x0, param_5);
        UVar5 = uVar3 | pUVar1;
        if(UVar5 != 0x0)
            break;
        UVar2 = pass1_1000_1e61(param_5, 0x2, 0x0, 0x0);
    } while(UVar2 != 0x0);
    if((uVar3 | pUVar1) != 0x0)
    {
        pUVar1[0x17]                = &PTR_PTR_1050_5f1a;
        pUVar1[0x18]                = &USHORT_1050_1050;
        pUVar1[0x15]                = globals->PTR_LOOP_1050_5f1e;
        pUVar1[0x16]                = globals->PTR_LOOP_1050_5f20;
        pUVar6                      = pUVar1;
        globals->PTR_LOOP_1050_5f1e = pUVar1;
        globals->PTR_LOOP_1050_5f20 = uVar3;
        for(iVar4 = 0x5; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
        {
            puVar1  = pUVar6;
            pUVar6  = pUVar6 + 0x1;
            *puVar1 = 0x0;
        }
        pUVar1[0x5]  = 0x0;
        pUVar1[0x7]  = 0x0;
        pUVar1[0x6]  = 0x0;
        pUVar1[0x9]  = 0x0;
        pUVar1[0x8]  = 0x0;
        pUVar1[0xa]  = 0xbead;
        pUVar1[0xb]  = param_1 & 0xfffd;
        pUVar1[0xc]  = 0x0;
        pUVar1[0xd]  = 0x2000;
        pUVar1[0xe]  = 0x800;
        DVar7        = mem_op_1000_1532(param_5);
        pUVar1[0xf]  = DVar7;
        pUVar1[0x10] = (DVar7 >> 0x10);
        pUVar1[0x12] = 0x0;
        pUVar1[0x11] = 0x0;
        pUVar1[0x13] = 0xfffe;
        pUVar1[0x14] = 0xffff;
        pUVar1[0x19] = 0x0;
        pUVar1[0x1a] = 0x0;
        pUVar1[0x20] = 0x0;
        pUVar1[0x1f] = 0x0;
        BVar3        = pass1_1000_1afe(param_4, (u1632)pUVar1, uVar3);
        if(BVar3 != 0x0)
        {
            if((param_3 | param_2) != 0x0)
            {
                pUVar6 = pUVar1;
                UVar5  = uVar3;
                uVar8  = pass1_1000_52be(param_2, param_3, param_4, 0x0);
                pass1_1000_010c(0x1, uVar8, (uVar8 >> 0x10), (u1632)pUVar6, UVar5, param_5);
            }
            return CONCAT22(uVar3, pUVar1);
        }
        mem_op_1000_1b9a(0x0, (u1632)pUVar1, uVar3, param_5);
    }
    return 0x0;
}


u32  mem_1000_0016(u32 param_1, u16 param_2)

{
    u32 uVar1;

    if((param_1 + 0x14) != -0x4153)
    {
        pass1_1000_1e61(param_2, 0xa, 0x0, 0x0);
        return 0xffffffff;
    }
    uVar1 = mem_op_1000_0052(0x0, param_2);
    return uVar1;
}


u32 mem_op_1000_0052(u16 param_1, u16 param_2)

{
    u16       *puVar1;
    u16        uVar2;
    i16        iVar3;
    u32 uVar4;
    u32        uVar5;
    i16        iStack14;
    i16        iStack12;
    i16        iStack10;
    u16        uStack8;

    uVar2   = (param_1 + 0x1e);
    iVar3   = (param_1 + 0x20);
    uStack8 = 0x0;
    do
    {
        iStack10 = (uStack8 * 0x2 + param_1);
        if((iStack10 != 0x0) && (uStack8 != 0x3))
        {
            iStack14 = 0x0;
            do
            {
                iStack12 = (iStack10 + 0x4);
                uVar4    = (iStack10 + 0x8);
                if((uVar4 + 0xa) == 0x0)
                {
                    uVar5 = mem_op_1000_0510(0x1, 0x0, param_2);
                    if(uVar5 == 0x0)
                        goto LAB_1000_00f9;
                    if(iStack12 == iStack10)
                    {
                        iStack12 = 0x0;
                    }
                }
                else
                {
                    if(iStack14 == 0x0)
                    {
                        iStack14 = iStack10;
                    }
                }
                iStack10 = iStack12;
            } while(iStack12 != iStack14);
        }
        uStack8 = uStack8 + 0x1;
    } while(uStack8 < 0x5);
    if((param_1 + 0x32) != 0x0)
    {
        (**(param_1 + 0x32))();
    }
LAB_1000_00f9:
    puVar1 = (param_1 + 0x1e);
    return CONCAT22((iVar3 - (param_1 + 0x20)) - (uVar2 < *puVar1), uVar2 - *puVar1);
}


u1632  pass1_1000_010c(i16 param_1, u16 param_2, u16 param_3, u1632 param_4, u16 param_5, u16 param_6)

{
    u16   uVar1;
    u1632 UVar2;
    u16   uVar2;
    bool  bVar3;
    u16   UVar4;
    u16   uStack8;
    u16   uStack6;
    u16   uStack4;

    uStack6 = 0x0;
    uStack8 = 0x0;
    if((param_4 + 0x14) != -0x4153)
    {
        param_5 = 0x0;
        param_4 = 0x0;
        UVar4   = 0xa;
    code_r0x10000128:
        pass1_1000_1e61(param_6, UVar4, param_4, param_5);
        return 0xffff;
    }
    DAT_1050_5f30 = 0x1;
    if(param_1 == 0x1)
    {
        uStack4 = 0x1;
        if((param_4 + 0x18) == 0x0)
        {
            UVar4 = 0x4;
            goto code_r0x10000128;
        }
    }
    else
    {
        if(param_1 == 0x2)
        {
            uStack4 = 0x2;
        }
        else
        {
            if(param_1 != 0x4)
            {
                DAT_1050_5f30 = 0x1;
                return 0xffff;
            }
            uStack4 = 0x0;
        }
    }
    while((uStack6 <= param_3 && (((uStack6 < param_3 || (uStack8 < param_2)) && (uVar1 = uStack4, UVar2 = mem_op_1000_03c6((s_version__d__d_1050_0012 + 0x8), 0x0, uStack4, 0x0, param_6, 0x0, '\0'), (UVar2 | uVar1) != 0x0)))))
    {
        uVar1   = (s_version__d__d_1050_0012 + 0x8);
        bVar3   = CARRY2(uStack8, uVar1);
        uStack8 = uStack8 + uVar1;
        uStack6 = uStack6 + bVar3;
    }
    return uStack6;
}


bool mem_op_1000_01b0(u16 param_1, u16 param_2)

{
    u16   *puVar1;
    i16   *piVar2;
    BOOL16 BVar3;
    u16    UVar4;
    u16    uVar5;
    DWORD  DVar6;
    DWORD  DVar7;
    u32    uVar8;
    u16    uVar9;
    u16    uVar10;
    u16    uStack14;
    u16    uStack12;
    i16    iStack10;
    u16    uStack6;
    i16    iStack4;

    uStack14 = 0x0;
    if(((param_1 + 0x40) | (param_1 + 0x3e)) == 0x0)
    {
        uVar5 = param_1 + 0x36;
        DVar6 = mem_op_1000_1532(param_2);
        DVar7 = DVar6;
    }
    else
    {
        DVar6 = mem_op_1000_1532(param_2);
        uVar5 = DVar6;
        if(((DVar6 >> 0x10) != 0x0) || (0xffef < uVar5))
        {
            pass1_1000_1e61(param_2, 0x8, param_1, &USHORT_1050_1050);
            return false;
        }
        if(0x1fff < uVar5)
        {
            uVar5 = 0x2000;
        }
        while(true)
        {
            uVar9 = uVar5;
            DVar7 = DVar6 + 0x18;
            if(((DVar7 >> 0x10) != 0x0) || (0xfff0 < DVar7))
            {
                DVar7 = 0xfff0;
            }
            BVar3   = mem_op_1000_14f2((param_1 + 0x16) | 0x1000, DVar7, (DVar7 >> 0x10), param_1, &USHORT_1050_1050);
            iStack4 = (DVar6 >> 0x10);
            uStack6 = DVar6;
            if(BVar3 != 0x0)
                break;
            uVar5 = uVar9 >> 0x1;
            if(uVar5 < 0xc)
            {
                UVar4 = pass1_1000_1e61(param_2, 0x2, param_1, &USHORT_1050_1050);
                if(UVar4 == 0x0)
                {
                    return (bool)('\x01' - ((param_1 + 0xa) == 0x0));
                }
                DVar6 = mem_op_1000_1532(param_2);
                uVar5 = uVar9 & 0xfffe;
            }
        }
        uVar8 = pass1_1000_5390(uStack6 - 0x42, iStack4 - (uStack6 < 0x42), 0xc, 0x0);
        uVar5 = uVar8 * 0xc + param_1 + 0x42;
    }
    puVar1  = (param_1 + 0x1e);
    uVar9   = *puVar1;
    *puVar1 = *puVar1 - DVar6;
    piVar2  = (param_1 + 0x20);
    *piVar2 = (*piVar2 - (DVar6 >> 0x10)) - (uVar9 < DVar6);
    if(uVar5 != 0x0)
    {
        uVar10   = 0x0;
        uVar9    = 0xc;
        DVar7    = mem_op_1000_1532(param_2);
        uVar8    = pass1_1000_5390(DVar7 - 0x42, (DVar7 >> 0x10) - (DVar7 < 0x42), uVar9, uVar10);
        uStack14 = uVar8 * 0xc + param_1 + 0x36;
    }
    iStack10 = (DVar7 >> 0x10);
    uStack12 = DVar7;
    puVar1   = (param_1 + 0x1e);
    uVar9    = *puVar1;
    *puVar1  = *puVar1 + uStack12;
    piVar2   = (param_1 + 0x20);
    *piVar2  = *piVar2 + iStack10 + CARRY2(uVar9, uStack12);
    uVar9    = (param_1 + 0xa);
    do
    {
        uVar10         = uVar5;
        (uVar10 + 0x4) = uVar9;
        uVar9          = uVar10;
        uVar5          = uVar10 + 0xc;
    } while(uVar10 < uStack14);
    (param_1 + 0xa) = uVar10;
    return true;
}


i16 mem_op_1000_0308(i16 param_1, i16 param_2)

{
    i16  iVar1;
    i16  iVar2;
    bool bVar3;
    u8   extraout_AH;
    i16 *piVar4;
    u16  unaff_CS;

    if((param_2 + 0xa) == 0x0)
    {
        bVar3 = mem_op_1000_01b0(param_2, unaff_CS);
        if(CONCAT11(extraout_AH, bVar3) == 0x0)
        {
            return 0x0;
        }
    }
    iVar1           = (param_2 + 0xa);
    (param_2 + 0xa) = (iVar1 + 0x4);
    piVar4          = (param_1 * 0x2 + param_2);
    if(*piVar4 == 0x0)
    {
        (iVar1 + 0x6) = iVar1;
        (iVar1 + 0x4) = iVar1;
    }
    else
    {
        iVar2                 = *piVar4;
        (iVar1 + 0x6)         = iVar2;
        (iVar1 + 0x4)         = (iVar2 + 0x4);
        ((iVar2 + 0x4) + 0x6) = iVar1;
        (iVar2 + 0x4)         = iVar1;
    }
    *piVar4 = iVar1;
    return iVar1;
}


u1632 mem_op_1000_03c6(u16 param_1, i16 param_2, u16 param_3, u1632 param_4, u16 param_5, u8 param_6, u168 param_7)

{
    u16  *puVar1;
    i16  *piVar2;
    u16   uVar3;
    u16   uVar4;
    u16  *puVar5;
    u16   UVar6;
    u16   uVar7;
    bool  bVar8;
    DWORD DVar9;
    u16   uStack20;
    u16   uVar9;

    uVar7  = CONCAT11(param_7, param_6);
    uVar3  = param_1 + 0xfff & 0xf000;
    puVar1 = (param_4 + 0x1e);
    uVar4  = uVar3 + *puVar1;
    uVar3  = param_2 + (0xf000 < param_1) + (param_4 + 0x20) + CARRY2(uVar3, *puVar1);
    puVar1 = (param_4 + 0x28);
    bVar8  = uVar3 < *puVar1;
    if((bVar8) || ((bVar8 || uVar3 == *puVar1 && (puVar1 = (param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1))))
    {
        if(param_3 == 0x3)
        {
            uStack20 = ((u8)(-((param_6 & 0x1) != 0x0) >> 0x8) & 0x1 | 0x20) << 0x8;
        }
        else
        {
            uStack20 = 0x1000;
        }
        uStack20 = (param_4 + 0x16) | uStack20;
        mem_op_1000_131c(uStack20, param_1, param_2, param_5);
        if((uVar3 | uStack20) != 0x0)
        {
            puVar5 = mem_op_1000_0308(param_3, param_4);
            if(puVar5 != 0x0)
            {
                puVar5[0x4]             = uStack20;
                puVar5[0x5]             = uVar3;
                uVar9                   = &USHORT_1050_1050;
                &PTR_LOOP_1050_000c     = param_3 | 0xcad0;
                *(u1632 *)0x0           = param_4;
                &PTR_LOOP_1050_0002     = &USHORT_1050_1050;
                *(u16 **)&DAT_1050_0004 = puVar5;
                (&DAT_1050_0004 + 0x2)  = &USHORT_1050_1050;
                &PTR_LOOP_1050_000a     = 0x0;
                DVar9                   = mem_op_1000_1532(param_5);
                UVar6                   = DVar9;
                if(param_3 == 0x1)
                {
                    uVar7 = pass1_1000_0782(param_4, UVar6, 0x0);
                }
                else
                {
                    if(param_3 == 0x3)
                    {
                        pass1_1000_05b4(param_6, 0x0);
                    }
                    else
                    {
                        uVar7 = pass1_1000_09ca(UVar6, (u1632 *)0x0);
                    }
                }
                param_2     = (DVar9 >> 0x10);
                *puVar5     = uVar7;
                puVar5[0x1] = 0x8000;
                puVar1      = (param_4 + 0x1e);
                uVar4       = *puVar1;
                *puVar1     = *puVar1 + UVar6;
                piVar2      = (param_4 + 0x20);
                *piVar2     = *piVar2 + param_2 + CARRY2(uVar4, UVar6);
                return uVar3;
            }
            mem_op_1000_13ce(param_5);
        }
    }
    else
    {
        pass1_1000_1e61(param_5, 0x7, param_4, &USHORT_1050_1050);
    }
    return 0x0;
}


u32 mem_op_1000_0510(u16 param_1, u16 param_2, u16 param_3)

{
    u16  *puVar1;
    i16  *piVar2;
    u8    bVar3;
    i16   iVar4;
    u16   uVar6;
    u16   uVar7;
    u16   uVar8;
    u16   uVar9;
    u16   uVar10;
    bool  bVar11;
    DWORD DVar12;
    long  lVar13;
    u16   uVar5;

    iVar4  = param_2;
    uVar5  = (param_2 + 0x2);
    uVar6  = (param_2 + 0x4);
    bVar3  = *(u8 *)(param_2 + 0xc);
    DVar12 = mem_op_1000_1532(param_3);
    uVar9  = (DVar12 >> 0x10);
    uVar8  = DVar12;
    if(param_1 != 0x0)
    {
        uVar7  = (iVar4 + 0x1e);
        uVar10 = ((iVar4 + 0x20) - uVar9) - (uVar7 < uVar8);
        puVar1 = (iVar4 + 0x24);
        bVar11 = uVar10 < *puVar1;
        if((bVar11 || uVar10 == *puVar1) && ((bVar11 || (uVar7 - uVar8 < (iVar4 + 0x22)))))
        {
            bVar11 = false;
            uVar9  = uVar10;
            goto LAB_1000_0595;
        }
    }
    pass1_1000_0368(uVar6, bVar3 & 0x7, 0x0);
    puVar1  = (s_version__d__d_1050_0012 + 0xc);
    uVar7   = *puVar1;
    *puVar1 = *puVar1 - uVar8;
    piVar2  = s_New_failed_in_Op__Op_1050_0020;
    *piVar2 = (*piVar2 - uVar9) - (uVar7 < uVar8);
    bVar11  = true;
LAB_1000_0595:
    if(bVar11)
    {
        (param_2 + 0xc) = 0x0;
        lVar13          = mem_op_1000_13ce(param_3);
        return CONCAT22((lVar13 >> 0x10), 0x1);
    }
    return uVar9 << 0x10;
}


u32 mem_op_1000_05e2(u16 param_1, i16 param_2, u16 param_3, u16 param_4, u16 param_5)

{
    u16 *puVar1;
    i16  iVar2;
    u16  uVar3;
    u16  uVar4;
    u16  UVar5;
    u32  UVar6;
    bool bVar5;
    u32  uVar6;

    iVar2 = param_2 + (0xffeb < param_1);
    do
    {
        uVar3       = 0x3;
        UVar6._0_1_ = (undefined)param_3;
        UVar6._1_1_ = (u168)(param_3 >> 0x8);
        UVar6._0_2_ = mem_op_1000_03c6(param_1 + 0x14, iVar2, 0x3, param_4, param_5, (undefined)UVar6, UVar6._1_1_);
        if(((u1632)UVar6 | uVar3) != 0x0)
        {
            return CONCAT22((u1632)UVar6, uVar3 + 0x14);
        }
        uVar6  = mem_op_1000_0052(param_4, param_5);
        uVar3  = param_1 + 0x1013 & 0xf000;
        puVar1 = (param_4 + 0x1e);
        uVar4  = uVar3 + *puVar1;
        uVar3  = iVar2 + (0xf000 < param_1 + 0x14) + (param_4 + 0x20) + CARRY2(uVar3, *puVar1);
        puVar1 = (param_4 + 0x28);
        bVar5  = uVar3 < *puVar1;
    } while(((bVar5 || uVar3 == *puVar1) && ((bVar5 || (puVar1 = (param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) && ((uVar6 != 0x0 || (UVar5 = pass1_1000_1e61(param_5, 0x2, param_4, &USHORT_1050_1050), UVar5 != 0x0))));
    return 0x0;
}


u32 mem_1000_0668(u16 param_1)

{
    u32 uVar1;

    uVar1 = mem_op_1000_0510(0x0, 0x0, param_1);
    return uVar1;
}


u16 mem_1000_0670(u16 param_1, i16 *param_2, u16 param_3, u32 *param_4, i16 param_5, WORD param_6)

{
    u16   *puVar1;
    i16   *piVar2;
    u16    UVar3;
    u16    UVar4;
    i16    iVar5;
    u16    UVar6;
    u32    uVar7;
    u16    uVar8;
    u16    uVar9;
    u16    UVar10;
    BOOL16 BVar11;
    u16    uVar12;
    u16    uVar13;
    u16    uVar14;
    DWORD  DVar15;
    DWORD  DVar16;

    UVar3  = param_4;
    UVar4  = (param_4 + 0x2);
    DVar15 = mem_op_1000_1532(param_6);
    UVar6  = param_5 + (0xffeb < param_3);
    uVar7  = *param_4;
    uVar8  = -((param_1 & 0x1) != 0x0) & 0x100 | -((param_1 & 0x4) != 0x0) & 0x400 | (uVar7 + 0x16);
    if(param_2 == 0x0)
    {
        BVar11 = mem_op_1000_14f2(uVar8 | 0x2000, param_3 + 0x14, UVar6, param_4, &USHORT_1050_1050);
        if(BVar11 == 0x0)
        {
            return 0x0;
        }
    }
    else
    {
        iVar5  = (param_4 + 0x1);
        uVar12 = (param_4 + 0x6);
        uVar14 = uVar12;
        do
        {
            uVar13 = uVar14;
            uVar9  = uVar8 | 0x2000;
            mem_op_1000_1408(uVar9, param_3 + 0x14, UVar6, param_6);
            uVar14 = uVar13 | uVar9;
            if(uVar14 != 0x0)
                break;
            UVar10 = pass1_1000_1e61(param_6, 0x2, UVar3, UVar4);
        } while(UVar10 != 0x0);
        if((uVar13 | uVar9) == 0x0)
        {
            (param_2 + 0x2) = 0x0;
            *param_2        = 0x0;
            return 0x0;
        }
        (iVar5 + 0x8)   = uVar9;
        (iVar5 + 0xa)   = uVar13;
        *param_2        = uVar9 + 0x14;
        (param_2 + 0x2) = uVar13;
    }
    DVar16  = mem_op_1000_1532(param_6);
    uVar12  = (DVar16 - DVar15);
    puVar1  = (UVar3 + 0x1e);
    uVar8   = *puVar1;
    *puVar1 = *puVar1 + uVar12;
    piVar2  = (UVar3 + 0x20);
    *piVar2 = *piVar2 + (DVar16 - DVar15 >> 0x10) + CARRY2(uVar8, uVar12);
    return 0x1;
}


Struct99 * pass1_1000_07fc(u16 param_1, u32 param_2)

{
    Struct99 *paVar1;

    if((param_2 + 0x14) != -0x4153)
    {
        pass1_1000_1e61(param_1, 0xa, 0x0, 0x0);
        return (Struct99 *)0x0;
    }
    paVar1 = (Struct99 *)mem_op_1000_0838(0x0, param_1);
    return paVar1;
}


u32 mem_op_1000_0838(u16 param_1, u16 param_2)

{
    u16  *puVar1;
    i16  *piVar2;
    i16   iVar3;
    u16  *puVar4;
    u16   uVar5;
    u16   uVar6;
    u16   UVar7;
    u1632 UVar8;
    i16  *piVar9;
    bool  bVar10;
    u16   uStack6;
    i16  *piStack4;

    piVar9   = *(i16 **)(param_1 + 0x2);
    piStack4 = piVar9;
    if((param_1 + 0x2) == 0x0)
        goto LAB_1000_085b;
    do
    {
        do
        {
            if(*piVar9 != 0x0)
            {
                iVar3  = piVar9[0x5];
                puVar4 = &PTR_LOOP_1050_000e;
                if(puVar4 != 0x0)
                {
                    &PTR_LOOP_1050_000e      = *puVar4;
                    piVar2                   = &PTR_LOOP_1050_000a;
                    *piVar2                  = *piVar2 + 0x1;
                    *(i16 **)(param_1 + 0x2) = piVar9;
                    return CONCAT22(iVar3, puVar4);
                }
                *piVar9 = 0x0;
            }
            piVar9 = piVar9[0x2];
        } while(piVar9 != piStack4);
    LAB_1000_085b:
        if((param_1 + 0x18) == 0x0)
        {
            pass1_1000_1e61(param_2, 0x4, param_1, &USHORT_1050_1050);
            return 0x0;
        }
        uVar5 = (param_1 + 0x1a);
        while(true)
        {
            uStack6 = uVar5;
            uVar5   = 0x1;
            UVar8   = mem_op_1000_03c6(uStack6, 0x0, 0x1, param_1, param_2, 0x0, '\0');
            if((UVar8 | uVar5) != 0x0)
                break;
            uVar5  = (param_1 + 0x1e);
            uVar6  = uVar5 + uStack6;
            uVar5  = (param_1 + 0x20) + CARRY2(uVar5, uStack6);
            puVar1 = (param_1 + 0x28);
            bVar10 = *puVar1 <= uVar5;
            if(bVar10)
            {
                if(bVar10 && uVar5 != *puVar1)
                {
                    return 0x0;
                }
                puVar1 = (param_1 + 0x26);
                if(*puVar1 <= uVar6 && uVar6 != *puVar1)
                {
                    return 0x0;
                }
            }
            uVar5 = uStack6 >> 0x1;
            if(uStack6 >> 0x1 < (param_1 + 0x18) + 0x14U)
            {
                UVar7 = pass1_1000_1e61(param_2, 0x2, param_1, &USHORT_1050_1050);
                uVar5 = uStack6 & 0xfffe;
                if(UVar7 == 0x0)
                {
                    return 0x0;
                }
            }
        }
        piVar9   = *(i16 **)(param_1 + 0x2);
        piStack4 = piVar9[0x2];
    } while(true);
}


u16  pass1_1000_093a(i16 *param_1, u16 param_2, u16 param_3)

{
    i16 *piVar1;

    if(&PTR_LOOP_1050_000c != -0x352f)
    {
        pass1_1000_1e61(param_3, 0xe, 0x0, 0x0);
        return 0x0;
    }
    *param_1 = &PTR_LOOP_1050_000e;
    if(*param_1 == 0x0)
    {
        &DAT_1050_0004 = 0x1;
    }
    *(i16 **)&PTR_LOOP_1050_000e = param_1;
    piVar1                       = &PTR_LOOP_1050_000a;
    *piVar1                      = *piVar1 + -0x1;
    if(*piVar1 == 0x0)
    {
        mem_op_1000_0510(0x1, 0x0, param_3);
    }
    return 0x1;
}


u8 *pass1_1000_09a0(u16 *param_1, u16 param_2)

{
    u8 *puVar1;
    u32 uVar2;

    *param_1 = globals->PTR_LOOP_1050_000e;
    if(PTR_LOOP_1050_000e == 0x0)
    {
        *DAT_1050_0004 = 0x1;
    }
    globals->PTR_LOOP_1050_000a = globals->PTR_LOOP_1050_000a + -0x1;
    puVar1                      = globals->PTR_LOOP_1050_000e;
    globals->PTR_LOOP_1050_000e = param_1;
    if(PTR_LOOP_1050_000a == 0x0)
    {
        uVar2  = mem_op_1000_0510(0x1, 0x0, param_2);
        puVar1 = uVar2;
    }
    return puVar1;
}


long  mem_op_1000_0a48(u8 param_1, u16 param_2, i16 param_3, u32 param_4, u16 param_5)

{
    u16  uVar1;
    u16 *puVar2;
    u16  uVar4;
    u16  uVar3;
    u16  UVar4;
    u32  uVar5;
    u8   in_stack_00000005;
    u16 *puVar1;

    UVar4 = (param_4 >> 0x10);
    if((param_4 + 0x14) == -0x4153)
    {
        if((param_3 == 0x0) && (param_2 <= (s_version__d__d_1050_0012 + 0x6)))
        {
            if(param_2 == 0x0)
            {
                pass1_1000_1e61(param_5, 0x4, param_4, UVar4);
                uVar5 = 0x0;
            }
            else
            {
                uVar5  = mem_op_1000_0838(0x0, param_5);
                uVar3  = (uVar5 >> 0x10);
                puVar2 = uVar5;
                if((uVar5 != 0x0) && ((param_1 & 0x1) != 0x0))
                {
                    uVar1 = (s_version__d__d_1050_0012 + 0x6);
                    for(uVar4 = uVar1 >> 0x1; uVar4 != 0x0; uVar4 = uVar4 - 0x1)
                    {
                        puVar1  = puVar2;
                        puVar2  = puVar2 + 0x1;
                        *puVar1 = 0x0;
                    }
                    if((uVar1 & 0x1) != 0x0)
                    {
                        *puVar2 = 0x0;
                    }
                }
            }
        }
        else
        {
            if((param_3 == 0x0) && (param_2 <= (s_version__d__d_1050_0012 + 0xa)))
            {
                uVar5 = mem_op_1000_0b20(_param_1 & 0xfffd, 0x0, param_2, param_5);
            }
            else
            {
                uVar5 = mem_op_1000_05e2(param_2, param_3, _param_1 & 0xfffd, 0x0, param_5);
            }
        }
        return (long)uVar5;
    }
    pass1_1000_1e61(param_5, 0xa, 0x0, 0x0);
    return 0x0;
}


u32 mem_op_1000_0b20(u16 param_1, u16 param_2, u16 param_3, u16 param_4)

{
    u16  *puVar1;
    u16   uVar2;
    u16   uVar3;
    u16   uVar4;
    u16   uVar5;
    u1632 UVar6;
    u16  *puVar7;
    u16   uVar8;
    bool  bVar9;
    u32   uVar10;
    u16   uStack20;
    u16  *puStack6;

    uVar8    = SUB42(&USHORT_1050_1050, 0x0);
    uVar2    = param_1 & 0x2;
    uVar4    = param_3 + 0x5 & 0xfffc;
    uVar4    = uVar4 - 0x8 & ~-(uVar4 < 0x8);
    uVar5    = uVar4 + 0x8;
    puVar7   = *(u16 **)(uVar2 * 0x2 + param_2);
    uStack20 = param_1;
    puStack6 = puVar7;
    if(puVar7 == 0x0)
        goto LAB_1000_0b64;
    do
    {
        do
        {
            if((uVar5 <= *puVar7) && (uVar10 = pass1_1000_0c32(uVar5, uStack20, 0x0), uVar10 != 0x0))
            {
                *(u16 **)(uVar2 * 0x2 + param_2) = puVar7;
                return uVar10;
            }
            puVar7 = puVar7[0x2];
        } while(puVar7 != puStack6);
    LAB_1000_0b64:
        if((((uStack20 & 0x2) == 0x0) || ((uStack20 & 0x40) != 0x0)) || ((param_2 + 0x32) == 0x0))
        {
        LAB_1000_0b9e:
            if(((uStack20 & 0x10) != 0x0) || (uVar3 = uVar2, UVar6 = mem_op_1000_03c6((param_2 + 0x1a), 0x0, uVar2, param_2, param_4, 0x0, '\0'), (UVar6 | uVar3) == 0x0))
            {
                if((uStack20 & 0x20) == 0x0)
                {
                    uVar2  = uVar4 + 0x1007 & 0xf000;
                    puVar1 = (param_2 + 0x1e);
                    uVar4  = uVar2 + *puVar1;
                    uVar2  = (param_2 + 0x20) + CARRY2(uVar2, *puVar1);
                    puVar1 = (param_2 + 0x28);
                    bVar9  = uVar2 < *puVar1;
                    if((bVar9 || uVar2 == *puVar1) && ((bVar9 || (puVar1 = (param_2 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1))))
                    {
                        uVar10 = mem_op_1000_05e2(uVar5, 0x0, uStack20, param_2, param_4);
                        return uVar10;
                    }
                }
                return 0x0;
            }
        }
        else
        {
            param_4 = 0x1000;
            uVar3   = (**(param_2 + 0x32))();
            if(uVar3 < uVar5)
                goto LAB_1000_0b9e;
            uStack20 = uStack20 | 0x40;
        }
        puVar7   = *(u16 **)(uVar2 * 0x2 + param_2);
        puStack6 = puVar7[0x2];
    } while(true);
}
