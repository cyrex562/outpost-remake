
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_7226(ulong param_1)

{
    ulong      uVar1;
    undefined4 uVar2;
    BOOL16     BVar3;
    int        iVar4;
    uint       uVar5;

    uVar5 = (uint)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    if(*(long *)(iVar4 + 0x1a) == 0x0)
    {
        struct_op_1030_73a8(param_1 & 0xffff | (ulong)uVar5 << 0x10);
    }
    uVar2 = *(undefined4 *)(iVar4 + 0x1a);
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, *(undefined2 *)((int)uVar2 + 0xc), 0x10);
    if(((BVar3 != 0x0) && (uVar2 = *(undefined4 *)(iVar4 + 0x1a), *(int *)((int)uVar2 + 0x12) == 0x5))
       && (uVar1 = *(ulong *)(iVar4 + 0x1a),
           uVar2 = *(undefined4 *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x14)),
           *(int *)((int)uVar2 + 0xa4) == 0x1e))
    {
        return;
    }
    return;
}


ulong __stdcall16far pass1_1030_51f0(ulong param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                         = (undefined2)(param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(undefined2 *)(iVar1 + 0xa4) = 0x0;
    *(undefined2 *)(iVar1 + 0xa6) = 0x0;
    *(undefined2 *)(iVar1 + 0xa8) = 0x0;
    *(undefined2 *)(iVar1 + 0xaa) = 0x0;
    *(undefined2 *)(iVar1 + 0xac) = 0x0;
    return param_1;
}


void __stdcall16far pass1_1030_67cc(ushort *param_1)

{
    astruct_687 *iVar1;
    undefined2   uVar1;

    struct_1030_1628(param_1);
    iVar1 = (astruct_687 *)param_1;
    iVar1 = (astruct_687 *)&iVar1->field_0xc;
    pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1->field_0x12 = 0x0;
    iVar1->field_0x14 = 0x0;
    iVar1->field_0x16 = 0x0;
    iVar1->field_0x1a = 0x0;
    iVar1->field_0x1e = 0x0;
    iVar1->field_0x22 = 0x0;
    iVar1->field_0x26 = 0x0;
    iVar1->field_0x2a = 0x0;
    iVar1->field_0x2e = 0x0;
    iVar1->field_0x32 = 0x0;
    iVar1->field_0x34 = 0x0;
    iVar1->field_0x38 = 0x0;
    iVar1->field_0x36 = 0x0;
    iVar1->field_0x3c = 0x0;
    iVar1->field_0x3a = 0x0;
    iVar1->field_0x40 = 0x0;
    iVar1->field_0x3e = 0x0;
    *param_1          = 0x8114;
    iVar1->field_0x2  = 0x1030;
    return;
}


void __stdcall16far pass1_1030_684c(
  ushort *param_1, ulong *param_2, ushort param_3, ushort param_4, ushort param_5, ulong param_6, ushort param_7)

{
    int        iVar1;
    undefined2 uVar2;

    pass1_1030_165e(param_1, 0x5000000, param_6, param_7);
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(ulong *)(iVar1 + 0xc)       = *param_2;
    *(undefined2 *)(iVar1 + 0x10) = *(undefined2 *)(param_2 + 0x1);
    *(ushort *)(iVar1 + 0x12)     = param_4;
    *(ushort *)(iVar1 + 0x14)     = param_4;
    *(undefined4 *)(iVar1 + 0x16) = 0x0;
    *(undefined4 *)(iVar1 + 0x1a) = 0x0;
    *(undefined4 *)(iVar1 + 0x1e) = 0x0;
    *(undefined4 *)(iVar1 + 0x22) = 0x0;
    *(undefined4 *)(iVar1 + 0x26) = 0x0;
    *(undefined4 *)(iVar1 + 0x2a) = 0x0;
    *(undefined4 *)(iVar1 + 0x2e) = 0x0;
    *(undefined2 *)(iVar1 + 0x32) = 0x0;
    *(undefined2 *)(iVar1 + 0x34) = 0x0;
    *(undefined4 *)(iVar1 + 0x36) = 0x0;
    *(undefined4 *)(iVar1 + 0x3a) = 0x0;
    *(undefined4 *)(iVar1 + 0x3e) = 0x0;
    *param_1                      = 0x8114;
    *(undefined2 *)(iVar1 + 0x2)  = 0x1030;
    return;
}


ushort *__stdcall16far pass1_1030_560e(ushort *param_1)

{
    int        iVar1;
    undefined2 uVar2;

    struct_1030_17ce(param_1, 0x64, 0x1f4);
    uVar2                         = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                         = (int)param_1;
    *(undefined4 *)(iVar1 + 0x10) = 0x0;
    pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x14)));
    *(undefined2 *)(iVar1 + 0x1a) = 0x0;
    *(undefined2 *)(iVar1 + 0x1c) = 0x0;
    *param_1                      = (ushort)s_procLo_1050_5bd0;
    *(undefined2 *)(iVar1 + 0x2)  = 0x1030;
    return param_1;
}


ushort *__stdcall16far struct_1030_565a(ushort *param_1, ulong param_2, uint param_3, uchar *param_4)

{
    astruct_353 *iVar1;
    undefined2   uVar1;

    pass1_1030_183c(param_1, 0x64, 0x1f4, 0x3000000, param_2, param_3, param_4);
    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_353 *)param_1;
    iVar1->field_0x10 = 0x0;
    pass1_1008_3e38((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x14));
    iVar1->field_0x1a = 0x0;
    iVar1->field_0x1c = 0x0;
    *param_1          = (ushort)s_procLo_1050_5bd0;
    iVar1->field_0x2  = 0x1030;
    return param_1;
}


void __stdcall16far pass1_1030_34da(ulong param_1)

{
    astruct_682 *iVar1;
    undefined2   uVar1;

    uVar1              = (undefined2)(param_1 >> 0x10);
    iVar1              = (astruct_682 *)param_1;
    iVar1->field_0x176 = 0x1;
    iVar1->field_0x178 = 0x1;
    iVar1->field_0x17a = 0x1;
    iVar1->field_0x17c = 0x1;
    iVar1->field_0x17e = 0x4;
    iVar1->field_0x182 = 0x32;
    iVar1->field_0x184 = 0xa;
    iVar1->field_0x186 = 0xa;
    iVar1->field_0x188 = 0xa;
    iVar1->field_0x18a = 0x4b;
    pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | ZEXT24(iVar1 + 0x1)), (WNDCLASS16 *)0x0, 0x18);
    return;
}


void __stdcall16far struct_1030_44be(ulong *param_1, ushort param_2)

