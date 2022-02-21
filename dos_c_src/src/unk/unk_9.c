
ushort *__stdcall16far pass1_1028_0b64(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0xbbc;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}

ushort *__stdcall16far pass1_1028_0c50(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(undefined2 *)(param_1 + 0x20)       = 0x0;
    *(undefined2 *)(param_1 + 0x22)       = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = (int)s_480_bmp_1050_1721 + 0x3;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}

void __stdcall16far pass1_1028_0c84(ulong param_1, ulong param_2, int param_3, ushort param_4)

{
    code      **ppcVar1;
    ushort      uVar2;
    undefined4 *puVar3;
    ushort      extraout_DX;
    uint        uVar4;
    ulong       uVar5;
    undefined4 *puVar6;
    ulong      *puVar7;
    byte        bStack55;
    undefined   local_32[0xa];
    undefined4  uStack40;
    undefined4  uStack36;
    undefined4 *puStack28;
    undefined4  local_1a;
    int         iStack22;
    ushort      uStack20;
    int         iStack18;
    ushort      uStack16;
    int         iStack14;
    undefined4  local_c;
    int         iStack8;
    ulong       uStack6;

    pass1_1028_b58e(param_1);
    uStack6   = CONCAT22(extraout_DX, param_3);
    local_c   = *(undefined4 *)(param_3 + 0xc);
    iStack18  = *(int *)(param_3 + 0x10);
    puStack28 = &local_c;
    uStack16  = extraout_DX;
    iStack14  = iStack18;
    iStack8   = iStack18;
    pass1_1028_bab6(param_1, iStack18, extraout_DX);
    uVar2    = pass1_1030_2fac(CONCAT22(uStack16, iStack18));
    local_1a = local_c;
    iStack22 = iStack8;
    uStack36 = CONCAT22(uStack36._2_2_, &local_1a);
    iStack14 = iStack14 + 0x1;
    uStack20 = uVar2;
    if(iStack14 <= (int)uVar2)
    {
        puVar7   = (ulong *)CONCAT22(param_4, local_32);
        iStack22 = iStack14;
        uVar5    = pass1_1028_bb24(param_1);
        uVar4    = (uint)(uVar5 >> 0x10);
        puVar3   = &local_1a;
        pass1_1030_64ce(param_4, puVar3, uVar4, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_4, puVar3), uVar5 & 0xffff | (ulong)uVar4 << 0x10, puVar7);
        uStack40 = *puVar3;
        uVar4    = *(uint *)((int)puVar3 + 0x2);
        bStack55 = (byte)((ulong)uStack40 >> 0x18);
        uVar2    = (ushort)bStack55;
        uStack36 = uStack40;
        if(bStack55 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack40, uVar4);
            puVar6  = (undefined4 *)struct_op_1030_73a8(CONCAT22(uVar4, uVar2));
            uVar2   = (ushort)puVar6;
            ppcVar1 = (code **)((int)*puVar6 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b46e(param_1, param_2, uVar2);
    fn_ptr_1030_7296(uStack6);
    return;
}


ushort __stdcall16far pass1_1028_0d80(ulong param_1)

{
    ushort uVar1;
    uint   uVar2;

    uVar2 = (uint)(param_1 >> 0x10);
    uVar1 = *(ushort *)((int)param_1 + 0x20);
    pass1_1028_1646(param_1 & 0xffff | (ulong)uVar2 << 0x10);
    return uVar1;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_0d9c(ulong param_1, int param_2, ushort param_3)

{
    code      **ppcVar1;
    undefined4 *puVar2;
    uint        uVar3;
    ushort      uVar4;
    BOOL16      BVar5;
    ushort      extraout_DX;
    uint        uVar6;
    ulong       uVar7;
    ulong      *puVar8;
    undefined4  uStack58;
    undefined   local_32[0x6];
    undefined4 *puStack44;
    undefined4  uStack40;
    undefined4  uStack36;
    undefined4 *puStack28;
    undefined4  local_1a;
    int         iStack22;
    ushort      uStack20;
    int         iStack18;
    ushort      uStack16;
    int         iStack14;
    undefined4  local_c;
    int         iStack8;
    int         iStack6;
    ushort      uStack4;

    pass1_1028_b514(param_1);
    pass1_1028_b58e(param_1);
    local_c   = *(undefined4 *)(param_2 + 0xc);
    iStack18  = *(int *)(param_2 + 0x10);
    puStack28 = &local_c;
    uStack16  = extraout_DX;
    iStack14  = iStack18;
    iStack8   = iStack18;
    iStack6   = param_2;
    uStack4   = extraout_DX;
    pass1_1028_bab6(param_1, iStack18, extraout_DX);
    uStack20 = pass1_1030_2fac(CONCAT22(uStack16, iStack18));
    local_1a = local_c;
    uStack36 = CONCAT22(uStack36._2_2_, &local_1a);
    iStack22 = iStack14 + 0x1;
    if(iStack22 <= (int)uStack20)
    {
        puVar8   = (ulong *)CONCAT22(param_3, local_32);
        iStack14 = iStack22;
        uVar7    = pass1_1028_bb24(param_1);
        uVar6    = (uint)(uVar7 >> 0x10);
        puVar2   = &local_1a;
        pass1_1030_64ce(param_3, puVar2, uVar6, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_3, puVar2), uVar7 & 0xffff | (ulong)uVar6 << 0x10, puVar8);
        uStack40       = *puVar2;
        uVar6          = *(uint *)((int)puVar2 + 0x2);
        uStack58._3_1_ = (byte)((ulong)uStack40 >> 0x18);
        uVar3          = (uint)uStack58._3_1_;
        if(uStack58._3_1_ != 0x0)
        {
            uStack36 = uStack40;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack40, uVar6);
            uStack58 = CONCAT22(uVar6, uVar3);
            uVar4    = pass1_1030_6fa0(CONCAT22(uVar6, uVar3));
            BVar5    = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar4, 0x13);
            if(BVar5 != 0x0)
            {
                puStack44 = (undefined4 *)struct_op_1030_73a8(uStack58);
                ppcVar1   = (code **)((int)*puStack44 + 0x24);
                (**ppcVar1)();
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_0ea6(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    int         *piVar1;
    BOOL16       BVar2;
    ushort       uVar3;
    astruct_597 *iVar3;
    uint         uVar4;

    uVar4 = (uint)((ulong)param_1 >> 0x10);
    iVar3 = (astruct_597 *)param_1;
    if(iVar3->field_0xc != 0x10)
    {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar3->field_0xc, 0x13);
        if(BVar2 == 0x0)
        {
            BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar3->field_0xc, 0x2);
            if(((BVar2 != 0x0) && (iVar3->field_0x12 != 0x7)) && (iVar3->field_0x12 != 0x4))
            {
                uVar3 = pass1_1028_1556((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10, BVar2, param_2, param_5);
                if(uVar3 == 0x0)
                    goto LAB_1028_0f0a;
                if(iVar3->field_0x12 == 0x9)
                {
                    iVar3->field_0x12 = 0x5;
                }
            }
        }
        else
        {
            if(iVar3->field_0x22 < 0x1)
            {
                if((iVar3->field_0x12 != 0x5) && (iVar3->field_0x12 != 0x6))
                {
                    return;
                }
                fn_ptr_1000_17ce(iVar3->field_0x14, 0x1000);
                iVar3->field_0x14 = (astruct_18 *)0x0;
            LAB_1028_0f0a:
                iVar3->field_0x12 = 0x9;
                return;
            }
        }
        pass1_1028_be2a(param_1, param_3, param_4, 0x1008, param_5);
        if(iVar3->field_0x12 == 0x5)
        {
            piVar1  = &iVar3->field_0x22;
            *piVar1 = *piVar1 + -0x1;
            pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar4 << 0x10);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_0fa4(ulong *param_1, uchar *param_2, ushort param_3, ushort param_4, ushort param_5, ushort param_6)

{
    BOOL16     BVar1;
    int        iVar2;
    undefined2 uVar3;
    ushort    *puVar4;
    ulong      uVar5;
    undefined2 uVar6;
    undefined2 uVar7;
    int        iVar8;

    pass1_1028_be9e(param_1, param_3, param_4, param_5, param_6);
    puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_6, param_2, param_4);
    iVar8  = *(int *)((int)puVar4 + 0x82);
    uVar3  = (undefined2)((ulong)param_1 >> 0x10);
    iVar2  = (int)param_1;
    if(*(int *)(iVar2 + 0x12) == 0x5)
    {
        BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)(iVar2 + 0xc), 0x2);
        if((BVar1 != 0x0) && ((PTR_LOOP_1050_4fbc == (undefined *)0x0 || (iVar8 != 0x0))))
        {
            PTR_LOOP_1050_4fbc = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
            uVar7              = 0x0;
            iVar8              = 0x4;
            uVar6              = 0x1;
            uVar5              = pass1_1028_b58e((ulong)param_1);
            pass1_1030_7c50(uVar5, CONCAT22(uVar7, uVar6), iVar8, (uint)uVar5, (uchar *)(uVar5 >> 0x10));
        }
        *(undefined2 *)(iVar2 + 0x22) = 0x64;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far pass1_1028_1024(ulong param_1, int param_2, ushort param_3, ushort param_4)

{
    BOOL16      BVar1;
    undefined4 *puVar2;
    uint        uVar3;
    uint        uVar4;
    uint        uVar5;
    ulong       uVar6;
    int         iStack26;
    int         iStack24;
    undefined4  local_16;
    int         iStack18;
    undefined2  uStack16;
    undefined2  uStack14;
    ulong       uStack12;
    ushort      uStack8;
    int         iStack6;
    ushort      uStack4;

    pass1_1028_bab6(param_1, param_2, param_3);
    iStack6  = param_2;
    uStack4  = param_3;
    uStack8  = pass1_1030_2fac(CONCAT22(param_3, param_2));
    uStack12 = pass1_1028_bb24(param_1);
    uVar6    = pass1_1028_b58e(param_1);
    uStack14 = (undefined2)(uVar6 >> 0x10);
    local_16 = *(undefined4 *)((int)uVar6 + 0xc);
    iStack26 = 0x0;
    iStack24 = 0x0;
    while(true)
    {
        if((int)uStack8 < iStack26)
        {
            return iStack24;
        }
        iStack18 = iStack26;
        puVar2   = &local_16;
        pass1_1030_627e(param_4, (uint)puVar2, (uint)(uVar6 >> 0x10), _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_4, puVar2), uStack12);
        uStack16 = (undefined2)uVar6;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)puVar2, (uint)(uVar6 >> 0x10));
        uStack16 = (undefined2)uVar6;
        if(((uint)(uVar6 >> 0x10) | (uint)puVar2) == 0x0)
            break;
        uVar6 = struct_op_1030_73a8(uVar6 & 0xffff0000 | ZEXT24(puVar2));
        uVar4 = (uint)(uVar6 >> 0x10);
        uVar3 = (uint)uVar6;
        uVar5 = uVar4 | uVar3;
        if(uVar6 == 0x0)
        {
            return iStack24;
        }
        BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)(uVar3 + 0xc), 0x13);
        uVar6 = CONCAT22(uVar5, uStack16);
        if((BVar1 != 0x0) && (*(int *)(uVar3 + 0x12) == 0x5))
        {
            iStack24 = iStack24 + 0x1;
        }
        iStack26 = iStack26 + 0x1;
    }
    return iStack24;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far pass1_1028_1106(ulong param_1, int param_2, ushort param_3, ushort param_4)