{
    astruct_138 *iVar1;
    int          unaff_DI;
    undefined2   uVar1;
    ushort       unaff_SS;
    ushort      *puVar2;

    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    iVar1              = (astruct_138 *)param_1;
    *param_1           = 0x0;
    iVar1->field_0x8   = 0x0;
    iVar1->field_0x12  = 0x0;
    iVar1->field_0x152 = 0x0;
    iVar1->field_0x154 = 0x0;
    iVar1->field_0x156 = 0x0;
    iVar1->field_0x158 = 0x0;
    iVar1->field_0x15a = 0x0;
    iVar1->field_0x15c = 0x0;
    iVar1->field_0x160 = 0x0;
    iVar1->field_0x164 = 0x0;
    iVar1->field_0x568 = 0x0;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, unaff_SS, (uchar *)param_2, unaff_DI);
    iVar1->field_0x568 = *(undefined4 *)((int)puVar2 + 0x64);
    return;
}


ulong __stdcall16far struct_1030_4574(ulong param_1)

{
    astruct_159 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)(param_1 >> 0x10);
    iVar1             = (astruct_159 *)param_1;
    iVar1->field_0xc  = DAT_1050_518c;
    iVar1->field_0xe  = 0x518e;
    iVar1->field_0x10 = (int)&USHORT_1050_1050;
    return param_1 & 0xffff0000 | ZEXT24(&iVar1->field_0xc);
}


void __stdcall16far pass1_1030_2068(ushort *param_1)

{
    int        iVar1;
    int        iVar2;
    undefined2 uVar3;
    int        iStack4;

    struct_1030_17ce(param_1, 0x0, 0x0);
    uVar3                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    *param_1                     = 0x293c;
    *(undefined2 *)(iVar1 + 0x2) = 0x1030;
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x10)), (WNDCLASS16 *)0x0, 0x106);
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x116)), (WNDCLASS16 *)0x0, 0x86);
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x19c)), (WNDCLASS16 *)0x0, 0xa);
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x2ac)), (WNDCLASS16 *)0x0, 0x106);
    iStack4 = 0x0;
    do
    {
        iVar2                          = iStack4 * 0x2 + iVar1;
        *(undefined2 *)(iVar2 + 0x10)  = 0xffff;
        *(undefined2 *)(iVar2 + 0x1a6) = 0x19;
        iStack4                        = iStack4 + 0x1;
    } while(iStack4 < 0x83);
    return;
}


void __stdcall16far struct_1030_2112(ushort *param_1, ulong param_2, uint param_3, uchar *param_4)

{
    astruct_366 *iVar1;
    astruct_367 *iVar2;
    undefined2   uVar1;
    int          iStack4;

    pass1_1030_183c(param_1, 0x1, 0x1, 0x8000000, param_2, param_3, param_4);
    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_366 *)param_1;
    *param_1         = 0x293c;
    iVar1->field_0x2 = 0x1030;
    iStack4          = 0x0;
    do
    {
        iVar2              = (astruct_367 *)(&iVar1->field_0x0 + iStack4 * 0x2);
        iVar2->field_0x10  = 0xffff;
        iVar2->field_0x1a6 = 0x19;
        iStack4            = iStack4 + 0x1;
    } while(iStack4 < 0x83);
    pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x116), (WNDCLASS16 *)0x0, 0x86);
    pass1_1000_4906(
      (astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x19c), (WNDCLASS16 *)0x0, 0xa);
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar1 + 0x1)), (WNDCLASS16 *)0x0, 0x106);
    iVar1->field_0x10  = 0x0;
    iVar1->field_0x14  = 0x0;
    iVar1->field_0x16  = 0x0;
    iVar1->field_0x20  = 0x0;
    iVar1->field_0x44  = 0x0;
    iVar1->field_0x50  = 0x0;
    iVar1->field_0x6a  = 0x0;
    iVar1->field_0x84  = 0x0;
    iVar1->field_0xc8  = 0x0;
    iVar1->field_0xe4  = 0x0;
    iVar1->field_0xf0  = 0x0;
    iVar1->field_0xf4  = 0x0;
    iVar1->field_0xf6  = 0x0;
    iVar1->field_0x102 = 0x0;
    iVar1->field_0xfe  = 0x0;
    iVar1->field_0x1a6 = 0x0;
    iVar1->field_0x1aa = 0x0;
    iVar1->field_0x1ac = 0x0;
    iVar1->field_0x1b6 = 0x0;
    iVar1->field_0x1da = 0x0;
    iVar1->field_0x1e6 = 0x0;
    iVar1->field_0x200 = 0x0;
    iVar1->field_0x21a = 0x0;
    iVar1->field_0x25e = 0x0;
    iVar1->field_0x27a = 0x0;
    iVar1->field_0x286 = 0x0;
    iVar1->field_0x28a = 0x0;
    iVar1->field_0x28c = 0x0;
    iVar1->field_0x298 = 0x0;
    iVar1->field_0x294 = 0x0;
    return;
}


void __stdcall16far pass1_1030_2958(ushort *param_1)

{
    astruct_347 *iVar1;
    undefined2   uVar1;

    struct_1030_17ce(param_1, 0x5, 0xf);
    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_347 *)param_1;
    iVar1->field_0x10 = 0x0;
    iVar1->field_0x14 = 0x0;
    iVar1->field_0x16 = 0x0;
    iVar1->field_0x18 = 0x2710;
    iVar1->field_0x1a = 0x0;
    *param_1          = 0x3130;
    iVar1->field_0x2  = 0x1030;
    return;
}


void __stdcall16far struct_1030_299a(ushort *param_1, ulong param_2, uint param_3, uchar *param_4)

{
    astruct_352 *iVar1;
    undefined2   uVar1;

    pass1_1030_183c(param_1, 0x5, 0xf, 0x2000000, param_2, param_3, param_4);
    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_352 *)param_1;
    iVar1->field_0x10 = 0x0;
    iVar1->field_0x14 = 0x0;
    iVar1->field_0x16 = 0x0;
    iVar1->field_0x18 = 0x2710;
    iVar1->field_0x1a = 0x0;
    *param_1          = 0x3130;
    iVar1->field_0x2  = 0x1030;
    return;
}


void __stdcall16far pass1_1030_1120(ulong param_1, ushort param_2, uchar *param_3, ushort param_4)

{
    uchar *puVar1;

    mem_op_1000_179c(0x3b2, param_3, 0x1000);
    puVar1 = (uchar *)((uint)param_3 | param_2);
    if(puVar1 == (uchar *)0x0)
    {
        param_2 = 0x0;
        puVar1  = (uchar *)0x0;
    }
    else
    {
        struct_1030_2112((ushort *)CONCAT22(param_3, param_2), 0x0, param_2, puVar1);
    }
    pass1_1030_1358(*(ulong *)((int)param_1 + 0x2a),
                    param_2,
                    (ushort)puVar1,
                    *(ulong *)(param_2 + 0x4) & 0xffff | (ulong)(*(uint *)(param_2 + 0x6) & 0xff) << 0x10,
                    param_4);
    return;
}


void __stdcall16far struct_1030_11aa(ushort *param_1, long param_2, long param_3, ushort param_4)

{
    astruct_156 *iVar1;
    undefined2   uVar1;

    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_156 *)param_1;
    *param_1          = 0x389a;
    iVar1->field_0x2  = 0x1008;
    iVar1->field_0x4  = 0x0;
    iVar1->field_0x6  = (undefined4 *)0x0;
    iVar1->field_0xa  = 0x0;
    iVar1->field_0xe  = param_3;
    iVar1->field_0x12 = 0x0;
    iVar1->field_0x16 = param_2;
    iVar1->field_0x1a = 0x1;
    *param_1          = (int)s_462_bmp_1050_1620 + 0x4;
    iVar1->field_0x2  = 0x1030;
    if(iVar1->field_0xe == 0x0)
    {
        iVar1->field_0xe = 0x5;
    }
    if(iVar1->field_0x16 == 0x0)
    {
        iVar1->field_0x16 = 0x5;
    }
    struct_1030_1550((ulong)param_1, param_4);
    *iVar1->field_0x6 = 0x0;
    return;
}


void __stdcall16far pass1_1030_1358(ulong param_1, ushort param_2, ushort param_3, ulong param_4, ushort param_5)

{
    ulong       *puVar1;
    uint        *puVar2;
    long         lVar3;
    astruct_291 *iVar4;
    int          iVar5;
    uint         uVar6;
    undefined2   uVar7;
    bool         bVar8;

    if(param_4 == 0x0)
    {
        return;
    }
    uVar6  = (uint)(param_1 >> 0x10);
    iVar4  = (astruct_291 *)param_1;
    puVar1 = (ulong *)&iVar4->field_0xa;
    if((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4->field_0x6 == 0x0))
    {
        puVar2 = (uint *)((int)&iVar4->field_0x12 + 0x2);
        bVar8  = *puVar2 < param_4._2_2_;
        if((bVar8 || *puVar2 == param_4._2_2_)
           && ((bVar8
                || (puVar1 = &iVar4->field_0x12, *(uint *)puVar1 < (uint)param_4 || *(uint *)puVar1 == (uint)param_4))))
        {
            struct_1030_1550(param_1 & 0xffff | (ulong)uVar6 << 0x10, param_5);
        }
        puVar1 = &iVar4->field_0x12;
        if(*puVar1 < param_4 || *puVar1 == param_4)
        {
            return;
        }
        if(iVar4->field_0x6 == 0x0)
        {
            return;
        }
        puVar2 = &iVar4->field_0xc;
        bVar8  = *puVar2 < param_4._2_2_;
        if((bVar8 || *puVar2 == param_4._2_2_)
           && ((bVar8 || (puVar2 = &iVar4->field_0xa, *puVar2 < (uint)param_4 || *puVar2 == (uint)param_4))))
        {
            iVar4->field_0xa = (uint)(param_4 + 0x1);
            iVar4->field_0xc = (uint)(param_4 + 0x1 >> 0x10);
        }
    }
    lVar3                                          = iVar4->field_0x6;
    uVar7                                          = (undefined2)((ulong)lVar3 >> 0x10);
    iVar5                                          = (int)lVar3;
    *(ushort *)(iVar5 + (uint)param_4 * 0x4)       = param_2;
    *(ushort *)(iVar5 + (uint)param_4 * 0x4 + 0x2) = param_3;
    return;
}


void __stdcall16far pass1_1030_14b4(ulong param_1, ushort param_2, ushort param_3, ulong param_4, ushort param_5)

{
    ulong       *puVar1;
    uint        *puVar2;
    long         lVar3;
    astruct_345 *iVar5;
    astruct_344 *iVar4;
    uint         uVar4;
    undefined2   uVar5;
    bool         bVar6;

    uVar4  = (uint)(param_1 >> 0x10);
    iVar5  = (astruct_345 *)param_1;
    puVar1 = (ulong *)&iVar5->field_0xa;
    if((*puVar1 < param_4 || *puVar1 == param_4) || (iVar5->field_0x6 == 0x0))
    {
        puVar2 = (uint *)((int)&iVar5->field_0x12 + 0x2);
        bVar6  = *puVar2 < param_4._2_2_;
        if((bVar6 || *puVar2 == param_4._2_2_)
           && ((bVar6
                || (puVar1 = &iVar5->field_0x12, *(uint *)puVar1 < (uint)param_4 || *(uint *)puVar1 == (uint)param_4))))
        {
            struct_1030_1550(param_1 & 0xffff | (ulong)uVar4 << 0x10, param_5);
        }
        puVar1 = &iVar5->field_0x12;
        if((*puVar1 < param_4 || *puVar1 == param_4) || (iVar5->field_0x6 == 0x0))
        {
            return;
        }
        puVar2 = &iVar5->field_0xc;
        bVar6  = *puVar2 < param_4._2_2_;
        if((bVar6 || *puVar2 == param_4._2_2_)
           && ((bVar6 || (puVar2 = &iVar5->field_0xa, *puVar2 < (uint)param_4 || *puVar2 == (uint)param_4))))
        {
            iVar5->field_0xa = (uint)(param_4 + 0x1);
            iVar5->field_0xc = (uint)(param_4 + 0x1 >> 0x10);
        }
    }
    lVar3                                          = iVar5->field_0x6;
    uVar5                                          = (undefined2)((ulong)lVar3 >> 0x10);
    iVar4                                          = (astruct_344 *)lVar3;
    *(ushort *)(iVar4 + (uint)param_4 * 0x4)       = param_2;
    *(ushort *)(iVar4 + (uint)param_4 * 0x4 + 0x2) = param_3;
    return;
}


void __stdcall16far struct_1030_1628(ushort *param_1)

{
    astruct_181 *iVar1;
    undefined2   uVar1;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_181 *)param_1;
    *param_1         = 0x389a;
    iVar1->field_0x2 = 0x1008;
    iVar1->field_0x4 = 0x0;
    iVar1->field_0x8 = 0x0;
    *param_1         = 0x17ba;
    iVar1->field_0x2 = 0x1030;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_165e(ushort *param_1, ulong param_2, ulong param_3, ushort param_4)

{
    astruct_175 *iVar1;
    undefined2   uVar1;

    uVar1                            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                            = (astruct_175 *)param_1;
    *param_1                         = 0x389a;
    iVar1->field_0x2                 = 0x1008;
    *(undefined4 *)&iVar1->field_0x4 = 0x0;
    iVar1->field_0x8                 = param_3;
    *param_1                         = 0x17ba;
    iVar1->field_0x2                 = 0x1030;
    pass1_1030_5c8a(_PTR_LOOP_1050_5736, param_2);
    iVar1->field_0x4 = (int)param_3;
    iVar1->field_0x6 = param_4;
    return;
}