{
    BOOL16      BVar1;
    undefined4 *puVar2;
    uint        uVar3;
    uint        uVar4;
    ulong       uVar5;
    int         iStack26;
    int         iStack24;
    undefined4  local_16;
    int         iStack18;
    undefined2  uStack16;
    undefined2  uStack14;
    ulong       uStack12;
    ushort      uStack8;
    int         iStack6;
    ushort      uStack4;

    pass1_1028_bab6(param_1, param_2, param_3);
    iStack6  = param_2;
    uStack4  = param_3;
    uStack8  = pass1_1030_2fac(CONCAT22(param_3, param_2));
    uStack12 = pass1_1028_bb24(param_1);
    uVar5    = pass1_1028_b58e(param_1);
    uStack14 = (undefined2)(uVar5 >> 0x10);
    local_16 = *(undefined4 *)((int)uVar5 + 0xc);
    iStack26 = 0x0;
    iStack24 = 0x0;
    while(true)
    {
        if((int)uStack8 < iStack26)
        {
            return iStack24;
        }
        iStack18 = iStack26;
        puVar2   = &local_16;
        pass1_1030_627e(param_4, (uint)puVar2, (uint)(uVar5 >> 0x10), _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_4, puVar2), uStack12);
        uStack16 = (undefined2)uVar5;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)puVar2, (uint)(uVar5 >> 0x10));
        uStack16 = (undefined2)uVar5;
        if(((uint)(uVar5 >> 0x10) | (uint)puVar2) == 0x0)
            break;
        uVar5 = struct_op_1030_73a8(uVar5 & 0xffff0000 | ZEXT24(puVar2));
        uVar3 = (uint)(uVar5 >> 0x10);
        uVar4 = uVar3 | (uint)uVar5;
        if(uVar5 == 0x0)
        {
            return iStack24;
        }
        BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((uint)uVar5 + 0xc), 0x13);
        uVar5 = CONCAT22(uVar4, uStack16);
        if(BVar1 != 0x0)
        {
            iStack24 = iStack24 + 0x1;
        }
        iStack26 = iStack26 + 0x1;
    }
    return iStack24;
}


bool __stdcall16far pass1_1028_11de(ulong param_1)

{
    undefined4 uVar1;

    uVar1 = pass1_1028_b58e(param_1);
    return *(int *)((int)uVar1 + 0x10) == 0x0;
}


ushort __stdcall16far pass1_1028_12be(ulong param_1, ulong *param_2, ushort param_3)

{
    int        *piVar1;
    uint        uVar2;
    code      **ppcVar3;
    bool        bVar4;
    undefined   extraout_AH;
    ushort      uVar5;
    undefined4 *puVar6;
    ulong       uVar7;
    ulong       uVar8;
    ushort      uStack8;

    bVar4 = pass1_1028_11de(param_1);
    if(CONCAT11(extraout_AH, bVar4) == 0x0)
    {
        puVar6  = (undefined4 *)pass1_1028_121e(param_1, param_3);
        ppcVar3 = (code **)((int)*puVar6 + 0x40);
        uVar5   = (**ppcVar3)();
        return uVar5;
    }
    *param_2 = 0x0;
    uVar7    = pass1_1028_b58e(param_1);
    uStack8  = 0x4;
    uVar8    = uVar7;
    do
    {
        uVar8            = pass1_1030_7c28(uVar7, uStack8, (uint)uVar8, (uint)(uVar8 >> 0x10), param_3);
        uVar2            = *(uint *)param_2;
        *(uint *)param_2 = *(int *)param_2 + (uint)uVar8;
        piVar1           = (int *)((int)param_2 + 0x2);
        *piVar1          = *piVar1 + (int)(uVar8 >> 0x10) + (uint)CARRY2(uVar2, (uint)uVar8);
        uStack8          = uStack8 + 0x1;
    } while((int)uStack8 < 0xe);
    if(0x1f4 < *param_2)
    {
        return 0x1;
    }
    return 0x0;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_134a(ulong *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int       *piVar1;
    code     **ppcVar2;
    BOOL16     BVar3;
    long      *plVar4;
    undefined2 uVar5;
    undefined2 uVar6;
    ulong      uVar7;
    long       lStack26;
    int        iStack22;
    ulong      uStack18;
    undefined4 uStack10;
    long       local_6;

    uVar6 = (undefined2)((ulong)param_1 >> 0x10);
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((int)param_1 + 0xc), 0x13);
    if(BVar3 != 0x0)
    {
        plVar4  = &local_6;
        ppcVar2 = (code **)((int)*param_1 + 0x40);
        (**ppcVar2)(0x1008, param_1, plVar4, param_4);
        if(plVar4 != (long *)0x0)
        {
            piVar1  = (int *)((int)param_1 + 0x22);
            *piVar1 = *piVar1 + 0x1;
            return;
        }
        uStack10 = 0x1f4 - local_6;
        uVar7    = pass1_1028_121e((ulong)param_1, param_4);
        uVar5    = (undefined2)(uVar7 >> 0x10);
        uVar6    = (undefined2)uVar7;
        pass1_1028_b58e(uVar7);
        uStack18 = CONCAT22(uVar5, uVar6);
        for(iStack22 = 0x0; iStack22 < 0xa; iStack22 = iStack22 + 0x1)
        {
            uStack10._0_2_ = *(uint *)(iStack22 * 0x2 + 0x4fbe);
            uStack10._2_2_ = (uchar *)((int)(uint)uStack10 >> 0xf);
            if(uStack10 < (int)(uint)uStack10)
            {
            }
            lStack26 = CONCAT22(uStack10._2_2_, (uint)uStack10);
            pass1_1030_7ddc(uStack18, CONCAT13((char)((uint)uStack10._2_2_ >> 0x8), CONCAT12((char)uStack10._2_2_, (uint)uStack10)), iStack22 + 0x4, (uint)uStack10, uStack10._2_2_, param_2, param_3, param_4);
            uStack10 = uStack10 - lStack26;
            if(uStack10 < 0x1)
            {
                return;
            }
        }
    }
    return;
}


int __stdcall16far pass1_1028_1416(ulong param_1, ushort param_2, ushort param_3)

{
    bool      bVar1;
    undefined extraout_AH;
    int       iVar2;
    uint      uVar3;
    ulong     uVar4;

    bVar1 = pass1_1028_11de(param_1);
    if(CONCAT11(extraout_AH, bVar1) == 0x0)
    {
        uVar4 = pass1_1028_121e(param_1, param_3);
        uVar3 = (uint)(uVar4 >> 0x10);
        iVar2 = pass1_1028_1416(uVar4 & 0xffff | (ulong)uVar3 << 0x10, uVar3, param_3);
        return iVar2;
    }
    iVar2 = pass1_1028_1024(param_1, CONCAT11(extraout_AH, bVar1), param_2, param_3);
    return iVar2 * 0xf;
}


ushort __stdcall16far pass1_1028_1556(ulong param_1, int param_2, ushort param_3, ushort param_4)

{
    int         iVar1;
    undefined4 *puVar2;
    uint        uVar3;
    BOOL16      BVar4;
    uint        uVar5;
    uint        uVar6;
    ulong       uVar7;
    int         iStack26;
    undefined4  local_16;
    int         iStack18;
    undefined2  uStack16;
    undefined2  uStack14;
    ulong       uStack12;
    ushort      uStack8;
    int         iStack6;
    ushort      uStack4;

    pass1_1028_bab6(param_1, param_2, param_3);
    iStack6  = param_2;
    uStack4  = param_3;
    uStack8  = pass1_1030_2fac(CONCAT22(param_3, param_2));
    uStack12 = pass1_1028_bb24(param_1);
    uVar7    = pass1_1028_b58e(param_1);
    uStack14 = (undefined2)(uVar7 >> 0x10);
    local_16 = *(undefined4 *)((int)uVar7 + 0xc);
    iStack26 = 0x1;
    while(true)
    {
        if((int)uStack8 < iStack26)
        {
            return 0x0;
        }
        iStack18 = iStack26;
        puVar2   = &local_16;
        pass1_1030_627e(param_4, (uint)puVar2, (uint)(uVar7 >> 0x10), _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_4, puVar2), uStack12);
        uStack16 = (undefined2)uVar7;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)puVar2, (uint)(uVar7 >> 0x10));
        uStack16 = (undefined2)uVar7;
        if(((uint)(uVar7 >> 0x10) | (uint)puVar2) == 0x0)
        {
            return 0x0;
        }
        uVar7 = struct_op_1030_73a8(uVar7 & 0xffff0000 | ZEXT24(puVar2));
        uVar5 = (uint)(uVar7 >> 0x10);
        uVar3 = (uint)uVar7;
        uVar6 = uVar5 | uVar3;
        if(uVar7 == 0x0)
        {
            return 0x0;
        }
        iVar1 = *(int *)(uVar3 + 0xc);
        BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, iVar1, 0x13);
        uVar7 = CONCAT22(uVar6, uStack16);
        if((BVar4 == 0x0) && (iVar1 != 0x75))
            break;
        if(*(int *)(uVar3 + 0x12) != 0x9)
        {
            return 0x1;
        }
        iStack26 = iStack26 + 0x1;
    }
    return 0x0;
}


astruct_409 *__stdcall16far pass1_1028_1646(ulong param_1)

{
    astruct_409 *paVar1;
    astruct_409 *uVar2;
    undefined2   uVar3;

    uVar3  = (undefined2)(param_1 >> 0x10);
    uVar2  = (astruct_409 *)param_1;
    paVar1 = (astruct_409 *)(uVar2->field_0x20 + -0x4);
    if(paVar1 < (astruct_409 *)&DAT_1050_0009)
    {
        switch(paVar1)
        {
        case(astruct_409 *)0x0:
            uVar2->field_0x20 = 0x5;
            break;
        case(astruct_409 *)0x1:
            uVar2->field_0x20 = 0x6;
            break;
        case(astruct_409 *)0x2:
            uVar2->field_0x20 = 0x7;
            break;
        case(astruct_409 *)0x3:
            uVar2->field_0x20 = 0x8;
            break;
        case(astruct_409 *)0x4:
            uVar2->field_0x20 = 0x9;
            break;
        case(astruct_409 *)0x5:
            uVar2->field_0x20 = 0xa;
            return uVar2;
        case(astruct_409 *)0x6:
            uVar2->field_0x20 = 0xb;
            return uVar2;
        case(astruct_409 *)0x7:
            uVar2->field_0x20 = 0xc;
            return uVar2;
        case(astruct_409 *)0x8:
            uVar2->field_0x20 = 0xd;
            return uVar2;
        }
        return uVar2;
    }
    uVar2->field_0x20 = 0x4;
    return paVar1;
}


ushort *__stdcall16far pass1_1028_17ae(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1030_dcc2(param_1, param_2, param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0x1b54;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1028_17d8(ushort param_1, ushort param_2, ushort param_3)

{
    undefined2 extraout_DX;

    pass1_1030_df0c(CONCAT22(param_2, param_1), param_3);
    pass1_1028_b58e(CONCAT22(param_2, param_1));
    pass1_1038_57dc(*(ulong *)(param_3 + 0x2e), 0x1, 0x3);
    return;
}


void __stdcall16far pass1_1028_1812(ulong *param_1, ushort param_2)

{
    pass1_1028_bdac(param_1, 0x2, param_2);
    return;
}


ushort *__stdcall16far pass1_1020_e91e(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1030_dcc2(param_1, param_2, param_3, param_4, param_5);
    *(undefined2 *)(param_1 + 0x24)       = 0x0;
    *(ushort *)CONCAT22(param_2, param_1) = 0xeef6;
    *(undefined2 *)(param_1 + 0x2)        = 0x1020;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1020_e9d4(ushort param_1, ushort param_2, ushort param_3)

{
    undefined2 extraout_DX;

    pass1_1030_df0c(CONCAT22(param_2, param_1), param_3);
    pass1_1028_b58e(CONCAT22(param_2, param_1));
    pass1_1038_57dc(*(ulong *)(param_3 + 0x2e), 0x1, 0x1);
    return;
}


void __stdcall16far pass1_1020_ea0e(ulong *param_1)

{
    pass1_1028_bdac(param_1, 0x1, (ushort)&USHORT_1050_1028);
    return;
}

void __stdcall16far pass1_1020_ecb0(ulong param_1, int param_2, ushort param_3)

{
    undefined4 uVar1;
    int        iVar2;
    uint       uVar3;
    ushort     unaff_SS;
    undefined2 uStack8;

    uVar3 = (uint)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    uVar1 = *(undefined4 *)(iVar2 + 0x8);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    if(*(int *)(iVar2 + 0x12) == 0x1)
    {
        switch(*(undefined2 *)(param_2 + 0x14))
        {
        case 0x2:
        case 0x7:
            uStack8 = 0x2;
            break;
        case 0x3:
        case 0x8:
            uStack8 = 0x3;
            break;
        default:
            uStack8 = *(undefined2 *)(param_2 + 0x14);
            break;
        case 0x5:
        case 0x6:
            uStack8 = 0x1;
        }
        *(undefined2 *)(iVar2 + 0x14) = uStack8;
        return;
    }
    pass1_1028_bf22(param_1 & 0xffff | (ulong)uVar3 << 0x10, (uchar *)param_3, unaff_SS);
    return;
}

void __stdcall16far pass1_1020_ed3c(ulong param_1, int param_2, ushort param_3, uchar param_4)

{
    int        *piVar1;
    ushort      uVar2;
    undefined2  extraout_DX;
    undefined2  extraout_DX_00;
    int         iVar3;
    undefined2  uVar4;
    undefined   local_138[0x112];
    ulong       uStack38;
    undefined4 *puStack30;
    ulong       uStack28;
    undefined4  uStack24;
    ushort      uStack20;
    int         local_12;
    undefined   local_10[0x2];
    undefined   local_e[0x2];
    undefined4  local_c;
    undefined2  uStack8;
    int         iStack6;
    undefined2  uStack4;

    uVar4   = (undefined2)(param_1 >> 0x10);
    iVar3   = (int)param_1;
    piVar1  = (int *)(iVar3 + 0x14);
    *piVar1 = *piVar1 + -0x1;
    if(*piVar1 == 0x0)
    {
        *(undefined2 *)(iVar3 + 0x12) = 0x0;
        pass1_1028_b58e(param_1);
        local_c   = *(undefined4 *)(param_2 + 0xc);
        uStack8   = *(undefined2 *)(param_2 + 0x10);
        puStack30 = &local_c;
        iStack6   = param_2;
        uStack4   = extraout_DX;
        pass1_1008_3eb4((ushort *)CONCAT22(param_3, &local_c), (ushort *)CONCAT22(param_3, &local_12), (ushort *)CONCAT22(param_3, local_10), (ushort *)CONCAT22(param_3, local_e));
        if(local_12 < 0x1)
        {
            puStack30 = (undefined4 *)0x5;
        }
        else
        {
            puStack30 = (undefined4 *)0x6;
        }
        *(undefined2 *)(iStack6 + 0x14) = puStack30;
        if(local_12 < 0x1)
        {
            uVar2 = 0x5;
        }
        else
        {
            uVar2 = 0x9;
        }
        uStack20 = uVar2;
        pass1_1020_ee3a(param_1, uVar2, uVar2, param_3, param_4);
        pass1_1028_b58e(param_1);
        uStack24 = CONCAT22(extraout_DX_00, uVar2);
        uStack28 = *(ulong *)(uVar2 + 0x2e);
        pass1_1038_5804(uStack28, 0x1, 0x1);
        if(0x0 < *(int *)(iVar3 + 0x24))
        {
            uStack38 = *(ulong *)((int)uStack28 + 0x4);
            pass1_1028_68de((astruct_100 *)CONCAT22(param_3, local_138), *(ushort *)(iVar3 + 0x24), uStack38, param_4, param_3);
            fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_3, local_138));
        }
    }
    return;
}

void __stdcall16far pass1_1020_ef5e(ushort *param_1)

{
    *param_1                            = 0x0;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    pass1_1028_b418(param_1);
    return;
}

void __stdcall16far pass1_1028_0138(ushort *param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    uVar5                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar4                        = (int)param_1;
    *param_1                     = 0x8ec;
    *(undefined2 *)(iVar4 + 0x2) = (int)&USHORT_1050_1028;
    puVar1                       = (undefined4 *)*(uint *)(iVar4 + 0x22);
    uVar2                        = *(uint *)(iVar4 + 0x24);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    pass1_1028_b418(param_1);
    return;
}

void __stdcall16far pass1_1028_01ec(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)((ulong)param_1 >> 0x10);
    iVar2 = (int)param_1;
    if((*(int *)(iVar2 + 0x12) == 0x6) || (*(int *)(iVar2 + 0x12) == 0x5))
    {
        uVar1 = *(undefined4 *)(iVar2 + 0x14);
        uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
        iVar2 = (int)uVar1;
        if((*(int *)(iVar2 + 0xa6) == 0x14) || (*(int *)(iVar2 + 0xa8) == 0x10))
        {
            pass1_1028_bdac(param_1, 0x6, param_4);
            return;
        }
        pass1_1028_be2a(param_1, param_2, param_3, param_4, param_5);
    }
    return;
}