void __stdcall16far pass1_1030_16b2(ushort *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *param_1                            = 0x17ba;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1030;
    *param_1                            = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    return;
}


void __stdcall16far struct_op_1030_1cd8(astruct_75 *param_1, ulong param_2, ulong param_3)

{
    astruct_75 *struct_var1;
    astruct_75 *struct_var2;

    struct_var2             = (astruct_75 *)((ulong)param_1 >> 0x10);
    struct_var1             = (astruct_75 *)param_1;
    param_1->field_0x0      = 0x389a;
    struct_var1->field_0x2  = 0x1008;
    struct_var1->field_0x4  = 0x0;
    struct_var1->field_0x8  = 0x0;
    struct_var1->field_0xc  = param_3;
    struct_var1->field_0x10 = 0x0;
    struct_var1->field_0x14 = param_2;
    param_1->field_0x0      = 0x2044;
    struct_var1->field_0x2  = 0x1030;
    return;
}


void __stdcall16far pass1_1030_1d28(ushort *param_1)

{
    astruct_594 *iVar1;
    undefined2   uVar1;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar1            = (astruct_594 *)param_1;
    *param_1         = 0x2044;
    iVar1->field_0x2 = 0x1030;
    fn_ptr_1000_17ce(iVar1->field_0x4, 0x1000);
    *param_1         = 0x389a;
    iVar1->field_0x2 = 0x1008;
    return;
}


ulong __stdcall16far pass1_1030_1d7c(uint param_1, uint param_2, ulong param_3)

{
    ulong uVar1;

    pass1_1030_1d58(param_3);
    if((param_2 | param_1) != 0x0)
    {
        uVar1 = struct_op_1030_73a8(CONCAT22(param_2, param_1));
        return uVar1;
    }
    return 0x0;
}


ushort *__stdcall16far switch_1030_0000(
  ushort param_1, ushort param_2, int param_3, uchar *param_4, uint param_5, ushort param_6, ushort param_7)

{
    uchar  *puVar1;
    uint    uVar2;
    ushort *puVar3;

    // Segment:    7
    // Offset:     000516c0
    // Length:     ef76
    // Min Alloc:  ef76
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    puVar3 = (ushort *)CONCAT22(param_4, param_5);
    switch(param_3 + -0x1)
    {
    case 0x0:
    case 0x1:
    case 0x2:
    case 0x3:
    case 0x4:
    case 0x5:
    case 0x6:
    case 0x7:
    case 0x8:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_489e((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x9:
        mem_op_1000_179c(0x22, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_2bdc((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0xa:
        mem_op_1000_179c(0x26, param_4, 0x1000);
        uVar2 = (uint)param_4 | param_5;
        goto joined_r0x103002a1;
    case 0xb:
        mem_op_1000_179c(0x2c, param_4, 0x1000);
        if((uchar *)((uint)param_4 | param_5) != (uchar *)0x0)
        {
            puVar3 = struct_1028_3670(
              (ushort *)CONCAT22(param_4, param_5), (uchar *)((uint)param_4 | param_5), param_6, param_7);
            return puVar3;
        }
        break;
    case 0xc:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_355e((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0xd:
        mem_op_1000_179c(0x26, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_3484((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0xe:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_406c((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0xf:
    case 0x32:
    case 0x33:
    case 0x5f:
    case 0x60:
        mem_op_1000_179c(0x24, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_0c24((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x10:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_0b42((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x11:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_4354((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x12:
    case 0x13:
    case 0x14:
    case 0x61:
    case 0x62:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_4b84((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x15:
    case 0x16:
    case 0x17:
        mem_op_1000_179c(0x24, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_1bbc((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    default:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        uVar2 = (uint)param_4 | param_5;
        if(uVar2 != 0x0)
        {
            struct_1028_b354((ushort *)CONCAT22(param_4, param_5));
            return (ushort *)CONCAT22(uVar2, param_5);
        }
        break;
    case 0x1a:
    case 0x1b:
    case 0x1c:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1030_be34((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x1d:
    case 0x1e:
    case 0x1f:
        mem_op_1000_179c(0x26, param_4, 0x1000);
        puVar1 = (uchar *)((uint)param_4 | param_5);
        if(puVar1 != (uchar *)0x0)
        {
            struct_1028_0068((ushort *)CONCAT22(param_4, param_5), puVar1);
            return (ushort *)CONCAT22(puVar1, param_5);
        }
        break;
    case 0x20:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_50d8((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x21:
    case 0x22:
    case 0x23:
        mem_op_1000_179c(0x24, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_3e94((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x24:
    case 0x25:
    case 0x26:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1020_d06c((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x27:
    case 0x28:
    case 0x5c:
    case 0x5d:
    case 0x5e:
        mem_op_1000_179c(0x22, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1030_c6f6((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x29:
    case 0x2a:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1020_cce4((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x2b:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_26b4((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x2c:
    case 0x2d:
        mem_op_1000_179c(0x2a, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_49aa((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x2e:
    case 0x2f:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1020_e7fa((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x30:
    case 0x31:
    case 0x6b:
    case 0x6c:
        mem_op_1000_179c(0x22, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1020_d37c((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x34:
    case 0x35:
        mem_op_1000_179c(0x2c, param_4, 0x1000);
        puVar1 = (uchar *)((uint)param_4 | param_5);
        if(puVar1 != (uchar *)0x0)
        {
            struct_1028_37a6((ushort *)CONCAT22(param_4, param_5), puVar1, param_6, param_7);
            return (ushort *)CONCAT22(puVar1, param_5);
        }
        break;
    case 0x36:
        mem_op_1000_179c(0x26, param_4, 0x1000);
        uVar2 = (uint)param_4 | param_5;
    joined_r0x103002a1:
        if(uVar2 != 0x0)
        {
            struct_1030_c06e((ushort *)CONCAT22(param_4, param_5));
            return (ushort *)CONCAT22(uVar2, param_5);
        }
        break;
    case 0x37:
    case 0x38:
        mem_op_1000_179c(0x9a, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1030_c9a8((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x39:
    case 0x3a:
        mem_op_1000_179c(0x24, param_4, 0x1000);
        if((uchar *)((uint)param_4 | param_5) != (uchar *)0x0)
        {
            puVar3
              = struct_1028_60bc((ushort *)CONCAT22(param_4, param_5), param_5, (uchar *)((uint)param_4 | param_5));
            return puVar3;
        }
        break;
    case 0x3b:
    case 0x3c:
        mem_op_1000_179c(0x24, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_44d2((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x3d:
        mem_op_1000_179c(0x22, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1020_cde6((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x3e:
        mem_op_1000_179c(0x26, param_4, 0x1000);
        puVar1 = (uchar *)((uint)param_4 | param_5);
        if(puVar1 != (uchar *)0x0)
        {
            struct_1028_1f56((ushort *)CONCAT22(param_4, param_5), puVar1);
            return (ushort *)CONCAT22(puVar1, param_5);
        }
        break;
    case 0x3f:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_25da((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x40:
        mem_op_1000_179c(0x22, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1020_c9ea((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x46:
    case 0x69:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1020_d5a6((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x47:
    case 0x48:
    case 0x49:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1020_d866((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x4b:
    case 0x4c:
    case 0x4d:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1030_d8f6((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x4e:
    case 0x4f:
    case 0x50:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_5c54((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x51:
    case 0x52:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_5966((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x53:
    case 0x54:
    case 0x55:
        mem_op_1000_179c(0x22, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_5ed8((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x56:
    case 0x57:
    case 0x58:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_53c6((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x59:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = pass1_1028_5884((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x5a:
    case 0x5b:
        mem_op_1000_179c(0x26, param_4, 0x1000);
        if((uchar *)((uint)param_4 | param_5) != (uchar *)0x0)
        {
            puVar3 = pass1_1028_5524((ushort *)CONCAT22(param_4, param_5), (uchar *)((uint)param_4 | param_5));
            return puVar3;
        }
        break;
    case 0x63:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = pass1_1028_5df6((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x64:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_5a48((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x65:
    case 0x66:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = (ushort *)struct_1028_52e8(param_5, param_4);
            return puVar3;
        }
        break;
    case 0x67:
    case 0x68:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_57a6((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x6d:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = (ushort *)struct_1028_5630((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x6f:
    case 0x70:
    case 0x71:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) == 0x0)
        {
            puVar3 = (ushort *)0x0;
        }
        else
        {
            puVar3 = struct_1020_d866((ushort *)CONCAT22(param_4, param_5));
        }
    case 0x72:
    case 0x76:
        mem_op_1000_179c(0x26, (uchar *)((ulong)puVar3 >> 0x10), 0x1000);
        if(puVar3 != (ushort *)0x0)
        {
            puVar3 = struct_1020_e8f6(puVar3);
            return puVar3;
        }
        break;
    case 0x73:
    case 0x77:
    case 0x78:
        mem_op_1000_179c(0x2c, param_4, 0x1000);
        uVar2 = (uint)param_4 | param_5;
        if(uVar2 != 0x0)
        {
            struct_1020_d954((ushort *)CONCAT22(param_4, param_5));
            return (ushort *)CONCAT22(uVar2, param_5);
        }
        break;
    case 0x74:
        mem_op_1000_179c(0x24, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_178c((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x75:
        mem_op_1000_179c(0x24, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = struct_1028_2afa((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
        break;
    case 0x79:
    case 0x7a:
    case 0x7b:
    case 0x7c:
    case 0x7d:
    case 0x7e:
        mem_op_1000_179c(0x20, param_4, 0x1000);
        if(((uint)param_4 | param_5) != 0x0)
        {
            puVar3 = (ushort *)struct_1028_27f0((ushort *)CONCAT22(param_4, param_5));
            return puVar3;
        }
    }
    return (ushort *)0x0;
}


void __stdcall16far pass1_1028_dc52(astruct_92 *param_1, int param_2, ushort param_3, uint param_4)

{
    undefined4  uVar1;
    astruct_92 *iVar2;
    undefined2  uVar2;

    uVar2                  = (undefined2)((ulong)param_1 >> 0x10);
    iVar2                  = (astruct_92 *)param_1;
    *(undefined2 *)param_1 = 0x389a;
    iVar2->field_0x2       = 0x1008;
    iVar2->field_0x4       = *(undefined4 *)((int)_PTR_LOOP_1050_65e2 + (param_4 >> 0x8) * 0x4 + 0xa);
    iVar2->field_0x8       = 0x1;
    iVar2->field_0x10      = param_2;
    *(undefined2 *)param_1 = 0x11a6;
    iVar2->field_0x2       = 0x1030;
    uVar1                  = iVar2->field_0x4;
    iVar2->field_0xc       = *(undefined4 *)((int)uVar1 + 0xa);
    if(param_2 == 0x0)
    {
        iVar2->field_0x8 = iVar2->field_0xc;
    }
    else
    {
        iVar2->field_0x8 = 0x1;
    }
    return;
}


BOOL16 __stdcall16far pass1_1028_c5a6(ushort  param_1,
                                      ushort  param_2,
                                      int     param_3,
                                      ushort *param_4,
                                      long    param_5,
                                      uint    param_6,
                                      uint    param_7,
                                      ushort  param_8)

{
    int   iVar1;
    uint  uVar2;
    ulong uVar3;
    int   iStack14;
    ulong uStack10;

    pass1_1030_627e(param_8, param_6, param_7, _PTR_LOOP_1050_5740, param_4, param_5);
    uVar2 = param_7 | param_6;
    if(uVar2 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_6, param_7);
        uStack10 = CONCAT22(uVar2, param_6);
        iStack14 = 0x7a;
        if(0x0 < *(int *)((int)param_4 + 0x4))
        {
            if(param_3 == 0x7b)
            {
                param_3 = 0x7e;
            }
            else
            {
                if(param_3 == 0x7c)
                {
                    param_3 = 0x7d;
                }
            }
            iStack14 = 0x7f;
        }
        if(uStack10 != 0x0)
        {
            uVar3 = struct_op_1030_73a8(uStack10);
            if((uVar3 != 0x0) && ((iVar1 = *(int *)((int)uVar3 + 0xc), iVar1 == iStack14 || (iVar1 == param_3))))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}


void __stdcall16far pass1_1028_c00a(ulong param_1, long param_2, int param_3, ushort param_4)

{
    code      **ppcVar1;
    uint        uVar2;
    ushort      uVar3;
    undefined2  extraout_DX;
    uchar      *puVar4;
    undefined2  extraout_DX_00;
    uint        extraout_DX_01;
    uint        uVar5;
    undefined2  uVar6;
    ulong      *puVar7;
    ulong       uVar8;
    ulong       uVar9;
    ulong       uStack26;
    ulong       uStack22;
    undefined4 *puStack18;

    pass1_1028_b58e(param_1);
    uVar8  = *(ulong *)(param_3 + 0x2e);
    puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x4);
    puVar4 = (uchar *)((ulong)puVar7 >> 0x10);
    uVar2  = (uint)puVar7;
    uVar6  = SUB42(&PTR_LOOP_1050_1038, 0x0);
    pass1_1038_4d6e(uVar8, puVar7, uVar2, puVar4);
    puStack18 = (undefined4 *)CONCAT22(puVar4, uVar2);
    ppcVar1   = (code **)((int)*puStack18 + 0x10);
    uVar5     = uVar2;
    (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar2, puVar4);
    uStack22 = CONCAT22(extraout_DX_00, uVar5);
    uStack26 = 0x0;
    do
    {
        if(uStack22 <= uStack26)
        {
        LAB_1028_c0d6:
            if(puStack18 != (undefined4 *)0x0)
            {
                ppcVar1 = (code **)*puStack18;
                (**ppcVar1)(uVar6, uVar2, (char)puVar4, 0x1);
            }
            return;
        }
        ppcVar1 = (code **)((int)*puStack18 + 0x4);
        uVar8   = uStack22;
        (**ppcVar1)((char)uVar6, uVar2, puVar4, (char)uStack26, (int)(uStack26 >> 0x10));
        uVar3 = (ushort)uVar8;
        uVar5 = extraout_DX_01;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, extraout_DX_01);
        uVar6 = 0x1030;
        uVar8 = struct_op_1030_73a8(CONCAT22(uVar5, uVar3));
        uVar9 = pass1_1028_6302(uVar8, param_4);
        uVar5 = (uint)(uVar9 >> 0x10);
        if((param_2._2_2_ <= uVar5) && ((param_2._2_2_ < uVar5 || ((uint)param_2 <= (uint)uVar9))))
        {
            pass1_1028_6356(uVar8, 0x0, (uint)param_2, param_2._2_2_, param_4);
            goto LAB_1028_c0d6;
        }
        pass1_1028_6356(uVar8, 0x0, (uint)uVar9, uVar5, param_4);
        param_2  = param_2 - uVar9;
        uStack26 = uStack26 + 0x1;
    } while(true);
}


void __stdcall16far
pass1_1028_c0f0(ulong param_1, long param_2, int param_3, ushort param_4, ushort param_5, ushort param_6)

{
    code      **ppcVar1;
    uint        uVar2;
    ushort      uVar3;
    undefined2  extraout_DX;
    uchar      *puVar4;
    uchar      *extraout_DX_00;
    uint        extraout_DX_01;
    uint        uVar5;
    uchar      *puVar6;
    uchar      *extraout_DX_02;
    undefined2  uVar7;
    ulong      *puVar8;
    ulong       uVar9;
    ulong       uStack28;
    ulong       uStack24;
    undefined4 *puStack20;
    ulong       uStack6;

    pass1_1028_b58e(param_1);
    uStack6 = CONCAT22(extraout_DX, param_3);
    uVar9   = *(ulong *)(param_3 + 0x2e);
    pass1_1028_cb04(param_1, param_4, param_5, param_6);
    uVar7 = (undefined2)(uVar9 >> 0x10);
    if((*(int *)((int)uVar9 + 0x204) == 0x0) && (*(int *)((int)uVar9 + 0x206) == 0x0))
    {
        puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x4);
        puVar4 = (uchar *)((ulong)puVar8 >> 0x10);
        uVar2  = (uint)puVar8;
        uVar7  = SUB42(&PTR_LOOP_1050_1038, 0x0);
        pass1_1038_4d6e(uVar9, puVar8, uVar2, puVar4);
        puStack20 = (undefined4 *)CONCAT22(puVar4, uVar2);
        ppcVar1   = (code **)((int)*puStack20 + 0x10);
        uVar5     = uVar2;
        (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar2, puVar4);
        uStack24 = CONCAT22(extraout_DX_00, uVar5);
        puVar6   = extraout_DX_00;
        for(uStack28 = 0x0; uStack28 < uStack24; uStack28 = uStack28 + 0x1)
        {
            ppcVar1 = (code **)((int)*puStack20 + 0x4);
            uVar9   = uStack24;
            (**ppcVar1)((char)uVar7, uVar2, puVar4, (char)uStack28, (int)(uStack28 >> 0x10));
            uVar3 = (ushort)uVar9;
            uVar5 = extraout_DX_01;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, extraout_DX_01);
            uVar7  = 0x1030;
            uVar9  = struct_op_1030_73a8(CONCAT22(uVar5, uVar3));
            uVar9  = pass1_1028_6302(uVar9, param_6);
            puVar6 = (uchar *)(uVar9 >> 0x10);
            uVar5  = (uint)uVar9;
            if((param_2._2_2_ <= puVar6) && ((param_2._2_2_ < puVar6 || ((uint)param_2 <= uVar5))))
            {
                param_2 = 0x0;
                break;
            }
            param_2 = CONCAT22(param_2._2_2_ + (-(uint)((uint)param_2 < uVar5) - (int)puVar6), (uint)param_2 - uVar5);
        }
        if(puStack20 != (undefined4 *)0x0)
        {
            ppcVar1 = (code **)*puStack20;
            (**ppcVar1)(uVar7, uVar2, (char)puVar4, 0x1);
            puVar6 = extraout_DX_02;
        }
        if(param_2 != 0x0)
        {
            pass1_1030_7d7c(uStack6,
                            (uint)param_2,
                            CONCAT22(0x1d, (int)((ulong)param_2 >> 0x10)),
                            (uint)puStack20,
                            puVar6,
                            param_4,
                            param_5,
                            param_6);
        }
    }
    return;
}


astruct_100 *__stdcall16far pass1_1028_a706(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0xbb7);
    param_1->field_0x0                  = 0xa856;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),
                         s_SCPrelimAlloc_1050_50f6);
    return param_1;
}


astruct_100 *__stdcall16far pass1_1028_a866(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x36af);
    param_1->field_0x0                  = 0xa9ae;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCProdSched_1050_5104);
    return param_1;
}


astruct_100 *__stdcall16far pass1_1028_a9be(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x176f);
    param_1->field_0x0                  = 0xab22;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCPower_1050_5110);
    return param_1;
}


astruct_100 *__stdcall16far pass1_1028_ab32(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x2edf);
    param_1->field_0x0                  = 0xaca6;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCRchSched_1050_5118);
    return param_1;
}


astruct_100 *__stdcall16far pass1_1028_acb6(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x3e7f);
    param_1->field_0x0                  = 0xae56;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCSetup_1050_5124);
    return param_1;
}


void __stdcall16far
pass1_1028_ae66(astruct_100 *param_1, ulong param_2, ulong param_3, ulong param_4, ushort param_5, uchar param_6)

{
    astruct_689 *iVar1;
    undefined2   uVar1;

    struct_op_1028_d1dc(param_5, param_6, param_1, 0x1387);
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    iVar1              = (astruct_689 *)param_1;
    iVar1->field_0x108 = param_4;
    iVar1->field_0x10c = param_3;
    iVar1->field_0x110 = param_2;
    iVar1->field_0x114 = 0x0;
    param_1->field_0x0 = 0xb0ce;
    iVar1->field_0x2   = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar1->field_0x8), s_SCStarve_1050_5156);
    return;
}


ushort *__stdcall16far pass1_1028_b204(ushort *param_1)

{
    undefined2 uVar1;

    struct_1030_1628(param_1);
    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *(undefined2 *)((int)param_1 + 0xc) = 0x0;
    *param_1                            = 0xb33c;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


void __stdcall16far struct_1028_b354(ushort *param_1)

{
    astruct_180 *iVar1;
    undefined2   uVar1;

    struct_1030_1628(param_1);
    uVar1             = (undefined2)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_180 *)param_1;
    iVar1->field_0xc  = 0x0;
    iVar1->field_0xe  = 0x0;
    iVar1->field_0x10 = 0x0;
    iVar1->field_0x12 = 0x0;
    iVar1->field_0x18 = 0x0;
    iVar1->field_0x1a = 0x0;
    iVar1->field_0x1c = 0x0;
    *param_1          = 0xcf6a;
    iVar1->field_0x2  = (int)&USHORT_1050_1028;
    iVar1->field_0x16 = 0x0;
    iVar1->field_0x14 = 0x0;
    return;
}


void __stdcall16far pass1_1028_b39e(ushort *param_1, int param_2, ulong param_3, ushort param_4)

{
    astruct_173 *iVar1;
    uint         uVar1;

    pass1_1030_165e(param_1, 0x7000000, param_3, param_4);
    uVar1             = (uint)((ulong)param_1 >> 0x10);
    iVar1             = (astruct_173 *)param_1;
    iVar1->field_0xc  = param_2;
    iVar1->field_0xe  = 0x42;
    iVar1->field_0x10 = 0x0;
    iVar1->field_0x12 = 0x0;
    iVar1->field_0x18 = 0x0;
    iVar1->field_0x1a = 0x0;
    iVar1->field_0x1c = 0x0;
    *param_1          = 0xcf6a;
    iVar1->field_0x2  = (int)&USHORT_1050_1028;
    pass1_1028_bf76((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10, 0x0);
    iVar1->field_0x14 = 0x0;
    if((0x4e < iVar1->field_0xc) && (iVar1->field_0xc < 0x70))
    {
        iVar1->field_0xe = 0x6b;
    }
    return;
}


void __stdcall16far
pass1_1028_9944(astruct_100 *param_1, ulong param_2, ulong param_3, ulong param_4, ushort param_5, uchar param_6)

{
    astruct_699 *iVar1;
    undefined2   uVar1;

    struct_op_1028_d1dc(param_5, param_6, param_1, 0x1387);
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    iVar1              = (astruct_699 *)param_1;
    iVar1->field_0x108 = param_4;
    iVar1->field_0x10c = param_3;
    iVar1->field_0x110 = param_2;
    iVar1->field_0x114 = 0x0;
    param_1->field_0x0 = 0x9c52;
    iVar1->field_0x2   = (int)&USHORT_1050_1028;
    return;
}


ushort *__stdcall16far struct_1028_9c62(int param_1, ushort param_2, ushort param_3, ushort param_4, uchar param_5)

{
    struct_op_1028_d1dc(param_4, param_5, (astruct_100 *)CONCAT22(param_2, param_1), param_3);
    *(ushort *)(param_1 + 0x108)          = param_3;
    *(ushort *)CONCAT22(param_2, param_1) = 0x9eb6;
    *(undefined2 *)(param_1 + 0x2)        = (int)&USHORT_1050_1028;
    return (ushort *)CONCAT22(param_2, param_1);
}


astruct_100 *__stdcall16far pass1_1028_9ec6(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, (int)s_noth_bmp_1050_2321 + 0x6);
    param_1->field_0x0                  = 0xa6f6;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), (char *)0x105050f0);
    return param_1;
}


astruct_100 *__stdcall16far pass1_1028_837e(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0xf9f);
    param_1->field_0x0                  = 0x84ba;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)),
                         s_SCFillResources_1050_500c);
    return param_1;
}


void __stdcall16far struct_op_1028_87f0(ushort      param_1,
                                        uchar       param_2,
                                        astruct_97 *param_3,
                                        ushort      param_4,
                                        ushort      param_5,
                                        ushort      param_6,
                                        ulong      *param_7,
                                        ushort      param_8,
                                        ulong       param_9,
                                        ulong       param_10)

{
    astruct_97 *iVar1;
    astruct_97 *puVar1;

    struct_op_1028_d1dc(param_1, param_2, (astruct_100 *)param_3, 0x3e8);
    puVar1                 = (astruct_97 *)((ulong)param_3 >> 0x10);
    iVar1                  = (astruct_97 *)param_3;
    iVar1->field_0x108     = param_10;
    iVar1->field_0x10c     = param_9;
    iVar1->field_0x110     = 0x0;
    iVar1->field_0x114     = *param_7;
    iVar1->field_0x118     = *(undefined2 *)(param_7 + 0x1);
    iVar1->field_0x11a     = param_6;
    iVar1->field_0x11c     = param_5;
    iVar1->field_0x11e     = param_4;
    iVar1->field_0x122     = 0x0;
    iVar1->field_0x120     = 0x0;
    *(undefined2 *)param_3 = 0x8d8e;
    iVar1->field_0x2       = (int)&USHORT_1050_1028;
    sys_1000_3f9c(&iVar1->field_0x8,
                  (uchar *)puVar1,
                  (ushort)s_SCInternalPutBldg_site_0x_08lx__b_1050_5046,
                  (ushort)&USHORT_1050_1050,
                  (ushort)param_10,
                  &stack0xfffe,
                  puVar1,
                  0x1000,
                  param_1,
                  param_2);
    return;
}


void __stdcall16far struct_op_1028_8888(ushort       param_1,
                                        uchar        param_2,
                                        astruct_100 *param_3,
                                        ushort       param_4,
                                        ushort       param_5,
                                        ulong       *param_6,
                                        ushort       param_7,
                                        ulong        param_8,
                                        ulong        param_9,
                                        ulong        param_10)

{
    astruct_100 *iVar1;
    uchar       *puVar1;

    struct_op_1028_d1dc(param_1, param_2, param_3, 0x3e8);
    puVar1             = (uchar *)((ulong)param_3 >> 0x10);
    iVar1              = (astruct_100 *)param_3;
    iVar1->field_0x108 = param_10;
    iVar1->field_0x10c = param_9;
    iVar1->field_0x110 = param_8;
    iVar1->field_0x114 = *param_6;
    iVar1->field_0x118 = *(ushort *)(param_6 + 0x1);
    iVar1->field_0x11a = param_5;
    iVar1->field_0x11c = 0x0;
    iVar1->field_0x11e = param_4;
    iVar1->field_0x122 = 0x0;
    iVar1->field_0x120 = 0x0;
    param_3->field_0x0 = 0x8d8e;
    iVar1->field_0x2   = (int)&USHORT_1050_1028;
    sys_1000_3f9c(&iVar1->field_0x8,
                  puVar1,
                  (ushort)s_SCInternalPutBldg2_site_0x_08lx__1050_506f,
                  (ushort)&USHORT_1050_1050,
                  (ushort)param_10,
                  &stack0xfffe,
                  puVar1,
                  0x1000,
                  param_1,
                  param_2);
    return;
}


void __stdcall16far
pass1_1028_8d9e(astruct_100 *param_1, ulong param_2, ulong param_3, ulong param_4, ushort param_5, uchar param_6)

{
    int        iVar1;
    undefined2 uVar2;

    struct_op_1028_d1dc(param_5, param_6, param_1, 0x3e8);
    uVar2                          = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                          = (int)param_1;
    *(ulong *)(iVar1 + 0x108)      = param_4;
    *(ulong *)(iVar1 + 0x10c)      = param_3;
    *(ulong *)(iVar1 + 0x110)      = param_2;
    *(undefined4 *)(iVar1 + 0x114) = 0x0;
    param_1->field_0x0             = 0x8fb0;
    *(undefined2 *)(iVar1 + 0x2)   = (int)&USHORT_1050_1028;
    return;
}


astruct_100 *__stdcall16far pass1_1028_90e6(astruct_100 *param_1, ushort param_2, ushort param_3, uchar param_4)

{
    undefined2 uVar1;

    struct_op_1028_d1dc(param_3, param_4, param_1, 0x1387);
    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *(ushort *)((int)param_1 + 0x108)   = param_2;
    param_1->field_0x0                  = 0x932c;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}

astruct_100 *__stdcall16far pass1_1028_74ae(astruct_100 *param_1)

{
    ushort unaff_SS;
    uchar  in_AF;

    struct_op_1028_d1dc(unaff_SS, in_AF, param_1, 0x1387);
    param_1->field_0x0                  = 0x819a;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCEvent_1050_4ff4);
    return param_1;
}

void __stdcall16far pass1_1028_780c(ushort param_1, ushort param_2, ulong param_3)

{
    code      **ppcVar1;
    uint        uVar2;
    ushort      uVar3;
    ulong       uVar4;
    uchar      *puVar5;
    uint        extraout_DX;
    uint        extraout_DX_00;
    uint        uVar6;
    undefined2  uVar7;
    ulong      *puVar8;
    undefined4 *puVar9;
    ulong       uStack18;
    ulong       uStack14;
    undefined4 *puStack10;

    puVar8 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, 0x25);
    puVar5 = (uchar *)((ulong)puVar8 >> 0x10);
    uVar2  = (uint)puVar8;
    uVar7  = SUB42(&PTR_LOOP_1050_1038, 0x0);
    pass1_1038_4e78(uVar2, puVar5, param_3, puVar8);
    puStack10 = (undefined4 *)CONCAT22(puVar5, uVar2);
    ppcVar1   = (code **)((int)*puStack10 + 0x10);
    uVar6     = uVar2;
    (**ppcVar1)((int)&PTR_LOOP_1050_1038, uVar2, puVar5);
    uStack14 = CONCAT22(extraout_DX, uVar6);
    if((extraout_DX | uVar6) == 0x0)
    {
        return;
    }
    for(uStack18 = 0x0; uStack18 < uStack14; uStack18 = uStack18 + 0x1)
    {
        ppcVar1 = (code **)((int)*puStack10 + 0x4);
        uVar4   = uStack14;
        (**ppcVar1)();
        uVar3 = (ushort)uVar4;
        uVar6 = extraout_DX_00;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, extraout_DX_00);
        uVar7   = 0x1030;
        puVar9  = (undefined4 *)struct_op_1030_73a8(CONCAT22(uVar6, uVar3));
        ppcVar1 = (code **)((int)*puVar9 + 0x24);
        (**ppcVar1)();
    }
    if(puStack10 != (undefined4 *)0x0)
    {
        ppcVar1 = (code **)*puStack10;
        (**ppcVar1)(uVar7, uVar2, (char)puVar5, 0x1);
    }
    return;
}


astruct_100 *__stdcall16far pass1_1028_81aa(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x1b57);
    param_1->field_0x0                  = 0x836e;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCFactory_1050_5002);
    return param_1;
}


void __stdcall16far pass1_1028_68de(astruct_100 *param_1, ushort param_2, ulong param_3, uchar param_4, ushort param_5)

{
    int        iVar1;
    undefined2 uVar2;

    struct_op_1028_d1dc(param_5, param_4, param_1, 0x3e8);
    uVar2                        = (undefined2)((ulong)param_1 >> 0x10);
    iVar1                        = (int)param_1;
    *(ulong *)(iVar1 + 0x108)    = param_3;
    *(ushort *)(iVar1 + 0x10c)   = param_2;
    param_1->field_0x0           = 0x6ae2;
    *(undefined2 *)(iVar1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x8)), s_SCAddSpew_1050_4fd2);
    return;
}


void __stdcall16far pass1_1028_6af2(astruct_100 *param_1, ulong param_2, ulong param_3, ushort param_4, uchar param_5)

{
    astruct_683 *iVar1;
    undefined2   uVar1;

    struct_op_1028_d1dc(param_4, param_5, param_1, 0x1387);
    uVar1              = (undefined2)((ulong)param_1 >> 0x10);
    iVar1              = (astruct_683 *)param_1;
    iVar1->field_0x108 = param_3;
    iVar1->field_0x10c = param_2;
    param_1->field_0x0 = 0x6e50;
    iVar1->field_0x2   = (int)&USHORT_1050_1028;
    return;
}


astruct_100 *__stdcall16far pass1_1028_6e60(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x32c7);
    param_1->field_0x0                  = 0x6fb0;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCConstruct_1050_4fdc);
    return param_1;
}


astruct_100 *__stdcall16far pass1_1028_6fc0(astruct_100 *param_1, ushort param_2, uchar param_3)

{
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x3e7);
    param_1->field_0x0                  = 0x749e;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x8)), s_SCEndSim_1050_4fea);
    return param_1;
}


ushort *__stdcall16far struct_1028_50d8(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x5280;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_52e8(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x535e;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_53c6(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x54bc;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far pass1_1028_5524(ushort *param_1, uchar *param_2)

{
    struct_1028_0068(param_1, param_2);
    *param_1                            = 0x55c8;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_5630(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x56ac;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far struct_1028_57a6(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x581c;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}


ushort *__stdcall16far pass1_1028_5884(ushort *param_1)

{
    struct_1028_b354(param_1);
    *param_1                            = 0x58fe;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
    return param_1;
}