ushort __stdcall16far pass1_1028_04ee(ulong param_1, ulong *param_2, ushort param_3)

{
    int       *piVar1;
    uint       uVar2;
    uint       uVar3;
    undefined2 uVar4;
    long       lVar5;
    undefined  local_a[0x8];

    *param_2 = 0x0;
    pass1_1008_5784((ulong *)CONCAT22(param_3, local_a), *(ulong *)((int)param_1 + 0x22));
    do
    {
        lVar5 = pass1_1008_5b12(local_a, param_3);
        if(lVar5 == 0x0)
        {
            return 0x0;
        }
        uVar2            = *(uint *)((int)lVar5 + 0xc);
        uVar4            = (undefined2)((ulong)param_2 >> 0x10);
        uVar3            = *(uint *)param_2;
        *(uint *)param_2 = *(int *)param_2 + uVar2;
        piVar1           = (int *)((int)param_2 + 0x2);
        *piVar1          = *piVar1 + (uint)CARRY2(uVar3, uVar2);
    } while((*(int *)((int)param_2 + 0x2) == 0x0) && (*(uint *)param_2 < 0x1e));
    return 0x1;
}


void __stdcall16far pass1_1028_0550(ulong *param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    uint       uVar1;
    ulong      uVar2;
    undefined2 uVar3;
    undefined2 uVar4;
    int        iVar5;

    pass1_1028_be9e(param_1, param_2, param_3, param_4, param_5);
    uVar1 = (uint)((ulong)param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x12) == 0x5)
    {
        uVar4 = 0x0;
        iVar5 = 0x4;
        uVar3 = 0x1;
        uVar2 = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
        pass1_1030_7c50(uVar2, CONCAT22(uVar4, uVar3), iVar5, (uint)uVar2, (uchar *)(uVar2 >> 0x10));
    }
    return;
}

void __stdcall16far pass1_1028_081e(ulong param_1, int param_2, ushort param_3)

{
    int       *piVar1;
    int        iVar2;
    undefined2 uVar3;
    undefined4 uVar4;
    uint       uVar5;
    int        iVar6;
    undefined2 uVar7;

    pass1_1028_b58e(param_1);
    uVar4   = *(undefined4 *)(param_2 + 0x2e);
    iVar2   = *(int *)((int)uVar4 + 0x18);
    uVar7   = (undefined2)(param_1 >> 0x10);
    iVar6   = (int)param_1;
    piVar1  = (int *)(iVar6 + 0x20);
    *piVar1 = *piVar1 + 0x1;
    uVar5   = *_PTR_LOOP_1050_65e2;
    uVar3   = *(undefined2 *)((int)_PTR_LOOP_1050_65e2 + 0x2);
    if(iVar2 < 0xfa)
    {
        uVar5 = uVar5 & 0x1;
    }
    else
    {
        if(0x1c1 < iVar2)
        {
            if(iVar2 < 0x226)
            {
                return;
            }
            if((iVar2 < 0x2ee) && (CONCAT22(uVar3, uVar5) % 0x3 != 0x0))
            {
                return;
            }
            piVar1  = (int *)(iVar6 + 0x20);
            *piVar1 = *piVar1 + 0x1;
            return;
        }
        uVar5 = (uint)((qword)CONCAT22(uVar3, uVar5) % 0x3);
    }
    if(uVar5 != 0x0)
    {
        return;
    }
    piVar1  = (int *)(iVar6 + 0x20);
    *piVar1 = *piVar1 + -0x1;
    return;
}


ushort *__stdcall16far pass1_1020_d888(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0xd8ec;
    *(undefined2 *)(param_1 + 0x2)        = 0x1020;
    return (ushort *)CONCAT22(param_2, param_1);
}

void __stdcall16far pass1_1020_d9fa(ulong param_1, ushort param_2)

{
    undefined2 extraout_DX;

    if(*(int *)((int)param_1 + 0xc) != 0x79)
    {
        pass1_1030_df0c(param_1, param_2);
        pass1_1028_b58e(param_1);
        pass1_1038_57dc(*(ulong *)(param_2 + 0x2e), 0x1, 0x2);
    }
    return;
}


void __stdcall16far pass1_1020_da3c(ulong *param_1)

{
    pass1_1028_bdac(param_1, 0x2, (ushort)&USHORT_1050_1028);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_da4e(ulong *param_1, ushort *param_2, ulong param_3, ulong param_4, ushort param_5, int param_6, ushort param_7)

{
    code      **ppcVar1;
    undefined4 *puVar2;
    uint        uVar3;
    BOOL16      BVar4;
    uchar      *extraout_DX;
    uchar      *puVar5;
    uchar      *extraout_DX_00;
    uint        uVar6;
    ulong       uVar7;
    ulong       uVar8;
    ushort      uVar9;
    ushort      uVar11;
    ushort     *puVar10;
    ulong       uVar12;
    byte        bStack31;
    undefined4  local_e;
    ushort      uStack10;
    uint        uStack8;
    undefined4  uStack6;

    puVar2 = &local_e;
    pass1_1030_64ce(param_7, puVar2, param_5, _PTR_LOOP_1050_5740, param_2, param_4, (ulong *)CONCAT22(param_7, puVar2));
    uStack6  = *puVar2;
    uVar6    = *(uint *)((int)puVar2 + 0x2);
    bStack31 = (byte)((ulong)uStack6 >> 0x18);
    uVar3    = (uint)bStack31;
    if(bStack31 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack6, uVar6);
        uVar7 = struct_op_1030_73a8(CONCAT22(uVar6, uVar3));
        uVar6 = (uint)(uVar7 >> 0x10);
        uVar3 = (uint)uVar7;
        if(*(int *)(uVar3 + 0xc) == 0x10)
        {
            PTR_LOOP_1050_50ca = (undefined *)0x6a9;
            return;
        }
    }
    uVar9  = (ushort)param_1;
    uVar11 = (ushort)((ulong)param_1 >> 0x10);
    pass1_1028_c7b6(param_7, uVar6, uVar9, uVar11, param_2, param_4);
    uVar8   = (ulong)param_1 & 0xffff | (ulong)uVar11 << 0x10;
    ppcVar1 = (code **)((int)*param_1 + 0x60);
    puVar10 = param_2;
    uVar7   = param_3;
    uVar12  = param_4;
    uStack8 = uVar3;
    (**ppcVar1)();
    if(((uVar3 != 0x0) && (puVar5 = extraout_DX, pass1_1028_c23e(uVar9, uVar11, param_2, param_3, param_4, uVar3, (uint)extraout_DX, param_7), uVar3 != 0x0))
       && (BVar4 = pass1_1028_c314(param_7, uVar3, (ushort)puVar5, uVar9, uVar11, param_2, (ushort)param_3, (ushort)(param_3 >> 0x10), param_4), BVar4 != 0x0))
    {
        uVar6 = (uint)((ulong)param_2 >> 0x10);
        if(((*(int *)((int)param_2 + 0x4) == 0x0) && (uStack8 != 0x4))
           && (ppcVar1 = (code **)((int)*param_1 + 0x5c), (**ppcVar1)((int)&USHORT_1050_1028, param_1, param_2, param_3, param_4, uVar8, puVar10, uVar7, uVar12), puVar5 = extraout_DX_00, BVar4 == 0x0))
        {
            return;
        }
        uStack10 = *(ushort *)((int)param_2 + 0x4);
        if(uStack10 != 0x0)
        {
            pass1_1020_df10((ulong)param_1, (ushort *)((ulong)param_2 & 0xffff | (ulong)uVar6 << 0x10), param_4, uStack10, puVar5, param_6, param_7);
            return;
        }
        pass1_1020_deac((ulong)param_1, (ushort *)((ulong)param_2 & 0xffff | (ulong)uVar6 << 0x10), param_4, 0x0, puVar5, param_6, param_7);
        return;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_db86(ushort param_1, ushort param_2, ushort *param_3, ulong param_4, long param_5, ushort param_6)

{
    int        iVar1;
    undefined *puVar2;
    uint       uVar3;
    ulong      uVar4;
    ushort    *puVar5;
    undefined  local_4[0x2];

    uVar4 = pass1_1030_bcae((ushort)local_4, param_6);
    uVar3 = (uint)(uVar4 >> 0x10);
    iVar1 = (int)uVar4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_4, (uint)(param_4 >> 0x10));
    uVar4  = *(ulong *)(iVar1 + 0x10);
    puVar5 = param_3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar4, (uint)(uVar4 >> 0x10));
    puVar2 = local_4;
    pass1_1030_bcde(param_6, (ushort)puVar2, param_6, uVar4 & 0xffff | (ulong)uVar3 << 0x10, puVar5, param_5);
    if((int)puVar2 < 0x0)
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6af;
    }
    else
    {
        if(((int)puVar2 < 0x1f) || (*(int *)((int)param_3 + 0x4) < 0x1))
        {
            return;
        }
        PTR_LOOP_1050_50ca = (undefined *)0x6b6;
        PTR_LOOP_1050_50cc = puVar2 + -0x1e;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_dc1c(ulong param_1, ushort *param_2, ushort param_3)

{
    int         iVar1;
    code      **ppcVar2;
    undefined4 *puVar3;
    uint        uVar4;
    uint        uVar5;
    ulong       uVar6;
    undefined4 *puVar7;
    ulong      *puVar8;
    byte        bStack27;
    undefined   local_a[0x4];
    undefined4  uStack6;

    puVar8 = (ulong *)CONCAT22(param_3, local_a);
    uVar6  = pass1_1028_bb24(param_1);
    uVar5  = (uint)(uVar6 >> 0x10);
    puVar3 = (undefined4 *)uVar6;
    pass1_1030_64ce(param_3, puVar3, uVar5, _PTR_LOOP_1050_5740, param_2, uVar6 & 0xffff | (ulong)uVar5 << 0x10, puVar8);
    uStack6  = *puVar3;
    uVar5    = *(uint *)((int)puVar3 + 0x2);
    bStack27 = (byte)((ulong)uStack6 >> 0x18);
    uVar4    = (uint)bStack27;
    if(bStack27 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack6, uVar5);
        puVar7 = (undefined4 *)struct_op_1030_73a8(CONCAT22(uVar5, uVar4));
        iVar1  = *(int *)((int)puVar7 + 0xc);
        if(((iVar1 < 0x1) || (SBORROW2(iVar1, 0x1))) || ((iVar1 != 0x9 && 0x7 < iVar1 + -0x1 && ((iVar1 + -0x9 < 0x6a || (0x6 < iVar1 + -0x73))))))
        {
            ppcVar2 = (code **)((int)*puVar7 + 0x24);
            (**ppcVar2)();
        }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1020_dca8(ulong param_1, ushort param_2, ushort param_3)

{
    undefined2  uVar1;
    ushort      uVar2;
    undefined   local_2e[0xe];
    undefined4 *puStack32;
    ushort      uStack30;
    ushort      uStack28;
    ushort      uStack26;
    ushort      uStack24;
    ushort      uStack22;
    ushort      uStack20;
    ushort      uStack18;
    undefined4  local_10;
    ushort      uStack12;
    undefined4  uStack10;
    undefined   local_6[0x2];
    int         local_4;

    pass1_1028_c1f8(param_3, (int)local_6, param_2, param_1, (ushort *)CONCAT22(param_3, local_6), (ushort *)CONCAT22(param_3, &local_4));
    uStack10  = pass1_1028_b58e(param_1);
    uVar1     = (undefined2)((ulong)uStack10 >> 0x10);
    local_10  = *(undefined4 *)((int)uStack10 + 0xc);
    uStack12  = *(ushort *)((int)uStack10 + 0x10);
    puStack32 = &local_10;
    uStack18  = (ushort)local_10;
    uStack20  = (ushort)((ulong)local_10 >> 0x10);
    uStack24  = (ushort)local_10 - 0x1;
    if((int)uStack24 < 0x0)
    {
        uStack24 = 0x0;
    }
    uVar2    = local_4 - 0x1;
    uStack26 = (ushort)local_10 + 0x1;
    if((int)uVar2 < (int)((ushort)local_10 + 0x1))
    {
        uStack26 = uVar2;
    }
    uStack28 = uStack20 - 0x1;
    if((int)uStack28 < 0x0)
    {
        uStack28 = 0x0;
    }
    uStack30 = uStack20 + 0x1;
    if((int)uVar2 < (int)(uStack20 + 0x1))
    {
        uStack30 = uVar2;
    }
    uStack22 = uStack12;
    pass1_1008_3e54((ushort *)CONCAT22(param_3, local_2e), uStack12, uStack28, uStack24);
    pass1_1020_dc1c(param_1, (ushort *)CONCAT22(param_3, local_2e), param_3);
    pass1_1008_3e54((ushort *)CONCAT22(param_3, local_2e), uStack22, uStack28, uStack18);
    pass1_1020_dc1c(param_1, (ushort *)CONCAT22(param_3, local_2e), param_3);
    pass1_1008_3e54((ushort *)CONCAT22(param_3, local_2e), uStack22, uStack28, uStack26);
    pass1_1020_dc1c(param_1, (ushort *)CONCAT22(param_3, local_2e), param_3);
    pass1_1008_3e54((ushort *)CONCAT22(param_3, local_2e), uStack22, uStack20, uStack24);
    pass1_1020_dc1c(param_1, (ushort *)CONCAT22(param_3, local_2e), param_3);
    pass1_1008_3e54((ushort *)CONCAT22(param_3, local_2e), uStack22, uStack20, uStack26);
    pass1_1020_dc1c(param_1, (ushort *)CONCAT22(param_3, local_2e), param_3);
    pass1_1008_3e54((ushort *)CONCAT22(param_3, local_2e), uStack22, uStack30, uStack24);
    pass1_1020_dc1c(param_1, (ushort *)CONCAT22(param_3, local_2e), param_3);
    pass1_1008_3e54((ushort *)CONCAT22(param_3, local_2e), uStack22, uStack30, uStack18);
    pass1_1020_dc1c(param_1, (ushort *)CONCAT22(param_3, local_2e), param_3);
    pass1_1008_3e54((ushort *)CONCAT22(param_3, local_2e), uStack22, uStack30, uStack26);
    pass1_1020_dc1c(param_1, (ushort *)CONCAT22(param_3, local_2e), param_3);
    return;
}

BOOL16 __stdcall16far pass1_1020_deac(ulong param_1, ushort *param_2, long param_3, int param_4, uchar *param_5, int param_6, ushort param_7)

{
    ushort uVar1;
    ushort uVar2;

    uVar1 = (ushort)param_1;
    uVar2 = (ushort)(param_1 >> 0x10);
    pass1_1028_c7b6(param_7, (ushort)param_5, uVar1, uVar2, param_2, param_3);
    if(param_4 < 0x1)
    {
        return 0x0;
    }
    if(SBORROW2(param_4, 0x1))
    {
        return 0x0;
    }
    if(param_4 != 0x3 && 0x0 < param_4 + -0x2)
    {
        if(param_4 == 0x4)
        {
            pass1_1020_de32(param_1, 0x4, param_5, param_6, param_7);
            if(*(int *)(uVar1 + 0x24) == 0x6)
            {
                return 0x1;
            }
            return 0x0;
        }
        if(param_4 != 0x5)
        {
            return 0x0;
        }
    }
    *(undefined2 *)(uVar1 + 0x24) = 0x1;
    return 0x1;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_df10(ulong param_1, ushort *param_2, long param_3, ushort param_4, uchar *param_5, int param_6, ushort param_7)

{
    undefined4 *puVar1;
    uint        uVar2;
    BOOL16      BVar3;
    uint        uVar4;
    ulong       uVar5;
    ushort      uVar6;
    ushort      uVar7;
    byte        bStack31;
    undefined4  local_e;
    undefined4  uStack10;
    ushort      uStack6;
    undefined2  uStack4;

    uStack4 = 0x0;
    uVar6   = (ushort)param_1;
    uVar7   = (ushort)(param_1 >> 0x10);
    pass1_1028_c7b6(param_7, (ushort)param_5, uVar6, uVar7, param_2, param_3);
    uStack6 = param_4;
    if(param_4 == 0x0)
    {
        puVar1 = &local_e;
        pass1_1030_64ce(param_7, puVar1, param_5, _PTR_LOOP_1050_5740, param_2, param_3, (ulong *)CONCAT22(param_7, puVar1));
        uStack10 = *puVar1;
        uVar4    = *(uint *)((int)puVar1 + 0x2);
        bStack31 = (byte)((ulong)uStack10 >> 0x18);
        uVar2    = (uint)bStack31;
        if(bStack31 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack10, uVar4);
            uVar5 = struct_op_1030_73a8(CONCAT22(uVar4, uVar2));
            if(*(int *)((int)uVar5 + 0xc) == 0x6a)
            {
                BVar3 = pass1_1020_e044(param_1);
                if(BVar3 == 0x0)
                {
                    *(undefined2 *)(uVar6 + 0x24) = 0x1;
                }
                else
                {
                    PTR_LOOP_1050_50ca = (undefined *)0x6ac;
                }
            }
        }
    }
    else
    {
        if(((0x5 < (int)param_4) && (!SBORROW2(param_4, 0x6))) && ((int)(param_4 - 0x6) < 0x4))
        {
            pass1_1020_de32(param_1, param_4, param_5, param_6, param_7);
            switch(*(undefined2 *)(uVar6 + 0x24))
            {
            case 0x1:
                BVar3 = pass1_1020_e044(param_1);
                if(BVar3 != 0x0)
                {
                    PTR_LOOP_1050_50ca = (undefined *)0x6ac;
                }
                break;
            case 0x2:
            case 0x3:
            case 0x4:
            case 0x5:
                pass1_1020_e652(param_1, (ulong *)param_2, (ushort)((ulong)param_2 >> 0x10), param_3, param_7);
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 __stdcall16far pass1_1020_e044(ulong param_1)

{
    undefined4 uVar1;
    ushort     uVar2;
    undefined2 uVar3;
    ulong      uVar4;

    uVar3 = (undefined2)(param_1 >> 0x10);
    uVar4 = pass1_1018_04b8(*(ulong *)((int)param_1 + 0x28));
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar4, (uint)(uVar4 >> 0x10));
    uVar2 = pass1_1030_2fac(uVar4);
    uVar1 = *(undefined4 *)((int)param_1 + 0x28);
    if((int)uVar2 <= *(int *)((int)uVar1 + 0x1e))
    {
        return 0x1;
    }
    return 0x0;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_e08e(ulong param_1, ushort param_2, ushort param_3, uchar param_4)

{
    int        iVar1;
    undefined4 uVar2;
    int        iVar3;
    int        iVar4;
    ulong      uVar5;
    undefined2 extraout_DX;
    uint       uVar6;
    ushort     uVar7;
    ushort     uVar8;
    int       *piVar9;
    ushort    *puVar10;
    ushort     uVar11;
    ushort     uVar12;
    undefined2 local_158;
    undefined2 uStack342;
    ulong      uStack50;
    ulong     *puStack42;
    int        local_22;
    undefined  local_20[0x2];
    undefined  local_1e[0x2];
    undefined2 uStack28;
    int       *piStack26;
    int        local_18;
    ushort     local_16;
    ulong      uStack20;
    ulong      local_10;
    int        iStack12;
    ulong      uStack10;
    undefined4 uStack6;

    uVar8 = (ushort)(param_1 >> 0x10);
    uVar7 = (ushort)param_1;
    iVar3 = *(int *)(uVar7 + 0xc);
    if(iVar3 == 0x74)
    {
        iVar1 = *(int *)(uVar7 + 0x24);
        iVar3 = iVar1 + -0x1;
        if(iVar3 == 0x0)
            goto LAB_1020_e0ae;
        iVar3 = iVar1 + -0x6;
        if(iVar3 != 0x0)
            goto LAB_1020_e0b9;
        uVar11 = 0x1;
    }
    else
    {
        if(iVar3 == 0x78)
        {
            iVar1 = *(int *)(uVar7 + 0x24);
            iVar4 = iVar1 + -0x1;
            if(iVar4 != 0x0)
            {
                iVar3 = iVar1 + -0x2;
                if((0x0 < iVar4) && (!SBORROW2(iVar4, 0x1)))
                {
                    if(iVar1 + -0x5 == 0x0 || iVar3 < 0x3)
                    {
                        iVar3 = iVar1 + -0x5;
                        pass1_1020_e49a(param_1, param_3, param_4);
                    }
                    else
                    {
                        iVar3 = iVar1 + -0x6;
                        if(iVar3 == 0x0)
                        {
                            pass1_1020_e39c(param_1, 0x6, 0x0, param_3, param_4);
                            pass1_1020_dca8(param_1, param_2, param_3);
                        }
                    }
                }
                goto LAB_1020_e0b9;
            }
            uVar11 = 0x6a;
            iVar3  = iVar4;
        }
        else
        {
            iVar3 = iVar3 + -0x79;
            if(iVar3 == 0x0)
            {
                pass1_1020_e49a(param_1, param_3, param_4);
                return;
            }
        LAB_1020_e0ae:
            uVar11 = 0x47;
        }
    }
    pass1_1020_e39c(param_1, uVar11, iVar3, param_3, param_4);
LAB_1020_e0b9:
    pass1_1028_b58e(param_1);
    uStack6  = CONCAT22(extraout_DX, iVar3);
    uVar5    = *(ulong *)(iVar3 + 0x2e);
    uVar6    = *(uint *)(iVar3 + 0x30);
    uStack10 = uVar5;
    if(*(int *)(uVar7 + 0xc) != 0x79)
    {
        pass1_1038_5804(uVar5 & 0xffff | (ulong)uVar6 << 0x10, 0x1, 0x2);
    }
    if(*(int *)(uVar7 + 0x24) == 0x6)
    {
        pass1_1038_43cc((int)uStack10, (int)(uStack10 >> 0x10), 0x1, 0x2, (int)uVar5, uVar6);
    }
    local_10  = *(ulong *)((int)uStack6 + 0xc);
    iStack12  = *(int *)((int)uStack6 + 0x10);
    puStack42 = &local_10;
    if((*(int *)(uVar7 + 0x24) == 0x6) && (iStack12 == 0x0))
    {
        return;
    }
    uVar2     = *(undefined4 *)(uVar7 + 0x28);
    uVar5     = *(ulong *)((int)uVar2 + 0x20);
    puVar10   = &local_16;
    piStack26 = &local_18;
    piVar9    = piStack26;
    uVar11    = param_3;
    uVar12    = param_3;
    uStack20  = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar5, (uint)(uVar5 >> 0x10));
    uStack28 = (undefined2)uVar5;
    pass1_1030_5b1c(uVar5 & 0xffff | ZEXT24(piStack26) << 0x10, (ushort *)CONCAT22(uVar11, piVar9), (ushort *)CONCAT22(uVar12, puVar10));
    pass1_1028_c8ee(param_3, uVar7, uVar8, *(int *)(uVar7 + 0x24), (ushort *)CONCAT22(param_3, &local_10));
    pass1_1008_3eb4((ushort *)CONCAT22(param_3, &local_10), (ushort *)CONCAT22(param_3, &local_22), (ushort *)CONCAT22(param_3, local_20), (ushort *)CONCAT22(param_3, local_1e));
    if(*(int *)(uVar7 + 0x24) == 0x1)
    {
        if(local_18 < local_22)
        {
            pass1_1030_5b3e(CONCAT22(piStack26, uStack28), local_22, local_16);
            pass1_1030_5b1c(CONCAT22(piStack26, uStack28), (ushort *)CONCAT22(param_3, &local_18), (ushort *)CONCAT22(param_3, &local_16));
        }
        uStack50 = *(ulong *)((int)uStack10 + 0x4);
        struct_op_1028_87f0(param_3, param_4, (astruct_97 *)CONCAT22(param_3, &local_158), 0x0, 0x0, 0x6a, &local_10, param_3, uStack50, uStack20);
        fn_ptr_1030_835a(_PTR_LOOP_1050_5748, (ulong *)CONCAT22(param_3, &local_158));
        local_158 = 0x389a;
        uStack342 = 0x1008;
    }
    pass1_1028_ccd0(param_4, param_3, param_1, (ushort *)CONCAT22(param_3, &local_10));
    return;
}

void __stdcall16far pass1_1020_e44c(ulong param_1, ushort param_2, ushort param_3, uchar param_4)

{
    int *piVar1;
    int  iVar2;
    uint uVar3;

    uVar3 = (uint)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(int *)(iVar2 + 0x12) == 0x2)
    {
        piVar1  = (int *)(iVar2 + 0x14);
        *piVar1 = *piVar1 + -0x1;
        if((*(int *)(iVar2 + 0x26) == 0x0) && (*(int *)(iVar2 + 0xc) == 0x78))
        {
            pass1_1020_e294(param_1 & 0xffff | (ulong)uVar3 << 0x10, param_3, param_4);
        }
        if(*(int *)(iVar2 + 0x14) == 0x0)
        {
            pass1_1020_e08e(param_1 & 0xffff | (ulong)uVar3 << 0x10, param_2, param_3, param_4);
            return;
        }
        if(*(int *)(iVar2 + 0x24) == 0x6)
        {
            *(undefined2 *)(iVar2 + 0xe) = 0x49;
        }
    }
    return;
}


void __stdcall16far pass1_1020_e49a(ulong param_1, ushort param_2, uchar param_3)

{
    int        iVar1;
    int        iVar2;
    undefined4 uVar3;
    ushort     uStack10;

    uVar3    = pass1_1028_b58e(param_1);
    iVar1    = *(int *)((int)uVar3 + 0x14);
    uStack10 = 0x0;
    iVar2    = iVar1 + -0x6;
    if(iVar2 == 0x0)
    {
        uStack10 = 0x9;
    }
    else
    {
        iVar2 = iVar1 + -0x7;
        if(iVar2 == 0x0)
        {
            uStack10 = 0x6;
        }
        else
        {
            iVar2 = iVar1 + -0x8;
            if(iVar2 == 0x0)
            {
                uStack10 = 0x7;
            }
            else
            {
                iVar2 = iVar1 + -0x9;
                if(iVar2 == 0x0)
                {
                    uStack10 = 0x8;
                }
            }
        }
    }
    pass1_1020_e39c(param_1, uStack10, iVar2, param_2, param_3);
    return;
}


int __stdcall16far pass1_1020_e4fa(ulong param_1, ushort param_2)

{
    undefined4 uVar1;
    int        iStack4;

    switch(param_2)
    {
    case 0x2:
    case 0x5:
    case 0x6:
    case 0x7:
        iStack4 = 0x4;
        break;
    case 0x3:
    case 0x8:
        iStack4 = 0x5;
        break;
    default:
        uVar1   = pass1_1028_b58e(param_1);
        iStack4 = *(int *)((int)uVar1 + 0x14) + 0x2;
    }
    return iStack4;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_e558(ulong param_1, int param_2, ushort param_3)

{
    undefined4 *puVar1;
    uint        uVar2;
    int         iVar3;
    undefined2  extraout_DX;
    undefined2  uVar4;
    uint        uVar5;
    ushort      uVar6;
    ushort      uVar7;
    byte        bStack45;
    undefined   local_24[0xc];
    undefined4  uStack24;
    undefined4  uStack20;
    undefined4  local_10;
    undefined2  uStack12;
    int         iStack10;
    ushort      uStack8;
    int         iStack6;
    undefined2  uStack4;

    uVar7 = (ushort)(param_1 >> 0x10);
    uVar6 = (ushort)param_1;
    if(*(int *)(uVar6 + 0xc) == 0x79)
    {
        param_2                       = *(int *)(uVar6 + 0x24);
        *(int *)(uVar6 + 0x14)        = param_2;
        *(undefined2 *)(uVar6 + 0x24) = 0x0;
    }
    if(*(int *)(uVar6 + 0x24) != 0x6)
    {
        pass1_1028_b58e(param_1);
        uStack8  = *(ushort *)(param_2 + 0x14);
        iStack6  = param_2;
        uStack4  = extraout_DX;
        iStack10 = pass1_1020_e4fa(param_1, uStack8);
        local_10 = *(undefined4 *)(iStack6 + 0xc);
        uStack12 = *(undefined2 *)(iStack6 + 0x10);
        uStack24 = CONCAT22(uStack24._2_2_, &local_10);
        uVar4    = uStack4;
        pass1_1028_c8ee(param_3, uVar6, uVar7, *(int *)(uVar6 + 0x24), (ushort *)CONCAT22(param_3, &local_10));
        puVar1 = &local_10;
        pass1_1028_c89c(param_1, (ushort *)CONCAT22(param_3, puVar1), (ulong *)CONCAT22(param_3, local_24), (int)puVar1, param_3);
        uStack24       = *puVar1;
        uVar5          = *(uint *)((int)puVar1 + 0x2);
        bStack45       = (byte)((ulong)uStack24 >> 0x18);
        uVar2          = (uint)bStack45;
        uStack20._0_2_ = (ushort)uStack24;
        uStack20       = uStack24;
        if(bStack45 != 0x0)
        {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uStack20, uVar5);
            uStack20._0_2_ = *(ushort *)(uVar2 + 0x14);
        }
        uStack8                = (ushort)uStack20;
        iVar3                  = pass1_1020_e4fa(param_1, (ushort)uStack20);
        *(int *)(uVar6 + 0x14) = iStack10 + iVar3;
        return;
    }
    *(undefined2 *)(uVar6 + 0x14) = 0x1;
    return;
}

ulong *__stdcall16far pass1_1020_e652(ulong param_1, ulong *param_2, ushort param_3, long param_4, ushort param_5)

{
    ulong     *puVar1;
    ushort     uVar2;
    ushort     uVar3;
    ulong      local_8;
    undefined2 uStack4;

    local_8 = *param_2;
    uStack4 = *(undefined2 *)(param_2 + 0x1);
    uVar3   = (ushort)(param_1 >> 0x10);
    uVar2   = (ushort)param_1;
    pass1_1028_c8ee(param_5, uVar2, uVar3, *(int *)(uVar2 + 0x24), (ushort *)CONCAT22(param_5, &local_8));
    puVar1 = &local_8;
    pass1_1028_c7b6(param_5, param_3, uVar2, uVar3, (ushort *)CONCAT22(param_5, puVar1), param_4);
    if(puVar1 != (ulong *)0x0)
    {
        puVar1 = (ulong *)((int)&PTR_LOOP_1050_0000 + 0x1);
    }
    return puVar1;
}

ushort *__stdcall16far pass1_1020_e81c(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0xe88e;
    *(undefined2 *)(param_1 + 0x2)        = 0x1020;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1020_e846(ushort *param_1)

{
    *param_1                            = 0xe88e;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
    pass1_1028_b418(param_1);
    return;
}

int __cdecl16far pass1_1020_c7fa(ulong param_1, ulong param_2)

{
    return *(int *)((int)param_1 + 0x4) - *(int *)((int)param_2 + 0x4);
}


ulong __stdcall16far pass1_1020_c860(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)param_1 + 0x6), *(undefined2 *)((int)param_1 + 0x4));
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_c872(ulong param_1, ulong param_2, ulong param_3)

{
    uint       *puVar1;
    ulong      *puVar2;
    int        *piVar3;
    astruct_98 *uVar4;
    uint        uVar6;
    int         iVar7;
    int         iVar8;
    undefined2  uVar9;
    undefined2  uVar10;
    undefined2  uVar11;
    bool        bVar12;
    ulong       uStack14;
    ulong       uStack10;
    astruct_99 *puStack6;
    astruct_99 *uVar5;

    puStack6 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_4fb8);
    uVar6    = (uint)((ulong)puStack6 >> 0x10);
    uVar5    = (astruct_99 *)puStack6;
    if((uVar6 | (uint)uVar5) == 0x0)
    {
        puStack6 = (astruct_99 *)0x0;
    }
    else
    {
        puStack6->field_0x0 = 0x389a;
        uVar5->field_0x2    = 0x1008;
        uVar5->field_0x4    = 0x0;
        uVar5->field_0x8    = 0x0;
        puStack6->field_0x0 = 0x5bc0;
        uVar5->field_0x2    = 0x1008;
        uVar5->field_0xe    = 0x0;
        uVar5->field_0xc    = 0x0;
        puStack6->field_0x0 = 0xc9e6;
        uVar5->field_0x2    = 0x1020;
    }
    if(puStack6 == (astruct_99 *)0x0)
    {
        return;
    }
    uVar9                   = (undefined2)((ulong)puStack6 >> 0x10);
    iVar7                   = (int)puStack6;
    *(ulong *)(iVar7 + 0x8) = param_3;
    *(ulong *)(iVar7 + 0xc) = param_2;
    uVar10                  = (undefined2)(param_1 >> 0x10);
    iVar8                   = (int)param_1;
    uStack14                = *(ulong *)(iVar8 + 0x4);
    uVar11                  = *(undefined2 *)(iVar8 + 0x6);
    if(*(int *)(iVar8 + 0x8) == 0x0)
    {
    LAB_1020_c92d:
        *(undefined4 *)(iVar7 + 0x4) = *(undefined4 *)(iVar8 + 0x4);
    }
    else
    {
        puVar1 = (uint *)((int)uStack14 + 0xe);
        bVar12 = *puVar1 < param_2._2_2_;
        if((bVar12 || *puVar1 == param_2._2_2_) && ((bVar12 || (puVar1 = (uint *)((int)uStack14 + 0xc), *puVar1 < (uint)param_2 || *puVar1 == (uint)param_2))))
            goto LAB_1020_c92d;
        bVar12 = false;
        while(true)
        {
            if(uStack14 == 0x0)
                break;
            uVar11 = (undefined2)(uStack14 >> 0x10);
            puVar2 = (ulong *)((int)uStack14 + 0xc);
            if(*puVar2 < param_2 || *puVar2 == param_2)
            {
                bVar12                                = true;
                *(ulong *)(iVar7 + 0x4)               = uStack14;
                *(astruct_99 **)((int)uStack10 + 0x4) = puStack6;
                break;
            }
            uStack10 = uStack14;
            uStack14 = *(ulong *)((int)uStack14 + 0x4);
        }
        param_1 = uStack10;
        if(bVar12)
            goto LAB_1020_c9ab;
    }
    uVar11                              = (undefined2)(param_1 >> 0x10);
    *(int *)((int)param_1 + 0x4)        = iVar7;
    *(undefined2 *)((int)param_1 + 0x6) = uVar9;
LAB_1020_c9ab:
    piVar3  = (int *)(iVar8 + 0x8);
    *piVar3 = *piVar3 + 0x1;
    return;
}


ushort *__stdcall16far pass1_1020_c9ba(ushort *param_1, byte param_2)

{
    ushort uVar1;

    uVar1                 = (ushort)((ulong)param_1 >> 0x10);
    *param_1              = 0x389a;
    ((int *)param_1)[0x1] = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        pass1_1000_093a((int *)param_1, uVar1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1020_ca0c(astruct_179 *param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_0982(param_1, param_2, param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0xcc7c;
    param_1->field_0x2                    = 0x1020;
    return (ushort *)CONCAT22(param_2, param_1);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_ca36(int param_1, ushort param_2, ushort param_3, int param_4, ushort param_5)

{
    uchar  *puVar1;
    ulong   uVar2;
    ushort *puVar3;

    pass1_1028_09b8(CONCAT22(param_2, param_1));
    uVar2  = pass1_1028_b4f2(CONCAT22(param_2, param_1));
    puVar1 = (uchar *)(uVar2 >> 0x10);
    if(*(long *)((int)uVar2 + 0x200) != 0x8000002)
    {
        puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x8, param_5, puVar1, param_4);
        pass1_1010_988c((ulong)puVar3, *(int *)(param_1 + 0xc));
    }
    return;
}


void __stdcall16far pass1_1020_ca82(ulong *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    uchar *puVar1;
    uint   uVar2;
    ulong  uVar3;

    pass1_1028_be9e(param_1, param_2, param_3, (ushort)&USHORT_1050_1028, param_4);
    uVar3  = pass1_1028_b4f2((ulong)param_1);
    puVar1 = (uchar *)(uVar3 >> 0x10);
    if(*(long *)((int)uVar3 + 0x200) != 0x8000002)
    {
        uVar2 = (uint)((ulong)param_1 >> 0x10);
        if(*(int *)((int)param_1 + 0x12) == 0x5)
        {
            pass1_1020_cac2((ulong)param_1 & 0xffff | (ulong)uVar2 << 0x10, puVar1, param_2, param_3, param_4);
        }
    }
    return;
}


ushort *__stdcall16far pass1_1020_cd06(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0xcd7e;
    *(undefined2 *)(param_1 + 0x2)        = 0x1020;
    return (ushort *)CONCAT22(param_2, param_1);
}


ushort __stdcall16far pass1_1020_cd30(ulong param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(((*(int *)(iVar1 + 0x12) == 0x6) || (*(int *)(iVar1 + 0x12) == 0x5)) && ((*(byte *)(iVar1 + 0x1a) & 0x2) != 0x0))
    {
        return 0x1;
    }
    return 0x0;
}


ushort *__stdcall16far pass1_1020_ce08(astruct_179 *param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_0982(param_1, param_2, param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0xd004;
    param_1->field_0x2                    = 0x1020;
    return (ushort *)CONCAT22(param_2, param_1);
}

void __stdcall16far pass1_1020_cf6c(ushort param_1, ushort param_2, int param_3, ulong param_4)

{
    uint      *puVar1;
    int       *piVar2;
    uint       uVar3;
    undefined4 uVar4;
    undefined2 uVar5;
    int        iVar6;
    uint       uVar7;
    uint       uVar8;
    int        iVar9;
    undefined2 uVar10;

    uVar10                                = (undefined2)(param_4 >> 0x10);
    uVar4                                 = *(undefined4 *)((int)param_4 + 0x1f6);
    iVar6                                 = (int)uVar4;
    uVar5                                 = (undefined2)((ulong)uVar4 >> 0x10);
    uVar7                                 = param_3 / 0x5;
    uVar8                                 = uVar7 * -0x4 + param_3;
    puVar1                                = (uint *)(iVar6 + 0x50);
    uVar3                                 = *puVar1;
    *puVar1                               = *puVar1 + uVar8;
    piVar2                                = (int *)(iVar6 + 0x52);
    *piVar2                               = *piVar2 + ((int)uVar8 >> 0xf) + (uint)CARRY2(uVar3, uVar8);
    iVar9                                 = (int)uVar7 >> 0xf;
    puVar1                                = (uint *)(iVar6 + 0x78);
    uVar3                                 = *puVar1;
    *puVar1                               = *puVar1 + uVar7;
    piVar2                                = (int *)(iVar6 + 0x7a);
    *piVar2                               = *piVar2 + iVar9 + (uint)CARRY2(uVar3, uVar7);
    puVar1                                = (uint *)(iVar6 + 0xa0);
    uVar3                                 = *puVar1;
    *puVar1                               = *puVar1 + uVar7;
    piVar2                                = (int *)(iVar6 + 0xa2);
    *piVar2                               = *piVar2 + iVar9 + (uint)CARRY2(uVar3, uVar7);
    puVar1                                = (uint *)(iVar6 + 0xc8);
    uVar3                                 = *puVar1;
    *puVar1                               = *puVar1 + uVar7;
    piVar2                                = (int *)(iVar6 + 0xca);
    *piVar2                               = *piVar2 + iVar9 + (uint)CARRY2(uVar3, uVar7);
    puVar1                                = (uint *)(iVar6 + 0xf0);
    uVar3                                 = *puVar1;
    *puVar1                               = *puVar1 + uVar7;
    piVar2                                = (int *)(iVar6 + 0xf2);
    *piVar2                               = *piVar2 + iVar9 + (uint)CARRY2(uVar3, uVar7);
    *(undefined2 *)((int)param_4 + 0x1fe) = 0x1;
    return;
}

ushort *__stdcall16far pass1_1020_d08e(int param_1, ushort param_2, int param_3, ulong param_4, ushort param_5)

{
    pass1_1028_b39e((ushort *)CONCAT22(param_2, param_1), param_3, param_4, param_5);
    *(ushort *)CONCAT22(param_2, param_1) = 0xd314;
    *(undefined2 *)(param_1 + 0x2)        = 0x1020;
    return (ushort *)CONCAT22(param_2, param_1);
}


void __stdcall16far pass1_1020_d0b8(ulong *param_1, ushort param_2, ushort param_3, ushort param_4)

{
    ulong uVar1;
    int   iVar2;

    if(*(int *)((int)param_1 + 0x12) != 0x6)
    {
        return;
    }
    uVar1 = pass1_1028_b4f2((ulong)param_1);
    iVar2 = (int)uVar1;
    if(*(long *)(iVar2 + 0x200) != 0x8000002)
    {
        pass1_1028_cb04((ulong)param_1, param_2, param_3, param_4);
        if((iVar2 == 0x0) || (pass1_1020_d194((ulong)param_1, param_3, param_4), iVar2 == 0x0))
        {
            iVar2 = 0x6;
            goto LAB_1020_d10b;
        }
        pass1_1028_c952((ulong)param_1, param_2, param_3, param_4);
    }
    iVar2 = 0x5;
LAB_1020_d10b:
    pass1_1028_bdac(param_1, iVar2, (ushort)&USHORT_1050_1028);
    return;
}


undefined2 __stdcall16far pass1_1020_d118(ulong param_1, ushort *param_2, ulong param_3, ulong param_4, int param_5, ushort param_6, ushort param_7)

{
    BOOL16 BVar1;
    ushort uVar2;
    ushort uVar3;

    uVar2 = (ushort)param_1;
    uVar3 = (ushort)(param_1 >> 0x10);
    pass1_1028_c7b6(param_7, param_6, uVar2, uVar3, param_2, param_4);
    if(param_5 == 0x5)
    {
        pass1_1028_c23e(uVar2, uVar3, param_2, param_3, param_4, 0x5, param_6, param_7);
        if(param_5 != 0x0)
        {
            pass1_1028_c3aa(uVar2, uVar3, param_2, param_3, param_4, param_7);
            if(param_5 != 0x0)
            {
                BVar1 = pass1_1028_c314(param_7, param_5, param_6, uVar2, uVar3, param_2, (ushort)param_3, (ushort)(param_3 >> 0x10), param_4);
                if(BVar1 != 0x0)
                {
                    return 0x1;
                }
            }
        }
    }
    else
    {
        PTR_LOOP_1050_50ca = (undefined *)0x6a8;
    }
    return 0x0;
}